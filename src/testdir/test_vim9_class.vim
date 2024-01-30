" Test Vim9 classes

source check.vim
import './vim9.vim' as v9

def Test_class_basic()
  # Class supported only in "vim9script"
  var lines =<< trim END
    class NotWorking
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1316: Class can only be defined in Vim9 script', 1)

  # First character in a class name should be capitalized.
  lines =<< trim END
    vim9script
    class notWorking
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1314: Class name must start with an uppercase letter: notWorking', 2)

  # Only alphanumeric characters are supported in a class name
  lines =<< trim END
    vim9script
    class Not@working
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1315: White space required after name: Not@working', 2)

  # Unsupported keyword (instead of class)
  lines =<< trim END
    vim9script
    abstract noclass Something
    endclass
  END
  v9.CheckSourceFailure(lines, 'E475: Invalid argument: noclass Something', 2)

  # Only the complete word "class" should be recognized
  lines =<< trim END
    vim9script
    abstract classy Something
    endclass
  END
  v9.CheckSourceFailure(lines, 'E475: Invalid argument: classy Something', 2)

  # The complete "endclass" should be specified.
  lines =<< trim END
    vim9script
    class Something
    endcl
  END
  v9.CheckSourceFailure(lines, 'E1065: Command cannot be shortened: endcl', 3)

  # Additional words after "endclass"
  lines =<< trim END
    vim9script
    class Something
    endclass school's out
  END
  v9.CheckSourceFailure(lines, "E488: Trailing characters: school's out", 3)

  # Additional commands after "endclass"
  lines =<< trim END
    vim9script
    class Something
    endclass | echo 'done'
  END
  v9.CheckSourceFailure(lines, "E488: Trailing characters: | echo 'done'", 3)

  # Use old "this." prefixed member variable declaration syntax (without intialization)
  lines =<< trim END
    vim9script
    class Something
      this.count: number
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: this.count: number', 3)

  # Use old "this." prefixed member variable declaration syntax (with intialization)
  lines =<< trim END
    vim9script
    class Something
      this.count: number = 42
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: this.count: number = 42', 3)

  # Use old "this." prefixed member variable declaration syntax (type inferred)
  lines =<< trim END
    vim9script
    class Something
      this.count = 42
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: this.count = 42', 3)

  # Use "this" without any member variable name
  lines =<< trim END
    vim9script
    class Something
      this
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: this', 3)

  # Use "this." without any member variable name
  lines =<< trim END
    vim9script
    class Something
      this.
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: this.', 3)

  # Space between "this" and ".<variable>"
  lines =<< trim END
    vim9script
    class Something
      this .count
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: this .count', 3)

  # Space between "this." and the member variable name
  lines =<< trim END
    vim9script
    class Something
      this. count
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: this. count', 3)

  # Use "that" instead of "this"
  lines =<< trim END
    vim9script
    class Something
      var count: number
      that.count
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: that.count', 4)

  # Use "variable" instead of "var" for member variable declaration (without initialization)
  lines =<< trim END
    vim9script
    class Something
      variable count: number
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: variable count: number', 3)

  # Use "variable" instead of "var" for member variable declaration (with initialization)
  lines =<< trim END
    vim9script
    class Something
      variable count: number = 42
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: variable count: number = 42', 3)

  # Use "variable" instead of "var" for member variable declaration (type inferred)
  lines =<< trim END
    vim9script
    class Something
      variable count = 42
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: variable count = 42', 3)

  # Use a non-existing member variable in new()
  lines =<< trim END
    vim9script
    class Something
      def new()
        this.state = 0
      enddef
    endclass
    var obj = Something.new()
  END
  v9.CheckSourceFailure(lines, 'E1326: Variable "state" not found in object "Something"', 1)

  # Space before ":" in a member variable declaration
  lines =<< trim END
    vim9script
    class Something
      var count : number
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1059: No white space allowed before colon: count : number', 3)

  # No space after ":" in a member variable declaration
  lines =<< trim END
    vim9script
    class Something
      var count:number
    endclass
  END
  v9.CheckSourceFailure(lines, "E1069: White space required after ':'", 3)

  # Missing ":var" in a "var" member variable declaration (without initialization)
  lines =<< trim END
    vim9script
    class Something
      var: number
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1317: Invalid object variable declaration: var: number', 3)

  # Missing ":var" in a "var" member variable declaration (with initialization)
  lines =<< trim END
    vim9script
    class Something
      var: number = 42
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1317: Invalid object variable declaration: var: number = 42', 3)

  # Missing ":var" in a "var" member variable declaration (type inferred)
  lines =<< trim END
    vim9script
    class Something
      var = 42
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1317: Invalid object variable declaration: var = 42', 3)

  # Test for unsupported comment specifier
  lines =<< trim END
    vim9script
    class Something
      # comment
      #{
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1170: Cannot use #{ to start a comment', 3)

  # Test for using class as a bool
  lines =<< trim END
    vim9script
    class A
    endclass
    if A
    endif
  END
  v9.CheckSourceFailure(lines, 'E1405: Class "A" cannot be used as a value', 4)

  # Test for using object as a bool
  lines =<< trim END
    vim9script
    class A
    endclass
    var a = A.new()
    if a
    endif
  END
  v9.CheckSourceFailure(lines, 'E1320: Using an Object as a Number', 5)

  # Test for using class as a float
  lines =<< trim END
    vim9script
    class A
    endclass
    sort([1.1, A], 'f')
  END
  v9.CheckSourceFailure(lines, 'E1405: Class "A" cannot be used as a value', 4)

  # Test for using object as a float
  lines =<< trim END
    vim9script
    class A
    endclass
    var a = A.new()
    sort([1.1, a], 'f')
  END
  v9.CheckSourceFailure(lines, 'E1322: Using an Object as a Float', 5)

  # Test for using class as a string
  lines =<< trim END
    vim9script
    class A
    endclass
    :exe 'call ' .. A
  END
  v9.CheckSourceFailure(lines, 'E1405: Class "A" cannot be used as a value', 4)

  # Test for using object as a string
  lines =<< trim END
    vim9script
    class A
    endclass
    var a = A.new()
    :exe 'call ' .. a
  END
  v9.CheckSourceFailure(lines, 'E1324: Using an Object as a String', 5)

  # Test creating a class with member variables and methods, calling a object
  # method.  Check for using type() and typename() with a class and an object.
  lines =<< trim END
    vim9script

    class TextPosition
      var lnum: number
      var col: number

      # make a nicely formatted string
      def ToString(): string
        return $'({this.lnum}, {this.col})'
      enddef
    endclass

    # use the automatically generated new() method
    var pos = TextPosition.new(2, 12)
    assert_equal(2, pos.lnum)
    assert_equal(12, pos.col)

    # call an object method
    assert_equal('(2, 12)', pos.ToString())

    assert_equal(v:t_class, type(TextPosition))
    assert_equal(v:t_object, type(pos))
    assert_equal('class<TextPosition>', typename(TextPosition))
    assert_equal('object<TextPosition>', typename(pos))
  END
  v9.CheckSourceSuccess(lines)

  # When referencing object methods, space cannot be used after a "."
  lines =<< trim END
    vim9script
    class A
      def Foo(): number
        return 10
      enddef
    endclass
    var a = A.new()
    var v = a. Foo()
  END
  v9.CheckSourceFailure(lines, "E1202: No white space allowed after '.'", 8)

  # Using an object without specifying a method or a member variable
  lines =<< trim END
    vim9script
    class A
      def Foo(): number
        return 10
      enddef
    endclass
    var a = A.new()
    var v = a.
  END
  v9.CheckSourceFailure(lines, 'E15: Invalid expression: "a."', 8)

  # Error when parsing the arguments of an object method.
  lines =<< trim END
    vim9script
    class A
      def Foo()
      enddef
    endclass
    var a = A.new()
    var v = a.Foo(,)
  END
  v9.CheckSourceFailure(lines, 'E15: Invalid expression: "a.Foo(,)"', 7)

  # Use a multi-line initialization for a member variable
  lines =<< trim END
    vim9script
    class A
      var y = {
        X: 1
      }
    endclass
    var a = A.new()
  END
  v9.CheckSourceSuccess(lines)
enddef

" Tests for object/class methods in a class
def Test_class_def_method()
  # Using the "public" keyword when defining an object method
  var lines =<< trim END
    vim9script
    class A
      public def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1331: Public must be followed by "var" or "static"', 3)

  # Using the "public" keyword when defining a class method
  lines =<< trim END
    vim9script
    class A
      public static def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1388: Public keyword not supported for a method', 3)

  # Using the "public" keyword when defining an object protected method
  lines =<< trim END
    vim9script
    class A
      public def _Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1331: Public must be followed by "var" or "static"', 3)

  # Using the "public" keyword when defining a class protected method
  lines =<< trim END
    vim9script
    class A
      public static def _Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1388: Public keyword not supported for a method', 3)

  # Using a "def" keyword without an object method name
  lines =<< trim END
    vim9script
    class A
      def
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: def', 3)

  # Using a "def" keyword without a class method name
  lines =<< trim END
    vim9script
    class A
      static def
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: static def', 3)
enddef

def Test_class_defined_twice()
  # class defined twice should fail
  var lines =<< trim END
    vim9script
    class There
    endclass
    class There
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1041: Redefining script item: "There"', 4)

  # one class, reload same script twice is OK
  lines =<< trim END
    vim9script
    class There
    endclass
  END
  writefile(lines, 'XclassTwice.vim', 'D')
  source XclassTwice.vim
  source XclassTwice.vim
enddef

def Test_returning_null_object()
  # this was causing an internal error
  var lines =<< trim END
    vim9script

    class BufferList
      def Current(): any
        return null_object
      enddef
    endclass

    var buffers = BufferList.new()
    echo buffers.Current()
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_using_null_class()
  var lines =<< trim END
    @_ = null_class.member
  END
  v9.CheckDefExecAndScriptFailure(lines, ['E715: Dictionary required', 'E1363: Incomplete type'])
enddef

def Test_class_interface_wrong_end()
  var lines =<< trim END
    vim9script
    abstract class SomeName
      var member = 'text'
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E476: Invalid command: endinterface, expected endclass', 4)

  lines =<< trim END
    vim9script
    export interface AnotherName
      var member: string
    endclass
  END
  v9.CheckSourceFailure(lines, 'E476: Invalid command: endclass, expected endinterface', 4)
enddef

def Test_object_not_set()
  # Use an uninitialized object in script context
  var lines =<< trim END
    vim9script

    class State
      var value = 'xyz'
    endclass

    var state: State
    var db = {'xyz': 789}
    echo db[state.value]
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 9)

  # Use an uninitialized object from a def function
  lines =<< trim END
    vim9script

    class Class
      var id: string
      def Method1()
        echo 'Method1' .. this.id
      enddef
    endclass

    var obj: Class
    def Func()
      obj.Method1()
    enddef
    Func()
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 1)

  # Pass an uninitialized object variable to a "new" function and try to call an
  # object method.
  lines =<< trim END
    vim9script

    class Background
      var background = 'dark'
    endclass

    class Colorscheme
      var _bg: Background

      def GetBackground(): string
        return this._bg.background
      enddef
    endclass

    var bg: Background           # UNINITIALIZED
    echo Colorscheme.new(bg).GetBackground()
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 1)

  # TODO: this should not give an error but be handled at runtime
  lines =<< trim END
    vim9script

    class Class
      var id: string
      def Method1()
        echo 'Method1' .. this.id
      enddef
    endclass

    var obj = null_object
    def Func()
      obj.Method1()
    enddef
    Func()
  END
  v9.CheckSourceFailure(lines, 'E1363: Incomplete type', 1)
enddef

" Null object assignment and comparison
def Test_null_object_assign_compare()
  var lines =<< trim END
    vim9script

    var nullo = null_object
    def F(): any
      return nullo
    enddef
    assert_equal('object<Unknown>', typename(F()))

    var o0 = F()
    assert_true(o0 == null_object)
    assert_true(o0 == null)

    var o1: any = nullo
    assert_true(o1 == null_object)
    assert_true(o1 == null)

    def G()
      var x = null_object
    enddef

    class C
    endclass
    var o2: C
    assert_true(o2 == null_object)
    assert_true(o2 == null)

    o2 = null_object
    assert_true(o2 == null)

    o2 = C.new()
    assert_true(o2 != null)

    o2 = null_object
    assert_true(o2 == null)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for object member initialization and disassembly
def Test_class_member_initializer()
  var lines =<< trim END
    vim9script

    class TextPosition
      var lnum: number = 1
      var col: number = 1

      # constructor with only the line number
      def new(lnum: number)
        this.lnum = lnum
      enddef
    endclass

    var pos = TextPosition.new(3)
    assert_equal(3, pos.lnum)
    assert_equal(1, pos.col)

    var instr = execute('disassemble TextPosition.new')
    assert_match('new\_s*' ..
          '0 NEW TextPosition size \d\+\_s*' ..
          '\d PUSHNR 1\_s*' ..
          '\d STORE_THIS 0\_s*' ..
          '\d PUSHNR 1\_s*' ..
          '\d STORE_THIS 1\_s*' ..
          'this.lnum = lnum\_s*' ..
          '\d LOAD arg\[-1]\_s*' ..
          '\d PUSHNR 0\_s*' ..
          '\d LOAD $0\_s*' ..
          '\d\+ STOREINDEX object\_s*' ..
          '\d\+ RETURN object.*',
          instr)
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_member_any_used_as_object()
  var lines =<< trim END
    vim9script

    class Inner
      var value: number = 0
    endclass

    class Outer
      var inner: any
    endclass

    def F(outer: Outer)
      outer.inner.value = 1
    enddef

    var inner_obj = Inner.new(0)
    var outer_obj = Outer.new(inner_obj)
    F(outer_obj)
    assert_equal(1, inner_obj.value)
  END
  v9.CheckSourceSuccess(lines)

  # Try modifying a protected variable using an "any" object
  lines =<< trim END
    vim9script

    class Inner
      var _value: string = ''
    endclass

    class Outer
      var inner: any
    endclass

    def F(outer: Outer)
      outer.inner._value = 'b'
    enddef

    var inner_obj = Inner.new('a')
    var outer_obj = Outer.new(inner_obj)
    F(outer_obj)
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_value" in class "Inner"', 1)

  # Try modifying a non-existing variable using an "any" object
  lines =<< trim END
    vim9script

    class Inner
      var value: string = ''
    endclass

    class Outer
      var inner: any
    endclass

    def F(outer: Outer)
      outer.inner.someval = 'b'
    enddef

    var inner_obj = Inner.new('a')
    var outer_obj = Outer.new(inner_obj)
    F(outer_obj)
  END
  v9.CheckSourceFailure(lines, 'E1326: Variable "someval" not found in object "Inner"', 1)
enddef

" Nested assignment to a object variable which is of another class type
def Test_assignment_nested_type()
  var lines =<< trim END
    vim9script

    class Inner
      public var value: number = 0
    endclass

    class Outer
      var inner: Inner
    endclass

    def F(outer: Outer)
      outer.inner.value = 1
    enddef

    def Test_assign_to_nested_typed_member()
      var inner = Inner.new(0)
      var outer = Outer.new(inner)
      F(outer)
      assert_equal(1, inner.value)
    enddef

    Test_assign_to_nested_typed_member()

    var script_inner = Inner.new(0)
    var script_outer = Outer.new(script_inner)
    script_outer.inner.value = 1
    assert_equal(1, script_inner.value)
  END
  v9.CheckSourceSuccess(lines)

  # Assignment where target item is read only in :def
  lines =<< trim END
    vim9script

    class Inner
      var value: number = 0
    endclass

    class Outer
      var inner: Inner
    endclass

    def F(outer: Outer)
      outer.inner.value = 1
    enddef

    def Test_assign_to_nested_typed_member()
      var inner = Inner.new(0)
      var outer = Outer.new(inner)
      F(outer)
      assert_equal(1, inner.value)
    enddef

    Test_assign_to_nested_typed_member()
  END
  v9.CheckSourceFailure(lines, 'E1335: Variable "value" in class "Inner" is not writable', 1)

  # Assignment where target item is read only script level
  lines =<< trim END
    vim9script

    class Inner
      var value: number = 0
    endclass

    class Outer
      var inner: Inner
    endclass

    def F(outer: Outer)
      outer.inner.value = 1
    enddef

    var script_inner = Inner.new(0)
    var script_outer = Outer.new(script_inner)
    script_outer.inner.value = 1
    assert_equal(1, script_inner.value)
  END
  v9.CheckSourceFailure(lines, 'E1335: Variable "value" in class "Inner" is not writable', 17)
enddef

def Test_assignment_with_operator()
  # Use "+=" to assign to a object variable
  var lines =<< trim END
    vim9script

    class Foo
      public var x: number

      def Add(n: number)
        this.x += n
      enddef
    endclass

    var f =  Foo.new(3)
    f.Add(17)
    assert_equal(20, f.x)

    def AddToFoo(obj: Foo)
      obj.x += 3
    enddef

    AddToFoo(f)
    assert_equal(23, f.x)
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_list_of_objects()
  var lines =<< trim END
    vim9script

    class Foo
      def Add()
      enddef
    endclass

    def ProcessList(fooList: list<Foo>)
      for foo in fooList
        foo.Add()
      endfor
    enddef

    var l: list<Foo> = [Foo.new()]
    ProcessList(l)
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_expr_after_using_object()
  var lines =<< trim END
    vim9script

    class Something
      var label: string = ''
    endclass

    def Foo(): Something
      var v = Something.new()
      echo 'in Foo(): ' .. typename(v)
      return v
    enddef

    Foo()
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_class_default_new()
  var lines =<< trim END
    vim9script

    class TextPosition
      var lnum: number = 1
      var col: number = 1
    endclass

    var pos = TextPosition.new()
    assert_equal(1, pos.lnum)
    assert_equal(1, pos.col)

    pos = TextPosition.new(v:none, v:none)
    assert_equal(1, pos.lnum)
    assert_equal(1, pos.col)

    pos = TextPosition.new(3, 22)
    assert_equal(3, pos.lnum)
    assert_equal(22, pos.col)

    pos = TextPosition.new(v:none, 33)
    assert_equal(1, pos.lnum)
    assert_equal(33, pos.col)
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    class Person
      var name: string
      var age: number = 42
      var education: string = "unknown"

      def new(this.name, this.age = v:none, this.education = v:none)
      enddef
    endclass

    var piet = Person.new("Piet")
    assert_equal("Piet", piet.name)
    assert_equal(42, piet.age)
    assert_equal("unknown", piet.education)

    var chris = Person.new("Chris", 4, "none")
    assert_equal("Chris", chris.name)
    assert_equal(4, chris.age)
    assert_equal("none", chris.education)
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    class Person
      var name: string
      var age: number = 42
      var education: string = "unknown"

      def new(this.name, this.age = v:none, this.education = v:none)
      enddef
    endclass

    var missing = Person.new()
  END
  v9.CheckSourceFailure(lines, 'E119: Not enough arguments for function: new', 11)

  # Using a specific value to initialize an instance variable in the new()
  # method.
  lines =<< trim END
    vim9script
    class A
      var val: string
      def new(this.val = 'a')
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, "E1328: Constructor default value must be v:none:  = 'a'", 4)
enddef

def Test_class_new_with_object_member()
  var lines =<< trim END
    vim9script

    class C
      var str: string
      var num: number
      def new(this.str, this.num)
      enddef
      def newVals(this.str, this.num)
      enddef
    endclass

    def Check()
      try
        var c = C.new('cats', 2)
        assert_equal('cats', c.str)
        assert_equal(2, c.num)

        c = C.newVals('dogs', 4)
        assert_equal('dogs', c.str)
        assert_equal(4, c.num)
      catch
        assert_report($'Unexpected exception was caught: {v:exception}')
      endtry
    enddef

    Check()
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class C
      var str: string
      var num: number
      def new(this.str, this.num)
      enddef
    endclass

    def Check()
      try
        var c = C.new(1, 2)
      catch
        assert_report($'Unexpected exception was caught: {v:exception}')
      endtry
    enddef

    Check()
  END
  v9.CheckSourceFailure(lines, 'E1013: Argument 1: type mismatch, expected string but got number', 2)

  lines =<< trim END
    vim9script

    class C
      var str: string
      var num: number
      def newVals(this.str, this.num)
      enddef
    endclass

    def Check()
      try
        var c = C.newVals('dogs', 'apes')
      catch
        assert_report($'Unexpected exception was caught: {v:exception}')
      endtry
    enddef

    Check()
  END
  v9.CheckSourceFailure(lines, 'E1013: Argument 2: type mismatch, expected number but got string', 2)

  lines =<< trim END
    vim9script

    class C
      var str: string
      def new(str: any)
      enddef
    endclass

    def Check()
      try
        var c = C.new(1)
      catch
        assert_report($'Unexpected exception was caught: {v:exception}')
      endtry
    enddef

    Check()
  END
  v9.CheckSourceSuccess(lines)

  # Try using "this." argument in a class method
  lines =<< trim END
    vim9script
    class A
      var val = 10
      static def Foo(this.val: number)
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1390: Cannot use an object variable "this.val" except with the "new" method', 4)

  # Try using "this." argument in an object method
  lines =<< trim END
    vim9script
    class A
      var val = 10
      def Foo(this.val: number)
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1390: Cannot use an object variable "this.val" except with the "new" method', 4)
enddef

def Test_class_object_member_inits()
  var lines =<< trim END
    vim9script
    class TextPosition
      var lnum: number
      var col = 1
      var addcol: number = 2
    endclass

    var pos = TextPosition.new()
    assert_equal(0, pos.lnum)
    assert_equal(1, pos.col)
    assert_equal(2, pos.addcol)
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    class TextPosition
      var lnum
      var col = 1
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)

  # If the type is not specified for a member, then it should be set during
  # object creation and not when defining the class.
  lines =<< trim END
    vim9script

    var init_count = 0
    def Init(): string
      init_count += 1
      return 'foo'
    enddef

    class A
      var str1 = Init()
      var str2: string = Init()
      var col = 1
    endclass

    assert_equal(init_count, 0)
    var a = A.new()
    assert_equal(init_count, 2)
  END
  v9.CheckSourceSuccess(lines)

  # Test for initializing an object member with an unknown variable/type
  lines =<< trim END
    vim9script
    class A
       var value = init_val
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1001: Variable not found: init_val', 1)

  # Test for initializing an object member with an special type
  lines =<< trim END
    vim9script
    class A
       var value: void
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1330: Invalid type for object variable: void', 3)
enddef

" Test for instance variable access
def Test_instance_variable_access()
  var lines =<< trim END
    vim9script
    class Triple
       var _one = 1
       var two = 2
       public var three = 3

       def GetOne(): number
         return this._one
       enddef
    endclass

    var trip = Triple.new()
    assert_equal(1, trip.GetOne())
    assert_equal(2, trip.two)
    assert_equal(3, trip.three)
    assert_fails('echo trip._one', 'E1333: Cannot access protected variable "_one" in class "Triple"')

    assert_fails('trip._one = 11', 'E1333: Cannot access protected variable "_one" in class "Triple"')
    assert_fails('trip.two = 22', 'E1335: Variable "two" in class "Triple" is not writable')
    trip.three = 33
    assert_equal(33, trip.three)

    assert_fails('trip.four = 4', 'E1326: Variable "four" not found in object "Triple"')
  END
  v9.CheckSourceSuccess(lines)

  # Test for a public member variable name beginning with an underscore
  lines =<< trim END
    vim9script
    class A
      public var _val = 10
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1332: Public variable name cannot start with underscore: public var _val = 10', 3)

  lines =<< trim END
    vim9script

    class MyCar
      var make: string
      var age = 5

      def new(make_arg: string)
        this.make = make_arg
      enddef

      def GetMake(): string
        return $"make = {this.make}"
      enddef
      def GetAge(): number
        return this.age
      enddef
    endclass

    var c = MyCar.new("abc")
    assert_equal('make = abc', c.GetMake())

    c = MyCar.new("def")
    assert_equal('make = def', c.GetMake())

    var c2 = MyCar.new("123")
    assert_equal('make = 123', c2.GetMake())

    def CheckCar()
      assert_equal("make = def", c.GetMake())
      assert_equal(5, c.GetAge())
    enddef
    CheckCar()
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class MyCar
      var make: string

      def new(make_arg: string)
        this.make = make_arg
      enddef
    endclass

    var c = MyCar.new("abc")
    var c = MyCar.new("def")
  END
  v9.CheckSourceFailure(lines, 'E1041: Redefining script item: "c"', 12)

  lines =<< trim END
    vim9script

    class Foo
      var x: list<number> = []

      def Add(n: number): any
        this.x->add(n)
        return this
      enddef
    endclass

    echo Foo.new().Add(1).Add(2).x
    echo Foo.new().Add(1).Add(2)
          .x
    echo Foo.new().Add(1)
          .Add(2).x
    echo Foo.new()
          .Add(1).Add(2).x
    echo Foo.new()
          .Add(1) 
          .Add(2)
          .x
  END
  v9.CheckSourceSuccess(lines)

  # Test for "public" cannot be abbreviated
  lines =<< trim END
    vim9script
    class Something
      pub var val = 1
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1065: Command cannot be shortened: pub var val = 1', 3)

  # Test for "public" keyword must be followed by "var" or "static".
  lines =<< trim END
    vim9script
    class Something
      public val = 1
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1331: Public must be followed by "var" or "static"', 3)

  # Modify a instance variable using the class name in the script context
  lines =<< trim END
    vim9script
    class A
      public var val = 1
    endclass
    A.val = 1
  END
  v9.CheckSourceFailure(lines, 'E1376: Object variable "val" accessible only using class "A" object', 5)

  # Read a instance variable using the class name in the script context
  lines =<< trim END
    vim9script
    class A
      public var val = 1
    endclass
    var i = A.val
  END
  v9.CheckSourceFailure(lines, 'E1376: Object variable "val" accessible only using class "A" object', 5)

  # Modify a instance variable using the class name in a def function
  lines =<< trim END
    vim9script
    class A
      public var val = 1
    endclass
    def T()
      A.val = 1
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1376: Object variable "val" accessible only using class "A" object', 1)

  # Read a instance variable using the class name in a def function
  lines =<< trim END
    vim9script
    class A
      public var val = 1
    endclass
    def T()
      var i = A.val
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1376: Object variable "val" accessible only using class "A" object', 1)

  # Access from child class extending a class:
  lines =<< trim END
    vim9script
    class A
      var ro_obj_var = 10
      public var rw_obj_var = 20
      var _priv_obj_var = 30
    endclass

    class B extends A
      def Foo()
        var x: number
        x = this.ro_obj_var
        this.ro_obj_var = 0
        x = this.rw_obj_var
        this.rw_obj_var = 0
        x = this._priv_obj_var
        this._priv_obj_var = 0
      enddef
    endclass

    var b = B.new()
    b.Foo()
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for class variable access
def Test_class_variable_access()
  # Test for "static" cannot be abbreviated
  var lines =<< trim END
    vim9script
    class Something
      stat var val = 1
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1065: Command cannot be shortened: stat var val = 1', 3)

  # Test for "static" cannot be followed by "public".
  lines =<< trim END
    vim9script
    class Something
      static public var val = 1
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1368: Static must be followed by "var" or "def"', 3)

  # A readonly class variable cannot be modified from a child class
  lines =<< trim END
    vim9script
    class A
      static var ro_class_var = 40
    endclass

    class B extends A
      def Foo()
        A.ro_class_var = 50
      enddef
    endclass

    var b = B.new()
    b.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1335: Variable "ro_class_var" in class "A" is not writable', 1)

  # A protected class variable cannot be accessed from a child class
  lines =<< trim END
    vim9script
    class A
      static var _priv_class_var = 60
    endclass

    class B extends A
      def Foo()
        var i = A._priv_class_var
      enddef
    endclass

    var b = B.new()
    b.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_priv_class_var" in class "A"', 1)

  # A protected class variable cannot be modified from a child class
  lines =<< trim END
    vim9script
    class A
      static var _priv_class_var = 60
    endclass

    class B extends A
      def Foo()
        A._priv_class_var = 0
      enddef
    endclass

    var b = B.new()
    b.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_priv_class_var" in class "A"', 1)

  # Access from child class extending a class and from script context
  lines =<< trim END
    vim9script
    class A
      static var ro_class_var = 10
      public static var rw_class_var = 20
      static var _priv_class_var = 30
    endclass

    class B extends A
      def Foo()
        var x: number
        x = A.ro_class_var
        assert_equal(10, x)
        x = A.rw_class_var
        assert_equal(25, x)
        A.rw_class_var = 20
        assert_equal(20, A.rw_class_var)
      enddef
    endclass

    assert_equal(10, A.ro_class_var)
    assert_equal(20, A.rw_class_var)
    A.rw_class_var = 25
    assert_equal(25, A.rw_class_var)
    var b = B.new()
    b.Foo()
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_class_object_compare()
  var class_lines =<< trim END
    vim9script
    class Item
      var nr = 0
      var name = 'xx'
    endclass
  END

  # used at the script level and in a compiled function
  var test_lines =<< trim END
    var i1 = Item.new()
    assert_equal(i1, i1)
    assert_true(i1 is i1)
    var i2 = Item.new()
    assert_equal(i1, i2)
    assert_false(i1 is i2)
    var i3 = Item.new(0, 'xx')
    assert_equal(i1, i3)

    var io1 = Item.new(1, 'xx')
    assert_notequal(i1, io1)
    var io2 = Item.new(0, 'yy')
    assert_notequal(i1, io2)
  END

  v9.CheckSourceSuccess(class_lines + test_lines)
  v9.CheckSourceSuccess(
    class_lines + ['def Test()'] + test_lines + ['enddef', 'Test()'])

  for op in ['>', '>=', '<', '<=', '=~', '!~']
    var op_lines = [
          'var i1 = Item.new()',
          'var i2 = Item.new()',
          'echo i1 ' .. op .. ' i2',
          ]
    v9.CheckSourceFailure(class_lines + op_lines, 'E1153: Invalid operation for object', 8)
    v9.CheckSourceFailure(class_lines
          + ['def Test()'] + op_lines + ['enddef', 'Test()'], 'E1153: Invalid operation for object')
  endfor
enddef

def Test_object_type()
  var lines =<< trim END
    vim9script

    class One
      var one = 1
    endclass
    class Two
      var two = 2
    endclass
    class TwoMore extends Two
      var more = 9
    endclass

    var o: One = One.new()
    var t: Two = Two.new()
    var m: TwoMore = TwoMore.new()
    var tm: Two = TwoMore.new()

    t = m
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class One
      var one = 1
    endclass
    class Two
      var two = 2
    endclass

    var o: One = Two.new()
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected object<One> but got object<Two>', 10)

  lines =<< trim END
    vim9script

    interface One
      def GetMember(): number
    endinterface
    class Two implements One
      var one = 1
      def GetMember(): number
        return this.one
      enddef
    endclass

    var o: One = Two.new(5)
    assert_equal(5, o.GetMember())
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class Num
      var n: number = 0
    endclass

    def Ref(name: string): func(Num): Num
      return (arg: Num): Num => {
        return eval(name)(arg)
      }
    enddef

    const Fn = Ref('Double')
    var Double = (m: Num): Num => Num.new(m.n * 2)

    echo Fn(Num.new(4))
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_class_member()
  # check access rules
  var lines =<< trim END
    vim9script
    class TextPos
       var lnum = 1
       var col = 1
       static var counter = 0
       static var _secret = 7
       public static var anybody = 42

       static def AddToCounter(nr: number)
         counter += nr
       enddef
    endclass

    assert_equal(0, TextPos.counter)
    TextPos.AddToCounter(3)
    assert_equal(3, TextPos.counter)
    assert_fails('echo TextPos.noSuchMember', 'E1337: Class variable "noSuchMember" not found in class "TextPos"')

    def GetCounter(): number
      return TextPos.counter
    enddef
    assert_equal(3, GetCounter())

    assert_fails('TextPos.noSuchMember = 2', 'E1337: Class variable "noSuchMember" not found in class "TextPos"')
    assert_fails('TextPos.counter = 5', 'E1335: Variable "counter" in class "TextPos" is not writable')
    assert_fails('TextPos.counter += 5', 'E1335: Variable "counter" in class "TextPos" is not writable')

    assert_fails('echo TextPos._secret', 'E1333: Cannot access protected variable "_secret" in class "TextPos"')
    assert_fails('TextPos._secret = 8', 'E1333: Cannot access protected variable "_secret" in class "TextPos"')

    assert_equal(42, TextPos.anybody)
    TextPos.anybody = 12
    assert_equal(12, TextPos.anybody)
    TextPos.anybody += 5
    assert_equal(17, TextPos.anybody)
  END
  v9.CheckSourceSuccess(lines)

  # example in the help
  lines =<< trim END
    vim9script
    class OtherThing
      var size: number
      static var totalSize: number

      def new(this.size)
        totalSize += this.size
      enddef
    endclass
    assert_equal(0, OtherThing.totalSize)
    var to3 = OtherThing.new(3)
    assert_equal(3, OtherThing.totalSize)
    var to7 = OtherThing.new(7)
    assert_equal(10, OtherThing.totalSize)
  END
  v9.CheckSourceSuccess(lines)

  # using static class member twice
  lines =<< trim END
    vim9script

    class HTML
      static var author: string = 'John Doe'

      static def MacroSubstitute(s: string): string
        return substitute(s, '{{author}}', author, 'gi')
      enddef
    endclass

    assert_equal('some text', HTML.MacroSubstitute('some text'))
    assert_equal('some text', HTML.MacroSubstitute('some text'))
  END
  v9.CheckSourceSuccess(lines)

  # access protected member in lambda
  lines =<< trim END
    vim9script

    class Foo
      var _x: number = 0

      def Add(n: number): number
        const F = (): number => this._x + n
        return F()
      enddef
    endclass

    var foo = Foo.new()
    assert_equal(5, foo.Add(5))
  END
  v9.CheckSourceSuccess(lines)

  # access protected member in lambda body
  lines =<< trim END
    vim9script

    class Foo
      var _x: number = 6

      def Add(n: number): number
        var Lam = () => {
          this._x = this._x + n
        }
        Lam()
        return this._x
      enddef
    endclass

    var foo = Foo.new()
    assert_equal(13, foo.Add(7))
  END
  v9.CheckSourceSuccess(lines)

  # check shadowing
  lines =<< trim END
    vim9script

    class Some
      static var count = 0
      def Method(count: number)
        echo count
      enddef
    endclass

    var s = Some.new()
    s.Method(7)
  END
  v9.CheckSourceFailure(lines, 'E1340: Argument already declared in the class: count', 5)

  # Use a local variable in a method with the same name as a class variable
  lines =<< trim END
    vim9script

    class Some
      static var count = 0
      def Method(arg: number)
        var count = 3
        echo arg count
      enddef
    endclass

    var s = Some.new()
    s.Method(7)
  END
  v9.CheckSourceFailure(lines, 'E1341: Variable already declared in the class: count', 1)

  # Test for using an invalid type for a member variable
  lines =<< trim END
    vim9script
    class A
      var val: xxx
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1010: Type not recognized: xxx', 3)

  # Test for setting a member on a null object
  lines =<< trim END
    vim9script
    class A
      public var val: string
    endclass

    def F()
      var obj: A
      obj.val = ""
    enddef
    F()
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 2)

  # Test for accessing a member on a null object
  lines =<< trim END
    vim9script
    class A
      var val: string
    endclass

    def F()
      var obj: A
      echo obj.val
    enddef
    F()
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 2)

  # Test for setting a member on a null object, at script level
  lines =<< trim END
    vim9script
    class A
      public var val: string
    endclass

    var obj: A
    obj.val = ""
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 7)

  # Test for accessing a member on a null object, at script level
  lines =<< trim END
    vim9script
    class A
      var val: string
    endclass

    var obj: A
    echo obj.val
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 7)

  # Test for no space before or after the '=' when initializing a member
  # variable
  lines =<< trim END
    vim9script
    class A
      var val: number= 10
    endclass
  END
  v9.CheckSourceFailure(lines, "E1004: White space required before and after '='", 3)
  lines =<< trim END
    vim9script
    class A
      var val: number =10
    endclass
  END
  v9.CheckSourceFailure(lines, "E1004: White space required before and after '='", 3)

  # Access a non-existing member
  lines =<< trim END
    vim9script
    class A
    endclass
    var a = A.new()
    var v = a.bar
  END
  v9.CheckSourceFailure(lines, 'E1326: Variable "bar" not found in object "A"', 5)
enddef

" These messages should show the defining class of the variable (base class),
" not the class that did the reference (super class)
def Test_defining_class_message()
  var lines =<< trim END
    vim9script

    class Base
      var _v1: list<list<number>>
    endclass

    class Child extends Base
    endclass

    var o = Child.new()
    var x = o._v1
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_v1" in class "Base"', 11)
  lines =<< trim END
    vim9script

    class Base
      var _v1: list<list<number>>
    endclass

    class Child extends Base
    endclass

    def F()
      var o = Child.new()
      var x = o._v1
    enddef
    F()
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_v1" in class "Base"', 2)
  lines =<< trim END
    vim9script

    class Base
      var v1: list<list<number>>
    endclass

    class Child extends Base
    endclass

    var o = Child.new()
    o.v1 = []
  END
  v9.CheckSourceFailure(lines, 'E1335: Variable "v1" in class "Base" is not writable', 11)
  lines =<< trim END
    vim9script

    class Base
      var v1: list<list<number>>
    endclass

    class Child extends Base
    endclass

    def F()
      var o = Child.new()
      o.v1 = []
    enddef
    F()
  END

  # Attempt to read a protected variable that is in the middle
  # of the class hierarchy.
  v9.CheckSourceFailure(lines, 'E1335: Variable "v1" in class "Base" is not writable', 2)
  lines =<< trim END
    vim9script

    class Base0
    endclass

    class Base extends Base0
      var _v1: list<list<number>>
    endclass

    class Child extends Base
    endclass

    def F()
      var o = Child.new()
      var x = o._v1
    enddef
    F()
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_v1" in class "Base"', 2)

  # Attempt to read a protected variable that is at the start
  # of the class hierarchy.
  lines =<< trim END
    vim9script

    class Base0
    endclass

    class Base extends Base0
    endclass

    class Child extends Base
      var _v1: list<list<number>>
    endclass

    def F()
      var o = Child.new()
      var x = o._v1
    enddef
    F()
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_v1" in class "Child"', 2)
enddef

func Test_class_garbagecollect()
  let lines =<< trim END
    vim9script

    class Point
      var p = [2, 3]
      static var pl = ['a', 'b']
      static var pd = {a: 'a', b: 'b'}
    endclass

    echo Point.pl Point.pd
    call test_garbagecollect_now()
    echo Point.pl Point.pd
  END
  call v9.CheckSourceSuccess(lines)

  let lines =<< trim END
    vim9script

    interface View
    endinterface

    class Widget
      var view: View
    endclass

    class MyView implements View
      var widget: Widget

      def new()
        # this will result in a circular reference to this object
        var widget = Widget.new(this)
      enddef
    endclass

    var view = MyView.new()

    # overwrite "view", will be garbage-collected next
    view = MyView.new()
    test_garbagecollect_now()
  END
  call v9.CheckSourceSuccess(lines)
endfunc

" Test interface garbage collection
func Test_interface_garbagecollect()
  let lines =<< trim END
    vim9script

    interface I
      var ro_obj_var: number

      def ObjFoo(): number
    endinterface

    class A implements I
      static var ro_class_var: number = 10
      public static var rw_class_var: number = 20
      static var _priv_class_var: number = 30
      var ro_obj_var: number = 40
      var _priv_obj_var: number = 60

      static def _ClassBar(): number
        return _priv_class_var
      enddef

      static def ClassFoo(): number
        return ro_class_var + rw_class_var + A._ClassBar()
      enddef

      def _ObjBar(): number
        return this._priv_obj_var
      enddef

      def ObjFoo(): number
        return this.ro_obj_var + this._ObjBar()
      enddef
    endclass

    assert_equal(60, A.ClassFoo())
    var o = A.new()
    assert_equal(100, o.ObjFoo())
    test_garbagecollect_now()
    assert_equal(60, A.ClassFoo())
    assert_equal(100, o.ObjFoo())
  END
  call v9.CheckSourceSuccess(lines)
endfunc

def Test_class_method()
  var lines =<< trim END
    vim9script
    class Value
      var value = 0
      static var objects = 0

      def new(v: number)
        this.value = v
        ++objects
      enddef

      static def GetCount(): number
        return objects
      enddef
    endclass

    assert_equal(0, Value.GetCount())
    var v1 = Value.new(2)
    assert_equal(1, Value.GetCount())
    var v2 = Value.new(7)
    assert_equal(2, Value.GetCount())
  END
  v9.CheckSourceSuccess(lines)

  # Test for cleaning up after a class definition failure when using class
  # functions.
  lines =<< trim END
    vim9script
    class A
      static def Foo()
      enddef
      aaa
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1318: Not a valid command in a class: aaa', 5)

  # Test for calling a class method from another class method without the class
  # name prefix.
  lines =<< trim END
    vim9script
    class A
      static var myList: list<number> = [1]
      static def Foo(n: number)
        myList->add(n)
      enddef
      static def Bar()
        Foo(2)
      enddef
      def Baz()
        Foo(3)
      enddef
    endclass
    A.Bar()
    var a = A.new()
    a.Baz()
    assert_equal([1, 2, 3], A.myList)
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_class_defcompile()
  var lines =<< trim END
    vim9script

    class C
      def Fo(i: number): string
        return i
      enddef
    endclass

    defcompile C.Fo
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected string but got number', 1)

  lines =<< trim END
    vim9script

    class C
      static def Fc(): number
        return 'x'
      enddef
    endclass

    defcompile C.Fc
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected number but got string', 1)

  lines =<< trim END
    vim9script

    class C
      static def new()
      enddef
    endclass

    defcompile C.new
  END
  v9.CheckSourceFailure(lines, 'E1370: Cannot define a "new" method as static', 5)

  # Trying to compile a function using a non-existing class variable
  lines =<< trim END
    vim9script
    defcompile x.Foo()
  END
  v9.CheckSourceFailure(lines, 'E475: Invalid argument: x.Foo()', 2)

  # Trying to compile a function using a variable which is not a class
  lines =<< trim END
    vim9script
    var x: number
    defcompile x.Foo()
  END
  v9.CheckSourceFailure(lines, 'E475: Invalid argument: x.Foo()', 3)

  # Trying to compile a function without specifying the name
  lines =<< trim END
    vim9script
    class A
    endclass
    defcompile A.
  END
  v9.CheckSourceFailure(lines, 'E475: Invalid argument: A.', 4)

  # Trying to compile a non-existing class object member function
  lines =<< trim END
    vim9script
    class A
    endclass
    var a = A.new()
    defcompile a.Foo()
  END
  v9.CheckSourceFailureList(lines, ['E1326: Variable "Foo" not found in object "A"', 'E475: Invalid argument: a.Foo()'])
enddef

def Test_class_object_to_string()
  var lines =<< trim END
    vim9script
    class TextPosition
      var lnum = 1
      var col = 22
    endclass

    assert_equal("class TextPosition", string(TextPosition))

    var pos = TextPosition.new()
    assert_equal("object of TextPosition {lnum: 1, col: 22}", string(pos))
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_interface_basics()
  var lines =<< trim END
    vim9script
    interface Something
      var ro_var: list<number>
      def GetCount(): number
    endinterface
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    interface SomethingWrong
      static var count = 7
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1342: Interface can only be defined in Vim9 script', 1)

  lines =<< trim END
    vim9script

    interface Some
      var value: number
      def Method(value: number)
    endinterface
  END
  # The argument name and the object member name are the same, but this is not a
  # problem because object members are always accessed with the "this." prefix.
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    interface somethingWrong
      static var count = 7
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1343: Interface name must start with an uppercase letter: somethingWrong', 2)

  lines =<< trim END
    vim9script
    interface SomethingWrong
      var value: string
      var count = 7
      def GetCount(): number
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1344: Cannot initialize a variable in an interface', 4)

  lines =<< trim END
    vim9script
    interface SomethingWrong
      var value: string
      var count: number
      def GetCount(): number
        return 5
      enddef
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1345: Not a valid command in an interface: return 5', 6)

  lines =<< trim END
    vim9script
    export interface EnterExit
      def Enter(): void
      def Exit(): void
    endinterface
  END
  writefile(lines, 'XdefIntf.vim', 'D')

  lines =<< trim END
    vim9script
    import './XdefIntf.vim' as defIntf
    export def With(ee: defIntf.EnterExit, F: func)
      ee.Enter()
      try
        F()
      finally
        ee.Exit()
      endtry
    enddef
  END
  v9.CheckScriptSuccess(lines)

  var imported =<< trim END
    vim9script
    export abstract class EnterExit
      def Enter(): void
      enddef
      def Exit(): void
      enddef
    endclass
  END
  writefile(imported, 'XdefIntf2.vim', 'D')

  lines[1] = " import './XdefIntf2.vim' as defIntf"
  v9.CheckScriptSuccess(lines)
enddef

