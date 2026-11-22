use crate::{
    AuthenticateError, LanguageModel, LanguageModelCompletionError, LanguageModelId,
    LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderState,
    unavailable_error,
};
use collections::{BTreeMap, HashSet};
use gpui::{App, Context, Entity, EventEmitter, Global, Task, prelude::*};
use std::{str::FromStr, sync::Arc};
use thiserror::Error;

/// Function type for checking if a built-in provider should be hidden.

pub fn init(cx: &mut App) {
    let registry = cx.new(|_cx| LanguageModelRegistry::default());
    cx.set_global(GlobalLanguageModelRegistry(registry));
}

struct GlobalLanguageModelRegistry(Entity<LanguageModelRegistry>);

impl Global for GlobalLanguageModelRegistry {}

#[derive(Error)]
pub enum ConfigurationError {
    #[error("Configure at least one LLM provider to start using the panel.")]
    NoProvider,
    #[error("LLM provider is not configured or does not support the configured model.")]
    ModelNotFound,
    #[error("{} LLM provider is not configured.", .0.name().0)]
    ProviderNotAuthenticated(Arc<dyn LanguageModelProvider>),
}

impl std::fmt::Debug for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoProvider => write!(f, "NoProvider"),
            Self::ModelNotFound => write!(f, "ModelNotFound"),
            Self::ProviderNotAuthenticated(provider) => {
                write!(f, "ProviderNotAuthenticated({})", provider.id())
            }
        }
    }
}

#[derive(Default)]
pub struct LanguageModelRegistry {
    /// True if the user has *NO* default model configured in settings
    should_use_fallback: bool,
    default_model: Option<LanguageModel>,
    /// This model is automatically configured by a user's environment after
    /// authenticating all providers. It's only used when `default_model` is not set.
    available_fallback_model: Option<LanguageModel>,
    inline_assistant_model: Option<LanguageModel>,
    commit_message_model: Option<LanguageModel>,
    thread_summary_model: Option<LanguageModel>,
    compaction_model: Option<LanguageModel>,
    providers: BTreeMap<LanguageModelProviderId, Arc<dyn LanguageModelProvider>>,
    authentication_tasks: BTreeMap<LanguageModelProviderId, Task<()>>,
    inline_alternatives: Vec<LanguageModel>,
}

#[derive(Debug)]
pub struct SelectedModel {
    pub provider: LanguageModelProviderId,
    pub model: LanguageModelId,
}

impl FromStr for SelectedModel {
    type Err = String;

    /// Parse string identifiers like `provider_id/model_id` into a `SelectedModel`
    fn from_str(id: &str) -> Result<SelectedModel, Self::Err> {
        let Some((provider_id, model_id)) = id.split_once('/') else {
            return Err(format!(
                "Invalid model identifier format: `{}`. Expected `provider_id/model_id`",
                id
            ));
        };

        if provider_id.is_empty() || model_id.is_empty() {
            return Err(format!("Provider and model ids can't be empty: `{}`", id));
        }

        Ok(SelectedModel {
            provider: LanguageModelProviderId(provider_id.to_string().into()),
            model: LanguageModelId(model_id.to_string().into()),
        })
    }
}

impl LanguageModel {
    /// Whether `other` is the same model from the same provider, regardless
    /// of changes to its capabilities.
    pub fn is_same_as(&self, other: &LanguageModel) -> bool {
        self.id == other.id && self.provider_id == other.provider_id
    }
}

pub enum Event {
    DefaultModelChanged,
    InlineAssistantModelChanged,
    CommitMessageModelChanged,
    CompactionModelChanged,
    ThreadSummaryModelChanged,
    ProviderStateChanged(LanguageModelProviderId),
    AddedProvider(LanguageModelProviderId),
    RemovedProvider(LanguageModelProviderId),
    /// Emitted when the set of available providers changes.
    ProvidersChanged,
}

impl EventEmitter<Event> for LanguageModelRegistry {}

impl LanguageModelRegistry {
    pub fn global(cx: &App) -> Entity<Self> {
        cx.global::<GlobalLanguageModelRegistry>().0.clone()
    }

    pub fn read_global(cx: &App) -> &Self {
        cx.global::<GlobalLanguageModelRegistry>().0.read(cx)
    }

    #[cfg(any(test, feature = "test-support"))]
    /// Installs a registry whose only provider is a fake, with its model as
    /// the default, and returns the fake.
    pub fn test(cx: &mut App) -> Arc<crate::fake_provider::FakeLanguageModelProvider> {
        let fake_provider = Arc::new(crate::fake_provider::FakeLanguageModelProvider::default());
        let registry = cx.new(|cx| {
            let mut registry = Self::default();
            registry.register_provider(fake_provider.clone(), cx);
            registry.set_default_model(Some(fake_provider.model("fake")), cx);
            registry
        });
        cx.set_global(GlobalLanguageModelRegistry(registry));
        fake_provider
    }

    pub fn set_should_use_fallback(&mut self, value: bool) {
        self.should_use_fallback = value;
    }

    pub fn register_provider<T: LanguageModelProvider + LanguageModelProviderState>(
        &mut self,
        provider: Arc<T>,
        cx: &mut Context<Self>,
    ) {
        let id = provider.id();

        let subscription = provider.subscribe(cx, {
            let id = id.clone();
            move |_, cx| {
                cx.emit(Event::ProviderStateChanged(id.clone()));
            }
        });
        if let Some(subscription) = subscription {
            subscription.detach();
        }

        let provider: Arc<dyn LanguageModelProvider> = provider;
        if self
            .providers
            .get(&id)
            .is_none_or(|registered| !Arc::ptr_eq(registered, &provider))
        {
            self.authentication_tasks.remove(&id);
        }
        self.providers.insert(id.clone(), provider);
        cx.emit(Event::AddedProvider(id));
    }

    pub fn unregister_provider(&mut self, id: LanguageModelProviderId, cx: &mut Context<Self>) {
        if self.providers.remove(&id).is_some() {
            self.authentication_tasks.remove(&id);
            cx.emit(Event::RemovedProvider(id));
        }
    }

    pub fn authenticate_providers(&mut self, cx: &mut Context<Self>) {
        self.authenticate_providers_with_retry(false, cx);
    }

    pub fn retry_authentication(&mut self, cx: &mut Context<Self>) {
        self.authenticate_providers_with_retry(true, cx);
    }

    pub fn providers(&self) -> Vec<Arc<dyn LanguageModelProvider>> {
        self.providers.values().cloned().collect()
    }

    /// Returns all registered providers.
    pub fn visible_providers(&self) -> Vec<Arc<dyn LanguageModelProvider>> {
        self.providers()
    }

    pub fn configuration_error(
        &self,
        model: Option<LanguageModel>,
        cx: &App,
    ) -> Option<ConfigurationError> {
        let Some(model) = model else {
            if !self.has_authenticated_provider(cx) {
                return Some(ConfigurationError::NoProvider);
            }
            return Some(ConfigurationError::ModelNotFound);
        };

        let Some(provider) = self.provider(&model.provider_id) else {
            return Some(ConfigurationError::ModelNotFound);
        };
        if !provider.is_authenticated(cx) {
            return Some(ConfigurationError::ProviderNotAuthenticated(provider));
        }

        None
    }

    /// Returns `true` if at least one provider that is authenticated.
    pub fn has_authenticated_provider(&self, cx: &App) -> bool {
        self.providers.values().any(|p| p.is_authenticated(cx))
    }

    pub fn available_models<'a>(&'a self, cx: &'a App) -> impl Iterator<Item = LanguageModel> + 'a {
        self.providers
            .values()
            .filter(|provider| provider.is_authenticated(cx))
            .flat_map(|provider| provider.provided_models(cx))
    }

    pub fn provider(&self, id: &LanguageModelProviderId) -> Option<Arc<dyn LanguageModelProvider>> {
        self.providers.get(id).cloned()
    }

    /// The registered provider that serves `model`.
    ///
    /// # Errors
    ///
    /// Returns [`LanguageModelCompletionError::ModelUnavailable`] when no
    /// provider is registered under the model's provider id.
    pub fn provider_for_model(
        &self,
        model: &LanguageModel,
    ) -> Result<Arc<dyn LanguageModelProvider>, LanguageModelCompletionError> {
        self.provider(&model.provider_id)
            .ok_or_else(|| unavailable_error(model))
    }

    pub fn select_default_model(&mut self, model: Option<&SelectedModel>, cx: &mut Context<Self>) {
        let configured_model = model.and_then(|model| self.select_model(model, cx));
        self.set_default_model(configured_model, cx);
    }

