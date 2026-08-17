use super::parser::{Selector, SelectorImplDescriptor};
use crate::selectors_vm::{DenseHashSet, MatchId};
use selectors::attr::{AttrSelectorOperator, ParsedCaseSensitivity};
use selectors::parser::{Combinator, Component, NthType};
use std::fmt::{self, Debug, Formatter};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub(crate) struct NthChild {
    step: i32,
    offset: i32,
}

impl NthChild {
    #[inline]
    #[must_use]
    pub const fn new(step: i32, offset: i32) -> Self {
        Self { step, offset }
    }

    #[must_use]
    pub const fn has_index(self, index: i32) -> bool {
        let Self { offset, step } = self;
        // wrap to prevent panic/abort. we won't wrap around anyway, even with a
        // max offset value (i32::MAX) since index is always more than 0
        let offsetted = index.wrapping_sub(offset);
        if step == 0 {
            offsetted == 0
        } else if (offsetted < 0 && step > 0) || (offsetted > 0 && step < 0) {
            false
        } else {
            // again, wrap the remainder op. overflow only occurs with
            // i32::MIN / -1. while the step can be -1, the offsetted
            // value will never be i32::MIN since this index is always
            // more than 0
            offsetted.wrapping_rem(step) == 0
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum OnTagNameExpr {
    ExplicitAny,
    Unmatchable,
    LocalName(Box<str>),
    NthChild(NthChild),
    NthOfType(NthChild),
}

#[derive(Eq, PartialEq)]
pub(crate) struct AttributeComparisonExpr {
    pub name: Box<str>,
    pub value: Box<str>,
    pub case_sensitivity: ParsedCaseSensitivity,
    pub operator: AttrSelectorOperator,
}

impl AttributeComparisonExpr {
    #[inline]
    #[must_use]
    pub const fn new(
        name: Box<str>,
        value: Box<str>,
        case_sensitivity: ParsedCaseSensitivity,
        operator: AttrSelectorOperator,
    ) -> Self {
        Self {
            name,
            value,
            case_sensitivity,
            operator,
        }
    }
}

impl Debug for AttributeComparisonExpr {
    #[cold]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AttributeExpr")
            .field("name", &self.name)
            .field("value", &self.value)
            .field("case_sensitivity", &self.case_sensitivity)
            .field(
                "operator",
                match self.operator {
                    AttrSelectorOperator::Equal => &"AttrSelectorOperator::Equal",
                    AttrSelectorOperator::Includes => &"AttrSelectorOperator::Includes",
                    AttrSelectorOperator::DashMatch => &"AttrSelectorOperator::DashMatch",
                    AttrSelectorOperator::Prefix => &"AttrSelectorOperator::Prefix",
                    AttrSelectorOperator::Substring => &"AttrSelectorOperator::Substring",
                    AttrSelectorOperator::Suffix => &"AttrSelectorOperator::Suffix",
                },
            )
            .finish()
    }
}

/// An attribute check when attributes are received and parsed.
#[derive(PartialEq, Eq, Debug)]
pub(crate) enum OnAttributesExpr {
    Id(Box<str>),
    Class(Box<str>),
    AttributeExists(Box<str>),
    AttributeComparisonExpr(AttributeComparisonExpr),
}

#[derive(PartialEq, Eq, Debug)]
/// Conditions executed as part of a predicate, or an "expect" in pseudo instructions.
/// These are executed in order of definition.
enum Condition {
    OnTagName(OnTagNameExpr),
    OnAttributes(OnAttributesExpr),
}

impl From<&Component<SelectorImplDescriptor>> for Condition {
    fn from(component: &Component<SelectorImplDescriptor>) -> Self {
        match component {
            Component::LocalName(n) => {
                Self::OnTagName(OnTagNameExpr::LocalName(n.name.to_boxed_slice()))
            }
            Component::ExplicitUniversalType | Component::ExplicitAnyNamespace => {
                Self::OnTagName(OnTagNameExpr::ExplicitAny)
            }
            Component::ExplicitNoNamespace => Self::OnTagName(OnTagNameExpr::Unmatchable),
            Component::ID(id) => Self::OnAttributes(OnAttributesExpr::Id(id.to_boxed_slice())),
            Component::Class(c) => Self::OnAttributes(OnAttributesExpr::Class(c.to_boxed_slice())),
            Component::AttributeInNoNamespaceExists { local_name, .. } => Self::OnAttributes(
                OnAttributesExpr::AttributeExists(local_name.to_boxed_slice()),
            ),
            &Component::AttributeInNoNamespace {
                ref local_name,
                ref value,
                operator,
                case_sensitivity,
            } => Self::OnAttributes(OnAttributesExpr::AttributeComparisonExpr(
                AttributeComparisonExpr::new(
                    local_name.to_boxed_slice(),
                    value.to_boxed_slice(),
                    case_sensitivity,
                    operator,
                ),
            )),
            Component::Nth(data) if data.ty == NthType::Child => Self::OnTagName(
                OnTagNameExpr::NthChild(NthChild::new(data.an_plus_b.0, data.an_plus_b.1)),
            ),
            Component::Nth(data) if data.ty == NthType::OfType => Self::OnTagName(
                OnTagNameExpr::NthOfType(NthChild::new(data.an_plus_b.0, data.an_plus_b.1)),
            ),
            // NOTE: the rest of the components are explicit namespace or
            // pseudo class-related. Ideally none of them should appear in
            // the parsed selector as we should bail earlier in the parser.
            // Otherwise, we'll have AST in invalid state in case of error.
            bad_selector => {
                debug_assert!(
                    false,
                    "Unsupported selector components should be filtered out by the parser: {bad_selector:?}"
                );
                Self::OnTagName(OnTagNameExpr::Unmatchable)
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct Expr<E>
where
    E: PartialEq + Eq + Debug,
{
    pub simple_expr: E,
    pub negation: bool,
}

impl<E> Expr<E>
where
    E: PartialEq + Eq + Debug,
{
    #[inline]
    const fn new(simple_expr: E, negation: bool) -> Self {
        Self {
            simple_expr,
            negation,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Default)]
pub(crate) struct Predicate {
    pub on_tag_name_exprs: Vec<Expr<OnTagNameExpr>>,
    pub on_attr_exprs: Vec<Expr<OnAttributesExpr>>,
}

#[inline]
fn add_expr_to_list<E>(list: &mut Vec<Expr<E>>, expr: E, negation: bool)
where
    E: PartialEq + Eq + Debug,
{
    list.push(Expr::new(expr, negation));
}

impl Predicate {
    #[inline]
    fn add_component(&mut self, component: &Component<SelectorImplDescriptor>, negation: bool) {
        match Condition::from(component) {
            Condition::OnTagName(e) => add_expr_to_list(&mut self.on_tag_name_exprs, e, negation),
            Condition::OnAttributes(e) => add_expr_to_list(&mut self.on_attr_exprs, e, negation),
        }
    }

    fn add_selector_components(
        &mut self,
        selector: &selectors::parser::Selector<SelectorImplDescriptor>,
        negation: bool,
    ) {
        for component in selector.iter() {
            match component {
                Component::Negation(nested_selectors) => {
                    for nested_selector in nested_selectors.slice() {
                        self.add_selector_components(nested_selector, !negation);
                    }
                }
                _ => {
                    self.add_component(component, negation);
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct AstNode {
    pub predicate: Predicate,
    pub children: Vec<Self>,
    pub descendants: Vec<Self>,
    pub match_ids: DenseHashSet,
}

impl AstNode {
    #[inline]
    #[must_use]
    fn new(predicate: Predicate) -> Self {
        Self {
            predicate,
            children: Vec::default(),
            descendants: Vec::default(),
            match_ids: DenseHashSet::new(),
        }
    }
}

// exposed for selectors_ast tool
#[derive(Default, PartialEq, Eq, Debug)]
pub struct Ast {
    pub(crate) root: Vec<AstNode>,
    // NOTE: used to preallocate instruction vector during compilation.
    pub(crate) cumulative_node_count: usize,
}

impl Ast {
    #[inline]
    fn host_expressions(
        predicate: Predicate,
        branches: &mut Vec<AstNode>,
        cumulative_node_count: &mut usize,
    ) -> usize {
        branches
            .iter()
            .enumerate()
            .find(|(_, n)| n.predicate == predicate)
            .map(|(i, _)| i)
            .unwrap_or_else(move || {
                branches.push(AstNode::new(predicate));
                *cumulative_node_count += 1;

                branches.len() - 1
            })
    }

    /// `match_id` is a small integer chosen by the caller. It will be returned back in `MatchInfo`
    pub fn add_selector(&mut self, selector: &Selector, match_id: MatchId) {
        for selector_item in (selector.0).slice() {
            let mut predicate = Predicate::default();
            let mut branches = &mut self.root;

            macro_rules! host_and_switch_branch_vec {
                ($branches:ident) => {{
                    let node_idx = Self::host_expressions(
                        predicate,
                        branches,
                        &mut self.cumulative_node_count,
                    );

                    branches = &mut branches[node_idx].$branches;
                    predicate = Predicate::default();
                }};
            }

            for component in selector_item.iter_raw_parse_order_from(0) {
                match component {
                    Component::Combinator(Combinator::Child) => {
                        host_and_switch_branch_vec!(children);
                    }
                    Component::Combinator(Combinator::Descendant) => {
                        host_and_switch_branch_vec!(descendants);
                    }
                    Component::Negation(ss) => {
                        for s in ss.slice() {
                            predicate.add_selector_components(s, true);
                        }
                    }
                    _ => predicate.add_component(component, false),
                }
            }

            let node_idx =
                Self::host_expressions(predicate, branches, &mut self.cumulative_node_count);

            branches[node_idx].match_ids.insert(match_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selectors_vm::{DenseHashSet, SelectorError};

    #[track_caller]
    fn assert_ast(selectors: &[&str], expected: Ast) {
        let mut ast = Ast::default();

        for (selector, match_id) in selectors.iter().zip(0..) {
            ast.add_selector(&selector.parse().unwrap(), match_id);
        }

        assert_eq!(ast, expected);
    }

    #[track_caller]
    fn assert_err(selector: &str, expected_err: SelectorError) {
        assert_eq!(selector.parse::<Selector>().unwrap_err(), expected_err);
    }

    #[test]
    fn simple_non_attr_expression() {
        for (selector, expected) in [
            (
                "*",
                Expr {
                    simple_expr: OnTagNameExpr::ExplicitAny,
                    negation: false,
                },
            ),
            (
                "div",
                Expr {
                    simple_expr: OnTagNameExpr::LocalName("div".into()),
                    negation: false,
                },
            ),
            (
                ":not(div)",
                Expr {
                    simple_expr: OnTagNameExpr::LocalName("div".into()),
                    negation: true,
                },
            ),
        ] {
            assert_ast(
                &[selector],
                Ast {
                    root: vec![AstNode {
                        predicate: Predicate {
                            on_tag_name_exprs: vec![expected],
                            ..Default::default()
                        },
                        children: vec![],
                        descendants: vec![],
                        match_ids: DenseHashSet::from([0]),
                    }],
                    cumulative_node_count: 1,
                },
            );
        }
    }

    #[test]
    fn simple_attr_expression() {
        for (selector, expected) in [
            (
                "#foo",
                Expr {
                    simple_expr: OnAttributesExpr::Id("foo".into()),
                    negation: false,
                },
            ),
            (
                ".bar",
                Expr {
                    simple_expr: OnAttributesExpr::Class("bar".into()),
                    negation: false,
                },
            ),
            (
                "[foo]",
                Expr {
                    simple_expr: OnAttributesExpr::AttributeExists("foo".into()),
                    negation: false,
                },
            ),
            (
                r#"[foo="bar"]"#,
                Expr {
                    simple_expr: OnAttributesExpr::AttributeComparisonExpr(
                        AttributeComparisonExpr {
                            name: "foo".into(),
                            value: "bar".into(),
                            case_sensitivity: ParsedCaseSensitivity::CaseSensitive,
                            operator: AttrSelectorOperator::Equal,
                        },
                    ),
                    negation: false,
                },
            ),
            (
                r#"[foo*=""]"#,
                Expr {
                    simple_expr: OnAttributesExpr::AttributeComparisonExpr(
                        AttributeComparisonExpr {
                            name: "foo".into(),
                            value: "".into(),
                            case_sensitivity: ParsedCaseSensitivity::CaseSensitive,
                            operator: AttrSelectorOperator::Substring,
                        },
                    ),
                    negation: false,
                },
            ),
            (
                r#"[foo~="bar" i]"#,
                Expr {
                    simple_expr: OnAttributesExpr::AttributeComparisonExpr(
                        AttributeComparisonExpr {
                            name: "foo".into(),
                            value: "bar".into(),
                            case_sensitivity: ParsedCaseSensitivity::AsciiCaseInsensitive,
                            operator: AttrSelectorOperator::Includes,
                        },
                    ),
                    negation: false,
                },
            ),
            (
                r#"[foo|="bar" s]"#,
                Expr {
                    simple_expr: OnAttributesExpr::AttributeComparisonExpr(
                        AttributeComparisonExpr {
                            name: "foo".into(),
                            value: "bar".into(),
                            case_sensitivity: ParsedCaseSensitivity::ExplicitCaseSensitive,
                            operator: AttrSelectorOperator::DashMatch,
                        },
                    ),
                    negation: false,
                },
            ),
            (
                r#"[foo^="bar"]"#,
                Expr {
                    simple_expr: OnAttributesExpr::AttributeComparisonExpr(
                        AttributeComparisonExpr {
                            name: "foo".into(),
                            value: "bar".into(),
                            case_sensitivity: ParsedCaseSensitivity::CaseSensitive,
                            operator: AttrSelectorOperator::Prefix,
                        },
                    ),
                    negation: false,
                },
            ),
            (
                r#"[foo*="bar"]"#,
                Expr {
                    simple_expr: OnAttributesExpr::AttributeComparisonExpr(
                        AttributeComparisonExpr {
                            name: "foo".into(),
                            value: "bar".into(),
                            case_sensitivity: ParsedCaseSensitivity::CaseSensitive,
                            operator: AttrSelectorOperator::Substring,
                        },
                    ),
                    negation: false,
                },
            ),
            (
                r#"[foo$="bar"]"#,
                Expr {
                    simple_expr: OnAttributesExpr::AttributeComparisonExpr(
                        AttributeComparisonExpr {
                            name: "foo".into(),
                            value: "bar".into(),
                            case_sensitivity: ParsedCaseSensitivity::CaseSensitive,
                            operator: AttrSelectorOperator::Suffix,
                        },
                    ),
                    negation: false,
                },
            ),
            (
                r#":not([foo$="bar"])"#,
                Expr {
                    simple_expr: OnAttributesExpr::AttributeComparisonExpr(
                        AttributeComparisonExpr {
                            name: "foo".into(),
                            value: "bar".into(),
                            case_sensitivity: ParsedCaseSensitivity::CaseSensitive,
                            operator: AttrSelectorOperator::Suffix,
                        },
                    ),
                    negation: true,
                },
            ),
        ] {
            assert_ast(
                &[selector],
                Ast {
                    root: vec![AstNode {
                        predicate: Predicate {
                            on_attr_exprs: vec![expected],
                            ..Default::default()
                        },
                        children: vec![],
                        descendants: vec![],
                        match_ids: DenseHashSet::from([0]),
                    }],
                    cumulative_node_count: 1,
                },
            );
        }
    }

    #[test]
    fn compound_selectors() {
        assert_ast(
            &["div.foo#bar:not([baz])"],
            Ast {
                root: vec![AstNode {
                    predicate: Predicate {
                        on_tag_name_exprs: vec![Expr {
                            simple_expr: OnTagNameExpr::LocalName("div".into()),
                            negation: false,
                        }],
                        on_attr_exprs: vec![
                            Expr {
                                simple_expr: OnAttributesExpr::AttributeExists("baz".into()),
                                negation: true,
                            },
                            Expr {
                                simple_expr: OnAttributesExpr::Id("bar".into()),
                                negation: false,
                            },
                            Expr {
                                simple_expr: OnAttributesExpr::Class("foo".into()),
                                negation: false,
                            },
                        ],
                    },
                    children: vec![],
                    descendants: vec![],
                    match_ids: DenseHashSet::from([0]),
                }],
                cumulative_node_count: 1,
            },
        );
    }

    #[test]
    fn multiple_payloads() {
        assert_ast(
            &["#foo", "#foo"],
            Ast {
                root: vec![AstNode {
                    predicate: Predicate {
                        on_attr_exprs: vec![Expr {
                            simple_expr: OnAttributesExpr::Id("foo".into()),
                            negation: false,
                        }],
                        ..Default::default()
                    },
                    children: vec![],
                    descendants: vec![],
                    match_ids: DenseHashSet::from([0, 1]),
                }],
                cumulative_node_count: 1,
            },
        );
    }

    #[test]
    fn selector_list() {
        assert_ast(
            &["#foo > div, #foo > span", "#foo > .c1, #foo > .c2"],
            Ast {
                root: vec![AstNode {
                    predicate: Predicate {
                        on_attr_exprs: vec![Expr {
                            simple_expr: OnAttributesExpr::Id("foo".into()),
                            negation: false,
                        }],
                        ..Default::default()
                    },
                    children: vec![
                        AstNode {
                            predicate: Predicate {
                                on_tag_name_exprs: vec![Expr {
                                    simple_expr: OnTagNameExpr::LocalName("div".into()),
                                    negation: false,
                                }],
                                ..Default::default()
                            },
                            children: vec![],
                            descendants: vec![],
                            match_ids: DenseHashSet::from([0]),
                        },
                        AstNode {
                            predicate: Predicate {
                                on_tag_name_exprs: vec![Expr {
                                    simple_expr: OnTagNameExpr::LocalName("span".into()),
                                    negation: false,
                                }],
                                ..Default::default()
                            },
                            children: vec![],
                            descendants: vec![],
                            match_ids: DenseHashSet::from([0]),
                        },
                        AstNode {
                            predicate: Predicate {
                                on_attr_exprs: vec![Expr {
                                    simple_expr: OnAttributesExpr::Class("c1".into()),
                                    negation: false,
                                }],
                                ..Default::default()
                            },
                            children: vec![],
                            descendants: vec![],
                            match_ids: DenseHashSet::from([1]),
                        },
                        AstNode {
                            predicate: Predicate {
                                on_attr_exprs: vec![Expr {
                                    simple_expr: OnAttributesExpr::Class("c2".into()),
                                    negation: false,
                                }],
                                ..Default::default()
                            },
                            children: vec![],
                            descendants: vec![],
                            match_ids: DenseHashSet::from([1]),
                        },
                    ],
                    descendants: vec![],
                    match_ids: DenseHashSet::from([]),
                }],
                cumulative_node_count: 5,
            },
        );
    }

    #[test]
    fn combinators() {
        assert_ast(
            &[
                ".c1 > .c2 .c3 #foo",
                ".c1 > .c2 #bar",
                ".c1 > #qux",
                ".c1 #baz",
                ".c1 [foo] [bar]",
                "#quz",
            ],
            Ast {
                root: vec![
                    AstNode {
                        predicate: Predicate {
                            on_attr_exprs: vec![Expr {
                                simple_expr: OnAttributesExpr::Class("c1".into()),
                                negation: false,
                            }],
                            ..Default::default()
                        },
                        children: vec![
                            AstNode {
                                predicate: Predicate {
                                    on_attr_exprs: vec![Expr {
                                        simple_expr: OnAttributesExpr::Class("c2".into()),
                                        negation: false,
                                    }],
                                    ..Default::default()
                                },
                                children: vec![],
                                descendants: vec![
                                    AstNode {
                                        predicate: Predicate {
                                            on_attr_exprs: vec![Expr {
                                                simple_expr: OnAttributesExpr::Class("c3".into()),
                                                negation: false,
                                            }],
                                            ..Default::default()
                                        },
                                        children: vec![],
                                        descendants: vec![AstNode {
                                            predicate: Predicate {
                                                on_attr_exprs: vec![Expr {
                                                    simple_expr: OnAttributesExpr::Id("foo".into()),
                                                    negation: false,
                                                }],
                                                ..Default::default()
                                            },
                                            children: vec![],
                                            descendants: vec![],
                                            match_ids: DenseHashSet::from([0]),
                                        }],
                                        match_ids: DenseHashSet::from([]),
                                    },
                                    AstNode {
                                        predicate: Predicate {
                                            on_attr_exprs: vec![Expr {
                                                simple_expr: OnAttributesExpr::Id("bar".into()),
                                                negation: false,
                                            }],
                                            ..Default::default()
                                        },
                                        children: vec![],
                                        descendants: vec![],
                                        match_ids: DenseHashSet::from([1]),
                                    },
                                ],
                                match_ids: DenseHashSet::from([]),
                            },
                            AstNode {
                                predicate: Predicate {
                                    on_attr_exprs: vec![Expr {
                                        simple_expr: OnAttributesExpr::Id("qux".into()),
                                        negation: false,
                                    }],
                                    ..Default::default()
                                },
                                children: vec![],
                                descendants: vec![],
                                match_ids: DenseHashSet::from([2]),
                            },
                        ],
                        descendants: vec![
                            AstNode {
                                predicate: Predicate {
                                    on_attr_exprs: vec![Expr {
                                        simple_expr: OnAttributesExpr::Id("baz".into()),
                                        negation: false,
                                    }],
                                    ..Default::default()
                                },
                                children: vec![],
                                descendants: vec![],
                                match_ids: DenseHashSet::from([3]),
                            },
                            AstNode {
                                predicate: Predicate {
                                    on_attr_exprs: vec![Expr {
                                        simple_expr: OnAttributesExpr::AttributeExists(
                                            "foo".into(),
                                        ),
                                        negation: false,
                                    }],
                                    ..Default::default()
                                },
                                children: vec![],
                                descendants: vec![AstNode {
                                    predicate: Predicate {
                                        on_attr_exprs: vec![Expr {
                                            simple_expr: OnAttributesExpr::AttributeExists(
                                                "bar".into(),
                                            ),
                                            negation: false,
                                        }],
                                        ..Default::default()
                                    },
                                    children: vec![],
                                    descendants: vec![],
                                    match_ids: DenseHashSet::from([4]),
                                }],
                                match_ids: DenseHashSet::from([]),
                            },
                        ],
                        match_ids: DenseHashSet::from([]),
                    },
                    AstNode {
                        predicate: Predicate {
                            on_attr_exprs: vec![Expr {
                                simple_expr: OnAttributesExpr::Id("quz".into()),
                                negation: false,
                            }],
                            ..Default::default()
                        },
                        children: vec![],
                        descendants: vec![],
                        match_ids: DenseHashSet::from([5]),
                    },
                ],
                cumulative_node_count: 10,
            },
        );
    }

    #[test]
    fn parse_errors() {
        assert_err("div@", SelectorError::UnexpectedToken);
        assert_err("div.", SelectorError::UnexpectedEnd);
        assert_err(r#"div[="foo"]"#, SelectorError::MissingAttributeName);
        assert_err("", SelectorError::EmptySelector);
        assert_err("div >", SelectorError::DanglingCombinator);
        assert_err(
            r#"div[foo~"bar"]"#,
            SelectorError::UnexpectedTokenInAttribute,
        );
        assert_err("svg|img", SelectorError::NamespacedSelector);
        assert_err(".foo()", SelectorError::InvalidClassName);
        assert_err(":not()", SelectorError::EmptySelector);
        assert_err("div + span", SelectorError::UnsupportedCombinator('+'));
        assert_err("div ~ span", SelectorError::UnsupportedCombinator('~'));
        assert_err(":nth-child(n of a)", SelectorError::UnexpectedToken);
    }

    #[test]
    fn pseudo_class_parse_errors() {
        for s in &[
            ":active",
            ":any-link",
            ":blank",
            ":checked",
            ":current",
            ":default",
            ":defined",
            ":dir(rtl)",
            ":disabled",
            ":drop",
            ":empty",
            ":enabled",
            ":first",
            ":fullscreen",
            ":future",
            ":focus",
            ":focus-visible",
            ":focus-within",
            ":has(div)",
            ":host",
            ":host(h1)",
            ":host-context(h1)",
            ":hover",
            ":indeterminate",
            ":in-range",
            ":invalid",
            ":is(header)",
            ":lang(en)",
            ":last-child",
            ":last-of-type",
            ":left",
            ":link",
            ":local-link",
            ":nth-col(1)",
            ":nth-last-child(1)",
            ":nth-last-col(1)",
            ":nth-last-of-type(1)",
            ":only-child",
            ":only-of-type",
            ":optional",
            ":out-of-range",
            ":past",
            ":placeholder-shown",
            ":read-only",
            ":read-write",
            ":required",
            ":right",
            ":root",
            ":scope",
            ":target",
            ":target-within",
            ":user-invalid",
            ":valid",
            ":visited",
            ":where(p)",
            ":not(foo bar)",
            ":not(foo > bar)",
            ":not(* > .x)",
        ] {
            assert_err(s, SelectorError::UnsupportedPseudoClassOrElement);
        }
    }

    #[test]
    fn pseudo_elements_parse_errors() {
        for s in &[
            "::after",
            "::backdrop",
            "::before",
            "::cue",
            "::first-letter",
            "::first-line",
            "::grammar-error",
            "::marker",
            "::placeholder",
            "::selection",
            "::slotted()",
            "::spelling-error",
        ] {
            assert_err(s, SelectorError::UnsupportedPseudoClassOrElement);
        }
    }

    #[test]
    fn negated_pseudo_class_parse_error() {
        assert_err(
            ":not(:nth-last-child(even))",
            SelectorError::UnsupportedPseudoClassOrElement,
        );
    }

    #[test]
    fn nested_not_selector() {
        assert_ast(
            &[":not(:not(div))"],
            Ast {
                root: vec![AstNode {
                    predicate: Predicate {
                        on_tag_name_exprs: vec![Expr {
                            simple_expr: OnTagNameExpr::LocalName("div".into()),
                            negation: false, // simplified double negation
                        }],
                        ..Default::default()
                    },
                    children: vec![],
                    descendants: vec![],
                    match_ids: DenseHashSet::from([0]),
                }],
                cumulative_node_count: 1,
            },
        );

        assert_ast(
            &[":not(:not(:not(div)))"],
            Ast {
                root: vec![AstNode {
                    predicate: Predicate {
                        on_tag_name_exprs: vec![Expr {
                            simple_expr: OnTagNameExpr::LocalName("div".into()),
                            negation: true,
                        }],
                        ..Default::default()
                    },
                    children: vec![],
                    descendants: vec![],
                    match_ids: DenseHashSet::from([0]),
                }],
                cumulative_node_count: 1,
            },
        );

        assert_ast(
            &["div:not(:not(.foo))"],
            Ast {
                root: vec![AstNode {
                    predicate: Predicate {
                        on_tag_name_exprs: vec![Expr {
                            simple_expr: OnTagNameExpr::LocalName("div".into()),
                            negation: false,
                        }],
                        on_attr_exprs: vec![Expr {
                            simple_expr: OnAttributesExpr::Class("foo".into()),
                            negation: false, // simplified double negation
                        }],
                    },
                    children: vec![],
                    descendants: vec![],
                    match_ids: DenseHashSet::from([0]),
                }],
                cumulative_node_count: 1,
            },
        );
    }

    #[test]
    fn nth_child_is_index() {
        let even = NthChild::new(2, 0);
        assert!(even.has_index(2));
        assert!(!even.has_index(1));

        let odd = NthChild::new(2, 1);
        assert!(odd.has_index(1));
        assert!(!odd.has_index(2));
        assert!(odd.has_index(3));

        let first = NthChild::new(0, 1);
        assert!(first.has_index(1));
        assert!(!first.has_index(2));
        assert!(!first.has_index(3));
    }
}
