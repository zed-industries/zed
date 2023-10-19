use std::ops::Deref;

use gpui3::{
    rems, AbsoluteLength, AnyElement, BorrowAppContext, Bounds, LayoutId, Pixels, WindowContext,
};

use crate::prelude::*;

/// Returns the user settings.
pub fn user_settings(cx: &WindowContext) -> FakeSettings {
    cx.global::<FakeSettings>().clone()
}

#[derive(Clone)]
pub enum SettingValue<T> {
    UserDefined(T),
    Default(T),
}

impl<T> Deref for SettingValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::UserDefined(value) => value,
            Self::Default(value) => value,
        }
    }
}

#[derive(Clone)]
pub struct TitlebarSettings {
    pub show_project_owner: SettingValue<bool>,
    pub show_git_status: SettingValue<bool>,
    pub show_git_controls: SettingValue<bool>,
}

impl Default for TitlebarSettings {
    fn default() -> Self {
        Self {
            show_project_owner: SettingValue::Default(true),
            show_git_status: SettingValue::Default(true),
            show_git_controls: SettingValue::Default(true),
        }
    }
}

// These should be merged into settings
#[derive(Clone)]
pub struct FakeSettings {
    pub default_panel_size: SettingValue<AbsoluteLength>,
    pub list_disclosure_style: SettingValue<DisclosureControlStyle>,
    pub list_indent_depth: SettingValue<AbsoluteLength>,
    pub titlebar: TitlebarSettings,
    pub ui_scale: SettingValue<f32>,
}

impl Default for FakeSettings {
    fn default() -> Self {
        Self {
            titlebar: TitlebarSettings::default(),
            list_disclosure_style: SettingValue::Default(DisclosureControlStyle::ChevronOnHover),
            list_indent_depth: SettingValue::Default(rems(0.3).into()),
            default_panel_size: SettingValue::Default(rems(16.).into()),
            ui_scale: SettingValue::Default(1.),
        }
    }
}

impl FakeSettings {}

pub fn with_settings<E, F>(
    settings: FakeSettings,
    cx: &mut ViewContext<E::ViewState>,
    build_child: F,
) -> WithSettings<E>
where
    E: Element,
    F: FnOnce(&mut ViewContext<E::ViewState>) -> E,
{
    let child = cx.with_global(settings.clone(), |cx| build_child(cx));
    WithSettings { settings, child }
}

pub struct WithSettings<E> {
    pub(crate) settings: FakeSettings,
    pub(crate) child: E,
}

impl<E> IntoAnyElement<E::ViewState> for WithSettings<E>
where
    E: Element,
{
    fn into_any(self) -> AnyElement<E::ViewState> {
        AnyElement::new(self)
    }
}

impl<E: Element> Element for WithSettings<E> {
    type ViewState = E::ViewState;
    type ElementState = E::ElementState;

    fn id(&self) -> Option<gpui3::ElementId> {
        None
    }

    fn initialize(
        &mut self,
        view_state: &mut Self::ViewState,
        element_state: Option<Self::ElementState>,
        cx: &mut ViewContext<Self::ViewState>,
    ) -> Self::ElementState {
        cx.with_global(self.settings.clone(), |cx| {
            self.child.initialize(view_state, element_state, cx)
        })
    }

    fn layout(
        &mut self,
        view_state: &mut E::ViewState,
        element_state: &mut Self::ElementState,
        cx: &mut ViewContext<E::ViewState>,
    ) -> LayoutId
    where
        Self: Sized,
    {
        cx.with_global(self.settings.clone(), |cx| {
            self.child.layout(view_state, element_state, cx)
        })
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        view_state: &mut Self::ViewState,
        frame_state: &mut Self::ElementState,
        cx: &mut ViewContext<Self::ViewState>,
    ) where
        Self: Sized,
    {
        cx.with_global(self.settings.clone(), |cx| {
            self.child.paint(bounds, view_state, frame_state, cx);
        });
    }
}
