(module_type_definition
  "module" @context
  "type" @context
  name: (_) @name) @item

(module_definition
    "module" @context
    (module_binding name: (_) @name)) @item

(type_definition
    "type" @context
    (type_binding name: (_) @name)) @item

(class_definition
    "class" @context
    (class_binding
      "virtual"? @context
      name: (_) @name)) @item

(class_type_definition
  "class" @context
  "type" @context
  (class_type_binding
    "virtual"? @context
    name: (_) @name)) @item

(instance_variable_definition
  "val" @context
  "method"? @context
  name: (_) @name) @item

(method_specification
  "method" @context
  "virtual"? @context
   (method_name) @name) @item

(value_specification
    "val" @context
    (value_name) @name) @item

(external
  "external" @context
  (value_name) @name) @item

(exception_definition
    "exception" @context
    (constructor_declaration
      (constructor_name) @name)) @item
