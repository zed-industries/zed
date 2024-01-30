vim9script

# MetaPost indent file
# Language:           MetaPost
# Maintainer:         Nicola Vitacolonna <nvitacolonna@gmail.com>
# Former Maintainers: Eugene Minkovskii <emin@mccme.ru>
# Latest Revision:    2022 Aug 12

if exists("b:did_indent")
  finish
endif

b:did_indent = 1

setlocal indentexpr=g:MetaPostIndent()
setlocal indentkeys+==end,=else,=fi,=fill,0),0]
setlocal nolisp
setlocal nosmartindent

b:undo_indent = "setl indentexpr< indentkeys< lisp< smartindent<"

# Regexps {{{
# Expressions starting indented blocks
const MP_OPEN_TAG = [
  '\<if\>',
  '\<else\%[if]\>',
  '\<for\%(\|ever\|suffixes\)\>',
  '\<begingroup\>',
  '\<\%(\|var\|primary\|secondary\|tertiary\)def\>',
  '^\s*\<begin\%(fig\|graph\|glyph\|char\|logochar\)\>',
  '[([{]',
  ]->extend(get(g:, "mp_open_tag", []))->join('\|')

# Expressions ending indented blocks
const MP_CLOSE_TAG = [
  '\<fi\>',
  '\<else\%[if]\>',
  '\<end\%(\|for\|group\|def\|fig\|char\|glyph\|graph\)\>',
  '[)\]}]'
  ]->extend(get(g:, "mp_close_tag", []))->join('\|')

# Statements that may span multiple lines and are ended by a semicolon. To
# keep this list short, statements that are unlikely to be very long or are
# not very common (e.g., keywords like `interim` or `showtoken`) are not
# included.
#
# The regex for assignments and equations (the last branch) is tricky, because
# it must not match things like `for i :=`, `if a=b`, `def...=`, etc... It is
# not perfect, but it works reasonably well.
const MP_STATEMENT = [
  '\<\%(\|un\|cut\)draw\%(dot\)\=\>',
  '\<\%(\|un\)fill\%[draw]\>',
  '\<draw\%(dbl\)\=arrow\>',
  '\<clip\>',
  '\<addto\>',
  '\<save\>',
  '\<setbounds\>',
  '\<message\>',
  '\<errmessage\>',
  '\<errhelp\>',
  '\<fontmapline\>',
  '\<pickup\>',
  '\<show\>',
  '\<special\>',
  '\<write\>',
  '\%(^\|;\)\%([^;=]*\%(' .. MP_OPEN_TAG .. '\)\)\@!.\{-}:\==',
  ]->join('\|')

# A line ends with zero or more spaces, possibly followed by a comment.
const EOL = '\s*\%($\|%\)'
# }}}

# Auxiliary functions {{{
# Returns true if (0-based) position immediately preceding `pos` in `line` is
# inside a string or a comment; returns false otherwise.

# This is the function that is called more often when indenting, so it is
# critical that it is efficient. The method we use is significantly faster
# than using syntax attributes, and more general (it does not require
# syntax_items). It is also faster than using a single regex matching an even
# number of quotes. It helps that MetaPost strings cannot span more than one
# line and cannot contain escaped quotes.
def IsCommentOrString(line: string, pos: number): bool
  var in_string = 0
  var q = stridx(line, '"')
  var c = stridx(line, '%')

  while q >= 0 && q < pos
    if c >= 0 && c < q
      if in_string # Find next percent symbol
        c = stridx(line, '%', q + 1)
      else # Inside comment
        return true
      endif
    endif
    in_string = 1 - in_string
    q = stridx(line, '"', q + 1) # Find next quote
  endwhile

  return in_string || (c >= 0 && c <= pos)
enddef

# Find the first non-comment non-blank line before the given line.
def PrevNonBlankNonComment(lnum: number): number
  var nr = prevnonblank(lnum - 1)
  while getline(nr) =~# '^\s*%'
    nr = prevnonblank(nr - 1)
  endwhile
  return nr
enddef

# Returns true if the last tag appearing in the line is an open tag; returns
# false otherwise.
def LastTagIsOpen(line: string): bool
  var o = LastValidMatchEnd(line, MP_OPEN_TAG, 0)
  if o == - 1
    return false
  endif
  return LastValidMatchEnd(line, MP_CLOSE_TAG, o) < 0
enddef

# A simple, efficient and quite effective heuristics is used to test whether
# a line should cause the next line to be indented: count the "opening tags"
# (if, for, def, ...) in the line, count the "closing tags" (endif, endfor,
# ...) in the line, and compute the difference. We call the result the
# "weight" of the line. If the weight is positive, then the next line should
# most likely be indented. Note that `else` and `elseif` are both opening and
# closing tags, so they "cancel out" in almost all cases, the only exception
# being a leading `else[if]`, which is counted as an opening tag, but not as
# a closing tag (so that, for instance, a line containing a single `else:`
# will have weight equal to one, not zero). We do not treat a trailing
# `else[if]` in any special way, because lines ending with an open tag are
# dealt with separately before this function is called (see MetaPostIndent()).
#
# Example:
#
#     forsuffixes $=a,b: if x.$ = y.$ : draw else: fill fi
#       % This line will be indented because |{forsuffixes,if,else}| > |{else,fi}| (3 > 2)
#     endfor
def Weight(line: string): number
  var o = 0
  var i = ValidMatchEnd(line, MP_OPEN_TAG, 0)
  while i > 0
    o += 1
    i = ValidMatchEnd(line, MP_OPEN_TAG, i)
  endwhile
  var c = 0
  i = matchend(line, '^\s*\<else\%[if]\>')  # Skip a leading else[if]
  i = ValidMatchEnd(line, MP_CLOSE_TAG, i)
  while i > 0
    c += 1
    i = ValidMatchEnd(line, MP_CLOSE_TAG, i)
  endwhile
  return o - c
