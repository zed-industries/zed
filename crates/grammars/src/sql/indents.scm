[
  (select)
  (cte)
  (column_definitions)
  (case)
  (subquery)
  (insert)
] @indent

(block
  (keyword_begin)) @indent

(column_definitions
  ")" @end) @indent

(subquery
  ")" @end) @indent

(cte
  ")" @end) @indent

[
  (keyword_end)
  (keyword_values)
  (keyword_into)
] @outdent

(keyword_end) @outdent
