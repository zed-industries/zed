" Test Vim9 assignments

source check.vim
import './vim9.vim' as v9
source term_util.vim

let s:appendToMe = 'xxx'
let s:addToMe = 111
let s:newVar = ''
let g:existing = 'yes'
let g:inc_counter = 1
let $SOME_ENV_VAR = 'some'
let g:alist = [7]
let g:adict = #{a: 1}
let g:astring = 'text'

def Test_assignment_bool()
  var bool1: bool = true
  assert_equal(v:true, bool1)
  var bool2: bool = false
  assert_equal(v:false, bool2)

  var bool3: bool = 0
  assert_equal(false, bool3)
  var bool4: bool = 1
  assert_equal(true, bool4)

  var bool5: bool = 1 && true
  assert_equal(true, bool5)
  var bool6: bool = 0 && 1
  assert_equal(false, bool6)
  var bool7: bool = 0 || 1 && true
  assert_equal(true, bool7)

  var lines =<< trim END
    vim9script
    def GetFlag(): bool
      var flag: bool = 1
      return flag
    enddef
    var flag: bool = GetFlag()
    assert_equal(true, flag)
    flag = 0
    assert_equal(false, flag)
    flag = 1
    assert_equal(true, flag)
    flag = 1 || true
    assert_equal(true, flag)
    flag = 1 && false
    assert_equal(false, flag)

    var cp: bool = &cp
    var fen: bool = &l:fen
  END
  v9.CheckScriptSuccess(lines)
  v9.CheckDefAndScriptFailure(['var x: bool = 2'], 'E1012:')
  v9.CheckDefAndScriptFailure(['var x: bool = -1'], 'E1012:')
  v9.CheckDefAndScriptFailure(['var x: bool = [1]'], 'E1012:')
  v9.CheckDefAndScriptFailure(['var x: bool = {}'], 'E1012:')
  v9.CheckDefAndScriptFailure(['var x: bool = "x"'], 'E1012:')

  v9.CheckDefAndScriptFailure(['var x: bool = "x"', '', 'eval 0'], 'E1012:', 1)
enddef

def Test_syntax()
  var name = 234
  var other: list<string> = ['asdf']
enddef

def Test_assignment()
  v9.CheckDefFailure(['var x:string'], 'E1069:')
  v9.CheckDefFailure(['var x:string = "x"'], 'E1069:')
  v9.CheckDefFailure(['var a:string = "x"'], 'E1069:')
  v9.CheckDefFailure(['var lambda = () => "lambda"'], 'E704:')
  v9.CheckScriptFailure(['var x = "x"'], 'E1124:')

  # lower case name is OK for a list
  var lambdaLines =<< trim END
      var lambdaList: list<func> = [g:Test_syntax]
      lambdaList[0] = () => "lambda"
  END
  v9.CheckDefAndScriptSuccess(lambdaLines)

  var nr: number = 1234
  v9.CheckDefFailure(['var nr: number = "asdf"'], 'E1012:')

  var a: number = 6 #comment
  assert_equal(6, a)

  if has('channel')
    var chan1: channel
    assert_equal('fail', ch_status(chan1))

    var job1: job
    assert_equal('fail', job_status(job1))

    # calling job_start() is in test_vim9_fails.vim, it causes leak reports
  endif
  var float1: float = 3.4
  var Funky1: func
  var Funky2: func = function('len')
  var Party2: func = funcref('g:Test_syntax')

  g:newvar = 'new'  #comment
  assert_equal('new', g:newvar)

  assert_equal('yes', g:existing)
  g:existing = 'no'
  assert_equal('no', g:existing)

  v:char = 'abc'
  assert_equal('abc', v:char)

  $ENVVAR = 'foobar'
  assert_equal('foobar', $ENVVAR)
  $ENVVAR = ''

  var lines =<< trim END
    vim9script
    $ENVVAR = 'barfoo'
    assert_equal('barfoo', $ENVVAR)
    $ENVVAR = ''
  END
  v9.CheckScriptSuccess(lines)

  appendToMe ..= 'yyy'
  assert_equal('xxxyyy', appendToMe)
  addToMe += 222
  assert_equal(333, addToMe)
  newVar = 'new'
  assert_equal('new', newVar)

  set ts=7
  var ts: number = &ts
  assert_equal(7, ts)
  &ts += 1
  assert_equal(8, &ts)
  &ts -= 3
  assert_equal(5, &ts)
  &ts *= 2
  assert_equal(10, &ts)
  &ts /= 3
  assert_equal(3, &ts)
  set ts=10
  &ts %= 4
  assert_equal(2, &ts)

  assert_fails('&ts /= 0', ['E1154:', 'E1154:'])
  assert_fails('&ts %= 0', ['E1154:', 'E1154:'])
  assert_fails('&ts /= []', ['E745:', 'E745:'])
  assert_fails('&ts %= []', ['E745:', 'E745:'])
  assert_equal(2, &ts)

  var f100: float = 100.0
  f100 /= 5
  assert_equal(20.0, f100)

  var f200: float = 200.0
  f200 /= 5.0
  assert_equal(40.0, f200)

  v9.CheckDefFailure(['var nr: number = 200', 'nr /= 5.0'], 'E1012:')

  lines =<< trim END
    &ts = 6
    &ts += 3
    assert_equal(9, &ts)

    &l:ts = 6
    assert_equal(6, &ts)
    &l:ts += 2
    assert_equal(8, &ts)

    &g:ts = 6
    assert_equal(6, &g:ts)
    &g:ts += 2
    assert_equal(8, &g:ts)

    &number = true
    assert_equal(true, &number)
    &number = 0
    assert_equal(false, &number)
    &number = 1
    assert_equal(true, &number)
    &number = false
    assert_equal(false, &number)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefFailure(['&notex += 3'], 'E113:')
  v9.CheckDefFailure(['&ts ..= "xxx"'], 'E1019:')
  v9.CheckDefFailure(['&ts = [7]'], 'E1012:')
  v9.CheckDefExecFailure(['&ts = g:alist'], 'E1012: Type mismatch; expected number but got list<number>')
  v9.CheckDefFailure(['&ts = "xx"'], 'E1012:')
  v9.CheckDefExecFailure(['&ts = g:astring'], 'E1012: Type mismatch; expected number but got string')
  v9.CheckDefFailure(['&path += 3'], 'E1012:')
  v9.CheckDefExecFailure(['&bs = "asdf"'], 'E474:')
  # test freeing ISN_STOREOPT
  v9.CheckDefFailure(['&ts = 3', 'var asdf'], 'E1022:')
  &ts = 8

  lines =<< trim END
    var save_TI = &t_TI
    &t_TI = ''
    assert_equal('', &t_TI)
    &t_TI = 'xxx'
    assert_equal('xxx', &t_TI)
    &t_TI = save_TI
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefFailure(['&t_TI = 123'], 'E1012:')
  v9.CheckScriptFailure(['vim9script', '&t_TI = 123'], 'E928:')

  v9.CheckDefFailure(['var s:var = 123'], 'E1101:')
  v9.CheckDefFailure(['var s:var: number'], 'E1101:')

  v9.CheckDefAndScriptFailure(['var $VAR: number'], ['E1016:', 'E475:'])

  lines =<< trim END
    vim9script
    def SomeFunc()
      s:var = 123
    enddef
    defcompile
  END
  v9.CheckScriptFailure(lines, 'E1268:')

  g:inc_counter += 1
  assert_equal(2, g:inc_counter)

  var f: float
  f += 1
  assert_equal(1.0, f)

  $SOME_ENV_VAR ..= 'more'
  assert_equal('somemore', $SOME_ENV_VAR)
  v9.CheckDefFailure(['$SOME_ENV_VAR += "more"'], 'E1051:')
  v9.CheckDefFailure(['$SOME_ENV_VAR += 123'], 'E1012:')

  v:errmsg = 'none'
  v:errmsg ..= 'again'
  assert_equal('noneagain', v:errmsg)
  v9.CheckDefFailure(['v:errmsg += "more"'], 'E1051:')
  v9.CheckDefFailure(['v:errmsg += 123'], 'E1012:')

  var text =<< trim END
    some text
  END
enddef

def Test_float_and_number()
  var lines =<< trim END
       var f: float
       f += 2
       f -= 1
       assert_equal(1.0, f)
       ++f
       --f
       assert_equal(1.0, f)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

let g:someNumber = 43

def Test_assign_concat()
  var lines =<< trim END
    var s = '-'
    s ..= 99
    s ..= true
    s ..= '-'
    s ..= v:null
    s ..= g:someNumber
    assert_equal('-99true-null43', s)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
    var s = '-'
    s ..= [1, 2]
  END
  v9.CheckDefAndScriptFailure(lines, ['E1105: Cannot convert list to string', 'E734: Wrong variable type for .='], 2)
  lines =<< trim END
    var s = '-'
    s ..= {a: 2}
  END
  v9.CheckDefAndScriptFailure(lines, ['E1105: Cannot convert dict to string', 'E734: Wrong variable type for .='], 2)

  lines =<< trim END
      var ls: list<string> = []
      ls[-1] ..= 'foo'
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E684: List index out of range: -1', 2)
enddef

def Test_assign_register()
  var lines =<< trim END
    @c = 'areg'
    @c ..= 'add'
    assert_equal('aregadd', @c)

    @@ = 'some text'
    assert_equal('some text', getreg('"'))
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefFailure(['@a += "more"'], 'E1051:')
  v9.CheckDefFailure(['@a += 123'], 'E1012:')
enddef

def Test_reserved_name()
  var more_names = ['null_job', 'null_channel']
  if !has('job')
    more_names = []
  endif

  for name in ['true',
               'false',
               'this',
               'super',
               'null',
               'null_blob',
               'null_dict',
               'null_function',
               'null_list',
               'null_partial',
               'null_string',
               ] + more_names
    v9.CheckDefExecAndScriptFailure(['var ' .. name .. ' =  0'], 'E1034:')
    v9.CheckDefExecAndScriptFailure(['var ' .. name .. ': bool'], 'E1034:')
  endfor

  var lines =<< trim END
      vim9script
      def Foo(super: bool)
	echo 'something'
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1034:')
enddef

def Test_null_values()
  var lines =<< trim END
      var b: blob = null_blob
      var dn: dict<number> = null_dict
      var ds: dict<string> = null_dict
      var ln: list<number> = null_list
      var ls: list<string> = null_list
      var Ff: func(string): string = null_function
      var Fp: func(number): number = null_partial
      var s: string = null_string
      if has('job')
        var j: job = null_job
        var c: channel = null_channel
      endif

      var d: dict<func> = {a: function('tr'), b: null_function}

      var bl: list<blob> = [0z12, null_blob]
      var dnl: list<dict<number>> = [{a: 1}, null_dict]
      var dsl: list<dict<string>> = [{a: 'x'}, null_dict]
      var lnl: list<list<number>> = [[1], null_list]
      var lsl: list<list<string>> = [['x'], null_list]
      def Len(v: string): number
        return len(v)
      enddef
      var Ffl: list<func(string): number> = [Len, null_function]
      var Fpl: list<func(string): number> = [Len, null_partial]
      var sl: list<string> = ['x', null_string]
      if has('job')
        var jl: list<job> = [null_job]
        var cl: list<channel> = [null_channel]
      endif
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_type_with_extra_white()
  var lines =<< trim END
      const x : number = 3
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1059')
enddef

def Test_keep_type_after_assigning_null()
  var lines =<< trim END
      var b: blob
      b = null_blob
      b = 'text'
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1012: Type mismatch; expected blob but got string')

  lines =<< trim END
      var l: list<number>
      l = null_list
      l = ['text']
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1012: Type mismatch; expected list<number> but got list<string>')

  lines =<< trim END
      var d: dict<string>
      d = null_dict
      d = {a: 1, b: 2}
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1012: Type mismatch; expected dict<string> but got dict<number>')
enddef

def Test_skipped_assignment()
  var lines =<< trim END
      for x in []
        var i: number = 1
        while false
          i += 1
        endwhile
      endfor
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_assign_keep_type()
  var lines =<< trim END
      vim9script
      var l: list<number> = [123]
      l = [123]
      l->add('string')
  END
  v9.CheckScriptFailure(lines, 'E1012:', 4)
