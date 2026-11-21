pub use env_var::{EnvVar, bool_env_var, env_var};
use std::sync::LazyLock;

/// Whether Zaply is running in stateless mode.
/// When true, Zaply will use in-memory databases instead of persistent storage.
pub static ZAPLY_STATELESS: LazyLock<bool> = bool_env_var!("ZAPLY_STATELESS");
