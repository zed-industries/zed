use std::rc::Rc;

use call::{ActiveCall, Room};
use channel::ChannelStore;
use gpui::{AppContext, Entity, RenderOnce, WeakEntity};
use project::Project;
use ui::{
    ActiveTheme, AnyElement, App, Avatar, Button, ButtonCommon, ButtonSize, ButtonStyle, Clickable,
    Color, Context, ContextMenu, ContextMenuItem, Element, FluentBuilder, Icon, IconButton,
    IconName, IconSize, IntoElement, Label, LabelCommon, LabelSize, ParentElement, PopoverMenu,
    PopoverMenuHandle, Render, SelectableButton, SharedString, SplitButton, SplitButtonStyle,
    Styled, StyledExt, TintColor, Toggleable, Tooltip, Window, div, h_flex, px, v_flex,
};
use workspace::Workspace;

pub struct CallOverlay {
    active_call: Entity<ActiveCall>,
    channel_store: Entity<ChannelStore>,
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    screen_share_popover_handle: PopoverMenuHandle<ContextMenu>,
}

impl CallOverlay {
    pub(crate) fn render_call_controls(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let Some(room) = self.active_call.read(cx).room() else {
            return Vec::default();
        };

        let room = room.read(cx);
        let project = self.project.read(cx);
        let is_local = project.is_local() || project.is_via_remote_server();
        let is_shared = is_local && project.is_shared();
        let is_muted = room.is_muted();
        let muted_by_user = room.muted_by_user();
        let is_deafened = room.is_deafened().unwrap_or(false);
        let is_screen_sharing = room.is_sharing_screen();
        let can_use_microphone = room.can_use_microphone();
        let can_share_projects = room.can_share_projects();
        let screen_sharing_supported = cx.is_screen_capture_supported();
        let is_connecting_to_project = self
            .workspace
            .update(cx, |workspace, cx| workspace.has_active_modal(window, cx))
            .unwrap_or(false);

        let mut children = Vec::new();

        if can_use_microphone {
            children.push(
                IconButton::new(
                    "mute-microphone",
                    if is_muted {
                        IconName::MicMute
                    } else {
                        IconName::Mic
                    },
                )
                .tooltip(move |window, cx| {
                    if is_muted {
                        if is_deafened {
                            Tooltip::with_meta(
                                "Unmute Microphone",
                                None,
                                "Audio will be unmuted",
                                window,
                                cx,
                            )
                        } else {
                            Tooltip::simple("Unmute Microphone", cx)
                        }
                    } else {
                        Tooltip::simple("Mute Microphone", cx)
                    }
                })
                .style(ButtonStyle::Subtle)
                .icon_size(IconSize::Small)
                .toggle_state(is_muted)
                .selected_icon_color(Color::Error)
                .on_click(move |_, _window, cx| {
                    // toggle_mute(&Default::default(), cx);
                    // todo!()
                })
                .into_any_element(),
            );
        }

        children.push(
            IconButton::new(
                "mute-sound",
                if is_deafened {
                    IconName::AudioOff
                } else {
                    IconName::AudioOn
                },
            )
            .style(ButtonStyle::Subtle)
            .selected_icon_color(Color::Error)
            .icon_size(IconSize::Small)
            .toggle_state(is_deafened)
            .tooltip(move |window, cx| {
                if is_deafened {
                    let label = "Unmute Audio";

                    if !muted_by_user {
                        Tooltip::with_meta(label, None, "Microphone will be unmuted", window, cx)
                    } else {
                        Tooltip::simple(label, cx)
                    }
                } else {
                    let label = "Mute Audio";

                    if !muted_by_user {
                        Tooltip::with_meta(label, None, "Microphone will be muted", window, cx)
                    } else {
                        Tooltip::simple(label, cx)
                    }
                }
            })
            .on_click(move |_, _, cx| {
                // toggle_deafen(&Default::default(), cx))
                // todo!()
            })
            .into_any_element(),
        );

        if can_use_microphone && screen_sharing_supported {
            children.push(
                IconButton::new("screen-share", IconName::Screen)
                    .style(ButtonStyle::Subtle)
                    .icon_size(IconSize::Small)
                    .toggle_state(is_screen_sharing)
                    .selected_icon_color(Color::Error)
                    .tooltip(Tooltip::text(if is_screen_sharing {
                        "Stop Sharing Screen"
                    } else {
                        "Share Screen"
                    }))
                    .on_click(move |_, window, cx| {
                        let should_share = ActiveCall::global(cx)
                            .read(cx)
                            .room()
                            .is_some_and(|room| !room.read(cx).is_sharing_screen());

                        // window
                        //     .spawn(cx, async move |cx| {
                        //         let screen = if should_share {
                        //             // cx.update(|_, cx| {
                        //             //     // pick_default_screen(cx)}
                        //             //     // todo!()
                        //             // })?
                        //             // .await
                        //         } else {
                        //             Ok(None)
                        //         };
                        //         cx.update(|window, cx| {
                        //             // toggle_screen_sharing(screen, window, cx)
                        //             // todo!()
                        //         })?;

                        //         Result::<_, anyhow::Error>::Ok(())
                        //     })
                        //     .detach();
                        // self.render_screen_list().into_any_element(),
                    })
                    .into_any_element(),
            );

            // children.push(
            //     SplitButton::new(trigger.render(window, cx))
            //         .style(SplitButtonStyle::Transparent)
            //         .into_any_element(),
            // );
        }

        children.push(div().pr_2().into_any_element());

        children
    }

    fn render_screen_list(&self) -> impl IntoElement {
        PopoverMenu::new("screen-share-screen-list")
            .with_handle(self.screen_share_popover_handle.clone())
            .trigger(
                ui::ButtonLike::new_rounded_right("screen-share-screen-list-trigger")
                    .child(
                        h_flex()
                            .mx_neg_0p5()
                            .h_full()
                            .justify_center()
                            .child(Icon::new(IconName::ChevronDown).size(IconSize::XSmall)),
                    )
                    .toggle_state(self.screen_share_popover_handle.is_deployed()),
            )
            .menu(|window, cx| {
                let screens = cx.screen_capture_sources();
                Some(ContextMenu::build(window, cx, |context_menu, _, cx| {
                    cx.spawn(async move |this: WeakEntity<ContextMenu>, cx| {
                        let screens = screens.await??;
                        this.update(cx, |this, cx| {
                            let active_screenshare_id = ActiveCall::global(cx)
                                .read(cx)
                                .room()
                                .and_then(|room| room.read(cx).shared_screen_id());
                            for screen in screens {
                                let Ok(meta) = screen.metadata() else {
                                    continue;
                                };

                                let label = meta
                                    .label
                                    .clone()
                                    .unwrap_or_else(|| SharedString::from("Unknown screen"));
                                let resolution = SharedString::from(format!(
                                    "{} × {}",
                                    meta.resolution.width.0, meta.resolution.height.0
                                ));
                                this.push_item(ContextMenuItem::CustomEntry {
                                    entry_render: Box::new(move |_, _| {
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                Icon::new(IconName::Screen)
                                                    .size(IconSize::XSmall)
                                                    .map(|this| {
                                                        if active_screenshare_id == Some(meta.id) {
                                                            this.color(Color::Accent)
                                                        } else {
                                                            this.color(Color::Muted)
                                                        }
                                                    }),
                                            )
                                            .child(Label::new(label.clone()))
                                            .child(
                                                Label::new(resolution.clone())
                                                    .color(Color::Muted)
                                                    .size(LabelSize::Small),
                                            )
                                            .into_any()
                                    }),
                                    selectable: true,
                                    documentation_aside: None,
                                    handler: Rc::new(move |_, window, cx| {
                                        // toggle_screen_sharing(Ok(Some(screen.clone())), window, cx);
                                    }),
                                });
                            }
                        })
                    })
                    .detach_and_log_err(cx);
                    context_menu
                }))
            })
    }
}

impl Render for CallOverlay {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(room) = self.active_call.read(cx).room() else {
            return gpui::Empty.into_any_element();
        };

        let title = if let Some(channel_id) = room.read(cx).channel_id()
            && let Some(channel) = self.channel_store.read(cx).channel_for_id(channel_id)
        {
            channel.name.clone()
        } else {
            "Unknown".into()
        };

        div()
            .p_1()
            .child(
                v_flex()
                    .elevation_3(cx)
                    .bg(cx.theme().colors().editor_background)
                    .p_2()
                    .w_full()
                    .gap_2()
                    .child(
                        h_flex()
                            .justify_between()
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Icon::new(IconName::Audio)
                                            .color(Color::VersionControlAdded),
                                    )
                                    .child(Label::new(title)),
                            )
                            .child(Icon::new(IconName::ChevronDown)),
                    )
                    .child(
                        h_flex()
                            .justify_between()
                            .child(h_flex().children(self.render_call_controls(window, cx)))
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Button::new("leave-call", "Leave")
                                            .icon(Some(IconName::Exit))
                                            .label_size(LabelSize::Small)
                                            .style(ButtonStyle::Tinted(TintColor::Error))
                                            .tooltip(Tooltip::text("Leave Call"))
                                            .icon_size(IconSize::Small)
                                            .on_click(move |_, _window, cx| {
                                                ActiveCall::global(cx)
                                                    .update(cx, |call, cx| call.hang_up(cx))
                                                    .detach_and_log_err(cx);
                                            }),
                                    )
                                    .into_any_element(),
                            ),
                    ),
            )
            .into_any_element()
    }
}

pub fn init(cx: &App) {
    cx.observe_new(|workspace: &mut Workspace, _, cx| {
        let dock = workspace.dock_at_position(workspace::dock::DockPosition::Left);
        let handle = cx.weak_entity();
        let project = workspace.project().clone();
        dock.update(cx, |dock, cx| {
            let overlay = cx.new(|cx| {
                let active_call = ActiveCall::global(cx);
                cx.observe(&active_call, |_, _, cx| cx.notify()).detach();
                let channel_store = ChannelStore::global(cx);
                CallOverlay {
                    channel_store,
                    active_call,
                    workspace: handle,
                    project,
                    screen_share_popover_handle: PopoverMenuHandle::default(),
                }
            });
            dock.add_overlay(
                cx,
                Box::new(move |window, cx| {
                    overlay.update(cx, |overlay, cx| {
                        overlay.render(window, cx).into_any_element()
                    })
                }),
            )
        });
    })
    .detach();
}