    pub fn select_inline_assistant_model(
        &mut self,
        model: Option<&SelectedModel>,
        cx: &mut Context<Self>,
    ) {
        let configured_model = model.and_then(|model| self.select_model(model, cx));
        self.set_inline_assistant_model(configured_model, cx);
    }

    pub fn select_commit_message_model(
        &mut self,
        model: Option<&SelectedModel>,
        cx: &mut Context<Self>,
    ) {
        let configured_model = model.and_then(|model| self.select_model(model, cx));
        self.set_commit_message_model(configured_model, cx);
    }

    pub fn select_thread_summary_model(
        &mut self,
        model: Option<&SelectedModel>,
        cx: &mut Context<Self>,
    ) {
        let configured_model = model.and_then(|model| self.select_model(model, cx));
        self.set_thread_summary_model(configured_model, cx);
    }

    pub fn select_compaction_model(
        &mut self,
        model: Option<&SelectedModel>,
        cx: &mut Context<Self>,
    ) {
        let configured_model = model.and_then(|model| self.select_model(model, cx));
        self.set_compaction_model(configured_model, cx);
    }

    /// Selects and sets the inline alternatives for language models based on
    /// provider name and id.
    pub fn select_inline_alternative_models(
        &mut self,
        alternatives: impl IntoIterator<Item = SelectedModel>,
        cx: &mut Context<Self>,
    ) {
        self.inline_alternatives = alternatives
            .into_iter()
            .flat_map(|alternative| self.select_model(&alternative, cx))
            .collect::<Vec<_>>();
    }

    pub fn select_model(
        &mut self,
        selected_model: &SelectedModel,
        cx: &mut Context<Self>,
    ) -> Option<LanguageModel> {
        self.provider(&selected_model.provider)?
            .provided_models(cx)
            .into_iter()
            .find(|model| model.id == selected_model.model)
    }

    pub fn set_default_model(&mut self, model: Option<LanguageModel>, cx: &mut Context<Self>) {
        match (self.default_model(), model.as_ref()) {
            (Some(old), Some(new)) if old.is_same_as(new) => {}
            (None, None) => {}
            _ => cx.emit(Event::DefaultModelChanged),
        }
        self.default_model = model;
    }

    pub fn refresh_fallback_model(&mut self, cx: &mut Context<Self>) {
        // If the fallback model was already set or we don't want to use it, do nothing
        if !self.should_use_fallback || self.available_fallback_model.is_some() {
            return;
        }

        let fallback_model = self
            .providers()
            .iter()
            .filter(|provider| provider.is_authenticated(cx))
            .find_map(|provider| {
                provider
                    .default_model(cx)
                    .or_else(|| provider.recommended_models(cx).first().cloned())
            });

        self.set_fallback_model(fallback_model, cx);
    }

    fn set_fallback_model(&mut self, model: Option<LanguageModel>, cx: &mut Context<Self>) {
        if self.default_model.is_none() {
            match (self.available_fallback_model.as_ref(), model.as_ref()) {
                (Some(old), Some(new)) if old.is_same_as(new) => {}
                (None, None) => {}
                _ => cx.emit(Event::DefaultModelChanged),
            }
        }
        self.available_fallback_model = model;
    }

    pub fn set_inline_assistant_model(
        &mut self,
        model: Option<LanguageModel>,
        cx: &mut Context<Self>,
    ) {
        match (self.inline_assistant_model.as_ref(), model.as_ref()) {
            (Some(old), Some(new)) if old.is_same_as(new) => {}
            (None, None) => {}
            _ => cx.emit(Event::InlineAssistantModelChanged),
        }
        self.inline_assistant_model = model;
    }

    pub fn set_commit_message_model(
        &mut self,
        model: Option<LanguageModel>,
        cx: &mut Context<Self>,
    ) {
        match (self.commit_message_model.as_ref(), model.as_ref()) {
            (Some(old), Some(new)) if old.is_same_as(new) => {}
            (None, None) => {}
            _ => cx.emit(Event::CommitMessageModelChanged),
        }
        self.commit_message_model = model;
    }

    pub fn set_thread_summary_model(
        &mut self,
        model: Option<LanguageModel>,
        cx: &mut Context<Self>,
    ) {
        match (self.thread_summary_model.as_ref(), model.as_ref()) {
            (Some(old), Some(new)) if old.is_same_as(new) => {}
            (None, None) => {}
            _ => cx.emit(Event::ThreadSummaryModelChanged),
        }
        self.thread_summary_model = model;
    }

