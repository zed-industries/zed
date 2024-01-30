" Test WinBar

source check.vim
CheckFeature menu

source shared.vim
source screendump.vim

func Test_add_remove_menu()
  new
  amenu 1.10 WinBar.Next :let g:did_next = 11<CR>
  amenu 1.20 WinBar.Cont :let g:did_cont = 12<CR>
  redraw
  call assert_match('Next    Cont', Screenline(1))

  emenu WinBar.Next
  call assert_equal(11, g:did_next)
  emenu WinBar.Cont
  call assert_equal(12, g:did_cont)

  wincmd w
  call assert_fails('emenu WinBar.Next', 'E334:')
  wincmd p

  aunmenu WinBar.Next
  aunmenu WinBar.Cont
  close
endfunc

" Create a WinBar with three buttons.
" Columns of the button edges:
" _Next_  _Cont_  _Close_
" 2    7  10  15  18   24
func SetupWinbar()
  amenu 1.10 WinBar.Next :let g:did_next = 11<CR>
  amenu 1.20 WinBar.Cont :let g:did_cont = 12<CR>
  amenu 1.30 WinBar.Close :close<CR>
  redraw
  call assert_match('Next    Cont    Close', Screenline(1))
endfunc

func Test_click_in_winbar()
  new
  call SetupWinbar()
  let save_mouse = &mouse
  set mouse=a

  let g:did_next = 0
  let g:did_cont = 0
  for col in [1, 8, 9, 16, 17, 25, 26]
    call test_setmouse(1, col)
    call feedkeys("\<LeftMouse>", "xt")
    call assert_equal(0, g:did_next, 'col ' .. col)
    call assert_equal(0, g:did_cont, 'col ' .. col)
  endfor

  for col in range(2, 7)
    let g:did_next = 0
    call test_setmouse(1, col)
    call feedkeys("\<LeftMouse>", "xt")
    call assert_equal(11, g:did_next, 'col ' .. col)
  endfor

  for col in range(10, 15)
    let g:did_cont = 0
    call test_setmouse(1, col)
    call feedkeys("\<LeftMouse>", "xt")
    call assert_equal(12, g:did_cont, 'col ' .. col)
  endfor

  let wincount = winnr('$')
  call test_setmouse(1, 20)
  call feedkeys("\<LeftMouse>", "xt")
  call assert_equal(wincount - 1, winnr('$'))

  let &mouse = save_mouse
endfunc

func Test_click_in_other_winbar()
  new
  call SetupWinbar()
  let save_mouse = &mouse
  set mouse=a
  let winid = win_getid()

  split
  let [row, col] = win_screenpos(winid)

  " Click on Next button in other window
  let g:did_next = 0
  call test_setmouse(row, 5)
  call feedkeys("\<LeftMouse>", "xt")
  call assert_equal(11, g:did_next)

  " Click on Cont button in other window from Visual mode
  let g:did_cont = 0
  call setline(1, 'select XYZ here')
  call test_setmouse(row, 12)
  call feedkeys("0fXvfZ\<LeftMouse>x", "xt")
  call assert_equal(12, g:did_cont)
  call assert_equal('select  here', getline(1))

  " Click on Close button in other window
  let wincount = winnr('$')
  let winid = win_getid()
  call test_setmouse(row, 20)
  call feedkeys("\<LeftMouse>", "xt")
  call assert_equal(wincount - 1, winnr('$'))
  call assert_equal(winid, win_getid())

  bwipe!
endfunc

func Test_redraw_after_scroll()
  new
  amenu 1.10 WinBar.Next :let g:did_next = 11<CR>
  redraw
  call assert_equal("  Next", Screenline(1))
  echo "some\nmore"
  redraw
  call assert_equal("  Next", Screenline(1))
  bwipe!
endfunc

func Test_winbar_not_visible()
  CheckScreendump

  let lines =<< trim END
      split
      nnoremenu WinBar.Test :test
      set winminheight=0
      wincmd j
      wincmd _
  END
  call writefile(lines, 'XtestWinbarNotVisible', 'D')
  let buf = RunVimInTerminal('-S XtestWinbarNotVisible', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_winbar_not_visible', {})

  " clean up
  call StopVimInTerminal(buf)
endfunction

func Test_winbar_not_visible_custom_statusline()
  CheckScreendump

  let lines =<< trim END
      split
      nnoremenu WinBar.Test :test
      set winminheight=0
      set statusline=abcde
      wincmd j
      wincmd _
  END
  call writefile(lines, 'XtestWinbarNotVisible', 'D')
  let buf = RunVimInTerminal('-S XtestWinbarNotVisible', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_winbar_not_visible_custom_statusline', {})

  " clean up
  call StopVimInTerminal(buf)
endfunction

func Test_drag_statusline_with_winbar()
  call SetupWinbar()
  let save_mouse = &mouse
  set mouse=a
  set laststatus=2

  call test_setmouse(&lines - 1, 1)
  call feedkeys("\<LeftMouse>", 'xt')
  call test_setmouse(&lines - 2, 1)
  call feedkeys("\<LeftDrag>", 'xt')
  call assert_equal(2, &cmdheight)

  call test_setmouse(&lines - 2, 1)
  call feedkeys("\<LeftMouse>", 'xt')
  call test_setmouse(&lines - 3, 1)
  call feedkeys("\<LeftDrag>", 'xt')
  call assert_equal(3, &cmdheight)

  call test_setmouse(&lines - 3, 1)
  call feedkeys("\<LeftMouse>", 'xt')
  call test_setmouse(&lines - 1, 1)
  call feedkeys("\<LeftDrag>", 'xt')
  call assert_equal(1, &cmdheight)

  let &mouse = save_mouse
  set laststatus&
endfunc

" vim: shiftwidth=2 sts=2 expandtab
