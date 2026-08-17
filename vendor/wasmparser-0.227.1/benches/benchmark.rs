use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use once_cell::unsync::Lazy;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use wasmparser::VisitSimdOperator;
use wasmparser::{DataKind, ElementKind, Parser, Payload, Validator, VisitOperator, WasmFeatures};

/// A benchmark input.
pub struct BenchmarkInput {
    /// The path to the benchmark file important for handling errors.
    pub path: PathBuf,
    /// The encoded Wasm module that is run by the benchmark.
    pub wasm: Vec<u8>,
}

impl BenchmarkInput {
    /// Creates a new benchmark input.
    pub fn new(test_path: PathBuf, encoded_wasm: Vec<u8>) -> Self {
        Self {
            path: test_path,
            wasm: encoded_wasm,
        }
    }
}

/// Returns a vector of all found benchmark input files under the given directory.
///
/// Benchmark input files can be `.wat` or `.wast` formatted files.
/// For `.wast` files we pull out all the module directives and run them in the benchmarks.
fn collect_test_files(path: &Path, list: &mut Vec<BenchmarkInput>) -> Result<()> {
    for entry in path.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_test_files(&path, list)?;
            continue;
        }
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("wasm") => {
                let wasm = fs::read(&path)?;
                list.push(BenchmarkInput::new(path, wasm));
            }
            Some("wat") | Some("txt") => {
                if let Ok(wasm) = wat::parse_file(&path) {
                    list.push(BenchmarkInput::new(path, wasm));
                }
            }
            Some("wast") => {
                let contents = fs::read_to_string(&path)?;
                let buf = match wast::parser::ParseBuffer::new(&contents) {
                    Ok(buf) => buf,
                    Err(_) => continue,
                };
                let wast: wast::Wast<'_> = match wast::parser::parse(&buf) {
                    Ok(wast) => wast,
                    Err(_) => continue,
                };
                for directive in wast.directives {
                    match directive {
                        wast::WastDirective::Module(mut module)
                        | wast::WastDirective::ModuleDefinition(mut module) => {
                            let wasm = module.encode()?;
                            list.push(BenchmarkInput::new(path.clone(), wasm));
                        }
                        _ => continue,
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}

/// Reads the input given the Wasm parser or validator.
///
/// The `path` specifies which benchmark input file we are currently operating on
/// so that we can report better errors in case of failures.
fn read_all_wasm(wasm: &[u8]) -> Result<()> {
    use Payload::*;
    for item in Parser::new(0).parse_all(wasm) {
        match item? {
            TypeSection(s) => {
                for item in s {
                    item?;
                }
            }
            ImportSection(s) => {
                for item in s {
                    item?;
                }
            }
            FunctionSection(s) => {
                for item in s {
                    item?;
                }
            }
            TableSection(s) => {
                for item in s {
                    item?;
                }
            }
            MemorySection(s) => {
                for item in s {
                    item?;
                }
            }
            TagSection(s) => {
                for item in s {
                    item?;
                }
            }
            GlobalSection(s) => {
                for item in s {
                    for op in item?.init_expr.get_operators_reader() {
                        op?;
                    }
                }
            }
            ExportSection(s) => {
                for item in s {
                    item?;
                }
            }
            ElementSection(s) => {
                for item in s {
                    let item = item?;
                    if let ElementKind::Active { offset_expr, .. } = item.kind {
                        for op in offset_expr.get_operators_reader() {
                            op?;
                        }
                    }
                    match item.items {
                        wasmparser::ElementItems::Functions(r) => {
                            for op in r {
                                op?;
                            }
                        }
                        wasmparser::ElementItems::Expressions(_, r) => {
                            for op in r {
                                op?;
                            }
                        }
                    }
                }
            }
            DataSection(s) => {
                for item in s {
                    let item = item?;
                    if let DataKind::Active { offset_expr, .. } = item.kind {
                        for op in offset_expr.get_operators_reader() {
                            op?;
                        }
                    }
                }
            }
            CodeSectionEntry(body) => {
                let mut reader = body.get_binary_reader();
                for _ in 0..reader.read_var_u32()? {
                    reader.read_var_u32()?;
                    reader.read::<wasmparser::ValType>()?;
                }
                while !reader.eof() {
                    reader.visit_operator(&mut NopVisit)?;
                }
            }

            // Component sections
            ModuleSection { .. } => {}
            InstanceSection(s) => {
                for item in s {
                    item?;
                }
            }
            CoreTypeSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentSection { .. } => {}
            ComponentInstanceSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentAliasSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentTypeSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentCanonicalSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentStartSection { .. } => {}
            ComponentImportSection(s) => {
                for item in s {
                    item?;
                }
            }
            ComponentExportSection(s) => {
                for item in s {
                    item?;
                }
            }

            Version { .. }
            | StartSection { .. }
            | DataCountSection { .. }
            | UnknownSection { .. }
            | CustomSection { .. }
            | CodeSectionStart { .. }
            | End(_) => {}

            other => {
                // NB: if you hit this panic if you'd be so kind as to grep
                // through other locations in the code base that need to be
                // updated as well. As of the time of this writing the locations
                // might be:
                //
                //  * src/bin/wasm-tools/objdump.rs
                //  * src/bin/wasm-tools/dump.rs
                //  * crates/wasm-encoder/src/reencode.rs
                //  * crates/wasm-encoder/src/reencode/component.rs
                //  * crates/wasmprinter/src/lib.rs
                //  * crates/wit-component/src/gc.rs
                //
                // This is required due to the `#[non_exhaustive]` nature of
                // the `Payload` enum.
                panic!("a new match statement should be added above for this case: {other:?}")
            }
        }
    }
    Ok(())
}

/// Returns the default benchmark inputs that are proper `wasmparser` benchmark
/// test inputs.
fn collect_benchmark_inputs() -> Vec<BenchmarkInput> {
    let mut ret = Vec::new();
    collect_test_files("../../tests".as_ref(), &mut ret).unwrap();
    // Sort to ideally get more deterministic perf that ignores filesystems
    ret.sort_by_key(|p| p.path.clone());
    ret
}

fn skip_validation(test: &Path) -> bool {
    let broken = [
        "gc/gc-rec-sub.wat",
        "proposals/gc/type-equivalence.wast",
        "proposals/gc/type-subtyping.wast",
    ];

    let test_path = test.to_str().unwrap().replace("\\", "/"); // for windows paths
    if broken.iter().any(|x| test_path.contains(x)) {
        return true;
    }

    false
}

fn define_benchmarks(c: &mut Criterion) {
    let _ = env_logger::try_init();

    fn validator() -> Validator {
        Validator::new_with_features(WasmFeatures::all())
    }
    fn old_validator() -> Validator {
        Validator::new_with_features(WasmFeatures::WASM2)
    }

    let test_inputs = once_cell::unsync::Lazy::new(collect_benchmark_inputs);

    let parse_inputs = once_cell::unsync::Lazy::new(|| {
        let mut list = Vec::new();
        for input in test_inputs.iter() {
            if read_all_wasm(&input.wasm).is_ok() {
                list.push(&input.wasm);
            }
        }
        list
    });
    c.bench_function("parse/tests", |b| {
        Lazy::force(&parse_inputs);
        b.iter(|| {
            for wasm in parse_inputs.iter() {
                read_all_wasm(wasm).unwrap();
            }
        })
    });

    let validate_inputs = once_cell::unsync::Lazy::new(|| {
        let mut list = Vec::new();
        for input in test_inputs.iter() {
            if skip_validation(&input.path) {
                continue;
            }
            log::debug!("Validating {}", input.path.display());
            if validator().validate_all(&input.wasm).is_ok() {
                list.push(&input.wasm);
            }
        }
        list
    });
    c.bench_function("validate/tests", |b| {
        Lazy::force(&validate_inputs);
        b.iter(|| {
            for wasm in validate_inputs.iter() {
                validator().validate_all(wasm).unwrap();
            }
        })
    });

    for file in std::fs::read_dir("benches").unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.extension().and_then(|s| s.to_str()) != Some("wasm") {
            continue;
        }
        let name = path.file_stem().unwrap().to_str().unwrap();
        let wasm = Lazy::new(|| std::fs::read(&path).unwrap());
        c.bench_function(&format!("validate/{name}"), |b| {
            Lazy::force(&wasm);
            b.iter(|| {
                validator().validate_all(&wasm).unwrap();
            })
        });
        if old_validator().validate_all(&wasm).is_ok() {
            c.bench_function(&format!("validate-old/{name}"), |b| {
                Lazy::force(&wasm);
                b.iter(|| {
                    old_validator().validate_all(&wasm).unwrap();
                })
            });
        }
        c.bench_function(&format!("parse/{name}"), |b| {
            Lazy::force(&wasm);
            b.iter(|| {
                read_all_wasm(&wasm).unwrap();
            })
        });
    }
}

criterion_group!(benchmark, define_benchmarks);
criterion_main!(benchmark);

struct NopVisit;

macro_rules! define_visit_operator {
    ($(@$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
        $(
            fn $visit(&mut self $($(,$arg: $argty)*)?) {
                define_visit_operator!(@visit $op $( $($arg)* )?);
            }
        )*
    };

    (@visit BrTable $table:ident) => {
        for target in $table.targets() {
            target.unwrap();
        }
    };
    (@visit $($rest:tt)*) => {}
}

#[allow(unused_variables)]
impl<'a> VisitOperator<'a> for NopVisit {
    type Output = ();

    fn simd_visitor(&mut self) -> Option<&mut dyn VisitSimdOperator<'a, Output = Self::Output>> {
        Some(self)
    }

    wasmparser::for_each_visit_operator!(define_visit_operator);
}

#[allow(unused_variables)]
impl<'a> VisitSimdOperator<'a> for NopVisit {
    wasmparser::for_each_visit_simd_operator!(define_visit_operator);
}
