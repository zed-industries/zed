" Test various aspects of the Vim9 script language.

source check.vim
source term_util.vim
source view_util.vim
import './vim9.vim' as v9
source screendump.vim

func Test_def_basic()
  def SomeFunc(): string
    return 'yes'
  enddef
  call SomeFunc()->assert_equal('yes')
endfunc

func Test_compiling_error()
  " use a terminal to see the whole error message
  CheckRunVimInTerminal

  call TestCompilingError()
  call TestCompilingErrorInTry()
endfunc

def TestCompilingError()
  var lines =<< trim END
    vim9script
    def Fails()
      echo nothing
    enddef
    defcompile
  END
  writefile(lines, 'XTest_compile_error', 'D')
  var buf = g:RunVimInTerminal('-S XTest_compile_error',
              {rows: 10, wait_for_ruler: 0})
  g:WaitForAssert(() => assert_match('Error detected while compiling command line.*Fails.*Variable not found: nothing',
                     g:Term_getlines(buf, range(1, 9))))

  # clean up
  g:StopVimInTerminal(buf)
enddef

def TestCompilingErrorInTry()
  var dir = 'Xcompdir/autoload'
  mkdir(dir, 'pR')

  var lines =<< trim END
      vim9script
      export def OnlyCompiled()
        g:runtime = 'yes'
        invalid
      enddef
  END
  writefile(lines, dir .. '/script.vim')

  lines =<< trim END
      vim9script
      todo
      try
        script#OnlyCompiled()
      catch /nothing/
      endtry
  END
  lines[1] = 'set rtp=' .. getcwd() .. '/Xcompdir'
  writefile(lines, 'XTest_compile_error', 'D')

  var buf = g:RunVimInTerminal('-S XTest_compile_error', {rows: 10, wait_for_ruler: 0})
  g:WaitForAssert(() => assert_match('Error detected while compiling command line.*function script#OnlyCompiled.*Invalid command: invalid',
                     g:Term_getlines(buf, range(1, 9))))

  # clean up
  g:StopVimInTerminal(buf)
enddef

def Test_comment_error()
  v9.CheckDefFailure(['#{ comment'], 'E1170:')
enddef

def Test_compile_error_in_called_function()
  var lines =<< trim END
      vim9script
      var n: number
      def Foo()
        &hls = n
      enddef
      def Bar()
        Foo()
      enddef
      silent! Foo()
      Bar()
  END
  v9.CheckScriptFailureList(lines, ['E1012:', 'E1191:'])
enddef

def Test_wrong_function_name()
  var lines =<< trim END
      vim9script
      func _Foo()
        echo 'foo'
      endfunc
  END
  v9.CheckScriptFailure(lines, 'E1267:')

  lines =<< trim END
      vim9script
      def _Foo()
        echo 'foo'
      enddef
  END
  v9.CheckScriptFailure(lines, 'E1267:')

  lines =<< trim END
      vim9script
      var Object = {}
      function Object.Method()
      endfunction
  END
  v9.CheckScriptFailure(lines, 'E1182:')

  lines =<< trim END
      vim9script
      var Object = {}
      def Object.Method()
      enddef
  END
  v9.CheckScriptFailure(lines, 'E1182:')

  lines =<< trim END
      vim9script
      g:Object = {}
      function g:Object.Method()
      endfunction
  END
  v9.CheckScriptFailure(lines, 'E1182:')

  lines =<< trim END
      let s:Object = {}
      def Define()
        function s:Object.Method()
        endfunction
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1182:')
  delfunc g:Define

  lines =<< trim END
      let s:Object = {}
      def Define()
        def Object.Method()
        enddef
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1182:')
  delfunc g:Define

  lines =<< trim END
      let g:Object = {}
      def Define()
        function g:Object.Method()
        endfunction
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1182:')
  delfunc g:Define
enddef

def Test_listing_function_error()
  var lines =<< trim END
      var filler = 123
      func DoesNotExist
  END
  v9.CheckDefExecFailure(lines, 'E123:', 2)
enddef

def Test_break_in_skipped_block()
  var lines =<< trim END
      vim9script

      def FixStackFrame(): string
          for _ in [2]
              var path = 'xxx'
              if !!path
                  if false
                      break
                  else
                      return 'foo'
                  endif
              endif
          endfor
          return 'xxx'
      enddef

      disas FixStackFrame

      FixStackFrame()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_autoload_name_mismatch()
  var dir = 'Xnamedir/autoload'
  mkdir(dir, 'pR')

  var lines =<< trim END
      vim9script
      export def NoFunction()
        # comment
        g:runtime = 'yes'
      enddef
  END
  writefile(lines, dir .. '/script.vim')

  var save_rtp = &rtp
  exe 'set rtp=' .. getcwd() .. '/Xnamedir'
  lines =<< trim END
      call script#Function()
  END
  v9.CheckScriptFailure(lines, 'E117:', 1)

  &rtp = save_rtp
enddef

def Test_autoload_names()
  var dir = 'Xandir/autoload'
  mkdir(dir, 'pR')

  var lines =<< trim END
      func foobar#function()
        return 'yes'
      endfunc
      let foobar#var = 'no'
  END
  writefile(lines, dir .. '/foobar.vim')

  var save_rtp = &rtp
  exe 'set rtp=' .. getcwd() .. '/Xandir'

  lines =<< trim END
      assert_equal('yes', foobar#function())
      var Function = foobar#function
      assert_equal('yes', Function())

      assert_equal('no', foobar#var)
  END
  v9.CheckDefAndScriptSuccess(lines)

  &rtp = save_rtp
enddef

def Test_autoload_error_in_script()
  var dir = 'Xaedir/autoload'
  mkdir(dir, 'pR')

  var lines =<< trim END
      func scripterror#function()
        let g:called_function = 'yes'
      endfunc
      let 0 = 1
  END
  writefile(lines, dir .. '/scripterror.vim')

  var save_rtp = &rtp
  exe 'set rtp=' .. getcwd() .. '/Xaedir'

  g:called_function = 'no'
  # The error in the autoload script cannot be checked with assert_fails(), use
  # CheckDefSuccess() instead of CheckDefFailure()
  try
    v9.CheckDefSuccess(['scripterror#function()'])
  catch
    assert_match('E121: Undefined variable: 0', v:exception)
  endtry
  assert_equal('no', g:called_function)

  lines =<< trim END
      func scriptcaught#function()
        let g:called_function = 'yes'
      endfunc
      try
        let 0 = 1
      catch
        let g:caught = v:exception
      endtry
  END
  writefile(lines, dir .. '/scriptcaught.vim')

  g:called_function = 'no'
  v9.CheckDefSuccess(['scriptcaught#function()'])
  assert_match('E121: Undefined variable: 0', g:caught)
  assert_equal('yes', g:called_function)

  &rtp = save_rtp
enddef

def s:CallRecursive(n: number): number
  return CallRecursive(n + 1)
enddef

def s:CallMapRecursive(l: list<number>): number
  return map(l, (_, v) => CallMapRecursive([v]))[0]
enddef

def Test_funcdepth_error()
  set maxfuncdepth=10

  var caught = false
  try
    CallRecursive(1)
  catch /E132:/
    caught = true
  endtry
  assert_true(caught)

  caught = false
  try
    CallMapRecursive([1])
  catch /E132:/
    caught = true
  endtry
  assert_true(caught)

  set maxfuncdepth&
enddef

def Test_endfunc_enddef()
  var lines =<< trim END
    def Test()
      echo 'test'
      endfunc
    enddef
  END
  v9.CheckScriptFailure(lines, 'E1151:', 3)

  lines =<< trim END
    def Test()
      func Nested()
        echo 'test'
      enddef
    enddef
  END
  v9.CheckScriptFailure(lines, 'E1152:', 4)

  lines =<< trim END
    def Ok()
      echo 'hello'
    enddef | echo 'there'
    def Bad()
      echo 'hello'
    enddef there
  END
  v9.CheckScriptFailure(lines, 'E1173: Text found after enddef: there', 6)
enddef

def Test_missing_endfunc_enddef()
  var lines =<< trim END
    vim9script
    def Test()
      echo 'test'
    endef
  END
  v9.CheckScriptFailure(lines, 'E1057:', 2)

  lines =<< trim END
    vim9script
    func Some()
      echo 'test'
    enfffunc
  END
  v9.CheckScriptFailure(lines, 'E126:', 2)
enddef

def Test_white_space_before_paren()
  var lines =<< trim END
    vim9script
    def Test ()
      echo 'test'
    enddef
  END
  v9.CheckScriptFailure(lines, 'E1068:', 2)

  lines =<< trim END
    vim9script
    func Test ()
      echo 'test'
    endfunc
  END
  v9.CheckScriptFailure(lines, 'E1068:', 2)

  lines =<< trim END
    def Test ()
      echo 'test'
    enddef
  END
  v9.CheckScriptFailure(lines, 'E1068:', 1)

  lines =<< trim END
    func Test ()
      echo 'test'
    endfunc
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_enddef_dict_key()
  var d = {
    enddef: 'x',
    endfunc: 'y',
  }
  assert_equal({enddef: 'x', endfunc: 'y'}, d)
enddef

def ReturnString(): string
  return 'string'
enddef

def ReturnNumber(): number
  return 123
enddef

let g:notNumber = 'string'

def ReturnGlobal(): number
  return g:notNumber
enddef

def Test_return_something()
  g:ReturnString()->assert_equal('string')
  g:ReturnNumber()->assert_equal(123)
  assert_fails('g:ReturnGlobal()', 'E1012: Type mismatch; expected number but got string', '', 1, 'ReturnGlobal')

  var lines =<< trim END
      vim9script

      def Msg()
          echomsg 'in Msg()...'
      enddef

      def Func()
        return Msg()
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1096:')
enddef

def Test_check_argument_type()
  var lines =<< trim END
      vim9script
      def Val(a: number, b: number): number
        return 0
      enddef
      def Func()
        var x: any = true
        Val(0, x)
      enddef
      disass Func
      Func()
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected number but got bool', 2)

  lines =<< trim END
      vim9script

      def Foobar(Fn: func(any, ?string): any)
      enddef

      Foobar((t) => 0)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_missing_return()
  v9.CheckDefFailure(['def Missing(): number',
                   '  if g:cond',
                   '    echo "no return"',
                   '  else',
                   '    return 0',
                   '  endif',
                   'enddef'], 'E1027:')
  v9.CheckDefFailure(['def Missing(): number',
                   '  if g:cond',
                   '    return 1',
                   '  else',
                   '    echo "no return"',
                   '  endif',
                   'enddef'], 'E1027:')
  v9.CheckDefFailure(['def Missing(): number',
                   '  if g:cond',
                   '    return 1',
                   '  else',
                   '    return 2',
                   '  endif',
                   '  return 3',
                   'enddef'], 'E1095:')
enddef

def Test_not_missing_return()
  var lines =<< trim END
      def Funky(): number
        if false
          return 0
        endif
        throw 'Error'
      enddef
      defcompile
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_return_bool()
  var lines =<< trim END
      vim9script
      def MenuFilter(id: number, key: string): bool
        return popup_filter_menu(id, key)
      enddef
      def YesnoFilter(id: number, key: string): bool
        return popup_filter_yesno(id, key)
      enddef
      defcompile
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_return_void_comment_follows()
  var lines =<< trim END
    vim9script
    def ReturnCommentFollows(): void
      return # Some comment
    enddef
    defcompile
  END
  v9.CheckScriptSuccess(lines)
enddef

let s:nothing = 0
def ReturnNothing()
  s:nothing = 1
  if true
    return
  endif
  s:nothing = 2
enddef

def Test_return_nothing()
  g:ReturnNothing()
  s:nothing->assert_equal(1)
enddef

def Test_return_invalid()
  var lines =<< trim END
    vim9script
    def Func(): invalid
      return xxx
    enddef
    defcompile
  END
  v9.CheckScriptFailure(lines, 'E1010:', 2)

  lines =<< trim END
      vim9script
      def Test(Fun: func(number): number): list<number>
          return map([1, 2, 3], (_, i) => Fun(i))
      enddef
      defcompile
      def Inc(nr: number): nr
        return nr + 2
      enddef
      echo Test(Inc)
  END
  # doing this twice was leaking memory
  v9.CheckScriptFailure(lines, 'E1010:')
  v9.CheckScriptFailure(lines, 'E1010:')
enddef

def Test_return_list_any()
  # This used to fail but now the actual list type is checked, and since it has
  # an item of type string it can be used as list<string>.
  var lines =<< trim END
      vim9script
      def Func(): list<string>
        var l: list<any>
        l->add('string')
        return l
      enddef
      echo Func()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Func(): list<string>
        var l: list<any>
        l += ['string']
        return l
      enddef
      echo Func()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_return_any_two_types()
  var lines =<< trim END
      vim9script

      def G(Fn: func(string): any)
        g:result = Fn("hello")
      enddef

      def F(a: number, b: string): any
        echo b
        if a > 0
          return 1
        else
          return []
        endif
      enddef

      G(function(F, [1]))
  END
  v9.CheckScriptSuccess(lines)
  assert_equal(1, g:result)
  unlet g:result
enddef

func s:Increment()
  let g:counter += 1
endfunc

def Test_call_ufunc_count()
  g:counter = 1
  Increment()
  Increment()
  Increment()
  # works with and without :call
  g:counter->assert_equal(4)
  eval g:counter->assert_equal(4)
  unlet g:counter
enddef

def Test_call_ufunc_failure()
  var lines =<< trim END
      vim9script
      def Tryit()
        g:Global(1, 2, 3)
      enddef

      func g:Global(a, b, c)
        echo a:a a:b a:c
      endfunc

      defcompile

      func! g:Global(a, b)
        echo a:a a:b
      endfunc
      Tryit()
  END
  v9.CheckScriptFailure(lines, 'E118: Too many arguments for function: Global')
  delfunc g:Global

  lines =<< trim END
      vim9script

      g:Ref = function('len')
      def Tryit()
        g:Ref('x')
      enddef

      defcompile

      g:Ref = function('add')
      Tryit()
  END
  v9.CheckScriptFailure(lines, 'E119: Not enough arguments for function: add')
  unlet g:Ref
enddef

def s:MyVarargs(arg: string, ...rest: list<string>): string
  var res = arg
  for s in rest
    res ..= ',' .. s
  endfor
  return res
enddef

def Test_call_varargs()
  MyVarargs('one')->assert_equal('one')
  MyVarargs('one', 'two')->assert_equal('one,two')
  MyVarargs('one', 'two', 'three')->assert_equal('one,two,three')
enddef

def Test_call_white_space()
  v9.CheckDefAndScriptFailure(["call Test ('text')"], ['E476:', 'E1068:'])
enddef

def MyDefaultArgs(name = 'string'): string
  return name
enddef

def s:MyDefaultSecond(name: string, second: bool  = true): string
  return second ? name : 'none'
enddef


def Test_call_default_args()
  g:MyDefaultArgs()->assert_equal('string')
  g:MyDefaultArgs(v:none)->assert_equal('string')
  g:MyDefaultArgs('one')->assert_equal('one')
  assert_fails('g:MyDefaultArgs("one", "two")', 'E118:', '', 4, 'Test_call_default_args')

  MyDefaultSecond('test')->assert_equal('test')
  MyDefaultSecond('test', true)->assert_equal('test')
  MyDefaultSecond('test', false)->assert_equal('none')

  var lines =<< trim END
      def MyDefaultThird(name: string, aa = 'aa', bb = 'bb'): string
        return name .. aa .. bb
      enddef

      MyDefaultThird('->')->assert_equal('->aabb')
      MyDefaultThird('->', v:none)->assert_equal('->aabb')
      MyDefaultThird('->', 'xx')->assert_equal('->xxbb')
      MyDefaultThird('->', v:none, v:none)->assert_equal('->aabb')
      MyDefaultThird('->', 'xx', v:none)->assert_equal('->xxbb')
      MyDefaultThird('->', v:none, 'yy')->assert_equal('->aayy')
      MyDefaultThird('->', 'xx', 'yy')->assert_equal('->xxyy')

      def DefArg(mandatory: any, optional = mandatory): string
        return mandatory .. optional
      enddef
      DefArg(1234)->assert_equal('12341234')
      DefArg("ok")->assert_equal('okok')
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckScriptFailure(['def Func(arg: number = asdf)', 'enddef', 'defcompile'], 'E1001:')
  delfunc g:Func
  v9.CheckScriptFailure(['def Func(arg: number = "text")', 'enddef', 'defcompile'], 'E1013: Argument 1: type mismatch, expected number but got string')
  delfunc g:Func
  v9.CheckDefFailure(['def Func(x: number = )', 'enddef'], 'E15:')

  lines =<< trim END
      vim9script
      def Func(a = b == 0 ? 1 : 2, b = 0)
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1001: Variable not found: b')

  # using script variable requires matching type or type cast when executed
  lines =<< trim END
      vim9script
      var a: any
      def Func(arg: string = a)
        echo arg
      enddef
      defcompile
  END
  v9.CheckScriptSuccess(lines + ['a = "text"', 'Func()'])
  v9.CheckScriptFailure(lines + ['a = 123', 'Func()'], 'E1013: Argument 1: type mismatch, expected string but got number')

  # using global variable does not require type cast
  lines =<< trim END
      vim9script
      def Func(arg: string = g:str)
        echo arg
      enddef
      g:str = 'works'
      Func()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_using_vnone_default()
  var lines =<< trim END
      vim9script

      def F(a: string = v:none)
         if a isnot v:none
            var b = a
         endif
      enddef
      F()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script

      export def Floats(x: float, y = 2.0, z = 5.0)
        g:result = printf("%.2f %.2f %.2f", x, y, z)
      enddef
  END
  writefile(lines, 'Xlib.vim', 'D')

  # test using a function reference in script-local variable
  lines =<< trim END
      vim9script

      import './Xlib.vim'
      const Floatfunc = Xlib.Floats
      Floatfunc(1.0, v:none, 3.0)
  END
  v9.CheckScriptSuccess(lines)
  assert_equal('1.00 2.00 3.00', g:result)
  unlet g:result

  # test calling the imported function
  lines =<< trim END
      vim9script

      import './Xlib.vim'
      Xlib.Floats(1.0, v:none, 3.0)
  END
  v9.CheckScriptSuccess(lines)
  assert_equal('1.00 2.00 3.00', g:result)
  unlet g:result

  # TODO: this should give an error for using a missing argument
  # lines =<< trim END
  #    vim9script

  #    def F(a: string = v:none)
  #       var b = a
  #    enddef
  #    F()
  # END
  # v9.CheckScriptFailure(lines, 'E99:')
enddef

def Test_convert_number_to_float()
  var lines =<< trim END
      vim9script
      def  Foo(a: float, b: float): float
         return a + b
      enddef

      assert_equal(5.3, Foo(3.3, 2))
  END
  v9.CheckScriptSuccess(lines)
enddef

def s:FuncWithComment(  # comment
  a: number, #comment
  b: bool, # comment
  c: string) #comment
  assert_equal(4, a)
  assert_equal(true, b)
  assert_equal('yes', c)
enddef

def Test_func_with_comments()
  FuncWithComment(4, true, 'yes')

  var lines =<< trim END
      def Func(# comment
        arg: string)
      enddef
  END
  v9.CheckScriptFailure(lines, 'E125:', 1)

  lines =<< trim END
      def Func(f=
      )
      enddef
  END
  v9.CheckScriptFailure(lines, 'E125:', 2)

  lines =<< trim END
      def Func(
        arg: string# comment
        )
      enddef
  END
  v9.CheckScriptFailure(lines, 'E475:', 2)

  lines =<< trim END
      def Func(
        arg: string
        )# comment
      enddef
  END
  v9.CheckScriptFailure(lines, 'E488:', 3)
enddef

def Test_nested_function()
  def NestedDef(arg: string): string
    return 'nested ' .. arg
  enddef
  NestedDef(':def')->assert_equal('nested :def')

  func NestedFunc(arg)
    return 'nested ' .. a:arg
  endfunc
  NestedFunc(':func')->assert_equal('nested :func')

  v9.CheckDefFailure(['def Nested()', 'enddef', 'Nested(66)'], 'E118:')
  v9.CheckDefFailure(['def Nested(arg: string)', 'enddef', 'Nested()'], 'E119:')

  v9.CheckDefFailure(['def s:Nested()', 'enddef'], 'E1075:')
  v9.CheckDefFailure(['def b:Nested()', 'enddef'], 'E1075:')

  var lines =<< trim END
      def Outer()
        def Inner()
          # comment
        enddef
        def Inner()
        enddef
      enddef
  END
  v9.CheckDefFailure(lines, 'E1073:')

  lines =<< trim END
      def Outer()
        def Inner()
          # comment
        enddef
        def! Inner()
        enddef
      enddef
  END
  v9.CheckDefFailure(lines, 'E1117:')

  lines =<< trim END
      vim9script
      def Outer()
        def Inner()
          g:result = 'ok'
        enddef
        Inner()
      enddef
      Outer()
      Inner()
  END
  v9.CheckScriptFailure(lines, 'E117: Unknown function: Inner')
  assert_equal('ok', g:result)
  unlet g:result

  lines =<< trim END
      vim9script
      def Outer()
        def _Inner()
          echo 'bad'
        enddef
        _Inner()
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1267:')

  lines =<< trim END
      vim9script
      def Outer()
        def g:inner()
          echo 'bad'
        enddef
        g:inner()
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1267:')

  lines =<< trim END
      vim9script
      def g:_Func()
        echo 'bad'
      enddef
  END
  v9.CheckScriptFailure(lines, 'E1267:')

  lines =<< trim END
      vim9script
      def _Func()
        echo 'bad'
      enddef
  END
  v9.CheckScriptFailure(lines, 'E1267:')

  # nested function inside conditional
  lines =<< trim END
      vim9script
      var thecount = 0
      if true
        def Test(): number
          def TheFunc(): number
            thecount += 1
            return thecount
          enddef
          return TheFunc()
        enddef
      endif
      defcompile
      assert_equal(1, Test())
      assert_equal(2, Test())
  END
  v9.CheckScriptSuccess(lines)

  # also works when "thecount" is inside the "if" block
  lines =<< trim END
      vim9script
      if true
        var thecount = 0
        def Test(): number
          def TheFunc(): number
            thecount += 1
            return thecount
          enddef
          return TheFunc()
        enddef
      endif
      defcompile
      assert_equal(1, Test())
      assert_equal(2, Test())
  END
  v9.CheckScriptSuccess(lines)

  # nested function with recursive call
  lines =<< trim END
      vim9script

      def MyFunc(): number
        def Fib(n: number): number
          if n < 2
            return 1
          endif
          return Fib(n - 2) + Fib(n - 1)
        enddef

        return Fib(5)
      enddef

      assert_equal(8, MyFunc())
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Outer()
        def Inner()
          echo 'hello'
        enddef burp
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1173: Text found after enddef: burp', 3)
enddef

def Test_nested_function_fails()
  var lines =<< trim END
      def T()
        def Func(g: string):string
        enddef
        Func()
      enddef
      silent! defcompile
  END
  v9.CheckScriptFailure(lines, 'E1069:')
enddef

def Test_not_nested_function()
  echo printf('%d',
      function('len')('xxx'))
enddef

func Test_call_default_args_from_func()
  call MyDefaultArgs()->assert_equal('string')
  call MyDefaultArgs('one')->assert_equal('one')
  call assert_fails('call MyDefaultArgs("one", "two")', 'E118:', '', 3, 'Test_call_default_args_from_func')
endfunc

def Test_nested_global_function()
  var lines =<< trim END
      vim9script
      def Outer()
          def g:Inner(): string
              return 'inner'
          enddef
      enddef
      defcompile
      Outer()
      g:Inner()->assert_equal('inner')
      delfunc g:Inner
      Outer()
      g:Inner()->assert_equal('inner')
      delfunc g:Inner
      Outer()
      g:Inner()->assert_equal('inner')
      delfunc g:Inner
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Outer()
          func g:Inner()
            return 'inner'
          endfunc
      enddef
      defcompile
      Outer()
      g:Inner()->assert_equal('inner')
      delfunc g:Inner
      Outer()
      g:Inner()->assert_equal('inner')
      delfunc g:Inner
      Outer()
      g:Inner()->assert_equal('inner')
      delfunc g:Inner
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Outer()
          def g:Inner(): string
              return 'inner'
          enddef
      enddef
      defcompile
      Outer()
      Outer()
  END
  v9.CheckScriptFailure(lines, "E122:")
  delfunc g:Inner

  lines =<< trim END
      vim9script
      def Outer()
        def g:Inner()
          echo map([1, 2, 3], (_, v) => v + 1)
        enddef
        g:Inner()
      enddef
      Outer()
  END
  v9.CheckScriptSuccess(lines)
  delfunc g:Inner

  lines =<< trim END
      vim9script
      def Func()
        echo 'script'
      enddef
      def Outer()
        def Func()
          echo 'inner'
        enddef
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, "E1073:", 1)

  lines =<< trim END
      vim9script
      def Func()
        echo 'script'
      enddef
      def Func()
        echo 'script'
      enddef
  END
  v9.CheckScriptFailure(lines, "E1073:", 5)
enddef

def DefListAll()
  def
enddef

def DefListOne()
  def DefListOne
enddef

def DefListMatches()
  def /DefList
enddef

def Test_nested_def_list()
  var funcs = split(execute('call DefListAll()'), "\n")
  assert_true(len(funcs) > 10)
  assert_true(funcs->index('def DefListAll()') >= 0)

  funcs = split(execute('call DefListOne()'), "\n")
  assert_equal(['   def DefListOne()', '1    def DefListOne', '   enddef'], funcs)

  funcs = split(execute('call DefListMatches()'), "\n")
  assert_true(len(funcs) >= 3)
  assert_true(funcs->index('def DefListAll()') >= 0)
  assert_true(funcs->index('def DefListOne()') >= 0)
  assert_true(funcs->index('def DefListMatches()') >= 0)

  var lines =<< trim END
    vim9script
    def Func()
      def +Func+
    enddef
    defcompile
  END
  v9.CheckScriptFailure(lines, 'E476:', 1)
enddef

def Test_global_function_not_found()
  var lines =<< trim END
      g:Ref = 123
      call g:Ref()
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E117:', 'E1085:'], 2)
enddef

def Test_global_local_function()
  var lines =<< trim END
      vim9script
      def g:Func(): string
          return 'global'
      enddef
      def Func(): string
          return 'local'
      enddef
      g:Func()->assert_equal('global')
      Func()->assert_equal('local')
      delfunc g:Func
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def g:Funcy()
        echo 'funcy'
      enddef
      Funcy()
  END
  v9.CheckScriptFailure(lines, 'E117:')
enddef

def Test_local_function_shadows_global()
  var lines =<< trim END
      vim9script
      def g:Gfunc(): string
        return 'global'
      enddef
      def AnotherFunc(): number
        var Gfunc = function('len')
        return Gfunc('testing')
      enddef
      g:Gfunc()->assert_equal('global')
      AnotherFunc()->assert_equal(7)
      delfunc g:Gfunc
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def g:Func(): string
        return 'global'
      enddef
      def AnotherFunc()
        g:Func = function('len')
      enddef
      AnotherFunc()
  END
  v9.CheckScriptFailure(lines, 'E705:')
  delfunc g:Func

  # global function is not found with g: prefix
  lines =<< trim END
      vim9script
      def g:Func(): string
        return 'global'
      enddef
      def AnotherFunc(): string
        return Func()
      enddef
      assert_equal('global', AnotherFunc())
  END
  v9.CheckScriptFailure(lines, 'E117:')
  delfunc g:Func

  lines =<< trim END
      vim9script
      def g:Func(): string
        return 'global'
      enddef
      assert_equal('global', g:Func())
      delfunc g:Func
  END
  v9.CheckScriptSuccess(lines)

  # This does not shadow "i" which is visible only inside the for loop
  lines =<< trim END
      vim9script

      def Foo(i: number)
        echo i
      enddef

      for i in range(3)
        # Foo() is compiled here
        Foo(i)
      endfor
  END
  v9.CheckScriptSuccess(lines)
enddef

func TakesOneArg(arg)
  echo a:arg
endfunc

def Test_call_wrong_args()
  v9.CheckDefFailure(['g:TakesOneArg()'], 'E119:')
  v9.CheckDefFailure(['g:TakesOneArg(11, 22)'], 'E118:')
  v9.CheckDefFailure(['bufnr(xxx)'], 'E1001:')
  v9.CheckScriptFailure(['def Func(Ref: func(s: string))'], 'E475:')

  var lines =<< trim END
    vim9script
    def Func(s: string)
      echo s
    enddef
    Func([])
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch, expected string but got list<any>', 5)

  # argument name declared earlier is found when declaring a function
  lines =<< trim END
    vim9script
    var name = 'piet'
    def FuncOne(name: string)
      echo name
    enddef
  END
  v9.CheckScriptFailure(lines, 'E1168:')

  # same, inside the same block
  lines =<< trim END
    vim9script
    if true
      var name = 'piet'
      def FuncOne(name: string)
        echo name
      enddef
    endif
  END
  v9.CheckScriptFailure(lines, 'E1168:')

  # variable in other block is OK
  lines =<< trim END
    vim9script
    if true
      var name = 'piet'
    endif
    def FuncOne(name: string)
      echo name
    enddef
  END
  v9.CheckScriptSuccess(lines)

  # with another variable in another block
  lines =<< trim END
    vim9script
    if true
      var name = 'piet'
      # define a function so that the variable isn't cleared
      def GetItem(): string
        return item
      enddef
    endif
    if true
      var name = 'peter'
      def FuncOne(name: string)
        echo name
      enddef
    endif
  END
  v9.CheckScriptFailure(lines, 'E1168:')

  # only variable in another block is OK
  lines =<< trim END
    vim9script
    if true
      var name = 'piet'
      # define a function so that the variable isn't cleared
      def GetItem(): string
        return item
      enddef
    endif
    if true
      def FuncOne(name: string)
        echo name
      enddef
    endif
  END
  v9.CheckScriptSuccess(lines)

  # argument name declared later is only found when compiling
  lines =<< trim END
    vim9script
    def FuncOne(name: string)
      echo nr
    enddef
    var name = 'piet'
  END
  v9.CheckScriptSuccess(lines)
  v9.CheckScriptFailure(lines + ['defcompile'], 'E1168:')

  lines =<< trim END
    vim9script
    def FuncOne(nr: number)
      echo nr
    enddef
    def FuncTwo()
      FuncOne()
    enddef
    defcompile
  END
  writefile(lines, 'Xscript')
  var didCatch = false
  try
    source Xscript
  catch
    assert_match('E119: Not enough arguments for function: <SNR>\d\+_FuncOne', v:exception)
    assert_match('Xscript\[8\]..function <SNR>\d\+_FuncTwo, line 1', v:throwpoint)
    didCatch = true
  endtry
  assert_true(didCatch)

  lines =<< trim END
    vim9script
    def FuncOne(nr: number)
      echo nr
    enddef
    def FuncTwo()
      FuncOne(1, 2)
    enddef
    defcompile
  END
  writefile(lines, 'Xscript', 'D')
  didCatch = false
  try
    source Xscript
  catch
    assert_match('E118: Too many arguments for function: <SNR>\d\+_FuncOne', v:exception)
    assert_match('Xscript\[8\]..function <SNR>\d\+_FuncTwo, line 1', v:throwpoint)
    didCatch = true
  endtry
  assert_true(didCatch)
enddef

def Test_call_funcref_wrong_args()
  var head =<< trim END
      vim9script
      def Func3(a1: string, a2: number, a3: list<number>)
        echo a1 .. a2 .. a3[0]
      enddef
      def Testme()
        var funcMap: dict<func> = {func: Func3}
  END
  var tail =<< trim END
      enddef
      Testme()
  END
  v9.CheckScriptSuccess(head + ["funcMap['func']('str', 123, [1, 2, 3])"] + tail)

  v9.CheckScriptFailure(head + ["funcMap['func']('str', 123)"] + tail, 'E119:')
  v9.CheckScriptFailure(head + ["funcMap['func']('str', 123, [1], 4)"] + tail, 'E118:')

  var lines =<< trim END
      vim9script
      var Ref: func(number): any
      Ref = (j) => !j
      echo Ref(false)
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got bool', 4)

  lines =<< trim END
      vim9script
      var Ref: func(number): any
      Ref = (j) => !j
      call Ref(false)
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got bool', 4)
enddef

def Test_call_lambda_args()
  var lines =<< trim END
    var Callback = (..._) => 'anything'
    assert_equal('anything', Callback())
    assert_equal('anything', Callback(1))
    assert_equal('anything', Callback('a', 2))

    assert_equal('xyz', ((a: string): string => a)('xyz'))
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefFailure(['echo ((i) => 0)()'],
                  'E119: Not enough arguments for function: ((i) => 0)()')

  lines =<< trim END
      var Ref = (x: number, y: number) => x + y
      echo Ref(1, 'x')
  END
  v9.CheckDefFailure(lines, 'E1013: Argument 2: type mismatch, expected number but got string')

  lines =<< trim END
    var Ref: func(job, string, number)
    Ref = (x, y) => 0
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:')

  lines =<< trim END
    var Ref: func(job, string)
    Ref = (x, y, z) => 0
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012:')

  lines =<< trim END
      var one = 1
      var l = [1, 2, 3]
      echo map(l, (one) => one)
  END
  v9.CheckDefFailure(lines, 'E1167:')
  v9.CheckScriptFailure(['vim9script'] + lines, 'E1168:')

  lines =<< trim END
    var Ref: func(any, ?any): bool
    Ref = (_, y = 1) => false
  END
  v9.CheckDefAndScriptFailure(lines, 'E1172:')

  lines =<< trim END
      var a = 0
      var b = (a == 0 ? 1 : 2)
      assert_equal(1, b)
      var txt = 'a'
      b = (txt =~ 'x' ? 1 : 2)
      assert_equal(2, b)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      def ShadowLocal()
        var one = 1
        var l = [1, 2, 3]
        echo map(l, (one) => one)
      enddef
  END
  v9.CheckDefFailure(lines, 'E1167:')

  lines =<< trim END
      def Shadowarg(one: number)
        var l = [1, 2, 3]
        echo map(l, (one) => one)
      enddef
  END
  v9.CheckDefFailure(lines, 'E1167:')

  lines =<< trim END
    echo ((a) => a)('aa', 'bb')
  END
  v9.CheckDefAndScriptFailure(lines, 'E118:', 1)

  lines =<< trim END
    echo 'aa'->((a) => a)('bb')
  END
  v9.CheckDefFailure(lines, 'E118: Too many arguments for function: ->((a) => a)(''bb'')', 1)
  v9.CheckScriptFailure(['vim9script'] + lines, 'E118: Too many arguments for function: <lambda>', 2)
enddef

def Test_lambda_line_nr()
  var lines =<< trim END
      vim9script
      # comment
      # comment
      var id = timer_start(1'000, (_) => 0)
      var out = execute('verbose ' .. timer_info(id)[0].callback
          ->string()
          ->substitute("('\\|')", ' ', 'g'))
      assert_match('Last set from .* line 4', out)
  END
  v9.CheckScriptSuccess(lines)
enddef

def FilterWithCond(x: string, Cond: func(string): bool): bool
  return Cond(x)
enddef

def Test_lambda_return_type()
  var lines =<< trim END
    var Ref = (): => 123
  END
  v9.CheckDefAndScriptFailure(lines, 'E1157:', 1)

  # no space before the return type
  lines =<< trim END
    var Ref = (x):number => x + 1
  END
  v9.CheckDefAndScriptFailure(lines, 'E1069:', 1)

  # this works
  for x in ['foo', 'boo']
    echo g:FilterWithCond(x, (v) => v =~ '^b')
  endfor

  # this fails
  lines =<< trim END
      echo g:FilterWithCond('foo', (v) => v .. '^b')
  END
  v9.CheckDefAndScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected func(string): bool but got func(any): string', 1)

  lines =<< trim END
      var Lambda1 = (x) => {
              return x
              }
      assert_equal('asdf', Lambda1('asdf'))
      var Lambda2 = (x): string => {
              return x
              }
      assert_equal('foo', Lambda2('foo'))
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      var Lambda = (x): string => {
              return x
              }
      echo Lambda(['foo'])
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1012:')
enddef

def Test_lambda_uses_assigned_var()
  v9.CheckDefSuccess([
        'var x: any = "aaa"',
        'x = filter(["bbb"], (_, v) => v =~ x)'])
enddef

def Test_lambda_invalid_block()
  var lines =<< trim END
      timer_start(0, (_) => { # echo
          echo 'yes'
        })
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      timer_start(0, (_) => { " echo
          echo 'yes'
        })
  END
  v9.CheckDefAndScriptFailure(lines, 'E488: Trailing characters: " echo')

  lines =<< trim END
      timer_start(0, (_) => { | echo
          echo 'yes'
        })
  END
  v9.CheckDefAndScriptFailure(lines, 'E488: Trailing characters: | echo')
enddef

def Test_lambda_with_following_cmd()
  var lines =<< trim END
      set ts=2
      var Lambda = () => {
          set ts=4
        } | set ts=3
      assert_equal(3, &ts)
      Lambda()
      assert_equal(4, &ts)
  END
  v9.CheckDefAndScriptSuccess(lines)
  set ts=8
enddef

def Test_pass_legacy_lambda_to_def_func()
  var lines =<< trim END
      vim9script
      func Foo()
        eval s:Bar({x -> 0})
      endfunc
      def Bar(y: any)
      enddef
      Foo()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def g:TestFunc(F: func)
      enddef
      legacy call g:TestFunc({-> 0})
      delfunc g:TestFunc

      def g:TestFunc(F: func(number))
      enddef
      legacy call g:TestFunc({nr -> 0})
      delfunc g:TestFunc
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_lambda_in_reduce_line_break()
  # this was using freed memory
  var lines =<< trim END
      vim9script
      const result: dict<number> =
          ['Bob', 'Sam', 'Cat', 'Bob', 'Cat', 'Cat']
          ->reduce((acc, val) => {
              if has_key(acc, val)
                  acc[val] += 1
                  return acc
              else
                  acc[val] = 1
                  return acc
              endif
          }, {})
      assert_equal({Bob: 2, Sam: 1, Cat: 3}, result)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_set_opfunc_to_lambda()
  var lines =<< trim END
    vim9script
    nnoremap <expr> <F4> <SID>CountSpaces() .. '_'
    def CountSpaces(type = ''): string
      if type == ''
        &operatorfunc = (t) => CountSpaces(t)
        return 'g@'
      endif
      normal! '[V']y
      g:result = getreg('"')->count(' ')
      return ''
    enddef
    new
    'a b c d e'->setline(1)
    feedkeys("\<F4>", 'x')
    assert_equal(4, g:result)
    bwipe!
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_set_opfunc_to_global_function()
  var lines =<< trim END
    vim9script
    def g:CountSpaces(type = ''): string
      normal! '[V']y
      g:result = getreg('"')->count(' ')
      return ''
    enddef
    # global function works at script level
    &operatorfunc = g:CountSpaces
    new
    'a b c d e'->setline(1)
    feedkeys("g@_", 'x')
    assert_equal(4, g:result)

    &operatorfunc = ''
    g:result = 0
    # global function works in :def function
    def Func()
      &operatorfunc = g:CountSpaces
    enddef
    Func()
    feedkeys("g@_", 'x')
    assert_equal(4, g:result)

    bwipe!
  END
  v9.CheckScriptSuccess(lines)
  &operatorfunc = ''
enddef

def Test_use_script_func_name_with_prefix()
  var lines =<< trim END
      vim9script
      func g:Getit()
        return 'it'
      endfunc
      var Fn = g:Getit
      assert_equal('it', Fn())
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_lambda_type_allocated()
  # Check that unreferencing a partial using a lambda can use the variable type
  # after the lambda has been freed and does not leak memory.
  var lines =<< trim END
    vim9script

    func MyomniFunc1(val, findstart, base)
      return a:findstart ? 0 : []
    endfunc

    var Lambda = (a, b) => MyomniFunc1(19, a, b)
    &omnifunc = Lambda
    Lambda = (a, b) => MyomniFunc1(20, a, b)
    &omnifunc = string(Lambda)
    Lambda = (a, b) => strlen(a)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_define_lambda_in_execute()
  var lines =<< trim [CODE]
      vim9script

      def BuildFuncMultiLine(): func
          var x =<< trim END
              g:SomeRandomFunc = (d: dict<any>) => {
                  return d.k1 + d.k2
              }
          END
          execute(x)
          return g:SomeRandomFunc
      enddef
      var ResultPlus = BuildFuncMultiLine()
      assert_equal(7, ResultPlus({k1: 3, k2: 4}))
  [CODE]
  v9.CheckScriptSuccess(lines)
  unlet g:SomeRandomFunc
enddef

" Default arg and varargs
def MyDefVarargs(one: string, two = 'foo', ...rest: list<string>): string
  var res = one .. ',' .. two
  for s in rest
    res ..= ',' .. s
  endfor
  return res
enddef

