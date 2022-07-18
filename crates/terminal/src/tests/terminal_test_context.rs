use std::time::Duration;

use alacritty_terminal::term::SizeInfo;
use gpui::{AppContext, ModelHandle, ReadModelWith, TestAppContext};
use itertools::Itertools;

use crate::{
    connection::TerminalConnection, DEBUG_CELL_WIDTH, DEBUG_LINE_HEIGHT, DEBUG_TERMINAL_HEIGHT,
    DEBUG_TERMINAL_WIDTH,
};

pub struct TerminalTestContext<'a> {
    pub cx: &'a mut TestAppContext,
    pub connection: ModelHandle<TerminalConnection>,
}

impl<'a> TerminalTestContext<'a> {
    pub fn new(cx: &'a mut TestAppContext) -> Self {
        cx.set_condition_duration(Some(Duration::from_secs(5)));

        let size_info = SizeInfo::new(
            DEBUG_TERMINAL_WIDTH,
            DEBUG_TERMINAL_HEIGHT,
            DEBUG_CELL_WIDTH,
            DEBUG_LINE_HEIGHT,
            0.,
            0.,
            false,
        );

        let connection =
            cx.add_model(|cx| TerminalConnection::new(None, None, None, size_info, cx));

        TerminalTestContext { cx, connection }
    }

    pub async fn execute_and_wait<F>(&mut self, command: &str, f: F) -> String
    where
        F: Fn(String, &AppContext) -> bool,
    {
        let command = command.to_string();
        self.connection.update(self.cx, |connection, _| {
            connection.get_terminal().unwrap().write_to_pty(command);
            connection
                .get_terminal()
                .unwrap()
                .write_to_pty("\r".to_string());
        });

        self.connection
            .condition(self.cx, |conn, cx| {
                let content = Self::grid_as_str(conn);
                f(content, cx)
            })
            .await;

        self.cx
            .read_model_with(&self.connection, &mut |conn, _: &AppContext| {
                Self::grid_as_str(conn)
            })
    }

    fn grid_as_str(connection: &TerminalConnection) -> String {
        let (grid_iterator, _) = connection.get_terminal().unwrap().renderable_content();
        let lines = grid_iterator.display_iter.group_by(|i| i.point.line.0);
        lines
            .into_iter()
            .map(|(_, line)| line.map(|i| i.c).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<'a> Drop for TerminalTestContext<'a> {
    fn drop(&mut self) {
        self.cx.set_condition_duration(None);
    }
}