enddef

def Test_assign_unpack()
  var lines =<< trim END
    var v1: number
    var v2: number
    [v1, v2] = [1, 2]
    assert_equal(1, v1)
    assert_equal(2, v2)

    [v1, _, v2, _] = [1, 99, 2, 77]
    assert_equal(1, v1)
    assert_equal(2, v2)

    [v1, v2; _] = [1, 2, 3, 4, 5]
    assert_equal(1, v1)
    assert_equal(2, v2)

    var _x: number
    [_x, v2] = [6, 7]
    assert_equal(6, _x)
    assert_equal(7, v2)

    var reslist = []
    for text in ['aaa {bbb} ccc', 'ddd {eee} fff']
      var before: string
      var middle: string
      var after: string
      [_, before, middle, after; _] = text->matchlist('\(.\{-\}\){\(.\{-\}\)}\(.*\)')
      reslist->add(before)->add(middle)->add(after)
    endfor
    assert_equal(['aaa ', 'bbb', ' ccc', 'ddd ', 'eee', ' fff'], reslist)

    var a = 1
    var b = 3
    [a, b] += [2, 4]
    assert_equal(3, a)
    assert_equal(7, b)

    [a, b] -= [1, 2]
    assert_equal(2, a)
    assert_equal(5, b)

    [a, b] *= [3, 2]
    assert_equal(6, a)
    assert_equal(10, b)

    [a, b] /= [2, 4]
    assert_equal(3, a)
    assert_equal(2, b)

    [a, b] = [17, 15]
    [a, b] %= [5, 3]
    assert_equal(2, a)
    assert_equal(0, b)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var v1: number
      var v2: number
      [v1, v2] = [1, 2, 3]
  END
  v9.CheckDefFailure(lines, 'E1093: Expected 2 items but got 3', 3)

  lines =<< trim END
      var v1: number
      var v2: number
      [v1, v2] = [1]
  END
  v9.CheckDefFailure(lines, 'E1093: Expected 2 items but got 1', 3)

  lines =<< trim END
      var v1: number
      var v2: number
      [v1, v2; _] = [1]
  END
  v9.CheckDefFailure(lines, 'E1093: Expected 2 items but got 1', 3)

  lines =<< trim END
      var v1: number
      var v2: number
      [v1, v2] = 
  END
  v9.CheckDefFailure(lines, 'E1097:', 5)

  lines =<< trim END
      var v1: number
      var v2: number
      [v1, v2] = xxx
  END
  v9.CheckDefFailure(lines, 'E1001:', 3)

  lines =<< trim END
      var v1: number
      var v2: number
      [v1, v2] = popup_clear()
  END
  v9.CheckDefFailure(lines, 'E1031:', 3)

  lines =<< trim END
      [v1, v2] = [1, 2]
  END
  v9.CheckDefFailure(lines, 'E1089', 1)
  v9.CheckScriptFailure(['vim9script'] + lines, 'E1089', 2)

  lines =<< trim END
      var v1: number
      var v2: number
      [v1, v2] = ''
  END
  v9.CheckDefFailure(lines, 'E1012: Type mismatch; expected list<any> but got string', 3)

  lines =<< trim END
    g:values = [false, 0]
    var x: bool
    var y: string
    [x, y] = g:values
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1163: Variable 2: type mismatch, expected string but got number')

  lines =<< trim END
    var x: number
    var y: number
    var z: string
    [x, y, z] = [1, 2, 3]
  END
  v9.CheckDefAndScriptFailure(lines, 'E1163: Variable 3: type mismatch, expected string but got number')

  lines =<< trim END
    var x: number
    var y: string
    var z: string
    [x, y, z] = [1, '2', 3]
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1163: Variable 3: type mismatch, expected string but got number')
enddef

def Test_assign_linebreak()
  var nr: number
  nr =
      123
  assert_equal(123, nr)

  var n2: number
  [nr, n2] =
     [12, 34]
  assert_equal(12, nr)
  assert_equal(34, n2)

  v9.CheckDefFailure(["var x = #"], 'E1097:', 3)

  var lines =<< trim END
      var x: list<string> = ['a']
      var y: list<number> = x
          ->copy()
          ->copy()
  END
  v9.CheckDefExecFailure(lines, 'E1012:', 4)

  lines =<< trim END
      var x: any
      x.key = 1
          + 2
          + 3
          + 4
          + 5
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1148:', 'E1203:'], 2)
enddef

def Test_assign_index()
  # list of list
  var l1: list<number>
  l1[0] = 123
  assert_equal([123], l1)

  var l2: list<list<number>>
  l2[0] = []
  l2[0][0] = 123
  assert_equal([[123]], l2)

  var l3: list<list<list<number>>>
  l3[0] = []
  l3[0][0] = []
  l3[0][0][0] = 123
  assert_equal([[[123]]], l3)

  var lines =<< trim END
      var l3: list<list<number>>
      l3[0] = []
      l3[0][0] = []
  END
  v9.CheckDefFailure(lines, 'E1012: Type mismatch; expected number but got list<any>', 3)

  # dict of dict
  var d1: dict<number>
  d1.one = 1
  assert_equal({one: 1}, d1)

  var d2: dict<dict<number>>
  d2.one = {}
  d2.one.two = 123
  assert_equal({one: {two: 123}}, d2)

  var d3: dict<dict<dict<number>>>
  d3.one = {}
  d3.one.two = {}
  d3.one.two.three = 123
  assert_equal({one: {two: {three: 123}}}, d3)

  # blob
  var bl: blob = 0z11223344
  bl[0] = 0x77
  assert_equal(0z77223344, bl)
  bl[-2] = 0x66
  assert_equal(0z77226644, bl)

  lines =<< trim END
      g:val = '22'
      var bl = 0z11
      bl[1] = g:val
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1030: Using a String as a Number: "22"')

  # should not read the next line when generating "a.b"
  var a = {}
  a.b = {}
  a.b.c = {}
          ->copy()

  lines =<< trim END
      var d3: dict<dict<number>>
      d3.one = {}
      d3.one.two = {}
  END
  v9.CheckDefFailure(lines, 'E1012: Type mismatch; expected number but got dict<any>', 3)

  lines =<< trim END
    var lines: list<string>
    lines['a'] = 'asdf'
  END
  v9.CheckDefFailure(lines, 'E1012:', 2)

  lines =<< trim END
    var lines: string
    lines[9] = 'asdf'
  END
  v9.CheckDefFailure(lines, 'E1141:', 2)

  # list of dict
  var ld: list<dict<number>>
  ld[0] = {}
  ld[0].one = 123
  assert_equal([{one: 123}], ld)

  lines =<< trim END
      var ld: list<dict<number>>
      ld[0] = []
  END
  v9.CheckDefFailure(lines, 'E1012: Type mismatch; expected dict<number> but got list<any>', 2)

  # dict of list
  var dl: dict<list<number>>
  dl.one = []
  dl.one[0] = 123
  assert_equal({one: [123]}, dl)

  lines =<< trim END
      var dl: dict<list<number>>
      dl.one = {}
  END
  v9.CheckDefFailure(lines, 'E1012: Type mismatch; expected list<number> but got dict<any>', 2)

  lines =<< trim END
      g:l = [1, 2]
      g:l['x'] = 3
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E39:', 'E1030:'], 2)

  lines =<< trim END
    var bl: blob = test_null_blob()
    bl[1] = 8
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1184:', 'E979:'], 2)

  lines =<< trim END
    g:bl = 'not a blob'
    g:bl[1 : 2] = 8
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E897:', 'E689:'], 2)
enddef

def Test_init_in_for_loop()
  var lines =<< trim END
      var l: list<number> = []
      for i in [3, 4]
        var n: number
        add(l, n)
        n = 123
      endfor
      assert_equal([0, 0], l)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var l: list<number> = []
      for i in [3, 4]
        var n: number = 0
        add(l, n)
        n = 123
      endfor
      assert_equal([0, 0], l)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var l: list<number> = []
      for i in [3, 4]
        var n: number = 3
        add(l, n)
        n = 123
      endfor
      assert_equal([3, 3], l)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_redir_is_not_assign()
  if false
    redir => res
    echo var_job
    redir END
  endif
enddef

def Test_extend_list()
  # using uninitialized list assigns empty list
  var lines =<< trim END
      var l1: list<number>
      var l2 = l1
      assert_true(l1 is l2)
      l1 += [123]
      assert_equal([123], l1)
      assert_true(l1 is l2)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var list: list<string>
      extend(list, ['x'])
      assert_equal(['x'], list)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # appending to uninitialized list from a function works
  lines =<< trim END
      vim9script
      var list: list<string>
      def Func()
        list += ['a', 'b']
      enddef
      Func()
      assert_equal(['a', 'b'], list)
  END
  v9.CheckScriptSuccess(lines)
  lines =<< trim END
      vim9script
      var list: list<string>
      def Func()
        extend(list, ['x', 'b'])
      enddef
      Func()
      assert_equal(['x', 'b'], list)
  END
  v9.CheckScriptSuccess(lines)

  # initialized to null, with type, does not default to empty list
  lines =<< trim END
      vim9script
      var l: list<string> = test_null_list()
      extend(l, ['x'])
  END
  v9.CheckScriptFailure(lines, 'E1134:', 3)

  # initialized to null, without type, does not default to empty list
  lines =<< trim END
      vim9script
      var l = null_list
      extend(l, ['x'])
  END
  v9.CheckScriptFailure(lines, 'E1134:', 3)

  # assigned null, does not default to empty list
  lines =<< trim END
      vim9script
      var l: list<string>
      l = null_list
      extend(l, ['x'])
  END
  v9.CheckScriptFailure(lines, 'E1134:', 4)

  lines =<< trim END
      vim9script
      extend(test_null_list(), ['x'])
  END
  v9.CheckScriptFailure(lines, 'E1134:', 2)

  # using global var has no declared type
  g:myList = []
  g:myList->extend([1])
  g:myList->extend(['x'])
  assert_equal([1, 'x'], g:myList)
  unlet g:myList

  # using declared list gives an error
  lines =<< trim END
      var l: list<number>
      g:myList = l
      g:myList->extend([1])
      g:myList->extend(['x'])
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected list<number> but got list<string>', 4)
  unlet g:myList

  lines =<< trim END
      vim9script
      var lds = [1, 2, 3]
      def Func()
          echo lds->extend(['x'])
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1013:')
enddef

def Test_extend_dict()
  var lines =<< trim END
      vim9script
      var d: dict<number>
      extend(d, {a: 1})
      assert_equal({a: 1}, d)

      var d2: dict<number>
      d2['one'] = 1
      assert_equal({one: 1}, d2)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      var d: dict<string> = test_null_dict()
      extend(d, {a: 'x'})
  END
  v9.CheckScriptFailure(lines, 'E1133:', 3)

  lines =<< trim END
      vim9script
      extend(test_null_dict(), {a: 'x'})
  END
  v9.CheckScriptFailure(lines, 'E1133:', 2)
enddef

def Test_single_letter_vars()
  # single letter variables
  var a: number = 123
  a = 123
  assert_equal(123, a)
  var b: number
  b = 123
  assert_equal(123, b)
  var g: number
  g = 123
  assert_equal(123, g)
  var s: number
  s = 123
  assert_equal(123, s)
  var t: number
  t = 123
  assert_equal(123, t)
  var v: number
  v = 123
  assert_equal(123, v)
  var w: number
  w = 123
  assert_equal(123, w)
enddef

def Test_vim9_single_char_vars()
  var lines =<< trim END
      vim9script

      # single character variable declarations work
      var a: string
      var b: number
      var l: list<any>
      var s: string
      var t: number
      var v: number
      var w: number

      # script-local variables can be used without s: prefix
      a = 'script-a'
      b = 111
      l = [1, 2, 3]
      s = 'script-s'
      t = 222
      v = 333
      w = 444

      assert_equal('script-a', a)
      assert_equal(111, b)
      assert_equal([1, 2, 3], l)
      assert_equal('script-s', s)
      assert_equal(222, t)
      assert_equal(333, v)
      assert_equal(444, w)
  END
  writefile(lines, 'Xsinglechar', 'D')
  source Xsinglechar
