" Tests for the terminal window.
" This is split in two, because it can take a lot of time.
" See test_terminal.vim and test_terminal3.vim for further tests.

source check.vim
CheckFeature terminal

source shared.vim
source screendump.vim
source mouse.vim
source term_util.vim

let $PROMPT_COMMAND=''

func Test_terminal_termwinsize_option_fixed()
  CheckRunVimInTerminal
  set termwinsize=6x40
  let text = []
  for n in range(10)
    call add(text, repeat(n, 50))
  endfor
  call writefile(text, 'Xwinsize', 'D')
  let buf = RunVimInTerminal('Xwinsize', {})
  let win = bufwinid(buf)
  call assert_equal([6, 40], term_getsize(buf))
  call assert_equal(6, winheight(win))
  call assert_equal(40, winwidth(win))

  " resizing the window doesn't resize the terminal.
  resize 10
  vertical resize 60
  call assert_equal([6, 40], term_getsize(buf))
  call assert_equal(10, winheight(win))
  call assert_equal(60, winwidth(win))

  call StopVimInTerminal(buf)

  call assert_fails('set termwinsize=40', 'E474:')
  call assert_fails('set termwinsize=10+40', 'E474:')
  call assert_fails('set termwinsize=abc', 'E474:')

  set termwinsize=
endfunc

func Test_terminal_termwinsize_option_zero()
  set termwinsize=0x0
  let buf = Run_shell_in_terminal({})
  let win = bufwinid(buf)
  call assert_equal([winheight(win), winwidth(win)], term_getsize(buf))
  call StopShellInTerminal(buf)
  exe buf . 'bwipe'

  set termwinsize=7x0
  let buf = Run_shell_in_terminal({})
  let win = bufwinid(buf)
  call assert_equal([7, winwidth(win)], term_getsize(buf))
  call StopShellInTerminal(buf)
  exe buf . 'bwipe'

  set termwinsize=0x33
  let buf = Run_shell_in_terminal({})
  let win = bufwinid(buf)
  call assert_equal([winheight(win), 33], term_getsize(buf))
  call StopShellInTerminal(buf)
  exe buf . 'bwipe'

  " This used to crash Vim
  set termwinsize=10000*10000
  let buf = Run_shell_in_terminal({})
  let win = bufwinid(buf)
  call assert_equal([1000, 1000], term_getsize(buf))
  call StopShellInTerminal(buf)
  exe buf . 'bwipe'

  set termwinsize=
endfunc

func Test_terminal_termwinsize_minimum()
  set termwinsize=10*50
  vsplit
  let buf = Run_shell_in_terminal({})
  let win = bufwinid(buf)
  call assert_inrange(10, 1000, winheight(win))
  call assert_inrange(50, 1000, winwidth(win))
  call assert_equal([winheight(win), winwidth(win)], term_getsize(buf))

  resize 15
  vertical resize 60
  redraw
  call assert_equal([15, 60], term_getsize(buf))
  call assert_equal(15, winheight(win))
  call assert_equal(60, winwidth(win))

  resize 7
  vertical resize 30
  redraw
  call assert_equal([10, 50], term_getsize(buf))
  call assert_equal(7, winheight(win))
  call assert_equal(30, winwidth(win))

  call StopShellInTerminal(buf)
  exe buf . 'bwipe'

  set termwinsize=0*0
  let buf = Run_shell_in_terminal({})
  let win = bufwinid(buf)
  call assert_equal([winheight(win), winwidth(win)], term_getsize(buf))
  call StopShellInTerminal(buf)
  exe buf . 'bwipe'

  set termwinsize=
endfunc

