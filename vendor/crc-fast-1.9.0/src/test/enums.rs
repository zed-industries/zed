// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

#![cfg(test)]
#![allow(dead_code)]

use crate::test::structs::*;
use crate::CrcAlgorithm;
use crate::CrcParams;
use crc::Crc;

pub enum AnyCrcTestConfig {
    CRC16(&'static Crc16TestConfig),
    CRC32(&'static Crc32TestConfig),
    CRC64(&'static Crc64TestConfig),
}

impl AnyCrcTestConfig {
    pub fn get_params(&self) -> &CrcParams {
        match self {
            AnyCrcTestConfig::CRC16(cfg) => &cfg.params,
            AnyCrcTestConfig::CRC32(cfg) => &cfg.params,
            AnyCrcTestConfig::CRC64(cfg) => &cfg.params,
        }
    }

    pub fn get_width(&self) -> u8 {
        self.get_params().width
    }

    pub fn get_poly(&self) -> u64 {
        self.get_params().poly
    }

    pub fn get_refin(&self) -> bool {
        self.get_params().refin
    }

    pub fn get_algorithm(&self) -> CrcAlgorithm {
        self.get_params().algorithm
    }

    pub fn get_init(&self) -> u64 {
        self.get_params().init
    }

    pub fn get_init_algorithm(&self) -> u64 {
        self.get_params().init_algorithm
    }

    pub fn get_xorout(&self) -> u64 {
        self.get_params().xorout
    }

    pub fn get_check(&self) -> u64 {
        self.get_params().check
    }

    pub fn get_name(&self) -> &str {
        self.get_params().name
    }

    pub fn get_keys(&self) -> [u64; 23] {
        self.get_params().keys.to_keys_array_23()
    }

    pub fn checksum_with_reference(&self, data: &[u8]) -> u64 {
        match self {
            AnyCrcTestConfig::CRC16(cfg) => cfg.reference_impl.checksum(data) as u64,
            AnyCrcTestConfig::CRC32(cfg) => cfg.reference_impl.checksum(data) as u64,
            AnyCrcTestConfig::CRC64(cfg) => cfg.reference_impl.checksum(data),
        }
    }

    pub fn with_reference_impl<F, R>(&self, f: F) -> R
    where
        F: FnOnce(
            Option<&Crc<u16, crc::Table<16>>>,
            Option<&Crc<u32, crc::Table<16>>>,
            Option<&Crc<u64, crc::Table<16>>>,
        ) -> R,
    {
        match self {
            AnyCrcTestConfig::CRC16(cfg) => f(Some(cfg.reference_impl), None, None),
            AnyCrcTestConfig::CRC32(cfg) => f(None, Some(cfg.reference_impl), None),
            AnyCrcTestConfig::CRC64(cfg) => f(None, None, Some(cfg.reference_impl)),
        }
    }
}
