//! You can run this test suite with:
//!
//!     cargo test --test witxt
//!
//! An argument can be passed as well to filter, based on filename, which test
//! to run
//!
//!     cargo test --test witxt foo.witxt

use anyhow::{anyhow, bail, Context, Result};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use wast::parser::{self, Parse, ParseBuffer, Parser};
use witx::{Documentation, Instruction, Representable, WasmType};

fn main() {
    let tests = find_tests();
    let filter = std::env::args().nth(1);

    let tests = tests
        .par_iter()
        .filter_map(|test| {
            if let Some(filter) = &filter {
                if let Some(s) = test.to_str() {
                    if !s.contains(filter) {
                        return None;
                    }
                }
            }
            let contents = std::fs::read(test).unwrap();
            Some((test, contents))
        })
        .collect::<Vec<_>>();

    println!("running {} test files\n", tests.len());

    let ntests = AtomicUsize::new(0);
    let errors = tests
        .par_iter()
        .filter_map(|(test, contents)| {
            WitxtRunner {
                ntests: &ntests,
                documents: HashMap::new(),
            }
            .run(test, contents)
            .err()
        })
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for msg in errors.iter() {
            eprintln!("{:?}", msg);
        }

        panic!("{} tests failed", errors.len())
    }

    println!(
        "test result: ok. {} directives passed\n",
        ntests.load(SeqCst)
    );
}

/// Recursively finds all tests in a whitelisted set of directories which we
/// then load up and test in parallel.
fn find_tests() -> Vec<PathBuf> {
    let mut tests = Vec::new();
    find_tests("tests/witxt".as_ref(), &mut tests);
    tests.sort();
    return tests;

    fn find_tests(path: &Path, tests: &mut Vec<PathBuf>) {
        for f in path.read_dir().unwrap() {
            let f = f.unwrap();
            if f.file_type().unwrap().is_dir() {
                find_tests(&f.path(), tests);
                continue;
            }

            match f.path().extension().and_then(|s| s.to_str()) {
                Some("witxt") => {}
                _ => continue,
            }
            tests.push(f.path());
        }
    }
}

struct WitxtRunner<'a> {
    ntests: &'a AtomicUsize,
    documents: HashMap<String, witx::Document>,
}

