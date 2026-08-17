// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
use shared::guiddef::REFGUID;
use shared::minwindef::{BOOL, DWORD, UINT};
use shared::windef::HWND;
use um::propsys::{IPropertyDescriptionList, IPropertyStore};
use um::shobjidl_core::{IModalWindow, IModalWindowVtbl, IShellItem, IShellItemFilter};
use um::shtypes::COMDLG_FILTERSPEC;
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, LPCWSTR, LPWSTR, WCHAR};
pub type IFileOperationProgressSink = IUnknown; // TODO
pub use um::shobjidl_core::{IShellItemArray, SIATTRIBFLAGS}; // FIXME: Remove these in the next major release
ENUM!{enum FDE_OVERWRITE_RESPONSE {
    FDEOR_DEFAULT = 0,
    FDEOR_ACCEPT = 1,
    FDEOR_REFUSE = 2,
}}
ENUM!{enum FDE_SHAREVIOLATION_RESPONSE {
    FDESVR_DEFAULT = 0,
    FDESVR_ACCEPT = 1,
    FDESVR_REFUSE = 2,
}}
ENUM!{enum FDAP {
    FDAP_BOTTOM = 0,
    FDAP_TOP = 1,
}}
RIDL!{#[uuid(0x973510db, 0x7d7f, 0x452b, 0x89, 0x75, 0x74, 0xa8, 0x58, 0x28, 0xd3, 0x54)]
interface IFileDialogEvents(IFileDialogEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnFileOk(
        pfd: *mut IFileDialog,
    ) -> HRESULT,
    fn OnFolderChanging(
        pfd: *mut IFileDialog,
        psiFolder: *mut IShellItem,
    ) -> HRESULT,
    fn OnFolderChange(
        pfd: *mut IFileDialog,
    ) -> HRESULT,
    fn OnSelectionChange(
        pfd: *mut IFileDialog,
    ) -> HRESULT,
    fn OnShareViolation(
        pfd: *mut IFileDialog,
        psi: *mut IShellItem,
        pResponse: *mut FDE_SHAREVIOLATION_RESPONSE,
    ) -> HRESULT,
    fn OnTypeChange(
        pfd: *mut IFileDialog,
    ) -> HRESULT,
    fn OnOverwrite(
        pfd: *mut IFileDialog,
        psi: *mut IShellItem,
        pResponse: *mut FDE_OVERWRITE_RESPONSE,
    ) -> HRESULT,
}}
ENUM!{enum FILEOPENDIALOGOPTIONS {
    FOS_OVERWRITEPROMPT = 0x2,
    FOS_STRICTFILETYPES = 0x4,
    FOS_NOCHANGEDIR = 0x8,
    FOS_PICKFOLDERS = 0x20,
    FOS_FORCEFILESYSTEM = 0x40,
    FOS_ALLNONSTORAGEITEMS = 0x80,
    FOS_NOVALIDATE = 0x100,
    FOS_ALLOWMULTISELECT = 0x200,
    FOS_PATHMUSTEXIST = 0x800,
    FOS_FILEMUSTEXIST = 0x1000,
    FOS_CREATEPROMPT = 0x2000,
    FOS_SHAREAWARE = 0x4000,
    FOS_NOREADONLYRETURN = 0x8000,
    FOS_NOTESTFILECREATE = 0x10000,
    FOS_HIDEMRUPLACES = 0x20000,
    FOS_HIDEPINNEDPLACES = 0x40000,
    FOS_NODEREFERENCELINKS = 0x100000,
    FOS_DONTADDTORECENT = 0x2000000,
    FOS_FORCESHOWHIDDEN = 0x10000000,
    FOS_DEFAULTNOMINIMODE = 0x20000000,
    FOS_FORCEPREVIEWPANEON = 0x40000000,
    FOS_SUPPORTSTREAMABLEITEMS = 0x80000000,
}}
RIDL!{#[uuid(0x42f85136, 0xdb7e, 0x439c, 0x85, 0xf1, 0xe4, 0x07, 0x5d, 0x13, 0x5f, 0xc8)]
interface IFileDialog(IFileDialogVtbl): IModalWindow(IModalWindowVtbl) {
    fn SetFileTypes(
        cFileTypes: UINT,
        rgFilterSpec: *const COMDLG_FILTERSPEC,
    ) -> HRESULT,
    fn SetFileTypeIndex(
        iFileType: UINT,
    ) -> HRESULT,
    fn GetFileTypeIndex(
        piFileType: *mut UINT,
    ) -> HRESULT,
    fn Advise(
        pfde: *mut IFileDialogEvents,
        pdwCookie: *mut DWORD,
    ) -> HRESULT,
    fn Unadvise(
        dwCookie: DWORD,
    ) -> HRESULT,
    fn SetOptions(
        fos: FILEOPENDIALOGOPTIONS,
    ) -> HRESULT,
    fn GetOptions(
        pfos: *mut FILEOPENDIALOGOPTIONS,
    ) -> HRESULT,
    fn SetDefaultFolder(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn SetFolder(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn GetFolder(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn GetCurrentSelection(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn SetFileName(
        pszName: LPCWSTR,
    ) -> HRESULT,
    fn GetFileName(
        pszName: *mut LPWSTR,
    ) -> HRESULT,
    fn SetTitle(
        pszTitle: LPCWSTR,
    ) -> HRESULT,
    fn SetOkButtonLabel(
        pszText: LPCWSTR,
    ) -> HRESULT,
    fn SetFileNameLabel(
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn GetResult(
        ppsi: *mut *mut IShellItem,
    ) -> HRESULT,
    fn AddPlace(
        psi: *mut IShellItem,
        fdap: FDAP,
    ) -> HRESULT,
    fn SetDefaultExtension(
        pszDefaultExtension: LPCWSTR,
    ) -> HRESULT,
    fn Close(
        hr: HRESULT,
    ) -> HRESULT,
    fn SetClientGuid(
        guid: REFGUID,
    ) -> HRESULT,
    fn ClearClientData() -> HRESULT,
    fn SetFilter(
        pFilter: *mut IShellItemFilter,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x84bccd23, 0x5fde, 0x4cdb, 0xae, 0xa4, 0xaf, 0x64, 0xb8, 0x3d, 0x78, 0xab)]
interface IFileSaveDialog(IFileSaveDialogVtbl): IFileDialog(IFileDialogVtbl) {
    fn SetSaveAsItem(
        psi: *mut IShellItem,
    ) -> HRESULT,
    fn SetProperties(
        pStore: *mut IPropertyStore,
    ) -> HRESULT,
    fn SetCollectedProperties(
        pList: *mut IPropertyDescriptionList,
        fAppendDefault: BOOL,
    ) -> HRESULT,
    fn GetProperties(
        ppStore: *mut *mut IPropertyStore,
    ) -> HRESULT,
    fn ApplyProperties(
        psi: *mut IShellItem,
        pStore: *mut IPropertyStore,
        hwnd: HWND,
        pSink: *mut IFileOperationProgressSink,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd57c7288, 0xd4ad, 0x4768, 0xbe, 0x02, 0x9d, 0x96, 0x95, 0x32, 0xd9, 0x60)]
interface IFileOpenDialog(IFileOpenDialogVtbl): IFileDialog(IFileDialogVtbl) {
    fn GetResults(
        ppenum: *mut *mut IShellItemArray,
    ) -> HRESULT,
    fn GetSelectedItems(
        ppsai: *mut *mut IShellItemArray,
    ) -> HRESULT,
}}
ENUM!{enum CDCONTROLSTATEF {
    CDCS_INACTIVE = 0,
    CDCS_ENABLED = 0x1,
    CDCS_VISIBLE = 0x2,
    CDCS_ENABLEDVISIBLE = 0x3,
}}
RIDL!{#[uuid(0xe6fdd21a, 0x163f, 0x4975, 0x9c, 0x8c, 0xa6, 0x9f, 0x1b, 0xa3, 0x70, 0x34)]
interface IFileDialogCustomize(IFileDialogCustomizeVtbl): IUnknown(IUnknownVtbl) {
    fn EnableOpenDropDown(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn AddMenu(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn AddPushButton(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn AddComboBox(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn AddRadioButtonList(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn AddCheckButton(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
        bChecked: BOOL,
    ) -> HRESULT,
    fn AddEditBox(
        dwIDCtl: DWORD,
        pszText: LPCWSTR,
    ) -> HRESULT,
    fn AddSeparator(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn AddText(
        dwIDCtl: DWORD,
        pszText: LPCWSTR,
    ) -> HRESULT,
    fn SetControlLabel(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn GetControlState(
        dwIDCtl: DWORD,
        pdwState: *mut CDCONTROLSTATEF,
    ) -> HRESULT,
    fn SetControlState(
        dwIDCtl: DWORD,
        dwState: CDCONTROLSTATEF,
    ) -> HRESULT,
    fn GetEditBoxText(
        dwIDCtl: DWORD,
        ppszText: *mut *mut WCHAR,
    ) -> HRESULT,
    fn SetEditBoxText(
        dwIDCtl: DWORD,
        pszText: LPCWSTR,
    ) -> HRESULT,
    fn GetCheckButtonState(
        dwIDCtl: DWORD,
        pbChecked: *mut BOOL,
    ) -> HRESULT,
    fn SetCheckButtonState(
        dwIDCtl: DWORD,
        bChecked: BOOL,
    ) -> HRESULT,
    fn AddControlItem(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn RemoveControlItem(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
    ) -> HRESULT,
    fn RemoveAllControlItems(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn GetControlItemState(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
        pdwState: *mut CDCONTROLSTATEF,
    ) -> HRESULT,
    fn SetControlItemState(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
        dwState: CDCONTROLSTATEF,
    ) -> HRESULT,
    fn GetSelectedControlItem(
        dwIDCtl: DWORD,
        pdwIDItem: *mut DWORD,
    ) -> HRESULT,
    fn SetSelectedControlItem(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
    ) -> HRESULT,
    fn StartVisualGroup(
        dwIDCtl: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn EndVisualGroup() -> HRESULT,
    fn MakeProminent(
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn SetControlItemText(
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
        pszLabel: LPCWSTR,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x36116642, 0xd713, 0x4b97, 0x9b, 0x83, 0x74, 0x84, 0xa9, 0xd0, 0x04, 0x33)]
interface IFileDialogControlEvents(IFileDialogControlEventsVtbl): IUnknown(IUnknownVtbl) {
    fn OnItemSelected(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: DWORD,
        dwIDItem: DWORD,
    ) -> HRESULT,
    fn OnButtonClicked(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: DWORD,
    ) -> HRESULT,
    fn OnCheckButtonToggled(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: DWORD,
        bChecked: BOOL,
    ) -> HRESULT,
    fn OnControlActivating(
        pfdc: *mut IFileDialogCustomize,
        dwIDCtl: DWORD,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x61744fc7, 0x85b5, 0x4791, 0xa9, 0xb0, 0x27, 0x22, 0x76, 0x30, 0x9b, 0x13)]
interface IFileDialog2(IFileDialog2Vtbl): IFileDialog(IFileDialogVtbl) {
    fn SetCancelButtonLabel(
        pszLabel: LPCWSTR,
    ) -> HRESULT,
    fn SetNavigationRoot(
        psi: IShellItem,
    ) -> HRESULT,
}}
