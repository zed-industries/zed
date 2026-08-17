use {
    anyhow::{Context, Result},
    wit_component::StringEncoding,
    wit_parser::Resolve,
};

const FOO: &str = r#"
(module
  (@dylink.0
    (mem-info (memory 4 4))
    (needed "libc.so")
  )
  (type (func))
  (type (func (param i32) (result i32)))
  (import "env" "memory" (memory 1))
  (import "env" "__indirect_function_table" (table 0 funcref))
  (import "env" "__stack_pointer" (global $__stack_pointer (mut i32)))
  (import "env" "__memory_base" (global $__memory_base i32))
  (import "env" "__table_base" (global $__table_base i32))
  (import "env" "malloc" (func $malloc (type 1)))
  (import "env" "abort" (func $abort (type 0)))
  (import "GOT.mem" "um" (global $um (mut i32)))
  (import "test:test/test" "bar" (func $bar (type 1)))
  (func $__wasm_call_ctors (type 0))
  (func $__wasm_apply_data_relocs (type 0))
  (func $foo (type 1) (param i32) (result i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    global.set $__stack_pointer

    i32.const 4
    call $malloc

    i32.const 0
    i32.eq
    if
      call $abort
      unreachable
    end

    local.get 0
    global.get $um
    i32.load offset=16
    i32.add
    i32.const 42
    i32.add

    call $bar

    global.get $__stack_pointer
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )
  (global i32 i32.const 0)
  (export "__wasm_call_ctors" (func $__wasm_call_ctors))
  (export "__wasm_apply_data_relocs" (func $__wasm_apply_data_relocs))
  (export "foo" (func $foo))
  (export "well" (global 4))
  (data $.data (global.get $__memory_base) "\04\00\00\00")
)
"#;

const BAR: &str = r#"
(module
  (@dylink.0
    (mem-info (memory 20 4))
    (needed "libfoo.so")
  )
  (type (func (param i32) (result i32)))
  (type (func))
  (import "env" "memory" (memory 1))
  (import "env" "__indirect_function_table" (table 0 funcref))
  (import "env" "__memory_base" (global $__memory_base i32))
  (import "env" "__table_base" (global $__table_base i32))
  (import "env" "foo" (func $foo (type 0)))
  (import "GOT.mem" "well" (global $well (mut i32)))
  (func $__wasm_call_ctors (type 1))
  (func $__wasm_apply_data_relocs (type 1))
  (func $bar (type 0) (param i32) (result i32)
    local.get 0
    call $foo
    global.get $well
    i32.load
    i32.add
  )
  (global i32 i32.const 0)
  (export "__wasm_call_ctors" (func $__wasm_call_ctors))
  (export "__wasm_apply_data_relocs" (func $__wasm_apply_data_relocs))
  (export "test:test/test#bar" (func $bar))
  (export "um" (global 3))
  (data $.data (global.get $__memory_base) "\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00")
)
"#;

const LIBC: &str = r#"
(module
  (@dylink.0)
  (type (func))
  (type (func (param i32) (result i32)))
  (import "GOT.mem" "__heap_base" (global $__heap_base (mut i32)))
  (import "GOT.mem" "__heap_end" (global $__heap_end (mut i32)))
  (global $heap (mut i32) i32.const 0)
  (func $start (type 0)
    global.get $__heap_base
    global.set $heap
  )
  (func $malloc (type 1) (param i32) (result i32)
    global.get $heap
    global.get $heap
    local.get 0
    i32.add
    global.set $heap
  )
  (func $abort (type 0)
    unreachable
  )
  (export "malloc" (func $malloc))
  (export "abort" (func $abort))
  (start $start)
)
"#;

const WIT: &str = r#"
package test:test;

interface test {
   bar: func(v: s32) -> s32;
}

world bar {
    import test;
    export test;
}
"#;

fn encode(wat: &str, wit: Option<&str>) -> Result<Vec<u8>> {
    let mut module = wat::parse_str(wat)?;

    if let Some(wit) = wit {
        let mut resolve = Resolve::default();
        let pkg = resolve.push_str("test.wit", wit)?;
        let world = resolve.select_world(pkg, None)?;

        wit_component::embed_component_metadata(
            &mut module,
            &resolve,
            world,
            StringEncoding::UTF8,
        )?;
    }

    wasmparser::validate(&module)?;

    Ok(module)
}

#[test]
fn linking() -> Result<()> {
    let component = [
        ("libfoo.so", FOO, None),
        ("libbar.so", BAR, Some(WIT)),
        ("libc.so", LIBC, None),
    ]
    .into_iter()
    .try_fold(
        wit_component::Linker::default().validate(true),
        |linker, (name, wat, wit)| {
            linker.library(
                name,
                &encode(wat, wit).with_context(|| name.to_owned())?,
                false,
            )
        },
    )?
    .encode()?;

    #[cfg(target_family = "wasm")]
    {
        _ = component;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        use {
            anyhow::anyhow,
            wasmtime::{
                component::{Component, Linker},
                Config, Engine, Store,
            },
        };

        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);
        linker
            .instance("test:test/test")?
            .func_wrap("bar", |_store, (v,): (i32,)| Ok((v + 7,)))?;
        let mut store = Store::new(&engine, ());
        let instance = linker.instantiate(&mut store, &Component::new(&engine, &component)?)?;
        let func = instance
            .get_export(&mut store, None, "test:test/test")
            .and_then(|i| instance.get_export(&mut store, Some(&i), "bar"))
            .and_then(|f| {
                instance
                    .get_typed_func::<(i32,), (i32,)>(&mut store, &f)
                    .ok()
            })
            .ok_or_else(|| anyhow!("func `test:test/test/bar` not found"))?;

        assert_eq!(65, func.call(&mut store, (7,))?.0);
    }

    Ok(())
}
