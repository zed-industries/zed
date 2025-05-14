use gpui::Entity;
use ui::{ContextMenu, DropdownMenu, DropdownStyle, prelude::*};

use crate::{debugger_panel::DebugPanel, session::DebugSession};

impl DebugPanel {
    pub fn render_session_menu(
        &mut self,
        active_session: &Entity<DebugSession>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let sessions = self.sessions().clone();
        let weak = cx.weak_entity();
        let label = active_session.read(cx).label_element(cx);

        DropdownMenu::new_with_element(
            "debugger-session-list",
            label,
            ContextMenu::build(window, cx, move |mut this, _, cx| {
                let context_menu = cx.weak_entity();
                for session in sessions.into_iter() {
                    let weak_session = session.downgrade();
                    let weak_session_id = weak_session.entity_id();

                    this = this.custom_entry(
                        {
                            let weak = weak.clone();
                            let context_menu = context_menu.clone();
                            move |_, cx| {
                                weak_session
                                    .read_with(cx, |session, cx| {
                                        let context_menu = context_menu.clone();
                                        let id: SharedString =
                                            format!("debug-session-{}", session.session_id(cx).0)
                                                .into();
                                        h_flex()
                                            .w_full()
                                            .group(id.clone())
                                            .justify_between()
                                            .child(session.label_element(cx))
                                            .child(
                                                IconButton::new(
                                                    "close-debug-session",
                                                    IconName::Close,
                                                )
                                                .visible_on_hover(id.clone())
                                                .icon_size(IconSize::Small)
                                                .on_click({
                                                    let weak = weak.clone();
                                                    move |_, window, cx| {
                                                        weak.update(cx, |panel, cx| {
                                                            panel.close_session(
                                                                weak_session_id,
                                                                window,
                                                                cx,
                                                            );
                                                        })
                                                        .ok();
                                                        context_menu
                                                            .update(cx, |this, cx| {
                                                                this.cancel(
                                                                    &Default::default(),
                                                                    window,
                                                                    cx,
                                                                );
                                                            })
                                                            .ok();
                                                    }
                                                }),
                                            )
                                            .into_any_element()
                                    })
                                    .unwrap_or_else(|_| div().into_any_element())
                            }
                        },
                        {
                            let weak = weak.clone();
                            move |window, cx| {
                                weak.update(cx, |panel, cx| {
                                    panel.activate_session(session.clone(), window, cx);
                                })
                                .ok();
                            }
                        },
                    );
                }
                this
            }),
        )
        .style(DropdownStyle::Ghost)
    }
}
