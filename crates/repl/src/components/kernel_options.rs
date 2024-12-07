use crate::kernels::KernelSpecification;
use crate::repl_store::ReplStore;
use crate::worktree_id_for_editor;
use crate::KERNEL_DOCS_URL;

use editor::Editor;
use gpui::DismissEvent;

use gpui::FontWeight;
use gpui::WeakView;
use picker::Picker;
use picker::PickerDelegate;
use project::WorktreeId;
use ui::ButtonLike;
use ui::Tooltip;

use std::sync::Arc;
use ui::ListItemSpacing;

use gpui::SharedString;
use gpui::Task;
use ui::{prelude::*, ListItem, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

pub type OnSelect = Box<dyn Fn(KernelSpecification, &mut WindowContext)>;

pub struct KernelSelector {
    handle: Option<PopoverMenuHandle<Picker<KernelPickerDelegate>>>,
    editor: WeakView<Editor>,
    info_text: Option<SharedString>,
    worktree_id: WorktreeId,
}

pub struct KernelPickerDelegate {
    all_kernels: Vec<KernelSpecification>,
    filtered_kernels: Vec<KernelSpecification>,
    selected_kernelspec: Option<KernelSpecification>,
    on_select: OnSelect,
    group: Group,
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

impl KernelSelector {
    pub fn new(editor: WeakView<Editor>, cx: &mut ViewContext<Self>) -> Self {
        // todo!()
        let worktree_id = worktree_id_for_editor(editor.clone(), cx).unwrap();

        KernelSelector {
            editor,
            handle: None,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Group {
    All,
    Jupyter,
    Python,
    Remote,
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

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_kernelspec = self.filtered_kernels.get(ix).cloned();
        cx.notify();
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a kernel...".into()
    }

    fn update_matches(&mut self, query: String, _cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let all_kernels = self.all_kernels.clone();

        if query.is_empty() {
            self.filtered_kernels = all_kernels;
            return Task::Ready(Some(()));
        }

        self.filtered_kernels = if query.is_empty() {
            all_kernels
        } else {
            all_kernels
                .into_iter()
                .filter(|kernel| kernel.name().to_lowercase().contains(&query.to_lowercase()))
                .collect()
        };

        return Task::Ready(Some(()));
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(kernelspec) = &self.selected_kernelspec {
            (self.on_select)(kernelspec.clone(), cx.window_context());
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
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
                .selected(selected)
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

    fn render_header(&self, cx: &mut ViewContext<Picker<Self>>) -> Option<gpui::AnyElement> {
        let mode = Group::All;

        Some(
            h_flex()
                .child(
                    div()
                        .id("all")
                        .px_2()
                        .py_1()
                        .cursor_pointer()
                        .border_b_2()
                        .when(mode == Group::All, |this| {
                            this.border_color(cx.theme().colors().border)
                        })
                        .child(Label::new("All"))
                        .on_click(cx.listener(|this, _, cx| {
                            this.delegate.set_group(Group::All, cx);
                        })),
                )
                .child(
                    div()
                        .id("jupyter")
                        .px_2()
                        .py_1()
                        .cursor_pointer()
                        .border_b_2()
                        .when(mode == Group::Jupyter, |this| {
                            this.border_color(cx.theme().colors().border)
                        })
                        .child(Label::new("Jupyter"))
                        .on_click(cx.listener(|this, _, cx| {
                            this.delegate.set_group(Group::Jupyter, cx);
                        })),
                )
                .child(
                    div()
                        .id("python")
                        .px_2()
                        .py_1()
                        .cursor_pointer()
                        .border_b_2()
                        .when(mode == Group::Python, |this| {
                            this.border_color(cx.theme().colors().border)
                        })
                        .child(Label::new("Python"))
                        .on_click(cx.listener(|this, _, cx| {
                            this.delegate.set_group(Group::Python, cx);
                        })),
                )
                .child(
                    div()
                        .id("remote")
                        .px_2()
                        .py_1()
                        .cursor_pointer()
                        .border_b_2()
                        .when(mode == Group::Remote, |this| {
                            this.border_color(cx.theme().colors().border)
                        })
                        .child(Label::new("Remote"))
                        .on_click(cx.listener(|this, _, cx| {
                            this.delegate.set_group(Group::Remote, cx);
                        })),
                )
                .into_any_element(),
        )
    }

    fn render_footer(&self, cx: &mut ViewContext<Picker<Self>>) -> Option<gpui::AnyElement> {
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
                        .on_click(move |_, cx| cx.open_url(KERNEL_DOCS_URL)),
                )
                .into_any(),
        )
    }
}

impl KernelPickerDelegate {
    fn new(
        on_select: OnSelect,
        kernels: Vec<KernelSpecification>,
        selected_kernelspec: Option<KernelSpecification>,
    ) -> Self {
        Self {
            on_select,
            all_kernels: kernels.clone(),
            filtered_kernels: kernels,
            group: Group::All,
            selected_kernelspec,
        }
    }

    fn set_group(&mut self, group: Group, cx: &mut ViewContext<Picker<Self>>) {
        dbg!(&group);
        self.group = group;
        cx.notify();
    }
}

impl Render for KernelSelector {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let store = ReplStore::global(cx).read(cx);

        let all_kernels: Vec<KernelSpecification> = store
            .kernel_specifications_for_worktree(self.worktree_id)
            .cloned()
            .collect();

        let selected_kernelspec = store.active_kernelspec(self.worktree_id, None, cx);
        let current_kernel_name = selected_kernelspec.as_ref().map(|spec| spec.name()).clone();

        let editor = self.editor.clone();
        let on_select: OnSelect = Box::new(move |kernelspec, cx| {
            crate::assign_kernelspec(kernelspec, editor.clone(), cx).ok();
        });

        let menu_handle: PopoverMenuHandle<Picker<KernelPickerDelegate>> =
            PopoverMenuHandle::default();

        let delegate =
            KernelPickerDelegate::new(on_select, all_kernels, selected_kernelspec.clone());

        let picker_view = cx.new_view(|cx| {
            let picker = Picker::uniform_list(delegate, cx)
                .width(rems(30.))
                .max_height(Some(rems(20.).into()));
            picker
        });

        PopoverMenu::new("kernel-switcher")
            .menu(move |_cx| Some(picker_view.clone()))
            .trigger(
                ButtonLike::new("kernel-selector")
                    .style(ButtonStyle::Subtle)
                    .child(
                        h_flex()
                            .w_full()
                            .gap_0p5()
                            .child(
                                div()
                                    .overflow_x_hidden()
                                    .flex_grow()
                                    .whitespace_nowrap()
                                    .child(
                                        Label::new(if let Some(name) = current_kernel_name {
                                            name
                                        } else {
                                            SharedString::from("Select Kernel")
                                        })
                                        .size(LabelSize::Small)
                                        .color(if selected_kernelspec.is_some() {
                                            Color::Default
                                        } else {
                                            Color::Placeholder
                                        })
                                        .into_any_element(),
                                    ),
                            )
                            .child(
                                Icon::new(IconName::ChevronDown)
                                    .color(Color::Muted)
                                    .size(IconSize::XSmall),
                            ),
                    )
                    .tooltip(move |cx| Tooltip::text("Select Kernel", cx)),
            )
            .attach(gpui::AnchorCorner::BottomLeft)
            .with_handle(menu_handle)
    }
}
