use std::{
    fmt::Display,
    num::{NonZero, NonZeroU32, NonZeroU64},
    rc::Rc,
    str::FromStr,
};

use editor::{Editor, actions::MoveDown, actions::MoveUp};
use gpui::{
    ClickEvent, Entity, FocusHandle, Focusable, FontWeight, Modifiers, TextAlign,
    TextStyleRefinement, WeakEntity,
};

use settings::{
    CenteredPaddingSettings, CodeFade, DelayMs, FontSize, FontWeightContent, InactiveOpacity,
    MinimumContrast,
};
use ui::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberFieldMode {
    #[default]
    Read,
    Edit,
}

pub trait NumberFieldType: Display + Copy + Clone + Sized + PartialOrd + FromStr + 'static {
    fn default_format(value: &Self) -> String {
        format!("{}", value)
    }
    fn default_step() -> Self;
    fn large_step() -> Self;
    fn small_step() -> Self;
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn saturating_add(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
}

macro_rules! impl_newtype_numeric_stepper_float {
    ($type:ident, $default:expr, $large:expr, $small:expr, $min:expr, $max:expr) => {
        impl NumberFieldType for $type {
            fn default_step() -> Self {
                $default.into()
            }

            fn large_step() -> Self {
                $large.into()
            }

            fn small_step() -> Self {
                $small.into()
            }

            fn min_value() -> Self {
                $min.into()
            }

            fn max_value() -> Self {
                $max.into()
            }

            fn saturating_add(self, rhs: Self) -> Self {
                $type((self.0 + rhs.0).min(Self::max_value().0))
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                $type((self.0 - rhs.0).max(Self::min_value().0))
            }
        }
    };
}

macro_rules! impl_newtype_numeric_stepper_int {
    ($type:ident, $default:expr, $large:expr, $small:expr, $min:expr, $max:expr) => {
        impl NumberFieldType for $type {
            fn default_step() -> Self {
                $default.into()
            }

            fn large_step() -> Self {
                $large.into()
            }

            fn small_step() -> Self {
                $small.into()
            }

            fn min_value() -> Self {
                $min.into()
            }

            fn max_value() -> Self {
                $max.into()
            }

            fn saturating_add(self, rhs: Self) -> Self {
                $type(self.0.saturating_add(rhs.0).min(Self::max_value().0))
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                $type(self.0.saturating_sub(rhs.0).max(Self::min_value().0))
            }
        }
    };
}

#[rustfmt::skip]
impl_newtype_numeric_stepper_float!(FontWeight, 50., 100., 10., FontWeight::THIN, FontWeight::BLACK);
impl_newtype_numeric_stepper_float!(
    FontWeightContent,
    50.,
    100.,
    10.,
    FontWeightContent::THIN,
    FontWeightContent::BLACK
);
impl_newtype_numeric_stepper_float!(CodeFade, 0.1, 0.2, 0.05, 0.0, 0.9);
impl_newtype_numeric_stepper_float!(FontSize, 1.0, 4.0, 0.5, 6.0, 72.0);
impl_newtype_numeric_stepper_float!(InactiveOpacity, 0.1, 0.2, 0.05, 0.0, 1.0);
impl_newtype_numeric_stepper_float!(MinimumContrast, 1., 10., 0.5, 0.0, 106.0);
impl_newtype_numeric_stepper_int!(DelayMs, 100, 500, 10, 0, 2000);
impl_newtype_numeric_stepper_float!(
    CenteredPaddingSettings,
    0.05,
    0.2,
    0.1,
    CenteredPaddingSettings::MIN_PADDING,
    CenteredPaddingSettings::MAX_PADDING
);

macro_rules! impl_numeric_stepper_int {
    ($type:ident) => {
        impl NumberFieldType for $type {
            fn default_step() -> Self {
                1
            }

            fn large_step() -> Self {
                10
            }

            fn small_step() -> Self {
                1
            }

            fn min_value() -> Self {
                <$type>::MIN
            }

            fn max_value() -> Self {
                <$type>::MAX
            }

            fn saturating_add(self, rhs: Self) -> Self {
                self.saturating_add(rhs)
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                self.saturating_sub(rhs)
            }
        }
    };
}

