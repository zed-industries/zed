vim9script

# Vim filetype plugin file
# Language:           ConTeXt typesetting engine
# Maintainer:         Nicola Vitacolonna <nvitacolonna@gmail.com>
# Former Maintainers: Nikolai Weibull <now@bitwi.se>
# Latest Revision:    2023 Dec 26

if exists("b:did_ftplugin")
  finish
endif

import autoload '../autoload/context.vim'

b:did_ftplugin = 1

if !exists('current_compiler')
  compiler context
endif

b:undo_ftplugin = "setl com< cms< def< inc< sua< fo< ofu<"

setlocal comments=b:%D,b:%C,b:%M,:%
setlocal commentstring=%\ %s
setlocal formatoptions+=tjcroql2
setlocal omnifunc=context.Complete
setlocal suffixesadd=.tex,.mkxl,.mkvi,.mkiv,.mkii

&l:define = '\\\%([egx]\|char\|mathchar\|count\|dimen\|muskip\|skip\|toks\)\='
..          'def\|\\font\|\\\%(future\)\=let'
..          '\|\\new\%(count\|dimen\|skip\|muskip\|box\|toks\|read\|write'
..          '\|fam\|insert\|if\)'

&l:include = '^\s*\\\%(input\|component\|product\|project\|environment\)'

if exists("g:loaded_matchit") && !exists("b:match_words")
  b:match_ignorecase = 0
  b:match_skip = 'r:\\\@<!\%(\\\\\)*%'
  b:match_words = '(:),\[:],{:},\\(:\\),\\\[:\\],\\start\(\a\+\):\\stop\1'
  b:undo_ftplugin ..= "| unlet! b:match_ignorecase b:match_words b:match_skip"
endif

if !get(g:, 'no_context_maps', 0) && !get(g:, 'no_plugin_maps', 0)
  const context_regex = {
    'beginsection': '\\\%(start\)\=\%(\%(sub\)*section\|\%(sub\)*subject\|chapter\|part\|component\|product\|title\)\>',
    'endsection':   '\\\%(stop\)\=\%(\%(sub\)*section\|\%(sub\)*subject\|chapter\|part\|component\|product\|title\)\>',
    'beginblock':   '\\\%(start\|setup\|define\)',
    'endblock':     '\\\%(stop\|setup\|define\)',
    }

  def UndoMap(mapping: string, modes: string)
    for mode in modes
      b:undo_ftplugin ..= printf(" | silent! execute '%sunmap <buffer> %s'", mode, mapping)
    endfor
  enddef

  def MoveAround(count: number, what: string, flags: string)
    search(context_regex[what], flags .. 's')  # 's' sets previous context mark
    var i = 2
    while i <= count
      search(context_regex[what], flags)
      i += 1
    endwhile
  enddef

  # Macros to move around
  nnoremap <silent><buffer> [[ <scriptcmd>MoveAround(v:count1, "beginsection", "bW")<cr>
  vnoremap <silent><buffer> [[ <scriptcmd>MoveAround(v:count1, "beginsection", "bW")<cr>
  nnoremap <silent><buffer> ]] <scriptcmd>MoveAround(v:count1, "beginsection", "W") <cr>
  vnoremap <silent><buffer> ]] <scriptcmd>MoveAround(v:count1, "beginsection", "W") <cr>
  nnoremap <silent><buffer> [] <scriptcmd>MoveAround(v:count1, "endsection",   "bW")<cr>
  vnoremap <silent><buffer> [] <scriptcmd>MoveAround(v:count1, "endsection",   "bW")<cr>
  nnoremap <silent><buffer> ][ <scriptcmd>MoveAround(v:count1, "endsection",   "W") <cr>
  vnoremap <silent><buffer> ][ <scriptcmd>MoveAround(v:count1, "endsection",   "W") <cr>
  nnoremap <silent><buffer> [{ <scriptcmd>MoveAround(v:count1, "beginblock",   "bW")<cr>
  vnoremap <silent><buffer> [{ <scriptcmd>MoveAround(v:count1, "beginblock",   "bW")<cr>
  nnoremap <silent><buffer> ]} <scriptcmd>MoveAround(v:count1, "endblock",     "W") <cr>
  vnoremap <silent><buffer> ]} <scriptcmd>MoveAround(v:count1, "endblock",     "W") <cr>

  for mapping in ['[[', ']]', '[]', '][', '[{', ']}']
    UndoMap(mapping, 'nv')
  endfor

  # Other useful mappings
  const tp_regex = '?^$\|^\s*\\\(item\|start\|stop\|blank\|\%(sub\)*section\|chapter\|\%(sub\)*subject\|title\|part\)'

  def TeXPar()
    cursor(search(tp_regex, 'bcW') + 1, 1)
    normal! V
    cursor(search(tp_regex, 'W') - 1, 1)
  enddef

  # Reflow paragraphs with mappings like gqtp ("gq TeX paragraph")
  onoremap <silent><buffer> tp <scriptcmd>TeXPar()<cr>
  # Select TeX paragraph
  vnoremap <silent><buffer> tp <scriptcmd>TeXPar()<cr>

  # $...$ text object
  onoremap <silent><buffer> i$ <scriptcmd>normal! T$vt$<cr>
  onoremap <silent><buffer> a$ <scriptcmd>normal! F$vf$<cr>
  vnoremap <buffer> i$ T$ot$
  vnoremap <buffer> a$ F$of$

  for mapping in ['tp', 'i$', 'a$']
    UndoMap(mapping, 'ov')
  endfor
endif

# Commands for asynchronous typesetting
command! -buffer -nargs=? -complete=buffer ConTeXt          context.Typeset(<q-args>)
command! -buffer -nargs=0                  ConTeXtLog       context.Log('%')
command!         -nargs=0                  ConTeXtJobStatus context.JobStatus()
command!         -nargs=0                  ConTeXtStopJobs  context.StopJobs()

# vim: sw=2 fdm=marker
