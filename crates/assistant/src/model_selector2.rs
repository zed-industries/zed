use std::sync::Arc;
use ui::ListItemSpacing;

use crate::assistant_settings::AssistantSettings;
use fs::Fs;
use gpui::SharedString;
use gpui::{AsyncWindowContext, Task};
use language_model::{LanguageModel, LanguageModelRegistry};
use picker::{Picker, PickerDelegate};
use settings::update_settings_file;
use ui::{prelude::*, ListItem, PopoverMenu, PopoverMenuHandle, PopoverTrigger};

#[derive(IntoElement)]
pub struct ModelSelector2<T: PopoverTrigger> {
    handle: Option<PopoverMenuHandle<Picker<ModelPickerDelegate>>>,
    fs: Arc<dyn Fs>,
    trigger: T,
    info_text: Option<SharedString>,
}

struct ModelPickerDelegate {
    fs: Arc<dyn Fs>,
    models: Vec<Arc<dyn LanguageModel>>,
    selected_index: usize,
    info_text: Option<SharedString>,
}

impl<T: PopoverTrigger> ModelSelector2<T> {
    pub fn new(fs: Arc<dyn Fs>, trigger: T) -> Self {
        ModelSelector2 {
            handle: None,
            fs,
            trigger,
            info_text: None,
        }
    }

    pub fn with_handle(mut self, handle: PopoverMenuHandle<Picker<ModelPickerDelegate>>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn with_info_text(mut self, text: impl Into<SharedString>) -> Self {
        self.info_text = Some(text.into());
        self
    }
}

impl PickerDelegate for ModelPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.models.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a model...".into()
    }

    fn update_matches(&mut self, _query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        cx.spawn(|_, _| async {})
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(model) = self.models.get(self.selected_index).cloned() {
            update_settings_file::<AssistantSettings>(self.fs.clone(), cx, move |settings, _| {
                settings.set_model(model)
            });
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let model = self.models.get(ix)?;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(Label::new(model.name().0.clone())),
        )
    }
}

impl<T: PopoverTrigger> RenderOnce for ModelSelector2<T> {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let models = LanguageModelRegistry::global(cx)
            .read(cx)
            .providers()
            .iter()
            .flat_map(|provider| provider.provided_models(cx))
            .collect::<Vec<_>>();

        let delegate = ModelPickerDelegate {
            fs: self.fs.clone(),
            models,
            selected_index: 0,
            info_text: self.info_text,
        };

        let picker_view = cx.new_view(|cx| {
            let mut picker = Picker::uniform_list(delegate, cx).max_height(Some(rems(20.).into()));
            // if let Some(handle) = self.handle {
            //     picker = picker.with_handle(handle);
            // }
            picker
        });

        PopoverMenu::new("model-switcher")
            .menu(move |_cx| Some(picker_view.clone()))
            .trigger(self.trigger)
            .attach(gpui::AnchorCorner::BottomLeft)
    }
}
