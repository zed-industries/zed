use crate::{
    contact_finder::{build_contact_finder, ContactFinder},
    contact_list::ContactList,
};
use client::UserStore;
use gpui::{
    actions, elements::*, platform::MouseButton, AppContext, Entity, ModelHandle, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use picker::PickerEvent;
use project::Project;
use workspace::Workspace;

actions!(contacts_popover, [ToggleContactFinder]);

pub fn init(cx: &mut AppContext) {
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
    workspace: WeakViewHandle<Workspace>,
    _subscription: Option<gpui::Subscription>,
}

impl ContactsPopover {
    pub fn new(
        project: ModelHandle<Project>,
        user_store: ModelHandle<UserStore>,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            child: Child::ContactList(cx.add_view(|cx| {
                ContactList::new(project.clone(), user_store.clone(), workspace.clone(), cx)
            })),
            project,
            user_store,
            workspace,
            _subscription: None,
        };
        this.show_contact_list(String::new(), cx);
        this
    }

    fn toggle_contact_finder(&mut self, _: &ToggleContactFinder, cx: &mut ViewContext<Self>) {
        match &self.child {
            Child::ContactList(list) => self.show_contact_finder(list.read(cx).editor_text(cx), cx),
            Child::ContactFinder(finder) => self.show_contact_list(finder.read(cx).query(cx), cx),
        }
    }

    fn show_contact_finder(&mut self, editor_text: String, cx: &mut ViewContext<ContactsPopover>) {
        let child = cx.add_view(|cx| {
            let finder = build_contact_finder(self.user_store.clone(), cx);
            finder.set_query(editor_text, cx);
            finder
        });
        cx.focus(&child);
        self._subscription = Some(cx.subscribe(&child, |_, _, event, cx| match event {
            PickerEvent::Dismiss => cx.emit(Event::Dismissed),
        }));
        self.child = Child::ContactFinder(child);
        cx.notify();
    }

    fn show_contact_list(&mut self, editor_text: String, cx: &mut ViewContext<ContactsPopover>) {
        let child = cx.add_view(|cx| {
            ContactList::new(
                self.project.clone(),
                self.user_store.clone(),
                self.workspace.clone(),
                cx,
            )
            .with_editor_text(editor_text, cx)
        });
        cx.focus(&child);
        self._subscription = Some(cx.subscribe(&child, |this, _, event, cx| match event {
            crate::contact_list::Event::Dismissed => cx.emit(Event::Dismissed),
            crate::contact_list::Event::ToggleContactFinder => {
                this.toggle_contact_finder(&Default::default(), cx)
            }
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

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx).clone();
        let child = match &self.child {
            Child::ContactList(child) => ChildView::new(child, cx),
            Child::ContactFinder(child) => ChildView::new(child, cx),
        };

        MouseEventHandler::<ContactsPopover, Self>::new(0, cx, |_, _| {
            Flex::column()
                .with_child(child.flex(1., true))
                .contained()
                .with_style(theme.contacts_popover.container)
                .constrained()
                .with_width(theme.contacts_popover.width)
                .with_height(theme.contacts_popover.height)
        })
        .on_down_out(MouseButton::Left, move |_, _, cx| cx.emit(Event::Dismissed))
        .into_any()
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
