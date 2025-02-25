//! # combobox

// Ideally, the relationship between the combo box and the picker should be inverted.
//
// A picker is essentially a combobox modal

use gpui::{AnyView, App, Corner, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable};

use picker::{Picker, PickerDelegate};
use ui::{prelude::*, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

pub struct ComboboxPopover<D: PickerDelegate> {
    picker: Entity<Picker<D>>,
}

impl<D: PickerDelegate> ComboboxPopover<D> {
    pub fn new(delegate: D, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .show_scrollbar(true)
                .width(rems(20.))
                .max_height(Some(rems(20.).into()))
        });

        Self { picker }
    }
}

impl<D: PickerDelegate> EventEmitter<DismissEvent> for ComboboxPopover<D> {}

impl<D: PickerDelegate> Focusable for ComboboxPopover<D> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl<D: PickerDelegate> Render for ComboboxPopover<D> {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.picker.clone()
    }
}

#[derive(IntoElement)]
pub struct Combobox<T, TT, D>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
    D: PickerDelegate,
{
    id: ElementId,
    combobox_popover: Entity<ComboboxPopover<D>>,
    trigger: T,
    tooltip: TT,
    handle: Option<PopoverMenuHandle<ComboboxPopover<D>>>,
    anchor: Corner,
}

impl<T, TT, D> Combobox<T, TT, D>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
    D: PickerDelegate,
{
    pub fn new(
        id: impl Into<ElementId>,
        combobox_popover: Entity<ComboboxPopover<D>>,
        trigger: T,
        tooltip: TT,
    ) -> Self {
        Self {
            id: id.into(),
            combobox_popover,
            trigger,
            tooltip,
            handle: None,
            anchor: Corner::TopLeft,
        }
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<ComboboxPopover<D>>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn corner(mut self, anchor: Corner) -> Self {
        self.anchor = anchor;
        self
    }
}

impl<T, TT, D> RenderOnce for Combobox<T, TT, D>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
    D: PickerDelegate,
{
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let combobox = self.combobox_popover.clone();

        PopoverMenu::new(self.id)
            .menu(move |_window, _cx| Some(combobox.clone()))
            .trigger_with_tooltip(self.trigger, self.tooltip)
            .anchor(self.anchor)
            .when_some(self.handle.clone(), |menu, handle| menu.with_handle(handle))
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
    }
}
