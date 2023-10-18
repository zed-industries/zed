use crate::{
    DispatchPhase, FocusEvent, FocusHandle, Interactive, KeyDownEvent, KeyUpEvent, ViewContext,
};

pub trait Focus: Interactive {
    fn handle(&self) -> &FocusHandle;

    fn on_focus(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &FocusEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        let handle = self.handle().clone();
        self.listeners()
            .focus
            .push(Box::new(move |view, event, cx| {
                if event.focused.as_ref() == Some(&handle) {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_blur(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &FocusEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        let handle = self.handle().clone();
        self.listeners()
            .focus
            .push(Box::new(move |view, event, cx| {
                if event.blurred.as_ref() == Some(&handle) {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_focus_in(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &FocusEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        let handle = self.handle().clone();
        self.listeners()
            .focus
            .push(Box::new(move |view, event, cx| {
                if event
                    .focused
                    .as_ref()
                    .map_or(false, |focused| focused.contains(&handle, cx))
                {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_focus_out(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &FocusEvent, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        let handle = self.handle().clone();
        self.listeners()
            .focus
            .push(Box::new(move |view, event, cx| {
                if event
                    .blurred
                    .as_ref()
                    .map_or(false, |blurred| handle.contains(&blurred, cx))
                {
                    listener(view, event, cx)
                }
            }));
        self
    }

    fn on_key_down(
        mut self,
        listener: impl Fn(
                &mut Self::ViewState,
                &KeyDownEvent,
                DispatchPhase,
                &mut ViewContext<Self::ViewState>,
            ) + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners().key_down.push(Box::new(listener));
        self
    }

    fn on_key_up(
        mut self,
        listener: impl Fn(&mut Self::ViewState, &KeyUpEvent, DispatchPhase, &mut ViewContext<Self::ViewState>)
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.listeners().key_up.push(Box::new(listener));
        self
    }
}
