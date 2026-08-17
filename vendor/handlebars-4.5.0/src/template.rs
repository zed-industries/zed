use std::collections::{HashMap, VecDeque};
use std::convert::From;
use std::iter::Peekable;
use std::str::FromStr;

use pest::error::LineColLocation;
use pest::iterators::Pair;
use pest::{Parser, Position, Span};
use serde_json::value::Value as Json;

use crate::error::{TemplateError, TemplateErrorReason};
use crate::grammar::{HandlebarsParser, Rule};
use crate::json::path::{parse_json_path_from_iter, Path};
use crate::support;

use self::TemplateElement::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TemplateMapping(pub usize, pub usize);

/// A handlebars template
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Template {
    pub name: Option<String>,
    pub elements: Vec<TemplateElement>,
    pub mapping: Vec<TemplateMapping>,
}

#[derive(Default)]
pub(crate) struct TemplateOptions {
    pub(crate) prevent_indent: bool,
    pub(crate) name: Option<String>,
}

impl TemplateOptions {
    fn name(&self) -> String {
        self.name
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "Unnamed".to_owned())
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Subexpression {
    // we use box here avoid resursive struct definition
    pub element: Box<TemplateElement>,
}

impl Subexpression {
    pub fn new(
        name: Parameter,
        params: Vec<Parameter>,
        hash: HashMap<String, Parameter>,
    ) -> Subexpression {
        Subexpression {
            element: Box::new(Expression(Box::new(HelperTemplate {
                name,
                params,
                hash,
                template: None,
                inverse: None,
                block_param: None,
                block: false,
            }))),
        }
    }

    pub fn is_helper(&self) -> bool {
        match *self.as_element() {
            TemplateElement::Expression(ref ht) => !ht.is_name_only(),
            _ => false,
        }
    }

    pub fn as_element(&self) -> &TemplateElement {
        self.element.as_ref()
    }

    pub fn name(&self) -> &str {
        match *self.as_element() {
            // FIXME: avoid unwrap here
            Expression(ref ht) => ht.name.as_name().unwrap(),
            _ => unreachable!(),
        }
    }

    pub fn params(&self) -> Option<&Vec<Parameter>> {
        match *self.as_element() {
            Expression(ref ht) => Some(&ht.params),
            _ => None,
        }
    }

