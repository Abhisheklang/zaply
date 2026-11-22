use client::telemetry;
use gpui::{App, ClipboardItem, PromptLevel, actions};
use system_specs::{CopySystemSpecsIntoClipboard, SystemSpecs};
use util::ResultExt;
use workspace::Workspace;
use zaply_actions::feedback::{FileBugReport, RequestFeature};

actions!(
    zaply,
    [
        /// Opens the Zaply repository on GitHub.
        OpenZaplyRepo
    ]
);

const ZAPLY_REPO_URL: &str = "https://github.com/Abhisheklang/zaply";

const REQUEST_FEATURE_URL: &str = "https://github.com/Abhisheklang/zaply/issues";

fn file_bug_report_url(specs: &SystemSpecs) -> String {
    format!(
        "https://github.com/Abhisheklang/zaply/issues/new?body={}",
        urlencoding::encode(&specs.to_string())
    )
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace
            .register_action(|_, _: &CopySystemSpecsIntoClipboard, window, cx| {
                let os_name = telemetry::os_name(cx);
                let specs = SystemSpecs::new(window, cx, os_name, telemetry::os_version());

                cx.spawn_in(window, async move |_, cx| {
                    let specs = specs.await.to_string();

                    cx.update(|_, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(specs.clone()))
                    })
                    .log_err();

                    cx.prompt(
                        PromptLevel::Info,
                        "Copied into clipboard",
                        Some(&specs),
                        &["OK"],
                    )
                    .await
                })
                .detach();
            })
            .register_action(|_, _: &RequestFeature, _, cx| {
                cx.open_url(REQUEST_FEATURE_URL);
            })
            .register_action(move |_, _: &FileBugReport, window, cx| {
                let os_name = telemetry::os_name(cx);
                let specs = SystemSpecs::new(window, cx, os_name, telemetry::os_version());
                cx.spawn_in(window, async move |_, cx| {
                    let specs = specs.await;
                    cx.update(|_, cx| {
                        cx.open_url(&file_bug_report_url(&specs));
                    })
                    .log_err();
                })
                .detach();
            })
            .register_action(move |_, _: &OpenZaplyRepo, _, cx| {
                cx.open_url(ZAPLY_REPO_URL);
            });
    })
    .detach();
}
