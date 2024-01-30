" Tests for ":highlight" and highlighting.

source view_util.vim
source screendump.vim
source check.vim
source script_util.vim
import './vim9.vim' as v9

func ClearDict(d)
  for k in keys(a:d)
    call remove(a:d, k)
  endfor
endfunc

func Test_highlight()
  " basic test if ":highlight" doesn't crash
  highlight
  hi Search

  " test setting colors.
  " test clearing one color and all doesn't generate error or warning
  silent! hi NewGroup term=bold cterm=italic ctermfg=DarkBlue ctermbg=Grey gui= guifg=#00ff00 guibg=Cyan
  silent! hi Group2 term= cterm=
  hi Group3 term=underline cterm=bold

  let res = split(execute("hi NewGroup"), "\n")[0]
  " filter ctermfg and ctermbg, the numbers depend on the terminal
  let res = substitute(res, 'ctermfg=\d*', 'ctermfg=2', '')
  let res = substitute(res, 'ctermbg=\d*', 'ctermbg=3', '')
  call assert_equal("NewGroup       xxx term=bold cterm=italic ctermfg=2 ctermbg=3",
				\ res)
  call assert_equal("Group2         xxx cleared",
				\ split(execute("hi Group2"), "\n")[0])
  call assert_equal("Group3         xxx term=underline cterm=bold",
				\ split(execute("hi Group3"), "\n")[0])

  hi clear NewGroup
  call assert_equal("NewGroup       xxx cleared",
				\ split(execute("hi NewGroup"), "\n")[0])
  call assert_equal("Group2         xxx cleared",
				\ split(execute("hi Group2"), "\n")[0])
  hi Group2 NONE
  call assert_equal("Group2         xxx cleared",
				\ split(execute("hi Group2"), "\n")[0])
  hi clear
  call assert_equal("Group3         xxx cleared",
				\ split(execute("hi Group3"), "\n")[0])
  call assert_fails("hi Crash term='asdf", "E475:")

  if has('gui_running')
    call assert_fails('hi NotUsed guibg=none', 'E1361:')
  endif
endfunc

func HighlightArgs(name)
  return 'hi ' . substitute(split(execute('hi ' . a:name), '\n')[0], '\<xxx\>', '', '')
endfunc

func IsColorable()
  return has('gui_running') || str2nr(&t_Co) >= 8
endfunc

func HiCursorLine()
  let hiCursorLine = HighlightArgs('CursorLine')
  if has('gui_running')
    let guibg = matchstr(hiCursorLine, 'guibg=\w\+')
    let hi_ul = 'hi CursorLine gui=underline guibg=NONE'
    let hi_bg = 'hi CursorLine gui=NONE ' . guibg
  else
    let hi_ul = 'hi CursorLine cterm=underline ctermbg=NONE'
    let hi_bg = 'hi CursorLine cterm=NONE ctermbg=Gray'
  endif
  return [hiCursorLine, hi_ul, hi_bg]
endfunc

func Check_lcs_eol_attrs(attrs, row, col)
  let save_lcs = &lcs
  set list

  call assert_equal(a:attrs, ScreenAttrs(a:row, a:col)[0])

  set nolist
  let &lcs = save_lcs
endfunc

func Test_highlight_eol_with_cursorline()
  let [hiCursorLine, hi_ul, hi_bg] = HiCursorLine()

  call NewWindow('topleft 5', 20)
  call setline(1, 'abcd')
  call matchadd('Search', '\n')

  " expected:
  " 'abcd      '
  "  ^^^^ ^^^^^   no highlight
  "      ^        'Search' highlight
  let attrs0 = ScreenAttrs(1, 10)[0]
  call assert_equal(repeat([attrs0[0]], 4), attrs0[0:3])
  call assert_equal(repeat([attrs0[0]], 5), attrs0[5:9])
  call assert_notequal(attrs0[0], attrs0[4])

  setlocal cursorline

  " underline
  exe hi_ul

  " expected:
  " 'abcd      '
  "  ^^^^         underline
  "      ^        'Search' highlight with underline
  "       ^^^^^   underline
  let attrs = ScreenAttrs(1, 10)[0]
  call assert_equal(repeat([attrs[0]], 4), attrs[0:3])
  call assert_equal([attrs[4]] + repeat([attrs[5]], 5), attrs[4:9])
  call assert_notequal(attrs[0], attrs[4])
  call assert_notequal(attrs[4], attrs[5])
  call assert_notequal(attrs0[0], attrs[0])
  call assert_notequal(attrs0[4], attrs[4])
  call Check_lcs_eol_attrs(attrs, 1, 10)

  if IsColorable()
    " bg-color
    exe hi_bg

    " expected:
    " 'abcd      '
    "  ^^^^         bg-color of 'CursorLine'
    "      ^        'Search' highlight
    "       ^^^^^   bg-color of 'CursorLine'
    let attrs = ScreenAttrs(1, 10)[0]
    call assert_equal(repeat([attrs[0]], 4), attrs[0:3])
    call assert_equal(repeat([attrs[5]], 5), attrs[5:9])
    call assert_equal(attrs0[4], attrs[4])
    call assert_notequal(attrs[0], attrs[4])
    call assert_notequal(attrs[4], attrs[5])
    call assert_notequal(attrs0[0], attrs[0])
    call assert_notequal(attrs0[5], attrs[5])
    call Check_lcs_eol_attrs(attrs, 1, 10)
  endif

  call CloseWindow()
  exe hiCursorLine
endfunc

func Test_highlight_eol_with_cursorline_vertsplit()
  let [hiCursorLine, hi_ul, hi_bg] = HiCursorLine()

  call NewWindow('topleft 5', 5)
  call setline(1, 'abcd')
  call matchadd('Search', '\n')

  let expected = "abcd |abcd     "
  let actual = ScreenLines(1, 15)[0]
  call assert_equal(expected, actual)

  " expected:
  " 'abcd |abcd     '
  "  ^^^^  ^^^^^^^^^   no highlight
  "      ^             'Search' highlight
  "       ^            'VertSplit' highlight
  let attrs0 = ScreenAttrs(1, 15)[0]
  call assert_equal(repeat([attrs0[0]], 4), attrs0[0:3])
  call assert_equal(repeat([attrs0[0]], 9), attrs0[6:14])
  call assert_notequal(attrs0[0], attrs0[4])
  call assert_notequal(attrs0[0], attrs0[5])
  call assert_notequal(attrs0[4], attrs0[5])

  setlocal cursorline

  " expected:
  " 'abcd |abcd     '
  "  ^^^^              underline
  "      ^             'Search' highlight with underline
  "       ^            'VertSplit' highlight
  "        ^^^^^^^^^   no highlight

  " underline
  exe hi_ul

  let actual = ScreenLines(1, 15)[0]
  call assert_equal(expected, actual)

  let attrs = ScreenAttrs(1, 15)[0]
  call assert_equal(repeat([attrs[0]], 4), attrs[0:3])
  call assert_equal(repeat([attrs[6]], 9), attrs[6:14])
  call assert_equal(attrs0[5:14], attrs[5:14])
  call assert_notequal(attrs[0], attrs[4])
  call assert_notequal(attrs[0], attrs[5])
  call assert_notequal(attrs[0], attrs[6])
  call assert_notequal(attrs[4], attrs[5])
  call assert_notequal(attrs[5], attrs[6])
  call assert_notequal(attrs0[0], attrs[0])
  call assert_notequal(attrs0[4], attrs[4])
  call Check_lcs_eol_attrs(attrs, 1, 15)

  if IsColorable()
    " bg-color
    exe hi_bg

    let actual = ScreenLines(1, 15)[0]
    call assert_equal(expected, actual)

    let attrs = ScreenAttrs(1, 15)[0]
    call assert_equal(repeat([attrs[0]], 4), attrs[0:3])
    call assert_equal(repeat([attrs[6]], 9), attrs[6:14])
    call assert_equal(attrs0[5:14], attrs[5:14])
    call assert_notequal(attrs[0], attrs[4])
    call assert_notequal(attrs[0], attrs[5])
    call assert_notequal(attrs[0], attrs[6])
    call assert_notequal(attrs[4], attrs[5])
    call assert_notequal(attrs[5], attrs[6])
    call assert_notequal(attrs0[0], attrs[0])
    call assert_equal(attrs0[4], attrs[4])
    call Check_lcs_eol_attrs(attrs, 1, 15)
  endif

  call CloseWindow()
  exe hiCursorLine
endfunc

func Test_highlight_eol_with_cursorline_rightleft()
  CheckFeature rightleft

  let [hiCursorLine, hi_ul, hi_bg] = HiCursorLine()

  call NewWindow('topleft 5', 10)
  setlocal rightleft
  call setline(1, 'abcd')
  call matchadd('Search', '\n')
  let attrs0 = ScreenAttrs(1, 10)[0]

  setlocal cursorline

  " underline
  exe hi_ul

  " expected:
  " '      dcba'
  "        ^^^^   underline
  "       ^       'Search' highlight with underline
  "  ^^^^^        underline
  let attrs = ScreenAttrs(1, 10)[0]
  call assert_equal(repeat([attrs[9]], 4), attrs[6:9])
  call assert_equal(repeat([attrs[4]], 5) + [attrs[5]], attrs[0:5])
  call assert_notequal(attrs[9], attrs[5])
  call assert_notequal(attrs[4], attrs[5])
  call assert_notequal(attrs0[9], attrs[9])
  call assert_notequal(attrs0[5], attrs[5])
  call Check_lcs_eol_attrs(attrs, 1, 10)

  if IsColorable()
    " bg-color
    exe hi_bg

    " expected:
    " '      dcba'
    "        ^^^^   bg-color of 'CursorLine'
    "       ^       'Search' highlight
    "  ^^^^^        bg-color of 'CursorLine'
    let attrs = ScreenAttrs(1, 10)[0]
    call assert_equal(repeat([attrs[9]], 4), attrs[6:9])
    call assert_equal(repeat([attrs[4]], 5), attrs[0:4])
    call assert_equal(attrs0[5], attrs[5])
    call assert_notequal(attrs[9], attrs[5])
    call assert_notequal(attrs[5], attrs[4])
    call assert_notequal(attrs0[9], attrs[9])
    call assert_notequal(attrs0[4], attrs[4])
    call Check_lcs_eol_attrs(attrs, 1, 10)
  endif

  call CloseWindow()
  exe hiCursorLine
endfunc

func Test_highlight_eol_with_cursorline_linewrap()
  let [hiCursorLine, hi_ul, hi_bg] = HiCursorLine()

  call NewWindow('topleft 5', 10)
  call setline(1, [repeat('a', 51) . 'bcd', ''])
  call matchadd('Search', '\n')

  setlocal wrap
  normal! gg$
  let attrs0 = ScreenAttrs(5, 10)[0]
  setlocal cursorline

  " underline
  exe hi_ul

  " expected:
  " 'abcd      '
  "  ^^^^         underline
  "      ^        'Search' highlight with underline
  "       ^^^^^   underline
  let attrs = ScreenAttrs(5, 10)[0]
  call assert_equal(repeat([attrs[0]], 4), attrs[0:3])
  call assert_equal([attrs[4]] + repeat([attrs[5]], 5), attrs[4:9])
  call assert_notequal(attrs[0], attrs[4])
  call assert_notequal(attrs[4], attrs[5])
  call assert_notequal(attrs0[0], attrs[0])
  call assert_notequal(attrs0[4], attrs[4])
  call Check_lcs_eol_attrs(attrs, 5, 10)

  if IsColorable()
    " bg-color
    exe hi_bg

    " expected:
    " 'abcd      '
    "  ^^^^         bg-color of 'CursorLine'
    "      ^        'Search' highlight
    "       ^^^^^   bg-color of 'CursorLine'
    let attrs = ScreenAttrs(5, 10)[0]
    call assert_equal(repeat([attrs[0]], 4), attrs[0:3])
    call assert_equal(repeat([attrs[5]], 5), attrs[5:9])
    call assert_equal(attrs0[4], attrs[4])
    call assert_notequal(attrs[0], attrs[4])
    call assert_notequal(attrs[4], attrs[5])
    call assert_notequal(attrs0[0], attrs[0])
    call assert_notequal(attrs0[5], attrs[5])
    call Check_lcs_eol_attrs(attrs, 5, 10)
  endif

  setlocal nocursorline nowrap
  normal! gg$
  let attrs0 = ScreenAttrs(1, 10)[0]
  setlocal cursorline

  " underline
  exe hi_ul

  " expected:
  " 'aaabcd    '
  "  ^^^^^^       underline
  "        ^      'Search' highlight with underline
  "         ^^^   underline
  let attrs = ScreenAttrs(1, 10)[0]
  call assert_equal(repeat([attrs[0]], 6), attrs[0:5])
  call assert_equal([attrs[6]] + repeat([attrs[7]], 3), attrs[6:9])
  call assert_notequal(attrs[0], attrs[6])
  call assert_notequal(attrs[6], attrs[7])
  call assert_notequal(attrs0[0], attrs[0])
  call assert_notequal(attrs0[6], attrs[6])
  call Check_lcs_eol_attrs(attrs, 1, 10)

  if IsColorable()
    " bg-color
    exe hi_bg

    " expected:
    " 'aaabcd    '
    "  ^^^^^^       bg-color of 'CursorLine'
    "        ^      'Search' highlight
    "         ^^^   bg-color of 'CursorLine'
    let attrs = ScreenAttrs(1, 10)[0]
    call assert_equal(repeat([attrs[0]], 6), attrs[0:5])
    call assert_equal(repeat([attrs[7]], 3), attrs[7:9])
    call assert_equal(attrs0[6], attrs[6])
    call assert_notequal(attrs[0], attrs[6])
    call assert_notequal(attrs[6], attrs[7])
    call assert_notequal(attrs0[0], attrs[0])
    call assert_notequal(attrs0[7], attrs[7])
    call Check_lcs_eol_attrs(attrs, 1, 10)
  endif

  call CloseWindow()
  exe hiCursorLine
endfunc

func Test_highlight_eol_with_cursorline_sign()
  CheckFeature signs

  let [hiCursorLine, hi_ul, hi_bg] = HiCursorLine()

  call NewWindow('topleft 5', 10)
  call setline(1, 'abcd')
  call matchadd('Search', '\n')

  sign define Sign text=>>
  exe 'sign place 1 line=1 name=Sign buffer=' . bufnr('')
  let attrs0 = ScreenAttrs(1, 10)[0]
  setlocal cursorline

  " underline
  exe hi_ul

  " expected:
  " '>>abcd    '
  "  ^^           sign
  "    ^^^^       underline
  "        ^      'Search' highlight with underline
  "         ^^^   underline
  let attrs = ScreenAttrs(1, 10)[0]
  call assert_equal(repeat([attrs[2]], 4), attrs[2:5])
  call assert_equal([attrs[6]] + repeat([attrs[7]], 3), attrs[6:9])
  call assert_notequal(attrs[2], attrs[6])
  call assert_notequal(attrs[6], attrs[7])
  call assert_notequal(attrs0[2], attrs[2])
  call assert_notequal(attrs0[6], attrs[6])
  call Check_lcs_eol_attrs(attrs, 1, 10)

  if IsColorable()
    " bg-color
    exe hi_bg

    " expected:
    " '>>abcd    '
    "  ^^           sign
    "    ^^^^       bg-color of 'CursorLine'
    "        ^      'Search' highlight
    "         ^^^   bg-color of 'CursorLine'
    let attrs = ScreenAttrs(1, 10)[0]
    call assert_equal(repeat([attrs[2]], 4), attrs[2:5])
    call assert_equal(repeat([attrs[7]], 3), attrs[7:9])
    call assert_equal(attrs0[6], attrs[6])
    call assert_notequal(attrs[2], attrs[6])
    call assert_notequal(attrs[6], attrs[7])
    call assert_notequal(attrs0[2], attrs[2])
    call assert_notequal(attrs0[7], attrs[7])
    call Check_lcs_eol_attrs(attrs, 1, 10)
  endif

  sign unplace 1
  call CloseWindow()
  exe hiCursorLine
endfunc

func Test_highlight_eol_with_cursorline_breakindent()
  CheckFeature linebreak

  let [hiCursorLine, hi_ul, hi_bg] = HiCursorLine()

  call NewWindow('topleft 5', 10)
  set showbreak=xxx
  setlocal breakindent breakindentopt=min:0,shift:1 showbreak=>
  call setline(1, ' ' . repeat('a', 9) . 'bcd')
  call matchadd('Search', '\n')
  let attrs0 = ScreenAttrs(2, 10)[0]
  setlocal cursorline

  " underline
  exe hi_ul

  " expected:
  " '  >bcd    '
  "  ^^^          breakindent and showbreak
  "     ^^^       underline
  "        ^      'Search' highlight with underline
  "         ^^^   underline
  let attrs = ScreenAttrs(2, 10)[0]
  call assert_equal(repeat([attrs[0]], 2), attrs[0:1])
  call assert_equal(repeat([attrs[3]], 3), attrs[3:5])
  call assert_equal([attrs[6]] + repeat([attrs[7]], 3), attrs[6:9])
  call assert_equal(attrs0[0], attrs[0])
  call assert_notequal(attrs[0], attrs[2])
  call assert_notequal(attrs[2], attrs[3])
  call assert_notequal(attrs[3], attrs[6])
  call assert_notequal(attrs[6], attrs[7])
  call assert_notequal(attrs0[2], attrs[2])
  call assert_notequal(attrs0[3], attrs[3])
  call assert_notequal(attrs0[6], attrs[6])
  call Check_lcs_eol_attrs(attrs, 2, 10)

  if IsColorable()
    " bg-color
    exe hi_bg

    " expected:
    " '  >bcd    '
    "  ^^^          breakindent and showbreak
    "     ^^^       bg-color of 'CursorLine'
    "        ^      'Search' highlight
    "         ^^^   bg-color of 'CursorLine'
    let attrs = ScreenAttrs(2, 10)[0]
    call assert_equal(repeat([attrs[0]], 2), attrs[0:1])
    call assert_equal(repeat([attrs[3]], 3), attrs[3:5])
    call assert_equal(repeat([attrs[7]], 3), attrs[7:9])
    call assert_equal(attrs0[0], attrs[0])
    call assert_equal(attrs0[6], attrs[6])
    call assert_notequal(attrs[0], attrs[2])
    call assert_notequal(attrs[2], attrs[3])
    call assert_notequal(attrs[3], attrs[6])
    call assert_notequal(attrs[6], attrs[7])
    call assert_notequal(attrs0[2], attrs[2])
    call assert_notequal(attrs0[3], attrs[3])
    call assert_notequal(attrs0[7], attrs[7])
    call Check_lcs_eol_attrs(attrs, 2, 10)
  endif

  call CloseWindow()
  set showbreak=
  setlocal showbreak=
  exe hiCursorLine
endfunc

func Test_highlight_eol_on_diff()
  call setline(1, ['abcd', ''])
  call matchadd('Search', '\n')
  let attrs0 = ScreenAttrs(1, 10)[0]

  diffthis
  botright new
  diffthis

  " expected:
  " '  abcd    '
  "  ^^           sign
  "    ^^^^ ^^^   'DiffAdd' highlight
  "        ^      'Search' highlight
  let attrs = ScreenAttrs(1, 10)[0]
  call assert_equal(repeat([attrs[0]], 2), attrs[0:1])
  call assert_equal(repeat([attrs[2]], 4), attrs[2:5])
  call assert_equal(repeat([attrs[2]], 3), attrs[7:9])
  call assert_equal(attrs0[4], attrs[6])
  call assert_notequal(attrs[0], attrs[2])
  call assert_notequal(attrs[0], attrs[6])
  call assert_notequal(attrs[2], attrs[6])
  call Check_lcs_eol_attrs(attrs, 1, 10)

  bwipe!
  diffoff
endfunc

func Test_termguicolors()
  CheckOption termguicolors
  if has('vtp') && !has('vcon') && !has('gui_running')
    " Win32: 'guicolors' doesn't work without virtual console.
    call assert_fails('set termguicolors', 'E954:')
    return
  endif

  " Basic test that setting 'termguicolors' works with one color.
  set termguicolors
  redraw
  set t_Co=1
  redraw
  set t_Co=0
  redraw
endfunc

func Test_cursorline_after_yank()
  CheckScreendump

  call writefile([
	\ 'set cul rnu',
	\ 'call setline(1, ["","1","2","3",""])',
	\ ], 'Xtest_cursorline_yank', 'D')
  let buf = RunVimInTerminal('-S Xtest_cursorline_yank', {'rows': 8})
  call TermWait(buf)
  call term_sendkeys(buf, "Gy3k")
  call TermWait(buf)
  call term_sendkeys(buf, "jj")

  call VerifyScreenDump(buf, 'Test_cursorline_yank_01', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" test for issue #4862
func Test_put_before_cursorline()
  new
  only!
  call setline(1, 'A')
  redraw
  let std_attr = screenattr(1, 1)
  set cursorline
  redraw
  let cul_attr = screenattr(1, 1)
  normal yyP
  redraw
  " Line 1 has cursor so it should be highlighted with CursorLine.
  call assert_equal(cul_attr, screenattr(1, 1))
  " And CursorLine highlighting from the second line should be gone.
  call assert_equal(std_attr, screenattr(2, 1))
  set nocursorline
  bwipe!
endfunc

func Test_cursorline_with_visualmode()
  CheckScreendump

  call writefile([
	\ 'set cul',
	\ 'call setline(1, repeat(["abc"], 50))',
	\ ], 'Xtest_cursorline_with_visualmode', 'D')
  let buf = RunVimInTerminal('-S Xtest_cursorline_with_visualmode', {'rows': 12})
  call TermWait(buf)
  call term_sendkeys(buf, "V\<C-f>kkkjk")

  call VerifyScreenDump(buf, 'Test_cursorline_with_visualmode_01', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_cursorcolumn_insert_on_tab()
  CheckScreendump

  let lines =<< trim END
    call setline(1, ['123456789', "a\tb"])
    set cursorcolumn
    call cursor(2, 2)
  END
  call writefile(lines, 'Xcuc_insert_on_tab', 'D')

  let buf = RunVimInTerminal('-S Xcuc_insert_on_tab', #{rows: 8})
  call TermWait(buf)
  call VerifyScreenDump(buf, 'Test_cursorcolumn_insert_on_tab_1', {})

  call term_sendkeys(buf, 'i')
  call TermWait(buf)
  call VerifyScreenDump(buf, 'Test_cursorcolumn_insert_on_tab_2', {})

  call term_sendkeys(buf, "\<C-O>")
  call TermWait(buf)
  call VerifyScreenDump(buf, 'Test_cursorcolumn_insert_on_tab_3', {})

  call term_sendkeys(buf, 'i')
  call TermWait(buf)
  call VerifyScreenDump(buf, 'Test_cursorcolumn_insert_on_tab_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_cursorcolumn_callback()
  CheckScreendump
  CheckFeature timers

  let lines =<< trim END
      call setline(1, ['aaaaa', 'bbbbb', 'ccccc', 'ddddd'])
      set cursorcolumn
      call cursor(4, 5)

      func Func(timer)
        call cursor(1, 1)
      endfunc

      call timer_start(300, 'Func')
  END
  call writefile(lines, 'Xcuc_timer', 'D')

  let buf = RunVimInTerminal('-S Xcuc_timer', #{rows: 8})
  call TermWait(buf, 310)
  call VerifyScreenDump(buf, 'Test_cursorcolumn_callback_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_wincolor()
  CheckScreendump
  " make sure the width is enough for the test
  set columns=80

  let lines =<< trim END
	set cursorline cursorcolumn rnu
	call setline(1, ["","1111111111","22222222222","3 here 3","","the cat is out of the bag"])
	set wincolor=Pmenu
	hi CatLine guifg=green ctermfg=green
	hi Reverse gui=reverse cterm=reverse
	syn match CatLine /^the.*/
	call prop_type_add("foo", {"highlight": "Reverse", "combine": 1})
	call prop_add(6, 12, {"type": "foo", "end_col": 15})
	/here
  END
  call writefile(lines, 'Xtest_wincolor', 'D')
  let buf = RunVimInTerminal('-S Xtest_wincolor', {'rows': 8})
  call TermWait(buf)
  call term_sendkeys(buf, "2G5lvj")
  call TermWait(buf)

  call VerifyScreenDump(buf, 'Test_wincolor_01', {})

  " clean up
  call term_sendkeys(buf, "\<Esc>")
  call StopVimInTerminal(buf)
endfunc

func Test_wincolor_listchars()
  CheckScreendump
  CheckFeature conceal

  let lines =<< trim END
	call setline(1, ["one","\t\tsome random text enough long to show 'extends' and 'precedes' includingnbsps, preceding tabs and trailing spaces    ","three"])
	set wincolor=Todo
	set nowrap cole=1 cocu+=n
	set list lcs=eol:$,tab:>-,space:.,trail:_,extends:>,precedes:<,conceal:*,nbsp:#
	call matchadd('Conceal', 'text')
	normal 2G5zl
  END
  call writefile(lines, 'Xtest_wincolorlcs', 'D')
  let buf = RunVimInTerminal('-S Xtest_wincolorlcs', {'rows': 8})

  call VerifyScreenDump(buf, 'Test_wincolor_lcs', {})

  " clean up
  call term_sendkeys(buf, "\<Esc>")
  call StopVimInTerminal(buf)
endfunc

func Test_colorcolumn()
  CheckScreendump

  " check that setting 'colorcolumn' when entering a buffer works
  let lines =<< trim END
	split
	edit X
	call setline(1, ["1111111111","22222222222","3333333333"])
	set nomodified
	set colorcolumn=3,9
	set number cursorline cursorlineopt=number
	wincmd w
	buf X
  END
  call writefile(lines, 'Xtest_colorcolumn', 'D')
  let buf = RunVimInTerminal('-S Xtest_colorcolumn', {'rows': 10})
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_colorcolumn_1', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_colorcolumn_bri()
  CheckScreendump

  " check 'colorcolumn' when 'breakindent' is set
  let lines =<< trim END
	call setline(1, 'The quick brown fox jumped over the lazy dogs')
  END
  call writefile(lines, 'Xtest_colorcolumn_bri', 'D')
  let buf = RunVimInTerminal('-S Xtest_colorcolumn_bri', {'rows': 10,'columns': 40})
  call term_sendkeys(buf, ":set co=40 linebreak bri briopt=shift:2 cc=40,41,43\<CR>")
  call VerifyScreenDump(buf, 'Test_colorcolumn_2', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_colorcolumn_sbr()
  CheckScreendump

  " check 'colorcolumn' when 'showbreak' is set
  let lines =<< trim END
	call setline(1, 'The quick brown fox jumped over the lazy dogs')
  END
  call writefile(lines, 'Xtest_colorcolumn_sbr', 'D')
  let buf = RunVimInTerminal('-S Xtest_colorcolumn_sbr', {'rows': 10,'columns': 40})
  call term_sendkeys(buf, ":set co=40 showbreak=+++>\\  cc=40,41,43\<CR>")
  call VerifyScreenDump(buf, 'Test_colorcolumn_3', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_visual_sbr()
  CheckScreendump

  " check Visual highlight when 'showbreak' is set
  let lines =<< trim END
      set showbreak=>
      call setline(1, 'Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.')
      exe "normal! z1\<CR>"
  END
  call writefile(lines, 'Xtest_visual_sbr', 'D')
  let buf = RunVimInTerminal('-S Xtest_visual_sbr', {'rows': 6,'columns': 60})

  call term_sendkeys(buf, "v$")
  call VerifyScreenDump(buf, 'Test_visual_sbr_1', {})

  " clean up
  call term_sendkeys(buf, "\<Esc>")
  call StopVimInTerminal(buf)
endfunc

" This test must come before the Test_cursorline test, as it appears this
" defines the Normal highlighting group anyway.
func Test_1_highlight_Normalgroup_exists()
  let hlNormal = HighlightArgs('Normal')
  if !has('gui_running')
    call assert_match('hi Normal\s*clear', hlNormal)
  elseif has('gui_gtk2') || has('gui_gnome') || has('gui_gtk3')
    " expect is DEFAULT_FONT of gui_gtk_x11.c
    call assert_match('hi Normal\s*font=Monospace 10', hlNormal)
  elseif has('gui_motif')
    " expect is DEFAULT_FONT of gui_x11.c
    call assert_match('hi Normal\s*font=7x13', hlNormal)
  elseif has('win32')
    " expect any font
    call assert_match('hi Normal\s*font=.*', hlNormal)
  endif
endfunc

" Do this test last, sometimes restoring the columns doesn't work
func Test_z_no_space_before_xxx()
  let l:org_columns = &columns
  set columns=17
  let l:hi_StatusLineTermNC = join(split(execute('hi StatusLineTermNC')))
  call assert_match('StatusLineTermNC xxx', l:hi_StatusLineTermNC)
  let &columns = l:org_columns
endfunc

" Test for :highlight command errors
func Test_highlight_cmd_errors()
  if has('gui_running')
    " This test doesn't fail in the MS-Windows console version.
    call assert_fails('hi Xcomment ctermbg=fg', 'E419:')
    call assert_fails('hi Xcomment ctermfg=bg', 'E420:')
    call assert_fails('hi Xcomment ctermfg=ul', 'E453:')
    call assert_fails('hi ' .. repeat('a', 201) .. ' ctermfg=black', 'E1249:')
  endif

  " Try using a very long terminal code. Define a dummy terminal code for this
  " test.
  let &t_fo = "\<Esc>1;"
  let c = repeat("t_fo,", 100) . "t_fo"
  call assert_fails('exe "hi Xgroup1 start=" . c', 'E422:')
  let &t_fo = ""
endfunc

" Test for 'highlight' option
func Test_highlight_opt()
  let save_hl = &highlight
  call assert_fails('set highlight=j:b', 'E474:')
  set highlight=f\ r
  call assert_equal('f r', &highlight)
  set highlight=fb
  call assert_equal('fb', &highlight)
  set highlight=fi
  call assert_equal('fi', &highlight)
  set highlight=f-
  call assert_equal('f-', &highlight)
  set highlight=fr
  call assert_equal('fr', &highlight)
  set highlight=fs
  call assert_equal('fs', &highlight)
  set highlight=fu
  call assert_equal('fu', &highlight)
  set highlight=fc
  call assert_equal('fc', &highlight)
  set highlight=ft
  call assert_equal('ft', &highlight)
  call assert_fails('set highlight=fr:Search', 'E474:')
  set highlight=f:$#
  call assert_match('W18:', v:statusmsg)
  let &highlight = save_hl
endfunc

" Test for User group highlighting used in the statusline
func Test_highlight_User()
  CheckNotGui
  hi User1 ctermfg=12
  redraw!
  call assert_equal('12', synIDattr(synIDtrans(hlID('User1')), 'fg'))
  hi clear
endfunc

" Test for using RGB color values in a highlight group
func Test_xxlast_highlight_RGB_color()
  CheckCanRunGui
  gui -f
  hi MySearch guifg=#110000 guibg=#001100 guisp=#000011
  call assert_equal('#110000', synIDattr(synIDtrans(hlID('MySearch')), 'fg#'))
  call assert_equal('#001100', synIDattr(synIDtrans(hlID('MySearch')), 'bg#'))
  call assert_equal('#000011', synIDattr(synIDtrans(hlID('MySearch')), 'sp#'))
  hi clear
endfunc

" Test for using default highlighting group
func Test_highlight_default()
  highlight MySearch ctermfg=7
  highlight default MySearch ctermfg=5
  let hlSearch = HighlightArgs('MySearch')
  call assert_match('ctermfg=7', hlSearch)

  highlight default QFName ctermfg=3
  call assert_match('ctermfg=3', HighlightArgs('QFName'))
  hi clear
endfunc

" Test for 'ctermul' in a highlight group
func Test_highlight_ctermul()
  CheckNotGui
  call assert_notmatch('ctermul=', HighlightArgs('Normal'))
  highlight Normal ctermul=3
  call assert_match('ctermul=3', HighlightArgs('Normal'))
  call assert_equal('3', synIDattr(synIDtrans(hlID('Normal')), 'ul'))
  highlight Normal ctermul=NONE
endfunc

" Test for 'ctermfont' in a highlight group
func Test_highlight_ctermfont()
  CheckNotGui
  call assert_notmatch('ctermfont=', HighlightArgs('Normal'))
  highlight Normal ctermfont=3
  call assert_match('ctermfont=3', HighlightArgs('Normal'))
  call assert_equal('3', synIDattr(synIDtrans(hlID('Normal')), 'font'))
  highlight Normal ctermfont=NONE
endfunc

" Test for specifying 'start' and 'stop' in a highlight group
func Test_highlight_start_stop()
  hi HlGrp1 start=<Esc>[27h;<Esc>[<Space>r;
  call assert_match("start=^[[27h;^[[ r;", HighlightArgs('HlGrp1'))
  hi HlGrp1 start=NONE
  call assert_notmatch("start=", HighlightArgs('HlGrp1'))
  hi HlGrp2 stop=<Esc>[27h;<Esc>[<Space>r;
  call assert_match("stop=^[[27h;^[[ r;", HighlightArgs('HlGrp2'))
  hi HlGrp2 stop=NONE
  call assert_notmatch("stop=", HighlightArgs('HlGrp2'))
  set t_xy=^[foo;
  set t_xz=^[bar;
  hi HlGrp3 start=t_xy stop=t_xz
  let d = hlget('HlGrp3')
  call assert_equal('^[foo;', d[0].start)
  call assert_equal('^[bar;', d[0].stop)
  set t_xy= t_xz=
  hi clear
endfunc

" Test for setting various 'term' attributes
func Test_highlight_term_attr()
  hi HlGrp3 term=bold,underline,undercurl,underdouble,underdotted,underdashed,strikethrough,reverse,italic,standout
  call assert_equal('hi HlGrp3          term=bold,standout,underline,undercurl,underdouble,underdotted,underdashed,italic,reverse,strikethrough', HighlightArgs('HlGrp3'))
  hi HlGrp3 term=NONE
  call assert_equal('hi HlGrp3          cleared', HighlightArgs('HlGrp3'))
  hi clear
endfunc

func Test_highlight_clear_restores_links()
  let aaa_id = hlID('aaa')
  call assert_equal(aaa_id, 0)

  " create default link aaa --> bbb
  hi def link aaa bbb
  let id_aaa = hlID('aaa')
  let hl_aaa_bbb = HighlightArgs('aaa')

  " try to redefine default link aaa --> ccc; check aaa --> bbb
  hi def link aaa ccc
  call assert_equal(HighlightArgs('aaa'), hl_aaa_bbb)

  " clear aaa; check aaa --> bbb
  hi clear aaa
  call assert_equal(HighlightArgs('aaa'), hl_aaa_bbb)

  " link aaa --> ccc; clear aaa; check aaa --> bbb
  hi link aaa ccc
  let id_ccc = hlID('ccc')
  call assert_equal(synIDtrans(id_aaa), id_ccc)
  hi clear aaa
  call assert_equal(HighlightArgs('aaa'), hl_aaa_bbb)

  " forcibly set default link aaa --> ddd
  hi! def link aaa ddd
  let id_ddd = hlID('ddd')
  let hl_aaa_ddd = HighlightArgs('aaa')
  call assert_equal(synIDtrans(id_aaa), id_ddd)

  " link aaa --> eee; clear aaa; check aaa --> ddd
  hi link aaa eee
  let eee_id = hlID('eee')
  call assert_equal(synIDtrans(id_aaa), eee_id)
  hi clear aaa
  call assert_equal(HighlightArgs('aaa'), hl_aaa_ddd)
endfunc

func Test_highlight_clear_restores_context()
  func FuncContextDefault()
    hi def link Context ContextDefault
  endfun

  func FuncContextRelink()
    " Dummy line
    hi link Context ContextRelink
  endfunc

  let scriptContextDefault = MakeScript("FuncContextDefault")
  let scriptContextRelink = MakeScript("FuncContextRelink")
  let patContextDefault = fnamemodify(scriptContextDefault, ':t') .. ' line 1'
  let patContextRelink = fnamemodify(scriptContextRelink, ':t') .. ' line 2'

  exec 'source ' .. scriptContextDefault
  let hlContextDefault = execute("verbose hi Context")
  call assert_match(patContextDefault, hlContextDefault)

  exec 'source ' .. scriptContextRelink
  let hlContextRelink = execute("verbose hi Context")
  call assert_match(patContextRelink, hlContextRelink)

  hi clear
  let hlContextAfterClear = execute("verbose hi Context")
  call assert_match(patContextDefault, hlContextAfterClear)

  delfunc FuncContextDefault
  delfunc FuncContextRelink
  call delete(scriptContextDefault)
  call delete(scriptContextRelink)
endfunc

func Test_highlight_default_colorscheme_restores_links()
  hi link TestLink Identifier
  hi TestHi ctermbg=red

  let hlTestLinkPre = HighlightArgs('TestLink')
  let hlTestHiPre = HighlightArgs('TestHi')

  " Test colorscheme
  call assert_equal("\ndefault", execute('colorscheme'))
  hi clear
  if exists('syntax_on')
    syntax reset
  endif
  let g:colors_name = 'test'
  call assert_equal("\ntest", execute('colorscheme'))
  hi link TestLink ErrorMsg
  hi TestHi ctermbg=green

  " Restore default highlighting
  colorscheme default
  " 'default' should work no matter if highlight group was cleared
  call assert_equal("\ndefault", execute('colorscheme'))
  hi def link TestLink Identifier
  hi def TestHi ctermbg=red
  let hlTestLinkPost = HighlightArgs('TestLink')
  let hlTestHiPost = HighlightArgs('TestHi')
  call assert_equal(hlTestLinkPre, hlTestLinkPost)
  call assert_equal(hlTestHiPre, hlTestHiPost)
  hi clear
endfunc

func Test_colornames_assignment_and_lookup()
  CheckAnyOf Feature:gui_running Feature:termguicolors

  " Ensure highlight command can find custom color.
  let v:colornames['a redish white'] = '#ffeedd'
  highlight Normal guifg='a redish white'
  highlight clear
  call ClearDict(v:colornames)
endfunc

func Test_colornames_default_list()
  CheckAnyOf Feature:gui_running Feature:termguicolors

  " Ensure default lists are loaded automatically and can be used for all gui fields.
  call assert_equal(0, len(v:colornames))
  highlight Normal guifg='rebecca purple' guibg='rebecca purple' guisp='rebecca purple'
  call assert_notequal(0, len(v:colornames))
  echo v:colornames['rebecca purple']
  highlight clear
  call ClearDict(v:colornames)
endfunc

func Test_colornames_overwrite_default()
  CheckAnyOf Feature:gui_running Feature:termguicolors

  " Ensure entries in v:colornames can be overwritten.
  " Load default color scheme to trigger default color list loading.
  colorscheme default
  let old_rebecca_purple = v:colornames['rebecca purple']
  highlight Normal guifg='rebecca purple' guibg='rebecca purple'
  let v:colornames['rebecca purple'] = '#550099'
  highlight Normal guifg='rebecca purple' guibg='rebecca purple'
  let v:colornames['rebecca purple'] = old_rebecca_purple
  highlight clear
endfunc

func Test_colornames_assignment_and_unassignment()
  " No feature check is needed for this test because the v:colornames dict
  " always exists with +eval. The feature checks are only required for
  " commands that do color lookup.

  " Ensure we cannot overwrite the v:colornames dict.
  call assert_fails("let v:colornames = {}", 'E46:')

  " Ensure we can delete entries from the v:colornames dict.
  let v:colornames['x1'] = '#111111'
  call assert_equal(v:colornames['x1'], '#111111')
  unlet v:colornames['x1']
  call assert_fails("echo v:colornames['x1']")
endfunc

" Test for the hlget() function
func Test_hlget()
  let lines =<< trim END
    call assert_notequal([], filter(hlget(), 'v:val.name == "Visual"'))
    call assert_equal([], hlget('SomeHLGroup'))
    highlight MyHLGroup term=standout cterm=reverse ctermfg=10 ctermbg=Black
    call assert_equal([{'id': hlID('MyHLGroup'), 'ctermfg': '10', 'name': 'MyHLGroup', 'term': {'standout': v:true}, 'ctermbg': '0', 'cterm': {'reverse': v:true}}], hlget('MyHLGroup'))
    highlight clear MyHLGroup
    call assert_equal(v:true, hlget('MyHLGroup')[0].cleared)
    highlight link MyHLGroup IncSearch
    call assert_equal('IncSearch', hlget('MyHLGroup')[0].linksto)
    highlight clear MyHLGroup
    call assert_equal([], hlget(test_null_string()))
    call assert_equal([], hlget(""))
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Test for resolving highlight group links
  let lines =<< trim END
    highlight hlgA term=bold
    VAR hlgAid = hlID('hlgA')
    highlight link hlgB hlgA
    VAR hlgBid = hlID('hlgB')
    highlight link hlgC hlgB
    VAR hlgCid = hlID('hlgC')
    call assert_equal('hlgA', hlget('hlgB')[0].linksto)
    call assert_equal('hlgB', hlget('hlgC')[0].linksto)
    call assert_equal([{'id': hlgAid, 'name': 'hlgA',
                      \ 'term': {'bold': v:true}}], hlget('hlgA'))
    call assert_equal([{'id': hlgBid, 'name': 'hlgB',
                      \ 'linksto': 'hlgA'}], hlget('hlgB'))
    call assert_equal([{'id': hlgCid, 'name': 'hlgC',
                      \ 'linksto': 'hlgB'}], hlget('hlgC'))
    call assert_equal([{'id': hlgAid, 'name': 'hlgA',
                      \ 'term': {'bold': v:true}}], hlget('hlgA', v:false))
    call assert_equal([{'id': hlgBid, 'name': 'hlgB',
                      \ 'linksto': 'hlgA'}], hlget('hlgB', 0))
    call assert_equal([{'id': hlgCid, 'name': 'hlgC',
                      \ 'linksto': 'hlgB'}], hlget('hlgC', v:false))
    call assert_equal([{'id': hlgAid, 'name': 'hlgA',
                      \ 'term': {'bold': v:true}}], hlget('hlgA', v:true))
    call assert_equal([{'id': hlgBid, 'name': 'hlgB',
                      \ 'term': {'bold': v:true}}], hlget('hlgB', 1))
    call assert_equal([{'id': hlgCid, 'name': 'hlgC',
                      \ 'term': {'bold': v:true}}], hlget('hlgC', v:true))
  END
  call v9.CheckLegacyAndVim9Success(lines)

  call assert_fails('call hlget([])', 'E1174:')
  call assert_fails('call hlget("abc", "xyz")', 'E1212:')
endfunc

" Test for the hlset() function
func Test_hlset()
  let lines =<< trim END
    call assert_equal(0, hlset(test_null_list()))
    call assert_equal(0, hlset([]))
    call assert_fails('call hlset(["Search"])', 'E715:')
    call hlset(hlget())
    call hlset([{'name': 'NewHLGroup', 'cterm': {'reverse': v:true}, 'ctermfg': '10'}])
    call assert_equal({'reverse': v:true}, hlget('NewHLGroup')[0].cterm)
    call hlset([{'name': 'NewHLGroup', 'cterm': {'bold': v:true}}])
    call assert_equal({'bold': v:true}, hlget('NewHLGroup')[0].cterm)
    call hlset([{'name': 'NewHLGroup', 'cleared': v:true}])
    call assert_equal(v:true, hlget('NewHLGroup')[0].cleared)
    call hlset([{'name': 'NewHLGroup', 'linksto': 'Search'}])
    call assert_false(has_key(hlget('NewHLGroup')[0], 'cleared'))
    call assert_equal('Search', hlget('NewHLGroup')[0].linksto)
    call assert_fails("call hlset([{'name': [], 'ctermfg': '10'}])", 'E928:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'cleared': []}])",
          \ 'E745:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'cterm': 'Blue'}])",
          \ 'E715:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'ctermbg': []}])",
          \ 'E928:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'ctermfg': []}])",
          \ 'E928:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'ctermul': []}])",
          \ 'E928:')
    if has('gui')
      call assert_fails("call hlset([{'name': 'NewHLGroup', 'font': []}])",
            \ 'E928:')
    endif
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'gui': 'Cyan'}])",
          \ 'E715:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'guibg': []}])",
          \ 'E928:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'guifg': []}])",
          \ 'E928:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'guisp': []}])",
          \ 'E928:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'linksto': []}])",
          \ 'E928:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'start': []}])",
          \ 'E928:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'stop': []}])",
          \ 'E928:')
    call assert_fails("call hlset([{'name': 'NewHLGroup', 'term': 'Cyan'}])",
          \ 'E715:')
    call assert_equal('Search', hlget('NewHLGroup')[0].linksto)
    highlight clear NewHLGroup
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Test for clearing the 'term', 'cterm' and 'gui' attributes of a highlight
  " group.
  let lines =<< trim END
    highlight myhlg1 term=bold cterm=italic gui=standout
    VAR id = hlID('myhlg1')
    call hlset([{'name': 'myhlg1', 'term': {}}])
    call assert_equal([{'id': id, 'name': 'myhlg1',
                \ 'cterm': {'italic': v:true}, 'gui': {'standout': v:true}}],
                \ hlget('myhlg1'))
    call hlset([{'name': 'myhlg1', 'cterm': {}}])
    call assert_equal([{'id': id, 'name': 'myhlg1',
                \ 'gui': {'standout': v:true}}], hlget('myhlg1'))
    call hlset([{'name': 'myhlg1', 'gui': {}}])
    call assert_equal([{'id': id, 'name': 'myhlg1', 'cleared': v:true}],
                \ hlget('myhlg1'))
    highlight clear myhlg1
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Test for setting all the 'term', 'cterm' and 'gui' attributes of a
  " highlight group
  let lines =<< trim END
    VAR attr = {'bold': v:true, 'underline': v:true,
                \ 'undercurl': v:true, 'underdouble': v:true,
                \ 'underdotted': v:true, 'underdashed': v:true,
                \ 'strikethrough': v:true, 'reverse': v:true, 'italic': v:true,
                \ 'standout': v:true, 'nocombine': v:true}
    call hlset([{'name': 'myhlg2', 'term': attr, 'cterm': attr, 'gui': attr}])
    VAR id2 = hlID('myhlg2')
    VAR expected = "myhlg2 xxx term=bold,standout,underline,undercurl,underdouble,underdotted,underdashed,italic,reverse,nocombine,strikethrough cterm=bold,standout,underline,undercurl,underdouble,underdotted,underdashed,italic,reverse,nocombine,strikethrough gui=bold,standout,underline,undercurl,underdouble,underdotted,underdashed,italic,reverse,nocombine,strikethrough"
    VAR output = execute('highlight myhlg2')
    LET output = output->split("\n")->join()->substitute('\s\+', ' ', 'g')
    call assert_equal(expected, output)
    call assert_equal([{'id': id2, 'name': 'myhlg2', 'gui': attr,
                      \ 'term': attr, 'cterm': attr}], hlget('myhlg2'))
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Test for clearing some of the 'term', 'cterm' and 'gui' attributes of a
  " highlight group
  let lines =<< trim END
    VAR attr = {'bold': v:false, 'underline': v:true, 'strikethrough': v:true}
    call hlset([{'name': 'myhlg2', 'term': attr, 'cterm': attr, 'gui': attr}])
    VAR id2 = hlID('myhlg2')
    VAR expected = "myhlg2 xxx term=underline,strikethrough cterm=underline,strikethrough gui=underline,strikethrough"
    VAR output = execute('highlight myhlg2')
    LET output = output->split("\n")->join()->substitute('\s\+', ' ', 'g')
    call assert_equal(expected, output)
    LET attr = {'underline': v:true, 'strikethrough': v:true}
    call assert_equal([{'id': id2, 'name': 'myhlg2', 'gui': attr,
                      \ 'term': attr, 'cterm': attr}], hlget('myhlg2'))
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Test for clearing the attributes and link of a highlight group
  let lines =<< trim END
    highlight myhlg3 ctermbg=green guibg=green
    highlight! default link myhlg3 ErrorMsg
    VAR id3 = hlID('myhlg3')
    call hlset([{'name': 'myhlg3', 'cleared': v:true, 'linksto': 'NONE'}])
    call assert_equal([{'id': id3, 'name': 'myhlg3', 'cleared': v:true}],
                      \ hlget('myhlg3'))
    highlight clear hlg3
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Test for setting default attributes for a highlight group
  let lines =<< trim END
    call hlset([{'name': 'hlg4', 'ctermfg': '8'}])
    call hlset([{'name': 'hlg4', 'default': v:true, 'ctermfg': '9'}])
    VAR id4 = hlID('hlg4')
    call assert_equal([{'id': id4, 'name': 'hlg4', 'ctermfg': '8'}],
                    \ hlget('hlg4'))
    highlight clear hlg4

    call hlset([{'name': 'hlg5', 'default': v:true, 'ctermbg': '2'}])
    call hlset([{'name': 'hlg5', 'ctermbg': '4'}])
    VAR id5 = hlID('hlg5')
    call assert_equal([{'id': id5, 'name': 'hlg5', 'ctermbg': '4'}],
                    \ hlget('hlg5'))
    highlight clear hlg5

    call hlset([{'name': 'hlg6', 'linksto': 'Error'}])
    VAR id6 = hlID('hlg6')
    call hlset([{'name': 'hlg6', 'default': v:true, 'ctermbg': '2'}])
    call assert_equal([{'id': id6, 'name': 'hlg6', 'linksto': 'Error'}],
                    \ hlget('hlg6'))
    highlight clear hlg6
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Test for setting default links for a highlight group
  let lines =<< trim END
    call hlset([{'name': 'hlg7', 'ctermfg': '5'}])
    call hlset([{'name': 'hlg7', 'default': v:true, 'linksto': 'Search'}])
    VAR id7 = hlID('hlg7')
    call assert_equal([{'id': id7, 'name': 'hlg7', 'ctermfg': '5'}],
                    \ hlget('hlg7'))
    highlight clear hlg7

    call hlset([{'name': 'hlg8', 'default': v:true, 'linksto': 'Search'}])
    VAR id8 = hlID('hlg8')
    call assert_equal([{'id': id8, 'name': 'hlg8', 'default': v:true,
                    \ 'linksto': 'Search'}], hlget('hlg8'))
    call hlset([{'name': 'hlg8', 'ctermbg': '2'}])
    call assert_equal([{'id': id8, 'name': 'hlg8', 'ctermbg': '2'}],
                    \ hlget('hlg8'))
    highlight clear hlg8

    highlight default link hlg9 ErrorMsg
    VAR hlg_save = hlget('hlg9')
    LET hlg_save[0]['name'] = 'hlg9dup'
    call hlset(hlg_save)
    VAR id9 = hlID('hlg9dup')
    highlight clear hlg9dup
    call assert_equal([{'id': id9, 'name': 'hlg9dup', 'default': v:true,
                    \ 'linksto': 'ErrorMsg'}], hlget('hlg9dup'))
    highlight clear hlg9
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Test for force creating a link to a highlight group
  let lines =<< trim END
    call hlset([{'name': 'hlg10', 'ctermfg': '8'}])
    call hlset([{'name': 'hlg10', 'linksto': 'Search'}])
    VAR id10 = hlID('hlg10')
    call assert_equal([{'id': id10, 'name': 'hlg10', 'ctermfg': '8'}],
                    \ hlget('hlg10'))
    call hlset([{'name': 'hlg10', 'linksto': 'Search', 'force': v:true}])
    call assert_equal([{'id': id10, 'name': 'hlg10', 'ctermfg': '8',
                    \ 'linksto': 'Search'}], hlget('hlg10'))
    highlight clear hlg10
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Test for empty values of attributes
  call hlset([{'name': 'hlg11', 'cterm': {}}])
  call hlset([{'name': 'hlg11', 'ctermfg': ''}])
  call hlset([{'name': 'hlg11', 'ctermbg': ''}])
  call hlset([{'name': 'hlg11', 'ctermul': ''}])
  call hlset([{'name': 'hlg11', 'ctermfont': ''}])
  call hlset([{'name': 'hlg11', 'font': ''}])
  call hlset([{'name': 'hlg11', 'gui': {}}])
  call hlset([{'name': 'hlg11', 'guifg': ''}])
  call hlset([{'name': 'hlg11', 'guibg': ''}])
  call hlset([{'name': 'hlg11', 'guisp': ''}])
  call hlset([{'name': 'hlg11', 'start': ''}])
  call hlset([{'name': 'hlg11', 'stop': ''}])
  call hlset([{'name': 'hlg11', 'term': {}}])
  call assert_true(hlget('hlg11')[0].cleared)
endfunc

" vim: shiftwidth=2 sts=2 expandtab
