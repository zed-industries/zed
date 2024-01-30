" Tests for the terminal window.
" This is split in two, because it can take a lot of time.
" See test_terminal.vim and test_terminal2.vim for further tests.

source check.vim
CheckFeature terminal

source shared.vim
source screendump.vim
source mouse.vim
source term_util.vim

let $PROMPT_COMMAND=''

func Test_terminal_altscreen()
  " somehow doesn't work on MS-Windows
  CheckUnix
  let cmd = "cat Xtext\<CR>"

  let buf = term_start(&shell, {})
  call TermWait(buf)
  call writefile(["\<Esc>[?1047h"], 'Xtext', 'D')
  call term_sendkeys(buf, cmd)
  call WaitForAssert({-> assert_equal(1, term_getaltscreen(buf))})

  call writefile(["\<Esc>[?1047l"], 'Xtext')
  call term_sendkeys(buf, cmd)
  call WaitForAssert({-> assert_equal(0, term_getaltscreen(buf))})

  call term_sendkeys(buf, "exit\r")
  exe buf . "bwipe!"
endfunc

func Test_terminal_shell_option()
  if has('unix')
    " exec is a shell builtin command, should fail without a shell.
    term exec ls runtest.vim
    call WaitForAssert({-> assert_match('job failed', term_getline(bufnr(), 1))})
    bwipe!

    term ++shell exec ls runtest.vim
    call WaitForAssert({-> assert_match('runtest.vim', term_getline(bufnr(), 1))})
    bwipe!
  elseif has('win32')
    " dir is a shell builtin command, should fail without a shell.
    " However, if dir.exe (which might be provided by Cygwin/MSYS2) exists in
    " the %PATH%, "term dir" succeeds unintentionally.  Use dir.com instead.
    try
      term dir.com /b runtest.vim
      call WaitForAssert({-> assert_match('job failed', term_getline(bufnr(), 1))})
    catch /CreateProcess/
      " ignore
    endtry
    bwipe!

    " This should execute the dir builtin command even with ".com".
    term ++shell dir.com /b runtest.vim
    call WaitForAssert({-> assert_match('runtest.vim', term_getline(bufnr(), 1))})
    bwipe!
  else
    throw 'Skipped: does not work on this platform'
  endif
endfunc

func Test_terminal_invalid_arg()
  call assert_fails('terminal ++xyz', 'E181:')
endfunc

