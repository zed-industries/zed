use crate::ast::{self, kw, HeapType};
use crate::parser::{Parse, Parser, Result};
use std::mem;

/// An expression, or a list of instructions, in the WebAssembly text format.
///
/// This expression type will parse s-expression-folded instructions into a flat
/// list of instructions for emission later on. The implicit `end` instruction
/// at the end of an expression is not included in the `instrs` field.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct Expression<'a> {
    pub instrs: Box<[Instruction<'a>]>,
}

impl<'a> Parse<'a> for Expression<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        ExpressionParser::default().parse(parser)
    }
}

/// Helper struct used to parse an `Expression` with helper methods and such.
///
/// The primary purpose of this is to avoid defining expression parsing as a
/// call-thread-stack recursive function. Since we're parsing user input that
/// runs the risk of blowing the call stack, so we want to be sure to use a heap
/// stack structure wherever possible.
#[derive(Default)]
struct ExpressionParser<'a> {
    /// The flat list of instructions that we've parsed so far, and will
    /// eventually become the final `Expression`.
    instrs: Vec<Instruction<'a>>,

    /// Descriptor of all our nested s-expr blocks. This only happens when
    /// instructions themselves are nested.
    stack: Vec<Level<'a>>,
}

enum Paren {
    None,
    Left,
    Right,
}

