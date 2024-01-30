" Tests for Lua.

source check.vim

" This test also works without the lua feature.
func Test_skip_lua()
  if 0
    lua print("Not executed")
  endif
endfunc

CheckFeature lua

" Depending on the lua version, the error messages are different.
let [s:major, s:minor, s:patch] = luaeval('vim.lua_version')->split('\.')->map({-> str2nr(v:val)})
let s:lua_53_or_later = 0
let s:lua_543 = 0
if (s:major == 5 && s:minor >= 3) || s:major > 5
  let s:lua_53_or_later = 1
  if s:major == 5 && s:minor == 4 && s:patch == 3
    let s:lua_543 = 1
  endif
endif

func TearDown()
  " Run garbage collection after each test to exercise luaV_setref().
  call test_garbagecollect_now()
endfunc

" Check that switching to another buffer does not trigger ml_get error.
func Test_lua_luado_change_buffer()
  new

  let wincount = winnr('$')
  call setline(1, ['one', 'two', 'three'])
  luado vim.command("new")
  call assert_equal(wincount + 1, winnr('$'))

  %bwipe!
endfunc

" Check that :luado deleting lines does not trigger ml_get error.
func Test_lua_luado_delete_lines()
  new

  call setline(1, ['one', 'two', 'three'])
  luado vim.command("%d_")
  call assert_equal([''], getline(1, '$'))

  call setline(1, ['one', 'two', 'three'])
  luado vim.command("1,2d_")
  call assert_equal(['three'], getline(1, '$'))

  call setline(1, ['one', 'two', 'three'])
  luado vim.command("2,3d_"); return "REPLACED"
  call assert_equal(['REPLACED'], getline(1, '$'))

  call setline(1, ['one', 'two', 'three'])
  2,3luado vim.command("1,2d_"); return "REPLACED"
  call assert_equal(['three'], getline(1, '$'))

  bwipe!
endfunc

func Test_lua_luado()
  new
  call setline(1, ['one', 'two'])
  luado return(linenr)
  call assert_equal(['1', '2'], getline(1, '$'))
  close!

  " Error cases
  call assert_fails('luado string.format()',
        \ "[string \"vim chunk\"]:1: bad argument #1 to 'format' (string expected, got no value)")
  if s:lua_543
    let msg = "[string \"vim chunk\"]:1: global 'func' is not callable (a nil value)"
  elseif s:lua_53_or_later
    let msg = "[string \"vim chunk\"]:1: attempt to call a nil value (global 'func')"
  else
    let msg = "[string \"vim chunk\"]:1: attempt to call global 'func' (a nil value)"
  endif
  call assert_fails('luado func()', msg)
  call assert_fails('luado error("failed")', "[string \"vim chunk\"]:1: failed")
endfunc

" Test vim.eval()
func Test_lua_eval()
  " lua.eval with a number
  lua v = vim.eval('123')
  call assert_equal('number', luaeval('vim.type(v)'))
  call assert_equal(123, luaeval('v'))

  " lua.eval with a string
  lua v = vim.eval('"abc"')
  call assert_equal('string', 'vim.type(v)'->luaeval())
  call assert_equal('abc', luaeval('v'))

  " lua.eval with a list
  lua v = vim.eval("['a']")
  call assert_equal('list', luaeval('vim.type(v)'))
  call assert_equal(['a'], luaeval('v'))

  " lua.eval with a dict
  lua v = vim.eval("{'a':'b'}")
  call assert_equal('dict', luaeval('vim.type(v)'))
  call assert_equal({'a':'b'}, luaeval('v'))

  " lua.eval with a blob
  lua v = vim.eval("0z00112233.deadbeef")
  call assert_equal('blob', luaeval('vim.type(v)'))
  call assert_equal(0z00112233.deadbeef, luaeval('v'))

  " lua.eval with a float
  lua v = vim.eval('3.14')
  call assert_equal('number', luaeval('vim.type(v)'))
  call assert_equal(3.14, luaeval('v'))

  " lua.eval with a bool
  lua v = vim.eval('v:true')
  call assert_equal('number', luaeval('vim.type(v)'))
  call assert_equal(1, luaeval('v'))
  lua v = vim.eval('v:false')
  call assert_equal('number', luaeval('vim.type(v)'))
  call assert_equal(0, luaeval('v'))

  " lua.eval with a null
  lua v = vim.eval('v:null')
  call assert_equal('nil', luaeval('vim.type(v)'))
  call assert_equal(v:null, luaeval('v'))

  call assert_fails('lua v = vim.eval(nil)',
        \ "[string \"vim chunk\"]:1: bad argument #1 to 'eval' (string expected, got nil)")
  call assert_fails('lua v = vim.eval(true)',
        \ "[string \"vim chunk\"]:1: bad argument #1 to 'eval' (string expected, got boolean)")
  call assert_fails('lua v = vim.eval({})',
        \ "[string \"vim chunk\"]:1: bad argument #1 to 'eval' (string expected, got table)")
  call assert_fails('lua v = vim.eval(print)',
        \ "[string \"vim chunk\"]:1: bad argument #1 to 'eval' (string expected, got function)")
  call assert_fails('lua v = vim.eval(vim.buffer())',
        \ "[string \"vim chunk\"]:1: bad argument #1 to 'eval' (string expected, got userdata)")

  lua v = nil
endfunc

" Test luaeval() with lambda
func Test_luaeval_with_lambda()
  lua function hello_luaeval_lambda(a, cb) return a .. cb() end
  call assert_equal('helloworld',
        \ luaeval('hello_luaeval_lambda(_A[1], _A[2])',
        \         ['hello', {->'world'}]))
  lua hello_luaeval_lambda = nil
endfunc

" Test vim.window()
func Test_lua_window()
  e Xfoo2
  new Xfoo1

  " Window 1 (top window) contains Xfoo1
  " Window 2 (bottom window) contains Xfoo2
  call assert_equal('Xfoo1', luaeval('vim.window(1):buffer().name'))
  call assert_equal('Xfoo2', luaeval('vim.window(2):buffer().name'))

  " Window 3 does not exist so vim.window(3) should return nil
  call assert_equal('nil', luaeval('tostring(vim.window(3))'))

  if s:lua_543
    let msg = "[string \"luaeval\"]:1: field 'xyz' is not callable (a nil value)"
  elseif s:lua_53_or_later
    let msg = "[string \"luaeval\"]:1: attempt to call a nil value (field 'xyz')"
  else
    let msg = "[string \"luaeval\"]:1: attempt to call field 'xyz' (a nil value)"
  endif
  call assert_fails("let n = luaeval('vim.window().xyz()')", msg)
  call assert_fails('lua vim.window().xyz = 1',
        \ "[string \"vim chunk\"]:1: invalid window property: `xyz'")

  %bwipe!
endfunc

" Test vim.window().height
func Test_lua_window_height()
  new
  lua vim.window().height = 2
  call assert_equal(2, winheight(0))
  lua vim.window().height = vim.window().height + 1
  call assert_equal(3, winheight(0))
  bwipe!
endfunc

" Test vim.window().width
func Test_lua_window_width()
  vert new
  lua vim.window().width = 2
  call assert_equal(2, winwidth(0))
  lua vim.window().width = vim.window().width + 1
  call assert_equal(3, winwidth(0))
  bwipe!
endfunc

" Test vim.window().line and vim.window.col
func Test_lua_window_line_col()
  new
  call setline(1, ['line1', 'line2', 'line3'])
  lua vim.window().line = 2
  lua vim.window().col = 4
  call assert_equal([0, 2, 4, 0], getpos('.'))
  lua vim.window().line = vim.window().line + 1
  lua vim.window().col = vim.window().col - 1
  call assert_equal([0, 3, 3, 0], getpos('.'))

  call assert_fails('lua vim.window().line = 10',
        \           '[string "vim chunk"]:1: line out of range')
  bwipe!
endfunc

" Test vim.call
func Test_lua_call()
  call assert_equal(has('lua'), luaeval('vim.call("has", "lua")'))
  call assert_equal(printf("Hello %s", "vim"), luaeval('vim.call("printf", "Hello %s", "vim")'))

  " Error cases
  call assert_fails("call luaeval('vim.call(\"min\", 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21)')",
        \ s:lua_53_or_later
        \ ? '[string "luaeval"]:1: Function called with too many arguments'
        \ : 'Function called with too many arguments')
  lua co = coroutine.create(function () print("hi") end)
  call assert_fails("call luaeval('vim.call(\"type\", co)')",
        \ s:lua_53_or_later
        \ ? '[string "luaeval"]:1: lua: cannot convert value'
        \ : 'lua: cannot convert value')
  lua co = nil
  call assert_fails("call luaeval('vim.call(\"abc\")')",
        \ ['E117:', s:lua_53_or_later ? '\[string "luaeval"]:1: lua: call_vim_function failed'
        \                             : 'lua: call_vim_function failed'])
endfunc

" Test vim.fn.*
func Test_lua_fn()
  call assert_equal(has('lua'), luaeval('vim.fn.has("lua")'))
  call assert_equal(printf("Hello %s", "vim"), luaeval('vim.fn.printf("Hello %s", "vim")'))
endfunc

" Test setting the current window
func Test_lua_window_set_current()
  new Xfoo1
  lua w1 = vim.window()
  new Xfoo2
  lua w2 = vim.window()

  call assert_equal('Xfoo2', bufname('%'))
  lua w1()
  call assert_equal('Xfoo1', bufname('%'))
  lua w2()
  call assert_equal('Xfoo2', bufname('%'))

  lua w1, w2 = nil
  %bwipe!
endfunc

" Test vim.window().buffer
func Test_lua_window_buffer()
  new Xfoo1
  lua w1 = vim.window()
  lua b1 = w1.buffer()
  new Xfoo2
  lua w2 = vim.window()
  lua b2 = w2.buffer()

  lua b1()
  call assert_equal('Xfoo1', bufname('%'))
  lua b2()
  call assert_equal('Xfoo2', bufname('%'))

  lua b1, b2, w1, w2 = nil
  %bwipe!
endfunc

" Test vim.window():previous() and vim.window():next()
func Test_lua_window_next_previous()
  new Xfoo1
  new Xfoo2
  new Xfoo3
  wincmd j

  call assert_equal('Xfoo2', luaeval('vim.window().buffer().name'))
  call assert_equal('Xfoo1', luaeval('vim.window():next():buffer().name'))
  call assert_equal('Xfoo3', luaeval('vim.window():previous():buffer().name'))

  %bwipe!
endfunc

" Test vim.window():isvalid()
func Test_lua_window_isvalid()
  new Xfoo
  lua w = vim.window()
  call assert_true(luaeval('w:isvalid()'))

  " FIXME: how to test the case when isvalid() returns v:false?
  " isvalid() gives errors when the window is deleted. Is it a bug?

  lua w = nil
  bwipe!
endfunc

" Test vim.buffer() with and without argument
func Test_lua_buffer()
  new Xfoo1
  let bn1 = bufnr('%')
  new Xfoo2
  let bn2 = bufnr('%')

  " Test vim.buffer() without argument.
  call assert_equal('Xfoo2', luaeval("vim.buffer().name"))

  " Test vim.buffer() with string argument.
  call assert_equal('Xfoo1', luaeval("vim.buffer('Xfoo1').name"))
  call assert_equal('Xfoo2', luaeval("vim.buffer('Xfoo2').name"))

  " Test vim.buffer() with integer argument.
  call assert_equal('Xfoo1', luaeval("vim.buffer(" . bn1 . ").name"))
  call assert_equal('Xfoo2', luaeval("vim.buffer(" . bn2 . ").name"))

  lua bn1, bn2 = nil
  %bwipe!
endfunc

" Test vim.buffer().name and vim.buffer().fname
func Test_lua_buffer_name()
  new
  call assert_equal('', luaeval('vim.buffer().name'))
  call assert_equal('', luaeval('vim.buffer().fname'))
  bwipe!

  new Xfoo
  call assert_equal('Xfoo', luaeval('vim.buffer().name'))
  call assert_equal(expand('%:p'), luaeval('vim.buffer().fname'))
  bwipe!
endfunc

" Test vim.buffer().number
func Test_lua_buffer_number()
  " All numbers in Lua are floating points number (no integers).
  call assert_equal(bufnr('%'), float2nr(luaeval('vim.buffer().number')))
endfunc

