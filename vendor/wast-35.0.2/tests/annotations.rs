use wasmparser::*;

#[test]
fn name_annotations() -> anyhow::Result<()> {
    assert_module_name("foo", r#"(module $foo)"#)?;
    assert_module_name("foo", r#"(module (@name "foo"))"#)?;
    assert_module_name("foo", r#"(module $bar (@name "foo"))"#)?;
    assert_module_name("foo bar", r#"(module $bar (@name "foo bar"))"#)?;
    Ok(())
}

fn assert_module_name(name: &str, wat: &str) -> anyhow::Result<()> {
    let wasm = wat::parse_str(wat)?;
    let mut found = false;
    for s in get_name_section(&wasm)? {
        match s? {
            Name::Module(n) => {
                assert_eq!(n.get_name()?, name);
                found = true;
            }
            _ => {}
        }
    }
    assert!(found);
    Ok(())
}

#[test]
fn func_annotations() -> anyhow::Result<()> {
    assert_func_name("foo", r#"(module (func $foo))"#)?;
    assert_func_name("foo", r#"(module (func (@name "foo")))"#)?;
    assert_func_name("foo", r#"(module (func $bar (@name "foo")))"#)?;
    assert_func_name("foo bar", r#"(module (func $bar (@name "foo bar")))"#)?;
    Ok(())
}

fn assert_func_name(name: &str, wat: &str) -> anyhow::Result<()> {
    let wasm = wat::parse_str(wat)?;
    let mut found = false;
    for s in get_name_section(&wasm)? {
        match s? {
            Name::Function(n) => {
                let mut map = n.get_map()?;
                let naming = map.read()?;
                assert_eq!(naming.index, 0);
                assert_eq!(naming.name, name);
                found = true;
            }
            _ => {}
        }
    }
    assert!(found);
    Ok(())
}

#[test]
fn local_annotations() -> anyhow::Result<()> {
    assert_local_name("foo", r#"(module (func (param $foo i32)))"#)?;
    assert_local_name("foo", r#"(module (func (local $foo i32)))"#)?;
    assert_local_name("foo", r#"(module (func (param (@name "foo") i32)))"#)?;
    assert_local_name("foo", r#"(module (func (local (@name "foo") i32)))"#)?;
    assert_local_name("foo", r#"(module (func (param $bar (@name "foo") i32)))"#)?;
    assert_local_name("foo", r#"(module (func (local $bar (@name "foo") i32)))"#)?;
    assert_local_name(
        "foo bar",
        r#"(module (func (param $bar (@name "foo bar") i32)))"#,
    )?;
    assert_local_name(
        "foo bar",
        r#"(module (func (local $bar (@name "foo bar") i32)))"#,
    )?;
    Ok(())
}

fn assert_local_name(name: &str, wat: &str) -> anyhow::Result<()> {
    let wasm = wat::parse_str(wat)?;
    let mut found = false;
    for s in get_name_section(&wasm)? {
        match s? {
            Name::Local(n) => {
                let mut reader = n.get_function_local_reader()?;
                let section = reader.read()?;
                let mut map = section.get_map()?;
                let naming = map.read()?;
                assert_eq!(naming.index, 0);
                assert_eq!(naming.name, name);
                found = true;
            }
            _ => {}
        }
    }
    assert!(found);
    Ok(())
}

fn get_name_section(wasm: &[u8]) -> anyhow::Result<NameSectionReader<'_>> {
    for payload in Parser::new(0).parse_all(&wasm) {
        if let Payload::CustomSection {
            name: "name",
            data,
            data_offset,
            range: _,
        } = payload?
        {
            return Ok(NameSectionReader::new(data, data_offset)?);
        }
    }
    panic!("no name section found");
}

#[test]
fn custom_section_order() -> anyhow::Result<()> {
    let wasm = wat::parse_str(
        r#"
            (module
              (@custom "A" "aaa")
              (type $t (func))
              (@custom "B" (after func) "bbb")
              (@custom "C" (before func) "ccc")
              (@custom "D" (after last) "ddd")
              (table 10 funcref)
              (func (type $t))
              (@custom "E" (after import) "eee")
              (@custom "F" (before type) "fff")
              (@custom "G" (after data) "ggg")
              (@custom "H" (after code) "hhh")
              (@custom "I" (after func) "iii")
              (@custom "J" (before func) "jjj")
              (@custom "K" (before first) "kkk")
            )
        "#,
    )?;
    macro_rules! assert_matches {
        ($a:expr, $b:pat $(,)?) => {
            match &$a {
                $b => {}
                a => panic!("`{:?}` doesn't match `{}`", a, stringify!($b)),
            }
        };
    }
    let wasm = Parser::new(0)
        .parse_all(&wasm)
        .collect::<Result<Vec<_>>>()?;
    assert_matches!(wasm[0], Payload::Version { .. });
    assert_matches!(wasm[1], Payload::CustomSection { name: "K", .. });
    assert_matches!(wasm[2], Payload::CustomSection { name: "F", .. });
    assert_matches!(wasm[3], Payload::TypeSection(_));
    assert_matches!(wasm[4], Payload::CustomSection { name: "E", .. });
    assert_matches!(wasm[5], Payload::CustomSection { name: "C", .. });
    assert_matches!(wasm[6], Payload::CustomSection { name: "J", .. });
    assert_matches!(wasm[7], Payload::FunctionSection(_));
    assert_matches!(wasm[8], Payload::CustomSection { name: "B", .. });
    assert_matches!(wasm[9], Payload::CustomSection { name: "I", .. });
    assert_matches!(wasm[10], Payload::TableSection(_));
    assert_matches!(wasm[11], Payload::CodeSectionStart { .. });
    assert_matches!(wasm[12], Payload::CodeSectionEntry { .. });
    assert_matches!(wasm[13], Payload::CustomSection { name: "H", .. });
    assert_matches!(wasm[14], Payload::CustomSection { name: "G", .. });
    assert_matches!(wasm[15], Payload::CustomSection { name: "A", .. });
    assert_matches!(wasm[16], Payload::CustomSection { name: "D", .. });
    assert_matches!(wasm[17], Payload::End);
    Ok(())
}