impl WitxtRunner<'_> {
    fn run(&mut self, test: &Path, contents: &[u8]) -> Result<()> {
        let contents = str::from_utf8(contents)?;
        macro_rules! adjust {
            ($e:expr) => {{
                let mut e = wast::Error::from($e);
                e.set_path(test);
                e.set_text(contents);
                e
            }};
        }
        let buf = ParseBuffer::new(contents).map_err(|e| adjust!(e))?;
        let witxt = parser::parse::<Witxt>(&buf).map_err(|e| adjust!(e))?;

        let errors = witxt
            .directives
            .into_iter()
            .filter_map(|directive| {
                let (line, col) = directive.span().linecol_in(contents);
                self.test_directive(contents, test, directive)
                    .with_context(|| {
                        format!(
                            "failed directive on {}:{}:{}",
                            test.display(),
                            line + 1,
                            col + 1
                        )
                    })
                    .err()
            })
            .collect::<Vec<_>>();
        if errors.is_empty() {
            return Ok(());
        }
        let mut s = format!("{} test failures in {}:", errors.len(), test.display());
        for mut error in errors {
            if let Some(err) = error.downcast_mut::<wast::Error>() {
                err.set_path(test);
                err.set_text(contents);
            }
            s.push_str("\n\n\t--------------------------------\n\n\t");
            s.push_str(&format!("{:?}", error).replace("\n", "\n\t"));
        }
        bail!("{}", s)
    }

    fn test_directive(
        &mut self,
        contents: &str,
        test: &Path,
        directive: WitxtDirective,
    ) -> Result<()> {
        self.bump_ntests();
        match directive {
            WitxtDirective::Witx(witx) => {
                let doc = witx.document(contents, test)?;
                self.assert_roundtrip(&doc)
                    .context("failed to round-trip the document")?;
                self.assert_md(&doc)?;
                if let Some(name) = witx.id {
                    self.documents.insert(name.name().to_string(), doc);
                }
            }
            WitxtDirective::AssertInvalid { witx, message, .. } => {
                let err = match witx.document(contents, test) {
                    Ok(_) => bail!("witx was valid when it shouldn't be"),
                    Err(e) => format!("{:?}", anyhow::Error::from(e)),
                };
                if !err.contains(message) {
                    bail!("expected error {:?}\nfound error {}", message, err);
                }
            }
            WitxtDirective::AssertRepresentable { repr, t1, t2, .. } => {
                let (t1m, t1t) = t1;
                let (t2m, t2t) = t2;
                let t1d = self
                    .documents
                    .get(t1m.name())
                    .ok_or_else(|| anyhow!("no document named {:?}", t1m.name()))?;
                let t2d = self
                    .documents
                    .get(t2m.name())
                    .ok_or_else(|| anyhow!("no document named {:?}", t2m.name()))?;
                let t1 = t1d
                    .typename(&witx::Id::new(t1t))
                    .ok_or_else(|| anyhow!("no type named {:?}", t1t))?;
                let t2 = t2d
                    .typename(&witx::Id::new(t2t))
                    .ok_or_else(|| anyhow!("no type named {:?}", t2t))?;
                match (repr, t1.type_().representable(&t2.type_())) {
                    (RepEquality::Eq, witx::RepEquality::Eq)
                    | (RepEquality::Superset, witx::RepEquality::Superset)
                    | (RepEquality::NotEq, witx::RepEquality::NotEq) => {}
                    (a, b) => {
                        bail!("expected {:?} representation, got {:?}", a, b);
                    }
                }
            }
            WitxtDirective::AssertAbi {
                witx,
                wasm,
                interface,
                wasm_signature: (wasm_params, wasm_results),
                ..
            } => {
                let doc = witx.document(contents, test)?;
                let module = doc.modules().next().ok_or_else(|| anyhow!("no modules"))?;
                let func = module.funcs().next().ok_or_else(|| anyhow!("no funcs"))?;

                let (params, results) = func.wasm_signature();
                if params != wasm_params {
                    bail!("expected params {:?}, found {:?}", wasm_params, params);
                }
                if results != wasm_results {
                    bail!("expected results {:?}, found {:?}", wasm_results, results);
                }

                let mut check = AbiBindgen {
                    abi: wasm.instrs.iter(),
                    err: None,
                    contents,
                };
                func.call_wasm(&module.name, &mut check);
                check.check()?;
                check.abi = interface.instrs.iter();
                func.call_interface(&module.name, &mut check);
                check.check()?;
            }
        }
        Ok(())
    }

    fn assert_roundtrip(&self, doc: &witx::Document) -> Result<()> {
        self.bump_ntests();
        let back_to_sexprs = format!("{}", doc);
        let doc2 = witx::parse(&back_to_sexprs)?;
        if *doc == doc2 {
            return Ok(());
        }

        // Try to get a more specific error message that isn't thousands of
        // lines long of debug representations.
        for type_ in doc.typenames() {
            let type2 = match doc2.typename(&type_.name) {
                Some(t) => t,
                None => bail!("doc2 missing datatype"),
            };
            if type_ != type2 {
                bail!("types are not equal\n{:?}\n   !=\n{:?}", type_, type2);
            }
        }
        for mod_ in doc.modules() {
            let mod2 = match doc2.module(&mod_.name) {
                Some(m) => m,
                None => bail!("doc2 missing module"),
            };
            for import in mod_.imports() {
                let import2 = match mod2.import(&import.name) {
                    Some(i) => i,
                    None => bail!("mod2 missing import"),
                };
                assert_eq!(import, import2);
            }
            for func in mod_.funcs() {
                let func2 = match mod2.func(&func.name) {
                    Some(f) => f,
                    None => bail!("mod2 missing func"),
                };
                assert_eq!(func, func2);
            }
        }
        bail!("{:?} != {:?}", doc, doc2)
    }

    fn assert_md(&self, doc: &witx::Document) -> Result<()> {
        self.bump_ntests();
        doc.to_md();
        Ok(())
    }

    fn bump_ntests(&self) {
        self.ntests.fetch_add(1, SeqCst);
    }
}

