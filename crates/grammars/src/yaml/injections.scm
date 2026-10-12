((comment) @injection.content
  (#set! injection.language "comment"))

; GitHub actions: JavaScript for workflow scripting (inline and block)
(block_mapping
  (block_mapping_pair
    key: (flow_node) @_uses
    (#eq? @_uses "uses")
    value: (flow_node) @_actions_ghs
    (#match? @_actions_ghs "^actions/github-script"))
  (block_mapping_pair
    key: (flow_node) @_with
    (#eq? @_with "with")
    value: (block_node
      (block_mapping
        (block_mapping_pair
          key: (flow_node) @_run
          (#eq? @_run "script")
          value: [
            (flow_node
              (plain_scalar
                (string_scalar) @injection.content))
            (block_node
              (block_scalar) @injection.content)
          ]
          (#set! injection.language "javascript"))))))

; SQL injection via comment annotation
; Examples:
;   # language=sql
;   query: SELECT * FROM users
;
;   # sql
;   query: |
;     SELECT * FROM users
(block_mapping
  (comment) @_comment
  .
  (block_mapping_pair
    value: [
      (flow_node
        (plain_scalar
          (string_scalar) @injection.content))
      (block_node
        (block_scalar) @injection.content)
    ])
  (#match? @_comment "^#\\s*(language=)?(?i:sql)\\s*$")
  (#set! injection.language "sql"))

(block_sequence
  (comment) @_comment
  .
  (block_sequence_item
    [
      (flow_node
        (plain_scalar
          (string_scalar) @injection.content))
      (block_node
        (block_scalar) @injection.content)
    ])
  (#match? @_comment "^#\\s*(language=)?(?i:sql)\\s*$")
  (#set! injection.language "sql"))
