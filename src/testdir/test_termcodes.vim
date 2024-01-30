" Tests for decoding escape sequences sent by the terminal.

" This only works for Unix in a terminal
source check.vim
CheckNotGui
CheckUnix

source shared.vim
source mouse.vim
source view_util.vim
source term_util.vim

func Test_term_mouse_left_click()
  new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  call setline(1, ['line 1', 'line 2', 'line 3 is a bit longer'])

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec + g:Ttymouse_netterm
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    go
    call assert_equal([0, 1, 1, 0], getpos('.'), msg)
    let row = 2
    let col = 6
    call MouseLeftClick(row, col)
    call MouseLeftRelease(row, col)
    call assert_equal([0, 2, 6, 0], getpos('.'), msg)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  bwipe!
endfunc

func Test_xterm_mouse_right_click_extends_visual()
  if has('mac')
    throw "Skipped: test right click in visual mode does not work on macOs (why?)"
  endif
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  for visual_mode in ["v", "V", "\<C-V>"]
    for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
      let msg = 'visual=' .. visual_mode .. ' ttymouse=' .. ttymouse_val
      exe 'set ttymouse=' .. ttymouse_val

      call setline(1, repeat([repeat('-', 7)], 7))
      call MouseLeftClick(4, 4)
      call MouseLeftRelease(4, 4)
      exe  "norm! " .. visual_mode

      " Right click extends top left of visual area.
      call MouseRightClick(2, 2)
      call MouseRightRelease(2, 2)

      " Right click extends bottom right of visual area.
      call MouseRightClick(6, 6)
      call MouseRightRelease(6, 6)
      norm! r1gv

      " Right click shrinks top left of visual area.
      call MouseRightClick(3, 3)
      call MouseRightRelease(3, 3)

      " Right click shrinks bottom right of visual area.
      call MouseRightClick(5, 5)
      call MouseRightRelease(5, 5)
      norm! r2

      if visual_mode ==# 'v'
        call assert_equal(['-------',
              \            '-111111',
              \            '1122222',
              \            '2222222',
              \            '2222211',
              \            '111111-',
              \            '-------'], getline(1, '$'), msg)
      elseif visual_mode ==# 'V'
        call assert_equal(['-------',
              \            '1111111',
              \            '2222222',
              \            '2222222',
              \            '2222222',
              \            '1111111',
              \            '-------'], getline(1, '$'), msg)
      else
        call assert_equal(['-------',
              \            '-11111-',
              \            '-12221-',
              \            '-12221-',
              \            '-12221-',
              \            '-11111-',
              \            '-------'], getline(1, '$'), msg)
      endif
    endfor
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  bwipe!
endfunc

" Test that <C-LeftMouse> jumps to help tag and <C-RightMouse> jumps back.
" Also test for g<LeftMouse> and g<RightMouse>.
func Test_xterm_mouse_tagjump()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  set mouse=a term=xterm
  call WaitForResponses()

  for ttymouse_val in g:Ttymouse_values
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    help
    /usr_02.txt
    norm! zt

    " CTRL-left click to jump to a tag
    let row = 1
    let col = 1
    call MouseCtrlLeftClick(row, col)
    call MouseLeftRelease(row, col)
    call assert_match('usr_02.txt$', bufname('%'), msg)
    call assert_equal('*usr_02.txt*', expand('<cWORD>'), msg)

    " CTRL-right click to pop a tag
    call MouseCtrlRightClick(row, col)
    call MouseRightRelease(row, col)
    call assert_match('help.txt$', bufname('%'), msg)
    call assert_equal('|usr_02.txt|', expand('<cWORD>'), msg)

    " Jump to a tag
    exe "normal \<C-]>"
    call assert_match('usr_02.txt$', bufname('%'), msg)
    call assert_equal('*usr_02.txt*', expand('<cWORD>'), msg)

    " Use CTRL-right click in insert mode to pop the tag
    new
    let str = 'iHello' .. MouseCtrlRightClickCode(row, col)
          \ .. MouseRightReleaseCode(row, col) .. "\<C-C>"
    call assert_fails('call feedkeys(str, "Lx!")', 'E37:', msg)
    close!

    " CTRL-right click with a count
    let str = "4" .. MouseCtrlRightClickCode(row, col)
          \ .. MouseRightReleaseCode(row, col)
    call assert_fails('call feedkeys(str, "Lx!")', 'E555:', msg)
    call assert_match('help.txt$', bufname('%'), msg)
    call assert_equal(1, line('.'), msg)

    " g<LeftMouse> to jump to a tag
    /usr_02.txt
    norm! zt
    call test_setmouse(row, col)
    exe "normal g\<LeftMouse>"
    call assert_match('usr_02.txt$', bufname('%'), msg)
    call assert_equal('*usr_02.txt*', expand('<cWORD>'), msg)

    " g<RightMouse> to pop to a tag
    call test_setmouse(row, col)
    exe "normal g\<RightMouse>"
    call assert_match('help.txt$', bufname('%'), msg)
    call assert_equal('|usr_02.txt|', expand('<cWORD>'), msg)

    %bw!
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
endfunc

