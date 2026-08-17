use wasm_encoder::*;

#[test]
fn big_type_indices() {
    const N: u32 = 100_000;
    let mut module = Module::new();
    let mut types = TypeSection::new();
    for _ in 0..N {
        types.function([], []);
    }
    module.section(&types);
    let mut funcs = FunctionSection::new();
    funcs.function(N - 1);
    module.section(&funcs);

    let mut elems = ElementSection::new();
    elems.declared(Elements::Functions(&[0]));
    module.section(&elems);

    let mut code = CodeSection::new();
    let mut body = Function::new([]);
    body.instruction(&Instruction::RefFunc(0));
    body.instruction(&Instruction::Drop);
    body.instruction(&Instruction::End);
    code.function(&body);
    module.section(&code);

    let wasm = module.finish();

    wasmparser::Validator::default()
        .validate_all(&wasm)
        .unwrap();
}
