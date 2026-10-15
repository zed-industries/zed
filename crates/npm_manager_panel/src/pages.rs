//! The five `SettingPage`s of the npm manager tab.
//!
//! Each page is a *view shell*: a cheap `SettingPage` whose body is a single
//! `SettingItem::render` that embeds a dedicated child `Entity<...>` view. The
//! heavy rows live in those child views and are built — by reading the panel
//! live through a `WeakEntity<NpmManagerPanel>` in each view's `Render::render`
//! — only for the page currently open in the `Settings` widget, which renders
//! just the active page's items.
//!
//! Building the pages therefore no longer clones the package lists up front,
//! so a re-render of the panel (e.g. the 60s auto-refresh) never traces through
//! every installed / outdated / audit / search row the way the old
//! `build_*_page` functions did.

use gpui::{
    AnyElement, App, AppContext as _, Context, Entity, FocusHandle, Hsla, InteractiveElement as _,
    IntoElement, MouseButton, ParentElement as _, Render, StatefulInteractiveElement as _,
    Styled as _, WeakEntity, Window, div, prelude::FluentBuilder as _,
};
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _, Size, StyledExt as _,
    button::{Button, ButtonVariants as _},
    h_flex,
    input::Input,
    setting::{SettingGroup, SettingItem, SettingPage},
    spinner::Spinner,
    v_flex,
};
use npm_backend::{PackageManager, UpdateKind, Verdict, classify_update};

use crate::{
    ActSelectedPackage, NpmManagerPanel, OpenSelectedPackage, SelectNextPackage, SelectPrevPackage,
};

/// The key context each package-list page's own container sets — see
/// `npm_manager_panel::init`'s `cx.bind_keys` for the up/down/enter/space
/// bindings scoped to it, and why they live there rather than in the JSON
/// keymap (mirrors `gpui_component::table::data_table`'s own `DataTable`
/// context for the same reason).
const PACKAGE_LIST_CONTEXT: &str = "NpmPackageList";

/// Moves `selected` one row up (`forward: false`) or down (`forward: true`)
/// within a `len`-row list, wrapping at both ends — matches
/// `gpui_component::table::TableState`'s default `loop_selection` behavior
/// — and starting from the top row on the very first press.
fn step_selected(selected: Option<usize>, len: usize, forward: bool) -> Option<usize> {
    if len == 0 {
        return None;
    }
    Some(match selected {
        // Nothing selected yet: the first press lands on the nearest end
        // rather than stepping past it.
        None => {
            if forward {
                0
            } else {
                len - 1
            }
        }
        Some(ix) => {
            if forward {
                (ix + 1) % len
            } else {
                (ix + len - 1) % len
            }
        }
    })
}

/// The five page views created alongside the panel. `PageViews::new` builds
/// them from a `WeakEntity` of the panel so they can read its state live in
/// their own `Render::render` without owning any of the data.
pub(super) struct PageViews {
    pub general: Entity<GeneralPage>,
    pub search: Entity<SearchPage>,
    pub installed: Entity<InstalledPage>,
    pub updates: Entity<UpdatesPage>,
    pub vuln: Entity<VulnPage>,
}

impl PageViews {
    pub(super) fn new(panel: WeakEntity<NpmManagerPanel>, cx: &mut App) -> Self {
        PageViews {
            general: cx.new(|_| GeneralPage::new(panel.clone())),
            search: cx.new(|cx| SearchPage::new(panel.clone(), cx)),
            installed: cx.new(|cx| InstalledPage::new(panel.clone(), cx)),
            updates: cx.new(|cx| UpdatesPage::new(panel.clone(), cx)),
            vuln: cx.new(|cx| VulnPage::new(panel.clone(), cx)),
        }
    }
}

pub(super) fn build_all(project: &crate::NpmProject, pages: &PageViews) -> Vec<SettingPage> {
    vec![
        build_general_page(pages),
        build_search_page(pages),
        build_installed_page(project, pages),
        build_updates_page(project, pages),
        build_vuln_page(project, pages),
    ]
}

/// A `SettingItem` that simply embeds a page view; the shell adds nothing else
/// per frame, so re-renders stay O(1) regardless of the lists' length.
fn embed_view<T>(view: Entity<T>) -> SettingItem
where
    T: Render + 'static,
{
    SettingItem::render(move |_options, _window, _cx| view.clone().into_any_element())
}

// ── General ────────────────────────────────────────────────────────────────

pub(super) struct GeneralPage {
    panel: WeakEntity<NpmManagerPanel>,
}

impl GeneralPage {
    fn new(panel: WeakEntity<NpmManagerPanel>) -> Self {
        Self { panel }
    }
}

