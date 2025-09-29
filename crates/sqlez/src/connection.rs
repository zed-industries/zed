use std::{
    cell::RefCell,
    ffi::{CStr, CString},
    marker::PhantomData,
    path::Path,
    ptr,
};

use anyhow::Result;
use libsqlite3_sys::*;

pub struct Connection {
    pub(crate) sqlite3: *mut sqlite3,
    persistent: bool,
    pub(crate) write: RefCell<bool>,
    _sqlite: PhantomData<sqlite3>,
}
unsafe impl Send for Connection {}

impl Connection {
    pub(crate) fn open(uri: &str, persistent: bool) -> Result<Self> {
        let mut connection = Self {
            sqlite3: ptr::null_mut(),
            persistent,
            write: RefCell::new(true),
            _sqlite: PhantomData,
        };

        let flags = SQLITE_OPEN_CREATE | SQLITE_OPEN_NOMUTEX | SQLITE_OPEN_READWRITE;
        unsafe {
            sqlite3_open_v2(
                CString::new(uri)?.as_ptr(),
                &mut connection.sqlite3,
                flags,
                ptr::null(),
            );

            // Turn on extended error codes
            sqlite3_extended_result_codes(connection.sqlite3, 1);

            connection.last_error()?;
        }

        Ok(connection)
    }

    /// Attempts to open the database at uri. If it fails, a shared memory db will be opened
    /// instead.
    pub fn open_file(uri: &str) -> Self {
        Self::open(uri, true).unwrap_or_else(|_| Self::open_memory(Some(uri)))
    }

    pub fn open_memory(uri: Option<&str>) -> Self {
        let in_memory_path = if let Some(uri) = uri {
            format!("file:{}?mode=memory&cache=shared", uri)
        } else {
            ":memory:".to_string()
        };

        Self::open(&in_memory_path, false).expect("Could not create fallback in memory db")
    }

    pub fn persistent(&self) -> bool {
        self.persistent
    }

    pub fn can_write(&self) -> bool {
        *self.write.borrow()
    }

    pub fn backup_main(&self, destination: &Connection) -> Result<()> {
        unsafe {
            let backup = sqlite3_backup_init(
                destination.sqlite3,
                CString::new("main")?.as_ptr(),
                self.sqlite3,
                CString::new("main")?.as_ptr(),
            );
            sqlite3_backup_step(backup, -1);
            sqlite3_backup_finish(backup);
            destination.last_error()
        }
    }

    pub fn backup_main_to(&self, destination: impl AsRef<Path>) -> Result<()> {
        let destination = Self::open_file(destination.as_ref().to_string_lossy().as_ref());
        self.backup_main(&destination)
    }

    pub fn sql_has_syntax_error(&self, sql: &str) -> Option<(String, usize)> {
        let sql = CString::new(sql).unwrap();
        let mut remaining_sql = sql.as_c_str();
        let sql_start = remaining_sql.as_ptr();

        let mut alter_table = None;
        while {
            let remaining_sql_str = remaining_sql.to_str().unwrap().trim();
            let any_remaining_sql = remaining_sql_str != ";" && !remaining_sql_str.is_empty();
            if any_remaining_sql {
                alter_table = parse_alter_table(remaining_sql_str);
            }
            any_remaining_sql
        } {
            let mut raw_statement = ptr::null_mut::<sqlite3_stmt>();
            let mut remaining_sql_ptr = ptr::null();

            let (res, offset, message, _conn) = if let Some((table_to_alter, column)) = alter_table
            {
                // ALTER TABLE is a weird statement. When preparing the statement the table's
                // existence is checked *before* syntax checking any other part of the statement.
                // Therefore, we need to make sure that the table has been created before calling
                // prepare. As we don't want to trash whatever database this is connected to, we
                // create a new in-memory DB to test.

                let temp_connection = Connection::open_memory(None);
                //This should always succeed, if it doesn't then you really should know about it
                temp_connection
                    .exec(&format!("CREATE TABLE {table_to_alter}({column})"))
                    .unwrap()()
                .unwrap();

                unsafe {
                    sqlite3_prepare_v2(
                        temp_connection.sqlite3,
                        remaining_sql.as_ptr(),
                        -1,
                        &mut raw_statement,
                        &mut remaining_sql_ptr,
                    )
                };

                #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                let offset = unsafe { sqlite3_error_offset(temp_connection.sqlite3) };

                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                let offset = 0;

                unsafe {
                    (
                        sqlite3_errcode(temp_connection.sqlite3),
                        offset,
                        sqlite3_errmsg(temp_connection.sqlite3),
                        Some(temp_connection),
                    )
                }
            } else {
                unsafe {
                    sqlite3_prepare_v2(
                        self.sqlite3,
                        remaining_sql.as_ptr(),
                        -1,
                        &mut raw_statement,
                        &mut remaining_sql_ptr,
                    )
                };

                #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                let offset = unsafe { sqlite3_error_offset(self.sqlite3) };

                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                let offset = 0;

                unsafe {
                    (
                        sqlite3_errcode(self.sqlite3),
                        offset,
                        sqlite3_errmsg(self.sqlite3),
                        None,
                    )
                }
            };

            unsafe { sqlite3_finalize(raw_statement) };

            if res == 1 && offset >= 0 {
                let sub_statement_correction = remaining_sql.as_ptr() as usize - sql_start as usize;
                let err_msg = String::from_utf8_lossy(unsafe {
                    CStr::from_ptr(message as *const _).to_bytes()
                })
                .into_owned();

                return Some((err_msg, offset as usize + sub_statement_correction));
            }
            remaining_sql = unsafe { CStr::from_ptr(remaining_sql_ptr) };
            alter_table = None;
        }
        None
    }