def Test_class_implements_interface()
  var lines =<< trim END
    vim9script

    interface Some
      var count: number
      def Method(nr: number)
    endinterface

    class SomeImpl implements Some
      var count: number
      def Method(nr: number)
        echo nr
      enddef
    endclass

    interface Another
      var member: string
    endinterface

    class AnotherImpl implements Some, Another
      var member = 'abc'
      var count = 20
      def Method(nr: number)
        echo nr
      enddef
    endclass
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    interface Some
      var count: number
    endinterface

    class SomeImpl implements Some implements Some
      var count: number
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1350: Duplicate "implements"', 7)

  lines =<< trim END
    vim9script

    interface Some
      var count: number
    endinterface

    class SomeImpl implements Some, Some
      var count: number
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1351: Duplicate interface after "implements": Some', 7)

  lines =<< trim END
    vim9script

    interface Some
      var counter: number
      def Method(nr: number)
    endinterface

    class SomeImpl implements Some
      var count: number
      def Method(nr: number)
        echo nr
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1348: Variable "counter" of interface "Some" is not implemented', 13)

  lines =<< trim END
    vim9script

    interface Some
      var count: number
      def Methods(nr: number)
    endinterface

    class SomeImpl implements Some
      var count: number
      def Method(nr: number)
        echo nr
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1349: Method "Methods" of interface "Some" is not implemented', 13)

  # Check different order of members in class and interface works.
  lines =<< trim END
    vim9script

      interface Result
        var label: string
        var errpos: number
      endinterface

      # order of members is opposite of interface
      class Failure implements Result
        public var lnum: number = 5
        var errpos: number = 42
        var label: string = 'label'
      endclass

    def Test()
      var result: Result = Failure.new()

        assert_equal('label', result.label)
        assert_equal(42, result.errpos)
      enddef

    Test()
  END
  v9.CheckSourceSuccess(lines)

  # Interface name after "extends" doesn't end in a space or NUL character
  lines =<< trim END
    vim9script
    interface A
    endinterface
    class B extends A"
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1315: White space required after name: A"', 4)

  # Trailing characters after a class name
  lines =<< trim END
    vim9script
    class A bbb
    endclass
  END
  v9.CheckSourceFailure(lines, 'E488: Trailing characters: bbb', 2)

  # using "implements" with a non-existing class
  lines =<< trim END
    vim9script
    class A implements B
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1346: Interface name not found: B', 3)

  # using "implements" with a regular class
  lines =<< trim END
    vim9script
    class A
    endclass
    class B implements A
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1347: Not a valid interface: A', 5)

  # using "implements" with a variable
  lines =<< trim END
    vim9script
    var T: number = 10
    class A implements T
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1347: Not a valid interface: T', 4)

  # implements should be followed by a white space
  lines =<< trim END
    vim9script
    interface A
    endinterface
    class B implements A;
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1315: White space required after name: A;', 4)

  lines =<< trim END
    vim9script

    interface One
      def IsEven(nr: number): bool
    endinterface
    class Two implements One
      def IsEven(nr: number): string
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1383: Method "IsEven": type mismatch, expected func(number): bool but got func(number): string', 9)

  lines =<< trim END
    vim9script

    interface One
      def IsEven(nr: number): bool
    endinterface
    class Two implements One
      def IsEven(nr: bool): bool
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1383: Method "IsEven": type mismatch, expected func(number): bool but got func(bool): bool', 9)

  lines =<< trim END
    vim9script

    interface One
      def IsEven(nr: number): bool
    endinterface
    class Two implements One
      def IsEven(nr: number, ...extra: list<number>): bool
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1383: Method "IsEven": type mismatch, expected func(number): bool but got func(number, ...list<number>): bool', 9)

  # access superclass interface members from subclass, mix variable order
  lines =<< trim END
    vim9script

    interface I1
      var mvar1: number
      var mvar2: number
    endinterface

    # NOTE: the order is swapped
    class A implements I1
      var mvar2: number
      var mvar1: number
      public static var svar2: number
      public static var svar1: number
      def new()
        svar1 = 11
        svar2 = 12
        this.mvar1 = 111
        this.mvar2 = 112
      enddef
    endclass

    class B extends A
      def new()
        this.mvar1 = 121
        this.mvar2 = 122
      enddef
    endclass

    class C extends B
      def new()
        this.mvar1 = 131
        this.mvar2 = 132
      enddef
    endclass

    def F2(i: I1): list<number>
      return [ i.mvar1, i.mvar2 ]
    enddef

    var oa = A.new()
    var ob = B.new()
    var oc = C.new()

    assert_equal([111, 112], F2(oa))
    assert_equal([121, 122], F2(ob))
    assert_equal([131, 132], F2(oc))
  END
  v9.CheckSourceSuccess(lines)

  # Access superclass interface members from subclass, mix variable order.
  # Two interfaces, one on A, one on B; each has both kinds of variables
  lines =<< trim END
    vim9script

    interface I1
      var mvar1: number
      var mvar2: number
    endinterface

    interface I2
      var mvar3: number
      var mvar4: number
    endinterface

    class A implements I1
      public static var svar1: number
      public static var svar2: number
      var mvar1: number
      var mvar2: number
      def new()
        svar1 = 11
        svar2 = 12
        this.mvar1 = 111
        this.mvar2 = 112
      enddef
    endclass

    class B extends A implements I2
      static var svar3: number
      static var svar4: number
      var mvar3: number
      var mvar4: number
      def new()
        svar3 = 23
        svar4 = 24
        this.mvar1 = 121
        this.mvar2 = 122
        this.mvar3 = 123
        this.mvar4 = 124
      enddef
    endclass

    class C extends B
      public static var svar5: number
      def new()
        svar5 = 1001
        this.mvar1 = 131
        this.mvar2 = 132
        this.mvar3 = 133
        this.mvar4 = 134
      enddef
    endclass

    def F2(i: I1): list<number>
      return [ i.mvar1, i.mvar2 ]
    enddef

    def F4(i: I2): list<number>
      return [ i.mvar3, i.mvar4 ]
    enddef

    var oa = A.new()
    var ob = B.new()
    var oc = C.new()

    assert_equal([[111, 112]], [F2(oa)])
    assert_equal([[121, 122], [123, 124]], [F2(ob), F4(ob)])
    assert_equal([[131, 132], [133, 134]], [F2(oc), F4(oc)])
  END
  v9.CheckSourceSuccess(lines)

  # Using two interface names without a space after the ","
  lines =<< trim END
    vim9script
    interface A
    endinterface
    interface B
    endinterface
    class C implements A,B
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1315: White space required after name: A,B', 6)

  # No interface name after a comma
  lines =<< trim END
    vim9script
    interface A
    endinterface
    class B implements A,
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1389: Missing name after implements', 4)

  # No interface name after implements
  lines =<< trim END
    vim9script
    class A implements
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1389: Missing name after implements', 2)
enddef

def Test_call_interface_method()
  var lines =<< trim END
    vim9script
    interface Base
      def Enter(): void
    endinterface

    class Child implements Base
      def Enter(): void
        g:result ..= 'child'
      enddef
    endclass

    def F(obj: Base)
      obj.Enter()
    enddef

    g:result = ''
    F(Child.new())
    assert_equal('child', g:result)
    unlet g:result
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    class Base
      def Enter(): void
        g:result ..= 'base'
      enddef
    endclass

    class Child extends Base
      def Enter(): void
        g:result ..= 'child'
      enddef
    endclass

    def F(obj: Base)
      obj.Enter()
    enddef

    g:result = ''
    F(Child.new())
    assert_equal('child', g:result)
    unlet g:result
  END
  v9.CheckSourceSuccess(lines)

  # method of interface returns a value
  lines =<< trim END
    vim9script
    interface Base
      def Enter(): string
    endinterface

    class Child implements Base
      def Enter(): string
        g:result ..= 'child'
        return "/resource"
      enddef
    endclass

    def F(obj: Base)
      var r = obj.Enter()
      g:result ..= r
    enddef

    g:result = ''
    F(Child.new())
    assert_equal('child/resource', g:result)
    unlet g:result
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    class Base
      def Enter(): string
        return null_string
      enddef
    endclass

    class Child extends Base
      def Enter(): string
        g:result ..= 'child'
        return "/resource"
      enddef
    endclass

    def F(obj: Base)
      var r = obj.Enter()
      g:result ..= r
    enddef

    g:result = ''
    F(Child.new())
    assert_equal('child/resource', g:result)
    unlet g:result
  END
  v9.CheckSourceSuccess(lines)

  # No class that implements the interface.
  lines =<< trim END
    vim9script

    interface IWithEE
      def Enter(): any
      def Exit(): void
    endinterface

    def With1(ee: IWithEE, F: func)
      var r = ee.Enter()
    enddef

    defcompile
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_class_used_as_type()
  var lines =<< trim END
    vim9script

    class Point
      var x = 0
      var y = 0
    endclass

    var p: Point
    p = Point.new(2, 33)
    assert_equal(2, p.x)
    assert_equal(33, p.y)
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    interface HasX
      var x: number
    endinterface

    class Point implements HasX
      var x = 0
      var y = 0
    endclass

    var p: Point
    p = Point.new(2, 33)
    var hx = p
    assert_equal(2, hx.x)
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class Point
      var x = 0
      var y = 0
    endclass

    var p: Point
    p = 'text'
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected object<Point> but got string', 9)
enddef

def Test_class_extends()
  var lines =<< trim END
    vim9script
    class Base
      var one = 1
      def GetOne(): number
        return this.one
      enddef
    endclass
    class Child extends Base
      var two = 2
      def GetTotal(): number
        return this.one + this.two
      enddef
    endclass
    var o = Child.new()
    assert_equal(1, o.one)
    assert_equal(2, o.two)
    assert_equal(1, o.GetOne())
    assert_equal(3, o.GetTotal())
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    class Base
      var one = 1
    endclass
    class Child extends Base
      var two = 2
    endclass
    var o = Child.new(3, 44)
    assert_equal(3, o.one)
    assert_equal(44, o.two)
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    class Base
      var one = 1
    endclass
    class Child extends Base extends Base
      var two = 2
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1352: Duplicate "extends"', 5)

  lines =<< trim END
    vim9script
    class Child extends BaseClass
      var two = 2
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1353: Class name not found: BaseClass', 4)

  lines =<< trim END
    vim9script
    var SomeVar = 99
    class Child extends SomeVar
      var two = 2
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1354: Cannot extend SomeVar', 5)

  lines =<< trim END
    vim9script
    class Base
      var name: string
      def ToString(): string
        return this.name
      enddef
    endclass

    class Child extends Base
      var age: number
      def ToString(): string
        return super.ToString() .. ': ' .. this.age
      enddef
    endclass

    var o = Child.new('John', 42)
    assert_equal('John: 42', o.ToString())
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    class Child
      var age: number
      def ToString(): number
        return this.age
      enddef
      def ToString(): string
        return this.age
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: ToString', 9)

  lines =<< trim END
    vim9script
    class Child
      var age: number
      def ToString(): string
        return super .ToString() .. ': ' .. this.age
      enddef
    endclass
    var o = Child.new(42)
    echo o.ToString()
  END
  v9.CheckSourceFailure(lines, 'E1356: "super" must be followed by a dot', 1)

  lines =<< trim END
    vim9script
    class Base
      var name: string
      def ToString(): string
        return this.name
      enddef
    endclass

    var age = 42
    def ToString(): string
      return super.ToString() .. ': ' .. age
    enddef
    echo ToString()
  END
  v9.CheckSourceFailure(lines, 'E1357: Using "super" not in a class method', 1)

  lines =<< trim END
    vim9script
    class Child
      var age: number
      def ToString(): string
        return super.ToString() .. ': ' .. this.age
      enddef
    endclass
    var o = Child.new(42)
    echo o.ToString()
  END
  v9.CheckSourceFailure(lines, 'E1358: Using "super" not in a child class', 1)

  lines =<< trim END
    vim9script
    class Base
      var name: string
      static def ToString(): string
        return 'Base class'
      enddef
    endclass

    class Child extends Base
      var age: number
      def ToString(): string
        return Base.ToString() .. ': ' .. this.age
      enddef
    endclass

    var o = Child.new('John', 42)
    assert_equal('Base class: 42', o.ToString())
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    class Base
      var value = 1
      def new(init: number)
        this.value = number + 1
      enddef
    endclass
    class Child extends Base
      def new()
        this.new(3)
      enddef
    endclass
    var c = Child.new()
  END
  v9.CheckSourceFailure(lines, 'E1385: Class method "new" accessible only using class "Child"', 1)

  # base class with more than one object member
  lines =<< trim END
    vim9script

    class Result
      var success: bool
      var value: any = null
    endclass

    class Success extends Result
      def new(this.value = v:none)
        this.success = true
      enddef
    endclass

    var v = Success.new('asdf')
    assert_equal("object of Success {success: true, value: 'asdf'}", string(v))
  END
  v9.CheckSourceSuccess(lines)

  # class name after "extends" doesn't end in a space or NUL character
  lines =<< trim END
    vim9script
    class A
    endclass
    class B extends A"
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1315: White space required after name: A"', 4)
enddef

def Test_using_base_class()
  var lines =<< trim END
    vim9script

    class BaseEE
      def Enter(): any
        return null
      enddef
      def Exit(resource: any): void
      enddef
    endclass

    class ChildEE extends BaseEE
      def Enter(): any
        return 42
      enddef

      def Exit(resource: number): void
        g:result ..= '/exit'
      enddef
    endclass

    def With(ee: BaseEE)
      var r = ee.Enter()
      try
        g:result ..= r
      finally
        g:result ..= '/finally'
        ee.Exit(r)
      endtry
    enddef

    g:result = ''
    With(ChildEE.new())
    assert_equal('42/finally/exit', g:result)
  END
  v9.CheckSourceSuccess(lines)
  unlet g:result

  # Using super, Child invokes Base method which has optional arg. #12471
  lines =<< trim END
    vim9script

    class Base
      var success: bool = false
      def Method(arg = 0)
        this.success = true
      enddef
    endclass

    class Child extends Base
      def new()
        super.Method()
      enddef
    endclass

    var obj = Child.new()
    assert_equal(true, obj.success)
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_class_import()
  var lines =<< trim END
    vim9script
    export class Animal
      var kind: string
      var name: string
    endclass
  END
  writefile(lines, 'Xanimal.vim', 'D')

  lines =<< trim END
    vim9script
    import './Xanimal.vim' as animal

    var a: animal.Animal
    a = animal.Animal.new('fish', 'Eric')
    assert_equal('fish', a.kind)
    assert_equal('Eric', a.name)

    var b: animal.Animal = animal.Animal.new('cat', 'Garfield')
    assert_equal('cat', b.kind)
    assert_equal('Garfield', b.name)
  END
  v9.CheckScriptSuccess(lines)
enddef

" Test for implementing an imported interface
def Test_implement_imported_interface()
  var lines =<< trim END
    vim9script
    export interface Imp_Intf1
      def Fn1(): number
    endinterface
    export interface Imp_Intf2
      def Fn2(): number
    endinterface
  END
  writefile(lines, 'Ximportinterface.vim', 'D')

  lines =<< trim END
    vim9script
    import './Ximportinterface.vim' as Xintf

    class A implements Xintf.Imp_Intf1, Xintf.Imp_Intf2
      def Fn1(): number
        return 10
      enddef
      def Fn2(): number
        return 20
      enddef
    endclass
    var a = A.new()
    assert_equal(10, a.Fn1())
    assert_equal(20, a.Fn2())
  END
  v9.CheckScriptSuccess(lines)
enddef

" Test for extending an imported class
def Test_extend_imported_class()
  var lines =<< trim END
    vim9script
    export class Imp_C1
      def Fn1(): number
        return 5
      enddef
    endclass
  END
  writefile(lines, 'Xextendimportclass.vim', 'D')

  lines =<< trim END
    vim9script
    import './Xextendimportclass.vim' as XClass

    class A extends XClass.Imp_C1
    endclass
    var a = A.new()
    assert_equal(5, a.Fn1())
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_abstract_class()
  var lines =<< trim END
    vim9script
    abstract class Base
      var name: string
    endclass
    class Person extends Base
      var age: number
    endclass
    var p: Base = Person.new('Peter', 42)
    assert_equal('Peter', p.name)
    assert_equal(42, p.age)
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    abstract class Base
      var name: string
    endclass
    class Person extends Base
      var age: number
    endclass
    var p = Base.new('Peter')
  END
  v9.CheckSourceFailure(lines, 'E1325: Method "new" not found in class "Base"', 8)

  lines =<< trim END
    abstract class Base
      var name: string
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1316: Class can only be defined in Vim9 script', 1)

  # Abstract class cannot have a "new" function
  lines =<< trim END
    vim9script
    abstract class Base
      def new()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1359: Cannot define a "new" method in an abstract class', 4)
enddef

def Test_closure_in_class()
  var lines =<< trim END
    vim9script

    class Foo
      var y: list<string> = ['B']

      def new()
        g:result = filter(['A', 'B'], (_, v) => index(this.y, v) == -1)
      enddef
    endclass

    Foo.new()
    assert_equal(['A'], g:result)
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_construct_object_from_legacy()
  # Cannot directly invoke constructor from legacy
  var lines =<< trim END
    vim9script

    var newCalled = false

    class A
      def new(arg: string)
        newCalled = true
      enddef
    endclass

    export def CreateA(...args: list<any>): A
      return call(A.new, args)
    enddef

    g:P = CreateA
    legacy call g:P('some_arg')
    assert_equal(true, newCalled)
    unlet g:P
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    var newCalled = false

    class A
      static def CreateA(options = {}): any
        return A.new()
      enddef
      def new()
        newCalled = true
      enddef
    endclass

    g:P = A.CreateA
    legacy call g:P()
    assert_equal(true, newCalled)
    unlet g:P
  END
  v9.CheckSourceSuccess(lines)

  # This also tests invoking "new()" with "call"
  lines =<< trim END
    vim9script

    var createdObject: any

    class A
      var val1: number
      var val2: number
      static def CreateA(...args: list<any>): any
        createdObject = call(A.new, args)
        return createdObject
      enddef
    endclass

    g:P = A.CreateA
    legacy call g:P(3, 5)
    assert_equal(3, createdObject.val1)
    assert_equal(5, createdObject.val2)
    legacy call g:P()
    assert_equal(0, createdObject.val1)
    assert_equal(0, createdObject.val2)
    legacy call g:P(7)
    assert_equal(7, createdObject.val1)
    assert_equal(0, createdObject.val2)
    unlet g:P
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_defer_with_object()
  var lines =<< trim END
    vim9script

    class CWithEE
      def Enter()
        g:result ..= "entered/"
      enddef
      def Exit()
        g:result ..= "exited"
      enddef
    endclass

    def With(ee: CWithEE, F: func)
      ee.Enter()
      defer ee.Exit()
      F()
    enddef

    g:result = ''
    var obj = CWithEE.new()
    obj->With(() => {
      g:result ..= "called/"
    })
    assert_equal('entered/called/exited', g:result)
  END
  v9.CheckSourceSuccess(lines)
  unlet g:result

  lines =<< trim END
    vim9script

    class BaseWithEE
      def Enter()
        g:result ..= "entered-base/"
      enddef
      def Exit()
        g:result ..= "exited-base"
      enddef
    endclass

    class CWithEE extends BaseWithEE
      def Enter()
        g:result ..= "entered-child/"
      enddef
      def Exit()
        g:result ..= "exited-child"
      enddef
    endclass

    def With(ee: BaseWithEE, F: func)
      ee.Enter()
      defer ee.Exit()
      F()
    enddef

    g:result = ''
    var obj = CWithEE.new()
    obj->With(() => {
      g:result ..= "called/"
    })
    assert_equal('entered-child/called/exited-child', g:result)
  END
  v9.CheckSourceSuccess(lines)
  unlet g:result
enddef

" The following test used to crash Vim (Github issue #12676)
def Test_extends_method_crashes_vim()
  var lines =<< trim END
    vim9script

    class Observer
    endclass

    class Property
      var value: any

      def Set(v: any)
        if v != this.value
          this.value = v
        endif
      enddef

      def Register(observer: Observer)
      enddef
    endclass

    class Bool extends Property
      var value2: bool
    endclass

    def Observe(obj: Property, who: Observer)
      obj.Register(who)
    enddef

    var p = Bool.new(false)
    var myObserver = Observer.new()

    Observe(p, myObserver)

    p.Set(true)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for calling a method in a class that is extended
def Test_call_method_in_extended_class()
  var lines =<< trim END
    vim9script

    var prop_init_called = false
    var prop_register_called = false

    class Property
      def Init()
        prop_init_called = true
      enddef

      def Register()
        prop_register_called = true
      enddef
    endclass

    class Bool extends Property
    endclass

    def Observe(obj: Property)
      obj.Register()
    enddef

    var p = Property.new()
    Observe(p)

    p.Init()
    assert_true(prop_init_called)
    assert_true(prop_register_called)
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_instanceof()
  var lines =<< trim END
    vim9script

    class Base1
    endclass

    class Base2 extends Base1
    endclass

    interface Intf1
    endinterface

    class Mix1 implements Intf1
    endclass

    class Base3 extends Mix1
    endclass

    type AliasBase1 = Base1
    type AliasBase2 = Base2
    type AliasIntf1 = Intf1
    type AliasMix1 = Mix1

    var b1 = Base1.new()
    var b2 = Base2.new()
    var b3 = Base3.new()

    assert_true(instanceof(b1, Base1))
    assert_true(instanceof(b2, Base1))
    assert_false(instanceof(b1, Base2))
    assert_true(instanceof(b3, Mix1))
    assert_true(instanceof(b3, Base1, Base2, Intf1))

    assert_true(instanceof(b1, AliasBase1))
    assert_true(instanceof(b2, AliasBase1))
    assert_false(instanceof(b1, AliasBase2))
    assert_true(instanceof(b3, AliasMix1))
    assert_true(instanceof(b3, AliasBase1, AliasBase2, AliasIntf1))

    def Foo()
      var a1 = Base1.new()
      var a2 = Base2.new()
      var a3 = Base3.new()

      assert_true(instanceof(a1, Base1))
      assert_true(instanceof(a2, Base1))
      assert_false(instanceof(a1, Base2))
      assert_true(instanceof(a3, Mix1))
      assert_true(instanceof(a3, Base1, Base2, Intf1))

      assert_true(instanceof(a1, AliasBase1))
      assert_true(instanceof(a2, AliasBase1))
      assert_false(instanceof(a1, AliasBase2))
      assert_true(instanceof(a3, AliasMix1))
      assert_true(instanceof(a3, AliasBase1, AliasBase2, AliasIntf1))
    enddef
    Foo()

    var o_null: Base1
    assert_false(instanceof(o_null, Base1))

  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class Base1
    endclass
    instanceof(Base1.new())
  END
  v9.CheckSourceFailure(lines, 'E119: Not enough arguments for function: instanceof')

  lines =<< trim END
    vim9script

    class Base1
    endclass
    def F()
      instanceof(Base1.new())
    enddef
    F()
  END
  v9.CheckSourceFailure(lines, 'E119: Not enough arguments for function: instanceof')

  lines =<< trim END
    vim9script

    class Base1
    endclass

    class Base2
    endclass

    var o = Base2.new()
    instanceof(o, Base1, Base2, 3)
  END
  v9.CheckSourceFailure(lines, 'E693: Class or class typealias required for argument 4', 10)

  lines =<< trim END
    vim9script

    class Base1
    endclass

    class Base2
    endclass

    def F()
      var o = Base2.new()
      instanceof(o, Base1, Base2, 3)
    enddef
    F()
  END
  v9.CheckSourceFailure(lines, 'E693: Class or class typealias required for argument 4')
enddef

" Test for calling a method in the parent class that is extended partially.
" This used to fail with the 'E118: Too many arguments for function: Text' error
" message (Github issue #12524).
def Test_call_method_in_parent_class()
  var lines =<< trim END
    vim9script

    class Widget
      var _lnum: number = 1

      def SetY(lnum: number)
        this._lnum = lnum
      enddef

      def Text(): string
        return ''
      enddef
    endclass

    class Foo extends Widget
      def Text(): string
        return '<Foo>'
      enddef
    endclass

    def Stack(w1: Widget, w2: Widget): list<Widget>
      w1.SetY(1)
      w2.SetY(2)
      return [w1, w2]
    enddef

    var foo1 = Foo.new()
    var foo2 = Foo.new()
    var l = Stack(foo1, foo2)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for calling methods from three levels of classes
def Test_multi_level_method_call()
  var lines =<< trim END
    vim9script

    var A_func1: number = 0
    var A_func2: number = 0
    var A_func3: number = 0
    var B_func2: number = 0
    var B_func3: number = 0
    var C_func3: number = 0

    class A
      def Func1()
        A_func1 += 1
      enddef

      def Func2()
        A_func2 += 1
      enddef

      def Func3()
        A_func3 += 1
      enddef
    endclass

    class B extends A
      def Func2()
        B_func2 += 1
      enddef

      def Func3()
        B_func3 += 1
      enddef
    endclass

    class C extends B
      def Func3()
        C_func3 += 1
      enddef
    endclass

    def A_CallFuncs(a: A)
      a.Func1()
      a.Func2()
      a.Func3()
    enddef

    def B_CallFuncs(b: B)
      b.Func1()
      b.Func2()
      b.Func3()
    enddef

    def C_CallFuncs(c: C)
      c.Func1()
      c.Func2()
      c.Func3()
    enddef

    var cobj = C.new()
    A_CallFuncs(cobj)
    B_CallFuncs(cobj)
    C_CallFuncs(cobj)
    assert_equal(3, A_func1)
    assert_equal(0, A_func2)
    assert_equal(0, A_func3)
    assert_equal(3, B_func2)
    assert_equal(0, B_func3)
    assert_equal(3, C_func3)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using members from three levels of classes
def Test_multi_level_member_access()
  var lines =<< trim END
    vim9script

    class A
      public var val1: number = 0
    endclass

    class B extends A
      public var val2: number = 0
    endclass

    class C extends B
      public var val3: number = 0
    endclass

    def A_members(a: A)
      a.val1 += 1
    enddef

    def B_members(b: B)
      b.val1 += 1
      b.val2 += 1
    enddef

    def C_members(c: C)
      c.val1 += 1
      c.val2 += 1
      c.val3 += 1
    enddef

    var cobj = C.new()
    A_members(cobj)
    B_members(cobj)
    C_members(cobj)
    assert_equal(3, cobj.val1)
    assert_equal(2, cobj.val2)
    assert_equal(1, cobj.val3)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test expansion of <stack> with class methods.