macro_rules! impl_numeric_stepper_nonzero_int {
    ($nonzero:ty, $inner:ty) => {
        impl NumberFieldType for $nonzero {
            fn default_step() -> Self {
                <$nonzero>::new(1).unwrap()
            }

            fn large_step() -> Self {
                <$nonzero>::new(10).unwrap()
            }

            fn small_step() -> Self {
                <$nonzero>::new(1).unwrap()
            }

            fn min_value() -> Self {
                <$nonzero>::MIN
            }

            fn max_value() -> Self {
                <$nonzero>::MAX
            }

            fn saturating_add(self, rhs: Self) -> Self {
                let result = self.get().saturating_add(rhs.get());
                <$nonzero>::new(result.max(1)).unwrap()
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                let result = self.get().saturating_sub(rhs.get()).max(1);
                <$nonzero>::new(result).unwrap()
            }
        }
    };
}

macro_rules! impl_numeric_stepper_float {
    ($type:ident) => {
        impl NumberFieldType for $type {
            fn default_format(value: &Self) -> String {
                format!("{:.2}", value)
            }

            fn default_step() -> Self {
                1.0
            }

            fn large_step() -> Self {
                10.0
            }

            fn small_step() -> Self {
                0.1
            }

            fn min_value() -> Self {
                <$type>::MIN
            }

            fn max_value() -> Self {
                <$type>::MAX
            }

            fn saturating_add(self, rhs: Self) -> Self {
                (self + rhs).clamp(Self::min_value(), Self::max_value())
            }

            fn saturating_sub(self, rhs: Self) -> Self {
                (self - rhs).clamp(Self::min_value(), Self::max_value())
            }
        }
    };
}

impl_numeric_stepper_float!(f32);
impl_numeric_stepper_float!(f64);
impl_numeric_stepper_int!(isize);
impl_numeric_stepper_int!(usize);
impl_numeric_stepper_int!(i32);
impl_numeric_stepper_int!(u32);
impl_numeric_stepper_int!(i64);
impl_numeric_stepper_int!(u64);

impl_numeric_stepper_nonzero_int!(NonZeroU32, u32);
impl_numeric_stepper_nonzero_int!(NonZeroU64, u64);
impl_numeric_stepper_nonzero_int!(NonZero<usize>, usize);

type OnChangeCallback<T> = Rc<dyn Fn(&T, &mut Window, &mut App) + 'static>;

#[derive(IntoElement, RegisterComponent)]
pub struct NumberField<T: NumberFieldType = usize> {
    id: ElementId,
    value: T,
    focus_handle: FocusHandle,
    mode: Entity<NumberFieldMode>,
    /// Stores a weak reference to the editor when in edit mode, so buttons can update its text
    edit_editor: Entity<Option<WeakEntity<Editor>>>,
    /// Stores the on_change callback in Entity state so it's not stale in focus_out handlers
    on_change_state: Entity<Option<OnChangeCallback<T>>>,
    /// Tracks the last prop value we synced to, so we can detect external changes (like reset)
    last_synced_value: Entity<Option<T>>,
    format: Box<dyn FnOnce(&T) -> String>,
    large_step: T,
    small_step: T,
    step: T,
    min_value: T,
    max_value: T,
    on_reset: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_change: Rc<dyn Fn(&T, &mut Window, &mut App) + 'static>,
    tab_index: Option<isize>,
}

impl<T: NumberFieldType> NumberField<T> {
    pub fn new(id: impl Into<ElementId>, value: T, window: &mut Window, cx: &mut App) -> Self {
        let id = id.into();

        let (mode, focus_handle, edit_editor, on_change_state, last_synced_value) =
            window.with_id(id.clone(), |window| {
                let mode = window.use_state(cx, |_, _| NumberFieldMode::default());
                let focus_handle = window.use_state(cx, |_, cx| cx.focus_handle());
                let edit_editor = window.use_state(cx, |_, _| None);
                let on_change_state: Entity<Option<OnChangeCallback<T>>> =
                    window.use_state(cx, |_, _| None);
                let last_synced_value: Entity<Option<T>> = window.use_state(cx, |_, _| None);
                (
                    mode,
                    focus_handle,
                    edit_editor,
                    on_change_state,
                    last_synced_value,
                )
            });

        Self {
            id,
            mode,
            edit_editor,
            on_change_state,
            last_synced_value,
            value,
            focus_handle: focus_handle.read(cx).clone(),
            format: Box::new(T::default_format),
            large_step: T::large_step(),
            step: T::default_step(),
            small_step: T::small_step(),
            min_value: T::min_value(),
            max_value: T::max_value(),
            on_reset: None,
            on_change: Rc::new(|_, _, _| {}),
            tab_index: None,
        }
    }

