use crate::algorithm::{BreakToken, Printer};
use crate::attr;
use crate::classify;
use crate::fixup::FixupContext;
use crate::iter::IterDelimited;
use crate::path::PathKind;
use crate::precedence::Precedence;
use crate::stmt;
use crate::INDENT;
use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::{
    token, Arm, Attribute, BinOp, Block, Expr, ExprArray, ExprAssign, ExprAsync, ExprAwait,
    ExprBinary, ExprBlock, ExprBreak, ExprCall, ExprCast, ExprClosure, ExprConst, ExprContinue,
    ExprField, ExprForLoop, ExprGroup, ExprIf, ExprIndex, ExprInfer, ExprLet, ExprLit, ExprLoop,
    ExprMacro, ExprMatch, ExprMethodCall, ExprParen, ExprPath, ExprRange, ExprRawAddr,
    ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple, ExprUnary,
    ExprUnsafe, ExprWhile, ExprYield, FieldValue, Index, Label, Lit, Member, PointerMutability,
    RangeLimits, ReturnType, Stmt, Token, UnOp,
};

impl Printer {
    pub fn expr(&mut self, expr: &Expr, mut fixup: FixupContext) {
        let needs_paren = fixup.parenthesize(expr);
        if needs_paren {
            self.word("(");
            fixup = FixupContext::NONE;
        }

        let beginning_of_line = false;

        match expr {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            Expr::Array(expr) => self.expr_array(expr),
            Expr::Assign(expr) => self.expr_assign(expr, fixup),
            Expr::Async(expr) => self.expr_async(expr),
            Expr::Await(expr) => self.expr_await(expr, beginning_of_line, fixup),
            Expr::Binary(expr) => self.expr_binary(expr, fixup),
            Expr::Block(expr) => self.expr_block(expr),
            Expr::Break(expr) => self.expr_break(expr, fixup),
            Expr::Call(expr) => self.expr_call(expr, beginning_of_line, fixup),
            Expr::Cast(expr) => self.expr_cast(expr, fixup),
            Expr::Closure(expr) => self.expr_closure(expr, fixup),
            Expr::Const(expr) => self.expr_const(expr),
            Expr::Continue(expr) => self.expr_continue(expr),
            Expr::Field(expr) => self.expr_field(expr, beginning_of_line, fixup),
            Expr::ForLoop(expr) => self.expr_for_loop(expr),
            Expr::Group(expr) => self.expr_group(expr, fixup),
            Expr::If(expr) => self.expr_if(expr),
            Expr::Index(expr) => self.expr_index(expr, beginning_of_line, fixup),
            Expr::Infer(expr) => self.expr_infer(expr),
            Expr::Let(expr) => self.expr_let(expr, fixup),
            Expr::Lit(expr) => self.expr_lit(expr),
            Expr::Loop(expr) => self.expr_loop(expr),
            Expr::Macro(expr) => self.expr_macro(expr),
            Expr::Match(expr) => self.expr_match(expr),
            Expr::MethodCall(expr) => self.expr_method_call(expr, beginning_of_line, fixup),
            Expr::Paren(expr) => self.expr_paren(expr),
            Expr::Path(expr) => self.expr_path(expr),
            Expr::Range(expr) => self.expr_range(expr, fixup),
            Expr::RawAddr(expr) => self.expr_raw_addr(expr, fixup),
            Expr::Reference(expr) => self.expr_reference(expr, fixup),
            Expr::Repeat(expr) => self.expr_repeat(expr),
            Expr::Return(expr) => self.expr_return(expr, fixup),
            Expr::Struct(expr) => self.expr_struct(expr),
            Expr::Try(expr) => self.expr_try(expr, beginning_of_line, fixup),
            Expr::TryBlock(expr) => self.expr_try_block(expr),
            Expr::Tuple(expr) => self.expr_tuple(expr),
            Expr::Unary(expr) => self.expr_unary(expr, fixup),
            Expr::Unsafe(expr) => self.expr_unsafe(expr),
            Expr::Verbatim(expr) => self.expr_verbatim(expr, fixup),
            Expr::While(expr) => self.expr_while(expr),
            Expr::Yield(expr) => self.expr_yield(expr, fixup),
            _ => unimplemented!("unknown Expr"),
        }

        if needs_paren {
            self.word(")");
        }
    }

    pub fn expr_beginning_of_line(
        &mut self,
        expr: &Expr,
        mut needs_paren: bool,
        beginning_of_line: bool,
        mut fixup: FixupContext,
    ) {
        needs_paren |= fixup.parenthesize(expr);
        if needs_paren {
            self.word("(");
            fixup = FixupContext::NONE;
        }

        match expr {
            Expr::Await(expr) => self.expr_await(expr, beginning_of_line, fixup),
            Expr::Field(expr) => self.expr_field(expr, beginning_of_line, fixup),
            Expr::Index(expr) => self.expr_index(expr, beginning_of_line, fixup),
            Expr::MethodCall(expr) => self.expr_method_call(expr, beginning_of_line, fixup),
            Expr::Try(expr) => self.expr_try(expr, beginning_of_line, fixup),
            _ => self.expr(expr, fixup),
        }

        if needs_paren {
            self.word(")");
        }
    }

    fn prefix_subexpr(
        &mut self,
        expr: &Expr,
        mut needs_paren: bool,
        beginning_of_line: bool,
        mut fixup: FixupContext,
    ) {
        needs_paren |= fixup.parenthesize(expr);
        if needs_paren {
            self.word("(");
            fixup = FixupContext::NONE;
        }

        match expr {
            Expr::Await(expr) => self.prefix_subexpr_await(expr, beginning_of_line, fixup),
            Expr::Call(expr) => self.prefix_subexpr_call(expr, fixup),
            Expr::Field(expr) => self.prefix_subexpr_field(expr, beginning_of_line, fixup),
            Expr::Index(expr) => self.prefix_subexpr_index(expr, beginning_of_line, fixup),
            Expr::MethodCall(expr) => {
                let unindent_call_args = false;
                self.prefix_subexpr_method_call(expr, beginning_of_line, unindent_call_args, fixup);
            }
            Expr::Try(expr) => self.prefix_subexpr_try(expr, beginning_of_line, fixup),
            _ => {
                self.cbox(-INDENT);
                self.expr(expr, fixup);
                self.end();
            }
        }

        if needs_paren {
            self.word(")");
        }
    }

