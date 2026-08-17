// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of ExDisp.h
use shared::basetsd::SHANDLE_PTR;
use shared::wtypes::{BSTR, VARIANT_BOOL};
use um::docobj::{OLECMDEXECOPT, OLECMDF, OLECMDID};
use um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};
use um::ocidl::READYSTATE;
use um::winnt::{HRESULT, INT, LONG};
DEFINE_GUID!{IID_IWebBrowser2,
    0xd30c1661, 0xcdaf, 0x11d0, 0x8a, 0x3e, 0x00, 0xc0, 0x4f, 0xc9, 0xe2, 0x6e}
RIDL!{#[uuid(0xeab22ac1, 0x30c1, 0x11cf, 0xa7, 0xeb, 0x00, 0x00, 0xc0, 0x5b, 0xae, 0x0b)]
interface IWebBrowser(IWebBrowserVtbl): IDispatch(IDispatchVtbl) {
    fn GoBack() -> HRESULT,
    fn GoForward() -> HRESULT,
    fn GoHome() -> HRESULT,
    fn GoSearch() -> HRESULT,
    fn Navigate(
        URL: BSTR,
        Flags: *const VARIANT,
        TargetFrameName: *const VARIANT,
        PostData: *const VARIANT,
        Headers: *const VARIANT,
    ) -> HRESULT,
    fn Refresh() -> HRESULT,
    fn Refresh2(
        Level: *const VARIANT,
    ) -> HRESULT,
    fn Stop() -> HRESULT,
    fn get_Application(
        ppDisp: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Parent(
        ppDisp: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Container(
        ppDisp: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_Document(
        ppDisp: *mut *mut IDispatch,
    ) -> HRESULT,
    fn get_TopLevelContainer(
        pBool: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Type(
        Type: *mut BSTR,
    ) -> HRESULT,
    fn get_Left(
        pl: *mut LONG,
    ) -> HRESULT,
    fn put_Left(
        Left: LONG,
    ) -> HRESULT,
    fn get_Top(
        pl: *mut LONG,
    ) -> HRESULT,
    fn put_Top(
        Top: LONG,
    ) -> HRESULT,
    fn get_Width(
        pl: *mut LONG,
    ) -> HRESULT,
    fn put_Width(
        Width: LONG,
    ) -> HRESULT,
    fn get_Height(
        pl: *mut LONG,
    ) -> HRESULT,
    fn put_Height(
        Height: LONG,
    ) -> HRESULT,
    fn get_LocationName(
        LocationName: *mut BSTR,
    ) -> HRESULT,
    fn get_LocationURL(
        LocationURL: *mut BSTR,
    ) -> HRESULT,
    fn get_Busy(
        pBool: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x0002df05, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IWebBrowserApp(IWebBrowserAppVtbl): IWebBrowser(IWebBrowserVtbl) {
    fn Quit() -> HRESULT,
    fn ClientToWindow(
        pcx: *mut INT,
        pcy: *mut INT,
    ) -> HRESULT,
    fn PutProperty(
        Property: BSTR,
        vtValue: VARIANT,
    ) -> HRESULT,
    fn GetProperty(
        Property: BSTR,
        pvtValue: *mut VARIANT,
    ) -> HRESULT,
    fn get_Name(
        Name: *mut BSTR,
    ) -> HRESULT,
    fn get_HWND(
        pHWND: *mut SHANDLE_PTR,
    ) -> HRESULT,
    fn get_FullName(
        FullName: *mut BSTR,
    ) -> HRESULT,
    fn get_Path(
        Path: *mut BSTR,
    ) -> HRESULT,
    fn get_Visible(
        pBool: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Visible(
        Value: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_StatusBar(
        pBool: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_StatusBar(
        Value: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_StatusText(
        StatusText: *mut BSTR,
    ) -> HRESULT,
    fn put_StatusText(
        StatusText: BSTR,
    ) -> HRESULT,
    fn get_ToolBar(
        Value: *mut INT,
    ) -> HRESULT,
    fn put_ToolBar(
        Value: INT,
    ) -> HRESULT,
    fn get_MenuBar(
        Value: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_MenuBar(
        Value: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_FullScreen(
        pbFullScreen: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_FullScreen(
        bFullScreen: VARIANT_BOOL,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd30c1661, 0xcdaf, 0x11d0, 0x8a, 0x3e, 0x00, 0xc0, 0x4f, 0xc9, 0xe2, 0x6e)]
interface IWebBrowser2(IWebBrowser2Vtbl): IWebBrowserApp(IWebBrowserAppVtbl) {
    fn Navigate2(
        URL: *const VARIANT,
        Flags: *const VARIANT,
        TargetFrameName: *const VARIANT,
        PostData: *const VARIANT,
        Headers: *const VARIANT,
    ) -> HRESULT,
    fn QueryStatusWB(
        cmdID: OLECMDID,
        pcmdf: *mut OLECMDF,
    ) -> HRESULT,
    fn ExecWB(
        cmdID: OLECMDID,
        cmdexecopt: OLECMDEXECOPT,
        pvaIn: *const VARIANT,
        pvaOut: *mut VARIANT,
    ) -> HRESULT,
    fn ShowBrowserBar(
        pvaClsid: *const VARIANT,
        pvarShow: *const VARIANT,
        pvarSize: *const VARIANT,
    ) -> HRESULT,
    fn get_ReadyState(
        plReadyState: *mut READYSTATE,
    ) -> HRESULT,
    fn get_Offline(
        pbOffline: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Offline(
        bOffline: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Silent(
        pbSilent: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Silent(
        bSilent: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RegisterAsBrowser(
        pbRegister: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RegisterAsBrowser(
        bRegister: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_RegisterAsDropTarget(
        pbRegister: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_RegisterAsDropTarget(
        bRegister: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_TheaterMode(
        pbRegister: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_TheaterMode(
        bRegister: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AddressBar(
        Value: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_AddressBar(
        Value: VARIANT_BOOL,
    ) -> HRESULT,
    fn get_Resizable(
        Value: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn put_Resizable(
        Value: VARIANT_BOOL,
    ) -> HRESULT,
}}
DEFINE_GUID!{CLSID_InternetExplorer,
    0x0002df01, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46}
