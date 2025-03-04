(subject) @markup.heading
(path) @string.special.path
(branch) @string.special.symbol
(commit) @constant
(item) @markup.link.url
(header) @tag

(change kind: "new file" @diff.plus)
(change kind: "deleted" @diff.minus)
(change kind: "modified" @diff.delta)
(change kind: "renamed" @diff.delta.moved)

(trailer
  key: (trailer_key) @variable.other.member
  value: (trailer_value) @string)

[":" "=" "->" (scissors)] @punctuation.delimiter
(comment) @comment