enddef

def Test_assignment_list()
  var list1: list<bool> = [false, true, false]
  var list2: list<number> = [1, 2, 3]
  var list3: list<string> = ['sdf', 'asdf']
  var list4: list<any> = ['yes', true, 1234]
  var list5: list<blob> = [0z01, 0z02]

  var listS: list<string> = []
  var listN: list<number> = []

  assert_equal([1, 2, 3], list2)
  list2[-1] = 99
  assert_equal([1, 2, 99], list2)
  list2[-2] = 88
  assert_equal([1, 88, 99], list2)
  list2[-3] = 77
  assert_equal([77, 88, 99], list2)
  list2 += [100]
  assert_equal([77, 88, 99, 100], list2)

  list3 += ['end']
  assert_equal(['sdf', 'asdf', 'end'], list3)

  v9.CheckDefExecFailure(['var ll = [1, 2, 3]', 'll[-4] = 6'], 'E684:')
  v9.CheckDefExecFailure(['var ll = [1, 2, 3]', 'unlet ll[8 : 9]'], 'E684:')
  v9.CheckDefExecFailure(['var ll = [1, 2, 3]', 'unlet ll[1 : -9]'], 'E684:')
  v9.CheckDefExecFailure(['var ll = [1, 2, 3]', 'unlet ll[2 : 1]'], 'E684:')

  # type becomes list<any>
  var somelist = rand() > 0 ? [1, 2, 3] : ['a', 'b', 'c']

  # type is list<any> even though initializer is list<number>
  var anyList: list<any> = [0]
  assert_equal([0, 'x'], extend(anyList, ['x']))

  var lines =<< trim END
    var d = {dd: test_null_list()}
    d.dd[0] = 0
  END
  v9.CheckDefExecFailure(lines, 'E1147:', 2)

  lines =<< trim END
      def OneArg(x: bool)
      enddef
      def TwoArgs(x: bool, y: bool)
      enddef
      var fl: list<func(bool, bool, bool)> = [OneArg, TwoArgs]
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1012:', 5)
enddef

def Test_list_declaration()
  var [v1, v2] = [1, 2]
  v1 += 3
  assert_equal(4, v1)
  v2 *= 3
  assert_equal(6, v2)

  var lines =<< trim END
      var [v1, v2] = [1]
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1093: Expected 2 items but got 1', 'E688:'])
  lines =<< trim END
      var testlist = [1]
      var [v1, v2] = testlist
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1093: Expected 2 items but got 1', 'E688:'])
  lines =<< trim END
      var [v1, v2] = [1, 2, 3]
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1093: Expected 2 items but got 3', 'E687:'])
  lines =<< trim END
      var testlist = [1, 2, 3]
      var [v1, v2] = testlist
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1093: Expected 2 items but got 3', 'E687:'])

  var [vnr, vstr] = [123, 'text']
  vnr += 3
  assert_equal(126, vnr)
  vstr ..= 'end'
  assert_equal('textend', vstr)

  var [vnr2: number, vstr2: string] = [123, 'text']
  vnr2 += 3
  assert_equal(126, vnr2)
  vstr2 ..= 'end'
  assert_equal('textend', vstr2)

  var [vnr3: number; vlist: list<string>] = [123, 'foo', 'bar']
  vnr3 += 5
  assert_equal(128, vnr3)
  assert_equal(['foo', 'bar'], vlist)

  lines =<< trim END
      var [vnr2: number, vstr2: number] = [123, 'text']
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1163: Variable 2: type mismatch, expected number but got string', 'E1012: Type mismatch; expected number but got string'])
  lines =<< trim END
      var testlist = [234, 'text']
      var [vnr2: number, vstr2: number] = testlist
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1163: Variable 2: type mismatch, expected number but got string', 'E1012: Type mismatch; expected number but got string'])
enddef

def PartFuncBool(b: bool): string
  return 'done'
enddef

def Test_assignment_partial()
  var lines =<< trim END
      var Partial: func(): string = function(g:PartFuncBool, [true])
      assert_equal('done', Partial())
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Func(b: bool)
      enddef
      var Ref: func = function(Func, [true])
      assert_equal('func()', typename(Ref))
      Ref()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script

      var nres: any
      var sres: any
      def Func(nr: number, s = '')
        nres = nr
        sres = s
      enddef

      var n: number
      var Ref = function(Func, [n])
      Ref('x')
      assert_equal(0, nres)
      assert_equal('x', sres)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script

      def Func(nr: number, s = '')
      enddef

      var n: number
      var Ref = function(Func, [n])
      Ref(0)
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected string but got number')
enddef

def Test_assignment_list_any_index()
   var l: list<number> = [1, 2]
  for  [x, y, _]
  in  [[0, 1, ''], [1, 3, '']]
      l[x] = l[x] + y
  endfor
  assert_equal([2, 5], l)
enddef

def Test_assignment_list_vim9script()
  var lines =<< trim END
    vim9script
    var v1: number
    var v2: number
    var v3: number
    [v1, v2, v3] = [1, 2, 3]
    assert_equal([1, 2, 3], [v1, v2, v3])
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_assignment_dict()
  var dict1: dict<bool> = {one: false, two: true}
  var dict2: dict<number> = {one: 1, two: 2}
  var dict3: dict<string> = {key: 'value'}
  var dict4: dict<any> = {one: 1, two: '2'}
  var dict5: dict<blob> = {one: 0z01, two: 0z02}

  # check the type is OK
  var events: dict<string> = v:event

  # overwrite
  dict3['key'] = 'another'
  assert_equal(dict3, {key: 'another'})
  dict3.key = 'yet another'
  assert_equal(dict3, {key: 'yet another'})

  # member "any" can also be a dict and assigned to
  var anydict: dict<any> = {nest: {}, nr: 0}
  anydict.nest['this'] = 123
  anydict.nest.that = 456
  assert_equal({nest: {this: 123, that: 456}, nr: 0}, anydict)

  var lines =<< trim END
    var dd = {}
    dd.two = 2
    assert_equal({two: 2}, dd)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
    var d = {dd: {}}
    d.dd[0] = 2
    d.dd['x'] = 3
    d.dd.y = 4
    assert_equal({dd: {0: 2, x: 3, y: 4}}, d)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
    var key = 'foo'
    g:[key] = 'value'
    assert_equal('value', g:foo)
    unlet g:foo
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
    var dd = {one: 1}
    dd.one) = 2
  END
  v9.CheckDefFailure(lines, 'E488:', 2)

  lines =<< trim END
    var dd = {one: 1}
    var dd.one = 2
  END
  v9.CheckDefAndScriptFailure(lines, 'E1017:', 2)

  # empty key can be used
  var dd = {}
  dd[""] = 6
  assert_equal({['']: 6}, dd)

  # type becomes dict<any>
  var somedict = rand() > 0 ? {a: 1, b: 2} : {a: 'a', b: 'b'}

  # type is dict<any> even though initializer is dict<number>
  var anyDict: dict<any> = {a: 0}
  assert_equal({a: 0, b: 'x'}, extend(anyDict, {b: 'x'}))

  # using global var, which has no declared type
  g:myDict = {}
  g:myDict->extend({a: 1})
  g:myDict->extend({b: 'x'})
  assert_equal({a: 1, b: 'x'}, g:myDict)
  unlet g:myDict

  # using list with declared type gives an error
  lines =<< trim END
      var d: dict<number>
      g:myDict = d
      g:myDict->extend({a: 1})
      g:myDict->extend({b: 'x'})
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected dict<number> but got dict<string>', 4)
  unlet g:myDict

  # assignment to script-local dict
  lines =<< trim END
    vim9script
    var test: dict<any> = {}
    def FillDict(): dict<any>
      test['a'] = 43
      return test
    enddef
    assert_equal({a: 43}, FillDict())
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    vim9script
    var test: dict<any>
    def FillDict(): dict<any>
      test['a'] = 43
      return test
    enddef
    FillDict()
    assert_equal({a: 43}, test)
  END
  v9.CheckScriptSuccess(lines)

  # assignment to global dict
  lines =<< trim END
    vim9script
    g:test = {}
    def FillDict(): dict<any>
      g:test['a'] = 43
      return g:test
    enddef
    assert_equal({a: 43}, FillDict())
  END
  v9.CheckScriptSuccess(lines)

  # assignment to buffer dict
  lines =<< trim END
    vim9script
    b:test = {}
    def FillDict(): dict<any>
      b:test['a'] = 43
      return b:test
    enddef
    assert_equal({a: 43}, FillDict())
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    var d = {dd: test_null_dict()}
    d.dd[0] = 0
  END
  v9.CheckDefExecFailure(lines, 'E1103:', 2)

  lines =<< trim END
    var d = {dd: 'string'}
    d.dd[0] = 0
  END
  v9.CheckDefExecFailure(lines, 'E1148:', 2)

  lines =<< trim END
    var n: any
    n.key = 5
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1148:', 'E1203: Dot not allowed after a number: n.key = 5'], 2)
enddef

def Test_assignment_local()
  # Test in a separated file in order not to the current buffer/window/tab is
  # changed.
  var script_lines: list<string> =<< trim END
    let b:existing = 'yes'
    let w:existing = 'yes'
    let t:existing = 'yes'

    def Test_assignment_local_internal()
      b:newvar = 'new'
      assert_equal('new', b:newvar)
      assert_equal('yes', b:existing)
      b:existing = 'no'
      assert_equal('no', b:existing)
      b:existing ..= 'NO'
      assert_equal('noNO', b:existing)

      w:newvar = 'new'
      assert_equal('new', w:newvar)
      assert_equal('yes', w:existing)
      w:existing = 'no'
      assert_equal('no', w:existing)
      w:existing ..= 'NO'
      assert_equal('noNO', w:existing)

      t:newvar = 'new'
      assert_equal('new', t:newvar)
      assert_equal('yes', t:existing)
      t:existing = 'no'
      assert_equal('no', t:existing)
      t:existing ..= 'NO'
      assert_equal('noNO', t:existing)
    enddef
    call Test_assignment_local_internal()
  END
  v9.CheckScriptSuccess(script_lines)
enddef

def Test_assignment_default()
  # Test default values.
  var thebool: bool
  assert_equal(v:false, thebool)

  var thenumber: number
  assert_equal(0, thenumber)

  var thefloat: float
  assert_equal(0.0, thefloat)

  var thestring: string
  assert_equal('', thestring)

  var theblob: blob
  assert_equal(0z, theblob)

  var Thefunc: func
  assert_equal(test_null_function(), Thefunc)

  var thelist: list<any>
  assert_equal([], thelist)

  var thedict: dict<any>
  assert_equal({}, thedict)

  if has('channel')
    var thejob: job
    assert_equal(test_null_job(), thejob)

    var thechannel: channel
    assert_equal(test_null_channel(), thechannel)

    if has('unix') && executable('cat')
      # check with non-null job and channel, types must match
      thejob = job_start("cat ", {})
      thechannel = job_getchannel(thejob)
      job_stop(thejob, 'kill')
    endif
  endif

  var nr = 1234 | nr = 5678
  assert_equal(5678, nr)
enddef

def Test_script_var_default()
  var lines =<< trim END
      vim9script
      var l: list<number>
      var li = [1, 2]
      var bl: blob
      var bli = 0z12
      var d: dict<number>
      var di = {'a': 1, 'b': 2}
      def Echo()
        assert_equal([], l)
        assert_equal([1, 2], li)
        assert_equal(0z, bl)
        assert_equal(0z12, bli)
        assert_equal({}, d)
        assert_equal({'a': 1, 'b': 2}, di)
      enddef
      Echo()
  END
  v9.CheckScriptSuccess(lines)
enddef

let s:scriptvar = 'init'

