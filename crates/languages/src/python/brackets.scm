("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
(((string_start) @open (string_end) @close) (#set! rainbow.exclude))
