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

; Minitest/RSpec
(call
   method: (identifier) @run (#any-of? @run "describe" "context" "test")
   arguments: (argument_list . (_) @name)
) @item
