" Vim syntax file
" Language: Comshare Dimension Definition Language
" Maintainer:	Raul Segura Acevedo <raulseguraaceved@netscape.net>
" Last change:	2016 Sep 20

" quit when a syntax file was already loaded
if exists("b:current_syntax")
	finish
endif

sy case ignore
sy sync fromstart
sy keyword	cdlStatement	dimension hierarchy group grouphierarchy schedule class
sy keyword	cdlType		add update file category main altername removeall required notrequired
sy keyword	cdlConditional	if then elseif else endif and or not cons rpt xlt
sy keyword	cdlFunction	ChildOf IChildOf LeafChildOf DescendantOf IDescendantOf LeafDescendantOf MemberIs CountOf

sy keyword	cdlIdentifier	contained id name desc description xlttype precision symbol curr_ name group_name rate_name
sy keyword	cdlIdentifier	contained xcheck endbal accounttype natsign consolidate formula pctown usage periodicity
sy match	cdlIdentifier	contained 'child\s*name'
sy match	cdlIdentifier	contained 'parent\s*name'
sy match	cdlIdentifier	contained 'grp\s*description'
sy match	cdlIdentifier	contained 'grpchild\s*name'
sy match	cdlIdentifier	contained 'grpparent\s*name'
sy match	cdlIdentifier	contained 'preceding\s*member'
sy match	cdlIdentifier	contained 'unit\s*name'
sy match	cdlIdentifier	contained 'unit\s*id'
sy match	cdlIdentifier	contained 'schedule\s*name'
sy match	cdlIdentifier	contained 'schedule\s*id'

sy match	cdlString	/\[[^]]*]/	contains=cdlRestricted,cdlNotSupported
sy match	cdlRestricted	contained /[&*,_]/
" not supported
sy match	cdlNotSupported	contained /[:"!']/

sy keyword	cdlTodo		contained TODO FIXME XXX
sy cluster	cdlCommentGroup contains=cdlTodo
sy match	cdlComment	'//.*' contains=@cdlCommentGroup
sy region	cdlComment	start="/\*" end="\*/" contains=@cdlCommentGroup fold
sy match	cdlCommentE	"\*/"

sy region	cdlParen	transparent start='(' end=')' contains=ALLBUT,cdlParenE,cdlRestricted,cdlNotSupported
"sy region	cdlParen	transparent start='(' end=')' contains=cdlIdentifier,cdlComment,cdlParenWordE
sy match	cdlParenE	")"
"sy match	cdlParenWordE	contained "\k\+"

sy keyword	cdlFxType	allocation downfoot expr xltgain
"sy keyword	cdlFxType	contained allocation downfoot expr xltgain
"sy region	cdlFx		transparent start='\k\+(' end=')' contains=cdlConditional,cdlFunction,cdlString,cdlComment,cdlFxType

set foldmethod=expr
set foldexpr=(getline(v:lnum+1)=~'{'\|\|getline(v:lnum)=~'//\\s\\*\\{5}.*table')?'>1':1
%foldo!
set foldmethod=manual
let b:match_words='\<if\>:\<then\>:\<elseif\>:\<else\>:\<endif\>'

" Define the default highlighting.
" Only when an item doesn't have highlighting yet

hi def link cdlStatement	Statement
hi def link cdlType		Type
hi def link cdlFxType	Type
hi def link cdlIdentifier	Identifier
hi def link cdlString	String
hi def link cdlRestricted	WarningMsg
hi def link cdlNotSupported	ErrorMsg
hi def link cdlTodo		Todo
hi def link cdlComment	Comment
hi def link cdlCommentE	ErrorMsg
hi def link cdlParenE	ErrorMsg
hi def link cdlParenWordE	ErrorMsg
hi def link cdlFunction	Function
hi def link cdlConditional	Conditional


let b:current_syntax = "cdl"

" vim: ts=8
