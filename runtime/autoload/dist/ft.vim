vim9script

# Vim functions for file type detection
#
# Maintainer:		The Vim Project <https://github.com/vim/vim>
# Last Change:		2024 Jan 05
# Former Maintainer:	Bram Moolenaar <Bram@vim.org>

# These functions are moved here from runtime/filetype.vim to make startup
# faster.

export def Check_inp()
  if getline(1) =~ '^\*'
    setf abaqus
  else
    var n = 1
    var nmax = line("$") > 500 ? 500 : line("$")
    while n <= nmax
      if getline(n) =~? "^header surface data"
	setf trasys
	break
      endif
      n += 1
    endwhile
  endif
enddef

# This function checks for the kind of assembly that is wanted by the user, or
# can be detected from the first five lines of the file.
export def FTasm()
  # make sure b:asmsyntax exists
  if !exists("b:asmsyntax")
    b:asmsyntax = ""
  endif

  if b:asmsyntax == ""
    FTasmsyntax()
  endif

  # if b:asmsyntax still isn't set, default to asmsyntax or GNU
  if b:asmsyntax == ""
    if exists("g:asmsyntax")
      b:asmsyntax = g:asmsyntax
    else
      b:asmsyntax = "asm"
    endif
  endif

  exe "setf " .. fnameescape(b:asmsyntax)
enddef

export def FTasmsyntax()
  # see if the file contains any asmsyntax=foo overrides. If so, change
  # b:asmsyntax appropriately
  var head = " " .. getline(1) .. " " .. getline(2) .. " "
	.. getline(3) .. " " .. getline(4) ..  " " .. getline(5) .. " "
  var match = matchstr(head, '\sasmsyntax=\zs[a-zA-Z0-9]\+\ze\s')
  if match != ''
    b:asmsyntax = match
  elseif ((head =~? '\.title') || (head =~? '\.ident') || (head =~? '\.macro') || (head =~? '\.subtitle') || (head =~? '\.library'))
    b:asmsyntax = "vmasm"
  endif
enddef

var ft_visual_basic_content = '\c^\s*\%(Attribute\s\+VB_Name\|Begin\s\+\%(VB\.\|{\%(\x\+-\)\+\x\+}\)\)'

# See FTfrm() for Visual Basic form file detection
export def FTbas()
  if exists("g:filetype_bas")
    exe "setf " .. g:filetype_bas
    return
  endif

  # most frequent FreeBASIC-specific keywords in distro files
  var fb_keywords = '\c^\s*\%(extern\|var\|enum\|private\|scope\|union\|byref\|operator\|constructor\|delete\|namespace\|public\|property\|with\|destructor\|using\)\>\%(\s*[:=(]\)\@!'
  var fb_preproc  = '\c^\s*\%(' ..
                      # preprocessor
                      '#\s*\a\+\|' ..
                      # compiler option
                      'option\s\+\%(byval\|dynamic\|escape\|\%(no\)\=gosub\|nokeyword\|private\|static\)\>\|' ..
                      # metacommand
                      '\%(''\|rem\)\s*\$lang\>\|' ..
                      # default datatype
                      'def\%(byte\|longint\|short\|ubyte\|uint\|ulongint\|ushort\)\>' ..
                    '\)'
  var fb_comment  = "^\\s*/'"

  # OPTION EXPLICIT, without the leading underscore, is common to many dialects
  var qb64_preproc = '\c^\s*\%($\a\+\|option\s\+\%(_explicit\|_\=explicitarray\)\>\)'

  for lnum in range(1, min([line("$"), 100]))
    var line = getline(lnum)
    if line =~ ft_visual_basic_content
      setf vb
      return
    elseif line =~ fb_preproc || line =~ fb_comment || line =~ fb_keywords
      setf freebasic
      return
    elseif line =~ qb64_preproc
      setf qb64
      return
    endif
  endfor
  setf basic
enddef

export def FTbtm()
  if exists("g:dosbatch_syntax_for_btm") && g:dosbatch_syntax_for_btm
    setf dosbatch
  else
    setf btm
  endif
enddef

export def BindzoneCheck(default = '')
  if getline(1) .. getline(2) .. getline(3) .. getline(4)
	  =~ '^; <<>> DiG [0-9.]\+.* <<>>\|$ORIGIN\|$TTL\|IN\s\+SOA'
    setf bindzone
  elseif default != ''
    exe 'setf ' .. default
  endif
enddef

# Returns true if file content looks like RAPID
def IsRapid(sChkExt: string = ""): bool
  if sChkExt == "cfg"
    return getline(1) =~? '\v^%(EIO|MMC|MOC|PROC|SIO|SYS):CFG'
  endif
  # called from FTmod, FTprg or FTsys
  return getline(nextnonblank(1)) =~? '\v^\s*%(\%{3}|module\s+\k+\s*%(\(|$))'
enddef

