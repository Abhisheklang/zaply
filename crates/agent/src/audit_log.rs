//! A persistent, on-device audit log of agent activity.
//!
//! Every tool call the built-in agent runs, and every permission decision
//! made for a tool call, is appended as one JSON object per line to a file
//! in the data directory. The log never leaves the device; it exists so the
//! user can review exactly what the agent did. It can be opened from within
//! Zaply with the "Open Agent Audit Log" action.

use std::io::Write as _;
use std::time::{SystemTime, UNIX_EPOCH};

/// The most characters of tool input detail stored in a single audit entry.
/// Inputs contain file paths and terminal commands, which are what an audit
/// needs, but there is no reason to store unbounded input text.
const MAX_DETAIL_LENGTH: usize = 300;

/// Appends one entry to the audit log. Recording is best-effort: a failure
/// to write the log must never interrupt the agent, so errors are logged
/// and otherwise ignored.
pub fn record(category: &str, tool_name: &str, detail: &str, outcome: &str) {
    let timestamp_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    let detail: String = detail.chars().take(MAX_DETAIL_LENGTH).collect();
    let entry = serde_json::json!({
        "timestamp": timestamp_seconds,
        "category": category,
        "tool": tool_name,
        "detail": detail,
        "outcome": outcome,
    });
    let path = paths::agent_audit_log_file();
    if let Some(directory) = path.parent()
        && let Err(error) = std::fs::create_dir_all(directory)
    {
        log::debug!("failed to create directory for agent audit log: {error}");
        return;
    }
    match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        Ok(mut file) => {
            if let Err(error) = writeln!(file, "{entry}") {
                log::debug!("failed to write agent audit log entry: {error}");
            }
        }
        Err(error) => {
            log::debug!("failed to open agent audit log: {error}");
        }
    }
}
