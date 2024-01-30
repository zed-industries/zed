" autoload/editorconfig.vim: EditorConfig native Vimscript plugin
" Copyright (c) 2011-2019 EditorConfig Team
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
" POSSIBILITY OF SUCH DAMAGE.
"

if v:version < 700
    finish
endif

let s:saved_cpo = &cpo
set cpo&vim

" {{{1 variables
let s:hook_list = []

function! editorconfig#AddNewHook(func) " {{{1
    " Add a new hook

    call add(s:hook_list, a:func)
endfunction

function! editorconfig#ApplyHooks(config) abort " {{{1
    " apply hooks

    for Hook in s:hook_list
        let l:hook_ret = Hook(a:config)

        if type(l:hook_ret) != type(0) && l:hook_ret != 0
            " TODO print some debug info here
        endif
    endfor
endfunction

" }}}

let &cpo = s:saved_cpo
unlet! s:saved_cpo

" vim: fdm=marker fdc=3
