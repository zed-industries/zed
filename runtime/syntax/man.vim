" Vim syntax file
" Language:	Man page
" Maintainer:	Jason Franklin <vim@justemail.net>
" Previous Maintainer:	SungHyun Nam <goweol@gmail.com>
" Previous Maintainer:	Gautam H. Mudunuri <gmudunur@informatica.com>
" Version Info:
" Last Change:	2020 Sep 19

" Additional highlighting by Johannes Tanzler <johannes.tanzler@aon.at>:
"	* manSubHeading
"	* manSynopsis (only for sections 2 and 3)

" quit when a syntax file was already loaded
if exists("b:current_syntax")
  finish
endif

" Get the CTRL-H syntax to handle backspaced text
runtime! syntax/ctrlh.vim

syn case ignore

" See notes about hyphenation in s:ParseIntoPageAndSection of
" autoload/dist/man.vim.
syn match  manReference       "\%(\f\+[\u2010-]\%(\n\|\r\n\=\)\s\+\)\=\f\+([1-9]\l*)"
syn match  manSectionHeading  "^\a.*$"
syn match  manSubHeading      "^\s\{3\}\a.*$"
syn match  manOptionDesc      "^\s*[+-][a-z0-9]\S*"
syn match  manLongOptionDesc  "^\s*--[a-z0-9-]\S*"
" syn match  manHistory		"^[a-z].*last change.*$"

syn match manHeader '\%1l.*'
exe 'syn match manFooter ''\%' . line('$') . 'l.*'''

if getline(1) =~ '^[a-zA-Z_]\+([23])'
  syntax include @cCode <sfile>:p:h/c.vim
  syn match manCFuncDefinition  display "\<\h\w*\>\s*("me=e-1 contained
  syn region manSynopsis start="^SYNOPSIS"hs=s+8 end="^\u\+\s*$"me=e-12 keepend contains=manSectionHeading,@cCode,manCFuncDefinition
endif


" Define the default highlighting.
" Only when an item doesn't have highlighting yet

hi def link manHeader Title
hi def link manFooter PreProc

hi def link manSectionHeading  Statement
hi def link manOptionDesc	    Constant
hi def link manLongOptionDesc  Constant
hi def link manReference	    PreProc
hi def link manSubHeading      Function
hi def link manCFuncDefinition Function


let b:current_syntax = "man"

" vim:ts=8 sts=2 sw=2:
