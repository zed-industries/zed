" Test import/export of the Vim9 script language.
" Also the autoload mechanism.

source check.vim
source term_util.vim
import './vim9.vim' as v9

let s:export_script_lines =<< trim END
  vim9script
  var name: string = 'bob'
  def Concat(arg: string): string
    return name .. arg
  enddef
  g:result = Concat('bie')
  g:localname = name

  export const CONST = 1234
  export var exported = 9876
  export var exp_name = 'John'
  export def Exported(): string
    return 'Exported'
  enddef
  export def ExportedValue(): number
    return exported
  enddef
  export def ExportedInc()
    exported += 5
  enddef
  export final theList = [1]
  export def AddSome(s: string): string
    return s .. 'some'
  enddef
  export var AddRef = AddSome
END

def s:Undo_export_script_lines()
  unlet g:result
  unlet g:localname
enddef

def Test_vim9_import_export()
  writefile(s:export_script_lines, 'Xexport.vim', 'D')
  var import_script_lines =<< trim END
    vim9script
    var dir = './'
    var ext = ".vim"
    import dir .. 'Xexport' .. ext as expo

    g:exported1 = expo.exported
    expo.exported += 3
    g:exported2 = expo.exported
    g:exported3 = expo.ExportedValue()

    expo.ExportedInc()
    g:exported_i1 = expo.exported
    g:exported_i2 = expo.ExportedValue()

    expo.exported = 11
    g:exported_s1 = expo.exported
    g:exported_s2 = expo.ExportedValue()

    g:imported_func = expo.Exported()

    def GetExported(): string
      var local_dict = {ref: expo.Exported}
      return local_dict.ref()
    enddef
    g:funcref_result = GetExported()

    def GetName(): string
      return expo.exp_name .. 'son'
    enddef
    g:long_name = GetName()

    g:imported_name = expo.exp_name
    expo.exp_name ..= ' Doe'
    expo.exp_name = expo.exp_name .. ' Maar'
    g:imported_name_appended = expo.exp_name
    g:exported_later = expo.exported

    expo.theList->add(2)
    assert_equal([1, 2], expo.theList)

    assert_equal('andthensome', 'andthen'->expo.AddSome())
    assert_equal('awesome', 'awe'->expo.AddRef())
  END
  writefile(import_script_lines, 'Ximport.vim', 'D')
  source Ximport.vim

  assert_equal('bobbie', g:result)
  assert_equal('bob', g:localname)
  assert_equal(9876, g:exported1)
  assert_equal(9879, g:exported2)
  assert_equal(9879, g:exported3)

  assert_equal(9884, g:exported_i1)
  assert_equal(9884, g:exported_i2)

  assert_equal(11, g:exported_s1)
  assert_equal(11, g:exported_s2)
  assert_equal(11, g:exported_later)

  assert_equal('Exported', g:imported_func)
  assert_equal('Exported', g:funcref_result)
  assert_equal('John', g:imported_name)
  assert_equal('Johnson', g:long_name)
  assert_equal('John Doe Maar', g:imported_name_appended)
  assert_false(exists('g:name'))

  Undo_export_script_lines()
  unlet g:exported1
  unlet g:exported2
  unlet g:exported3
  unlet g:exported_i1
  unlet g:exported_i2
  unlet g:exported_later
  unlet g:imported_func
  unlet g:imported_name g:long_name g:imported_name_appended
  delete('Ximport.vim')

  # similar, with line breaks
  var import_line_break_script_lines =<< trim END
    vim9script
    import './Xexport.vim'
        as expo
    g:exported = expo.exported
    expo.exported += 7
    g:exported_added = expo.exported
    g:imported_func = expo.Exported()
  END
  writefile(import_line_break_script_lines, 'Ximport_lbr.vim')
  source Ximport_lbr.vim

  assert_equal(11, g:exported)
  assert_equal(18, g:exported_added)
  assert_equal('Exported', g:imported_func)

  # exported script not sourced again
  assert_false(exists('g:result'))
  unlet g:exported
  unlet g:exported_added
  unlet g:imported_func
  delete('Ximport_lbr.vim')

  var import_shadows_cmdmod_lines =<< trim END
      vim9script
      import './Xexport.vim' as vim9
      vim9.exp_name = 'Shadow'
      assert_equal('Shadow', vim9.exp_name)
  END
  v9.CheckScriptSuccess(import_shadows_cmdmod_lines)

  var line_break_before_dot =<< trim END
    vim9script
    import './Xexport.vim' as expo
    g:exported = expo
                  .exported
  END
  writefile(line_break_before_dot, 'Ximport_lbr_before_dot.vim')
  assert_fails('source Ximport_lbr_before_dot.vim', 'E1060:', '', 3)
  delete('Ximport_lbr_before_dot.vim')

  var line_break_after_dot =<< trim END
    vim9script
    import './Xexport.vim' as expo
    g:exported = expo.
                  exported
  END
  writefile(line_break_after_dot, 'Ximport_lbr_after_dot.vim')
  assert_fails('source Ximport_lbr_after_dot.vim', 'E1074:', '', 3)
  delete('Ximport_lbr_after_dot.vim')

  var import_star_as_lines =<< trim END
    vim9script
    import './Xexport.vim' as Export
    def UseExport()
      g:exported_def = Export.exported
    enddef
    g:exported_script = Export.exported
    assert_equal(1, exists('Export.exported'))
    assert_equal(0, exists('Export.notexported'))
    UseExport()
  END
  writefile(import_star_as_lines, 'Ximport.vim')
  source Ximport.vim

  assert_equal(18, g:exported_def)
  assert_equal(18, g:exported_script)
  unlet g:exported_def
  unlet g:exported_script

  var import_star_as_lines_no_dot =<< trim END
    vim9script
    import './Xexport.vim' as Export
    def Func()
      var dummy = 1
      var imported = Export + dummy
    enddef
    defcompile
  END
  writefile(import_star_as_lines_no_dot, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1060:', '', 2, 'Func')

  var import_star_as_lines_dot_space =<< trim END
    vim9script
    import './Xexport.vim' as Export
    def Func()
      var imported = Export . exported
    enddef
    defcompile
  END
  writefile(import_star_as_lines_dot_space, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1074:', '', 1, 'Func')

  writefile(s:export_script_lines, 'Xexport2.vim')
  var import_as_duplicated =<< trim END
    vim9script
    import './Xexport.vim' as expo
    import './Xexport2.vim' as expo
  END
  writefile(import_as_duplicated, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1073:', '', 3, 'Ximport.vim')
  delete('Xexport2.vim')

  var import_star_as_lines_script_no_dot =<< trim END
    vim9script
    import './Xexport.vim' as Export
    g:imported_script = Export exported
  END
  writefile(import_star_as_lines_script_no_dot, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1060: Expected dot after name: Export exported')

  var import_star_as_lines_script_space_after_dot =<< trim END
    vim9script
    import './Xexport.vim' as Export
    g:imported_script = Export. exported
  END
  writefile(import_star_as_lines_script_space_after_dot, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1074:')

  var import_star_as_lines_missing_name =<< trim END
    vim9script
    import './Xexport.vim' as Export
    def Func()
      var imported = Export.
    enddef
    defcompile
  END
  writefile(import_star_as_lines_missing_name, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1048:', '', 1, 'Func')

  var import_star_as_lbr_lines =<< trim END
    vim9script
    import './Xexport.vim'
        as Export
    def UseExport()
      g:exported = Export.exported
    enddef
    UseExport()
  END
  writefile(import_star_as_lbr_lines, 'Ximport.vim')
  source Ximport.vim
  assert_equal(18, g:exported)
  unlet g:exported

  # try to use something that exists but is not exported
  var import_not_exported_lines =<< trim END
    vim9script
    import './Xexport.vim' as expo
    echo expo.name
  END
  writefile(import_not_exported_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1049:', '', 3, 'Ximport.vim')

  # try to import something that is already defined
  var import_already_defined =<< trim END
    vim9script
    var exported = 'something'
    import './Xexport.vim' as exported
  END
  writefile(import_already_defined, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1054:', '', 3, 'Ximport.vim')

  # try changing an imported const
  var import_assign_to_const =<< trim END
    vim9script
    import './Xexport.vim' as expo
    def Assign()
      expo.CONST = 987
    enddef
    defcompile
  END
  writefile(import_assign_to_const, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E46:', '', 1, '_Assign')

  # try changing an imported final
  var import_assign_to_final =<< trim END
    vim9script
    import './Xexport.vim' as expo
    def Assign()
      expo.theList = [2]
    enddef
    defcompile
  END
  writefile(import_assign_to_final, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E46:', '', 1, '_Assign')

  var import_no_as_lines =<< trim END
    vim9script
    import './Xexport.vim' name
  END
  writefile(import_no_as_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E488:', '', 2, 'Ximport.vim')

  # trailing starts with "as"
  var import_bad_as_lines =<< trim END
    vim9script
    import './Xexport.vim' asname
  END
  writefile(import_no_as_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E488:', '', 2, 'Ximport.vim')

  var import_invalid_string_lines =<< trim END
    vim9script
    import Xexport.vim
  END
  writefile(import_invalid_string_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E121:', '', 2, 'Ximport.vim')

  var import_wrong_name_lines =<< trim END
    vim9script
    import './XnoExport.vim'
  END
  writefile(import_wrong_name_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1053:', '', 2, 'Ximport.vim')

  var import_redefining_lines =<< trim END
    vim9script
    import './Xexport.vim' as exported
    var exported = 5
  END
  writefile(import_redefining_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1213: Redefining imported item "exported"', '', 3)

  var import_missing_dot_lines =<< trim END
    vim9script
    import './Xexport.vim' as expo
    def Test()
      expo = 9
    enddef
    defcompile
  END
  writefile(import_missing_dot_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1258:', '', 1)

  var import_missing_name_lines =<< trim END
    vim9script
    import './Xexport.vim' as expo
    def Test()
      expo.99 = 9
    enddef
    defcompile
  END
  writefile(import_missing_name_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1259:', '', 1)

  var import_assign_wrong_type_lines =<< trim END
    vim9script
    import './Xexport.vim' as expo
    expo.exported = 'xxx'
  END
  writefile(import_assign_wrong_type_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E1012: Type mismatch; expected number but got string', '', 3)

  var import_assign_const_lines =<< trim END
    vim9script
    import './Xexport.vim' as expo
    expo.CONST = 4321
  END
  writefile(import_assign_const_lines, 'Ximport.vim')
  assert_fails('source Ximport.vim', 'E46: Cannot change read-only variable "CONST"', '', 3)

  # Check that in a Vim9 script 'cpo' is set to the Vim default.
  # Flags added or removed are also applied to the restored value.
  set cpo=abcd
  var lines =<< trim END
    vim9script
    g:cpo_in_vim9script = &cpo
    set cpo+=f
    set cpo-=c
    g:cpo_after_vim9script = &cpo
  END
  writefile(lines, 'Xvim9_script', 'D')
  source Xvim9_script
  assert_equal('fabd', &cpo)
  set cpo&vim
  assert_equal(&cpo, g:cpo_in_vim9script)
  var newcpo = substitute(&cpo, 'c', '', '') .. 'f'
  assert_equal(newcpo, g:cpo_after_vim9script)

  delete('Xvim9_script')
enddef

def Test_import_very_long_name()
  var lines =<< trim END
      vim9script

      export var verylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongname = 'asdf'
  END
  writefile(lines, 'Xverylong.vim', 'D')

  lines =<< trim END
      vim9script
      import './Xverylong.vim'

      g:result = Xverylong.verylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongnameverylongname
  END
  v9.CheckScriptSuccess(lines)
  assert_equal('asdf', g:result)

  unlet g:result
enddef

def Test_import_funcref()
  var lines =<< trim END
      vim9script
      export def F(): number
        return 42
      enddef
      export const G = F
  END
  writefile(lines, 'Xlib.vim', 'D')

  lines =<< trim END
      vim9script
      import './Xlib.vim' as lib
      const Foo = lib.G()
      assert_equal(42, Foo)

      def DoTest()
        const Goo = lib.G()
        assert_equal(42, Goo)
      enddef
      DoTest()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_export_closure()
  # tests that the closure in block can be compiled, not the import part
  var lines =<< trim END
      vim9script
      {
        var foo = 42
        export def Bar(): number
          return foo
        enddef
      }
      assert_equal(42, Bar())
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_import_duplicate_function()
  # Function Hover() exists in both scripts, partial should refer to the right
  # one.
  var lines =<< trim END
      vim9script

      def Hover(d: dict<any>): string
        return 'found it'
      enddef

      export def NewLspServer(): dict<any>
        var d: dict<any> = {}
        d->extend({hover: function('Hover', [d])})
        return d
      enddef

      NewLspServer()
  END
  writefile(lines, 'Xserver.vim', 'D')

  lines =<< trim END
      vim9script

      import './Xserver.vim' as server

      export def Hover()
      enddef

      def AddServer()
        var d: dict<any> = server.NewLspServer()
        assert_equal('found it', d.hover())
      enddef
      AddServer()
  END
  v9.CheckScriptSuccess(lines)
enddef


def Test_import_fails()
  writefile([], 'Xfoo.vim', 'D')
  var lines =<< trim END
      import './Xfoo.vim' as foo
      foo = 'bar'
  END
  v9.CheckDefAndScriptFailure(lines, ['E1094:', 'E1236: Cannot use foo itself'])
  lines =<< trim END
      vim9script
      import './Xfoo.vim' as foo
      var that = foo
  END
  v9.CheckScriptFailure(lines, 'E1060: Expected dot after name: foo')
  lines =<< trim END
      vim9script
      import './Xfoo.vim' as foo
      var that: any
      that += foo
  END
  v9.CheckScriptFailure(lines, 'E1060: Expected dot after name: foo')
  lines =<< trim END
      vim9script
      import './Xfoo.vim' as foo
      foo += 9
  END
  v9.CheckScriptFailure(lines, 'E1060: Expected dot after name: foo')

  lines =<< trim END
      vim9script
      import './Xfoo.vim' as 9foo
  END
  v9.CheckScriptFailure(lines, 'E1047:')
  lines =<< trim END
      vim9script
      import './Xfoo.vim' as the#foo
  END
  v9.CheckScriptFailure(lines, 'E1047:')
  lines =<< trim END
      vim9script
      import './Xfoo.vim' as g:foo
  END
  v9.CheckScriptFailure(lines, 'E1047:')

  lines =<< trim END
      vim9script
      def TheFunc()
        echo 'the func'
      enddef
      export var Ref = TheFunc
  END
  writefile([], 'Xthat.vim')

  lines =<< trim END
      import './Xthat.vim' as That
      That()
  END
  v9.CheckDefAndScriptFailure(lines, ['E1094:', 'E1236: Cannot use That itself'])

  lines =<< trim END
      vim9script
      import './Xthat.vim' as That
      def Func()
        echo That()
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E1236: Cannot use That itself')

  lines =<< trim END
      import './Xthat.vim' as one
      import './Xthat.vim' as two
  END
  v9.CheckScriptFailure(lines, 'E1262:')

  delete('Xthat.vim')

  lines =<< trim END
      vim9script
      export var item = 'hello'
      import './Xyourself.vim'
  END
  writefile(lines, 'Xyourself.vim', 'D')
  assert_fails('source Xyourself.vim', 'E1088:')

  mkdir('Ximport', 'R')

  writefile(['vim9script'], 'Ximport/.vim')
  lines =<< trim END
      vim9script
      import './Ximport/.vim'
  END
  v9.CheckScriptFailure(lines, 'E1261: Cannot import .vim without using "as"')
  lines =<< trim END
      vim9script
      import './Ximport/.vim' as vim
  END
  v9.CheckScriptSuccess(lines)

  writefile(['vim9script'], 'Ximport/.vimrc')
  lines =<< trim END
      vim9script
      import './Ximport/.vimrc'
  END
  v9.CheckScriptFailure(lines, 'E1257: Imported script must use "as" or end in .vim')
  lines =<< trim END
      vim9script
      import './Ximport/.vimrc' as vimrc
  END
  v9.CheckScriptSuccess(lines)

  new
  setline(1, ['vim9script', 'import "" as abc'])
  assert_fails('source', 'E1071: Invalid string for :import: "" as abc')
  setline(2, 'import [] as abc')
  assert_fails('source', 'E1071: Invalid string for :import: [] as abc')
  setline(2, 'import test_null_string() as abc')
  assert_fails('source', 'E1071: Invalid string for :import: test_null_string() as abc')
  bw!
  call writefile(['vim9script', "import './Xfoo.vim' ask expo"], 'Xbar.vim')
  assert_fails('source Xbar.vim', 'E488: Trailing characters: ask expo')
  writefile([], 'Xtemp')
  call writefile(['vim9script', "import './Xtemp'"], 'Xbar.vim')
  assert_fails('source Xbar.vim', 'E1257: Imported script must use "as" or end in .vim: Xtemp')
  delete('Xtemp')
  call writefile(['vim9script', "import './Xfoo.vim' as abc | foobar"], 'Xbar.vim')
  assert_fails('source Xbar.vim', 'E492: Not an editor command:  foobar')
  call delete('Xbar.vim')
enddef

func g:Trigger()
  source Ximport.vim
  return "echo 'yes'\<CR>"
endfunc

def Test_import_export_expr_map()
  # check that :import and :export work when buffer is locked
  var export_lines =<< trim END
    vim9script
    export def That(): string
      return 'yes'
    enddef
  END
  writefile(export_lines, 'Xexport_that.vim', 'D')

  var import_lines =<< trim END
    vim9script
    import './Xexport_that.vim' as that
    assert_equal('yes', that.That())
  END
  writefile(import_lines, 'Ximport.vim', 'D')

  nnoremap <expr> trigger g:Trigger()
  feedkeys('trigger', "xt")

  nunmap trigger
enddef

def Test_import_in_filetype()
  # check that :import works when the buffer is locked
  mkdir('ftplugin', 'pR')
  var export_lines =<< trim END
    vim9script
    export var That = 'yes'
  END
  writefile(export_lines, 'ftplugin/Xexport_ft.vim')

  var import_lines =<< trim END
    vim9script
    import './Xexport_ft.vim' as ft
    assert_equal('yes', ft.That)
    g:did_load_mytpe = 1
  END
  writefile(import_lines, 'ftplugin/qf.vim')

  var save_rtp = &rtp
  &rtp = getcwd() .. ',' .. &rtp

  filetype plugin on
  copen
  assert_equal(1, g:did_load_mytpe)

  quit!
  &rtp = save_rtp
enddef

def Test_use_import_in_mapping()
  var lines =<< trim END
      vim9script
      export def Funcx(nr: number)
        g:result = nr
      enddef
  END
  writefile(lines, 'XsomeExport.vim', 'D')
  lines =<< trim END
      vim9script
      import './XsomeExport.vim' as some
      var Funcy = some.Funcx
      nnoremap <F3> :call <sid>Funcy(42)<cr>
      nnoremap <F4> :call <sid>some.Funcx(44)<cr>
  END
  writefile(lines, 'Xmapscript.vim', 'D')

  source Xmapscript.vim
  feedkeys("\<F3>", "xt")
  assert_equal(42, g:result)
  feedkeys("\<F4>", "xt")
  assert_equal(44, g:result)

  unlet g:result
  nunmap <F3>
  nunmap <F4>
enddef

def Test_use_relative_autoload_import_in_mapping()
  var lines =<< trim END
      vim9script
      export def Func()
        g:result = 42
      enddef
  END
  writefile(lines, 'XrelautoloadExport.vim', 'D')
  lines =<< trim END
      vim9script
      import autoload './XrelautoloadExport.vim' as some
      nnoremap <F3> :call <SID>some.Func()<CR>
  END
  writefile(lines, 'Xmapscript.vim', 'D')

  source Xmapscript.vim
  assert_match('\d\+ A: .*XrelautoloadExport.vim', execute('scriptnames')->split("\n")[-1])
  var l = getscriptinfo()
  assert_match('XrelautoloadExport.vim$', l[-1].name)
  assert_true(l[-1].autoload)
  feedkeys("\<F3>", "xt")
  assert_equal(42, g:result)
  l = getscriptinfo({name: 'XrelautoloadExport'})
  assert_true(len(l) == 1)
  assert_match('XrelautoloadExport.vim$', l[0].name)
  assert_false(l[0].autoload)
  assert_equal(999999, l[0].version)

  unlet g:result
  nunmap <F3>
enddef

def Test_autoload_import_var()
  # variable name starts with "autoload"
  var lines =<< trim END
      vim9script
      var autoloaded = "Xtest.vim"
      import autoloaded
  END
  v9.CheckScriptFailure(lines, 'E1053: Could not import "Xtest.vim')
enddef

def Test_use_autoload_import_in_mapping()
  var lines =<< trim END
      vim9script
      export def Func()
        g:result = 49
      enddef
  END
  mkdir('Ximpdir/autoload', 'pR')
  writefile(lines, 'Ximpdir/autoload/XautoloadExport.vim')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Ximpdir'

  lines =<< trim END
      vim9script
      import autoload 'XautoloadExport.vim' as some
      nnoremap <F3> :call <SID>some.Func()<CR>
  END
  writefile(lines, 'Xmapscript.vim', 'D')

  source Xmapscript.vim
  assert_match('\d\+ A: .*autoload/XautoloadExport.vim', execute('scriptnames')->split("\n")[-1])
  feedkeys("\<F3>", "xt")
  assert_equal(49, g:result)

  unlet g:result
  nunmap <F3>
  &rtp = save_rtp
enddef

def Test_use_import_in_command_completion()
  var lines =<< trim END
      vim9script
      export def Complete(..._): list<string>
        return ['abcd']
      enddef
  END
  writefile(lines, 'Xscript.vim', 'D')

  lines =<< trim END
      vim9script
      import './Xscript.vim'

      command -nargs=1 -complete=customlist,Xscript.Complete Cmd echo 'ok'
      feedkeys(":Cmd ab\<Tab>\<C-B>#\<CR>", 'xnt')
      assert_equal('#Cmd abcd', @:)
  END
  v9.CheckScriptSuccess(lines)

  delcommand Cmd
enddef

def Test_use_import_with_funcref_in_command_completion()
  var lines =<< trim END
      vim9script
      export def Complete(..._): list<string>
        return ['abcd']
      enddef
  END
  writefile(lines, 'Xscript.vim', 'D')

  lines =<< trim END
      vim9script
      import './Xscript.vim'

      var Ref = Xscript.Complete
      exe "command -nargs=1 -complete=customlist," .. expand('<SID>') .. "Ref  Cmd echo 'ok'"
      feedkeys(":Cmd ab\<Tab>\<C-B>#\<CR>", 'xnt')
      assert_equal('#Cmd abcd', @:)
  END
  v9.CheckScriptSuccess(lines)

  delcommand Cmd
enddef

def Test_use_autoload_import_in_insert_completion()
  mkdir('Xinsdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xinsdir'

  var lines =<< trim END
      vim9script
      export def ThesaurusFunc(findbase: bool, _): any
        if findbase
          return 1
        endif
        return [
          'check',
          'experiment',
          'test',
          'verification'
          ]
      enddef
      g:completion_loaded = 'yes'
  END
  writefile(lines, 'Xinsdir/autoload/completion.vim')

  new
  lines =<< trim END
      vim9script
      g:completion_loaded = 'no'
      import autoload 'completion.vim'
      set thesaurusfunc=completion.ThesaurusFunc
      assert_equal('no', g:completion_loaded)
      feedkeys("i\<C-X>\<C-T>\<C-N>\<Esc>", 'xt')
      assert_equal('experiment', getline(1))
      assert_equal('yes', g:completion_loaded)
  END
  v9.CheckScriptSuccess(lines)

  set thesaurusfunc=
  bwipe!
  &rtp = save_rtp
enddef

def Test_use_autoload_import_partial_in_opfunc()
  mkdir('Xpartdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xpartdir'

  var lines =<< trim END
      vim9script
      export def Opfunc1(..._)
        g:opfunc_called = 'yes'
      enddef
  END
  writefile(lines, 'Xpartdir/autoload/opfunc.vim')

  new
  lines =<< trim END
      vim9script
      import autoload 'opfunc.vim'
      nnoremap <expr> <F3> TheFunc()
      def TheFunc(): string
        &operatorfunc = function('opfunc.Opfunc1', [0])
        return 'g@'
      enddef
      feedkeys("\<F3>l", 'xt')
      assert_equal('yes', g:opfunc_called)
  END
  v9.CheckScriptSuccess(lines)

  set opfunc=
  bwipe!
  nunmap <F3>
  &rtp = save_rtp
enddef

def Test_set_opfunc_to_autoload_func_directly()
  mkdir('Xdirdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xdirdir'

  var lines =<< trim END
      vim9script
      export def Opfunc2(..._)
        g:opfunc_called = 'yes'
      enddef
  END
  writefile(lines, 'Xdirdir/autoload/opfunc.vim')

  new
  lines =<< trim END
      vim9script
      import autoload 'opfunc.vim'
      nnoremap <expr> <F3> TheFunc()
      def TheFunc(): string
        &operatorfunc = opfunc.Opfunc2
        return 'g@'
      enddef
      feedkeys("\<F3>l", 'xt')
      assert_equal('yes', g:opfunc_called)
  END
  v9.CheckScriptSuccess(lines)

  set opfunc=
  bwipe!
  nunmap <F3>
  &rtp = save_rtp
enddef

def Test_use_autoload_import_in_fold_expression()
  mkdir('Xfolddir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xfolddir'

  var lines =<< trim END
      vim9script
      export def Expr(): string
        return getline(v:lnum) =~ '^#' ? '>1' : '1'
      enddef
      export def Text(): string
        return 'fold text'
      enddef
      g:fold_loaded = 'yes'
  END
  writefile(lines, 'Xfolddir/autoload/fold.vim')

  lines =<< trim END
      vim9script
      import autoload 'fold.vim'
      &foldexpr = 'fold.Expr()'
      &foldtext = 'fold.Text()'
      &foldmethod = 'expr'
      &debug = 'throw'
  END
  new
  setline(1, ['# one', 'text', '# two', 'text'])
  g:fold_loaded = 'no'
  v9.CheckScriptSuccess(lines)
  assert_equal('no', g:fold_loaded)
  redraw
  assert_equal('yes', g:fold_loaded)

  # Check that script context of 'foldexpr' is copied to another buffer.
  edit! otherfile
  redraw

  set foldexpr= foldtext& foldmethod& debug=
  bwipe!
  &rtp = save_rtp
enddef

def Test_autoload_import_relative()
  var lines =<< trim END
      vim9script

      g:loaded = 'yes'
      export def RelFunc(): string
        return 'relfunc'
      enddef
      def NotExported()
        echo 'not'
      enddef

      export var someText = 'some text'
      var notexp = 'bad'
  END
  writefile(lines, 'XimportRel.vim', 'D')
  writefile(lines, 'XimportRel2.vim', 'D')
  writefile(lines, 'XimportRel3.vim', 'D')
  writefile(lines, 'XimportRel4.vim', 'D')
  writefile(lines, 'XimportRel5.vim', 'D')

  lines =<< trim END
      vim9script
      g:loaded = 'no'
      import autoload './XimportRel.vim'
      assert_equal('no', g:loaded)

      def AFunc(): string
        var res = ''
        res ..= XimportRel.RelFunc()
        res ..= '/'
        res ..= XimportRel.someText
        XimportRel.someText = 'from AFunc'
        return res
      enddef
      # script not loaded when compiling
      defcompile
      assert_equal('no', g:loaded)

      assert_equal('relfunc/some text', AFunc())
      assert_equal('yes', g:loaded)
      unlet g:loaded

      assert_equal('from AFunc', XimportRel.someText)
      XimportRel.someText = 'from script'
      assert_equal('from script', XimportRel.someText)
  END
  v9.CheckScriptSuccess(lines)

  lines =<< trim END
      vim9script
      import autoload './XimportRel.vim'
      echo XimportRel.NotExported()
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: NotExported', 3)

  lines =<< trim END
      vim9script
      import autoload './XimportRel.vim'
      echo XimportRel.notexp
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: notexp', 3)

  lines =<< trim END
      vim9script
      import autoload './XimportRel.vim'
      XimportRel.notexp = 'bad'
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: notexp', 3)

  lines =<< trim END
      vim9script
      import autoload './XimportRel.vim'
      def Func()
        echo XimportRel.NotExported()
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: NotExported', 1)

  lines =<< trim END
      vim9script
      import autoload './XimportRel.vim'
      def Func()
        echo XimportRel.notexp
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: notexp', 1)

  # Same, script not imported before
  lines =<< trim END
      vim9script
      import autoload './XimportRel4.vim'
      def Func()
        echo XimportRel4.notexp
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: notexp', 1)

  # does not fail if the script wasn't loaded yet and only compiling
  g:loaded = 'no'
  lines =<< trim END
      vim9script
      import autoload './XimportRel2.vim'
      def Func()
        echo XimportRel2.notexp
      enddef
      defcompile
  END
  v9.CheckScriptSuccess(lines)
  assert_equal('no', g:loaded)

  lines =<< trim END
      vim9script
      import autoload './XimportRel.vim'
      def Func()
        XimportRel.notexp = 'bad'
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: notexp', 1)

  # fails with a not loaded import
  lines =<< trim END
      vim9script
      import autoload './XimportRel3.vim'
      def Func()
        XimportRel3.notexp = 'bad'
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: notexp', 1)
  assert_equal('yes', g:loaded)
  unlet g:loaded

  lines =<< trim END
      vim9script
      import autoload './XimportRel5.vim'
      def Func()
        XimportRel5.nosuchvar = 'bad'
      enddef
      Func()
  END
  v9.CheckScriptFailure(lines, 'E121: Undefined variable: nosuchvar', 1)
  unlet g:loaded

  # nasty: delete script after compiling function
  writefile(['vim9script'], 'XimportRelDel.vim')
  lines =<< trim END
      vim9script

      import autoload './XimportRelDel.vim'
      def DoIt()
        echo XimportRelDel.var
      enddef
      defcompile
      delete('XimportRelDel.vim')
      DoIt()
  END
  v9.CheckScriptFailure(lines, 'E484:')
enddef

def Test_autoload_import_relative_autoload_dir()
  mkdir('autoload', 'pR')
  var lines =<< trim END
      vim9script
      export def Bar()
        g:called_bar = 'yes'
      enddef
  END
  writefile(lines, 'autoload/script.vim')

  lines =<< trim END
      vim9script
      import autoload './autoload/script.vim'
      def Foo()
        script.Bar()
      enddef
      Foo()
      assert_equal('yes', g:called_bar)
  END
  v9.CheckScriptSuccess(lines)

  unlet g:called_bar
enddef

def Test_autoload_import_deleted()
  var lines =<< trim END
      vim9script
      export const FOO = 1
  END
  writefile(lines, 'Xa.vim', 'D')

  lines =<< trim END
      vim9script
      import autoload './Xa.vim'

      delete('Xa.vim')
      var x = Xa.FOO
  END
  v9.CheckScriptFailure(lines, 'E484:')
enddef

def Test_autoload_import_using_const()
  mkdir('Xdir/autoload', 'pR')
  var lines =<< trim END
      vim9script
      export const FOO = 42
      echomsg FOO
  END
  writefile(lines, 'Xdir/autoload/exp.vim')

  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xdir'
  lines =<< trim END
      vim9script
      import autoload 'exp.vim'
      assert_equal(42, exp.FOO)
  END
  v9.CheckScriptSuccess(lines)
  &rtp = save_rtp
enddef

func Test_import_in_diffexpr()
  CheckExecutable diff

  call Run_Test_import_in_diffexpr()
endfunc

def Run_Test_import_in_diffexpr()
  var lines =<< trim END
      vim9script

      export def DiffExpr()
        # Prepend some text to check diff type detection
        writefile(['warning', '  message'], v:fname_out)
        silent exe '!diff ' .. v:fname_in .. ' '
                            .. v:fname_new .. '>>' .. v:fname_out
      enddef
  END
  writefile(lines, 'Xdiffexpr', 'D')

  lines =<< trim END
      vim9script
      import './Xdiffexpr' as diff

      set diffexpr=diff.DiffExpr()
      set diffopt=foldcolumn:0
  END
  v9.CheckScriptSuccess(lines)

  enew!
  call setline(1, ['one', 'two', 'three'])
  diffthis

  botright vert new
  call setline(1, ['one', 'two', 'three.'])
  diffthis
  # we only check if this does not cause errors
  redraw

  diffoff!
  set diffexpr=
  set diffopt&
  bwipe!
  bwipe!
enddef

def Test_import_in_patchexpr()
  var lines =<< trim END
    vim9script
    export def TPatch()
      call writefile(['output file'], v:fname_out)
    enddef
  END
  writefile(lines, 'Xpatchexpr', 'D')

  lines =<< trim END
      vim9script
      import './Xpatchexpr' as patch
      set patchexpr=patch.TPatch()
  END
  v9.CheckScriptSuccess(lines)

  call writefile(['input file'], 'Xinput', 'D')
  call writefile(['diff file'], 'Xdiff', 'D')
  :%bwipe!
  edit Xinput
  diffpatch Xdiff
  call assert_equal('output file', getline(1))

  set patchexpr&
  :%bwipe!
enddef

def Test_import_in_formatexpr()
  var lines =<< trim END
      vim9script
      export def MyFormatExpr(): number
        g:did_format = 'yes'
        return 0
      enddef
  END
  writefile(lines, 'Xformatter', 'D')

  lines =<< trim END
      vim9script
      import './Xformatter' as format
      set formatexpr=format.MyFormatExpr()
  END
  v9.CheckScriptSuccess(lines)

  new
  setline(1, ['a', 'b', 'c'])
  normal gqG
  assert_equal('yes', g:did_format)

  bwipe!
  unlet g:did_format
  set formatexpr=
enddef

def Test_import_in_includeexpr()
  writefile(['found it'], 'Xthisfile', 'D')
  new

  var lines =<< trim END
      vim9script
      export def DoSub(): string
        return substitute(v:fname, 'that', 'this', '')
      enddef
  END
  writefile(lines, 'Xinclude.vim', 'D')

  lines =<< trim END
    vim9script
    import './Xinclude.vim'
    set includeexpr=Xinclude.DoSub()
  END
  v9.CheckScriptSuccess(lines)

  setline(1, ['Xthatfile'])
  exe "normal \<C-W>f"
  assert_equal('Xthisfile', expand('%'))

  bwipe!
  bwipe!
  set includeexpr=
enddef

def Test_import_in_indentexpr()
  var lines =<< trim END
      vim9script
      export def GetIndent(): number
        return 5
      enddef
  END
  writefile(lines, 'Xindenter', 'D')

  lines =<< trim END
      vim9script
      import './Xindenter' as indent
      set indentexpr=indent.GetIndent()
      set debug=throw
  END
  v9.CheckScriptSuccess(lines)

  new
  setline(1, 'hello')
  normal ==
  assert_equal('     hello', getline(1))

  bwipe!
  set indentexpr= debug=
enddef

func Test_import_in_printexpr()
  CheckFeature postscript
  call Run_Test_import_in_printexpr()
endfunc

def Run_Test_import_in_printexpr()
  var lines =<< trim END
      vim9script
      export def PrintFile(): bool
        g:printed = 'yes'
        delete('v:fname_in')
        return false
      enddef
  END
  writefile(lines, 'Xprint.vim', 'D')

  lines =<< trim END
      vim9script
      import './Xprint.vim'
      set printexpr=Xprint.PrintFile()
  END
  v9.CheckScriptSuccess(lines)

  help
  hardcopy dummy args
  assert_equal('yes', g:printed)

  set printexpr=
enddef

def Test_import_in_charconvert()
  var lines =<< trim END
      vim9script
      export def MakeUpper(): bool
        var data = readfile(v:fname_in)
        map(data, 'toupper(v:val)')
        writefile(data, v:fname_out)
        return false  # success
      enddef
  END
  writefile(lines, 'Xconvert.vim', 'D')

  lines =<< trim END
      vim9script
      import './Xconvert.vim' as conv
      set charconvert=conv.MakeUpper()
  END
  v9.CheckScriptSuccess(lines)

  writefile(['one', 'two'], 'Xiicfile', 'D')
  new Xiicfile
  write ++enc=ucase Xiicfile1
  assert_equal(['ONE', 'TWO'], readfile('Xiicfile1'))

  delete('Xiicfile1')
  bwipe!
  set charconvert&
enddef

func Test_import_in_spellsuggest_expr()
  CheckFeature spell
  call Run_Test_import_in_spellsuggest_expr()
endfunc

def Run_Test_import_in_spellsuggest_expr()
  var lines =<< trim END
      vim9script
      export def MySuggest(): list<any>
        return [['Fox', 8], ['Fop', 9]]
      enddef
  END
  writefile(lines, 'Xsuggest.vim', 'D')

  lines =<< trim END
      vim9script
      import './Xsuggest.vim' as sugg
      set spell spellsuggest=expr:sugg.MySuggest()
  END
  v9.CheckScriptSuccess(lines)

  set verbose=1  # report errors
  call assert_equal(['Fox', 'Fop'], spellsuggest('Fo', 2))

  set nospell spellsuggest& verbose=0
enddef

def Test_import_in_lambda_method()
  var lines =<< trim END
      vim9script
      export def Retarg(e: any): any
        return e
      enddef
  END
  writefile(lines, 'XexportRetarg.vim', 'D')
  lines =<< trim END
      vim9script
      import './XexportRetarg.vim'
      def Lambda(): string
        var F = (x) => x->XexportRetarg.Retarg()
        return F('arg')
      enddef
      assert_equal('arg', Lambda())
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_export_shadows_global_function()
  mkdir('Xglobdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xglobdir'

  var lines =<< trim END
      vim9script
      export def Shadow(): string
        return 'Shadow()'
      enddef
  END
  writefile(lines, 'Xglobdir/autoload/shadow.vim')

  lines =<< trim END
      vim9script

      def g:Shadow(): string
        return 'global'
      enddef

      import autoload 'shadow.vim'
      assert_equal('Shadow()', shadow.Shadow())
  END
  v9.CheckScriptSuccess(lines)

  delfunc g:Shadow
  bwipe!
  &rtp = save_rtp
enddef

def Test_export_fails()
  v9.CheckScriptFailure(['export var some = 123'], 'E1042:')
  v9.CheckScriptFailure(['vim9script', 'export var g:some'], 'E1022:')
  v9.CheckScriptFailure(['vim9script', 'export echo 134'], 'E1043:')
  v9.CheckScriptFailure(['vim9script', 'export function /a1b2c3'], 'E1044:')

  assert_fails('export echo 1', 'E1043:')
enddef

func Test_import_fails_without_script()
  CheckRunVimInTerminal

  " call indirectly to avoid compilation error for missing functions
  call Run_Test_import_fails_on_command_line()
endfunc

def Run_Test_import_fails_on_command_line()
  var export =<< trim END
    vim9script
    export def Foo(): number
        return 0
    enddef
  END
  writefile(export, 'XexportCmd.vim', 'D')

  var buf = g:RunVimInTerminal('-c "import Foo from ''./XexportCmd.vim''"', {
                rows: 6, wait_for_ruler: 0})
  g:WaitForAssert(() => assert_match('^E1094:', term_getline(buf, 5)))

  g:StopVimInTerminal(buf)
enddef

def Test_vim9_reload_noclear()
  var lines =<< trim END
    vim9script
    export var exported = 'thexport'

    export def TheFunc(x = 0)
    enddef
  END
  writefile(lines, 'XExportReload', 'D')
  lines =<< trim END
    vim9script noclear
    g:loadCount += 1
    var reloaded = 'init'
    import './XExportReload' as exp

    def Again(): string
      return 'again'
    enddef

    exp.TheFunc()

    if exists('loaded') | finish | endif
    var loaded = true

    var notReloaded = 'yes'
    reloaded = 'first'
    def g:Values(): list<string>
      return [reloaded, notReloaded, Again(), Once(), exp.exported]
    enddef

    def Once(): string
      return 'once'
    enddef
  END
  writefile(lines, 'XReloaded', 'D')
  g:loadCount = 0
  source XReloaded
  assert_equal(1, g:loadCount)
  assert_equal(['first', 'yes', 'again', 'once', 'thexport'], g:Values())
  source XReloaded
  assert_equal(2, g:loadCount)
  assert_equal(['init', 'yes', 'again', 'once', 'thexport'], g:Values())
  source XReloaded
  assert_equal(3, g:loadCount)
  assert_equal(['init', 'yes', 'again', 'once', 'thexport'], g:Values())

  delfunc g:Values
  unlet g:loadCount

  lines =<< trim END
      vim9script
      def Inner()
      enddef
  END
  lines->writefile('XreloadScript.vim', 'D')
  source XreloadScript.vim

  lines =<< trim END
      vim9script
      def Outer()
        def Inner()
        enddef
      enddef
      defcompile
  END
  lines->writefile('XreloadScript.vim')
  source XreloadScript.vim
enddef

def Test_vim_reload_noclear_arg_count()
  var lines =<< trim END
      vim9script noclear

      if !exists('g:didload')
        def Test(a: string, b: string)
          echo a b
        enddef
        def Call()
          Test('a', 'b')
        enddef
      else
        # redefine with one argument less
        def Test(a: string)
          echo a
        enddef
      endif
      Call()
      g:didload = 1
  END
  lines->writefile('XreloadScript_1.vim', 'D')
  source XreloadScript_1.vim
  assert_fails('source XreloadScript_1.vim', 'E1106: One argument too many')
  unlet g:didload

  lines =<< trim END
      vim9script noclear

      if !exists('g:didload')
        def Test(a: string, b: string, c: string)
          echo a b
        enddef
        def Call()
          Test('a', 'b', 'c')
        enddef
      else
        # redefine with one argument less
        def Test(a: string)
          echo a
        enddef
      endif
      Call()
      g:didload = 1
  END
  lines->writefile('XreloadScript_2.vim', 'D')
  source XreloadScript_2.vim
  assert_fails('source XreloadScript_2.vim', 'E1106: 2 arguments too many')
  unlet g:didload

  lines =<< trim END
      vim9script noclear

      if !exists('g:didload')
        def Test(a: string)
          echo a
        enddef
        def Call()
          Test('a')
        enddef
      else
        # redefine with one argument extra
        def Test(a: string, b: string)
          echo a b
        enddef
      endif
      Call()
      g:didload = 1
  END
  lines->writefile('XreloadScript_3.vim', 'D')
  source XreloadScript_3.vim
  assert_fails('source XreloadScript_3.vim', 'E1190: One argument too few')
  unlet g:didload

  lines =<< trim END
      vim9script noclear

      if !exists('g:didload')
        def Test(a: string)
          echo a
        enddef
        def Call()
          Test('a')
        enddef
      else
        # redefine with two arguments extra
        def Test(a: string, b: string, c: string)
          echo a b
        enddef
      endif
      Call()
      g:didload = 1
  END
  lines->writefile('XreloadScript_4.vim', 'D')
  source XreloadScript_4.vim
  assert_fails('source XreloadScript_4.vim', 'E1190: 2 arguments too few')
  unlet g:didload
enddef

def Test_vim9_reload_noclear_error()
  var lines =<< trim END
      vim9script noclear

      if !exists('g:didload')
        def Test(a: string)
          echo a
        enddef
        def Call()
          Test('a')
        enddef
      else
        # redefine with a compile error
        def Test(a: string)
          echo ax
        enddef
      endif
      Call()
      g:didload = 1
  END
  lines->writefile('XreloadScriptErr.vim', 'D')
  source XreloadScriptErr.vim
  assert_fails('source XreloadScriptErr.vim', 'E1001: Variable not found: ax')

  unlet g:didload
enddef

def Test_vim9_reload_import()
  var lines =<< trim END
    vim9script
    const var = ''
    var valone = 1234
    def MyFunc(arg: string)
       valone = 5678
    enddef
  END
  var morelines =<< trim END
    var valtwo = 222
    export def GetValtwo(): number
      return valtwo
    enddef
  END
  writefile(lines + morelines, 'Xreload.vim', 'D')
  source Xreload.vim
  source Xreload.vim
  source Xreload.vim

  # cannot declare a var twice
  lines =<< trim END
    vim9script
    var valone = 1234
    var valone = 5678
  END
  writefile(lines, 'Xreload.vim')
  assert_fails('source Xreload.vim', 'E1041:', '', 3, 'Xreload.vim')

  delete('Ximport.vim')
enddef

" if a script is reloaded with a script-local variable that changed its type, a
" compiled function using that variable must fail.
def Test_script_reload_change_type()
  var lines =<< trim END
    vim9script noclear
    var str = 'string'
    def g:GetStr(): string
      return str .. 'xxx'
    enddef
  END
  writefile(lines, 'Xreload.vim', 'D')
  source Xreload.vim
  echo g:GetStr()

  lines =<< trim END
    vim9script noclear
    var str = 1234
  END
  writefile(lines, 'Xreload.vim')
  source Xreload.vim
  assert_fails('echo g:GetStr()', 'E1150:')

  delfunc g:GetStr
enddef

" Define CallFunc so that the test can be compiled
command CallFunc echo 'nop'

def Test_script_reload_from_function()
  var lines =<< trim END
      vim9script

      if exists('g:loadedThis')
        finish
      endif
      g:loadedThis = 1
      delcommand CallFunc
      command CallFunc Func()
      def Func()
        so XreloadFunc.vim
        g:didTheFunc = 1
      enddef
  END
  writefile(lines, 'XreloadFunc.vim', 'D')
  source XreloadFunc.vim
  CallFunc
  assert_equal(1, g:didTheFunc)

  delcommand CallFunc
  unlet g:loadedThis
  unlet g:didTheFunc
enddef

def s:RetSome(): string
  return 'some'
enddef

" Not exported function that is referenced needs to be accessed by the
" script-local name.
def Test_vim9_funcref()
  var sortlines =<< trim END
      vim9script
      def Compare(i1: number, i2: number): number
        return i2 - i1
      enddef

      export def FastSort(): list<number>
        return range(5)->sort(Compare)
      enddef

      export def GetString(arg: string): string
        return arg
      enddef
  END
  writefile(sortlines, 'Xsort.vim', 'D')

  var lines =<< trim END
    vim9script
    import './Xsort.vim'
    def Test()
      g:result = Xsort.FastSort()
    enddef
    Test()
  END
  writefile(lines, 'Xscript.vim', 'D')
  source Xscript.vim
  assert_equal([4, 3, 2, 1, 0], g:result)
  unlet g:result

  lines =<< trim END
    vim9script
    # using a function imported with "as"
    import './Xsort.vim' as anAlias
    assert_equal('yes', anAlias.GetString('yes'))

    # using the function from a compiled function
    def TestMore(): string
      var s = s:anAlias.GetString('foo')
      return s .. anAlias.GetString('bar')
    enddef
    assert_equal('foobar', TestMore())

    # error when using a function that isn't exported
    assert_fails('anAlias.Compare(1, 2)', 'E1049:')
  END
  writefile(lines, 'Xscript.vim')

  var Funcref = function('s:RetSome')
  assert_equal('some', Funcref())
enddef

" Check that when searching for "FilterFunc" it finds the import in the
" script where FastFilter() is called from, both as a string and as a direct
" function reference.
def Test_vim9_funcref_other_script()
  var filterLines =<< trim END
    vim9script
    export def FilterFunc(idx: number, val: number): bool
      return idx % 2 == 1
    enddef
    export def FastFilter(): list<number>
      return range(10)->filter('FilterFunc(v:key, v:val)')
    enddef
    export def FastFilterDirect(): list<number>
      return range(10)->filter(FilterFunc)
    enddef
  END
  writefile(filterLines, 'Xfilter.vim', 'D')

  var lines =<< trim END
    vim9script
    import './Xfilter.vim' as filter
    def Test()
      var x: list<number> = filter.FastFilter()
    enddef
    Test()
    def TestDirect()
      var x: list<number> = filter.FastFilterDirect()
    enddef
    TestDirect()
  END
  v9.CheckScriptSuccess(lines)
enddef

def Test_import_absolute()
  var import_lines = [
        'vim9script',
        'import "' .. escape(getcwd(), '\') .. '/Xexport_abs.vim" as abs',
        'def UseExported()',
        '  g:imported_abs = abs.exported',
        '  abs.exported = 8888',
        '  g:imported_after = abs.exported',
        'enddef',
        'UseExported()',
        'g:import_disassembled = execute("disass UseExported")',
        ]
  writefile(import_lines, 'Ximport_abs.vim', 'D')
  writefile(s:export_script_lines, 'Xexport_abs.vim', 'D')

  source Ximport_abs.vim

  assert_equal(9876, g:imported_abs)
  assert_equal(8888, g:imported_after)
  assert_match('<SNR>\d\+_UseExported\_s*' ..
          'g:imported_abs = abs.exported\_s*' ..
          '0 LOADSCRIPT exported-2 from .*Xexport_abs.vim\_s*' ..
          '1 STOREG g:imported_abs\_s*' ..
          'abs.exported = 8888\_s*' ..
          '2 PUSHNR 8888\_s*' ..
          '3 STORESCRIPT exported-2 in .*Xexport_abs.vim\_s*' ..
          'g:imported_after = abs.exported\_s*' ..
          '4 LOADSCRIPT exported-2 from .*Xexport_abs.vim\_s*' ..
          '5 STOREG g:imported_after',
        g:import_disassembled)

  Undo_export_script_lines()
  unlet g:imported_abs
  unlet g:import_disassembled
enddef

def Test_import_rtp()
  var import_lines = [
        'vim9script',
        'import "Xexport_rtp.vim" as rtp',
        'g:imported_rtp = rtp.exported',
        ]
  writefile(import_lines, 'Ximport_rtp.vim', 'D')
  mkdir('import', 'pR')
  writefile(s:export_script_lines, 'import/Xexport_rtp.vim')

  var save_rtp = &rtp
  &rtp = getcwd()
  source Ximport_rtp.vim
  &rtp = save_rtp

  assert_equal(9876, g:imported_rtp)

  Undo_export_script_lines()
  unlet g:imported_rtp
enddef

def Test_import_compile_error()
  var export_lines = [
        'vim9script',
        'export def ExpFunc(): string',
        '  return notDefined',
        'enddef',
        ]
  writefile(export_lines, 'Xexported.vim', 'D')

  var import_lines = [
        'vim9script',
        'import "./Xexported.vim" as expo',
        'def ImpFunc()',
        '  echo expo.ExpFunc()',
        'enddef',
        'defcompile',
        ]
  writefile(import_lines, 'Ximport.vim', 'D')

  try
    source Ximport.vim
  catch /E1001/
    # Error should be before the Xexported.vim file.
    assert_match('E1001: Variable not found: notDefined', v:exception)
    assert_match('function <SNR>\d\+_ImpFunc\[1\]..<SNR>\d\+_ExpFunc, line 1', v:throwpoint)
  endtry
enddef

def Test_func_overrules_import_fails()
  var export_lines =<< trim END
      vim9script
      export def Func()
        echo 'imported'
      enddef
  END
  writefile(export_lines, 'XexportedFunc.vim', 'D')

  var lines =<< trim END
    vim9script
    import './XexportedFunc.vim' as Func
    def Func()
      echo 'local to function'
    enddef
  END
  v9.CheckScriptFailure(lines, 'E1213: Redefining imported item "Func"')

  lines =<< trim END
    vim9script
    import './XexportedFunc.vim' as Func
    def Outer()
      def Func()
        echo 'local to function'
      enddef
    enddef
    defcompile
  END
  v9.CheckScriptFailure(lines, 'E1236:')
enddef

def Test_source_vim9_from_legacy()
  var vim9_lines =<< trim END
    vim9script
    var local = 'local'
    g:global = 'global'
    export var exported = 'exported'
    export def GetText(): string
       return 'text'
    enddef
  END
  writefile(vim9_lines, 'Xvim9_script.vim', 'D')

  var legacy_lines =<< trim END
    source Xvim9_script.vim

    call assert_false(exists('local'))
    call assert_false(exists('exported'))
    call assert_false(exists('s:exported'))
    call assert_equal('global', global)
    call assert_equal('global', g:global)
  END
  writefile(legacy_lines, 'Xlegacy_script.vim', 'D')

  source Xlegacy_script.vim
  assert_equal('global', g:global)
  unlet g:global

  legacy_lines =<< trim END
    import './Xvim9_script.vim'
    let g:global = s:Xvim9_script.GetText()
  END
  writefile(legacy_lines, 'Xlegacyimport.vim', 'D')
  source Xlegacyimport.vim
  assert_equal('text', g:global)
  unlet g:global
enddef

def Test_import_vim9_from_legacy()
  var vim9_lines =<< trim END
    vim9script
    var local = 'local'
    g:global = 'global'
    export var exported = 'exported'
    export def GetText(): string
       return 'text'
    enddef
  END
  writefile(vim9_lines, 'Xvim9_export.vim', 'D')

  var legacy_lines =<< trim END
    import './Xvim9_export.vim' as vim9

    call assert_false(exists('vim9'))
    call assert_false(exists('local'))
    call assert_false(exists('s:vim9.local'))
    call assert_equal('global', global)
    call assert_equal('global', g:global)
    call assert_false(exists('exported'))
    call assert_false(exists('s:exported'))
    call assert_false(exists('*GetText'))

    " imported symbol is script-local
    call assert_equal('exported', s:vim9.exported)
    call assert_equal('text', s:vim9.GetText())
  END
  writefile(legacy_lines, 'Xlegacy_script.vim', 'D')

  source Xlegacy_script.vim
  assert_equal('global', g:global)
  unlet g:global
enddef

def Test_cmdline_win()
  # if the Vim syntax highlighting uses Vim9 constructs they can be used from
  # the command line window.
  mkdir('rtp/syntax', 'pR')
  var export_lines =<< trim END
    vim9script
    export var That = 'yes'
  END
  writefile(export_lines, 'rtp/syntax/Xexport.vim')
  var import_lines =<< trim END
    vim9script
    import './Xexport.vim' as exp
    echo exp.That
  END
  writefile(import_lines, 'rtp/syntax/vim.vim')
  var save_rtp = &rtp
  &rtp = getcwd() .. '/rtp' .. ',' .. &rtp
  syntax on
  augroup CmdWin
    autocmd CmdwinEnter * g:got_there = 'yes'
  augroup END
  # this will open and also close the cmdline window
  feedkeys('q:', 'xt')
  assert_equal('yes', g:got_there)

  augroup CmdWin
    au!
  augroup END
  &rtp = save_rtp
enddef

def Test_import_gone_when_sourced_twice()
  var exportlines =<< trim END
      vim9script
      if exists('g:guard')
        finish
      endif
      g:guard = 1
      export var name = 'someName'
  END
  writefile(exportlines, 'XexportScript.vim', 'D')

  var lines =<< trim END
      vim9script
      import './XexportScript.vim' as expo
      def g:GetName(): string
        return expo.name
      enddef
  END
  writefile(lines, 'XscriptImport.vim', 'D')
  so XscriptImport.vim
  assert_equal('someName', g:GetName())

  so XexportScript.vim
  assert_fails('call g:GetName()', 'E1149:')

  delfunc g:GetName
  unlet g:guard
enddef

" test using an auto-loaded function and variable
def Test_vim9_autoload_full_name()
  var lines =<< trim END
     vim9script
     export def Gettest(): string
       return 'test'
     enddef
     g:some#name = 'name'
     g:some#dict = {key: 'value'}

     export def Varargs(a1: string, ...l: list<string>): string
       return a1 .. l[0] .. l[1]
     enddef
  END

  mkdir('Xfulldir/autoload', 'pR')
  writefile(lines, 'Xfulldir/autoload/some.vim')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xfulldir'

  assert_equal('test', g:some#Gettest())
  assert_equal('name', g:some#name)
  assert_equal('value', g:some#dict.key)
  g:some#other = 'other'
  assert_equal('other', g:some#other)

  assert_equal('abc', some#Varargs('a', 'b', 'c'))

  # upper case script name works
  lines =<< trim END
     vim9script
     export def GetOther(): string
       return 'other'
     enddef
  END
  writefile(lines, 'Xfulldir/autoload/Other.vim')
  assert_equal('other', g:Other#GetOther())

  &rtp = save_rtp
enddef

def Test_vim9script_autoload()
  mkdir('Xaldir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xaldir'

  # when the path has "/autoload/" prefix is not needed
  var lines =<< trim END
     vim9script
     g:prefixed_loaded += 1

     export def Gettest(): string
       return 'test'
     enddef

     export var name = 'name'

     export func GetFunc()
       return Gettest() .. 'more' .. s:name
     endfunc

     export def GetDef(): string
       return Gettest() .. 'more' .. name
     enddef

     export final fname = 'final'
     export const cname = 'const'
  END
  writefile(lines, 'Xaldir/autoload/prefixed.vim')

  g:prefixed_loaded = 0
  g:expected_loaded = 0
  lines =<< trim END
      vim9script
      import autoload 'prefixed.vim'
      assert_equal(g:expected_loaded, g:prefixed_loaded)
      assert_equal('test', prefixed.Gettest())
      assert_equal(1, g:prefixed_loaded)

      assert_equal('testmorename', prefixed.GetFunc())
      assert_equal('testmorename', prefixed.GetDef())
      assert_equal('name', prefixed.name)
      assert_equal('final', prefixed.fname)
      assert_equal('const', prefixed.cname)
  END
  v9.CheckScriptSuccess(lines)
  # can source it again, autoload script not loaded again
  g:expected_loaded = 1
  v9.CheckScriptSuccess(lines)

  # can also get the items by autoload name
  lines =<< trim END
      call assert_equal('test', prefixed#Gettest())
      call assert_equal('testmorename', prefixed#GetFunc())
      call assert_equal('name', prefixed#name)
      call assert_equal('final', prefixed#fname)
      call assert_equal('const', prefixed#cname)
  END
  v9.CheckScriptSuccess(lines)

  unlet g:prefixed_loaded
  unlet g:expected_loaded
  &rtp = save_rtp
enddef

def Test_import_autoload_not_exported()
  mkdir('Xnimdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xnimdir'

  # error when using an item that is not exported from an autoload script
  var exportLines =<< trim END
      vim9script
      var notExported = 123
      def NotExport()
        echo 'nop'
      enddef
  END
  writefile(exportLines, 'Xnimdir/autoload/notExport1.vim')

  var lines =<< trim END
      vim9script
      import autoload 'notExport1.vim'
      echo notExport1.notFound
  END
  v9.CheckScriptFailure(lines, 'E1048: Item not found in script: notFound')

  lines =<< trim END
      vim9script
      import autoload 'notExport1.vim'
      echo notExport1.notExported
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: notExported')

  lines =<< trim END
      vim9script
      import autoload 'notExport1.vim'
      echo notExport1.NotFunc()
  END
  v9.CheckScriptFailure(lines, 'E1048: Item not found in script: NotFunc')

  lines =<< trim END
      vim9script
      import autoload 'notExport1.vim'
      echo notExport1.NotExport()
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: NotExport')

  lines =<< trim END
      vim9script
      import autoload 'notExport1.vim'
      echo 'text'->notExport1.NotFunc()
  END
  v9.CheckScriptFailure(lines, 'E1048: Item not found in script: NotFunc')

  lines =<< trim END
      vim9script
      import autoload 'notExport1.vim'
      echo 'text'->notExport1.NotExport()
  END
  v9.CheckScriptFailure(lines, 'E1049: Item not exported in script: NotExport')

  # using a :def function we use a different autoload script every time so that
  # the function is compiled without the script loaded
  writefile(exportLines, 'Xnimdir/autoload/notExport2.vim')
  lines =<< trim END
      vim9script
      import autoload 'notExport2.vim'
      def Testit()
        echo notExport2.notFound
      enddef
      Testit()
  END
  v9.CheckScriptFailure(lines, 'E1048: Item not found in script: notExport2#notFound')

  writefile(exportLines, 'Xnimdir/autoload/notExport3.vim')
  lines =<< trim END
      vim9script
      import autoload 'notExport3.vim'
      def Testit()
        echo notExport3.notExported
      enddef
      Testit()
  END
  # don't get E1049 because it is too complicated to figure out
  v9.CheckScriptFailure(lines, 'E1048: Item not found in script: notExport3#notExported')

  writefile(exportLines, 'Xnimdir/autoload/notExport4.vim')
  lines =<< trim END
      vim9script
      import autoload 'notExport4.vim'
      def Testit()
        echo notExport4.NotFunc()
      enddef
      Testit()
  END
  v9.CheckScriptFailure(lines, 'E117: Unknown function: notExport4#NotFunc')

  writefile(exportLines, 'Xnimdir/autoload/notExport5.vim')
  lines =<< trim END
      vim9script
      import autoload 'notExport5.vim'
      def Testit()
        echo notExport5.NotExport()
      enddef
      Testit()
  END
  v9.CheckScriptFailure(lines, 'E117: Unknown function: notExport5#NotExport')

  writefile(exportLines, 'Xnimdir/autoload/notExport6.vim')
  lines =<< trim END
      vim9script
      import autoload 'notExport6.vim'
      def Testit()
        echo 'text'->notExport6.NotFunc()
      enddef
      Testit()
  END
  v9.CheckScriptFailure(lines, 'E117: Unknown function: notExport6#NotFunc')

  writefile(exportLines, 'Xnimdir/autoload/notExport7.vim')
  lines =<< trim END
      vim9script
      import autoload 'notExport7.vim'
      def Testit()
        echo 'text'->notExport7.NotExport()
      enddef
      Testit()
  END
  v9.CheckScriptFailure(lines, 'E117: Unknown function: notExport7#NotExport')

  &rtp = save_rtp
enddef

def Test_vim9script_autoload_call()
  mkdir('Xcalldir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xcalldir'

  var lines =<< trim END
     vim9script

     export def RetArg(arg: string): string
       return arg
     enddef

     export def Getother()
       g:result = 'other'
     enddef
  END
  writefile(lines, 'Xcalldir/autoload/another.vim')

  lines =<< trim END
      vim9script
      import autoload 'another.vim'

      # compile this before 'another.vim' is loaded
      def CallAnother()
        assert_equal('foo', 'foo'->another.RetArg())
      enddef
      CallAnother()

      call another.Getother()
      assert_equal('other', g:result)

      assert_equal('arg', call('another.RetArg', ['arg']))

      verbose function another.Getother
      # should we disallow this?
      verbose function another#Getother
  END
  v9.CheckScriptSuccess(lines)

  unlet g:result
  &rtp = save_rtp
enddef

def Test_vim9script_noclear_autoload()
  mkdir('Xnocdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xnocdir'

  var lines =<< trim END
      vim9script
      export def Func(): string
        return 'called'
      enddef
      g:double_loaded = 'yes'
  END
  writefile(lines, 'Xnocdir/autoload/double.vim')

  lines =<< trim END
      vim9script noclear
      if exists('g:script_loaded')
        finish
      endif
      g:script_loaded = true

      import autoload 'double.vim'
      nnoremap <F3> <ScriptCmd>g:result = double.Func()<CR>
  END
  g:double_loaded = 'no'
  writefile(lines, 'Xloaddouble', 'D')
  source Xloaddouble
  assert_equal('no', g:double_loaded)
  assert_equal(true, g:script_loaded)
  source Xloaddouble
  feedkeys("\<F3>", 'xt')
  assert_equal('called', g:result)
  assert_equal('yes', g:double_loaded)

  unlet g:double_loaded
  unlet g:script_loaded
  unlet g:result
  &rtp = save_rtp
enddef

def Test_vim9script_autoload_duplicate()
  mkdir('Xdupdir/autoload', 'pR')

  var lines =<< trim END
     vim9script

     export def Func()
     enddef

     def Func()
     enddef
  END
  writefile(lines, 'Xdupdir/autoload/dupfunc.vim')
  assert_fails('source Xdupdir/autoload/dupfunc.vim', 'E1073:')

  lines =<< trim END
     vim9script

     def Func()
     enddef

     export def Func()
     enddef
  END
  writefile(lines, 'Xdupdir/autoload/dup2func.vim')
  assert_fails('source Xdupdir/autoload/dup2func.vim', 'E1073:')

  lines =<< trim END
     vim9script

     def Func()
     enddef

     export var Func = 'asdf'
  END
  writefile(lines, 'Xdupdir/autoload/dup3func.vim')
  assert_fails('source Xdupdir/autoload/dup3func.vim', 'E1041: Redefining script item: "Func"')

  lines =<< trim END
     vim9script

     export var Func = 'asdf'

     def Func()
     enddef
  END
  writefile(lines, 'Xdupdir/autoload/dup4func.vim')
  assert_fails('source Xdupdir/autoload/dup4func.vim', 'E1041:')

  lines =<< trim END
     vim9script

     var Func = 'asdf'

     export def Func()
     enddef
  END
  writefile(lines, 'Xdupdir/autoload/dup5func.vim')
  assert_fails('source Xdupdir/autoload/dup5func.vim', 'E707:')

  lines =<< trim END
     vim9script

     export def Func()
     enddef

     var Func = 'asdf'
  END
  writefile(lines, 'Xdupdir/autoload/dup6func.vim')
  assert_fails('source Xdupdir/autoload/dup6func.vim', 'E1041: Redefining script item: "Func"')
enddef

def Test_autoload_missing_function_name()
  mkdir('Xmisdir/autoload', 'pR')

  var lines =<< trim END
     vim9script

     def loadme#()
     enddef
  END
  writefile(lines, 'Xmisdir/autoload/loadme.vim')
  assert_fails('source Xmisdir/autoload/loadme.vim', 'E129:')
enddef

def Test_autoload_name_wrong()
  var lines =<< trim END
     def Xscriptname#Func()
     enddef
  END
  writefile(lines, 'Xscriptname.vim', 'D')
  v9.CheckScriptFailure(lines, 'E746:')

  mkdir('Xwrodir/autoload', 'pR')
  lines =<< trim END
     vim9script
     def somescript#Func()
     enddef
  END
  writefile(lines, 'Xwrodir/autoload/somescript.vim')
  assert_fails('source Xwrodir/autoload/somescript.vim', 'E1263:')

  delete('Xwrodir', 'rf')
enddef

def Test_import_autoload_postponed()
  mkdir('Xpostdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xpostdir'

  var lines =<< trim END
      vim9script

      g:loaded_postponed = 'true'
      export var variable = 'bla'
      export def Function(): string
        return 'bla'
      enddef
  END
  writefile(lines, 'Xpostdir/autoload/postponed.vim')

  lines =<< trim END
      vim9script

      import autoload 'postponed.vim'
      def Tryit()
        echo postponed.variable
        echo postponed.Function()
      enddef
      defcompile
  END
  v9.CheckScriptSuccess(lines)
  assert_false(exists('g:loaded_postponed'))
  v9.CheckScriptSuccess(lines + ['Tryit()'])
  assert_equal('true', g:loaded_postponed)

  unlet g:loaded_postponed
  &rtp = save_rtp
enddef

def Test_import_autoload_override()
  mkdir('Xoverdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xoverdir'
  test_override('autoload', 1)

  var lines =<< trim END
      vim9script

      g:loaded_override = 'true'
      export var variable = 'bla'
      export def Function(): string
        return 'bla'
      enddef
  END
  writefile(lines, 'Xoverdir/autoload/override.vim')

  lines =<< trim END
      vim9script

      import autoload 'override.vim'
      assert_equal('true', g:loaded_override)

      def Tryit()
        echo override.doesNotExist
      enddef
      defcompile
  END
  v9.CheckScriptFailure(lines, 'E1048: Item not found in script: doesNotExist', 1)

  test_override('autoload', 0)
  unlet g:loaded_override
  &rtp = save_rtp
enddef

def Test_autoload_mapping()
  mkdir('Xmapdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xmapdir'

  var lines =<< trim END
      vim9script

      g:toggle_loaded = 'yes'

      export def Toggle(): string
        return ":g:toggle_called = 'yes'\<CR>"
      enddef
      export def Doit()
        g:doit_called = 'yes'
      enddef
  END
  writefile(lines, 'Xmapdir/autoload/toggle.vim')

  lines =<< trim END
      vim9script

      import autoload 'toggle.vim'

      nnoremap <silent> <expr> tt toggle.Toggle()
      nnoremap <silent> xx <ScriptCmd>toggle.Doit()<CR>
      nnoremap <silent> yy <Cmd>toggle.Doit()<CR>
  END
  v9.CheckScriptSuccess(lines)
  assert_false(exists("g:toggle_loaded"))
  assert_false(exists("g:toggle_called"))
  assert_match('\d A: \f*[/\\]toggle.vim', execute('scriptnames'))

  feedkeys("tt", 'xt')
  assert_equal('yes', g:toggle_loaded)
  assert_equal('yes', g:toggle_called)
  assert_match('\d: \f*[/\\]toggle.vim', execute('scriptnames'))

  feedkeys("xx", 'xt')
  assert_equal('yes', g:doit_called)

  assert_fails('call feedkeys("yy", "xt")', 'E121: Undefined variable: toggle')

  nunmap tt
  nunmap xx
  nunmap yy
  unlet g:toggle_loaded
  unlet g:toggle_called
  &rtp = save_rtp
enddef

def Test_vim9script_autoload_fails()
  var lines =<< trim END
      vim9script autoload
      var n = 0
  END
  v9.CheckScriptFailure(lines, 'E475: Invalid argument: autoload')

  lines =<< trim END
      vim9script noclear noclear
      var n = 0
  END
  v9.CheckScriptFailure(lines, 'E983: Duplicate argument: noclear')

  lines =<< trim END
      vim9script noclears
      var n = 0
  END
  v9.CheckScriptFailure(lines, 'E475: Invalid argument: noclears')
enddef

def Test_import_autoload_fails()
  var lines =<< trim END
      vim9script
      import autoload autoload 'prefixed.vim'
  END
  v9.CheckScriptFailure(lines, 'E121: Undefined variable: autoload')

  lines =<< trim END
      vim9script
      import autoload './doesNotExist.vim'
  END
  v9.CheckScriptFailure(lines, 'E282:', 2)

  lines =<< trim END
      vim9script
      import autoload '/dir/doesNotExist.vim'
  END
  v9.CheckScriptFailure(lines, 'E282:', 2)

  lines =<< trim END
      vim9script
      import autoload '../testdir'
  END
  v9.CheckScriptFailure(lines, 'E17:', 2)

  lines =<< trim END
      vim9script
      import autoload 'doesNotExist.vim'
  END
  v9.CheckScriptFailure(lines, 'E1053: Could not import "doesNotExist.vim"')
enddef

" test disassembling an auto-loaded function starting with "debug"
def Test_vim9_autoload_disass()
  mkdir('Xdasdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xdasdir'

  var lines =<< trim END
     vim9script
     export def Test(): string
       return 'debug'
     enddef
  END
  writefile(lines, 'Xdasdir/autoload/debugit.vim')

  lines =<< trim END
     vim9script
     export def Test(): string
       return 'profile'
     enddef
  END
  writefile(lines, 'Xdasdir/autoload/profileit.vim')

  lines =<< trim END
    vim9script
    assert_equal('debug', debugit#Test())
    disass debugit#Test
    assert_equal('profile', profileit#Test())
    disass profileit#Test
  END
  v9.CheckScriptSuccess(lines)

  &rtp = save_rtp
enddef

" test using a vim9script that is auto-loaded from an autocmd
def Test_vim9_aucmd_autoload()
  var lines =<< trim END
     vim9script
     export def Test()
         echomsg getreg('"')
     enddef
  END

  mkdir('Xauldir/autoload', 'pR')
  writefile(lines, 'Xauldir/autoload/foo.vim')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xauldir'
  augroup test
    autocmd TextYankPost * call foo#Test()
  augroup END

  normal Y

  augroup test
    autocmd!
  augroup END
  &rtp = save_rtp
enddef

" test using a autoloaded file that is case sensitive
def Test_vim9_autoload_case_sensitive()
  var lines =<< trim END
     vim9script
     export def CaseSensitive(): string
       return 'done'
     enddef
  END

  mkdir('Xcasedir/autoload', 'pR')
  writefile(lines, 'Xcasedir/autoload/CaseSensitive.vim')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xcasedir'

  lines =<< trim END
      vim9script
      import autoload 'CaseSensitive.vim'
      assert_equal('done', CaseSensitive.CaseSensitive())
  END
  v9.CheckScriptSuccess(lines)

  if !has('fname_case')
    lines =<< trim END
        vim9script
        import autoload 'CaseSensitive.vim'
        import autoload 'casesensitive.vim'
    END
    v9.CheckScriptFailure(lines, 'E1262:')
  endif

  &rtp = save_rtp
enddef

" This was causing a crash because suppress_errthrow wasn't reset.
def Test_vim9_autoload_error()
  var lines =<< trim END
      vim9script
      def crash#func()
          try
              for x in List()
              endfor
          catch
          endtry
          g:ok = true
      enddef
      fu List()
          invalid
      endfu
      try
          alsoinvalid
      catch /wontmatch/
      endtry
  END
  call mkdir('Xruntime/autoload', 'pR')
  call writefile(lines, 'Xruntime/autoload/crash.vim')

  # run in a separate Vim to avoid the side effects of assert_fails()
  lines =<< trim END
    exe 'set rtp^=' .. getcwd() .. '/Xruntime'
    call crash#func()
    call writefile(['ok'], 'Xdidit')
    qall!
  END
  writefile(lines, 'Xscript', 'D')
  g:RunVim([], [], '-S Xscript')
  assert_equal(['ok'], readfile('Xdidit'))

  delete('Xdidit')

  lines =<< trim END
    vim9script
    var foo#bar = 'asdf'
  END
  v9.CheckScriptFailure(lines, 'E461: Illegal variable name: foo#bar', 2)
enddef

def Test_vim9_import_symlink()
  if !has('unix')
    CheckUnix
  else
    mkdir('Xto/plugin', 'pR')
    var lines =<< trim END
        vim9script
        import autoload 'bar.vim'
        g:resultFunc = bar.Func()
        g:resultValue = bar.value
    END
    writefile(lines, 'Xto/plugin/foo.vim')

    mkdir('Xto/autoload', 'pR')
    lines =<< trim END
        vim9script
        export def Func(): string
          return 'func'
        enddef
        export var value = 'val'
    END
    writefile(lines, 'Xto/autoload/bar.vim')

    var save_rtp = &rtp
    &rtp = getcwd() .. '/Xfrom'
    system('ln -s ' .. getcwd() .. '/Xto Xfrom')

    source Xfrom/plugin/foo.vim
    assert_equal('func', g:resultFunc)
    assert_equal('val', g:resultValue)

    var infoTo = getscriptinfo()->filter((_, v) => v.name =~ 'Xto/autoload/bar')
    var infoFrom = getscriptinfo()->filter((_, v) => v.name =~ 'Xfrom/autoload/bar')
    assert_equal(1, len(infoTo))
    assert_equal(1, len(infoFrom))
    assert_equal(infoTo[0].sid, infoFrom[0].sourced)
    var output: string
    redir => output
    scriptnames
    redir END
    assert_match(infoFrom[0].sid .. '->' .. infoFrom[0].sourced .. '.*Xfrom', output)

    unlet g:resultFunc
    unlet g:resultValue
    &rtp = save_rtp
    delete('Xfrom', 'rf')
  endif
enddef

def Test_export_in_conditional_block()
  var lines =<< trim END
      vim9script
      if exists('this_will_fail')
        export var MyVar = "hello"
      endif
  END
  v9.CheckScriptSuccess(lines)
enddef


" vim: ts=8 sw=2 sts=2 expandtab tw=80 fdm=marker
