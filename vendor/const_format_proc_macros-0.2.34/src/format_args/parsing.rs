use super::{
    ExpandFormatted, ExpandInto, ExpandWithFormatter, FormatArg, FormatArgs, FormatIfArgs,
    LocalVariable, UncheckedFormatArg, UncheckedFormatArgs, WriteArgs,
};

use crate::{
    format_str::{FmtArg, FmtStrComponent, FormatStr, WhichArg},
    parse_utils::{LitStr, MyParse, ParseBuffer, ParseStream, TokenTreeExt},
    shared_arg_parsing::ExprArg,
    spanned::Spans,
    utils::{dummy_ident, LinearResult},
};

use proc_macro2::{Ident, Span, TokenTree};

////////////////////////////////////////////////

impl MyParse for UncheckedFormatArg {
    fn parse(input: ParseStream<'_>) -> Result<Self, crate::Error> {
        // the compile wraps `:expr` in macro_rules macros in a TokenStream::Group
        // with no delimiters.
        input.parse_unwrap_group(|content| {
            let mut ident = None;
            if matches!(content.peek2(), Some(x) if x.is_punct('=')) {
                ident = Some(content.parse_ident()?);
                content.next();
            }

            // For some reason,
            // the compile wraps closures in parentheses when passing them as
            // expressions to proc macros.
            content.parse_unwrap_paren(|content| {
                let mut fmt_ident = None;

                if matches!(content.peek(), Some(x) if x.is_punct('|'))
                    && matches!(content.peek2(), Some(TokenTree::Ident(_)))
                {
                    content.next();
                    fmt_ident = Some(content.parse_ident()?);
                    content.parse_punct('|')?;
                }

                let (expr, spans) = if content.peek2().is_some() {
                    content.parse_token_stream_and_span()
                } else {
                    content.parse_unwrap_tt(|content| Ok(content.parse_token_stream_and_span()))?
                };

                Ok(Self {
                    spans,
                    ident,
                    fmt_ident,
                    expr,
                })
            })
        })
    }
}

////////////////////////////////////////////////

fn lit_str_to_fmt_lit(lit: &LitStr) -> Result<FormatStr, crate::Error> {
    let lit_str = lit.value();
    let format_str_span = lit.span;
    FormatStr::parse(lit.value(), lit.rawness)
        .map_err(|e| e.into_crate_err(format_str_span, lit_str))
}

