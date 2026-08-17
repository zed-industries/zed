// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![cfg(test)]
#![allow(dead_code)]

use crate::CrcParams;
use crc::{Crc, Table};

pub struct CrcTestConfig<T: crc::Width, I: crc::Implementation + 'static> {
    pub params: CrcParams,
    pub reference_impl: &'static Crc<T, I>,
}

pub type Crc16TestConfig = CrcTestConfig<u16, Table<16>>;

pub type Crc32TestConfig = CrcTestConfig<u32, Table<16>>;

pub type Crc64TestConfig = CrcTestConfig<u64, Table<16>>;
