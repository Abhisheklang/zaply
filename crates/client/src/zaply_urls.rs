//! Contains helper functions for constructing URLs to various Zaply pages.
//!
//! Zaply change (2026-11-21): Zaply has no account system, no hosted
//! services, and no separate documentation site. Every link that used to
//! point at the upstream hosted service now points at the Zaply project
//! repository on GitHub instead. These functions keep their upstream
//! signatures so callers do not change; several of them (trial, upgrade,
//! account) exist only because upstream UI referenced them, and their
//! surfaces have been removed from Zaply.

use gpui::App;

/// The Zaply project repository on GitHub.
const REPOSITORY_URL: &str = "https://github.com/Abhisheklang/zaply";

fn repository_url() -> String {
    REPOSITORY_URL.to_string()
}

/// Returns the URL of the Zaply project repository. There are no user
/// accounts in Zaply; this replaces the upstream account page link.
pub fn account_url(_cx: &App) -> String {
    repository_url()
}

/// Returns the URL of the Zaply project repository. Zaply has no trials;
/// this replaces the upstream start-trial link.
pub fn start_trial_url(_cx: &App) -> String {
    repository_url()
}

/// Returns the URL of the Zaply project repository. Zaply has no paid
/// plans; this replaces the upstream upgrade link.
pub fn upgrade_to_zaply_pro_url(_cx: &App) -> String {
    repository_url()
}

/// Returns the URL of the Zaply project repository, whose licence section
/// carries the terms that apply to Zaply itself.
pub fn terms_of_service(_cx: &App) -> String {
    repository_url()
}

/// Returns the URL of the Zaply project repository. Zaply's privacy
/// stance (no account, no telemetry, bring your own API key) is stated
/// in the repository README.
pub fn ai_privacy_and_security(_cx: &App) -> String {
    repository_url()
}

/// Returns the URL of the Zaply project repository, which documents the
/// edit prediction providers Zaply supports.
pub fn edit_prediction_docs(_cx: &App) -> String {
    repository_url()
}

/// Returns the URL of the Zaply project repository.
pub fn skills_docs(_cx: &App) -> String {
    repository_url()
}

/// Returns the URL of the Zaply project repository.
///
/// Pass `section` to mirror the upstream signature; Zaply documentation
/// lives in the repository, so every section resolves to the same place.
pub fn sandboxing_docs(_section: Option<&str>, _cx: &App) -> String {
    repository_url()
}

/// Returns the URL of the Zaply project repository, which lists the
/// language model providers Zaply works with.
pub fn llm_provider_docs(_cx: &App) -> String {
    repository_url()
}

/// Returns the URL of the Zaply project repository.
pub fn acp_registry_blog(_cx: &App) -> String {
    repository_url()
}

pub fn shared_agent_thread_url(session_id: &str) -> String {
    format!("zaply://agent/shared/{}", session_id)
}
