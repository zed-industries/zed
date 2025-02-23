;; Match common comment annotations
((line_comment) @comment.note
  (#match? @comment.note "^\\s*//\\s*(NOTE|NOTES|XXX|HACK|FIXME|TODO|BUG|BUGS|REVIEW|OPTIMIZE|QUESTION|INFO)\\b"))

((block_comment) @comment.note
  (#match? @comment.note "/\\*\\s*(NOTE|NOTES|XXX|HACK|FIXME|TODO|BUG|BUGS|REVIEW|OPTIMIZE|QUESTION|INFO)\\b"))

;; Match doc comment annotations
((line_comment (doc_comment)) @comment.doc.note
  (#match? @comment.doc.note "^\\s*///\\s*(NOTE|NOTES|XXX|HACK|FIXME|TODO|BUG|BUGS|REVIEW|OPTIMIZE|QUESTION|INFO)\\b"))

((block_comment (doc_comment)) @comment.doc.note
  (#match? @comment.doc.note "/\\*\\*\\s*(NOTE|NOTES|XXX|HACK|FIXME|TODO|BUG|BUGS|REVIEW|OPTIMIZE|QUESTION|INFO)\\b")) 