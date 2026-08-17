#[macro_use]
extern crate pest_derive;
extern crate pest;

use pest::Parser;

#[derive(Parser)]
#[grammar = "../examples/help-menu.pest"]
struct HelpMenuGrammar;

const INPUT: &str = r"cli help
cli positional-command <required-single-argument> [optional-single-argument]
cli [choice | of | one | or | none | of | these | options]
cli <choice | of | one | of | these | options>
cli [nesting | <is | ok>]
";

fn main() {
    HelpMenuGrammar::parse(Rule::HelpMenu, INPUT)
        .expect("Error parsing file")
        .next()
        .expect("Infallible")
        .into_inner()
        .filter(|pair| Rule::Command == pair.as_rule())
        .for_each(|pair| {
            println!("{:#?}", pair);
        });
}
