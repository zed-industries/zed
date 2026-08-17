use std::borrow::Cow;
use wasm_encoder::*;

#[test]
fn big_type_indices() {
    const N: u32 = 100_000;
    let mut module = Module::new();
    let mut types = TypeSection::new();
    for _ in 0..N {
        types.ty().function([], []);
    }
    module.section(&types);
    let mut funcs = FunctionSection::new();
    funcs.function(N - 1);
    module.section(&funcs);

    let mut elems = ElementSection::new();
    elems.declared(Elements::Functions(Cow::Borrowed(&[0])));
    module.section(&elems);

    let mut code = CodeSection::new();
    let mut body = Function::new([]);
    body.instructions().ref_func(0);
    body.instructions().drop();
    body.instructions().end();
    code.function(&body);
    module.section(&code);

    let wasm = module.finish();

    wasmparser::Validator::default()
        .validate_all(&wasm)
        .unwrap();
}

#[test]
fn big_function_body() {
    let mut module = Module::new();

    let mut types = TypeSection::new();
    types.ty().function([], []);
    module.section(&types);
    let mut funcs = FunctionSection::new();
    funcs.function(0);
    module.section(&funcs);

    let mut code = CodeSection::new();
    let mut body = Function::new([]);
    // Function body larger than the 7_654_321-byte implementation
    // limit.
    for _ in 0..8_000_000 {
        body.instructions().unreachable();
    }
    body.instructions().end();
    code.function(&body);
    module.section(&code);

    let wasm = module.finish();

    let result = wasmparser::Validator::default().validate_all(&wasm);
    assert!(result.is_err());
}
