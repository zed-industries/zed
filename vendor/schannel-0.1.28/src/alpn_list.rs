use std::alloc;
use std::mem;
use std::ptr;
use std::slice;

use windows_sys::Win32::Security::Authentication::Identity;

// This is manually calculated here rather than using `size_of::<SEC_APPLICATION_PROTOCOL_LIST>()`,
// as the latter is 2 bytes too large because it accounts for padding at the end of the struct for
// alignment requirements, which is irrelevant in actual usage because there is a variable-length
// array at the end of the struct.
const SEC_APPLICATION_PROTOCOL_LIST_HEADER_SIZE: usize =
    mem::size_of::<u32>() + mem::size_of::<u16>();
const SEC_APPLICATION_PROTOCOL_HEADER_SIZE: usize =
    mem::size_of::<u32>() + SEC_APPLICATION_PROTOCOL_LIST_HEADER_SIZE;

pub struct AlpnList {
    layout: alloc::Layout,
    memory: ptr::NonNull<u8>,
}

impl Drop for AlpnList {
    fn drop(&mut self) {
        unsafe {
            // Safety: `self.memory` was allocated with `self.layout` and is non-null.
            alloc::dealloc(self.memory.as_ptr(), self.layout);
        }
    }
}

impl AlpnList {
    pub fn new(protos: &[Vec<u8>]) -> Self {
        // ALPN wire format is each ALPN preceded by its length as a byte.
        let mut alpn_wire_format =
            Vec::with_capacity(protos.iter().map(Vec::len).sum::<usize>() + protos.len());
        for alpn in protos {
            alpn_wire_format.push(alpn.len() as u8);
            alpn_wire_format.extend(alpn);
        }

        let size = SEC_APPLICATION_PROTOCOL_HEADER_SIZE + alpn_wire_format.len();
        let layout = alloc::Layout::from_size_align(
            size,
            mem::align_of::<Identity::SEC_APPLICATION_PROTOCOLS>(),
        )
        .unwrap();

        unsafe {
            // Safety: `layout` is guaranteed to have non-zero size.
            let memory = match ptr::NonNull::new(alloc::alloc(layout)) {
                Some(ptr) => ptr,
                None => alloc::handle_alloc_error(layout),
            };

            // Safety: `memory` was created from `layout`.
            let buf = slice::from_raw_parts_mut(memory.as_ptr(), layout.size());
            let protocols = &mut *(buf.as_mut_ptr() as *mut Identity::SEC_APPLICATION_PROTOCOLS);
            protocols.ProtocolListsSize =
                (SEC_APPLICATION_PROTOCOL_LIST_HEADER_SIZE + alpn_wire_format.len()) as u32;

            let protocol = &mut *protocols.ProtocolLists.as_mut_ptr();
            protocol.ProtoNegoExt = Identity::SecApplicationProtocolNegotiationExt_ALPN;
            protocol.ProtocolListSize = alpn_wire_format.len() as u16;

            let protocol_list_offset =
                protocol.ProtocolList.as_ptr() as usize - buf.as_ptr() as usize;
            let protocol_list = &mut buf[protocol_list_offset..];
            protocol_list.copy_from_slice(&alpn_wire_format);

            Self { layout, memory }
        }
    }
}

impl std::ops::Deref for AlpnList {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            // Safety: `self.memory` was created from `self.layout`.
            slice::from_raw_parts(self.memory.as_ptr(), self.layout.size())
        }
    }
}

impl std::ops::DerefMut for AlpnList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            // Safety: `self.memory` was created from `self.layout`.
            slice::from_raw_parts_mut(self.memory.as_ptr(), self.layout.size())
        }
    }
}
