use std::sync::Arc;

use client::{Client, UserStore};
use gpui::{Entity, IntoElement, ParentElement};
use language_model::LanguageModelRegistry;
use ui::prelude::*;

use crate::{AgentPanelOnboardingCard, ApiKeysWithoutProviders};

pub struct AgentPanelOnboarding {
    user_store: Entity<UserStore>,
    client: Arc<Client>,
    has_configured_providers: bool,
    continue_with_zaply_ai: Arc<dyn Fn(&mut Window, &mut App)>,
}

impl AgentPanelOnboarding {
    pub fn new(
        user_store: Entity<UserStore>,
        client: Arc<Client>,
        continue_with_zaply_ai: impl Fn(&mut Window, &mut App) + 'static,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.subscribe(
            &LanguageModelRegistry::global(cx),
            |this: &mut Self, _registry, event: &language_model::Event, cx| match event {
                language_model::Event::ProviderStateChanged(_)
                | language_model::Event::AddedProvider(_)
                | language_model::Event::RemovedProvider(_)
                | language_model::Event::ProvidersChanged => {
                    this.has_configured_providers = Self::has_configured_providers(cx)
                }
                _ => {}
            },
        )
        .detach();

        Self {
            user_store,
            client,
            has_configured_providers: Self::has_configured_providers(cx),
            continue_with_zaply_ai: Arc::new(continue_with_zaply_ai),
        }
    }

    fn has_configured_providers(cx: &App) -> bool {
        LanguageModelRegistry::read_global(cx)
            .visible_providers()
            .iter()
            .any(|provider| provider.is_authenticated(cx))
    }
}

impl Render for AgentPanelOnboarding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Zaply change (2026-11-21): the upstream onboarding pushed
        // hosted plans and trials. Zaply has no accounts and no hosted
        // plans, so the first-run card simply welcomes the user and
        // points at the bring-your-own-key provider setup below.
        let dismiss_callback = self.continue_with_zaply_ai.clone();

        AgentPanelOnboardingCard::new()
            .child(
                v_flex()
                    .relative()
                    .min_w_0()
                    .gap_1()
                    .child(Headline::new("Welcome to Zaply"))
                    .child(
                        Label::new(
                            "Zaply works with the AI provider you choose. Add your own \
                             API key, or use a local model. There is no Zaply account \
                             and nothing to sign in to.",
                        )
                        .color(Color::Muted)
                        .mb_2(),
                    )
                    .child(
                        h_flex().justify_end().child(
                            Button::new("dismiss_ai_onboarding", "Dismiss")
                                .label_size(LabelSize::Small)
                                .on_click(move |_, window, cx| dismiss_callback(window, cx)),
                        ),
                    ),
            )
            .map(|this| {
                if self.has_configured_providers {
                    this
                } else {
                    this.child(ApiKeysWithoutProviders::new())
                }
            })
    }
}
