vim9script

# Vim function for detecting a filetype from the file contents.
# Invoked from "scripts.vim" in 'runtimepath'
#
# Maintainer:	The Vim Project <https://github.com/vim/vim>
# Last Change:	2023 Aug 10
# Former Maintainer:	Bram Moolenaar <Bram@vim.org>

export def DetectFiletype()
  var line1 = getline(1)
  if line1[0] == '#' && line1[1] == '!'
    # File that starts with "#!".
    DetectFromHashBang(line1)
  else
    # File does not start with "#!".
    DetectFromText(line1)
  endif
enddef

# Called for a script that has "#!" in the first line.
def DetectFromHashBang(firstline: string)
  var line1 = firstline

  # Check for a line like "#!/usr/bin/env {options} bash".  Turn it into
  # "#!/usr/bin/bash" to make matching easier.
  # Recognize only a few {options} that are commonly used.
  if line1 =~ '^#!\s*\S*\<env\s'
    line1 = substitute(line1, '\S\+=\S\+', '', 'g')
    line1 = substitute(line1, '\(-[iS]\|--ignore-environment\|--split-string\)', '', '')
    line1 = substitute(line1, '\<env\s\+', '', '')
  endif

  # Get the program name.
  # Only accept spaces in PC style paths: "#!c:/program files/perl [args]".
  # If the word env is used, use the first word after the space:
  # "#!/usr/bin/env perl [path/args]"
  # If there is no path use the first word: "#!perl [path/args]".
  # Otherwise get the last word after a slash: "#!/usr/bin/perl [path/args]".
  var name: string
  if line1 =~ '^#!\s*\a:[/\\]'
    name = substitute(line1, '^#!.*[/\\]\(\i\+\).*', '\1', '')
  elseif line1 =~ '^#!.*\<env\>'
    name = substitute(line1, '^#!.*\<env\>\s\+\(\i\+\).*', '\1', '')
  elseif line1 =~ '^#!\s*[^/\\ ]*\>\([^/\\]\|$\)'
    name = substitute(line1, '^#!\s*\([^/\\ ]*\>\).*', '\1', '')
  else
    name = substitute(line1, '^#!\s*\S*[/\\]\(\f\+\).*', '\1', '')
  endif

  # tcl scripts may have #!/bin/sh in the first line and "exec wish" in the
  # third line.  Suggested by Steven Atkinson.
  if getline(3) =~ '^exec wish'
    name = 'wish'
  endif

  var ft = Exe2filetype(name, line1)
  if ft != ''
    exe 'setl ft=' .. ft
  endif
enddef

