use gpui::{
    actions,
    elements::{Component, Flex, ParentElement, SafeStylable},
    AppContext, Element, Entity, ModelHandle, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use project::Project;
use theme::components::{action_button::Button, label::Label, ComponentExt};
use workspace::{
    item::Item, register_deserializable_item, ItemId, Pane, PaneBackdrop, Workspace, WorkspaceId,
};

pub fn init(cx: &mut AppContext) {
    cx.add_action(ComponentTest::toggle_disclosure);
    cx.add_action(ComponentTest::toggle_toggle);
    cx.add_action(ComponentTest::deploy);
    register_deserializable_item::<ComponentTest>(cx);
}

actions!(
    test,
    [NoAction, ToggleDisclosure, ToggleToggle, NewComponentTest]
);

struct ComponentTest {
    disclosed: bool,
    toggled: bool,
}

impl ComponentTest {
    fn new() -> Self {
        Self {
            disclosed: false,
            toggled: false,
        }
    }

    fn deploy(workspace: &mut Workspace, _: &NewComponentTest, cx: &mut ViewContext<Workspace>) {
        workspace.add_item(Box::new(cx.add_view(|_| ComponentTest::new())), cx);
    }

    fn toggle_disclosure(&mut self, _: &ToggleDisclosure, cx: &mut ViewContext<Self>) {
        self.disclosed = !self.disclosed;
        cx.notify();
    }

    fn toggle_toggle(&mut self, _: &ToggleToggle, cx: &mut ViewContext<Self>) {
        self.toggled = !self.toggled;
        cx.notify();
    }
}

impl Entity for ComponentTest {
    type Event = ();
}

impl View for ComponentTest {
    fn ui_name() -> &'static str {
        "Component Test"
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> gpui::AnyElement<Self> {
        let theme = theme::current(cx);

        PaneBackdrop::new(
            cx.view_id(),
            Flex::column()
                .with_spacing(10.)
                .with_child(
                    Button::action(NoAction)
                        .with_tooltip("Here's what a tooltip looks like", theme.tooltip.clone())
                        .with_contents(Label::new("Click me!"))
                        .with_style(theme.component_test.button.clone())
                        .element(),
                )
                .with_child(
                    Button::action(ToggleToggle)
                        .with_tooltip("Here's what a tooltip looks like", theme.tooltip.clone())
                        .with_contents(Label::new("Toggle me!"))
                        .toggleable(self.toggled)
                        .with_style(theme.component_test.toggle.clone())
                        .element(),
                )
                .with_child(
                    Label::new("A disclosure")
                        .disclosable(Some(self.disclosed), Box::new(ToggleDisclosure))
                        .with_style(theme.component_test.disclosure.clone())
                        .element(),
                )
                .constrained()
                .with_width(200.)
                .aligned()
                .into_any(),
        )
        .into_any()
    }
}

impl Item for ComponentTest {
    fn tab_content<V: 'static>(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &AppContext,
    ) -> gpui::AnyElement<V> {
        gpui::elements::Label::new("Component test", style.label.clone()).into_any()
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some("ComponentTest")
    }

    fn deserialize(
        _project: ModelHandle<Project>,
        _workspace: WeakViewHandle<Workspace>,
        _workspace_id: WorkspaceId,
        _item_id: ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<anyhow::Result<ViewHandle<Self>>> {
        Task::ready(Ok(cx.add_view(|_| Self::new())))
    }
}
