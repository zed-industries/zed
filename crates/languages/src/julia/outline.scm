(module_definition
  ["module" "baremodule"] @context
  name: (identifier) @name) @item

(primitive_definition
  name: (identifier) @name) @item

(abstract_definition
  name: (identifier) @name
  (type_clause)? @context) @item

(function_definition
  "function" @context
  name: (_) @name
  (type_parameter_list)? @context
  parameters: (parameter_list)? @context
  (where_clause)? @context) @item

(short_function_definition
  name: (_) @name
  (type_parameter_list)? @context
  parameters: (parameter_list) @context
  (where_clause)? @context) @item

(macro_definition
  "macro" @context
  name: (identifier) @name
  parameters: (parameter_list) @context) @item

(struct_definition
  "mutable"? @context
  "struct" @context
  name: (_) @name
  (type_parameter_list)? @context) @item

(const_statement
  "const" @context
  (assignment
    (_) @name
    (operator)
    (_))) @item
