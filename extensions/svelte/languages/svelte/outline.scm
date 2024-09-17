
(script_element
    (start_tag) @name
    (raw_text) @context @item
)

(script_element
    (end_tag) @name @item
)

(style_element
    (start_tag) @name
    (raw_text) @context
) @item


(document) @item

(comment) @annotation

(if_statement
    (if_start) @name
) @item

(else_block
    (else_start) @name
) @item

(else_if_block
    (else_if_start) @name
) @item

(element
    (start_tag) @name
) @item

(element
    (self_closing_tag) @name
) @item


; (if_end) @name @item

(each_statement
    (each_start) @name
) @item


(snippet_statement
    (snippet_start) @name
) @item

(snippet_end) @name @item

(html_tag) @name @item

(const_tag) @name @item

(await_statement
    (await_start) @name
) @item

(then_block
    (then_start) @name
) @item

(catch_block
    (catch_start) @name
) @item
