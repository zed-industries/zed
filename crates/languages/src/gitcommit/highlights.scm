(subject) @heading.markup
(path) @string.special.path
(branch) @string.special.symbol
(commit) @constant
(item) @link.uri.markup
(header) @tag
(comment) @comment

(change kind: "new file" @diff.plus)
(change kind: "deleted" @diff.minus)
(change kind: "modified" @diff.delta)
(change kind: "renamed" @diff.delta.moved)

(trailer
  key: (trailer_key) @variable.other.member
  value: (trailer_value) @string)

[":" "=" "->" (scissors)] @punctuation.delimiter
