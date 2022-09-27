use gpui::{elements::*, Entity, RenderContext, View};
use settings::Settings;

pub struct AddParticipantPopover {}

impl Entity for AddParticipantPopover {
    type Event = ();
}

impl View for AddParticipantPopover {
    fn ui_name() -> &'static str {
        "AddParticipantPopover"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx
            .global::<Settings>()
            .theme
            .workspace
            .titlebar
            .add_participant_popover;
        Empty::new()
            .contained()
            .with_style(theme.container)
            .constrained()
            .with_width(theme.width)
            .with_height(theme.height)
            .boxed()
    }
}

impl AddParticipantPopover {
    pub fn new() -> Self {
        Self {}
    }
}
