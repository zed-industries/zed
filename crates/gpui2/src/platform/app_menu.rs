use crate::{Action, AppContext, Platform};
use util::ResultExt;

pub struct Menu<'a> {
    pub name: &'a str,
    pub items: Vec<MenuItem<'a>>,
}

pub enum MenuItem<'a> {
    Separator,
    Submenu(Menu<'a>),
    Action {
        name: &'a str,
        action: Box<dyn Action>,
        os_action: Option<OsAction>,
    },
}

impl<'a> MenuItem<'a> {
    pub fn separator() -> Self {
        Self::Separator
    }

    pub fn submenu(menu: Menu<'a>) -> Self {
        Self::Submenu(menu)
    }

    pub fn action(name: &'a str, action: impl Action) -> Self {
        Self::Action {
            name,
            action: Box::new(action),
            os_action: None,
        }
    }

    pub fn os_action(name: &'a str, action: impl Action, os_action: OsAction) -> Self {
        Self::Action {
            name,
            action: Box::new(action),
            os_action: Some(os_action),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum OsAction {
    Cut,
    Copy,
    Paste,
    SelectAll,
    Undo,
    Redo,
}

pub(crate) fn init(platform: &dyn Platform, cx: &mut AppContext) {
    platform.on_will_open_menu(Box::new({
        let cx = cx.to_async();
        move || {
            cx.update(|cx| cx.clear_pending_keystrokes()).ok();
        }
    }));

    platform.on_validate_menu_command(Box::new({
        let cx = cx.to_async();
        move |action| {
            cx.update(|cx| cx.is_action_available(action))
                .unwrap_or(false)
        }
    }));

    platform.on_menu_command(Box::new({
        let cx = cx.to_async();
        move |action| {
            cx.update(|cx| {
                // if let Some(main_window) = cx.active_window() {
                //     let dispatched = main_window
                //         .update(&mut *cx, |cx| {
                //             if let Some(view_id) = cx.focused_view_id() {
                //                 cx.dispatch_action(Some(view_id), action);
                //                 true
                //             } else {
                //                 false
                //             }
                //         })
                //         .unwrap_or(false);

                //     if dispatched {
                //         return;
                //     }
                // }
                // cx.dispatch_global_action_any(action);
            })
            .log_err();
        }
    }));
}
