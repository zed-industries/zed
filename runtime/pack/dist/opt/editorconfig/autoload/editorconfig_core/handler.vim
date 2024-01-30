" autoload/editorconfig_core/handler.vim: Main worker for
" editorconfig-core-vimscript and editorconfig-vim.
" Modified from the Python core's handler.py.

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

let s:saved_cpo = &cpo
set cpo&vim

" Return full filepath for filename in each directory in and above path. {{{1
" Input path must be an absolute path.
" TODO shellslash/shellescape?
function! s:get_filenames(path, config_filename)
    let l:path = a:path
    let l:path_list = []
    while 1
        call add(l:path_list, editorconfig_core#util#path_join(l:path, a:config_filename))
        let l:newpath = fnamemodify(l:path, ':h')
        if l:path ==? l:newpath || !strlen(l:path)
            break
        endif
        let l:path = l:newpath
    endwhile
    return l:path_list
endfunction " get_filenames

" }}}1
" === Main ============================================================== {{{1

" Find EditorConfig files and return all options matching target_filename.
" Throws on failure.
" @param job    {Dictionary}    required 'target'; optional 'config' and 'version'
function! editorconfig_core#handler#get_configurations(job)
    " TODO? support VERSION checks?

"    Special exceptions that may be raised by this function include:
"    - ``VersionError``: self.version is invalid EditorConfig version
"    - ``PathError``: self.filepath is not a valid absolute filepath
"    - ``ParsingError``: improperly formatted EditorConfig file found

    let l:job = deepcopy(a:job)
    if has_key(l:job, 'config')
        let l:config_filename = l:job.config
    else
        let l:config_filename = '.editorconfig'
        let l:job.config = l:config_filename
    endif

    if has_key(l:job, 'version')
        let l:version = l:job.version
    else
        let l:version = editorconfig_core#version()
        let l:job.version = l:version
    endif

    let l:target_filename = l:job.target

    "echom 'Beginning job ' . string(l:job)
    if !s:check_assertions(l:job)
        throw "Assertions failed"
    endif

    let l:fullpath = fnamemodify(l:target_filename,':p')
    let l:path = fnamemodify(l:fullpath, ':h')
    let l:conf_files = s:get_filenames(l:path, l:config_filename)

    " echom 'fullpath ' . l:fullpath
    " echom 'path ' . l:path

    let l:retval = {}

    " Attempt to find and parse every EditorConfig file in filetree
    for l:conf_fn in l:conf_files
        "echom 'Trying ' . l:conf_fn
        let l:parsed = editorconfig_core#ini#read_ini_file(l:conf_fn, l:target_filename)
        if !has_key(l:parsed, 'options')
            continue
        endif
        " echom '  Has options'

        " Merge new EditorConfig file's options into current options
        let l:old_options = l:retval
        let l:retval = l:parsed.options
        " echom 'Old options ' . string(l:old_options)
        " echom 'New options ' . string(l:retval)
        call extend(l:retval, l:old_options, 'force')

        " Stop parsing if parsed file has a ``root = true`` option
        if l:parsed.root
            break
        endif
    endfor

    call s:preprocess_values(l:job, l:retval)
    return l:retval
endfunction " get_configurations

function! s:check_assertions(job)
" TODO
"    """Raise error if filepath or version have invalid values"""

"    # Raise ``PathError`` if filepath isn't an absolute path
"    if not os.path.isabs(self.filepath):
"        raise PathError("Input file must be a full path name.")

    " Throw if version specified is greater than current
    let l:v = a:job.version
    let l:us = editorconfig_core#version()
    " echom 'Comparing requested version ' . string(l:v) .
    "     \ ' to our version ' . string(l:us)
    if l:v[0] > l:us[0] || l:v[1] > l:us[1] || l:v[2] > l:us[2]
        throw 'Required version ' . string(l:v) .
                    \ ' is greater than the current version ' . string(l:us)
    endif

    return 1    " All OK if we got here
endfunction " check_assertions

" }}}1

" Preprocess option values for consumption by plugins.  {{{1
" Modifies its argument in place.
function! s:preprocess_values(job, opts)

    " Lowercase option value for certain options
    for l:name in ['end_of_line', 'indent_style', 'indent_size',
                \ 'insert_final_newline', 'trim_trailing_whitespace',
                \ 'charset']
        if has_key(a:opts, l:name)
            let a:opts[l:name] = tolower(a:opts[l:name])
        endif
    endfor

    " Set indent_size to "tab" if indent_size is unspecified and
    " indent_style is set to "tab", provided we are at least v0.10.0.
    if get(a:opts, 'indent_style', '') ==? "tab" &&
                \ !has_key(a:opts, 'indent_size') &&
                \ ( a:job.version[0]>0 || a:job.version[1] >=10 )
        let a:opts['indent_size'] = 'tab'
    endif

    " Set tab_width to indent_size if indent_size is specified and
    " tab_width is unspecified
    if has_key(a:opts, 'indent_size') && !has_key(a:opts, 'tab_width') &&
                \ get(a:opts, 'indent_size', '') !=? "tab"
        let a:opts['tab_width'] = a:opts['indent_size']
    endif

    " Set indent_size to tab_width if indent_size is "tab"
    if has_key(a:opts, 'indent_size') && has_key(a:opts, 'tab_width') &&
                \ get(a:opts, 'indent_size', '') ==? "tab"
        let a:opts['indent_size'] = a:opts['tab_width']
    endif
endfunction " preprocess_values

" }}}1

let &cpo = s:saved_cpo
unlet! s:saved_cpo

" vi: set fdm=marker fdl=1:
