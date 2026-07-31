//! Clipboard integration via the OpenHarmony Pasteboard and UDMF APIs.
//!
//! Uses `OH_Pasteboard` (system pasteboard) with UDMF records to read and
//! write plain-text data.  Requires the process to have the
//! `ohos.permission.READ_PASTEBOARD` and `ohos.permission.WRITE_PASTEBOARD`
//! capabilities declared in its `module.json5`.

use gpui::ClipboardItem;
use log::warn;
use std::ffi::{CStr, CString};

// UDMF (database/udmf/uds.h + database/udmf/udmf.h)
#[allow(non_camel_case_types)]
type OH_UdsPlainText = std::ffi::c_void;
#[allow(non_camel_case_types)]
type OH_UdmfRecord = std::ffi::c_void;
#[allow(non_camel_case_types)]
type OH_UdmfData = std::ffi::c_void;
#[allow(non_camel_case_types)]
type OH_Pasteboard = std::ffi::c_void;

#[link(name = "pasteboard", kind = "dylib")]
#[link(name = "udmf", kind = "dylib")]
#[allow(non_snake_case, non_camel_case_types)]
unsafe extern "C" {
    fn OH_Pasteboard_Create() -> *mut OH_Pasteboard;
    fn OH_Pasteboard_Destroy(pasteboard: *mut OH_Pasteboard);
    fn OH_Pasteboard_HasData(pasteboard: *mut OH_Pasteboard) -> bool;
    fn OH_Pasteboard_GetData(pasteboard: *mut OH_Pasteboard, status: *mut i32) -> *mut OH_UdmfData;
    fn OH_Pasteboard_SetData(pasteboard: *mut OH_Pasteboard, data: *mut OH_UdmfData) -> i32;
    fn OH_Pasteboard_ClearData(pasteboard: *mut OH_Pasteboard) -> i32;

    // UDMF data
    fn OH_UdmfData_Create() -> *mut OH_UdmfData;
    fn OH_UdmfData_Destroy(data: *mut OH_UdmfData);
    fn OH_UdmfData_AddRecord(data: *mut OH_UdmfData, record: *mut OH_UdmfRecord) -> i32;
    fn OH_UdmfData_GetPrimaryPlainText(data: *mut OH_UdmfData, plain_text: *mut OH_UdsPlainText) -> i32;

    // UDMF record
    fn OH_UdmfRecord_Create() -> *mut OH_UdmfRecord;
    fn OH_UdmfRecord_Destroy(record: *mut OH_UdmfRecord);
    fn OH_UdmfRecord_AddPlainText(record: *mut OH_UdmfRecord, plain_text: *mut OH_UdsPlainText) -> i32;

    // UDS plain text
    fn OH_UdsPlainText_Create() -> *mut OH_UdsPlainText;
    fn OH_UdsPlainText_Destroy(plain_text: *mut OH_UdsPlainText);
    fn OH_UdsPlainText_SetContent(plain_text: *mut OH_UdsPlainText, content: *const std::ffi::c_char) -> i32;
    fn OH_UdsPlainText_GetContent(plain_text: *mut OH_UdsPlainText) -> *const std::ffi::c_char;
}

// ── Public API ────────────────────────────────────────────────────────────────

pub(crate) fn read_from_clipboard() -> Option<ClipboardItem> {
    unsafe {
        let pasteboard = OH_Pasteboard_Create();
        if pasteboard.is_null() {
            return None;
        }

        let _guard = PasteboardGuard(pasteboard);

        if !OH_Pasteboard_HasData(pasteboard) {
            return None;
        }

        let mut status: i32 = 0;
        let data = OH_Pasteboard_GetData(pasteboard, &mut status as *mut i32);
        if data.is_null() {
            warn!("OH_Pasteboard_GetData failed, status={status}");
            return None;
        }
        let _data_guard = UdmfDataGuard(data);

        let mut pt: *mut OH_UdsPlainText = std::ptr::null_mut();
        if OH_UdmfData_GetPrimaryPlainText(data, std::ptr::addr_of_mut!(pt).cast()) != 0 || pt.is_null() {
            // No plain-text record in clipboard.
            return None;
        }
        let _pt_guard = UdsPlainTextGuard(pt);

        let raw = OH_UdsPlainText_GetContent(pt);
        if raw.is_null() {
            return None;
        }
        let text = CStr::from_ptr(raw).to_string_lossy().into_owned();
        if text.is_empty() {
            return None;
        }
        Some(ClipboardItem::new_string(text))
    }
}

pub(crate) fn write_to_clipboard(item: &ClipboardItem) {
    let Some(text) = item.text() else {
        return;
    };

    let Ok(c_text) = CString::new(text) else {
        return;
    };

    unsafe {
        let pt = OH_UdsPlainText_Create();
        if pt.is_null() {
            return;
        }
        let _pt_guard = UdsPlainTextGuard(pt);

        if OH_UdsPlainText_SetContent(pt, c_text.as_ptr()) != 0 {
            warn!("OH_UdsPlainText_SetContent failed");
            return;
        }

        let record = OH_UdmfRecord_Create();
        if record.is_null() {
            return;
        }
        let _rec_guard = UdmfRecordGuard(record);

        if OH_UdmfRecord_AddPlainText(record, pt) != 0 {
            warn!("OH_UdmfRecord_AddPlainText failed");
            return;
        }

        let data = OH_UdmfData_Create();
        if data.is_null() {
            return;
        }
        let _data_guard = UdmfDataGuard(data);

        if OH_UdmfData_AddRecord(data, record) != 0 {
            warn!("OH_UdmfData_AddRecord failed");
            return;
        }

        let pasteboard = OH_Pasteboard_Create();
        if pasteboard.is_null() {
            return;
        }
        let _pb_guard = PasteboardGuard(pasteboard);

        OH_Pasteboard_ClearData(pasteboard);
        if OH_Pasteboard_SetData(pasteboard, data) != 0 {
            warn!("OH_Pasteboard_SetData failed");
        }
    }
}

// ── RAII guards for OHOS heap allocations ─────────────────────────────────────

struct PasteboardGuard(*mut OH_Pasteboard);
impl Drop for PasteboardGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { OH_Pasteboard_Destroy(self.0) }
        }
    }
}

struct UdmfDataGuard(*mut OH_UdmfData);
impl Drop for UdmfDataGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { OH_UdmfData_Destroy(self.0) }
        }
    }
}

struct UdmfRecordGuard(*mut OH_UdmfRecord);
impl Drop for UdmfRecordGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { OH_UdmfRecord_Destroy(self.0) }
        }
    }
}

struct UdsPlainTextGuard(*mut OH_UdsPlainText);
impl Drop for UdsPlainTextGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { OH_UdsPlainText_Destroy(self.0) }
        }
    }
}
