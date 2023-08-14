use crate::{platform::ForegroundPlatform, Action, App, AppContext};

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

impl AppContext {
    pub fn set_menus(&mut self, menus: Vec<Menu>) {
        self.foreground_platform
            .set_menus(menus, &self.keystroke_matcher);
    }
}

pub(crate) fn setup_menu_handlers(foreground_platform: &dyn ForegroundPlatform, app: &App) {
    foreground_platform.on_will_open_menu(Box::new({
        let cx = app.0.clone();
        move || {
            let mut cx = cx.borrow_mut();
            cx.keystroke_matcher.clear_pending();
        }
    }));
    foreground_platform.on_validate_menu_command(Box::new({
        let cx = app.0.clone();
        move |action| {
            let cx = cx.borrow_mut();
            !cx.keystroke_matcher.has_pending_keystrokes() && cx.is_action_available(action)
        }
    }));
    foreground_platform.on_menu_command(Box::new({
        let cx = app.0.clone();
        move |action| {
            let mut cx = cx.borrow_mut();
            if let Some(main_window) = cx.active_window() {
                let dispatched = main_window
                    .update(&mut *cx, |cx| {
                        if let Some(view_id) = cx.focused_view_id() {
                            cx.dispatch_action(Some(view_id), action);
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false);

                if dispatched {
                    return;
                }
            }
            cx.dispatch_global_action_any(action);
        }
    }));
}
