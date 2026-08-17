extern crate peg;

peg::parser!( grammar arithmetic() for str {
    rule ident() -> &'input str = $(['a'..='z']+)

    pub rule expression() -> Node = precedence!{
        start:position!() node:@ end:position!() { Node { start, node, end} }
        --
        x:(@) "+" y:@ { Op::Add(Box::new(x), Box::new(y)) }
        --
        x:(@) "*" y:@ { Op::Mul(Box::new(x), Box::new(y)) }
        --
        i:ident() [' '|'\n']* { Op::Ident(i.to_owned()) }
    }
});

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    node: Op,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Op {
    Ident(String),
    Add(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
}

fn main(){
    assert_eq!(arithmetic::expression("a+b*c").unwrap(),
        Node {
            start: 0,
            end: 5,
            node: Op::Add(
                Box::new(Node {
                    start: 0,
                    end: 1,
                    node: Op::Ident("a".into())
                }),
                Box::new(Node {
                    start: 2,
                    end: 5,
                    node: Op::Mul(
                        Box::new(Node {
                            start: 2,
                            end: 3,
                            node: Op::Ident("b".into())
                        }),
                        Box::new(Node {
                            start: 4,
                            end: 5,
                            node: Op::Ident("c".into())
                        })
                    )
                })
            )
        }
    );
}