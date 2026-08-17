//! Parser for ISLE language.

use crate::ast::*;
use crate::error::{Error, Span};
use crate::lexer::{Lexer, Pos, Token};

type Result<T> = std::result::Result<T, Error>;

/// Parse the top-level ISLE definitions and return their AST.
pub fn parse(lexer: Lexer) -> Result<Vec<Def>> {
    let parser = Parser::new(lexer);
    parser.parse_defs()
}

/// Parse without positional information. Provided mainly to support testing, to
/// enable equality testing on structure alone.
pub fn parse_without_pos(lexer: Lexer) -> Result<Vec<Def>> {
    let parser = Parser::new_without_pos_tracking(lexer);
    parser.parse_defs()
}

/// The ISLE parser.
///
/// Takes in a lexer and creates an AST.
#[derive(Clone, Debug)]
struct Parser<'a> {
    lexer: Lexer<'a>,
    disable_pos: bool,
}

/// Used during parsing a `(rule ...)` to encapsulate some form that
/// comes after the top-level pattern: an if-let clause, or the final
/// top-level expr.
enum IfLetOrExpr {
    IfLet(IfLet),
    Expr(Expr),
}

impl<'a> Parser<'a> {
    /// Construct a new parser from the given lexer.
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            disable_pos: false,
        }
    }

    fn new_without_pos_tracking(lexer: Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            disable_pos: true,
        }
    }

    fn error(&self, pos: Pos, msg: String) -> Error {
        Error::ParseError {
            msg,
            span: Span::new_single(pos),
        }
    }

    fn expect<F: Fn(&Token) -> bool>(&mut self, f: F) -> Result<Token> {
        if let Some(&(pos, ref peek)) = self.lexer.peek() {
            if !f(peek) {
                return Err(self.error(pos, format!("Unexpected token {peek:?}")));
            }
            Ok(self.lexer.next()?.unwrap().1)
        } else {
            Err(self.error(self.lexer.pos(), "Unexpected EOF".to_string()))
        }
    }

    fn eat<F: Fn(&Token) -> bool>(&mut self, f: F) -> Result<Option<Token>> {
        if let Some(&(_pos, ref peek)) = self.lexer.peek() {
            if !f(peek) {
                return Ok(None);
            }
            Ok(Some(self.lexer.next()?.unwrap().1))
        } else {
            Ok(None) // EOF
        }
    }

    fn is<F: Fn(&Token) -> bool>(&self, f: F) -> bool {
        if let Some((_, peek)) = self.lexer.peek() {
            f(peek)
        } else {
            false
        }
    }

    fn pos(&self) -> Pos {
        if !self.disable_pos {
            self.lexer
                .peek()
                .map_or_else(|| self.lexer.pos(), |(pos, _)| *pos)
        } else {
            Pos::default()
        }
    }

    fn is_lparen(&self) -> bool {
        self.is(|tok| *tok == Token::LParen)
    }
    fn is_rparen(&self) -> bool {
        self.is(|tok| *tok == Token::RParen)
    }
    fn is_at(&self) -> bool {
        self.is(|tok| *tok == Token::At)
    }
    fn is_sym(&self) -> bool {
        self.is(Token::is_sym)
    }
    fn is_int(&self) -> bool {
        self.is(Token::is_int)
    }

    fn is_const(&self) -> bool {
        self.is(|tok| match tok {
            Token::Symbol(tok_s) if tok_s.starts_with('$') => true,
            _ => false,
        })
    }

    fn is_spec_bit_vector(&self) -> bool {
        self.is(|tok| match tok {
            Token::Symbol(tok_s) if tok_s.starts_with("#x") || tok_s.starts_with("#b") => true,
            _ => false,
        })
    }

    fn is_spec_bool(&self) -> bool {
        self.is(|tok| match tok {
            Token::Symbol(tok_s) if tok_s == "true" || tok_s == "false" => true,
            _ => false,
        })
    }

    fn expect_lparen(&mut self) -> Result<()> {
        self.expect(|tok| *tok == Token::LParen).map(|_| ())
    }
    fn expect_rparen(&mut self) -> Result<()> {
        self.expect(|tok| *tok == Token::RParen).map(|_| ())
    }
    fn expect_at(&mut self) -> Result<()> {
        self.expect(|tok| *tok == Token::At).map(|_| ())
    }

    fn expect_symbol(&mut self) -> Result<String> {
        match self.expect(Token::is_sym)? {
            Token::Symbol(s) => Ok(s),
            _ => unreachable!(),
        }
    }

    fn eat_sym_str(&mut self, s: &str) -> Result<bool> {
        self.eat(|tok| match tok {
            Token::Symbol(tok_s) if tok_s == s => true,
            _ => false,
        })
        .map(|token| token.is_some())
    }

    fn expect_int(&mut self) -> Result<i128> {
        match self.expect(Token::is_int)? {
            Token::Int(i) => Ok(i),
            _ => unreachable!(),
        }
    }

    fn parse_defs(mut self) -> Result<Vec<Def>> {
        let mut defs = vec![];
        while !self.lexer.eof() {
            defs.push(self.parse_def()?);
        }
        Ok(defs)
    }

    fn parse_def(&mut self) -> Result<Def> {
        self.expect_lparen()?;
        let pos = self.pos();
        let def = match &self.expect_symbol()?[..] {
            "pragma" => Def::Pragma(self.parse_pragma()?),
            "type" => Def::Type(self.parse_type()?),
            "decl" => Def::Decl(self.parse_decl()?),
            "spec" => Def::Spec(self.parse_spec()?),
            "model" => Def::Model(self.parse_model()?),
            "form" => Def::Form(self.parse_form()?),
            "instantiate" => Def::Instantiation(self.parse_instantiation()?),
            "rule" => Def::Rule(self.parse_rule()?),
            "extractor" => Def::Extractor(self.parse_etor()?),
            "extern" => Def::Extern(self.parse_extern()?),
            "convert" => Def::Converter(self.parse_converter()?),
            s => {
                return Err(self.error(pos, format!("Unexpected identifier: {s}")));
            }
        };
        self.expect_rparen()?;
        Ok(def)
    }

    fn str_to_ident(&self, pos: Pos, s: &str) -> Result<Ident> {
        let first = s
            .chars()
            .next()
            .ok_or_else(|| self.error(pos, "empty symbol".into()))?;
        if !first.is_alphabetic() && first != '_' && first != '$' {
            return Err(self.error(
                pos,
                format!("Identifier '{s}' does not start with letter or _ or $"),
            ));
        }
        if s.chars()
            .skip(1)
            .any(|c| !c.is_alphanumeric() && c != '_' && c != '.' && c != '$')
        {
            return Err(self.error(
                pos,
                format!("Identifier '{s}' contains invalid character (not a-z, A-Z, 0-9, _, ., $)"),
            ));
        }
        Ok(Ident(s.to_string(), pos))
    }

    fn parse_ident(&mut self) -> Result<Ident> {
        let pos = self.pos();
        let s = self.expect_symbol()?;
        self.str_to_ident(pos, &s)
    }

    fn parse_const(&mut self) -> Result<Ident> {
        let pos = self.pos();
        let ident = self.parse_ident()?;
        if let Some(s) = ident.0.strip_prefix('$') {
            Ok(Ident(s.to_string(), ident.1))
        } else {
            Err(self.error(
                pos,
                "Not a constant identifier; must start with a '$'".to_string(),
            ))
        }
    }

    fn parse_pragma(&mut self) -> Result<Pragma> {
        let ident = self.parse_ident()?;
        // currently, no pragmas are defined, but the infrastructure is useful to keep around
        let pragma = ident.0.as_str();
        Err(self.error(ident.1, format!("Unknown pragma '{pragma}'")))
    }

    fn parse_type(&mut self) -> Result<Type> {
        let pos = self.pos();
        let name = self.parse_ident()?;

        let mut is_extern = false;
        let mut is_nodebug = false;

        while self.lexer.peek().map_or(false, |(_pos, tok)| tok.is_sym()) {
            let sym = self.expect_symbol()?;
            if sym == "extern" {
                is_extern = true;
            } else if sym == "nodebug" {
                is_nodebug = true;
            } else {
                return Err(self.error(
                    self.pos(),
                    format!("unknown type declaration modifier: {sym}"),
                ));
            }
        }

        let ty = self.parse_typevalue()?;
        Ok(Type {
            name,
            is_extern,
            is_nodebug,
            ty,
            pos,
        })
    }

    fn parse_typevalue(&mut self) -> Result<TypeValue> {
        let pos = self.pos();
        self.expect_lparen()?;
        if self.eat_sym_str("primitive")? {
            let primitive_ident = self.parse_ident()?;
            self.expect_rparen()?;
            Ok(TypeValue::Primitive(primitive_ident, pos))
        } else if self.eat_sym_str("enum")? {
            let mut variants = vec![];
            while !self.is_rparen() {
                let variant = self.parse_type_variant()?;
                variants.push(variant);
            }
            self.expect_rparen()?;
            Ok(TypeValue::Enum(variants, pos))
        } else {
            Err(self.error(pos, "Unknown type definition".to_string()))
        }
    }

    fn parse_type_variant(&mut self) -> Result<Variant> {
        if self.is_sym() {
            let pos = self.pos();
            let name = self.parse_ident()?;
            Ok(Variant {
                name,
                fields: vec![],
                pos,
            })
        } else {
            let pos = self.pos();
            self.expect_lparen()?;
            let name = self.parse_ident()?;
            let mut fields = vec![];
            while !self.is_rparen() {
                fields.push(self.parse_type_field()?);
            }
            self.expect_rparen()?;
            Ok(Variant { name, fields, pos })
        }
    }

    fn parse_type_field(&mut self) -> Result<Field> {
        let pos = self.pos();
        self.expect_lparen()?;
        let name = self.parse_ident()?;
        let ty = self.parse_ident()?;
        self.expect_rparen()?;
        Ok(Field { name, ty, pos })
    }

    fn parse_decl(&mut self) -> Result<Decl> {
        let pos = self.pos();

        let pure = self.eat_sym_str("pure")?;
        let multi = self.eat_sym_str("multi")?;
        let partial = self.eat_sym_str("partial")?;

        let term = self.parse_ident()?;

        self.expect_lparen()?;
        let mut arg_tys = vec![];
        while !self.is_rparen() {
            arg_tys.push(self.parse_ident()?);
        }
        self.expect_rparen()?;

        let ret_ty = self.parse_ident()?;

        Ok(Decl {
            term,
            arg_tys,
            ret_ty,
            pure,
            multi,
            partial,
            pos,
        })
    }

    fn parse_spec(&mut self) -> Result<Spec> {
        let pos = self.pos();
        self.expect_lparen()?; // term with args: (spec (<term> <args>) (provide ...) ...)
        let term = self.parse_ident()?;
        let mut args = vec![];
        while !self.is_rparen() {
            args.push(self.parse_ident()?);
        }
        self.expect_rparen()?; // end term with args

        self.expect_lparen()?; // provide
        if !self.eat_sym_str("provide")? {
            return Err(self.error(
                pos,
                "Invalid spec: expected (spec (<term> <args>) (provide ...) ...)".to_string(),
            ));
        };
        let mut provides = vec![];
        while !self.is_rparen() {
            provides.push(self.parse_spec_expr()?);
        }
        self.expect_rparen()?; // end provide

        let requires = if self.is_lparen() {
            self.expect_lparen()?;
            if !self.eat_sym_str("require")? {
                return Err(self.error(
                    pos,
                    "Invalid spec: expected (spec (<term> <args>) (provide ...) (require ...))"
                        .to_string(),
                ));
            }
            let mut require = vec![];
            while !self.is_rparen() {
                require.push(self.parse_spec_expr()?);
            }
            self.expect_rparen()?; // end provide
            require
        } else {
            vec![]
        };

        Ok(Spec {
            term,
            args,
            provides,
            requires,
        })
    }

    fn parse_spec_expr(&mut self) -> Result<SpecExpr> {
        let pos = self.pos();
        if self.is_spec_bit_vector() {
            let (val, width) = self.parse_spec_bit_vector()?;
            return Ok(SpecExpr::ConstBitVec { val, width, pos });
        } else if self.is_int() {
            return Ok(SpecExpr::ConstInt {
                val: self.expect_int()?,
                pos,
            });
        } else if self.is_spec_bool() {
            let val = self.parse_spec_bool()?;
            return Ok(SpecExpr::ConstBool { val, pos });
        } else if self.is_sym() {
            let var = self.parse_ident()?;
            return Ok(SpecExpr::Var { var, pos });
        } else if self.is_lparen() {
            self.expect_lparen()?;
            if self.eat_sym_str("switch")? {
                let mut args = vec![];
                args.push(self.parse_spec_expr()?);
                while !(self.is_rparen()) {
                    self.expect_lparen()?;
                    let l = Box::new(self.parse_spec_expr()?);
                    let r = Box::new(self.parse_spec_expr()?);
                    self.expect_rparen()?;
                    args.push(SpecExpr::Pair { l, r });
                }
                self.expect_rparen()?;
                return Ok(SpecExpr::Op {
                    op: SpecOp::Switch,
                    args,
                    pos,
                });
            }
            if self.is_sym() && !self.is_spec_bit_vector() {
                let sym = self.expect_symbol()?;
                if let Ok(op) = self.parse_spec_op(sym.as_str()) {
                    let mut args: Vec<SpecExpr> = vec![];
                    while !self.is_rparen() {
                        args.push(self.parse_spec_expr()?);
                    }
                    self.expect_rparen()?;
                    return Ok(SpecExpr::Op { op, args, pos });
                };
                let ident = self.str_to_ident(pos, &sym)?;
                if self.is_rparen() {
                    self.expect_rparen()?;
                    return Ok(SpecExpr::Enum { name: ident });
                };
            }
            // Unit
            if self.is_rparen() {
                self.expect_rparen()?;
                return Ok(SpecExpr::ConstUnit { pos });
            }
        }
        Err(self.error(pos, "Unexpected spec expression".into()))
    }

    fn parse_spec_op(&mut self, s: &str) -> Result<SpecOp> {
        let pos = self.pos();
        match s {
            "=" => Ok(SpecOp::Eq),
            "and" => Ok(SpecOp::And),
            "not" => Ok(SpecOp::Not),
            "=>" => Ok(SpecOp::Imp),
            "or" => Ok(SpecOp::Or),
            "<=" => Ok(SpecOp::Lte),
            "<" => Ok(SpecOp::Lt),
            ">=" => Ok(SpecOp::Gte),
            ">" => Ok(SpecOp::Gt),
            "bvnot" => Ok(SpecOp::BVNot),
            "bvand" => Ok(SpecOp::BVAnd),
            "bvor" => Ok(SpecOp::BVOr),
            "bvxor" => Ok(SpecOp::BVXor),
            "bvneg" => Ok(SpecOp::BVNeg),
            "bvadd" => Ok(SpecOp::BVAdd),
            "bvsub" => Ok(SpecOp::BVSub),
            "bvmul" => Ok(SpecOp::BVMul),
            "bvudiv" => Ok(SpecOp::BVUdiv),
            "bvurem" => Ok(SpecOp::BVUrem),
            "bvsdiv" => Ok(SpecOp::BVSdiv),
            "bvsrem" => Ok(SpecOp::BVSrem),
            "bvshl" => Ok(SpecOp::BVShl),
            "bvlshr" => Ok(SpecOp::BVLshr),
            "bvashr" => Ok(SpecOp::BVAshr),
            "bvsaddo" => Ok(SpecOp::BVSaddo),
            "bvule" => Ok(SpecOp::BVUle),
            "bvult" => Ok(SpecOp::BVUlt),
            "bvugt" => Ok(SpecOp::BVUgt),
            "bvuge" => Ok(SpecOp::BVUge),
            "bvslt" => Ok(SpecOp::BVSlt),
            "bvsle" => Ok(SpecOp::BVSle),
            "bvsgt" => Ok(SpecOp::BVSgt),
            "bvsge" => Ok(SpecOp::BVSge),
            "rotr" => Ok(SpecOp::Rotr),
            "rotl" => Ok(SpecOp::Rotl),
            "extract" => Ok(SpecOp::Extract),
            "zero_ext" => Ok(SpecOp::ZeroExt),
            "sign_ext" => Ok(SpecOp::SignExt),
            "concat" => Ok(SpecOp::Concat),
            "conv_to" => Ok(SpecOp::ConvTo),
            "int2bv" => Ok(SpecOp::Int2BV),
            "bv2int" => Ok(SpecOp::BV2Int),
            "widthof" => Ok(SpecOp::WidthOf),
            "if" => Ok(SpecOp::If),
            "switch" => Ok(SpecOp::Switch),
            "subs" => Ok(SpecOp::Subs),
            "popcnt" => Ok(SpecOp::Popcnt),
            "rev" => Ok(SpecOp::Rev),
            "cls" => Ok(SpecOp::Cls),
            "clz" => Ok(SpecOp::Clz),
            "load_effect" => Ok(SpecOp::LoadEffect),
            "store_effect" => Ok(SpecOp::StoreEffect),
            x => Err(self.error(pos, format!("Not a valid spec operator: {x}"))),
        }
    }

    fn parse_spec_bit_vector(&mut self) -> Result<(i128, i8)> {
        let pos = self.pos();
        let s = self.expect_symbol()?;
        if let Some(s) = s.strip_prefix("#b") {
            match i128::from_str_radix(s, 2) {
                Ok(i) => Ok((i, s.len() as i8)),
                Err(_) => Err(self.error(pos, "Not a constant binary bit vector".to_string())),
            }
        } else if let Some(s) = s.strip_prefix("#x") {
            match i128::from_str_radix(s, 16) {
                Ok(i) => Ok((i, (s.len() as i8) * 4)),
                Err(_) => Err(self.error(pos, "Not a constant hex bit vector".to_string())),
            }
        } else {
            Err(self.error(
                pos,
                "Not a constant bit vector; must start with `#x` (hex) or `#b` (binary)"
                    .to_string(),
            ))
        }
    }

    fn parse_spec_bool(&mut self) -> Result<bool> {
        let pos = self.pos();
        let s = self.expect_symbol()?;
        match s.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            x => Err(self.error(pos, format!("Not a valid spec boolean: {x}"))),
        }
    }

    fn parse_model(&mut self) -> Result<Model> {
        let pos = self.pos();
        let name = self.parse_ident()?;
        self.expect_lparen()?; // body
        let val = if self.eat_sym_str("type")? {
            let ty = self.parse_model_type();
            ModelValue::TypeValue(ty?)
        } else if self.eat_sym_str("enum")? {
            let mut variants = vec![];
            let mut has_explicit_value = false;
            let mut implicit_idx = None;

            while !self.is_rparen() {
                self.expect_lparen()?; // enum value
                let name = self.parse_ident()?;
                let val = if self.is_rparen() {
                    // has implicit enum value
                    if has_explicit_value {
                        return Err(self.error(
                            pos,
                            format!(
                                "Spec enum has unexpected implicit value after implicit value."
                            ),
                        ));
                    }
                    implicit_idx = Some(if let Some(idx) = implicit_idx {
                        idx + 1
                    } else {
                        0
                    });
                    SpecExpr::ConstInt {
                        val: implicit_idx.unwrap(),
                        pos,
                    }
                } else {
                    if implicit_idx.is_some() {
                        return Err(self.error(
                            pos,
                            format!(
                                "Spec enum has unexpected explicit value after implicit value."
                            ),
                        ));
                    }
                    has_explicit_value = true;
                    self.parse_spec_expr()?
                };
                self.expect_rparen()?;
                variants.push((name, val));
            }
            ModelValue::EnumValues(variants)
        } else {
            return Err(self.error(pos, "Model must be a type or enum".to_string()));
        };

        self.expect_rparen()?; // end body
        Ok(Model { name, val })
    }

    fn parse_model_type(&mut self) -> Result<ModelType> {
        let pos = self.pos();
        if self.eat_sym_str("Bool")? {
            Ok(ModelType::Bool)
        } else if self.eat_sym_str("Int")? {
            Ok(ModelType::Int)
        } else if self.eat_sym_str("Unit")? {
            Ok(ModelType::Unit)
        } else if self.is_lparen() {
            self.expect_lparen()?;
            let width = if self.eat_sym_str("bv")? {
                if self.is_rparen() {
                    None
                } else if self.is_int() {
                    Some(usize::try_from(self.expect_int()?).map_err(|err| {
                        self.error(pos, format!("Invalid BitVector width: {err}"))
                    })?)
                } else {
                    return Err(self.error(pos, "Badly formed BitVector (bv ...)".to_string()));
                }
            } else {
                return Err(self.error(pos, "Badly formed BitVector (bv ...)".to_string()));
            };
            self.expect_rparen()?;
            Ok(ModelType::BitVec(width))
        } else {
            Err(self.error(
                pos,
                "Model type be a Bool, Int, or BitVector (bv ...)".to_string(),
            ))
        }
    }

    fn parse_form(&mut self) -> Result<Form> {
        let pos = self.pos();
        let name = self.parse_ident()?;
        let signatures = self.parse_signatures()?;
        Ok(Form {
            name,
            signatures,
            pos,
        })
    }

    fn parse_signatures(&mut self) -> Result<Vec<Signature>> {
        let mut signatures = vec![];
        while !self.is_rparen() {
            signatures.push(self.parse_signature()?);
        }
        Ok(signatures)
    }

    fn parse_signature(&mut self) -> Result<Signature> {
        self.expect_lparen()?;
        let pos = self.pos();
        let args = self.parse_tagged_types("args")?;
        let ret = self.parse_tagged_type("ret")?;
        let canonical = self.parse_tagged_type("canon")?;
        self.expect_rparen()?;
        Ok(Signature {
            args,
            ret,
            canonical,
            pos,
        })
    }

    fn parse_tagged_types(&mut self, tag: &str) -> Result<Vec<ModelType>> {
        self.expect_lparen()?;
        let pos = self.pos();
        if !self.eat_sym_str(tag)? {
            return Err(self.error(pos, format!("Invalid {tag}: expected ({tag} <arg> ...)")));
        };
        let mut params = vec![];
        while !self.is_rparen() {
            params.push(self.parse_model_type()?);
        }
        self.expect_rparen()?;
        Ok(params)
    }

    fn parse_tagged_type(&mut self, tag: &str) -> Result<ModelType> {
        self.expect_lparen()?;
        let pos = self.pos();
        if !self.eat_sym_str(tag)? {
            return Err(self.error(pos, format!("Invalid {tag}: expected ({tag} <arg>)")));
        };
        let ty = self.parse_model_type()?;
        self.expect_rparen()?;
        Ok(ty)
    }

    fn parse_instantiation(&mut self) -> Result<Instantiation> {
        let pos = self.pos();
        let term = self.parse_ident()?;
        // Instantiation either has an explicit signatures list, which would
        // open with a left paren. Or it has an identifier referencing a
        // predefined set of signatures.
        if self.is_lparen() {
            let signatures = self.parse_signatures()?;
            Ok(Instantiation {
                term,
                form: None,
                signatures,
                pos,
            })
        } else {
            let form = self.parse_ident()?;
            Ok(Instantiation {
                term,
                form: Some(form),
                signatures: vec![],
                pos,
            })
        }
    }

    fn parse_extern(&mut self) -> Result<Extern> {
        let pos = self.pos();
        if self.eat_sym_str("constructor")? {
            let term = self.parse_ident()?;
            let func = self.parse_ident()?;
            Ok(Extern::Constructor { term, func, pos })
        } else if self.eat_sym_str("extractor")? {
            let infallible = self.eat_sym_str("infallible")?;

            let term = self.parse_ident()?;
            let func = self.parse_ident()?;

            Ok(Extern::Extractor {
                term,
                func,
                pos,
                infallible,
            })
        } else if self.eat_sym_str("const")? {
            let pos = self.pos();
            let name = self.parse_const()?;
            let ty = self.parse_ident()?;
            Ok(Extern::Const { name, ty, pos })
        } else {
            Err(self.error(
                pos,
                "Invalid extern: must be (extern constructor ...), (extern extractor ...) or (extern const ...)"
                    .to_string(),
            ))
        }
    }

    fn parse_etor(&mut self) -> Result<Extractor> {
        let pos = self.pos();
        self.expect_lparen()?;
        let term = self.parse_ident()?;
        let mut args = vec![];
        while !self.is_rparen() {
            args.push(self.parse_ident()?);
        }
        self.expect_rparen()?;
        let template = self.parse_pattern()?;
        Ok(Extractor {
            term,
            args,
            template,
            pos,
        })
    }

    fn parse_rule(&mut self) -> Result<Rule> {
        let pos = self.pos();
        let name = if self.is_sym() {
            Some(
                self.parse_ident()
                    .map_err(|err| self.error(pos, format!("Invalid rule name: {err:?}")))?,
            )
        } else {
            None
        };
        let prio = if self.is_int() {
            Some(
                i64::try_from(self.expect_int()?)
                    .map_err(|err| self.error(pos, format!("Invalid rule priority: {err}")))?,
            )
        } else {
            None
        };
        let pattern = self.parse_pattern()?;
        let mut iflets = vec![];
        loop {
            match self.parse_iflet_or_expr()? {
                IfLetOrExpr::IfLet(iflet) => {
                    iflets.push(iflet);
                }
                IfLetOrExpr::Expr(expr) => {
                    return Ok(Rule {
                        pattern,
                        iflets,
                        expr,
                        pos,
                        prio,
                        name,
                    });
                }
            }
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern> {
        let pos = self.pos();
        if self.is_int() {
            Ok(Pattern::ConstInt {
                val: self.expect_int()?,
                pos,
            })
        } else if self.is_const() {
            let val = self.parse_const()?;
            Ok(Pattern::ConstPrim { val, pos })
        } else if self.eat_sym_str("_")? {
            Ok(Pattern::Wildcard { pos })
        } else if self.eat_sym_str("true")? {
            Ok(Pattern::ConstBool { val: true, pos })
        } else if self.eat_sym_str("false")? {
            Ok(Pattern::ConstBool { val: false, pos })
        } else if self.is_sym() {
            let var = self.parse_ident()?;
            if self.is_at() {
                self.expect_at()?;
                let subpat = Box::new(self.parse_pattern()?);
                Ok(Pattern::BindPattern { var, subpat, pos })
            } else {
                Ok(Pattern::Var { var, pos })
            }
        } else if self.is_lparen() {
            self.expect_lparen()?;
            if self.eat_sym_str("and")? {
                let mut subpats = vec![];
                while !self.is_rparen() {
                    subpats.push(self.parse_pattern()?);
                }
                self.expect_rparen()?;
                Ok(Pattern::And { subpats, pos })
            } else {
                let sym = self.parse_ident()?;
                let mut args = vec![];
                while !self.is_rparen() {
                    args.push(self.parse_pattern()?);
                }
                self.expect_rparen()?;
                Ok(Pattern::Term { sym, args, pos })
            }
        } else {
            Err(self.error(pos, "Unexpected pattern".into()))
        }
    }

    fn parse_iflet_or_expr(&mut self) -> Result<IfLetOrExpr> {
        let pos = self.pos();
        if self.is_lparen() {
            self.expect_lparen()?;
            let ret = if self.eat_sym_str("if-let")? {
                IfLetOrExpr::IfLet(self.parse_iflet()?)
            } else if self.eat_sym_str("if")? {
                // Shorthand form: `(if (x))` desugars to `(if-let _
                // (x))`.
                IfLetOrExpr::IfLet(self.parse_iflet_if()?)
            } else {
                IfLetOrExpr::Expr(self.parse_expr_inner_parens(pos)?)
            };
            self.expect_rparen()?;
            Ok(ret)
        } else {
            self.parse_expr().map(IfLetOrExpr::Expr)
        }
    }

    fn parse_iflet(&mut self) -> Result<IfLet> {
        let pos = self.pos();
        let pattern = self.parse_pattern()?;
        let expr = self.parse_expr()?;
        Ok(IfLet { pattern, expr, pos })
    }

    fn parse_iflet_if(&mut self) -> Result<IfLet> {
        let pos = self.pos();
        let expr = self.parse_expr()?;
        Ok(IfLet {
            pattern: Pattern::Wildcard { pos },
            expr,
            pos,
        })
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let pos = self.pos();
        if self.is_lparen() {
            self.expect_lparen()?;
            let ret = self.parse_expr_inner_parens(pos)?;
            self.expect_rparen()?;
            Ok(ret)
        } else if self.is_const() {
            let val = self.parse_const()?;
            Ok(Expr::ConstPrim { val, pos })
        } else if self.eat_sym_str("true")? {
            Ok(Expr::ConstBool { val: true, pos })
        } else if self.eat_sym_str("false")? {
            Ok(Expr::ConstBool { val: false, pos })
        } else if self.is_sym() {
            let name = self.parse_ident()?;
            Ok(Expr::Var { name, pos })
        } else if self.is_int() {
            let val = self.expect_int()?;
            Ok(Expr::ConstInt { val, pos })
        } else {
            Err(self.error(pos, "Invalid expression".into()))
        }
    }

    fn parse_expr_inner_parens(&mut self, pos: Pos) -> Result<Expr> {
        if self.eat_sym_str("let")? {
            self.expect_lparen()?;
            let mut defs = vec![];
            while !self.is_rparen() {
                let def = self.parse_letdef()?;
                defs.push(def);
            }
            self.expect_rparen()?;
            let body = Box::new(self.parse_expr()?);
            Ok(Expr::Let { defs, body, pos })
        } else {
            let sym = self.parse_ident()?;
            let mut args = vec![];
            while !self.is_rparen() {
                args.push(self.parse_expr()?);
            }
            Ok(Expr::Term { sym, args, pos })
        }
    }

    fn parse_letdef(&mut self) -> Result<LetDef> {
        let pos = self.pos();
        self.expect_lparen()?;
        let var = self.parse_ident()?;
        let ty = self.parse_ident()?;
        let val = Box::new(self.parse_expr()?);
        self.expect_rparen()?;
        Ok(LetDef { var, ty, val, pos })
    }

    fn parse_converter(&mut self) -> Result<Converter> {
        let pos = self.pos();
        let inner_ty = self.parse_ident()?;
        let outer_ty = self.parse_ident()?;
        let term = self.parse_ident()?;
        Ok(Converter {
            term,
            inner_ty,
            outer_ty,
            pos,
        })
    }
}