export def FTcfg()
  if exists("g:filetype_cfg")
    exe "setf " .. g:filetype_cfg
  elseif IsRapid("cfg")
    setf rapid
  else
    setf cfg
  endif
enddef

export def FTcls()
  if exists("g:filetype_cls")
    exe "setf " .. g:filetype_cls
    return
  endif

  var line1 = getline(1)
  if line1 =~ '^#!.*\<\%(rexx\|regina\)\>'
    setf rexx
    return
  elseif line1 == 'VERSION 1.0 CLASS'
    setf vb
    return
  endif

  var nonblank1 = getline(nextnonblank(1))
  if nonblank1 =~ '^\v%(\%|\\)'
    setf tex
  elseif nonblank1 =~ '^\s*\%(/\*\|::\w\)'
    setf rexx
  else
    setf st
  endif
enddef

export def FTlpc()
  if exists("g:lpc_syntax_for_c")
    var lnum = 1
    while lnum <= 12
      if getline(lnum) =~# '^\(//\|inherit\|private\|protected\|nosave\|string\|object\|mapping\|mixed\)'
	setf lpc
	return
      endif
      lnum += 1
    endwhile
  endif
  setf c
enddef

export def FTheader()
  if match(getline(1, min([line("$"), 200])), '^@\(interface\|end\|class\)') > -1
    if exists("g:c_syntax_for_h")
      setf objc
    else
      setf objcpp
    endif
  elseif exists("g:c_syntax_for_h")
    setf c
  elseif exists("g:ch_syntax_for_h")
    setf ch
  else
    setf cpp
  endif
enddef

# This function checks if one of the first ten lines start with a '@'.  In
# that case it is probably a change file.
# If the first line starts with # or ! it's probably a ch file.
# If a line has "main", "include", "//" or "/*" it's probably ch.
# Otherwise CHILL is assumed.
export def FTchange()
  var lnum = 1
  while lnum <= 10
    if getline(lnum)[0] == '@'
      setf change
      return
    endif
    if lnum == 1 && (getline(1)[0] == '#' || getline(1)[0] == '!')
      setf ch
      return
    endif
    if getline(lnum) =~ "MODULE"
      setf chill
      return
    endif
    if getline(lnum) =~ 'main\s*(\|#\s*include\|//'
      setf ch
      return
    endif
    lnum += 1
  endwhile
  setf chill
enddef

export def FTent()
  # This function checks for valid cl syntax in the first five lines.
  # Look for either an opening comment, '#', or a block start, '{'.
  # If not found, assume SGML.
  var lnum = 1
  while lnum < 6
    var line = getline(lnum)
    if line =~ '^\s*[#{]'
      setf cl
      return
    elseif line !~ '^\s*$'
      # Not a blank line, not a comment, and not a block start,
      # so doesn't look like valid cl code.
      break
    endif
    lnum += 1
  endwhile
  setf dtd
enddef

export def ExCheck()
  var lines = getline(1, min([line("$"), 100]))
  if exists('g:filetype_euphoria')
    exe 'setf ' .. g:filetype_euphoria
  elseif match(lines, '^--\|^ifdef\>\|^include\>') > -1
    setf euphoria3
  else
    setf elixir
  endif
enddef

export def EuphoriaCheck()
  if exists('g:filetype_euphoria')
    exe 'setf ' .. g:filetype_euphoria
  else
    setf euphoria3
  endif
enddef

export def DtraceCheck()
  if did_filetype()
    # Filetype was already detected
    return
  endif
  var lines = getline(1, min([line("$"), 100]))
  if match(lines, '^module\>\|^import\>') > -1
    # D files often start with a module and/or import statement.
    setf d
  elseif match(lines, '^#!\S\+dtrace\|#pragma\s\+D\s\+option\|:\S\{-}:\S\{-}:') > -1
    setf dtrace
  else
    setf d
  endif
enddef

export def FTdef()
  if get(g:, "filetype_def", "") == "modula2" || IsModula2()
    SetFiletypeModula2()
    return
  endif

  if exists("g:filetype_def")
    exe "setf " .. g:filetype_def
  else
    setf def
  endif
enddef

export def FTe()
  if exists('g:filetype_euphoria')
    exe 'setf ' .. g:filetype_euphoria
  else
    var n = 1
    while n < 100 && n <= line("$")
      if getline(n) =~ "^\\s*\\(<'\\|'>\\)\\s*$"
	setf specman
	return
      endif
      n += 1
    endwhile
    setf eiffel
  endif
enddef

def IsForth(): bool
  var first_line = nextnonblank(1)

  # SwiftForth block comment (line is usually filled with '-' or '=') or
  # OPTIONAL (sometimes precedes the header comment)
  if getline(first_line) =~? '^\%({\%(\s\|$\)\|OPTIONAL\s\)'
    return true
  endif

  var n = first_line
  while n < 100 && n <= line("$")
    # Forth comments and colon definitions
    if getline(n) =~ '^[:(\\] '
      return true
    endif
    n += 1
  endwhile
  return false
enddef

# Distinguish between Forth and Fortran
export def FTf()
  if exists("g:filetype_f")
    exe "setf " .. g:filetype_f
  elseif IsForth()
    setf forth
  else
    setf fortran
  endif
enddef

export def FTfrm()
  if exists("g:filetype_frm")
    exe "setf " .. g:filetype_frm
    return
  endif

  if getline(1) == "VERSION 5.00"
    setf vb
    return
  endif

  var lines = getline(1, min([line("$"), 5]))

  if match(lines, ft_visual_basic_content) > -1
    setf vb
  else
    setf form
  endif
enddef

# Distinguish between Forth and F#
export def FTfs()
  if exists("g:filetype_fs")
    exe "setf " .. g:filetype_fs
  elseif IsForth()
    setf forth
  else
    setf fsharp
  endif
enddef

# Distinguish between HTML, XHTML and Django
export def FThtml()
  var n = 1
  while n < 10 && n <= line("$")
    if getline(n) =~ '\<DTD\s\+XHTML\s'
      setf xhtml
      return
    endif
    if getline(n) =~ '{%\s*\(extends\|block\|load\)\>\|{#\s\+'
      setf htmldjango
      return
    endif
    n += 1
  endwhile
  setf FALLBACK html
enddef

# Distinguish between standard IDL and MS-IDL
export def FTidl()
  var n = 1
  while n < 50 && n <= line("$")
    if getline(n) =~ '^\s*import\s\+"\(unknwn\|objidl\)\.idl"'
      setf msidl
      return
    endif
    n += 1
  endwhile
  setf idl
enddef

# Distinguish between "default", Prolog and Cproto prototype file.
export def ProtoCheck(default: string)
  # Cproto files have a comment in the first line and a function prototype in
  # the second line, it always ends in ";".  Indent files may also have
  # comments, thus we can't match comments to see the difference.
  # IDL files can have a single ';' in the second line, require at least one
  # chacter before the ';'.
  if getline(2) =~ '.;$'
    setf cpp
  else
    # recognize Prolog by specific text in the first non-empty line
    # require a blank after the '%' because Perl uses "%list" and "%translate"
    var lnum = getline(nextnonblank(1))
    if lnum =~ '\<prolog\>' || lnum =~ '^\s*\(%\+\(\s\|$\)\|/\*\)' || lnum =~ ':-'
      setf prolog
    else
      exe 'setf ' .. default
    endif
  endif
enddef

export def FTm()
  if exists("g:filetype_m")
    exe "setf " .. g:filetype_m
    return
  endif

  # excluding end(for|function|if|switch|while) common to Murphi
  var octave_block_terminators = '\<end\%(_try_catch\|classdef\|enumeration\|events\|methods\|parfor\|properties\)\>'

  var objc_preprocessor = '^\s*#\s*\%(import\|include\|define\|if\|ifn\=def\|undef\|line\|error\|pragma\)\>'

  var n = 1
  var saw_comment = 0  # Whether we've seen a multiline comment leader.
  while n < 100
    var line = getline(n)
    if line =~ '^\s*/\*'
      # /* ... */ is a comment in Objective C and Murphi, so we can't conclude
      # it's either of them yet, but track this as a hint in case we don't see
      # anything more definitive.
      saw_comment = 1
    endif
    if line =~ '^\s*//' || line =~ '^\s*@import\>' || line =~ objc_preprocessor
      setf objc
      return
    endif
    if line =~ '^\s*\%(#\|%!\)' || line =~ '^\s*unwind_protect\>' ||
	  \ line =~ '\%(^\|;\)\s*' .. octave_block_terminators
      setf octave
      return
    endif
    # TODO: could be Matlab or Octave
    if line =~ '^\s*%'
      setf matlab
      return
    endif
    if line =~ '^\s*(\*'
      setf mma
      return
    endif
    if line =~ '^\c\s*\(\(type\|var\)\>\|--\)'
      setf murphi
      return
    endif
    n += 1
  endwhile

  if saw_comment
    # We didn't see anything definitive, but this looks like either Objective C
    # or Murphi based on the comment leader. Assume the former as it is more
    # common.
    setf objc
  else
    # Default is Matlab
    setf matlab
  endif
enddef

export def FTmms()
  var n = 1
  while n < 20
    var line = getline(n)
    if line =~ '^\s*\(%\|//\)' || line =~ '^\*'
      setf mmix
      return
    endif
    if line =~ '^\s*#'
      setf make
      return
    endif
    n += 1
  endwhile
  setf mmix
enddef

# This function checks if one of the first five lines start with a dot.  In
# that case it is probably an nroff file: 'filetype' is set and 1 is returned.
export def FTnroff(): number
  if getline(1)[0] .. getline(2)[0] .. getline(3)[0]
    			.. getline(4)[0] .. getline(5)[0] =~ '\.'
    setf nroff
    return 1
  endif
  return 0
enddef

