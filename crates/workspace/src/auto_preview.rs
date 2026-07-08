use gpui::{App, Context, EntityId, Focusable as _, Global, Window};
use settings::Settings as _;

use crate::{AutoPreview, ItemHandle, SplitDirection, Workspace, WorkspaceSettings};

/// Integration point for preview views (e.g. Markdown or SVG previews) that can
/// automatically accompany or replace text editors, driven by the `auto_preview` setting.
#[derive(Clone, Copy)]
pub struct AutoPreviewProvider {
    /// Whether this provider can preview the file shown by the given item.
    pub applies_to: fn(&dyn ItemHandle, &App) -> bool,
    /// Whether any items this provider applies to are open in the workspace.
    pub has_open_sources: fn(&Workspace, &App) -> bool,
    /// Whether the given item is this provider's preview that follows the active editor.
    pub is_follow_view: fn(&dyn ItemHandle, &App) -> bool,
    /// Whether the given item is this provider's preview of a single file.
    pub is_preview_view: fn(&dyn ItemHandle, &App) -> bool,
    /// Builds a preview that follows the workspace's active editor.
    pub build_follow_view:
        fn(&mut Workspace, &mut Window, &mut Context<Workspace>) -> Option<Box<dyn ItemHandle>>,
    /// Builds a preview of the file shown by the given item.
    pub build_preview_view: fn(
        &mut Workspace,
        &dyn ItemHandle,
        &mut Window,
        &mut Context<Workspace>,
    ) -> Option<Box<dyn ItemHandle>>,
    /// Returns an editor for the file shown by the given preview item.
    pub source_view: fn(
        &mut Workspace,
        &dyn ItemHandle,
        &mut Window,
        &mut Context<Workspace>,
    ) -> Option<Box<dyn ItemHandle>>,
}

/// Per-workspace state of the side preview managed by [`sync_side_preview`].
#[derive(Default)]
pub(crate) struct AutoPreviewState {
    /// The follow preview currently managed in this workspace.
    follow_view_id: Option<EntityId>,
    /// The item that was active when the user closed the side preview: the preview is
    /// not reopened until another item gets activated.
    suppressed_for_item: Option<EntityId>,
}

#[derive(Default)]
struct GlobalAutoPreviewProviders(Vec<AutoPreviewProvider>);

impl Global for GlobalAutoPreviewProviders {}

pub fn register_auto_preview_provider(provider: AutoPreviewProvider, cx: &mut App) {
    cx.default_global::<GlobalAutoPreviewProviders>()
        .0
        .push(provider);
}

fn providers(cx: &App) -> Vec<AutoPreviewProvider> {
    cx.try_global::<GlobalAutoPreviewProviders>()
        .map(|providers| providers.0.clone())
        .unwrap_or_default()
}

/// Keeps a single follow-mode preview in a pane to the side of previewable editors
/// when the `auto_preview` setting is set to `to_the_side`: the preview is created when
/// a previewable editor becomes active, switches its kind together with the active
/// editor's file type, and is removed when no previewable editors remain open.
pub(crate) fn sync_side_preview(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if WorkspaceSettings::get_global(cx).auto_preview != AutoPreview::ToTheSide {
        return;
    }
    let providers = providers(cx);
    if providers.is_empty() {
        return;
    }
    let follow_views = collect_follow_views(workspace, &providers, cx);
    let active_item = workspace.active_item(cx);
    let active_item_id = active_item.as_ref().map(|item| item.item_id());

    // The managed preview disappeared without the sync removing it: the user has
    // closed its tab. Pause reopening until another item gets activated.
    if let Some(managed_id) = workspace.auto_preview_state.follow_view_id
        && !follow_views
            .iter()
            .any(|(_, view)| view.item_id() == managed_id)
    {
        workspace.auto_preview_state.follow_view_id = None;
        workspace.auto_preview_state.suppressed_for_item = active_item_id;
    }
    if workspace.auto_preview_state.suppressed_for_item.is_some()
        && workspace.auto_preview_state.suppressed_for_item != active_item_id
    {
        workspace.auto_preview_state.suppressed_for_item = None;
    }

    let active_provider = active_item.as_ref().and_then(|item| {
        providers
            .iter()
            .position(|provider| (provider.applies_to)(item.as_ref(), cx))
    });
    let Some(provider_index) = active_provider else {
        for (index, view) in follow_views {
            if !(providers[index].has_open_sources)(workspace, cx)
                && let Some(pane) = workspace.pane_for(view.as_ref())
            {
                pane.update(cx, |pane, cx| {
                    pane.remove_item(view.item_id(), false, true, window, cx);
                });
                if workspace.auto_preview_state.follow_view_id == Some(view.item_id()) {
                    workspace.auto_preview_state.follow_view_id = None;
                }
            }
        }
        return;
    };

    if workspace.auto_preview_state.suppressed_for_item.is_some() {
        return;
    }

    show_side_preview(
        workspace,
        &providers,
        provider_index,
        follow_views,
        window,
        cx,
    );
}

/// Ensures the active previewable file is shown as its text editor accompanied by a
/// side preview: an active in-place preview is switched to its source editor first,
/// and the side preview is reused or created the way the `auto_preview` setting's
/// `to_the_side` mode does. Returns false when no provider can preview the active item.
pub fn show_side_preview_for_active_item(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> bool {
    let providers = providers(cx);
    let Some(active_item) = workspace.active_item(cx) else {
        return false;
    };
    if let Some(provider) = providers
        .iter()
        .find(|provider| (provider.is_preview_view)(active_item.as_ref(), cx))
    {
        let Some(source) = (provider.source_view)(workspace, active_item.as_ref(), window, cx)
        else {
            return false;
        };
        if !workspace.activate_item(source.as_ref(), true, true, window, cx) {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.add_item(source, true, true, None, window, cx);
            });
        }
    }
    let Some(active_item) = workspace.active_item(cx) else {
        return false;
    };
    let Some(provider_index) = providers
        .iter()
        .position(|provider| (provider.applies_to)(active_item.as_ref(), cx))
    else {
        return false;
    };
    workspace.auto_preview_state.suppressed_for_item = None;
    let follow_views = collect_follow_views(workspace, &providers, cx);
    show_side_preview(
        workspace,
        &providers,
        provider_index,
        follow_views,
        window,
        cx,
    );
    true
}

/// Reuses this provider's existing follow preview or creates one in a pane to the
/// right, removing follow previews of the other providers along the way.
fn show_side_preview(
    workspace: &mut Workspace,
    providers: &[AutoPreviewProvider],
    provider_index: usize,
    follow_views: Vec<(usize, Box<dyn ItemHandle>)>,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let existing = follow_views
        .iter()
        .find(|(index, _)| *index == provider_index)
        .map(|(_, view)| view.boxed_clone());

    // Remove follow previews of other types: a single, dynamic preview is kept to the
    // side, and the new preview takes the tab slot vacated by the previous one.
    let mut vacated_slot = None;
    for (index, view) in &follow_views {
        if *index == provider_index {
            continue;
        }
        let Some(pane) = workspace.pane_for(view.as_ref()) else {
            continue;
        };
        let reuse_slot = existing.is_none() && vacated_slot.is_none();
        if reuse_slot && let Some(item_index) = pane.read(cx).index_for_item(view.as_ref()) {
            vacated_slot = Some((pane.clone(), item_index));
        }
        pane.update(cx, |pane, cx| {
            pane.remove_item(view.item_id(), false, !reuse_slot, window, cx);
        });
        if workspace.auto_preview_state.follow_view_id == Some(view.item_id()) {
            workspace.auto_preview_state.follow_view_id = None;
        }
    }

    if let Some(view) = existing {
        workspace.auto_preview_state.follow_view_id = Some(view.item_id());
        if let Some(pane) = workspace.pane_for(view.as_ref()) {
            pane.update(cx, |pane, cx| {
                if let Some(index) = pane.index_for_item(view.as_ref())
                    && pane
                        .active_item()
                        .is_none_or(|item| item.item_id() != view.item_id())
                {
                    pane.activate_item(index, false, false, window, cx);
                }
            });
        }
    } else {
        let Some(view) = (providers[provider_index].build_follow_view)(workspace, window, cx)
        else {
            return;
        };
        workspace.auto_preview_state.follow_view_id = Some(view.item_id());
        let (pane, destination_index) = match vacated_slot {
            Some((pane, index)) => (pane, Some(index)),
            None => {
                let pane = workspace
                    .find_pane_in_direction(SplitDirection::Right, cx)
                    .unwrap_or_else(|| {
                        workspace.split_pane(
                            workspace.active_pane().clone(),
                            SplitDirection::Right,
                            window,
                            cx,
                        )
                    });
                (pane, None)
            }
        };
        pane.update(cx, |pane, cx| {
            pane.add_item(view, false, false, destination_index, window, cx);
        });
        // Splitting a pane moves the focus into it: return the focus to the source item.
        if let Some(item) = workspace.active_item(cx) {
            item.item_focus_handle(cx).focus(window, cx);
        }
    }
}

