extern crate peg;
use peg::parser;

// C++ in Rust
trait Operation<'a>: std::fmt::Debug {}
trait Operand<'a>: std::fmt::Debug + AsDynOperand<'a> {}
trait Location<'a>: Operand<'a> {}
impl<'a, T: ?Sized + Location<'a>> Operand<'a> for T {}

// Thanks to quinedot for their comprehensive write-up on dyn Traits.
// https://quinedot.github.io/rust-learning/dyn-trait-combining.html#manual-supertrait-upcasting
trait AsDynOperand<'a> {
    fn as_dyn_operand(self: Box<Self>) -> Box<dyn Operand<'a> + 'a>;
}

impl<'a, T: /* Sized + */ Operand<'a> + 'a> AsDynOperand<'a> for T {
    fn as_dyn_operand(self: Box<Self>) -> Box<dyn Operand<'a> + 'a> {
        self
    }
}



#[derive(Debug)]
pub struct Program<'a>(Vec<Box<dyn Operation<'a> + 'a>>);

#[derive(Debug)]
struct Add<'a> {
    result: Box<dyn Location<'a> + 'a>,
    lhs: Box<dyn Operand<'a> + 'a>,
    rhs: Box<dyn Operand<'a> + 'a>,
}
impl<'a> Operation<'a> for Add<'a> {}

#[derive(Debug)]
struct Sub<'a> {
    result: Box<dyn Location<'a> + 'a>,
    lhs: Box<dyn Operand<'a> + 'a>,
    rhs: Box<dyn Operand<'a> + 'a>,
}
impl<'a> Operation<'a> for Sub<'a> {}

#[derive(Debug)]
struct Register<'a>(&'a str);
impl<'a> Location<'a> for Register<'a> {}

#[derive(Debug)]
struct Global<'a>(&'a str);
impl<'a> Location<'a> for Global<'a> {}

#[derive(Debug)]
struct Literal(i32);
impl<'a> Operand<'a> for Literal {}

parser!{
grammar assembly() for str {
    pub rule program() -> Program<'input>
        = op:operation() ** "\n" { Program(op) }

    rule _ = [' ']*

    rule operation() -> Box<dyn Operation<'input> + 'input>
        = a:add() {a} / s:sub() {s}

    rule add() -> Box<Add<'input>>
        = result:location() _ "=" _ "add" _ lhs:operand() _ rhs:operand() { Box::new(Add{ result, lhs, rhs }) }

    rule sub() -> Box<Sub<'input>>
        = result:location() _ "=" _ "sub" _ lhs:operand() _ rhs:operand() { Box::new(Sub{ result, lhs, rhs }) }

    rule location() -> Box<dyn Location<'input> + 'input>
        = r:register() {r} / g:global() {g}

    rule register() -> Box<Register<'input>>
        = "%" _ id:identifier() { Box::new(Register(id)) }

    rule global() -> Box<Global<'input>>
        = "@" _ id:identifier() { Box::new(Global(id)) }

    rule identifier() -> &'input str
        = $(['a'..='z' | 'A'..='Z']+)

    rule operand() -> Box<dyn Operand<'input> + 'input>
        = l:location() {l.as_dyn_operand()} / l:literal() {Box::new(l)}
    
    rule literal() -> Literal
        = n:$(['0'..='9']+) {?
            let n = n.parse::<i32>().map_err(|_| "invalid int literal")?;
            Ok(Literal(n))
        }

}}

fn main() {
    let parsed = assembly::program("%apple = add 1 @g
@b = add 2 %a
%c = sub 82 @b
@dog = sub @b 12").unwrap();
    let expected = Program(vec![
        Box::new(Add{
            result: Box::new(Register("apple")),
            lhs: Box::new(Literal(1)),
            rhs: Box::new(Global("g"))
        }),
        Box::new(Add{
            result: Box::new(Global("b")),
            lhs: Box::new(Literal(2)),
            rhs: Box::new(Register("a"))
        }),
        Box::new(Sub{
            result: Box::new(Register("c")),
            lhs: Box::new(Literal(82)),
            rhs: Box::new(Global("b"))
        }),
        Box::new(Sub{
            result: Box::new(Global("dog")),
            lhs: Box::new(Global("b")),
            rhs: Box::new(Literal(12))
        }),
    ]);
    assert_eq!(format!("{parsed:?}"), format!("{expected:?}"));
}
