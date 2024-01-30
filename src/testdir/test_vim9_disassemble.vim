" Test the :disassemble command, and compilation as a side effect

source check.vim
import './vim9.vim' as v9

func s:NotCompiled()
  echo "not"
endfunc

let s:scriptvar = 4
let g:globalvar = 'g'
let b:buffervar = 'b'
let w:windowvar = 'w'
let t:tabpagevar = 't'

def s:ScriptFuncLoad(arg: string)
  var local = 1
  buffers
  echo
  echo arg
  echo local
  echo &lines
  echo v:version
  echo s:scriptvar
  echo g:globalvar
  echo get(g:, "global")
  echo g:auto#var
  echo b:buffervar
  echo get(b:, "buffer")
  echo w:windowvar
  echo get(w:, "window")
  echo t:tabpagevar
  echo get(t:, "tab")
  echo &tabstop
  echo $ENVVAR
  echo @z
enddef

def Test_disassemble_load()
  assert_fails('disass NoFunc', 'E1061:')
  assert_fails('disass NotCompiled', 'E1091:')
  assert_fails('disass', 'E471:')
  assert_fails('disass [', 'E475:')
  assert_fails('disass 234', 'E129:')
  assert_fails('disass <XX>foo', 'E129:')
  assert_fails('disass Test_disassemble_load burp', 'E488:')
  assert_fails('disass debug debug Test_disassemble_load', 'E488:')
  assert_fails('disass profile profile Test_disassemble_load', 'E488:')

  var res = execute('disass s:ScriptFuncLoad')
  assert_match('<SNR>\d*_ScriptFuncLoad.*' ..
        'buffers\_s*' ..
        '\d\+ EXEC \+buffers\_s*' ..
        'echo\_s*' ..
        'echo arg\_s*' ..
        '\d\+ LOAD arg\[-1\]\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'echo local\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'echo &lines\_s*' ..
        '\d\+ LOADOPT &lines\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'echo v:version\_s*' ..
        '\d\+ LOADV v:version\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'echo s:scriptvar\_s*' ..
        '\d\+ LOADS s:scriptvar from .*test_vim9_disassemble.vim\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'echo g:globalvar\_s*' ..
        '\d\+ LOADG g:globalvar\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'echo get(g:, "global")\_s*' ..
        '\d\+ LOAD g:\_s*' ..
        '\d\+ PUSHS "global"\_s*' ..
        '\d\+ BCALL get(argc 2)\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'echo g:auto#var\_s*' ..
        '\d\+ LOADAUTO g:auto#var\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'echo b:buffervar\_s*' ..
        '\d\+ LOADB b:buffervar\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'echo get(b:, "buffer")\_s*' ..
        '\d\+ LOAD b:\_s*' ..
        '\d\+ PUSHS "buffer"\_s*' ..
        '\d\+ BCALL get(argc 2).*' ..
        ' LOADW w:windowvar.*' ..
        'echo get(w:, "window")\_s*' ..
        '\d\+ LOAD w:\_s*' ..
        '\d\+ PUSHS "window"\_s*' ..
        '\d\+ BCALL get(argc 2).*' ..
        ' LOADT t:tabpagevar.*' ..
        'echo get(t:, "tab")\_s*' ..
        '\d\+ LOAD t:\_s*' ..
        '\d\+ PUSHS "tab"\_s*' ..
        '\d\+ BCALL get(argc 2).*' ..
        ' LOADENV $ENVVAR.*' ..
        ' LOADREG @z.*',
        res)
enddef

def s:EditExpand()
  var filename = "file"
  var filenr = 123
  edit the`=filename``=filenr`.txt
enddef

def Test_disassemble_exec_expr()
  var res = execute('disass s:EditExpand')
  assert_match('<SNR>\d*_EditExpand\_s*' ..
        ' var filename = "file"\_s*' ..
        '\d PUSHS "file"\_s*' ..
        '\d STORE $0\_s*' ..
        ' var filenr = 123\_s*' ..
        '\d STORE 123 in $1\_s*' ..
        ' edit the`=filename``=filenr`.txt\_s*' ..
        '\d PUSHS "edit the"\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d LOAD $1\_s*' ..
        '\d 2STRING stack\[-1\]\_s*' ..
        '\d\+ PUSHS ".txt"\_s*' ..
        '\d\+ EXECCONCAT 4\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

if has('python3')
  def s:PyHeredoc()
    python3 << EOF
      print('hello')
EOF
  enddef

  def Test_disassemble_python_heredoc()
    var res = execute('disass s:PyHeredoc')
    assert_match('<SNR>\d*_PyHeredoc.*' ..
          "    python3 << EOF^@      print('hello')^@EOF\\_s*" ..
          '\d EXEC_SPLIT     python3 << EOF^@      print(''hello'')^@EOF\_s*' ..
          '\d RETURN void',
          res)
  enddef
endif

def s:Substitute()
  var expr = "abc"
  :%s/a/\=expr/&g#c
enddef

def Test_disassemble_substitute()
  var res = execute('disass s:Substitute')
  assert_match('<SNR>\d*_Substitute.*' ..
        ' var expr = "abc"\_s*' ..
        '\d PUSHS "abc"\_s*' ..
        '\d STORE $0\_s*' ..
        ' :%s/a/\\=expr/&g#c\_s*' ..
        '\d SUBSTITUTE   :%s/a/\\=expr/&g#c\_s*' ..
        '    0 LOAD $0\_s*' ..
        '    -------------\_s*' ..
        '\d RETURN void',
        res)
enddef


def s:SearchPair()
  var col = 8
  searchpair("{", "", "}", "", "col('.') > col")
enddef

def Test_disassemble_seachpair()
  var res = execute('disass s:SearchPair')
  assert_match('<SNR>\d*_SearchPair.*' ..
        ' var col = 8\_s*' ..
        '\d STORE 8 in $0\_s*' ..
        ' searchpair("{", "", "}", "", "col(''.'') > col")\_s*' ..
        '\d PUSHS "{"\_s*' ..
        '\d PUSHS ""\_s*' ..
        '\d PUSHS "}"\_s*' ..
        '\d PUSHS ""\_s*' ..
        '\d INSTR\_s*' ..
        '  0 PUSHS "."\_s*' ..
        '  1 BCALL col(argc 1)\_s*' ..
        '  2 LOAD $0\_s*' ..
        '  3 COMPARENR >\_s*' ..
        ' -------------\_s*' ..
        '\d BCALL searchpair(argc 5)\_s*' ..
        '\d DROP\_s*' ..
        '\d RETURN void',
        res)
enddef


def s:SubstituteExpr()
    substitute('a', 'b', '\=123', 'g')
enddef

def Test_disassemble_substitute_expr()
  var res = execute('disass s:SubstituteExpr')
  assert_match('<SNR>\d*_SubstituteExpr.*' ..
        'substitute(''a'', ''b'', ''\\=123'', ''g'')\_s*' ..
        '\d PUSHS "a"\_s*' ..
        '\d PUSHS "b"\_s*' ..
        '\d INSTR\_s*' ..
        '  0 PUSHNR 123\_s*' ..
        ' -------------\_s*' ..
        '\d PUSHS "g"\_s*' ..
        '\d BCALL substitute(argc 4)\_s*' ..
        '\d DROP\_s*' ..
        '\d RETURN void',
        res)
enddef

def s:RedirVar()
  var result: string
  redir =>> result
    echo "text"
  redir END
enddef

def Test_disassemble_redir_var()
  var res = execute('disass s:RedirVar')
  assert_match('<SNR>\d*_RedirVar.*' ..
        ' var result: string\_s*' ..
        '\d PUSHS "\[NULL\]"\_s*' ..
        '\d STORE $0\_s*' ..
        ' redir =>> result\_s*' ..
        '\d REDIR\_s*' ..
        ' echo "text"\_s*' ..
        '\d PUSHS "text"\_s*' ..
        '\d ECHO 1\_s*' ..
        ' redir END\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d REDIR END\_s*' ..
        '\d CONCAT size 2\_s*' ..
        '\d STORE $0\_s*' ..
        '\d RETURN void',
        res)
enddef

def s:Cexpr()
  var errors = "list of errors"
  cexpr errors
enddef

def Test_disassemble_cexpr()
  var res = execute('disass s:Cexpr')
  assert_match('<SNR>\d*_Cexpr.*' ..
        ' var errors = "list of errors"\_s*' ..
        '\d PUSHS "list of errors"\_s*' ..
        '\d STORE $0\_s*' ..
        ' cexpr errors\_s*' ..
        '\d CEXPR pre cexpr\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d CEXPR core cexpr "cexpr errors"\_s*' ..
        '\d RETURN void',
        res)
enddef

def s:YankRange()
  norm! m[jjm]
  :'[,']yank
enddef

def Test_disassemble_yank_range()
  var res = execute('disass s:YankRange')
  assert_match('<SNR>\d*_YankRange.*' ..
        ' norm! m\[jjm\]\_s*' ..
        '\d EXEC   norm! m\[jjm\]\_s*' ..
        '  :''\[,''\]yank\_s*' ..
        '\d EXEC   :''\[,''\]yank\_s*' ..
        '\d RETURN void',
        res)
enddef

def s:PutExpr()
  :3put ="text"
enddef

def Test_disassemble_put_expr()
  var res = execute('disass s:PutExpr')
  assert_match('<SNR>\d*_PutExpr.*' ..
        ' :3put ="text"\_s*' ..
        '\d PUSHS "text"\_s*' ..
        '\d PUT = 3\_s*' ..
        '\d RETURN void',
        res)
enddef

def s:PutRange()
  :$-2put a
  :$-3put! b
enddef

def Test_disassemble_put_range()
  var res = execute('disass s:PutRange')
  assert_match('<SNR>\d*_PutRange.*' ..
        ' :$-2put a\_s*' ..
        '\d RANGE $-2\_s*' ..
        '\d PUT a range\_s*' ..

        ' :$-3put! b\_s*' ..
        '\d RANGE $-3\_s*' ..
        '\d PUT b above range\_s*' ..
        '\d RETURN void',
        res)
enddef

def s:ScriptFuncPush()
  var localbool = true
  var localspec = v:none
  var localblob = 0z1234
  var localfloat = 1.234
enddef

def Test_disassemble_push()
  mkdir('Xdisdir/autoload', 'pR')
  var save_rtp = &rtp
  exe 'set rtp^=' .. getcwd() .. '/Xdisdir'

  var lines =<< trim END
      vim9script
  END
  writefile(lines, 'Xdisdir/autoload/autoscript.vim')

  lines =<< trim END
      vim9script
      import autoload 'autoscript.vim'

      def AutoloadFunc()
        &operatorfunc = autoscript.Opfunc
      enddef

      var res = execute('disass AutoloadFunc')
      assert_match('<SNR>\d*_AutoloadFunc.*' ..
            '&operatorfunc = autoscript.Opfunc\_s*' ..
            '0 AUTOLOAD autoscript#Opfunc\_s*' ..
            '1 STOREFUNCOPT &operatorfunc\_s*' ..
            '2 RETURN void',
            res)
  END
  v9.CheckScriptSuccess(lines)

  &rtp = save_rtp
enddef

def Test_disassemble_import_autoload()
  writefile(['vim9script'], 'XimportAL.vim', 'D')

  var lines =<< trim END
      vim9script
      import autoload './XimportAL.vim'

      def AutoloadFunc()
        echo XimportAL.SomeFunc()
        echo XimportAL.someVar
        XimportAL.someVar = "yes"
      enddef

      var res = execute('disass AutoloadFunc')
      assert_match('<SNR>\d*_AutoloadFunc.*' ..
            'echo XimportAL.SomeFunc()\_s*' ..
            '\d SOURCE .*/testdir/XimportAL.vim\_s*' ..
            '\d PUSHFUNC "<80><fd>R\d\+_SomeFunc"\_s*' ..
            '\d PCALL top (argc 0)\_s*' ..
            '\d PCALL end\_s*' ..
            '\d ECHO 1\_s*' ..

            'echo XimportAL.someVar\_s*' ..
            '\d SOURCE .*/testdir/XimportAL.vim\_s*' ..
            '\d LOADEXPORT s:someVar from .*/testdir/XimportAL.vim\_s*' ..
            '\d ECHO 1\_s*' ..

            'XimportAL.someVar = "yes"\_s*' ..
            '\d\+ PUSHS "yes"\_s*' ..
            '\d\+ SOURCE .*/testdir/XimportAL.vim\_s*' ..
            '\d\+ STOREEXPORT someVar in .*/testdir/XimportAL.vim\_s*' ..

            '\d\+ RETURN void',
            res)
  END
  v9.CheckScriptSuccess(lines)
enddef

def s:ScriptFuncStore()
  var localnr = 1
  localnr = 2
  var localstr = 'abc'
  localstr = 'xyz'
  v:char = 'abc'
  s:scriptvar = 'sv'
  g:globalvar = 'gv'
  g:auto#var = 'av'
  b:buffervar = 'bv'
  w:windowvar = 'wv'
  t:tabpagevar = 'tv'
  &tabstop = 8
  &opfunc = (t) => len(t)
  $ENVVAR = 'ev'
  @z = 'rv'
enddef

