" tests for listener_add() and listener_remove()

func s:StoreList(s, e, a, l)
  let s:start = a:s
  let s:end = a:e
  let s:added = a:a
  let s:text = getline(a:s)
  let s:list = a:l
endfunc

func s:AnotherStoreList(l)
  let s:list2 = a:l
endfunc

func s:EvilStoreList(l)
  let s:list3 = a:l
  call assert_fails("call add(a:l, 'myitem')", "E742:")
endfunc

func Test_listening()
  new
  call setline(1, ['one', 'two'])
  let s:list = []
  let id = listener_add({b, s, e, a, l -> s:StoreList(s, e, a, l)})
  call setline(1, 'one one')
  call listener_flush()
  call assert_equal([{'lnum': 1, 'end': 2, 'col': 1, 'added': 0}], s:list)

  " Undo is also a change
  set undolevels&  " start new undo block
  call append(2, 'two two')
  undo
  call assert_equal([{'lnum': 3, 'end': 3, 'col': 1, 'added': 1}], s:list)
  redraw
  " the two changes are not merged
  call assert_equal([{'lnum': 3, 'end': 4, 'col': 1, 'added': -1}], s:list)
  1

  " Two listeners, both get called.  Also check column.
  call setline(1, ['one one', 'two'])
  call listener_flush()
  let id2 = listener_add({b, s, e, a, l -> s:AnotherStoreList(l)})
  let s:list = []
  let s:list2 = []
  exe "normal $asome\<Esc>"
  redraw
  call assert_equal([{'lnum': 1, 'end': 2, 'col': 8, 'added': 0}], s:list)
  call assert_equal([{'lnum': 1, 'end': 2, 'col': 8, 'added': 0}], s:list2)

  " removing listener works
  call listener_remove(id2)
  call setline(1, ['one one', 'two'])
  call listener_flush()
  let s:list = []
  let s:list2 = []
  call setline(3, 'three')
  redraw
  call assert_equal([{'lnum': 3, 'end': 3, 'col': 1, 'added': 1}], s:list)
  call assert_equal([], s:list2)

  " a change above a previous change without a line number change is reported
  " together
  call setline(1, ['one one', 'two'])
  call listener_flush(bufnr())
  call append(2, 'two two')
  call setline(1, 'something')
  call bufnr()->listener_flush()
  call assert_equal([{'lnum': 3, 'end': 3, 'col': 1, 'added': 1},
	\ {'lnum': 1, 'end': 2, 'col': 1, 'added': 0}], s:list)
  call assert_equal(1, s:start)
  call assert_equal(3, s:end)
  call assert_equal(1, s:added)

  " an insert just above a previous change that was the last one does not get
  " merged
  call setline(1, ['one one', 'two'])
  call listener_flush()
  let s:list = []
  call setline(2, 'something')
  call append(1, 'two two')
  call assert_equal([{'lnum': 2, 'end': 3, 'col': 1, 'added': 0}], s:list)
  call listener_flush()
  call assert_equal([{'lnum': 2, 'end': 2, 'col': 1, 'added': 1}], s:list)

  " an insert above a previous change causes a flush
  call setline(1, ['one one', 'two'])
  call listener_flush()
  call setline(2, 'something')
  call append(0, 'two two')
  call assert_equal([{'lnum': 2, 'end': 3, 'col': 1, 'added': 0}], s:list)
  call assert_equal('something', s:text)
  call listener_flush()
  call assert_equal([{'lnum': 1, 'end': 1, 'col': 1, 'added': 1}], s:list)
  call assert_equal('two two', s:text)

  " a delete at a previous change that was the last one does not get merged
  call setline(1, ['one one', 'two'])
  call listener_flush()
  let s:list = []
  call setline(2, 'something')
  2del
  call assert_equal([{'lnum': 2, 'end': 3, 'col': 1, 'added': 0}], s:list)
  call listener_flush()
  call assert_equal([{'lnum': 2, 'end': 3, 'col': 1, 'added': -1}], s:list)

  " a delete above a previous change causes a flush
  call setline(1, ['one one', 'two'])
  call listener_flush()
  call setline(2, 'another')
  1del
  call assert_equal([{'lnum': 2, 'end': 3, 'col': 1, 'added': 0}], s:list)
  call assert_equal(2, s:start)
  call assert_equal('another', s:text)
  call listener_flush()
  call assert_equal([{'lnum': 1, 'end': 2, 'col': 1, 'added': -1}], s:list)
  call assert_equal('another', s:text)

  " the "o" command first adds an empty line and then changes it
  %del
  call setline(1, ['one one', 'two'])
  call listener_flush()
  let s:list = []
  exe "normal Gofour\<Esc>"
  redraw
  call assert_equal([{'lnum': 3, 'end': 3, 'col': 1, 'added': 1},
	\ {'lnum': 3, 'end': 4, 'col': 1, 'added': 0}], s:list)

  " Remove last listener
  let s:list = []
  call listener_remove(id)
  call setline(1, 'asdfasdf')
  redraw
  call assert_equal([], s:list)

  " Trying to change the list fails
  let id = listener_add({b, s, e, a, l -> s:EvilStoreList(l)})
  let s:list3 = []
  call setline(1, 'asdfasdf')
  redraw
  call assert_equal([{'lnum': 1, 'end': 2, 'col': 1, 'added': 0}], s:list3)

  eval id->listener_remove()
  bwipe!