export def FTmm()
  var n = 1
  while n < 20
    if getline(n) =~ '^\s*\(#\s*\(include\|import\)\>\|@import\>\|/\*\)'
      setf objcpp
      return
    endif
    n += 1
  endwhile
  setf nroff
enddef

# Returns true if file content looks like LambdaProlog module
def IsLProlog(): bool
  # skip apparent comments and blank lines, what looks like
  # LambdaProlog comment may be RAPID header
  var lnum: number = nextnonblank(1)
  while lnum > 0 && lnum < line('$') && getline(lnum) =~ '^\s*%' # LambdaProlog comment
    lnum = nextnonblank(lnum + 1)
  endwhile
  # this pattern must not catch a go.mod file
  return getline(lnum) =~ '\<module\s\+\w\+\s*\.\s*\(%\|$\)'
enddef

def IsModula2(): bool
  return getline(nextnonblank(1)) =~ '\<MODULE\s\+\w\+\s*;\|^\s*(\*'
enddef

def SetFiletypeModula2()
  const KNOWN_DIALECTS = ["iso", "pim", "r10"]
  const KNOWN_EXTENSIONS = ["gm2"]
  const LINE_COUNT = 200
  const TAG = '(\*!m2\(\w\+\)\%(+\(\w\+\)\)\=\*)'

  var dialect = get(g:, "modula2_default_dialect", "pim")
  var extension = get(g:, "modula2_default_extension", "")

  var matches = []

  # ignore unknown dialects or badly formatted tags
  for lnum in range(1, min([line("$"), LINE_COUNT]))
    matches = matchlist(getline(lnum), TAG)
    if !empty(matches)
      if index(KNOWN_DIALECTS, matches[1]) >= 0
         dialect = matches[1]
      endif
      if index(KNOWN_EXTENSIONS, matches[2]) >= 0
         extension = matches[2]
      endif
      break
    endif
  endfor

  modula2#SetDialect(dialect, extension)

  setf modula2
enddef

# Determine if *.mod is ABB RAPID, LambdaProlog, Modula-2, Modsim III or go.mod
export def FTmod()
  if get(g:, "filetype_mod", "") == "modula2" || IsModula2()
    SetFiletypeModula2()
    return
  endif

  if exists("g:filetype_mod")
    exe "setf " .. g:filetype_mod
  elseif expand("<afile>") =~ '\<go.mod$'
    setf gomod
  elseif IsLProlog()
    setf lprolog
  elseif IsRapid()
    setf rapid
  else
    # Nothing recognized, assume modsim3
    setf modsim3
  endif
enddef

export def FTpl()
  if exists("g:filetype_pl")
    exe "setf " .. g:filetype_pl
  else
    # recognize Prolog by specific text in the first non-empty line
    # require a blank after the '%' because Perl uses "%list" and "%translate"
    var line = getline(nextnonblank(1))
    if line =~ '\<prolog\>' || line =~ '^\s*\(%\+\(\s\|$\)\|/\*\)' || line =~ ':-'
      setf prolog
    else
      setf perl
    endif
  endif
enddef

export def FTinc()
  if exists("g:filetype_inc")
    exe "setf " .. g:filetype_inc
  else
    var lines = getline(1) .. getline(2) .. getline(3)
    if lines =~? "perlscript"
      setf aspperl
    elseif lines =~ "<%"
      setf aspvbs
    elseif lines =~ "<?"
      setf php
    # Pascal supports // comments but they're vary rarely used for file
    # headers so assume POV-Ray
    elseif lines =~ '^\s*\%({\|(\*\)' || lines =~? ft_pascal_keywords
      setf pascal
    elseif lines =~# '\<\%(require\|inherit\)\>' || lines =~# '[A-Z][A-Za-z0-9_:${}]*\s\+\%(??\|[?:+]\)\?= '
      setf bitbake
    else
      FTasmsyntax()
      if exists("b:asmsyntax")
        exe "setf " .. fnameescape(b:asmsyntax)
      else
        setf pov
      endif
    endif
  endif
enddef

export def FTprogress_cweb()
  if exists("g:filetype_w")
    exe "setf " .. g:filetype_w
    return
  endif
  if getline(1) =~ '&ANALYZE' || getline(3) =~ '&GLOBAL-DEFINE'
    setf progress
  else
    setf cweb
  endif
enddef

# These include the leading '%' sign
var ft_swig_keywords = '^\s*%\%(addmethods\|apply\|beginfile\|clear\|constant\|define\|echo\|enddef\|endoffile\|extend\|feature\|fragment\|ignore\|import\|importfile\|include\|includefile\|inline\|insert\|keyword\|module\|name\|namewarn\|native\|newobject\|parms\|pragma\|rename\|template\|typedef\|typemap\|types\|varargs\|warn\)'
# This is the start/end of a block that is copied literally to the processor file (C/C++)
var ft_swig_verbatim_block_start = '^\s*%{'

