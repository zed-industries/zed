" Vim plugin for using Vim as manpager.
" Maintainer: Enno Nagel <ennonagel+vim@gmail.com>
" Last Change: 2022 Oct 17

if exists('g:loaded_manpager_plugin')
  finish
endif
let g:loaded_manpager_plugin = 1

" Set up the current buffer (likely read from stdin) as a manpage
command MANPAGER call s:ManPager()

function s:ManPager()
  " global options, keep these to a minimum to avoid side effects
  if &compatible
    set nocompatible
  endif
  if exists('+viminfofile')
    set viminfofile=NONE
  endif
  syntax on

  " Make this an unlisted, readonly scratch buffer
  setlocal buftype=nofile noswapfile bufhidden=hide nobuflisted readonly

  " Ensure text width matches window width
  setlocal foldcolumn& nofoldenable nonumber norelativenumber

  " In case Vim was invoked with -M
  setlocal modifiable

  " Emulate 'col -b'
  exe 'silent! keepj keepp %s/\v(.)\b\ze\1?//e' .. (&gdefault ? '' : 'g')

  " Remove ansi sequences
  exe 'silent! keepj keepp %s/\v\e\[%(%(\d;)?\d{1,2})?[mK]//e' .. (&gdefault ? '' : 'g')

  " Remove empty lines above the header
  call cursor(1, 1)
  let n = search(".*(.*)", "c")
  if n > 1
    exe "1," . n-1 . "d"
  endif

  " Finished preprocessing the buffer, prevent any further modifications
  setlocal nomodified nomodifiable

  " Set filetype to man even if ftplugin is disabled
  setlocal filetype=man
  runtime ftplugin/man.vim
endfunction
