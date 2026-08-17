// Copyright (c) 2024 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use crate::ZipDateTime;

/// A builder for [`ZipDateTime`].
pub struct ZipDateTimeBuilder(pub(crate) ZipDateTime);

impl From<ZipDateTime> for ZipDateTimeBuilder {
    fn from(date: ZipDateTime) -> Self {
        Self(date)
    }
}

impl Default for ZipDateTimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ZipDateTimeBuilder {
    /// Constructs a new builder which defines the raw underlying data of a ZIP entry.
    pub fn new() -> Self {
        Self(ZipDateTime { date: 0, time: 0 })
    }

    /// Sets the date and time's year.
    pub fn year(mut self, year: i32) -> Self {
        let year: u16 = (((year - 1980) << 9) & 0xFE00).try_into().unwrap();
        self.0.date |= year;
        self
    }

    /// Sets the date and time's month.
    pub fn month(mut self, month: u32) -> Self {
        let month: u16 = ((month << 5) & 0x1E0).try_into().unwrap();
        self.0.date |= month;
        self
    }

    /// Sets the date and time's day.
    pub fn day(mut self, day: u32) -> Self {
        let day: u16 = (day & 0x1F).try_into().unwrap();
        self.0.date |= day;
        self
    }

    /// Sets the date and time's hour.
    pub fn hour(mut self, hour: u32) -> Self {
        let hour: u16 = ((hour << 11) & 0xF800).try_into().unwrap();
        self.0.time |= hour;
        self
    }

    /// Sets the date and time's minute.
    pub fn minute(mut self, minute: u32) -> Self {
        let minute: u16 = ((minute << 5) & 0x7E0).try_into().unwrap();
        self.0.time |= minute;
        self
    }

    /// Sets the date and time's second.
    ///
    /// Note that MS-DOS has a maximum granularity of two seconds.
    pub fn second(mut self, second: u32) -> Self {
        let second: u16 = ((second >> 1) & 0x1F).try_into().unwrap();
        self.0.time |= second;
        self
    }

    /// Consumes this builder and returns a final [`ZipDateTime`].
    ///
    /// This is equivalent to:
    /// ```
    /// # use async_zip::{ZipDateTime, ZipDateTimeBuilder, Compression};
    /// #
    /// # let builder = ZipDateTimeBuilder::new().year(2024).month(3).day(2);
    /// let date: ZipDateTime = builder.into();
    /// ```
    pub fn build(self) -> ZipDateTime {
        self.into()
    }
}
