pub mod save_or_reopen {
    use gpui::Styled;
    use gpui::{AppContext, ParentElement};
    use picker::Picker;
    use picker::PickerDelegate;
    use std::sync::atomic::AtomicBool;
    use util::ResultExt;

    use fuzzy::{StringMatch, StringMatchCandidate};
    use gpui::{DismissEvent, Entity, EventEmitter, Focusable, WeakEntity};

    use ui::{Context, Label, ListItem, Render, Window, rems, v_flex};
    use workspace::{ModalView, Workspace};

    pub struct EncodingSaveOrReopenSelector {
        picker: Entity<Picker<EncodingSaveOrReopenDelegate>>,
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

    impl Focusable for EncodingSaveOrReopenSelector {
        fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
            self.picker.focus_handle(cx)
        }
    }

    impl Render for EncodingSaveOrReopenSelector {
        fn render(
            &mut self,
            _window: &mut Window,
            _cx: &mut Context<Self>,
        ) -> impl ui::IntoElement {
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

        fn confirm(
            &mut self,
            secondary: bool,
            window: &mut Window,
            cx: &mut Context<Picker<Self>>,
        ) {
        }

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

    pub fn get_current_encoding() -> &'static str {
        "UTF-8"
    }
}

pub mod encoding {
    use std::sync::atomic::AtomicBool;

    use fuzzy::{StringMatch, StringMatchCandidate};
    use gpui::{
        AppContext, BackgroundExecutor, DismissEvent, Entity, EventEmitter, Focusable, WeakEntity,
    };
    use picker::{Picker, PickerDelegate};
    use ui::{Context, Label, ListItem, ParentElement, Render, Styled, Window, rems, v_flex};
    use util::{ResultExt, TryFutureExt};
    use workspace::{ModalView, Workspace};

    pub struct EncodingSelector {
        pub picker: Entity<Picker<EncodingSelectorDelegate>>,
    }

    pub struct EncodingSelectorDelegate {
        current_selection: usize,
        encodings: Vec<StringMatchCandidate>,
        matches: Vec<StringMatch>,
        selector: WeakEntity<EncodingSelector>,
    }

    impl EncodingSelectorDelegate {
        pub fn new(selector: WeakEntity<EncodingSelector>) -> EncodingSelectorDelegate {
            EncodingSelectorDelegate {
                current_selection: 0,
                encodings: vec![
                    StringMatchCandidate::new(0, "UTF-8"),
                    StringMatchCandidate::new(1, "ISO 8859-1"),
                ],
                matches: Vec::new(),
                selector,
            }
        }
    }

    impl PickerDelegate for EncodingSelectorDelegate {
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
            window: &mut Window,
            cx: &mut Context<Picker<Self>>,
        ) {
            self.current_selection = ix;
        }

        fn placeholder_text(&self, _window: &mut Window, _cx: &mut ui::App) -> std::sync::Arc<str> {
            "Select an encoding...".into()
        }

        fn update_matches(
            &mut self,
            query: String,
            window: &mut Window,
            cx: &mut Context<Picker<Self>>,
        ) -> gpui::Task<()> {
            let executor = cx.background_executor().clone();
            let encodings = self.encodings.clone();
            let current_selection = self.current_selection;

            cx.spawn_in(window, async move |picker, cx| {
                let matches: Vec<StringMatch>;

                if query.is_empty() {
                    matches = encodings
                        .into_iter()
                        .enumerate()
                        .map(|(index, value)| StringMatch {
                            candidate_id: index,
                            score: 0.0,
                            positions: Vec::new(),
                            string: value.string,
                        })
                        .collect();
                } else {
                    matches = fuzzy::match_strings(
                        &encodings,
                        &query,
                        false,
                        false,
                        0,
                        &AtomicBool::new(false),
                        executor,
                    )
                    .await
                }
            })
        }

        fn confirm(
            &mut self,
            secondary: bool,
            window: &mut Window,
            cx: &mut Context<Picker<Self>>,
        ) {
        }

        fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
            self.selector
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

    impl EncodingSelector {
        pub fn new(window: &mut Window, cx: &mut Context<EncodingSelector>) -> EncodingSelector {
            let delegate = EncodingSelectorDelegate::new(cx.entity().downgrade());
            let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

            EncodingSelector { picker: picker }
        }

        pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
            workspace.toggle_modal(window, cx, |window, cx| EncodingSelector::new(window, cx));
        }
    }

    impl EventEmitter<DismissEvent> for EncodingSelector {}

    impl Focusable for EncodingSelector {
        fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
            cx.focus_handle()
        }
    }

    impl ModalView for EncodingSelector {}

    impl Render for EncodingSelector {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl ui::IntoElement {
            v_flex().w(rems(34.0)).child(self.picker.clone())
        }
    }
}
