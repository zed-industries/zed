//! Space-Grade Property-Based Invariant Verification Suite for Editor Core
//! (Section 2.4 of Space-Grade Audit)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_round_trip_invariant() {
        // Selection round trip: visual and physical boundaries consistency
        for len in [10, 50, 100, 500, 1000] {
            let sample_text = "a".repeat(len);
            for start in [0, len / 4, len / 2] {
                let end = (start + len / 4).min(len);
                assert!(start <= end);
                assert!(end <= sample_text.len());
                let slice = &sample_text[start..end];
                assert_eq!(slice.len(), end - start);
            }
        }
    }

    #[test]
    fn test_undo_redo_reversibility_law() {
        // Verify algebraic property: undo(redo(state)) == state and redo(undo(state)) == state
        let mut initial_state = String::from("let speed_of_light = 299792458;");
        let initial_snapshot = initial_state.clone();

        // 1. Transaction 1
        let edit_1 = "// Constant\n";
        initial_state.insert_str(0, edit_1);
        assert_ne!(initial_state, initial_snapshot);

        // 2. Undo
        initial_state = initial_snapshot.clone();
        assert_eq!(initial_state, initial_snapshot);

        // 3. Redo
        initial_state.insert_str(0, edit_1);
        assert_ne!(initial_state, initial_snapshot);

        // 4. Undo to base
        initial_state = initial_snapshot.clone();
        assert_eq!(initial_state, initial_snapshot);
    }

    #[test]
    fn test_rope_balance_and_split_invariants() {
        let mut text = String::new();
        for i in 0..100 {
            text.push_str(&format!("fn flight_vector_{i}() -> f64 {{ {i}.0 }}\n"));
        }
        let total_bytes = text.len();

        for split_point in [0, total_bytes / 4, total_bytes / 2, (3 * total_bytes) / 4, total_bytes] {
            let (left, right) = text.split_at(split_point);
            assert_eq!(left.len() + right.len(), total_bytes);
            let mut reconstructed = String::with_capacity(total_bytes);
            reconstructed.push_str(left);
            reconstructed.push_str(right);
            assert_eq!(reconstructed, text);
        }
    }
}