struct AbiBindgen<'a> {
    abi: std::slice::Iter<'a, (wast::Span, &'a str)>,
    err: Option<anyhow::Error>,
    contents: &'a str,
}

impl AbiBindgen<'_> {
    fn check(&mut self) -> Result<()> {
        match self.err.take() {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }

    fn assert(&mut self, name: &str) {
        if self.err.is_some() {
            return;
        }
        match self.abi.next() {
            Some((_, s)) if *s == name => {}
            Some((span, s)) => {
                let (line, col) = span.linecol_in(self.contents);
                self.err = Some(anyhow!(
                    "line {}:{} - expected `{}` found `{}`",
                    line + 1,
                    col + 1,
                    name,
                    s,
                ));
            }
            None => {
                self.err = Some(anyhow!(
                    "extra instruction `{}` found when none was expected",
                    name
                ));
            }
        }
    }
}

impl witx::Bindgen for AbiBindgen<'_> {
    type Operand = ();
    fn emit(
        &mut self,
        inst: &Instruction<'_>,
        _operands: &mut Vec<Self::Operand>,
        results: &mut Vec<Self::Operand>,
    ) {
        use witx::Instruction::*;
        match inst {
            GetArg { nth } => self.assert(&format!("get-arg{}", nth)),
            AddrOf => self.assert("addr-of"),
            I32FromChar => self.assert("i32.from_char"),
            I64FromU64 => self.assert("i64.from_u64"),
            I64FromS64 => self.assert("i64.from_s64"),
            I32FromU32 => self.assert("i32.from_u32"),
            I32FromS32 => self.assert("i32.from_s32"),
            I32FromUsize => self.assert("i32.from_usize"),
            I32FromU16 => self.assert("i32.from_u16"),
            I32FromS16 => self.assert("i32.from_s16"),
            I32FromU8 => self.assert("i32.from_u8"),
            I32FromS8 => self.assert("i32.from_s8"),
            I32FromChar8 => self.assert("i32.from_char8"),
            I32FromPointer => self.assert("i32.from_pointer"),
            I32FromConstPointer => self.assert("i32.from_const_pointer"),
            I32FromHandle { .. } => self.assert("i32.from_handle"),
            ListPointerLength => self.assert("list.pointer_length"),
            ListFromPointerLength { .. } => self.assert("list.from_pointer_length"),
            F32FromIf32 => self.assert("f32.from_if32"),
            F64FromIf64 => self.assert("f64.from_if64"),
            CallWasm { .. } => self.assert("call.wasm"),
            CallInterface { .. } => self.assert("call.interface"),
            S8FromI32 => self.assert("s8.from_i32"),
            U8FromI32 => self.assert("u8.from_i32"),
            S16FromI32 => self.assert("s16.from_i32"),
            U16FromI32 => self.assert("u16.from_i32"),
            S32FromI32 => self.assert("s32.from_i32"),
            U32FromI32 => self.assert("u32.from_i32"),
            S64FromI64 => self.assert("s64.from_i64"),
            U64FromI64 => self.assert("u64.from_i64"),
            CharFromI32 => self.assert("char.from_i32"),
            Char8FromI32 => self.assert("char8.from_i32"),
            UsizeFromI32 => self.assert("usize.from_i32"),
            If32FromF32 => self.assert("if32.from_f32"),
            If64FromF64 => self.assert("if64.from_f64"),
            HandleFromI32 { .. } => self.assert("handle.from_i32"),
            PointerFromI32 { .. } => self.assert("pointer.from_i32"),
            ConstPointerFromI32 { .. } => self.assert("const_pointer.from_i32"),
            ReturnPointerGet { n } => self.assert(&format!("return_pointer.get{}", n)),
            ResultLift => self.assert("result.lift"),
            ResultLower { .. } => self.assert("result.lower"),
            EnumLift { .. } => self.assert("enum.lift"),
            EnumLower { .. } => self.assert("enum.lower"),
            TupleLift { .. } => self.assert("tuple.lift"),
            TupleLower { .. } => self.assert("tuple.lower"),
            ReuseReturn => self.assert("reuse_return"),
            Load { .. } => self.assert("load"),
            Store { .. } => self.assert("store"),
            Return { .. } => self.assert("return"),
            VariantPayload => self.assert("variant-payload"),
            I32FromBitflags { .. } => self.assert("i32.from_bitflags"),
            BitflagsFromI32 { .. } => self.assert("bitflags.from_i32"),
            I64FromBitflags { .. } => self.assert("i64.from_bitflags"),
            BitflagsFromI64 { .. } => self.assert("bitflags.from_i64"),
        }
        for _ in 0..inst.results_len() {
            results.push(());
        }
    }

    fn allocate_space(&mut self, _: usize, _: &witx::NamedType) {
        self.assert("allocate-space");
    }

    fn push_block(&mut self) {
        self.assert("block.push");
    }

    fn finish_block(&mut self, _operand: Option<Self::Operand>) {
        self.assert("block.finish");
    }
}

