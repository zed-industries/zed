use noop_proc_macro::Deserialize;
use noop_proc_macro::Serialize;

#[derive(Serialize, Deserialize)]
struct S {
    #[serde(default)]
    a: usize,
}

#[test]
fn test() {
    let _ = S { a: 0 };
}
