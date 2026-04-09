; functions
(function_signature_item) @function.around

(function_item
  body: (_
    "{"
    (_)* @function.inside
    "}")) @function.around

; classes
(struct_item
  body: (_
    [
      "{"
      "("
    ]?
    [
      (_)
      ","?
    ]* @class.inside
    [
      "}"
      ")"
    ]?)) @class.around

(enum_item
  body: (_
    "{"
    [
      (_)
      ","?
    ]* @class.inside
    "}")) @class.around

(union_item
  body: (_
    "{"
    [
      (_)
      ","?
    ]* @class.inside
    "}")) @class.around

(trait_item
  body: (_
    "{"
    [
      (_)
      ","?
    ]* @class.inside
    "}")) @class.around

(impl_item
  body: (_
    "{"
    [
      (_)
      ","?
    ]* @class.inside
    "}")) @class.around

(mod_item
  body: (_
    "{"
    [
      (_)
      ","?
    ]* @class.inside
    "}")) @class.around

; comments
(line_comment)+ @comment.around

(block_comment) @comment.around