enddef

# Similar to matchend(), but skips strings and comments.
# line: a String
def ValidMatchEnd(line: string, pat: string, start: number): number
  var i = matchend(line, pat, start)
  while i > 0 && IsCommentOrString(line, i)
    i = matchend(line, pat, i)
  endwhile
  return i
enddef

# Like s:ValidMatchEnd(), but returns the end position of the last (i.e.,
# rightmost) match.
def LastValidMatchEnd(line: string, pat: string, start: number): number
  var last_found = -1
  var i = matchend(line, pat, start)
  while i > 0
    if !IsCommentOrString(line, i)
      last_found = i
    endif
    i = matchend(line, pat, i)
  endwhile
  return last_found
enddef

def DecreaseIndentOnClosingTag(curr_indent: number): number
  var cur_text = getline(v:lnum)
  if cur_text =~# '^\s*\%(' .. MP_CLOSE_TAG .. '\)'
    return max([curr_indent - shiftwidth(), 0])
  endif
  return curr_indent
enddef
# }}}

# Main function {{{
def g:MetaPostIndent(): number
  # Do not touch indentation inside verbatimtex/btex.. etex blocks.
  if synIDattr(synID(v:lnum, 1, 1), "name") =~# '^mpTeXinsert$\|^tex\|^Delimiter'
    return -1
  endif

  # At the start of a MetaPost block inside ConTeXt, do not touch indentation
  if synIDattr(synID(prevnonblank(v:lnum - 1), 1, 1), "name") == "contextBlockDelim"
    return -1
  endif

  var lnum = PrevNonBlankNonComment(v:lnum)

  # At the start of the file use zero indent.
  if lnum == 0
    return 0
  endif

  var prev_text = getline(lnum)

  # Every rule of indentation in MetaPost is very subjective. We might get
  # creative, but things get murky very soon (there are too many corner
  # cases). So, we provide a means for the user to decide what to do when this
  # script doesn't get it. We use a simple idea: use '%>', '%<', '%=', and
  # '%!', to explicitly control indentation. The '<' and '>' symbols may be
  # repeated many times (e.g., '%>>' will cause the next line to be indented
  # twice).
  #
  # User-defined overrides take precedence over anything else.
  var j = match(prev_text, '%[<>=!]')
  if j > 0
    var i = strlen(matchstr(prev_text, '%>\+', j)) - 1
    if i > 0
      return indent(lnum) + i * shiftwidth()
    endif

    i = strlen(matchstr(prev_text, '%<\+', j)) - 1
    if i > 0
      return max([indent(lnum) - i * shiftwidth(), 0])
    endif

    if match(prev_text, '%=', j) > -1
      return indent(lnum)
    endif

    if match(prev_text, '%!', j) > -1
      return -1
    endif
  endif

  # If the reference line ends with an open tag, indent.
  #
  # Example:
  #
  # if c:
  #     0
  # else:
  #     1
  # fi if c2: % Note that this line has weight equal to zero.
  #     ...   % This line will be indented
  if LastTagIsOpen(prev_text)
    return DecreaseIndentOnClosingTag(indent(lnum) + shiftwidth())
  endif

  # Lines with a positive weight are unbalanced and should likely be indented.
  #
  # Example:
  #
  # def f = enddef for i = 1 upto 5: if x[i] > 0: 1 else: 2 fi
  #     ... % This line will be indented (because of the unterminated `for`)
  if Weight(prev_text) > 0
    return DecreaseIndentOnClosingTag(indent(lnum) + shiftwidth())
  endif

  # Unterminated statements cause indentation to kick in.
  #
  # Example:
  #
  # draw unitsquare
  #     withcolor black; % This line is indented because of `draw`.
  # x := a + b + c
  #     + d + e;         % This line is indented because of `:=`.
  #
  var i = LastValidMatchEnd(prev_text, MP_STATEMENT, 0)
  if i >= 0 # Does the line contain a statement?
    if ValidMatchEnd(prev_text, ';', i) < 0 # Is the statement unterminated?
      return indent(lnum) + shiftwidth()
    else
      return DecreaseIndentOnClosingTag(indent(lnum))
    endif
  endif

  # Deal with the special case of a statement spanning multiple lines. If the
  # current reference line L ends with a semicolon, search backwards for
  # another semicolon or a statement keyword. If the latter is found first,
  # its line is used as the reference line for indenting the current line
  # instead of L.
  #
  #  Example:
  #
  #  if cond:
  #    draw if a: z0 else: z1 fi
  #        shifted S
  #        scaled T;      % L
  #
  #    for i = 1 upto 3:  % <-- Current line: this gets the same indent as `draw ...`
  #
  # NOTE: we get here only if L does not contain a statement (among those
  # listed in g:MP_STATEMENT).
  if ValidMatchEnd(prev_text, ';' .. EOL, 0) >= 0 # L ends with a semicolon
    var stm_lnum = PrevNonBlankNonComment(lnum)
    while stm_lnum > 0
      prev_text = getline(stm_lnum)
      var sc_pos = LastValidMatchEnd(prev_text, ';', 0)
      var stm_pos = ValidMatchEnd(prev_text, MP_STATEMENT, sc_pos)
      if stm_pos > sc_pos
        lnum = stm_lnum
        break
      elseif sc_pos > stm_pos
        break
      endif
      stm_lnum = PrevNonBlankNonComment(stm_lnum)
    endwhile
  endif

  return DecreaseIndentOnClosingTag(indent(lnum))
enddef
# }}}

# vim: sw=2 fdm=marker
