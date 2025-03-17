use anyhow::Result;
use db::{
    define_connection, query,
    sqlez::{bindable::Column, statement::Statement},
    sqlez_macros::sql,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct SerializedCommand {
    pub(crate) command_name: String,
    pub(crate) invocations: u16,
}

impl Column for SerializedCommand {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (command_name, next_index): (String, i32) = Column::column(statement, start_index)?;
        let (invocations, next_index): (u16, i32) = Column::column(statement, next_index)?;

        let command = Self {
            command_name,
            invocations,
        };
        Ok((command, next_index))
    }
}

define_connection!(pub static ref COMMAND_PALETTE_HISTORY: CommandPaletteDB<()> =
    &[sql!(
        CREATE TABLE IF NOT EXISTS command_palette_history(
            command_name TEXT PRIMARY KEY,
            invocations INTEGER DEFAULT 0,
            last_invoked TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL
        ) STRICT;
    )];
);

impl CommandPaletteDB {
    pub async fn write_command(&self, command_name: String) -> Result<()> {
        self.write_command_internal(command_name, u16::MAX).await
    }
    query! {
        pub fn read_command_history(command: &str) -> Result<Option<SerializedCommand>> {
            SELECT command_name, invocations FROM command_palette_history WHERE command_name=(?)
        }
    }
    query! {
        async fn write_command_internal(command_name: String, max_invocations: u16) -> Result<()> {
            // Upsert
            INSERT INTO command_palette_history(command_name, invocations) VALUES  ((?), 1)
            ON CONFLICT DO
            UPDATE SET invocations = MIN(invocations + 1, (?))
        }
    }

    query! {
        pub fn list_commands_used() -> Result<Vec<SerializedCommand>> {
            SELECT command_name, invocations FROM command_palette_history ORDER BY invocations DESC
        }
    }
}

#[cfg(test)]
mod tests {
    use db::sqlez_macros::sql;

    use crate::persistence::CommandPaletteDB;

    #[gpui::test]
    async fn test_saves_hitcount_with_last_used() {
        let db = CommandPaletteDB(db::open_test_db("test_saves_hitcount_with_last_used").await);

        let retrieved_cmd = db.read_command_history("editor: backspace").unwrap();

        assert!(retrieved_cmd.is_none());

        db.write_command("editor: backspace".to_string())
            .await
            .unwrap();

        let used_command = db.read_command_history("editor: backspace").unwrap();

        assert!(used_command.is_some());
        assert_eq!(used_command.expect("is some").invocations, 1);

        db.write_command("editor: backspace".to_string())
            .await
            .unwrap();

        let repeated_command = db.read_command_history("editor: backspace").unwrap();

        assert!(repeated_command.is_some());
        assert_eq!(repeated_command.expect("is some").invocations, 2);
    }

    #[gpui::test]
    async fn test_lists_ordered_by_hitcount() {
        let db = CommandPaletteDB(db::open_test_db("test_lists_ordered_by_hitcount").await);

        let empty_commands = db.list_commands_used();
        assert!(empty_commands.is_ok());
        assert_eq!(empty_commands.expect("is ok").len(), 0);

        db.write_command("go to line: toggle".to_string())
            .await
            .unwrap();
        db.write_command("editor: backspace".to_string())
            .await
            .unwrap();
        db.write_command("editor: backspace".to_string())
            .await
            .unwrap();

        let commands = db.list_commands_used();

        assert!(commands.is_ok());
        let commands = commands.expect("is ok");
        assert_eq!(commands.len(), 2);
        assert_eq!(commands.as_slice()[0].command_name, "editor: backspace");
        assert_eq!(commands.as_slice()[0].invocations, 2);
        assert_eq!(commands.as_slice()[1].command_name, "go to line: toggle");
        assert_eq!(commands.as_slice()[1].invocations, 1);
    }

    #[gpui::test]
    async fn test_handles_max_integer_value() {
        let db = CommandPaletteDB(db::open_test_db("test_handles_max_integer_value").await);
        db.write(|conn| {
            conn.exec_bound(
                sql!(INSERT INTO command_palette_history(command_name, invocations ) VALUES ((?), (?)))
            ).unwrap()(("some-command", u16::MAX)).unwrap();
        }).await;

        db.write_command("some-command".to_string()).await.unwrap();

        let some_command = db.read_command_history("some-command").unwrap();

        assert!(some_command.is_some());
        assert_eq!(some_command.expect("is some").invocations, u16::MAX);
    }
}
