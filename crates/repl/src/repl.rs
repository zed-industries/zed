#[allow(unused)]
use anyhow::Result;
#[allow(unused)]
use collections::{HashMap, HashSet};
#[allow(unused)]
use editor::{
    display_map::{
        BlockContext, BlockDisposition, BlockId, BlockProperties, BlockStyle, RenderBlock,
    },
    Anchor, AnchorRangeExt, Editor,
};
#[allow(unused)]
use futures::{future::Shared, StreamExt};
use gpui::Task;
use gpui::{prelude::*, AppContext};
#[allow(unused)]
use gpui::{Entity, View};
use language::Point;
#[allow(unused)]
use outputs::{ExecutionStatus, ExecutionView, LineHeight as _};
#[allow(unused)]
use runtime_panel::Run;
use runtime_settings::JupyterSettings;
use settings::Settings as _;
use std::ops::Range;
use theme::{ActiveTheme, ThemeSettings};
use ui::prelude::*;
use workspace::Workspace;

mod outputs;
mod runtime_manager;
mod runtime_panel;
mod runtime_session;
mod runtime_settings;
mod runtimes;
mod stdio;

pub use runtime_manager::RuntimeManager;
pub use runtime_panel::RuntimePanel;
pub use runtime_session::Session;
use runtimes::RunningKernel;

#[derive(Debug)]
pub enum Kernel {
    RunningKernel(RunningKernel),
    StartingKernel(Shared<Task<()>>),
    FailedLaunch,
    ErroredLaunch(String),
}

#[derive(Debug, Clone)]
pub struct EditorRuntimeState {
    pub blocks: Vec<EditorRuntimeBlock>,
    // todo!(): Store a subscription to the editor so we can drop them when the editor is dropped
    // subscription: gpui::Subscription,
}

#[derive(Debug, Clone)]
pub struct EditorRuntimeBlock {
    pub code_range: Range<Anchor>,
    pub _execution_id: String,
    pub block_id: BlockId,
    pub _execution_view: View<ExecutionView>,
}

pub fn get_active_editor(
    workspace: &mut Workspace,
    cx: &mut ViewContext<Workspace>,
) -> Option<View<Editor>> {
    workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
}

pub fn init(cx: &mut AppContext) {
    runtime_panel::init(cx)
}

// Gets the active selection in the editor or the current line
pub fn selection(editor: View<Editor>, cx: &mut ViewContext<Workspace>) -> Range<Anchor> {
    let editor = editor.read(cx);
    let selection = editor.selections.newest::<usize>(cx);
    let buffer = editor.buffer().read(cx).snapshot(cx);

    let range = if selection.is_empty() {
        let cursor = selection.head();

        let line_start = buffer.offset_to_point(cursor).row;
        let mut start_offset = buffer.point_to_offset(Point::new(line_start, 0));

        // Iterate backwards to find the start of the line
        while start_offset > 0 {
            let ch = buffer.chars_at(start_offset - 1).next().unwrap_or('\0');
            if ch == '\n' {
                break;
            }
            start_offset -= 1;
        }

        let mut end_offset = cursor;

        // Iterate forwards to find the end of the line
        while end_offset < buffer.len() {
            let ch = buffer.chars_at(end_offset).next().unwrap_or('\0');
            if ch == '\n' {
                break;
            }
            end_offset += 1;
        }

        // Create a range from the start to the end of the line
        start_offset..end_offset
    } else {
        selection.range()
    };

    let anchor_range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
    anchor_range
}

// pub fn old_run(workspace: &mut Workspace, _: &Run, cx: &mut ViewContext<Workspace>) {
//     dbg!();
//     let (editor, runtime_manager) = if let (Some(editor), Some(runtime_manager)) =
//         (get_active_editor(workspace, cx), RuntimeManager::global(cx))
//     {
//         (editor, runtime_manager)
//     } else {
//         dbg!(RuntimeManager::global(cx));
//         log::warn!("No active editor or runtime manager found");
//         return;
//     };
//     dbg!();

//     let anchor_range = selection(editor.clone(), cx);

//     let buffer = editor.read(cx).buffer().read(cx).snapshot(cx);

//     let selected_text = buffer
//         .text_for_range(anchor_range.clone())
//         .collect::<String>();

//     let start_language = buffer.language_at(anchor_range.start);
//     let end_language = buffer.language_at(anchor_range.end);