impl Render for GeneralPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(view) = self.panel.upgrade() else {
            return div().into_any_element();
        };
        let panel = view.read(cx);
        let project = panel.active();
        let theme = cx.theme();

        let manager = project.package_manager.cli_name().to_string();
        let dev = panel.install_as_dev;
        let busy = panel.running_action.is_some()
            || project.installed_loading
            || project.outdated_loading
            || project.audit_loading;
        let running_action = panel.running_action.clone();
        let error = panel.error.clone();
        let engine = project.engine.clone();

        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .gap_1()
                    .children(["npm", "yarn", "pnpm", "bun"].map(|name| {
                        let view = view.clone();
                        Button::new(format!("npm-pm-{name}"))
                            .label(name)
                            .with_size(Size::Small)
                            .map(|button| {
                                if manager == name {
                                    button.primary()
                                } else {
                                    button.outline()
                                }
                            })
                            .on_click(move |_, _window, cx| {
                                view.update(cx, |panel, cx| {
                                    panel.active_mut().package_manager = match name {
                                        "yarn" => PackageManager::Yarn,
                                        "pnpm" => PackageManager::Pnpm,
                                        "bun" => PackageManager::Bun,
                                        _ => PackageManager::Npm,
                                    };
                                    cx.notify();
                                });
                            })
                    })),
            )
            .child(
                Button::new("npm-toggle-dev")
                    .label(if dev {
                        "Save new packages as devDependencies"
                    } else {
                        "Save new packages as dependencies"
                    })
                    .with_size(Size::Small)
                    .map(|button| {
                        if dev {
                            button.primary()
                        } else {
                            button.outline()
                        }
                    })
                    .on_click({
                        let view = view.clone();
                        move |_, _window, cx| {
                            view.update(cx, |panel, cx| {
                                panel.install_as_dev = !panel.install_as_dev;
                                cx.notify();
                            });
                        }
                    }),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Button::new("npm-reload")
                            .icon(IconName::Redo2)
                            .label("Refresh")
                            .with_size(Size::Small)
                            .on_click({
                                let view = view.clone();
                                move |_, _window, cx| {
                                    view.update(cx, |panel, cx| panel.reload_visible(cx));
                                }
                            }),
                    )
                    .when(busy, |row| {
                        row.child(Spinner::new().with_size(Size::XSmall))
                    }),
            )
            .child({
                let mut status = v_flex().gap_1().text_xs();
                if let Some(action) = &running_action {
                    status = status.child(
                        div()
                            .text_color(theme.foreground)
                            .child(format!("{action}…")),
                    );
                }
                if let Some(err) = &error {
                    status =
                        status.child(div().text_color(theme.danger).truncate().child(err.clone()));
                }
                if let Some(engine) = &engine {
                    // A wrong version is a warning while browsing; installing
                    // asks first (`NpmManagerPanel::run_checked`).
                    let (verdict, text) = engine.summary();
                    let color = match verdict {
                        Verdict::Fine => theme.success,
                        Verdict::Unknown => theme.muted_foreground,
                        Verdict::Problem => theme.warning,
                    };
                    status = status.child(
                        div()
                            .id("npm-engine-status")
                            .text_color(color)
                            .truncate()
                            .child(text.clone())
                            .tooltip(move |window, cx| {
                                gpui_component::tooltip::Tooltip::new(text.clone())
                                    .build(window, cx)
                            }),
                    );
                }
                status
            })
            .into_any_element()
    }
}

fn build_general_page(pages: &PageViews) -> SettingPage {
    SettingPage::new("General")
        .default_open(true)
        .resettable(false)
        .icon(Icon::new(IconName::Settings))
        .description("The package manager and install behavior npm commands use.")
        .group(
            SettingGroup::new()
                .title("Package manager")
                .description("Which CLI the install / remove / update actions run.")
                .item(embed_view(pages.general.clone())),
        )
}

// ── Search ─────────────────────────────────────────────────────────────────

pub(super) struct SearchPage {
    panel: WeakEntity<NpmManagerPanel>,
    focus_handle: FocusHandle,
    /// Index into `panel.search_results` — see `InstalledPage::selected`.
    /// Scoped to just the results list below the search box (not the whole
    /// page), so the search `Input`'s own arrow-key/Enter behavior while
    /// typing is untouched — see the results `v_flex`'s own
    /// `track_focus`/`key_context` below, not the page root.
    selected: Option<usize>,
}

impl SearchPage {
    fn new(panel: WeakEntity<NpmManagerPanel>, cx: &mut Context<Self>) -> Self {
        Self { panel, focus_handle: cx.focus_handle(), selected: None }
    }

    fn select_next(&mut self, _: &SelectNextPackage, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(view) = self.panel.upgrade() else { return };
        let len = view.read(cx).search_results.len();
        self.selected = step_selected(self.selected, len, true);
        cx.notify();
    }

    fn select_prev(&mut self, _: &SelectPrevPackage, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(view) = self.panel.upgrade() else { return };
        let len = view.read(cx).search_results.len();
        self.selected = step_selected(self.selected, len, false);
        cx.notify();
    }

    fn open_selected(
        &mut self,
        _: &OpenSelectedPackage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = self.panel.upgrade() else { return };
        let Some(name) = self
            .selected
            .and_then(|ix| view.read(cx).search_results.get(ix))
            .map(|r| r.name.clone())
        else {
            return;
        };
        view.update(cx, |panel, cx| panel.fetch_details_and_readme(name, cx));
    }

