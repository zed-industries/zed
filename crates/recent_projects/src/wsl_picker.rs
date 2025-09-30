use std::{path::PathBuf, sync::Arc};

use gpui::{AppContext, DismissEvent, Entity, EventEmitter, Focusable, Subscription, Task};
use picker::Picker;
use remote::{RemoteConnectionOptions, WslConnectionOptions};
use ui::{
    App, Context, HighlightedLabel, Icon, IconName, InteractiveElement, ListItem, ParentElement,
    Render, Styled, StyledExt, Toggleable, Window, div, h_flex, rems, v_flex,
};
use util::ResultExt as _;
use workspace::{ModalView, Workspace};

use crate::open_remote_project;

#[derive(Clone, Debug)]
pub struct WslDistroSelected {
    pub secondary: bool,
    pub distro: String,
}

#[derive(Clone, Debug)]
pub struct WslPickerDismissed;

pub(crate) struct WslPickerDelegate {
    selected_index: usize,
    distro_list: Option<Vec<String>>,
    matches: Vec<fuzzy::StringMatch>,
}

impl WslPickerDelegate {
    pub fn new() -> Self {
        WslPickerDelegate {
            selected_index: 0,
            distro_list: None,
            matches: Vec::new(),
        }
    }

    pub fn selected_distro(&self) -> Option<String> {
        self.matches
            .get(self.selected_index)
            .map(|m| m.string.clone())
    }
}

impl WslPickerDelegate {
    fn fetch_distros() -> anyhow::Result<Vec<String>> {
        use anyhow::Context;
        use windows_registry::CURRENT_USER;

        let lxss_key = CURRENT_USER
            .open("Software\\Microsoft\\Windows\\CurrentVersion\\Lxss")
            .context("failed to get lxss wsl key")?;

        let distros = lxss_key
            .keys()
            .context("failed to get wsl distros")?
            .filter_map(|key| {
                lxss_key
                    .open(&key)
                    .context("failed to open subkey for distro")
                    .log_err()
            })
            .filter_map(|distro| distro.get_string("DistributionName").ok())
            .collect::<Vec<_>>();

        Ok(distros)
    }
}

impl EventEmitter<WslDistroSelected> for Picker<WslPickerDelegate> {}

impl EventEmitter<WslPickerDismissed> for Picker<WslPickerDelegate> {}

impl picker::PickerDelegate for WslPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::from("Enter WSL distro name")
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        use fuzzy::StringMatchCandidate;

        let needs_fetch = self.distro_list.is_none();
        if needs_fetch {
            let distros = Self::fetch_distros().log_err();
            self.distro_list = distros;
        }

        if let Some(distro_list) = &self.distro_list {
            use ordered_float::OrderedFloat;

            let candidates = distro_list
                .iter()
                .enumerate()
                .map(|(id, distro)| StringMatchCandidate::new(id, distro))
                .collect::<Vec<_>>();

            let query = query.trim_start();
            let smart_case = query.chars().any(|c| c.is_uppercase());
            self.matches = smol::block_on(fuzzy::match_strings(
                candidates.as_slice(),
                query,
                smart_case,
                true,
                100,
                &Default::default(),
                cx.background_executor().clone(),
            ));
            self.matches.sort_unstable_by_key(|m| m.candidate_id);

            self.selected_index = self
                .matches
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|(_, m)| OrderedFloat(m.score))
                .map(|(index, _)| index)
                .unwrap_or(0);
        }

        Task::ready(())
    }

    fn confirm(&mut self, secondary: bool, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(distro) = self.matches.get(self.selected_index) {
            cx.emit(WslDistroSelected {
                secondary,
                distro: distro.string.clone(),
            });
        }
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(WslPickerDismissed);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let matched = self.matches.get(ix)?;
        Some(
            ListItem::new(ix)
                .toggle_state(selected)
                .inset(true)
                .spacing(ui::ListItemSpacing::Sparse)
                .child(
                    h_flex()
                        .flex_grow()
                        .gap_3()
                        .child(Icon::new(IconName::Linux))
                        .child(v_flex().child(HighlightedLabel::new(
                            matched.string.clone(),
                            matched.positions.clone(),
                        ))),
                ),
        )
    }
}

pub(crate) struct WslOpenModal {
    paths: Vec<PathBuf>,
    create_new_window: bool,
    picker: Entity<Picker<WslPickerDelegate>>,
    _subscriptions: [Subscription; 2],
}

impl WslOpenModal {
    pub fn new(
        paths: Vec<PathBuf>,
        create_new_window: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = WslPickerDelegate::new();
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(false));

        let selected = cx.subscribe_in(
            &picker,
            window,
            |this, _, event: &WslDistroSelected, window, cx| {
                this.confirm(&event.distro, event.secondary, window, cx);
            },
        );

        let dismissed = cx.subscribe_in(
            &picker,
            window,
            |this, _, _: &WslPickerDismissed, window, cx| {
                this.cancel(&menu::Cancel, window, cx);
            },
        );

        WslOpenModal {
            paths,
            create_new_window,
            picker,
            _subscriptions: [selected, dismissed],
        }
    }

    fn confirm(
        &mut self,
        distro: &str,
        secondary: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let app_state = workspace::AppState::global(cx);
        let Some(app_state) = app_state.upgrade() else {
            return;
        };

        let connection_options = RemoteConnectionOptions::Wsl(WslConnectionOptions {
            distro_name: distro.to_string(),
            user: None,
        });

        let replace_current_window = match self.create_new_window {
            true => secondary,
            false => !secondary,
        };
        let replace_window = match replace_current_window {
            true => window.window_handle().downcast::<Workspace>(),
            false => None,
        };

        let paths = self.paths.clone();
        let open_options = workspace::OpenOptions {
            replace_window,
            ..Default::default()
        };

        cx.emit(DismissEvent);
        cx.spawn_in(window, async move |_, cx| {
            open_remote_project(connection_options, paths, app_state, open_options, cx).await
        })
        .detach();
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl ModalView for WslOpenModal {}

impl Focusable for WslOpenModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for WslOpenModal {}

impl Render for WslOpenModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        div()
            .on_mouse_down_out(cx.listener(|_, _, _, cx| cx.emit(DismissEvent)))
            .on_action(cx.listener(Self::cancel))
            .elevation_3(cx)
            .w(rems(34.))
            .flex_1()
            .overflow_hidden()
            .child(self.picker.clone())
    }
}
