use editor::{movement, scroll::Autoscroll};
use ui::WindowContext;

use crate::{motion::Motion, normal::normal_motion, visual::visual_motion, Vim};

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
            // TODO not accurate
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
    // TODO: inner WORD boundries (like . in abc.abc) cause massive problems

    // exceptions from simple "select from next char to delimiter"
    let mut select_cur = false;
    let mut rev = false;
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let mut current_char = ' ';
                    let mut next_char = ' ';

                    // find chars around cursor
                    let mut chars = map.display_chars_at(selection.start);
                    if !selection.reversed {
                        for ch in chars {
                            if ch.1 == selection.head() {
                                next_char = ch.0;
                                break;
                            } else {
                                current_char = ch.0;
                            }
                        }
                        match motion {
                            Motion::NextWordStart { .. } => {
                                if !(current_char == ' ' && next_char != ' ') {
                                    select_cur = true;
                                }
                            }
                            Motion::NextWordEnd { .. } => {
                                if !(current_char != ' ' && next_char == ' ') {
                                    select_cur = true;
                                }
                            }
                            _ => todo!("error"),
                        }
                    } else {
                        if let (Some(cur), Some(next)) = (chars.next(), chars.next()) {
                            if cur.0 == ' ' && next.0 != ' ' {
                                rev = true;
                            }
                        }
                    }
                });
            });
        });
    });
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