    /// Space's action on this page: Install — not destructive, so unlike
    /// Remove/Uninstall this needs no confirmation, matching the existing
    /// Install button.
    fn act_on_selected(
        &mut self,
        _: &ActSelectedPackage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = self.panel.upgrade() else { return };
        let Some((name, version, dev)) = self.selected.and_then(|ix| {
            let panel = view.read(cx);
            let r = panel.search_results.get(ix)?;
            Some((r.name.clone(), r.version.clone(), panel.install_as_dev))
        }) else {
            return;
        };
        view.update(cx, |panel, cx| panel.install_pkg(&name, &version, dev, window, cx));
    }
}

impl Render for SearchPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(view) = self.panel.upgrade() else {
            return div().into_any_element();
        };
        let panel = view.read(cx);
        let theme = cx.theme();

        let search_input = panel.search_input.clone();
        let search_total = panel.search_total;
        let search_loading = panel.search_loading;
        let search_error = panel.search_error.clone();
        let results = &panel.search_results;
        let selected = self.selected;

        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(Input::new(&search_input).xsmall().w_full()),
                    )
                    .child(
                        Button::new("npm-search-go")
                            .icon(IconName::Search)
                            .label("Search")
                            .with_size(Size::Small)
                            .on_click({
                                let view = view.clone();
                                move |_, _window, cx| {
                                    view.update(cx, |panel, cx| {
                                        panel.search_page = 0;
                                        panel.search_npm(cx);
                                    });
                                }
                            }),
                    ),
            )
            .child({
                let content: AnyElement = if search_loading {
                    h_flex()
                        .gap_2()
                        .items_center()
                        .child(Spinner::new().with_size(Size::XSmall))
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.muted_foreground)
                                .child("Searching npmjs.com…"),
                        )
                        .into_any_element()
                } else if let Some(err) = &search_error {
                    div()
                        .text_xs()
                        .text_color(theme.danger)
                        .child(err.clone())
                        .into_any_element()
                } else if results.is_empty() {
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("Search the npm registry for a package to install.")
                        .into_any_element()
                } else {
                    let mut list = v_flex()
                        .id("npm-search-results-list")
                        .track_focus(&self.focus_handle)
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    window.focus(&this.focus_handle, cx);
                }))
                        .key_context(PACKAGE_LIST_CONTEXT)
                        .on_action(cx.listener(Self::select_next))
                        .on_action(cx.listener(Self::select_prev))
                        .on_action(cx.listener(Self::open_selected))
                        .on_action(cx.listener(Self::act_on_selected))
                        .gap_2()
                        .w_full();
                    list = list.child(
                        div()
                            .text_xs()
                            .text_color(theme.muted_foreground)
                            .child(format!("{search_total} packages found")),
                    );
                    for (ix, result) in results.iter().enumerate() {
                        let name = result.name.clone();
                        let version = result.version.clone();
                        let description = result.description.clone().unwrap_or_default();
                        let compat = result.compat;
                        let is_selected = selected == Some(ix);

                        // Card header: the package name, then the compat
                        // badge when the engines check produced an answer.
                        let mut header = h_flex().gap_2().items_center().w_full();
                        header = header.child(
                            div()
                                .id(format!("npm-result-label-{name}"))
                                .text_sm()
                                .font_semibold()
                                .text_color(theme.foreground)
                                .child(name.clone()),
                        );
                        match compat {
                            Some(true) => {
                                header = header.child(
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded_md()
                                        .bg(theme.success.opacity(0.2))
                                        .text_color(theme.success)
                                        .text_xs()
                                        .child("node OK"),
                                );
                            }
                            Some(false) => {
                                header = header.child(
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded_md()
                                        .bg(theme.danger.opacity(0.2))
                                        .text_color(theme.danger)
                                        .text_xs()
                                        .child("node mismatch"),
                                );
                            }
                            None => {}
                        }

                        // "latest" line: `[name@version]`, per the card spec.
                        let latest = h_flex()
                            .gap_1()
                            .items_center()
                            .w_full()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.muted_foreground)
                                    .child("latest"),
                            )
                            .child(
                                div()
                                    .px_1p5()
                                    .py_0p5()
                                    .rounded_md()
                                    .bg(theme.border.opacity(0.5))
                                    .text_color(theme.foreground)
                                    .text_xs()
                                    .child(format!("{name}@{version}")),
                            );

                        // Description row with the install action on the
                        // right edge.
                        let install_button = Button::new(format!("npm-install-{name}"))
                            .label("Install")
                            .with_size(Size::Small)
                            .on_click({
                                let view = view.clone();
                                let name = name.clone();
                                let version = version.clone();
                                move |_, window, cx| {
                                    view.update(cx, |panel, cx| {
                                        panel.install_pkg(
                                            &name,
                                            &version,
                                            panel.install_as_dev,
                                            window,
                                            cx,
                                        );
                                    });
                                }
                            });

                        let card = v_flex()
                            .id(("npm-search-card", ix))
                            .gap_1()
                            .w_full()
                            .p_2()
                            .rounded_md()
                            .border_1()
                            .border_color(if is_selected { theme.primary } else { theme.border.opacity(0.6) })
                            .when(is_selected, |card| card.bg(theme.primary.opacity(0.05)))
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.selected = Some(ix);
                                cx.notify();
                            }))
                            .child(header)
                            .child(latest)
                            .child(
                                h_flex()
                                    .gap_2()
                                    .items_center()
                                    .w_full()
                                    .child(
                                        div()
                                            .flex_1()
                                            .min_w_0()
                                            .text_xs()
                                            .text_color(theme.muted_foreground)
                                            .child(description),
                                    )
                                    .child(install_button),
                            )
                            .child(
                                h_flex().w_full().child(
                                    Button::new(format!("npm-more-info-{name}"))
                                        .ghost()
                                        .xsmall()
                                        .icon(IconName::Info)
                                        .label("More info")
                                        .on_click({
                                            let view = view.clone();
                                            let name = name.clone();
                                            move |_, _window, cx| {
                                                view.update(cx, |panel, cx| {
                                                    panel
                                                        .fetch_details_and_readme(name.clone(), cx);
                                                });
                                            }
                                        }),
                                ),
                            );
                        list = list.child(card);
                    }
                    list = list.when(results.len() < search_total, |list| {
                        list.child(
                            div().w_full().child(
                                Button::new("npm-search-more")
                                    .ghost()
                                    .label("Load more")
                                    .with_size(Size::Small)
                                    .on_click({
                                        let view = view.clone();
                                        move |_, _window, cx| {
                                            view.update(cx, |panel, cx| {
                                                panel.search_page += 1;
                                                panel.search_npm(cx);
                                            });
                                        }
                                    }),
                            ),
                        )
                    });
                    list.into_any_element()
                };
                content
            })
            .into_any_element()
    }
}

