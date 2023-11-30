use gpui::AppContext;

pub mod deploy_feedback_button;
pub mod feedback_editor;
pub mod feedback_info_text;
pub mod feedback_modal;
pub mod submit_feedback_button;

mod system_specs;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(feedback_modal::FeedbackModal::register)
        .detach();
}

// actions!(
//     zed,
//     [
//         CopySystemSpecsIntoClipboard,
//         FileBugReport,
//         RequestFeature,
//         OpenZedCommunityRepo
//     ]
// );

// pub fn init(cx: &mut AppContext) {
//     feedback_editor::init(cx);

//     cx.add_action(
//         move |_: &mut Workspace,
//               _: &CopySystemSpecsIntoClipboard,
//               cx: &mut ViewContext<Workspace>| {
//             let specs = SystemSpecs::new(&cx).to_string();
//             cx.prompt(
//                 PromptLevel::Info,
//                 &format!("Copied into clipboard:\n\n{specs}"),
//                 &["OK"],
//             );
//             let item = ClipboardItem::new(specs.clone());
//             cx.write_to_clipboard(item);
//         },
//     );

//     cx.add_action(
//         |_: &mut Workspace, _: &RequestFeature, cx: &mut ViewContext<Workspace>| {
//             let url = "https://github.com/zed-industries/community/issues/new?assignees=&labels=enhancement%2Ctriage&template=0_feature_request.yml";
//             cx.platform().open_url(url);
//         },
//     );

//     cx.add_action(
//         move |_: &mut Workspace, _: &FileBugReport, cx: &mut ViewContext<Workspace>| {
//             let url = format!(
//                 "https://github.com/zed-industries/community/issues/new?assignees=&labels=defect%2Ctriage&template=2_bug_report.yml&environment={}",
//                 urlencoding::encode(&SystemSpecs::new(&cx).to_string())
//             );
//             cx.platform().open_url(&url);
//         },
//     );

//     cx.add_global_action(open_zed_community_repo);
// }

// pub fn open_zed_community_repo(_: &OpenZedCommunityRepo, cx: &mut AppContext) {
//     let url = "https://github.com/zed-industries/community";
//     cx.platform().open_url(&url);
// }
