pub enum ContextMenu {
    Completions(CompletionsMenu),
    CodeActions(CodeActionsMenu),
}

impl ContextMenu {
    pub fn select_prev(&mut self, cx: &mut ViewContext<Editor>) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_prev(cx),
                ContextMenu::CodeActions(menu) => menu.select_prev(cx),
            }
            true
        } else {
            false
        }
    }

    pub fn select_next(&mut self, cx: &mut ViewContext<Editor>) -> bool {
        if self.visible() {
            match self {
                ContextMenu::Completions(menu) => menu.select_next(cx),
                ContextMenu::CodeActions(menu) => menu.select_next(cx),
            }
            true
        } else {
            false
        }
    }

    pub fn visible(&self) -> bool {
        match self {
            ContextMenu::Completions(menu) => menu.visible(),
            ContextMenu::CodeActions(menu) => menu.visible(),
        }
    }

    pub fn render(
        &self,
        cursor_position: DisplayPoint,
        style: EditorStyle,
        cx: &AppContext,
    ) -> (DisplayPoint, ElementBox) {
        match self {
            ContextMenu::Completions(menu) => (cursor_position, menu.render(style, cx)),
            ContextMenu::CodeActions(menu) => menu.render(cursor_position, style),
        }
    }
}

struct CompletionsMenu {
    id: CompletionId,
    initial_position: Anchor,
    buffer: ModelHandle<Buffer>,
    completions: Arc<[Completion]>,
    match_candidates: Vec<StringMatchCandidate>,
    matches: Arc<[StringMatch]>,
    selected_item: usize,
    list: UniformListState,
}

impl CompletionsMenu {
    fn select_prev(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
            self.list.scroll_to(ScrollTarget::Show(self.selected_item));
        }
        cx.notify();
    }

    fn select_next(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item + 1 < self.matches.len() {
            self.selected_item += 1;
            self.list.scroll_to(ScrollTarget::Show(self.selected_item));
        }
        cx.notify();
    }

    fn visible(&self) -> bool {
        !self.matches.is_empty()
    }

    fn render(&self, style: EditorStyle, _: &AppContext) -> ElementBox {
        enum CompletionTag {}

        let completions = self.completions.clone();
        let matches = self.matches.clone();
        let selected_item = self.selected_item;
        let container_style = style.autocomplete.container;
        UniformList::new(self.list.clone(), matches.len(), move |range, items, cx| {
            let start_ix = range.start;
            for (ix, mat) in matches[range].iter().enumerate() {
                let completion = &completions[mat.candidate_id];
                let item_ix = start_ix + ix;
                items.push(
                    MouseEventHandler::new::<CompletionTag, _, _>(
                        mat.candidate_id,
                        cx,
                        |state, _| {
                            let item_style = if item_ix == selected_item {
                                style.autocomplete.selected_item
                            } else if state.hovered {
                                style.autocomplete.hovered_item
                            } else {
                                style.autocomplete.item
                            };

                            Text::new(completion.label.text.clone(), style.text.clone())
                                .with_soft_wrap(false)
                                .with_highlights(combine_syntax_and_fuzzy_match_highlights(
                                    &completion.label.text,
                                    style.text.color.into(),
                                    styled_runs_for_code_label(&completion.label, &style.syntax),
                                    &mat.positions,
                                ))
                                .contained()
                                .with_style(item_style)
                                .boxed()
                        },
                    )
                    .with_cursor_style(CursorStyle::PointingHand)
                    .on_mouse_down(move |cx| {
                        cx.dispatch_action(ConfirmCompletion(Some(item_ix)));
                    })
                    .boxed(),
                );
            }
        })
        .with_width_from_item(
            self.matches
                .iter()
                .enumerate()
                .max_by_key(|(_, mat)| {
                    self.completions[mat.candidate_id]
                        .label
                        .text
                        .chars()
                        .count()
                })
                .map(|(ix, _)| ix),
        )
        .contained()
        .with_style(container_style)
        .boxed()
    }

    pub async fn filter(&mut self, query: Option<&str>, executor: Arc<executor::Background>) {
        let mut matches = if let Some(query) = query {
            fuzzy::match_strings(
                &self.match_candidates,
                query,
                false,
                100,
                &Default::default(),
                executor,
            )
            .await
        } else {
            self.match_candidates
                .iter()
                .enumerate()
                .map(|(candidate_id, candidate)| StringMatch {
                    candidate_id,
                    score: Default::default(),
                    positions: Default::default(),
                    string: candidate.string.clone(),
                })
                .collect()
        };
        matches.sort_unstable_by_key(|mat| {
            (
                Reverse(OrderedFloat(mat.score)),
                self.completions[mat.candidate_id].sort_key(),
            )
        });

        for mat in &mut matches {
            let filter_start = self.completions[mat.candidate_id].label.filter_range.start;
            for position in &mut mat.positions {
                *position += filter_start;
            }
        }

        self.matches = matches.into();
    }
}

#[derive(Clone)]
struct CodeActionsMenu {
    actions: Arc<[CodeAction]>,
    buffer: ModelHandle<Buffer>,
    selected_item: usize,
    list: UniformListState,
    deployed_from_indicator: bool,
}

impl CodeActionsMenu {
    fn select_prev(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item > 0 {
            self.selected_item -= 1;
            cx.notify()
        }
    }

    fn select_next(&mut self, cx: &mut ViewContext<Editor>) {
        if self.selected_item + 1 < self.actions.len() {
            self.selected_item += 1;
            cx.notify()
        }
    }

    fn visible(&self) -> bool {
        !self.actions.is_empty()
    }

    fn render(
        &self,
        mut cursor_position: DisplayPoint,
        style: EditorStyle,
    ) -> (DisplayPoint, ElementBox) {
        enum ActionTag {}

        let container_style = style.autocomplete.container;
        let actions = self.actions.clone();
        let selected_item = self.selected_item;
        let element =
            UniformList::new(self.list.clone(), actions.len(), move |range, items, cx| {
                let start_ix = range.start;
                for (ix, action) in actions[range].iter().enumerate() {
                    let item_ix = start_ix + ix;
                    items.push(
                        MouseEventHandler::new::<ActionTag, _, _>(item_ix, cx, |state, _| {
                            let item_style = if item_ix == selected_item {
                                style.autocomplete.selected_item
                            } else if state.hovered {
                                style.autocomplete.hovered_item
                            } else {
                                style.autocomplete.item
                            };

                            Text::new(action.lsp_action.title.clone(), style.text.clone())
                                .with_soft_wrap(false)
                                .contained()
                                .with_style(item_style)
                                .boxed()
                        })
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_mouse_down(move |cx| {
                            cx.dispatch_action(ConfirmCodeAction(Some(item_ix)));
                        })
                        .boxed(),
                    );
                }
            })
            .with_width_from_item(
                self.actions
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, action)| action.lsp_action.title.chars().count())
                    .map(|(ix, _)| ix),
            )
            .contained()
            .with_style(container_style)
            .boxed();

        if self.deployed_from_indicator {
            *cursor_position.column_mut() = 0;
        }

        (cursor_position, element)
    }
}
