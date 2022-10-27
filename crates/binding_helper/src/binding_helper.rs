use std::collections::BTreeMap;

use itertools::Itertools;
use settings::Settings;
use smallvec::SmallVec;

use gpui::{
    elements::{Empty, Flex, Label, Overlay, ParentElement},
    keymap::{Keystroke, MatchResult},
    Element, Entity, View, ViewContext,
};

pub struct BindingHelper {
    next_bindings: BTreeMap<SmallVec<[Keystroke; 2]>, (&'static str, &'static str)>,
}

impl BindingHelper {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        cx.observe_keystrokes(|this, _keystroke, result, cx| {
            if let Some(focused_view) = cx.focused_view_id(cx.window_id()) {
                match result {
                    MatchResult::None => this.next_bindings.clear(),
                    MatchResult::Pending => {
                        this.next_bindings = cx
                            .available_bindings(cx.window_id(), focused_view)
                            .into_iter()
                            .map(|(keys, binding)| {
                                (
                                    keys,
                                    (binding.action().namespace(), binding.action().name()),
                                )
                            })
                            .collect();
                    }
                    MatchResult::Match { action, .. } => {
                        if action.namespace() == "zed" && action.name() == "Leader" {
                            this.next_bindings = cx
                                .available_bindings(cx.window_id(), focused_view)
                                .into_iter()
                                .filter(|(_, binding)| {
                                    binding.context_contains_identifier("Leader")
                                })
                                .map(|(keys, binding)| {
                                    (
                                        keys,
                                        (binding.action().namespace(), binding.action().name()),
                                    )
                                })
                                .collect();
                        } else {
                            this.next_bindings.clear();
                        }
                    }
                }
            }

            cx.notify();
            true
        })
        .detach();

        Self {
            next_bindings: Default::default(),
        }
    }
}

impl Entity for BindingHelper {
    type Event = ();
}

impl View for BindingHelper {
    fn ui_name() -> &'static str {
        "Binding Helper"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        if self.next_bindings.is_empty() {
            return Empty::new().boxed();
        }

        let style = cx.global::<Settings>().theme.context_menu.clone();
        let next_keys = Flex::row()
            .with_children(
                self.next_bindings
                    .iter()
                    .chunks(7)
                    .into_iter()
                    .map(|chunk| {
                        Flex::column()
                            .with_children(chunk.into_iter().map(|(keys, action)| {
                                let style = style.item.default.clone();

                                Flex::row()
                                    .with_children(keys.iter().map(|keystroke| {
                                        Label::new(
                                            keystroke.to_string(),
                                            style.keystroke.text.clone(),
                                        )
                                        .contained()
                                        .with_style(style.keystroke.container.clone())
                                        .boxed()
                                    }))
                                    .with_child({
                                        Label::new(
                                            format!("{}::{}", action.0, action.1),
                                            style.label.clone(),
                                        )
                                        .contained()
                                        .boxed()
                                    })
                                    .boxed()
                            }))
                            .boxed()
                    }),
            )
            .aligned()
            .bottom()
            .contained()
            .boxed();

        Overlay::new(next_keys).boxed()
    }
}