    fn expr_condition(&mut self, expr: &Expr) {
        self.cbox(0);
        self.expr(expr, FixupContext::new_condition());
        if needs_newline_if_wrap(expr) {
            self.space();
        } else {
            self.nbsp();
        }
        self.end();
    }

    pub fn subexpr(&mut self, expr: &Expr, needs_paren: bool, mut fixup: FixupContext) {
        if needs_paren {
            self.word("(");
            fixup = FixupContext::NONE;
        }

        self.expr(expr, fixup);

        if needs_paren {
            self.word(")");
        }
    }

    fn expr_array(&mut self, expr: &ExprArray) {
        self.outer_attrs(&expr.attrs);
        if expr.elems.is_empty() {
            self.word("[]");
        } else if simple_array(&expr.elems) {
            self.cbox(INDENT);
            self.word("[");
            self.zerobreak();
            self.ibox(0);
            for elem in expr.elems.iter().delimited() {
                self.expr(&elem, FixupContext::NONE);
                if !elem.is_last {
                    self.word(",");
                    self.space();
                }
            }
            self.end();
            self.trailing_comma(true);
            self.offset(-INDENT);
            self.word("]");
            self.end();
        } else {
            self.word("[");
            self.cbox(INDENT);
            self.zerobreak();
            for elem in expr.elems.iter().delimited() {
                self.expr(&elem, FixupContext::NONE);
                self.trailing_comma(elem.is_last);
            }
            self.offset(-INDENT);
            self.end();
            self.word("]");
        }
    }

    fn expr_assign(&mut self, expr: &ExprAssign, fixup: FixupContext) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_operator(
            &expr.left,
            false,
            false,
            Precedence::Assign,
        );
        let right_fixup = fixup.rightmost_subexpression_fixup(false, false, Precedence::Assign);

