use crate::KERNEL_DOCS_URL;
use crate::kernels::KernelSpecification;
use crate::repl_store::ReplStore;

use gpui::{AnyView, DismissEvent, FontWeight, SharedString, Task};
use picker::{Picker, PickerDelegate};
use project::WorktreeId;
use std::sync::Arc;
use ui::{ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, PopoverTrigger, prelude::*};

type OnSelect = Box<dyn Fn(KernelSpecification, &mut Window, &mut App)>;

#[derive(Clone)]
pub enum KernelPickerEntry {
    SectionHeader(SharedString),
    Kernel {
        spec: KernelSpecification,
        is_recommended: bool,
    },
}

fn build_grouped_entries(store: &ReplStore, worktree_id: WorktreeId) -> Vec<KernelPickerEntry> {
    let mut entries = Vec::new();
    let mut recommended_entry: Option<KernelPickerEntry> = None;

    let mut python_envs = Vec::new();
    let mut jupyter_kernels = Vec::new();
    let mut remote_kernels = Vec::new();

    for spec in store.kernel_specifications_for_worktree(worktree_id) {
        let is_recommended = store.is_recommended_kernel(worktree_id, spec);

        if is_recommended {
            recommended_entry = Some(KernelPickerEntry::Kernel {
                spec: spec.clone(),
                is_recommended: true,
            });
        }

        match spec {
            KernelSpecification::PythonEnv(_) => {
                python_envs.push(KernelPickerEntry::Kernel {
                    spec: spec.clone(),
                    is_recommended,
                });
            }
            KernelSpecification::Jupyter(_) => {
                jupyter_kernels.push(KernelPickerEntry::Kernel {
                    spec: spec.clone(),
                    is_recommended,
                });
            }
            KernelSpecification::Remote(_) => {
                remote_kernels.push(KernelPickerEntry::Kernel {
                    spec: spec.clone(),
                    is_recommended,
                });
            }
        }
    }

    // Sort Python envs: has_ipykernel first, then by name
    python_envs.sort_by(|a, b| {
        let (spec_a, spec_b) = match (a, b) {
            (
                KernelPickerEntry::Kernel { spec: sa, .. },
                KernelPickerEntry::Kernel { spec: sb, .. },
            ) => (sa, sb),
            _ => return std::cmp::Ordering::Equal,
        };
        spec_b
            .has_ipykernel()
            .cmp(&spec_a.has_ipykernel())
            .then_with(|| spec_a.name().cmp(&spec_b.name()))
    });

    // Recommended section
    if let Some(rec) = recommended_entry {
        entries.push(KernelPickerEntry::SectionHeader("Recommended".into()));
        entries.push(rec);
    }

    // Python Environments section
    if !python_envs.is_empty() {
        entries.push(KernelPickerEntry::SectionHeader(
            "Python Environments".into(),
        ));
        entries.extend(python_envs);
    }

    // Jupyter Kernels section
    if !jupyter_kernels.is_empty() {
        entries.push(KernelPickerEntry::SectionHeader("Jupyter Kernels".into()));
        entries.extend(jupyter_kernels);
    }

    // Remote section
    if !remote_kernels.is_empty() {
        entries.push(KernelPickerEntry::SectionHeader("Remote Servers".into()));
        entries.extend(remote_kernels);
    }

    entries
}

#[derive(IntoElement)]
pub struct KernelSelector<T, TT>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    handle: Option<PopoverMenuHandle<Picker<KernelPickerDelegate>>>,
    on_select: OnSelect,
    trigger: T,
    tooltip: TT,
    info_text: Option<SharedString>,
    worktree_id: WorktreeId,
}

pub struct KernelPickerDelegate {
    all_entries: Vec<KernelPickerEntry>,
    filtered_entries: Vec<KernelPickerEntry>,
    selected_kernelspec: Option<KernelSpecification>,
    selected_index: usize,
    on_select: OnSelect,
}

