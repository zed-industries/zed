mod parser {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "../examples/base.pest"]
    #[grammar = "../examples/calc.pest"]
    pub struct Parser;
}

use parser::Rule;
use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc::*, Op, PrattParser},
    Parser,
};
use std::io::{stdin, stdout, Write};

fn parse_to_str(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> String {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::int => primary.as_str().to_owned(),
            Rule::expr => parse_to_str(primary.into_inner(), pratt),
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::neg => format!("(-{})", rhs),
            _ => unreachable!(),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::fac => format!("({}!)", lhs),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => format!("({}+{})", lhs, rhs),
            Rule::sub => format!("({}-{})", lhs, rhs),
            Rule::mul => format!("({}*{})", lhs, rhs),
            Rule::div => format!("({}/{})", lhs, rhs),
            Rule::pow => format!("({}^{})", lhs, rhs),
            _ => unreachable!(),
        })
        .parse(pairs)
}

fn parse_to_i32(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> i128 {
    pratt
        .map_primary(|primary| match primary.as_rule() {
            Rule::int => primary.as_str().parse().unwrap(),
            Rule::expr => parse_to_i32(primary.into_inner(), pratt),
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::neg => -rhs,
            _ => unreachable!(),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::fac => (1..lhs + 1).product(),
            _ => unreachable!(),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::sub => lhs - rhs,
            Rule::mul => lhs * rhs,
            Rule::div => lhs / rhs,
            Rule::pow => (1..rhs + 1).map(|_| lhs).product(),
            _ => unreachable!(),
        })
        .parse(pairs)
}

fn main() {
    let pratt = PrattParser::new()
        .op(Op::infix(Rule::add, Left) | Op::infix(Rule::sub, Left))
        .op(Op::infix(Rule::mul, Left) | Op::infix(Rule::div, Left))
        .op(Op::infix(Rule::pow, Right))
        .op(Op::prefix(Rule::neg))
        .op(Op::postfix(Rule::fac));

    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        let source = {
            print!("> ");
            let _ = stdout.flush();
            let mut input = String::new();
            let _ = stdin.read_line(&mut input);
            input.trim().to_string()
        };

        let pairs = match parser::Parser::parse(Rule::program, &source) {
            Ok(mut parse_tree) => {
                parse_tree
                    .next()
                    .unwrap()
                    .into_inner() // inner of program
                    .next()
                    .unwrap()
                    .into_inner() // inner of expr
            }
            Err(err) => {
                println!("Failed parsing input: {:}", err);
                continue;
            }
        };

        print!("{} => ", source);
        print!("{} => ", parse_to_str(pairs.clone(), &pratt));
        println!("{}", parse_to_i32(pairs.clone(), &pratt));
    }
}