        self.outer_attrs(&expr.attrs);
        self.ibox(0);
        if !expr.attrs.is_empty() {
            self.word("(");
        }
        self.subexpr(&expr.left, left_prec <= Precedence::Range, left_fixup);
        self.word(" = ");
        self.neverbreak();
        self.expr(&expr.right, right_fixup);
        if !expr.attrs.is_empty() {
            self.word(")");
        }
        self.end();
    }

    fn expr_async(&mut self, expr: &ExprAsync) {
        self.outer_attrs(&expr.attrs);
        self.word("async ");
        if expr.capture.is_some() {
            self.word("move ");
        }
        self.cbox(INDENT);
        self.small_block(&expr.block, &expr.attrs);
        self.end();
    }

    fn expr_await(&mut self, expr: &ExprAwait, beginning_of_line: bool, fixup: FixupContext) {
        self.outer_attrs(&expr.attrs);
        self.cbox(INDENT);
        self.prefix_subexpr_await(expr, beginning_of_line, fixup);
        self.end();
    }

    fn prefix_subexpr_await(
        &mut self,
        expr: &ExprAwait,
        beginning_of_line: bool,
        fixup: FixupContext,
    ) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_dot(&expr.base);

        self.prefix_subexpr(
            &expr.base,
            left_prec < Precedence::Unambiguous,
            beginning_of_line,
            left_fixup,
        );
        if !(beginning_of_line && is_short_ident(&expr.base)) {
            self.scan_break(BreakToken {
                no_break: self.ends_with('.').then_some(' '),
                ..BreakToken::default()
            });
        }
        self.word(".await");
    }

    fn expr_binary(&mut self, expr: &ExprBinary, fixup: FixupContext) {
        let binop_prec = Precedence::of_binop(&expr.op);
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_operator(
            &expr.left,
            match &expr.op {
                BinOp::Sub(_)
                | BinOp::Mul(_)
                | BinOp::And(_)
                | BinOp::Or(_)
                | BinOp::BitAnd(_)
                | BinOp::BitOr(_)
                | BinOp::Shl(_)
                | BinOp::Lt(_) => true,
                _ => false,
            },
            match &expr.op {
                BinOp::Shl(_) | BinOp::Lt(_) => true,
                _ => false,
            },
            binop_prec,
        );
        let left_needs_group = match binop_prec {
            Precedence::Assign => left_prec <= Precedence::Range,
            Precedence::Compare => left_prec <= binop_prec,
            _ => left_prec < binop_prec,
        };
        let right_fixup = fixup.rightmost_subexpression_fixup(false, false, binop_prec);
        let right_needs_group = binop_prec != Precedence::Assign
            && right_fixup.rightmost_subexpression_precedence(&expr.right) <= binop_prec;

        self.outer_attrs(&expr.attrs);
        self.ibox(INDENT);
        self.ibox(-INDENT);
        if !expr.attrs.is_empty() {
            self.word("(");
        }
        self.subexpr(&expr.left, left_needs_group, left_fixup);
        self.end();
        self.space();
        self.binary_operator(&expr.op);
        self.nbsp();
        self.subexpr(&expr.right, right_needs_group, right_fixup);
        if !expr.attrs.is_empty() {
            self.word(")");
        }
        self.end();
    }

    pub fn expr_block(&mut self, expr: &ExprBlock) {
        self.outer_attrs(&expr.attrs);
        if let Some(label) = &expr.label {
            self.label(label);
        }
        self.cbox(INDENT);
        self.small_block(&expr.block, &expr.attrs);
        self.end();
    }

    fn expr_break(&mut self, expr: &ExprBreak, fixup: FixupContext) {
        self.outer_attrs(&expr.attrs);
        self.word("break");
        if let Some(lifetime) = &expr.label {
            self.nbsp();
            self.lifetime(lifetime);
        }
        if let Some(value) = &expr.expr {
            self.nbsp();
            self.subexpr(
                value,
                expr.label.is_none() && classify::expr_leading_label(value),
                fixup.rightmost_subexpression_fixup(true, true, Precedence::Jump),
            );
        }
    }

    fn expr_call(&mut self, expr: &ExprCall, beginning_of_line: bool, fixup: FixupContext) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_operator(
            &expr.func,
            true,
            false,
            Precedence::Unambiguous,
        );
        let needs_paren = if let Expr::Field(func) = &*expr.func {
            matches!(func.member, Member::Named(_))
        } else {
            left_prec < Precedence::Unambiguous
        };

        self.outer_attrs(&expr.attrs);
        self.expr_beginning_of_line(&expr.func, needs_paren, beginning_of_line, left_fixup);
        self.word("(");
        self.call_args(&expr.args);
        self.word(")");
    }

    fn prefix_subexpr_call(&mut self, expr: &ExprCall, fixup: FixupContext) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_operator(
            &expr.func,
            true,
            false,
            Precedence::Unambiguous,
        );
        let needs_paren = if let Expr::Field(func) = &*expr.func {
            matches!(func.member, Member::Named(_))
        } else {
            left_prec < Precedence::Unambiguous
        };

        let beginning_of_line = false;
        self.prefix_subexpr(&expr.func, needs_paren, beginning_of_line, left_fixup);
        self.word("(");
        self.call_args(&expr.args);
        self.word(")");
    }

    fn expr_cast(&mut self, expr: &ExprCast, fixup: FixupContext) {
        let (left_prec, left_fixup) =
            fixup.leftmost_subexpression_with_operator(&expr.expr, false, false, Precedence::Cast);

        self.outer_attrs(&expr.attrs);
        self.ibox(INDENT);
        self.ibox(-INDENT);
        if !expr.attrs.is_empty() {
            self.word("(");
        }
        self.subexpr(&expr.expr, left_prec < Precedence::Cast, left_fixup);
        self.end();
        self.space();
        self.word("as ");
        self.ty(&expr.ty);
        if !expr.attrs.is_empty() {
            self.word(")");
        }
        self.end();
    }

    fn expr_closure(&mut self, expr: &ExprClosure, fixup: FixupContext) {
        self.outer_attrs(&expr.attrs);
        self.ibox(0);
        if let Some(bound_lifetimes) = &expr.lifetimes {
            self.bound_lifetimes(bound_lifetimes);
        }
        if expr.constness.is_some() {
            self.word("const ");
        }
        if expr.movability.is_some() {
            self.word("static ");
        }
        if expr.asyncness.is_some() {
            self.word("async ");
        }
        if expr.capture.is_some() {
            self.word("move ");
        }
        self.cbox(INDENT);
        self.word("|");
        for pat in expr.inputs.iter().delimited() {
            if pat.is_first {
                self.zerobreak();
            }
            self.pat(&pat);
            if !pat.is_last {
                self.word(",");
                self.space();
            }
        }
        match &expr.output {
            ReturnType::Default => {
                self.word("|");
                self.space();
                self.offset(-INDENT);
                self.end();
                self.neverbreak();
                let wrap_in_brace = match &*expr.body {
                    Expr::Match(ExprMatch { attrs, .. }) | Expr::Call(ExprCall { attrs, .. }) => {
                        attr::has_outer(attrs)
                    }
                    body => !is_blocklike(body),
                };
                if wrap_in_brace {
                    self.cbox(INDENT);
                    let okay_to_brace = parseable_as_stmt(&expr.body);
                    self.scan_break(BreakToken {
                        pre_break: Some(if okay_to_brace { '{' } else { '(' }),
                        ..BreakToken::default()
                    });
                    self.expr(
                        &expr.body,
                        fixup.rightmost_subexpression_fixup(false, false, Precedence::Jump),
                    );
                    self.scan_break(BreakToken {
                        offset: -INDENT,
                        pre_break: (okay_to_brace && stmt::add_semi(&expr.body)).then_some(';'),
                        post_break: if okay_to_brace { "}" } else { ")" },
                        ..BreakToken::default()
                    });
                    self.end();
                } else {
                    self.expr(
                        &expr.body,
                        fixup.rightmost_subexpression_fixup(false, false, Precedence::Jump),
                    );
                }
            }
            ReturnType::Type(_arrow, ty) => {
                if !expr.inputs.is_empty() {
                    self.trailing_comma(true);
                    self.offset(-INDENT);
                }
                self.word("|");
                self.end();
                self.word(" -> ");
                self.ty(ty);
                self.nbsp();
                self.neverbreak();
                if matches!(&*expr.body, Expr::Block(body) if body.attrs.is_empty() && body.label.is_none())
                {
                    self.expr(
                        &expr.body,
                        fixup.rightmost_subexpression_fixup(false, false, Precedence::Jump),
                    );
                } else {
                    self.cbox(INDENT);
                    self.expr_as_small_block(&expr.body, 0);
                    self.end();
                }
            }
        }
        self.end();
    }

    pub fn expr_const(&mut self, expr: &ExprConst) {
        self.outer_attrs(&expr.attrs);
        self.word("const ");
        self.cbox(INDENT);
        self.small_block(&expr.block, &expr.attrs);
        self.end();
    }

    fn expr_continue(&mut self, expr: &ExprContinue) {
        self.outer_attrs(&expr.attrs);
        self.word("continue");
        if let Some(lifetime) = &expr.label {
            self.nbsp();
            self.lifetime(lifetime);
        }
    }

    fn expr_field(&mut self, expr: &ExprField, beginning_of_line: bool, fixup: FixupContext) {
        self.outer_attrs(&expr.attrs);
        self.cbox(INDENT);
        self.prefix_subexpr_field(expr, beginning_of_line, fixup);
        self.end();
    }

    fn prefix_subexpr_field(
        &mut self,
        expr: &ExprField,
        beginning_of_line: bool,
        fixup: FixupContext,
    ) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_dot(&expr.base);

        self.prefix_subexpr(
            &expr.base,
            left_prec < Precedence::Unambiguous,
            beginning_of_line,
            left_fixup,
        );
        if !(beginning_of_line && is_short_ident(&expr.base)) {
            self.scan_break(BreakToken {
                no_break: self.ends_with('.').then_some(' '),
                ..BreakToken::default()
            });
        }
        self.word(".");
        self.member(&expr.member);
    }

    fn expr_for_loop(&mut self, expr: &ExprForLoop) {
        self.outer_attrs(&expr.attrs);
        self.ibox(0);
        if let Some(label) = &expr.label {
            self.label(label);
        }
        self.word("for ");
        self.pat(&expr.pat);
        self.word(" in ");
        self.neverbreak();
        self.expr_condition(&expr.expr);
        self.word("{");
        self.neverbreak();
        self.cbox(INDENT);
        self.hardbreak_if_nonempty();
        self.inner_attrs(&expr.attrs);
        for stmt in expr.body.stmts.iter().delimited() {
            self.stmt(&stmt, stmt.is_last);
        }
        self.offset(-INDENT);
        self.end();
        self.word("}");
        self.end();
    }

    fn expr_group(&mut self, expr: &ExprGroup, fixup: FixupContext) {
        self.outer_attrs(&expr.attrs);
        self.expr(&expr.expr, fixup);
    }

    fn expr_if(&mut self, expr: &ExprIf) {
        self.outer_attrs(&expr.attrs);
        self.cbox(INDENT);
        self.word("if ");
        self.cbox(-INDENT);
        self.expr_condition(&expr.cond);
        self.end();
        if let Some((_else_token, else_branch)) = &expr.else_branch {
            let mut else_branch = &**else_branch;
            self.small_block(&expr.then_branch, &[]);
            loop {
                self.word(" else ");
                match else_branch {
                    Expr::If(expr) => {
                        self.word("if ");
                        self.cbox(-INDENT);
                        self.expr_condition(&expr.cond);
                        self.end();
                        self.small_block(&expr.then_branch, &[]);
                        if let Some((_else_token, next)) = &expr.else_branch {
                            else_branch = next;
                            continue;
                        }
                    }
                    Expr::Block(expr) => {
                        self.small_block(&expr.block, &[]);
                    }
                    // If not one of the valid expressions to exist in an else
                    // clause, wrap in a block.
                    other => self.expr_as_small_block(other, INDENT),
                }
                break;
            }
        } else if expr.then_branch.stmts.is_empty() {
            self.word("{}");
        } else {
            self.word("{");
            self.hardbreak();
            for stmt in expr.then_branch.stmts.iter().delimited() {
                self.stmt(&stmt, stmt.is_last);
            }
            self.offset(-INDENT);
            self.word("}");
        }
        self.end();
    }

    fn expr_index(&mut self, expr: &ExprIndex, beginning_of_line: bool, fixup: FixupContext) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_operator(
            &expr.expr,
            true,
            false,
            Precedence::Unambiguous,
        );

        self.outer_attrs(&expr.attrs);
        self.expr_beginning_of_line(
            &expr.expr,
            left_prec < Precedence::Unambiguous,
            beginning_of_line,
            left_fixup,
        );
        self.word("[");
        self.expr(&expr.index, FixupContext::NONE);
        self.word("]");
    }

    fn prefix_subexpr_index(
        &mut self,
        expr: &ExprIndex,
        beginning_of_line: bool,
        fixup: FixupContext,
    ) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_operator(
            &expr.expr,
            true,
            false,
            Precedence::Unambiguous,
        );

        self.prefix_subexpr(
            &expr.expr,
            left_prec < Precedence::Unambiguous,
            beginning_of_line,
            left_fixup,
        );
        self.word("[");
        self.expr(&expr.index, FixupContext::NONE);
        self.word("]");
    }

    fn expr_infer(&mut self, expr: &ExprInfer) {
        self.outer_attrs(&expr.attrs);
        self.word("_");
    }

    fn expr_let(&mut self, expr: &ExprLet, fixup: FixupContext) {
        let (right_prec, right_fixup) = fixup.rightmost_subexpression(&expr.expr, Precedence::Let);

        self.outer_attrs(&expr.attrs);
        self.ibox(0);
        self.word("let ");
        self.ibox(0);
        self.pat(&expr.pat);
        self.end();
        self.word(" = ");
        self.neverbreak();
        self.ibox(0);
        self.subexpr(&expr.expr, right_prec < Precedence::Let, right_fixup);
        self.end();
        self.end();
    }

    pub fn expr_lit(&mut self, expr: &ExprLit) {
        self.outer_attrs(&expr.attrs);
        self.lit(&expr.lit);
    }

    fn expr_loop(&mut self, expr: &ExprLoop) {
        self.outer_attrs(&expr.attrs);
        if let Some(label) = &expr.label {
            self.label(label);
        }
        self.word("loop {");
        self.cbox(INDENT);
        self.hardbreak_if_nonempty();
        self.inner_attrs(&expr.attrs);
        for stmt in expr.body.stmts.iter().delimited() {
            self.stmt(&stmt, stmt.is_last);
        }
        self.offset(-INDENT);
        self.end();
        self.word("}");
    }

    pub fn expr_macro(&mut self, expr: &ExprMacro) {
        self.outer_attrs(&expr.attrs);
        let semicolon = false;
        self.mac(&expr.mac, None, semicolon);
    }

    fn expr_match(&mut self, expr: &ExprMatch) {
        self.outer_attrs(&expr.attrs);
        self.ibox(0);
        self.word("match ");
        self.expr_condition(&expr.expr);
        self.word("{");
        self.neverbreak();
        self.cbox(INDENT);
        self.hardbreak_if_nonempty();
        self.inner_attrs(&expr.attrs);
        for arm in &expr.arms {
            self.arm(arm);
            self.hardbreak();
        }
        self.offset(-INDENT);
        self.end();
        self.word("}");
        self.end();
    }

    fn expr_method_call(
        &mut self,
        expr: &ExprMethodCall,
        beginning_of_line: bool,
        fixup: FixupContext,
    ) {
        self.outer_attrs(&expr.attrs);
        self.cbox(INDENT);
        let unindent_call_args = beginning_of_line && is_short_ident(&expr.receiver);
        self.prefix_subexpr_method_call(expr, beginning_of_line, unindent_call_args, fixup);
        self.end();
    }

    fn prefix_subexpr_method_call(
        &mut self,
        expr: &ExprMethodCall,
        beginning_of_line: bool,
        unindent_call_args: bool,
        fixup: FixupContext,
    ) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_dot(&expr.receiver);

        self.prefix_subexpr(
            &expr.receiver,
            left_prec < Precedence::Unambiguous,
            beginning_of_line,
            left_fixup,
        );
        if !(beginning_of_line && is_short_ident(&expr.receiver)) {
            self.scan_break(BreakToken {
                no_break: self.ends_with('.').then_some(' '),
                ..BreakToken::default()
            });
        }
        self.word(".");
        self.ident(&expr.method);
        if let Some(turbofish) = &expr.turbofish {
            self.angle_bracketed_generic_arguments(turbofish, PathKind::Expr);
        }
        self.cbox(if unindent_call_args { -INDENT } else { 0 });
        self.word("(");
        self.call_args(&expr.args);
        self.word(")");
        self.end();
    }

    fn expr_paren(&mut self, expr: &ExprParen) {
        self.outer_attrs(&expr.attrs);
        self.word("(");
        self.expr(&expr.expr, FixupContext::NONE);
        self.word(")");
    }

    pub fn expr_path(&mut self, expr: &ExprPath) {
        self.outer_attrs(&expr.attrs);
        self.qpath(&expr.qself, &expr.path, PathKind::Expr);
    }

    pub fn expr_range(&mut self, expr: &ExprRange, fixup: FixupContext) {
        self.outer_attrs(&expr.attrs);
        if !expr.attrs.is_empty() {
            self.word("(");
        }
        if let Some(start) = &expr.start {
            let (left_prec, left_fixup) =
                fixup.leftmost_subexpression_with_operator(start, true, false, Precedence::Range);
            self.subexpr(start, left_prec <= Precedence::Range, left_fixup);
        } else if self.ends_with('.') {
            self.nbsp();
        }
        self.word(match expr.limits {
            RangeLimits::HalfOpen(_) => "..",
            RangeLimits::Closed(_) => "..=",
        });
        if let Some(end) = &expr.end {
            let right_fixup = fixup.rightmost_subexpression_fixup(false, true, Precedence::Range);
            let right_prec = right_fixup.rightmost_subexpression_precedence(end);
            self.subexpr(end, right_prec <= Precedence::Range, right_fixup);
        }
        if !expr.attrs.is_empty() {
            self.word(")");
        }
    }

    fn expr_raw_addr(&mut self, expr: &ExprRawAddr, fixup: FixupContext) {
        let (right_prec, right_fixup) =
            fixup.rightmost_subexpression(&expr.expr, Precedence::Prefix);

        self.outer_attrs(&expr.attrs);
        self.word("&raw ");
        self.pointer_mutability(&expr.mutability);
        self.nbsp();
        self.subexpr(&expr.expr, right_prec < Precedence::Prefix, right_fixup);
    }

    fn expr_reference(&mut self, expr: &ExprReference, fixup: FixupContext) {
        let (right_prec, right_fixup) =
            fixup.rightmost_subexpression(&expr.expr, Precedence::Prefix);

        self.outer_attrs(&expr.attrs);
        self.word("&");
        if expr.mutability.is_some() {
            self.word("mut ");
        }
        self.subexpr(&expr.expr, right_prec < Precedence::Prefix, right_fixup);
    }

    fn expr_repeat(&mut self, expr: &ExprRepeat) {
        self.outer_attrs(&expr.attrs);
        self.word("[");
        self.expr(&expr.expr, FixupContext::NONE);
        self.word("; ");
        self.expr(&expr.len, FixupContext::NONE);
        self.word("]");
    }

    fn expr_return(&mut self, expr: &ExprReturn, fixup: FixupContext) {
        self.outer_attrs(&expr.attrs);
        self.word("return");
        if let Some(value) = &expr.expr {
            self.nbsp();
            self.expr(
                value,
                fixup.rightmost_subexpression_fixup(true, false, Precedence::Jump),
            );
        }
    }

    fn expr_struct(&mut self, expr: &ExprStruct) {
        self.outer_attrs(&expr.attrs);
        self.cbox(INDENT);
        self.ibox(-INDENT);
        self.qpath(&expr.qself, &expr.path, PathKind::Expr);
        self.end();
        self.word(" {");
        self.space_if_nonempty();
        for field_value in expr.fields.iter().delimited() {
            self.field_value(&field_value);
            self.trailing_comma_or_space(field_value.is_last && expr.rest.is_none());
        }
        if let Some(rest) = &expr.rest {
            self.word("..");
            self.expr(rest, FixupContext::NONE);
            self.space();
        }
        self.offset(-INDENT);
        self.end_with_max_width(34);
        self.word("}");
    }

    fn expr_try(&mut self, expr: &ExprTry, beginning_of_line: bool, fixup: FixupContext) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_dot(&expr.expr);

        self.outer_attrs(&expr.attrs);
        self.expr_beginning_of_line(
            &expr.expr,
            left_prec < Precedence::Unambiguous,
            beginning_of_line,
            left_fixup,
        );
        self.word("?");
    }

    fn prefix_subexpr_try(&mut self, expr: &ExprTry, beginning_of_line: bool, fixup: FixupContext) {
        let (left_prec, left_fixup) = fixup.leftmost_subexpression_with_dot(&expr.expr);

        self.prefix_subexpr(
            &expr.expr,
            left_prec < Precedence::Unambiguous,
            beginning_of_line,
            left_fixup,
        );
        self.word("?");
    }

    fn expr_try_block(&mut self, expr: &ExprTryBlock) {
        self.outer_attrs(&expr.attrs);
        self.word("try ");
        self.cbox(INDENT);
        self.small_block(&expr.block, &expr.attrs);
        self.end();
    }

    fn expr_tuple(&mut self, expr: &ExprTuple) {
        self.outer_attrs(&expr.attrs);
        self.word("(");
        self.cbox(INDENT);
        self.zerobreak();
        for elem in expr.elems.iter().delimited() {
            self.expr(&elem, FixupContext::NONE);
            if expr.elems.len() == 1 {
                self.word(",");
                self.zerobreak();
            } else {
                self.trailing_comma(elem.is_last);
            }
        }
        self.offset(-INDENT);
        self.end();
        self.word(")");
    }

    fn expr_unary(&mut self, expr: &ExprUnary, fixup: FixupContext) {
        let (right_prec, right_fixup) =
            fixup.rightmost_subexpression(&expr.expr, Precedence::Prefix);

        self.outer_attrs(&expr.attrs);
        self.unary_operator(&expr.op);
        self.subexpr(&expr.expr, right_prec < Precedence::Prefix, right_fixup);
    }

    fn expr_unsafe(&mut self, expr: &ExprUnsafe) {
        self.outer_attrs(&expr.attrs);
        self.word("unsafe ");
        self.cbox(INDENT);
        self.small_block(&expr.block, &expr.attrs);
        self.end();
    }

    #[cfg(not(feature = "verbatim"))]
    fn expr_verbatim(&mut self, expr: &TokenStream, _fixup: FixupContext) {
        if !expr.is_empty() {
            unimplemented!("Expr::Verbatim `{}`", expr);
        }
    }

    #[cfg(feature = "verbatim")]
    fn expr_verbatim(&mut self, tokens: &TokenStream, fixup: FixupContext) {
        use syn::parse::discouraged::Speculative;
        use syn::parse::{Parse, ParseStream, Result};
        use syn::{parenthesized, Ident};

        enum ExprVerbatim {
            Empty,
            Ellipsis,
            Become(Become),
            Builtin(Builtin),
        }

        struct Become {
            attrs: Vec<Attribute>,
            tail_call: Expr,
        }

        struct Builtin {
            attrs: Vec<Attribute>,
            name: Ident,
            args: TokenStream,
        }

        mod kw {
            syn::custom_keyword!(builtin);
            syn::custom_keyword!(raw);
        }

        impl Parse for ExprVerbatim {
            fn parse(input: ParseStream) -> Result<Self> {
                let ahead = input.fork();
                let attrs = ahead.call(Attribute::parse_outer)?;
                let lookahead = ahead.lookahead1();
                if input.is_empty() {
                    Ok(ExprVerbatim::Empty)
                } else if lookahead.peek(Token![become]) {
                    input.advance_to(&ahead);
                    input.parse::<Token![become]>()?;
                    let tail_call: Expr = input.parse()?;
                    Ok(ExprVerbatim::Become(Become { attrs, tail_call }))
                } else if lookahead.peek(kw::builtin) {
                    input.advance_to(&ahead);
                    input.parse::<kw::builtin>()?;
                    input.parse::<Token![#]>()?;
                    let name: Ident = input.parse()?;
                    let args;
                    parenthesized!(args in input);
                    let args: TokenStream = args.parse()?;
                    Ok(ExprVerbatim::Builtin(Builtin { attrs, name, args }))
                } else if lookahead.peek(Token![...]) {
                    input.parse::<Token![...]>()?;
                    Ok(ExprVerbatim::Ellipsis)
                } else {
                    Err(lookahead.error())
                }
            }
        }

        let expr: ExprVerbatim = match syn::parse2(tokens.clone()) {
            Ok(expr) => expr,
            Err(_) => unimplemented!("Expr::Verbatim `{}`", tokens),
        };

        match expr {
            ExprVerbatim::Empty => {}
            ExprVerbatim::Ellipsis => {
                self.word("...");
            }
            ExprVerbatim::Become(expr) => {
                self.outer_attrs(&expr.attrs);
                self.word("become");
                self.nbsp();
                self.expr(
                    &expr.tail_call,
                    fixup.rightmost_subexpression_fixup(true, false, Precedence::Jump),
                );
            }
            ExprVerbatim::Builtin(expr) => {
                self.outer_attrs(&expr.attrs);
                self.word("builtin # ");
                self.ident(&expr.name);
                self.word("(");
                if !expr.args.is_empty() {
                    self.cbox(INDENT);
                    self.zerobreak();
                    self.ibox(0);
                    self.macro_rules_tokens(expr.args, false);
                    self.end();
                    self.zerobreak();
                    self.offset(-INDENT);
                    self.end();
                }
                self.word(")");
            }
        }
    }

    fn expr_while(&mut self, expr: &ExprWhile) {
        self.outer_attrs(&expr.attrs);
        if let Some(label) = &expr.label {
            self.label(label);
        }
        self.word("while ");
        self.expr_condition(&expr.cond);
        self.word("{");
        self.neverbreak();
        self.cbox(INDENT);
        self.hardbreak_if_nonempty();
        self.inner_attrs(&expr.attrs);
        for stmt in expr.body.stmts.iter().delimited() {
            self.stmt(&stmt, stmt.is_last);
        }
        self.offset(-INDENT);
        self.end();
        self.word("}");
    }

    fn expr_yield(&mut self, expr: &ExprYield, fixup: FixupContext) {
        self.outer_attrs(&expr.attrs);
        self.word("yield");
        if let Some(value) = &expr.expr {
            self.nbsp();
            self.expr(
                value,
                fixup.rightmost_subexpression_fixup(true, false, Precedence::Jump),
            );
        }
    }

    fn label(&mut self, label: &Label) {
        self.lifetime(&label.name);
        self.word(": ");
    }

    fn field_value(&mut self, field_value: &FieldValue) {
        self.outer_attrs(&field_value.attrs);
        self.member(&field_value.member);
        if field_value.colon_token.is_some() {
            self.word(": ");
            self.ibox(0);
            self.expr(&field_value.expr, FixupContext::NONE);
            self.end();
        }
    }

    fn arm(&mut self, arm: &Arm) {
        self.outer_attrs(&arm.attrs);
        self.ibox(0);
        self.pat(&arm.pat);
        if let Some((_if_token, guard)) = &arm.guard {
            self.word(" if ");
            self.expr(guard, FixupContext::NONE);
        }
        self.word(" => ");
        let empty_block;
        let mut body = &*arm.body;
        while let Expr::Block(expr) = body {
            if expr.attrs.is_empty() && expr.label.is_none() {
                let mut stmts = expr.block.stmts.iter();
                if let (Some(Stmt::Expr(inner, None)), None) = (stmts.next(), stmts.next()) {
                    body = inner;
                    continue;
                }
            }
            break;
        }
        if let Expr::Tuple(expr) = body {
            if expr.elems.is_empty() && expr.attrs.is_empty() {
                empty_block = Expr::Block(ExprBlock {
                    attrs: Vec::new(),
                    label: None,
                    block: Block {
                        brace_token: token::Brace::default(),
                        stmts: Vec::new(),
                    },
                });
                body = &empty_block;
            }
        }
        if let Expr::Block(body) = body {
            if let Some(label) = &body.label {
                self.label(label);
            }
            self.word("{");
            self.neverbreak();
            self.cbox(INDENT);
            self.hardbreak_if_nonempty();
            self.inner_attrs(&body.attrs);
            for stmt in body.block.stmts.iter().delimited() {
                self.stmt(&stmt, stmt.is_last);
            }
            self.offset(-INDENT);
            self.end();
            self.word("}");
        } else {
            self.neverbreak();
            self.cbox(INDENT);
            let okay_to_brace = parseable_as_stmt(body);
            self.scan_break(BreakToken {
                pre_break: Some(if okay_to_brace { '{' } else { '(' }),
                ..BreakToken::default()
            });
            self.expr_beginning_of_line(body, false, true, FixupContext::new_match_arm());
            self.scan_break(BreakToken {
                offset: -INDENT,
                pre_break: (okay_to_brace && stmt::add_semi(body)).then_some(';'),
                post_break: if okay_to_brace { "}" } else { ")," },
                no_break: classify::requires_comma_to_be_match_arm(body).then_some(','),
                ..BreakToken::default()
            });
            self.end();
        }
        self.end();
    }

    fn call_args(&mut self, args: &Punctuated<Expr, Token![,]>) {
        let mut iter = args.iter();
        match (iter.next(), iter.next()) {
            (Some(expr), None) if is_blocklike(expr) => {
                self.expr(expr, FixupContext::NONE);
            }
            _ => {
                self.cbox(INDENT);
                self.zerobreak();
                for arg in args.iter().delimited() {
                    self.expr(&arg, FixupContext::NONE);
                    self.trailing_comma(arg.is_last);
                }
                self.offset(-INDENT);
                self.end();
            }
        }
    }

    pub fn small_block(&mut self, block: &Block, attrs: &[Attribute]) {
        self.word("{");
        if attr::has_inner(attrs) || !block.stmts.is_empty() {
            self.space();
            self.inner_attrs(attrs);
            match block.stmts.as_slice() {
                [Stmt::Expr(expr, None)] if stmt::break_after(expr) => {
                    self.ibox(0);
                    self.expr_beginning_of_line(expr, false, true, FixupContext::new_stmt());
                    self.end();
                    self.space();
                }
                _ => {
                    for stmt in block.stmts.iter().delimited() {
                        self.stmt(&stmt, stmt.is_last);
                    }
                }
            }
            self.offset(-INDENT);
        }
        self.word("}");
    }

    pub fn expr_as_small_block(&mut self, expr: &Expr, indent: isize) {
        self.word("{");
        self.space();
        self.ibox(indent);
        self.expr_beginning_of_line(expr, false, true, FixupContext::new_stmt());
        self.end();
        self.space();
        self.offset(-INDENT);
        self.word("}");
    }

    pub fn member(&mut self, member: &Member) {
        match member {
            Member::Named(ident) => self.ident(ident),
            Member::Unnamed(index) => self.index(index),
        }
    }

    fn index(&mut self, member: &Index) {
        self.word(member.index.to_string());
    }

    fn binary_operator(&mut self, op: &BinOp) {
        self.word(
            match op {
                #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
                BinOp::Add(_) => "+",
                BinOp::Sub(_) => "-",
                BinOp::Mul(_) => "*",
                BinOp::Div(_) => "/",
                BinOp::Rem(_) => "%",
                BinOp::And(_) => "&&",
                BinOp::Or(_) => "||",
                BinOp::BitXor(_) => "^",
                BinOp::BitAnd(_) => "&",
                BinOp::BitOr(_) => "|",
                BinOp::Shl(_) => "<<",
                BinOp::Shr(_) => ">>",
                BinOp::Eq(_) => "==",
                BinOp::Lt(_) => "<",
                BinOp::Le(_) => "<=",
                BinOp::Ne(_) => "!=",
                BinOp::Ge(_) => ">=",
                BinOp::Gt(_) => ">",
                BinOp::AddAssign(_) => "+=",
                BinOp::SubAssign(_) => "-=",
                BinOp::MulAssign(_) => "*=",
                BinOp::DivAssign(_) => "/=",
                BinOp::RemAssign(_) => "%=",
                BinOp::BitXorAssign(_) => "^=",
                BinOp::BitAndAssign(_) => "&=",
                BinOp::BitOrAssign(_) => "|=",
                BinOp::ShlAssign(_) => "<<=",
                BinOp::ShrAssign(_) => ">>=",
                _ => unimplemented!("unknown BinOp"),
            },
        );
    }

    fn unary_operator(&mut self, op: &UnOp) {
        self.word(
            match op {
                #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
                UnOp::Deref(_) => "*",
                UnOp::Not(_) => "!",
                UnOp::Neg(_) => "-",
                _ => unimplemented!("unknown UnOp"),
            },
        );
    }

    fn pointer_mutability(&mut self, mutability: &PointerMutability) {
        match mutability {
            PointerMutability::Const(_) => self.word("const"),
            PointerMutability::Mut(_) => self.word("mut"),
        }
    }
}

