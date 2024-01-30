" autoload/editorconfig_core/fnmatch.vim: Globbing for
" editorconfig-vim.  Ported from the Python core's fnmatch.py.

" Copyright (c) 2012-2019 EditorConfig Team {{{1
" All rights reserved.
"
" Redistribution and use in source and binary forms, with or without
" modification, are permitted provided that the following conditions are met:
"
" 1. Redistributions of source code must retain the above copyright notice,
"    this list of conditions and the following disclaimer.
" 2. Redistributions in binary form must reproduce the above copyright notice,
"    this list of conditions and the following disclaimer in the documentation
"    and/or other materials provided with the distribution.
"
" THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
" IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
" ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
" LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
" CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
" SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
" INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
" CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
" ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
" POSSIBILITY OF SUCH DAMAGE. }}}1

"Filename matching with shell patterns.
"
"fnmatch(FILENAME, PATH, PATTERN) matches according to the local convention.
"fnmatchcase(FILENAME, PATH, PATTERN) always takes case in account.
"
"The functions operate by translating the pattern into a regular
"expression.  They cache the compiled regular expressions for speed.
"
"The function translate(PATTERN) returns a regular expression
"corresponding to PATTERN.  (It does not compile it.)

let s:saved_cpo = &cpo
set cpo&vim

" variables {{{1
if !exists('g:editorconfig_core_vimscript_debug')
    let g:editorconfig_core_vimscript_debug = 0
endif
" }}}1
" === Regexes =========================================================== {{{1
let s:LEFT_BRACE = '\v[\\]@8<!\{'
" 8 is an arbitrary byte-count limit to the lookbehind (micro-optimization)
"LEFT_BRACE = re.compile(
"    r"""
"
"    (?<! \\ ) # Not preceded by "\"
"
"    \{                  # "{"
"
"    """, re.VERBOSE
")

let s:RIGHT_BRACE = '\v[\\]@8<!\}'
" 8 is an arbitrary byte-count limit to the lookbehind (micro-optimization)
"RIGHT_BRACE = re.compile(
"    r"""
"
"    (?<! \\ ) # Not preceded by "\"
"
"    \}                  # "}"
"
"    """, re.VERBOSE
")

let s:NUMERIC_RANGE = '\v([+-]?\d+)' . '\.\.' . '([+-]?\d+)'
"NUMERIC_RANGE = re.compile(
"    r"""
"    (               # Capture a number
"        [+-] ?      # Zero or one "+" or "-" characters
"        \d +        # One or more digits
"    )
"
"    \.\.            # ".."
"
"    (               # Capture a number
"        [+-] ?      # Zero or one "+" or "-" characters
"        \d +        # One or more digits
"    )
"    """, re.VERBOSE
")

" }}}1
" === Internal functions ================================================ {{{1

" Dump the bytes of a:text.  For debugging use.
function! s:dump_bytes(text)
    let l:idx=0
    while l:idx < strlen(a:text)
        let l:byte_val = char2nr(a:text[l:idx])
        echom printf('%10s%-5d%02x %s', '', l:idx, l:byte_val,
            \ a:text[l:idx])
        let l:idx+=1
    endwhile
endfunction "s:dump_bytes

" Dump the characters of a:text and their codepoints.  For debugging use.
function! s:dump_chars(text)
    let l:chars = split(a:text, '\zs')
    let l:idx = 0
    let l:out1 = ''
    let l:out2 = ''
    while l:idx < len(l:chars)
        let l:char = l:chars[l:idx]
        let l:out1 .= printf('%5s', l:char)
        let l:out2 .= printf('%5x', char2nr(l:char))
        let l:idx+=1
    endwhile

    echom l:out1
    echom l:out2
endfunction "s:dump_chars

" }}}1
" === Translating globs to patterns ===================================== {{{1

" Used by s:re_escape: backslash-escape any character below U+0080;
" replace all others with a %U escape.
" See https://vi.stackexchange.com/a/19617/1430 by yours truly
" (https://vi.stackexchange.com/users/1430/cxw).
unlockvar s:replacement_expr
let s:replacement_expr =
    \ '\=' .
    \ '((char2nr(submatch(1)) >= 128) ? ' .
    \       'printf("%%U%08x", char2nr(submatch(1))) : ' .
    \       '("\\" . submatch(1))' .
    \ ')'
lockvar s:replacement_expr

" Escaper for very-magic regexes
function! s:re_escape(text)
    return substitute(a:text, '\v([^0-9a-zA-Z_])', s:replacement_expr, 'g')
endfunction

"def translate(pat, nested=0):
"    Translate a shell PATTERN to a regular expression.
"    There is no way to quote meta-characters.
function! editorconfig_core#fnmatch#translate(pat, ...)
    let l:nested = 0
    if a:0
        let l:nested = a:1
    endif

    if g:editorconfig_core_vimscript_debug
        echom '- fnmatch#translate: pattern ' . a:pat
        echom printf(
            \ '- %d chars', strlen(substitute(a:pat, ".", "x", "g")))
        call s:dump_chars(a:pat)
    endif

    let l:pat = a:pat   " TODO remove if we wind up not needing this

    " Note: the Python sets MULTILINE and DOTALL, but Vim has \_.
    " instead of DOTALL, and \_^ / \_$ instead of MULTILINE.

    let l:is_escaped = 0

    " Find out whether the pattern has balanced braces.
    let l:left_braces=[]
    let l:right_braces=[]
    call substitute(l:pat, s:LEFT_BRACE, '\=add(l:left_braces, 1)', 'g')
    call substitute(l:pat, s:RIGHT_BRACE, '\=add(l:right_braces, 1)', 'g')
    " Thanks to http://jeromebelleman.gitlab.io/posts/productivity/vimsub/
    let l:matching_braces = (len(l:left_braces) == len(l:right_braces))

    " Unicode support (#2).  Indexing l:pat[l:index] returns bytes, per
    " https://github.com/neovim/neovim/issues/68#issue-28114985 .
    " Instead, use split() per vimdoc to break the input string into an
    " array of *characters*, and process that.
    let l:characters = split(l:pat, '\zs')

    let l:index = 0     " character index
    let l:length = len(l:characters)
    let l:brace_level = 0
    let l:in_brackets = 0

    let l:result = ''
    let l:numeric_groups = []
    while l:index < l:length
        let l:current_char = l:characters[l:index]
        let l:index += 1

"         if g:editorconfig_core_vimscript_debug
"             echom ' - fnmatch#translate: ' . l:current_char . '@' .
"                 \ (l:index-1) . '; result ' . l:result
"         endif

        if l:current_char ==# '*'
            let l:pos = l:index
            if l:pos < l:length && l:characters[l:pos] ==# '*'
                let l:result .= '\_.*'
                let l:index += 1    " skip the second star
            else
                let l:result .= '[^/]*'
            endif

        elseif l:current_char ==# '?'
            let l:result .= '\_[^/]'

        elseif l:current_char ==# '['
            if l:in_brackets
                let l:result .= '\['
            else
                let l:pos = l:index
                let l:has_slash = 0
                while l:pos < l:length && l:characters[l:pos] != ']'
                    if l:characters[l:pos] ==# '/' && l:characters[l:pos-1] !=# '\'
                        let has_slash = 1
                        break
                    endif
                    let l:pos += 1
                endwhile
                if l:has_slash
                    " POSIX IEEE 1003.1-2017 sec. 2.13.3: '/' cannot occur
                    " in a bracket expression, so [/] matches a literal
                    " three-character string '[' . '/' . ']'.
                    let l:result .= '\['
                        \ . s:re_escape(join(l:characters[l:index : l:pos-1], ''))
                        \ . '\/'
                        " escape the slash
                    let l:index = l:pos + 1
                        " resume after the slash
                else
                    if l:index < l:length && l:characters[l:index] =~# '\v%(\^|\!)'
                        let l:index += 1
                        let l:result .= '[^'
                    else
                        let l:result .= '['
                    endif
                    let l:in_brackets = 1
                endif
            endif

        elseif l:current_char ==# '-'
            if l:in_brackets
                let l:result .= l:current_char
            else
                let l:result .= '\' . l:current_char
            endif

        elseif l:current_char ==# ']'
            if l:in_brackets && !l:is_escaped
                let l:result .= ']'
                let l:in_brackets = 0
            elseif l:is_escaped
                let l:result .= '\]'
                let l:is_escaped = 0
            else
                let l:result .= '\]'
            endif

        elseif l:current_char ==# '{'
            let l:pos = l:index
            let l:has_comma = 0
            while l:pos < l:length && (l:characters[l:pos] !=# '}' || l:is_escaped)
                if l:characters[l:pos] ==# ',' && ! l:is_escaped
                    let l:has_comma = 1
                    break
                endif
                let l:is_escaped = l:characters[l:pos] ==# '\' && ! l:is_escaped
                let l:pos += 1
            endwhile
            if ! l:has_comma && l:pos < l:length
                let l:num_range =
                    \ matchlist(join(l:characters[l:index : l:pos-1], ''),
                    \           s:NUMERIC_RANGE)
                if len(l:num_range) > 0     " Remember the ranges
                    call add(l:numeric_groups, [ 0+l:num_range[1], 0+l:num_range[2] ])
                    let l:result .= '([+-]?\d+)'
                else
                    let l:inner_xlat = editorconfig_core#fnmatch#translate(
                        \ join(l:characters[l:index : l:pos-1], ''), 1)
                    let l:inner_result = l:inner_xlat[0]
                    let l:inner_groups = l:inner_xlat[1]
                    let l:result .= '\{' . l:inner_result . '\}'
                    let l:numeric_groups += l:inner_groups
                endif
                let l:index = l:pos + 1
            elseif l:matching_braces
                let l:result .= '%('
                let l:brace_level += 1
            else
                let l:result .= '\{'
            endif

        elseif l:current_char ==# ','
            if l:brace_level > 0 && ! l:is_escaped
                let l:result .= '|'
            else
                let l:result .= '\,'
            endif

        elseif l:current_char ==# '}'
            if l:brace_level > 0 && ! l:is_escaped
                let l:result .= ')'
                let l:brace_level -= 1
            else
                let l:result .= '\}'
            endif

        elseif l:current_char ==# '/'
            if join(l:characters[l:index : (l:index + 2)], '') ==# '**/'
                let l:result .= '%(/|/\_.*/)'
                let l:index += 3
            else
                let l:result .= '\/'
            endif

        elseif l:current_char != '\'
            let l:result .= s:re_escape(l:current_char)
        endif

        if l:current_char ==# '\'
            if l:is_escaped
                let l:result .= s:re_escape(l:current_char)
            endif
            let l:is_escaped = ! l:is_escaped
        else
            let l:is_escaped = 0
        endif

    endwhile

    if ! l:nested
        let l:result .= '\_$'
    endif

    return [l:result, l:numeric_groups]
endfunction " #editorconfig_core#fnmatch#translate

let s:_cache = {}
function! s:cached_translate(pat)
    if ! has_key(s:_cache, a:pat)
        "regex = re.compile(res)
        let s:_cache[a:pat] =
            \ editorconfig_core#fnmatch#translate(a:pat)
            " we don't compile the regex
    endif
    return s:_cache[a:pat]
endfunction " cached_translate

" }}}1
" === Matching functions ================================================ {{{1