//     let language_name = if start_language == end_language {
//         start_language
//             .map(|language| language.code_fence_block_name())
//             .filter(|lang| **lang != *"markdown")
//     } else {
//         // If the selection spans multiple languages, don't run it
//         return;
//     };

//     let language_name = if let Some(language_name) = language_name {
//         language_name
//     } else {
//         return;
//     };

//     let entity_id = editor.entity_id();

//     dbg!();

//     let execution_view = cx.new_view(|cx| ExecutionView::new(cx));

//     // If any block overlaps with the new block, remove it
//     // TODO: When inserting a new block, put it in order so that search is efficient
//     let blocks_to_remove = runtime_manager.update(cx, |runtime_manager, _cx| {
//         // Get the current `EditorRuntimeState` for this runtime_manager, inserting it if it doesn't exist
//         let editor_runtime_state = runtime_manager
//             .editors
//             .entry(editor.downgrade())
//             .or_insert_with(|| EditorRuntimeState { blocks: Vec::new() });

//         let mut blocks_to_remove: HashSet<BlockId> = HashSet::default();

//         editor_runtime_state.blocks.retain(|block| {
//             if anchor_range.overlaps(&block.code_range, &buffer) {
//                 blocks_to_remove.insert(block.block_id);
//                 // Drop this block
//                 false
//             } else {
//                 true
//             }
//         });

//         blocks_to_remove
//     });

//     let blocks_to_remove = blocks_to_remove.clone();

//     let block_id = editor.update(cx, |editor, cx| {
//         editor.remove_blocks(blocks_to_remove, None, cx);
//         let block = BlockProperties {
//             position: anchor_range.end,
//             height: execution_view.num_lines(cx).saturating_add(1),
//             style: BlockStyle::Sticky,
//             render: create_output_area_render(execution_view.clone()),
//             disposition: BlockDisposition::Below,
//         };

//         editor.insert_blocks([block], None, cx)[0]
//     });

//     let receiver = runtime_manager.update(cx, |runtime_manager, cx| {
//         let editor_runtime_state = runtime_manager
//             .editors
//             .entry(editor.downgrade())
//             .or_insert_with(|| EditorRuntimeState { blocks: Vec::new() });

//         let editor_runtime_block = EditorRuntimeBlock {
//             code_range: anchor_range.clone(),
//             block_id,
//             _execution_view: execution_view.clone(),
//             _execution_id: Default::default(),
//         };

//         editor_runtime_state
//             .blocks
//             .push(editor_runtime_block.clone());

//         runtime_manager.execute_code(entity_id, language_name, selected_text.clone(), cx)
//     });

//     cx.spawn(|_this, mut cx| async move {
//         execution_view.update(&mut cx, |execution_view, cx| {
//             execution_view.set_status(ExecutionStatus::ConnectingToKernel, cx);
//         })?;
//         let mut receiver = receiver.await?;

//         let execution_view = execution_view.clone();
//         while let Some(content) = receiver.next().await {
//             execution_view.update(&mut cx, |execution_view, cx| {
//                 execution_view.push_message(&content, cx)
//             })?;

//             editor.update(&mut cx, |editor, cx| {
//                 let mut replacements = HashMap::default();
//                 replacements.insert(
//                     block_id,
//                     (
//                         Some(execution_view.num_lines(cx).saturating_add(1)),
//                         create_output_area_render(execution_view.clone()),
//                     ),
//                 );
//                 editor.replace_blocks(replacements, None, cx);
//             })?;
//         }
//         anyhow::Ok(())
//     })
//     .detach_and_log_err(cx);
// }

fn create_output_area_render(execution_view: View<ExecutionView>) -> RenderBlock {
    let render = move |cx: &mut BlockContext| {
        let execution_view = execution_view.clone();
        let text_font = ThemeSettings::get_global(cx).buffer_font.family.clone();
        // Note: we'll want to use `cx.anchor_x` when someone runs something with no output -- just show a checkmark and not make the full block below the line

        let gutter_width = cx.gutter_dimensions.width;

        h_flex()
            .w_full()
            .bg(cx.theme().colors().background)
            .border_y_1()
            .border_color(cx.theme().colors().border)
            .pl(gutter_width)
            .child(
                div()
                    .font_family(text_font)
                    // .ml(gutter_width)
                    .mx_1()
                    .my_2()
                    .h_full()
                    .w_full()
                    .mr(gutter_width)
                    .child(execution_view),
            )
            .into_any_element()
    };

    Box::new(render)
}
