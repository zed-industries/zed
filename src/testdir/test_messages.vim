" Tests for :messages, :echomsg, :echoerr

source check.vim
source shared.vim
source term_util.vim
source view_util.vim
source screendump.vim

func Test_messages()
  let oldmore = &more
  try
    set nomore

    let arr = map(range(10), '"hello" . v:val')
    for s in arr
      echomsg s | redraw
    endfor

    " get last two messages
    redir => result
    2messages | redraw
    redir END
    let msg_list = split(result, "\n")
    call assert_equal(["hello8", "hello9"], msg_list)

    " clear messages without last one
    1messages clear
    let msg_list = GetMessages()
    call assert_equal(['hello9'], msg_list)

    " clear all messages
    messages clear
    let msg_list = GetMessages()
    call assert_equal([], msg_list)
  finally
    let &more = oldmore
  endtry

  call assert_fails('message 1', 'E474:')
endfunc

" Patch 7.4.1696 defined the "clearmode()" function for clearing the mode
" indicator (e.g., "-- INSERT --") when ":stopinsert" is invoked.  Message
" output could then be disturbed when 'cmdheight' was greater than one.
" This test ensures that the bugfix for this issue remains in place.
func Test_stopinsert_does_not_break_message_output()
  set cmdheight=2
  redraw!

  stopinsert | echo 'test echo'
  call assert_equal(116, screenchar(&lines - 1, 1))
  call assert_equal(32, screenchar(&lines, 1))
  redraw!

  stopinsert | echomsg 'test echomsg'
  call assert_equal(116, screenchar(&lines - 1, 1))
  call assert_equal(32, screenchar(&lines, 1))
  redraw!

  set cmdheight&
endfunc

