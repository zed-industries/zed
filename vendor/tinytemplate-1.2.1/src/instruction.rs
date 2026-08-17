use std::ops::Deref;

/// TinyTemplate implements a simple bytecode interpreter for its template engine. Instructions
/// for this interpreter are represented by the Instruction enum and typically contain various
/// parameters such as the path to context values or name strings.
///
/// In TinyTemplate, the template string itself is assumed to be statically available (or at least
/// longer-lived than the TinyTemplate instance) so paths and instructions simply borrow string
/// slices from the template text. These string slices can then be appended directly to the output
/// string.

/// Enum for a step in a path which optionally contains a parsed index.
#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) enum PathStep<'template> {
    Name(&'template str),
    Index(&'template str, usize),
}
impl<'template> Deref for PathStep<'template> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            PathStep::Name(s) => s,
            PathStep::Index(s, _) => s,
        }
    }
}

/// Sequence of named steps used for looking up values in the context
pub(crate) type Path<'template> = Vec<PathStep<'template>>;

/// Path, but as a slice.
pub(crate) type PathSlice<'a, 'template> = &'a [PathStep<'template>];

/// Enum representing the bytecode instructions.
#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) enum Instruction<'template> {
    /// Emit a literal string into the output buffer
    Literal(&'template str),

    /// Look up the value for the given path and render it into the output buffer using the default
    /// formatter
    Value(Path<'template>),

    /// Look up the value for the given path and pass it to the formatter with the given name
    FormattedValue(Path<'template>, &'template str),

    /// Look up the value at the given path and jump to the given instruction index if that value
    /// is truthy (if the boolean is true) or falsy (if the boolean is false)
    Branch(Path<'template>, bool, usize),

    /// Push a named context on the stack, shadowing only that name.
    PushNamedContext(Path<'template>, &'template str),

    /// Push an iteration context on the stack, shadowing the given name with the current value from
    /// the vec pointed to by the path. The current value will be updated by the Iterate instruction.
    /// This is always generated before an Iterate instruction which actually starts the iterator.
    PushIterationContext(Path<'template>, &'template str),

    /// Pop a context off the stack
    PopContext,

    /// Advance the topmost iterator on the context stack by one and update that context. If the
    /// iterator is empty, jump to the given instruction.
    Iterate(usize),

    /// Unconditionally jump to the given instruction. Used to skip else blocks and repeat loops.
    Goto(usize),

    /// Look up the named template and render it into the output buffer with the value pointed to
    /// by the path as its context.
    Call(&'template str, Path<'template>),
}

/// Convert a path back into a dotted string.
pub(crate) fn path_to_str(path: PathSlice) -> String {
    let mut path_str = "".to_string();
    for (i, step) in path.iter().enumerate() {
        if i > 0 {
            path_str.push('.');
        }
        path_str.push_str(step);
    }
    path_str
}
