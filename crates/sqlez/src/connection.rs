use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
};

use anyhow::{anyhow, Result};
use libsqlite3_sys::*;

use crate::statement::Statement;

pub struct Connection {
    pub(crate) sqlite3: *mut sqlite3,
    persistent: bool,
    phantom: PhantomData<sqlite3>,
}
unsafe impl Send for Connection {}

impl Connection {
    fn open(uri: &str, persistent: bool) -> Result<Self> {
        let mut connection = Self {
            sqlite3: 0 as *mut _,
            persistent,
            phantom: PhantomData,
        };

        let flags = SQLITE_OPEN_CREATE | SQLITE_OPEN_NOMUTEX | SQLITE_OPEN_READWRITE;
        unsafe {
            sqlite3_open_v2(
                CString::new(uri)?.as_ptr(),
                &mut connection.sqlite3,
                flags,
                0 as *const _,
            );

            connection.last_error()?;
        }

        Ok(connection)
    }

    /// Attempts to open the database at uri. If it fails, a shared memory db will be opened
    /// instead.
    pub fn open_file(uri: &str) -> Self {
        Self::open(uri, true).unwrap_or_else(|_| Self::open_memory(uri))
    }

    pub fn open_memory(uri: &str) -> Self {
        let in_memory_path = format!("file:{}?mode=memory&cache=shared", uri);
        Self::open(&in_memory_path, false).expect("Could not create fallback in memory db")
    }

    pub fn persistent(&self) -> bool {
        self.persistent
    }

    pub fn exec(&self, query: impl AsRef<str>) -> Result<()> {
        unsafe {
            sqlite3_exec(
                self.sqlite3,
                CString::new(query.as_ref())?.as_ptr(),
                None,
                0 as *mut _,
                0 as *mut _,
            );
            self.last_error()?;
        }
        Ok(())
    }

    pub fn prepare<T: AsRef<str>>(&self, query: T) -> Result<Statement> {
        Statement::prepare(&self, query)
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

    pub(crate) fn last_error(&self) -> Result<()> {
        const NON_ERROR_CODES: &[i32] = &[SQLITE_OK, SQLITE_ROW];
        unsafe {
            let code = sqlite3_errcode(self.sqlite3);
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

            Err(anyhow!(
                "Sqlite call failed with code {} and message: {:?}",
                code as isize,
                message
            ))
        }
    }
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
        let connection = Connection::open_memory("string_round_trips");
        connection
            .exec(indoc! {"
            CREATE TABLE text (
                text TEXT
            );"})
            .unwrap();

        let text = "Some test text";

        connection
            .prepare("INSERT INTO text (text) VALUES (?);")
            .unwrap()
            .bound(text)
            .unwrap()
            .run()
            .unwrap();

        assert_eq!(
            &connection
                .prepare("SELECT text FROM text;")
                .unwrap()
                .row::<String>()
                .unwrap(),
            text
        );

        Ok(())
    }

    #[test]
    fn tuple_round_trips() {
        let connection = Connection::open_memory("tuple_round_trips");
        connection
            .exec(indoc! {"
                CREATE TABLE test (
                    text TEXT,
                    integer INTEGER,
                    blob BLOB
                );"})
            .unwrap();

        let tuple1 = ("test".to_string(), 64, vec![0, 1, 2, 4, 8, 16, 32, 64]);
        let tuple2 = ("test2".to_string(), 32, vec![64, 32, 16, 8, 4, 2, 1, 0]);

        let mut insert = connection
            .prepare("INSERT INTO test (text, integer, blob) VALUES (?, ?, ?)")
            .unwrap();

        insert.bound(tuple1.clone()).unwrap().run().unwrap();
        insert.bound(tuple2.clone()).unwrap().run().unwrap();

        assert_eq!(
            connection
                .prepare("SELECT * FROM test")
                .unwrap()
                .rows::<(String, usize, Vec<u8>)>()
                .unwrap(),
            vec![tuple1, tuple2]
        );
    }

    #[test]
    fn backup_works() {
        let connection1 = Connection::open_memory("backup_works");
        connection1
            .exec(indoc! {"
                CREATE TABLE blobs (
                    data BLOB
                );"})
            .unwrap();
        let blob = &[0, 1, 2, 4, 8, 16, 32, 64];
        let mut write = connection1
            .prepare("INSERT INTO blobs (data) VALUES (?);")
            .unwrap();
        write.bind_blob(1, blob).unwrap();
        write.run().unwrap();

        // Backup connection1 to connection2
        let connection2 = Connection::open_memory("backup_works_other");
        connection1.backup_main(&connection2).unwrap();

        // Delete the added blob and verify its deleted on the other side
        let read_blobs = connection1
            .prepare("SELECT * FROM blobs;")
            .unwrap()
            .rows::<Vec<u8>>()
            .unwrap();
        assert_eq!(read_blobs, vec![blob]);
    }
}