def Test_assignment_var_list()
  var lines =<< trim END
      var v1: string
      var v2: string
      var vrem: list<string>
      [v1] = ['aaa']
      assert_equal('aaa', v1)

      [v1, v2] = ['one', 'two']
      assert_equal('one', v1)
      assert_equal('two', v2)

      [v1, v2; vrem] = ['one', 'two']
      assert_equal('one', v1)
      assert_equal('two', v2)
      assert_equal([], vrem)

      [v1, v2; vrem] = ['one', 'two', 'three']
      assert_equal('one', v1)
      assert_equal('two', v2)
      assert_equal(['three'], vrem)

      [&ts, &sw] = [3, 4]
      assert_equal(3, &ts)
      assert_equal(4, &sw)
      set ts=8 sw=4

      [@a, @z] = ['aa', 'zz']
      assert_equal('aa', @a)
      assert_equal('zz', @z)

      [$SOME_VAR, $OTHER_VAR] = ['some', 'other']
      assert_equal('some', $SOME_VAR)
      assert_equal('other', $OTHER_VAR)

      [g:globalvar, b:bufvar, w:winvar, t:tabvar, v:errmsg] =
            ['global', 'buf', 'win', 'tab', 'error']
      assert_equal('global', g:globalvar)
      assert_equal('buf', b:bufvar)
      assert_equal('win', w:winvar)
      assert_equal('tab', t:tabvar)
      assert_equal('error', v:errmsg)
      unlet g:globalvar
  END
  v9.CheckDefAndScriptSuccess(lines)

  [g:globalvar, scriptvar, b:bufvar] = ['global', 'script', 'buf']
  assert_equal('global', g:globalvar)
  assert_equal('script', scriptvar)
  assert_equal('buf', b:bufvar)

  lines =<< trim END
      vim9script
      var scriptvar = 'init'
      [g:globalvar, scriptvar, w:winvar] = ['global', 'script', 'win']
      assert_equal('global', g:globalvar)
      assert_equal('script', scriptvar)
      assert_equal('win', w:winvar)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_assignment_empty_list()
  var lines =<< trim END
      var l2: list<any> = []
      var l: list<string>
      l = l2
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_assignment_vim9script()
  var lines =<< trim END
    vim9script
    def Func(): list<number>
      return [1, 2]
    enddef
    var name1: number
    var name2: number
    [name1, name2] =
          Func()
    assert_equal(1, name1)
    assert_equal(2, name2)
    var ll =
          Func()
    assert_equal([1, 2], ll)

    @/ = 'text'
    assert_equal('text', @/)
    @0 = 'zero'
    assert_equal('zero', @0)
    @1 = 'one'
    assert_equal('one', @1)
    @9 = 'nine'
    assert_equal('nine', @9)
    @- = 'minus'
    assert_equal('minus', @-)
    if has('clipboard_working')
      @* = 'star'
      assert_equal('star', @*)
      @+ = 'plus'
      assert_equal('plus', @+)
    endif

    var a: number = 123
    assert_equal(123, a)
    var s: string = 'yes'
    assert_equal('yes', s)
    var b: number = 42
    assert_equal(42, b)
    var w: number = 43
    assert_equal(43, w)
    var t: number = 44
    assert_equal(44, t)

    var to_var = 0
    to_var = 3
    assert_equal(3, to_var)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      var n: number
      def Func()
        n = 'string'
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1012: Type mismatch; expected number but got string')
enddef

def Mess(): string
  v:foldstart = 123
  return 'xxx'
enddef

def Test_assignment_failure()
  v9.CheckDefFailure(['var name=234'], 'E1004:')
  v9.CheckDefFailure(['var name =234'], 'E1004:')
  v9.CheckDefFailure(['var name= 234'], 'E1004:')

  v9.CheckScriptFailure(['vim9script', 'var name=234'], 'E1004:')
  v9.CheckScriptFailure(['vim9script', 'var name=234'], "before and after '='")
  v9.CheckScriptFailure(['vim9script', 'var name =234'], 'E1004:')
  v9.CheckScriptFailure(['vim9script', 'var name= 234'], 'E1004:')
  v9.CheckScriptFailure(['vim9script', 'var name = 234', 'name+=234'], 'E1004:')
  v9.CheckScriptFailure(['vim9script', 'var name = 234', 'name+=234'], "before and after '+='")
  v9.CheckScriptFailure(['vim9script', 'var name = "x"', 'name..="y"'], 'E1004:')
  v9.CheckScriptFailure(['vim9script', 'var name = "x"', 'name..="y"'], "before and after '..='")

  v9.CheckDefFailure(['var true = 1'], 'E1034:')
  v9.CheckDefFailure(['var false = 1'], 'E1034:')
  v9.CheckDefFailure(['var null = 1'], 'E1034:')
  v9.CheckDefFailure(['var this = 1'], 'E1034:')
  v9.CheckDefFailure(['var super = 1'], 'E1034:')

  v9.CheckDefFailure(['[a; b; c] = g:list'], 'E1001:')
  v9.CheckDefFailure(['var [a; b; c] = g:list'], 'E1080:')
  v9.CheckDefExecFailure(['var a: number',
                       '[a] = test_null_list()'], 'E1093:')
  v9.CheckDefExecFailure(['var a: number',
                       '[a] = []'], 'E1093:')
  v9.CheckDefExecFailure(['var x: number',
                       'var y: number',
                       '[x, y] = [1]'], 'E1093:')
  v9.CheckDefExecFailure(['var x: string',
                       'var y: string',
                       '[x, y] = ["x"]'], 'E1093:')
  v9.CheckDefExecFailure(['var x: number',
                       'var y: number',
                       'var z: list<number>',
                       '[x, y; z] = [1]'], 'E1093:')

  v9.CheckDefFailure(['var somevar'], "E1022:")
  v9.CheckDefFailure(['var &tabstop = 4'], 'E1052:')
  v9.CheckDefFailure(['&g:option = 5'], 'E113:')
  v9.CheckScriptFailure(['vim9script', 'var &tabstop = 4'], 'E1052:')

  v9.CheckDefFailure(['var $VAR = 5'], 'E1016: Cannot declare an environment variable:')
  v9.CheckScriptFailure(['vim9script', 'var $ENV = "xxx"'], 'E1016:')

  if has('dnd')
    v9.CheckDefFailure(['var @~ = 5'], 'E1066:')
  else
    v9.CheckDefFailure(['var @~ = 5'], 'E354:')
    v9.CheckDefFailure(['@~ = 5'], 'E354:')
  endif
  v9.CheckDefFailure(['var @a = 5'], 'E1066:')
  v9.CheckDefFailure(['var @/ = "x"'], 'E1066:')
  v9.CheckScriptFailure(['vim9script', 'var @a = "abc"'], 'E1066:')

  v9.CheckDefFailure(['var g:var = 5'], 'E1016: Cannot declare a global variable:')
  v9.CheckDefFailure(['var w:var = 5'], 'E1016: Cannot declare a window variable:')
  v9.CheckDefFailure(['var b:var = 5'], 'E1016: Cannot declare a buffer variable:')
  v9.CheckDefFailure(['var t:var = 5'], 'E1016: Cannot declare a tab variable:')

  v9.CheckDefFailure(['var anr = 4', 'anr ..= "text"'], 'E1019:')
  v9.CheckDefFailure(['var xnr += 4'], 'E1020:', 1)
  v9.CheckScriptFailure(['vim9script', 'var xnr += 4'], 'E1020:')
  v9.CheckDefFailure(["var xnr = xnr + 1"], 'E1001:', 1)
  v9.CheckScriptFailure(['vim9script', 'var xnr = xnr + 4'], 'E121:')

  v9.CheckScriptFailure(['vim9script', 'def Func()', 'var dummy = notfound', 'enddef', 'defcompile'], 'E1001:')

  v9.CheckDefFailure(['var name: list<string> = [123]'], 'expected list<string> but got list<number>')
  v9.CheckDefFailure(['var name: list<number> = ["xx"]'], 'expected list<number> but got list<string>')

  v9.CheckDefFailure(['var name: dict<string> = {key: 123}'], 'expected dict<string> but got dict<number>')
  v9.CheckDefFailure(['var name: dict<number> = {key: "xx"}'], 'expected dict<number> but got dict<string>')

  v9.CheckDefFailure(['var name = feedkeys("0")'], 'E1031:')
  v9.CheckDefFailure(['var name: number = feedkeys("0")'], 'expected number but got void')

  v9.CheckDefFailure(['var name: dict <number>'], 'E1068:')
  v9.CheckDefFailure(['var name: dict<number'], 'E1009: Missing > after type: <number')

  assert_fails('s/^/\=g:Mess()/n', 'E794:')
  v9.CheckDefFailure(['var name: dict<number'], 'E1009:')

  v9.CheckDefFailure(['w:foo: number = 10'],
                  'E1016: Cannot declare a window variable: w:foo')
  v9.CheckDefFailure(['t:foo: bool = true'],
                  'E1016: Cannot declare a tab variable: t:foo')
  v9.CheckDefFailure(['b:foo: string = "x"'],
                  'E1016: Cannot declare a buffer variable: b:foo')
  v9.CheckDefFailure(['g:foo: number = 123'],
                  'E1016: Cannot declare a global variable: g:foo')

  v9.CheckScriptFailure(['vim9script', 'w:foo: number = 123'],
                  'E1304: Cannot use type with this variable: w:foo:')
  v9.CheckScriptFailure(['vim9script', 't:foo: number = 123'],
                  'E1304: Cannot use type with this variable: t:foo:')
  v9.CheckScriptFailure(['vim9script', 'b:foo: number = 123'],
                  'E1304: Cannot use type with this variable: b:foo:')
  v9.CheckScriptFailure(['vim9script', 'g:foo: number = 123'],
                  'E1304: Cannot use type with this variable: g:foo:')

  v9.CheckScriptFailure(['vim9script', 'const w:FOO: number = 123'],
                  'E1304: Cannot use type with this variable: w:FOO:')
  v9.CheckScriptFailure(['vim9script', 'const t:FOO: number = 123'],
                  'E1304: Cannot use type with this variable: t:FOO:')
  v9.CheckScriptFailure(['vim9script', 'const b:FOO: number = 123'],
                  'E1304: Cannot use type with this variable: b:FOO:')
  v9.CheckScriptFailure(['vim9script', 'const g:FOO: number = 123'],
                  'E1304: Cannot use type with this variable: g:FOO:')
enddef

def Test_assign_list()
  var lines =<< trim END
      var l: list<string> = []
      l[0] = 'value'
      assert_equal('value', l[0])

      l[1] = 'asdf'
      assert_equal('value', l[0])
      assert_equal('asdf', l[1])
      assert_equal('asdf', l[-1])
      assert_equal('value', l[-2])

      var nrl: list<number> = []
      for i in range(5)
        nrl[i] = i
      endfor
      assert_equal([0, 1, 2, 3, 4], nrl)

      var ul: list<any>
      ul[0] = 1
      ul[1] = 2
      ul[2] = 3
      assert_equal([1, 2, 3], ul)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var l = [1, 2]
      g:idx = 'x'
      l[g:idx : 1] = [0]
      echo l
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1012: Type mismatch; expected number but got string', 'E1030: Using a String as a Number: "x"'])

  lines =<< trim END
      var l = [1, 2]
      g:idx = 3
      l[g:idx : 1] = [0]
      echo l
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E684: List index out of range: 3')

  lines =<< trim END
      var l = [1, 2]
      g:idx = 'y'
      l[1 : g:idx] = [0]
      echo l
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E1012: Type mismatch; expected number but got string', 'E1030: Using a String as a Number: "y"'])

  v9.CheckDefFailure(["var l: list<number> = ['', true]"], 'E1012: Type mismatch; expected list<number> but got list<any>', 1)
  v9.CheckDefFailure(["var l: list<list<number>> = [['', true]]"], 'E1012: Type mismatch; expected list<list<number>> but got list<list<any>>', 1)
enddef