def Test_disassemble_store()
  var res = execute('disass s:ScriptFuncStore')
  assert_match('<SNR>\d*_ScriptFuncStore.*' ..
        'var localnr = 1.*' ..
        'localnr = 2.*' ..
        ' STORE 2 in $0.*' ..
        'var localstr = ''abc''.*' ..
        'localstr = ''xyz''.*' ..
        ' STORE $1.*' ..
        'v:char = ''abc''.*' ..
        'STOREV v:char.*' ..
        's:scriptvar = ''sv''.*' ..
        ' STORES s:scriptvar in .*test_vim9_disassemble.vim.*' ..
        'g:globalvar = ''gv''.*' ..
        ' STOREG g:globalvar.*' ..
        'g:auto#var = ''av''.*' ..
        ' STOREAUTO g:auto#var.*' ..
        'b:buffervar = ''bv''.*' ..
        ' STOREB b:buffervar.*' ..
        'w:windowvar = ''wv''.*' ..
        ' STOREW w:windowvar.*' ..
        't:tabpagevar = ''tv''.*' ..
        ' STORET t:tabpagevar.*' ..
        '&tabstop = 8\_s*' ..
        '\d\+ PUSHNR 8\_s*' ..
        '\d\+ STOREOPT &tabstop\_s*' ..
        '&opfunc = (t) => len(t)\_s*' ..
        '\d\+ FUNCREF <lambda>\d\+\_s*' ..
        '\d\+ STOREFUNCOPT &opfunc\_s*' ..
        '$ENVVAR = ''ev''\_s*' ..
        '\d\+ PUSHS "ev"\_s*' ..
        '\d\+ STOREENV $ENVVAR\_s*' ..
        '@z = ''rv''.*' ..
        '\d\+ STOREREG @z.*',
        res)
enddef

def s:ScriptFuncStoreMember()
  var locallist: list<number> = []
  locallist[0] = 123
  var localdict: dict<number> = {}
  localdict["a"] = 456
  var localblob: blob = 0z1122
  localblob[1] = 33
enddef