def Test_call_def_varargs()
  assert_fails('g:MyDefVarargs()', 'E119:', '', 1, 'Test_call_def_varargs')
  g:MyDefVarargs('one')->assert_equal('one,foo')
  g:MyDefVarargs('one', 'two')->assert_equal('one,two')
  g:MyDefVarargs('one', 'two', 'three')->assert_equal('one,two,three')
  v9.CheckDefFailure(['g:MyDefVarargs("one", 22)'],
      'E1013: Argument 2: type mismatch, expected string but got number')
  v9.CheckDefFailure(['g:MyDefVarargs("one", "two", 123)'],
      'E1013: Argument 3: type mismatch, expected string but got number')

  var lines =<< trim END
      vim9script
      def Func(...l: list<string>)
        echo l
      enddef
      Func('a', 'b', 'c')
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Func(...l: list<string>)
        echo l
      enddef
      Func()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Func(...l: list<any>)
        echo l
      enddef
      Func(0)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Func(...l: any)
        echo l
      enddef
      Func(0)
  END
  v9.CheckScriptFailure(lines, 'E1180:', 2)

  lines =<< trim END
      vim9script
      def Func(..._l: list<string>)
        echo _l
      enddef
      Func('a', 'b', 'c')
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Func(...l: list<string>)
        echo l
      enddef
      Func(1, 2, 3)
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch')

  lines =<< trim END
      vim9script
      def Func(...l: list<string>)
        echo l
      enddef
      Func('a', 9)
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 2: type mismatch')

  lines =<< trim END
      vim9script
      def Func(...l: list<string>)
        echo l
      enddef
      Func(1, 'a')
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch')

  lines =<< trim END
      vim9script
      def Func(  # some comment
                ...l = []
                )
        echo l
      enddef
  END
  v9.CheckScriptFailure(lines, 'E1160:')

  lines =<< trim END
      vim9script
      def DoIt()
        g:Later('')
      enddef
      defcompile
      def g:Later(...l:  list<number>)
      enddef
      DoIt()
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch, expected number but got string')
enddef

let s:value = ''

def FuncOneDefArg(opt = 'text')
  s:value = opt
enddef

def FuncTwoDefArg(nr = 123, opt = 'text'): string
  return nr .. opt
enddef

def FuncVarargs(...arg: list<string>): string
  return join(arg, ',')
enddef

def Test_func_type_varargs()
  var RefDefArg: func(?string)
  RefDefArg = g:FuncOneDefArg
  RefDefArg()
  s:value->assert_equal('text')
  RefDefArg('some')
  s:value->assert_equal('some')

  var RefDef2Arg: func(?number, ?string): string
  RefDef2Arg = g:FuncTwoDefArg
  RefDef2Arg()->assert_equal('123text')
  RefDef2Arg(99)->assert_equal('99text')
  RefDef2Arg(77, 'some')->assert_equal('77some')

  v9.CheckDefFailure(['var RefWrong: func(string?)'], 'E1010:')
  v9.CheckDefFailure(['var RefWrong: func(?string, string)'], 'E1007:')

  var RefVarargs: func(...list<string>): string
  RefVarargs = g:FuncVarargs
  RefVarargs()->assert_equal('')
  RefVarargs('one')->assert_equal('one')
  RefVarargs('one', 'two')->assert_equal('one,two')

  v9.CheckDefFailure(['var RefWrong: func(...list<string>, string)'], 'E110:')
  v9.CheckDefFailure(['var RefWrong: func(...list<string>, ?string)'], 'E110:')
enddef

" Only varargs
def MyVarargsOnly(...args: list<string>): string
  return join(args, ',')
enddef

def Test_call_varargs_only()
  g:MyVarargsOnly()->assert_equal('')
  g:MyVarargsOnly('one')->assert_equal('one')
  g:MyVarargsOnly('one', 'two')->assert_equal('one,two')
  v9.CheckDefFailure(['g:MyVarargsOnly(1)'], 'E1013: Argument 1: type mismatch, expected string but got number')
  v9.CheckDefFailure(['g:MyVarargsOnly("one", 2)'], 'E1013: Argument 2: type mismatch, expected string but got number')
enddef

def Test_varargs_mismatch()
  var lines =<< trim END
      vim9script

      def Map(Fn: func(...any): number): number
        return Fn('12')
      enddef

      var res = Map((v) => str2nr(v))
      assert_equal(12, res)
  END
  v9.CheckScriptFailure(lines, 'E1180: Variable arguments type must be a list: any')
enddef

def Test_using_var_as_arg()
  var lines =<< trim END
      def Func(x: number)
        var x = 234
      enddef
  END
  v9.CheckDefFailure(lines, 'E1006:')

  lines =<< trim END
      def Func(Ref: number)
        def Ref()
        enddef
      enddef
  END
  v9.CheckDefFailure(lines, 'E1073:')
enddef

def s:DictArg(arg: dict<string>)
  arg['key'] = 'value'
enddef

def s:ListArg(arg: list<string>)
  arg[0] = 'value'
enddef

def Test_assign_to_argument()
  # works for dict and list
  var d: dict<string> = {}
  DictArg(d)
  d['key']->assert_equal('value')
  var l: list<string> = []
  ListArg(l)
  l[0]->assert_equal('value')

  v9.CheckScriptFailure(['def Func(arg: number)', 'arg = 3', 'enddef', 'defcompile'], 'E1090:')
  delfunc! g:Func
enddef

" These argument names are reserved in legacy functions.
def s:WithReservedNames(firstline: string, lastline: string): string
  return firstline .. lastline
enddef

def Test_argument_names()
  assert_equal('OK', WithReservedNames('O', 'K'))
enddef

def Test_call_func_defined_later()
  g:DefinedLater('one')->assert_equal('one')
  assert_fails('NotDefined("one")', 'E117:', '', 2, 'Test_call_func_defined_later')
enddef

func DefinedLater(arg)
  return a:arg
endfunc

def Test_call_funcref()
  g:SomeFunc('abc')->assert_equal(3)
  assert_fails('NotAFunc()', 'E117:', '', 2, 'Test_call_funcref') # comment after call
  assert_fails('g:NotAFunc()', 'E1085:', '', 3, 'Test_call_funcref')

  var lines =<< trim END
    vim9script
    def RetNumber(): number
      return 123
    enddef
    var Funcref: func: number = function('RetNumber')
    Funcref()->assert_equal(123)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    vim9script
    def RetNumber(): number
      return 123
    enddef
    def Bar(F: func: number): number
      return F()
    enddef
    var Funcref = function('RetNumber')
    Bar(Funcref)->assert_equal(123)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    vim9script
    def UseNumber(nr: number)
      echo nr
    enddef
    var Funcref: func(number) = function('UseNumber')
    Funcref(123)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    vim9script
    def UseNumber(nr: number)
      echo nr
    enddef
    var Funcref: func(string) = function('UseNumber')
  END
  v9.CheckScriptFailure(lines, 'E1012: Type mismatch; expected func(string) but got func(number)')

  lines =<< trim END
    vim9script
    def EchoNr(nr = 34)
      g:echo = nr
    enddef
    var Funcref: func(?number) = function('EchoNr')
    Funcref()
    g:echo->assert_equal(34)
    Funcref(123)
    g:echo->assert_equal(123)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    vim9script
    def EchoList(...l: list<number>)
      g:echo = l
    enddef
    var Funcref: func(...list<number>) = function('EchoList')
    Funcref()
    g:echo->assert_equal([])
    Funcref(1, 2, 3)
    g:echo->assert_equal([1, 2, 3])
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    vim9script
    def OptAndVar(nr: number, opt = 12, ...l: list<number>): number
      g:optarg = opt
      g:listarg = l
      return nr
    enddef
    var Funcref: func(number, ?number, ...list<number>): number = function('OptAndVar')
    Funcref(10)->assert_equal(10)
    g:optarg->assert_equal(12)
    g:listarg->assert_equal([])

    Funcref(11, 22)->assert_equal(11)
    g:optarg->assert_equal(22)
    g:listarg->assert_equal([])

    Funcref(17, 18, 1, 2, 3)->assert_equal(17)
    g:optarg->assert_equal(18)
    g:listarg->assert_equal([1, 2, 3])
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    function s:func(num)
      return a:num * 2
    endfunction

    def s:CallFuncref()
      var Funcref = function('s:func')
      Funcref(3)->assert_equal(6)
    enddef
    call s:CallFuncref()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    function s:func(num)
      return a:num * 2
    endfunction

    def s:CallFuncref()
      var Funcref = function(s:func)
      Funcref(3)->assert_equal(6)
    enddef
    call s:CallFuncref()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
    function s:func(num)
      return a:num * 2
    endfunction

    def s:CallFuncref()
      var Funcref = s:func
      Funcref(3)->assert_equal(6)
    enddef
    call s:CallFuncref()
  END
  v9.CheckScriptSuccess(lines)
enddef

let SomeFunc = function('len')
let NotAFunc = 'text'

def CombineFuncrefTypes()
  # same arguments, different return type
  var Ref1: func(bool): string
  var Ref2: func(bool): number
  var Ref3: func(bool): any
  Ref3 = g:cond ? Ref1 : Ref2

  # different number of arguments
  var Refa1: func(bool): number
  var Refa2: func(bool, number): number
  var Refa3: func: number
  Refa3 = g:cond ? Refa1 : Refa2

  # different argument types
  var Refb1: func(bool, string): number
  var Refb2: func(string, number): number
  var Refb3: func(any, any): number
  Refb3 = g:cond ? Refb1 : Refb2
enddef

def FuncWithForwardCall()
  return g:DefinedEvenLater("yes")
enddef

def DefinedEvenLater(arg: string): string
  return arg
enddef

def Test_error_in_nested_function()
  # Error in called function requires unwinding the call stack.
  assert_fails('g:FuncWithForwardCall()', 'E1096:', '', 1, 'FuncWithForwardCall')
enddef

def Test_nested_function_with_nextcmd()
  var lines =<< trim END
      vim9script
      # Define an outer function
      def FirstFunction()
        # Define an inner function
        def SecondFunction()
          # the function has a body, a double free is detected.
          AAAAA

         # enddef followed by | or } followed by # one or more characters
         enddef|BBBB
      enddef

      # Compile all functions
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1173: Text found after enddef: BBBB')
enddef

def Test_nested_function_with_args_split()
  var lines =<< trim END
      vim9script
      def FirstFunction()
        def SecondFunction(
        )
        # had a double free if the right parenthesis of the nested function is
        # on the next line

        enddef|BBBB
      enddef
      # Compile all functions
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1173: Text found after enddef: BBBB')

  lines =<< trim END
      vim9script
      def FirstFunction()
        func SecondFunction()
        endfunc|BBBB
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1173: Text found after endfunction: BBBB')
enddef

