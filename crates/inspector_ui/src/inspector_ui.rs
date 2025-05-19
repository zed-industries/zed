use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use anyhow::{Context as _, anyhow};
use editor::{Editor, EditorEvent};
use gpui::{App, DivInspectorState, Empty, Entity, InspectorElementId, IntoElement, Window};
use language::language_settings::SoftWrap;
use ui::prelude::*;
use ui::{Button, Label, LabelSize, h_flex, v_flex};
use util::ResultExt as _;
use util::command::new_smol_command;

pub fn init(cx: &mut App) {
    // TODO: Instead toggle a global debug mode? Not all windows support the command pallete.
    cx.on_action(|_: &zed_actions::dev::ToggleInspector, cx| {
        let Some(active_window) = cx
            .active_window()
            .context("no active window to toggle inspector")
            .log_err()
        else {
            return;
        };
        // This is deferred to avoid double lease due to window already being updated.
        cx.defer(move |cx| {
            active_window
                .update(cx, |_, window, cx| window.toggle_inspector(cx))
                .log_err();
        });
    });
    let last_div_inspector = Rc::new(RefCell::new(None));
    cx.register_inspector_element(move |id, state, window, cx| {
        div_inspector(id, state, last_div_inspector.clone(), window, cx)
    })
}

pub fn div_inspector(
    id: InspectorElementId,
    state: &DivInspectorState,
    last_div_inspector: Rc<RefCell<Option<(Entity<Editor>, InspectorElementId)>>>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    let mut last_div_inspector = last_div_inspector.borrow_mut();
    if let Some((editor, last_id)) = &*last_div_inspector {
        if last_id == &id {
            return div().w_64().h_64().child(editor.clone());
        }
    }
    // todo! Better error handling
    let Some(json_text) = serde_json::to_string_pretty(state).log_err() else {
        return div();
    };
    let editor = cx.new(|cx| {
        let mut editor = Editor::multi_line(window, cx);
        editor.set_text(json_text, window, cx);
        editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
        editor.set_show_line_numbers(false, cx);
        editor.set_show_code_actions(false, cx);
        editor.set_show_breakpoints(false, cx);
        editor.set_show_git_diff_gutter(false, cx);
        editor.set_show_runnables(false, cx);
        editor.set_show_edit_predictions(Some(false), window, cx);
        editor
    });
    window
        .subscribe(&editor, cx, {
            let id = id.clone();
            move |editor, event: &EditorEvent, window, cx| {
                match event {
                    EditorEvent::BufferEdited => {
                        let json_text = editor.read(cx).text(cx);
                        // todo! error handling
                        let Some(new_state) = serde_json_lenient::from_str(&json_text).log_err()
                        else {
                            return;
                        };
                        window.update_inspector_state::<DivInspectorState, _>(
                            &id,
                            |state, _window| {
                                *state = new_state;
                            },
                        )
                    }
                    _ => {}
                }
            }
        })
        .detach();
    *last_div_inspector = Some((editor.clone(), id));
    return div().w_64().h_64().child(editor);
    /*
    v_flex()
        .bg(cx.theme().colors().elevated_surface_background)
        .p_4()
        .mt_4()
        .mr_4()
        .rounded_lg()
        .shadow_lg()
        .child(h_flex().child(Label::new(id.to_string()).size(LabelSize::XSmall)))
        .child(Button::new("open", "Open").on_click({
            let id = id.clone();
            move |_event, _window, cx| {
                cx.background_spawn(open_zed_source_location(id.source))
                    .detach_and_log_err(cx);
            }
        }))
    */
}

// TODO: Move to some other crate (along with build.rs) and also use this in error notifications.
async fn open_zed_source_location(
    location: &'static std::panic::Location<'static>,
) -> anyhow::Result<()> {
    let mut path = Path::new(env!("ZED_REPO_DIR")).to_path_buf();
    path.push(Path::new(location.file()));
    let path_arg = format!(
        "{}:{}:{}",
        path.display(),
        location.line(),
        location.column()
    );

    let output = new_smol_command("zed")
        .arg(&path_arg)
        .output()
        .await
        .with_context(|| format!("running zed to open {path_arg} failed"))?;

    if !output.status.success() {
        Err(anyhow!(
            "running zed to open {path_arg} failed with stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    } else {
        Ok(())
    }
}
