" Tests for bracketed paste and other forms of pasting.

" Bracketed paste only works with "xterm".  Not in GUI or Windows console.
source check.vim
source term_util.vim
CheckNotMSWindows
CheckNotGui

set term=xterm

source shared.vim

func Test_paste_normal_mode()
  new
  " In first column text is inserted
  call setline(1, ['a', 'b', 'c'])
  call cursor(2, 1)
  call feedkeys("\<Esc>[200~foo\<CR>bar\<Esc>[201~", 'xt')
  call assert_equal('foo', getline(2))
  call assert_equal('barb', getline(3))
  call assert_equal('c', getline(4))

  " When repeating text is appended
  normal .
  call assert_equal('barfoo', getline(3))
  call assert_equal('barb', getline(4))
  call assert_equal('c', getline(5))
  bwipe!

  " In second column text is appended
  call setline(1, ['a', 'bbb', 'c'])
  call cursor(2, 2)
  call feedkeys("\<Esc>[200~foo\<CR>bar\<Esc>[201~", 'xt')
  call assert_equal('bbfoo', getline(2))
  call assert_equal('barb', getline(3))
  call assert_equal('c', getline(4))

  " In last column text is appended
  call setline(1, ['a', 'bbb', 'c'])
  call cursor(2, 3)
  call feedkeys("\<Esc>[200~foo\<CR>bar\<Esc>[201~", 'xt')
  call assert_equal('bbbfoo', getline(2))
  call assert_equal('bar', getline(3))
  call assert_equal('c', getline(4))
endfunc

func Test_paste_insert_mode()
  new
  call setline(1, ['a', 'b', 'c'])
  2
  call feedkeys("i\<Esc>[200~foo\<CR>bar\<Esc>[201~ done\<Esc>", 'xt')
  call assert_equal('foo', getline(2))
  call assert_equal('bar doneb', getline(3))
  call assert_equal('c', getline(4))

  normal .
  call assert_equal('bar donfoo', getline(3))
  call assert_equal('bar doneeb', getline(4))
  call assert_equal('c', getline(5))

  set ai et tw=10
  call setline(1, ['a', '    b', 'c'])
  2
  call feedkeys("A\<Esc>[200~foo\<CR> bar bar bar\<Esc>[201~\<Esc>", 'xt')
  call assert_equal('    bfoo', getline(2))
  call assert_equal(' bar bar bar', getline(3))
  call assert_equal('c', getline(4))

  set ai& et& tw=0
  bwipe!
endfunc

func Test_paste_clipboard()
  CheckFeature clipboard_working

  let @+ = "nasty\<Esc>:!ls\<CR>command"
  new
  exe "normal i\<C-R>+\<Esc>"
  call assert_equal("nasty\<Esc>:!ls\<CR>command", getline(1))
  bwipe!
endfunc

" bracketed paste in command line
func Test_paste_cmdline()
  call feedkeys(":a\<Esc>[200~foo\<CR>bar\<Esc>[201~b\<Home>\"\<CR>", 'xt')
  call assert_equal("\"afoo\<CR>barb", getreg(':'))
endfunc

" bracketed paste in Ex-mode
func Test_paste_ex_mode()
  unlet! foo
  call feedkeys("Qlet foo=\"\<Esc>[200~foo\<CR>bar\<Esc>[201~\"\<CR>vi\<CR>", 'xt')
  call assert_equal("foo\rbar", foo)

  " pasting more than 40 bytes
  exe "norm Q\<PasteStart>0000000000000000000000000000000000000000000000000000000000000000000000\<C-C>"
endfunc

func Test_paste_onechar()
  new
  let @f='abc'
  call feedkeys("i\<C-R>\<Esc>[200~foo\<CR>bar\<Esc>[201~", 'xt')
  call assert_equal("abc", getline(1))
  close!
endfunc

