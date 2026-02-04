[
  "syntax"
  "package"
  "option"
  "optional"
  "import"
  "service"
  "rpc"
  "returns"
  "message"
  "enum"
  "extend"
  "oneof"
  "repeated"
  "reserved"
  "to"
] @keyword

[
  (key_type)
  (type)
  (message_name)
  (enum_name)
  (service_name)
  (rpc_name)
  (message_or_enum_type)
] @type

(enum_field
  (identifier) @constant)

[
  (string)
  "\"proto3\""
] @string

(int_lit) @number

[
  (true)
  (false)
] @boolean

(comment) @comment

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "<"
  ">"
]  @punctuation.bracket

[
 ";"
 ","
] @punctuation.delimiter

"=" @operator
