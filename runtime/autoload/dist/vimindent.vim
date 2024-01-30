vim9script

# Language:     Vim script
# Maintainer:   github user lacygoill
# Last Change:  2023 Jun 29

# NOTE: Whenever you change the code, make sure the tests are still passing:
#
#     $ cd runtime/indent/
#     $ make clean; make test || vimdiff testdir/vim.{ok,fail}

# Config {{{1

const TIMEOUT: number = get(g:, 'vim_indent', {})
    ->get('searchpair_timeout', 100)

def IndentMoreInBracketBlock(): number # {{{2
    if get(g:, 'vim_indent', {})
            ->get('more_in_bracket_block', false)
        return shiftwidth()
    else
        return 0
    endif
enddef

def IndentMoreLineContinuation(): number # {{{2
    var n: any = get(g:, 'vim_indent', {})
        # We inspect `g:vim_indent_cont` to stay backward compatible.
        ->get('line_continuation', get(g:, 'vim_indent_cont', shiftwidth() * 3))

    if n->typename() == 'string'
        return n->eval()
    else
        return n
    endif
enddef
# }}}2

# Init {{{1
var patterns: list<string>
# Tokens {{{2
# BAR_SEPARATION {{{3

const BAR_SEPARATION: string = '[^|\\]\@1<=|'

# OPENING_BRACKET {{{3

const OPENING_BRACKET: string = '[[{(]'

# CLOSING_BRACKET {{{3

const CLOSING_BRACKET: string = '[]})]'

# NON_BRACKET {{{3

const NON_BRACKET: string = '[^[\]{}()]'

# LIST_OR_DICT_CLOSING_BRACKET {{{3

const LIST_OR_DICT_CLOSING_BRACKET: string = '[]}]'

# LIST_OR_DICT_OPENING_BRACKET {{{3

const LIST_OR_DICT_OPENING_BRACKET: string = '[[{]'

# CHARACTER_UNDER_CURSOR {{{3

const CHARACTER_UNDER_CURSOR: string = '\%.c.'

# INLINE_COMMENT {{{3

# TODO: It is not required for an inline comment to be surrounded by whitespace.
# But it might help against false positives.
# To be more reliable, we should inspect the syntax, and only require whitespace
# before  the `#`  comment leader.   But that  might be  too costly  (because of
# `synstack()`).
const INLINE_COMMENT: string = '\s[#"]\%(\s\|[{}]\{3}\)'

# INLINE_VIM9_COMMENT {{{3

const INLINE_VIM9_COMMENT: string = '\s#'

# COMMENT {{{3

# TODO: Technically, `"\s` is wrong.
#
# First, whitespace is not required.
# Second, in Vim9, a string might appear at the start of the line.
# To be sure, we should also inspect the syntax.
# We can't use `INLINE_COMMENT` here. {{{
#
#     const COMMENT: string = $'^\s*{INLINE_COMMENT}'
#                                    ^------------^
#                                          ✘
#
# Because  `INLINE_COMMENT` asserts  the  presence of  a  whitespace before  the
# comment leader.  This assertion is not satisfied for a comment starting at the
# start of the line.
#}}}
const COMMENT: string = '^\s*\%(#\|"\\\=\s\).*$'

# DICT_KEY {{{3

const DICT_KEY: string = '^\s*\%('
    .. '\%(\w\|-\)\+'
    .. '\|'
    .. '"[^"]*"'
    .. '\|'
    .. "'[^']*'"
    .. '\|'
    .. '\[[^]]\+\]'
    .. '\)'
    .. ':\%(\s\|$\)'

# END_OF_COMMAND {{{3

const END_OF_COMMAND: string = $'\s*\%($\|||\@!\|{INLINE_COMMENT}\)'

# END_OF_LINE {{{3

const END_OF_LINE: string = $'\s*\%($\|{INLINE_COMMENT}\)'

# END_OF_VIM9_LINE {{{3

const END_OF_VIM9_LINE: string = $'\s*\%($\|{INLINE_VIM9_COMMENT}\)'

# OPERATOR {{{3

const OPERATOR: string = '\%(^\|\s\)\%([-+*/%]\|\.\.\|||\|&&\|??\|?\|<<\|>>\|\%([=!]=\|[<>]=\=\|[=!]\~\|is\|isnot\)[?#]\=\)\%(\s\|$\)\@=\%(\s*[|<]\)\@!'
    # assignment operators
    .. '\|' .. '\s\%([-+*/%]\|\.\.\)\==\%(\s\|$\)\@='
    # support `:` when used inside conditional operator `?:`
    .. '\|' .. '\%(\s\|^\):\%(\s\|$\)'

# HEREDOC_OPERATOR {{{3

const HEREDOC_OPERATOR: string = '\s=<<\s\@=\%(\s\+\%(trim\|eval\)\)\{,2}'

# PATTERN_DELIMITER {{{3

# A better regex would be:
#
#     [^-+*/%.:# \t[:alnum:]\"|]\@=.\|->\@!\%(=\s\)\@!\|[+*/%]\%(=\s\)\@!
#
# But sometimes, it can be too costly and cause `E363` to be given.
const PATTERN_DELIMITER: string = '[-+*/%]\%(=\s\)\@!'
# }}}2
# Syntaxes {{{2
# BLOCKS {{{3

const BLOCKS: list<list<string>> = [
    ['if', 'el\%[se]', 'elseif\=', 'en\%[dif]'],
    ['for', 'endfor\='],
    ['wh\%[ile]', 'endw\%[hile]'],
    ['try', 'cat\%[ch]', 'fina\|finally\=', 'endt\%[ry]'],
    ['def', 'enddef'],
    ['fu\%[nction](\@!', 'endf\%[unction]'],
    ['class', 'endclass'],
    ['interface', 'endinterface'],
    ['enum', 'endenum'],
    ['aug\%[roup]\%(\s\+[eE][nN][dD]\)\@!\s\+\S\+', 'aug\%[roup]\s\+[eE][nN][dD]'],
]

# MODIFIERS {{{3

# some keywords can be prefixed by modifiers (e.g. `def` can be prefixed by `export`)
const MODIFIERS: dict<string> = {
    def: ['export', 'static'],
    class: ['export', 'abstract', 'export abstract'],
    interface: ['export'],
}
#     ...
#     class: ['export', 'abstract', 'export abstract'],
#     ...
#     →
#     ...
#     class: '\%(export\|abstract\|export\s\+abstract\)\s\+',
#     ...
->map((_, mods: list<string>): string =>
    '\%(' .. mods
    ->join('\|')
    ->substitute('\s\+', '\\s\\+', 'g')
    .. '\)' .. '\s\+')

# HIGHER_ORDER_COMMAND {{{3

patterns =<< trim eval END
    argdo\>!\=
    bufdo\>!\=
    cdo\>!\=
    folddoc\%[losed]\>
    foldd\%[oopen]\>
    ldo\=\>!\=
    tabdo\=\>
    windo\>
    au\%[tocmd]\>!\=.*
    com\%[mand]\>!\=.*
    g\%[lobal]!\={PATTERN_DELIMITER}.*
    v\%[global]!\={PATTERN_DELIMITER}.*
END

const HIGHER_ORDER_COMMAND: string = $'\%(^\|{BAR_SEPARATION}\)\s*\<\%({patterns->join('\|')}\)\%(\s\|$\)\@='

# START_MIDDLE_END {{{3

# Let's derive this constant from `BLOCKS`:
#
#     [['if', 'el\%[se]', 'elseif\=', 'en\%[dif]'],
#      ['for', 'endfor\='],
#      ...,
#      [...]]
#     →
#     {
#      'for': ['for', '', 'endfor\='],
#      'endfor': ['for', '', 'endfor\='],
#      'if': ['if', 'el\%[se]\|elseif\=', 'en\%[dif]'],
#      'else': ['if', 'el\%[se]\|elseif\=', 'en\%[dif]'],
#      'elseif': ['if', 'el\%[se]\|elseif\=', 'en\%[dif]'],
#      'endif': ['if', 'el\%[se]\|elseif\=', 'en\%[dif]'],
#      ...
#     }
var START_MIDDLE_END: dict<list<string>>

def Unshorten(kwd: string): string
    return BlockStartKeyword(kwd)
enddef

def BlockStartKeyword(line: string): string
    var kwd: string = line->matchstr('\l\+')
    return fullcommand(kwd, false)
enddef

{
    for kwds: list<string> in BLOCKS
        var [start: string, middle: string, end: string] = [kwds[0], '', kwds[-1]]
        if MODIFIERS->has_key(start->Unshorten())
            start = $'\%({MODIFIERS[start]}\)\={start}'
        endif
        if kwds->len() > 2
            middle = kwds[1 : -2]->join('\|')
        endif
        for kwd: string in kwds
            START_MIDDLE_END->extend({[kwd->Unshorten()]: [start, middle, end]})
        endfor
    endfor
}

START_MIDDLE_END = START_MIDDLE_END
    ->map((_, kwds: list<string>) =>
        kwds->map((_, kwd: string) => kwd == ''
        ? ''
        : $'\%(^\|{BAR_SEPARATION}\|\<sil\%[ent]\|{HIGHER_ORDER_COMMAND}\)\s*'
        .. $'\<\%({kwd}\)\>\%(\s\|$\|!\)\@=\%(\s*{OPERATOR}\)\@!'))

lockvar! START_MIDDLE_END

# ENDS_BLOCK {{{3

const ENDS_BLOCK: string = '^\s*\%('
    .. BLOCKS
    ->copy()
    ->map((_, kwds: list<string>): string => kwds[-1])
    ->join('\|')
    .. '\|' .. CLOSING_BRACKET
    .. $'\){END_OF_COMMAND}'

# ENDS_BLOCK_OR_CLAUSE {{{3

patterns = BLOCKS
    ->copy()
    ->map((_, kwds: list<string>) => kwds[1 :])
    ->flattennew()
    # `catch` and `elseif` need to be handled as special cases
    ->filter((_, pat: string): bool => pat->Unshorten() !~ '^\%(catch\|elseif\)\>')

const ENDS_BLOCK_OR_CLAUSE: string = '^\s*\%(' .. patterns->join('\|') .. $'\){END_OF_COMMAND}'
    .. $'\|^\s*cat\%[ch]\%(\s\+\({PATTERN_DELIMITER}\).*\1\)\={END_OF_COMMAND}'
    .. $'\|^\s*elseif\=\>\%(\s\|$\)\@=\%(\s*{OPERATOR}\)\@!'

# STARTS_NAMED_BLOCK {{{3

patterns = []
{
    for kwds: list<string> in BLOCKS
        for kwd: string in kwds[0 : -2]
            if MODIFIERS->has_key(kwd->Unshorten())
                patterns += [$'\%({MODIFIERS[kwd]}\)\={kwd}']
            else
                patterns += [kwd]
            endif
        endfor
    endfor
}

const STARTS_NAMED_BLOCK: string = $'^\s*\%(sil\%[ent]\s\+\)\=\%({patterns->join('\|')}\)\>\%(\s\|$\|!\)\@='

# STARTS_CURLY_BLOCK {{{3

# TODO: `{` alone on a line is not necessarily the start of a block.
# It  could be  a dictionary  if the  previous line  ends with  a binary/ternary
# operator.  This  can cause  an issue whenever  we use  `STARTS_CURLY_BLOCK` or
# `LINE_CONTINUATION_AT_EOL`.
const STARTS_CURLY_BLOCK: string = '\%('
    .. '^\s*{'
    .. '\|' .. '^.*\zs\s=>\s\+{'
    .. '\|' ..  $'^\%(\s*\|.*{BAR_SEPARATION}\s*\)\%(com\%[mand]\|au\%[tocmd]\).*\zs\s{{'
    .. '\)' .. END_OF_COMMAND

# STARTS_FUNCTION {{{3

const STARTS_FUNCTION: string = $'^\s*\%({MODIFIERS.def}\)\=def\>!\=\s\@='

# ENDS_FUNCTION {{{3

const ENDS_FUNCTION: string = $'^\s*enddef\>{END_OF_COMMAND}'

# ASSIGNS_HEREDOC {{{3

const ASSIGNS_HEREDOC: string = $'^\%({COMMENT}\)\@!.*\%({HEREDOC_OPERATOR}\)\s\+\zs[A-Z]\+{END_OF_LINE}'

# PLUS_MINUS_COMMAND {{{3

# In legacy, the `:+` and `:-` commands are not required to be preceded by a colon.
# As a result, when `+` or `-` is alone on a line, there is ambiguity.
# It might be an operator or a command.
# To not break the indentation in legacy scripts, we might need to consider such
# lines as commands.
const PLUS_MINUS_COMMAND: string = '^\s*[+-]\s*$'

# TRICKY_COMMANDS {{{3

