use core::{slice, str};

use crate::{Error, Packet, Result, EOPNOTSUPP, ESKMSG, SKMSG_FRETURNFD};

pub use self::{
    scheme::Scheme, scheme_block::SchemeBlock, scheme_block_mut::SchemeBlockMut,
    scheme_mut::SchemeMut, seek::*,
};

unsafe fn str_from_raw_parts(ptr: *const u8, len: usize) -> Option<&'static str> {
    let slice = slice::from_raw_parts(ptr, len);
    str::from_utf8(slice).ok()
}

mod scheme;
mod scheme_block;
mod scheme_block_mut;
mod scheme_mut;
mod seek;

pub struct CallerCtx {
    pub pid: usize,
    pub uid: u32,
    pub gid: u32,
}

pub enum OpenResult {
    ThisScheme { number: usize },
    OtherScheme { fd: usize },
}

// TODO: Find a better solution than generate.sh
pub(crate) fn convert_to_this_scheme(r: Result<usize>) -> Result<OpenResult> {
    r.map(|number| OpenResult::ThisScheme { number })
}
pub(crate) fn convert_to_this_scheme_block(r: Result<Option<usize>>) -> Result<Option<OpenResult>> {
    r.map(|o| o.map(|number| OpenResult::ThisScheme { number }))
}
pub(crate) fn convert_in_scheme_handle_block(
    _: &Packet,
    result: Result<Option<OpenResult>>,
) -> Result<Option<usize>> {
    match result {
        Ok(Some(OpenResult::ThisScheme { number })) => Ok(Some(number)),
        Ok(Some(OpenResult::OtherScheme { .. })) => Err(Error::new(EOPNOTSUPP)),
        Ok(None) => Ok(None),
        Err(err) => Err(err),
    }
}
pub(crate) fn convert_in_scheme_handle(
    packet: &mut Packet,
    result: Result<OpenResult>,
) -> Result<usize> {
    match result {
        Ok(OpenResult::ThisScheme { number }) => Ok(number),
        Ok(OpenResult::OtherScheme { fd }) => {
            packet.b = SKMSG_FRETURNFD;
            packet.c = fd;
            Err(Error::new(ESKMSG))
        }
        Err(err) => Err(err),
    }
}

impl CallerCtx {
    pub fn from_packet(packet: &Packet) -> Self {
        Self {
            pid: packet.pid,
            uid: packet.uid,
            gid: packet.gid,
        }
    }
}
