(decorator) @annotation

(class_definition
  "class" @context
  name: (identifier) @name) @item @item.class

(function_definition
  "async"? @context
  "def" @context
  name: (_) @name) @item @item.function