func Test_term_mouse_middle_click()
  CheckFeature clipboard_working

  new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  let save_quotestar = @*
  let save_quoteplus = @+
  set mouse=a term=xterm
  call WaitForResponses()

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    call setline(1, ['123456789', '123456789'])
    let @* = 'abc'

    " Middle-click in the middle of the line pastes text where clicked.
    let row = 1
    let col = 6
    call MouseMiddleClick(row, col)
    call MouseMiddleRelease(row, col)
    call assert_equal(['12345abc6789', '123456789'], getline(1, '$'), msg)

    " Middle-click beyond end of the line pastes text at the end of the line.
    let col = 20
    call MouseMiddleClick(row, col)
    call MouseMiddleRelease(row, col)
    call assert_equal(['12345abc6789abc', '123456789'], getline(1, '$'), msg)

    " Middle-click beyond the last line pastes in the last line.
    let row = 5
    let col = 3
    call MouseMiddleClick(row, col)
    call MouseMiddleRelease(row, col)
    call assert_equal(['12345abc6789abc', '12abc3456789'], getline(1, '$'), msg)

    " Middle mouse click in operator pending mode beeps
    call assert_beeps('exe "normal c\<MiddleMouse>"')

    " Clicking middle mouse in visual mode, yanks the selection and pastes the
    " clipboard contents
    let save_clipboard = &clipboard
    set clipboard=
    let @" = ''
    call cursor(1, 1)
    call feedkeys("v3l" ..
          \ MouseMiddleClickCode(2, 7) .. MouseMiddleReleaseCode(2, 7), 'Lx!')
    call assert_equal(['12345abc6789abc', '12abc3abc456789'],
          \ getline(1, '$'), msg)
    call assert_equal('1234', @", msg)
    let &clipboard = save_clipboard

    " Clicking middle mouse in select mode, replaces the selected text with
    " the clipboard contents
    let @+ = 'xyz'
    call cursor(1, 3)
    exe "normal gh\<Right>\<Right>\<MiddleMouse>"
    call assert_equal(['12xyzabc6789abc', '12abc3abc456789'],
          \ getline(1, '$'), msg)

    " Prefixing middle click with [ or ] fixes the indent after pasting.
    %d
    call setline(1, "    one two")
    call setreg('r', 'red blue', 'l')
    call test_setmouse(1, 5)
    exe "normal \"r[\<MiddleMouse>"
    call assert_equal('    red blue', getline(1), msg)
    call test_setmouse(2, 5)
    exe "normal \"r]\<MiddleMouse>"
    call assert_equal('    red blue', getline(3), msg)
    %d
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  let @* = save_quotestar
  let @+ = save_quoteplus
  bwipe!
endfunc

" If clipboard is not working, then clicking the middle mouse button in visual
" mode, will copy and paste the selected text.
func Test_term_mouse_middle_click_no_clipboard()
  if has('clipboard_working')
    throw 'Skipped: clipboard support works'
  endif
  new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    call setline(1, ['123456789', '123456789'])

    " Clicking middle mouse in visual mode, yanks the selection and pastes it
    call cursor(1, 1)
    call feedkeys("v3l" ..
          \ MouseMiddleClickCode(2, 7) .. MouseMiddleReleaseCode(2, 7), 'Lx!')
    call assert_equal(['123456789', '1234561234789'],
          \ getline(1, '$'), msg)
  endfor

  call test_override('no_query_mouse', 0)
  let &ttymouse = save_ttymouse
  let &term = save_term
  let &mouse = save_mouse
  close!
endfunc

func Test_term_mouse_middle_click_insert_mode()
  CheckFeature clipboard_working

  new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    call setline(1, ['123456789', '123456789'])
    let @* = 'abc'

    " Middle-click in insert mode doesn't move the cursor but inserts the
    " contents of a register
    call cursor(1, 4)
    call feedkeys('i' ..
          \ MouseMiddleClickCode(2, 7) .. MouseMiddleReleaseCode(2, 7) ..
          \ "\<C-C>", 'Lx!')
    call assert_equal(['123abc456789', '123456789'],
          \ getline(1, '$'), msg)
    call assert_equal([1, 6], [line('.'), col('.')], msg)

    " Middle-click in replace mode
    call cursor(1, 1)
    call feedkeys('$R' ..
          \ MouseMiddleClickCode(2, 7) .. MouseMiddleReleaseCode(2, 7) ..
          \ "\<C-C>", 'Lx!')
    call assert_equal(['123abc45678abc', '123456789'],
          \ getline(1, '$'), msg)
    call assert_equal([1, 14], [line('.'), col('.')], msg)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  close!
endfunc

" Test for switching window using mouse in insert mode
func Test_term_mouse_switch_win_insert_mode()
  5new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm ttymouse=xterm2
  call WaitForResponses()

  call feedkeys('ivim' ..
        \ MouseLeftClickCode(8, 6) .. MouseLeftReleaseCode(8, 6) ..
        \ "\<C-C>", 'Lx!')
  call assert_equal(2, winnr())
  wincmd w
  call assert_equal('n', mode())
  call assert_equal(['vim'], getline(1, '$'))

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  close!
endfunc

" Test for using the mouse to increase the height of the cmdline window
func Test_mouse_cmdwin_resize()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm ttymouse=xterm2
  call WaitForResponses()

  5new
  redraw!

  let h = 0
  let row = &lines - &cmdwinheight - 2
  call feedkeys("q:" ..
        \ MouseLeftClickCode(row, 1) ..
        \ MouseLeftDragCode(row - 1, 1) ..
        \ MouseLeftReleaseCode(row - 2, 1) ..
        \ "alet h = \<C-R>=winheight(0)\<CR>\<CR>", 'Lx!')
  call assert_equal(&cmdwinheight + 2, h)

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  close!
endfunc

" TODO: for unclear reasons this test fails if it comes after
" Test_xterm_mouse_ctrl_click()
func Test_1xterm_mouse_wheel()
  new
  let save_mouse = &mouse
  let save_term = &term
  let save_wrap = &wrap
  let save_ttymouse = &ttymouse
  set mouse=a term=xterm nowrap
  call WaitForResponses()

  call setline(1, range(100000000000000, 100000000000100))

  for ttymouse_val in g:Ttymouse_values
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    go
    call assert_equal(1, line('w0'), msg)
    call assert_equal([0, 1, 1, 0], getpos('.'), msg)

    call MouseWheelDown(1, 1)
    call assert_equal(4, line('w0'), msg)
    call assert_equal([0, 4, 1, 0], getpos('.'), msg)

    call MouseWheelDown(1, 1)
    call assert_equal(7, line('w0'), msg)
    call assert_equal([0, 7, 1, 0], getpos('.'), msg)

    call MouseWheelUp(1, 1)
    call assert_equal(4, line('w0'), msg)
    call assert_equal([0, 7, 1, 0], getpos('.'), msg)

    call MouseWheelUp(1, 1)
    call assert_equal(1, line('w0'), msg)
    call assert_equal([0, 7, 1, 0], getpos('.'), msg)

    call MouseWheelRight(1, 1)
    call assert_equal(7, 1 + virtcol(".") - wincol(), msg)
    call assert_equal([0, 7, 7, 0], getpos('.'), msg)

    call MouseWheelRight(1, 1)
    call assert_equal(13, 1 + virtcol(".") - wincol(), msg)
    call assert_equal([0, 7, 13, 0], getpos('.'), msg)

    call MouseWheelLeft(1, 1)
    call assert_equal(7, 1 + virtcol(".") - wincol(), msg)
    call assert_equal([0, 7, 13, 0], getpos('.'), msg)

    call MouseWheelLeft(1, 1)
    call assert_equal(1, 1 + virtcol(".") - wincol(), msg)
    call assert_equal([0, 7, 13, 0], getpos('.'), msg)

  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &wrap = save_wrap
  let &ttymouse = save_ttymouse
  bwipe!
endfunc

" Test that dragging beyond the window (at the bottom and at the top)
" scrolls window content by the number of lines beyond the window.
func Test_term_mouse_drag_beyond_window()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  let col = 1
  call setline(1, range(1, 100))

  " Split into 3 windows, and go into the middle window
  " so we test dragging mouse below and above the window.
  2split
  wincmd j
  2split

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val

    " Line #10 at the top.
    norm! 10zt
    redraw
    call assert_equal(10, winsaveview().topline, msg)
    call assert_equal(2, winheight(0), msg)

    let row = 4
    call MouseLeftClick(row, col)
    call assert_equal(10, winsaveview().topline, msg)

    " Drag downwards. We're still in the window so topline should
    " not change yet.
    let row += 1
    call MouseLeftDrag(row, col)
    call assert_equal(10, winsaveview().topline, msg)

    " We now leave the window at the bottom, so the window content should
    " scroll by 1 line, then 2 lines (etc) as we drag further away.
    let row += 1
    call MouseLeftDrag(row, col)
    call assert_equal(11, winsaveview().topline, msg)

    let row += 1
    call MouseLeftDrag(row, col)
    call assert_equal(13, winsaveview().topline, msg)

    " Now drag upwards.
    let row -= 1
    call MouseLeftDrag(row, col)
    call assert_equal(14, winsaveview().topline, msg)

    " We're now back in the window so the topline should not change.
    let row -= 1
    call MouseLeftDrag(row, col)
    call assert_equal(14, winsaveview().topline, msg)

    let row -= 1
    call MouseLeftDrag(row, col)
    call assert_equal(14, winsaveview().topline, msg)

    " We now leave the window at the top so the window content should
    " scroll by 1 line, then 2, then 3 (etc) in the opposite direction.
    let row -= 1
    call MouseLeftDrag(row, col)
    call assert_equal(13, winsaveview().topline, msg)

    let row -= 1
    call MouseLeftDrag(row, col)
    call assert_equal(11, winsaveview().topline, msg)

    let row -= 1
    call MouseLeftDrag(row, col)
    call assert_equal(8, winsaveview().topline, msg)

    call MouseLeftRelease(row, col)
    call assert_equal(8, winsaveview().topline, msg)
    call assert_equal(2, winheight(0), msg)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  bwipe!
endfunc

func Test_term_mouse_drag_window_separator()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val

    " Split horizontally and test dragging the horizontal window separator.
    split
    let rowseparator = winheight(0) + 1
    let row = rowseparator
    let col = 1

    " When 'ttymouse' is 'xterm2', row/col bigger than 223 are not supported.
    if ttymouse_val !=# 'xterm2' || row <= 223
      call MouseLeftClick(row, col)
      let row -= 1
      call MouseLeftDrag(row, col)
      call assert_equal(rowseparator - 1, winheight(0) + 1, msg)
      let row += 1
      call MouseLeftDrag(row, col)
      call assert_equal(rowseparator, winheight(0) + 1, msg)
      call MouseLeftRelease(row, col)
      call assert_equal(rowseparator, winheight(0) + 1, msg)
    endif
    bwipe!

    " Split vertically and test dragging the vertical window separator.
    vsplit
    let colseparator = winwidth(0) + 1
    let row = 1
    let col = colseparator

    " When 'ttymouse' is 'xterm2', row/col bigger than 223 are not supported.
    if ttymouse_val !=# 'xterm2' || col <= 223
      call MouseLeftClick(row, col)
      let col -= 1
      call MouseLeftDrag(row, col)
      call assert_equal(colseparator - 1, winwidth(0) + 1, msg)
      let col += 1
      call MouseLeftDrag(row, col)
      call assert_equal(colseparator, winwidth(0) + 1, msg)
      call MouseLeftRelease(row, col)
      call assert_equal(colseparator, winwidth(0) + 1, msg)
    endif
    bwipe!
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
endfunc

func Test_term_mouse_drag_statusline()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  let save_laststatus = &laststatus
  set mouse=a term=xterm laststatus=2
  call WaitForResponses()

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val

    call assert_equal(1, &cmdheight, msg)
    let rowstatusline = winheight(0) + 1
    let row = rowstatusline
    let col = 1

    if ttymouse_val ==# 'xterm2' && row > 223
      " When 'ttymouse' is 'xterm2', row/col bigger than 223 are not supported.
      continue
    endif

    call MouseLeftClick(row, col)
    let row -= 1
    call MouseLeftDrag(row, col)
    call assert_equal(2, &cmdheight, msg)
    call assert_equal(rowstatusline - 1, winheight(0) + 1, msg)
    let row += 1
    call MouseLeftDrag(row, col)
    call assert_equal(1, &cmdheight, msg)
    call assert_equal(rowstatusline, winheight(0) + 1, msg)
    call MouseLeftRelease(row, col)
    call assert_equal(1, &cmdheight, msg)
    call assert_equal(rowstatusline, winheight(0) + 1, msg)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  let &laststatus = save_laststatus
endfunc

func Test_term_mouse_click_tab()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  let row = 1

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec + g:Ttymouse_netterm
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    e Xfoo
    tabnew Xbar

    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '    Xfoo',
        \              'Tab page 2',
        \              '>   Xbar'], a, msg)

    " Test clicking on tab names in the tabline at the top.
    let col = 2
    redraw
    call MouseLeftClick(row, col)
    call MouseLeftRelease(row, col)
    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '>   Xfoo',
        \              'Tab page 2',
        \              '    Xbar'], a, msg)

    let col = 9
    call MouseLeftClick(row, col)
    call MouseLeftRelease(row, col)
    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '    Xfoo',
        \              'Tab page 2',
        \              '>   Xbar'], a, msg)

    %bwipe!
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
endfunc

func Test_term_mouse_click_X_to_close_tab()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  let row = 1
  let col = &columns

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec + g:Ttymouse_netterm
    if ttymouse_val ==# 'xterm2' && col > 223
      " When 'ttymouse' is 'xterm2', row/col bigger than 223 are not supported.
      continue
    endif
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    e Xtab1
    tabnew Xtab2
    tabnew Xtab3
    tabn 2

    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '    Xtab1',
        \              'Tab page 2',
        \              '>   Xtab2',
        \              'Tab page 3',
        \              '    Xtab3'], a, msg)

    " Click on "X" in tabline to close current tab i.e. Xtab2.
    redraw
    call MouseLeftClick(row, col)
    call MouseLeftRelease(row, col)
    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '    Xtab1',
        \              'Tab page 2',
        \              '>   Xtab3'], a, msg)

    %bwipe!
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
endfunc

func Test_term_mouse_drag_to_move_tab()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  " Set 'mousetime' to 1 to avoid recognizing a double-click in the loop
  set mouse=a term=xterm mousetime=1
  call WaitForResponses()

  let row = 1

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    e Xtab1
    tabnew Xtab2

    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '    Xtab1',
        \              'Tab page 2',
        \              '>   Xtab2'], a, msg)
    redraw

    " Click in tab2 and drag it to tab1.
    " Check getcharmod() to verify that click is not
    " interpreted as a spurious double-click.
    call MouseLeftClick(row, 10)
    call assert_equal(0, getcharmod(), msg)
    for col in [9, 8, 7, 6]
      call MouseLeftDrag(row, col)
    endfor
    call MouseLeftRelease(row, col)
    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '>   Xtab2',
        \              'Tab page 2',
        \              '    Xtab1'], a, msg)

    " Switch to tab1
    tabnext
    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '    Xtab2',
        \              'Tab page 2',
        \              '>   Xtab1'], a, msg)

    " Click in tab2 and drag it to tab1.
    " This time it is non-current tab.
    call MouseLeftClick(row, 6)
    call assert_equal(0, getcharmod(), msg)
    for col in [7, 8, 9, 10]
      call MouseLeftDrag(row, col)
    endfor
    call MouseLeftRelease(row, col)
    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '    Xtab1',
        \              'Tab page 2',
        \              '>   Xtab2'], a, msg)

    " Click elsewhere so that click in next iteration is not
    " interpreted as unwanted double-click.
    call MouseLeftClick(row, 11)
    call MouseLeftRelease(row, 11)

    %bwipe!
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  set mousetime&
endfunc

