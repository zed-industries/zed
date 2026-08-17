// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io;

use super::ReadBytes;

/// A `Monitor` provides a common interface to examine the operations observed be
/// a [`MonitorStream`].
pub trait Monitor {
    fn process_byte(&mut self, byte: u8);

    #[inline(always)]
    fn process_double_bytes(&mut self, buf: [u8; 2]) {
        self.process_byte(buf[0]);
        self.process_byte(buf[1]);
    }

    #[inline(always)]
    fn process_triple_bytes(&mut self, buf: [u8; 3]) {
        self.process_byte(buf[0]);
        self.process_byte(buf[1]);
        self.process_byte(buf[2]);
    }

    #[inline(always)]
    fn process_quad_bytes(&mut self, buf: [u8; 4]) {
        self.process_byte(buf[0]);
        self.process_byte(buf[1]);
        self.process_byte(buf[2]);
        self.process_byte(buf[3]);
    }

    fn process_buf_bytes(&mut self, buf: &[u8]);
}

/// A `MonitorStream` is a passive stream that observes all operations performed on the inner
/// stream and forwards an immutable reference of the result to a [`Monitor`].
pub struct MonitorStream<B: ReadBytes, M: Monitor> {
    inner: B,
    monitor: M,
}

impl<B: ReadBytes, M: Monitor> MonitorStream<B, M> {
    pub fn new(inner: B, monitor: M) -> MonitorStream<B, M> {
        MonitorStream { inner, monitor }
    }

    pub fn inner(&self) -> &B {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut B {
        &mut self.inner
    }

    pub fn into_inner(self) -> B {
        self.inner
    }

    pub fn monitor(&self) -> &M {
        &self.monitor
    }

    pub fn monitor_mut(&mut self) -> &mut M {
        &mut self.monitor
    }
}

impl<B: ReadBytes, M: Monitor> ReadBytes for MonitorStream<B, M> {
    #[inline(always)]
    fn read_byte(&mut self) -> io::Result<u8> {
        let byte = self.inner.read_byte()?;
        self.monitor.process_byte(byte);
        Ok(byte)
    }

    #[inline(always)]
    fn read_double_bytes(&mut self) -> io::Result<[u8; 2]> {
        let bytes = self.inner.read_double_bytes()?;
        self.monitor.process_double_bytes(bytes);
        Ok(bytes)
    }

    #[inline(always)]
    fn read_triple_bytes(&mut self) -> io::Result<[u8; 3]> {
        let bytes = self.inner.read_triple_bytes()?;
        self.monitor.process_triple_bytes(bytes);
        Ok(bytes)
    }

    #[inline(always)]
    fn read_quad_bytes(&mut self) -> io::Result<[u8; 4]> {
        let bytes = self.inner.read_quad_bytes()?;
        self.monitor.process_quad_bytes(bytes);
        Ok(bytes)
    }

    fn read_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.inner.read_buf(buf)?;
        self.monitor.process_buf_bytes(&buf[0..len]);
        Ok(len)
    }

    fn read_buf_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.inner.read_buf_exact(buf)?;
        self.monitor.process_buf_bytes(buf);
        Ok(())
    }

    fn scan_bytes_aligned<'a>(
        &mut self,
        pattern: &[u8],
        align: usize,
        buf: &'a mut [u8],
    ) -> io::Result<&'a mut [u8]> {
        let result = self.inner.scan_bytes_aligned(pattern, align, buf)?;
        self.monitor.process_buf_bytes(result);
        Ok(result)
    }

    fn ignore_bytes(&mut self, count: u64) -> io::Result<()> {
        self.inner.ignore_bytes(count)
    }

    #[inline(always)]
    fn pos(&self) -> u64 {
        self.inner.pos()
    }
}
