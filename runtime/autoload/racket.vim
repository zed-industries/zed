" Maintainer: D. Ben Knoble <ben.knoble+github@gmail.com>
" URL: https://github.com/benknoble/vim-racket
" Last Change: 2023 Sep 22
vim9script

def MakePatternFromLiterals(xs: list<string>): string
  return printf('\V%s', xs->mapnew((_, v) => escape(v, '\'))->join('\|'))
enddef

const openers = ['(', '[', '{']
const closers = {'(': ')', '[': ']', '{': '}'}
const brackets_pattern: string = closers->items()->flattennew()->MakePatternFromLiterals()

# transliterated from a modified copy of src/indent.c

export def Indent(): number
  if InHerestring(v:lnum)
    return -1
  endif
  # Indent from first column to avoid odd results from nested forms.
  cursor(v:lnum, 1)
  const bracket = FindBracket()
  if bracket == null_dict || !bracket.found
    return -1
  endif

  # assert_report(printf('{lnum: %d, str: %s, found: %s, line: %d, column: %d}',
  #   v:lnum, getline(bracket.line)[bracket.column - 1], bracket.found, bracket.line, bracket.column))
  # N.B. Column =/= Line Index; Columns start at 1
  const amount: number = bracket.column
  const line = getline(bracket.line)

  const lw = Lispword(line[bracket.column :])
  if !IsForFold(lw) # skip: see comments about for/fold special case below
    # "Extra trick"
    var current = prevnonblank(v:lnum - 1)
    while current > bracket.line
      cursor(current, 1)
      if getline(current) !~# '^\s*;' && synID(current, 1, 0)->synIDattr('name') !~? 'string' && FindBracket() == bracket
        return indent(current)
      endif
      current = prevnonblank(current - 1)
    endwhile
    cursor(v:lnum, 1)
  endif

  if index(openers, line[bracket.column - 1]) >= 0 && !empty(lw)
    # Special case for/fold &co. The iterator clause (2nd form) is indented
    # under the accumulator clause (1st form). Everything else is standard.
    const start_of_first_form = match(line[bracket.column :], MakePatternFromLiterals(openers))
    # assert_report(printf('{line: %s}', line))
    # assert_report(printf('{start: %s}', start_of_first_form >= 0 ? line[bracket.column + start_of_first_form :] : '<NULL>'))
    if IsForFold(lw) && IsSecondForm(bracket.line, bracket.column, v:lnum) && start_of_first_form >= 0
      return amount + start_of_first_form
    else
      # Lispword, but not for/fold second form (or first form couldn't be
      # found): indent like define or lambda.
      # 2 extra indent, but subtract 1 for columns starting at 1.
      # Current vim9 doesn't constant fold "x + 2 - 1", so write "x + 1"
      return amount + 1
    endif
  else
    # assert_report(printf('{line: %s}', line[bracket.column :]))
    return amount + IndentForContinuation(bracket.line, bracket.column, line[bracket.column :])
  endif
enddef

def InHerestring(start: number): bool
  return synID(start, col([start, '$']) - 1, 0)->synIDattr('name') =~? 'herestring'
enddef

def FindBracket(): dict<any>
  const paren = FindMatch('(', ')')
  const square = FindMatch('\[', ']')
  const curly = FindMatch('{', '}')
  return null_dict
    ->MatchMax(paren)
    ->MatchMax(square)
    ->MatchMax(curly)
enddef

def Lispword(line: string): string
  # assume keyword on same line as opener
  const word: string = matchstr(line, '^\s*\k\+\>')->trim()
  # assert_report(printf('line: %s; word: %s', line, word))
  # assert_report(&l:lispwords->split(',')->index(word) >= 0 ? 't' : 'f')
  return &l:lispwords->split(',')->index(word) >= 0 ? word : ''
enddef

# line contains everything on line_nr after column
def IndentForContinuation(line_nr: number, column: number, line: string): number
  const end = len(line)
  var indent = match(line, '[^[:space:]]')
  # first word is a string or some other literal (or maybe a form); assume that
  # the current line is outside such a thing
  if indent < end && ['"', '#']->index(line[indent]) >= 0
    return indent
  endif
  if indent < end && ["'", '`']->index(line[indent]) >= 0
    # could be a form or a word. Advance one and see.
    ++indent
  endif
  if indent < end && ['(', '[', '{']->index(line[indent]) >= 0
    # there's a form; assume outside, but need to skip it to see if any others
    cursor(line_nr, column + indent + 1)
    # assert_report(getline(line_nr)[column + indent :])
    normal! %
    const [_, matched_line, matched_col, _, _] = getcursorcharpos()
    if line_nr != matched_line || matched_col == column + indent + 1
      return indent
    endif
    indent = matched_col - column
  endif
  var in_delim: bool
  var quoted: bool
  while indent < end && (line[indent] !~# '\s' || in_delim || quoted)
    if line[indent] == '\' && !in_delim
      quoted = true
    else
      quoted = false
    endif
    if line[indent] == '|' && !quoted
      in_delim = !in_delim
    endif
    ++indent
  endwhile
  # not handling newlines in first words
  if quoted || in_delim
    return 0
  endif
  # no other word on this line
  if indent == end
    return 0
  endif
  # find beginning of next word
  indent += match(line[indent :], '[^[:space:]]')
  return indent
enddef

def FindMatch(start: string, end: string): dict<any>
  # TODO too slow…
  # could try replicating C? might have false positives. Or make "100"
  # configurable number: for amounts of indent bodies, we're still fast enough…
  const [linenr, column] = searchpairpos(start, '', end, 'bnzW',
    () =>
      synID(line('.'), col('.'), 0)->synIDattr('name') =~? 'char\|string\|comment',
    line('.') > 100 ? line('.') - 100 : 0)
  if linenr > 0 && column > 0
    return {found: true, line: linenr, column: column}
  else
    return {found: false, line: linenr, column: column}
  endif
enddef

def MatchMax(left: dict<any>, right: dict<any>): dict<any>
  if left == null_dict || !left.found
    return right
  endif
  if right == null_dict || !right.found
    return left
  endif
  # left and right non-null, both found
  return PosLT(left, right) ? right : left
enddef

def PosLT(left: dict<any>, right: dict<any>): bool
  return left.line != right.line
     \ ? left.line < right.line
     \ : (left.column != right.column && left.column < right.column)
enddef

def IsForFold(word: string): bool
  return ['for/fold', 'for/foldr', 'for*/fold', 'for*/foldr']->index(word) >= 0
enddef

def IsSecondForm(blnum: number, bcol: number, vlnum: number): bool
  var forms_seen: number # "top-level" (inside for/fold) counter only
  var [lnum, col] = [blnum, bcol + 1]
  cursor(lnum, col)
  var stack: list<string> = []

  while lnum <= vlnum
    const found = search(brackets_pattern, '', vlnum, 0, () =>
      synID(line('.'), col('.'), 0)->synIDattr('name') =~? 'char\|string\|comment')
    if found <= 0
      break
    endif
    const pos = getcursorcharpos()
    lnum = pos[1]
    col = pos[2]
    var current_char = getline(lnum)[col - 1]
    # assert_report(printf('search: %d, %d: %s', lnum, col, current_char))
    # assert_report(printf('forms seen post-search: %d', forms_seen))
    if index(openers, current_char) >= 0
      insert(stack, current_char)
    elseif !empty(stack) && current_char ==# closers[stack[0]]
      stack = stack[1 :]
      if empty(stack)
        ++forms_seen
      endif
    else
      # parse failure of some kind: not an opener or not the correct closer
      return false
    endif
    # assert_report(printf('forms seen pre-check: %d', forms_seen))
    if forms_seen > 2
      return false
    endif
  endwhile

  # assert_report(printf('forms seen pre-return: %d', forms_seen))
  return forms_seen == 2 || (forms_seen == 1 && !empty(stack))
enddef
