//! This module implements the bytecode interpreter that actually renders the templates.

use compiler::TemplateCompiler;
use error::Error::*;
use error::*;
use instruction::{Instruction, PathSlice, PathStep};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Write;
use std::slice;
use ValueFormatter;

/// Enum defining the different kinds of records on the context stack.
enum ContextElement<'render, 'template> {
    /// Object contexts shadow everything below them on the stack, because every name is looked up
    /// in this object.
    Object(&'render Value),
    /// Named contexts shadow only one name. Any path that starts with that name is looked up in
    /// this object, and all others are passed on down the stack.
    Named(&'template str, &'render Value),
    /// Iteration contexts shadow one name with the current value of the iteration. They also
    /// store the iteration state. The two usizes are the index of the current value and the length
    /// of the array that we're iterating over.
    Iteration(
        &'template str,
        &'render Value,
        usize,
        usize,
        slice::Iter<'render, Value>,
    ),
}

/// Helper struct which mostly exists so that I have somewhere to put functions that access the
/// rendering context stack.
struct RenderContext<'render, 'template> {
    original_text: &'template str,
    context_stack: Vec<ContextElement<'render, 'template>>,
}
impl<'render, 'template> RenderContext<'render, 'template> {
    /// Look up the given path in the context stack and return the value (if found) or an error (if
    /// not)
    fn lookup(&self, path: PathSlice) -> Result<&'render Value> {
        for stack_layer in self.context_stack.iter().rev() {
            match stack_layer {
                ContextElement::Object(obj) => return self.lookup_in(path, obj),
                ContextElement::Named(name, obj) => {
                    if *name == &*path[0] {
                        return self.lookup_in(&path[1..], obj);
                    }
                }
                ContextElement::Iteration(name, obj, _, _, _) => {
                    if *name == &*path[0] {
                        return self.lookup_in(&path[1..], obj);
                    }
                }
            }
        }
        panic!("Attempted to do a lookup with an empty context stack. That shouldn't be possible.")
    }

    /// Look up a path within a given value object and return the resulting value (if found) or
    /// an error (if not)
    fn lookup_in(&self, path: PathSlice, object: &'render Value) -> Result<&'render Value> {
        let mut current = object;
        for step in path.iter() {
            if let PathStep::Index(_, n) = step {
                if let Some(next) = current.get(n) {
                    current = next;
                    continue;
                }
            }

            let step: &str = &*step;

            match current.get(step) {
                Some(next) => current = next,
                None => return Err(lookup_error(self.original_text, step, path, current)),
            }
        }
        Ok(current)
    }

    /// Look up the index and length values for the top iteration context on the stack.
    fn lookup_index(&self) -> Result<(usize, usize)> {
        for stack_layer in self.context_stack.iter().rev() {
            match stack_layer {
                ContextElement::Iteration(_, _, index, length, _) => return Ok((*index, *length)),
                _ => continue,
            }
        }
        Err(GenericError {
            msg: "Used @index outside of a foreach block.".to_string(),
        })
    }

    /// Look up the root context object
    fn lookup_root(&self) -> Result<&'render Value> {
        match self.context_stack.get(0) {
            Some(ContextElement::Object(obj)) => Ok(obj),
            Some(_) => {
                panic!("Expected Object value at root of context stack, but was something else.")
            }
            None => panic!(
                "Attempted to do a lookup with an empty context stack. That shouldn't be possible."
            ),
        }
    }
}

/// Structure representing a parsed template. It holds the bytecode program for rendering the
/// template as well as the length of the original template string, which is used as a guess to
/// pre-size the output string buffer.
pub(crate) struct Template<'template> {
    original_text: &'template str,
    instructions: Vec<Instruction<'template>>,
    template_len: usize,
}
impl<'template> Template<'template> {
    /// Create a Template from the given template string.
    pub fn compile(text: &'template str) -> Result<Template> {
        Ok(Template {
            original_text: text,
            template_len: text.len(),
            instructions: TemplateCompiler::new(text).compile()?,
        })
    }

    /// Render this template into a string and return it (or any error if one is encountered).
    pub fn render(
        &self,
        context: &Value,
        template_registry: &HashMap<&str, Template>,
        formatter_registry: &HashMap<&str, Box<ValueFormatter>>,
        default_formatter: &ValueFormatter,
    ) -> Result<String> {
        // The length of the original template seems like a reasonable guess at the length of the
        // output.
        let mut output = String::with_capacity(self.template_len);
        self.render_into(
            context,
            template_registry,
            formatter_registry,
            default_formatter,
            &mut output,
        )?;
        Ok(output)
    }

    /// Render this template into a given string. Used for calling other templates.
    pub fn render_into(
        &self,
        context: &Value,
        template_registry: &HashMap<&str, Template>,
        formatter_registry: &HashMap<&str, Box<ValueFormatter>>,
        default_formatter: &ValueFormatter,
        output: &mut String,
    ) -> Result<()> {
        let mut program_counter = 0;
        let mut render_context = RenderContext {
            original_text: self.original_text,
            context_stack: vec![ContextElement::Object(context)],
        };

        while program_counter < self.instructions.len() {
            match &self.instructions[program_counter] {
                Instruction::Literal(text) => {
                    output.push_str(text);
                    program_counter += 1;
                }
                Instruction::Value(path) => {
                    let first = path.first().unwrap();
                    if first.starts_with('@') {
                        // Currently we just hard-code the special @-keywords and have special
                        // lookup functions to use them because there are lifetime complexities with
                        // looking up values that don't live for as long as the given context object.
                        let first: &str = &*first;
                        match first {
                            "@index" => {
                                write!(output, "{}", render_context.lookup_index()?.0).unwrap()
                            }
                            "@first" => {
                                write!(output, "{}", render_context.lookup_index()?.0 == 0).unwrap()
                            }
                            "@last" => {
                                let (index, length) = render_context.lookup_index()?;
                                write!(output, "{}", index == length - 1).unwrap()
                            }
                            "@root" => {
                                let value_to_render = render_context.lookup_root()?;
                                default_formatter(value_to_render, output)?;
                            }
                            _ => panic!(), // This should have been caught by the parser.
                        }
                    } else {
                        let value_to_render = render_context.lookup(path)?;
                        default_formatter(value_to_render, output)?;
                    }
                    program_counter += 1;
                }
                Instruction::FormattedValue(path, name) => {
                    // The @ keywords aren't supported for formatted values. Should they be?
                    let value_to_render = render_context.lookup(path)?;
                    match formatter_registry.get(name) {
                        Some(formatter) => {
                            let formatter_result = formatter(value_to_render, output);
                            if let Err(err) = formatter_result {
                                return Err(called_formatter_error(self.original_text, name, err));
                            }
                        }
                        None => return Err(unknown_formatter(self.original_text, name)),
                    }
                    program_counter += 1;
                }
                Instruction::Branch(path, negate, target) => {
                    let first = path.first().unwrap();
                    let mut truthy = if first.starts_with('@') {
                        let first: &str = &*first;
                        match &*first {
                            "@index" => render_context.lookup_index()?.0 != 0,
                            "@first" => render_context.lookup_index()?.0 == 0,
                            "@last" => {
                                let (index, length) = render_context.lookup_index()?;
                                index == (length - 1)
                            }
                            "@root" => self.value_is_truthy(render_context.lookup_root()?, path)?,
                            other => panic!("Unknown keyword {}", other), // This should have been caught by the parser.
                        }
                    } else {
                        let value_to_render = render_context.lookup(path)?;
                        self.value_is_truthy(value_to_render, path)?
                    };
                    if *negate {
                        truthy = !truthy;
                    }

                    if truthy {
                        program_counter = *target;
                    } else {
                        program_counter += 1;
                    }
                }
                Instruction::PushNamedContext(path, name) => {
                    let context_value = render_context.lookup(path)?;
                    render_context
                        .context_stack
                        .push(ContextElement::Named(name, context_value));
                    program_counter += 1;
                }
                Instruction::PushIterationContext(path, name) => {
                    // We push a context with an invalid index and no value and then wait for the
                    // following Iterate instruction to set the index and value properly.
                    let first = path.first().unwrap();
                    let context_value = match first {
                        PathStep::Name("@root") => render_context.lookup_root()?,
                        PathStep::Name(other) if other.starts_with('@') => {
                            return Err(not_iterable_error(self.original_text, path))
                        }
                        _ => render_context.lookup(path)?,
                    };
                    match context_value {
                        Value::Array(ref arr) => {
                            render_context.context_stack.push(ContextElement::Iteration(
                                name,
                                &Value::Null,
                                ::std::usize::MAX,
                                arr.len(),
                                arr.iter(),
                            ))
                        }
                        _ => return Err(not_iterable_error(self.original_text, path)),
                    };
                    program_counter += 1;
                }
                Instruction::PopContext => {
                    render_context.context_stack.pop();
                    program_counter += 1;
                }
                Instruction::Goto(target) => {
                    program_counter = *target;
                }
                Instruction::Iterate(target) => {
                    match render_context.context_stack.last_mut() {
                        Some(ContextElement::Iteration(_, val, index, _, iter)) => {
                            match iter.next() {
                                Some(new_val) => {
                                    *val = new_val;
                                    // On the first iteration, this will be usize::MAX so it will
                                    // wrap around to zero.
                                    *index = index.wrapping_add(1);
                                    program_counter += 1;
                                }
                                None => {
                                    program_counter = *target;
                                }
                            }
                        }
                        _ => panic!("Malformed program."),
                    };
                }
                Instruction::Call(template_name, path) => {
                    let context_value = render_context.lookup(path)?;
                    match template_registry.get(template_name) {
                        Some(templ) => {
                            let called_templ_result = templ.render_into(
                                context_value,
                                template_registry,
                                formatter_registry,
                                default_formatter,
                                output,
                            );
                            if let Err(err) = called_templ_result {
                                return Err(called_template_error(
                                    self.original_text,
                                    template_name,
                                    err,
                                ));
                            }
                        }
                        None => return Err(unknown_template(self.original_text, template_name)),
                    }
                    program_counter += 1;
                }
            }
        }
        Ok(())
    }

