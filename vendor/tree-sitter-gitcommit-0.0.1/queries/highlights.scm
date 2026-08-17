; # Reference highlights
;
; These highlights use custom scopes that won't exist in the editor or app
; you want to use tree-sitter-git-commit in. These are just for testing
; purposes. When modifying/re-using these queries, be sure to change the
; scopes to the scopes your editor uses.
;
; Note: these highlights also won't look good if you use
; 'tree-sitter highlight' from tree-sitter-cli.

(subject) @subject
(path) @path
(branch) @branch
(commit) @commit
(item) @item
(header) @header
(message) @message

(change kind: "new file" @plus)
(change kind: "deleted" @minus)
(change kind: "modified" @delta)
(change kind: "renamed" @delta.renamed)

(trailer
  key: (trailer_key) @trailer.key
  value: (trailer_value) @trailer.value)

[":" "=" "->" (scissors)] @punctuation.delimiter
(comment) @comment

