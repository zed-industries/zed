" Tests for popup windows

source check.vim
CheckFeature popupwin

source screendump.vim
source term_util.vim

func Test_simple_popup()
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 100))
	hi PopupColor1 ctermbg=lightblue
	hi PopupColor2 ctermbg=lightcyan
	hi EndOfBuffer ctermbg=lightgrey
	hi Comment ctermfg=red
	call prop_type_add('comment', #{highlight: 'Comment'})
	let winid = popup_create('hello there', #{line: 3, col: 11, minwidth: 20, highlight: 'PopupColor1'})
	let winid2 = popup_create(['another one', 'another two', 'another three'], #{line: 3, col: 25, minwidth: 20})
	call setwinvar(winid2, '&wincolor', 'PopupColor2')
  END
  call writefile(lines, 'XtestPopup', 'D')
  let buf = RunVimInTerminal('-S XtestPopup', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_01', {})

  " Add a tabpage
  call term_sendkeys(buf, ":tabnew\<CR>")
  call term_sendkeys(buf, ":let popupwin = popup_create(["
	\ .. "#{text: 'other tab'},"
	\ .. "#{text: 'a comment line', props: [#{"
	\ .. "col: 3, length: 7, minwidth: 20, type: 'comment'"
	\ .. "}]},"
	\ .. "], #{line: 4, col: 9, minwidth: 20})\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_02', {})

  " switch back to first tabpage
  call term_sendkeys(buf, "gt")
  call VerifyScreenDump(buf, 'Test_popupwin_03', {})

  " close that tabpage
  call term_sendkeys(buf, ":quit!\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_04', {})

  " set 'columns' to a small value, size must be recomputed
  call term_sendkeys(buf, ":let cols = &columns\<CR>")
  call term_sendkeys(buf, ":set columns=12\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_04a', {})
  call term_sendkeys(buf, ":let &columns = cols\<CR>")

  " resize popup, show empty line at bottom
  call term_sendkeys(buf, ":call popup_move(popupwin, #{minwidth: 15, maxwidth: 25, minheight: 3, maxheight: 5})\<CR>")
  call term_sendkeys(buf, ":redraw\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_05', {})

  " show not fitting line at bottom
  call term_sendkeys(buf, ":call setbufline(winbufnr(popupwin), 3, 'this line will not fit here')\<CR>")
  call term_sendkeys(buf, ":redraw\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_06', {})

  " move popup over ruler
  call term_sendkeys(buf, ":set cmdheight=2\<CR>")
  call term_sendkeys(buf, ":call popup_move(popupwin, #{line: 7, col: 55})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_07', {})

  " clear all popups after moving the cursor a bit, so that ruler is updated
  call term_sendkeys(buf, "axxx\<Esc>")
  call TermWait(buf)
  call term_sendkeys(buf, "0")
  call TermWait(buf)
  call term_sendkeys(buf, ":call popup_clear()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_08', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_with_border_and_padding()
  CheckScreendump

  for iter in range(0, 1)
    let lines =<< trim END
	  call setline(1, range(1, 100))
	  call popup_create('hello border', #{line: 2, col: 3, border: []})
	  call popup_create('hello padding', #{line: 2, col: 23, padding: []})
	  call popup_create('hello both', #{line: 2, col: 43, border: [], padding: [], highlight: 'Normal'})
	  call popup_create('border TL', #{line: 6, col: 3, border: [1, 0, 0, 4]})
	  call popup_create('paddings', #{line: 6, col: 23, padding: range(1, 4)})
	  call popup_create('wrapped longer text', #{line: 8, col: 55, padding: [0, 3, 0, 3], border: [0, 1, 0, 1]})
	  call popup_create('right aligned text', #{line: 11, col: 56, wrap: 0, padding: [0, 3, 0, 3], border: [0, 1, 0, 1]})
	  call popup_create('X', #{line: 2, col: 73})
	  call popup_create('X', #{line: 3, col: 74})
	  call popup_create('X', #{line: 4, col: 75})
	  call popup_create('X', #{line: 5, col: 76})
    END
    call insert(lines, iter == 1 ? '' : 'set enc=latin1')
    call writefile(lines, 'XtestPopupBorder', 'D')
    let buf = RunVimInTerminal('-S XtestPopupBorder', #{rows: 15})
    call VerifyScreenDump(buf, 'Test_popupwin_2' .. iter, {})

    call StopVimInTerminal(buf)
  endfor

  let lines =<< trim END
	call setline(1, range(1, 100))
	hi BlueColor ctermbg=lightblue
	hi TopColor ctermbg=253
	hi RightColor ctermbg=245
	hi BottomColor ctermbg=240
	hi LeftColor ctermbg=248
	call popup_create('hello border', #{line: 2, col: 3, border: [], borderhighlight: ['BlueColor']})
	call popup_create(['hello border', 'and more'], #{line: 2, col: 23, border: [], borderhighlight: ['TopColor', 'RightColor', 'BottomColor', 'LeftColor']})
	call popup_create(['hello border', 'lines only'], #{line: 2, col: 43, border: [], borderhighlight: ['BlueColor'], borderchars: ['x']})
	call popup_create(['hello border', 'with corners'], #{line: 2, col: 60, border: [], borderhighlight: ['BlueColor'], borderchars: ['x', '#']})
	let winid = popup_create(['hello border', 'with numbers'], #{line: 6, col: 3, border: [], borderhighlight: ['BlueColor'], borderchars: ['0', '1', '2', '3', '4', '5', '6', '7']})
	call popup_create(['hello border', 'just blanks'], #{line: 7, col: 23, border: [], borderhighlight: ['BlueColor'], borderchars: [' ']})
	func MultiByte()
	  call popup_create(['hello'], #{line: 8, col: 43, border: [], borderchars: ['─', '│', '─', '│', '┌', '┐', '┘', '└']})
	endfunc
  END
  call writefile(lines, 'XtestPopupBorder', 'D')
  let buf = RunVimInTerminal('-S XtestPopupBorder', #{rows: 12})
  call VerifyScreenDump(buf, 'Test_popupwin_22', {})

  " check that changing borderchars triggers a redraw
  call term_sendkeys(buf, ":call popup_setoptions(winid, #{borderchars: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']})\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_23', {})

  " check multi-byte border only with 'ambiwidth' single
  if &ambiwidth == 'single'
    call term_sendkeys(buf, ":call MultiByte()\<CR>")
    call VerifyScreenDump(buf, 'Test_popupwin_24', {})
  endif

  call StopVimInTerminal(buf)

  let with_border_or_padding = #{
	\ line: 2,
	\ core_line: 3,
	\ col: 3,
	\ core_col: 4,
	\ width: 14,
	\ core_width: 12,
	\ height: 3,
	\ core_height: 1,
	\ firstline: 1,
	\ lastline: 1,
	\ scrollbar: 0,
	\ visible: 1}
  let winid = popup_create('hello border', #{line: 2, col: 3, border: []})",
  call assert_equal(with_border_or_padding, winid->popup_getpos())
  let options = popup_getoptions(winid)
  call assert_equal([], options.border)
  call assert_false(has_key(options, "padding"))

  let winid = popup_create('hello padding', #{line: 2, col: 3, padding: []})
  let with_border_or_padding.width = 15
  let with_border_or_padding.core_width = 13
  call assert_equal(with_border_or_padding, popup_getpos(winid))
  let options = popup_getoptions(winid)
  call assert_false(has_key(options, "border"))
  call assert_equal([], options.padding)

  call popup_setoptions(winid, #{
	\ padding: [1, 2, 3, 4],
	\ border: [4, 0, 7, 8],
	\ borderhighlight: ['Top', 'Right', 'Bottom', 'Left'],
	\ borderchars: ['1', '^', '2', '>', '3', 'v', '4', '<'],
	\ })
  let options = popup_getoptions(winid)
  call assert_equal([1, 0, 1, 1], options.border)
  call assert_equal([1, 2, 3, 4], options.padding)
  call assert_equal(['Top', 'Right', 'Bottom', 'Left'], options.borderhighlight)
  call assert_equal(['1', '^', '2', '>', '3', 'v', '4', '<'], options.borderchars)

  " Check that popup_setoptions() takes the output of popup_getoptions()
  call popup_setoptions(winid, options)
  call assert_equal(options, popup_getoptions(winid))

  " Check that range() doesn't crash
  call popup_setoptions(winid, #{
	\ padding: range(1, 4),
	\ border: range(5, 8),
	\ borderhighlight: range(4),
	\ borderchars: range(8),
	\ })

  let winid = popup_create('hello both', #{line: 3, col: 8, border: [], padding: []})
  call assert_equal(#{
	\ line: 3,
	\ core_line: 5,
	\ col: 8,
	\ core_col: 10,
	\ width: 14,
	\ core_width: 10,
	\ height: 5,
	\ scrollbar: 0,
	\ core_height: 1,
	\ firstline: 1,
	\ lastline: 1,
	\ visible: 1}, popup_getpos(winid))

  call popup_clear()
endfunc

func Test_popup_with_syntax_win_execute()
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 100))
	hi PopupColor ctermbg=lightblue
	let winid = popup_create([
	    \ '#include <stdio.h>',
	    \ 'int main(void)',
	    \ '{',
	    \ '    printf(123);',
	    \ '}',
	    \], #{line: 3, col: 25, highlight: 'PopupColor'})
	call win_execute(winid, 'set syntax=cpp')
  END
  call writefile(lines, 'XtestPopup', 'D')
  let buf = RunVimInTerminal('-S XtestPopup', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_10', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_with_syntax_setbufvar()
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 100))
	hi PopupColor ctermbg=lightgrey
	let winid = popup_create([
	    \ '#include <stdio.h>',
	    \ 'int main(void)',
	    \ '{',
	    \ "\tprintf(567);",
	    \ '}',
	    \], #{line: 3, col: 21, highlight: 'PopupColor'})
	call setbufvar(winbufnr(winid), '&syntax', 'cpp')
  END
  call writefile(lines, 'XtestPopup', 'D')
  let buf = RunVimInTerminal('-S XtestPopup', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_11', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_with_matches()
  CheckScreendump

  let lines =<< trim END
	call setline(1, ['111 222 333', '444 555 666'])
	let winid = popup_create([
	    \ '111 222 333',
	    \ '444 555 666',
	    \], #{line: 3, col: 10, border: []})
	set hlsearch
	hi VeryBlue ctermfg=blue guifg=blue
	/666
	call matchadd('ErrorMsg', '111')
	call matchadd('VeryBlue', '444')
	call win_execute(winid, "call matchadd('ErrorMsg', '111')")
	call win_execute(winid, "call matchadd('VeryBlue', '555')")
  END
  call writefile(lines, 'XtestPopupMatches', 'D')
  let buf = RunVimInTerminal('-S XtestPopupMatches', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_matches', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_all_corners()
  CheckScreendump

  let lines =<< trim END
	call setline(1, repeat([repeat('-', 60)], 15))
	set so=0
	normal 2G3|r#
	let winid1 = popup_create(['first', 'second'], #{
	      \ line: 'cursor+1',
	      \ col: 'cursor',
	      \ pos: 'topleft',
	      \ border: [],
	      \ padding: [],
	      \ })
	normal 24|r@
	let winid1 = popup_create(['First', 'SeconD'], #{
	      \ line: 'cursor+1',
	      \ col: 'cursor',
	      \ pos: 'topright',
	      \ border: [],
	      \ padding: [],
	      \ })
	normal 9G27|r%
	let winid1 = popup_create(['fiRSt', 'seCOnd'], #{
	      \ line: 'cursor-1',
	      \ col: 'cursor',
	      \ pos: 'botleft',
	      \ border: [],
	      \ padding: [],
	      \ })
	normal 48|r&
	let winid1 = popup_create(['FIrsT', 'SEcoND'], #{
	      \ line: 'cursor-1',
	      \ col: 'cursor',
	      \ pos: 'botright',
	      \ border: [],
	      \ padding: [],
	      \ })
	normal 1G51|r*
	let winid1 = popup_create(['one', 'two'], #{
	      \ line: 'cursor-1',
	      \ col: 'cursor',
	      \ pos: 'botleft',
	      \ border: [],
	      \ padding: [],
	      \ })
  END
  call writefile(lines, 'XtestPopupCorners', 'D')
  let buf = RunVimInTerminal('-S XtestPopupCorners', #{rows: 12})
  call VerifyScreenDump(buf, 'Test_popupwin_corners', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_nospace()
  CheckScreendump

  let lines =<< trim END
	call setline(1, repeat([repeat('-', 60)], 15))
	set so=0

	" cursor in a line in top half, using "botleft" with popup that
	" does fit
	normal 5G2|r@
	let winid1 = popup_create(['one', 'two'], #{
	      \ line: 'cursor-1',
	      \ col: 'cursor',
	      \ pos: 'botleft',
	      \ border: [],
	      \ })
	" cursor in a line in top half, using "botleft" with popup that
	" doesn't fit: gets truncated
	normal 5G9|r#
	let winid1 = popup_create(['one', 'two', 'tee'], #{
	      \ line: 'cursor-1',
	      \ col: 'cursor',
	      \ pos: 'botleft',
	      \ posinvert: 0,
	      \ border: [],
	      \ })
	" cursor in a line in top half, using "botleft" with popup that
	" doesn't fit and 'posinvert' set: flips to below.
	normal 5G16|r%
	let winid1 = popup_create(['one', 'two', 'tee'], #{
	      \ line: 'cursor-1',
	      \ col: 'cursor',
	      \ pos: 'botleft',
	      \ border: [],
	      \ })
	" cursor in a line in bottom half, using "botleft" with popup that
	" doesn't fit: does not flip.
	normal 8G23|r*
	let winid1 = popup_create(['aaa', 'bbb', 'ccc', 'ddd', 'eee', 'fff'], #{
	      \ line: 'cursor-1',
	      \ col: 'cursor',
	      \ pos: 'botleft',
	      \ border: [],
	      \ })

	" cursor in a line in bottom half, using "topleft" with popup that
	" does fit
	normal 8G30|r@
	let winid1 = popup_create(['one', 'two'], #{
	      \ line: 'cursor+1',
	      \ col: 'cursor',
	      \ pos: 'topleft',
	      \ border: [],
	      \ })
	" cursor in a line in top half, using "topleft" with popup that
	" doesn't fit: truncated
	normal 8G37|r#
	let winid1 = popup_create(['one', 'two', 'tee'], #{
	      \ line: 'cursor+1',
	      \ col: 'cursor',
	      \ pos: 'topleft',
	      \ posinvert: 0,
	      \ border: [],
	      \ })
	" cursor in a line in top half, using "topleft" with popup that
	" doesn't fit and "posinvert" set: flips to above.
	normal 8G44|r%
	let winid1 = popup_create(['one', 'two', 'tee', 'fou', 'fiv'], #{
	      \ line: 'cursor+1',
	      \ col: 'cursor',
	      \ pos: 'topleft',
	      \ border: [],
	      \ })
	" cursor in a line in top half, using "topleft" with popup that
	" doesn't fit: does not flip.
	normal 5G51|r*
	let winid1 = popup_create(['aaa', 'bbb', 'ccc', 'ddd', 'eee', 'fff'], #{
	      \ line: 'cursor+1',
	      \ col: 'cursor',
	      \ pos: 'topleft',
	      \ border: [],
	      \ })
  END
  call writefile(lines, 'XtestPopupNospace', 'D')
  let buf = RunVimInTerminal('-S XtestPopupNospace', #{rows: 12})
  call VerifyScreenDump(buf, 'Test_popupwin_nospace', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_firstline_dump()
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 20))
	let winid = popup_create(['1111', '222222', '33333', '44', '5', '666666', '77777', '888', '9999999999999999'], #{
	      \ maxheight: 4,
	      \ firstline: 3,
	      \ })
  END
  call writefile(lines, 'XtestPopupFirstline', 'D')
  let buf = RunVimInTerminal('-S XtestPopupFirstline', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_firstline_1', {})

  call term_sendkeys(buf, ":call popup_setoptions(winid, #{firstline: -1})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_firstline_2', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_firstline()
  let winid = popup_create(['1111', '222222', '33333', '44444'], #{
	\ maxheight: 2,
	\ firstline: 3,
	\ })
  call assert_equal(3, popup_getoptions(winid).firstline)
  call popup_setoptions(winid, #{firstline: 1})
  call assert_equal(1, popup_getoptions(winid).firstline)
  eval winid->popup_close()

  let winid = popup_create(['xxx']->repeat(50), #{
	\ maxheight: 3,
	\ firstline: 11,
	\ })
  redraw
  call assert_equal(11, popup_getoptions(winid).firstline)
  call assert_equal(11, popup_getpos(winid).firstline)
  " check line() works with popup window
  call assert_equal(11, line('.', winid))
  call assert_equal(50, line('$', winid))
  call assert_equal(0, line('$', 123456))

  " Normal command changes what is displayed but not "firstline"
  call win_execute(winid, "normal! \<c-y>")
  call assert_equal(11, popup_getoptions(winid).firstline)
  call assert_equal(10, popup_getpos(winid).firstline)

  " Making some property change applies "firstline" again
  call popup_setoptions(winid, #{line: 4})
  call assert_equal(11, popup_getoptions(winid).firstline)
  call assert_equal(11, popup_getpos(winid).firstline)

  " Remove "firstline" property and scroll
  call popup_setoptions(winid, #{firstline: 0})
  call win_execute(winid, "normal! \<c-y>")
  call assert_equal(0, popup_getoptions(winid).firstline)
  call assert_equal(10, popup_getpos(winid).firstline)

  " Making some property change has no side effect
  call popup_setoptions(winid, #{line: 3})
  call assert_equal(0, popup_getoptions(winid).firstline)
  call assert_equal(10, popup_getpos(winid).firstline)
  call popup_close(winid)

  " CTRL-D scrolls down half a page
  let winid = popup_create(['xxx']->repeat(50), #{
	\ maxheight: 8,
	\ })
  redraw
  call assert_equal(1, popup_getpos(winid).firstline)
  call win_execute(winid, "normal! \<C-D>")
  call assert_equal(5, popup_getpos(winid).firstline)
  call win_execute(winid, "normal! \<C-D>")
  call assert_equal(9, popup_getpos(winid).firstline)
  call win_execute(winid, "normal! \<C-U>")
  call assert_equal(5, popup_getpos(winid).firstline)

  call win_execute(winid, "normal! \<C-F>")
  call assert_equal(11, popup_getpos(winid).firstline)
  call win_execute(winid, "normal! \<C-B>")
  call assert_equal(5, popup_getpos(winid).firstline)

  call popup_close(winid)

  " Popup with less elements than the maximum height and negative firstline:
  " check that the popup height is correctly computed.
  let winid = popup_create(['xxx']->repeat(4), #{
        \ firstline: -1,
        \ maxheight: 6,
	\ })

  let pos = popup_getpos(winid)
  call assert_equal(3, pos.width)
  call assert_equal(4, pos.height)

  call popup_close(winid)
endfunc

func Test_popup_firstline_cursorline()
  let winid = popup_create(['1111', '222222', '33333', '44444'], #{
	\ maxheight: 2,
	\ firstline: 3,
	\ cursorline: 1,
	\ })
  call assert_equal(3, popup_getoptions(winid).firstline)
  call assert_equal(3, getwininfo(winid)[0].topline)
  call assert_equal(3, getcurpos(winid)[1])

  call popup_close(winid)
endfunc

func Test_popup_noscrolloff()
  set scrolloff=5
  let winid = popup_create(['xxx']->repeat(50), #{
	\ maxheight: 5,
	\ firstline: 11,
	\ })
  redraw
  call assert_equal(11, popup_getoptions(winid).firstline)
  call assert_equal(11, popup_getpos(winid).firstline)

  call popup_setoptions(winid, #{firstline: 0})
  call win_execute(winid, "normal! \<c-y>")
  call assert_equal(0, popup_getoptions(winid).firstline)
  call assert_equal(10, popup_getpos(winid).firstline)

  call popup_close(winid)
endfunc

func Test_popup_drag()
  CheckScreendump

  " create a popup that covers the command line
  let lines =<< trim END
	call setline(1, range(1, 20))
	split
	vsplit
	$wincmd w
	vsplit
	1wincmd w
	let winid = popup_create(['1111', '222222', '33333'], #{
	      \ drag: 1,
	      \ resize: 1,
	      \ border: [],
	      \ line: &lines - 4,
	      \ })
	func Dragit()
	  map <silent> <F3> :call test_setmouse(&lines - 4, &columns / 2)<CR>
	  map <silent> <F4> :call test_setmouse(&lines - 8, &columns / 2 - 20)<CR>
	  call feedkeys("\<F3>\<LeftMouse>\<F4>\<LeftDrag>\<LeftRelease>", "xt")
	endfunc
	func Resize()
	  map <silent> <F5> :call test_setmouse(6, 21)<CR>
	  map <silent> <F6> :call test_setmouse(7, 25)<CR>
	  call feedkeys("\<F5>\<LeftMouse>\<F6>\<LeftDrag>\<LeftRelease>", "xt")
	endfunc
	func ClickAndDrag()
	  map <silent> <F3> :call test_setmouse(5, 2)<CR>
	  map <silent> <F4> :call test_setmouse(3, 14)<CR>
	  map <silent> <F5> :call test_setmouse(3, 18)<CR>
	  call feedkeys("\<F3>\<LeftMouse>\<LeftRelease>", "xt")
	  call feedkeys("\<F4>\<LeftMouse>\<F5>\<LeftDrag>\<LeftRelease>", "xt")
	endfunc
	func DragAllStart()
	  call popup_clear()
	  call popup_create('hello', #{line: 3, col: 5, dragall: 1})
	endfunc
	func DragAllDrag()
	  map <silent> <F3> :call test_setmouse(3, 5)<CR>
	  map <silent> <F4> :call test_setmouse(5, 36)<CR>
	  call feedkeys("\<F3>\<LeftMouse>\<F4>\<LeftDrag>\<LeftRelease>", "xt")
	endfunc
  END
  call writefile(lines, 'XtestPopupDrag', 'D')
  let buf = RunVimInTerminal('-S XtestPopupDrag', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_drag_01', {})

  call term_sendkeys(buf, ":call Dragit()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_drag_02', {})

  call term_sendkeys(buf, ":call Resize()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_drag_03', {})

  " dragging works after click on a status line
  call term_sendkeys(buf, ":call ClickAndDrag()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_drag_04', {})

  " dragging without border
  call term_sendkeys(buf, ":call DragAllStart()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_drag_05', {})
  call term_sendkeys(buf, ":call DragAllDrag()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_drag_06', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_drag_minwidth()
  CheckScreendump

  " create a popup that does not fit
  let lines =<< trim END
      call range(40)
	      \ ->map({_,i -> string(i)})
	      \ ->popup_create({
	      \   'drag': 1,
	      \   'wrap': 0,
	      \   'border': [],
	      \   'scrollbar': 1,
	      \   'minwidth': 100,
	      \   'filter': {w, k -> k ==# 'q' ? len([popup_close(w)]) : 0},
	      \ })
	func DragitDown()
	  map <silent> <F3> :call test_setmouse(1, 10)<CR>
	  map <silent> <F4> :call test_setmouse(5, 40)<CR>
	  call feedkeys("\<F3>\<LeftMouse>\<F4>\<LeftDrag>\<LeftRelease>", "xt")
	endfunc
	func DragitUp()
	  map <silent> <F3> :call test_setmouse(5, 40)<CR>
	  map <silent> <F4> :call test_setmouse(4, 40)<CR>
	  map <silent> <F5> :call test_setmouse(3, 40)<CR>
	  call feedkeys("\<F3>\<LeftMouse>\<F4>\<LeftDrag>\<F5>\<LeftDrag>\<LeftRelease>", "xt")
	endfunc
  END
  call writefile(lines, 'XtestPopupDrag', 'D')
  let buf = RunVimInTerminal('-S XtestPopupDrag', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_drag_minwidth_1', {})

  call term_sendkeys(buf, ":call DragitDown()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_drag_minwidth_2', {})

  call term_sendkeys(buf, ":call DragitUp()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_drag_minwidth_3', {})

  call term_sendkeys(buf, 'q')

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_drag_termwin()
  CheckUnix
  CheckScreendump
  CheckFeature terminal

  " create a popup that covers the terminal window
  let lines =<< trim END
	set foldmethod=marker
	call setline(1, range(100))
	for nr in range(7)
	  call setline(nr * 12 + 1, "fold {{{")
	  call setline(nr * 12 + 11, "end }}}")
	endfor
	%foldclose
	set shell=/bin/sh noruler
	unlet $PROMPT_COMMAND
	let $PS1 = 'vim> '
        terminal ++rows=4
	$wincmd w
	let winid = popup_create(['1111', '2222'], #{
	      \ drag: 1,
	      \ resize: 1,
	      \ border: [],
	      \ line: 3,
	      \ })
	func DragitLeft()
	  call feedkeys("\<F3>\<LeftMouse>\<F4>\<LeftDrag>\<LeftRelease>", "xt")
	endfunc
	func DragitDown()
	  call feedkeys("\<F4>\<LeftMouse>\<F5>\<LeftDrag>\<LeftRelease>", "xt")
	endfunc
	func DragitDownLeft()
	  call feedkeys("\<F5>\<LeftMouse>\<F6>\<LeftDrag>\<LeftRelease>", "xt")
	endfunc
	map <silent> <F3> :call test_setmouse(3, &columns / 2)<CR>
	map <silent> <F4> :call test_setmouse(3, &columns / 2 - 20)<CR>
	map <silent> <F5> :call test_setmouse(12, &columns / 2)<CR>
	map <silent> <F6> :call test_setmouse(12, &columns / 2 - 20)<CR>
  END
  call writefile(lines, 'XtestPopupTerm', 'D')
  let buf = RunVimInTerminal('-S XtestPopupTerm', #{rows: 16})
  call VerifyScreenDump(buf, 'Test_popupwin_term_01', {})

  call term_sendkeys(buf, ":call DragitLeft()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_term_02', {})

  call term_sendkeys(buf, ":call DragitDown()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_term_03', {})

  call term_sendkeys(buf, ":call DragitDownLeft()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_term_04', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_close_with_mouse()
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 20))
	" With border, can click on X
	let winid = popup_create('foobar', #{
	      \ close: 'button',
	      \ border: [],
	      \ line: 1,
	      \ col: 1,
	      \ })
	func CloseMsg(id, result)
	  echomsg 'Popup closed with ' .. a:result
	endfunc
	let winid = popup_create('notification', #{
	      \ close: 'click',
	      \ line: 3,
	      \ col: 15,
	      \ callback: 'CloseMsg',
	      \ })
	let winid = popup_create('no border here', #{
	      \ close: 'button',
	      \ line: 5,
	      \ col: 3,
	      \ })
	let winid = popup_create('only padding', #{
	      \ close: 'button',
	      \ padding: [],
	      \ line: 5,
	      \ col: 23,
	      \ })
	func CloseWithX()
	  call feedkeys("\<F3>\<LeftMouse>\<LeftRelease>", "xt")
	endfunc
	map <silent> <F3> :call test_setmouse(1, len('foobar') + 2)<CR>
	func CloseWithClick()
	  call feedkeys("\<F4>\<LeftMouse>\<LeftRelease>", "xt")
	endfunc
	map <silent> <F4> :call test_setmouse(3, 17)<CR>
	func CreateWithMenuFilter()
	  let winid = popup_create('barfoo', #{
		\ close: 'button',
		\ filter: 'popup_filter_menu',
		\ border: [],
		\ line: 1,
		\ col: 40,
		\ })
	endfunc
  END
  call writefile(lines, 'XtestPopupClose', 'D')
  let buf = RunVimInTerminal('-S XtestPopupClose', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_close_01', {})

  call term_sendkeys(buf, ":call CloseWithX()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_close_02', {})

  call term_sendkeys(buf, ":call CloseWithClick()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_close_03', {})

  call term_sendkeys(buf, ":call CreateWithMenuFilter()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_close_04', {})

  " We have to send the actual mouse code, feedkeys() would be caught the
  " filter.
  call term_sendkeys(buf, "\<Esc>[<0;47;1M")
  call VerifyScreenDump(buf, 'Test_popupwin_close_05', {})

  " clean up
  call StopVimInTerminal(buf)
endfunction

func Test_popup_menu_wrap()
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 20))
	call popup_create([
	      \ 'one',
	      \ 'asdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfasdfas',
	      \ 'three',
	      \ 'four',
	      \ ], #{
	      \ pos: "botleft",
	      \ border: [],
	      \ padding: [0,1,0,1],
	      \ maxheight: 3,
	      \ cursorline: 1,
	      \ filter: 'popup_filter_menu',
	      \ })
  END
  call writefile(lines, 'XtestPopupWrap', 'D')
  let buf = RunVimInTerminal('-S XtestPopupWrap', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_wrap_1', {})

  call term_sendkeys(buf, "jj")
  call VerifyScreenDump(buf, 'Test_popupwin_wrap_2', {})

  " clean up
  call term_sendkeys(buf, "\<Esc>")
  call StopVimInTerminal(buf)
endfunction

func Test_popup_with_mask()
  CheckScreendump

  let lines =<< trim END
	call setline(1, repeat([join(range(1, 42), '')], 13))
	hi PopupColor ctermbg=lightgrey
	let winid = popup_create([
	    \ 'some text',
	    \ 'another line',
	    \], #{
	    \ line: 1,
	    \ col: 10,
	    \ posinvert: 0,
	    \ wrap: 0,
	    \ fixed: 1,
	    \ scrollbar: v:false,
	    \ zindex: 90,
	    \ padding: [],
	    \ highlight: 'PopupColor',
	    \ mask: [[1,1,1,1], [-5,-1,4,4], [7,9,2,3], [2,4,3,3]]})
	call popup_create([
	    \ 'xxxxxxxxx',
	    \ 'yyyyyyyyy',
	    \], #{
	    \ line: 3,
	    \ col: 18,
	    \ zindex: 20})
	let winidb = popup_create([
	    \ 'just one line',
	    \], #{
	    \ line: 7,
	    \ col: 10,
	    \ posinvert: 0,
	    \ wrap: 0,
	    \ fixed: 1,
	    \ scrollbar: v:false,
	    \ close: 'button',
	    \ zindex: 90,
	    \ padding: [],
	    \ border: [],
	    \ mask: [[1,2,1,1], [-5,-1,4,4], [7,9,2,3], [3,5,5,5],[-7,-4,5,5]]})
  END
  call writefile(lines, 'XtestPopupMask', 'D')
  let buf = RunVimInTerminal('-S XtestPopupMask', #{rows: 13})
  call VerifyScreenDump(buf, 'Test_popupwin_mask_1', {})

  call term_sendkeys(buf, ":call popup_move(winid, #{col: 11, line: 2})\<CR>")
  call term_sendkeys(buf, ":call popup_move(winidb, #{col: 12})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_mask_2', {})

  call term_sendkeys(buf, ":call popup_move(winid, #{col: 65, line: 2})\<CR>")
  call term_sendkeys(buf, ":call popup_move(winidb, #{col: 63})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_mask_3', {})

  call term_sendkeys(buf, ":call popup_move(winid, #{pos: 'topright', col: 12, line: 2})\<CR>")
  call term_sendkeys(buf, ":call popup_move(winidb, #{pos: 'topright', col: 12})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_mask_4', {})

  call term_sendkeys(buf, ":call popup_move(winid, #{pos: 'topright', col: 12, line: 11})\<CR>")
  call term_sendkeys(buf, ":call popup_move(winidb, #{pos: 'topleft', col: 42, line: 11})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_mask_5', {})

  " clean up
  call StopVimInTerminal(buf)

  " this was causing a crash
  call popup_create('test', #{mask: [[0, 0, 0, 0]]})
  call popup_clear()

  " this was causing an internal error
  enew
  set nowrap
  call repeat('x', &columns)->setline(1)
  call prop_type_add('textprop', {})
  call prop_add(1, 1, #{length: &columns, type: 'textprop'})
  vsplit
  let opts = popup_create('', #{textprop: 'textprop'})
	\ ->popup_getoptions()
	\ ->extend(#{mask: [[1, 1, 1, 1]]})
  call popup_create('', opts)
  redraw

  close!
  bwipe!
  call prop_type_delete('textprop')
  call popup_clear()
  set wrap&
endfunc

func Test_popup_select()
  CheckScreendump
  CheckFeature clipboard_working

  " create a popup with some text to be selected
  let lines =<< trim END
    set clipboard=autoselect
    call setline(1, range(1, 20))
    let winid = popup_create(['the word', 'some more', 'several words here', 'invisible', '5', '6', '7'], #{
	  \ drag: 1,
	  \ border: [],
	  \ line: 3,
	  \ col: 10,
	  \ maxheight: 3,
	  \ })
    func Select1()
      call feedkeys("\<F3>\<LeftMouse>\<F4>\<LeftDrag>\<LeftRelease>", "xt")
    endfunc
    map <silent> <F3> :call test_setmouse(4, 15)<CR>
    map <silent> <F4> :call test_setmouse(6, 23)<CR>
  END
  call writefile(lines, 'XtestPopupSelect', 'D')
  let buf = RunVimInTerminal('-S XtestPopupSelect', #{rows: 10})
  call term_sendkeys(buf, ":call Select1()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_select_01', {})

  call term_sendkeys(buf, ":call popup_close(winid)\<CR>")
  call term_sendkeys(buf, "\"*p")
  " clean the command line, sometimes it still shows a command
  call term_sendkeys(buf, ":\<esc>")

  call VerifyScreenDump(buf, 'Test_popupwin_select_02', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_in_tab()
  " default popup is local to tab, not visible when in other tab
  let winid = popup_create("text", {})
  let bufnr = winbufnr(winid)
  call assert_equal(1, popup_getpos(winid).visible)
  call assert_equal(0, popup_getoptions(winid).tabpage)
  tabnew
  call assert_equal(0, popup_getpos(winid).visible)
  call assert_equal(1, popup_getoptions(winid).tabpage)
  quit
  call assert_equal(1, popup_getpos(winid).visible)

  call assert_equal(1, bufexists(bufnr))
  call popup_clear()
  " buffer is gone now
  call assert_equal(0, bufexists(bufnr))

  " global popup is visible in any tab
  let winid = popup_create("text", #{tabpage: -1})
  call assert_equal(1, popup_getpos(winid).visible)
  call assert_equal(-1, popup_getoptions(winid).tabpage)
  tabnew
  call assert_equal(1, popup_getpos(winid).visible)
  call assert_equal(-1, popup_getoptions(winid).tabpage)
  quit
  call assert_equal(1, popup_getpos(winid).visible)
  call popup_clear()

  " create popup in other tab
  tabnew
  let winid = popup_create("text", #{tabpage: 1})
  call assert_equal(0, popup_getpos(winid).visible)
  call assert_equal(1, popup_getoptions(winid).tabpage)
  quit
  call assert_equal(1, popup_getpos(winid).visible)
  call assert_equal(0, popup_getoptions(winid).tabpage)
  call popup_clear()
endfunc

func Test_popup_valid_arguments()
  call assert_equal(0, len(popup_list()))

  " Zero value is like the property wasn't there
  let winid = popup_create("text", #{col: 0})
  let pos = popup_getpos(winid)
  call assert_inrange(&columns / 2 - 1, &columns / 2 + 1, pos.col)
  call assert_equal([winid], popup_list())
  call popup_clear()

  " using cursor column has minimum value of 1
  let winid = popup_create("text", #{col: 'cursor-100'})
  let pos = popup_getpos(winid)
  call assert_equal(1, pos.col)
  call popup_clear()

  " center
  let winid = popup_create("text", #{pos: 'center'})
  let pos = popup_getpos(winid)
  let around = (&columns - pos.width) / 2
  call assert_inrange(around - 1, around + 1, pos.col)
  let around = (&lines - pos.height) / 2
  call assert_inrange(around - 1, around + 1, pos.line)
  call popup_clear()
endfunc

func Test_popup_invalid_arguments()
  call assert_fails('call popup_create(666, {})', 'E86:')
  call popup_clear()
  call assert_fails('call popup_create("text", "none")', 'E1206:')
  call popup_clear()
  call assert_fails('call popup_create(test_null_string(), {})', 'E450:')
  call assert_fails('call popup_create(test_null_list(), {})', 'E450:')
  call popup_clear()

  call assert_fails('call popup_create("text", #{col: "xxx"})', 'E475:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{col: "cursor8"})', 'E15:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{col: "cursor+x"})', 'E15:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{col: "cursor+8x"})', 'E15:')
  call popup_clear()

  call assert_fails('call popup_create("text", #{line: "xxx"})', 'E475:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{line: "cursor8"})', 'E15:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{line: "cursor+x"})', 'E15:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{line: "cursor+8x"})', 'E15:')
  call popup_clear()

  call assert_fails('call popup_create("text", #{pos: "there"})', 'E475:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{padding: "none"})', 'E714:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{border: "none"})', 'E714:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{borderhighlight: "none"})', 'E714:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{borderhighlight: test_null_list()})', 'E714:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{borderchars: "none"})', 'E714:')
  call popup_clear()

  call assert_fails('call popup_create([#{text: "text"}, 666], {})', 'E1284: Argument 1, list item 2: Dictionary required')
  call popup_clear()
  call assert_fails('call popup_create([#{text: "text", props: "none"}], {})', 'E714:')
  call popup_clear()
  call assert_fails('call popup_create([#{text: "text", props: ["none"]}], {})', 'E715:')
  call popup_clear()
  call assert_fails('call popup_create([#{text: "text", props: range(3)}], {})', 'E715:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{mask: ["asdf"]})', 'E475:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{mask: range(5)})', 'E475:')
  call popup_clear()
  call popup_create("text", #{mask: [range(4)]})
  call popup_clear()
  call assert_fails('call popup_create("text", #{mask: test_null_list()})', 'E475:')
  call assert_fails('call popup_create("text", #{mapping: []})', 'E745:')
  call popup_clear()
  call assert_fails('call popup_create("text", #{tabpage : 4})', 'E997:')
  call popup_clear()

  call assert_fails('call popup_create(range(10), {})', 'E1024:')
  call popup_clear()
  call assert_fails('call popup_create([1, 2], {})', 'E1284: Argument 1, list item 1: Dictionary required')
  call popup_clear()
endfunc

func Test_win_execute_closing_curwin()
  split
  let winid = popup_create('some text', {})
  call assert_fails('call win_execute(winid, winnr() .. "close")', 'E994:')
  call popup_clear()

  let winid = popup_create('some text', {})
  call assert_fails('call win_execute(winid, printf("normal! :\<C-u>call popup_close(%d)\<CR>", winid))', 'E994:')
  call popup_clear()
endfunc

func Test_win_execute_not_allowed()
  let winid = popup_create('some text', {})
  call assert_fails('call win_execute(winid, "split")', 'E994:')
  call assert_fails('call win_execute(winid, "vsplit")', 'E994:')
  call assert_fails('call win_execute(winid, "close")', 'E994:')
  call assert_fails('call win_execute(winid, "bdelete")', 'E994:')
  call assert_fails('call win_execute(winid, "bwipe!")', 'E994:')
  call assert_fails('call win_execute(winid, "tabnew")', 'E994:')
  call assert_fails('call win_execute(winid, "tabnext")', 'E994:')
  call assert_fails('call win_execute(winid, "next")', 'E994:')
  call assert_fails('call win_execute(winid, "rewind")', 'E994:')
  call assert_fails('call win_execute(winid, "pedit filename")', 'E994:')
  call assert_fails('call win_execute(winid, "buf")', 'E994:')
  call assert_fails('call win_execute(winid, "bnext")', 'E994:')
  call assert_fails('call win_execute(winid, "bprev")', 'E994:')
  call assert_fails('call win_execute(winid, "bfirst")', 'E994:')
  call assert_fails('call win_execute(winid, "blast")', 'E994:')
  call assert_fails('call win_execute(winid, "edit")', 'E994:')
  call assert_fails('call win_execute(winid, "enew")', 'E994:')
  call assert_fails('call win_execute(winid, "help")', 'E994:')
  call assert_fails('call win_execute(winid, "1only")', 'E994:')
  call assert_fails('call win_execute(winid, "wincmd x")', 'E994:')
  call assert_fails('call win_execute(winid, "wincmd w")', 'E994:')
  call assert_fails('call win_execute(winid, "wincmd t")', 'E994:')
  call assert_fails('call win_execute(winid, "wincmd b")', 'E994:')
  call popup_clear()
endfunc

func Test_popup_with_wrap()
  CheckScreendump

  let lines =<< trim END
	 call setline(1, range(1, 100))
	 let winid = popup_create(
	   \ 'a long line that wont fit',
	   \ #{line: 3, col: 20, maxwidth: 10, wrap: 1})
  END
  call writefile(lines, 'XtestPopup', 'D')
  let buf = RunVimInTerminal('-S XtestPopup', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_wrap', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_without_wrap()
  CheckScreendump

  let lines =<< trim END
	 call setline(1, range(1, 100))
	 let winid = popup_create(
	   \ 'a long line that wont fit',
	   \ #{line: 3, col: 20, maxwidth: 10, wrap: 0})
  END
  call writefile(lines, 'XtestPopup', 'D')
  let buf = RunVimInTerminal('-S XtestPopup', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_nowrap', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_with_showbreak()
  CheckScreendump

  let lines =<< trim END
	 set showbreak=>>\ 
	 call setline(1, range(1, 20))
	 let winid = popup_dialog(
	   \ 'a long line here that wraps',
	   \ #{filter: 'popup_filter_yesno',
	   \   maxwidth: 12})
  END
  call writefile(lines, 'XtestPopupShowbreak', 'D')
  let buf = RunVimInTerminal('-S XtestPopupShowbreak', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_showbreak', {})

  " clean up
  call term_sendkeys(buf, "y")
  call StopVimInTerminal(buf)
endfunc

func Test_popup_time()
  CheckFeature timers

  topleft vnew
  call setline(1, 'hello')

  let winid = popup_create('world', #{
	\ line: 1,
	\ col: 1,
	\ minwidth: 20,
	\ time: 500,
	\})
  redraw
  let line = join(map(range(1, 5), 'screenstring(1, v:val)'), '')
  call assert_equal('world', line)

  call assert_equal(winid, popup_locate(1, 1))
  call assert_equal(winid, popup_locate(1, 20))
  call assert_equal(0, popup_locate(1, 21))
  call assert_equal(0, popup_locate(2, 1))

  " Mac is usually a bit slow
  let delay = has('mac') ? '900m' : '700m'
  exe 'sleep ' .. delay
  redraw

  let line = join(map(range(1, 5), '1->screenstring(v:val)'), '')
  call assert_equal('hello', line)

  call popup_create('on the command line', #{
	\ line: &lines,
	\ col: 10,
	\ minwidth: 20,
	\ time: 500,
	\})
  redraw
  let line = join(map(range(1, 30), 'screenstring(&lines, v:val)'), '')
  call assert_match('.*on the command line.*', line)

  exe 'sleep ' .. delay
  redraw
  let line = join(map(range(1, 30), 'screenstring(&lines, v:val)'), '')
  call assert_notmatch('.*on the command line.*', line)

  bwipe!
endfunc

func Test_popup_hide()
  topleft vnew
  call setline(1, 'hello')

  let winid = popup_create('world', #{
	\ line: 1,
	\ col: 1,
	\ minwidth: 20,
	\})
  redraw
  let line = join(map(range(1, 5), 'screenstring(1, v:val)'), '')
  call assert_equal('world', line)
  call assert_equal(1, popup_getpos(winid).visible)
  " buffer is still listed and active
  call assert_match(winbufnr(winid) .. 'u a.*\[Popup\]', execute('ls u'))

  call popup_hide(winid)
  redraw
  let line = join(map(range(1, 5), 'screenstring(1, v:val)'), '')
  call assert_equal('hello', line)
  call assert_equal(0, popup_getpos(winid).visible)
  " buffer is still listed but hidden
  call assert_match(winbufnr(winid) .. 'u a.*\[Popup\]', execute('ls u'))

  eval winid->popup_show()
  redraw
  let line = join(map(range(1, 5), 'screenstring(1, v:val)'), '')
  call assert_equal('world', line)
  call assert_equal(1, popup_getpos(winid).visible)


  call popup_close(winid)
  redraw
  let line = join(map(range(1, 5), 'screenstring(1, v:val)'), '')
  call assert_equal('hello', line)

  " error is given for existing non-popup window
  call assert_fails('call popup_hide(win_getid())', 'E993:')

  " no error non-existing window
  eval 1234234->popup_hide()
  call popup_show(41234234)

  bwipe!
endfunc

func Test_popup_move()
  topleft vnew
  call setline(1, 'hello')

  let winid = popup_create('world', #{
	\ line: 1,
	\ col: 1,
	\ minwidth: 20,
	\})
  redraw
  let line = join(map(range(1, 6), 'screenstring(1, v:val)'), '')
  call assert_equal('world ', line)

  call popup_move(winid, #{line: 2, col: 2})
  redraw
  let line = join(map(range(1, 6), 'screenstring(1, v:val)'), '')
  call assert_equal('hello ', line)
  let line = join(map(range(1, 6), 'screenstring(2, v:val)'), '')
  call assert_equal('~world', line)

  eval winid->popup_move(#{line: 1})
  redraw
  let line = join(map(range(1, 6), 'screenstring(1, v:val)'), '')
  call assert_equal('hworld', line)

  call assert_fails('call popup_move(winid, [])', 'E1206:')
  call assert_fails('call popup_move(winid, test_null_dict())', 'E1297:')

  call popup_close(winid)

  call assert_equal(0, popup_move(-1, {}))

  bwipe!
endfunc

func Test_popup_getpos()
  let winid = popup_create('hello', #{
    \ line: 2,
    \ col: 3,
    \ minwidth: 10,
    \ minheight: 11,
    \})
  redraw
  let res = popup_getpos(winid)
  call assert_equal(2, res.line)
  call assert_equal(3, res.col)
  call assert_equal(10, res.width)
  call assert_equal(11, res.height)
  call assert_equal(1, res.visible)

  call popup_close(winid)
endfunc

func Test_popup_width_longest()
  let tests = [
	\ [['hello', 'this', 'window', 'displays', 'all of its text'], 15],
	\ [['hello', 'this', 'window', 'all of its text', 'displays'], 15],
	\ [['hello', 'this', 'all of its text', 'window', 'displays'], 15],
	\ [['hello', 'all of its text', 'this', 'window', 'displays'], 15],
	\ [['all of its text', 'hello', 'this', 'window', 'displays'], 15],
	\ ]

  for test in tests
    let winid = popup_create(test[0], #{line: 2, col: 3})
    redraw
    let position = popup_getpos(winid)
    call assert_equal(test[1], position.width)
    call popup_close(winid)
  endfor
endfunc

func Test_popup_wraps()
  let tests = [
	\ ['nowrap', 6, 1],
	\ ['a line that wraps once', 12, 2],
	\ ['a line that wraps two times', 12, 3],
	\ ]
  for test in tests
    let winid = popup_create(test[0],
	  \ #{line: 2, col: 3, maxwidth: 12})
    redraw
    let position = popup_getpos(winid)
    call assert_equal(test[1], position.width)
    call assert_equal(test[2], position.height)

    call popup_close(winid)
    call assert_equal({}, popup_getpos(winid))
  endfor
endfunc

func Test_popup_getoptions()
  let winid = popup_create('hello', #{
    \ line: 2,
    \ col: 3,
    \ minwidth: 10,
    \ minheight: 11,
    \ maxwidth: 20,
    \ maxheight: 21,
    \ zindex: 100,
    \ time: 5000,
    \ fixed: 1
    \})
  redraw
  let res = popup_getoptions(winid)
  call assert_equal(2, res.line)
  call assert_equal(3, res.col)
  call assert_equal(10, res.minwidth)
  call assert_equal(11, res.minheight)
  call assert_equal(20, res.maxwidth)
  call assert_equal(21, res.maxheight)
  call assert_equal(100, res.zindex)
  call assert_equal(1, res.fixed)
  call assert_equal(1, res.mapping)
  if has('timers')
    call assert_equal(5000, res.time)
  endif
  call popup_close(winid)

  let winid = popup_create('hello', {})
  redraw
  let res = popup_getoptions(winid)
  call assert_equal(0, res.line)
  call assert_equal(0, res.col)
  call assert_equal(0, res.minwidth)
  call assert_equal(0, res.minheight)
  call assert_equal(0, res.maxwidth)
  call assert_equal(0, res.maxheight)
  call assert_equal(50, res.zindex)
  call assert_equal(0, res.fixed)
  if has('timers')
    call assert_equal(0, res.time)
  endif
  call popup_close(winid)
  call assert_equal({}, popup_getoptions(winid))
endfunc

func Test_popup_option_values()
  new
  " window-local
  setlocal number
  setlocal nowrap
  " buffer-local
  setlocal omnifunc=Something
  " global/buffer-local
  setlocal path=/there
  " global/window-local
  setlocal statusline=2

  let winid = popup_create('hello', {})
  call assert_equal(0, getwinvar(winid, '&number'))
  call assert_equal(1, getwinvar(winid, '&wrap'))
  call assert_equal('', getwinvar(winid, '&omnifunc'))
  call assert_equal(&g:path, getwinvar(winid, '&path'))
  call assert_equal(&g:statusline, getwinvar(winid, '&statusline'))

  " 'scrolloff' is reset to zero
  call assert_equal(5, &scrolloff)
  call assert_equal(0, getwinvar(winid, '&scrolloff'))

  call popup_close(winid)
  bwipe
endfunc

func Test_popup_atcursor()
  topleft vnew
  call setline(1, [
  \  'xxxxxxxxxxxxxxxxx',
  \  'xxxxxxxxxxxxxxxxx',
  \  'xxxxxxxxxxxxxxxxx',
  \])

  call cursor(2, 2)
  redraw
  let winid = popup_atcursor('vim', {})
  redraw
  let line = join(map(range(1, 17), 'screenstring(1, v:val)'), '')
  call assert_equal('xvimxxxxxxxxxxxxx', line)
  call popup_close(winid)

  call cursor(3, 4)
  redraw
  let winid = 'vim'->popup_atcursor({})
  redraw
  let line = join(map(range(1, 17), 'screenstring(2, v:val)'), '')
  call assert_equal('xxxvimxxxxxxxxxxx', line)
  call popup_close(winid)

  call cursor(1, 1)
  redraw
  let winid = popup_create('vim', #{
	\ line: 'cursor+2',
	\ col: 'cursor+1',
	\})
  redraw
  let line = join(map(range(1, 17), 'screenstring(3, v:val)'), '')
  call assert_equal('xvimxxxxxxxxxxxxx', line)
  call popup_close(winid)

  call cursor(3, 3)
  redraw
  let winid = popup_create('vim', #{
	\ line: 'cursor-2',
	\ col: 'cursor-1',
	\})
  redraw
  let line = join(map(range(1, 17), 'screenstring(1, v:val)'), '')
  call assert_equal('xvimxxxxxxxxxxxxx', line)
  call popup_close(winid)

  " just enough room above
  call cursor(3, 3)
  redraw
  let winid = popup_atcursor(['vim', 'is great'], {})
  redraw
  let pos = popup_getpos(winid)
  call assert_equal(1, pos.line)
  call popup_close(winid)

  " not enough room above, popup goes below the cursor
  call cursor(3, 3)
  redraw
  let winid = popup_atcursor(['vim', 'is', 'great'], {})
  redraw
  let pos = popup_getpos(winid)
  call assert_equal(4, pos.line)
  call popup_close(winid)

  " cursor in first line, popup in line 2
  call cursor(1, 1)
  redraw
  let winid = popup_atcursor(['vim', 'is', 'great'], {})
  redraw
  let pos = popup_getpos(winid)
  call assert_equal(2, pos.line)
  call popup_close(winid)

  bwipe!
endfunc

func Test_popup_atcursor_pos()
  CheckScreendump
  CheckFeature conceal

  let lines =<< trim END
	call setline(1, repeat([repeat('-', 60)], 15))
	set so=0

	normal 9G3|r#
	let winid1 = popup_atcursor(['first', 'second'], #{
	      \ moved: [0, 0, 0],
	      \ })
	normal 9G21|r&
	let winid1 = popup_atcursor(['FIrsT', 'SEcoND'], #{
	      \ pos: 'botright',
	      \ moved: [0, 0, 0],
	      \ })
	normal 3G27|r%
	let winid1 = popup_atcursor(['fiRSt', 'seCOnd'], #{
	      \ pos: 'topleft',
	      \ moved: [0, 0, 0],
	      \ })
	normal 3G45|r@
	let winid1 = popup_atcursor(['First', 'SeconD'], #{
	      \ pos: 'topright',
	      \ moved: range(3),
	      \ mousemoved: range(3),
	      \ })

	normal 9G27|Rconcealed  X
	syn match Hidden /concealed/ conceal
	set conceallevel=2 concealcursor=n
	redraw
	normal 0fX
	call popup_atcursor('mark', {})
  END
  call writefile(lines, 'XtestPopupAtcursorPos', 'D')
  let buf = RunVimInTerminal('-S XtestPopupAtcursorPos', #{rows: 12})
  call VerifyScreenDump(buf, 'Test_popupwin_atcursor_pos', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_beval()
  CheckScreendump
  CheckFeature balloon_eval_term

  let lines =<< trim END
	call setline(1, range(1, 20))
	call setline(5, 'here is some text to hover over')
	set balloonevalterm
	set balloonexpr=BalloonExpr()
	set balloondelay=100
	func BalloonExpr()
	  let s:winid = [v:beval_text]->popup_beval({})
	  return ''
	endfunc
	func Hover()
	  call test_setmouse(5, 15)
	  call feedkeys("\<MouseMove>\<Ignore>", "xt")
	  sleep 100m
	endfunc
	func MoveOntoPopup()
	  call test_setmouse(4, 17)
	  call feedkeys("\<F4>\<MouseMove>\<Ignore>", "xt")
	endfunc
	func MoveAway()
	  call test_setmouse(5, 13)
	  call feedkeys("\<F5>\<MouseMove>\<Ignore>", "xt")
	endfunc
  END
  call writefile(lines, 'XtestPopupBeval', 'D')
  let buf = RunVimInTerminal('-S XtestPopupBeval', #{rows: 10})
  call TermWait(buf, 50)
  call term_sendkeys(buf, 'j')
  call term_sendkeys(buf, ":call Hover()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_beval_1', {})

  call term_sendkeys(buf, ":call MoveOntoPopup()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_beval_2', {})

  call term_sendkeys(buf, ":call MoveAway()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_beval_3', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_filter()
  new
  call setline(1, 'some text')

  func MyPopupFilter(winid, c)
    if a:c == 'e' || a:c == "\<F9>"
      let g:eaten = a:c
      return 1
    endif
    if a:c == '0'
      let g:ignored = '0'
      return 0
    endif
    if a:c == 'x'
      call popup_close(a:winid)
      return 1
    endif
    return 0
  endfunc

  let winid = 'something'->popup_create(#{filter: 'MyPopupFilter'})
  redraw

  " e is consumed by the filter
  let g:eaten = ''
  call feedkeys('e', 'xt')
  call assert_equal('e', g:eaten)
  call feedkeys("\<F9>", 'xt')
  call assert_equal("\<F9>", g:eaten)

  " 0 is ignored by the filter
  let g:ignored = ''
  normal $
  call assert_equal(9, getcurpos()[2])
  call feedkeys('0', 'xt')
  call assert_equal('0', g:ignored)

  if has('win32') && has('gui_running')
    echo "FIXME: this check is very flaky on MS-Windows GUI, the cursor doesn't move"
  else
    call assert_equal(1, getcurpos()[2])
  endif

  " x closes the popup
  call feedkeys('x', 'xt')
  call assert_equal("\<F9>", g:eaten)
  call assert_equal(-1, winbufnr(winid))

  unlet g:eaten
  unlet g:ignored
  delfunc MyPopupFilter
  call popup_clear()
endfunc

" this tests that the filter is not used for :normal command
func Test_popup_filter_normal_cmd()
  CheckScreendump

  let lines =<< trim END
      let text = range(1, 20)->map({_, v -> string(v)})
      let g:winid = popup_create(text, #{maxheight: 5, minwidth: 3, filter: 'invalidfilter'})
      call timer_start(0, {-> win_execute(g:winid, 'norm! 10Gzz')})
  END
  call writefile(lines, 'XtestPopupNormal', 'D')
  let buf = RunVimInTerminal('-S XtestPopupNormal', #{rows: 10})
  call TermWait(buf, 100)
  call VerifyScreenDump(buf, 'Test_popupwin_normal_cmd', {})

  call StopVimInTerminal(buf)
endfunc

" test that cursor line highlight is updated after using win_execute()
func Test_popup_filter_win_execute()
  CheckScreendump

  let lines =<< trim END
      let lines = range(1, &lines * 2)->map({_, v -> string(v)})
      let g:id = popup_create(lines, #{
	  \ minheight: &lines - 5,
	  \ maxheight: &lines - 5,
	  \ cursorline: 1,
	  \ })
      redraw
  END
  call writefile(lines, 'XtestPopupWinExecute', 'D')
  let buf = RunVimInTerminal('-S XtestPopupWinExecute', #{rows: 14})

  call term_sendkeys(buf, ":call win_execute(g:id, ['normal 17Gzz'])\<CR>")
  call term_sendkeys(buf, ":\<CR>")

  call VerifyScreenDump(buf, 'Test_popupwin_win_execute_cursorline', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popup_set_firstline()
  CheckScreendump

  let lines =<< trim END
      let lines = range(1, 50)->map({_, v -> string(v)})
      let g:id = popup_create(lines, #{
	  \ minwidth: 20,
	  \ maxwidth: 20,
	  \ minheight: &lines - 5,
	  \ maxheight: &lines - 5,
	  \ cursorline: 1,
	  \ })
      call popup_setoptions(g:id, #{firstline: 10})
      redraw
  END
  call writefile(lines, 'XtestPopupWinSetFirstline', 'D')
  let buf = RunVimInTerminal('-S XtestPopupWinSetFirstline', #{rows: 16})

  call VerifyScreenDump(buf, 'Test_popupwin_set_firstline_1', {})

  call term_sendkeys(buf, ":call popup_setoptions(g:id, #{firstline: 5})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_set_firstline_2', {})

  call StopVimInTerminal(buf)
endfunc

" this tests that we don't get stuck with an error in "win_execute()"
func Test_popup_filter_win_execute_error()
  CheckScreendump

  let lines =<< trim END
      let g:winid = popup_create('some text', {'filter': 'invalidfilter'})
      call timer_start(0, {-> win_execute(g:winid, 'invalidCommand')})
  END
  call writefile(lines, 'XtestPopupWinExecuteError', 'D')
  let buf = RunVimInTerminal('-S XtestPopupWinExecuteError', #{rows: 10, wait_for_ruler: 0})

  call WaitFor({-> term_getline(buf, 9) =~ 'Not an editor command: invalidCommand'})
  call term_sendkeys(buf, "\<CR>")
  call WaitFor({-> term_getline(buf, 9) =~ 'Unknown function: invalidfilter'})
  call term_sendkeys(buf, "\<CR>")
  call WaitFor({-> term_getline(buf, 9) =~ 'Not allowed in a popup window'})
  call term_sendkeys(buf, "\<CR>")
  call term_sendkeys(buf, "\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_win_execute', {})

  call StopVimInTerminal(buf)
endfunc

func ShowDialog(key, result)
  let s:cb_res = 999
  let winid = popup_dialog('do you want to quit (Yes/no)?', #{
	  \ filter: 'popup_filter_yesno',
	  \ callback: 'QuitCallback',
	  \ })
  redraw
  call feedkeys(a:key, "xt")
  call assert_equal(winid, s:cb_winid)
  call assert_equal(a:result, s:cb_res)
endfunc

func Test_popup_dialog()
  func QuitCallback(id, res)
    let s:cb_winid = a:id
    let s:cb_res = a:res
  endfunc

  let winid = ShowDialog("y", 1)
  let winid = ShowDialog("Y", 1)
  let winid = ShowDialog("n", 0)
  let winid = ShowDialog("N", 0)
  let winid = ShowDialog("x", 0)
  let winid = ShowDialog("X", 0)
  let winid = ShowDialog("\<Esc>", 0)
  let winid = ShowDialog("\<C-C>", -1)

  delfunc QuitCallback
endfunc

func ShowMenu(key, result)
  let s:cb_res = 999
  let winid = popup_menu(['one', 'two', 'something else'], #{
	  \ callback: 'QuitCallback',
	  \ })
  redraw
  call feedkeys(a:key, "xt")
  call assert_equal(winid, s:cb_winid)
  call assert_equal(a:result, s:cb_res)
endfunc

func Test_popup_menu()
  func QuitCallback(id, res)
    let s:cb_winid = a:id
    let s:cb_res = a:res
  endfunc
  " mapping won't be used in popup
  map j k

  let winid = ShowMenu(" ", 1)
  let winid = ShowMenu("j \<CR>", 2)
  let winid = ShowMenu("JjK \<CR>", 2)
  " wraps around
  let winid = ShowMenu("jjjjjj ", 1)
  let winid = ShowMenu("kkk ", 1)
  let winid = ShowMenu("x", -1)
  let winid = ShowMenu("X", -1)
  let winid = ShowMenu("\<Esc>", -1)
  let winid = ShowMenu("\<C-C>", -1)

  delfunc QuitCallback
  unmap j
endfunc

func Test_popup_menu_screenshot()
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 20))
	hi PopupSelected ctermbg=lightblue
	call popup_menu(['one', 'two', 'another'], #{callback: 'MenuDone', title: ' make a choice from the list '})
	func MenuDone(id, res)
	  echomsg "selected " .. a:res
	endfunc
  END
  call writefile(lines, 'XtestPopupMenu', 'D')
  let buf = RunVimInTerminal('-S XtestPopupMenu', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_menu_01', {})

  call term_sendkeys(buf, "jj")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_02', {})

  call term_sendkeys(buf, " ")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_03', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_menu_narrow()
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 20))
	hi PopupSelected ctermbg=green
	call popup_menu(['one', 'two', 'three'], #{callback: 'MenuDone'})
	func MenuDone(id, res)
	  echomsg "selected " .. a:res
	endfunc
  END
  call writefile(lines, 'XtestPopupNarrowMenu', 'D')
  let buf = RunVimInTerminal('-S XtestPopupNarrowMenu', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_menu_04', {})

  " clean up
  call term_sendkeys(buf, "x")
  call StopVimInTerminal(buf)
endfunc

func Test_popup_title()
  CheckScreendump

  " Create a popup without title or border, a line of padding will be added to
  " put the title on.
  let lines =<< trim END
	call setline(1, range(1, 20))
	let winid = popup_create(['one', 'two', 'another'], #{title: 'Title String'})
  END
  call writefile(lines, 'XtestPopupTitle', 'D')
  let buf = RunVimInTerminal('-S XtestPopupTitle', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_title', {})

  call term_sendkeys(buf, ":call popup_setoptions(winid, #{maxwidth: 20, title: 'a very long title that is not going to fit'})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_longtitle_1', {})

  call term_sendkeys(buf, ":call popup_setoptions(winid, #{border: []})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_longtitle_2', {})

  call term_sendkeys(buf, ":call popup_clear()\<CR>")
  call term_sendkeys(buf, ":call popup_create(['aaa', 'bbb'], #{title: 'Title', minwidth: 12, padding: [2, 2, 2, 2]})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_longtitle_3', {})

  call term_sendkeys(buf, ":call popup_clear()\<CR>")
  call term_sendkeys(buf, ":call popup_create(['aaa', 'bbb'], #{title: 'Title', minwidth: 12, border: [], padding: [2, 2, 2, 2]})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_longtitle_4', {})

  call term_sendkeys(buf, ":call popup_clear()\<CR>")
  call term_sendkeys(buf, ":call popup_menu(['This is a line', 'and another line'], #{title: '▶Äあいうえお◀', })\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_multibytetitle', {})
  call term_sendkeys(buf, "x")

  " clean up
  call StopVimInTerminal(buf)

  let winid = popup_create('something', #{title: 'Some Title'})
  call assert_equal('Some Title', popup_getoptions(winid).title)
  call popup_setoptions(winid, #{title: 'Another Title'})
  call assert_equal('Another Title', popup_getoptions(winid).title)

  call popup_clear()
endfunc

func Test_popup_close_callback()
  func PopupDone(id, result)
    let g:result = a:result
  endfunc
  let winid = popup_create('something', #{callback: 'PopupDone'})
  redraw
  call popup_close(winid, 'done')
  call assert_equal('done', g:result)
endfunc

func Test_popup_empty()
  let winid = popup_create('', #{padding: [2,2,2,2]})
  redraw
  let pos = popup_getpos(winid)
  call assert_equal(5, pos.width)
  call assert_equal(5, pos.height)
  call popup_close(winid)

  let winid = popup_create([], #{border: []})
  redraw
  let pos = popup_getpos(winid)
  call assert_equal(3, pos.width)
  call assert_equal(3, pos.height)
  call popup_close(winid)
endfunc

func Test_popup_never_behind()
  CheckScreendump

  " +-----------------------------+
  " |             |               |
  " |             |               |
  " |             |               |
  " |            line1            |
  " |------------line2------------|
  " |            line3            |
  " |            line4            |
  " |                             |
  " |                             |
  " +-----------------------------+
  let lines =<< trim END
    split
    vsplit
    let info_window1 = getwininfo()[0]
    let line = info_window1['height']
    let col = info_window1['width']
    call popup_create(['line1', 'line2', 'line3', 'line4'], #{
	      \   line : line,
	      \   col : col,
	      \ })
  END
  call writefile(lines, 'XtestPopupBehind', 'D')
  let buf = RunVimInTerminal('-S XtestPopupBehind', #{rows: 10})
  call term_sendkeys(buf, "\<C-W>w")
  call VerifyScreenDump(buf, 'Test_popupwin_behind', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func s:VerifyPosition(p, msg, line, col, width, height)
  call assert_equal(a:line,   popup_getpos(a:p).line,   a:msg . ' (l)')
  call assert_equal(a:col,    popup_getpos(a:p).col,    a:msg . ' (c)')
  call assert_equal(a:width,  popup_getpos(a:p).width,  a:msg . ' (w)')
  call assert_equal(a:height, popup_getpos(a:p).height, a:msg . ' (h)')
endfunc

func Test_popup_position_adjust()
  " Anything placed past the last cell on the right of the screen is moved to
  " the left.
  "
  " When wrapping is disabled, we also shift to the left to display on the
  " screen, unless fixed is set.

  " Entries for cases which don't vary based on wrapping.
  " Format is per tests described below
  let both_wrap_tests = [
	\       ['a', 5, &columns,        5, &columns, 1, 1],
	\       ['b', 5, &columns + 1,    5, &columns, 1, 1],
	\       ['c', 5, &columns - 1,    5, &columns - 1, 1, 1],
	\       ['d', 5, &columns - 2,    5, &columns - 2, 1, 1],
	\       ['e', 5, &columns - 3,    5, &columns - 3, 1, 1]]

  " these test groups are dicts with:
  "  - comment: something to identify the group of tests by
  "  - options: dict of options to merge with the row/col in tests
  "  - tests: list of cases. Each one is a list with elements:
  "     - text
  "     - row
  "     - col
  "     - expected row
  "     - expected col
  "     - expected width
  "     - expected height
  let tests = [
	\ #{
	\   comment: 'left-aligned with wrapping',
	\   options: #{
	\     wrap: 1,
	\     pos: 'botleft',
	\   },
	\   tests: both_wrap_tests + [
	\       ['aa', 5, &columns,        4, &columns, 1, 2],
	\       ['bb', 5, &columns + 1,    4, &columns, 1, 2],
	\       ['cc', 5, &columns - 1,    5, &columns - 1, 2, 1],
	\       ['dd', 5, &columns - 2,    5, &columns - 2, 2, 1],
	\       ['ee', 5, &columns - 3,    5, &columns - 3, 2, 1],
	\
	\       ['aaa', 5, &columns,        3, &columns, 1, 3],
	\       ['bbb', 5, &columns + 1,    3, &columns, 1, 3],
	\       ['ccc', 5, &columns - 1,    4, &columns - 1, 2, 2],
	\       ['ddd', 5, &columns - 2,    5, &columns - 2, 3, 1],
	\       ['eee', 5, &columns - 3,    5, &columns - 3, 3, 1],
	\
	\       ['aaaa', 5, &columns,        2, &columns, 1, 4],
	\       ['bbbb', 5, &columns + 1,    2, &columns, 1, 4],
	\       ['cccc', 5, &columns - 1,    4, &columns - 1, 2, 2],
	\       ['dddd', 5, &columns - 2,    4, &columns - 2, 3, 2],
	\       ['eeee', 5, &columns - 3,    5, &columns - 3, 4, 1],
	\       ['eeee', 5, &columns - 4,    5, &columns - 4, 4, 1],
	\   ],
	\ },
	\ #{
	\   comment: 'left aligned without wrapping',
	\   options: #{
	\     wrap: 0,
	\     pos: 'botleft',
	\   },
	\   tests: both_wrap_tests + [
	\       ['aa', 5, &columns,        5, &columns - 1, 2, 1],
	\       ['bb', 5, &columns + 1,    5, &columns - 1, 2, 1],
	\       ['cc', 5, &columns - 1,    5, &columns - 1, 2, 1],
	\       ['dd', 5, &columns - 2,    5, &columns - 2, 2, 1],
	\       ['ee', 5, &columns - 3,    5, &columns - 3, 2, 1],
	\
	\       ['aaa', 5, &columns,        5, &columns - 2, 3, 1],
	\       ['bbb', 5, &columns + 1,    5, &columns - 2, 3, 1],
	\       ['ccc', 5, &columns - 1,    5, &columns - 2, 3, 1],
	\       ['ddd', 5, &columns - 2,    5, &columns - 2, 3, 1],
	\       ['eee', 5, &columns - 3,    5, &columns - 3, 3, 1],
	\
	\       ['aaaa', 5, &columns,        5, &columns - 3, 4, 1],
	\       ['bbbb', 5, &columns + 1,    5, &columns - 3, 4, 1],
	\       ['cccc', 5, &columns - 1,    5, &columns - 3, 4, 1],
	\       ['dddd', 5, &columns - 2,    5, &columns - 3, 4, 1],
	\       ['eeee', 5, &columns - 3,    5, &columns - 3, 4, 1],
	\   ],
	\ },
	\ #{
	\   comment: 'left aligned with fixed position',
	\   options: #{
	\     wrap: 0,
	\     fixed: 1,
	\     pos: 'botleft',
	\   },
	\   tests: both_wrap_tests + [
	\       ['aa', 5, &columns,        5, &columns, 1, 1],
	\       ['bb', 5, &columns + 1,    5, &columns, 1, 1],
	\       ['cc', 5, &columns - 1,    5, &columns - 1, 2, 1],
	\       ['dd', 5, &columns - 2,    5, &columns - 2, 2, 1],
	\       ['ee', 5, &columns - 3,    5, &columns - 3, 2, 1],
	\
	\       ['aaa', 5, &columns,        5, &columns, 1, 1],
	\       ['bbb', 5, &columns + 1,    5, &columns, 1, 1],
	\       ['ccc', 5, &columns - 1,    5, &columns - 1, 2, 1],
	\       ['ddd', 5, &columns - 2,    5, &columns - 2, 3, 1],
	\       ['eee', 5, &columns - 3,    5, &columns - 3, 3, 1],
	\
	\       ['aaaa', 5, &columns,        5, &columns, 1, 1],
	\       ['bbbb', 5, &columns + 1,    5, &columns, 1, 1],
	\       ['cccc', 5, &columns - 1,    5, &columns - 1, 2, 1],
	\       ['dddd', 5, &columns - 2,    5, &columns - 2, 3, 1],
	\       ['eeee', 5, &columns - 3,    5, &columns - 3, 4, 1],
	\   ],
	\ },
	\ ]

  for test_group in tests
    for test in test_group.tests
      let [ text, line, col, e_line, e_col, e_width, e_height ] = test
      let options = #{
	    \ line: line,
	    \ col: col,
	    \ }
      call extend(options, test_group.options)

      let p = popup_create(text, options)

      let msg = string(extend(options, #{text: text}))
      call s:VerifyPosition(p, msg, e_line, e_col, e_width, e_height)
      call popup_close(p)
    endfor
  endfor

  call popup_clear()
  %bwipe!
endfunc

func Test_adjust_left_past_screen_width()
  " width of screen
  let X = join(map(range(&columns), {->'X'}), '')

  let p = popup_create(X, #{line: 1, col: 1, wrap: 0})
  call s:VerifyPosition(p, 'full width topleft', 1, 1, &columns, 1)

  redraw
  let line = join(map(range(1, &columns + 1), 'screenstring(1, v:val)'), '')
  call assert_equal(X, line)

  call popup_close(p)
  redraw

  " Same if placed on the right hand side
  let p = popup_create(X, #{line: 1, col: &columns, wrap: 0})
  call s:VerifyPosition(p, 'full width topright', 1, 1, &columns, 1)

  redraw
  let line = join(map(range(1, &columns + 1), 'screenstring(1, v:val)'), '')
  call assert_equal(X, line)

  call popup_close(p)
  redraw

  " Extend so > window width
  let X .= 'x'

  let p = popup_create(X, #{line: 1, col: 1, wrap: 0})
  call s:VerifyPosition(p, 'full width +  1 topleft', 1, 1, &columns, 1)

  redraw
  let line = join(map(range(1, &columns + 1), 'screenstring(1, v:val)'), '')
  call assert_equal(X[ : -2 ], line)

  call popup_close(p)
  redraw

  " Shifted then truncated (the x is not visible)
  let p = popup_create(X, #{line: 1, col: &columns - 3, wrap: 0})
  call s:VerifyPosition(p, 'full width + 1 topright', 1, 1, &columns, 1)

  redraw
  let line = join(map(range(1, &columns + 1), 'screenstring(1, v:val)'), '')
  call assert_equal(X[ : -2 ], line)

  call popup_close(p)
  redraw

  " Not shifted, just truncated
  let p = popup_create(X,
	\ #{line: 1, col: 2, wrap: 0, fixed: 1})
  call s:VerifyPosition(p, 'full width + 1 fixed', 1, 2, &columns - 1, 1)

  redraw
  let line = join(map(range(1, &columns + 1), 'screenstring(1, v:val)'), '')
  let e_line = ' ' . X[ 1 : -2 ]
  call assert_equal(e_line, line)

  call popup_close(p)
  redraw

  call popup_clear()
  %bwipe!
endfunc

func Test_popup_moved()
  new
  call test_override('char_avail', 1)
  call setline(1, ['one word to move around', 'a WORD.and->some thing'])

  exe "normal gg0/word\<CR>"
  let winid = popup_atcursor('text', #{moved: 'any'})
  redraw
  call assert_equal(1, popup_getpos(winid).visible)
  call assert_equal([1, 4, 4], popup_getoptions(winid).moved)
  " trigger the check for last_cursormoved by going into insert mode
  call feedkeys("li\<Esc>", 'xt')
  call assert_equal({}, popup_getpos(winid))
  call popup_clear()

  exe "normal gg0/word\<CR>"
  let winid = popup_atcursor('text', #{moved: 'word'})
  redraw
  call assert_equal(1, popup_getpos(winid).visible)
  call assert_equal([1, 4, 7], popup_getoptions(winid).moved)
  call feedkeys("hi\<Esc>", 'xt')
  call assert_equal({}, popup_getpos(winid))
  call popup_clear()

  exe "normal gg0/word\<CR>"
  let winid = popup_atcursor('text', #{moved: 'word'})
  redraw
  call assert_equal(1, popup_getpos(winid).visible)
  call assert_equal([1, 4, 7], popup_getoptions(winid).moved)
  call feedkeys("li\<Esc>", 'xt')
  call assert_equal(1, popup_getpos(winid).visible)
  call feedkeys("ei\<Esc>", 'xt')
  call assert_equal(1, popup_getpos(winid).visible)
  call feedkeys("eli\<Esc>", 'xt')
  call assert_equal({}, popup_getpos(winid))
  call popup_clear()

  " WORD is the default
  exe "normal gg0/WORD\<CR>"
  let winid = popup_atcursor('text', {})
  redraw
  call assert_equal(1, popup_getpos(winid).visible)
  call assert_equal([2, 2, 15], popup_getoptions(winid).moved)
  call feedkeys("eli\<Esc>", 'xt')
  call assert_equal(1, popup_getpos(winid).visible)
  call feedkeys("wi\<Esc>", 'xt')
  call assert_equal(1, popup_getpos(winid).visible)
  call feedkeys("Eli\<Esc>", 'xt')
  call assert_equal({}, popup_getpos(winid))
  call popup_clear()

  exe "normal gg0/word\<CR>"
  let winid = popup_atcursor('text', #{moved: [5, 10]})
  redraw
  call assert_equal(1, popup_getpos(winid).visible)
  call feedkeys("eli\<Esc>", 'xt')
  call feedkeys("ei\<Esc>", 'xt')
  call assert_equal(1, popup_getpos(winid).visible)
  call feedkeys("eli\<Esc>", 'xt')
  call assert_equal({}, popup_getpos(winid))
  call popup_clear()

  bwipe!
  call test_override('ALL', 0)
endfunc

func Test_notifications()
  CheckFeature timers
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 20))
	hi Notification ctermbg=lightblue
	call popup_notification('first notification', {})
  END
  call writefile(lines, 'XtestNotifications', 'D')
  let buf = RunVimInTerminal('-S XtestNotifications', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_notify_01', {})

  " second one goes below the first one
  call term_sendkeys(buf, ":hi link PopupNotification Notification\<CR>")
  call term_sendkeys(buf, ":call popup_notification('another important notification', {})\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_notify_02', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_scrollbar()
  CheckScreendump

  let lines =<< trim END
    call setline(1, range(1, 20))
    hi ScrollThumb ctermbg=blue
    hi ScrollBar ctermbg=red
    let winid = popup_create(['one', 'two', 'three', 'four', 'five',
	  \ 'six', 'seven', 'eight', 'nine'], #{
	  \ minwidth: 8,
	  \ maxheight: 4,
	  \ })
    func ScrollUp()
      call feedkeys("\<F3>\<ScrollWheelUp>", "xt")
    endfunc
    func ScrollDown()
      call feedkeys("\<F3>\<ScrollWheelDown>", "xt")
    endfunc
    func ClickTop()
      call feedkeys("\<F4>\<LeftMouse>", "xt")
    endfunc
    func ClickBot()
      call popup_setoptions(g:winid, #{border: [], close: 'button'})
      call feedkeys("\<F5>\<LeftMouse>", "xt")
    endfunc
    func Popup_filter(winid, key)
      if a:key == 'j'
	silent! this_throws_an_error_but_is_ignored
	let line = popup_getoptions(a:winid).firstline
	let nlines = line('$', a:winid)
	let newline = line < nlines ? (line + 1) : nlines
	call popup_setoptions(a:winid, #{firstline: newline})
	return v:true
      elseif a:key == 'x'
	call popup_close(a:winid)
	return v:true
      endif
    endfunc

    def CreatePopup(text: list<string>): number
      return popup_create(text, {
	    \ minwidth: 30,
	    \ maxwidth: 30,
	    \ minheight: 4,
	    \ maxheight: 4,
	    \ firstline: 1,
	    \ lastline: 4,
	    \ wrap: true,
	    \ scrollbar: true,
	    \ mapping: false,
	    \ filter: g:Popup_filter,
	    \ })
    enddef

    func PopupScroll()
      call popup_clear()
      let text =<< trim END
	  1
	  2
	  3
	  4
	  long line long line long line long line long line long line
	  long line long line long line long line long line long line
	  long line long line long line long line long line long line
      END
      call CreatePopup(text)
    endfunc
    func ScrollBottom()
      call popup_clear()
      let id = CreatePopup(range(100)->map({k, v -> string(v)}))
      call popup_setoptions(id, #{firstline: 100, minheight: 9, maxheight: 9})
    endfunc
    map <silent> <F3> :call test_setmouse(5, 36)<CR>
    map <silent> <F4> :call test_setmouse(4, 42)<CR>
    map <silent> <F5> :call test_setmouse(7, 42)<CR>
  END
  call writefile(lines, 'XtestPopupScroll', 'D')
  let buf = RunVimInTerminal('-S XtestPopupScroll', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_1', {})

  call term_sendkeys(buf, ":call popup_setoptions(winid, #{firstline: 2})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_2', {})

  call term_sendkeys(buf, ":call popup_setoptions(winid, #{firstline: 6})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_3', {})

  call term_sendkeys(buf, ":call popup_setoptions(winid, #{firstline: 9})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_4', {})

  call term_sendkeys(buf, ":call popup_setoptions(winid, #{scrollbarhighlight: 'ScrollBar', thumbhighlight: 'ScrollThumb', firstline: 5})\<CR>")
  " this scrolls two lines (half the window height)
  call term_sendkeys(buf, ":call ScrollUp()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_5', {})

  call term_sendkeys(buf, ":call ScrollDown()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_6', {})

  call term_sendkeys(buf, ":call ScrollDown()\<CR>")
  " wait a bit, otherwise it fails sometimes (double click recognized?)
  sleep 100m
  call term_sendkeys(buf, ":call ScrollDown()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_7', {})

  call term_sendkeys(buf, ":call ClickTop()\<CR>")
  sleep 100m
  call term_sendkeys(buf, ":call ClickTop()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_8', {})

  call term_sendkeys(buf, ":call ClickBot()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_9', {})

  " remove the minwidth and maxheight
  call term_sendkeys(buf, ":call popup_setoptions(winid, #{maxheight: 0, minwidth: 0})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_10', {})

  " check size with non-wrapping lines
  call term_sendkeys(buf, ":call g:PopupScroll()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_11', {})

  " check size with wrapping lines
  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_12', {})

  " check thumb when scrolled all the way down
  call term_sendkeys(buf, ":call ScrollBottom()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_scroll_13', {})

  " clean up
  call term_sendkeys(buf, "x")
  call StopVimInTerminal(buf)
endfunc

func Test_popup_too_high_scrollbar()
  CheckScreendump

  let lines =<< trim END
    call setline(1, range(1, 20)->map({i, v -> repeat(v, 10)}))
    set scrolloff=0
    func ShowPopup()
      let winid = popup_atcursor(['one', 'two', 'three', 'four', 'five',
	    \ 'six', 'seven', 'eight', 'nine', 'ten', 'eleven', 'twelve'], #{
	    \ minwidth: 8,
	    \ border: [],
	    \ })
    endfunc
    normal 3G$
    call ShowPopup()
  END
  call writefile(lines, 'XtestPopupToohigh', 'D')
  let buf = RunVimInTerminal('-S XtestPopupToohigh', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_toohigh_1', {})

  call term_sendkeys(buf, ":call popup_clear()\<CR>")
  call term_sendkeys(buf, "8G$")
  call term_sendkeys(buf, ":call ShowPopup()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_toohigh_2', {})

  call term_sendkeys(buf, ":call popup_clear()\<CR>")
  call term_sendkeys(buf, "gg$")
  call term_sendkeys(buf, ":call ShowPopup()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_toohigh_3', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_fitting_scrollbar()
  " this was causing a crash, divide by zero
  let winid = popup_create([
	\ 'one', 'two', 'longer line that wraps', 'four', 'five'], #{
	\ scrollbar: 1,
	\ maxwidth: 10,
	\ maxheight: 5,
	\ firstline: 2})
  redraw
  call popup_clear()
endfunc

func Test_popup_settext()
  CheckScreendump

  let lines =<< trim END
    let opts = #{wrap: 0}
    let p = popup_create('test', opts)
    eval p->popup_settext('this is a text')
  END

  call writefile(lines, 'XtestPopupSetText', 'D')
  let buf = RunVimInTerminal('-S XtestPopupSetText', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popup_settext_01', {})

  " Setting to empty string clears it
  call term_sendkeys(buf, ":call popup_settext(p, '')\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_settext_02', {})

  " Setting a list
  call term_sendkeys(buf, ":call popup_settext(p, ['a','b','c'])\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_settext_03', {})

  " Shrinking with a list
  call term_sendkeys(buf, ":call popup_settext(p, ['a'])\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_settext_04', {})

  " Growing with a list
  call term_sendkeys(buf, ":call popup_settext(p, ['a','b','c'])\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_settext_03', {})

  " Empty list clears
  call term_sendkeys(buf, ":call popup_settext(p, [])\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_settext_05', {})

  " Dicts
  call term_sendkeys(buf, ":call popup_settext(p, [#{text: 'aaaa'}, #{text: 'bbbb'}, #{text: 'cccc'}])\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_settext_06', {})

  " range() (doesn't work)
  call term_sendkeys(buf, ":call popup_settext(p, range(4, 8))\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_settext_07', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_settext_getline()
  let id = popup_create('', #{ tabpage: 0 })
  call popup_settext(id, ['a','b'])
  call assert_equal(2, line('$', id)) " OK :)
  call popup_close(id)

  let id = popup_create('', #{ tabpage: -1 })
  call popup_settext(id, ['a','b'])
  call assert_equal(2, line('$', id)) " Fails :(
  call popup_close(id)
endfunc

func Test_popup_settext_null()
  let id = popup_create('', #{ tabpage: 0 })
  call popup_settext(id, test_null_list())
  call popup_close(id)

  let id = popup_create('', #{ tabpage: 0 })
  call popup_settext(id, test_null_string())
  call popup_close(id)
endfunc

func Test_popup_hidden()
  new

  let winid = popup_atcursor('text', #{hidden: 1})
  redraw
  call assert_equal(0, popup_getpos(winid).visible)
  call popup_close(winid)

  let winid = popup_create('text', #{hidden: 1})
  redraw
  call assert_equal(0, popup_getpos(winid).visible)
  call popup_close(winid)

  func QuitCallback(id, res)
    let s:cb_winid = a:id
    let s:cb_res = a:res
  endfunc
  let winid = 'make a choice'->popup_dialog(#{hidden: 1,
	  \ filter: 'popup_filter_yesno',
	  \ callback: 'QuitCallback',
	  \ })
  redraw
  call assert_equal(0, popup_getpos(winid).visible)
  call assert_equal(function('popup_filter_yesno'), popup_getoptions(winid).filter)
  call assert_equal(function('QuitCallback'), popup_getoptions(winid).callback)
  exe "normal anot used by filter\<Esc>"
  call assert_equal('not used by filter', getline(1))

  call popup_show(winid)
  call feedkeys('y', "xt")
  call assert_equal(1, s:cb_res)

  bwipe!
  delfunc QuitCallback
endfunc

" Test options not checked elsewhere
func Test_set_get_options()
  let winid = popup_create('some text', #{highlight: 'Beautiful'})
  let options = popup_getoptions(winid)
  call assert_equal(1, options.wrap)
  call assert_equal(0, options.drag)
  call assert_equal('Beautiful', options.highlight)

  call popup_setoptions(winid, #{wrap: 0, drag: 1, highlight: 'Another'})
  let options = popup_getoptions(winid)
  call assert_equal(0, options.wrap)
  call assert_equal(1, options.drag)
  call assert_equal('Another', options.highlight)

  call assert_fails('call popup_setoptions(winid, [])', 'E1206:')
  call assert_fails('call popup_setoptions(winid, test_null_dict())', 'E1297:')

  call popup_close(winid)
  call assert_equal(0, popup_setoptions(winid, options.wrap))
endfunc

func Test_popupwin_garbage_collect()
  func MyPopupFilter(x, winid, c)
    " NOP
  endfunc

  let winid = popup_create('something', #{filter: function('MyPopupFilter', [{}])})
  call test_garbagecollect_now()
  redraw
  " Must not crash caused by invalid memory access
  call feedkeys('j', 'xt')
  call assert_true(v:true)

  call popup_close(winid)
  delfunc MyPopupFilter
endfunc

func Test_popupwin_filter_mode()
  func MyPopupFilter(winid, c)
    let s:typed = a:c
    if a:c == ':' || a:c == "\r" || a:c == 'v'
      " can start cmdline mode, get out, and start/stop Visual mode
      return 0
    endif
    return 1
  endfunc

  " Normal, Visual and Insert mode
  let winid = popup_create('something', #{filter: 'MyPopupFilter', filtermode: 'nvi'})
  redraw
  call feedkeys('x', 'xt')
  call assert_equal('x', s:typed)

  call feedkeys(":let g:foo = 'foo'\<CR>", 'xt')
  call assert_equal(':', s:typed)
  call assert_equal('foo', g:foo)

  let @x = 'something'
  call feedkeys('v$"xy', 'xt')
  call assert_equal('y', s:typed)
  call assert_equal('something', @x)  " yank command is filtered out
  call feedkeys('v', 'xt')  " end Visual mode

  call popup_close(winid)

  " only Normal mode
  let winid = popup_create('something', #{filter: 'MyPopupFilter', filtermode: 'n'})
  redraw
  call feedkeys('x', 'xt')
  call assert_equal('x', s:typed)

  call feedkeys(":let g:foo = 'foo'\<CR>", 'xt')
  call assert_equal(':', s:typed)
  call assert_equal('foo', g:foo)

  let @x = 'something'
  call feedkeys('v$"xy', 'xt')
  call assert_equal('v', s:typed)
  call assert_notequal('something', @x)

  call popup_close(winid)

  " default: all modes
  let winid = popup_create('something', #{filter: 'MyPopupFilter'})
  redraw
  call feedkeys('x', 'xt')
  call assert_equal('x', s:typed)

  let g:foo = 'bar'
  call feedkeys(":let g:foo = 'foo'\<CR>", 'xt')
  call assert_equal("\r", s:typed)
  call assert_equal('bar', g:foo)

  let @x = 'something'
  call feedkeys('v$"xy', 'xt')
  call assert_equal('y', s:typed)
  call assert_equal('something', @x)  " yank command is filtered out
  call feedkeys('v', 'xt')  " end Visual mode

  call popup_close(winid)
  delfunc MyPopupFilter
endfunc

func Test_popupwin_filter_mouse()
  func MyPopupFilter(winid, c)
    let g:got_mousepos = getmousepos()
    return 0
  endfunc

  call setline(1, ['.'->repeat(25)]->repeat(10))
  let winid = popup_create(['short', 'long line that will wrap', 'other'], #{
	\ line: 2,
	\ col: 4,
	\ maxwidth: 12,
	\ padding: [],
	\ border: [],
	\ filter: 'MyPopupFilter',
	\ })
  redraw
  "    123456789012345678901
  "  1 .....................
  "  2 ...+--------------+..
  "  3 ...|              |..
  "  4 ...| short        |..
  "  5 ...| long line th |..
  "  6 ...| at will wrap |..
  "  7 ...| other        |..
  "  8 ...|              |..
  "  9 ...+--------------+..
  " 10 .....................
  let tests = []

  func AddItemOutsidePopup(tests, row, col)
    eval a:tests->add(#{clickrow: a:row, clickcol: a:col, result: #{
	  \ screenrow: a:row, screencol: a:col,
	  \ winid: win_getid(), winrow: a:row, wincol: a:col,
	  \ line: a:row, column: a:col, coladd: 0,
	  \ }})
  endfunc
  func AddItemInPopupBorder(tests, winid, row, col)
    eval a:tests->add(#{clickrow: a:row, clickcol: a:col, result: #{
	  \ screenrow: a:row, screencol: a:col,
	  \ winid: a:winid, winrow: a:row - 1, wincol: a:col - 3,
	  \ line: 0, column: 0, coladd: 0,
	  \ }})
  endfunc
  func AddItemInPopupText(tests, winid, row, col, textline, textcol, coladd = 0)
    eval a:tests->add(#{clickrow: a:row, clickcol: a:col, result: #{
	  \ screenrow: a:row, screencol: a:col,
	  \ winid: a:winid, winrow: a:row - 1, wincol: a:col - 3,
	  \ line: a:textline, column: a:textcol, coladd: a:coladd,
	  \ }})
  endfunc

  " above and below popup
  for c in range(1, 21)
    call AddItemOutsidePopup(tests, 1, c)
    call AddItemOutsidePopup(tests, 10, c)
  endfor
  " left and right of popup
  for r in range(1, 10)
    call AddItemOutsidePopup(tests, r, 3)
    call AddItemOutsidePopup(tests, r, 20)
  endfor
  " top and bottom in popup
  for c in range(4, 19)
    call AddItemInPopupBorder(tests, winid, 2, c)
    call AddItemInPopupBorder(tests, winid, 3, c)
    call AddItemInPopupBorder(tests, winid, 8, c)
    call AddItemInPopupBorder(tests, winid, 9, c)
  endfor
  " left and right margin in popup
  for r in range(2, 9)
    call AddItemInPopupBorder(tests, winid, r, 4)
    call AddItemInPopupBorder(tests, winid, r, 5)
    call AddItemInPopupBorder(tests, winid, r, 18)
    call AddItemInPopupBorder(tests, winid, r, 19)
  endfor
  " text "short"
  call AddItemInPopupText(tests, winid, 4, 6, 1, 1)
  call AddItemInPopupText(tests, winid, 4, 10, 1, 5)
  call AddItemInPopupText(tests, winid, 4, 11, 1, 6)
  call AddItemInPopupText(tests, winid, 4, 17, 1, 6, 6)
  " text "long line th"
  call AddItemInPopupText(tests, winid, 5, 6, 2, 1)
  call AddItemInPopupText(tests, winid, 5, 10, 2, 5)
  call AddItemInPopupText(tests, winid, 5, 17, 2, 12)
  " text "at will wrap"
  call AddItemInPopupText(tests, winid, 6, 6, 2, 13)
  call AddItemInPopupText(tests, winid, 6, 10, 2, 17)
  call AddItemInPopupText(tests, winid, 6, 17, 2, 24)
  " text "other"
  call AddItemInPopupText(tests, winid, 7, 6, 3, 1)
  call AddItemInPopupText(tests, winid, 7, 10, 3, 5)
  call AddItemInPopupText(tests, winid, 7, 11, 3, 6)
  call AddItemInPopupText(tests, winid, 7, 17, 3, 6, 6)

  for item in tests
    call test_setmouse(item.clickrow, item.clickcol)
    call feedkeys("\<LeftMouse>", 'xt')
    call assert_equal(item.result, g:got_mousepos)
  endfor

  call popup_close(winid)
  enew!
  delfunc MyPopupFilter
endfunc

func Test_popupwin_with_buffer()
  call writefile(['some text', 'in a buffer'], 'XsomeFile', 'D')
  let buf = bufadd('XsomeFile')
  call assert_equal(0, bufloaded(buf))

  setlocal number
  call setbufvar(buf, "&wrapmargin", 13)

  let winid = popup_create(buf, {})
  call assert_notequal(0, winid)
  let pos = popup_getpos(winid)
  call assert_equal(2, pos.height)
  call assert_equal(1, bufloaded(buf))

  " window-local option is set to default, buffer-local is not
  call assert_equal(0, getwinvar(winid, '&number'))
  call assert_equal(13, getbufvar(buf, '&wrapmargin'))

  call popup_close(winid)
  call assert_equal({}, popup_getpos(winid))
  call assert_equal(1, bufloaded(buf))
  exe 'bwipe! ' .. buf
  setlocal nonumber

  edit test_popupwin.vim
  let winid = popup_create(bufnr(''), {})
  redraw
  call popup_close(winid)
endfunc

func Test_popupwin_buffer_with_swapfile()
  call writefile(['some text', 'in a buffer'], 'XopenFile', 'D')
  call writefile([''], '.XopenFile.swp', 'D')
  let g:ignoreSwapExists = 1

  let bufnr = bufadd('XopenFile')
  call assert_equal(0, bufloaded(bufnr))
  let winid = popup_create(bufnr, {'hidden': 1})
  call assert_equal(1, bufloaded(bufnr))
  call popup_close(winid)

  exe 'buffer ' .. bufnr
  call assert_equal(1, &readonly)
  bwipe!

  unlet g:ignoreSwapExists
endfunc

func Test_popupwin_terminal_buffer()
  CheckFeature terminal
  CheckUnix
  " Starting a terminal to run a shell in is considered flaky.
  let g:test_is_flaky = 1

  let origwin = win_getid()

  " open help window to test that :help below fails
  help

  let termbuf = term_start(&shell, #{hidden: 1})
  let winid = popup_create(termbuf, #{minwidth: 40, minheight: 10, border: []})
  " Wait for shell to start
  call WaitForAssert({-> assert_equal("run", job_status(term_getjob(termbuf)))})
  " Wait for a prompt (see border char first, then space after prompt)
  call WaitForAssert({ -> assert_equal(' ', screenstring(screenrow(), screencol() - 1))})

  " When typing a character, the cursor is after it.
  call feedkeys("x", 'xt')
  call term_wait(termbuf)
  redraw
  call WaitForAssert({ -> assert_equal('x', screenstring(screenrow(), screencol() - 1))})
  call feedkeys("\<BS>", 'xt')

  " Check this doesn't crash
  call assert_equal(winnr(), winnr('j'))
  call assert_equal(winnr(), winnr('k'))
  call assert_equal(winnr(), winnr('h'))
  call assert_equal(winnr(), winnr('l'))

  " Cannot quit while job is running
  call assert_fails('call feedkeys("\<C-W>:quit\<CR>", "xt")', 'E948:')

  " Cannot enter Terminal-Normal mode. (TODO: but it works...)
  call feedkeys("xxx\<C-W>N", 'xt')
  call assert_fails('call feedkeys("gf", "xt")', 'E863:')
  call feedkeys("a\<C-U>", 'xt')

  " Cannot escape from terminal window
  call assert_fails('tab drop xxx', 'E863:')
  call assert_fails('help', 'E994:')

  " Cannot open a second one.
  let termbuf2 = term_start(&shell, #{hidden: 1})
  call assert_fails('call popup_create(termbuf2, #{})', 'E861:')
  call term_sendkeys(termbuf2, "exit\<CR>")

  " Exiting shell puts popup window in Terminal-Normal mode.
  call feedkeys("exit\<CR>", 'xt')
  " Wait for shell to exit
  call WaitForAssert({-> assert_equal("dead", job_status(term_getjob(termbuf)))})

  helpclose
  call feedkeys(":quit\<CR>", 'xt')
  call assert_equal(origwin, win_getid())
endfunc

func Test_popupwin_terminal_buffer_none()
  CheckFeature terminal
  CheckUnix

  " Starting a terminal to run a shell in is considered flaky.
  let g:test_is_flaky = 1

  let origwin = win_getid()
  call term_start("NONE", {"hidden": 1})->popup_create({"border": []})
  sleep 50m

  " since no actual job is running can close the window with :quit
  call feedkeys("\<C-W>:q\<CR>", 'xt')
  call assert_equal([], popup_list())

  call assert_equal(origwin, win_getid())
endfunc

func Test_popupwin_terminal_scrollbar()
  CheckFeature terminal
  CheckScreendump
  CheckUnix

  call writefile(range(50), 'Xtestfile', 'D')
  let lines =<< trim END
      vim9script

      # testing CTRL-W CTRL-W requires two windows
      split

      term_start(['cat', 'Xtestfile'], {hidden: true})
	  ->popup_create({
	      minwidth: 40,
	      maxwidth: 40,
	      minheight: 8,
	      maxheight: 8,
	      scrollbar: true,
	      border: []
	  })
  END
  call writefile(lines, 'Xpterm', 'D')
  let buf = RunVimInTerminal('-S Xpterm', #{rows: 15})
  call VerifyScreenDump(buf, 'Test_popupwin_poptermscroll_1', {})

  " scroll to the middle
  call term_sendkeys(buf, "50%")
  call VerifyScreenDump(buf, 'Test_popupwin_poptermscroll_2', {})

  " get error if trying to escape the window
  call term_sendkeys(buf, "\<C-W>\<C-W>")
  call VerifyScreenDump(buf, 'Test_popupwin_poptermscroll_3', {})

  " close the popupwin.
  call term_sendkeys(buf, ":q\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_poptermscroll_4', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popupwin_close_prevwin()
  CheckFeature terminal
  call Popupwin_close_prevwin()
endfunc

def Popupwin_close_prevwin()
  assert_equal(1, winnr('$'))
  split
  wincmd b
  assert_equal(2, winnr())
  var buf = term_start(&shell, {hidden: 1})
  popup_create(buf, {})
  g:TermWait(buf, 100)
  popup_clear(true)
  assert_equal(2, winnr())

  quit
  exe 'bwipe! ' .. buf
enddef

func Test_popupwin_with_buffer_and_filter()
  new Xwithfilter
  call setline(1, range(100))
  let bufnr = bufnr()
  hide

  func BufferFilter(win, key)
    if a:key == 'G'
      " recursive use of "G" does not cause problems.
      call win_execute(a:win, 'normal! G')
      return 1
    endif
    return 0
  endfunc

  let winid = popup_create(bufnr, #{maxheight: 5, filter: 'BufferFilter'})
  call assert_equal(1, popup_getpos(winid).firstline)
  redraw
  call feedkeys("G", 'xt')
  call assert_equal(99, popup_getpos(winid).firstline)

  call popup_close(winid)
  exe 'bwipe! ' .. bufnr
endfunc

func Test_popupwin_width()
  let winid = popup_create(repeat(['short', 'long long long line', 'medium width'], 50), #{
	\ maxwidth: 40,
	\ maxheight: 10,
	\ })
  for top in range(1, 20)
    eval winid->popup_setoptions(#{firstline: top})
    redraw
    call assert_equal(19, popup_getpos(winid).width)
  endfor
  call popup_clear()
endfunc

func Test_popupwin_buf_close()
  let buf = bufadd('Xtestbuf')
  call bufload(buf)
  call setbufline(buf, 1, ['just', 'some', 'lines'])
  let winid = popup_create(buf, {})
  redraw
  call assert_equal(3, popup_getpos(winid).height)
  let bufinfo = getbufinfo(buf)[0]
  call assert_equal(1, bufinfo.changed)
  call assert_equal(0, bufinfo.hidden)
  call assert_equal(0, bufinfo.listed)
  call assert_equal(1, bufinfo.loaded)
  call assert_equal([], bufinfo.windows)
  call assert_equal([winid], bufinfo.popups)

  call popup_close(winid)
  call assert_equal({}, popup_getpos(winid))
  let bufinfo = getbufinfo(buf)[0]
  call assert_equal(1, bufinfo.changed)
  call assert_equal(1, bufinfo.hidden)
  call assert_equal(0, bufinfo.listed)
  call assert_equal(1, bufinfo.loaded)
  call assert_equal([], bufinfo.windows)
  call assert_equal([], bufinfo.popups)
  exe 'bwipe! ' .. buf
endfunc

func Test_popup_menu_with_maxwidth()
  CheckScreendump

  let lines =<< trim END
	call setline(1, range(1, 10))
	hi ScrollThumb ctermbg=blue
	hi ScrollBar ctermbg=red
	func PopupMenu(lines, line, col, scrollbar = 0)
		return popup_menu(a:lines, #{
			\ maxwidth: 10,
			\ maxheight: 3,
			\ pos : 'topleft',
			\ col : a:col,
			\ line : a:line,
			\ scrollbar : a:scrollbar,
			\ })
	endfunc
	call PopupMenu(['x'], 1, 1)
	call PopupMenu(['123456789|'], 1, 16)
	call PopupMenu(['123456789|' .. ' '], 7, 1)
	call PopupMenu([repeat('123456789|', 100)], 7, 16)
	call PopupMenu(repeat(['123456789|' .. ' '], 5), 1, 33, 1)
  END
  call writefile(lines, 'XtestPopupMenuMaxWidth', 'D')
  let buf = RunVimInTerminal('-S XtestPopupMenuMaxWidth', #{rows: 13})
  call VerifyScreenDump(buf, 'Test_popupwin_menu_maxwidth_1', {})

  " close the menu popupwin.
  call term_sendkeys(buf, " ")
  call term_sendkeys(buf, " ")
  call term_sendkeys(buf, " ")
  call term_sendkeys(buf, " ")
  call term_sendkeys(buf, " ")

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_menu_with_scrollbar()
  CheckScreendump

  let lines =<< trim END
    call setline(1, range(1, 20))
    hi ScrollThumb ctermbg=blue
    hi ScrollBar ctermbg=red
    eval ['one', 'two', 'three', 'four', 'five',
	  \ 'six', 'seven', 'eight', 'nine']
	  \ ->popup_menu(#{
	  \ minwidth: 8,
	  \ maxheight: 3,
	  \ })
  END
  call writefile(lines, 'XtestPopupMenuScroll', 'D')
  let buf = RunVimInTerminal('-S XtestPopupMenuScroll', #{rows: 10})

  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_scroll_1', {})

  call term_sendkeys(buf, "jjj")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_scroll_2', {})

  " the cursor wraps around at the bottom
  call term_sendkeys(buf, repeat("j", 20))
  call VerifyScreenDump(buf, 'Test_popupwin_menu_scroll_3', {})

  " if the cursor is again at the bottom line
  call term_sendkeys(buf, repeat("j", 2))
  call VerifyScreenDump(buf, 'Test_popupwin_menu_scroll_3a', {})

  call term_sendkeys(buf, "kk")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_scroll_4', {})

  call term_sendkeys(buf, "k")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_scroll_5', {})

  " the cursor wraps around at the top
  call term_sendkeys(buf, repeat("k", 20))
  call VerifyScreenDump(buf, 'Test_popupwin_menu_scroll_6', {})

  " the cursor at the top of the window again
  call term_sendkeys(buf, repeat("k", 3))
  call VerifyScreenDump(buf, 'Test_popupwin_menu_scroll_6a', {})

  " close the menu popupwin.
  call term_sendkeys(buf, " ")

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_menu_filter()
  CheckScreendump

  let lines =<< trim END
	function! MyFilter(winid, key) abort
	  if a:key == "0"
		call win_execute(a:winid, "call setpos('.', [0, 1, 1, 0])")
		return 1
	  endif
	  if a:key == "G"
		call win_execute(a:winid, "call setpos('.', [0, line('$'), 1, 0])")
		return 1
	  endif
	  if a:key == "j"
		call win_execute(a:winid, "call setpos('.', [0, line('.') + 1, 1, 0])")
		return 1
	  endif
	  if a:key == "k"
		call win_execute(a:winid, "call setpos('.', [0, line('.') - 1, 1, 0])")
		return 1
	  endif
	  if a:key == ':'
		call popup_close(a:winid)
		return 0
	  endif
	  return 0
	endfunction
	call popup_menu(['111', '222', '333', '444', '555', '666', '777', '888', '999'], #{
	  \ maxheight : 3,
	  \ filter : 'MyFilter'
	  \ })
  END
  call writefile(lines, 'XtestPopupMenuFilter', 'D')
  let buf = RunVimInTerminal('-S XtestPopupMenuFilter', #{rows: 10})

  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_filter_1', {})

  call term_sendkeys(buf, "k")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_filter_2', {})

  call term_sendkeys(buf, "G")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_filter_3', {})

  call term_sendkeys(buf, "0")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_filter_4', {})

  " check that when the popup is closed in the filter the screen is redrawn
  call term_sendkeys(buf, ":")
  call VerifyScreenDump(buf, 'Test_popupwin_menu_filter_5', {})
  call term_sendkeys(buf, "\<CR>")

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_cursorline()
  CheckScreendump

  let winid = popup_create('some text', {})
  call assert_equal(0, popup_getoptions(winid).cursorline)
  call popup_close(winid)

  let winid = popup_create('some text', #{ cursorline: 1, })
  call assert_equal(1, popup_getoptions(winid).cursorline)
  call popup_close(winid)

  let winid = popup_create('some text', #{ cursorline: v:true, })
  call assert_equal(1, popup_getoptions(winid).cursorline)
  call popup_close(winid)

  let winid = popup_create('some text', #{ cursorline: 0, })
  call assert_equal(0, popup_getoptions(winid).cursorline)
  call popup_close(winid)

  let winid = popup_menu('some text', {})
  call assert_equal(1, popup_getoptions(winid).cursorline)
  call popup_close(winid)

  let winid = popup_menu('some text', #{ cursorline: 1, })
  call assert_equal(1, popup_getoptions(winid).cursorline)
  call popup_close(winid)

  let winid = popup_menu('some text', #{ cursorline: 0, })
  call assert_equal(0, popup_getoptions(winid).cursorline)
  call popup_close(winid)

  " ---------
  " Pattern 1
  " ---------
  let lines =<< trim END
	call popup_create(['111', '222', '333'], #{ cursorline : 0 })
  END
  call writefile(lines, 'XtestPopupCursorLine', 'D')
  let buf = RunVimInTerminal('-S XtestPopupCursorLine', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_cursorline_1', {})
  call term_sendkeys(buf, ":call popup_clear()\<cr>")
  call StopVimInTerminal(buf)

  " ---------
  " Pattern 2
  " ---------
  let lines =<< trim END
	call popup_create(['111', '222', '333'], #{ cursorline : 1 })
  END
  call writefile(lines, 'XtestPopupCursorLine')
  let buf = RunVimInTerminal('-S XtestPopupCursorLine', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_cursorline_2', {})
  call term_sendkeys(buf, ":call popup_clear()\<cr>")
  call StopVimInTerminal(buf)

  " ---------
  " Pattern 3
  " ---------
  let lines =<< trim END
	function! MyFilter(winid, key) abort
	  if a:key == "j"
		call win_execute(a:winid, "call setpos('.', [0, line('.') + 1, 1, 0]) | redraw")
		return 1
	  endif
	  if a:key == 'x'
		call popup_close(a:winid)
		return 1
	  endif
	  return 0
	endfunction
	call popup_menu(['111', '222', '333'], #{
	  \ cursorline : 0,
	  \ maxheight : 2,
	  \ filter : 'MyFilter',
	  \ })
  END
  call writefile(lines, 'XtestPopupCursorLine')
  let buf = RunVimInTerminal('-S XtestPopupCursorLine', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_cursorline_3', {})
  call term_sendkeys(buf, "j")
  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_popupwin_cursorline_4', {})
  call term_sendkeys(buf, "x")
  call StopVimInTerminal(buf)

  " ---------
  " Pattern 4
  " ---------
  let lines =<< trim END
	function! MyFilter(winid, key) abort
	  if a:key == "j"
		call win_execute(a:winid, "call setpos('.', [0, line('.') + 1, 1, 0]) | redraw")
		return 1
	  endif
	  if a:key == 'x'
		call popup_close(a:winid)
		return 1
	  endif
	  return 0
	endfunction
	call popup_menu(['111', '222', '333'], #{
	  \ cursorline : 1,
	  \ maxheight : 2,
	  \ filter : 'MyFilter',
	  \ })
  END
  call writefile(lines, 'XtestPopupCursorLine')
  let buf = RunVimInTerminal('-S XtestPopupCursorLine', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_cursorline_5', {})
  call term_sendkeys(buf, "j")
  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_popupwin_cursorline_6', {})
  call term_sendkeys(buf, "x")
  call StopVimInTerminal(buf)

  " ---------
  " Cursor in second line when creating the popup
  " ---------
  let lines =<< trim END
    let winid = popup_create(['111', '222', '333'], #{
	  \ cursorline : 1,
	  \ })
    call win_execute(winid, "2")
  END
  call writefile(lines, 'XtestPopupCursorLine')
  let buf = RunVimInTerminal('-S XtestPopupCursorLine', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_cursorline_7', {})
  call StopVimInTerminal(buf)

  " ---------
  " Use current buffer for popupmenu
  " ---------
  let lines =<< trim END
    call setline(1, ['one', 'two', 'three'])
    let winid = popup_create(bufnr('%'), #{
	  \ cursorline : 1,
	  \ })
    call win_execute(winid, "2")
  END
  call writefile(lines, 'XtestPopupCursorLine')
  let buf = RunVimInTerminal('-S XtestPopupCursorLine', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_cursorline_8', {})
  call StopVimInTerminal(buf)
endfunc

def Test_popup_cursorline_vim9()
  var winid = popup_create('some text', { cursorline: true, })
  assert_equal(1, popup_getoptions(winid).cursorline)
  popup_close(winid)

  assert_fails("popup_create('some text', { cursorline: 2, })", 'E1023:')
  popup_clear()
enddef

func Test_previewpopup()
  CheckScreendump
  CheckFeature quickfix

  call writefile([
        \ "!_TAG_FILE_ENCODING\tutf-8\t//",
        \ "another\tXtagfile\t/^this is another",
        \ "theword\tXtagfile\t/^theword"],
        \ 'Xtags', 'D')
  call writefile(range(1,20)
        \ + ['theword is here']
        \ + range(22, 27)
        \ + ['this is another place']
        \ + range(29, 40),
        \ "Xtagfile", 'D')
  call writefile(range(1,10)
        \ + ['searched word is here']
        \ + range(12, 20),
        \ "Xheader.h", 'D')
  let lines =<< trim END
        set tags=Xtags
	call setline(1, [
	      \ 'one',
	      \ '#include "Xheader.h"',
	      \ 'three',
	      \ 'four',
	      \ 'five',
	      \ 'six',
	      \ 'seven',
	      \ 'find theword somewhere',
	      \ 'nine',
	      \ 'this is another word',
	      \ 'very long line where the word is also another'])
        set previewpopup=height:4,width:40
	hi OtherColor ctermbg=lightcyan guibg=lightcyan
	set path=.
  END
  call writefile(lines, 'XtestPreviewPopup', 'D')
  let buf = RunVimInTerminal('-S XtestPreviewPopup', #{rows: 14})

  call term_sendkeys(buf, "/theword\<CR>\<C-W>}")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_1', {})

  call term_sendkeys(buf, ":set previewpopup+=highlight:OtherColor\<CR>")
  call term_sendkeys(buf, "/another\<CR>\<C-W>}")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_2', {})

  call term_sendkeys(buf, ":call popup_move(popup_findpreview(), #{col: 15})\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_3', {})

  call term_sendkeys(buf, "/another\<CR>\<C-W>}")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_4', {})

  call term_sendkeys(buf, ":silent cd ..\<CR>:\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_5', {})
  call term_sendkeys(buf, ":silent cd testdir\<CR>")

  call term_sendkeys(buf, ":set previewpopup-=highlight:OtherColor\<CR>")
  call term_sendkeys(buf, ":pclose\<CR>")
  call term_sendkeys(buf, ":\<BS>")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_6', {})

  call term_sendkeys(buf, ":pedit +/theword Xtagfile\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_7', {})

  call term_sendkeys(buf, ":pclose\<CR>")
  call term_sendkeys(buf, ":psearch searched\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_8', {})

  call term_sendkeys(buf, "\<C-W>p")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_9', {})

  call term_sendkeys(buf, ":call win_execute(popup_findpreview(), 'call popup_clear()')\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_previewpopup_10', {})

  call StopVimInTerminal(buf)
endfunc

func Test_previewpopup_pum()
  CheckScreendump
  CheckFeature quickfix

  let lines =<< trim END
      let a = 3
      let b = 1
      echo a
      echo b
      call system('echo hello')
      " the end
  END
  call writefile(lines, 'XpreviewText.vim', 'D')

  let lines =<< trim END
      call setline(1, ['one', 'two', 'three', 'other', 'once', 'only', 'off'])
      set previewpopup=height:6,width:40
      pedit XpreviewText.vim
  END
  call writefile(lines, 'XtestPreviewPum', 'D')
  let buf = RunVimInTerminal('-S XtestPreviewPum', #{rows: 12})

  call term_sendkeys(buf, "A o\<C-N>")
  call VerifyScreenDump(buf, 'Test_pum_preview_1', {})

  call term_sendkeys(buf, "\<C-N>")
  call VerifyScreenDump(buf, 'Test_pum_preview_2', {})

  call term_sendkeys(buf, "\<C-N>")
  call VerifyScreenDump(buf, 'Test_pum_preview_3', {})

  call term_sendkeys(buf, "\<C-N>")
  call VerifyScreenDump(buf, 'Test_pum_preview_4', {})

  call term_sendkeys(buf, "\<Esc>")
  call StopVimInTerminal(buf)
endfunc


func Get_popupmenu_lines()
  let lines =<< trim END
      set completeopt+=preview,popup
      set completefunc=CompleteFuncDict
      hi InfoPopup ctermbg=yellow

      func CompleteFuncDict(findstart, base)
	if a:findstart
	  if col('.') > 10
	    return col('.') - 10
	  endif
	  return 0
	endif

	return {
		\ 'words': [
		  \ {
		    \ 'word': 'aword',
		    \ 'abbr': 'wrd',
		    \ 'menu': 'extra text',
		    \ 'info': 'words are cool',
		    \ 'kind': 'W',
		    \ 'user_data': 'test'
		  \ },
		  \ {
		    \ 'word': 'anotherword',
		    \ 'abbr': 'anotwrd',
		    \ 'menu': 'extra text',
		    \ 'info': "other words are\ncooler than this and some more text\nto make wrap",
		    \ 'kind': 'W',
		    \ 'user_data': 'notest'
		  \ },
		  \ {
		    \ 'word': 'noinfo',
		    \ 'abbr': 'noawrd',
		    \ 'menu': 'extra text',
		    \ 'info': "lets\nshow\na\nscrollbar\nhere",
		    \ 'kind': 'W',
		    \ 'user_data': 'notest'
		  \ },
		  \ {
		    \ 'word': 'thatword',
		    \ 'abbr': 'thatwrd',
		    \ 'menu': 'extra text',
		    \ 'info': 'that word is cool',
		    \ 'kind': 'W',
		    \ 'user_data': 'notest'
		  \ },
		\ ]
	      \ }
      endfunc
      call setline(1, 'text text text text text text text ')
      func ChangeColor()
	let id = popup_findinfo()
	if buflisted(winbufnr(id))
	  call setline(1, 'buffer is listed')
	endif
	eval id->popup_setoptions(#{highlight: 'InfoPopup'})
      endfunc

      func InfoHidden()
	set completepopup=height:4,border:off,align:menu
	set completeopt-=popup completeopt+=popuphidden
	au CompleteChanged * call HandleChange()
      endfunc

      let s:counter = 0
      func HandleChange()
	let s:counter += 1
	let selected = complete_info(['selected']).selected
	if selected <= 0
	  " First time: do nothing, info remains hidden
	  return
	endif
	if selected == 1
	  " Second time: show info right away
	  let id = popup_findinfo()
	  if id
	    call popup_settext(id, 'immediate info ' .. s:counter)
	    call popup_show(id)
	  endif
	else
	  " Third time: show info after a short delay
	  call timer_start(100, 'ShowInfo')
	endif
      endfunc

      func ShowInfo(...)
	let id = popup_findinfo()
	if id
	  call popup_settext(id, 'async info ' .. s:counter)
	  call popup_show(id)
	endif
      endfunc

      func OpenOtherPopups()
	call popup_create([
		\ 'popup below',
		\ 'popup below',
		\ 'popup below',
		\ 'popup below',
	      \ ], #{
		\ line: 'cursor',
		\ col: 'cursor+3',
		\ highlight: 'ErrorMsg',
		\ minwidth: 17,
		\ zindex: 50,
	      \ })
	call popup_create([
		\ 'popup on top',
		\ 'popup on top',
		\ 'popup on top',
	      \ ], #{
		\ line: 'cursor+3',
		\ col: 'cursor-10',
		\ highlight: 'Search',
		\ minwidth: 10,
		\ zindex: 200,
	      \ })
      endfunc

      " Check that no autocommands are triggered for the info popup
      au WinEnter * if win_gettype() == 'popup' | call setline(2, 'WinEnter') | endif
      au WinLeave * if win_gettype() == 'popup' | call setline(2, 'WinLeave') | endif
  END
  return lines
endfunc

func Test_popupmenu_info_border()
  CheckScreendump
  CheckFeature quickfix

  let lines = Get_popupmenu_lines()
  call add(lines, 'set completepopup=height:4,highlight:InfoPopup')
  call writefile(lines, 'XtestInfoPopup', 'D')

  let buf = RunVimInTerminal('-S XtestInfoPopup', #{rows: 14})
  call TermWait(buf, 25)

  call term_sendkeys(buf, "A\<C-X>\<C-U>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_1', {})

  call term_sendkeys(buf, "\<C-N>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_2', {})

  call term_sendkeys(buf, "\<C-N>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_3', {})

  call term_sendkeys(buf, "\<C-N>\<C-N>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_4', {})

  " info on the left with scrollbar
  call term_sendkeys(buf, "test text test text\<C-X>\<C-U>")
  call term_sendkeys(buf, "\<C-N>\<C-N>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_5', {})

  " Test that the popupmenu's scrollbar and infopopup do not overlap
  call term_sendkeys(buf, "\<Esc>")
  call term_sendkeys(buf, ":set pumheight=3\<CR>")
  call term_sendkeys(buf, "cc\<C-X>\<C-U>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_6', {})

  " Hide the info popup, cycle through buffers, make sure it didn't get
  " deleted.
  call term_sendkeys(buf, "\<Esc>")
  call term_sendkeys(buf, ":set hidden\<CR>")
  call term_sendkeys(buf, ":bn\<CR>")
  call term_sendkeys(buf, ":bn\<CR>")
  call term_sendkeys(buf, "otest text test text\<C-X>\<C-U>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_7', {})

  " Test that when the option is changed the popup changes.
  call term_sendkeys(buf, "\<Esc>")
  call term_sendkeys(buf, ":set completepopup=border:off\<CR>")
  call term_sendkeys(buf, "a\<C-X>\<C-U>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_8', {})

  call term_sendkeys(buf, " \<Esc>")
  call term_sendkeys(buf, ":set completepopup+=width:10\<CR>")
  call term_sendkeys(buf, "a\<C-X>\<C-U>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_9', {})

  call term_sendkeys(buf, "\<Esc>")
  call StopVimInTerminal(buf)
endfunc

func Test_popupmenu_info_noborder()
  CheckScreendump
  CheckFeature quickfix

  let lines = Get_popupmenu_lines()
  call add(lines, 'set completepopup=height:4,border:off')
  call writefile(lines, 'XtestInfoPopupNb', 'D')

  let buf = RunVimInTerminal('-S XtestInfoPopupNb', #{rows: 14})
  call TermWait(buf, 25)

  call term_sendkeys(buf, "A\<C-X>\<C-U>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_nb_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popupmenu_info_align_menu()
  CheckScreendump
  CheckFeature quickfix

  let lines = Get_popupmenu_lines()
  call add(lines, 'set completepopup=height:4,border:off,align:menu')
  call writefile(lines, 'XtestInfoPopupNb', 'D')

  let buf = RunVimInTerminal('-S XtestInfoPopupNb', #{rows: 14})
  call TermWait(buf, 25)

  call term_sendkeys(buf, "A\<C-X>\<C-U>")
  call term_sendkeys(buf, "\<C-N>")
  call term_sendkeys(buf, "\<C-N>")
  call term_sendkeys(buf, "\<C-N>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_align_1', {})

  call term_sendkeys(buf, "test text test text test\<C-X>\<C-U>")
  call term_sendkeys(buf, "\<C-N>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_align_2', {})

  call term_sendkeys(buf, "\<Esc>")
  call term_sendkeys(buf, ":call ChangeColor()\<CR>")
  call term_sendkeys(buf, ":call setline(2, ['x']->repeat(10))\<CR>")
  call term_sendkeys(buf, "Gotest text test text\<C-X>\<C-U>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_align_3', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popupmenu_info_hidden()
  CheckScreendump
  CheckFeature quickfix

  let lines = Get_popupmenu_lines()
  call add(lines, 'call InfoHidden()')
  call writefile(lines, 'XtestInfoPopupHidden', 'D')

  let buf = RunVimInTerminal('-S XtestInfoPopupHidden', #{rows: 14})
  call TermWait(buf, 25)

  call term_sendkeys(buf, "A\<C-X>\<C-U>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_hidden_1', {})

  call term_sendkeys(buf, "\<C-N>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_hidden_2', {})

  call term_sendkeys(buf, "\<C-N>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_hidden_3', {})

  call term_sendkeys(buf, "\<Esc>")
  call StopVimInTerminal(buf)
endfunc

func Test_popupmenu_info_too_wide()
  CheckScreendump
  CheckFeature quickfix

  let lines =<< trim END
    call setline(1, range(10))

    set completeopt+=preview,popup
    set completepopup=align:menu
    set omnifunc=OmniFunc
    hi InfoPopup ctermbg=lightgrey

    func OmniFunc(findstart, base)
      if a:findstart
        return 0
      endif

      let menuText = 'some long text to make sure the menu takes up all of the width of the window'
      return #{
	\ words: [
	  \ #{
	    \ word: 'scrap',
	    \ menu: menuText,
	    \ info: "other words are\ncooler than this and some more text\nto make wrap",
	  \ },
	  \ #{
	    \ word: 'scappier',
	    \ menu: menuText,
	    \ info: 'words are cool',
	  \ },
	  \ #{
	    \ word: 'scrappier2',
	    \ menu: menuText,
	    \ info: 'words are cool',
	  \ },
	\ ]
     \ }
    endfunc
  END

  call writefile(lines, 'XtestInfoPopupWide', 'D')
  let buf = RunVimInTerminal('-S XtestInfoPopupWide', #{rows: 8})
  call TermWait(buf, 25)

  call term_sendkeys(buf, "Ascr\<C-X>\<C-O>")
  call VerifyScreenDump(buf, 'Test_popupwin_infopopup_wide_1', {})

  call term_sendkeys(buf, "\<Esc>")
  call StopVimInTerminal(buf)
endfunc

func Test_popupmenu_masking()
  " Test that popup windows that are opened while popup menu is open are
  " properly displayed.
  CheckScreendump
  CheckFeature quickfix

  let lines = Get_popupmenu_lines()
  call add(lines, 'inoremap <C-A> <Cmd>call OpenOtherPopups()<CR>')
  call writefile(lines, 'XtestPopupmenuMasking', 'D')

  let buf = RunVimInTerminal('-S XtestPopupmenuMasking', #{rows: 14})
  call TermWait(buf, 25)

  call term_sendkeys(buf, "A" .. GetEscCodeWithModifier('C', 'X')
			    \ .. GetEscCodeWithModifier('C', 'U')
			    \ .. GetEscCodeWithModifier('C', 'A'))
  call VerifyScreenDump(buf, 'Test_popupwin_popupmenu_masking_1', {})

  call term_sendkeys(buf, "\<Esc>")
  call VerifyScreenDump(buf, 'Test_popupwin_popupmenu_masking_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popupwin_recycle_bnr()
  let bufnr = popup_notification('nothing wrong', {})->winbufnr()
  call popup_clear()
  let winid = 'nothing wrong'->popup_notification({})
  call assert_equal(bufnr, winbufnr(winid))
  call popup_clear()
endfunc

func Test_popupwin_getoptions_tablocal()
  topleft split
  let win1 = popup_create('nothing', #{maxheight: 8})
  let win2 = popup_create('something', #{maxheight: 10})
  let win3 = popup_create('something', #{maxheight: 15})
  call assert_equal(8, popup_getoptions(win1).maxheight)
  call assert_equal(10, popup_getoptions(win2).maxheight)
  call assert_equal(15, popup_getoptions(win3).maxheight)
  call popup_clear()
  quit
endfunc

func Test_popupwin_cancel()
  let win1 = popup_create('one', #{line: 5, filter: {... -> 0}})
  let win2 = popup_create('two', #{line: 10, filter: {... -> 0}})
  let win3 = popup_create('three', #{line: 15, filter: {... -> 0}})
  call assert_equal(5, popup_getpos(win1).line)
  call assert_equal(10, popup_getpos(win2).line)
  call assert_equal(15, popup_getpos(win3).line)
  " TODO: this also works without patch 8.1.2110
  call feedkeys("\<C-C>", 'xt')
  call assert_equal(5, popup_getpos(win1).line)
  call assert_equal(10, popup_getpos(win2).line)
  call assert_equal({}, popup_getpos(win3))
  call feedkeys("\<C-C>", 'xt')
  call assert_equal(5, popup_getpos(win1).line)
  call assert_equal({}, popup_getpos(win2))
  call assert_equal({}, popup_getpos(win3))
  call feedkeys("\<C-C>", 'xt')
  call assert_equal({}, popup_getpos(win1))
  call assert_equal({}, popup_getpos(win2))
  call assert_equal({}, popup_getpos(win3))
endfunc

func Test_popupwin_filter_redraw()
  " Create two popups with a filter that closes the popup when typing "0".
  " Both popups should close, even though the redraw also calls
  " popup_reset_handled()

  func CloseFilter(winid, key)
    if a:key == '0'
      call popup_close(a:winid)
      redraw
    endif
    return 0  " pass the key
  endfunc

  let id1 = popup_create('first one', #{
	\ line: 1,
	\ col: 1,
	\ filter: 'CloseFilter',
	\ })
  let id2 = popup_create('second one', #{
	\ line: 9,
	\ col: 1,
	\ filter: 'CloseFilter',
	\ })
  call assert_equal(1, popup_getpos(id1).line)
  call assert_equal(9, popup_getpos(id2).line)

  call feedkeys('0', 'xt')
  call assert_equal({}, popup_getpos(id1))
  call assert_equal({}, popup_getpos(id2))

  call popup_clear()
  delfunc CloseFilter
endfunc

func Test_popupwin_double_width()
  CheckScreendump

  let lines =<< trim END
    call setline(1, 'x你好世界你好世你好世界你好')
    call setline(2, '你好世界你好世你好世界你好')
    call setline(3, 'x你好世界你好世你好世界你好')
    call popup_create('你好，世界 - 你好，世界xxxxx', #{line: 1, col: 3, maxwidth: 14})
  END
  call writefile(lines, 'XtestPopupWide', 'D')

  let buf = RunVimInTerminal('-S XtestPopupWide', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_doublewidth_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popupwin_sign()
  CheckScreendump

  let lines =<< trim END
    call setline(1, range(10))
    call sign_define('Current', {
	    \ 'text': '>>',
	    \ 'texthl': 'WarningMsg',
	    \ 'linehl': 'Error',
	    \ })
    call sign_define('Other', {
	    \ 'text': '#!',
	    \ 'texthl': 'Error',
	    \ 'linehl': 'Search',
	    \ })
    let winid = popup_create(['hello', 'bright', 'world'], {
	    \ 'minwidth': 20,
	    \ })
    call setwinvar(winid, "&signcolumn", "yes")
    let winbufnr = winbufnr(winid)

    " add sign to current buffer, shows
    call sign_place(1, 'Selected', 'Current', bufnr('%'), {'lnum': 1})
    " add sign to current buffer, does not show
    call sign_place(2, 'PopUpSelected', 'Other', bufnr('%'), {'lnum': 2})

    " add sign to popup buffer, shows
    call sign_place(3, 'PopUpSelected', 'Other', winbufnr, {'lnum': 1})
    " add sign to popup buffer, does not show
    call sign_place(4, 'Selected', 'Current', winbufnr, {'lnum': 2})

    func SetOptions()
      call setwinvar(g:winid, '&number', 1)
      call setwinvar(g:winid, '&foldcolumn', 2)
      call popup_settext(g:winid, 'a longer line to check the width')
    endfunc
  END
  call writefile(lines, 'XtestPopupSign', 'D')

  let buf = RunVimInTerminal('-S XtestPopupSign', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popupwin_sign_1', {})

  " set more options to check the width is adjusted
  call term_sendkeys(buf, ":call SetOptions()\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_sign_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popupwin_bufnr()
  let popwin = popup_create(['blah'], #{})
  let popbuf = winbufnr(popwin)
  split asdfasdf
  let newbuf = bufnr()
  call assert_true(newbuf > popbuf, 'New buffer number is higher')
  call assert_equal(newbuf, bufnr('$'))
  call popup_clear()
  let popwin = popup_create(['blah'], #{})
  " reuses previous buffer number
  call assert_equal(popbuf, winbufnr(popwin))
  call assert_equal(newbuf, bufnr('$'))

  call popup_clear()
  bwipe!
endfunc

func Test_popupwin_filter_input_multibyte()
  func MyPopupFilter(winid, c)
    let g:bytes = range(a:c->strlen())->map({i -> char2nr(a:c[i])})
    return 0
  endfunc
  let winid = popup_create('', #{mapping: 0, filter: 'MyPopupFilter'})

  " UTF-8: E3 80 80, including K_SPECIAL(0x80)
  call feedkeys("\u3000", 'xt')
  call assert_equal([0xe3, 0x80, 0x80], g:bytes)

  " UTF-8: E3 80 9B, including CSI(0x9B)
  call feedkeys("\u301b", 'xt')
  call assert_equal([0xe3, 0x80, 0x9b], g:bytes)

  if has('unix')
    " with modifyOtherKeys <M-S-a> does not include a modifier sequence
    if has('gui_running')
      call feedkeys("\x9b\xfc\x08A", 'Lx!')
    else
      call feedkeys("\<Esc>[27;4;65~", 'Lx!')
    endif
    call assert_equal([0xc3, 0x81], g:bytes)
  endif

  call popup_clear()
  delfunc MyPopupFilter
  unlet g:bytes
endfunc

func Test_popupwin_filter_close_ctrl_c()
  CheckScreendump

  let lines =<< trim END
      vsplit
      set laststatus=2
      set statusline=%!Statusline()

      function Statusline() abort
	  return '%<%f %h%m%r%=%-14.(%l,%c%V%) %P'
      endfunction

      call popup_create('test test test test...', {'filter': {-> 0}})
  END
  call writefile(lines, 'XtestPopupCtrlC', 'D')

  let buf = RunVimInTerminal('-S XtestPopupCtrlC', #{rows: 10})

  call term_sendkeys(buf, "\<C-C>")
  call VerifyScreenDump(buf, 'Test_popupwin_ctrl_c', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popupwin_filter_close_wrong_name()
  CheckScreendump

  let lines =<< trim END
      call popup_create('one two three...', {'filter': 'NoSuchFunc'})
  END
  call writefile(lines, 'XtestPopupWrongName', 'D')

  let buf = RunVimInTerminal('-S XtestPopupWrongName', #{rows: 10})

  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_popupwin_wrong_name', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popupwin_filter_close_three_errors()
  CheckScreendump

  let lines =<< trim END
      set cmdheight=2
      call popup_create('one two three...', {'filter': 'filter'})
  END
  call writefile(lines, 'XtestPopupThreeErrors', 'D')

  let buf = RunVimInTerminal('-S XtestPopupThreeErrors', #{rows: 10})

  call term_sendkeys(buf, "jj")
  call VerifyScreenDump(buf, 'Test_popupwin_three_errors_1', {})
  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_popupwin_three_errors_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_popupwin_latin1_encoding()
  CheckScreendump
  CheckUnix

  " When 'encoding' is a single-byte encoding a terminal window will mess up
  " the display.  Check that showing a popup on top of that doesn't crash.
  let lines =<< trim END
      set encoding=latin1
      terminal cat Xmultibyte
      call popup_create(['one', 'two', 'three', 'four'], #{line: 1, col: 10})
      redraw
      " wait for "cat" to finish
      while execute('ls!') !~ 'finished'
	sleep 10m
      endwhile
      echo "Done"
  END
  call writefile(lines, 'XtestPopupLatin', 'D')
  call writefile([repeat("\u3042 ", 120)], 'Xmultibyte', 'D')

  let buf = RunVimInTerminal('-S XtestPopupLatin', #{rows: 10})
  call WaitForAssert({-> assert_match('Done', term_getline(buf, 10))})

  call term_sendkeys(buf, ":q\<CR>")
  call StopVimInTerminal(buf)
endfunc

func Test_popupwin_atcursor_far_right()
  new

  " this was getting stuck
  set signcolumn=yes
  call setline(1, repeat('=', &columns))
  normal! ggg$
  let winid = popup_atcursor(repeat('x', 500), #{moved: 'any', border: []})

  " 'signcolumn' was getting reset
  call setwinvar(winid, '&signcolumn', 'yes')
  call popup_setoptions(winid, {'zindex': 1000})
  call assert_equal('yes', getwinvar(winid, '&signcolumn'))

  call popup_close(winid)
  bwipe!
  set signcolumn&
endfunc

func Test_popupwin_splitmove()
  vsplit
  let win2 = win_getid()
  let popup_winid = popup_dialog('hello', {})
  call assert_fails('call win_splitmove(popup_winid, win2)', 'E957:')
  call assert_fails('call win_splitmove(win2, popup_winid)', 'E957:')

  call popup_clear()
  bwipe
endfunc

func Test_popupwin_exiting_terminal()
  CheckFeature terminal

  " Tests that when creating a popup right after closing a terminal window does
  " not make the popup the current window.
  let winid = win_getid()
  try
    augroup Test_popupwin_exiting_terminal
      autocmd!
      autocmd WinEnter * :call popup_create('test', {})
    augroup END
    let bnr = term_start(&shell, #{term_finish: 'close'})
    call term_sendkeys(bnr, "exit\r\n")
    call WaitForAssert({-> assert_equal(winid, win_getid())})
  finally
    call popup_clear(1)
    augroup Test_popupwin_exiting_terminal
      autocmd!
    augroup END
  endtry
endfunc

func Test_popup_filter_menu()
  let colors = ['red', 'green', 'blue']
  call popup_menu(colors, #{callback: {_, result -> assert_equal('green', colors[result - 1])}})
  call feedkeys("\<c-n>\<c-n>\<c-p>\<cr>", 'xt')
endfunc

func Test_popup_getoptions_other_tab()
  new
  call setline(1, 'some text')
  call prop_type_add('textprop', {})
  call prop_add(1, 1, #{type: 'textprop', length: 1})
  let id = popup_create('TEST', #{textprop: 'textprop', highlight: 'ErrorMsg', tabpage: 1})
  tab sp
  call assert_equal(['textprop', 'textpropid', 'textpropwin'], popup_getoptions(id)->keys()->filter({_, v -> v =~ 'textprop'}))

  tabclose
  call popup_close(id)
  bwipe!
  call prop_type_delete('textprop')
endfunc


func Test_popup_setoptions_other_tab()
  new Xpotfile
  let winid = win_getid()
  call setline(1, 'some text')
  call prop_type_add('textprop', {})
  call prop_add(1, 1, #{type: 'textprop', length: 1})
  let id = popup_create('TEST', #{textprop: 'textprop'})
  tab sp
  call popup_setoptions(id, #{textprop: 'textprop', textpropwin: winid})
  call assert_equal(winid, popup_getoptions(id).textpropwin)

  tabclose
  call popup_close(id)
  bwipe! Xpotfile
  call prop_type_delete('textprop')
endfunc

func Test_popup_prop_not_visible()
  CheckScreendump

  let lines =<< trim END
      vim9script
      set nowrap stal=2
      rightbelow :31vnew
      setline(1, ['', 'some text', '', 'other text'])
      prop_type_add('someprop', {})
      prop_add(2, 9, {type: 'someprop', length: 5})
      g:some_id = popup_create('attached to "some"', {
          textprop: 'someprop',
          highlight: 'ErrorMsg',
          line: -1,
          wrap: false,
          fixed: true,
          })
      prop_type_add('otherprop', {})
      prop_add(4, 10, {type: 'otherprop', length: 5})
      popup_create('attached to "other"', {
          textprop: 'otherprop',
          highlight: 'ErrorMsg',
          line: -1,
          wrap: false,
          fixed: false,
          })
  END
  call writefile(lines, 'XtestPropNotVisible', 'D')
  let buf = RunVimInTerminal('-S XtestPropNotVisible', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popup_prop_not_visible_01', {})

  " check that hiding and unhiding the popup works
  call term_sendkeys(buf, ":call popup_hide(g:some_id)\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_prop_not_visible_01a', {})
  call term_sendkeys(buf, ":call popup_show(g:some_id)\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_prop_not_visible_01b', {})

  call term_sendkeys(buf, ":vert resize -14\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_prop_not_visible_02', {})

  call term_sendkeys(buf, ":vert resize -8\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_prop_not_visible_03', {})

  " clean up
  call StopVimInTerminal(buf)
endfunction

func Test_bufdel_skips_popupwin_buffer()
    let id = popup_create("Some text", {})
    %bd
    call popup_close(id)
endfunc

func Test_term_popup_bufline()
  " very specific situation where a non-existing buffer line is used, leading
  " to an ml_get error
  CheckScreendump

  let lines =<< trim END
      vim9script
      &scrolloff = 5
      term_start('seq 1 5', {term_finish: 'open'})
      timer_start(50, (_) => {
	  set cpoptions&vim
	  var buf = popup_create([], {})->winbufnr()
	  appendbufline(buf, 0, range(5))
      })
  END
  call writefile(lines, 'XtestTermPopup', 'D')
  let buf = RunVimInTerminal('-S XtestTermPopup', #{rows: 15})
  call VerifyScreenDump(buf, 'Test_term_popup_bufline', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popupwin_with_error()
  CheckScreendump

  let lines =<< trim END
  let options = {'border': 'ERROR', 'line': 1, 'col': 1, 'minwidth': &columns, 'title': 'TITLE'}

  END
  "call popup_create('Hello world!', options)
  call writefile(lines, 'XtestPopupError', 'D')
  let buf = RunVimInTerminal('-S XtestPopupError', {})
  call term_sendkeys(buf, ":call popup_create('Hello world!', options)\<CR>")
  call VerifyScreenDump(buf, 'Test_popupwin_with_error_1', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_popup_close_callback_recursive()
  set maxfuncdepth=20
  " this invokes the callback recursively
  let winid = popup_create('something', #{callback: 'popup_close'})
  redraw
  call assert_fails('call popup_close(winid)', 'E169:')

  set maxfuncdepth&
endfunc

func Test_popupwin_setbufvar_changing_window_view()
  " Test for Github Issue https://github.com/vim/vim/issues/13863
  " using setbufvar(buf, '&option') should not scroll
  " the current window
  20new
  call append(0, range(1, 25))
  setlocal scrollbind
  norm! G
  let topline = winsaveview()['topline']
  call setbufvar(winbufnr(popup_atcursor(['foobar'], {})), '&syntax', 'python')
  " close popup
  call popup_clear()
  call assert_equal(topline, winsaveview()['topline'])

  " clean up
  bw!
endfunc

" vim: shiftwidth=2 sts=2
