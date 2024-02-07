mod runnables_settings;

use std::path::PathBuf;

use editor::{Editor, EditorElement, EditorStyle};
use gpui::{
    actions, div, list, px, relative, rems, AppContext, EventEmitter, FocusHandle, FocusableView,
    FontStyle, FontWeight, IntoElement, ListAlignment, ListState, Model, ParentElement as _,
    Render, SharedString, Styled as _, TextStyle, View, ViewContext, VisualContext as _,
    WhiteSpace, WindowContext,
};
use project::Inventory;
use settings::Settings as _;
use theme::ThemeSettings;
use ui::{
    v_flex, ActiveTheme, Button, Clickable, FluentBuilder, Icon, IconButton, IconName, ListItem,
    StyledExt,
};
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
    // todo: po: should this be weak?
    inventory: Model<Inventory>,
}

impl RunnablesPanel {
    pub fn new(inventory: Model<Inventory>, cx: &mut WindowContext<'_>) -> View<Self> {
        cx.new_view(|cx| {
            let filter_editor = cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("Filter...", cx);
                editor
            });
            Self {
                focus_handle: cx.focus_handle(),
                filter_editor,
                inventory,
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
        let runnables: Vec<_> = self
            .inventory
            .read(cx)
            .list_runnables(&PathBuf::new(), cx)
            .collect();

        //let list = List::new().empty_message("There are no runnables");
        let state = ListState::new(
            runnables.len(),
            ListAlignment::Top,
            px(2.),
            move |index, cx| {
                let runnable = runnables[index].clone();
                let result = runnable.result(cx);
                let cancelable = runnable.cancel_handle(cx).filter(|_| result.is_none());
                ListItem::new(("Runnables", runnable.id()))
                    .child(
                        Button::new(
                            ("Runnable trigger", runnable.id()),
                            SharedString::from(runnable.metadata().display_name().to_owned()),
                        )
                        .on_click({
                            let runnable = runnable.clone();
                            move |_, cx| {
                                runnable.schedule(cx).ok();
                            }
                        }),
                    )
                    .when_some(result, |this, result| {
                        let succeeded = result.is_ok();
                        let icon = if succeeded {
                            IconName::Check
                        } else {
                            IconName::AtSign
                        };
                        this.start_slot(Icon::new(icon))
                    })
                    .when_some(cancelable, |this, cancel_token| {
                        this.end_slot(
                            IconButton::new(
                                ("Runnable cancel button", runnable.id()),
                                IconName::XCircle,
                            )
                            .on_click(move |_, _| {
                                cancel_token.abort();
                            }),
                        )
                    })
                    .into_any_element()
            },
        );
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
    }
}
