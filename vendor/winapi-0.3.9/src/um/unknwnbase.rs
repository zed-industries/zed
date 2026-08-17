// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use ctypes::c_void;
use shared::guiddef::REFIID;
use shared::minwindef::{BOOL, ULONG};
use um::winnt::HRESULT;
RIDL!{#[uuid(0x00000000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IUnknown(IUnknownVtbl) {
    fn QueryInterface(
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn AddRef() -> ULONG,
    fn Release() -> ULONG,
}}
pub type LPUNKNOWN = *mut IUnknown;
RIDL!{#[uuid(0x000e0000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface AsyncIUnknown(AsyncIUnknownVtbl): IUnknown(IUnknownVtbl) {
    fn Begin_QueryInterface(
        riid: REFIID,
    ) -> HRESULT,
    fn Finish_QueryInterface(
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn Begin_AddRef() -> HRESULT,
    fn Finish_AddRef() -> ULONG,
    fn Begin_Release() -> HRESULT,
    fn Finish_Release() -> ULONG,
}}
RIDL!{#[uuid(0x00000001, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IClassFactory(IClassFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateInstance(
        pUnkOuter: *mut IUnknown,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn LockServer(
        fLock: BOOL,
    ) -> HRESULT,
}}
