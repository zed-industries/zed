vim9script

# Vim syntax file
# Language:           ConTeXt typesetting engine
# Maintainer:         Nicola Vitacolonna <nvitacolonna@gmail.com>
# Former Maintainers: Nikolai Weibull <now@bitwi.se>
# Latest Revision:    2023 Dec 26

if exists("b:current_syntax")
  finish
endif

# Dictionary of (filetype, group) pairs to highlight between \startGROUP \stopGROUP.
var context_include = get(b:, 'context_include', get(g:, 'context_include', {'xml': 'XML'}))

# Deprecation warning
if type(context_include) ==# type([])
  echomsg "[ConTeXt] b:context_include/g:context_include must be Dictionaries."
  context_include = {'xml': 'XML'}
endif

syn iskeyword @,48-57,_,!,?,a-z,A-Z,192-255

syn spell toplevel

runtime! syntax/shared/context-data-context.vim
runtime! syntax/shared/context-data-interfaces.vim
runtime! syntax/shared/context-data-tex.vim

syn match contextCommand '\\\k\+\>' display contains=@NoSpell

# ConTeXt options, i.e., [...] blocks
syn region  contextOptions    matchgroup=contextDelimiter start='\[' end=']\|\ze\\stop' skip='\\\[\|\\\]' contains=TOP,@Spell

# Highlight braces
syn match   contextDelimiter  '[{}]'

# Comments
syn match   contextComment    '\%(\_^\|[^\\]\)\%(\\\\\)*\zs%.*$' display contains=contextTodo,contextMagicLine
syn match   contextComment    '^\s*%[CDM].*$'                    display contains=contextTodo,contextMagicLine
syn keyword contextTodo       TODO FIXME XXX NOTE                contained
syn match   contextMagicLine  '^\s*%\s*!TEX.*$'                  contained

syn match   contextBlockDelim '\\\%(start\|stop\)\k\+' contains=@NoSpell

syn region  contextEscaped    matchgroup=contextPreProc start='\\type\%(\s*\|\n\)*\z([^A-Za-z%]\)' end='\z1'
syn region  contextEscaped    matchgroup=contextPreProc start='\\type\=\%(\s\|\n\)*{' end='}'
syn region  contextEscaped    matchgroup=contextPreProc start='\\type\=\%(\s*\|\n\)*<<' end='>>'
syn region  contextEscaped    matchgroup=contextPreProc
                              \ start='\\start\z(\a*\%(typing\|typen\)\)'
                              \ end='\\stop\z1' contains=contextComment keepend
syn region  contextEscaped    matchgroup=contextPreProc start='\\\h\+Type\%(\s\|\n\)*{' end='}'
syn region  contextEscaped    matchgroup=contextPreProc start='\\Typed\h\+\%(\s\|\n\)*{' end='}'

syn match   contextBuiltin    '\\unexpanded\>' display contains=@NoSpell

# \unprotect... \protect regions
syn region  contextUnprotect  matchgroup=contextBuiltin start='\\unprotect' end='\\protect' contains=TOP
syn match   contextSequence   '\\[a-zA-Z]*[@_!?]\+[a-zA-Z@_!?]*' contains=@NoSpell contained containedin=contextUnprotect

# Math
syn match  contextMathCmd '\\m\%(ath\%(ematics\)\=\)\=\>'
syn region contextInlineMath matchgroup=contextMathDelim start='\$' skip='\\\\\|\\\$' end='\$'
syn region contextDisplayMath matchgroup=contextMathDelim start='\$\$' skip='\\\\\|\\\$' end='\$\$' keepend
syn region contextDisplayMath matchgroup=contextBlockDelim start='\\startformula' end='\\stopformula' contains=TOP

# MetaFun
b:mp_metafun = 1
syn include @mpTop syntax/mp.vim
unlet b:current_syntax