export def FTi()
  if exists("g:filetype_i")
    exe "setf " .. g:filetype_i
    return
  endif
  # This function checks for an assembly comment or a SWIG keyword or verbatim block in the first 50 lines.
  # If not found, assume Progress.
  var lnum = 1
  while lnum <= 50 && lnum < line('$')
    var line = getline(lnum)
    if line =~ '^\s*;' || line =~ '^\*'
      FTasm()
      return
    elseif line =~ ft_swig_keywords || line =~ ft_swig_verbatim_block_start
      setf swig
      return
    endif
    lnum += 1
  endwhile
  setf progress
enddef

var ft_pascal_comments = '^\s*\%({\|(\*\|//\)'
var ft_pascal_keywords = '^\s*\%(program\|unit\|library\|uses\|begin\|procedure\|function\|const\|type\|var\)\>'

export def FTprogress_pascal()
  if exists("g:filetype_p")
    exe "setf " .. g:filetype_p
    return
  endif
  # This function checks for valid Pascal syntax in the first ten lines.
  # Look for either an opening comment or a program start.
  # If not found, assume Progress.
  var lnum = 1
  while lnum <= 10 && lnum < line('$')
    var line = getline(lnum)
    if line =~ ft_pascal_comments || line =~? ft_pascal_keywords
      setf pascal
      return
    elseif line !~ '^\s*$' || line =~ '^/\*'
      # Not an empty line: Doesn't look like valid Pascal code.
      # Or it looks like a Progress /* comment
      break
    endif
    lnum += 1
  endwhile
  setf progress
enddef

export def FTpp()
  if exists("g:filetype_pp")
    exe "setf " .. g:filetype_pp
  else
    var line = getline(nextnonblank(1))
    if line =~ ft_pascal_comments || line =~? ft_pascal_keywords
      setf pascal
    else
      setf puppet
    endif
  endif
enddef

# Determine if *.prg is ABB RAPID. Can also be Clipper, FoxPro or eviews
export def FTprg()
  if exists("g:filetype_prg")
    exe "setf " .. g:filetype_prg
  elseif IsRapid()
    setf rapid
  else
    # Nothing recognized, assume Clipper
    setf clipper
  endif
enddef

export def FTr()
  var max = line("$") > 50 ? 50 : line("$")

  for n in range(1, max)
    # Rebol is easy to recognize, check for that first
    if getline(n) =~? '\<REBOL\>'
      setf rebol
      return
    endif
  endfor

  for n in range(1, max)
    # R has # comments
    if getline(n) =~ '^\s*#'
      setf r
      return
    endif
    # Rexx has /* comments */
    if getline(n) =~ '^\s*/\*'
      setf rexx
      return
    endif
  endfor

  # Nothing recognized, use user default or assume Rexx
  if exists("g:filetype_r")
    exe "setf " .. g:filetype_r
  else
    # Rexx used to be the default, but R appears to be much more popular.
    setf r
  endif
enddef

export def McSetf()
  # Rely on the file to start with a comment.
  # MS message text files use ';', Sendmail files use '#' or 'dnl'
  for lnum in range(1, min([line("$"), 20]))
    var line = getline(lnum)
    if line =~ '^\s*\(#\|dnl\)'
      setf m4  # Sendmail .mc file
      return
    elseif line =~ '^\s*;'
      setf msmessages  # MS Message text file
      return
    endif
  endfor
  setf m4  # Default: Sendmail .mc file
enddef

# Called from filetype.vim and scripts.vim.
# When "setft" is passed and false then the 'filetype' option is not set.
export def SetFileTypeSH(name: string, setft = true): string
  if setft && did_filetype()
    # Filetype was already detected
    return ''
  endif
  if setft && expand("<amatch>") =~ g:ft_ignore_pat
    return ''
  endif
  if name =~ '\<csh\>'
    # Some .sh scripts contain #!/bin/csh.
    return SetFileTypeShell("csh", setft)
  elseif name =~ '\<tcsh\>'
    # Some .sh scripts contain #!/bin/tcsh.
    return SetFileTypeShell("tcsh", setft)
  elseif name =~ '\<zsh\>'
    # Some .sh scripts contain #!/bin/zsh.
    return SetFileTypeShell("zsh", setft)
  elseif name =~ '\<ksh\>'
    b:is_kornshell = 1
    if exists("b:is_bash")
      unlet b:is_bash
    endif
    if exists("b:is_sh")
      unlet b:is_sh
    endif
  elseif exists("g:bash_is_sh") || name =~ '\<bash\>' || name =~ '\<bash2\>'
    b:is_bash = 1
    if exists("b:is_kornshell")
      unlet b:is_kornshell
    endif
    if exists("b:is_sh")
      unlet b:is_sh
    endif
  elseif name =~ '\<sh\>' || name =~ '\<dash\>'
    # Ubuntu links "sh" to "dash", thus it is expected to work the same way
    b:is_sh = 1
    if exists("b:is_kornshell")
      unlet b:is_kornshell
    endif
    if exists("b:is_bash")
      unlet b:is_bash
    endif
  endif

  return SetFileTypeShell("sh", setft)
