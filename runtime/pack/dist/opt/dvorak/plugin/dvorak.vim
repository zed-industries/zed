" When using a dvorak keyboard this file may be of help to you.
" These mappings have been made by Lawrence Kesteloot <kesteloo@cs.unc.edu>.
" What they do is that the most often used keys, like hjkl, are put in a more
" easy to use position.
" It may take some time to learn using this.

if exists("g:loaded_dvorak_plugin")
  finish
endif
let g:loaded_dvorak_plugin = 1

" Key to go into dvorak mode:
map ,d :runtime dvorak/enable.vim<CR>

" Key to get out of dvorak mode:
map ,q :runtime dvorak/disable.vim<CR>
