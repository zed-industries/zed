(test_declaration 
  "test" @context
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

(source_file
  (variable_declaration
    "pub"? @context
    (identifier) @name
    "=" (_) @context) @item)

(struct_declaration
  (variable_declaration
    "pub"? @context
    (identifier) @name
    "=" (_) @context) @item)

(union_declaration
  (variable_declaration
    "pub"? @context
    (identifier) @name
    "=" (_) @context) @item)

(enum_declaration
  (variable_declaration
    "pub"? @context
    (identifier) @name
    "=" (_) @context) @item)

(opaque_declaration
  (variable_declaration
    "pub"? @context
    (identifier) @name
    "=" (_) @context) @item)

(container_field
  . (_) @name) @item