endfunc

func s:StoreListArgs(buf, start, end, added, list)
  let s:buf = a:buf
  let s:start = a:start
  let s:end = a:end
  let s:added = a:added
  let s:list = a:list
endfunc

func Test_listener_args()
  new
  call setline(1, ['one', 'two'])
  let s:list = []
  let id = listener_add('s:StoreListArgs')

  " just one change
  call setline(1, 'one one')
  call listener_flush()
  call assert_equal(bufnr(''), s:buf)
  call assert_equal(1, s:start)
  call assert_equal(2, s:end)
  call assert_equal(0, s:added)
  call assert_equal([{'lnum': 1, 'end': 2, 'col': 1, 'added': 0}], s:list)

  " two disconnected changes
  call setline(1, ['one', 'two', 'three', 'four'])
  call listener_flush()
  call setline(1, 'one one')
  call setline(3, 'three three')
  call listener_flush()
  call assert_equal(bufnr(''), s:buf)
  call assert_equal(1, s:start)
  call assert_equal(4, s:end)
  call assert_equal(0, s:added)
  call assert_equal([{'lnum': 1, 'end': 2, 'col': 1, 'added': 0},
	\ {'lnum': 3, 'end': 4, 'col': 1, 'added': 0}], s:list)

  " add and remove lines
  call setline(1, ['one', 'two', 'three', 'four', 'five', 'six'])
  call listener_flush()
  call append(2, 'two two')
  4del
  call append(5, 'five five')
  call listener_flush()
  call assert_equal(bufnr(''), s:buf)
  call assert_equal(3, s:start)
  call assert_equal(6, s:end)
  call assert_equal(1, s:added)
  call assert_equal([{'lnum': 3, 'end': 3, 'col': 1, 'added': 1},
	\ {'lnum': 4, 'end': 5, 'col': 1, 'added': -1},
	\ {'lnum': 6, 'end': 6, 'col': 1, 'added': 1}], s:list)

  " split a line then insert one, should get two disconnected change lists
  call setline(1, 'split here')
  call listener_flush()
  let s:list = []
  exe "normal 1ggwi\<CR>\<Esc>"
  1
  normal o
  call assert_equal([{'lnum': 1, 'end': 2, 'col': 7, 'added': 1}], s:list)
  call listener_flush()
  call assert_equal([{'lnum': 2, 'end': 2, 'col': 1, 'added': 1}], s:list)

  call listener_remove(id)
  bwipe!

  " Invalid arguments
  call assert_fails('call listener_add([])', 'E921:')
  call assert_fails('call listener_add("s:StoreListArgs", [])', 'E730:')
  call assert_fails('call listener_flush([])', 'E730:')

  call assert_fails('eval ""->listener_add()', 'E119:')
endfunc

func s:StoreBufList(buf, start, end, added, list)
  let s:bufnr = a:buf
  let s:list = a:list
endfunc

func Test_listening_other_buf()
  new
  call setline(1, ['one', 'two'])
  let bufnr = bufnr('')
  normal ww
  let id = bufnr->listener_add(function('s:StoreBufList'))
  let s:list = []
  call setbufline(bufnr, 1, 'hello')
  redraw
  call assert_equal(bufnr, s:bufnr)
  call assert_equal([{'lnum': 1, 'end': 2, 'col': 1, 'added': 0}], s:list)

  call listener_remove(id)
  exe "buf " .. bufnr
  bwipe!
endfunc

func Test_listener_garbage_collect()
  func MyListener(x, bufnr, start, end, added, changes)
    " NOP
  endfunc

  new
  let id = listener_add(function('MyListener', [{}]), bufnr(''))
  call test_garbagecollect_now()
  " must not crash caused by invalid memory access
  normal ia
  call assert_true(v:true)

  call listener_remove(id)
  delfunc MyListener
  bwipe!
endfunc

" This verifies the fix for issue #4455
func Test_listener_caches_buffer_line()
  new
  inoremap <silent> <CR> <CR><Esc>O

  function EchoChanges(bufnr, start, end, added, changes)
    for l:change in a:changes
      let text = getbufline(a:bufnr, l:change.lnum, l:change.end-1+l:change.added)
    endfor
  endfunction
  let lid = listener_add("EchoChanges")
  set autoindent
  set cindent

  call setline(1, ["{", "\tif true {}", "}"])
  exe "normal /{}\nl"
  call feedkeys("i\r\e", 'xt')
  call assert_equal(["{", "\tif true {", "", "\t}", "}"], getline(1, 5))

  bwipe!
  delfunc EchoChanges
  call listener_remove(lid)
  iunmap <CR>
  set nocindent
endfunc

" Verify the fix for issue #4908
func Test_listener_undo_line_number()
  function DoIt()
    " NOP
  endfunction
  function EchoChanges(bufnr, start, end, added, changes)
    call DoIt()
  endfunction

  new
  let lid = listener_add("EchoChanges")
  call setline(1, ['a', 'b', 'c'])
  set undolevels&  " start new undo block
  call feedkeys("ggcG\<Esc>", 'xt')
  undo

  bwipe!
  delfunc DoIt
  delfunc EchoChanges
  call listener_remove(lid)
endfunc

func Test_listener_undo_delete_all()
  new
  call setline(1, [1, 2, 3, 4])
  let s:changes = []
  func s:ExtendList(bufnr, start, end, added, changes)
    call extend(s:changes, a:changes)
  endfunc
  let id = listener_add('s:ExtendList')

  set undolevels&  " start new undo block
  normal! ggdG
  undo
  call listener_flush()
  call assert_equal(2, s:changes->len())
  " delete removes four lines, empty line remains
  call assert_equal({'lnum': 1, 'end': 5, 'col': 1, 'added': -4}, s:changes[0])
  " undo replaces empty line and adds 3 lines
  call assert_equal({'lnum': 1, 'end': 2, 'col': 1, 'added': 3}, s:changes[1])

  call listener_remove(id)
  delfunc s:ExtendList
  unlet s:changes
  bwipe!
endfunc

func Test_listener_cleared_newbuf()
  func Listener(bufnr, start, end, added, changes)
    let g:gotCalled += 1
  endfunc
  new
  " check that listening works
  let g:gotCalled = 0
  let lid = listener_add("Listener")
  call feedkeys("axxx\<Esc>", 'xt')
  call listener_flush(bufnr())
  call assert_equal(1, g:gotCalled)
  %bwipe!
  let bufnr = bufnr()
  let b:testing = 123
  let lid = listener_add("Listener")
  enew!
  " check buffer is reused
  call assert_equal(bufnr, bufnr())
  call assert_false(exists('b:testing'))

  " check that listening stops when reusing the buffer
  let g:gotCalled = 0
  call feedkeys("axxx\<Esc>", 'xt')
  call listener_flush(bufnr())
  call assert_equal(0, g:gotCalled)
  unlet g:gotCalled

  bwipe!
  delfunc Listener
endfunc

func Test_col_after_deletion_moved_cur()
  func Listener(bufnr, start, end, added, changes)
    call assert_equal([#{lnum: 1, end: 2, added: 0, col: 2}], a:changes)
  endfunc
  new
  call setline(1, ['foo'])
  let lid = listener_add('Listener')
  call feedkeys("lD", 'xt')
  call listener_flush()
  bwipe!
  delfunc Listener
endfunc

func Test_remove_listener_in_callback()
  new
  let s:ID = listener_add('Listener')
  func Listener(...)
    call listener_remove(s:ID)
    let g:listener_called = 'yes'
  endfunc
  call setline(1, ['foo'])
  call feedkeys("lD", 'xt')
  call listener_flush()
  call assert_equal('yes', g:listener_called)

  bwipe!
  delfunc Listener
  unlet g:listener_called
endfunc

" When multiple listeners are registered, remove one listener and verify the
" other listener is still called
func Test_remove_one_listener_in_callback()
  new
  let g:listener1_called = 0
  let g:listener2_called = 0
  let s:ID1 = listener_add('Listener1')
  let s:ID2 = listener_add('Listener2')
  func Listener1(...)
    call listener_remove(s:ID1)
    let g:listener1_called += 1
  endfunc
  func Listener2(...)
    let g:listener2_called += 1
  endfunc
  call setline(1, ['foo'])
  call feedkeys("~", 'xt')
  call listener_flush()
  call feedkeys("~", 'xt')
  call listener_flush()
  call assert_equal(1, g:listener1_called)
  call assert_equal(2, g:listener2_called)

  call listener_remove(s:ID2)
  bwipe!
  delfunc Listener1
  delfunc Listener2
  unlet g:listener1_called
  unlet g:listener2_called
endfunc

func Test_no_change_for_empty_undo()
  new
  let text = ['some word here', 'second line']
  call setline(1, text)
  let g:entries = []
  func Listener(bufnr, start, end, added, changes)
    for change in a:changes
      call add(g:entries, [change.lnum, change.end, change.added])
    endfor
  endfunc
  let s:ID = listener_add('Listener')
  let @a = "one line\ntwo line\nthree line"
  set undolevels&  " start new undo block
  call feedkeys('fwviw"ap', 'xt')
  call listener_flush(bufnr())
  " first change deletes "word", second change inserts the register
  call assert_equal([[1, 2, 0], [1, 2, 2]], g:entries)
  let g:entries = []

  set undolevels&  " start new undo block
  undo
  call listener_flush(bufnr())
  call assert_equal([[1, 4, -2]], g:entries)
  call assert_equal(text, getline(1, 2))

  call listener_remove(s:ID)
  bwipe!
  unlet g:entries
  delfunc Listener
endfunc


" vim: shiftwidth=2 sts=2 expandtab