# Some  commands  are tricky  because  they  accept  an  argument which  can  be
# conflated with an operator.  Examples:
#
#     argdelete *
#     cd -
#     normal! ==
#     nunmap <buffer> (
#
# TODO: Other commands might accept operators as argument.  Handle them too.
patterns =<< trim eval END
    {'\'}<argd\%[elete]\s\+\*\s*$
    \<[lt]\=cd!\=\s\+-\s*$
    \<norm\%[al]!\=\s*\S\+$
    \%(\<sil\%[ent]!\=\s\+\)\=\<[nvxsoilct]\=\%(nore\|un\)map!\=\s
    {PLUS_MINUS_COMMAND}
END

const TRICKY_COMMANDS: string = patterns->join('\|')
# }}}2
# EOL {{{2
# OPENING_BRACKET_AT_EOL {{{3

const OPENING_BRACKET_AT_EOL: string = OPENING_BRACKET .. END_OF_VIM9_LINE

# CLOSING_BRACKET_AT_EOL {{{3

const CLOSING_BRACKET_AT_EOL: string = CLOSING_BRACKET .. END_OF_VIM9_LINE

# COMMA_AT_EOL {{{3

const COMMA_AT_EOL: string = $',{END_OF_VIM9_LINE}'

# COMMA_OR_DICT_KEY_AT_EOL {{{3

const COMMA_OR_DICT_KEY_AT_EOL: string = $'\%(,\|{DICT_KEY}\){END_OF_VIM9_LINE}'

# LAMBDA_ARROW_AT_EOL {{{3

const LAMBDA_ARROW_AT_EOL: string = $'\s=>{END_OF_VIM9_LINE}'

# LINE_CONTINUATION_AT_EOL {{{3

const LINE_CONTINUATION_AT_EOL: string = '\%('
    .. ','
    .. '\|' .. OPERATOR
    .. '\|' .. '\s=>'
    .. '\|' .. '[^=]\zs[[(]'
    .. '\|' .. DICT_KEY
    # `{` is ambiguous.
    # It can be the start of a dictionary or a block.
    # We only want to match the former.
    .. '\|' .. $'^\%({STARTS_CURLY_BLOCK}\)\@!.*\zs{{'
    .. '\)\s*\%(\s#.*\)\=$'
# }}}2
# SOL {{{2
# BACKSLASH_AT_SOL {{{3

const BACKSLASH_AT_SOL: string = '^\s*\%(\\\|[#"]\\ \)'

# CLOSING_BRACKET_AT_SOL {{{3

const CLOSING_BRACKET_AT_SOL: string = $'^\s*{CLOSING_BRACKET}'

# LINE_CONTINUATION_AT_SOL {{{3

const LINE_CONTINUATION_AT_SOL: string = '^\s*\%('
    .. '\\'
    .. '\|' .. '[#"]\\ '
    .. '\|' .. OPERATOR
    .. '\|' .. '->\s*\h'
    .. '\|' .. '\.\h'  # dict member
    .. '\|' .. '|'
    # TODO: `}` at the start of a line is not necessarily a line continuation.
    # Could be the end of a block.
    .. '\|' .. CLOSING_BRACKET
    .. '\)'

# RANGE_AT_SOL {{{3

const RANGE_AT_SOL: string = '^\s*:\S'
# }}}1
# Interface {{{1
export def Expr(lnum = v:lnum): number # {{{2
    # line which is indented
    var line_A: dict<any> = {text: getline(lnum), lnum: lnum}
    # line above, on which we'll base the indent of line A
    var line_B: dict<any>

    if line_A->AtStartOf('HereDoc')
        line_A->CacheHeredoc()
    elseif line_A.lnum->IsInside('HereDoc')
        return line_A.text->HereDocIndent()
    elseif line_A.lnum->IsRightBelow('HereDoc')
        var ind: number = b:vimindent.startindent
        unlet! b:vimindent
        return ind
    endif

    # Don't move this block after the function header one.
    # Otherwise, we  might clear the cache  too early if the  line following the
    # header is a comment.
    if line_A.text =~ COMMENT
        return CommentIndent()
    endif

    line_B = PrevCodeLine(line_A.lnum)
    if line_A.text =~ BACKSLASH_AT_SOL
        if line_B.text =~ BACKSLASH_AT_SOL
            return Indent(line_B.lnum)
        else
            return Indent(line_B.lnum) + IndentMoreLineContinuation()
        endif
    endif

    if line_A->AtStartOf('FuncHeader')
            && !IsInInterface()
        line_A.lnum->CacheFuncHeader()
    elseif line_A.lnum->IsInside('FuncHeader')
        return b:vimindent.startindent + 2 * shiftwidth()
    elseif line_A.lnum->IsRightBelow('FuncHeader')
        var startindent: number = b:vimindent.startindent
        unlet! b:vimindent
        if line_A.text =~ ENDS_FUNCTION
            return startindent
        else
            return startindent + shiftwidth()
        endif
    endif

    var past_bracket_block: dict<any>
    if exists('b:vimindent')
            && b:vimindent->has_key('is_BracketBlock')
        past_bracket_block = RemovePastBracketBlock(line_A)
    endif
    if line_A->AtStartOf('BracketBlock')
        line_A->CacheBracketBlock()
    endif
    if line_A.lnum->IsInside('BracketBlock')
        var is_in_curly_block: bool = IsInCurlyBlock()
        for block: dict<any> in b:vimindent.block_stack
            if line_A.lnum <= block.startlnum
                continue
            endif
            if !block->has_key('startindent')
                block.startindent = Indent(block.startlnum)
            endif
            if !is_in_curly_block
                return BracketBlockIndent(line_A, block)
            endif
        endfor
    endif
    if line_A.text->ContinuesBelowBracketBlock(line_B, past_bracket_block)
            && line_A.text !~ CLOSING_BRACKET_AT_SOL
        return past_bracket_block.startindent
            + (past_bracket_block.startline =~ STARTS_NAMED_BLOCK ? 2 * shiftwidth() : 0)
    endif

    # Problem: If we press `==` on the line right below the start of a multiline
    # lambda (split after its arrow `=>`), the indent is not correct.
    # Solution: Indent relative to the line above.
    if line_B->EndsWithLambdaArrow()
        return Indent(line_B.lnum) + shiftwidth() + IndentMoreInBracketBlock()
    endif
    # FIXME: Similar issue here:
    #
    #     var x = []
    #         ->filter((_, _) =>
    #             true)
    #         ->items()
    #
    # Press `==` on last line.
    # Expected: The `->items()` line is indented like `->filter(...)`.
    # Actual: It's indented like `true)`.
    # Is it worth fixing? `=ip` gives  the correct indentation, because then the
    # cache is used.

    # Don't move this block before the heredoc one.{{{
    #
    # A heredoc might be assigned on the very first line.
    # And if it is, we need to cache some info.
    #}}}
    # Don't move it before the function header and bracket block ones either.{{{
    #
    # You could, because these blocks of code deal with construct which can only
    # appear  in a  Vim9  script.  And  in  a  Vim9 script,  the  first line  is
    # `vim9script`.  Or  maybe some legacy code/comment  (see `:help vim9-mix`).
    # But you  can't find a  Vim9 function header or  Vim9 bracket block  on the
    # first line.
    #
    # Anyway, even if you could, don't.  First, it would be inconsistent.
    # Second, it  could give unexpected results  while we're trying to  fix some
    # failing test.
    #}}}
    if line_A.lnum == 1
        return 0
    endif

    # Don't do that:
    #     if line_A.text !~ '\S'
    #         return -1
    #     endif
    # It would prevent  a line from being automatically indented  when using the
    # normal command `o`.
    # TODO: Can we write a test for this?

    if line_B.text =~ STARTS_CURLY_BLOCK
        return Indent(line_B.lnum) + shiftwidth() + IndentMoreInBracketBlock()

    elseif line_A.text =~ CLOSING_BRACKET_AT_SOL
        var start: number = MatchingOpenBracket(line_A)
        if start <= 0
            return -1
        endif
        return Indent(start) + IndentMoreInBracketBlock()

    elseif line_A.text =~ ENDS_BLOCK_OR_CLAUSE
            && !line_B->EndsWithLineContinuation()
        var kwd: string = BlockStartKeyword(line_A.text)
        if !START_MIDDLE_END->has_key(kwd)
            return -1
        endif

        # If the cursor  is after the match  for the end pattern,  we won't find
        # the start of the block.  Let's make sure that doesn't happen.
        cursor(line_A.lnum, 1)

        var [start: string, middle: string, end: string] = START_MIDDLE_END[kwd]
        var block_start: number = SearchPairStart(start, middle, end)
        if block_start > 0
            return Indent(block_start)
        else
            return -1
        endif
    endif

    var base_ind: number
    if line_A->IsFirstLineOfCommand(line_B)
        line_A.isfirst = true
        line_B = line_B->FirstLinePreviousCommand()
        base_ind = Indent(line_B.lnum)

        if line_B->EndsWithCurlyBlock()
                && !line_A->IsInThisBlock(line_B.lnum)
            return base_ind
        endif

    else
        line_A.isfirst = false
        base_ind = Indent(line_B.lnum)

        var line_C: dict<any> = PrevCodeLine(line_B.lnum)
        if !line_B->IsFirstLineOfCommand(line_C) || line_C.lnum <= 0
            return base_ind
        endif
    endif

    var ind: number = base_ind + Offset(line_A, line_B)
    return [ind, 0]->max()
enddef

def g:GetVimIndent(): number # {{{2
    # for backward compatibility
    return Expr()
enddef
# }}}1
# Core {{{1
def Offset( # {{{2
        # we indent this line ...
        line_A: dict<any>,
        # ... relatively to this line
        line_B: dict<any>,
        ): number

    if line_B->AtStartOf('FuncHeader')
            && IsInInterface()
        return 0

    # increase indentation inside a block
    elseif line_B.text =~ STARTS_NAMED_BLOCK
            || line_B->EndsWithCurlyBlock()
        # But don't indent if the line starting the block also closes it.
        if line_B->AlsoClosesBlock()
            return 0
        # Indent twice for  a line continuation in the block  header itself, so that
        # we can easily  distinguish the end of  the block header from  the start of
        # the block body.
        elseif (line_B->EndsWithLineContinuation()
                && !line_A.isfirst)
                || (line_A.text =~ LINE_CONTINUATION_AT_SOL
                && line_A.text !~ PLUS_MINUS_COMMAND)
                || line_A.text->Is_IN_KeywordForLoop(line_B.text)
            return 2 * shiftwidth()
        else
            return shiftwidth()
        endif

    # increase indentation of  a line if it's the continuation  of a command which
    # started on a previous line
    elseif !line_A.isfirst
            && (line_B->EndsWithLineContinuation()
            || line_A.text =~ LINE_CONTINUATION_AT_SOL)
        return shiftwidth()
    endif

    return 0
enddef

def HereDocIndent(line_A: string): number # {{{2
    # at the end of a heredoc
    if line_A =~ $'^\s*{b:vimindent.endmarker}$'
        # `END` must be at the very start of the line if the heredoc is not trimmed
        if !b:vimindent.is_trimmed
            # We can't invalidate the cache just yet.
            # The indent of `END` is meaningless;  it's always 0.  The next line
            # will need to be indented relative to the start of the heredoc.  It
            # must know where it starts; it needs the cache.
            return 0
        else
            var ind: number = b:vimindent.startindent
            # invalidate the cache so that it's not used for the next heredoc
            unlet! b:vimindent
            return ind
        endif
    endif

    # In a non-trimmed heredoc, all of leading whitespace is semantic.
    # Leave it alone.
    if !b:vimindent.is_trimmed
        # But do save the indent of the assignment line.
        if !b:vimindent->has_key('startindent')
            b:vimindent.startindent = b:vimindent.startlnum->Indent()
        endif
        return -1
    endif

    # In a trimmed heredoc, *some* of the leading whitespace is semantic.
    # We want to preserve  it, so we can't just indent  relative to the assignment
    # line.  That's because we're dealing with data, not with code.
    # Instead, we need  to compute by how  much the indent of  the assignment line
    # was increased  or decreased.   Then, we  need to apply  that same  change to
    # every line inside the body.
    var offset: number
    if !b:vimindent->has_key('offset')
        var old_startindent: number = b:vimindent.startindent
        var new_startindent: number = b:vimindent.startlnum->Indent()
        offset = new_startindent - old_startindent

        # If all the non-empty lines in  the body have a higher indentation relative
        # to the assignment, there is no need to indent them more.
        # But if  at least one of  them does have  the same indentation level  (or a
        # lower one), then we want to indent it further (and the whole block with it).
        # This way,  we can clearly distinguish  the heredoc block from  the rest of
        # the code.
        var end: number = search($'^\s*{b:vimindent.endmarker}$', 'nW')
        var should_indent_more: bool = range(v:lnum, end - 1)
            ->indexof((_, lnum: number): bool => Indent(lnum) <= old_startindent && getline(lnum) != '') >= 0
        if should_indent_more
            offset += shiftwidth()
        endif

        b:vimindent.offset = offset
        b:vimindent.startindent = new_startindent
    endif

    return [0, Indent(v:lnum) + b:vimindent.offset]->max()
enddef

def CommentIndent(): number # {{{2
    var line_B: dict<any>
    line_B.lnum = prevnonblank(v:lnum - 1)
    line_B.text = getline(line_B.lnum)
    if line_B.text =~ COMMENT
        return Indent(line_B.lnum)
    endif

    var next: number = NextCodeLine()
    if next == 0
        return 0
    endif
    var vimindent_save: dict<any> = get(b:, 'vimindent', {})->deepcopy()
    var ind: number = next->Expr()
    # The previous `Expr()` might have set or deleted `b:vimindent`.
    # This could  cause issues (e.g.  when indenting  2 commented lines  above a
    # heredoc).  Let's make sure the state of the variable is not altered.
    if vimindent_save->empty()
        unlet! b:vimindent
    else
        b:vimindent = vimindent_save
    endif
    if getline(next) =~ ENDS_BLOCK
        return ind + shiftwidth()
    else
        return ind
    endif
enddef

def BracketBlockIndent(line_A: dict<any>, block: dict<any>): number # {{{2
    var ind: number = block.startindent

    if line_A.text =~ CLOSING_BRACKET_AT_SOL
        if b:vimindent.is_on_named_block_line
            ind += 2 * shiftwidth()
        endif
        return ind + IndentMoreInBracketBlock()
    endif

    var startline: dict<any> = {
        text: block.startline,
        lnum: block.startlnum
    }
    if startline->EndsWithComma()
            || startline->EndsWithLambdaArrow()
            || (startline->EndsWithOpeningBracket()
            # TODO: Is that reliable?
            && block.startline !~
            $'^\s*{NON_BRACKET}\+{LIST_OR_DICT_CLOSING_BRACKET},\s\+{LIST_OR_DICT_OPENING_BRACKET}')
        ind += shiftwidth() + IndentMoreInBracketBlock()
    endif

    if b:vimindent.is_on_named_block_line
        ind += shiftwidth()
    endif

    if block.is_dict
            && line_A.text !~ DICT_KEY
        ind += shiftwidth()
    endif

    return ind
enddef

def CacheHeredoc(line_A: dict<any>) # {{{2
    var endmarker: string = line_A.text->matchstr(ASSIGNS_HEREDOC)
    var endlnum: number = search($'^\s*{endmarker}$', 'nW')
    var is_trimmed: bool = line_A.text =~ $'.*\s\%(trim\%(\s\+eval\)\=\)\s\+[A-Z]\+{END_OF_LINE}'
    b:vimindent = {
        is_HereDoc: true,
        startlnum: line_A.lnum,
        endlnum: endlnum,
        endmarker: endmarker,
        is_trimmed: is_trimmed,
    }
    if is_trimmed
        b:vimindent.startindent = Indent(line_A.lnum)
    endif
    RegisterCacheInvalidation()
enddef

def CacheFuncHeader(startlnum: number) # {{{2
    var pos: list<number> = getcurpos()
    cursor(startlnum, 1)
    if search('(', 'W', startlnum) <= 0
        return
    endif
    var endlnum: number = SearchPair('(', '', ')', 'nW')
    setpos('.', pos)
    if endlnum == startlnum
        return
    endif

    b:vimindent = {
        is_FuncHeader: true,
        startindent: startlnum->Indent(),
        endlnum: endlnum,
    }
    RegisterCacheInvalidation()
enddef

def CacheBracketBlock(line_A: dict<any>) # {{{2
    var pos: list<number> = getcurpos()
    var opening: string = line_A.text->matchstr(CHARACTER_UNDER_CURSOR)
    var closing: string = {'[': ']', '{': '}', '(': ')'}[opening]
    var endlnum: number = SearchPair(opening, '', closing, 'nW')
    setpos('.', pos)
    if endlnum <= line_A.lnum
        return
    endif

    if !exists('b:vimindent')
        b:vimindent = {
            is_BracketBlock: true,
            is_on_named_block_line: line_A.text =~ STARTS_NAMED_BLOCK,
            block_stack: [],
        }
    endif

    var is_dict: bool
    var is_curly_block: bool
    if opening == '{'
        if line_A.text =~ STARTS_CURLY_BLOCK
            [is_dict, is_curly_block] = [false, true]
        else
            [is_dict, is_curly_block] = [true, false]
        endif
    endif
    b:vimindent.block_stack->insert({
        is_dict: is_dict,
        is_curly_block: is_curly_block,
        startline: line_A.text,
        startlnum: line_A.lnum,
        endlnum: endlnum,
    })

    RegisterCacheInvalidation()
enddef

def RegisterCacheInvalidation() # {{{2
    # invalidate the cache so that it's not used for the next `=` normal command
    autocmd_add([{
        cmd: 'unlet! b:vimindent',
        event: 'ModeChanged',
        group: '__VimIndent__',
        once: true,
        pattern: '*:n',
        replace: true,
    }])
enddef

def RemovePastBracketBlock(line_A: dict<any>): dict<any> # {{{2
    var stack: list<dict<any>> = b:vimindent.block_stack

    var removed: dict<any>
    if line_A.lnum > stack[0].endlnum
        removed = stack[0]
    endif

    stack->filter((_, block: dict<any>): bool => line_A.lnum <= block.endlnum)
    if stack->empty()
        unlet! b:vimindent
    endif
    return removed
enddef
# }}}1
# Util {{{1
# Get {{{2
def Indent(lnum: number): number # {{{3
    if lnum <= 0
        # Don't  return `-1`.  It could cause `Expr()` to return a non-multiple of `'shiftwidth'`.{{{
        #
        # It would be  OK if we were always returning  `Indent()` directly.  But
        # we  don't.  Most  of  the  time, we  include  it  in some  computation
        # like  `Indent(...) + shiftwidth()`.   If  `'shiftwidth'` is  `4`,  and
        # `Indent()` returns `-1`, `Expr()` will end up returning `3`.
        #}}}
        return 0
    endif
    return indent(lnum)
enddef

def MatchingOpenBracket(line: dict<any>): number # {{{3
    var end: string = line.text->matchstr(CLOSING_BRACKET)
    var start: string = {']': '[', '}': '{', ')': '('}[end]
    cursor(line.lnum, 1)
    return SearchPairStart(start, '', end)
enddef

def FirstLinePreviousCommand(line: dict<any>): dict<any> # {{{3
    var line_B: dict<any> = line

    while line_B.lnum > 1
        var code_line_above: dict<any> = PrevCodeLine(line_B.lnum)

        if line_B.text =~ CLOSING_BRACKET_AT_SOL
            var n: number = MatchingOpenBracket(line_B)

            if n <= 0
                break
            endif

            line_B.lnum = n
            line_B.text = getline(line_B.lnum)
            continue

        elseif line_B->IsFirstLineOfCommand(code_line_above)
            break
        endif

        line_B = code_line_above
    endwhile

    return line_B
enddef

def PrevCodeLine(lnum: number): dict<any> # {{{3
    var line: string = getline(lnum)
    if line =~ '^\s*[A-Z]\+$'
        var endmarker: string = line->matchstr('[A-Z]\+')
        var pos: list<number> = getcurpos()
        cursor(lnum, 1)
        var n: number = search(ASSIGNS_HEREDOC, 'bnW')
        setpos('.', pos)
        if n > 0
            line = getline(n)
            if line =~ $'{HEREDOC_OPERATOR}\s\+{endmarker}'
                return {lnum: n, text: line}
            endif
        endif
    endif

    var n: number = prevnonblank(lnum - 1)
    line = getline(n)
    while line =~ COMMENT && n > 1
        n = prevnonblank(n - 1)
        line = getline(n)
    endwhile
    # If we get back to the first line, we return 1 no matter what; even if it's a
    # commented line.   That should not  cause an issue  though.  We just  want to
    # avoid a  commented line above which  there is a  line of code which  is more
    # relevant.  There is nothing above the first line.
    return {lnum: n, text: line}
enddef

def NextCodeLine(): number # {{{3
    var last: number = line('$')
    if v:lnum == last
        return 0
    endif

    var lnum: number = v:lnum + 1
    while lnum <= last
        var line: string = getline(lnum)
        if line != '' && line !~ COMMENT
            return lnum
        endif
        ++lnum
    endwhile
    return 0
enddef

def SearchPair( # {{{3
        start: string,
        middle: string,
        end: string,
        flags: string,
        stopline = 0,
        ): number

    var s: string = start
    var e: string = end
    if start == '[' || start == ']'
        s = s->escape('[]')
    endif
    if end == '[' || end == ']'
        e = e->escape('[]')
    endif
    return searchpair('\C' .. s, (middle == '' ? '' : '\C' .. middle), '\C' .. e,
        flags, (): bool => InCommentOrString(), stopline, TIMEOUT)
enddef

def SearchPairStart( # {{{3
        start: string,
        middle: string,
        end: string,
        ): number
    return SearchPair(start, middle, end, 'bnW')
enddef

def SearchPairEnd( # {{{3
        start: string,
        middle: string,
        end: string,
        stopline = 0,
        ): number
    return SearchPair(start, middle, end, 'nW', stopline)
enddef
# }}}2
# Test {{{2
def AtStartOf(line_A: dict<any>, syntax: string): bool # {{{3
    if syntax == 'BracketBlock'
        return AtStartOfBracketBlock(line_A)
    endif

    var pat: string = {
        HereDoc: ASSIGNS_HEREDOC,
        FuncHeader: STARTS_FUNCTION
    }[syntax]
    return line_A.text =~ pat
        && (!exists('b:vimindent') || !b:vimindent->has_key('is_HereDoc'))
enddef

def AtStartOfBracketBlock(line_A: dict<any>): bool # {{{3
    # We  ignore bracket  blocks  while we're  indenting  a function  header
    # because  it makes  the logic  simpler.  It  might mean  that we  don't
    # indent correctly a  multiline bracket block inside  a function header,
    # but that's  a corner case for  which it doesn't seem  worth making the
    # code more complex.
    if exists('b:vimindent')
            && !b:vimindent->has_key('is_BracketBlock')
        return false
    endif

    var pos: list<number> = getcurpos()
    cursor(line_A.lnum, [line_A.lnum, '$']->col())

    if SearchPair(OPENING_BRACKET, '', CLOSING_BRACKET, 'bcW', line_A.lnum) <= 0
        setpos('.', pos)
        return false
    endif
    # Don't restore the cursor position.
    # It needs to be on a bracket for `CacheBracketBlock()` to work as intended.

    return line_A->EndsWithOpeningBracket()
        || line_A->EndsWithCommaOrDictKey()
        || line_A->EndsWithLambdaArrow()
enddef

def ContinuesBelowBracketBlock( # {{{3
        line_A: string,
        line_B: dict<any>,
        block: dict<any>
        ): bool

    return !block->empty()
        && (line_A =~ LINE_CONTINUATION_AT_SOL
        || line_B->EndsWithLineContinuation())
enddef

def IsInside(lnum: number, syntax: string): bool # {{{3
    if !exists('b:vimindent')
            || !b:vimindent->has_key($'is_{syntax}')
        return false
    endif

    if syntax == 'BracketBlock'
        if !b:vimindent->has_key('block_stack')
                || b:vimindent.block_stack->empty()
            return false
        endif
        return lnum <= b:vimindent.block_stack[0].endlnum
    endif

    return lnum <= b:vimindent.endlnum
enddef

def IsRightBelow(lnum: number, syntax: string): bool # {{{3
    return exists('b:vimindent')
        && b:vimindent->has_key($'is_{syntax}')
        && lnum > b:vimindent.endlnum
enddef

def IsInCurlyBlock(): bool # {{{3
    return b:vimindent.block_stack
        ->indexof((_, block: dict<any>): bool => block.is_curly_block) >= 0
enddef

def IsInThisBlock(line_A: dict<any>, lnum: number): bool # {{{3
    var pos: list<number> = getcurpos()
    cursor(lnum, [lnum, '$']->col())
    var end: number = SearchPairEnd('{', '', '}')
    setpos('.', pos)

    return line_A.lnum <= end
enddef

def IsInInterface(): bool # {{{3
    return SearchPair('interface', '', 'endinterface', 'nW') > 0
enddef

def IsFirstLineOfCommand(line_1: dict<any>, line_2: dict<any>): bool # {{{3
    if line_1.text->Is_IN_KeywordForLoop(line_2.text)
        return false
    endif

    if line_1.text =~ RANGE_AT_SOL
            || line_1.text =~ PLUS_MINUS_COMMAND
        return true
    endif

    if line_2.text =~ DICT_KEY
            && !line_1->IsInThisBlock(line_2.lnum)
        return true
    endif

    var line_1_is_good: bool = line_1.text !~ COMMENT
        && line_1.text !~ DICT_KEY
        && line_1.text !~ LINE_CONTINUATION_AT_SOL

    var line_2_is_good: bool = !line_2->EndsWithLineContinuation()

    return line_1_is_good && line_2_is_good
enddef

def Is_IN_KeywordForLoop(line_1: string, line_2: string): bool # {{{3
    return line_2 =~ '^\s*for\s'
        && line_1 =~ '^\s*in\s'
enddef

def InCommentOrString(): bool # {{{3
    return synstack('.', col('.'))
        ->indexof((_, id: number): bool => synIDattr(id, 'name') =~ '\ccomment\|string\|heredoc') >= 0
enddef

def AlsoClosesBlock(line_B: dict<any>): bool # {{{3
    # We know that `line_B` opens a block.
    # Let's see if it also closes that block.
    var kwd: string = BlockStartKeyword(line_B.text)
    if !START_MIDDLE_END->has_key(kwd)
        return false
    endif

    var [start: string, middle: string, end: string] = START_MIDDLE_END[kwd]
    var pos: list<number> = getcurpos()
    cursor(line_B.lnum, 1)
    var block_end: number = SearchPairEnd(start, middle, end, line_B.lnum)
    setpos('.', pos)

    return block_end > 0
enddef

def EndsWithComma(line: dict<any>): bool # {{{3
    return NonCommentedMatch(line, COMMA_AT_EOL)
enddef

def EndsWithCommaOrDictKey(line_A: dict<any>): bool # {{{3
    return NonCommentedMatch(line_A, COMMA_OR_DICT_KEY_AT_EOL)
enddef

def EndsWithCurlyBlock(line_B: dict<any>): bool # {{{3
    return NonCommentedMatch(line_B, STARTS_CURLY_BLOCK)
enddef

def EndsWithLambdaArrow(line_A: dict<any>): bool # {{{3
    return NonCommentedMatch(line_A, LAMBDA_ARROW_AT_EOL)
enddef

def EndsWithLineContinuation(line_B: dict<any>): bool # {{{3
    return NonCommentedMatch(line_B, LINE_CONTINUATION_AT_EOL)
enddef

def EndsWithOpeningBracket(line: dict<any>): bool # {{{3
    return NonCommentedMatch(line, OPENING_BRACKET_AT_EOL)
enddef

def EndsWithClosingBracket(line: dict<any>): bool # {{{3
    return NonCommentedMatch(line, CLOSING_BRACKET_AT_EOL)
enddef

def NonCommentedMatch(line: dict<any>, pat: string): bool # {{{3
    # Could happen if there is no code above us, and we're not on the 1st line.
    # In that case, `PrevCodeLine()` returns `{lnum: 0, line: ''}`.
    if line.lnum == 0
        return false
    endif

    # Technically, that's wrong.  A  line might start with a range  and end with a
    # line continuation symbol.  But it's unlikely.  And it's useful to assume the
    # opposite because it  prevents us from conflating a mark  with an operator or
    # the start of a list:
    #
    #              not a comparison operator
    #              v
    #     :'< mark <
    #     :'< mark [
    #              ^
    #              not the start of a list
    if line.text =~ RANGE_AT_SOL
        return false
    endif

    #                    that's not an arithmetic operator
    #                    v
    #     catch /pattern /
    #
    # When `/` is used as a pattern delimiter, it's always present twice.
    # And  usually, the  first occurrence  is  in the  middle of  a sequence  of
    # non-whitespace characters.  If we can find  such a `/`, we assume that the
    # trailing `/` is not an operator.
    # Warning: Here, don't use a too complex pattern.{{{
    #
    # In particular, avoid backreferences.
    # For example, this would be too costly:
    #
    #     if line.text =~ $'\%(\S*\({PATTERN_DELIMITER}\)\S\+\|\S\+\({PATTERN_DELIMITER}\)\S*\)'
    #             .. $'\s\+\1{END_OF_COMMAND}'
    #
    # Sometimes, it could even give `E363`.
    #}}}
    var delim: string = line.text
        ->matchstr($'\s\+\zs{PATTERN_DELIMITER}\ze{END_OF_COMMAND}')
    if !delim->empty()
        delim = $'\V{delim}\m'
        if line.text =~ $'\%(\S*{delim}\S\+\|\S\+{delim}\S*\)\s\+{delim}{END_OF_COMMAND}'
            return false
        endif
    endif
    # TODO: We might still miss some corner cases:{{{
    #
    #                          conflated with arithmetic division
    #                          v
    #     substitute/pat / rep /
    #         echo
    #     ^--^
    #      ✘
    #
    # A better way to handle all these corner cases, would be to inspect the top
    # of the syntax stack:
    #
    #     :echo synID('.', col('.'), v:false)->synIDattr('name')
    #
    # Unfortunately, the legacy syntax plugin is not accurate enough.
    # For example, it doesn't highlight a slash as an operator.
    # }}}

    # `%` at the end of a line is tricky.
    # It might be the modulo operator or the current file (e.g. `edit %`).
    # Let's assume it's the latter.
    if line.text =~ $'%{END_OF_COMMAND}'
        return false
    endif

    if line.text =~ TRICKY_COMMANDS
        return false
    endif

    var pos: list<number> = getcurpos()
    cursor(line.lnum, 1)
    var match_lnum: number = search(pat, 'cnW', line.lnum, TIMEOUT, (): bool => InCommentOrString())
    setpos('.', pos)
    return match_lnum > 0
enddef
# }}}1
# vim:sw=4