def Test_error_in_function_args()
  var lines =<< trim END
      def FirstFunction()
        def SecondFunction(J  =
        # Nois
        # one

        enddef|BBBB
      enddef
      # Compile all functions
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E488:')
enddef

def Test_return_type_wrong()
  v9.CheckScriptFailure([
        'def Func(): number',
        'return "a"',
        'enddef',
        'defcompile'], 'expected number but got string')
  delfunc! g:Func
  v9.CheckScriptFailure([
        'def Func(): string',
        'return 1',
        'enddef',
        'defcompile'], 'expected string but got number')
  delfunc! g:Func
  v9.CheckScriptFailure([
        'def Func(): void',
        'return "a"',
        'enddef',
        'defcompile'],
        'E1096: Returning a value in a function without a return type')
  delfunc! g:Func
  v9.CheckScriptFailure([
        'def Func()',
        'return "a"',
        'enddef',
        'defcompile'],
        'E1096: Returning a value in a function without a return type')
  delfunc! g:Func

  v9.CheckScriptFailure([
        'def Func(): number',
        'return',
        'enddef',
        'defcompile'], 'E1003:')
  delfunc! g:Func

  v9.CheckScriptFailure([
        'def Func():number',
        'return 123',
        'enddef',
        'defcompile'], 'E1069:')
  delfunc! g:Func

  v9.CheckScriptFailure([
        'def Func() :number',
        'return 123',
        'enddef',
        'defcompile'], 'E1059:')
  delfunc! g:Func

  v9.CheckScriptFailure([
        'def Func() : number',
        'return 123',
        'enddef',
        'defcompile'], 'E1059:')
  delfunc! g:Func

  v9.CheckScriptFailure(['def Func(): list', 'return []', 'enddef'], 'E1008: Missing <type> after list')
  delfunc! g:Func
  v9.CheckScriptFailure(['def Func(): dict', 'return {}', 'enddef'], 'E1008: Missing <type> after dict')
  delfunc! g:Func
  v9.CheckScriptFailure(['def Func()', 'return 1'], 'E1057:')
  delfunc! g:Func

  v9.CheckScriptFailure([
        'vim9script',
        'def FuncB()',
        '  return 123',
        'enddef',
        'def FuncA()',
        '   FuncB()',
        'enddef',
        'defcompile'], 'E1096:')
enddef

def Test_arg_type_wrong()
  v9.CheckScriptFailure(['def Func3(items: list)', 'echo "a"', 'enddef'], 'E1008: Missing <type> after list')
  v9.CheckScriptFailure(['def Func4(...)', 'echo "a"', 'enddef'], 'E1055: Missing name after ...')
  v9.CheckScriptFailure(['def Func5(items:string)', 'echo "a"'], 'E1069:')
  v9.CheckScriptFailure(['def Func5(items)', 'echo "a"'], 'E1077:')
  v9.CheckScriptFailure(['def Func6(...x:list<number>)', 'echo "a"', 'enddef'], 'E1069:')
  v9.CheckScriptFailure(['def Func7(...x: int)', 'echo "a"', 'enddef'], 'E1010:')
enddef

def Test_white_space_before_comma()
  var lines =<< trim END
    vim9script
    def Func(a: number , b: number)
    enddef
  END
  v9.CheckScriptFailure(lines, 'E1068:')
  call assert_fails('vim9cmd echo stridx("a" .. "b" , "a")', 'E1068:')
enddef

def Test_white_space_after_comma()
  var lines =<< trim END
    vim9script
    def Func(a: number,b: number)
    enddef
  END
  v9.CheckScriptFailure(lines, 'E1069:')

  # OK in legacy function
  lines =<< trim END
    vim9script
    func Func(a,b)
    endfunc
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_vim9script_call()
  var lines =<< trim END
    vim9script
    var name = ''
    def MyFunc(arg: string)
       name = arg
    enddef
    MyFunc('foobar')
    name->assert_equal('foobar')

    var str = 'barfoo'
    str->MyFunc()
    name->assert_equal('barfoo')

    g:value = 'value'
    g:value->MyFunc()
    name->assert_equal('value')

    var listvar = []
    def ListFunc(arg: list<number>)
       listvar = arg
    enddef
    [1, 2, 3]->ListFunc()
    listvar->assert_equal([1, 2, 3])

    var dictvar = {}
    def DictFunc(arg: dict<number>)
       dictvar = arg
    enddef
    {a: 1, b: 2}->DictFunc()
    dictvar->assert_equal({a: 1, b: 2})
    def CompiledDict()
      {a: 3, b: 4}->DictFunc()
    enddef
    CompiledDict()
    dictvar->assert_equal({a: 3, b: 4})

    {a: 3, b: 4}->DictFunc()
    dictvar->assert_equal({a: 3, b: 4})

    ('text')->MyFunc()
    name->assert_equal('text')
    ("some")->MyFunc()
    name->assert_equal('some')

    # line starting with single quote is not a mark
    # line starting with double quote can be a method call
    'asdfasdf'->MyFunc()
    name->assert_equal('asdfasdf')
    "xyz"->MyFunc()
    name->assert_equal('xyz')

    def UseString()
      'xyork'->MyFunc()
    enddef
    UseString()
    name->assert_equal('xyork')

    def UseString2()
      "knife"->MyFunc()
    enddef
    UseString2()
    name->assert_equal('knife')

    # prepending a colon makes it a mark
    new
    setline(1, ['aaa', 'bbb', 'ccc'])
    normal! 3Gmt1G
    :'t
    getcurpos()[1]->assert_equal(3)
    bwipe!

    MyFunc(
        'continued'
        )
    assert_equal('continued',
            name
            )

    call MyFunc(
        'more'
          ..
          'lines'
        )
    assert_equal(
        'morelines',
        name)
  END
  writefile(lines, 'Xcall.vim', 'D')
  source Xcall.vim
enddef

def Test_vim9script_call_fail_decl()
  var lines =<< trim END
    vim9script
    var name = ''
    def MyFunc(arg: string)
       var name = 123
    enddef
    defcompile
  END
  v9.CheckScriptFailure(lines, 'E1054:')
enddef

def Test_vim9script_call_fail_type()
  var lines =<< trim END
    vim9script
    def MyFunc(arg: string)
      echo arg
    enddef
    MyFunc(1234)
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch, expected string but got number')
enddef

def Test_vim9script_call_fail_const()
  var lines =<< trim END
    vim9script
    const var = ''
    def MyFunc(arg: string)
       var = 'asdf'
    enddef
    defcompile
  END
  writefile(lines, 'Xcall_const.vim', 'D')
  assert_fails('source Xcall_const.vim', 'E46:', '', 1, 'MyFunc')

  lines =<< trim END
      const g:Aconst = 77
      def Change()
        # comment
        g:Aconst = 99
      enddef
      call Change()
      unlet g:Aconst
  END
  v9.CheckScriptFailure(lines, 'E741: Value is locked: Aconst', 2)
enddef

" Test that inside :function a Python function can be defined, :def is not
" recognized.
func Test_function_python()
  CheckFeature python3
  let py = 'python3'
  execute py "<< EOF"
def do_something():
  return 1
EOF
endfunc

def Test_delfunc()
  var lines =<< trim END
    vim9script
    def g:GoneSoon()
      echo 'hello'
    enddef

    def CallGoneSoon()
      g:GoneSoon()
    enddef
    defcompile

    delfunc g:GoneSoon
    CallGoneSoon()
  END
  writefile(lines, 'XToDelFunc', 'D')
  assert_fails('so XToDelFunc', 'E933:', '', 1, 'CallGoneSoon')
  assert_fails('so XToDelFunc', 'E933:', '', 1, 'CallGoneSoon')
enddef

func Test_free_dict_while_in_funcstack()
  " relies on the sleep command
  CheckUnix
  call Run_Test_free_dict_while_in_funcstack()
endfunc

def Run_Test_free_dict_while_in_funcstack()
  # this was freeing the TermRun() default argument dictionary while it was
  # still referenced in a funcstack_T
  var lines =<< trim END
      vim9script

      &updatetime = 400
      def TermRun(_ = {})
          def Post()
          enddef
          def Exec()
              term_start('sleep 1', {
                  term_finish: 'close',
                  exit_cb: (_, _) => Post(),
              })
          enddef
          Exec()
      enddef
      nnoremap <F4> <Cmd>call <SID>TermRun()<CR>
      timer_start(100, (_) => feedkeys("\<F4>"))
      timer_start(1000, (_) => feedkeys("\<F4>"))
      sleep 1500m
  END
  v9.CheckScriptSuccess(lines)
  nunmap <F4>
  set updatetime&
enddef

def Test_redef_failure()
  writefile(['def Func0(): string',  'return "Func0"', 'enddef'], 'Xdef')
  so Xdef
  writefile(['def Func1(): string',  'return "Func1"', 'enddef'], 'Xdef')
  so Xdef
  writefile(['def! Func0(): string', 'enddef', 'defcompile'], 'Xdef')
  assert_fails('so Xdef', 'E1027:', '', 1, 'Func0')
  writefile(['def Func2(): string',  'return "Func2"', 'enddef'], 'Xdef')
  so Xdef
  delete('Xdef')

  assert_fails('g:Func0()', 'E1091:')
  g:Func1()->assert_equal('Func1')
  g:Func2()->assert_equal('Func2')

  delfunc! Func0
  delfunc! Func1
  delfunc! Func2
enddef

def Test_vim9script_func()
  var lines =<< trim END
    vim9script
    func Func(arg)
      echo a:arg
    endfunc
    Func('text')
  END
  writefile(lines, 'XVim9Func', 'D')
  so XVim9Func
enddef

let s:funcResult = 0

def FuncNoArgNoRet()
  s:funcResult = 11
enddef

def FuncNoArgRetNumber(): number
  s:funcResult = 22
  return 1234
enddef

def FuncNoArgRetString(): string
  s:funcResult = 45
  return 'text'
enddef

def FuncOneArgNoRet(arg: number)
  s:funcResult = arg
enddef

def FuncOneArgRetNumber(arg: number): number
  s:funcResult = arg
  return arg
enddef

def FuncTwoArgNoRet(one: bool, two: number)
  s:funcResult = two
enddef

def s:FuncOneArgRetString(arg: string): string
  return arg
enddef

def s:FuncOneArgRetAny(arg: any): any
  return arg
enddef

def Test_func_type()
  var Ref1: func()
  s:funcResult = 0
  Ref1 = g:FuncNoArgNoRet
  Ref1()
  s:funcResult->assert_equal(11)

  var Ref2: func
  s:funcResult = 0
  Ref2 = g:FuncNoArgNoRet
  Ref2()
  s:funcResult->assert_equal(11)

  s:funcResult = 0
  Ref2 = g:FuncOneArgNoRet
  Ref2(12)
  s:funcResult->assert_equal(12)

  s:funcResult = 0
  Ref2 = g:FuncNoArgRetNumber
  Ref2()->assert_equal(1234)
  s:funcResult->assert_equal(22)

  s:funcResult = 0
  Ref2 = g:FuncOneArgRetNumber
  Ref2(13)->assert_equal(13)
  s:funcResult->assert_equal(13)
enddef

def Test_repeat_return_type()
  var res = 0
  for n in repeat([1], 3)
    res += n
  endfor
  res->assert_equal(3)

  res = 0
  for n in repeat(0z01, 3)->blob2list()
    res += n
  endfor
  res->assert_equal(3)

  res = 0
  for n in add([1, 2], 3)
    res += n
  endfor
  res->assert_equal(6)
enddef

def Test_argv_return_type()
  next fileone filetwo
  var res = ''
  for name in argv()
    res ..= name
  endfor
  res->assert_equal('fileonefiletwo')
enddef

def Test_func_type_part()
  var RefVoid: func: void
  RefVoid = g:FuncNoArgNoRet
  RefVoid = g:FuncOneArgNoRet
  v9.CheckDefFailure(['var RefVoid: func: void', 'RefVoid = g:FuncNoArgRetNumber'], 'E1012: Type mismatch; expected func(...) but got func(): number')
  v9.CheckDefFailure(['var RefVoid: func: void', 'RefVoid = g:FuncNoArgRetString'], 'E1012: Type mismatch; expected func(...) but got func(): string')

  var RefAny: func(): any
  RefAny = g:FuncNoArgRetNumber
  RefAny = g:FuncNoArgRetString
  v9.CheckDefFailure(['var RefAny: func(): any', 'RefAny = g:FuncNoArgNoRet'], 'E1012: Type mismatch; expected func(): any but got func()')
  v9.CheckDefFailure(['var RefAny: func(): any', 'RefAny = g:FuncOneArgNoRet'], 'E1012: Type mismatch; expected func(): any but got func(number)')

  var RefAnyNoArgs: func: any = RefAny

  var RefNr: func: number
  RefNr = g:FuncNoArgRetNumber
  RefNr = g:FuncOneArgRetNumber
  v9.CheckDefFailure(['var RefNr: func: number', 'RefNr = g:FuncNoArgNoRet'], 'E1012: Type mismatch; expected func(...): number but got func()')
  v9.CheckDefFailure(['var RefNr: func: number', 'RefNr = g:FuncNoArgRetString'], 'E1012: Type mismatch; expected func(...): number but got func(): string')

  var RefStr: func: string
  RefStr = g:FuncNoArgRetString
  RefStr = FuncOneArgRetString
  v9.CheckDefFailure(['var RefStr: func: string', 'RefStr = g:FuncNoArgNoRet'], 'E1012: Type mismatch; expected func(...): string but got func()')
  v9.CheckDefFailure(['var RefStr: func: string', 'RefStr = g:FuncNoArgRetNumber'], 'E1012: Type mismatch; expected func(...): string but got func(): number')
enddef

def Test_func_type_fails()
  v9.CheckDefFailure(['var ref1: func()'], 'E704:')

  v9.CheckDefFailure(['var Ref1: func()', 'Ref1 = g:FuncNoArgRetNumber'], 'E1012: Type mismatch; expected func() but got func(): number')
  v9.CheckDefFailure(['var Ref1: func()', 'Ref1 = g:FuncOneArgNoRet'], 'E1012: Type mismatch; expected func() but got func(number)')
  v9.CheckDefFailure(['var Ref1: func()', 'Ref1 = g:FuncOneArgRetNumber'], 'E1012: Type mismatch; expected func() but got func(number): number')
  v9.CheckDefFailure(['var Ref1: func(bool)', 'Ref1 = g:FuncTwoArgNoRet'], 'E1012: Type mismatch; expected func(bool) but got func(bool, number)')
  v9.CheckDefFailure(['var Ref1: func(?bool)', 'Ref1 = g:FuncTwoArgNoRet'], 'E1012: Type mismatch; expected func(?bool) but got func(bool, number)')
  v9.CheckDefFailure(['var Ref1: func(...bool)', 'Ref1 = g:FuncTwoArgNoRet'], 'E1180: Variable arguments type must be a list: bool')

  v9.CheckDefFailure(['var RefWrong: func(string ,number)'], 'E1068:')
  v9.CheckDefFailure(['var RefWrong: func(string,number)'], 'E1069:')
  v9.CheckDefFailure(['var RefWrong: func(bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool)'], 'E1005:')
  v9.CheckDefFailure(['var RefWrong: func(bool):string'], 'E1069:')
enddef

def Test_func_return_type()
  var nr: number
  nr = g:FuncNoArgRetNumber()
  nr->assert_equal(1234)

  nr = FuncOneArgRetAny(122)
  nr->assert_equal(122)

  var str: string
  str = FuncOneArgRetAny('yes')
  str->assert_equal('yes')

  v9.CheckDefFailure(['var str: string', 'str = g:FuncNoArgRetNumber()'], 'E1012: Type mismatch; expected string but got number')
enddef

def Test_func_common_type()
  def FuncOne(n: number): number
    return n
  enddef
  def FuncTwo(s: string): number
    return len(s)
  enddef
  def FuncThree(n: number, s: string): number
    return n + len(s)
  enddef
  var list = [FuncOne, FuncTwo, FuncThree]
  assert_equal(8, list[0](8))
  assert_equal(4, list[1]('word'))
  assert_equal(7, list[2](3, 'word'))
enddef

def s:MultiLine(
    arg1: string,
    arg2 = 1234,
    ...rest: list<string>
      ): string
  return arg1 .. arg2 .. join(rest, '-')
enddef

def MultiLineComment(
    arg1: string, # comment
    arg2 = 1234, # comment
    ...rest: list<string> # comment
      ): string # comment
  return arg1 .. arg2 .. join(rest, '-')
enddef

def Test_multiline()
  MultiLine('text')->assert_equal('text1234')
  MultiLine('text', 777)->assert_equal('text777')
  MultiLine('text', 777, 'one')->assert_equal('text777one')
  MultiLine('text', 777, 'one', 'two')->assert_equal('text777one-two')
enddef

func Test_multiline_not_vim9()
  call s:MultiLine('text')->assert_equal('text1234')
  call s:MultiLine('text', 777)->assert_equal('text777')
  call s:MultiLine('text', 777, 'one')->assert_equal('text777one')
  call s:MultiLine('text', 777, 'one', 'two')->assert_equal('text777one-two')
endfunc


" When using CheckScriptFailure() for the below test, E1010 is generated instead
" of E1056.
func Test_E1056_1059()
  let caught_1056 = 0
  try
    def F():
      return 1
    enddef
  catch /E1056:/
    let caught_1056 = 1
  endtry
  eval caught_1056->assert_equal(1)

  let caught_1059 = 0
  try
    def F5(items : list)
      echo 'a'
    enddef
  catch /E1059:/
    let caught_1059 = 1
  endtry
  eval caught_1059->assert_equal(1)
endfunc

func DelMe()
  echo 'DelMe'
endfunc

def Test_error_reporting()
  # comment lines at the start of the function
  var lines =<< trim END
    " comment
    def Func()
      # comment
      # comment
      invalid
    enddef
    defcompile
  END
  writefile(lines, 'Xdef', 'D')
  try
    source Xdef
    assert_report('should have failed')
  catch /E476:/
    v:exception->assert_match('Invalid command: invalid')
    v:throwpoint->assert_match(', line 3$')
  endtry
  delfunc! g:Func

  # comment lines after the start of the function
  lines =<< trim END
    " comment
    def Func()
      var x = 1234
      # comment
      # comment
      invalid
    enddef
    defcompile
  END
  writefile(lines, 'Xdef')
  try
    source Xdef
    assert_report('should have failed')
  catch /E476:/
    v:exception->assert_match('Invalid command: invalid')
    v:throwpoint->assert_match(', line 4$')
  endtry
  delfunc! g:Func

  lines =<< trim END
    vim9script
    def Func()
      var db = {foo: 1, bar: 2}
      # comment
      var x = db.asdf
    enddef
    defcompile
    Func()
  END
  writefile(lines, 'Xdef')
  try
    source Xdef
    assert_report('should have failed')
  catch /E716:/
    v:throwpoint->assert_match('_Func, line 3$')
  endtry
  delfunc! g:Func
enddef

def Test_deleted_function()
  v9.CheckDefExecFailure([
      'var RefMe: func = function("g:DelMe")',
      'delfunc g:DelMe',
      'echo RefMe()'], 'E117:')
enddef

def Test_unknown_function()
  v9.CheckDefExecFailure([
      'var Ref: func = function("NotExist")',
      'delfunc g:NotExist'], 'E700:')
enddef

def s:RefFunc(Ref: func(any): any): string
  return Ref('more')
enddef

def Test_closure_simple()
  var local = 'some '
  RefFunc((s) => local .. s)->assert_equal('some more')
enddef

def s:MakeRef()
  var local = 'some '
  g:Ref = (s) => local .. s
enddef

def Test_closure_ref_after_return()
  MakeRef()
  g:Ref('thing')->assert_equal('some thing')
  unlet g:Ref
enddef

def s:MakeTwoRefs()
  var local = ['some']
  g:Extend = (s) => local->add(s)
  g:Read = () => local
enddef

def Test_closure_two_refs()
  MakeTwoRefs()
  join(g:Read(), ' ')->assert_equal('some')
  g:Extend('more')
  join(g:Read(), ' ')->assert_equal('some more')
  g:Extend('even')
  join(g:Read(), ' ')->assert_equal('some more even')

  unlet g:Extend
  unlet g:Read
enddef

def s:ReadRef(Ref: func(): list<string>): string
  return join(Ref(), ' ')
enddef

def s:ExtendRef(Ref: func(string): list<string>, add: string)
  Ref(add)
enddef

def Test_closure_two_indirect_refs()
  MakeTwoRefs()
  ReadRef(g:Read)->assert_equal('some')
  ExtendRef(g:Extend, 'more')
  ReadRef(g:Read)->assert_equal('some more')
  ExtendRef(g:Extend, 'even')
  ReadRef(g:Read)->assert_equal('some more even')

  unlet g:Extend
  unlet g:Read
enddef

def s:MakeArgRefs(theArg: string)
  var local = 'loc_val'
  g:UseArg = (s) => theArg .. '/' .. local .. '/' .. s
enddef

def s:MakeArgRefsVarargs(theArg: string, ...rest: list<string>)
  var local = 'the_loc'
  g:UseVararg = (s) => theArg .. '/' .. local .. '/' .. s .. '/' .. join(rest)
enddef

def Test_closure_using_argument()
  MakeArgRefs('arg_val')
  g:UseArg('call_val')->assert_equal('arg_val/loc_val/call_val')

  MakeArgRefsVarargs('arg_val', 'one', 'two')
  g:UseVararg('call_val')->assert_equal('arg_val/the_loc/call_val/one two')

  unlet g:UseArg
  unlet g:UseVararg

  var lines =<< trim END
      vim9script
      def Test(Fun: func(number): number): list<number>
        return map([1, 2, 3], (_, i) => Fun(i))
      enddef
      def Inc(nr: number): number
        return nr + 2
      enddef
      assert_equal([3, 4, 5], Test(Inc))
  END
  v9.CheckScriptSuccess(lines)
enddef

def s:MakeGetAndAppendRefs()
  var local = 'a'

  def Append(arg: string)
    local ..= arg
  enddef
  g:Append = Append

  def Get(): string
    return local
  enddef
  g:Get = Get
enddef

def Test_closure_append_get()
  MakeGetAndAppendRefs()
  g:Get()->assert_equal('a')
  g:Append('-b')
  g:Get()->assert_equal('a-b')
  g:Append('-c')
  g:Get()->assert_equal('a-b-c')

  unlet g:Append
  unlet g:Get
enddef

def Test_nested_closure()
  var local = 'text'
  def Closure(arg: string): string
    return local .. arg
  enddef
  Closure('!!!')->assert_equal('text!!!')
enddef

func s:GetResult(Ref)
  return a:Ref('some')
endfunc

def Test_call_closure_not_compiled()
  var text = 'text'
  g:Ref = (s) =>  s .. text
  GetResult(g:Ref)->assert_equal('sometext')
enddef

def Test_double_closure_fails()
  var lines =<< trim END
    vim9script
    def Func()
      var name = 0
      for i in range(2)
          timer_start(0, () => name)
      endfor
    enddef
    Func()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_nested_closure_used()
  var lines =<< trim END
      vim9script
      def Func()
        var x = 'hello'
        var Closure = () => x
        g:Myclosure = () => Closure()
      enddef
      Func()
      assert_equal('hello', g:Myclosure())
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_nested_closure_fails()
  var lines =<< trim END
    vim9script
    def FuncA()
      FuncB(0)
    enddef
    def FuncB(n: number): list<string>
      return map([0], (_, v) => n)
    enddef
    FuncA()
  END
  v9.CheckScriptFailure(lines, 'E1012:')
enddef

def Run_Test_closure_in_for_loop_fails()
  var lines =<< trim END
    vim9script
    redraw
    for n in [0]
        # time should be enough for startup to finish
        timer_start(200, (_) => {
            echo n
        })
    endfor
  END
  writefile(lines, 'XTest_closure_fails', 'D')

  # Check that an error shows
  var buf = g:RunVimInTerminal('-S XTest_closure_fails', {rows: 6, wait_for_ruler: 0})
  g:VerifyScreenDump(buf, 'Test_vim9_closure_fails', {wait: 3000})

  # clean up
  g:StopVimInTerminal(buf)
enddef

func Test_closure_in_for_loop_fails()
  CheckScreendump
  call Run_Test_closure_in_for_loop_fails()
endfunc

def Test_global_closure()
  var lines =<< trim END
      vim9script
      def ReverseEveryNLines(n: number, line1: number, line2: number)
        var mods = 'sil keepj keepp lockm '
        var range = ':' .. line1 .. ',' .. line2
        def g:Offset(): number
            var offset = (line('.') - line1 + 1) % n
            return offset != 0 ? offset : n
        enddef
        exe mods .. range .. 'g/^/exe "m .-" .. g:Offset()'
      enddef

      new
      repeat(['aaa', 'bbb', 'ccc'], 3)->setline(1)
      ReverseEveryNLines(3, 1, 9)
  END
  v9.CheckScriptSuccess(lines)
  var expected = repeat(['ccc', 'bbb', 'aaa'], 3)
  assert_equal(expected, getline(1, 9))
  bwipe!
enddef

def Test_global_closure_called_directly()
  var lines =<< trim END
      vim9script
      def Outer()
        var x = 1
        def g:Inner()
          var y = x
          x += 1
          assert_equal(1, y)
        enddef
        g:Inner()
        assert_equal(2, x)
      enddef
      Outer()
  END
  v9.CheckScriptSuccess(lines)
  delfunc g:Inner
enddef

def Test_closure_called_from_legacy()
  var lines =<< trim END
      vim9script
      def Func()
        var outer = 'foo'
        var F = () => {
              outer = 'bar'
            }
        execute printf('call %s()', string(F))
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E1248')
enddef

def Test_failure_in_called_function()
  # this was using the frame index as the return value
  var lines =<< trim END
      vim9script
      au TerminalWinOpen * eval [][0]
      def PopupTerm(a: any)
        # make sure typvals on stack are string
        ['a', 'b', 'c', 'd', 'e', 'f', 'g']->join()
        FireEvent()
      enddef
      def FireEvent()
          do TerminalWinOpen
      enddef
      # use try/catch to make eval fail
      try
          call PopupTerm(0)
      catch
      endtry
      au! TerminalWinOpen
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_nested_lambda()
  var lines =<< trim END
    vim9script
    def Func()
      var x = 4
      var Lambda1 = () => 7
      var Lambda2 = () => [Lambda1(), x]
      var res = Lambda2()
      assert_equal([7, 4], res)
    enddef
    Func()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_double_nested_lambda()
  var lines =<< trim END
      vim9script
      def F(head: string): func(string): func(string): string
        return (sep: string): func(string): string => ((tail: string): string => {
            return head .. sep .. tail
          })
      enddef
      assert_equal('hello-there', F('hello')('-')('there'))
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_nested_inline_lambda()
  var lines =<< trim END
      vim9script
      def F(text: string): func(string): func(string): string
        return (arg: string): func(string): string => ((sep: string): string => {
            return sep .. arg .. text
          })
      enddef
      assert_equal('--there++', F('++')('there')('--'))
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      echo range(4)->mapnew((_, v) => {
        return range(v) ->mapnew((_, s) => {
          return string(s)
          })
        })
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script

      def Func()
        range(10)
          ->mapnew((_, _) => ({
            key: range(10)->mapnew((_, _) => {
              return ' '
            }),
          }))
      enddef

      defcomp
  END
  v9.CheckScriptSuccess(lines)
enddef

def Shadowed(): list<number>
  var FuncList: list<func: number> = [() => 42]
  return FuncList->mapnew((_, Shadowed) => Shadowed())
enddef

def Test_lambda_arg_shadows_func()
  assert_equal([42], g:Shadowed())
enddef

def Test_compiling_referenced_func_no_shadow()
  var lines =<< trim END
      vim9script

      def InitializeReply(lspserver: dict<any>)
      enddef

      def ProcessReply(lspserver: dict<any>)
        var lsp_reply_handlers: dict<func> =
          { 'initialize': InitializeReply }
        lsp_reply_handlers['initialize'](lspserver)
      enddef

      call ProcessReply({})
  END
  v9.CheckScriptSuccess(lines)
enddef

def s:Line_continuation_in_def(dir: string = ''): string
  var path: string = empty(dir)
          \ ? 'empty'
          \ : 'full'
  return path
enddef

def Test_line_continuation_in_def()
  Line_continuation_in_def('.')->assert_equal('full')
enddef

def Test_script_var_in_lambda()
  var lines =<< trim END
      vim9script
      var script = 'test'
      assert_equal(['test'], map(['one'], (_, _) => script))
  END
  v9.CheckScriptSuccess(lines)
enddef

def s:Line_continuation_in_lambda(): list<string>
  var x = range(97, 100)
      ->mapnew((_, v) => nr2char(v)
          ->toupper())
      ->reverse()
  return x
enddef

def Test_line_continuation_in_lambda()
  Line_continuation_in_lambda()->assert_equal(['D', 'C', 'B', 'A'])

  var lines =<< trim END
      vim9script
      var res = [{n: 1, m: 2, s: 'xxx'}]
                ->mapnew((_, v: dict<any>): string => printf('%d:%d:%s',
                    v.n,
                    v.m,
                    substitute(v.s, '.*', 'yyy', '')
                    ))
      assert_equal(['1:2:yyy'], res)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_list_lambda()
  timer_start(1000, (_) => 0)
  var body = execute(timer_info()[0].callback
         ->string()
         ->substitute("('", ' ', '')
         ->substitute("')", '', '')
         ->substitute('function\zs', ' ', ''))
  assert_match('def <lambda>\d\+(_: any): number\n1  return 0\n   enddef', body)
enddef

def Test_lambda_block_variable()
  var lines =<< trim END
      vim9script
      var flist: list<func>
      for i in range(10)
          var inloop = i
          flist[i] = () => inloop
      endfor
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      if true
        var outloop = 5
        var flist: list<func>
        for i in range(10)
          flist[i] = () => outloop
        endfor
      endif
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      if true
        var outloop = 5
      endif
      var flist: list<func>
      for i in range(10)
        flist[i] = () => outloop
      endfor
  END
  v9.CheckScriptFailure(lines, 'E1001: Variable not found: outloop', 1)

  lines =<< trim END
      vim9script
      for i in range(10)
        var Ref = () => 0
      endfor
      assert_equal(0, ((i) => 0)(0))
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_legacy_lambda()
  legacy echo {x -> 'hello ' .. x}('foo')

  var lines =<< trim END
      echo {x -> 'hello ' .. x}('foo')
  END
  v9.CheckDefAndScriptFailure(lines, 'E720:')

  lines =<< trim END
      vim9script
      def Func()
        echo (() => 'no error')()
      enddef
      legacy call s:Func()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_legacy()
  var lines =<< trim END
      vim9script
      func g:LegacyFunction()
        let g:legacyvar = 1
      endfunc
      def Testit()
        legacy call g:LegacyFunction()
      enddef
      Testit()
      assert_equal(1, g:legacyvar)
      unlet g:legacyvar
      delfunc g:LegacyFunction
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_legacy_errors()
  for cmd in ['if', 'elseif', 'else', 'endif',
              'for', 'endfor', 'continue', 'break',
              'while', 'endwhile',
              'try', 'catch', 'finally', 'endtry']
    v9.CheckDefFailure(['legacy ' .. cmd .. ' expr'], 'E1189:')
  endfor
enddef

def Test_call_legacy_with_dict()
  var lines =<< trim END
      vim9script
      func Legacy() dict
        let g:result = self.value
      endfunc
      def TestDirect()
        var d = {value: 'yes', func: Legacy}
        d.func()
      enddef
      TestDirect()
      assert_equal('yes', g:result)
      unlet g:result

      def TestIndirect()
        var d = {value: 'foo', func: Legacy}
        var Fi = d.func
        Fi()
      enddef
      TestIndirect()
      assert_equal('foo', g:result)
      unlet g:result

      var d = {value: 'bar', func: Legacy}
      d.func()
      assert_equal('bar', g:result)
      unlet g:result
  END
  v9.CheckScriptSuccess(lines)
enddef

def s:DoFilterThis(a: string): list<string>
  # closure nested inside another closure using argument
  var Filter = (l) => filter(l, (_, v) => stridx(v, a) == 0)
  return ['x', 'y', 'a', 'x2', 'c']->Filter()
enddef

def Test_nested_closure_using_argument()
  assert_equal(['x', 'x2'], DoFilterThis('x'))
enddef

def Test_triple_nested_closure()
  var what = 'x'
  var Match = (val: string, cmp: string): bool => stridx(val, cmp) == 0
  var Filter = (l) => filter(l, (_, v) => Match(v, what))
  assert_equal(['x', 'x2'], ['x', 'y', 'a', 'x2', 'c']->Filter())
enddef

func Test_silent_echo()
  CheckScreendump
  call Run_Test_silent_echo()
endfunc

def Run_Test_silent_echo()
  var lines =<< trim END
    vim9script
    def EchoNothing()
      silent echo ''
    enddef
    defcompile
  END
  writefile(lines, 'XTest_silent_echo', 'D')

  # Check that the balloon shows up after a mouse move
  var buf = g:RunVimInTerminal('-S XTest_silent_echo', {'rows': 6})
  term_sendkeys(buf, ":abc")
  g:VerifyScreenDump(buf, 'Test_vim9_silent_echo', {})

  # clean up
  g:StopVimInTerminal(buf)
enddef

def SilentlyError()
  execute('silent! invalid')
  g:did_it = 'yes'
enddef

func s:UserError()
  silent! invalid
endfunc

def SilentlyUserError()
  UserError()
  g:did_it = 'yes'
enddef

" This can't be a :def function, because the assert would not be reached.
func Test_ignore_silent_error()
  let g:did_it = 'no'
  call SilentlyError()
  call assert_equal('yes', g:did_it)

  let g:did_it = 'no'
  call SilentlyUserError()
  call assert_equal('yes', g:did_it)

  unlet g:did_it
endfunc

def Test_ignore_silent_error_in_filter()
  var lines =<< trim END
      vim9script
      def Filter(winid: number, key: string): bool
          if key == 'o'
              silent! eval [][0]
              return true
          endif
          return popup_filter_menu(winid, key)
      enddef

      popup_create('popup', {filter: Filter})
      feedkeys("o\r", 'xnt')
  END
  v9.CheckScriptSuccess(lines)
enddef

def s:Fibonacci(n: number): number
  if n < 2
    return n
  else
    return Fibonacci(n - 1) + Fibonacci(n - 2)
  endif
enddef

def Test_recursive_call()
  Fibonacci(20)->assert_equal(6765)
enddef

def s:TreeWalk(dir: string): list<any>
  return readdir(dir)->mapnew((_, val) =>
            fnamemodify(dir .. '/' .. val, ':p')->isdirectory()
               ? {[val]: TreeWalk(dir .. '/' .. val)}
               : val
             )
enddef

def Test_closure_in_map()
  mkdir('XclosureDir/tdir', 'pR')
  writefile(['111'], 'XclosureDir/file1')
  writefile(['222'], 'XclosureDir/file2')
  writefile(['333'], 'XclosureDir/tdir/file3')

  TreeWalk('XclosureDir')->assert_equal(['file1', 'file2', {tdir: ['file3']}])
enddef

def Test_invalid_function_name()
  var lines =<< trim END
      vim9script
      def s: list<string>
  END
  v9.CheckScriptFailure(lines, 'E1268:')

  lines =<< trim END
      vim9script
      def g: list<string>
  END
  v9.CheckScriptFailure(lines, 'E129:')

  lines =<< trim END
      vim9script
      def <SID>: list<string>
  END
  v9.CheckScriptFailure(lines, 'E884:')

  lines =<< trim END
      vim9script
      def F list<string>
  END
  v9.CheckScriptFailure(lines, 'E488:')
enddef

def Test_partial_call()
  var lines =<< trim END
      var Xsetlist: func
      Xsetlist = function('setloclist', [0])
      Xsetlist([], ' ', {title: 'test'})
      getloclist(0, {title: 1})->assert_equal({title: 'test'})

      Xsetlist = function('setloclist', [0, [], ' '])
      Xsetlist({title: 'test'})
      getloclist(0, {title: 1})->assert_equal({title: 'test'})

      Xsetlist = function('setqflist')
      Xsetlist([], ' ', {title: 'test'})
      getqflist({title: 1})->assert_equal({title: 'test'})

      Xsetlist = function('setqflist', [[], ' '])
      Xsetlist({title: 'test'})
      getqflist({title: 1})->assert_equal({title: 'test'})

      var Len: func: number = function('len', ['word'])
      assert_equal(4, Len())

      var RepeatFunc = function('repeat', ['o'])
      assert_equal('ooooo', RepeatFunc(5))
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def Foo(Parser: any)
      enddef
      var Expr: func(dict<any>): dict<any>
      const Call = Foo(Expr)
  END
  v9.CheckScriptFailure(lines, 'E1031:')

  # Test for calling a partial that takes a single argument.
  # This used to produce a "E340: Internal error" message.
  lines =<< trim END
      def Foo(n: number): number
        return n * 2
      enddef
      var Fn = function(Foo, [10])
      assert_equal(20, Fn())
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_partial_double_nested()
  var idx = 123
  var Get = () => idx
  var Ref = function(Get, [])
  var RefRef = function(Ref, [])
  assert_equal(123, RefRef())
enddef

def Test_partial_null_function()
  var lines =<< trim END
      var d: dict<func> = {f: null_function}
      var Ref = d.f
      assert_equal('func(...): unknown', typename(Ref))
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_cmd_modifier()
  tab echo '0'
  v9.CheckDefFailure(['5tab echo 3'], 'E16:')
enddef

def Test_restore_modifiers()
  # check that when compiling a :def function command modifiers are not messed
  # up.
  var lines =<< trim END
      vim9script
      set eventignore=
      autocmd QuickFixCmdPost * copen
      def AutocmdsDisabled()
        eval 1 + 2
      enddef
      func Func()
        noautocmd call s:AutocmdsDisabled()
        let g:ei_after = &eventignore
      endfunc
      Func()
  END
  v9.CheckScriptSuccess(lines)
  g:ei_after->assert_equal('')
enddef

def StackTop()
  eval 1 + 2
  eval 2 + 3
  # call not on fourth line
  g:StackBot()
enddef

def StackBot()
  # throw an error
  eval [][0]
enddef

def Test_callstack_def()
  try
    g:StackTop()
  catch
    v:throwpoint->assert_match('Test_callstack_def\[2\]..StackTop\[4\]..StackBot, line 2')
  endtry
enddef

" Re-using spot for variable used in block
def Test_block_scoped_var()
  var lines =<< trim END
      vim9script
      def Func()
        var x = ['a', 'b', 'c']
        if 1
          var y = 'x'
          map(x, (_, _) => y)
        endif
        var z = x
        assert_equal(['x', 'x', 'x'], z)
      enddef
      Func()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_reset_did_emsg()
  var lines =<< trim END
      @s = 'blah'
      au BufWinLeave * #
      def Func()
        var winid = popup_create('popup', {})
        exe '*s'
        popup_close(winid)
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E492:', 8)
  delfunc! g:Func
enddef

def Test_did_emsg_reset()
  # executing an autocommand resets did_emsg, this should not result in a
  # builtin function considered failing
  var lines =<< trim END
      vim9script
      au BufWinLeave * #
      def Func()
          popup_menu('', {callback: (a, b) => popup_create('', {})->popup_close()})
          eval [][0]
      enddef
      nno <F3> <cmd>call <sid>Func()<cr>
      feedkeys("\<F3>\e", 'xt')
  END
  writefile(lines, 'XemsgReset', 'D')
  assert_fails('so XemsgReset', ['E684:', 'E684:'], lines, 2)

  nunmap <F3>
  au! BufWinLeave
enddef

def Test_abort_with_silent_call()
  var lines =<< trim END
      vim9script
      g:result = 'none'
      def Func()
        g:result += 3
        g:result = 'yes'
      enddef
      # error is silenced, but function aborts on error
      silent! Func()
      assert_equal('none', g:result)
      unlet g:result
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_continues_with_silent_error()
  var lines =<< trim END
      vim9script
      g:result = 'none'
      def Func()
        silent!  g:result += 3
        g:result = 'yes'
      enddef
      # error is silenced, function does not abort
      Func()
      assert_equal('yes', g:result)
      unlet g:result
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_abort_even_with_silent()
  var lines =<< trim END
      vim9script
      g:result = 'none'
      def Func()
        eval {-> ''}() .. '' .. {}['X']
        g:result = 'yes'
      enddef
      silent! Func()
      assert_equal('none', g:result)
      unlet g:result
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_cmdmod_silent_restored()
  var lines =<< trim END
      vim9script
      def Func()
        g:result = 'none'
        silent! g:result += 3
        g:result = 'none'
        g:result += 3
      enddef
      Func()
  END
  # can't use CheckScriptFailure, it ignores the :silent!
  var fname = 'Xdefsilent'
  writefile(lines, fname, 'D')
  var caught = 'no'
  try
    exe 'source ' .. fname
  catch /E1030:/
    caught = 'yes'
    assert_match('Func, line 4', v:throwpoint)
  endtry
  assert_equal('yes', caught)
enddef

def Test_cmdmod_silent_nested()
  var lines =<< trim END
      vim9script
      var result = ''

      def Error()
          result ..= 'Eb'
          eval [][0]
          result ..= 'Ea'
      enddef

      def Crash()
          result ..= 'Cb'
          sil! Error()
          result ..= 'Ca'
      enddef

      Crash()
      assert_equal('CbEbEaCa', result)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_dict_member_with_silent()
  var lines =<< trim END
      vim9script
      g:result = 'none'
      var d: dict<any>
      def Func()
        try
          g:result = map([], (_, v) => ({}[v]))->join() .. d['']
        catch
        endtry
      enddef
      silent! Func()
      assert_equal('0', g:result)
      unlet g:result
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_skip_cmds_with_silent()
  var lines =<< trim END
      vim9script

      def Func(b: bool)
        Crash()
      enddef

      def Crash()
        sil! :/not found/d _
        sil! :/not found/put _
      enddef

      Func(true)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_opfunc()
  nnoremap <F3> <cmd>set opfunc=g:Opfunc<cr>g@
  def g:Opfunc(_: any): string
    setline(1, 'ASDF')
    return ''
  enddef
  new
  setline(1, 'asdf')
  feedkeys("\<F3>$", 'x')
  assert_equal('ASDF', getline(1))

  bwipe!
  nunmap <F3>
enddef

func Test_opfunc_error()
  CheckScreendump
  call Run_Test_opfunc_error()
endfunc

def Run_Test_opfunc_error()
  # test that the error from Opfunc() is displayed right away
  var lines =<< trim END
      vim9script

      def Opfunc(type: string)
        try
          eval [][0]
        catch /nothing/  # error not caught
        endtry
      enddef
      &operatorfunc = Opfunc
      nnoremap <expr> l <SID>L()
      def L(): string
        return 'l'
      enddef
      'x'->repeat(10)->setline(1)
      feedkeys('g@l', 'n')
      feedkeys('llll')
  END
  call writefile(lines, 'XTest_opfunc_error', 'D')

  var buf = g:RunVimInTerminal('-S XTest_opfunc_error', {rows: 6, wait_for_ruler: 0})
  g:WaitForAssert(() => assert_match('Press ENTER', term_getline(buf, 6)))
  g:WaitForAssert(() => assert_match('E684: List index out of range: 0', term_getline(buf, 5)))

  # clean up
  g:StopVimInTerminal(buf)
enddef

" this was crashing on exit
def Test_nested_lambda_in_closure()
  var lines =<< trim END
      vim9script
      command WriteDone writefile(['Done'], 'XnestedDone')
      def Outer()
          def g:Inner()
              echo map([1, 2, 3], {_, v -> v + 1})
          enddef
          g:Inner()
      enddef
      defcompile
      # not reached
  END
  if !g:RunVim([], lines, '--clean -c WriteDone -c quit')
    return
  endif
  assert_equal(['Done'], readfile('XnestedDone'))
  delete('XnestedDone')
enddef

def Test_nested_closure_funcref()
  var lines =<< trim END
      vim9script
      def Func()
          var n: number
          def Nested()
              ++n
          enddef
          Nested()
          g:result_one = n
          var Ref = function(Nested)
          Ref()
          g:result_two = n
      enddef
      Func()
  END
  v9.CheckScriptSuccess(lines)
  assert_equal(1, g:result_one)
  assert_equal(2, g:result_two)
  unlet g:result_one g:result_two
enddef

def Test_nested_closure_in_dict()
  var lines =<< trim END
      vim9script
      def Func(): dict<any>
        var n: number
        def Inc(): number
          ++n
          return n
        enddef
        return {inc: function(Inc)}
      enddef
      disas Func
      var d = Func()
      assert_equal(1, d.inc())
      assert_equal(2, d.inc())
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_script_local_other_script()
  var lines =<< trim END
      function LegacyJob()
        let FuncRef = function('s:close_cb')
      endfunction
      function s:close_cb(...)
      endfunction
  END
  lines->writefile('Xlegacy.vim', 'D')
  source Xlegacy.vim
  g:LegacyJob()
  g:LegacyJob()
  g:LegacyJob()

  delfunc g:LegacyJob
enddef

def Test_check_func_arg_types()
  var lines =<< trim END
      vim9script
      def F1(x: string): string
        return x
      enddef

      def F2(x: number): number
        return x + 1
      enddef

      def G(Fg: func): dict<func>
        return {f: Fg}
      enddef

      def H(d: dict<func>): string
        return d.f('a')
      enddef
  END

  v9.CheckScriptSuccess(lines + ['echo H(G(F1))'])
  v9.CheckScriptFailure(lines + ['echo H(G(F2))'], 'E1013:')

  v9.CheckScriptFailure(lines + ['def SomeFunc(ff: func)', 'enddef'], 'E704:')
enddef

def Test_call_func_with_null()
  var lines =<< trim END
      def Fstring(v: string)
        assert_equal(null_string, v)
      enddef
      Fstring(null_string)
      def Fblob(v: blob)
        assert_equal(null_blob, v)
      enddef
      Fblob(null_blob)
      def Flist(v: list<number>)
        assert_equal(null_list, v)
      enddef
      Flist(null_list)
      def Fdict(v: dict<number>)
        assert_equal(null_dict, v)
      enddef
      Fdict(null_dict)
      def Ffunc(Fv: func(number): number)
        assert_equal(null_function, Fv)
      enddef
      Ffunc(null_function)
      if has('channel')
        def Fchannel(v: channel)
          assert_equal(null_channel, v)
        enddef
        Fchannel(null_channel)
        def Fjob(v: job)
          assert_equal(null_job, v)
        enddef
        Fjob(null_job)
      endif
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_null_default_argument()
  var lines =<< trim END
      def Fstring(v: string = null_string)
        assert_equal(null_string, v)
      enddef
      Fstring()
      def Fblob(v: blob = null_blob)
        assert_equal(null_blob, v)
      enddef
      Fblob()
      def Flist(v: list<number> = null_list)
        assert_equal(null_list, v)
      enddef
      Flist()
      def Fdict(v: dict<number> = null_dict)
        assert_equal(null_dict, v)
      enddef
      Fdict()
      def Ffunc(Fv: func(number): number = null_function)
        assert_equal(null_function, Fv)
      enddef
      Ffunc()
      if has('channel')
        def Fchannel(v: channel = null_channel)
          assert_equal(null_channel, v)
        enddef
        Fchannel()
        def Fjob(v: job = null_job)
          assert_equal(null_job, v)
        enddef
        Fjob()
      endif
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_null_return()
  var lines =<< trim END
      def Fstring(): string
        return null_string
      enddef
      assert_equal(null_string, Fstring())
      def Fblob(): blob
        return null_blob
      enddef
      assert_equal(null_blob, Fblob())
      def Flist(): list<number>
        return null_list
      enddef
      assert_equal(null_list, Flist())
      def Fdict(): dict<number>
        return null_dict
      enddef
      assert_equal(null_dict, Fdict())
      def Ffunc(): func(number): number
        return null_function
      enddef
      assert_equal(null_function, Ffunc())
      if has('channel')
        def Fchannel(): channel
          return null_channel
        enddef
        assert_equal(null_channel, Fchannel())
        def Fjob(): job
          return null_job
        enddef
        assert_equal(null_job, Fjob())
      endif
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_list_any_type_checked()
  var lines =<< trim END
      vim9script
      def Foo()
        --decl--
        Bar(l)
      enddef
      def Bar(ll: list<dict<any>>)
      enddef
      Foo()
  END
  # "any" could be "dict<any>", thus OK
  lines[2] = 'var l: list<any>'
  v9.CheckScriptSuccess(lines)
  lines[2] = 'var l: list<any> = []'
  v9.CheckScriptSuccess(lines)

  lines[2] = 'var l: list<any> = [11]'
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch, expected list<dict<any>> but got list<number>', 2)
enddef

def Test_compile_error()
  var lines =<< trim END
    def g:Broken()
      echo 'a' + {}
    enddef
    call g:Broken()
  END
  # First call: compilation error
  v9.CheckScriptFailure(lines, 'E1051: Wrong argument type for +')

  # Second call won't try compiling again
  assert_fails('call g:Broken()', 'E1091: Function is not compiled: Broken')
  delfunc g:Broken

  # No error when compiling with :silent!
  lines =<< trim END
    def g:Broken()
      echo 'a' + []
    enddef
    silent! defcompile
  END
  v9.CheckScriptSuccess(lines)

  # Calling the function won't try compiling again
  assert_fails('call g:Broken()', 'E1091: Function is not compiled: Broken')
  delfunc g:Broken
enddef

def Test_ignored_argument()
  var lines =<< trim END
      vim9script
      def Ignore(_, _): string
        return 'yes'
      enddef
      assert_equal('yes', Ignore(1, 2))

      func Ok(_)
        return a:_
      endfunc
      assert_equal('ok', Ok('ok'))

      func Oktoo()
        let _ = 'too'
        return _
      endfunc
      assert_equal('too', Oktoo())

      assert_equal([[1], [2], [3]], range(3)->mapnew((_, v) => [v]->map((_, w) => w + 1)))
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      def Ignore(_: string): string
        return _
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1181:', 1)

  lines =<< trim END
      var _ = 1
  END
  v9.CheckDefAndScriptFailure(lines, 'E1181:', 1)

  lines =<< trim END
      var x = _
  END
  v9.CheckDefAndScriptFailure(lines, 'E1181:', 1)
enddef

def Test_too_many_arguments()
  var lines =<< trim END
    echo [0, 1, 2]->map(() => 123)
  END
  v9.CheckDefAndScriptFailure(lines, ['E176:', 'E1106: 2 arguments too many'], 1)

  lines =<< trim END
    echo [0, 1, 2]->map((_) => 123)
  END
  v9.CheckDefAndScriptFailure(lines, ['E176', 'E1106: One argument too many'], 1)

  lines =<< trim END
      vim9script
      def OneArgument(arg: string)
        echo arg
      enddef
      var Ref = OneArgument
      Ref('a', 'b')
  END
  v9.CheckScriptFailure(lines, 'E118:')
enddef

def Test_funcref_with_base()
  var lines =<< trim END
      vim9script
      def TwoArguments(str: string, nr: number)
        echo str nr
      enddef
      var Ref = TwoArguments
      Ref('a', 12)
      'b'->Ref(34)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def TwoArguments(str: string, nr: number)
        echo str nr
      enddef
      var Ref = TwoArguments
      'a'->Ref('b')
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected number but got string', 6)

  lines =<< trim END
      vim9script
      def TwoArguments(str: string, nr: number)
        echo str nr
      enddef
      var Ref = TwoArguments
      123->Ref(456)
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch, expected string but got number')

  lines =<< trim END
      vim9script
      def TwoArguments(nr: number, str: string)
        echo str nr
      enddef
      var Ref = TwoArguments
      123->Ref('b')
      def AndNowCompiled()
        456->Ref('x')
      enddef
      AndNowCompiled()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_closing_brace_at_start_of_line()
  var lines =<< trim END
      def Func()
      enddef
      Func(
      )
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

func s:CreateMydict()
  let g:mydict = {}
  func g:mydict.afunc()
    let g:result = self.key
  endfunc
endfunc

def Test_numbered_function_reference()
  CreateMydict()
  var output = execute('legacy func g:mydict.afunc')
  var funcName = 'g:' .. substitute(output, '.*function \(\d\+\).*', '\1', '')
  execute 'function(' .. funcName .. ', [], {key: 42})()'
  # check that the function still exists
  assert_equal(output, execute('legacy func g:mydict.afunc'))
  unlet g:mydict
enddef

def Test_numbered_function_call()
  var lines =<< trim END
      let s:legacyscript = {}
      func s:legacyscript.Helper() abort
        return "Success"
      endfunc
      let g:legacyscript = deepcopy(s:legacyscript)

      let g:legacy_result = eval("g:legacyscript.Helper()")
      vim9cmd g:vim9_result = eval("g:legacyscript.Helper()")
  END
  v9.CheckScriptSuccess(lines)
  assert_equal('Success', g:legacy_result)
  assert_equal('Success', g:vim9_result)

  unlet g:legacy_result
  unlet g:vim9_result
enddef

def Test_go_beyond_end_of_cmd()
  # this was reading the byte after the end of the line
  var lines =<< trim END
    def F()
      cal
    enddef
    defcompile
  END
  v9.CheckScriptFailure(lines, 'E476:')
enddef

" Test for memory allocation failure when defining a new lambda
func Test_lambda_allocation_failure()
  new
  let lines =<< trim END
    vim9script
    g:Xlambda = (x): number => {
        return x + 1
      }
  END
  call setline(1, lines)
  call test_alloc_fail(GetAllocId('get_func'), 0, 0)
  call assert_fails('source', 'E342:')
  call assert_false(exists('g:Xlambda'))
  bw!
endfunc

def Test_lambda_argument_type_check()
  var lines =<< trim END
      vim9script

      def Scan(ll: list<any>): func(func(any))
        return (Emit: func(any)) => {
          for e in ll
            Emit(e)
          endfor
        }
      enddef

      def Sum(Cont: func(func(any))): any
        var sum = 0.0
        Cont((v: float) => {  # <== NOTE: the lambda expects a float
          sum += v
        })
        return sum
      enddef

      const ml = [3.0, 2, '7']
      echo Scan(ml)->Sum()
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch, expected float but got string')
enddef

def Test_multiple_funcref()
  # This was using a NULL pointer
  var lines =<< trim END
      vim9script
      def A(F: func, ...args: list<any>): func
          return funcref(F, args)
      enddef

      def B(F: func): func
          return funcref(A, [F])
      enddef

      def Test(n: number)
      enddef

      const X = B(Test)
      X(1)
  END
  v9.CheckScriptSuccess(lines)

  # slightly different case
  lines =<< trim END
      vim9script

      def A(F: func, ...args: list<any>): any
          return call(F, args)
      enddef

      def B(F: func): func
          return funcref(A, [F])
      enddef

      def Test(n: number)
      enddef

      const X = B(Test)
      X(1)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_cexpr_errmsg_line_number()
  var lines =<< trim END
      vim9script
      def Func()
        var qfl = {}
        cexpr qfl
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E777', 2)
enddef

def AddDefer(s: string)
  g:deferred->extend([s])
enddef

def DeferTwo()
  g:deferred->extend(['in Two'])
  for n in range(3)
    defer g:AddDefer('two' .. n)
  endfor
  g:deferred->extend(['end Two'])
enddef

def DeferOne()
  g:deferred->extend(['in One'])
  defer g:AddDefer('one')
  g:DeferTwo()
  g:deferred->extend(['end One'])

  writefile(['text'], 'XdeferFile')
  defer delete('XdeferFile')
enddef

def Test_defer()
  g:deferred = []
  g:DeferOne()
  assert_equal(['in One', 'in Two', 'end Two', 'two2', 'two1', 'two0', 'end One', 'one'], g:deferred)
  unlet g:deferred
  assert_equal('', glob('XdeferFile'))
enddef

def Test_invalid_redir()
  var lines =<< trim END
      def Tone()
        if 1
          redi =>@ 0
          redi END
        endif
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E354:')
  delfunc g:Tone

  # this was reading past the end of the line
  lines =<< trim END
      def Ttwo()
        if 0
          redi =>@ 0
          redi END
        endif
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E354:')
  delfunc g:Ttwo
enddef

func Test_keytyped_in_nested_function()
  CheckRunVimInTerminal

  call Run_Test_keytyped_in_nested_function()
endfunc

def Run_Test_keytyped_in_nested_function()
  var lines =<< trim END
      vim9script
      autocmd CmdlineEnter * sample#Init()

      exe 'set rtp=' .. getcwd() .. '/Xrtpdir'
  END
  writefile(lines, 'Xkeytyped', 'D')

  var dir = 'Xrtpdir/autoload'
  mkdir(dir, 'pR')

  lines =<< trim END
      vim9script
      export def Init(): void
         cnoremap <expr>" <SID>Quote('"')
      enddef
      def Quote(str: string): string
         def InPair(): number
            return 0
         enddef
         return str
      enddef
  END
  writefile(lines, dir .. '/sample.vim')

  var buf = g:RunVimInTerminal('-S Xkeytyped', {rows: 6})

  term_sendkeys(buf, ':"')
  g:VerifyScreenDump(buf, 'Test_keytyped_in_nested_func', {})

  # clean up
  term_sendkeys(buf, "\<Esc>")
  g:StopVimInTerminal(buf)
enddef

" The following messes up syntax highlight, keep near the end.
if has('python3')
  def Test_python3_command()
    py3 import vim
    py3 vim.command("g:done = 'yes'")
    assert_equal('yes', g:done)
    unlet g:done
  enddef

  def Test_python3_heredoc()
    py3 << trim EOF
      import vim
      vim.vars['didit'] = 'yes'
    EOF
    assert_equal('yes', g:didit)

    python3 << trim EOF
      import vim
      vim.vars['didit'] = 'again'
    EOF
    assert_equal('again', g:didit)
  enddef
endif

if has('lua')
  def Test_lua_heredoc()
    g:d = {}
    lua << trim EOF
        x = vim.eval('g:d')
        x['key'] = 'val'
    EOF
    assert_equal('val', g:d.key)
  enddef

  def Test_lua_heredoc_fails()
    var lines = [
      'vim9script',
      'def ExeLua()',
        'lua << trim EOLUA',
            "x = vim.eval('g:nodict')",
        'EOLUA',
      'enddef',
      'ExeLua()',
      ]
    v9.CheckScriptFailure(lines, 'E121: Undefined variable: g:nodict')
  enddef
endif

if has('perl')
  def Test_perl_heredoc_nested()
    var lines =<< trim END
        vim9script
        def F(): string
            def G(): string
                perl << EOF
        EOF
                return 'done'
            enddef
            return G()
        enddef
        assert_equal('done', F())
    END
    v9.CheckScriptSuccess(lines)
  enddef
endif


" vim: ts=8 sw=2 sts=2 expandtab tw=80 fdm=marker
