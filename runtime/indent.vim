" Vim support file to switch on loading indent files for file types
"
" Maintainer:	The Vim Project <https://github.com/vim/vim>
" Last Change:	2023 Aug 10
" Former Maintainer:	Bram Moolenaar <Bram@vim.org>

if exists("did_indent_on")
  finish
endif
let did_indent_on = 1

augroup filetypeindent
  au FileType * call s:LoadIndent()
augroup END

def s:LoadIndent()
  if exists("b:undo_indent")
    legacy exe b:undo_indent
    unlet! b:undo_indent b:did_indent
  endif
  var s = expand("<amatch>")
  if s != ""
    if exists("b:did_indent")
      unlet b:did_indent
    endif

    # When there is a dot it is used to separate filetype names.  Thus for
    # "aaa.bbb" load "indent/aaa.vim" and then "indent/bbb.vim".
    for name in split(s, '\.')
      exe 'runtime! indent/' .. name .. '.vim'
    endfor
  endif
enddef
