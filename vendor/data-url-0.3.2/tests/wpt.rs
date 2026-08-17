use tester as test;

#[macro_use]
extern crate serde;

fn run_data_url(
    input: String,
    expected_mime: Option<String>,
    expected_body: Option<Vec<u8>>,
    expected_panic: bool,
) {
    let priorhook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |p| {
        if !expected_panic {
            priorhook(p);
        }
    }));
    let url = data_url::DataUrl::process(&input);
    if let Some(expected_mime) = expected_mime {
        let url = url.unwrap();
        let (body, _) = url.decode_to_vec().unwrap();
        if expected_mime.is_empty() {
            assert_eq!(url.mime_type().to_string(), "text/plain;charset=US-ASCII")
        } else {
            assert_eq!(url.mime_type().to_string(), expected_mime)
        }
        if let Some(expected_body) = expected_body {
            assert_eq!(body, expected_body)
        }
    } else if let Ok(url) = url {
        assert!(url.decode_to_vec().is_err(), "{:?}", url.mime_type())
    }
}

fn collect_data_url<F>(add_test: &mut F)
where
    F: FnMut(String, bool, test::TestFn),
{
    let known_failures = ["data://test:test/,X"];

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TestCase {
        Two(String, Option<String>),
        Three(String, Option<String>, Vec<u8>),
    }

    let v: Vec<TestCase> = serde_json::from_str(include_str!("data-urls.json")).unwrap();
    for test in v {
        let (input, expected_mime, expected_body) = match test {
            TestCase::Two(i, m) => (i, m, None),
            TestCase::Three(i, m, b) => (i, m, Some(b)),
        };
        let should_panic = known_failures.contains(&&*input);
        add_test(
            format!("data: URL {:?}", input),
            should_panic,
            test::TestFn::DynTestFn(Box::new(move || {
                run_data_url(input, expected_mime, expected_body, should_panic)
            })),
        );
    }
}

fn run_base64(input: String, expected: Option<Vec<u8>>) {
    let result = data_url::forgiving_base64::decode_to_vec(input.as_bytes());
    match (result, expected) {
        (Ok(bytes), Some(expected)) => assert_eq!(bytes, expected),
        (Ok(bytes), None) => panic!("Expected error, got {:?}", bytes),
        (Err(e), Some(expected)) => panic!("Expected {:?}, got error {:?}", expected, e),
        (Err(_), None) => {}
    }
}

fn collect_base64<F>(add_test: &mut F)
where
    F: FnMut(String, bool, test::TestFn),
{
    let known_failures = [];

    let v: Vec<(String, Option<Vec<u8>>)> =
        serde_json::from_str(include_str!("base64.json")).unwrap();
    for (input, expected) in v {
        let should_panic = known_failures.contains(&&*input);
        add_test(
            format!("base64 {:?}", input),
            should_panic,
            test::TestFn::DynTestFn(Box::new(move || run_base64(input, expected))),
        );
    }
}

fn run_mime(input: String, expected: Option<String>) {
    let result = input.parse::<data_url::mime::Mime>();
    match (result, expected) {
        (Ok(mime), Some(expected)) => assert_eq!(mime.to_string(), expected),
        (Ok(mime), None) => panic!("Expected error, got {:?}", mime),
        (Err(e), Some(expected)) => panic!("Expected {:?}, got error {:?}", expected, e),
        (Err(_), None) => {}
    }
}

fn collect_mime<F>(add_test: &mut F)
where
    F: FnMut(String, bool, test::TestFn),
{
    let known_failures = [];

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Entry {
        Comment(String),
        TestCase {
            input: String,
            output: Option<String>,
        },
    }

    let v: Vec<Entry> = serde_json::from_str(include_str!("mime-types.json")).unwrap();
    let v2: Vec<Entry> = serde_json::from_str(include_str!("generated-mime-types.json")).unwrap();
    let entries = v.into_iter().chain(v2);

    let mut last_comment = None;
    for entry in entries {
        let (input, expected) = match entry {
            Entry::TestCase { input, output } => (input, output),
            Entry::Comment(s) => {
                last_comment = Some(s);
                continue;
            }
        };

        let should_panic = known_failures.contains(&&*input);
        add_test(
            if let Some(ref s) = last_comment {
                format!("MIME type {:?} {:?}", s, input)
            } else {
                format!("MIME type {:?}", input)
            },
            should_panic,
            test::TestFn::DynTestFn(Box::new(move || run_mime(input, expected))),
        );
    }
}

fn main() {
    let mut tests = Vec::new();
    {
        let mut add_one = |name: String, should_panic: bool, run: test::TestFn| {
            let desc = test::TestDesc {
                name: test::DynTestName(name),
                ignore: false,
                should_panic: match should_panic {
                    true => test::ShouldPanic::Yes,
                    false => test::ShouldPanic::No,
                },
                allow_fail: false,
                test_type: test::TestType::Unknown,
            };
            tests.push(test::TestDescAndFn { desc, testfn: run })
        };
        collect_data_url(&mut add_one);
        collect_base64(&mut add_one);
        collect_mime(&mut add_one);
    }
    test::test_main(&std::env::args().collect::<Vec<_>>(), tests, None)
}
