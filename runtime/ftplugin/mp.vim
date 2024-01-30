vim9script

# Vim filetype plugin file
# Language:           MetaPost
# Maintainer:         Nicola Vitacolonna <nvitacolonna@gmail.com>
# Former Maintainers: Nikolai Weibull <now@bitwi.se>
# Latest Revision:    2022 Aug 12
#                     2024 Jan 14 by Vim Project (browsefilter)

if exists("b:did_ftplugin")
  finish
endif

b:did_ftplugin = 1
b:undo_ftplugin = "setl com< cms< fo< sua< inc< def< ofu<"

setlocal comments=:%
setlocal commentstring=%\ %s
setlocal formatoptions+=cjroql2
setlocal formatoptions-=t
setlocal omnifunc=syntaxcomplete#Complete
setlocal suffixesadd=.mp,.mpiv,.mpvi,.mpxl

&l:include = '\<\%(input\|loadmodule\)\>'  # loadmodule is from MetaFun
&l:define = '\<\%(let\|newinternal\|interim\|def\|vardef\)\>\|\<\%(primary\|secondary\|tertiary\)def\>\s*[^ .]\+'

g:omni_syntax_group_include_mp = 'mf\w\+,mp\w\+,metafun\w\+'
g:omni_syntax_group_exclude_mp = 'mfTodoComment'

var fignum: number

def FixBeginfigs()
  fignum = 1
  g/^\s*beginfig(\d*)\s*;\(\s*%.*\)\=$/s/^.\{-};/\='beginfig(' .. fignum .. ');'/ | ++fignum
enddef

command! -buffer -nargs=0 -bar FixBeginfigs FixBeginfigs()

if exists("g:loaded_matchit") && !exists("b:match_words")
  b:match_ignorecase = 0
  b:match_skip = 'synIDattr(synID(line("."), col("."), 1), "name") =~# "^mf\\%(Comment\\|String\\|\\)$\\|^mpTeXinsert$"'
  b:match_words = '\<if\>:\<else\%[if]\>:\<fi\>,'
  ..              '\<for\%(\|suffixes\|ever\)\>:\<exit\%(if\|unless\)\>:\<endfor\>,'
  ..              '\<\%(\|var\|primary\|secondary\|tertiary\)def\>:\<enddef\>,'
  ..              '\<begin\(\a\+\)\>:end\1,'
  ..              '\<beginlogochar\>:\<endchar\>'
  b:undo_ftplugin ..= "| unlet! b:match_ignorecase b:match_words b:match_skip"
endif

if !get(g:, 'no_mp_maps', 0) && !get(g:, 'no_plugin_maps', 0)
  const mp_regex = {
    'beginsection': '^\s*\%(\%(\|var\|primary\|secondary\|tertiary\)def\|begin\%(fig\|char\|logochar\|glyph\|graph\)\)\>',
    'endsection':   '^\s*\%(enddef\|end\%(fig\|char\|glyph\|graph\)\)\>',
    'beginblock':   '^\s*\%(begingroup\|if\|for\%(\|suffixes\|ever\)\)\>',
    'endblock':     '^\s*\%(endgroup\|fi\|endfor\)\>'}

  def MoveAround(count: number, what: string, flags: string)
    search(mp_regex[what], flags .. 's')  # 's' sets previous context mark
    var i = 2
    while i <= count
      search(mp_regex[what], flags)
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

  for mapping in ["[[", "]]", "[]", "][", "[{", "]}"]
    b:undo_ftplugin ..= printf(" | silent! execute 'nunmap <buffer> %s'", mapping)
    b:undo_ftplugin ..= printf(" | silent! execute 'vunmap <buffer> %s'", mapping)
  endfor
endif

if (has('gui_win32') || has('gui_gtk')) && !exists('b:browsefilter')
  b:browsefilter = "MetaPost Source Files (*.mp)\t*.mp\n"
  if has("win32")
    b:browsefilter ..= "All Files (*.*)\t*\n"
  else
    b:browsefilter ..= "All Files (*)\t*\n"
  endif
  b:undo_ftplugin ..= ' | unlet! b:browsefilter'
endif

# vim: sw=2 fdm=marker
