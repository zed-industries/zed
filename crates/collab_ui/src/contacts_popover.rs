use crate::{contact_finder::ContactFinder, contact_list::ContactList, ToggleContactsMenu};
use client::UserStore;
use gpui::{
    actions, elements::*, Entity, ModelHandle, MouseButton, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle,
};
use project::Project;
use settings::Settings;

actions!(contacts_popover, [ToggleContactFinder]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ContactsPopover::toggle_contact_finder);
}

pub enum Event {
    Dismissed,
}

enum Child {
    ContactList(ViewHandle<ContactList>),
    ContactFinder(ViewHandle<ContactFinder>),
}

pub struct ContactsPopover {
    child: Child,
    project: ModelHandle<Project>,
    user_store: ModelHandle<UserStore>,
    _subscription: Option<gpui::Subscription>,
}

impl ContactsPopover {
    pub fn new(
        project: ModelHandle<Project>,
        user_store: ModelHandle<UserStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            child: Child::ContactList(
                cx.add_view(|cx| ContactList::new(project.clone(), user_store.clone(), cx)),
            ),
            project,
            user_store,
            _subscription: None,
        };
        this.show_contact_list(String::new(), cx);
        this
    }

    fn toggle_contact_finder(&mut self, _: &ToggleContactFinder, cx: &mut ViewContext<Self>) {
        match &self.child {
            Child::ContactList(list) => self.show_contact_finder(list.read(cx).editor_text(cx), cx),
            Child::ContactFinder(finder) => {
                self.show_contact_list(finder.read(cx).editor_text(cx), cx)
            }
        }
    }

    fn show_contact_finder(&mut self, editor_text: String, cx: &mut ViewContext<ContactsPopover>) {
        let child = cx.add_view(|cx| {
            ContactFinder::new(self.user_store.clone(), cx).with_editor_text(editor_text, cx)
        });
        cx.focus(&child);
        self._subscription = Some(cx.subscribe(&child, |_, _, event, cx| match event {
            crate::contact_finder::Event::Dismissed => cx.emit(Event::Dismissed),
        }));
        self.child = Child::ContactFinder(child);
        cx.notify();
    }

    fn show_contact_list(&mut self, editor_text: String, cx: &mut ViewContext<ContactsPopover>) {
        let child = cx.add_view(|cx| {
            ContactList::new(self.project.clone(), self.user_store.clone(), cx)
                .with_editor_text(editor_text, cx)
        });
        cx.focus(&child);
        self._subscription = Some(cx.subscribe(&child, |_, _, event, cx| match event {
            crate::contact_list::Event::Dismissed => cx.emit(Event::Dismissed),
        }));
        self.child = Child::ContactList(child);
        cx.notify();
    }
}

impl Entity for ContactsPopover {
    type Event = Event;
}

impl View for ContactsPopover {
    fn ui_name() -> &'static str {
        "ContactsPopover"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let child = match &self.child {
            Child::ContactList(child) => ChildView::new(child, cx),
            Child::ContactFinder(child) => ChildView::new(child, cx),
        };

        MouseEventHandler::<ContactsPopover>::new(0, cx, |_, _| {
            Flex::column()
                .with_child(child.flex(1., true).boxed())
                .contained()
                .with_style(theme.contacts_popover.container)
                .constrained()
                .with_width(theme.contacts_popover.width)
                .with_height(theme.contacts_popover.height)
                .boxed()
        })
        .on_down_out(MouseButton::Left, move |_, cx| {
            cx.dispatch_action(ToggleContactsMenu);
        })
        .boxed()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            match &self.child {
                Child::ContactList(child) => cx.focus(child),
                Child::ContactFinder(child) => cx.focus(child),
            }
        }
    }
}