    pub fn format(mut self, format: impl FnOnce(&T) -> String + 'static) -> Self {
        self.format = Box::new(format);
        self
    }

    pub fn small_step(mut self, step: T) -> Self {
        self.small_step = step;
        self
    }

    pub fn normal_step(mut self, step: T) -> Self {
        self.step = step;
        self
    }

    pub fn large_step(mut self, step: T) -> Self {
        self.large_step = step;
        self
    }

    pub fn min(mut self, min: T) -> Self {
        self.min_value = min;
        self
    }

    pub fn max(mut self, max: T) -> Self {
        self.max_value = max;
        self
    }

    pub fn mode(self, mode: NumberFieldMode, cx: &mut App) -> Self {
        self.mode.write(cx, mode);
        self
    }

    pub fn on_reset(
        mut self,
        on_reset: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_reset = Some(Box::new(on_reset));
        self
    }

    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = Some(tab_index);
        self
    }

    pub fn on_change(mut self, on_change: impl Fn(&T, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Rc::new(on_change);
        self
    }

    fn sync_on_change_state(&self, cx: &mut App) {
        self.on_change_state
            .update(cx, |state, _| *state = Some(self.on_change.clone()));
    }
}

#[derive(Clone, Copy)]
enum ValueChangeDirection {
    Increment,
    Decrement,
}

