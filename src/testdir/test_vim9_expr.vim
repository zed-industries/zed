" Tests for Vim9 script expressions

source check.vim
import './vim9.vim' as v9

let g:cond = v:false
def FuncOne(arg: number): string
  return 'yes'
enddef
def FuncTwo(arg: number): number
  return 123
enddef

" test cond ? expr : expr
def Test_expr1_ternary()
  var lines =<< trim END
      assert_equal('one', true ? 'one' : 'two')
      assert_equal('one', 1 ?
                            'one' :
                            'two')
      assert_equal('one', !!0.1 ? 'one' : 'two')
      assert_equal('one', !!'x' ? 'one' : 'two')
      assert_equal('one', !!'x'
                            ? 'one'
                            : 'two')
      assert_equal('one', !!0z1234 ? 'one' : 'two')
      assert_equal('one', !![0] ? 'one' : 'two')
      assert_equal('one', !!{x: 0} ? 'one' : 'two')
      var name = 1
      assert_equal('one', name ? 'one' : 'two')

      assert_equal('two', false ? 'one' : 'two')
      assert_equal('two', 0 ? 'one' : 'two')
      assert_equal('two', !!0.0 ? 'one' : 'two')
      assert_equal('two', !!'' ? 'one' : 'two')
      assert_equal('two', !!0z ? 'one' : 'two')
      assert_equal('two', !![] ? 'one' : 'two')
      assert_equal('two', !!{} ? 'one' : 'two')
      name = 0
      assert_equal('two', name ? 'one' : 'two')

      echo ['a'] + (1 ? ['b'] : ['c']
                )
      echo ['a'] + (1 ? ['b'] : ['c'] # comment
                )

      # with constant condition expression is not evaluated
      assert_equal('one', 1 ? 'one' : xxx)

      var Some: func = function('len')
      var Other: func = function('winnr')
      var Res: func = g:atrue ? Some : Other
      assert_equal(function('len'), Res)

      var RetOne: func(string): number = function('len')
      var RetTwo: func(string): number = function('strlen')
      var RetThat: func = g:atrue ? RetOne : RetTwo
      assert_equal(function('len'), RetThat)

      var X = g:FuncOne
      var Y = g:FuncTwo
      var Z = g:cond ? g:FuncOne : g:FuncTwo
      assert_equal(123, Z(3))
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var Z = g:cond ? FuncOne : FuncTwo
  END
  v9.CheckDefAndScriptFailure(lines, ['E1001: Variable not found: FuncOne', 'E121: Undefined variable: FuncTwo'])
enddef

