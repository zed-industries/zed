(comment) @comment.around

(rule_set
  (block
    ("{"
      (_)* @function.inside
      "}"))) @function.around

(keyframe_block
  (block
    ("{"
      (_)* @function.inside
      "}"))) @function.around

(media_statement
  (block
    ("{"
      (_)* @class.inside
      "}"))) @class.around

(supports_statement
  (block
    ("{"
      (_)* @class.inside
      "}"))) @class.around

(keyframes_statement
  (keyframe_block_list
    ("{"
      (_)* @class.inside
      "}"))) @class.around
