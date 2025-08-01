use db::kvp::Dismissable;
use editor::Editor;
use gpui::{App, AppContext as _, Context, EventEmitter, Subscription};
use ui::{
    Banner, Button, Clickable, FluentBuilder as _, IconButton, IconName, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, Window, div, h_flex,
};
use workspace::{
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
    notifications::{NotificationId, simple_message_notification::MessageNotification},
};

impl Dismissable for BasedPyrightNote {
    const KEY: &str = "basedpyright-note";
}

// pub fn init(cx: &mut App) {
//     cx.observe_new(move |workspace: &mut Workspace, window, cx| {
//         let Some(window) = window else {
//             return;
//         };

//         cx.subscribe_in(workspace.project(), window, |_, _, event, window, cx| {
//             if let project::Event::LanguageServerAdded(_, name, _) = event
//                 && name == "basedpyright"
//             {
//                 if BasedPyrightNote::dismissed() {
//                     return;
//                 }

//                 cx.on_next_frame(window, move |workspace, _, cx| {
//                     workspace.show_notification(
//                         NotificationId::unique::<BasedPyrightNote>(),
//                         cx,
//                         |cx| {
//                             cx.new(move |cx| {
//                                 MessageNotification::new(
//                                     "basedpyright is now the default language server for Python",
//                                     cx,
//                                 )
//                                 .more_info_message("Learn More")
//                                 .more_info_url("https://zed.dev/FIXME")
//                                 // .primary_message("Yes, install extension")
//                                 // .primary_icon(IconName::Check)
//                                 // .primary_icon_color(Color::Success)
//                                 // .primary_on_click({
//                                 //     let extension_id = extension_id.clone();
//                                 //     move |_window, cx| {
//                                 //         let extension_id = extension_id.clone();
//                                 //         let extension_store = ExtensionStore::global(cx);
//                                 //         extension_store.update(cx, move |store, cx| {
//                                 //             store.install_latest_extension(extension_id, cx);
//                                 //         });
//                                 //     }
//                                 // })
//                                 // .secondary_message("No, don't install it")
//                                 // .secondary_icon(IconName::Close)
//                                 // .secondary_icon_color(Color::Error)
//                                 // .secondary_on_click(move |_window, cx| {
//                                 //     let key = language_extension_key(&extension_id);
//                                 //     db::write_and_log(cx, move || {
//                                 //         KEY_VALUE_STORE.write_kvp(key, "dismissed".to_string())
//                                 //     });
//                                 // })
//                             })
//                         },
//                     );
//                 })
//             }
//         })
//         .detach();
//     })
//     .detach();
// }

pub struct BasedPyrightBanner {
    dismissed: bool,
    have_basedpyright: bool,
    _subscriptions: [Subscription; 1],
}

impl BasedPyrightBanner {
    pub fn new(workspace: &Workspace, cx: &mut Context<Self>) -> Self {
        let subscription = cx.subscribe(workspace.project(), |this, _, event, cx| {
            if let project::Event::LanguageServerAdded(_, name, _) = event
                && name == "basedpyright"
            {
                this.have_basedpyright = true;
            }
        });
        Self {
            dismissed: false,
            have_basedpyright: false,
            _subscriptions: [subscription],
        }
    }
}

impl EventEmitter<ToolbarItemEvent> for BasedPyrightBanner {}

impl Render for BasedPyrightBanner {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("basedpyright-banner")
            .when(!self.dismissed && self.have_basedpyright, |el| {
                el.child(
                    Banner::new()
                        .severity(ui::Severity::Info)
                        .child(
                            h_flex()
                                .child("Basedpyright is now the default language server for Python")
                                .child(
                                    Button::new("learn-more", "Learn More")
                                        .icon(IconName::ArrowUpRight),
                                ),
                        )
                        .action_slot(IconButton::new("dismiss", IconName::Close).on_click(
                            cx.listener(|this, _, _, cx| {
                                this.dismissed = true;
                                cx.notify();
                            }),
                        ))
                        .into_any_element(),
                )
            })
    }
}

impl ToolbarItemView for BasedPyrightBanner {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        window: &mut ui::Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if let Some(item) = active_pane_item
            && let Some(editor) = item.downcast::<Editor>()
            && let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton()
            && let Some(file) = buffer.read(cx).file()
            && file
                .file_name(cx)
                .as_encoded_bytes()
                .ends_with(".py".as_bytes())
        {
            return ToolbarItemLocation::Secondary;
        }

        ToolbarItemLocation::Hidden
    }
}
