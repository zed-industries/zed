use crate::signature_help_popover::SignatureHelpPopover;
use gpui::Task;

pub const SIGNATURE_HELP_DELAY: u64 = 600;

#[derive(Debug)]
pub struct SignatureHelpState {
    active: bool,
    show_signature_help_task: Option<Task<()>>,
    hide_signature_help_task: Option<Task<()>>,
    signature_help_popover: Option<SignatureHelpPopover>,
}

impl SignatureHelpState {
    pub fn new() -> Self {
        Self {
            active: false,
            show_signature_help_task: None,
            hide_signature_help_task: None,
            signature_help_popover: None,
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn set_show_signature_help_task(&mut self, task: Option<Task<()>>) {
        self.show_signature_help_task = task;
    }

    pub fn set_hide_signature_help_task(&mut self, task: Option<Task<()>>) {
        self.hide_signature_help_task = task;
    }

    pub fn signature_help_popover_mut(&mut self) -> Option<&mut SignatureHelpPopover> {
        self.signature_help_popover.as_mut()
    }

    pub fn set_signature_help_popover(&mut self, popover: Option<SignatureHelpPopover>) {
        self.signature_help_popover = popover;
    }
}

#[cfg(test)]
impl SignatureHelpState {
    pub fn signature_help_popover(&self) -> Option<&SignatureHelpPopover> {
        self.signature_help_popover.as_ref()
    }
}