/// Applies a change of the `auto_preview` setting to the already open items:
/// previewable editors and their previews are converted into each other in place,
/// and the side preview is created or removed.
pub(crate) fn auto_preview_setting_changed(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let providers = providers(cx);
    if providers.is_empty() {
        return;
    }
    workspace.auto_preview_state = AutoPreviewState::default();
    match WorkspaceSettings::get_global(cx).auto_preview {
        AutoPreview::Off => {
            remove_follow_views(workspace, &providers, window, cx);
            convert_previews_to_sources(workspace, &providers, window, cx);
        }
        AutoPreview::InPlace => {
            remove_follow_views(workspace, &providers, window, cx);
            convert_items(workspace, window, cx, |workspace, item, window, cx| {
                let provider = providers
                    .iter()
                    .find(|provider| (provider.applies_to)(item, cx))?;
                (provider.build_preview_view)(workspace, item, window, cx)
            });
        }
        AutoPreview::ToTheSide => {
            convert_previews_to_sources(workspace, &providers, window, cx);
            sync_side_preview(workspace, window, cx);
        }
    }
}

fn collect_follow_views(
    workspace: &Workspace,
    providers: &[AutoPreviewProvider],
    cx: &App,
) -> Vec<(usize, Box<dyn ItemHandle>)> {
    let mut views = Vec::new();
    for pane in workspace.panes() {
        for item in pane.read(cx).items() {
            if let Some(index) = providers
                .iter()
                .position(|provider| (provider.is_follow_view)(item.as_ref(), cx))
            {
                views.push((index, item.boxed_clone()));
            }
        }
    }
    views
}

fn remove_follow_views(
    workspace: &mut Workspace,
    providers: &[AutoPreviewProvider],
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    for (_, view) in collect_follow_views(workspace, providers, cx) {
        if let Some(pane) = workspace.pane_for(view.as_ref()) {
            pane.update(cx, |pane, cx| {
                pane.remove_item(view.item_id(), false, true, window, cx);
            });
        }
    }
}

fn convert_previews_to_sources(
    workspace: &mut Workspace,
    providers: &[AutoPreviewProvider],
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    convert_items(workspace, window, cx, |workspace, item, window, cx| {
        let provider = providers
            .iter()
            .find(|provider| (provider.is_preview_view)(item, cx))?;
        (provider.source_view)(workspace, item, window, cx)
    });
}

/// Replaces items in their tab slots with the converted counterparts, keeping the
/// panes' active tabs, ephemeral (preview tab) statuses and the focus in place.
fn convert_items(
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
    mut convert: impl FnMut(
        &mut Workspace,
        &dyn ItemHandle,
        &mut Window,
        &mut Context<Workspace>,
    ) -> Option<Box<dyn ItemHandle>>,
) {
    let panes = workspace.panes().to_vec();
    for pane in panes {
        let pane_had_focus = pane.read(cx).focus_handle(cx).contains_focused(window, cx);
        let original_active_id = pane.read(cx).active_item().map(|item| item.item_id());
        let items = pane
            .read(cx)
            .items()
            .map(|item| item.boxed_clone())
            .collect::<Vec<_>>();
        let mut converted = false;
        let mut active_replacement = None;
        for item in items {
            let Some(new_item) = convert(workspace, item.as_ref(), window, cx) else {
                continue;
            };
            // The replacement may already be open elsewhere (e.g. an editor opened via
            // `preview::OpenSource`): drop the converted item and let that tab stand.
            let already_open = workspace.pane_for(new_item.as_ref()).is_some();
            let was_active = original_active_id == Some(item.item_id());
            pane.update(cx, |pane, cx| {
                let Some(index) = pane.index_for_item(item.as_ref()) else {
                    return;
                };
                let was_ephemeral = pane.is_active_preview_item(item.item_id());
                if !already_open {
                    pane.add_item(
                        new_item.boxed_clone(),
                        false,
                        false,
                        Some(index),
                        window,
                        cx,
                    );
                }
                pane.remove_item(item.item_id(), false, already_open, window, cx);
                if was_ephemeral && !already_open {
                    pane.set_preview_item_id(Some(new_item.item_id()), cx);
                }
                converted = true;
            });
            if was_active && !already_open {
                active_replacement = Some(new_item);
            }
        }
        if !converted || !workspace.panes().contains(&pane) {
            continue;
        }
        // Adding items disturbs the pane's active tab: restore it (or its replacement).
        let target = active_replacement.or_else(|| {
            let pane = pane.read(cx);
            original_active_id.and_then(|id| {
                pane.items()
                    .find(|item| item.item_id() == id)
                    .map(|item| item.boxed_clone())
            })
        });
        if let Some(target) = target {
            pane.update(cx, |pane, cx| {
                if let Some(index) = pane.index_for_item(target.as_ref()) {
                    pane.activate_item(index, pane_had_focus, pane_had_focus, window, cx);
                }
            });
        }
    }
}
