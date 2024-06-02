use gpui::{div, Element, Render, Subscription, ViewContext, WeakModel};
use workspace::{item::ItemHandle, ui::prelude::*, StatusItemView};

use crate::{editor_state::EditorState, EasyMotion, GlobalEasyMotion};

pub struct BufferDisplay {
    pub(crate) buffer: String,
    model: Option<WeakModel<EasyMotion>>,
    model_subscription: Option<Subscription>,
    _global_subscription: Subscription,
}

impl BufferDisplay {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let global_subscription =
            cx.observe_global::<GlobalEasyMotion>(|this, cx| this.update_model(cx));
        let mut this = Self {
            buffer: String::new(),
            model: None,
            model_subscription: None,
            _global_subscription: global_subscription,
        };
        this.update_model(cx);
        this
    }

    fn update_model(&mut self, cx: &mut ViewContext<Self>) {
        let Some(model) = EasyMotion::global(cx) else {
            self.model = None;
            self.model_subscription = None;
            return;
        };
        let subcription = cx.observe(&model, |this, easy, cx| {
            let easy = easy.read(cx);
            let state = easy.latest_state();
            let str = match state {
                EditorState::NCharInput(n_char) => n_char.chars().to_string(),
                EditorState::Selection(selection) => selection.selection().to_string(),
                _ => String::new(),
            };
            this.buffer = str;
        });
        self.model_subscription = Some(subcription);
        self.model = Some(model.downgrade());
    }
}

impl Render for BufferDisplay {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        if self.buffer.is_empty() {
            return div().into_any();
        }

        Label::new(self.buffer.clone())
            .size(LabelSize::Small)
            .line_height_style(LineHeightStyle::UiLabel)
            .into_any_element()
    }
}

impl StatusItemView for BufferDisplay {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _cx: &mut ViewContext<Self>,
    ) {
        // nothing to do.
    }
}
