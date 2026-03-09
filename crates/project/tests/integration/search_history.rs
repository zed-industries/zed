use project::search_history::{QueryInsertionBehavior, SearchHistory, SearchHistoryCursor};

#[test]
fn test_add() {
    const MAX_HISTORY_LEN: usize = 20;
    let mut search_history = SearchHistory::new(
        Some(MAX_HISTORY_LEN),
        QueryInsertionBehavior::ReplacePreviousIfContains,
    );
    let mut cursor = SearchHistoryCursor::default();

    assert_eq!(
        search_history.current(&cursor),
        None,
        "No current selection should be set for the default search history"
    );

    search_history.add(&mut cursor, "rust".to_string());
    assert_eq!(
        search_history.current(&cursor),
        Some("rust"),
        "Newly added item should be selected"
    );

    // check if duplicates are not added
    search_history.add(&mut cursor, "rust".to_string());
    assert_eq!(search_history.len(), 1, "Should not add a duplicate");
    assert_eq!(search_history.current(&cursor), Some("rust"));

    // check if new string containing the previous string replaces it
    search_history.add(&mut cursor, "rustlang".to_string());
    assert_eq!(
        search_history.len(),
        1,
        "Should replace previous item if it's a substring"
    );
    assert_eq!(search_history.current(&cursor), Some("rustlang"));

    // add item when it equals to current item if it's not the last one
    search_history.add(&mut cursor, "php".to_string());
    search_history.previous(&mut cursor);
    assert_eq!(search_history.current(&cursor), Some("rustlang"));
    search_history.add(&mut cursor, "rustlang".to_string());
    assert_eq!(search_history.len(), 3, "Should add item");
    assert_eq!(search_history.current(&cursor), Some("rustlang"));

    // push enough items to test SEARCH_HISTORY_LIMIT
    for i in 0..MAX_HISTORY_LEN * 2 {
        search_history.add(&mut cursor, format!("item{i}"));
    }
    assert!(search_history.len() <= MAX_HISTORY_LEN);
}

#[test]
fn test_next_and_previous() {
    let mut search_history = SearchHistory::new(None, QueryInsertionBehavior::AlwaysInsert);
    let mut cursor = SearchHistoryCursor::default();

    assert_eq!(
        search_history.next(&mut cursor),
        None,
        "Default search history should not have a next item"
    );

    search_history.add(&mut cursor, "Rust".to_string());
    assert_eq!(search_history.next(&mut cursor), None);
    search_history.add(&mut cursor, "JavaScript".to_string());
    assert_eq!(search_history.next(&mut cursor), None);
    search_history.add(&mut cursor, "TypeScript".to_string());
    assert_eq!(search_history.next(&mut cursor), None);

    assert_eq!(search_history.current(&cursor), Some("TypeScript"));

    assert_eq!(search_history.previous(&mut cursor), Some("JavaScript"));
    assert_eq!(search_history.current(&cursor), Some("JavaScript"));

    assert_eq!(search_history.previous(&mut cursor), Some("Rust"));
    assert_eq!(search_history.current(&cursor), Some("Rust"));

    assert_eq!(search_history.previous(&mut cursor), None);
    assert_eq!(search_history.current(&cursor), Some("Rust"));

    assert_eq!(search_history.next(&mut cursor), Some("JavaScript"));
    assert_eq!(search_history.current(&cursor), Some("JavaScript"));

    assert_eq!(search_history.next(&mut cursor), Some("TypeScript"));
    assert_eq!(search_history.current(&cursor), Some("TypeScript"));

    assert_eq!(search_history.next(&mut cursor), None);
    assert_eq!(search_history.current(&cursor), Some("TypeScript"));
}

#[test]
fn test_reset_selection() {
    let mut search_history = SearchHistory::new(None, QueryInsertionBehavior::AlwaysInsert);
    let mut cursor = SearchHistoryCursor::default();

    search_history.add(&mut cursor, "Rust".to_string());
    search_history.add(&mut cursor, "JavaScript".to_string());
    search_history.add(&mut cursor, "TypeScript".to_string());

    assert_eq!(search_history.current(&cursor), Some("TypeScript"));
    cursor.reset();
    assert_eq!(search_history.current(&cursor), None);
    assert_eq!(
        search_history.previous(&mut cursor),
        Some("TypeScript"),
        "Should start from the end after reset on previous item query"
    );

    search_history.previous(&mut cursor);
    assert_eq!(search_history.current(&cursor), Some("JavaScript"));
    search_history.previous(&mut cursor);
    assert_eq!(search_history.current(&cursor), Some("Rust"));

    cursor.reset();
    assert_eq!(search_history.current(&cursor), None);
}

#[test]
fn test_multiple_cursors() {
    let mut search_history = SearchHistory::new(None, QueryInsertionBehavior::AlwaysInsert);
    let mut cursor1 = SearchHistoryCursor::default();
    let mut cursor2 = SearchHistoryCursor::default();

    search_history.add(&mut cursor1, "Rust".to_string());
    search_history.add(&mut cursor1, "JavaScript".to_string());
    search_history.add(&mut cursor1, "TypeScript".to_string());

    search_history.add(&mut cursor2, "Python".to_string());
    search_history.add(&mut cursor2, "Java".to_string());
    search_history.add(&mut cursor2, "C++".to_string());

    assert_eq!(search_history.current(&cursor1), Some("TypeScript"));
    assert_eq!(search_history.current(&cursor2), Some("C++"));

    assert_eq!(search_history.previous(&mut cursor1), Some("JavaScript"));
    assert_eq!(search_history.previous(&mut cursor2), Some("Java"));

    assert_eq!(search_history.next(&mut cursor1), Some("TypeScript"));
    assert_eq!(search_history.next(&mut cursor1), Some("Python"));

    cursor1.reset();
    cursor2.reset();

    assert_eq!(search_history.current(&cursor1), None);
    assert_eq!(search_history.current(&cursor2), None);
}