function! editorconfig_core#fnmatch#fnmatch(name, path, pattern)
"def fnmatch(name, pat):
"    """Test whether FILENAME matches PATH/PATTERN.
"
"    Patterns are Unix shell style:
"
"    - ``*``             matches everything except path separator
"    - ``**``            matches everything
"    - ``?``             matches any single character
"    - ``[seq]``         matches any character in seq
"    - ``[!seq]``        matches any char not in seq
"    - ``{s1,s2,s3}``    matches any of the strings given (separated by commas)
"
"    An initial period in FILENAME is not special.
"    Both FILENAME and PATTERN are first case-normalized
"    if the operating system requires it.
"    If you don't want this, use fnmatchcase(FILENAME, PATTERN).
"    """
"
    " Note: This throws away the backslash in '\.txt' on Cygwin, but that
    " makes sense since it's Windows under the hood.
    " We don't care about shellslash since we're going to change backslashes
    " to slashes in just a moment anyway.
    let l:localname = fnamemodify(a:name, ':p')

    if editorconfig_core#util#is_win()      " normalize
        let l:localname = substitute(tolower(l:localname), '\v\\', '/', 'g')
        let l:path = substitute(tolower(a:path), '\v\\', '/', 'g')
        let l:pattern = tolower(a:pattern)
    else
        let l:localname = l:localname
        let l:path = a:path
        let l:pattern = a:pattern
    endif

    if g:editorconfig_core_vimscript_debug
        echom '- fnmatch#fnmatch testing <' . l:localname . '> against <' .
            \ l:pattern . '> wrt <' . l:path . '>'
    endif

    return editorconfig_core#fnmatch#fnmatchcase(l:localname, l:path, l:pattern)
endfunction " fnmatch

function! editorconfig_core#fnmatch#fnmatchcase(name, path, pattern)
"def fnmatchcase(name, pat):
"    """Test whether FILENAME matches PATH/PATTERN, including case.
"
"    This is a version of fnmatch() which doesn't case-normalize
"    its arguments.
"    """
"
    let [regex, num_groups] = s:cached_translate(a:pattern)

    let l:escaped_path = s:re_escape(a:path)
    let l:regex = '\v' . l:escaped_path . l:regex

    if g:editorconfig_core_vimscript_debug
        echom '- fnmatch#fnmatchcase: regex    ' . l:regex
        call s:dump_chars(l:regex)
        echom '- fnmatch#fnmatchcase: checking ' . a:name
        call s:dump_chars(a:name)
    endif

    let l:match_groups = matchlist(a:name, l:regex)[1:]   " [0] = full match

    if g:editorconfig_core_vimscript_debug
        echom printf('  Got %d matches', len(l:match_groups))
    endif

    if len(l:match_groups) == 0
        return 0
    endif

    " Check numeric ranges
    let pattern_matched = 1
    for l:idx in range(0,len(l:match_groups))
        let l:num = l:match_groups[l:idx]
        if l:num ==# ''
            break
        endif

        let [min_num, max_num] = num_groups[l:idx]
        if (min_num > (0+l:num)) || ((0+l:num) > max_num)
            let pattern_matched = 0
            break
        endif

        " Reject leading zeros without sign.  This is very odd ---
        " see editorconfig/editorconfig#371.
        if match(l:num, '\v^0') != -1
            let pattern_matched = 0
            break
        endif
    endfor

    if g:editorconfig_core_vimscript_debug
        echom '- fnmatch#fnmatchcase: ' . (pattern_matched ? 'matched' : 'did not match')
    endif

    return pattern_matched
endfunction " fnmatchcase

" }}}1
" === Copyright notices ================================================= {{{1
" Based on code from fnmatch.py file distributed with Python 2.6.
" Portions Copyright (c) 2001-2010 Python Software Foundation;
" All Rights Reserved.  Licensed under PSF License (see LICENSE.PSF file).
"
" Changes to original fnmatch:
"
" - translate function supports ``*`` and ``**`` similarly to fnmatch C library
" }}}1

let &cpo = s:saved_cpo
unlet! s:saved_cpo

" vi: set fdm=marker:
