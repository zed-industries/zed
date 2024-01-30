" Vim filetype plugin autoload file
" Language:	man
" Maintainer:	Jason Franklin <jason@oneway.dev>
" Maintainer:	SungHyun Nam <goweol@gmail.com>
" Autoload Split: Bram Moolenaar
" Last Change: 	2024 Jan 17 (make it work on AIX, see #13847)

let s:cpo_save = &cpo
set cpo-=C

let s:man_tag_depth = 0

let s:man_sect_arg = ""
let s:man_find_arg = "-w"
try
  if !has("win32") && $OSTYPE !~ 'cygwin\|linux'
    " cache the value
    let uname_s = system('uname -s')

    if uname_s =~ "SunOS" && system('uname -r') =~ "^5"
      " Special Case for Man on SunOS
      let s:man_sect_arg = "-s"
      let s:man_find_arg = "-l"
    elseif uname_s =~? 'AIX'
      " Special Case for Man on AIX
      let s:man_sect_arg = ""
      let s:man_find_arg = ""
    endif
  endif
catch /E145:/
  " Ignore the error in restricted mode
endtry

unlet! uname_s

func s:ParseIntoPageAndSection()
  " Accommodate a reference that terminates in a hyphen.
  "
  " See init_charset_table() at
  " https://git.savannah.gnu.org/cgit/groff.git/tree/src/roff/troff/input.cpp?h=1.22.4#n6794
  "
  " See can_break_after() at
  " https://git.savannah.gnu.org/cgit/groff.git/tree/src/roff/troff/charinfo.h?h=1.22.4#n140
  "
  " Assumptions and limitations:
  " 1) Manual-page references (in consequence of command-related filenames)
  "    do not contain non-ASCII HYPHENs (0x2010), any terminating HYPHEN
  "    must have been introduced to mark division of a word at the end of
  "    a line and can be discarded; whereas similar references may contain
  "    ASCII HYPHEN-MINUSes (0x002d) and any terminating HYPHEN-MINUS forms
  "    a compound word in addition to marking word division.
  " 2) Well-formed manual-page references always have a section suffix, e.g.
  "    "git-commit(1)", therefore suspended hyphenated compounds are not
  "    determined, e.g.     [V] (With cursor at _git-merge-_ below...)
  "    ".................... git-merge- and git-merge-base. (See git-cherry-
  "    pick(1) and git-cherry(1).)" (... look up "git-merge-pick(1)".)
  "
  " Note that EM DASH (0x2014), a third stooge from init_charset_table(),
  " neither connects nor divides parts of a word.
  let str = expand("<cWORD>")

  if str =~ '\%u2010$'	" HYPHEN (-1).
    let str = strpart(str, 0, strridx(str, "\u2010"))

    " Append the leftmost WORD (or an empty string) from the line below.
    let str .= get(split(get(getbufline(bufnr('%'), line('.') + 1), 0, '')), 0, '')
  elseif str =~ '-$'	" HYPHEN-MINUS.
    " Append the leftmost WORD (or an empty string) from the line below.
    let str .= get(split(get(getbufline(bufnr('%'), line('.') + 1), 0, '')), 0, '')
  endif

  " According to man(1), section name formats vary (MANSECT):
  " 1 n l 8 3 2 3posix 3pm 3perl 3am 5 4 9 6 7
  let parts = matchlist(str, '\(\k\+\)(\(\k\+\))')
  return (len(parts) > 2)
	  \ ? {'page': parts[1], 'section': parts[2]}
	  \ : {'page': matchstr(str, '\k\+'), 'section': ''}
endfunc

func dist#man#PreGetPage(cnt)
  if a:cnt == 0
    let what = s:ParseIntoPageAndSection()
    let sect = what.section
    let page = what.page
  else
    let what = s:ParseIntoPageAndSection()
    let sect = a:cnt
    let page = what.page
  endif

  call dist#man#GetPage('', sect, page)
endfunc

func s:GetCmdArg(sect, page)
  if empty(a:sect)
    return shellescape(a:page)
  endif

  return s:man_sect_arg . ' ' . shellescape(a:sect) . ' ' . shellescape(a:page)
endfunc

func s:FindPage(sect, page)
  let l:cmd = printf('man %s %s', s:man_find_arg, s:GetCmdArg(a:sect, a:page))
  call system(l:cmd)

  if v:shell_error
    return 0
  endif

  return 1
endfunc

func dist#man#GetPage(cmdmods, ...)
  if a:0 >= 2
    let sect = a:1
    let page = a:2
  elseif a:0 >= 1
    let sect = ""
    let page = a:1
  else
    return
  endif

  " To support:	    nmap K :Man <cWORD><CR>
  if page ==? '<cword>'
    let what = s:ParseIntoPageAndSection()
    let sect = what.section
    let page = what.page
  endif

  if !exists('g:ft_man_no_sect_fallback') || (g:ft_man_no_sect_fallback == 0)
    if sect != "" && s:FindPage(sect, page) == 0
      let sect = ""
    endif
  endif
  if s:FindPage(sect, page) == 0
    let msg = 'man.vim: no manual entry for "' . page . '"'
    if !empty(sect)
      let msg .= ' in section ' . sect
    endif
    echomsg msg
    return
  endif
  exec "let s:man_tag_buf_".s:man_tag_depth." = ".bufnr("%")
  exec "let s:man_tag_lin_".s:man_tag_depth." = ".line(".")
  exec "let s:man_tag_col_".s:man_tag_depth." = ".col(".")
  let s:man_tag_depth = s:man_tag_depth + 1

  let open_cmd = 'edit'

  " Use an existing "man" window if it exists, otherwise open a new one.
  if &filetype != "man"
    let thiswin = winnr()
    exe "norm! \<C-W>b"
    if winnr() > 1
      exe "norm! " . thiswin . "\<C-W>w"
      while 1
	if &filetype == "man"
	  break
	endif
	exe "norm! \<C-W>w"
	if thiswin == winnr()
	  break
	endif
      endwhile
    endif
    if &filetype != "man"
      if exists("g:ft_man_open_mode")
        if g:ft_man_open_mode == 'vert'
	  let open_cmd = 'vsplit'
        elseif g:ft_man_open_mode == 'tab'
	  let open_cmd = 'tabedit'
        else
	  let open_cmd = 'split'
        endif
      else
	let open_cmd = a:cmdmods . ' split'
      endif
    endif
  endif

  silent execute open_cmd . " $HOME/" . page . '.' . sect . '~'

  " Avoid warning for editing the dummy file twice
  setl buftype=nofile noswapfile

  setl fdc=0 ma nofen nonu nornu
  %delete _
  let unsetwidth = 0
  if empty($MANWIDTH)
    let $MANWIDTH = winwidth(0)
    let unsetwidth = 1
  endif

  " Ensure Vim is not recursively invoked (man-db does this) when doing ctrl-[
  " on a man page reference by unsetting MANPAGER.
  " Some versions of env(1) do not support the '-u' option, and in such case
  " we set MANPAGER=cat.
  if !exists('s:env_has_u')
    call system('env -u x true')
    let s:env_has_u = (v:shell_error == 0)
  endif
  let env_cmd = s:env_has_u ? 'env -u MANPAGER' : 'env MANPAGER=cat'
  let env_cmd .= ' GROFF_NO_SGR=1'
  let man_cmd = env_cmd . ' man ' . s:GetCmdArg(sect, page)

  silent exec "r !" . man_cmd

  " Emulate piping the buffer through the "col -b" command.
  " Ref: https://github.com/vim/vim/issues/12301
  exe 'silent! keepjumps keeppatterns %s/\v(.)\b\ze\1?//e' .. (&gdefault ? '' : 'g')

  if unsetwidth
    let $MANWIDTH = ''
  endif
  " Remove blank lines from top and bottom.
  while line('$') > 1 && getline(1) =~ '^\s*$'
    1delete _
  endwhile
  while line('$') > 1 && getline('$') =~ '^\s*$'
    $delete _
  endwhile
  1
  setl ft=man nomod
  setl bufhidden=hide
  setl nobuflisted
  setl noma
endfunc

func dist#man#PopPage()
  if s:man_tag_depth > 0
    let s:man_tag_depth = s:man_tag_depth - 1
    exec "let s:man_tag_buf=s:man_tag_buf_".s:man_tag_depth
    exec "let s:man_tag_lin=s:man_tag_lin_".s:man_tag_depth
    exec "let s:man_tag_col=s:man_tag_col_".s:man_tag_depth

    exec s:man_tag_buf."b"
    call cursor(s:man_tag_lin, s:man_tag_col)

    exec "unlet s:man_tag_buf_".s:man_tag_depth
    exec "unlet s:man_tag_lin_".s:man_tag_depth
    exec "unlet s:man_tag_col_".s:man_tag_depth
    unlet s:man_tag_buf s:man_tag_lin s:man_tag_col
  endif
endfunc

let &cpo = s:cpo_save
unlet s:cpo_save

" vim: set sw=2 ts=8 noet:
