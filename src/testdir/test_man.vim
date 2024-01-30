" Test specifically for the Man filetype plugin.

runtime ftplugin/man.vim

func Test_g_ft_man_open_mode()
  vnew
  let l:h = winheight(1)
  q
  let l:w = winwidth(1)

  " split horizontally
  let wincnt = winnr('$')
  Man vim
  if wincnt == winnr('$')
    " Vim manual page cannot be found.
    return
  endif

  call assert_inrange(l:w - 2, l:w + 2, winwidth(1))
  call assert_true(l:h > winheight(1))
  call assert_equal(1, tabpagenr('$'))
  call assert_equal(1, tabpagenr())
  q

  " split horizontally
  let g:ft_man_open_mode = "horz"
  Man vim
  call assert_inrange(l:w - 2, l:w + 2, winwidth(1))
  call assert_true(l:h > winheight(1))
  call assert_equal(1, tabpagenr('$'))
  call assert_equal(1, tabpagenr())
  q

  " split vertically
  let g:ft_man_open_mode = "vert"
  Man vim
  call assert_true(l:w > winwidth(1))
  call assert_equal(l:h, winheight(1))
  call assert_equal(1, tabpagenr('$'))
  call assert_equal(1, tabpagenr())
  q

  " separate tab
  let g:ft_man_open_mode = "tab"
  Man vim
  call assert_inrange(l:w - 2, l:w + 2, winwidth(1))
  call assert_inrange(l:h - 1, l:h + 1, winheight(1))
  call assert_equal(2, tabpagenr('$'))
  call assert_equal(2, tabpagenr())
  q

  unlet g:ft_man_open_mode
endfunc

func Test_nomodifiable()
  let wincnt = winnr('$')
  Man vim
  if wincnt == winnr('$')
    " Vim manual page cannot be found.
    return
  endif
  call assert_false(&l:modifiable)
  q
endfunc

func Test_buffer_count_hidden()
  %bw!
  set hidden

  call assert_equal(1, len(getbufinfo()))

  let wincnt = winnr('$')
  Man vim
  if wincnt == winnr('$')
    " Vim manual page cannot be found.
    return
  endif

  call assert_equal(1, len(getbufinfo({'buflisted':1})))
  call assert_equal(2, len(getbufinfo()))
  q

  Man vim

  call assert_equal(1, len(getbufinfo({'buflisted':1})))
  call assert_equal(2, len(getbufinfo()))
  q

  set hidden&
endfunc

" Check that we do not alter the settings in the initial window.
func Test_local_options()
  %bw!
  set foldcolumn=1 number

  let wincnt = winnr('$')
  Man vim
  if wincnt == winnr('$')
    " Vim manual page cannot be found.
    return
  endif

  " man page
  call assert_false(&nu)
  call assert_equal(0, &fdc)

  " initial window
  wincmd p
  call assert_true(&nu)
  call assert_equal(1, &fdc)

  %bw!
  set foldcolumn& number&
endfunc

" Check that the unnamed register is not overwritten.
func Test_keep_unnamed_register()
  %bw!

  let @" = '---'

  let wincnt = winnr('$')
  Man vim
  if wincnt == winnr('$')
    " Vim manual page cannot be found.
    return
  endif

  call assert_equal('---', @")

  %bw!
endfunc

" Check that underlying shell command arguments are escaped.
func Test_Man_uses_shellescape()
  Man `touch\ Xbar` `touch\ Xfoo`

  redir => msg
  1messages
  redir END
  call assert_match('no manual entry for "`touch Xfoo`"', msg)

  call assert_false(filereadable('Xbar'))
  call assert_false(filereadable('Xfoo'))
endfunc


" vim: shiftwidth=2 sts=2 expandtab