def Test_disassemble_store_member()
  var res = execute('disass s:ScriptFuncStoreMember')
  assert_match('<SNR>\d*_ScriptFuncStoreMember\_s*' ..
        'var locallist: list<number> = []\_s*' ..
        '\d NEWLIST size 0\_s*' ..
        '\d SETTYPE list<number>\_s*' ..
        '\d STORE $0\_s*' ..
        'locallist\[0\] = 123\_s*' ..
        '\d PUSHNR 123\_s*' ..
        '\d PUSHNR 0\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d STOREINDEX list\_s*' ..
        'var localdict: dict<number> = {}\_s*' ..
        '\d NEWDICT size 0\_s*' ..
        '\d SETTYPE dict<number>\_s*' ..
        '\d STORE $1\_s*' ..
        'localdict\["a"\] = 456\_s*' ..
        '\d\+ PUSHNR 456\_s*' ..
        '\d\+ PUSHS "a"\_s*' ..
        '\d\+ LOAD $1\_s*' ..
        '\d\+ STOREINDEX dict\_s*' ..
        'var localblob: blob = 0z1122\_s*' ..
        '\d\+ PUSHBLOB 0z1122\_s*' ..
        '\d\+ STORE $2\_s*' ..
        'localblob\[1\] = 33\_s*' ..
        '\d\+ PUSHNR 33\_s*' ..
        '\d\+ PUSHNR 1\_s*' ..
        '\d\+ LOAD $2\_s*' ..
        '\d\+ STOREINDEX blob\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

if has('job')
  def s:StoreNull()
    var ss = null_string
    var bb = null_blob
    var dd = null_dict
    var ll = null_list
    var Ff = null_function
    var Pp = null_partial
    var jj = null_job
    var cc = null_channel
    var oo = null_object
    var nc = null_class
  enddef

  def Test_disassemble_assign_null()
    var res = execute('disass s:StoreNull')
    assert_match('<SNR>\d*_StoreNull\_s*' ..
          'var ss = null_string\_s*' ..
          '\d\+ PUSHS "\[NULL\]"\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          'var bb = null_blob\_s*' ..
          '\d\+ PUSHBLOB 0z\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          'var dd = null_dict\_s*' ..
          '\d\+ NEWDICT size -1\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          'var ll = null_list\_s*' ..
          '\d\+ NEWLIST size -1\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          'var Ff = null_function\_s*' ..
          '\d\+ PUSHFUNC "\[none\]"\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          'var Pp = null_partial\_s*' ..
          '\d\+ NEWPARTIAL\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          'var jj = null_job\_s*' ..
          '\d\+ PUSHJOB "no process"\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          'var cc = null_channel\_s*' ..
          '\d\+ PUSHCHANNEL 0\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          'var oo = null_object\_s*' ..
          '\d\+ PUSHOBJ null\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          'var nc = null_class\_s*' ..
          '\d\+ PUSHCLASS null\_s*' ..
          '\d\+ STORE $\d\_s*' ..

          '\d\+ RETURN void',
          res)
  enddef
endif

def s:ScriptFuncStoreIndex()
  var d = {dd: {}}
  d.dd[0] = 0
enddef

def Test_disassemble_store_index()
  var res = execute('disass s:ScriptFuncStoreIndex')
  assert_match('<SNR>\d*_ScriptFuncStoreIndex\_s*' ..
        'var d = {dd: {}}\_s*' ..
        '\d PUSHS "dd"\_s*' ..
        '\d NEWDICT size 0\_s*' ..
        '\d NEWDICT size 1\_s*' ..
        '\d SETTYPE dict<dict<any>>\_s*' ..
        '\d STORE $0\_s*' ..
        'd.dd\[0\] = 0\_s*' ..
        '\d PUSHNR 0\_s*' ..
        '\d PUSHNR 0\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d MEMBER dd\_s*' ..
        '\d\+ USEDICT\_s*' ..
        '\d\+ STOREINDEX any\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:ListAssign()
  var x: string
  var y: string
  var l: list<any>
  [x, y; l] = g:stringlist
enddef

def Test_disassemble_list_assign()
  var res = execute('disass s:ListAssign')
  assert_match('<SNR>\d*_ListAssign\_s*' ..
        'var x: string\_s*' ..
        '\d PUSHS "\[NULL\]"\_s*' ..
        '\d STORE $0\_s*' ..
        'var y: string\_s*' ..
        '\d PUSHS "\[NULL\]"\_s*' ..
        '\d STORE $1\_s*' ..
        'var l: list<any>\_s*' ..
        '\d NEWLIST size 0\_s*' ..
        '\d STORE $2\_s*' ..
        '\[x, y; l\] = g:stringlist\_s*' ..
        '\d LOADG g:stringlist\_s*' ..
        '\d CHECKTYPE list<any> stack\[-1\]\_s*' ..
        '\d CHECKLEN >= 2\_s*' ..
        '\d\+ ITEM 0\_s*' ..
        '\d\+ CHECKTYPE string stack\[-1\] var 1\_s*' ..
        '\d\+ STORE $0\_s*' ..
        '\d\+ ITEM 1\_s*' ..
        '\d\+ CHECKTYPE string stack\[-1\] var 2\_s*' ..
        '\d\+ STORE $1\_s*' ..
        '\d\+ SLICE 2\_s*' ..
        '\d\+ STORE $2\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:ListAssignWithOp()
  var a = 2
  var b = 3
  [a, b] += [4, 5]
enddef

def Test_disassemble_list_assign_with_op()
  var res = execute('disass s:ListAssignWithOp')
  assert_match('<SNR>\d*_ListAssignWithOp\_s*' ..
        'var a = 2\_s*' ..
        '\d STORE 2 in $0\_s*' ..
        'var b = 3\_s*' ..
        '\d STORE 3 in $1\_s*' ..
        '\[a, b\] += \[4, 5\]\_s*' ..
        '\d\+ PUSHNR 4\_s*' ..
        '\d\+ PUSHNR 5\_s*' ..
        '\d\+ NEWLIST size 2\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ ITEM 0 with op\_s*' ..
        '\d\+ OPNR +\_s*' ..
        '\d\+ STORE $0\_s*' ..
        '\d\+ LOAD $1\_s*' ..
        '\d\+ ITEM 1 with op\_s*' ..
        '\d\+ OPNR +\_s*' ..
        '\d\+ STORE $1\_s*' ..
        '\d\+ DROP\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:ListAdd()
  var l: list<number> = []
  add(l, 123)
  add(l, g:aNumber)
enddef

def Test_disassemble_list_add()
  var res = execute('disass s:ListAdd')
  assert_match('<SNR>\d*_ListAdd\_s*' ..
        'var l: list<number> = []\_s*' ..
        '\d NEWLIST size 0\_s*' ..
        '\d SETTYPE list<number>\_s*' ..
        '\d STORE $0\_s*' ..
        'add(l, 123)\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d PUSHNR 123\_s*' ..
        '\d LISTAPPEND\_s*' ..
        '\d DROP\_s*' ..
        'add(l, g:aNumber)\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d\+ LOADG g:aNumber\_s*' ..
        '\d\+ CHECKTYPE number stack\[-1\]\_s*' ..
        '\d\+ LISTAPPEND\_s*' ..
        '\d\+ DROP\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:BlobAdd()
  var b: blob = 0z
  add(b, 123)
  add(b, g:aNumber)
enddef

def Test_disassemble_blob_add()
  var res = execute('disass s:BlobAdd')
  assert_match('<SNR>\d*_BlobAdd\_s*' ..
        'var b: blob = 0z\_s*' ..
        '\d PUSHBLOB 0z\_s*' ..
        '\d STORE $0\_s*' ..
        'add(b, 123)\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d PUSHNR 123\_s*' ..
        '\d BLOBAPPEND\_s*' ..
        '\d DROP\_s*' ..
        'add(b, g:aNumber)\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d\+ LOADG g:aNumber\_s*' ..
        '\d\+ CHECKTYPE number stack\[-1\]\_s*' ..
        '\d\+ BLOBAPPEND\_s*' ..
        '\d\+ DROP\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:BlobIndexSlice()
  var b: blob = 0z112233
  echo b[1]
  echo b[1 : 2]
enddef

def Test_disassemble_blob_index_slice()
  var res = execute('disass s:BlobIndexSlice')
  assert_match('<SNR>\d*_BlobIndexSlice\_s*' ..
        'var b: blob = 0z112233\_s*' ..
        '\d PUSHBLOB 0z112233\_s*' ..
        '\d STORE $0\_s*' ..
        'echo b\[1\]\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d BLOBINDEX\_s*' ..
        '\d ECHO 1\_s*' ..
        'echo b\[1 : 2\]\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d\+ PUSHNR 2\_s*' ..
        '\d\+ BLOBSLICE\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:ScriptFuncUnlet()
  g:somevar = "value"
  unlet g:somevar
  unlet! g:somevar
  unlet $SOMEVAR

  var l = [1, 2, 3]
  unlet l[2]
  unlet l[0 : 1]
enddef

def Test_disassemble_unlet()
  var res = execute('disass s:ScriptFuncUnlet')
  assert_match('<SNR>\d*_ScriptFuncUnlet\_s*' ..
        'g:somevar = "value"\_s*' ..
        '\d PUSHS "value"\_s*' ..
        '\d STOREG g:somevar\_s*' ..
        'unlet g:somevar\_s*' ..
        '\d UNLET g:somevar\_s*' ..
        'unlet! g:somevar\_s*' ..
        '\d UNLET! g:somevar\_s*' ..
        'unlet $SOMEVAR\_s*' ..
        '\d UNLETENV $SOMEVAR\_s*' ..

        'var l = \[1, 2, 3]\_s*' ..
        '\d\+ PUSHNR 1\_s*' ..
        '\d\+ PUSHNR 2\_s*' ..
        '\d\+ PUSHNR 3\_s*' ..
        '\d\+ NEWLIST size 3\_s*' ..
        '\d\+ SETTYPE list<number>\_s*' ..
        '\d\+ STORE $0\_s*' ..

        'unlet l\[2]\_s*' ..
        '\d\+ PUSHNR 2\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ UNLETINDEX\_s*' ..

        'unlet l\[0 : 1]\_s*' ..
        '\d\+ PUSHNR 0\_s*' ..
        '\d\+ PUSHNR 1\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ UNLETRANGE\_s*',
        res)
enddef

def s:LockLocal()
  var d = {a: 1}
  lockvar d.a
  const nr = 22
enddef

def Test_disassemble_lock_local()
  var res = execute('disass s:LockLocal')
  assert_match('<SNR>\d*_LockLocal\_s*' ..
        'var d = {a: 1}\_s*' ..
        '\d PUSHS "a"\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d NEWDICT size 1\_s*' ..
        '\d SETTYPE dict<number>\_s*' ..
        '\d STORE $0\_s*' ..
        'lockvar d.a\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d LOCKUNLOCK lockvar 2 d.a\_s*' ..

        'const nr = 22\_s*' ..
        '\d\+ PUSHNR 22\_s*' ..
        '\d\+ LOCKCONST\_s*' ..
        '\d\+ STORE $1',
        res)
enddef

def s:ScriptFuncTry()
  try
    echo "yes"
  catch /fail/
    echo "no"
  finally
    throw "end"
  endtry
enddef

def Test_disassemble_try()
  var res = execute('disass s:ScriptFuncTry')
  assert_match('<SNR>\d*_ScriptFuncTry\_s*' ..
        'try\_s*' ..
        '\d TRY catch -> \d\+, finally -> \d\+, endtry -> \d\+\_s*' ..
        'echo "yes"\_s*' ..
        '\d PUSHS "yes"\_s*' ..
        '\d ECHO 1\_s*' ..
        'catch /fail/\_s*' ..
        '\d JUMP -> \d\+\_s*' ..
        '\d PUSH v:exception\_s*' ..
        '\d PUSHS "fail"\_s*' ..
        '\d COMPARESTRING =\~\_s*' ..
        '\d JUMP_IF_FALSE -> \d\+\_s*' ..
        '\d CATCH\_s*' ..
        'echo "no"\_s*' ..
        '\d\+ PUSHS "no"\_s*' ..
        '\d\+ ECHO 1\_s*' ..
        'finally\_s*' ..
        '\d\+ FINALLY\_s*' ..
        'throw "end"\_s*' ..
        '\d\+ PUSHS "end"\_s*' ..
        '\d\+ THROW\_s*' ..
        'endtry\_s*' ..
        '\d\+ ENDTRY',
        res)
enddef

def s:ScriptFuncNew()
  var ll = [1, "two", 333]
  var dd = {one: 1, two: "val"}
enddef

def Test_disassemble_new()
  var res = execute('disass s:ScriptFuncNew')
  assert_match('<SNR>\d*_ScriptFuncNew\_s*' ..
        'var ll = \[1, "two", 333\]\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d PUSHS "two"\_s*' ..
        '\d PUSHNR 333\_s*' ..
        '\d NEWLIST size 3\_s*' ..
        '\d STORE $0\_s*' ..
        'var dd = {one: 1, two: "val"}\_s*' ..
        '\d PUSHS "one"\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d PUSHS "two"\_s*' ..
        '\d PUSHS "val"\_s*' ..
        '\d NEWDICT size 2\_s*',
        res)
enddef

def s:FuncWithArg(arg: any)
  echo arg
enddef

func s:UserFunc()
  echo 'nothing'
endfunc

func s:UserFuncWithArg(arg)
  echo a:arg
endfunc

def s:ScriptFuncCall(): string
  changenr()
  char2nr("abc")
  g:Test_disassemble_new()
  FuncWithArg(343)
  ScriptFuncNew()
  s:ScriptFuncNew()
  UserFunc()
  UserFuncWithArg("foo")
  var FuncRef = function("UserFunc")
  FuncRef()
  var FuncRefWithArg = function("UserFuncWithArg")
  FuncRefWithArg("bar")
  return "yes"
enddef

def Test_disassemble_call()
  var res = execute('disass s:ScriptFuncCall')
  assert_match('<SNR>\d\+_ScriptFuncCall\_s*' ..
        'changenr()\_s*' ..
        '\d BCALL changenr(argc 0)\_s*' ..
        '\d DROP\_s*' ..
        'char2nr("abc")\_s*' ..
        '\d PUSHS "abc"\_s*' ..
        '\d BCALL char2nr(argc 1)\_s*' ..
        '\d DROP\_s*' ..
        'g:Test_disassemble_new()\_s*' ..
        '\d DCALL Test_disassemble_new(argc 0)\_s*' ..
        '\d DROP\_s*' ..
        'FuncWithArg(343)\_s*' ..
        '\d\+ PUSHNR 343\_s*' ..
        '\d\+ DCALL <SNR>\d\+_FuncWithArg(argc 1)\_s*' ..
        '\d\+ DROP\_s*' ..
        'ScriptFuncNew()\_s*' ..
        '\d\+ DCALL <SNR>\d\+_ScriptFuncNew(argc 0)\_s*' ..
        '\d\+ DROP\_s*' ..
        's:ScriptFuncNew()\_s*' ..
        '\d\+ DCALL <SNR>\d\+_ScriptFuncNew(argc 0)\_s*' ..
        '\d\+ DROP\_s*' ..
        'UserFunc()\_s*' ..
        '\d\+ UCALL <80><fd>R\d\+_UserFunc(argc 0)\_s*' ..
        '\d\+ DROP\_s*' ..
        'UserFuncWithArg("foo")\_s*' ..
        '\d\+ PUSHS "foo"\_s*' ..
        '\d\+ UCALL <80><fd>R\d\+_UserFuncWithArg(argc 1)\_s*' ..
        '\d\+ DROP\_s*' ..
        'var FuncRef = function("UserFunc")\_s*' ..
        '\d\+ PUSHS "UserFunc"\_s*' ..
        '\d\+ BCALL function(argc 1)\_s*' ..
        '\d\+ STORE $0\_s*' ..
        'FuncRef()\_s*' ..
        '\d\+ LOAD $\d\_s*' ..
        '\d\+ PCALL (argc 0)\_s*' ..
        '\d\+ DROP\_s*' ..
        'var FuncRefWithArg = function("UserFuncWithArg")\_s*' ..
        '\d\+ PUSHS "UserFuncWithArg"\_s*' ..
        '\d\+ BCALL function(argc 1)\_s*' ..
        '\d\+ STORE $1\_s*' ..
        'FuncRefWithArg("bar")\_s*' ..
        '\d\+ PUSHS "bar"\_s*' ..
        '\d\+ LOAD $\d\_s*' ..
        '\d\+ PCALL (argc 1)\_s*' ..
        '\d\+ DROP\_s*' ..
        'return "yes"\_s*' ..
        '\d\+ PUSHS "yes"\_s*' ..
        '\d\+ RETURN',
        res)
enddef


def s:CreateRefs()
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

def Test_disassemble_closure()
  CreateRefs()
  var res = execute('disass g:Append')
  assert_match('<lambda>\d\_s*' ..
        'local ..= arg\_s*' ..
        '\d LOADOUTER level 1 $0\_s*' ..
        '\d LOAD arg\[-1\]\_s*' ..
        '\d CONCAT size 2\_s*' ..
        '\d STOREOUTER level 1 $0\_s*' ..
        '\d RETURN void',
        res)

  res = execute('disass g:Get')
  assert_match('<lambda>\d\_s*' ..
        'return local\_s*' ..
        '\d LOADOUTER level 1 $0\_s*' ..
        '\d RETURN',
        res)

  unlet g:Append
  unlet g:Get
enddef

def s:ClosureArg(arg: string)
  var Ref = () => arg .. "x"
enddef

def Test_disassemble_closure_arg()
  var res = execute('disass s:ClosureArg')
  assert_match('<SNR>\d\+_ClosureArg\_s*' ..
        'var Ref = () => arg .. "x"\_s*' ..
        '\d FUNCREF <lambda>\d\+',
        res)
  var lres = execute('disass ' .. matchstr(res, '<lambda>\d\+'))
  assert_match('<lambda>\d\+\_s*' ..
        'return arg .. "x"\_s*' ..
        '\d LOADOUTER level 1 arg\[-1]\_s*' ..
        '\d PUSHS "x"\_s*' ..
        '\d CONCAT size 2\_s*' ..
        '\d RETURN',
         lres)
enddef

def s:ClosureInLoop()
  for i in range(5)
    var ii = i
    continue
    break
    if g:val
      return
    endif
    g:Ref = () => ii
    continue
    break
    if g:val
      return
    endif
  endfor
enddef

" Mainly check that ENDLOOP is only produced after a closure was created.
def Test_disassemble_closure_in_loop()
  var res = execute('disass s:ClosureInLoop')
  assert_match('<SNR>\d\+_ClosureInLoop\_s*' ..
        'for i in range(5)\_s*' ..
        '\d\+ STORE -1 in $0\_s*' ..
        '\d\+ PUSHNR 5\_s*' ..
        '\d\+ BCALL range(argc 1)\_s*' ..
        '\d\+ FOR $0 -> \d\+\_s*' ..
        '\d\+ STORE $2\_s*' ..

        'var ii = i\_s*' ..
        '\d\+ LOAD $2\_s*' ..
        '\d\+ STORE $3\_s*' ..

        'continue\_s*' ..
        '\d\+ JUMP -> \d\+\_s*' ..

        'break\_s*' ..
        '\d\+ JUMP -> \d\+\_s*' ..

        'if g:val\_s*' ..
        '\d\+ LOADG g:val\_s*' ..
        '\d\+ COND2BOOL\_s*' ..
        '\d\+ JUMP_IF_FALSE -> \d\+\_s*' ..

        '  return\_s*' ..
        '\d\+ PUSHNR 0\_s*' ..
        '\d\+ RETURN\_s*' ..

        'endif\_s*' ..
        'g:Ref = () => ii\_s*' ..
        '\d\+ FUNCREF <lambda>4 vars  $3-$3\_s*' ..
        '\d\+ STOREG g:Ref\_s*' ..

        'continue\_s*' ..
        '\d\+ ENDLOOP ref $1 save $3-$3 depth 0\_s*' ..
        '\d\+ JUMP -> \d\+\_s*' ..

        'break\_s*' ..
        '\d\+ ENDLOOP ref $1 save $3-$3 depth 0\_s*' ..
        '\d\+ JUMP -> \d\+\_s*' ..

         'if g:val\_s*' ..
        '\d\+ LOADG g:val\_s*' ..
        '\d\+ COND2BOOL\_s*' ..
        '\d\+ JUMP_IF_FALSE -> \d\+\_s*' ..

        '  return\_s*' ..
        '\d\+ PUSHNR 0\_s*' ..
        '\d\+ ENDLOOP ref $1 save $3-$3 depth 0\_s*' ..
        '\d\+ RETURN\_s*' ..

        'endif\_s*' ..
        'endfor\_s*' ..
        '\d\+ ENDLOOP ref $1 save $3-$3 depth 0\_s*' ..
        '\d\+ JUMP -> \d\+\_s*' ..
        '\d\+ DROP\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def EchoArg(arg: string): string
  return arg
enddef
def s:RefThis(): func
  return function('EchoArg')
enddef
def s:ScriptPCall()
  RefThis()("text")
enddef

def Test_disassemble_pcall()
  var res = execute('disass s:ScriptPCall')
  assert_match('<SNR>\d\+_ScriptPCall\_s*' ..
        'RefThis()("text")\_s*' ..
        '\d DCALL <SNR>\d\+_RefThis(argc 0)\_s*' ..
        '\d PUSHS "text"\_s*' ..
        '\d PCALL top (argc 1)\_s*' ..
        '\d PCALL end\_s*' ..
        '\d DROP\_s*' ..
        '\d RETURN void',
        res)
enddef


def s:FuncWithForwardCall(): string
  return g:DefinedLater("yes")
enddef

def DefinedLater(arg: string): string
  return arg
enddef

def Test_disassemble_update_instr()
  var res = execute('disass s:FuncWithForwardCall')
  assert_match('FuncWithForwardCall\_s*' ..
        'return g:DefinedLater("yes")\_s*' ..
        '\d PUSHS "yes"\_s*' ..
        '\d DCALL DefinedLater(argc 1)\_s*' ..
        '\d RETURN',
        res)

  # Calling the function will change UCALL into the faster DCALL
  assert_equal('yes', FuncWithForwardCall())

  res = execute('disass s:FuncWithForwardCall')
  assert_match('FuncWithForwardCall\_s*' ..
        'return g:DefinedLater("yes")\_s*' ..
        '\d PUSHS "yes"\_s*' ..
        '\d DCALL DefinedLater(argc 1)\_s*' ..
        '\d RETURN',
        res)
enddef


def FuncWithDefault(l: number, arg: string = "default", nr = 77): string
  return arg .. nr
enddef

def Test_disassemble_call_default()
  var res = execute('disass FuncWithDefault')
  assert_match('FuncWithDefault\_s*' ..
        '  arg = "default"\_s*' ..
        '\d JUMP_IF_ARG_SET arg\[-2\] -> 3\_s*' ..
        '\d PUSHS "default"\_s*' ..
        '\d STORE arg\[-2]\_s*' ..
        '  nr = 77\_s*' ..
        '3 JUMP_IF_ARG_SET arg\[-1\] -> 6\_s*' ..
        '\d PUSHNR 77\_s*' ..
        '\d STORE arg\[-1]\_s*' ..
        '  return arg .. nr\_s*' ..
        '6 LOAD arg\[-2]\_s*' ..
        '\d LOAD arg\[-1]\_s*' ..
        '\d 2STRING stack\[-1]\_s*' ..
        '\d\+ CONCAT size 2\_s*' ..
        '\d\+ RETURN',
        res)
enddef


def s:HasEval()
  if has("eval")
    echo "yes"
  else
    echo "no"
  endif
enddef

def s:HasNothing()
  if has("nothing")
    echo "yes"
  else
    echo "no"
  endif
enddef

def s:HasSomething()
  if has("nothing")
    echo "nothing"
  elseif has("something")
    echo "something"
  elseif has("eval")
    echo "eval"
  elseif has("less")
    echo "less"
  endif
enddef

def s:HasGuiRunning()
  if has("gui_running")
    echo "yes"
  else
    echo "no"
  endif
enddef

def s:LenConstant(): number
  return len("foo") + len("fighters")
enddef

def Test_disassemble_const_expr()
  var instr = execute('disassemble LenConstant')
  assert_match('LenConstant\_s*' ..
    'return len("foo") + len("fighters")\_s*' ..
    '\d PUSHNR 11\_s*',
    instr)
  assert_notmatch('BCALL len', instr)

  assert_equal("\nyes", execute('HasEval()'))
  instr = execute('disassemble HasEval')
  assert_match('HasEval\_s*' ..
        'if has("eval")\_s*' ..
        'echo "yes"\_s*' ..
        '\d PUSHS "yes"\_s*' ..
        '\d ECHO 1\_s*' ..
        'else\_s*' ..
        'echo "no"\_s*' ..
        'endif\_s*',
        instr)
  assert_notmatch('JUMP', instr)

  assert_equal("\nno", execute('HasNothing()'))
  instr = execute('disassemble HasNothing')
  assert_match('HasNothing\_s*' ..
        'if has("nothing")\_s*' ..
        'echo "yes"\_s*' ..
        'else\_s*' ..
        'echo "no"\_s*' ..
        '\d PUSHS "no"\_s*' ..
        '\d ECHO 1\_s*' ..
        'endif',
        instr)
  assert_notmatch('PUSHS "yes"', instr)
  assert_notmatch('JUMP', instr)

  assert_equal("\neval", execute('HasSomething()'))
  instr = execute('disassemble HasSomething')
  assert_match('HasSomething.*' ..
        'if has("nothing")\_s*' ..
        'echo "nothing"\_s*' ..
        'elseif has("something")\_s*' ..
        'echo "something"\_s*' ..
        'elseif has("eval")\_s*' ..
        'echo "eval"\_s*' ..
        '\d PUSHS "eval"\_s*' ..
        '\d ECHO 1\_s*' ..
        'elseif has("less").*' ..
        'echo "less"\_s*' ..
        'endif',
        instr)
  assert_notmatch('PUSHS "nothing"', instr)
  assert_notmatch('PUSHS "something"', instr)
  assert_notmatch('PUSHS "less"', instr)
  assert_notmatch('JUMP', instr)

  var result: string
  var instr_expected: string
  if has('gui')
    if has('gui_running')
      # GUI already running, always returns "yes"
      result = "\nyes"
      instr_expected = 'HasGuiRunning.*' ..
          'if has("gui_running")\_s*' ..
          '  echo "yes"\_s*' ..
          '\d PUSHS "yes"\_s*' ..
          '\d ECHO 1\_s*' ..
          'else\_s*' ..
          '  echo "no"\_s*' ..
          'endif'
    else
      result = "\nno"
      if has('unix')
        # GUI not running but can start later, call has()
        instr_expected = 'HasGuiRunning.*' ..
            'if has("gui_running")\_s*' ..
            '\d PUSHS "gui_running"\_s*' ..
            '\d BCALL has(argc 1)\_s*' ..
            '\d COND2BOOL\_s*' ..
            '\d JUMP_IF_FALSE -> \d\_s*' ..
            '  echo "yes"\_s*' ..
            '\d PUSHS "yes"\_s*' ..
            '\d ECHO 1\_s*' ..
            'else\_s*' ..
            '\d JUMP -> \d\_s*' ..
            '  echo "no"\_s*' ..
            '\d PUSHS "no"\_s*' ..
            '\d ECHO 1\_s*' ..
            'endif'
      else
        # GUI not running, always return "no"
        instr_expected = 'HasGuiRunning.*' ..
            'if has("gui_running")\_s*' ..
            '  echo "yes"\_s*' ..
            'else\_s*' ..
            '  echo "no"\_s*' ..
            '\d PUSHS "no"\_s*' ..
            '\d ECHO 1\_s*' ..
            'endif'
      endif
    endif
  else
    # GUI not supported, always return "no"
    result = "\nno"
    instr_expected = 'HasGuiRunning.*' ..
        'if has("gui_running")\_s*' ..
        '  echo "yes"\_s*' ..
        'else\_s*' ..
        '  echo "no"\_s*' ..
        '\d PUSHS "no"\_s*' ..
        '\d ECHO 1\_s*' ..
        'endif'
  endif

  assert_equal(result, execute('HasGuiRunning()'))
  instr = execute('disassemble HasGuiRunning')
  assert_match(instr_expected, instr)
enddef

def ReturnInIf(): string
  if 1 < 0
    return "maybe"
  endif
  if g:cond
    return "yes"
  else
    return "no"
  endif
enddef

def Test_disassemble_return_in_if()
  var instr = execute('disassemble ReturnInIf')
  assert_match('ReturnInIf\_s*' ..
        'if 1 < 0\_s*' ..
        '  return "maybe"\_s*' ..
        'endif\_s*' ..
        'if g:cond\_s*' ..
        '0 LOADG g:cond\_s*' ..
        '1 COND2BOOL\_s*' ..
        '2 JUMP_IF_FALSE -> 5\_s*' ..
        'return "yes"\_s*' ..
        '3 PUSHS "yes"\_s*' ..
        '4 RETURN\_s*' ..
        'else\_s*' ..
        ' return "no"\_s*' ..
        '5 PUSHS "no"\_s*' ..
        '6 RETURN$',
        instr)
enddef

def WithFunc()
  var Funky1: func
  var Funky2: func = function("len")
  var Party2: func = funcref("UserFunc")
enddef

def Test_disassemble_function()
  var instr = execute('disassemble WithFunc')
  assert_match('WithFunc\_s*' ..
        'var Funky1: func\_s*' ..
        '0 PUSHFUNC "\[none]"\_s*' ..
        '1 STORE $0\_s*' ..
        'var Funky2: func = function("len")\_s*' ..
        '2 PUSHS "len"\_s*' ..
        '3 BCALL function(argc 1)\_s*' ..
        '4 STORE $1\_s*' ..
        'var Party2: func = funcref("UserFunc")\_s*' ..
        '\d PUSHS "UserFunc"\_s*' ..
        '\d BCALL funcref(argc 1)\_s*' ..
        '\d STORE $2\_s*' ..
        '\d RETURN void',
        instr)
enddef

if has('channel')
  def WithChannel()
    var job1: job
    var job2: job = job_start("donothing")
    var chan1: channel
  enddef
endif

def Test_disassemble_channel()
  CheckFeature channel

  var instr = execute('disassemble WithChannel')
  assert_match('WithChannel\_s*' ..
        'var job1: job\_s*' ..
        '\d PUSHJOB "no process"\_s*' ..
        '\d STORE $0\_s*' ..
        'var job2: job = job_start("donothing")\_s*' ..
        '\d PUSHS "donothing"\_s*' ..
        '\d BCALL job_start(argc 1)\_s*' ..
        '\d STORE $1\_s*' ..
        'var chan1: channel\_s*' ..
        '\d PUSHCHANNEL 0\_s*' ..
        '\d STORE $2\_s*' ..
        '\d RETURN void',
        instr)
enddef

def s:WithLambda(): string
  var F = (a) => "X" .. a .. "X"
  return F("x")
enddef

def Test_disassemble_lambda()
  assert_equal("XxX", WithLambda())
  var instr = execute('disassemble WithLambda')
  assert_match('WithLambda\_s*' ..
        'var F = (a) => "X" .. a .. "X"\_s*' ..
        '\d FUNCREF <lambda>\d\+\_s*' ..
        '\d STORE $0\_s*' ..
        'return F("x")\_s*' ..
        '\d PUSHS "x"\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d PCALL (argc 1)\_s*' ..
        '\d RETURN',
        instr)

   var name = substitute(instr, '.*\(<lambda>\d\+\).*', '\1', '')
   instr = execute('disassemble ' .. name)
   assert_match('<lambda>\d\+\_s*' ..
        'return "X" .. a .. "X"\_s*' ..
        '\d PUSHS "X"\_s*' ..
        '\d LOAD arg\[-1\]\_s*' ..
        '\d 2STRING_ANY stack\[-1\]\_s*' ..
        '\d CONCAT size 2\_s*' ..
        '\d PUSHS "X"\_s*' ..
        '\d CONCAT size 2\_s*' ..
        '\d RETURN',
        instr)
enddef

def s:LambdaWithType(): number
  var Ref = (a: number) => a + 10
  return Ref(g:value)
enddef

def Test_disassemble_lambda_with_type()
  g:value = 5
  assert_equal(15, LambdaWithType())
  var instr = execute('disassemble LambdaWithType')
  assert_match('LambdaWithType\_s*' ..
        'var Ref = (a: number) => a + 10\_s*' ..
        '\d FUNCREF <lambda>\d\+\_s*' ..
        '\d STORE $0\_s*' ..
        'return Ref(g:value)\_s*' ..
        '\d LOADG g:value\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d CHECKTYPE number stack\[-2\] arg 1\_s*' ..
        '\d PCALL (argc 1)\_s*' ..
        '\d RETURN',
        instr)
enddef

def NestedOuter()
  def g:Inner()
    echomsg "inner"
  enddef
enddef

def Test_disassemble_nested_func()
   var instr = execute('disassemble NestedOuter')
   assert_match('NestedOuter\_s*' ..
        'def g:Inner()\_s*' ..
        'echomsg "inner"\_s*' ..
        'enddef\_s*' ..
        '\d NEWFUNC <lambda>\d\+ Inner\_s*' ..
        '\d RETURN void',
        instr)
enddef

def NestedDefList()
  def
  def Info
  def /Info
  def /Info/
enddef

def Test_disassemble_nested_def_list()
   var instr = execute('disassemble NestedDefList')
   assert_match('NestedDefList\_s*' ..
        'def\_s*' ..
        '\d DEF \_s*' ..
        'def Info\_s*' ..
        '\d DEF Info\_s*' ..
        'def /Info\_s*' ..
        '\d DEF /Info\_s*' ..
        'def /Info/\_s*' ..
        '\d DEF /Info/\_s*' ..
        '\d RETURN void',
        instr)
enddef

def s:AndOr(arg: any): string
  if arg == 1 && arg != 2 || arg == 4
    return 'yes'
  endif
  return 'no'
enddef

def Test_disassemble_and_or()
  assert_equal("yes", AndOr(1))
  assert_equal("no", AndOr(2))
  assert_equal("yes", AndOr(4))
  var instr = execute('disassemble AndOr')
  assert_match('AndOr\_s*' ..
        'if arg == 1 && arg != 2 || arg == 4\_s*' ..
        '\d LOAD arg\[-1]\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d COMPAREANY ==\_s*' ..
        '\d JUMP_IF_COND_FALSE -> \d\+\_s*' ..
        '\d LOAD arg\[-1]\_s*' ..
        '\d PUSHNR 2\_s*' ..
        '\d COMPAREANY !=\_s*' ..
        '\d JUMP_IF_COND_TRUE -> \d\+\_s*' ..
        '\d LOAD arg\[-1]\_s*' ..
        '\d\+ PUSHNR 4\_s*' ..
        '\d\+ COMPAREANY ==\_s*' ..
        '\d\+ JUMP_IF_FALSE -> \d\+',
        instr)
enddef

def s:AndConstant(arg: any): string
  if true && arg
    return "yes"
  endif
  if false && arg
    return "never"
  endif
  return "no"
enddef

def Test_disassemble_and_constant()
  assert_equal("yes", AndConstant(1))
  assert_equal("no", AndConstant(false))
  var instr = execute('disassemble AndConstant')
  assert_match('AndConstant\_s*' ..
      'if true && arg\_s*' ..
      '0 LOAD arg\[-1\]\_s*' ..
      '1 COND2BOOL\_s*' ..
      '2 JUMP_IF_FALSE -> 5\_s*' ..
      'return "yes"\_s*' ..
      '3 PUSHS "yes"\_s*' ..
      '4 RETURN\_s*' ..
      'endif\_s*' ..
      'if false && arg\_s*' ..
      'return "never"\_s*' ..
      'endif\_s*' ..
      'return "no"\_s*' ..
      '5 PUSHS "no"\_s*' ..
      '6 RETURN',
      instr)
enddef

def s:ForLoop(): list<number>
  var res: list<number>
  for i in range(3)
    res->add(i)
  endfor
  return res
enddef

def Test_disassemble_for_loop()
  assert_equal([0, 1, 2], ForLoop())
  var instr = execute('disassemble ForLoop')
  assert_match('ForLoop\_s*' ..
        'var res: list<number>\_s*' ..
        '\d NEWLIST size 0\_s*' ..
        '\d SETTYPE list<number>\_s*' ..
        '\d STORE $0\_s*' ..

        'for i in range(3)\_s*' ..
        '\d STORE -1 in $1\_s*' ..
        '\d PUSHNR 3\_s*' ..
        '\d BCALL range(argc 1)\_s*' ..
        '\d FOR $1 -> \d\+\_s*' ..
        '\d STORE $3\_s*' ..

        'res->add(i)\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d LOAD $3\_s*' ..
        '\d\+ LISTAPPEND\_s*' ..
        '\d\+ DROP\_s*' ..

        'endfor\_s*' ..
        '\d\+ JUMP -> \d\+\_s*' ..
        '\d\+ DROP',
        instr)
enddef

def s:ForLoopEval(): string
  var res = ""
  for str in eval('["one", "two"]')
    res ..= str
  endfor
  return res
enddef

def Test_disassemble_for_loop_eval()
  assert_equal('onetwo', ForLoopEval())
  var instr = execute('disassemble ForLoopEval')
  assert_match('ForLoopEval\_s*' ..
        'var res = ""\_s*' ..
        '\d PUSHS ""\_s*' ..
        '\d STORE $0\_s*' ..

        'for str in eval(''\["one", "two"\]'')\_s*' ..
        '\d STORE -1 in $1\_s*' ..
        '\d PUSHS "\["one", "two"\]"\_s*' ..
        '\d BCALL eval(argc 1)\_s*' ..
        '\d FOR $1 -> \d\+\_s*' ..
        '\d STORE $3\_s*' ..

        'res ..= str\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ LOAD $3\_s*' ..
        '\d 2STRING_ANY stack\[-1\]\_s*' ..
        '\d\+ CONCAT size 2\_s*' ..
        '\d\+ STORE $0\_s*' ..

        'endfor\_s*' ..
        '\d\+ JUMP -> 5\_s*' ..
        '\d\+ DROP\_s*' ..

        'return res\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ RETURN',
        instr)
enddef

def s:ForLoopUnpack()
  for [x1, x2] in [[1, 2], [3, 4]]
    echo x1 x2
  endfor
enddef

def Test_disassemble_for_loop_unpack()
  var instr = execute('disassemble ForLoopUnpack')
  assert_match('ForLoopUnpack\_s*' ..
        'for \[x1, x2\] in \[\[1, 2\], \[3, 4\]\]\_s*' ..
        '\d\+ STORE -1 in $0\_s*' ..
        '\d\+ PUSHNR 1\_s*' ..
        '\d\+ PUSHNR 2\_s*' ..
        '\d\+ NEWLIST size 2\_s*' ..
        '\d\+ PUSHNR 3\_s*' ..
        '\d\+ PUSHNR 4\_s*' ..
        '\d\+ NEWLIST size 2\_s*' ..
        '\d\+ NEWLIST size 2\_s*' ..
        '\d\+ FOR $0 -> 16\_s*' ..
        '\d\+ UNPACK 2\_s*' ..
        '\d\+ STORE $2\_s*' ..
        '\d\+ STORE $3\_s*' ..

        'echo x1 x2\_s*' ..
        '\d\+ LOAD $2\_s*' ..
        '\d\+ LOAD $3\_s*' ..
        '\d\+ ECHO 2\_s*' ..

        'endfor\_s*' ..
        '\d\+ JUMP -> 8\_s*' ..
        '\d\+ DROP\_s*' ..
        '\d\+ RETURN void',
        instr)
enddef

def s:ForLoopContinue()
  for nr in [1, 2]
    try
      echo "ok"
      try
        echo "deeper"
      catch
        continue
      endtry
    catch
      echo "not ok"
    endtry
  endfor
enddef

def Test_disassemble_for_loop_continue()
  var instr = execute('disassemble ForLoopContinue')
  assert_match('ForLoopContinue\_s*' ..
        'for nr in \[1, 2]\_s*' ..
        '0 STORE -1 in $0\_s*' ..
        '1 PUSHNR 1\_s*' ..
        '2 PUSHNR 2\_s*' ..
        '3 NEWLIST size 2\_s*' ..
        '4 FOR $0 -> 22\_s*' ..
        '5 STORE $2\_s*' ..

        'try\_s*' ..
        '6 TRY catch -> 17, endtry -> 20\_s*' ..

        'echo "ok"\_s*' ..
        '7 PUSHS "ok"\_s*' ..
        '8 ECHO 1\_s*' ..

        'try\_s*' ..
        '9 TRY catch -> 13, endtry -> 15\_s*' ..

        'echo "deeper"\_s*' ..
        '10 PUSHS "deeper"\_s*' ..
        '11 ECHO 1\_s*' ..

        'catch\_s*' ..
        '12 JUMP -> 15\_s*' ..
        '13 CATCH\_s*' ..

        'continue\_s*' ..
        '14 TRY-CONTINUE 2 levels -> 4\_s*' ..

        'endtry\_s*' ..
        '15 ENDTRY\_s*' ..

        'catch\_s*' ..
        '16 JUMP -> 20\_s*' ..
        '17 CATCH\_s*' ..

        'echo "not ok"\_s*' ..
        '18 PUSHS "not ok"\_s*' ..
        '19 ECHO 1\_s*' ..

        'endtry\_s*' ..
        '20 ENDTRY\_s*' ..

        'endfor\_s*' ..
        '21 JUMP -> 4\_s*' ..
        '\d\+ DROP\_s*' ..
        '\d\+ RETURN void',
        instr)
enddef

let g:number = 42

def s:TypeCast()
  var l: list<number> = [23, <number>g:number]
enddef

def Test_disassemble_typecast()
  var instr = execute('disassemble TypeCast')
  assert_match('TypeCast.*' ..
        'var l: list<number> = \[23, <number>g:number\].*' ..
        '\d PUSHNR 23\_s*' ..
        '\d LOADG g:number\_s*' ..
        '\d CHECKTYPE number stack\[-1\]\_s*' ..
        '\d NEWLIST size 2\_s*' ..
        '\d SETTYPE list<number>\_s*' ..
        '\d STORE $0\_s*' ..
        '\d RETURN void\_s*',
        instr)
enddef

def s:Computing()
  var nr = 3
  var nrres = nr + 7
  nrres = nr - 7
  nrres = nr * 7
  nrres = nr / 7
  nrres = nr % 7

  var anyres = g:number + 7
  anyres = g:number - 7
  anyres = g:number * 7
  anyres = g:number / 7
  anyres = g:number % 7

  var fl = 3.0
  var flres = fl + 7.0
  flres = fl - 7.0
  flres = fl * 7.0
  flres = fl / 7.0
enddef

def Test_disassemble_computing()
  var instr = execute('disassemble Computing')
  assert_match('Computing.*' ..
        'var nr = 3.*' ..
        '\d STORE 3 in $0.*' ..
        'var nrres = nr + 7.*' ..
        '\d LOAD $0.*' ..
        '\d PUSHNR 7.*' ..
        '\d OPNR +.*' ..
        '\d STORE $1.*' ..
        'nrres = nr - 7.*' ..
        '\d OPNR -.*' ..
        'nrres = nr \* 7.*' ..
        '\d OPNR \*.*' ..
        'nrres = nr / 7.*' ..
        '\d OPNR /.*' ..
        'nrres = nr % 7.*' ..
        '\d OPNR %.*' ..
        'var anyres = g:number + 7.*' ..
        '\d LOADG g:number.*' ..
        '\d PUSHNR 7.*' ..
        '\d OPANY +.*' ..
        '\d STORE $2.*' ..
        'anyres = g:number - 7.*' ..
        '\d OPANY -.*' ..
        'anyres = g:number \* 7.*' ..
        '\d OPANY \*.*' ..
        'anyres = g:number / 7.*' ..
        '\d OPANY /.*' ..
        'anyres = g:number % 7.*' ..
        '\d OPANY %.*',
        instr)
  assert_match('Computing.*' ..
      'var fl = 3.0.*' ..
      '\d PUSHF 3.0.*' ..
      '\d STORE $3.*' ..
      'var flres = fl + 7.0.*' ..
      '\d LOAD $3.*' ..
      '\d PUSHF 7.0.*' ..
      '\d OPFLOAT +.*' ..
      '\d STORE $4.*' ..
      'flres = fl - 7.0.*' ..
      '\d OPFLOAT -.*' ..
      'flres = fl \* 7.0.*' ..
      '\d OPFLOAT \*.*' ..
      'flres = fl / 7.0.*' ..
      '\d OPFLOAT /.*',
      instr)
enddef

def s:AddListBlob()
  var reslist = [1, 2] + [3, 4]
  var resblob = 0z1122 + 0z3344
enddef

def Test_disassemble_add_list_blob()
  var instr = execute('disassemble AddListBlob')
  assert_match('AddListBlob.*' ..
        'var reslist = \[1, 2] + \[3, 4].*' ..
        '\d PUSHNR 1.*' ..
        '\d PUSHNR 2.*' ..
        '\d NEWLIST size 2.*' ..
        '\d PUSHNR 3.*' ..
        '\d PUSHNR 4.*' ..
        '\d NEWLIST size 2.*' ..
        '\d ADDLIST.*' ..
        '\d STORE $.*.*' ..
        'var resblob = 0z1122 + 0z3344.*' ..
        '\d PUSHBLOB 0z1122.*' ..
        '\d PUSHBLOB 0z3344.*' ..
        '\d ADDBLOB.*' ..
        '\d STORE $.*',
        instr)
enddef

let g:aa = 'aa'
def s:ConcatString(): string
  var res = g:aa .. "bb"
  return res
enddef

def Test_disassemble_concat()
  var instr = execute('disassemble ConcatString')
  assert_match('ConcatString.*' ..
        'var res = g:aa .. "bb".*' ..
        '\d LOADG g:aa.*' ..
        '\d PUSHS "bb".*' ..
        '\d 2STRING_ANY stack\[-2].*' ..
        '\d CONCAT.*' ..
        '\d STORE $.*',
        instr)
  assert_equal('aabb', ConcatString())
enddef

def s:StringIndex(): string
  var s = "abcd"
  var res = s[1]
  return res
enddef

def Test_disassemble_string_index()
  var instr = execute('disassemble StringIndex')
  assert_match('StringIndex\_s*' ..
        'var s = "abcd"\_s*' ..
        '\d PUSHS "abcd"\_s*' ..
        '\d STORE $0\_s*' ..
        'var res = s\[1]\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d STRINDEX\_s*' ..
        '\d STORE $1\_s*',
        instr)
  assert_equal('b', StringIndex())
enddef

def s:StringSlice(): string
  var s = "abcd"
  var res = s[1 : 8]
  return res
enddef

def Test_disassemble_string_slice()
  var instr = execute('disassemble StringSlice')
  assert_match('StringSlice\_s*' ..
        'var s = "abcd"\_s*' ..
        '\d PUSHS "abcd"\_s*' ..
        '\d STORE $0\_s*' ..
        'var res = s\[1 : 8]\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d PUSHNR 8\_s*' ..
        '\d STRSLICE\_s*' ..
        '\d STORE $1\_s*',
        instr)
  assert_equal('bcd', StringSlice())
enddef

def s:ListIndex(): number
  var l = [1, 2, 3]
  var res = l[1]
  return res
enddef

def Test_disassemble_list_index()
  var instr = execute('disassemble ListIndex')
  assert_match('ListIndex\_s*' ..
        'var l = \[1, 2, 3]\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d PUSHNR 2\_s*' ..
        '\d PUSHNR 3\_s*' ..
        '\d NEWLIST size 3\_s*' ..
        '\d SETTYPE list<number>\_s*' ..
        '\d STORE $0\_s*' ..
        'var res = l\[1]\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d LISTINDEX\_s*' ..
        '\d STORE $1\_s*',
        instr)
  assert_equal(2, ListIndex())
enddef

def s:ListSlice(): list<number>
  var l = [1, 2, 3]
  var res = l[1 : 8]
  return res
enddef

def Test_disassemble_list_slice()
  var instr = execute('disassemble ListSlice')
  assert_match('ListSlice\_s*' ..
        'var l = \[1, 2, 3]\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d PUSHNR 2\_s*' ..
        '\d PUSHNR 3\_s*' ..
        '\d NEWLIST size 3\_s*' ..
        '\d SETTYPE list<number>\_s*' ..
        '\d STORE $0\_s*' ..
        'var res = l\[1 : 8]\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d PUSHNR 8\_s*' ..
        '\d\+ LISTSLICE\_s*' ..
        '\d\+ SETTYPE list<number>\_s*' ..
        '\d\+ STORE $1\_s*',
        instr)
  assert_equal([2, 3], ListSlice())
enddef

def s:DictMember(): number
  var d = {item: 1}
  var res = d.item
  res = d["item"]
  return res
enddef

def Test_disassemble_dict_member()
  var instr = execute('disassemble DictMember')
  assert_match('DictMember\_s*' ..
        'var d = {item: 1}\_s*' ..
        '\d PUSHS "item"\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d NEWDICT size 1\_s*' ..
        '\d SETTYPE dict<number>\_s*' ..
        '\d STORE $0\_s*' ..
        'var res = d.item\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ MEMBER item\_s*' ..
        '\d\+ USEDICT\_s*' ..
        '\d\+ STORE $1\_s*' ..
        'res = d\["item"\]\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ PUSHS "item"\_s*' ..
        '\d\+ MEMBER\_s*' ..
        '\d\+ USEDICT\_s*' ..
        '\d\+ STORE $1\_s*',
        instr)
  assert_equal(1, DictMember())
enddef

let somelist = [1, 2, 3, 4, 5]
def s:AnyIndex(): number
  var res = g:somelist[2]
  return res
enddef

def Test_disassemble_any_index()
  var instr = execute('disassemble AnyIndex')
  assert_match('AnyIndex\_s*' ..
        'var res = g:somelist\[2\]\_s*' ..
        '\d LOADG g:somelist\_s*' ..
        '\d PUSHNR 2\_s*' ..
        '\d ANYINDEX\_s*' ..
        '\d STORE $0\_s*' ..
        'return res\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d CHECKTYPE number stack\[-1\]\_s*' ..
        '\d RETURN',
        instr)
  assert_equal(3, AnyIndex())
enddef

def s:AnySlice(): list<number>
  var res = g:somelist[1 : 3]
  return res
enddef

def Test_disassemble_any_slice()
  var instr = execute('disassemble AnySlice')
  assert_match('AnySlice\_s*' ..
        'var res = g:somelist\[1 : 3\]\_s*' ..
        '\d LOADG g:somelist\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d PUSHNR 3\_s*' ..
        '\d ANYSLICE\_s*' ..
        '\d STORE $0\_s*' ..
        'return res\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d CHECKTYPE list<number> stack\[-1\]\_s*' ..
        '\d RETURN',
        instr)
  assert_equal([2, 3, 4], AnySlice())
enddef

def s:NegateNumber(): number
  g:nr = 9
  var plus = +g:nr
  var minus = -g:nr
  return minus
enddef

def Test_disassemble_negate_number()
  var instr = execute('disassemble NegateNumber')
  assert_match('NegateNumber\_s*' ..
        'g:nr = 9\_s*' ..
        '\d PUSHNR 9\_s*' ..
        '\d STOREG g:nr\_s*' ..
        'var plus = +g:nr\_s*' ..
        '\d LOADG g:nr\_s*' ..
        '\d CHECKTYPE number stack\[-1\]\_s*' ..
        '\d STORE $0\_s*' ..
        'var minus = -g:nr\_s*' ..
        '\d LOADG g:nr\_s*' ..
        '\d CHECKTYPE number stack\[-1\]\_s*' ..
        '\d NEGATENR\_s*' ..
        '\d STORE $1\_s*',
        instr)
  assert_equal(-9, NegateNumber())
enddef

def s:InvertBool(): bool
  var flag = true
  var invert = !flag
  var res = !!flag
  return res
enddef

def Test_disassemble_invert_bool()
  var instr = execute('disassemble InvertBool')
  assert_match('InvertBool\_s*' ..
        'var flag = true\_s*' ..
        '\d PUSH true\_s*' ..
        '\d STORE $0\_s*' ..
        'var invert = !flag\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d INVERT -1 (!val)\_s*' ..
        '\d STORE $1\_s*' ..
        'var res = !!flag\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d 2BOOL -1 (!!val)\_s*' ..
        '\d STORE $2\_s*',
        instr)
  assert_equal(true, InvertBool())
enddef

def s:ReturnBool(): bool
  var one = 1
  var zero = 0
  var none: number
  var name: bool = one && zero || one
  return name
enddef

def Test_disassemble_return_bool()
  var instr = execute('disassemble ReturnBool')
  assert_match('ReturnBool\_s*' ..
        'var one = 1\_s*' ..
        '0 STORE 1 in $0\_s*' ..
        'var zero = 0\_s*' ..
        'var none: number\_s*' ..
        'var name: bool = one && zero || one\_s*' ..
        '1 LOAD $0\_s*' ..
        '2 COND2BOOL\_s*' ..
        '3 JUMP_IF_COND_FALSE -> 6\_s*' ..
        '4 LOAD $1\_s*' ..
        '5 COND2BOOL\_s*' ..
        '6 JUMP_IF_COND_TRUE -> 9\_s*' ..
        '7 LOAD $0\_s*' ..
        '8 COND2BOOL\_s*' ..
        '9 STORE $3\_s*' ..
        'return name\_s*' ..
        '\d\+ LOAD $3\_s*' ..
        '\d\+ RETURN',
        instr)
  assert_equal(true, InvertBool())
enddef

def s:AutoInit()
  var t: number
  t = 1
  t = 0
enddef

def Test_disassemble_auto_init()
  var instr = execute('disassemble AutoInit')
  assert_match('AutoInit\_s*' ..
        'var t: number\_s*' ..
        't = 1\_s*' ..
        '\d STORE 1 in $0\_s*' ..
        't = 0\_s*' ..
        '\d STORE 0 in $0\_s*' ..
        '\d\+ RETURN void',
        instr)
enddef

def Test_disassemble_compare()
  var cases = [
        ['true == isFalse', 'COMPAREBOOL =='],
        ['true != isFalse', 'COMPAREBOOL !='],
        ['v:none == isNull', 'COMPARESPECIAL =='],
        ['v:none != isNull', 'COMPARESPECIAL !='],
        ['"text" == isNull', 'COMPARENULL =='],
        ['"text" != isNull', 'COMPARENULL !='],

        ['111 == aNumber', 'COMPARENR =='],
        ['111 != aNumber', 'COMPARENR !='],
        ['111 > aNumber', 'COMPARENR >'],
        ['111 < aNumber', 'COMPARENR <'],
        ['111 >= aNumber', 'COMPARENR >='],
        ['111 <= aNumber', 'COMPARENR <='],
        ['111 =~ aNumber', 'COMPARENR =\~'],
        ['111 !~ aNumber', 'COMPARENR !\~'],

        ['"xx" != aString', 'COMPARESTRING !='],
        ['"xx" > aString', 'COMPARESTRING >'],
        ['"xx" < aString', 'COMPARESTRING <'],
        ['"xx" >= aString', 'COMPARESTRING >='],
        ['"xx" <= aString', 'COMPARESTRING <='],
        ['"xx" =~ aString', 'COMPARESTRING =\~'],
        ['"xx" !~ aString', 'COMPARESTRING !\~'],
        ['"xx" is aString', 'COMPARESTRING is'],
        ['"xx" isnot aString', 'COMPARESTRING isnot'],

        ['0z11 == aBlob', 'COMPAREBLOB =='],
        ['0z11 != aBlob', 'COMPAREBLOB !='],
        ['0z11 is aBlob', 'COMPAREBLOB is'],
        ['0z11 isnot aBlob', 'COMPAREBLOB isnot'],

        ['[1, 2] == aList', 'COMPARELIST =='],
        ['[1, 2] != aList', 'COMPARELIST !='],
        ['[1, 2] is aList', 'COMPARELIST is'],
        ['[1, 2] isnot aList', 'COMPARELIST isnot'],

        ['{a: 1} == aDict', 'COMPAREDICT =='],
        ['{a: 1} != aDict', 'COMPAREDICT !='],
        ['{a: 1} is aDict', 'COMPAREDICT is'],
        ['{a: 1} isnot aDict', 'COMPAREDICT isnot'],

        ['(() => 33) == (() => 44)', 'COMPAREFUNC =='],
        ['(() => 33) != (() => 44)', 'COMPAREFUNC !='],
        ['(() => 33) is (() => 44)', 'COMPAREFUNC is'],
        ['(() => 33) isnot (() => 44)', 'COMPAREFUNC isnot'],

        ['77 == g:xx', 'COMPAREANY =='],
        ['77 != g:xx', 'COMPAREANY !='],
        ['77 > g:xx', 'COMPAREANY >'],
        ['77 < g:xx', 'COMPAREANY <'],
        ['77 >= g:xx', 'COMPAREANY >='],
        ['77 <= g:xx', 'COMPAREANY <='],
        ['77 =~ g:xx', 'COMPAREANY =\~'],
        ['77 !~ g:xx', 'COMPAREANY !\~'],
        ['77 is g:xx', 'COMPAREANY is'],
        ['77 isnot g:xx', 'COMPAREANY isnot'],
        ]
  var floatDecl = ''
  cases->extend([
      ['1.1 == aFloat', 'COMPAREFLOAT =='],
      ['1.1 != aFloat', 'COMPAREFLOAT !='],
      ['1.1 > aFloat', 'COMPAREFLOAT >'],
      ['1.1 < aFloat', 'COMPAREFLOAT <'],
      ['1.1 >= aFloat', 'COMPAREFLOAT >='],
      ['1.1 <= aFloat', 'COMPAREFLOAT <='],
      ['1.1 =~ aFloat', 'COMPAREFLOAT =\~'],
      ['1.1 !~ aFloat', 'COMPAREFLOAT !\~'],
      ])
  floatDecl = 'var aFloat = 2.2'

  var nr = 1
  for case in cases
    # declare local variables to get a non-constant with the right type
    writefile(['def TestCase' .. nr .. '()',
             '  var isFalse = false',
             '  var isNull = v:null',
             '  var aNumber = 222',
             '  var aString = "yy"',
             '  var aBlob = 0z22',
             '  var aList = [3, 4]',
             '  var aDict = {x: 2}',
             floatDecl,
             '  if ' .. case[0],
             '    echo 42',
             '  endif',
             'enddef'], 'Xdisassemble')
    source Xdisassemble
    var instr = execute('disassemble TestCase' .. nr)
    assert_match('TestCase' .. nr .. '.*' ..
        'if ' .. substitute(case[0], '[[~]', '\\\0', 'g') .. '.*' ..
        '\d \(PUSH\|FUNCREF\).*' ..
        '\d \(PUSH\|FUNCREF\|LOAD\).*' ..
        '\d ' .. case[1] .. '.*' ..
        '\d JUMP_IF_FALSE -> \d\+.*',
        instr)

    nr += 1
  endfor

  delete('Xdisassemble')
enddef

def s:FalsyOp()
  echo g:flag ?? "yes"
  echo [] ?? "empty list"
  echo "" ?? "empty string"
enddef

def Test_disassemble_falsy_op()
  var res = execute('disass s:FalsyOp')
  assert_match('\<SNR>\d*_FalsyOp\_s*' ..
      'echo g:flag ?? "yes"\_s*' ..
      '0 LOADG g:flag\_s*' ..
      '1 JUMP_AND_KEEP_IF_TRUE -> 3\_s*' ..
      '2 PUSHS "yes"\_s*' ..
      '3 ECHO 1\_s*' ..
      'echo \[\] ?? "empty list"\_s*' ..
      '4 NEWLIST size 0\_s*' ..
      '5 JUMP_AND_KEEP_IF_TRUE -> 7\_s*' ..
      '6 PUSHS "empty list"\_s*' ..
      '7 ECHO 1\_s*' ..
      'echo "" ?? "empty string"\_s*' ..
      '\d\+ PUSHS "empty string"\_s*' ..
      '\d\+ ECHO 1\_s*' ..
      '\d\+ RETURN void',
      res)
enddef

def Test_disassemble_compare_const()
  var cases = [
        ['"xx" == "yy"', false],
        ['"aa" == "aa"', true],
        ['has("eval") ? true : false', true],
        ['has("asdf") ? true : false', false],
        ]

  var nr = 1
  for case in cases
    writefile(['def TestCase' .. nr .. '()',
             '  if ' .. case[0],
             '    echo 42',
             '  endif',
             'enddef'], 'Xdisassemble')
    source Xdisassemble
    var instr = execute('disassemble TestCase' .. nr)
    if case[1]
      # condition true, "echo 42" executed
      assert_match('TestCase' .. nr .. '.*' ..
          'if ' .. substitute(case[0], '[[~]', '\\\0', 'g') .. '.*' ..
          '\d PUSHNR 42.*' ..
          '\d ECHO 1.*' ..
          '\d RETURN void',
          instr)
    else
      # condition false, function just returns
      assert_match('TestCase' .. nr .. '.*' ..
          'if ' .. substitute(case[0], '[[~]', '\\\0', 'g') .. '[ \n]*' ..
          'echo 42[ \n]*' ..
          'endif[ \n]*' ..
          '\d RETURN void',
          instr)
    endif

    nr += 1
  endfor

  delete('Xdisassemble')
enddef

def s:Execute()
  execute 'help vim9.txt'
  var cmd = 'help vim9.txt'
  execute cmd
  var tag = 'vim9.txt'
  execute 'help ' .. tag
enddef

def Test_disassemble_execute()
  var res = execute('disass s:Execute')
  assert_match('\<SNR>\d*_Execute\_s*' ..
        "execute 'help vim9.txt'\\_s*" ..
        '\d PUSHS "help vim9.txt"\_s*' ..
        '\d EXECUTE 1\_s*' ..
        "var cmd = 'help vim9.txt'\\_s*" ..
        '\d PUSHS "help vim9.txt"\_s*' ..
        '\d STORE $0\_s*' ..
        'execute cmd\_s*' ..
        '\d LOAD $0\_s*' ..
        '\d EXECUTE 1\_s*' ..
        "var tag = 'vim9.txt'\\_s*" ..
        '\d PUSHS "vim9.txt"\_s*' ..
        '\d STORE $1\_s*' ..
        "execute 'help ' .. tag\\_s*" ..
        '\d\+ PUSHS "help "\_s*' ..
        '\d\+ LOAD $1\_s*' ..
        '\d\+ CONCAT size 2\_s*' ..
        '\d\+ EXECUTE 1\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:OnlyRange()
  :$
  :123
  :'m
enddef

def Test_disassemble_range_only()
  var res = execute('disass s:OnlyRange')
  assert_match('\<SNR>\d*_OnlyRange\_s*' ..
        ':$\_s*' ..
        '\d EXECRANGE $\_s*' ..
        ':123\_s*' ..
        '\d EXECRANGE 123\_s*' ..
        ':''m\_s*' ..
        '\d EXECRANGE ''m\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:StoreRange()
  var l = [1, 2]
  l[0 : 1] = [7, 8]
enddef

def Test_disassemble_store_range()
  var res = execute('disass s:StoreRange')
  assert_match('\<SNR>\d*_StoreRange\_s*' ..
        'var l = \[1, 2]\_s*' ..
        '\d PUSHNR 1\_s*' ..
        '\d PUSHNR 2\_s*' ..
        '\d NEWLIST size 2\_s*' ..
        '\d SETTYPE list<number>\_s*' ..
        '\d STORE $0\_s*' ..

        'l\[0 : 1] = \[7, 8]\_s*' ..
        '\d\+ PUSHNR 7\_s*' ..
        '\d\+ PUSHNR 8\_s*' ..
        '\d\+ NEWLIST size 2\_s*' ..
        '\d\+ PUSHNR 0\_s*' ..
        '\d\+ PUSHNR 1\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ STORERANGE\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:Echomsg()
  echomsg 'some' 'message'
  echoconsole 'nothing'
  echoerr 'went' .. 'wrong'
  var local = 'window'
  echowin 'in' local
  :5echowin 'five'
enddef

def Test_disassemble_echomsg()
  var res = execute('disass s:Echomsg')
  assert_match('\<SNR>\d*_Echomsg\_s*' ..
        "echomsg 'some' 'message'\\_s*" ..
        '\d PUSHS "some"\_s*' ..
        '\d PUSHS "message"\_s*' ..
        '\d ECHOMSG 2\_s*' ..
        "echoconsole 'nothing'\\_s*" ..
        '\d PUSHS "nothing"\_s*' ..
        '\d ECHOCONSOLE 1\_s*' ..
        "echoerr 'went' .. 'wrong'\\_s*" ..
        '\d PUSHS "wentwrong"\_s*' ..
        '\d ECHOERR 1\_s*' ..
        "var local = 'window'\\_s*" ..
        '\d\+ PUSHS "window"\_s*' ..
        '\d\+ STORE $0\_s*' ..
        "echowin 'in' local\\_s*" ..
        '\d\+ PUSHS "in"\_s*' ..
        '\d\+ LOAD $0\_s*' ..
        '\d\+ ECHOWINDOW 2\_s*' ..
        ":5echowin 'five'\\_s*" ..
        '\d\+ PUSHS "five"\_s*' ..
        '\d\+ ECHOWINDOW 1 (5 sec)\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def SomeStringArg(arg: string)
  echo arg
enddef

def SomeAnyArg(arg: any)
  echo arg
enddef

def SomeStringArgAndReturn(arg: string): string
  return arg
enddef

def Test_display_func()
  var res1 = execute('function SomeStringArg')
  assert_match('.* def SomeStringArg(arg: string)\_s*' ..
        '\d *echo arg.*' ..
        ' *enddef',
        res1)

  var res2 = execute('function SomeAnyArg')
  assert_match('.* def SomeAnyArg(arg: any)\_s*' ..
        '\d *echo arg\_s*' ..
        ' *enddef',
        res2)

  var res3 = execute('function SomeStringArgAndReturn')
  assert_match('.* def SomeStringArgAndReturn(arg: string): string\_s*' ..
        '\d *return arg\_s*' ..
        ' *enddef',
        res3)
enddef

def Test_vim9script_forward_func()
  var lines =<< trim END
    vim9script
    def FuncOne(): string
      return FuncTwo()
    enddef
    def FuncTwo(): string
      return 'two'
    enddef
    g:res_FuncOne = execute('disass FuncOne')
  END
  writefile(lines, 'Xdisassemble', 'D')
  source Xdisassemble

  # check that the first function calls the second with DCALL
  assert_match('\<SNR>\d*_FuncOne\_s*' ..
        'return FuncTwo()\_s*' ..
        '\d DCALL <SNR>\d\+_FuncTwo(argc 0)\_s*' ..
        '\d RETURN',
        g:res_FuncOne)

  unlet g:res_FuncOne
enddef

def s:ConcatStrings(): string
  return 'one' .. 'two' .. 'three'
enddef

def s:ComputeConst(): number
  return 2 + 3 * 4 / 6 + 7
enddef

def s:ComputeConstParen(): number
  return ((2 + 4) * (8 / 2)) / (3 + 4)
enddef

def Test_simplify_const_expr()
  var res = execute('disass s:ConcatStrings')
  assert_match('<SNR>\d*_ConcatStrings\_s*' ..
        "return 'one' .. 'two' .. 'three'\\_s*" ..
        '\d PUSHS "onetwothree"\_s*' ..
        '\d RETURN',
        res)

  res = execute('disass s:ComputeConst')
  assert_match('<SNR>\d*_ComputeConst\_s*' ..
        'return 2 + 3 \* 4 / 6 + 7\_s*' ..
        '\d PUSHNR 11\_s*' ..
        '\d RETURN',
        res)

  res = execute('disass s:ComputeConstParen')
  assert_match('<SNR>\d*_ComputeConstParen\_s*' ..
        'return ((2 + 4) \* (8 / 2)) / (3 + 4)\_s*' ..
        '\d PUSHNR 3\>\_s*' ..
        '\d RETURN',
        res)
enddef

def s:CallAppend()
  eval "some text"->append(2)
enddef

def Test_shuffle()
  var res = execute('disass s:CallAppend')
  assert_match('<SNR>\d*_CallAppend\_s*' ..
        'eval "some text"->append(2)\_s*' ..
        '\d PUSHS "some text"\_s*' ..
        '\d PUSHNR 2\_s*' ..
        '\d SHUFFLE 2 up 1\_s*' ..
        '\d BCALL append(argc 2)\_s*' ..
        '\d DROP\_s*' ..
        '\d RETURN void',
        res)
enddef


def s:SilentMessage()
  silent echomsg "text"
  silent! echoerr "error"
enddef

def Test_silent()
  var res = execute('disass s:SilentMessage')
  assert_match('<SNR>\d*_SilentMessage\_s*' ..
        'silent echomsg "text"\_s*' ..
        '\d CMDMOD silent\_s*' ..
        '\d PUSHS "text"\_s*' ..
        '\d ECHOMSG 1\_s*' ..
        '\d CMDMOD_REV\_s*' ..
        'silent! echoerr "error"\_s*' ..
        '\d CMDMOD silent!\_s*' ..
        '\d PUSHS "error"\_s*' ..
        '\d ECHOERR 1\_s*' ..
        '\d CMDMOD_REV\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:SilentIf()
  silent if 4 == g:five
  silent elseif 4 == g:five
  endif
enddef

def Test_silent_if()
  var res = execute('disass s:SilentIf')
  assert_match('<SNR>\d*_SilentIf\_s*' ..
        'silent if 4 == g:five\_s*' ..
        '\d\+ CMDMOD silent\_s*' ..
        '\d\+ PUSHNR 4\_s*' ..
        '\d\+ LOADG g:five\_s*' ..
        '\d\+ COMPAREANY ==\_s*' ..
        '\d\+ CMDMOD_REV\_s*' ..
        '\d\+ JUMP_IF_FALSE -> \d\+\_s*' ..
        'silent elseif 4 == g:five\_s*' ..
        '\d\+ JUMP -> \d\+\_s*' ..
        '\d\+ CMDMOD silent\_s*' ..
        '\d\+ PUSHNR 4\_s*' ..
        '\d\+ LOADG g:five\_s*' ..
        '\d\+ COMPAREANY ==\_s*' ..
        '\d\+ CMDMOD_REV\_s*' ..
        '\d\+ JUMP_IF_FALSE -> \d\+\_s*' ..
        'endif\_s*' ..
        '\d\+ RETURN void',
        res)
enddef

def s:SilentFor()
  silent for i in [0]
  endfor
enddef

def Test_silent_for()
  var res = execute('disass s:SilentFor')
  assert_match('<SNR>\d*_SilentFor\_s*' ..
        'silent for i in \[0\]\_s*' ..
        '\d CMDMOD silent\_s*' ..
        '\d STORE -1 in $0\_s*' ..
        '\d PUSHNR 0\_s*' ..
        '\d NEWLIST size 1\_s*' ..
        '\d CMDMOD_REV\_s*' ..
        '5 FOR $0 -> 8\_s*' ..
        '\d STORE $2\_s*' ..

        'endfor\_s*' ..
        '\d JUMP -> 5\_s*' ..
        '8 DROP\_s*' ..
        '\d RETURN void\_s*',
        res)
enddef

def s:SilentWhile()
  silent while g:not
  endwhile
enddef

def Test_silent_while()
  var res = execute('disass s:SilentWhile')
  assert_match('<SNR>\d*_SilentWhile\_s*' ..
        'silent while g:not\_s*' ..
        '0 CMDMOD silent\_s*' ..
        '\d LOADG g:not\_s*' ..
        '\d COND2BOOL\_s*' ..
        '\d CMDMOD_REV\_s*' ..
        '\d WHILE $0 -> 6\_s*' ..

        'endwhile\_s*' ..
        '\d JUMP -> 0\_s*' ..
        '6 RETURN void\_s*',
         res)
enddef

def s:SilentReturn(): string
  silent return "done"
enddef

def Test_silent_return()
  var res = execute('disass s:SilentReturn')
  assert_match('<SNR>\d*_SilentReturn\_s*' ..
        'silent return "done"\_s*' ..
        '\d CMDMOD silent\_s*' ..
        '\d PUSHS "done"\_s*' ..
        '\d CMDMOD_REV\_s*' ..
        '\d RETURN',
        res)
enddef

def s:Profiled(): string
  # comment
  echo "profiled"
  # comment
  var some = "some text"
  return "done"
enddef

def Test_profiled()
  if !has('profile')
    MissingFeature 'profile'
  endif
  var res = execute('disass profile s:Profiled')
  assert_match('<SNR>\d*_Profiled\_s*' ..
        '# comment\_s*' ..
        'echo "profiled"\_s*' ..
        '\d PROFILE START line 2\_s*' ..
        '\d PUSHS "profiled"\_s*' ..
        '\d ECHO 1\_s*' ..
        '# comment\_s*' ..
        'var some = "some text"\_s*' ..
        '\d PROFILE END\_s*' ..
        '\d PROFILE START line 4\_s*' ..
        '\d PUSHS "some text"\_s*' ..
        '\d STORE $0\_s*' ..
        'return "done"\_s*' ..
        '\d PROFILE END\_s*' ..
        '\d PROFILE START line 5\_s*' ..
        '\d PUSHS "done"\_s*' ..
        '\d\+ RETURN\_s*' ..
        '\d\+ PROFILE END',
        res)
enddef

def Test_debugged()
  var res = execute('disass debug s:Profiled')
  assert_match('<SNR>\d*_Profiled\_s*' ..
        '# comment\_s*' ..
        'echo "profiled"\_s*' ..
        '\d DEBUG line 1-2 varcount 0\_s*' ..
        '\d PUSHS "profiled"\_s*' ..
        '\d ECHO 1\_s*' ..
        '# comment\_s*' ..
        'var some = "some text"\_s*' ..
        '\d DEBUG line 3-4 varcount 0\_s*' ..
        '\d PUSHS "some text"\_s*' ..
        '\d STORE $0\_s*' ..
        'return "done"\_s*' ..
        '\d DEBUG line 5-5 varcount 1\_s*' ..
        '\d PUSHS "done"\_s*' ..
        '\d RETURN\_s*',
        res)
enddef

def s:ElseifConstant()
  if g:value
    echo "one"
  elseif true
    echo "true"
  elseif false
    echo "false"
  endif
  if 0
    echo "yes"
  elseif 0
    echo "no"
  endif
enddef

def Test_debug_elseif_constant()
  var res = execute('disass debug s:ElseifConstant')
  assert_match('<SNR>\d*_ElseifConstant\_s*' ..
          'if g:value\_s*' ..
          '0 DEBUG line 1-1 varcount 0\_s*' ..
          '1 LOADG g:value\_s*' ..
          '2 COND2BOOL\_s*' ..
          '3 JUMP_IF_FALSE -> 8\_s*' ..
          'echo "one"\_s*' ..
          '4 DEBUG line 2-2 varcount 0\_s*' ..
          '5 PUSHS "one"\_s*' ..
          '6 ECHO 1\_s*' ..
          'elseif true\_s*' ..
          '7 JUMP -> 12\_s*' ..
          '8 DEBUG line 3-3 varcount 0\_s*' ..
          'echo "true"\_s*' ..
          '9 DEBUG line 4-4 varcount 0\_s*' ..
          '10 PUSHS "true"\_s*' ..
          '11 ECHO 1\_s*' ..
          'elseif false\_s*' ..
          'echo "false"\_s*' ..
          'endif\_s*' ..
          'if 0\_s*' ..
          '12 DEBUG line 8-8 varcount 0\_s*' ..
          'echo "yes"\_s*' ..
          'elseif 0\_s*' ..
          '13 DEBUG line 11-10 varcount 0\_s*' ..
          'echo "no"\_s*' ..
          'endif\_s*' ..
          '14 RETURN void*',
        res)
enddef

def s:DebugElseif()
  var b = false
  if b
    eval 1 + 0
  silent elseif !b
    eval 2 + 0
  endif
enddef

def Test_debug_elseif()
  var res = execute('disass debug s:DebugElseif')
  assert_match('<SNR>\d*_DebugElseif\_s*' ..
          'var b = false\_s*' ..
          '0 DEBUG line 1-1 varcount 0\_s*' ..
          '1 PUSH false\_s*' ..
          '2 STORE $0\_s*' ..

          'if b\_s*' ..
          '3 DEBUG line 2-2 varcount 1\_s*' ..
          '4 LOAD $0\_s*' ..
          '5 JUMP_IF_FALSE -> 10\_s*' ..

          'eval 1 + 0\_s*' ..
          '6 DEBUG line 3-3 varcount 1\_s*' ..
          '7 PUSHNR 1\_s*' ..
          '8 DROP\_s*' ..

          'silent elseif !b\_s*' ..
          '9 JUMP -> 20\_s*' ..
          '10 CMDMOD silent\_s*' ..
          '11 DEBUG line 4-4 varcount 1\_s*' ..
          '12 LOAD $0\_s*' ..
          '13 INVERT -1 (!val)\_s*' ..
          '14 CMDMOD_REV\_s*' ..
          '15 JUMP_IF_FALSE -> 20\_s*' ..

          'eval 2 + 0\_s*' ..
          '16 DEBUG line 5-5 varcount 1\_s*' ..
          '17 PUSHNR 2\_s*' ..
          '18 DROP\_s*' ..

          'endif\_s*' ..
          '19 DEBUG line 6-6 varcount 1\_s*' ..
          '20 RETURN void*',
        res)
enddef

def s:DebugFor()
  echo "hello"
  for a in [0]
    echo a
  endfor
enddef

def Test_debug_for()
  var res = execute('disass debug s:DebugFor')
  assert_match('<SNR>\d*_DebugFor\_s*' ..
          'echo "hello"\_s*' ..
          '0 DEBUG line 1-1 varcount 0\_s*' ..
          '1 PUSHS "hello"\_s*' ..
          '2 ECHO 1\_s*' ..

          'for a in \[0\]\_s*' ..
          '3 DEBUG line 2-2 varcount 0\_s*' ..
          '4 STORE -1 in $0\_s*' ..
          '5 PUSHNR 0\_s*' ..
          '6 NEWLIST size 1\_s*' ..
          '7 DEBUG line 2-2 varcount 3\_s*' ..
          '8 FOR $0 -> 15\_s*' ..
          '9 STORE $2\_s*' ..

          'echo a\_s*' ..
          '10 DEBUG line 3-3 varcount 3\_s*' ..
          '11 LOAD $2\_s*' ..
          '12 ECHO 1\_s*' ..

          'endfor\_s*' ..
          '13 DEBUG line 4-4 varcount 3\_s*' ..
          '14 JUMP -> 7\_s*' ..
          '15 DROP\_s*' ..
          '16 RETURN void*',
        res)
enddef

def s:TryCatch()
  try
    echo "try"
  catch /error/
    echo "caught"
  endtry
enddef

def Test_debug_try_catch()
  var res = execute('disass debug s:TryCatch')
  assert_match('<SNR>\d*_TryCatch\_s*' ..
          'try\_s*' ..
          '0 DEBUG line 1-1 varcount 0\_s*' ..
          '1 TRY catch -> 7, endtry -> 17\_s*' ..
          'echo "try"\_s*' ..
          '2 DEBUG line 2-2 varcount 0\_s*' ..
          '3 PUSHS "try"\_s*' ..
          '4 ECHO 1\_s*' ..
          'catch /error/\_s*' ..
          '5 DEBUG line 3-3 varcount 0\_s*' ..
          '6 JUMP -> 17\_s*' ..
          '7 DEBUG line 4-3 varcount 0\_s*' ..
          '8 PUSH v:exception\_s*' ..
          '9 PUSHS "error"\_s*' ..
          '10 COMPARESTRING =\~\_s*' ..
          '11 JUMP_IF_FALSE -> 17\_s*' ..
          '12 CATCH\_s*' ..
          'echo "caught"\_s*' ..
          '13 DEBUG line 4-4 varcount 0\_s*' ..
          '14 PUSHS "caught"\_s*' ..
          '15 ECHO 1\_s*' ..
          'endtry\_s*' ..
          '16 DEBUG line 5-5 varcount 0\_s*' ..
          '17 ENDTRY\_s*' ..
          '\d\+ RETURN void',
        res)
enddef

func s:Legacy() dict
  echo 'legacy'
endfunc

def s:UseMember()
  var d = {func: Legacy}
  var v = d.func()
enddef

def Test_disassemble_dict_stack()
  var res = execute('disass s:UseMember')
  assert_match('<SNR>\d*_UseMember\_s*' ..
          'var d = {func: Legacy}\_s*' ..
          '\d PUSHS "func"\_s*' ..
          '\d PUSHFUNC "<80><fd>R\d\+_Legacy"\_s*' ..
          '\d NEWDICT size 1\_s*' ..
          '\d SETTYPE dict<func(...): any>\_s*' ..
          '\d STORE $0\_s*' ..

          'var v = d.func()\_s*' ..
          '\d LOAD $0\_s*' ..
          '\d MEMBER func\_s*' ..
          '\d PCALL top (argc 0)\_s*' ..
          '\d PCALL end\_s*' ..
          '\d CLEARDICT\_s*' ..
          '\d\+ STORE $1\_s*' ..
          '\d\+ RETURN void*',
        res)
enddef

def s:RetLegacy(): string
  legacy return "yes"
enddef

def Test_disassemble_return_legacy()
  var res = execute('disass s:RetLegacy')
  assert_match('<SNR>\d*_RetLegacy\_s*' ..
          'legacy return "yes"\_s*' ..
          '\d CMDMOD legacy\_s*' ..
          '\d EVAL legacy "yes"\_s*' ..
          '\d CHECKTYPE string stack\[-1]\_s*' ..
          '\d CMDMOD_REV\_s*' ..
          '\d RETURN',
        res)
enddef

def s:EchoMessages()
  echohl ErrorMsg | echom v:exception | echohl NONE
enddef

def Test_disassemble_nextcmd()
  # splitting commands and removing trailing blanks should not change the line
  var res = execute('disass s:EchoMessages')
  assert_match('<SNR>\d*_EchoMessages\_s*' ..
        'echohl ErrorMsg | echom v:exception | echohl NONE',
        res)
enddef

def Test_disassemble_after_reload()
  var lines =<< trim END
      vim9script
      if exists('g:ThisFunc')
        finish
      endif
      var name: any
      def g:ThisFunc(): number
        g:name = name
        return 0
      enddef
      def g:ThatFunc(): number
        name = g:name
        return 0
      enddef
  END
  lines->writefile('Xreload.vim', 'D')

  source Xreload.vim
  g:ThisFunc()
  g:ThatFunc()

  source Xreload.vim
  var res = execute('disass g:ThisFunc')
  assert_match('ThisFunc\_s*' ..
        'g:name = name\_s*' ..
        '\d LOADSCRIPT \[deleted\] from .*/Xreload.vim\_s*' ..
        '\d STOREG g:name\_s*' ..
        'return 0\_s*' ..
        '\d PUSHNR 0\_s*' ..
        '\d RETURN\_s*',
        res)

  res = execute('disass g:ThatFunc')
  assert_match('ThatFunc\_s*' ..
        'name = g:name\_s*' ..
        '\d LOADG g:name\_s*' ..
        '\d STORESCRIPT \[deleted\] in .*/Xreload.vim\_s*' ..
        'return 0\_s*' ..
        '\d PUSHNR 0\_s*' ..
        '\d RETURN\_s*',
        res)

  delfunc g:ThisFunc
  delfunc g:ThatFunc
enddef

def s:MakeString(x: number): string
  return $"x={x} x^2={x * x}"
enddef

def Test_disassemble_string_interp()
  var instr = execute('disassemble s:MakeString')
  assert_match('MakeString\_s*' ..
        'return $"x={x} x^2={x \* x}"\_s*' ..
        '0 PUSHS "x="\_s*' ..
        '1 LOAD arg\[-1\]\_s*' ..
        '2 2STRING stack\[-1\]\_s*' ..
        '3 PUSHS " x^2="\_s*' ..
        '4 LOAD arg\[-1\]\_s*' ..
        '5 LOAD arg\[-1\]\_s*' ..
        '6 OPNR \*\_s*' ..
        '7 2STRING stack\[-1\]\_s*' ..
        '8 CONCAT size 4\_s*' ..
        '9 RETURN\_s*',
        instr)
enddef

def BitShift()
  var a = 1 << 2
  var b = 8 >> 1
  var c = a << b
  var d = b >> a
enddef

def Test_disassemble_bitshift()
  var instr = execute('disassemble BitShift')
  assert_match('BitShift\_s*' ..
               'var a = 1 << 2\_s*' ..
               '0 STORE 4 in $0\_s*' ..
               'var b = 8 >> 1\_s*' ..
               '1 STORE 4 in $1\_s*' ..
               'var c = a << b\_s*' ..
               '2 LOAD $0\_s*' ..
               '3 LOAD $1\_s*' ..
               '4 OPNR <<\_s*' ..
               '5 STORE $2\_s*' ..
               'var d = b >> a\_s*' ..
               '6 LOAD $1\_s*' ..
               '7 LOAD $0\_s*' ..
               '8 OPNR >>\_s*' ..
               '9 STORE $3\_s*' ..
               '10 RETURN void', instr)
enddef

def s:OneDefer()
  defer delete("file")
enddef

def Test_disassemble_defer()
  var instr = execute('disassemble s:OneDefer')
  assert_match('OneDefer\_s*' ..
        'defer delete("file")\_s*' ..
        '\d PUSHFUNC "delete"\_s*' ..
        '\d PUSHS "file"\_s*' ..
        '\d DEFER 1 args\_s*' ..
        '\d RETURN\_s*',
        instr)
enddef

def Test_disassemble_class_function()
  var lines =<< trim END
      vim9script

      class Cl
          static def Fc(): string
            return "x"
          enddef
      endclass

      g:instr = execute('disassemble Cl.Fc')
  END
  v9.CheckScriptSuccess(lines)
  assert_match('Fc\_s*' ..
        'return "x"\_s*' ..
        '\d PUSHS "x"\_s*' ..
        '\d RETURN\_s*',
        g:instr)

  lines =<< trim END
      vim9script

      class Cl
          def Fo(): string
            return "y"
          enddef
      endclass

      g:instr = execute('disassemble Cl.Fo')
  END
  v9.CheckScriptSuccess(lines)
  assert_match('Fo\_s*' ..
        'return "y"\_s*' ..
        '\d PUSHS "y"\_s*' ..
        '\d RETURN\_s*',
        g:instr)

  unlet g:instr
enddef

" Disassemble instructions for using an interface with static and regular member
" variables.
def Test_disassemble_interface_static_member()
  var lines =<< trim END
    vim9script
    interface I
      var o_var: number
      var o_var2: number
    endinterface

    class C implements I
      public static var s_var: number
      var o_var: number
      public static var s_var2: number
      var o_var2: number
    endclass

    def F1(i: I)
      var x: number
      x = i.o_var
      x = i.o_var2
    enddef

    def F2(o: C)
      var x: number
      x = o.o_var
      x = o.o_var2
    enddef

    g:instr1 = execute('disassemble F1')
    g:instr2 = execute('disassemble F2')
  END
  v9.CheckScriptSuccess(lines)
  assert_match('<SNR>\d*_F1\_s*' ..
    'var x: number\_s*' ..
    'x = i.o_var\_s*' ..
    '0 LOAD arg\[-1\]\_s*' ..
    '1 ITF_MEMBER 0 on I\_s*' ..
    '2 STORE $0\_s*' ..
    'x = i.o_var2\_s*' ..
    '3 LOAD arg\[-1\]\_s*' ..
    '4 ITF_MEMBER 1 on I\_s*' ..
    '5 STORE $0\_s*' ..
    '6 RETURN void\_s*',
    g:instr1)
  assert_match('<SNR>\d*_F2\_s*' ..
    'var x: number\_s*' ..
    'x = o.o_var\_s*' ..
    '0 LOAD arg\[-1\]\_s*' ..
    '1 OBJ_MEMBER 0\_s*' ..
    '2 STORE $0\_s*' ..
    'x = o.o_var2\_s*' ..
    '3 LOAD arg\[-1\]\_s*' ..
    '4 OBJ_MEMBER 1\_s*' ..
    '5 STORE $0\_s*' ..
    '6 RETURN void',
    g:instr2)

  unlet g:instr1
  unlet g:instr2
enddef

" Disassemble instructions for loading and storing class variables
def Test_disassemble_class_variable()
  var lines =<< trim END
    vim9script

    class A
      public static var val = 10
      def Foo(): number
        val = 20
        return val
      enddef
    endclass

    g:instr = execute('disassemble A.Foo')
  END
  v9.CheckScriptSuccess(lines)
  assert_match('Foo\_s*' ..
    'val = 20\_s*' ..
    '0 PUSHNR 20\_s*' ..
    '1 STORE CLASSMEMBER A.val\_s*' ..
    'return val\_s*' ..
    '2 LOAD CLASSMEMBER A.val\_s*' ..
    '3 RETURN', g:instr)

  unlet g:instr
enddef

" Disassemble instructions for METHODCALL
def Test_disassemble_methodcall()
  var lines =<< trim END
    vim9script
    interface A
      def Foo()
    endinterface
    def Bar(a: A)
      a.Foo()
    enddef
    g:instr = execute('disassemble Bar')
  END
  v9.CheckScriptSuccess(lines)
  assert_match('<SNR>\d*_Bar\_s*' ..
    'a.Foo()\_s*' ..
    '0 LOAD arg\[-1\]\_s*' ..
    '1 METHODCALL A.Foo(argc 0)\_s*' ..
    '2 DROP\_s*' ..
    '3 RETURN void', g:instr)

  unlet g:instr
enddef

" Disassemble instructions for ISN_JUMP_IF_ARG_NOT_SET
def Test_disassemble_ifargnotset()
  var lines =<< trim END
    vim9script
    class A
      var val: number = 10
    endclass
    g:instr = execute('disassemble A.new')
  END
  v9.CheckScriptSuccess(lines)
  assert_match('new\_s*' ..
    '0 NEW A size \d\+\_s*' ..
    '1 PUSHNR 10\_s*' ..
    '2 STORE_THIS 0\_s*' ..
    'ifargisset 0 this.val = val\_s*' ..
    '3 JUMP_IF_ARG_NOT_SET arg\[-1\] -> 8\_s*' ..
    '4 LOAD arg\[-1\]\_s*' ..
    '5 PUSHNR 0\_s*' ..
    '6 LOAD $0\_s*' ..
    '7 STOREINDEX object\_s*' ..
    '8 RETURN object', g:instr)

  unlet g:instr
enddef

" Disassemble instructions for ISN_COMPAREOBJECT
def Test_disassemble_compare_class_object()
  var lines =<< trim END
    vim9script
    class A
    endclass
    class B
    endclass
    def Foo(a: A, b: B)
      if a == b
      endif
    enddef
    g:instr = execute('disassemble Foo')
  END
  v9.CheckScriptSuccess(lines)
  assert_match('<SNR>\d*_Foo\_s*' ..
    'if a == b\_s*' ..
    '0 LOAD arg\[-2\]\_s*' ..
    '1 LOAD arg\[-1\]\_s*' ..
    '2 COMPAREOBJECT ==\_s*' ..
    '3 JUMP_IF_FALSE -> 4\_s*' ..
    'endif\_s*' ..
    '4 RETURN void', g:instr)
  unlet g:instr
enddef

" Disassemble instructions for ISN_CHECKTYPE with a float|number
def Test_checktype_float()
  var lines =<< trim END
    vim9script
    def Foo()
      var f: float = 0.0
      var a: any
      f += a
    enddef
    g:instr = execute('disassemble Foo')
  END
  v9.CheckScriptSuccess(lines)
  assert_match('<SNR>\d*_Foo\_s*' ..
    'var f: float = 0.0\_s*' ..
    '0 PUSHF 0.0\_s*' ..
    '1 STORE $0\_s*' ..
    'var a: any\_s*' ..
    'f += a\_s*' ..
    '2 LOAD $0\_s*' ..
    '3 LOAD $1\_s*' ..
    '4 CHECKTYPE float|number stack\[-1\]\_s*' ..
    '5 OPANY +\_s*' ..
    '6 STORE $0\_s*' ..
    '7 RETURN void', g:instr)
  unlet g:instr
enddef

" Disassemble instructions for ISN_FUNCREF with a class
def Test_funcref_with_class()
  var lines =<< trim END
    vim9script
    class A
      def Foo()
      enddef
    endclass
    class B extends A
      def Foo()
      enddef
    endclass
    def Bar(a: A)
      defer a.Foo()
    enddef
    g:instr = execute('disassemble Bar')
  END
  v9.CheckScriptSuccess(lines)
  assert_match('<SNR>\d*_Bar\_s*' ..
    'defer a.Foo()\_s*' ..
    '0 LOAD arg\[-1\]\_s*' ..
    '1 FUNCREF A.Foo\_s*' ..
    '2 DEFER 0 args\_s*' ..
    '3 RETURN void', g:instr)
  unlet g:instr
enddef

" vim: ts=8 sw=2 sts=2 expandtab tw=80 fdm=marker
