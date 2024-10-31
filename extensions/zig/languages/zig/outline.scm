(test_declaration "test" @context
  [
    (string)
    (identifier)
  ] @name) @item


(function_declaration 
  "pub"? @context
  [
    "extern"
    "export"
    "inline"
    "noinline"
  ]? @context
  "fn" @context
  name: (_) @name) @item

(variable_declaration
  "pub"? @context
  [
    "extern"
    "export"
  ]? @context
  ["const" "var"] @context
  (identifier) @name) @item
