use editor::{Editor, display_map::BlockId};
use gpui::{Context, Entity};
use language::Buffer;
use project::ConflictSet;

struct ConflictAddon {
    conflict_set: Entity<ConflictSet>,
    block_ids: Vec<BlockId>,
}

impl editor::Addon for ConflictAddon {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn register_editor(editor: &mut Editor, buffer: Entity<Buffer>, cx: &mut Context<Editor>) {
    let Some(project) = &editor.project else {
        return;
    };
    let git_store = project.read(cx).git_store().clone();
    let conflict_set_task =
        git_store.update(cx, |git_store, cx| git_store.open_conflict_set(buffer, cx));

    cx.spawn(async move |editor, cx| {
        let conflict_set = conflict_set_task.await?;

        let conflict_view = ConflictAddon {
            conflict_set,
            block_ids: Vec::new(),
        };

        editor.update(cx, |editor, cx| {
            cx.subscribe(&conflict_view.conflict_set, |editor, _, event, cx| {
                let conflict_addon = editor.addon::<ConflictAddon>().unwrap();
                conflicts_updated(editor, conflict_addon, event, cx);
            });
            editor.register_addon(conflict_view);
        })
    })
    .detach();
}

fn conflicts_updated(
    editor: &mut Editor,
    conflict_view: &ConflictAddon,
    event: &gpui::Event,
    cx: &mut Context<Editor>,
) {
    match event {
        gpui::Event::DidUpdate => {
            let conflict_addon = editor.addon::<ConflictAddon>().unwrap();
            conflict_addon.block_ids.clear();
            conflict_addon
                .block_ids
                .extend(conflict_view.conflict_set.read(cx).blocks());
        }
        _ => {}
    }
}