    pub fn set_compaction_model(&mut self, model: Option<LanguageModel>, cx: &mut Context<Self>) {
        match (self.compaction_model.as_ref(), model.as_ref()) {
            (Some(old), Some(new)) if old.is_same_as(new) => {}
            (None, None) => {}
            _ => cx.emit(Event::CompactionModelChanged),
        }
        self.compaction_model = model;
    }

    pub fn default_model(&self) -> Option<LanguageModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZAPLY_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.default_model.clone().or_else(|| {
            if self.should_use_fallback {
                self.available_fallback_model.clone()
            } else {
                None
            }
        })
    }

    pub fn default_fast_model(&self, cx: &App) -> Option<LanguageModel> {
        let default_model = self.default_model()?;
        self.provider(&default_model.provider_id)?
            .default_fast_model(cx)
    }

    pub fn inline_assistant_model(&self) -> Option<LanguageModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZAPLY_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.inline_assistant_model
            .clone()
            .or_else(|| self.default_model())
    }

    pub fn commit_message_model(&self, cx: &App) -> Option<LanguageModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZAPLY_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.commit_message_model
            .clone()
            .or_else(|| self.default_fast_model(cx))
            .or_else(|| self.default_model())
    }

    pub fn thread_summary_model(&self, cx: &App) -> Option<LanguageModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZAPLY_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.thread_summary_model
            .clone()
            .or_else(|| self.default_fast_model(cx))
            .or_else(|| self.default_model())
    }

    /// Returns the configured compaction model without falling back through
    /// `default_fast_model`/`default_model`. Callers that want a fallback to
    /// the thread's primary model should handle `None` themselves.
    pub fn compaction_model(&self) -> Option<LanguageModel> {
        #[cfg(debug_assertions)]
        if std::env::var("ZAPLY_SIMULATE_NO_LLM_PROVIDER").is_ok() {
            return None;
        }

        self.compaction_model.clone()
    }

    /// The models to use for inline assists. Returns the union of the active
    /// model and all inline alternatives. When there are multiple models, the
    /// user will be able to cycle through results.
    pub fn inline_alternative_models(&self) -> &[LanguageModel] {
        &self.inline_alternatives
    }

    fn authenticate_providers_with_retry(&mut self, retry: bool, cx: &mut Context<Self>) {
        for provider in self.visible_providers() {
            let provider_id = provider.id();
            if let Some(task) = self.authentication_tasks.get(&provider_id) {
                if !retry {
                    let registry = cx.weak_entity();
                    cx.defer(move |cx| {
                        if registry
                            .read_with(cx, |registry, _| {
                                registry
                                    .providers
                                    .get(&provider_id)
                                    .is_some_and(|registered| Arc::ptr_eq(registered, &provider))
                            })
                            .unwrap_or(false)
                        {
                            provider.refresh_models(cx);
                        }
                    });
                    continue;
                }
                if !task.is_ready() {
                    continue;
                }
            }
            if retry && provider.is_authenticated(cx) {
                continue;
            }
            self.authentication_tasks.insert(
                provider_id.clone(),
                cx.spawn(async move |registry, cx| {
                    if let Err(error) = cx.update(|cx| provider.authenticate(cx)).await {
                        match error {
                            AuthenticateError::CredentialsNotFound => {
                                // Since we're authenticating these providers in the
                                // background for the purposes of populating the
                                // language selector, we don't care about providers
                                // where the credentials are not found.
                            }
                            AuthenticateError::ConnectionRefused => {
                                // Not logging connection refused errors as they are mostly from LM Studio's noisy auth failures.
                                // LM Studio only has one auth method (endpoint call) which fails for users who haven't enabled it.
                                // TODO: Better manage LM Studio auth logic to avoid these noisy failures.
                            }
                            _ => {
                                // Some providers have noisy failure states that we
                                // don't want to spam the logs with every time the
                                // language model selector is initialized.
                                //
                                // Ideally these should have more clear failure modes
                                // that we know are safe to ignore here, like what we do
                                // with `CredentialsNotFound` above.
                                match provider_id.0.as_ref() {
                                    "lmstudio" | "ollama" => {
                                        // LM Studio and Ollama both make fetch requests to the local APIs to determine if they are "authenticated".
                                        //
                                        // These fail noisily, so we don't log them.
                                    }
                                    "copilot_chat" => {
                                        // Copilot Chat returns an error if Copilot is not enabled, so we don't log those errors.
                                    }
                                    _ => log::error!(
                                        "Failed to authenticate provider: {}: {error:#}",
                                        provider.name().0
                                    ),
                                }
                            }
                        }
                    }
                    registry
                        .update(cx, |registry, cx| registry.refresh_fallback_model(cx))
                        .ok();
                }),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_provider::FakeLanguageModelProvider;

    #[test]
    fn selected_model_allows_slashes_in_model_id() {
        let selected = SelectedModel::from_str("custom-provider/organization/model-name")
            .expect("model identifier should parse");

        assert_eq!(
            selected.provider,
            LanguageModelProviderId("custom-provider".into())
        );
        assert_eq!(
            selected.model,
            LanguageModelId("organization/model-name".into())
        );
    }

    #[test]
    fn selected_model_rejects_missing_separator_or_empty_parts() {
        assert!(SelectedModel::from_str("custom-provider").is_err());
        assert!(SelectedModel::from_str("/organization/model-name").is_err());
        assert!(SelectedModel::from_str("custom-provider/").is_err());
    }

    #[gpui::test]
    fn test_register_providers(cx: &mut App) {
        let registry = cx.new(|_| LanguageModelRegistry::default());

        let provider = Arc::new(FakeLanguageModelProvider::default());
        registry.update(cx, |registry, cx| {
            registry.register_provider(provider.clone(), cx);
        });

        let providers = registry.read(cx).providers();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].id(), provider.id());

        registry.update(cx, |registry, cx| {
            registry.unregister_provider(provider.id(), cx);
        });

        let providers = registry.read(cx).providers();
        assert!(providers.is_empty());
    }

    #[gpui::test]

    #[gpui::test]

    #[gpui::test]

    #[gpui::test]
    async fn test_configure_fallback_model(cx: &mut gpui::TestAppContext) {
        let registry = cx.new(|_| LanguageModelRegistry::default());

        let provider = Arc::new(FakeLanguageModelProvider::default());
        registry.update(cx, |registry, cx| {
            registry.register_provider(provider.clone(), cx);
        });

        cx.update(|cx| provider.authenticate(cx)).await.unwrap();

        registry.update(cx, |registry, cx| {
            let provider = registry.provider(&provider.id()).unwrap();
            let model = provider.default_model(cx).unwrap();

            registry.set_fallback_model(Some(model.clone()), cx);

            assert!(registry.default_model().is_none());
            assert!(registry.inline_assistant_model().is_none());

            registry.set_should_use_fallback(true);

            let default_model = registry.default_model().unwrap();
            assert_eq!(default_model.id, model.id);
            assert_eq!(default_model.provider_id, provider.id());
            assert!(
                registry
                    .inline_assistant_model()
                    .is_some_and(|inline_model| inline_model.is_same_as(&default_model))
            );
        });
    }

    #[gpui::test]
    fn test_inline_assistant_model_precedence(cx: &mut App) {
        let registry = cx.new(|_| LanguageModelRegistry::default());
        let provider = FakeLanguageModelProvider::default();
        let [inline_model, default_model, fallback_model] =
            ["inline", "default", "fallback"].map(|model_id| provider.model(model_id));

        registry.update(cx, |registry, cx| {
            registry.set_should_use_fallback(true);
            registry.set_fallback_model(Some(fallback_model.clone()), cx);
            registry.set_default_model(Some(default_model.clone()), cx);
            registry.set_inline_assistant_model(Some(inline_model.clone()), cx);

            assert!(
                registry
                    .inline_assistant_model()
                    .is_some_and(|model| model.is_same_as(&inline_model))
            );

            registry.set_inline_assistant_model(None, cx);
            assert!(
                registry
                    .inline_assistant_model()
                    .is_some_and(|model| model.is_same_as(&default_model))
            );

            registry.set_default_model(None, cx);
            assert!(
                registry
                    .inline_assistant_model()
                    .is_some_and(|model| model.is_same_as(&fallback_model))
            );

            registry.set_should_use_fallback(false);
            assert!(registry.inline_assistant_model().is_none());
        });
    }
}
