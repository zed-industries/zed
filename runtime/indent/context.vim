vim9script

# Language:           ConTeXt typesetting engine
# Maintainer:         Nicola Vitacolonna <nvitacolonna@gmail.com>
# Former Maintainers: Nikolai Weibull <now@bitwi.se>
# Latest Revision:    2023 Dec 26

if exists("b:did_indent")
  finish
endif

# Load MetaPost indentation script (this will also set b:did_indent)
runtime! indent/mp.vim

setlocal indentexpr=ConTeXtIndent()

b:undo_indent = "setl indentexpr<"

def PrevNotComment(l: number): number
  var prevlnum = prevnonblank(l)

  while prevlnum > 0 && getline(prevlnum) =~# '^\s*%'
    prevlnum = prevnonblank(prevlnum - 1)
  endwhile

  return prevlnum
enddef

def FindPair(pstart: string, pmid: string, pend: string): number
  cursor(v:lnum, 1)
  return indent(searchpair(pstart, pmid, pend, 'bWn',
    'synIDattr(synID(line("."), col("."), 0), "name") =~? "string\\|comment"'))
enddef

def ConTeXtIndent(): number
  # Use MetaPost rules inside MetaPost graphic environments
  if len(synstack(v:lnum, 1)) > 0 &&
    synIDattr(synstack(v:lnum, 1)[0], "name") ==# 'contextMPGraphic'
    return g:MetaPostIndent()
  endif

  const prevlnum = PrevNotComment(v:lnum - 1)
  const prevind  = indent(prevlnum)
  const prevline = getline(prevlnum)
  const currline = getline(v:lnum)

  # If the current line starts with ], match indentation.
  if currline =~# '^\s*\]'
    return FindPair('\[', '', '\]')
  endif

  # If the current line starts with }, match indentation.
  if currline =~# '^\s*}'
    return FindPair('{', '', '}')
  endif

  # If the previous line ends with [ or { (possibly followed by a comment) then indent.
  if prevline =~# '[{[]\s*\%(%.*\)\=$'
    return prevind + shiftwidth()
  endif

  return -1
enddef

# vim: sw=2 fdm=marker
