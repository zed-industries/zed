/// This module contains the encoding selectors for saving or reopening files with a different encoding.
/// It provides a modal view that allows the user to choose between saving with a different encoding
/// or reopening with a different encoding, and then selecting the desired encoding from a list.
pub mod save_or_reopen {
    use editor::Editor;
    use gpui::Styled;
    use gpui::{AppContext, ParentElement};
    use picker::Picker;
    use picker::PickerDelegate;
    use std::sync::atomic::AtomicBool;
    use util::ResultExt;

    use fuzzy::{StringMatch, StringMatchCandidate};
    use gpui::{DismissEvent, Entity, EventEmitter, Focusable, WeakEntity};

    use ui::{Context, HighlightedLabel, ListItem, Render, Window, rems, v_flex};
    use workspace::{ModalView, Workspace};

    use crate::selectors::encoding::{Action, EncodingSelector};

    /// A modal view that allows the user to select between saving with a different encoding or
    /// reopening with a different encoding.
    pub struct EncodingSaveOrReopenSelector {
        picker: Entity<Picker<EncodingSaveOrReopenDelegate>>,
        pub current_selection: usize,
    }

    impl EncodingSaveOrReopenSelector {
        pub fn new(
            window: &mut Window,
            cx: &mut Context<EncodingSaveOrReopenSelector>,
            workspace: WeakEntity<Workspace>,
        ) -> Self {
            let delegate = EncodingSaveOrReopenDelegate::new(cx.entity().downgrade(), workspace);

            let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

            Self {
                picker,
                current_selection: 0,
            }
        }

        /// Toggle the modal view for selecting between saving with a different encoding or
        /// reopening with a different encoding.
        pub fn toggle(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
            let weak_workspace = workspace.weak_handle();
            workspace.toggle_modal(window, cx, |window, cx| {
                EncodingSaveOrReopenSelector::new(window, cx, weak_workspace)
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
        selector: WeakEntity<EncodingSaveOrReopenSelector>,
        current_selection: usize,
        matches: Vec<StringMatch>,
        pub actions: Vec<StringMatchCandidate>,
        workspace: WeakEntity<Workspace>,
    }

    impl EncodingSaveOrReopenDelegate {
        pub fn new(
            selector: WeakEntity<EncodingSaveOrReopenSelector>,
            workspace: WeakEntity<Workspace>,
        ) -> Self {
            Self {
                selector,
                current_selection: 0,
                matches: Vec::new(),
                actions: vec![
                    StringMatchCandidate::new(0, "Save with encoding"),
                    StringMatchCandidate::new(1, "Reopen with encoding"),
                ],
                workspace,
            }
        }

        pub fn get_actions(&self) -> (&str, &str) {
            (&self.actions[0].string, &self.actions[1].string)
        }

        /// Handle the action selected by the user.
        pub fn post_selection(
            &self,
            cx: &mut Context<Picker<EncodingSaveOrReopenDelegate>>,
            window: &mut Window,
        ) -> Option<()> {
            if self.current_selection == 0 {
                if let Some(workspace) = self.workspace.upgrade() {
                    let (_, buffer, _) = workspace
                        .read(cx)
                        .active_item(cx)?
                        .act_as::<Editor>(cx)?
                        .read(cx)
                        .active_excerpt(cx)?;

                    let weak_workspace = workspace.read(cx).weak_handle();

                    workspace.update(cx, |workspace, cx| {
                        workspace.toggle_modal(window, cx, |window, cx| {
                            let selector = EncodingSelector::new(
                                window,
                                cx,
                                Action::Save,
                                Some(buffer.downgrade()),
                                weak_workspace,
                                None,
                            );
                            selector
                        })
                    });
                }
            } else if self.current_selection == 1 {
                if let Some(workspace) = self.workspace.upgrade() {
                    let (_, buffer, _) = workspace
                        .read(cx)
                        .active_item(cx)?
                        .act_as::<Editor>(cx)?
                        .read(cx)
                        .active_excerpt(cx)?;

                    let weak_workspace = workspace.read(cx).weak_handle();

                    workspace.update(cx, |workspace, cx| {
                        workspace.toggle_modal(window, cx, |window, cx| {
                            let selector = EncodingSelector::new(
                                window,
                                cx,
                                Action::Reopen,
                                Some(buffer.downgrade()),
                                weak_workspace,
                                None,
                            );
                            selector
                        });
                    });
                }
            }

            Some(())
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
            cx: &mut Context<Picker<Self>>,
        ) {
            self.current_selection = ix;
            self.selector
                .update(cx, |selector, _cx| {
                    selector.current_selection = ix;
                })
                .log_err();
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
                    delegate.matches = matches;
                    delegate.current_selection = delegate
                        .current_selection
                        .min(delegate.matches.len().saturating_sub(1));
                    delegate
                        .selector
                        .update(cx, |selector, _cx| {
                            selector.current_selection = delegate.current_selection
                        })
                        .log_err();
                    cx.notify();
                })
                .log_err();
            })
        }

        fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
            self.dismissed(window, cx);
            if self.selector.is_upgradable() {
                self.post_selection(cx, window);
            }
        }

        fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
            self.selector
                .update(cx, |_, cx| cx.emit(DismissEvent))
                .log_err();
        }

        fn render_match(
            &self,
            ix: usize,
            _: bool,
            _: &mut Window,
            _: &mut Context<Picker<Self>>,
        ) -> Option<Self::ListItem> {
            Some(
                ListItem::new(ix)
                    .child(HighlightedLabel::new(
                        &self.matches[ix].string,
                        self.matches[ix].positions.clone(),
                    ))
                    .spacing(ui::ListItemSpacing::Sparse),
            )
        }
    }
}

/// This module contains the encoding selector for choosing an encoding to save or reopen a file with.
pub mod encoding {
    use std::{path::PathBuf, sync::atomic::AtomicBool};

    use fuzzy::{StringMatch, StringMatchCandidate};
    use gpui::{AppContext, DismissEvent, Entity, EventEmitter, Focusable, WeakEntity};
    use language::Buffer;
    use picker::{Picker, PickerDelegate};
    use ui::{
        Context, HighlightedLabel, ListItem, ListItemSpacing, ParentElement, Render, Styled,
        Window, rems, v_flex,
    };
    use util::{ResultExt, TryFutureExt};
    use workspace::{ModalView, Workspace};

    use crate::encoding_from_name;

    /// A modal view that allows the user to select an encoding from a list of encodings.
    pub struct EncodingSelector {
        picker: Entity<Picker<EncodingSelectorDelegate>>,
        workspace: WeakEntity<Workspace>,
        path: Option<PathBuf>,
    }

    pub struct EncodingSelectorDelegate {
        current_selection: usize,
        encodings: Vec<StringMatchCandidate>,
        matches: Vec<StringMatch>,
        selector: WeakEntity<EncodingSelector>,
        buffer: Option<WeakEntity<Buffer>>,
        action: Action,
    }

