use crate::KERNEL_DOCS_URL;
use crate::kernels::KernelSpecification;
use crate::repl_store::ReplStore;

use gpui::AnyView;
use gpui::DismissEvent;

use gpui::FontWeight;
use picker::Picker;
use picker::PickerDelegate;
use project::WorktreeId;

use std::sync::Arc;
use ui::ListItemSpacing;

use gpui::SharedString;
use gpui::Task;
use ui::{ListItem, PopoverMenu, PopoverMenuHandle, PopoverTrigger, prelude::*};

type OnSelect = Box<dyn Fn(KernelSpecification, &mut Window, &mut App)>;

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
    all_kernels: Vec<KernelSpecification>,
    filtered_kernels: Vec<KernelSpecification>,
    selected_kernelspec: Option<KernelSpecification>,
    on_select: OnSelect,
}

// Helper function to truncate long paths
fn truncate_path(path: &SharedString, max_length: usize) -> SharedString {
    if path.len() <= max_length {
        path.to_string().into()
    } else {
        let truncated = path.chars().rev().take(max_length - 3).collect::<String>();
        format!("...{}", truncated.chars().rev().collect::<String>()).into()
    }
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

impl PickerDelegate for KernelPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.filtered_kernels.len()
    }

    fn selected_index(&self) -> usize {
        if let Some(kernelspec) = self.selected_kernelspec.as_ref() {
            self.filtered_kernels
                .iter()
                .position(|k| k == kernelspec)
                .unwrap_or(0)
        } else {
            0
        }
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_kernelspec = self.filtered_kernels.get(ix).cloned();
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
        let all_kernels = self.all_kernels.clone();

        if query.is_empty() {
            self.filtered_kernels = all_kernels;
            return Task::ready(());
        }

        self.filtered_kernels = if query.is_empty() {
            all_kernels
        } else {
            all_kernels
                .into_iter()
                .filter(|kernel| kernel.name().to_lowercase().contains(&query.to_lowercase()))
                .collect()
        };

        return Task::ready(());
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(kernelspec) = &self.selected_kernelspec {
            (self.on_select)(kernelspec.clone(), window, cx);
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
        let kernelspec = self.filtered_kernels.get(ix)?;
        let is_selected = self.selected_kernelspec.as_ref() == Some(kernelspec);
        let icon = kernelspec.icon(cx);

        let (name, kernel_type, path_or_url) = match kernelspec {
            KernelSpecification::Jupyter(_) => (kernelspec.name(), "Jupyter", None),
            KernelSpecification::PythonEnv(_) => (
                kernelspec.name(),
                "Python Env",
                Some(truncate_path(&kernelspec.path(), 42)),
            ),
            KernelSpecification::Remote(_) => (
                kernelspec.name(),
                "Remote",
                Some(truncate_path(&kernelspec.path(), 42)),
            ),
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
                        .child(icon.color(Color::Default).size(IconSize::Medium))
                        .child(
                            v_flex()
                                .flex_grow()
                                .gap_0p5()
                                .child(
                                    h_flex()
                                        .justify_between()
                                        .child(
                                            div().w_48().text_ellipsis().child(
                                                Label::new(name)
                                                    .weight(FontWeight::MEDIUM)
                                                    .size(LabelSize::Default),
                                            ),
                                        )
                                        .when_some(path_or_url.clone(), |flex, path| {
                                            flex.text_ellipsis().child(
                                                Label::new(path)
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted),
                                            )
                                        }),
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            Label::new(kernelspec.language())
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            Label::new(kernel_type)
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        ),
                                ),
                        ),
                )
                .when(is_selected, |item| {
                    item.end_slot(
                        Icon::new(IconName::Check)
                            .color(Color::Accent)
                            .size(IconSize::Small),
                    )
                }),
        )
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
                        .icon(IconName::ExternalLink)
                        .icon_size(IconSize::XSmall)
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

        let all_kernels: Vec<KernelSpecification> = store
            .kernel_specifications_for_worktree(self.worktree_id)
            .cloned()
            .collect();

        let selected_kernelspec = store.active_kernelspec(self.worktree_id, None, cx);

        let delegate = KernelPickerDelegate {
            on_select: self.on_select,
            all_kernels: all_kernels.clone(),
            filtered_kernels: all_kernels,
            selected_kernelspec,
        };

        let picker_view = cx.new(|cx| {
            let picker = Picker::uniform_list(delegate, window, cx)
                .width(rems(30.))
                .max_height(Some(rems(20.).into()));
            picker
        });

        PopoverMenu::new("kernel-switcher")
            .menu(move |_window, _cx| Some(picker_view.clone()))
            .trigger_with_tooltip(self.trigger, self.tooltip)
            .attach(gpui::Corner::BottomLeft)
            .when_some(self.handle, |menu, handle| menu.with_handle(handle))
    }
}
