#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableLookupKind {
    Variable,
    Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableScope {
    Local,
    Global,
}

#[derive(Debug, Clone)]
pub struct InlineValueLocation {
    pub variable_name: String,
    pub scope: VariableScope,
    pub lookup: VariableLookupKind,
    pub row: usize,
    pub column: usize,
}

/// A trait for providing inline values for debugging purposes.
///
/// Implementors of this trait are responsible for analyzing a given node in the
/// source code and extracting variable information, including their names,
/// scopes, and positions. This information is used to display inline values
/// during debugging sessions. Implementors must also handle variable scoping
/// themselves by traversing the syntax tree upwards to determine whether a
/// variable is local or global.
pub trait InlineValueProvider {
    /// Provides a list of inline value locations based on the given node and source code.
    ///
    /// # Parameters
    /// - `node`: The root node of the active debug line. Implementors should traverse
    ///   upwards from this node to gather variable information and determine their scope.
    /// - `source`: The source code as a string slice, used to extract variable names.
    /// - `max_row`: The maximum row to consider when collecting variables. Variables
    ///   declared beyond this row should be ignored.
    ///
    /// # Returns
    /// A vector of `InlineValueLocation` instances, each representing a variable's
    /// name, scope, and the position of the inline value should be shown.
    fn provide(
        &self,
        node: language::Node,
        source: &str,
        max_row: usize,
    ) -> Vec<InlineValueLocation>;
}

pub struct RustInlineValueProvider;

impl InlineValueProvider for RustInlineValueProvider {
    fn provide(
        &self,
        mut node: language::Node,
        source: &str,
        max_row: usize,
    ) -> Vec<InlineValueLocation> {
        let mut variables = Vec::new();

        loop {
            for child in node.named_children(&mut node.walk()) {
                if child.start_position().row >= max_row {
                    break;
                }

                if child.kind() == "let_declaration" {
                    if let Some(identifier) = child.child_by_field_name("pattern") {
                        variables.push(InlineValueLocation {
                            variable_name: source[identifier.byte_range()].to_string(),
                            scope: VariableScope::Local,
                            lookup: VariableLookupKind::Variable,
                            row: identifier.end_position().row,
                            column: identifier.end_position().column,
                        });
                    }
                }
            }

            if node.kind() == "function_item" {
                break;
            }

            if let Some(parent) = node.parent() {
                node = parent;
            } else {
                break;
            }
        }

        variables
    }
}
