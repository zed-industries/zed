use std::{
    ffi::OsStr,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result};

use crate::statement::{SqlType, Statement};

pub trait Bind {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32>;
}

pub trait Column: Sized {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)>;
}

impl Bind for bool {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement
            .bind(self.then_some(1).unwrap_or(0), start_index)
            .with_context(|| format!("Failed to bind bool at index {start_index}"))
    }
}

impl Column for bool {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        i32::column(statement, start_index)
            .map(|(i, next_index)| (i != 0, next_index))
            .with_context(|| format!("Failed to read bool at index {start_index}"))
    }
}

impl Bind for &[u8] {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement
            .bind_blob(start_index, self)
            .with_context(|| format!("Failed to bind &[u8] at index {start_index}"))?;
        Ok(start_index + 1)
    }
}

impl<const C: usize> Bind for &[u8; C] {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement
            .bind_blob(start_index, self.as_slice())
            .with_context(|| format!("Failed to bind &[u8; C] at index {start_index}"))?;
        Ok(start_index + 1)
    }
}

impl Bind for Vec<u8> {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement
            .bind_blob(start_index, self)
            .with_context(|| format!("Failed to bind Vec<u8> at index {start_index}"))?;
        Ok(start_index + 1)
    }
}

impl Column for Vec<u8> {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let result = statement
            .column_blob(start_index)
            .with_context(|| format!("Failed to read Vec<u8> at index {start_index}"))?;

        Ok((Vec::from(result), start_index + 1))
    }
}

impl Bind for f64 {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement
            .bind_double(start_index, *self)
            .with_context(|| format!("Failed to bind f64 at index {start_index}"))?;
        Ok(start_index + 1)
    }
}

impl Column for f64 {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let result = statement
            .column_double(start_index)
            .with_context(|| format!("Failed to parse f64 at index {start_index}"))?;

        Ok((result, start_index + 1))
    }
}

impl Bind for i32 {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement
            .bind_int(start_index, *self)
            .with_context(|| format!("Failed to bind i32 at index {start_index}"))?;

        Ok(start_index + 1)
    }
}

impl Column for i32 {
    fn column<'a>(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let result = statement.column_int(start_index)?;
        Ok((result, start_index + 1))
    }
}

impl Bind for i64 {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement
            .bind_int64(start_index, *self)
            .with_context(|| format!("Failed to bind i64 at index {start_index}"))?;
        Ok(start_index + 1)
    }
}

impl Column for i64 {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let result = statement.column_int64(start_index)?;
        Ok((result, start_index + 1))
    }
}

impl Bind for usize {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        (*self as i64)
            .bind(statement, start_index)
            .with_context(|| format!("Failed to bind usize at index {start_index}"))
    }
}

impl Column for usize {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let result = statement.column_int64(start_index)?;
        Ok((result as usize, start_index + 1))
    }
}

impl Bind for &str {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement.bind_text(start_index, self)?;
        Ok(start_index + 1)
    }
}

impl Bind for Arc<str> {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement.bind_text(start_index, self.as_ref())?;
        Ok(start_index + 1)
    }
}

impl Bind for String {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        statement.bind_text(start_index, self)?;
        Ok(start_index + 1)
    }
}

impl Column for Arc<str> {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let result = statement.column_text(start_index)?;
        Ok((Arc::from(result), start_index + 1))
    }
}

impl Column for String {
    fn column<'a>(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let result = statement.column_text(start_index)?;
        Ok((result.to_owned(), start_index + 1))
    }
}

impl<T: Bind> Bind for Option<T> {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        if let Some(this) = self {
            this.bind(statement, start_index)
        } else {
            statement.bind_null(start_index)?;
            Ok(start_index + 1)
        }
    }
}

impl<T: Column> Column for Option<T> {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        if let SqlType::Null = statement.column_type(start_index)? {
            Ok((None, start_index + 1))
        } else {
            T::column(statement, start_index).map(|(result, next_index)| (Some(result), next_index))
        }
    }
}

impl<T: Bind, const COUNT: usize> Bind for [T; COUNT] {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let mut current_index = start_index;
        for binding in self {
            current_index = binding.bind(statement, current_index)?
        }

        Ok(current_index)
    }
}