fn needs_newline_if_wrap(expr: &Expr) -> bool {
    match expr {
        #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
        Expr::Array(_)
        | Expr::Async(_)
        | Expr::Block(_)
        | Expr::Break(ExprBreak { expr: None, .. })
        | Expr::Closure(_)
        | Expr::Const(_)
        | Expr::Continue(_)
        | Expr::ForLoop(_)
        | Expr::If(_)
        | Expr::Infer(_)
        | Expr::Lit(_)
        | Expr::Loop(_)
        | Expr::Macro(_)
        | Expr::Match(_)
        | Expr::Path(_)
        | Expr::Range(ExprRange { end: None, .. })
        | Expr::Repeat(_)
        | Expr::Return(ExprReturn { expr: None, .. })
        | Expr::Struct(_)
        | Expr::TryBlock(_)
        | Expr::Tuple(_)
        | Expr::Unsafe(_)
        | Expr::Verbatim(_)
        | Expr::While(_)
        | Expr::Yield(ExprYield { expr: None, .. }) => false,

        Expr::Assign(_)
        | Expr::Await(_)
        | Expr::Binary(_)
        | Expr::Cast(_)
        | Expr::Field(_)
        | Expr::Index(_)
        | Expr::MethodCall(_) => true,

        Expr::Break(ExprBreak { expr: Some(e), .. })
        | Expr::Call(ExprCall { func: e, .. })
        | Expr::Group(ExprGroup { expr: e, .. })
        | Expr::Let(ExprLet { expr: e, .. })
        | Expr::Paren(ExprParen { expr: e, .. })
        | Expr::Range(ExprRange { end: Some(e), .. })
        | Expr::RawAddr(ExprRawAddr { expr: e, .. })
        | Expr::Reference(ExprReference { expr: e, .. })
        | Expr::Return(ExprReturn { expr: Some(e), .. })
        | Expr::Try(ExprTry { expr: e, .. })
        | Expr::Unary(ExprUnary { expr: e, .. })
        | Expr::Yield(ExprYield { expr: Some(e), .. }) => needs_newline_if_wrap(e),

        _ => false,
    }
}

fn is_short_ident(expr: &Expr) -> bool {
    if let Expr::Path(expr) = expr {
        return expr.attrs.is_empty()
            && expr.qself.is_none()
            && expr
                .path
                .get_ident()
                .map_or(false, |ident| ident.to_string().len() as isize <= INDENT);
    }
    false
}

fn is_blocklike(expr: &Expr) -> bool {
    match expr {
        #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
        Expr::Array(ExprArray { attrs, .. })
        | Expr::Async(ExprAsync { attrs, .. })
        | Expr::Block(ExprBlock { attrs, .. })
        | Expr::Closure(ExprClosure { attrs, .. })
        | Expr::Const(ExprConst { attrs, .. })
        | Expr::Struct(ExprStruct { attrs, .. })
        | Expr::TryBlock(ExprTryBlock { attrs, .. })
        | Expr::Tuple(ExprTuple { attrs, .. })
        | Expr::Unsafe(ExprUnsafe { attrs, .. }) => !attr::has_outer(attrs),

        Expr::Assign(_)
        | Expr::Await(_)
        | Expr::Binary(_)
        | Expr::Break(_)
        | Expr::Call(_)
        | Expr::Cast(_)
        | Expr::Continue(_)
        | Expr::Field(_)
        | Expr::ForLoop(_)
        | Expr::If(_)
        | Expr::Index(_)
        | Expr::Infer(_)
        | Expr::Let(_)
        | Expr::Lit(_)
        | Expr::Loop(_)
        | Expr::Macro(_)
        | Expr::Match(_)
        | Expr::MethodCall(_)
        | Expr::Paren(_)
        | Expr::Path(_)
        | Expr::Range(_)
        | Expr::RawAddr(_)
        | Expr::Reference(_)
        | Expr::Repeat(_)
        | Expr::Return(_)
        | Expr::Try(_)
        | Expr::Unary(_)
        | Expr::Verbatim(_)
        | Expr::While(_)
        | Expr::Yield(_) => false,

        Expr::Group(e) => is_blocklike(&e.expr),

        _ => false,
    }
}

pub fn simple_block(expr: &Expr) -> Option<&ExprBlock> {
    if let Expr::Block(expr) = expr {
        if expr.attrs.is_empty() && expr.label.is_none() {
            return Some(expr);
        }
    }
    None
}

pub fn simple_array(elements: &Punctuated<Expr, Token![,]>) -> bool {
    for expr in elements {
        if let Expr::Lit(expr) = expr {
            match expr.lit {
                #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
                Lit::Byte(_) | Lit::Char(_) | Lit::Int(_) | Lit::Bool(_) => {}

                Lit::Str(_) | Lit::ByteStr(_) | Lit::CStr(_) | Lit::Float(_) | Lit::Verbatim(_) => {
                    return false;
                }

                _ => return false,
            }
        } else {
            return false;
        }
    }
    true
}

// Expressions for which `$expr` and `{ $expr }` mean the same thing.
//
// This is not the case for all expressions. For example `{} | x | x` has some
// bitwise OR operators while `{ {} |x| x }` has a block followed by a closure.
fn parseable_as_stmt(mut expr: &Expr) -> bool {
    loop {
        match expr {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            Expr::Array(_)
            | Expr::Async(_)
            | Expr::Block(_)
            | Expr::Break(_)
            | Expr::Closure(_)
            | Expr::Const(_)
            | Expr::Continue(_)
            | Expr::ForLoop(_)
            | Expr::If(_)
            | Expr::Infer(_)
            | Expr::Lit(_)
            | Expr::Loop(_)
            | Expr::Macro(_)
            | Expr::Match(_)
            | Expr::Paren(_)
            | Expr::Path(_)
            | Expr::RawAddr(_)
            | Expr::Reference(_)
            | Expr::Repeat(_)
            | Expr::Return(_)
            | Expr::Struct(_)
            | Expr::TryBlock(_)
            | Expr::Tuple(_)
            | Expr::Unary(_)
            | Expr::Unsafe(_)
            | Expr::Verbatim(_)
            | Expr::While(_)
            | Expr::Yield(_) => return true,

            Expr::Let(_) => return false,

            Expr::Assign(e) => {
                if !classify::requires_semi_to_be_stmt(&e.left) {
                    return false;
                }
                expr = &e.left;
            }
            Expr::Await(e) => expr = &e.base,
            Expr::Binary(e) => {
                if !classify::requires_semi_to_be_stmt(&e.left) {
                    return false;
                }
                expr = &e.left;
            }
            Expr::Call(e) => {
                if !classify::requires_semi_to_be_stmt(&e.func) {
                    return false;
                }
                expr = &e.func;
            }
            Expr::Cast(e) => {
                if !classify::requires_semi_to_be_stmt(&e.expr) {
                    return false;
                }
                expr = &e.expr;
            }
            Expr::Field(e) => expr = &e.base,
            Expr::Group(e) => expr = &e.expr,
            Expr::Index(e) => {
                if !classify::requires_semi_to_be_stmt(&e.expr) {
                    return false;
                }
                expr = &e.expr;
            }
            Expr::MethodCall(e) => expr = &e.receiver,
            Expr::Range(e) => match &e.start {
                None => return true,
                Some(start) => {
                    if !classify::requires_semi_to_be_stmt(start) {
                        return false;
                    }
                    expr = start;
                }
            },
            Expr::Try(e) => expr = &e.expr,

            _ => return false,
        }
    }
}