impl<T, TT> KernelSelector<T, TT>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    pub fn new(on_select: OnSelect, worktree_id: WorktreeId, trigger: T, tooltip: TT) -> Self {
        KernelSelector {
            on_select,
            handle: None,
            trigger,
            tooltip,
            info_text: None,
            worktree_id,
        }
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<Picker<KernelPickerDelegate>>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn with_info_text(mut self, text: impl Into<SharedString>) -> Self {
        self.info_text = Some(text.into());
        self
    }
}

impl KernelPickerDelegate {
    fn first_selectable_index(entries: &[KernelPickerEntry]) -> usize {
        entries
            .iter()
            .position(|e| matches!(e, KernelPickerEntry::Kernel { .. }))
            .unwrap_or(0)
    }

    fn next_selectable_index(&self, from: usize, direction: i32) -> usize {
        let len = self.filtered_entries.len();
        if len == 0 {
            return 0;
        }

        let mut index = from as i32 + direction;
        while index >= 0 && (index as usize) < len {
            if matches!(
                self.filtered_entries.get(index as usize),
                Some(KernelPickerEntry::Kernel { .. })
            ) {
                return index as usize;
            }
            index += direction;
        }

        from
    }
}

impl PickerDelegate for KernelPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.filtered_entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        if matches!(
            self.filtered_entries.get(ix),
            Some(KernelPickerEntry::SectionHeader(_))
        ) {
            let forward = self.next_selectable_index(ix, 1);
            if forward != ix {
                self.selected_index = forward;
            } else {
                self.selected_index = self.next_selectable_index(ix, -1);
            }
        } else {
            self.selected_index = ix;
        }

        if let Some(KernelPickerEntry::Kernel { spec, .. }) =
            self.filtered_entries.get(self.selected_index)
        {
            self.selected_kernelspec = Some(spec.clone());
        }
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a kernel...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query.is_empty() {
            self.filtered_entries = self.all_entries.clone();
        } else {
            let query_lower = query.to_lowercase();
            let mut filtered = Vec::new();
            let mut pending_header: Option<KernelPickerEntry> = None;

            for entry in &self.all_entries {
                match entry {
                    KernelPickerEntry::SectionHeader(_) => {
                        pending_header = Some(entry.clone());
                    }
                    KernelPickerEntry::Kernel { spec, .. } => {
                        if spec.name().to_lowercase().contains(&query_lower) {
                            if let Some(header) = pending_header.take() {
                                filtered.push(header);
                            }
                            filtered.push(entry.clone());
                        }
                    }
                }
            }

            self.filtered_entries = filtered;
        }

        self.selected_index = Self::first_selectable_index(&self.filtered_entries);
        if let Some(KernelPickerEntry::Kernel { spec, .. }) =
            self.filtered_entries.get(self.selected_index)
        {
            self.selected_kernelspec = Some(spec.clone());
        }

        Task::ready(())
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        let mut separators = Vec::new();
        for (index, entry) in self.filtered_entries.iter().enumerate() {
            if matches!(entry, KernelPickerEntry::SectionHeader(_)) && index > 0 {
                separators.push(index - 1);
            }
        }
        separators
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(KernelPickerEntry::Kernel { spec, .. }) =
            self.filtered_entries.get(self.selected_index)
        {
            (self.on_select)(spec.clone(), window, cx);
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = self.filtered_entries.get(ix)?;

        match entry {
            KernelPickerEntry::SectionHeader(title) => Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Dense)
                    .selectable(false)
                    .child(
                        Label::new(title.clone())
                            .size(LabelSize::Small)
                            .weight(FontWeight::SEMIBOLD)
                            .color(Color::Muted),
                    ),
            ),
            KernelPickerEntry::Kernel {
                spec,
                is_recommended,
            } => {
                let is_currently_selected = self.selected_kernelspec.as_ref() == Some(spec);
                let icon = spec.icon(cx);
                let has_ipykernel = spec.has_ipykernel();

                let subtitle = match spec {
                    KernelSpecification::Jupyter(_) => None,
                    KernelSpecification::PythonEnv(_) | KernelSpecification::Remote(_) => {
                        let env_kind = spec.environment_kind_label();
                        let path = spec.path();
                        match env_kind {
                            Some(kind) => Some(format!("{} \u{2013} {}", kind, path)),
                            None => Some(path.to_string()),
                        }
                    }
                };

                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(
                            h_flex()
                                .w_full()
                                .gap_3()
                                .when(!has_ipykernel, |flex| flex.opacity(0.5))
                                .child(icon.color(Color::Default).size(IconSize::Medium))
                                .child(
                                    v_flex()
                                        .flex_grow()
                                        .overflow_x_hidden()
                                        .gap_0p5()
                                        .child(
                                            h_flex()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .overflow_x_hidden()
                                                        .flex_shrink()
                                                        .text_ellipsis()
                                                        .child(
                                                            Label::new(spec.name())
                                                                .weight(FontWeight::MEDIUM)
                                                                .size(LabelSize::Default),
                                                        ),
                                                )
                                                .when(*is_recommended, |flex| {
                                                    flex.child(
                                                        Label::new("Recommended")
                                                            .size(LabelSize::XSmall)
                                                            .color(Color::Accent),
                                                    )
                                                })
                                                .when(!has_ipykernel, |flex| {
                                                    flex.child(
                                                        Label::new("ipykernel not installed")
                                                            .size(LabelSize::XSmall)
                                                            .color(Color::Warning),
                                                    )
                                                }),
                                        )
                                        .when_some(subtitle, |flex, subtitle| {
                                            flex.child(
                                                div().overflow_x_hidden().text_ellipsis().child(
                                                    Label::new(subtitle)
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                ),
                                            )
                                        }),
                                ),
                        )
                        .when(is_currently_selected, |item| {
                            item.end_slot(
                                Icon::new(IconName::Check)
                                    .color(Color::Accent)
                                    .size(IconSize::Small),
                            )
                        }),
                )
            }
        }
    }

    fn render_footer(
        &self,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<gpui::AnyElement> {
        Some(
            h_flex()
                .w_full()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .p_1()
                .gap_4()
                .child(
                    Button::new("kernel-docs", "Kernel Docs")
                        .icon(IconName::ArrowUpRight)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .icon_position(IconPosition::End)
                        .on_click(move |_, _, cx| cx.open_url(KERNEL_DOCS_URL)),
                )
                .into_any(),
        )
    }
}

impl<T, TT> RenderOnce for KernelSelector<T, TT>
where
    T: PopoverTrigger + ButtonCommon,
    TT: Fn(&mut Window, &mut App) -> AnyView + 'static,
{
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let store = ReplStore::global(cx).read(cx);

        let all_entries = build_grouped_entries(store, self.worktree_id);
        let selected_kernelspec = store.active_kernelspec(self.worktree_id, None, cx);
        let selected_index = all_entries
            .iter()
            .position(|entry| {
                if let KernelPickerEntry::Kernel { spec, .. } = entry {
                    selected_kernelspec.as_ref() == Some(spec)
                } else {
                    false
                }
            })
            .unwrap_or_else(|| KernelPickerDelegate::first_selectable_index(&all_entries));

        let delegate = KernelPickerDelegate {
            on_select: self.on_select,
            all_entries: all_entries.clone(),
            filtered_entries: all_entries,
            selected_kernelspec,
            selected_index,
        };

        let picker_view = cx.new(|cx| {
            Picker::list(delegate, window, cx)
                .list_measure_all()
                .width(rems(34.))
                .max_height(Some(rems(24.).into()))
        });

        PopoverMenu::new("kernel-switcher")
            .menu(move |_window, _cx| Some(picker_view.clone()))
            .trigger_with_tooltip(self.trigger, self.tooltip)
            .attach(gpui::Corner::BottomLeft)
            .when_some(self.handle, |menu, handle| menu.with_handle(handle))
    }
}
