// Some streaming edit problems can be tested the old-fashioned way!

// 1. Parser must handle chunks split ANYWHERE
#[gpui::test(iterations = 100)]
fn test_parser_random_chunks(mut rng: StdRng) {
    let input = "<old_text>hello world</old_text><new_text>goodbye</new_text>";

    // Generate random chunk boundaries
    let mut chunks = vec![];
    let mut pos = 0;
    while pos < input.len() {
        let chunk_size = rng.gen_range(1..=10);
        let end = (pos + chunk_size).min(input.len());
        chunks.push(&input[pos..end]);
        pos = end;
    }

    // Parser MUST handle any chunking
    let mut parser = EditParser::new();
    let events: Vec<_> = chunks.iter().flat_map(|chunk| parser.push(chunk)).collect();

    assert_eq!(
        events,
        vec![Event::OldText("hello world"), Event::NewText("goodbye")]
    );
}

// 2. Fuzzy matcher algorithm (without LLM input)
#[test]
fn test_fuzzy_match_algorithm() {
    let buffer = "fn calculate_price() {\n    // TODO\n}";
    let query = "fn  calculate_price()  {"; // Extra spaces

    let matcher = FuzzyMatcher::new(buffer);
    let range = matcher.find(query);

    assert_eq!(range, Some(0..24)); // Found despite whitespace
}

// 3. Streaming diff computes edits incrementally
#[test]
fn test_streaming_diff() {
    let old_text = "fn calculate() {\n    todo!()\n}";
    let mut diff = StreamingDiff::new(old_text);

    // Simulate new text arriving in chunks
    let ops1 = diff.push_new("fn calc");
    assert_eq!(
        ops1,
        vec![
        CharOp::Keep(7),  // "fn calc"
    ]
    );

    let ops2 = diff.push_new("ulate_total(");
    assert_eq!(
        ops2,
        vec![
            CharOp::Insert("_total"), // Insert "_total"
            CharOp::Keep(5),          // "ulate"
            CharOp::Delete(2),        // Remove "()"
            CharOp::Keep(1),          // "("
        ]
    );

    let ops3 = diff.push_new("items: &[Item]) {\n    items.iter().sum()\n}");
    assert_eq!(
        ops3,
        vec![
            CharOp::Insert("items: &[Item]"),
            CharOp::Keep(4),    // ") {\n"
            CharOp::Delete(10), // Remove "    todo!()"
            CharOp::Insert("    items.iter().sum()"),
            CharOp::Keep(2), // "\n}"
        ]
    );

    // The magic: we computed a valid diff while text was still arriving!
}
