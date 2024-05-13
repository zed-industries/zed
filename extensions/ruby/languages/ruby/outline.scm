(class
    "class" @context
    name: (_) @name) @item

((identifier) @context
  (#match? @context "^(private|protected|public)$")) @item

(method
    "def" @context
    name: (_) @name) @item

(singleton_method
    "def" @context
    object: (_) @context
    "." @context
    name: (_) @name) @item

(module
    "module" @context
    name: (_) @name) @item