enddef

# For shell-like file types, check for an "exec" command hidden in a comment,
# as used for Tcl.
# When "setft" is passed and false then the 'filetype' option is not set.
# Also called from scripts.vim, thus can't be local to this script.
export def SetFileTypeShell(name: string, setft = true): string
  if setft && did_filetype()
    # Filetype was already detected
    return ''
  endif
  if setft && expand("<amatch>") =~ g:ft_ignore_pat
    return ''
  endif

  var lnum = 2
  while lnum < 20 && lnum < line("$") && getline(lnum) =~ '^\s*\(#\|$\)'
    # Skip empty and comment lines.
    lnum += 1
  endwhile
  if lnum < line("$") && getline(lnum) =~ '\s*exec\s' && getline(lnum - 1) =~ '^\s*#.*\\$'
    # Found an "exec" line after a comment with continuation
    var n = substitute(getline(lnum), '\s*exec\s\+\([^ ]*/\)\=', '', '')
    if n =~ '\<tclsh\|\<wish'
      if setft
	setf tcl
      endif
      return 'tcl'
    endif
  endif

  if setft
    exe "setf " .. name
  endif
  return name
enddef

export def CSH()
  if did_filetype()
    # Filetype was already detected
    return
  endif
  if exists("g:filetype_csh")
    SetFileTypeShell(g:filetype_csh)
  elseif &shell =~ "tcsh"
    SetFileTypeShell("tcsh")
  else
    SetFileTypeShell("csh")
  endif
enddef

var ft_rules_udev_rules_pattern = '^\s*\cudev_rules\s*=\s*"\([^"]\{-1,}\)/*".*'
export def FTRules()
  var path = expand('<amatch>:p')
  if path =~ '/\(etc/udev/\%(rules\.d/\)\=.*\.rules\|\%(usr/\)\=lib/udev/\%(rules\.d/\)\=.*\.rules\)$'
    setf udevrules
    return
  endif
  if path =~ '^/etc/ufw/'
    setf conf  # Better than hog
    return
  endif
  if path =~ '^/\(etc\|usr/share\)/polkit-1/rules\.d'
    setf javascript
    return
  endif
  var config_lines: list<string>
  try
    config_lines = readfile('/etc/udev/udev.conf')
  catch /^Vim\%((\a\+)\)\=:E484/
    setf hog
    return
  endtry
  var dir = expand('<amatch>:p:h')
  for line in config_lines
    if line =~ ft_rules_udev_rules_pattern
      var udev_rules = substitute(line, ft_rules_udev_rules_pattern, '\1', "")
      if dir == udev_rules
	setf udevrules
      endif
      break
    endif
  endfor
  setf hog
enddef

export def SQL()
  if exists("g:filetype_sql")
    exe "setf " .. g:filetype_sql
  else
    setf sql
  endif
enddef

# This function checks the first 25 lines of file extension "sc" to resolve
# detection between scala and SuperCollider.
# NOTE: We don't check for 'Class : Method', as this can easily be confused
# 	with valid Scala like `val x : Int = 3`. So we instead only rely on
# 	checks that can't be confused.
export def FTsc()
  for lnum in range(1, min([line("$"), 25]))
    if getline(lnum) =~# 'var\s<\|classvar\s<\|\^this.*\||\w\+|\|+\s\w*\s{\|\*ar\s'
      setf supercollider
      return
    endif
  endfor
  setf scala
enddef

# This function checks the first line of file extension "scd" to resolve
# detection between scdoc and SuperCollider
export def FTscd()
  if getline(1) =~# '\%^\S\+(\d[0-9A-Za-z]*)\%(\s\+\"[^"]*\"\%(\s\+\"[^"]*\"\)\=\)\=$'
    setf scdoc
  else
    setf supercollider
  endif
enddef

# If the file has an extension of 't' and is in a directory 't' or 'xt' then
# it is almost certainly a Perl test file.
# If the first line starts with '#' and contains 'perl' it's probably a Perl
# file.
# (Slow test) If a file contains a 'use' statement then it is almost certainly
# a Perl file.
export def FTperl(): number
  var dirname = expand("%:p:h:t")
  if expand("%:e") == 't' && (dirname == 't' || dirname == 'xt')
    setf perl
    return 1
  endif
  if getline(1)[0] == '#' && getline(1) =~ 'perl'
    setf perl
    return 1
  endif
  var save_cursor = getpos('.')
  call cursor(1, 1)
  var has_use = search('^use\s\s*\k', 'c', 30) > 0
  call setpos('.', save_cursor)
  if has_use
    setf perl
    return 1
  endif
  return 0
enddef

# LambdaProlog and Standard ML signature files
export def FTsig()
  if exists("g:filetype_sig")
    exe "setf " .. g:filetype_sig
    return
  endif

  var lprolog_comment = '^\s*\%(/\*\|%\)'
  var lprolog_keyword = '^\s*sig\s\+\a'
  var sml_comment = '^\s*(\*'
  var sml_keyword = '^\s*\%(signature\|structure\)\s\+\a'

  var line = getline(nextnonblank(1))

  if line =~ lprolog_comment || line =~# lprolog_keyword
    setf lprolog
  elseif line =~ sml_comment || line =~# sml_keyword
    setf sml
  endif
enddef

# This function checks the first 100 lines of files matching "*.sil" to
# resolve detection between Swift Intermediate Language and SILE.
export def FTsil()
  for lnum in range(1, [line('$'), 100]->min())
    var line: string = getline(lnum)
    if line =~ '^\s*[\\%]'
      setf sile
      return
    elseif line =~ '^\s*\S'
      setf sil
      return
    endif
  endfor
  # no clue, default to "sil"
  setf sil
enddef

export def FTsys()
  if exists("g:filetype_sys")
    exe "setf " .. g:filetype_sys
  elseif IsRapid()
    setf rapid
  else
    setf bat
  endif
enddef

# Choose context, plaintex, or tex (LaTeX) based on these rules:
# 1. Check the first line of the file for "%&<format>".
# 2. Check the first 1000 non-comment lines for LaTeX or ConTeXt keywords.
# 3. Default to "plain" or to g:tex_flavor, can be set in user's vimrc.
export def FTtex()
  var firstline = getline(1)
  var format: string
  if firstline =~ '^%&\s*\a\+'
    format = tolower(matchstr(firstline, '\a\+'))
    format = substitute(format, 'pdf', '', '')
    if format == 'tex'
      format = 'latex'
    elseif format == 'plaintex'
      format = 'plain'
    endif
  elseif expand('%') =~ 'tex/context/.*/.*.tex'
    format = 'context'
  else
    # Default value, may be changed later:
    format = exists("g:tex_flavor") ? g:tex_flavor : 'plain'
    # Save position, go to the top of the file, find first non-comment line.
    var save_cursor = getpos('.')
    call cursor(1, 1)
    var firstNC = search('^\s*[^[:space:]%]', 'c', 1000)
    if firstNC > 0
      # Check the next thousand lines for a LaTeX or ConTeXt keyword.
      var lpat = 'documentclass\>\|usepackage\>\|begin{\|newcommand\>\|renewcommand\>'
      var cpat = 'start\a\+\|setup\a\+\|usemodule\|enablemode\|enableregime\|setvariables\|useencoding\|usesymbols\|stelle\a\+\|verwende\a\+\|stel\a\+\|gebruik\a\+\|usa\a\+\|imposta\a\+\|regle\a\+\|utilisemodule\>'
      var kwline = search('^\s*\\\%(' .. lpat .. '\)\|^\s*\\\(' .. cpat .. '\)',
			      'cnp', firstNC + 1000)
      if kwline == 1		# lpat matched
	format = 'latex'
      elseif kwline == 2	# cpat matched
	format = 'context'
      endif		# If neither matched, keep default set above.
      # let lline = search('^\s*\\\%(' . lpat . '\)', 'cn', firstNC + 1000)
      # let cline = search('^\s*\\\%(' . cpat . '\)', 'cn', firstNC + 1000)
      # if cline > 0
      #   let format = 'context'
      # endif
      # if lline > 0 && (cline == 0 || cline > lline)
      #   let format = 'tex'
      # endif
    endif # firstNC
    call setpos('.', save_cursor)
  endif # firstline =~ '^%&\s*\a\+'

  # Translation from formats to file types.  TODO:  add AMSTeX, RevTex, others?
  if format == 'plain'
    setf plaintex
  elseif format == 'context'
    setf context
  else # probably LaTeX
    setf tex
  endif
  return
enddef

export def FTxml()
  var n = 1
  while n < 100 && n <= line("$")
    var line = getline(n)
    # DocBook 4 or DocBook 5.
    var is_docbook4 = line =~ '<!DOCTYPE.*DocBook'
    var is_docbook5 = line =~ ' xmlns="http://docbook.org/ns/docbook"'
    if is_docbook4 || is_docbook5
      b:docbk_type = "xml"
      if is_docbook5
	b:docbk_ver = 5
      else
	b:docbk_ver = 4
      endif
      setf docbk
      return
    endif
    if line =~ 'xmlns:xbl="http://www.mozilla.org/xbl"'
      setf xbl
      return
    endif
    n += 1
  endwhile
  setf xml
enddef

export def FTy()
  var n = 1
  while n < 100 && n <= line("$")
    var line = getline(n)
    if line =~ '^\s*%'
      setf yacc
      return
    endif
    if getline(n) =~ '^\s*\(#\|class\>\)' && getline(n) !~ '^\s*#\s*include'
      setf racc
      return
    endif
    n += 1
  endwhile
  setf yacc
enddef

export def Redif()
  var lnum = 1
  while lnum <= 5 && lnum < line('$')
    if getline(lnum) =~ "^\ctemplate-type:"
      setf redif
      return
    endif
    lnum += 1
  endwhile
enddef

# This function is called for all files under */debian/patches/*, make sure not
# to non-dep3patch files, such as README and other text files.
export def Dep3patch()
  if expand('%:t') ==# 'series'
    return
  endif

  for ln in getline(1, 100)
    if ln =~# '^\%(Description\|Subject\|Origin\|Bug\|Forwarded\|Author\|From\|Reviewed-by\|Acked-by\|Last-Updated\|Applied-Upstream\):'
      setf dep3patch
      return
    elseif ln =~# '^---'
      # end of headers found. stop processing
      return
    endif
  endfor
enddef

# This function checks the first 15 lines for appearance of 'FoamFile'
# and then 'object' in a following line.
# In that case, it's probably an OpenFOAM file
export def FTfoam()
    var ffile = 0
    var lnum = 1
    while lnum <= 15
      if getline(lnum) =~# '^FoamFile'
	ffile = 1
      elseif ffile == 1 && getline(lnum) =~# '^\s*object'
	setf foam
	return
      endif
      lnum += 1
    endwhile
enddef

# Determine if a *.tf file is TF mud client or terraform
export def FTtf()
  var numberOfLines = line('$')
  for i in range(1, numberOfLines)
    var currentLine = trim(getline(i))
    var firstCharacter = currentLine[0]
    if firstCharacter !=? ";" && firstCharacter !=? "/" && firstCharacter !=? ""
      setf terraform
      return
    endif
  endfor
  setf tf
enddef

var ft_krl_header = '\&\w+'
# Determine if a *.src file is Kuka Robot Language
export def FTsrc()
  var ft_krl_def_or_deffct = '%(global\s+)?def%(fct)?>'
  if exists("g:filetype_src")
    exe "setf " .. g:filetype_src
  elseif getline(nextnonblank(1)) =~? '\v^\s*%(' .. ft_krl_header .. '|' .. ft_krl_def_or_deffct .. ')'
    setf krl
  endif
enddef

# Determine if a *.dat file is Kuka Robot Language
export def FTdat()
  var ft_krl_defdat = 'defdat>'
  if exists("g:filetype_dat")
    exe "setf " .. g:filetype_dat
  elseif getline(nextnonblank(1)) =~? '\v^\s*%(' .. ft_krl_header .. '|' .. ft_krl_defdat .. ')'
    setf krl
  endif
enddef

export def FTlsl()
  if exists("g:filetype_lsl")
    exe "setf " .. g:filetype_lsl
  endif

  var line = getline(nextnonblank(1))
  if line =~ '^\s*%' || line =~# ':\s*trait\s*$'
    setf larch
  else
    setf lsl
  endif
enddef

export def FTtyp()
  if exists("g:filetype_typ")
    exe "setf " .. g:filetype_typ
    return
  endif

  # Look for SQL type definition syntax
  for line in getline(1, 200)
    # SQL type files may define the casing
    if line =~ '^CASE\s\==\s\=\(SAME\|LOWER\|UPPER\|OPPOSITE\)$'
      setf sql
      return
    endif

    # SQL type files may define some types as follows
    if line =~ '^TYPE\s.*$'
      setf sql
      return
    endif
  endfor

  # Otherwise, affect the typst filetype
  setf typst
enddef

# Set the filetype of a *.v file to Verilog, V or Cog based on the first 200
# lines.
export def FTv()
  if did_filetype()
    # ":setf" will do nothing, bail out early
    return
  endif
  if exists("g:filetype_v")
    exe "setf " .. g:filetype_v
    return
  endif

  var in_comment = 0
  for lnum in range(1, min([line("$"), 200]))
    var line = getline(lnum)
    # Skip Verilog and V comments (lines and blocks).
    if line =~ '^\s*/\*'
      # start comment block
      in_comment = 1
    endif
    if in_comment == 1
      if line =~ '\*/'
        # end comment block
        in_comment = 0
      endif
      # skip comment-block line
      continue
    endif
    if line =~ '^\s*//'
      # skip comment line
      continue
    endif

    # Coq: line ends with a '.' followed by an optional variable number of
    # spaces or contains the start of a comment, but not inside a Verilog or V
    # comment.
    # Example: "Definition x := 10. (*".
    if (line =~ '\.\s*$' && line !~ '/[/*]') || (line =~ '(\*' && line !~ '/[/*].*(\*')
      setf coq
      return
    endif

    # Verilog: line ends with ';' followed by an optional variable number of
    # spaces and an optional start of a comment.
    # Example: " b <= a + 1; // Add 1".
    if line =~ ';\s*\(/[/*].*\)\?$'
      setf verilog
      return
    endif
  endfor

  # No line matched, fall back to "v".
  setf v
enddef

export def FTvba()
  if getline(1) =~ '^["#] Vimball Archiver'
    setf vim
  else
    setf vb
  endif
enddef

# Uncomment this line to check for compilation errors early
defcompile
