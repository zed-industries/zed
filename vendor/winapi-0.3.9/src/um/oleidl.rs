// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::minwindef::DWORD;
use shared::ntdef::HRESULT;
use shared::windef::POINTL;
use um::objidl::IDataObject;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
pub const MK_ALT: DWORD = 20;
pub const DROPEFFECT_NONE: DWORD = 0;
pub const DROPEFFECT_COPY: DWORD = 1;
pub const DROPEFFECT_MOVE: DWORD = 2;
pub const DROPEFFECT_LINK: DWORD = 4;
pub const DROPEFFECT_SCROLL: DWORD = 0x80000000;
pub const DD_DEFSCROLLINSET: DWORD = 11;
pub const DD_DEFSCROLLDELAY: DWORD = 50;
pub const DD_DEFSCROLLINTERVAL: DWORD = 50;
pub const DD_DEFDRAGDELAY: DWORD = 200;
pub const DD_DEFDRAGMINDIST: DWORD = 2;
pub type LPDROPTARGET = *mut IDropTarget;
RIDL!{#[uuid(0x00000122, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IDropTarget(IDropTargetVtbl): IUnknown(IUnknownVtbl) {
    fn DragEnter(
        pDataObj: *const IDataObject,
        grfKeyState: DWORD,
        pt: *const POINTL,
        pdwEffect: *mut DWORD,
    ) -> HRESULT,
    fn DragOver(
        grfKeyState: DWORD,
        pt: *const POINTL,
        pdwEffect: *mut DWORD,
    ) -> HRESULT,
    fn DragLeave() -> HRESULT,
    fn Drop(
        pDataObj: *const IDataObject,
        grfKeyState: DWORD,
        pt: *const POINTL,
        pdwEffect: *mut DWORD,
    ) -> HRESULT,
}}
