(stylesheet
  (import_statement
    "@import" @context
    (string_value) @name) @item)

(rule_set
  (selectors
    .
    (_) @name
    ("," @name
      (_) @name)*)) @item

(media_statement
  "@media" @context
  (_) @name
  (block)) @item
