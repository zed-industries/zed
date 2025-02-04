(function_definition
    "function"?
    body: (
        _
        "{" @start
        "}" @end
    )) @indent

(array
    "(" @start
    ")" @end
    ) @indent
