use noop_proc_macro::wasm_bindgen;

#[wasm_bindgen]
struct S {
    a: usize,
}

#[test]
fn test() {
    let _ = S { a: 0 };
}