func Test_term_mouse_double_click_to_create_tab()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  " Set 'mousetime' to a small value, so that double-click works but we don't
  " have to wait long to avoid a triple-click.
  set mouse=a term=xterm mousetime=200
  call WaitForResponses()

  let row = 1
  let col = 10

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    e Xtab1
    tabnew Xtab2

    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '    Xtab1',
        \              'Tab page 2',
        \              '>   Xtab2'], a, msg)

    redraw
    call MouseLeftClick(row, col)
    " Check getcharmod() to verify that first click is not
    " interpreted as a spurious double-click.
    call assert_equal(0, getcharmod(), msg)
    call MouseLeftRelease(row, col)
    call MouseLeftClick(row, col)
    call assert_equal(32, getcharmod(), msg) " double-click
    call MouseLeftRelease(row, col)
    let a = split(execute(':tabs'), "\n")
    call assert_equal(['Tab page 1',
        \              '    Xtab1',
        \              'Tab page 2',
        \              '>   [No Name]',
        \              'Tab page 3',
        \              '    Xtab2'], a, msg)

    " Click elsewhere so that click in next iteration is not
    " interpreted as unwanted double click.
    call MouseLeftClick(row, col + 1)
    call MouseLeftRelease(row, col + 1)

    %bwipe!
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  set mousetime&
endfunc

