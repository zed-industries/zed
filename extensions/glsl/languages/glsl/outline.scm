(comment) @annotation

(preproc_def
  "#define" @context
  (identifier) @name) @item

(preproc_function_def
  "#define" @context
  (identifier) @name
  (preproc_params
    "(" @context
    ")" @context)) @item

(struct_specifier
  "struct" @context
  (type_identifier) @name) @item

(union_specifier
  "union" @context
  (type_identifier) @name) @item

(enum_specifier
  "enum" @context
  (type_identifier) @name) @item

(enumerator
  (identifier) @name) @item

(declaration
  (layout_specification)? @context
  [
    "in"
    "out"
    "inout"
    "uniform"
    "shared"
    "attribute"
    "varying"
    "buffer"
    "coherent"
    "readonly"
    "writeonly"
    "precision"
    "highp"
    "mediump"
    "lowp"
    "subroutine"
    "centroid"
    "sample"
    "patch"
    "smooth"
    "flat"
    "noperspective"
    "invariant"
    "precise"
    (extension_storage_class)
  ]+ @context
  [
    (type_identifier)
    (primitive_type)
  ] @context
  [
    (identifier) @name
    (array_declarator
      (identifier) @name)
  ]) @item

(declaration
  (layout_specification)? @context
  [
    "in"
    "out"
    "inout"
    "uniform"
    "shared"
    "buffer"
    "coherent"
    "readonly"
    "writeonly"
    (extension_storage_class)
  ]* @context
  (identifier) @name
  (field_declaration_list)) @item

(field_declaration
  [
    (type_identifier)
    (primitive_type)
  ] @context
  [
    (field_identifier) @name
    (array_declarator
      (field_identifier) @name)
    (pointer_declarator
      (field_identifier) @name)
  ]) @item

(declaration
  "subroutine"? @context
  [
    (type_identifier)
    (primitive_type)
  ] @context
  (function_declarator
    (identifier) @name
    (parameter_list
      "(" @context
      ")" @context))) @item

(function_definition
  "subroutine"? @context
  [
    (type_identifier)
    (primitive_type)
  ] @context
  (function_declarator
    (identifier) @name
    (parameter_list
      "(" @context
      ")" @context))) @item
