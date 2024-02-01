(comment) @comment
(generated_comment) @comment
(title) @text.title
(text) @text
(branch) @text.reference
(change) @keyword
(filepath) @text.uri
(arrow) @punctuation.delimiter

(subject) @text.title
(subject (overflow) @text)
(prefix (type) @keyword)
(prefix (scope) @parameter)
(prefix [
    "("
    ")"
    ":"
] @punctuation.delimiter)
(prefix [
    "!"
] @punctuation.special)

(message) @text

(trailer (token) @keyword)
(trailer (value) @text)

(breaking_change (token) @text.warning)
(breaking_change (value) @text)

(scissor) @comment
(subject_prefix) @keyword

(ERROR) @error