" Test double/triple/quadruple click in normal mode to visually select.
func Test_term_mouse_multiple_clicks_to_visually_select()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)

  " 'mousetime' must be sufficiently large, or else the test is flaky when
  " using a ssh connection with X forwarding; i.e. ssh -X (issue #7563).
  set mouse=a term=xterm mousetime=600
  call WaitForResponses()

  new

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    call setline(1, ['foo [foo bar] foo', 'foo'])

    " Double-click on word should visually select the word.
    call MouseLeftClick(1, 2)
    call assert_equal(0, getcharmod(), msg)
    call MouseLeftRelease(1, 2)
    call MouseLeftClick(1, 2)
    call assert_equal(32, getcharmod(), msg) " double-click
    call MouseLeftRelease(1, 2)
    call assert_equal('v', mode(), msg)
    norm! r1
    call assert_equal(['111 [foo bar] foo', 'foo'], getline(1, '$'), msg)

    " Double-click on opening square bracket should visually
    " select the whole [foo bar].
    call MouseLeftClick(1, 5)
    call assert_equal(0, getcharmod(), msg)
    call MouseLeftRelease(1, 5)
    call MouseLeftClick(1, 5)
    call assert_equal(32, getcharmod(), msg) " double-click
    call MouseLeftRelease(1, 5)
    call assert_equal('v', mode(), msg)
    norm! r2
    call assert_equal(['111 222222222 foo', 'foo'], getline(1, '$'), msg)

    " Triple-click should visually select the whole line.
    call MouseLeftClick(1, 3)
    call assert_equal(0, getcharmod(), msg)
    call MouseLeftRelease(1, 3)
    call MouseLeftClick(1, 3)
    call assert_equal(32, getcharmod(), msg) " double-click
    call MouseLeftRelease(1, 3)
    call MouseLeftClick(1, 3)
    call assert_equal(64, getcharmod(), msg) " triple-click
    call MouseLeftRelease(1, 3)
    call assert_equal('V', mode(), msg)
    norm! r3
    call assert_equal(['33333333333333333', 'foo'], getline(1, '$'), msg)

    " Quadruple-click should start visual block select.
    call MouseLeftClick(1, 2)
    call assert_equal(0, getcharmod(), msg)
    call MouseLeftRelease(1, 2)
    call MouseLeftClick(1, 2)
    call assert_equal(32, getcharmod(), msg) " double-click
    call MouseLeftRelease(1, 2)
    call MouseLeftClick(1, 2)
    call assert_equal(64, getcharmod(), msg) " triple-click
    call MouseLeftRelease(1, 2)
    call MouseLeftClick(1, 2)
    call assert_equal(96, getcharmod(), msg) " quadruple-click
    call MouseLeftRelease(1, 2)
    call assert_equal("\<c-v>", mode(), msg)
    norm! r4
    call assert_equal(['34333333333333333', 'foo'], getline(1, '$'), msg)

    " Double-click on a space character should visually select all the
    " consecutive space characters.
    %d
    call setline(1, '    one two')
    call MouseLeftClick(1, 2)
    call MouseLeftRelease(1, 2)
    call MouseLeftClick(1, 2)
    call MouseLeftRelease(1, 2)
    call assert_equal('v', mode(), msg)
    norm! r1
    call assert_equal(['1111one two'], getline(1, '$'), msg)

    " Double-click on a word with exclusive selection
    set selection=exclusive
    let @" = ''
    call MouseLeftClick(1, 10)
    call MouseLeftRelease(1, 10)
    call MouseLeftClick(1, 10)
    call MouseLeftRelease(1, 10)
    norm! y
    call assert_equal('two', @", msg)

    " Double click to select a block of text with exclusive selection
    %d
    call setline(1, 'one (two) three')
    call MouseLeftClick(1, 5)
    call MouseLeftRelease(1, 5)
    call MouseLeftClick(1, 5)
    call MouseLeftRelease(1, 5)
    norm! y
    call assert_equal(5, col("'<"), msg)
    call assert_equal(10, col("'>"), msg)

    call MouseLeftClick(1, 9)
    call MouseLeftRelease(1, 9)
    call MouseLeftClick(1, 9)
    call MouseLeftRelease(1, 9)
    norm! y
    call assert_equal(5, col("'<"), msg)
    call assert_equal(10, col("'>"), msg)
    set selection&

    " Click somewhere else so that the clicks above is not combined with the
    " clicks in the next iteration.
    call MouseRightClick(3, 10)
    call MouseRightRelease(3, 10)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousetime&
  call test_override('no_query_mouse', 0)
  bwipe!
endfunc

" Test for selecting text in visual blockwise mode using Alt-LeftClick
func Test_mouse_alt_leftclick()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm mousetime=200
  set mousemodel=popup
  call WaitForResponses()

  new
  call setline(1, 'one (two) three')

  for ttymouse_val in g:Ttymouse_values
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val

    " Left click with the Alt modifier key should extend the selection in
    " blockwise visual mode.
    let @" = ''
    call MouseLeftClick(1, 3)
    call MouseLeftRelease(1, 3)
    call MouseAltLeftClick(1, 11)
    call MouseLeftRelease(1, 11)
    call assert_equal("\<C-V>", mode(), msg)
    normal! y
    call assert_equal('e (two) t', @")
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousetime& mousemodel&
  call test_override('no_query_mouse', 0)
  close!
endfunc

func Test_xterm_mouse_click_in_fold_columns()
  new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  let save_foldcolumn = &foldcolumn
  set mouse=a term=xterm foldcolumn=3 ttymouse=xterm2
  call WaitForResponses()

  " Create 2 nested folds.
  call setline(1, range(1, 7))
  2,6fold
  norm! zR
  4,5fold
  call assert_equal([-1, -1, -1, 4, 4, -1, -1],
        \           map(range(1, 7), 'foldclosed(v:val)'))

  " Click in "+" of inner fold in foldcolumn should open it.
  redraw
  let row = 4
  let col = 2
  call MouseLeftClick(row, col)
  call MouseLeftRelease(row, col)
  call assert_equal([-1, -1, -1, -1, -1, -1, -1],
        \           map(range(1, 7), 'foldclosed(v:val)'))

  " Click in "-" of outer fold in foldcolumn should close it.
  redraw
  let row = 2
  let col = 1
  call MouseLeftClick(row, col)
  call MouseLeftRelease(row, col)
  call assert_equal([-1, 2, 2, 2, 2, 2, -1],
        \           map(range(1, 7), 'foldclosed(v:val)'))
  norm! zR

  " Click in "|" of inner fold in foldcolumn should close it.
  redraw
  let row = 5
  let col = 2
  call MouseLeftClick(row, col)
  call MouseLeftRelease(row, col)
  call assert_equal([-1, -1, -1, 4, 4, -1, -1],
        \           map(range(1, 7), 'foldclosed(v:val)'))

  let &foldcolumn = save_foldcolumn
  let &ttymouse = save_ttymouse
  let &term = save_term
  let &mouse = save_mouse
  bwipe!
endfunc

" Left or right click in Ex command line sets position of the cursor.
func Test_term_mouse_click_in_cmdline_to_set_pos()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  let row = &lines

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    " When 'ttymouse' is 'xterm2', row/col bigger than 223 are not supported.
    if ttymouse_val !=# 'xterm2' || row <= 223
      let msg = 'ttymouse=' .. ttymouse_val
      exe 'set ttymouse=' .. ttymouse_val


      call feedkeys(':"3456789'
            \       .. MouseLeftClickCode(row, 7)
            \       .. MouseLeftReleaseCode(row, 7) .. 'L'
            \       .. MouseRightClickCode(row, 4)
            \       .. MouseRightReleaseCode(row, 4) .. 'R'
            \       .. "\<CR>", 'Lx!')
      call assert_equal('"3R456L789', @:, msg)
    endif
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousetime&
  call test_override('no_query_mouse', 0)
endfunc

" Middle click in command line pastes at position of cursor.
func Test_term_mouse_middle_click_in_cmdline_to_paste()
  CheckFeature clipboard_working
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm
  call WaitForResponses()

  let row = &lines
  " Column values does not matter, paste is done at position of cursor.
  let col = 1
  let @* = 'paste'

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val

    call feedkeys(":\"->"
          \       .. MouseMiddleReleaseCode(row, col)
          \       .. MouseMiddleClickCode(row, col)
          \       .. "<-"
          \       .. MouseMiddleReleaseCode(row, col)
          \       .. MouseMiddleClickCode(row, col)
          \       .. "\<CR>", 'Lx!')
    call assert_equal('"->paste<-paste', @:, msg)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  let @* = ''
  call test_override('no_query_mouse', 0)
endfunc

" Test for making sure S-Middlemouse doesn't do anything
func Test_term_mouse_shift_middle_click()
  new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm ttymouse=xterm2 mousemodel=
  call WaitForResponses()

  call test_setmouse(1, 1)
  exe "normal \<S-MiddleMouse>"
  call assert_equal([''], getline(1, '$'))
  call assert_equal(1, winnr())

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousemodel&
  call test_override('no_query_mouse', 0)
  close!
endfunc

" Test for using mouse in visual mode
func Test_term_mouse_visual_mode()
  new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set term=xterm ttymouse=xterm2
  call WaitForResponses()

  " If visual mode is not present in 'mouse', then left click should not
  " do anything in visal mode.
  call setline(1, ['one two three four'])
  set mouse=nci
  call cursor(1, 5)
  let @" = ''
  call feedkeys("ve"
        \ .. MouseLeftClickCode(1, 15) .. MouseLeftReleaseCode(1, 15)
        \ .. 'y', 'Lx!')
  call assert_equal(5, col('.'))
  call assert_equal('two', @")

  " Pressing right click in visual mode should change the visual selection
  " if 'mousemodel' doesn't contain popup.
  " Right click after the visual selection
  set mousemodel=
  set mouse=a
  call test_setmouse(1, 13)
  exe "normal 5|ve\<RightMouse>y"
  call assert_equal('two three', @")
  call assert_equal(5, col('.'))

  " Right click before the visual selection
  call test_setmouse(1, 9)
  exe "normal 15|ve\<RightMouse>y"
  call assert_equal('three four', @")
  call assert_equal(9, col('.'))

  " Right click inside the selection closer to the start of the selection
  call test_setmouse(1, 7)
  exe "normal 5|vee\<RightMouse>lly"
  call assert_equal('three', @")
  call assert_equal(9, col('.'))
  call assert_equal(9, col("'<"))
  call assert_equal(13, col("'>"))

  " Right click inside the selection closer to the end of the selection
  call test_setmouse(1, 11)
  exe "normal 5|vee\<RightMouse>ly"
  call assert_equal('two thre', @")
  call assert_equal(5, col('.'))
  call assert_equal(5, col("'<"))
  call assert_equal(12, col("'>"))

  " Multi-line selection. Right click inside the selection.
  call setline(1, repeat(['aaaaaa'], 7))
  call test_setmouse(3, 1)
  exe "normal ggVG\<RightMouse>y"
  call assert_equal(3, line("'<"))
  call test_setmouse(5, 1)
  exe "normal ggVG\<RightMouse>y"
  call assert_equal(5, line("'>"))

  " Click right in the middle line of the selection
  call test_setmouse(4, 3)
  exe "normal ggVG$\<RightMouse>y"
  call assert_equal(4, line("'<"))
  call test_setmouse(4, 4)
  exe "normal ggVG$\<RightMouse>y"
  call assert_equal(4, line("'>"))

  set mousemodel&
  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  close!
endfunc

" Test for displaying the popup menu using the right mouse click
func Test_term_mouse_popup_menu()
  CheckFeature menu
  new
  call setline(1, 'popup menu test')
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  let save_mousemodel = &mousemodel
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm mousemodel=popup
  call WaitForResponses()

  menu PopUp.foo :let g:menustr = 'foo'<CR>
  menu PopUp.bar :let g:menustr = 'bar'<CR>
  menu PopUp.baz :let g:menustr = 'baz'<CR>

  for ttymouse_val in g:Ttymouse_values
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    let g:menustr = ''
    call feedkeys(MouseRightClickCode(1, 4)
		\ .. MouseRightReleaseCode(1, 4) .. "\<Down>\<Down>\<CR>", "x")
    call assert_equal('bar', g:menustr, msg)
  endfor

  unmenu PopUp
  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  let &mousemodel = save_mousemodel
  call test_override('no_query_mouse', 0)
  close!
endfunc

" Test for 'mousemodel' set to popup_setpos to move the cursor where the popup
" menu is displayed.
func Test_term_mouse_popup_menu_setpos()
  CheckFeature menu
  5new
  call setline(1, ['the dish ran away with the spoon',
        \ 'the cow jumped over the moon' ])
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  let save_mousemodel = &mousemodel
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm mousemodel=popup_setpos
  call WaitForResponses()

  nmenu PopUp.foo :let g:menustr = 'foo'<CR>
  nmenu PopUp.bar :let g:menustr = 'bar'<CR>
  nmenu PopUp.baz :let g:menustr = 'baz'<CR>
  vmenu PopUp.foo y:<C-U>let g:menustr = 'foo'<CR>
  vmenu PopUp.bar y:<C-U>let g:menustr = 'bar'<CR>
  vmenu PopUp.baz y:<C-U>let g:menustr = 'baz'<CR>

  for ttymouse_val in g:Ttymouse_values
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    let g:menustr = ''
    call cursor(1, 1)
    call feedkeys(MouseRightClickCode(1, 5)
		\ .. MouseRightReleaseCode(1, 5) .. "\<Down>\<Down>\<CR>", "x")
    call assert_equal('bar', g:menustr, msg)
    call assert_equal([1, 5], [line('.'), col('.')], msg)

    " Test for right click in visual mode inside the selection
    let @" = ''
    call cursor(1, 10)
    call feedkeys('vee' .. MouseRightClickCode(1, 12)
		\ .. MouseRightReleaseCode(1, 12) .. "\<Down>\<CR>", "x")
    call assert_equal([1, 10], [line('.'), col('.')], msg)
    call assert_equal('ran away', @", msg)

    " Test for right click in visual mode right before the selection
    let @" = ''
    call cursor(1, 10)
    call feedkeys('vee' .. MouseRightClickCode(1, 9)
		\ .. MouseRightReleaseCode(1, 9) .. "\<Down>\<CR>", "x")
    call assert_equal([1, 9], [line('.'), col('.')], msg)
    call assert_equal('', @", msg)

    " Test for right click in visual mode right after the selection
    let @" = ''
    call cursor(1, 10)
    call feedkeys('vee' .. MouseRightClickCode(1, 18)
		\ .. MouseRightReleaseCode(1, 18) .. "\<Down>\<CR>", "x")
    call assert_equal([1, 18], [line('.'), col('.')], msg)
    call assert_equal('', @", msg)

    " Test for right click in block-wise visual mode inside the selection
    let @" = ''
    call cursor(1, 16)
    call feedkeys("\<C-V>j3l" .. MouseRightClickCode(2, 17)
		\ .. MouseRightReleaseCode(2, 17) .. "\<Down>\<CR>", "x")
    call assert_equal([1, 16], [line('.'), col('.')], msg)
    call assert_equal("\<C-V>4", getregtype('"'), msg)

    " Test for right click in block-wise visual mode outside the selection
    let @" = ''
    call cursor(1, 16)
    call feedkeys("\<C-V>j3l" .. MouseRightClickCode(2, 2)
		\ .. MouseRightReleaseCode(2, 2) .. "\<Down>\<CR>", "x")
    call assert_equal([2, 2], [line('.'), col('.')], msg)
    call assert_equal('v', getregtype('"'), msg)
    call assert_equal('', @", msg)

    " Test for right click in line-wise visual mode inside the selection
    let @" = ''
    call cursor(1, 16)
    call feedkeys("V" .. MouseRightClickCode(1, 10)
		\ .. MouseRightReleaseCode(1, 10) .. "\<Down>\<CR>", "x")
    call assert_equal([1, 1], [line('.'), col('.')], msg) " After yanking, the cursor goes to 1,1
    call assert_equal("V", getregtype('"'), msg)
    call assert_equal(1, len(getreg('"', 1, v:true)), msg)

    " Test for right click in multi-line line-wise visual mode inside the selection
    let @" = ''
    call cursor(1, 16)
    call feedkeys("Vj" .. MouseRightClickCode(2, 20)
		\ .. MouseRightReleaseCode(2, 20) .. "\<Down>\<CR>", "x")
    call assert_equal([1, 1], [line('.'), col('.')], msg) " After yanking, the cursor goes to 1,1
    call assert_equal("V", getregtype('"'), msg)
    call assert_equal(2, len(getreg('"', 1, v:true)), msg)

    " Test for right click in line-wise visual mode outside the selection
    let @" = ''
    call cursor(1, 16)
    call feedkeys("V" .. MouseRightClickCode(2, 10)
		\ .. MouseRightReleaseCode(2, 10) .. "\<Down>\<CR>", "x")
    call assert_equal([2, 10], [line('.'), col('.')], msg)
    call assert_equal("", @", msg)

    " Try clicking on the status line
    let @" = ''
    call cursor(1, 10)
    call feedkeys('vee' .. MouseRightClickCode(6, 2)
		\ .. MouseRightReleaseCode(6, 2) .. "\<Down>\<CR>", "x")
    call assert_equal([1, 10], [line('.'), col('.')], msg)
    call assert_equal('ran away', @", msg)

    " Try clicking outside the window
    let @" = ''
    call cursor(2, 2)
    call feedkeys('vee' .. MouseRightClickCode(7, 2)
		\ .. MouseRightReleaseCode(7, 2) .. "\<Down>\<CR>", "x")
    call assert_equal(2, winnr(), msg)
    call assert_equal('', @", msg)
    wincmd w
  endfor

  unmenu PopUp
  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  let &mousemodel = save_mousemodel
  call test_override('no_query_mouse', 0)
  close!
endfunc

" Test for searching for the word under the cursor using Shift-Right or
" Shift-Left click.
func Test_term_mouse_search()
  new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm ttymouse=xterm2
  set mousemodel=
  call WaitForResponses()

  " In normal mode, Shift-Left or Shift-Right click should search for the word
  " under the cursor.
  call setline(1, ['one two three four', 'four three two one'])
  call test_setmouse(1, 4)
  exe "normal \<S-LeftMouse>"
  call assert_equal([2, 12], [line('.'), col('.')])
  call test_setmouse(2, 16)
  exe "normal \<S-RightMouse>"
  call assert_equal([1, 1], [line('.'), col('.')])

  " In visual mode, Shift-Left or Shift-Right click should search for the word
  " under the cursor and extend the selection.
  call test_setmouse(1, 4)
  exe "normal 4|ve\<S-LeftMouse>y"
  call assert_equal([2, 12], [line("'>"), col("'>")])
  call test_setmouse(2, 16)
  exe "normal 2G16|ve\<S-RightMouse>y"
  call assert_equal([1, 1], [line("'<"), col("'<")])

  set mousemodel&
  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  close!
endfunc

" Test for selecting an entry in the quickfix/location list window using the
" mouse.
func Test_term_mouse_quickfix_window()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm ttymouse=xterm2
  set mousemodel=
  call WaitForResponses()

  cgetexpr "Xfile1:1:L1"
  copen 5
  call test_setmouse(&lines - 7, 1)
  exe "normal \<2-LeftMouse>"
  call assert_equal('Xfile1', @%)
  %bw!

  lgetexpr "Xfile2:1:L1"
  lopen 5
  call test_setmouse(&lines - 7, 1)
  exe "normal \<2-LeftMouse>"
  call assert_equal('Xfile2', @%)
  %bw!

  set mousemodel&
  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
endfunc

" Test for the 'h' flag in the 'mouse' option. Using mouse in the help window.
func Test_term_mouse_help_window()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=h term=xterm mousetime=200
  call WaitForResponses()

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val
    help
    let @" = ''
    call MouseLeftClick(2, 5)
    call MouseLeftRelease(2, 5)
    call MouseLeftClick(1, 1)
    call MouseLeftDrag(1, 10)
    call MouseLeftRelease(1, 10)
    norm! y
    call assert_equal('*help.txt*', @", msg)
    helpclose

    " Click somewhere else to make sure the left click above is not combined
    " with the next left click and treated as a double click
    call MouseRightClick(5, 10)
    call MouseRightRelease(5, 10)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousetime&
  call test_override('no_query_mouse', 0)
  %bw!
endfunc

" Test for the translation of various mouse terminal codes
func Test_mouse_termcodes()
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm mousetime=200
  call WaitForResponses()

  new
  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec + g:Ttymouse_netterm
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val

    let mouse_codes = [
          \ ["\<LeftMouse>", "<LeftMouse>"],
          \ ["\<MiddleMouse>", "<MiddleMouse>"],
          \ ["\<RightMouse>", "<RightMouse>"],
          \ ["\<S-LeftMouse>", "<S-LeftMouse>"],
          \ ["\<S-MiddleMouse>", "<S-MiddleMouse>"],
          \ ["\<S-RightMouse>", "<S-RightMouse>"],
          \ ["\<C-LeftMouse>", "<C-LeftMouse>"],
          \ ["\<C-MiddleMouse>", "<C-MiddleMouse>"],
          \ ["\<C-RightMouse>", "<C-RightMouse>"],
          \ ["\<M-LeftMouse>", "<M-LeftMouse>"],
          \ ["\<M-MiddleMouse>", "<M-MiddleMouse>"],
          \ ["\<M-RightMouse>", "<M-RightMouse>"],
          \ ["\<2-LeftMouse>", "<2-LeftMouse>"],
          \ ["\<2-MiddleMouse>", "<2-MiddleMouse>"],
          \ ["\<2-RightMouse>", "<2-RightMouse>"],
          \ ["\<3-LeftMouse>", "<3-LeftMouse>"],
          \ ["\<3-MiddleMouse>", "<3-MiddleMouse>"],
          \ ["\<3-RightMouse>", "<3-RightMouse>"],
          \ ["\<4-LeftMouse>", "<4-LeftMouse>"],
          \ ["\<4-MiddleMouse>", "<4-MiddleMouse>"],
          \ ["\<4-RightMouse>", "<4-RightMouse>"],
          \ ["\<LeftDrag>", "<LeftDrag>"],
          \ ["\<MiddleDrag>", "<MiddleDrag>"],
          \ ["\<RightDrag>", "<RightDrag>"],
          \ ["\<LeftRelease>", "<LeftRelease>"],
          \ ["\<MiddleRelease>", "<MiddleRelease>"],
          \ ["\<RightRelease>", "<RightRelease>"],
          \ ["\<ScrollWheelUp>", "<ScrollWheelUp>"],
          \ ["\<S-ScrollWheelUp>", "<S-ScrollWheelUp>"],
          \ ["\<C-ScrollWheelUp>", "<C-ScrollWheelUp>"],
          \ ["\<ScrollWheelDown>", "<ScrollWheelDown>"],
          \ ["\<S-ScrollWheelDown>", "<S-ScrollWheelDown>"],
          \ ["\<C-ScrollWheelDown>", "<C-ScrollWheelDown>"],
          \ ["\<ScrollWheelLeft>", "<ScrollWheelLeft>"],
          \ ["\<S-ScrollWheelLeft>", "<S-ScrollWheelLeft>"],
          \ ["\<C-ScrollWheelLeft>", "<C-ScrollWheelLeft>"],
          \ ["\<ScrollWheelRight>", "<ScrollWheelRight>"],
          \ ["\<S-ScrollWheelRight>", "<S-ScrollWheelRight>"],
          \ ["\<C-ScrollWheelRight>", "<C-ScrollWheelRight>"]
          \ ]

    for [code, outstr] in mouse_codes
      exe "normal ggC\<C-K>" . code
      call assert_equal(outstr, getline(1), msg)
    endfor
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousetime&
  call test_override('no_query_mouse', 0)
  %bw!
endfunc

" This only checks if the sequence is recognized.
func Test_term_rgb_response()
  set t_RF=x
  set t_RB=y

  " response to t_RF, 4 digits
  let red = 0x12
  let green = 0x34
  let blue = 0x56
  let seq = printf("\<Esc>]10;rgb:%02x00/%02x00/%02x00\x07", red, green, blue)
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termrfgresp)

  " response to t_RF, 2 digits
  let red = 0x78
  let green = 0x9a
  let blue = 0xbc
  let seq = printf("\<Esc>]10;rgb:%02x/%02x/%02x\x07", red, green, blue)
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termrfgresp)

  " response to t_RB, 4 digits, dark
  set background=light
  eval 'background'->test_option_not_set()
  let red = 0x29
  let green = 0x4a
  let blue = 0x6b
  let seq = printf("\<Esc>]11;rgb:%02x00/%02x00/%02x00\x07", red, green, blue)
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termrbgresp)
  call assert_equal('dark', &background)

  " response to t_RB, 4 digits, light
  set background=dark
  call test_option_not_set('background')
  let red = 0x81
  let green = 0x63
  let blue = 0x65
  let seq = printf("\<Esc>]11;rgb:%02x00/%02x00/%02x00\x07", red, green, blue)
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termrbgresp)
  call assert_equal('light', &background)

  " response to t_RB, 2 digits, dark
  set background=light
  call test_option_not_set('background')
  let red = 0x47
  let green = 0x59
  let blue = 0x5b
  let seq = printf("\<Esc>]11;rgb:%02x/%02x/%02x\x07", red, green, blue)
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termrbgresp)
  call assert_equal('dark', &background)

  " response to t_RB, 2 digits, light
  set background=dark
  call test_option_not_set('background')
  let red = 0x83
  let green = 0xa4
  let blue = 0xc2
  let seq = printf("\<Esc>]11;rgb:%02x/%02x/%02x\x07", red, green, blue)
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termrbgresp)
  call assert_equal('light', &background)

  set t_RF= t_RB=
endfunc

" This only checks if the sequence is recognized.
" This must be after other tests, because it has side effects to xterm
" properties.
func Test_xx01_term_style_response()
  " Termresponse is only parsed when t_RV is not empty.
  set t_RV=x
  call test_override('term_props', 1)

  " send the termresponse to trigger requesting the XT codes
  let seq = "\<Esc>[>41;337;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)

  let seq = "\<Esc>P1$r2 q\<Esc>\\"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termstyleresp)

  call assert_equal(#{
        \ cursor_style: 'u',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'u',
        \ mouse: 's',
        \ kitty: 'u',
        \ }, terminalprops())

  set t_RV=
  call test_override('term_props', 0)
endfunc

" This checks the iTerm2 version response.
" This must be after other tests, because it has side effects to xterm
" properties.
func Test_xx02_iTerm2_response()
  " Termresponse is only parsed when t_RV is not empty.
  set t_RV=x
  call test_override('term_props', 1)

  " Old versions of iTerm2 used a different style term response.
  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>0;95;c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('xterm', &ttymouse)

  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>0;95;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('sgr', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'n',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'u',
        \ mouse: 's',
        \ kitty: 'u',
        \ }, terminalprops())

  set t_RV=
  call test_override('term_props', 0)
endfunc

func Run_libvterm_konsole_response(code)
  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>0;" .. a:code .. ";0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('sgr', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'n',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'u',
        \ mouse: 's',
        \ kitty: 'u',
        \ }, terminalprops())
endfunc

" This checks the libvterm version response.
" This must be after other tests, because it has side effects to xterm
" properties.
func Test_xx03_libvterm_konsole_response()
  " Termresponse is only parsed when t_RV is not empty.
  set t_RV=x
  call test_override('term_props', 1)

  " libvterm
  call Run_libvterm_konsole_response(100)
  " Konsole
  call Run_libvterm_konsole_response(115)

  set t_RV=
  call test_override('term_props', 0)
endfunc

" This checks the Mac Terminal.app version response.
" This must be after other tests, because it has side effects to xterm
" properties.
func Test_xx04_Mac_Terminal_response()
  " Termresponse is only parsed when t_RV is not empty.
  set t_RV=x
  call test_override('term_props', 1)

  set ttymouse=xterm
  " t_8u is not reset
  let &t_8u = "\<Esc>[58;2;%lu;%lu;%lum"
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>1;95;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('sgr', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'n',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'y',
        \ mouse: 's',
        \ kitty: 'u',
        \ }, terminalprops())
  call assert_equal("\<Esc>[58;2;%lu;%lu;%lum", &t_8u)

  " Reset is_not_xterm and is_mac_terminal.
  set t_RV=
  set term=xterm
  set t_RV=x
  call test_override('term_props', 0)
endfunc

" This checks the mintty version response.
" This must be after other tests, because it has side effects to xterm
" properties.
func Test_xx05_mintty_response()
  " Termresponse is only parsed when t_RV is not empty.
  set t_RV=x
  call test_override('term_props', 1)

  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>77;20905;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('sgr', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'n',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'y',
        \ mouse: 's',
        \ kitty: 'u',
        \ }, terminalprops())

  set t_RV=
  call test_override('term_props', 0)
endfunc

" This checks the screen version response.
" This must be after other tests, because it has side effects to xterm
" properties.
func Test_xx06_screen_response()
  " Termresponse is only parsed when t_RV is not empty.
  set t_RV=x
  call test_override('term_props', 1)

  " Old versions of screen don't support SGR mouse mode.
  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>83;40500;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('xterm', &ttymouse)

  " screen supports SGR mouse mode starting in version 4.7.
  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>83;40700;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('sgr', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'n',
        \ cursor_blink_mode: 'n',
        \ underline_rgb: 'y',
        \ mouse: 's',
        \ kitty: 'u',
        \ }, terminalprops())

  set t_RV=
  call test_override('term_props', 0)
endfunc

func Do_check_t_8u_set_reset(set_by_user)
  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let default_value = "\<Esc>[58;2;%lu;%lu;%lum"
  let &t_8u = default_value
  if !a:set_by_user
    call test_option_not_set('t_8u')
  endif
  let seq = "\<Esc>[>0;279;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('sgr', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'u',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'u',
        \ mouse: 's',
        \ kitty: 'u',
        \ }, terminalprops())
  call assert_equal(a:set_by_user ? default_value : '', &t_8u)