impl<T: Column + Default + Copy, const COUNT: usize> Column for [T; COUNT] {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let mut array = [Default::default(); COUNT];
        let mut current_index = start_index;
        for i in 0..COUNT {
            (array[i], current_index) = T::column(statement, current_index)?;
        }
        Ok((array, current_index))
    }
}

impl<T: Bind> Bind for Vec<T> {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let mut current_index = start_index;
        for binding in self.iter() {
            current_index = binding.bind(statement, current_index)?
        }

        Ok(current_index)
    }
}

impl<T: Bind> Bind for &[T] {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let mut current_index = start_index;
        for binding in *self {
            current_index = binding.bind(statement, current_index)?
        }

        Ok(current_index)
    }
}

impl Bind for &Path {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        self.as_os_str().as_bytes().bind(statement, start_index)
    }
}

impl Bind for Arc<Path> {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        self.as_ref().bind(statement, start_index)
    }
}

impl Bind for PathBuf {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        (self.as_ref() as &Path).bind(statement, start_index)
    }
}

impl Column for PathBuf {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let blob = statement.column_blob(start_index)?;

        Ok((
            PathBuf::from(OsStr::from_bytes(blob).to_owned()),
            start_index + 1,
        ))
    }
}

/// Unit impls do nothing. This simplifies query macros
impl Bind for () {
    fn bind(&self, _statement: &Statement, start_index: i32) -> Result<i32> {
        Ok(start_index)
    }
}

impl Column for () {
    fn column(_statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        Ok(((), start_index))
    }
}

impl<T1: Bind, T2: Bind> Bind for (T1, T2) {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = self.0.bind(statement, start_index)?;
        self.1.bind(statement, next_index)
    }
}

impl<T1: Column, T2: Column> Column for (T1, T2) {
    fn column<'a>(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (first, next_index) = T1::column(statement, start_index)?;
        let (second, next_index) = T2::column(statement, next_index)?;
        Ok(((first, second), next_index))
    }
}

impl<T1: Bind, T2: Bind, T3: Bind> Bind for (T1, T2, T3) {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = self.0.bind(statement, start_index)?;
        let next_index = self.1.bind(statement, next_index)?;
        self.2.bind(statement, next_index)
    }
}

impl<T1: Column, T2: Column, T3: Column> Column for (T1, T2, T3) {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (first, next_index) = T1::column(statement, start_index)?;
        let (second, next_index) = T2::column(statement, next_index)?;
        let (third, next_index) = T3::column(statement, next_index)?;
        Ok(((first, second, third), next_index))
    }
}

impl<T1: Bind, T2: Bind, T3: Bind, T4: Bind> Bind for (T1, T2, T3, T4) {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = self.0.bind(statement, start_index)?;
        let next_index = self.1.bind(statement, next_index)?;
        let next_index = self.2.bind(statement, next_index)?;
        self.3.bind(statement, next_index)
    }
}

impl<T1: Column, T2: Column, T3: Column, T4: Column> Column for (T1, T2, T3, T4) {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (first, next_index) = T1::column(statement, start_index)?;
        let (second, next_index) = T2::column(statement, next_index)?;
        let (third, next_index) = T3::column(statement, next_index)?;
        let (fourth, next_index) = T4::column(statement, next_index)?;
        Ok(((first, second, third, fourth), next_index))
    }
}

impl<T1: Bind, T2: Bind, T3: Bind, T4: Bind, T5: Bind> Bind for (T1, T2, T3, T4, T5) {
    fn bind(&self, statement: &Statement, start_index: i32) -> Result<i32> {
        let next_index = self.0.bind(statement, start_index)?;
        let next_index = self.1.bind(statement, next_index)?;
        let next_index = self.2.bind(statement, next_index)?;
        let next_index = self.3.bind(statement, next_index)?;
        self.4.bind(statement, next_index)
    }
}

impl<T1: Column, T2: Column, T3: Column, T4: Column, T5: Column> Column for (T1, T2, T3, T4, T5) {
    fn column(statement: &mut Statement, start_index: i32) -> Result<(Self, i32)> {
        let (first, next_index) = T1::column(statement, start_index)?;
        let (second, next_index) = T2::column(statement, next_index)?;
        let (third, next_index) = T3::column(statement, next_index)?;
        let (fourth, next_index) = T4::column(statement, next_index)?;
        let (fifth, next_index) = T5::column(statement, next_index)?;
        Ok(((first, second, third, fourth, fifth), next_index))
    }
}
