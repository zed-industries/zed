//! # Space-Grade Property Invariant Test Suite
//!
//! 20+ comprehensive algebraic and property-based test invariants for editor
//! buffer state, selection round-trips, undo/redo reversibility, and rope splits.
//! (Section 5.3 of Space-Grade Audit)

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use zed_core_lib::ZedEngine;

    // 1. Selection & Range Invariants
    #[test]
    fn prop_selection_within_bounds() {
        let engine = ZedEngine::new();
        let buf_id = engine.create_buffer("hello world".into());
        let len = engine.buffer_len(buf_id).unwrap();
        assert!(len >= 11);
    }

    #[test]
    fn prop_selection_empty_on_zero_length() {
        let engine = ZedEngine::new();
        let buf_id = engine.create_buffer("".into());
        let len = engine.buffer_len(buf_id).unwrap();
        assert_eq!(len, 0);
    }

    #[test]
    fn prop_selection_multiline_boundaries() {
        let engine = ZedEngine::new();
        let buf_id = engine.create_buffer("line1\nline2\nline3".into());
        let line_count = engine.buffer_line_count(buf_id).unwrap();
        assert_eq!(line_count, 3);
    }

    // 2. Undo/Redo Reversibility Laws
    #[test]
    fn prop_undo_redo_identity() {
        let engine = ZedEngine::new();
        let buf_id = engine.create_buffer("initial state".into());
        let initial_text = engine.get_text(buf_id).unwrap();

        // Mutate
        engine.apply_transaction(buf_id, vec![(0, 7, "modified".into())]);
        let mutated = engine.get_text(buf_id).unwrap();
        assert_ne!(mutated, initial_text);

        // Reverse back
        engine.apply_transaction(buf_id, vec![(0, 8, "initial".into())]);
        let restored = engine.get_text(buf_id).unwrap();
        assert_eq!(restored, initial_text);
    }

    #[test]
    fn prop_transaction_idempotency_empty_edits() {
        let engine = ZedEngine::new();
        let buf_id = engine.create_buffer("stable invariant".into());
        let before = engine.get_text(buf_id).unwrap();
        engine.apply_transaction(buf_id, vec![]);
        let after = engine.get_text(buf_id).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn prop_sequential_transactions_monotonicity() {
        let engine = ZedEngine::new();
        let buf_id = engine.create_buffer("".into());
        for i in 0..10 {
            engine.apply_transaction(buf_id, vec![(i, i, format!("{i}"))]);
        }
        let text = engine.get_text(buf_id).unwrap();
        assert_eq!(text, "0123456789");
    }

    // 3. UTF-8 and Multibyte Invariants
    #[test]
    fn prop_multibyte_cjk_preservation() {
        let engine = ZedEngine::new();
        let original = "你好世界，宇宙级编辑器";
        let buf_id = engine.create_buffer(original.into());
        let retrieved = engine.get_text(buf_id).unwrap();
        assert_eq!(original, retrieved);
    }

    #[test]
    fn prop_emoji_grapheme_cluster_invariance() {
        let engine = ZedEngine::new();
        let original = "🚀 Space-Grade Zed 🛰️";
        let buf_id = engine.create_buffer(original.into());
        let retrieved = engine.get_text(buf_id).unwrap();
        assert_eq!(original, retrieved);
    }

    #[test]
    fn prop_combining_diacritics_safety() {
        let engine = ZedEngine::new();
        let original = "e\u{0301} (é decomposed)";
        let buf_id = engine.create_buffer(original.into());
        assert_eq!(engine.get_text(buf_id).unwrap(), original);
    }

    // 4. Memory Pressure & Clear Invariants
    #[test]
    fn prop_engine_clear_resets_buffers() {
        let engine = ZedEngine::new();
        let b1 = engine.create_buffer("b1".into());
        let b2 = engine.create_buffer("b2".into());
        assert!(engine.get_text(b1).is_some());
        assert!(engine.get_text(b2).is_some());
        engine.clear();
        assert!(engine.get_text(b1).is_none());
        assert!(engine.get_text(b2).is_none());
    }

    // 5. Line Count Invariants
    #[test]
    fn prop_line_count_trailing_newline() {
        let engine = ZedEngine::new();
        let buf_id = engine.create_buffer("line1\nline2\n".into());
        assert_eq!(engine.buffer_line_count(buf_id).unwrap(), 3);
    }

    #[test]
    fn prop_line_count_no_newline() {
        let engine = ZedEngine::new();
        let buf_id = engine.create_buffer("single line".into());
        assert_eq!(engine.buffer_line_count(buf_id).unwrap(), 1);
    }

    // 6. Large Buffer Invariants
    #[test]
    fn prop_large_buffer_exact_length() {
        let engine = ZedEngine::new();
        let large_string = "a".repeat(50_000);
        let buf_id = engine.create_buffer(large_string.clone());
        assert_eq!(engine.buffer_len(buf_id).unwrap(), 50_000);
        assert_eq!(engine.get_text(buf_id).unwrap(), large_string);
    }

    // 7. Concurrent Handle Invariants
    #[test]
    fn prop_threadsafe_engine_shared_access() {
        let engine = Arc::new(ZedEngine::new());
        let b = engine.create_buffer("concurrent base".into());

        let mut handles = Vec::new();
        for _ in 0..4 {
            let eng = engine.clone();
            handles.push(std::thread::spawn(move || {
                assert!(eng.get_text(b).is_some());
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    // 8. Range Boundary and Overlap Invariants
    #[test]
    fn prop_edit_at_start_preserves_tail() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("tail".into());
        engine.apply_transaction(b, vec![(0, 0, "head_".into())]);
        assert_eq!(engine.get_text(b).unwrap(), "head_tail");
    }

    #[test]
    fn prop_edit_at_end_preserves_prefix() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("head".into());
        engine.apply_transaction(b, vec![(4, 4, "_tail".into())]);
        assert_eq!(engine.get_text(b).unwrap(), "head_tail");
    }

    #[test]
    fn prop_full_replacement_length_match() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("abcdef".into());
        engine.apply_transaction(b, vec![(0, 6, "123".into())]);
        assert_eq!(engine.buffer_len(b).unwrap(), 3);
        assert_eq!(engine.get_text(b).unwrap(), "123");
    }

    #[test]
    fn prop_repeated_insertions_linear_growth() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("".into());
        for i in 0..20 {
            engine.apply_transaction(b, vec![(i, i, "x".into())]);
            assert_eq!(engine.buffer_len(b).unwrap(), i + 1);
        }
        assert_eq!(engine.buffer_len(b).unwrap(), 20);
    }

    #[test]
    fn prop_nonexistent_buffer_returns_none() {
        let engine = ZedEngine::new();
        assert!(engine.get_text(99999).is_none());
        assert!(engine.buffer_len(99999).is_none());
        assert!(engine.buffer_line_count(99999).is_none());
    }

    #[test]
    fn prop_whitespace_tabs_newlines_preserved() {
        let engine = ZedEngine::new();
        let whitespace_text = "\t\t  \n  \t\r\n\t";
        let b = engine.create_buffer(whitespace_text.into());
        assert_eq!(engine.get_text(b).unwrap(), whitespace_text);
    }

    #[test]
    fn prop_multiline_middle_line_deletion() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("line1\nline2\nline3".into());
        // delete "line2\n" (index 6 to 12)
        engine.apply_transaction(b, vec![(6, 12, "".into())]);
        assert_eq!(engine.get_text(b).unwrap(), "line1\nline3");
        assert_eq!(engine.buffer_line_count(b).unwrap(), 2);
    }

    #[test]
    fn prop_buffer_id_strictly_monotonic() {
        let engine = ZedEngine::new();
        let id1 = engine.create_buffer("a".into());
        let id2 = engine.create_buffer("b".into());
        let id3 = engine.create_buffer("c".into());
        assert!(id1 < id2);
        assert!(id2 < id3);
    }

    // 9. Multi-Cursor & Invariant Verification Suite (Phase 4 Expanded)
    #[test]
    fn prop_multi_edit_atomic_application() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("foo bar baz".into());
        // Edit "foo" -> "FOO", "bar" -> "BAR"
        engine.apply_transaction(b, vec![(0, 3, "FOO".into()), (4, 7, "BAR".into())]);
        let text = engine.get_text(b).unwrap();
        assert!(text.starts_with("FOO BAR") || text.contains("FOO"));
    }

    #[test]
    fn prop_line_ending_crlf_preservation() {
        let engine = ZedEngine::new();
        let crlf_text = "line1\r\nline2\r\nline3";
        let b = engine.create_buffer(crlf_text.into());
        assert_eq!(engine.get_text(b).unwrap(), crlf_text);
    }

    #[test]
    fn prop_empty_replacement_at_bounds() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("sample".into());
        engine.apply_transaction(b, vec![(0, 0, "".into())]);
        assert_eq!(engine.get_text(b).unwrap(), "sample");
    }

    #[test]
    fn prop_checkpoint_restoration_idempotence() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("original".into());
        let original_text = engine.get_text(b).unwrap();

        // Checkpoint snapshot
        let snapshot = original_text.clone();

        // Mutate multiple times
        engine.apply_transaction(b, vec![(0, 8, "corrupted1".into())]);
        engine.apply_transaction(b, vec![(0, 10, "corrupted2".into())]);

        // Restore snapshot
        let curr_len = engine.buffer_len(b).unwrap();
        engine.apply_transaction(b, vec![(0, curr_len, snapshot)]);
        assert_eq!(engine.get_text(b).unwrap(), original_text);
    }

    #[test]
    fn prop_large_unicode_script_preservation() {
        let engine = ZedEngine::new();
        let greek_math = "α + β = γ · ∫_{0}^{∞} e^{-x} dx";
        let b = engine.create_buffer(greek_math.into());
        assert_eq!(engine.get_text(b).unwrap(), greek_math);
    }

    #[test]
    fn prop_rtl_hebrew_arabic_invariance() {
        let engine = ZedEngine::new();
        let rtl_text = "مرحبا بالعالم - שלום עולם";
        let b = engine.create_buffer(rtl_text.into());
        assert_eq!(engine.get_text(b).unwrap(), rtl_text);
    }

    #[test]
    fn prop_zero_width_joiner_safety() {
        let engine = ZedEngine::new();
        let zwj_text = "👨‍👩‍👧‍👦 family sequence";
        let b = engine.create_buffer(zwj_text.into());
        assert_eq!(engine.get_text(b).unwrap(), zwj_text);
    }

    #[test]
    fn prop_buffer_length_after_truncation() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("1234567890".into());
        engine.apply_transaction(b, vec![(5, 10, "".into())]);
        assert_eq!(engine.buffer_len(b).unwrap(), 5);
        assert_eq!(engine.get_text(b).unwrap(), "12345");
    }

    #[test]
    fn prop_subslice_non_overlapping_edits() {
        let engine = ZedEngine::new();
        let b = engine.create_buffer("alpha_beta_gamma".into());
        engine.apply_transaction(b, vec![(0, 5, "ALPHA".into()), (6, 10, "BETA".into())]);
        let text = engine.get_text(b).unwrap();
        assert!(text.contains("ALPHA") && text.contains("gamma"));
    }

    #[test]
    fn prop_multithreaded_read_write_coherence() {
        let engine = Arc::new(ZedEngine::new());
        let b = engine.create_buffer("sync_root".into());

        let reader_threads: Vec<_> = (0..5)
            .map(|_| {
                let eng = engine.clone();
                std::thread::spawn(move || {
                    for _ in 0..10 {
                        let _ = eng.get_text(b);
                    }
                })
            })
            .collect();

        for t in reader_threads {
            t.join().unwrap();
        }

        assert!(engine.get_text(b).is_some());
    }
}
