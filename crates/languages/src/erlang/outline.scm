(module_attribute
    "module" @context
    name: (_) @name) @item

(behaviour_attribute
    "behaviour" @context
    (atom) @name) @item

(type_alias
    "type" @context
    name: (_) @name) @item

(opaque
    "opaque" @context
    name: (_) @name) @item

(pp_define
    "define" @context
    lhs: (_) @name) @item

(record_decl
    "record" @context
    name: (_) @name) @item

(callback
    "callback" @context
    fun: (_) @function ( (_) @name)) @item

(fun_decl (function_clause
    name: (_) @name
    args: (_) @context)) @item
