use std::path::Path;

use anyhow::{Context, Result, bail};

macro_rules! genexpand {
    ($id:ident $name:tt $path:tt) => {
        process_expanded($path, "", wasmtime::component::bindgen!({
            path: $path,
            stringify: true,
        }))?;

        process_expanded($path, "_async", wasmtime::component::bindgen!({
            path: $path,
            stringify: true,
            imports: { default: async },
            exports: { default: async },
        }))?;

        process_expanded($path, "_concurrent", wasmtime::component::bindgen!({
            path: $path,
            imports: { default: async | store },
            exports: { default: async | store },
            stringify: true,
        }))?;

        process_expanded($path, "_tracing_async", wasmtime::component::bindgen!({
            path: $path,
            imports: { default: async | tracing },
            exports: { default: async | tracing },
            stringify: true,
        }))?;
    };
}

fn process_expanded(path: &str, suffix: &str, src: &str) -> Result<()> {
    let formatted_src = {
        let syn_file = syn::parse_file(src).unwrap();
        prettyplease::unparse(&syn_file)
    };
    let expanded_path = {
        let mut stem = Path::new(path).file_stem().unwrap().to_os_string();
        stem.push(suffix);
        Path::new("tests/expanded").join(stem).with_extension("rs")
    };
    if std::env::var("BINDGEN_TEST_BLESS").is_ok_and(|val| !val.is_empty()) {
        std::fs::write(expanded_path, formatted_src)?;
    } else {
        match std::fs::read_to_string(&expanded_path) {
            Ok(expected) if formatted_src == expected => (),
            Ok(expected) => {
                bail!(
                    "checked-in expanded bindings from {expanded_path:?} \
                    do not match those generated from {path:?}
                    \n\
                    {diff}\n\
                    \n\
                    This test assertion can be automatically updated by setting the\n\
                    BINDGEN_TEST_BLESS=1 environment variable when running this test.",
                    diff = similar::TextDiff::from_lines(&expected, &formatted_src)
                        .unified_diff()
                        .header("expected", "actual")
                )
            }
            Err(err) => return Err(err).with_context(|| {
                format!(
                    "failed to read {expanded_path:?}; re-run with BINDGEN_TEST_BLESS=1 to create"
                )
            }),
        }
    }
    Ok(())
}

#[test]
fn expand_wits() -> Result<()> {
    component_macro_test_helpers::foreach!(genexpand);
    Ok(())
}
