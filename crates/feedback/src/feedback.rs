use std::sync::Arc;

pub mod feedback_editor;
mod system_specs;
use gpui::{actions, impl_actions, ClipboardItem, MutableAppContext, PromptLevel, ViewContext};
use serde::Deserialize;
use system_specs::SystemSpecs;
use workspace::{AppState, Workspace};

#[derive(Deserialize, Clone, PartialEq)]
pub struct OpenBrowser {
    pub url: Arc<str>,
}

impl_actions!(zed, [OpenBrowser]);

actions!(
    zed,
    [CopySystemSpecsIntoClipboard, FileBugReport, RequestFeature]
);

pub fn init(app_state: Arc<AppState>, cx: &mut MutableAppContext) {
    let system_specs = SystemSpecs::new(&cx);
    let system_specs_text = system_specs.to_string();

    feedback_editor::init(system_specs, app_state, cx);

    cx.add_global_action(move |action: &OpenBrowser, cx| cx.platform().open_url(&action.url));

    let url = format!(
        "https://github.com/zed-industries/feedback/issues/new?assignees=&labels=defect%2Ctriage&template=2_bug_report.yml&environment={}", 
        urlencoding::encode(&system_specs_text)
    );

    cx.add_action(
        move |_: &mut Workspace,
              _: &CopySystemSpecsIntoClipboard,
              cx: &mut ViewContext<Workspace>| {
            cx.prompt(
                PromptLevel::Info,
                &format!("Copied into clipboard:\n\n{system_specs_text}"),
                &["OK"],
            );
            let item = ClipboardItem::new(system_specs_text.clone());
            cx.write_to_clipboard(item);
        },
    );

    cx.add_action(
        |_: &mut Workspace, _: &RequestFeature, cx: &mut ViewContext<Workspace>| {
            let url = "https://github.com/zed-industries/feedback/issues/new?assignees=&labels=enhancement%2Ctriage&template=0_feature_request.yml";
            cx.dispatch_action(OpenBrowser {
                url: url.into(),
            });
        },
    );

    cx.add_action(
        move |_: &mut Workspace, _: &FileBugReport, cx: &mut ViewContext<Workspace>| {
            cx.dispatch_action(OpenBrowser {
                url: url.clone().into(),
            });
        },
    );
}
