use fancy_regex::{Matches, Regex};

#[cfg(feature = "std")]
use std::sync::Arc;

// Helper functions to ensure types implement Send and Sync
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[test]
fn test_regex_is_send() {
    assert_send::<Regex>();
}

#[test]
fn test_regex_is_sync() {
    assert_sync::<Regex>();
}

#[test]
fn test_matches_is_send() {
    assert_send::<Matches>();
}

#[cfg(all(feature = "std", feature = "variable-lookbehinds"))]
#[test]
fn test_threading_with_arc_and_variable_lookbehinds() {
    // Create a single regex with variable-length lookbehind wrapped in Arc
    let re = Arc::new(Regex::new(r"(?<=ab+)x").unwrap());

    // Test data: different haystacks to match against
    let test_cases = vec![
        ("abbx", true),
        ("abbbx", true),
        ("ax", false),
        ("x", false),
        ("abbbbbbx", true),
    ];

    assert_test_cases(re, test_cases);
}

#[cfg(feature = "std")]
#[test]
fn test_threading_with_arc() {
    // Create a single regex
    let re = Arc::new(Regex::new(r"(?<=ab)x").unwrap());

    // Test data: different haystacks to match against
    let test_cases = vec![
        ("abx", true),
        ("aaabx", true),
        ("ab", false),
        ("aby", false),
        ("bbx", false),
    ];

    assert_test_cases(re, test_cases);
}

#[cfg(all(test, feature = "std"))]
fn assert_test_cases(re: Arc<Regex>, test_cases: Vec<(&str, bool)>) {
    use std::thread;

    // Create multiple threads that share the same regex instance
    let mut handles = vec![];

    for (haystack, expected) in test_cases {
        let re_clone = Arc::clone(&re);
        let haystack_owned = haystack.to_string();
        let handle = thread::spawn(move || {
            // Use the same regex instance in each thread
            let result = re_clone.is_match(&haystack_owned).unwrap();
            (haystack_owned, result)
        });
        handles.push((handle, expected));
    }

    // Collect results from all threads
    for (handle, expected) in handles {
        let (haystack, result) = handle.join().unwrap();
        assert_eq!(
            result, expected,
            "Failed for haystack '{}': expected {}, got {}",
            haystack, expected, result
        );
    }
}
