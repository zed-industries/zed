use editor::{movement, scroll::Autoscroll, Editor};
use gpui::{actions, Action};
use ui::ViewContext;

use crate::{motion::Motion, state::Mode, Vim};

actions!(vim, [HelixNormalAfter]);

pub fn register(editor: &mut Editor, cx: &mut ViewContext<Vim>) {
    Vim::action(editor, cx, Vim::helix_normal_after);
}

impl Vim {
    
    pub fn helix_normal_after(&mut self, action: &HelixNormalAfter, cx: &mut ViewContext<Self>) {
        if self.active_operator().is_some() {
            self.operator_stack.clear();
            self.sync_vim_settings(cx);
            return;
        }
        self.stop_recording_immediately(action.boxed_clone(), cx);
        self.switch_mode(Mode::HelixNormal, false, cx);
        return
    }
    
    pub fn helix_normal_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        cx: &mut ViewContext<Self>,
    ) {
        self.helix_move_cursor(motion, times, cx);
    }

  pub fn helix_move_cursor(
    &mut self,
    motion: Motion,
    times: Option<usize>,
    cx: &mut ViewContext<Self>,
  ) {
    self.update_editor(cx, |_, editor, cx| {
        let text_layout_details = editor.text_layout_details(cx);
        editor.change_selections(Some(Autoscroll::fit()), cx, |s| match motion {
            Motion::Left | Motion::Right | Motion::Up { .. } | Motion::Down { .. } => {
                s.move_cursors_with(|map, cursor, goal| {
                    motion
                        .move_point(map, cursor, goal, times, &text_layout_details)
                        .unwrap_or((cursor, goal))
                })
            }

            Motion::NextWordStart { ignore_punctuation } => {
                s.move_with(|map, selection| {
                    let times = times.unwrap_or(1);

                    if selection.head() == map.max_point() {
                        return;
                    }

                    // collapse to block cursor
                    if selection.tail() < selection.head() {
                        selection.set_tail(movement::left(map, selection.head()), selection.goal);
                    } else {
                        selection.set_tail(selection.head(), selection.goal);
                        selection.set_head(movement::right(map, selection.head()), selection.goal);
                    }
                    
                    // create a classifier
                    let classifier = map
                        .buffer_snapshot
                        .char_classifier_at(selection.head().to_point(map))
                        .ignore_punctuation(ignore_punctuation);
                    use language::CharKind;

                    let mut last_selection = selection.clone();
                    for _ in 0..times  {

                        let (new_tail, new_head) = movement::find_boundary_trail(map, selection.head(), selection.tail(), |left, right| {
                            let left_kind = classifier.kind(left);
                            let right_kind = classifier.kind(right);
                            let at_newline = right == '\n';
                
                            let found = left_kind != right_kind && right_kind != CharKind::Whitespace && !at_newline;
                                
                            found
                        });

                        selection.set_head(new_head, selection.goal);
                        selection.set_tail(new_tail, selection.goal);

                        if selection.head() == last_selection.head() && selection.tail() == last_selection.tail() {
                            break;
                        }
                        last_selection = selection.clone();
                    }

                });
            },
            Motion::NextWordEnd { ignore_punctuation } => {
                s.move_with(|map, selection| {
                    let times = times.unwrap_or(1);

                    if selection.head() == map.max_point() {
                        return;
                    }

                    // collapse to block cursor
                    if selection.tail() < selection.head() {
                        selection.set_tail(movement::left(map, selection.head()), selection.goal);
                    } else {
                        selection.set_tail(selection.head(), selection.goal);
                        selection.set_head(movement::right(map, selection.head()), selection.goal);
                    }
                    
                    // create a classifier
                    let classifier = map
                        .buffer_snapshot
                        .char_classifier_at(selection.head().to_point(map))
                        .ignore_punctuation(ignore_punctuation);
                    use language::CharKind;

                    let mut last_selection = selection.clone();
                    for _ in 0..times  {

                        let (new_tail, new_head) = movement::find_boundary_trail(map, selection.head(), selection.tail(), |left, right| {
                            let left_kind = classifier.kind(left);
                            let right_kind = classifier.kind(right);
                            let at_newline = right == '\n';
                
                            let found = left_kind != right_kind && (left_kind != CharKind::Whitespace || at_newline);
                                
                            found
                        });

                        selection.set_head(new_head, selection.goal);
                        selection.set_tail(new_tail, selection.goal);

                        if selection.head() == last_selection.head() && selection.tail() == last_selection.tail() {
                            break;
                        }
                        last_selection = selection.clone();
                    }

                });
            }

            Motion::PreviousWordEnd { ignore_punctuation } => {
                s.move_with(|map, selection| {
                    let times = times.unwrap_or(1);

                    // cheeky way to determine minimum point
                    if selection.head() == movement::left(map, selection.head()){
                        return;
                    }

                    // collapse to block cursor
                    if selection.tail() < selection.head() {
                        selection.set_tail(movement::left(map, selection.head()), selection.goal);
                    } else {
                        selection.set_tail(selection.head(), selection.goal);
                        selection.set_head(movement::right(map, selection.head()), selection.goal);
                    }

                    // flip the selection
                    selection.swap_head_tail();
                    
                    // create a classifier
                    let classifier = map
                        .buffer_snapshot
                        .char_classifier_at(selection.head().to_point(map))
                        .ignore_punctuation(ignore_punctuation);
                    use language::CharKind;

                    let mut last_selection = selection.clone();
                    for _ in 0..times  {

                        let (new_tail, new_head) = movement::find_preceding_boundary_trail(map, selection.head(), selection.tail(), |left, right| {
                            
                            let left_kind = classifier.kind(left);
                            let right_kind = classifier.kind(right);
                            let at_newline = right == '\n';
                
                            let found = left_kind != right_kind && right_kind != CharKind::Whitespace && !at_newline;
                                
                            found
                        });

                        selection.set_head(new_head, selection.goal);
                        selection.set_tail(new_tail, selection.goal);

                        if selection.head() == last_selection.head() && selection.tail() == last_selection.tail() {
                            break;
                        }
                        last_selection = selection.clone();
                    }

                });
            }

            Motion::PreviousWordStart { ignore_punctuation } => {
                s.move_with(|map, selection| {
                    let times = times.unwrap_or(1);

                    // cheeky way to determine minimum point
                    if selection.head() == movement::left(map, selection.head()){
                        return;
                    }

                    // collapse to block cursor
                    if selection.tail() < selection.head() {
                        selection.set_tail(movement::left(map, selection.head()), selection.goal);
                    } else {
                        selection.set_tail(selection.head(), selection.goal);
                        selection.set_head(movement::right(map, selection.head()), selection.goal);
                    }

                    // flip the selection
                    selection.swap_head_tail();
                    
                    // create a classifier
                    let classifier = map
                        .buffer_snapshot
                        .char_classifier_at(selection.head().to_point(map))
                        .ignore_punctuation(ignore_punctuation);
                    use language::CharKind;

                    let mut last_selection = selection.clone();
                    for _ in 0..times  {

                        let (new_tail, new_head) = movement::find_preceding_boundary_trail(map, selection.head(), selection.tail(), |left, right| {
                            
                            let left_kind = classifier.kind(left);
                            let right_kind = classifier.kind(right);
                            let at_newline = right == '\n';
                
                            let found = left_kind != right_kind && (left_kind != CharKind::Whitespace || at_newline);
                                
                            found
                        });

                        selection.set_head(new_head, selection.goal);
                        selection.set_tail(new_tail, selection.goal);

                        if selection.head() == last_selection.head() && selection.tail() == last_selection.tail() {
                            break;
                        }
                        last_selection = selection.clone();
                    }

                });
            }
            
            _ => {

            }
        })
    });
}
}