endfunc

" This checks the xterm version response.
" This must be after other tests, because it has side effects to xterm
" properties.
func Test_xx07_xterm_response()
  " Termresponse is only parsed when t_RV is not empty.
  set t_RV=x
  call test_override('term_props', 1)

  " Do Terminal.app first to check that is_mac_terminal is reset.
  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>1;95;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('sgr', &ttymouse)

  " xterm < 95: "xterm" (actually unmodified)
  set t_RV=
  set term=xterm
  call WaitForResponses()

  set t_RV=x
  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>0;94;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('xterm', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'n',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'y',
        \ mouse: 'u',
        \ kitty: 'u',
        \ }, terminalprops())

  " xterm >= 95 < 277 "xterm2"
  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>0;267;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('xterm2', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'n',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'u',
        \ mouse: '2',
        \ kitty: 'u',
        \ }, terminalprops())

  " xterm >= 277: "sgr"
  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>0;277;0c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('sgr', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'n',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'u',
        \ mouse: 's',
        \ kitty: 'u',
        \ }, terminalprops())

  " xterm >= 279: "sgr" and cursor_style not reset; also check t_8u reset,
  " except when it was set by the user
  call Do_check_t_8u_set_reset(0)
  call Do_check_t_8u_set_reset(1)

  set t_RV=
  call test_override('term_props', 0)
