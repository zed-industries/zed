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

//! A simple test app for exercising and debugging the regex engine.

use fancy_regex::internal::{
    analyze, can_compile_as_anchored, compile, optimize, run_trace, Insn, Prog,
};
use fancy_regex::*;
use std::env;
use std::fmt::{Display, Formatter, Result};
use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut args = env::args().skip(1);
    if let Some(cmd) = args.next() {
        if cmd == "parse" {
            let re = args.next().expect("expected regexp argument");
            let e = Expr::parse_tree(&re);
            println!("{:#?}", e);
        } else if cmd == "optimize" {
            let re = args.next().expect("expected regexp argument");
            let mut e = Expr::parse_tree(&re).expect("expected regexp to be parsed successfully");
            let explicit_capture_group0 = optimize(&mut e);
            println!("explicit capture group 0: {:?}", explicit_capture_group0);
            println!("{:#?}", e);
        } else if cmd == "analyze" {
            let re = args.next().expect("expected regexp argument");
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            write!(&mut handle, "{}", AnalyzeFormatterWrapper { regex: &re })
                .expect("error analyzing regexp");
        } else if cmd == "compile" {
            let re = args.next().expect("expected regexp argument");
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            write!(&mut handle, "{}", CompileFormatterWrapper { regex: &re })
                .expect("error compiling regexp");
        } else if cmd == "run" {
            let re = args.next().expect("expected regexp argument");
            let r = Regex::new(&re).unwrap();
            let text = args.next().expect("expected text argument");
            let mut pos = 0;
            if let Some(pos_str) = args.next() {
                pos = usize::from_str(&pos_str).unwrap();
            }
            if let Some(caps) = r.captures_from_pos(&text, pos).unwrap() {
                print!("captures:");
                for i in 0..caps.len() {
                    print!(" {}:", i);
                    if let Some(m) = caps.get(i) {
                        print!("[{}..{}] \"{}\"", m.start(), m.end(), m.as_str());
                    } else {
                        print!("_");
                    }
                }
                println!("");
                for cap in caps.iter() {
                    println!("iterate {:?}", cap);
                }
            } else {
                println!("no match");
            }
        } else if cmd == "trace" {
            let re = args.next().expect("expected regexp argument");
            let prog = prog(&re);
            let text = args.next().expect("expected text argument");
            run_trace(&prog, &text, 0).unwrap();
        } else if cmd == "trace-inner" {
            let re = args.next().expect("expected regexp argument");
            let tree = Expr::parse_tree(&re).unwrap();
            let text = args.next().expect("expected text argument");
            let a = analyze(&tree, false).unwrap();
            let p = compile(&a, true).unwrap();
            run_trace(&p, &text, 0).unwrap();
        } else if cmd == "graph" {
            let re = args.next().expect("expected regexp argument");
            graph(&re, &mut io::stdout()).expect("error making graph");
        } else {
            println!("commands: parse|analyze|compile|graph <expr>, run|trace|trace-inner <expr> <input>");
        }
    }
}

fn graph(re: &str, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
    let prog = prog(re);
    write!(writer, "digraph G {{\n")?;
    for (i, insn) in prog.body.iter().enumerate() {
        let label = format!("{:?}", insn)
            .replace(r#"\"#, r#"\\"#)
            .replace(r#"""#, r#"\""#);
        write!(writer, r#"{:3} [label="{}: {}"];{}"#, i, i, label, "\n")?;
        match *insn {
            Insn::Split(a, b) => {
                write!(writer, "{:3} -> {};\n", i, a)?;
                write!(writer, "{:3} -> {};\n", i, b)?;
            }
            Insn::Jmp(target) => {
                write!(writer, "{:3} -> {};\n", i, target)?;
            }
            Insn::End => {}
            _ => {
                write!(writer, "{:3} -> {};\n", i, i + 1)?;
            }
        }
    }
    write!(writer, "}}\n")?;
    Ok(())
}

fn show_analysis(re: &str, writer: &mut Formatter<'_>) -> Result {
    let mut tree = Expr::parse_tree(&re).unwrap();
    let requires_capture_group_fixup = optimize(&mut tree);
    let a = analyze(&tree, requires_capture_group_fixup);
    writeln!(writer, "{:#?}", a)
}

fn show_compiled_program(re: &str, writer: &mut Formatter<'_>) -> Result {
    let r = Regex::new(re).unwrap();
    r.debug_print(writer)
}

fn prog(re: &str) -> Prog {
    // one thing to note here is that we want the prog, but in lib.rs,
    // constructing a regex might not produce a prog - it may be wrapped Regex instead,
    // which means that "toy" behaves differently to tests etc.
    let mut tree = Expr::parse_tree(re).expect("Expected parsing regex to work");
    let requires_capture_group_fixup = optimize(&mut tree);
    let result = analyze(&tree, requires_capture_group_fixup).expect("Expected analyze to succeed");
    compile(&result, can_compile_as_anchored(&tree.expr)).expect("Expected compile to succeed")
}

struct AnalyzeFormatterWrapper<'a> {
    regex: &'a str,
}

impl<'a> Display for AnalyzeFormatterWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        show_analysis(self.regex, f)
    }
}

