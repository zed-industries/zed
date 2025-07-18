use db::kvp::KEY_VALUE_STORE;
use feature_flags::FeatureFlag;
use gpui::{
    App, AppContext, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, Render,
    SharedString, Subscription, Task, WeakEntity, Window,
};
use settings::SettingsStore;
use std::sync::Arc;
use ui::{ParentElement as _, div};
use workspace::{
    AppState, Workspace, WorkspaceId,
    dock::DockPosition,
    item::{Item, ItemEvent},
    open_new,
};

pub struct OnBoardingFeatureFlag {}

impl FeatureFlag for OnBoardingFeatureFlag {
    const NAME: &'static str = "onboarding";
}

pub const FIRST_OPEN: &str = "first_open";

pub fn show_onboarding_view(app_state: Arc<AppState>, cx: &mut App) -> Task<anyhow::Result<()>> {
    open_new(
        Default::default(),
        app_state,
        cx,
        |workspace, window, cx| {
            workspace.toggle_dock(DockPosition::Left, window, cx);
            let onboarding_page = Onboarding::new(workspace.weak_handle(), cx);
            workspace.add_item_to_center(Box::new(onboarding_page.clone()), window, cx);

            window.focus(&onboarding_page.focus_handle(cx));

            cx.notify();

            db::write_and_log(cx, || {
                KEY_VALUE_STORE.write_kvp(FIRST_OPEN.to_string(), "false".to_string())
            });
        },
    )
}

struct Onboarding {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    _settings_subscription: Subscription,
}

impl Onboarding {
    fn new(workspace: WeakEntity<Workspace>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            workspace,
            focus_handle: cx.focus_handle(),
            _settings_subscription: cx.observe_global::<SettingsStore>(move |_, cx| cx.notify()),
        })
    }
}

impl EventEmitter<ItemEvent> for Onboarding {}

impl Focusable for Onboarding {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for Onboarding {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Onboarding".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Onboarding Page Opened")
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>> {
        Some(Onboarding::new(self.workspace.clone(), cx))
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        f(*event)
    }
}

impl Render for Onboarding {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Onboarding Page")
    }
}
