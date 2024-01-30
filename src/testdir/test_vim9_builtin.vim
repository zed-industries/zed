" Test using builtin functions in the Vim9 script language.

source check.vim
source screendump.vim
import './vim9.vim' as v9

" Test for passing too many or too few arguments to builtin functions
func Test_internalfunc_arg_error()
  let l =<< trim END
    def! FArgErr(): float
      return ceil(1.1, 2)
    enddef
    defcompile
  END
  call writefile(l, 'Xinvalidarg', 'D')
  call assert_fails('so Xinvalidarg', 'E118:', '', 1, 'FArgErr')
  let l =<< trim END
    def! FArgErr(): float
      return ceil()
    enddef
    defcompile
  END
  call writefile(l, 'Xinvalidarg')
  call assert_fails('so Xinvalidarg', 'E119:', '', 1, 'FArgErr')
endfunc

" Test for builtin functions returning different types
func Test_InternalFuncRetType()
  let lines =<< trim END
    def RetFloat(): float
      return ceil(1.456)
    enddef

    def RetListAny(): list<any>
      return items({k: 'v'})
    enddef

    def RetListString(): list<string>
      return split('a:b:c', ':')
    enddef

    def RetListDictAny(): list<dict<any>>
      return getbufinfo()
    enddef

    def RetDictNumber(): dict<number>
      return wordcount()
    enddef

    def RetDictString(): dict<string>
      return environ()
    enddef
  END
  call writefile(lines, 'Xscript', 'D')
  source Xscript

  call RetFloat()->assert_equal(2.0)
  call RetListAny()->assert_equal([['k', 'v']])
  call RetListString()->assert_equal(['a', 'b', 'c'])
  call RetListDictAny()->assert_notequal([])
  call RetDictNumber()->assert_notequal({})
  call RetDictString()->assert_notequal({})
endfunc

