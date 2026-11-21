//! Local-only API usage counts.
//!
//! Every time a provider reports token usage for an agent request, the
//! delta is added to a small JSON file in the user's data directory,
//! bucketed by provider, model, and day. The counts never leave the
//! device: nothing here makes a network call, and the file exists only
//! so the user can see what their own API keys have been used for. The
//! "API Usage" view (opened from the `OpenApiUsage` action) renders this
//! file as text.

use language_model::TokenUsage;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UsageEntry {
    pub provider: String,
    pub model: String,
    pub day: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct UsageFile {
    entries: Vec<UsageEntry>,
}

fn usage_file_state() -> &'static Mutex<UsageFile> {
    static STATE: OnceLock<Mutex<UsageFile>> = OnceLock::new();
    STATE.get_or_init(|| {
        let loaded = std::fs::read_to_string(paths::api_usage_file())
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default();
        Mutex::new(loaded)
    })
}

/// Records one token-usage delta for the given provider and model.
/// Errors are only debug-logged: usage accounting must never interrupt
/// the agent.
pub fn record_usage(provider: &str, model: &str, usage: TokenUsage) {
    if usage.input_tokens == 0
        && usage.output_tokens == 0
        && usage.cache_creation_input_tokens == 0
        && usage.cache_read_input_tokens == 0
    {
        return;
    }
    let day = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let Ok(mut state) = usage_file_state().lock() else {
        return;
    };
    let entry = match state
        .entries
        .iter_mut()
        .find(|entry| entry.provider == provider && entry.model == model && entry.day == day)
    {
        Some(entry) => entry,
        None => {
            state.entries.push(UsageEntry {
                provider: provider.to_string(),
                model: model.to_string(),
                day,
                ..Default::default()
            });
            state.entries.last_mut().expect("just pushed")
        }
    };
    entry.input_tokens += usage.input_tokens;
    entry.output_tokens += usage.output_tokens;
    entry.cache_creation_input_tokens += usage.cache_creation_input_tokens;
    entry.cache_read_input_tokens += usage.cache_read_input_tokens;
    match serde_json::to_string_pretty(&*state) {
        Ok(rendered) => {
            if let Err(error) = std::fs::write(paths::api_usage_file(), rendered) {
                log::debug!("Failed to write the API usage file: {error}");
            }
        }
        Err(error) => {
            log::debug!("Failed to serialize API usage counts: {error}");
        }
    }
}
