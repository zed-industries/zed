//! Type and helper to collect the gramamr and rule documentation.

use pest::iterators::Pairs;
use pest_meta::parser::Rule;
use std::collections::HashMap;

/// Abstraction for the grammer and rule doc.
#[derive(Debug)]
pub struct DocComment {
    /// The grammar documentation is defined at the beginning of a file with //!.
    pub grammar_doc: String,

    /// HashMap for store all doc_comments for rules.
    /// key is rule name, value is doc_comment.
    pub line_docs: HashMap<String, String>,
}

/// Consume pairs to matches `Rule::grammar_doc`, `Rule::line_doc` into `DocComment`
///
/// e.g.
///
/// a pest file:
///
/// ```ignore
/// //! This is a grammar doc
/// /// line doc 1
/// /// line doc 2
/// foo = {}
///
/// /// line doc 3
/// bar = {}
/// ```
///
/// Then will get:
///
/// ```ignore
/// grammar_doc = "This is a grammar doc"
/// line_docs = { "foo": "line doc 1\nline doc 2", "bar": "line doc 3" }
/// ```
pub fn consume(pairs: Pairs<'_, Rule>) -> DocComment {
    let mut grammar_doc = String::new();

    let mut line_docs: HashMap<String, String> = HashMap::new();
    let mut line_doc = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::grammar_doc => {
                // grammar_doc > inner_doc
                let inner_doc = pair.into_inner().next().unwrap();
                grammar_doc.push_str(inner_doc.as_str());
                grammar_doc.push('\n');
            }
            Rule::grammar_rule => {
                if let Some(inner) = pair.into_inner().next() {
                    // grammar_rule > line_doc | identifier
                    match inner.as_rule() {
                        Rule::line_doc => {
                            if let Some(inner_doc) = inner.into_inner().next() {
                                line_doc.push_str(inner_doc.as_str());
                                line_doc.push('\n');
                            }
                        }
                        Rule::identifier => {
                            if !line_doc.is_empty() {
                                let rule_name = inner.as_str().to_owned();

                                // Remove last \n
                                line_doc.pop();
                                line_docs.insert(rule_name, line_doc.clone());
                                line_doc.clear();
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    if !grammar_doc.is_empty() {
        // Remove last \n
        grammar_doc.pop();
    }

    DocComment {
        grammar_doc,
        line_docs,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pest_meta::parser;
    use pest_meta::parser::Rule;

    #[test]
    fn test_doc_comment() {
        let pairs = match parser::parse(Rule::grammar_rules, include_str!("../tests/test.pest")) {
            Ok(pairs) => pairs,
            Err(_) => panic!("error parsing tests/test.pest"),
        };

        let doc_comment = super::consume(pairs);

        let mut expected = HashMap::new();
        expected.insert("foo".to_owned(), "Matches foo str, e.g.: `foo`".to_owned());
        expected.insert(
            "bar".to_owned(),
            "Matches bar str\n\n  Indent 2, e.g: `bar` or `foobar`".to_owned(),
        );
        expected.insert(
            "dar".to_owned(),
            "Matches dar\n\nMatch dar description\n".to_owned(),
        );
        assert_eq!(expected, doc_comment.line_docs);

        assert_eq!(
            "A parser for JSON file.\nAnd this is a example for JSON parser.\n\n    indent-4-space\n",
            doc_comment.grammar_doc
        );
    }

    #[test]
    fn test_empty_grammar_doc() {
        assert!(parser::parse(Rule::grammar_rules, "//!").is_ok());
        assert!(parser::parse(Rule::grammar_rules, "///").is_ok());
        assert!(parser::parse(Rule::grammar_rules, "//").is_ok());
        assert!(parser::parse(Rule::grammar_rules, "/// Line Doc").is_ok());
        assert!(parser::parse(Rule::grammar_rules, "//! Grammar Doc").is_ok());
        assert!(parser::parse(Rule::grammar_rules, "// Comment").is_ok());
    }
}