struct CompileFormatterWrapper<'a> {
    regex: &'a str,
}

impl<'a> Display for CompileFormatterWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        show_compiled_program(self.regex, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::Write;

    #[test]
    fn test_simple_graph() {
        assert_graph(
            "^a+bc?",
            "\
digraph G {
  0 [label=\"0: Save(0)\"];
  0 -> 1;
  1 [label=\"1: Delegate(Delegate { pattern: \\\"^a+bc?\\\", start_group: 1, end_group: 1 })\"];
  1 -> 2;
  2 [label=\"2: Save(1)\"];
  2 -> 3;
  3 [label=\"3: End\"];
}
",
        );
    }

    #[test]
    fn test_backref_graph() {
        assert_graph(
            r"a+(?<b>b*)(?=c)\k<b>",
            "\
digraph G {
  0 [label=\"0: Split(3, 1)\"];
  0 -> 3;
  0 -> 1;
  1 [label=\"1: Any\"];
  1 -> 2;
  2 [label=\"2: Jmp(0)\"];
  2 -> 0;
  3 [label=\"3: Save(0)\"];
  3 -> 4;
  4 [label=\"4: Lit(\\\"a\\\")\"];
  4 -> 5;
  5 [label=\"5: Split(4, 6)\"];
  5 -> 4;
  5 -> 6;
  6 [label=\"6: Save(2)\"];
  6 -> 7;
  7 [label=\"7: Split(8, 10)\"];
  7 -> 8;
  7 -> 10;
  8 [label=\"8: Lit(\\\"b\\\")\"];
  8 -> 9;
  9 [label=\"9: Jmp(7)\"];
  9 -> 7;
 10 [label=\"10: Save(3)\"];
 10 -> 11;
 11 [label=\"11: Save(4)\"];
 11 -> 12;
 12 [label=\"12: Lit(\\\"c\\\")\"];
 12 -> 13;
 13 [label=\"13: Restore(4)\"];
 13 -> 14;
 14 [label=\"14: Backref { slot: 2, casei: false }\"];
 14 -> 15;
 15 [label=\"15: Save(1)\"];
 15 -> 16;
 16 [label=\"16: End\"];
}
",
        );
    }

    #[test]
    fn test_simple_analysis() {
        assert_analysis_succeeds("a+bc?");
    }

    #[test]
    fn test_backref_analysis() {
        assert_analysis_succeeds("a+(?<b>b*)(?=c)\\k<b>");
    }

    #[test]
    fn test_compilation_fancy_debug_output() {
        let expected = "  ".to_owned()
            + "\
  0: Split(3, 1)
  1: Any
  2: Jmp(0)
  3: Save(0)
  4: Lit(\"a\")
  5: Split(4, 6)
  6: Save(2)
  7: Split(8, 10)
  8: Lit(\"b\")
  9: Jmp(7)
 10: Save(3)
 11: Save(4)
 12: Lit(\"c\")
 13: Restore(4)
 14: Backref { slot: 2, casei: false }
 15: Save(1)
 16: End
";

        assert_compiled_prog("a+(?<b>b*)(?=c)\\k<b>", &expected);
    }

    #[test]
    fn test_compilation_wrapped_debug_output() {
        let expected = "wrapped Regex \"a+bc?\", explicit_capture_group_0: false";

        assert_compiled_prog("a+bc?", &expected);
    }

    #[test]
    fn test_compilation_wrapped_debug_output_explict_capture_group_zero() {
        let expected = "wrapped Regex \"(a+b)c\", explicit_capture_group_0: true";

        assert_compiled_prog("a+b(?=c)", &expected);
    }

    #[test]
    fn test_compilation_wrapped_debug_output_explict_capture_group_zero_with_non_capture_group() {
        let expected = "wrapped Regex \"(a+b)(?:c|d)\", explicit_capture_group_0: true";

        assert_compiled_prog("a+b(?=c|d)", &expected);
    }

    fn assert_graph(re: &str, expected: &str) {
        use crate::graph;
        let mut buf = Vec::new();
        graph(&re, &mut buf).expect("error compiling regexp and building graph");
        let output = String::from_utf8(buf).expect("string not utf8");
        assert_eq!(&output, &expected);
    }

    fn assert_compiled_prog(re: &str, expected: &str) {
        use crate::CompileFormatterWrapper;
        let mut buf = Vec::new();
        write!(&mut buf, "{}", CompileFormatterWrapper { regex: &re })
            .expect("error compiling regexp");
        let output = String::from_utf8(buf).expect("string not utf8");
        assert_eq!(&output, &expected);
    }

    fn assert_analysis_succeeds(re: &str) {
        use crate::AnalyzeFormatterWrapper;
        let mut buf = Vec::new();
        write!(&mut buf, "{}", AnalyzeFormatterWrapper { regex: &re })
            .expect("error analyzing regexp");
        let output = String::from_utf8(buf).expect("string not utf8");
        println!("{}", output);
        assert!(&output.starts_with("Ok(\n    Info {"));
    }
}