" Test inserting lines in buffer.
func Test_lua_buffer_insert()
  new
  lua vim.buffer()[1] = '3'
  lua vim.buffer():insert('1', 0)
  lua vim.buffer():insert('2', 1)
  lua vim.buffer():insert('4', 10)

  call assert_equal(['1', '2', '3', '4'], getline(1, '$'))
  call assert_equal('4', luaeval('vim.buffer()[4]'))
  call assert_equal(v:null, luaeval('vim.buffer()[5]'))
  call assert_equal(v:null, luaeval('vim.buffer()[{}]'))
  if s:lua_543
    let msg = "[string \"vim chunk\"]:1: method 'xyz' is not callable (a nil value)"
  elseif s:lua_53_or_later
    let msg = "[string \"vim chunk\"]:1: attempt to call a nil value (method 'xyz')"
  else
    let msg = "[string \"vim chunk\"]:1: attempt to call method 'xyz' (a nil value)"
  endif
  call assert_fails('lua vim.buffer():xyz()', msg)
  call assert_fails('lua vim.buffer()[1] = {}',
        \ '[string "vim chunk"]:1: wrong argument to change')
  bwipe!
endfunc

" Test deleting line in buffer
func Test_lua_buffer_delete()
  new
  call setline(1, ['1', '2', '3'])
  call cursor(3, 1)
  lua vim.buffer()[2] = nil
  call assert_equal(['1', '3'], getline(1, '$'))

  call assert_fails('lua vim.buffer()[3] = nil',
        \           '[string "vim chunk"]:1: invalid line number')
  bwipe!
endfunc

" Test #vim.buffer() i.e. number of lines in buffer
func Test_lua_buffer_number_lines()
  new
  call setline(1, ['a', 'b', 'c'])
  call assert_equal(3, luaeval('#vim.buffer()'))
  bwipe!
endfunc

" Test vim.buffer():next() and vim.buffer():previous()
" Note that these functions get the next or previous buffers
" but do not switch buffer.
func Test_lua_buffer_next_previous()
  new Xfoo1
  new Xfoo2
  new Xfoo3
  b Xfoo2

  lua bn = vim.buffer():next()
  lua bp = vim.buffer():previous()

  call assert_equal('Xfoo2', luaeval('vim.buffer().name'))
  call assert_equal('Xfoo1', luaeval('bp.name'))
  call assert_equal('Xfoo3', luaeval('bn.name'))

  call assert_equal('Xfoo2', bufname('%'))

  lua bn()
  call assert_equal('Xfoo3', luaeval('vim.buffer().name'))
  call assert_equal('Xfoo3', bufname('%'))

  lua bp()
  call assert_equal('Xfoo1', luaeval('vim.buffer().name'))
  call assert_equal('Xfoo1', bufname('%'))

  lua bn, bp = nil
  %bwipe!
endfunc

" Test vim.buffer():isvalid()
func Test_lua_buffer_isvalid()
  new Xfoo
  lua b = vim.buffer()
  call assert_true(luaeval('b:isvalid()'))

  " FIXME: how to test the case when isvalid() returns v:false?
  " isvalid() gives errors when the buffer is wiped. Is it a bug?

  lua b = nil
  bwipe!
endfunc

func Test_lua_list()
  call assert_equal([], luaeval('vim.list()'))

  let l = []
  lua l = vim.eval('l')
  lua l:add(123)
  lua l:add('abc')
  lua l:add(true)
  lua l:add(false)
  lua l:add(nil)
  lua l:add(vim.eval("[1, 2, 3]"))
  lua l:add(vim.eval("{'a':1, 'b':2, 'c':3}"))
  call assert_equal([123, 'abc', v:true, v:false, v:null, [1, 2, 3], {'a': 1, 'b': 2, 'c': 3}], l)
  call assert_equal(7, luaeval('#l'))
  call assert_match('^list: \%(0x\)\?\x\+$', luaeval('tostring(l)'))

  lua l[1] = 124
  lua l[6] = nil
  lua l:insert('first')
  lua l:insert('xx', 3)
  call assert_fails('lua l:insert("xx", -20)',
        \ '[string "vim chunk"]:1: invalid position')
  call assert_equal(['first', 124, 'abc', 'xx', v:true, v:false, v:null, {'a': 1, 'b': 2, 'c': 3}], l)

  lockvar 1 l
  call assert_fails('lua l:add("x")', '[string "vim chunk"]:1: list is locked')
  call assert_fails('lua l:insert(2)', '[string "vim chunk"]:1: list is locked')
  call assert_fails('lua l[9] = 1', '[string "vim chunk"]:1: list is locked')

  unlockvar l
  let l = [1, 2]
  lua ll = vim.eval('l')
  let x = luaeval("ll[3]")
  call assert_equal(v:null, x)
  if s:lua_543
    let msg = "[string \"luaeval\"]:1: method 'xyz' is not callable (a nil value)"
  elseif s:lua_53_or_later
    let msg = "[string \"luaeval\"]:1: attempt to call a nil value (method 'xyz')"
  else
    let msg = "[string \"luaeval\"]:1: attempt to call method 'xyz' (a nil value)"
  endif
  call assert_fails('let x = luaeval("ll:xyz(3)")', msg)
  let y = luaeval("ll[{}]")
  call assert_equal(v:null, y)

  lua l = nil
endfunc

func Test_lua_list_table()
  " See :help lua-vim
  " Non-numeric keys should not be used to initialize the list
  " so say = 'hi' should be ignored.
  lua t = {3.14, 'hello', false, true, say = 'hi'}
  call assert_equal([3.14, 'hello', v:false, v:true], luaeval('vim.list(t)'))
  lua t = nil

  call assert_fails('lua vim.list(1)', '[string "vim chunk"]:1: table expected, got number')
  call assert_fails('lua vim.list("x")', '[string "vim chunk"]:1: table expected, got string')
  call assert_fails('lua vim.list(print)', '[string "vim chunk"]:1: table expected, got function')
  call assert_fails('lua vim.list(true)', '[string "vim chunk"]:1: table expected, got boolean')
endfunc

