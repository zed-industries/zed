" Tests for editing the command line.

source check.vim
source screendump.vim

func Test_getcmdwintype()
  call feedkeys("q/:let a = getcmdwintype()\<CR>:q\<CR>", 'x!')
  call assert_equal('/', a)

  call feedkeys("q?:let a = getcmdwintype()\<CR>:q\<CR>", 'x!')
  call assert_equal('?', a)

  call feedkeys("q::let a = getcmdwintype()\<CR>:q\<CR>", 'x!')
  call assert_equal(':', a)

  call feedkeys(":\<C-F>:let a = getcmdwintype()\<CR>:q\<CR>", 'x!')
  call assert_equal(':', a)

  call assert_equal('', getcmdwintype())
endfunc

func Test_getcmdwin_autocmd()
  let s:seq = []
  augroup CmdWin
  au WinEnter * call add(s:seq, 'WinEnter ' .. win_getid())
  au WinLeave * call add(s:seq, 'WinLeave ' .. win_getid())
  au BufEnter * call add(s:seq, 'BufEnter ' .. bufnr())
  au BufLeave * call add(s:seq, 'BufLeave ' .. bufnr())
  au CmdWinEnter * call add(s:seq, 'CmdWinEnter ' .. win_getid())
  au CmdWinLeave * call add(s:seq, 'CmdWinLeave ' .. win_getid())

  let org_winid = win_getid()
  let org_bufnr = bufnr()
  call feedkeys("q::let a = getcmdwintype()\<CR>:let s:cmd_winid = win_getid()\<CR>:let s:cmd_bufnr = bufnr()\<CR>:q\<CR>", 'x!')
  call assert_equal(':', a)
  call assert_equal([
	\ 'WinLeave ' .. org_winid,
	\ 'WinEnter ' .. s:cmd_winid,
	\ 'BufLeave ' .. org_bufnr,
	\ 'BufEnter ' .. s:cmd_bufnr,
	\ 'CmdWinEnter ' .. s:cmd_winid,
	\ 'CmdWinLeave ' .. s:cmd_winid,
	\ 'BufLeave ' .. s:cmd_bufnr,
	\ 'WinLeave ' .. s:cmd_winid,
	\ 'WinEnter ' .. org_winid,
	\ 'BufEnter ' .. org_bufnr,
	\ ], s:seq)

  au!
  augroup END
endfunc

func Test_cmdwin_bug()
  let winid = win_getid()
  sp
  try
    call feedkeys("q::call win_gotoid(" .. winid .. ")\<CR>:q\<CR>", 'x!')
  catch /^Vim\%((\a\+)\)\=:E11/
  endtry
  bw!
endfunc

func Test_cmdwin_restore()
  CheckScreendump

  let lines =<< trim [SCRIPT]
    augroup vimHints | au! | augroup END
    call setline(1, range(30))
    2split
  [SCRIPT]
  call writefile(lines, 'XTest_restore', 'D')

  let buf = RunVimInTerminal('-S XTest_restore', {'rows': 12})
  call term_sendkeys(buf, "q:")
  call VerifyScreenDump(buf, 'Test_cmdwin_restore_1', {})

  " normal restore
  call term_sendkeys(buf, ":q\<CR>")
  call VerifyScreenDump(buf, 'Test_cmdwin_restore_2', {})

  " restore after setting 'lines' with one window
  call term_sendkeys(buf, ":close\<CR>")
  call term_sendkeys(buf, "q:")
  call term_sendkeys(buf, ":set lines=18\<CR>")
  call term_sendkeys(buf, ":q\<CR>")
  call VerifyScreenDump(buf, 'Test_cmdwin_restore_3', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_cmdwin_no_terminal()
  CheckScreendump

  let buf = RunVimInTerminal('', {'rows': 12})
  call term_sendkeys(buf, ":set cmdheight=2\<CR>")
  call term_sendkeys(buf, "q:")
  call term_sendkeys(buf, ":let buf = term_start(['/bin/echo'], #{hidden: 1})\<CR>")
  call VerifyScreenDump(buf, 'Test_cmdwin_no_terminal', {})
  call term_sendkeys(buf, ":q\<CR>")
  call StopVimInTerminal(buf)
endfunc

func Test_cmdwin_wrong_command()
  CheckScreendump

  let buf = RunVimInTerminal('', {'rows': 12})
  call term_sendkeys(buf, "q:")
  call term_sendkeys(buf, "als\<Esc>")
  call term_sendkeys(buf, "\<C-W>k")
  call VerifyScreenDump(buf, 'Test_cmdwin_wrong_command_1', {})

  call term_sendkeys(buf, "\<C-C>")
  call VerifyScreenDump(buf, 'Test_cmdwin_wrong_command_2', {})

  call term_sendkeys(buf, ":q\<CR>")
  call StopVimInTerminal(buf)
endfunc

func Test_cmdwin_feedkeys()
  " This should not generate E488
  call feedkeys("q:\<CR>", 'x')
  " Using feedkeys with q: only should automatically close the cmd window
  call feedkeys('q:', 'xt')
  call assert_equal(1, winnr('$'))
  call assert_equal('', getcmdwintype())
endfunc

" Tests for the issues fixed in 7.4.441.
" When 'cedit' is set to Ctrl-C, opening the command window hangs Vim
func Test_cmdwin_cedit()
  exe "set cedit=\<C-c>"
  normal! :
  call assert_equal(1, winnr('$'))

  let g:cmd_wintype = ''
  func CmdWinType()
      let g:cmd_wintype = getcmdwintype()
      let g:wintype = win_gettype()
      return ''
  endfunc

  call feedkeys("\<C-c>a\<C-R>=CmdWinType()\<CR>\<CR>")
  echo input('')
  call assert_equal('@', g:cmd_wintype)
  call assert_equal('command', g:wintype)

  set cedit&vim
  delfunc CmdWinType
endfunc

" Test for CmdwinEnter autocmd
func Test_cmdwin_autocmd()
  augroup CmdWin
    au!
    autocmd BufLeave * if &buftype == '' | update | endif
    autocmd CmdwinEnter * startinsert
  augroup END

  call assert_fails('call feedkeys("q:xyz\<CR>", "xt")', 'E492:')
  call assert_equal('xyz', @:)

  augroup CmdWin
    au!
  augroup END
  augroup! CmdWin
endfunc

func Test_cmdwin_jump_to_win()
  call assert_fails('call feedkeys("q:\<C-W>\<C-W>\<CR>", "xt")', 'E11:')
  new
  set modified
  call assert_fails('call feedkeys("q/:qall\<CR>", "xt")', ['E37:', 'E162:'])
  close!
  call feedkeys("q/:close\<CR>", "xt")
  call assert_equal(1, winnr('$'))
  call feedkeys("q/:exit\<CR>", "xt")
  call assert_equal(1, winnr('$'))

  " opening command window twice should fail
  call assert_beeps('call feedkeys("q:q:\<CR>\<CR>", "xt")')
  call assert_equal(1, winnr('$'))
endfunc

func Test_cmdwin_tabpage()
  tabedit
  call assert_fails("silent norm q/g	:I\<Esc>", 'E11:')
  tabclose!
endfunc

func Test_cmdwin_interrupted_more_prompt()
  CheckScreendump

  " aborting the :smile output caused the cmdline window to use the current
  " buffer.
  let lines =<< trim [SCRIPT]
    au WinNew * smile
  [SCRIPT]
  call writefile(lines, 'XTest_cmdwin', 'D')

  let buf = RunVimInTerminal('-S XTest_cmdwin', {'rows': 18})
  " open cmdwin
  call term_sendkeys(buf, "q:")
  call WaitForAssert({-> assert_match('-- More --', term_getline(buf, 18))})
  " quit more prompt for :smile command
  call term_sendkeys(buf, "q")
  call WaitForAssert({-> assert_match('^$', term_getline(buf, 18))})
  " execute a simple command
  call term_sendkeys(buf, "aecho 'done'\<CR>")
  call VerifyScreenDump(buf, 'Test_cmdwin_interrupted', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" Test for recursively getting multiple command line inputs
func Test_cmdwin_multi_input()
  call feedkeys(":\<C-R>=input('P: ')\<CR>\"cyan\<CR>\<CR>", 'xt')
  call assert_equal('"cyan', @:)
endfunc

" Test for normal mode commands not supported in the cmd window
func Test_cmdwin_blocked_commands()
  call assert_fails('call feedkeys("q:\<C-T>\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-]>\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-^>\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:Q\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:Z\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<F1>\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>s\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>v\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>^\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>n\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>z\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>o\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>w\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>j\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>k\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>h\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>l\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>T\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>x\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>r\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>R\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>K\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>}\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>]\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>f\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>d\<CR>", "xt")', 'E11:')
  call assert_fails('call feedkeys("q:\<C-W>g\<CR>", "xt")', 'E11:')
endfunc

" Close the Cmd-line window in insert mode using CTRL-C
func Test_cmdwin_insert_mode_close()
  %bw!
  let s = ''
  exe "normal q:a\<C-C>let s='Hello'\<CR>"
  call assert_equal('Hello', s)
  call assert_equal(1, winnr('$'))
endfunc

func Test_cmdwin_ex_mode_with_modifier()
  " this was accessing memory after allocated text in Ex mode
  new
  call setline(1, ['some', 'text', 'lines'])
  silent! call feedkeys("gQnormal vq:atopleft\<C-V>\<CR>\<CR>", 'xt')
  bwipe!
endfunc

func s:ComplInCmdwin_GlobalCompletion(a, l, p)
  return 'global'
endfunc

func s:ComplInCmdwin_LocalCompletion(a, l, p)
  return 'local'
endfunc

func Test_compl_in_cmdwin()
  set wildmenu wildchar=<Tab>
  com! -nargs=1 -complete=command GetInput let input = <q-args>
  com! -buffer TestCommand echo 'TestCommand'
  let w:test_winvar = 'winvar'
  let b:test_bufvar = 'bufvar'

  " User-defined commands
  let input = ''
  call feedkeys("q:iGetInput T\<C-x>\<C-v>\<CR>", 'tx!')
  call assert_equal('TestCommand', input)

  let input = ''
  call feedkeys("q::GetInput T\<Tab>\<CR>:q\<CR>", 'tx!')
  call assert_equal('T', input)


  com! -nargs=1 -complete=var GetInput let input = <q-args>
  " Window-local variables
  let input = ''
  call feedkeys("q:iGetInput w:test_\<C-x>\<C-v>\<CR>", 'tx!')
  call assert_equal('w:test_winvar', input)

  let input = ''
  call feedkeys("q::GetInput w:test_\<Tab>\<CR>:q\<CR>", 'tx!')
  call assert_equal('w:test_', input)

  " Buffer-local variables
  let input = ''
  call feedkeys("q:iGetInput b:test_\<C-x>\<C-v>\<CR>", 'tx!')
  call assert_equal('b:test_bufvar', input)

  let input = ''
  call feedkeys("q::GetInput b:test_\<Tab>\<CR>:q\<CR>", 'tx!')
  call assert_equal('b:test_', input)


  " Argument completion of buffer-local command
  func s:ComplInCmdwin_GlobalCompletionList(a, l, p)
    return ['global']
  endfunc

  func s:ComplInCmdwin_LocalCompletionList(a, l, p)
    return ['local']
  endfunc

  func s:ComplInCmdwin_CheckCompletion(arg)
    call assert_equal('local', a:arg)
  endfunc

  com! -nargs=1 -complete=custom,<SID>ComplInCmdwin_GlobalCompletion
       \ TestCommand call s:ComplInCmdwin_CheckCompletion(<q-args>)
  com! -buffer -nargs=1 -complete=custom,<SID>ComplInCmdwin_LocalCompletion
       \ TestCommand call s:ComplInCmdwin_CheckCompletion(<q-args>)
  call feedkeys("q:iTestCommand \<Tab>\<CR>", 'tx!')

  com! -nargs=1 -complete=customlist,<SID>ComplInCmdwin_GlobalCompletionList
       \ TestCommand call s:ComplInCmdwin_CheckCompletion(<q-args>)
  com! -buffer -nargs=1 -complete=customlist,<SID>ComplInCmdwin_LocalCompletionList
       \ TestCommand call s:ComplInCmdwin_CheckCompletion(<q-args>)

  call feedkeys("q:iTestCommand \<Tab>\<CR>", 'tx!')

  func! s:ComplInCmdwin_CheckCompletion(arg)
    call assert_equal('global', a:arg)
  endfunc
  new
  call feedkeys("q:iTestCommand \<Tab>\<CR>", 'tx!')
  quit

  delfunc s:ComplInCmdwin_GlobalCompletion
  delfunc s:ComplInCmdwin_LocalCompletion
  delfunc s:ComplInCmdwin_GlobalCompletionList
  delfunc s:ComplInCmdwin_LocalCompletionList
  delfunc s:ComplInCmdwin_CheckCompletion

  delcom -buffer TestCommand
  delcom TestCommand
  delcom GetInput
  unlet w:test_winvar
  unlet b:test_bufvar
  set wildmenu& wildchar&
endfunc

func Test_cmdwin_cmd_completion()
  set wildmenu wildchar=<Tab>
  com! -nargs=* -complete=command SomeOne echo 'one'
  com! -nargs=* -complete=command SomeTwo echo 'two'
  call feedkeys("q:aSome\<Tab>\<Home>\"\<CR>", 'tx')
  call assert_equal('"SomeOne', @:)
  call feedkeys("q:aSome\<Tab>\<Tab>\<Home>\"\<CR>", 'tx')
  call assert_equal('"SomeTwo', @:)
  call feedkeys("q:aSome\<Tab>\<Tab>\<S-Tab>\<Home>\"\<CR>", 'tx')
  call assert_equal('"SomeOne', @:)

  delcom SomeOne
  delcom SomeTwo
  set wildmenu& wildchar&
endfunc

func Test_cmdwin_ctrl_bsl()
  " Using CTRL-\ CTRL-N in cmd window should close the window
  call feedkeys("q:\<C-\>\<C-N>", 'xt')
  call assert_equal('', getcmdwintype())
endfunc

func Test_cant_open_cmdwin_in_cmdwin()
  try
    call feedkeys("q:q::q\<CR>", "x!")
  catch
    let caught = v:exception
  endtry
  call assert_match('E1292:', caught)
endfunc

func Test_cmdwin_virtual_edit()
  enew!
  set ve=all cpo+=$
  silent normal q/s

  set ve= cpo-=$
endfunc

" Check that a :normal command can be used to stop Visual mode without side
" effects.
func Test_normal_escape()
  call feedkeys("q:i\" foo\<Esc>:normal! \<C-V>\<Esc>\<CR>:\" bar\<CR>", 'ntx')
  call assert_equal('" bar', @:)
endfunc

" This was using a pointer to a freed buffer
func Test_cmdwin_freed_buffer_ptr()
  " this does not work on MS-Windows because renaming an open file fails
  CheckNotMSWindows

  au BufEnter * next 0| file 
  edit 0
  silent! norm q/

  au! BufEnter
  bwipe!
endfunc

" This was resulting in a window with negative width.
" The test doesn't reproduce the illegal memory access though...
func Test_cmdwin_split_often()
  let lines = &lines
  let columns = &columns
  set t_WS=

  try
    set encoding=iso8859
    set ruler
    winsize 0 0
    noremap 0 H
    sil norm 0000000q:
  catch /E36:/
  endtry

  bwipe!
  set encoding=utf8
  let &lines = lines
  let &columns = columns
endfunc

func Test_cmdwin_restore_heights()
  set showtabline=0 cmdheight=2 laststatus=0
  call feedkeys("q::set cmdheight=1\<CR>:q\<CR>", 'ntx')
  call assert_equal(&lines - 1, winheight(0))

  set showtabline=2 cmdheight=3
  call feedkeys("q::set showtabline=0\<CR>:q\<CR>", 'ntx')
  call assert_equal(&lines - 3, winheight(0))

  set cmdheight=1 laststatus=2
  call feedkeys("q::set laststatus=0\<CR>:q\<CR>", 'ntx')
  call assert_equal(&lines - 1, winheight(0))

  set laststatus=2
  call feedkeys("q::set laststatus=1\<CR>:q\<CR>", 'ntx')
  call assert_equal(&lines - 1, winheight(0))

  set laststatus=2
  belowright vsplit
  wincmd _
  let restcmds = winrestcmd()
  call feedkeys("q::set laststatus=1\<CR>:q\<CR>", 'ntx')
  " As we have 2 windows, &ls = 1 should still have a statusline on the last
  " window. As such, the number of available rows hasn't changed and the window
  " sizes should be restored.
  call assert_equal(restcmds, winrestcmd())

  set cmdheight& showtabline& laststatus&
endfunc

func Test_cmdwin_temp_curwin()
  func CheckWraps(expect_wrap)
    setlocal textwidth=0 wrapmargin=1

    call deletebufline('', 1, '$')
    let as = repeat('a', winwidth(0) - 2 - &wrapmargin)
    call setline(1, as .. ' b')
    normal! gww

    setlocal textwidth& wrapmargin&
    call assert_equal(a:expect_wrap ? [as, 'b'] : [as .. ' b'], getline(1, '$'))
  endfunc

  func CheckCmdWin()
    call assert_equal('command', win_gettype())
    " textoff and &wrapmargin formatting considers the cmdwin_type char.
    call assert_equal(1, getwininfo(win_getid())[0].textoff)
    call CheckWraps(1)
  endfunc

  func CheckOtherWin()
    call assert_equal('', win_gettype())
    call assert_equal(0, getwininfo(win_getid())[0].textoff)
    call CheckWraps(0)
  endfunc

  call feedkeys("q::call CheckCmdWin()\<CR>:call win_execute(win_getid(winnr('#')), 'call CheckOtherWin()')\<CR>:q<CR>", 'ntx')

  %bwipe!
  delfunc CheckWraps
  delfunc CheckCmdWin
  delfunc CheckOtherWin
endfunc

func Test_cmdwin_interrupted()
  func CheckInterrupted()
    call feedkeys("q::call assert_equal('', getcmdwintype())\<CR>:call assert_equal('', getcmdtype())\<CR>:q<CR>", 'ntx')
  endfunc

  augroup CmdWin

  " While opening the cmdwin's split:
  " Close the cmdwin's window.
  au WinEnter * ++once quit
  call CheckInterrupted()

  " Close the old window.
  au WinEnter * ++once execute winnr('#') 'quit'
  call CheckInterrupted()

  " Switch back to the old window.
  au WinEnter * ++once wincmd p
  call CheckInterrupted()

  " Change the old window's buffer.
  au WinEnter * ++once call win_execute(win_getid(winnr('#')), 'enew')
  call CheckInterrupted()

  " Using BufLeave autocmds as cmdwin restrictions do not apply to them when
  " fired from opening the cmdwin...
  " After opening the cmdwin's split, while creating the cmdwin's buffer:
  " Delete the cmdwin's buffer.
  au BufLeave * ++once bwipe
  call CheckInterrupted()

  " Close the cmdwin's window.
  au BufLeave * ++once quit
  call CheckInterrupted()

  " Close the old window.
  au BufLeave * ++once execute winnr('#') 'quit'
  call CheckInterrupted()

  " Switch to a different window.
  au BufLeave * ++once split
  call CheckInterrupted()

  " Change the old window's buffer.
  au BufLeave * ++once call win_execute(win_getid(winnr('#')), 'enew')
  call CheckInterrupted()

  " However, changing the current buffer is OK and does not interrupt.
  au BufLeave * ++once edit other
  call feedkeys("q::let t=getcmdwintype()\<CR>:let b=bufnr()\<CR>:clo<CR>", 'ntx')
  call assert_equal(':', t)
  call assert_equal(1, bufloaded('other'))
  call assert_notequal(b, bufnr('other'))

  augroup END

  " No autocmds should remain, but clear the augroup to be sure.
  augroup CmdWin
    au!
  augroup END

  %bwipe!
  delfunc CheckInterrupted
endfunc

func Test_cmdwin_existing_bufname()
  func CheckName()
    call assert_equal(1, getbufinfo('')[0].command)
    call assert_equal(0, getbufinfo('[Command Line]')[0].command)
    call assert_match('#a\s*"\[Command Line\]"', execute('ls'))
    call assert_match('%a\s*"\[Command Line\]"', execute('ls'))
  endfunc

  file [Command Line]
  call feedkeys("q::call CheckName()\<CR>:q\<CR>", 'ntx')
  0file
  delfunc CheckName
endfunc

" vim: shiftwidth=2 sts=2 expandtab
