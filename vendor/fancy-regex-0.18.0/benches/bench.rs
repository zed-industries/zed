// Copyright 2016 The Fancy Regex Authors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::time::Duration;

use fancy_regex::internal::{analyze, compile, run_default, AnalyzeContext, CompileOptions};
use fancy_regex::Expr;
use regex::Regex;

fn parse_lifetime_re(c: &mut Criterion) {
    c.bench_function("parse_lifetime_re", |b| {
        b.iter(|| Expr::parse_tree("\\'[a-zA-Z_][a-zA-Z0-9_]*(?!\\')\\b").unwrap())
    });
}

fn parse_literal_re(c: &mut Criterion) {
    c.bench_function("parse_literal_re", |b| {
        b.iter(|| Expr::parse_tree("^\\\\([!-/:-@\\[-`\\{-~aftnrv]|[0-7]{1,3}|x[0-9a-fA-F]{2}|x\\{[0-9a-fA-F]{1,6}\\})").unwrap())
    });
}

fn parse_literal_re_regex(c: &mut Criterion) {
    c.bench_function("parse_literal_re_regex", |b| {
        b.iter(|| Regex::new("^\\\\([!-/:-@\\[-`\\{-~aftnrv]|[0-7]{1,3}|x[0-9a-fA-F]{2}|x\\{[0-9a-fA-F]{1,6}\\})").unwrap())
    });
}

fn parse_misc(c: &mut Criterion) {
    c.bench_function("parse_misc", |b| {
        b.iter(|| Expr::parse_tree("^\\p{L}|\\p{N}|\\s|.|\\d").unwrap())
    });
}

fn analyze_literal_re(c: &mut Criterion) {
    let re = "^\\\\([!-/:-@\\[-`\\{-~aftnrv]|[0-7]{1,3}|x[0-9a-fA-F]{2}|x\\{[0-9a-fA-F]{1,6}\\})";
    let tree = Expr::parse_tree(re).unwrap();
    c.bench_function("analyze_literal_re", |b| {
        b.iter(|| analyze(&tree, AnalyzeContext::default()).unwrap())
    });
}

fn run_backtrack(c: &mut Criterion) {
    let tree = Expr::parse_tree("^.*?(([ab]+)\\1b)").unwrap();
    let a = analyze(
        &tree,
        AnalyzeContext {
            explicit_capture_group_0: true,
            ..Default::default()
        },
    )
    .unwrap();
    let p = compile(
        &a,
        CompileOptions {
            anchored: true,
            contains_subroutines: tree.contains_subroutines,
            ..CompileOptions::default()
        },
    )
    .unwrap();
    c.bench_function("run_backtrack", |b| {
        b.iter(|| {
            let result = run_default(&p, "babab", 0).unwrap();
            assert_eq!(result, Some(vec![0, 5, 0, 2]));
            return result;
        })
    });
}

// The following regex is a pathological case for backtracking
// implementations, see README.md:
fn run_tricky(c: &mut Criterion) {
    let tree = Expr::parse_tree("(a|b|ab)*bc").unwrap();
    let a = analyze(&tree, AnalyzeContext::default()).unwrap();
    let p = compile(
        &a,
        CompileOptions {
            contains_subroutines: tree.contains_subroutines,
            ..CompileOptions::default()
        },
    )
    .unwrap();
    let mut s = String::new();
    for _ in 0..28 {
        s.push_str("ab");
    }
    s.push_str("ac");
    c.bench_function("run_tricky", |b| b.iter(|| run_default(&p, &s, 0).unwrap()));
}

fn run_backtrack_limit(c: &mut Criterion) {
    let tree = Expr::parse_tree("(?i)(a|b|ab)*(?>c)").unwrap();
    let a = analyze(&tree, AnalyzeContext::default()).unwrap();
    let p = compile(
        &a,
        CompileOptions {
            contains_subroutines: tree.contains_subroutines,
            ..CompileOptions::default()
        },
    )
    .unwrap();
    let s = "abababababababababababababababababababababababababababab";
    c.bench_function("run_backtrack_limit", |b| {
        b.iter(|| run_default(&p, &s, 0).unwrap_err())
    });
}

#[cfg(feature = "variable-lookbehinds")]
fn const_size_lookbehind(c: &mut Criterion) {
    // Benchmark const-size lookbehind (should use simple GoBack)
    let tree = Expr::parse_tree(r"(?<=ab)x").unwrap();
    let a = analyze(&tree, AnalyzeContext::default()).unwrap();
    let p = compile(
        &a,
        CompileOptions {
            contains_subroutines: tree.contains_subroutines,
            ..CompileOptions::default()
        },
    )
    .unwrap();
    let input = "abx";
    c.bench_function("const_size_lookbehind", |b| {
        b.iter(|| run_default(&p, input, 0).unwrap())
    });
}

#[cfg(feature = "variable-lookbehinds")]
fn variable_size_lookbehind(c: &mut Criterion) {
    // Benchmark variable-size lookbehind (uses reverse DFA)
    let tree = Expr::parse_tree(r"(?<=a+b+)x").unwrap();
    let a = analyze(&tree, AnalyzeContext::default()).unwrap();
    let p = compile(
        &a,
        CompileOptions {
            contains_subroutines: tree.contains_subroutines,
            ..CompileOptions::default()
        },
    )
    .unwrap();
    let input = "aaabbbbx";
    c.bench_function("variable_size_lookbehind", |b| {
        b.iter(|| run_default(&p, input, 0).unwrap())
    });
}

#[cfg(feature = "variable-lookbehinds")]
fn variable_size_alt_lookbehind(c: &mut Criterion) {
    // Benchmark variable-size lookbehind with alternation
    let tree = Expr::parse_tree(r"(?<=a|bc)x").unwrap();
    let a = analyze(&tree, AnalyzeContext::default()).unwrap();
    let p = compile(
        &a,
        CompileOptions {
            contains_subroutines: tree.contains_subroutines,
            ..CompileOptions::default()
        },
    )
    .unwrap();
    let input = "bcx";
    c.bench_function("variable_size_alt_lookbehind", |b| {
        b.iter(|| run_default(&p, input, 0).unwrap())
    });
}

#[cfg(feature = "variable-lookbehinds")]
criterion_group!(
    name = lookbehind_benches;
    config = Criterion::default();
    targets = const_size_lookbehind,
    variable_size_lookbehind,
    variable_size_alt_lookbehind,
);

criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_secs(10));
    targets = parse_lifetime_re,
    parse_literal_re,
    parse_literal_re_regex,
    parse_misc,
    analyze_literal_re,
    run_backtrack,
    run_tricky,
);
criterion_group!(
    name = slow_benches;
    config = Criterion::default().sample_size(10);
    targets = run_backtrack_limit,
);

fn continue_from_end_of_prev_match_short_haystack(c: &mut Criterion) {
    // Benchmark \G with a short haystack that doesn't match
    let tree = Expr::parse_tree(r"\Gfoo").unwrap();
    let a = analyze(&tree, AnalyzeContext::default()).unwrap();
    let p = compile(
        &a,
        CompileOptions {
            contains_subroutines: tree.contains_subroutines,
            ..CompileOptions::default()
        },
    )
    .unwrap();
    let input = "bar"; // 3 bytes, doesn't match
    c.bench_function("continue_from_end_of_prev_match_short_haystack", |b| {
        b.iter(|| run_default(&p, input, 0).unwrap())
    });
}

fn continue_from_end_of_prev_match_long_haystack(c: &mut Criterion) {
    // Benchmark \G with a long haystack that doesn't match
    let tree = Expr::parse_tree(r"\Gfoo").unwrap();
    let a = analyze(&tree, AnalyzeContext::default()).unwrap();
    let p = compile(
        &a,
        CompileOptions {
            contains_subroutines: tree.contains_subroutines,
            ..CompileOptions::default()
        },
    )
    .unwrap();
    let mut input = String::new();
    for _ in 0..10000 {
        input.push('x');
    }
    c.bench_function("continue_from_end_of_prev_match_long_haystack", |b| {
        b.iter(|| run_default(&p, &input, 0).unwrap())
    });
}

criterion_group!(
    name = continue_from_end_of_prev_match_benches;
    config = Criterion::default();
    targets = continue_from_end_of_prev_match_short_haystack,
    continue_from_end_of_prev_match_long_haystack,
);

#[cfg(feature = "variable-lookbehinds")]
criterion_main!(
    benches,
    slow_benches,
    lookbehind_benches,
    continue_from_end_of_prev_match_benches
);

#[cfg(not(feature = "variable-lookbehinds"))]
criterion_main!(
    benches,
    slow_benches,
    continue_from_end_of_prev_match_benches
);
