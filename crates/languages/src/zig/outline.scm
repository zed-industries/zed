(Decl (
    FnProto(
        "fn" @context
        function: (_) @name
    )
)
 ) @item

(
    Decl (
        VarDecl (
                "const"
                variable_type_function: (_) @name
                (ErrorUnionExpr) @context
            )
    )
) @item

(
    TestDecl (
        "test" @context
        (STRINGLITERALSINGLE)? @name
    )
) @item