def Test_stack_expansion_with_methods()
  var lines =<< trim END
    vim9script

    class C
      def M1()
        F0()
      enddef
    endclass

    def F0()
      assert_match('<SNR>\d\+_F\[1\]\.\.C\.M1\[1\]\.\.<SNR>\d\+_F0\[1\]$', expand('<stack>'))
    enddef

    def F()
      C.new().M1()
    enddef

    F()
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test the return type of the new() constructor
def Test_new_return_type()
  # new() uses the default return type and there is no return statement
  var lines =<< trim END
    vim9script

    class C
      var _bufnr: number

      def new(this._bufnr)
        if !bufexists(this._bufnr)
          this._bufnr = -1
        endif
      enddef
    endclass

    var c = C.new(12345)
    assert_equal('object<C>', typename(c))

    var v1: C
    v1 = C.new(12345)
    assert_equal('object<C>', typename(v1))

    def F()
      var v2: C
      v2 = C.new(12345)
      assert_equal('object<C>', typename(v2))
    enddef
    F()
  END
  v9.CheckSourceSuccess(lines)

  # new() uses the default return type and an empty 'return' statement
  lines =<< trim END
    vim9script

    class C
      var _bufnr: number

      def new(this._bufnr)
        if !bufexists(this._bufnr)
          this._bufnr = -1
          return
        endif
      enddef
    endclass

    var c = C.new(12345)
    assert_equal('object<C>', typename(c))

    var v1: C
    v1 = C.new(12345)
    assert_equal('object<C>', typename(v1))

    def F()
      var v2: C
      v2 = C.new(12345)
      assert_equal('object<C>', typename(v2))
    enddef
    F()
  END
  v9.CheckSourceSuccess(lines)

  # new() uses "any" return type and returns "this"
  lines =<< trim END
    vim9script

    class C
      var _bufnr: number

      def new(this._bufnr): any
        if !bufexists(this._bufnr)
          this._bufnr = -1
          return this
        endif
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1365: Cannot use a return type with the "new" method', 11)

  # new() uses 'Dict' return type and returns a Dict
  lines =<< trim END
    vim9script

    class C
      var _state: dict<any>

      def new(): dict<any>
        this._state = {}
        return this._state
      enddef
    endclass

    var c = C.new()
    assert_equal('object<C>', typename(c))
  END
  v9.CheckSourceFailure(lines, 'E1365: Cannot use a return type with the "new" method', 9)
enddef

" Test for checking a member initialization type at run time.
def Test_runtime_type_check_for_member_init()
  var lines =<< trim END
    vim9script

    var retnum: bool = false

    def F(): any
      retnum = !retnum
      if retnum
        return 1
      else
        return "hello"
      endif
    enddef

    class C
      var _foo: bool = F()
    endclass

    var c1 = C.new()
    var c2 = C.new()
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected bool but got string', 0)
enddef

" Test for locking a variable referring to an object and reassigning to another
" object.
def Test_lockvar_object()
  var lines =<< trim END
    vim9script

    class C
      var val: number
      def new(this.val)
      enddef
    endclass

    var some_dict: dict<C> = { a: C.new(1), b: C.new(2), c: C.new(3), }
    lockvar 2 some_dict

    var current: C
    current = some_dict['c']
    assert_equal(3, current.val)
    current = some_dict['b']
    assert_equal(2, current.val)

    def F()
      current = some_dict['c']
    enddef

    def G()
      current = some_dict['b']
    enddef

    F()
    assert_equal(3, current.val)
    G()
    assert_equal(2, current.val)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test trying to lock an object variable from various places
def Test_lockvar_object_variable()
  # An object variable lockvar has several cases:
  # object method, scriptlevel, scriplevel from :def, :def arg
  # method arg, static method arg.
  # Also different depths

  #
  # lockvar of read-only object variable
  #

  # read-only lockvar from object method
  var lines =<< trim END
    vim9script

    class C
      var val1: number
      def Lock()
        lockvar this.val1
      enddef
    endclass
    var o = C.new(3)
    o.Lock()
  END
  v9.CheckSourceFailure(lines, 'E1391: Cannot (un)lock variable "this.val1" in class "C"')

  # read-only lockvar from scriptlevel
  lines =<< trim END
    vim9script

    class C
      var val2: number
    endclass
    var o = C.new(3)
    lockvar o.val2
  END
  v9.CheckSourceFailure(lines, 'E1335: Variable "val2" in class "C" is not writable')

  # read-only lockvar of scriptlevel variable from def
  lines =<< trim END
    vim9script

    class C
      var val3: number
    endclass
    var o = C.new(3)
    def Lock()
      lockvar o.val3
    enddef
    Lock()
  END
  v9.CheckSourceFailure(lines, 'E1335: Variable "val3" in class "C" is not writable')

  # read-only lockvar of def argument variable
  lines =<< trim END
    vim9script

    class C
      var val4: number
    endclass
    def Lock(o: C)
      lockvar o.val4
    enddef
    Lock(C.new(3))
  END
  v9.CheckSourceFailure(lines, 'E1335: Variable "val4" in class "C" is not writable')

  # TODO: the following tests use type "any" for argument. Need a run time
  #       check for access. Probably OK as is for now.

  # read-only lockvar from object method arg
  lines =<< trim END
    vim9script

    class C
      var val5: number
      def Lock(o_any: any)
        lockvar o_any.val5
      enddef
    endclass
    var o = C.new(3)
    o.Lock(C.new(5))
  END
  v9.CheckSourceFailure(lines, 'E1391: Cannot (un)lock variable "o_any.val5" in class "C"')

  # read-only lockvar from class method arg
  lines =<< trim END
    vim9script

    class C
      var val6: number
      static def Lock(o_any: any)
        lockvar o_any.val6
      enddef
    endclass
    var o = C.new(3)
    C.Lock(o)
  END
  v9.CheckSourceFailure(lines, 'E1391: Cannot (un)lock variable "o_any.val6" in class "C"')

  #
  # lockvar of public object variable
  #

  # lockvar from object method
  lines =<< trim END
    vim9script

    class C
      public var val1: number
      def Lock()
        lockvar this.val1
      enddef
    endclass
    var o = C.new(3)
    o.Lock()
  END
  v9.CheckSourceFailure(lines, 'E1391: Cannot (un)lock variable "this.val1" in class "C"', 1)

  # lockvar from scriptlevel
  lines =<< trim END
    vim9script

    class C
      public var val2: number
    endclass
    var o = C.new(3)
    lockvar o.val2
  END
  v9.CheckSourceFailure(lines, 'E1391: Cannot (un)lock variable "o.val2" in class "C"', 7)

  # lockvar of scriptlevel variable from def
  lines =<< trim END
    vim9script

    class C
      public var val3: number
    endclass
    var o = C.new(3)
    def Lock()
      lockvar o.val3
    enddef
    Lock()
  END
  v9.CheckSourceFailure(lines, 'E1391: Cannot (un)lock variable "o.val3" in class "C"', 1)

  # lockvar of def argument variable
  lines =<< trim END
    vim9script

    class C
      public var val4: number
    endclass
    def Lock(o: C)
      lockvar o.val4
    enddef
    Lock(C.new(3))
  END
  v9.CheckSourceFailure(lines, 'E1391: Cannot (un)lock variable "o.val4" in class "C"', 1)

  # lockvar from object method arg
  lines =<< trim END
    vim9script

    class C
      public var val5: number
      def Lock(o_any: any)
        lockvar o_any.val5
      enddef
    endclass
    var o = C.new(3)
    o.Lock(C.new(5))
  END
  v9.CheckSourceFailure(lines, 'E1391: Cannot (un)lock variable "o_any.val5" in class "C"', 1)

  # lockvar from class method arg
  lines =<< trim END
    vim9script

    class C
      public var val6: number
      static def Lock(o_any: any)
        lockvar o_any.val6
      enddef
    endclass
    var o = C.new(3)
    C.Lock(o)
  END
  v9.CheckSourceFailure(lines, 'E1391: Cannot (un)lock variable "o_any.val6" in class "C"', 1)
enddef

" Test trying to lock a class variable from various places
def Test_lockvar_class_variable()

  # lockvar bare static from object method
  var lines =<< trim END
    vim9script

    class C
      public static var sval1: number
      def Lock()
        lockvar sval1
      enddef
    endclass
    var o = C.new()
    o.Lock()
  END
  v9.CheckSourceFailure(lines, 'E1392: Cannot (un)lock class variable "sval1" in class "C"', 1)

  # lockvar C.static from object method
  lines =<< trim END
    vim9script

    class C
      public static var sval2: number
      def Lock()
        lockvar C.sval2
      enddef
    endclass
    var o = C.new()
    o.Lock()
  END
  v9.CheckSourceFailure(lines, 'E1392: Cannot (un)lock class variable "C.sval2" in class "C"', 1)

  # lockvar bare static from class method
  lines =<< trim END
    vim9script

    class C
      public static var sval3: number
      static def Lock()
        lockvar sval3
      enddef
    endclass
    C.Lock()
  END
  v9.CheckSourceFailure(lines, 'E1392: Cannot (un)lock class variable "sval3" in class "C"', 1)

  # lockvar C.static from class method
  lines =<< trim END
    vim9script

    class C
      public static var sval4: number
      static def Lock()
        lockvar C.sval4
      enddef
    endclass
    C.Lock()
  END
  v9.CheckSourceFailure(lines, 'E1392: Cannot (un)lock class variable "C.sval4" in class "C"', 1)

  # lockvar C.static from script level
  lines =<< trim END
    vim9script

    class C
      public static var sval5: number
    endclass
    lockvar C.sval5
  END
  v9.CheckSourceFailure(lines, 'E1392: Cannot (un)lock class variable "C.sval5" in class "C"', 6)

  # lockvar o.static from script level
  lines =<< trim END
    vim9script

    class C
      public static var sval6: number
    endclass
    var o = C.new()
    lockvar o.sval6
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "sval6" accessible only using class "C"', 7)
enddef

" Test locking an argument to :def
def Test_lockvar_argument()
  # Lockvar a function arg
  var lines =<< trim END
    vim9script

    def Lock(val: any)
        lockvar val
    enddef

    var d = {a: 1, b: 2}
    Lock(d)

    d->extend({c: 3})
  END
  v9.CheckSourceFailure(lines, 'E741: Value is locked: extend() argument')

  # Lockvar a function arg. Verify "sval" is interpreted as argument and not a
  # class member in "C". This tests lval_root_is_arg.
  lines =<< trim END
    vim9script

    class C
      public static var sval: list<number>
    endclass

    def Lock2(sval: any)
      lockvar sval
    enddef

    var o = C.new()
    Lock2(o)
  END
  v9.CheckSourceSuccess(lines)

  # Lock a class.
  lines =<< trim END
    vim9script

    class C
      public static var sval: list<number>
    endclass

    def Lock2(sval: any)
      lockvar sval
    enddef

    Lock2(C)
  END
  v9.CheckSourceFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # Lock an object.
  lines =<< trim END
    vim9script

    class C
      public static var sval: list<number>
    endclass

    def Lock2(sval: any)
      lockvar sval
    enddef

    Lock2(C.new())
  END
  v9.CheckSourceSuccess(lines)

  # In this case (unlike previous) "lockvar sval" is a class member.
  lines =<< trim END
    vim9script

    class C
      public static var sval: list<number>
      def Lock2()
        lockvar sval
      enddef
    endclass


    var o = C.new()
    o.Lock2()
  END
  v9.CheckSourceFailure(lines, 'E1392: Cannot (un)lock class variable "sval" in class "C"', 1)
enddef

" Test that this can be locked without error
def Test_lockvar_this()
  # lockvar this
  var lines =<< trim END
    vim9script
    class C
      def TLock()
        lockvar this
      enddef
    endclass
    var o = C.new()
    o.TLock()
  END
  v9.CheckSourceSuccess(lines)

  # lockvar four   (four letter word, but not this)
  lines =<< trim END
    vim9script
    class C
      def TLock4()
        var four: number
        lockvar four
      enddef
    endclass
    var o = C.new()
    o.TLock4()
  END
  v9.CheckSourceFailure(lines, 'E1178: Cannot lock or unlock a local variable')

  # lockvar this5; "this" + one char, 5 letter word, starting with "this"
  lines =<< trim END
    vim9script
    class C
      def TLock5()
        var this5: number
        lockvar this5
      enddef
    endclass
    var o = C.new()
    o.TLock5()
  END
  v9.CheckSourceFailure(lines, 'E1178: Cannot lock or unlock a local variable')
enddef

" Test some general lockvar cases
def Test_lockvar_general()
  # lockvar an object and a class. It does nothing
  var lines =<< trim END
    vim9script
    class C
    endclass
    var o = C.new()
    lockvar o
    lockvar C
  END
  v9.CheckSourceSuccess(lines)

  # Lock a list element that's nested in an object variable from a :def
  lines =<< trim END
    vim9script

    class C
      public var val: list<list<number>> = [ [1], [2], [3] ]
    endclass
    def Lock2(obj: any)
      lockvar obj.val[1]
    enddef

    var o = C.new()
    Lock2(o)
    o.val[0] = [9]
    assert_equal([ [9], [2], [3] ], o.val)
    try
      o.val[1] = [999]
      call assert_false(true, 'assign should have failed')
    catch
      assert_exception('E741:')
    endtry
    o.val[2] = [8]
    assert_equal([ [9], [2], [8] ], o.val)
  END
  v9.CheckSourceSuccess(lines)

  # Lock a list element that's nested in an object variable from scriptlevel
  lines =<< trim END
    vim9script

    class C
      public var val: list<list<number>> = [ [1], [2], [3] ]
    endclass

    var o = C.new()
    lockvar o.val[1]
    o.val[0] = [9]
    assert_equal([ [9], [2], [3] ], o.val)
    try
      o.val[1] = [999]
      call assert_false(true, 'assign should have failed')
    catch
      assert_exception('E741:')
    endtry
    o.val[2] = [8]
    assert_equal([ [9], [2], [8] ], o.val)
  END
  v9.CheckSourceSuccess(lines)

  # lock a script level variable from an object method
  lines =<< trim END
    vim9script

    class C
      def Lock()
        lockvar l
      enddef
    endclass

    var l = [1]
    C.new().Lock()
    l[0] = 11
  END
  v9.CheckSourceFailure(lines, 'E741: Value is locked: l[0] = 11', 11)

  # lock a list element referenced by a protected object variable
  # in an object fetched via a script level list
  lines =<< trim END
    vim9script

    class C
      var _v1: list<list<number>>
      def Lock()
        lockvar lc[0]._v1[1]
      enddef
    endclass

    var l = [[1], [2], [3]]
    var o = C.new(l)
    var lc: list<C> = [ o ]

    o.Lock()
    l[0] = [22]
    l[1] = [33]
  END
  v9.CheckSourceFailure(lines, 'E741: Value is locked: l[1] = [33]', 16)

  # similar to the previous test, except the locking code is executing
  # in a class that does not own the protected variable.
  # Note that the locking code is in a class has a protected variable of
  # the same name.
  lines =<< trim END
    vim9script

    class C2
      var _v1: list<list<number>>
      def Lock(obj: any)
        lockvar lc[0]._v1[1]
      enddef
    endclass

    class C
      var _v1: list<list<number>>
    endclass

    var l = [[1], [2], [3]]
    var o = C.new(l)
    var lc: list<C> = [ o ]

    var o2 = C2.new()
    o2.Lock(o)
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_v1" in class "C"')
enddef

" Test builtin islocked()
def Test_lockvar_islocked()
  # Can't lock class/object variable
  # Lock class/object variable's value
  # Lock item of variable's value (a list item)
  # variable is at index 1 within class/object
  var lines =<< trim END
    vim9script

    class C
      var o0: list<list<number>> = [ [0],  [1],  [2]]
      var o1: list<list<number>> = [[10], [11], [12]]
      static var c0: list<list<number>> = [[20], [21], [22]]
      static var c1: list<list<number>> = [[30], [31], [32]]
    endclass

    def LockIt(arg: any)
      lockvar arg
    enddef

    def UnlockIt(arg: any)
      unlockvar arg
    enddef

    var obj = C.new()
    #lockvar obj.o1         # can't lock something you can't write to

    try
      lockvar obj.o1         # can't lock something you can't write to
      call assert_false(1, '"lockvar obj.o1" should have failed')
    catch
      call assert_exception('E1335:')
    endtry

    LockIt(obj.o1)         # but can lock it's value
    assert_equal(1, islocked("obj.o1"))
    assert_equal(1, islocked("obj.o1[0]"))
    assert_equal(1, islocked("obj.o1[1]"))
    UnlockIt(obj.o1)
    assert_equal(0, islocked("obj.o1"))
    assert_equal(0, islocked("obj.o1[0]"))

    lockvar obj.o1[0]
    assert_equal(0, islocked("obj.o1"))
    assert_equal(1, islocked("obj.o1[0]"))
    assert_equal(0, islocked("obj.o1[1]"))
    unlockvar obj.o1[0]
    assert_equal(0, islocked("obj.o1"))
    assert_equal(0, islocked("obj.o1[0]"))

    # Same thing, but with a static

    try
      lockvar C.c1         # can't lock something you can't write to
      call assert_false(1, '"lockvar C.c1" should have failed')
    catch
      call assert_exception('E1335:')
    endtry

    LockIt(C.c1)         # but can lock it's value
    assert_equal(1, islocked("C.c1"))
    assert_equal(1, islocked("C.c1[0]"))
    assert_equal(1, islocked("C.c1[1]"))
    UnlockIt(C.c1)
    assert_equal(0, islocked("C.c1"))
    assert_equal(0, islocked("C.c1[0]"))

    lockvar C.c1[0]
    assert_equal(0, islocked("C.c1"))
    assert_equal(1, islocked("C.c1[0]"))
    assert_equal(0, islocked("C.c1[1]"))
    unlockvar C.c1[0]
    assert_equal(0, islocked("C.c1"))
    assert_equal(0, islocked("C.c1[0]"))
  END
  v9.CheckSourceSuccess(lines)

  # Do islocked() from an object method
  # and then from a class method
  lines =<< trim END
    vim9script

    var l0o0 = [  [0],   [1],   [2]]
    var l0o1 = [ [10],  [11],  [12]]
    var l0c0 = [[120], [121], [122]]
    var l0c1 = [[130], [131], [132]]

    class C0
      var o0: list<list<number>> =   l0o0
      var o1: list<list<number>> =   l0o1
      static var c0: list<list<number>> = l0c0
      static var c1: list<list<number>> = l0c1
      def Islocked(arg: string): number
          return islocked(arg)
      enddef
      static def SIslocked(arg: string): number
        return islocked(arg)
      enddef
    endclass

    var l2o0 = [[20000], [20001], [20002]]
    var l2o1 = [[20010], [20011], [20012]]
    var l2c0 = [[20120], [20121], [20122]]
    var l2c1 = [[20130], [20131], [20132]]

    class C2
      var o0: list<list<number>> =   l2o0
      var o1: list<list<number>> =   l2o1
      static var c0: list<list<number>> = l2c0
      static var c1: list<list<number>> = l2c1
      def Islocked(arg: string): number
          return islocked(arg)
      enddef
      static def SIslocked(arg: string): number
        return islocked(arg)
      enddef
    endclass

    var obj0 = C0.new()
    var obj2 = C2.new()

    var l = [ obj0, null_object, obj2 ]

    # lock list, object func access through script var expr
    assert_equal(0, obj0.Islocked("l[0].o0"))
    assert_equal(0, obj0.Islocked("l[0].o0[2]"))
    lockvar l0o0
    assert_equal(1, obj0.Islocked("l[0].o0"))
    assert_equal(1, obj0.Islocked("l[0].o0[2]"))

    #echo "check-b" obj2.Islocked("l[1].o1")    # NULL OBJECT

    # lock list element, object func access through script var expr
    lockvar l0o1[1]
    assert_equal(0, obj0.Islocked("this.o1[0]"))
    assert_equal(1, obj0.Islocked("this.o1[1]"))

    assert_equal(0, obj0.Islocked("this.o1"))
    lockvar l0o1
    assert_equal(1, obj0.Islocked("this.o1"))
    unlockvar l0o1

    lockvar l0c1[1]

    # static by class name member expr from same class
    assert_equal(0, obj0.Islocked("C0.c1[0]"))
    assert_equal(1, obj0.Islocked("C0.c1[1]"))
    # static by bare name member expr from same class
    assert_equal(0, obj0.Islocked("c1[0]"))
    assert_equal(1, obj0.Islocked("c1[1]"))

    # static by class name member expr from other class
    assert_equal(0, obj2.Islocked("C0.c1[0]"))
    assert_equal(1, obj2.Islocked("C0.c1[1]"))
    # static by bare name member expr from other class
    assert_equal(0, obj2.Islocked("c1[0]"))
    assert_equal(0, obj2.Islocked("c1[1]"))


    # static by bare name in same class
    assert_equal(0, obj0.Islocked("c0"))
    lockvar l0c0
    assert_equal(1, obj0.Islocked("c0"))

    #
    # similar stuff, but use static method
    #

    unlockvar l0o0

    # lock list, object func access through script var expr
    assert_equal(0, C0.SIslocked("l[0].o0"))
    assert_equal(0, C0.SIslocked("l[0].o0[2]"))
    lockvar l0o0
    assert_equal(1, C0.SIslocked("l[0].o0"))
    assert_equal(1, C0.SIslocked("l[0].o0[2]"))

    unlockvar l0o1

    # can't access "this" from class method
    try
      C0.SIslocked("this.o1[0]")
      call assert_0(1, '"C0.SIslocked("this.o1[0]")" should have failed')
    catch
      call assert_exception('E121: Undefined variable: this')
    endtry

    lockvar l0c1[1]

    # static by class name member expr from same class
    assert_equal(0, C0.SIslocked("C0.c1[0]"))
    assert_equal(1, C0.SIslocked("C0.c1[1]"))
    # static by bare name member expr from same class
    assert_equal(0, C0.SIslocked("c1[0]"))
    assert_equal(1, C0.SIslocked("c1[1]"))

    # static by class name member expr from other class
    assert_equal(0, C2.SIslocked("C0.c1[0]"))
    assert_equal(1, C2.SIslocked("C0.c1[1]"))
    # static by bare name member expr from other class
    assert_equal(0, C2.SIslocked("c1[0]"))
    assert_equal(0, C2.SIslocked("c1[1]"))


    # static by bare name in same class
    unlockvar l0c0
    assert_equal(0, C0.SIslocked("c0"))
    lockvar l0c0
    assert_equal(1, C0.SIslocked("c0"))
  END
  v9.CheckSourceSuccess(lines)

  # Check islocked class/object from various places.
  lines =<< trim END
    vim9script

    class C
      def Islocked(arg: string): number
        return islocked(arg)
      enddef
      static def SIslocked(arg: string): number
        return islocked(arg)
      enddef
    endclass
    var obj = C.new()

    # object method
    assert_equal(0, obj.Islocked("this"))
    assert_equal(0, obj.Islocked("C"))

    # class method
    ### assert_equal(0, C.SIslocked("this"))
    assert_equal(0, C.SIslocked("C"))

    #script level
    var v: number
    v = islocked("C")
    assert_equal(0, v)
    v = islocked("obj")
    assert_equal(0, v)
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_lockvar_islocked_notfound()
  # Try non-existent things
  var lines =<< trim END
    vim9script

    class C
      def Islocked(arg: string): number
          return islocked(arg)
      enddef
      static def SIslocked(arg: string): number
        return islocked(arg)
      enddef
    endclass
    var obj = C.new()
    assert_equal(-1, obj.Islocked("anywhere"))
    assert_equal(-1, C.SIslocked("notanywhere"))
  END
  v9.CheckSourceSuccess(lines)

  # Something not found of the form "name1.name2" is an error
  lines =<< trim END
    vim9script

    islocked("one.two")
  END
  v9.CheckSourceFailure(lines, 'E121: Undefined variable: one')

  lines =<< trim END
    vim9script

    class C
      var val = { key: "value" }
      def Islocked(arg: string): number
          return islocked(arg)
      enddef
    endclass
    var obj = C.new()
    obj.Islocked("this.val.not_there"))
  END
  v9.CheckSourceFailure(lines, 'E716: Key not present in Dictionary: "not_there"')

  lines =<< trim END
    vim9script

    class C
      def Islocked(arg: string): number
          return islocked(arg)
      enddef
    endclass
    var obj = C.new()
    obj.Islocked("this.notobjmember")
  END
  v9.CheckSourceFailure(lines, 'E1326: Variable "notobjmember" not found in object "C"')

  # access a script variable through methods
  lines =<< trim END
    vim9script

    var l = [1]
    class C
      def Islocked(arg: string): number
          return islocked(arg)
      enddef
      static def SIslocked(arg: string): number
        return islocked(arg)
      enddef
    endclass
    var obj = C.new()
    assert_equal(0, obj.Islocked("l"))
    assert_equal(0, C.SIslocked("l"))
    lockvar l
    assert_equal(1, obj.Islocked("l"))
    assert_equal(1, C.SIslocked("l"))
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for a protected object method
def Test_private_object_method()
  # Try calling a protected method using an object (at the script level)
  var lines =<< trim END
    vim9script

    class A
      def _Foo(): number
        return 1234
      enddef
    endclass
    var a = A.new()
    a._Foo()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 9)

  # Try calling a protected method using an object (from a def function)
  lines =<< trim END
    vim9script

    class A
      def _Foo(): number
        return 1234
      enddef
    endclass
    def T()
      var a = A.new()
      a._Foo()
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo()', 2)

  # Use a protected method from another object method (in script context)
  lines =<< trim END
    vim9script

    class A
      def _Foo(): number
        return 1234
      enddef
      def Bar(): number
        return this._Foo()
      enddef
    endclass
    var a = A.new()
    assert_equal(1234, a.Bar())
  END
  v9.CheckSourceSuccess(lines)

  # Use a protected method from another object method (def function context)
  lines =<< trim END
    vim9script

    class A
      def _Foo(): number
        return 1234
      enddef
      def Bar(): number
        return this._Foo()
      enddef
    endclass
    def T()
      var a = A.new()
      assert_equal(1234, a.Bar())
    enddef
    T()
  END
  v9.CheckSourceSuccess(lines)

  # Try calling a protected method without the "this" prefix
  lines =<< trim END
    vim9script

    class A
      def _Foo(): number
        return 1234
      enddef
      def Bar(): number
        return _Foo()
      enddef
    endclass
    var a = A.new()
    a.Bar()
  END
  v9.CheckSourceFailure(lines, 'E117: Unknown function: _Foo', 1)

  # Try calling a protected method using the class name
  lines =<< trim END
    vim9script

    class A
      def _Foo(): number
        return 1234
      enddef
    endclass
    A._Foo()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 8)

  # Define two protected methods with the same name
  lines =<< trim END
    vim9script

    class A
      def _Foo()
      enddef
      def _Foo()
      enddef
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: _Foo', 7)

  # Define a protected method and a object method with the same name
  lines =<< trim END
    vim9script

    class A
      def _Foo()
      enddef
      def Foo()
      enddef
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: Foo', 7)

  # Define an object method and a protected method with the same name
  lines =<< trim END
    vim9script

    class A
      def Foo()
      enddef
      def _Foo()
      enddef
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: _Foo', 7)

  # Call a public method and a protected method from a protected method
  lines =<< trim END
    vim9script

    class A
      def Foo(): number
        return 100
      enddef
      def _Bar(): number
        return 200
      enddef
      def _Baz()
        assert_equal(100, this.Foo())
        assert_equal(200, this._Bar())
      enddef
      def T()
        this._Baz()
      enddef
    endclass
    var a = A.new()
    a.T()
  END
  v9.CheckSourceSuccess(lines)

  # Try calling a protected method from another class
  lines =<< trim END
    vim9script

    class A
      def _Foo(): number
        return 100
      enddef
    endclass
    class B
      def Foo(): number
        var a = A.new()
        a._Foo()
      enddef
    endclass
    var b = B.new()
    b.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo()', 2)

  # Call a protected object method from a child class object method
  lines =<< trim END
    vim9script
    class A
      def _Foo(): number
        return 1234
      enddef
    endclass
    class B extends A
      def Bar()
      enddef
    endclass
    class C extends B
      def Baz(): number
        return this._Foo()
      enddef
    endclass
    var c = C.new()
    assert_equal(1234, c.Baz())
  END
  v9.CheckSourceSuccess(lines)

  # Call a protected object method from a child class object
  lines =<< trim END
    vim9script
    class A
      def _Foo(): number
        return 1234
      enddef
    endclass
    class B extends A
      def Bar()
      enddef
    endclass
    class C extends B
      def Baz(): number
      enddef
    endclass
    var c = C.new()
    assert_equal(1234, c._Foo())
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 16)

  # Using "_" prefix in a method name should fail outside of a class
  lines =<< trim END
    vim9script
    def _Foo(): number
      return 1234
    enddef
    var a = _Foo()
  END
  v9.CheckSourceFailure(lines, 'E1267: Function name must start with a capital: _Foo(): number', 2)
enddef

" Test for an protected class method
def Test_private_class_method()
  # Try calling a class protected method (at the script level)
  var lines =<< trim END
    vim9script

    class A
      static def _Foo(): number
        return 1234
      enddef
    endclass
    A._Foo()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 8)

  # Try calling a class protected method (from a def function)
  lines =<< trim END
    vim9script

    class A
      static def _Foo(): number
        return 1234
      enddef
    endclass
    def T()
      A._Foo()
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo()', 1)

  # Try calling a class protected method using an object (at the script level)
  lines =<< trim END
    vim9script

    class A
      static def _Foo(): number
        return 1234
      enddef
    endclass
    var a = A.new()
    a._Foo()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 9)

  # Try calling a class protected method using an object (from a def function)
  lines =<< trim END
    vim9script

    class A
      static def _Foo(): number
        return 1234
      enddef
    endclass
    def T()
      var a = A.new()
      a._Foo()
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 2)

  # Use a class protected method from an object method
  lines =<< trim END
    vim9script

    class A
      static def _Foo(): number
        return 1234
      enddef
      def Bar()
        assert_equal(1234, _Foo())
      enddef
    endclass
    var a = A.new()
    a.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Use a class protected method from another class protected method without the
  # class name prefix.
  lines =<< trim END
    vim9script

    class A
      static def _Foo1(): number
        return 1234
      enddef
      static def _Foo2()
        assert_equal(1234, _Foo1())
      enddef
      def Bar()
        _Foo2()
      enddef
    endclass
    var a = A.new()
    a.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Declare a class method and a class protected method with the same name
  lines =<< trim END
    vim9script

    class A
      static def _Foo()
      enddef
      static def Foo()
      enddef
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: Foo', 7)

  # Try calling a class protected method from another class
  lines =<< trim END
    vim9script

    class A
      static def _Foo(): number
        return 1234
      enddef
    endclass
    class B
      def Foo(): number
        return A._Foo()
      enddef
    endclass
    var b = B.new()
    assert_equal(1234, b.Foo())
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo()', 1)

  # Call a protected class method from a child class object method
  lines =<< trim END
    vim9script
    class A
      static def _Foo(): number
        return 1234
      enddef
    endclass
    class B extends A
      def Bar()
      enddef
    endclass
    class C extends B
      def Baz(): number
        return A._Foo()
      enddef
    endclass
    var c = C.new()
    assert_equal(1234, c.Baz())
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo()', 1)

  # Call a protected class method from a child class protected class method
  lines =<< trim END
    vim9script
    class A
      static def _Foo(): number
        return 1234
      enddef
    endclass
    class B extends A
      def Bar()
      enddef
    endclass
    class C extends B
      static def Baz(): number
        return A._Foo()
      enddef
    endclass
    assert_equal(1234, C.Baz())
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo()', 1)

  # Call a protected class method from a child class object
  lines =<< trim END
    vim9script
    class A
      static def _Foo(): number
        return 1234
      enddef
    endclass
    class B extends A
      def Bar()
      enddef
    endclass
    class C extends B
      def Baz(): number
      enddef
    endclass
    var c = C.new()
    assert_equal(1234, C._Foo())
  END
  v9.CheckSourceFailure(lines, 'E1325: Method "_Foo" not found in class "C"', 16)
enddef

" Test for using the return value of a class/object method as a function
" argument.
def Test_objmethod_funcarg()
  var lines =<< trim END
    vim9script

    class C
      def Foo(): string
        return 'foo'
      enddef
    endclass

    def Bar(a: number, s: string): string
      return s
    enddef

    def Baz(c: C)
      assert_equal('foo', Bar(10, c.Foo()))
    enddef

    var t = C.new()
    Baz(t)
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class C
      static def Foo(): string
        return 'foo'
      enddef
    endclass

    def Bar(a: number, s: string): string
      return s
    enddef

    def Baz()
      assert_equal('foo', Bar(10, C.Foo()))
    enddef

    Baz()
  END
  v9.CheckSourceSuccess(lines)
enddef

def Test_static_inheritence()
  # subclasses get their own static copy
  var lines =<< trim END
    vim9script

    class A
      static var _svar: number
      var _mvar: number
      def new()
        _svar = 1
        this._mvar = 101
      enddef
      def AccessObject(): number
        return this._mvar
      enddef
      def AccessStaticThroughObject(): number
        return _svar
      enddef
    endclass

    class B extends A
      def new()
        this._mvar = 102
      enddef
    endclass

    class C extends B
      def new()
        this._mvar = 103
      enddef

      def AccessPrivateStaticThroughClassName(): number
        assert_equal(1, A._svar)
        return 444
      enddef
    endclass

    var oa = A.new()
    var ob = B.new()
    var oc = C.new()
    assert_equal(101, oa.AccessObject())
    assert_equal(102, ob.AccessObject())
    assert_equal(103, oc.AccessObject())

    assert_fails('echo oc.AccessPrivateStaticThroughClassName()', 'E1333: Cannot access protected variable "_svar" in class "A"')

    # verify object properly resolves to correct static
    assert_equal(1, oa.AccessStaticThroughObject())
    assert_equal(1, ob.AccessStaticThroughObject())
    assert_equal(1, oc.AccessStaticThroughObject())
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for declaring duplicate object and class members
def Test_dup_member_variable()
  # Duplicate member variable
  var lines =<< trim END
    vim9script
    class C
      var val = 10
      var val = 20
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: val', 4)

  # Duplicate protected member variable
  lines =<< trim END
    vim9script
    class C
      var _val = 10
      var _val = 20
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: _val', 4)

  # Duplicate public member variable
  lines =<< trim END
    vim9script
    class C
      public var val = 10
      public var val = 20
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: val', 4)

  # Duplicate protected member variable
  lines =<< trim END
    vim9script
    class C
      var val = 10
      var _val = 20
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: _val', 4)

  # Duplicate public and protected member variable
  lines =<< trim END
    vim9script
    class C
      var _val = 20
      public var val = 10
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: val', 4)

  # Duplicate class member variable
  lines =<< trim END
    vim9script
    class C
      static var s: string = "abc"
      static var _s: string = "def"
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: _s', 4)

  # Duplicate public and protected class member variable
  lines =<< trim END
    vim9script
    class C
      public static var s: string = "abc"
      static var _s: string = "def"
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: _s', 4)

  # Duplicate class and object member variable
  lines =<< trim END
    vim9script
    class C
      static var val = 10
      var val = 20
      def new()
      enddef
    endclass
    var c = C.new()
    assert_equal(10, C.val)
    assert_equal(20, c.val)
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: val', 4)

  # Duplicate object member variable in a derived class
  lines =<< trim END
    vim9script
    class A
      var val = 10
    endclass
    class B extends A
    endclass
    class C extends B
      var val = 20
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: val', 9)

  # Duplicate object protected member variable in a derived class
  lines =<< trim END
    vim9script
    class A
      var _val = 10
    endclass
    class B extends A
    endclass
    class C extends B
      var _val = 20
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: _val', 9)

  # Duplicate object protected member variable in a derived class
  lines =<< trim END
    vim9script
    class A
      var val = 10
    endclass
    class B extends A
    endclass
    class C extends B
      var _val = 20
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: _val', 9)

  # Duplicate object member variable in a derived class
  lines =<< trim END
    vim9script
    class A
      var _val = 10
    endclass
    class B extends A
    endclass
    class C extends B
      var val = 20
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: val', 9)

  # Two member variables with a common prefix
  lines =<< trim END
    vim9script
    class A
      public static var svar2: number
      public static var svar: number
    endclass
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for accessing a protected member outside a class in a def function
def Test_private_member_access_outside_class()
  # protected object member variable
  var lines =<< trim END
    vim9script
    class A
      var _val = 10
      def GetVal(): number
        return this._val
      enddef
    endclass
    def T()
      var a = A.new()
      a._val = 20
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_val" in class "A"', 2)

  # access a non-existing protected object member variable
  lines =<< trim END
    vim9script
    class A
      var _val = 10
    endclass
    def T()
      var a = A.new()
      a._a = 1
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1326: Variable "_a" not found in object "A"', 2)

  # protected static member variable
  lines =<< trim END
    vim9script
    class A
      static var _val = 10
    endclass
    def T()
      var a = A.new()
      var x = a._val
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "_val" accessible only using class "A"', 2)

  # protected static member variable
  lines =<< trim END
    vim9script
    class A
      static var _val = 10
    endclass
    def T()
      var a = A.new()
      a._val = 3
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "_val" accessible only using class "A"', 2)

  # protected static class variable
  lines =<< trim END
    vim9script
    class A
      static var _val = 10
    endclass
    def T()
      var x = A._val
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_val" in class "A"', 1)

  # protected static class variable
  lines =<< trim END
    vim9script
    class A
      static var _val = 10
    endclass
    def T()
      A._val = 3
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1333: Cannot access protected variable "_val" in class "A"', 1)
enddef

" Test for changing the member access of an interface in a implementation class
def Test_change_interface_member_access()
  var lines =<< trim END
    vim9script
    interface A
      var val: number
    endinterface
    class B implements A
      public var val = 10
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1367: Access level of variable "val" of interface "A" is different', 7)

  lines =<< trim END
    vim9script
    interface A
      var val: number
    endinterface
    class B implements A
      public var val = 10
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1367: Access level of variable "val" of interface "A" is different', 7)
enddef

" Test for trying to change a readonly member from a def function
def Test_readonly_member_change_in_def_func()
  var lines =<< trim END
    vim9script
    class A
      var val: number
    endclass
    def T()
      var a = A.new()
      a.val = 20
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1335: Variable "val" in class "A" is not writable', 2)
enddef

" Test for reading and writing a class member from a def function
def Test_modify_class_member_from_def_function()
  var lines =<< trim END
    vim9script
    class A
      var var1: number = 10
      public static var var2: list<number> = [1, 2]
      public static var var3: dict<number> = {a: 1, b: 2}
      static var _priv_var4: number = 40
    endclass
    def T()
      assert_equal([1, 2], A.var2)
      assert_equal({a: 1, b: 2}, A.var3)
      A.var2 = [3, 4]
      A.var3 = {c: 3, d: 4}
      assert_equal([3, 4], A.var2)
      assert_equal({c: 3, d: 4}, A.var3)
      assert_fails('echo A._priv_var4', 'E1333: Cannot access protected variable "_priv_var4" in class "A"')
    enddef
    T()
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for accessing a class member variable using an object
def Test_class_variable_access_using_object()
  var lines =<< trim END
    vim9script
    class A
      public static var svar1: list<number> = [1]
      public static var svar2: list<number> = [2]
    endclass

    A.svar1->add(3)
    A.svar2->add(4)
    assert_equal([1, 3], A.svar1)
    assert_equal([2, 4], A.svar2)

    def Foo()
      A.svar1->add(7)
      A.svar2->add(8)
      assert_equal([1, 3, 7], A.svar1)
      assert_equal([2, 4, 8], A.svar2)
    enddef
    Foo()
  END
  v9.CheckSourceSuccess(lines)

  # Cannot read from a class variable using an object in script context
  lines =<< trim END
    vim9script
    class A
      public var var1: number
      public static var svar2: list<number> = [1]
    endclass

    var a = A.new()
    echo a.svar2
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "svar2" accessible only using class "A"', 8)

  # Cannot write to a class variable using an object in script context
  lines =<< trim END
    vim9script
    class A
      public var var1: number
      public static var svar2: list<number> = [1]
    endclass

    var a = A.new()
    a.svar2 = [2]
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "svar2" accessible only using class "A"', 8)

  # Cannot read from a class variable using an object in def method context
  lines =<< trim END
    vim9script
    class A
      public var var1: number
      public static var svar2: list<number> = [1]
    endclass

    def T()
      var a = A.new()
      echo a.svar2
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "svar2" accessible only using class "A"', 2)

  # Cannot write to a class variable using an object in def method context
  lines =<< trim END
    vim9script
    class A
      public var var1: number
      public static var svar2: list<number> = [1]
    endclass

    def T()
      var a = A.new()
      a.svar2 = [2]
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "svar2" accessible only using class "A"', 2)
enddef

" Test for using a interface method using a child object
def Test_interface_method_from_child()
  var lines =<< trim END
    vim9script

    interface A
      def Foo(): string
    endinterface

    class B implements A
      def Foo(): string
        return 'foo'
      enddef
    endclass

    class C extends B
      def Bar(): string
        return 'bar'
      enddef
    endclass

    def T1(a: A)
      assert_equal('foo', a.Foo())
    enddef

    def T2(b: B)
      assert_equal('foo', b.Foo())
    enddef

    var c = C.new()
    T1(c)
    T2(c)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using an interface method using a child object when it is overridden
" by the child class.
" FIXME: This test fails.
" def Test_interface_overridden_method_from_child()
"   var lines =<< trim END
"     vim9script
"
"     interface A
"       def Foo(): string
"     endinterface
"
"     class B implements A
"       def Foo(): string
"         return 'b-foo'
"       enddef
"     endclass
"
"     class C extends B
"       def Bar(): string
"         return 'bar'
"       enddef
"       def Foo(): string
"         return 'c-foo'
"       enddef
"     endclass
"
"     def T1(a: A)
"       assert_equal('c-foo', a.Foo())
"     enddef
"
"     def T2(b: B)
"       assert_equal('c-foo', b.Foo())
"     enddef
"
"     var c = C.new()
"     T1(c)
"     T2(c)
"   END
"   v9.CheckSourceSuccess(lines)
" enddef

" Test for abstract methods
def Test_abstract_method()
  # Use two abstract methods
  var lines =<< trim END
    vim9script
    abstract class A
      def M1(): number
        return 10
      enddef
      abstract def M2(): number
      abstract def M3(): number
    endclass
    class B extends A
      def M2(): number
        return 20
      enddef
      def M3(): number
        return 30
      enddef
    endclass
    var b = B.new()
    assert_equal([10, 20, 30], [b.M1(), b.M2(), b.M3()])
  END
  v9.CheckSourceSuccess(lines)

  # Don't define an abstract method
  lines =<< trim END
    vim9script
    abstract class A
      abstract def Foo()
    endclass
    class B extends A
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1373: Abstract method "Foo" is not implemented', 6)

  # Use abstract method in a concrete class
  lines =<< trim END
    vim9script
    class A
      abstract def Foo()
    endclass
    class B extends A
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1372: Abstract method "abstract def Foo()" cannot be defined in a concrete class', 3)

  # Use abstract method in an interface
  lines =<< trim END
    vim9script
    interface A
      abstract def Foo()
    endinterface
    class B implements A
      def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1404: Abstract cannot be used in an interface', 3)

  # Use abstract static method in an interface
  lines =<< trim END
    vim9script
    interface A
      abstract static def Foo()
      enddef
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1404: Abstract cannot be used in an interface', 3)

  # Use abstract static variable in an interface
  lines =<< trim END
    vim9script
    interface A
      abstract static foo: number = 10
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1404: Abstract cannot be used in an interface', 3)

  # Abbreviate the "abstract" keyword
  lines =<< trim END
    vim9script
    class A
      abs def Foo()
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1065: Command cannot be shortened: abs def Foo()', 3)

  # Use "abstract" with a member variable
  lines =<< trim END
    vim9script
    abstract class A
      abstract this.val = 10
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1371: Abstract must be followed by "def"', 3)

  # Use a static abstract method
  lines =<< trim END
    vim9script
    abstract class A
      abstract static def Foo(): number
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1371: Abstract must be followed by "def"', 3)

  # Type mismatch between abstract method and concrete method
  lines =<< trim END
    vim9script
    abstract class A
      abstract def Foo(a: string, b: number): list<number>
    endclass
    class B extends A
      def Foo(a: number, b: string): list<string>
        return []
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1383: Method "Foo": type mismatch, expected func(string, number): list<number> but got func(number, string): list<string>', 9)

  # Invoke an abstract method from a def function
  lines =<< trim END
    vim9script
    abstract class A
      abstract def Foo(): list<number>
    endclass
    class B extends A
      def Foo(): list<number>
        return [3, 5]
      enddef
    endclass
    def Bar(c: B)
      assert_equal([3, 5], c.Foo())
    enddef
    var b = B.new()
    Bar(b)
  END
  v9.CheckSourceSuccess(lines)

  # Use a static method in an abstract class
  lines =<< trim END
    vim9script
    abstract class A
      static def Foo(): string
        return 'foo'
      enddef
    endclass
    assert_equal('foo', A.Foo())
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for calling a class method from a subclass
def Test_class_method_call_from_subclass()
  # class method call from a subclass
  var lines =<< trim END
    vim9script

    class A
      static def Foo()
        echo "foo"
      enddef
    endclass

    class B extends A
      def Bar()
        Foo()
      enddef
    endclass

    var b = B.new()
    b.Bar()
  END
  v9.CheckSourceFailure(lines, 'E1384: Class method "Foo" accessible only inside class "A"', 1)
enddef

" Test for calling a class method using an object in a def function context and
" script context.
def Test_class_method_call_using_object()
  # script context
  var lines =<< trim END
    vim9script
    class A
      static def Foo(): list<string>
        return ['a', 'b']
      enddef
      def Bar()
        assert_equal(['a', 'b'], A.Foo())
        assert_equal(['a', 'b'], Foo())
      enddef
    endclass

    def T()
      assert_equal(['a', 'b'], A.Foo())
      var t_a = A.new()
      t_a.Bar()
    enddef

    assert_equal(['a', 'b'], A.Foo())
    var a = A.new()
    a.Bar()
    T()
  END
  v9.CheckSourceSuccess(lines)

  # script context
  lines =<< trim END
    vim9script
    class A
      static def Foo(): string
        return 'foo'
      enddef
    endclass

    var a = A.new()
    assert_equal('foo', a.Foo())
  END
  v9.CheckSourceFailure(lines, 'E1385: Class method "Foo" accessible only using class "A"', 9)

  # def function context
  lines =<< trim END
    vim9script
    class A
      static def Foo(): string
        return 'foo'
      enddef
    endclass

    def T()
      var a = A.new()
      assert_equal('foo', a.Foo())
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1385: Class method "Foo" accessible only using class "A"', 2)
enddef

def Test_class_variable()
  var lines =<< trim END
    vim9script

    class A
      public static var val: number = 10
      static def ClassFunc()
        assert_equal(10, val)
      enddef
      def ObjFunc()
        assert_equal(10, val)
      enddef
    endclass

    class B extends A
    endclass

    assert_equal(10, A.val)
    A.ClassFunc()
    var a = A.new()
    a.ObjFunc()
    var b = B.new()
    b.ObjFunc()

    def T1(a1: A)
      a1.ObjFunc()
      A.ClassFunc()
    enddef
    T1(b)

    A.val = 20
    assert_equal(20, A.val)
  END
  v9.CheckSourceSuccess(lines)

  # Modifying a parent class variable from a child class method
  lines =<< trim END
    vim9script

    class A
      static var val: number = 10
    endclass

    class B extends A
      static def ClassFunc()
        val = 20
      enddef
    endclass
    B.ClassFunc()
  END
  v9.CheckSourceFailure(lines, 'E1374: Class variable "val" accessible only inside class "A"', 1)

  # Reading a parent class variable from a child class method
  lines =<< trim END
    vim9script

    class A
      static var val: number = 10
    endclass

    class B extends A
      static def ClassFunc()
        var i = val
      enddef
    endclass
    B.ClassFunc()
  END
  v9.CheckSourceFailure(lines, 'E1374: Class variable "val" accessible only inside class "A"', 1)

  # Modifying a parent class variable from a child object method
  lines =<< trim END
    vim9script

    class A
      static var val: number = 10
    endclass

    class B extends A
      def ObjFunc()
        val = 20
      enddef
    endclass
    var b = B.new()
    b.ObjFunc()
  END
  v9.CheckSourceFailure(lines, 'E1374: Class variable "val" accessible only inside class "A"', 1)

  # Reading a parent class variable from a child object method
  lines =<< trim END
    vim9script

    class A
      static var val: number = 10
    endclass

    class B extends A
      def ObjFunc()
        var i = val
      enddef
    endclass
    var b = B.new()
    b.ObjFunc()
  END
  v9.CheckSourceFailure(lines, 'E1374: Class variable "val" accessible only inside class "A"', 1)

  # Modifying a class variable using an object at script level
  lines =<< trim END
    vim9script

    class A
      static var val: number = 10
    endclass
    var a = A.new()
    a.val = 20
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "val" accessible only using class "A"', 7)

  # Reading a class variable using an object at script level
  lines =<< trim END
    vim9script

    class A
      static var val: number = 10
    endclass
    var a = A.new()
    var i = a.val
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "val" accessible only using class "A"', 7)

  # Modifying a class variable using an object at function level
  lines =<< trim END
    vim9script

    class A
      static var val: number = 10
    endclass

    def T()
      var a = A.new()
      a.val = 20
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "val" accessible only using class "A"', 2)

  # Reading a class variable using an object at function level
  lines =<< trim END
    vim9script

    class A
      static var val: number = 10
    endclass
    def T()
      var a = A.new()
      var i = a.val
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1375: Class variable "val" accessible only using class "A"', 2)

  # Use old implicit var declaration syntax (without initialization)
  lines =<< trim END
    vim9script

    class A
      static val: number
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1368: Static must be followed by "var" or "def"', 4)

  # Use old implicit var declaration syntax (with initialization)
  lines =<< trim END
    vim9script

    class A
      static val: number = 10
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1368: Static must be followed by "var" or "def"', 4)

  # Use old implicit var declaration syntax (type inferred)
  lines =<< trim END
    vim9script

    class A
      static val = 10
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1368: Static must be followed by "var" or "def"', 4)

  # Missing ":var" in "var" class variable declaration (without initialization)
  lines =<< trim END
    vim9script

    class A
      static var: number
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1329: Invalid class variable declaration: static var: number', 4)

  # Missing ":var" in "var" class variable declaration (with initialization)
  lines =<< trim END
    vim9script

    class A
      static var: number = 10
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1329: Invalid class variable declaration: static var: number = 10', 4)

  # Missing ":var" in "var" class variable declaration (type inferred)
  lines =<< trim END
    vim9script

    class A
      static var = 10
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1329: Invalid class variable declaration: static var = 10', 4)

enddef

" Test for using a duplicate class method and class variable in a child class
def Test_dup_class_member()
  # duplicate class variable, class method and overridden object method
  var lines =<< trim END
    vim9script
    class A
      static var sval = 100
      static def Check()
        assert_equal(100, sval)
      enddef
      def GetVal(): number
        return sval
      enddef
    endclass

    class B extends A
      static var sval = 200
      static def Check()
        assert_equal(200, sval)
      enddef
      def GetVal(): number
        return sval
      enddef
    endclass

    def T1(aa: A): number
      return aa.GetVal()
    enddef

    def T2(bb: B): number
      return bb.GetVal()
    enddef

    assert_equal(100, A.sval)
    assert_equal(200, B.sval)
    var a = A.new()
    assert_equal(100, a.GetVal())
    var b = B.new()
    assert_equal(200, b.GetVal())
    assert_equal(200, T1(b))
    assert_equal(200, T2(b))
  END
  v9.CheckSourceSuccess(lines)

  # duplicate class variable and class method
  lines =<< trim END
    vim9script
    class A
      static var sval = 100
      static def Check()
        assert_equal(100, sval)
      enddef
      def GetVal(): number
        return sval
      enddef
    endclass

    class B extends A
      static var sval = 200
      static def Check()
        assert_equal(200, sval)
      enddef
    endclass

    def T1(aa: A): number
      return aa.GetVal()
    enddef

    def T2(bb: B): number
      return bb.GetVal()
    enddef

    assert_equal(100, A.sval)
    assert_equal(200, B.sval)
    var a = A.new()
    assert_equal(100, a.GetVal())
    var b = B.new()
    assert_equal(100, b.GetVal())
    assert_equal(100, T1(b))
    assert_equal(100, T2(b))
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for calling an instance method using the class
def Test_instance_method_call_using_class()
  # Invoke an object method using a class in script context
  var lines =<< trim END
    vim9script
    class A
      def Foo()
        echo "foo"
      enddef
    endclass
    A.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1386: Object method "Foo" accessible only using class "A" object', 7)

  # Invoke an object method using a class in def function context
  lines =<< trim END
    vim9script
    class A
      def Foo()
        echo "foo"
      enddef
    endclass
    def T()
      A.Foo()
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1386: Object method "Foo" accessible only using class "A" object', 1)
enddef

" Test for duplicate class method and instance method
def Test_dup_classmethod_objmethod()
  # Duplicate instance method
  var lines =<< trim END
    vim9script
    class A
      static def Foo()
      enddef
      def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: Foo', 6)

  # Duplicate protected instance method
  lines =<< trim END
    vim9script
    class A
      static def Foo()
      enddef
      def _Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: _Foo', 6)

  # Duplicate class method
  lines =<< trim END
    vim9script
    class A
      def Foo()
      enddef
      static def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: Foo', 6)

  # Duplicate protected class method
  lines =<< trim END
    vim9script
    class A
      def Foo()
      enddef
      static def _Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: _Foo', 6)

  # Duplicate protected class and object method
  lines =<< trim END
    vim9script
    class A
      def _Foo()
      enddef
      static def _Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1355: Duplicate function: _Foo', 6)
enddef

" Test for an instance method access level comparison with parent instance
" methods.
def Test_instance_method_access_level()
  # protected method in subclass
  var lines =<< trim END
    vim9script
    class A
      def Foo()
      enddef
    endclass
    class B extends A
    endclass
    class C extends B
      def _Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1377: Access level of method "_Foo" is different in class "A"', 11)

  # Public method in subclass
  lines =<< trim END
    vim9script
    class A
      def _Foo()
      enddef
    endclass
    class B extends A
    endclass
    class C extends B
      def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1377: Access level of method "Foo" is different in class "A"', 11)
enddef

def Test_extend_empty_class()
  var lines =<< trim END
    vim9script
    class A
    endclass
    class B extends A
    endclass
    class C extends B
      public static var rw_class_var = 1
      public var rw_obj_var = 2
      static def ClassMethod(): number
        return 3
      enddef
      def ObjMethod(): number
        return 4
      enddef
    endclass
    assert_equal(1, C.rw_class_var)
    assert_equal(3, C.ClassMethod())
    var c = C.new()
    assert_equal(2, c.rw_obj_var)
    assert_equal(4, c.ObjMethod())
  END
  v9.CheckSourceSuccess(lines)
enddef

" A interface cannot have a static variable or a static method or a private
" variable or a protected method or a public variable
def Test_interface_with_unsupported_members()
  var lines =<< trim END
    vim9script
    interface A
      static var num: number
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1378: Static member not supported in an interface', 3)

  lines =<< trim END
    vim9script
    interface A
      static var _num: number
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1378: Static member not supported in an interface', 3)

  lines =<< trim END
    vim9script
    interface A
      public static var num: number
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1387: Public variable not supported in an interface', 3)

  lines =<< trim END
    vim9script
    interface A
      public static var num: number
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1387: Public variable not supported in an interface', 3)

  lines =<< trim END
    vim9script
    interface A
      static var _num: number
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1378: Static member not supported in an interface', 3)

  lines =<< trim END
    vim9script
    interface A
      static def Foo(d: dict<any>): list<string>
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1378: Static member not supported in an interface', 3)

  lines =<< trim END
    vim9script
    interface A
      static def _Foo(d: dict<any>): list<string>
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1378: Static member not supported in an interface', 3)

  lines =<< trim END
    vim9script
    interface A
      var _Foo: list<string>
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1379: Protected variable not supported in an interface', 3)

  lines =<< trim END
    vim9script
    interface A
      def _Foo(d: dict<any>): list<string>
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1380: Protected method not supported in an interface', 3)
enddef

" Test for extending an interface
def Test_extend_interface()
  var lines =<< trim END
    vim9script
    interface A
      var var1: list<string>
      def Foo()
    endinterface
    interface B extends A
      var var2: dict<string>
      def Bar()
    endinterface
    class C implements A, B
      var var1 = [1, 2]
      def Foo()
      enddef
      var var2 = {a: '1'}
      def Bar()
      enddef
    endclass
  END
  v9.CheckSourceSuccess(lines)

  # extending empty interface
  lines =<< trim END
    vim9script
    interface A
    endinterface
    interface B extends A
    endinterface
    class C implements B
    endclass
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script
    interface A
      def Foo()
    endinterface
    interface B extends A
      var var2: dict<string>
    endinterface
    class C implements A, B
      var var2 = {a: '1'}
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1349: Method "Foo" of interface "A" is not implemented', 10)

  lines =<< trim END
    vim9script
    interface A
      def Foo()
    endinterface
    interface B extends A
      var var2: dict<string>
    endinterface
    class C implements A, B
      def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1348: Variable "var2" of interface "B" is not implemented', 11)

  # interface cannot extend a class
  lines =<< trim END
    vim9script
    class A
    endclass
    interface B extends A
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1354: Cannot extend A', 5)

  # class cannot extend an interface
  lines =<< trim END
    vim9script
    interface A
    endinterface
    class B extends A
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1354: Cannot extend A', 5)

  # interface cannot implement another interface
  lines =<< trim END
    vim9script
    interface A
    endinterface
    interface B implements A
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1381: Interface cannot use "implements"', 4)

  # interface cannot extend multiple interfaces
  lines =<< trim END
    vim9script
    interface A
    endinterface
    interface B
    endinterface
    interface C extends A, B
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1315: White space required after name: A, B', 6)

  # Variable type in an extended interface is of different type
  lines =<< trim END
    vim9script
    interface A
      var val1: number
    endinterface
    interface B extends A
      var val2: string
    endinterface
    interface C extends B
      var val1: string
      var val2: number
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1382: Variable "val1": type mismatch, expected number but got string', 11)
enddef

" Test for a child class implementing an interface when some of the methods are
" defined in the parent class.
def Test_child_class_implements_interface()
  var lines =<< trim END
    vim9script

    interface Intf
      def F1(): list<list<number>>
      def F2(): list<list<number>>
      def F3(): list<list<number>>
      var var1: list<dict<number>>
      var var2: list<dict<number>>
      var var3: list<dict<number>>
    endinterface

    class A
      def A1()
      enddef
      def F3(): list<list<number>>
        return [[3]]
      enddef
      var v1: list<list<number>> = [[0]]
      var var3 = [{c: 30}]
    endclass

    class B extends A
      def B1()
      enddef
      def F2(): list<list<number>>
        return [[2]]
      enddef
      var v2: list<list<number>> = [[0]]
      var var2 = [{b: 20}]
    endclass

    class C extends B implements Intf
      def C1()
      enddef
      def F1(): list<list<number>>
        return [[1]]
      enddef
      var v3: list<list<number>> = [[0]]
      var var1 = [{a: 10}]
    endclass

    def T(if: Intf)
      assert_equal([[1]], if.F1())
      assert_equal([[2]], if.F2())
      assert_equal([[3]], if.F3())
      assert_equal([{a: 10}], if.var1)
      assert_equal([{b: 20}], if.var2)
      assert_equal([{c: 30}], if.var3)
    enddef

    var c = C.new()
    T(c)
    assert_equal([[1]], c.F1())
    assert_equal([[2]], c.F2())
    assert_equal([[3]], c.F3())
    assert_equal([{a: 10}], c.var1)
    assert_equal([{b: 20}], c.var2)
    assert_equal([{c: 30}], c.var3)
  END
  v9.CheckSourceSuccess(lines)

  # One of the interface methods is not found
  lines =<< trim END
    vim9script

    interface Intf
      def F1()
      def F2()
      def F3()
    endinterface

    class A
      def A1()
      enddef
    endclass

    class B extends A
      def B1()
      enddef
      def F2()
      enddef
    endclass

    class C extends B implements Intf
      def C1()
      enddef
      def F1()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1349: Method "F3" of interface "Intf" is not implemented', 26)

  # One of the interface methods is of different type
  lines =<< trim END
    vim9script

    interface Intf
      def F1()
      def F2()
      def F3()
    endinterface

    class A
      def F3(): number
        return 0
      enddef
      def A1()
      enddef
    endclass

    class B extends A
      def B1()
      enddef
      def F2()
      enddef
    endclass

    class C extends B implements Intf
      def C1()
      enddef
      def F1()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1383: Method "F3": type mismatch, expected func() but got func(): number', 29)

  # One of the interface variables is not present
  lines =<< trim END
    vim9script

    interface Intf
      var var1: list<dict<number>>
      var var2: list<dict<number>>
      var var3: list<dict<number>>
    endinterface

    class A
      var v1: list<list<number>> = [[0]]
    endclass

    class B extends A
      var v2: list<list<number>> = [[0]]
      var var2 = [{b: 20}]
    endclass

    class C extends B implements Intf
      var v3: list<list<number>> = [[0]]
      var var1 = [{a: 10}]
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1348: Variable "var3" of interface "Intf" is not implemented', 21)

  # One of the interface variables is of different type
  lines =<< trim END
    vim9script

    interface Intf
      var var1: list<dict<number>>
      var var2: list<dict<number>>
      var var3: list<dict<number>>
    endinterface

    class A
      var v1: list<list<number>> = [[0]]
      var var3: list<dict<string>>
    endclass

    class B extends A
      var v2: list<list<number>> = [[0]]
      var var2 = [{b: 20}]
    endclass

    class C extends B implements Intf
      var v3: list<list<number>> = [[0]]
      var var1 = [{a: 10}]
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1382: Variable "var3": type mismatch, expected list<dict<number>> but got list<dict<string>>', 22)
enddef

" Test for extending an interface with duplicate variables and methods
def Test_interface_extends_with_dup_members()
  var lines =<< trim END
    vim9script
    interface A
      var n1: number
      def Foo1(): number
    endinterface
    interface B extends A
      var n2: number
      var n1: number
      def Foo2(): number
      def Foo1(): number
    endinterface
    class C implements B
      var n1 = 10
      var n2 = 20
      def Foo1(): number
        return 30
      enddef
      def Foo2(): number
        return 40
      enddef
    endclass
    def T1(a: A)
      assert_equal(10, a.n1)
      assert_equal(30, a.Foo1())
    enddef
    def T2(b: B)
      assert_equal(10, b.n1)
      assert_equal(20, b.n2)
      assert_equal(30, b.Foo1())
      assert_equal(40, b.Foo2())
    enddef
    var c = C.new()
    T1(c)
    T2(c)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using "any" type for a variable in a sub-class while it has a
" concrete type in the interface
def Test_implements_using_var_type_any()
  var lines =<< trim END
    vim9script
    interface A
      var val: list<dict<string>>
    endinterface
    class B implements A
      var val = [{a: '1'}, {b: '2'}]
    endclass
    var b = B.new()
    assert_equal([{a: '1'}, {b: '2'}], b.val)
  END
  v9.CheckSourceSuccess(lines)

  # initialize instance variable using a different type
  lines =<< trim END
    vim9script
    interface A
      var val: list<dict<string>>
    endinterface
    class B implements A
      var val = {a: 1, b: 2}
    endclass
    var b = B.new()
  END
  v9.CheckSourceFailure(lines, 'E1382: Variable "val": type mismatch, expected list<dict<string>> but got dict<number>', 1)
enddef

" Test for assigning to a member variable in a nested class
def Test_nested_object_assignment()
  var lines =<< trim END
    vim9script

    class A
      var value: number
    endclass

    class B
      var a: A = A.new()
    endclass

    class C
      var b: B = B.new()
    endclass

    class D
      var c: C = C.new()
    endclass

    def T(da: D)
      da.c.b.a.value = 10
    enddef

    var d = D.new()
    T(d)
  END
  v9.CheckSourceFailure(lines, 'E1335: Variable "value" in class "A" is not writable', 1)
enddef

" Test for calling methods using a null object
def Test_null_object_method_call()
  # Calling a object method using a null object in script context
  var lines =<< trim END
    vim9script

    class C
      def Foo()
        assert_report('This method should not be executed')
      enddef
    endclass

    var o: C
    o.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 10)

  # Calling a object method using a null object in def function context
  lines =<< trim END
    vim9script

    class C
      def Foo()
        assert_report('This method should not be executed')
      enddef
    endclass

    def T()
      var o: C
      o.Foo()
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 2)

  # Calling a object method through another class method using a null object in
  # script context
  lines =<< trim END
    vim9script

    class C
      def Foo()
        assert_report('This method should not be executed')
      enddef

      static def Bar(o_any: any)
        var o_typed: C = o_any
        o_typed.Foo()
      enddef
    endclass

    var o: C
    C.Bar(o)
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 2)

  # Calling a object method through another class method using a null object in
  # def function context
  lines =<< trim END
    vim9script

    class C
      def Foo()
        assert_report('This method should not be executed')
      enddef

      static def Bar(o_any: any)
        var o_typed: C = o_any
        o_typed.Foo()
      enddef
    endclass

    def T()
      var o: C
      C.Bar(o)
    enddef
    T()
  END
  v9.CheckSourceFailure(lines, 'E1360: Using a null object', 2)
enddef

" Test for using a dict as an object member
def Test_dict_object_member()
  var lines =<< trim END
    vim9script

    class Context
      public var state: dict<number> = {}
      def GetState(): dict<number>
        return this.state
      enddef
    endclass

    var ctx = Context.new()
    ctx.state->extend({a: 1})
    ctx.state['b'] = 2
    assert_equal({a: 1, b: 2}, ctx.GetState())

    def F()
      ctx.state['c'] = 3
      assert_equal({a: 1, b: 2, c: 3}, ctx.GetState())
    enddef
    F()
    assert_equal(3, ctx.state.c)
    ctx.state.c = 4
    assert_equal(4, ctx.state.c)
  END
  v9.CheckSourceSuccess(lines)
enddef

" The following test was failing after 9.0.1914.  This was caused by using a
" freed object from a previous method call.
def Test_freed_object_from_previous_method_call()
  var lines =<< trim END
    vim9script

    class Context
    endclass

    class Result
    endclass

    def Failure(): Result
      return Result.new()
    enddef

    def GetResult(ctx: Context): Result
      return Failure()
    enddef

    def Test_GetResult()
      var ctx = Context.new()
      var result = GetResult(ctx)
    enddef

    Test_GetResult()
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for duplicate object and class variable
def Test_duplicate_variable()
  # Object variable name is same as the class variable name
  var lines =<< trim END
    vim9script
    class A
      public static var sval: number
      public var sval: number
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: sval', 4)

  # Duplicate variable name and calling a class method
  lines =<< trim END
    vim9script
    class A
      public static var sval: number
      public var sval: number
      def F1()
        echo this.sval
      enddef
      static def F2()
        echo sval
      enddef
    endclass
    A.F2()
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: sval', 4)

  # Duplicate variable with an empty constructor
  lines =<< trim END
    vim9script
    class A
      public static var sval: number
      public var sval: number
      def new()
      enddef
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1369: Duplicate variable: sval', 4)
enddef

" Test for using a reserved keyword as a variable name
def Test_reserved_varname()
  for kword in ['true', 'false', 'null', 'null_blob', 'null_dict',
                'null_function', 'null_list', 'null_partial', 'null_string',
                'null_channel', 'null_job', 'super', 'this']

    var lines =<< trim eval END
      vim9script
      class C
        public var {kword}: list<number> = [1, 2, 3]
      endclass
      var o = C.new()
    END
    v9.CheckSourceFailure(lines, $'E1034: Cannot use reserved name {kword}', 3)

    lines =<< trim eval END
      vim9script
      class C
        public var {kword}: list<number> = [1, 2, 3]
        def new()
        enddef
      endclass
      var o = C.new()
    END
    v9.CheckSourceFailure(lines, $'E1034: Cannot use reserved name {kword}', 3)

    lines =<< trim eval END
      vim9script
      class C
        public var {kword}: list<number> = [1, 2, 3]
        def new()
        enddef
        def F()
          echo this.{kword}
        enddef
      endclass
      var o = C.new()
      o.F()
    END
    v9.CheckSourceFailure(lines, $'E1034: Cannot use reserved name {kword}', 3)

    # class variable name
    if kword != 'this'
      lines =<< trim eval END
        vim9script
        class C
          public static var {kword}: list<number> = [1, 2, 3]
        endclass
      END
      v9.CheckSourceFailure(lines, $'E1034: Cannot use reserved name {kword}', 3)
    endif
  endfor
enddef

" Test for checking the type of the arguments and the return value of a object
" method in an extended class.
def Test_extended_obj_method_type_check()
  var lines =<< trim END
    vim9script

    class A
    endclass
    class B extends A
    endclass
    class C extends B
    endclass

    class Foo
      def Doit(p: B): B
        return B.new()
      enddef
    endclass

    class Bar extends Foo
      def Doit(p: C): B
        return B.new()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1383: Method "Doit": type mismatch, expected func(object<B>): object<B> but got func(object<C>): object<B>', 20)

  lines =<< trim END
    vim9script

    class A
    endclass
    class B extends A
    endclass
    class C extends B
    endclass

    class Foo
      def Doit(p: B): B
        return B.new()
      enddef
    endclass

    class Bar extends Foo
      def Doit(p: B): C
        return C.new()
      enddef
    endclass
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class A
    endclass
    class B extends A
    endclass
    class C extends B
    endclass

    class Foo
      def Doit(p: B): B
        return B.new()
      enddef
    endclass

    class Bar extends Foo
      def Doit(p: A): B
        return B.new()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1383: Method "Doit": type mismatch, expected func(object<B>): object<B> but got func(object<A>): object<B>', 20)

  lines =<< trim END
    vim9script

    class A
    endclass
    class B extends A
    endclass
    class C extends B
    endclass

    class Foo
      def Doit(p: B): B
        return B.new()
      enddef
    endclass

    class Bar extends Foo
      def Doit(p: B): A
        return A.new()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1383: Method "Doit": type mismatch, expected func(object<B>): object<B> but got func(object<B>): object<A>', 20)

  # check varargs type mismatch
  lines =<< trim END
    vim9script

    class B
      def F(...xxx: list<any>)
      enddef
    endclass
    class C extends B
      def F(xxx: list<any>)
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1383: Method "F": type mismatch, expected func(...list<any>) but got func(list<any>)', 10)
enddef

" Test type checking for class variable in assignments
func Test_class_variable_complex_type_check()
  " class variable with a specific type.  Try assigning a different type at
  " script level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public static var Fn: func(list<dict<blob>>): dict<list<blob>> = Foo
    endclass
    test_garbagecollect_now()
    A.Fn = "abc"
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 9)

  " class variable with a specific type.  Try assigning a different type at
  " class def method level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public static var Fn: func(list<dict<blob>>): dict<list<blob>> = Foo
      def Bar()
        Fn = "abc"
      enddef
    endclass
    var a = A.new()
    test_garbagecollect_now()
    a.Bar()
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 1)

  " class variable with a specific type.  Try assigning a different type at
  " script def method level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public static var Fn: func(list<dict<blob>>): dict<list<blob>> = Foo
    endclass
    def Bar()
      A.Fn = "abc"
    enddef
    test_garbagecollect_now()
    Bar()
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 1)

  " class variable without any type.  Should be set to the initialization
  " expression type.  Try assigning a different type from script level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public static var Fn = Foo
    endclass
    test_garbagecollect_now()
    A.Fn = "abc"
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 9)

  " class variable without any type.  Should be set to the initialization
  " expression type.  Try assigning a different type at class def level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public static var Fn = Foo
      def Bar()
        Fn = "abc"
      enddef
    endclass
    var a = A.new()
    test_garbagecollect_now()
    a.Bar()
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 1)

  " class variable without any type.  Should be set to the initialization
  " expression type.  Try assigning a different type at script def level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public static var Fn = Foo
    endclass
    def Bar()
      A.Fn = "abc"
    enddef
    test_garbagecollect_now()
    Bar()
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 1)

  " class variable with 'any" type.  Can be assigned different types.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public static var Fn: any = Foo
      public static var Fn2: any
    endclass
    test_garbagecollect_now()
    assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(A.Fn))
    A.Fn = "abc"
    test_garbagecollect_now()
    assert_equal('string', typename(A.Fn))
    A.Fn2 = Foo
    test_garbagecollect_now()
    assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(A.Fn2))
    A.Fn2 = "xyz"
    test_garbagecollect_now()
    assert_equal('string', typename(A.Fn2))
  END
  call v9.CheckSourceSuccess(lines)

  " class variable with 'any" type.  Can be assigned different types.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public static var Fn: any = Foo
      public static var Fn2: any

      def Bar()
        assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(Fn))
        Fn = "abc"
        assert_equal('string', typename(Fn))
        Fn2 = Foo
        assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(Fn2))
        Fn2 = "xyz"
        assert_equal('string', typename(Fn2))
      enddef
    endclass
    var a = A.new()
    test_garbagecollect_now()
    a.Bar()
    test_garbagecollect_now()
    A.Fn = Foo
    a.Bar()
  END
  call v9.CheckSourceSuccess(lines)

  " class variable with 'any" type.  Can be assigned different types.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public static var Fn: any = Foo
      public static var Fn2: any
    endclass

    def Bar()
      assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(A.Fn))
      A.Fn = "abc"
      assert_equal('string', typename(A.Fn))
      A.Fn2 = Foo
      assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(A.Fn2))
      A.Fn2 = "xyz"
      assert_equal('string', typename(A.Fn2))
    enddef
    Bar()
    test_garbagecollect_now()
    A.Fn = Foo
    Bar()
  END
  call v9.CheckSourceSuccess(lines)

  let lines =<< trim END
    vim9script
    class A
      public static var foo = [0z10, 0z20]
    endclass
    assert_equal([0z10, 0z20], A.foo)
    A.foo = [0z30]
    assert_equal([0z30], A.foo)
    var a = A.foo
    assert_equal([0z30], a)
  END
  call v9.CheckSourceSuccess(lines)
endfunc

" Test type checking for object variable in assignments
func Test_object_variable_complex_type_check()
  " object variable with a specific type.  Try assigning a different type at
  " script level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public var Fn: func(list<dict<blob>>): dict<list<blob>> = Foo
    endclass
    var a = A.new()
    test_garbagecollect_now()
    a.Fn = "abc"
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 10)

  " object variable with a specific type.  Try assigning a different type at
  " object def method level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public var Fn: func(list<dict<blob>>): dict<list<blob>> = Foo
      def Bar()
        this.Fn = "abc"
        this.Fn = Foo
      enddef
    endclass
    var a = A.new()
    test_garbagecollect_now()
    a.Bar()
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 1)

  " object variable with a specific type.  Try assigning a different type at
  " script def method level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public var Fn: func(list<dict<blob>>): dict<list<blob>> = Foo
    endclass
    def Bar()
      var a = A.new()
      a.Fn = "abc"
      a.Fn = Foo
    enddef
    test_garbagecollect_now()
    Bar()
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 2)

  " object variable without any type.  Should be set to the initialization
  " expression type.  Try assigning a different type from script level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public var Fn = Foo
    endclass
    var a = A.new()
    test_garbagecollect_now()
    a.Fn = "abc"
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 10)

  " object variable without any type.  Should be set to the initialization
  " expression type.  Try assigning a different type at object def level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public var Fn = Foo
      def Bar()
        this.Fn = "abc"
        this.Fn = Foo
      enddef
    endclass
    var a = A.new()
    test_garbagecollect_now()
    a.Bar()
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 1)

  " object variable without any type.  Should be set to the initialization
  " expression type.  Try assigning a different type at script def level.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public var Fn = Foo
    endclass
    def Bar()
      var a = A.new()
      a.Fn = "abc"
      a.Fn = Foo
    enddef
    test_garbagecollect_now()
    Bar()
  END
  call v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(list<dict<blob>>): dict<list<blob>> but got string', 2)

  " object variable with 'any" type.  Can be assigned different types.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public var Fn: any = Foo
      public var Fn2: any
    endclass

    var a = A.new()
    test_garbagecollect_now()
    assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(a.Fn))
    a.Fn = "abc"
    test_garbagecollect_now()
    assert_equal('string', typename(a.Fn))
    a.Fn2 = Foo
    test_garbagecollect_now()
    assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(a.Fn2))
    a.Fn2 = "xyz"
    test_garbagecollect_now()
    assert_equal('string', typename(a.Fn2))
  END
  call v9.CheckSourceSuccess(lines)

  " object variable with 'any" type.  Can be assigned different types.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public var Fn: any = Foo
      public var Fn2: any

      def Bar()
        assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(this.Fn))
        this.Fn = "abc"
        assert_equal('string', typename(this.Fn))
        this.Fn2 = Foo
        assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(this.Fn2))
        this.Fn2 = "xyz"
        assert_equal('string', typename(this.Fn2))
      enddef
    endclass

    var a = A.new()
    test_garbagecollect_now()
    a.Bar()
    test_garbagecollect_now()
    a.Fn = Foo
    a.Bar()
  END
  call v9.CheckSourceSuccess(lines)

  " object variable with 'any" type.  Can be assigned different types.
  let lines =<< trim END
    vim9script
    def Foo(l: list<dict<blob>>): dict<list<blob>>
      return {}
    enddef
    class A
      public var Fn: any = Foo
      public var Fn2: any
    endclass

    def Bar()
      var a = A.new()
      assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(a.Fn))
      a.Fn = "abc"
      assert_equal('string', typename(a.Fn))
      a.Fn2 = Foo
      assert_equal('func(list<dict<blob>>): dict<list<blob>>', typename(a.Fn2))
      a.Fn2 = "xyz"
      assert_equal('string', typename(a.Fn2))
    enddef
    test_garbagecollect_now()
    Bar()
    test_garbagecollect_now()
    Bar()
  END
  call v9.CheckSourceSuccess(lines)
endfunc

" Test for recursively calling an object method.  This used to cause an
" use-after-free error.
def Test_recursive_object_method_call()
  var lines =<< trim END
    vim9script
    class A
      var val: number = 0
      def Foo(): number
        if this.val >= 90
          return this.val
        endif
        this.val += 1
        return this.Foo()
      enddef
    endclass
    var a = A.new()
    assert_equal(90, a.Foo())
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for recursively calling a class method.
def Test_recursive_class_method_call()
  var lines =<< trim END
    vim9script
    class A
      static var val: number = 0
      static def Foo(): number
        if val >= 90
          return val
        endif
        val += 1
        return Foo()
      enddef
    endclass
    assert_equal(90, A.Foo())
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for checking the argument types and the return type when assigning a
" funcref to make sure the invariant class type is used.
def Test_funcref_argtype_returntype_check()
  var lines =<< trim END
    vim9script
    class A
    endclass
    class B extends A
    endclass

    def Foo(p: B): B
      return B.new()
    enddef

    var Bar: func(A): A = Foo
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(object<A>): object<A> but got func(object<B>): object<B>', 11)

  lines =<< trim END
    vim9script
    class A
    endclass
    class B extends A
    endclass

    def Foo(p: B): B
      return B.new()
    enddef

    def Baz()
      var Bar: func(A): A = Foo
    enddef
    Baz()
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(object<A>): object<A> but got func(object<B>): object<B>', 1)
enddef

def Test_funcref_argtype_invariance_check()
  var lines =<< trim END
    vim9script

    class A
    endclass
    class B extends A
    endclass
    class C extends B
    endclass

    var Func: func(B): number
    Func = (o: B): number => 3
    assert_equal(3, Func(B.new()))
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class A
    endclass
    class B extends A
    endclass
    class C extends B
    endclass

    var Func: func(B): number
    Func = (o: A): number => 3
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(object<B>): number but got func(object<A>): number', 11)

  lines =<< trim END
    vim9script

    class A
    endclass
    class B extends A
    endclass
    class C extends B
    endclass

    var Func: func(B): number
    Func = (o: C): number => 3
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected func(object<B>): number but got func(object<C>): number', 11)
enddef

" Test for using an operator (e.g. +) with an assignment
def Test_op_and_assignment()
  # Using += with a class variable
  var lines =<< trim END
    vim9script
    class A
      public static var val: list<number> = []
      static def Foo(): list<number>
        val += [1]
        return val
      enddef
    endclass
    def Bar(): list<number>
      A.val += [2]
      return A.val
    enddef
    assert_equal([1], A.Foo())
    assert_equal([1, 2], Bar())
    A.val += [3]
    assert_equal([1, 2, 3], A.val)
  END
  v9.CheckSourceSuccess(lines)

  # Using += with an object variable
  lines =<< trim END
    vim9script
    class A
      public var val: list<number> = []
      def Foo(): list<number>
        this.val += [1]
        return this.val
      enddef
    endclass
    def Bar(bar_a: A): list<number>
      bar_a.val += [2]
      return bar_a.val
    enddef
    var a = A.new()
    assert_equal([1], a.Foo())
    assert_equal([1, 2], Bar(a))
    a.val += [3]
    assert_equal([1, 2, 3], a.val)
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using an object method as a funcref
def Test_object_funcref()
  # Using object method funcref from a def function
  var lines =<< trim END
    vim9script
    class A
      def Foo(): list<number>
        return [3, 2, 1]
      enddef
    endclass
    def Bar()
      var a = A.new()
      var Fn = a.Foo
      assert_equal([3, 2, 1], Fn())
    enddef
    Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using object method funcref at the script level
  lines =<< trim END
    vim9script
    class A
      def Foo(): dict<number>
        return {a: 1, b: 2}
      enddef
    endclass
    var a = A.new()
    var Fn = a.Foo
    assert_equal({a: 1, b: 2}, Fn())
  END
  v9.CheckSourceSuccess(lines)

  # Using object method funcref at the script level
  lines =<< trim END
    vim9script
    class A
      var val: number
      def Foo(): number
        return this.val
      enddef
    endclass
    var a = A.new(345)
    var Fn = a.Foo
    assert_equal(345, Fn())
  END
  v9.CheckSourceSuccess(lines)

  # Using object method funcref from another object method
  lines =<< trim END
    vim9script
    class A
      def Foo(): list<number>
        return [3, 2, 1]
      enddef
      def Bar()
        var Fn = this.Foo
        assert_equal([3, 2, 1], Fn())
      enddef
    endclass
    var a = A.new()
    a.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using function() to get a object method funcref
  lines =<< trim END
    vim9script
    class A
      def Foo(l: list<any>): list<any>
        return l
      enddef
    endclass
    var a = A.new()
    var Fn = function(a.Foo, [[{a: 1, b: 2}, [3, 4]]])
    assert_equal([{a: 1, b: 2}, [3, 4]], Fn())
  END
  v9.CheckSourceSuccess(lines)

  # Use an object method with a function returning a funcref and then call the
  # funcref.
  lines =<< trim END
    vim9script

    def Map(F: func(number): number): func(number): number
      return (n: number) => F(n)
    enddef

    class Math
      def Double(n: number): number
        return 2 * n
      enddef
    endclass

    const math = Math.new()
    assert_equal(48, Map(math.Double)(24))
  END
  v9.CheckSourceSuccess(lines)

  # Try using a protected object method funcref from a def function
  lines =<< trim END
    vim9script
    class A
      def _Foo()
      enddef
    endclass
    def Bar()
      var a = A.new()
      var Fn = a._Foo
    enddef
    Bar()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 2)

  # Try using a protected object method funcref at the script level
  lines =<< trim END
    vim9script
    class A
      def _Foo()
      enddef
    endclass
    var a = A.new()
    var Fn = a._Foo
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 7)

  # Using a protected object method funcref from another object method
  lines =<< trim END
    vim9script
    class A
      def _Foo(): list<number>
        return [3, 2, 1]
      enddef
      def Bar()
        var Fn = this._Foo
        assert_equal([3, 2, 1], Fn())
      enddef
    endclass
    var a = A.new()
    a.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using object method funcref using call()
  lines =<< trim END
    vim9script
    class A
      var val: number
      def Foo(): number
        return this.val
      enddef
    endclass

    def Bar(obj: A)
      assert_equal(123, call(obj.Foo, []))
    enddef

    var a = A.new(123)
    Bar(a)
    assert_equal(123, call(a.Foo, []))
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using a class method as a funcref
def Test_class_funcref()
  # Using class method funcref in a def function
  var lines =<< trim END
    vim9script
    class A
      static def Foo(): list<number>
        return [3, 2, 1]
      enddef
    endclass
    def Bar()
      var Fn = A.Foo
      assert_equal([3, 2, 1], Fn())
    enddef
    Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using class method funcref at script level
  lines =<< trim END
    vim9script
    class A
      static def Foo(): dict<number>
        return {a: 1, b: 2}
      enddef
    endclass
    var Fn = A.Foo
    assert_equal({a: 1, b: 2}, Fn())
  END
  v9.CheckSourceSuccess(lines)

  # Using class method funcref at the script level
  lines =<< trim END
    vim9script
    class A
      public static var val: number
      static def Foo(): number
        return val
      enddef
    endclass
    A.val = 567
    var Fn = A.Foo
    assert_equal(567, Fn())
  END
  v9.CheckSourceSuccess(lines)

  # Using function() to get a class method funcref
  lines =<< trim END
    vim9script
    class A
      static def Foo(l: list<any>): list<any>
        return l
      enddef
    endclass
    var Fn = function(A.Foo, [[{a: 1, b: 2}, [3, 4]]])
    assert_equal([{a: 1, b: 2}, [3, 4]], Fn())
  END
  v9.CheckSourceSuccess(lines)

  # Using a class method funcref from another class method
  lines =<< trim END
    vim9script
    class A
      static def Foo(): list<number>
        return [3, 2, 1]
      enddef
      static def Bar()
        var Fn = Foo
        assert_equal([3, 2, 1], Fn())
      enddef
    endclass
    A.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Use a class method with a function returning a funcref and then call the
  # funcref.
  lines =<< trim END
    vim9script

    def Map(F: func(number): number): func(number): number
      return (n: number) => F(n)
    enddef

    class Math
      static def StaticDouble(n: number): number
        return 2 * n
      enddef
    endclass

    assert_equal(48, Map(Math.StaticDouble)(24))
  END
  v9.CheckSourceSuccess(lines)

  # Try using a protected class method funcref in a def function
  lines =<< trim END
    vim9script
    class A
      static def _Foo()
      enddef
    endclass
    def Bar()
      var Fn = A._Foo
    enddef
    Bar()
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 1)

  # Try using a protected class method funcref at script level
  lines =<< trim END
    vim9script
    class A
      static def _Foo()
      enddef
    endclass
    var Fn = A._Foo
  END
  v9.CheckSourceFailure(lines, 'E1366: Cannot access protected method: _Foo', 6)

  # Using a protected class method funcref from another class method
  lines =<< trim END
    vim9script
    class A
      static def _Foo(): list<number>
        return [3, 2, 1]
      enddef
      static def Bar()
        var Fn = _Foo
        assert_equal([3, 2, 1], Fn())
      enddef
    endclass
    A.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using class method funcref using call()
  lines =<< trim END
    vim9script
    class A
      public static var val: number
      static def Foo(): number
        return val
      enddef
    endclass

    def Bar()
      A.val = 468
      assert_equal(468, call(A.Foo, []))
    enddef
    Bar()
    assert_equal(468, call(A.Foo, []))
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using an object member as a funcref
def Test_object_member_funcref()
  # Using a funcref object variable in an object method
  var lines =<< trim END
    vim9script
    def Foo(n: number): number
      return n * 10
    enddef

    class A
      var Cb: func(number): number = Foo
      def Bar()
        assert_equal(200, this.Cb(20))
      enddef
    endclass

    var a = A.new()
    a.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref object variable in a def method
  lines =<< trim END
    vim9script
    def Foo(n: number): number
      return n * 10
    enddef

    class A
      var Cb: func(number): number = Foo
    endclass

    def Bar()
      var a = A.new()
      assert_equal(200, a.Cb(20))
    enddef
    Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref object variable at script level
  lines =<< trim END
    vim9script
    def Foo(n: number): number
      return n * 10
    enddef

    class A
      var Cb: func(number): number = Foo
    endclass

    var a = A.new()
    assert_equal(200, a.Cb(20))
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref object variable pointing to an object method in an object
  # method.
  lines =<< trim END
    vim9script
    class A
      var Cb: func(number): number = this.Foo
      def Foo(n: number): number
        return n * 10
      enddef
      def Bar()
        assert_equal(200, this.Cb(20))
      enddef
    endclass

    var a = A.new()
    a.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref object variable pointing to an object method in a def
  # method.
  lines =<< trim END
    vim9script
    class A
      var Cb: func(number): number = this.Foo
      def Foo(n: number): number
        return n * 10
      enddef
    endclass

    def Bar()
      var a = A.new()
      assert_equal(200, a.Cb(20))
    enddef
    Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref object variable pointing to an object method at script
  # level.
  lines =<< trim END
    vim9script
    class A
      var Cb = this.Foo
      def Foo(n: number): number
        return n * 10
      enddef
    endclass

    var a = A.new()
    assert_equal(200, a.Cb(20))
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using a class member as a funcref
def Test_class_member_funcref()
  # Using a funcref class variable in a class method
  var lines =<< trim END
    vim9script
    def Foo(n: number): number
      return n * 10
    enddef

    class A
      static var Cb = Foo
      static def Bar()
        assert_equal(200, Cb(20))
      enddef
    endclass

    A.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref class variable in a def method
  lines =<< trim END
    vim9script
    def Foo(n: number): number
      return n * 10
    enddef

    class A
      public static var Cb = Foo
    endclass

    def Bar()
      assert_equal(200, A.Cb(20))
    enddef
    Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref class variable at script level
  lines =<< trim END
    vim9script
    def Foo(n: number): number
      return n * 10
    enddef

    class A
      public static var Cb = Foo
    endclass

    assert_equal(200, A.Cb(20))
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref class variable pointing to a class method in a class
  # method.
  lines =<< trim END
    vim9script
    class A
      static var Cb: func(number): number
      static def Foo(n: number): number
        return n * 10
      enddef
      static def Init()
        Cb = Foo
      enddef
      static def Bar()
        assert_equal(200, Cb(20))
      enddef
    endclass

    A.Init()
    A.Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref class variable pointing to a class method in a def method.
  lines =<< trim END
    vim9script
    class A
      static var Cb: func(number): number
      static def Foo(n: number): number
        return n * 10
      enddef
      static def Init()
        Cb = Foo
      enddef
    endclass

    def Bar()
      A.Init()
      assert_equal(200, A.Cb(20))
    enddef
    Bar()
  END
  v9.CheckSourceSuccess(lines)

  # Using a funcref class variable pointing to a class method at script level.
  lines =<< trim END
    vim9script
    class A
      static var Cb: func(number): number
      static def Foo(n: number): number
        return n * 10
      enddef
      static def Init()
        Cb = Foo
      enddef
    endclass

    A.Init()
    assert_equal(200, A.Cb(20))
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using object methods as popup callback functions
def Test_objmethod_popup_callback()
  # Use the popup from the script level
  var lines =<< trim END
    vim9script

    class A
      var selection: number = -1
      var filterkeys: list<string> = []

      def PopupFilter(id: number, key: string): bool
        add(this.filterkeys, key)
        return popup_filter_yesno(id, key)
      enddef

      def PopupCb(id: number, result: number)
        this.selection = result ? 100 : 200
      enddef
    endclass

    var a = A.new()
    feedkeys('', 'xt')
    var winid = popup_create('Y/N?',
                              {filter: a.PopupFilter, callback: a.PopupCb})
    feedkeys('y', 'xt')
    popup_close(winid)
    assert_equal(100, a.selection)
    assert_equal(['y'], a.filterkeys)
    feedkeys('', 'xt')
    winid = popup_create('Y/N?',
                              {filter: a.PopupFilter, callback: a.PopupCb})
    feedkeys('n', 'xt')
    popup_close(winid)
    assert_equal(200, a.selection)
    assert_equal(['y', 'n'], a.filterkeys)
  END
  v9.CheckSourceSuccess(lines)

  # Use the popup from a def function
  lines =<< trim END
    vim9script

    class A
      var selection: number = -1
      var filterkeys: list<string> = []

      def PopupFilter(id: number, key: string): bool
        add(this.filterkeys, key)
        return popup_filter_yesno(id, key)
      enddef

      def PopupCb(id: number, result: number)
        this.selection = result ? 100 : 200
      enddef
    endclass

    def Foo()
      var a = A.new()
      feedkeys('', 'xt')
      var winid = popup_create('Y/N?',
                                {filter: a.PopupFilter, callback: a.PopupCb})
      feedkeys('y', 'xt')
      popup_close(winid)
      assert_equal(100, a.selection)
      assert_equal(['y'], a.filterkeys)
      feedkeys('', 'xt')
      winid = popup_create('Y/N?',
                                {filter: a.PopupFilter, callback: a.PopupCb})
      feedkeys('n', 'xt')
      popup_close(winid)
      assert_equal(200, a.selection)
      assert_equal(['y', 'n'], a.filterkeys)
    enddef
    Foo()
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using class methods as popup callback functions
def Test_classmethod_popup_callback()
  # Use the popup from the script level
  var lines =<< trim END
    vim9script

    class A
      static var selection: number = -1
      static var filterkeys: list<string> = []

      static def PopupFilter(id: number, key: string): bool
        add(filterkeys, key)
        return popup_filter_yesno(id, key)
      enddef

      static def PopupCb(id: number, result: number)
        selection = result ? 100 : 200
      enddef
    endclass

    feedkeys('', 'xt')
    var winid = popup_create('Y/N?',
                              {filter: A.PopupFilter, callback: A.PopupCb})
    feedkeys('y', 'xt')
    popup_close(winid)
    assert_equal(100, A.selection)
    assert_equal(['y'], A.filterkeys)
    feedkeys('', 'xt')
    winid = popup_create('Y/N?',
                              {filter: A.PopupFilter, callback: A.PopupCb})
    feedkeys('n', 'xt')
    popup_close(winid)
    assert_equal(200, A.selection)
    assert_equal(['y', 'n'], A.filterkeys)
  END
  v9.CheckSourceSuccess(lines)

  # Use the popup from a def function
  lines =<< trim END
    vim9script

    class A
      static var selection: number = -1
      static var filterkeys: list<string> = []

      static def PopupFilter(id: number, key: string): bool
        add(filterkeys, key)
        return popup_filter_yesno(id, key)
      enddef

      static def PopupCb(id: number, result: number)
        selection = result ? 100 : 200
      enddef
    endclass

    def Foo()
      feedkeys('', 'xt')
      var winid = popup_create('Y/N?',
                                {filter: A.PopupFilter, callback: A.PopupCb})
      feedkeys('y', 'xt')
      popup_close(winid)
      assert_equal(100, A.selection)
      assert_equal(['y'], A.filterkeys)
      feedkeys('', 'xt')
      winid = popup_create('Y/N?',
                                {filter: A.PopupFilter, callback: A.PopupCb})
      feedkeys('n', 'xt')
      popup_close(winid)
      assert_equal(200, A.selection)
      assert_equal(['y', 'n'], A.filterkeys)
    enddef
    Foo()
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using an object method as a timer callback function
def Test_objmethod_timer_callback()
  # Use the timer callback from script level
  var lines =<< trim END
    vim9script

    class A
      var timerTick: number = -1
      def TimerCb(timerID: number)
        this.timerTick = 6
      enddef
    endclass

    var a = A.new()
    timer_start(0, a.TimerCb)
    var maxWait = 5
    while maxWait > 0 && a.timerTick == -1
      :sleep 10m
      maxWait -= 1
    endwhile
    assert_equal(6, a.timerTick)
  END
  v9.CheckSourceSuccess(lines)

  # Use the timer callback from a def function
  lines =<< trim END
    vim9script

    class A
      var timerTick: number = -1
      def TimerCb(timerID: number)
        this.timerTick = 6
      enddef
    endclass

    def Foo()
      var a = A.new()
      timer_start(0, a.TimerCb)
      var maxWait = 5
      while maxWait > 0 && a.timerTick == -1
        :sleep 10m
        maxWait -= 1
      endwhile
      assert_equal(6, a.timerTick)
    enddef
    Foo()
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using a class method as a timer callback function
def Test_classmethod_timer_callback()
  # Use the timer callback from script level
  var lines =<< trim END
    vim9script

    class A
      static var timerTick: number = -1
      static def TimerCb(timerID: number)
        timerTick = 6
      enddef
    endclass

    timer_start(0, A.TimerCb)
    var maxWait = 5
    while maxWait > 0 && A.timerTick == -1
      :sleep 10m
      maxWait -= 1
    endwhile
    assert_equal(6, A.timerTick)
  END
  v9.CheckSourceSuccess(lines)

  # Use the timer callback from a def function
  lines =<< trim END
    vim9script

    class A
      static var timerTick: number = -1
      static def TimerCb(timerID: number)
        timerTick = 6
      enddef
    endclass

    def Foo()
      timer_start(0, A.TimerCb)
      var maxWait = 5
      while maxWait > 0 && A.timerTick == -1
        :sleep 10m
        maxWait -= 1
      endwhile
      assert_equal(6, A.timerTick)
    enddef
    Foo()
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for using a class variable as the first and/or second operand of a binary
" operator.
def Test_class_variable_as_operands()
  var lines =<< trim END
    vim9script
    class Tests
      static var truthy: bool = true
      public static var TruthyFn: func
      static var list: list<any> = []
      static var four: number = 4
      static var str: string = 'hello'

      static def Str(): string
        return str
      enddef

      static def Four(): number
        return four
      enddef

      static def List(): list<any>
        return list
      enddef

      static def Truthy(): bool
        return truthy
      enddef

      def TestOps()
        assert_true(Tests.truthy == truthy)
        assert_true(truthy == Tests.truthy)
        assert_true(Tests.list isnot [])
        assert_true([] isnot Tests.list)
        assert_equal(2, Tests.four >> 1)
        assert_equal(16, 1 << Tests.four)
        assert_equal(8, Tests.four + four)
        assert_equal(8, four + Tests.four)
        assert_equal('hellohello', Tests.str .. str)
        assert_equal('hellohello', str .. Tests.str)

        # Using class variable for list indexing
        var l = range(10)
        assert_equal(4, l[Tests.four])
        assert_equal([4, 5, 6], l[Tests.four : Tests.four + 2])

        # Using class variable for Dict key
        var d = {hello: 'abc'}
        assert_equal('abc', d[Tests.str])
      enddef
    endclass

    def TestOps2()
      assert_true(Tests.truthy == Tests.Truthy())
      assert_true(Tests.Truthy() == Tests.truthy)
      assert_true(Tests.truthy == Tests.TruthyFn())
      assert_true(Tests.TruthyFn() == Tests.truthy)
      assert_true(Tests.list is Tests.List())
      assert_true(Tests.List() is Tests.list)
      assert_equal(2, Tests.four >> 1)
      assert_equal(16, 1 << Tests.four)
      assert_equal(8, Tests.four + Tests.Four())
      assert_equal(8, Tests.Four() + Tests.four)
      assert_equal('hellohello', Tests.str .. Tests.Str())
      assert_equal('hellohello', Tests.Str() .. Tests.str)

      # Using class variable for list indexing
      var l = range(10)
      assert_equal(4, l[Tests.four])
      assert_equal([4, 5, 6], l[Tests.four : Tests.four + 2])

      # Using class variable for Dict key
      var d = {hello: 'abc'}
      assert_equal('abc', d[Tests.str])
    enddef

    Tests.TruthyFn = Tests.Truthy
    var t = Tests.new()
    t.TestOps()
    TestOps2()

    assert_true(Tests.truthy == Tests.Truthy())
    assert_true(Tests.Truthy() == Tests.truthy)
    assert_true(Tests.truthy == Tests.TruthyFn())
    assert_true(Tests.TruthyFn() == Tests.truthy)
    assert_true(Tests.list is Tests.List())
    assert_true(Tests.List() is Tests.list)
    assert_equal(2, Tests.four >> 1)
    assert_equal(16, 1 << Tests.four)
    assert_equal(8, Tests.four + Tests.Four())
    assert_equal(8, Tests.Four() + Tests.four)
    assert_equal('hellohello', Tests.str .. Tests.Str())
    assert_equal('hellohello', Tests.Str() .. Tests.str)

    # Using class variable for list indexing
    var l = range(10)
    assert_equal(4, l[Tests.four])
    assert_equal([4, 5, 6], l[Tests.four : Tests.four + 2])

    # Using class variable for Dict key
    var d = {hello: 'abc'}
    assert_equal('abc', d[Tests.str])
  END
  v9.CheckSourceSuccess(lines)
