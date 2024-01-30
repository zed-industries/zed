" util.vim: part of editorconfig-core-vimscript and editorconfig-vim.
" Copyright (c) 2018-2019 EditorConfig Team, including Chris White {{{1
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

" A verbatim copy of ingo#fs#path#Separator()  {{{1
" from https://github.com/vim-scripts/ingo-library/blob/558132e2221db3af26dc2f2c6756d092d48a459f/autoload/ingo/fs/path.vim
" distributed under the Vim license.
function! editorconfig_core#util#Separator()
    return (exists('+shellslash') && ! &shellslash ? '\' : '/')
endfunction " }}}1

" path_join(): ('a','b')->'a/b'; ('a/','b')->'a/b'. {{{1
function! editorconfig_core#util#path_join(a, b)
    " TODO shellescape/shellslash?
    "echom 'Joining <' . a:a . '> and <' . a:b . '>'
    "echom 'Length is ' . strlen(a:a)
    "echom 'Last char is ' . char2nr(a:a[-1])
    if a:a !~# '\v%(\/|\\)$'
        return a:a . editorconfig_core#util#Separator() . a:b
    else
        return a:a . a:b
    endif
endfunction " }}}1

" is_win() by xolox {{{1
" The following function is modified from
" https://github.com/xolox/vim-misc/blob/master/autoload/xolox/misc/os.vim
" Copyright (c) 2015 Peter Odding <peter@peterodding.com>
"
" Permission is hereby granted, free of charge, to any person obtaining a copy
" of this software and associated documentation files (the "Software"), to deal
" in the Software without restriction, including without limitation the rights
" to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
" copies of the Software, and to permit persons to whom the Software is
" furnished to do so, subject to the following conditions:
"
" The above copyright notice and this permission notice shall be included in all
" copies or substantial portions of the Software.
"
" THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
" IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
" FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
" AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
" LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
" OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
" SOFTWARE.
function! editorconfig_core#util#is_win()
    " Returns 1 (true) when on Microsoft Windows, 0 (false) otherwise.
    return has('win16') || has('win32') || has('win64')
endfunction " }}}1

" strip() {{{1
function! editorconfig_core#util#strip(s)
    return substitute(a:s, '\v^\s+|\s+$','','g')
endfunction " }}}1

let &cpo = s:saved_cpo
unlet! s:saved_cpo

" vi: set fdm=marker:
