(class_definition
    "class" @context
    name: (_) @name) @item

(enum_definition
    "enum" @context
    name: (_) @name) @item

(object_definition
    "object" @context
    name: (_) @name) @item

(trait_definition
    "trait" @context
    name: (_) @name) @item

(type_definition
    "type" @context
    name: (_) @name) @item

(function_definition
    "def" @context
    name: (_) @name) @item

(val_definition
  "val" @context
  pattern: (identifier) @name) @item