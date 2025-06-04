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

// 3. Indentation adjustment is purely algorithmic
#[test]
fn test_indentation_delta() {
    let old_text = "    fn foo() {"; // 4 spaces
    let new_text = "fn foo() {\n    bar();"; // 0 spaces

    let delta = calculate_indent_delta(old_text, new_text);
    assert_eq!(delta, 4);

    let adjusted = apply_indent_delta(new_text, delta);
    assert_eq!(adjusted, "    fn foo() {\n        bar();");
}
