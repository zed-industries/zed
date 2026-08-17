// (C) Copyright 2016 Jethro G. Beekman
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
extern crate cexpr;
extern crate clang_sys;

use std::collections::HashMap;
use std::io::Write;
use std::str::{self, FromStr};
use std::{char, ffi, mem, ptr, slice};

use cexpr::assert_full_parse;
use cexpr::expr::{fn_macro_declaration, EvalResult, IdentifierParser};
use cexpr::literal::CChar;
use cexpr::token::Token;
use clang_sys::*;

// main testing routine
fn test_definition(
    ident: Vec<u8>,
    tokens: &[Token],
    idents: &mut HashMap<Vec<u8>, EvalResult>,
) -> bool {
    fn bytes_to_int(value: &[u8]) -> Option<EvalResult> {
        str::from_utf8(value)
            .ok()
            .map(|s| s.replace("n", "-"))
            .map(|s| s.replace("_", ""))
            .and_then(|v| i64::from_str(&v).ok())
            .map(::std::num::Wrapping)
            .map(Int)
    }

    use cexpr::expr::EvalResult::*;

    let display_name = String::from_utf8_lossy(&ident).into_owned();

    let functional;
    let test = {
        // Split name such as Str_test_string into (Str,test_string)
        let pos = ident
            .iter()
            .position(|c| *c == b'_')
            .expect(&format!("Invalid definition in testcase: {}", display_name));
        let mut expected = &ident[..pos];
        let mut value = &ident[(pos + 1)..];

        functional = expected == b"Fn";

        if functional {
            let ident = value;
            let pos = ident
                .iter()
                .position(|c| *c == b'_')
                .expect(&format!("Invalid definition in testcase: {}", display_name));
            expected = &ident[..pos];
            value = &ident[(pos + 1)..];
        }

        if expected == b"Str" {
            let mut splits = value.split(|c| *c == b'U');
            let mut s = Vec::with_capacity(value.len());
            s.extend_from_slice(splits.next().unwrap());
            for split in splits {
                let (chr, rest) = split.split_at(6);
                let chr = u32::from_str_radix(str::from_utf8(chr).unwrap(), 16).unwrap();
                write!(s, "{}", char::from_u32(chr).unwrap()).unwrap();
                s.extend_from_slice(rest);
            }
            Some(Str(s))
        } else if expected == b"Int" {
            bytes_to_int(value)
        } else if expected == b"Float" {
            str::from_utf8(value)
                .ok()
                .map(|s| s.replace("n", "-").replace("p", "."))
                .and_then(|v| f64::from_str(&v).ok())
                .map(Float)
        } else if expected == b"CharRaw" {
            str::from_utf8(value)
                .ok()
                .and_then(|v| u64::from_str(v).ok())
                .map(CChar::Raw)
                .map(Char)
        } else if expected == b"CharChar" {
            str::from_utf8(value)
                .ok()
                .and_then(|v| u32::from_str(v).ok())
                .and_then(char::from_u32)
                .map(CChar::Char)
                .map(Char)
        } else {
            Some(Invalid)
        }
        .expect(&format!("Invalid definition in testcase: {}", display_name))
    };

    let result = if functional {
        let mut fnidents;
        let expr_tokens;
        match fn_macro_declaration(&tokens) {
            Ok((rest, (_, args))) => {
                fnidents = idents.clone();
                expr_tokens = rest;
                for arg in args {
                    let val = match test {
                        Int(_) => bytes_to_int(&arg),
                        Str(_) => Some(Str(arg.to_owned())),
                        _ => unimplemented!(),
                    }
                    .expect(&format!(
                        "Invalid argument in functional macro testcase: {}",
                        display_name
                    ));
                    fnidents.insert(arg.to_owned(), val);
                }
            }
            e => {
                println!(
                    "Failed test for {}, unable to parse functional macro declaration: {:?}",
                    display_name, e
                );
                return false;
            }
        }
        assert_full_parse(IdentifierParser::new(&fnidents).expr(&expr_tokens))
    } else {
        IdentifierParser::new(idents)
            .macro_definition(&tokens)
            .map(|(i, (_, val))| (i, val))
    };

    match result {
        Ok((_, val)) => {
            if val == test {
                if let Some(_) = idents.insert(ident, val) {
                    panic!("Duplicate definition for testcase: {}", display_name);
                }
                true
            } else {
                println!(
                    "Failed test for {}, expected {:?}, got {:?}",
                    display_name, test, val
                );
                false
            }
        }
        e => {
            if test == Invalid {
                true
            } else {
                println!(
                    "Failed test for {}, expected {:?}, got {:?}",
                    display_name, test, e
                );
                false
            }
        }
    }
}

// support code for the clang lexer
unsafe fn clang_str_to_vec(s: CXString) -> Vec<u8> {
    let vec = ffi::CStr::from_ptr(clang_getCString(s))
        .to_bytes()
        .to_owned();
    clang_disposeString(s);
    vec
}