mod kw {
    wast::custom_keyword!(assert_invalid);
    wast::custom_keyword!(assert_representable);
    wast::custom_keyword!(assert_abi);
    wast::custom_keyword!(witx);
    wast::custom_keyword!(eq);
    wast::custom_keyword!(noteq);
    wast::custom_keyword!(load);
    wast::custom_keyword!(superset);
    wast::custom_keyword!(call_wasm);
    wast::custom_keyword!(call_interface);
    wast::custom_keyword!(param);
    wast::custom_keyword!(result);
    wast::custom_keyword!(wasm);
    wast::custom_keyword!(i32);
    wast::custom_keyword!(i64);
    wast::custom_keyword!(f32);
    wast::custom_keyword!(f64);
}

struct Witxt<'a> {
    directives: Vec<WitxtDirective<'a>>,
}

impl<'a> Parse<'a> for Witxt<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let mut directives = Vec::new();
        while !parser.is_empty() {
            directives.push(parser.parens(|p| p.parse())?);
        }
        Ok(Witxt { directives })
    }
}

enum WitxtDirective<'a> {
    Witx(Witx<'a>),
    AssertInvalid {
        span: wast::Span,
        witx: Witx<'a>,
        message: &'a str,
    },
    AssertRepresentable {
        span: wast::Span,
        repr: RepEquality,
        t1: (wast::Id<'a>, &'a str),
        t2: (wast::Id<'a>, &'a str),
    },
    AssertAbi {
        span: wast::Span,
        witx: Witx<'a>,
        wasm_signature: (Vec<WasmType>, Vec<WasmType>),
        wasm: Abi<'a>,
        interface: Abi<'a>,
    },
}

impl WitxtDirective<'_> {
    fn span(&self) -> wast::Span {
        match self {
            WitxtDirective::Witx(w) => w.span,
            WitxtDirective::AssertInvalid { span, .. }
            | WitxtDirective::AssertAbi { span, .. }
            | WitxtDirective::AssertRepresentable { span, .. } => *span,
        }
    }
}

impl<'a> Parse<'a> for WitxtDirective<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::witx>() {
            Ok(WitxtDirective::Witx(parser.parse()?))
        } else if l.peek::<kw::assert_invalid>() {
            let span = parser.parse::<kw::assert_invalid>()?.0;
            Ok(WitxtDirective::AssertInvalid {
                span,
                witx: parser.parens(|p| p.parse())?,
                message: parser.parse()?,
            })
        } else if l.peek::<kw::assert_representable>() {
            let span = parser.parse::<kw::assert_representable>()?.0;
            Ok(WitxtDirective::AssertRepresentable {
                span,
                repr: parser.parse()?,
                t1: (parser.parse()?, parser.parse()?),
                t2: (parser.parse()?, parser.parse()?),
            })
        } else if l.peek::<kw::assert_abi>() {
            let span = parser.parse::<kw::assert_abi>()?.0;
            Ok(WitxtDirective::AssertAbi {
                span,
                witx: parser.parens(|p| p.parse())?,
                wasm_signature: parser.parens(|p| {
                    p.parse::<kw::wasm>()?;
                    let mut params = Vec::new();
                    let mut results = Vec::new();
                    if p.peek2::<kw::param>() {
                        p.parens(|p| {
                            p.parse::<kw::param>()?;
                            while !p.is_empty() {
                                params.push(parse_wasmtype(p)?);
                            }
                            Ok(())
                        })?;
                    }
                    if p.peek2::<kw::result>() {
                        p.parens(|p| {
                            p.parse::<kw::result>()?;
                            while !p.is_empty() {
                                results.push(parse_wasmtype(p)?);
                            }
                            Ok(())
                        })?;
                    }
                    Ok((params, results))
                })?,
                wasm: parser.parens(|p| {
                    p.parse::<kw::call_wasm>()?;
                    p.parse()
                })?,
                interface: parser.parens(|p| {
                    p.parse::<kw::call_interface>()?;
                    p.parse()
                })?,
            })
        } else {
            Err(l.error())
        }
    }
}