fn build_search_page(pages: &PageViews) -> SettingPage {
    SettingPage::new("Search")
        .resettable(false)
        .icon(Icon::new(IconName::Search))
        .description("Find packages on the npm registry and install them.")
        .group(
            SettingGroup::new()
                .title("npmjs.com")
                .item(embed_view(pages.search.clone())),
        )
}

// ── Installed ──────────────────────────────────────────────────────────────

pub(super) struct InstalledPage {
    panel: WeakEntity<NpmManagerPanel>,
    focus_handle: FocusHandle,
    /// Index into `project.installed` — the row `up`/`down`/`enter`/`space`
    /// act on. `None` until the list first gets focus (a click, or Tab).
    selected: Option<usize>,
}

impl InstalledPage {
    fn new(panel: WeakEntity<NpmManagerPanel>, cx: &mut Context<Self>) -> Self {
        Self { panel, focus_handle: cx.focus_handle(), selected: None }
    }

    fn select_next(&mut self, _: &SelectNextPackage, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(view) = self.panel.upgrade() else { return };
        let len = view.read(cx).active().installed.len();
        self.selected = step_selected(self.selected, len, true);
        cx.notify();
    }

    fn select_prev(&mut self, _: &SelectPrevPackage, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(view) = self.panel.upgrade() else { return };
        let len = view.read(cx).active().installed.len();
        self.selected = step_selected(self.selected, len, false);
        cx.notify();
    }

    fn open_selected(
        &mut self,
        _: &OpenSelectedPackage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = self.panel.upgrade() else { return };
        let Some(name) = self
            .selected
            .and_then(|ix| view.read(cx).active().installed.get(ix))
            .map(|pkg| pkg.name.clone())
        else {
            return;
        };
        view.update(cx, |panel, cx| panel.fetch_details(name, cx));
    }

    /// Space's action on this page: Remove (confirmed — see
    /// `remove_package`'s own doc comment).
    fn act_on_selected(
        &mut self,
        _: &ActSelectedPackage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = self.panel.upgrade() else { return };
        let Some(name) = self
            .selected
            .and_then(|ix| view.read(cx).active().installed.get(ix))
            .map(|pkg| pkg.name.clone())
        else {
            return;
        };
        view.update(cx, |panel, cx| panel.remove_package(&name, window, cx));
    }
}

