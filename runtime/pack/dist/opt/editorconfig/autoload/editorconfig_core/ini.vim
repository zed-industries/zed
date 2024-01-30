" autoload/editorconfig_core/ini.vim: Config-file parser for
" editorconfig-core-vimscript and editorconfig-vim.
" Modified from the Python core's ini.py.

" Copyright (c) 2012-2019 EditorConfig Team {{{2
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
" POSSIBILITY OF SUCH DAMAGE. }}}2

let s:saved_cpo = &cpo
set cpo&vim

" variables {{{2
if !exists('g:editorconfig_core_vimscript_debug')
    let g:editorconfig_core_vimscript_debug = 0
endif
" }}}2
" === Constants, including regexes ====================================== {{{2
" Regular expressions for parsing section headers and options.
" Allow ``]`` and escaped ``;`` and ``#`` characters in section headers.
" In fact, allow \ to escape any single character - it needs to cover at
" least \ * ? [ ! ] { }.
unlockvar s:SECTCRE s:OPTCRE s:MAX_SECTION_NAME s:MAX_PROPERTY_NAME s:MAX_PROPERTY_VALUE
let s:SECTCRE = '\v^\s*\[(%([^\\#;]|\\.)+)\]'

" Regular expression for parsing option name/values.
" Allow any amount of whitespaces, followed by separator
" (either ``:`` or ``=``), followed by any amount of whitespace and then
" any characters to eol
let s:OPTCRE = '\v\s*([^:=[:space:]][^:=]*)\s*([:=])\s*(.*)$'

let s:MAX_SECTION_NAME = 4096
let s:MAX_PROPERTY_NAME = 1024
let s:MAX_PROPERTY_VALUE = 4096

lockvar s:SECTCRE s:OPTCRE s:MAX_SECTION_NAME s:MAX_PROPERTY_NAME s:MAX_PROPERTY_VALUE

" }}}2
" === Main ============================================================== {{{1

" Read \p config_filename and return the options applicable to
" \p target_filename.  This is the main entry point in this file.
function! editorconfig_core#ini#read_ini_file(config_filename, target_filename)
    if !filereadable(a:config_filename)
        return {}
    endif

    try
        let l:lines = readfile(a:config_filename)
        if &encoding !=? 'utf-8'
            " strip BOM
            if len(l:lines) > 0 && l:lines[0][:2] ==# "\xEF\xBB\xBF"
                let l:lines[0] = l:lines[0][3:]
            endif
            " convert from UTF-8 to 'encoding'
            call map(l:lines, 'iconv(v:val, "utf-8", &encoding)')
        endif
        let result = s:parse(a:config_filename, a:target_filename, l:lines)
    catch
        " rethrow, but with a prefix since throw 'Vim...' fails.
        throw 'Could not read editorconfig file at ' . v:throwpoint . ': ' . string(v:exception)
    endtry

    return result
endfunction

function! s:parse(config_filename, target_filename, lines)
"    Parse a sectioned setup file.
"    The sections in setup file contains a title line at the top,
"    indicated by a name in square brackets (`[]'), plus key/value
"    options lines, indicated by `name: value' format lines.
"    Continuations are represented by an embedded newline then
"    leading whitespace.  Blank lines, lines beginning with a '#',
"    and just about everything else are ignored.

    let l:in_section = 0
    let l:matching_section = 0
    let l:optname = ''
    let l:lineno = 0
    let l:e = []    " Errors, if any

    let l:options = {}  " Options applicable to this file
    let l:is_root = 0   " Whether a:config_filename declares root=true

    while 1
        if l:lineno == len(a:lines)
            break
        endif

        let l:line = a:lines[l:lineno]
        let l:lineno = l:lineno + 1

        " comment or blank line?
        if editorconfig_core#util#strip(l:line) ==# ''
            continue
        endif
        if l:line =~# '\v^[#;]'
            continue
        endif

        " is it a section header?
        if g:editorconfig_core_vimscript_debug
            echom "Header? <" . l:line . ">"
        endif

        let l:mo = matchlist(l:line, s:SECTCRE)
        if len(l:mo)
            let l:sectname = l:mo[1]
            let l:in_section = 1
            if strlen(l:sectname) > s:MAX_SECTION_NAME
                " Section name too long => ignore the section
                let l:matching_section = 0
            else
                let l:matching_section = s:matches_filename(
                    \ a:config_filename, a:target_filename, l:sectname)
            endif

            if g:editorconfig_core_vimscript_debug
                echom 'In section ' . l:sectname . ', which ' .
                    \ (l:matching_section ? 'matches' : 'does not match')
                    \ ' file ' . a:target_filename . ' (config ' .
                    \ a:config_filename . ')'
            endif

            " So sections can't start with a continuation line
            let l:optname = ''

        " Is it an option line?
        else
            let l:mo = matchlist(l:line, s:OPTCRE)
            if len(l:mo)
                let l:optname = mo[1]
                let l:optval = mo[3]

                if g:editorconfig_core_vimscript_debug
                    echom printf('Saw raw opt <%s>=<%s>', l:optname, l:optval)
                endif

                let l:optval = editorconfig_core#util#strip(l:optval)
                " allow empty values
                if l:optval ==? '""'
                    let l:optval = ''
                endif
                let l:optname = s:optionxform(l:optname)
                if !l:in_section && optname ==? 'root'
                    let l:is_root = (optval ==? 'true')
                endif
                if g:editorconfig_core_vimscript_debug
                    echom printf('Saw opt <%s>=<%s>', l:optname, l:optval)
                endif

                if l:matching_section &&
                            \ strlen(l:optname) <= s:MAX_PROPERTY_NAME &&
                            \ strlen(l:optval) <= s:MAX_PROPERTY_VALUE
                    let l:options[l:optname] = l:optval
                endif
            else
                " a non-fatal parsing error occurred.  set up the
                " exception but keep going. the exception will be
                " raised at the end of the file and will contain a
                " list of all bogus lines
                call add(e, "Parse error in '" . a:config_filename . "' at line " .
                    \ l:lineno . ": '" . l:line . "'")
            endif
        endif
    endwhile

    " if any parsing errors occurred, raise an exception
    if len(l:e)
        throw string(l:e)
    endif

    return {'root': l:is_root, 'options': l:options}
endfunction!

" }}}1
" === Helpers =========================================================== {{{1

" Preprocess option names
function! s:optionxform(optionstr)
    let l:result = substitute(a:optionstr, '\v\s+$', '', 'g')   " rstrip
    return tolower(l:result)
endfunction

" Return true if \p glob matches \p target_filename
function! s:matches_filename(config_filename, target_filename, glob)
"    config_dirname = normpath(dirname(config_filename)).replace(sep, '/')
    let l:config_dirname = fnamemodify(a:config_filename, ':p:h') . '/'

    if editorconfig_core#util#is_win()
        " Regardless of whether shellslash is set, make everything slashes
        let l:config_dirname =
                \ tolower(substitute(l:config_dirname, '\v\\', '/', 'g'))
    endif

    let l:glob = substitute(a:glob, '\v\\([#;])', '\1', 'g')

    " Take account of the path to the editorconfig file.
    " editorconfig-core-c/src/lib/editorconfig.c says:
    "  "Pattern would be: /dir/of/editorconfig/file[double_star]/[section] if
    "   section does not contain '/', or /dir/of/editorconfig/file[section]
    "   if section starts with a '/', or /dir/of/editorconfig/file/[section] if
    "   section contains '/' but does not start with '/'."

    if stridx(l:glob, '/') != -1    " contains a slash
        if l:glob[0] ==# '/'
            let l:glob = l:glob[1:]     " trim leading slash
        endif
" This will be done by fnmatch
"        let l:glob = l:config_dirname . l:glob
    else                            " does not contain a slash
        let l:config_dirname = l:config_dirname[:-2]
            " Trim trailing slash
        let l:glob = '**/' . l:glob
    endif

    if g:editorconfig_core_vimscript_debug
        echom '- ini#matches_filename: checking <' . a:target_filename .
            \ '> against <' . l:glob . '> with respect to config file <' .
            \ a:config_filename . '>'
        echom '- ini#matches_filename: config_dirname is ' . l:config_dirname
    endif

    return editorconfig_core#fnmatch#fnmatch(a:target_filename,
        \ l:config_dirname, l:glob)
endfunction " matches_filename

" }}}1
" === Copyright notices ================================================= {{{2
" Based on code from ConfigParser.py file distributed with Python 2.6.
" Portions Copyright (c) 2001-2010 Python Software Foundation;
" All Rights Reserved.  Licensed under PSF License (see LICENSE.PSF file).
"
" Changes to original ConfigParser:
"
" - Special characters can be used in section names
" - Octothorpe can be used for comments (not just at beginning of line)
" - Only track INI options in sections that match target filename
" - Stop parsing files with when ``root = true`` is found
" }}}2

let &cpo = s:saved_cpo
unlet! s:saved_cpo

" vi: set fdm=marker fdl=1:
