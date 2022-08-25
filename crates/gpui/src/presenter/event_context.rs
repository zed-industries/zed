use std::ops::{Deref, DerefMut};

use collections::{HashMap, HashSet};

use crate::{Action, ElementBox, Event, FontCache, MutableAppContext, TextLayoutCache};

pub struct EventContext<'a> {
    rendered_views: &'a mut HashMap<usize, ElementBox>,
    pub font_cache: &'a FontCache,
    pub text_layout_cache: &'a TextLayoutCache,
    pub app: &'a mut MutableAppContext,
    pub window_id: usize,
    pub notify_count: usize,
    view_stack: Vec<usize>,
    pub(crate) handled: bool,
    pub(crate) invalidated_views: HashSet<usize>,
}

impl<'a> EventContext<'a> {
    pub(crate) fn dispatch_event(&mut self, view_id: usize, event: &Event) -> bool {
        if let Some(mut element) = self.rendered_views.remove(&view_id) {
            let result =
                self.with_current_view(view_id, |this| element.dispatch_event(event, this));
            self.rendered_views.insert(view_id, element);
            result
        } else {
            false
        }
    }

    pub(crate) fn with_current_view<F, T>(&mut self, view_id: usize, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        self.view_stack.push(view_id);
        let result = f(self);
        self.view_stack.pop();
        result
    }

    pub fn window_id(&self) -> usize {
        self.window_id
    }

    pub fn view_id(&self) -> Option<usize> {
        self.view_stack.last().copied()
    }

    pub fn is_parent_view_focused(&self) -> bool {
        if let Some(parent_view_id) = self.view_stack.last() {
            self.app.focused_view_id(self.window_id) == Some(*parent_view_id)
        } else {
            false
        }
    }

    pub fn focus_parent_view(&mut self) {
        if let Some(parent_view_id) = self.view_stack.last() {
            self.app.focus(self.window_id, Some(*parent_view_id))
        }
    }

    pub fn dispatch_any_action(&mut self, action: Box<dyn Action>) {
        self.app
            .dispatch_any_action_at(self.window_id, *self.view_stack.last().unwrap(), action)
    }

    pub fn dispatch_action<A: Action>(&mut self, action: A) {
        self.dispatch_any_action(Box::new(action));
    }

    pub fn notify(&mut self) {
        self.notify_count += 1;
        if let Some(view_id) = self.view_stack.last() {
            self.invalidated_views.insert(*view_id);
        }
    }

    pub fn notify_count(&self) -> usize {
        self.notify_count
    }

    pub fn propogate_event(&mut self) {
        self.handled = false;
    }
}

impl<'a> Deref for EventContext<'a> {
    type Target = MutableAppContext;

    fn deref(&self) -> &Self::Target {
        self.app
    }
}

impl<'a> DerefMut for EventContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.app
    }
}