    pub fn hash(&self) -> Option<&HashMap<String, Parameter>> {
        match *self.as_element() {
            Expression(ref ht) => Some(&ht.hash),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BlockParam {
    Single(Parameter),
    Pair((Parameter, Parameter)),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ExpressionSpec {
    pub name: Parameter,
    pub params: Vec<Parameter>,
    pub hash: HashMap<String, Parameter>,
    pub block_param: Option<BlockParam>,
    pub omit_pre_ws: bool,
    pub omit_pro_ws: bool,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Parameter {
    // for helper name only
    Name(String),
    // for expression, helper param and hash
    Path(Path),
    Literal(Json),
    Subexpression(Subexpression),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct HelperTemplate {
    pub name: Parameter,
    pub params: Vec<Parameter>,
    pub hash: HashMap<String, Parameter>,
    pub block_param: Option<BlockParam>,
    pub template: Option<Template>,
    pub inverse: Option<Template>,
    pub block: bool,
}

impl HelperTemplate {
    pub fn new(exp: ExpressionSpec, block: bool) -> HelperTemplate {
        HelperTemplate {
            name: exp.name,
            params: exp.params,
            hash: exp.hash,
            block_param: exp.block_param,
            block,
            template: None,
            inverse: None,
        }
    }

    // test only
    pub(crate) fn with_path(path: Path) -> HelperTemplate {
        HelperTemplate {
            name: Parameter::Path(path),
            params: Vec::with_capacity(5),
            hash: HashMap::new(),
            block_param: None,
            template: None,
            inverse: None,
            block: false,
        }
    }

    pub(crate) fn is_name_only(&self) -> bool {
        !self.block && self.params.is_empty() && self.hash.is_empty()
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DecoratorTemplate {
    pub name: Parameter,
    pub params: Vec<Parameter>,
    pub hash: HashMap<String, Parameter>,
    pub template: Option<Template>,
    // for partial indent
    pub indent: Option<String>,
}

impl DecoratorTemplate {
    pub fn new(exp: ExpressionSpec) -> DecoratorTemplate {
        DecoratorTemplate {
            name: exp.name,
            params: exp.params,
            hash: exp.hash,
            template: None,
            indent: None,
        }
    }
}

impl Parameter {
    pub fn as_name(&self) -> Option<&str> {
        match self {
            Parameter::Name(ref n) => Some(n),
            Parameter::Path(ref p) => Some(p.raw()),
            _ => None,
        }
    }

    pub fn parse(s: &str) -> Result<Parameter, TemplateError> {
        let parser = HandlebarsParser::parse(Rule::parameter, s)
            .map_err(|_| TemplateError::of(TemplateErrorReason::InvalidParam(s.to_owned())))?;

        let mut it = parser.flatten().peekable();
        Template::parse_param(s, &mut it, s.len() - 1)
    }

    fn debug_name(&self) -> String {
        if let Some(name) = self.as_name() {
            name.to_owned()
        } else {
            format!("{:?}", self)
        }
    }
}

impl Template {
    pub fn new() -> Template {
        Template::default()
    }

    fn push_element(&mut self, e: TemplateElement, line: usize, col: usize) {
        self.elements.push(e);
        self.mapping.push(TemplateMapping(line, col));
    }

    fn parse_subexpression<'a, I>(
        source: &'a str,
        it: &mut Peekable<I>,
        limit: usize,
    ) -> Result<Parameter, TemplateError>
    where
        I: Iterator<Item = Pair<'a, Rule>>,
    {
        let espec = Template::parse_expression(source, it.by_ref(), limit)?;
        Ok(Parameter::Subexpression(Subexpression::new(
            espec.name,
            espec.params,
            espec.hash,
        )))
    }

    fn parse_name<'a, I>(
        source: &'a str,
        it: &mut Peekable<I>,
        _: usize,
    ) -> Result<Parameter, TemplateError>
    where
        I: Iterator<Item = Pair<'a, Rule>>,
    {
        let name_node = it.next().unwrap();
        let rule = name_node.as_rule();
        let name_span = name_node.as_span();
        match rule {
            Rule::identifier | Rule::partial_identifier | Rule::invert_tag_item => {
                Ok(Parameter::Name(name_span.as_str().to_owned()))
            }
            Rule::reference => {
                let paths = parse_json_path_from_iter(it, name_span.end());
                Ok(Parameter::Path(Path::new(name_span.as_str(), paths)))
            }
            Rule::subexpression => {
                Template::parse_subexpression(source, it.by_ref(), name_span.end())
            }
            _ => unreachable!(),
        }
    }

    fn parse_param<'a, I>(
        source: &'a str,
        it: &mut Peekable<I>,
        _: usize,
    ) -> Result<Parameter, TemplateError>
    where
        I: Iterator<Item = Pair<'a, Rule>>,
    {
        let mut param = it.next().unwrap();
        if param.as_rule() == Rule::param {
            param = it.next().unwrap();
        }
        let param_rule = param.as_rule();
        let param_span = param.as_span();
        let result = match param_rule {
            Rule::reference => {
                let path_segs = parse_json_path_from_iter(it, param_span.end());
                Parameter::Path(Path::new(param_span.as_str(), path_segs))
            }
            Rule::literal => {
                // Parse the parameter as a JSON literal
                let param_literal = it.next().unwrap();
                let json_result = match param_literal.as_rule() {
                    Rule::string_literal
                        if it.peek().unwrap().as_rule() == Rule::string_inner_single_quote =>
                    {
                        // ...unless the parameter is a single-quoted string.
                        // In that case, transform it to a double-quoted string
                        // and then parse it as a JSON literal.
                        let string_inner_single_quote = it.next().unwrap();
                        let double_quoted = format!(
                            "\"{}\"",
                            string_inner_single_quote
                                .as_str()
                                .replace("\\'", "'")
                                .replace('"', "\\\"")
                        );
                        Json::from_str(&double_quoted)
                    }
                    _ => Json::from_str(param_span.as_str()),
                };
                if let Ok(json) = json_result {
                    Parameter::Literal(json)
                } else {
                    return Err(TemplateError::of(TemplateErrorReason::InvalidParam(
                        param_span.as_str().to_owned(),
                    )));
                }
            }
            Rule::subexpression => {
                Template::parse_subexpression(source, it.by_ref(), param_span.end())?
            }
            _ => unreachable!(),
        };

        while let Some(n) = it.peek() {
            let n_span = n.as_span();
            if n_span.end() > param_span.end() {
                break;
            }
            it.next();
        }

        Ok(result)
    }

    fn parse_hash<'a, I>(
        source: &'a str,
        it: &mut Peekable<I>,
        limit: usize,
    ) -> Result<(String, Parameter), TemplateError>
    where
        I: Iterator<Item = Pair<'a, Rule>>,
    {
        let name = it.next().unwrap();
        let name_node = name.as_span();
        // identifier
        let key = name_node.as_str().to_owned();

        let value = Template::parse_param(source, it.by_ref(), limit)?;
        Ok((key, value))
    }

    fn parse_block_param<'a, I>(_: &'a str, it: &mut Peekable<I>, limit: usize) -> BlockParam
    where
        I: Iterator<Item = Pair<'a, Rule>>,
    {
        let p1_name = it.next().unwrap();
        let p1_name_span = p1_name.as_span();
        // identifier
        let p1 = p1_name_span.as_str().to_owned();

        let p2 = it.peek().and_then(|p2_name| {
            let p2_name_span = p2_name.as_span();
            if p2_name_span.end() <= limit {
                Some(p2_name_span.as_str().to_owned())
            } else {
                None
            }
        });

        if let Some(p2) = p2 {
            it.next();
            BlockParam::Pair((Parameter::Name(p1), Parameter::Name(p2)))
        } else {
            BlockParam::Single(Parameter::Name(p1))
        }
    }

    fn parse_expression<'a, I>(
        source: &'a str,
        it: &mut Peekable<I>,
        limit: usize,
    ) -> Result<ExpressionSpec, TemplateError>
    where
        I: Iterator<Item = Pair<'a, Rule>>,
    {
        let mut params: Vec<Parameter> = Vec::new();
        let mut hashes: HashMap<String, Parameter> = HashMap::new();
        let mut omit_pre_ws = false;
        let mut omit_pro_ws = false;
        let mut block_param = None;

        if it.peek().unwrap().as_rule() == Rule::pre_whitespace_omitter {
            omit_pre_ws = true;
            it.next();
        }

        let name = Template::parse_name(source, it.by_ref(), limit)?;

        loop {
            let rule;
            let end;
            if let Some(pair) = it.peek() {
                let pair_span = pair.as_span();
                if pair_span.end() < limit {
                    rule = pair.as_rule();
                    end = pair_span.end();
                } else {
                    break;
                }
            } else {
                break;
            }

            it.next();

            match rule {
                Rule::param => {
                    params.push(Template::parse_param(source, it.by_ref(), end)?);
                }
                Rule::hash => {
                    let (key, value) = Template::parse_hash(source, it.by_ref(), end)?;
                    hashes.insert(key, value);
                }
                Rule::block_param => {
                    block_param = Some(Template::parse_block_param(source, it.by_ref(), end));
                }
                Rule::pro_whitespace_omitter => {
                    omit_pro_ws = true;
                }
                _ => {}
            }
        }
        Ok(ExpressionSpec {
            name,
            params,
            hash: hashes,
            block_param,
            omit_pre_ws,
            omit_pro_ws,
        })
    }

    fn remove_previous_whitespace(template_stack: &mut VecDeque<Template>) {
        let t = template_stack.front_mut().unwrap();
        if let Some(RawString(ref mut text)) = t.elements.last_mut() {
            *text = text.trim_end().to_owned();
        }
    }

    // in handlebars, the whitespaces around statement are
    // automatically trimed.
    // this function checks if current span has both leading and
    // trailing whitespaces, which we treat as a standalone statement.
    //
    //
    fn process_standalone_statement(
        template_stack: &mut VecDeque<Template>,
        source: &str,
        current_span: &Span<'_>,
        prevent_indent: bool,
    ) -> bool {
        let with_trailing_newline =
            support::str::starts_with_empty_line(&source[current_span.end()..]);

        if with_trailing_newline {
            let with_leading_newline =
                support::str::ends_with_empty_line(&source[..current_span.start()]);

            // prevent_indent: a special toggle for partial expression
            // (>) that leading whitespaces are kept
            if prevent_indent && with_leading_newline {
                let t = template_stack.front_mut().unwrap();
                // check the last element before current
                if let Some(RawString(ref mut text)) = t.elements.last_mut() {
                    // trim leading space for standalone statement
                    *text = text
                        .trim_end_matches(support::str::whitespace_matcher)
                        .to_owned();
                }
            }

            // return true when the item is the first element in root template
            current_span.start() == 0 || with_leading_newline
        } else {
            false
        }
    }

    fn raw_string<'a>(
        source: &'a str,
        pair: Option<Pair<'a, Rule>>,
        trim_start: bool,
        trim_start_line: bool,
    ) -> TemplateElement {
        let mut s = String::from(source);

        if let Some(pair) = pair {
            // the source may contains leading space because of pest's limitation
            // we calculate none space start here in order to correct the offset
            let pair_span = pair.as_span();

            let current_start = pair_span.start();
            let span_length = pair_span.end() - current_start;
            let leading_space_offset = s.len() - span_length;

            // we would like to iterate pair reversely in order to remove certain
            // index from our string buffer so here we convert the inner pairs to
            // a vector.
            for sub_pair in pair.into_inner().rev() {
                // remove escaped backslash
                if sub_pair.as_rule() == Rule::escape {
                    let escape_span = sub_pair.as_span();

                    let backslash_pos = escape_span.start();
                    let backslash_rel_pos = leading_space_offset + backslash_pos - current_start;
                    s.remove(backslash_rel_pos);
                }
            }
        }

        if trim_start {
            RawString(s.trim_start().to_owned())
        } else if trim_start_line {
            let s = s.trim_start_matches(support::str::whitespace_matcher);
            RawString(support::str::strip_first_newline(s).to_owned())
        } else {
            RawString(s)
        }
    }

    pub(crate) fn compile2(
        source: &str,
        options: TemplateOptions,
    ) -> Result<Template, TemplateError> {
        let mut helper_stack: VecDeque<HelperTemplate> = VecDeque::new();
        let mut decorator_stack: VecDeque<DecoratorTemplate> = VecDeque::new();
        let mut template_stack: VecDeque<Template> = VecDeque::new();

        let mut omit_pro_ws = false;
        // flag for newline removal of standalone statements
        // this option is marked as true when standalone statement is detected
        // then the leading whitespaces and newline of next rawstring will be trimed
        let mut trim_line_required = false;

        let parser_queue = HandlebarsParser::parse(Rule::handlebars, source).map_err(|e| {
            let (line_no, col_no) = match e.line_col {
                LineColLocation::Pos(line_col) => line_col,
                LineColLocation::Span(line_col, _) => line_col,
            };
            TemplateError::of(TemplateErrorReason::InvalidSyntax)
                .at(source, line_no, col_no)
                .in_template(options.name())
        })?;

        // dbg!(parser_queue.clone().flatten());

        // remove escape from our pair queue
        let mut it = parser_queue
            .flatten()
            .filter(|p| {
                // remove rules that should be silent but not for now due to pest limitation
                !matches!(p.as_rule(), Rule::escape)
            })
            .peekable();
        let mut end_pos: Option<Position<'_>> = None;
        loop {
            if let Some(pair) = it.next() {
                let prev_end = end_pos.as_ref().map(|p| p.pos()).unwrap_or(0);
                let rule = pair.as_rule();
                let span = pair.as_span();

                let is_trailing_string = rule != Rule::template
                    && span.start() != prev_end
                    && !omit_pro_ws
                    && rule != Rule::raw_text
                    && rule != Rule::raw_block_text;

                if is_trailing_string {
                    // trailing string check
                    let (line_no, col_no) = span.start_pos().line_col();
                    if rule == Rule::raw_block_end {
                        let mut t = Template::new();
                        t.push_element(
                            Template::raw_string(
                                &source[prev_end..span.start()],
                                None,
                                false,
                                trim_line_required,
                            ),
                            line_no,
                            col_no,
                        );
                        template_stack.push_front(t);
                    } else {
                        let t = template_stack.front_mut().unwrap();
                        t.push_element(
                            Template::raw_string(
                                &source[prev_end..span.start()],
                                None,
                                false,
                                trim_line_required,
                            ),
                            line_no,
                            col_no,
                        );
                    }

                    // reset standalone statement marker
                    trim_line_required = false;
                }

                let (line_no, col_no) = span.start_pos().line_col();
                match rule {
                    Rule::template => {
                        template_stack.push_front(Template::new());
                    }
                    Rule::raw_text => {
                        // leading space fix
                        let start = if span.start() != prev_end {
                            prev_end
                        } else {
                            span.start()
                        };

                        let t = template_stack.front_mut().unwrap();
                        t.push_element(
                            Template::raw_string(
                                &source[start..span.end()],
                                Some(pair.clone()),
                                omit_pro_ws,
                                trim_line_required,
                            ),
                            line_no,
                            col_no,
                        );

                        // reset standalone statement marker
                        trim_line_required = false;
                    }
                    Rule::helper_block_start
                    | Rule::raw_block_start
                    | Rule::decorator_block_start
                    | Rule::partial_block_start => {
                        let exp = Template::parse_expression(source, it.by_ref(), span.end())?;

                        match rule {
                            Rule::helper_block_start | Rule::raw_block_start => {
                                let helper_template = HelperTemplate::new(exp.clone(), true);
                                helper_stack.push_front(helper_template);
                            }
                            Rule::decorator_block_start | Rule::partial_block_start => {
                                let decorator = DecoratorTemplate::new(exp.clone());
                                decorator_stack.push_front(decorator);
                            }
                            _ => unreachable!(),
                        }

                        if exp.omit_pre_ws {
                            Template::remove_previous_whitespace(&mut template_stack);
                        }
                        omit_pro_ws = exp.omit_pro_ws;

                        // standalone statement check, it also removes leading whitespaces of
                        // previous rawstring when standalone statement detected
                        trim_line_required = Template::process_standalone_statement(
                            &mut template_stack,
                            source,
                            &span,
                            true,
                        );

                        let t = template_stack.front_mut().unwrap();
                        t.mapping.push(TemplateMapping(line_no, col_no));
                    }
                    Rule::invert_tag => {
                        // hack: invert_tag structure is similar to ExpressionSpec, so I
                        // use it here to represent the data
                        let exp = Template::parse_expression(source, it.by_ref(), span.end())?;

                        if exp.omit_pre_ws {
                            Template::remove_previous_whitespace(&mut template_stack);
                        }
                        omit_pro_ws = exp.omit_pro_ws;

                        // standalone statement check, it also removes leading whitespaces of
                        // previous rawstring when standalone statement detected
                        trim_line_required = Template::process_standalone_statement(
                            &mut template_stack,
                            source,
                            &span,
                            true,
                        );

                        let t = template_stack.pop_front().unwrap();
                        let h = helper_stack.front_mut().unwrap();
                        h.template = Some(t);
                    }
                    Rule::raw_block_text => {
                        let mut t = Template::new();
                        t.push_element(
                            Template::raw_string(
                                span.as_str(),
                                Some(pair.clone()),
                                omit_pro_ws,
                                trim_line_required,
                            ),
                            line_no,
                            col_no,
                        );
                        template_stack.push_front(t);
                    }
                    Rule::expression
                    | Rule::html_expression
                    | Rule::decorator_expression
                    | Rule::partial_expression
                    | Rule::helper_block_end
                    | Rule::raw_block_end
                    | Rule::decorator_block_end
                    | Rule::partial_block_end => {
                        let exp = Template::parse_expression(source, it.by_ref(), span.end())?;

                        if exp.omit_pre_ws {
                            Template::remove_previous_whitespace(&mut template_stack);
                        }
                        omit_pro_ws = exp.omit_pro_ws;

                        match rule {
                            Rule::expression | Rule::html_expression => {
                                let helper_template = HelperTemplate::new(exp.clone(), false);
                                let el = if rule == Rule::expression {
                                    Expression(Box::new(helper_template))
                                } else {
                                    HtmlExpression(Box::new(helper_template))
                                };
                                let t = template_stack.front_mut().unwrap();
                                t.push_element(el, line_no, col_no);
                            }
                            Rule::decorator_expression | Rule::partial_expression => {
                                // do not auto trim ident spaces for
                                // partial_expression(>)
                                let prevent_indent = rule != Rule::partial_expression;
                                trim_line_required = Template::process_standalone_statement(
                                    &mut template_stack,
                                    source,
                                    &span,
                                    prevent_indent,
                                );

                                // indent for partial expression >
                                let mut indent = None;
                                if rule == Rule::partial_expression
                                    && !options.prevent_indent
                                    && !exp.omit_pre_ws
                                {
                                    indent = support::str::find_trailing_whitespace_chars(
                                        &source[..span.start()],
                                    );
                                }

                                let mut decorator = DecoratorTemplate::new(exp.clone());
                                decorator.indent = indent.map(|s| s.to_owned());

                                let el = if rule == Rule::decorator_expression {
                                    DecoratorExpression(Box::new(decorator))
                                } else {
                                    PartialExpression(Box::new(decorator))
                                };
                                let t = template_stack.front_mut().unwrap();
                                t.push_element(el, line_no, col_no);
                            }
                            Rule::helper_block_end | Rule::raw_block_end => {
                                // standalone statement check, it also removes leading whitespaces of
                                // previous rawstring when standalone statement detected
                                trim_line_required = Template::process_standalone_statement(
                                    &mut template_stack,
                                    source,
                                    &span,
                                    true,
                                );

                                let mut h = helper_stack.pop_front().unwrap();
                                let close_tag_name = exp.name.as_name();
                                if h.name.as_name() == close_tag_name {
                                    let prev_t = template_stack.pop_front().unwrap();
                                    if h.template.is_some() {
                                        h.inverse = Some(prev_t);
                                    } else {
                                        h.template = Some(prev_t);
                                    }
                                    let t = template_stack.front_mut().unwrap();
                                    t.elements.push(HelperBlock(Box::new(h)));
                                } else {
                                    return Err(TemplateError::of(
                                        TemplateErrorReason::MismatchingClosedHelper(
                                            h.name.debug_name(),
                                            exp.name.debug_name(),
                                        ),
                                    )
                                    .at(source, line_no, col_no)
                                    .in_template(options.name()));
                                }
                            }
                            Rule::decorator_block_end | Rule::partial_block_end => {
                                // standalone statement check, it also removes leading whitespaces of
                                // previous rawstring when standalone statement detected
                                trim_line_required = Template::process_standalone_statement(
                                    &mut template_stack,
                                    source,
                                    &span,
                                    true,
                                );

                                let mut d = decorator_stack.pop_front().unwrap();
                                let close_tag_name = exp.name.as_name();
                                if d.name.as_name() == close_tag_name {
                                    let prev_t = template_stack.pop_front().unwrap();
                                    d.template = Some(prev_t);
                                    let t = template_stack.front_mut().unwrap();
                                    if rule == Rule::decorator_block_end {
                                        t.elements.push(DecoratorBlock(Box::new(d)));
                                    } else {
                                        t.elements.push(PartialBlock(Box::new(d)));
                                    }
                                } else {
                                    return Err(TemplateError::of(
                                        TemplateErrorReason::MismatchingClosedDecorator(
                                            d.name.debug_name(),
                                            exp.name.debug_name(),
                                        ),
                                    )
                                    .at(source, line_no, col_no)
                                    .in_template(options.name()));
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    Rule::hbs_comment_compact => {
                        trim_line_required = Template::process_standalone_statement(
                            &mut template_stack,
                            source,
                            &span,
                            true,
                        );

                        let text = span
                            .as_str()
                            .trim_start_matches("{{!")
                            .trim_end_matches("}}");
                        let t = template_stack.front_mut().unwrap();
                        t.push_element(Comment(text.to_owned()), line_no, col_no);
                    }
                    Rule::hbs_comment => {
                        trim_line_required = Template::process_standalone_statement(
                            &mut template_stack,
                            source,
                            &span,
                            true,
                        );

                        let text = span
                            .as_str()
                            .trim_start_matches("{{!--")
                            .trim_end_matches("--}}");
                        let t = template_stack.front_mut().unwrap();
                        t.push_element(Comment(text.to_owned()), line_no, col_no);
                    }
                    _ => {}
                }

                if rule != Rule::template {
                    end_pos = Some(span.end_pos());
                }
            } else {
                let prev_end = end_pos.as_ref().map(|e| e.pos()).unwrap_or(0);
                if prev_end < source.len() {
                    let text = &source[prev_end..source.len()];
                    // is some called in if check
                    let (line_no, col_no) = end_pos.unwrap().line_col();
                    let t = template_stack.front_mut().unwrap();
                    t.push_element(RawString(text.to_owned()), line_no, col_no);
                }
                let mut root_template = template_stack.pop_front().unwrap();
                root_template.name = options.name;
                return Ok(root_template);
            }
        }
    }

    // These two compile functions are kept for compatibility with 4.x
    // Template APIs in case that some developers are using them
    // without registry.

    pub fn compile(source: &str) -> Result<Template, TemplateError> {
        Self::compile2(source, TemplateOptions::default())
    }

    pub fn compile_with_name<S: AsRef<str>>(
        source: S,
        name: String,
    ) -> Result<Template, TemplateError> {
        Self::compile2(
            source.as_ref(),
            TemplateOptions {
                name: Some(name),
                ..Default::default()
            },
        )
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TemplateElement {
    RawString(String),
    HtmlExpression(Box<HelperTemplate>),
    Expression(Box<HelperTemplate>),
    HelperBlock(Box<HelperTemplate>),
    DecoratorExpression(Box<DecoratorTemplate>),
    DecoratorBlock(Box<DecoratorTemplate>),
    PartialExpression(Box<DecoratorTemplate>),
    PartialBlock(Box<DecoratorTemplate>),
    Comment(String),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::TemplateErrorReason;

    #[test]
    fn test_parse_escaped_tag_raw_string() {
        let source = r"foo \{{bar}}";
        let t = Template::compile(source).ok().unwrap();
        assert_eq!(t.elements.len(), 1);
        assert_eq!(
            *t.elements.get(0).unwrap(),
            RawString("foo {{bar}}".to_string())
        );
    }

    #[test]
    fn test_pure_backslash_raw_string() {
        let source = r"\\\\";
        let t = Template::compile(source).ok().unwrap();
        assert_eq!(t.elements.len(), 1);
        assert_eq!(*t.elements.get(0).unwrap(), RawString(source.to_string()));
    }

    #[test]
    fn test_parse_escaped_block_raw_string() {
        let source = r"\{{{{foo}}}} bar";
        let t = Template::compile(source).ok().unwrap();
        assert_eq!(t.elements.len(), 1);
        assert_eq!(
            *t.elements.get(0).unwrap(),
            RawString("{{{{foo}}}} bar".to_string())
        );
    }

    #[test]
    fn test_parse_template() {
        let source = "<h1>{{title}} 你好</h1> {{{content}}}
{{#if date}}<p>good</p>{{else}}<p>bad</p>{{/if}}<img>{{foo bar}}中文你好
{{#unless true}}kitkat{{^}}lollipop{{/unless}}";
        let t = Template::compile(source).ok().unwrap();

        assert_eq!(t.elements.len(), 10);

        assert_eq!(*t.elements.get(0).unwrap(), RawString("<h1>".to_string()));
        assert_eq!(
            *t.elements.get(1).unwrap(),
            Expression(Box::new(HelperTemplate::with_path(Path::with_named_paths(
                &["title"]
            ))))
        );

        assert_eq!(
            *t.elements.get(3).unwrap(),
            HtmlExpression(Box::new(HelperTemplate::with_path(Path::with_named_paths(
                &["content"],
            ))))
        );

        match *t.elements.get(5).unwrap() {
            HelperBlock(ref h) => {
                assert_eq!(h.name.as_name().unwrap(), "if".to_string());
                assert_eq!(h.params.len(), 1);
                assert_eq!(h.template.as_ref().unwrap().elements.len(), 1);
            }
            _ => {
                panic!("Helper expected here.");
            }
        };

        match *t.elements.get(7).unwrap() {
            Expression(ref h) => {
                assert_eq!(h.name.as_name().unwrap(), "foo".to_string());
                assert_eq!(h.params.len(), 1);
                assert_eq!(
                    *(h.params.get(0).unwrap()),
                    Parameter::Path(Path::with_named_paths(&["bar"]))
                )
            }
            _ => {
                panic!("Helper expression here");
            }
        };

        match *t.elements.get(9).unwrap() {
            HelperBlock(ref h) => {
                assert_eq!(h.name.as_name().unwrap(), "unless".to_string());
                assert_eq!(h.params.len(), 1);
                assert_eq!(h.inverse.as_ref().unwrap().elements.len(), 1);
            }
            _ => {
                panic!("Helper expression here");
            }
        };
    }

    #[test]
    fn test_parse_block_partial_path_identifier() {
        let source = "{{#> foo/bar}}{{/foo/bar}}";
        assert!(Template::compile(source).is_ok());
    }

    #[test]
    fn test_parse_error() {
        let source = "{{#ifequals name compare=\"hello\"}}\nhello\n\t{{else}}\ngood";

        let terr = Template::compile(source).unwrap_err();

        assert!(matches!(terr.reason(), TemplateErrorReason::InvalidSyntax));
        assert_eq!(terr.line_no.unwrap(), 4);
        assert_eq!(terr.column_no.unwrap(), 5);
    }

    #[test]
    fn test_subexpression() {
        let source =
            "{{foo (bar)}}{{foo (bar baz)}} hello {{#if (baz bar) then=(bar)}}world{{/if}}";
        let t = Template::compile(source).ok().unwrap();

        assert_eq!(t.elements.len(), 4);
        match *t.elements.get(0).unwrap() {
            Expression(ref h) => {
                assert_eq!(h.name.as_name().unwrap(), "foo".to_owned());
                assert_eq!(h.params.len(), 1);
                if let &Parameter::Subexpression(ref t) = h.params.get(0).unwrap() {
                    assert_eq!(t.name(), "bar".to_owned());
                } else {
                    panic!("Subexpression expected");
                }
            }
            _ => {
                panic!("Helper expression expected");
            }
        };

        match *t.elements.get(1).unwrap() {
            Expression(ref h) => {
                assert_eq!(h.name.as_name().unwrap(), "foo".to_string());
                assert_eq!(h.params.len(), 1);
                if let &Parameter::Subexpression(ref t) = h.params.get(0).unwrap() {
                    assert_eq!(t.name(), "bar".to_owned());
                    if let Some(Parameter::Path(p)) = t.params().unwrap().get(0) {
                        assert_eq!(p, &Path::with_named_paths(&["baz"]));
                    } else {
                        panic!("non-empty param expected ");
                    }
                } else {
                    panic!("Subexpression expected");
                }
            }
            _ => {
                panic!("Helper expression expected");
            }
        };

        match *t.elements.get(3).unwrap() {
            HelperBlock(ref h) => {
                assert_eq!(h.name.as_name().unwrap(), "if".to_string());
                assert_eq!(h.params.len(), 1);
                assert_eq!(h.hash.len(), 1);

                if let &Parameter::Subexpression(ref t) = h.params.get(0).unwrap() {
                    assert_eq!(t.name(), "baz".to_owned());
                    if let Some(Parameter::Path(p)) = t.params().unwrap().get(0) {
                        assert_eq!(p, &Path::with_named_paths(&["bar"]));
                    } else {
                        panic!("non-empty param expected ");
                    }
                } else {
                    panic!("Subexpression expected (baz bar)");
                }

                if let &Parameter::Subexpression(ref t) = h.hash.get("then").unwrap() {
                    assert_eq!(t.name(), "bar".to_owned());
                } else {
                    panic!("Subexpression expected (bar)");
                }
            }
            _ => {
                panic!("HelperBlock expected");
            }
        }
    }

    #[test]
    fn test_white_space_omitter() {
        let source = "hello~     {{~world~}} \n  !{{~#if true}}else{{/if~}}";
        let t = Template::compile(source).ok().unwrap();

        assert_eq!(t.elements.len(), 4);

        assert_eq!(t.elements[0], RawString("hello~".to_string()));
        assert_eq!(
            t.elements[1],
            Expression(Box::new(HelperTemplate::with_path(Path::with_named_paths(
                &["world"]
            ))))
        );
        assert_eq!(t.elements[2], RawString("!".to_string()));

        let t2 = Template::compile("{{#if true}}1  {{~ else ~}} 2 {{~/if}}")
            .ok()
            .unwrap();
        assert_eq!(t2.elements.len(), 1);
        match t2.elements[0] {
            HelperBlock(ref h) => {
                assert_eq!(
                    h.template.as_ref().unwrap().elements[0],
                    RawString("1".to_string())
                );
                assert_eq!(
                    h.inverse.as_ref().unwrap().elements[0],
                    RawString("2".to_string())
                );
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_unclosed_expression() {
        let sources = ["{{invalid", "{{{invalid", "{{invalid}", "{{!hello"];
        for s in sources.iter() {
            let result = Template::compile(s.to_owned());
            assert!(matches!(
                *result.unwrap_err().reason(),
                TemplateErrorReason::InvalidSyntax
            ));
        }
    }

    #[test]
    fn test_raw_helper() {
        let source = "hello{{{{raw}}}}good{{night}}{{{{/raw}}}}world";
        match Template::compile(source) {
            Ok(t) => {
                assert_eq!(t.elements.len(), 3);
                assert_eq!(t.elements[0], RawString("hello".to_owned()));
                assert_eq!(t.elements[2], RawString("world".to_owned()));
                match t.elements[1] {
                    HelperBlock(ref h) => {
                        assert_eq!(h.name.as_name().unwrap(), "raw".to_owned());
                        if let Some(ref ht) = h.template {
                            assert_eq!(ht.elements.len(), 1);
                            assert_eq!(
                                *ht.elements.get(0).unwrap(),
                                RawString("good{{night}}".to_owned())
                            );
                        } else {
                            panic!("helper template not found");
                        }
                    }
                    _ => {
                        panic!("Unexpected element type");
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_literal_parameter_parser() {
        match Template::compile("{{hello 1 name=\"value\" valid=false ref=someref}}") {
            Ok(t) => {
                if let Expression(ref ht) = t.elements[0] {
                    assert_eq!(ht.params[0], Parameter::Literal(json!(1)));
                    assert_eq!(
                        ht.hash["name"],
                        Parameter::Literal(Json::String("value".to_owned()))
                    );
                    assert_eq!(ht.hash["valid"], Parameter::Literal(Json::Bool(false)));
                    assert_eq!(
                        ht.hash["ref"],
                        Parameter::Path(Path::with_named_paths(&["someref"]))
                    );
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_template_mapping() {
        match Template::compile("hello\n  {{~world}}\n{{#if nice}}\n\thello\n{{/if}}") {
            Ok(t) => {
                assert_eq!(t.mapping.len(), t.elements.len());
                assert_eq!(t.mapping[0], TemplateMapping(1, 1));
                assert_eq!(t.mapping[1], TemplateMapping(2, 3));
                assert_eq!(t.mapping[3], TemplateMapping(3, 1));
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_whitespace_elements() {
        let c = Template::compile(
            "  {{elem}}\n\t{{#if true}} \
         {{/if}}\n{{{{raw}}}} {{{{/raw}}}}\n{{{{raw}}}}{{{{/raw}}}}\n",
        );
        let r = c.unwrap();
        // the \n after last raw block is dropped by pest
        assert_eq!(r.elements.len(), 9);
    }

    #[test]
    fn test_block_param() {
        match Template::compile("{{#each people as |person|}}{{person}}{{/each}}") {
            Ok(t) => {
                if let HelperBlock(ref ht) = t.elements[0] {
                    if let Some(BlockParam::Single(Parameter::Name(ref n))) = ht.block_param {
                        assert_eq!(n, "person");
                    } else {
                        panic!("block param expected.")
                    }
                } else {
                    panic!("Helper block expected");
                }
            }
            Err(e) => panic!("{}", e),
        }

        match Template::compile("{{#each people as |val key|}}{{person}}{{/each}}") {
            Ok(t) => {
                if let HelperBlock(ref ht) = t.elements[0] {
                    if let Some(BlockParam::Pair((
                        Parameter::Name(ref n1),
                        Parameter::Name(ref n2),
                    ))) = ht.block_param
                    {
                        assert_eq!(n1, "val");
                        assert_eq!(n2, "key");
                    } else {
                        panic!("helper block param expected.");
                    }
                } else {
                    panic!("Helper block expected");
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn test_decorator() {
        match Template::compile("hello {{* ssh}} world") {
            Err(e) => panic!("{}", e),
            Ok(t) => {
                if let DecoratorExpression(ref de) = t.elements[1] {
                    assert_eq!(de.name.as_name(), Some("ssh"));
                    assert_eq!(de.template, None);
                }
            }
        }

        match Template::compile("hello {{> ssh}} world") {
            Err(e) => panic!("{}", e),
            Ok(t) => {
                if let PartialExpression(ref de) = t.elements[1] {
                    assert_eq!(de.name.as_name(), Some("ssh"));
                    assert_eq!(de.template, None);
                }
            }
        }

        match Template::compile("{{#*inline \"hello\"}}expand to hello{{/inline}}{{> hello}}") {
            Err(e) => panic!("{}", e),
            Ok(t) => {
                if let DecoratorBlock(ref db) = t.elements[0] {
                    assert_eq!(db.name, Parameter::Name("inline".to_owned()));
                    assert_eq!(
                        db.params[0],
                        Parameter::Literal(Json::String("hello".to_owned()))
                    );
                    assert_eq!(
                        db.template.as_ref().unwrap().elements[0],
                        TemplateElement::RawString("expand to hello".to_owned())
                    );
                }
            }
        }

        match Template::compile("{{#> layout \"hello\"}}expand to hello{{/layout}}{{> hello}}") {
            Err(e) => panic!("{}", e),
            Ok(t) => {
                if let PartialBlock(ref db) = t.elements[0] {
                    assert_eq!(db.name, Parameter::Name("layout".to_owned()));
                    assert_eq!(
                        db.params[0],
                        Parameter::Literal(Json::String("hello".to_owned()))
                    );
                    assert_eq!(
                        db.template.as_ref().unwrap().elements[0],
                        TemplateElement::RawString("expand to hello".to_owned())
                    );
                }
            }
        }
    }

    #[test]
    fn test_panic_with_tag_name() {
        let s = "{{#>(X)}}{{/X}}";
        let result = Template::compile(s);
        assert!(result.is_err());
        assert_eq!("decorator \"Subexpression(Subexpression { element: Expression(HelperTemplate { name: Path(Relative(([Named(\\\"X\\\")], \\\"X\\\"))), params: [], hash: {}, block_param: None, template: None, inverse: None, block: false }) })\" was opened, but \"X\" is closing", format!("{}", result.unwrap_err().reason()));
    }
}
