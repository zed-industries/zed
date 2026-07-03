use feature_flags::{FeatureFlagDescriptor, FeatureFlagStore, FeatureFlagVariant};
use fs::Fs;
use gpui::{ScrollHandle, prelude::*};
use ui::{Checkbox, ToggleState, prelude::*};

use crate::SettingsWindow;

pub(crate) fn render_feature_flags_page(
    _settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    // Sort by flag name so the list is stable between renders even though
    // `inventory::iter` order depends on link order.
    let mut descriptors: Vec<&'static FeatureFlagDescriptor> =
        FeatureFlagStore::known_flags().collect();
    descriptors.sort_by_key(|descriptor| descriptor.name);

    v_flex()
        .id("feature-flags-page")
        .min_w_0()
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .gap_4()
        .overflow_y_scroll()
        .track_scroll(scroll_handle)
        .children(
            descriptors
                .into_iter()
                .map(|descriptor| render_flag_row(descriptor, cx)),
        )
        .into_any_element()
}

fn render_flag_row(
    descriptor: &'static FeatureFlagDescriptor,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    let forced_on = FeatureFlagStore::is_forced_on(descriptor);
    let resolved = cx.global::<FeatureFlagStore>().resolved_key(descriptor, cx);
    let has_override = FeatureFlagStore::override_for(descriptor.name, cx).is_some();

    let header =
        h_flex()
            .justify_between()
            .items_center()
            .child(
                h_flex()
                    .gap_2()
                    .child(Label::new(descriptor.name).size(LabelSize::Default).color(
                        if forced_on {
                            Color::Muted
                        } else {
                            Color::Default
                        },
                    ))
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
                    Button::new(SharedString::from(format!("reset-{}", name)), "Reset")
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(move |_, _, _, cx| {
                            FeatureFlagStore::clear_override(name, <dyn Fs>::global(cx), cx);
                        })),
                )
            });

    v_flex()
        .id(SharedString::from(format!("flag-row-{}", descriptor.name)))
        .gap_1()
        .child(header)
        .child(render_flag_variants(descriptor, resolved, forced_on, cx))
        .into_any_element()
}

fn render_flag_variants(
    descriptor: &'static FeatureFlagDescriptor,
    resolved: &'static str,
    forced_on: bool,
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    let variants: Vec<FeatureFlagVariant> = (descriptor.variants)();

    let row_items = variants.into_iter().map({
        let name = descriptor.name;
        move |variant| {
            let key = variant.override_key;
            let label = variant.label;
            let selected = resolved == key;
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
                checkbox =
                    checkbox.on_click(cx.listener(move |_, new_state: &ToggleState, _, cx| {
                        // Clicking an already-selected option is a no-op rather than a
                        // "deselect" — there's no valid "nothing selected" state.
                        if *new_state == ToggleState::Unselected {
                            return;
                        }
                        FeatureFlagStore::set_override(
                            name,
                            key.to_string(),
                            <dyn Fs>::global(cx),
                            cx,
                        );
                    }));
            }
            checkbox.into_any_element()
        }
    });

    h_flex().gap_4().flex_wrap().children(row_items)
}
