mod key_context_view;
pub mod lsp_button;
pub mod lsp_log_view;
mod syntax_tree_view;

#[cfg(test)]
mod lsp_log_view_tests;

use gpui::{App, AppContext, Entity};

pub use lsp_log_view::LspLogView;
pub use syntax_tree_view::{SyntaxTreeToolbarItemView, SyntaxTreeView};
use ui::{Context, Window};
use workspace::{Item, ItemHandle, SplitDirection, Workspace};

pub fn init(cx: &mut App) {
    lsp_log_view::init(false, cx);
    syntax_tree_view::init(cx);
    key_context_view::init(cx);
}

fn get_or_create_tool<T>(
    workspace: &mut Workspace,
    destination: SplitDirection,
    window: &mut Window,
    cx: &mut Context<Workspace>,
    new_tool: impl FnOnce(&mut Window, &mut Context<T>) -> T,
) -> Entity<T>
where
    T: Item,
{
    if let Some(item) = workspace.item_of_type::<T>(cx) {
        return item;
    }

    let new_tool = cx.new(|cx| new_tool(window, cx));
    match workspace.find_pane_in_direction(destination, cx) {
        Some(right_pane) => {
            workspace.add_item(
                right_pane,
                new_tool.boxed_clone(),
                None,
                true,
                true,
                window,
                cx,
            );
        }
        None => {
            workspace.split_item(destination, new_tool.boxed_clone(), window, cx);
        }
    }
    new_tool
}