func Test_paste_visual_mode()
  new
  call setline(1, 'here are some words')
  call feedkeys("0fsve\<Esc>[200~more\<Esc>[201~", 'xt')
  call assert_equal('here are more words', getline(1))
  call assert_equal('some', getreg('-'))
  normal! u
  call assert_equal('here are some words', getline(1))
  exe "normal! \<C-R>"
  call assert_equal('here are more words', getline(1))

  " include last char in the line
  call feedkeys("0fwve\<Esc>[200~noises\<Esc>[201~", 'xt')
  call assert_equal('here are more noises', getline(1))
  call assert_equal('words', getreg('-'))
  normal! u
  call assert_equal('here are more words', getline(1))
  exe "normal! \<C-R>"
  call assert_equal('here are more noises', getline(1))

  " exclude last char in the line
  call setline(1, 'some words!')
  call feedkeys("0fwve\<Esc>[200~noises\<Esc>[201~", 'xt')
  call assert_equal('some noises!', getline(1))
  call assert_equal('words', getreg('-'))
  normal! u
  call assert_equal('some words!', getline(1))
  exe "normal! \<C-R>"
  call assert_equal('some noises!', getline(1))

  " multi-line selection
  call setline(1, ['some words', 'and more'])
  call feedkeys("0fwvj0fd\<Esc>[200~letters\<Esc>[201~", 'xt')
  call assert_equal('some letters more', getline(1))
  call assert_equal("words\nand", getreg('1'))
  normal! u
  call assert_equal(['some words', 'and more'], getline(1, 2))
  exe "normal! \<C-R>"
  call assert_equal('some letters more', getline(1))

  " linewise non-last line, cursor at start of line
  call setline(1, ['some words', 'and more'])
  call feedkeys("0V\<Esc>[200~letters\<Esc>[201~", 'xt')
  call assert_equal('lettersand more', getline(1))
  call assert_equal("some words\n", getreg('1'))
  normal! u
  call assert_equal(['some words', 'and more'], getline(1, 2))
  exe "normal! \<C-R>"
  call assert_equal('lettersand more', getline(1))

  " linewise non-last line, cursor in the middle of line
  call setline(1, ['some words', 'and more'])
  call feedkeys("0fwV\<Esc>[200~letters\<Esc>[201~", 'xt')
  call assert_equal('lettersand more', getline(1))
  call assert_equal("some words\n", getreg('1'))
  normal! u
  call assert_equal(['some words', 'and more'], getline(1, 2))
  exe "normal! \<C-R>"
  call assert_equal('lettersand more', getline(1))

  " linewise last line
  call setline(1, ['some words', 'and more'])
  call feedkeys("j0V\<Esc>[200~letters\<Esc>[201~", 'xt')
  call assert_equal(['some words', 'letters'], getline(1, 2))
  call assert_equal("and more\n", getreg('1'))
  normal! u
  call assert_equal(['some words', 'and more'], getline(1, 2))
  exe "normal! \<C-R>"
  call assert_equal(['some words', 'letters'], getline(1, 2))

  bwipe!
endfunc

func CheckCopyPaste()
  call setline(1, ['copy this', ''])
  normal 1G0"*y$
  normal j"*p
  call assert_equal('copy this', getline(2))
endfunc

func Test_xrestore()
  CheckFeature xterm_clipboard
  let g:test_is_flaky = 1

  let display = $DISPLAY
  new
  call CheckCopyPaste()

  xrestore
  call CheckCopyPaste()

  exe "xrestore " .. display
  call CheckCopyPaste()

  bwipe!
endfunc

" Test for 'pastetoggle'
func Test_pastetoggle()
  new
  set pastetoggle=<F4>
  set nopaste
  call feedkeys("iHello\<F4>", 'xt')
  call assert_true(&paste)
  call feedkeys("i\<F4>", 'xt')
  call assert_false(&paste)
  call assert_equal('Hello', getline(1))
  " command-line completion for 'pastetoggle' value
  call feedkeys(":set pastetoggle=\<Tab>\<C-B>\"\<CR>", 'xt')
  call assert_equal('"set pastetoggle=<F4>', @:)
  set pastetoggle&
  bwipe!
endfunc

func Test_pastetoggle_timeout_no_typed_after_mapped()
  CheckRunVimInTerminal

  let lines =<< trim END
    set pastetoggle=abc
    set ttimeoutlen=10000
    imap d a
  END
  call writefile(lines, 'Xpastetoggle_no_typed_after_mapped.vim', 'D')
  let buf = RunVimInTerminal('-S Xpastetoggle_no_typed_after_mapped.vim', #{rows: 8})
  call TermWait(buf)
  call term_sendkeys(buf, ":call feedkeys('id', 't')\<CR>")
  call term_wait(buf, 200)
  call term_sendkeys(buf, 'bc')
  " 'ttimeoutlen' should NOT apply
  call WaitForAssert({-> assert_match('^-- INSERT --', term_getline(buf, 8))})

  call StopVimInTerminal(buf)
endfunc

func Test_pastetoggle_timeout_typed_after_mapped()
  CheckRunVimInTerminal

  let lines =<< trim END
    set pastetoggle=abc
    set ttimeoutlen=10000
    imap d a
  END
  call writefile(lines, 'Xpastetoggle_typed_after_mapped.vim', 'D')
  let buf = RunVimInTerminal('-S Xpastetoggle_typed_after_mapped.vim', #{rows: 8})
  call TermWait(buf)
  call term_sendkeys(buf, ":call feedkeys('idb', 't')\<CR>")
  call term_wait(buf, 200)
  call term_sendkeys(buf, 'c')
  " 'ttimeoutlen' should apply
  call WaitForAssert({-> assert_match('^-- INSERT (paste) --', term_getline(buf, 8))})

  call StopVimInTerminal(buf)
endfunc

func Test_pastetoggle_timeout_typed_after_noremap()
  CheckRunVimInTerminal

  let lines =<< trim END
    set pastetoggle=abc
    set ttimeoutlen=10000
    inoremap d a
  END
  call writefile(lines, 'Xpastetoggle_typed_after_noremap.vim', 'D')
  let buf = RunVimInTerminal('-S Xpastetoggle_typed_after_noremap.vim', #{rows: 8})
  call TermWait(buf)
  call term_sendkeys(buf, ":call feedkeys('idb', 't')\<CR>")
  call term_wait(buf, 200)
  call term_sendkeys(buf, 'c')
  " 'ttimeoutlen' should apply
  call WaitForAssert({-> assert_match('^-- INSERT (paste) --', term_getline(buf, 8))})

  call StopVimInTerminal(buf)
endfunc

" Test for restoring option values when 'paste' is disabled
func Test_paste_opt_restore()
  set autoindent expandtab ruler showmatch
  if has('rightleft')
    set revins hkmap
  endif
  set smarttab softtabstop=3 textwidth=27 wrapmargin=12
  if has('vartabs')
    set varsofttabstop=10,20
  endif

  " enabling 'paste' should reset the above options
  set paste
  call assert_false(&autoindent)
  call assert_false(&expandtab)
  if has('rightleft')
    call assert_false(&revins)
    call assert_false(&hkmap)
  endif
  call assert_false(&ruler)
  call assert_false(&showmatch)
  call assert_false(&smarttab)
  call assert_equal(0, &softtabstop)
  call assert_equal(0, &textwidth)
  call assert_equal(0, &wrapmargin)
  if has('vartabs')
    call assert_equal('', &varsofttabstop)
  endif

  " disabling 'paste' should restore the option values
  set nopaste
  call assert_true(&autoindent)
  call assert_true(&expandtab)
  if has('rightleft')
    call assert_true(&revins)
    call assert_true(&hkmap)
  endif
  call assert_true(&ruler)
  call assert_true(&showmatch)
  call assert_true(&smarttab)
  call assert_equal(3, &softtabstop)
  call assert_equal(27, &textwidth)
  call assert_equal(12, &wrapmargin)
  if has('vartabs')
    call assert_equal('10,20', &varsofttabstop)
  endif

  set autoindent& expandtab& ruler& showmatch&
  if has('rightleft')
    set revins& hkmap&
  endif
  set smarttab& softtabstop& textwidth& wrapmargin&
  if has('vartabs')
    set varsofttabstop&
  endif
endfunc

" vim: shiftwidth=2 sts=2 expandtab