# Returns the filetype name associated with program "name".
# "line1" is the #! line at the top of the file.  Use the same as "name" if
# not available.
# Returns an empty string when not recognized.
export def Exe2filetype(name: string, line1: string): string
    # Bourne-like shell scripts: bash bash2 dash ksh ksh93 sh
  if name =~ '^\(bash\d*\|dash\|ksh\d*\|sh\)\>'
    return dist#ft#SetFileTypeSH(line1, false)

    # csh scripts
  elseif name =~ '^csh\>'
    return dist#ft#SetFileTypeShell(exists("g:filetype_csh") ? g:filetype_csh : 'csh', false)

    # tcsh scripts
  elseif name =~ '^tcsh\>'
    return dist#ft#SetFileTypeShell("tcsh", false)

    # Z shell scripts
  elseif name =~ '^zsh\>'
    return 'zsh'

    # TCL scripts
  elseif name =~ '^\(tclsh\|wish\|expectk\|itclsh\|itkwish\)\>'
    return 'tcl'

    # Expect scripts
  elseif name =~ '^expect\>'
    return 'expect'

    # Gnuplot scripts
  elseif name =~ '^gnuplot\>'
    return 'gnuplot'

    # Makefiles
  elseif name =~ 'make\>'
    return 'make'

    # Pike
  elseif name =~ '^pike\%(\>\|[0-9]\)'
    return 'pike'

    # Lua
  elseif name =~ 'lua'
    return 'lua'

    # Perl
  elseif name =~ 'perl'
    return 'perl'

    # PHP
  elseif name =~ 'php'
    return 'php'

    # Python
  elseif name =~ 'python'
    return 'python'

    # Groovy
  elseif name =~ '^groovy\>'
    return 'groovy'

    # Raku
  elseif name =~ 'raku'
    return 'raku'

    # Ruby
  elseif name =~ 'ruby'
    return 'ruby'

    # JavaScript
  elseif name =~ 'node\(js\)\=\>\|js\>' || name =~ 'rhino\>'
    return 'javascript'

    # BC calculator
  elseif name =~ '^bc\>'
    return 'bc'

    # sed
  elseif name =~ 'sed\>'
    return 'sed'

    # OCaml-scripts
  elseif name =~ 'ocaml'
    return 'ocaml'

    # Awk scripts; also finds "gawk"
  elseif name =~ 'awk\>'
    return 'awk'

    # Website MetaLanguage
  elseif name =~ 'wml'
    return 'wml'

    # Scheme scripts
  elseif name =~ 'scheme'
    return 'scheme'

    # CFEngine scripts
  elseif name =~ 'cfengine'
    return 'cfengine'

    # Erlang scripts
  elseif name =~ 'escript'
    return 'erlang'

    # Haskell
  elseif name =~ 'haskell'
    return 'haskell'

    # Scala
  elseif name =~ 'scala\>'
    return 'scala'

    # Clojure
  elseif name =~ 'clojure'
    return 'clojure'

    # Free Pascal
  elseif name =~ 'instantfpc\>'
    return 'pascal'

    # Fennel
  elseif name =~ 'fennel\>'
    return 'fennel'

    # MikroTik RouterOS script
  elseif name =~ 'rsc\>'
    return 'routeros'

    # Fish shell
  elseif name =~ 'fish\>'
    return 'fish'

    # Gforth
  elseif name =~ 'gforth\>'
    return 'forth'

    # Icon
  elseif name =~ 'icon\>'
    return 'icon'

    # Guile
  elseif name =~ 'guile'
    return 'scheme'

    # Nix
  elseif name =~ 'nix-shell'
    return 'nix'

    # Crystal
  elseif name =~ '^crystal\>'
    return 'crystal'

    # Rexx
  elseif name =~ '^\%(rexx\|regina\)\>'
    return 'rexx'

    # Janet
  elseif name =~ '^janet\>'
    return 'janet'

    # Dart
  elseif name =~ '^dart\>'
    return 'dart'

    # Execline (s6)
  elseif name =~ '^execlineb\>'
    return 'execline'

  endif

  return ''
enddef


