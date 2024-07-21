use std::sync::Arc;

use collections::HashMap;
use editor::Bias;
use gpui::AppContext;
use lazy_static::lazy_static;
use settings::Settings;
use ui::WindowContext;

use crate::{
    normal::normal_replace,
    replace::multi_replace,
    state::{Mode, Operator},
    visual::visual_replace,
    Vim, VimSettings,
};

mod default;

lazy_static! {
    static ref DEFAULT_DIGRAPHS_MAP: HashMap<String, char> = {
        let mut map = HashMap::default();
        for &(a, b, c) in default::DEFAULT_DIGRAPHS {
            let key = format!("{a}{b}");
            map.insert(key, char::from_u32(c).unwrap());
        }
        map
    };
}

fn lookup_digraph(a: char, b: char, cx: &AppContext) -> char {
    let custom_digraphs = &VimSettings::get_global(cx).custom_digraphs;
    let input = [a, b].into_iter().collect::<String>();
    let reversed = [b, a].into_iter().collect::<String>();

    custom_digraphs
        .get(&input)
        .or_else(|| DEFAULT_DIGRAPHS_MAP.get(&input))
        .or_else(|| custom_digraphs.get(&reversed))
        .or_else(|| DEFAULT_DIGRAPHS_MAP.get(&reversed))
        .copied()
        .unwrap_or(b)
}

pub fn insert_digraph(first_char: char, second_char: char, cx: &mut WindowContext) {
    let mapped_char = lookup_digraph(first_char, second_char, &cx);
    let text: Arc<str> = mapped_char.to_string().into();

    Vim::update(cx, |vim, cx| vim.pop_operator(cx));

    match Vim::read(cx).active_operator() {
        Some(Operator::Replace) => match Vim::read(cx).state().mode {
            Mode::Normal => normal_replace(text, cx),
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => visual_replace(text, cx),
            _ => Vim::update(cx, |vim, cx| vim.clear_operator(cx)),
        },
        _ => match Vim::read(cx).state().mode {
            Mode::Insert => multi_insert(text, cx),
            Mode::Replace => multi_replace(text, cx),
            _ => {}
        },
    }
}

fn multi_insert(mapped_str: Arc<str>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            let (display_map, display_selections) = editor.selections.all_adjusted_display(cx);
            let edits = display_selections
                .iter()
                .map(|range| {
                    let start = range.start.to_offset(&display_map, Bias::Right);
                    let end = range.end.to_offset(&display_map, Bias::Left);
                    (start..end, mapped_str.clone())
                })
                .collect::<Vec<_>>();

            editor.buffer().update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });
        });
    });
}

#[cfg(test)]
mod test {
    use collections::HashMap;
    use settings::SettingsStore;

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
        VimSettings,
    };

    #[gpui::test]
    async fn test_digraph_insert_mode(cx: &mut gpui::TestAppContext) {
        let mut cx: NeovimBackedTestContext = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("Hellˇo").await;
        cx.simulate_shared_keystrokes("a ctrl-k o : escape").await;
        cx.shared_state().await.assert_eq("Helloˇö");

        cx.set_shared_state("Hellˇo").await;
        cx.simulate_shared_keystrokes("a ctrl-k : o escape").await;
        cx.shared_state().await.assert_eq("Helloˇö");

        cx.set_shared_state("Hellˇo").await;
        cx.simulate_shared_keystrokes("i ctrl-k o : escape").await;
        cx.shared_state().await.assert_eq("Hellˇöo");
    }

    #[gpui::test]
    async fn test_digraph_replace(cx: &mut gpui::TestAppContext) {
        let mut cx: NeovimBackedTestContext = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("Hellˇo").await;
        cx.simulate_shared_keystrokes("r ctrl-k o :").await;
        cx.shared_state().await.assert_eq("Hellˇö");
    }

    #[gpui::test]
    async fn test_digraph_replace_mode(cx: &mut gpui::TestAppContext) {
        let mut cx: NeovimBackedTestContext = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇHello").await;
        cx.simulate_shared_keystrokes(
            "shift-r ctrl-k a ' ctrl-k e ` ctrl-k i : ctrl-k o ~ ctrl-k u - escape",
        )
        .await;
        cx.shared_state().await.assert_eq("áèïõˇū");
    }

    #[gpui::test]
    async fn test_digraph_custom(cx: &mut gpui::TestAppContext) {
        let mut cx: VimTestContext = VimTestContext::new(cx, true).await;

        cx.update_global(|store: &mut SettingsStore, cx| {
            store.update_user_settings::<VimSettings>(cx, |s| {
                let mut custom_digraphs = HashMap::default();
                custom_digraphs.insert("|-".into(), '⊢');
                s.custom_digraphs = Some(custom_digraphs);
            });
        });

        cx.simulate_keystrokes("a ctrl-k | - escape");
        cx.assert_state("ˇ⊢", Mode::Normal);
    }
}
