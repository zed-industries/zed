" Vim syntax file
" Language:     EditorConfig
" Maintainer:   Gregory Anders <greg@gpanders.com>
" Last Change:  2023-07-20

if exists('b:current_syntax')
  finish
endif

runtime! syntax/dosini.vim
unlet! b:current_syntax

syntax match editorconfigUnknownProperty "^\s*\zs[a-zA-Z0-9_-]\+\ze\s*="

syntax keyword editorconfigProperty root charset end_of_line indent_style
syntax keyword editorconfigProperty indent_size tab_width max_line_length
syntax keyword editorconfigProperty trim_trailing_whitespace insert_final_newline

hi def link editorconfigProperty dosiniLabel

let b:current_syntax = 'editorconfig'