#[allow(non_upper_case_globals)]
unsafe fn token_clang_to_cexpr(tu: CXTranslationUnit, orig: &CXToken) -> Token {
    Token {
        kind: match clang_getTokenKind(*orig) {
            CXToken_Comment => cexpr::token::Kind::Comment,
            CXToken_Identifier => cexpr::token::Kind::Identifier,
            CXToken_Keyword => cexpr::token::Kind::Keyword,
            CXToken_Literal => cexpr::token::Kind::Literal,
            CXToken_Punctuation => cexpr::token::Kind::Punctuation,
            _ => panic!("invalid token kind: {:?}", *orig),
        },
        raw: clang_str_to_vec(clang_getTokenSpelling(tu, *orig)).into_boxed_slice(),
    }
}

extern "C" fn visit_children_thunk<F>(
    cur: CXCursor,
    parent: CXCursor,
    closure: CXClientData,
) -> CXChildVisitResult
where
    F: FnMut(CXCursor, CXCursor) -> CXChildVisitResult,
{
    unsafe { (&mut *(closure as *mut F))(cur, parent) }
}

unsafe fn visit_children<F>(cursor: CXCursor, mut f: F)
where
    F: FnMut(CXCursor, CXCursor) -> CXChildVisitResult,
{
    clang_visitChildren(
        cursor,
        visit_children_thunk::<F> as _,
        &mut f as *mut F as CXClientData,
    );
}

unsafe fn location_in_scope(r: CXSourceRange) -> bool {
    let start = clang_getRangeStart(r);
    let mut file = ptr::null_mut();
    clang_getSpellingLocation(
        start,
        &mut file,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );
    clang_Location_isFromMainFile(start) != 0
        && clang_Location_isInSystemHeader(start) == 0
        && file != ptr::null_mut()
}

/// tokenize_range_adjust can be used to work around LLVM bug 9069
/// https://bugs.llvm.org//show_bug.cgi?id=9069
fn file_visit_macros<F: FnMut(Vec<u8>, Vec<Token>)>(
    file: &str,
    tokenize_range_adjust: bool,
    mut visitor: F,
) {
    unsafe {
        let tu = {
            let index = clang_createIndex(true as _, false as _);
            let cfile = ffi::CString::new(file).unwrap();
            let mut tu = mem::MaybeUninit::uninit();
            assert!(
                clang_parseTranslationUnit2(
                    index,
                    cfile.as_ptr(),
                    [b"-std=c11\0".as_ptr() as *const ::std::os::raw::c_char].as_ptr(),
                    1,
                    ptr::null_mut(),
                    0,
                    CXTranslationUnit_DetailedPreprocessingRecord,
                    &mut *tu.as_mut_ptr()
                ) == CXError_Success,
                "Failure reading test case {}",
                file
            );
            tu.assume_init()
        };
        visit_children(clang_getTranslationUnitCursor(tu), |cur, _parent| {
            if cur.kind == CXCursor_MacroDefinition {
                let mut range = clang_getCursorExtent(cur);
                if !location_in_scope(range) {
                    return CXChildVisit_Continue;
                }
                range.end_int_data -= if tokenize_range_adjust { 1 } else { 0 };
                let mut token_ptr = ptr::null_mut();
                let mut num = 0;
                clang_tokenize(tu, range, &mut token_ptr, &mut num);
                if token_ptr != ptr::null_mut() {
                    let tokens = slice::from_raw_parts(token_ptr, num as usize);
                    let tokens: Vec<_> = tokens
                        .iter()
                        .filter_map(|t| {
                            if clang_getTokenKind(*t) != CXToken_Comment {
                                Some(token_clang_to_cexpr(tu, t))
                            } else {
                                None
                            }
                        })
                        .collect();
                    clang_disposeTokens(tu, token_ptr, num);
                    visitor(clang_str_to_vec(clang_getCursorSpelling(cur)), tokens)
                }
            }
            CXChildVisit_Continue
        });
        clang_disposeTranslationUnit(tu);
    };
}

fn test_file(file: &str) -> bool {
    let mut idents = HashMap::new();
    let mut all_succeeded = true;
    file_visit_macros(file, fix_bug_9069(), |ident, tokens| {
        all_succeeded &= test_definition(ident, &tokens, &mut idents)
    });
    all_succeeded
}

fn fix_bug_9069() -> bool {
    fn check_bug_9069() -> bool {
        let mut token_sets = vec![];
        file_visit_macros(
            "tests/input/test_llvm_bug_9069.h",
            false,
            |ident, tokens| {
                assert_eq!(&ident, b"A");
                token_sets.push(tokens);
            },
        );
        assert_eq!(token_sets.len(), 2);
        token_sets[0] != token_sets[1]
    }

    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Once;

    static CHECK_FIX: Once = Once::new();
    static FIX: AtomicBool = AtomicBool::new(false);

    CHECK_FIX.call_once(|| FIX.store(check_bug_9069(), Ordering::SeqCst));

    FIX.load(Ordering::SeqCst)
}

macro_rules! test_file {
    ($f:ident) => {
        #[test]
        fn $f() {
            assert!(
                test_file(concat!("tests/input/", stringify!($f), ".h")),
                "test_file"
            )
        }
    };
}

test_file!(floats);
test_file!(chars);
test_file!(strings);
test_file!(int_signed);
test_file!(int_unsigned);
test_file!(fail);
