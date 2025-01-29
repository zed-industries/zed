use gpui::{actions, AppContext, ClipboardItem, PromptLevel};
use system_specs::SystemSpecs;
use util::ResultExt;
use workspace::Workspace;

pub mod feedback_modal;

mod system_specs;

actions!(
    zed,
    [
        CopySystemSpecsIntoClipboard,
        FileBugReport,
        RequestFeature,
        OpenZedRepo
    ]
);

const fn zed_repo_url() -> &'static str {
    "https://github.com/zed-industries/zed"
}

fn request_feature_url() -> String {
    "https://github.com/zed-industries/zed/discussions/new/choose".to_string()
}

fn file_bug_report_url(specs: &SystemSpecs) -> String {
    format!(
        concat!(
            "https://github.com/zed-industries/zed/issues/new",
            "?",
            "template=1_bug_report.yml",
            "&",
            "environment={}"
        ),
        urlencoding::encode(&specs.to_string())
    )
}

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, cx| {
        feedback_modal::FeedbackModal::register(workspace, cx);
        workspace
            .register_action(|_, _: &CopySystemSpecsIntoClipboard, cx| {
                let specs = SystemSpecs::new(cx);

                cx.spawn(|_, mut cx| async move {
                    let specs = specs.await.to_string();

                    cx.update(|cx| cx.write_to_clipboard(ClipboardItem::new_string(specs.clone())))
                        .log_err();

                    cx.prompt(
                        PromptLevel::Info,
                        "Copied into clipboard",
                        Some(&specs),
                        &["OK"],
                    )
                    .await
                    .ok();
                })
                .detach();
            })
            .register_action(|_, _: &RequestFeature, cx| {
                cx.spawn(|_, mut cx| async move {
                    cx.update(|cx| {
                        cx.open_url(&request_feature_url());
                    })
                    .log_err();
                })
                .detach();
            })
            .register_action(move |_, _: &FileBugReport, cx| {
                let specs = SystemSpecs::new(cx);
                cx.spawn(|_, mut cx| async move {
                    let specs = specs.await;
                    cx.update(|cx| {
                        cx.open_url(&file_bug_report_url(&specs));
                    })
                    .log_err();
                })
                .detach();
            })
            .register_action(move |_, _: &OpenZedRepo, cx| {
                cx.open_url(zed_repo_url());
            });
    })
    .detach();
}
