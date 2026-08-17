#[test]
fn test_ex2_1_seq_scalars() {
    let mut v = str_to_test_events(EX2_1).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_2_mapping_scalars_to_scalars() {
    let mut v = str_to_test_events(EX2_2).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_3_mapping_scalars_to_sequences() {
    let mut v = str_to_test_events(EX2_3).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_4_sequence_of_mappings() {
    let mut v = str_to_test_events(EX2_4).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_5_sequence_of_sequences() {
    let mut v = str_to_test_events(EX2_5).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_6_mapping_of_mappings() {
    let mut v = str_to_test_events(EX2_6).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_7_two_documents_in_a_stream() {
    let mut v = str_to_test_events(EX2_7).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_8_play_by_play_feed() {
    let mut v = str_to_test_events(EX2_8).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_9_single_document_with_two_comments() {
    let mut v = str_to_test_events(EX2_9).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_10_simple_anchor() {
    let mut v = str_to_test_events(EX2_10).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_11_mapping_between_sequences() {
    let mut v = str_to_test_events(EX2_11).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_12_compact_nested_mapping() {
    let mut v = str_to_test_events(EX2_12).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_13_in_literals_newlines_are_preserved() {
    let mut v = str_to_test_events(EX2_13).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_14_in_folded_scalars_newlines_become_spaces() {
    let mut v = str_to_test_events(EX2_14).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_15_folded_newlines_are_preserved_for_more_indented_and_blank_lines() {
    let mut v = str_to_test_events(EX2_15).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_16_indentation_determines_scope() {
    let mut v = str_to_test_events(EX2_16).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_17_quoted_scalars() {
    let mut v = str_to_test_events(EX2_17).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_18_multi_line_flow_scalars() {
    let mut v = str_to_test_events(EX2_18).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_23_various_explicit_tags() {
    let mut v = str_to_test_events(EX2_23).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_24_global_tags() {
    let mut v = str_to_test_events(EX2_24).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_25_unordered_sets() {
    let mut v = str_to_test_events(EX2_25).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_26_ordered_mappings() {
    let mut v = str_to_test_events(EX2_26).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_27_invoice() {
    let mut v = str_to_test_events(EX2_27).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex2_28_log_file() {
    let mut v = str_to_test_events(EX2_28).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex5_3_block_structure_indicators() {
    let mut v = str_to_test_events(EX5_3).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex5_4_flow_structure_indicators() {
    let mut v = str_to_test_events(EX5_4).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex5_6_node_property_indicators() {
    let mut v = str_to_test_events(EX5_6).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex5_7_block_scalar_indicators() {
    let mut v = str_to_test_events(EX5_7).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex5_8_quoted_scalar_indicators() {
    let mut v = str_to_test_events(EX5_8).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex5_11_line_break_characters() {
    let mut v = str_to_test_events(EX5_11).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex5_12_tabs_and_spaces() {
    let mut v = str_to_test_events(EX5_12).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex5_13_escaped_characters() {
    let mut v = str_to_test_events(EX5_13).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_1_indentation_spaces() {
    let mut v = str_to_test_events(EX6_1).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_2_indentation_indicators() {
    let mut v = str_to_test_events(EX6_2).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_3_separation_spaces() {
    let mut v = str_to_test_events(EX6_3).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_4_line_prefixes() {
    let mut v = str_to_test_events(EX6_4).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_5_empty_lines() {
    let mut v = str_to_test_events(EX6_5).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_6_line_folding() {
    let mut v = str_to_test_events(EX6_6).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_7_block_folding() {
    let mut v = str_to_test_events(EX6_7).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_8_flow_folding() {
    let mut v = str_to_test_events(EX6_8).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_9_separated_comment() {
    let mut v = str_to_test_events(EX6_9).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_12_separation_spaces_ii() {
    let mut v = str_to_test_events(EX6_12).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_13_reserved_directives() {
    let mut v = str_to_test_events(EX6_13).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_14_yaml_directive() {
    let mut v = str_to_test_events(EX6_14).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_16_tag_directive() {
    let mut v = str_to_test_events(EX6_16).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_18_primary_tag_handle() {
    let mut v = str_to_test_events(EX6_18).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_19_secondary_tag_handle() {
    let mut v = str_to_test_events(EX6_19).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_20_tag_handles() {
    let mut v = str_to_test_events(EX6_20).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_21_local_tag_prefix() {
    let mut v = str_to_test_events(EX6_21).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_22_global_tag_prefix() {
    let mut v = str_to_test_events(EX6_22).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_23_node_properties() {
    let mut v = str_to_test_events(EX6_23).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_24_verbatim_tags() {
    let mut v = str_to_test_events(EX6_24).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_26_tag_shorthands() {
    let mut v = str_to_test_events(EX6_26).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_28_non_specific_tags() {
    let mut v = str_to_test_events(EX6_28).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex6_29_node_anchors() {
    let mut v = str_to_test_events(EX6_29).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_1_alias_nodes() {
    let mut v = str_to_test_events(EX7_1).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[allow(dead_code)]
fn test_ex7_2_empty_nodes() {
    let mut v = str_to_test_events(EX7_2).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_3_completely_empty_nodes() {
    let mut v = str_to_test_events(EX7_3).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_4_double_quoted_implicit_keys() {
    let mut v = str_to_test_events(EX7_4).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_5_double_quoted_line_breaks() {
    let mut v = str_to_test_events(EX7_5).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_6_double_quoted_lines() {
    let mut v = str_to_test_events(EX7_6).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_7_single_quoted_characters() {
    let mut v = str_to_test_events(EX7_7).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_8_single_quoted_implicit_keys() {
    let mut v = str_to_test_events(EX7_8).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_9_single_quoted_lines() {
    let mut v = str_to_test_events(EX7_9).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[allow(dead_code)]
fn test_ex7_10_plain_characters() {
    let mut v = str_to_test_events(EX7_10).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_11_plain_implicit_keys() {
    let mut v = str_to_test_events(EX7_11).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_12_plain_lines() {
    let mut v = str_to_test_events(EX7_12).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_13_flow_sequence() {
    let mut v = str_to_test_events(EX7_13).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_14_flow_sequence_entries() {
    let mut v = str_to_test_events(EX7_14).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_15_flow_mappings() {
    let mut v = str_to_test_events(EX7_15).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_16_flow_mapping_entries() {
    let mut v = str_to_test_events(EX7_16).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[allow(dead_code)]
fn test_ex7_17_flow_mapping_separate_values() {
    let mut v = str_to_test_events(EX7_17).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_18_flow_mapping_adjacent_values() {
    let mut v = str_to_test_events(EX7_18).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_19_single_pair_flow_mappings() {
    let mut v = str_to_test_events(EX7_19).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_20_single_pair_explicit_entry() {
    let mut v = str_to_test_events(EX7_20).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[allow(dead_code)]
fn test_ex7_21_single_pair_implicit_entries() {
    let mut v = str_to_test_events(EX7_21).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_23_flow_content() {
    let mut v = str_to_test_events(EX7_23).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex7_24_flow_nodes() {
    let mut v = str_to_test_events(EX7_24).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnAlias);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_1_block_scalar_header() {
    let mut v = str_to_test_events(EX8_1).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[allow(dead_code)]
fn test_ex8_2_block_indentation_header() {
    let mut v = str_to_test_events(EX8_2).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_4_chomping_final_line_break() {
    let mut v = str_to_test_events(EX8_4).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_6_empty_scalar_chomping() {
    let mut v = str_to_test_events(EX8_6).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_7_literal_scalar() {
    let mut v = str_to_test_events(EX8_7).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_8_literal_content() {
    let mut v = str_to_test_events(EX8_8).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_9_folded_scalar() {
    let mut v = str_to_test_events(EX8_9).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_10_folded_lines() {
    let mut v = str_to_test_events(EX8_10).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_11_more_indented_lines() {
    let mut v = str_to_test_events(EX8_11).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_12_empty_separation_lines() {
    let mut v = str_to_test_events(EX8_12).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_13_final_empty_lines() {
    let mut v = str_to_test_events(EX8_13).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_14_block_sequence() {
    let mut v = str_to_test_events(EX8_14).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_15_block_sequence_entry_types() {
    let mut v = str_to_test_events(EX8_15).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_16_block_mappings() {
    let mut v = str_to_test_events(EX8_16).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_17_explicit_block_mapping_entries() {
    let mut v = str_to_test_events(EX8_17).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_18_implicit_block_mapping_entries() {
    let mut v = str_to_test_events(EX8_18).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnNull);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_19_compact_block_mappings() {
    let mut v = str_to_test_events(EX8_19).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_20_block_node_types() {
    let mut v = str_to_test_events(EX8_20).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

#[test]
fn test_ex8_22_block_collection_nodes() {
    let mut v = str_to_test_events(EX8_22).into_iter();
    assert_next!(v, TestEvent::OnDocumentStart);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnSequenceEnd);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapStart);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnScalar);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnMapEnd);
    assert_next!(v, TestEvent::OnDocumentEnd);
}

