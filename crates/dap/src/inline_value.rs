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
pub trait InlineValueProvider: 'static + Send + Sync {
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

            if matches!(node.kind(), "function_definition" | "module")
                && node.range().end_point.row < max_row
            {
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

pub struct GoInlineValueProvider;

impl InlineValueProvider for GoInlineValueProvider {
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
                        "var_declaration" => {
                            for var_spec in child.named_children(&mut child.walk()) {
                                if var_spec.kind() == "var_spec" {
                                    if let Some(name_node) = var_spec.child_by_field_name("name") {
                                        let variable_name =
                                            source[name_node.byte_range()].to_string();

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
                                            row: name_node.end_position().row,
                                            column: name_node.end_position().column,
                                        });
                                    }
                                }
                            }
                        }
                        "short_var_declaration" => {
                            if let Some(left_side) = child.child_by_field_name("left") {
                                for identifier in left_side.named_children(&mut left_side.walk()) {
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
                        "assignment_statement" => {
                            if let Some(left_side) = child.child_by_field_name("left") {
                                for identifier in left_side.named_children(&mut left_side.walk()) {
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
                        "function_declaration" | "method_declaration" => {
                            if let Some(params) = child.child_by_field_name("parameters") {
                                for param in params.named_children(&mut params.walk()) {
                                    if param.kind() == "parameter_declaration" {
                                        if let Some(name_node) = param.child_by_field_name("name") {
                                            let variable_name =
                                                source[name_node.byte_range()].to_string();

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
                                                row: name_node.end_position().row,
                                                column: name_node.end_position().column,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        "for_statement" => {
                            if let Some(clause) = child.named_child(0) {
                                if clause.kind() == "for_clause" {
                                    if let Some(init) = clause.named_child(0) {
                                        if init.kind() == "short_var_declaration" {
                                            if let Some(left_side) =
                                                init.child_by_field_name("left")
                                            {
                                                if left_side.kind() == "expression_list" {
                                                    for identifier in left_side
                                                        .named_children(&mut left_side.walk())
                                                    {
                                                        if identifier.kind() == "identifier" {
                                                            let variable_name = source
                                                                [identifier.byte_range()]
                                                            .to_string();

                                                            if variable_names
                                                                .contains(&variable_name)
                                                            {
                                                                continue;
                                                            }

                                                            if let Some(index) =
                                                                variable_names_in_scope
                                                                    .get(&variable_name)
                                                            {
                                                                variables.remove(*index);
                                                            }

                                                            variable_names_in_scope.insert(
                                                                variable_name.clone(),
                                                                variables.len(),
                                                            );
                                                            variables.push(InlineValueLocation {
                                                                variable_name,
                                                                scope: VariableScope::Local,
                                                                lookup:
                                                                    VariableLookupKind::Variable,
                                                                row: identifier.end_position().row,
                                                                column: identifier
                                                                    .end_position()
                                                                    .column,
                                                            });
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if clause.kind() == "range_clause" {
                                    if let Some(left) = clause.child_by_field_name("left") {
                                        if left.kind() == "expression_list" {
                                            for identifier in left.named_children(&mut left.walk())
                                            {
                                                if identifier.kind() == "identifier" {
                                                    let variable_name =
                                                        source[identifier.byte_range()].to_string();

                                                    if variable_name == "_" {
                                                        continue;
                                                    }

                                                    if variable_names.contains(&variable_name) {
                                                        continue;
                                                    }

                                                    if let Some(index) =
                                                        variable_names_in_scope.get(&variable_name)
                                                    {
                                                        variables.remove(*index);
                                                    }
                                                    variable_names_in_scope.insert(
                                                        variable_name.clone(),
                                                        variables.len(),
                                                    );
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
                        }
                        _ => {}
                    }
                } else if child.kind() == "var_declaration" {
                    for var_spec in child.named_children(&mut child.walk()) {
                        if var_spec.kind() == "var_spec" {
                            if let Some(name_node) = var_spec.child_by_field_name("name") {
                                let variable_name = source[name_node.byte_range()].to_string();
                                variables.push(InlineValueLocation {
                                    variable_name,
                                    scope: VariableScope::Global,
                                    lookup: VariableLookupKind::Expression,
                                    row: name_node.end_position().row,
                                    column: name_node.end_position().column,
                                });
                            }
                        }
                    }
                }
            }

            variable_names.extend(variable_names_in_scope.keys().cloned());

            if matches!(node.kind(), "function_declaration" | "method_declaration") {
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
#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_go_inline_value_provider() {
        let provider = GoInlineValueProvider;
        let source = r#"
package main

func main() {
    items := []int{1, 2, 3, 4, 5}
    for i, v := range items {
        println(i, v)
    }
    for j := 0; j < 10; j++ {
        println(j)
    }
}
"#;

        let mut parser = Parser::new();
        if parser
            .set_language(&tree_sitter_go::LANGUAGE.into())
            .is_err()
        {
            return;
        }
        let Some(tree) = parser.parse(source, None) else {
            return;
        };
        let root_node = tree.root_node();

        let mut main_body = None;
        for child in root_node.named_children(&mut root_node.walk()) {
            if child.kind() == "function_declaration" {
                if let Some(name) = child.child_by_field_name("name") {
                    if &source[name.byte_range()] == "main" {
                        if let Some(body) = child.child_by_field_name("body") {
                            main_body = Some(body);
                            break;
                        }
                    }
                }
            }
        }

        let Some(main_body) = main_body else {
            return;
        };

        let variables = provider.provide(main_body, source, 100);
        assert!(variables.len() >= 2);

        let variable_names: Vec<&str> =
            variables.iter().map(|v| v.variable_name.as_str()).collect();
        assert!(variable_names.contains(&"items"));
        assert!(variable_names.contains(&"j"));
    }

    #[test]
    fn test_go_inline_value_provider_counter_pattern() {
        let provider = GoInlineValueProvider;
        let source = r#"
package main

func main() {
    N := 10
    for i := range N {
        println(i)
    }
}
"#;

        let mut parser = Parser::new();
        if parser
            .set_language(&tree_sitter_go::LANGUAGE.into())
            .is_err()
        {
            return;
        }
        let Some(tree) = parser.parse(source, None) else {
            return;
        };
        let root_node = tree.root_node();

        let mut main_body = None;
        for child in root_node.named_children(&mut root_node.walk()) {
            if child.kind() == "function_declaration" {
                if let Some(name) = child.child_by_field_name("name") {
                    if &source[name.byte_range()] == "main" {
                        if let Some(body) = child.child_by_field_name("body") {
                            main_body = Some(body);
                            break;
                        }
                    }
                }
            }
        }

        let Some(main_body) = main_body else {
            return;
        };
        let variables = provider.provide(main_body, source, 100);

        let variable_names: Vec<&str> =
            variables.iter().map(|v| v.variable_name.as_str()).collect();
        assert!(variable_names.contains(&"N"));
        assert!(variable_names.contains(&"i"));
    }
}