    fn value_is_truthy(&self, value: &Value, path: PathSlice) -> Result<bool> {
        let truthy = match value {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => match n.as_f64() {
                Some(float) => float != 0.0,
                None => {
                    return Err(truthiness_error(self.original_text, path));
                }
            },
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(_) => true,
        };
        Ok(truthy)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use compiler::TemplateCompiler;

    fn compile(text: &'static str) -> Template<'static> {
        Template {
            original_text: text,
            template_len: text.len(),
            instructions: TemplateCompiler::new(text).compile().unwrap(),
        }
    }

    #[derive(Serialize)]
    struct NestedContext {
        value: usize,
    }

    #[derive(Serialize)]
    struct TestContext {
        number: usize,
        string: &'static str,
        boolean: bool,
        null: Option<usize>,
        array: Vec<usize>,
        nested: NestedContext,
        escapes: &'static str,
    }

    fn context() -> Value {
        let ctx = TestContext {
            number: 5,
            string: "test",
            boolean: true,
            null: None,
            array: vec![1, 2, 3],
            nested: NestedContext { value: 10 },
            escapes: "1:< 2:> 3:& 4:' 5:\"",
        };
        ::serde_json::to_value(&ctx).unwrap()
    }

    fn other_templates() -> HashMap<&'static str, Template<'static>> {
        let mut map = HashMap::new();
        map.insert("my_macro", compile("{value}"));
        map
    }

    fn format(value: &Value, output: &mut String) -> Result<()> {
        output.push_str("{");
        ::format(value, output)?;
        output.push_str("}");
        Ok(())
    }

    fn formatters() -> HashMap<&'static str, Box<ValueFormatter>> {
        let mut map = HashMap::<&'static str, Box<ValueFormatter>>::new();
        map.insert("my_formatter", Box::new(format));
        map
    }

    pub fn default_formatter() -> &'static ValueFormatter {
        &::format
    }

    #[test]
    fn test_literal() {
        let template = compile("Hello!");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Hello!", &string);
    }

    #[test]
    fn test_value() {
        let template = compile("{ number }");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("5", &string);
    }

    #[test]
    fn test_path() {
        let template = compile("The number of the day is { nested.value }.");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("The number of the day is 10.", &string);
    }

    #[test]
    fn test_if_taken() {
        let template = compile("{{ if boolean }}Hello!{{ endif }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Hello!", &string);
    }

