" Tests for popup windows for text properties

source check.vim
CheckFeature popupwin
CheckFeature textprop

source screendump.vim
CheckScreendump

func Test_textprop_popup()
  let lines =<< trim END
	call setline(1, range(1, 100))
	call setline(50, 'some text to work with')
	50
	normal zz
	set scrolloff=0
	call prop_type_add('popupMarker', #{highlight: 'DiffAdd', bufnr: bufnr('%')})
	call prop_add(50, 11, #{
		\ length: 7,
		\ type: 'popupMarker',
		\ bufnr: bufnr('%'),
		\ })
	let winid = popup_create('the text', #{
	      \ pos: 'botleft',
	      \ textprop: 'popupMarker',
	      \ border: [],
	      \ padding: [0,1,0,1],
	      \ close: 'click',
	      \ })
  END
  call writefile(lines, 'XtestTextpropPopup', 'D')
  let buf = RunVimInTerminal('-S XtestTextpropPopup', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_popup_textprop_01', {})

  call term_sendkeys(buf, "zt")
  call VerifyScreenDump(buf, 'Test_popup_textprop_02', {})

  call term_sendkeys(buf, "zzIawe\<Esc>")
  call VerifyScreenDump(buf, 'Test_popup_textprop_03', {})

  call term_sendkeys(buf, "0dw")
  call VerifyScreenDump(buf, 'Test_popup_textprop_04', {})

  call term_sendkeys(buf, "Oinserted\<Esc>")
  call VerifyScreenDump(buf, 'Test_popup_textprop_05', {})

  call term_sendkeys(buf, "k2dd")
  call VerifyScreenDump(buf, 'Test_popup_textprop_06', {})

  call term_sendkeys(buf, "4\<C-E>")
  call VerifyScreenDump(buf, 'Test_popup_textprop_07', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_textprop_popup_corners()
  let lines =<< trim END
	call setline(1, range(1, 100))
	call setline(50, 'now working with some longer text here')
	50
	normal zz
	set scrolloff=0
	call prop_type_add('popupMarker', #{highlight: 'DiffAdd'})
	call prop_add(50, 23, #{
		\ length: 6,
		\ type: 'popupMarker',
		\ })
	let winid = popup_create('bottom left', #{
	      \ pos: 'botleft',
	      \ textprop: 'popupMarker',
	      \ textpropwin: win_getid(),
	      \ padding: [0,1,0,1],
	      \ })
	let winid = popup_create('bottom right', #{
	      \ pos: 'botright',
	      \ textprop: 'popupMarker',
	      \ border: [],
	      \ padding: [0,1,0,1],
	      \ })
	let winid = popup_create('top left', #{
	      \ pos: 'topleft',
	      \ textprop: 'popupMarker',
	      \ border: [],
	      \ padding: [0,1,0,1],
	      \ })
	let winid = popup_create('top right', #{
	      \ pos: 'topright',
	      \ textprop: 'popupMarker',
	      \ padding: [0,1,0,1],
	      \ })
  END
  call writefile(lines, 'XtestTextpropPopupCorners', 'D')
  let buf = RunVimInTerminal('-S XtestTextpropPopupCorners', #{rows: 12})
  call VerifyScreenDump(buf, 'Test_popup_textprop_corn_1', {})

  call term_sendkeys(buf, "0dw")
  call VerifyScreenDump(buf, 'Test_popup_textprop_corn_2', {})

  call term_sendkeys(buf, "46Goextra\<Esc>")
  call VerifyScreenDump(buf, 'Test_popup_textprop_corn_3', {})

  call term_sendkeys(buf, "u")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_textprop_corn_4', {})

  call term_sendkeys(buf, ":vsplit foo\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_textprop_corn_5', {})

  call term_sendkeys(buf, ":only!\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_textprop_corn_6', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_textprop_popup_offsets()
  let lines =<< trim END
	call setline(1, range(1, 100))
	call setline(50, 'now working with some longer text here')
	50
	normal zz
	set scrolloff=0
	call prop_type_add('popupMarker', #{highlight: 'DiffAdd'})
	call prop_add(50, 23, #{
		\ length: 6,
		\ type: 'popupMarker',
		\ })
	let winid = popup_create('bottom left', #{
	      \ pos: 'botleft',
	      \ line: -1,
	      \ col: 2,
	      \ textprop: 'popupMarker',
	      \ padding: [0,1,0,1],
	      \ })
	let winid = popup_create('bottom right', #{
	      \ pos: 'botright',
	      \ line: -1,
	      \ col: -2,
	      \ textprop: 'popupMarker',
	      \ border: [],
	      \ padding: [0,1,0,1],
	      \ })
	let winid = popup_create('top left', #{
	      \ pos: 'topleft',
	      \ line: 1,
	      \ col: 2,
	      \ textprop: 'popupMarker',
	      \ border: [],
	      \ padding: [0,1,0,1],
	      \ })
	let winid = popup_create('top right', #{
	      \ pos: 'topright',
	      \ line: 1,
	      \ col: -2,
	      \ textprop: 'popupMarker',
	      \ padding: [0,1,0,1],
	      \ })
  END
  call writefile(lines, 'XtestTextpropPopupOffset', 'D')
  let buf = RunVimInTerminal('-S XtestTextpropPopupOffset', #{rows: 12})
  call VerifyScreenDump(buf, 'Test_popup_textprop_off_1', {})

  " test that removing the text property closes the popups
  call term_sendkeys(buf, ":call prop_clear(50)\<CR>")
  call VerifyScreenDump(buf, 'Test_popup_textprop_off_2', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc


" vim: shiftwidth=2 sts=2
