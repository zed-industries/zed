use std::path::PathBuf;

use file_icons::FileIcons;
use gpui::{AnyElement, App};
use ui::{Divider, prelude::*};

#[derive(Clone)]
pub enum SandboxRow {
    Message(SharedString),
    Path(PathBuf),
    Git(PathBuf),
    Domain(SharedString),
}

impl SandboxRow {
    pub fn message(message: impl Into<SharedString>) -> Self {
        Self::Message(message.into())
    }

    pub fn path(path: impl Into<PathBuf>) -> Self {
        Self::Path(path.into())
    }

    pub fn git(path: impl Into<PathBuf>) -> Self {
        Self::Git(path.into())
    }

    pub fn domain(domain: impl Into<SharedString>) -> Self {
        Self::Domain(domain.into())
    }

    fn render(self, cx: &App) -> AnyElement {
        let icon_basic = |icon_name: IconName| {
            Icon::new(icon_name)
                .color(Color::Muted)
                .size(IconSize::Small)
        };

        let (icon, label) = match self {
            SandboxRow::Message(message) => {
                return Label::new(message)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
                    .into_any_element();
            }
            SandboxRow::Path(path) => {
                let icon = FileIcons::get_icon(&path, cx)
                    .map(|icon| {
                        Icon::from_path(icon)
                            .color(Color::Muted)
                            .size(IconSize::Small)
                    })
                    .unwrap_or_else(|| icon_basic(IconName::Folder));
                (icon, path.display().to_string())
            }
            SandboxRow::Git(path) => (icon_basic(IconName::GitBranch), path.display().to_string()),
            SandboxRow::Domain(domain) => (icon_basic(IconName::Public), domain.to_string()),
        };

        h_flex()
            .items_start()
            .min_w_0()
            .gap_1p5()
            .child(icon)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .child(Label::new(label).size(LabelSize::XSmall).buffer_font(cx)),
            )
            .into_any_element()
    }
}

#[derive(Clone)]
pub struct SandboxGroup {
    heading: SharedString,
    rows: Vec<SandboxRow>,
}

impl SandboxGroup {
    pub fn new(heading: impl Into<SharedString>) -> Self {
        Self {
            heading: heading.into(),
            rows: Vec::new(),
        }
    }

    pub fn row(mut self, row: SandboxRow) -> Self {
        self.rows.push(row);
        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = SandboxRow>) -> Self {
        self.rows.extend(rows);
        self
    }

    fn render(self, cx: &App) -> impl IntoElement {
        v_flex()
            .gap_1p5()
            .child(
                Label::new(self.heading)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .children(self.rows.into_iter().map(|row| row.render(cx)))
    }
}

#[derive(Clone)]
pub struct SandboxSection {
    title: SharedString,
    groups: Vec<SandboxGroup>,
}

impl SandboxSection {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            groups: Vec::new(),
        }
    }

    pub fn group(mut self, group: SandboxGroup) -> Self {
        self.groups.push(group);
        self
    }

    fn render(self, cx: &App) -> AnyElement {
        v_flex()
            .gap_2()
            .child(Label::new(self.title).size(LabelSize::Small))
            .children(self.groups.into_iter().map(|group| {
                v_flex()
                    .gap_2()
                    .child(Divider::horizontal())
                    .child(group.render(cx))
            }))
            .into_any_element()
    }
}

#[derive(Clone, IntoElement, RegisterComponent)]
pub enum SandboxStatusTooltip {
    Enabled {
        settings: SandboxSection,
        thread: Option<SandboxSection>,
    },
    DisabledForThread {
        settings: SandboxSection,
    },
    DisabledInSettings,
}

impl SandboxStatusTooltip {
    pub fn enabled(settings: SandboxSection, thread: Option<SandboxSection>) -> Self {
        Self::Enabled { settings, thread }
    }

    pub fn disabled_for_thread(settings: SandboxSection) -> Self {
        Self::DisabledForThread { settings }
    }

    pub fn disabled_in_settings() -> Self {
        Self::DisabledInSettings
    }
}

impl RenderOnce for SandboxStatusTooltip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let content = match self {
            SandboxStatusTooltip::DisabledInSettings => v_flex()
                .child(
                    Label::new("You have sandboxing disabled in settings.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
            SandboxStatusTooltip::DisabledForThread { settings } => v_flex()
                .gap_1()
                .child(div().opacity(0.5).child(settings.render(cx)))
                .child(Divider::horizontal())
                .child(Label::new("Sandboxing is disabled for this thread").size(LabelSize::Small))
                .into_any_element(),
            SandboxStatusTooltip::Enabled { settings, thread } => v_flex()
                .gap_2()
                .child(settings.render(cx))
                .children(thread.map(|thread| {
                    v_flex()
                        .gap_2()
                        .child(Divider::horizontal())
                        .child(thread.render(cx))
                }))
                .into_any_element(),
        };

        v_flex()
            .w(rems_from_px(280.))
            .gap_1()
            .child(Label::new("Sandboxing"))
            .child(content)
    }
}

impl Component for SandboxStatusTooltip {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn name() -> &'static str {
        "Sandbox Status Tooltip"
    }

    fn description() -> &'static str {
        "The tooltip shown on the sandboxing lock icon in the agent panel, \
        describing the filesystem, network, and Git access granted to the \
        agent for each of the possible sandbox states."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        let settings_section = SandboxSection::new("Defined in your settings:")
            .group(SandboxGroup::new("Write Access").rows([
                SandboxRow::path("/Users/you/project"),
                SandboxRow::path("/tmp (isolated)"),
            ]))
            .group(SandboxGroup::new("Network Access").rows([
                SandboxRow::domain("github.com"),
                SandboxRow::domain("*.npmjs.org"),
            ]));

        let thread_section = SandboxSection::new("Allowed for this thread:")
            .group(
                SandboxGroup::new("Write Access").row(SandboxRow::path("/Users/you/project/build")),
            )
            .group(SandboxGroup::new("Network Access").row(SandboxRow::message("None")))
            .group(
                SandboxGroup::new("Git Metadata Access")
                    .row(SandboxRow::git("/Users/you/project/.git")),
            );

        let unrestricted_section = SandboxSection::new("Defined in your settings:")
            .group(
                SandboxGroup::new("Write Access")
                    .row(SandboxRow::message("All paths (unrestricted)")),
            )
            .group(
                SandboxGroup::new("Network Access")
                    .row(SandboxRow::message("All domains (unrestricted)")),
            );

        let container = || div().p_2().elevation_2(cx).max_w_112();

        v_flex()
            .gap_4()
            .child(example_group(vec![
                single_example(
                    "Enabled",
                    container()
                        .child(SandboxStatusTooltip::enabled(
                            settings_section.clone(),
                            Some(thread_section),
                        ))
                        .into_any_element(),
                ),
                single_example(
                    "Enabled (unrestricted, no overrides)",
                    container()
                        .child(SandboxStatusTooltip::enabled(unrestricted_section, None))
                        .into_any_element(),
                ),
                single_example(
                    "Disabled for thread",
                    container()
                        .child(SandboxStatusTooltip::disabled_for_thread(settings_section))
                        .into_any_element(),
                ),
                single_example(
                    "Disabled in settings",
                    container()
                        .child(SandboxStatusTooltip::disabled_in_settings())
                        .into_any_element(),
                ),
            ]))
            .into_any_element()
    }
}