    pub(crate) fn last_error(&self) -> Result<()> {
        unsafe {
            let code = sqlite3_errcode(self.sqlite3);
            const NON_ERROR_CODES: &[i32] = &[SQLITE_OK, SQLITE_ROW];
            if NON_ERROR_CODES.contains(&code) {
                return Ok(());
            }

            let message = sqlite3_errmsg(self.sqlite3);
            let message = if message.is_null() {
                None
            } else {
                Some(
                    String::from_utf8_lossy(CStr::from_ptr(message as *const _).to_bytes())
                        .into_owned(),
                )
            };

            anyhow::bail!("Sqlite call failed with code {code} and message: {message:?}")
        }
    }

    pub(crate) fn with_write<T>(&self, callback: impl FnOnce(&Connection) -> T) -> T {
        *self.write.borrow_mut() = true;
        let result = callback(self);
        *self.write.borrow_mut() = false;
        result
    }
}

fn parse_alter_table(remaining_sql_str: &str) -> Option<(String, String)> {
    let remaining_sql_str = remaining_sql_str.to_lowercase();
    if remaining_sql_str.starts_with("alter")
        && let Some(table_offset) = remaining_sql_str.find("table")
    {
        let after_table_offset = table_offset + "table".len();
        let table_to_alter = remaining_sql_str
            .chars()
            .skip(after_table_offset)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        if !table_to_alter.is_empty() {
            let column_name = if let Some(rename_offset) = remaining_sql_str.find("rename column") {
                let after_rename_offset = rename_offset + "rename column".len();
                remaining_sql_str
                    .chars()
                    .skip(after_rename_offset)
                    .skip_while(|c| c.is_whitespace())
                    .take_while(|c| !c.is_whitespace())
                    .collect::<String>()
            } else if let Some(drop_offset) = remaining_sql_str.find("drop column") {
                let after_drop_offset = drop_offset + "drop column".len();
                remaining_sql_str
                    .chars()
                    .skip(after_drop_offset)
                    .skip_while(|c| c.is_whitespace())
                    .take_while(|c| !c.is_whitespace())
                    .collect::<String>()
            } else {
                "__place_holder_column_for_syntax_checking".to_string()
            };
            return Some((table_to_alter, column_name));
        }
    }
    None
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { sqlite3_close(self.sqlite3) };
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use crate::connection::Connection;

    #[test]
    fn string_round_trips() -> Result<()> {
        let connection = Connection::open_memory(Some("string_round_trips"));
        connection
            .exec(indoc! {"
            CREATE TABLE text (
                text TEXT
            );"})
            .unwrap()()
        .unwrap();

        let text = "Some test text";

        connection
            .exec_bound("INSERT INTO text (text) VALUES (?);")
            .unwrap()(text)
        .unwrap();

        assert_eq!(
            connection.select_row("SELECT text FROM text;").unwrap()().unwrap(),
            Some(text.to_string())
        );

        Ok(())
    }

    #[test]
    fn tuple_round_trips() {
        let connection = Connection::open_memory(Some("tuple_round_trips"));
        connection
            .exec(indoc! {"
                CREATE TABLE test (
                    text TEXT,
                    integer INTEGER,
                    blob BLOB
                );"})
            .unwrap()()
        .unwrap();

        let tuple1 = ("test".to_string(), 64, vec![0, 1, 2, 4, 8, 16, 32, 64]);
        let tuple2 = ("test2".to_string(), 32, vec![64, 32, 16, 8, 4, 2, 1, 0]);

        let mut insert = connection
            .exec_bound::<(String, usize, Vec<u8>)>(
                "INSERT INTO test (text, integer, blob) VALUES (?, ?, ?)",
            )
            .unwrap();

        insert(tuple1.clone()).unwrap();
        insert(tuple2.clone()).unwrap();

        assert_eq!(
            connection
                .select::<(String, usize, Vec<u8>)>("SELECT * FROM test")
                .unwrap()()
            .unwrap(),
            vec![tuple1, tuple2]
        );
    }

    #[test]
    fn bool_round_trips() {
        let connection = Connection::open_memory(Some("bool_round_trips"));
        connection
            .exec(indoc! {"
                CREATE TABLE bools (
                    t INTEGER,
                    f INTEGER
                );"})
            .unwrap()()
        .unwrap();

        connection
            .exec_bound("INSERT INTO bools(t, f) VALUES (?, ?)")
            .unwrap()((true, false))
        .unwrap();

        assert_eq!(
            connection
                .select_row::<(bool, bool)>("SELECT * FROM bools;")
                .unwrap()()
            .unwrap(),
            Some((true, false))
        );
    }

    #[test]
    fn backup_works() {
        let connection1 = Connection::open_memory(Some("backup_works"));
        connection1
            .exec(indoc! {"
                CREATE TABLE blobs (
                    data BLOB
                );"})
            .unwrap()()
        .unwrap();
        let blob = vec![0, 1, 2, 4, 8, 16, 32, 64];
        connection1
            .exec_bound::<Vec<u8>>("INSERT INTO blobs (data) VALUES (?);")
            .unwrap()(blob.clone())
        .unwrap();

        // Backup connection1 to connection2
        let connection2 = Connection::open_memory(Some("backup_works_other"));
        connection1.backup_main(&connection2).unwrap();

        // Delete the added blob and verify its deleted on the other side
        let read_blobs = connection1
            .select::<Vec<u8>>("SELECT * FROM blobs;")
            .unwrap()()
        .unwrap();
        assert_eq!(read_blobs, vec![blob]);
    }

    #[test]
    fn multi_step_statement_works() {
        let connection = Connection::open_memory(Some("multi_step_statement_works"));

        connection
            .exec(indoc! {"
                CREATE TABLE test (
                    col INTEGER
                )"})
            .unwrap()()
        .unwrap();

        connection
            .exec(indoc! {"
            INSERT INTO test(col) VALUES (2)"})
            .unwrap()()
        .unwrap();

        assert_eq!(
            connection
                .select_row::<usize>("SELECT * FROM test")
                .unwrap()()
            .unwrap(),
            Some(2)
        );
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    #[test]
    fn test_sql_has_syntax_errors() {
        let connection = Connection::open_memory(Some("test_sql_has_syntax_errors"));
        let first_stmt =
            "CREATE TABLE kv_store(key TEXT PRIMARY KEY, value TEXT NOT NULL) STRICT ;";
        let second_stmt = "SELECT FROM";

        let second_offset = connection.sql_has_syntax_error(second_stmt).unwrap().1;

        let res = connection
            .sql_has_syntax_error(&format!("{}\n{}", first_stmt, second_stmt))
            .map(|(_, offset)| offset);

        assert_eq!(res, Some(first_stmt.len() + second_offset + 1));
    }

    #[test]
    fn test_alter_table_syntax() {
        let connection = Connection::open_memory(Some("test_alter_table_syntax"));

        assert!(
            connection
                .sql_has_syntax_error("ALTER TABLE test ADD x TEXT")
                .is_none()
        );

        assert!(
            connection
                .sql_has_syntax_error("ALTER TABLE test AAD x TEXT")
                .is_some()
        );
    }
}