    #[test]
    fn test_if_untaken() {
        let template = compile("{{ if null }}Hello!{{ endif }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("", &string);
    }

    #[test]
    fn test_if_else_taken() {
        let template = compile("{{ if boolean }}Hello!{{ else }}Goodbye!{{ endif }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Hello!", &string);
    }

    #[test]
    fn test_if_else_untaken() {
        let template = compile("{{ if null }}Hello!{{ else }}Goodbye!{{ endif }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Goodbye!", &string);
    }

    #[test]
    fn test_ifnot_taken() {
        let template = compile("{{ if not boolean }}Hello!{{ endif }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("", &string);
    }

    #[test]
    fn test_ifnot_untaken() {
        let template = compile("{{ if not null }}Hello!{{ endif }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Hello!", &string);
    }

    #[test]
    fn test_ifnot_else_taken() {
        let template = compile("{{ if not boolean }}Hello!{{ else }}Goodbye!{{ endif }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Goodbye!", &string);
    }

    #[test]
    fn test_ifnot_else_untaken() {
        let template = compile("{{ if not null }}Hello!{{ else }}Goodbye!{{ endif }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Hello!", &string);
    }

    #[test]
    fn test_nested_ifs() {
        let template = compile(
            "{{ if boolean }}Hi, {{ if null }}there!{{ else }}Hello!{{ endif }}{{ endif }}",
        );
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Hi, Hello!", &string);
    }

    #[test]
    fn test_with() {
        let template = compile("{{ with nested as n }}{ n.value } { number }{{endwith}}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("10 5", &string);
    }

    #[test]
    fn test_for_loop() {
        let template = compile("{{ for a in array }}{ a }{{ endfor }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("123", &string);
    }

    #[test]
    fn test_for_loop_index() {
        let template = compile("{{ for a in array }}{ @index }{{ endfor }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("012", &string);
    }

    #[test]
    fn test_for_loop_first() {
        let template =
            compile("{{ for a in array }}{{if @first }}{ @index }{{ endif }}{{ endfor }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("0", &string);
    }

    #[test]
    fn test_for_loop_last() {
        let template =
            compile("{{ for a in array }}{{ if @last}}{ @index }{{ endif }}{{ endfor }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("2", &string);
    }

    #[test]
    fn test_whitespace_stripping_value() {
        let template = compile("1  \n\t   {- number -}  \n   1");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("151", &string);
    }

    #[test]
    fn test_call() {
        let template = compile("{{ call my_macro with nested }}");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("10", &string);
    }

    #[test]
    fn test_formatter() {
        let template = compile("{ nested.value | my_formatter }");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("{10}", &string);
    }

    #[test]
    fn test_unknown() {
        let template = compile("{ foobar }");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap_err();
    }

    #[test]
    fn test_escaping() {
        let template = compile("{ escapes }");
        let context = context();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("1:&lt; 2:&gt; 3:&amp; 4:&#39; 5:&quot;", &string);
    }

    #[test]
    fn test_unescaped() {
        let template = compile("{ escapes | unescaped }");
        let context = context();
        let template_registry = other_templates();
        let mut formatter_registry = formatters();
        formatter_registry.insert("unescaped", Box::new(::format_unescaped));
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("1:< 2:> 3:& 4:' 5:\"", &string);
    }

    #[test]
    fn test_root_print() {
        let template = compile("{ @root }");
        let context = "Hello World!";
        let context = ::serde_json::to_value(&context).unwrap();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Hello World!", &string);
    }

    #[test]
    fn test_root_branch() {
        let template = compile("{{ if @root }}Hello World!{{ endif }}");
        let context = true;
        let context = ::serde_json::to_value(&context).unwrap();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("Hello World!", &string);
    }

    #[test]
    fn test_root_iterate() {
        let template = compile("{{ for a in @root }}{ a }{{ endfor }}");
        let context = vec!["foo", "bar"];
        let context = ::serde_json::to_value(&context).unwrap();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("foobar", &string);
    }

    #[test]
    fn test_number_truthiness_zero() {
        let template = compile("{{ if @root }}truthy{{else}}not truthy{{ endif }}");
        let context = 0;
        let context = ::serde_json::to_value(&context).unwrap();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("not truthy", &string);
    }

    #[test]
    fn test_number_truthiness_one() {
        let template = compile("{{ if @root }}truthy{{else}}not truthy{{ endif }}");
        let context = 1;
        let context = ::serde_json::to_value(&context).unwrap();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("truthy", &string);
    }

    #[test]
    fn test_indexed_paths() {
        #[derive(Serialize)]
        struct Context {
            foo: (usize, usize),
        }

        let template = compile("{ foo.1 }{ foo.0 }");
        let context = Context { foo: (123, 456) };
        let context = ::serde_json::to_value(&context).unwrap();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("456123", &string);
    }

    #[test]
    fn test_indexed_paths_fall_back_to_string_lookup() {
        #[derive(Serialize)]
        struct Context {
            foo: HashMap<&'static str, usize>,
        }

        let template = compile("{ foo.1 }{ foo.0 }");
        let mut foo = HashMap::new();
        foo.insert("0", 123);
        foo.insert("1", 456);
        let context = Context { foo };
        let context = ::serde_json::to_value(&context).unwrap();
        let template_registry = other_templates();
        let formatter_registry = formatters();
        let string = template
            .render(
                &context,
                &template_registry,
                &formatter_registry,
                &default_formatter(),
            )
            .unwrap();
        assert_eq!("456123", &string);
    }
}