# Called for a script that does not have "#!" in the first line.
def DetectFromText(line1: string)
  var line2 = getline(2)
  var line3 = getline(3)
  var line4 = getline(4)
  var line5 = getline(5)

  # Bourne-like shell scripts: sh ksh bash bash2
  if line1 =~ '^:$'
    call dist#ft#SetFileTypeSH(line1)

  # Z shell scripts
  elseif line1 =~ '^#compdef\>'
      || line1 =~ '^#autoload\>'
      || "\n" .. line1 .. "\n" .. line2 .. "\n" .. line3 ..
	 "\n" .. line4 .. "\n" .. line5
	 =~ '\n\s*emulate\s\+\%(-[LR]\s\+\)\=[ckz]\=sh\>'
    setl ft=zsh

  # ELM Mail files
  elseif line1 =~ '^From \([a-zA-Z][a-zA-Z_0-9\.=-]*\(@[^ ]*\)\=\|-\) .* \(19\|20\)\d\d$'
    setl ft=mail

  # Mason
  elseif line1 =~ '^<[%&].*>'
    setl ft=mason

  # Vim scripts (must have '" vim' as the first line to trigger this)
  elseif line1 =~ '^" *[vV]im$'
    setl ft=vim

  # libcxx and libstdc++ standard library headers like "iostream" do not have
  # an extension, recognize the Emacs file mode.
  elseif line1 =~? '-\*-.*C++.*-\*-'
    setl ft=cpp

  # MOO
  elseif line1 =~ '^\*\* LambdaMOO Database, Format Version \%([1-3]\>\)\@!\d\+ \*\*$'
    setl ft=moo

    # Diff file:
    # - "diff" in first line (context diff)
    # - "Only in " in first line
    # - "--- " in first line and "+++ " in second line (unified diff).
    # - "*** " in first line and "--- " in second line (context diff).
    # - "# It was generated by makepatch " in the second line (makepatch diff).
    # - "Index: <filename>" in the first line (CVS file)
    # - "=== ", line of "=", "---", "+++ " (SVK diff)
    # - "=== ", "--- ", "+++ " (bzr diff, common case)
    # - "=== (removed|added|renamed|modified)" (bzr diff, alternative)
    # - "# HG changeset patch" in first line (Mercurial export format)
  elseif line1 =~ '^\(diff\>\|Only in \|\d\+\(,\d\+\)\=[cda]\d\+\>\|# It was generated by makepatch \|Index:\s\+\f\+\r\=$\|===== \f\+ \d\+\.\d\+ vs edited\|==== //\f\+#\d\+\|# HG changeset patch\)'
	 || (line1 =~ '^--- ' && line2 =~ '^+++ ')
	 || (line1 =~ '^\* looking for ' && line2 =~ '^\* comparing to ')
	 || (line1 =~ '^\*\*\* ' && line2 =~ '^--- ')
	 || (line1 =~ '^=== ' && ((line2 =~ '^=\{66\}' && line3 =~ '^--- ' && line4 =~ '^+++') || (line2 =~ '^--- ' && line3 =~ '^+++ ')))
	 || (line1 =~ '^=== \(removed\|added\|renamed\|modified\)')
    setl ft=diff

    # PostScript Files (must have %!PS as the first line, like a2ps output)
  elseif line1 =~ '^%![ \t]*PS'
    setl ft=postscr

    # M4 scripts: Guess there is a line that starts with "dnl".
  elseif line1 =~ '^\s*dnl\>'
	 || line2 =~ '^\s*dnl\>'
	 || line3 =~ '^\s*dnl\>'
	 || line4 =~ '^\s*dnl\>'
	 || line5 =~ '^\s*dnl\>'
    setl ft=m4

    # AmigaDos scripts
  elseif $TERM == "amiga" && (line1 =~ "^;" || line1 =~? '^\.bra')
    setl ft=amiga

    # SiCAD scripts (must have procn or procd as the first line to trigger this)
  elseif line1 =~? '^ *proc[nd] *$'
    setl ft=sicad

    # Purify log files start with "****  Purify"
  elseif line1 =~ '^\*\*\*\*  Purify'
    setl ft=purifylog

    # XML
  elseif line1 =~ '<?\s*xml.*?>'
    setl ft=xml

    # XHTML (e.g.: PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN")
  elseif line1 =~ '\<DTD\s\+XHTML\s'
    setl ft=xhtml

    # HTML (e.g.: <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN")
    # Avoid "doctype html", used by slim.
  elseif line1 =~? '<!DOCTYPE\s\+html\>'
    setl ft=html

    # PDF
  elseif line1 =~ '^%PDF-'
    setl ft=pdf

    # XXD output
  elseif line1 =~ '^\x\{7}: \x\{2} \=\x\{2} \=\x\{2} \=\x\{2} '
    setl ft=xxd

    # RCS/CVS log output
  elseif line1 =~ '^RCS file:' || line2 =~ '^RCS file:'
    setl ft=rcslog

    # CVS commit
  elseif line2 =~ '^CVS:' || getline("$") =~ '^CVS: '
    setl ft=cvs

    # Prescribe
  elseif line1 =~ '^!R!'
    setl ft=prescribe

    # Send-pr
  elseif line1 =~ '^SEND-PR:'
    setl ft=sendpr

    # SNNS files
  elseif line1 =~ '^SNNS network definition file'
    setl ft=snnsnet
  elseif line1 =~ '^SNNS pattern definition file'
    setl ft=snnspat
  elseif line1 =~ '^SNNS result file'
    setl ft=snnsres

    # Virata
  elseif line1 =~ '^%.\{-}[Vv]irata'
	 || line2 =~ '^%.\{-}[Vv]irata'
	 || line3 =~ '^%.\{-}[Vv]irata'
	 || line4 =~ '^%.\{-}[Vv]irata'
	 || line5 =~ '^%.\{-}[Vv]irata'
    setl ft=virata

    # Strace
    # inaccurate fast match first, then use accurate slow match
  elseif (line1 =~ 'execve(' && line1 =~ '^[0-9:. ]*execve(')
	   || line1 =~ '^__libc_start_main'
    setl ft=strace

    # VSE JCL
  elseif line1 =~ '^\* $$ JOB\>' || line1 =~ '^// *JOB\>'
    setl ft=vsejcl

    # TAK and SINDA
  elseif line4 =~ 'K & K  Associates' || line2 =~ 'TAK 2000'
    setl ft=takout
  elseif line3 =~ 'S Y S T E M S   I M P R O V E D '
    setl ft=sindaout
  elseif getline(6) =~ 'Run Date: '
    setl ft=takcmp
  elseif getline(9) =~ 'Node    File  1'
    setl ft=sindacmp

    # DNS zone files
  elseif line1 .. line2 .. line3 .. line4 =~ '^; <<>> DiG [0-9.]\+.* <<>>\|$ORIGIN\|$TTL\|IN\s\+SOA'
    setl ft=bindzone

    # BAAN
  elseif line1 =~ '|\*\{1,80}' && line2 =~ 'VRC '
	 || line2 =~ '|\*\{1,80}' && line3 =~ 'VRC '
    setl ft=baan

    # Valgrind
  elseif line1 =~ '^==\d\+== valgrind' || line3 =~ '^==\d\+== Using valgrind'
    setl ft=valgrind

    # Go docs
  elseif line1 =~ '^PACKAGE DOCUMENTATION$'
    setl ft=godoc

    # Renderman Interface Bytestream
  elseif line1 =~ '^##RenderMan'
    setl ft=rib

    # Scheme scripts
  elseif line1 =~ 'exec\s\+\S*scheme' || line2 =~ 'exec\s\+\S*scheme'
    setl ft=scheme

    # Git output
  elseif line1 =~ '^\(commit\|tree\|object\) \x\{40,\}\>\|^tag \S\+$'
    setl ft=git

    # Gprof (gnu profiler)
  elseif line1 == 'Flat profile:'
	&& line2 == ''
	&& line3 =~ '^Each sample counts as .* seconds.$'
    setl ft=gprof

    # Erlang terms
    # (See also: http://www.gnu.org/software/emacs/manual/html_node/emacs/Choosing-Modes.html#Choosing-Modes)
  elseif line1 =~? '-\*-.*erlang.*-\*-'
    setl ft=erlang

    # YAML
  elseif line1 =~ '^%YAML'
    setl ft=yaml

    # MikroTik RouterOS script
  elseif line1 =~ '^#.*by RouterOS.*$'
    setl ft=routeros

    # Sed scripts
    # #ncomment is allowed but most likely a false positive so require a space
    # before any trailing comment text
  elseif line1 =~ '^#n\%($\|\s\)'
    setl ft=sed

  else
    var lnum = 1
    while getline(lnum) =~ "^? " && lnum < line("$")
      lnum += 1
    endwhile
    if getline(lnum) =~ '^Index:\s\+\f\+$'
      # CVS diff
      setl ft=diff

      # locale input files: Formal Definitions of Cultural Conventions
      # filename must be like en_US, fr_FR@euro or en_US.UTF-8
    elseif expand("%") =~ '\a\a_\a\a\($\|[.@]\)\|i18n$\|POSIX$\|translit_'
      lnum = 1
      while lnum < 100 && lnum < line("$")
	if getline(lnum) =~ '^LC_\(IDENTIFICATION\|CTYPE\|COLLATE\|MONETARY\|NUMERIC\|TIME\|MESSAGES\|PAPER\|TELEPHONE\|MEASUREMENT\|NAME\|ADDRESS\)$'
	  setf fdcc
	  break
	endif
	lnum += 1
      endwhile
    endif
  endif
enddef
