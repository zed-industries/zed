use crate::kernels::KernelSpecification;
use crate::repl_store::ReplStore;
use crate::KERNEL_DOCS_URL;

// - [@nate] Add a split button for running/selecting kernels
//   - In REPL editor mode show the REPL icon
//   - In notebook editor mode show a play icon
// - [@nate/@kyle] Display recently used kernels at the top of the menu, using namespaces for each category (`py:conda:agents-service`, `jupyter:deno`, `my-server:python3.9`)
// - [ ] Categorize kernel options for selection
//   - [ ] Local Kernel
//   - [ ] Jupyter Kernel
//   - [ ] Python Env
//   - [ ] Remote Kernel
// - [ ] Kernel selection via available Jupyter kernels in submenu
//   - [ ] Display kernels like `deno`, `python3`, `rust`
// - [ ] Kernel selection via available Python environments in submenu
//   - [ ] Group listings by environment type (Pyenv, Poetry, Conda)
//   - [ ] Display Python versions for each environment
//   - [ ] List project-specific environments
//   - [ ] Install ipykernel in the environment if necessary
//   - [ ] Generate on-demand IPython kernelspec when selecting a python environment
// - [ ] Remote kernel selection
//   - [ ] Show server names for remote kernels
//   - [ ] Each remote server has a submenu of kernels they offer
//   - [ ] Allow adding new remote servers
//     - [ ] Implement modal for new server input (name, URL, token)
//   - [ ] Consider storing user's remote servers in their Zed.dev account
//   - [ ] Treat kernel selection settings as the global default that gets overriden by project settings, a notebook, or selecting in the selector
// ---
use gpui::DismissEvent;

use gpui::EntityId;
use picker::Picker;
use picker::PickerDelegate;

use std::sync::Arc;
use ui::ListItemSpacing;

use gpui::SharedString;
use gpui::Task;
use ui::{prelude::*, ListItem, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

type OnSelect = Box<dyn Fn(KernelSpecification, &mut WindowContext)>;

#[derive(IntoElement)]
pub struct KernelSelector<T: PopoverTrigger> {
    handle: Option<PopoverMenuHandle<Picker<KernelPickerDelegate>>>,
    on_select: OnSelect,
    trigger: T,
    info_text: Option<SharedString>,
}

pub struct KernelPickerDelegate {
    all_kernels: Vec<KernelSpecification>,
    filtered_kernels: Vec<KernelSpecification>,
    selected_index: usize,
    on_select: OnSelect,
}

impl<T: PopoverTrigger> KernelSelector<T> {
    pub fn new(on_select: OnSelect, trigger: T) -> Self {
        KernelSelector {
            on_select,
            handle: None,
            trigger,
            info_text: None,
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
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix.min(self.filtered_kernels.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a kernel...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let all_kernels = self.all_kernels.clone();
        cx.spawn(|this, mut cx| async move {
            let filtered_kernels = cx
                .background_executor()
                .spawn(async move {
                    if query.is_empty() {
                        all_kernels
                    } else {
                        all_kernels
                            .into_iter()
                            .filter(|kernel| {
                                // TODO: there is probably an off the shelf way to do this fuzzy
                                kernel.name().to_lowercase().contains(&query.to_lowercase())
                            })
                            .collect()
                    }
                })
                .await;

            this.update(&mut cx, |this, cx| {
                this.delegate.filtered_kernels = filtered_kernels;
                this.delegate.set_selected_index(0, cx);
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(kernelspec) = self.filtered_kernels.get(self.selected_index) {
            (self.on_select)(kernelspec.clone(), cx.window_context());
            cx.emit(DismissEvent);
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let kernel = self.filtered_kernels.get(ix)?;
        let kernel_name = kernel.name();
        let is_selected = self.selected_index == ix;

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                // .start_slot(
                //     div().pr_0p5().child(
                //         Icon::new(kernel_info.icon)
                //             .color(Color::Muted)
                //             .size(IconSize::Medium),
                //     ),
                // )
                .child(
                    h_flex().w_full().justify_between().min_w(px(200.)).child(
                        h_flex()
                            .gap_1p5()
                            .child(Label::new(kernel_name)) // TODO: Replace with actual kernel name
                            .child(
                                Label::new(kernel.type_name()) // TODO: Replace with actual kernel type
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            ),
                    ),
                )
                .end_slot(div().when(is_selected, |this| {
                    this.child(
                        Icon::new(IconName::Check)
                            .color(Color::Accent)
                            .size(IconSize::Small),
                    )
                })),
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

impl<T: PopoverTrigger> RenderOnce for KernelSelector<T> {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        // TODO: Implement kernel selection logic
        let store = ReplStore::global(cx).read(cx);
        let all_kernels: Vec<KernelSpecification> =
            store.kernel_specifications().cloned().collect();

        let delegate = KernelPickerDelegate {
            on_select: self.on_select,
            all_kernels: all_kernels.clone(),
            filtered_kernels: all_kernels,
            selected_index: 0,
        };

        let picker_view = cx.new_view(|cx| {
            let picker = Picker::uniform_list(delegate, cx).max_height(Some(rems(20.).into()));
            picker
        });

        PopoverMenu::new("kernel-switcher")
            .menu(move |_cx| Some(picker_view.clone()))
            .trigger(self.trigger)
            .attach(gpui::AnchorCorner::BottomLeft)
            .when_some(self.handle, |menu, handle| menu.with_handle(handle))
    }
}
