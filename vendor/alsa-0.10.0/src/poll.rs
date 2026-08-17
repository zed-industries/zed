//! Tiny poll ffi
//!
//! A tiny wrapper around libc's poll system call.

use libc;
use super::error::*;
use std::io;
pub use libc::pollfd;


bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags: ::libc::c_short {
        const IN  = ::libc::POLLIN;
        const PRI = ::libc::POLLPRI;
        const OUT = ::libc::POLLOUT;
        const ERR = ::libc::POLLERR;
        const HUP = ::libc::POLLHUP;
        const NVAL = ::libc::POLLNVAL;
    }
}

pub trait Descriptors {
    fn count(&self) -> usize;
    fn fill(&self, _: &mut [pollfd]) -> Result<usize>;
    fn revents(&self, _: &[pollfd]) -> Result<Flags>;

    /// Wrapper around count and fill - returns an array of pollfds
    fn get(&self) -> Result<Vec<pollfd>> {
        let mut v = vec![pollfd { fd: 0, events: 0, revents: 0 }; self.count()];
        if self.fill(&mut v)? != v.len() { Err(Error::unsupported("did not fill the poll descriptors array")) }
        else { Ok(v) }
    }
}

impl Descriptors for pollfd {
    fn count(&self) -> usize { 1 }
    fn fill(&self, a: &mut [pollfd]) -> Result<usize> { a[0] = *self; Ok(1) }
    fn revents(&self, a: &[pollfd]) -> Result<Flags> { Ok(Flags::from_bits_truncate(a[0].revents)) }
}

/// Wrapper around the libc poll call.
pub fn poll(fds: &mut[pollfd], timeout: i32) -> Result<usize> {
    let r = unsafe { libc::poll(fds.as_mut_ptr(), fds.len() as libc::nfds_t, timeout as libc::c_int) };
    if r >= 0 { Ok(r as usize) } else {
         from_code("poll", -io::Error::last_os_error().raw_os_error().unwrap()).map(|_| unreachable!())
    }
}

/// Builds a pollfd array, polls it, and returns the poll descriptors which have non-zero revents.
pub fn poll_all<'a>(desc: &[&'a dyn Descriptors], timeout: i32) -> Result<Vec<(&'a dyn Descriptors, Flags)>> {

    let mut pollfds: Vec<pollfd> = vec!();
    let mut indices = vec!();
    for v2 in desc.iter().map(|q| q.get()) {
        let v = v2?;
        indices.push(pollfds.len() .. pollfds.len()+v.len());
        pollfds.extend(v);
    };

    poll(&mut pollfds, timeout)?;

    let mut res = vec!();
    for (i, r) in indices.into_iter().enumerate() {
        let z = desc[i].revents(&pollfds[r])?;
        if !z.is_empty() { res.push((desc[i], z)); }
    }
    Ok(res)
}