impl Render for InstalledPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(view) = self.panel.upgrade() else {
            return div().into_any_element();
        };
        let panel = view.read(cx);
        let theme = cx.theme();
        let project = panel.active();
        let installed = &project.installed;
        let installed_loading = project.installed_loading;

        if installed_loading {
            // Rows appear once `npm ls` returns; the sidebar badge spins meanwhile.
            return div().into_any_element();
        }

        let selected = self.selected;
        let mut list = v_flex()
            .id("npm-installed-list")
            .track_focus(&self.focus_handle)
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    window.focus(&this.focus_handle, cx);
                }))
            .key_context(PACKAGE_LIST_CONTEXT)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::open_selected))
            .on_action(cx.listener(Self::act_on_selected))
            .gap_1p5();
        if installed.is_empty() {
            list = list.child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child("No packages installed — run `npm install` first."),
            );
        } else {
            for (ix, pkg) in installed.iter().enumerate() {
                let name = pkg.name.clone();
                let version = pkg.version.clone();
                let is_dev = pkg.is_dev;
                let is_selected = selected == Some(ix);

                // Card header: the package name (clickable → details), then
                // the dev badge in the same slot as the search cards' compat
                // badge.
                let mut header = h_flex().gap_2().items_center().w_full();
                header = header.child(
                    div()
                        .id(format!("npm-installed-{name}"))
                        .text_sm()
                        .font_semibold()
                        .text_color(theme.foreground)
                        .child(name.clone())
                        .cursor_pointer()
                        .on_click({
                            let view = view.clone();
                            let name = name.clone();
                            move |_, _window, cx| {
                                view.update(cx, |panel, cx| {
                                    panel.fetch_details(name.clone(), cx);
                                });
                            }
                        }),
                );
                header = header.when(is_dev, |header| {
                    header.child(
                        div()
                            .px_1p5()
                            .py_0p5()
                            .rounded_md()
                            .bg(theme.warning.opacity(0.2))
                            .text_color(theme.warning)
                            .text_xs()
                            .child("dev"),
                    )
                });

                // "installed" line: `[name@version]`, per the card spec.
                let installed_line = h_flex()
                    .gap_1()
                    .items_center()
                    .w_full()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.muted_foreground)
                            .child("installed"),
                    )
                    .child(
                        div()
                            .px_1p5()
                            .py_0p5()
                            .rounded_md()
                            .bg(theme.border.opacity(0.5))
                            .text_color(theme.foreground)
                            .text_xs()
                            .child(format!("{name}@{version}")),
                    );

                let card = v_flex()
                    .id(("npm-installed-card", ix))
                    .gap_1()
                    .w_full()
                    .p_2()
                    .rounded_md()
                    .border_1()
                    .border_color(if is_selected { theme.primary } else { theme.border.opacity(0.6) })
                    .when(is_selected, |card| card.bg(theme.primary.opacity(0.05)))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.selected = Some(ix);
                        cx.notify();
                    }))
                    .child(header)
                    .child(installed_line)
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .w_full()
                            .child(div().flex_1().min_w_0())
                            .child(
                                Button::new(format!("npm-remove-{name}"))
                                    .danger()
                                    .label("Remove")
                                    .with_size(Size::Small)
                                    .on_click({
                                        let view = view.clone();
                                        let name = name.clone();
                                        move |_, window, cx| {
                                            view.update(cx, |panel, cx| {
                                                panel.remove_package(&name, window, cx);
                                            });
                                        }
                                    }),
                            ),
                    )
                    .child(
                        h_flex().w_full().child(
                            Button::new(format!("npm-more-info-{name}"))
                                .ghost()
                                .xsmall()
                                .icon(IconName::Info)
                                .label("More info")
                                .on_click({
                                    let view = view.clone();
                                    let name = name.clone();
                                    move |_, _window, cx| {
                                        view.update(cx, |panel, cx| {
                                            panel.fetch_details(name.clone(), cx);
                                        });
                                    }
                                }),
                        ),
                    );
                list = list.child(card);
            }
        }
        list.into_any_element()
    }
}

fn build_installed_page(project: &crate::NpmProject, pages: &PageViews) -> SettingPage {
    let count = project.installed.len();
    let loading = project.installed_loading;
    SettingPage::new("Installed")
        .resettable(false)
        .icon(Icon::new(IconName::Inbox))
        .description("Packages installed in this workspace.")
        .group(
            SettingGroup::new()
                .title("Installed packages")
                .item(embed_view(pages.installed.clone())),
        )
        .sidebar_badge(move |_, cx| count_badge(count, loading, cx))
}

// ── Updates ────────────────────────────────────────────────────────────────

pub(super) struct UpdatesPage {
    panel: WeakEntity<NpmManagerPanel>,
    focus_handle: FocusHandle,
    /// Index into `project.outdated` — see `InstalledPage::selected`.
    selected: Option<usize>,
}

impl UpdatesPage {
    fn new(panel: WeakEntity<NpmManagerPanel>, cx: &mut Context<Self>) -> Self {
        Self { panel, focus_handle: cx.focus_handle(), selected: None }
    }

    fn select_next(&mut self, _: &SelectNextPackage, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(view) = self.panel.upgrade() else { return };
        let len = view.read(cx).active().outdated.len();
        self.selected = step_selected(self.selected, len, true);
        cx.notify();
    }

    fn select_prev(&mut self, _: &SelectPrevPackage, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(view) = self.panel.upgrade() else { return };
        let len = view.read(cx).active().outdated.len();
        self.selected = step_selected(self.selected, len, false);
        cx.notify();
    }

    fn open_selected(
        &mut self,
        _: &OpenSelectedPackage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = self.panel.upgrade() else { return };
        let Some(name) = self
            .selected
            .and_then(|ix| view.read(cx).active().outdated.get(ix))
            .map(|pkg| pkg.name.clone())
        else {
            return;
        };
        view.update(cx, |panel, cx| panel.fetch_details(name, cx));
    }

