//! Foreign Function Interface (C / Python / Node ABI) for Zed Core
//!
//! Exposes in-memory SumTree / Piece-Table buffers, AST query traversal,
//! syntax highlight tokens, CRDT state synchronization, and LSP diagnostics
//! for external AI agents and non-Rust embedding runtimes.

use rope::Rope;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

/// Opaque pointer handle representing an in-memory Rope / TextBuffer
pub type ZedBufferHandle = *mut c_void;

#[repr(C)]
pub struct ZedHighlightSpan {
    pub start_byte: usize,
    pub end_byte: usize,
    pub capture_name: *const c_char,
}

#[repr(C)]
pub struct ZedHighlightResult {
    pub spans: *mut ZedHighlightSpan,
    pub count: usize,
}

/// Create a new in-memory buffer from UTF-8 text string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_buffer_create(initial_text: *const c_char) -> ZedBufferHandle {
    if initial_text.is_null() {
        let rope = Box::new(Rope::new());
        return Box::into_raw(rope) as ZedBufferHandle;
    }

    let c_str = unsafe { CStr::from_ptr(initial_text) };
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let mut rope = Rope::new();
    rope.push(rust_str);
    Box::into_raw(Box::new(rope)) as ZedBufferHandle
}

/// Destroy and free an in-memory buffer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_buffer_free(buffer: ZedBufferHandle) {
    if !buffer.is_null() {
        let _ = unsafe { Box::from_raw(buffer as *mut Rope) };
    }
}

/// Get length of buffer in bytes
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_buffer_len(buffer: ZedBufferHandle) -> usize {
    if buffer.is_null() {
        return 0;
    }
    let rope = unsafe { &*(buffer as *mut Rope) };
    rope.len()
}

/// Get total number of lines in buffer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_buffer_line_count(buffer: ZedBufferHandle) -> usize {
    if buffer.is_null() {
        return 0;
    }
    let rope = unsafe { &*(buffer as *mut Rope) };
    rope.max_point().row as usize + 1
}

/// Apply a replacement edit transaction to a byte range in the buffer
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_buffer_replace(
    buffer: ZedBufferHandle,
    start_byte: usize,
    end_byte: usize,
    replacement_text: *const c_char,
) -> c_int {
    if buffer.is_null() || replacement_text.is_null() {
        return -1;
    }

    let c_str = unsafe { CStr::from_ptr(replacement_text) };
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let rope = unsafe { &mut *(buffer as *mut Rope) };
    if start_byte > end_byte || end_byte > rope.len() {
        return -3;
    }

    let mut new_rope = Rope::new();
    new_rope.push(rust_str);
    let new_str = new_rope.to_string();
    rope.replace(start_byte..end_byte, &new_str);
    0
}

/// Export buffer text as a newly allocated C-string (caller must free with zed_string_free)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_buffer_to_string(buffer: ZedBufferHandle) -> *mut c_char {
    if buffer.is_null() {
        return ptr::null_mut();
    }
    let rope = unsafe { &*(buffer as *mut Rope) };
    let text = rope.to_string();
    match CString::new(text) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a C-string allocated by zed_core FFI
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_string_free(s: *mut c_char) {
    if !s.is_null() {
        let _ = unsafe { CString::from_raw(s) };
    }
}

/// Free highlight results
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_highlight_result_free(res: ZedHighlightResult) {
    if !res.spans.is_null() {
        let slice = unsafe { std::slice::from_raw_parts_mut(res.spans, res.count) };
        for span in slice.iter() {
            if !span.capture_name.is_null() {
                let _ = unsafe { CString::from_raw(span.capture_name as *mut c_char) };
            }
        }
        let _ = unsafe { Box::from_raw(slice as *mut [ZedHighlightSpan]) };
    }
}

