$ !
$ !=====================================================================
$ !
$ !	VimTutor.com	version 29-Aug-2002
$ !
$ !	Author: Tom Wyant <Thomas.R.Wyant-III@usa.dupont.com>
$ !
$ !	This DCL command procedure executes the vimtutor command
$ !	(surprise, surprise!) which gives you a brief tutorial on the
$ !	VIM editor. Languages other than the default are supported in
$ !	the usual way, as are at least some of the command qualifiers,
$ !	though you'll need to play some fairly serious games with DCL
$ !	to specify ones that need quoting.
$ !
$ !	Copyright (c) 2002 E. I. DuPont de Nemours and Company, Inc
$ !
$ !	This program is free software; you can redistribute it and/or
$ !	modify it under the terms of the VIM license as available from
$ !	the vim 6.1 ":help license" command or (at your option) the
$ !	license from any later version of vim.
$ !
$ !	This program is distributed in the hope that it will be useful,
$ !	but WITHOUT ANY WARRANTY; without even the implied warranty of
$ !	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
$ !
$ !=====================================================================
$ !
$ !
$ !	Check for the existence of VIM, and die if it isn't there.
$ !
$	if f$search ("vim:vim.exe") .eqs. ""
$	then
$	    write sys$error "Error - Can't run tutorial. VIM not found."
$	    exit
$	endif
$ !
$ !
$ !	Pick up the argument, if any.
$ !
$	inx = 0
$ arg_loop:
$	inx = inx + 1
$	if f$type (p'inx') .nes. ""
$	then
$	    if p'inx' .nes. "" .and. f$locate ("-", p'inx') .ne. 0
$	    then
$		xx = p'inx'
$		assign/nolog "''xx'" xx
$		p'inx' = ""
$	    endif
$	    goto arg_loop
$	endif
$ !
$ !
$ !	Make sure we clean up our toys when we're through playing.
$ !
$	on error then goto exit
$ !
$ !
$ !	Create the VIM foreign command if needed
$ !
$	if f$type (vim) .eqs. "" then vim := $vim:vim
$ !
$ !
$ !	Build the name for our temp file.
$ !
$	tutfil = "sys$login:vimtutor_" + -
		f$edit (f$getjpi (0, "pid"), "trim") + "."
$	assign/nolog 'tutfil' TUTORCOPY
$ !
$ !
$ !	Copy the selected file to the temp file
$ !
$	assign/nolog/user nla0: sys$error
$	assign/nolog/user nla0: sys$output
$	vim -u "NONE" -c "so $VIMRUNTIME/tutor/tutor.vim"
$ !
$ !
$ !	Run the tutorial
$ !
$	assign/nolog/user sys$command sys$input
$	vim -u "NONE" -c "set nocp" 'p1' 'p2' 'p3' 'p4' 'p5' 'p6' 'p7' 'p8' 'tutfil'
$ !
$ !
$ !	Ditch the copy.
$ !
$ exit:
$	if f$type (tutfil) .nes. "" .and. f$search (tutfil) .nes. "" then -
$	    delete 'tutfil';*
$	if f$type (xx) .nes. "" then deassign xx
$	deassign TUTORCOPY
$	exit
$ !
$ !=====================================================================
$ !
$ !		Modification history
$ !
$ !	29-Aug-2002	T. R. Wyant
$ !		Changed license to vim.
$ !		Fix error "input is not from a terminal"
$ !		Juggle documentation (copyright and contact to front,
$ !			modification history to end).
$ !	25-Jul-2002	T. R. Wyant
$ !		Initial version