    /// Space's action on this page: Update to latest — not destructive (a
    /// version bump), so unlike Remove/Uninstall this needs no confirmation,
    /// matching the existing Update button.
    fn act_on_selected(
        &mut self,
        _: &ActSelectedPackage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = self.panel.upgrade() else { return };
        let Some((name, latest)) = self
            .selected
            .and_then(|ix| view.read(cx).active().outdated.get(ix))
            .map(|pkg| (pkg.name.clone(), pkg.latest.clone()))
        else {
            return;
        };
        view.update(cx, |panel, cx| panel.update_pkg(&name, &latest, window, cx));
    }
}

impl Render for UpdatesPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(view) = self.panel.upgrade() else {
            return div().into_any_element();
        };
        let panel = view.read(cx);
        let theme = cx.theme();
        let project = panel.active();
        let outdated = &project.outdated;
        let outdated_loading = project.outdated_loading;

        if outdated_loading {
            return div().into_any_element();
        }

        let selected = self.selected;
        let mut list = v_flex()
            .id("npm-updates-list")
            .track_focus(&self.focus_handle)
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    window.focus(&this.focus_handle, cx);
                }))
            .key_context(PACKAGE_LIST_CONTEXT)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::open_selected))
            .on_action(cx.listener(Self::act_on_selected))
            .gap_1p5();
        if outdated.is_empty() {
            list = list.child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child("All packages are up to date."),
            );
        } else {
            list = list.child(
                h_flex().gap_1().children(
                    [
                        ("npm-update-all-safe", "Update All Safe", "safe"),
                        ("npm-update-all-patch", "Update All Patch", "patch"),
                    ]
                    .map(|(id, label, ty)| {
                        let view = view.clone();
                        Button::new(id)
                            .label(label)
                            .with_size(Size::Small)
                            .on_click(move |_, window, cx| {
                                view.update(cx, |panel, cx| {
                                    panel.update_all(ty, window, cx);
                                });
                            })
                    }),
                ),
            );
            for (ix, pkg) in outdated.iter().enumerate() {
                let name = pkg.name.clone();
                let current = pkg.current.clone();
                let latest = pkg.latest.clone();
                let is_selected = selected == Some(ix);
                let kind = classify_update(&pkg.current, &pkg.latest);
                // Red is reserved for the Vulnerabilities page — "major"
                // here doesn't mean "broken", just "bigger diff to
                // review", matching `dotnet_panel`/`node_panel`'s corrected
                // scheme (and this page's own Vulnerabilities badge below).
                let (kind_color, kind_label) = match kind {
                    UpdateKind::Major => (theme.warning, "major"),
                    UpdateKind::Minor => (theme.info, "minor"),
                    UpdateKind::Patch => (theme.muted_foreground, "patch"),
                    UpdateKind::Current => (theme.muted_foreground, "same"),
                };

                // Card header: the package name (clickable → details), then
                // the update-kind badge in the same slot as the search cards'
                // compat badge.
                let mut header = h_flex().gap_2().items_center().w_full();
                header = header.child(
                    div()
                        .id(format!("npm-outdated-{name}"))
                        .text_sm()
                        .font_semibold()
                        .text_color(theme.foreground)
                        .child(name.clone())
                        .cursor_pointer()
                        .on_click({
                            let view = view.clone();
                            let name = name.clone();
                            move |_, _window, cx| {
                                view.update(cx, |panel, cx| {
                                    panel.fetch_details(name.clone(), cx);
                                });
                            }
                        }),
                );
                header = header.child(
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded_md()
                        .bg(kind_color.opacity(0.2))
                        .text_color(kind_color)
                        .text_xs()
                        .child(kind_label),
                );

                // "update" line: `[current → latest]`, per the card spec.
                let update_line = h_flex()
                    .gap_1()
                    .items_center()
                    .w_full()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.muted_foreground)
                            .child("update"),
                    )
                    .child(
                        div()
                            .px_1p5()
                            .py_0p5()
                            .rounded_md()
                            .bg(theme.border.opacity(0.5))
                            .text_color(theme.foreground)
                            .text_xs()
                            .child(format!("{current} → {latest}")),
                    );

                let card = v_flex()
                    .id(("npm-outdated-card", ix))
                    .gap_1()
                    .w_full()
                    .p_2()
                    .rounded_md()
                    .border_1()
                    .border_color(if is_selected { theme.primary } else { theme.border.opacity(0.6) })
                    .when(is_selected, |card| card.bg(theme.primary.opacity(0.05)))
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.selected = Some(ix);
                        cx.notify();
                    }))
                    .child(header)
                    .child(update_line)
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .w_full()
                            .child(div().flex_1().min_w_0())
                            .child(
                                Button::new(format!("npm-update-{name}"))
                                    .label("Update")
                                    .with_size(Size::Small)
                                    .on_click({
                                        let view = view.clone();
                                        let name = name.clone();
                                        let latest = latest.clone();
                                        move |_, window, cx| {
                                            view.update(cx, |panel, cx| {
                                                panel.update_pkg(&name, &latest, window, cx);
                                            });
                                        }
                                    }),
                            ),
                    )
                    .child(
                        h_flex().w_full().child(
                            Button::new(format!("npm-more-info-{name}"))
                                .ghost()
                                .xsmall()
                                .icon(IconName::Info)
                                .label("More info")
                                .on_click({
                                    let view = view.clone();
                                    let name = name.clone();
                                    move |_, _window, cx| {
                                        view.update(cx, |panel, cx| {
                                            panel.fetch_details(name.clone(), cx);
                                        });
                                    }
                                }),
                        ),
                    );
                list = list.child(card);
            }
        }
        list.into_any_element()
    }
}

fn build_updates_page(project: &crate::NpmProject, pages: &PageViews) -> SettingPage {
    let count = project.outdated.len();
    let loading = project.outdated_loading;
    SettingPage::new("Updates")
        .resettable(false)
        .icon(Icon::new(IconName::ArrowUp))
        .description("Packages behind their latest version.")
        .group(
            SettingGroup::new()
                .title("Outdated packages")
                .item(embed_view(pages.updates.clone())),
        )
        .sidebar_badge(move |_, cx| count_badge(count, loading, cx))
}

// ── Vulnerabilities ────────────────────────────────────────────────────────

pub(super) struct VulnPage {
    panel: WeakEntity<NpmManagerPanel>,
    focus_handle: FocusHandle,
    /// Index into `project.audit` — see `InstalledPage::selected`.
    selected: Option<usize>,
}

impl VulnPage {
    fn new(panel: WeakEntity<NpmManagerPanel>, cx: &mut Context<Self>) -> Self {
        Self { panel, focus_handle: cx.focus_handle(), selected: None }
    }

    fn select_next(&mut self, _: &SelectNextPackage, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(view) = self.panel.upgrade() else { return };
        let len = view.read(cx).active().audit.len();
        self.selected = step_selected(self.selected, len, true);
        cx.notify();
    }

    fn select_prev(&mut self, _: &SelectPrevPackage, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(view) = self.panel.upgrade() else { return };
        let len = view.read(cx).active().audit.len();
        self.selected = step_selected(self.selected, len, false);
        cx.notify();
    }

    fn open_selected(
        &mut self,
        _: &OpenSelectedPackage,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = self.panel.upgrade() else { return };
        let Some(package) = self
            .selected
            .and_then(|ix| view.read(cx).active().audit.get(ix))
            .map(|vuln| vuln.package.clone())
        else {
            return;
        };
        view.update(cx, |panel, cx| panel.fetch_details(package, cx));
    }

    /// Space's action on this page: Fix (update to the version the advisory
    /// names) — only when one is offered, exactly like the Fix button, which
    /// this mirrors; a no-op otherwise rather than falling back to something
    /// the row didn't offer.
    fn act_on_selected(
        &mut self,
        _: &ActSelectedPackage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(view) = self.panel.upgrade() else { return };
        let Some((package, version)) = self.selected.and_then(|ix| {
            let vuln = view.read(cx).active().audit.get(ix)?;
            Some((vuln.package.clone(), vuln.fixed_in.clone()?))
        }) else {
            return;
        };
        view.update(cx, |panel, cx| panel.update_pkg(&package, &version, window, cx));
    }
}

impl Render for VulnPage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(view) = self.panel.upgrade() else {
            return div().into_any_element();
        };
        let panel = view.read(cx);
        let theme = cx.theme();
        let project = panel.active();
        let audit = &project.audit;
        let audit_loading = project.audit_loading;

        if audit_loading {
            return div().into_any_element();
        }

        let selected = self.selected;
        let mut list = v_flex()
            .id("npm-vuln-list")
            .track_focus(&self.focus_handle)
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                    window.focus(&this.focus_handle, cx);
                }))
            .key_context(PACKAGE_LIST_CONTEXT)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_prev))
            .on_action(cx.listener(Self::open_selected))
            .on_action(cx.listener(Self::act_on_selected))
            .gap_1p5();
        if audit.is_empty() {
            list = list.child(
                div()
                    .text_xs()
                    .text_color(theme.success)
                    .child("No known vulnerabilities for this package set."),
            );
        } else {
            for (ix, vuln) in audit.iter().enumerate() {
                let package = vuln.package.clone();
                let severity = vuln.severity.clone();
                let title = vuln.title.clone();
                let fixed_in = vuln.fixed_in.clone();
                let is_selected = selected == Some(ix);
                let severity_color = match severity.as_str() {
                    "critical" | "high" => theme.danger,
                    "moderate" => theme.warning,
                    _ => theme.muted_foreground,
                };
                let mut row = h_flex()
                    .id(("npm-vuln-row", ix))
                    .gap_2()
                    .items_center()
                    .w_full()
                    .rounded_md()
                    .when(is_selected, |row| {
                        row.bg(theme.primary.opacity(0.08)).border_1().border_color(theme.primary)
                    })
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.selected = Some(ix);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .px_1p5()
                            .py_0p5()
                            .rounded_md()
                            .bg(severity_color.opacity(0.2))
                            .text_color(severity_color)
                            .text_xs()
                            .child(severity.clone()),
                    )
                    .child(
                        div()
                            .id(format!("npm-vuln-{package}"))
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .cursor_pointer()
                            .text_xs()
                            .text_color(theme.foreground)
                            .child(package.clone())
                            .on_click({
                                let view = view.clone();
                                let package = package.clone();
                                move |_, _window, cx| {
                                    view.update(cx, |panel, cx| {
                                        panel.fetch_details(package.clone(), cx);
                                    });
                                }
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .text_xs()
                            .text_color(theme.muted_foreground)
                            .child(title.clone()),
                    );
                if let Some(version) = &fixed_in {
                    row = row.child(
                        Button::new(format!("npm-fix-{package}"))
                            .primary()
                            .label(format!("Fix → {version}"))
                            .with_size(Size::Small)
                            .on_click({
                                let view = view.clone();
                                let package = package.clone();
                                let version = version.clone();
                                move |_, window, cx| {
                                    view.update(cx, |panel, cx| {
                                        panel.update_pkg(&package, &version, window, cx);
                                    });
                                }
                            }),
                    );
                }
                list = list.child(row);
            }
        }
        list.into_any_element()
    }
}