impl<T: NumberFieldType> RenderOnce for NumberField<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // Sync the on_change callback to Entity state so focus_out handlers can access it
        self.sync_on_change_state(cx);

        let is_edit_mode = matches!(*self.mode.read(cx), NumberFieldMode::Edit);

        let get_step = {
            let large_step = self.large_step;
            let step = self.step;
            let small_step = self.small_step;
            move |modifiers: Modifiers| -> T {
                if modifiers.shift {
                    large_step
                } else if modifiers.alt {
                    small_step
                } else {
                    step
                }
            }
        };

        let clamp_value = {
            let min = self.min_value;
            let max = self.max_value;
            move |value: T| -> T {
                if value < min {
                    min
                } else if value > max {
                    max
                } else {
                    value
                }
            }
        };

        let change_value = {
            move |current: T, step: T, direction: ValueChangeDirection| -> T {
                let new_value = match direction {
                    ValueChangeDirection::Increment => current.saturating_add(step),
                    ValueChangeDirection::Decrement => current.saturating_sub(step),
                };
                clamp_value(new_value)
            }
        };

        let get_current_value = {
            let value = self.value;
            let edit_editor = self.edit_editor.clone();

            Rc::new(move |cx: &App| -> T {
                if !is_edit_mode {
                    return value;
                }
                edit_editor
                    .read(cx)
                    .as_ref()
                    .and_then(|weak| weak.upgrade())
                    .and_then(|editor| editor.read(cx).text(cx).parse::<T>().ok())
                    .unwrap_or(value)
            })
        };

        let update_editor_text = {
            let edit_editor = self.edit_editor.clone();

            Rc::new(move |new_value: T, window: &mut Window, cx: &mut App| {
                if !is_edit_mode {
                    return;
                }
                let Some(editor) = edit_editor
                    .read(cx)
                    .as_ref()
                    .and_then(|weak| weak.upgrade())
                else {
                    return;
                };
                editor.update(cx, |editor, cx| {
                    editor.set_text(format!("{}", new_value), window, cx);
                });
            })
        };

        let bg_color = cx.theme().colors().surface_background;
        let hover_bg_color = cx.theme().colors().element_hover;

        let border_color = cx.theme().colors().border_variant;
        let focus_border_color = cx.theme().colors().border_focused;

        let base_button = |icon: IconName| {
            h_flex()
                .cursor_pointer()
                .p_1p5()
                .size_full()
                .justify_center()
                .overflow_hidden()
                .border_1()
                .border_color(border_color)
                .bg(bg_color)
                .hover(|s| s.bg(hover_bg_color))
                .focus_visible(|s| s.border_color(focus_border_color).bg(hover_bg_color))
                .child(Icon::new(icon).size(IconSize::Small))
        };

        h_flex()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .gap_1()
            .when_some(self.on_reset, |this, on_reset| {
                this.child(
                    IconButton::new("reset", IconName::RotateCcw)
                        .icon_size(IconSize::Small)
                        .when_some(self.tab_index, |this, _| this.tab_index(0isize))
                        .on_click(on_reset),
                )
            })
            .child({
                let on_change_for_increment = self.on_change.clone();

                h_flex()
                    .map(|decrement| {
                        let decrement_handler = {
                            let on_change = self.on_change.clone();
                            let get_current_value = get_current_value.clone();
                            let update_editor_text = update_editor_text.clone();

                            move |click: &ClickEvent, window: &mut Window, cx: &mut App| {
                                let current_value = get_current_value(cx);
                                let step = get_step(click.modifiers());
                                let new_value = change_value(
                                    current_value,
                                    step,
                                    ValueChangeDirection::Decrement,
                                );

                                update_editor_text(new_value, window, cx);
                                on_change(&new_value, window, cx);
                            }
                        };

                        decrement.child(
                            base_button(IconName::Dash)
                                .id((self.id.clone(), "decrement_button"))
                                .rounded_tl_sm()
                                .rounded_bl_sm()
                                .when_some(self.tab_index, |this, _| this.tab_index(0isize))
                                .on_click(decrement_handler),
                        )
                    })
                    .child({
                        h_flex()
                            .min_w_16()
                            .size_full()
                            .border_y_1()
                            .border_color(border_color)
                            .bg(bg_color)
                            .in_focus(|this| this.border_color(focus_border_color))
                            .child(match *self.mode.read(cx) {
                                NumberFieldMode::Read => h_flex()
                                    .px_1()
                                    .flex_1()
                                    .justify_center()
                                    .child(
                                        Label::new((self.format)(&self.value)).color(Color::Muted),
                                    )
                                    .into_any_element(),
                                NumberFieldMode::Edit => {
                                    let expected_text = format!("{}", self.value);

                                    let editor = window.use_state(cx, {
                                        let expected_text = expected_text.clone();

                                        move |window, cx| {
                                            let mut editor = Editor::single_line(window, cx);

                                            editor.set_text_style_refinement(TextStyleRefinement {
                                                color: Some(cx.theme().colors().text),
                                                text_align: Some(TextAlign::Center),
                                                ..Default::default()
                                            });

                                            editor.set_text(expected_text, window, cx);

                                            let editor_weak = cx.entity().downgrade();

                                            self.edit_editor.update(cx, |state, _| {
                                                *state = Some(editor_weak);
                                            });

                                            editor
                                                .register_action::<MoveUp>({
                                                    let on_change = self.on_change.clone();
                                                    let editor_handle = cx.entity().downgrade();
                                                    move |_, window, cx| {
                                                        let Some(editor) = editor_handle.upgrade()
                                                        else {
                                                            return;
                                                        };
                                                        editor.update(cx, |editor, cx| {
                                                            if let Ok(current_value) =
                                                                editor.text(cx).parse::<T>()
                                                            {
                                                                let step =
                                                                    get_step(window.modifiers());
                                                                let new_value = change_value(
                                                                    current_value,
                                                                    step,
                                                                    ValueChangeDirection::Increment,
                                                                );
                                                                editor.set_text(
                                                                    format!("{}", new_value),
                                                                    window,
                                                                    cx,
                                                                );
                                                                on_change(&new_value, window, cx);
                                                            }
                                                        });
                                                    }
                                                })
                                                .detach();

                                            editor
                                                .register_action::<MoveDown>({
                                                    let on_change = self.on_change.clone();
                                                    let editor_handle = cx.entity().downgrade();
                                                    move |_, window, cx| {
                                                        let Some(editor) = editor_handle.upgrade()
                                                        else {
                                                            return;
                                                        };
                                                        editor.update(cx, |editor, cx| {
                                                            if let Ok(current_value) =
                                                                editor.text(cx).parse::<T>()
                                                            {
                                                                let step =
                                                                    get_step(window.modifiers());
                                                                let new_value = change_value(
                                                                    current_value,
                                                                    step,
                                                                    ValueChangeDirection::Decrement,
                                                                );
                                                                editor.set_text(
                                                                    format!("{}", new_value),
                                                                    window,
                                                                    cx,
                                                                );
                                                                on_change(&new_value, window, cx);
                                                            }
                                                        });
                                                    }
                                                })
                                                .detach();

                                            cx.on_focus_out(&editor.focus_handle(cx), window, {
                                                let on_change_state = self.on_change_state.clone();
                                                move |this, _, window, cx| {
                                                    if let Ok(parsed_value) =
                                                        this.text(cx).parse::<T>()
                                                    {
                                                        let new_value = clamp_value(parsed_value);
                                                        let on_change =
                                                            on_change_state.read(cx).clone();

                                                        if let Some(on_change) = on_change.as_ref()
                                                        {
                                                            on_change(&new_value, window, cx);
                                                        }
                                                    };
                                                }
                                            })
                                            .detach();

                                            editor
                                        }
                                    });

                                    let focus_handle = editor.focus_handle(cx);
                                    let is_focused = focus_handle.is_focused(window);

                                    if !is_focused {
                                        let current_text = editor.read(cx).text(cx);
                                        let last_synced = *self.last_synced_value.read(cx);

                                        // Detect if the value changed externally (e.g., reset button)
                                        let value_changed_externally = last_synced
                                            .map(|last| last != self.value)
                                            .unwrap_or(true);

                                        let should_sync = if value_changed_externally {
                                            true
                                        } else {
                                            match current_text.parse::<T>().ok() {
                                                Some(parsed) => parsed == self.value,
                                                None => true,
                                            }
                                        };

                                        if should_sync && current_text != expected_text {
                                            editor.update(cx, |editor, cx| {
                                                editor.set_text(expected_text.clone(), window, cx);
                                            });
                                        }

                                        self.last_synced_value
                                            .update(cx, |state, _| *state = Some(self.value));
                                    }

                                    let focus_handle = if self.tab_index.is_some() {
                                        focus_handle.tab_index(0isize).tab_stop(true)
                                    } else {
                                        focus_handle
                                    };

                                    h_flex()
                                        .flex_1()
                                        .h_full()
                                        .track_focus(&focus_handle)
                                        .when(is_focused, |this| {
                                            this.border_1()
                                                .border_color(cx.theme().colors().border_focused)
                                        })
                                        .child(editor)
                                        .on_action::<menu::Confirm>({
                                            move |_, window, _| {
                                                window.blur();
                                            }
                                        })
                                        .into_any_element()
                                }
                            })
                    })
                    .map(|increment| {
                        let increment_handler = {
                            let on_change = on_change_for_increment.clone();
                            let get_current_value = get_current_value.clone();
                            let update_editor_text = update_editor_text.clone();

                            move |click: &ClickEvent, window: &mut Window, cx: &mut App| {
                                let current_value = get_current_value(cx);
                                let step = get_step(click.modifiers());
                                let new_value = change_value(
                                    current_value,
                                    step,
                                    ValueChangeDirection::Increment,
                                );

                                update_editor_text(new_value, window, cx);
                                on_change(&new_value, window, cx);
                            }
                        };

                        increment.child(
                            base_button(IconName::Plus)
                                .id((self.id.clone(), "increment_button"))
                                .rounded_tr_sm()
                                .rounded_br_sm()
                                .when_some(self.tab_index, |this, _| this.tab_index(0isize))
                                .on_click(increment_handler),
                        )
                    })
            })
    }
}

impl Component for NumberField<usize> {
    fn scope() -> ComponentScope {
        ComponentScope::Input
    }

    fn name() -> &'static str {
        "Number Field"
    }

    fn description() -> Option<&'static str> {
        Some("A numeric input element with increment and decrement buttons.")
    }

    fn preview(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let default_ex = window.use_state(cx, |_, _| 100.0);
        let edit_ex = window.use_state(cx, |_, _| 500.0);

        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    single_example(
                        "Button-Only Number Field",
                        NumberField::new("number-field", *default_ex.read(cx), window, cx)
                            .on_change({
                                let default_ex = default_ex.clone();
                                move |value, _, cx| default_ex.write(cx, *value)
                            })
                            .min(1.0)
                            .max(100.0)
                            .into_any_element(),
                    ),
                    single_example(
                        "Editable Number Field",
                        NumberField::new("editable-number-field", *edit_ex.read(cx), window, cx)
                            .on_change({
                                let edit_ex = edit_ex.clone();
                                move |value, _, cx| edit_ex.write(cx, *value)
                            })
                            .min(100.0)
                            .max(500.0)
                            .mode(NumberFieldMode::Edit, cx)
                            .into_any_element(),
                    ),
                ])
                .into_any_element(),
        )
    }
}
