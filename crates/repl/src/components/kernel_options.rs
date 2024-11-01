use crate::kernels::KernelOption;
use crate::repl_store::ReplStore;

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
use gpui::Action;
use gpui::DismissEvent;

use picker::Picker;
use picker::PickerDelegate;
use workspace::ShowConfiguration;

use std::sync::Arc;
use ui::ListItemSpacing;

use gpui::SharedString;
use gpui::Task;
use ui::{prelude::*, ListItem, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

#[derive(IntoElement)]
pub struct KernelSelector<T: PopoverTrigger> {
    handle: Option<PopoverMenuHandle<Picker<KernelPickerDelegate>>>,
    trigger: T,
    info_text: Option<SharedString>,
}

pub struct KernelPickerDelegate {
    all_kernels: Vec<KernelOption>,
    filtered_kernels: Vec<KernelOption>,
    selected_index: usize,
}

impl<T: PopoverTrigger> KernelSelector<T> {
    pub fn new(trigger: T) -> Self {
        KernelSelector {
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
        if let Some(kernel) = self.filtered_kernels.get(self.selected_index) {
            let kernel = kernel.clone();
            println!("Selected kernel: {:?}", kernel);

            cx.emit(DismissEvent);
            todo!();
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
                                Label::new("Kernel Type") // TODO: Replace with actual kernel type
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
                .justify_between()
                .child(
                    Button::new("configure", "Configure")
                        .icon(IconName::Settings)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .icon_position(IconPosition::Start)
                        .on_click(|_, cx| {
                            cx.dispatch_action(ShowConfiguration.boxed_clone());
                        }),
                )
                .into_any(),
        )
    }
}

impl<T: PopoverTrigger> RenderOnce for KernelSelector<T> {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        // TODO: Implement kernel selection logic
        let mut all_kernels = Vec::new();
        let store = ReplStore::global(cx).read(cx);
        for kernel_spec in store.kernel_specifications() {
            all_kernels.push(KernelOption::Jupyter(kernel_spec.clone()));
        }

        let delegate = KernelPickerDelegate {
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
