use crate::{prelude::*, Checkbox, ListHeader};

use super::DropdownMenu;

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum ToggleType {
    Checkbox,
    // Switch,
}

impl From<ToggleType> for SettingType {
    fn from(toggle_type: ToggleType) -> Self {
        SettingType::Toggle(toggle_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputType {
    Text,
    Number,
}

impl From<InputType> for SettingType {
    fn from(input_type: InputType) -> Self {
        SettingType::Input(input_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecondarySettingType {
    Dropdown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingType {
    Toggle(ToggleType),
    ToggleAnd(SecondarySettingType),
    Input(InputType),
    Dropdown,
    Range,
    Unsupported,
}

#[derive(Debug, Clone, IntoElement)]
pub struct SettingsGroup {
    pub name: String,
    settings: Vec<SettingsItem>,
}

impl SettingsGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            settings: Vec::new(),
        }
    }

    pub fn add_setting(mut self, setting: SettingsItem) -> Self {
        self.settings.push(setting);
        self
    }
}

impl RenderOnce for SettingsGroup {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let empty_message = format!("No settings available for {}", self.name);

        let header = ListHeader::new(self.name);

        let settings = self.settings.clone().into_iter();

        v_flex()
            .p_1()
            .gap_2()
            .child(header)
            .when(self.settings.len() == 0, |this| {
                this.child(Label::new(empty_message))
            })
            .children(settings)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettingLayout {
    Stacked,
    AutoWidth,
    FullLine,
    FullLineJustified,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingId(pub SharedString);

impl From<SettingId> for ElementId {
    fn from(id: SettingId) -> Self {
        ElementId::Name(id.0)
    }
}

impl From<&str> for SettingId {
    fn from(id: &str) -> Self {
        Self(id.to_string().into())
    }
}

impl From<SharedString> for SettingId {
    fn from(id: SharedString) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingValue(pub SharedString);

impl From<SharedString> for SettingValue {
    fn from(value: SharedString) -> Self {
        Self(value)
    }
}

impl From<String> for SettingValue {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<bool> for SettingValue {
    fn from(value: bool) -> Self {
        Self(value.to_string().into())
    }
}

impl From<SettingValue> for bool {
    fn from(value: SettingValue) -> Self {
        value.0 == "true"
    }
}

#[derive(Debug, Clone, IntoElement)]
pub struct SettingsItem {
    pub id: SettingId,
    current_value: Option<SettingValue>,
    disabled: bool,
    hide_label: bool,
    icon: Option<IconName>,
    layout: SettingLayout,
    name: SharedString,
    // possible_values: Option<Vec<SettingValue>>,
    setting_type: SettingType,
    toggled: Option<bool>,
}

impl SettingsItem {
    pub fn new(
        id: impl Into<SettingId>,
        name: SharedString,
        setting_type: SettingType,
        current_value: Option<SettingValue>,
    ) -> Self {
        let toggled = match setting_type {
            SettingType::Toggle(_) | SettingType::ToggleAnd(_) => Some(false),
            _ => None,
        };

        Self {
            id: id.into(),
            current_value,
            disabled: false,
            hide_label: false,
            icon: None,
            layout: SettingLayout::FullLine,
            name,
            // possible_values: None,
            setting_type,
            toggled,
        }
    }

    pub fn layout(mut self, layout: SettingLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn toggled(mut self, toggled: bool) -> Self {
        self.toggled = Some(toggled);
        self
    }

    // pub fn hide_label(mut self, hide_label: bool) -> Self {
    //     self.hide_label = hide_label;
    //     self
    // }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    // pub fn disabled(mut self, disabled: bool) -> Self {
    //     self.disabled = disabled;
    //     self
    // }
}

impl RenderOnce for SettingsItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let id: ElementId = self.id.clone().into();

        // When the setting is disabled or toggled off, we don't want any secondary elements to be interactable
        let secondary_element_disabled = self.disabled || self.toggled == Some(false);

        let full_width = match self.layout {
            SettingLayout::FullLine | SettingLayout::FullLineJustified => true,
            _ => false,
        };

        let hide_label = self.hide_label || self.icon.is_some();

        let justified = match (self.layout.clone(), self.setting_type.clone()) {
            (_, SettingType::ToggleAnd(_)) => true,
            (SettingLayout::FullLineJustified, _) => true,
            _ => false,
        };

        let (setting_type, current_value) = (self.setting_type.clone(), self.current_value.clone());
        let current_string = if let Some(current_value) = current_value.clone() {
            Some(current_value.0)
        } else {
            None
        };

        let toggleable = match setting_type {
            SettingType::Toggle(_) => true,
            SettingType::ToggleAnd(_) => true,
            _ => false,
        };

        let setting_element = match setting_type {
            SettingType::Toggle(_) => None,
            SettingType::ToggleAnd(secondary_setting_type) => match secondary_setting_type {
                SecondarySettingType::Dropdown => Some(
                    DropdownMenu::new(id.clone(), &cx)
                        .current_item(current_string)
                        .disabled(secondary_element_disabled)
                        .into_any_element(),
                ),
            },
            SettingType::Input(input_type) => match input_type {
                InputType::Text => Some(div().child("text").into_any_element()),
                InputType::Number => Some(div().child("number").into_any_element()),
            },
            SettingType::Dropdown => Some(
                DropdownMenu::new(id.clone(), &cx)
                    .current_item(current_string)
                    .full_width(true)
                    .into_any_element(),
            ),
            SettingType::Range => Some(div().child("range").into_any_element()),
            SettingType::Unsupported => None,
        };

        let checkbox = Checkbox::new(
            ElementId::Name(format!("toggle-{}", self.id.0).to_string().into()),
            self.toggled.into(),
        )
        .disabled(self.disabled);

        let toggle_element = match (toggleable, self.setting_type.clone()) {
            (true, SettingType::Toggle(toggle_type)) => match toggle_type {
                ToggleType::Checkbox => Some(checkbox.into_any_element()),
            },
            (true, SettingType::ToggleAnd(_)) => Some(checkbox.into_any_element()),
            (_, _) => None,
        };

        let item = if self.layout == SettingLayout::Stacked {
            v_flex()
        } else {
            h_flex()
        };

        item.id(id)
            .gap_2()
            .w_full()
            .when_some(self.icon, |this, icon| {
                this.child(div().px_0p5().child(Icon::new(icon).color(Color::Muted)))
            })
            .children(toggle_element)
            .children(if hide_label {
                None
            } else {
                Some(Label::new(self.name.clone()))
            })
            .when(justified, |this| this.child(div().flex_1().size_full()))
            .child(
                h_flex()
                    .when(full_width, |this| this.w_full())
                    .when(self.layout == SettingLayout::FullLineJustified, |this| {
                        this.justify_end()
                    })
                    .children(setting_element),
            )
            // help flex along when full width is disabled
            //
            // this probably isn't needed, but fighting with flex to
            // get this right without inspection tools will be a pain
            .when(!full_width, |this| this.child(div().size_full().flex_1()))
    }
}

pub struct SettingsMenu {
    name: SharedString,
    groups: Vec<SettingsGroup>,
}

impl SettingsMenu {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            groups: Vec::new(),
        }
    }

    pub fn add_group(mut self, group: SettingsGroup) -> Self {
        self.groups.push(group);
        self
    }
}

impl Render for SettingsMenu {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let is_empty = self.groups.is_empty();
        v_flex()
            .id(ElementId::Name(self.name.clone()))
            .elevation_2(cx)
            .min_w_56()
            .max_w_96()
            .max_h_2_3()
            .px_2()
            .when_else(
                is_empty,
                |empty| empty.py_1(),
                |not_empty| not_empty.pt_0().pb_1(),
            )
            .gap_1()
            .when(is_empty, |this| {
                this.child(Label::new("No settings found").color(Color::Muted))
            })
            .children(self.groups.clone())
    }
}