endfunc

func Test_xx08_kitty_response()
  " Termresponse is only parsed when t_RV is not empty.
  set t_RV=x
  call test_override('term_props', 1)

  set ttymouse=xterm
  call test_option_not_set('ttymouse')
  let seq = "\<Esc>[>1;4001;12c"
  call feedkeys(seq, 'Lx!')
  call assert_equal(seq, v:termresponse)
  call assert_equal('sgr', &ttymouse)

  call assert_equal(#{
        \ cursor_style: 'u',
        \ cursor_blink_mode: 'u',
        \ underline_rgb: 'y',
        \ mouse: 's',
        \ kitty: 'y',
        \ }, terminalprops())

  set t_RV=
  call test_override('term_props', 0)
endfunc

func Test_focus_events()
  let save_term = &term
  let save_ttymouse = &ttymouse
  set term=xterm ttymouse=xterm2
  call WaitForResponses()

  au FocusGained * let g:focus_gained += 1
  au FocusLost * let g:focus_lost += 1
  let g:focus_gained = 0
  let g:focus_lost = 0

  call feedkeys("\<Esc>[O", "Lx!")
  call assert_equal(1, g:focus_lost)
  call feedkeys("\<Esc>[I", "Lx!")
  call assert_equal(1, g:focus_gained)

  " still works when 'ttymouse' is empty
  set ttymouse=
  call feedkeys("\<Esc>[O", "Lx!")
  call assert_equal(2, g:focus_lost)
  call feedkeys("\<Esc>[I", "Lx!")
  call assert_equal(2, g:focus_gained)

  au! FocusGained
  au! FocusLost
  let &term = save_term
  let &ttymouse = save_ttymouse
endfunc

func Test_get_termcode()
  try
    let k1 = &t_k1
  catch /E113/
    throw 'Skipped: Unable to query termcodes'
  endtry
  set t_k1=
  set t_k1&
  call assert_equal(k1, &t_k1)

  " use external termcap first
  set nottybuiltin
  set t_k1=
  set t_k1&
  " when using external termcap may get something else, but it must not be
  " empty, since we would fallback to the builtin one.
  call assert_notequal('', &t_k1)

  if &term =~ 'xterm'
    " use internal termcap first
    let term_save = &term
    let &term = 'builtin_' .. &term
    set t_k1=
    set t_k1&
    call assert_equal(k1, &t_k1)
    let &term = term_save
  endif

  set ttybuiltin
endfunc