func Test_lua_list_table_insert_remove()
  if !s:lua_53_or_later
    throw 'Skipped: Lua version < 5.3'
  endif

  let l = [1, 2]
  lua t = vim.eval('l')
  lua table.insert(t, 10)
  lua t[#t + 1] = 20
  lua table.insert(t, 2, 30)
  call assert_equal(l, [1, 30, 2, 10, 20])
  lua table.remove(t, 2)
  call assert_equal(l, [1, 2, 10, 20])
  lua t[3] = nil
  call assert_equal(l, [1, 2, 20])
  lua removed_value = table.remove(t, 3)
  call assert_equal(luaeval('removed_value'), 20)
  lua t = nil
  lua removed_value = nil
  unlet l
endfunc

" Test l() i.e. iterator on list
func Test_lua_list_iter()
  lua l = vim.list():add('foo'):add('bar')
  lua str = ''
  lua for v in l() do str = str .. v end
  call assert_equal('foobar', luaeval('str'))

  lua str, l = nil
endfunc

func Test_lua_recursive_list()
  lua l = vim.list():add(1):add(2)
  lua l = l:add(l)

  call assert_equal(1, luaeval('l[1]'))
  call assert_equal(2, luaeval('l[2]'))

  call assert_equal(1, luaeval('l[3][1]'))
  call assert_equal(2, luaeval('l[3][2]'))

  call assert_equal(1, luaeval('l[3][3][1]'))
  call assert_equal(2, luaeval('l[3][3][2]'))

  call assert_equal('[1, 2, [...]]', string(luaeval('l')))

  call assert_match('^list: \%(0x\)\?\x\+$', luaeval('tostring(l)'))
  call assert_equal(luaeval('tostring(l)'), luaeval('tostring(l[3])'))

  call assert_equal(luaeval('l'), luaeval('l[3]'))
  call assert_equal(luaeval('l'), luaeval('l[3][3]'))

  lua l = nil
endfunc

func Test_lua_dict()
  call assert_equal({}, luaeval('vim.dict()'))

  let d = {}
  lua d = vim.eval('d')
  lua d[0] = 123
  lua d[1] = "abc"
  lua d[2] = true
  lua d[3] = false
  lua d[4] = vim.eval("[1, 2, 3]")
  lua d[5] = vim.eval("{'a':1, 'b':2, 'c':3}")
  call assert_equal({'0':123, '1':'abc', '2':v:true, '3':v:false, '4': [1, 2, 3], '5': {'a':1, 'b':2, 'c':3}}, d)
  call assert_equal(6, luaeval('#d'))
  call assert_match('^dict: \%(0x\)\?\x\+$', luaeval('tostring(d)'))

  call assert_equal('abc', luaeval('d[1]'))
  call assert_equal(v:null, luaeval('d[22]'))

  lua d[0] = 124
  lua d[4] = nil
  call assert_equal({'0':124, '1':'abc', '2':v:true, '3':v:false, '5': {'a':1, 'b':2, 'c':3}}, d)

  lockvar 1 d
  call assert_fails('lua d[6] = 1', '[string "vim chunk"]:1: dict is locked')
  unlockvar d

  " Error case
  lua d = {}
  lua d[''] = 10
  call assert_fails("let t = luaeval('vim.dict(d)')",
        \ s:lua_53_or_later
        \ ? '[string "luaeval"]:1: table has empty key'
        \ : 'table has empty key')
  let d = {}
  lua x = vim.eval('d')
  call assert_fails("lua x[''] = 10", '[string "vim chunk"]:1: empty key')
  lua x['a'] = nil
  call assert_equal({}, d)

  " cannot assign funcrefs in the global scope
  lua x = vim.eval('g:')
  call assert_fails("lua x['min'] = vim.funcref('max')",
        \ '[string "vim chunk"]:1: cannot assign funcref to builtin scope')

  lua d = nil
endfunc

func Test_lua_dict_table()
  lua t = {key1 = 'x', key2 = 3.14, key3 = true, key4 = false}
  call assert_equal({'key1': 'x', 'key2': 3.14, 'key3': v:true, 'key4': v:false},
        \           luaeval('vim.dict(t)'))

  " Same example as in :help lua-vim.
  lua t = {math.pi, false, say = 'hi'}
  " FIXME: commented out as it currently does not work as documented:
  " Expected {'say': 'hi'}
  " but got {'1': 3.141593, '2': v:false, 'say': 'hi'}
  " Is the documentation or the code wrong?
  "call assert_equal({'say' : 'hi'}, luaeval('vim.dict(t)'))
  lua t = nil

  call assert_fails('lua vim.dict(1)', '[string "vim chunk"]:1: table expected, got number')
  call assert_fails('lua vim.dict("x")', '[string "vim chunk"]:1: table expected, got string')
  call assert_fails('lua vim.dict(print)', '[string "vim chunk"]:1: table expected, got function')
  call assert_fails('lua vim.dict(true)', '[string "vim chunk"]:1: table expected, got boolean')
endfunc

" Test d() i.e. iterator on dictionary
func Test_lua_dict_iter()
  let d = {'a': 1, 'b':2}
  lua d = vim.eval('d')
  lua str = ''
  lua for k,v in d() do str = str .. k ..':' .. v .. ',' end
  call assert_equal('a:1,b:2,', luaeval('str'))

  lua str, d = nil
endfunc

func Test_lua_blob()
  call assert_equal(0z, luaeval('vim.blob("")'))
  call assert_equal(0z31326162, luaeval('vim.blob("12ab")'))
  call assert_equal(0z00010203, luaeval('vim.blob("\000\001\002\003")'))
  call assert_equal(0z8081FEFF, luaeval('vim.blob("\128\129\254\255")'))

  lua b = vim.blob("\000\000\000\000")
  call assert_equal(0z00000000, luaeval('b'))
  call assert_equal(4, luaeval('#b'))
  lua b[0], b[1], b[2], b[3] = 1, 32, 256, 0xff
  call assert_equal(0z012000ff, luaeval('b'))
  lua b[4] = string.byte("z", 1)
  call assert_equal(0z012000ff.7a, luaeval('b'))
  call assert_equal(5, luaeval('#b'))
  call assert_fails('lua b[#b+1] = 0x80', '[string "vim chunk"]:1: index out of range')
  lua b:add("12ab")
  call assert_equal(0z012000ff.7a313261.62, luaeval('b'))
  call assert_equal(9, luaeval('#b'))
  call assert_fails('lua b:add(nil)', '[string "vim chunk"]:1: string expected, got nil')
  call assert_fails('lua b:add(true)', '[string "vim chunk"]:1: string expected, got boolean')
  call assert_fails('lua b:add({})', '[string "vim chunk"]:1: string expected, got table')
  lua b = nil

  let b = 0z0102
  lua lb = vim.eval('b')
  let n = luaeval('lb[1]')
  call assert_equal(2, n)
  let n = luaeval('lb[6]')
  call assert_equal(v:null, n)
  if s:lua_543
    let msg = "[string \"luaeval\"]:1: method 'xyz' is not callable (a nil value)"
  elseif s:lua_53_or_later
    let msg = "[string \"luaeval\"]:1: attempt to call a nil value (method 'xyz')"
  else
    let msg = "[string \"luaeval\"]:1: attempt to call method 'xyz' (a nil value)"
  endif
  call assert_fails('let x = luaeval("lb:xyz(3)")', msg)
  let y = luaeval("lb[{}]")
  call assert_equal(v:null, y)

  lockvar b
  call assert_fails('lua lb[1] = 2', '[string "vim chunk"]:1: blob is locked')
  call assert_fails('lua lb:add("12")', '[string "vim chunk"]:1: blob is locked')

  " Error cases
  lua t = {}
  call assert_fails('lua b = vim.blob(t)',
        \ '[string "vim chunk"]:1: string expected, got table')
endfunc

def Vim9Test(Callback: func())
  Callback()
enddef

func Test_call_lua_func_from_vim9_func()
  " this only tests that Vim doesn't crash
  lua << EOF
vim.fn.Vim9Test(function () print('Hello') end)
EOF
endfunc

func Test_lua_funcref()
  function I(x)
    return a:x
  endfunction
  let R = function('I')
  lua i1 = vim.funcref"I"
  lua i2 = vim.eval"R"
  lua msg = "funcref|test|" .. (#i2(i1) == #i1(i2) and "OK" or "FAIL")
  lua msg = vim.funcref"tr"(msg, "|", " ")
  call assert_equal("funcref test OK", luaeval('msg'))

  " Error cases
  call assert_fails('lua f1 = vim.funcref("")',
        \ '[string "vim chunk"]:1: invalid function name: ')
  call assert_fails('lua f1 = vim.funcref("10")',
        \ '[string "vim chunk"]:1: invalid function name: 10')
  let fname = test_null_string()
  call assert_fails('lua f1 = vim.funcref(fname)',
        \ "[string \"vim chunk\"]:1: bad argument #1 to 'funcref' (string expected, got nil)")
  call assert_fails('lua vim.funcref("abc")()',
        \ ['E117:', '\[string "vim chunk"]:1: cannot call funcref'])

  " dict funcref
  function Mylen() dict
    return len(self.data)
  endfunction
  let l = [0, 1, 2, 3]
  let mydict = {'data': l}
  lua d = vim.eval"mydict"
  lua d.len = vim.funcref"Mylen" -- assign d as 'self'
  lua res = (d.len() == vim.funcref"len"(vim.eval"l")) and "OK" or "FAIL"
  call assert_equal("OK", luaeval('res'))
  call assert_equal(function('Mylen', {'data': l, 'len': function('Mylen')}), mydict.len)

  lua i1, i2, msg, d, res = nil
endfunc

" Test vim.type()
func Test_lua_type()
  " The following values are identical to Lua's type function.
  call assert_equal('string',   luaeval('vim.type("foo")'))
  call assert_equal('number',   luaeval('vim.type(1)'))
  call assert_equal('number',   luaeval('vim.type(1.2)'))
  call assert_equal('function', luaeval('vim.type(print)'))
  call assert_equal('table',    luaeval('vim.type({})'))
  call assert_equal('boolean',  luaeval('vim.type(true)'))
  call assert_equal('boolean',  luaeval('vim.type(false)'))
  call assert_equal('nil',      luaeval('vim.type(nil)'))

  " The following values are specific to Vim.
  call assert_equal('window',   luaeval('vim.type(vim.window())'))
  call assert_equal('buffer',   luaeval('vim.type(vim.buffer())'))
  call assert_equal('list',     luaeval('vim.type(vim.list())'))
  call assert_equal('dict',     luaeval('vim.type(vim.dict())'))
  call assert_equal('funcref',  luaeval('vim.type(vim.funcref("Test_type"))'))
endfunc

" Test vim.open()
func Test_lua_open()
  call assert_notmatch('XOpen', execute('ls'))

  " Open a buffer XOpen1, but do not jump to it.
  lua b = vim.open('XOpen1')
  call assert_equal('XOpen1', luaeval('b.name'))
  call assert_equal('', bufname('%'))

  call assert_match('XOpen1', execute('ls'))
  call assert_notequal('XOpen2', bufname('%'))

  " Open a buffer XOpen2 and jump to it.
  lua b = vim.open('XOpen2')()
  call assert_equal('XOpen2', luaeval('b.name'))
  call assert_equal('XOpen2', bufname('%'))

  lua b = nil
  %bwipe!
endfunc

func Test_update_package_paths()
  set runtimepath+=./testluaplugin
  call assert_equal("hello from lua", luaeval("require('testluaplugin').hello()"))
endfunc

func Vim_func_call_lua_callback(Concat, Cb)
  let l:message = a:Concat("hello", "vim")
  call a:Cb(l:message)
endfunc

func Test_pass_lua_callback_to_vim_from_lua()
  lua pass_lua_callback_to_vim_from_lua_result = ""
  call assert_equal("", luaeval("pass_lua_callback_to_vim_from_lua_result"))
  lua <<EOF
  vim.funcref('Vim_func_call_lua_callback')(
    function(greeting, message)
      return greeting .. " " .. message
    end,
    function(message)
      pass_lua_callback_to_vim_from_lua_result = message
    end)
EOF
  call assert_equal("hello vim", luaeval("pass_lua_callback_to_vim_from_lua_result"))
endfunc

func Vim_func_call_metatable_lua_callback(Greet)
  return a:Greet("world")
endfunc

func Test_pass_lua_metatable_callback_to_vim_from_lua()
  let result = luaeval("vim.funcref('Vim_func_call_metatable_lua_callback')(setmetatable({ space = ' '}, { __call = function(tbl, msg) return 'hello' .. tbl.space .. msg  end }) )")
  call assert_equal("hello world", result)
endfunc

" Test vim.line()
func Test_lua_line()
  new
  call setline(1, ['first line', 'second line'])
  1
  call assert_equal('first line', luaeval('vim.line()'))
  2
  call assert_equal('second line', luaeval('vim.line()'))
  bwipe!
endfunc

" Test vim.beep()
func Test_lua_beep()
  call assert_beeps('lua vim.beep()')
endfunc

" Test errors in luaeval()
func Test_luaeval_error()
  " Compile error
  call assert_fails("call luaeval('-nil')",
  \ '[string "luaeval"]:1: attempt to perform arithmetic on a nil value')
  call assert_fails("call luaeval(']')",
  \ "[string \"luaeval\"]:1: unexpected symbol near ']'")
  lua co = coroutine.create(function () print("hi") end)
  call assert_fails('let i = luaeval("co")', 'luaeval: cannot convert value')
  lua co = nil
  call assert_fails('let m = luaeval("{}")', 'luaeval: cannot convert value')
endfunc

" Test :luafile foo.lua
func Test_luafile()
  call writefile(["str = 'hello'", "num = 123" ], 'Xlua_file', 'D')
  call setfperm('Xlua_file', 'r-xr-xr-x')

  luafile Xlua_file
  call assert_equal('hello', luaeval('str'))
  call assert_equal(123, luaeval('num'))

  lua str, num = nil
endfunc

" Test :luafile %
func Test_luafile_percent()
  new Xlua_file
  append
    str, num = 'foo', 321.0
    print(string.format('str=%s, num=%d', str, num))
.
  w!
  luafile %
  let msg = split(execute('message'), "\n")[-1]
  call assert_equal('str=foo, num=321', msg)

  lua str, num = nil
  call delete('Xlua_file')
  bwipe!
endfunc

" Test :luafile with syntax error
func Test_luafile_error()
  new Xlua_file
  call writefile(['nil = 0' ], 'Xlua_file', 'D')
  call setfperm('Xlua_file', 'r-xr-xr-x')

  call assert_fails('luafile Xlua_file', "Xlua_file:1: unexpected symbol near 'nil'")

  bwipe!
endfunc

" Test :luafile printing a long string
func Test_luafile_print()
  new Xlua_file
  let lines =<< trim END
      local data = ''
      for i = 1, 130 do
	data = data .. 'xxxxx asd as as dad sad sad xz cxz  czxcxzczxc ad ad asd asd asd asd asd'
      end
      print(data)
  END
  call setline(1, lines)
  w
  luafile %

  call delete('Xlua_file')
  bwipe!
endfunc

" Test for dealing with strings containing newlines and null character
func Test_lua_string_with_newline()
  let x = execute('lua print("Hello\nWorld", 2)')
  call assert_equal("\nHello\nWorld 2", x)
  new
  lua k = vim.buffer(vim.eval('bufnr()'))
  lua k:insert("Hello\0World", 0)
  call assert_equal(["Hello\nWorld", ''], getline(1, '$'))
  close!
endfunc

func Test_lua_set_cursor()
  " Check that setting the cursor position works.
  new
  call setline(1, ['first line', 'second line'])
  normal gg
  lua << trim EOF
    w = vim.window()
    w.line = 1
    w.col = 5
  EOF
  call assert_equal([1, 5], [line('.'), col('.')])

  " Check that movement after setting cursor position keeps current column.
  normal j
  call assert_equal([2, 5], [line('.'), col('.')])
endfunc

" Test for various heredoc syntax
func Test_lua_heredoc()
  lua << END
vim.command('let s = "A"')
END
  lua <<
vim.command('let s ..= "B"')
.
  lua << trim END
    vim.command('let s ..= "C"')
  END
  lua << trim
    vim.command('let s ..= "D"')
  .
  lua << trim eof
    vim.command('let s ..= "E"')
  eof
  call assert_equal('ABCDE', s)
endfunc

" Test for adding, accessing and removing global variables using the vim.g
" Lua table
func Test_lua_global_var_table()
  " Access global variables with different types of values
  let g:Var1 = 10
  let g:Var2 = 'Hello'
  let g:Var3 = ['a', 'b']
  let g:Var4 = #{x: 'edit', y: 'run'}
  let g:Var5 = function('min')
  call assert_equal(10, luaeval('vim.g.Var1'))
  call assert_equal('Hello', luaeval('vim.g.Var2'))
  call assert_equal(['a', 'b'], luaeval('vim.g.Var3'))
  call assert_equal(#{x: 'edit', y: 'run'}, luaeval('vim.g.Var4'))
  call assert_equal(2, luaeval('vim.g.Var5')([5, 9, 2]))

  " Access list of dictionaries and dictionary of lists
  let g:Var1 = [#{a: 10}, #{b: 20}]
  let g:Var2 = #{p: [5, 6], q: [1.1, 2.2]}
  call assert_equal([#{a: 10}, #{b: 20}], luaeval('vim.g.Var1'))
  call assert_equal(#{p: [5, 6], q: [1.1, 2.2]}, luaeval('vim.g.Var2'))

  " Create new global variables with different types of values
  unlet g:Var1 g:Var2 g:Var3 g:Var4 g:Var5
  lua << trim END
    vim.g.Var1 = 34
    vim.g.Var2 = 'World'
    vim.g.Var3 = vim.list({'#', '$'})
    vim.g.Var4 = vim.dict({model='honda', year=2020})
    vim.g.Var5 = vim.funcref('max')
  END
  call assert_equal(34, g:Var1)
  call assert_equal('World', g:Var2)
  call assert_equal(['#', '$'], g:Var3)
  call assert_equal(#{model: 'honda', year: 2020}, g:Var4)
  call assert_equal(10, g:Var5([5, 10, 9]))

  " Create list of dictionaries and dictionary of lists
  unlet g:Var1 g:Var2
  lua << trim END
    vim.g.Var1 = vim.list({vim.dict({a=10}), vim.dict({b=20})})
    vim.g.Var2 = vim.dict({p=vim.list({5, 6}), q=vim.list({1.1, 2.2})})
  END
  call assert_equal([#{a: 10}, #{b: 20}], luaeval('vim.g.Var1'))
  call assert_equal(#{p: [5, 6], q: [1.1, 2.2]}, luaeval('vim.g.Var2'))

  " Modify a global variable with a list value or a dictionary value
  let g:Var1 = [10, 20]
  let g:Var2 = #{one: 'mercury', two: 'mars'}
  lua << trim END
    vim.g.Var1[2] = nil
    vim.g.Var1[3] = 15
    vim.g.Var2['two'] = nil
    vim.g.Var2['three'] = 'earth'
  END
  call assert_equal([10, 15], g:Var1)
  call assert_equal(#{one: 'mercury', three: 'earth'}, g:Var2)

  " Remove global variables with different types of values
  let g:Var1 = 10
  let g:Var2 = 'Hello'
  let g:Var3 = ['a', 'b']
  let g:Var4 = #{x: 'edit', y: 'run'}
  let g:Var5 = function('min')
  lua << trim END
    vim.g.Var1 = nil
    vim.g.Var2 = nil
    vim.g.Var3 = nil
    vim.g.Var4 = nil
    vim.g.Var5 = nil
  END
  call assert_false(exists('g:Var1'))
  call assert_false(exists('g:Var2'))
  call assert_false(exists('g:Var3'))
  call assert_false(exists('g:Var4'))
  call assert_false(exists('g:Var5'))

  " Try to modify and remove a locked global variable
  let g:Var1 = 10
  lockvar g:Var1
  call assert_fails('lua vim.g.Var1 = 20', 'variable is locked')
  call assert_fails('lua vim.g.Var1 = nil', 'variable is locked')
  unlockvar g:Var1
  let g:Var2 = [7, 14]
  lockvar 0 g:Var2
  lua vim.g.Var2[2] = nil
  lua vim.g.Var2[3] = 21
  call assert_fails('lua vim.g.Var2 = nil', 'variable is locked')
  call assert_equal([7, 21], g:Var2)
  lockvar 1 g:Var2
  call assert_fails('lua vim.g.Var2[2] = nil', 'list is locked')
  call assert_fails('lua vim.g.Var2[3] = 21', 'list is locked')
  unlockvar g:Var2

  let g:TestFunc = function('len')
  call assert_fails('lua vim.g.func = vim.g.TestFunc', ['E704:', 'Couldn''t add to dictionary'])
  unlet g:TestFunc

  " Attempt to access a non-existing global variable
  call assert_equal(v:null, luaeval('vim.g.NonExistingVar'))
  lua vim.g.NonExisting = nil

  unlet! g:Var1 g:Var2 g:Var3 g:Var4 g:Var5
endfunc

" Test for accessing and modifying predefined vim variables using the vim.v
" Lua table
func Test_lua_predefined_var_table()
  call assert_equal(v:progpath, luaeval('vim.v.progpath'))
  let v:errmsg = 'SomeError'
  call assert_equal('SomeError', luaeval('vim.v.errmsg'))
  lua vim.v.errmsg = 'OtherError'
  call assert_equal('OtherError', v:errmsg)
  lua vim.v.errmsg = 42
  call assert_equal('42', v:errmsg)
  call assert_fails('lua vim.v.errmsg = nil', 'variable is fixed')
  let v:oldfiles = ['one', 'two']
  call assert_equal(['one', 'two'], luaeval('vim.v.oldfiles'))
  lua vim.v.oldfiles = vim.list({})
  call assert_equal([], v:oldfiles)
  call assert_fails('lua vim.v.oldfiles = "a"',
        \ 'Setting v:oldfiles to value with wrong type')
  call assert_equal([], v:oldfiles)
  call assert_equal(v:null, luaeval('vim.v.null'))
  call assert_fails('lua vim.v.argv[1] = nil', 'list is locked')
  call assert_fails('lua vim.v.newvar = 1', 'Dictionary is locked')

  new
  call setline(1, ' foo foo foo')
  /foo
  call assert_equal([0, 1, 2, 0, 2], getcurpos())
  call assert_equal(1, v:searchforward)
  normal! n
  call assert_equal([0, 1, 6, 0, 6], getcurpos())
  lua vim.v.searchforward = 0
  call assert_equal(0, v:searchforward)
  normal! n
  call assert_equal([0, 1, 2, 0, 2], getcurpos())
  lua vim.v.searchforward = 1
  call assert_equal(1, v:searchforward)
  normal! n
  call assert_equal([0, 1, 6, 0, 6], getcurpos())
  bwipe!
endfunc

" Test for adding, accessing and modifying window-local variables using the
" vim.w Lua table
func Test_lua_window_var_table()
  " Access window variables with different types of values
  new
  let w:wvar1 = 10
  let w:wvar2 = 'edit'
  let w:wvar3 = 3.14
  let w:wvar4 = 0zdeadbeef
  let w:wvar5 = ['a', 'b']
  let w:wvar6 = #{action: 'run'}
  call assert_equal(10, luaeval('vim.w.wvar1'))
  call assert_equal('edit', luaeval('vim.w.wvar2'))
  call assert_equal(3.14, luaeval('vim.w.wvar3'))
  call assert_equal(0zdeadbeef, luaeval('vim.w.wvar4'))
  call assert_equal(['a', 'b'], luaeval('vim.w.wvar5'))
  call assert_equal(#{action: 'run'}, luaeval('vim.w.wvar6'))
  call assert_equal(v:null, luaeval('vim.w.NonExisting'))

  " modify a window variable
  lua vim.w.wvar2 = 'paste'
  call assert_equal('paste', w:wvar2)

  " change the type stored in a variable
  let w:wvar2 = [1, 2]
  lua vim.w.wvar2 = vim.dict({a=10, b=20})
  call assert_equal(#{a: 10, b: 20}, w:wvar2)

  " create a new window variable
  lua vim.w.wvar7 = vim.dict({a=vim.list({1, 2}), b=20})
  call assert_equal(#{a: [1, 2], b: 20}, w:wvar7)

  " delete a window variable
  lua vim.w.wvar2 = nil
  call assert_false(exists('w:wvar2'))

  new
  call assert_equal(v:null, luaeval('vim.w.wvar1'))
  call assert_equal(v:null, luaeval('vim.w.wvar2'))
  %bw!
endfunc

" Test for adding, accessing and modifying buffer-local variables using the
" vim.b Lua table
func Test_lua_buffer_var_table()
  " Access buffer variables with different types of values
  let b:bvar1 = 10
  let b:bvar2 = 'edit'
  let b:bvar3 = 3.14
  let b:bvar4 = 0zdeadbeef
  let b:bvar5 = ['a', 'b']
  let b:bvar6 = #{action: 'run'}
  call assert_equal(10, luaeval('vim.b.bvar1'))
  call assert_equal('edit', luaeval('vim.b.bvar2'))
  call assert_equal(3.14, luaeval('vim.b.bvar3'))
  call assert_equal(0zdeadbeef, luaeval('vim.b.bvar4'))
  call assert_equal(['a', 'b'], luaeval('vim.b.bvar5'))
  call assert_equal(#{action: 'run'}, luaeval('vim.b.bvar6'))
  call assert_equal(v:null, luaeval('vim.b.NonExisting'))

  " modify a buffer variable
  lua vim.b.bvar2 = 'paste'
  call assert_equal('paste', b:bvar2)

  " change the type stored in a variable
  let b:bvar2 = [1, 2]
  lua vim.b.bvar2 = vim.dict({a=10, b=20})
  call assert_equal(#{a: 10, b: 20}, b:bvar2)

  " create a new buffer variable
  lua vim.b.bvar7 = vim.dict({a=vim.list({1, 2}), b=20})
  call assert_equal(#{a: [1, 2], b: 20}, b:bvar7)

  " delete a buffer variable
  lua vim.b.bvar2 = nil
  call assert_false(exists('b:bvar2'))

  new
  call assert_equal(v:null, luaeval('vim.b.bvar1'))
  call assert_equal(v:null, luaeval('vim.b.bvar2'))
  %bw!
endfunc

" Test for adding, accessing and modifying tabpage-local variables using the
" vim.t Lua table
func Test_lua_tabpage_var_table()
  " Access tabpage variables with different types of values
  let t:tvar1 = 10
  let t:tvar2 = 'edit'
  let t:tvar3 = 3.14
  let t:tvar4 = 0zdeadbeef
  let t:tvar5 = ['a', 'b']
  let t:tvar6 = #{action: 'run'}
  call assert_equal(10, luaeval('vim.t.tvar1'))
  call assert_equal('edit', luaeval('vim.t.tvar2'))
  call assert_equal(3.14, luaeval('vim.t.tvar3'))
  call assert_equal(0zdeadbeef, luaeval('vim.t.tvar4'))
  call assert_equal(['a', 'b'], luaeval('vim.t.tvar5'))
  call assert_equal(#{action: 'run'}, luaeval('vim.t.tvar6'))
  call assert_equal(v:null, luaeval('vim.t.NonExisting'))

  " modify a tabpage variable
  lua vim.t.tvar2 = 'paste'
  call assert_equal('paste', t:tvar2)

  " change the type stored in a variable
  let t:tvar2 = [1, 2]
  lua vim.t.tvar2 = vim.dict({a=10, b=20})
  call assert_equal(#{a: 10, b: 20}, t:tvar2)

  " create a new tabpage variable
  lua vim.t.tvar7 = vim.dict({a=vim.list({1, 2}), b=20})
  call assert_equal(#{a: [1, 2], b: 20}, t:tvar7)

  " delete a tabpage variable
  lua vim.t.tvar2 = nil
  call assert_false(exists('t:tvar2'))

  tabnew
  call assert_equal(v:null, luaeval('vim.t.tvar1'))
  call assert_equal(v:null, luaeval('vim.t.tvar2'))
  %bw!
endfunc

" Test for vim.version()
func Test_lua_vim_version()
  lua << trim END
    vimver = vim.version()
    vimver_n = vimver.major * 100 + vimver.minor
  END
  call assert_equal(v:version, luaeval('vimver_n'))
endfunc

" Test for running multiple commands using vim.command()
func Test_lua_multiple_commands()
  lua << trim END
    vim.command([[
        let Var1 = []
        for i in range(3)
          let Var1 += [#{name: 'x'}]
        endfor
        augroup Luagroup
          autocmd!
          autocmd User Luatest echo 'Hello'
        augroup END
      ]])
  END
  call assert_equal([{'name': 'x'}, {'name': 'x'}, {'name': 'x'}], Var1)
  call assert_true(exists('#Luagroup'))
  call assert_true(exists('#Luagroup#User#Luatest'))
  augroup Luagroup
    autocmd!
  augroup END
  augroup! Luagroup
endfunc

func Test_lua_debug()
  CheckRunVimInTerminal

  let buf = RunVimInTerminal('', {'rows': 10})
  call term_sendkeys(buf, ":lua debug.debug()\n")
  call WaitForAssert({-> assert_equal('lua_debug> ', term_getline(buf, 10))})

  call term_sendkeys(buf, "foo = 42\n")
  call WaitForAssert({-> assert_equal('lua_debug> foo = 42', term_getline(buf, 9))})
  call WaitForAssert({-> assert_equal('lua_debug> ',         term_getline(buf, 10))})

  call term_sendkeys(buf, "print(foo)\n")
  call WaitForAssert({-> assert_equal('lua_debug> print(foo)', term_getline(buf, 8))})
  call WaitForAssert({-> assert_equal('42',                    term_getline(buf, 9))})
  call WaitForAssert({-> assert_equal('lua_debug> ',           term_getline(buf, 10))})

  call term_sendkeys(buf, "-\n")
  call WaitForAssert({-> assert_equal("(debug command):1: unexpected symbol near '-'",
  \                                                  term_getline(buf, 9))})
  call WaitForAssert({-> assert_equal('lua_debug> ', term_getline(buf, 10))})

  call term_sendkeys(buf, "cont\n")
  call WaitForAssert({-> assert_match(' All$', term_getline(buf, 10))})

  " Entering an empty line also exits the debugger.
  call term_sendkeys(buf, ":lua debug.debug()\n")
  call WaitForAssert({-> assert_equal('lua_debug> ', term_getline(buf, 10))})
  call term_sendkeys(buf, "\n")
  call WaitForAssert({-> assert_match(' All$', term_getline(buf, 10))})

  call StopVimInTerminal(buf)
endfunc

" Test for a crash when a Lua funcref is invoked with parameters from Vim
func Test_lua_funcref_with_params()
  let Lua_funcref = luaeval('function(d) local s = "in Lua callback" end')
  call Lua_funcref({'a' : 'b'})
  call assert_true(v:true)
endfunc

" vim: shiftwidth=2 sts=2 expandtab
