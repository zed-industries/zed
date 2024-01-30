" Tests for the terminal window.
" This is split in two, because it can take a lot of time.
" See test_terminal2.vim and test_terminal3.vim for further tests.

source check.vim
CheckFeature terminal

source shared.vim
source screendump.vim
source mouse.vim
source term_util.vim

let s:python = PythonProg()
let $PROMPT_COMMAND=''

func Test_terminal_basic()
  call test_override('vterm_title', 1)
  au TerminalOpen * let b:done = 'yes'
  let buf = Run_shell_in_terminal({})

  call assert_equal('t', mode())
  call assert_equal('yes', b:done)
  call assert_match('%aR[^\n]*running]', execute('ls'))
  call assert_match('%aR[^\n]*running]', execute('ls R'))
  call assert_notmatch('%[^\n]*running]', execute('ls F'))
  call assert_notmatch('%[^\n]*running]', execute('ls ?'))
  call assert_fails('set modifiable', 'E946:')

  call StopShellInTerminal(buf)
  call assert_equal('n', mode())
  call assert_match('%aF[^\n]*finished]', execute('ls'))
  call assert_match('%aF[^\n]*finished]', execute('ls F'))
  call assert_notmatch('%[^\n]*finished]', execute('ls R'))
  call assert_notmatch('%[^\n]*finished]', execute('ls ?'))

  " closing window wipes out the terminal buffer a with finished job
  close
  call assert_equal("", bufname(buf))

  au! TerminalOpen
  call test_override('ALL', 0)
  unlet g:job
endfunc

func Test_terminal_no_name()
  let buf = Run_shell_in_terminal({})
  call assert_match('^!', bufname(buf))
  0file
  call assert_equal("", bufname(buf))
  call assert_match('\[No Name\]', execute('file'))
  call StopShellInTerminal(buf)
endfunc

func Test_terminal_TerminalWinOpen()
  au TerminalWinOpen * let b:done = 'yes'
  let buf = Run_shell_in_terminal({})
  call assert_equal('yes', b:done)
  call StopShellInTerminal(buf)
  " closing window wipes out the terminal buffer with the finished job
  close

  if has("unix")
    terminal ++hidden ++open sleep 1
    sleep 1
    call assert_fails("echo b:done", 'E121:')
  endif

  au! TerminalWinOpen
endfunc

func Test_terminal_make_change()
  let buf = Run_shell_in_terminal({})
  call StopShellInTerminal(buf)

  setlocal modifiable
  exe "normal Axxx\<Esc>"
  call assert_fails(buf . 'bwipe', 'E89:')
  undo

  exe buf . 'bwipe'
  unlet g:job
endfunc

func Test_terminal_paste_register()
  let @" = "text to paste"

  let buf = Run_shell_in_terminal({})
  " Wait for the shell to display a prompt
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})

  call feedkeys("echo \<C-W>\"\" \<C-W>\"=37 + 5\<CR>\<CR>", 'xt')
  call WaitForAssert({-> assert_match("echo text to paste 42$", getline(1))})
  call WaitForAssert({-> assert_equal('text to paste 42',       2->getline())})

  exe buf . 'bwipe!'
  unlet g:job
endfunc

func Test_terminal_unload_buffer()
  let buf = Run_shell_in_terminal({})
  call assert_fails(buf . 'bunload', 'E948:')
  exe buf . 'bunload!'
  call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
  call assert_equal("", bufname(buf))

  unlet g:job
endfunc

func Test_terminal_wipe_buffer()
  let buf = Run_shell_in_terminal({})
  call assert_fails(buf . 'bwipe', 'E948:')
  exe buf . 'bwipe!'
  call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
  call assert_equal("", bufname(buf))

  unlet g:job
endfunc

" Test that using ':confirm bwipe' on terminal works
func Test_terminal_confirm_wipe_buffer()
  CheckUnix
  CheckNotGui
  CheckFeature dialog_con
  let buf = Run_shell_in_terminal({})
  call assert_fails(buf . 'bwipe', 'E948:')
  call feedkeys('n', 'L')
  call assert_fails('confirm ' .. buf .. 'bwipe', 'E517:')
  call assert_equal(buf, bufnr())
  call assert_equal(1, &modified)
  call feedkeys('y', 'L')
  exe 'confirm ' .. buf .. 'bwipe'
  call assert_notequal(buf, bufnr())
  call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
  call assert_equal("", bufname(buf))

  unlet g:job
endfunc

" Test that using :b! will hide the terminal
func Test_terminal_goto_buffer()
  let buf_mod = bufnr()
  let buf_term = Run_shell_in_terminal({})
  call assert_equal(buf_term, bufnr())
  call assert_fails(buf_mod . 'b', 'E948:')
  exe buf_mod . 'b!'
  call assert_equal(buf_mod, bufnr())
  call assert_equal('run', job_status(g:job))
  call assert_notequal('', bufname(buf_term))
  exec buf_mod .. 'bwipe!'
  exec buf_term .. 'bwipe!'

  unlet g:job
endfunc

" Test that using ':confirm :b' will kill terminal
func Test_terminal_confirm_goto_buffer()
  CheckUnix
  CheckNotGui
  CheckFeature dialog_con
  let buf_mod = bufnr()
  let buf_term = Run_shell_in_terminal({})
  call feedkeys('n', 'L')
  exe 'confirm ' .. buf_mod .. 'b'
  call assert_equal(buf_term, bufnr())
  call feedkeys('y', 'L')
  exec 'confirm ' .. buf_mod .. 'b'
  call assert_equal(buf_mod, bufnr())
  call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
  call assert_equal("", bufname(buf_term))
  exec buf_mod .. 'bwipe!'

  unlet g:job
endfunc

" Test that using :close! will hide the terminal
func Test_terminal_close_win()
  let buf = Run_shell_in_terminal({})
  call assert_equal(buf, bufnr())
  call assert_fails('close', 'E948:')
  close!
  call assert_notequal(buf, bufnr())
  call assert_equal('run', job_status(g:job))
  call assert_notequal('', bufname(buf))
  exec buf .. 'bwipe!'

  unlet g:job
endfunc

" Test that using ':confirm close' will kill terminal
func Test_terminal_confirm_close_win()
  CheckUnix
  CheckNotGui
  CheckFeature dialog_con
  let buf = Run_shell_in_terminal({})
  call feedkeys('n', 'L')
  confirm close
  call assert_equal(buf, bufnr())
  call feedkeys('y', 'L')
  confirm close
  call assert_notequal(buf, bufnr())
  call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
  call assert_equal("", bufname(buf))

  unlet g:job
endfunc

" Test that using :quit! will kill the terminal
func Test_terminal_quit()
  let buf = Run_shell_in_terminal({})
  call assert_equal(buf, bufnr())
  call assert_fails('quit', 'E948:')
  quit!
  call assert_notequal(buf, bufnr())
  call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
  call assert_equal("", bufname(buf))

  unlet g:job
endfunc

" Test that using ':confirm quit' will kill terminal
func Test_terminal_confirm_quit()
  CheckUnix
  CheckNotGui
  CheckFeature dialog_con
  let buf = Run_shell_in_terminal({})
  call feedkeys('n', 'L')
  confirm quit
  call assert_equal(buf, bufnr())
  call feedkeys('y', 'L')
  confirm quit
  call assert_notequal(buf, bufnr())
  call WaitForAssert({-> assert_equal('dead', job_status(g:job))})

  unlet g:job
endfunc

" Test :q or :next

func Test_terminal_split_quit()
  let buf = Run_shell_in_terminal({})
  split
  quit!
  call TermWait(buf)
  sleep 50m
  call assert_equal('run', job_status(g:job))

  quit!
  call WaitForAssert({-> assert_equal('dead', job_status(g:job))})

  call assert_equal("", bufname(buf))
  unlet g:job
endfunc

func Test_terminal_hide_buffer_job_running()
  let buf = Run_shell_in_terminal({})
  setlocal bufhidden=hide
  quit
  for nr in range(1, winnr('$'))
    call assert_notequal(winbufnr(nr), buf)
  endfor
  call assert_true(bufloaded(buf))
  call assert_true(buflisted(buf))

  exe 'split ' . buf . 'buf'
  call StopShellInTerminal(buf)
  exe buf . 'bwipe'

  unlet g:job
endfunc

func Test_terminal_hide_buffer_job_finished()
  term echo hello
  let buf = bufnr()
  call WaitForAssert({-> assert_equal('finished', term_getstatus(buf))})

  call assert_true(bufloaded(buf))
  call assert_true(buflisted(buf))

  " Test :hide
  hide
  call assert_true(bufloaded(buf))
  call assert_true(buflisted(buf))
  split
  exe buf .. 'buf'
  call assert_equal(buf, bufnr())

  " Test bufhidden, which exercises a different code path
  setlocal bufhidden=hide
  edit Xasdfasdf
  call assert_true(bufloaded(buf))
  call assert_true(buflisted(buf))
  exe buf .. 'buf'
  call assert_equal(buf, bufnr())
  setlocal bufhidden=

  edit Xasdfasdf
  call assert_false(bufloaded(buf))
  call assert_false(buflisted(buf))
  bwipe Xasdfasdf
endfunc

func Test_terminal_rename_buffer()
  let cmd = Get_cat_123_cmd()
  let buf = term_start(cmd, {'term_name': 'foo'})
  call WaitForAssert({-> assert_equal('finished', term_getstatus(buf))})
  call assert_equal('foo', bufname())
  call assert_match('foo.*finished', execute('ls'))
  file bar
  call assert_equal('bar', bufname())
  call assert_match('bar.*finished', execute('ls'))
  exe 'bwipe! ' .. buf
  call delete('Xtext')
endfunc

func s:Nasty_exit_cb(job, st)
  exe g:buf . 'bwipe!'
  let g:buf = 0
endfunc

func Get_cat_123_cmd()
  if has('win32')
    if !has('conpty')
      return 'cmd /c "cls && color 2 && echo 123"'
    else
      " When clearing twice, extra sequence is not output.
      return 'cmd /c "cls && cls && color 2 && echo 123"'
    endif
  else
    call writefile(["\<Esc>[32m123"], 'Xtext')
    return "cat Xtext"
  endif
endfunc

func Test_terminal_nasty_cb()
  let cmd = Get_cat_123_cmd()
  let g:buf = term_start(cmd, {'exit_cb': function('s:Nasty_exit_cb')})
  let g:job = term_getjob(g:buf)

  call WaitForAssert({-> assert_equal("dead", job_status(g:job))})
  call WaitForAssert({-> assert_equal(0, g:buf)})
  unlet g:job
  unlet g:buf
  call delete('Xtext')
endfunc

func Check_123(buf)
  let l = term_scrape(a:buf, 0)
  call assert_true(len(l) == 0)
  let l = term_scrape(a:buf, 999)
  call assert_true(len(l) == 0)
  let l = a:buf->term_scrape(1)
  call assert_true(len(l) > 0)
  call assert_equal('1', l[0].chars)
  call assert_equal('2', l[1].chars)
  call assert_equal('3', l[2].chars)
  call assert_equal('#00e000', l[0].fg)
  call assert_equal(0, term_getattr(l[0].attr, 'bold'))
  call assert_equal(0, l[0].attr->term_getattr('italic'))
  if has('win32')
    " On Windows 'background' always defaults to dark, even though the terminal
    " may use a light background.  Therefore accept both white and black.
    call assert_match('#ffffff\|#000000', l[0].bg)
  else
    if &background == 'light'
      call assert_equal('#ffffff', l[0].bg)
    else
      call assert_equal('#000000', l[0].bg)
    endif
  endif

  let l = term_getline(a:buf, -1)
  call assert_equal('', l)
  let l = term_getline(a:buf, 0)
  call assert_equal('', l)
  let l = term_getline(a:buf, 999)
  call assert_equal('', l)
  let l = term_getline(a:buf, 1)
  call assert_equal('123', l)
endfunc

func Test_terminal_scrape_123()
  let cmd = Get_cat_123_cmd()
  let buf = term_start(cmd)

  let termlist = term_list()
  call assert_equal(1, len(termlist))
  call assert_equal(buf, termlist[0])

  " Nothing happens with invalid buffer number
  call term_wait(1234)

  call TermWait(buf)
  " On MS-Windows we first get a startup message of two lines, wait for the
  " "cls" to happen, after that we have one line with three characters.
  call WaitForAssert({-> assert_equal(3, len(term_scrape(buf, 1)))})
  call Check_123(buf)

  " Must still work after the job ended.
  let job = term_getjob(buf)
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  call TermWait(buf)
  call Check_123(buf)

  exe buf . 'bwipe'
  call delete('Xtext')
endfunc

func Test_terminal_scrape_multibyte()
  call writefile(["léttまrs"], 'Xtext', 'D')
  if has('win32')
    " Run cmd with UTF-8 codepage to make the type command print the expected
    " multibyte characters.
    let buf = term_start("cmd /K chcp 65001")
    call term_sendkeys(buf, "type Xtext\<CR>")
    eval buf->term_sendkeys("exit\<CR>")
    let line = 4
  else
    let buf = term_start("cat Xtext")
    let line = 1
  endif

  call WaitFor({-> len(term_scrape(buf, line)) >= 7 && term_scrape(buf, line)[0].chars == "l"})
  let l = term_scrape(buf, line)
  call assert_true(len(l) >= 7)
  call assert_equal('l', l[0].chars)
  call assert_equal('é', l[1].chars)
  call assert_equal(1, l[1].width)
  call assert_equal('t', l[2].chars)
  call assert_equal('t', l[3].chars)
  call assert_equal('ま', l[4].chars)
  call assert_equal(2, l[4].width)
  call assert_equal('r', l[5].chars)
  call assert_equal('s', l[6].chars)

  let job = term_getjob(buf)
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  call TermWait(buf)

  exe buf . 'bwipe'
endfunc

func Test_terminal_one_column()
  " This creates a terminal, displays a double-wide character and makes the
  " window one column wide.  This used to cause a crash.
  let width = &columns
  botright vert term
  let buf = bufnr('$')
  call TermWait(buf, 100)
  exe "set columns=" .. (width / 2)
  redraw
  call term_sendkeys(buf, "キ")
  call TermWait(buf, 10)
  exe "set columns=" .. width
  exe buf . 'bwipe!'
endfunc

func Test_terminal_scroll()
  call writefile(range(1, 200), 'Xtext', 'D')
  if has('win32')
    let cmd = 'cmd /c "type Xtext"'
  else
    let cmd = "cat Xtext"
  endif
  let buf = term_start(cmd)

  let job = term_getjob(buf)
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  call TermWait(buf)

  " wait until the scrolling stops
  while 1
    let scrolled = buf->term_getscrolled()
    sleep 20m
    if scrolled == buf->term_getscrolled()
      break
    endif
  endwhile

  call assert_equal('1', getline(1))
  call assert_equal('1', term_getline(buf, 1 - scrolled))
  call assert_equal('49', getline(49))
  call assert_equal('49', term_getline(buf, 49 - scrolled))
  call assert_equal('200', getline(200))
  call assert_equal('200', term_getline(buf, 200 - scrolled))

  exe buf . 'bwipe'
endfunc

func Test_terminal_scrollback()
  let buf = Run_shell_in_terminal({'term_rows': 15})
  set termwinscroll=100
  call writefile(range(150), 'Xtext', 'D')
  if has('win32')
    call term_sendkeys(buf, "type Xtext\<CR>")
  else
    call term_sendkeys(buf, "cat Xtext\<CR>")
  endif
  let rows = term_getsize(buf)[0]
  " On MS-Windows there is an empty line, check both last line and above it.
  call WaitForAssert({-> assert_match( '149', term_getline(buf, rows - 1) . term_getline(buf, rows - 2))})
  let lines = line('$')
  call assert_inrange(91, 100, lines)

  call StopShellInTerminal(buf)
  exe buf . 'bwipe'
  set termwinscroll&
endfunc

func Test_terminal_postponed_scrollback()
  " tail -f only works on Unix
  CheckUnix

  call writefile(range(50), 'Xtext', 'D')
  call writefile([
	\ 'set shell=/bin/sh noruler',
	\ 'terminal',
	\ 'sleep 200m',
	\ 'call feedkeys("tail -n 100 -f Xtext\<CR>", "xt")',
	\ 'sleep 100m',
	\ 'call feedkeys("\<C-W>N", "xt")',
	\ ], 'XTest_postponed', 'D')
  let buf = RunVimInTerminal('-S XTest_postponed', {})
  " Check that the Xtext lines are displayed and in Terminal-Normal mode
  call VerifyScreenDump(buf, 'Test_terminal_scrollback_1', {})

  silent !echo 'one more line' >>Xtext
  " Screen will not change, move cursor to get a different dump
  call term_sendkeys(buf, "k")
  call VerifyScreenDump(buf, 'Test_terminal_scrollback_2', {})

  " Back to Terminal-Job mode, text will scroll and show the extra line.
  call term_sendkeys(buf, "a")
  call VerifyScreenDump(buf, 'Test_terminal_scrollback_3', {})

  " stop "tail -f"
  call term_sendkeys(buf, "\<C-C>")
  call TermWait(buf, 25)
  " stop shell
  call term_sendkeys(buf, "exit\<CR>")
  call TermWait(buf, 50)
  " close terminal window
  let tsk_ret = term_sendkeys(buf, ":q\<CR>")

  " check type of term_sendkeys() return value
  echo type(tsk_ret)

  call StopVimInTerminal(buf)
endfunc

" Run diff on two dumps with different size.
func Test_terminal_dumpdiff_size()
  call assert_equal(1, winnr('$'))
  call term_dumpdiff('dumps/Test_incsearch_search_01.dump', 'dumps/Test_popup_command_01.dump')
  call assert_equal(2, winnr('$'))
  call assert_match('Test_incsearch_search_01.dump', getline(10))
  call assert_match('      +++++$', getline(11))
  call assert_match('Test_popup_command_01.dump', getline(31))
  call assert_equal(repeat('+', 75), getline(30))
  quit
endfunc

func Test_terminal_size()
  let cmd = Get_cat_123_cmd()

  exe 'terminal ++rows=5 ' . cmd
  let size = term_getsize('')
  bwipe!
  call assert_equal(5, size[0])

  call term_start(cmd, {'term_rows': 6})
  let size = term_getsize('')
  bwipe!
  call assert_equal(6, size[0])

  vsplit
  exe 'terminal ++rows=5 ++cols=33 ' . cmd
  call assert_equal([5, 33], ''->term_getsize())

  call term_setsize('', 6, 0)
  call assert_equal([6, 33], term_getsize(''))

  eval ''->term_setsize(0, 35)
  call assert_equal([6, 35], term_getsize(''))

  call term_setsize('', 7, 30)
  call assert_equal([7, 30], term_getsize(''))

  bwipe!
  call assert_fails("call term_setsize('', 7, 30)", "E955:")

  call term_start(cmd, {'term_rows': 6, 'term_cols': 36})
  let size = term_getsize('')
  bwipe!
  call assert_equal([6, 36], size)

  exe 'vertical terminal ++cols=20 ' . cmd
  let size = term_getsize('')
  bwipe!
  call assert_equal(20, size[1])

  eval cmd->term_start({'vertical': 1, 'term_cols': 26})
  let size = term_getsize('')
  bwipe!
  call assert_equal(26, size[1])

  split
  exe 'vertical terminal ++rows=6 ++cols=20 ' . cmd
  let size = term_getsize('')
  bwipe!
  call assert_equal([6, 20], size)

  call term_start(cmd, {'vertical': 1, 'term_rows': 7, 'term_cols': 27})
  let size = term_getsize('')
  bwipe!
  call assert_equal([7, 27], size)

  call assert_fails("call term_start(cmd, {'term_rows': -1})", 'E475:')
  call assert_fails("call term_start(cmd, {'term_rows': 1001})", 'E475:')
  call assert_fails("call term_start(cmd, {'term_rows': 10.0})", 'E805:')

  call assert_fails("call term_start(cmd, {'term_cols': -1})", 'E475:')
  call assert_fails("call term_start(cmd, {'term_cols': 1001})", 'E475:')
  call assert_fails("call term_start(cmd, {'term_cols': 10.0})", 'E805:')

  call delete('Xtext')
endfunc

func Test_terminal_zero_height()
  split
  wincmd j
  anoremenu 1.1 WinBar.test :
  terminal ++curwin
  wincmd k
  wincmd _
  redraw

  call term_sendkeys(bufnr(), "exit\r")
  bwipe!
endfunc

func Test_terminal_curwin()
  let cmd = Get_cat_123_cmd()
  call assert_equal(1, winnr('$'))

  split Xdummy
  call setline(1, 'dummy')
  write
  call assert_equal(1, getbufinfo('Xdummy')[0].loaded)
  exe 'terminal ++curwin ' . cmd
  call assert_equal(2, winnr('$'))
  call assert_equal(0, getbufinfo('Xdummy')[0].loaded)
  bwipe!

  split Xdummy
  call term_start(cmd, {'curwin': 1})
  call assert_equal(2, winnr('$'))
  bwipe!

  split Xdummy
  call setline(1, 'change')
  call assert_fails('terminal ++curwin ' . cmd, 'E37:')
  call assert_equal(2, winnr('$'))
  exe 'terminal! ++curwin ' . cmd
  call assert_equal(2, winnr('$'))
  bwipe!

  split Xdummy
  call setline(1, 'change')
  call assert_fails("call term_start(cmd, {'curwin': 1})", 'E37:')
  call assert_equal(2, winnr('$'))
  bwipe!

  split Xdummy
  bwipe!
  call delete('Xtext')
  call delete('Xdummy')
endfunc

func s:get_sleep_cmd()
  if s:python != ''
    let cmd = s:python . " test_short_sleep.py"
    " 500 was not enough for Travis
    let waittime = 900
  else
    echo 'This will take five seconds...'
    let waittime = 2000
    if has('win32')
      let cmd = $windir . '\system32\timeout.exe 1'
    else
      let cmd = 'sleep 1'
    endif
  endif
  return [cmd, waittime]
endfunc

func Test_terminal_finish_open_close()
  call assert_equal(1, winnr('$'))

  let [cmd, waittime] = s:get_sleep_cmd()

  " shell terminal closes automatically
  terminal
  let buf = bufnr('%')
  call assert_equal(2, winnr('$'))
  " Wait for the shell to display a prompt
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})
  call StopShellInTerminal(buf)
  call WaitForAssert({-> assert_equal(1, winnr('$'))}, waittime)

  " shell terminal that does not close automatically
  terminal ++noclose
  let buf = bufnr('%')
  call assert_equal(2, winnr('$'))
  " Wait for the shell to display a prompt
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})
  call StopShellInTerminal(buf)
  call assert_equal(2, winnr('$'))
  quit
  call assert_equal(1, winnr('$'))

  exe 'terminal ++close ' . cmd
  call assert_equal(2, winnr('$'))
  wincmd p
  call WaitForAssert({-> assert_equal(1, winnr('$'))}, waittime)

  call term_start(cmd, {'term_finish': 'close'})
  call assert_equal(2, winnr('$'))
  wincmd p
  call WaitForAssert({-> assert_equal(1, winnr('$'))}, waittime)
  call assert_equal(1, winnr('$'))

  exe 'terminal ++open ' . cmd
  close!
  call WaitForAssert({-> assert_equal(2, winnr('$'))}, waittime)
  bwipe

  call term_start(cmd, {'term_finish': 'open'})
  close!
  call WaitForAssert({-> assert_equal(2, winnr('$'))}, waittime)
  bwipe

  exe 'terminal ++hidden ++open ' . cmd
  call assert_equal(1, winnr('$'))
  call WaitForAssert({-> assert_equal(2, winnr('$'))}, waittime)
  bwipe

  call term_start(cmd, {'term_finish': 'open', 'hidden': 1})
  call assert_equal(1, winnr('$'))
  call WaitForAssert({-> assert_equal(2, winnr('$'))}, waittime)
  bwipe

  call assert_fails("call term_start(cmd, {'term_opencmd': 'open'})", 'E475:')
  call assert_fails("call term_start(cmd, {'term_opencmd': 'split %x'})", 'E475:')
  call assert_fails("call term_start(cmd, {'term_opencmd': 'split %d and %s'})", 'E475:')
  call assert_fails("call term_start(cmd, {'term_opencmd': 'split % and %d'})", 'E475:')

  call term_start(cmd, {'term_finish': 'open', 'term_opencmd': '4split | buffer %d | let g:result = "opened the buffer in a window"'})
  close!
  call WaitForAssert({-> assert_equal(2, winnr('$'))}, waittime)
  call assert_equal(4, winheight(0))
  call assert_equal('opened the buffer in a window', g:result)
  unlet g:result
  bwipe
endfunc

func Test_terminal_cwd()
  if has('win32')
    let cmd = 'cmd /c cd'
  else
    CheckExecutable pwd
    let cmd = 'pwd'
  endif
  call mkdir('Xtermdir')
  let buf = term_start(cmd, {'cwd': 'Xtermdir'})
  " if the path is very long it may be split over two lines, join them
  " together
  call WaitForAssert({-> assert_equal('Xtermdir', fnamemodify(getline(1) .. getline(2), ":t"))})

  exe buf . 'bwipe'
  call delete('Xtermdir', 'rf')
endfunc

func Test_terminal_cwd_failure()
  " Case 1: Provided directory is not actually a directory.  Attempt to make
  " the file executable as well.
  call writefile([], 'Xtcfile', 'D')
  call setfperm('Xtcfile', 'rwx------')
  call assert_fails("call term_start(&shell, {'cwd': 'Xtcfile'})", 'E475:')

  " Case 2: Directory does not exist.
  call assert_fails("call term_start(&shell, {'cwd': 'Xdir'})", 'E475:')

  " Case 3: Directory exists but is not accessible.
  " Skip this for root, it will be accessible anyway.
  if !IsRoot()
    call mkdir('XdirNoAccess', '', '0600')
    " return early if the directory permissions could not be set properly
    if getfperm('XdirNoAccess')[2] == 'x'
      call delete('XdirNoAccess', 'rf')
      return
    endif
    call assert_fails("call term_start(&shell, {'cwd': 'XdirNoAccess'})", 'E475:')
    call delete('XdirNoAccess', 'rf')
  endif
endfunc

func Test_terminal_servername()
  CheckFeature clientserver
  call s:test_environment("VIM_SERVERNAME", v:servername)
endfunc

func Test_terminal_version()
  call s:test_environment("VIM_TERMINAL", string(v:version))
endfunc

func s:test_environment(name, value)
  let buf = Run_shell_in_terminal({})
  " Wait for the shell to display a prompt
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})
  if has('win32')
    call term_sendkeys(buf, "echo %" . a:name . "%\r")
  else
    call term_sendkeys(buf, "echo $" . a:name . "\r")
  endif
  call TermWait(buf)
  call StopShellInTerminal(buf)
  call WaitForAssert({-> assert_equal(a:value, getline(2))})

  exe buf . 'bwipe'
  unlet buf
endfunc

func Test_terminal_env()
  let buf = Run_shell_in_terminal({'env': {'TESTENV': 'correct'}})
  " Wait for the shell to display a prompt
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})
  if has('win32')
    call term_sendkeys(buf, "echo %TESTENV%\r")
  else
    call term_sendkeys(buf, "echo $TESTENV\r")
  endif
  eval buf->TermWait()
  call StopShellInTerminal(buf)
  call WaitForAssert({-> assert_equal('correct', getline(2))})

  exe buf . 'bwipe'
endfunc

func Test_terminal_list_args()
  let buf = term_start([&shell, &shellcmdflag, 'echo "123"'])
  call assert_fails(buf . 'bwipe', 'E948:')
  exe buf . 'bwipe!'
  call assert_equal("", bufname(buf))
endfunction

func Test_terminal_noblock()
  let g:test_is_flaky = 1
  let buf = term_start(&shell)
  " Starting a terminal can be slow, esp. on busy CI machines.
  let wait_time = 7500
  let letters = 'abcdefghijklmnopqrstuvwxyz'
  if has('bsd') || has('mac') || has('sun')
    " The shell or something else has a problem dealing with more than 1000
    " characters at the same time.  It's very slow too.
    let len = 1000
    let wait_time = 15000
    let letters = 'abcdefghijklm'
  " NPFS is used in Windows, nonblocking mode does not work properly.
  elseif has('win32')
    let len = 1
  else
    let len = 5000
  endif

  " Send a lot of text lines, should be buffered properly.
  for c in split(letters, '\zs')
    call term_sendkeys(buf, 'echo ' . repeat(c, len) . "\<cr>")
  endfor
  call term_sendkeys(buf, "echo done\<cr>")

  " On MS-Windows there is an extra empty line below "done".  Find "done" in
  " the last-but-one or the last-but-two line.
  let lnum = term_getsize(buf)[0] - 1
  call WaitForAssert({-> assert_match('done', term_getline(buf, lnum - 1) .. '//' .. term_getline(buf, lnum))}, wait_time)
  let line = term_getline(buf, lnum)
  if line !~ 'done'
    let line = term_getline(buf, lnum - 1)
  endif
  call assert_match('done', line)

  let g:job = term_getjob(buf)
  call StopShellInTerminal(buf)
  unlet g:job
  bwipe
endfunc

func Test_terminal_write_stdin()
  " TODO: enable once writing to stdin works on MS-Windows
  CheckNotMSWindows
  CheckExecutable wc
  let g:test_is_flaky = 1

  call setline(1, ['one', 'two', 'three'])
  %term wc
  call WaitForAssert({-> assert_match('3', getline("$"))})
  let nrs = split(getline('$'))
  call assert_equal(['3', '3', '14'], nrs)
  %bwipe!

  call setline(1, ['one', 'two', 'three', 'four'])
  2,3term wc
  call WaitForAssert({-> assert_match('2', getline("$"))})
  let nrs = split(getline('$'))
  call assert_equal(['2', '2', '10'], nrs)
  %bwipe!
endfunc

func Test_terminal_eof_arg()
  call CheckPython(s:python)
  let g:test_is_flaky = 1

  call setline(1, ['print("hello")'])
  exe '1term ++eof=exit(123) ' .. s:python
  " MS-Windows echoes the input, Unix doesn't.
  if has('win32')
    call WaitFor({-> getline('$') =~ 'exit(123)'})
    call assert_equal('hello', getline(line('$') - 1))
  else
    call WaitFor({-> getline('$') =~ 'hello'})
    call assert_equal('hello', getline('$'))
  endif
  call assert_equal(123, bufnr()->term_getjob()->job_info().exitval)
  %bwipe!
endfunc

func Test_terminal_eof_arg_win32_ctrl_z()
  CheckMSWindows
  call CheckPython(s:python)
  let g:test_is_flaky = 1

  call setline(1, ['print("hello")'])
  exe '1term ++eof=<C-Z> ' .. s:python
  call WaitForAssert({-> assert_match('\^Z', getline(line('$') - 1))})
  call assert_match('\^Z', getline(line('$') - 1))
  %bwipe!
endfunc

func Test_terminal_duplicate_eof_arg()
  call CheckPython(s:python)
  let g:test_is_flaky = 1

  " Check the last specified ++eof arg is used and does not leak memory.
  new
  call setline(1, ['print("hello")'])
  exe '1term ++eof=<C-Z> ++eof=exit(123) ' .. s:python
  " MS-Windows echoes the input, Unix doesn't.
  if has('win32')
    call WaitFor({-> getline('$') =~ 'exit(123)'})
    call assert_equal('hello', getline(line('$') - 1))
  else
    call WaitFor({-> getline('$') =~ 'hello'})
    call assert_equal('hello', getline('$'))
  endif
  call assert_equal(123, bufnr()->term_getjob()->job_info().exitval)
  %bwipe!
endfunc

func Test_terminal_no_cmd()
  let g:test_is_flaky = 1
  let buf = term_start('NONE', {})
  call assert_notequal(0, buf)

  let pty = job_info(term_getjob(buf))['tty_out']
  call assert_notequal('', pty)
  if has('gui_running') && !has('win32')
    " In the GUI job_start() doesn't work, it does not read from the pty.
    call system('echo "look here" > ' . pty)
  else
    " Otherwise using a job works on all systems.
    call job_start([&shell, &shellcmdflag, 'echo "look here" > ' . pty])
  endif
  call WaitForAssert({-> assert_match('look here', term_getline(buf, 1))})

  bwipe!
endfunc

func Test_terminal_special_chars()
  " this file name only works on Unix
  CheckUnix

  call mkdir('Xdir with spaces', 'R')
  call writefile(['x'], 'Xdir with spaces/quoted"file')
  term ls Xdir\ with\ spaces/quoted\"file
  call WaitForAssert({-> assert_match('quoted"file', term_getline('', 1))})
  " make sure the job has finished
  call WaitForAssert({-> assert_match('finish', term_getstatus(bufnr()))})

  bwipe
endfunc

func Test_terminal_wrong_options()
  call assert_fails('call term_start(&shell, {
	\ "in_io": "file",
	\ "in_name": "xxx",
	\ "out_io": "file",
	\ "out_name": "xxx",
	\ "err_io": "file",
	\ "err_name": "xxx"
	\ })', 'E474:')
  call assert_fails('call term_start(&shell, {
	\ "out_buf": bufnr("%")
	\ })', 'E474:')
  call assert_fails('call term_start(&shell, {
	\ "err_buf": bufnr("%")
	\ })', 'E474:')
endfunc

func Test_terminal_redir_file()
  let g:test_is_flaky = 1
  let cmd = Get_cat_123_cmd()
  let buf = term_start(cmd, {'out_io': 'file', 'out_name': 'Xtrfile'})
  call TermWait(buf)
  " ConPTY may precede escape sequence. There are things that are not so.
  if !has('conpty')
    call WaitForAssert({-> assert_notequal(0, len(readfile("Xtrfile")))})
    call assert_match('123', readfile('Xtrfile')[0])
  endif
  let g:job = term_getjob(buf)
  call WaitForAssert({-> assert_equal("dead", job_status(g:job))})

  if has('win32')
    " On Windows we cannot delete a file being used by a process.  When
    " job_status() returns "dead", the process remains for a short time.
    " Just wait for a moment.
    sleep 50m
  endif
  call delete('Xtrfile')
  bwipe

  if has('unix')
    call writefile(['one line'], 'Xtrfile', 'D')
    let buf = term_start('cat', {'in_io': 'file', 'in_name': 'Xtrfile'})
    call TermWait(buf)
    call WaitForAssert({-> assert_equal('one line', term_getline(buf, 1))})
    let g:job = term_getjob(buf)
    call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
    bwipe
  endif

  call delete('Xtext')
endfunc

func TerminalTmap(remap)
  let buf = Run_shell_in_terminal({})
  " Wait for the shell to display a prompt
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})
  call assert_equal('t', mode())

  if a:remap
    tmap 123 456
  else
    tnoremap 123 456
  endif
  " don't use abcde, it's an existing command
  tmap 456 abxde
  call assert_equal('456', maparg('123', 't'))
  call assert_equal('abxde', maparg('456', 't'))
  call feedkeys("123", 'tx')
  call WaitForAssert({-> assert_match('abxde\|456', term_getline(buf, term_getcursor(buf)[0]))})
  let lnum = term_getcursor(buf)[0]
  if a:remap
    call assert_match('abxde', term_getline(buf, lnum))
  else
    call assert_match('456', term_getline(buf, lnum))
  endif

  call term_sendkeys(buf, "\r")
  call StopShellInTerminal(buf)

  tunmap 123
  tunmap 456
  call assert_equal('', maparg('123', 't'))
  exe buf . 'bwipe'
  unlet g:job
endfunc

func Test_terminal_tmap()
  call TerminalTmap(1)
  call TerminalTmap(0)
endfunc

func Test_terminal_wall()
  let buf = Run_shell_in_terminal({})
  wall
  call StopShellInTerminal(buf)
  exe buf . 'bwipe'
  unlet g:job
endfunc

func Test_terminal_wqall()
  let buf = Run_shell_in_terminal({})
  call assert_fails('wqall', 'E948:')
  call StopShellInTerminal(buf)
  exe buf . 'bwipe'
  unlet g:job
endfunc

func Test_terminal_composing_unicode()
  let g:test_is_flaky = 1
  let save_enc = &encoding
  set encoding=utf-8

  if has('win32')
    let cmd = "cmd /K chcp 65001"
    let lnum = [3, 6, 9]
  else
    let cmd = &shell
    let lnum = [1, 3, 5]
  endif

  enew
  let buf = term_start(cmd, {'curwin': 1})
  let g:job = term_getjob(buf)
  call WaitFor({-> term_getline(buf, 1) !=# ''}, 1000)

  if has('win32')
    call assert_equal('cmd', job_info(g:job).cmd[0])
  else
    call assert_equal(&shell, job_info(g:job).cmd[0])
  endif

  " ascii + composing
  let txt = "a\u0308bc"
  call term_sendkeys(buf, "echo " . txt)
  call TermWait(buf, 25)
  call assert_match("echo " . txt, term_getline(buf, lnum[0]))
  call term_sendkeys(buf, "\<cr>")
  call WaitForAssert({-> assert_equal(txt, term_getline(buf, lnum[0] + 1))}, 1000)
  let l = term_scrape(buf, lnum[0] + 1)
  call assert_equal("a\u0308", l[0].chars)
  call assert_equal("b", l[1].chars)
  call assert_equal("c", l[2].chars)

  " multibyte + composing: がぎぐげご
  let txt = "\u304b\u3099\u304e\u304f\u3099\u3052\u3053\u3099"
  call term_sendkeys(buf, "echo " . txt)
  call TermWait(buf, 25)
  call assert_match("echo " . txt, term_getline(buf, lnum[1]))
  call term_sendkeys(buf, "\<cr>")
  call WaitForAssert({-> assert_equal(txt, term_getline(buf, lnum[1] + 1))}, 1000)
  let l = term_scrape(buf, lnum[1] + 1)
  call assert_equal("\u304b\u3099", l[0].chars)
  call assert_equal(2, l[0].width)
  call assert_equal("\u304e", l[1].chars)
  call assert_equal(2, l[1].width)
  call assert_equal("\u304f\u3099", l[2].chars)
  call assert_equal(2, l[2].width)
  call assert_equal("\u3052", l[3].chars)
  call assert_equal(2, l[3].width)
  call assert_equal("\u3053\u3099", l[4].chars)
  call assert_equal(2, l[4].width)

  " \u00a0 + composing
  let txt = "abc\u00a0\u0308"
  call term_sendkeys(buf, "echo " . txt)
  call TermWait(buf, 25)
  call assert_match("echo " . txt, term_getline(buf, lnum[2]))
  call term_sendkeys(buf, "\<cr>")
  call WaitForAssert({-> assert_equal(txt, term_getline(buf, lnum[2] + 1))}, 1000)
  let l = term_scrape(buf, lnum[2] + 1)
  call assert_equal("\u00a0\u0308", l[3].chars)

  call term_sendkeys(buf, "exit\r")
  call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
  bwipe!
  unlet g:job
  let &encoding = save_enc
endfunc

func Test_terminal_aucmd_on_close()
  fun Nop()
    let s:called = 1
  endfun

  aug repro
      au!
      au BufWinLeave * call Nop()
  aug END

  let [cmd, waittime] = s:get_sleep_cmd()

  call assert_equal(1, winnr('$'))
  new
  call setline(1, ['one', 'two'])
  exe 'term ++close ' . cmd
  wincmd p
  call WaitForAssert({-> assert_equal(2, winnr('$'))}, waittime)
  call assert_equal(1, s:called)
  bwipe!

  unlet s:called
  au! repro
  delfunc Nop
endfunc

func Test_terminal_term_start_empty_command()
  let cmd = "call term_start('', {'curwin' : 1, 'term_finish' : 'close'})"
  call assert_fails(cmd, 'E474:')
  let cmd = "call term_start('', {'curwin' : 1, 'term_finish' : 'close'})"
  call assert_fails(cmd, 'E474:')
  let cmd = "call term_start({}, {'curwin' : 1, 'term_finish' : 'close'})"
  call assert_fails(cmd, 'E474:')
  let cmd = "call term_start(0, {'curwin' : 1, 'term_finish' : 'close'})"
  call assert_fails(cmd, 'E474:')
  let cmd = "call term_start('', {'term_name' : []})"
  call assert_fails(cmd, 'E730:')
  let cmd = "call term_start('', {'term_finish' : 'axby'})"
  call assert_fails(cmd, 'E475:')
  let cmd = "call term_start('', {'eof_chars' : []})"
  call assert_fails(cmd, 'E730:')
  let cmd = "call term_start('', {'term_kill' : []})"
  call assert_fails(cmd, 'E730:')
  let cmd = "call term_start('', {'tty_type' : []})"
  call assert_fails(cmd, 'E730:')
  let cmd = "call term_start('', {'tty_type' : 'abc'})"
  call assert_fails(cmd, 'E475:')
  let cmd = "call term_start('', {'term_highlight' : []})"
  call assert_fails(cmd, 'E730:')
  if has('gui') || has('termguicolors')
    let cmd = "call term_start('', {'ansi_colors' : 'abc'})"
    call assert_fails(cmd, 'E475:')
    let cmd = "call term_start('', {'ansi_colors' : [[]]})"
    call assert_fails(cmd, 'E730:')
    let cmd = "call term_start('', {'ansi_colors' : repeat(['blue'], 18)})"
    if has('gui_running') || has('termguicolors')
      call assert_fails(cmd, 'E475:')
    else
      call assert_fails(cmd, 'E254:')
    endif
  endif
endfunc

func Test_terminal_response_to_control_sequence()
  CheckUnix

  let buf = Run_shell_in_terminal({})
  call WaitForAssert({-> assert_notequal('', term_getline(buf, 1))})

  call term_sendkeys(buf, "cat\<CR>")
  call WaitForAssert({-> assert_match('cat', term_getline(buf, 1))})

  " Request the cursor position.
  call term_sendkeys(buf, "\x1b[6n\<CR>")

  " Wait for output from tty to display, below an empty line.
  call WaitForAssert({-> assert_match('3;1R', term_getline(buf, 4))})

  " End "cat" gently.
  call term_sendkeys(buf, "\<CR>\<C-D>")

  call StopShellInTerminal(buf)
  exe buf . 'bwipe'
  unlet g:job
endfunc

" Run this first, it fails when run after other tests.
func Test_aa_terminal_focus_events()
  CheckNotGui
  CheckUnix
  CheckRunVimInTerminal

  let save_term = &term
  let save_ttymouse = &ttymouse
  set term=xterm ttymouse=xterm2

  let lines =<< trim END
      set term=xterm ttymouse=xterm2
      au FocusLost * call setline(1, 'I am lost') | set nomod
      au FocusGained * call setline(1, 'I am back') | set nomod
  END
  call writefile(lines, 'XtermFocus', 'D')
  let buf = RunVimInTerminal('-S XtermFocus', #{rows: 6})

  " Send a focus event to ourselves, it should be forwarded to the terminal
  call feedkeys("\<Esc>[O", "Lx!")
  call VerifyScreenDump(buf, 'Test_terminal_focus_1', {})

  call feedkeys("\<Esc>[I", "Lx!")
  call VerifyScreenDump(buf, 'Test_terminal_focus_2', {})

  " check that a command line being edited is redrawn in place
  call term_sendkeys(buf, ":" .. repeat('x', 80))
  call TermWait(buf)
  call feedkeys("\<Esc>[O", "Lx!")
  call VerifyScreenDump(buf, 'Test_terminal_focus_3', {})
  call term_sendkeys(buf, "\<Esc>")

  call StopVimInTerminal(buf)
  let &term = save_term
  let &ttymouse = save_ttymouse
endfunc

" Run Vim, start a terminal in that Vim with the kill argument,
" :qall works.
func Run_terminal_qall_kill(line1, line2)
  " 1. Open a terminal window and wait for the prompt to appear
  " 2. set kill using term_setkill()
  " 3. make Vim exit, it will kill the shell
  let after = [
	\ a:line1,
	\ 'let buf = bufnr("%")',
	\ 'while term_getline(buf, 1) =~ "^\\s*$"',
	\ '  sleep 10m',
	\ 'endwhile',
	\ a:line2,
	\ 'au VimLeavePre * call writefile(["done"], "Xdone")',
	\ 'qall',
	\ ]
  if !RunVim([], after, '')
    return
  endif
  call assert_equal("done", readfile("Xdone")[0])
  call delete("Xdone")
endfunc

" Run Vim in a terminal, then start a terminal in that Vim with a kill
" argument, check that :qall works.
func Test_terminal_qall_kill_arg()
  call Run_terminal_qall_kill('term ++kill=kill', '')
endfunc

" Run Vim, start a terminal in that Vim, set the kill argument with
" term_setkill(), check that :qall works.
func Test_terminal_qall_kill_func()
  call Run_terminal_qall_kill('term', 'eval buf->term_setkill("kill")')
endfunc

" Run Vim, start a terminal in that Vim without the kill argument,
" check that :qall does not exit, :qall! does.
func Test_terminal_qall_exit()
  let after =<< trim [CODE]
    term
    let buf = bufnr("%")
    while term_getline(buf, 1) =~ "^\\s*$"
      sleep 10m
    endwhile
    set nomore
    au VimLeavePre * call writefile(["too early"], "Xdone")
    qall
    au! VimLeavePre * exe buf . "bwipe!" | call writefile(["done"], "Xdone")
    cquit
  [CODE]

  if !RunVim([], after, '')
    return
  endif
  call assert_equal("done", readfile("Xdone")[0])
  call delete("Xdone")
endfunc

" Run Vim in a terminal, then start a terminal in that Vim without a kill
" argument, check that :confirm qall works.
func Test_terminal_qall_prompt()
  CheckRunVimInTerminal

  let buf = RunVimInTerminal('', {})

  " the shell may set the window title, we don't want that here
  call term_sendkeys(buf, ":call test_override('vterm_title', 1)\<CR>")

  " Open a terminal window and wait for the prompt to appear
  call term_sendkeys(buf, ":term\<CR>")
  call WaitForAssert({-> assert_match('\[running]', term_getline(buf, 10))})
  call WaitForAssert({-> assert_notmatch('^\s*$', term_getline(buf, 1))})

  " make Vim exit, it will prompt to kill the shell
  call term_sendkeys(buf, "\<C-W>:confirm qall\<CR>")
  call WaitForAssert({-> assert_match('\[Y\]es, (N)o:', term_getline(buf, 20))})
  call term_sendkeys(buf, "y")
  call WaitForAssert({-> assert_equal('finished', term_getstatus(buf))})

  " close the terminal window where Vim was running
  quit
endfunc

" Run Vim in a terminal, then start a terminal window with a shell and check
" that Vim exits if it is closed.
func Test_terminal_exit()
  CheckRunVimInTerminal

  let lines =<< trim END
     let winid = win_getid()
     help
     term
     let termid = win_getid()
     call win_gotoid(winid)
     close
     call win_gotoid(termid)
  END
  call writefile(lines, 'XtermExit', 'D')
  let buf = RunVimInTerminal('-S XtermExit', #{rows: 10})
  let job = term_getjob(buf)
  call WaitForAssert({-> assert_equal("run", job_status(job))})

  " quit the shell, it will make Vim exit
  call term_sendkeys(buf, "exit\<CR>")
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
endfunc

func Test_terminal_open_autocmd()
  augroup repro
    au!
    au TerminalOpen * let s:called += 1
  augroup END

  let s:called = 0

  " Open a terminal window with :terminal
  terminal
  call assert_equal(1, s:called)
  bwipe!

  " Open a terminal window with term_start()
  call term_start(&shell)
  call assert_equal(2, s:called)
  bwipe!

  " Open a hidden terminal buffer with :terminal
  terminal ++hidden
  call assert_equal(3, s:called)
  for buf in term_list()
    exe buf . "bwipe!"
  endfor

  " Open a hidden terminal buffer with term_start()
  let buf = term_start(&shell, {'hidden': 1})
  call assert_equal(4, s:called)
  exe buf . "bwipe!"

  unlet s:called
  au! repro
endfunc

func Test_open_term_from_cmd()
  CheckUnix
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ['a', 'b', 'c'])
      3
      set incsearch
      cnoremap <F3> <Cmd>call term_start(['/bin/sh', '-c', ':'])<CR>
  END
  call writefile(lines, 'Xopenterm', 'D')
  let buf = RunVimInTerminal('-S Xopenterm', {})

  " this opens a window, incsearch should not use the old cursor position
  call term_sendkeys(buf, "/\<F3>")
  call VerifyScreenDump(buf, 'Test_terminal_from_cmd', {})
  call term_sendkeys(buf, "\<Esc>")
  call term_sendkeys(buf, ":q\<CR>")

  call StopVimInTerminal(buf)
endfunc

func Test_combining_double_width()
  CheckUnix
  CheckRunVimInTerminal

  call writefile(["\xe3\x83\x9b\xe3\x82\x9a"], 'Xonedouble', 'D')
  let lines =<< trim END
      call term_start(['/bin/sh', '-c', 'cat Xonedouble'])
  END
  call writefile(lines, 'Xcombining', 'D')
  let buf = RunVimInTerminal('-S Xcombining', #{rows: 9})

  " this opens a window, incsearch should not use the old cursor position
  call VerifyScreenDump(buf, 'Test_terminal_combining', {})
  call term_sendkeys(buf, ":q\<CR>")

  call StopVimInTerminal(buf)
endfunc

func Test_terminal_popup_with_cmd()
  " this was crashing
  let buf = term_start(&shell, #{hidden: v:true})
  let s:winid = popup_create(buf, {})
  tnoremap <F3> <Cmd>call popup_close(s:winid)<CR>
  call feedkeys("\<F3>", 'xt')

  tunmap  <F3>
  exe 'bwipe! ' .. buf
  unlet s:winid
endfunc

func Test_terminal_popup_bufload()
  let termbuf = term_start(&shell, #{hidden: v:true, term_finish: 'close'})
  let winid = popup_create(termbuf, {})
  sleep 50m

  let newbuf = bufadd('')
  call bufload(newbuf)
  call setbufline(newbuf, 1, 'foobar')

  " must not have switched to another window
  call assert_equal(winid, win_getid())

  call StopShellInTerminal(termbuf)
  call WaitFor({-> win_getid() != winid})
  exe 'bwipe! ' .. newbuf
endfunc

func Test_terminal_popup_two_windows()
  CheckRunVimInTerminal
  CheckUnix

  " use "sh" instead of "&shell" in the hope it will use a short prompt
  let lines =<< trim END
      let termbuf = term_start('sh', #{hidden: v:true, term_finish: 'close'})
      exe 'buffer ' .. termbuf

      let winid = popup_create(termbuf, #{line: 2, minwidth: 30, border: []})
      sleep 50m

      call term_sendkeys(termbuf, "echo 'test'")
  END
  call writefile(lines, 'XpopupScript', 'D')
  let buf = RunVimInTerminal('-S XpopupScript', {})

  " typed text appears both in normal window and in popup
  call WaitForAssert({-> assert_match("echo 'test'", term_getline(buf, 1))})
  call WaitForAssert({-> assert_match("echo 'test'", term_getline(buf, 3))})

  call term_sendkeys(buf, "\<CR>\<CR>exit\<CR>")
  call TermWait(buf)
  call term_sendkeys(buf, ":q\<CR>")
  call StopVimInTerminal(buf)
endfunc

func Test_terminal_popup_insert_cmd()
  CheckUnix

  inoremap <F3> <Cmd>call StartTermInPopup()<CR>
  func StartTermInPopup()
    call term_start(['/bin/sh', '-c', 'cat'], #{hidden: v:true, term_finish: 'close'})->popup_create(#{highlight: 'Pmenu'})
  endfunc
  call feedkeys("i\<F3>")
  sleep 10m
  call assert_equal('n', mode())

  call feedkeys("\<C-D>", 'xt')
  call WaitFor({-> popup_list() == []})
  delfunc StartTermInPopup
  iunmap <F3>
endfunc

func Check_dump01(off)
  call assert_equal('one two three four five', trim(getline(a:off + 1)))
  call assert_equal('~           Select Word', trim(getline(a:off + 7)))
  call assert_equal(':popup PopUp', trim(getline(a:off + 20)))
endfunc

func Test_terminal_dumpwrite_composing()
  CheckRunVimInTerminal

  let save_enc = &encoding
  set encoding=utf-8
  call assert_equal(1, winnr('$'))

  let text = " a\u0300 e\u0302 o\u0308"
  call writefile([text], 'Xcomposing', 'D')
  let buf = RunVimInTerminal('--cmd "set encoding=utf-8" Xcomposing', {})
  call WaitForAssert({-> assert_match(text, term_getline(buf, 1))})
  eval 'Xdump'->term_dumpwrite(buf)
  let dumpline = readfile('Xdump')[0]
  call assert_match('|à| |ê| |ö', dumpline)

  call StopVimInTerminal(buf)
  call delete('Xdump')
  let &encoding = save_enc
endfunc

" Tests for failures in the term_dumpwrite() function
func Test_terminal_dumpwrite_errors()
  CheckRunVimInTerminal
  call assert_fails("call term_dumpwrite({}, 'Xtest.dump')", 'E728:')
  let buf = RunVimInTerminal('', {})
  call TermWait(buf)
  call assert_fails("call term_dumpwrite(buf, 'Xtest.dump', '')", 'E1206:')
  call assert_fails("call term_dumpwrite(buf, [])", 'E730:')
  call writefile([], 'Xtest.dump')
  call assert_fails("call term_dumpwrite(buf, 'Xtest.dump')", 'E953:')
  call delete('Xtest.dump')
  call assert_fails("call term_dumpwrite(buf, '')", 'E482:')
  call assert_fails("call term_dumpwrite(buf, test_null_string())", 'E482:')
  call test_garbagecollect_now()
  call StopVimInTerminal(buf, 0)
  call TermWait(buf)
  call assert_fails("call term_dumpwrite(buf, 'Xtest.dump')", 'E958:')
  call assert_fails('call term_sendkeys([], ":q\<CR>")', 'E745:')
  call assert_equal(0, term_sendkeys(buf, ":q\<CR>"))
endfunc

" just testing basic functionality.
func Test_terminal_dumpload()
  let curbuf = winbufnr('')
  call assert_equal(1, winnr('$'))
  let buf = term_dumpload('dumps/Test_popup_command_01.dump')
  call assert_equal(2, winnr('$'))
  call assert_equal(20, line('$'))
  call Check_dump01(0)

  " Load another dump in the same window
  let buf2 = 'dumps/Test_diff_01.dump'->term_dumpload({'bufnr': buf})
  call assert_equal(buf, buf2)
  call assert_notequal('one two three four five', trim(getline(1)))

  " Load the first dump again in the same window
  let buf2 = term_dumpload('dumps/Test_popup_command_01.dump', {'bufnr': buf})
  call assert_equal(buf, buf2)
  call Check_dump01(0)

  call assert_fails("call term_dumpload('dumps/Test_popup_command_01.dump', {'bufnr': curbuf})", 'E475:')
  call assert_fails("call term_dumpload('dumps/Test_popup_command_01.dump', {'bufnr': 9999})", 'E86:')
  new
  let closedbuf = winbufnr('')
  quit
  call assert_fails("call term_dumpload('dumps/Test_popup_command_01.dump', {'bufnr': closedbuf})", 'E475:')
  call assert_fails('call term_dumpload([])', 'E730:')
  call assert_fails('call term_dumpload("xabcy.dump")', 'E485:')

  quit
endfunc

func Test_terminal_dumpload_dump()
  CheckRunVimInTerminal

  let lines =<< trim END
     call term_dumpload('dumps/Test_popupwin_22.dump', #{term_rows: 12})
  END
  call writefile(lines, 'XtermDumpload', 'D')
  let buf = RunVimInTerminal('-S XtermDumpload', #{rows: 15})
  call VerifyScreenDump(buf, 'Test_terminal_dumpload', {})

  call StopVimInTerminal(buf)
endfunc

func Test_terminal_dumpdiff()
  call assert_equal(1, winnr('$'))
  eval 'dumps/Test_popup_command_01.dump'->term_dumpdiff('dumps/Test_popup_command_02.dump')
  call assert_equal(2, winnr('$'))
  call assert_equal(62, line('$'))
  call Check_dump01(0)
  call Check_dump01(42)
  call assert_equal('           bbbbbbbbbbbbbbbbbb ', getline(26)[0:29])
  quit

  call assert_fails('call term_dumpdiff("X1.dump", [])', 'E730:')
  call assert_fails('call term_dumpdiff("X1.dump", "X2.dump")', 'E485:')
  call writefile([], 'X1.dump', 'D')
  call assert_fails('call term_dumpdiff("X1.dump", "X2.dump")', 'E485:')
endfunc

func Test_terminal_dumpdiff_swap()
  call assert_equal(1, winnr('$'))
  call term_dumpdiff('dumps/Test_popup_command_01.dump', 'dumps/Test_popup_command_03.dump')
  call assert_equal(2, winnr('$'))
  call assert_equal(62, line('$'))
  call assert_match('Test_popup_command_01.dump', getline(21))
  call assert_match('Test_popup_command_03.dump', getline(42))
  call assert_match('Undo', getline(3))
  call assert_match('three four five', getline(45))

  normal s
  call assert_match('Test_popup_command_03.dump', getline(21))
  call assert_match('Test_popup_command_01.dump', getline(42))
  call assert_match('three four five', getline(3))
  call assert_match('Undo', getline(45))
  quit

  " Diff two terminal dump files with different number of rows
  " Swap the diffs
  call term_dumpdiff('dumps/Test_popup_command_01.dump', 'dumps/Test_winline_rnu.dump')
  call assert_match('Test_popup_command_01.dump', getline(21))
  call assert_match('Test_winline_rnu.dump', getline(42))
  normal s
  call assert_match('Test_winline_rnu.dump', getline(6))
  call assert_match('Test_popup_command_01.dump', getline(27))
  quit
endfunc

func Test_terminal_dumpdiff_options()
  set laststatus=0
  call assert_equal(1, winnr('$'))
  let height = winheight(0)
  call term_dumpdiff('dumps/Test_popup_command_01.dump', 'dumps/Test_popup_command_02.dump', {'vertical': 1, 'term_cols': 33})
  call assert_equal(2, winnr('$'))
  call assert_equal(height, winheight(winnr()))
  call assert_equal(33, winwidth(winnr()))
  call assert_equal('dump diff dumps/Test_popup_command_01.dump', bufname('%'))
  quit

  call assert_equal(1, winnr('$'))
  call term_dumpdiff('dumps/Test_popup_command_01.dump', 'dumps/Test_popup_command_02.dump', {'vertical': 0, 'term_rows': 13, 'term_name': 'something else'})
  call assert_equal(2, winnr('$'))
  call assert_equal(&columns, winwidth(0))
  call assert_equal(13, winheight(0))
  call assert_equal('something else', bufname('%'))
  quit

  call assert_equal(1, winnr('$'))
  call term_dumpdiff('dumps/Test_popup_command_01.dump', 'dumps/Test_popup_command_02.dump', {'curwin': 1})
  call assert_equal(1, winnr('$'))
  call assert_fails("call term_dumpdiff('dumps/Test_popup_command_01.dump', 'dumps/Test_popup_command_02.dump', {'bufnr': -1})", 'E475:')
  bwipe

  set laststatus&
endfunc

" When drawing the statusline the cursor position may not have been updated
" yet.
" 1. create a terminal, make it show 2 lines
" 2. 0.5 sec later: leave terminal window, execute "i"
" 3. 0.5 sec later: clear terminal window, now it's 1 line
" 4. 0.5 sec later: redraw, including statusline (used to trigger bug)
" 4. 0.5 sec later: should be done, clean up
func Test_terminal_statusline()
  CheckUnix
  CheckFeature timers

  set statusline=x
  terminal
  let tbuf = bufnr('')
  call term_sendkeys(tbuf, "clear; echo a; echo b; sleep 1; clear\n")
  call timer_start(500, { tid -> feedkeys("\<C-w>j", 'tx') })
  call timer_start(1500, { tid -> feedkeys("\<C-l>", 'tx') })
  au BufLeave * if &buftype == 'terminal' | silent! normal i | endif

  sleep 2
  exe tbuf . 'bwipe!'
  au! BufLeave
  set statusline=
endfunc

func CheckTerminalWindowWorks(buf)
  call WaitForAssert({-> assert_match('!sh \[running\]', term_getline(a:buf, 10))})
  call term_sendkeys(a:buf, "exit\<CR>")
  call WaitForAssert({-> assert_match('!sh \[finished\]', term_getline(a:buf, 10))})
  call term_sendkeys(a:buf, ":q\<CR>")
  call WaitForAssert({-> assert_match('^\~', term_getline(a:buf, 10))})
endfunc

func Test_start_terminal_from_timer()
  CheckUnix
  CheckFeature timers

  " Open a terminal window from a timer, typed text goes to the terminal
  call writefile(["call timer_start(100, { -> term_start('sh') })"], 'XtimerTerm', 'D')
  let buf = RunVimInTerminal('-S XtimerTerm', {})
  call CheckTerminalWindowWorks(buf)

  " do the same in Insert mode
  call term_sendkeys(buf, ":call timer_start(200, { -> term_start('sh') })\<CR>a")
  call CheckTerminalWindowWorks(buf)

  call StopVimInTerminal(buf)
endfunc

func Test_terminal_window_focus()
  let winid1 = win_getid()
  terminal
  let winid2 = win_getid()
  call feedkeys("\<C-W>j", 'xt')
  call assert_equal(winid1, win_getid())
  call feedkeys("\<C-W>k", 'xt')
  call assert_equal(winid2, win_getid())
  " can use a cursor key here
  call feedkeys("\<C-W>\<Down>", 'xt')
  call assert_equal(winid1, win_getid())
  call feedkeys("\<C-W>\<Up>", 'xt')
  call assert_equal(winid2, win_getid())

  bwipe!
endfunc

func Api_drop_common(options)
  call assert_equal(1, winnr('$'))

  " Use the title termcap entries to output the escape sequence.
  call writefile([
	\ 'set title',
	\ 'exe "set t_ts=\<Esc>]51; t_fs=\x07"',
	\ 'let &titlestring = ''["drop","Xtextfile"' . a:options . ']''',
	\ 'redraw',
	\ "set t_ts=",
	\ ], 'Xscript')
  let buf = RunVimInTerminal('-S Xscript', {})
  call WaitFor({-> bufnr('Xtextfile') > 0})
  call assert_equal('Xtextfile', expand('%:t'))
  call assert_true(winnr('$') >= 3)
  return buf
endfunc

func Test_terminal_api_drop_newwin()
  CheckRunVimInTerminal
  let buf = Api_drop_common('')
  call assert_equal(0, &bin)
  call assert_equal('', &fenc)

  call StopVimInTerminal(buf)
  call delete('Xscript')
  bwipe Xtextfile
endfunc

func Test_terminal_api_drop_newwin_bin()
  CheckRunVimInTerminal
  let buf = Api_drop_common(',{"bin":1}')
  call assert_equal(1, &bin)

  call StopVimInTerminal(buf)
  call delete('Xscript')
  bwipe Xtextfile
endfunc

func Test_terminal_api_drop_newwin_binary()
  CheckRunVimInTerminal
  let buf = Api_drop_common(',{"binary":1}')
  call assert_equal(1, &bin)

  call StopVimInTerminal(buf)
  call delete('Xscript')
  bwipe Xtextfile
endfunc

func Test_terminal_api_drop_newwin_nobin()
  CheckRunVimInTerminal
  set binary
  let buf = Api_drop_common(',{"nobin":1}')
  call assert_equal(0, &bin)

  call StopVimInTerminal(buf)
  call delete('Xscript')
  bwipe Xtextfile
  set nobinary
endfunc

func Test_terminal_api_drop_newwin_nobinary()
  CheckRunVimInTerminal
  set binary
  let buf = Api_drop_common(',{"nobinary":1}')
  call assert_equal(0, &bin)

  call StopVimInTerminal(buf)
  call delete('Xscript')
  bwipe Xtextfile
  set nobinary
endfunc

func Test_terminal_api_drop_newwin_ff()
  CheckRunVimInTerminal
  let buf = Api_drop_common(',{"ff":"dos"}')
  call assert_equal("dos", &ff)

  call StopVimInTerminal(buf)
  call delete('Xscript')
  bwipe Xtextfile
endfunc

func Test_terminal_api_drop_newwin_fileformat()
  CheckRunVimInTerminal
  let buf = Api_drop_common(',{"fileformat":"dos"}')
  call assert_equal("dos", &ff)

  call StopVimInTerminal(buf)
  call delete('Xscript')
  bwipe Xtextfile
endfunc

func Test_terminal_api_drop_newwin_enc()
  CheckRunVimInTerminal
  let buf = Api_drop_common(',{"enc":"utf-16"}')
  call assert_equal("utf-16", &fenc)

  call StopVimInTerminal(buf)
  call delete('Xscript')
  bwipe Xtextfile
endfunc

func Test_terminal_api_drop_newwin_encoding()
  CheckRunVimInTerminal
  let buf = Api_drop_common(',{"encoding":"utf-16"}')
  call assert_equal("utf-16", &fenc)

  call StopVimInTerminal(buf)
  call delete('Xscript')
  bwipe Xtextfile
endfunc

func Test_terminal_api_drop_oldwin()
  CheckRunVimInTerminal
  let firstwinid = win_getid()
  split Xtextfile
  let textfile_winid = win_getid()
  call assert_equal(2, winnr('$'))
  call win_gotoid(firstwinid)

  " Use the title termcap entries to output the escape sequence.
  call writefile([
	\ 'set title',
	\ 'exe "set t_ts=\<Esc>]51; t_fs=\x07"',
	\ 'let &titlestring = ''["drop","Xtextfile"]''',
	\ 'redraw',
	\ "set t_ts=",
	\ ], 'Xscript', 'D')
  let buf = RunVimInTerminal('-S Xscript', {'rows': 10})
  call WaitForAssert({-> assert_equal('Xtextfile', expand('%:t'))})
  call assert_equal(textfile_winid, win_getid())

  call StopVimInTerminal(buf)
  bwipe Xtextfile
endfunc

func Tapi_TryThis(bufnum, arg)
  let g:called_bufnum = a:bufnum
  let g:called_arg = a:arg
endfunc

func WriteApiCall(funcname)
  " Use the title termcap entries to output the escape sequence.
  call writefile([
	\ 'set title',
	\ 'exe "set t_ts=\<Esc>]51; t_fs=\x07"',
	\ 'let &titlestring = ''["call","' . a:funcname . '",["hello",123]]''',
	\ 'redraw',
	\ "set t_ts=",
	\ ], 'Xscript')
endfunc

func Test_terminal_api_call()
  CheckRunVimInTerminal

  unlet! g:called_bufnum
  unlet! g:called_arg

  call WriteApiCall('Tapi_TryThis')

  " Default
  let buf = RunVimInTerminal('-S Xscript', {})
  call WaitFor({-> exists('g:called_bufnum')})
  call assert_equal(buf, g:called_bufnum)
  call assert_equal(['hello', 123], g:called_arg)
  call StopVimInTerminal(buf)

  unlet! g:called_bufnum
  unlet! g:called_arg

  " Enable explicitly
  let buf = RunVimInTerminal('-S Xscript', {'term_api': 'Tapi_Try'})
  call WaitFor({-> exists('g:called_bufnum')})
  call assert_equal(buf, g:called_bufnum)
  call assert_equal(['hello', 123], g:called_arg)
  call StopVimInTerminal(buf)

  unlet! g:called_bufnum
  unlet! g:called_arg

  func! ApiCall_TryThis(bufnum, arg)
    let g:called_bufnum2 = a:bufnum
    let g:called_arg2 = a:arg
  endfunc

  call WriteApiCall('ApiCall_TryThis')

  " Use prefix match
  let buf = RunVimInTerminal('-S Xscript', {'term_api': 'ApiCall_'})
  call WaitFor({-> exists('g:called_bufnum2')})
  call assert_equal(buf, g:called_bufnum2)
  call assert_equal(['hello', 123], g:called_arg2)
  call StopVimInTerminal(buf)

  call assert_fails("call term_start('ls', {'term_api' : []})", 'E730:')

  unlet! g:called_bufnum2
  unlet! g:called_arg2

  call delete('Xscript')
  delfunction! ApiCall_TryThis
  unlet! g:called_bufnum2
  unlet! g:called_arg2
endfunc

func Test_terminal_api_call_fails()
  CheckRunVimInTerminal

  func! TryThis(bufnum, arg)
    let g:called_bufnum3 = a:bufnum
    let g:called_arg3 = a:arg
  endfunc

  call WriteApiCall('TryThis')

  unlet! g:called_bufnum3
  unlet! g:called_arg3

  " Not permitted
  call ch_logfile('Xlog', 'w')
  let buf = RunVimInTerminal('-S Xscript', {'term_api': ''})
  call WaitForAssert({-> assert_match('Unpermitted function: TryThis', string(readfile('Xlog')))})
  call assert_false(exists('g:called_bufnum3'))
  call assert_false(exists('g:called_arg3'))
  call StopVimInTerminal(buf)

  " No match
  call ch_logfile('Xlog', 'w')
  let buf = RunVimInTerminal('-S Xscript', {'term_api': 'TryThat'})
  call WaitFor({-> string(readfile('Xlog')) =~ 'Unpermitted function: TryThis'})
  call assert_false(exists('g:called_bufnum3'))
  call assert_false(exists('g:called_arg3'))
  call StopVimInTerminal(buf)

  call delete('Xscript')
  call ch_logfile('')
  call delete('Xlog')
  delfunction! TryThis
  unlet! g:called_bufnum3
  unlet! g:called_arg3
endfunc

let s:caught_e937 = 0

func Tapi_Delete(bufnum, arg)
  try
    execute 'bdelete!' a:bufnum
  catch /E937:/
    let s:caught_e937 = 1
  endtry
endfunc

func Test_terminal_api_call_fail_delete()
  CheckRunVimInTerminal

  call WriteApiCall('Tapi_Delete')
  let buf = RunVimInTerminal('-S Xscript', {})
  call WaitForAssert({-> assert_equal(1, s:caught_e937)})

  call StopVimInTerminal(buf)
  call delete('Xscript')
  call ch_logfile('', '')
endfunc

func Test_terminal_setapi_and_call()
  CheckRunVimInTerminal

  call WriteApiCall('Tapi_TryThis')
  call ch_logfile('Xlog', 'w')

  unlet! g:called_bufnum
  unlet! g:called_arg

  let buf = RunVimInTerminal('-S Xscript', {'term_api': ''})
  call WaitForAssert({-> assert_match('Unpermitted function: Tapi_TryThis', string(readfile('Xlog')))})
  call assert_false(exists('g:called_bufnum'))
  call assert_false(exists('g:called_arg'))

  eval buf->term_setapi('Tapi_')
  call term_sendkeys(buf, ":set notitle\<CR>")
  call term_sendkeys(buf, ":source Xscript\<CR>")
  call WaitFor({-> exists('g:called_bufnum')})
  call assert_equal(buf, g:called_bufnum)
  call assert_equal(['hello', 123], g:called_arg)

  call StopVimInTerminal(buf)

  call delete('Xscript')
  call ch_logfile('')
  call delete('Xlog')
  unlet! g:called_bufnum
  unlet! g:called_arg
endfunc

func Test_terminal_api_arg()
  CheckRunVimInTerminal

  call WriteApiCall('Tapi_TryThis')
  call ch_logfile('Xlog', 'w')

  unlet! g:called_bufnum
  unlet! g:called_arg

  execute 'term ++api= ' .. GetVimCommandCleanTerm() .. '-S Xscript'
  let buf = bufnr('%')
  call WaitForAssert({-> assert_match('Unpermitted function: Tapi_TryThis', string(readfile('Xlog')))})
  call assert_false(exists('g:called_bufnum'))
  call assert_false(exists('g:called_arg'))

  call StopVimInTerminal(buf)

  call ch_logfile('Xlog', 'w')

  execute 'term ++api=Tapi_ ' .. GetVimCommandCleanTerm() .. '-S Xscript'
  let buf = bufnr('%')
  call WaitFor({-> exists('g:called_bufnum')})
  call assert_equal(buf, g:called_bufnum)
  call assert_equal(['hello', 123], g:called_arg)

  call StopVimInTerminal(buf)

  call delete('Xscript')
  call ch_logfile('')
  call delete('Xlog')
  unlet! g:called_bufnum
  unlet! g:called_arg
endfunc

func Test_terminal_ansicolors_default()
  CheckFunction term_getansicolors

  let colors = [
	\ '#000000', '#e00000',
	\ '#00e000', '#e0e000',
	\ '#0000e0', '#e000e0',
	\ '#00e0e0', '#e0e0e0',
	\ '#808080', '#ff4040',
	\ '#40ff40', '#ffff40',
	\ '#4040ff', '#ff40ff',
	\ '#40ffff', '#ffffff',
	\]

  let buf = Run_shell_in_terminal({})
  call assert_equal(colors, term_getansicolors(buf))
  call StopShellInTerminal(buf)
  call assert_equal([], term_getansicolors(buf))

  exe buf . 'bwipe'
endfunc

let s:test_colors = [
	\ '#616e64', '#0d0a79',
	\ '#6d610d', '#0a7373',
	\ '#690d0a', '#6d696e',
	\ '#0d0a6f', '#616e0d',
	\ '#0a6479', '#6d0d0a',
	\ '#617373', '#0d0a69',
	\ '#6d690d', '#0a6e6f',
	\ '#610d0a', '#6e6479',
	\]

func Test_terminal_ansicolors_global()
  CheckFeature termguicolors
  CheckFunction term_getansicolors

  if has('vtp') && !has('vcon') && !has('gui_running')
    throw 'Skipped: does not support termguicolors'
  endif

  set tgc
  let g:terminal_ansi_colors = reverse(copy(s:test_colors))
  let buf = Run_shell_in_terminal({})
  call assert_equal(g:terminal_ansi_colors, term_getansicolors(buf))
  call StopShellInTerminal(buf)
  set tgc&

  exe buf . 'bwipe'
  unlet g:terminal_ansi_colors
endfunc

func Test_terminal_ansicolors_func()
  CheckFeature termguicolors
  CheckFunction term_getansicolors

  if has('vtp') && !has('vcon') && !has('gui_running')
    throw 'Skipped: does not support termguicolors'
  endif

  set tgc
  let g:terminal_ansi_colors = reverse(copy(s:test_colors))
  let buf = Run_shell_in_terminal({'ansi_colors': s:test_colors})
  call assert_equal(s:test_colors, term_getansicolors(buf))

  call term_setansicolors(buf, g:terminal_ansi_colors)
  call assert_equal(g:terminal_ansi_colors, buf->term_getansicolors())

  let colors = [
	\ 'ivory', 'AliceBlue',
	\ 'grey67', 'dark goldenrod',
	\ 'SteelBlue3', 'PaleVioletRed4',
	\ 'MediumPurple2', 'yellow2',
	\ 'RosyBrown3', 'OrangeRed2',
	\ 'white smoke', 'navy blue',
	\ 'grey47', 'gray97',
	\ 'MistyRose2', 'DodgerBlue4',
	\]
  eval buf->term_setansicolors(colors)

  let colors[4] = 'Invalid'
  call assert_fails('call term_setansicolors(buf, colors)', 'E254:')
  call assert_fails('call term_setansicolors(buf, {})', 'E1211:')
  call assert_fails('call term_setansicolors(buf, [])', 'E475: Invalid value for argument "colors"')
  set tgc&

  call StopShellInTerminal(buf)
  call assert_equal(0, term_setansicolors(buf, []))
  exe buf . 'bwipe'
endfunc

func Test_terminal_all_ansi_colors()
  CheckRunVimInTerminal

  " Use all the ANSI colors.
  call writefile([
	\ 'call setline(1, "AABBCCDDEEFFGGHHIIJJKKLLMMNNOOPP XXYYZZ")',
	\ 'hi Tblack ctermfg=0 ctermbg=8',
	\ 'hi Tdarkred ctermfg=1 ctermbg=9',
	\ 'hi Tdarkgreen ctermfg=2 ctermbg=10',
	\ 'hi Tbrown ctermfg=3 ctermbg=11',
	\ 'hi Tdarkblue ctermfg=4 ctermbg=12',
	\ 'hi Tdarkmagenta ctermfg=5 ctermbg=13',
	\ 'hi Tdarkcyan ctermfg=6 ctermbg=14',
	\ 'hi Tlightgrey ctermfg=7 ctermbg=15',
	\ 'hi Tdarkgrey ctermfg=8 ctermbg=0',
	\ 'hi Tred ctermfg=9 ctermbg=1',
	\ 'hi Tgreen ctermfg=10 ctermbg=2',
	\ 'hi Tyellow ctermfg=11 ctermbg=3',
	\ 'hi Tblue ctermfg=12 ctermbg=4',
	\ 'hi Tmagenta ctermfg=13 ctermbg=5',
	\ 'hi Tcyan ctermfg=14 ctermbg=6',
	\ 'hi Twhite ctermfg=15 ctermbg=7',
	\ 'hi TdarkredBold ctermfg=1 cterm=bold',
	\ 'hi TgreenBold ctermfg=10 cterm=bold',
	\ 'hi TmagentaBold ctermfg=13 cterm=bold ctermbg=5',
	\ '',
	\ 'call  matchadd("Tblack", "A")',
	\ 'call  matchadd("Tdarkred", "B")',
	\ 'call  matchadd("Tdarkgreen", "C")',
	\ 'call  matchadd("Tbrown", "D")',
	\ 'call  matchadd("Tdarkblue", "E")',
	\ 'call  matchadd("Tdarkmagenta", "F")',
	\ 'call  matchadd("Tdarkcyan", "G")',
	\ 'call  matchadd("Tlightgrey", "H")',
	\ 'call  matchadd("Tdarkgrey", "I")',
	\ 'call  matchadd("Tred", "J")',
	\ 'call  matchadd("Tgreen", "K")',
	\ 'call  matchadd("Tyellow", "L")',
	\ 'call  matchadd("Tblue", "M")',
	\ 'call  matchadd("Tmagenta", "N")',
	\ 'call  matchadd("Tcyan", "O")',
	\ 'call  matchadd("Twhite", "P")',
	\ 'call  matchadd("TdarkredBold", "X")',
	\ 'call  matchadd("TgreenBold", "Y")',
	\ 'call  matchadd("TmagentaBold", "Z")',
	\ 'redraw',
	\ ], 'Xcolorscript', 'D')
  let buf = RunVimInTerminal('-S Xcolorscript', {'rows': 10})
  call VerifyScreenDump(buf, 'Test_terminal_all_ansi_colors', {})

  call term_sendkeys(buf, ":q\<CR>")
  call StopVimInTerminal(buf)
endfunc

function On_BufFilePost()
    doautocmd <nomodeline> User UserEvent
endfunction

func Test_terminal_nested_autocmd()
  new
  call setline(1, range(500))
  $
  let lastline = line('.')

  augroup TermTest
    autocmd BufFilePost * call On_BufFilePost()
    autocmd User UserEvent silent
  augroup END

  let cmd = Get_cat_123_cmd()
  let buf = term_start(cmd, #{term_finish: 'close', hidden: 1})
  call assert_equal(lastline, line('.'))

  let job = term_getjob(buf)
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  call delete('Xtext')
  augroup TermTest
    au!
  augroup END
endfunc

func Test_terminal_adds_jump()
  clearjumps
  call term_start("ls", #{curwin: 1})
  call assert_equal(1, getjumplist()[0]->len())
  bwipe!
endfunc

func Close_cb(ch, ctx)
  call term_wait(a:ctx.bufnr)
  let g:close_done = 'done'
endfunc

func Test_term_wait_in_close_cb()
  let g:close_done = ''
  let ctx = {}
  let ctx.bufnr = term_start('echo "HELLO WORLD"',
        \ {'close_cb': {ch -> Close_cb(ch, ctx)}})

  call WaitForAssert({-> assert_equal("done", g:close_done)})

  unlet g:close_done
  bwipe!
endfunc

func Test_term_TextChangedT()
  augroup TermTest
    autocmd TextChangedT * ++once
          \ execute expand('<abuf>') . 'buffer' |
          \ let b:called = 1 |
          \ split |
          \ enew
  augroup END

  terminal

  let term_buf = bufnr()

  let b:called = 0

  call term_sendkeys(term_buf, "aaabbc\r")
  call TermWait(term_buf)

  call assert_equal(1, getbufvar(term_buf, 'called'))

  " Current buffer will be restored
  call assert_equal(bufnr(), term_buf)

  bwipe!
  augroup TermTest
    au!
  augroup END
endfunc

func Test_term_TextChangedT_close()
  augroup TermTest
    autocmd TextChangedT * ++once split | enew | 1close!
  augroup END

  terminal

  let term_buf = bufnr()

  call term_sendkeys(term_buf, "aaabbc\r")
  call TermWait(term_buf)

  " Current buffer will be restored
  call assert_equal(bufnr(), term_buf)

  bwipe!
  augroup TermTest
    au!
  augroup END
endfunc

" vim: shiftwidth=2 sts=2 expandtab
