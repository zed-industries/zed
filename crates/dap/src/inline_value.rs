use std::collections::{HashMap, HashSet};

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
        let mut variable_names = HashSet::new();
        let mut scope = VariableScope::Local;

        loop {
            let mut variable_names_in_scope = HashMap::new();
            for child in node.named_children(&mut node.walk()) {
                if child.start_position().row >= max_row {
                    break;
                }

                if scope == VariableScope::Local && child.kind() == "let_declaration" {
                    if let Some(identifier) = child.child_by_field_name("pattern") {
                        let variable_name = source[identifier.byte_range()].to_string();

                        if variable_names.contains(&variable_name) {
                            continue;
                        }

                        if let Some(index) = variable_names_in_scope.get(&variable_name) {
                            variables.remove(*index);
                        }

                        variable_names_in_scope.insert(variable_name.clone(), variables.len());
                        variables.push(InlineValueLocation {
                            variable_name,
                            scope: VariableScope::Local,
                            lookup: VariableLookupKind::Variable,
                            row: identifier.end_position().row,
                            column: identifier.end_position().column,
                        });
                    }
                } else if child.kind() == "static_item" {
                    if let Some(name) = child.child_by_field_name("name") {
                        let variable_name = source[name.byte_range()].to_string();
                        variables.push(InlineValueLocation {
                            variable_name,
                            scope: scope.clone(),
                            lookup: VariableLookupKind::Expression,
                            row: name.end_position().row,
                            column: name.end_position().column,
                        });
                    }
                }
            }

            variable_names.extend(variable_names_in_scope.keys().cloned());

            if matches!(node.kind(), "function_item" | "closure_expression") {
                scope = VariableScope::Global;
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

pub struct PythonInlineValueProvider;

impl InlineValueProvider for PythonInlineValueProvider {
    fn provide(
        &self,
        mut node: language::Node,
        source: &str,
        max_row: usize,
    ) -> Vec<InlineValueLocation> {
        let mut variables = Vec::new();
        let mut variable_names = HashSet::new();
        let mut scope = VariableScope::Local;

        loop {
            let mut variable_names_in_scope = HashMap::new();
            for child in node.named_children(&mut node.walk()) {
                if child.start_position().row >= max_row {
                    break;
                }

                if scope == VariableScope::Local {
                    match child.kind() {
                        "expression_statement" => {
                            if let Some(expr) = child.child(0) {
                                if expr.kind() == "assignment" {
                                    if let Some(param) = expr.child(0) {
                                        let param_identifier = if param.kind() == "identifier" {
                                            Some(param)
                                        } else if param.kind() == "typed_parameter" {
                                            param.child(0)
                                        } else {
                                            None
                                        };

                                        if let Some(identifier) = param_identifier {
                                            if identifier.kind() == "identifier" {
                                                let variable_name =
                                                    source[identifier.byte_range()].to_string();

                                                if variable_names.contains(&variable_name) {
                                                    continue;
                                                }

                                                if let Some(index) =
                                                    variable_names_in_scope.get(&variable_name)
                                                {
                                                    variables.remove(*index);
                                                }

                                                variable_names_in_scope
                                                    .insert(variable_name.clone(), variables.len());
                                                variables.push(InlineValueLocation {
                                                    variable_name,
                                                    scope: VariableScope::Local,
                                                    lookup: VariableLookupKind::Variable,
                                                    row: identifier.end_position().row,
                                                    column: identifier.end_position().column,
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        "function_definition" => {
                            if let Some(params) = child.child_by_field_name("parameters") {
                                for param in params.named_children(&mut params.walk()) {
                                    let param_identifier = if param.kind() == "identifier" {
                                        Some(param)
                                    } else if param.kind() == "typed_parameter" {
                                        param.child(0)
                                    } else {
                                        None
                                    };

                                    if let Some(identifier) = param_identifier {
                                        if identifier.kind() == "identifier" {
                                            let variable_name =
                                                source[identifier.byte_range()].to_string();

                                            if variable_names.contains(&variable_name) {
                                                continue;
                                            }

                                            if let Some(index) =
                                                variable_names_in_scope.get(&variable_name)
                                            {
                                                variables.remove(*index);
                                            }

                                            variable_names_in_scope
                                                .insert(variable_name.clone(), variables.len());
                                            variables.push(InlineValueLocation {
                                                variable_name,
                                                scope: VariableScope::Local,
                                                lookup: VariableLookupKind::Variable,
                                                row: identifier.end_position().row,
                                                column: identifier.end_position().column,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        "for_statement" => {
                            if let Some(target) = child.child_by_field_name("left") {
                                if target.kind() == "identifier" {
                                    let variable_name = source[target.byte_range()].to_string();

                                    if variable_names.contains(&variable_name) {
                                        continue;
                                    }

                                    if let Some(index) = variable_names_in_scope.get(&variable_name)
                                    {
                                        variables.remove(*index);
                                    }

                                    variable_names_in_scope
                                        .insert(variable_name.clone(), variables.len());
                                    variables.push(InlineValueLocation {
                                        variable_name,
                                        scope: VariableScope::Local,
                                        lookup: VariableLookupKind::Variable,
                                        row: target.end_position().row,
                                        column: target.end_position().column,
                                    });
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            variable_names.extend(variable_names_in_scope.keys().cloned());

            if matches!(node.kind(), "function_definition" | "module") {
                scope = VariableScope::Global;
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