func Test_terminal_termwinsize_overruled()
  let cmd = GetDummyCmd()
  set termwinsize=5x43
  let buf = term_start(cmd, #{term_rows: 7, term_cols: 50})
  call TermWait(buf)
  call assert_equal([7, 50], term_getsize(buf))
  exe "bwipe! " .. buf

  let buf = term_start(cmd, #{term_cols: 50})
  call TermWait(buf)
  call assert_equal([5, 50], term_getsize(buf))
  exe "bwipe! " .. buf

  let buf = term_start(cmd, #{term_rows: 7})
  call TermWait(buf)
  call assert_equal([7, 43], term_getsize(buf))
  exe "bwipe! " .. buf

  set termwinsize=
endfunc

" hidden terminal must not change current window size
func Test_terminal_hidden_winsize()
  let cmd = GetDummyCmd()
  let rows = winheight(0)
  let buf = term_start(cmd, #{hidden: 1, term_rows: 10})
  call TermWait(buf)
  call assert_equal(rows, winheight(0))
  call assert_equal([10, &columns], term_getsize(buf))
  exe "bwipe! " .. buf
endfunc

func Test_terminal_termwinkey()
  " make three tabpages, terminal in the middle
  0tabnew
  tabnext
  tabnew
  tabprev
  call assert_equal(1, winnr('$'))
  call assert_equal(2, tabpagenr())
  let thiswin = win_getid()

  let buf = Run_shell_in_terminal({})
  let termwin = bufwinid(buf)
  set termwinkey=<C-L>
  call feedkeys("\<C-L>w", 'tx')
  call assert_equal(thiswin, win_getid())
  call feedkeys("\<C-W>w", 'tx')
  call assert_equal(termwin, win_getid())

  if has('langmap')
    set langmap=xjyk
    call feedkeys("\<C-L>x", 'tx')
    call assert_equal(thiswin, win_getid())
    call feedkeys("\<C-W>y", 'tx')
    call assert_equal(termwin, win_getid())
    set langmap=
  endif

  call feedkeys("\<C-L>gt", "xt")
  call assert_equal(3, tabpagenr())
  tabprev
  call assert_equal(2, tabpagenr())
  call assert_equal(termwin, win_getid())

  call feedkeys("\<C-L>gT", "xt")
  call assert_equal(1, tabpagenr())
  tabnext
  call assert_equal(2, tabpagenr())
  call assert_equal(termwin, win_getid())

  let job = term_getjob(buf)
  call feedkeys("\<C-L>\<C-C>", 'tx')
  call WaitForAssert({-> assert_equal("dead", job_status(job))})

  set termwinkey&
  tabnext
  tabclose
  tabprev
  tabclose
endfunc

func Test_terminal_out_err()
  CheckUnix

  call writefile([
	\ '#!/bin/sh',
	\ 'echo "this is standard error" >&2',
	\ 'echo "this is standard out" >&1',
	\ ], 'Xechoerrout.sh', 'D')
  call setfperm('Xechoerrout.sh', 'rwxrwx---')

  let outfile = 'Xtermstdout'
  let buf = term_start(['./Xechoerrout.sh'], {'out_io': 'file', 'out_name': outfile})
  call TermWait(buf)

  call WaitFor({-> !empty(readfile(outfile)) && !empty(term_getline(buf, 1))})
  call assert_equal(['this is standard out'], readfile(outfile))
  call assert_equal('this is standard error', term_getline(buf, 1))

  call WaitForAssert({-> assert_equal('dead', job_status(term_getjob(buf)))})
  exe buf . 'bwipe'
  call delete(outfile)
endfunc

func Test_termwinscroll()
  CheckUnix
  " TODO: Somehow this test sometimes hangs in the GUI
  CheckNotGui
  let g:test_is_flaky = 1

  " Let the terminal output more than 'termwinscroll' lines, some at the start
  " will be dropped.
  exe 'set termwinscroll=' . &lines
  let buf = term_start('/bin/sh')
  call TermWait(buf)
  for i in range(1, &lines)
    call feedkeys("echo " . i . "\<CR>", 'xt')
    call WaitForAssert({-> assert_match(string(i), term_getline(buf, term_getcursor(buf)[0] - 1))})
  endfor
  " Go to Terminal-Normal mode to update the buffer.
  call feedkeys("\<C-W>N", 'xt')
  call assert_inrange(&lines, &lines * 110 / 100 + winheight(0), line('$'))

  " Every "echo nr" must only appear once
  let lines = getline(1, line('$'))
  for i in range(&lines - len(lines) / 2 + 2, &lines)
    let filtered = filter(copy(lines), {idx, val -> val =~ 'echo ' . i . '\>'})
    call assert_equal(1, len(filtered), 'for "echo ' . i . '"')
  endfor

  exe buf . 'bwipe!'
endfunc

" Resizing the terminal window caused an ml_get error.
" TODO: This does not reproduce the original problem.
" TODO: This test starts timing out in Github CI Gui test, why????
func Test_terminal_resize()
  if has('gui_running') && expand('$GITHUB_ACTIONS') ==# 'true'
    throw 'Skipped: FIXME: this test times-out in Github Actions CI with GUI. Why?'
  endif
  set statusline=x
  terminal
  call assert_equal(2, winnr('$'))
  let buf = bufnr()

  " Wait for the shell to display a prompt
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})

  " Fill the terminal with text.
  if has('win32')
    call feedkeys("dir\<CR>", 'xt')
  else
    call feedkeys("ls\<CR>", 'xt')
  endif
  " Wait for some output
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 3))})

  " Go to Terminal-Normal mode for a moment.
  call feedkeys("\<C-W>N", 'xt')
  " Open a new window
  call feedkeys("i\<C-W>n", 'xt')
  call assert_equal(3, winnr('$'))
  redraw

  close
  call assert_equal(2, winnr('$'))
  call feedkeys("exit\<CR>", 'xt')
  call TermWait(buf)
  set statusline&
endfunc

" TODO: This test starts timing out in Github CI Gui test, why????
func Test_terminal_resize2()
  CheckNotMSWindows
  if has('gui_running') && expand('$GITHUB_ACTIONS') ==# 'true'
    throw 'Skipped: FIXME: this test times-out in Github Actions CI with GUI. Why?'
  endif
  set statusline=x
  terminal
  call assert_equal(2, winnr('$'))
  let buf = bufnr()

  " Wait for the shell to display a prompt
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})

  " This used to crash Vim
  call feedkeys("printf '\033[8;99999;99999t'\<CR>", 'xt')
  redraw

  call feedkeys("exit\<CR>", 'xt')
  call TermWait(buf)
  set statusline&
endfunc

" must be nearly the last, we can't go back from GUI to terminal
func Test_zz1_terminal_in_gui()
  CheckCanRunGui

  " Ignore the "failed to create input context" error.
  call test_ignore_error('E285:')

  gui -f

  call assert_equal(1, winnr('$'))
  let buf = Run_shell_in_terminal({'term_finish': 'close'})
  call StopShellInTerminal(buf)

  " closing window wipes out the terminal buffer a with finished job
  call WaitForAssert({-> assert_equal(1, winnr('$'))})
  call assert_equal("", bufname(buf))

  unlet g:job
endfunc

func Test_zz2_terminal_guioptions_bang()
  CheckGui
  set guioptions+=!

  let filename = 'Xtestscript'
  if has('win32')
    let filename .= '.bat'
    let prefix = ''
    let contents = ['@echo off', 'exit %1']
  else
    let filename .= '.sh'
    let prefix = './'
    let contents = ['#!/bin/sh', 'exit $1']
  endif
  call writefile(contents, filename, 'D')
  call setfperm(filename, 'rwxrwx---')

  " Check if v:shell_error is equal to the exit status.
  let exitval = 0
  execute printf(':!%s%s %d', prefix, filename, exitval)
  call assert_equal(exitval, v:shell_error)

  let exitval = 9
  execute printf(':!%s%s %d', prefix, filename, exitval)
  call assert_equal(exitval, v:shell_error)

  set guioptions&
endfunc

func Test_terminal_hidden()
  CheckUnix

  term ++hidden cat
  let bnr = bufnr('$')
  call assert_equal('terminal', getbufvar(bnr, '&buftype'))
  exe 'sbuf ' . bnr
  call assert_equal('terminal', &buftype)
  call term_sendkeys(bnr, "asdf\<CR>")
  call WaitForAssert({-> assert_match('asdf', term_getline(bnr, 2))})
  call term_sendkeys(bnr, "\<C-D>")
  call WaitForAssert({-> assert_equal('finished', bnr->term_getstatus())})
  bwipe!
endfunc

func Test_terminal_switch_mode()
  term
  let bnr = bufnr('$')
  call WaitForAssert({-> assert_equal('running', term_getstatus(bnr))})
  " In the GUI the first switch sometimes doesn't work.  Switch twice to avoid
  " flakiness.
  call feedkeys("\<C-W>N", 'xt')
  call feedkeys("A", 'xt')
  call WaitForAssert({-> assert_equal('running', term_getstatus(bnr))})
  call feedkeys("\<C-W>N", 'xt')
  call WaitForAssert({-> assert_equal('running,normal', term_getstatus(bnr))})
  call feedkeys("A", 'xt')
  call WaitForAssert({-> assert_equal('running', term_getstatus(bnr))})
  call feedkeys("\<C-\>\<C-N>", 'xt')
  call WaitForAssert({-> assert_equal('running,normal', term_getstatus(bnr))})
  call feedkeys("I", 'xt')
  call WaitForAssert({-> assert_equal('running', term_getstatus(bnr))})
  call feedkeys("\<C-W>Nv", 'xt')
  call WaitForAssert({-> assert_equal('running,normal', term_getstatus(bnr))})
  call feedkeys("I", 'xt')
  call WaitForAssert({-> assert_equal('running', term_getstatus(bnr))})
  call feedkeys("\<C-W>Nv", 'xt')
  call WaitForAssert({-> assert_equal('running,normal', term_getstatus(bnr))})
  call feedkeys("A", 'xt')
  call WaitForAssert({-> assert_equal('running', term_getstatus(bnr))})
  bwipe!
endfunc

func Test_terminal_normal_mode()
  CheckRunVimInTerminal

  " Run Vim in a terminal and open a terminal window to run Vim in.
  let lines =<< trim END
    call setline(1, range(11111, 11122))
    3
  END
  call writefile(lines, 'XtermNormal', 'D')
  let buf = RunVimInTerminal('-S XtermNormal', {'rows': 8})
  call TermWait(buf)

  call term_sendkeys(buf, "\<C-W>N")
  call term_sendkeys(buf, ":set number cursorline culopt=both\r")
  call VerifyScreenDump(buf, 'Test_terminal_normal_1', {})

  call term_sendkeys(buf, ":set culopt=number\r")
  call VerifyScreenDump(buf, 'Test_terminal_normal_2', {})

  call term_sendkeys(buf, ":set culopt=line\r")
  call VerifyScreenDump(buf, 'Test_terminal_normal_3', {})

  call assert_fails('call term_sendkeys(buf, [])', 'E730:')
  call term_sendkeys(buf, "a:q!\<CR>:q\<CR>:q\<CR>")
  call StopVimInTerminal(buf)
endfunc

func Test_terminal_hidden_and_close()
  CheckUnix

  call assert_equal(1, winnr('$'))
  term ++hidden ++close ls
  let bnr = bufnr('$')
  call assert_equal('terminal', getbufvar(bnr, '&buftype'))
  call WaitForAssert({-> assert_false(bufexists(bnr))})
  call assert_equal(1, winnr('$'))
endfunc

func Test_terminal_does_not_truncate_last_newlines()
  if has('conpty')
    throw 'Skipped: fail on ConPTY'
  endif
  let g:test_is_flaky = 1
  let contents = [
  \   [ 'One', '', 'X' ],
  \   [ 'Two', '', '' ],
  \   [ 'Three' ] + repeat([''], 30)
  \ ]

  for c in contents
    call writefile(c, 'Xdntfile', 'D')
    if has('win32')
      term cmd /c type Xdntfile
    else
      term cat Xdntfile
    endif
    let bnr = bufnr('$')
    call assert_equal('terminal', getbufvar(bnr, '&buftype'))
    call WaitForAssert({-> assert_equal('finished', term_getstatus(bnr))})
    sleep 100m
    call assert_equal(c, getline(1, line('$')))
    quit
  endfor
endfunc

func GetDummyCmd()
  if has('win32')
    return 'cmd /c ""'
  else
    CheckExecutable false
    return 'false'
  endif
endfunc

func Test_terminal_no_job()
  let cmd = GetDummyCmd()
  let term = term_start(cmd, {'term_finish': 'close'})
  call WaitForAssert({-> assert_equal(v:null, term_getjob(term)) })
endfunc

func Test_term_getcursor()
  CheckUnix

  let buf = Run_shell_in_terminal({})

  " Wait for the shell to display a prompt.
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})

  " Hide the cursor.
  call term_sendkeys(buf, "echo -e '\\033[?25l'\r")
  call WaitForAssert({-> assert_equal(0, term_getcursor(buf)[2].visible)})

  " Show the cursor.
  call term_sendkeys(buf, "echo -e '\\033[?25h'\r")
  call WaitForAssert({-> assert_equal(1, buf->term_getcursor()[2].visible)})

  " Change color of cursor.
  call WaitForAssert({-> assert_equal('', term_getcursor(buf)[2].color)})
  call term_sendkeys(buf, "echo -e '\\033]12;blue\\007'\r")
  call WaitForAssert({-> assert_equal('blue', term_getcursor(buf)[2].color)})
  call term_sendkeys(buf, "echo -e '\\033]12;green\\007'\r")
  call WaitForAssert({-> assert_equal('green', term_getcursor(buf)[2].color)})

  " Make cursor a blinking block.
  call term_sendkeys(buf, "echo -e '\\033[1 q'\r")
  call WaitForAssert({-> assert_equal([1, 1],
  \ [term_getcursor(buf)[2].blink, term_getcursor(buf)[2].shape])})

  " Make cursor a steady block.
  call term_sendkeys(buf, "echo -e '\\033[2 q'\r")
  call WaitForAssert({-> assert_equal([0, 1],
  \ [term_getcursor(buf)[2].blink, term_getcursor(buf)[2].shape])})

  " Make cursor a blinking underline.
  call term_sendkeys(buf, "echo -e '\\033[3 q'\r")
  call WaitForAssert({-> assert_equal([1, 2],
  \ [term_getcursor(buf)[2].blink, term_getcursor(buf)[2].shape])})

  " Make cursor a steady underline.
  call term_sendkeys(buf, "echo -e '\\033[4 q'\r")
  call WaitForAssert({-> assert_equal([0, 2],
  \ [term_getcursor(buf)[2].blink, term_getcursor(buf)[2].shape])})

  " Make cursor a blinking vertical bar.
  call term_sendkeys(buf, "echo -e '\\033[5 q'\r")
  call WaitForAssert({-> assert_equal([1, 3],
  \ [term_getcursor(buf)[2].blink, term_getcursor(buf)[2].shape])})

  " Make cursor a steady vertical bar.
  call term_sendkeys(buf, "echo -e '\\033[6 q'\r")
  call WaitForAssert({-> assert_equal([0, 3],
  \ [term_getcursor(buf)[2].blink, term_getcursor(buf)[2].shape])})

  call StopShellInTerminal(buf)
endfunc

" Test for term_gettitle()
func Test_term_gettitle()
  " term_gettitle() returns an empty string for a non-terminal buffer
  " and for a non-existing buffer.
  call assert_equal('', bufnr('%')->term_gettitle())
  call assert_equal('', term_gettitle(bufnr('$') + 1))

  if !has('title') || empty(&t_ts)
    throw "Skipped: can't get/set title"
  endif

  let term = term_start([GetVimProg(), '--clean', '-c', 'set noswapfile', '-c', 'set title'])
  call TermWait(term)
  " When Vim is running as a server then the title ends in VIM{number}, thus
  " optionally match a number after "VIM".
  call WaitForAssert({-> assert_match('^\[No Name\] - VIM\d*$', term_gettitle(term)) })
  call term_sendkeys(term, ":e Xfoo\r")
  call WaitForAssert({-> assert_match('^Xfoo (.*[/\\]testdir) - VIM\d*$', term_gettitle(term)) })

  call term_sendkeys(term, ":set titlestring=foo\r")
  call WaitForAssert({-> assert_equal('foo', term_gettitle(term)) })

  exe term . 'bwipe!'
endfunc

func Test_term_gettty()
  let buf = Run_shell_in_terminal({})
  let gettty = term_gettty(buf)

  if has('unix') && executable('tty')
    " Find tty using the tty shell command.
    call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})
    call term_sendkeys(buf, "tty\r")
    call WaitForAssert({-> assert_notequal('', term_getline(buf, 3))})
    let tty = term_getline(buf, 2)
    call assert_equal(tty, gettty)
  endif

  let gettty0 = term_gettty(buf, 0)
  let gettty1 = term_gettty(buf, 1)

  call assert_equal(gettty, gettty0)
  call assert_equal(job_info(g:job).tty_out, gettty0)
  call assert_equal(job_info(g:job).tty_in,  gettty1)

  if has('unix')
    " For unix, term_gettty(..., 0) and term_gettty(..., 1)
    " are identical according to :help term_gettty()
    call assert_equal(gettty0, gettty1)
    call assert_match('^/dev/', gettty)
  else
    " ConPTY works on anonymous pipe.
    if !has('conpty')
      call assert_match('^\\\\.\\pipe\\', gettty0)
      call assert_match('^\\\\.\\pipe\\', gettty1)
    endif
  endif

  call assert_fails('call term_gettty(buf, 2)', 'E475:')
  call assert_fails('call term_gettty(buf, -1)', 'E475:')

  call assert_equal('', term_gettty(buf + 1))

  call StopShellInTerminal(buf)
  exe buf . 'bwipe'
endfunc


" vim: shiftwidth=2 sts=2 expandtab
