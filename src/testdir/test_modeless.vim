" Test for modeless selection

" This only works for Unix in a terminal
source check.vim
CheckNotGui
CheckUnix

source shared.vim
source mouse.vim

" Test for modeless characterwise selection (single click)
func Test_modeless_characterwise_selection()
  CheckFeature clipboard_working
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm mousetime=200
  call WaitForResponses()

  new
  call setline(1, ['one two three', 'foo bar baz'])
  redraw!

  " Wait a bit for any terminal responses to get processed.
  sleep 50m

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val

    " select multiple characters within a line
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 6)
    let keys ..= MouseLeftDragCode(1, 10)
    let keys ..= MouseLeftReleaseCode(1, 10)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("wo th", @*, msg)

    " select multiple characters including the end of line
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 11)
    let keys ..= MouseLeftDragCode(1, 16)
    let keys ..= MouseLeftReleaseCode(1, 16)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("ree\n", @*, msg)

    " extend a selection using right mouse click
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    set mousemodel=extend
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 2)
    let keys ..= MouseLeftDragCode(1, 5)
    let keys ..= MouseLeftReleaseCode(1, 5)
    let keys ..= MouseRightClickCode(1, 10)
    let keys ..= MouseRightReleaseCode(1, 10)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("ne two th", @*, msg)
    set mousemodel&

    " extend a selection backwards using right mouse click
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    set mousemodel=extend
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 7)
    let keys ..= MouseLeftDragCode(1, 11)
    let keys ..= MouseLeftReleaseCode(1, 11)
    let keys ..= MouseRightClickCode(1, 3)
    let keys ..= MouseRightReleaseCode(1, 3)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("e two thr", @*, msg)
    set mousemodel&

    " select multiple characters within a line backwards
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 9)
    let keys ..= MouseLeftDragCode(1, 3)
    let keys ..= MouseLeftReleaseCode(1, 3)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("e two t", @*, msg)

    " select multiple characters across lines with (end row > start row) and
    " (end column < start column)
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 9)
    let keys ..= MouseLeftDragCode(2, 3)
    let keys ..= MouseLeftReleaseCode(2, 3)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("three\nfoo", @*, msg)

    " select multiple characters across lines with (end row > start row) and
    " (end column > start column)
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 4)
    let keys ..= MouseLeftDragCode(2, 8)
    let keys ..= MouseLeftReleaseCode(2, 8)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal(" two three\nfoo bar ", @*, msg)

    " select multiple characters across lines with (end row < start row) and
    " (end column < start column)
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(2, 7)
    let keys ..= MouseLeftDragCode(1, 5)
    let keys ..= MouseLeftReleaseCode(1, 5)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("two three\nfoo bar", @*, msg)

    " select multiple characters across lines with (end row < start row) and
    " (end column > start column)
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(2, 11)
    let keys ..= MouseLeftDragCode(1, 13)
    let keys ..= MouseLeftReleaseCode(1, 13)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("e\nfoo bar baz", @*, msg)

    " select multiple characters across lines with (end row < start row) and
    " the end column is greater than the line length
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(2, 7)
    let keys ..= MouseLeftDragCode(1, 16)
    let keys ..= MouseLeftReleaseCode(1, 16)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("\nfoo bar", @*, msg)

    " select multiple characters across lines with start/end row and start/end
    " column outside the lines in the buffer
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(4, 3)
    let keys ..= MouseLeftDragCode(3, 2)
    let keys ..= MouseLeftReleaseCode(3, 2)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("\n~  ", @*, msg)

    " change selection using right mouse click within the selected text
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    set mousemodel=extend
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 5)
    let keys ..= MouseLeftDragCode(1, 13)
    let keys ..= MouseLeftReleaseCode(1, 13)
    let keys ..= MouseRightClickCode(1, 7)
    let keys ..= MouseRightReleaseCode(1, 7)
    let keys ..= MouseRightClickCode(1, 11)
    let keys ..= MouseRightReleaseCode(1, 11)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("o thr", @*, msg)
    set mousemodel&

    " select text multiple times at different places
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 3)
    let keys ..= MouseLeftDragCode(1, 5)
    let keys ..= MouseLeftReleaseCode(1, 5)
    let keys ..= MouseLeftClickCode(2, 7)
    let keys ..= MouseLeftDragCode(2, 9)
    let keys ..= MouseLeftReleaseCode(2, 9)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("r b", @*, msg)

    " Test for 'clipboard' set to 'autoselectml' to automatically copy the
    " modeless selection to the clipboard
    set clipboard=autoselectml
    let @* = 'clean'
    let keys = ":"
    let keys ..= MouseLeftClickCode(2, 5)
    let keys ..= MouseLeftDragCode(2, 7)
    let keys ..= MouseLeftReleaseCode(2, 7)
    let keys ..= "\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("bar", @*)
    set clipboard&

    " quadruple click should start characterwise selectmode
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 10)
    let keys ..= MouseLeftReleaseCode(1, 10)
    let keys ..= MouseLeftClickCode(1, 10)
    let keys ..= MouseLeftReleaseCode(1, 10)
    let keys ..= MouseLeftClickCode(1, 10)
    let keys ..= MouseLeftReleaseCode(1, 10)
    let keys ..= MouseLeftClickCode(1, 10)
    let keys ..= MouseLeftDragCode(1, 11)
    let keys ..= MouseLeftReleaseCode(1, 11)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("hree", @*, msg)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousetime&
  call test_override('no_query_mouse', 0)
  close!
endfunc

" Test for modeless word selection (double click)
func Test_modeless_word_selection()
  CheckFeature clipboard_working
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm mousetime=200
  call WaitForResponses()

  new
  call setline(1, ['one two three', 'foo bar baz'])
  redraw!

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val

    " select multiple words within a line
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 6)
    let keys ..= MouseLeftReleaseCode(1, 6)
    let keys ..= MouseLeftClickCode(1, 6)
    let keys ..= MouseLeftDragCode(1, 10)
    let keys ..= MouseLeftReleaseCode(1, 10)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("two three", @*, msg)

    " select a single word
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(2, 6)
    let keys ..= MouseLeftReleaseCode(2, 6)
    let keys ..= MouseLeftClickCode(2, 6)
    let keys ..= MouseLeftReleaseCode(2, 6)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("bar", @*, msg)

    " select multiple words backwards within a line
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(2, 11)
    let keys ..= MouseLeftReleaseCode(2, 11)
    let keys ..= MouseLeftClickCode(2, 11)
    let keys ..= MouseLeftDragCode(2, 7)
    let keys ..= MouseLeftReleaseCode(2, 7)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("bar baz", @*, msg)

    " select multiple words backwards across lines
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(2, 7)
    let keys ..= MouseLeftReleaseCode(2, 7)
    let keys ..= MouseLeftClickCode(2, 7)
    let keys ..= MouseLeftDragCode(1, 6)
    let keys ..= MouseLeftReleaseCode(1, 6)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("two three\nfoo bar", @*, msg)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousetime&
  call test_override('no_query_mouse', 0)
  close!
endfunc

" Test for modeless line selection (triple click)
func Test_modeless_line_selection()
  CheckFeature clipboard_working
  let save_mouse = &mouse
  let save_term = &term
  let save_ttymouse = &ttymouse
  call test_override('no_query_mouse', 1)
  set mouse=a term=xterm mousetime=200
  call WaitForResponses()

  new
  call setline(1, ['one two three', 'foo bar baz'])
  redraw!

  for ttymouse_val in g:Ttymouse_values + g:Ttymouse_dec
    let msg = 'ttymouse=' .. ttymouse_val
    exe 'set ttymouse=' .. ttymouse_val

    " select single line
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(2, 6)
    let keys ..= MouseLeftReleaseCode(2, 6)
    let keys ..= MouseLeftClickCode(2, 6)
    let keys ..= MouseLeftReleaseCode(2, 6)
    let keys ..= MouseLeftClickCode(2, 6)
    let keys ..= MouseLeftReleaseCode(2, 6)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("foo bar baz\n", @*, msg)

    " select multiple lines
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(1, 6)
    let keys ..= MouseLeftReleaseCode(1, 6)
    let keys ..= MouseLeftClickCode(1, 6)
    let keys ..= MouseLeftReleaseCode(1, 6)
    let keys ..= MouseLeftClickCode(1, 6)
    let keys ..= MouseLeftDragCode(2, 12)
    let keys ..= MouseLeftReleaseCode(2, 12)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("one two three\nfoo bar baz\n", @*, msg)

    " select multiple lines backwards
    let @* = 'clean'
    call MouseRightClick(1, 1)
    call MouseRightRelease(1, 1)
    let keys = ":"
    let keys ..= MouseLeftClickCode(2, 10)
    let keys ..= MouseLeftReleaseCode(2, 10)
    let keys ..= MouseLeftClickCode(2, 10)
    let keys ..= MouseLeftReleaseCode(2, 10)
    let keys ..= MouseLeftClickCode(2, 10)
    let keys ..= MouseLeftDragCode(1, 3)
    let keys ..= MouseLeftReleaseCode(1, 3)
    let keys ..= "\<C-Y>\<CR>"
    call feedkeys(keys, "x")
    call assert_equal("one two three\nfoo bar baz\n", @*, msg)
  endfor

  let &mouse = save_mouse
  let &term = save_term
  let &ttymouse = save_ttymouse
  set mousetime&
  call test_override('no_query_mouse', 0)
  close!
endfunc

" vim: shiftwidth=2 sts=2 expandtab