enddef

" Test for checking the type of the key used to access an object dict member.
def Test_dict_member_key_type_check()
  var lines =<< trim END
    vim9script

    abstract class State
      var numbers: dict<string> = {0: 'nil', 1: 'unity'}
    endclass

    class Test extends State
      def ObjMethodTests()
        var cursor: number = 0
        var z: number = 0
        [this.numbers[cursor]] = ['zero.1']
        assert_equal({0: 'zero.1', 1: 'unity'}, this.numbers)
        [this.numbers[string(cursor)], z] = ['zero.2', 1]
        assert_equal({0: 'zero.2', 1: 'unity'}, this.numbers)
        [z, this.numbers[string(cursor)]] = [1, 'zero.3']
        assert_equal({0: 'zero.3', 1: 'unity'}, this.numbers)
        [this.numbers[cursor], z] = ['zero.4', 1]
        assert_equal({0: 'zero.4', 1: 'unity'}, this.numbers)
        [z, this.numbers[cursor]] = [1, 'zero.5']
        assert_equal({0: 'zero.5', 1: 'unity'}, this.numbers)
      enddef

      static def ClassMethodTests(that: State)
        var cursor: number = 0
        var z: number = 0
        [that.numbers[cursor]] = ['zero.1']
        assert_equal({0: 'zero.1', 1: 'unity'}, that.numbers)
        [that.numbers[string(cursor)], z] = ['zero.2', 1]
        assert_equal({0: 'zero.2', 1: 'unity'}, that.numbers)
        [z, that.numbers[string(cursor)]] = [1, 'zero.3']
        assert_equal({0: 'zero.3', 1: 'unity'}, that.numbers)
        [that.numbers[cursor], z] = ['zero.4', 1]
        assert_equal({0: 'zero.4', 1: 'unity'}, that.numbers)
        [z, that.numbers[cursor]] = [1, 'zero.5']
        assert_equal({0: 'zero.5', 1: 'unity'}, that.numbers)
      enddef

      def new()
      enddef

      def newMethodTests()
        var cursor: number = 0
        var z: number
        [this.numbers[cursor]] = ['zero.1']
        assert_equal({0: 'zero.1', 1: 'unity'}, this.numbers)
        [this.numbers[string(cursor)], z] = ['zero.2', 1]
        assert_equal({0: 'zero.2', 1: 'unity'}, this.numbers)
        [z, this.numbers[string(cursor)]] = [1, 'zero.3']
        assert_equal({0: 'zero.3', 1: 'unity'}, this.numbers)
        [this.numbers[cursor], z] = ['zero.4', 1]
        assert_equal({0: 'zero.4', 1: 'unity'}, this.numbers)
        [z, this.numbers[cursor]] = [1, 'zero.5']
        assert_equal({0: 'zero.5', 1: 'unity'}, this.numbers)
      enddef
    endclass

    def DefFuncTests(that: Test)
      var cursor: number = 0
      var z: number
      [that.numbers[cursor]] = ['zero.1']
      assert_equal({0: 'zero.1', 1: 'unity'}, that.numbers)
      [that.numbers[string(cursor)], z] = ['zero.2', 1]
      assert_equal({0: 'zero.2', 1: 'unity'}, that.numbers)
      [z, that.numbers[string(cursor)]] = [1, 'zero.3']
      assert_equal({0: 'zero.3', 1: 'unity'}, that.numbers)
      [that.numbers[cursor], z] = ['zero.4', 1]
      assert_equal({0: 'zero.4', 1: 'unity'}, that.numbers)
      [z, that.numbers[cursor]] = [1, 'zero.5']
      assert_equal({0: 'zero.5', 1: 'unity'}, that.numbers)
    enddef

    Test.newMethodTests()
    Test.new().ObjMethodTests()
    Test.ClassMethodTests(Test.new())
    DefFuncTests(Test.new())

    const test: Test = Test.new()
    var cursor: number = 0
    [test.numbers[cursor], cursor] = ['zero', 1]
    [cursor, test.numbers[cursor]] = [1, 'one']
    assert_equal({0: 'zero', 1: 'one'}, test.numbers)
  END
  v9.CheckSourceSuccess(lines)

  lines =<< trim END
    vim9script

    class A
      var numbers: dict<string> = {a: '1', b: '2'}

      def new()
      enddef

      def Foo()
        var z: number
        [this.numbers.a, z] = [{}, 10]
      enddef
    endclass

    var a = A.new()
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected string but got dict<any>', 2)

  lines =<< trim END
    vim9script

    class A
      var numbers: dict<number> = {a: 1, b: 2}

      def new()
      enddef

      def Foo()
        var x: string = 'a'
        var y: number
        [this.numbers[x], y] = [{}, 10]
      enddef
    endclass

    var a = A.new()
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1012: Type mismatch; expected number but got dict<any>', 3)
enddef

def Test_compile_many_def_functions_in_funcref_instr()
  # This used to crash Vim.  This is reproducible only when run on new instance
  # of Vim.
  var lines =<< trim END
    vim9script

    class A
      def new()
        this.TakeFunc(this.F00)
      enddef

      def TakeFunc(F: func)
      enddef

      def F00()
        this.F01()
        this.F02()
        this.F03()
        this.F04()
        this.F05()
        this.F06()
        this.F07()
        this.F08()
        this.F09()
        this.F10()
        this.F11()
        this.F12()
        this.F13()
        this.F14()
        this.F15()
        this.F16()
        this.F17()
        this.F18()
        this.F19()
        this.F20()
        this.F21()
        this.F22()
        this.F23()
        this.F24()
        this.F25()
        this.F26()
        this.F27()
        this.F28()
        this.F29()
        this.F30()
        this.F31()
        this.F32()
        this.F33()
        this.F34()
        this.F35()
        this.F36()
        this.F37()
        this.F38()
        this.F39()
        this.F40()
        this.F41()
        this.F42()
        this.F43()
        this.F44()
        this.F45()
        this.F46()
        this.F47()
      enddef

      def F01()
      enddef
      def F02()
      enddef
      def F03()
      enddef
      def F04()
      enddef
      def F05()
      enddef
      def F06()
      enddef
      def F07()
      enddef
      def F08()
      enddef
      def F09()
      enddef
      def F10()
      enddef
      def F11()
      enddef
      def F12()
      enddef
      def F13()
      enddef
      def F14()
      enddef
      def F15()
      enddef
      def F16()
      enddef
      def F17()
      enddef
      def F18()
      enddef
      def F19()
      enddef
      def F20()
      enddef
      def F21()
      enddef
      def F22()
      enddef
      def F23()
      enddef
      def F24()
      enddef
      def F25()
      enddef
      def F26()
      enddef
      def F27()
      enddef
      def F28()
      enddef
      def F29()
      enddef
      def F30()
      enddef
      def F31()
      enddef
      def F32()
      enddef
      def F33()
      enddef
      def F34()
      enddef
      def F35()
      enddef
      def F36()
      enddef
      def F37()
      enddef
      def F38()
      enddef
      def F39()
      enddef
      def F40()
      enddef
      def F41()
      enddef
      def F42()
      enddef
      def F43()
      enddef
      def F44()
      enddef
      def F45()
      enddef
      def F46()
      enddef
      def F47()
      enddef
    endclass

    A.new()
  END
  writefile(lines, 'Xscript', 'D')
  g:RunVim([], [], '-u NONE -S Xscript -c qa')
  assert_equal(0, v:shell_error)
enddef

" Test for 'final' class and object variables
def Test_final_class_object_variable()
  # Test for changing a final object variable from an object function
  var lines =<< trim END
    vim9script
    class A
      final foo: string = "abc"
      def Foo()
        this.foo = "def"
      enddef
    endclass
    defcompile A.Foo
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "foo" in class "A"', 1)

  # Test for changing a final object variable from the 'new' function
  lines =<< trim END
    vim9script
    class A
      final s1: string
      final s2: string
      def new(this.s1)
        this.s2 = 'def'
      enddef
    endclass
    var a = A.new('abc')
    assert_equal('abc', a.s1)
    assert_equal('def', a.s2)
  END
  v9.CheckSourceSuccess(lines)

  # Test for a final class variable
  lines =<< trim END
    vim9script
    class A
      static final s1: string = "abc"
    endclass
    assert_equal('abc', A.s1)
  END
  v9.CheckSourceSuccess(lines)

  # Test for changing a final class variable from a class function
  lines =<< trim END
    vim9script
    class A
      static final s1: string = "abc"
      static def Foo()
        s1 = "def"
      enddef
    endclass
    A.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "s1" in class "A"', 1)

  # Test for changing a public final class variable at script level
  lines =<< trim END
    vim9script
    class A
      public static final s1: string = "abc"
    endclass
    assert_equal('abc', A.s1)
    A.s1 = 'def'
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "s1" in class "A"', 6)

  # Test for changing a public final class variable from a class function
  lines =<< trim END
    vim9script
    class A
      public static final s1: string = "abc"
      static def Foo()
        s1 = "def"
      enddef
    endclass
    A.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "s1" in class "A"', 1)

  # Test for changing a public final class variable from a function
  lines =<< trim END
    vim9script
    class A
      public static final s1: string = "abc"
    endclass
    def Foo()
      A.s1 = 'def'
    enddef
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "s1" in class "A"', 1)

  # Test for using a final variable of composite type
  lines =<< trim END
    vim9script
    class A
      public final l: list<number>
      def new()
        this.l = [1, 2]
      enddef
      def Foo()
        this.l[0] = 3
        this.l->add(4)
      enddef
    endclass
    var a = A.new()
    assert_equal([1, 2], a.l)
    a.Foo()
    assert_equal([3, 2, 4], a.l)
  END
  v9.CheckSourceSuccess(lines)

  # Test for changing a final variable of composite type from another object
  # function
  lines =<< trim END
    vim9script
    class A
      public final l: list<number> = [1, 2]
      def Foo()
        this.l = [3, 4]
      enddef
    endclass
    var a = A.new()
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "l" in class "A"', 1)

  # Test for modifying a final variable of composite type at script level
  lines =<< trim END
    vim9script
    class A
      public final l: list<number> = [1, 2]
    endclass
    var a = A.new()
    a.l[0] = 3
    a.l->add(4)
    assert_equal([3, 2, 4], a.l)
  END
  v9.CheckSourceSuccess(lines)

  # Test for modifying a final variable of composite type from a function
  lines =<< trim END
    vim9script
    class A
      public final l: list<number> = [1, 2]
    endclass
    def Foo()
      var a = A.new()
      a.l[0] = 3
      a.l->add(4)
      assert_equal([3, 2, 4], a.l)
    enddef
    Foo()
  END
  v9.CheckSourceSuccess(lines)

  # Test for modifying a final variable of composite type from another object
  # function
  lines =<< trim END
    vim9script
    class A
      public final l: list<number> = [1, 2]
      def Foo()
        this.l[0] = 3
        this.l->add(4)
      enddef
    endclass
    var a = A.new()
    a.Foo()
    assert_equal([3, 2, 4], a.l)
  END
  v9.CheckSourceSuccess(lines)

  # Test for assigning a new value to a final variable of composite type at
  # script level
  lines =<< trim END
    vim9script
    class A
      public final l: list<number> = [1, 2]
    endclass
    var a = A.new()
    a.l = [3, 4]
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "l" in class "A"', 6)

  # Test for assigning a new value to a final variable of composite type from
  # another object function
  lines =<< trim END
    vim9script
    class A
      public final l: list<number> = [1, 2]
      def Foo()
        this.l = [3, 4]
      enddef
    endclass
    var a = A.new()
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "l" in class "A"', 1)

  # Test for assigning a new value to a final variable of composite type from
  # another function
  lines =<< trim END
    vim9script
    class A
      public final l: list<number> = [1, 2]
    endclass
    def Foo()
      var a = A.new()
      a.l = [3, 4]
    enddef
    Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "l" in class "A"', 2)

  # Error case: Use 'final' with just a variable name
  lines =<< trim END
    vim9script
    class A
      final foo
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)

  # Error case: Use 'final' followed by 'public'
  lines =<< trim END
    vim9script
    class A
      final public foo: number
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)

  # Error case: Use 'final' followed by 'static'
  lines =<< trim END
    vim9script
    class A
      final static foo: number
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)

  # Error case: 'final' cannot be used in an interface
  lines =<< trim END
    vim9script
    interface A
      final foo: number = 10
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1408: Final variable not supported in an interface', 3)

  # Error case: 'final' not supported for an object method
  lines =<< trim END
    vim9script
    class A
      final def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)

  # Error case: 'final' not supported for a class method
  lines =<< trim END
    vim9script
    class A
      static final def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)
enddef

" Test for 'const' class and object variables
def Test_const_class_object_variable()
  # Test for changing a const object variable from an object function
  var lines =<< trim END
    vim9script
    class A
      const foo: string = "abc"
      def Foo()
        this.foo = "def"
      enddef
    endclass
    defcompile A.Foo
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "foo" in class "A"', 1)

  # Test for changing a const object variable from the 'new' function
  lines =<< trim END
    vim9script
    class A
      const s1: string
      const s2: string
      def new(this.s1)
        this.s2 = 'def'
      enddef
    endclass
    var a = A.new('abc')
    assert_equal('abc', a.s1)
    assert_equal('def', a.s2)
  END
  v9.CheckSourceSuccess(lines)

  # Test for changing a const object variable from an object method called from
  # the 'new' function
  lines =<< trim END
    vim9script
    class A
      const s1: string = 'abc'
      def new()
        this.ChangeStr()
      enddef
      def ChangeStr()
        this.s1 = 'def'
      enddef
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "s1" in class "A"', 1)

  # Test for a const class variable
  lines =<< trim END
    vim9script
    class A
      static const s1: string = "abc"
    endclass
    assert_equal('abc', A.s1)
  END
  v9.CheckSourceSuccess(lines)

  # Test for changing a const class variable from a class function
  lines =<< trim END
    vim9script
    class A
      static const s1: string = "abc"
      static def Foo()
        s1 = "def"
      enddef
    endclass
    A.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "s1" in class "A"', 1)

  # Test for changing a public const class variable at script level
  lines =<< trim END
    vim9script
    class A
      public static const s1: string = "abc"
    endclass
    assert_equal('abc', A.s1)
    A.s1 = 'def'
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "s1" in class "A"', 6)

  # Test for changing a public const class variable from a class function
  lines =<< trim END
    vim9script
    class A
      public static const s1: string = "abc"
      static def Foo()
        s1 = "def"
      enddef
    endclass
    A.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "s1" in class "A"', 1)

  # Test for changing a public const class variable from a function
  lines =<< trim END
    vim9script
    class A
      public static const s1: string = "abc"
    endclass
    def Foo()
      A.s1 = 'def'
    enddef
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "s1" in class "A"', 1)

  # Test for changing a const List item from an object function
  lines =<< trim END
    vim9script
    class A
      public const l: list<number>
      def new()
        this.l = [1, 2]
      enddef
      def Foo()
        this.l[0] = 3
      enddef
    endclass
    var a = A.new()
    assert_equal([1, 2], a.l)
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1119: Cannot change locked list item', 1)

  # Test for adding a value to a const List from an object function
  lines =<< trim END
    vim9script
    class A
      public const l: list<number>
      def new()
        this.l = [1, 2]
      enddef
      def Foo()
        this.l->add(3)
      enddef
    endclass
    var a = A.new()
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E741: Value is locked: add() argument', 1)

  # Test for reassigning a const List from an object function
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
      def Foo()
        this.l = [3, 4]
      enddef
    endclass
    var a = A.new()
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "l" in class "A"', 1)

  # Test for changing a const List item at script level
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
    endclass
    var a = A.new()
    a.l[0] = 3
  END
  v9.CheckSourceFailure(lines, 'E741: Value is locked:',  6)

  # Test for adding a value to a const List item at script level
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
    endclass
    var a = A.new()
    a.l->add(4)
  END
  v9.CheckSourceFailure(lines, 'E741: Value is locked:', 6)

  # Test for changing a const List item from a function
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
    endclass
    def Foo()
      var a = A.new()
      a.l[0] = 3
    enddef
    Foo()
  END
  v9.CheckSourceFailure(lines, 'E1119: Cannot change locked list item', 2)

  # Test for adding a value to a const List item from a function
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
    endclass
    def Foo()
      var a = A.new()
      a.l->add(4)
    enddef
    Foo()
  END
  v9.CheckSourceFailure(lines, 'E741: Value is locked: add() argument', 2)

  # Test for changing a const List item from an object method
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
      def Foo()
        this.l[0] = 3
      enddef
    endclass
    var a = A.new()
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1119: Cannot change locked list item', 1)

  # Test for adding a value to a const List item from an object method
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
      def Foo()
        this.l->add(4)
      enddef
    endclass
    var a = A.new()
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E741: Value is locked: add() argument', 1)

  # Test for reassigning a const List object variable at script level
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
    endclass
    var a = A.new()
    a.l = [3, 4]
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "l" in class "A"', 6)

  # Test for reassigning a const List object variable from an object method
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
      def Foo()
        this.l = [3, 4]
      enddef
    endclass
    var a = A.new()
    a.Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "l" in class "A"', 1)

  # Test for reassigning a const List object variable from another function
  lines =<< trim END
    vim9script
    class A
      public const l: list<number> = [1, 2]
    endclass
    def Foo()
      var a = A.new()
      a.l = [3, 4]
    enddef
    Foo()
  END
  v9.CheckSourceFailure(lines, 'E1409: Cannot change read-only variable "l" in class "A"', 2)

  # Error case: Use 'const' with just a variable name
  lines =<< trim END
    vim9script
    class A
      const foo
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)

  # Error case: Use 'const' followed by 'public'
  lines =<< trim END
    vim9script
    class A
      const public foo: number
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)

  # Error case: Use 'const' followed by 'static'
  lines =<< trim END
    vim9script
    class A
      const static foo: number
    endclass
    var a = A.new()
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)

  # Error case: 'const' cannot be used in an interface
  lines =<< trim END
    vim9script
    interface A
      const foo: number = 10
    endinterface
  END
  v9.CheckSourceFailure(lines, 'E1410: Const variable not supported in an interface', 3)

  # Error case: 'const' not supported for an object method
  lines =<< trim END
    vim9script
    class A
      const def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)

  # Error case: 'const' not supported for a class method
  lines =<< trim END
    vim9script
    class A
      static const def Foo()
      enddef
    endclass
  END
  v9.CheckSourceFailure(lines, 'E1022: Type or initialization required', 3)
enddef

" Test for using double underscore prefix in a class/object method name.
def Test_method_double_underscore_prefix()
  # class method
  var lines =<< trim END
    vim9script
    class A
      static def __foo()
        echo "foo"
      enddef
    endclass
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1034: Cannot use reserved name __foo()', 3)

  # object method
  lines =<< trim END
    vim9script
    class A
      def __foo()
        echo "foo"
      enddef
    endclass
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E1034: Cannot use reserved name __foo()', 3)
enddef

" Test for compiling class/object methods using :defcompile
def Test_defcompile_class()
  # defcompile all the classes in the current script
  var lines =<< trim END
    vim9script
    class A
      def Foo()
        var i = 10
      enddef
    endclass
    class B
      def Bar()
        var i = 20
        xxx
      enddef
    endclass
    defcompile
  END
  v9.CheckSourceFailure(lines, 'E476: Invalid command: xxx', 2)

  # defcompile a specific class
  lines =<< trim END
    vim9script
    class A
      def Foo()
        xxx
      enddef
    endclass
    class B
      def Bar()
        yyy
      enddef
    endclass
    defcompile B
  END
  v9.CheckSourceFailure(lines, 'E476: Invalid command: yyy', 1)

  # defcompile a non-class
  lines =<< trim END
    vim9script
    class A
      def Foo()
      enddef
    endclass
    var X: list<number> = []
    defcompile X
  END
  v9.CheckSourceFailure(lines, 'E1061: Cannot find function X', 7)

  # defcompile a class twice
  lines =<< trim END
    vim9script
    class A
      def new()
      enddef
    endclass
    defcompile A
    defcompile A
    assert_equal('Function A.new does not need compiling', v:statusmsg)
  END
  v9.CheckSourceSuccess(lines)

  # defcompile should not compile an imported class
  lines =<< trim END
    vim9script
    export class A
      def Foo()
        xxx
      enddef
    endclass
  END
  writefile(lines, 'Xdefcompileimport.vim', 'D')
  lines =<< trim END
    vim9script

    import './Xdefcompileimport.vim'
    class B
    endclass
    defcompile
  END
  v9.CheckScriptSuccess(lines)
enddef

" vim: ts=8 sw=2 sts=2 expandtab tw=80 fdm=marker
