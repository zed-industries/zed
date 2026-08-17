use gh_workflow::Workflow;
use pretty_assertions::assert_eq;
use serde_json::Value;

fn split(content: &str) -> (Value, Value) {
    let parsed = Workflow::parse(content).unwrap();
    let actual = serde_yaml::from_str::<Value>(&parsed.to_string().unwrap()).unwrap();
    let expected = serde_yaml::from_str::<Value>(content).unwrap();

    (actual, expected)
}

#[test]
fn test_workflow_bench() {
    let (actual, expected) = split(include_str!("./fixtures/workflow-bench.yml"));
    assert_eq!(actual, expected);
}

#[test]
fn test_workflow_ci() {
    let (actual, expected) = split(include_str!("./fixtures/workflow-ci.yml"));
    assert_eq!(actual, expected);
}

#[test]
fn test_workflow_demo() {
    let (actual, expected) = split(include_str!("./fixtures/workflow-demo.yml"));
    assert_eq!(actual, expected);
}

#[test]
fn test_workflow_rust() {
    let (actual, expected) = split(include_str!("./fixtures/workflow-rust.yml"));
    assert_eq!(actual, expected);
}
