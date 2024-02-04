; Function calls
(function_call_expr
  caller:  (variable_expr
      (identifier)@name ))@reference.call

(function_call_expr
  caller: (field_access_expr (identifier)@name .))@reference.call

; Function definitions
(value_declaration(decl_left 
  (identifier_pattern 
   (identifier)@name))(expr_body(anon_fun_expr)))@definition.function

