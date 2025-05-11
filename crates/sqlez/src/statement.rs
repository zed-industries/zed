use std::ffi::{CStr, CString, c_int};
use std::marker::PhantomData;
use std::{ptr, slice, str};

use anyhow::{Context, Result, anyhow, bail};
use libsqlite3_sys::*;

use crate::bindable::{Bind, Column};
use crate::connection::Connection;

pub struct Statement<'a> {
    /// vector of pointers to the raw SQLite statement objects.
    /// it holds the actual prepared statements that will be executed.
    pub raw_statements: Vec<*mut sqlite3_stmt>,
    /// Index of the current statement being executed from the `raw_statements` vector.
    current_statement: usize,
    /// A reference to the database connection.
    /// This is used to execute the statements and check for errors.
    connection: &'a Connection,
    ///Indicates that the `Statement` struct is tied to the lifetime of the SQLite statement
    phantom: PhantomData<sqlite3_stmt>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StepResult {
    Row,
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SqlType {
    Text,
    Integer,
    Blob,
    Float,
    Null,
}

impl<'a> Statement<'a> {
    pub fn prepare<T: AsRef<str>>(connection: &'a Connection, query: T) -> Result<Self> {
        let mut statement = Self {
            raw_statements: Default::default(),
            current_statement: 0,
            connection,
            phantom: PhantomData,
        };
        unsafe {
            let sql = CString::new(query.as_ref()).context("Error creating cstr")?;
            let mut remaining_sql = sql.as_c_str();
            while {
                let remaining_sql_str = remaining_sql
                    .to_str()
                    .context("Parsing remaining sql")?
                    .trim();
                remaining_sql_str != ";" && !remaining_sql_str.is_empty()
            } {
                let mut raw_statement = ptr::null_mut::<sqlite3_stmt>();
                let mut remaining_sql_ptr = ptr::null();
                sqlite3_prepare_v2(
                    connection.sqlite3,
                    remaining_sql.as_ptr(),
                    -1,
                    &mut raw_statement,
                    &mut remaining_sql_ptr,
                );

                connection.last_error().with_context(|| {
                    format!("Prepare call failed for query:\n{}", query.as_ref())
                })?;

                remaining_sql = CStr::from_ptr(remaining_sql_ptr);
                statement.raw_statements.push(raw_statement);

                if !connection.can_write() && sqlite3_stmt_readonly(raw_statement) == 0 {
                    let sql = CStr::from_ptr(sqlite3_sql(raw_statement));

                    bail!(
                        "Write statement prepared with connection that is not write capable. SQL:\n{} ",
                        sql.to_str()?
                    )
                }
            }
        }

        Ok(statement)
    }

    fn current_statement(&self) -> *mut sqlite3_stmt {
        *self.raw_statements.get(self.current_statement).unwrap()
    }

    pub fn reset(&mut self) {
        unsafe {
            for raw_statement in self.raw_statements.iter() {
                sqlite3_reset(*raw_statement);
            }
        }
        self.current_statement = 0;
    }

    pub fn parameter_count(&self) -> i32 {
        unsafe {
            self.raw_statements
                .iter()
                .map(|raw_statement| sqlite3_bind_parameter_count(*raw_statement))
                .max()
                .unwrap_or(0)
        }
    }

    fn bind_index_with(&self, index: i32, bind: impl Fn(&*mut sqlite3_stmt)) -> Result<()> {
        let mut any_succeed = false;
        unsafe {
            for raw_statement in self.raw_statements.iter() {
                if index <= sqlite3_bind_parameter_count(*raw_statement) {
                    bind(raw_statement);
                    self.connection
                        .last_error()
                        .with_context(|| format!("Failed to bind value at index {index}"))?;
                    any_succeed = true;
                } else {
                    continue;
                }
            }
        }
        if any_succeed {
            Ok(())
        } else {
            Err(anyhow!("Failed to bind parameters"))
        }
    }

    pub fn bind_blob(&self, index: i32, blob: &[u8]) -> Result<()> {
        let index = index as c_int;
        let blob_pointer = blob.as_ptr() as *const _;
        let len = blob.len() as c_int;

        self.bind_index_with(index, |raw_statement| unsafe {
            sqlite3_bind_blob(*raw_statement, index, blob_pointer, len, SQLITE_TRANSIENT());
        })
    }

    pub fn column_blob(&mut self, index: i32) -> Result<&[u8]> {
        let index = index as c_int;
        let pointer = unsafe { sqlite3_column_blob(self.current_statement(), index) };

        self.connection
            .last_error()
            .with_context(|| format!("Failed to read blob at index {index}"))?;
        if pointer.is_null() {
            return Ok(&[]);
        }
        let len = unsafe { sqlite3_column_bytes(self.current_statement(), index) as usize };
        self.connection
            .last_error()
            .with_context(|| format!("Failed to read length of blob at index {index}"))?;

        unsafe { Ok(slice::from_raw_parts(pointer as *const u8, len)) }
    }

    pub fn bind_double(&self, index: i32, double: f64) -> Result<()> {
        let index = index as c_int;

        self.bind_index_with(index, |raw_statement| unsafe {
            sqlite3_bind_double(*raw_statement, index, double);
        })
    }

    pub fn column_double(&self, index: i32) -> Result<f64> {
        let index = index as c_int;
        let result = unsafe { sqlite3_column_double(self.current_statement(), index) };
        self.connection
            .last_error()
            .with_context(|| format!("Failed to read double at index {index}"))?;
        Ok(result)
    }

    pub fn bind_int(&self, index: i32, int: i32) -> Result<()> {
        let index = index as c_int;
        self.bind_index_with(index, |raw_statement| unsafe {
            sqlite3_bind_int(*raw_statement, index, int);
        })
    }

    pub fn column_int(&self, index: i32) -> Result<i32> {
        let index = index as c_int;
        let result = unsafe { sqlite3_column_int(self.current_statement(), index) };
        self.connection
            .last_error()
            .with_context(|| format!("Failed to read int at index {index}"))?;
        Ok(result)
    }

    pub fn bind_int64(&self, index: i32, int: i64) -> Result<()> {
        let index = index as c_int;
        self.bind_index_with(index, |raw_statement| unsafe {
            sqlite3_bind_int64(*raw_statement, index, int);
        })
    }

    pub fn column_int64(&self, index: i32) -> Result<i64> {
        let index = index as c_int;
        let result = unsafe { sqlite3_column_int64(self.current_statement(), index) };
        self.connection
            .last_error()
            .with_context(|| format!("Failed to read i64 at index {index}"))?;
        Ok(result)
    }

    pub fn bind_null(&self, index: i32) -> Result<()> {
        let index = index as c_int;
        self.bind_index_with(index, |raw_statement| unsafe {
            sqlite3_bind_null(*raw_statement, index);
        })
    }

    pub fn bind_text(&self, index: i32, text: &str) -> Result<()> {
        let index = index as c_int;
        let text_pointer = text.as_ptr() as *const _;
        let len = text.len() as c_int;

        self.bind_index_with(index, |raw_statement| unsafe {
            sqlite3_bind_text(*raw_statement, index, text_pointer, len, SQLITE_TRANSIENT());
        })
    }

    pub fn column_text(&mut self, index: i32) -> Result<&str> {
        let index = index as c_int;
        let pointer = unsafe { sqlite3_column_text(self.current_statement(), index) };

        self.connection
            .last_error()
            .with_context(|| format!("Failed to read text from column {index}"))?;
        if pointer.is_null() {
            return Ok("");
        }
        let len = unsafe { sqlite3_column_bytes(self.current_statement(), index) as usize };
        self.connection
            .last_error()
            .with_context(|| format!("Failed to read text length at {index}"))?;

        let slice = unsafe { slice::from_raw_parts(pointer, len) };
        Ok(str::from_utf8(slice)?)
    }

    pub fn bind<T: Bind>(&self, value: &T, index: i32) -> Result<i32> {
        debug_assert!(index > 0);
        value.bind(self, index)
    }

    pub fn column<T: Column>(&mut self) -> Result<T> {
        Ok(T::column(self, 0)?.0)
    }

    pub fn column_type(&mut self, index: i32) -> Result<SqlType> {
        let result = unsafe { sqlite3_column_type(self.current_statement(), index) };
        self.connection.last_error()?;
        match result {
            SQLITE_INTEGER => Ok(SqlType::Integer),
            SQLITE_FLOAT => Ok(SqlType::Float),
            SQLITE_TEXT => Ok(SqlType::Text),
            SQLITE_BLOB => Ok(SqlType::Blob),
            SQLITE_NULL => Ok(SqlType::Null),
            _ => Err(anyhow!("Column type returned was incorrect ")),
        }
    }

    pub fn with_bindings(&mut self, bindings: &impl Bind) -> Result<&mut Self> {
        self.bind(bindings, 1)?;
        Ok(self)
    }

    fn step(&mut self) -> Result<StepResult> {
        unsafe {
            match sqlite3_step(self.current_statement()) {
                SQLITE_ROW => Ok(StepResult::Row),
                SQLITE_DONE => {
                    if self.current_statement >= self.raw_statements.len() - 1 {
                        Ok(StepResult::Done)
                    } else {
                        self.current_statement += 1;
                        self.step()
                    }
                }
                SQLITE_MISUSE => Err(anyhow!("Statement step returned SQLITE_MISUSE")),
                _other_error => {
                    self.connection.last_error()?;
                    unreachable!("Step returned error code and last error failed to catch it");
                }
            }
        }
    }

    pub fn exec(&mut self) -> Result<()> {
        fn logic(this: &mut Statement) -> Result<()> {
            while this.step()? == StepResult::Row {}
            Ok(())
        }
        let result = logic(self);
        self.reset();
        result
    }

    pub fn map<R>(&mut self, callback: impl FnMut(&mut Statement) -> Result<R>) -> Result<Vec<R>> {
        fn logic<R>(
            this: &mut Statement,
            mut callback: impl FnMut(&mut Statement) -> Result<R>,
        ) -> Result<Vec<R>> {
            let mut mapped_rows = Vec::new();
            while this.step()? == StepResult::Row {
                mapped_rows.push(callback(this)?);
            }
            Ok(mapped_rows)
        }

        let result = logic(self, callback);
        self.reset();
        result
    }

    pub fn rows<R: Column>(&mut self) -> Result<Vec<R>> {
        self.map(|s| s.column::<R>())
    }

    pub fn single<R>(&mut self, callback: impl FnOnce(&mut Statement) -> Result<R>) -> Result<R> {
        fn logic<R>(
            this: &mut Statement,
            callback: impl FnOnce(&mut Statement) -> Result<R>,
        ) -> Result<R> {
            println!("{:?}", std::any::type_name::<R>());
            if this.step()? != StepResult::Row {
                return Err(anyhow!("single called with query that returns no rows."));
            }
            let result = callback(this)?;

            if this.step()? != StepResult::Done {
                return Err(anyhow!(
                    "single called with a query that returns more than one row."
                ));
            }

            Ok(result)
        }
        let result = logic(self, callback);
        self.reset();
        result
    }

    pub fn row<R: Column>(&mut self) -> Result<R> {
        self.single(|this| this.column::<R>())
    }

    pub fn maybe<R>(
        &mut self,
        callback: impl FnOnce(&mut Statement) -> Result<R>,
    ) -> Result<Option<R>> {
        fn logic<R>(
            this: &mut Statement,
            callback: impl FnOnce(&mut Statement) -> Result<R>,
        ) -> Result<Option<R>> {
            if this.step().context("Failed on step call")? != StepResult::Row {
                return Ok(None);
            }

            let result = callback(this)
                .map(|r| Some(r))
                .context("Failed to parse row result")?;

            if this.step().context("Second step call")? != StepResult::Done {
                return Err(anyhow!(
                    "maybe called with a query that returns more than one row."
                ));
            }

            Ok(result)
        }
        let result = logic(self, callback);
        self.reset();
        result
    }

    pub fn maybe_row<R: Column>(&mut self) -> Result<Option<R>> {
        self.maybe(|this| this.column::<R>())
    }
}

impl Drop for Statement<'_> {
    fn drop(&mut self) {
        unsafe {
            for raw_statement in self.raw_statements.iter() {
                sqlite3_finalize(*raw_statement);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{
        connection::Connection,
        statement::{Statement, StepResult},
    };

    #[test]
    fn binding_multiple_statements_with_parameter_gaps() {
        let connection =
            Connection::open_memory(Some("binding_multiple_statements_with_parameter_gaps"));

        connection
            .exec(indoc! {"
            CREATE TABLE test (
                col INTEGER
            )"})
            .unwrap()()
        .unwrap();

        let statement = Statement::prepare(
            &connection,
            indoc! {"
                INSERT INTO test(col) VALUES (?3);
                SELECT * FROM test WHERE col = ?1"},
        )
        .unwrap();

        statement
            .bind_int(1, 1)
            .expect("Could not bind parameter to first index");
        statement
            .bind_int(2, 2)
            .expect("Could not bind parameter to second index");
        statement
            .bind_int(3, 3)
            .expect("Could not bind parameter to third index");
    }

    #[test]
    fn blob_round_trips() {
        let connection1 = Connection::open_memory(Some("blob_round_trips"));
        connection1
            .exec(indoc! {"
                CREATE TABLE blobs (
                    data BLOB
                )"})
            .unwrap()()
        .unwrap();

        let blob = &[0, 1, 2, 4, 8, 16, 32, 64];

        let mut write =
            Statement::prepare(&connection1, "INSERT INTO blobs (data) VALUES (?)").unwrap();
        write.bind_blob(1, blob).unwrap();
        assert_eq!(write.step().unwrap(), StepResult::Done);

        // Read the blob from the
        let connection2 = Connection::open_memory(Some("blob_round_trips"));
        let mut read = Statement::prepare(&connection2, "SELECT * FROM blobs").unwrap();
        assert_eq!(read.step().unwrap(), StepResult::Row);
        assert_eq!(read.column_blob(0).unwrap(), blob);
        assert_eq!(read.step().unwrap(), StepResult::Done);

        // Delete the added blob and verify its deleted on the other side
        connection2.exec("DELETE FROM blobs").unwrap()().unwrap();
        let mut read = Statement::prepare(&connection1, "SELECT * FROM blobs").unwrap();
        assert_eq!(read.step().unwrap(), StepResult::Done);
    }

    #[test]
    pub fn maybe_returns_options() {
        let connection = Connection::open_memory(Some("maybe_returns_options"));
        connection
            .exec(indoc! {"
                CREATE TABLE texts (
                    text TEXT
                )"})
            .unwrap()()
        .unwrap();

        assert!(
            connection
                .select_row::<String>("SELECT text FROM texts")
                .unwrap()()
            .unwrap()
            .is_none()
        );

        let text_to_insert = "This is a test";

        connection
            .exec_bound("INSERT INTO texts VALUES (?)")
            .unwrap()(text_to_insert)
        .unwrap();

        assert_eq!(
            connection.select_row("SELECT text FROM texts").unwrap()().unwrap(),
            Some(text_to_insert.to_string())
        );
    }
}
