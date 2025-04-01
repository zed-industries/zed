use anyhow::Result;
use db::{
    define_connection, query,
    sqlez::{bindable::Column, statement::Statement},
    sqlez_macros::sql,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct SerializedCommandInvocation {
    pub(crate) command_name: String,
    pub(crate) user_query: String,
    pub(crate) last_invoked: OffsetDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub(crate) struct SerializedCommandUsage {
    pub(crate) command_name: String,
    pub(crate) invocations: u16,
    pub(crate) last_invoked: OffsetDateTime,
}

impl Column for SerializedCommandUsage {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (command_name, next_index): (String, i32) = Column::column(statement, start_index)?;
        let (invocations, next_index): (u16, i32) = Column::column(statement, next_index)?;
        let (last_invoked_raw, next_index): (i64, i32) = Column::column(statement, next_index)?;

        let usage = Self {
            command_name,
            invocations,
            last_invoked: OffsetDateTime::from_unix_timestamp(last_invoked_raw)?,
        };
        Ok((usage, next_index))
    }
}

impl Column for SerializedCommandInvocation {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (command_name, next_index): (String, i32) = Column::column(statement, start_index)?;
        let (user_query, next_index): (String, i32) = Column::column(statement, next_index)?;
        let (last_invoked_raw, next_index): (i64, i32) = Column::column(statement, next_index)?;
        let command_invocation = Self {
            command_name,
            user_query,
            last_invoked: OffsetDateTime::from_unix_timestamp(last_invoked_raw)?,
        };
        Ok((command_invocation, next_index))
    }
}

define_connection!(pub static ref COMMAND_PALETTE_HISTORY: CommandPaletteDB<()> =
    &[sql!(
        CREATE TABLE IF NOT EXISTS command_invocations(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            command_name TEXT NOT NULL,
            user_query TEXT NOT NULL,
            last_invoked INTEGER DEFAULT (unixepoch())  NOT NULL
        ) STRICT;
    )];
);

impl CommandPaletteDB {
    pub async fn write_command_invocation(
        &self,
        command_name: impl Into<String>,
        user_query: impl Into<String>,
    ) -> Result<()> {
        self.write_command_invocation_internal(command_name.into(), user_query.into())
            .await
    }

    query! {
        pub fn get_last_invoked(command: &str) -> Result<Option<SerializedCommandInvocation>> {
            SELECT
            command_name,
            user_query,
            last_invoked FROM command_invocations
            WHERE command_name=(?)
            ORDER BY last_invoked DESC
            LIMIT 1
        }
    }

    query! {
        pub fn get_command_usage(command: &str) -> Result<Option<SerializedCommandUsage>> {
            SELECT command_name, COUNT(1), MAX(last_invoked)
            FROM command_invocations
            WHERE command_name=(?)
            GROUP BY command_name
        }
    }

    query! {
        async fn write_command_invocation_internal(command_name: String, user_query: String) -> Result<()> {
            INSERT INTO command_invocations (command_name, user_query) VALUES ((?), (?));
            DELETE FROM command_invocations WHERE id IN (SELECT MIN(id) FROM command_invocations HAVING COUNT(1) > 1000);
        }
    }

    query! {
        pub fn list_commands_used() -> Result<Vec<SerializedCommandUsage>> {
            SELECT command_name, COUNT(1), MAX(last_invoked)
            FROM command_invocations
            GROUP BY command_name
            ORDER BY COUNT(1) DESC
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::persistence::{CommandPaletteDB, SerializedCommandUsage};

    #[gpui::test]
    async fn test_saves_and_retrieves_command_invocation() {
        let db =
            CommandPaletteDB(db::open_test_db("test_saves_and_retrieves_command_invocation").await);

        let retrieved_cmd = db.get_last_invoked("editor: backspace").unwrap();

        assert!(retrieved_cmd.is_none());

        db.write_command_invocation("editor: backspace", "")
            .await
            .unwrap();

        let retrieved_cmd = db.get_last_invoked("editor: backspace").unwrap();

        assert!(retrieved_cmd.is_some());
        let retrieved_cmd = retrieved_cmd.expect("is some");
        assert_eq!(retrieved_cmd.command_name, "editor: backspace".to_string());
        assert_eq!(retrieved_cmd.user_query, "".to_string());
    }

    #[gpui::test]
    async fn test_gets_usage_history() {
        let db = CommandPaletteDB(db::open_test_db("test_gets_usage_history").await);
        db.write_command_invocation("go to line: toggle", "200")
            .await
            .unwrap();
        db.write_command_invocation("go to line: toggle", "201")
            .await
            .unwrap();

        let retrieved_cmd = db.get_last_invoked("go to line: toggle").unwrap();

        assert!(retrieved_cmd.is_some());
        let retrieved_cmd = retrieved_cmd.expect("is some");

        let command_usage = db.get_command_usage("go to line: toggle").unwrap();

        assert!(command_usage.is_some());
        let command_usage: SerializedCommandUsage = command_usage.expect("is some");

        assert_eq!(command_usage.command_name, "go to line: toggle");
        assert_eq!(command_usage.invocations, 2);
        assert_eq!(command_usage.last_invoked, retrieved_cmd.last_invoked);
    }

    #[gpui::test]
    async fn test_lists_ordered_by_usage() {
        let db = CommandPaletteDB(db::open_test_db("test_lists_ordered_by_usage").await);

        let empty_commands = db.list_commands_used();
        match &empty_commands {
            Ok(_) => (),
            Err(e) => println!("Error: {:?}", e),
        }
        assert!(empty_commands.is_ok());
        assert_eq!(empty_commands.expect("is ok").len(), 0);

        db.write_command_invocation("go to line: toggle", "200")
            .await
            .unwrap();
        db.write_command_invocation("editor: backspace", "")
            .await
            .unwrap();
        db.write_command_invocation("editor: backspace", "")
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
    async fn test_handles_max_invocation_entries() {
        let db = CommandPaletteDB(db::open_test_db("test_handles_max_invocation_entries").await);

        for i in 1..=1001 {
            db.write_command_invocation("some-command", &i.to_string())
                .await
                .unwrap();
        }
        let some_command = db.get_command_usage("some-command").unwrap();

        assert!(some_command.is_some());
        assert_eq!(some_command.expect("is some").invocations, 1000);
    }
}