fn parse_fmt_lit(this: &mut FormatStr, input: ParseStream<'_>) -> Result<(), crate::Error> {
    input.parse_unwrap_tt(|input| {
        let tt = input.next();

        match tt {
            Some(TokenTree::Literal(lit)) => {
                let mut lit = lit_str_to_fmt_lit(&LitStr::parse_from_literal(&lit)?)?;

                this.list.append(&mut lit.list);

                Ok(())
            }
            Some(TokenTree::Ident(ident)) if ident == "concat" => {
                input.next(); // skipping the `!`
                let paren = input.parse_paren()?;
                let mut input = ParseBuffer::new(paren.contents);

                while !input.is_empty() {
                    parse_fmt_lit(this, &mut input)?;
                    input.parse_opt_punct(',')?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    })
}

impl MyParse for UncheckedFormatArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self, crate::Error> {
        let mut literal = FormatStr { list: Vec::new() };

        // Have to parse `concat!()` because it's not expanded before the proc macro is called.
        {
            let paren = input.parse_paren()?;
            let mut input = ParseBuffer::new(paren.contents);

            parse_fmt_lit(&mut literal, &mut input)?;
        }

        input.parse_opt_punct(',')?;

        let mut args = Vec::new();

        while !input.is_empty() {
            args.push(UncheckedFormatArg::parse(input)?);

            input.parse_opt_punct(',')?;
        }

        Ok(Self { literal, args })
    }
}

////////////////////////////////////////////////

impl MyParse for FormatArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self, crate::Error> {
        let prefix = Ident::new("__const_fmt_local_", Span::call_site());
        FormatArgs::parse_with(input, prefix)
    }
}

impl FormatArgs {
    pub fn parse_with(input: ParseStream<'_>, prefix: Ident) -> Result<FormatArgs, crate::Error> {
        let mut res = LinearResult::ok();

        let unchecked_fargs = UncheckedFormatArgs::parse(input)?;

        let mut first_named_arg = unchecked_fargs.args.len();

        let mut named_arg_names = Vec::<Ident>::new();
        let mut args = Vec::<FormatArg>::with_capacity(unchecked_fargs.args.len());
        let mut local_variables = Vec::<LocalVariable>::with_capacity(unchecked_fargs.args.len());

        let arg_span_idents: Vec<(Spans, Option<Ident>)> = unchecked_fargs
            .args
            .iter()
            .map(|x| (x.spans, x.ident.clone()))
            .collect();

        {
            let mut prev_is_named_arg = false;
            for (i, arg) in unchecked_fargs.args.into_iter().enumerate() {
                let expr_span = arg.spans;

                let make_ident = |s: String| Ident::new(&s, expr_span.start);

                let is_named_arg = arg.ident.is_some();

                let var_name = if let Some(ident) = arg.ident {
                    if !prev_is_named_arg {
                        first_named_arg = i;
                    }

                    let name = make_ident(format!("{}{}", prefix, ident));
                    named_arg_names.push(ident);
                    name
                } else {
                    if prev_is_named_arg {
                        return Err(crate::Error::spanned(
                            arg.spans,
                            "expected a named argument, \
                             named arguments cannot be followed by positional arguments.",
                        ));
                    }

                    make_ident(format!("{}{}", prefix, i))
                };

                let format_arg = if let Some(fmt_ident) = &arg.fmt_ident {
                    FormatArg::WithFormatter {
                        fmt_ident: fmt_ident.clone(),
                        expr: arg.expr.clone(),
                    }
                } else {
                    local_variables.push(LocalVariable {
                        ident: var_name.clone(),
                        expr: arg.expr.clone(),
                    });

                    FormatArg::WithLocal(var_name)
                };

                args.push(format_arg);

                prev_is_named_arg = is_named_arg;
            }
        }

        let mut unused_args = vec![true; args.len()];

        let first_named_arg = first_named_arg;
        let named_arg_names = named_arg_names;
        let args = args;

        let positional_args = &args[..first_named_arg];
        let named_args = &args[first_named_arg..];

        let fmt_str_components = unchecked_fargs.literal.list;

        let expanded_into: Vec<ExpandInto> = {
            let mut current_pos_arg = 0;
            let mut get_variable_name = |param: FmtArg| -> ExpandInto {
                let FmtArg {
                    which_arg,
                    formatting,
                    rawness,
                } = param;

                let arg = match which_arg {
                    WhichArg::Ident(ident) => {
                        if let Some(pos) = named_arg_names.iter().position(|x| *x == ident) {
                            unused_args[pos + first_named_arg] = false;
                            &named_args[pos]
                        } else {
                            // `formatcp!("{FOO}")` assumes that FOO is a constant in scope
                            return ExpandInto::Formatted(ExpandFormatted {
                                local_variable: Ident::new(&ident, rawness.span()),
                                format: formatting,
                            });
                        }
                    }
                    WhichArg::Positional(opt_pos) => {
                        let pos = opt_pos.unwrap_or_else(|| {
                            let pos = current_pos_arg;
                            current_pos_arg += 1;
                            pos
                        });

                        match positional_args.get(pos) {
                            Some(arg) => {
                                unused_args[pos] = false;
                                arg
                            }
                            None => {
                                res.push_err(crate::Error::new(
                                    rawness.span(),
                                    format!(
                                        "attempting to use nonexistent  positional argument `{}`",
                                        pos,
                                    ),
                                ));
                                return ExpandInto::Formatted(ExpandFormatted {
                                    local_variable: dummy_ident(),
                                    format: formatting,
                                });
                            }
                        }
                    }
                };

                match arg {
                    FormatArg::WithFormatter { fmt_ident, expr } => {
                        ExpandInto::WithFormatter(ExpandWithFormatter {
                            format: formatting,
                            fmt_ident: fmt_ident.clone(),
                            expr: expr.clone(),
                        })
                    }
                    FormatArg::WithLocal(local_variable) => {
                        ExpandInto::Formatted(ExpandFormatted {
                            format: formatting,
                            local_variable: local_variable.clone(),
                        })
                    }
                }
            };

            fmt_str_components
                .into_iter()
                .map(|fmt_str_comp| match fmt_str_comp {
                    FmtStrComponent::Str(str, str_rawness) => ExpandInto::Str(str, str_rawness),
                    FmtStrComponent::Arg(arg) => get_variable_name(arg),
                })
                .collect()
        };

        for (i, (is_it_unused, (spans, ident))) in
            unused_args.iter().zip(&arg_span_idents).enumerate()
        {
            if *is_it_unused {
                let msg = if let Some(ident) = ident {
                    format!("the '{}' argument is unused", ident)
                } else {
                    format!("argument number {} is unused", i)
                };
                res.push_err(crate::Error::spanned(*spans, msg));
            }
        }
        res.take()?;

        Ok(FormatArgs {
            condition: None,
            local_variables,
            expanded_into,
        })
    }
}

////////////////////////////////////////////////

impl MyParse for FormatIfArgs {
    fn parse(input: ParseStream) -> Result<Self, crate::Error> {
        let condition = ExprArg::parse(input)?;

        let mut inner = FormatArgs::parse(input)?;
        inner.condition = Some(condition);

        Ok(Self { inner })
    }
}

////////////////////////////////////////////////

impl MyParse for WriteArgs {
    fn parse(input: ParseStream) -> Result<Self, crate::Error> {
        let prefix = Ident::new("__const_fmt_local_", Span::call_site());

        let paren = input.parse_paren()?;

        let mut content = ParseBuffer::new(paren.contents);

        let (writer_expr, spans) =
            content.parse_unwrap_tt(|content| Ok(content.parse_token_stream_and_span()))?;

        let format_args = FormatArgs::parse_with(input, prefix)?;

        Ok(Self {
            writer_expr,
            writer_span: spans.joined(),
            format_args,
        })
    }
}
