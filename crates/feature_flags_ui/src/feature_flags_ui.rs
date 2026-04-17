//! Developer UI for inspecting and overriding feature flags at runtime.
//!
//! Registered under the `dev` namespace to make it clear this is not intended
//! for end users. Open with the `dev: open feature flags` action (or via the
//! command palette in debug builds).

use feature_flags::{FeatureFlagDescriptor, FeatureFlagStore, FeatureFlagVariant};
use gpui::{
    App, BorrowAppContext, DismissEvent, EventEmitter, FocusHandle, Focusable, ScrollHandle,
    Subscription, Window, actions, prelude::*,
};
use ui::{Checkbox, Modal, ModalHeader, ToggleState, WithScrollbar, prelude::*};
use workspace::{ModalView, Workspace};

actions!(
    dev,
    [
        /// Opens the feature flag configuration modal.
        OpenFeatureFlags
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &OpenFeatureFlags, window, cx| {
            workspace.toggle_modal(window, cx, |_window, cx| FeatureFlagsModal::new(cx));
        });
    })
    .detach();
}

struct FlagRow {
    descriptor: &'static FeatureFlagDescriptor,
    /// Cached variant list. The descriptor's variants fn is called exactly once
    /// when the modal opens; variants are static metadata and don't change.
    variants: Vec<FeatureFlagVariant>,
}

pub struct FeatureFlagsModal {
    focus_handle: FocusHandle,
    rows: Vec<FlagRow>,
    scroll_handle: ScrollHandle,
    _store_subscription: Subscription,
}

impl FeatureFlagsModal {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut rows: Vec<FlagRow> = FeatureFlagStore::known_flags()
            .map(|descriptor| FlagRow {
                descriptor,
                variants: (descriptor.variants)(),
            })
            .collect();
        rows.sort_by_key(|row| row.descriptor.name);

        let subscription = cx.observe_global::<FeatureFlagStore>(|_, cx| cx.notify());

        Self {
            focus_handle: cx.focus_handle(),
            rows,
            scroll_handle: ScrollHandle::new(),
            _store_subscription: subscription,
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn reset_flag(flag_name: &'static str, cx: &mut App) {
        cx.update_global::<FeatureFlagStore, _>(|store, cx| {
            store.clear_override(flag_name, cx);
        });
    }

    fn set_override(flag_name: &'static str, override_key: String, cx: &mut App) {
        cx.update_global::<FeatureFlagStore, _>(|store, cx| {
            store.set_override(flag_name, override_key, cx);
        });
    }

    fn render_row(&self, row: &FlagRow, cx: &mut Context<Self>) -> impl IntoElement {
        let descriptor = row.descriptor;
        let forced_on = FeatureFlagStore::is_forced_on(descriptor);
        let store = cx.global::<FeatureFlagStore>();
        let resolved = store.resolved_key(descriptor);
        let has_override = store.override_for(descriptor.name).is_some();

        let header = h_flex()
            .justify_between()
            .items_center()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Label::new(descriptor.name)
                            .size(LabelSize::Default)
                            .color(if forced_on {
                                Color::Muted
                            } else {
                                Color::Default
                            }),
                    )
                    .when(forced_on, |this| {
                        this.child(
                            Label::new("enabled for all")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
            .when(has_override && !forced_on, |this| {
                let name = descriptor.name;
                this.child(
                    Button::new(
                        SharedString::from(format!("reset-{}", name)),
                        "Reset",
                    )
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(move |_, _, _, cx| {
                            Self::reset_flag(name, cx);
                        })),
                )
            });

        let options = self.render_options(row, resolved, forced_on, cx);

        v_flex()
            .id(SharedString::from(format!("flag-row-{}", descriptor.name)))
            .gap_1()
            .py_2()
            .child(header)
            .child(options)
    }

    fn render_options(
        &self,
        row: &FlagRow,
        resolved: Option<&'static str>,
        forced_on: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let descriptor = row.descriptor;
        let selected_key = resolved;

        let row_items = row.variants.iter().map({
            let name = descriptor.name;
            move |variant| {
                let key = variant.override_key;
                let label = variant.label;
                let selected = selected_key == Some(key);
                let state = if selected {
                    ToggleState::Selected
                } else {
                    ToggleState::Unselected
                };
                let checkbox_id = SharedString::from(format!("{}-{}", name, key));
                let disabled = forced_on;
                let mut checkbox = Checkbox::new(ElementId::from(checkbox_id), state)
                    .label(label)
                    .disabled(disabled);
                if !disabled {
                    checkbox = checkbox.on_click(cx.listener(
                        move |_, new_state: &ToggleState, _, cx| {
                            if *new_state == ToggleState::Unselected {
                                // Clicking an already-selected option is a no-op rather than
                                // a "deselect" — there's no valid "nothing selected" state.
                                return;
                            }
                            Self::set_override(name, key.to_string(), cx);
                        },
                    ));
                }
                checkbox.into_any_element()
            }
        });

        h_flex().gap_4().flex_wrap().children(row_items)
    }

    fn render_body(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.rows.is_empty() {
            return div()
                .p_4()
                .child(
                    Label::new("No feature flags registered.").color(Color::Muted),
                )
                .into_any_element();
        }

        div()
            .size_full()
            .pb_2()
            .child(
                v_flex()
                    .id("feature-flags-list")
                    .px_2()
                    .max_h_128()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .children(
                        self.rows
                            .iter()
                            .map(|row| self.render_row(row, cx).into_any_element()),
                    ),
            )
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
            .into_any_element()
    }
}

impl ModalView for FeatureFlagsModal {}

impl Focusable for FeatureFlagsModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for FeatureFlagsModal {}

impl Render for FeatureFlagsModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("FeatureFlagsModal")
            .occlude()
            .elevation_3(cx)
            .w(rems(36.))
            .on_action(cx.listener(Self::dismiss))
            .track_focus(&self.focus_handle)
            .child(
                Modal::new("feature-flags", None::<ScrollHandle>)
                    .header(
                        ModalHeader::new()
                            .headline("Feature Flags")
                            .description(SharedString::from(
                                "Local overrides applied to feature flags. These persist across restarts.",
                            ))
                            .show_dismiss_button(true),
                    )
                    .child(self.render_body(window, cx)),
            )
    }
}