" Check a terminal with different colors
func Terminal_color(group_name, highlight_cmds, highlight_opt, open_cmds)
  CheckRunVimInTerminal
  CheckUnix

  let lines = [
	\ 'call setline(1, range(20))',
	\ 'func OpenTerm()',
	\ '  set noruler',
	\ "  call term_start('cat', #{vertical: 1, " .. a:highlight_opt .. "})",
	\ ] + a:open_cmds + [
	\ 'endfunc',
	\ ] + a:highlight_cmds
  call writefile(lines, 'XtermStart', 'D')
  let buf = RunVimInTerminal('-S XtermStart', #{rows: 15})
  call TermWait(buf, 100)
  call term_sendkeys(buf, ":call OpenTerm()\<CR>")
  call TermWait(buf, 50)
  call term_sendkeys(buf, "hello\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_color_' .. a:group_name, {})

  call term_sendkeys(buf, "\<C-D>")
  call TermWait(buf, 50)
  call StopVimInTerminal(buf)
endfunc

func Test_terminal_color_Terminal()
  call Terminal_color("Terminal", [
  \ "highlight Terminal ctermfg=blue ctermbg=yellow",
  \ ], "", [])
endfunc

func Test_terminal_color_group()
  call Terminal_color("MyTermCol", [
  \ "highlight MyTermCol ctermfg=darkgreen ctermbg=lightblue",
  \ ], "term_highlight: 'MyTermCol',", [])
endfunc

func Test_terminal_color_wincolor()
  call Terminal_color("MyWinCol", [
  \ "highlight MyWinCol ctermfg=red ctermbg=darkyellow",
  \ ], "", [
  \ 'set wincolor=MyWinCol',
  \ ])
endfunc

func Test_terminal_color_group_over_Terminal()
  call Terminal_color("MyTermCol_over_Terminal", [
  \ "highlight Terminal ctermfg=blue ctermbg=yellow",
  \ "highlight MyTermCol ctermfg=darkgreen ctermbg=lightblue",
  \ ], "term_highlight: 'MyTermCol',", [])
endfunc

func Test_terminal_color_wincolor_over_group()
  call Terminal_color("MyWinCol_over_group", [
  \ "highlight MyTermCol ctermfg=darkgreen ctermbg=lightblue",
  \ "highlight MyWinCol ctermfg=red ctermbg=darkyellow",
  \ ], "term_highlight: 'MyTermCol',", [
  \ 'set wincolor=MyWinCol',
  \ ])
endfunc

func Test_terminal_color_wincolor_split()
  CheckRunVimInTerminal
  CheckUnix

  let lines = [
	\ 'call setline(1, range(20))',
	\ 'func OpenTerm()',
	\ '  set noruler',
	\ "  call term_start('cat', #{vertical: 1, term_highlight: 'MyTermCol'})",
	\ 'endfunc',
  \ 'highlight MyTermCol ctermfg=darkgreen ctermbg=lightblue',
  \ 'highlight MyWinCol ctermfg=red ctermbg=darkyellow',
  \ 'highlight MyWinCol2 ctermfg=black ctermbg=blue',
	\ ]
  call writefile(lines, 'XtermStart', 'D')
  let buf = RunVimInTerminal('-S XtermStart', #{rows: 15})
  call TermWait(buf, 100)
  call term_sendkeys(buf, ":call OpenTerm()\<CR>")
  call TermWait(buf, 50)
  call term_sendkeys(buf, "hello\<CR>")
  call TermWait(buf, 50)

  call term_sendkeys(buf, "\<C-W>:split\<CR>")
  call term_sendkeys(buf, "\<C-W>:set wincolor=MyWinCol\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_wincolor_split_MyWinCol', {})

  call term_sendkeys(buf, "\<C-W>b:2sb\<CR>")
  call term_sendkeys(buf, "\<C-W>:set wincolor=MyWinCol2\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_wincolor_split_MyWinCol2', {})

  call term_sendkeys(buf, "\<C-D>")
  call TermWait(buf, 50)
  call StopVimInTerminal(buf)
endfunc

func Test_terminal_color_transp_Terminal()
  call Terminal_color("transp_Terminal", [
  \ "highlight Terminal ctermfg=blue",
  \ ], "", [])
endfunc

func Test_terminal_color_transp_group()
  call Terminal_color("transp_MyTermCol", [
  \ "highlight MyTermCol ctermfg=darkgreen",
  \ ], "term_highlight: 'MyTermCol',", [])
endfunc

func Test_terminal_color_transp_wincolor()
  call Terminal_color("transp_MyWinCol", [
  \ "highlight MyWinCol ctermfg=red",
  \ ], "", [
  \ 'set wincolor=MyWinCol',
  \ ])
endfunc

func Test_terminal_color_gui_Terminal()
  CheckFeature termguicolors
  call Terminal_color("gui_Terminal", [
  \ "set termguicolors",
  \ "highlight Terminal guifg=#3344ff guibg=#b0a700",
  \ ], "", [])
endfunc

func Test_terminal_color_gui_group()
  CheckFeature termguicolors
  call Terminal_color("gui_MyTermCol", [
  \ "set termguicolors",
  \ "highlight MyTermCol guifg=#007800 guibg=#6789ff",
  \ ], "term_highlight: 'MyTermCol',", [])
endfunc

func Test_terminal_color_gui_wincolor()
  CheckFeature termguicolors
  call Terminal_color("gui_MyWinCol", [
  \ "set termguicolors",
  \ "highlight MyWinCol guifg=#fe1122 guibg=#818100",
  \ ], "", [
  \ 'set wincolor=MyWinCol',
  \ ])
endfunc

func Test_terminal_color_gui_transp_Terminal()
  CheckFeature termguicolors
  call Terminal_color("gui_transp_Terminal", [
  \ "set termguicolors",
  \ "highlight Terminal guifg=#3344ff",
  \ ], "", [])
endfunc

func Test_terminal_color_gui_transp_group()
  CheckFeature termguicolors
  call Terminal_color("gui_transp_MyTermCol", [
  \ "set termguicolors",
  \ "highlight MyTermCol guifg=#007800",
  \ ], "term_highlight: 'MyTermCol',", [])
endfunc

func Test_terminal_color_gui_transp_wincolor()
  CheckFeature termguicolors
  call Terminal_color("gui_transp_MyWinCol", [
  \ "set termguicolors",
  \ "highlight MyWinCol guifg=#fe1122",
  \ ], "", [
  \ 'set wincolor=MyWinCol',
  \ ])
endfunc

func Test_terminal_in_popup()
  CheckRunVimInTerminal

  let text =<< trim END
    some text
    to edit
    in a popup window
  END
  call writefile(text, 'Xtext', 'D')
  let cmd = GetVimCommandCleanTerm()
  let lines = [
	\ 'call setline(1, range(20))',
	\ 'hi PopTerm ctermbg=grey',
	\ 'func OpenTerm(setColor)',
	\ "  set noruler",
	\ "  let s:buf = term_start('" .. cmd .. " Xtext', #{hidden: 1, term_finish: 'close'})",
	\ '  let g:winid = popup_create(s:buf, #{minwidth: 45, minheight: 7, border: [], drag: 1, resize: 1})',
	\ '  if a:setColor',
	\ '    call win_execute(g:winid, "set wincolor=PopTerm")',
	\ '  endif',
	\ 'endfunc',
	\ 'func HidePopup()',
	\ '  call popup_hide(g:winid)',
	\ 'endfunc',
	\ 'func ClosePopup()',
	\ '  call popup_close(g:winid)',
	\ 'endfunc',
	\ 'func ReopenPopup()',
	\ '  call popup_create(s:buf, #{minwidth: 40, minheight: 6, border: []})',
	\ 'endfunc',
	\ ]
  call writefile(lines, 'XtermPopup', 'D')
  let buf = RunVimInTerminal('-S XtermPopup', #{rows: 15})
  call TermWait(buf, 200)
  call term_sendkeys(buf, ":call OpenTerm(0)\<CR>")
  call TermWait(buf, 800)
  call term_sendkeys(buf, ":\<CR>")
  call TermWait(buf, 500)
  call term_sendkeys(buf, "\<C-W>:echo getwinvar(g:winid, \"&buftype\") win_gettype(g:winid)\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_popup_1', {})

  call term_sendkeys(buf, ":q\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_popup_2', {})
 
  call term_sendkeys(buf, ":call OpenTerm(1)\<CR>")
  call TermWait(buf, 800)
  call term_sendkeys(buf, ":set hlsearch\<CR>")
  call TermWait(buf, 500)
  call term_sendkeys(buf, "/edit\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_popup_3', {})
 
  call term_sendkeys(buf, "\<C-W>:call HidePopup()\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_popup_4', {})
  call term_sendkeys(buf, "\<CR>")
  call TermWait(buf, 50)

  call term_sendkeys(buf, "\<C-W>:call ClosePopup()\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_popup_5', {})

  call term_sendkeys(buf, "\<C-W>:call ReopenPopup()\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_popup_6', {})

  " Go to terminal-Normal mode and visually select text.
  call term_sendkeys(buf, "\<C-W>Ngg/in\<CR>vww")
  call VerifyScreenDump(buf, 'Test_terminal_popup_7', {})

  " Back to job mode, redraws
  call term_sendkeys(buf, "A")
  call VerifyScreenDump(buf, 'Test_terminal_popup_8', {})

  call TermWait(buf, 50)
  call term_sendkeys(buf, ":q\<CR>")
  call TermWait(buf, 250)  " wait for terminal to vanish

  call StopVimInTerminal(buf)
endfunc

" Check a terminal in popup window uses the default minimum size.
func Test_terminal_in_popup_min_size()
  CheckRunVimInTerminal

  let text =<< trim END
    another text
    to show
    in a popup window
  END
  call writefile(text, 'Xtext', 'D')
  let lines = [
	\ 'call setline(1, range(20))',
	\ 'func OpenTerm()',
	\ "  let s:buf = term_start('cat Xtext', #{hidden: 1})",
	\ '  let g:winid = popup_create(s:buf, #{ border: []})',
	\ 'endfunc',
	\ ]
  call writefile(lines, 'XtermPopup', 'D')
  let buf = RunVimInTerminal('-S XtermPopup', #{rows: 15})
  call TermWait(buf, 100)
  call term_sendkeys(buf, ":set noruler\<CR>")
  call term_sendkeys(buf, ":call OpenTerm()\<CR>")
  call TermWait(buf, 50)
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_popup_m1', {})

  call TermWait(buf, 50)
  call term_sendkeys(buf, ":q\<CR>")
  call TermWait(buf, 50)  " wait for terminal to vanish
  call StopVimInTerminal(buf)
endfunc

" Check a terminal in popup window with different colors
func Terminal_in_popup_color(group_name, highlight_cmds, highlight_opt, popup_cmds, popup_opt)
  CheckRunVimInTerminal
  CheckUnix

  let lines = [
	\ 'call setline(1, range(20))',
	\ 'func OpenTerm()',
	\ "  let s:buf = term_start('cat', #{hidden: 1, "
	\ .. a:highlight_opt .. "})",
	\ '  let g:winid = popup_create(s:buf, #{border: [], '
  \ .. a:popup_opt .. '})',
  \ ] + a:popup_cmds + [
	\ 'endfunc',
	\ ] + a:highlight_cmds
  call writefile(lines, 'XtermPopup', 'D')
  let buf = RunVimInTerminal('-S XtermPopup', #{rows: 15})
  call TermWait(buf, 100)
  call term_sendkeys(buf, ":set noruler\<CR>")
  call term_sendkeys(buf, ":call OpenTerm()\<CR>")
  call TermWait(buf, 50)
  call term_sendkeys(buf, "hello\<CR>")
  call VerifyScreenDump(buf, 'Test_terminal_popup_' .. a:group_name, {})

  call term_sendkeys(buf, "\<C-D>")
  call TermWait(buf, 50)
  call term_sendkeys(buf, ":q\<CR>")
  call TermWait(buf, 50)  " wait for terminal to vanish
  call StopVimInTerminal(buf)
endfunc

func Test_terminal_in_popup_color_Terminal()
  call Terminal_in_popup_color("Terminal", [
  \ "highlight Terminal ctermfg=blue ctermbg=yellow",
  \ ], "", [], "")
endfunc

func Test_terminal_in_popup_color_group()
  call Terminal_in_popup_color("MyTermCol", [
  \ "highlight MyTermCol ctermfg=darkgreen ctermbg=lightblue",
  \ ], "term_highlight: 'MyTermCol',", [], "")
endfunc

func Test_terminal_in_popup_color_wincolor()
  call Terminal_in_popup_color("MyWinCol", [
  \ "highlight MyWinCol ctermfg=red ctermbg=darkyellow",
  \ ], "", [
  \ 'call setwinvar(g:winid, "&wincolor", "MyWinCol")',
  \ ], "")
endfunc

func Test_terminal_in_popup_color_popup_highlight()
  call Terminal_in_popup_color("MyPopupHlCol", [
  \ "highlight MyPopupHlCol ctermfg=cyan ctermbg=green",
  \ ], "", [], "highlight: 'MyPopupHlCol'")
endfunc

func Test_terminal_in_popup_color_group_over_Terminal()
  call Terminal_in_popup_color("MyTermCol_over_Terminal", [
  \ "highlight Terminal ctermfg=blue ctermbg=yellow",
  \ "highlight MyTermCol ctermfg=darkgreen ctermbg=lightblue",
  \ ], "term_highlight: 'MyTermCol',", [], "")
endfunc

func Test_terminal_in_popup_color_wincolor_over_group()
  call Terminal_in_popup_color("MyWinCol_over_group", [
  \ "highlight MyTermCol ctermfg=darkgreen ctermbg=lightblue",
  \ "highlight MyWinCol ctermfg=red ctermbg=darkyellow",
  \ ], "term_highlight: 'MyTermCol',", [
  \ 'call setwinvar(g:winid, "&wincolor", "MyWinCol")',
  \ ], "")
endfunc

func Test_terminal_in_popup_color_transp_Terminal()
  call Terminal_in_popup_color("transp_Terminal", [
  \ "highlight Terminal ctermfg=blue",
  \ ], "", [], "")
endfunc

func Test_terminal_in_popup_color_transp_group()
  call Terminal_in_popup_color("transp_MyTermCol", [
  \ "highlight MyTermCol ctermfg=darkgreen",
  \ ], "term_highlight: 'MyTermCol',", [], "")
endfunc

func Test_terminal_in_popup_color_transp_wincolor()
  call Terminal_in_popup_color("transp_MyWinCol", [
  \ "highlight MyWinCol ctermfg=red",
  \ ], "", [
  \ 'call setwinvar(g:winid, "&wincolor", "MyWinCol")',
  \ ], "")
endfunc

func Test_terminal_in_popup_color_transp_popup_highlight()
  call Terminal_in_popup_color("transp_MyPopupHlCol", [
  \ "highlight MyPopupHlCol ctermfg=cyan",
  \ ], "", [], "highlight: 'MyPopupHlCol'")
endfunc

func Test_terminal_in_popup_color_gui_Terminal()
  CheckFeature termguicolors
  call Terminal_in_popup_color("gui_Terminal", [
  \ "set termguicolors",
  \ "highlight Terminal guifg=#3344ff guibg=#b0a700",
  \ ], "", [], "")
endfunc

func Test_terminal_in_popup_color_gui_group()
  CheckFeature termguicolors
  call Terminal_in_popup_color("gui_MyTermCol", [
  \ "set termguicolors",
  \ "highlight MyTermCol guifg=#007800 guibg=#6789ff",
  \ ], "term_highlight: 'MyTermCol',", [], "")
endfunc

func Test_terminal_in_popup_color_gui_wincolor()
  CheckFeature termguicolors
  call Terminal_in_popup_color("gui_MyWinCol", [
  \ "set termguicolors",
  \ "highlight MyWinCol guifg=#fe1122 guibg=#818100",
  \ ], "", [
  \ 'call setwinvar(g:winid, "&wincolor", "MyWinCol")',
  \ ], "")
endfunc

func Test_terminal_in_popup_color_gui_popup_highlight()
  CheckFeature termguicolors
  call Terminal_in_popup_color("gui_MyPopupHlCol", [
  \ "set termguicolors",
  \ "highlight MyPopupHlCol guifg=#00e8f0 guibg=#126521",
  \ ], "", [], "highlight: 'MyPopupHlCol'")
endfunc

func Test_terminal_in_popup_color_gui_transp_Terminal()
  CheckFeature termguicolors
  call Terminal_in_popup_color("gui_transp_Terminal", [
  \ "set termguicolors",
  \ "highlight Terminal guifg=#3344ff",
  \ ], "", [], "")
endfunc

func Test_terminal_in_popup_color_gui_transp_group()
  CheckFeature termguicolors
  call Terminal_in_popup_color("gui_transp_MyTermCol", [
  \ "set termguicolors",
  \ "highlight MyTermCol guifg=#007800",
  \ ], "term_highlight: 'MyTermCol',", [], "")
endfunc

func Test_terminal_in_popup_color_gui_transp_wincolor()
  CheckFeature termguicolors
  call Terminal_in_popup_color("gui_transp_MyWinCol", [
  \ "set termguicolors",
  \ "highlight MyWinCol guifg=#fe1122",
  \ ], "", [
  \ 'call setwinvar(g:winid, "&wincolor", "MyWinCol")',
  \ ], "")
endfunc

func Test_terminal_in_popup_color_gui_transp_popup_highlight()
  CheckFeature termguicolors
  call Terminal_in_popup_color("gui_transp_MyPopupHlCol", [
  \ "set termguicolors",
  \ "highlight MyPopupHlCol guifg=#00e8f0",
  \ ], "", [], "highlight: 'MyPopupHlCol'")
endfunc

func Test_double_popup_terminal()
  let buf1 = term_start(&shell, #{hidden: 1})
  let win1 = popup_create(buf1, {})
  let buf2 = term_start(&shell, #{hidden: 1})
  call assert_fails('call popup_create(buf2, {})', 'E861:')
  call popup_close(win1)
  exe buf1 .. 'bwipe!'
  exe buf2 .. 'bwipe!'
endfunc

func Test_escape_popup_terminal()
  set hidden

  " Cannot escape a terminal popup window using win_gotoid
  let prev_win = win_getid()
  eval term_start('sh', #{hidden: 1, term_finish: 'close'})->popup_create({})
  call assert_fails("call win_gotoid(" .. prev_win .. ")", 'E863:')

  call popup_clear(1)
  set hidden&
endfunc

func Test_issue_5607()
  let wincount = winnr('$')
  exe 'terminal' &shell &shellcmdflag 'exit'
  let job = term_getjob(bufnr())
  call WaitForAssert({-> assert_equal("dead", job_status(job))})

  let old_wincolor = &wincolor
  try
    set wincolor=
  finally
    let &wincolor = old_wincolor
    bw!
  endtry
endfunc

func Test_hidden_terminal()
  let buf = term_start(&shell, #{hidden: 1})
  call assert_equal('', bufname('^$'))
  call StopShellInTerminal(buf)
endfunc

func Test_term_nasty_callback()
  CheckExecutable sh

  set hidden
  let g:buf0 = term_start('sh', #{hidden: 1, term_finish: 'close'})
  call popup_create(g:buf0, {})
  call assert_fails("call term_start(['sh', '-c'], #{curwin: 1})", 'E863:')

  call popup_clear(1)
  set hidden&
endfunc

func Test_term_and_startinsert()
  CheckRunVimInTerminal
  CheckUnix

  let lines =<< trim EOL
     put='some text'
     term
     startinsert
  EOL
  call writefile(lines, 'XTest_startinsert', 'D')
  let buf = RunVimInTerminal('-S XTest_startinsert', {})

  call term_sendkeys(buf, "exit\r")
  call WaitForAssert({-> assert_equal("some text", term_getline(buf, 1))})
  call term_sendkeys(buf, "0l")
  call term_sendkeys(buf, "A<\<Esc>")
  call WaitForAssert({-> assert_equal("some text<", term_getline(buf, 1))})

  call StopVimInTerminal(buf)
endfunc

" Test for passing invalid arguments to terminal functions
func Test_term_func_invalid_arg()
  call assert_fails('let b = term_getaltscreen([])', 'E745:')
  call assert_fails('let a = term_getattr(1, [])', 'E730:')
  call assert_fails('let c = term_getcursor([])', 'E745:')
  call assert_fails('let l = term_getline([], 1)', 'E745:')
  call assert_fails('let l = term_getscrolled([])', 'E745:')
  call assert_fails('let s = term_getsize([])', 'E745:')
  call assert_fails('let s = term_getstatus([])', 'E745:')
  call assert_fails('let s = term_scrape([], 1)', 'E745:')
  call assert_fails('call term_sendkeys([], "a")', 'E745:')
  call assert_fails('call term_setapi([], "")', 'E745:')
  call assert_fails('call term_setrestore([], "")', 'E745:')
  call assert_fails('call term_setkill([], "")', 'E745:')
  if has('gui') || has('termguicolors')
    call assert_fails('let p = term_getansicolors([])', 'E745:')
    call assert_fails('call term_setansicolors([], [])', 'E745:')
  endif
  let buf = term_start('echo')
  call assert_fails('call term_setapi(' .. buf .. ', {})', 'E731:')
  call assert_fails('call term_setkill(' .. buf .. ', {})', 'E731:')
  call assert_fails('call term_setrestore(' .. buf .. ', {})', 'E731:')
  exe buf . "bwipe!"
endfunc

" Test for sending various special keycodes to a terminal
func Test_term_keycode_translation()
  CheckRunVimInTerminal

  let buf = RunVimInTerminal('', {})
  call term_sendkeys(buf, ":set nocompatible\<CR>")
  call term_sendkeys(buf, ":set timeoutlen=20\<CR>")

  let keys = ["\<F1>", "\<F2>", "\<F3>", "\<F4>", "\<F5>", "\<F6>", "\<F7>",
        \ "\<F8>", "\<F9>", "\<F10>", "\<F11>", "\<F12>", "\<Home>",
        \ "\<S-Home>", "\<C-Home>", "\<End>", "\<S-End>", "\<C-End>",
	\ "\<Ins>", "\<Del>", "\<Left>", "\<S-Left>", "\<C-Left>", "\<Right>",
        \ "\<S-Right>", "\<C-Right>", "\<Up>", "\<S-Up>", "\<Down>",
        \ "\<S-Down>"]
  let output = ['<F1>', '<F2>', '<F3>', '<F4>', '<F5>', '<F6>', '<F7>',
        \ '<F8>', '<F9>', '<F10>', '<F11>', '<F12>', '<Home>', '<S-Home>',
        \ '<C-Home>', '<End>', '<S-End>', '<C-End>', '<Insert>', '<Del>',
        \ '<Left>', '<S-Left>', '<C-Left>', '<Right>', '<S-Right>',
        \ '<C-Right>', '<Up>', '<S-Up>', '<Down>', '<S-Down>']

  call term_sendkeys(buf, "i")
  for i in range(len(keys))
    call term_sendkeys(buf, "\<C-U>\<C-K>" .. keys[i])
    call WaitForAssert({-> assert_equal(output[i], term_getline(buf, 1))}, 200)
  endfor

  let keypad_keys = ["\<k0>", "\<k1>", "\<k2>", "\<k3>", "\<k4>", "\<k5>",
        \ "\<k6>", "\<k7>", "\<k8>", "\<k9>", "\<kPoint>", "\<kPlus>",
        \ "\<kMinus>", "\<kMultiply>", "\<kDivide>"]
  let keypad_output = ['0', '1', '2', '3', '4', '5',
        \ '6', '7', '8', '9', '.', '+',
        \ '-', '*', '/']
  for i in range(len(keypad_keys))
    " TODO: Mysteriously keypad 3 and 9 do not work on some systems.
    if keypad_output[i] == '3' || keypad_output[i] == '9'
      continue
    endif
    call term_sendkeys(buf, "\<C-U>" .. keypad_keys[i])
    call WaitForAssert({-> assert_equal(keypad_output[i], term_getline(buf, 1))}, 100)
  endfor

  call feedkeys("\<C-U>\<kEnter>\<BS>one\<C-W>.two", 'xt')
  call WaitForAssert({-> assert_equal('two', term_getline(buf, 1))})

  call StopVimInTerminal(buf)
endfunc

" Test for using the mouse in a terminal
func Test_term_mouse()
  CheckNotGui
  CheckRunVimInTerminal

  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  let save_clipboard = &clipboard
  set mouse=a term=xterm ttymouse=sgr mousetime=200 clipboard=

  let lines =<< trim END
    one two three four five
    red green yellow red blue
    vim emacs sublime nano
  END
  call writefile(lines, 'Xtest_mouse', 'D')

  " Create a terminal window running Vim for the test with mouse enabled
  let prev_win = win_getid()
  let buf = RunVimInTerminal('Xtest_mouse -n', {})
  call term_sendkeys(buf, ":set nocompatible\<CR>")
  call term_sendkeys(buf, ":set mouse=a term=xterm ttymouse=sgr\<CR>")
  call term_sendkeys(buf, ":set clipboard=\<CR>")
  call term_sendkeys(buf, ":set mousemodel=extend\<CR>")
  call TermWait(buf)
  redraw!

  " Funcref used in WaitFor() to check that the "Xbuf" file is readable and
  " has some contents.  This avoids a "List index out of range" error when the
  " file hasn't been written yet.
  let XbufNotEmpty = {-> filereadable('Xbuf') && len(readfile('Xbuf')) > 0}

  " Use the mouse to enter the terminal window
  call win_gotoid(prev_win)
  call feedkeys(MouseLeftClickCode(1, 1), 'x')
  call feedkeys(MouseLeftReleaseCode(1, 1), 'x')
  call assert_equal(1, getwininfo(win_getid())[0].terminal)

  " Test for <LeftMouse> click/release
  call test_setmouse(2, 5)
  call feedkeys("\<LeftMouse>\<LeftRelease>", 'xt')
  call test_setmouse(3, 8)
  call term_sendkeys(buf, "\<LeftMouse>\<LeftRelease>")
  call TermWait(buf, 50)
  call delete('Xbuf')
  call term_sendkeys(buf, ":call writefile([json_encode(getpos('.'))], 'Xbuf')\<CR>")
  call TermWait(buf, 50)
  call WaitFor(XbufNotEmpty)
  let pos = json_decode(readfile('Xbuf')[0])
  call assert_equal([3, 8], pos[1:2])
  call delete('Xbuf')

  " Test for selecting text using mouse
  call test_setmouse(2, 11)
  call term_sendkeys(buf, "\<LeftMouse>")
  call test_setmouse(2, 16)
  call term_sendkeys(buf, "\<LeftRelease>y")
  call TermWait(buf, 50)
  call term_sendkeys(buf, ":call writefile([@\"], 'Xbuf')\<CR>")
  call WaitFor(XbufNotEmpty)
  call WaitForAssert({-> assert_equal('yellow', readfile('Xbuf')[0])})
  call delete('Xbuf')

  " Test for selecting text using double click
  call test_setmouse(1, 11)
  call term_sendkeys(buf, "\<LeftMouse>\<LeftRelease>\<LeftMouse>")
  call test_setmouse(1, 17)
  call term_sendkeys(buf, "\<LeftRelease>y")
  call TermWait(buf, 50)
  call term_sendkeys(buf, ":call writefile([@\"], 'Xbuf')\<CR>")
  call WaitFor(XbufNotEmpty)
  call assert_equal('three four', readfile('Xbuf')[0])
  call delete('Xbuf')

  " Test for selecting a line using triple click
  call test_setmouse(3, 2)
  call term_sendkeys(buf, "\<LeftMouse>\<LeftRelease>\<LeftMouse>\<LeftRelease>\<LeftMouse>\<LeftRelease>y")
  call TermWait(buf, 50)
  call term_sendkeys(buf, ":call writefile([@\"], 'Xbuf')\<CR>")
  call WaitFor(XbufNotEmpty)
  call assert_equal("vim emacs sublime nano\n", readfile('Xbuf')[0])
  call delete('Xbuf')

  " Test for selecting a block using quadruple click
  call test_setmouse(1, 11)
  call term_sendkeys(buf, "\<LeftMouse>\<LeftRelease>\<LeftMouse>\<LeftRelease>\<LeftMouse>\<LeftRelease>\<LeftMouse>")
  call test_setmouse(3, 13)
  call term_sendkeys(buf, "\<LeftRelease>y")
  call TermWait(buf, 50)
  call term_sendkeys(buf, ":call writefile([@\"], 'Xbuf')\<CR>")
  call WaitFor(XbufNotEmpty)
  call assert_equal("ree\nyel\nsub", readfile('Xbuf')[0])
  call delete('Xbuf')

  " Test for extending a selection using right click
  call test_setmouse(2, 9)
  call term_sendkeys(buf, "\<LeftMouse>\<LeftRelease>")
  call test_setmouse(2, 16)
  call term_sendkeys(buf, "\<RightMouse>\<RightRelease>y")
  call TermWait(buf, 50)
  call term_sendkeys(buf, ":call writefile([@\"], 'Xbuf')\<CR>")
  call WaitFor(XbufNotEmpty)
  call assert_equal("n yellow", readfile('Xbuf')[0])
  call delete('Xbuf')

  " Test for pasting text using middle click
  call term_sendkeys(buf, ":let @r='bright '\<CR>")
  call test_setmouse(2, 22)
  call term_sendkeys(buf, "\"r\<MiddleMouse>\<MiddleRelease>")
  call TermWait(buf, 50)
  call term_sendkeys(buf, ":call writefile([getline(2)], 'Xbuf')\<CR>")
  call WaitFor(XbufNotEmpty)
  call assert_equal("red bright blue", readfile('Xbuf')[0][-15:])
  call delete('Xbuf')

  " cleanup
  call TermWait(buf)
  call StopVimInTerminal(buf)
  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  let &clipboard = save_clipboard
  set mousetime&
  call delete('Xbuf')
endfunc

" Test for sync buffer cwd with shell's pwd
func Test_terminal_sync_shell_dir()
  CheckUnix
  " The test always use sh (see src/testdir/unix.vim).
  " BSD's sh doesn't seem to play well with the OSC 7 escape sequence.
  CheckNotBSD

  set asd
  " , is
  "  1. a valid character for directory names
  "  2. a reserved character in url-encoding
  let chars = ",a"
  " "," is url-encoded as '%2C'
  let chars_url = "%2Ca"
  let tmpfolder = fnamemodify(tempname(),':h') .. '/' .. chars
  let tmpfolder_url = fnamemodify(tempname(),':h') .. '/' .. chars_url
  call mkdir(tmpfolder, "p")
  let buf = Run_shell_in_terminal({})
  call term_sendkeys(buf, "echo $'\\e\]7;file://" .. tmpfolder_url .. "\\a'\<CR>")
  "call term_sendkeys(buf, "cd " .. tmpfolder .. "\<CR>")
  call TermWait(buf)
  if has("mac")
    let expected = "/private" .. tmpfolder
  else
    let expected = tmpfolder
  endif
  call assert_equal(expected, getcwd(winnr()))

  set noasd
endfunc

" Test for modeless selection in a terminal
func Test_term_modeless_selection()
  CheckUnix
  CheckNotGui
  CheckRunVimInTerminal
  CheckFeature clipboard_working

  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  set mouse=a term=xterm ttymouse=sgr mousetime=200
  set clipboard=autoselectml

  let lines =<< trim END
    one two three four five
    red green yellow red blue
    vim emacs sublime nano
  END
  call writefile(lines, 'Xtest_modeless', 'D')

  " Create a terminal window running Vim for the test with mouse disabled
  let prev_win = win_getid()
  let buf = RunVimInTerminal('Xtest_modeless -n', {})
  call term_sendkeys(buf, ":set nocompatible\<CR>")
  call term_sendkeys(buf, ":set mouse=\<CR>")
  call TermWait(buf)
  redraw!

  " Use the mouse to enter the terminal window
  call win_gotoid(prev_win)
  call feedkeys(MouseLeftClickCode(1, 1), 'x')
  call feedkeys(MouseLeftReleaseCode(1, 1), 'x')
  call TermWait(buf)
  call assert_equal(1, getwininfo(win_getid())[0].terminal)

  " Test for copying a modeless selection to clipboard
  let @* = 'clean'
  " communicating with X server may take a little time
  sleep 100m
  call feedkeys(MouseLeftClickCode(2, 3), 'x')
  call feedkeys(MouseLeftDragCode(2, 11), 'x')
  call feedkeys(MouseLeftReleaseCode(2, 11), 'x')
  call assert_equal("d green y", @*)

  " cleanup
  call TermWait(buf)
  call StopVimInTerminal(buf)
  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousetime& clipboard&
  new | only!
endfunc

func Test_terminal_getwinpos()
  CheckRunVimInTerminal

  " split, go to the bottom-right window
  split
  wincmd j
  set splitright

  let buf = RunVimInTerminal('', {'cols': 60})
  call TermWait(buf, 100)
  call term_sendkeys(buf, ":echo getwinpos(500)\<CR>")

  " Find the output of getwinpos() in the bottom line.
  let rows = term_getsize(buf)[0]
  call WaitForAssert({-> assert_match('\[\d\+, \d\+\]', term_getline(buf, rows))})
  let line = term_getline(buf, rows)
  let xpos = str2nr(substitute(line, '\[\(\d\+\), \d\+\]', '\1', ''))
  let ypos = str2nr(substitute(line, '\[\d\+, \(\d\+\)\]', '\1', ''))

  " Position must be bigger than the getwinpos() result of Vim itself.
  " The calculation in the console assumes a 10 x 7 character cell.
  " In the GUI it can be more, let's assume a 20 x 14 cell.
  " And then add 100 / 200 tolerance.
  let [xroot, yroot] = getwinpos()
  let winpos = 50->getwinpos()
  call assert_equal(xroot, winpos[0])
  call assert_equal(yroot, winpos[1])
  let [winrow, wincol] = win_screenpos(0)
  let xoff = wincol * (has('gui_running') ? 14 : 7) + 100
  let yoff = winrow * (has('gui_running') ? 20 : 10) + 200
  call assert_inrange(xroot + 2, xroot + xoff, xpos)
  call assert_inrange(yroot + 2, yroot + yoff, ypos)

  call TermWait(buf)
  call term_sendkeys(buf, ":q\<CR>")
  call StopVimInTerminal(buf)
  set splitright&
  only!
endfunc

func Test_terminal_term_start_error()
  func s:term_start_error() abort
    try
      return term_start([[]])
    catch
      return v:exception
    finally
      "
    endtry
  endfunc
  autocmd WinEnter * call type(0)

  " Must not crash in s:term_start_error, nor the exception thrown.
  let result = s:term_start_error()
  call assert_match('^Vim(return):E730:', result)

  autocmd! WinEnter
  delfunc s:term_start_error
endfunc


" vim: shiftwidth=2 sts=2 expandtab
