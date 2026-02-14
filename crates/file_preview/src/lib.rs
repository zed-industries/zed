pub mod formats;
mod preview_view;

pub use formats::{FilePreviewFormat, MermaidFormat, SvgFormat};
pub use preview_view::{FilePreviewView, PreviewMode};

use std::sync::Arc;

use gpui::{App, Context, Window, actions};
use workspace::Workspace;
use zed_actions::preview::{mermaid, svg};

actions!(svg, [OpenFollowingSvgPreview]);
actions!(mermaid, [OpenFollowingMermaidPreview]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else { return; };
        register_svg(workspace, window, cx);
        register_mermaid(workspace, window, cx);
    })
    .detach();
}

fn register_svg(
    workspace: &mut Workspace,
    _window: &mut Window,
    _cx: &mut Context<Workspace>,
) {
    let fmt = Arc::new(SvgFormat);
    let fmt2 = fmt.clone();
    let fmt3 = fmt.clone();

    workspace.register_action(move |w, _: &svg::OpenPreview, win, cx| {
        FilePreviewView::open_in_place(fmt.clone(), w, win, cx);
    });
    workspace.register_action(move |w, _: &svg::OpenPreviewToTheSide, win, cx| {
        FilePreviewView::open_to_side(fmt2.clone(), w, win, cx);
    });
    workspace.register_action(move |w, _: &OpenFollowingSvgPreview, win, cx| {
        FilePreviewView::open_following(fmt3.clone(), w, win, cx);
    });
}

fn register_mermaid(
    workspace: &mut Workspace,
    _window: &mut Window,
    _cx: &mut Context<Workspace>,
) {
    let fmt = Arc::new(MermaidFormat);
    let fmt2 = fmt.clone();
    let fmt3 = fmt.clone();

    workspace.register_action(move |w, _: &mermaid::OpenPreview, win, cx| {
        FilePreviewView::open_in_place(fmt.clone(), w, win, cx);
    });
    workspace.register_action(move |w, _: &mermaid::OpenPreviewToTheSide, win, cx| {
        FilePreviewView::open_to_side(fmt2.clone(), w, win, cx);
    });
    workspace.register_action(move |w, _: &OpenFollowingMermaidPreview, win, cx| {
        FilePreviewView::open_following(fmt3.clone(), w, win, cx);
    });
}