/// A "kind" of nested block that we can be parsing inside of.
enum Level<'a> {
    /// This is a normal `block` or `loop` or similar, where the instruction
    /// payload here is pushed when the block is exited.
    EndWith(Instruction<'a>),

    /// This is a pretty special variant which means that we're parsing an `if`
    /// statement, and the state of the `if` parsing is tracked internally in
    /// the payload.
    If(If<'a>),

    /// This means we're either parsing inside of `(then ...)` or `(else ...)`
    /// which don't correspond to terminating instructions, we're just in a
    /// nested block.
    IfArm,

    /// Similar to `If` but for `Try` statements, which has simpler parsing
    /// state to track.
    Try(Try<'a>),

    /// Similar to `IfArm` but for `(do ...)` and `(catch ...)` blocks.
    TryArm,
}

/// Possible states of "what should be parsed next?" in an `if` expression.
enum If<'a> {
    /// Only the `if` has been parsed, next thing to parse is the clause, if
    /// any, of the `if` instruction.
    Clause(Instruction<'a>),
    /// Next thing to parse is the `then` block
    Then(Instruction<'a>),
    /// Next thing to parse is the `else` block
    Else,
    /// This `if` statement has finished parsing and if anything remains it's a
    /// syntax error.
    End,
}

/// Possible state of "what should be parsed next?" in a `try` expression.
enum Try<'a> {
    /// Next thing to parse is the `do` block.
    Do(Instruction<'a>),
    /// Next thing to parse is `catch`/`catch_all`, `unwind`, or `delegate`.
    CatchUnwindOrDelegate,
    /// Next thing to parse is a `catch` block or `catch_all`.
    Catch,
    /// Finished parsing like the `End` case, but does not push `end` opcode.
    Delegate,
    /// This `try` statement has finished parsing and if anything remains it's a
    /// syntax error.
    End,
}

impl<'a> ExpressionParser<'a> {
    fn parse(mut self, parser: Parser<'a>) -> Result<Expression<'a>> {
        // Here we parse instructions in a loop, and we do not recursively
        // invoke this parse function to avoid blowing the stack on
        // deeply-recursive parses.
        //
        // Our loop generally only finishes once there's no more input left int
        // the `parser`. If there's some unclosed delimiters though (on our
        // `stack`), then we also keep parsing to generate error messages if
        // there's no input left.
        while !parser.is_empty() || !self.stack.is_empty() {
            // As a small ease-of-life adjustment here, if we're parsing inside
            // of an `if block then we require that all sub-components are
            // s-expressions surrounded by `(` and `)`, so verify that here.
            if let Some(Level::If(_)) | Some(Level::Try(_)) = self.stack.last() {
                if !parser.is_empty() && !parser.peek::<ast::LParen>() {
                    return Err(parser.error("expected `(`"));
                }
            }

            match self.paren(parser)? {
                // No parenthesis seen? Then we just parse the next instruction
                // and move on.
                Paren::None => self.instrs.push(parser.parse()?),

                // If we see a left-parenthesis then things are a little
                // special. We handle block-like instructions specially
                // (`block`, `loop`, and `if`), and otherwise all other
                // instructions simply get appended once we reach the end of the
                // s-expression.
                //
                // In all cases here we push something onto the `stack` to get
                // popped when the `)` character is seen.
                Paren::Left => {
                    // First up is handling `if` parsing, which is funky in a
                    // whole bunch of ways. See the method internally for more
                    // information.
                    if self.handle_if_lparen(parser)? {
                        continue;
                    }
                    // Second, we handle `try` parsing, which is simpler than
                    // `if` but more complicated than, e.g., `block`.
                    if self.handle_try_lparen(parser)? {
                        continue;
                    }
                    match parser.parse()? {
                        // If block/loop show up then we just need to be sure to
                        // push an `end` instruction whenever the `)` token is
                        // seen
                        i @ Instruction::Block(_)
                        | i @ Instruction::Loop(_)
                        | i @ Instruction::Let(_) => {
                            self.instrs.push(i);
                            self.stack.push(Level::EndWith(Instruction::End(None)));
                        }

                        // Parsing an `if` instruction is super tricky, so we
                        // push an `If` scope and we let all our scope-based
                        // parsing handle the remaining items.
                        i @ Instruction::If(_) => {
                            self.stack.push(Level::If(If::Clause(i)));
                        }

                        // Parsing a `try` is easier than `if` but we also push
                        // a `Try` scope to handle the required nested blocks.
                        i @ Instruction::Try(_) => {
                            self.stack.push(Level::Try(Try::Do(i)));
                        }

                        // Anything else means that we're parsing a nested form
                        // such as `(i32.add ...)` which means that the
                        // instruction we parsed will be coming at the end.
                        other => self.stack.push(Level::EndWith(other)),
                    }
                }

                // If we registered a `)` token as being seen, then we're
                // guaranteed there's an item in the `stack` stack for us to
                // pop. We peel that off and take a look at what it says to do.
                Paren::Right => match self.stack.pop().unwrap() {
                    Level::EndWith(i) => self.instrs.push(i),
                    Level::IfArm => {}
                    Level::TryArm => {}

                    // If an `if` statement hasn't parsed the clause or `then`
                    // block, then that's an error because there weren't enough
                    // items in the `if` statement. Otherwise we're just careful
                    // to terminate with an `end` instruction.
                    Level::If(If::Clause(_)) => {
                        return Err(parser.error("previous `if` had no clause"));
                    }
                    Level::If(If::Then(_)) => {
                        return Err(parser.error("previous `if` had no `then`"));
                    }
                    Level::If(_) => {
                        self.instrs.push(Instruction::End(None));
                    }

                    // Both `do` and `catch` are required in a `try` statement, so
                    // we will signal those errors here. Otherwise, terminate with
                    // an `end` or `delegate` instruction.
                    Level::Try(Try::Do(_)) => {
                        return Err(parser.error("previous `try` had no `do`"));
                    }
                    Level::Try(Try::CatchUnwindOrDelegate) => {
                        return Err(parser.error(
                            "previous `try` had no `catch`, `catch_all`, `unwind`, or `delegate`",
                        ));
                    }
                    Level::Try(Try::Delegate) => {}
                    Level::Try(_) => {
                        self.instrs.push(Instruction::End(None));
                    }
                },
            }
        }

        Ok(Expression {
            instrs: self.instrs.into(),
        })
    }

    /// Parses either `(`, `)`, or nothing.
    fn paren(&self, parser: Parser<'a>) -> Result<Paren> {
        parser.step(|cursor| {
            Ok(match cursor.lparen() {
                Some(rest) => (Paren::Left, rest),
                None if self.stack.is_empty() => (Paren::None, cursor),
                None => match cursor.rparen() {
                    Some(rest) => (Paren::Right, rest),
                    None => (Paren::None, cursor),
                },
            })
        })
    }

    /// Handles all parsing of an `if` statement.
    ///
    /// The syntactical form of an `if` stament looks like:
    ///
    /// ```wat
    /// (if $clause (then $then) (else $else))
    /// ```
    ///
    /// but it turns out we practically see a few things in the wild:
    ///
    /// * inside the `(if ...)` every sub-thing is surrounded by parens
    /// * The `then` and `else` keywords are optional
    /// * The `$then` and `$else` blocks don't need to be surrounded by parens
    ///
    /// That's all attempted to be handled here. The part about all sub-parts
    /// being surrounded by `(` and `)` means that we hook into the `LParen`
    /// parsing above to call this method there unconditionally.
    ///
    /// Returns `true` if the rest of the arm above should be skipped, or
    /// `false` if we should parse the next item as an instruction (because we
    /// didn't handle the lparen here).
    fn handle_if_lparen(&mut self, parser: Parser<'a>) -> Result<bool> {
        // Only execute the code below if there's an `If` listed last.
        let i = match self.stack.last_mut() {
            Some(Level::If(i)) => i,
            _ => return Ok(false),
        };

        // The first thing parsed in an `if` statement is the clause. If the
        // clause starts with `then`, however, then we know to skip the clause
        // and fall through to below.
        if let If::Clause(if_instr) = i {
            let instr = mem::replace(if_instr, Instruction::End(None));
            *i = If::Then(instr);
            if !parser.peek::<kw::then>() {
                return Ok(false);
            }
        }

        // All `if` statements are required to have a `then`. This is either the
        // second s-expr (with or without a leading `then`) or the first s-expr
        // with a leading `then`. The optionality of `then` isn't strictly what
        // the text spec says but it matches wabt for now.
        //
        // Note that when we see the `then`, that's when we actually add the
        // original `if` instruction to the stream.
        if let If::Then(if_instr) = i {
            let instr = mem::replace(if_instr, Instruction::End(None));
            self.instrs.push(instr);
            *i = If::Else;
            if parser.parse::<Option<kw::then>>()?.is_some() {
                self.stack.push(Level::IfArm);
                return Ok(true);
            }
            return Ok(false);
        }

        // effectively the same as the `then` parsing above
        if let If::Else = i {
            self.instrs.push(Instruction::Else(None));
            if parser.parse::<Option<kw::r#else>>()?.is_some() {
                if parser.is_empty() {
                    self.instrs.pop();
                }
                self.stack.push(Level::IfArm);
                return Ok(true);
            }
            *i = If::End;
            return Ok(false);
        }

        // If we made it this far then we're at `If::End` which means that there
        // were too many s-expressions inside the `(if)` and we don't want to
        // parse anything else.
        Err(parser.error("too many payloads inside of `(if)`"))
    }

    /// Handles parsing of a `try` statement. A `try` statement is simpler
    /// than an `if` as the syntactic form is:
    ///
    /// ```wat
    /// (try (do $do) (catch $event $catch))
    /// ```
    ///
    /// where the `do` and `catch` keywords are mandatory, even for an empty
    /// $do or $catch.
    ///
    /// Returns `true` if the rest of the arm above should be skipped, or
    /// `false` if we should parse the next item as an instruction (because we
    /// didn't handle the lparen here).
    fn handle_try_lparen(&mut self, parser: Parser<'a>) -> Result<bool> {
        // Only execute the code below if there's a `Try` listed last.
        let i = match self.stack.last_mut() {
            Some(Level::Try(i)) => i,
            _ => return Ok(false),
        };

        // Try statements must start with a `do` block.
        if let Try::Do(try_instr) = i {
            let instr = mem::replace(try_instr, Instruction::End(None));
            self.instrs.push(instr);
            if parser.parse::<Option<kw::r#do>>()?.is_some() {
                // The state is advanced here only if the parse succeeds in
                // order to strictly require the keyword.
                *i = Try::CatchUnwindOrDelegate;
                self.stack.push(Level::TryArm);
                return Ok(true);
            }
            // We return here and continue parsing instead of raising an error
            // immediately because the missing keyword will be caught more
            // generally in the `Paren::Right` case in `parse`.
            return Ok(false);
        }

        // After a try's `do`, there are several possible kinds of handlers.
        if let Try::CatchUnwindOrDelegate = i {
            // `catch` may be followed by more `catch`s or `catch_all`.
            if parser.parse::<Option<kw::catch>>()?.is_some() {
                let evt = parser.parse::<ast::Index<'a>>()?;
                self.instrs.push(Instruction::Catch(evt));
                *i = Try::Catch;
                self.stack.push(Level::TryArm);
                return Ok(true);
            }
            // `catch_all` can only come at the end and has no argument.
            if parser.parse::<Option<kw::catch_all>>()?.is_some() {
                self.instrs.push(Instruction::CatchAll);
                *i = Try::End;
                self.stack.push(Level::TryArm);
                return Ok(true);
            }
            // `unwind` is similar to `catch_all`.
            if parser.parse::<Option<kw::unwind>>()?.is_some() {
                self.instrs.push(Instruction::Unwind);
                *i = Try::End;
                self.stack.push(Level::TryArm);
                return Ok(true);
            }
            // `delegate` has an index, and also ends the block like `end`.
            if parser.parse::<Option<kw::delegate>>()?.is_some() {
                let depth = parser.parse::<ast::Index<'a>>()?;
                self.instrs.push(Instruction::Delegate(depth));
                *i = Try::Delegate;
                match self.paren(parser)? {
                    Paren::Left | Paren::None => return Ok(false),
                    Paren::Right => return Ok(true),
                }
            }
            return Ok(false);
        }

        if let Try::Catch = i {
            if parser.parse::<Option<kw::catch>>()?.is_some() {
                let evt = parser.parse::<ast::Index<'a>>()?;
                self.instrs.push(Instruction::Catch(evt));
                *i = Try::Catch;
                self.stack.push(Level::TryArm);
                return Ok(true);
            }
            if parser.parse::<Option<kw::catch_all>>()?.is_some() {
                self.instrs.push(Instruction::CatchAll);
                *i = Try::End;
                self.stack.push(Level::TryArm);
                return Ok(true);
            }
            return Err(parser.error("unexpected items after `catch`"));
        }

        Err(parser.error("too many payloads inside of `(try)`"))
    }
}

// TODO: document this obscenity
macro_rules! instructions {
    (pub enum Instruction<'a> {
        $(
            $(#[$doc:meta])*
            $name:ident $(($($arg:tt)*))? : [$($binary:tt)*] : $instr:tt $( | $deprecated:tt )?,
        )*
    }) => (
        /// A listing of all WebAssembly instructions that can be in a module
        /// that this crate currently parses.
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Instruction<'a> {
            $(
                $(#[$doc])*
                $name $(( instructions!(@ty $($arg)*) ))?,
            )*
        }

        #[allow(non_snake_case)]
        impl<'a> Parse<'a> for Instruction<'a> {
            fn parse(parser: Parser<'a>) -> Result<Self> {
                $(
                    fn $name<'a>(_parser: Parser<'a>) -> Result<Instruction<'a>> {
                        Ok(Instruction::$name $((
                            instructions!(@parse _parser $($arg)*)?
                        ))?)
                    }
                )*
                let parse_remainder = parser.step(|c| {
                    let (kw, rest) = match c.keyword() {
                        Some(pair) => pair,
                        None => return Err(c.error("expected an instruction")),
                    };
                    match kw {
                        $($instr $( | $deprecated )?=> Ok(($name as fn(_) -> _, rest)),)*
                        _ => return Err(c.error("unknown operator or unexpected token")),
                    }
                })?;
                parse_remainder(parser)
            }
        }

        impl crate::binary::Encode for Instruction<'_> {
            #[allow(non_snake_case)]
            fn encode(&self, v: &mut Vec<u8>) {
                match self {
                    $(
                        Instruction::$name $((instructions!(@first $($arg)*)))? => {
                            fn encode<'a>($(arg: &instructions!(@ty $($arg)*),)? v: &mut Vec<u8>) {
                                instructions!(@encode v $($binary)*);
                                $(<instructions!(@ty $($arg)*) as crate::binary::Encode>::encode(arg, v);)?
                            }
                            encode($( instructions!(@first $($arg)*), )? v)
                        }
                    )*
                }
            }
        }

        impl<'a> Instruction<'a> {
            /// Returns the associated [`MemArg`] if one is available for this
            /// instruction.
            #[allow(unused_variables, non_snake_case)]
            pub fn memarg_mut(&mut self) -> Option<&mut MemArg<'a>> {
                match self {
                    $(
                        Instruction::$name $((instructions!(@memarg_binding a $($arg)*)))? => {
                            instructions!(@get_memarg a $($($arg)*)?)
                        }
                    )*
                }
            }
        }
    );

    (@ty MemArg<$amt:tt>) => (MemArg<'a>);
    (@ty LoadOrStoreLane<$amt:tt>) => (LoadOrStoreLane<'a>);
    (@ty $other:ty) => ($other);

    (@first $first:ident $($t:tt)*) => ($first);

    (@parse $parser:ident MemArg<$amt:tt>) => (MemArg::parse($parser, $amt));
    (@parse $parser:ident MemArg) => (compile_error!("must specify `MemArg` default"));
    (@parse $parser:ident LoadOrStoreLane<$amt:tt>) => (LoadOrStoreLane::parse($parser, $amt));
    (@parse $parser:ident LoadOrStoreLane) => (compile_error!("must specify `LoadOrStoreLane` default"));
    (@parse $parser:ident $other:ty) => ($parser.parse::<$other>());

    // simd opcodes prefixed with `0xfd` get a varuint32 encoding for their payload
    (@encode $dst:ident 0xfd, $simd:tt) => ({
        $dst.push(0xfd);
        <u32 as crate::binary::Encode>::encode(&$simd, $dst);
    });
    (@encode $dst:ident $($bytes:tt)*) => ($dst.extend_from_slice(&[$($bytes)*]););

    (@get_memarg $name:ident MemArg<$amt:tt>) => (Some($name));
    (@get_memarg $name:ident LoadOrStoreLane<$amt:tt>) => (Some(&mut $name.memarg));
    (@get_memarg $($other:tt)*) => (None);

    (@memarg_binding $name:ident MemArg<$amt:tt>) => ($name);
    (@memarg_binding $name:ident LoadOrStoreLane<$amt:tt>) => ($name);
    (@memarg_binding $name:ident $other:ty) => (_);
}

instructions! {
    pub enum Instruction<'a> {
        Block(BlockType<'a>) : [0x02] : "block",
        If(BlockType<'a>) : [0x04] : "if",
        Else(Option<ast::Id<'a>>) : [0x05] : "else",
        Loop(BlockType<'a>) : [0x03] : "loop",
        End(Option<ast::Id<'a>>) : [0x0b] : "end",

        Unreachable : [0x00] : "unreachable",
        Nop : [0x01] : "nop",
        Br(ast::Index<'a>) : [0x0c] : "br",
        BrIf(ast::Index<'a>) : [0x0d] : "br_if",
        BrTable(BrTableIndices<'a>) : [0x0e] : "br_table",
        Return : [0x0f] : "return",
        Call(ast::IndexOrRef<'a, kw::func>) : [0x10] : "call",
        CallIndirect(CallIndirect<'a>) : [0x11] : "call_indirect",

        // tail-call proposal
        ReturnCall(ast::IndexOrRef<'a, kw::func>) : [0x12] : "return_call",
        ReturnCallIndirect(CallIndirect<'a>) : [0x13] : "return_call_indirect",

        // function-references proposal
        CallRef : [0x14] : "call_ref",
        ReturnCallRef : [0x15] : "return_call_ref",
        FuncBind(FuncBindType<'a>) : [0x16] : "func.bind",
        Let(LetType<'a>) : [0x17] : "let",

        Drop : [0x1a] : "drop",
        Select(SelectTypes<'a>) : [] : "select",
        LocalGet(ast::Index<'a>) : [0x20] : "local.get" | "get_local",
        LocalSet(ast::Index<'a>) : [0x21] : "local.set" | "set_local",
        LocalTee(ast::Index<'a>) : [0x22] : "local.tee" | "tee_local",
        GlobalGet(ast::IndexOrRef<'a, kw::global>) : [0x23] : "global.get" | "get_global",
        GlobalSet(ast::IndexOrRef<'a, kw::global>) : [0x24] : "global.set" | "set_global",

        TableGet(TableArg<'a>) : [0x25] : "table.get",
        TableSet(TableArg<'a>) : [0x26] : "table.set",

        I32Load(MemArg<4>) : [0x28] : "i32.load",
        I64Load(MemArg<8>) : [0x29] : "i64.load",
        F32Load(MemArg<4>) : [0x2a] : "f32.load",
        F64Load(MemArg<8>) : [0x2b] : "f64.load",
        I32Load8s(MemArg<1>) : [0x2c] : "i32.load8_s",
        I32Load8u(MemArg<1>) : [0x2d] : "i32.load8_u",
        I32Load16s(MemArg<2>) : [0x2e] : "i32.load16_s",
        I32Load16u(MemArg<2>) : [0x2f] : "i32.load16_u",
        I64Load8s(MemArg<1>) : [0x30] : "i64.load8_s",
        I64Load8u(MemArg<1>) : [0x31] : "i64.load8_u",
        I64Load16s(MemArg<2>) : [0x32] : "i64.load16_s",
        I64Load16u(MemArg<2>) : [0x33] : "i64.load16_u",
        I64Load32s(MemArg<4>) : [0x34] : "i64.load32_s",
        I64Load32u(MemArg<4>) : [0x35] : "i64.load32_u",
        I32Store(MemArg<4>) : [0x36] : "i32.store",
        I64Store(MemArg<8>) : [0x37] : "i64.store",
        F32Store(MemArg<4>) : [0x38] : "f32.store",
        F64Store(MemArg<8>) : [0x39] : "f64.store",
        I32Store8(MemArg<1>) : [0x3a] : "i32.store8",
        I32Store16(MemArg<2>) : [0x3b] : "i32.store16",
        I64Store8(MemArg<1>) : [0x3c] : "i64.store8",
        I64Store16(MemArg<2>) : [0x3d] : "i64.store16",
        I64Store32(MemArg<4>) : [0x3e] : "i64.store32",

        // Lots of bulk memory proposal here as well
        MemorySize(MemoryArg<'a>) : [0x3f] : "memory.size" | "current_memory",
        MemoryGrow(MemoryArg<'a>) : [0x40] : "memory.grow" | "grow_memory",
        MemoryInit(MemoryInit<'a>) : [0xfc, 0x08] : "memory.init",
        MemoryCopy(MemoryCopy<'a>) : [0xfc, 0x0a] : "memory.copy",
        MemoryFill(MemoryArg<'a>) : [0xfc, 0x0b] : "memory.fill",
        DataDrop(ast::Index<'a>) : [0xfc, 0x09] : "data.drop",
        ElemDrop(ast::Index<'a>) : [0xfc, 0x0d] : "elem.drop",
        TableInit(TableInit<'a>) : [0xfc, 0x0c] : "table.init",
        TableCopy(TableCopy<'a>) : [0xfc, 0x0e] : "table.copy",
        TableFill(TableArg<'a>) : [0xfc, 0x11] : "table.fill",
        TableSize(TableArg<'a>) : [0xfc, 0x10] : "table.size",
        TableGrow(TableArg<'a>) : [0xfc, 0x0f] : "table.grow",

        RefNull(HeapType<'a>) : [0xd0] : "ref.null",
        RefIsNull : [0xd1] : "ref.is_null",
        RefExtern(u32) : [0xff] : "ref.extern", // only used in test harness
        RefFunc(ast::IndexOrRef<'a, kw::func>) : [0xd2] : "ref.func",

        // function-references proposal
        RefAsNonNull : [0xd3] : "ref.as_non_null",
        BrOnNull(ast::Index<'a>) : [0xd4] : "br_on_null",

        // gc proposal: eqref
        RefEq : [0xd5] : "ref.eq",

        // gc proposal (moz specific, will be removed)
        StructNew(ast::Index<'a>) : [0xfb, 0x0] : "struct.new",

        // gc proposal: struct
        StructNewWithRtt(ast::Index<'a>) : [0xfb, 0x01] : "struct.new_with_rtt",
        StructNewDefaultWithRtt(ast::Index<'a>) : [0xfb, 0x02] : "struct.new_default_with_rtt",
        StructGet(StructAccess<'a>) : [0xfb, 0x03] : "struct.get",
        StructGetS(StructAccess<'a>) : [0xfb, 0x04] : "struct.get_s",
        StructGetU(StructAccess<'a>) : [0xfb, 0x05] : "struct.get_u",
        StructSet(StructAccess<'a>) : [0xfb, 0x06] : "struct.set",

        // gc proposal: array
        ArrayNewWithRtt(ast::Index<'a>) : [0xfb, 0x11] : "array.new_with_rtt",
        ArrayNewDefaultWithRtt(ast::Index<'a>) : [0xfb, 0x12] : "array.new_default_with_rtt",
        ArrayGet(ast::Index<'a>) : [0xfb, 0x13] : "array.get",
        ArrayGetS(ast::Index<'a>) : [0xfb, 0x14] : "array.get_s",
        ArrayGetU(ast::Index<'a>) : [0xfb, 0x15] : "array.get_u",
        ArraySet(ast::Index<'a>) : [0xfb, 0x16] : "array.set",
        ArrayLen(ast::Index<'a>) : [0xfb, 0x17] : "array.len",

        // gc proposal, i31
        I31New : [0xfb, 0x20] : "i31.new",
        I31GetS : [0xfb, 0x21] : "i31.get_s",
        I31GetU : [0xfb, 0x22] : "i31.get_u",

        // gc proposal, rtt casting
        RTTCanon(ast::Index<'a>) : [0xfb, 0x30] : "rtt.canon",
        RTTSub(ast::Index<'a>) : [0xfb, 0x31] : "rtt.sub",
        RefTest : [0xfb, 0x40] : "ref.test",
        RefCast : [0xfb, 0x41] : "ref.cast",
        BrOnCast(ast::Index<'a>) : [0xfb, 0x42] : "br_on_cast",

        // gc proposal, heap casting
        RefIsFunc : [0xfb, 0x50] : "ref.is_func",
        RefIsData : [0xfb, 0x51] : "ref.is_data",
        RefIsI31 : [0xfb, 0x52] : "ref.is_i31",
        RefAsFunc : [0xfb, 0x58] : "ref.as_func",
        RefAsData : [0xfb, 0x59] : "ref.as_data",
        RefAsI31 : [0xfb, 0x5a] : "ref.as_i31",
        BrOnFunc(ast::Index<'a>) : [0xfb, 0x60] : "br_on_func",
        BrOnData(ast::Index<'a>) : [0xfb, 0x61] : "br_on_data",
        BrOnI31(ast::Index<'a>) : [0xfb, 0x62] : "br_on_i31",

        I32Const(i32) : [0x41] : "i32.const",
        I64Const(i64) : [0x42] : "i64.const",
        F32Const(ast::Float32) : [0x43] : "f32.const",
        F64Const(ast::Float64) : [0x44] : "f64.const",

        I32Clz : [0x67] : "i32.clz",
        I32Ctz : [0x68] : "i32.ctz",
        I32Popcnt : [0x69] : "i32.popcnt",
        I32Add : [0x6a] : "i32.add",
        I32Sub : [0x6b] : "i32.sub",
        I32Mul : [0x6c] : "i32.mul",
        I32DivS : [0x6d] : "i32.div_s",
        I32DivU : [0x6e] : "i32.div_u",
        I32RemS : [0x6f] : "i32.rem_s",
        I32RemU : [0x70] : "i32.rem_u",
        I32And : [0x71] : "i32.and",
        I32Or : [0x72] : "i32.or",
        I32Xor : [0x73] : "i32.xor",
        I32Shl : [0x74] : "i32.shl",
        I32ShrS : [0x75] : "i32.shr_s",
        I32ShrU : [0x76] : "i32.shr_u",
        I32Rotl : [0x77] : "i32.rotl",
        I32Rotr : [0x78] : "i32.rotr",

        I64Clz : [0x79] : "i64.clz",
        I64Ctz : [0x7a] : "i64.ctz",
        I64Popcnt : [0x7b] : "i64.popcnt",
        I64Add : [0x7c] : "i64.add",
        I64Sub : [0x7d] : "i64.sub",
        I64Mul : [0x7e] : "i64.mul",
        I64DivS : [0x7f] : "i64.div_s",
        I64DivU : [0x80] : "i64.div_u",
        I64RemS : [0x81] : "i64.rem_s",
        I64RemU : [0x82] : "i64.rem_u",
        I64And : [0x83] : "i64.and",
        I64Or : [0x84] : "i64.or",
        I64Xor : [0x85] : "i64.xor",
        I64Shl : [0x86] : "i64.shl",
        I64ShrS : [0x87] : "i64.shr_s",
        I64ShrU : [0x88] : "i64.shr_u",
        I64Rotl : [0x89] : "i64.rotl",
        I64Rotr : [0x8a] : "i64.rotr",

        F32Abs : [0x8b] : "f32.abs",
        F32Neg : [0x8c] : "f32.neg",
        F32Ceil : [0x8d] : "f32.ceil",
        F32Floor : [0x8e] : "f32.floor",
        F32Trunc : [0x8f] : "f32.trunc",
        F32Nearest : [0x90] : "f32.nearest",
        F32Sqrt : [0x91] : "f32.sqrt",
        F32Add : [0x92] : "f32.add",
        F32Sub : [0x93] : "f32.sub",
        F32Mul : [0x94] : "f32.mul",
        F32Div : [0x95] : "f32.div",
        F32Min : [0x96] : "f32.min",
        F32Max : [0x97] : "f32.max",
        F32Copysign : [0x98] : "f32.copysign",

        F64Abs : [0x99] : "f64.abs",
        F64Neg : [0x9a] : "f64.neg",
        F64Ceil : [0x9b] : "f64.ceil",
        F64Floor : [0x9c] : "f64.floor",
        F64Trunc : [0x9d] : "f64.trunc",
        F64Nearest : [0x9e] : "f64.nearest",
        F64Sqrt : [0x9f] : "f64.sqrt",
        F64Add : [0xa0] : "f64.add",
        F64Sub : [0xa1] : "f64.sub",
        F64Mul : [0xa2] : "f64.mul",
        F64Div : [0xa3] : "f64.div",
        F64Min : [0xa4] : "f64.min",
        F64Max : [0xa5] : "f64.max",
        F64Copysign : [0xa6] : "f64.copysign",

        I32Eqz : [0x45] : "i32.eqz",
        I32Eq : [0x46] : "i32.eq",
        I32Ne : [0x47] : "i32.ne",
        I32LtS : [0x48] : "i32.lt_s",
        I32LtU : [0x49] : "i32.lt_u",
        I32GtS : [0x4a] : "i32.gt_s",
        I32GtU : [0x4b] : "i32.gt_u",
        I32LeS : [0x4c] : "i32.le_s",
        I32LeU : [0x4d] : "i32.le_u",
        I32GeS : [0x4e] : "i32.ge_s",
        I32GeU : [0x4f] : "i32.ge_u",

        I64Eqz : [0x50] : "i64.eqz",
        I64Eq : [0x51] : "i64.eq",
        I64Ne : [0x52] : "i64.ne",
        I64LtS : [0x53] : "i64.lt_s",
        I64LtU : [0x54] : "i64.lt_u",
        I64GtS : [0x55] : "i64.gt_s",
        I64GtU : [0x56] : "i64.gt_u",
        I64LeS : [0x57] : "i64.le_s",
        I64LeU : [0x58] : "i64.le_u",
        I64GeS : [0x59] : "i64.ge_s",
        I64GeU : [0x5a] : "i64.ge_u",

        F32Eq : [0x5b] : "f32.eq",
        F32Ne : [0x5c] : "f32.ne",
        F32Lt : [0x5d] : "f32.lt",
        F32Gt : [0x5e] : "f32.gt",
        F32Le : [0x5f] : "f32.le",
        F32Ge : [0x60] : "f32.ge",

        F64Eq : [0x61] : "f64.eq",
        F64Ne : [0x62] : "f64.ne",
        F64Lt : [0x63] : "f64.lt",
        F64Gt : [0x64] : "f64.gt",
        F64Le : [0x65] : "f64.le",
        F64Ge : [0x66] : "f64.ge",

        I32WrapI64 : [0xa7] : "i32.wrap_i64" | "i32.wrap/i64",
        I32TruncF32S : [0xa8] : "i32.trunc_f32_s" | "i32.trunc_s/f32",
        I32TruncF32U : [0xa9] : "i32.trunc_f32_u" | "i32.trunc_u/f32",
        I32TruncF64S : [0xaa] : "i32.trunc_f64_s" | "i32.trunc_s/f64",
        I32TruncF64U : [0xab] : "i32.trunc_f64_u" | "i32.trunc_u/f64",
        I64ExtendI32S : [0xac] : "i64.extend_i32_s" | "i64.extend_s/i32",
        I64ExtendI32U : [0xad] : "i64.extend_i32_u" | "i64.extend_u/i32",
        I64TruncF32S : [0xae] : "i64.trunc_f32_s" | "i64.trunc_s/f32",
        I64TruncF32U : [0xaf] : "i64.trunc_f32_u" | "i64.trunc_u/f32",
        I64TruncF64S : [0xb0] : "i64.trunc_f64_s" | "i64.trunc_s/f64",
        I64TruncF64U : [0xb1] : "i64.trunc_f64_u" | "i64.trunc_u/f64",
        F32ConvertI32S : [0xb2] : "f32.convert_i32_s" | "f32.convert_s/i32",
        F32ConvertI32U : [0xb3] : "f32.convert_i32_u" | "f32.convert_u/i32",
        F32ConvertI64S : [0xb4] : "f32.convert_i64_s" | "f32.convert_s/i64",
        F32ConvertI64U : [0xb5] : "f32.convert_i64_u" | "f32.convert_u/i64",
        F32DemoteF64 : [0xb6] : "f32.demote_f64" | "f32.demote/f64",
        F64ConvertI32S : [0xb7] : "f64.convert_i32_s" | "f64.convert_s/i32",
        F64ConvertI32U : [0xb8] : "f64.convert_i32_u" | "f64.convert_u/i32",
        F64ConvertI64S : [0xb9] : "f64.convert_i64_s" | "f64.convert_s/i64",
        F64ConvertI64U : [0xba] : "f64.convert_i64_u" | "f64.convert_u/i64",
        F64PromoteF32 : [0xbb] : "f64.promote_f32" | "f64.promote/f32",
        I32ReinterpretF32 : [0xbc] : "i32.reinterpret_f32" | "i32.reinterpret/f32",
        I64ReinterpretF64 : [0xbd] : "i64.reinterpret_f64" | "i64.reinterpret/f64",
        F32ReinterpretI32 : [0xbe] : "f32.reinterpret_i32" | "f32.reinterpret/i32",
        F64ReinterpretI64 : [0xbf] : "f64.reinterpret_i64" | "f64.reinterpret/i64",

        // non-trapping float to int
        I32TruncSatF32S : [0xfc, 0x00] : "i32.trunc_sat_f32_s" | "i32.trunc_s:sat/f32",
        I32TruncSatF32U : [0xfc, 0x01] : "i32.trunc_sat_f32_u" | "i32.trunc_u:sat/f32",
        I32TruncSatF64S : [0xfc, 0x02] : "i32.trunc_sat_f64_s" | "i32.trunc_s:sat/f64",
        I32TruncSatF64U : [0xfc, 0x03] : "i32.trunc_sat_f64_u" | "i32.trunc_u:sat/f64",
        I64TruncSatF32S : [0xfc, 0x04] : "i64.trunc_sat_f32_s" | "i64.trunc_s:sat/f32",
        I64TruncSatF32U : [0xfc, 0x05] : "i64.trunc_sat_f32_u" | "i64.trunc_u:sat/f32",
        I64TruncSatF64S : [0xfc, 0x06] : "i64.trunc_sat_f64_s" | "i64.trunc_s:sat/f64",
        I64TruncSatF64U : [0xfc, 0x07] : "i64.trunc_sat_f64_u" | "i64.trunc_u:sat/f64",

        // sign extension proposal
        I32Extend8S : [0xc0] : "i32.extend8_s",
        I32Extend16S : [0xc1] : "i32.extend16_s",
        I64Extend8S : [0xc2] : "i64.extend8_s",
        I64Extend16S : [0xc3] : "i64.extend16_s",
        I64Extend32S : [0xc4] : "i64.extend32_s",

        // atomics proposal
        MemoryAtomicNotify(MemArg<4>) : [0xfe, 0x00] : "memory.atomic.notify" | "atomic.notify",
        MemoryAtomicWait32(MemArg<4>) : [0xfe, 0x01] : "memory.atomic.wait32" | "i32.atomic.wait",
        MemoryAtomicWait64(MemArg<8>) : [0xfe, 0x02] : "memory.atomic.wait64" | "i64.atomic.wait",
        AtomicFence : [0xfe, 0x03, 0x00] : "atomic.fence",

        I32AtomicLoad(MemArg<4>) : [0xfe, 0x10] : "i32.atomic.load",
        I64AtomicLoad(MemArg<8>) : [0xfe, 0x11] : "i64.atomic.load",
        I32AtomicLoad8u(MemArg<1>) : [0xfe, 0x12] : "i32.atomic.load8_u",
        I32AtomicLoad16u(MemArg<2>) : [0xfe, 0x13] : "i32.atomic.load16_u",
        I64AtomicLoad8u(MemArg<1>) : [0xfe, 0x14] : "i64.atomic.load8_u",
        I64AtomicLoad16u(MemArg<2>) : [0xfe, 0x15] : "i64.atomic.load16_u",
        I64AtomicLoad32u(MemArg<4>) : [0xfe, 0x16] : "i64.atomic.load32_u",
        I32AtomicStore(MemArg<4>) : [0xfe, 0x17] : "i32.atomic.store",
        I64AtomicStore(MemArg<8>) : [0xfe, 0x18] : "i64.atomic.store",
        I32AtomicStore8(MemArg<1>) : [0xfe, 0x19] : "i32.atomic.store8",
        I32AtomicStore16(MemArg<2>) : [0xfe, 0x1a] : "i32.atomic.store16",
        I64AtomicStore8(MemArg<1>) : [0xfe, 0x1b] : "i64.atomic.store8",
        I64AtomicStore16(MemArg<2>) : [0xfe, 0x1c] : "i64.atomic.store16",
        I64AtomicStore32(MemArg<4>) : [0xfe, 0x1d] : "i64.atomic.store32",

        I32AtomicRmwAdd(MemArg<4>) : [0xfe, 0x1e] : "i32.atomic.rmw.add",
        I64AtomicRmwAdd(MemArg<8>) : [0xfe, 0x1f] : "i64.atomic.rmw.add",
        I32AtomicRmw8AddU(MemArg<1>) : [0xfe, 0x20] : "i32.atomic.rmw8.add_u",
        I32AtomicRmw16AddU(MemArg<2>) : [0xfe, 0x21] : "i32.atomic.rmw16.add_u",
        I64AtomicRmw8AddU(MemArg<1>) : [0xfe, 0x22] : "i64.atomic.rmw8.add_u",
        I64AtomicRmw16AddU(MemArg<2>) : [0xfe, 0x23] : "i64.atomic.rmw16.add_u",
        I64AtomicRmw32AddU(MemArg<4>) : [0xfe, 0x24] : "i64.atomic.rmw32.add_u",

        I32AtomicRmwSub(MemArg<4>) : [0xfe, 0x25] : "i32.atomic.rmw.sub",
        I64AtomicRmwSub(MemArg<8>) : [0xfe, 0x26] : "i64.atomic.rmw.sub",
        I32AtomicRmw8SubU(MemArg<1>) : [0xfe, 0x27] : "i32.atomic.rmw8.sub_u",
        I32AtomicRmw16SubU(MemArg<2>) : [0xfe, 0x28] : "i32.atomic.rmw16.sub_u",
        I64AtomicRmw8SubU(MemArg<1>) : [0xfe, 0x29] : "i64.atomic.rmw8.sub_u",
        I64AtomicRmw16SubU(MemArg<2>) : [0xfe, 0x2a] : "i64.atomic.rmw16.sub_u",
        I64AtomicRmw32SubU(MemArg<4>) : [0xfe, 0x2b] : "i64.atomic.rmw32.sub_u",

        I32AtomicRmwAnd(MemArg<4>) : [0xfe, 0x2c] : "i32.atomic.rmw.and",
        I64AtomicRmwAnd(MemArg<8>) : [0xfe, 0x2d] : "i64.atomic.rmw.and",
        I32AtomicRmw8AndU(MemArg<1>) : [0xfe, 0x2e] : "i32.atomic.rmw8.and_u",
        I32AtomicRmw16AndU(MemArg<2>) : [0xfe, 0x2f] : "i32.atomic.rmw16.and_u",
        I64AtomicRmw8AndU(MemArg<1>) : [0xfe, 0x30] : "i64.atomic.rmw8.and_u",
        I64AtomicRmw16AndU(MemArg<2>) : [0xfe, 0x31] : "i64.atomic.rmw16.and_u",
        I64AtomicRmw32AndU(MemArg<4>) : [0xfe, 0x32] : "i64.atomic.rmw32.and_u",

        I32AtomicRmwOr(MemArg<4>) : [0xfe, 0x33] : "i32.atomic.rmw.or",
        I64AtomicRmwOr(MemArg<8>) : [0xfe, 0x34] : "i64.atomic.rmw.or",
        I32AtomicRmw8OrU(MemArg<1>) : [0xfe, 0x35] : "i32.atomic.rmw8.or_u",
        I32AtomicRmw16OrU(MemArg<2>) : [0xfe, 0x36] : "i32.atomic.rmw16.or_u",
        I64AtomicRmw8OrU(MemArg<1>) : [0xfe, 0x37] : "i64.atomic.rmw8.or_u",
        I64AtomicRmw16OrU(MemArg<2>) : [0xfe, 0x38] : "i64.atomic.rmw16.or_u",
        I64AtomicRmw32OrU(MemArg<4>) : [0xfe, 0x39] : "i64.atomic.rmw32.or_u",

        I32AtomicRmwXor(MemArg<4>) : [0xfe, 0x3a] : "i32.atomic.rmw.xor",
        I64AtomicRmwXor(MemArg<8>) : [0xfe, 0x3b] : "i64.atomic.rmw.xor",
        I32AtomicRmw8XorU(MemArg<1>) : [0xfe, 0x3c] : "i32.atomic.rmw8.xor_u",
        I32AtomicRmw16XorU(MemArg<2>) : [0xfe, 0x3d] : "i32.atomic.rmw16.xor_u",
        I64AtomicRmw8XorU(MemArg<1>) : [0xfe, 0x3e] : "i64.atomic.rmw8.xor_u",
        I64AtomicRmw16XorU(MemArg<2>) : [0xfe, 0x3f] : "i64.atomic.rmw16.xor_u",
        I64AtomicRmw32XorU(MemArg<4>) : [0xfe, 0x40] : "i64.atomic.rmw32.xor_u",

        I32AtomicRmwXchg(MemArg<4>) : [0xfe, 0x41] : "i32.atomic.rmw.xchg",
        I64AtomicRmwXchg(MemArg<8>) : [0xfe, 0x42] : "i64.atomic.rmw.xchg",
        I32AtomicRmw8XchgU(MemArg<1>) : [0xfe, 0x43] : "i32.atomic.rmw8.xchg_u",
        I32AtomicRmw16XchgU(MemArg<2>) : [0xfe, 0x44] : "i32.atomic.rmw16.xchg_u",
        I64AtomicRmw8XchgU(MemArg<1>) : [0xfe, 0x45] : "i64.atomic.rmw8.xchg_u",
        I64AtomicRmw16XchgU(MemArg<2>) : [0xfe, 0x46] : "i64.atomic.rmw16.xchg_u",
        I64AtomicRmw32XchgU(MemArg<4>) : [0xfe, 0x47] : "i64.atomic.rmw32.xchg_u",

        I32AtomicRmwCmpxchg(MemArg<4>) : [0xfe, 0x48] : "i32.atomic.rmw.cmpxchg",
        I64AtomicRmwCmpxchg(MemArg<8>) : [0xfe, 0x49] : "i64.atomic.rmw.cmpxchg",
        I32AtomicRmw8CmpxchgU(MemArg<1>) : [0xfe, 0x4a] : "i32.atomic.rmw8.cmpxchg_u",
        I32AtomicRmw16CmpxchgU(MemArg<2>) : [0xfe, 0x4b] : "i32.atomic.rmw16.cmpxchg_u",
        I64AtomicRmw8CmpxchgU(MemArg<1>) : [0xfe, 0x4c] : "i64.atomic.rmw8.cmpxchg_u",
        I64AtomicRmw16CmpxchgU(MemArg<2>) : [0xfe, 0x4d] : "i64.atomic.rmw16.cmpxchg_u",
        I64AtomicRmw32CmpxchgU(MemArg<4>) : [0xfe, 0x4e] : "i64.atomic.rmw32.cmpxchg_u",

        // proposal: simd
        //
        // https://webassembly.github.io/simd/core/binary/instructions.html
        V128Load(MemArg<16>) : [0xfd, 0] : "v128.load",
        V128Load8x8S(MemArg<8>) : [0xfd, 1] : "v128.load8x8_s",
        V128Load8x8U(MemArg<8>) : [0xfd, 2] : "v128.load8x8_u",
        V128Load16x4S(MemArg<8>) : [0xfd, 3] : "v128.load16x4_s",
        V128Load16x4U(MemArg<8>) : [0xfd, 4] : "v128.load16x4_u",
        V128Load32x2S(MemArg<8>) : [0xfd, 5] : "v128.load32x2_s",
        V128Load32x2U(MemArg<8>) : [0xfd, 6] : "v128.load32x2_u",
        V128Load8Splat(MemArg<1>) : [0xfd, 7] : "v128.load8_splat",
        V128Load16Splat(MemArg<2>) : [0xfd, 8] : "v128.load16_splat",
        V128Load32Splat(MemArg<4>) : [0xfd, 9] : "v128.load32_splat",
        V128Load64Splat(MemArg<8>) : [0xfd, 10] : "v128.load64_splat",
        V128Load32Zero(MemArg<4>) : [0xfd, 92] : "v128.load32_zero",
        V128Load64Zero(MemArg<8>) : [0xfd, 93] : "v128.load64_zero",
        V128Store(MemArg<16>) : [0xfd, 11] : "v128.store",

        V128Load8Lane(LoadOrStoreLane<1>) : [0xfd, 84] : "v128.load8_lane",
        V128Load16Lane(LoadOrStoreLane<2>) : [0xfd, 85] : "v128.load16_lane",
        V128Load32Lane(LoadOrStoreLane<4>) : [0xfd, 86] : "v128.load32_lane",
        V128Load64Lane(LoadOrStoreLane<8>): [0xfd, 87] : "v128.load64_lane",
        V128Store8Lane(LoadOrStoreLane<1>) : [0xfd, 88] : "v128.store8_lane",
        V128Store16Lane(LoadOrStoreLane<2>) : [0xfd, 89] : "v128.store16_lane",
        V128Store32Lane(LoadOrStoreLane<4>) : [0xfd, 90] : "v128.store32_lane",
        V128Store64Lane(LoadOrStoreLane<8>) : [0xfd, 91] : "v128.store64_lane",

        V128Const(V128Const) : [0xfd, 12] : "v128.const",
        I8x16Shuffle(I8x16Shuffle) : [0xfd, 13] : "i8x16.shuffle",

        I8x16ExtractLaneS(LaneArg) : [0xfd, 21] : "i8x16.extract_lane_s",
        I8x16ExtractLaneU(LaneArg) : [0xfd, 22] : "i8x16.extract_lane_u",
        I8x16ReplaceLane(LaneArg) : [0xfd, 23] : "i8x16.replace_lane",
        I16x8ExtractLaneS(LaneArg) : [0xfd, 24] : "i16x8.extract_lane_s",
        I16x8ExtractLaneU(LaneArg) : [0xfd, 25] : "i16x8.extract_lane_u",
        I16x8ReplaceLane(LaneArg) : [0xfd, 26] : "i16x8.replace_lane",
        I32x4ExtractLane(LaneArg) : [0xfd, 27] : "i32x4.extract_lane",
        I32x4ReplaceLane(LaneArg) : [0xfd, 28] : "i32x4.replace_lane",
        I64x2ExtractLane(LaneArg) : [0xfd, 29] : "i64x2.extract_lane",
        I64x2ReplaceLane(LaneArg) : [0xfd, 30] : "i64x2.replace_lane",
        F32x4ExtractLane(LaneArg) : [0xfd, 31] : "f32x4.extract_lane",
        F32x4ReplaceLane(LaneArg) : [0xfd, 32] : "f32x4.replace_lane",
        F64x2ExtractLane(LaneArg) : [0xfd, 33] : "f64x2.extract_lane",
        F64x2ReplaceLane(LaneArg) : [0xfd, 34] : "f64x2.replace_lane",

        I8x16Swizzle : [0xfd, 14] : "i8x16.swizzle",
        I8x16Splat : [0xfd, 15] : "i8x16.splat",
        I16x8Splat : [0xfd, 16] : "i16x8.splat",
        I32x4Splat : [0xfd, 17] : "i32x4.splat",
        I64x2Splat : [0xfd, 18] : "i64x2.splat",
        F32x4Splat : [0xfd, 19] : "f32x4.splat",
        F64x2Splat : [0xfd, 20] : "f64x2.splat",

        I8x16Eq : [0xfd, 35] : "i8x16.eq",
        I8x16Ne : [0xfd, 36] : "i8x16.ne",
        I8x16LtS : [0xfd, 37] : "i8x16.lt_s",
        I8x16LtU : [0xfd, 38] : "i8x16.lt_u",
        I8x16GtS : [0xfd, 39] : "i8x16.gt_s",
        I8x16GtU : [0xfd, 40] : "i8x16.gt_u",
        I8x16LeS : [0xfd, 41] : "i8x16.le_s",
        I8x16LeU : [0xfd, 42] : "i8x16.le_u",
        I8x16GeS : [0xfd, 43] : "i8x16.ge_s",
        I8x16GeU : [0xfd, 44] : "i8x16.ge_u",

        I16x8Eq : [0xfd, 45] : "i16x8.eq",
        I16x8Ne : [0xfd, 46] : "i16x8.ne",
        I16x8LtS : [0xfd, 47] : "i16x8.lt_s",
        I16x8LtU : [0xfd, 48] : "i16x8.lt_u",
        I16x8GtS : [0xfd, 49] : "i16x8.gt_s",
        I16x8GtU : [0xfd, 50] : "i16x8.gt_u",
        I16x8LeS : [0xfd, 51] : "i16x8.le_s",
        I16x8LeU : [0xfd, 52] : "i16x8.le_u",
        I16x8GeS : [0xfd, 53] : "i16x8.ge_s",
        I16x8GeU : [0xfd, 54] : "i16x8.ge_u",

        I32x4Eq : [0xfd, 55] : "i32x4.eq",
        I32x4Ne : [0xfd, 56] : "i32x4.ne",
        I32x4LtS : [0xfd, 57] : "i32x4.lt_s",
        I32x4LtU : [0xfd, 58] : "i32x4.lt_u",
        I32x4GtS : [0xfd, 59] : "i32x4.gt_s",
        I32x4GtU : [0xfd, 60] : "i32x4.gt_u",
        I32x4LeS : [0xfd, 61] : "i32x4.le_s",
        I32x4LeU : [0xfd, 62] : "i32x4.le_u",
        I32x4GeS : [0xfd, 63] : "i32x4.ge_s",
        I32x4GeU : [0xfd, 64] : "i32x4.ge_u",

        I64x2Eq : [0xfd, 214] : "i64x2.eq",
        I64x2Ne : [0xfd, 215] : "i64x2.ne",
        I64x2LtS : [0xfd, 216] : "i64x2.lt_s",
        I64x2GtS : [0xfd, 217] : "i64x2.gt_s",
        I64x2LeS : [0xfd, 218] : "i64x2.le_s",
        I64x2GeS : [0xfd, 219] : "i64x2.ge_s",

        F32x4Eq : [0xfd, 65] : "f32x4.eq",
        F32x4Ne : [0xfd, 66] : "f32x4.ne",
        F32x4Lt : [0xfd, 67] : "f32x4.lt",
        F32x4Gt : [0xfd, 68] : "f32x4.gt",
        F32x4Le : [0xfd, 69] : "f32x4.le",
        F32x4Ge : [0xfd, 70] : "f32x4.ge",

        F64x2Eq : [0xfd, 71] : "f64x2.eq",
        F64x2Ne : [0xfd, 72] : "f64x2.ne",
        F64x2Lt : [0xfd, 73] : "f64x2.lt",
        F64x2Gt : [0xfd, 74] : "f64x2.gt",
        F64x2Le : [0xfd, 75] : "f64x2.le",
        F64x2Ge : [0xfd, 76] : "f64x2.ge",

        V128Not : [0xfd, 77] : "v128.not",
        V128And : [0xfd, 78] : "v128.and",
        V128Andnot : [0xfd, 79] : "v128.andnot",
        V128Or : [0xfd, 80] : "v128.or",
        V128Xor : [0xfd, 81] : "v128.xor",
        V128Bitselect : [0xfd, 82] : "v128.bitselect",
        V128AnyTrue : [0xfd, 83] : "v128.any_true",

        I8x16Abs : [0xfd, 96] : "i8x16.abs",
        I8x16Neg : [0xfd, 97] : "i8x16.neg",
        I8x16Popcnt : [0xfd, 98] : "i8x16.popcnt",
        I8x16AllTrue : [0xfd, 99] : "i8x16.all_true",
        I8x16Bitmask : [0xfd, 100] : "i8x16.bitmask",
        I8x16NarrowI16x8S : [0xfd, 101] : "i8x16.narrow_i16x8_s",
        I8x16NarrowI16x8U : [0xfd, 102] : "i8x16.narrow_i16x8_u",
        I8x16Shl : [0xfd, 107] : "i8x16.shl",
        I8x16ShrS : [0xfd, 108] : "i8x16.shr_s",
        I8x16ShrU : [0xfd, 109] : "i8x16.shr_u",
        I8x16Add : [0xfd, 110] : "i8x16.add",
        I8x16AddSatS : [0xfd, 111] : "i8x16.add_sat_s",
        I8x16AddSatU : [0xfd, 112] : "i8x16.add_sat_u",
        I8x16Sub : [0xfd, 113] : "i8x16.sub",
        I8x16SubSatS : [0xfd, 114] : "i8x16.sub_sat_s",
        I8x16SubSatU : [0xfd, 115] : "i8x16.sub_sat_u",
        I8x16MinS : [0xfd, 118] : "i8x16.min_s",
        I8x16MinU : [0xfd, 119] : "i8x16.min_u",
        I8x16MaxS : [0xfd, 120] : "i8x16.max_s",
        I8x16MaxU : [0xfd, 121] : "i8x16.max_u",
        I8x16AvgrU : [0xfd, 123] : "i8x16.avgr_u",

        I16x8ExtAddPairwiseI8x16S : [0xfd, 124] : "i16x8.extadd_pairwise_i8x16_s",
        I16x8ExtAddPairwiseI8x16U : [0xfd, 125] : "i16x8.extadd_pairwise_i8x16_u",
        I16x8Abs : [0xfd, 128] : "i16x8.abs",
        I16x8Neg : [0xfd, 129] : "i16x8.neg",
        I16x8Q15MulrSatS : [0xfd, 130] : "i16x8.q15mulr_sat_s",
        I16x8AllTrue : [0xfd, 131] : "i16x8.all_true",
        I16x8Bitmask : [0xfd, 132] : "i16x8.bitmask",
        I16x8NarrowI32x4S : [0xfd, 133] : "i16x8.narrow_i32x4_s",
        I16x8NarrowI32x4U : [0xfd, 134] : "i16x8.narrow_i32x4_u",
        I16x8ExtendLowI8x16S : [0xfd, 135] : "i16x8.extend_low_i8x16_s",
        I16x8ExtendHighI8x16S : [0xfd, 136] : "i16x8.extend_high_i8x16_s",
        I16x8ExtendLowI8x16U : [0xfd, 137] : "i16x8.extend_low_i8x16_u",
        I16x8ExtendHighI8x16u : [0xfd, 138] : "i16x8.extend_high_i8x16_u",
        I16x8Shl : [0xfd, 139] : "i16x8.shl",
        I16x8ShrS : [0xfd, 140] : "i16x8.shr_s",
        I16x8ShrU : [0xfd, 141] : "i16x8.shr_u",
        I16x8Add : [0xfd, 142] : "i16x8.add",
        I16x8AddSatS : [0xfd, 143] : "i16x8.add_sat_s",
        I16x8AddSatU : [0xfd, 144] : "i16x8.add_sat_u",
        I16x8Sub : [0xfd, 145] : "i16x8.sub",
        I16x8SubSatS : [0xfd, 146] : "i16x8.sub_sat_s",
        I16x8SubSatU : [0xfd, 147] : "i16x8.sub_sat_u",
        I16x8Mul : [0xfd, 149] : "i16x8.mul",
        I16x8MinS : [0xfd, 150] : "i16x8.min_s",
        I16x8MinU : [0xfd, 151] : "i16x8.min_u",
        I16x8MaxS : [0xfd, 152] : "i16x8.max_s",
        I16x8MaxU : [0xfd, 153] : "i16x8.max_u",
        I16x8AvgrU : [0xfd, 155] : "i16x8.avgr_u",
        I16x8ExtMulLowI8x16S : [0xfd, 156] : "i16x8.extmul_low_i8x16_s",
        I16x8ExtMulHighI8x16S : [0xfd, 157] : "i16x8.extmul_high_i8x16_s",
        I16x8ExtMulLowI8x16U : [0xfd, 158] : "i16x8.extmul_low_i8x16_u",
        I16x8ExtMulHighI8x16U : [0xfd, 159] : "i16x8.extmul_high_i8x16_u",

        I32x4ExtAddPairwiseI16x8S : [0xfd, 126] : "i32x4.extadd_pairwise_i16x8_s",
        I32x4ExtAddPairwiseI16x8U : [0xfd, 127] : "i32x4.extadd_pairwise_i16x8_u",
        I32x4Abs : [0xfd, 160] : "i32x4.abs",
        I32x4Neg : [0xfd, 161] : "i32x4.neg",
        I32x4AllTrue : [0xfd, 163] : "i32x4.all_true",
        I32x4Bitmask : [0xfd, 164] : "i32x4.bitmask",
        I32x4ExtendLowI16x8S : [0xfd, 167] : "i32x4.extend_low_i16x8_s",
        I32x4ExtendHighI16x8S : [0xfd, 168] : "i32x4.extend_high_i16x8_s",
        I32x4ExtendLowI16x8U : [0xfd, 169] : "i32x4.extend_low_i16x8_u",
        I32x4ExtendHighI16x8U : [0xfd, 170] : "i32x4.extend_high_i16x8_u",
        I32x4Shl : [0xfd, 171] : "i32x4.shl",
        I32x4ShrS : [0xfd, 172] : "i32x4.shr_s",
        I32x4ShrU : [0xfd, 173] : "i32x4.shr_u",
        I32x4Add : [0xfd, 174] : "i32x4.add",
        I32x4Sub : [0xfd, 177] : "i32x4.sub",
        I32x4Mul : [0xfd, 181] : "i32x4.mul",
        I32x4MinS : [0xfd, 182] : "i32x4.min_s",
        I32x4MinU : [0xfd, 183] : "i32x4.min_u",
        I32x4MaxS : [0xfd, 184] : "i32x4.max_s",
        I32x4MaxU : [0xfd, 185] : "i32x4.max_u",
        I32x4DotI16x8S : [0xfd, 186] : "i32x4.dot_i16x8_s",
        I32x4ExtMulLowI16x8S : [0xfd, 188] : "i32x4.extmul_low_i16x8_s",
        I32x4ExtMulHighI16x8S : [0xfd, 189] : "i32x4.extmul_high_i16x8_s",
        I32x4ExtMulLowI16x8U : [0xfd, 190] : "i32x4.extmul_low_i16x8_u",
        I32x4ExtMulHighI16x8U : [0xfd, 191] : "i32x4.extmul_high_i16x8_u",

        I64x2Abs : [0xfd, 192] : "i64x2.abs",
        I64x2Neg : [0xfd, 193] : "i64x2.neg",
        I64x2AllTrue : [0xfd, 195] : "i64x2.all_true",
        I64x2Bitmask : [0xfd, 196] : "i64x2.bitmask",
        I64x2ExtendLowI32x4S : [0xfd, 199] : "i64x2.extend_low_i32x4_s",
        I64x2ExtendHighI32x4S : [0xfd, 200] : "i64x2.extend_high_i32x4_s",
        I64x2ExtendLowI32x4U : [0xfd, 201] : "i64x2.extend_low_i32x4_u",
        I64x2ExtendHighI32x4U : [0xfd, 202] : "i64x2.extend_high_i32x4_u",
        I64x2Shl : [0xfd, 203] : "i64x2.shl",
        I64x2ShrS : [0xfd, 204] : "i64x2.shr_s",
        I64x2ShrU : [0xfd, 205] : "i64x2.shr_u",
        I64x2Add : [0xfd, 206] : "i64x2.add",
        I64x2Sub : [0xfd, 209] : "i64x2.sub",
        I64x2Mul : [0xfd, 213] : "i64x2.mul",
        I64x2ExtMulLowI32x4S : [0xfd, 220] : "i64x2.extmul_low_i32x4_s",
        I64x2ExtMulHighI32x4S : [0xfd, 221] : "i64x2.extmul_high_i32x4_s",
        I64x2ExtMulLowI32x4U : [0xfd, 222] : "i64x2.extmul_low_i32x4_u",
        I64x2ExtMulHighI32x4U : [0xfd, 223] : "i64x2.extmul_high_i32x4_u",

        F32x4Ceil : [0xfd, 103] : "f32x4.ceil",
        F32x4Floor : [0xfd, 104] : "f32x4.floor",
        F32x4Trunc : [0xfd, 105] : "f32x4.trunc",
        F32x4Nearest : [0xfd, 106] : "f32x4.nearest",
        F32x4Abs : [0xfd, 224] : "f32x4.abs",
        F32x4Neg : [0xfd, 225] : "f32x4.neg",
        F32x4Sqrt : [0xfd, 227] : "f32x4.sqrt",
        F32x4Add : [0xfd, 228] : "f32x4.add",
        F32x4Sub : [0xfd, 229] : "f32x4.sub",
        F32x4Mul : [0xfd, 230] : "f32x4.mul",
        F32x4Div : [0xfd, 231] : "f32x4.div",
        F32x4Min : [0xfd, 232] : "f32x4.min",
        F32x4Max : [0xfd, 233] : "f32x4.max",
        F32x4PMin : [0xfd, 234] : "f32x4.pmin",
        F32x4PMax : [0xfd, 235] : "f32x4.pmax",

        F64x2Ceil : [0xfd, 116] : "f64x2.ceil",
        F64x2Floor : [0xfd, 117] : "f64x2.floor",
        F64x2Trunc : [0xfd, 122] : "f64x2.trunc",
        F64x2Nearest : [0xfd, 148] : "f64x2.nearest",
        F64x2Abs : [0xfd, 236] : "f64x2.abs",
        F64x2Neg : [0xfd, 237] : "f64x2.neg",
        F64x2Sqrt : [0xfd, 239] : "f64x2.sqrt",
        F64x2Add : [0xfd, 240] : "f64x2.add",
        F64x2Sub : [0xfd, 241] : "f64x2.sub",
        F64x2Mul : [0xfd, 242] : "f64x2.mul",
        F64x2Div : [0xfd, 243] : "f64x2.div",
        F64x2Min : [0xfd, 244] : "f64x2.min",
        F64x2Max : [0xfd, 245] : "f64x2.max",
        F64x2PMin : [0xfd, 246] : "f64x2.pmin",
        F64x2PMax : [0xfd, 247] : "f64x2.pmax",

        I32x4TruncSatF32x4S : [0xfd, 248] : "i32x4.trunc_sat_f32x4_s",
        I32x4TruncSatF32x4U : [0xfd, 249] : "i32x4.trunc_sat_f32x4_u",
        F32x4ConvertI32x4S : [0xfd, 250] : "f32x4.convert_i32x4_s",
        F32x4ConvertI32x4U : [0xfd, 251] : "f32x4.convert_i32x4_u",
        I32x4TruncSatF64x2SZero : [0xfd, 252] : "i32x4.trunc_sat_f64x2_s_zero",
        I32x4TruncSatF64x2UZero : [0xfd, 253] : "i32x4.trunc_sat_f64x2_u_zero",
        F64x2ConvertLowI32x4S : [0xfd, 254] : "f64x2.convert_low_i32x4_s",
        F64x2ConvertLowI32x4U : [0xfd, 255] : "f64x2.convert_low_i32x4_u",
        F32x4DemoteF64x2Zero : [0xfd, 94] : "f32x4.demote_f64x2_zero",
        F64x2PromoteLowF32x4 : [0xfd, 95] : "f64x2.promote_low_f32x4",

        // Exception handling proposal
        Try(BlockType<'a>) : [0x06] : "try",
        Catch(ast::Index<'a>) : [0x07] : "catch",
        Throw(ast::Index<'a>) : [0x08] : "throw",
        Rethrow(ast::Index<'a>) : [0x09] : "rethrow",
        Unwind : [0x0a] : "unwind",
        Delegate(ast::Index<'a>) : [0x18] : "delegate",
        CatchAll : [0x19] : "catch_all",
    }
}

/// Extra information associated with block-related instructions.
///
/// This is used to label blocks and also annotate what types are expected for
/// the block.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct BlockType<'a> {
    pub label: Option<ast::Id<'a>>,
    pub ty: ast::TypeUse<'a, ast::FunctionType<'a>>,
}

impl<'a> Parse<'a> for BlockType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        Ok(BlockType {
            label: parser.parse()?,
            ty: parser
                .parse::<ast::TypeUse<'a, ast::FunctionTypeNoNames<'a>>>()?
                .into(),
        })
    }
}

/// Extra information associated with the func.bind instruction.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct FuncBindType<'a> {
    pub ty: ast::TypeUse<'a, ast::FunctionType<'a>>,
}

impl<'a> Parse<'a> for FuncBindType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        Ok(FuncBindType {
            ty: parser
                .parse::<ast::TypeUse<'a, ast::FunctionTypeNoNames<'a>>>()?
                .into(),
        })
    }
}

/// Extra information associated with the let instruction.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct LetType<'a> {
    pub block: BlockType<'a>,
    pub locals: Vec<ast::Local<'a>>,
}

impl<'a> Parse<'a> for LetType<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        Ok(LetType {
            block: parser.parse()?,
            locals: ast::Local::parse_remainder(parser)?,
        })
    }
}

/// Extra information associated with the `br_table` instruction.
#[allow(missing_docs)]
#[derive(Debug)]
pub struct BrTableIndices<'a> {
    pub labels: Vec<ast::Index<'a>>,
    pub default: ast::Index<'a>,
}

impl<'a> Parse<'a> for BrTableIndices<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut labels = Vec::new();
        labels.push(parser.parse()?);
        while parser.peek::<ast::Index>() {
            labels.push(parser.parse()?);
        }
        let default = labels.pop().unwrap();
        Ok(BrTableIndices { labels, default })
    }
}

/// Payload for lane-related instructions. Unsigned with no + prefix.
#[derive(Debug)]
pub struct LaneArg {
    /// The lane argument.
    pub lane: u8,
}

impl<'a> Parse<'a> for LaneArg {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let lane = parser.step(|c| {
            if let Some((i, rest)) = c.integer() {
                if i.sign() == None {
                    let (src, radix) = i.val();
                    let val = u8::from_str_radix(src, radix)
                        .map_err(|_| c.error("malformed lane index"))?;
                    Ok((val, rest))
                } else {
                    Err(c.error("unexpected token"))
                }
            } else {
                Err(c.error("expected a lane index"))
            }
        })?;
        Ok(LaneArg { lane })
    }
}

/// Payload for memory-related instructions indicating offset/alignment of
/// memory accesses.
#[derive(Debug)]
pub struct MemArg<'a> {
    /// The alignment of this access.
    ///
    /// This is not stored as a log, this is the actual alignment (e.g. 1, 2, 4,
    /// 8, etc).
    pub align: u32,
    /// The offset, in bytes of this access.
    pub offset: u32,
    /// The memory index we're accessing
    pub memory: ast::ItemRef<'a, kw::memory>,
}

impl<'a> MemArg<'a> {
    fn parse(parser: Parser<'a>, default_align: u32) -> Result<Self> {
        fn parse_field(name: &str, parser: Parser<'_>) -> Result<Option<u32>> {
            parser.step(|c| {
                let (kw, rest) = match c.keyword() {
                    Some(p) => p,
                    None => return Ok((None, c)),
                };
                if !kw.starts_with(name) {
                    return Ok((None, c));
                }
                let kw = &kw[name.len()..];
                if !kw.starts_with("=") {
                    return Ok((None, c));
                }
                let num = &kw[1..];
                let num = if num.starts_with("0x") {
                    match u32::from_str_radix(&num[2..], 16) {
                        Ok(n) => n,
                        Err(_) => return Err(c.error("i32 constant out of range")),
                    }
                } else {
                    match num.parse() {
                        Ok(n) => n,
                        Err(_) => return Err(c.error("i32 constant out of range")),
                    }
                };

                Ok((Some(num), rest))
            })
        }
        let memory = parser
            .parse::<Option<ast::ItemRef<'a, kw::memory>>>()?
            .unwrap_or(idx_zero(parser.prev_span(), kw::memory));
        let offset = parse_field("offset", parser)?.unwrap_or(0);
        let align = match parse_field("align", parser)? {
            Some(n) if !n.is_power_of_two() => {
                return Err(parser.error("alignment must be a power of two"))
            }
            n => n.unwrap_or(default_align),
        };

        Ok(MemArg {
            offset,
            align,
            memory,
        })
    }
}

fn idx_zero<T>(span: ast::Span, mk_kind: fn(ast::Span) -> T) -> ast::ItemRef<'static, T> {
    ast::ItemRef::Item {
        kind: mk_kind(span),
        idx: ast::Index::Num(0, span),
        exports: Vec::new(),
    }
}

/// Extra data associated with the `loadN_lane` and `storeN_lane` instructions.
#[derive(Debug)]
pub struct LoadOrStoreLane<'a> {
    /// The memory argument for this instruction.
    pub memarg: MemArg<'a>,
    /// The lane argument for this instruction.
    pub lane: LaneArg,
}

impl<'a> LoadOrStoreLane<'a> {
    fn parse(parser: Parser<'a>, default_align: u32) -> Result<Self> {
        Ok(LoadOrStoreLane {
            memarg: MemArg::parse(parser, default_align)?,
            lane: LaneArg::parse(parser)?,
        })
    }
}

/// Extra data associated with the `call_indirect` instruction.
#[derive(Debug)]
pub struct CallIndirect<'a> {
    /// The table that this call is going to be indexing.
    pub table: ast::ItemRef<'a, kw::table>,
    /// Type type signature that this `call_indirect` instruction is using.
    pub ty: ast::TypeUse<'a, ast::FunctionType<'a>>,
}

impl<'a> Parse<'a> for CallIndirect<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let prev_span = parser.prev_span();
        let mut table: Option<ast::IndexOrRef<_>> = parser.parse()?;
        let ty = parser.parse::<ast::TypeUse<'a, ast::FunctionTypeNoNames<'a>>>()?;
        // Turns out the official test suite at this time thinks table
        // identifiers comes first but wabt's test suites asserts differently
        // putting them second. Let's just handle both.
        if table.is_none() {
            table = parser.parse()?;
        }
        Ok(CallIndirect {
            table: table.map(|i| i.0).unwrap_or(idx_zero(prev_span, kw::table)),
            ty: ty.into(),
        })
    }
}

/// Extra data associated with the `table.init` instruction
#[derive(Debug)]
pub struct TableInit<'a> {
    /// The index of the table we're copying into.
    pub table: ast::ItemRef<'a, kw::table>,
    /// The index of the element segment we're copying into a table.
    pub elem: ast::Index<'a>,
}

impl<'a> Parse<'a> for TableInit<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let prev_span = parser.prev_span();
        let (elem, table) =
            if parser.peek::<ast::ItemRef<kw::table>>() || parser.peek2::<ast::Index>() {
                let table = parser.parse::<ast::IndexOrRef<_>>()?.0;
                (parser.parse()?, table)
            } else {
                (parser.parse()?, idx_zero(prev_span, kw::table))
            };
        Ok(TableInit { table, elem })
    }
}

/// Extra data associated with the `table.copy` instruction.
#[derive(Debug)]
pub struct TableCopy<'a> {
    /// The index of the destination table to copy into.
    pub dst: ast::ItemRef<'a, kw::table>,
    /// The index of the source table to copy from.
    pub src: ast::ItemRef<'a, kw::table>,
}

impl<'a> Parse<'a> for TableCopy<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let (dst, src) = match parser.parse::<Option<ast::IndexOrRef<_>>>()? {
            Some(dst) => (dst.0, parser.parse::<ast::IndexOrRef<_>>()?.0),
            None => (
                idx_zero(parser.prev_span(), kw::table),
                idx_zero(parser.prev_span(), kw::table),
            ),
        };
        Ok(TableCopy { dst, src })
    }
}

/// Extra data associated with unary table instructions.
#[derive(Debug)]
pub struct TableArg<'a> {
    /// The index of the table argument.
    pub dst: ast::ItemRef<'a, kw::table>,
}

impl<'a> Parse<'a> for TableArg<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let dst = if let Some(dst) = parser.parse::<Option<ast::IndexOrRef<_>>>()? {
            dst.0
        } else {
            idx_zero(parser.prev_span(), kw::table)
        };
        Ok(TableArg { dst })
    }
}

/// Extra data associated with unary memory instructions.
#[derive(Debug)]
pub struct MemoryArg<'a> {
    /// The index of the memory space.
    pub mem: ast::ItemRef<'a, kw::memory>,
}

impl<'a> Parse<'a> for MemoryArg<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mem = if let Some(mem) = parser.parse::<Option<ast::IndexOrRef<_>>>()? {
            mem.0
        } else {
            idx_zero(parser.prev_span(), kw::memory)
        };
        Ok(MemoryArg { mem })
    }
}

/// Extra data associated with the `memory.init` instruction
#[derive(Debug)]
pub struct MemoryInit<'a> {
    /// The index of the data segment we're copying into memory.
    pub data: ast::Index<'a>,
    /// The index of the memory we're copying into,
    pub mem: ast::ItemRef<'a, kw::memory>,
}

impl<'a> Parse<'a> for MemoryInit<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let data = parser.parse()?;
        let mem = parser
            .parse::<Option<ast::IndexOrRef<_>>>()?
            .map(|i| i.0)
            .unwrap_or(idx_zero(parser.prev_span(), kw::memory));
        Ok(MemoryInit { data, mem })
    }
}

/// Extra data associated with the `memory.copy` instruction
#[derive(Debug)]
pub struct MemoryCopy<'a> {
    /// The index of the memory we're copying from.
    pub src: ast::ItemRef<'a, kw::memory>,
    /// The index of the memory we're copying to.
    pub dst: ast::ItemRef<'a, kw::memory>,
}

impl<'a> Parse<'a> for MemoryCopy<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let (src, dst) = match parser.parse::<Option<ast::IndexOrRef<_>>>()? {
            Some(dst) => (parser.parse::<ast::IndexOrRef<_>>()?.0, dst.0),
            None => (
                idx_zero(parser.prev_span(), kw::memory),
                idx_zero(parser.prev_span(), kw::memory),
            ),
        };
        Ok(MemoryCopy { src, dst })
    }
}

/// Extra data associated with the `struct.get/set` instructions
#[derive(Debug)]
pub struct StructAccess<'a> {
    /// The index of the struct type we're accessing.
    pub r#struct: ast::Index<'a>,
    /// The index of the field of the struct we're accessing
    pub field: ast::Index<'a>,
}

impl<'a> Parse<'a> for StructAccess<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        Ok(StructAccess {
            r#struct: parser.parse()?,
            field: parser.parse()?,
        })
    }
}

/// Different ways to specify a `v128.const` instruction
#[derive(Debug)]
#[rustfmt::skip]
#[allow(missing_docs)]
pub enum V128Const {
    I8x16([i8; 16]),
    I16x8([i16; 8]),
    I32x4([i32; 4]),
    I64x2([i64; 2]),
    F32x4([ast::Float32; 4]),
    F64x2([ast::Float64; 2]),
}

impl V128Const {
    /// Returns the raw little-ended byte sequence used to represent this
    /// `v128` constant`
    ///
    /// This is typically suitable for encoding as the payload of the
    /// `v128.const` instruction.
    #[rustfmt::skip]
    pub fn to_le_bytes(&self) -> [u8; 16] {
        match self {
            V128Const::I8x16(arr) => [
                arr[0] as u8,
                arr[1] as u8,
                arr[2] as u8,
                arr[3] as u8,
                arr[4] as u8,
                arr[5] as u8,
                arr[6] as u8,
                arr[7] as u8,
                arr[8] as u8,
                arr[9] as u8,
                arr[10] as u8,
                arr[11] as u8,
                arr[12] as u8,
                arr[13] as u8,
                arr[14] as u8,
                arr[15] as u8,
            ],
            V128Const::I16x8(arr) => {
                let a1 = arr[0].to_le_bytes();
                let a2 = arr[1].to_le_bytes();
                let a3 = arr[2].to_le_bytes();
                let a4 = arr[3].to_le_bytes();
                let a5 = arr[4].to_le_bytes();
                let a6 = arr[5].to_le_bytes();
                let a7 = arr[6].to_le_bytes();
                let a8 = arr[7].to_le_bytes();
                [
                    a1[0], a1[1],
                    a2[0], a2[1],
                    a3[0], a3[1],
                    a4[0], a4[1],
                    a5[0], a5[1],
                    a6[0], a6[1],
                    a7[0], a7[1],
                    a8[0], a8[1],
                ]
            }
            V128Const::I32x4(arr) => {
                let a1 = arr[0].to_le_bytes();
                let a2 = arr[1].to_le_bytes();
                let a3 = arr[2].to_le_bytes();
                let a4 = arr[3].to_le_bytes();
                [
                    a1[0], a1[1], a1[2], a1[3],
                    a2[0], a2[1], a2[2], a2[3],
                    a3[0], a3[1], a3[2], a3[3],
                    a4[0], a4[1], a4[2], a4[3],
                ]
            }
            V128Const::I64x2(arr) => {
                let a1 = arr[0].to_le_bytes();
                let a2 = arr[1].to_le_bytes();
                [
                    a1[0], a1[1], a1[2], a1[3], a1[4], a1[5], a1[6], a1[7],
                    a2[0], a2[1], a2[2], a2[3], a2[4], a2[5], a2[6], a2[7],
                ]
            }
            V128Const::F32x4(arr) => {
                let a1 = arr[0].bits.to_le_bytes();
                let a2 = arr[1].bits.to_le_bytes();
                let a3 = arr[2].bits.to_le_bytes();
                let a4 = arr[3].bits.to_le_bytes();
                [
                    a1[0], a1[1], a1[2], a1[3],
                    a2[0], a2[1], a2[2], a2[3],
                    a3[0], a3[1], a3[2], a3[3],
                    a4[0], a4[1], a4[2], a4[3],
                ]
            }
            V128Const::F64x2(arr) => {
                let a1 = arr[0].bits.to_le_bytes();
                let a2 = arr[1].bits.to_le_bytes();
                [
                    a1[0], a1[1], a1[2], a1[3], a1[4], a1[5], a1[6], a1[7],
                    a2[0], a2[1], a2[2], a2[3], a2[4], a2[5], a2[6], a2[7],
                ]
            }
        }
    }
}

impl<'a> Parse<'a> for V128Const {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut l = parser.lookahead1();
        if l.peek::<kw::i8x16>() {
            parser.parse::<kw::i8x16>()?;
            Ok(V128Const::I8x16([
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ]))
        } else if l.peek::<kw::i16x8>() {
            parser.parse::<kw::i16x8>()?;
            Ok(V128Const::I16x8([
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ]))
        } else if l.peek::<kw::i32x4>() {
            parser.parse::<kw::i32x4>()?;
            Ok(V128Const::I32x4([
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ]))
        } else if l.peek::<kw::i64x2>() {
            parser.parse::<kw::i64x2>()?;
            Ok(V128Const::I64x2([parser.parse()?, parser.parse()?]))
        } else if l.peek::<kw::f32x4>() {
            parser.parse::<kw::f32x4>()?;
            Ok(V128Const::F32x4([
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ]))
        } else if l.peek::<kw::f64x2>() {
            parser.parse::<kw::f64x2>()?;
            Ok(V128Const::F64x2([parser.parse()?, parser.parse()?]))
        } else {
            Err(l.error())
        }
    }
}

/// Lanes being shuffled in the `i8x16.shuffle` instruction
#[derive(Debug)]
pub struct I8x16Shuffle {
    #[allow(missing_docs)]
    pub lanes: [u8; 16],
}

impl<'a> Parse<'a> for I8x16Shuffle {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        Ok(I8x16Shuffle {
            lanes: [
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
                parser.parse()?,
            ],
        })
    }
}

/// Payload of the `select` instructions
#[derive(Debug)]
pub struct SelectTypes<'a> {
    #[allow(missing_docs)]
    pub tys: Option<Vec<ast::ValType<'a>>>,
}

impl<'a> Parse<'a> for SelectTypes<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let mut tys = None;
        while parser.peek2::<kw::result>() {
            let mut list = Vec::new();
            parser.parens(|p| {
                p.parse::<kw::result>()?;
                while !p.is_empty() {
                    list.push(p.parse()?);
                }
                Ok(())
            })?;
            tys = Some(list);
        }
        Ok(SelectTypes { tys })
    }
}
