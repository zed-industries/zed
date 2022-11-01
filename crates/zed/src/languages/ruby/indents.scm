(method "end" @end) @indent
(class "end" @end) @indent
(module "end" @end) @indent
(begin "end" @end) @indent
(do_block "end" @end) @indent

(then) @indent
(call) @indent

(ensure) @outdent
(rescue) @outdent
(else) @outdent


(_ "[" "]" @end) @indent
(_ "{" "}" @end) @indent
(_ "(" ")" @end) @indent
