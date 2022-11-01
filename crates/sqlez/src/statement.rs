use std::ffi::{c_int, CString};
use std::marker::PhantomData;
use std::{slice, str};

use anyhow::{anyhow, Context, Result};
use libsqlite3_sys::*;

use crate::bindable::{Bind, Column};
use crate::connection::Connection;

pub struct Statement<'a> {
    raw_statement: *mut sqlite3_stmt,
    connection: &'a Connection,
    phantom: PhantomData<sqlite3_stmt>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StepResult {
    Row,
    Done,
    Misuse,
    Other(i32),
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
            raw_statement: 0 as *mut _,
            connection,
            phantom: PhantomData,
        };

        unsafe {
            sqlite3_prepare_v2(
                connection.sqlite3,
                CString::new(query.as_ref())?.as_ptr(),
                -1,
                &mut statement.raw_statement,
                0 as *mut _,
            );

            connection.last_error().context("Prepare call failed.")?;
        }

        Ok(statement)
    }

    pub fn reset(&mut self) {
        unsafe {
            sqlite3_reset(self.raw_statement);
        }
    }

    pub fn bind_blob(&self, index: i32, blob: &[u8]) -> Result<()> {
        let index = index as c_int;
        let blob_pointer = blob.as_ptr() as *const _;
        let len = blob.len() as c_int;
        unsafe {
            sqlite3_bind_blob(
                self.raw_statement,
                index,
                blob_pointer,
                len,
                SQLITE_TRANSIENT(),
            );
        }
        self.connection.last_error()
    }

    pub fn column_blob<'b>(&'b mut self, index: i32) -> Result<&'b [u8]> {
        let index = index as c_int;
        let pointer = unsafe { sqlite3_column_blob(self.raw_statement, index) };

        self.connection.last_error()?;
        if pointer.is_null() {
            return Ok(&[]);
        }
        let len = unsafe { sqlite3_column_bytes(self.raw_statement, index) as usize };
        self.connection.last_error()?;
        unsafe { Ok(slice::from_raw_parts(pointer as *const u8, len)) }
    }

    pub fn bind_double(&self, index: i32, double: f64) -> Result<()> {
        let index = index as c_int;

        unsafe {
            sqlite3_bind_double(self.raw_statement, index, double);
        }
        self.connection.last_error()
    }

    pub fn column_double(&self, index: i32) -> Result<f64> {
        let index = index as c_int;
        let result = unsafe { sqlite3_column_double(self.raw_statement, index) };
        self.connection.last_error()?;
        Ok(result)
    }

    pub fn bind_int(&self, index: i32, int: i32) -> Result<()> {
        let index = index as c_int;

        unsafe {
            sqlite3_bind_int(self.raw_statement, index, int);
        }
        self.connection.last_error()
    }

    pub fn column_int(&self, index: i32) -> Result<i32> {
        let index = index as c_int;
        let result = unsafe { sqlite3_column_int(self.raw_statement, index) };
        self.connection.last_error()?;
        Ok(result)
    }

    pub fn bind_int64(&self, index: i32, int: i64) -> Result<()> {
        let index = index as c_int;
        unsafe {
            sqlite3_bind_int64(self.raw_statement, index, int);
        }
        self.connection.last_error()
    }

    pub fn column_int64(&self, index: i32) -> Result<i64> {
        let index = index as c_int;
        let result = unsafe { sqlite3_column_int64(self.raw_statement, index) };
        self.connection.last_error()?;
        Ok(result)
    }

    pub fn bind_null(&self, index: i32) -> Result<()> {
        let index = index as c_int;
        unsafe {
            sqlite3_bind_null(self.raw_statement, index);
        }
        self.connection.last_error()
    }

    pub fn bind_text(&self, index: i32, text: &str) -> Result<()> {
        let index = index as c_int;
        let text_pointer = text.as_ptr() as *const _;
        let len = text.len() as c_int;
        unsafe {
            sqlite3_bind_blob(
                self.raw_statement,
                index,
                text_pointer,
                len,
                SQLITE_TRANSIENT(),
            );
        }
        self.connection.last_error()
    }

    pub fn column_text<'b>(&'b mut self, index: i32) -> Result<&'b str> {
        let index = index as c_int;
        let pointer = unsafe { sqlite3_column_text(self.raw_statement, index) };

        self.connection.last_error()?;
        if pointer.is_null() {
            return Ok("");
        }
        let len = unsafe { sqlite3_column_bytes(self.raw_statement, index) as usize };
        self.connection.last_error()?;

        let slice = unsafe { slice::from_raw_parts(pointer as *const u8, len) };
        Ok(str::from_utf8(slice)?)
    }

    pub fn bind<T: Bind>(&self, value: T) -> Result<()> {
        value.bind(self, 1)?;
        Ok(())
    }

    pub fn column<T: Column>(&mut self) -> Result<T> {
        let (result, _) = T::column(self, 0)?;
        Ok(result)
    }

    pub fn column_type(&mut self, index: i32) -> Result<SqlType> {
        let result = unsafe { sqlite3_column_type(self.raw_statement, index) }; // SELECT <FRIEND> FROM TABLE
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

    pub fn bound(&mut self, bindings: impl Bind) -> Result<&mut Self> {
        self.bind(bindings)?;
        Ok(self)
    }

    fn step(&mut self) -> Result<StepResult> {
        unsafe {
            match sqlite3_step(self.raw_statement) {
                SQLITE_ROW => Ok(StepResult::Row),
                SQLITE_DONE => Ok(StepResult::Done),
                SQLITE_MISUSE => Ok(StepResult::Misuse),
                other => self
                    .connection
                    .last_error()
                    .map(|_| StepResult::Other(other)),
            }
        }
    }

    pub fn run(&mut self) -> Result<()> {
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
            if this.step()? != StepResult::Row {
                return Err(anyhow!(
                    "Single(Map) called with query that returns no rows."
                ));
            }
            callback(this)
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
            if this.step()? != StepResult::Row {
                return Ok(None);
            }
            callback(this).map(|r| Some(r))
        }
        let result = logic(self, callback);
        self.reset();
        result
    }

    pub fn maybe_row<R: Column>(&mut self) -> Result<Option<R>> {
        self.maybe(|this| this.column::<R>())
    }
}

impl<'a> Drop for Statement<'a> {
    fn drop(&mut self) {
        unsafe {
            sqlite3_finalize(self.raw_statement);
            self.connection
                .last_error()
                .expect("sqlite3 finalize failed for statement :(");
        };
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::{connection::Connection, statement::StepResult};

    #[test]
    fn blob_round_trips() {
        let connection1 = Connection::open_memory("blob_round_trips");
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
        assert_eq!(write.step().unwrap(), StepResult::Done);

        // Read the blob from the
        let connection2 = Connection::open_memory("blob_round_trips");
        let mut read = connection2.prepare("SELECT * FROM blobs;").unwrap();
        assert_eq!(read.step().unwrap(), StepResult::Row);
        assert_eq!(read.column_blob(0).unwrap(), blob);
        assert_eq!(read.step().unwrap(), StepResult::Done);

        // Delete the added blob and verify its deleted on the other side
        connection2.exec("DELETE FROM blobs;").unwrap();
        let mut read = connection1.prepare("SELECT * FROM blobs;").unwrap();
        assert_eq!(read.step().unwrap(), StepResult::Done);
    }
}