def Test_abs()
  assert_equal(0, abs(0))
  assert_equal(2, abs(-2))
  assert_equal(3, abs(3))
  v9.CheckDefAndScriptFailure(['abs("text")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal(0, abs(0))
  assert_equal(2.0, abs(-2.0))
  assert_equal(3.0, abs(3.0))
enddef

def Test_add()
  v9.CheckDefAndScriptFailure(['add({}, 1)'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<any>', 'E1226: List or Blob required for argument 1'])
  v9.CheckDefAndScriptFailure(['add([])'], 'E119:')
  v9.CheckDefExecFailure([
        'var ln: list<number> = [1]',
        'add(ln, "a")'],
        'E1012: Type mismatch; expected number but got string')
  assert_equal([1, 'a'], add([1], 'a'))
  assert_equal(0z1234, add(0z12, 0x34))

  var lines =<< trim END
    vim9script
    g:thelist = [1]
    lockvar g:thelist
    def TryChange()
      g:thelist->add(2)
    enddef
    TryChange()
  END
  v9.CheckScriptFailure(lines, 'E741:')
enddef

def Test_add_blob()
  var b1: blob = 0z12
  add(b1, 0x34)
  assert_equal(0z1234, b1)

  var b2: blob # defaults to empty blob
  add(b2, 0x67)
  assert_equal(0z67, b2)

  var lines =<< trim END
      var b: blob
      add(b, "x")
  END
  v9.CheckDefFailure(lines, 'E1012:', 2)

  lines =<< trim END
      add(test_null_blob(), 123)
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1131:', 1)

  lines =<< trim END
      var b: blob = test_null_blob()
      add(b, 123)
  END
  v9.CheckDefExecFailure(lines, 'E1131:', 2)

  # Getting variable with NULL blob fails
  lines =<< trim END
      vim9script
      var b: blob = test_null_blob()
      add(b, 123)
  END
  v9.CheckScriptFailure(lines, 'E1131:', 3)
enddef

def Test_add_list()
  var l: list<number>  # defaults to empty list
  add(l, 9)
  assert_equal([9], l)

  var lines =<< trim END
      var l: list<number>
      add(l, "x")
  END
  v9.CheckDefFailure(lines, 'E1012:', 2)

  lines =<< trim END
      add(test_null_list(), 123)
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1130:', 1)

  lines =<< trim END
      var l: list<number> = test_null_list()
      add(l, 123)
  END
  v9.CheckDefExecFailure(lines, 'E1130:', 2)

  # Getting an uninitialized variable allocates a new list at script level
  lines =<< trim END
      vim9script
      var l: list<number>
      add(l, 123)
  END
  v9.CheckScriptSuccess(lines)

  # Adding to a variable set to a NULL list fails
  lines =<< trim END
      vim9script
      var l: list<number> = test_null_list()
      add(l, 123)
  END
  v9.CheckScriptFailure(lines, 'E1130:', 3)

  lines =<< trim END
      vim9script
      var l: list<string> = ['a']
      l->add(123)
  END
  v9.CheckScriptFailure(lines, 'E1012: Type mismatch; expected string but got number', 3)

  lines =<< trim END
      vim9script
      var l: list<string>
      l->add(123)
  END
  v9.CheckScriptFailure(lines, 'E1012: Type mismatch; expected string but got number', 3)
enddef

def Test_add_const()
  var lines =<< trim END
      const l = [1, 2]
      add(l, 3)
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const list<number>')

  lines =<< trim END
      final l = [1, 2]
      add(l, 3)
      assert_equal([1, 2, 3], l)
  END
  v9.CheckDefSuccess(lines)

  lines =<< trim END
      const b = 0z0102
      add(b,  0z03)
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const blob')
enddef


def Test_and()
  v9.CheckDefAndScriptFailure(['and("x", 0x2)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['and(0x1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_append()
  new
  setline(1, range(3))
  var res1: number = append(1, 'one')
  assert_equal(0, res1)
  var res2: bool = append(3, 'two')
  assert_equal(false, res2)
  assert_equal(['0', 'one', '1', 'two', '2'], getline(1, 6))

  append(0, 'zero')
  assert_equal('zero', getline(1))
  append(0, {a: 10})
  assert_equal("{'a': 10}", getline(1))
  append(0, function('min'))
  assert_equal("function('min')", getline(1))
  v9.CheckDefAndScriptFailure(['append([1], "x")'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['append("", "x")'], 'E1209: Invalid value for a line number')
  v9.CheckDefExecAndScriptFailure(['append(".a", "x")'], 'E1209: Invalid value for a line number')
  # only get one error
  assert_fails('append("''aa", "x")', ['E1209: Invalid value for a line number: "''aa"', 'E1209:'])
  v9.CheckDefExecAndScriptFailure(['append(-1, "x")'], 'E966: Invalid line number: -1')
  bwipe!
enddef

def Test_appendbufline()
  new
  var bnum: number = bufnr()
  :wincmd w
  appendbufline(bnum, 0, range(3))
  var res1: number = appendbufline(bnum, 1, 'one')
  assert_equal(0, res1)
  var res2: bool = appendbufline(bnum, 3, 'two')
  assert_equal(false, res2)
  assert_equal(['0', 'one', '1', 'two', '2', ''], getbufline(bnum, 1, '$'))
  appendbufline(bnum, 0, 'zero')
  assert_equal(['zero'], getbufline(bnum, 1))
  v9.CheckDefAndScriptFailure(['appendbufline([1], 1, "x")'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['appendbufline(1, [1], "x")'], ['E1013: Argument 2: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['appendbufline(' .. bnum .. ', -1, "x")'], 'E966: Invalid line number: -1')
  v9.CheckDefExecAndScriptFailure(['appendbufline(' .. bnum .. ', "$a", "x")'], 'E1030: Using a String as a Number: "$a"')
  assert_fails('appendbufline(' .. bnum .. ', "$a", "x")', ['E1030: Using a String as a Number: "$a"', 'E1030:'])
  v9.CheckDefAndScriptFailure(['appendbufline(1, 1, {"a": 10})'], ['E1013: Argument 3: type mismatch, expected string but got dict<number>', 'E1224: String, Number or List required for argument 3'])
  bnum->bufwinid()->win_gotoid()
  appendbufline('', 0, 'numbers')
  getline(1)->assert_equal('numbers')
  bwipe!
enddef

def Test_argc()
  v9.CheckDefAndScriptFailure(['argc("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_arglistid()
  v9.CheckDefAndScriptFailure(['arglistid("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['arglistid(1, "y")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['arglistid("x", "y")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_argv()
  v9.CheckDefAndScriptFailure(['argv("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['argv(1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['argv("x", "y")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_assert_beeps()
  v9.CheckDefAndScriptFailure(['assert_beeps(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
enddef

def Test_assert_equalfile()
  v9.CheckDefAndScriptFailure(['assert_equalfile(1, "f2")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['assert_equalfile("f1", true)'], ['E1013: Argument 2: type mismatch, expected string but got bool', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['assert_equalfile("f1", "f2", ["a"])'], ['E1013: Argument 3: type mismatch, expected string but got list<string>', 'E1174: String required for argument 3'])
enddef

def Test_assert_exception()
  v9.CheckDefAndScriptFailure(['assert_exception({})'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['assert_exception("E1:", v:null)'], ['E1013: Argument 2: type mismatch, expected string but got special', 'E1174: String required for argument 2'])
enddef

def Test_assert_fails()
  v9.CheckDefAndScriptFailure(['assert_fails([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['assert_fails("a", true)'], ['E1013: Argument 2: type mismatch, expected string but got bool', 'E1222: String or List required for argument 2'])
  v9.CheckDefAndScriptFailure(['assert_fails("a", "b", "c", "d")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  v9.CheckDefAndScriptFailure(['assert_fails("a", "b", "c", 4, 5)'], ['E1013: Argument 5: type mismatch, expected string but got number', 'E1174: String required for argument 5'])
enddef

def Test_assert_inrange()
  v9.CheckDefAndScriptFailure(['assert_inrange("a", 2, 3)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['assert_inrange(1, "b", 3)'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['assert_inrange(1, 2, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['assert_inrange(1, 2, 3, 4)'], ['E1013: Argument 4: type mismatch, expected string but got number', 'E1174: String required for argument 4'])
enddef

def Test_assert_match()
  v9.CheckDefAndScriptFailure(['assert_match({}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', ''])
  v9.CheckDefAndScriptFailure(['assert_match("a", 1)'], ['E1013: Argument 2: type mismatch, expected string but got number', ''])
  v9.CheckDefAndScriptFailure(['assert_match("a", "b", null)'], ['E1013: Argument 3: type mismatch, expected string but got special', ''])
enddef

def Test_assert_nobeep()
  v9.CheckDefAndScriptFailure(['assert_nobeep(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
enddef

def Test_assert_notmatch()
  v9.CheckDefAndScriptFailure(['assert_notmatch({}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', ''])
  v9.CheckDefAndScriptFailure(['assert_notmatch("a", 1)'], ['E1013: Argument 2: type mismatch, expected string but got number', ''])
  v9.CheckDefAndScriptFailure(['assert_notmatch("a", "b", null)'], ['E1013: Argument 3: type mismatch, expected string but got special', ''])
enddef

def Test_assert_report()
  v9.CheckDefAndScriptFailure(['assert_report([1, 2])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1174: String required for argument 1'])
enddef

def Test_autocmd_add()
  v9.CheckDefAndScriptFailure(['autocmd_add({})'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<any>', 'E1211: List required for argument 1'])
enddef

def Test_autocmd_delete()
  v9.CheckDefAndScriptFailure(['autocmd_delete({})'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<any>', 'E1211: List required for argument 1'])
enddef

def Test_autocmd_get()
  v9.CheckDefAndScriptFailure(['autocmd_get(10)'], ['E1013: Argument 1: type mismatch, expected dict<any> but got number', 'E1206: Dictionary required for argument 1'])
enddef

def Test_balloon_show()
  CheckGui
  CheckFeature balloon_eval

  assert_fails('balloon_show(10)', 'E1222:')
  assert_fails('balloon_show(true)', 'E1222:')

  v9.CheckDefAndScriptFailure(['balloon_show(1.2)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['balloon_show({"a": 10})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1222: String or List required for argument 1'])
enddef

def Test_balloon_split()
  CheckFeature balloon_eval_term

  assert_fails('balloon_split([])', 'E1174:')
  assert_fails('balloon_split(true)', 'E1174:')
enddef

def Test_blob2list()
  assert_equal(['x', 'x'], blob2list(0z1234)->map((_, _) => 'x'))
  v9.CheckDefAndScriptFailure(['blob2list(10)'], ['E1013: Argument 1: type mismatch, expected blob but got number', 'E1238: Blob required for argument 1'])
enddef

def Test_browse()
  CheckFeature browse

  v9.CheckDefAndScriptFailure(['browse(2, "title", "dir", "file")'], ['E1013: Argument 1: type mismatch, expected bool but got number', 'E1212: Bool required for argument 1'])
  v9.CheckDefAndScriptFailure(['browse(true, 2, "dir", "file")'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['browse(true, "title", 3, "file")'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  v9.CheckDefAndScriptFailure(['browse(true, "title", "dir", 4)'], ['E1013: Argument 4: type mismatch, expected string but got number', 'E1174: String required for argument 4'])
enddef

def Test_browsedir()
  if has('browse')
    v9.CheckDefAndScriptFailure(['browsedir({}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', 'E1174: String required for argument 1'])
    v9.CheckDefAndScriptFailure(['browsedir("a", [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1174: String required for argument 2'])
  endif
enddef

def Test_bufadd()
  assert_fails('bufadd([])', 'E1174:')
enddef

def Test_bufexists()
  assert_fails('bufexists(true)', 'E1220:')
  bufexists('')->assert_false()
enddef

def Test_buflisted()
  var res: bool = buflisted('asdf')
  assert_equal(false, res)
  assert_fails('buflisted(true)', 'E1220:')
  assert_fails('buflisted([])', 'E1220:')
  buflisted('')->assert_false()
enddef

def Test_bufload()
  assert_fails('bufload([])', 'E1220:')
  bufload('')->assert_equal(0)
enddef

def Test_bufloaded()
  assert_fails('bufloaded(true)', 'E1220:')
  assert_fails('bufloaded([])', 'E1220:')
  bufloaded('')->assert_false()
enddef

def Test_bufname()
  split SomeFile
  bufname('%')->assert_equal('SomeFile')
  edit OtherFile
  bufname('#')->assert_equal('SomeFile')
  close
  assert_fails('bufname(true)', 'E1220:')
  assert_fails('bufname([])', 'E1220:')
enddef

let s:bufnr_res = 0

def Test_bufnr()
  var buf = bufnr()
  bufnr('%')->assert_equal(buf)

  # check the lock is not taken over through the stack
  const nr = 10
  bufnr_res = bufnr()
  bufnr_res = 12345

  buf = bufnr('Xdummy', true)
  buf->assert_notequal(-1)
  exe 'bwipe! ' .. buf
  v9.CheckDefAndScriptFailure(['bufnr([1])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['bufnr(1, 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
enddef

def Test_bufwinid()
  var origwin = win_getid()
  below split SomeFile
  var SomeFileID = win_getid()
  below split OtherFile
  below split SomeFile
  bufwinid('SomeFile')->assert_equal(SomeFileID)

  win_gotoid(origwin)
  only
  bwipe SomeFile
  bwipe OtherFile

  assert_fails('bufwinid(true)', 'E1220:')
  assert_fails('bufwinid([])', 'E1220:')
enddef

def Test_bufwinnr()
  assert_fails('bufwinnr(true)', 'E1220:')
  assert_fails('bufwinnr([])', 'E1220:')
enddef

def Test_byte2line()
  v9.CheckDefAndScriptFailure(['byte2line("1")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['byte2line([])'], ['E1013: Argument 1: type mismatch, expected number but got list<any>', 'E1210: Number required for argument 1'])
  byte2line(0)->assert_equal(-1)
enddef

def Test_byteidx()
  v9.CheckDefAndScriptFailure(['byteidx(1, 2)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['byteidx("a", "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['byteidx("a", 0, "")'], ['E1013: Argument 3: type mismatch, expected bool but got string', 'E1212: Bool required for argument 3'])
  byteidx('', 0)->assert_equal(0)
  byteidx('', 1)->assert_equal(-1)
enddef

def Test_byteidxcomp()
  v9.CheckDefAndScriptFailure(['byteidxcomp(1, 2)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['byteidxcomp("a", "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['byteidxcomp("a", 0, "")'], ['E1013: Argument 3: type mismatch, expected bool but got string', 'E1212: Bool required for argument 3'])
enddef

def Test_call_call()
  var l = [3, 2, 1]
  call('reverse', [l])
  l->assert_equal([1, 2, 3])

  var lines =<< trim END
      vim9script
      def Outer()
        def g:Inner()
          g:done = 'Inner'
        enddef
        call(g:Inner, [])
      enddef
      Outer()
      assert_equal('Inner', g:done)
      unlet g:done
  END
  v9.CheckScriptSuccess(lines)
  delfunc g:Inner

  v9.CheckDefExecAndScriptFailure(['call(123, [2])'], 'E1256: String or function required for argument 1')
  v9.CheckDefExecAndScriptFailure(['call(true, [2])'], 'E1256: String or function required for argument 1')
  v9.CheckDefAndScriptFailure(['call("reverse", 2)'], ['E1013: Argument 2: type mismatch, expected list<any> but got number', 'E1211: List required for argument 2'])
  v9.CheckDefAndScriptFailure(['call("reverse", [2], [1])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])
enddef

def Test_ch_canread()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_canread(10)'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
  endif
enddef

def Test_ch_close()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_close("c")'], ['E1013: Argument 1: type mismatch, expected channel but got string', 'E1217: Channel or Job required for argument 1'])
  endif
enddef

def Test_ch_close_in()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_close_in(true)'], ['E1013: Argument 1: type mismatch, expected channel but got bool', 'E1217: Channel or Job required for argument 1'])
  endif
enddef

def Test_ch_evalexpr()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_evalexpr(1, "a")'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_evalexpr(test_null_channel(), 1, [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
  endif
enddef

def Test_ch_evalraw()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_evalraw(1, "")'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_evalraw(test_null_channel(), 1)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1221: String or Blob required for argument 2'])
    v9.CheckDefAndScriptFailure(['ch_evalraw(test_null_channel(), "", [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
  endif
enddef

def Test_ch_getbufnr()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_getbufnr(1, "a")'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_getbufnr(test_null_channel(), 1)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
    # test empty string argument for ch_getbufnr()
    var job: job = job_start(&shell)
    g:WaitForAssert(() => assert_equal('run', job_status(job)))
    job->ch_getbufnr('')->assert_equal(-1)
    job_stop(job)
  endif
enddef

def Test_ch_getjob()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_getjob(1)'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_getjob({"a": 10})'], ['E1013: Argument 1: type mismatch, expected channel but got dict<number>', 'E1217: Channel or Job required for argument 1'])
    assert_equal(0, ch_getjob(test_null_channel()))
  endif
enddef

def Test_ch_info()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_info([1])'], ['E1013: Argument 1: type mismatch, expected channel but got list<number>', 'E1217: Channel or Job required for argument 1'])
  endif
enddef

def Test_ch_log()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_log(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1174: String required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_log("a", 1)'], ['E1013: Argument 2: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 2'])
  endif
enddef

def Test_ch_logfile()
  if !has('channel')
    CheckFeature channel
  else
    assert_fails('ch_logfile(true)', 'E1174:')
    assert_fails('ch_logfile("foo", true)', 'E1174:')
    ch_logfile('', '')->assert_equal(0)

    v9.CheckDefAndScriptFailure(['ch_logfile(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_logfile("a", true)'], ['E1013: Argument 2: type mismatch, expected string but got bool', 'E1174: String required for argument 2'])
  endif
enddef

def Test_ch_open()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_open({"a": 10}, "a")'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_open("a", [1])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 2'])
    v9.CheckDefExecAndScriptFailure(['ch_open("")'], 'E475: Invalid argument')
  endif
enddef

def Test_ch_read()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_read(1)'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_read(test_null_channel(), [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
  endif
enddef

def Test_ch_readblob()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_readblob(1)'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_readblob(test_null_channel(), [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
  endif
enddef

def Test_ch_readraw()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_readraw(1)'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_readraw(test_null_channel(), [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
  endif
enddef

def Test_ch_sendexpr()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_sendexpr(1, "a")'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_sendexpr(test_null_channel(), 1, [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
  endif
enddef

def Test_ch_sendraw()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_sendraw(1, "")'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_sendraw(test_null_channel(), 1)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1221: String or Blob required for argument 2'])
    v9.CheckDefAndScriptFailure(['ch_sendraw(test_null_channel(), "", [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
  endif
enddef

def Test_ch_setoptions()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_setoptions(1, {})'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_setoptions(test_null_channel(), [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
  endif
enddef

def Test_ch_status()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['ch_status(1)'], ['E1013: Argument 1: type mismatch, expected channel but got number', 'E1217: Channel or Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['ch_status(test_null_channel(), [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
  endif
enddef

def Test_char2nr()
  char2nr('あ', true)->assert_equal(12354)

  assert_fails('char2nr(true)', 'E1174:')
  v9.CheckDefAndScriptFailure(['char2nr(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['char2nr("a", 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
  assert_equal(97, char2nr('a', 1))
  assert_equal(97, char2nr('a', 0))
  assert_equal(97, char2nr('a', true))
  assert_equal(97, char2nr('a', false))
  char2nr('')->assert_equal(0)
enddef

def Test_charclass()
  assert_fails('charclass(true)', 'E1174:')
  charclass('')->assert_equal(0)
enddef

def Test_charcol()
  v9.CheckDefAndScriptFailure(['charcol(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['charcol({a: 10})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['charcol(".", [])'], ['E1013: Argument 2: type mismatch, expected number but got list<any>', 'E1210: Number required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['charcol("")'], 'E1209: Invalid value for a line number')
  new
  setline(1, ['abcdefgh'])
  cursor(1, 4)
  assert_equal(4, charcol('.'))
  assert_equal(9, charcol([1, '$']))
  assert_equal(0, charcol([10, '$']))
  bw!
enddef

def Test_charidx()
  v9.CheckDefAndScriptFailure(['charidx(0z10, 1)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['charidx("a", "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['charidx("a", 1, "")'], ['E1013: Argument 3: type mismatch, expected bool but got string', 'E1212: Bool required for argument 3'])
  v9.CheckDefAndScriptFailure(['charidx("a", 1, 0, "")'], ['E1013: Argument 4: type mismatch, expected bool but got string', 'E1212: Bool required for argument 4'])
  charidx('', 0)->assert_equal(0)
  charidx('', 1)->assert_equal(-1)
enddef

def Test_chdir()
  assert_fails('chdir(true)', 'E1174:')
enddef

def Test_cindent()
  v9.CheckDefAndScriptFailure(['cindent([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['cindent(null)'], ['E1013: Argument 1: type mismatch, expected string but got special', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['cindent("")'], 'E1209: Invalid value for a line number')
  assert_equal(-1, cindent(0))
  assert_equal(0, cindent('.'))
enddef

def Test_clearmatches()
  v9.CheckDefAndScriptFailure(['clearmatches("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_col()
  new
  setline(1, 'abcdefgh')
  cursor(1, 4)
  assert_equal(4, col('.'))
  col([1, '$'])->assert_equal(9)
  assert_equal(0, col([10, '$']))

  assert_fails('col(true)', 'E1222:')

  v9.CheckDefAndScriptFailure(['col(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['col({a: 10})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['col(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['col(".", [])'], ['E1013: Argument 2: type mismatch, expected number but got list<any>', 'E1210: Number required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['col("")'], 'E1209: Invalid value for a line number')
  bw!
enddef

def Test_complete()
  v9.CheckDefAndScriptFailure(['complete("1", [])'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['complete(1, {})'], ['E1013: Argument 2: type mismatch, expected list<any> but got dict<any>', 'E1211: List required for argument 2'])
enddef

def Test_complete_add()
  v9.CheckDefAndScriptFailure(['complete_add([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1223: String or Dictionary required for argument 1'])
enddef

def Test_complete_info()
  v9.CheckDefAndScriptFailure(['complete_info("")'], ['E1013: Argument 1: type mismatch, expected list<string> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['complete_info({})'], ['E1013: Argument 1: type mismatch, expected list<string> but got dict<any>', 'E1211: List required for argument 1'])
  assert_equal({'pum_visible': 0, 'mode': '', 'selected': -1, 'items': []}, complete_info())
  assert_equal({'mode': '', 'items': []}, complete_info(['mode', 'items']))
enddef

def Test_confirm()
  if !has('dialog_con') && !has('dialog_gui')
    CheckFeature dialog_con
  endif

  assert_fails('confirm(true)', 'E1174:')
  assert_fails('confirm("yes", true)', 'E1174:')
  assert_fails('confirm("yes", "maybe", 2, true)', 'E1174:')
  v9.CheckDefAndScriptFailure(['confirm(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['confirm("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['confirm("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['confirm("a", "b", 3, 4)'], ['E1013: Argument 4: type mismatch, expected string but got number', 'E1174: String required for argument 4'])
enddef

def Test_copy_return_type()
  var l = copy([1, 2, 3])
  var res = 0
  for n in l
    res += n
  endfor
  res->assert_equal(6)

  var dl = deepcopy([1, 2, 3])
  res = 0
  for n in dl
    res += n
  endfor
  res->assert_equal(6)

  dl = deepcopy([1, 2, 3], true)

  # after a copy() the type can change, but not the item itself
  var nl: list<number> = [1, 2]
  assert_equal([1, 2, 'x'], nl->copy()->extend(['x']))

  var lines =<< trim END
      var nll: list<list<number>> = [[1, 2]]
      nll->copy()[0]->extend(['x'])
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected list<number> but got list<string> in extend()')

  var nd: dict<number> = {a: 1, b: 2}
  assert_equal({a: 1, b: 2, c: 'x'}, nd->copy()->extend({c: 'x'}))
  lines =<< trim END
      var ndd: dict<dict<number>> = {a: {x: 1, y: 2}}
      ndd->copy()['a']->extend({z: 'x'})
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected dict<number> but got dict<string> in extend()')

  # after a deepcopy() the item type can also change
  var nll: list<list<number>> = [[1, 2]]
  assert_equal([1, 2, 'x'], nll->deepcopy()[0]->extend(['x']))

  var ndd: dict<dict<number>> = {a: {x: 1, y: 2}}
  assert_equal({x: 1, y: 2, z: 'x'}, ndd->deepcopy()['a']->extend({z: 'x'}))

  var ldn: list<dict<number>> = [{a: 0}]->deepcopy()
  assert_equal([{a: 0}], ldn)
enddef

def Test_count()
  count('ABC ABC ABC', 'b', true)->assert_equal(3)
  count('ABC ABC ABC', 'b', false)->assert_equal(0)
  v9.CheckDefAndScriptFailure(['count(10, 1)'], 'E1225: String, List or Dictionary required for argument 1')
  v9.CheckDefAndScriptFailure(['count("a", [1], 2)'], ['E1013: Argument 3: type mismatch, expected bool but got number', 'E1212: Bool required for argument 3'])
  v9.CheckDefAndScriptFailure(['count("a", [1], 0, "b")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  count([1, 2, 2, 3], 2)->assert_equal(2)
  count([1, 2, 2, 3], 2, false, 2)->assert_equal(1)
  count({a: 1.1, b: 2.2, c: 1.1}, 1.1)->assert_equal(2)
enddef

def Test_cscope_connection()
  CheckFeature cscope
  assert_equal(0, cscope_connection())
  v9.CheckDefAndScriptFailure(['cscope_connection("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['cscope_connection(1, 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['cscope_connection(1, "b", 3)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
enddef

def Test_cursor()
  new
  setline(1, range(4))
  cursor(2, 1)
  assert_equal(2, getcurpos()[1])
  cursor('$', 1)
  assert_equal(4, getcurpos()[1])
  cursor([2, 1])
  assert_equal(2, getcurpos()[1])

  var lines =<< trim END
    cursor('2', 1)
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1209:')
  v9.CheckDefAndScriptFailure(['cursor(0z10, 1)'], ['E1013: Argument 1: type mismatch, expected number but got blob', 'E1224: String, Number or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['cursor(1, "2")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['cursor(1, 2, "3")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefExecAndScriptFailure(['cursor("", 2)'], 'E1209: Invalid value for a line number')
enddef

def Test_debugbreak()
  CheckMSWindows
  v9.CheckDefAndScriptFailure(['debugbreak("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_deepcopy()
  v9.CheckDefAndScriptFailure(['deepcopy({}, 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
enddef

def Test_delete()
  var res: bool = delete('doesnotexist')
  assert_equal(true, res)

  v9.CheckDefAndScriptFailure(['delete(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['delete("a", 10)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['delete("")'], 'E474: Invalid argument')
enddef

def Test_deletebufline()
  v9.CheckDefAndScriptFailure(['deletebufline([], 2)'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['deletebufline("a", [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['deletebufline("a", 2, 0z10)'], ['E1013: Argument 3: type mismatch, expected string but got blob', 'E1220: String or Number required for argument 3'])
  new
  setline(1, ['one', 'two'])
  deletebufline('', 1)
  getline(1, '$')->assert_equal(['two'])

  assert_fails('deletebufline("", "$a", "$b")', ['E1030: Using a String as a Number: "$a"', 'E1030: Using a String as a Number: "$a"'])
  assert_fails('deletebufline("", "$", "$b")', ['E1030: Using a String as a Number: "$b"', 'E1030: Using a String as a Number: "$b"'])

  bwipe!
enddef

def Test_diff_filler()
  v9.CheckDefAndScriptFailure(['diff_filler([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['diff_filler(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['diff_filler("")'], 'E1209: Invalid value for a line number')
  assert_equal(0, diff_filler(1))
  assert_equal(0, diff_filler('.'))
enddef

def Test_diff_hlID()
  v9.CheckDefAndScriptFailure(['diff_hlID(0z10, 1)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['diff_hlID(1, "a")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['diff_hlID("", 10)'], 'E1209: Invalid value for a line number')
enddef

def Test_digraph_get()
  v9.CheckDefAndScriptFailure(['digraph_get(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['digraph_get("")'], 'E1214: Digraph must be just two characters')
enddef

def Test_digraph_getlist()
  v9.CheckDefAndScriptFailure(['digraph_getlist(10)'], ['E1013: Argument 1: type mismatch, expected bool but got number', 'E1212: Bool required for argument 1'])
  v9.CheckDefAndScriptFailure(['digraph_getlist("")'], ['E1013: Argument 1: type mismatch, expected bool but got string', 'E1212: Bool required for argument 1'])
enddef

def Test_digraph_set()
  v9.CheckDefAndScriptFailure(['digraph_set(10, "a")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['digraph_set("ab", 0z10)'], ['E1013: Argument 2: type mismatch, expected string but got blob', 'E1174: String required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['digraph_set("", "a")'], 'E1214: Digraph must be just two characters')
enddef

def Test_digraph_setlist()
  v9.CheckDefAndScriptFailure(['digraph_setlist("a")'], ['E1013: Argument 1: type mismatch, expected list<string> but got string', 'E1216: digraph_setlist() argument must be a list of lists with two items'])
  v9.CheckDefAndScriptFailure(['digraph_setlist({})'], ['E1013: Argument 1: type mismatch, expected list<string> but got dict<any>', 'E1216: digraph_setlist() argument must be a list of lists with two items'])
enddef

def Test_echoraw()
  v9.CheckDefAndScriptFailure(['echoraw(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['echoraw(["x"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
enddef

def Test_escape()
  v9.CheckDefAndScriptFailure(['escape(10, " ")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['escape(true, false)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['escape("a", 10)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  assert_equal('a\:b', escape("a:b", ":"))
  escape('abc', '')->assert_equal('abc')
  escape('', ':')->assert_equal('')
enddef

def Test_eval()
  v9.CheckDefAndScriptFailure(['eval(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['eval(null)'], ['E1013: Argument 1: type mismatch, expected string but got special', 'E1174: String required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['eval("")'], 'E15: Invalid expression')
  assert_equal(2, eval('1 + 1'))
enddef

def Test_executable()
  assert_false(executable(""))
  assert_false(executable(test_null_string()))

  v9.CheckDefExecFailure(['echo executable(123)'], 'E1013:')
  v9.CheckDefExecFailure(['echo executable(true)'], 'E1013:')
enddef

def Test_execute()
  var res = execute("echo 'hello'")
  assert_equal("\nhello", res)
  res = execute(["echo 'here'", "echo 'there'"])
  assert_equal("\nhere\nthere", res)

  v9.CheckDefAndScriptFailure(['execute(123)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1222: String or List required for argument 1'])
  v9.CheckDefFailure(['execute([123])'], 'E1013: Argument 1: type mismatch, expected list<string> but got list<number>')
  v9.CheckDefExecFailure(['echo execute(["xx", 123])'], 'E492')
  v9.CheckDefAndScriptFailure(['execute("xx", 123)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
enddef

def Test_exepath()
  v9.CheckDefExecFailure(['echo exepath(true)'], 'E1013:')
  v9.CheckDefExecFailure(['echo exepath(v:null)'], 'E1013:')
  v9.CheckDefExecFailure(['echo exepath("")'], 'E1175:')
enddef

command DoSomeCommand let g:didSomeCommand = 4

def Test_exists()
  v9.CheckDefAndScriptFailure(['exists(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  call assert_equal(1, exists('&tabstop'))

  var lines =<< trim END
    if exists('+newoption')
      if &newoption == 'ok'
      endif
    endif
  END
  v9.CheckDefFailure(lines, 'E113:')
  v9.CheckScriptSuccess(lines)
enddef

def Test_exists_compiled()
  call assert_equal(1, exists_compiled('&tabstop'))
  v9.CheckDefAndScriptFailure(['exists_compiled(10)'], ['E1232:', 'E1233:'])
  v9.CheckDefAndScriptFailure(['exists_compiled(v:progname)'], ['E1232:', 'E1233:'])

  if exists_compiled('+newoption')
    if &newoption == 'ok'
    endif
  endif
  if exists_compiled('&newoption')
    if &newoption == 'ok'
    endif
  endif
  if exists_compiled('+tabstop')
    assert_equal(8, &tabstop)
  else
    assert_report('tabstop option not existing?')
  endif
  if exists_compiled('&tabstop')
    assert_equal(8, &tabstop)
  else
    assert_report('tabstop option not existing?')
  endif

  if exists_compiled(':DoSomeCommand') >= 2
    DoSomeCommand
  endif
  assert_equal(4, g:didSomeCommand)
  if exists_compiled(':NoSuchCommand') >= 2
    NoSuchCommand
  endif

  var found = false
  if exists_compiled('*CheckFeature')
    found = true
  endif
  assert_false(found)
  found = false
  if exists_compiled('*g:CheckFeature')
    found = true
  endif
  assert_true(found)

  if exists_compiled('*NoSuchFunction')
    NoSuchFunction()
  endif
  if exists_compiled('*no_such_function')
    no_such_function()
  endif
enddef

def Test_expand()
  split SomeFile
  expand('%', true, true)->assert_equal(['SomeFile'])
  close
  v9.CheckDefAndScriptFailure(['expand(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['expand("a", 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
  v9.CheckDefAndScriptFailure(['expand("a", true, 2)'], ['E1013: Argument 3: type mismatch, expected bool but got number', 'E1212: Bool required for argument 3'])
  expand('')->assert_equal('')

  var caught = false
  try
    echo expand("<sfile>")
  catch /E1245:/
    caught = true
  endtry
  assert_true(caught)
enddef

def Test_expandcmd()
  $FOO = "blue"
  assert_equal("blue sky", expandcmd("`=$FOO .. ' sky'`"))

  assert_equal("yes", expandcmd("`={a: 'yes'}['a']`"))
  expandcmd('')->assert_equal('')

  v9.CheckDefAndScriptFailure(['expandcmd([1])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['expandcmd("abc", [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_extend_arg_types()
  g:number_one = 1
  g:string_keep = 'keep'
  var lines =<< trim END
      assert_equal([1, 2, 3], extend([1, 2], [3]))
      assert_equal([3, 1, 2], extend([1, 2], [3], 0))
      assert_equal([1, 3, 2], extend([1, 2], [3], 1))
      assert_equal([1, 3, 2], extend([1, 2], [3], g:number_one))

      assert_equal({a: 1, b: 2, c: 3}, extend({a: 1, b: 2}, {c: 3}))
      assert_equal({a: 1, b: 4}, extend({a: 1, b: 2}, {b: 4}))
      assert_equal({a: 1, b: 2}, extend({a: 1, b: 2}, {b: 4}, 'keep'))
      assert_equal({a: 1, b: 2}, extend({a: 1, b: 2}, {b: 4}, g:string_keep))

      # mix of types is OK without a declaration

      var res: list<dict<any>>
      extend(res, mapnew([1, 2], (_, v) => ({})))
      assert_equal([{}, {}], res)

      var dany: dict<any> = {a: 0}
      dany->extend({b: 'x'})
      assert_equal({a: 0, b: 'x'}, dany)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      assert_equal([1, 2, "x"], extend([1, 2], ["x"]))
      assert_equal([1, "b", 1], extend([1], ["b", 1]))

      assert_equal({a: 1, b: "x"}, extend({a: 1}, {b: "x"}))
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(['extend("a", 1)'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E712: Argument of extend() must be a List or Dictionary'])
  v9.CheckDefAndScriptFailure(['extend([1, 2], 3)'], ['E1013: Argument 2: type mismatch, expected list<any> but got number', 'E712: Argument of extend() must be a List or Dictionary'])
  v9.CheckDefAndScriptFailure(['var ll = [1, 2]', 'extend(ll, ["x"])'], ['E1013: Argument 2: type mismatch, expected list<number> but got list<string>', 'E1013: Argument 2: type mismatch, expected list<number> but got list<string>'])
  v9.CheckDefFailure(['extend([1, 2], [3], "x")'], 'E1013: Argument 3: type mismatch, expected number but got string')

  v9.CheckDefFailure(['extend({a: 1}, 42)'], 'E1013: Argument 2: type mismatch, expected dict<any> but got number')
  v9.CheckDefFailure(['extend({a: 1}, {b: 2}, 1)'], 'E1013: Argument 3: type mismatch, expected string but got number')

  v9.CheckScriptFailure(['vim9script', 'var l = [1]', 'extend(l, ["b", 1])'], 'E1013: Argument 2: type mismatch, expected list<number> but got list<any> in extend()')
enddef

func g:ExtendDict(d)
  call extend(a:d, #{xx: 'x'})
endfunc

def Test_extend_dict_item_type()
  var lines =<< trim END
       var d: dict<number> = {a: 1}
       extend(d, {b: 2})
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
       var d: dict<number> = {a: 1}
       extend(d, {b: 'x'})
  END
  v9.CheckDefAndScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected dict<number> but got dict<string>', 2)

  lines =<< trim END
       var d: dict<number> = {a: 1}
       g:ExtendDict(d)
  END
  v9.CheckDefExecFailure(lines, 'E1012: Type mismatch; expected number but got string', 0)
  v9.CheckScriptFailure(['vim9script'] + lines, 'E1012:', 1)

  lines =<< trim END
       var d: dict<bool>
       extend(d, {b: 0})
  END
  v9.CheckDefAndScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected dict<bool> but got dict<number>', 2)
enddef

func g:ExtendList(l)
  call extend(a:l, ['x'])
endfunc

def Test_extend_list_item_type()
  var lines =<< trim END
       var l: list<number> = [1]
       extend(l, [2])
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
       var l: list<number> = [1]
       extend(l, ['x'])
  END
  v9.CheckDefAndScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected list<number> but got list<string>', 2)

  lines =<< trim END
       var l: list<number> = [1]
       g:ExtendList(l)
  END
  v9.CheckDefExecFailure(lines, 'E1012: Type mismatch; expected number but got string', 0)
  v9.CheckScriptFailure(['vim9script'] + lines, 'E1012:', 1)

  lines =<< trim END
       var l: list<bool>
       extend(l, [0])
  END
  v9.CheckDefAndScriptFailure(lines, 'E1013: Argument 2: type mismatch, expected list<bool> but got list<number>', 2)
enddef

def Test_extend_return_type()
  var l = extend([1, 2], [3])
  var res = 0
  for n in l
    res += n
  endfor
  res->assert_equal(6)
enddef

def Test_extend_with_error_function()
  var lines =<< trim END
      vim9script
      def F()
        {
          var m = 10
        }
        echo m
      enddef

      def Test()
        var d: dict<any> = {}
        d->extend({A: 10, Func: function('F', [])})
        d.Func()
      enddef

      Test()
  END
  v9.CheckScriptFailure(lines, 'E1001: Variable not found: m')
enddef

def Test_extend_const()
  var lines =<< trim END
      const l = [1, 2]
      extend(l, [3])
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const list<number>')

  lines =<< trim END
      const d = {a: 1, b: 2}
      extend(d, {c: 3})
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const dict<number>')

  lines =<< trim END
      final d = {a: 1, b: 2}
      extend(d, {c: 3})
      assert_equal({a: 1, b: 2, c: 3}, d)
  END
  v9.CheckDefSuccess(lines)

  # item in a for loop is final
  lines =<< trim END
      var l: list<dict<any>> = [{n: 1}]
      for item in l
        item->extend({x: 2})
      endfor
  END
  v9.CheckDefSuccess(lines)
enddef

def Test_extendnew()
  assert_equal([1, 2, 'a'], extendnew([1, 2], ['a']))
  assert_equal({one: 1, two: 'a'}, extendnew({one: 1}, {two: 'a'}))

  v9.CheckDefAndScriptFailure(['extendnew({a: 1}, 42)'], ['E1013: Argument 2: type mismatch, expected dict<number> but got number', 'E712: Argument of extendnew() must be a List or Dictionary'])
  v9.CheckDefAndScriptFailure(['extendnew({a: 1}, [42])'], ['E1013: Argument 2: type mismatch, expected dict<number> but got list<number>', 'E712: Argument of extendnew() must be a List or Dictionary'])
  v9.CheckDefAndScriptFailure(['extendnew([1, 2], "x")'], ['E1013: Argument 2: type mismatch, expected list<number> but got string', 'E712: Argument of extendnew() must be a List or Dictionary'])
  v9.CheckDefAndScriptFailure(['extendnew([1, 2], {x: 1})'], ['E1013: Argument 2: type mismatch, expected list<number> but got dict<number>', 'E712: Argument of extendnew() must be a List or Dictionary'])
enddef

def Test_feedkeys()
  v9.CheckDefAndScriptFailure(['feedkeys(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['feedkeys("x", 10)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['feedkeys([], {})'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1174: String required for argument 1'])
  g:TestVar = 1
  feedkeys(":g:TestVar = 789\n", 'xt')
  assert_equal(789, g:TestVar)
  unlet g:TestVar
enddef

def Test_filereadable()
  assert_false(filereadable(""))
  assert_false(filereadable(test_null_string()))

  v9.CheckDefExecFailure(['echo filereadable(123)'], 'E1013:')
  v9.CheckDefExecFailure(['echo filereadable(true)'], 'E1013:')
enddef

def Test_filewritable()
  assert_false(filewritable(""))
  assert_false(filewritable(test_null_string()))

  v9.CheckDefExecFailure(['echo filewritable(123)'], 'E1013:')
  v9.CheckDefExecFailure(['echo filewritable(true)'], 'E1013:')
enddef

def Test_finddir()
  mkdir('Xtestdir')
  finddir('Xtestdir', '**', -1)->assert_equal(['Xtestdir'])
  var lines =<< trim END
      var l: list<string> = finddir('nothing', '*;', -1)
  END
  v9.CheckDefAndScriptSuccess(lines)
  delete('Xtestdir', 'rf')

  v9.CheckDefAndScriptFailure(['finddir(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['finddir(v:null)'], ['E1013: Argument 1: type mismatch, expected string but got special', 'E1174: String required for argument 1'])
  v9.CheckDefExecFailure(['echo finddir("")'], 'E1175:')
  v9.CheckDefAndScriptFailure(['finddir("a", [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['finddir("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  finddir('abc', '')->assert_equal('')

  v9.CheckDefFailure(['var s: list<string> = finddir("foo")'], 'E1012: Type mismatch; expected list<string> but got string')
  v9.CheckDefFailure(['var s: list<string> = finddir("foo", "path")'], 'E1012: Type mismatch; expected list<string> but got string')
  # with third argument only runtime type checking
  v9.CheckDefCompileSuccess(['var s: list<string> = finddir("foo", "path", 1)'])
enddef

def Test_findfile()
  findfile('runtest.vim', '**', -1)->assert_equal(['runtest.vim'])
  var lines =<< trim END
      var l: list<string> = findfile('nothing', '*;', -1)
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefExecFailure(['findfile(true)'], 'E1013: Argument 1: type mismatch, expected string but got bool')
  v9.CheckDefExecFailure(['findfile(v:null)'], 'E1013: Argument 1: type mismatch, expected string but got special')
  v9.CheckDefExecFailure(['findfile("")'], 'E1175:')
  v9.CheckDefAndScriptFailure(['findfile("a", [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['findfile("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  findfile('abc', '')->assert_equal('')
enddef

def Test_flatten()
  var lines =<< trim END
      echo flatten([1, 2, 3])
  END
  v9.CheckDefAndScriptFailure(lines, 'E1158:')
enddef

def Test_flattennew()
  var lines =<< trim END
      var l = [1, [2, [3, 4]], 5]
      call assert_equal([1, 2, 3, 4, 5], flattennew(l))
      call assert_equal([1, [2, [3, 4]], 5], l)

      call assert_equal([1, 2, [3, 4], 5], flattennew(l, 1))
      call assert_equal([1, [2, [3, 4]], 5], l)

      var ll: list<list<string>> = [['a', 'b', 'c']]
      assert_equal(['a', 'b', 'c'], ll->flattennew())
  END
  v9.CheckDefAndScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(['flattennew({})'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<any>', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['flattennew([], "1")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

" Test for float functions argument type
def Test_float_funcs_args()
  # acos()
  v9.CheckDefAndScriptFailure(['acos("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('1.570796', string(acos(0.0)))
  # asin()
  v9.CheckDefAndScriptFailure(['asin("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('0.0', string(asin(0.0)))
  # atan()
  v9.CheckDefAndScriptFailure(['atan("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('0.0', string(atan(0.0)))
  # atan2()
  v9.CheckDefAndScriptFailure(['atan2("a", 1.1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('-2.356194', string(atan2(-1, -1)))
  v9.CheckDefAndScriptFailure(['atan2(1.2, "a")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['atan2(1.2)'], ['E119:', 'E119:'])
  # ceil()
  v9.CheckDefAndScriptFailure(['ceil("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('2.0', string(ceil(2.0)))
  # cos()
  v9.CheckDefAndScriptFailure(['cos("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('1.0', string(cos(0.0)))
  # cosh()
  v9.CheckDefAndScriptFailure(['cosh("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('1.0', string(cosh(0.0)))
  # exp()
  v9.CheckDefAndScriptFailure(['exp("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('1.0', string(exp(0.0)))
  # float2nr()
  v9.CheckDefAndScriptFailure(['float2nr("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal(1, float2nr(1.234))
  # floor()
  v9.CheckDefAndScriptFailure(['floor("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('2.0', string(floor(2.0)))
  # fmod()
  v9.CheckDefAndScriptFailure(['fmod(1.1, "a")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['fmod("a", 1.1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['fmod(1.1)'], ['E119:', 'E119:'])
  assert_equal('0.13', string(fmod(12.33, 1.22)))
  # isinf()
  v9.CheckDefAndScriptFailure(['isinf("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal(1, isinf(1.0 / 0.0))
  # isnan()
  v9.CheckDefAndScriptFailure(['isnan("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_true(isnan(0.0 / 0.0))
  # log()
  v9.CheckDefAndScriptFailure(['log("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('0.0', string(log(1.0)))
  # log10()
  v9.CheckDefAndScriptFailure(['log10("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('0.0', string(log10(1.0)))
  # pow()
  v9.CheckDefAndScriptFailure(['pow("a", 1.1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['pow(1.1, "a")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['pow(1.1)'], ['E119:', 'E119:'])
  assert_equal('1.0', string(pow(0.0, 0.0)))
  # round()
  v9.CheckDefAndScriptFailure(['round("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('2.0', string(round(2.1)))
  # sin()
  v9.CheckDefAndScriptFailure(['sin("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('0.0', string(sin(0.0)))
  # sinh()
  v9.CheckDefAndScriptFailure(['sinh("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('0.0', string(sinh(0.0)))
  # sqrt()
  v9.CheckDefAndScriptFailure(['sqrt("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('0.0', string(sqrt(0.0)))
  # tan()
  v9.CheckDefAndScriptFailure(['tan("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('0.0', string(tan(0.0)))
  # tanh()
  v9.CheckDefAndScriptFailure(['tanh("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('0.0', string(tanh(0.0)))
  # trunc()
  v9.CheckDefAndScriptFailure(['trunc("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1219: Float or Number required for argument 1'])
  assert_equal('2.0', string(trunc(2.1)))
enddef

def Test_fnameescape()
  v9.CheckDefAndScriptFailure(['fnameescape(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal('\+a\%b\|', fnameescape('+a%b|'))
  fnameescape('')->assert_equal('')
enddef

def Test_fnamemodify()
  v9.CheckDefSuccess(['echo fnamemodify(test_null_string(), ":p")'])
  v9.CheckDefSuccess(['echo fnamemodify("", ":p")'])
  v9.CheckDefSuccess(['echo fnamemodify("file", test_null_string())'])
  v9.CheckDefSuccess(['echo fnamemodify("file", "")'])

  v9.CheckDefExecFailure(['echo fnamemodify(true, ":p")'], 'E1013: Argument 1: type mismatch, expected string but got bool')
  v9.CheckDefExecFailure(['echo fnamemodify(v:null, ":p")'], 'E1013: Argument 1: type mismatch, expected string but got special')
  v9.CheckDefExecFailure(['echo fnamemodify("file", true)'],  'E1013: Argument 2: type mismatch, expected string but got bool')
enddef

def Wrong_dict_key_type(items: list<number>): list<number>
  return filter(items, (_, val) => get({[val]: 1}, 'x'))
enddef

def Test_filter()
  assert_equal([], filter([1, 2, 3], '0'))
  assert_equal([1, 2, 3], filter([1, 2, 3], '1'))
  assert_equal({b: 20}, filter({a: 10, b: 20}, 'v:val == 20'))

  def GetFiltered(): list<number>
    var Odd: func = (_, v) => v % 2
    return range(3)->filter(Odd)
  enddef
  assert_equal([1], GetFiltered())

  var lines =<< trim END
      vim9script
      def Func(): list<string>
        var MatchWord: func: bool = (_, v) => true
        var l = ['xxx']
        return l->filter(MatchWord)
      enddef
      assert_equal(['xxx'], Func())
  END
  v9.CheckScriptSuccess(lines)

  v9.CheckDefAndScriptFailure(['filter(1.1, "1")'], ['E1013: Argument 1: type mismatch, expected list<any> but got float', 'E1251: List, Dictionary, Blob or String required for argument 1'])
  v9.CheckDefAndScriptFailure(['filter([1, 2], 4)'], ['E1256: String or function required for argument 2', 'E1024: Using a Number as a String'])

  lines =<< trim END
    def F(i: number, v: any): string
      return 'bad'
    enddef
    echo filter([1, 2, 3], F)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?any): bool but got func(number, any): string', 'E1135: Using a String as a Bool:'])

  # check first function argument type
  lines =<< trim END
    var l = [1, 2, 3]
    filter(l, (i: string, v: number) => true)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?number): bool but got func(string, number): bool', 'E1013: Argument 1: type mismatch, expected string but got number'])
  lines =<< trim END
    var d = {a: 1}
    filter(d, (i: number, v: number) => true)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?string, ?number): bool but got func(number, number): bool', 'E1013: Argument 1: type mismatch, expected number but got string'])
  lines =<< trim END
    var b = 0z1122
    filter(b, (i: string, v: number) => true)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?number): bool but got func(string, number): bool', 'E1013: Argument 1: type mismatch, expected string but got number'])
  lines =<< trim END
    var s = 'text'
    filter(s, (i: string, v: string) => true)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?string): bool but got func(string, string): bool', 'E1013: Argument 1: type mismatch, expected string but got number'])

  # check second function argument type
  lines =<< trim END
    var l = [1, 2, 3]
    filter(l, (i: number, v: string) => true)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?number): bool but got func(number, string): bool', 'E1013: Argument 2: type mismatch, expected string but got number'])
  lines =<< trim END
    var d = {a: 1}
    filter(d, (i: string, v: string) => true)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?string, ?number): bool but got func(string, string): bool', 'E1013: Argument 2: type mismatch, expected string but got number'])
  lines =<< trim END
    var b = 0z1122
    filter(b, (i: number, v: string) => true)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?number): bool but got func(number, string): bool', 'E1013: Argument 2: type mismatch, expected string but got number'])
  lines =<< trim END
    var s = 'text'
    filter(s, (i: number, v: number) => true)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?string): bool but got func(number, number): bool', 'E1013: Argument 2: type mismatch, expected number but got string'])
enddef

def Test_filter_wrong_dict_key_type()
  assert_fails('g:Wrong_dict_key_type([1, v:null, 3])', 'E1013:')
enddef

def Test_filter_return_type()
  var l = filter([1, 2, 3], (_, _) => 1)
  var res = 0
  for n in l
    res += n
  endfor
  res->assert_equal(6)
enddef

def Test_filter_missing_argument()
  var dict = {aa: [1], ab: [2], ac: [3], de: [4]}
  var res = dict->filter((k, _) => k =~ 'a' && k !~ 'b')
  res->assert_equal({aa: [1], ac: [3]})
enddef

def Test_filter_const()
  var lines =<< trim END
      const l = [1, 2, 3]
      filter(l, 'v:val == 2')
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const list<number>')

  lines =<< trim END
      const d = {a: 1, b: 2}
      filter(d, 'v:val == 2')
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const dict<number>')
enddef

def Test_foldclosed()
  v9.CheckDefAndScriptFailure(['foldclosed(function("min"))'], ['E1013: Argument 1: type mismatch, expected string but got func(...): unknown', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['foldclosed("")'], 'E1209: Invalid value for a line number')
  assert_equal(-1, foldclosed(1))
  assert_equal(-1, foldclosed('$'))
enddef

def Test_foldclosedend()
  v9.CheckDefAndScriptFailure(['foldclosedend(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['foldclosedend("")'], 'E1209: Invalid value for a line number')
  assert_equal(-1, foldclosedend(1))
  assert_equal(-1, foldclosedend('w0'))
enddef

def Test_foldlevel()
  v9.CheckDefAndScriptFailure(['foldlevel(0z10)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['foldlevel("")'], 'E1209: Invalid value for a line number')
  assert_equal(0, foldlevel(1))
  assert_equal(0, foldlevel('.'))
enddef

def Test_foldtextresult()
  v9.CheckDefAndScriptFailure(['foldtextresult(1.1)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['foldtextresult("")'], 'E1209: Invalid value for a line number')
  assert_equal('', foldtextresult(1))
  assert_equal('', foldtextresult('.'))
enddef

def Test_fullcommand()
  assert_equal('next', fullcommand('n'))
  assert_equal('noremap', fullcommand('no'))
  assert_equal('noremap', fullcommand('nor'))
  assert_equal('normal', fullcommand('norm'))

  assert_equal('', fullcommand('k'))
  assert_equal('keepmarks', fullcommand('ke'))
  assert_equal('keepmarks', fullcommand('kee'))
  assert_equal('keepmarks', fullcommand('keep'))
  assert_equal('keepjumps', fullcommand('keepj'))

  assert_equal('dlist', fullcommand('dl'))
  assert_equal('', fullcommand('dp'))
  assert_equal('delete', fullcommand('del'))
  assert_equal('', fullcommand('dell'))
  assert_equal('', fullcommand('delp'))

  assert_equal('srewind', fullcommand('sre'))
  assert_equal('scriptnames', fullcommand('scr'))
  assert_equal('', fullcommand('scg'))
  fullcommand('')->assert_equal('')

  assert_equal('', fullcommand('en'))
  legacy call assert_equal('endif', fullcommand('en'))
  assert_equal('endif', fullcommand('en', 0))
  legacy call assert_equal('endif', fullcommand('en', 0))
  assert_equal('', fullcommand('en', 1))
  legacy call assert_equal('', fullcommand('en', 1))
enddef

def Test_funcref()
  v9.CheckDefAndScriptFailure(['funcref("reverse", 2)'], ['E1013: Argument 2: type mismatch, expected list<any> but got number', 'E1211: List required for argument 2'])
  v9.CheckDefAndScriptFailure(['funcref("reverse", [2], [1])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])

  var lines =<< trim END
      vim9script
      def UseBool(b: bool)
      enddef
      def GetRefOk()
        var Ref1: func(bool) = funcref(UseBool)
        var Ref2: func(bool) = funcref('UseBool')
      enddef
      def GetRefBad()
        # only fails at runtime
        var Ref1: func(number) = funcref(UseBool)
      enddef
      defcompile
      GetRefOk()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def UseBool(b: bool)
      enddef
      def GetRefBad()
        # only fails at runtime
        var Ref1: func(number) = funcref(UseBool)
      enddef
      GetRefBad()
  END
  v9.CheckScriptFailure(lines, 'E1012: Type mismatch; expected func(number) but got func(bool)')
enddef

def Test_function()
  v9.CheckDefExecAndScriptFailure(['function(123)'], 'E1256: String or function required for argument 1')

  v9.CheckDefAndScriptFailure(['function("reverse", 2)'], ['E1013: Argument 2: type mismatch, expected list<any> but got number', 'E1211: List required for argument 2'])
  v9.CheckDefAndScriptFailure(['function("reverse", [2], [1])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])

  var lines =<< trim END
      vim9script
      def UseBool(b: bool)
      enddef
      def GetRefOk()
        var Ref1: func(bool) = function(UseBool)
        var Ref2: func(bool) = function('UseBool')
      enddef
      def GetRefBad()
        # only fails at runtime
        var Ref1: func(number) = function(UseBool)
      enddef
      defcompile
      GetRefOk()
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def UseBool(b: bool)
      enddef
      def GetRefBad()
        # only fails at runtime
        var Ref1: func(number) = function(UseBool)
      enddef
      GetRefBad()
  END
  v9.CheckScriptFailure(lines, 'E1012: Type mismatch; expected func(number) but got func(bool)')
enddef

def Test_garbagecollect()
  garbagecollect(true)
  v9.CheckDefAndScriptFailure(['garbagecollect("1")'], ['E1013: Argument 1: type mismatch, expected bool but got string', 'E1212: Bool required for argument 1'])
  v9.CheckDefAndScriptFailure(['garbagecollect(20)'], ['E1013: Argument 1: type mismatch, expected bool but got number', 'E1212: Bool required for argument 1'])
enddef

def Test_get()
  v9.CheckDefAndScriptFailure(['get("a", 1)'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E896: Argument of get() must be a List, Dictionary or Blob'])
  [3, 5, 2]->get(1)->assert_equal(5)
  [3, 5, 2]->get(3)->assert_equal(0)
  [3, 5, 2]->get(3, 9)->assert_equal(9)
  assert_equal(get(0z102030, 2), 0x30)
  {a: 7, b: 11, c: 13}->get('c')->assert_equal(13)
  {10: 'a', 20: 'b', 30: 'd'}->get(20)->assert_equal('b')
  function('max')->get('name')->assert_equal('max')
  var F: func = function('min', [[5, 8, 6]])
  F->get('name')->assert_equal('min')
  F->get('args')->assert_equal([[5, 8, 6]])

  var lines =<< trim END
      vim9script
      def DoThat(): number
        var Getqflist: func = function('getqflist', [{id: 42}])
        return Getqflist()->get('id', 77)
      enddef
      assert_equal(0, DoThat())
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_getbufinfo()
  var bufinfo = getbufinfo(bufnr())
  getbufinfo('%')->assert_equal(bufinfo)

  edit Xtestfile1
  hide edit Xtestfile2
  hide enew
  getbufinfo({bufloaded: true, buflisted: true, bufmodified: false})
      ->len()->assert_equal(3)
  bwipe Xtestfile1 Xtestfile2
  v9.CheckDefAndScriptFailure(['getbufinfo(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1174: String required for argument 1'])
enddef

def Test_getbufline()
  e SomeFile
  var buf = bufnr()
  sp Otherfile
  var lines = ['aaa', 'bbb', 'ccc']
  setbufline(buf, 1, lines)
  getbufline('#', 1, '$')->assert_equal(lines)
  getbufline(-1, '$', '$')->assert_equal([])
  getbufline(-1, 1, '$')->assert_equal([])

  getbufoneline('#', 1)->assert_equal(lines[0])

  assert_equal([7, 7, 7], getbufline('#', 1, '$')->map((_, _) => 7))

  assert_fails('getbufline("", "$a", "$b")', ['E1030: Using a String as a Number: "$a"', 'E1030: Using a String as a Number: "$a"'])
  assert_fails('getbufline("", "$", "$b")', ['E1030: Using a String as a Number: "$b"', 'E1030: Using a String as a Number: "$b"'])
  bwipe!

  assert_fails('getbufoneline("", "$a")', ['E1030: Using a String as a Number: "$a"', 'E1030: Using a String as a Number: "$a"'])
  bwipe!

  v9.CheckDefAndScriptFailure(['getbufline([], 2)'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getbufline("a", [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['getbufline("a", 2, 0z10)'], ['E1013: Argument 3: type mismatch, expected string but got blob', 'E1220: String or Number required for argument 3'])

  v9.CheckDefAndScriptFailure(['getbufoneline([], 2)'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getbufoneline("a", [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 2'])
enddef

def Test_getbufvar()
  v9.CheckDefAndScriptFailure(['getbufvar(true, "v")'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getbufvar(1, 2, 3)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
enddef

def Test_getchangelist()
  new
  setline(1, 'some text')
  var changelist = bufnr()->getchangelist()
  getchangelist('%')->assert_equal(changelist)
  bwipe!
enddef

def Test_getchar()
  while getchar(0)
  endwhile
  getchar(true)->assert_equal(0)
  getchar(1)->assert_equal(0)
  v9.CheckDefAndScriptFailure(['getchar(2)'], ['E1013: Argument 1: type mismatch, expected bool but got number', 'E1212: Bool required for argument 1'])
  v9.CheckDefAndScriptFailure(['getchar("1")'], ['E1013: Argument 1: type mismatch, expected bool but got string', 'E1212: Bool required for argument 1'])
enddef

def Test_getcharpos()
  assert_equal(['x', 'x', 'x', 'x'], getcharpos('.')->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['getcharpos(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['getcharpos(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['getcharpos("")'], 'E1209: Invalid value for a line number')
enddef

def Test_getcharstr()
  v9.CheckDefAndScriptFailure(['getcharstr(2)'], ['E1013: Argument 1: type mismatch, expected bool but got number', 'E1212: Bool required for argument 1'])
  v9.CheckDefAndScriptFailure(['getcharstr("1")'], ['E1013: Argument 1: type mismatch, expected bool but got string', 'E1212: Bool required for argument 1'])
enddef

def Test_getcompletion()
  set wildignore=*.vim,*~
  var l = getcompletion('run', 'file', true)
  l->assert_equal([])
  set wildignore&
  v9.CheckDefAndScriptFailure(['getcompletion(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['getcompletion("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['getcompletion("a", "b", 2)'], ['E1013: Argument 3: type mismatch, expected bool but got number', 'E1212: Bool required for argument 3'])
  v9.CheckDefExecAndScriptFailure(['getcompletion("a", "")'], 'E475: Invalid argument')
  getcompletion('', 'messages')->assert_equal(['clear'])
enddef

def Test_getcurpos()
  assert_equal(['x', 'x', 'x', 'x', 'x'], getcurpos()->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['getcurpos("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_getcursorcharpos()
  assert_equal(['x', 'x', 'x', 'x', 'x'], getcursorcharpos()->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['getcursorcharpos("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_getcwd()
  v9.CheckDefAndScriptFailure(['getcwd("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getcwd("x", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getcwd(1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_getenv()
  if getenv('does-not_exist') == ''
    assert_report('getenv() should return null')
  endif
  if getenv('does-not_exist') == null
  else
    assert_report('getenv() should return null')
  endif
  $SOMEENVVAR = 'some'
  assert_equal('some', getenv('SOMEENVVAR'))
  assert_notequal(null, getenv('SOMEENVVAR'))
  unlet $SOMEENVVAR
  getenv('')->assert_equal(v:null)
enddef

def Test_getfontname()
  v9.CheckDefAndScriptFailure(['getfontname(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  #getfontname('')->assert_equal('')
enddef

def Test_getfperm()
  assert_equal('', getfperm(""))
  assert_equal('', getfperm(test_null_string()))

  v9.CheckDefExecFailure(['echo getfperm(true)'], 'E1013:')
  v9.CheckDefExecFailure(['echo getfperm(v:null)'], 'E1013:')
enddef

def Test_getfsize()
  assert_equal(-1, getfsize(""))
  assert_equal(-1, getfsize(test_null_string()))

  v9.CheckDefExecFailure(['echo getfsize(true)'], 'E1013:')
  v9.CheckDefExecFailure(['echo getfsize(v:null)'], 'E1013:')
enddef

def Test_getftime()
  assert_equal(-1, getftime(""))
  assert_equal(-1, getftime(test_null_string()))

  v9.CheckDefExecFailure(['echo getftime(true)'], 'E1013:')
  v9.CheckDefExecFailure(['echo getftime(v:null)'], 'E1013:')
enddef

def Test_getftype()
  assert_equal('', getftype(""))
  assert_equal('', getftype(test_null_string()))

  v9.CheckDefExecFailure(['echo getftype(true)'], 'E1013:')
  v9.CheckDefExecFailure(['echo getftype(v:null)'], 'E1013:')
enddef

def Test_getjumplist()
  v9.CheckDefAndScriptFailure(['getjumplist("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getjumplist("x", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getjumplist(1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_getline()
  var lines =<< trim END
      new
      setline(1, ['hello', 'there', 'again'])
      assert_equal('hello', getline(1))
      assert_equal('hello', getline('.'))

      normal 2Gvjv
      assert_equal('there', getline("'<"))
      assert_equal('again', getline("'>"))

      assert_equal([3, 3, 3], getline(1, 3)->map((_, _) => 3))
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      echo getline('1')
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1209:')
  v9.CheckDefAndScriptFailure(['getline(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getline(1, true)'], ['E1013: Argument 2: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['getline("")'], 'E1209: Invalid value for a line number')
enddef

def Test_getloclist()
  v9.CheckDefAndScriptFailure(['getloclist("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getloclist(1, [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_getloclist_return_type()
  var l = getloclist(1)
  l->assert_equal([])

  var d = getloclist(1, {items: 0})
  d->assert_equal({items: []})
enddef

def Test_getmarklist()
  v9.CheckDefAndScriptFailure(['getmarklist([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  assert_equal([], getmarklist(10000))
  assert_fails('getmarklist("a%b@#")', 'E94:')
enddef

def Test_getmatches()
  v9.CheckDefAndScriptFailure(['getmatches("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_getpos()
  assert_equal(['x', 'x', 'x', 'x'], getpos('.')->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['getpos(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal([0, 1, 1, 0], getpos('.'))
  v9.CheckDefExecFailure(['getpos("a")'], 'E1209:')
  v9.CheckDefExecAndScriptFailure(['getpos("")'], 'E1209: Invalid value for a line number')
enddef

def Test_getqflist()
  v9.CheckDefAndScriptFailure(['getqflist([])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 1'])
  call assert_equal({}, getqflist({}))
enddef

def Test_getqflist_return_type()
  var l = getqflist()
  l->assert_equal([])

  var d = getqflist({items: 0})
  d->assert_equal({items: []})
enddef

def Test_getreg()
  var lines = ['aaa', 'bbb', 'ccc']
  setreg('a', lines)
  getreg('a', true, true)->assert_equal(lines)
  assert_equal([7, 7, 7], getreg('a', true, true)->map((_, _) => 7))

  assert_fails('getreg("ab")', 'E1162:')
  v9.CheckDefAndScriptFailure(['getreg(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['getreg(".", 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
  v9.CheckDefAndScriptFailure(['getreg(".", 1, "b")'], ['E1013: Argument 3: type mismatch, expected bool but got string', 'E1212: Bool required for argument 3'])
  @" = 'A1B2C3'
  getreg('')->assert_equal('A1B2C3')
enddef

def Test_getreg_return_type()
  var s1: string = getreg('"')
  var s2: string = getreg('"', 1)
  var s3: list<string> = getreg('"', 1, 1)
enddef

def Test_getreginfo()
  var text = 'abc'
  setreg('a', text)
  getreginfo('a')->assert_equal({regcontents: [text], regtype: 'v', isunnamed: false})
  assert_fails('getreginfo("ab")', 'E1162:')
  @" = 'D1E2F3'
  getreginfo('').regcontents->assert_equal(['D1E2F3'])
enddef

def Test_getregtype()
  var lines = ['aaa', 'bbb', 'ccc']
  setreg('a', lines)
  getregtype('a')->assert_equal('V')
  assert_fails('getregtype("ab")', 'E1162:')
  setreg('"', 'ABCD', 'b')
  getregtype('')->assert_equal("\<C-V>4")
enddef

def Test_getscriptinfo()
  v9.CheckDefAndScriptFailure(['getscriptinfo("x")'], ['E1013: Argument 1: type mismatch, expected dict<any> but got string', 'E1206: Dictionary required for argument 1'])

  var lines1 =<< trim END
    vim9script
    g:loaded_script_id = expand("<SID>")
    var XscriptVar = [1, {v: 2}]
    func XgetScriptVar()
      return XscriptVar
    endfunc
    func Xscript_legacy_func1()
    endfunc
    def Xscript_def_func1()
    enddef
    func g:Xscript_legacy_func2()
    endfunc
    def g:Xscript_def_func2()
    enddef
  END
  writefile(lines1, 'X22script92', 'D')

  var lines2 =<< trim END
    source X22script92
    var sid = matchstr(g:loaded_script_id, '<SNR>\zs\d\+\ze_')->str2nr()

    var l = getscriptinfo({sid: sid, name: 'ignored'})
    assert_match('X22script92$', l[0].name)
    assert_equal(g:loaded_script_id, $"<SNR>{l[0].sid}_")
    assert_equal(999999, l[0].version)
    assert_equal(0, l[0].sourced)
    assert_equal({XscriptVar: [1, {v: 2}]}, l[0].variables)
    var funcs = ['Xscript_legacy_func2',
          $"<SNR>{sid}_Xscript_legacy_func1",
          $"<SNR>{sid}_Xscript_def_func1",
          'Xscript_def_func2',
          $"<SNR>{sid}_XgetScriptVar"]
    for f in funcs
      assert_true(index(l[0].functions, f) != -1)
    endfor
  END
  v9.CheckDefAndScriptSuccess(lines2)
enddef

def Test_gettabinfo()
  v9.CheckDefAndScriptFailure(['gettabinfo("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_gettabvar()
  v9.CheckDefAndScriptFailure(['gettabvar("a", "b")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['gettabvar(1, 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
enddef

def Test_gettabwinvar()
  v9.CheckDefAndScriptFailure(['gettabwinvar("a", 2, "c")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['gettabwinvar(1, "b", "c", [])'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['gettabwinvar(1, 1, 3, {})'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
enddef

def Test_gettagstack()
  v9.CheckDefAndScriptFailure(['gettagstack("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_gettext()
  v9.CheckDefAndScriptFailure(['gettext(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['gettext("")'], 'E1175: Non-empty string required for argument 1')
  assert_equal('abc', gettext("abc"))
  assert_fails('gettext("")', 'E1175:')
enddef

def Test_getwininfo()
  v9.CheckDefAndScriptFailure(['getwininfo("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_getwinpos()
  assert_equal(['x', 'x'], getwinpos()->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['getwinpos("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_getwinvar()
  v9.CheckDefAndScriptFailure(['getwinvar("a", "b")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['getwinvar(1, 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
enddef

def Test_glob()
  glob('runtest.vim', true, true, true)->assert_equal(['runtest.vim'])
  v9.CheckDefAndScriptFailure(['glob(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['glob("a", 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
  v9.CheckDefAndScriptFailure(['glob("a", 1, "b")'], ['E1013: Argument 3: type mismatch, expected bool but got string', 'E1212: Bool required for argument 3'])
  v9.CheckDefAndScriptFailure(['glob("a", 1, true, 2)'], ['E1013: Argument 4: type mismatch, expected bool but got number', 'E1212: Bool required for argument 4'])
  glob('')->assert_equal('')
enddef

def Test_glob2regpat()
  v9.CheckDefAndScriptFailure(['glob2regpat(null)'], ['E1013: Argument 1: type mismatch, expected string but got special', 'E1174: String required for argument 1'])
  glob2regpat('')->assert_equal('^$')
enddef

def Test_globpath()
  globpath('.', 'runtest.vim', true, true, true)->assert_equal(['./runtest.vim'])
  v9.CheckDefAndScriptFailure(['globpath(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['globpath("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['globpath("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected bool but got string', 'E1212: Bool required for argument 3'])
  v9.CheckDefAndScriptFailure(['globpath("a", "b", true, "d")'], ['E1013: Argument 4: type mismatch, expected bool but got string', 'E1212: Bool required for argument 4'])
  v9.CheckDefAndScriptFailure(['globpath("a", "b", true, false, "e")'], ['E1013: Argument 5: type mismatch, expected bool but got string', 'E1212: Bool required for argument 5'])
  globpath('', '')->assert_equal('')
enddef

def Test_has()
  has('eval', true)->assert_equal(1)
  v9.CheckDefAndScriptFailure(['has(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['has("a", "b")'], ['E1013: Argument 2: type mismatch, expected bool but got string', 'E1212: Bool required for argument 2'])
  has('')->assert_equal(0)
enddef

def Test_has_key()
  var d = {123: 'xx'}
  assert_true(has_key(d, '123'))
  assert_true(has_key(d, 123))
  assert_false(has_key(d, 'x'))
  assert_false(has_key(d, 99))

  v9.CheckDefAndScriptFailure(['has_key([1, 2], "k")'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 1'])
  v9.CheckDefAndScriptFailure(['has_key({"a": 10}, ["a"])'], ['E1013: Argument 2: type mismatch, expected string but got list<string>', 'E1220: String or Number required for argument 2'])
enddef

def Test_haslocaldir()
  v9.CheckDefAndScriptFailure(['haslocaldir("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['haslocaldir("x", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['haslocaldir(1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_hasmapto()
  hasmapto('foobar', 'i', true)->assert_equal(0)
  iabbrev foo foobar
  hasmapto('foobar', 'i', true)->assert_equal(1)
  iunabbrev foo
  v9.CheckDefAndScriptFailure(['hasmapto(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['hasmapto("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['hasmapto("a", "b", 2)'], ['E1013: Argument 3: type mismatch, expected bool but got number', 'E1212: Bool required for argument 3'])
  hasmapto('', '')->assert_equal(0)
enddef

def Test_histadd()
  v9.CheckDefAndScriptFailure(['histadd(1, "x")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['histadd(":", 10)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  histadd("search", 'skyblue')
  assert_equal('skyblue', histget('/', -1))
  histadd("search", '')->assert_equal(0)
enddef

def Test_histdel()
  v9.CheckDefAndScriptFailure(['histdel(1, "x")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['histdel(":", true)'], ['E1013: Argument 2: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 2'])
  histdel('search', '')->assert_equal(0)
enddef

def Test_histget()
  v9.CheckDefAndScriptFailure(['histget(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['histget("a", "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_histnr()
  v9.CheckDefAndScriptFailure(['histnr(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal(-1, histnr('abc'))
enddef

def Test_hlID()
  v9.CheckDefAndScriptFailure(['hlID(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal(0, hlID('NonExistingHighlight'))
  hlID('')->assert_equal(0)
enddef

def Test_hlexists()
  v9.CheckDefAndScriptFailure(['hlexists([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1174: String required for argument 1'])
  assert_equal(0, hlexists('NonExistingHighlight'))
  hlexists('')->assert_equal(0)
enddef

def Test_hlget()
  v9.CheckDefAndScriptFailure(['hlget([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1174: String required for argument 1'])
  hlget('')->assert_equal([])
enddef

def Test_hlset()
  v9.CheckDefAndScriptFailure(['hlset("id")'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E1211: List required for argument 1'])
  hlset([])->assert_equal(0)
enddef

def Test_iconv()
  v9.CheckDefAndScriptFailure(['iconv(1, "from", "to")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['iconv("abc", 10, "to")'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['iconv("abc", "from", 20)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  assert_equal('abc', iconv('abc', 'fromenc', 'toenc'))
  iconv('', '', '')->assert_equal('')
enddef

def Test_indent()
  v9.CheckDefAndScriptFailure(['indent([1])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['indent(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['indent("")'], 'E1209: Invalid value for a line number')
  v9.CheckDefExecAndScriptFailure(['indent(-1)'], 'E966: Invalid line number: -1')
  assert_equal(0, indent(1))
enddef

def Test_index()
  index(['a', 'b', 'a', 'B'], 'b', 2, true)->assert_equal(3)
  v9.CheckDefAndScriptFailure(['index("a", "a")'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E1226: List or Blob required for argument 1'])
  v9.CheckDefFailure(['index(["1"], 1)'], 'E1013: Argument 2: type mismatch, expected string but got number')
  v9.CheckDefAndScriptFailure(['index(0z10, "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['index([1], 1, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['index(0z1020, 10, 1, 2)'], ['E1013: Argument 4: type mismatch, expected bool but got number', 'E1212: Bool required for argument 4'])
enddef

def Test_indexof()
  var l = [{color: 'red'}, {color: 'blue'}, {color: 'green'}, {color: 'blue'}]
  indexof(l, (i, v) => v.color == 'blue')->assert_equal(1)
  indexof(l, (i, v) => v.color == 'blue', {startidx: 1})->assert_equal(1)
  indexof(l, (i, v) => v.color == 'blue', {startidx: 2})->assert_equal(3)
  var b = 0zdeadbeef
  indexof(b, "v:val == 0xef")->assert_equal(3)

  def TestIdx1(k: number, v: dict<any>): bool
    return v.color == 'blue'
  enddef
  indexof(l, TestIdx1)->assert_equal(1)

  var lines =<< trim END
    def TestIdx(v: dict<any>): bool
      return v.color == 'blue'
    enddef

    indexof([{color: "red"}], TestIdx)
  END
  v9.CheckDefAndScriptFailure(lines, ['E176: Invalid number of arguments', 'E118: Too many arguments for function'])

  lines =<< trim END
    def TestIdx(k: number, v: dict<any>)
    enddef

    indexof([{color: "red"}], TestIdx)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?any): bool', 'E1031: Cannot use void value'])

  lines =<< trim END
    def TestIdx(k: number, v: dict<any>): string
      return "abc"
    enddef

    indexof([{color: "red"}], TestIdx)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?any): bool', 'E1135: Using a String as a Bool'])
enddef

def Test_input()
  v9.CheckDefAndScriptFailure(['input(5)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['input(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['input("p", 10)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['input("p", "q", 20)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
enddef

def Test_inputdialog()
  v9.CheckDefAndScriptFailure(['inputdialog(5)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['inputdialog(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['inputdialog("p", 10)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['inputdialog("p", "q", 20)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
enddef

def Test_inputlist()
  v9.CheckDefAndScriptFailure(['inputlist(10)'], ['E1013: Argument 1: type mismatch, expected list<string> but got number', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['inputlist("abc")'], ['E1013: Argument 1: type mismatch, expected list<string> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefFailure(['inputlist([1, 2, 3])'], 'E1013: Argument 1: type mismatch, expected list<string> but got list<number>')
  feedkeys("2\<CR>", 't')
  var r: number = inputlist(['a', 'b', 'c'])
  assert_equal(2, r)
enddef

def Test_inputsecret()
  v9.CheckDefAndScriptFailure(['inputsecret(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['inputsecret("Pass:", 20)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  feedkeys("\<CR>", 't')
  var ans: string = inputsecret('Pass:', '123')
  assert_equal('123', ans)
enddef

let s:number_one = 1
let s:number_two = 2
let s:string_keep = 'keep'

def Test_insert()
  var l = insert([2, 1], 3)
  var res = 0
  for n in l
    res += n
  endfor
  res->assert_equal(6)

  var m: any = []
  insert(m, 4)
  call assert_equal([4], m)
  extend(m, [6], 0)
  call assert_equal([6, 4], m)

  var lines =<< trim END
      insert(test_null_list(), 123)
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1130:', 1)

  lines =<< trim END
      insert(test_null_blob(), 123)
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1131:', 1)

  assert_equal([1, 2, 3], insert([2, 3], 1))
  assert_equal([1, 2, 3], insert([2, 3], number_one))
  assert_equal([1, 2, 3], insert([1, 2], 3, 2))
  assert_equal([1, 2, 3], insert([1, 2], 3, number_two))
  assert_equal(['a', 'b', 'c'], insert(['b', 'c'], 'a'))
  assert_equal(0z1234, insert(0z34, 0x12))

  v9.CheckDefAndScriptFailure(['insert("a", 1)'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E1226: List or Blob required for argument 1'])
  v9.CheckDefFailure(['insert([2, 3], "a")'], 'E1013: Argument 2: type mismatch, expected number but got string')
  v9.CheckDefAndScriptFailure(['insert([2, 3], 1, "x")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
enddef

def Test_instanceof()
  var lines =<< trim END
    vim9script
    class Foo
    endclass
    instanceof('hello', Foo)
  END
  v9.CheckScriptFailure(lines, 'E616: Object required for argument 1')

  lines =<< trim END
    vim9script
    class Foo
    endclass
    instanceof(Foo.new(), 123)
  END
  v9.CheckScriptFailure(lines, 'E693: Class or class typealias required for argument 2')

  lines =<< trim END
    vim9script
    class Foo
    endclass
    def Bar()
      instanceof('hello', Foo)
    enddef
    Bar()
  END
  v9.CheckScriptFailure(lines, 'E1013: Argument 1: type mismatch, expected object<Unknown> but got string')

  lines =<< trim END
    vim9script
    class Foo
    endclass
    def Bar()
      instanceof(Foo.new(), 123)
    enddef
    Bar()
  END
  v9.CheckScriptFailure(lines, 'E693: Class or class typealias required for argument 2')

  lines =<< trim END
    vim9script
    class Foo
    endclass
    instanceof(Foo.new(), [{}])
  END
  v9.CheckSourceFailure(lines, 'E693: Class or class typealias required for argument 2')

  lines =<< trim END
    vim9script
    class Foo
    endclass
    def Bar()
      instanceof(Foo.new(), [{}])
    enddef
    Bar()
  END
  v9.CheckSourceFailure(lines, 'E693: Class or class typealias required for argument 2')
enddef

def Test_invert()
  v9.CheckDefAndScriptFailure(['invert("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_isdirectory()
  v9.CheckDefAndScriptFailure(['isdirectory(1.1)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1174: String required for argument 1'])
  assert_false(isdirectory('NonExistingDir'))
  assert_false(isdirectory(''))
enddef

def Test_islocked()
  v9.CheckDefAndScriptFailure(['islocked(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['var n1: number = 10', 'islocked(n1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  g:v1 = 10
  assert_false(islocked('g:v1'))
  lockvar g:v1
  assert_true(islocked('g:v1'))
  unlet g:v1
  islocked('')->assert_equal(-1)
enddef

def Test_items()
  v9.CheckDefFailure(['123->items()'], 'E1225:')
  assert_equal([['a', 10], ['b', 20]], {'a': 10, 'b': 20}->items())
  assert_equal([], {}->items())
  assert_equal(['x', 'x'], {'a': 10, 'b': 20}->items()->map((_, _) => 'x'))

  assert_equal([[0, 'a'], [1, 'b']], ['a', 'b']->items())
  assert_equal([], []->items())
  assert_equal([], test_null_list()->items())

  assert_equal([[0, 'a'], [1, '웃'], [2, 'ć']], 'a웃ć'->items())
  assert_equal([], ''->items())
  assert_equal([], test_null_string()->items())
enddef

def Test_job_getchannel()
  if !has('job')
    CheckFeature job
  else
    v9.CheckDefAndScriptFailure(['job_getchannel("a")'], ['E1013: Argument 1: type mismatch, expected job but got string', 'E1218: Job required for argument 1'])
    assert_fails('job_getchannel(test_null_job())', 'E916: Not a valid job')
  endif
enddef

def Test_job_info()
  if !has('job')
    CheckFeature job
  else
    v9.CheckDefAndScriptFailure(['job_info("a")'], ['E1013: Argument 1: type mismatch, expected job but got string', 'E1218: Job required for argument 1'])
    assert_fails('job_info(test_null_job())', 'E916: Not a valid job')
  endif
enddef

" Test_job_info_return_type() is in test_vim9_fails.vim

def Test_job_setoptions()
  if !has('job')
    CheckFeature job
  else
    v9.CheckDefAndScriptFailure(['job_setoptions(test_null_channel(), {})'], ['E1013: Argument 1: type mismatch, expected job but got channel', 'E1218: Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['job_setoptions(test_null_job(), [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
    assert_equal('fail', job_status(test_null_job()))
  endif
enddef

def Test_job_status()
  if !has('job')
    CheckFeature job
  else
    v9.CheckDefAndScriptFailure(['job_status("a")'], ['E1013: Argument 1: type mismatch, expected job but got string', 'E1218: Job required for argument 1'])
    assert_equal('fail', job_status(test_null_job()))
  endif
enddef

def Test_job_stop()
  if !has('job')
    CheckFeature job
  else
    v9.CheckDefAndScriptFailure(['job_stop("a")'], ['E1013: Argument 1: type mismatch, expected job but got string', 'E1218: Job required for argument 1'])
    v9.CheckDefAndScriptFailure(['job_stop(test_null_job(), true)'], ['E1013: Argument 2: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 2'])
  endif
enddef

def Test_join()
  v9.CheckDefAndScriptFailure(['join("abc")'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['join([], 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  join([''], '')->assert_equal('')
enddef

def Test_js_decode()
  v9.CheckDefAndScriptFailure(['js_decode(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal([1, 2], js_decode('[1,2]'))
  js_decode('')->assert_equal(v:none)
enddef

def Test_json_decode()
  v9.CheckDefAndScriptFailure(['json_decode(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1174: String required for argument 1'])
  assert_equal(1.0, json_decode('1.0'))
  json_decode('')->assert_equal(v:none)
enddef

def Test_keys()
  assert_equal([7, 7], keys({a: 1, b: 2})->map((_, _) => 7))

  v9.CheckDefAndScriptFailure(['keys([])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 1'])
  assert_equal(['a'], {a: 'v'}->keys())
  assert_equal([], {}->keys())
enddef

def Test_keys_return_type()
  const var: list<string> = {a: 1, b: 2}->keys()
  var->assert_equal(['a', 'b'])
enddef

def Test_len()
  v9.CheckDefAndScriptFailure(['len(true)'], ['E1013: Argument 1: type mismatch, expected list<any> but got bool', 'E701: Invalid type for len()'])
  assert_equal(2, "ab"->len())
  assert_equal(3, 456->len())
  assert_equal(0, []->len())
  assert_equal(1, {a: 10}->len())
  assert_equal(4, 0z20304050->len())
enddef

def Test_libcall()
  CheckFeature libcall
  v9.CheckDefAndScriptFailure(['libcall(1, "b", 3)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['libcall("a", 2, 3)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['libcall("a", "b", 1.1)'], ['E1013: Argument 3: type mismatch, expected string but got float', 'E1220: String or Number required for argument 3'])
enddef

def Test_libcallnr()
  CheckFeature libcall
  v9.CheckDefAndScriptFailure(['libcallnr(1, "b", 3)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['libcallnr("a", 2, 3)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['libcallnr("a", "b", 1.1)'], ['E1013: Argument 3: type mismatch, expected string but got float', 'E1220: String or Number required for argument 3'])
enddef

def Test_line()
  assert_fails('line(true)', 'E1174:')
  v9.CheckDefAndScriptFailure(['line(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['line(".", "a")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['line("")'], 'E1209: Invalid value for a line number')
enddef

def Test_line2byte()
  v9.CheckDefAndScriptFailure(['line2byte(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['line2byte("")'], 'E1209: Invalid value for a line number')
  assert_equal(-1, line2byte(1))
  assert_equal(-1, line2byte(10000))
enddef

def Test_lispindent()
  v9.CheckDefAndScriptFailure(['lispindent({})'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['lispindent("")'], 'E1209: Invalid value for a line number')
  v9.CheckDefExecAndScriptFailure(['lispindent(-1)'], 'E966: Invalid line number: -1')
  assert_equal(0, lispindent(1))
enddef

def Test_list2blob()
  v9.CheckDefAndScriptFailure(['list2blob(10)'], ['E1013: Argument 1: type mismatch, expected list<number> but got number', 'E1211: List required for argument 1'])
  v9.CheckDefFailure(['list2blob([0z10, 0z02])'], 'E1013: Argument 1: type mismatch, expected list<number> but got list<blob>')
enddef

def Test_list2str_str2list_utf8()
  var s = "\u3042\u3044"
  var l = [0x3042, 0x3044]
  str2list(s, true)->assert_equal(l)
  list2str(l, true)->assert_equal(s)
enddef

def Test_list2str()
  v9.CheckDefAndScriptFailure(['list2str(".", true)'], ['E1013: Argument 1: type mismatch, expected list<number> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['list2str([1], 0z10)'], ['E1013: Argument 2: type mismatch, expected bool but got blob', 'E1212: Bool required for argument 2'])
enddef

def s:SID(): number
  return expand('<SID>')
          ->matchstr('<SNR>\zs\d\+\ze_$')
          ->str2nr()
enddef

def Test_listener_add()
  v9.CheckDefAndScriptFailure(['listener_add("1", true)'], ['E1013: Argument 2: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 2'])
enddef

def Test_listener_flush()
  v9.CheckDefAndScriptFailure(['listener_flush([1])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
enddef

def Test_listener_remove()
  v9.CheckDefAndScriptFailure(['listener_remove("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_luaeval()
  if !has('lua')
    CheckFeature lua
  endif
  v9.CheckDefAndScriptFailure(['luaeval(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  if exists_compiled('*luaeval')
    luaeval('')->assert_equal(v:null)
  endif
enddef

def Test_map()
  if has('channel')
    v9.CheckDefAndScriptFailure(['map(test_null_channel(), "1")'], ['E1013: Argument 1: type mismatch, expected list<any> but got channel', 'E1251: List, Dictionary, Blob or String required for argument 1'])
  endif
  v9.CheckDefAndScriptFailure(['map(1, "1")'], ['E1013: Argument 1: type mismatch, expected list<any> but got number', 'E1251: List, Dictionary, Blob or String required for argument 1'])
  v9.CheckDefAndScriptFailure(['map([1, 2], 4)'], ['E1256: String or function required for argument 2', 'E1024: Using a Number as a String'])

  # type of dict remains dict<any> even when type of values changes
  # same for list
  var lines =<< trim END
      var d: dict<any> = {a: 0}
      d->map((k, v) => true)
      d->map((k, v) => 'x')
      assert_equal({a: 'x'}, d)

      d = {a: 0}
      g:gd = d
      map(g:gd, (k, v) => true)
      assert_equal({a: true}, g:gd)

      var l: list<any> = [0]
      l->map((k, v) => true)
      l->map((k, v) => 'x')
      assert_equal(['x'], l)

      l = [1]
      g:gl = l
      map(g:gl, (k, v) => true)
      assert_equal([true], g:gl)

      assert_equal(['x'], [[1, 2]]->map((_, v) => 'x'))
      assert_equal(['x'], [{a: 0}]->map((_, v) => 'x'))
      assert_equal({a: 'x'}, {a: [1, 2]}->map((_, v) => 'x'))
      assert_equal({a: 'x'}, {a: {b: 2}}->map((_, v) => 'x'))
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_map_failure()
  CheckFeature job

  var lines =<< trim END
      vim9script
      writefile([], 'Xtmpfile')
      silent e Xtmpfile
      var d = {[bufnr('%')]: {a: 0}}
      au BufReadPost * Func()
      def Func()
          if d->has_key('')
          endif
          eval d[expand('<abuf>')]->mapnew((_, v: dict<job>) => 0)
      enddef
      e
  END
  v9.CheckScriptFailure(lines, 'E1013:')
  au! BufReadPost
  delete('Xtmpfile')

  lines =<< trim END
      var d: dict<number> = {a: 1}
      g:gd = d
      map(g:gd, (k, v) => true)
  END
  v9.CheckDefExecAndScriptFailure(lines, 'E1012: Type mismatch; expected number but got bool')
enddef

def Test_map_const()
  var lines =<< trim END
      const l = [1, 2, 3]
      map(l, 'SomeFunc')
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const list<number>')

  lines =<< trim END
      const d = {a: 1, b: 2}
      map(d, 'SomeFunc')
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const dict<number>')
enddef

def Test_map_function_arg()
  var lines =<< trim END
      def MapOne(i: number, v: string): string
        return i .. ':' .. v
      enddef
      var l = ['a', 'b', 'c']
      map(l, MapOne)
      assert_equal(['0:a', '1:b', '2:c'], l)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      range(3)->map((a, b, c) => a + b + c)
  END
  v9.CheckDefAndScriptFailure(lines, ['E176:', 'E1190: One argument too few'])
  lines =<< trim END
      range(3)->map((a, b, c, d) => a + b + c + d)
  END
  v9.CheckDefAndScriptFailure(lines, ['E176:', 'E1190: 2 arguments too few'])

  # declared list cannot change type
  lines =<< trim END
    def Map(i: number, v: number): string
      return 'bad'
    enddef
    var ll: list<number> = [1, 2, 3]
    echo map(ll, Map)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?number): number but got func(number, number): string', 'E1012: Type mismatch; expected number but got string'])

  # not declared list can change type
  echo [1, 2, 3]->map((..._) => 'x')
enddef

def Test_map_item_type()
  var lines =<< trim END
      var l = ['a', 'b', 'c']
      map(l, (k, v) => k .. '/' .. v )
      assert_equal(['0/a', '1/b', '2/c'], l)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
    var l: list<number> = [0]
    echo map(l, (_, v) => [])
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?number): number but got func(any, any): list<any>', 'E1012: Type mismatch; expected number but got list<any>'], 2)

  lines =<< trim END
    var l: list<number> = range(2)
    echo map(l, (_, v) => [])
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?number): number but got func(any, any): list<any>', 'E1012: Type mismatch; expected number but got list<any>'], 2)

  lines =<< trim END
    var d: dict<number> = {key: 0}
    echo map(d, (_, v) => [])
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?string, ?number): number but got func(any, any): list<any>', 'E1012: Type mismatch; expected number but got list<any>'], 2)
enddef

def Test_maparg()
  var lnum = str2nr(expand('<sflnum>'))
  map foo bar
  maparg('foo', '', false, true)->assert_equal({
        lnum: lnum + 1,
        script: 0,
        mode: ' ',
        silent: 0,
        noremap: 0,
        lhs: 'foo',
        lhsraw: 'foo',
        nowait: 0,
        expr: 0,
        sid: SID(),
        scriptversion: 999999,
        rhs: 'bar',
        buffer: 0,
        abbr: 0,
        mode_bits: 0x47})
  unmap foo
  v9.CheckDefAndScriptFailure(['maparg(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['maparg("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['maparg("a", "b", 2)'], ['E1013: Argument 3: type mismatch, expected bool but got number', 'E1212: Bool required for argument 3'])
  v9.CheckDefAndScriptFailure(['maparg("a", "b", true, 2)'], ['E1013: Argument 4: type mismatch, expected bool but got number', 'E1212: Bool required for argument 4'])
  maparg('')->assert_equal('')

  # value argument type is checked at compile time
  var lines =<< trim END
      var l = [123]
      l->map((i: number, v: string) => 0)
  END
  v9.CheckDefFailure(lines, 'E1013: Argument 2: type mismatch, expected func(?number, ?number): number but got func(number, string): number')

  lines =<< trim END
      var d = {a: 123}
      d->map((i: string, v: string) => 0)
  END
  v9.CheckDefFailure(lines, 'E1013: Argument 2: type mismatch, expected func(?string, ?number): number but got func(string, string): number')

  lines =<< trim END
    var s = 'abc'
    s->map((i: number, v: number) => 'x')
  END
  v9.CheckDefFailure(lines, 'E1013: Argument 2: type mismatch, expected func(?number, ?string): string but got func(number, number): string')

  lines =<< trim END
    var s = 0z1122
    s->map((i: number, v: string) => 0)
  END
  v9.CheckDefFailure(lines, 'E1013: Argument 2: type mismatch, expected func(?number, ?number): number but got func(number, string): number')

  # index argument type is checked at compile time
  lines =<< trim END
      ['x']->map((i: string, v: string) => 'y')
  END
  v9.CheckDefFailure(lines, 'E1013: Argument 2: type mismatch, expected func(?number, ?any): any but got func(string, string): string')

  lines =<< trim END
    {a: 1}->map((i: number, v: number) => 0)
  END
  v9.CheckDefFailure(lines, 'E1013: Argument 2: type mismatch, expected func(?string, ?any): any but got func(number, number): number')

  lines =<< trim END
    'abc'->map((i: string, v: string) => 'x')
  END
  v9.CheckDefFailure(lines, 'E1013: Argument 2: type mismatch, expected func(?number, ?string): string but got func(string, string): string')

  lines =<< trim END
    0z1122->map((i: string, v: number) => 0)
  END
  v9.CheckDefFailure(lines, 'E1013: Argument 2: type mismatch, expected func(?number, ?number): number but got func(string, number): number')
enddef

def Test_maparg_mapset()
  nnoremap <F3> :echo "hit F3"<CR>
  var mapsave = maparg('<F3>', 'n', false, true)
  mapset('n', false, mapsave)

  nunmap <F3>
enddef

def Test_mapcheck()
  iabbrev foo foobar
  mapcheck('foo', 'i', true)->assert_equal('foobar')
  iunabbrev foo
  v9.CheckDefAndScriptFailure(['mapcheck(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['mapcheck("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['mapcheck("a", "b", 2)'], ['E1013: Argument 3: type mismatch, expected bool but got number', 'E1212: Bool required for argument 3'])
  mapcheck('')->assert_equal('')
  mapcheck('', '')->assert_equal('')
enddef

def Test_mapnew()
  if has('channel')
    v9.CheckDefAndScriptFailure(['mapnew(test_null_job(), "1")'], ['E1013: Argument 1: type mismatch, expected list<any> but got job', 'E1251: List, Dictionary, Blob or String required for argument 1'])
  endif
  v9.CheckDefAndScriptFailure(['mapnew(1, "1")'], ['E1013: Argument 1: type mismatch, expected list<any> but got number', 'E1251: List, Dictionary, Blob or String required for argument 1'])
enddef

def Test_mapset()
  v9.CheckDefAndScriptFailure(['mapset(1, true, {})'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1223: String or Dictionary required for argument 1'])
  v9.CheckDefAndScriptFailure(['mapset("a", 2, {})'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
  v9.CheckDefAndScriptFailure(['mapset("a", false, [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
enddef

def Test_match()
  v9.CheckDefAndScriptFailure(['match(0z12, "p")'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['match(["s"], [2])'], ['E1013: Argument 2: type mismatch, expected string but got list<number>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['match("s", "p", "q")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['match("s", "p", 1, "r")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  assert_equal(2, match('ab12cd', '12'))
  assert_equal(-1, match('ab12cd', '34'))
  assert_equal(6, match('ab12cd12ef', '12', 4))
  assert_equal(2, match('abcd', '..', 0, 3))
  assert_equal(1, match(['a', 'b', 'c'], 'b'))
  assert_equal(-1, match(['a', 'b', 'c'], 'd'))
  assert_equal(3, match(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2))
  assert_equal(5, match(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2, 2))
  match('', 'a')->assert_equal(-1)
  match('abc', '')->assert_equal(0)
  match('', '')->assert_equal(0)
enddef

def Test_matchadd()
  v9.CheckDefAndScriptFailure(['matchadd(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchadd("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['matchadd("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['matchadd("a", "b", 1, "d")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  v9.CheckDefAndScriptFailure(['matchadd("a", "b", 1, 1, [])'], ['E1013: Argument 5: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 5'])
  matchadd('', 'a')->assert_equal(-1)
  matchadd('Search', '')->assert_equal(-1)
enddef

def Test_matchaddpos()
  v9.CheckDefAndScriptFailure(['matchaddpos(1, [1])'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchaddpos("a", "b")'], ['E1013: Argument 2: type mismatch, expected list<any> but got string', 'E1211: List required for argument 2'])
  v9.CheckDefAndScriptFailure(['matchaddpos("a", [1], "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['matchaddpos("a", [1], 1, "d")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  v9.CheckDefAndScriptFailure(['matchaddpos("a", [1], 1, 1, [])'], ['E1013: Argument 5: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 5'])
  matchaddpos('', [1])->assert_equal(-1)
enddef

def Test_matcharg()
  v9.CheckDefAndScriptFailure(['matcharg("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_matchdelete()
  v9.CheckDefAndScriptFailure(['matchdelete("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchdelete("x", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchdelete(1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_matchend()
  v9.CheckDefAndScriptFailure(['matchend(0z12, "p")'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchend(["s"], [2])'], ['E1013: Argument 2: type mismatch, expected string but got list<number>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['matchend("s", "p", "q")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['matchend("s", "p", 1, "r")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  assert_equal(4, matchend('ab12cd', '12'))
  assert_equal(-1, matchend('ab12cd', '34'))
  assert_equal(8, matchend('ab12cd12ef', '12', 4))
  assert_equal(4, matchend('abcd', '..', 0, 3))
  assert_equal(1, matchend(['a', 'b', 'c'], 'b'))
  assert_equal(-1, matchend(['a', 'b', 'c'], 'd'))
  assert_equal(3, matchend(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2))
  assert_equal(5, matchend(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2, 2))
  matchend('', 'a')->assert_equal(-1)
  matchend('abc', '')->assert_equal(0)
  matchend('', '')->assert_equal(0)
enddef

def Test_matchfuzzy()
  v9.CheckDefAndScriptFailure(['matchfuzzy({}, "p")'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<any>', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchfuzzy([], 1)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['matchfuzzy([], "a", [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
  matchfuzzy(['abc', 'xyz'], '')->assert_equal([])
  var lines =<< trim END
    var items = [{name: 'xyz', id: 1}, {name: 'def', id: 2},
                 {name: 'abc', id: 3}]
    var l: list<dict<any>> = matchfuzzy(items, 'abc', {key: 'name'})
    assert_equal([{name: 'abc', id: 3}], l)
    var k: list<string> = matchfuzzy(['one', 'two', 'who'], 'o')
    assert_equal(['one', 'two', 'who'], k)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_matchfuzzypos()
  v9.CheckDefAndScriptFailure(['matchfuzzypos({}, "p")'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<any>', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchfuzzypos([], 1)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['matchfuzzypos([], "a", [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
  matchfuzzypos(['abc', 'xyz'], '')->assert_equal([[], [], []])
  var lines =<< trim END
    var items = [{name: 'xyz', id: 1}, {name: 'def', id: 2},
                 {name: 'abc', id: 3}]
    var l: list<dict<any>> = matchfuzzypos(items, 'abc', {key: 'name'})[0]
    assert_equal([{name: 'abc', id: 3}], l)
    var k: list<string> = matchfuzzypos(['one', 'two', 'who'], 'o')[0]
    assert_equal(['one', 'two', 'who'], k)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_matchlist()
  v9.CheckDefAndScriptFailure(['matchlist(0z12, "p")'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchlist(["s"], [2])'], ['E1013: Argument 2: type mismatch, expected string but got list<number>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['matchlist("s", "p", "q")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['matchlist("s", "p", 1, "r")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  var l: list<string> = ['12',  '', '', '', '', '', '', '', '', '']
  assert_equal(l, matchlist('ab12cd', '12'))
  assert_equal([], matchlist('ab12cd', '34'))
  assert_equal(l, matchlist('ab12cd12ef', '12', 4))
  l[0] = 'cd'
  assert_equal(l, matchlist('abcd', '..', 0, 3))
  l[0] = 'b'
  assert_equal(l, matchlist(['a', 'b', 'c'], 'b'))
  assert_equal([], matchlist(['a', 'b', 'c'], 'd'))
  assert_equal(l, matchlist(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2))
  assert_equal(l, matchlist(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2, 2))
  matchlist('', 'a')->assert_equal([])
  matchlist('abc', '')->assert_equal(repeat([''], 10))
  matchlist('', '')->assert_equal(repeat([''], 10))
enddef

def Test_matchstr()
  v9.CheckDefAndScriptFailure(['matchstr(0z12, "p")'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchstr(["s"], [2])'], ['E1013: Argument 2: type mismatch, expected string but got list<number>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['matchstr("s", "p", "q")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['matchstr("s", "p", 1, "r")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  assert_equal('12', matchstr('ab12cd', '12'))
  assert_equal('', matchstr('ab12cd', '34'))
  assert_equal('12', matchstr('ab12cd12ef', '12', 4))
  assert_equal('cd', matchstr('abcd', '..', 0, 3))
  assert_equal('b', matchstr(['a', 'b', 'c'], 'b'))
  assert_equal('', matchstr(['a', 'b', 'c'], 'd'))
  assert_equal('b', matchstr(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2))
  assert_equal('b', matchstr(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2, 2))
  matchstr('', 'a')->assert_equal('')
  matchstr('abc', '')->assert_equal('')
  matchstr('', '')->assert_equal('')
enddef

def Test_matchstrpos()
  v9.CheckDefAndScriptFailure(['matchstrpos(0z12, "p")'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['matchstrpos(["s"], [2])'], ['E1013: Argument 2: type mismatch, expected string but got list<number>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['matchstrpos("s", "p", "q")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['matchstrpos("s", "p", 1, "r")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  assert_equal(['12', 2, 4], matchstrpos('ab12cd', '12'))
  assert_equal(['', -1, -1], matchstrpos('ab12cd', '34'))
  assert_equal(['12', 6, 8], matchstrpos('ab12cd12ef', '12', 4))
  assert_equal(['cd', 2, 4], matchstrpos('abcd', '..', 0, 3))
  assert_equal(['b', 1, 0, 1], matchstrpos(['a', 'b', 'c'], 'b'))
  assert_equal(['', -1, -1, -1], matchstrpos(['a', 'b', 'c'], 'd'))
  assert_equal(['b', 3, 0, 1],
                    matchstrpos(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2))
  assert_equal(['b', 5, 0, 1],
                    matchstrpos(['a', 'b', 'c', 'b', 'd', 'b'], 'b', 2, 2))
  matchstrpos('', 'a')->assert_equal(['', -1, -1])
  matchstrpos('abc', '')->assert_equal(['', 0, 0])
  matchstrpos('', '')->assert_equal(['', 0, 0])
enddef

def Test_max()
  g:flag = true
  var l1: list<number> = g:flag
          ? [1, max([2, 3])]
          : [4, 5]
  assert_equal([1, 3], l1)

  g:flag = false
  var l2: list<number> = g:flag
          ? [1, max([2, 3])]
          : [4, 5]
  assert_equal([4, 5], l2)
  v9.CheckDefAndScriptFailure(['max(5)'], ['E1013: Argument 1: type mismatch, expected list<any> but got number', 'E1227: List or Dictionary required for argument 1'])
enddef

def Test_menu_info()
  v9.CheckDefAndScriptFailure(['menu_info(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['menu_info(10, "n")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['menu_info("File", 10)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  assert_equal({}, menu_info('aMenu'))
enddef

def Test_min()
  g:flag = true
  var l1: list<number> = g:flag
          ? [1, min([2, 3])]
          : [4, 5]
  assert_equal([1, 2], l1)

  g:flag = false
  var l2: list<number> = g:flag
          ? [1, min([2, 3])]
          : [4, 5]
  assert_equal([4, 5], l2)
  v9.CheckDefAndScriptFailure(['min(5)'], ['E1013: Argument 1: type mismatch, expected list<any> but got number', 'E1227: List or Dictionary required for argument 1'])
enddef

def Test_mkdir()
  v9.CheckDefAndScriptFailure(['mkdir(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['mkdir("a", {})'], ['E1013: Argument 2: type mismatch, expected string but got dict<any>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['mkdir("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefExecAndScriptFailure(['mkdir("")'], 'E1175: Non-empty string required for argument 1')
  delete('a', 'rf')
enddef

def Test_mode()
  v9.CheckDefAndScriptFailure(['mode("1")'], ['E1013: Argument 1: type mismatch, expected bool but got string', 'E1212: Bool required for argument 1'])
  v9.CheckDefAndScriptFailure(['mode(2)'], ['E1013: Argument 1: type mismatch, expected bool but got number', 'E1212: Bool required for argument 1'])
enddef

def Test_mzeval()
  if !has('mzscheme')
    CheckFeature mzscheme
  endif
  v9.CheckDefAndScriptFailure(['mzeval(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
enddef

def Test_nextnonblank()
  v9.CheckDefAndScriptFailure(['nextnonblank(null)'], ['E1013: Argument 1: type mismatch, expected string but got special', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['nextnonblank("")'], 'E1209: Invalid value for a line number')
  assert_equal(0, nextnonblank(1))
enddef

def Test_nr2char()
  nr2char(97, true)->assert_equal('a')
  v9.CheckDefAndScriptFailure(['nr2char("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['nr2char(1, "a")'], ['E1013: Argument 2: type mismatch, expected bool but got string', 'E1212: Bool required for argument 2'])
enddef

def Test_or()
  v9.CheckDefAndScriptFailure(['or("x", 0x2)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['or(0x1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_pathshorten()
  v9.CheckDefAndScriptFailure(['pathshorten(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['pathshorten("a", "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  pathshorten('')->assert_equal('')
enddef

def Test_perleval()
  if !has('perl')
    CheckFeature perl
  endif
  v9.CheckDefAndScriptFailure(['perleval(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
enddef

def Test_popup_atcursor()
  v9.CheckDefAndScriptFailure(['popup_atcursor({"a": 10}, {})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1224: String, Number or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_atcursor("a", [1, 2])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 2'])

  # Pass variable of type 'any' to popup_atcursor()
  var what: any = 'Hello'
  var popupID = what->popup_atcursor({moved: 'any'})
  assert_equal(0, popupID->popup_getoptions().tabpage)
  popupID->popup_close()
enddef

def Test_popup_beval()
  v9.CheckDefAndScriptFailure(['popup_beval({"a": 10}, {})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1224: String, Number or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_beval("a", [1, 2])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_popup_clear()
  v9.CheckDefAndScriptFailure(['popup_clear(["a"])'], ['E1013: Argument 1: type mismatch, expected bool but got list<string>', 'E1212: Bool required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_clear(2)'], ['E1013: Argument 1: type mismatch, expected bool but got number', 'E1212: Bool required for argument 1'])
enddef

def Test_popup_close()
  v9.CheckDefAndScriptFailure(['popup_close("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_popup_create()
  # Pass variable of type 'any' to popup_create()
  var what: any = 'Hello'
  var popupID = what->popup_create({})
  assert_equal(0, popupID->popup_getoptions().tabpage)
  popupID->popup_close()
enddef

def Test_popup_dialog()
  v9.CheckDefAndScriptFailure(['popup_dialog({"a": 10}, {})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1224: String, Number or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_dialog("a", [1, 2])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_popup_filter_menu()
  v9.CheckDefAndScriptFailure(['popup_filter_menu("x", "")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_filter_menu(1, 1)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  var id: number = popup_menu(["one", "two", "three"], {})
  popup_filter_menu(id, '')
  popup_close(id)
enddef

def Test_popup_filter_yesno()
  v9.CheckDefAndScriptFailure(['popup_filter_yesno("x", "")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_filter_yesno(1, 1)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
enddef

def Test_popup_getoptions()
  v9.CheckDefAndScriptFailure(['popup_getoptions("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_getoptions(true)'], ['E1013: Argument 1: type mismatch, expected number but got bool', 'E1210: Number required for argument 1'])
enddef

def Test_popup_getpos()
  v9.CheckDefAndScriptFailure(['popup_getpos("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_getpos(true)'], ['E1013: Argument 1: type mismatch, expected number but got bool', 'E1210: Number required for argument 1'])
enddef

def Test_popup_hide()
  v9.CheckDefAndScriptFailure(['popup_hide("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_hide(true)'], ['E1013: Argument 1: type mismatch, expected number but got bool', 'E1210: Number required for argument 1'])
enddef

def Test_popup_locate()
  v9.CheckDefAndScriptFailure(['popup_locate("a", 20)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_locate(10, "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_popup_menu()
  v9.CheckDefAndScriptFailure(['popup_menu({"a": 10}, {})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1224: String, Number or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_menu("a", [1, 2])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_popup_move()
  v9.CheckDefAndScriptFailure(['popup_move("x", {})'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_move(1, [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_popup_notification()
  v9.CheckDefAndScriptFailure(['popup_notification({"a": 10}, {})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1224: String, Number or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_notification("a", [1, 2])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_popup_setoptions()
  v9.CheckDefAndScriptFailure(['popup_setoptions("x", {})'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_setoptions(1, [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_popup_settext()
  v9.CheckDefAndScriptFailure(['popup_settext("x", [])'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_settext(1, 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1222: String or List required for argument 2'])
enddef

def Test_popup_show()
  v9.CheckDefAndScriptFailure(['popup_show("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['popup_show(true)'], ['E1013: Argument 1: type mismatch, expected number but got bool', 'E1210: Number required for argument 1'])
enddef

def Test_prevnonblank()
  v9.CheckDefAndScriptFailure(['prevnonblank(null)'], ['E1013: Argument 1: type mismatch, expected string but got special', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['prevnonblank("")'], 'E1209: Invalid value for a line number')
  assert_equal(0, prevnonblank(1))
enddef

def Test_printf()
  v9.CheckDefAndScriptFailure(['printf([1])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
  printf(0x10)->assert_equal('16')
  assert_equal(" abc", "abc"->printf("%4s"))
enddef

def Test_prompt_getprompt()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['prompt_getprompt([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
    assert_equal('', prompt_getprompt('NonExistingBuf'))
  endif
enddef

def Test_prompt_setcallback()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['prompt_setcallback(true, "1")'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  endif
enddef

def Test_prompt_setinterrupt()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['prompt_setinterrupt(true, "1")'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  endif
enddef

def Test_prompt_setprompt()
  if !has('channel')
    CheckFeature channel
  else
    v9.CheckDefAndScriptFailure(['prompt_setprompt([], "p")'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
    v9.CheckDefAndScriptFailure(['prompt_setprompt(1, [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1174: String required for argument 2'])
  endif
enddef

def Test_prop_add()
  v9.CheckDefAndScriptFailure(['prop_add("a", 2, {})'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_add(1, "b", {})'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['prop_add(1, 2, [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
enddef

def Test_prop_add_list()
  v9.CheckDefAndScriptFailure(['prop_add_list([], [])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_add_list({}, {})'], ['E1013: Argument 2: type mismatch, expected list<any> but got dict<any>', 'E1211: List required for argument 2'])
enddef

def Test_prop_clear()
  v9.CheckDefAndScriptFailure(['prop_clear("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_clear(1, "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['prop_clear(1, 2, [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
enddef

def Test_prop_find()
  v9.CheckDefAndScriptFailure(['prop_find([1, 2])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_find([1, 2], "k")'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_find({"a": 10}, ["a"])'], ['E1013: Argument 2: type mismatch, expected string but got list<string>', 'E1174: String required for argument 2'])
  assert_fails("prop_find({}, '')", 'E474:')
enddef

def Test_prop_list()
  v9.CheckDefAndScriptFailure(['prop_list("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_list(1, [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_prop_remove()
  v9.CheckDefAndScriptFailure(['prop_remove([])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_remove({}, "a")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['prop_remove({}, 1, "b")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
enddef

def Test_prop_type_add()
  v9.CheckDefAndScriptFailure(['prop_type_add({"a": 10}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_type_add("a", "b")'], ['E1013: Argument 2: type mismatch, expected dict<any> but got string', 'E1206: Dictionary required for argument 2'])
  assert_fails("prop_type_add('', {highlight: 'Search'})", 'E475:')
enddef

def Test_prop_type_change()
  v9.CheckDefAndScriptFailure(['prop_type_change({"a": 10}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_type_change("a", "b")'], ['E1013: Argument 2: type mismatch, expected dict<any> but got string', 'E1206: Dictionary required for argument 2'])
  assert_fails("prop_type_change('', {highlight: 'Search'})", 'E475:')
enddef

def Test_prop_type_delete()
  v9.CheckDefAndScriptFailure(['prop_type_delete({"a": 10})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_type_delete({"a": 10}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_type_delete("a", "b")'], ['E1013: Argument 2: type mismatch, expected dict<any> but got string', 'E1206: Dictionary required for argument 2'])
  assert_fails("prop_type_delete('')", 'E475:')
enddef

def Test_prop_type_get()
  v9.CheckDefAndScriptFailure(['prop_type_get({"a": 10})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_type_get({"a": 10}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_type_get("a", "b")'], ['E1013: Argument 2: type mismatch, expected dict<any> but got string', 'E1206: Dictionary required for argument 2'])
  assert_fails("prop_type_get('')", 'E475:')
enddef

def Test_prop_type_list()
  v9.CheckDefAndScriptFailure(['prop_type_list(["a"])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<string>', 'E1206: Dictionary required for argument 1'])
  v9.CheckDefAndScriptFailure(['prop_type_list(2)'], ['E1013: Argument 1: type mismatch, expected dict<any> but got number', 'E1206: Dictionary required for argument 1'])
enddef

def Test_py3eval()
  if !has('python3')
    CheckFeature python3
  endif
  v9.CheckDefAndScriptFailure(['py3eval([2])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1174: String required for argument 1'])
enddef

def Test_pyeval()
  if !has('python')
    CheckFeature python
  endif
  v9.CheckDefAndScriptFailure(['pyeval([2])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1174: String required for argument 1'])
enddef

def Test_pyxeval()
  if !has('python') && !has('python3')
    CheckFeature python
  endif
  v9.CheckDefAndScriptFailure(['pyxeval([2])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1174: String required for argument 1'])
enddef

def Test_rand()
  v9.CheckDefAndScriptFailure(['rand(10)'], ['E1013: Argument 1: type mismatch, expected list<number> but got number', 'E1211: List required for argument 1'])
  v9.CheckDefFailure(['rand(["a"])'], 'E1013: Argument 1: type mismatch, expected list<number> but got list<string>')
  assert_true(rand() >= 0)
  assert_true(rand(srand()) >= 0)
enddef

def Test_range()
  v9.CheckDefAndScriptFailure(['range("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['range(10, "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['range(10, 20, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])

  # returns a list<number> but it's not declared as such
  assert_equal(['x', 'x'], range(2)->map((i, v) => 'x'))
enddef

def Test_readdir()
  eval expand('sautest')->readdir((e) => e[0] !=# '.')
  eval expand('sautest')->readdirex((e) => e.name[0] !=# '.')
  v9.CheckDefAndScriptFailure(['readdir(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['readdir("a", "1", [3])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])
  if has('unix')
    # only fails on Unix-like systems
    assert_fails('readdir("")', 'E484: Can''t open file')
  endif
enddef

def Test_readdirex()
  v9.CheckDefAndScriptFailure(['readdirex(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['readdirex("a", "1", [3])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])
  if has('unix')
    # only fails on Unix-like systems
    assert_fails('readdirex("")', 'E484: Can''t open file')
  endif
enddef

def Test_readblob()
  var blob = 0z12341234
  writefile(blob, 'Xreadblob', 'D')
  var read: blob = readblob('Xreadblob')
  assert_equal(blob, read)

  var lines =<< trim END
      var read: list<string> = readblob('Xreadblob')
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012: Type mismatch; expected list<string> but got blob', 1)
  v9.CheckDefExecAndScriptFailure(['readblob("")'], 'E484: Can''t open file <empty>')
enddef

def Test_readfile()
  var text = ['aaa', 'bbb', 'ccc']
  writefile(text, 'Xreadfile', 'D')
  var read: list<string> = readfile('Xreadfile')
  assert_equal(text, read)
  assert_equal([7, 7, 7], readfile('Xreadfile')->map((_, _) => 7))

  var lines =<< trim END
      var read: dict<string> = readfile('Xreadfile')
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012: Type mismatch; expected dict<string> but got list<string>', 1)

  v9.CheckDefAndScriptFailure(['readfile("a", 0z10)'], ['E1013: Argument 2: type mismatch, expected string but got blob', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['readfile("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefExecAndScriptFailure(['readfile("")'], 'E1175: Non-empty string required for argument 1')
enddef

def Test_reduce()
  v9.CheckDefAndScriptFailure(['reduce({a: 10}, "1")'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<number>', 'E1252: String, List or Blob required for argument 1'])
  assert_equal(6, [1, 2, 3]->reduce((r, c) => r + c, 0))
  assert_equal(11, 0z0506->reduce((r, c) => r + c, 0))
enddef

def Test_reltime()
  CheckFeature reltime

  v9.CheckDefExecAndScriptFailure(['[]->reltime()'], 'E474:')
  v9.CheckDefExecAndScriptFailure(['[]->reltime([])'], 'E474:')

  v9.CheckDefAndScriptFailure(['reltime("x")'], ['E1013: Argument 1: type mismatch, expected list<number> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefFailure(['reltime(["x", "y"])'], 'E1013: Argument 1: type mismatch, expected list<number> but got list<string>')
  v9.CheckDefAndScriptFailure(['reltime([1, 2], 10)'], ['E1013: Argument 2: type mismatch, expected list<number> but got number', 'E1211: List required for argument 2'])
  v9.CheckDefFailure(['reltime([1, 2], ["a", "b"])'], 'E1013: Argument 2: type mismatch, expected list<number> but got list<string>')
  var start: list<any> = reltime()
  assert_true(type(reltime(start)) == v:t_list)
  var end: list<any> = reltime()
  assert_true(type(reltime(start, end)) == v:t_list)
enddef

def Test_reltimefloat()
  CheckFeature reltime

  v9.CheckDefExecAndScriptFailure(['[]->reltimefloat()'], 'E474:')

  v9.CheckDefAndScriptFailure(['reltimefloat("x")'], ['E1013: Argument 1: type mismatch, expected list<number> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefFailure(['reltimefloat([1.1])'], 'E1013: Argument 1: type mismatch, expected list<number> but got list<float>')
  assert_true(type(reltimefloat(reltime())) == v:t_float)
enddef

def Test_reltimestr()
  CheckFeature reltime

  v9.CheckDefExecAndScriptFailure(['[]->reltimestr()'], 'E474:')

  v9.CheckDefAndScriptFailure(['reltimestr(true)'], ['E1013: Argument 1: type mismatch, expected list<number> but got bool', 'E1211: List required for argument 1'])
  v9.CheckDefFailure(['reltimestr([true])'], 'E1013: Argument 1: type mismatch, expected list<number> but got list<bool>')
  assert_true(type(reltimestr(reltime())) == v:t_string)
enddef

def Test_remote_expr()
  CheckFeature clientserver
  CheckEnv DISPLAY
  v9.CheckDefAndScriptFailure(['remote_expr(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['remote_expr("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['remote_expr("a", "b", 3)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  v9.CheckDefAndScriptFailure(['remote_expr("a", "b", "c", "d")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  v9.CheckDefExecAndScriptFailure(['remote_expr("", "")'], 'E241: Unable to send to ')
enddef

def Test_remote_foreground()
  CheckFeature clientserver
  # remote_foreground() doesn't fail on MS-Windows
  CheckNotMSWindows
  CheckX11

  v9.CheckDefAndScriptFailure(['remote_foreground(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_fails('remote_foreground("NonExistingServer")', 'E241:')
  assert_fails('remote_foreground("")', 'E241:')
enddef

def Test_remote_peek()
  CheckFeature clientserver
  CheckEnv DISPLAY
  v9.CheckDefAndScriptFailure(['remote_peek(0z10)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['remote_peek("a5b6c7", [1])'], ['E1013: Argument 2: type mismatch, expected string but got list<number>', 'E1174: String required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['remote_peek("")'], 'E573: Invalid server id used')
enddef

def Test_remote_read()
  CheckFeature clientserver
  CheckEnv DISPLAY
  v9.CheckDefAndScriptFailure(['remote_read(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['remote_read("a", "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['remote_read("")'], 'E573: Invalid server id used')
enddef

def Test_remote_send()
  CheckFeature clientserver
  CheckEnv DISPLAY
  v9.CheckDefAndScriptFailure(['remote_send(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['remote_send("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['remote_send("a", "b", 3)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  assert_fails('remote_send("", "")', 'E241:')
enddef

def Test_remote_startserver()
  CheckFeature clientserver
  CheckEnv DISPLAY
  v9.CheckDefAndScriptFailure(['remote_startserver({})'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', 'E1174: String required for argument 1'])
enddef

def Test_remove_literal_list()
  var l: list<number> = [1, 2, 3, 4]
  assert_equal([1, 2], remove(l, 0, 1))
  assert_equal([3, 4], l)
enddef

def Test_remove_const()
  var lines =<< trim END
      const l = [1, 2, 3, 4]
      remove(l, 1)
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const list<number>')

  lines =<< trim END
      const d = {a: 1, b: 2}
      remove(d, 'a')
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const dict<number>')

  lines =<< trim END
      const b = 0z010203
      remove(b, 1)
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const blob')
enddef

def Test_remove()
  v9.CheckDefAndScriptFailure(['remove("a", 1)'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E1228: List, Dictionary or Blob required for argument 1'])
  v9.CheckDefAndScriptFailure(['remove([], "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['remove([], 1, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['remove({}, 1.1)'], ['E1013: Argument 2: type mismatch, expected string but got float', 'E1220: String or Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['remove(0z10, "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['remove(0z20, 1, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  var l: any = [1, 2, 3, 4]
  remove(l, 1)
  assert_equal([1, 3, 4], l)
  remove(l, 0, 1)
  assert_equal([4], l)
  var b: any = 0z1234.5678.90
  remove(b, 1)
  assert_equal(0z1256.7890, b)
  remove(b, 1, 2)
  assert_equal(0z1290, b)
  var d: any = {a: 10, b: 20, c: 30}
  remove(d, 'b')
  assert_equal({a: 10, c: 30}, d)
  var d2: any = {1: 'a', 2: 'b', 3: 'c'}
  remove(d2, 2)
  assert_equal({1: 'a', 3: 'c'}, d2)

  # using declared type
  var x: string = range(2)->extend(['x'])->remove(2)
  assert_equal('x', x)
enddef

def Test_remove_return_type()
  var l: list<number> = remove({one: [1, 2], two: [3, 4]}, 'one')
  l->assert_equal([1, 2])

  var ll: list<number> = remove(range(3), 0, 1)
  ll->assert_equal([0, 1])
enddef

def Test_rename()
  v9.CheckDefAndScriptFailure(['rename(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['rename("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  rename('', '')->assert_equal(0)
enddef

def Test_repeat()
  v9.CheckDefAndScriptFailure(['repeat(1.1, 2)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1301: String, Number, List or Blob required for argument 1'])
  v9.CheckDefAndScriptFailure(['repeat({a: 10}, 2)'], ['E1013: Argument 1: type mismatch, expected string but got dict<', 'E1301: String, Number, List or Blob required for argument 1'])
  var lines =<< trim END
      assert_equal('aaa', repeat('a', 3))
      assert_equal('111', repeat(1, 3))
      assert_equal([1, 1, 1], repeat([1], 3))
      assert_equal(0z000102000102000102, repeat(0z000102, 3))
      assert_equal(0z000000, repeat(0z00, 3))
      var s = '-'
      s ..= repeat(5, 3)
      assert_equal('-555', s)
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_resolve()
  v9.CheckDefAndScriptFailure(['resolve([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1174: String required for argument 1'])
  assert_equal('SomeFile', resolve('SomeFile'))
  resolve('')->assert_equal('')
enddef

def Test_reverse()
  v9.CheckDefAndScriptFailure(['reverse(10)'], ['E1013: Argument 1: type mismatch, expected list<any> but got number', 'E1252: String, List or Blob required for argument 1'])
enddef

def Test_reverse_return_type()
  var l = reverse([1, 2, 3])
  var res = 0
  for n in l
    res += n
  endfor
  res->assert_equal(6)
enddef

def Test_reverse_const()
  var lines =<< trim END
      const l = [1, 2, 3, 4]
      reverse(l)
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const list<number>')

  lines =<< trim END
      const b = 0z010203
      reverse(b)
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const blob')
enddef

def Test_rubyeval()
  if !has('ruby')
    CheckFeature ruby
  endif
  v9.CheckDefAndScriptFailure(['rubyeval([2])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1174: String required for argument 1'])
enddef

def Test_screenattr()
  v9.CheckDefAndScriptFailure(['screenattr("x", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['screenattr(1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_screenchar()
  v9.CheckDefAndScriptFailure(['screenchar("x", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['screenchar(1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_screenchars()
  assert_equal(['x'], screenchars(1, 1)->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['screenchars("x", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['screenchars(1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_screenpos()
  v9.CheckDefAndScriptFailure(['screenpos("a", 1, 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['screenpos(1, "b", 1)'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['screenpos(1, 1, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  assert_equal({col: 1, row: 1, endcol: 1, curscol: 1}, screenpos(1, 1, 1))
enddef

def Test_screenstring()
  v9.CheckDefAndScriptFailure(['screenstring("x", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['screenstring(1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_search()
  new
  setline(1, ['foo', 'bar'])
  var val = 0
  # skip expr returns boolean
  search('bar', 'W', 0, 0, () => val == 1)->assert_equal(2)
  :1
  search('bar', 'W', 0, 0, () => val == 0)->assert_equal(0)
  # skip expr returns number, only 0 and 1 are accepted
  :1
  search('bar', 'W', 0, 0, () => 0)->assert_equal(2)
  :1
  search('bar', 'W', 0, 0, () => 1)->assert_equal(0)
  assert_fails("search('bar', '', 0, 0, () => -1)", 'E1023:')
  assert_fails("search('bar', '', 0, 0, () => -1)", 'E1023:')

  setline(1, "find this word")
  normal gg
  var col = 7
  assert_equal(1, search('this', '', 0, 0, 'col(".") > col'))
  normal 0
  assert_equal([1, 6], searchpos('this', '', 0, 0, 'col(".") > col'))

  col = 5
  normal 0
  assert_equal(0, search('this', '', 0, 0, 'col(".") > col'))
  normal 0
  assert_equal([0, 0], searchpos('this', '', 0, 0, 'col(".") > col'))
  bwipe!
  v9.CheckDefAndScriptFailure(['search(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['search("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['search("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['search("a", "b", 3, "d")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  new
  setline(1, "match this")
  v9.CheckDefAndScriptFailure(['search("a", "", 9, 0, [0])'], ['E1013: Argument 5: type mismatch, expected func(...): any but got list<number>', 'E730: Using a List as a String'])
  bwipe!
enddef

def Test_searchcount()
  new
  setline(1, "foo bar")
  :/foo
  searchcount({recompute: true})
      ->assert_equal({
          exact_match: 1,
          current: 1,
          total: 1,
          maxcount: 99,
          incomplete: 0})
  bwipe!
  v9.CheckDefAndScriptFailure(['searchcount([1])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 1'])
enddef

def Test_searchdecl()
  searchdecl('blah', true, true)->assert_equal(1)
  v9.CheckDefAndScriptFailure(['searchdecl(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['searchdecl("a", 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
  v9.CheckDefAndScriptFailure(['searchdecl("a", true, 2)'], ['E1013: Argument 3: type mismatch, expected bool but got number', 'E1212: Bool required for argument 3'])

  # search for an empty string declaration
  var lines: list<string> =<< trim END
    int var1;

    {
       int var2;
       var1 = 10;
    }
  END
  new
  setline(1, lines)
  cursor(5, 4)
  searchdecl('')
  assert_equal([3, 1], [line('.'), col('.')])
  bw!
enddef

def Test_searchpair()
  new
  setline(1, "here { and } there")

  normal f{
  var col = 15
  assert_equal(1, searchpair('{', '', '}', '', 'col(".") > col'))
  assert_equal(12, col('.'))
  normal 0f{
  assert_equal([1, 12], searchpairpos('{', '', '}', '', 'col(".") > col'))

  col = 8
  normal 0f{
  assert_equal(0, searchpair('{', '', '}', '', 'col(".") > col'))
  assert_equal(6, col('.'))
  normal 0f{
  assert_equal([0, 0], searchpairpos('{', '', '}', '', 'col(".") > col'))

  # searchpair with empty strings
  normal 8|
  assert_equal(0, searchpair('', '', ''))
  assert_equal([0, 0], searchpairpos('', '', ''))

  assert_equal(['x', 'x'], searchpairpos('', '', '')->map((_, _) => 'x'))

  var lines =<< trim END
      vim9script
      setline(1, '()')
      normal gg
      func RetList()
        return [0]
      endfunc
      def Fail()
        try
          searchpairpos('(', '', ')', 'nW', 'RetList()')
        catch
          g:caught = 'yes'
        endtry
      enddef
      Fail()
  END
  v9.CheckScriptSuccess(lines)
  assert_equal('yes', g:caught)
  unlet g:caught
  bwipe!

  lines =<< trim END
      echo searchpair("a", "b", "c", "d", "f", 33)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1001: Variable not found: f', 'E475: Invalid argument: d'])

  var errors = ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1']
  v9.CheckDefAndScriptFailure(['searchpair(1, "b", "c")'], errors)
  v9.CheckDefAndScriptFailure(['searchpairpos(1, "b", "c")'], errors)

  errors = ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2']
  v9.CheckDefAndScriptFailure(['searchpair("a", 2, "c")'], errors)
  v9.CheckDefAndScriptFailure(['searchpairpos("a", 2, "c")'], errors)

  errors = ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3']
  v9.CheckDefAndScriptFailure(['searchpair("a", "b", 3)'], errors)
  v9.CheckDefAndScriptFailure(['searchpairpos("a", "b", 3)'], errors)

  errors = ['E1013: Argument 4: type mismatch, expected string but got number', 'E1174: String required for argument 4']
  v9.CheckDefAndScriptFailure(['searchpair("a", "b", "c", 4)'], errors)

  new
  setline(1, "match this")
  errors = ['E1013: Argument 5: type mismatch, expected func(...): any but got list<number>', 'E730: Using a List as a String']
  v9.CheckDefAndScriptFailure(['searchpair("a", "b", "c", "r", [0])'], errors)
  v9.CheckDefAndScriptFailure(['searchpairpos("a", "b", "c", "r", [0])'], errors)
  bwipe!

  errors = ['E1013: Argument 6: type mismatch, expected number but got string', 'E1210: Number required for argument 6']
  v9.CheckDefAndScriptFailure(['searchpair("a", "b", "c", "r", "1", "f")'], errors)
  v9.CheckDefAndScriptFailure(['searchpairpos("a", "b", "c", "r", "1", "f")'], errors)

  errors = ['E1013: Argument 7: type mismatch, expected number but got string', 'E1210: Number required for argument 7']
  v9.CheckDefAndScriptFailure(['searchpair("a", "b", "c", "r", "1", 3, "g")'], errors)
  v9.CheckDefAndScriptFailure(['searchpairpos("a", "b", "c", "r", "1", 3, "g")'], errors)
enddef

def Test_searchpos()
  assert_equal(['x', 'x'], searchpos('.')->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['searchpos(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['searchpos("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['searchpos("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['searchpos("a", "b", 3, "d")'], ['E1013: Argument 4: type mismatch, expected number but got string', 'E1210: Number required for argument 4'])
  new
  setline(1, "match this")
  v9.CheckDefAndScriptFailure(['searchpos("a", "", 9, 0, [0])'], ['E1013: Argument 5: type mismatch, expected func(...): any but got list<number>', 'E730: Using a List as a String'])
  bwipe!
enddef

def Test_server2client()
  CheckFeature clientserver
  CheckEnv DISPLAY
  v9.CheckDefAndScriptFailure(['server2client(10, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['server2client("a", 10)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['server2client("", "a")'], 'E573: Invalid server id used')
  v9.CheckDefExecAndScriptFailure(['server2client("", "")'], 'E573: Invalid server id used')
enddef

def Test_shellescape()
  v9.CheckDefAndScriptFailure(['shellescape(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['shellescape("a", 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
  if has('unix')
    assert_equal("''", shellescape(''))
  endif
enddef

def Test_set_get_bufline()
  # similar to Test_setbufline_getbufline()
  var lines =<< trim END
      new
      var b = bufnr('%')
      hide
      assert_equal(0, setbufline(b, 1, ['foo', 'bar']))
      assert_equal(['foo'], getbufline(b, 1))
      assert_equal(['bar'], getbufline(b, '$'))
      assert_equal(['foo', 'bar'], getbufline(b, 1, 2))
      exe "bd!" b
      assert_equal([], getbufline(b, 1, 2))

      split Xtest
      setline(1, ['a', 'b', 'c'])
      b = bufnr('%')
      wincmd w

      assert_equal(1, setbufline(b, 5, 'x'))
      assert_equal(1, setbufline(b, 5, ['x']))
      assert_equal(0, setbufline(b, 5, []))
      assert_equal(0, setbufline(b, 5, test_null_list()))

      assert_equal(1, 'x'->setbufline(bufnr('$') + 1, 1))
      assert_equal(1, ['x']->setbufline(bufnr('$') + 1, 1))
      assert_equal(1, []->setbufline(bufnr('$') + 1, 1))
      assert_equal(1, test_null_list()->setbufline(bufnr('$') + 1, 1))

      assert_equal(['a', 'b', 'c'], getbufline(b, 1, '$'))

      assert_equal(0, setbufline(b, 4, ['d', 'e']))
      assert_equal(['c'], b->getbufline(3))
      assert_equal(['d'], getbufline(b, 4))
      assert_equal(['e'], getbufline(b, 5))
      assert_equal([], getbufline(b, 6))
      assert_equal([], getbufline(b, 2, 1))

      if has('job')
        setbufline(b, 2, [function('eval'), {key: 123}, string(test_null_job())])
        assert_equal(["function('eval')",
                        "{'key': 123}",
                        "no process"],
                        getbufline(b, 2, 4))
      endif

      exe 'bwipe! ' .. b
  END
  v9.CheckDefAndScriptSuccess(lines)
enddef

def Test_setbufvar()
  setbufvar(bufnr('%'), '&syntax', 'vim')
  &syntax->assert_equal('vim')
  setbufvar(bufnr('%'), '&ts', 16)
  &ts->assert_equal(16)
  setbufvar(bufnr('%'), '&ai', true)
  &ai->assert_equal(true)
  setbufvar(bufnr('%'), '&ft', 'filetype')
  &ft->assert_equal('filetype')

  settabwinvar(1, 1, '&syntax', 'vam')
  &syntax->assert_equal('vam')
  settabwinvar(1, 1, '&ts', 15)
  &ts->assert_equal(15)
  setlocal ts=8
  settabwinvar(1, 1, '&list', false)
  &list->assert_equal(false)
  settabwinvar(1, 1, '&list', true)
  &list->assert_equal(true)
  setlocal list&

  setbufvar('%', 'myvar', 123)
  getbufvar('%', 'myvar')->assert_equal(123)

  v9.CheckDefAndScriptFailure(['setbufvar(true, "v", 3)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['setbufvar(1, 2, 3)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  assert_fails('setbufvar("%", "", 10)', 'E461: Illegal variable name')
enddef

def Test_setbufline()
  new
  var bnum = bufnr('%')
  :wincmd w
  setbufline(bnum, 1, range(1, 3))
  setbufline(bnum, 4, 'one')
  setbufline(bnum, 5, 10)
  setbufline(bnum, 6, ['two', 11])
  assert_equal(['1', '2', '3', 'one', '10', 'two', '11'], getbufline(bnum, 1, '$'))
  v9.CheckDefAndScriptFailure(['setbufline([1], 1, "x")'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['setbufline(1, [1], "x")'], ['E1013: Argument 2: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['setbufline(' .. bnum .. ', -1, "x")'], 'E966: Invalid line number: -1')
  v9.CheckDefAndScriptFailure(['setbufline(1, 1, {"a": 10})'], ['E1013: Argument 3: type mismatch, expected string but got dict<number>', 'E1224: String, Number or List required for argument 3'])
  bnum->bufwinid()->win_gotoid()
  setbufline('', 1, 'nombres')
  getline(1)->assert_equal('nombres')
  bw!
enddef

def Test_setcellwidths()
  v9.CheckDefAndScriptFailure(['setcellwidths(1)'], ['E1013: Argument 1: type mismatch, expected list<any> but got number', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['setcellwidths({"a": 10})'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<number>', 'E1211: List required for argument 1'])
enddef

def Test_setcharpos()
  v9.CheckDefAndScriptFailure(['setcharpos(1, [])'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefFailure(['setcharpos(".", ["a"])'], 'E1013: Argument 2: type mismatch, expected list<number> but got list<string>')
  v9.CheckDefAndScriptFailure(['setcharpos(".", 1)'], ['E1013: Argument 2: type mismatch, expected list<number> but got number', 'E1211: List required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['setcharpos("", [0, 1, 1, 1])'], 'E474: Invalid argument')
enddef

def Test_setcharsearch()
  v9.CheckDefAndScriptFailure(['setcharsearch("x")'], ['E1013: Argument 1: type mismatch, expected dict<any> but got string', 'E1206: Dictionary required for argument 1'])
  v9.CheckDefAndScriptFailure(['setcharsearch([])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 1'])
  var d: dict<any> = {char: 'x', forward: 1, until: 1}
  setcharsearch(d)
  assert_equal(d, getcharsearch())
enddef

def Test_setcmdline()
  v9.CheckDefAndScriptSuccess(['setcmdline("ls", 2)'])
  v9.CheckDefAndScriptFailure(['setcmdline(123)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['setcmdline("ls", "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_setcmdpos()
  v9.CheckDefAndScriptFailure(['setcmdpos("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_setcursorcharpos()
  v9.CheckDefAndScriptFailure(['setcursorcharpos(0z10, 1)'], ['E1013: Argument 1: type mismatch, expected number but got blob', 'E1224: String, Number or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['setcursorcharpos(1, "2")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['setcursorcharpos(1, 2, "3")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefExecAndScriptFailure(['setcursorcharpos("", 10)'], 'E1209: Invalid value for a line number')
enddef

def Test_setenv()
  v9.CheckDefAndScriptFailure(['setenv(1, 2)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal(0, setenv('', ''))
  assert_equal(0, setenv('', v:null))
enddef

def Test_setfperm()
  v9.CheckDefAndScriptFailure(['setfperm(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['setfperm("a", 0z10)'], ['E1013: Argument 2: type mismatch, expected string but got blob', 'E1174: String required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['setfperm("Xfile", "")'], 'E475: Invalid argument')
  v9.CheckDefExecAndScriptFailure(['setfperm("", "")'], 'E475: Invalid argument')
  assert_equal(0, setfperm('', 'rw-r--r--'))
enddef

def Test_setline()
  new
  setline(1, range(1, 4))
  assert_equal(['1', '2', '3', '4'], getline(1, '$'))
  setline(1, ['a', 'b', 'c', 'd'])
  assert_equal(['a', 'b', 'c', 'd'], getline(1, '$'))
  setline(1, 'one')
  assert_equal(['one', 'b', 'c', 'd'], getline(1, '$'))
  setline(1, 10)
  assert_equal(['10', 'b', 'c', 'd'], getline(1, '$'))
  v9.CheckDefAndScriptFailure(['setline([1], "x")'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['setline("", "x")'], 'E1209: Invalid value for a line number')
  v9.CheckDefExecAndScriptFailure(['setline(-1, "x")'], 'E966: Invalid line number: -1')
  assert_fails('setline(".a", "x")', ['E1209:', 'E1209:'])
  bw!
enddef

def Test_setloclist()
  var items = [{filename: '/tmp/file', lnum: 1, valid: true}]
  var what = {items: items}
  setqflist([], ' ', what)
  setloclist(0, [], ' ', what)
  v9.CheckDefAndScriptFailure(['setloclist("1", [])'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['setloclist(1, 2)'], ['E1013: Argument 2: type mismatch, expected list<any> but got number', 'E1211: List required for argument 2'])
  v9.CheckDefAndScriptFailure(['setloclist(1, [], 3)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  v9.CheckDefAndScriptFailure(['setloclist(1, [], "a", [])'], ['E1013: Argument 4: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 4'])
enddef

def Test_setmatches()
  v9.CheckDefAndScriptFailure(['setmatches({})'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<any>', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['setmatches([], "1")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_setpos()
  v9.CheckDefAndScriptFailure(['setpos(1, [])'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefFailure(['setpos(".", ["a"])'], 'E1013: Argument 2: type mismatch, expected list<number> but got list<string>')
  v9.CheckDefAndScriptFailure(['setpos(".", 1)'], ['E1013: Argument 2: type mismatch, expected list<number> but got number', 'E1211: List required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['setpos("", [0, 1, 1, 1])'], 'E474: Invalid argument')
enddef

def Test_setqflist()
  v9.CheckDefAndScriptFailure(['setqflist(1, "")'], ['E1013: Argument 1: type mismatch, expected list<any> but got number', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['setqflist([], 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['setqflist([], "", [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
enddef

def Test_setreg()
  setreg('a', ['aaa', 'bbb', 'ccc'])
  var reginfo = getreginfo('a')
  setreg('a', reginfo)
  getreginfo('a')->assert_equal(reginfo)
  assert_fails('setreg("ab", 0)', 'E1162:')
  v9.CheckDefAndScriptFailure(['setreg(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['setreg("a", "b", 3)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  setreg('', '1a2b3c')
  assert_equal('1a2b3c', @")
enddef

def Test_settabvar()
  v9.CheckDefAndScriptFailure(['settabvar("a", "b", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['settabvar(1, 2, "c")'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  assert_fails('settabvar(1, "", 10)', 'E461: Illegal variable name')
enddef

def Test_settabwinvar()
  v9.CheckDefAndScriptFailure(['settabwinvar("a", 2, "c", true)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['settabwinvar(1, "b", "c", [])'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['settabwinvar(1, 1, 3, {})'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  assert_fails('settabwinvar(1, 1, "", 10)', 'E461: Illegal variable name')
enddef

def Test_settagstack()
  v9.CheckDefAndScriptFailure(['settagstack(true, {})'], ['E1013: Argument 1: type mismatch, expected number but got bool', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['settagstack(1, [1])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 2'])
  v9.CheckDefAndScriptFailure(['settagstack(1, {}, 2)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  assert_fails('settagstack(1, {}, "")', 'E962: Invalid action')
enddef

def Test_setwinvar()
  v9.CheckDefAndScriptFailure(['setwinvar("a", "b", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['setwinvar(1, 2, "c")'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  assert_fails('setwinvar(1, "", 10)', 'E461: Illegal variable name')
  assert_fails('setwinvar(0, "&rulerformat", true)', ['E928:', 'E928:'])
enddef

def Test_sha256()
  v9.CheckDefAndScriptFailure(['sha256(100)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['sha256(0zABCD)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1174: String required for argument 1'])
  assert_equal('ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad', sha256('abc'))
  assert_equal('e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855', sha256(''))
enddef

def Test_shiftwidth()
  v9.CheckDefAndScriptFailure(['shiftwidth("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_sign_define()
  v9.CheckDefAndScriptFailure(['sign_define({"a": 10})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_define({"a": 10}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_define("a", ["b"])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<string>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_sign_getdefined()
  v9.CheckDefAndScriptFailure(['sign_getdefined(["x"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_getdefined(2)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  sign_getdefined('')->assert_equal([])
enddef

def Test_sign_getplaced()
  v9.CheckDefAndScriptFailure(['sign_getplaced(["x"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_getplaced(1, ["a"])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<string>', 'E1206: Dictionary required for argument 2'])
  v9.CheckDefAndScriptFailure(['sign_getplaced("a", 1.1)'], ['E1013: Argument 2: type mismatch, expected dict<any> but got float', 'E1206: Dictionary required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['sign_getplaced(bufnr(), {lnum: ""})'], 'E1030: Using a String as a Number:')
  sign_getplaced('')->assert_equal([{signs: [], bufnr: bufnr()}])
enddef

def Test_sign_jump()
  v9.CheckDefAndScriptFailure(['sign_jump("a", "b", "c")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_jump(1, 2, 3)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['sign_jump(1, "b", true)'], ['E1013: Argument 3: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 3'])
enddef

def Test_sign_place()
  v9.CheckDefAndScriptFailure(['sign_place("a", "b", "c", "d")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_place(1, 2, "c", "d")'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['sign_place(1, "b", 3, "d")'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  v9.CheckDefAndScriptFailure(['sign_place(1, "b", "c", 1.1)'], ['E1013: Argument 4: type mismatch, expected string but got float', 'E1220: String or Number required for argument 4'])
  v9.CheckDefAndScriptFailure(['sign_place(1, "b", "c", "d", [1])'], ['E1013: Argument 5: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 5'])
  v9.CheckDefExecAndScriptFailure(['sign_place(0, "", "MySign", bufnr(), {lnum: ""})'], 'E1209: Invalid value for a line number: ""')
  assert_fails("sign_place(0, '', '', '')", 'E155:')
enddef

def Test_sign_placelist()
  v9.CheckDefAndScriptFailure(['sign_placelist("x")'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_placelist({"a": 10})'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<number>', 'E1211: List required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['sign_placelist([{"name": "MySign", "buffer": bufnr(), "lnum": ""}])'], 'E1209: Invalid value for a line number: ""')
  assert_fails('sign_placelist([{name: "MySign", buffer: "", lnum: 1}])', 'E155:')
enddef

def Test_sign_undefine()
  v9.CheckDefAndScriptFailure(['sign_undefine({})'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_undefine([1])'], ['E1013: Argument 1: type mismatch, expected list<string> but got list<number>', 'E155: Unknown sign:'])
enddef

def Test_sign_unplace()
  v9.CheckDefAndScriptFailure(['sign_unplace({"a": 10})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_unplace({"a": 10}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_unplace("a", ["b"])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<string>', 'E1206: Dictionary required for argument 2'])
enddef

def Test_sign_unplacelist()
  v9.CheckDefAndScriptFailure(['sign_unplacelist("x")'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['sign_unplacelist({"a": 10})'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<number>', 'E1211: List required for argument 1'])
enddef

def Test_simplify()
  v9.CheckDefAndScriptFailure(['simplify(100)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  call assert_equal('NonExistingFile', simplify('NonExistingFile'))
  simplify('')->assert_equal('')
enddef

def Test_slice()
  assert_equal('12345', slice('012345', 1))
  assert_equal('123', slice('012345', 1, 4))
  assert_equal('1234', slice('012345', 1, -1))
  assert_equal('1', slice('012345', 1, -4))
  assert_equal('', slice('012345', 1, -5))
  assert_equal('', slice('012345', 1, -6))

  assert_equal([1, 2, 3, 4, 5], slice(range(6), 1))
  assert_equal([1, 2, 3], slice(range(6), 1, 4))
  assert_equal([1, 2, 3, 4], slice(range(6), 1, -1))
  assert_equal([1], slice(range(6), 1, -4))
  assert_equal([], slice(range(6), 1, -5))
  assert_equal([], slice(range(6), 1, -6))

  var lds: list<dict<string>> = [{key: 'value'}]
  assert_equal(['val'], lds->slice(0, 1)->map((_, v) => 'val'))
  assert_equal(['val'], lds[ : ]->map((_, v) => 'val'))

  assert_equal(0z1122334455, slice(0z001122334455, 1))
  assert_equal(0z112233, slice(0z001122334455, 1, 4))
  assert_equal(0z11223344, slice(0z001122334455, 1, -1))
  assert_equal(0z11, slice(0z001122334455, 1, -4))
  assert_equal(0z, slice(0z001122334455, 1, -5))
  assert_equal(0z, slice(0z001122334455, 1, -6))
  v9.CheckDefAndScriptFailure(['slice({"a": 10}, 1)'], ['E1013: Argument 1: type mismatch, expected list<any> but got dict<number>', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['slice([1, 2, 3], "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['slice("abc", 1, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
enddef

def Test_spellsuggest()
  if !has('spell')
    CheckFeature spell
  else
    spellsuggest('marrch', 1, true)->assert_equal(['March'])
  endif
  v9.CheckDefAndScriptFailure(['spellsuggest(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['spellsuggest("a", "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['spellsuggest("a", 1, 0z01)'], ['E1013: Argument 3: type mismatch, expected bool but got blob', 'E1212: Bool required for argument 3'])
  spellsuggest('')->assert_equal([])
enddef

def Test_sound_playevent()
  CheckFeature sound
  v9.CheckDefAndScriptFailure(['sound_playevent(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
enddef

def Test_sound_playfile()
  CheckFeature sound
  v9.CheckDefAndScriptFailure(['sound_playfile(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
enddef

def Test_sound_stop()
  CheckFeature sound
  v9.CheckDefAndScriptFailure(['sound_stop("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_soundfold()
  v9.CheckDefAndScriptFailure(['soundfold(20)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal('abc', soundfold('abc'))
  assert_equal('', soundfold(''))
enddef

def Test_sort_return_type()
  var res: list<number>
  res = [1, 2, 3]->sort()
enddef

def Test_sort_argument()
  var lines =<< trim END
    var res = ['b', 'a', 'c']->sort('i')
    res->assert_equal(['a', 'b', 'c'])

    def Compare(a: number, b: number): number
      return a - b
    enddef
    var l = [3, 6, 7, 1, 8, 2, 4, 5]
    sort(l, Compare)
    assert_equal([1, 2, 3, 4, 5, 6, 7, 8], l)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      sort([1, 2, 3], (a: any, b: any) => 1)
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
      vim9script
      def SortedList(): list<number>
        var Lambda: func: number = (a, b): number => a - b
        var l = [3, 2, 1]
        return l->sort(Lambda)
      enddef
      SortedList()->assert_equal([1, 2, 3])
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_sort_const()
  var lines =<< trim END
      const l = [1, 2, 3, 4]
      sort(l)
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const list<number>')
enddef

def Test_sort_compare_func_fails()
  v9.CheckDefAndScriptFailure(['sort("a")'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['sort([1], "", [1])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])

  var lines =<< trim END
    vim9script
    echo ['a', 'b', 'c']->sort((a: number, b: number) => 0)
  END
  writefile(lines, 'Xbadsort', 'D')
  assert_fails('source Xbadsort', ['E1013:', 'E702:'])

  lines =<< trim END
      var l = [1, 2, 3]
      sort(l, (a: string, b: number) => 1)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?number): number but got func(string, number): number', 'E1013: Argument 1: type mismatch, expected string but got number'])

  lines =<< trim END
      var l = ['a', 'b', 'c']
      sort(l, (a: string, b: number) => 1)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?string, ?string): number but got func(string, number): number', 'E1013: Argument 2: type mismatch, expected number but got string'])

  lines =<< trim END
      sort([1, 2, 3], (a: number, b: number) => true)
  END
  v9.CheckDefAndScriptFailure(lines, ['E1013: Argument 2: type mismatch, expected func(?number, ?number): number but got func(number, number): bool', 'E1138: Using a Bool as a Number'])
enddef

def Test_spellbadword()
  v9.CheckDefAndScriptFailure(['spellbadword(100)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  spellbadword('good')->assert_equal(['', ''])
  spellbadword('')->assert_equal(['', ''])
enddef

def Test_split()
  split('  aa  bb  ', '\W\+', true)->assert_equal(['', 'aa', 'bb', ''])
  v9.CheckDefAndScriptFailure(['split(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['split("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['split("a", "b", 2)'], ['E1013: Argument 3: type mismatch, expected bool but got number', 'E1212: Bool required for argument 3'])
  split('')->assert_equal([])
  split('', '')->assert_equal([])
enddef

def Test_srand()
  var expected = srand()->len()->range()->map((_, _) => 'x')
  assert_equal(expected, srand()->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['srand("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  type(srand(100))->assert_equal(v:t_list)
enddef

def Test_state()
  v9.CheckDefAndScriptFailure(['state({})'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', 'E1174: String required for argument 1'])
  assert_equal('', state('a'))
enddef

def Test_str2float()
  str2float("1.00")->assert_equal(1.00)
  str2float("2e-2")->assert_equal(0.02)
  str2float('')->assert_equal(0.0)

  v9.CheckDefAndScriptFailure(['str2float(123)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
enddef

def Test_str2list()
  assert_equal(['x', 'x', 'x'], str2list('abc')->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['str2list(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['str2list("a", 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
  assert_equal([97], str2list('a'))
  assert_equal([97], str2list('a', 1))
  assert_equal([97], str2list('a', true))
  str2list('')->assert_equal([])
enddef

def Test_str2nr()
  str2nr("1'000'000", 10, true)->assert_equal(1000000)
  str2nr('')->assert_equal(0)

  v9.CheckDefAndScriptFailure(['str2nr(123)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['str2nr("123", "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['str2nr("123", 10, "x")'], ['E1013: Argument 3: type mismatch, expected bool but got string', 'E1212: Bool required for argument 3'])
enddef

def Test_strcharlen()
  v9.CheckDefAndScriptFailure(['strcharlen([1])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
  "abc"->strcharlen()->assert_equal(3)
  strcharlen(99)->assert_equal(2)
enddef

def Test_strcharpart()
  v9.CheckDefAndScriptFailure(['strcharpart(1, 2)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['strcharpart("a", "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['strcharpart("a", 1, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['strcharpart("a", 1, 1, 2)'], ['E1013: Argument 4: type mismatch, expected bool but got number', 'E1212: Bool required for argument 4'])
  strcharpart('', 0)->assert_equal('')
enddef

def Test_strchars()
  strchars("A\u20dd", true)->assert_equal(1)
  v9.CheckDefAndScriptFailure(['strchars(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['strchars("a", 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
  assert_equal(3, strchars('abc'))
  assert_equal(3, strchars('abc', 1))
  assert_equal(3, strchars('abc', true))
  strchars('')->assert_equal(0)
enddef

def Test_strdisplaywidth()
  v9.CheckDefAndScriptFailure(['strdisplaywidth(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['strdisplaywidth("a", "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  strdisplaywidth('')->assert_equal(0)
enddef

def Test_strftime()
  if exists('*strftime')
    v9.CheckDefAndScriptFailure(['strftime(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
    v9.CheckDefAndScriptFailure(['strftime("a", "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
    strftime('')->assert_equal('')
  endif
enddef

def Test_strgetchar()
  v9.CheckDefAndScriptFailure(['strgetchar(1, 1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['strgetchar("a", "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  strgetchar('', 0)->assert_equal(-1)
  strgetchar('', 1)->assert_equal(-1)
enddef

def Test_stridx()
  v9.CheckDefAndScriptFailure(['stridx([1], "b")'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['stridx("a", {})'], ['E1013: Argument 2: type mismatch, expected string but got dict<any>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['stridx("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  stridx('', '')->assert_equal(0)
  stridx('', 'a')->assert_equal(-1)
  stridx('a', '')->assert_equal(0)
enddef

def Test_strlen()
  v9.CheckDefAndScriptFailure(['strlen([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  "abc"->strlen()->assert_equal(3)
  strlen(99)->assert_equal(2)
enddef

def Test_strpart()
  v9.CheckDefAndScriptFailure(['strpart(1, 2)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['strpart("a", "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['strpart("a", 1, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  v9.CheckDefAndScriptFailure(['strpart("a", 1, 1, 2)'], ['E1013: Argument 4: type mismatch, expected bool but got number', 'E1212: Bool required for argument 4'])
  strpart('', 0)->assert_equal('')
enddef

def Test_strptime()
  CheckFunction strptime
  CheckNotBSD
  if exists_compiled('*strptime')
    v9.CheckDefAndScriptFailure(['strptime(10, "2021")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
    v9.CheckDefAndScriptFailure(['strptime("%Y", 2021)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
    assert_notequal(0, strptime('%Y', '2021'))
    # This fails on BSD 14 and returns
    # -2209078800 instead of 0
    assert_equal(0, strptime('%Y', ''))
  endif
enddef

def Test_strridx()
  v9.CheckDefAndScriptFailure(['strridx([1], "b")'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['strridx("a", {})'], ['E1013: Argument 2: type mismatch, expected string but got dict<any>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['strridx("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  strridx('', '')->assert_equal(0)
  strridx('', 'a')->assert_equal(-1)
  strridx('a', '')->assert_equal(1)
enddef

def Test_strtrans()
  v9.CheckDefAndScriptFailure(['strtrans(20)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal('abc', strtrans('abc'))
  strtrans('')->assert_equal('')
enddef

def Test_strutf16len()
  v9.CheckDefAndScriptFailure(['strutf16len([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['strutf16len("a", "")'], ['E1013: Argument 2: type mismatch, expected bool but got string', 'E1212: Bool required for argument 2'])
  ""->strutf16len()->assert_equal(0)
  '-ą́-ą́'->strutf16len(true)->assert_equal(8)
  '-ą́-ą́'->strutf16len(false)->assert_equal(4)
enddef

def Test_strwidth()
  v9.CheckDefAndScriptFailure(['strwidth(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal(4, strwidth('abcd'))
  strwidth('')->assert_equal(0)
enddef

def Test_submatch()
  var pat = 'A\(.\)\(.\)\(.\)\(.\)\(.\)\(.\)\(.\)\(.\)\(.\)'
  var Rep = () => range(10)->mapnew((_, v) => submatch(v, true))->string()
  var actual = substitute('A123456789', pat, Rep, '')
  var expected = "[['A123456789'], ['1'], ['2'], ['3'], ['4'], ['5'], ['6'], ['7'], ['8'], ['9']]"
  actual->assert_equal(expected)
  v9.CheckDefAndScriptFailure(['submatch("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['submatch(1, "a")'], ['E1013: Argument 2: type mismatch, expected bool but got string', 'E1212: Bool required for argument 2'])
enddef

def Test_substitute()
  var res = substitute('A1234', '\d', 'X', '')
  assert_equal('AX234', res)

  if has('job')
    assert_fails('"text"->substitute(".*", () => test_null_job(), "")', 'E908: Using an invalid value as a String: job')
    assert_fails('"text"->substitute(".*", () => test_null_channel(), "")', 'E908: Using an invalid value as a String: channel')
  endif
  v9.CheckDefAndScriptFailure(['substitute(1, "b", "1", "d")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['substitute("a", 2, "1", "d")'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['substitute("a", "b", "1", 4)'], ['E1013: Argument 4: type mismatch, expected string but got number', 'E1174: String required for argument 4'])
  substitute('', '', '', '')->assert_equal('')

  var lines =<< trim END
    assert_equal("4", substitute("3", '\d', '\=str2nr(submatch(0)) + 1', 'g'))
  END
  v9.CheckDefAndScriptSuccess(lines)

  lines =<< trim END
    assert_equal("4", substitute("3", '\d', '\="text" x', 'g'))
  END
  v9.CheckDefAndScriptFailure(lines, 'E488: Trailing characters: x')
enddef

def Test_swapinfo()
  v9.CheckDefAndScriptFailure(['swapinfo({})'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', 'E1174: String required for argument 1'])
  call swapinfo('x')->assert_equal({error: 'Cannot open file'})
  call swapinfo('')->assert_equal({error: 'Cannot open file'})
enddef

def Test_swapname()
  v9.CheckDefAndScriptFailure(['swapname([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  assert_fails('swapname("NonExistingBuf")', 'E94:')
enddef

def Test_synID()
  new
  setline(1, "text")
  synID(1, 1, true)->assert_equal(0)
  bwipe!
  v9.CheckDefAndScriptFailure(['synID(0z10, 1, true)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['synID("a", true, false)'], ['E1013: Argument 2: type mismatch, expected number but got bool', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['synID(1, 1, 2)'], ['E1013: Argument 3: type mismatch, expected bool but got number', 'E1212: Bool required for argument 3'])
  v9.CheckDefExecAndScriptFailure(['synID("", 10, true)'], 'E1209: Invalid value for a line number')
enddef

def Test_synIDattr()
  v9.CheckDefAndScriptFailure(['synIDattr("a", "b")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['synIDattr(1, 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['synIDattr(1, "b", 3)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  synIDattr(1, '', '')->assert_equal('')
enddef

def Test_synIDtrans()
  v9.CheckDefAndScriptFailure(['synIDtrans("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_synconcealed()
  v9.CheckDefAndScriptFailure(['synconcealed(0z10, 1)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['synconcealed(1, "a")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  if has('conceal')
    v9.CheckDefExecAndScriptFailure(['synconcealed("", 4)'], 'E1209: Invalid value for a line number')
  endif
enddef

def Test_synstack()
  v9.CheckDefAndScriptFailure(['synstack(0z10, 1)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['synstack(1, "a")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['synstack("", 4)'], 'E1209: Invalid value for a line number')
enddef

def Test_system()
  v9.CheckDefAndScriptFailure(['system(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['system("a", {})'], ['E1013: Argument 2: type mismatch, expected string but got dict<any>', 'E1224: String, Number or List required for argument 2'])
  assert_equal("123\n", system('echo 123'))
enddef

def Test_systemlist()
  v9.CheckDefAndScriptFailure(['systemlist(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['systemlist("a", {})'], ['E1013: Argument 2: type mismatch, expected string but got dict<any>', 'E1224: String, Number or List required for argument 2'])
  if has('win32')
    call assert_equal(["123\r"], systemlist('echo 123'))
  else
    call assert_equal(['123'], systemlist('echo 123'))
  endif
enddef

def Test_tabpagebuflist()
  v9.CheckDefAndScriptFailure(['tabpagebuflist("t")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  assert_equal([bufnr('')], tabpagebuflist())
  assert_equal([bufnr('')], tabpagebuflist(1))
  assert_equal(['x'], tabpagebuflist()->map((_, _) => 'x'))
enddef

def Test_tabpagenr()
  v9.CheckDefAndScriptFailure(['tabpagenr(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['tabpagenr("")'], 'E15: Invalid expression')
  assert_equal(1, tabpagenr('$'))
  assert_equal(1, tabpagenr())
enddef

def Test_tabpagewinnr()
  v9.CheckDefAndScriptFailure(['tabpagewinnr("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['tabpagewinnr(1, 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['tabpagewinnr(1, "")'], 'E15: Invalid expression')
enddef

def Test_taglist()
  v9.CheckDefAndScriptFailure(['taglist([1])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['taglist("a", [2])'], ['E1013: Argument 2: type mismatch, expected string but got list<number>', 'E1174: String required for argument 2'])
  taglist('')->assert_equal(0)
  taglist('', '')->assert_equal(0)
enddef

def Test_term_dumpload()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_dumpload({"a": 10})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_dumpload({"a": 10}, "b")'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_dumpload("a", ["b"])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<string>', 'E1206: Dictionary required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['term_dumpload("")'], 'E485: Can''t read file')
enddef

def Test_term_dumpdiff()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_dumpdiff(1, "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_dumpdiff("a", 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['term_dumpdiff("a", "b", [1])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])
  v9.CheckDefExecAndScriptFailure(['term_dumpdiff("", "")'], 'E485: Can''t read file')
enddef

def Test_term_dumpwrite()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_dumpwrite(true, "b")'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_dumpwrite(1, 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['term_dumpwrite("a", "b", [1])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])
enddef

def Test_term_getaltscreen()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_getaltscreen(true)'], ['E1013: Argument 1: type mismatch, expected string but got bool', 'E1220: String or Number required for argument 1'])
enddef

def Test_term_getansicolors()
  CheckRunVimInTerminal
  CheckFeature termguicolors
  v9.CheckDefAndScriptFailure(['term_getansicolors(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1220: String or Number required for argument 1'])
enddef

def Test_term_getattr()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_getattr("x", "a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_getattr(1, 2)'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
enddef

def Test_term_getcursor()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_getcursor({"a": 10})'], ['E1013: Argument 1: type mismatch, expected string but got dict<number>', 'E1220: String or Number required for argument 1'])
enddef

def Test_term_getjob()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_getjob(0z10)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1220: String or Number required for argument 1'])
enddef

def Test_term_getline()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_getline(1.1, 1)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_getline(1, 1.1)'], ['E1013: Argument 2: type mismatch, expected string but got float', 'E1220: String or Number required for argument 2'])
enddef

def Test_term_getscrolled()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_getscrolled(1.1)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1220: String or Number required for argument 1'])
enddef

def Test_term_getsize()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_getsize(1.1)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1220: String or Number required for argument 1'])
enddef

def Test_term_getstatus()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_getstatus(1.1)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1220: String or Number required for argument 1'])
enddef

def Test_term_gettitle()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_gettitle(1.1)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1220: String or Number required for argument 1'])
enddef

def Test_term_gettty()
  if !has('terminal')
    CheckFeature terminal
  else
    var buf = g:Run_shell_in_terminal({})
    term_gettty(buf, true)->assert_notequal('')
    g:StopShellInTerminal(buf)
  endif
  v9.CheckDefAndScriptFailure(['term_gettty([1])'], ['E1013: Argument 1: type mismatch, expected string but got list<number>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_gettty(1, 2)'], ['E1013: Argument 2: type mismatch, expected bool but got number', 'E1212: Bool required for argument 2'])
enddef

def Test_term_scrape()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_scrape(1.1, 1)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_scrape(1, 1.1)'], ['E1013: Argument 2: type mismatch, expected string but got float', 'E1220: String or Number required for argument 2'])
enddef

def Test_term_sendkeys()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_sendkeys([], "p")'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_sendkeys(1, [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1174: String required for argument 2'])
enddef

def Test_term_setansicolors()
  CheckRunVimInTerminal

  if has('termguicolors') || has('gui')
    v9.CheckDefAndScriptFailure(['term_setansicolors([], "p")'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
    v9.CheckDefAndScriptFailure(['term_setansicolors(10, {})'], ['E1013: Argument 2: type mismatch, expected list<any> but got dict<any>', 'E1211: List required for argument 2'])
  else
    throw 'Skipped: Only works with termguicolors or gui feature'
  endif
enddef

def Test_term_setapi()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_setapi([], "p")'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_setapi(1, [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1174: String required for argument 2'])
enddef

def Test_term_setkill()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_setkill([], "p")'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_setkill(1, [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1174: String required for argument 2'])
enddef

def Test_term_setrestore()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_setrestore([], "p")'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_setrestore(1, [])'], ['E1013: Argument 2: type mismatch, expected string but got list<any>', 'E1174: String required for argument 2'])
enddef

def Test_term_setsize()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_setsize(1.1, 2, 3)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_setsize(1, "2", 3)'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['term_setsize(1, 2, "3")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
enddef

def Test_term_start()
  if !has('terminal')
    CheckFeature terminal
  else
    botright new
    var winnr = winnr()
    term_start(&shell, {curwin: true})
    winnr()->assert_equal(winnr)
    bwipe!
  endif
  v9.CheckDefAndScriptFailure(['term_start({})'], ['E1013: Argument 1: type mismatch, expected string but got dict<any>', 'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_start([], [])'], ['E1013: Argument 2: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 2'])
  v9.CheckDefAndScriptFailure(['term_start("", "")'], ['E1013: Argument 2: type mismatch, expected dict<any> but got string', 'E1206: Dictionary required for argument 2'])
  v9.CheckDefExecAndScriptFailure(['term_start("")'], 'E474: Invalid argument')
enddef

def Test_term_wait()
  CheckRunVimInTerminal
  v9.CheckDefAndScriptFailure(['term_wait(0z10, 1)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1220: String or Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['term_wait(1, "a")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_test_alloc_fail()
  v9.CheckDefAndScriptFailure(['test_alloc_fail("a", 10, 20)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['test_alloc_fail(10, "b", 20)'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['test_alloc_fail(10, 20, "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
enddef

def Test_test_feedinput()
  v9.CheckDefAndScriptFailure(['test_feedinput(test_void())'], ['E1013: Argument 1: type mismatch, expected string but got void', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['test_feedinput(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
enddef

def Test_test_getvalue()
  v9.CheckDefAndScriptFailure(['test_getvalue(1.1)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1174: String required for argument 1'])
enddef

def Test_test_gui_event()
  CheckGui
  v9.CheckDefAndScriptFailure(['test_gui_event([], {})'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['test_gui_event("abc", 1)'], ['E1013: Argument 2: type mismatch, expected dict<any> but got number', 'E1206: Dictionary required for argument 2'])
enddef

def Test_test_ignore_error()
  v9.CheckDefAndScriptFailure(['test_ignore_error([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1174: String required for argument 1'])
  test_ignore_error('RESET')
enddef

def Test_test_option_not_set()
  v9.CheckDefAndScriptFailure(['test_option_not_set([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1174: String required for argument 1'])
enddef

def Test_test_override()
  v9.CheckDefAndScriptFailure(['test_override(1, 1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['test_override("a", "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_test_setmouse()
  v9.CheckDefAndScriptFailure(['test_setmouse("a", 10)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['test_setmouse(10, "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_test_settime()
  v9.CheckDefAndScriptFailure(['test_settime([1])'], ['E1013: Argument 1: type mismatch, expected number but got list<number>', 'E1210: Number required for argument 1'])
enddef

def Test_test_srand_seed()
  v9.CheckDefAndScriptFailure(['test_srand_seed([1])'], ['E1013: Argument 1: type mismatch, expected number but got list<number>', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['test_srand_seed("10")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_timer_info()
  v9.CheckDefAndScriptFailure(['timer_info("id")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  assert_equal([], timer_info(100))
  assert_equal([], timer_info()->filter((_, t) => t.callback->string() !~ 'TestTimeout'))
enddef

def Test_timer_pause()
  v9.CheckDefAndScriptFailure(['timer_pause("x", 1)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['timer_pause(1, "a")'], ['E1013: Argument 2: type mismatch, expected bool but got string', 'E1212: Bool required for argument 2'])
enddef

def Test_timer_paused()
  var id = timer_start(50, () => 0)
  timer_pause(id, true)
  var info = timer_info(id)
  info[0]['paused']->assert_equal(1)
  timer_stop(id)
enddef

def Test_timer_start()
  v9.CheckDefAndScriptFailure(['timer_start("a", "1")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['timer_start(1, "1", [1])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])
  v9.CheckDefExecAndScriptFailure(['timer_start(100, 0)'], 'E921:')
  v9.CheckDefExecAndScriptFailure(['timer_start(100, "")'], 'E921:')
enddef

def Test_timer_stop()
  v9.CheckDefAndScriptFailure(['timer_stop("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  assert_equal(0, timer_stop(100))
enddef

def Test_tolower()
  v9.CheckDefAndScriptFailure(['tolower(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  tolower('')->assert_equal('')
enddef

def Test_toupper()
  v9.CheckDefAndScriptFailure(['toupper(1)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  toupper('')->assert_equal('')
enddef

def Test_tr()
  v9.CheckDefAndScriptFailure(['tr(1, "a", "b")'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['tr("a", 1, "b")'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['tr("a", "a", 1)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
  tr('', '', '')->assert_equal('')
  tr('ab', '', '')->assert_equal('ab')
  assert_fails("tr('ab', 'ab', '')", 'E475:')
  assert_fails("tr('ab', '', 'AB')", 'E475:')
enddef

def Test_trim()
  v9.CheckDefAndScriptFailure(['trim(["a"])'], ['E1013: Argument 1: type mismatch, expected string but got list<string>', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['trim("a", ["b"])'], ['E1013: Argument 2: type mismatch, expected string but got list<string>', 'E1174: String required for argument 2'])
  v9.CheckDefAndScriptFailure(['trim("a", "b", "c")'], ['E1013: Argument 3: type mismatch, expected number but got string', 'E1210: Number required for argument 3'])
  trim('')->assert_equal('')
  trim('', '')->assert_equal('')
enddef

def Test_typename()
  assert_equal('func([unknown], [unknown]): float', typename(function('pow')))
  assert_equal('func(...): unknown', test_null_partial()->typename())
  assert_equal('list<any>', test_null_list()->typename())
  assert_equal('dict<any>', test_null_dict()->typename())
  if has('job')
    assert_equal('job', test_null_job()->typename())
  endif
  if has('channel')
    assert_equal('channel', test_null_channel()->typename())
  endif
enddef

def Test_undofile()
  v9.CheckDefAndScriptFailure(['undofile(10)'], ['E1013: Argument 1: type mismatch, expected string but got number', 'E1174: String required for argument 1'])
  assert_equal('.abc.un~', fnamemodify(undofile('abc'), ':t'))
  undofile('')->assert_equal('')
enddef

def Test_uniq()
  v9.CheckDefAndScriptFailure(['uniq("a")'], ['E1013: Argument 1: type mismatch, expected list<any> but got string', 'E1211: List required for argument 1'])
  v9.CheckDefAndScriptFailure(['uniq([1], "", [1])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<number>', 'E1206: Dictionary required for argument 3'])

  v9.CheckDefFailure(['var l: list<number> = uniq(["a", "b"])'], 'E1012: Type mismatch; expected list<number> but got list<string>')
enddef

def Test_utf16idx()
  v9.CheckDefAndScriptFailure(['utf16idx(0z10, 1)'], ['E1013: Argument 1: type mismatch, expected string but got blob', 'E1174: String required for argument 1'])
  v9.CheckDefAndScriptFailure(['utf16idx("a", "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['utf16idx("a", 1, "")'], ['E1013: Argument 3: type mismatch, expected bool but got string', 'E1212: Bool required for argument 3'])
  v9.CheckDefAndScriptFailure(['utf16idx("a", 1, 0, "")'], ['E1013: Argument 4: type mismatch, expected bool but got string', 'E1212: Bool required for argument 4'])
  utf16idx('', 0)->assert_equal(0)
  utf16idx('', 1)->assert_equal(-1)
enddef

def Test_uniq_const()
  var lines =<< trim END
      const l = [1, 2, 3, 4]
      uniq(l)
  END
  v9.CheckDefFailure(lines, 'E1307: Argument 1: Trying to modify a const list<number>')
enddef

def Test_values()
  v9.CheckDefAndScriptFailure(['values([])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 1'])
  assert_equal([], {}->values())
  assert_equal(['sun'], {star: 'sun'}->values())

  # the return type of values() is list<member>
  var lines =<< trim END
      vim9script

      class Foo
        var val: number
        def Add()
          echo this.val
        enddef
      endclass

      def Process(FooDict: dict<Foo>)
        for foo in values(FooDict)
          foo.Add()
        endfor
      enddef

      disas Process

      var D = {'x': Foo.new(22)}

      Process(D)
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_virtcol()
  v9.CheckDefAndScriptFailure(['virtcol(1.1)'], [
    'E1013: Argument 1: type mismatch, expected string but got float',
    'E1222: String or List required for argument 1'])
  v9.CheckDefAndScriptFailure(['virtcol(".", "a")'], [
    'E1013: Argument 2: type mismatch, expected bool but got string',
    'E1212: Bool required for argument 2'])
  v9.CheckDefAndScriptFailure(['virtcol(".", v:true, [])'], [
    'E1013: Argument 3: type mismatch, expected number but got list',
    'E1210: Number required for argument 3'])
  v9.CheckDefExecAndScriptFailure(['virtcol("")'],
    'E1209: Invalid value for a line number')
  new
  setline(1, ['abcde和平fgh'])
  cursor(1, 4)
  assert_equal(4, virtcol('.'))
  assert_equal([4, 4], virtcol('.', 1))
  cursor(1, 6)
  assert_equal([6, 7], virtcol('.', 1))
  assert_equal(4, virtcol([1, 4]))
  assert_equal(13, virtcol([1, '$']))
  assert_equal(0, virtcol([10, '$']))
  bw!
enddef

def Test_visualmode()
  v9.CheckDefAndScriptFailure(['visualmode("1")'], ['E1013: Argument 1: type mismatch, expected bool but got string', 'E1212: Bool required for argument 1'])
  v9.CheckDefAndScriptFailure(['visualmode(2)'], ['E1013: Argument 1: type mismatch, expected bool but got number', 'E1212: Bool required for argument 1'])
enddef

def Test_win_execute()
  assert_equal("\n" .. winnr(), win_execute(win_getid(), 'echo winnr()'))
  assert_equal("\n" .. winnr(), 'echo winnr()'->win_execute(win_getid()))
  assert_equal("\n" .. winnr(), win_execute(win_getid(), 'echo winnr()', 'silent'))
  assert_equal('', win_execute(342343, 'echo winnr()'))
  v9.CheckDefAndScriptFailure(['win_execute("a", "b", "c")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['win_execute(1, 2, "c")'], ['E1013: Argument 2: type mismatch, expected string but got number', 'E1222: String or List required for argument 2'])
  v9.CheckDefAndScriptFailure(['win_execute(1, "b", 3)'], ['E1013: Argument 3: type mismatch, expected string but got number', 'E1174: String required for argument 3'])
enddef

def Test_win_findbuf()
  v9.CheckDefAndScriptFailure(['win_findbuf("a")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  assert_equal([], win_findbuf(1000))
  assert_equal([win_getid()], win_findbuf(bufnr('')))
enddef

def Test_win_getid()
  v9.CheckDefAndScriptFailure(['win_getid(".")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['win_getid(1, ".")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  assert_equal(win_getid(), win_getid(1, 1))
enddef

def Test_win_gettype()
  v9.CheckDefAndScriptFailure(['win_gettype("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_win_gotoid()
  v9.CheckDefAndScriptFailure(['win_gotoid("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

func Test_win_gotoid_in_mapping()
  CheckScreendump

  " requires a working clipboard and this doesn't work on MacOS
  if has('clipboard_working') && !has('mac')
    let @* = 'foo'
    let lines =<< trim END
        set cmdheight=2
        func On_click()
          call win_gotoid(getmousepos().winid)
          execute "norm! \<LeftMouse>"
        endfunc
        noremap <LeftMouse> <Cmd>call On_click()<CR>

        autocmd SafeState * echo 'reg = "' .. @* .. '"'

        call setline(1, range(20))
        set nomodified
        botright new
        call setline(1, range(21, 40))
        set nomodified

        func Click()
          map <silent> <F3> :call test_setmouse(3, 1)<CR>
	  call feedkeys("\<F3>\<LeftMouse>\<LeftRelease>", "xt")
        endfunc
    END
    call writefile(lines, 'Xgotoscript', 'D')
    let buf = RunVimInTerminal('-S Xgotoscript', #{rows: 15, wait_for_ruler: 0})
    " wait longer here, since we didn't wait for the ruler
    call VerifyScreenDump(buf, 'Test_win_gotoid_1', #{wait: 3000})
    call term_sendkeys(buf, "3Gvl")
    call VerifyScreenDump(buf, 'Test_win_gotoid_2', {})

    call term_sendkeys(buf, ":call Click()\<CR>")
    call VerifyScreenDump(buf, 'Test_win_gotoid_3', {})

    call StopVimInTerminal(buf)
  endif
endfunc

def Test_win_id2tabwin()
  v9.CheckDefAndScriptFailure(['win_id2tabwin("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_win_id2win()
  v9.CheckDefAndScriptFailure(['win_id2win("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_win_screenpos()
  assert_equal(['x', 'x'], win_screenpos(1)->map((_, _) => 'x'))

  v9.CheckDefAndScriptFailure(['win_screenpos("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_win_splitmove()
  split
  win_splitmove(1, 2, {vertical: true, rightbelow: true})
  close
  v9.CheckDefAndScriptFailure(['win_splitmove("a", 2)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['win_splitmove(1, "b")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
  v9.CheckDefAndScriptFailure(['win_splitmove(1, 2, [])'], ['E1013: Argument 3: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 3'])
enddef

def Test_winbufnr()
  v9.CheckDefAndScriptFailure(['winbufnr("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_winheight()
  v9.CheckDefAndScriptFailure(['winheight("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_winlayout()
  v9.CheckDefAndScriptFailure(['winlayout("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_winnr()
  v9.CheckDefAndScriptFailure(['winnr([])'], ['E1013: Argument 1: type mismatch, expected string but got list<any>', 'E1174: String required for argument 1'])
  v9.CheckDefExecAndScriptFailure(['winnr("")'], 'E15: Invalid expression')
  assert_equal(1, winnr())
  assert_equal(1, winnr('$'))
enddef

def Test_winrestcmd()
  split
  var cmd = winrestcmd()
  wincmd _
  exe cmd
  assert_equal(cmd, winrestcmd())
  close
enddef

def Test_winrestview()
  v9.CheckDefAndScriptFailure(['winrestview([])'], ['E1013: Argument 1: type mismatch, expected dict<any> but got list<any>', 'E1206: Dictionary required for argument 1'])
  :%d _
  setline(1, 'Hello World')
  winrestview({lnum: 1, col: 6})
  assert_equal([1, 7], [line('.'), col('.')])
enddef

def Test_winsaveview()
  var view: dict<number> = winsaveview()

  var lines =<< trim END
      var view: list<number> = winsaveview()
  END
  v9.CheckDefAndScriptFailure(lines, 'E1012: Type mismatch; expected list<number> but got dict<number>', 1)
enddef

def Test_winwidth()
  v9.CheckDefAndScriptFailure(['winwidth("x")'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
enddef

def Test_xor()
  v9.CheckDefAndScriptFailure(['xor("x", 0x2)'], ['E1013: Argument 1: type mismatch, expected number but got string', 'E1210: Number required for argument 1'])
  v9.CheckDefAndScriptFailure(['xor(0x1, "x")'], ['E1013: Argument 2: type mismatch, expected number but got string', 'E1210: Number required for argument 2'])
enddef

def Test_writefile()
  v9.CheckDefExecAndScriptFailure(['writefile(["a"], "")'], 'E482: Can''t create file <empty>')
enddef

def Test_passing_type_to_builtin()
  # type, typename, string, instanceof are allowed type argument
  var lines =<< trim END
    vim9script
    class C
    endclass
    type T = number
    type U = C
    var x: any
    x = type(C)
    x = type(T)
    x = typename(C)
    x = typename(T)
    x = string(C)
    x = string(T)
    x = instanceof(C.new(), U, C)
  END
  v9.CheckScriptSuccess(lines)

  # check argument to add at script level
  # Note: add() is special cased in compile_call in vim9expr
  lines =<< trim END
    vim9script
    class C
    endclass
    add([], C)
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # check argument to add in :def
  lines =<< trim END
    vim9script
    class C
    endclass
    def F()
      add([], C)
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # check member call argument to add at script level
  lines =<< trim END
    vim9script
    class C
    endclass
    []->add(C)
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # check member call argument to add in :def
  lines =<< trim END
    vim9script
    class C
    endclass
    def F()
      []->add(C)
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # Try "empty()" builtin
  # check argument to empty at script level
  lines =<< trim END
    vim9script
    class C
    endclass
    empty(C)
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # check argument to empty in :def
  lines =<< trim END
    vim9script
    class C
    endclass
    def F()
      empty(C)
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # check member call argument to empty at script level
  lines =<< trim END
    vim9script
    class C
    endclass
    C->empty()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # check member call argument to empty in :def
  lines =<< trim END
    vim9script
    class C
    endclass
    def F()
      C->empty()
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # Try "abs()" builtin
  # check argument to abs at script level
  lines =<< trim END
    vim9script
    class C
    endclass
    abs(C)
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # check argument to abs in :def
  lines =<< trim END
    vim9script
    class C
    endclass
    def F()
      abs(C)
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # check member call argument to abs at script level
  lines =<< trim END
    vim9script
    class C
    endclass
    C->abs()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')

  # check member call argument to abs in :def
  lines =<< trim END
    vim9script
    class C
    endclass
    def F()
      C->abs()
    enddef
    F()
  END
  v9.CheckScriptFailure(lines, 'E1405: Class "C" cannot be used as a value')
enddef

" vim: ts=8 sw=2 sts=2 expandtab tw=80 fdm=marker