fn parse_wasmtype(p: Parser<'_>) -> parser::Result<WasmType> {
    let mut l = p.lookahead1();
    if l.peek::<kw::i32>() {
        p.parse::<kw::i32>()?;
        Ok(WasmType::I32)
    } else if l.peek::<kw::i64>() {
        p.parse::<kw::i64>()?;
        Ok(WasmType::I64)
    } else if l.peek::<kw::f32>() {
        p.parse::<kw::f32>()?;
        Ok(WasmType::F32)
    } else if l.peek::<kw::f64>() {
        p.parse::<kw::f64>()?;
        Ok(WasmType::F64)
    } else {
        Err(l.error())
    }
}

struct Witx<'a> {
    span: wast::Span,
    id: Option<wast::Id<'a>>,
    def: WitxDef<'a>,
}

enum WitxDef<'a> {
    Fs(Vec<&'a str>),
    Inline(Vec<witx::parser::Documented<'a, witx::parser::DeclSyntax<'a>>>),
}

impl Witx<'_> {
    fn document(&self, contents: &str, file: &Path) -> Result<witx::Document> {
        match &self.def {
            WitxDef::Inline(decls) => {
                let mut validator = witx::DocValidation::new();
                let mut definitions = Vec::new();
                for decl in decls {
                    validator
                        .scope(contents, file)
                        .validate_decl(&decl.item, &decl.comments, &mut definitions)
                        .map_err(witx::WitxError::Validation)?;
                }
                Ok(validator.into_document(definitions))
            }
            WitxDef::Fs(paths) => {
                let parent = file.parent().unwrap();
                let paths = paths.iter().map(|p| parent.join(p)).collect::<Vec<_>>();
                Ok(witx::load(&paths)?)
            }
        }
    }
}

impl<'a> Parse<'a> for Witx<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let span = parser.parse::<kw::witx>()?.0;
        let id = parser.parse()?;

        let def = if parser.peek2::<kw::load>() {
            parser.parens(|p| {
                p.parse::<kw::load>()?;
                let mut paths = Vec::new();
                while !p.is_empty() {
                    paths.push(p.parse()?);
                }
                Ok(WitxDef::Fs(paths))
            })?
        } else {
            let mut decls = Vec::new();
            while !parser.is_empty() {
                decls.push(parser.parens(|p| p.parse())?);
            }
            WitxDef::Inline(decls)
        };
        Ok(Witx { id, span, def })
    }
}

#[derive(Debug)]
enum RepEquality {
    Eq,
    NotEq,
    Superset,
}

impl<'a> Parse<'a> for RepEquality {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::eq>() {
            parser.parse::<kw::eq>()?;
            Ok(RepEquality::Eq)
        } else if l.peek::<kw::noteq>() {
            parser.parse::<kw::noteq>()?;
            Ok(RepEquality::NotEq)
        } else if l.peek::<kw::superset>() {
            parser.parse::<kw::superset>()?;
            Ok(RepEquality::Superset)
        } else {
            Err(l.error())
        }
    }
}

struct Abi<'a> {
    instrs: Vec<(wast::Span, &'a str)>,
}

impl<'a> Parse<'a> for Abi<'a> {
    fn parse(parser: Parser<'a>) -> parser::Result<Self> {
        let mut instrs = Vec::new();
        while !parser.is_empty() {
            instrs.push(parser.step(|cursor| {
                let (kw, next) = cursor
                    .keyword()
                    .ok_or_else(|| cursor.error("expected keyword"))?;
                Ok(((cursor.cur_span(), kw), next))
            })?);
        }
        Ok(Abi { instrs })
    }
}
