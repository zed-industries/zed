use gpui::{prelude::*, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, WeakEntity};
use ui::prelude::*;
use workspace::{ModalView, Workspace};

use crate::assistant_configuration::profile_picker::{ProfilePicker, ProfilePickerDelegate};
use crate::ManageProfiles;

enum Mode {
    ChooseProfile(Entity<ProfilePicker>),
}

pub struct ManageProfilesModal {
    #[allow(dead_code)]
    workspace: WeakEntity<Workspace>,
    mode: Mode,
}

impl ManageProfilesModal {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &ManageProfiles, window, cx| {
            let workspace_handle = cx.entity().downgrade();
            workspace.toggle_modal(window, cx, |window, cx| {
                Self::new(workspace_handle, window, cx)
            })
        });
    }

    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            workspace,
            mode: Mode::ChooseProfile(cx.new(|cx| {
                let delegate = ProfilePickerDelegate::new(cx);
                ProfilePicker::new(delegate, window, cx)
            })),
        }
    }

    fn confirm(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn cancel(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}
}

impl ModalView for ManageProfilesModal {}

impl Focusable for ManageProfilesModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            Mode::ChooseProfile(profile_picker) => profile_picker.read(cx).focus_handle(cx),
        }
    }
}

impl EventEmitter<DismissEvent> for ManageProfilesModal {}

impl Render for ManageProfilesModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("ManageProfilesModal")
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| this.cancel(window, cx)))
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| this.confirm(window, cx)))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .on_mouse_down_out(cx.listener(|_this, _, _, cx| cx.emit(DismissEvent)))
            .child(match &self.mode {
                Mode::ChooseProfile(profile_picker) => profile_picker.clone().into_any_element(),
            })
    }
}
