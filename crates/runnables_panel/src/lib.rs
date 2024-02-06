mod runnables_settings;

use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    actions, div, list, px, red, relative, rems, AppContext, EventEmitter, FocusHandle,
    FocusableView, FontStyle, FontWeight, IntoElement, ListAlignment, ListState,
    ParentElement as _, Render, Styled as _, TextStyle, View, ViewContext, VisualContext as _,
    WhiteSpace, WindowContext,
};
use settings::Settings as _;
use theme::ThemeSettings;
use ui::{h_flex, v_flex, ActiveTheme, List, StyledExt};
use workspace::{
    dock::{Panel, PanelEvent},
    Workspace,
};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<RunnablesPanel>(cx);
            });
        },
    )
    .detach();
}

pub struct RunnablesPanel {
    filter_editor: View<Editor>,
    focus_handle: FocusHandle,
}

impl RunnablesPanel {
    pub fn new(cx: &mut WindowContext<'_>) -> View<Self> {
        cx.new_view(|cx| {
            let filter_editor = cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("Filter...", cx);
                editor
            });

            Self {
                focus_handle: cx.focus_handle(),
                filter_editor,
            }
        })
    }
    fn render_filter_input(
        &self,
        editor: &View<Editor>,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: if editor.read(cx).read_only(cx) {
                cx.theme().colors().text_disabled
            } else {
                cx.theme().colors().text
            },
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features,
            font_size: rems(0.875).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.3).into(),
            background_color: None,
            underline: None,
            white_space: WhiteSpace::Normal,
        };

        EditorElement::new(
            editor,
            EditorStyle {
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}
actions!(runnables_panel, [ToggleFocus]);
impl FocusableView for RunnablesPanel {
    fn focus_handle(&self, _cx: &gpui::AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl EventEmitter<PanelEvent> for RunnablesPanel {}

impl Panel for RunnablesPanel {
    fn persistent_name() -> &'static str {
        "RunnablesPanel"
    }

    fn position(&self, cx: &ui::prelude::WindowContext) -> workspace::dock::DockPosition {
        workspace::dock::DockPosition::Right
    }

    fn position_is_valid(&self, position: workspace::dock::DockPosition) -> bool {
        matches!(
            position,
            workspace::dock::DockPosition::Left | workspace::dock::DockPosition::Right
        )
    }

    fn set_position(
        &mut self,
        position: workspace::dock::DockPosition,
        cx: &mut ui::prelude::ViewContext<Self>,
    ) {
    }

    fn size(&self, cx: &ui::prelude::WindowContext) -> ui::prelude::Pixels {
        px(400.)
    }

    fn set_size(
        &mut self,
        size: Option<ui::prelude::Pixels>,
        cx: &mut ui::prelude::ViewContext<Self>,
    ) {
    }

    fn icon(&self, cx: &ui::prelude::WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::Return)
    }

    fn icon_tooltip(&self, cx: &ui::prelude::WindowContext) -> Option<&'static str> {
        Some("Runnables panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

impl Render for RunnablesPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        //let list = List::new().empty_message("There are no runnables");
        let state = ListState::new(2, ListAlignment::Top, px(2.), |index, cx| {
            div().child("XD").into_any_element()
        });
        v_flex()
            .size_full()
            //.child(list(self.list_state.clone()).full())
            .child(list(state).full())
            .child(
                v_flex()
                    .child(div().mx_2().border_primary(cx).border_t())
                    .child(
                        v_flex()
                            .p_2()
                            .child(self.render_filter_input(&self.filter_editor, cx)),
                    ),
            )
        // h_flex().bg(red()).w_full().h_full().min_w(px(400.))
        // .child("Hey there little man")
    }
}
