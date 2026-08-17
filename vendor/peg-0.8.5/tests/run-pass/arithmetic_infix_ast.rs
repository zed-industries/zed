extern crate peg;

peg::parser!( grammar arithmetic() for str {
    rule ident() -> &'input str = $(['a'..='z']+)
    rule haskell_op() -> String = "`" i:ident() "`" [' '|'\n']* { i.to_owned() }
    rule plus() = "+" [' '|'\n']*

    pub rule expression() -> InfixAst = precedence!{
        x:(@) plus() y:@ { InfixAst::Add(Box::new(x), Box::new(y)) }
        --
        x:(@) op:haskell_op() y:@ { InfixAst::Op(op, Box::new(x), Box::new(y)) }
        --
        i:ident() [' '|'\n']* { InfixAst::Ident(i.to_owned()) }
    }
});

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InfixAst {
    Ident(String),
    Add(Box<InfixAst>, Box<InfixAst>),
    Op(String, Box<InfixAst>, Box<InfixAst>)
}

fn main(){
    assert_eq!(arithmetic::expression("a + b `x` c").unwrap(),
        InfixAst::Add(
            Box::new(InfixAst::Ident("a".to_owned())),
            Box::new(InfixAst::Op("x".to_owned(),
                Box::new(InfixAst::Ident("b".to_owned())),
                Box::new(InfixAst::Ident("c".to_owned()))
            ))
        )
    )
}