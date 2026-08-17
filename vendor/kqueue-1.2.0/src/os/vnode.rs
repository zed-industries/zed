use kqueue_sys::constants::FilterFlag;

use super::super::Vnode;

#[cfg(target_os = "freebsd")]
pub(crate) fn handle_vnode_extras(ff: FilterFlag) -> Option<Vnode> {
    if ff.contains(FilterFlag::NOTE_CLOSE_WRITE) {
        Some(Vnode::CloseWrite)
    } else if ff.contains(FilterFlag::NOTE_CLOSE) {
        Some(Vnode::Close)
    } else if ff.contains(FilterFlag::NOTE_OPEN) {
        Some(Vnode::Open)
    } else if ff.contains(FilterFlag::NOTE_READ) {
        Some(Vnode::Read)
    } else {
        None
    }
}

#[cfg(not(target_os = "freebsd"))]
pub(crate) fn handle_vnode_extras(_ff: FilterFlag) -> Option<Vnode> {
    None
}