def Test_assign_dict()
  var lines =<< trim END
      var d: dict<string> = {}
      d['key'] = 'value'
      assert_equal('value', d['key'])

      d[123] = 'qwerty'
      assert_equal('qwerty', d[123])
      assert_equal('qwerty', d['123'])

      var nrd: dict<number> = {}
      for i in range(3)
        nrd[i] = i
      endfor
      assert_equal({0: 0, 1: 1, 2: 2}, nrd)

      d.somekey = 'someval'
      assert_equal({key: 'value', '123': 'qwerty', somekey: 'someval'}, d)
      unlet d.somekey
      assert_equal({key: 'value', '123': 'qwerty'}, d)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefFailure(["var d: dict<number> = {a: '', b: true}"], 'E1012: Type mismatch; expected dict<number> but got dict<any>', 1)
  v9.CheckDefFailure(["var d: dict<dict<number>> = {x: {a: '', b: true}}"], 'E1012: Type mismatch; expected dict<dict<number>> but got dict<dict<any>>', 1)
  v9.CheckDefFailure(["var d = {x: 1}", "d[1 : 2] = {y: 2}"], 'E1165: Cannot use a range with an assignment: d[1 : 2] =', 2)
enddef

def Test_assign_dict_unknown_type()
  var lines =<< trim END
      vim9script
      var mylist = []
      mylist += [{one: 'one'}]
      def Func()
        var dd = mylist[0]
        assert_equal('one', dd.one)
      enddef
      Func()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      var mylist = [[]]
      mylist[0] += [{one: 'one'}]
      def Func()
        var dd = mylist[0][0]
        assert_equal('one', dd.one)
      enddef
      Func()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_assign_dict_with_op()
  var lines =<< trim END
    var ds: dict<string> = {a: 'x'}
    ds['a'] ..= 'y'
    ds.a ..= 'z'
    assert_equal('xyz', ds.a)

    var dn: dict<number> = {a: 9}
    dn['a'] += 2
    assert_equal(11, dn.a)
    dn.a += 2
    assert_equal(13, dn.a)

    dn['a'] -= 3
    assert_equal(10, dn.a)
    dn.a -= 2
    assert_equal(8, dn.a)

    dn['a'] *= 2
    assert_equal(16, dn.a)
    dn.a *= 2
    assert_equal(32, dn.a)

    dn['a'] /= 3
    assert_equal(10, dn.a)
    dn.a /= 2
    assert_equal(5, dn.a)

    dn['a'] %= 3
    assert_equal(2, dn.a)
    dn.a %= 6
    assert_equal(2, dn.a)

    var dd: dict<dict<list<any>>>
    dd.a = {}
    dd.a.b = [0]
    dd.a.b += [1]
    assert_equal({a: {b: [0, 1]}}, dd)

    var dab = {a: ['b']}
    dab.a[0] ..= 'c'
    assert_equal({a: ['bc']}, dab)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_assign_list_with_op()
  var lines =<< trim END
    var ls: list<string> = ['x']
    ls[0] ..= 'y'
    assert_equal('xy', ls[0])

    var ln: list<number> = [9]
    ln[0] += 2
    assert_equal(11, ln[0])

    ln[0] -= 3
    assert_equal(8, ln[0])

    ln[0] *= 2
    assert_equal(16, ln[0])

    ln[0] /= 3
    assert_equal(5, ln[0])

    ln[0] %= 3
    assert_equal(2, ln[0])
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_assign_with_op_fails()
  var lines =<< trim END
      var s = 'abc'
      s[1] += 'x'
  END
  v9.CheckDefAndScriptFailure(lines, ['E1141:', 'E689:'], 2)

  lines =<< trim END
      var s = 'abc'
      s[1] ..= 'x'
  END
  v9.CheckDefAndScriptFailure(lines, ['E1141:', 'E689:'], 2)

  lines =<< trim END
      var dd: dict<dict<list<any>>>
      dd.a = {}
      dd.a.b += [1]
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E716:', 3)
enddef

def Test_assign_lambda()
  # check if assign a lambda to a variable which type is func or any.
  var lines =<< trim END
      vim9script
      var FuncRef = () => 123
      assert_equal(123, FuncRef())
      var FuncRef_Func: func = () => 123
      assert_equal(123, FuncRef_Func())
      var FuncRef_Any: any = () => 123
      assert_equal(123, FuncRef_Any())
      var FuncRef_Number: func(): number = () => 321
      assert_equal(321, FuncRef_Number())
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      var Ref: func(number)
      Ref = (j) => !j
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012: Type mismatch; expected func(number) but got func(any): bool')

  lines =<< trim END
      echo filter([1, 2, 3], (_, v: string) => v + 1)
  END
  v9.CheckDefAndScriptFailure(lines, 'E1051:')
enddef

def Test_assign_funcref_args()
  # unspecified arguments match everything, including varargs
  var lines =<< trim END
    vim9script

    var FuncUnknown: func: number

    FuncUnknown = (v): number => v
    assert_equal(5, FuncUnknown(5))

    FuncUnknown = (v1, v2): number => v1 + v2
    assert_equal(7, FuncUnknown(3, 4))

    FuncUnknown = (...v1): number => v1[0] + v1[1] + len(v1) * 1000
    assert_equal(4007, FuncUnknown(3, 4, 5, 6))

    FuncUnknown = (v: list<any>): number => v[0] + v[1] + len(v) * 1000
    assert_equal(5009, FuncUnknown([4, 5, 6, 7, 8]))
  END
  v9.CheckScriptSuccess(lines)

  # varargs must match
  lines =<< trim END
    vim9script
    var FuncAnyVA: func(...any): number
    FuncAnyVA = (v): number => v
  END
  v9.CheckScriptFailure(lines, 'E1180: Variable arguments type must be a list: any')

  # varargs must match
  lines =<< trim END
    vim9script
    var FuncAnyVA: func(...any): number
    FuncAnyVA = (v1, v2): number => v1 + v2
  END
  v9.CheckScriptFailure(lines, 'E1180: Variable arguments type must be a list: any')

  # varargs must match
  lines =<< trim END
    vim9script
    var FuncAnyVA: func(...any): number
    FuncAnyVA = (v1: list<any>): number => 3
  END
  v9.CheckScriptFailure(lines, 'E1180: Variable arguments type must be a list: any')
enddef

def Test_assign_funcref_arg_any()
  var lines =<< trim END
    vim9script
    var FuncAnyVA: func(any): number
    FuncAnyVA = (v): number => v
  END
  # TODO: Verify this should succeed.
  v9.CheckScriptSuccess(lines)
enddef

def Test_heredoc()
  # simple heredoc
  var lines =<< trim END
      var text =<< trim TEXT # comment
        abc
      TEXT
      assert_equal(['abc'], text)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # empty heredoc
  lines =<< trim END
       var text =<< trim TEXT
       TEXT
       assert_equal([], text)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # heredoc with a single empty line
  lines =<< trim END
      var text =<< trim TEXT

      TEXT
      assert_equal([''], text)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # assign heredoc to variable with type
  lines =<< trim END
      var text: list<string> =<< trim TEXT
        var foo =<< trim FOO
      TEXT
      assert_equal(['var foo =<< trim FOO'], text)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # extra whitespace before type is allowed
  lines =<< trim END
      var text:   list<string> =<< trim TEXT
        var foo =<< trim FOO
      TEXT
      assert_equal(['var foo =<< trim FOO'], text)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # missing whitespace before type is an error
  lines =<< trim END
      var text:list<string> =<< trim TEXT
        var foo =<< trim FOO
      TEXT
      assert_equal(['var foo =<< trim FOO'], text)
  END
  v9.CheckDefAndScriptFailure(lines, 'E1069:')

  # assign heredoc to list slice
  lines =<< trim END
      var text = ['']
      text[ : ] =<< trim TEXT
        var foo =<< trim FOO
      TEXT
      assert_equal(['var foo =<< trim FOO'], text)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # assign heredoc to curly braces name in legacy function in Vim9 script
  lines =<< trim END
      vim9script
      func Func()
        let foo_3_bar = ['']
        let foo_{1 + 2}_bar[ : ] =<< trim TEXT
          var foo =<< trim FOO
        TEXT
        call assert_equal(['var foo =<< trim FOO'], foo_3_bar)
      endfunc
      Func()
  END
  v9.CheckScriptSuccess(lines)

  v9.CheckDefFailure(['var lines =<< trim END X', 'END'], 'E488:')
  v9.CheckDefFailure(['var lines =<< trim END " comment', 'END'], 'E488:')

  lines =<< trim [END]
      def Func()
        var&lines =<< trim END
        x
        x
      enddef
      defcompile
  [END]
  v9.CheckScriptFailure(lines, 'E1145: Missing heredoc end marker: END')
  delfunc! g:Func

  lines =<< trim [END]
      def Func()
        var lines =<< trim END
        x
        x
        x
        x
        x
        x
        x
        x
      enddef
      call Func()
  [END]
  v9.CheckScriptFailure(lines, 'E1145: Missing heredoc end marker: END')
  delfunc! g:Func

  lines =<< trim END
      var lines: number =<< trim STOP
        aaa
        bbb
      STOP
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012: Type mismatch; expected number but got list<string>', 1)

  lines =<< trim END
      var lines=<< STOP
        xxx
      STOP
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''=<<'' at "=<< STOP"', 1)
  lines =<< trim END
      var lines =<<STOP
        xxx
      STOP
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''=<<'' at "=<<STOP"', 1)
  lines =<< trim END
      var lines=<<STOP
        xxx
      STOP
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''=<<'' at "=<<STOP"', 1)
enddef

def Test_var_func_call()
  var lines =<< trim END
    vim9script
    func GetValue()
      if exists('g:count')
        let g:count += 1
      else
        let g:count = 1
      endif
      return 'this'
    endfunc
    var val: string = GetValue()
    # env var is always a string
    var env = $TERM
  END
  writefile(lines, 'Xfinished', 'D')
  source Xfinished
  # GetValue() is not called during discovery phase
  assert_equal(1, g:count)

  unlet g:count
enddef

def Test_var_missing_type()
  var lines =<< trim END
    vim9script
    var name = g:unknown
  END
  v9.CheckScriptFailure(lines, 'E121:')

  lines =<< trim END
    vim9script
    var nr: number = 123
    var name = nr
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_var_declaration()
  var lines =<< trim END
    vim9script
    var name: string
    g:var_uninit = name
    name = 'text'
    g:var_test = name
    # prefixing s: is not allowed
    name = 'prefixed'
    g:var_prefixed = name

    const FOO: number = 123
    assert_equal(123, FOO)
    const FOOS = 'foos'
    assert_equal('foos', FOOS)
    final FLIST = [1]
    assert_equal([1], FLIST)
    FLIST[0] = 11
    assert_equal([11], FLIST)

    const g:FOOS = 'gfoos'
    assert_equal('gfoos', g:FOOS)
    final g:FLIST = [2]
    assert_equal([2], g:FLIST)
    g:FLIST[0] = 22
    assert_equal([22], g:FLIST)

    def SetGlobalConst()
      const g:globConst = 123
    enddef
    SetGlobalConst()
    assert_equal(123, g:globConst)
    assert_true(islocked('g:globConst'))

    const w:FOOS = 'wfoos'
    assert_equal('wfoos', w:FOOS)
    final w:FLIST = [3]
    assert_equal([3], w:FLIST)
    w:FLIST[0] = 33
    assert_equal([33], w:FLIST)

    var s:other: number
    other = 1234
    g:other_var = other

    var xyz: string  # comment

    # type is inferred
    var dict = {['a']: 222}
    def GetDictVal(key: any)
      g:dict_val = dict[key]
    enddef
    GetDictVal('a')

    final adict: dict<string> = {}
    def ChangeAdict()
      adict.foo = 'foo'
    enddef
    ChangeAdict()
  END
  v9.CheckScriptSuccess(lines)
  assert_equal('', g:var_uninit)
  assert_equal('text', g:var_test)
  assert_equal('prefixed', g:var_prefixed)
  assert_equal(1234, g:other_var)
  assert_equal(222, g:dict_val)

  unlet g:var_uninit
  unlet g:var_test
  unlet g:var_prefixed
  unlet g:other_var
  unlet g:globConst
  unlet g:FOOS
  unlet g:FLIST
  unlet w:FOOS
  unlet w:FLIST
enddef

def Test_create_list_after_const()
  const a = 1
  g:ll = []
  assert_equal(0, islocked('g:ll'))
  unlet g:ll
enddef

def Test_var_declaration_fails()
  var lines =<< trim END
    vim9script
    final var: string
  END
  v9.CheckScriptFailure(lines, 'E1125:')

  lines =<< trim END
    vim9script
    const g:constvar = 'string'
    g:constvar = 'xx'
  END
  v9.CheckScriptFailure(lines, 'E741:')
  unlet g:constvar

  lines =<< trim END
    vim9script
    var name = 'one'
    lockvar name
    def SetLocked()
      name = 'two'
    enddef
    SetLocked()
  END
  v9.CheckScriptFailure(lines, 'E741: Value is locked: name', 1)

  lines =<< trim END
    let s:legacy = 'one'
    lockvar s:legacy
    def SetLocked()
      s:legacy = 'two'
    enddef
    call SetLocked()
  END
  v9.CheckScriptFailure(lines, 'E741: Value is locked: s:legacy', 1)

  lines =<< trim END
    vim9script
    def SetGlobalConst()
      const g:globConst = 123
    enddef
    SetGlobalConst()
    g:globConst = 234
  END
  v9.CheckScriptFailure(lines, 'E741: Value is locked: g:globConst', 6)
  unlet g:globConst

  lines =<< trim END
    vim9script
    const cdict: dict<string> = {}
    def Change()
      cdict.foo = 'foo'
    enddef
    defcompile
  END
  v9.CheckScriptFailure(lines, 'E46:')

  lines =<< trim END
    vim9script
    final w:finalvar = [9]
    w:finalvar = [8]
  END
  v9.CheckScriptFailure(lines, 'E1122:')
  unlet w:finalvar

  lines =<< trim END
    vim9script
    const var: string
  END
  v9.CheckScriptFailure(lines, 'E1021:')

  lines =<< trim END
    vim9script
    var 9var: string
  END
  v9.CheckScriptFailure(lines, 'E488:')

  v9.CheckDefFailure(['var foo.bar = 2'], 'E1087:')
  v9.CheckDefFailure(['var foo[3] = 2'], 'E1087:')
  v9.CheckDefFailure(['const foo: number'], 'E1021:')

  lines =<< trim END
      va foo = 123
  END
  v9.CheckDefAndScriptFailure(lines, 'E1065:', 1)

  lines =<< trim END
      var foo: func(number
  END
  v9.CheckDefAndScriptFailure(lines, 'E110:', 1)

  lines =<< trim END
      var foo: func(number): func(
  END
  v9.CheckDefAndScriptFailure(lines, 'E110:', 1)

  for type in ['num_ber',
               'anys', 'ani',
               'bools', 'boel',
               'blobs', 'blub',
               'channels', 'channol',
               'dicts', 'duct',
               'floats', 'floot',
               'funcs', 'funk',
               'jobs', 'jop',
               'lists', 'last',
               'numbers', 'numbar',
               'strings', 'strung',
               'voids', 'viod']
    v9.CheckDefAndScriptFailure([$'var foo: {type}'], 'E1010:', 1)
  endfor
enddef

def Test_var_declaration_inferred()
  # check that type is set on the list so that extend() fails
  var lines =<< trim END
      vim9script
      def GetList(): list<number>
        var l = [1, 2, 3]
        return l
      enddef
      echo GetList()->extend(['x'])
  END
  v9.CheckScriptFailure(lines, 'E1013:', 6)

  lines =<< trim END
      vim9script
      def GetNr(): number
        return 5
      enddef
      def TestOne()
        var some = [function('len'), GetNr]
        g:res = typename(some)
      enddef
      TestOne()
      assert_equal('list<func(): number>', g:res)

      def TestTwo()
        var some = [function('len'), GetNr]
        g:res = typename(some)
      enddef
      TestTwo()
      assert_equal('list<func(): number>', g:res)
      unlet g:res

      # FIXME: why is the type different?
      var first = [function('len'), GetNr]
      assert_equal('list<func(...): number>', typename(first))
      var second = [GetNr, function('len')]
      assert_equal('list<func(...): number>', typename(second))
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_script_local_in_legacy()
  # OK to define script-local later but before compiling
  var lines =<< trim END
    def SetLater()
      legvar = 'two'
    enddef
    let s:legvar = 'one'
    defcompile
    call SetLater()
    call assert_equal('two', s:legvar)
  END
  v9.CheckScriptSuccess(lines)

  # OK to leave out s: prefix when script-local already defined
  lines =<< trim END
    let s:legvar = 'one'
    def SetNoPrefix()
      legvar = 'two'
    enddef
    call SetNoPrefix()
    call assert_equal('two', s:legvar)
  END
  v9.CheckScriptSuccess(lines)

  # Not OK to leave out s: prefix when script-local defined after compiling
  lines =<< trim END
    def SetLaterNoPrefix()
      legvar = 'two'
    enddef
    defcompile
    let s:legvar = 'one'
  END
  v9.CheckScriptFailure(lines, 'E476:', 1)

  edit! Xslfile
  lines =<< trim END
      var edit: bool
      legacy edit
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_var_type_check()
  var lines =<< trim END
    vim9script
    var name: string
    name = 1234
  END
  v9.CheckScriptFailure(lines, 'E1012:')

  lines =<< trim END
    vim9script
    var name:string
  END
  v9.CheckScriptFailure(lines, 'E1069:')

  v9.CheckDefAndScriptFailure(['var n:number = 42'], 'E1069:')

  lines =<< trim END
    vim9script
    var name: asdf
  END
  v9.CheckScriptFailure(lines, 'E1010:')

  lines =<< trim END
    vim9script
    var l: list<number>
    l = []
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    vim9script
    var d: dict<number>
    d = {}
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    vim9script
    var d = {a: 1, b: [2]}
    def Func(b: bool)
      var l: list<number> = b ? d.b : [3]
    enddef
    defcompile
  END
  v9.CheckScriptSuccess(lines)
enddef

let g:dict_number = #{one: 1, two: 2}

def Test_var_list_dict_type()
  var ll: list<number>
  ll = [1, 2, 2, 3, 3, 3]->uniq()
  ll->assert_equal([1, 2, 3])

  var dd: dict<number>
  dd = g:dict_number
  dd->assert_equal(g:dict_number)

  var lines =<< trim END
      var ll: list<number>
      ll = [1, 2, 3]->map('"one"')
  END
  v9.CheckDefExecFailure(lines, 'E1012: Type mismatch; expected list<number> but got list<string>')
enddef

def Test_cannot_use_let()
  v9.CheckDefAndScriptFailure(['let a = 34'], 'E1126:', 1)
enddef

def Test_unlet()
  g:somevar = 'yes'
  assert_true(exists('g:somevar'))
  unlet g:somevar
  assert_false(exists('g:somevar'))
  unlet! g:somevar

  # also works for script-local variable in legacy Vim script
  s:somevar = 'legacy'
  assert_true(exists('s:somevar'))
  unlet s:somevar
  assert_false(exists('s:somevar'))
  unlet! s:somevar

  if 0
    unlet g:does_not_exist
  endif

  v9.CheckDefExecFailure(['unlet v:notfound.key'], 'E1001:')

  v9.CheckDefExecFailure([
    'var dd = 111',
    'unlet dd',
    ], 'E1081:', 2)

  # dict unlet
  var dd = {a: 1, b: 2, c: 3, 4: 4}
  unlet dd['a']
  unlet dd.c
  unlet dd[4]
  assert_equal({b: 2}, dd)

  # null key works like empty string
  dd = {'': 1, x: 9}
  unlet dd[null_string]
  assert_equal({x: 9}, dd)

  # list unlet
  var ll = [1, 2, 3, 4]
  unlet ll[1]
  unlet ll[-1]
  assert_equal([1, 3], ll)

  ll = [1, 2, 3, 4]
  unlet ll[0 : 1]
  assert_equal([3, 4], ll)

  ll = [1, 2, 3, 4]
  unlet ll[2 : 8]
  assert_equal([1, 2], ll)

  ll = [1, 2, 3, 4]
  unlet ll[-2 : -1]
  assert_equal([1, 2], ll)

  g:nrdict = {1: 1, 2: 2}
  g:idx = 1
  unlet g:nrdict[g:idx]
  assert_equal({2: 2}, g:nrdict)
  unlet g:nrdict
  unlet g:idx

  v9.CheckDefFailure([
    'var ll = [1, 2]',
    'll[1 : 2] = 7',
    ], 'E1012: Type mismatch; expected list<number> but got number', 2)
  v9.CheckDefFailure([
    'var dd = {a: 1}',
    'unlet dd["a" : "a"]',
    ], 'E1166:', 2)
  v9.CheckDefExecFailure([
    'unlet g:adict[0 : 1]',
    ], 'E1148:', 1)
  v9.CheckDefFailure([
    'var ll = [1, 2]',
    'unlet ll[0:1]',
    ], 'E1004:', 2)
  v9.CheckDefFailure([
    'var ll = [1, 2]',
    'unlet ll[0 :1]',
    ], 'E1004:', 2)
  v9.CheckDefFailure([
    'var ll = [1, 2]',
    'unlet ll[0: 1]',
    ], 'E1004:', 2)

  v9.CheckDefExecFailure([
    'g:ll = [1, 2]',
    'g:idx = "x"',
    'unlet g:ll[g:idx]',
    ], 'E1029: Expected number but got string', 3)

  v9.CheckDefExecFailure([
    'g:ll = [1, 2, 3]',
    'g:idx = "x"',
    'unlet g:ll[g:idx : 2]',
    ], 'E1029: Expected number but got string', 3)

  v9.CheckDefExecFailure([
    'g:ll = [1, 2, 3]',
    'g:idx = "x"',
    'unlet g:ll[0 : g:idx]',
    ], 'E1029: Expected number but got string', 3)

  # command recognized as assignment when skipping, should not give an error
  v9.CheckScriptSuccess([
    'vim9script',
    'for i in []',
    "  put =''",
    'endfor'])

  v9.CheckDefFailure([
    'var ll = [1, 2]',
    'unlet ll["x" : 1]',
    ], 'E1012:', 2)
  v9.CheckDefFailure([
    'var ll = [1, 2]',
    'unlet ll[0 : "x"]',
    ], 'E1012:', 2)

  # list of dict unlet
  var dl = [{a: 1, b: 2}, {c: 3}]
  unlet dl[0]['b']
  assert_equal([{a: 1}, {c: 3}], dl)

  v9.CheckDefExecFailure([
    'var ll = test_null_list()',
    'unlet ll[0]',
    ], 'E684:', 2)
  v9.CheckDefExecFailure([
    'var ll = [1]',
    'unlet ll[2]',
    ], 'E684:', 2)
  v9.CheckDefExecFailure([
    'var ll = [1]',
    'unlet ll[g:astring]',
    ], 'E1012:', 2)
  v9.CheckDefExecFailure([
    'var dd = test_null_dict()',
    'unlet dd["a"]',
    ], 'E716:', 2)
  v9.CheckDefExecFailure([
    'var dd = {a: 1}',
    'unlet dd["b"]',
    ], 'E716:', 2)
  v9.CheckDefExecFailure([
    'var dd = {a: 1}',
    'unlet dd[g:alist]',
    ], 'E1105:', 2)

  v9.CheckDefExecFailure([
    'g:dd = {"a": 1, 2: 2}',
    'unlet g:dd[0z11]',
    ], 'E1029:', 2)
  v9.CheckDefExecFailure([
    'g:str = "a string"',
    'unlet g:str[0]',
    ], 'E1148: Cannot index a string', 2)

  # can compile unlet before variable exists
  g:someDict = {key: 'val'}
  var k = 'key'
  unlet g:someDict[k]
  assert_equal({}, g:someDict)
  unlet g:someDict
  assert_false(exists('g:someDict'))

  v9.CheckScriptFailure([
   'vim9script',
   'var svar = 123',
   'unlet svar',
   ], 'E1081:')
  v9.CheckScriptFailure([
   'vim9script',
   'var svar = 123',
   'unlet s:svar',
   ], 'E1268:')
  v9.CheckScriptFailure([
   'vim9script',
   'var svar = 123',
   'def Func()',
   '  unlet svar',
   'enddef',
   'defcompile',
   ], 'E1081:')
  v9.CheckScriptFailure([
   'vim9script',
   'var svar = 123',
   'func Func()',
   '  unlet s:svar',
   'endfunc',
   'Func()',
   ], 'E1081:')
  v9.CheckScriptFailure([
   'vim9script',
   'var svar = 123',
   'def Func()',
   '  unlet s:svar',
   'enddef',
   'defcompile',
   ], 'E1081:')

  v9.CheckScriptFailure([
   'vim9script',
   'def Delcount(dict: dict<any>)',
   '  unlet dict.count',
   'enddef',
   'Delcount(v:)',
   ], 'E742:')

  v9.CheckScriptFailure([
   'vim9script',
   'def DelChangedtick(dict: dict<any>)',
   '  unlet dict.changedtick',
   'enddef',
   'DelChangedtick(b:)',
   ], 'E795:')

  writefile(['vim9script', 'export var svar = 1234'], 'XunletExport.vim', 'D')
  var lines =<< trim END
    vim9script
    import './XunletExport.vim' as exp
    def UnletSvar()
      unlet exp.svar
    enddef
    defcompile
  END
  v9.CheckScriptFailure(lines, 'E1260:', 1)

  $ENVVAR = 'foobar'
  assert_equal('foobar', $ENVVAR)
  unlet $ENVVAR
  assert_equal('', $ENVVAR)
enddef

def Test_expr_error_no_assign()
  var lines =<< trim END
      vim9script
      var x = invalid
      echo x
  END
  v9.CheckScriptFailureList(lines, ['E121:', 'E121:'])

  lines =<< trim END
      vim9script
      var x = 1 / 0
      echo x
  END
  v9.CheckScriptFailure(lines, 'E1154:')

  lines =<< trim END
      vim9script
      var x = 1 % 0
      echo x
  END
  v9.CheckScriptFailure(lines, 'E1154:')

  lines =<< trim END
      var x: string  'string'
  END
  v9.CheckDefAndScriptFailure(lines, 'E488:')
enddef


def Test_assign_command_modifier()
  var lines =<< trim END
      var verbose = 0
      verbose = 1
      assert_equal(1, verbose)
      silent verbose = 2
      assert_equal(2, verbose)
      silent verbose += 2
      assert_equal(4, verbose)
      silent verbose -= 1
      assert_equal(3, verbose)

      var topleft = {one: 1}
      sandbox topleft.one = 3
      assert_equal({one: 3}, topleft)
      leftabove topleft[' '] = 4
      assert_equal({one: 3, ' ': 4}, topleft)

      var x: number
      var y: number
      silent [x, y] = [1, 2]
      assert_equal(1, x)
      assert_equal(2, y)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_assign_alt_buf_register()
  var lines =<< trim END
      edit 'file_b1'
      var b1 = bufnr()
      edit 'file_b2'
      var b2 = bufnr()
      assert_equal(b1, bufnr('#'))
      @# = b2
      assert_equal(b2, bufnr('#'))
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_script_funcref_case()
  var lines =<< trim END
      var Len = (s: string): number => len(s) + 1
      assert_equal(5, Len('asdf'))
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var len = (s: string): number => len(s) + 1
  END
  v9.CheckDefAndScriptFailure(lines, 'E704:')

  lines =<< trim END
      vim9script
      var Len = (s: string): number => len(s) + 2
      assert_equal(6, Len('asdf'))
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      var len = (s: string): number => len(s) + 1
  END
  v9.CheckScriptFailure(lines, 'E704:')
enddef

def Test_script_funcref_runtime_type_check()
  var lines =<< trim END
      vim9script
      def FuncWithNumberArg(n: number)
      enddef
      def Test()
        var Ref: func(string) = function(FuncWithNumberArg)
      enddef
      defcompile
  END
  # OK at compile time
  v9.CheckScriptSuccess(lines)

  # Type check fails at runtime
  v9.CheckScriptFailure(lines + ['Test()'], 'E1012: Type mismatch; expected func(string) but got func(number)')
enddef

def Test_inc_dec()
  var lines =<< trim END
      var nr = 7
      ++nr
      assert_equal(8, nr)
      --nr
      assert_equal(7, nr)
      ++nr | ++nr
      assert_equal(9, nr)
      ++nr # comment
      assert_equal(10, nr)

      var ll = [1, 2]
      --ll[0]
      ++ll[1]
      assert_equal([0, 3], ll)

      g:count = 1
      ++g:count
      --g:count
      assert_equal(1, g:count)
      unlet g:count
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var nr = 7
      ++ nr
  END
  v9.CheckDefAndScriptFailure(lines, "E1202: No white space allowed after '++': ++ nr")
enddef

def Test_abort_after_error()
  # should abort after strpart() fails, not give another type error
  var lines =<< trim END
      vim9script
      var x: string
      x = strpart(1, 2)
  END
  writefile(lines, 'Xtestscript', 'D')
  var expected = 'E1174: String required for argument 1'
  assert_fails('so Xtestscript', [expected, expected], 3)
enddef

def Test_using_s_var_in_function()
  var lines =<< trim END
      vim9script
      var scriptlevel = 123
      def SomeFunc()
        echo s:scriptlevel
      enddef
      SomeFunc()
  END
  v9.CheckScriptFailure(lines, 'E1268:')

  # OK in legacy script
  lines =<< trim END
      let s:scriptlevel = 123
      def s:SomeFunc()
        echo s:scriptlevel
      enddef
      call s:SomeFunc()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      var scriptlevel = 123
      def SomeFunc()
        s:scriptlevel = 456
      enddef
      SomeFunc()
  END
  v9.CheckScriptFailure(lines, 'E1268:')

  # OK in legacy script
  lines =<< trim END
      let s:scriptlevel = 123
      def s:SomeFunc()
        s:scriptlevel = 456
      enddef
      call s:SomeFunc()
      call assert_equal(456, s:scriptlevel)
  END
  v9.CheckScriptSuccess(lines)
enddef

" Test for specifying a type in assignment
def Test_type_specification_in_assignment()
  # specify type for an existing script local variable without "var"
  var lines =<< trim END
    vim9script
    var n: number = 10
    n: number = 20
  END
  v9.CheckSourceFailure(lines, 'E488: Trailing characters: : number = 20', 3)

  # specify type for a non-existing script local variable without "var"
  lines =<< trim END
    vim9script
    MyVar: string = 'abc'
  END
  v9.CheckSourceFailure(lines, "E492: Not an editor command: MyVar: string = 'abc'", 2)

  # specify type for an existing def local variable without "var"
  lines =<< trim END
    vim9script
    def Foo()
      var n: number = 10
      n: number = 20
    enddef
    Foo()
  END
  v9.CheckSourceFailure(lines, 'E488: Trailing characters: : number = 20', 2)

  # specify type for a non-existing def local variable without "var"
  lines =<< trim END
    vim9script
    def Foo()
      MyVar: string = 'abc'
    enddef
    Foo()
  END
  v9.CheckSourceFailure(lines, "E476: Invalid command: MyVar: string = 'abc'", 1)
enddef

let g:someVar = 'X'

" Test for heredoc with Vim expressions.
" This messes up highlighting, keep it near the end.
def Test_heredoc_expr()
  var lines =<< trim CODE
    var s = "local"
    var a1 = "1"
    var a2 = "2"
    var a3 = "3"
    var a4 = ""
    var code =<< trim eval END
      var a = {5 + 10}
      var b = {min([10, 6])} + {max([4, 6])}
      var c = "{s}"
      var d = x{a1}x{a2}x{a3}x{a4}
    END
    assert_equal(['var a = 15', 'var b = 6 + 6', 'var c = "local"', 'var d = x1x2x3x'], code)
  CODE
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim CODE
    var code =<< eval trim END
      var s = "{$SOME_ENV_VAR}"
    END
    assert_equal(['var s = "somemore"'], code)
  CODE
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim CODE
    var code =<< eval END
      var s = "{$SOME_ENV_VAR}"
    END
    assert_equal(['  var s = "somemore"'], code)
  CODE
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim CODE
    var code =<< eval trim END
      let a = {{abc}}
      let b = {g:someVar}
      let c = {{
    END
    assert_equal(['let a = {abc}', 'let b = X', 'let c = {'], code)
  CODE
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim LINES
      var text =<< eval trim END
        let b = {
      END
  LINES
  v9.CheckDefAndScriptFailure(lines, "E1279: Missing '}'")

  lines =<< trim LINES
      var text =<< eval trim END
        let b = {abc
      END
  LINES
  v9.CheckDefAndScriptFailure(lines, "E1279: Missing '}'")

  lines =<< trim LINES
      var text =<< eval trim END
        let b = {}
      END
  LINES
  v9.CheckDefAndScriptFailure(lines, 'E15: Invalid expression: "}"')
enddef

" Test for assigning to a multi-dimensional list item.
def Test_list_item_assign()
  var lines =<< trim END
    vim9script

    def Foo()
      var l: list<list<string>> = [['x', 'x', 'x'], ['y', 'y', 'y']]
      var z: number = 1

      [l[1][2], z] = ['a', 20]
      assert_equal([['x', 'x', 'x'], ['y', 'y', 'a']], l)
    enddef
    Foo()
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    var l: list<list<string>> = [['x', 'x', 'x'], ['y', 'y', 'y']]
    var z: number = 1

    [l[1][2], z] = ['a', 20]
    assert_equal([['x', 'x', 'x'], ['y', 'y', 'a']], l)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for assigning to a multi-dimensional dict item.
def Test_dict_item_assign()
  # This used to fail with the error "E1105: Cannot convert list to string"
  # (Github issue #13485)
  var lines =<< trim END
    vim9script
    def F()
      var d: dict<dict<number>> = {a: {b: 0}}

      for group in keys(d)
        d['a']['b'] += 1
      endfor
      assert_equal({a: {b: 1}}, d)
    enddef
    F()
  END
  v9.CheckSourceSuccess(lines)

  # This used to crash Vim
  lines =<< trim END
    vim9script
    def F()
      var d: dict<dict<number>> = {a: {b: 0}}
      d['a']['b'] += 1
      assert_equal({a: {b: 1}}, d)
    enddef
    F()
  END
  v9.CheckSourceSuccess(lines)

  # Assignment at script level
  lines =<< trim END
    vim9script
    var d: dict<dict<number>> = {a: {b: 0}}

    for group in keys(d)
      d['a']['b'] += 1
    endfor
    assert_equal({a: {b: 1}}, d)
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_class_assign()
  var lines =<< trim END
    vim9script
    class C
    endclass
    class D
    endclass
    assert_fails('C = D', 'E1405: Class "D" cannot be used as a value')
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using various types (dict, list, blob, funcref, class) as variable
" in assignments with a different type
def Test_type_check()
  var lines =<< trim END
    vim9script
    class A
    endclass
    type T = number
    var N: number = 1
    var S: string = 'abc'
    var d: dict<number> = {}
    var l: list<number> = []
    var b: blob = 0z10
    var Fn: func = function('min')
    var o: A = A.new()

    # Assign a number
    assert_fails('d = N', 'E1012: Type mismatch; expected dict<number> but got number')
    assert_fails('l = N', 'E1012: Type mismatch; expected list<number> but got number')
    assert_fails('b = N', 'E1012: Type mismatch; expected blob but got number')
    assert_fails('Fn = N', 'E1012: Type mismatch; expected func(...): unknown but got number')
    assert_fails('A = N', 'E1405: Class "A" cannot be used as a value')
    assert_fails('o = N', 'E1012: Type mismatch; expected object<A> but got number')
    assert_fails('T = N', 'E1403: Type alias "T" cannot be used as a value')

    # Use a compound operator with different LHS types
    assert_fails('d += N', 'E734: Wrong variable type for +=')
    assert_fails('l += N', 'E734: Wrong variable type for +=')
    assert_fails('b += N', 'E734: Wrong variable type for +=')
    assert_fails('Fn += N', 'E734: Wrong variable type for +=')
    assert_fails('A += N', 'E1405: Class "A" cannot be used as a value')
    assert_fails('o += N', 'E734: Wrong variable type for +=')
    assert_fails('T += N', 'E1403: Type alias "T" cannot be used as a value')

    # Assign to a number variable
    assert_fails('N = d', 'E1012: Type mismatch; expected number but got dict<number>')
    assert_fails('N = l', 'E1012: Type mismatch; expected number but got list<number>')
    assert_fails('N = b', 'E1012: Type mismatch; expected number but got blob')
    assert_fails('N = Fn', 'E1012: Type mismatch; expected number but got func([unknown]): number')
    assert_fails('N = A', 'E1405: Class "A" cannot be used as a value')
    assert_fails('N = o', 'E1012: Type mismatch; expected number but got object<A>')
    assert_fails('N = T', 'E1403: Type alias "T" cannot be used as a value')

    # Use a compound operator with different RHS types
    assert_fails('N += d', 'E734: Wrong variable type for +=')
    assert_fails('N += l', 'E734: Wrong variable type for +=')
    assert_fails('N += b', 'E974: Using a Blob as a Number')
    assert_fails('N += Fn', 'E734: Wrong variable type for +=')
    assert_fails('N += A', 'E1405: Class "A" cannot be used as a value')
    assert_fails('N += o', 'E1320: Using an Object as a Number')
    assert_fails('N += T', 'E1403: Type alias "T" cannot be used as a value')

    # Initialize multiple variables using []
    assert_fails('var [X1: number, Y: number] = [1, d]', 'E1012: Type mismatch; expected number but got dict<number>')
    assert_fails('var [X2: number, Y: number] = [1, l]', 'E1012: Type mismatch; expected number but got list<number>')
    assert_fails('var [X3: number, Y: number] = [1, b]', 'E1012: Type mismatch; expected number but got blob')
    assert_fails('var [X4: number, Y: number] = [1, Fn]', 'E1012: Type mismatch; expected number but got func([unknown]): number')
    assert_fails('var [X7: number, Y: number] = [1, A]', 'E1405: Class "A" cannot be used as a value')
    assert_fails('var [X8: number, Y: number] = [1, o]', 'E1012: Type mismatch; expected number but got object<A>')
    assert_fails('var [X8: number, Y: number] = [1, T]', 'E1403: Type alias "T" cannot be used as a value')

    # String concatenation with various LHS types
    assert_fails('S ..= d', 'E734: Wrong variable type for .=')
    assert_fails('S ..= l', 'E734: Wrong variable type for .=')
    assert_fails('S ..= b', 'E976: Using a Blob as a String')
    assert_fails('S ..= Fn', 'E734: Wrong variable type for .=')
    assert_fails('S ..= A', 'E1405: Class "A" cannot be used as a value')
    assert_fails('S ..= o', 'E1324: Using an Object as a String')
    assert_fails('S ..= T', 'E1403: Type alias "T" cannot be used as a value')

    # String concatenation with various RHS types
    assert_fails('d ..= S', 'E734: Wrong variable type for .=')
    assert_fails('l ..= S', 'E734: Wrong variable type for .=')
    assert_fails('b ..= S', 'E734: Wrong variable type for .=')
    assert_fails('Fn ..= S', 'E734: Wrong variable type for .=')
    assert_fails('A ..= S', 'E1405: Class "A" cannot be used as a value')
    assert_fails('o ..= S', 'E734: Wrong variable type for .=')
    assert_fails('T ..= S', 'E1403: Type alias "T" cannot be used as a value')
  END
  v9.CheckSourceSuccess(lines)

  if has('channel')
    lines =<< trim END
      vim9script
      var N: number = 1
      var S: string = 'abc'
      var j: job = test_null_job()
      var ch: channel = test_null_channel()
      assert_fails('j = N', 'E1012: Type mismatch; expected job but got number')
      assert_fails('ch = N', 'E1012: Type mismatch; expected channel but got number')
      assert_fails('j += N', 'E734: Wrong variable type for +=')
      assert_fails('ch += N', 'E734: Wrong variable type for +=')
      assert_fails('N = j', 'E1012: Type mismatch; expected number but got job')
      assert_fails('N = ch', 'E1012: Type mismatch; expected number but got channel')
      assert_fails('N += j', 'E910: Using a Job as a Number')
      assert_fails('N += ch', 'E913: Using a Channel as a Number')
      assert_fails('var [X5: number, Y: number] = [1, j]', 'E1012: Type mismatch; expected number but got job')
      assert_fails('var [X6: number, Y: number] = [1, ch]', 'E1012: Type mismatch; expected number but got channel')
      assert_fails('S ..= j', 'E908: Using an invalid value as a String: job')
      assert_fails('S ..= ch', 'E908: Using an invalid value as a String: channel')
      assert_fails('j ..= S', 'E734: Wrong variable type for .=')
      assert_fails('ch ..= S', 'E734: Wrong variable type for .=')
    END
    v9.CheckSourceSuccess(lines)
  endif

  lines =<< trim END
    vim9script
    class A
    endclass

    def F()
      A += 3
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "A" cannot be used as a value')

  lines =<< trim END
    vim9script
    class A
    endclass

    var o = A.new()
    def F()
      o += 4
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1411: Missing dot after object "o"')
enddef

" Test for checking the argument type of a def function
def Test_func_argtype_check()
  var lines =<< trim END
    vim9script

    # Passing different types as argument to a function expecting a number
    def IntArg(n: number)
    enddef

    class A
    endclass
    var N: number = 1
    var S: string = 'abc'
    var d: dict<number> = {}
    var l: list<number> = []
    var b: blob = 0z10
    var Fn: func = function('min')
    var o: A = A.new()

    assert_fails('IntArg(d)', 'E1013: Argument 1: type mismatch, expected number but got dict<number>')
    assert_fails('IntArg(l)', 'E1013: Argument 1: type mismatch, expected number but got list<number>')
    assert_fails('IntArg(b)', 'E1013: Argument 1: type mismatch, expected number but got blob')
    assert_fails('IntArg(Fn)', 'E1013: Argument 1: type mismatch, expected number but got func([unknown]): number')
    if has('channel')
      var j: job = test_null_job()
      var ch: channel = test_null_channel()
      assert_fails('IntArg(j)', 'E1013: Argument 1: type mismatch, expected number but got job')
      assert_fails('IntArg(ch)', 'E1013: Argument 1: type mismatch, expected number but got channel')
    endif
    assert_fails('IntArg(A)', 'E1405: Class "A" cannot be used as a value')
    assert_fails('IntArg(o)', 'E1013: Argument 1: type mismatch, expected number but got object<A>')

    # Passing a number to functions accepting different argument types
    def DictArg(_: dict<number>)
    enddef
    assert_fails('DictArg(N)', 'E1013: Argument 1: type mismatch, expected dict<number> but got number')

    def ListArg(_: list<number>)
    enddef
    assert_fails('ListArg(N)', 'E1013: Argument 1: type mismatch, expected list<number> but got number')

    def BlobArg(_: blob)
    enddef
    assert_fails('BlobArg(N)', 'E1013: Argument 1: type mismatch, expected blob but got number')

    def FuncArg(Fn_arg: func)
    enddef
    assert_fails('FuncArg(N)', 'E1013: Argument 1: type mismatch, expected func(...): unknown but got number')

    if has('channel')
      def JobArg(_: job)
      enddef
      assert_fails('JobArg(N)', 'E1013: Argument 1: type mismatch, expected job but got number')

      def ChannelArg(_: channel)
      enddef
      assert_fails('ChannelArg(N)', 'E1013: Argument 1: type mismatch, expected channel but got number')
    endif

    def ObjectArg(_: A)
    enddef
    assert_fails('ObjectArg(N)', 'E1013: Argument 1: type mismatch, expected object<A> but got number')
  END
  v9.CheckSourceSuccess(lines)

  # Calling a function expecting a number type with different argument types
  # from another function
  var pre_lines =<< trim END
    vim9script
    class A
    endclass
    def IntArg(n: number)
    enddef
    def Foo()
  END
  var post_lines =<< trim END
    enddef
    defcompile
  END
  lines = pre_lines + ['var d: dict<number> = {}', 'IntArg(d)'] + post_lines
  v9.CheckSourceFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got dict<number>', 2)
  lines = pre_lines + ['var l: list<number> = []', 'IntArg(l)'] + post_lines
  v9.CheckSourceFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got list<number>', 2)
  lines = pre_lines + ['var b: blob = 0z12', 'IntArg(b)'] + post_lines
  v9.CheckSourceFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got blob', 2)
  lines = pre_lines + ['var Fn: func = function("min")', 'IntArg(Fn)'] + post_lines
  v9.CheckSourceFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got func(...): unknown', 2)
  if has('channel')
    lines = pre_lines + ['var j: job = test_null_job()', 'IntArg(j)'] + post_lines
    v9.CheckSourceFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got job', 2)
    lines = pre_lines + ['var ch: channel = test_null_channel()', 'IntArg(ch)'] + post_lines
    v9.CheckSourceFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got channel', 2)
  endif
  lines = pre_lines + ['IntArg(A)'] + post_lines
  v9.CheckSourceFailure(lines, 'E1405: Class "A" cannot be used as a value', 1)
  lines = pre_lines + ['var o: A = A.new()', 'IntArg(o)'] + post_lines
  v9.CheckSourceFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got object<A>', 2)
enddef

" Test for checking the return type of a def function
def Test_func_rettype_check()
  var lines =<< trim END
    vim9script
    def Fn(): dict<number>
      return 10
    enddef
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected dict<number> but got number', 1)

  lines =<< trim END
    vim9script
    def Fn(): list<number>
      return 10
    enddef
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected list<number> but got number', 1)

  lines =<< trim END
    vim9script
    def Fn(): blob
      return 10
    enddef
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected blob but got number', 1)

  lines =<< trim END
    vim9script
    def Fn(): func
      return 10
    enddef
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(...): unknown but got number', 1)

  lines =<< trim END
    vim9script
    def Fn(): job
      return 10
    enddef
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected job but got number', 1)

  lines =<< trim END
    vim9script
    def Fn(): channel
      return 10
    enddef
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected channel but got number', 1)

  lines =<< trim END
    vim9script
    class A
    endclass
    def Fn(): A
      return 10
    enddef
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected object<A> but got number', 1)
enddef

" Test for assigning different types of value to a variable of type "any"
def Test_assign_to_any()
  for [typestr, val] in [
                          ["'bool'", 'true'],
                          ["'number'", '100'],
                          ["'float'", '1.1'],
                          ["'string'", '"abc"'],
                          ["'blob'", '0z10'],
                          ["'list<number>'", '[1, 2, 3]'],
                          ["'dict<number>'", '{a: 1}'],
                        ]
    var lines =<< trim eval END
      vim9script
      var x: any = {val}
      assert_equal({typestr}, typename(x))
      x = [{{a: 1}}, {{b: 2}}]
      assert_equal('list<dict<number>>', typename(x))
      def Foo(xarg: any, s: string)
        assert_equal(s, typename(xarg))
      enddef
      Foo({val}, {typestr})
    END
    v9.CheckSourceSuccess(lines)
  endfor
enddef

def Test_assign_type_to_list_dict()
  var lines =<< trim END
    vim9script
    class C
    endclass

    var x = [C]
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  lines =<< trim END
    vim9script
    class C
    endclass
    type T = C

    def F()
      var x = [3, T, C]
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  lines =<< trim END
    vim9script
    type T = number

    def F()
      var x = [3, T]
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1407: Cannot use a Typealias as a variable or value')

  lines =<< trim END
    vim9script
    class C
    endclass

    var x = {e: C}
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  lines =<< trim END
    vim9script
    class C
    endclass

    def F()
      var x = {e: C}
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  lines =<< trim END
    vim9script
    type T = number

    def F()
      var x = {e: T}
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1407: Cannot use a Typealias as a variable or value')

  lines =<< trim END
    vim9script
    class C
    endclass

    def F()
      var x = {e: [C]}
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  lines =<< trim END
    vim9script
    type T = number

    def F()
      var x = {e: [T]}
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1407: Cannot use a Typealias as a variable or value')
enddef

" vim: ts=8 sw=2 sts=2 expandtab tw=80 fdm=marker