def Test_expr1_ternary_vimscript()
  # check line continuation
  var lines =<< trim END
      var name = 1
		? 'yes'
		: 'no'
      assert_equal('yes', name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = v:false
		? 'yes'
		: 'no'
      assert_equal('no', name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = v:false ?
		'yes' :
		'no'
      assert_equal('no', name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = v:false ?  # comment
		'yes' :
                # comment
		'no' # comment
      assert_equal('no', name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # check white space
  lines =<< trim END
      var name = v:true?1:2
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''?'' at "?1:2"', 1)

  lines =<< trim END
      var name = v:true? 1 : 2
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      var name = v:true ?1 : 2
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      var name = v:true ? 1: 2
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after '':'' at ": 2"', 1)

  lines =<< trim END
      var name = v:true ? 1 :2
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      var name = 'x' ? 1 : 2
  END
  v9.CheckDefAndScriptFailure(lines, 'E1135:', 1)

  lines =<< trim END
      var name = [] ? 1 : 2
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E745:', 1)

  lines =<< trim END
      var name = {} ? 1 : 2
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E728:', 1)

  # check after failure eval_flags is reset
  lines =<< trim END
      try
        eval('0 ? 1: 2')
      catch
      endtry
      assert_equal(v:true, eval(string(v:true)))
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      try
        eval('0 ? 1 :2')
      catch
      endtry
      assert_equal(v:true, eval(string(v:true)))
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

func Test_expr1_ternary_fails()
  call v9.CheckDefAndScriptFailure(["var x = 1 ? 'one'"], "Missing ':' after '?'", 1)

  let msg = "White space required before and after '?'"
  call v9.CheckDefAndScriptFailure(["var x = 1? 'one' : 'two'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 ?'one' : 'two'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1?'one' : 'two'"], msg, 1)
  let lines =<< trim END
    var x = 1
     ?'one' : 'two'
     # comment
  END
  call v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''?'' at "?''one'' : ''two''"', 2)

  let msg = "White space required before and after ':'"
  call v9.CheckDefAndScriptFailure(["var x = 1 ? 'one': 'two'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 ? 'one' :'two'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 ? 'one':'two'"], msg, 1)
  let lines =<< trim END
    var x = 1 ? 'one'
          :'two'
          # Comment
  END
  call v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after '':'' at ":''two''"', 2)

  call v9.CheckDefAndScriptFailure(["var x = 'x' ? 'one' : 'two'"], 'E1135:', 1)
  call v9.CheckDefAndScriptFailure(["var x = 0z1234 ? 'one' : 'two'"], 'E974:', 1)
  call v9.CheckDefExecAndScriptFailure(["var x = [] ? 'one' : 'two'"], 'E745:', 1)
  call v9.CheckDefExecAndScriptFailure(["var x = {} ? 'one' : 'two'"], 'E728:', 1)

  call v9.CheckDefExecFailure(["var x = false ? "], 'E1097:', 3)
  call v9.CheckScriptFailure(['vim9script', "var x = false ? "], 'E15:', 2)
  call v9.CheckDefExecFailure(["var x = false ? 'one' : "], 'E1097:', 3)
  call v9.CheckScriptFailure(['vim9script', "var x = false ? 'one' : "], 'E15:', 2)

  call v9.CheckDefExecAndScriptFailure(["var x = true ? xxx : 'foo'"], ['E1001:', 'E121:'], 1)
  call v9.CheckDefExecAndScriptFailure(["var x = false ? 'foo' : xxx"], ['E1001:', 'E121:'], 1)

  call v9.CheckDefAndScriptFailure(["var x = 0.1 ? 'one' : 'two'"], 'E805:', 1)

  " missing argument detected even when common type is used
  call v9.CheckDefAndScriptFailure([
	\ 'var X = g:FuncOne',
	\ 'var Y = g:FuncTwo',
	\ 'var Z = g:cond ? g:FuncOne : g:FuncTwo',
	\ 'Z()'], 'E119:', 4)
endfunc

def Test_expr1_falsy()
  var lines =<< trim END
      assert_equal(v:true, v:true ?? 456)
      assert_equal(123, 123 ?? 456)
      assert_equal('yes', 'yes' ?? 456)
      assert_equal([1], [1] ?? 456)
      assert_equal({one: 1}, {one: 1} ?? 456)
      assert_equal(0.1, 0.1 ?? 456)

      assert_equal(456, v:false ?? 456)
      assert_equal(456, 0 ?? 456)
      assert_equal(456, '' ?? 456)
      assert_equal(456, [] ?? 456)
      assert_equal(456, {} ?? 456)
      assert_equal(456, 0.0 ?? 456)
  END
  v9.CheckDefAndScriptSuccess(lines)

  var msg = "White space required before and after '??'"
  call v9.CheckDefAndScriptFailure(["var x = 1?? 'one' : 'two'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 ??'one' : 'two'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1??'one' : 'two'"], msg, 1)
  lines =<< trim END
    var x = 1
      ??'one' : 'two'
      #comment
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''??'' at "??''one'' : ''two''"', 2)
enddef

def g:Record(val: any): any
  g:vals->add(val)
  return val
enddef

" test ||
def Test_expr2()
  var lines =<< trim END
      assert_equal(true, 1 || 0)
      assert_equal(true, 0 ||
                        0 ||
                        1)
      assert_equal(true, 0 ||
			0 ||
			!!7)
      assert_equal(false, 0 || 0)
      assert_equal(false, 0
                        || 0)
      assert_equal(false, 0 || false)

      g:vals = []
      assert_equal(true, g:Record(1) || g:Record(3))
      assert_equal([1], g:vals)

      g:vals = []
      assert_equal(true, g:Record(0) || g:Record(1))
      assert_equal([0, 1], g:vals)

      g:vals = []
      assert_equal(true, g:Record(0) || g:Record(true))
      assert_equal([0, true], g:vals)

      g:vals = []
      assert_equal(true, g:Record(0)
                          || g:Record(1)
                          || g:Record(0))
      assert_equal([0, 1], g:vals)

      g:vals = []
      assert_equal(true, g:Record(0)
			  || g:Record(true)
			  || g:Record(0))
      assert_equal([0, true], g:vals)

      g:vals = []
      assert_equal(true, g:Record(true) || g:Record(false))
      assert_equal([true], g:vals)

      g:vals = []
      assert_equal(false, g:Record(0) || g:Record(false) || g:Record(0))
      assert_equal([0, false, 0], g:vals)

      g:vals = []
      var x = 1
      if x || true
        g:vals = [1]
      endif
      assert_equal([1], g:vals)

      g:vals = []
      x = 3
      if true || x
        g:vals = [1]
      endif
      assert_equal([1], g:vals)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr2_vimscript()
  # check line continuation
  var lines =<< trim END
      var name = 0
		|| 1
      assert_equal(true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = v:false
		|| v:true
		|| v:false
      assert_equal(v:true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = v:false ||
		v:true ||
		v:false
      assert_equal(v:true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = v:false || # comment
                # comment
		v:true ||
                # comment
		v:false # comment
      assert_equal(v:true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # check white space
  lines =<< trim END
      var name = v:true||v:true
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1004: White space required before and after ''||'' at "||v:true"', 1)

  lines =<< trim END
      var name = v:true ||v:true
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      var name = v:true|| v:true
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)
enddef

def Test_expr2_fails()
  var msg = "White space required before and after '||'"
  v9.CheckDefAndScriptFailure(["var x = 1||0"], msg, 1)
  v9.CheckDefAndScriptFailure(["var x = 1 ||0"], msg, 1)
  v9.CheckDefAndScriptFailure(["var x = 1|| 0"], msg, 1)

  v9.CheckDefFailure(["var x = false || "], 'E1097:', 3)
  v9.CheckScriptFailure(['vim9script', "var x = false || "], 'E15:', 2)

  # script does not fail, the second expression is skipped
  v9.CheckDefFailure(["var x = 1 || xxx"], 'E1001:', 1)

  v9.CheckDefAndScriptFailure(["var x = [] || false"], ['E1012:', 'E745:'], 1)

  v9.CheckDefAndScriptFailure(["if 'yes' || 0", 'echo 0', 'endif'], ['E1012: Type mismatch; expected bool but got string', 'E1135: Using a String as a Bool'], 1)

  v9.CheckDefAndScriptFailure(["var x = 3 || false"], ['E1012:', 'E1023:'], 1)
  v9.CheckDefAndScriptFailure(["var x = false || 3"], ['E1012:', 'E1023:'], 1)

  v9.CheckDefAndScriptFailure(["if 3"], 'E1023:', 1)
  v9.CheckDefExecAndScriptFailure(['var x = 3', 'if x', 'endif'], 'E1023:', 2)

  v9.CheckDefAndScriptFailure(["var x = [] || false"], ['E1012: Type mismatch; expected bool but got list<any>', 'E745:'], 1)

  var lines =<< trim END
    vim9script
    echo false
      ||true
    # comment
  END
  v9.CheckScriptFailure(lines, 'E1004: White space required before and after ''||'' at "||true"', 3)

  lines =<< trim END
      var x = false
              || false
              || a.b
  END
  v9.CheckDefFailure(lines, 'E1001:', 3)
enddef

" test &&
def Test_expr3()
  var lines =<< trim END
      assert_equal(false, 1 && 0)
      assert_equal(false, 0 &&
                    0 &&
                    1)
      assert_equal(true, 1
                        && true
                        && 1)
      assert_equal(false, 0 && 0)
      assert_equal(false, 0 && false)
      assert_equal(true, 1 && true)

      g:vals = []
      assert_equal(true, g:Record(true) && g:Record(1))
      assert_equal([true, 1], g:vals)

      g:vals = []
      assert_equal(true, g:Record(1) && g:Record(true))
      assert_equal([1, true], g:vals)

      g:vals = []
      assert_equal(false, g:Record(0) && g:Record(1))
      assert_equal([0], g:vals)

      g:vals = []
      assert_equal(false, g:Record(0) && g:Record(1) && g:Record(0))
      assert_equal([0], g:vals)

      g:vals = []
      assert_equal(false, g:Record(0) && g:Record(4) && g:Record(0))
      assert_equal([0], g:vals)

      g:vals = []
      assert_equal(false, g:Record(1) && g:Record(true) && g:Record(0))
      assert_equal([1, true, 0], g:vals)

      var failed = false
      if false && g:a == g:b.c
        failed = true
      endif
      assert_false(failed)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr3_vimscript()
  # check line continuation
  var lines =<< trim END
      var name = 0
		&& 1
      assert_equal(false, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = v:true
		&& v:true
		&& v:true
      assert_equal(v:true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = v:true &&
		v:true &&
		v:true
      assert_equal(v:true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = v:true &&  # comment
                # comment
		v:true &&
                # comment
		v:true
      assert_equal(v:true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # check white space
  lines =<< trim END
      var name = v:true&&v:true
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      var name = v:true &&v:true
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''&&'' at "&&v:true"', 1)

  lines =<< trim END
      var name = v:true&& v:true
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)
enddef

def Test_expr3_fails()
  var msg = "White space required before and after '&&'"
  v9.CheckDefAndScriptFailure(["var x = 1&&0"], msg, 1)
  v9.CheckDefAndScriptFailure(["var x = 1 &&0"], msg, 1)
  v9.CheckDefAndScriptFailure(["var x = 1&& 0"], msg, 1)
  var lines =<< trim END
    var x = 1
      &&0
    # comment
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''&&'' at "&&0"', 2)

  g:vals = []
  v9.CheckDefAndScriptFailure(["if 'yes' && 0", 'echo 0', 'endif'], ['E1012: Type mismatch; expected bool but got string', 'E1135: Using a String as a Bool'], 1)

  v9.CheckDefExecAndScriptFailure(['assert_equal(false, g:Record(1) && g:Record(4) && g:Record(0))'], 'E1023: Using a Number as a Bool: 4', 1)

  lines =<< trim END
      if 3
          && true
      endif
  END
  v9.CheckDefAndScriptFailure(lines, ['E1012:', 'E1023:'], 1)

  lines =<< trim END
      if true
          && 3
      endif
  END
  v9.CheckDefAndScriptFailure(lines, ['E1012:', 'E1023:'], 2)

  lines =<< trim END
      if 'yes'
          && true
      endif
  END
  v9.CheckDefAndScriptFailure(lines, ['E1012:', 'E1135: Using a String as a Bool'], 1)

  lines =<< trim END
      var s = 'asdf'
      echo true && s
  END
  v9.CheckDefAndScriptFailure(lines, ['E1012: Type mismatch; expected bool but got string', 'E1135: Using a String as a Bool: "asdf"'])
enddef

" global variables to use for tests with the "any" type
let atrue = v:true
let afalse = v:false
let anone = v:none
let anull = v:null
let anint = 10
let theone = 1
let thefour = 4
let afloat = 0.1
let astring = 'asdf'
let ablob = 0z01ab
let alist = [2, 3, 4]
let adict = #{aaa: 2, bbb: 8}

" test == comparator
def Test_expr4_equal()
  var lines =<< trim END
      var trueVar = true
      var falseVar = false
      assert_equal(true, true == true)
      assert_equal(false, true ==
                            false)
      assert_equal(true, true
                            == trueVar)
      assert_equal(false, true == falseVar)
      assert_equal(true, true == g:atrue)
      assert_equal(false, g:atrue == false)

      assert_equal(true, v:none == v:none)
      assert_equal(false, v:none == v:null)
      assert_equal(true, g:anone == v:none)
      assert_equal(true, null == v:null)
      assert_equal(true, null == g:anull)
      assert_equal(false, v:none == g:anull)

      var nr0 = 0
      var nr61 = 61
      assert_equal(false, 2 == 0)
      assert_equal(false, 2 == nr0)
      assert_equal(true, 61 == 61)
      assert_equal(true, 61 == nr61)
      assert_equal(true, g:anint == 10)
      assert_equal(false, 61 == g:anint)

      var ff = 0.3
      assert_equal(true, ff == 0.3)
      assert_equal(false, 0.4 == ff)
      assert_equal(true, 0.1 == g:afloat)
      assert_equal(false, g:afloat == 0.3)

      ff = 3.0
      assert_equal(true, ff == 3)
      assert_equal(true, 3 == ff)
      ff = 3.1
      assert_equal(false, ff == 3)
      assert_equal(false, 3 == ff)

      assert_equal(true, 'abc' == 'abc')
      assert_equal(false, 'xyz' == 'abc')
      assert_equal(true, g:astring == 'asdf')
      assert_equal(false, 'xyz' == g:astring)

      assert_equal(false, 'abc' == 'aBc')
      assert_equal(false, 'abc' ==# 'aBc')
      assert_equal(true, 'abc' ==? 'aBc')

      assert_equal(false, 'abc' == 'ABC')
      set ignorecase
      assert_equal(false, 'abc' == 'ABC')
      assert_equal(false, 'abc' ==# 'ABC')
      assert_equal(true, 'abc' ==? 'ABC')
      set noignorecase

      var bb = 0z3f
      assert_equal(true, 0z3f == bb)
      assert_equal(false, bb == 0z4f)
      assert_equal(true, g:ablob == 0z01ab)
      assert_equal(false, 0z3f == g:ablob)

      assert_equal(true, [1, 2, 3] == [1, 2, 3])
      assert_equal(false, [1, 2, 3] == [2, 3, 1])
      assert_equal(true, [2, 3, 4] == g:alist)
      assert_equal(false, g:alist == [2, 3, 1])
      assert_equal(false, [1, 2, 3] == [])
      assert_equal(false, [1, 2, 3] == ['1', '2', '3'])

      assert_equal(true, {one: 1, two: 2} == {one: 1, two: 2})
      assert_equal(false, {one: 1, two: 2} == {one: 2, two: 2})
      assert_equal(false, {one: 1, two: 2} == {two: 2})
      assert_equal(false, {one: 1, two: 2} == {})
      assert_equal(true, g:adict == {bbb: 8, aaa: 2})
      assert_equal(false, {ccc: 9, aaa: 2} == g:adict)

      assert_equal(true, function('g:Test_expr4_equal') == function('g:Test_expr4_equal'))
      assert_equal(false, function('g:Test_expr4_equal') == function('g:Test_expr4_is'))

      assert_equal(true, function('g:Test_expr4_equal', [123]) == function('g:Test_expr4_equal', [123]))
      assert_equal(false, function('g:Test_expr4_equal', [123]) == function('g:Test_expr4_is', [123]))
      assert_equal(false, function('g:Test_expr4_equal', [123]) == function('g:Test_expr4_equal', [999]))

      if true
        var OneFunc: func
        var TwoFunc: func
        OneFunc = function('len')
        TwoFunc = function('len')
        assert_equal(true, OneFunc('abc') == TwoFunc('123'))
      endif

      # check this doesn't fail when skipped
      if false
        var OneFunc: func
        var TwoFunc: func
        OneFunc = function('len')
        TwoFunc = function('len')
        assert_equal(true, OneFunc('abc') == TwoFunc('123'))
      endif
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["var x = 'a' == xxx"], ['E1001:', 'E121:'], 1)
  v9.CheckDefFailure(["var x = 'a' == "], 'E1097:', 3)
  v9.CheckScriptFailure(['vim9script', "var x = 'a' == "], 'E15:', 2)

  v9.CheckDefExecAndScriptFailure(['var items: any', 'eval 1 + 1', 'eval 2 + 2', 'if items == []', 'endif'], ['E691:', 'E1072:'], 4)

  v9.CheckDefExecAndScriptFailure(['var x: any = "a"', 'echo x == true'], 'E1072: Cannot compare string with bool', 2)
  v9.CheckDefExecAndScriptFailure(["var x: any = true", 'echo x == ""'], 'E1072: Cannot compare bool with string', 2)
  v9.CheckDefExecAndScriptFailure(["var x: any = 99", 'echo x == true'], ['E1138', 'E1072:'], 2)
  v9.CheckDefExecAndScriptFailure(["var x: any = 'a'", 'echo x == 99'], ['E1030:', 'E1072:'], 2)

  lines =<< trim END
      vim9script
      var n: any = 2
      def Compare()
        eval n == '3'
        g:notReached = false
      enddef
      g:notReached = true
      Compare()
  END
  v9.CheckScriptFailure(lines, 'E1030: Using a String as a Number: "3"')
  assert_true(g:notReached)

  lines =<< trim END
      vim9script
      var n: any = 2.2
      def Compare()
        eval n == '3'
        g:notReached = false
      enddef
      g:notReached = true
      Compare()
  END
  v9.CheckScriptFailure(lines, 'E892: Using a String as a Float')
  assert_true(g:notReached)

  unlet g:notReached
enddef

def Test_expr4_compare_null()
  g:null_dict = test_null_dict()
  g:not_null_list = []
  var lines =<< trim END
      assert_false(true == null)
      assert_false(false == null)
      assert_false(null == true)
      assert_false(null == false)
      assert_true(true != null)
      assert_true(false != null)
      assert_true(null != true)
      assert_true(null != false)

      assert_false(123 == null)
      assert_false(0 == null)
      assert_false(null == 123)
      assert_false(null == 0)
      assert_true(123 != null)
      assert_true(0 != null)
      assert_true(null != 123)
      assert_true(null != 0)

      assert_false(12.3 == null)
      assert_false(0.0 == null)
      assert_false(null == 12.3)
      assert_false(null == 0.0)
      assert_true(12.3 != null)
      assert_true(0.0 != null)
      assert_true(null != 12.3)
      assert_true(null != 0.0)

      assert_true(test_null_blob() == v:null)
      assert_true(null_blob == null)
      assert_true(v:null == test_null_blob())
      assert_true(null == null_blob)
      assert_false(test_null_blob() != v:null)
      assert_false(null_blob != null)
      assert_false(v:null != test_null_blob())
      assert_false(null != null_blob)

      var nb = null_blob
      assert_true(nb == null_blob)
      assert_true(nb == null)
      assert_true(null_blob == nb)
      assert_true(null == nb)

      if has('channel')
        assert_true(test_null_channel() == v:null)
        assert_true(null_channel == null)
        assert_true(v:null == test_null_channel())
        assert_true(null == null_channel)
        assert_false(test_null_channel() != v:null)
        assert_false(null_channel != null)
        assert_false(v:null != test_null_channel())
        assert_false(null != null_channel)

        var nc = null_channel
        assert_true(nc == null_channel)
        assert_true(nc == null)
        assert_true(null_channel == nc)
        assert_true(null == nc)
      endif

      assert_true(test_null_dict() == v:null)
      assert_true(null_dict == null)
      assert_true(v:null == test_null_dict())
      assert_true(null == null_dict)
      assert_false(test_null_dict() != v:null)
      assert_false(null_dict != null)
      assert_false(v:null != test_null_dict())
      assert_false(null != null_dict)

      assert_true(g:null_dict == v:null)
      assert_true(v:null == g:null_dict)
      assert_false(g:null_dict != v:null)
      assert_false(v:null != g:null_dict)

      var nd = null_dict
      assert_true(nd == null_dict)
      assert_true(nd == null)
      assert_true(null_dict == nd)
      assert_true(null == nd)

      assert_true(test_null_function() == v:null)
      assert_true(null_function == null)
      assert_true(v:null == test_null_function())
      assert_true(null == null_function)
      assert_false(test_null_function() != v:null)
      assert_false(null_function != null)
      assert_false(v:null != test_null_function())
      assert_false(null != null_function)

      var Nf = null_function
      assert_true(Nf == null_function)
      assert_true(Nf == null)
      assert_true(null_function == Nf)
      assert_true(null == Nf)

      if has('job')
        assert_true(test_null_job() == v:null)
        assert_true(null_job == null)
        assert_true(v:null == test_null_job())
        assert_true(null == null_job)
        assert_false(test_null_job() != v:null)
        assert_false(null_job != null)
        assert_false(v:null != test_null_job())
        assert_false(null != null_job)

        var nj = null_job
        assert_true(nj == null_job)
        assert_true(nj == null)
        assert_true(null_job == nj)
        assert_true(null == nj)
      endif

      assert_true(test_null_list() == v:null)
      assert_true(null_list == null)
      assert_true(v:null == test_null_list())
      assert_true(null == null_list)
      assert_false(test_null_list() != v:null)
      assert_false(null_list != null)
      assert_false(v:null != test_null_list())
      assert_false(null != null_list)

      assert_false(g:not_null_list == v:null)
      assert_false(v:null == g:not_null_list)
      assert_true(g:not_null_list != v:null)
      assert_true(v:null != g:not_null_list)

      var nl = null_list
      assert_true(nl == null_list)
      assert_true(nl == null)
      assert_true(null_list == nl)
      assert_true(null == nl)

      assert_true(test_null_partial() == v:null)
      assert_true(null_partial == null)
      assert_true(v:null == test_null_partial())
      assert_true(null == null_partial)
      assert_false(test_null_partial() != v:null)
      assert_false(null_partial != null)
      assert_false(v:null != test_null_partial())
      assert_false(null != null_partial)

      var Np = null_partial
      assert_true(Np == null_partial)
      assert_true(Np == null)
      assert_true(null_partial == Np)
      assert_true(null == Np)

      assert_true(test_null_string() == v:null)
      assert_true(null_string == null)
      assert_true(v:null == test_null_string())
      assert_true(null == null_string)
      assert_false(test_null_string() != v:null)
      assert_false(null_string != null)
      assert_false(v:null != test_null_string())
      assert_false(null != null_string)

      assert_true(null_string is test_null_string())
      assert_false(null_string is '')
      assert_false('' is null_string)
      assert_false(null_string isnot test_null_string())
      assert_true(null_string isnot '')
      assert_true('' isnot null_string)

      var ns = null_string
      assert_true(ns == null_string)
      assert_true(ns == null)
      assert_true(null_string == ns)
      assert_true(null == ns)
  END
  v9.CheckDefAndScriptSuccess(lines)
  unlet g:null_dict
  unlet g:not_null_list

  # variables declared at script level used in a :def function
  lines =<< trim END
      vim9script

      var l_decl: list<number>
      var l_empty = []
      var l_null = null_list

      def TestList()
        assert_false(l_decl == null)
        assert_false(l_decl is null_list)
        assert_false(l_empty == null)
        assert_false(l_empty is null_list)
        assert_true(l_null == null)
        assert_true(l_null is null_list)
        assert_true(l_null == null_list)

        add(l_decl, 6)
        assert_equal([6], l_decl)
        add(l_empty, 7)
        assert_equal([7], l_empty)
        var caught = false
        try
          add(l_null, 9)
        catch /E1130:/
          caught = true
        endtry
        assert_true(caught)
      enddef
      TestList()

      var b_decl: blob
      var b_empty = 0z
      var b_null = null_blob

      def TestBlob()
        assert_false(b_decl == null)
        assert_false(b_decl is null_blob)
        assert_false(b_empty == null)
        assert_false(b_empty is null_blob)
        assert_true(b_null == null)
        assert_true(b_null is null_blob)
        assert_true(b_null == null_blob)

        add(b_decl, 6)
        assert_equal(0z06, b_decl)
        add(b_empty, 7)
        assert_equal(0z07, b_empty)
        var caught = false
        try
          add(b_null, 9)
        catch /E1131:/
          caught = true
        endtry
        assert_true(caught)
      enddef
      TestBlob()

      var d_decl: dict<number>
      var d_empty = {}
      var d_null = null_dict

      def TestDict()
        assert_false(d_decl == null)
        assert_false(d_decl is null_dict)
        assert_false(d_empty == null)
        assert_false(d_empty is null_dict)
        assert_true(d_null == null)
        assert_true(d_null is null_dict)
        assert_true(d_null == null_dict)

        d_decl['a'] = 6
        assert_equal({a: 6}, d_decl)
        d_empty['b'] = 7
        assert_equal({b: 7}, d_empty)
        var caught = false
        try
          d_null['c'] = 9
        catch /E1103:/
          caught = true
        endtry
        assert_true(caught)
      enddef
      TestDict()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      var d: dict<func> = {f: null_function}
      assert_equal(null_function, d.f)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr4_compare_none()
  var lines =<< trim END
      assert_false('' == v:none)
      assert_false('text' == v:none)
      assert_true(v:none == v:none)
      assert_false(v:none == '')
      assert_false(v:none == 'text')
      assert_true(v:none == v:none)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(['echo [] == v:none'], 'E1072: Cannot compare list with special')
  v9.CheckDefAndScriptFailure(['echo 123 == v:none'], 'E1072: Cannot compare number with special')
  v9.CheckDefAndScriptFailure(['echo 0z00 == v:none'], 'E1072: Cannot compare blob with special')

  lines =<< trim END
      echo [] == v:none

      eval 0 + 0
  END
  v9.CheckDefAndScriptFailure(lines, 'E1072:', 1)
enddef

def Test_expr4_wrong_type()
  for op in ['>', '>=', '<', '<=', '=~', '!~']
    v9.CheckDefExecAndScriptFailure([
        "var a: any = 'a'",
        'var b: any = true',
        'echo a ' .. op .. ' b'], 'E1072:', 3)
  endfor
  for op in ['>', '>=', '<', '<=']
    v9.CheckDefExecAndScriptFailure([
        "var n: any = 2",
        'echo n ' .. op .. ' "3"'], ['E1030:', 'E1072:'], 2)
  endfor
  for op in ['=~', '!~']
    v9.CheckDefExecAndScriptFailure([
        "var n: any = 2",
        'echo n ' .. op .. ' "3"'], 'E1072:', 2)
  endfor

  v9.CheckDefAndScriptFailure([
      'echo v:none == true'], 'E1072:', 1)
  v9.CheckDefAndScriptFailure([
      'echo false >= true'], 'E1072:', 1)
  v9.CheckDefExecAndScriptFailure([
      "var n: any = v:none",
      'echo n == true'], 'E1072:', 2)
  v9.CheckDefExecAndScriptFailure([
      "var n: any = v:none",
      'echo n < true'], 'E1072:', 2)
enddef

" test != comparator
def Test_expr4_notequal()
  var lines =<< trim END
      var trueVar = true
      var falseVar = false
      assert_equal(false, true != true)
      assert_equal(true, true !=
                            false)
      assert_equal(false, true
                            != trueVar)
      assert_equal(true, true != falseVar)
      assert_equal(false, true != g:atrue)
      assert_equal(true, g:atrue != false)

      assert_equal(false, v:none != v:none)
      assert_equal(true, v:none != v:null)
      assert_equal(false, g:anone != v:none)
      assert_equal(true, v:none != g:anull)

      var nr55 = 55
      var nr0 = 55
      assert_equal(true, 2 != 0)
      assert_equal(true, 2 != nr0)
      assert_equal(false, 55 != 55)
      assert_equal(false, 55 != nr55)
      assert_equal(false, g:anint != 10)
      assert_equal(true, 61 != g:anint)

      var ff = 0.3
      assert_equal(false, 0.3 != ff)
      assert_equal(true, 0.4 != ff)
      assert_equal(false, 0.1 != g:afloat)
      assert_equal(true, g:afloat != 0.3)

      ff = 3.0
      assert_equal(false, ff != 3)
      assert_equal(false, 3 != ff)
      ff = 3.1
      assert_equal(true, ff != 3)
      assert_equal(true, 3 != ff)

      assert_equal(false, 'abc' != 'abc')
      assert_equal(true, 'xyz' != 'abc')
      assert_equal(false, g:astring != 'asdf')
      assert_equal(true, 'xyz' != g:astring)

      assert_equal(true, 'abc' != 'ABC')
      set ignorecase
      assert_equal(true, 'abc' != 'ABC')
      assert_equal(true, 'abc' !=# 'ABC')
      assert_equal(false, 'abc' !=? 'ABC')
      set noignorecase

      var bb = 0z3f
      assert_equal(false, 0z3f != bb)
      assert_equal(true, bb != 0z4f)
      assert_equal(false, g:ablob != 0z01ab)
      assert_equal(true, 0z3f != g:ablob)

      assert_equal(false, [1, 2, 3] != [1, 2, 3])
      assert_equal(true, [1, 2, 3] != [2, 3, 1])
      assert_equal(false, [2, 3, 4] != g:alist)
      assert_equal(true, g:alist != [2, 3, 1])
      assert_equal(true, [1, 2, 3] != [])
      assert_equal(true, [1, 2, 3] != ['1', '2', '3'])

      assert_equal(false, {one: 1, two: 2} != {one: 1, two: 2})
      assert_equal(true, {one: 1, two: 2} != {one: 2, two: 2})
      assert_equal(true, {one: 1, two: 2} != {two: 2})
      assert_equal(true, {one: 1, two: 2} != {})
      assert_equal(false, g:adict != {bbb: 8, aaa: 2})
      assert_equal(true, {ccc: 9, aaa: 2} != g:adict)

      assert_equal(false, function('g:Test_expr4_equal') != function('g:Test_expr4_equal'))
      assert_equal(true, function('g:Test_expr4_equal') != function('g:Test_expr4_is'))

      assert_equal(false, function('g:Test_expr4_equal', [123]) != function('g:Test_expr4_equal', [123]))
      assert_equal(true, function('g:Test_expr4_equal', [123]) != function('g:Test_expr4_is', [123]))
      assert_equal(true, function('g:Test_expr4_equal', [123]) != function('g:Test_expr4_equal', [999]))
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

" test > comparator
def Test_expr4_greater()
  var lines =<< trim END
      assert_true(2 > 0)
      assert_true(2 >
                    1)
      assert_false(2 > 2)
      assert_false(2 > 3)
      var nr2 = 2
      assert_true(nr2 > 0)
      assert_true(nr2 >
                    1)
      assert_false(nr2 > 2)
      assert_false(nr2
                        > 3)
      var ff = 2.0
      assert_true(ff > 0.0)
      assert_true(ff > 1.0)
      assert_false(ff > 2.0)
      assert_false(ff > 3.0)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

" test >= comparator
def Test_expr4_greaterequal()
  var lines =<< trim END
      assert_true(2 >= 0)
      assert_true(2 >=
                            2)
      assert_false(2 >= 3)
      var nr2 = 2
      assert_true(nr2 >= 0)
      assert_true(nr2 >= 2)
      assert_false(nr2 >= 3)
      var ff = 2.0
      assert_true(ff >= 0.0)
      assert_true(ff >= 2.0)
      assert_false(ff >= 3.0)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

" test < comparator
def Test_expr4_smaller()
  var lines =<< trim END
      assert_false(2 < 0)
      assert_false(2 <
                            2)
      assert_true(2
                    < 3)
      var nr2 = 2
      assert_false(nr2 < 0)
      assert_false(nr2 < 2)
      assert_true(nr2 < 3)
      var ff = 2.0
      assert_false(ff < 0.0)
      assert_false(ff < 2.0)
      assert_true(ff < 3.0)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

" test <= comparator
def Test_expr4_smallerequal()
  var lines =<< trim END
      assert_false(2 <= 0)
      assert_false(2 <=
                            1)
      assert_true(2
                    <= 2)
      assert_true(2 <= 3)
      var nr2 = 2
      assert_false(nr2 <= 0)
      assert_false(nr2 <= 1)
      assert_true(nr2 <= 2)
      assert_true(nr2 <= 3)
      var ff = 2.0
      assert_false(ff <= 0.0)
      assert_false(ff <= 1.0)
      assert_true(ff <= 2.0)
      assert_true(ff <= 3.0)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

" test =~ comparator
def Test_expr4_match()
  var lines =<< trim END
      assert_equal(false, '2' =~ '0')
      assert_equal(false, ''
                             =~ '0')
      assert_equal(true, '2' =~
                            '[0-9]')
      set ignorecase
      assert_equal(false, 'abc' =~ 'ABC')
      assert_equal(false, 'abc' =~# 'ABC')
      assert_equal(true, 'abc' =~? 'ABC')
      set noignorecase
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

" test !~ comparator
def Test_expr4_nomatch()
  var lines =<< trim END
      assert_equal(true, '2' !~ '0')
      assert_equal(true, ''
                            !~ '0')
      assert_equal(false, '2' !~
                            '[0-9]')
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

" test is comparator
def Test_expr4_is()
  var lines =<< trim END
      var mylist = [2]
      assert_false(mylist is [2])
      var other = mylist
      assert_true(mylist is
                    other)

      var myblob = 0z1234
      assert_false(myblob
                            is 0z1234)
      var otherblob = myblob
      assert_true(myblob is otherblob)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

" test isnot comparator
def Test_expr4_isnot()
  var lines =<< trim END
      var mylist = [2]
      assert_true('2' isnot '0')
      assert_true(mylist isnot [2])
      var other = mylist
      assert_false(mylist isnot
                            other)

      var myblob = 0z1234
      assert_true(myblob
                    isnot 0z1234)
      var otherblob = myblob
      assert_false(myblob isnot otherblob)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def g:RetVoid()
  var x = 1
enddef

def Test_expr4_vim9script()
  # check line continuation
  var lines =<< trim END
      var name = 0
		< 1
      assert_equal(true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = 123
                # comment
		!= 123
      assert_equal(false, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = 123 ==
			123
      assert_equal(true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var list = [1, 2, 3]
      var name = list
		is list
      assert_equal(true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var list = [1, 2, 3]
      var name = list # comment
                 # comment
		is list
      assert_equal(true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var myblob = 0z1234
      var name = myblob
		isnot 0z11
      assert_equal(true, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # spot check mismatching types
  lines =<< trim END
      echo '' == 0
  END
  v9.CheckDefAndScriptFailure(lines, 'E1072:', 1)

  lines =<< trim END
      echo v:true > v:false
  END
  v9.CheckDefAndScriptFailure(lines, 'Cannot compare bool with bool', 1)

  lines =<< trim END
      echo 123 is 123
  END
  v9.CheckDefAndScriptFailure(lines, 'Cannot use "is" with number', 1)

  # check missing white space
  lines =<< trim END
    echo 2>3
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''>'' at ">3"', 1)

  lines =<< trim END
    echo 2 >3
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
    echo 2> 3
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
    echo 2!=3
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
    echo 2 !=3
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''!='' at "!=3"', 1)

  lines =<< trim END
    echo 2!= 3
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  for op in ['==', '>', '>=', '<', '<=', '=~', '!~', 'is', 'isnot']
    lines = ["echo 'aaa'", op .. "'bbb'", '# comment']
    var msg = printf("E1004: White space required before and after '%s'", op)
    v9.CheckDefAndScriptFailure(lines, msg, 2)
  endfor

  lines =<< trim END
    echo len('xxx') == 3
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
    var line = 'abc'
    echo line[1] =~ '\w'
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

func Test_expr4_fails()
  let msg = "White space required before and after '>'"
  call v9.CheckDefAndScriptFailure(["var x = 1>2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 >2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1> 2"], msg, 1)

  let msg = "White space required before and after '=='"
  call v9.CheckDefAndScriptFailure(["var x = 1==2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 ==2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1== 2"], msg, 1)

  let msg = "White space required before and after 'is'"
  call v9.CheckDefAndScriptFailure(["var x = '1'is'2'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = '1' is'2'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = '1'is '2'"], msg, 1)

  let msg = "White space required before and after 'isnot'"
  call v9.CheckDefAndScriptFailure(["var x = '1'isnot'2'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = '1' isnot'2'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = '1'isnot '2'"], msg, 1)

  call v9.CheckDefAndScriptFailure(["var x = 1 is# 2"], 'E15:', 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 is? 2"], 'E15:', 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 isnot# 2"], 'E15:', 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 isnot? 2"], 'E15:', 1)

  call v9.CheckDefAndScriptFailure(["var x = 1 == '2'"], 'Cannot compare number with string', 1)
  call v9.CheckDefAndScriptFailure(["var x = '1' == 2"], 'Cannot compare string with number', 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 == g:RetVoid()"], 'Cannot compare number with void', 1)
  call v9.CheckDefAndScriptFailure(["var x = g:RetVoid() == 1"], 'Cannot compare void with number', 1)

  call v9.CheckDefAndScriptFailure(["var x = true > false"], 'Cannot compare bool with bool', 1)
  call v9.CheckDefAndScriptFailure(["var x = true >= false"], 'Cannot compare bool with bool', 1)
  call v9.CheckDefAndScriptFailure(["var x = true < false"], 'Cannot compare bool with bool', 1)
  call v9.CheckDefAndScriptFailure(["var x = true <= false"], 'Cannot compare bool with bool', 1)
  call v9.CheckDefAndScriptFailure(["var x = true =~ false"], 'Cannot compare bool with bool', 1)
  call v9.CheckDefAndScriptFailure(["var x = true !~ false"], 'Cannot compare bool with bool', 1)
  call v9.CheckDefAndScriptFailure(["var x = true is false"], 'Cannot use "is" with bool', 1)
  call v9.CheckDefAndScriptFailure(["var x = true isnot false"], 'Cannot use "isnot" with bool', 1)

  call v9.CheckDefAndScriptFailure(["var x = v:none is v:null"], 'Cannot use "is" with special', 1)
  call v9.CheckDefAndScriptFailure(["var x = v:none isnot v:null"], 'Cannot use "isnot" with special', 1)
  call v9.CheckDefAndScriptFailure(["var x = 123 is 123"], 'Cannot use "is" with number', 1)
  call v9.CheckDefAndScriptFailure(["var x = 123 isnot 123"], 'Cannot use "isnot" with number', 1)
  call v9.CheckDefAndScriptFailure(["var x = 1.3 is 1.3"], 'Cannot use "is" with float', 1)
  call v9.CheckDefAndScriptFailure(["var x = 1.3 isnot 1.3"], 'Cannot use "isnot" with float', 1)

  call v9.CheckDefAndScriptFailure(["var x = 0za1 > 0z34"], 'Cannot compare blob with blob', 1)
  call v9.CheckDefAndScriptFailure(["var x = 0za1 >= 0z34"], 'Cannot compare blob with blob', 1)
  call v9.CheckDefAndScriptFailure(["var x = 0za1 < 0z34"], 'Cannot compare blob with blob', 1)
  call v9.CheckDefAndScriptFailure(["var x = 0za1 <= 0z34"], 'Cannot compare blob with blob', 1)
  call v9.CheckDefAndScriptFailure(["var x = 0za1 =~ 0z34"], 'Cannot compare blob with blob', 1)
  call v9.CheckDefAndScriptFailure(["var x = 0za1 !~ 0z34"], 'Cannot compare blob with blob', 1)

  call v9.CheckDefAndScriptFailure(["var x = [13] > [88]"], 'Cannot compare list with list', 1)
  call v9.CheckDefAndScriptFailure(["var x = [13] >= [88]"], 'Cannot compare list with list', 1)
  call v9.CheckDefAndScriptFailure(["var x = [13] < [88]"], 'Cannot compare list with list', 1)
  call v9.CheckDefAndScriptFailure(["var x = [13] <= [88]"], 'Cannot compare list with list', 1)
  call v9.CheckDefAndScriptFailure(["var x = [13] =~ [88]"], 'Cannot compare list with list', 1)
  call v9.CheckDefAndScriptFailure(["var x = [13] !~ [88]"], 'Cannot compare list with list', 1)

  if has('job')
    call v9.CheckDefAndScriptFailure(['var j: job', 'var chan: channel', 'var r = j == chan'], 'Cannot compare job with channel', 3)
    call v9.CheckDefAndScriptFailure(['var j: job', 'var x: list<any>', 'var r = j == x'], 'Cannot compare job with list', 3)
    call v9.CheckDefAndScriptFailure(['var j: job', 'var Xx: func', 'var r = j == Xx'], 'Cannot compare job with func', 3)
    call v9.CheckDefAndScriptFailure(['var j: job', 'var Xx: func', 'var r = j == Xx'], 'Cannot compare job with func', 3)
  endif
endfunc

" test bitwise left and right shift operators
" The tests for this is in test_expr.vim (Test_bitwise_shift)
" def Test_expr5()
" enddef

" test addition, subtraction, concatenation
def Test_expr6()
  var lines =<< trim END
      assert_equal(66, 60 + 6)
      assert_equal(70, 60 +
                            g:anint)
      assert_equal(9, g:thefour
                            + 5)
      assert_equal(14, g:thefour + g:anint)
      assert_equal([1, 2, 3, 4], [1] + g:alist)

      assert_equal(54, 60 - 6)
      assert_equal(50, 60 -
                        g:anint)
      assert_equal(-1, g:thefour
                            - 5)
      assert_equal(-6, g:thefour - g:anint)

      assert_equal('hello', 'hel' .. 'lo')
      assert_equal('hello 123', 'hello ' ..
                                            123)
      assert_equal('hello 123', 'hello '
                                    ..  123)
      assert_equal('123 hello', 123 .. ' hello')
      assert_equal('123456', 123 .. 456)

      assert_equal('atrue', 'a' .. true)
      assert_equal('afalse', 'a' .. false)
      assert_equal('anull', 'a' .. v:null)
      assert_equal('av:none', 'a' .. v:none)
      assert_equal('a0.123', 'a' .. 0.123)

      assert_equal(3, 1 + [2, 3, 4][0])
      assert_equal(5, 2 + {key: 3}['key'])

      set digraph
      assert_equal('val: true', 'val: ' .. &digraph)
      set nodigraph
      assert_equal('val: false', 'val: ' .. &digraph)

      assert_equal([1, 2, 3, 4], [1, 2] + [3, 4])
      assert_equal(0z11223344, 0z1122 + 0z3344)
      assert_equal(0z112201ab, 0z1122
                                    + g:ablob)
      assert_equal(0z01ab3344, g:ablob + 0z3344)
      assert_equal(0z01ab01ab, g:ablob + g:ablob)

      # concatenate non-constant to constant
      var save_path = &path
      &path = 'b'
      assert_equal('ab', 'a' .. &path)
      &path = save_path

      @b = 'b'
      assert_equal('ab', 'a' .. @b)

      $ENVVAR = 'env'
      assert_equal('aenv', 'a' .. $ENVVAR)

      assert_equal('val', '' .. {key: 'val'}['key'])
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr6_vim9script()
  # check line continuation
  var lines =<< trim END
      var name = 11
		+ 77
		- 22
      assert_equal(66, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = 11 +
		  77 -
		  22
      assert_equal(66, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = 11 +  # comment
		  77 -
                  # comment
		  22
      assert_equal(66, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = 'one'
		.. 'two'
      assert_equal('onetwo', name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      echo 'abc' is# 'abc'
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:', 1)

  lines =<< trim END
      echo {} - 22
  END
  v9.CheckDefAndScriptFailure(lines, ['E1036:', 'E728:'], 1)

  lines =<< trim END
      echo [] - 33
  END
  v9.CheckDefAndScriptFailure(lines, ['E1036:', 'E745:'], 1)

  lines =<< trim END
      echo 0z1234 - 44
  END
  v9.CheckDefAndScriptFailure(lines, ['E1036', 'E974:'], 1)

  lines =<< trim END
      echo 'abc' is? 'abc'
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:', 1)

  lines =<< trim END
      echo 'abc' isnot# 'abc'
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:', 1)

  lines =<< trim END
      echo 'abc' isnot? 'abc'
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:', 1)

  # check white space
  lines =<< trim END
      echo 5+6
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)
  lines =<< trim END
      echo 5 +6
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      echo 5+ 6
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      echo 'a'..'b'
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''..'' at "..''b''"', 1)

  lines =<< trim END
      echo 'a' ..'b'
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      echo 'a'.. 'b'
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''..'' at ".. ''b''"', 1)

  lines =<< trim END
      echo 'a'
          ..'b'
      # comment
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004: White space required before and after ''..'' at "..''b''"', 2)

  # check invalid string concatenation
  lines =<< trim END
      echo 'a' .. [1]
  END
  v9.CheckDefAndScriptFailure(lines, ['E1105:', 'E730:'], 1)

  lines =<< trim END
      echo 'a' .. {a: 1}
  END
  v9.CheckDefAndScriptFailure(lines, ['E1105:', 'E731:'], 1)

  lines =<< trim END
      echo 'a' .. test_void()
  END
  v9.CheckDefAndScriptFailure(lines, ['E1105:', 'E908:'], 1)

  lines =<< trim END
      echo 'a' .. 0z33
  END
  v9.CheckDefAndScriptFailure(lines, ['E1105:', 'E976:'], 1)

  lines =<< trim END
      echo 'a' .. function('len')
  END
  v9.CheckDefAndScriptFailure(lines, ['E1105:', 'E729:'], 1)

  lines =<< trim END
      new
      ['']->setline(1)
      /pattern

      eval 0
      bwipe!
  END
  v9.CheckDefAndScriptFailure(lines, "E1004: White space required before and after '/' at \"/pattern", 3)

  for op in ['+', '-']
    lines = ['var x = 1', op .. '2', '# comment']
    var msg = printf("E1004: White space required before and after '%s' at \"%s2\"", op, op)
    v9.CheckDefAndScriptFailure(lines, msg, 2)
  endfor
enddef

def Test_expr6_vim9script_channel()
  if !has('channel')
    MissingFeature 'channel'
  else
    var lines =<< trim END
        echo 'a' .. test_null_job()
    END
    v9.CheckDefAndScriptFailure(lines, ['E1105:', 'E908:'], 1)
    lines =<< trim END
        echo 'a' .. test_null_channel()
    END
    v9.CheckDefAndScriptFailure(lines, ['E1105:', 'E908:'], 1)
  endif
enddef

def Test_expr6_float()
  var lines =<< trim END
      assert_equal(66.0, 60.0 + 6.0)
      assert_equal(66.0, 60.0 + 6)
      assert_equal(66.0, 60 +
                           6.0)
      assert_equal(5.1, g:afloat
                          + 5)
      assert_equal(8.1, 8 + g:afloat)
      assert_equal(10.1, g:anint + g:afloat)
      assert_equal(10.1, g:afloat + g:anint)

      assert_equal(54.0, 60.0 - 6.0)
      assert_equal(54.0, 60.0
                              - 6)
      assert_equal(54.0, 60 - 6.0)
      assert_equal(-4.9, g:afloat - 5)
      assert_equal(7.9, 8 - g:afloat)
      assert_equal(9.9, g:anint - g:afloat)
      assert_equal(-9.9, g:afloat - g:anint)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

func Test_expr6_fails()
  let msg = "White space required before and after '+'"
  call v9.CheckDefAndScriptFailure(["var x = 1+2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 +2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1+ 2"], msg, 1)

  let msg = "White space required before and after '-'"
  call v9.CheckDefAndScriptFailure(["var x = 1-2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 -2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1- 2"], msg, 1)

  let msg = "White space required before and after '..'"
  call v9.CheckDefAndScriptFailure(["var x = '1'..'2'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = '1' ..'2'"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = '1'.. '2'"], msg, 1)

  call v9.CheckDefAndScriptFailure(["var x = 0z1122 + 33"], ['E1051:', 'E974:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 0z1122 + [3]"], ['E1051:', 'E974:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 0z1122 + 'asd'"], ['E1051:', 'E974:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 33 + 0z1122"], ['E1051:', 'E974:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = [3] + 0z1122"], ['E1051:', 'E745:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 'asdf' + 0z1122"], ['E1051:', 'E1030:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 6 + xxx"], ['E1001:', 'E121:'], 1)

  call v9.CheckDefAndScriptFailure(["var x = 'a' .. [1]"], ['E1105:', 'E730:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 'a' .. {a: 1}"], ['E1105:', 'E731:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 'a' .. test_void()"], ['E1105:', 'E908:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 'a' .. 0z32"], ['E1105:', 'E976:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 'a' .. function('len')"], ['E1105:', 'E729:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 'a' .. function('len', ['a'])"], ['E1105:', 'E729:'], 1)

  call v9.CheckDefAndScriptFailure(['var x = 1 + v:none'], ['E1051:', 'E611:'], 1)
  call v9.CheckDefAndScriptFailure(['var x = 1 + v:null'], ['E1051:', 'E611:'], 1)
  call v9.CheckDefAndScriptFailure(['var x = 1 + v:true'], ['E1051:', 'E1138:'], 1)
  call v9.CheckDefAndScriptFailure(['var x = 1 + v:false'], ['E1051:', 'E1138:'], 1)
  call v9.CheckDefAndScriptFailure(['var x = 1 + true'], ['E1051:', 'E1138:'], 1)
  call v9.CheckDefAndScriptFailure(['var x = 1 + false'], ['E1051:', 'E1138:'], 1)
endfunc

func Test_expr6_fails_channel()
  CheckFeature channel

  call v9.CheckDefAndScriptFailure(["var x = 'a' .. test_null_job()"], ['E1105:', 'E908:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 'a' .. test_null_channel()"], ['E1105:', 'E908:'], 1)
endfunc

def Test_expr6_list_add()
  var lines =<< trim END
      # concatenating two lists with same member types is OK
      var d = {}
      for i in ['a'] + ['b']
        d = {[i]: 0}
      endfor

      # concatenating two lists with different member types results in "any"
      var dany = {}
      for i in ['a'] + [12]
        dany[i] = i
      endfor
      assert_equal({a: 'a', 12: 12}, dany)

      # result of glob() is "any", runtime type check
      var sl: list<string> = glob('*.txt', false, true) + ['']

      var lln: list<list<number>> = [[1] + [2]]
      assert_equal([[1, 2]], lln)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var ln: list<number> = [0]
      var lln: list<list<number>> = [ln + []]
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

" test multiply, divide, modulo
def Test_expr7()
  var lines =<< trim END
      assert_equal(36, 6 * 6)
      assert_equal(24, 6 *
                            g:thefour)
      assert_equal(24, g:thefour
                            * 6)
      assert_equal(40, g:anint * g:thefour)

      assert_equal(10, 60 / 6)
      assert_equal(6, 60 /
                            g:anint)
      assert_equal(1, g:anint / 6)
      assert_equal(2, g:anint
                            / g:thefour)

      assert_equal(5, 11 % 6)
      assert_equal(4, g:anint % 6)
      assert_equal(3, 13 %
                            g:anint)
      assert_equal(2, g:anint
                            % g:thefour)

      assert_equal(4, 6 * 4 / 6)

      var x = [2]
      var y = [3]
      assert_equal(5, x[0] + y[0])
      assert_equal(6, x[0] * y[0])
      var xf = [2.0]
      var yf = [3.0]
      assert_equal(5.0, xf[0]
                          + yf[0])
      assert_equal(6.0, xf[0]
                          * yf[0])
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["var x = 6 * xxx"], ['E1001:', 'E121:'], 1)
  v9.CheckDefFailure(["var d = 6 * "], 'E1097:', 3)
  v9.CheckScriptFailure(['vim9script', "var d = 6 * "], 'E15:', 2)

  v9.CheckDefAndScriptFailure(['echo 1 / 0'], 'E1154', 1)
  v9.CheckDefAndScriptFailure(['echo 1 % 0'], 'E1154', 1)

  g:zero = 0
  v9.CheckDefExecFailure(['echo 123 / g:zero'], 'E1154: Divide by zero')
  v9.CheckDefExecFailure(['echo 123 % g:zero'], 'E1154: Divide by zero')

  v9.CheckDefExecAndScriptFailure([
        'g:one = 1.0',
        'g:two = 2.0',
        'echo g:one % g:two',
        ], 'E804', 3)

  lines =<< trim END
    var n = 0
    eval 1 / n
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1154', 2)

  lines =<< trim END
    var n = 0
    eval 1 % n
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1154', 2)
enddef

def Test_expr7_vim9script()
  # check line continuation
  var lines =<< trim END
      var name = 11
		* 22
		/ 3
      assert_equal(80, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = 25
		% 10
      assert_equal(5, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = 25
                # comment

                # comment
		% 10
      assert_equal(5, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var name = 11 *
		22 /
		3
      assert_equal(80, name)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # check white space
  lines =<< trim END
      echo 5*6
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      echo 5 *6
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)

  lines =<< trim END
      echo 5* 6
  END
  v9.CheckDefAndScriptFailure(lines, 'E1004:', 1)
enddef

def Test_expr7_float()
  var lines =<< trim END
      assert_equal(36.0, 6.0 * 6)
      assert_equal(36.0, 6 *
                             6.0)
      assert_equal(36.0, 6.0 * 6.0)
      assert_equal(1.0, g:afloat * g:anint)

      assert_equal(10.0, 60 / 6.0)
      assert_equal(10.0, 60.0 /
                          6)
      assert_equal(10.0, 60.0 / 6.0)
      assert_equal(0.01, g:afloat / g:anint)

      assert_equal(4.0, 6.0 * 4 / 6)
      assert_equal(4.0, 6 *
                          4.0 /
                          6)
      assert_equal(4.0, 6 * 4 / 6.0)
      assert_equal(4.0, 6.0 * 4.0 / 6)
      assert_equal(4.0, 6 * 4.0 / 6.0)
      assert_equal(4.0, 6.0 * 4 / 6.0)
      assert_equal(4.0, 6.0 * 4.0 / 6.0)

      assert_equal(4.0, 6.0 * 4.0 / 6.0)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

func Test_expr7_fails()
  let msg = "White space required before and after '*'"
  call v9.CheckDefAndScriptFailure(["var x = 1*2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 *2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1* 2"], msg, 1)

  let msg = "White space required before and after '/'"
  call v9.CheckDefAndScriptFailure(["var x = 1/2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 /2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1/ 2"], msg, 1)

  let msg = "White space required before and after '%'"
  call v9.CheckDefAndScriptFailure(["var x = 1%2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1 %2"], msg, 1)
  call v9.CheckDefAndScriptFailure(["var x = 1% 2"], msg, 1)

  call v9.CheckDefAndScriptFailure(["var x = '1' * '2'"], ['E1036:', 'E1030:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = '1' / '2'"], ['E1036:', 'E1030:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = '1' % '2'"], ['E1035:', 'E1030:'], 1)

  call v9.CheckDefAndScriptFailure(["var x = 0z01 * 0z12"], ['E1036:', 'E974:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 0z01 / 0z12"], ['E1036:', 'E974:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 0z01 % 0z12"], ['E1035:', 'E974:'], 1)

  call v9.CheckDefAndScriptFailure(["var x = [1] * [2]"], ['E1036:', 'E745:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = [1] / [2]"], ['E1036:', 'E745:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = [1] % [2]"], ['E1035:', 'E745:'], 1)

  call v9.CheckDefAndScriptFailure(["var x = {one: 1} * {two: 2}"], ['E1036:', 'E728:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = {one: 1} / {two: 2}"], ['E1036:', 'E728:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = {one: 1} % {two: 2}"], ['E1035:', 'E728:'], 1)

  call v9.CheckDefAndScriptFailure(["var x = 0xff[1]"], ['E1107:', 'E1062:'], 1)
  call v9.CheckDefAndScriptFailure(["var x = 0.7[1]"], ['E1107:', 'E806:'], 1)

  for op in ['*', '/', '%']
    let lines = ['var x = 1', op .. '2', '# comment']
    let msg = printf("E1004: White space required before and after '%s' at \"%s2\"", op, op)
    call v9.CheckDefAndScriptFailure(lines, msg, 2)
  endfor
endfunc

func Test_expr7_float_fails()
  call v9.CheckDefAndScriptFailure(["var x = 1.0 % 2"], ['E1035:', 'E804:'], 1)
endfunc

" define here to use old style parsing
let g:float_zero = 0.0
let g:float_neg = -9.8
let g:float_big = 9.9e99

let g:blob_empty = 0z
let g:blob_one = 0z01
let g:blob_long = 0z0102.0304

let g:string_empty = ''
let g:string_short = 'x'
let g:string_long = 'abcdefghijklm'
let g:string_special = "ab\ncd\ref\ekk"

let g:special_true = v:true
let g:special_false = v:false
let g:special_null = v:null
let g:special_none = v:none

let g:list_empty = []
let g:list_mixed = [1, 'b', v:false]

let g:dict_empty = {}
let g:dict_one = #{one: 1}

let $TESTVAR = 'testvar'

" type casts
def Test_expr8()
  var lines =<< trim END
      var ls: list<string> = ['a', <string>g:string_empty]
      var ln: list<number> = [<number>g:anint, <number>g:thefour]
      var nr = <number>234
      assert_equal(234, nr)
      var b: bool = <bool>1
      assert_equal(true, b)
      var text =
            <string>
              'text'
      if false
        text = <number>'xxx'
      endif
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["var x = <nr>123"], 'E1010:', 1)
  v9.CheckDefFailure(["var x = <number>"], 'E1097:', 3)
  v9.CheckDefFailure(["var x = <number>string(1)"], 'E1012:', 1)
  v9.CheckScriptFailure(['vim9script', "var x = <number>"], 'E15:', 2)
  v9.CheckDefAndScriptFailure(["var x = <number >123"], 'E1068:', 1)
  v9.CheckDefAndScriptFailure(["var x = <number 123"], 'E1104:', 1)

  lines =<< trim END
      vim9script

      def Sum(v: any): float
        var sum = 0.0
        sum += v
        return sum
      enddef

      const kk = 1
      echo Sum(kk)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script

      def Sum(v: any): float
        var sum = 0.0
        sum += <float>v
        return sum
      enddef

      const kk = 1
      Sum(kk)
  END
  v9.CheckScriptFailure(lines, 'E1012: Type mismatch; expected float but got number')
enddef

" test low level expression
def Test_expr9_number()
  # number constant
  var lines =<< trim END
      assert_equal(0, 0)
      assert_equal(654, 0654)

      assert_equal(6, 0x6)
      assert_equal(15, 0xf)
      assert_equal(255, 0xff)

      const INFTY = 1.0 / 0.0
      def Test()
        assert_equal(1, isinf(INFTY))
        assert_equal(-1, isinf(-INFTY))
      enddef
      Test()
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_float()
  # float constant
  var lines =<< trim END
      assert_equal(g:float_zero, .0)
      assert_equal(g:float_zero, 0.0)
      assert_equal(g:float_neg, -9.8)
      assert_equal(g:float_big, 9.9e99)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_blob()
  # blob constant
  var lines =<< trim END
      assert_equal(g:blob_empty, 0z)
      assert_equal(g:blob_one, 0z01)
      assert_equal(g:blob_long, 0z0102.0304)

      var testblob = 0z010203
      assert_equal(0x01, testblob[0])
      assert_equal(0x02, testblob[1])
      assert_equal(0x03, testblob[-1])
      assert_equal(0x02, testblob[-2])

      assert_equal(0z01, testblob[0 : 0])
      assert_equal(0z0102, testblob[0 : 1])
      assert_equal(0z010203, testblob[0 : 2])
      assert_equal(0z010203, testblob[0 : ])
      assert_equal(0z0203, testblob[1 : ])
      assert_equal(0z0203, testblob[1 : 2])
      assert_equal(0z0203, testblob[1 : -1])
      assert_equal(0z03, testblob[-1 : -1])
      assert_equal(0z02, testblob[-2 : -2])

      # blob slice accepts out of range
      assert_equal(0z, testblob[3 : 3])
      assert_equal(0z, testblob[0 : -4])
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["var x = 0z123"], 'E973:', 1)
enddef

def Test_expr9_string()
  # string constant
  var lines =<< trim END
      assert_equal(g:string_empty, '')
      assert_equal(g:string_empty, "")
      assert_equal(g:string_short, 'x')
      assert_equal(g:string_short, "x")
      assert_equal(g:string_long, 'abcdefghijklm')
      assert_equal(g:string_long, "abcdefghijklm")
      assert_equal(g:string_special, "ab\ncd\ref\ekk")
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(['var x = "abc'], 'E114:', 1)
  v9.CheckDefAndScriptFailure(["var x = 'abc"], 'E115:', 1)
  v9.CheckDefFailure(["if 0", "echo 'xx", "endif"], 'E115', 2)

  # interpolated string
  var val = 'val'
  var vv = $"some {val}"
  assert_equal('some val', vv)
  vv = $'other {val}'
  assert_equal('other val', vv)

  v9.CheckDefAndScriptFailure(['var x = $"foo'], 'E114:', 1)
  v9.CheckDefAndScriptFailure(['var x = $"foo{xxx}"'], ['E1001: Variable not found: xxx', 'E121: Undefined variable: xxx'], 1)

  var x = 'x'
  var vl = 'foo xxx bar xxx baz'
              ->split($'x{x}x')
              ->map((_, v: string) => v =~ 'bar')
  assert_equal([false, true, false], vl)

  # interpolated string in a lambda
  lines =<< trim END
      assert_equal(['gnome-256color', 'xterm-256color'], ['gnome', 'xterm']
              ->map((_, term: string) => $'{term}-256color'))
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_vimvar()
  v:errors = []
  var errs: list<string> = v:errors
  v9.CheckDefFailure(['var errs: list<number> = v:errors'], 'E1012:')

  var old: list<string> = v:oldfiles
  v9.CheckDefFailure(['var old: list<number> = v:oldfiles'], 'E1012:')

  var compl: dict<string> = v:completed_item
  v9.CheckDefFailure(['var compl: dict<number> = v:completed_item'], 'E1012:')

  var args: list<string> = v:argv
  v9.CheckDefFailure(['var args: list<number> = v:argv'], 'E1012:')

  var colors: dict<string> = v:colornames
  v9.CheckDefFailure(['var colors: dict<number> = v:colornames'], 'E1012:')

  v9.CheckDefFailure(["var old: list<number> = v:oldfiles"], 'E1012: Type mismatch; expected list<number> but got list<string>', 1)
  v9.CheckScriptFailure(['vim9script', 'v:oldfiles = ["foo"]', "var old: list<number> = v:oldfiles"], 'E1012: Type mismatch; expected list<number> but got list<string>', 3)
  new
  exec "normal! afoo fo\<C-N>\<Esc>"
  v9.CheckDefExecAndScriptFailure(["var old: dict<number> = v:completed_item"], 'E1012: Type mismatch; expected dict<number> but got dict<string>', 1)
  bwipe!
enddef

def Test_expr9_special()
  # special constant
  var lines =<< trim END
      assert_equal(g:special_true, true)
      assert_equal(g:special_false, false)
      assert_equal(g:special_true, v:true)
      assert_equal(g:special_false, v:false)
      assert_equal(v:true, true)
      assert_equal(v:false, false)

      assert_equal(true, !false)
      assert_equal(false, !true)
      assert_equal(true, !0)
      assert_equal(false, !1)
      assert_equal(false, !!false)
      assert_equal(true, !!true)
      assert_equal(false, !!0)
      assert_equal(true, !!1)

      var t = true
      var f = false
      assert_equal(true, t)
      assert_equal(false, f)

      assert_equal(g:special_null, v:null)
      assert_equal(g:special_null, null)
      assert_equal(g:special_none, v:none)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(['v:true = true'], 'E46:', 1)
  v9.CheckDefAndScriptFailure(['v:true = false'], 'E46:', 1)
  v9.CheckDefAndScriptFailure(['v:false = true'], 'E46:', 1)
  v9.CheckDefAndScriptFailure(['v:null = 11'], 'E46:', 1)
  v9.CheckDefAndScriptFailure(['v:none = 22'], 'E46:', 1)
enddef

def Test_expr9_list()
  # list
  var lines =<< trim END
      assert_equal(g:list_empty, [])
      assert_equal(g:list_empty, [  ])

      var numbers: list<number> = [1, 2, 3]
      numbers = [1]
      numbers = []

      var strings: list<string> = ['a', 'b', 'c']
      strings = ['x']
      strings = []

      var mixed: list<any> = [1, 'b', false,]
      assert_equal(g:list_mixed, mixed)
      assert_equal('b', mixed[1])

      echo [1,
            2] [3,
                    4]

      var llstring: list<list<string>> = [['text'], []]
      llstring = [[], ['text']]
      llstring = [[], []]

      var ls = [null_string]
      assert_equal('list<string>', typename(ls))
      var lb = [null_blob]
      assert_equal('list<blob>', typename(lb))
  END
  v9.CheckDefAndScriptSuccess(lines)

  var rangelist: list<number> = range(3)
  g:rangelist = range(3)
  v9.CheckDefExecAndScriptFailure(["var x: list<string> = g:rangelist"], 'E1012: Type mismatch; expected list<string> but got list<number>', 1)

  v9.CheckDefAndScriptFailure(["var x = 1234[3]"], ['E1107:', 'E1062:'], 1)
  v9.CheckDefExecAndScriptFailure(["var x = g:anint[3]"], 'E1062:', 1)

  v9.CheckDefAndScriptFailure(["var x = g:list_mixed[xxx]"], ['E1001:', 'E121:'], 1)

  v9.CheckDefAndScriptFailure(["var x = [1,2,3]"], 'E1069:', 1)
  v9.CheckDefAndScriptFailure(["var x = [1 ,2, 3]"], 'E1068:', 1)

  v9.CheckDefExecAndScriptFailure(["echo 1", "var x = [][0]", "echo 3"], 'E684:', 2)

  v9.CheckDefExecAndScriptFailure(["var x = g:list_mixed['xx']"], ['E1012:', 'E1030:'], 1)
  v9.CheckDefFailure(["var x = g:list_mixed["], 'E1097:', 3)
  v9.CheckScriptFailure(['vim9script', "var x = g:list_mixed["], 'E15:', 2)
  v9.CheckDefFailure(["var x = g:list_mixed[0"], 'E1097:', 3)
  v9.CheckScriptFailure(['vim9script', "var x = g:list_mixed[0"], 'E111:', 2)
  v9.CheckDefExecAndScriptFailure(["var x = g:list_empty[3]"], 'E684:', 1)
  v9.CheckDefExecAndScriptFailure(["var l: list<number> = [234, 'x']"], 'E1012:', 1)
  v9.CheckDefExecAndScriptFailure(["var l: list<number> = ['x', 234]"], 'E1012:', 1)
  v9.CheckDefExecAndScriptFailure(["var l: list<string> = [234, 'x']"], 'E1012:', 1)
  v9.CheckDefExecAndScriptFailure(["var l: list<string> = ['x', 123]"], 'E1012:', 1)

  lines =<< trim END
      var datalist: list<string>
      def Main()
        datalist += ['x'.
      enddef
      Main()
  END
  v9.CheckDefAndScriptFailure(lines, 'E1127:')

  lines =<< trim END
      var numbers = [1, 2, 3, 4]
      var a = 1
      var b = 2
  END
  v9.CheckDefAndScriptFailure(lines + ['echo numbers[1:b]'],
      'E1004: White space required before and after '':'' at ":b]"', 4)
  v9.CheckDefAndScriptFailure(lines + ['echo numbers[1: b]'], 'E1004:', 4)
  v9.CheckDefAndScriptFailure(lines + ['echo numbers[a :b]'], 'E1004:', 4)
enddef

def Test_expr9_list_vim9script()
  var lines =<< trim END
      var l = [
		11,
		22,
		]
      assert_equal([11, 22], l)

      echo [1,
	    2] [3,
		    4]

      echo [1, # comment
            # comment
	    2] [3,
            # comment
		    4]
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var l = [11,
		22]
      assert_equal([11, 22], l)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var l = [11,22]
  END
  v9.CheckDefAndScriptFailure(lines, 'E1069:', 1)

  lines =<< trim END
      var l = [11 , 22]
  END
  v9.CheckDefAndScriptFailure(lines, 'E1068:', 1)

  lines =<< trim END
    var l: list<number> = [234, 'x']
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:', 1)

  lines =<< trim END
    var l: list<number> = ['x', 234]
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:', 1)

  lines =<< trim END
    var l: list<string> = ['x', 234]
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:', 1)

  lines =<< trim END
    var l: list<string> = [234, 'x']
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:', 1)

  lines =<< trim END
      def Failing()
        job_stop()
      enddef
      var list = [Failing]
  END
  if has('channel')
    v9.CheckDefAndScriptFailure(lines, 'E119:', 0)
  else
    v9.CheckDefAndScriptFailure(lines, 'E117:', 0)
  endif
enddef

def LambdaWithComments(): func
  return (x) =>
            # some comment
            x == 1
            # some comment
            ||
            x == 2
enddef

def LambdaUsingArg(x: number): func
  return () =>
            # some comment
            x == 1
            # some comment
            ||
            x == 2
enddef

def Test_expr9_lambda()
  var lines =<< trim END
      var La = () => 'result'
      # comment
      assert_equal('result', La())
      assert_equal([1, 3, 5], [1, 2, 3]->map((key, val) => key + val))

      # line continuation inside lambda with "cond ? expr : expr" works
      var ll = range(3)
      var dll = mapnew(ll, (k, v) => v % 2 ? {
                ['111']: 111 } : {}
            )
      assert_equal([{}, {111: 111}, {}], dll)

      # comment halfway an expression
      var Ref = () => 4
      # comment
      + 6
      assert_equal(10, Ref())

      ll = range(3)
      map(ll, (k, v) => v == 8 || v
                    == 9
                    || v % 2 ? 111 : 222
            )
      assert_equal([222, 111, 222], ll)

      ll = range(3)
      map(ll, (k, v) => v != 8 && v
                    != 9
                    && v % 2 == 0 ? 111 : 222
            )
      assert_equal([111, 222, 111], ll)

      var dl = [{key: 0}, {key: 22}]->filter(( _, v) => !!v['key'] )
      assert_equal([{key: 22}], dl)

      dl = [{key: 12}, {['foo']: 34}]
      assert_equal([{key: 12}], filter(dl,
            (_, v) => has_key(v, 'key') ? v['key'] == 12 : 0))

      assert_equal(false, g:LambdaWithComments()(0))
      assert_equal(true, g:LambdaWithComments()(1))
      assert_equal(true, g:LambdaWithComments()(2))
      assert_equal(false, g:LambdaWithComments()(3))

      assert_equal(false, g:LambdaUsingArg(0)())
      assert_equal(true, g:LambdaUsingArg(1)())

      var res = map([1, 2, 3], (i: number, v: number) => i + v)
      assert_equal([1, 3, 5], res)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["var Ref = (a)=>a + 1"], 'E1004:')
  v9.CheckDefAndScriptFailure(["var Ref = (a)=> a + 1"], 'E1004: White space required before and after ''=>'' at "=> a + 1"')
  v9.CheckDefAndScriptFailure(["var Ref = (a) =>a + 1"], 'E1004:')
  v9.CheckDefAndScriptFailure(["var Ref = (a) =< a + 1"], ['E1001:', 'E121:'])
  v9.CheckDefAndScriptFailure(["var Ref = (a: int) => a + 1"], 'E1010:')
  v9.CheckDefAndScriptFailure(["var Ref = (a): int => a + 1"], 'E1010:')

  v9.CheckDefAndScriptFailure(["filter([1, 2], (k,v) => 1)"], 'E1069:', 1)
  # error is in first line of the lambda
  v9.CheckDefAndScriptFailure(["var L = (a) => a + b"], 'E1001:', 0)

  assert_equal('xxxyyy', 'xxx'->((a, b) => a .. b)('yyy'))

  v9.CheckDefExecFailure(["var s = 'asdf'->((a) => a)('x')"], 'E118:')
  v9.CheckDefExecFailure(["var s = 'asdf'->((a) => a)('x', 'y')"], 'E118:')
  v9.CheckDefAndScriptFailure(["echo 'asdf'->((a) => a)(x)"], ['E1001:', 'E121:'], 1)

  v9.CheckDefAndScriptSuccess(['var Fx = (a) => ({k1: 0,', ' k2: 1})'])
  v9.CheckDefAndScriptFailure(['var Fx = (a) => ({k1: 0', ' k2: 1})'], 'E722:', 2)
  v9.CheckDefAndScriptFailure(['var Fx = (a) => ({k1: 0,', ' k2 1})'], 'E720:', 2)

  v9.CheckDefAndScriptSuccess(['var Fx = (a) => [0,', ' 1]'])
  v9.CheckDefAndScriptFailure(['var Fx = (a) => [0', ' 1]'], 'E696:', 2)
  v9.CheckDefAndScriptFailure(['var l = [1 2]'], 'E696:', 1)

  # no error for existing script variable when checking for lambda
  lines =<< trim END
    var name = 0
    eval (name + 2) / 3
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_lambda_block()
  var lines =<< trim END
      var Func = (s: string): string => {
                      return 'hello ' .. s
                    }
      assert_equal('hello there', Func('there'))

      var ll = range(3)
      var dll = mapnew(ll, (k, v): string => {
          if v % 2
            return 'yes'
          endif
          return 'no'
        })
      assert_equal(['no', 'yes', 'no'], dll)

      # ignored_inline(0, (_) => {
      #   echo 'body'
      # })

      sandbox var Safe = (nr: number): number => {
          return nr + 7
        }
      assert_equal(10, Safe(3))
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      map([1, 2], (k, v) => { redrawt })
  END
  v9.CheckDefAndScriptFailure(lines, 'E488')

  lines =<< trim END
      var Func = (nr: int) => {
              echo nr
            }
  END
  v9.CheckDefAndScriptFailure(lines, 'E1010', 1)

  lines =<< trim END
      var Func = (nr: number): int => {
              return nr
            }
  END
  v9.CheckDefAndScriptFailure(lines, 'E1010', 1)

  lines =<< trim END
      var Func = (nr: number): int => {
              return nr
  END
  v9.CheckDefFailure(lines, 'E1171', 0)  # line nr is function start
  v9.CheckScriptFailure(['vim9script'] + lines, 'E1171', 2)

  lines =<< trim END
      var Func = (nr: number): int => {
          var ll =<< ENDIT
             nothing
  END
  v9.CheckDefFailure(lines, 'E1145: Missing heredoc end marker: ENDIT', 0)
  v9.CheckScriptFailure(['vim9script'] + lines, 'E1145: Missing heredoc end marker: ENDIT', 2)
enddef

def NewLambdaWithComments(): func
  return (x) =>
            # some comment
            x == 1
            # some comment
            ||
            x == 2
enddef

def NewLambdaUsingArg(x: number): func
  return () =>
            # some comment
            x == 1
            # some comment
            ||
            x == 2
enddef

def Test_expr9_new_lambda()
  var lines =<< trim END
      var La = () => 'result'
      assert_equal('result', La())
      assert_equal([1, 3, 5], [1, 2, 3]->map((key, val) => key + val))

      # line continuation inside lambda with "cond ? expr : expr" works
      var ll = range(3)
      var dll = mapnew(ll, (k, v) => v % 2 ? {
                ['111']: 111 } : {}
            )
      assert_equal([{}, {111: 111}, {}], dll)

      ll = range(3)
      map(ll, (k, v) => v == 8 || v
                    == 9
                    || v % 2 ? 111 : 222
            )
      assert_equal([222, 111, 222], ll)

      ll = range(3)
      map(ll, (k, v) => v != 8 && v
                    != 9
                    && v % 2 == 0 ? 111 : 222
            )
      assert_equal([111, 222, 111], ll)

      var dl = [{key: 0}, {key: 22}]->filter(( _, v) => !!v['key'] )
      assert_equal([{key: 22}], dl)

      dl = [{key: 12}, {['foo']: 34}]
      assert_equal([{key: 12}], filter(dl,
            (_, v) => has_key(v, 'key') ? v['key'] == 12 : 0))

      assert_equal(false, g:NewLambdaWithComments()(0))
      assert_equal(true, g:NewLambdaWithComments()(1))
      assert_equal(true, g:NewLambdaWithComments()(2))
      assert_equal(false, g:NewLambdaWithComments()(3))

      assert_equal(false, g:NewLambdaUsingArg(0)())
      assert_equal(true, g:NewLambdaUsingArg(1)())

      var res = map([1, 2, 3], (i: number, v: number) => i + v)
      assert_equal([1, 3, 5], res)

      # Lambda returning a dict
      var Lmb = () => ({key: 42})
      assert_equal({key: 42}, Lmb())

      var RefOne: func(number): string = (a: number): string => 'x'
      var RefTwo: func(number): any = (a: number): any => 'x'

      var Fx = (a) => ({k1: 0,
                         k2: 1})
      var Fy = (a) => [0,
                       1]
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["var Ref = (a)=>a + 1"], 'E1004:')
  v9.CheckDefAndScriptFailure(["var Ref = (a)=> a + 1"], 'E1004:')
  v9.CheckDefAndScriptFailure(["var Ref = (a) =>a + 1"],
      'E1004: White space required before and after ''=>'' at " =>a + 1"')

  v9.CheckDefAndScriptFailure(["var Ref: func(number): number = (a: number): string => 'x'"], 'E1012:')
  v9.CheckDefAndScriptFailure(["var Ref: func(number): string = (a: number): string => 99"], 'E1012:')

  v9.CheckDefAndScriptFailure(["filter([1, 2], (k,v) => 1)"], 'E1069:', 1)
  # error is in first line of the lambda
  v9.CheckDefAndScriptFailure(["var L = (a) -> a + b"], ['E1001:', 'E121:'], 1)

  assert_equal('xxxyyy', 'xxx'->((a, b) => a .. b)('yyy'))

  v9.CheckDefExecFailure(["var s = 'asdf'->((a) => a)('x')"],
        'E118: Too many arguments for function:')
  v9.CheckDefExecFailure(["var s = 'asdf'->((a) => a)('x', 'y')"],
        'E118: Too many arguments for function:')
  v9.CheckDefFailure(["echo 'asdf'->((a) => a)(x)"], 'E1001:', 1)

  v9.CheckDefAndScriptFailure(['var Fx = (a) => ({k1: 0', ' k2: 1})'], 'E722:', 2)
  v9.CheckDefAndScriptFailure(['var Fx = (a) => ({k1: 0,', ' k2 1})'], 'E720:', 2)

  v9.CheckDefAndScriptFailure(['var Fx = (a) => [0', ' 1]'], 'E696:', 2)
enddef

def Test_expr9_lambda_vim9script()
  var lines =<< trim END
      var v = 10->((a) =>
	    a
	      + 2
            )()
      assert_equal(12, v)
  END
  v9.CheckDefAndScriptSuccess(lines)

  # nested lambda with line breaks
  lines =<< trim END
      search('"', 'cW', 0, 0, () =>
	synstack('.', col('.'))
          ->mapnew((_, v) => synIDattr(v, 'name'))->len())
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9funcref()
  var lines =<< trim END
      def RetNumber(): number
        return 123
      enddef
      var FuncRef = RetNumber
      assert_equal(123, FuncRef())
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      vim9script
      func g:GlobalFunc()
        return 'global'
      endfunc
      func ScriptFunc()
        return 'script'
      endfunc
      def Test()
        var Ref = g:GlobalFunc
        assert_equal('global', Ref())
        Ref = g:GlobalFunc
        assert_equal('global', Ref())

        Ref = ScriptFunc
        assert_equal('script', Ref())
        Ref = ScriptFunc
        assert_equal('script', Ref())
      enddef
      Test()
  END
  v9.CheckScriptSuccess(lines)

  # using funcref in legacy script
  lines =<< trim END
      def s:Refme(): string
        return 'yes'
      enddef

      def TestFunc()
        var TheRef = s:Refme
        assert_equal('yes', TheRef())
      enddef

      call TestFunc()
  END
  v9.CheckScriptSuccess(lines)
enddef

let g:test_space_dict = {'': 'empty', ' ': 'space'}
let g:test_hash_dict = #{one: 1, two: 2}

def Test_expr9_dict()
  # dictionary
  var lines =<< trim END
      assert_equal(g:dict_empty, {})
      assert_equal(g:dict_empty, {  })
      assert_equal(g:dict_one, {['one']: 1})
      var key = 'one'
      var val = 1
      assert_equal(g:dict_one, {[key]: val})

      var numbers: dict<number> = {a: 1, b: 2, c: 3}
      numbers = {a: 1}
      numbers = {}

      var strings: dict<string> = {a: 'a', b: 'b', c: 'c'}
      strings = {a: 'x'}
      strings = {}

      var dash = {xx-x: 8}
      assert_equal({['xx-x']: 8}, dash)

      var dnr = {8: 8}
      assert_equal({['8']: 8}, dnr)

      var mixed: dict<any> = {a: 'a', b: 42}
      mixed = {a: 'x'}
      mixed = {a: 234}
      mixed = {}

      var dictlist: dict<list<string>> = {absent: [], present: ['hi']}
      dictlist = {absent: ['hi'], present: []}
      dictlist = {absent: [], present: []}

      var dictdict: dict<dict<string>> = {one: {a: 'text'}, two: {}}
      dictdict = {one: {}, two: {a: 'text'}}
      dictdict = {one: {}, two: {}}

      assert_equal({['']: 0}, {[matchstr('string', 'wont match')]: 0})

      assert_equal(g:test_space_dict, {['']: 'empty', [' ']: 'space'})
      assert_equal(g:test_hash_dict, {one: 1, two: 2})

      assert_equal({['a a']: 1, ['b/c']: 2}, {'a a': 1, "b/c": 2})

      var d = {a: () => 3, b: () => 7}
      assert_equal(3, d.a())
      assert_equal(7, d.b())

      var cd = { # comment
                key: 'val' # comment
               }

      # different types used for the key
      var dkeys = {['key']: 'string',
                   [12]: 'numberexpr',
                   34: 'number',
                   [true]: 'bool'}
      assert_equal('string', dkeys['key'])
      assert_equal('numberexpr', dkeys[12])
      assert_equal('number', dkeys[34])
      assert_equal('bool', dkeys[true])
      dkeys = {[1.2]: 'floatexpr', [3.4]: 'float'}
      assert_equal('floatexpr', dkeys[1.2])
      assert_equal('float', dkeys[3.4])

      # automatic conversion from number to string
      var n = 123
      var dictnr = {[n]: 1}

      # comment to start fold is OK
      var x1: number #{{ fold
      var x2 = 9 #{{ fold

      var ds = {k: null_string}
      assert_equal('dict<string>', typename(ds))
      var dl = {a: null_list}
      assert_equal('dict<list<any>>', typename(dl))
  END
  v9.CheckDefAndScriptSuccess(lines)

  # legacy syntax doesn't work
  v9.CheckDefAndScriptFailure(["var x = #{key: 8}"], 'E1170:', 1)
  v9.CheckDefAndScriptFailure(["var x = 'a' #{a: 1}"], 'E1170:', 1)
  v9.CheckDefAndScriptFailure(["var x = 'a' .. #{a: 1}"], 'E1170:', 1)
  v9.CheckDefAndScriptFailure(["var x = true ? #{a: 1}"], 'E1170:', 1)

  v9.CheckDefAndScriptFailure(["var x = 'a'", " #{a: 1}"], 'E1170:', 1)

  v9.CheckDefAndScriptFailure(["var x = {a:8}"], 'E1069:', 1)
  v9.CheckDefAndScriptFailure(["var x = {a : 8}"], 'E1068:', 1)
  v9.CheckDefAndScriptFailure(["var x = {a :8}"], 'E1068:', 1)
  v9.CheckDefAndScriptFailure(["var x = {a: 8 , b: 9}"], 'E1068:', 1)
  v9.CheckDefAndScriptFailure(["var x = {a: 1,b: 2}"], 'E1069:', 1)

  v9.CheckDefAndScriptFailure(["var x = {xxx}"], 'E720:', 1)
  v9.CheckDefAndScriptFailure(["var x = {xxx: 1", "var y = 2"], 'E722:', 2)
  v9.CheckDefFailure(["var x = {xxx: 1,"], 'E723:', 2)
  v9.CheckScriptFailure(['vim9script', "var x = {xxx: 1,"], 'E723:', 2)
  v9.CheckDefAndScriptFailure(["var x = {['a']: xxx}"], ['E1001:', 'E121:'], 1)
  v9.CheckDefAndScriptFailure(["var x = {a: 1, a: 2}"], 'E721:', 1)
  g:key = 'x'
  v9.CheckDefExecAndScriptFailure(["var x = {[g:key]: 'text', [g:key]: 'text'}"], 'E721:', 1)
  unlet g:key
  v9.CheckDefExecAndScriptFailure(["var x = {[notexists]: 'text'}"], ['E1001:', 'E121: Undefined variable: notexists'], 1)
  v9.CheckDefExecAndScriptFailure(["var x = g:anint.member"], ['E715:', 'E488:'], 1)
  v9.CheckDefExecAndScriptFailure(["var x = g:dict_empty.member"], 'E716:', 1)

  v9.CheckDefExecAndScriptFailure(['var x: dict<number> = {a: 234, b: "1"}'], 'E1012:', 1)
  v9.CheckDefExecAndScriptFailure(['var x: dict<number> = {a: "x", b: 134}'], 'E1012:', 1)
  v9.CheckDefExecAndScriptFailure(['var x: dict<string> = {a: 234, b: "1"}'], 'E1012:', 1)
  v9.CheckDefExecAndScriptFailure(['var x: dict<string> = {a: "x", b: 134}'], 'E1012:', 1)

  # invalid types for the key
  v9.CheckDefAndScriptFailure(["var x = {[[1, 2]]: 0}"], ['E1105:', 'E730:'], 1)

  v9.CheckDefFailure(['var x = ({'], 'E723:', 2)
  v9.CheckScriptFailure(['vim9script', 'var x = ({'], 'E723:', 2)
  v9.CheckDefExecAndScriptFailure(['{}[getftype("file")]'], 'E716: Key not present in Dictionary: ""', 1)
enddef

def Test_expr9_dict_vim9script()
  var lines =<< trim END
      var d = {
		['one']:
		   1,
		['two']: 2,
		   }
      assert_equal({one: 1, two: 2}, d)

      d = {  # comment
		['one']:
                # comment

		   1,
                # comment
                # comment
		['two']: 2,
		   }
      assert_equal({one: 1, two: 2}, d)

      var dd = {k: 123->len()}
      assert_equal(3, dd.k)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var d = { ["one"]: "one", ["two"]: "two", }
      assert_equal({one: 'one', two: 'two'}, d)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var d = {one: 1,
		two: 2,
	       }
      assert_equal({one: 1, two: 2}, d)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var d = {one:1, two: 2}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1069:', 1)

  lines =<< trim END
      var d = {one: 1,two: 2}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1069:', 1)

  lines =<< trim END
      var d = {one : 1}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1068:', 1)

  lines =<< trim END
      var d = {one:1}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1069:', 1)

  lines =<< trim END
      var d = {one: 1 , two: 2}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1068:', 1)

  lines =<< trim END
    var l: dict<number> = {a: 234, b: 'x'}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:', 1)

  lines =<< trim END
    var l: dict<number> = {a: 'x', b: 234}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:', 1)

  lines =<< trim END
    var l: dict<string> = {a: 'x', b: 234}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:', 1)

  lines =<< trim END
    var l: dict<string> = {a: 234, b: 'x'}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:', 1)

  lines =<< trim END
    var d = {['a']: 234, ['b': 'x'}
  END
  v9.CheckDefAndScriptFailure(lines, 'E1139:', 1)

  lines =<< trim END
    def Func()
      var d = {['a']: 234, ['b': 'x'}
    enddef
    defcompile
  END
  v9.CheckDefAndScriptFailure(lines, 'E1139:', 0)

  lines =<< trim END
    var d = {'a':
  END
  v9.CheckDefFailure(lines, 'E723:', 2)
  v9.CheckScriptFailure(['vim9script'] + lines, 'E15:', 2)

  lines =<< trim END
    def Func()
      var d = {'a':
    enddef
    defcompile
  END
  v9.CheckDefAndScriptFailure(lines, 'E723:', 0)

  lines =<< trim END
      def Failing()
        job_stop()
      enddef
      var dict = {name: Failing}
  END
  if has('channel')
    v9.CheckDefAndScriptFailure(lines, 'E119:', 0)
  else
    v9.CheckDefAndScriptFailure(lines, 'E117:', 0)
  endif

  lines =<< trim END
      vim9script
      var x = 99
      assert_equal({x: 99}, s:)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_expr9_dict_in_block()
  var lines =<< trim END
      vim9script
      command MyCommand {
          echo {
              k: 0, }
      }
      MyCommand

      command YourCommand {
         g:global = {
           key: 'value' }
         }
      YourCommand
      assert_equal({key: 'value'}, g:global)
      unlet g:global
  END
  v9.CheckScriptSuccess(lines)

  delcommand MyCommand
  delcommand YourCommand
enddef

def Test_expr9_call_2bool()
  var lines =<< trim END
      vim9script

      def BrokenCall(nr: number, mode: bool, use: string): void
        assert_equal(3, nr)
        assert_equal(false, mode)
        assert_equal('ab', use)
      enddef

      def TestBrokenCall(): void
        BrokenCall(3, 0, 'ab')
      enddef

      TestBrokenCall()
  END
  v9.CheckScriptSuccess(lines)
enddef

let g:oneString = 'one'

def Test_expr_member()
  var lines =<< trim END
      assert_equal(1, g:dict_one.one)
      var d: dict<number> = g:dict_one
      assert_equal(1, d['one'])
      assert_equal(1, d[
                      'one'
                      ])
      assert_equal(1, d
            .one)
      d = {1: 1, _: 2}
      assert_equal(1, d
            .1)
      assert_equal(2, d
            ._)

      # getting the one member should clear the dict after getting the item
      assert_equal('one', {one: 'one'}.one)
      assert_equal('one', {one: 'one'}[g:oneString])
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["var x = g:dict_one.#$!"], ['E1002:', 'E15:'], 1)
  v9.CheckDefExecAndScriptFailure(["var d: dict<any>", "echo d['a']"], 'E716:', 2)
  v9.CheckDefExecAndScriptFailure(["var d: dict<number>", "d = g:list_empty"], 'E1012: Type mismatch; expected dict<number> but got list<any>', 2)
enddef

def Test_expr9_any_index_slice()
  var lines =<< trim END
    # getting the one member should clear the list only after getting the item
    assert_equal('bbb', ['aaa', 'bbb', 'ccc'][1])

    # string is permissive, index out of range accepted
    g:teststring = 'abcdef'
    assert_equal('b', g:teststring[1])
    assert_equal('f', g:teststring[-1])
    assert_equal('', g:teststring[99])

    assert_equal('b', g:teststring[1 : 1])
    assert_equal('bcdef', g:teststring[1 :])
    assert_equal('abcd', g:teststring[: 3])
    assert_equal('cdef', g:teststring[-4 :])
    assert_equal('abcdef', g:teststring[-9 :])
    assert_equal('abcd', g:teststring[: -3])
    assert_equal('', g:teststring[: -9])

    # composing characters are included
    g:teststring = 'àéû'
    assert_equal('à', g:teststring[0])
    assert_equal('é', g:teststring[1])
    assert_equal('û', g:teststring[2])
    assert_equal('', g:teststring[3])
    assert_equal('', g:teststring[4])

    assert_equal('û', g:teststring[-1])
    assert_equal('é', g:teststring[-2])
    assert_equal('à', g:teststring[-3])
    assert_equal('', g:teststring[-4])
    assert_equal('', g:teststring[-5])

    assert_equal('à', g:teststring[0 : 0])
    assert_equal('é', g:teststring[1 : 1])
    assert_equal('àé', g:teststring[0 : 1])
    assert_equal('àéû', g:teststring[0 : -1])
    assert_equal('àé', g:teststring[0 : -2])
    assert_equal('à', g:teststring[0 : -3])
    assert_equal('', g:teststring[0 : -4])
    assert_equal('', g:teststring[0 : -5])
    assert_equal('àéû', g:teststring[ : ])
    assert_equal('àéû', g:teststring[0 : ])
    assert_equal('éû', g:teststring[1 : ])
    assert_equal('û', g:teststring[2 : ])
    assert_equal('', g:teststring[3 : ])
    assert_equal('', g:teststring[4 : ])

    # blob index cannot be out of range
    g:testblob = 0z01ab
    assert_equal(0x01, g:testblob[0])
    assert_equal(0xab, g:testblob[1])
    assert_equal(0xab, g:testblob[-1])
    assert_equal(0x01, g:testblob[-2])

    # blob slice accepts out of range
    assert_equal(0z01ab, g:testblob[0 : 1])
    assert_equal(0z01, g:testblob[0 : 0])
    assert_equal(0z01, g:testblob[-2 : -2])
    assert_equal(0zab, g:testblob[1 : 1])
    assert_equal(0zab, g:testblob[-1 : -1])
    assert_equal(0z, g:testblob[2 : 2])
    assert_equal(0z, g:testblob[0 : -3])

    # list index cannot be out of range
    g:testlist = [0, 1, 2, 3]
    assert_equal(0, g:testlist[0])
    assert_equal(1, g:testlist[1])
    assert_equal(3, g:testlist[3])
    assert_equal(3, g:testlist[-1])
    assert_equal(0, g:testlist[-4])
    assert_equal(1, g:testlist[g:theone])

    # list slice accepts out of range
    assert_equal([0], g:testlist[0 : 0])
    assert_equal([3], g:testlist[3 : 3])
    assert_equal([0, 1], g:testlist[0 : 1])
    assert_equal([0, 1, 2, 3], g:testlist[0 : 3])
    assert_equal([0, 1, 2, 3], g:testlist[0 : 9])
    assert_equal([], g:testlist[-1 : 1])
    assert_equal([1], g:testlist[-3 : 1])
    assert_equal([0, 1], g:testlist[-4 : 1])
    assert_equal([0, 1], g:testlist[-9 : 1])
    assert_equal([1, 2, 3], g:testlist[1 : -1])
    assert_equal([1], g:testlist[1 : -3])
    assert_equal([], g:testlist[1 : -4])
    assert_equal([], g:testlist[1 : -9])

    g:testdict = {a: 1, b: 2}
    assert_equal(1, g:testdict['a'])
    assert_equal(2, g:testdict['b'])
  END

  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      vim9script

      def PosIdx(s: string): string
        return s[1]
      enddef
      def NegIdx(s: string): string
        return s[-1]
      enddef

      set enc=latin1
      assert_equal("\xe4", PosIdx("a\xe4\xe5"))
      assert_equal("\xe5", NegIdx("a\xe4\xe5"))
      set enc=utf-8
  END
  v9.CheckScriptSuccess(lines)

  v9.CheckDefExecAndScriptFailure(['echo g:testblob[2]'], 'E979:', 1)
  v9.CheckDefExecAndScriptFailure(['echo g:testblob[-3]'], 'E979:', 1)

  v9.CheckDefExecAndScriptFailure(['echo g:testlist[4]'], 'E684: List index out of range: 4', 1)
  v9.CheckDefExecAndScriptFailure(['echo g:testlist[-5]'], 'E684:', 1)

  v9.CheckDefExecAndScriptFailure(['echo g:testdict["a" : "b"]'], 'E719:', 1)
  v9.CheckDefExecAndScriptFailure(['echo g:testdict[1]'], 'E716:', 1)

  unlet g:teststring
  unlet g:testblob
  unlet g:testlist
enddef

def s:GetList(): list<string>
  return ['a', 'b', 'z']
enddef

def Test_slice_const_list()
  const list = GetList()
  final sliced = list[0 : 1]
  # OK to change the list after slicing, it is a copy now
  add(sliced, 'Z')
  assert_equal(['a', 'b', 'Z'], sliced)
enddef

def Test_expr9_const_any_index_slice()
  var lines =<< trim END
      vim9script

      export def V(): dict<any>
        return {a: [1, 43], b: 0}
      enddef
  END
  writefile(lines, 'XexportDict.vim', 'D')

  lines =<< trim END
      vim9script

      import './XexportDict.vim' as x

      def Test()
        const v = x.V()
        assert_equal(43, v.a[1])
      enddef
      Test()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_expr_member_vim9script()
  var lines =<< trim END
      var d = {one:
		'one',
		two: 'two',
		1: 1,
		_: 2}
      assert_equal('one', d.one)
      assert_equal('one', d
                            .one)
      assert_equal(1, d
                            .1)
      assert_equal(2, d
                            ._)
      assert_equal('one', d[
			    'one'
			    ])
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var l = [1,
		  2,
		  3, 4
		  ]
      assert_equal(2, l[
			    1
			    ])
      assert_equal([2, 3], l[1 : 2])
      assert_equal([1, 2, 3], l[
				:
				2
				])
      assert_equal([3, 4], l[
				2
				:
				])
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def SetSomeVar()
  b:someVar = &fdm
enddef

def Test_expr9_option()
  var lines =<< trim END
      # option
      set ts=11
      assert_equal(11, &ts)
      &ts = 9
      assert_equal(9, &ts)
      set ts=8
      set grepprg=some\ text
      assert_equal('some text', &grepprg)
      &grepprg = test_null_string()
      assert_equal('', &grepprg)
      set grepprg&

      # check matching type
      var bval: bool = &tgc
      var nval: number = &ts
      var sval: string = &path

      # check v_lock is cleared (requires using valgrind, doesn't always show)
      g:SetSomeVar()
      b:someVar = 0
      unlet b:someVar
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_environment()
  var lines =<< trim END
      # environment variable
      assert_equal('testvar', $TESTVAR)
      assert_equal('', $ASDF_ASD_XXX)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["var x = $$$"], ['E1002:', 'E15:'], 1)
  v9.CheckDefAndScriptFailure(["$"], ['E1002:', 'E15:'], 1)
enddef

def Test_expr9_register()
  var lines =<< trim END
      @a = 'register a'
      assert_equal('register a', @a)

      var fname = expand('%')
      assert_equal(fname, @%)

      feedkeys(":echo 'some'\<CR>", "xt")
      assert_equal("echo 'some'", @:)

      normal axyz
      assert_equal("xyz", @.)

      @/ = 'slash'
      assert_equal('slash', @/)

      @= = 'equal'
      assert_equal('equal', @=)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["@. = 'yes'"], ['E354:', 'E488:'], 1)
enddef

" This is slow when run under valgrind.
def Test_expr9_namespace()
  var lines =<< trim END
      g:some_var = 'some'
      assert_equal('some', get(g:, 'some_var'))
      assert_equal('some', get(g:, 'some_var', 'xxx'))
      assert_equal('xxx', get(g:, 'no_var', 'xxx'))
      unlet g:some_var

      b:some_var = 'some'
      assert_equal('some', get(b:, 'some_var'))
      assert_equal('some', get(b:, 'some_var', 'xxx'))
      assert_equal('xxx', get(b:, 'no_var', 'xxx'))
      unlet b:some_var

      w:some_var = 'some'
      assert_equal('some', get(w:, 'some_var'))
      assert_equal('some', get(w:, 'some_var', 'xxx'))
      assert_equal('xxx', get(w:, 'no_var', 'xxx'))
      unlet w:some_var

      t:some_var = 'some'
      assert_equal('some', get(t:, 'some_var'))
      assert_equal('some', get(t:, 'some_var', 'xxx'))
      assert_equal('xxx', get(t:, 'no_var', 'xxx'))
      unlet t:some_var
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_namespace_loop_def()
  var lines =<< trim END
      # check using g: in a for loop more than DO_NOT_FREE_CNT times
      var exists = 0
      var exists_not = 0
      for i in range(100000)
        if has_key(g:, 'does-not-exist')
          exists += 1
        else
          exists_not += 1
        endif
      endfor
      assert_equal(0, exists)
      assert_equal(100000, exists_not)
  END
  v9.CheckDefSuccess(lines)
enddef

" NOTE: this is known to be slow.  To skip use:
"   :let $TEST_SKIP_PAT = 'Test_expr9_namespace_loop_script'
def Test_expr9_namespace_loop_script()
  var lines =<< trim END
      vim9script
      # check using g: in a for loop more than DO_NOT_FREE_CNT times
      var exists = 0
      var exists_not = 0
      for i in range(100000)
        if has_key(g:, 'does-not-exist')
          exists += 1
        else
          exists_not += 1
        endif
      endfor
      assert_equal(0, exists)
      assert_equal(100000, exists_not)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_expr9_parens()
  # (expr)
  var lines =<< trim END
      assert_equal(4, (6 * 4) / 6)
      assert_equal(0, 6 * ( 4 / 6 ))

      assert_equal(6, +6)
      assert_equal(-6, -6)
      assert_equal(false, !-3)
      assert_equal(true, !+0)

      assert_equal(7, 5 + (
                    2))
      assert_equal(7, 5 + (
                    2
                    ))
      assert_equal(7, 5 + ( # comment
                    2))
      assert_equal(7, 5 + ( # comment
                    # comment
                    2))

      var s = (
		'one'
		..
		'two'
		)
      assert_equal('onetwo', s)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(['echo ('], ['E1097: Line incomplete', 'E15: Invalid expression: "("'])
  v9.CheckDefAndScriptFailure(['echo (123]'], "E110: Missing ')'", 1)

  # this uses up the ppconst stack
  lines =<< eval trim END
    vim9script
    def F()
      g:result = 1 + {repeat('(1 + ', 51)}1{repeat(')', 51)}
    enddef
    F()
  END
  v9.CheckScriptSuccess(lines)
  assert_equal(g:result, 53)
  unlet g:result
enddef

def Test_expr9_negate_add()
  var lines =<< trim END
      assert_equal(-99, -99)
      assert_equal(-99, - 99)
      assert_equal(99, +99)

      var nr = 88
      assert_equal(-88, -nr)
      assert_equal(-88, - nr)
      assert_equal(88, + nr)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
    var n = 12
    echo ++n
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:')
  lines =<< trim END
    var n = 12
    echo --n
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:')
  lines =<< trim END
    var n = 12
    echo +-n
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:')
  lines =<< trim END
    var n = 12
    echo -+n
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:')
  lines =<< trim END
    var n = 12
    echo - -n
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:')
  lines =<< trim END
    var n = 12
    echo + +n
  END
  v9.CheckDefAndScriptFailure(lines, 'E15:')
enddef

def LegacyReturn(): string
  legacy return #{key: 'ok'}.key
enddef

def Test_expr9_legacy_script()
  var lines =<< trim END
      let s:legacy = 'legacy'
      def GetLocal(): string
        return legacy
      enddef
      def GetLocalPrefix(): string
        return s:legacy
      enddef
      call assert_equal('legacy', GetLocal())
      call assert_equal('legacy', GetLocalPrefix())
  END
  v9.CheckScriptSuccess(lines)

  assert_equal('ok', g:LegacyReturn())

  lines =<< trim END
      vim9script
      def GetNumber(): number
          legacy return notexists
      enddef
      echo GetNumber()
  END
  v9.CheckScriptFailure(lines, 'E121: Undefined variable: notexists')

  lines =<< trim END
      vim9script
      def GetNumber(): number
          legacy return range(3)->map('v:val + 1')
      enddef
      echo GetNumber()
  END
  v9.CheckScriptFailure(lines, 'E1012: Type mismatch; expected number but got list<number>')
enddef

def Echo(arg: any): string
  return arg
enddef

def s:Echo4Arg(arg: any): string
  return arg
enddef

def Test_expr9_call()
  var lines =<< trim END
      assert_equal('yes', 'yes'->g:Echo())
      assert_equal(true, !range(5)->empty())
      assert_equal([0, 1, 2], 3->range())
  END
  v9.CheckDefAndScriptSuccess(lines)

  assert_equal('yes', 'yes'
                        ->s:Echo4Arg())

  v9.CheckDefAndScriptFailure(["var x = 'yes'->g:Echo"], 'E107:', 1)
  v9.CheckDefAndScriptFailure([
       "var x = substitute ('x', 'x', 'x', 'x')"
       ], ['E1001:', 'E121:'], 1)
  v9.CheckDefAndScriptFailure(["var Ref = function('len' [1, 2])"], ['E1123:', 'E116:'], 1)
  v9.CheckDefAndScriptFailure(["echo match(['foo'] , 'foo')"], 'E1068:', 1)
enddef

def g:ExistingGlobal(): string
  return 'existing'
enddef

def Test_expr9_call_global()
  assert_equal('existing', g:ExistingGlobal())

  def g:DefinedLater(): string
    return 'later'
  enddef
  assert_equal('later', g:DefinedLater())

  var lines =<< trim END
    echo ExistingGlobal()
  END
  v9.CheckDefAndScriptFailure(lines, 'E117: Unknown function: ExistingGlobal')
enddef

def Test_expr9_autoload_var()
  var auto_lines =<< trim END
      let autofile#var = 'found'
  END
  mkdir('Xruntime/autoload', 'pR')
  writefile(auto_lines, 'Xruntime/autoload/autofile.vim')
  var save_rtp = &rtp
  &rtp = getcwd() .. '/Xruntime,' .. &rtp

  var lines =<< trim END
      assert_equal('found', autofile#var)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      echo autofile#other
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E121: Undefined variable: autofile#other')

  &rtp = save_rtp
enddef

def Test_expr9_call_autoload()
  var auto_lines =<< trim END
      def g:some#func(): string
	return 'found'
      enddef
  END
  mkdir('Xruntime/autoload', 'pR')
  writefile(auto_lines, 'Xruntime/autoload/some.vim')
  var save_rtp = &rtp
  &rtp = getcwd() .. '/Xruntime,' .. &rtp
  assert_equal('found', g:some#func())
  assert_equal('found', some#func())

  &rtp = save_rtp
enddef

def Test_expr9_method_call()
  var lines =<< trim END
      new
      setline(1, ['first', 'last'])
      'second'->append(1)
      "third"->append(2)
      $"fourth"->append(3)
      $'fifth'->append(4)
      assert_equal(['first', 'second', 'third', 'fourth', 'fifth', 'last'], getline(1, '$'))
      bwipe!

      var bufnr = bufnr()
      var loclist = [{bufnr: bufnr, lnum: 42, col: 17, text: 'wrong'}]
      loclist->setloclist(0)
      assert_equal([{bufnr: bufnr,
                    lnum: 42,
                    end_lnum: 0,
                    col: 17,
                    end_col: 0,
                    text: 'wrong',
                    pattern: '',
                    valid: 1,
                    vcol: 0,
                    nr: 0,
                    type: '',
                    module: ''}
                    ], getloclist(0))

      var result: bool = get({n: 0}, 'n', 0)
      assert_equal(false, result)

      assert_equal('+string+', 'string'->((s) => '+' .. s .. '+')())
      assert_equal('-text-', 'text'->((s, c) => c .. s .. c)('-'))

      var Join = (l) => join(l, 'x')
      assert_equal('axb', ['a', 'b']->(Join)())

      var sorted = [3, 1, 2]
                    -> sort()
      assert_equal([1, 2, 3], sorted)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      def SetNumber(n: number)
        g:number = n
      enddef
      const Setit = SetNumber
      len('text')->Setit()
      assert_equal(4, g:number)

      const SetFuncref = funcref(SetNumber)
      len('longer')->SetFuncref()
      assert_equal(6, g:number)

      const SetList = [SetNumber, SetFuncref]
      len('xx')->SetList[0]()
      assert_equal(2, g:number)
      len('xxx')->SetList[1]()
      assert_equal(3, g:number)

      const SetDict = {key: SetNumber}
      len('xxxx')->SetDict['key']()
      assert_equal(4, g:number)
      len('xxxxx')->SetDict.key()
      assert_equal(5, g:number)

      unlet g:number
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
    def RetVoid()
    enddef
    RetVoid()->byteidx(3)
  END
  v9.CheckDefExecFailure(lines, 'E1013:')

  lines =<< trim END
      const SetList = [function('len')]
      echo 'xx'->SetList[x]()
  END
  v9.CheckDefFailure(lines, 'E1001: Variable not found: x')

  lines =<< trim END
      const SetList = [function('len')]
      echo 'xx'->SetList[0]x()
  END
  v9.CheckDefFailure(lines, 'E15: Invalid expression: "->SetList[0]x()"')
enddef

def Test_expr9_method_call_linebreak()
  # this was giving an error when skipping over the expression
  var lines =<< trim END
      vim9script
      def Test()
        var a: dict<any> = {b: {}}
        a.b->extend({f1: 1,
                         f2: 2})
        echo a
      enddef
      defcompile
      assert_equal('', v:errmsg)
  END
  v9.CheckScriptSuccess(lines)

  # this was skipping over the expression without an error
  lines =<< trim END
      vim9script
      def Test(s: string): string
        return substitute(s, 'A', '\=toupper("x")', 'g')
                          ->tolower()
      enddef
      assert_equal('xbcd', Test('ABCD'))
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_expr9_method_call_import()
  var lines =<< trim END
      vim9script
      export def Square(items: list<number>): list<number>
          return map(items, (_, i) => i * i)
      enddef
  END
  call writefile(lines, 'Xsquare.vim', 'D')

  lines =<< trim END
      vim9script
      import './Xsquare.vim'

      def Test(): list<number>
        return range(5)
            ->Xsquare.Square()
            ->map((_, i) => i * 10)
      enddef

      assert_equal([0, 10, 40, 90, 160], Test())
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      import './Xsquare.vim'

      echo range(5)->Xsquare.NoSuchFunc()
  END
  v9.CheckScriptFailure(lines, 'E1048: Item not found in script: NoSuchFunc')
enddef


def Test_expr9_not()
  var lines =<< trim END
      assert_equal(true, !'')
      assert_equal(true, ![])
      assert_equal(false, !'asdf')
      assert_equal(false, ![2])
      assert_equal(true, !!'asdf')
      assert_equal(true, !![2])

      assert_equal(true, ! false)
      assert_equal(true, !! true)
      assert_equal(true, ! ! true)
      assert_equal(true, !!! false)
      assert_equal(true, ! ! ! false)

      g:true = true
      g:false = false
      assert_equal(true, ! g:false)
      assert_equal(true, !! g:true)
      assert_equal(true, ! ! g:true)
      assert_equal(true, !!! g:false)
      assert_equal(true, ! ! ! g:false)
      unlet g:true
      unlet g:false

      assert_equal(true, !test_null_partial())
      assert_equal(false, !() => 'yes')

      assert_equal(true, !test_null_dict())
      assert_equal(true, !{})
      assert_equal(false, !{yes: 'no'})

      if has('channel')
	assert_equal(true, !test_null_job())
	assert_equal(true, !test_null_channel())
      endif

      assert_equal(true, !test_null_blob())
      assert_equal(true, !0z)
      assert_equal(false, !0z01)

      assert_equal(true, !test_void())
      assert_equal(true, !test_unknown())

      assert_equal(false, ![1, 2, 3]->reverse())
      assert_equal(true, ![]->reverse())
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

let g:anumber = 42

def Test_expr9_negate()
  var lines =<< trim END
      var nr = 1
      assert_equal(-1, -nr)
      assert_equal(-42, -g:anumber)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

func Test_expr9_fails()
  call v9.CheckDefFailure(["var x = (12"], "E1097:", 3)
  call v9.CheckScriptFailure(['vim9script', "var x = (12"], 'E110:', 2)

  call v9.CheckDefAndScriptFailure(["var x = -'xx'"], "E1030:", 1)
  call v9.CheckDefAndScriptFailure(["var x = +'xx'"], "E1030:", 1)
  call v9.CheckDefAndScriptFailure(["var x = -0z12"], "E974:", 1)
  call v9.CheckDefExecAndScriptFailure(["var x = -[8]"], ["E1012:", 'E745:'], 1)
  call v9.CheckDefExecAndScriptFailure(["var x = -{a: 1}"], ["E1012:", 'E728:'], 1)

  call v9.CheckDefAndScriptFailure(["var x = @"], "E1002:", 1)
  call v9.CheckDefAndScriptFailure(["var x = @<"], "E354:", 1)

  call v9.CheckDefFailure(["var x = [1, 2"], "E697:", 2)
  call v9.CheckScriptFailure(['vim9script', "var x = [1, 2"], 'E696:', 2)

  call v9.CheckDefAndScriptFailure(["var x = [notfound]"], ["E1001:", 'E121:'], 1)

  call v9.CheckDefAndScriptFailure(["var X = () => 123)"], 'E488:', 1)
  call v9.CheckDefAndScriptFailure(["var x = 123->((x) => x + 5)"], "E107:", 1)

  call v9.CheckDefAndScriptFailure(["var x = &notexist"], 'E113:', 1)
  call v9.CheckDefAndScriptFailure(["&grepprg = [343]"], ['E1012:', 'E730:'], 1)

  call v9.CheckDefExecAndScriptFailure(["echo s:doesnt_exist"], ['E121:', 'E1268:'], 1)
  call v9.CheckDefExecAndScriptFailure(["echo g:doesnt_exist"], 'E121:', 1)

  call v9.CheckDefAndScriptFailure(["echo a:somevar"], ['E1075:', 'E121:'], 1)
  call v9.CheckDefAndScriptFailure(["echo l:somevar"], ['E1075:', 'E121:'], 1)
  call v9.CheckDefAndScriptFailure(["echo x:somevar"], ['E1075:', 'E121:'], 1)

  call v9.CheckDefExecAndScriptFailure(["var x = +g:astring"], ['E1012:', 'E1030:'], 1)
  call v9.CheckDefExecAndScriptFailure(["var x = +g:ablob"], ['E1012:', 'E974:'], 1)
  call v9.CheckDefExecAndScriptFailure(["var x = +g:alist"], ['E1012:', 'E745:'], 1)
  call v9.CheckDefExecAndScriptFailure(["var x = +g:adict"], ['E1012:', 'E728:'], 1)

  call v9.CheckDefAndScriptFailure(["var x = ''", "var y = x.memb"], ['E1229: Expected dictionary for using key "memb", but got string', 'E488:'], 2)

  call v9.CheckDefAndScriptFailure(["'yes'->", "Echo()"], ['E488: Trailing characters: ->', 'E260: Missing name after ->'], 1)

  call v9.CheckDefExecFailure(["[1, 2->len()"], 'E697:', 2)
  call v9.CheckScriptFailure(['vim9script', "[1, 2->len()"], 'E696:', 2)

  call v9.CheckDefFailure(["{a: 1->len()"], 'E723:', 2)
  call v9.CheckScriptFailure(['vim9script', "{a: 1->len()"], 'E722:', 2)

  call v9.CheckDefExecFailure(["{['a']: 1->len()"], 'E723:', 2)
  call v9.CheckScriptFailure(['vim9script', "{['a']: 1->len()"], 'E722:', 2)

  call v9.CheckDefFailure(['echo #{}'], 'E1170:')
endfunc

let g:Funcrefs = [function('add')]

func CallMe(arg)
  return a:arg
endfunc

func CallMe2(one, two)
  return a:one .. a:two
endfunc

def Test_expr9_trailing()
  var lines =<< trim END
      # user function call
      assert_equal(123, g:CallMe(123))
      assert_equal(123, g:CallMe(  123))
      assert_equal(123, g:CallMe(123  ))
      assert_equal('yesno', g:CallMe2('yes', 'no'))
      assert_equal('yesno', g:CallMe2( 'yes', 'no' ))
      assert_equal('nothing', g:CallMe('nothing'))

      # partial call
      var Part = function('g:CallMe')
      assert_equal('yes', Part('yes'))

      # funcref call, using list index
      var l = []
      g:Funcrefs[0](l, 2)
      assert_equal([2], l)

      # method call
      l = [2, 5, 6]
      l->map((k, v) => k + v)
      assert_equal([2, 6, 8], l)

      # lambda method call
      l = [2, 5]
      l->((ll) => add(ll, 8))()
      assert_equal([2, 5, 8], l)

      # dict member
      var d = {key: 123}
      assert_equal(123, d.key)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_string_subscript()
  var lines =<< trim END
    var text = 'abcdef'
    assert_equal('f', text[-1])
    assert_equal('a', text[0])
    assert_equal('e', text[4])
    assert_equal('f', text[5])
    assert_equal('', text[6])

    text = 'ábçdë'
    assert_equal('ë', text[-1])
    assert_equal('d', text[-2])
    assert_equal('ç', text[-3])
    assert_equal('b', text[-4])
    assert_equal('á', text[-5])
    assert_equal('', text[-6])

    text = 'ábçdëf'
    assert_equal('', text[-999])
    assert_equal('f', text[-1])
    assert_equal('á', text[0])
    assert_equal('b', text[1])
    assert_equal('ç', text[2])
    assert_equal('d', text[3])
    assert_equal('ë', text[4])
    assert_equal('f', text[5])
    assert_equal('', text[6])
    assert_equal('', text[999])

    assert_equal('ábçdëf', text[0 : -1])
    assert_equal('ábçdëf', text[0 : -1])
    assert_equal('ábçdëf', text[0 : -1])
    assert_equal('ábçdëf', text[0 : -1])
    assert_equal('ábçdëf', text[0
                  : -1])
    assert_equal('ábçdëf', text[0 :
                  -1])
    assert_equal('ábçdëf', text[0 : -1
                  ])
    assert_equal('bçdëf', text[1 : -1])
    assert_equal('çdëf', text[2 : -1])
    assert_equal('dëf', text[3 : -1])
    assert_equal('ëf', text[4 : -1])
    assert_equal('f', text[5 : -1])
    assert_equal('', text[6 : -1])
    assert_equal('', text[999 : -1])

    assert_equal('ábçd', text[: 3])
    assert_equal('bçdëf', text[1 :])
    assert_equal('ábçdëf', text[:])

    assert_equal('a', g:astring[0])
    assert_equal('sd', g:astring[1 : 2])
    assert_equal('asdf', g:astring[:])
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var d = 'asdf'[1 :
  END
  v9.CheckDefFailure(lines, 'E1097:', 3)
  v9.CheckScriptFailure(['vim9script'] + lines, 'E15:', 2)

  lines =<< trim END
      var d = 'asdf'[1 : xxx]
  END
  v9.CheckDefAndScriptFailure(lines, ['E1001:', 'E121:'], 1)

  lines =<< trim END
      var d = 'asdf'[1 : 2
  END
  v9.CheckDefFailure(lines, 'E1097:', 3)
  v9.CheckScriptFailure(['vim9script'] + lines, 'E111:', 2)

  lines =<< trim END
      var d = 'asdf'[1 : 2
      echo d
  END
  v9.CheckDefAndScriptFailure(lines, 'E111:', 2)

  lines =<< trim END
      var d = 'asdf'['1']
      echo d
  END
  v9.CheckDefAndScriptFailure(lines, ['E1012: Type mismatch; expected number but got string', 'E1030: Using a String as a Number: "1"'], 1)

  lines =<< trim END
      var d = 'asdf'['1' : 2]
      echo d
  END
  v9.CheckDefAndScriptFailure(lines, ['E1012: Type mismatch; expected number but got string', 'E1030: Using a String as a Number: "1"'], 1)

  lines =<< trim END
      var d = 'asdf'[1 : '2']
      echo d
  END
  v9.CheckDefAndScriptFailure(lines, ['E1012: Type mismatch; expected number but got string', 'E1030: Using a String as a Number: "2"'], 1)
enddef

def Test_expr9_list_subscript()
  var lines =<< trim END
      var list = [0, 1, 2, 3, 4]
      assert_equal(0, list[0])
      assert_equal(4, list[4])
      assert_equal(4, list[-1])
      assert_equal(0, list[-5])

      assert_equal([0, 1, 2, 3, 4], list[0 : 4])
      assert_equal([0, 1, 2, 3, 4], list[:])
      assert_equal([1, 2, 3, 4], list[1 :])
      assert_equal([2, 3, 4], list[2 : -1])
      assert_equal([4], list[4 : -1])
      assert_equal([], list[5 : -1])
      assert_equal([], list[999 : -1])
      assert_equal([1, 2, 3, 4], list[g:theone : g:thefour])

      assert_equal([0, 1, 2, 3], list[0 : 3])
      assert_equal([0], list[0 : 0])
      assert_equal([0, 1, 2, 3, 4], list[0 : -1])
      assert_equal([0, 1, 2], list[0 : -3])
      assert_equal([0], list[0 : -5])
      assert_equal([], list[0 : -6])
      assert_equal([], list[0 : -99])

      assert_equal(2, g:alist[0])
      assert_equal([2, 3, 4], g:alist[:])
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines = ['var l = [0, 1, 2]', 'echo l[g:astring : g:theone]']
  v9.CheckDefExecAndScriptFailure(lines, ['E1012:', 'E1030:'], 2)

  lines =<< trim END
      var ld = []
      def Func()
        eval ld[0].key
      enddef
      defcompile
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_dict_subscript()
  var lines =<< trim END
      var l = [{lnum: 2}, {lnum: 1}]
      var res = l[0].lnum > l[1].lnum
      assert_true(res)

      assert_equal(2, g:adict['aaa'])
      assert_equal(8, g:adict.bbb)

      var dd = {}
      def Func1()
        eval dd.key1.key2
      enddef
      def Func2()
        eval dd['key1'].key2
      enddef
      defcompile
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_blob_subscript()
  var lines =<< trim END
      var b = 0z112233
      assert_equal(0x11, b[0])
      assert_equal(0z112233, b[:])

      assert_equal(0x01, g:ablob[0])
      assert_equal(0z01ab, g:ablob[:])
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_expr9_funcref_subscript()
  var lines =<< trim END
      var l = function('len')("abc")
      assert_equal(3, l)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(["var l = function('len')(xxx)"], ['E1001: Variable not found: xxx', 'E121: Undefined variable: xxx'], 1)
enddef

def Test_expr9_subscript_linebreak()
  var lines =<< trim END
      var range = range(
                    3)
      var l = range
            ->mapnew('string(v:key)')
      assert_equal(['0', '1', '2'], l)

      l = range
            ->mapnew('string(v:key)')
      assert_equal(['0', '1', '2'], l)

      l = range # comment
            ->mapnew('string(v:key)')
      assert_equal(['0', '1', '2'], l)

      l = range

            ->mapnew('string(v:key)')
      assert_equal(['0', '1', '2'], l)

      l = range
            # comment
            ->mapnew('string(v:key)')
      assert_equal(['0', '1', '2'], l)

      assert_equal('1', l[
            1])

      var d = {one: 33}
      assert_equal(33, d
            .one)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var d = {one: 33}
      assert_equal(33, d.
            one)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1127:', 'E116:'], 2)
enddef

func Test_expr9_trailing_fails()
  call v9.CheckDefAndScriptFailure(['var l = [2]', 'l->((ll) => add(ll, 8))'], 'E107:', 2)
  call v9.CheckDefAndScriptFailure(['var l = [2]', 'l->((ll) => add(ll, 8)) ()'], 'E274:', 2)
endfunc

func Test_expr_fails()
  call v9.CheckDefAndScriptFailure(["var x = '1'is2"], 'E488:', 1)
  call v9.CheckDefAndScriptFailure(["var x = '1'isnot2"], 'E488:', 1)

  call v9.CheckDefAndScriptFailure(["CallMe ('yes')"], ['E476:', 'E492:'], 1)

  call v9.CheckDefAndScriptFailure(["CallMe2('yes','no')"], 'E1069:', 1)

  call v9.CheckDefAndScriptFailure(["v:nosuch += 3"], ['E1001:', 'E121:'], 1)
  call v9.CheckDefAndScriptFailure(["var v:statusmsg = ''"], 'E1016: Cannot declare a v: variable:', 1)
  call v9.CheckDefAndScriptFailure(["var asdf = v:nosuch"], ['E1001:', 'E121:'], 1)

  call v9.CheckDefFailure(["echo len('asdf'"], 'E110:', 2)
  call v9.CheckScriptFailure(['vim9script', "echo len('asdf'"], 'E116:', 2)

  call v9.CheckDefAndScriptFailure(["echo Func01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789()"], ['E1011:', 'E117:'], 1)
  call v9.CheckDefAndScriptFailure(["echo doesnotexist()"], 'E117:', 1)
endfunc

" vim: shiftwidth=2 sts=2 expandtab