func Test_message_completion()
  call feedkeys(":message \<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"message clear', @:)
endfunc

func Test_echomsg()
  call assert_equal("\nhello", execute(':echomsg "hello"'))
  call assert_equal("\n", execute(':echomsg ""'))
  call assert_equal("\n12345", execute(':echomsg 12345'))
  call assert_equal("\n[]", execute(':echomsg []'))
  call assert_equal("\n[1, 2, 3]", execute(':echomsg [1, 2, 3]'))
  call assert_equal("\n[1, 2, []]", execute(':echomsg [1, 2, test_null_list()]'))
  call assert_equal("\n{}", execute(':echomsg {}'))
  call assert_equal("\n{'a': 1, 'b': 2}", execute(':echomsg {"a": 1, "b": 2}'))
  call assert_equal("\n1.23", execute(':echomsg 1.23'))
  call assert_match("function('<lambda>\\d*')", execute(':echomsg {-> 1234}'))
endfunc

func Test_echoerr()
  call test_ignore_error('IgNoRe')
  call assert_equal("\nIgNoRe hello", execute(':echoerr "IgNoRe hello"'))
  call assert_equal("\n12345 IgNoRe", execute(':echoerr 12345 "IgNoRe"'))
  call assert_equal("\n[1, 2, 'IgNoRe']", execute(':echoerr [1, 2, "IgNoRe"]'))
  call assert_equal("\n{'IgNoRe': 2, 'a': 1}", execute(':echoerr {"a": 1, "IgNoRe": 2}'))
  call assert_equal("\n1.23 IgNoRe", execute(':echoerr 1.23 "IgNoRe"'))
  eval '<lambda>'->test_ignore_error()
  call assert_match("function('<lambda>\\d*')", execute(':echoerr {-> 1234}'))
  call test_ignore_error('RESET')
endfunc

func Test_mode_message_at_leaving_insert_by_ctrl_c()
  CheckFeature terminal
  CheckNotGui

  " Set custom statusline built by user-defined function.
  let testfile = 'Xtest.vim'
  let lines =<< trim END
        func StatusLine() abort
          return ""
        endfunc
        set statusline=%!StatusLine()
        set laststatus=2
  END
  call writefile(lines, testfile, 'D')

  let rows = 10
  let buf = term_start([GetVimProg(), '--clean', '-S', testfile], {'term_rows': rows})
  call TermWait(buf, 100)
  call assert_equal('run', job_status(term_getjob(buf)))

  call term_sendkeys(buf, "i")
  call WaitForAssert({-> assert_match('^-- INSERT --\s*$', term_getline(buf, rows))})
  call term_sendkeys(buf, "\<C-C>")
  call WaitForAssert({-> assert_match('^\s*$', term_getline(buf, rows))})

  call term_sendkeys(buf, ":qall!\<CR>")
  call WaitForAssert({-> assert_equal('dead', job_status(term_getjob(buf)))})

  exe buf . 'bwipe!'
endfunc

func Test_mode_message_at_leaving_insert_with_esc_mapped()
  CheckFeature terminal
  CheckNotGui

  " Set custom statusline built by user-defined function.
  let testfile = 'Xtest.vim'
  let lines =<< trim END
        set laststatus=2
        inoremap <Esc> <Esc>00
  END
  call writefile(lines, testfile, 'D')

  let rows = 10
  let buf = term_start([GetVimProg(), '--clean', '-S', testfile], {'term_rows': rows})
  call WaitForAssert({-> assert_match('0,0-1\s*All$', term_getline(buf, rows - 1))})
  call assert_equal('run', job_status(term_getjob(buf)))

  call term_sendkeys(buf, "i")
  call WaitForAssert({-> assert_match('^-- INSERT --\s*$', term_getline(buf, rows))})
  call term_sendkeys(buf, "\<Esc>")
  call WaitForAssert({-> assert_match('^\s*$', term_getline(buf, rows))})

  call term_sendkeys(buf, ":qall!\<CR>")
  call WaitForAssert({-> assert_equal('dead', job_status(term_getjob(buf)))})

  exe buf . 'bwipe!'
endfunc

func Test_echospace()
  set noruler noshowcmd laststatus=1
  call assert_equal(&columns - 1, v:echospace)
  split
  call assert_equal(&columns - 1, v:echospace)
  set ruler
  call assert_equal(&columns - 1, v:echospace)
  close
  call assert_equal(&columns - 19, v:echospace)
  set showcmd noruler
  call assert_equal(&columns - 12, v:echospace)
  set showcmd ruler
  call assert_equal(&columns - 29, v:echospace)

  set ruler& showcmd&
endfunc

func Test_warning_scroll()
  CheckRunVimInTerminal
  let lines =<< trim END
      call test_override('ui_delay', 50)
      set noruler
      set readonly
      undo
  END
  call writefile(lines, 'XTestWarningScroll', 'D')
  let buf = RunVimInTerminal('', #{rows: 8})

  " When the warning comes from a script, messages are scrolled so that the
  " stacktrace is visible.
  call term_sendkeys(buf, ":source XTestWarningScroll\n")
  " only match the final colon in the line that shows the source
  call WaitForAssert({-> assert_match(':$', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('line    4:W10: Warning: Changing a readonly file', term_getline(buf, 6))})
  call WaitForAssert({-> assert_equal('Already at oldest change', term_getline(buf, 7))})
  call WaitForAssert({-> assert_equal('Press ENTER or type command to continue', term_getline(buf, 8))})
  call term_sendkeys(buf, "\n")

  " When the warning does not come from a script, messages are not scrolled.
  call term_sendkeys(buf, ":enew\n")
  call term_sendkeys(buf, ":set readonly\n")
  call term_sendkeys(buf, 'u')
  call WaitForAssert({-> assert_equal('W10: Warning: Changing a readonly file', term_getline(buf, 8))})
  call WaitForAssert({-> assert_equal('Already at oldest change', term_getline(buf, 8))})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" Test more-prompt (see :help more-prompt).
func Test_message_more()
  CheckRunVimInTerminal
  let buf = RunVimInTerminal('', {'rows': 6})
  call term_sendkeys(buf, ":call setline(1, range(1, 100))\n")

  call term_sendkeys(buf, ":%pfoo\<C-H>\<C-H>\<C-H>#")
  call WaitForAssert({-> assert_equal(':%p#', term_getline(buf, 6))})
  call term_sendkeys(buf, "\n")
  call WaitForAssert({-> assert_equal('  5 5', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('-- More --', term_getline(buf, 6))})

  call term_sendkeys(buf, '?')
  call WaitForAssert({-> assert_equal('  5 5', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('-- More -- SPACE/d/j: screen/page/line down, b/u/k: up, q: quit ', term_getline(buf, 6))})

  " Down a line with j, <CR>, <NL> or <Down>.
  call term_sendkeys(buf, "j")
  call WaitForAssert({-> assert_equal('  6 6', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('-- More --', term_getline(buf, 6))})
  call term_sendkeys(buf, "\<NL>")
  call WaitForAssert({-> assert_equal('  7 7', term_getline(buf, 5))})
  call term_sendkeys(buf, "\<CR>")
  call WaitForAssert({-> assert_equal('  8 8', term_getline(buf, 5))})
  call term_sendkeys(buf, "\<Down>")
  call WaitForAssert({-> assert_equal('  9 9', term_getline(buf, 5))})

  " Down a screen with <Space>, f, or <PageDown>.
  call term_sendkeys(buf, 'f')
  call WaitForAssert({-> assert_equal(' 14 14', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('-- More --', term_getline(buf, 6))})
  call term_sendkeys(buf, ' ')
  call WaitForAssert({-> assert_equal(' 19 19', term_getline(buf, 5))})
  call term_sendkeys(buf, "\<PageDown>")
  call WaitForAssert({-> assert_equal(' 24 24', term_getline(buf, 5))})

  " Down a page (half a screen) with d.
  call term_sendkeys(buf, 'd')
  call WaitForAssert({-> assert_equal(' 27 27', term_getline(buf, 5))})

  " Down all the way with 'G'.
  call term_sendkeys(buf, 'G')
  call WaitForAssert({-> assert_equal('100 100', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('Press ENTER or type command to continue', term_getline(buf, 6))})

  " Up a line k, <BS> or <Up>.
  call term_sendkeys(buf, 'k')
  call WaitForAssert({-> assert_equal(' 99 99', term_getline(buf, 5))})
  call term_sendkeys(buf, "\<BS>")
  call WaitForAssert({-> assert_equal(' 98 98', term_getline(buf, 5))})
  call term_sendkeys(buf, "\<Up>")
  call WaitForAssert({-> assert_equal(' 97 97', term_getline(buf, 5))})

  " Up a screen with b or <PageUp>.
  call term_sendkeys(buf, 'b')
  call WaitForAssert({-> assert_equal(' 92 92', term_getline(buf, 5))})
  call term_sendkeys(buf, "\<PageUp>")
  call WaitForAssert({-> assert_equal(' 87 87', term_getline(buf, 5))})

  " Up a page (half a screen) with u.
  call term_sendkeys(buf, 'u')
  call WaitForAssert({-> assert_equal(' 84 84', term_getline(buf, 5))})

  " Up all the way with 'g'.
  call term_sendkeys(buf, 'g')
  call WaitForAssert({-> assert_equal('  4 4', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal(':%p#', term_getline(buf, 1))})
  call WaitForAssert({-> assert_equal('-- More --', term_getline(buf, 6))})

  " All the way down. Pressing f should do nothing but pressing
  " space should end the more prompt.
  call term_sendkeys(buf, 'G')
  call WaitForAssert({-> assert_equal('100 100', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('Press ENTER or type command to continue', term_getline(buf, 6))})
  call term_sendkeys(buf, 'f')
  call WaitForAssert({-> assert_equal('100 100', term_getline(buf, 5))})
  call term_sendkeys(buf, ' ')
  call WaitForAssert({-> assert_equal('100', term_getline(buf, 5))})

  " Pressing g< shows the previous command output.
  call term_sendkeys(buf, 'g<')
  call WaitForAssert({-> assert_equal('100 100', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('Press ENTER or type command to continue', term_getline(buf, 6))})

  " A command line that doesn't print text is appended to scrollback,
  " even if it invokes a nested command line.
  call term_sendkeys(buf, ":\<C-R>=':'\<CR>:\<CR>g<")
  call WaitForAssert({-> assert_equal('100 100', term_getline(buf, 4))})
  call WaitForAssert({-> assert_equal(':::', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('Press ENTER or type command to continue', term_getline(buf, 6))})

  call term_sendkeys(buf, ":%p#\n")
  call WaitForAssert({-> assert_equal('  5 5', term_getline(buf, 5))})
  call WaitForAssert({-> assert_equal('-- More --', term_getline(buf, 6))})

  " Stop command output with q, <Esc> or CTRL-C.
  call term_sendkeys(buf, 'q')
  call WaitForAssert({-> assert_equal('100', term_getline(buf, 5))})

  " Execute a : command from the more prompt
  call term_sendkeys(buf, ":%p#\n")
  call term_wait(buf)
  call WaitForAssert({-> assert_equal('-- More --', term_getline(buf, 6))})
  call term_sendkeys(buf, ":")
  call term_wait(buf)
  call WaitForAssert({-> assert_equal(':', term_getline(buf, 6))})
  call term_sendkeys(buf, "echo 'Hello'\n")
  call term_wait(buf)
  call WaitForAssert({-> assert_equal('Hello ', term_getline(buf, 5))})

  call StopVimInTerminal(buf)
endfunc

" Test more-prompt scrollback
func Test_message_more_scrollback()
  CheckRunVimInTerminal

  let lines =<< trim END
      set t_ut=
      hi Normal ctermfg=15 ctermbg=0
      for i in range(100)
          echo i
      endfor
  END
  call writefile(lines, 'XmoreScrollback', 'D')
  let buf = RunVimInTerminal('-S XmoreScrollback', {'rows': 10})
  call VerifyScreenDump(buf, 'Test_more_scrollback_1', {})

  call term_sendkeys(buf, 'f')
  call TermWait(buf)
  call term_sendkeys(buf, 'b')
  call VerifyScreenDump(buf, 'Test_more_scrollback_2', {})

  call term_sendkeys(buf, 'q')
  call TermWait(buf)
  call StopVimInTerminal(buf)
endfunc

func Test_message_not_cleared_after_mode()
  CheckRunVimInTerminal

  let lines =<< trim END
      nmap <silent> gx :call DebugSilent('normal')<CR>
      vmap <silent> gx :call DebugSilent('visual')<CR>
      function DebugSilent(arg)
          echomsg "from DebugSilent" a:arg
      endfunction
      set showmode
      set cmdheight=1
      call test_settime(1)
      call setline(1, ['one', 'NoSuchFile', 'three'])
  END
  call writefile(lines, 'XmessageMode', 'D')
  let buf = RunVimInTerminal('-S XmessageMode', {'rows': 10})

  call term_sendkeys(buf, 'gx')
  call TermWait(buf)
  call VerifyScreenDump(buf, 'Test_message_not_cleared_after_mode_1', {})

  " removing the mode message used to also clear the intended message
  call term_sendkeys(buf, 'vEgx')
  call TermWait(buf)
  call VerifyScreenDump(buf, 'Test_message_not_cleared_after_mode_2', {})

  " removing the mode message used to also clear the error message
  call term_sendkeys(buf, ":set cmdheight=2\<CR>")
  call term_sendkeys(buf, '2GvEgf')
  call TermWait(buf)
  call VerifyScreenDump(buf, 'Test_message_not_cleared_after_mode_3', {})

  call StopVimInTerminal(buf)
endfunc

" Test verbose message before echo command
func Test_echo_verbose_system()
  CheckRunVimInTerminal
  CheckUnix    " needs the "seq" command
  CheckNotMac  " doesn't use /tmp

  let buf = RunVimInTerminal('', {'rows': 10})
  call term_sendkeys(buf, ":4 verbose echo system('seq 20')\<CR>")
  " Note that the screendump is filtered to remove the name of the temp file
  call VerifyScreenDump(buf, 'Test_verbose_system_1', {})

  " display a page and go back, results in exactly the same view
  call term_sendkeys(buf, ' ')
  call TermWait(buf, 50)
  call term_sendkeys(buf, 'b')
  call VerifyScreenDump(buf, 'Test_verbose_system_1', {})

  " do the same with 'cmdheight' set to 2
  call term_sendkeys(buf, 'q')
  call TermWait(buf)
  call term_sendkeys(buf, ":set ch=2\<CR>")
  call TermWait(buf)
  call term_sendkeys(buf, ":4 verbose echo system('seq 20')\<CR>")
  call VerifyScreenDump(buf, 'Test_verbose_system_2', {})

  call term_sendkeys(buf, ' ')
  call TermWait(buf, 50)
  call term_sendkeys(buf, 'b')
  call VerifyScreenDump(buf, 'Test_verbose_system_2', {})

  call term_sendkeys(buf, 'q')
  call TermWait(buf)
  call StopVimInTerminal(buf)
endfunc


func Test_ask_yesno()
  CheckRunVimInTerminal
  let buf = RunVimInTerminal('', {'rows': 6})
  call term_sendkeys(buf, ":call setline(1, range(1, 2))\n")

  call term_sendkeys(buf, ":2,1s/^/n/\n")
  call WaitForAssert({-> assert_equal('Backwards range given, OK to swap (y/n)?', term_getline(buf, 6))})
  call term_sendkeys(buf, "n")
  call WaitForAssert({-> assert_match('^Backwards range given, OK to swap (y/n)?n *1,1 *All$', term_getline(buf, 6))})
  call WaitForAssert({-> assert_equal('1', term_getline(buf, 1))})

  call term_sendkeys(buf, ":2,1s/^/Esc/\n")
  call WaitForAssert({-> assert_equal('Backwards range given, OK to swap (y/n)?', term_getline(buf, 6))})
  call term_sendkeys(buf, "\<Esc>")
  call WaitForAssert({-> assert_match('^Backwards range given, OK to swap (y/n)?n *1,1 *All$', term_getline(buf, 6))})
  call WaitForAssert({-> assert_equal('1', term_getline(buf, 1))})

  call term_sendkeys(buf, ":2,1s/^/y/\n")
  call WaitForAssert({-> assert_equal('Backwards range given, OK to swap (y/n)?', term_getline(buf, 6))})
  call term_sendkeys(buf, "y")
  call WaitForAssert({-> assert_match('^Backwards range given, OK to swap (y/n)?y *2,1 *All$', term_getline(buf, 6))})
  call WaitForAssert({-> assert_equal('y1', term_getline(buf, 1))})
  call WaitForAssert({-> assert_equal('y2', term_getline(buf, 2))})

  call StopVimInTerminal(buf)
endfunc

func Test_null()
  echom test_null_list()
  echom test_null_dict()
  echom test_null_blob()
  echom test_null_string()
  echom test_null_function()
  echom test_null_partial()
  if has('job')
    echom test_null_job()
    echom test_null_channel()
  endif
endfunc

func Test_mapping_at_hit_return_prompt()
  nnoremap <C-B> :echo "hit ctrl-b"<CR>
  call feedkeys(":ls\<CR>", "xt")
  call feedkeys("\<*C-B>", "xt")
  call assert_match('hit ctrl-b', Screenline(&lines - 1))
  nunmap <C-B>
endfunc

func Test_quit_long_message()
  CheckScreendump

  let content =<< trim END
    echom range(9999)->join("\x01")
  END
  call writefile(content, 'Xtest_quit_message', 'D')
  let buf = RunVimInTerminal('-S Xtest_quit_message', #{rows: 10, wait_for_ruler: 0})
  call WaitForAssert({-> assert_match('^-- More --', term_getline(buf, 10))})
  call term_sendkeys(buf, "q")
  call VerifyScreenDump(buf, 'Test_quit_long_message', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" this was missing a terminating NUL
func Test_echo_string_partial()
  function CountSpaces()
  endfunction
  call assert_equal("function('CountSpaces', [{'ccccccccccc': ['ab', 'cd'], 'aaaaaaaaaaa': v:false, 'bbbbbbbbbbbb': ''}])", string(function('CountSpaces', [#{aaaaaaaaaaa: v:false, bbbbbbbbbbbb: '', ccccccccccc: ['ab', 'cd']}])))
endfunc

" Test that fileinfo is shown properly when 'cmdheight' has just decreased
" due to switching tabpage and 'shortmess' doesn't contain 'o' or 'O'.
func Test_fileinfo_tabpage_cmdheight()
  CheckRunVimInTerminal

  let content =<< trim END
    set shortmess-=o
    set shortmess-=O
    set shortmess-=F
    tabnew
    set cmdheight=2
    tabprev
    edit Xfileinfo.txt
  END

  call writefile(content, 'Xtest_fileinfo_tabpage_cmdheight', 'D')
  let buf = RunVimInTerminal('-S Xtest_fileinfo_tabpage_cmdheight', #{rows: 6})
  call WaitForAssert({-> assert_match('^"Xfileinfo.txt" \[New\]', term_getline(buf, 6))})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" Message output was previously overwritten by the fileinfo display, shown
" when switching buffers. If a buffer is switched to, then a message if
" echoed, we should show the message, rather than overwriting it with
" fileinfo.
func Test_fileinfo_after_echo()
  CheckScreendump

  let content =<< trim END
    file a.txt

    hide edit b.txt
    call setline(1, "hi")
    setlocal modified

    hide buffer a.txt

    autocmd CursorHold * buf b.txt | w | echo "'b' written"
  END

  call writefile(content, 'Xtest_fileinfo_after_echo', 'D')
  let buf = RunVimInTerminal('-S Xtest_fileinfo_after_echo', #{rows: 6})
  call term_sendkeys(buf, ":set updatetime=50\<CR>")
  call term_sendkeys(buf, "0$")
  call VerifyScreenDump(buf, 'Test_fileinfo_after_echo', {})

  call term_sendkeys(buf, ":q\<CR>")

  " clean up
  call StopVimInTerminal(buf)
  call delete('b.txt')
endfunc

func Test_echowindow()
  CheckScreendump

  let lines =<< trim END
      call setline(1, 'some text')
      func ShowMessage(arg)
        echowindow a:arg
      endfunc
      echowindow 'first line'
      func ManyMessages()
        for n in range(20)
          echowindow 'line' n
        endfor
      endfunc

      def TwoMessages()
        popup_clear()
        set cmdheight=2
        redraw
        timer_start(100, (_) => {
            echowin 'message'
          })
        echo 'one'
        echo 'two'
      enddef

      def ThreeMessages()
        popup_clear()
        redraw
        timer_start(100, (_) => {
            echowin 'later message'
          })
        echo 'one'
        echo 'two'
        echo 'three'
      enddef

      def HideWin()
        popup_hide(popup_findecho())
      enddef
  END
  call writefile(lines, 'XtestEchowindow', 'D')
  let buf = RunVimInTerminal('-S XtestEchowindow', #{rows: 8})
  call VerifyScreenDump(buf, 'Test_echowindow_1', {})

  call term_sendkeys(buf, ":call ShowMessage('second line')\<CR>")
  call VerifyScreenDump(buf, 'Test_echowindow_2', {})

  call term_sendkeys(buf, ":call popup_clear()\<CR>")
  call VerifyScreenDump(buf, 'Test_echowindow_3', {})

  call term_sendkeys(buf, ":call ManyMessages()\<CR>")
  call VerifyScreenDump(buf, 'Test_echowindow_4', {})

  call term_sendkeys(buf, ":call TwoMessages()\<CR>")
  call VerifyScreenDump(buf, 'Test_echowindow_5', {})

  call term_sendkeys(buf, ":call ThreeMessages()\<CR>")
  sleep 120m
  call VerifyScreenDump(buf, 'Test_echowindow_6', {})

  call term_sendkeys(buf, "\<CR>")
  call VerifyScreenDump(buf, 'Test_echowindow_7', {})

  call term_sendkeys(buf, ":tabnew\<CR>")
  call term_sendkeys(buf, ":7echowin 'more'\<CR>")
  call VerifyScreenDump(buf, 'Test_echowindow_8', {})

  call term_sendkeys(buf, ":call HideWin()\<CR>")
  call VerifyScreenDump(buf, 'Test_echowindow_9', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" messages window should not be used while evaluating the :echowin argument
func Test_echowin_eval()
  CheckScreendump

  let lines =<< trim END
      func ShowMessage()
        echo 123
        return 'test'
      endfunc
      echowindow ShowMessage()
  END
  call writefile(lines, 'XtestEchowindow', 'D')
  let buf = RunVimInTerminal('-S XtestEchowindow', #{rows: 8})
  call VerifyScreenDump(buf, 'Test_echowin_eval', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" messages window should not be used for showing the mode
func Test_echowin_showmode()
  CheckScreendump

  let lines =<< trim END
      vim9script
      setline(1, ['one', 'two'])
      timer_start(100, (_) => {
           echowin 'echo window'
         })
      normal V
  END
  call writefile(lines, 'XtestEchowinMode', 'D')
  let buf = RunVimInTerminal('-S XtestEchowinMode', #{rows: 8})
  call VerifyScreenDump(buf, 'Test_echowin_showmode', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc


" vim: shiftwidth=2 sts=2 expandtab
