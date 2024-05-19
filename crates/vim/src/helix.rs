use editor::{movement, scroll::Autoscroll};
use gpui::actions;
use language::{char_kind, CharKind};
use ui::{ViewContext, WindowContext};
use workspace::Workspace;

use crate::{motion::Motion, normal::normal_motion, visual::visual_motion, Vim};

actions!(helix, [SelectNextLine,]);

pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
    workspace.register_action(|_: &mut Workspace, _: &SelectNextLine, cx: _| select_next_line(cx));
}

fn select_next_line(cx: &mut WindowContext) {}

pub fn helix_normal_motion(motion: Motion, times: Option<usize>, cx: &mut WindowContext) {
    // Helix only selects the last of the motions
    if let Some(times) = times {
        normal_motion(motion.to_owned(), None, Some(times - 1), cx);
    };
    match motion {
        Motion::Up { .. } | Motion::Down { .. } | Motion::Right | Motion::Left => {
            normal_motion(motion.to_owned(), None, None, cx);
            select_current(cx);
        }
        Motion::NextWordStart { .. } => {
            next_word(motion, cx);
        }
        Motion::NextWordEnd { .. } => {
            next_word(motion, cx);
        }
        Motion::PreviousWordStart { .. } => {
            // TODO same problem as NextWordEnd (single punctuation in word)
            clear_selection(cx);
            normal_motion(Motion::Left, None, None, cx);
            visual_motion(motion.to_owned(), None, cx);
        }
        _ => {
            clear_selection(cx);
            visual_motion(motion.to_owned(), None, cx);
        }
    };
}

// calling this with motion other than NextWordStart or NextWordEnd panicks
fn next_word(motion: Motion, cx: &mut WindowContext) {
    // TODO: single punctuation in word are skipped in NextWordEnd

    // exceptions from simple "select from next char to delimiter"
    let mut select_cur = false;
    let mut rev = false;
    let (left_kind, right_kind) = get_left_right_kind(cx);
    if !is_selection_reverse(cx) {
        match motion {
            Motion::NextWordStart { .. } => {
                if !(left_kind == CharKind::Whitespace && right_kind != CharKind::Whitespace) {
                    select_cur = true;
                }
            }
            Motion::NextWordEnd { .. } => {
                if !(left_kind != CharKind::Whitespace && right_kind == CharKind::Whitespace) {
                    select_cur = true;
                }
            }
            _ => todo!("error"),
        }
        if (left_kind == CharKind::Punctuation || right_kind == CharKind::Punctuation)
            && left_kind != right_kind
        {
            select_cur = false;
        }
    } else {
        if left_kind == CharKind::Whitespace && right_kind != CharKind::Whitespace {
            rev = true;
        }
    }
    println!(
        "{:?}, {:?}; rev: {rev}, sel_cur: {select_cur}",
        left_kind, right_kind
    );
    if select_cur {
        select_current(cx);
    } else if rev {
        normal_motion(Motion::Right, None, None, cx);
        clear_selection(cx);
    } else {
        clear_selection(cx);
    }
    visual_motion(motion.to_owned(), None, cx);
    match motion {
        Motion::NextWordStart { .. } => visual_motion(Motion::Left, None, cx),
        _ => {}
    }
}

fn is_selection_reverse(cx: &mut WindowContext) -> bool {
    let mut rev = false;
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|_, selection| {
                    rev = selection.reversed;
                });
            });
        });
    });
    rev
}

fn get_left_right_kind(cx: &mut WindowContext) -> (CharKind, CharKind) {
    let mut right_kind = CharKind::Whitespace;
    let mut left_kind = CharKind::Whitespace;
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let scope = map
                        .buffer_snapshot
                        .language_scope_at(selection.end.to_point(map));

                    // find chars around cursor
                    let mut chars = map.display_chars_at(selection.start);
                    if !selection.reversed {
                        for ch in chars {
                            if ch.1 == selection.head() {
                                right_kind = char_kind(&scope, ch.0);
                                break;
                            } else {
                                left_kind = char_kind(&scope, ch.0);
                            }
                        }
                    } else {
                        if let (Some(left), Some(right)) = (chars.next(), chars.next()) {
                            (left_kind, right_kind) =
                                (char_kind(&scope, left.0), char_kind(&scope, right.0));
                        }
                    }
                });
            });
        });
    });
    (left_kind, right_kind)
}

fn clear_selection(cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|_, selection| {
                    let point = selection.head();
                    selection.collapse_to(point, selection.goal);
                });
            });
        });
    });
}

fn select_current(cx: &mut WindowContext) {
    // go left so selecting right selects current
    normal_motion(Motion::Left, None, None, cx);
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    // Clear previous selection
                    let point = selection.head();
                    selection.collapse_to(point, selection.goal);
                    //select right
                    selection.end = movement::right(map, selection.start)
                });
            });
        });
    });
}

// fn next_word_start(motion: Motion, cx: &mut WindowContext) {
//     let mut select_cur = false;
//     let mut rev = false;
//     Vim::update(cx, |vim, cx| {
//         vim.update_active_editor(cx, |_, editor, cx| {
//             editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
//                 s.move_with(|map, selection| {
//                     let mut current_char = ' ';
//                     let mut next_char = ' ';

//                     let mut chars = map.display_chars_at(selection.start);
//                     if !selection.reversed {
//                         for ch in chars {
//                             if ch.1 == selection.head() {
//                                 next_char = ch.0;
//                                 break;
//                             } else {
//                                 current_char = ch.0;
//                             }
//                         }
//                         // if on space right before word, dont select that space
//                         if !(current_char == ' ' && next_char != ' ') {
//                             select_cur = true;
//                         }
//                     } else {
//                         if let (Some(cur), Some(next)) = (chars.next(), chars.next()) {
//                             if cur.0 == ' ' && next.0 != ' ' {
//                                 rev = true;
//                             }
//                         }
//                     }
//                 });
//             });
//         });
//     });

//     if select_cur {
//         select_current(cx);
//     } else if rev {
//         normal_motion(Motion::Right, None, None, cx);
//         clear_selection(cx);
//     } else {
//         clear_selection(cx);
//     }
//     visual_motion(motion.to_owned(), None, cx);
//     visual_motion(Motion::Left, None, cx);
// }

// fn next_word_end(motion: Motion, cx: &mut WindowContext) {
//     let mut select_cur = false;
//     let mut rev = false;
//     Vim::update(cx, |vim, cx| {
//         vim.update_active_editor(cx, |_, editor, cx| {
//             editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
//                 s.move_with(|map, selection| {
//                     let mut current_char = ' ';
//                     let mut next_char = ' ';

//                     let mut chars = map.display_chars_at(selection.start);
//                     if !selection.reversed {
//                         for ch in chars {
//                             if ch.1 == selection.head() {
//                                 next_char = ch.0;
//                                 break;
//                             } else {
//                                 current_char = ch.0;
//                             }
//                         }
//                         if !(current_char != ' ' && next_char == ' ') {
//                             select_cur = true;
//                         }
//                     } else {
//                         if let (Some(cur), Some(next)) = (chars.next(), chars.next()) {
//                             if cur.0 == ' ' && next.0 != ' ' {
//                                 rev = true;
//                             }
//                         }
//                     }
//                     // if on space right before word, dont select that space
//                 });
//             });
//         });
//     });
//     if select_cur {
//         select_current(cx);
//     } else if rev {
//         normal_motion(Motion::Right, None, None, cx);
//         clear_selection(cx);
//     } else {
//         clear_selection(cx);
//     }
//     visual_motion(motion.to_owned(), None, cx);
// }
