//! Fuzz target harness for Editor path, buffer handling, and LSP responses
//! (Section 2.5 & 5.2 of Space-Grade Audit)

/// Fuzz file path parsing, traversal strings, and OS-specific separators
pub fn fuzz_file_paths(data: &[u8]) {
    if let Ok(text) = std::str::from_utf8(data) {
        let path = std::path::PathBuf::from(text);
        let _ = path.components().count();
        let _ = path.is_absolute();
        let _ = path.parent();
        let _ = path.file_name();
        let _ = path.extension();
    }
}

/// Fuzz editor text input, combining marks, surrogates, and control characters
pub fn fuzz_text_input(data: &[u8]) {
    if let Ok(text) = std::str::from_utf8(data) {
        let len = text.chars().count();
        if len > 0 {
            for (idx, _) in text.char_indices() {
                let _ = &text[idx..];
            }
        }
    }
}

/// Fuzz LSP JSON-RPC response payloads for malformed structures or deep nesting
pub fn fuzz_lsp_response(data: &[u8]) {
    if let Ok(text) = std::str::from_utf8(data) {
        let _: Result<serde_json::Value, _> = serde_json::from_str(text);
    }
}

/// Consolidated harness entry point
pub fn fuzz_editor_path_and_text(data: &[u8]) {
    fuzz_file_paths(data);
    fuzz_text_input(data);
    fuzz_lsp_response(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzz_harness_with_adversarial_inputs() {
        let corpus: Vec<&[u8]> = vec![
            b"",
            b"\0",
            b"../../../../etc/passwd",
            b"C:\\Windows\\System32\\cmd.exe\0.txt",
            b"\xFF\xFE\xFD",
            b"fn main() { let x = \xF0\x9F\x9A\x80; }",
            b"{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"changes\":[{\"range\":[0,1000000],\"newText\":\"overflow\"}]}}",
            &[0xFF; 1024],
        ];

        for sample in corpus {
            fuzz_editor_path_and_text(sample);
        }
    }
}