syn region  contextMPGraphic  matchgroup=contextBlockDelim
      \ start='\\start\z(MP\%(clip\|code\|definitions\|drawing\|environment\|extensions\|inclusions\|initializations\|page\|\)\)\>.*$'
      \ end='\\stop\z1'
      \ contains=@mpTop,@NoSpell
syn region  contextMPGraphic  matchgroup=contextBlockDelim
      \ start='\\start\z(\%(\%[re]usable\|use\|unique\|static\)MPgraphic\|staticMPfigure\|uniqueMPpagegraphic\)\>.*$'
      \ end='\\stop\z1'
      \ contains=@mpTop,@NoSpell

# Lua
syn include @luaTop syntax/lua.vim
unlet b:current_syntax

syn region  contextLuaCode    matchgroup=contextBlockDelim
      \ start='\\startluacode\>'
      \ end='\\stopluacode\>' keepend
      \ contains=@luaTop,@NoSpell
syn match   contextDirectLua  "\\\%(directlua\|ctxlua\)\>\%(\s*%.*$\)\="
      \ nextgroup=contextBeginEndLua skipwhite skipempty
      \ contains=contextComment
syn region  contextBeginEndLua matchgroup=contextSpecial
      \ start="{" end="}" skip="\\[{}]" keepend
      \ contained contains=@luaTop,@NoSpell

for synname in keys(context_include)
  execute 'syn include @' .. synname .. 'Top' 'syntax/' .. synname .. '.vim'
  unlet b:current_syntax
  execute 'syn region context' .. context_include[synname] .. 'Code'
        \ 'matchgroup=contextBlockDelim'
        \ 'start=+\\start' .. context_include[synname] .. '\w*+'
        \ 'end=+\\stop' .. context_include[synname] .. '\w*+'
        \ 'contains=@' .. synname .. 'Top,@NoSpell'
endfor

syn match   contextSectioning '\\\%(start\|stop\)\=\%(\%(sub\)*section\|\%(sub\)*subject\|chapter\|part\|component\|product\|title\)\>' contains=@NoSpell

syn match   contextSpecial    '\\par\>\|-\{2,3}\||[<>/]\=|'                     contains=@NoSpell
syn match   contextSpecial    /\\[`'"]/
syn match   contextSpecial    +\\char\%(\d\{1,3}\|'\o\{1,3}\|"\x\{1,2}\)\>+     contains=@NoSpell
syn match   contextSpecial    '\^\^.'
syn match   contextSpecial    '`\%(\\.\|\^\^.\|.\)'

syn match   contextStyle      '\\\%(em\|ss\|hw\|cg\|mf\)\>'                     contains=@NoSpell
syn match   contextFont       '\\\%(CAP\|Cap\|cap\|Caps\|kap\|nocap\)\>'        contains=@NoSpell
syn match   contextFont       '\\\%(Word\|WORD\|Words\|WORDS\)\>'               contains=@NoSpell
syn match   contextFont       '\\\%(vi\{1,3}\|ix\|xi\{0,2}\)\>'                 contains=@NoSpell
syn match   contextFont       '\\\%(tf\|b[si]\|s[cl]\|os\)\%(xx\|[xabcd]\)\=\>' contains=@NoSpell

hi def link contextBlockDelim Keyword
hi def link contextBuiltin    Keyword
hi def link contextCommand    Keyword
hi def link contextComment    Comment
hi def link contextDelimiter  Delimiter
hi def link contextDirectLua  Keyword
hi def link contextEscaped    String
hi def link contextFont       contextType
hi def link contextKeyword    Keyword
hi def link contextInlineMath String
hi def link contextMagicLine  PreProc
hi def link contextMathCmd    Identifier
hi def link contextMathDelim  Delimiter
hi def link contextOptions    Typedef
hi def link contextPreProc    PreProc
hi def link contextSectioning PreProc
hi def link contextSequence   Identifier
hi def link contextSpecial    Special
hi def link contextStyle      contextType
hi def link contextTodo       Todo
hi def link contextType       Type

b:current_syntax = 'context'

# vim: sw=2 fdm=marker
