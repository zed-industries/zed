//! Documentation of TinyTemplate's template syntax.
//!
//! ### Context Types
//!
//! TinyTemplate uses `serde_json`'s Value structure to represent the context. Therefore, any
//! `Serializable` structure can be used as a context. All values in such structures are mapped to
//! their JSON representations - booleans, numbers, strings, arrays, objects and nulls.
//!
//! ### Values
//!
//! Template values are marked with `{...}`. For example, this will look up the "name" field in
//! the context structure and insert it into the rendered string:
//!
//! ```text
//! Hello, {name}, how are you?
//! ```
//!
//! Optionally, a value formatter may be provided. One formatter, "unescaped", is provided by
//! default. Any other formatters must be registered with the
//! [`TinyTemplate.add_formatter`](../struct.TinyTemplate.html#method.add_formatter)
//! function prior to rendering or an error will be generated. This will call the formatter function
//! registered as "percent_formatter" with the value of the "percentage" field:
//!
//! ```text
//! Give it {percentage | percent_formatter}!
//! ```
//!
//! The value may be a dotted path through a hierarchy of context objects. This will look up the
//! "friend" field in the context structure, then substitute the "name" field from the "friend"
//! object.
//!
//! ```text
//! And hello to {friend.name} as well!
//! ```
//!
//! Additionally, you may use the `@root` keyword to refer to the root object of your context.
//! Since TinyTemplate can't normally print complex context objects, this is only useful if the
//! context is a simple object like an integer or string.
//!
//! ### Conditionals
//!
//! TinyTemplate blocks are marked with `{{...}}` - double-braces where values are single-braces.
//!
//! Conditionals are denoted by "{{ if path }}...{{ else }}...{{ endif }}". The Else block is
//! optional. Else-if is not currently supported. If "path" evaluates to a truthy expression
//! (true if boolean, non-zero if numeric, non-empty for strings and arrays, and non-null for
//! objects) then the section of the template between "if" and "else" is evaluated, otherwise the
//! section between "else" and "endif" (if present) is evaluated.
//!
//! ```text
//! {{ if user.is_birthday }}
//! Happy Birthday!
//! {{ else }}
//! Have a nice day!
//! {{ endif }}
//! ```
//!
//! The condition can be negated by using "{{ if not path }}":
//!
//! ```text
//! {{ if not user.is_birthday }}
//! Have a nice day!
//! {{ else }}
//! Happy Birthday!
//! {{ endif }}
//! ```
//!
//! If desired, the `@root` keyword can be used to branch on the root context object.
//!
//! ### Loops
//!
//! TinyTemplate supports iterating over the values of arrays. Only arrays are supported. Loops
//! are denoted by "{{ for value_name in value.path }}...{{ endfor }}". The section of the template between
//! the two tags will be executed once for each value in the array denoted by "value.path".
//!
//! ```text
//! Hello to {{ for name in guests }}
//! {name}
//! {{ endfor }}
//! ```
//!
//! If the iteration value chosen in the "for" tag is the same as that of a regular context value,
//! the name in the tag will shadow the context value for the scope of the loop. For nested loops,
//! inner loops will shadow the values of outer loops.
//!
//! ```text
//! {{ for person in guests }}
//! Hello to {person}{{ for person in person.friends }} and your friend {person}{{ endfor }}
//! {{ endfor }}
//! ```
//!
//! There are three special values which are available within a loop:
//!
//! * `@index` - zero-based index of the current value within the array.
//! * `@first` - true if this is the first iteration of the loop, otherwise false.
//! * `@last` - true if this is the last iteration of the loop, otherwise false.
//!
//! ```text
//! Hello to {{ for name in guests -}}
//! { @index }. {name},
//! {{- endfor }}
//! ```
//!
//!
//! In case of nested loops, these values refer to the innermost loop which contains them.
//!
//! If the root context object is an array, the `@root` keyword can be used to iterate over the
//! root object.
//!
//! ### With Blocks
//!
//! Templates can use with blocks to partially shadows the outer context, the same way that
//! for-loops do. These are formed like so:
//!
//! "{{ with path.to.value as name }}..{{ endwith }}""
//!
//! For example:
//!
//! ```text
//! {{ with person.spouse as s }}
//! Hello { s.name }!
//! {{ endwith }}
//! ```
//!
//! This looks up "person.spouse" and adds that to the context as "s" within the block. Only the
//! name "s" is shadowed within the with block and otherwise the outer context is still accessible.
//!
//! ### Trimming Whitespace
//!
//! If a block tag, comment or value tag includes a "-" character at the start, the trailing
//! whitespace of the previous text section will be skipped in the output. Likewise, if the tag
//! ends with a "-", the leading whitespace of the following text will be skipped.
//!
//! ```text
//! Hello { friend.name -}
//! , how are you?
//!
//! {{- if status.good }} I am fine.               {{- endif }}
//! ```
//!
//! This will print "Hello friend, how are you? I am fine." without the newlines or extra spaces.
//!
//! ### Calling other Templates
//!
//! Templates may call other templates by name. The other template must have been registered using
//! the [`TinyTemplate.add_template`](../struct.TinyTemplate.html#method.add_template) function
//! before rendering or an error will be generated. This is done with the "call" tag:
//!
//! "{{ call template_name with path.to.context }}"
//!
//! The call tag has no closing tag. This will look up the "path.to.context" path in the current
//! context, then render the "template_name" template using the value at that path as the context
//! for the other template. The string produced by the called template is then inserted into the
//! output from the calling template. This can be used for a limited form of template code reuse.
//!
//! ### Comments
//!
//! Comments in the templates are denoted by "{# comment text #}". Comments will be skipped when
//! rendering the template, though whitespace adjacent to comments will not be stripped unless the
//! "-" is added. For example:
//!
//! ```text
//! Hello
//!
//! {#- This is a comment #} world!
//! ```
//!
//! This will print "Hello world!".
//!
//! ### Escaping Curly Braces
//!
//! If your template contains opening curly-braces (`{`), they must be escaped using a leading `\`
//! character. For example:
//!
//! ```text
//! h2 \{
//!     font-size: {fontsize};
//! }
//! ```
//!
//! If using a string literal in rust source code, the `\` itself must be escaped, producing `\\{`.
//!

// There's nothing here, this module is solely for documentation.
