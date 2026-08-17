#![allow(clippy::large_stack_arrays)]

mod tests {
    use jsonschema::{Draft, PatternOptions};
    #[cfg(not(target_arch = "wasm32"))]
    use std::env;
    #[cfg(not(target_arch = "wasm32"))]
    use std::fs;
    use testsuite::{suite, Test};

    #[suite(
    path = "crates/jsonschema/tests/suite",
    drafts = [
        "draft4",
        "draft6",
        "draft7",
        "draft2019-09",
        "draft2020-12",
    ],
    xfail = [
        "draft4::optional::bignum::integer::a_bignum_is_an_integer",
        "draft4::optional::bignum::integer::a_negative_bignum_is_an_integer",
    ]
)]
    fn test_suite(test: &Test) {
        enum RegexEngine {
            Regex,
            FancyRegex,
        }

        let mut options = jsonschema::options();
        match test.draft {
            "draft4" => {
                options = options.with_draft(Draft::Draft4);
            }
            "draft6" => {
                options = options.with_draft(Draft::Draft6);
            }
            "draft7" => {
                options = options.with_draft(Draft::Draft7);
            }
            "draft2019-09" | "draft2020-12" => {}
            _ => panic!("Unsupported draft"),
        }
        if should_skip_draft(test.draft) {
            return;
        }
        if test.is_optional {
            options = options.should_validate_formats(true);
        }
        options = options.with_retriever(testsuite_retriever());

        for engine in [RegexEngine::FancyRegex, RegexEngine::Regex] {
            match engine {
                RegexEngine::Regex => {
                    options = options.with_pattern_options(PatternOptions::regex());
                }
                RegexEngine::FancyRegex => {
                    options = options.with_pattern_options(PatternOptions::fancy_regex());
                }
            }
            let validator = options
                .build(&test.schema)
                .expect("Failed to build a schema");

            if test.valid {
                if let Some(first) = validator.iter_errors(&test.data).next() {
                    panic!(
                    "Test case should not have validation errors:\nGroup: {}\nTest case: {}\nSchema: {}\nInstance: {}\nError: {first:?}",
                    test.case,
                    test.description,
                    pretty_json(&test.schema),
                    pretty_json(&test.data),
                );
                }
                assert!(
                    validator.is_valid(&test.data),
                    "Test case should be valid:\nCase: {}\nTest: {}\nSchema: {}\nInstance: {}",
                    test.case,
                    test.description,
                    pretty_json(&test.schema),
                    pretty_json(&test.data),
                );
                assert!(
                    validator.validate(&test.data).is_ok(),
                    "Test case should be valid:\nCase: {}\nTest: {}\nSchema: {}\nInstance: {}",
                    test.case,
                    test.description,
                    pretty_json(&test.schema),
                    pretty_json(&test.data),
                );
                let evaluation = validator.evaluate(&test.data);
                assert!(
                    evaluation.flag().valid,
                    "Evaluation output should be valid:\nCase: {}\nTest: {}\nSchema: {}\nInstance: {}",
                    test.case,
                    test.description,
                    pretty_json(&test.schema),
                    pretty_json(&test.data),
                );
                let _ =
                    serde_json::to_value(evaluation.list()).expect("List output should serialize");
                let _ = serde_json::to_value(evaluation.hierarchical())
                    .expect("Hierarchical output should serialize");
            } else {
                let errors = validator.iter_errors(&test.data).collect::<Vec<_>>();
                assert!(
                !errors.is_empty(),
                "Test case should have validation errors:\nCase: {}\nTest: {}\nSchema: {}\nInstance: {}",
                test.case,
                test.description,
                pretty_json(&test.schema),
                pretty_json(&test.data),
            );
                for error in errors {
                    let pointer = error.instance_path().as_str();
                    assert_eq!(
                    test.data.pointer(pointer), Some(error.instance().as_ref()),
                    "Expected error instance did not match actual error instance:\nCase: {}\nTest: {}\nSchema: {}\nInstance: {}\nExpected pointer: {:#?}\nActual pointer: {:#?}",
                    test.case,
                    test.description,
                    pretty_json(&test.schema),
                    pretty_json(&test.data),
                    error.instance().as_ref(),
                    &pointer,
                );
                }
                assert!(
                    !validator.is_valid(&test.data),
                    "Test case should be invalid:\nCase: {}\nTest: {}\nSchema: {}\nInstance: {}",
                    test.case,
                    test.description,
                    pretty_json(&test.schema),
                    pretty_json(&test.data),
                );
                let Some(error) = validator.validate(&test.data).err() else {
                    panic!(
                    "Test case should be invalid:\nCase: {}\nTest: {}\nSchema: {}\nInstance: {}",
                    test.case,
                    test.description,
                    pretty_json(&test.schema),
                    pretty_json(&test.data),
                );
                };
                let pointer = error.instance_path().as_str();
                assert_eq!(
                test.data.pointer(pointer), Some(error.instance().as_ref()),
                "Expected error instance did not match actual error instance:\nCase: {}\nTest: {}\nSchema: {}\nInstance: {}\nExpected pointer: {:#?}\nActual pointer: {:#?}",
                test.case,
                test.description,
                pretty_json(&test.schema),
                pretty_json(&test.data),
                error.instance().as_ref(),
                &pointer,
            );
                let evaluation = validator.evaluate(&test.data);
                assert!(
                    !evaluation.flag().valid,
                    "Evaluation output should be invalid:\nCase: {}\nTest: {}\nSchema: {}\nInstance: {}",
                    test.case,
                    test.description,
                    pretty_json(&test.schema),
                    pretty_json(&test.data),
                );
                let _ =
                    serde_json::to_value(evaluation.list()).expect("List output should serialize");
                let _ = serde_json::to_value(evaluation.hierarchical())
                    .expect("Hierarchical output should serialize");
            }
        }
    }

    fn pretty_json(v: &serde_json::Value) -> String {
        serde_json::to_string_pretty(v).expect("Failed to format JSON")
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_instance_path() {
        let expectations: serde_json::Value =
            serde_json::from_str(include_str!("draft7_instance_paths.json")).expect("Valid JSON");
        for (filename, expected) in expectations.as_object().expect("Is object") {
            let test_file = fs::read_to_string(format!("tests/suite/tests/draft7/{filename}"))
                .unwrap_or_else(|_| panic!("Valid file: {filename}"));
            let data: serde_json::Value = serde_json::from_str(&test_file).expect("Valid JSON");
            for item in expected.as_array().expect("Is array") {
                let suite_id = usize::try_from(item["suite_id"].as_u64().expect("Is integer"))
                    .expect("suite_id fits in usize");
                let schema = &data[suite_id]["schema"];
                let validator = jsonschema::options()
                    .with_draft(Draft::Draft7)
                    .with_retriever(testsuite_retriever())
                    .build(schema)
                    .unwrap_or_else(|_| {
                        panic!(
                    "Valid schema. File: {filename}; Suite ID: {suite_id}; Schema: {schema}",
                )
                    });
                for test_data in item["tests"].as_array().expect("Valid array") {
                    let test_id = usize::try_from(test_data["id"].as_u64().expect("Is integer"))
                        .expect("test_id fits in usize");
                    let mut instance_path = String::new();

                    for segment in test_data["instance_path"].as_array().expect("Valid array") {
                        instance_path.push('/');
                        instance_path.push_str(segment.as_str().expect("A string"));
                    }
                    let instance = &data[suite_id]["tests"][test_id]["data"];
                    let error = validator.validate(instance).expect_err(&format!(
                        "\nFile: {}\nSuite: {}\nTest: {}",
                        filename,
                        &data[suite_id]["description"],
                        &data[suite_id]["tests"][test_id]["description"],
                    ));
                    assert_eq!(
                        error.instance_path().as_str(),
                        instance_path,
                        "\nFile: {}\nSuite: {}\nTest: {}\nError: {}",
                        filename,
                        &data[suite_id]["description"],
                        &data[suite_id]["tests"][test_id]["description"],
                        &error
                    );
                }
            }
        }
    }

    fn should_skip_draft(draft: &str) -> bool {
        if let Some(filter) = allowed_draft_filter() {
            for entry in filter.split(',') {
                if entry.trim().eq_ignore_ascii_case(draft) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn allowed_draft_filter() -> Option<String> {
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(value) = env::var("JSONSCHEMA_SUITE_DRAFT_FILTER") {
            return Some(value);
        }
        option_env!("JSONSCHEMA_SUITE_DRAFT_FILTER").map(str::to_string)
    }
}