fn build_vuln_page(project: &crate::NpmProject, pages: &PageViews) -> SettingPage {
    let count = project.audit.len();
    let loading = project.audit_loading;
    SettingPage::new("Vulnerabilities")
        .resettable(false)
        .icon(Icon::new(IconName::TriangleAlert))
        .description("Findings from the latest `npm audit` run.")
        .group(
            SettingGroup::new()
                .title("npm audit findings")
                .item(embed_view(pages.vuln.clone())),
        )
        .sidebar_badge(move |_, cx| colored_count_badge(count, loading, cx.theme().danger))
}

// ── Shared ─────────────────────────────────────────────────────────────────

fn count_badge(count: usize, loading: bool, cx: &App) -> AnyElement {
    colored_count_badge(count, loading, cx.theme().primary)
}

/// Like [`count_badge`], but with the pill's fill color set by the caller —
/// used for the Vulnerabilities badge, which needs the error color rather
/// than the primary-theme color the Installed/Updates badges use.
fn colored_count_badge(count: usize, loading: bool, color: Hsla) -> AnyElement {
    h_flex()
        .child(
            div()
                .when(count == 0 && !loading, |this| this.size_0())
                .when(count > 0, |this| {
                    this.flex()
                        .bg(color)
                        .rounded_full()
                        .px_1p5()
                        .min_w_3p5()
                        .h_4()
                        .items_center()
                        .justify_center()
                        .text_xs()
                        .text_color(badge_text(color))
                        .child(count.to_string())
                })
                .when(count == 0 && loading, |this| {
                    this.child(Spinner::new().with_size(Size::XSmall))
                }),
        )
        .into_any_element()
}

/// The text colour for a count badge filled with `fill`: black on a light
/// fill, white on a dark one.
///
/// The badge used to be white on whatever the theme's accent was, which is
/// unreadable in themes with a light accent (white on lilac, say). Choosing
/// by the fill's brightness keeps the number legible in any theme without
/// relying on the theme to define a matching foreground.
fn badge_text(fill: Hsla) -> Hsla {
    let rgb = fill.to_rgb();
    // How bright the colour looks, green counting most and blue least.
    let brightness = 0.2126 * rgb.r + 0.7152 * rgb.g + 0.0722 * rgb.b;
    if brightness > 0.55 {
        gpui::black()
    } else {
        gpui::white()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_selected_starts_at_the_nearest_end() {
        assert_eq!(step_selected(None, 3, true), Some(0));
        assert_eq!(step_selected(None, 3, false), Some(2));
    }

    #[test]
    fn step_selected_wraps_at_both_ends() {
        assert_eq!(step_selected(Some(0), 3, true), Some(1));
        assert_eq!(step_selected(Some(2), 3, true), Some(0));
        assert_eq!(step_selected(Some(0), 3, false), Some(2));
        assert_eq!(step_selected(Some(1), 3, false), Some(0));
        assert_eq!(step_selected(Some(0), 1, true), Some(0));
        assert_eq!(step_selected(Some(0), 1, false), Some(0));
    }

    #[test]
    fn step_selected_handles_empty_and_shrunken_lists() {
        assert_eq!(step_selected(None, 0, true), None);
        assert_eq!(step_selected(Some(4), 0, false), None);
        // A selection left over from a longer list still lands in range.
        assert!(step_selected(Some(9), 3, true).is_some_and(|ix| ix < 3));
        assert!(step_selected(Some(9), 3, false).is_some_and(|ix| ix < 3));
    }

    #[test]
    fn badge_text_contrasts_with_its_fill() {
        // Light fills take dark text: a lilac accent, a yellow warning.
        assert_eq!(badge_text(gpui::rgb(0xc4b5fd).into()), gpui::black());
        assert_eq!(badge_text(gpui::rgb(0xeab308).into()), gpui::black());
        assert_eq!(badge_text(gpui::white()), gpui::black());
        // Dark fills take light text: a deep blue accent, a red danger.
        assert_eq!(badge_text(gpui::rgb(0x1d4ed8).into()), gpui::white());
        assert_eq!(badge_text(gpui::rgb(0xb91c1c).into()), gpui::white());
        assert_eq!(badge_text(gpui::black()), gpui::white());
    }
}