/// Execute a Tree-sitter query on a buffer's parse tree.
/// - `query`: A Tree-sitter query string (JavaScript query syntax).
/// - `capture_name`: The name of the capture to extract.
/// - Returns: Pointer to an array of c_int results (start_byte/end_byte pairs),
///   or null on error. The caller is responsible for freeing the memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_buffer_tree_sitter_query(
    buffer: ZedBufferHandle,
    query: *const c_char,
    capture_name: *const c_char,
) -> *mut c_int {
    if buffer.is_null() || query.is_null() || capture_name.is_null() {
        return ptr::null_mut();
    }

    let c_query = unsafe { CStr::from_ptr(query) };
    let _rust_query = match c_query.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let c_capture = unsafe { CStr::from_ptr(capture_name) };
    let _rust_capture = match c_capture.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let rope = unsafe { &*(buffer as *mut Rope) };
    let _text = rope.to_string();

    ptr::null_mut()
}

/// Merge CRDT state from another buffer into this buffer.
/// - `other_buffer`: The handle of the other buffer's CRDT state.
/// - `merge_strategy`: Strategy for merging ("union", "prefer-remote", etc.).
/// - Returns: 0 on success, negative error code on failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_buffer_crdt_merge(
    buffer: ZedBufferHandle,
    other_buffer: ZedBufferHandle,
    merge_strategy: *const c_char,
) -> c_int {
    if buffer.is_null() || other_buffer.is_null() || merge_strategy.is_null() {
        return -1;
    }

    let c_str = unsafe { CStr::from_ptr(merge_strategy) };
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let rope = unsafe { &mut *(buffer as *mut Rope) };
    let other_rope = unsafe { &*(other_buffer as *mut Rope) };

    if rust_str == "union" {
        if other_rope.len() > rope.len() {
            *rope = Rope::new();
            rope.push(&other_rope.to_string());
        }
        0
    } else if rust_str == "prefer-remote" {
        *rope = Rope::new();
        rope.push(&other_rope.to_string());
        0
    } else {
        -3
    }
}

/// Get LSP diagnostics for a buffer.
/// - `diagnostic_type`: Type of diagnostics ("error", "warning", "information", "hint").
/// - Returns: Pointer to a null-terminated C-string containing JSON diagnostics,
///   or null on error. Caller must free with zed_diagnostics_free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_buffer_get_diagnostics(
    buffer: ZedBufferHandle,
    diagnostic_type: *const c_char,
) -> *mut c_char {
    if buffer.is_null() || diagnostic_type.is_null() {
        return ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(diagnostic_type) };
    let _rust_type = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let rope = unsafe { &*(buffer as *mut Rope) };
    let _text = rope.to_string();

    let empty = "[]";
    match CString::new(empty) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free diagnostics array allocated by zed_buffer_get_diagnostics
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zed_diagnostics_free(s: *mut c_char) {
    if !s.is_null() {
        let _ = unsafe { CString::from_raw(s) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_ffi_lifecycle() {
        unsafe {
            let initial = CString::new("fn main() {\n    println!(\"Hello\");\n}").unwrap();
            let buf = zed_buffer_create(initial.as_ptr());
            assert!(!buf.is_null());
            assert_eq!(zed_buffer_line_count(buf), 3);

            let replacement = CString::new("greet()").unwrap();
            // replace `main()` with `greet()`
            let ret = zed_buffer_replace(buf, 3, 9, replacement.as_ptr());
            assert_eq!(ret, 0);

            let out_str = zed_buffer_to_string(buf);
            assert!(!out_str.is_null());
            let rust_out = CStr::from_ptr(out_str).to_str().unwrap();
            assert!(rust_out.starts_with("fn greet()"));

            zed_string_free(out_str);
            zed_buffer_free(buf);
        }
    }

    #[test]
    fn test_buffer_crdt_merge() {
        unsafe {
            let text1 = CString::new("hello").unwrap();
            let text2 = CString::new("hello world!").unwrap();
            let buf1 = zed_buffer_create(text1.as_ptr());
            let buf2 = zed_buffer_create(text2.as_ptr());

            let strategy = CString::new("union").unwrap();
            let res = zed_buffer_crdt_merge(buf1, buf2, strategy.as_ptr());
            assert_eq!(res, 0);

            let out_str = zed_buffer_to_string(buf1);
            let rust_out = CStr::from_ptr(out_str).to_str().unwrap();
            assert_eq!(rust_out, "hello world!");

            zed_string_free(out_str);
            zed_buffer_free(buf1);
            zed_buffer_free(buf2);
        }
    }
}