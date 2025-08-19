use std::sync::Weak;
use std::sync::atomic::AtomicBool;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{AppContext, ClickEvent, DismissEvent, Entity, EventEmitter, Focusable, WeakEntity};
use language::Buffer;
use picker::{Picker, PickerDelegate};
use ui::{
    Button, ButtonCommon, Context, Label, LabelSize, ListItem, Render, Styled, Tooltip, Window,
    div, rems, v_flex,
};
use ui::{Clickable, ParentElement};
use util::ResultExt;
use workspace::{ItemHandle, ModalView, StatusItemView, Workspace};

pub enum Encoding {
    Utf8(WeakEntity<Workspace>),
}

impl Encoding {
    pub fn as_str(&self) -> &str {
        match &self {
            Encoding::Utf8(_) => "UTF-8",
        }
    }
}

impl EncodingSaveOrReopenSelector {
    pub fn new(window: &mut Window, cx: &mut Context<EncodingSaveOrReopenSelector>) -> Self {
        let delegate = EncodingSaveOrReopenDelegate::new(cx.entity().downgrade());

        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        Self { picker }
    }

    pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
        workspace.toggle_modal(window, cx, |window, cx| {
            EncodingSaveOrReopenSelector::new(window, cx)
        });
    }
}

pub struct EncodingSaveOrReopenSelector {
    picker: Entity<Picker<EncodingSaveOrReopenDelegate>>,
}

impl Focusable for EncodingSaveOrReopenSelector {
    fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for EncodingSaveOrReopenSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl ui::IntoElement {
        v_flex().w(rems(34.0)).child(self.picker.clone())
    }
}

impl ModalView for EncodingSaveOrReopenSelector {}

impl EventEmitter<DismissEvent> for EncodingSaveOrReopenSelector {}

pub struct EncodingSaveOrReopenDelegate {
    encoding_selector: WeakEntity<EncodingSaveOrReopenSelector>,
    current_selection: usize,
    matches: Vec<StringMatch>,
    pub actions: Vec<StringMatchCandidate>,
}

impl EncodingSaveOrReopenDelegate {
    pub fn new(selector: WeakEntity<EncodingSaveOrReopenSelector>) -> Self {
        Self {
            encoding_selector: selector,
            current_selection: 0,
            matches: Vec::new(),
            actions: vec![
                StringMatchCandidate::new(0, "Save with encoding"),
                StringMatchCandidate::new(1, "Reopen with encoding"),
            ],
        }
    }

    pub fn get_actions(&self) -> (&str, &str) {
        (&self.actions[0].string, &self.actions[1].string)
    }
}

impl PickerDelegate for EncodingSaveOrReopenDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.current_selection
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.current_selection = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut ui::App) -> std::sync::Arc<str> {
        "Select an action...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> gpui::Task<()> {
        let executor = cx.background_executor().clone();
        let actions = self.actions.clone();

        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                actions
                    .into_iter()
                    .enumerate()
                    .map(|(index, value)| StringMatch {
                        candidate_id: index,
                        score: 0.0,
                        positions: vec![],
                        string: value.string,
                    })
                    .collect::<Vec<StringMatch>>()
            } else {
                fuzzy::match_strings(
                    &actions,
                    &query,
                    false,
                    false,
                    2,
                    &AtomicBool::new(false),
                    executor,
                )
                .await
            };

            this.update(cx, |picker, cx| {
                let delegate = &mut picker.delegate;
                delegate.current_selection = matches.len().saturating_sub(1);
                delegate.matches = matches;
                cx.notify();
            })
            .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {}

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.encoding_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some(ListItem::new(ix).child(Label::new(&self.matches[ix].string)))
    }
}

fn get_current_encoding() -> &'static str {
    "UTF-8"
}

impl Render for Encoding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let encoding_indicator = div();

        encoding_indicator.child(
            Button::new("encoding", get_current_encoding())
                .label_size(LabelSize::Small)
                .tooltip(Tooltip::text("Select Encoding"))
                .on_click(cx.listener(|encoding, _: &ClickEvent, window, cx| {
                    if let Some(workspace) = match encoding {
                        Encoding::Utf8(workspace) => workspace.upgrade(),
                    } {
                        workspace.update(cx, |workspace, cx| {
                            EncodingSaveOrReopenSelector::toggle(workspace, window, cx)
                        })
                    } else {
                    }
                })),
        )
    }
}

impl StatusItemView for Encoding {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