func Test_list_builtin_terminals()
  CheckRunVimInTerminal

  call RunVimInTerminal('', #{rows: 14})
  call term_sendkeys('', ":set cmdheight=3\<CR>")
  call TermWait('', 100)
  call term_sendkeys('', ":set term=xxx\<CR>")
  call TermWait('', 100)

  " Check that the list ends in "builtin_dumb" and "builtin_debug".
  let dumb_idx = 0
  for n in range(8, 12)
    if term_getline('', n) =~ 'builtin_dumb'
      let dumb_idx = n
      break
    endif
  endfor
  call assert_notequal(0, dumb_idx, 'builtin_dumb not found')

  call assert_match('builtin_dumb', term_getline('', dumb_idx))
  call assert_match('builtin_debug', term_getline('', dumb_idx + 1))
  call assert_match('Not found in termcap', term_getline('', dumb_idx + 2))

  call StopVimInTerminal('')
endfunc

" This checks the CSI sequences when in modifyOtherKeys mode.
" The mode doesn't need to be enabled, the codes are always detected.
func RunTest_modifyOtherKeys(func)
  new
  set timeoutlen=10

  " Shift-X is sent as 'X' with the shift modifier
  call feedkeys('a' .. a:func('X', 2) .. "\<Esc>", 'Lx!')
  call assert_equal('X', getline(1))

  " Ctrl-i is Tab
  call setline(1, '')
  call feedkeys('a' .. a:func('i', 5) .. "\<Esc>", 'Lx!')
  call assert_equal("\t", getline(1))

  " Ctrl-I is also Tab
  call setline(1, '')
  call feedkeys('a' .. a:func('I', 5) .. "\<Esc>", 'Lx!')
  call assert_equal("\t", getline(1))

  " Alt-x is ø
  call setline(1, '')
  call feedkeys('a' .. a:func('x', 3) .. "\<Esc>", 'Lx!')
  call assert_equal("ø", getline(1))

  " Meta-x is also ø
  call setline(1, '')
  call feedkeys('a' .. a:func('x', 9) .. "\<Esc>", 'Lx!')
  call assert_equal("ø", getline(1))

  " Alt-X is Ø
  call setline(1, '')
  call feedkeys('a' .. a:func('X', 3) .. "\<Esc>", 'Lx!')
  call assert_equal("Ø", getline(1))

  " Meta-X is ø
  call setline(1, '')
  call feedkeys('a' .. a:func('X', 9) .. "\<Esc>", 'Lx!')
  call assert_equal("Ø", getline(1))

  " Ctrl-6 is Ctrl-^
  split aaa
  edit bbb
  call feedkeys(a:func('6', 5), 'Lx!')
  call assert_equal("aaa", bufname())
  bwipe aaa
  bwipe bbb

  " Ctrl-V X 33 is 3
  call setline(1, '')
  call feedkeys("a\<C-V>" .. a:func('X', 2) .. "33\<Esc>", 'Lx!')
  call assert_equal("3", getline(1))

  " Ctrl-V U 12345 is Unicode 12345
  call setline(1, '')
  call feedkeys("a\<C-V>" .. a:func('U', 2) .. "12345\<Esc>", 'Lx!')
  call assert_equal("\U12345", getline(1))

  bwipe!
  set timeoutlen&
endfunc

func Test_modifyOtherKeys_basic()
  call RunTest_modifyOtherKeys(function('GetEscCodeCSI27'))
  call RunTest_modifyOtherKeys(function('GetEscCodeCSIu'))
endfunc

func Test_modifyOtherKeys_no_mapping()
  set timeoutlen=10

  let @a = 'aaa'
  call feedkeys(":let x = '" .. GetEscCodeCSI27('R', 5) .. GetEscCodeCSI27('R', 5) .. "a'\<CR>", 'Lx!')
  call assert_equal("let x = 'aaa'", @:)

  new
  call feedkeys("a" .. GetEscCodeCSI27('R', 5) .. GetEscCodeCSI27('R', 5) .. "a\<Esc>", 'Lx!')
  call assert_equal("aaa", getline(1))
  bwipe!

  new
  call feedkeys("axx\<CR>yy" .. GetEscCodeCSI27('G', 5) .. GetEscCodeCSI27('K', 5) .. "a\<Esc>", 'Lx!')
  call assert_equal("axx", getline(1))
  call assert_equal("yy", getline(2))
  bwipe!

  set timeoutlen&
endfunc

func Test_CSIu_keys_without_modifiers()
  " make this execute faster
  set timeoutlen=10

  call WaitForResponses()

  " Escape sent as `CSI 27 u` should act as normal escape and not undo
  call setline(1, 'a')
  call feedkeys('a' .. GetEscCodeCSIuWithoutModifier("\e"), 'Lx!')
  call assert_equal('n', mode())
  call assert_equal('a', getline(1))

  " Tab sent as `CSI 9 u` should work
  call setline(1, '')
  call feedkeys('a' .. GetEscCodeCSIuWithoutModifier("\t") .. "\<Esc>", 'Lx!')
  call assert_equal("\t", getline(1))

  set timeoutlen&
endfunc

" Check that when DEC mouse codes are recognized a special key is handled.
func Test_ignore_dec_mouse()
  silent !infocmp gnome >/dev/null 2>&1
  if v:shell_error != 0
    throw 'Skipped: gnome entry missing in the terminfo db'
  endif

  new
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=gnome ttymouse=
  call WaitForResponses()

  execute "set <xF1>=\<Esc>[1;*P"
  nnoremap <S-F1> agot it<Esc>
  call feedkeys("\<Esc>[1;2P", 'Lx!')
  call assert_equal('got it', getline(1))

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  call test_override('no_query_mouse', 0)
  bwipe!
endfunc

func RunTest_mapping_shift(key, func)
  call setline(1, '')
  if a:key == '|'
    exe 'inoremap \| xyz'
  else
    exe 'inoremap ' .. a:key .. ' xyz'
  endif
  call feedkeys('a' .. a:func(a:key, 2) .. "\<Esc>", 'Lx!')
  call assert_equal("xyz", getline(1))
  if a:key == '|'
    exe 'iunmap \|'
  else
    exe 'iunmap ' .. a:key
  endif
endfunc

func Test_modifyOtherKeys_mapped()
  set timeoutlen=10
  imap ' <C-W>
  imap <C-W><C-A> c-a
  call setline(1, '')

  " single quote is turned into single byte CTRL-W
  " CTRL-A is added with a separate modifier, and needs to be simplified before
  " the mapping can match.
  call feedkeys("a'" .. GetEscCodeCSI27('A', 5) .. "\<Esc>", 'Lx!')
  call assert_equal('c-a', getline(1))

  iunmap '
  iunmap <C-W><C-A>
  set timeoutlen&
endfunc

func Test_modifyOtherKeys_ambiguous_mapping()
  new
  set timeoutlen=10
  map <C-J> a
  map <C-J>x <Nop>
  call setline(1, 'x')

  " CTRL-J b should have trigger the <C-J> mapping and then insert "b"
  call feedkeys(GetEscCodeCSI27('J', 5) .. "b\<Esc>", 'Lx!')
  call assert_equal('xb', getline(1))

  unmap <C-J>
  unmap <C-J>x

  " if a special character is following there should be a check for a termcode
  nnoremap s aX<Esc>
  nnoremap s<BS> aY<Esc>
  set t_kb=
  call setline(1, 'x')
  call feedkeys("s\x08", 'Lx!')
  call assert_equal('xY', getline(1))

  set timeoutlen&
  bwipe!
endfunc

" Whether Shift-Tab sends "ESC [ Z" or "ESC [ 27 ; 2 ; 9 ~" is unpredictable,
" both should work.
func Test_modifyOtherKeys_shift_tab()
  set timeoutlen=10

  call setline(1, '')
  call feedkeys("a\<C-K>" .. GetEscCodeCSI27("\t", '2') .. "\<Esc>", 'Lx!')
  eval getline(1)->assert_equal('<S-Tab>')

  call setline(1, '')
  call feedkeys("a\<C-K>\<Esc>[Z\<Esc>", 'Lx!')
  eval getline(1)->assert_equal('<S-Tab>')

  set timeoutlen&
  bwipe!
endfunc

func RunTest_mapping_works_with_shift(func)
  new
  set timeoutlen=10

  call RunTest_mapping_shift('@', a:func)
  call RunTest_mapping_shift('A', a:func)
  call RunTest_mapping_shift('Z', a:func)
  call RunTest_mapping_shift('^', a:func)
  call RunTest_mapping_shift('_', a:func)
  call RunTest_mapping_shift('{', a:func)
  call RunTest_mapping_shift('|', a:func)
  call RunTest_mapping_shift('}', a:func)
  call RunTest_mapping_shift('~', a:func)

  bwipe!
  set timeoutlen&
endfunc

func Test_mapping_works_with_shift_plain()
  call RunTest_mapping_works_with_shift(function('GetEscCodeCSI27'))
  call RunTest_mapping_works_with_shift(function('GetEscCodeCSIu'))
endfunc

func RunTest_mapping_mods(map, key, func, code)
  call setline(1, '')
  exe 'inoremap ' .. a:map .. ' xyz'
  call feedkeys('a' .. a:func(a:key, a:code) .. "\<Esc>", 'Lx!')
  call assert_equal("xyz", getline(1))
  exe 'iunmap ' .. a:map
endfunc

func RunTest_mapping_works_with_mods(func, mods, code)
  new
  set timeoutlen=10

  if a:mods !~ 'S'
    " Shift by itself has no effect
    call RunTest_mapping_mods('<' .. a:mods .. '-@>', '@', a:func, a:code)
  endif
  call RunTest_mapping_mods('<' .. a:mods .. '-A>', 'A', a:func, a:code)
  call RunTest_mapping_mods('<' .. a:mods .. '-Z>', 'Z', a:func, a:code)
  if a:mods !~ 'S'
    " with Shift code is always upper case
    call RunTest_mapping_mods('<' .. a:mods .. '-a>', 'a', a:func, a:code)
    call RunTest_mapping_mods('<' .. a:mods .. '-z>', 'z', a:func, a:code)
  endif
  if a:mods != 'A'
    " with Alt code is not in upper case
    call RunTest_mapping_mods('<' .. a:mods .. '-a>', 'A', a:func, a:code)
    call RunTest_mapping_mods('<' .. a:mods .. '-z>', 'Z', a:func, a:code)
  endif
  call RunTest_mapping_mods('<' .. a:mods .. '-á>', 'á', a:func, a:code)
  if a:mods !~ 'S'
    " Shift by itself has no effect
    call RunTest_mapping_mods('<' .. a:mods .. '-^>', '^', a:func, a:code)
    call RunTest_mapping_mods('<' .. a:mods .. '-_>', '_', a:func, a:code)
    call RunTest_mapping_mods('<' .. a:mods .. '-{>', '{', a:func, a:code)
    call RunTest_mapping_mods('<' .. a:mods .. '-\|>', '|', a:func, a:code)
    call RunTest_mapping_mods('<' .. a:mods .. '-}>', '}', a:func, a:code)
    call RunTest_mapping_mods('<' .. a:mods .. '-~>', '~', a:func, a:code)
  endif

  bwipe!
  set timeoutlen&
endfunc

func Test_mapping_works_with_shift()
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSI27'), 'S', 2)
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSIu'), 'S', 2)
endfunc

func Test_mapping_works_with_ctrl()
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSI27'), 'C', 5)
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSIu'), 'C', 5)

  new
  set timeoutlen=10

  " CTRL-@ actually produces the code for CTRL-2, which is converted
  call RunTest_mapping_mods('<C-@>', '2', function('GetEscCodeCSI27'), 5)
  call RunTest_mapping_mods('<C-@>', '2', function('GetEscCodeCSIu'), 5)

  " CTRL-^ actually produces the code for CTRL-6, which is converted
  call RunTest_mapping_mods('<C-^>', '6', function('GetEscCodeCSI27'), 5)
  call RunTest_mapping_mods('<C-^>', '6', function('GetEscCodeCSIu'), 5)

  " CTRL-_ actually produces the code for CTRL--, which is converted
  call RunTest_mapping_mods('<C-_>', '-', function('GetEscCodeCSI27'), 5)
  call RunTest_mapping_mods('<C-_>', '-', function('GetEscCodeCSIu'), 5)

  bwipe!
  set timeoutlen&
endfunc

func Test_mapping_works_with_shift_ctrl()
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSI27'), 'C-S', 6)
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSIu'), 'C-S', 6)

  new
  set timeoutlen=10

  " Ctrl-Shift-[ actually produces CTRL-Shift-{ which is mapped as <C-{>
  call RunTest_mapping_mods('<C-{>', '{', function('GetEscCodeCSI27'), 6)
  call RunTest_mapping_mods('<C-{>', '{', function('GetEscCodeCSIu'), 6)

  " Ctrl-Shift-] actually produces CTRL-Shift-} which is mapped as <C-}>
  call RunTest_mapping_mods('<C-{>', '{', function('GetEscCodeCSI27'), 6)
  call RunTest_mapping_mods('<C-{>', '{', function('GetEscCodeCSIu'), 6)

  " Ctrl-Shift-\ actually produces CTRL-Shift-| which is mapped as <C-|>
  call RunTest_mapping_mods('<C-\|>', '|', function('GetEscCodeCSI27'), 6)
  call RunTest_mapping_mods('<C-\|>', '|', function('GetEscCodeCSIu'), 6)

  bwipe!
  set timeoutlen&
endfunc

" Below we also test the "u" code with Alt, This works, but libvterm would not
" send the Alt key like this but by prefixing an Esc.

func Test_mapping_works_with_alt()
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSI27'), 'A', 3)
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSIu'), 'A', 3)
endfunc

func Test_mapping_works_with_shift_alt()
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSI27'), 'S-A', 4)
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSIu'), 'S-A', 4)
endfunc

func Test_mapping_works_with_alt_and_shift()
  new
  set timeoutlen=10

  " mapping <A-?> works even though the code is A-S-?
  for c in ['!', '$', '+', ':', '?', '^', '~']
    call RunTest_mapping_mods('<A-' .. c .. '>', c, function('GetEscCodeCSI27'), 4)
    call RunTest_mapping_mods('<A-' .. c .. '>', c, function('GetEscCodeCSIu'), 4)
  endfor

  bwipe!
  set timeoutlen&
endfunc

func Test_mapping_works_with_ctrl_alt()
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSI27'), 'C-A', 7)
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSIu'), 'C-A', 7)
endfunc

func Test_mapping_works_with_shift_ctrl_alt()
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSI27'), 'C-S-A', 8)
  call RunTest_mapping_works_with_mods(function('GetEscCodeCSIu'), 'C-S-A', 8)
endfunc

func Test_mapping_works_with_unknown_modifiers()
  new
  set timeoutlen=10

  for Func in [function('GetEscCodeCSI27'), function('GetEscCodeCSIu')]
    call RunTest_mapping_mods('<C-z>', 'z', Func, 5)
    " Add 16, 32, 64 or 128 for modifiers we currently don't support.
    call RunTest_mapping_mods('<C-z>', 'z', Func, 5 + 16)
    call RunTest_mapping_mods('<C-z>', 'z', Func, 5 + 32)
    call RunTest_mapping_mods('<C-z>', 'z', Func, 5 + 64)
    call RunTest_mapping_mods('<C-z>', 'z', Func, 5 + 128)

    call RunTest_mapping_mods('<S-X>', 'X', Func, 2)
    " Add 16, 32, 64 or 128 for modifiers we currently don't support.
    call RunTest_mapping_mods('<S-X>', 'X', Func, 2 + 16)
    call RunTest_mapping_mods('<S-X>', 'X', Func, 2 + 32)
    call RunTest_mapping_mods('<S-X>', 'X', Func, 2 + 64)
    call RunTest_mapping_mods('<S-X>', 'X', Func, 2 + 128)
  endfor

  bwipe!
  set timeoutlen&
endfunc

func RunTest_mapping_funckey(map, func, key, code)
  call setline(1, '')
  exe 'inoremap ' .. a:map .. ' xyz'
  call feedkeys('a' .. a:func(a:key, a:code) .. "\<Esc>", 'Lx!')
  call assert_equal("xyz", getline(1), 'mapping ' .. a:map)
  exe 'iunmap ' .. a:map
endfunc

func Test_mapping_kitty_function_keys()
  new
  set timeoutlen=10

  " Function keys made with CSI and ending in [ABCDEFHPQRS].
  " 'E' is keypad BEGIN, not supported
  let maps = [
        \    ['<Up>', 'A', 0],
        \    ['<S-Up>', 'A', 2],
        \    ['<C-Up>', 'A', 5],
        \    ['<C-S-Up>', 'A', 6],
        \
        \    ['<Down>', 'B', 0],
        \    ['<S-Down>', 'B', 2],
        \    ['<C-Down>', 'B', 5],
        \    ['<C-S-Down>', 'B', 6],
        \
        \    ['<Right>', 'C', 0],
        \    ['<S-Right>', 'C', 2],
        \    ['<C-Right>', 'C', 5],
        \    ['<C-S-Right>', 'C', 6],
        \
        \    ['<Left>', 'D', 0],
        \    ['<S-Left>', 'D', 2],
        \    ['<C-Left>', 'D', 5],
        \    ['<C-S-Left>', 'D', 6],
        \
        \    ['<End>', 'F', 0],
        \    ['<S-End>', 'F', 2],
        \    ['<C-End>', 'F', 5],
        \    ['<C-S-End>', 'F', 6],
        \
        \    ['<Home>', 'H', 0],
        \    ['<S-Home>', 'H', 2],
        \    ['<C-Home>', 'H', 5],
        \    ['<C-S-Home>', 'H', 6],
        \
        \    ['<F1>', 'P', 0],
        \    ['<S-F1>', 'P', 2],
        \    ['<C-F1>', 'P', 5],
        \    ['<C-S-F1>', 'P', 6],
        \
        \    ['<F2>', 'Q', 0],
        \    ['<S-F2>', 'Q', 2],
        \    ['<C-F2>', 'Q', 5],
        \    ['<C-S-F2>', 'Q', 6],
        \
        \    ['<F3>', 'R', 0],
        \    ['<S-F3>', 'R', 2],
        \    ['<C-F3>', 'R', 5],
        \    ['<C-S-F3>', 'R', 6],
        \
        \    ['<F4>', 'S', 0],
        \    ['<S-F4>', 'S', 2],
        \    ['<C-F4>', 'S', 5],
        \    ['<C-S-F4>', 'S', 6],
        \ ]

  for map in maps
    call RunTest_mapping_funckey(map[0], function('GetEscCodeFunckey'), map[1], map[2])
  endfor

  bwipe!
  set timeoutlen&
endfunc

func Test_insert_literal()
  set timeoutlen=10

  call WaitForResponses()

  new
  " CTRL-V CTRL-X inserts a ^X
  call feedkeys('a' .. GetEscCodeCSIu('V', '5') .. GetEscCodeCSIu('X', '5') .. "\<Esc>", 'Lx!')
  call assert_equal("\<C-X>", getline(1))

  call setline(1, '')
  call feedkeys('a' .. GetEscCodeCSI27('V', '5') .. GetEscCodeCSI27('X', '5') .. "\<Esc>", 'Lx!')
  call assert_equal("\<C-X>", getline(1))

  " CTRL-SHIFT-V CTRL-X inserts escape sequence
  call setline(1, '')
  call feedkeys('a' .. GetEscCodeCSIu('V', '6') .. GetEscCodeCSIu('X', '5') .. "\<Esc>", 'Lx!')
  call assert_equal("\<Esc>[88;5u", getline(1))

  call setline(1, '')
  call feedkeys('a' .. GetEscCodeCSI27('V', '6') .. GetEscCodeCSI27('X', '5') .. "\<Esc>", 'Lx!')
  call assert_equal("\<Esc>[27;5;88~", getline(1))

  bwipe!
  set timeoutlen&
endfunc

func Test_cmdline_literal()
  set timeoutlen=10

  " CTRL-V CTRL-Y inserts a ^Y
  call feedkeys(':' .. GetEscCodeCSIu('V', '5') .. GetEscCodeCSIu('Y', '5') .. "\<C-B>\"\<CR>", 'Lx!')
  call assert_equal("\"\<C-Y>", @:)

  call feedkeys(':' .. GetEscCodeCSI27('V', '5') .. GetEscCodeCSI27('Y', '5') .. "\<C-B>\"\<CR>", 'Lx!')
  call assert_equal("\"\<C-Y>", @:)

  " CTRL-SHIFT-V CTRL-Y inserts escape sequence
  call feedkeys(':' .. GetEscCodeCSIu('V', '6') .. GetEscCodeCSIu('Y', '5') .. "\<C-B>\"\<CR>", 'Lx!')
  call assert_equal("\"\<Esc>[89;5u", @:)

  call setline(1, '')
  call feedkeys(':' .. GetEscCodeCSI27('V', '6') .. GetEscCodeCSI27('Y', '5') .. "\<C-B>\"\<CR>", 'Lx!')
  call assert_equal("\"\<Esc>[27;5;89~", @:)

  set timeoutlen&
endfunc

func Test_mapping_esc()
  set timeoutlen=10

  new
  nnoremap <Up> iHello<Esc>
  nnoremap <Esc> <Nop>

  call feedkeys(substitute(&t_ku, '\*', '', 'g'), 'Lx!')
  call assert_equal("Hello", getline(1))

  bwipe!
  nunmap <Up>
  nunmap <Esc>
  set timeoutlen&
endfunc

" Test for translation of special key codes (<xF1>, <xF2>, etc.)
func Test_Keycode_Translation()
  let keycodes = [
        \ ["<xUp>", "<Up>"],
        \ ["<xDown>", "<Down>"],
        \ ["<xLeft>", "<Left>"],
        \ ["<xRight>", "<Right>"],
        \ ["<xHome>", "<Home>"],
        \ ["<xEnd>", "<End>"],
        \ ["<zHome>", "<Home>"],
        \ ["<zEnd>", "<End>"],
        \ ["<xF1>", "<F1>"],
        \ ["<xF2>", "<F2>"],
        \ ["<xF3>", "<F3>"],
        \ ["<xF4>", "<F4>"],
        \ ["<S-xF1>", "<S-F1>"],
        \ ["<S-xF2>", "<S-F2>"],
        \ ["<S-xF3>", "<S-F3>"],
        \ ["<S-xF4>", "<S-F4>"]]
  for [k1, k2] in keycodes
    exe "nnoremap " .. k1 .. " 2wx"
    call assert_true(maparg(k1, 'n', 0, 1).lhs == k2)
    exe "nunmap " .. k1
  endfor
endfunc

" Test for terminal keycodes that doesn't have termcap entries
func Test_special_term_keycodes()
  new
  " Test for <xHome>, <S-xHome> and <C-xHome>
  " send <K_SPECIAL> <KS_EXTRA> keycode
  call feedkeys("i\<C-K>\x80\xfd\x3f\n", 'xt')
  " send <K_SPECIAL> <KS_MODIFIER> bitmap <K_SPECIAL> <KS_EXTRA> keycode
  call feedkeys("i\<C-K>\x80\xfc\x2\x80\xfd\x3f\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfc\x4\x80\xfd\x3f\n", 'xt')
  " Test for <xEnd>, <S-xEnd> and <C-xEnd>
  call feedkeys("i\<C-K>\x80\xfd\x3d\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfc\x2\x80\xfd\x3d\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfc\x4\x80\xfd\x3d\n", 'xt')
  " Test for <zHome>, <S-zHome> and <C-zHome>
  call feedkeys("i\<C-K>\x80\xfd\x40\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfc\x2\x80\xfd\x40\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfc\x4\x80\xfd\x40\n", 'xt')
  " Test for <zEnd>, <S-zEnd> and <C-zEnd>
  call feedkeys("i\<C-K>\x80\xfd\x3e\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfc\x2\x80\xfd\x3e\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfc\x4\x80\xfd\x3e\n", 'xt')
  " Test for <xUp>, <xDown>, <xLeft> and <xRight>
  call feedkeys("i\<C-K>\x80\xfd\x41\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfd\x42\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfd\x43\n", 'xt')
  call feedkeys("i\<C-K>\x80\xfd\x44\n", 'xt')
  call assert_equal(['<Home>', '<S-Home>', '<C-Home>',
        \ '<End>', '<S-End>', '<C-End>',
        \ '<Home>', '<S-Home>', '<C-Home>',
        \ '<End>', '<S-End>', '<C-End>',
        \ '<Up>', '<Down>', '<Left>', '<Right>', ''], getline(1, '$'))
  bw!
endfunc

func Test_home_key_works()
  " The '@' character in K_HOME must only match "1" when followed by ";",
  " otherwise this code for Home is not recognized: "<Esc>[1~"
  " Set termcap values like "xterm" uses them.  Except using F2 for xHome,
  " because that termcap entry can't be set here.
  let save_K1 = exists('&t_K1') ? &t_K1 : ''
  let save_kh = exists('&t_kh') ? &t_kh : ''
  let save_k2 = exists('&t_k2') ? &t_k2 : ''
  let save_k3 = exists('&t_k3') ? &t_k3 : ''
  let save_end = exists('&t_@7') ? &t_@7 : ''

  let &t_K1 = "\<Esc>[1;*~"      " <kHome>
  let &t_kh = "\<Esc>[@;*H"      " <Home>
  let &t_k2 = "\<Esc>O*H"        " use <F2> for <xHome>
  let &t_k3 = "\<Esc>[7;*~"      " use <F3> for <zHome>
  let &t_@7 = "\<Esc>[@;*F"      " <End>

  new
  call feedkeys("i\<C-K>\<Esc>OH\n\<Esc>", 'tx')
  call feedkeys("i\<C-K>\<Esc>[1~\n\<Esc>", 'tx')
  call assert_equal([
        \ '<F2>',
        \ '<kHome>',
        \ ''], getline(1, '$'))

  bwipe!
  let &t_K1 = save_K1
  let &t_kh = save_kh
  let &t_k2 = save_k2
  let &t_k3 = save_k3
  let &t_@7 = save_end
endfunc

func Test_terminal_builtin_without_gui()
  CheckNotMSWindows

  " builtin_gui should not be output by :set term=xxx
  let output = systemlist("TERM=dumb " .. v:progpath .. " --clean -c ':set t_ti= t_te=' -c 'set term=xxx' -c ':q!'")
  redraw!
  call map(output, {_, val -> trim(val)})
  call assert_equal(-1, index(output, 'builtin_gui'))
  call assert_notequal(-1, index(output, 'builtin_dumb'))
endfunc


" vim: shiftwidth=2 sts=2 expandtab