    impl EncodingSelectorDelegate {
        pub fn new(
            selector: WeakEntity<EncodingSelector>,
            buffer: Option<WeakEntity<Buffer>>,
            action: Action,
        ) -> EncodingSelectorDelegate {
            EncodingSelectorDelegate {
                current_selection: 0,
                encodings: vec![
                    StringMatchCandidate::new(0, "UTF-8"),
                    StringMatchCandidate::new(1, "UTF-16 LE"),
                    StringMatchCandidate::new(2, "UTF-16 BE"),
                    StringMatchCandidate::new(3, "Windows-1252"),
                    StringMatchCandidate::new(4, "Windows-1251"),
                    StringMatchCandidate::new(5, "Windows-1250"),
                    StringMatchCandidate::new(6, "ISO 8859-2"),
                    StringMatchCandidate::new(7, "ISO 8859-3"),
                    StringMatchCandidate::new(8, "ISO 8859-4"),
                    StringMatchCandidate::new(9, "ISO 8859-5"),
                    StringMatchCandidate::new(10, "ISO 8859-6"),
                    StringMatchCandidate::new(11, "ISO 8859-7"),
                    StringMatchCandidate::new(12, "ISO 8859-8"),
                    StringMatchCandidate::new(13, "ISO 8859-13"),
                    StringMatchCandidate::new(14, "ISO 8859-15"),
                    StringMatchCandidate::new(15, "KOI8-R"),
                    StringMatchCandidate::new(16, "KOI8-U"),
                    StringMatchCandidate::new(17, "MacRoman"),
                    StringMatchCandidate::new(18, "Mac Cyrillic"),
                    StringMatchCandidate::new(19, "Windows-874"),
                    StringMatchCandidate::new(20, "Windows-1253"),
                    StringMatchCandidate::new(21, "Windows-1254"),
                    StringMatchCandidate::new(22, "Windows-1255"),
                    StringMatchCandidate::new(23, "Windows-1256"),
                    StringMatchCandidate::new(24, "Windows-1257"),
                    StringMatchCandidate::new(25, "Windows-1258"),
                    StringMatchCandidate::new(26, "Windows-949"),
                    StringMatchCandidate::new(27, "EUC-JP"),
                    StringMatchCandidate::new(28, "ISO 2022-JP"),
                    StringMatchCandidate::new(29, "GBK"),
                    StringMatchCandidate::new(30, "GB18030"),
                    StringMatchCandidate::new(31, "Big5"),
                ],
                matches: Vec::new(),
                selector,
                buffer: buffer,
                action,
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

        fn set_selected_index(&mut self, ix: usize, _: &mut Window, _: &mut Context<Picker<Self>>) {
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
                        true,
                        false,
                        30,
                        &AtomicBool::new(false),
                        executor,
                    )
                    .await
                }
                picker
                    .update(cx, |picker, cx| {
                        let delegate = &mut picker.delegate;
                        delegate.matches = matches;
                        delegate.current_selection = delegate
                            .current_selection
                            .min(delegate.matches.len().saturating_sub(1));
                        cx.notify();
                    })
                    .log_err();
            })
        }

        fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
            let workspace = self
                .selector
                .upgrade()
                .unwrap()
                .read(cx)
                .workspace
                .upgrade()
                .unwrap();

            if let Some(buffer) = &self.buffer
                && let Some(buffer) = buffer.upgrade()
            {
                buffer.update(cx, |buffer, cx| {
                    let buffer_encoding = buffer.encoding.clone();
                    let buffer_encoding = &mut *buffer_encoding.lock().unwrap();
                    *buffer_encoding =
                        encoding_from_name(self.matches[self.current_selection].string.as_str());
                    if self.action == Action::Reopen {
                        let executor = cx.background_executor().clone();
                        executor.spawn(buffer.reload(cx)).detach();
                    } else if self.action == Action::Save {
                        let executor = cx.background_executor().clone();

                        executor
                            .spawn(workspace.update(cx, |workspace, cx| {
                                workspace
                                    .save_active_item(workspace::SaveIntent::Save, window, cx)
                                    .log_err()
                            }))
                            .detach();
                    }
                });
            } else {
                workspace.update(cx, |workspace, cx| {
                    workspace
                        .open_abs_path(
                            self.selector
                                .upgrade()
                                .unwrap()
                                .read(cx)
                                .path
                                .as_ref()
                                .unwrap()
                                .clone(),
                            Default::default(),
                            window,
                            cx,
                        )
                        .detach();
                })
            }
            self.dismissed(window, cx);
        }

        fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
            self.selector
                .update(cx, |_, cx| cx.emit(DismissEvent))
                .log_err();
        }

        fn render_match(
            &self,
            ix: usize,
            _: bool,
            _: &mut Window,
            _: &mut Context<Picker<Self>>,
        ) -> Option<Self::ListItem> {
            Some(
                ListItem::new(ix)
                    .child(HighlightedLabel::new(
                        &self.matches[ix].string,
                        self.matches[ix].positions.clone(),
                    ))
                    .spacing(ListItemSpacing::Sparse),
            )
        }
    }

    /// The action to perform after selecting an encoding.
    #[derive(PartialEq, Clone)]
    pub enum Action {
        Save,
        Reopen,
    }

    impl EncodingSelector {
        pub fn new(
            window: &mut Window,
            cx: &mut Context<EncodingSelector>,
            action: Action,
            buffer: Option<WeakEntity<Buffer>>,
            workspace: WeakEntity<Workspace>,
            path: Option<PathBuf>,
        ) -> EncodingSelector {
            let delegate = EncodingSelectorDelegate::new(cx.entity().downgrade(), buffer, action);
            let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

            EncodingSelector {
                picker,
                workspace,
                path,
            }
        }
    }

    impl EventEmitter<DismissEvent> for EncodingSelector {}

    impl Focusable for EncodingSelector {
        fn focus_handle(&self, cx: &ui::App) -> gpui::FocusHandle {
            self.picker.focus_handle(cx)
        }
    }

    impl ModalView for EncodingSelector {}

    impl Render for EncodingSelector {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl ui::IntoElement {
            v_flex().w(rems(34.0)).child(self.picker.clone())
        }
    }
}
