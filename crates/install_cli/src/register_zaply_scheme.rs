use client::ZAPLY_URL_SCHEME;
use gpui::{AsyncApp, actions};

actions!(
    cli,
    [
        /// Registers the zaply:// URL scheme handler.
        RegisterZaplyScheme
    ]
);

pub async fn register_zaply_scheme(cx: &AsyncApp) -> anyhow::Result<()> {
    cx.update(|cx| cx.register_url_scheme(ZAPLY_URL_SCHEME)).await
}
