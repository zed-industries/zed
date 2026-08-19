use proptest::prelude::*;

use super::*;

#[derive(Debug, Clone, proptest_derive::Arbitrary)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, proptest_derive::Arbitrary)]
pub enum TestAction {
    #[proptest(weight = 4)]
    Type(String),
    Backspace {
        #[proptest(strategy = "1usize..100")]
        count: usize,
    },
    Move {
        #[proptest(strategy = "1usize..100")]
        count: usize,
        direction: Direction,
    },
}

impl Editor {
    pub fn apply_test_action(
        &mut self,
        action: &TestAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match action {
            TestAction::Type(text) => self.insert(&text, window, cx),
            TestAction::Backspace { count } => {
                for _ in 0..*count {
                    self.delete(&Default::default(), window, cx);
                }
            }
            TestAction::Move { count, direction } => {
                for _ in 0..*count {
                    match direction {
                        Direction::Up => self.move_up(&Default::default(), window, cx),
                        Direction::Down => self.move_down(&Default::default(), window, cx),
                        Direction::Left => self.move_left(&Default::default(), window, cx),
                        Direction::Right => self.move_right(&Default::default(), window, cx),
                    }
                }
            }
        }
    }
}

fn test_actions() -> impl Strategy<Value = Vec<TestAction>> {
    proptest::collection::vec(any::<TestAction>(), 1..10)
}

#[gpui::property_test(config = ProptestConfig {cases: 100, ..Default::default()})]
fn editor_property_test(
    cx: &mut TestAppContext,
    #[strategy = test_actions()] actions: Vec<TestAction>,
) {
    init_test(cx, |_| {});

    let group_interval = Duration::from_millis(1);

    let buffer = cx.new(|cx| {
        let mut buf = language::Buffer::local("123456", cx);
        buf.set_group_interval(group_interval);
        buf
    });

    let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
    let editor = cx.add_window(|window, cx| build_editor(buffer.clone(), window, cx));

    editor
        .update(cx, |editor, window, cx| {
            for action in actions {
                editor.apply_test_action(&action, window, cx);
            }
        })
        .unwrap();
}
