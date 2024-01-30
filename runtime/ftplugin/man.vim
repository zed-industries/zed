" Vim filetype plugin file
" Language:	man
" Maintainer:	Jason Franklin <vim@justemail.net>
" Maintainer:	SungHyun Nam <goweol@gmail.com>
" Autoload Split: Bram Moolenaar
" Last Change: 	2023 Mar 21

" To make the ":Man" command available before editing a manual page, source
" this script from your startup vimrc file.

" If 'filetype' isn't "man", we must have been called to define ":Man" and not
" to do the filetype plugin stuff.
if &filetype == "man"

  " Only do this when not done yet for this buffer
  if exists("b:did_ftplugin")
    finish
  endif
  let b:did_ftplugin = 1
endif

let s:cpo_save = &cpo
set cpo-=C

if &filetype == "man"
  " Allow hyphen, plus, colon, dot, and commercial at in manual page name.
  " Parentheses are not here but in dist#man#PreGetPage()
  setlocal iskeyword=48-57,_,a-z,A-Z,-,+,:,.,@-@
  let b:undo_ftplugin = "setlocal iskeyword<"

  " Add mappings, unless the user didn't want this.
  if !exists("no_plugin_maps") && !exists("no_man_maps")
    if !hasmapto('<Plug>ManBS')
      nmap <buffer> <LocalLeader>h <Plug>ManBS
      let b:undo_ftplugin = b:undo_ftplugin
	    \ . '|silent! nunmap <buffer> <LocalLeader>h'
    endif
    nnoremap <buffer> <Plug>ManBS :%s/.\b//g<CR>:setl nomod<CR>''

    nnoremap <buffer> <silent> <c-]> :call dist#man#PreGetPage(v:count)<CR>
    nnoremap <buffer> <silent> <c-t> :call dist#man#PopPage()<CR>
    nnoremap <buffer> <silent> q :q<CR>

    " Add undo commands for the maps
    let b:undo_ftplugin = b:undo_ftplugin
	  \ . '|silent! nunmap <buffer> <Plug>ManBS'
	  \ . '|silent! nunmap <buffer> <c-]>'
	  \ . '|silent! nunmap <buffer> <c-t>'
	  \ . '|silent! nunmap <buffer> q'
  endif

  if exists('g:ft_man_folding_enable') && (g:ft_man_folding_enable == 1)
    setlocal foldmethod=indent foldnestmax=1 foldenable
    let b:undo_ftplugin = b:undo_ftplugin
	  \ . '|silent! setl fdm< fdn< fen<'
  endif

endif

if exists(":Man") != 2
  com -nargs=+ -complete=shellcmd Man call dist#man#GetPage(<q-mods>, <f-args>)
  nmap <Leader>K :call dist#man#PreGetPage(0)<CR>
  nmap <Plug>ManPreGetPage :call dist#man#PreGetPage(0)<CR>
endif

let &cpo = s:cpo_save
unlet s:cpo_save

" vim: set sw=2 ts=8 noet:
