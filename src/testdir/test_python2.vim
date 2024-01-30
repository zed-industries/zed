" Test for python 2 commands.

source check.vim
CheckFeature python
CheckFeature quickfix
source shared.vim

" NOTE: This will cause errors when run under valgrind.
" This would require recompiling Python with:
"   ./configure --without-pymalloc
" See http://svn.python.org/view/python/trunk/Misc/README.valgrind?view=markup
"

" This function should be called first. This sets up python functions used by
" the other tests.
func Test_AAA_python_setup()
  py << trim EOF
    import vim
    import sys

    def emsg(ei):
      return ei[0].__name__ + ':' + repr(ei[1].args)

    def ee(expr, g=globals(), l=locals()):
      try:
        exec(expr, g, l)
      except:
        ei = sys.exc_info()
        msg = emsg(ei)
        msg = msg.replace('TypeError:(\'argument 1 ', 'TypeError:(\'')
        if expr.find('None') > -1:
          msg = msg.replace('TypeError:(\'iteration over non-sequence\',)',
                        'TypeError:("\'NoneType\' object is not iterable",)')
        if expr.find('FailingNumber') > -1:
          msg = msg.replace(', not \'FailingNumber\'', '').replace('"', '\'')
          msg = msg.replace('TypeError:(\'iteration over non-sequence\',)',
                    'TypeError:("\'FailingNumber\' object is not iterable",)')
        if msg.find('(\'\'') > -1 or msg.find('(\'can\'t') > -1:
          msg = msg.replace('(\'', '("').replace('\',)', '",)')
        # Some Python versions say can't, others cannot.
        if msg.find('can\'t') > -1:
          msg = msg.replace('can\'t', 'cannot')
        # Some Python versions use single quote, some double quote
        if msg.find('"cannot ') > -1:
          msg = msg.replace('"cannot ', '\'cannot ')
        if msg.find(' attributes"') > -1:
          msg = msg.replace(' attributes"', ' attributes\'')
        if expr == 'fd(self=[])':
          # HACK: PyMapping_Check changed meaning
          msg = msg.replace('AttributeError:(\'keys\',)',
                  'TypeError:(\'unable to convert list to vim dictionary\',)')
        vim.current.buffer.append(expr + ':' + msg)
      else:
        vim.current.buffer.append(expr + ':NOT FAILED')
  EOF
endfunc

func Test_pydo()
  new

  " Check deleting lines does not trigger an ml_get error.
  call setline(1, ['one', 'two', 'three'])
  pydo vim.command("%d_")
  call assert_equal([''], getline(1, '$'))

  call setline(1, ['one', 'two', 'three'])
  pydo vim.command("1,2d_")
  call assert_equal(['three'], getline(1, '$'))

  call setline(1, ['one', 'two', 'three'])
  pydo vim.command("2,3d_"); return "REPLACED"
  call assert_equal(['REPLACED'], getline(1, '$'))

  call setline(1, ['one', 'two', 'three'])
  2,3pydo vim.command("1,2d_"); return "REPLACED"
  call assert_equal(['three'], getline(1, '$'))

  bwipe!

  " Check switching to another buffer does not trigger an ml_get error.
  new
  let wincount = winnr('$')
  call setline(1, ['one', 'two', 'three'])
  pydo vim.command("new")
  call assert_equal(wincount + 1, winnr('$'))
  bwipe!
  bwipe!

  " Try modifying a buffer with 'nomodifiable' set
  set nomodifiable
  call assert_fails('pydo toupper(line)', 'E21:')
  set modifiable

  " Invalid command
  call AssertException(['pydo non_existing_cmd'],
        \ "Vim(pydo):NameError: global name 'non_existing_cmd' is not defined")
  call AssertException(["pydo raise Exception('test')"],
        \ 'Vim(pydo):Exception: test')
  call AssertException(["pydo {lambda}"],
        \ 'Vim(pydo):SyntaxError: invalid syntax')
endfunc

func Test_set_cursor()
  " Check that setting the cursor position works.
  new
  call setline(1, ['first line', 'second line'])
  normal gg
  pydo vim.current.window.cursor = (1, 5)
  call assert_equal([1, 6], [line('.'), col('.')])

  " Check that movement after setting cursor position keeps current column.
  normal j
  call assert_equal([2, 6], [line('.'), col('.')])
endfunc

func Test_vim_function()
  " Check creating vim.Function object

  func s:foo()
    return matchstr(expand('<sfile>'), '<SNR>\zs\d\+_foo$')
  endfunc
  let name = '<SNR>' . s:foo()

  try
    py f = vim.bindeval('function("s:foo")')
    call assert_equal(name, pyeval('f.name'))
  catch
    call assert_false(v:exception)
  endtry

  try
    py f = vim.Function('\x80\xfdR' + vim.eval('s:foo()'))
    call assert_equal(name, 'f.name'->pyeval())
  catch
    call assert_false(v:exception)
  endtry

  " Non-existing function attribute
  call AssertException(["let x = pyeval('f.abc')"],
        \ 'Vim(let):AttributeError: abc')

  py del f
  delfunc s:foo
endfunc

func Test_skipped_python_command_does_not_affect_pyxversion()
  set pyxversion=0
  if 0
    python import vim
  endif
  call assert_equal(0, &pyxversion)  " This assertion would have failed with Vim 8.0.0251. (pyxversion was introduced in 8.0.0251.)
endfunc

func _SetUpHiddenBuffer()
  new
  edit hidden
  setlocal bufhidden=hide

  enew
  let lnum = 0
  while lnum < 10
    call append( 1, string( lnum ) )
    let lnum = lnum + 1
  endwhile
  normal G

  call assert_equal( line( '.' ), 11 )
endfunc

func _CleanUpHiddenBuffer()
  bwipe! hidden
  bwipe!
endfunc

func Test_Write_To_HiddenBuffer_Does_Not_Fix_Cursor_Clear()
  call _SetUpHiddenBuffer()
  py vim.buffers[ int( vim.eval( 'bufnr("hidden")' ) ) ][:] = None
  call assert_equal( line( '.' ), 11 )
  call _CleanUpHiddenBuffer()
endfunc

func Test_Write_To_HiddenBuffer_Does_Not_Fix_Cursor_List()
  call _SetUpHiddenBuffer()
  py vim.buffers[ int( vim.eval( 'bufnr("hidden")' ) ) ][:] = [ 'test' ]
  call assert_equal( line( '.' ), 11 )
  call _CleanUpHiddenBuffer()
endfunc

func Test_Write_To_HiddenBuffer_Does_Not_Fix_Cursor_Str()
  call _SetUpHiddenBuffer()
  py vim.buffers[ int( vim.eval( 'bufnr("hidden")' ) ) ][0] = 'test'
  call assert_equal( line( '.' ), 11 )
  call _CleanUpHiddenBuffer()
endfunc

func Test_Write_To_HiddenBuffer_Does_Not_Fix_Cursor_ClearLine()
  call _SetUpHiddenBuffer()
  py vim.buffers[ int( vim.eval( 'bufnr("hidden")' ) ) ][0] = None
  call assert_equal( line( '.' ), 11 )
  call _CleanUpHiddenBuffer()
endfunc

func _SetUpVisibleBuffer()
  new
  let lnum = 0
  while lnum < 10
    call append( 1, string( lnum ) )
    let lnum = lnum + 1
  endwhile
  normal G
  call assert_equal( line( '.' ), 11 )
endfunc

func Test_Write_To_Current_Buffer_Fixes_Cursor_Clear()
  call _SetUpVisibleBuffer()

  py vim.current.buffer[:] = None
  call assert_equal( line( '.' ), 1 )

  bwipe!
endfunc

func Test_Write_To_Current_Buffer_Fixes_Cursor_List()
  call _SetUpVisibleBuffer()

  py vim.current.buffer[:] = [ 'test' ]
  call assert_equal( line( '.' ), 1 )

  bwipe!
endfunc

func Test_Write_To_Current_Buffer_Fixes_Cursor_Str()
  call _SetUpVisibleBuffer()

  py vim.current.buffer[-1] = None
  call assert_equal( line( '.' ), 10 )

  bwipe!
endfunc

func Test_Catch_Exception_Message()
  try
    py raise RuntimeError( 'TEST' )
  catch /.*/
    call assert_match( '^Vim(.*):RuntimeError: TEST$', v:exception )
  endtry
endfunc

" Test for various heredoc syntax
func Test_python_heredoc()
  python << END
s='A'
END
  python <<
s+='B'
.
  python << trim END
    s+='C'
  END
  python << trim
    s+='D'
  .
  python << trim eof
    s+='E'
  eof
  call assert_equal('ABCDE', pyxeval('s'))
endfunc

" Test for the buffer range object
func Test_python_range()
  new
  call setline(1, ['one', 'two', 'three'])
  py b = vim.current.buffer
  py r = b.range(1, 3)
  call assert_equal(0, pyeval('r.start'))
  call assert_equal(2, pyeval('r.end'))
  call assert_equal('one', pyeval('r[0]'))
  call assert_equal('one', pyeval('r[-3]'))
  call assert_equal('three', pyeval('r[-4]'))
  call assert_equal(['two', 'three'], pyeval('r[1:]'))
  py r[0] = 'green'
  call assert_equal(['green', 'two', 'three'], getline(1, '$'))
  py r[0:2] = ['red', 'blue']
  call assert_equal(['red', 'blue', 'three'], getline(1, '$'))
  call assert_equal(['start', 'end', '__members__'], pyeval('r.__members__'))

  " try different invalid start/end index for the range slice
  %d
  call setline(1, ['one', 'two', 'three'])
  py r[-10:1] = ["a"]
  py r[10:12] = ["b"]
  py r[-10:-9] = ["c"]
  py r[1:0] = ["d"]
  call assert_equal(['c', 'd', 'a', 'two', 'three', 'b'], getline(1, '$'))

  " The following code used to trigger an ml_get error
  %d
  let x = pyeval('r[:]')

  " Non-existing range attribute
  call AssertException(["let x = pyeval('r.abc')"],
        \ 'Vim(let):AttributeError: abc')

  close!
endfunc

" Test for the python tabpage object
func Test_python_tabpage()
  tabnew
  py t = vim.tabpages[1]
  py wl = t.windows
  tabclose
  " Accessing a closed tabpage
  call AssertException(["let n = pyeval('t.number')"],
        \ 'Vim(let):vim.error: attempt to refer to deleted tab page')
  call AssertException(["let n = pyeval('len(wl)')"],
        \ 'Vim(let):vim.error: attempt to refer to deleted tab page')
  call AssertException(["py w = wl[0]"],
        \ 'Vim(python):vim.error: attempt to refer to deleted tab page')
  call AssertException(["py vim.current.tabpage = t"],
        \ 'Vim(python):vim.error: attempt to refer to deleted tab page')
  call assert_match('<tabpage object (deleted)', pyeval('repr(t)'))
  %bw!
endfunc

" Test for the python window object
func Test_python_window()
  " Test for setting the window height
  10new
  py vim.current.window.height = 5
  call assert_equal(5, winheight(0))
  py vim.current.window.height = 3.2
  call assert_equal(3, winheight(0))

  " Test for setting the window width
  10vnew
  py vim.current.window.width = 6
  call assert_equal(6, winwidth(0))

  " Try accessing a closed window
  py w = vim.current.window
  py wopts = w.options
  close
  " Access the attributes of a closed window
  call AssertException(["let n = pyeval('w.number')"],
        \ 'Vim(let):vim.error: attempt to refer to deleted window')
  call AssertException(["py w.height = 5"],
        \ 'Vim(python):vim.error: attempt to refer to deleted window')
  call AssertException(["py vim.current.window = w"],
        \ 'Vim(python):vim.error: attempt to refer to deleted window')
  " Try to set one of the options of the closed window
  " The following caused an ASAN failure
  call AssertException(["py wopts['list'] = False"],
        \ 'vim.error: attempt to refer to deleted window')
  call assert_match('<window object (deleted)', pyeval("repr(w)"))
  %bw!
endfunc

" Test for the python List object
func Test_python_list()
  let l = [1, 2]
  py pl = vim.bindeval('l')
  call assert_equal(['locked', '__members__'], pyeval('pl.__members__'))

  " Try to convert a null List
  call AssertException(["py t = vim.eval('test_null_list()')"],
        \ 'Vim(python):SystemError: error return without exception set')

  " Try to convert a List with a null List item
  call AssertException(["py t = vim.eval('[test_null_list()]')"],
        \ 'Vim(python):SystemError: error return without exception set')

  " Try to bind a null List variable (works because an empty list is used)
  let cmds =<< trim END
    let l = test_null_list()
    py ll = vim.bindeval('l')
  END
  call AssertException(cmds, '')

  let l = []
  py l = vim.bindeval('l')
  py f = vim.bindeval('function("strlen")')
  " Extending List directly with different types
  py l.extend([1, "as'd", [1, 2, f, {'a': 1}]])
  call assert_equal([1, "as'd", [1, 2, function("strlen"), {'a': 1}]], l)
  call assert_equal([1, 2, function("strlen"), {'a': 1}], l[-1])
  call assert_fails('echo l[-4]', 'E684:')

  " List assignment
  py l[0] = 0
  call assert_equal([0, "as'd", [1, 2, function("strlen"), {'a': 1}]], l)
  py l[-2] = f
  call assert_equal([0, function("strlen"), [1, 2, function("strlen"), {'a': 1}]], l)

  " appending to a list
  let l = [1, 2]
  py ll = vim.bindeval('l')
  py ll[2] = 8
  call assert_equal([1, 2, 8], l)

  " Using dict as an index
  call AssertException(['py ll[{}] = 10'],
        \ 'Vim(python):TypeError: index must be int or slice, not dict')
endfunc

" Test for the python Dict object
func Test_python_dict()
  let d = {}
  py pd = vim.bindeval('d')
  call assert_equal(['locked', 'scope', '__members__'],
        \ pyeval('pd.__members__'))

  " Try to convert a null Dict
  call AssertException(["py t = vim.eval('test_null_dict()')"],
        \ 'Vim(python):SystemError: error return without exception set')

  " Try to convert a Dict with a null List value
  call AssertException(["py t = vim.eval(\"{'a' : test_null_list()}\")"],
        \ 'Vim(python):SystemError: error return without exception set')

  " Try to convert a Dict with a null string key
  py t = vim.eval("{test_null_string() : 10}")
  call assert_fails("let d = pyeval('t')", 'E859:')

  " Dict length
  let d = {'a' : 10, 'b' : 20}
  py d = vim.bindeval('d')
  call assert_equal(2, pyeval('len(d)'))

  " Deleting a non-existing key
  call AssertException(["py del d['c']"], "Vim(python):KeyError: 'c'")
endfunc

" Extending Dictionary directly with different types
func Test_python_dict_extend()
  let d = {}
  func d.f()
    return 1
  endfunc

  py f = vim.bindeval('function("strlen")')
  py << trim EOF
    d = vim.bindeval('d')
    d['1'] = 'asd'
    d.update()  # Must not do anything, including throwing errors
    d.update(b = [1, 2, f])
    d.update((('-1', {'a': 1}),))
    d.update({'0': -1})
    dk = d.keys()
    dv = d.values()
    di = d.items()
    cmpfun = lambda a, b: cmp(repr(a), repr(b))
    dk.sort(cmpfun)
    dv.sort(cmpfun)
    di.sort(cmpfun)
  EOF

  " Try extending a locked dictionary
  lockvar d
  call AssertException(["py d.update({'b' : 20})"],
        \ 'Vim(python):vim.error: dictionary is locked')
  unlockvar d

  call assert_equal(1, pyeval("d['f'](self={})"))
  call assert_equal("['-1', '0', '1', 'b', 'f']", pyeval('repr(dk)'))
  call assert_equal("['asd', -1L, <vim.Function '1'>, <vim.dictionary object at >, <vim.list object at >]", substitute(pyeval('repr(dv)'),'0x\x\+','','g'))
  call assert_equal("[('-1', <vim.dictionary object at >), ('0', -1L), ('1', 'asd'), ('b', <vim.list object at >), ('f', <vim.Function '1'>)]", substitute(pyeval('repr(di)'),'0x\x\+','','g'))
  call assert_equal(['0', '1', 'b', 'f', '-1'], keys(d))
  call assert_equal("[-1, 'asd', [1, 2, function('strlen')], function('1'), {'a': 1}]", string(values(d)))
  py del dk
  py del di
  py del dv
endfunc

func Test_python_list_del_items()
  " removing items with del
  let l = [0, function("strlen"), [1, 2, function("strlen"), {'a': 1}]]
  py l = vim.bindeval('l')
  py del l[2]
  call assert_equal("[0, function('strlen')]", string(l))

  let l = range(8)
  py l = vim.bindeval('l')
  py del l[:3]
  py del l[1:]
  call assert_equal([3], l)

  " removing items out of range: silently skip items that don't exist

  " The following two ranges delete nothing as they match empty list:
  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[2:1]
  call assert_equal([0, 1, 2, 3], l)
  py del l[2:2]
  call assert_equal([0, 1, 2, 3], l)
  py del l[2:3]
  call assert_equal([0, 1, 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[2:4]
  call assert_equal([0, 1], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[2:5]
  call assert_equal([0, 1], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[2:6]
  call assert_equal([0, 1], l)

  " The following two ranges delete nothing as they match empty list:
  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[-1:2]
  call assert_equal([0, 1, 2, 3], l)
  py del l[-2:2]
  call assert_equal([0, 1, 2, 3], l)
  py del l[-3:2]
  call assert_equal([0, 2, 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[-4:2]
  call assert_equal([2, 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[-5:2]
  call assert_equal([2, 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[-6:2]
  call assert_equal([2, 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[::2]
  call assert_equal([1, 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[3:0:-2]
  call assert_equal([0, 2], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py del l[2:4:-2]
  let l = [0, 1, 2, 3]
endfunc

func Test_python_dict_del_items()
  let d = eval("{'0' : -1, '1' : 'asd', 'b' : [1, 2, function('strlen')], 'f' : function('min'), '-1' : {'a': 1}}")
  py d = vim.bindeval('d')
  py del d['-1']
  py del d['f']
  call assert_equal([1, 2, function('strlen')], pyeval('d.get(''b'', 1)'))
  call assert_equal([1, 2, function('strlen')], pyeval('d.pop(''b'')'))
  call assert_equal(1, pyeval('d.get(''b'', 1)'))
  call assert_equal('asd', pyeval('d.pop(''1'', 2)'))
  call assert_equal(2, pyeval('d.pop(''1'', 2)'))
  call assert_equal('True', pyeval('repr(d.has_key(''0''))'))
  call assert_equal('False', pyeval('repr(d.has_key(''1''))'))
  call assert_equal('True', pyeval('repr(''0'' in d)'))
  call assert_equal('False', pyeval('repr(''1'' in d)'))
  call assert_equal("['0']", pyeval('repr(list(iter(d)))'))
  call assert_equal({'0' : -1}, d)
  call assert_equal("('0', -1L)", pyeval('repr(d.popitem())'))
  call assert_equal('None', pyeval('repr(d.get(''0''))'))
  call assert_equal('[]', pyeval('repr(list(iter(d)))'))
endfunc

" Slice assignment to a list
func Test_python_slice_assignment()
  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py l[0:0] = ['a']
  call assert_equal(['a', 0, 1, 2, 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py l[1:2] = ['b']
  call assert_equal([0, 'b', 2, 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py l[2:4] = ['c']
  call assert_equal([0, 1, 'c'], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py l[4:4] = ['d']
  call assert_equal([0, 1, 2, 3, 'd'], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py l[-1:2] = ['e']
  call assert_equal([0, 1, 2, 'e', 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py l[-10:2] = ['f']
  call assert_equal(['f', 2, 3], l)

  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  py l[2:-10] = ['g']
  call assert_equal([0, 1, 'g', 2, 3], l)

  let l = []
  py l = vim.bindeval('l')
  py l[0:0] = ['h']
  call assert_equal(['h'], l)

  let l = range(8)
  py l = vim.bindeval('l')
  py l[2:6:2] = [10, 20]
  call assert_equal([0, 1, 10, 3, 20, 5, 6, 7], l)

  let l = range(8)
  py l = vim.bindeval('l')
  py l[6:2:-2] = [10, 20]
  call assert_equal([0, 1, 2, 3, 20, 5, 10, 7], l)

  let l = range(8)
  py l = vim.bindeval('l')
  py l[6:2] = ()
  call assert_equal([0, 1, 2, 3, 4, 5, 6, 7], l)

  let l = range(8)
  py l = vim.bindeval('l')
  py l[6:2:1] = ()
  call assert_equal([0, 1, 2, 3, 4, 5, 6, 7], l)

  let l = range(8)
  py l = vim.bindeval('l')
  py l[2:2:1] = ()
  call assert_equal([0, 1, 2, 3, 4, 5, 6, 7], l)

  call AssertException(["py x = l[10:11:0]"],
        \ "Vim(python):ValueError: slice step cannot be zero")
endfunc

" Locked variables
func Test_python_lockedvar()
  new
  py cb = vim.current.buffer
  let l = [0, 1, 2, 3]
  py l = vim.bindeval('l')
  lockvar! l
  py << trim EOF
    try:
        l[2]='i'
    except vim.error:
        cb.append('l[2] threw vim.error: ' + emsg(sys.exc_info()))
  EOF
  call assert_equal(['', "l[2] threw vim.error: error:('list is locked',)"],
        \ getline(1, '$'))

  " Try to concatenate a locked list
  call AssertException(['py l += [4, 5]'],
        \ 'Vim(python):vim.error: list is locked')

  call assert_equal([0, 1, 2, 3], l)
  unlockvar! l
  close!
endfunc

" Test for calling a function
func Test_python_function_call()
  func New(...)
    return ['NewStart'] + a:000 + ['NewEnd']
  endfunc

  func DictNew(...) dict
    return ['DictNewStart'] + a:000 + ['DictNewEnd', self]
  endfunc

  new
  let l = [function('New'), function('DictNew')]
  py l = vim.bindeval('l')
  py l.extend(list(l[0](1, 2, 3)))
  call assert_equal([function('New'), function('DictNew'), 'NewStart', 1, 2, 3, 'NewEnd'], l)
  py l.extend(list(l[1](1, 2, 3, self={'a': 'b'})))
  call assert_equal([function('New'), function('DictNew'), 'NewStart', 1, 2, 3, 'NewEnd', 'DictNewStart', 1, 2, 3, 'DictNewEnd', {'a': 'b'}], l)
  py l.extend([l[0].name])
  call assert_equal([function('New'), function('DictNew'), 'NewStart', 1, 2, 3, 'NewEnd', 'DictNewStart', 1, 2, 3, 'DictNewEnd', {'a': 'b'}, 'New'], l)
  py ee('l[1](1, 2, 3)')
  call assert_equal("l[1](1, 2, 3):error:('Vim:E725: Calling dict function without Dictionary: DictNew',)", getline(2))
  %d
  py f = l[0]
  delfunction New
  py ee('f(1, 2, 3)')
  call assert_equal("f(1, 2, 3):error:('Vim:E117: Unknown function: New',)", getline(2))
  close!
  delfunction DictNew
endfunc

func Test_python_float()
  let l = [0.0]
  py l = vim.bindeval('l')
  py l.extend([0.0])
  call assert_equal([0.0, 0.0], l)
endfunc

" Test for Dict key errors
func Test_python_dict_key_error()
  let messages = []
  py << trim EOF
    d = vim.bindeval('{}')
    m = vim.bindeval('messages')
    def em(expr, g=globals(), l=locals()):
      try:
        exec(expr, g, l)
      except:
        m.extend([sys.exc_type.__name__])

    em('d["abc1"]')
    em('d["abc1"]="\\0"')
    em('d["abc1"]=vim')
    em('d[""]=1')
    em('d["a\\0b"]=1')
    em('d[u"a\\0b"]=1')
    em('d.pop("abc1")')
    em('d.popitem()')
    del em
    del m
  EOF

  call assert_equal(['KeyError', 'TypeError', 'TypeError', 'ValueError',
        \ 'TypeError', 'TypeError', 'KeyError', 'KeyError'], messages)
  unlet messages
endfunc

" Test for locked and scope attributes
func Test_python_lock_scope_attr()
  let d = {} | let dl = {} | lockvar dl
  let res = []
  for s in split("d dl v: g:")
    let name = tr(s, ':', 's')
    execute 'py ' .. name .. ' = vim.bindeval("' .. s .. '")'
    call add(res, s .. ' : ' .. join(map(['locked', 'scope'],
          \ 'v:val .. ":" .. pyeval(name .. "." .. v:val)'), ';'))
  endfor
  call assert_equal(['d : locked:0;scope:0', 'dl : locked:1;scope:0',
        \ 'v: : locked:2;scope:1', 'g: : locked:0;scope:2'], res)

  silent! let d.abc2 = 1
  silent! let dl.abc3 = 1
  py d.locked = True
  py dl.locked = False
  silent! let d.def = 1
  silent! let dl.def = 1
  call assert_equal({'abc2': 1}, d)
  call assert_equal({'def': 1}, dl)
  unlet d dl

  let l = [] | let ll = [] | lockvar ll
  let res = []
  for s in split("l ll")
    let name = tr(s, ':', 's')
    execute 'py ' .. name .. '=vim.bindeval("' .. s .. '")'
    call add(res, s .. ' : locked:' .. pyeval(name .. '.locked'))
  endfor
  call assert_equal(['l : locked:0', 'll : locked:1'], res)

  silent! call extend(l, [0])
  silent! call extend(ll, [0])
  py l.locked = True
  py ll.locked = False
  silent! call extend(l, [1])
  silent! call extend(ll, [1])
  call assert_equal([0], l)
  call assert_equal([1], ll)
  unlet l ll

  " Try changing an attribute of a fixed list
  py a = vim.bindeval('v:argv')
  call AssertException(['py a.locked = 0'],
        \ 'Vim(python):TypeError: cannot modify fixed list')
endfunc

" Test for pyeval()
func Test_python_pyeval()
  let l = pyeval('range(3)')
  call assert_equal([0, 1, 2], l)

  let d = pyeval('{"a": "b", "c": 1, "d": ["e"]}')
  call assert_equal([['a', 'b'], ['c', 1], ['d', ['e']]], sort(items(d)))

  let v:errmsg = ''
  call assert_equal(v:none, pyeval('None'))
  call assert_equal('', v:errmsg)

  py v = vim.eval('test_null_function()')
  call assert_equal(v:none, pyeval('v'))

  call assert_equal(0.0, pyeval('0.0'))

  " Evaluate an invalid values
  call AssertException(['let v = pyeval(''"\0"'')'], 'E859:')
  call AssertException(['let v = pyeval(''{"\0" : 1}'')'], 'E859:')
  call AssertException(['let v = pyeval("undefined_name")'],
        \ "Vim(let):NameError: name 'undefined_name' is not defined")
  call AssertException(['let v = pyeval("vim")'], 'E859:')
endfunc

" Test for vim.bindeval()
func Test_python_vim_bindeval()
  " Float
  let f = 3.14
  py f = vim.bindeval('f')
  call assert_equal(3.14, pyeval('f'))

  " Blob
  let b = 0z12
  py b = vim.bindeval('b')
  call assert_equal("\x12", pyeval('b'))

  " Bool
  call assert_equal(1, pyeval("vim.bindeval('v:true')"))
  call assert_equal(0, pyeval("vim.bindeval('v:false')"))
  call assert_equal(v:none, pyeval("vim.bindeval('v:null')"))
  call assert_equal(v:none, pyeval("vim.bindeval('v:none')"))

  " channel/job
  if has('channel')
    call assert_equal(v:none, pyeval("vim.bindeval('test_null_channel()')"))
  endif
  if has('job')
    call assert_equal(v:none, pyeval("vim.bindeval('test_null_job()')"))
  endif
endfunc

" threading
" Running pydo command (Test_pydo) before this test, stops the python thread
" from running. So this test should be run before the pydo test
func Test_aaa_python_threading()
  let l = [0]
  py l = vim.bindeval('l')
  py << trim EOF
    import threading
    import time

    class T(threading.Thread):
      def __init__(self):
        threading.Thread.__init__(self)
        self.t = 0
        self.running = True

      def run(self):
        while self.running:
          self.t += 1
          time.sleep(0.1)

    t = T()
    del T
    t.start()
  EOF

  sleep 1
  py t.running = False
  py t.join()

  " Check if the background thread is working.  Count should be 10, but on a
  " busy system (AppVeyor) it can be much lower.
  py l[0] = t.t > 4
  py del time
  py del threading
  py del t
  call assert_equal([1], l)
endfunc

" settrace
func Test_python_settrace()
  let l = []
  py l = vim.bindeval('l')
  py << trim EOF
    import sys

    def traceit(frame, event, arg):
      global l
      if event == "line":
          l.extend([frame.f_lineno])
      return traceit

    def trace_main():
      for i in range(5):
        pass
  EOF
  py sys.settrace(traceit)
  py trace_main()
  py sys.settrace(None)
  py del traceit
  py del trace_main
  call assert_equal([1, 10, 11, 10, 11, 10, 11, 10, 11, 10, 11, 10, 1], l)
endfunc

" Slice
func Test_python_list_slice()
  py ll = vim.bindeval('[0, 1, 2, 3, 4, 5]')
  py l = ll[:4]
  call assert_equal([0, 1, 2, 3], pyeval('l'))
  py l = ll[2:]
  call assert_equal([2, 3, 4, 5], pyeval('l'))
  py l = ll[:-4]
  call assert_equal([0, 1], pyeval('l'))
  py l = ll[-2:]
  call assert_equal([4, 5], pyeval('l'))
  py l = ll[2:4]
  call assert_equal([2, 3], pyeval('l'))
  py l = ll[4:2]
  call assert_equal([], pyeval('l'))
  py l = ll[-4:-2]
  call assert_equal([2, 3], pyeval('l'))
  py l = ll[-2:-4]
  call assert_equal([], pyeval('l'))
  py l = ll[:]
  call assert_equal([0, 1, 2, 3, 4, 5], pyeval('l'))
  py l = ll[0:6]
  call assert_equal([0, 1, 2, 3, 4, 5], pyeval('l'))
  py l = ll[-10:10]
  call assert_equal([0, 1, 2, 3, 4, 5], pyeval('l'))
  py l = ll[4:2:-1]
  call assert_equal([4, 3], pyeval('l'))
  py l = ll[::2]
  call assert_equal([0, 2, 4], pyeval('l'))
  py l = ll[4:2:1]
  call assert_equal([], pyeval('l'))

  " Error case: Use an invalid index
  call AssertException(['py ll[-10] = 5'], 'Vim(python):vim.error: internal error:')

  " Use a step value of 0
  call AssertException(['py ll[0:3:0] = [1, 2, 3]'],
        \ 'Vim(python):ValueError: slice step cannot be zero')

  " Error case: Invalid slice type
  call AssertException(["py x = ll['abc']"],
        \ 'Vim(python):TypeError: index must be int or slice, not str')
  py del l

  " Error case: List with a null list item
  let l = [test_null_list()]
  py ll = vim.bindeval('l')
  call AssertException(["py x = ll[:]"],
        \ 'Vim(python):SystemError: error return without exception set')
endfunc

" Vars
func Test_python_vars()
  let g:foo = 'bac'
  let w:abc3 = 'def'
  let b:baz = 'bar'
  let t:bar = 'jkl'
  try
    throw "Abc"
  catch /Abc/
    call assert_equal('Abc', pyeval('vim.vvars[''exception'']'))
  endtry
  call assert_equal('bac', pyeval('vim.vars[''foo'']'))
  call assert_equal('def', pyeval('vim.current.window.vars[''abc3'']'))
  call assert_equal('bar', pyeval('vim.current.buffer.vars[''baz'']'))
  call assert_equal('jkl', pyeval('vim.current.tabpage.vars[''bar'']'))
endfunc

" Options
" paste:          boolean, global
" previewheight   number,  global
" operatorfunc:   string,  global
" number:         boolean, window-local
" numberwidth:    number,  window-local
" colorcolumn:    string,  window-local
" statusline:     string,  window-local/global
" autoindent:     boolean, buffer-local
" shiftwidth:     number,  buffer-local
" omnifunc:       string,  buffer-local
" preserveindent: boolean, buffer-local/global
" path:           string,  buffer-local/global
func Test_python_opts()
  let g:res = []
  let g:bufs = [bufnr('%')]
  new
  let g:bufs += [bufnr('%')]
  vnew
  let g:bufs += [bufnr('%')]
  wincmd j
  vnew
  let g:bufs += [bufnr('%')]
  wincmd l

  func RecVars(opt)
    let gval = string(eval('&g:' .. a:opt))
    let wvals = join(map(range(1, 4),
          \ 'v:val .. ":" .. string(getwinvar(v:val, "&" .. a:opt))'))
    let bvals = join(map(copy(g:bufs),
          \ 'v:val .. ":" .. string(getbufvar(v:val, "&" .. a:opt))'))
    call add(g:res, '  G: ' .. gval)
    call add(g:res, '  W: ' .. wvals)
    call add(g:res, '  B: ' .. wvals)
  endfunc

  py << trim EOF
    def e(s, g=globals(), l=locals()):
      try:
        exec(s, g, l)
      except:
        vim.command('return ' + repr(sys.exc_type.__name__))

    def ev(s, g=globals(), l=locals()):
      try:
        return eval(s, g, l)
      except:
        vim.command('let exc=' + repr(sys.exc_type.__name__))
        return 0
  EOF

  func E(s)
    python e(vim.eval('a:s'))
  endfunc

  func Ev(s)
    let r = pyeval('ev(vim.eval("a:s"))')
    if exists('exc')
      throw exc
    endif
    return r
  endfunc

  py gopts1 = vim.options
  py wopts1 = vim.windows[2].options
  py wopts2 = vim.windows[0].options
  py wopts3 = vim.windows[1].options
  py bopts1 = vim.buffers[vim.bindeval("g:bufs")[2]].options
  py bopts2 = vim.buffers[vim.bindeval("g:bufs")[1]].options
  py bopts3 = vim.buffers[vim.bindeval("g:bufs")[0]].options
  call add(g:res, 'wopts iters equal: ' ..
        \ pyeval('list(wopts1) == list(wopts2)'))
  call add(g:res, 'bopts iters equal: ' ..
        \ pyeval('list(bopts1) == list(bopts2)'))
  py gset = set(iter(gopts1))
  py wset = set(iter(wopts1))
  py bset = set(iter(bopts1))

  set path=.,..,,
  let lst = []
  let lst += [['paste', 1, 0, 1, 2, 1, 1, 0]]
  let lst += [['previewheight', 5, 1, 6, 'a', 0, 1, 0]]
  let lst += [['operatorfunc', 'A', 'B', 'C', 2, 0, 1, 0]]
  let lst += [['number', 0, 1, 1, 0, 1, 0, 1]]
  let lst += [['numberwidth', 2, 3, 5, -100, 0, 0, 1]]
  let lst += [['colorcolumn', '+1', '+2', '+3', 'abc4', 0, 0, 1]]
  let lst += [['statusline', '1', '2', '4', 0, 0, 1, 1]]
  let lst += [['autoindent', 0, 1, 1, 2, 1, 0, 2]]
  let lst += [['shiftwidth', 0, 2, 1, 3, 0, 0, 2]]
  let lst += [['omnifunc', 'A', 'B', 'C', 1, 0, 0, 2]]
  let lst += [['preserveindent', 0, 1, 1, 2, 1, 1, 2]]
  let lst += [['path', '.,,', ',,', '.', 0, 0, 1, 2]]
  for  [oname, oval1, oval2, oval3, invval, bool, global, local] in lst
    py oname = vim.eval('oname')
    py oval1 = vim.bindeval('oval1')
    py oval2 = vim.bindeval('oval2')
    py oval3 = vim.bindeval('oval3')
    if invval is 0 || invval is 1
      py invval = bool(vim.bindeval('invval'))
    else
      py invval = vim.bindeval('invval')
    endif
    if bool
      py oval1 = bool(oval1)
      py oval2 = bool(oval2)
      py oval3 = bool(oval3)
    endif
    call add(g:res, '>>> ' .. oname)
    call add(g:res, '  g/w/b:' .. pyeval('oname in gset') .. '/' ..
          \ pyeval('oname in wset') .. '/' .. pyeval('oname in bset'))
    call add(g:res, '  g/w/b (in):' .. pyeval('oname in gopts1') .. '/' ..
          \ pyeval('oname in wopts1') .. '/' .. pyeval('oname in bopts1'))
    for v in ['gopts1', 'wopts1', 'bopts1']
      try
        call add(g:res, '  p/' .. v .. ': ' .. Ev('repr(' .. v .. '[''' .. oname .. '''])'))
      catch
        call add(g:res, '  p/' .. v .. '! ' .. v:exception)
      endtry
      let r = E(v .. '[''' .. oname .. ''']=invval')
      if r isnot 0
        call add(g:res, '  inv: ' .. string(invval) .. '! ' .. r)
      endif
      for vv in (v is# 'gopts1' ? [v] : [v, v[:-2] .. '2', v[:-2] .. '3'])
        let val = substitute(vv, '^.opts', 'oval', '')
        let r = E(vv .. '[''' .. oname .. ''']=' .. val)
        if r isnot 0
            call add(g:res, '  ' .. vv .. '! ' .. r)
        endif
      endfor
    endfor
    call RecVars(oname)
    for v in ['wopts3', 'bopts3']
      let r = E('del ' .. v .. '["' .. oname .. '"]')
      if r isnot 0
        call add(g:res, '  del ' .. v .. '! ' .. r)
      endif
    endfor
    call RecVars(oname)
  endfor
  delfunction RecVars
  delfunction E
  delfunction Ev
  py del ev
  py del e
  only
  for buf in g:bufs[1:]
    execute 'bwipeout!' buf
  endfor
  py del gopts1
  py del wopts1
  py del wopts2
  py del wopts3
  py del bopts1
  py del bopts2
  py del bopts3
  py del oval1
  py del oval2
  py del oval3
  py del oname
  py del invval

  let expected =<< trim END
    wopts iters equal: 1
    bopts iters equal: 1
    >>> paste
      g/w/b:1/0/0
      g/w/b (in):1/0/0
      p/gopts1: False
      p/wopts1! KeyError
      inv: 2! KeyError
      wopts1! KeyError
      wopts2! KeyError
      wopts3! KeyError
      p/bopts1! KeyError
      inv: 2! KeyError
      bopts1! KeyError
      bopts2! KeyError
      bopts3! KeyError
      G: 1
      W: 1:1 2:1 3:1 4:1
      B: 1:1 2:1 3:1 4:1
      del wopts3! KeyError
      del bopts3! KeyError
      G: 1
      W: 1:1 2:1 3:1 4:1
      B: 1:1 2:1 3:1 4:1
    >>> previewheight
      g/w/b:1/0/0
      g/w/b (in):1/0/0
      p/gopts1: 12
      inv: 'a'! TypeError
      p/wopts1! KeyError
      inv: 'a'! KeyError
      wopts1! KeyError
      wopts2! KeyError
      wopts3! KeyError
      p/bopts1! KeyError
      inv: 'a'! KeyError
      bopts1! KeyError
      bopts2! KeyError
      bopts3! KeyError
      G: 5
      W: 1:5 2:5 3:5 4:5
      B: 1:5 2:5 3:5 4:5
      del wopts3! KeyError
      del bopts3! KeyError
      G: 5
      W: 1:5 2:5 3:5 4:5
      B: 1:5 2:5 3:5 4:5
    >>> operatorfunc
      g/w/b:1/0/0
      g/w/b (in):1/0/0
      p/gopts1: ''
      inv: 2! TypeError
      p/wopts1! KeyError
      inv: 2! KeyError
      wopts1! KeyError
      wopts2! KeyError
      wopts3! KeyError
      p/bopts1! KeyError
      inv: 2! KeyError
      bopts1! KeyError
      bopts2! KeyError
      bopts3! KeyError
      G: 'A'
      W: 1:'A' 2:'A' 3:'A' 4:'A'
      B: 1:'A' 2:'A' 3:'A' 4:'A'
      del wopts3! KeyError
      del bopts3! KeyError
      G: 'A'
      W: 1:'A' 2:'A' 3:'A' 4:'A'
      B: 1:'A' 2:'A' 3:'A' 4:'A'
    >>> number
      g/w/b:0/1/0
      g/w/b (in):0/1/0
      p/gopts1! KeyError
      inv: 0! KeyError
      gopts1! KeyError
      p/wopts1: False
      p/bopts1! KeyError
      inv: 0! KeyError
      bopts1! KeyError
      bopts2! KeyError
      bopts3! KeyError
      G: 0
      W: 1:1 2:1 3:0 4:0
      B: 1:1 2:1 3:0 4:0
      del wopts3! ValueError
      del bopts3! KeyError
      G: 0
      W: 1:1 2:1 3:0 4:0
      B: 1:1 2:1 3:0 4:0
    >>> numberwidth
      g/w/b:0/1/0
      g/w/b (in):0/1/0
      p/gopts1! KeyError
      inv: -100! KeyError
      gopts1! KeyError
      p/wopts1: 4
      inv: -100! error
      p/bopts1! KeyError
      inv: -100! KeyError
      bopts1! KeyError
      bopts2! KeyError
      bopts3! KeyError
      G: 4
      W: 1:3 2:5 3:2 4:4
      B: 1:3 2:5 3:2 4:4
      del wopts3! ValueError
      del bopts3! KeyError
      G: 4
      W: 1:3 2:5 3:2 4:4
      B: 1:3 2:5 3:2 4:4
    >>> colorcolumn
      g/w/b:0/1/0
      g/w/b (in):0/1/0
      p/gopts1! KeyError
      inv: 'abc4'! KeyError
      gopts1! KeyError
      p/wopts1: ''
      inv: 'abc4'! error
      p/bopts1! KeyError
      inv: 'abc4'! KeyError
      bopts1! KeyError
      bopts2! KeyError
      bopts3! KeyError
      G: ''
      W: 1:'+2' 2:'+3' 3:'+1' 4:''
      B: 1:'+2' 2:'+3' 3:'+1' 4:''
      del wopts3! ValueError
      del bopts3! KeyError
      G: ''
      W: 1:'+2' 2:'+3' 3:'+1' 4:''
      B: 1:'+2' 2:'+3' 3:'+1' 4:''
    >>> statusline
      g/w/b:1/1/0
      g/w/b (in):1/1/0
      p/gopts1: ''
      inv: 0! TypeError
      p/wopts1: None
      inv: 0! TypeError
      p/bopts1! KeyError
      inv: 0! KeyError
      bopts1! KeyError
      bopts2! KeyError
      bopts3! KeyError
      G: '1'
      W: 1:'2' 2:'4' 3:'1' 4:'1'
      B: 1:'2' 2:'4' 3:'1' 4:'1'
      del bopts3! KeyError
      G: '1'
      W: 1:'2' 2:'1' 3:'1' 4:'1'
      B: 1:'2' 2:'1' 3:'1' 4:'1'
    >>> autoindent
      g/w/b:0/0/1
      g/w/b (in):0/0/1
      p/gopts1! KeyError
      inv: 2! KeyError
      gopts1! KeyError
      p/wopts1! KeyError
      inv: 2! KeyError
      wopts1! KeyError
      wopts2! KeyError
      wopts3! KeyError
      p/bopts1: False
      G: 0
      W: 1:0 2:1 3:0 4:1
      B: 1:0 2:1 3:0 4:1
      del wopts3! KeyError
      del bopts3! ValueError
      G: 0
      W: 1:0 2:1 3:0 4:1
      B: 1:0 2:1 3:0 4:1
    >>> shiftwidth
      g/w/b:0/0/1
      g/w/b (in):0/0/1
      p/gopts1! KeyError
      inv: 3! KeyError
      gopts1! KeyError
      p/wopts1! KeyError
      inv: 3! KeyError
      wopts1! KeyError
      wopts2! KeyError
      wopts3! KeyError
      p/bopts1: 8
      G: 8
      W: 1:0 2:2 3:8 4:1
      B: 1:0 2:2 3:8 4:1
      del wopts3! KeyError
      del bopts3! ValueError
      G: 8
      W: 1:0 2:2 3:8 4:1
      B: 1:0 2:2 3:8 4:1
    >>> omnifunc
      g/w/b:0/0/1
      g/w/b (in):0/0/1
      p/gopts1! KeyError
      inv: 1! KeyError
      gopts1! KeyError
      p/wopts1! KeyError
      inv: 1! KeyError
      wopts1! KeyError
      wopts2! KeyError
      wopts3! KeyError
      p/bopts1: ''
      inv: 1! TypeError
      G: ''
      W: 1:'A' 2:'B' 3:'' 4:'C'
      B: 1:'A' 2:'B' 3:'' 4:'C'
      del wopts3! KeyError
      del bopts3! ValueError
      G: ''
      W: 1:'A' 2:'B' 3:'' 4:'C'
      B: 1:'A' 2:'B' 3:'' 4:'C'
    >>> preserveindent
      g/w/b:0/0/1
      g/w/b (in):0/0/1
      p/gopts1! KeyError
      inv: 2! KeyError
      gopts1! KeyError
      p/wopts1! KeyError
      inv: 2! KeyError
      wopts1! KeyError
      wopts2! KeyError
      wopts3! KeyError
      p/bopts1: False
      G: 0
      W: 1:0 2:1 3:0 4:1
      B: 1:0 2:1 3:0 4:1
      del wopts3! KeyError
      del bopts3! ValueError
      G: 0
      W: 1:0 2:1 3:0 4:1
      B: 1:0 2:1 3:0 4:1
    >>> path
      g/w/b:1/0/1
      g/w/b (in):1/0/1
      p/gopts1: '.,..,,'
      inv: 0! TypeError
      p/wopts1! KeyError
      inv: 0! KeyError
      wopts1! KeyError
      wopts2! KeyError
      wopts3! KeyError
      p/bopts1: None
      inv: 0! TypeError
      G: '.,,'
      W: 1:'.,,' 2:',,' 3:'.,,' 4:'.'
      B: 1:'.,,' 2:',,' 3:'.,,' 4:'.'
      del wopts3! KeyError
      G: '.,,'
      W: 1:'.,,' 2:',,' 3:'.,,' 4:'.,,'
      B: 1:'.,,' 2:',,' 3:'.,,' 4:'.,,'
  END

  call assert_equal(expected, g:res)
  unlet g:res

  call assert_equal(0, pyeval("'' in vim.options"))

  " use an empty key to index vim.options
  call AssertException(["let v = pyeval(\"vim.options['']\")"],
        \ 'Vim(let):ValueError: empty keys are not allowed')
  call AssertException(["py vim.current.window.options[''] = 0"],
        \ 'Vim(python):ValueError: empty keys are not allowed')
  call AssertException(["py vim.current.window.options[{}] = 0"],
        \ 'Vim(python):TypeError: expected str() or unicode() instance, but got dict')

  " set one of the number options to a very large number
  let cmd = ["py vim.options['previewheight'] = 9999999999999999"]
  call AssertException(cmd, 'OverflowError:')

  " unset a global-local string option
  call AssertException(["py del vim.options['errorformat']"],
        \ 'Vim(python):ValueError: unable to unset global option errorformat')
endfunc

" Test for vim.buffer object
func Test_python_buffer()
  new
  call setline(1, "Hello\nWorld")
  call assert_fails("let x = pyeval('vim.current.buffer[0]')", 'E859:')
  %bw!

  edit Xfile1
  let bnr1 = bufnr()
  py cb = vim.current.buffer
  vnew Xfile2
  let bnr2 = bufnr()
  call setline(1, ['First line', 'Second line', 'Third line'])
  py b = vim.current.buffer
  wincmd w

  " Test for getting lines from the buffer using a slice
  call assert_equal(['First line'], pyeval('b[-10:1]'))
  call assert_equal(['Third line'], pyeval('b[2:10]'))
  call assert_equal([], pyeval('b[2:0]'))
  call assert_equal([], pyeval('b[10:12]'))
  call assert_equal([], pyeval('b[-10:-8]'))
  call AssertException(["py x = b[0:3:0]"],
        \ "Vim(python):TypeError: sequence index must be integer, not 'slice'")
  call AssertException(["py b[0:3:0] = 'abc'"],
        \ "Vim(python):TypeError: sequence index must be integer, not 'slice'")
  call AssertException(["py x = b[{}]"],
        \ "Vim(python):TypeError: sequence index must be integer, not 'dict'")
  call AssertException(["py b[{}] = 'abc'"],
        \ "Vim(python):TypeError: sequence index must be integer, not 'dict'")

  " Test for getting lines using a range
  call AssertException(["py x = b.range(0,3)[0:2:0]"],
        \ "Vim(python):TypeError: sequence index must be integer, not 'slice'")
  call AssertException(["py b.range(0,3)[0:2:0] = 'abc'"],
        \ "Vim(python):TypeError: sequence index must be integer, not 'slice'")

  " Tests BufferAppend and BufferItem
  py cb.append(b[0])
  call assert_equal(['First line'], getbufline(bnr1, 2))
  %d

  " Try to append using out-of-range line number
  call AssertException(["py b.append('abc', 10)"],
        \ 'Vim(python):IndexError: line number out of range')

  " Append a non-string item
  call AssertException(["py b.append([22])"],
        \ 'Vim(python):TypeError: expected str() or unicode() instance, but got int')

  " Tests BufferSlice and BufferAssSlice
  py cb.append('abc5') # Will be overwritten
  py cb[-1:] = b[:-2]
  call assert_equal(['First line'], getbufline(bnr1, 2))
  %d

  " Test BufferLength and BufferAssSlice
  py cb.append('def') # Will not be overwritten
  py cb[len(cb):] = b[:]
  call assert_equal(['def', 'First line', 'Second line', 'Third line'],
        \ getbufline(bnr1, 2, '$'))
  %d

  " Test BufferAssItem and BufferMark
  call setbufline(bnr1, 1, ['one', 'two', 'three'])
  call cursor(1, 3)
  normal ma
  py cb.append('ghi') # Will be overwritten
  py cb[-1] = repr((len(cb) - cb.mark('a')[0], cb.mark('a')[1]))
  call assert_equal(['(3, 2)'], getbufline(bnr1, 4))
  %d

  " Test BufferRepr
  py cb.append(repr(cb) + repr(b))
  call assert_equal(['<buffer Xfile1><buffer Xfile2>'], getbufline(bnr1, 2))
  %d

  " Modify foreign buffer
  py << trim EOF
    b.append('foo')
    b[0]='bar'
    b[0:0]=['baz']
    vim.command('call append("$", getbufline(%i, 1, "$"))' % b.number)
  EOF
  call assert_equal(['baz', 'bar', 'Second line', 'Third line', 'foo'],
        \ getbufline(bnr2, 1, '$'))
  %d

  " Test assigning to name property
  augroup BUFS
    autocmd BufFilePost * python cb.append(vim.eval('expand("<abuf>")') + ':BufFilePost:' + vim.eval('bufnr("%")'))
    autocmd BufFilePre * python cb.append(vim.eval('expand("<abuf>")') + ':BufFilePre:' + vim.eval('bufnr("%")'))
  augroup END
  py << trim EOF
    import os
    old_name = cb.name
    cb.name = 'foo'
    cb.append(cb.name[-11:].replace(os.path.sep, '/'))
    b.name = 'bar'
    cb.append(b.name[-11:].replace(os.path.sep, '/'))
    cb.name = old_name
    cb.append(cb.name[-14:].replace(os.path.sep, '/'))
    del old_name
  EOF
  call assert_equal([bnr1 .. ':BufFilePre:' .. bnr1,
        \ bnr1 .. ':BufFilePost:' .. bnr1,
        \ 'testdir/foo',
        \ bnr2 .. ':BufFilePre:' .. bnr2,
        \ bnr2 .. ':BufFilePost:' .. bnr2,
        \ 'testdir/bar',
        \ bnr1 .. ':BufFilePre:' .. bnr1,
        \ bnr1 .. ':BufFilePost:' .. bnr1,
        \ 'testdir/Xfile1'], getbufline(bnr1, 2, '$'))
  %d

  " Test CheckBuffer
  py << trim EOF
    for _b in vim.buffers:
      if _b is not cb:
        vim.command('bwipeout! ' + str(_b.number))
    del _b
    cb.append('valid: b:%s, cb:%s' % (repr(b.valid), repr(cb.valid)))
  EOF
  call assert_equal('valid: b:False, cb:True', getline(2))
  %d

  py << trim EOF
    for expr in ('b[1]','b[:] = ["A", "B"]','b[:]','b.append("abc6")', 'b.name = "!"'):
      try:
        exec(expr)
      except vim.error:
        pass
      else:
        # Usually a SEGV here
        # Should not happen in any case
        cb.append('No exception for ' + expr)
    vim.command('cd .')
    del b
  EOF
  call assert_equal([''], getline(1, '$'))

  " Delete all the lines in a buffer
  call setline(1, ['a', 'b', 'c'])
  py vim.current.buffer[:] = []
  call assert_equal([''], getline(1, '$'))

  " Test for buffer marks
  call assert_equal(v:none, pyeval("vim.current.buffer.mark('r')"))

  " Test for modifying a 'nomodifiable' buffer
  setlocal nomodifiable
  call AssertException(["py vim.current.buffer[0] = 'abc'"],
        \ "Vim(python):vim.error: Vim:E21: Cannot make changes, 'modifiable' is off")
  call AssertException(["py vim.current.buffer[0] = None"],
        \ "Vim(python):vim.error: Vim:E21: Cannot make changes, 'modifiable' is off")
  call AssertException(["py vim.current.buffer[:] = None"],
        \ "Vim(python):vim.error: Vim:E21: Cannot make changes, 'modifiable' is off")
  call AssertException(["py vim.current.buffer[:] = []"],
        \ "Vim(python):vim.error: Vim:E21: Cannot make changes, 'modifiable' is off")
  call AssertException(["py vim.current.buffer.append('abc')"],
        \ "Vim(python):vim.error: Vim:E21: Cannot make changes, 'modifiable' is off")
  call AssertException(["py vim.current.buffer.append([])"],
        \ "Vim(python):vim.error: Vim:E21: Cannot make changes, 'modifiable' is off")
  setlocal modifiable

  augroup BUFS
    autocmd!
  augroup END
  augroup! BUFS
  %bw!

  " Range object for a deleted buffer
  new Xpbuffile
  call setline(1, ['one', 'two', 'three'])
  py b = vim.current.buffer
  py r = vim.current.buffer.range(0, 2)
  call assert_equal('<range Xpbuffile (0:2)>', pyeval('repr(r)'))
  %bw!
  call AssertException(['py r[:] = []'],
        \ 'Vim(python):vim.error: attempt to refer to deleted buffer')
  call assert_match('<buffer object (deleted)', pyeval('repr(b)'))
  call assert_match('<range object (for deleted buffer)', pyeval('repr(r)'))
  call AssertException(["let n = pyeval('len(r)')"],
        \ 'Vim(let):vim.error: attempt to refer to deleted buffer')
  call AssertException(["py r.append('abc')"],
        \ 'Vim(python):vim.error: attempt to refer to deleted buffer')

  " object for a deleted buffer
  call AssertException(["py b[0] = 'one'"],
        \ 'Vim(python):vim.error: attempt to refer to deleted buffer')
  call AssertException(["py b.append('one')"],
        \ 'Vim(python):vim.error: attempt to refer to deleted buffer')
  call AssertException(["let n = pyeval('len(b)')"],
        \ 'Vim(let):vim.error: attempt to refer to deleted buffer')
  call AssertException(["py pos = b.mark('a')"],
        \ 'Vim(python):vim.error: attempt to refer to deleted buffer')
  call AssertException(["py vim.current.buffer = b"],
        \ 'Vim(python):vim.error: attempt to refer to deleted buffer')
  call AssertException(["py rn = b.range(0, 2)"],
        \ 'Vim(python):vim.error: attempt to refer to deleted buffer')
endfunc

" Test vim.buffers object
func Test_python_buffers()
  %bw!
  edit Xpbuffile
  py cb = vim.current.buffer
  set hidden
  edit a
  buffer #
  edit b
  buffer #
  edit c
  buffer #
  py << trim EOF
    try:
      from __builtin__ import next
    except ImportError:
      next = lambda o: o.next()
    # Check GCing iterator that was not fully exhausted
    i = iter(vim.buffers)
    cb.append('i:' + str(next(i)))
    # and also check creating more than one iterator at a time
    i2 = iter(vim.buffers)
    cb.append('i2:' + str(next(i2)))
    cb.append('i:' + str(next(i)))
    # The following should trigger GC and not cause any problems
    del i
    del i2
    i3 = iter(vim.buffers)
    cb.append('i3:' + str(next(i3)))
    del i3
  EOF
  call assert_equal(['i:<buffer Xpbuffile>',
        \ 'i2:<buffer Xpbuffile>', 'i:<buffer a>', 'i3:<buffer Xpbuffile>'],
        \ getline(2, '$'))
  %d

  py << trim EOF
    prevnum = 0
    for b in vim.buffers:
      # Check buffer order
      if prevnum >= b.number:
        cb.append('!!! Buffer numbers not in strictly ascending order')
      # Check indexing: vim.buffers[number].number == number
      cb.append(str(b.number) + ':' + repr(vim.buffers[b.number]) + \
                                                            '=' + repr(b))
      prevnum = b.number
    del prevnum

    cb.append(str(len(vim.buffers)))
  EOF
  call assert_equal([bufnr('Xpbuffile') .. ':<buffer Xpbuffile>=<buffer Xpbuffile>',
        \ bufnr('a') .. ':<buffer a>=<buffer a>',
        \ bufnr('b') .. ':<buffer b>=<buffer b>',
        \ bufnr('c') .. ':<buffer c>=<buffer c>', '4'], getline(2, '$'))
  %d

  py << trim EOF
    bnums = list(map(lambda b: b.number, vim.buffers))[1:]

    # Test wiping out buffer with existing iterator
    i4 = iter(vim.buffers)
    cb.append('i4:' + str(next(i4)))
    vim.command('bwipeout! ' + str(bnums.pop(0)))
    try:
      next(i4)
    except vim.error:
      pass
    else:
      cb.append('!!!! No vim.error')
    i4 = iter(vim.buffers)
    vim.command('bwipeout! ' + str(bnums.pop(-1)))
    vim.command('bwipeout! ' + str(bnums.pop(-1)))
    cb.append('i4:' + str(next(i4)))
    try:
      next(i4)
    except StopIteration:
      cb.append('StopIteration')
    del i4
    del bnums
  EOF
  call assert_equal(['i4:<buffer Xpbuffile>',
        \ 'i4:<buffer Xpbuffile>', 'StopIteration'], getline(2, '$'))
  %bw!
endfunc

" Test vim.{tabpage,window}list and vim.{tabpage,window} objects
func Test_python_tabpage_window()
  %bw
  edit Xpbuffile
  py cb = vim.current.buffer
  tabnew 0
  tabnew 1
  vnew a.1
  tabnew 2
  vnew a.2
  vnew b.2
  vnew c.2

  call assert_equal(4, pyeval('vim.current.window.tabpage.number'))

  py << trim EOF
    cb.append('Number of tabs: ' + str(len(vim.tabpages)))
    cb.append('Current tab pages:')
    def W(w):
      if repr(w).find('(unknown)') != -1:
        return '<window object (unknown)>'
      else:
        return repr(w)

    start = len(cb)

    def Cursor(w):
      if w.buffer is cb:
        return repr((start - w.cursor[0], w.cursor[1]))
      else:
        return repr(w.cursor)

    for t in vim.tabpages:
      cb.append('  ' + repr(t) + '(' + str(t.number) + ')' + ': ' + \
                str(len(t.windows)) + ' windows, current is ' + W(t.window))
      cb.append('  Windows:')
      for w in t.windows:
        cb.append('    ' + W(w) + '(' + str(w.number) + ')' + \
                                  ': displays buffer ' + repr(w.buffer) + \
                                  '; cursor is at ' + Cursor(w))
        # Other values depend on the size of the terminal, so they are checked
        # partly:
        for attr in ('height', 'row', 'width', 'col'):
          try:
            aval = getattr(w, attr)
            if type(aval) is not long:
              raise TypeError
            if aval < 0:
              raise ValueError
          except Exception:
            cb.append('!!!!!! Error while getting attribute ' + attr + \
                                            ': ' + sys.exc_type.__name__)
        del aval
        del attr
        w.cursor = (len(w.buffer), 0)
    del W
    del Cursor
    cb.append('Number of windows in current tab page: ' + \
                                                    str(len(vim.windows)))
    if list(vim.windows) != list(vim.current.tabpage.windows):
      cb.append('!!!!!! Windows differ')
  EOF

  let expected =<< trim END
    Number of tabs: 4
    Current tab pages:
      <tabpage 0>(1): 1 windows, current is <window object (unknown)>
      Windows:
        <window object (unknown)>(1): displays buffer <buffer Xpbuffile>; cursor is at (2, 0)
      <tabpage 1>(2): 1 windows, current is <window object (unknown)>
      Windows:
        <window object (unknown)>(1): displays buffer <buffer 0>; cursor is at (1, 0)
      <tabpage 2>(3): 2 windows, current is <window object (unknown)>
      Windows:
        <window object (unknown)>(1): displays buffer <buffer a.1>; cursor is at (1, 0)
        <window object (unknown)>(2): displays buffer <buffer 1>; cursor is at (1, 0)
      <tabpage 3>(4): 4 windows, current is <window 0>
      Windows:
        <window 0>(1): displays buffer <buffer c.2>; cursor is at (1, 0)
        <window 1>(2): displays buffer <buffer b.2>; cursor is at (1, 0)
        <window 2>(3): displays buffer <buffer a.2>; cursor is at (1, 0)
        <window 3>(4): displays buffer <buffer 2>; cursor is at (1, 0)
    Number of windows in current tab page: 4
  END
  call assert_equal(expected, getbufline(bufnr('Xpbuffile'), 2, '$'))
  %bw!
endfunc

" Test vim.current
func Test_python_vim_current()
  %bw
  edit Xpbuffile
  py cb = vim.current.buffer
  tabnew 0
  tabnew 1
  vnew a.1
  tabnew 2
  vnew a.2
  vnew b.2
  vnew c.2

  py << trim EOF
    def H(o):
      return repr(o)
    cb.append('Current tab page: ' + repr(vim.current.tabpage))
    cb.append('Current window: ' + repr(vim.current.window) + ': ' + \
               H(vim.current.window) + ' is ' + H(vim.current.tabpage.window))
    cb.append('Current buffer: ' + repr(vim.current.buffer) + ': ' + \
               H(vim.current.buffer) + ' is ' + H(vim.current.window.buffer)+ \
               ' is ' + H(vim.current.tabpage.window.buffer))
    del H
  EOF
  let expected =<< trim END
    Current tab page: <tabpage 3>
    Current window: <window 0>: <window 0> is <window 0>
    Current buffer: <buffer c.2>: <buffer c.2> is <buffer c.2> is <buffer c.2>
  END
  call assert_equal(expected, getbufline(bufnr('Xpbuffile'), 2, '$'))
  call deletebufline(bufnr('Xpbuffile'), 1, '$')

  " Assigning: fails
  py << trim EOF
    try:
      vim.current.window = vim.tabpages[0].window
    except ValueError:
      cb.append('ValueError at assigning foreign tab window')

    for attr in ('window', 'tabpage', 'buffer'):
      try:
        setattr(vim.current, attr, None)
      except TypeError:
        cb.append('Type error at assigning None to vim.current.' + attr)
    del attr
  EOF

  let expected =<< trim END
    ValueError at assigning foreign tab window
    Type error at assigning None to vim.current.window
    Type error at assigning None to vim.current.tabpage
    Type error at assigning None to vim.current.buffer
  END
  call assert_equal(expected, getbufline(bufnr('Xpbuffile'), 2, '$'))
  call deletebufline(bufnr('Xpbuffile'), 1, '$')

  call setbufline(bufnr('Xpbuffile'), 1, 'python interface')
  py << trim EOF
    # Assigning: success
    vim.current.tabpage = vim.tabpages[-2]
    vim.current.buffer = cb
    vim.current.window = vim.windows[0]
    vim.current.window.cursor = (len(vim.current.buffer), 0)
    cb.append('Current tab page: ' + repr(vim.current.tabpage))
    cb.append('Current window: ' + repr(vim.current.window))
    cb.append('Current buffer: ' + repr(vim.current.buffer))
    cb.append('Current line: ' + repr(vim.current.line))
  EOF

  let expected =<< trim END
    Current tab page: <tabpage 2>
    Current window: <window 0>
    Current buffer: <buffer Xpbuffile>
    Current line: 'python interface'
  END
  call assert_equal(expected, getbufline(bufnr('Xpbuffile'), 2, '$'))
  py vim.current.line = 'one line'
  call assert_equal('one line', getline('.'))
  call deletebufline(bufnr('Xpbuffile'), 1, '$')

  py << trim EOF
    ws = list(vim.windows)
    ts = list(vim.tabpages)
    for b in vim.buffers:
      if b is not cb:
        vim.command('bwipeout! ' + str(b.number))
    del b
    cb.append('w.valid: ' + repr([w.valid for w in ws]))
    cb.append('t.valid: ' + repr([t.valid for t in ts]))
    del w
    del t
    del ts
    del ws
  EOF
  let expected =<< trim END
    w.valid: [True, False]
    t.valid: [True, False, True, False]
  END
  call assert_equal(expected, getbufline(bufnr('Xpbuffile'), 2, '$'))
  %bw!
endfunc

" Test types
func Test_python_types()
  %d
  py cb = vim.current.buffer
  py << trim EOF
    for expr, attr in (
      ('vim.vars',                         'Dictionary'),
      ('vim.options',                      'Options'),
      ('vim.bindeval("{}")',               'Dictionary'),
      ('vim.bindeval("[]")',               'List'),
      ('vim.bindeval("function(\'tr\')")', 'Function'),
      ('vim.current.buffer',               'Buffer'),
      ('vim.current.range',                'Range'),
      ('vim.current.window',               'Window'),
      ('vim.current.tabpage',              'TabPage'),
    ):
      cb.append(expr + ':' + attr + ':' + \
                                repr(type(eval(expr)) is getattr(vim, attr)))
    del expr
    del attr
  EOF
  let expected =<< trim END
    vim.vars:Dictionary:True
    vim.options:Options:True
    vim.bindeval("{}"):Dictionary:True
    vim.bindeval("[]"):List:True
    vim.bindeval("function('tr')"):Function:True
    vim.current.buffer:Buffer:True
    vim.current.range:Range:True
    vim.current.window:Window:True
    vim.current.tabpage:TabPage:True
  END
  call assert_equal(expected, getline(2, '$'))
endfunc

" Test __dir__() method
func Test_python_dir_method()
  %d
  py cb = vim.current.buffer
  py << trim EOF
    for name, o in (
            ('current',    vim.current),
            ('buffer',     vim.current.buffer),
            ('window',     vim.current.window),
            ('tabpage',    vim.current.tabpage),
            ('range',      vim.current.range),
            ('dictionary', vim.bindeval('{}')),
            ('list',       vim.bindeval('[]')),
            ('function',   vim.bindeval('function("tr")')),
            ('output',     sys.stdout),
        ):
        cb.append(name + ':' + ','.join(dir(o)))
    del name
    del o
  EOF
  let expected =<< trim END
    current:__dir__,__members__,buffer,line,range,tabpage,window
    buffer:__dir__,__members__,append,mark,name,number,options,range,valid,vars
    window:__dir__,__members__,buffer,col,cursor,height,number,options,row,tabpage,valid,vars,width
    tabpage:__dir__,__members__,number,valid,vars,window,windows
    range:__dir__,__members__,append,end,start
    dictionary:__dir__,__members__,get,has_key,items,keys,locked,pop,popitem,scope,update,values
    list:__dir__,__members__,extend,locked
    function:__dir__,__members__,args,auto_rebind,self,softspace
    output:__dir__,__members__,close,closed,flush,isatty,readable,seekable,softspace,writable,write,writelines
  END
  call assert_equal(expected, getline(2, '$'))
endfunc

" Test vim.*.__new__
func Test_python_new()
  call assert_equal({}, pyeval('vim.Dictionary({})'))
  call assert_equal({'a': 1}, pyeval('vim.Dictionary(a=1)'))
  call assert_equal({'a': 1}, pyeval('vim.Dictionary(((''a'', 1),))'))
  call assert_equal([], pyeval('vim.List()'))
  call assert_equal(['a', 'b', 'c', '7'], pyeval('vim.List(iter(''abc7''))'))
  call assert_equal(function('tr'), pyeval('vim.Function(''tr'')'))
  call assert_equal(function('tr', [123, 3, 4]),
        \ pyeval('vim.Function(''tr'', args=[123, 3, 4])'))
  call assert_equal(function('tr'), pyeval('vim.Function(''tr'', args=[])'))
  call assert_equal(function('tr', {}),
        \ pyeval('vim.Function(''tr'', self={})'))
  call assert_equal(function('tr', [123, 3, 4], {}),
        \ pyeval('vim.Function(''tr'', args=[123, 3, 4], self={})'))
  call assert_equal(function('tr'),
        \ pyeval('vim.Function(''tr'', auto_rebind=False)'))
  call assert_equal(function('tr', [123, 3, 4]),
        \ pyeval('vim.Function(''tr'', args=[123, 3, 4], auto_rebind=False)'))
  call assert_equal(function('tr'),
        \ pyeval('vim.Function(''tr'', args=[], auto_rebind=False)'))
  call assert_equal(function('tr', {}),
        \ pyeval('vim.Function(''tr'', self={}, auto_rebind=False)'))
  call assert_equal(function('tr', [123, 3, 4], {}),
        \ pyeval('vim.Function(''tr'', args=[123, 3, 4], self={}, auto_rebind=False)'))
endfunc

" Test vim.Function
func Test_python_vim_func()
  func Args(...)
    return a:000
  endfunc

  func SelfArgs(...) dict
    return [a:000, self]
  endfunc

  " The following four lines should not crash
  let Pt = function('tr', [[]], {'l': []})
  py Pt = vim.bindeval('Pt')
  unlet Pt
  py del Pt

  call assert_equal(3, pyeval('vim.strwidth("a\tb")'))

  %bw!
  py cb = vim.current.buffer
  py << trim EOF
    def ecall(out_prefix, func, *args, **kwargs):
        line = out_prefix + ': '
        try:
            ret = func(*args, **kwargs)
        except Exception:
            line += '!exception: ' + emsg(sys.exc_info())
        else:
            line += '!result: ' + vim.Function('string')(ret)
        cb.append(line)
    a = vim.Function('Args')
    pa1 = vim.Function('Args', args=['abcArgsPA1'])
    pa2 = vim.Function('Args', args=[])
    pa3 = vim.Function('Args', args=['abcArgsPA3'], self={'abcSelfPA3': 'abcSelfPA3Val'})
    pa4 = vim.Function('Args', self={'abcSelfPA4': 'abcSelfPA4Val'})
    cb.append('a: ' + repr(a))
    cb.append('pa1: ' + repr(pa1))
    cb.append('pa2: ' + repr(pa2))
    cb.append('pa3: ' + repr(pa3))
    cb.append('pa4: ' + repr(pa4))
    sa = vim.Function('SelfArgs')
    psa1 = vim.Function('SelfArgs', args=['abcArgsPSA1'])
    psa2 = vim.Function('SelfArgs', args=[])
    psa3 = vim.Function('SelfArgs', args=['abcArgsPSA3'], self={'abcSelfPSA3': 'abcSelfPSA3Val'})
    psa4 = vim.Function('SelfArgs', self={'abcSelfPSA4': 'abcSelfPSA4Val'})
    psa5 = vim.Function('SelfArgs', self={'abcSelfPSA5': 'abcSelfPSA5Val'}, auto_rebind=0)
    psa6 = vim.Function('SelfArgs', args=['abcArgsPSA6'], self={'abcSelfPSA6': 'abcSelfPSA6Val'}, auto_rebind=())
    psa7 = vim.Function('SelfArgs', args=['abcArgsPSA7'], auto_rebind=[])
    psa8 = vim.Function('SelfArgs', auto_rebind=False)
    psa9 = vim.Function('SelfArgs', self={'abcSelfPSA9': 'abcSelfPSA9Val'}, auto_rebind=True)
    psaA = vim.Function('SelfArgs', args=['abcArgsPSAA'], self={'abcSelfPSAA': 'abcSelfPSAAVal'}, auto_rebind=1)
    psaB = vim.Function('SelfArgs', args=['abcArgsPSAB'], auto_rebind={'abcARPSAB': 'abcARPSABVal'})
    psaC = vim.Function('SelfArgs', auto_rebind=['abcARPSAC'])
    cb.append('sa: ' + repr(sa))
    cb.append('psa1: ' + repr(psa1))
    cb.append('psa2: ' + repr(psa2))
    cb.append('psa3: ' + repr(psa3))
    cb.append('psa4: ' + repr(psa4))
    cb.append('psa5: ' + repr(psa5))
    cb.append('psa6: ' + repr(psa6))
    cb.append('psa7: ' + repr(psa7))
    cb.append('psa8: ' + repr(psa8))
    cb.append('psa9: ' + repr(psa9))
    cb.append('psaA: ' + repr(psaA))
    cb.append('psaB: ' + repr(psaB))
    cb.append('psaC: ' + repr(psaC))

    psar = vim.Function('SelfArgs', args=[{'abcArgsPSAr': 'abcArgsPSArVal'}], self={'abcSelfPSAr': 'abcSelfPSArVal'})
    psar.args[0]['abcArgsPSAr2'] = [psar.self, psar.args[0]]
    psar.self['rec'] = psar
    psar.self['self'] = psar.self
    psar.self['args'] = psar.args

    try:
        cb.append('psar: ' + repr(psar))
    except Exception:
        cb.append('!!!!!!!! Caught exception: ' + emsg(sys.exc_info()))
  EOF

  let expected =<< trim END
    a: <vim.Function 'Args'>
    pa1: <vim.Function 'Args', args=['abcArgsPA1']>
    pa2: <vim.Function 'Args'>
    pa3: <vim.Function 'Args', args=['abcArgsPA3'], self={'abcSelfPA3': 'abcSelfPA3Val'}>
    pa4: <vim.Function 'Args', self={'abcSelfPA4': 'abcSelfPA4Val'}>
    sa: <vim.Function 'SelfArgs'>
    psa1: <vim.Function 'SelfArgs', args=['abcArgsPSA1']>
    psa2: <vim.Function 'SelfArgs'>
    psa3: <vim.Function 'SelfArgs', args=['abcArgsPSA3'], self={'abcSelfPSA3': 'abcSelfPSA3Val'}>
    psa4: <vim.Function 'SelfArgs', self={'abcSelfPSA4': 'abcSelfPSA4Val'}>
    psa5: <vim.Function 'SelfArgs', self={'abcSelfPSA5': 'abcSelfPSA5Val'}>
    psa6: <vim.Function 'SelfArgs', args=['abcArgsPSA6'], self={'abcSelfPSA6': 'abcSelfPSA6Val'}>
    psa7: <vim.Function 'SelfArgs', args=['abcArgsPSA7']>
    psa8: <vim.Function 'SelfArgs'>
    psa9: <vim.Function 'SelfArgs', self={'abcSelfPSA9': 'abcSelfPSA9Val'}, auto_rebind=True>
    psaA: <vim.Function 'SelfArgs', args=['abcArgsPSAA'], self={'abcSelfPSAA': 'abcSelfPSAAVal'}, auto_rebind=True>
    psaB: <vim.Function 'SelfArgs', args=['abcArgsPSAB']>
    psaC: <vim.Function 'SelfArgs'>
    psar: <vim.Function 'SelfArgs', args=[{'abcArgsPSAr2': [{'rec': function('SelfArgs', [{...}], {...}), 'self': {...}, 'abcSelfPSAr': 'abcSelfPSArVal', 'args': [{...}]}, {...}], 'abcArgsPSAr': 'abcArgsPSArVal'}], self={'rec': function('SelfArgs', [{'abcArgsPSAr2': [{...}, {...}], 'abcArgsPSAr': 'abcArgsPSArVal'}], {...}), 'self': {...}, 'abcSelfPSAr': 'abcSelfPSArVal', 'args': [{'abcArgsPSAr2': [{...}, {...}], 'abcArgsPSAr': 'abcArgsPSArVal'}]}>
  END
  call assert_equal(expected, getline(2, '$'))
  %d

  call assert_equal(function('Args'), pyeval('a'))
  call assert_equal(function('Args', ['abcArgsPA1']), pyeval('pa1'))
  call assert_equal(function('Args'), pyeval('pa2'))
  call assert_equal(function('Args', ['abcArgsPA3'], {'abcSelfPA3': 'abcSelfPA3Val'}), pyeval('pa3'))
  call assert_equal(function('Args', {'abcSelfPA4': 'abcSelfPA4Val'}), pyeval('pa4'))
  call assert_equal(function('SelfArgs'), pyeval('sa'))
  call assert_equal(function('SelfArgs', ['abcArgsPSA1']), pyeval('psa1'))
  call assert_equal(function('SelfArgs'), pyeval('psa2'))
  call assert_equal(function('SelfArgs', ['abcArgsPSA3'], {'abcSelfPSA3': 'abcSelfPSA3Val'}), pyeval('psa3'))
  call assert_equal(function('SelfArgs', {'abcSelfPSA4': 'abcSelfPSA4Val'}), pyeval('psa4'))
  call assert_equal(function('SelfArgs', {'abcSelfPSA5': 'abcSelfPSA5Val'}), pyeval('psa5'))
  call assert_equal(function('SelfArgs', ['abcArgsPSA6'], {'abcSelfPSA6': 'abcSelfPSA6Val'}), pyeval('psa6'))
  call assert_equal(function('SelfArgs', ['abcArgsPSA7']), pyeval('psa7'))
  call assert_equal(function('SelfArgs'), pyeval('psa8'))
  call assert_equal(function('SelfArgs', {'abcSelfPSA9': 'abcSelfPSA9Val'}), pyeval('psa9'))
  call assert_equal(function('SelfArgs', ['abcArgsPSAA'], {'abcSelfPSAA': 'abcSelfPSAAVal'}), pyeval('psaA'))
  call assert_equal(function('SelfArgs', ['abcArgsPSAB']), pyeval('psaB'))
  call assert_equal(function('SelfArgs'), pyeval('psaC'))

  let res = []
  for v in ['sa', 'psa1', 'psa2', 'psa3', 'psa4', 'psa5', 'psa6', 'psa7',
        \ 'psa8', 'psa9', 'psaA', 'psaB', 'psaC']
    let d = {'f': pyeval(v)}
    call add(res, 'd.' .. v .. '(): ' .. string(d.f()))
  endfor

  let expected =<< trim END
    d.sa(): [[], {'f': function('SelfArgs')}]
    d.psa1(): [['abcArgsPSA1'], {'f': function('SelfArgs', ['abcArgsPSA1'])}]
    d.psa2(): [[], {'f': function('SelfArgs')}]
    d.psa3(): [['abcArgsPSA3'], {'abcSelfPSA3': 'abcSelfPSA3Val'}]
    d.psa4(): [[], {'abcSelfPSA4': 'abcSelfPSA4Val'}]
    d.psa5(): [[], {'abcSelfPSA5': 'abcSelfPSA5Val'}]
    d.psa6(): [['abcArgsPSA6'], {'abcSelfPSA6': 'abcSelfPSA6Val'}]
    d.psa7(): [['abcArgsPSA7'], {'f': function('SelfArgs', ['abcArgsPSA7'])}]
    d.psa8(): [[], {'f': function('SelfArgs')}]
    d.psa9(): [[], {'f': function('SelfArgs', {'abcSelfPSA9': 'abcSelfPSA9Val'})}]
    d.psaA(): [['abcArgsPSAA'], {'f': function('SelfArgs', ['abcArgsPSAA'], {'abcSelfPSAA': 'abcSelfPSAAVal'})}]
    d.psaB(): [['abcArgsPSAB'], {'f': function('SelfArgs', ['abcArgsPSAB'])}]
    d.psaC(): [[], {'f': function('SelfArgs')}]
  END
  call assert_equal(expected, res)

  py ecall('a()', a, )
  py ecall('pa1()', pa1, )
  py ecall('pa2()', pa2, )
  py ecall('pa3()', pa3, )
  py ecall('pa4()', pa4, )
  py ecall('sa()', sa, )
  py ecall('psa1()', psa1, )
  py ecall('psa2()', psa2, )
  py ecall('psa3()', psa3, )
  py ecall('psa4()', psa4, )

  py ecall('a(42, 43)', a, 42, 43)
  py ecall('pa1(42, 43)', pa1, 42, 43)
  py ecall('pa2(42, 43)', pa2, 42, 43)
  py ecall('pa3(42, 43)', pa3, 42, 43)
  py ecall('pa4(42, 43)', pa4, 42, 43)
  py ecall('sa(42, 43)', sa, 42, 43)
  py ecall('psa1(42, 43)', psa1, 42, 43)
  py ecall('psa2(42, 43)', psa2, 42, 43)
  py ecall('psa3(42, 43)', psa3, 42, 43)
  py ecall('psa4(42, 43)', psa4, 42, 43)

  py ecall('a(42, self={"20": 1})', a, 42, self={'20': 1})
  py ecall('pa1(42, self={"20": 1})', pa1, 42, self={'20': 1})
  py ecall('pa2(42, self={"20": 1})', pa2, 42, self={'20': 1})
  py ecall('pa3(42, self={"20": 1})', pa3, 42, self={'20': 1})
  py ecall('pa4(42, self={"20": 1})', pa4, 42, self={'20': 1})
  py ecall('sa(42, self={"20": 1})', sa, 42, self={'20': 1})
  py ecall('psa1(42, self={"20": 1})', psa1, 42, self={'20': 1})
  py ecall('psa2(42, self={"20": 1})', psa2, 42, self={'20': 1})
  py ecall('psa3(42, self={"20": 1})', psa3, 42, self={'20': 1})
  py ecall('psa4(42, self={"20": 1})', psa4, 42, self={'20': 1})

  py ecall('a(self={"20": 1})', a, self={'20': 1})
  py ecall('pa1(self={"20": 1})', pa1, self={'20': 1})
  py ecall('pa2(self={"20": 1})', pa2, self={'20': 1})
  py ecall('pa3(self={"20": 1})', pa3, self={'20': 1})
  py ecall('pa4(self={"20": 1})', pa4, self={'20': 1})
  py ecall('sa(self={"20": 1})', sa, self={'20': 1})
  py ecall('psa1(self={"20": 1})', psa1, self={'20': 1})
  py ecall('psa2(self={"20": 1})', psa2, self={'20': 1})
  py ecall('psa3(self={"20": 1})', psa3, self={'20': 1})
  py ecall('psa4(self={"20": 1})', psa4, self={'20': 1})

  py << trim EOF
    def s(v):
        if v is None:
            return repr(v)
        else:
            return vim.Function('string')(v)

    cb.append('a.args: ' + s(a.args))
    cb.append('pa1.args: ' + s(pa1.args))
    cb.append('pa2.args: ' + s(pa2.args))
    cb.append('pa3.args: ' + s(pa3.args))
    cb.append('pa4.args: ' + s(pa4.args))
    cb.append('sa.args: ' + s(sa.args))
    cb.append('psa1.args: ' + s(psa1.args))
    cb.append('psa2.args: ' + s(psa2.args))
    cb.append('psa3.args: ' + s(psa3.args))
    cb.append('psa4.args: ' + s(psa4.args))

    cb.append('a.self: ' + s(a.self))
    cb.append('pa1.self: ' + s(pa1.self))
    cb.append('pa2.self: ' + s(pa2.self))
    cb.append('pa3.self: ' + s(pa3.self))
    cb.append('pa4.self: ' + s(pa4.self))
    cb.append('sa.self: ' + s(sa.self))
    cb.append('psa1.self: ' + s(psa1.self))
    cb.append('psa2.self: ' + s(psa2.self))
    cb.append('psa3.self: ' + s(psa3.self))
    cb.append('psa4.self: ' + s(psa4.self))

    cb.append('a.name: ' + s(a.name))
    cb.append('pa1.name: ' + s(pa1.name))
    cb.append('pa2.name: ' + s(pa2.name))
    cb.append('pa3.name: ' + s(pa3.name))
    cb.append('pa4.name: ' + s(pa4.name))
    cb.append('sa.name: ' + s(sa.name))
    cb.append('psa1.name: ' + s(psa1.name))
    cb.append('psa2.name: ' + s(psa2.name))
    cb.append('psa3.name: ' + s(psa3.name))
    cb.append('psa4.name: ' + s(psa4.name))

    cb.append('a.auto_rebind: ' + s(a.auto_rebind))
    cb.append('pa1.auto_rebind: ' + s(pa1.auto_rebind))
    cb.append('pa2.auto_rebind: ' + s(pa2.auto_rebind))
    cb.append('pa3.auto_rebind: ' + s(pa3.auto_rebind))
    cb.append('pa4.auto_rebind: ' + s(pa4.auto_rebind))
    cb.append('sa.auto_rebind: ' + s(sa.auto_rebind))
    cb.append('psa1.auto_rebind: ' + s(psa1.auto_rebind))
    cb.append('psa2.auto_rebind: ' + s(psa2.auto_rebind))
    cb.append('psa3.auto_rebind: ' + s(psa3.auto_rebind))
    cb.append('psa4.auto_rebind: ' + s(psa4.auto_rebind))
    cb.append('psa5.auto_rebind: ' + s(psa5.auto_rebind))
    cb.append('psa6.auto_rebind: ' + s(psa6.auto_rebind))
    cb.append('psa7.auto_rebind: ' + s(psa7.auto_rebind))
    cb.append('psa8.auto_rebind: ' + s(psa8.auto_rebind))
    cb.append('psa9.auto_rebind: ' + s(psa9.auto_rebind))
    cb.append('psaA.auto_rebind: ' + s(psaA.auto_rebind))
    cb.append('psaB.auto_rebind: ' + s(psaB.auto_rebind))
    cb.append('psaC.auto_rebind: ' + s(psaC.auto_rebind))

    del s

    del a
    del pa1
    del pa2
    del pa3
    del pa4
    del sa
    del psa1
    del psa2
    del psa3
    del psa4
    del psa5
    del psa6
    del psa7
    del psa8
    del psa9
    del psaA
    del psaB
    del psaC
    del psar

    del ecall
  EOF

  let expected =<< trim END
    a(): !result: []
    pa1(): !result: ['abcArgsPA1']
    pa2(): !result: []
    pa3(): !result: ['abcArgsPA3']
    pa4(): !result: []
    sa(): !exception: error:('Vim:E725: Calling dict function without Dictionary: SelfArgs',)
    psa1(): !exception: error:('Vim:E725: Calling dict function without Dictionary: SelfArgs',)
    psa2(): !exception: error:('Vim:E725: Calling dict function without Dictionary: SelfArgs',)
    psa3(): !result: [['abcArgsPSA3'], {'abcSelfPSA3': 'abcSelfPSA3Val'}]
    psa4(): !result: [[], {'abcSelfPSA4': 'abcSelfPSA4Val'}]
    a(42, 43): !result: [42, 43]
    pa1(42, 43): !result: ['abcArgsPA1', 42, 43]
    pa2(42, 43): !result: [42, 43]
    pa3(42, 43): !result: ['abcArgsPA3', 42, 43]
    pa4(42, 43): !result: [42, 43]
    sa(42, 43): !exception: error:('Vim:E725: Calling dict function without Dictionary: SelfArgs',)
    psa1(42, 43): !exception: error:('Vim:E725: Calling dict function without Dictionary: SelfArgs',)
    psa2(42, 43): !exception: error:('Vim:E725: Calling dict function without Dictionary: SelfArgs',)
    psa3(42, 43): !result: [['abcArgsPSA3', 42, 43], {'abcSelfPSA3': 'abcSelfPSA3Val'}]
    psa4(42, 43): !result: [[42, 43], {'abcSelfPSA4': 'abcSelfPSA4Val'}]
    a(42, self={"20": 1}): !result: [42]
    pa1(42, self={"20": 1}): !result: ['abcArgsPA1', 42]
    pa2(42, self={"20": 1}): !result: [42]
    pa3(42, self={"20": 1}): !result: ['abcArgsPA3', 42]
    pa4(42, self={"20": 1}): !result: [42]
    sa(42, self={"20": 1}): !result: [[42], {'20': 1}]
    psa1(42, self={"20": 1}): !result: [['abcArgsPSA1', 42], {'20': 1}]
    psa2(42, self={"20": 1}): !result: [[42], {'20': 1}]
    psa3(42, self={"20": 1}): !result: [['abcArgsPSA3', 42], {'20': 1}]
    psa4(42, self={"20": 1}): !result: [[42], {'20': 1}]
    a(self={"20": 1}): !result: []
    pa1(self={"20": 1}): !result: ['abcArgsPA1']
    pa2(self={"20": 1}): !result: []
    pa3(self={"20": 1}): !result: ['abcArgsPA3']
    pa4(self={"20": 1}): !result: []
    sa(self={"20": 1}): !result: [[], {'20': 1}]
    psa1(self={"20": 1}): !result: [['abcArgsPSA1'], {'20': 1}]
    psa2(self={"20": 1}): !result: [[], {'20': 1}]
    psa3(self={"20": 1}): !result: [['abcArgsPSA3'], {'20': 1}]
    psa4(self={"20": 1}): !result: [[], {'20': 1}]
    a.args: None
    pa1.args: ['abcArgsPA1']
    pa2.args: None
    pa3.args: ['abcArgsPA3']
    pa4.args: None
    sa.args: None
    psa1.args: ['abcArgsPSA1']
    psa2.args: None
    psa3.args: ['abcArgsPSA3']
    psa4.args: None
    a.self: None
    pa1.self: None
    pa2.self: None
    pa3.self: {'abcSelfPA3': 'abcSelfPA3Val'}
    pa4.self: {'abcSelfPA4': 'abcSelfPA4Val'}
    sa.self: None
    psa1.self: None
    psa2.self: None
    psa3.self: {'abcSelfPSA3': 'abcSelfPSA3Val'}
    psa4.self: {'abcSelfPSA4': 'abcSelfPSA4Val'}
    a.name: 'Args'
    pa1.name: 'Args'
    pa2.name: 'Args'
    pa3.name: 'Args'
    pa4.name: 'Args'
    sa.name: 'SelfArgs'
    psa1.name: 'SelfArgs'
    psa2.name: 'SelfArgs'
    psa3.name: 'SelfArgs'
    psa4.name: 'SelfArgs'
    a.auto_rebind: 1
    pa1.auto_rebind: 1
    pa2.auto_rebind: 1
    pa3.auto_rebind: 0
    pa4.auto_rebind: 0
    sa.auto_rebind: 1
    psa1.auto_rebind: 1
    psa2.auto_rebind: 1
    psa3.auto_rebind: 0
    psa4.auto_rebind: 0
    psa5.auto_rebind: 0
    psa6.auto_rebind: 0
    psa7.auto_rebind: 1
    psa8.auto_rebind: 1
    psa9.auto_rebind: 1
    psaA.auto_rebind: 1
    psaB.auto_rebind: 1
    psaC.auto_rebind: 1
  END
  call assert_equal(expected, getline(2, '$'))
  %bw!
endfunc

" Test stdout/stderr
func Test_python_stdin_stderr()
  let caught_writeerr = 0
  let caught_writelineerr = 0
  redir => messages
  py sys.stdout.write('abc8') ; sys.stdout.write('def')
  try
    py sys.stderr.write('abc9') ; sys.stderr.write('def')
  catch /abc9def/
    let caught_writeerr = 1
  endtry
  py sys.stdout.writelines(iter('abcA'))
  try
    py sys.stderr.writelines(iter('abcB'))
  catch /abcB/
    let caught_writelineerr = 1
  endtry
  redir END
  call assert_equal("\nabc8def\nabcA", messages)
  call assert_equal(1, caught_writeerr)
  call assert_equal(1, caught_writelineerr)
endfunc

" Test subclassing
func Test_python_subclass()
  new
  func Put(...)
    return a:000
  endfunc

  py << trim EOF
    class DupDict(vim.Dictionary):
      def __setitem__(self, key, value):
        super(DupDict, self).__setitem__(key, value)
        super(DupDict, self).__setitem__('dup_' + key, value)
    dd = DupDict()
    dd['a'] = 'b'

    class DupList(vim.List):
      def __getitem__(self, idx):
        return [super(DupList, self).__getitem__(idx)] * 2

    dl = DupList()
    dl2 = DupList(iter('abcC'))
    dl.extend(dl2[0])

    class DupFun(vim.Function):
      def __call__(self, arg):
        return super(DupFun, self).__call__(arg, arg)

    df = DupFun('Put')
  EOF

  call assert_equal(['a', 'dup_a'], sort(keys(pyeval('dd'))))
  call assert_equal(['a', 'a'], pyeval('dl'))
  call assert_equal(['a', 'b', 'c', 'C'], pyeval('dl2'))
  call assert_equal([2, 2], pyeval('df(2)'))
  call assert_equal(1, pyeval('dl') is# pyeval('dl'))
  call assert_equal(1, pyeval('dd') is# pyeval('dd'))
  call assert_equal(function('Put'), pyeval('df'))
  delfunction Put
  py << trim EOF
    del DupDict
    del DupList
    del DupFun
    del dd
    del dl
    del dl2
    del df
  EOF
  close!
endfunc

" Test chdir
func Test_python_chdir()
  new Xpycfile
  py cb = vim.current.buffer
  py << trim EOF
    import os
    fnamemodify = vim.Function('fnamemodify')
    cb.append(fnamemodify('.', ':p:h:t'))
    cb.append(vim.eval('@%'))
    os.chdir('..')
    path = fnamemodify('.', ':p:h:t')
    if path != 'src' and path != 'src2':
      # Running tests from a shadow directory, so move up another level
      # This will result in @% looking like shadow/testdir/Xpycfile, hence the
      # extra fnamemodify
      os.chdir('..')
      cb.append(fnamemodify('.', ':p:h:t'))
      cb.append(fnamemodify(vim.eval('@%'), ':s?^%s.??' % path).replace(os.path.sep, '/'))
      os.chdir(path)
      del path
    else:
      # Also accept running from src2/testdir/ for MS-Windows CI.
      cb.append(fnamemodify('.', ':p:h:t').replace('src2', 'src'))
      cb.append(vim.eval('@%').replace(os.path.sep, '/'))
    os.chdir('testdir')
    cb.append(fnamemodify('.', ':p:h:t'))
    cb.append(vim.eval('@%'))
    del fnamemodify
  EOF
  call assert_equal(['testdir', 'Xpycfile', 'src', 'testdir/Xpycfile', 'testdir',
        \ 'Xpycfile'], getline(2, '$'))
  close!
  call AssertException(["py vim.chdir(None)"], "Vim(python):TypeError:")
endfunc

" Test errors
func Test_python_errors()
  func F() dict
  endfunc

  func D()
  endfunc

  new
  py cb = vim.current.buffer

  py << trim EOF
    d = vim.Dictionary()
    ned = vim.Dictionary(foo='bar', baz='abcD')
    dl = vim.Dictionary(a=1)
    dl.locked = True
    l = vim.List()
    ll = vim.List('abcE')
    ll.locked = True
    nel = vim.List('abcO')
    f = vim.Function('string')
    fd = vim.Function('F')
    fdel = vim.Function('D')
    vim.command('delfunction D')

    def subexpr_test(expr, name, subexprs):
        cb.append('>>> Testing %s using %s' % (name, expr))
        for subexpr in subexprs:
            ee(expr % subexpr)
        cb.append('<<< Finished')

    def stringtochars_test(expr):
        return subexpr_test(expr, 'StringToChars', (
            '1',       # Fail type checks
            'u"\\0"',  # Fail PyString_AsStringAndSize(bytes, , NULL) check
            '"\\0"',   # Fail PyString_AsStringAndSize(object, , NULL) check
        ))

    class Mapping(object):
        def __init__(self, d):
            self.d = d

        def __getitem__(self, key):
            return self.d[key]

        def keys(self):
            return self.d.keys()

        def items(self):
            return self.d.items()

    def convertfrompyobject_test(expr, recurse=True):
        # pydict_to_tv
        stringtochars_test(expr % '{%s : 1}')
        if recurse:
            convertfrompyobject_test(expr % '{"abcF" : %s}', False)
        # pymap_to_tv
        stringtochars_test(expr % 'Mapping({%s : 1})')
        if recurse:
            convertfrompyobject_test(expr % 'Mapping({"abcG" : %s})', False)
        # pyseq_to_tv
        iter_test(expr)
        return subexpr_test(expr, 'ConvertFromPyObject', (
            'None',                 # Not conversible
            '{"": 1}',              # Empty key not allowed
            '{u"": 1}',             # Same, but with unicode object
            'FailingMapping()',     #
            'FailingMappingKey()',  #
            'FailingNumber()',      #
        ))

    def convertfrompymapping_test(expr):
        convertfrompyobject_test(expr)
        return subexpr_test(expr, 'ConvertFromPyMapping', (
            '[]',
        ))

    def iter_test(expr):
        return subexpr_test(expr, '*Iter*', (
            'FailingIter()',
            'FailingIterNext()',
        ))

    def number_test(expr, natural=False, unsigned=False):
        if natural:
            unsigned = True
        return subexpr_test(expr, 'NumberToLong', (
            '[]',
            'None',
        ) + (unsigned and ('-1',) or ())
        + (natural and ('0',) or ()))

    class FailingTrue(object):
        def __nonzero__(self):
            raise NotImplementedError('bool')

    class FailingIter(object):
        def __iter__(self):
            raise NotImplementedError('iter')

    class FailingIterNext(object):
        def __iter__(self):
            return self

        def next(self):
            raise NotImplementedError('next')

    class FailingIterNextN(object):
        def __init__(self, n):
            self.n = n

        def __iter__(self):
            return self

        def next(self):
            if self.n:
                self.n -= 1
                return 1
            else:
                raise NotImplementedError('next N')

    class FailingMappingKey(object):
        def __getitem__(self, item):
            raise NotImplementedError('getitem:mappingkey')

        def keys(self):
            return list("abcH")

    class FailingMapping(object):
        def __getitem__(self):
            raise NotImplementedError('getitem:mapping')

        def keys(self):
            raise NotImplementedError('keys')

    class FailingList(list):
        def __getitem__(self, idx):
            if i == 2:
                raise NotImplementedError('getitem:list')
            else:
                return super(FailingList, self).__getitem__(idx)

    class NoArgsCall(object):
        def __call__(self):
            pass

    class FailingCall(object):
        def __call__(self, path):
            raise NotImplementedError('call')

    class FailingNumber(object):
        def __int__(self):
            raise NotImplementedError('int')

    cb.append("> Output")
    cb.append(">> OutputSetattr")
    ee('del sys.stdout.softspace')
    number_test('sys.stdout.softspace = %s', unsigned=True)
    number_test('sys.stderr.softspace = %s', unsigned=True)
    ee('assert sys.stdout.isatty()==False')
    ee('assert sys.stdout.seekable()==False')
    ee('sys.stdout.close()')
    ee('sys.stdout.flush()')
    ee('assert sys.stderr.isatty()==False')
    ee('assert sys.stderr.seekable()==False')
    ee('sys.stderr.close()')
    ee('sys.stderr.flush()')
    ee('sys.stdout.attr = None')
    cb.append(">> OutputWrite")
    ee('assert sys.stdout.writable()==True')
    ee('assert sys.stdout.readable()==False')
    ee('assert sys.stderr.writable()==True')
    ee('assert sys.stderr.readable()==False')
    ee('assert sys.stdout.closed()==False')
    ee('assert sys.stderr.closed()==False')
    ee('assert sys.stdout.errors=="strict"')
    ee('assert sys.stderr.errors=="strict"')
    ee('assert sys.stdout.encoding==sys.stderr.encoding')
    ee('sys.stdout.write(None)')
    cb.append(">> OutputWriteLines")
    ee('sys.stdout.writelines(None)')
    ee('sys.stdout.writelines([1])')
    iter_test('sys.stdout.writelines(%s)')
    cb.append("> VimCommand")
    stringtochars_test('vim.command(%s)')
    ee('vim.command("", 2)')
    #! Not checked: vim->python exceptions translating: checked later
    cb.append("> VimToPython")
    #! Not checked: everything: needs errors in internal python functions
    cb.append("> VimEval")
    stringtochars_test('vim.eval(%s)')
    ee('vim.eval("", FailingTrue())')
    #! Not checked: everything: needs errors in internal python functions
    cb.append("> VimEvalPy")
    stringtochars_test('vim.bindeval(%s)')
    ee('vim.eval("", 2)')
    #! Not checked: vim->python exceptions translating: checked later
    cb.append("> VimStrwidth")
    stringtochars_test('vim.strwidth(%s)')
    cb.append("> VimForeachRTP")
    ee('vim.foreach_rtp(None)')
    ee('vim.foreach_rtp(NoArgsCall())')
    ee('vim.foreach_rtp(FailingCall())')
    ee('vim.foreach_rtp(int, 2)')
    cb.append('> import')
    old_rtp = vim.options['rtp']
    vim.options['rtp'] = os.getcwd().replace('\\', '\\\\').replace(',', '\\,')
    ee('import xxx_no_such_module_xxx')
    ee('import failing_import')
    ee('import failing')
    vim.options['rtp'] = old_rtp
    del old_rtp
    cb.append("> Options")
    cb.append(">> OptionsItem")
    ee('vim.options["abcQ"]')
    ee('vim.options[""]')
    stringtochars_test('vim.options[%s]')
    cb.append(">> OptionsContains")
    stringtochars_test('%s in vim.options')
    cb.append("> Dictionary")
    cb.append(">> DictionaryConstructor")
    ee('vim.Dictionary("abcI")')
    ##! Not checked: py_dict_alloc failure
    cb.append(">> DictionarySetattr")
    ee('del d.locked')
    ee('d.locked = FailingTrue()')
    ee('vim.vvars.locked = False')
    ee('d.scope = True')
    ee('d.xxx = True')
    cb.append(">> _DictionaryItem")
    ee('d.get("a", 2, 3)')
    stringtochars_test('d.get(%s)')
    ee('d.pop("a")')
    ee('dl.pop("a")')
    cb.append(">> DictionaryContains")
    ee('"" in d')
    ee('0 in d')
    cb.append(">> DictionaryIterNext")
    ee('for i in ned: ned["a"] = 1')
    del i
    cb.append(">> DictionaryAssItem")
    ee('dl["b"] = 1')
    stringtochars_test('d[%s] = 1')
    convertfrompyobject_test('d["a"] = %s')
    cb.append(">> DictionaryUpdate")
    cb.append(">>> kwargs")
    cb.append(">>> iter")
    ee('d.update(FailingMapping())')
    ee('d.update([FailingIterNext()])')
    ee('d.update([FailingIterNextN(1)])')
    iter_test('d.update(%s)')
    convertfrompyobject_test('d.update(%s)')
    stringtochars_test('d.update(((%s, 0),))')
    convertfrompyobject_test('d.update((("a", %s),))')
    cb.append(">> DictionaryPopItem")
    ee('d.popitem(1, 2)')
    cb.append(">> DictionaryHasKey")
    ee('d.has_key()')
    cb.append("> List")
    cb.append(">> ListConstructor")
    ee('vim.List(1, 2)')
    ee('vim.List(a=1)')
    iter_test('vim.List(%s)')
    convertfrompyobject_test('vim.List([%s])')
    cb.append(">> ListItem")
    ee('l[1000]')
    cb.append(">> ListAssItem")
    ee('ll[1] = 2')
    ee('l[1000] = 3')
    cb.append(">> ListAssSlice")
    ee('ll[1:100] = "abcJ"')
    iter_test('l[:] = %s')
    ee('nel[1:10:2]  = "abcK"')
    cb.append(repr(tuple(nel)))
    ee('nel[1:10:2]  = "a"')
    cb.append(repr(tuple(nel)))
    ee('nel[1:1:-1]  = "a"')
    cb.append(repr(tuple(nel)))
    ee('nel[:] = FailingIterNextN(2)')
    cb.append(repr(tuple(nel)))
    convertfrompyobject_test('l[:] = [%s]')
    cb.append(">> ListConcatInPlace")
    iter_test('l.extend(%s)')
    convertfrompyobject_test('l.extend([%s])')
    cb.append(">> ListSetattr")
    ee('del l.locked')
    ee('l.locked = FailingTrue()')
    ee('l.xxx = True')
    cb.append("> Function")
    cb.append(">> FunctionConstructor")
    cb.append(">>> FunctionConstructor")
    ee('vim.Function("123")')
    ee('vim.Function("xxx_non_existent_function_xxx")')
    ee('vim.Function("xxx#non#existent#function#xxx")')
    ee('vim.Function("xxx_non_existent_function_xxx2", args=[])')
    ee('vim.Function("xxx_non_existent_function_xxx3", self={})')
    ee('vim.Function("xxx_non_existent_function_xxx4", args=[], self={})')
    cb.append(">>> FunctionNew")
    ee('vim.Function("tr", self="abcFuncSelf")')
    ee('vim.Function("tr", args=427423)')
    ee('vim.Function("tr", self="abcFuncSelf2", args="abcFuncArgs2")')
    ee('vim.Function(self="abcFuncSelf2", args="abcFuncArgs2")')
    ee('vim.Function("tr", "", self="abcFuncSelf2", args="abcFuncArgs2")')
    ee('vim.Function("tr", "")')
    cb.append(">> FunctionCall")
    convertfrompyobject_test('f(%s)')
    convertfrompymapping_test('fd(self=%s)')
    cb.append("> TabPage")
    cb.append(">> TabPageAttr")
    ee('vim.current.tabpage.xxx')
    cb.append("> TabList")
    cb.append(">> TabListItem")
    ee('vim.tabpages[1000]')
    cb.append("> Window")
    cb.append(">> WindowAttr")
    ee('vim.current.window.xxx')
    cb.append(">> WindowSetattr")
    ee('vim.current.window.buffer = 0')
    ee('vim.current.window.cursor = (100000000, 100000000)')
    ee('vim.current.window.cursor = True')
    number_test('vim.current.window.height = %s', unsigned=True)
    number_test('vim.current.window.width = %s', unsigned=True)
    ee('vim.current.window.xxxxxx = True')
    cb.append("> WinList")
    cb.append(">> WinListItem")
    ee('vim.windows[1000]')
    cb.append("> Buffer")
    cb.append(">> StringToLine (indirect)")
    ee('vim.current.buffer[0] = "\\na"')
    ee('vim.current.buffer[0] = u"\\na"')
    cb.append(">> SetBufferLine (indirect)")
    ee('vim.current.buffer[0] = True')
    cb.append(">> SetBufferLineList (indirect)")
    ee('vim.current.buffer[:] = True')
    ee('vim.current.buffer[:] = ["\\na", "bc"]')
    cb.append(">> InsertBufferLines (indirect)")
    ee('vim.current.buffer.append(None)')
    ee('vim.current.buffer.append(["\\na", "bc"])')
    ee('vim.current.buffer.append("\\nbc")')
    cb.append(">> RBItem")
    ee('vim.current.buffer[100000000]')
    cb.append(">> RBAsItem")
    ee('vim.current.buffer[100000000] = ""')
    cb.append(">> BufferAttr")
    ee('vim.current.buffer.xxx')
    cb.append(">> BufferSetattr")
    ee('vim.current.buffer.name = True')
    ee('vim.current.buffer.xxx = True')
    cb.append(">> BufferMark")
    ee('vim.current.buffer.mark(0)')
    ee('vim.current.buffer.mark("abcM")')
    ee('vim.current.buffer.mark("!")')
    cb.append(">> BufferRange")
    ee('vim.current.buffer.range(1, 2, 3)')
    cb.append("> BufMap")
    cb.append(">> BufMapItem")
    ee('vim.buffers[100000000]')
    number_test('vim.buffers[%s]', natural=True)
    cb.append("> Current")
    cb.append(">> CurrentGetattr")
    ee('vim.current.xxx')
    cb.append(">> CurrentSetattr")
    ee('vim.current.line = True')
    ee('vim.current.buffer = True')
    ee('vim.current.window = True')
    ee('vim.current.tabpage = True')
    ee('vim.current.xxx = True')
    del d
    del ned
    del dl
    del l
    del ll
    del nel
    del f
    del fd
    del fdel
    del subexpr_test
    del stringtochars_test
    del Mapping
    del convertfrompyobject_test
    del convertfrompymapping_test
    del iter_test
    del number_test
    del FailingTrue
    del FailingIter
    del FailingIterNext
    del FailingIterNextN
    del FailingMapping
    del FailingMappingKey
    del FailingList
    del NoArgsCall
    del FailingCall
    del FailingNumber
  EOF
  delfunction F

  let expected =<< trim END
    > Output
    >> OutputSetattr
    del sys.stdout.softspace:AttributeError:('cannot delete OutputObject attributes',)
    >>> Testing NumberToLong using sys.stdout.softspace = %s
    sys.stdout.softspace = []:TypeError:('expected int(), long() or something supporting coercing to long(), but got list',)
    sys.stdout.softspace = None:TypeError:('expected int(), long() or something supporting coercing to long(), but got NoneType',)
    sys.stdout.softspace = -1:ValueError:('number must be greater or equal to zero',)
    <<< Finished
    >>> Testing NumberToLong using sys.stderr.softspace = %s
    sys.stderr.softspace = []:TypeError:('expected int(), long() or something supporting coercing to long(), but got list',)
    sys.stderr.softspace = None:TypeError:('expected int(), long() or something supporting coercing to long(), but got NoneType',)
    sys.stderr.softspace = -1:ValueError:('number must be greater or equal to zero',)
    <<< Finished
    assert sys.stdout.isatty()==False:NOT FAILED
    assert sys.stdout.seekable()==False:NOT FAILED
    sys.stdout.close():NOT FAILED
    sys.stdout.flush():NOT FAILED
    assert sys.stderr.isatty()==False:NOT FAILED
    assert sys.stderr.seekable()==False:NOT FAILED
    sys.stderr.close():NOT FAILED
    sys.stderr.flush():NOT FAILED
    sys.stdout.attr = None:AttributeError:('invalid attribute: attr',)
    >> OutputWrite
    assert sys.stdout.writable()==True:NOT FAILED
    assert sys.stdout.readable()==False:NOT FAILED
    assert sys.stderr.writable()==True:NOT FAILED
    assert sys.stderr.readable()==False:NOT FAILED
    assert sys.stdout.closed()==False:NOT FAILED
    assert sys.stderr.closed()==False:NOT FAILED
    assert sys.stdout.errors=="strict":NOT FAILED
    assert sys.stderr.errors=="strict":NOT FAILED
    assert sys.stdout.encoding==sys.stderr.encoding:NOT FAILED
    sys.stdout.write(None):TypeError:('coercing to Unicode: need string or buffer, NoneType found',)
    >> OutputWriteLines
    sys.stdout.writelines(None):TypeError:("'NoneType' object is not iterable",)
    sys.stdout.writelines([1]):TypeError:('coercing to Unicode: need string or buffer, int found',)
    >>> Testing *Iter* using sys.stdout.writelines(%s)
    sys.stdout.writelines(FailingIter()):NotImplementedError:('iter',)
    sys.stdout.writelines(FailingIterNext()):NotImplementedError:('next',)
    <<< Finished
    > VimCommand
    >>> Testing StringToChars using vim.command(%s)
    vim.command(1):TypeError:('expected str() or unicode() instance, but got int',)
    vim.command(u"\0"):TypeError:('expected string without null bytes',)
    vim.command("\0"):TypeError:('expected string without null bytes',)
    <<< Finished
    vim.command("", 2):TypeError:('command() takes exactly one argument (2 given)',)
    > VimToPython
    > VimEval
    >>> Testing StringToChars using vim.eval(%s)
    vim.eval(1):TypeError:('expected str() or unicode() instance, but got int',)
    vim.eval(u"\0"):TypeError:('expected string without null bytes',)
    vim.eval("\0"):TypeError:('expected string without null bytes',)
    <<< Finished
    vim.eval("", FailingTrue()):TypeError:('function takes exactly 1 argument (2 given)',)
    > VimEvalPy
    >>> Testing StringToChars using vim.bindeval(%s)
    vim.bindeval(1):TypeError:('expected str() or unicode() instance, but got int',)
    vim.bindeval(u"\0"):TypeError:('expected string without null bytes',)
    vim.bindeval("\0"):TypeError:('expected string without null bytes',)
    <<< Finished
    vim.eval("", 2):TypeError:('function takes exactly 1 argument (2 given)',)
    > VimStrwidth
    >>> Testing StringToChars using vim.strwidth(%s)
    vim.strwidth(1):TypeError:('expected str() or unicode() instance, but got int',)
    vim.strwidth(u"\0"):TypeError:('expected string without null bytes',)
    vim.strwidth("\0"):TypeError:('expected string without null bytes',)
    <<< Finished
    > VimForeachRTP
    vim.foreach_rtp(None):TypeError:("'NoneType' object is not callable",)
    vim.foreach_rtp(NoArgsCall()):TypeError:('__call__() takes exactly 1 argument (2 given)',)
    vim.foreach_rtp(FailingCall()):NotImplementedError:('call',)
    vim.foreach_rtp(int, 2):TypeError:('foreach_rtp() takes exactly one argument (2 given)',)
    > import
    import xxx_no_such_module_xxx:ImportError:('No module named xxx_no_such_module_xxx',)
    import failing_import:ImportError:()
    import failing:NotImplementedError:()
    > Options
    >> OptionsItem
    vim.options["abcQ"]:KeyError:('abcQ',)
    vim.options[""]:ValueError:('empty keys are not allowed',)
    >>> Testing StringToChars using vim.options[%s]
    vim.options[1]:TypeError:('expected str() or unicode() instance, but got int',)
    vim.options[u"\0"]:TypeError:('expected string without null bytes',)
    vim.options["\0"]:TypeError:('expected string without null bytes',)
    <<< Finished
    >> OptionsContains
    >>> Testing StringToChars using %s in vim.options
    1 in vim.options:TypeError:('expected str() or unicode() instance, but got int',)
    u"\0" in vim.options:TypeError:('expected string without null bytes',)
    "\0" in vim.options:TypeError:('expected string without null bytes',)
    <<< Finished
    > Dictionary
    >> DictionaryConstructor
    vim.Dictionary("abcI"):ValueError:('expected sequence element of size 2, but got sequence of size 1',)
    >> DictionarySetattr
    del d.locked:AttributeError:('cannot delete vim.Dictionary attributes',)
    d.locked = FailingTrue():NotImplementedError:('bool',)
    vim.vvars.locked = False:TypeError:('cannot modify fixed dictionary',)
    d.scope = True:AttributeError:('cannot set attribute scope',)
    d.xxx = True:AttributeError:('cannot set attribute xxx',)
    >> _DictionaryItem
    d.get("a", 2, 3):TypeError:('function takes at most 2 arguments (3 given)',)
    >>> Testing StringToChars using d.get(%s)
    d.get(1):TypeError:('expected str() or unicode() instance, but got int',)
    d.get(u"\0"):TypeError:('expected string without null bytes',)
    d.get("\0"):TypeError:('expected string without null bytes',)
    <<< Finished
    d.pop("a"):KeyError:('a',)
    dl.pop("a"):error:('dictionary is locked',)
    >> DictionaryContains
    "" in d:ValueError:('empty keys are not allowed',)
    0 in d:TypeError:('expected str() or unicode() instance, but got int',)
    >> DictionaryIterNext
    for i in ned: ned["a"] = 1:RuntimeError:('hashtab changed during iteration',)
    >> DictionaryAssItem
    dl["b"] = 1:error:('dictionary is locked',)
    >>> Testing StringToChars using d[%s] = 1
    d[1] = 1:TypeError:('expected str() or unicode() instance, but got int',)
    d[u"\0"] = 1:TypeError:('expected string without null bytes',)
    d["\0"] = 1:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d["a"] = {%s : 1}
    d["a"] = {1 : 1}:TypeError:('expected str() or unicode() instance, but got int',)
    d["a"] = {u"\0" : 1}:TypeError:('expected string without null bytes',)
    d["a"] = {"\0" : 1}:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d["a"] = {"abcF" : {%s : 1}}
    d["a"] = {"abcF" : {1 : 1}}:TypeError:('expected str() or unicode() instance, but got int',)
    d["a"] = {"abcF" : {u"\0" : 1}}:TypeError:('expected string without null bytes',)
    d["a"] = {"abcF" : {"\0" : 1}}:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d["a"] = {"abcF" : Mapping({%s : 1})}
    d["a"] = {"abcF" : Mapping({1 : 1})}:TypeError:('expected str() or unicode() instance, but got int',)
    d["a"] = {"abcF" : Mapping({u"\0" : 1})}:TypeError:('expected string without null bytes',)
    d["a"] = {"abcF" : Mapping({"\0" : 1})}:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using d["a"] = {"abcF" : %s}
    d["a"] = {"abcF" : FailingIter()}:TypeError:('unable to convert FailingIter to a Vim structure',)
    d["a"] = {"abcF" : FailingIterNext()}:NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using d["a"] = {"abcF" : %s}
    d["a"] = {"abcF" : None}:NOT FAILED
    d["a"] = {"abcF" : {"": 1}}:ValueError:('empty keys are not allowed',)
    d["a"] = {"abcF" : {u"": 1}}:ValueError:('empty keys are not allowed',)
    d["a"] = {"abcF" : FailingMapping()}:NotImplementedError:('keys',)
    d["a"] = {"abcF" : FailingMappingKey()}:NotImplementedError:('getitem:mappingkey',)
    d["a"] = {"abcF" : FailingNumber()}:TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing StringToChars using d["a"] = Mapping({%s : 1})
    d["a"] = Mapping({1 : 1}):TypeError:('expected str() or unicode() instance, but got int',)
    d["a"] = Mapping({u"\0" : 1}):TypeError:('expected string without null bytes',)
    d["a"] = Mapping({"\0" : 1}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d["a"] = Mapping({"abcG" : {%s : 1}})
    d["a"] = Mapping({"abcG" : {1 : 1}}):TypeError:('expected str() or unicode() instance, but got int',)
    d["a"] = Mapping({"abcG" : {u"\0" : 1}}):TypeError:('expected string without null bytes',)
    d["a"] = Mapping({"abcG" : {"\0" : 1}}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d["a"] = Mapping({"abcG" : Mapping({%s : 1})})
    d["a"] = Mapping({"abcG" : Mapping({1 : 1})}):TypeError:('expected str() or unicode() instance, but got int',)
    d["a"] = Mapping({"abcG" : Mapping({u"\0" : 1})}):TypeError:('expected string without null bytes',)
    d["a"] = Mapping({"abcG" : Mapping({"\0" : 1})}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using d["a"] = Mapping({"abcG" : %s})
    d["a"] = Mapping({"abcG" : FailingIter()}):TypeError:('unable to convert FailingIter to a Vim structure',)
    d["a"] = Mapping({"abcG" : FailingIterNext()}):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using d["a"] = Mapping({"abcG" : %s})
    d["a"] = Mapping({"abcG" : None}):NOT FAILED
    d["a"] = Mapping({"abcG" : {"": 1}}):ValueError:('empty keys are not allowed',)
    d["a"] = Mapping({"abcG" : {u"": 1}}):ValueError:('empty keys are not allowed',)
    d["a"] = Mapping({"abcG" : FailingMapping()}):NotImplementedError:('keys',)
    d["a"] = Mapping({"abcG" : FailingMappingKey()}):NotImplementedError:('getitem:mappingkey',)
    d["a"] = Mapping({"abcG" : FailingNumber()}):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing *Iter* using d["a"] = %s
    d["a"] = FailingIter():TypeError:('unable to convert FailingIter to a Vim structure',)
    d["a"] = FailingIterNext():NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using d["a"] = %s
    d["a"] = None:NOT FAILED
    d["a"] = {"": 1}:ValueError:('empty keys are not allowed',)
    d["a"] = {u"": 1}:ValueError:('empty keys are not allowed',)
    d["a"] = FailingMapping():NotImplementedError:('keys',)
    d["a"] = FailingMappingKey():NotImplementedError:('getitem:mappingkey',)
    d["a"] = FailingNumber():TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >> DictionaryUpdate
    >>> kwargs
    >>> iter
    d.update(FailingMapping()):NotImplementedError:('keys',)
    d.update([FailingIterNext()]):NotImplementedError:('next',)
    d.update([FailingIterNextN(1)]):NotImplementedError:('next N',)
    >>> Testing *Iter* using d.update(%s)
    d.update(FailingIter()):NotImplementedError:('iter',)
    d.update(FailingIterNext()):NotImplementedError:('next',)
    <<< Finished
    >>> Testing StringToChars using d.update({%s : 1})
    d.update({1 : 1}):TypeError:('expected str() or unicode() instance, but got int',)
    d.update({u"\0" : 1}):TypeError:('expected string without null bytes',)
    d.update({"\0" : 1}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d.update({"abcF" : {%s : 1}})
    d.update({"abcF" : {1 : 1}}):TypeError:('expected str() or unicode() instance, but got int',)
    d.update({"abcF" : {u"\0" : 1}}):TypeError:('expected string without null bytes',)
    d.update({"abcF" : {"\0" : 1}}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d.update({"abcF" : Mapping({%s : 1})})
    d.update({"abcF" : Mapping({1 : 1})}):TypeError:('expected str() or unicode() instance, but got int',)
    d.update({"abcF" : Mapping({u"\0" : 1})}):TypeError:('expected string without null bytes',)
    d.update({"abcF" : Mapping({"\0" : 1})}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using d.update({"abcF" : %s})
    d.update({"abcF" : FailingIter()}):TypeError:('unable to convert FailingIter to a Vim structure',)
    d.update({"abcF" : FailingIterNext()}):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using d.update({"abcF" : %s})
    d.update({"abcF" : None}):NOT FAILED
    d.update({"abcF" : {"": 1}}):ValueError:('empty keys are not allowed',)
    d.update({"abcF" : {u"": 1}}):ValueError:('empty keys are not allowed',)
    d.update({"abcF" : FailingMapping()}):NotImplementedError:('keys',)
    d.update({"abcF" : FailingMappingKey()}):NotImplementedError:('getitem:mappingkey',)
    d.update({"abcF" : FailingNumber()}):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing StringToChars using d.update(Mapping({%s : 1}))
    d.update(Mapping({1 : 1})):TypeError:('expected str() or unicode() instance, but got int',)
    d.update(Mapping({u"\0" : 1})):TypeError:('expected string without null bytes',)
    d.update(Mapping({"\0" : 1})):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d.update(Mapping({"abcG" : {%s : 1}}))
    d.update(Mapping({"abcG" : {1 : 1}})):TypeError:('expected str() or unicode() instance, but got int',)
    d.update(Mapping({"abcG" : {u"\0" : 1}})):TypeError:('expected string without null bytes',)
    d.update(Mapping({"abcG" : {"\0" : 1}})):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d.update(Mapping({"abcG" : Mapping({%s : 1})}))
    d.update(Mapping({"abcG" : Mapping({1 : 1})})):TypeError:('expected str() or unicode() instance, but got int',)
    d.update(Mapping({"abcG" : Mapping({u"\0" : 1})})):TypeError:('expected string without null bytes',)
    d.update(Mapping({"abcG" : Mapping({"\0" : 1})})):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using d.update(Mapping({"abcG" : %s}))
    d.update(Mapping({"abcG" : FailingIter()})):TypeError:('unable to convert FailingIter to a Vim structure',)
    d.update(Mapping({"abcG" : FailingIterNext()})):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using d.update(Mapping({"abcG" : %s}))
    d.update(Mapping({"abcG" : None})):NOT FAILED
    d.update(Mapping({"abcG" : {"": 1}})):ValueError:('empty keys are not allowed',)
    d.update(Mapping({"abcG" : {u"": 1}})):ValueError:('empty keys are not allowed',)
    d.update(Mapping({"abcG" : FailingMapping()})):NotImplementedError:('keys',)
    d.update(Mapping({"abcG" : FailingMappingKey()})):NotImplementedError:('getitem:mappingkey',)
    d.update(Mapping({"abcG" : FailingNumber()})):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing *Iter* using d.update(%s)
    d.update(FailingIter()):NotImplementedError:('iter',)
    d.update(FailingIterNext()):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using d.update(%s)
    d.update(None):TypeError:("'NoneType' object is not iterable",)
    d.update({"": 1}):ValueError:('empty keys are not allowed',)
    d.update({u"": 1}):ValueError:('empty keys are not allowed',)
    d.update(FailingMapping()):NotImplementedError:('keys',)
    d.update(FailingMappingKey()):NotImplementedError:('getitem:mappingkey',)
    d.update(FailingNumber()):TypeError:("'FailingNumber' object is not iterable",)
    <<< Finished
    >>> Testing StringToChars using d.update(((%s, 0),))
    d.update(((1, 0),)):TypeError:('expected str() or unicode() instance, but got int',)
    d.update(((u"\0", 0),)):TypeError:('expected string without null bytes',)
    d.update((("\0", 0),)):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d.update((("a", {%s : 1}),))
    d.update((("a", {1 : 1}),)):TypeError:('expected str() or unicode() instance, but got int',)
    d.update((("a", {u"\0" : 1}),)):TypeError:('expected string without null bytes',)
    d.update((("a", {"\0" : 1}),)):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d.update((("a", {"abcF" : {%s : 1}}),))
    d.update((("a", {"abcF" : {1 : 1}}),)):TypeError:('expected str() or unicode() instance, but got int',)
    d.update((("a", {"abcF" : {u"\0" : 1}}),)):TypeError:('expected string without null bytes',)
    d.update((("a", {"abcF" : {"\0" : 1}}),)):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d.update((("a", {"abcF" : Mapping({%s : 1})}),))
    d.update((("a", {"abcF" : Mapping({1 : 1})}),)):TypeError:('expected str() or unicode() instance, but got int',)
    d.update((("a", {"abcF" : Mapping({u"\0" : 1})}),)):TypeError:('expected string without null bytes',)
    d.update((("a", {"abcF" : Mapping({"\0" : 1})}),)):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using d.update((("a", {"abcF" : %s}),))
    d.update((("a", {"abcF" : FailingIter()}),)):TypeError:('unable to convert FailingIter to a Vim structure',)
    d.update((("a", {"abcF" : FailingIterNext()}),)):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using d.update((("a", {"abcF" : %s}),))
    d.update((("a", {"abcF" : None}),)):error:("failed to add key 'a' to dictionary",)
    d.update((("a", {"abcF" : {"": 1}}),)):ValueError:('empty keys are not allowed',)
    d.update((("a", {"abcF" : {u"": 1}}),)):ValueError:('empty keys are not allowed',)
    d.update((("a", {"abcF" : FailingMapping()}),)):NotImplementedError:('keys',)
    d.update((("a", {"abcF" : FailingMappingKey()}),)):NotImplementedError:('getitem:mappingkey',)
    d.update((("a", {"abcF" : FailingNumber()}),)):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing StringToChars using d.update((("a", Mapping({%s : 1})),))
    d.update((("a", Mapping({1 : 1})),)):TypeError:('expected str() or unicode() instance, but got int',)
    d.update((("a", Mapping({u"\0" : 1})),)):TypeError:('expected string without null bytes',)
    d.update((("a", Mapping({"\0" : 1})),)):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d.update((("a", Mapping({"abcG" : {%s : 1}})),))
    d.update((("a", Mapping({"abcG" : {1 : 1}})),)):TypeError:('expected str() or unicode() instance, but got int',)
    d.update((("a", Mapping({"abcG" : {u"\0" : 1}})),)):TypeError:('expected string without null bytes',)
    d.update((("a", Mapping({"abcG" : {"\0" : 1}})),)):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using d.update((("a", Mapping({"abcG" : Mapping({%s : 1})})),))
    d.update((("a", Mapping({"abcG" : Mapping({1 : 1})})),)):TypeError:('expected str() or unicode() instance, but got int',)
    d.update((("a", Mapping({"abcG" : Mapping({u"\0" : 1})})),)):TypeError:('expected string without null bytes',)
    d.update((("a", Mapping({"abcG" : Mapping({"\0" : 1})})),)):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using d.update((("a", Mapping({"abcG" : %s})),))
    d.update((("a", Mapping({"abcG" : FailingIter()})),)):TypeError:('unable to convert FailingIter to a Vim structure',)
    d.update((("a", Mapping({"abcG" : FailingIterNext()})),)):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using d.update((("a", Mapping({"abcG" : %s})),))
    d.update((("a", Mapping({"abcG" : None})),)):error:("failed to add key 'a' to dictionary",)
    d.update((("a", Mapping({"abcG" : {"": 1}})),)):ValueError:('empty keys are not allowed',)
    d.update((("a", Mapping({"abcG" : {u"": 1}})),)):ValueError:('empty keys are not allowed',)
    d.update((("a", Mapping({"abcG" : FailingMapping()})),)):NotImplementedError:('keys',)
    d.update((("a", Mapping({"abcG" : FailingMappingKey()})),)):NotImplementedError:('getitem:mappingkey',)
    d.update((("a", Mapping({"abcG" : FailingNumber()})),)):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing *Iter* using d.update((("a", %s),))
    d.update((("a", FailingIter()),)):TypeError:('unable to convert FailingIter to a Vim structure',)
    d.update((("a", FailingIterNext()),)):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using d.update((("a", %s),))
    d.update((("a", None),)):error:("failed to add key 'a' to dictionary",)
    d.update((("a", {"": 1}),)):ValueError:('empty keys are not allowed',)
    d.update((("a", {u"": 1}),)):ValueError:('empty keys are not allowed',)
    d.update((("a", FailingMapping()),)):NotImplementedError:('keys',)
    d.update((("a", FailingMappingKey()),)):NotImplementedError:('getitem:mappingkey',)
    d.update((("a", FailingNumber()),)):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >> DictionaryPopItem
    d.popitem(1, 2):TypeError:('popitem() takes no arguments (2 given)',)
    >> DictionaryHasKey
    d.has_key():TypeError:('has_key() takes exactly one argument (0 given)',)
    > List
    >> ListConstructor
    vim.List(1, 2):TypeError:('function takes at most 1 argument (2 given)',)
    vim.List(a=1):TypeError:('list constructor does not accept keyword arguments',)
    >>> Testing *Iter* using vim.List(%s)
    vim.List(FailingIter()):NotImplementedError:('iter',)
    vim.List(FailingIterNext()):NotImplementedError:('next',)
    <<< Finished
    >>> Testing StringToChars using vim.List([{%s : 1}])
    vim.List([{1 : 1}]):TypeError:('expected str() or unicode() instance, but got int',)
    vim.List([{u"\0" : 1}]):TypeError:('expected string without null bytes',)
    vim.List([{"\0" : 1}]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using vim.List([{"abcF" : {%s : 1}}])
    vim.List([{"abcF" : {1 : 1}}]):TypeError:('expected str() or unicode() instance, but got int',)
    vim.List([{"abcF" : {u"\0" : 1}}]):TypeError:('expected string without null bytes',)
    vim.List([{"abcF" : {"\0" : 1}}]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using vim.List([{"abcF" : Mapping({%s : 1})}])
    vim.List([{"abcF" : Mapping({1 : 1})}]):TypeError:('expected str() or unicode() instance, but got int',)
    vim.List([{"abcF" : Mapping({u"\0" : 1})}]):TypeError:('expected string without null bytes',)
    vim.List([{"abcF" : Mapping({"\0" : 1})}]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using vim.List([{"abcF" : %s}])
    vim.List([{"abcF" : FailingIter()}]):TypeError:('unable to convert FailingIter to a Vim structure',)
    vim.List([{"abcF" : FailingIterNext()}]):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using vim.List([{"abcF" : %s}])
    vim.List([{"abcF" : None}]):NOT FAILED
    vim.List([{"abcF" : {"": 1}}]):ValueError:('empty keys are not allowed',)
    vim.List([{"abcF" : {u"": 1}}]):ValueError:('empty keys are not allowed',)
    vim.List([{"abcF" : FailingMapping()}]):NotImplementedError:('keys',)
    vim.List([{"abcF" : FailingMappingKey()}]):NotImplementedError:('getitem:mappingkey',)
    vim.List([{"abcF" : FailingNumber()}]):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing StringToChars using vim.List([Mapping({%s : 1})])
    vim.List([Mapping({1 : 1})]):TypeError:('expected str() or unicode() instance, but got int',)
    vim.List([Mapping({u"\0" : 1})]):TypeError:('expected string without null bytes',)
    vim.List([Mapping({"\0" : 1})]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using vim.List([Mapping({"abcG" : {%s : 1}})])
    vim.List([Mapping({"abcG" : {1 : 1}})]):TypeError:('expected str() or unicode() instance, but got int',)
    vim.List([Mapping({"abcG" : {u"\0" : 1}})]):TypeError:('expected string without null bytes',)
    vim.List([Mapping({"abcG" : {"\0" : 1}})]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using vim.List([Mapping({"abcG" : Mapping({%s : 1})})])
    vim.List([Mapping({"abcG" : Mapping({1 : 1})})]):TypeError:('expected str() or unicode() instance, but got int',)
    vim.List([Mapping({"abcG" : Mapping({u"\0" : 1})})]):TypeError:('expected string without null bytes',)
    vim.List([Mapping({"abcG" : Mapping({"\0" : 1})})]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using vim.List([Mapping({"abcG" : %s})])
    vim.List([Mapping({"abcG" : FailingIter()})]):TypeError:('unable to convert FailingIter to a Vim structure',)
    vim.List([Mapping({"abcG" : FailingIterNext()})]):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using vim.List([Mapping({"abcG" : %s})])
    vim.List([Mapping({"abcG" : None})]):NOT FAILED
    vim.List([Mapping({"abcG" : {"": 1}})]):ValueError:('empty keys are not allowed',)
    vim.List([Mapping({"abcG" : {u"": 1}})]):ValueError:('empty keys are not allowed',)
    vim.List([Mapping({"abcG" : FailingMapping()})]):NotImplementedError:('keys',)
    vim.List([Mapping({"abcG" : FailingMappingKey()})]):NotImplementedError:('getitem:mappingkey',)
    vim.List([Mapping({"abcG" : FailingNumber()})]):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing *Iter* using vim.List([%s])
    vim.List([FailingIter()]):TypeError:('unable to convert FailingIter to a Vim structure',)
    vim.List([FailingIterNext()]):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using vim.List([%s])
    vim.List([None]):NOT FAILED
    vim.List([{"": 1}]):ValueError:('empty keys are not allowed',)
    vim.List([{u"": 1}]):ValueError:('empty keys are not allowed',)
    vim.List([FailingMapping()]):NotImplementedError:('keys',)
    vim.List([FailingMappingKey()]):NotImplementedError:('getitem:mappingkey',)
    vim.List([FailingNumber()]):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >> ListItem
    l[1000]:IndexError:('list index out of range',)
    >> ListAssItem
    ll[1] = 2:error:('list is locked',)
    l[1000] = 3:IndexError:('list index out of range',)
    >> ListAssSlice
    ll[1:100] = "abcJ":error:('list is locked',)
    >>> Testing *Iter* using l[:] = %s
    l[:] = FailingIter():NotImplementedError:('iter',)
    l[:] = FailingIterNext():NotImplementedError:('next',)
    <<< Finished
    nel[1:10:2]  = "abcK":ValueError:('attempt to assign sequence of size greater than 2 to extended slice',)
    ('a', 'b', 'c', 'O')
    nel[1:10:2]  = "a":ValueError:('attempt to assign sequence of size 1 to extended slice of size 2',)
    ('a', 'b', 'c', 'O')
    nel[1:1:-1]  = "a":ValueError:('attempt to assign sequence of size greater than 0 to extended slice',)
    ('a', 'b', 'c', 'O')
    nel[:] = FailingIterNextN(2):NotImplementedError:('next N',)
    ('a', 'b', 'c', 'O')
    >>> Testing StringToChars using l[:] = [{%s : 1}]
    l[:] = [{1 : 1}]:TypeError:('expected str() or unicode() instance, but got int',)
    l[:] = [{u"\0" : 1}]:TypeError:('expected string without null bytes',)
    l[:] = [{"\0" : 1}]:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using l[:] = [{"abcF" : {%s : 1}}]
    l[:] = [{"abcF" : {1 : 1}}]:TypeError:('expected str() or unicode() instance, but got int',)
    l[:] = [{"abcF" : {u"\0" : 1}}]:TypeError:('expected string without null bytes',)
    l[:] = [{"abcF" : {"\0" : 1}}]:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using l[:] = [{"abcF" : Mapping({%s : 1})}]
    l[:] = [{"abcF" : Mapping({1 : 1})}]:TypeError:('expected str() or unicode() instance, but got int',)
    l[:] = [{"abcF" : Mapping({u"\0" : 1})}]:TypeError:('expected string without null bytes',)
    l[:] = [{"abcF" : Mapping({"\0" : 1})}]:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using l[:] = [{"abcF" : %s}]
    l[:] = [{"abcF" : FailingIter()}]:TypeError:('unable to convert FailingIter to a Vim structure',)
    l[:] = [{"abcF" : FailingIterNext()}]:NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using l[:] = [{"abcF" : %s}]
    l[:] = [{"abcF" : None}]:NOT FAILED
    l[:] = [{"abcF" : {"": 1}}]:ValueError:('empty keys are not allowed',)
    l[:] = [{"abcF" : {u"": 1}}]:ValueError:('empty keys are not allowed',)
    l[:] = [{"abcF" : FailingMapping()}]:NotImplementedError:('keys',)
    l[:] = [{"abcF" : FailingMappingKey()}]:NotImplementedError:('getitem:mappingkey',)
    l[:] = [{"abcF" : FailingNumber()}]:TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing StringToChars using l[:] = [Mapping({%s : 1})]
    l[:] = [Mapping({1 : 1})]:TypeError:('expected str() or unicode() instance, but got int',)
    l[:] = [Mapping({u"\0" : 1})]:TypeError:('expected string without null bytes',)
    l[:] = [Mapping({"\0" : 1})]:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using l[:] = [Mapping({"abcG" : {%s : 1}})]
    l[:] = [Mapping({"abcG" : {1 : 1}})]:TypeError:('expected str() or unicode() instance, but got int',)
    l[:] = [Mapping({"abcG" : {u"\0" : 1}})]:TypeError:('expected string without null bytes',)
    l[:] = [Mapping({"abcG" : {"\0" : 1}})]:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using l[:] = [Mapping({"abcG" : Mapping({%s : 1})})]
    l[:] = [Mapping({"abcG" : Mapping({1 : 1})})]:TypeError:('expected str() or unicode() instance, but got int',)
    l[:] = [Mapping({"abcG" : Mapping({u"\0" : 1})})]:TypeError:('expected string without null bytes',)
    l[:] = [Mapping({"abcG" : Mapping({"\0" : 1})})]:TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using l[:] = [Mapping({"abcG" : %s})]
    l[:] = [Mapping({"abcG" : FailingIter()})]:TypeError:('unable to convert FailingIter to a Vim structure',)
    l[:] = [Mapping({"abcG" : FailingIterNext()})]:NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using l[:] = [Mapping({"abcG" : %s})]
    l[:] = [Mapping({"abcG" : None})]:NOT FAILED
    l[:] = [Mapping({"abcG" : {"": 1}})]:ValueError:('empty keys are not allowed',)
    l[:] = [Mapping({"abcG" : {u"": 1}})]:ValueError:('empty keys are not allowed',)
    l[:] = [Mapping({"abcG" : FailingMapping()})]:NotImplementedError:('keys',)
    l[:] = [Mapping({"abcG" : FailingMappingKey()})]:NotImplementedError:('getitem:mappingkey',)
    l[:] = [Mapping({"abcG" : FailingNumber()})]:TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing *Iter* using l[:] = [%s]
    l[:] = [FailingIter()]:TypeError:('unable to convert FailingIter to a Vim structure',)
    l[:] = [FailingIterNext()]:NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using l[:] = [%s]
    l[:] = [None]:NOT FAILED
    l[:] = [{"": 1}]:ValueError:('empty keys are not allowed',)
    l[:] = [{u"": 1}]:ValueError:('empty keys are not allowed',)
    l[:] = [FailingMapping()]:NotImplementedError:('keys',)
    l[:] = [FailingMappingKey()]:NotImplementedError:('getitem:mappingkey',)
    l[:] = [FailingNumber()]:TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >> ListConcatInPlace
    >>> Testing *Iter* using l.extend(%s)
    l.extend(FailingIter()):NotImplementedError:('iter',)
    l.extend(FailingIterNext()):NotImplementedError:('next',)
    <<< Finished
    >>> Testing StringToChars using l.extend([{%s : 1}])
    l.extend([{1 : 1}]):TypeError:('expected str() or unicode() instance, but got int',)
    l.extend([{u"\0" : 1}]):TypeError:('expected string without null bytes',)
    l.extend([{"\0" : 1}]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using l.extend([{"abcF" : {%s : 1}}])
    l.extend([{"abcF" : {1 : 1}}]):TypeError:('expected str() or unicode() instance, but got int',)
    l.extend([{"abcF" : {u"\0" : 1}}]):TypeError:('expected string without null bytes',)
    l.extend([{"abcF" : {"\0" : 1}}]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using l.extend([{"abcF" : Mapping({%s : 1})}])
    l.extend([{"abcF" : Mapping({1 : 1})}]):TypeError:('expected str() or unicode() instance, but got int',)
    l.extend([{"abcF" : Mapping({u"\0" : 1})}]):TypeError:('expected string without null bytes',)
    l.extend([{"abcF" : Mapping({"\0" : 1})}]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using l.extend([{"abcF" : %s}])
    l.extend([{"abcF" : FailingIter()}]):TypeError:('unable to convert FailingIter to a Vim structure',)
    l.extend([{"abcF" : FailingIterNext()}]):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using l.extend([{"abcF" : %s}])
    l.extend([{"abcF" : None}]):NOT FAILED
    l.extend([{"abcF" : {"": 1}}]):ValueError:('empty keys are not allowed',)
    l.extend([{"abcF" : {u"": 1}}]):ValueError:('empty keys are not allowed',)
    l.extend([{"abcF" : FailingMapping()}]):NotImplementedError:('keys',)
    l.extend([{"abcF" : FailingMappingKey()}]):NotImplementedError:('getitem:mappingkey',)
    l.extend([{"abcF" : FailingNumber()}]):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing StringToChars using l.extend([Mapping({%s : 1})])
    l.extend([Mapping({1 : 1})]):TypeError:('expected str() or unicode() instance, but got int',)
    l.extend([Mapping({u"\0" : 1})]):TypeError:('expected string without null bytes',)
    l.extend([Mapping({"\0" : 1})]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using l.extend([Mapping({"abcG" : {%s : 1}})])
    l.extend([Mapping({"abcG" : {1 : 1}})]):TypeError:('expected str() or unicode() instance, but got int',)
    l.extend([Mapping({"abcG" : {u"\0" : 1}})]):TypeError:('expected string without null bytes',)
    l.extend([Mapping({"abcG" : {"\0" : 1}})]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using l.extend([Mapping({"abcG" : Mapping({%s : 1})})])
    l.extend([Mapping({"abcG" : Mapping({1 : 1})})]):TypeError:('expected str() or unicode() instance, but got int',)
    l.extend([Mapping({"abcG" : Mapping({u"\0" : 1})})]):TypeError:('expected string without null bytes',)
    l.extend([Mapping({"abcG" : Mapping({"\0" : 1})})]):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using l.extend([Mapping({"abcG" : %s})])
    l.extend([Mapping({"abcG" : FailingIter()})]):TypeError:('unable to convert FailingIter to a Vim structure',)
    l.extend([Mapping({"abcG" : FailingIterNext()})]):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using l.extend([Mapping({"abcG" : %s})])
    l.extend([Mapping({"abcG" : None})]):NOT FAILED
    l.extend([Mapping({"abcG" : {"": 1}})]):ValueError:('empty keys are not allowed',)
    l.extend([Mapping({"abcG" : {u"": 1}})]):ValueError:('empty keys are not allowed',)
    l.extend([Mapping({"abcG" : FailingMapping()})]):NotImplementedError:('keys',)
    l.extend([Mapping({"abcG" : FailingMappingKey()})]):NotImplementedError:('getitem:mappingkey',)
    l.extend([Mapping({"abcG" : FailingNumber()})]):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing *Iter* using l.extend([%s])
    l.extend([FailingIter()]):TypeError:('unable to convert FailingIter to a Vim structure',)
    l.extend([FailingIterNext()]):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using l.extend([%s])
    l.extend([None]):NOT FAILED
    l.extend([{"": 1}]):ValueError:('empty keys are not allowed',)
    l.extend([{u"": 1}]):ValueError:('empty keys are not allowed',)
    l.extend([FailingMapping()]):NotImplementedError:('keys',)
    l.extend([FailingMappingKey()]):NotImplementedError:('getitem:mappingkey',)
    l.extend([FailingNumber()]):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >> ListSetattr
    del l.locked:AttributeError:('cannot delete vim.List attributes',)
    l.locked = FailingTrue():NotImplementedError:('bool',)
    l.xxx = True:AttributeError:('cannot set attribute xxx',)
    > Function
    >> FunctionConstructor
    >>> FunctionConstructor
    vim.Function("123"):ValueError:('unnamed function 123 does not exist',)
    vim.Function("xxx_non_existent_function_xxx"):ValueError:('function xxx_non_existent_function_xxx does not exist',)
    vim.Function("xxx#non#existent#function#xxx"):NOT FAILED
    vim.Function("xxx_non_existent_function_xxx2", args=[]):ValueError:('function xxx_non_existent_function_xxx2 does not exist',)
    vim.Function("xxx_non_existent_function_xxx3", self={}):ValueError:('function xxx_non_existent_function_xxx3 does not exist',)
    vim.Function("xxx_non_existent_function_xxx4", args=[], self={}):ValueError:('function xxx_non_existent_function_xxx4 does not exist',)
    >>> FunctionNew
    vim.Function("tr", self="abcFuncSelf"):TypeError:('unable to convert str to a Vim dictionary',)
    vim.Function("tr", args=427423):TypeError:('unable to convert int to a Vim list',)
    vim.Function("tr", self="abcFuncSelf2", args="abcFuncArgs2"):TypeError:('unable to convert str to a Vim dictionary',)
    vim.Function(self="abcFuncSelf2", args="abcFuncArgs2"):TypeError:('unable to convert str to a Vim dictionary',)
    vim.Function("tr", "", self="abcFuncSelf2", args="abcFuncArgs2"):TypeError:('unable to convert str to a Vim dictionary',)
    vim.Function("tr", ""):TypeError:('function takes exactly 1 argument (2 given)',)
    >> FunctionCall
    >>> Testing StringToChars using f({%s : 1})
    f({1 : 1}):TypeError:('expected str() or unicode() instance, but got int',)
    f({u"\0" : 1}):TypeError:('expected string without null bytes',)
    f({"\0" : 1}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using f({"abcF" : {%s : 1}})
    f({"abcF" : {1 : 1}}):TypeError:('expected str() or unicode() instance, but got int',)
    f({"abcF" : {u"\0" : 1}}):TypeError:('expected string without null bytes',)
    f({"abcF" : {"\0" : 1}}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using f({"abcF" : Mapping({%s : 1})})
    f({"abcF" : Mapping({1 : 1})}):TypeError:('expected str() or unicode() instance, but got int',)
    f({"abcF" : Mapping({u"\0" : 1})}):TypeError:('expected string without null bytes',)
    f({"abcF" : Mapping({"\0" : 1})}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using f({"abcF" : %s})
    f({"abcF" : FailingIter()}):TypeError:('unable to convert FailingIter to a Vim structure',)
    f({"abcF" : FailingIterNext()}):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using f({"abcF" : %s})
    f({"abcF" : None}):NOT FAILED
    f({"abcF" : {"": 1}}):ValueError:('empty keys are not allowed',)
    f({"abcF" : {u"": 1}}):ValueError:('empty keys are not allowed',)
    f({"abcF" : FailingMapping()}):NotImplementedError:('keys',)
    f({"abcF" : FailingMappingKey()}):NotImplementedError:('getitem:mappingkey',)
    f({"abcF" : FailingNumber()}):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing StringToChars using f(Mapping({%s : 1}))
    f(Mapping({1 : 1})):TypeError:('expected str() or unicode() instance, but got int',)
    f(Mapping({u"\0" : 1})):TypeError:('expected string without null bytes',)
    f(Mapping({"\0" : 1})):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using f(Mapping({"abcG" : {%s : 1}}))
    f(Mapping({"abcG" : {1 : 1}})):TypeError:('expected str() or unicode() instance, but got int',)
    f(Mapping({"abcG" : {u"\0" : 1}})):TypeError:('expected string without null bytes',)
    f(Mapping({"abcG" : {"\0" : 1}})):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using f(Mapping({"abcG" : Mapping({%s : 1})}))
    f(Mapping({"abcG" : Mapping({1 : 1})})):TypeError:('expected str() or unicode() instance, but got int',)
    f(Mapping({"abcG" : Mapping({u"\0" : 1})})):TypeError:('expected string without null bytes',)
    f(Mapping({"abcG" : Mapping({"\0" : 1})})):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using f(Mapping({"abcG" : %s}))
    f(Mapping({"abcG" : FailingIter()})):TypeError:('unable to convert FailingIter to a Vim structure',)
    f(Mapping({"abcG" : FailingIterNext()})):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using f(Mapping({"abcG" : %s}))
    f(Mapping({"abcG" : None})):NOT FAILED
    f(Mapping({"abcG" : {"": 1}})):ValueError:('empty keys are not allowed',)
    f(Mapping({"abcG" : {u"": 1}})):ValueError:('empty keys are not allowed',)
    f(Mapping({"abcG" : FailingMapping()})):NotImplementedError:('keys',)
    f(Mapping({"abcG" : FailingMappingKey()})):NotImplementedError:('getitem:mappingkey',)
    f(Mapping({"abcG" : FailingNumber()})):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing *Iter* using f(%s)
    f(FailingIter()):TypeError:('unable to convert FailingIter to a Vim structure',)
    f(FailingIterNext()):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using f(%s)
    f(None):NOT FAILED
    f({"": 1}):ValueError:('empty keys are not allowed',)
    f({u"": 1}):ValueError:('empty keys are not allowed',)
    f(FailingMapping()):NotImplementedError:('keys',)
    f(FailingMappingKey()):NotImplementedError:('getitem:mappingkey',)
    f(FailingNumber()):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing StringToChars using fd(self={%s : 1})
    fd(self={1 : 1}):TypeError:('expected str() or unicode() instance, but got int',)
    fd(self={u"\0" : 1}):TypeError:('expected string without null bytes',)
    fd(self={"\0" : 1}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using fd(self={"abcF" : {%s : 1}})
    fd(self={"abcF" : {1 : 1}}):TypeError:('expected str() or unicode() instance, but got int',)
    fd(self={"abcF" : {u"\0" : 1}}):TypeError:('expected string without null bytes',)
    fd(self={"abcF" : {"\0" : 1}}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using fd(self={"abcF" : Mapping({%s : 1})})
    fd(self={"abcF" : Mapping({1 : 1})}):TypeError:('expected str() or unicode() instance, but got int',)
    fd(self={"abcF" : Mapping({u"\0" : 1})}):TypeError:('expected string without null bytes',)
    fd(self={"abcF" : Mapping({"\0" : 1})}):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using fd(self={"abcF" : %s})
    fd(self={"abcF" : FailingIter()}):TypeError:('unable to convert FailingIter to a Vim structure',)
    fd(self={"abcF" : FailingIterNext()}):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using fd(self={"abcF" : %s})
    fd(self={"abcF" : None}):NOT FAILED
    fd(self={"abcF" : {"": 1}}):ValueError:('empty keys are not allowed',)
    fd(self={"abcF" : {u"": 1}}):ValueError:('empty keys are not allowed',)
    fd(self={"abcF" : FailingMapping()}):NotImplementedError:('keys',)
    fd(self={"abcF" : FailingMappingKey()}):NotImplementedError:('getitem:mappingkey',)
    fd(self={"abcF" : FailingNumber()}):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing StringToChars using fd(self=Mapping({%s : 1}))
    fd(self=Mapping({1 : 1})):TypeError:('expected str() or unicode() instance, but got int',)
    fd(self=Mapping({u"\0" : 1})):TypeError:('expected string without null bytes',)
    fd(self=Mapping({"\0" : 1})):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using fd(self=Mapping({"abcG" : {%s : 1}}))
    fd(self=Mapping({"abcG" : {1 : 1}})):TypeError:('expected str() or unicode() instance, but got int',)
    fd(self=Mapping({"abcG" : {u"\0" : 1}})):TypeError:('expected string without null bytes',)
    fd(self=Mapping({"abcG" : {"\0" : 1}})):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing StringToChars using fd(self=Mapping({"abcG" : Mapping({%s : 1})}))
    fd(self=Mapping({"abcG" : Mapping({1 : 1})})):TypeError:('expected str() or unicode() instance, but got int',)
    fd(self=Mapping({"abcG" : Mapping({u"\0" : 1})})):TypeError:('expected string without null bytes',)
    fd(self=Mapping({"abcG" : Mapping({"\0" : 1})})):TypeError:('expected string without null bytes',)
    <<< Finished
    >>> Testing *Iter* using fd(self=Mapping({"abcG" : %s}))
    fd(self=Mapping({"abcG" : FailingIter()})):TypeError:('unable to convert FailingIter to a Vim structure',)
    fd(self=Mapping({"abcG" : FailingIterNext()})):NotImplementedError:('next',)
    <<< Finished
    >>> Testing ConvertFromPyObject using fd(self=Mapping({"abcG" : %s}))
    fd(self=Mapping({"abcG" : None})):NOT FAILED
    fd(self=Mapping({"abcG" : {"": 1}})):ValueError:('empty keys are not allowed',)
    fd(self=Mapping({"abcG" : {u"": 1}})):ValueError:('empty keys are not allowed',)
    fd(self=Mapping({"abcG" : FailingMapping()})):NotImplementedError:('keys',)
    fd(self=Mapping({"abcG" : FailingMappingKey()})):NotImplementedError:('getitem:mappingkey',)
    fd(self=Mapping({"abcG" : FailingNumber()})):TypeError:('long() argument must be a string or a number',)
    <<< Finished
    >>> Testing *Iter* using fd(self=%s)
    fd(self=FailingIter()):TypeError:('unable to convert FailingIter to a Vim dictionary',)
    fd(self=FailingIterNext()):TypeError:('unable to convert FailingIterNext to a Vim dictionary',)
    <<< Finished
    >>> Testing ConvertFromPyObject using fd(self=%s)
    fd(self=None):TypeError:('unable to convert NoneType to a Vim dictionary',)
    fd(self={"": 1}):ValueError:('empty keys are not allowed',)
    fd(self={u"": 1}):ValueError:('empty keys are not allowed',)
    fd(self=FailingMapping()):NotImplementedError:('keys',)
    fd(self=FailingMappingKey()):NotImplementedError:('getitem:mappingkey',)
    fd(self=FailingNumber()):TypeError:('unable to convert FailingNumber to a Vim dictionary',)
    <<< Finished
    >>> Testing ConvertFromPyMapping using fd(self=%s)
    fd(self=[]):TypeError:('unable to convert list to a Vim dictionary',)
    <<< Finished
    > TabPage
    >> TabPageAttr
    vim.current.tabpage.xxx:AttributeError:('xxx',)
    > TabList
    >> TabListItem
    vim.tabpages[1000]:IndexError:('no such tab page',)
    > Window
    >> WindowAttr
    vim.current.window.xxx:AttributeError:('xxx',)
    >> WindowSetattr
    vim.current.window.buffer = 0:TypeError:('readonly attribute: buffer',)
    vim.current.window.cursor = (100000000, 100000000):error:('cursor position outside buffer',)
    vim.current.window.cursor = True:TypeError:('argument must be 2-item sequence, not bool',)
    >>> Testing NumberToLong using vim.current.window.height = %s
    vim.current.window.height = []:TypeError:('expected int(), long() or something supporting coercing to long(), but got list',)
    vim.current.window.height = None:TypeError:('expected int(), long() or something supporting coercing to long(), but got NoneType',)
    vim.current.window.height = -1:ValueError:('number must be greater or equal to zero',)
    <<< Finished
    >>> Testing NumberToLong using vim.current.window.width = %s
    vim.current.window.width = []:TypeError:('expected int(), long() or something supporting coercing to long(), but got list',)
    vim.current.window.width = None:TypeError:('expected int(), long() or something supporting coercing to long(), but got NoneType',)
    vim.current.window.width = -1:ValueError:('number must be greater or equal to zero',)
    <<< Finished
    vim.current.window.xxxxxx = True:AttributeError:('xxxxxx',)
    > WinList
    >> WinListItem
    vim.windows[1000]:IndexError:('no such window',)
    > Buffer
    >> StringToLine (indirect)
    vim.current.buffer[0] = "\na":error:('string cannot contain newlines',)
    vim.current.buffer[0] = u"\na":error:('string cannot contain newlines',)
    >> SetBufferLine (indirect)
    vim.current.buffer[0] = True:TypeError:('bad argument type for built-in operation',)
    >> SetBufferLineList (indirect)
    vim.current.buffer[:] = True:TypeError:('bad argument type for built-in operation',)
    vim.current.buffer[:] = ["\na", "bc"]:error:('string cannot contain newlines',)
    >> InsertBufferLines (indirect)
    vim.current.buffer.append(None):TypeError:('bad argument type for built-in operation',)
    vim.current.buffer.append(["\na", "bc"]):error:('string cannot contain newlines',)
    vim.current.buffer.append("\nbc"):error:('string cannot contain newlines',)
    >> RBItem
    vim.current.buffer[100000000]:IndexError:('line number out of range',)
    >> RBAsItem
    vim.current.buffer[100000000] = "":IndexError:('line number out of range',)
    >> BufferAttr
    vim.current.buffer.xxx:AttributeError:('xxx',)
    >> BufferSetattr
    vim.current.buffer.name = True:TypeError:('expected str() or unicode() instance, but got bool',)
    vim.current.buffer.xxx = True:AttributeError:('xxx',)
    >> BufferMark
    vim.current.buffer.mark(0):TypeError:('expected str() or unicode() instance, but got int',)
    vim.current.buffer.mark("abcM"):ValueError:('mark name must be a single character',)
    vim.current.buffer.mark("!"):error:('invalid mark name',)
    >> BufferRange
    vim.current.buffer.range(1, 2, 3):TypeError:('function takes exactly 2 arguments (3 given)',)
    > BufMap
    >> BufMapItem
    vim.buffers[100000000]:KeyError:(100000000,)
    >>> Testing NumberToLong using vim.buffers[%s]
    vim.buffers[[]]:TypeError:('expected int(), long() or something supporting coercing to long(), but got list',)
    vim.buffers[None]:TypeError:('expected int(), long() or something supporting coercing to long(), but got NoneType',)
    vim.buffers[-1]:ValueError:('number must be greater than zero',)
    vim.buffers[0]:ValueError:('number must be greater than zero',)
    <<< Finished
    > Current
    >> CurrentGetattr
    vim.current.xxx:AttributeError:('xxx',)
    >> CurrentSetattr
    vim.current.line = True:TypeError:('bad argument type for built-in operation',)
    vim.current.buffer = True:TypeError:('expected vim.Buffer object, but got bool',)
    vim.current.window = True:TypeError:('expected vim.Window object, but got bool',)
    vim.current.tabpage = True:TypeError:('expected vim.TabPage object, but got bool',)
    vim.current.xxx = True:AttributeError:('xxx',)
  END

  call assert_equal(expected, getline(2, '$'))
  close!
endfunc

" Test import
func Test_python_import()
  new
  py cb = vim.current.buffer

  py << trim EOF
    sys.path.insert(0, os.path.join(os.getcwd(), 'python_before'))
    sys.path.append(os.path.join(os.getcwd(), 'python_after'))
    vim.options['rtp'] = os.getcwd().replace(',', '\\,').replace('\\', '\\\\')
    l = []
    def callback(path):
        l.append(path[-len('/testdir'):].replace(os.path.sep, '/'))
    vim.foreach_rtp(callback)
    cb.append(repr(l))
    del l
    def callback(path):
        return path[-len('/testdir'):].replace(os.path.sep, '/')
    cb.append(repr(vim.foreach_rtp(callback)))
    del callback
    from module import dir as d
    from modulex import ddir
    cb.append(d + ',' + ddir)
    import before
    cb.append(before.dir)
    import after
    cb.append(after.dir)
    import topmodule as tm
    import topmodule.submodule as tms
    import topmodule.submodule.subsubmodule.subsubsubmodule as tmsss
    cb.append(tm.__file__.replace('.pyc', '.py').replace(os.path.sep, '/')[-len('modulex/topmodule/__init__.py'):])
    cb.append(tms.__file__.replace('.pyc', '.py').replace(os.path.sep, '/')[-len('modulex/topmodule/submodule/__init__.py'):])
    cb.append(tmsss.__file__.replace('.pyc', '.py').replace(os.path.sep, '/')[-len('modulex/topmodule/submodule/subsubmodule/subsubsubmodule.py'):])

    del before
    del after
    del d
    del ddir
    del tm
    del tms
    del tmsss
  EOF

  let expected =<< trim END
    ['/testdir']
    '/testdir'
    2,xx
    before
    after
    pythonx/topmodule/__init__.py
    pythonx/topmodule/submodule/__init__.py
    pythonx/topmodule/submodule/subsubmodule/subsubsubmodule.py
  END
  call assert_equal(expected, getline(2, '$'))
  close!

  " Try to import a non-existing module with a dot (.)
  call AssertException(['py import a.b.c'], 'ImportError:')
endfunc

" Test exceptions
func Test_python_exception()
  func Exe(e)
    execute a:e
  endfunc

  new
  py cb = vim.current.buffer

  py << trim EOF
    Exe = vim.bindeval('function("Exe")')
    ee('vim.command("throw \'abcN\'")')
    ee('Exe("throw \'def\'")')
    ee('vim.eval("Exe(\'throw \'\'ghi\'\'\')")')
    ee('vim.eval("Exe(\'echoerr \'\'jkl\'\'\')")')
    ee('vim.eval("Exe(\'xxx_non_existent_command_xxx\')")')
    ee('vim.eval("xxx_unknown_function_xxx()")')
    ee('vim.bindeval("Exe(\'xxx_non_existent_command_xxx\')")')
    del Exe
  EOF
  delfunction Exe

  let expected =<< trim END
    vim.command("throw 'abcN'"):error:('abcN',)
    Exe("throw 'def'"):error:('def',)
    vim.eval("Exe('throw ''ghi''')"):error:('ghi',)
    vim.eval("Exe('echoerr ''jkl''')"):error:('Vim(echoerr):jkl',)
    vim.eval("Exe('xxx_non_existent_command_xxx')"):error:('Vim:E492: Not an editor command: xxx_non_existent_command_xxx',)
    vim.eval("xxx_unknown_function_xxx()"):error:('Vim:E117: Unknown function: xxx_unknown_function_xxx',)
    vim.bindeval("Exe('xxx_non_existent_command_xxx')"):error:('Vim:E492: Not an editor command: xxx_non_existent_command_xxx',)
  END
  call assert_equal(expected, getline(2, '$'))
  close!
endfunc

" Regression: interrupting vim.command propagates to next vim.command
func Test_python_keyboard_interrupt()
  new
  py cb = vim.current.buffer
  py << trim EOF
    def test_keyboard_interrupt():
        try:
            vim.command('while 1 | endwhile')
        except KeyboardInterrupt:
            cb.append('Caught KeyboardInterrupt')
        except Exception:
            cb.append('!!!!!!!! Caught exception: ' + emsg(sys.exc_info()))
        else:
            cb.append('!!!!!!!! No exception')
        try:
            vim.command('$ put =\'Running :put\'')
        except KeyboardInterrupt:
            cb.append('!!!!!!!! Caught KeyboardInterrupt')
        except Exception:
            cb.append('!!!!!!!! Caught exception: ' + emsg(sys.exc_info()))
        else:
            cb.append('No exception')
  EOF

  debuggreedy
  call inputsave()
  call feedkeys("s\ns\ns\ns\nq\n")
  redir => output
  debug silent! py test_keyboard_interrupt()
  redir END
  0 debuggreedy
  call inputrestore()
  py del test_keyboard_interrupt

  let expected =<< trim END
    Caught KeyboardInterrupt
    Running :put
    No exception
  END
  call assert_equal(expected, getline(2, '$'))
  call assert_equal('', output)
  close!
endfunc

func Test_python_non_utf8_string()
  smap <Esc>@ <A-@>
  python vim.command('redir => _tmp_smaps | smap | redir END')
  python vim.eval('_tmp_smaps').splitlines()
  sunmap <Esc>@
endfunc

" vim: shiftwidth=2 sts=2 expandtab
