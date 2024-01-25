use gpui::{actions, AppContext, ClipboardItem, PromptLevel};
use system_specs::SystemSpecs;
use workspace::Workspace;

pub mod deploy_feedback_button;
pub mod feedback_modal;

actions!(feedback, [GiveFeedback, SubmitFeedback]);

mod system_specs;

actions!(
    zed,
    [
        CopySystemSpecsIntoClipboard,
        FileBugReport,
        RequestFeature,
        OpenZedCommunityRepo
    ]
);

pub fn init(cx: &mut AppContext) {
    // TODO: a way to combine these two into one?
    cx.observe_new_views(feedback_modal::FeedbackModal::register)
        .detach();

    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace
            .register_action(|_, _: &CopySystemSpecsIntoClipboard, cx| {
                    let specs = SystemSpecs::new(&cx).to_string();

                    let prompt = cx.prompt(
                        PromptLevel::Info,
                        "Copied into clipboard",
                        Some(&specs),
                        &["OK"],
                    );
                    cx.spawn(|_, _cx| async move {
                        prompt.await.ok();
                    })
                    .detach();
                    let item = ClipboardItem::new(specs.clone());
                    cx.write_to_clipboard(item);
                })
            .register_action(|_, _: &RequestFeature, cx| {
                let url = "https://github.com/zed-industries/community/issues/new?assignees=&labels=enhancement%2Ctriage&template=0_feature_request.yml";
                cx.open_url(url);
            })
            .register_action(move |_, _: &FileBugReport, cx| {
                let url = format!(
                    "https://github.com/zed-industries/community/issues/new?assignees=&labels=defect%2Ctriage&template=2_bug_report.yml&environment={}",
                    urlencoding::encode(&SystemSpecs::new(&cx).to_string())
                );
                cx.open_url(&url);
            })
            .register_action(move |_, _: &OpenZedCommunityRepo, cx| {
                let url = "https://github.com/zed-industries/community";
                cx.open_url(&url);
        });
    })
    .detach();
}
