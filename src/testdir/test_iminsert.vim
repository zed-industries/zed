" Test for 'iminsert'

source view_util.vim
source check.vim
import './vim9.vim' as v9

let s:imactivatefunc_called = 0
let s:imstatusfunc_called = 0
let s:imstatus_active = 0

func IM_activatefunc(active)
  let s:imactivatefunc_called = 1
  let s:imstatus_active = a:active
endfunc

func IM_statusfunc()
  let s:imstatusfunc_called = 1
  return s:imstatus_active
endfunc

func Test_iminsert2()
  let s:imactivatefunc_called = 0
  let s:imstatusfunc_called = 0

  set imactivatefunc=IM_activatefunc
  set imstatusfunc=IM_statusfunc
  set iminsert=2
  normal! i
  set iminsert=0
  set imactivatefunc=
  set imstatusfunc=

  let expected = (has('win32') && has('gui_running')) ? 0 : 1
  call assert_equal(expected, s:imactivatefunc_called)
  call assert_equal(expected, s:imstatusfunc_called)
endfunc

func Test_getimstatus()
  if has('win32')
    CheckFeature multi_byte_ime
  else
    CheckFeature xim
  endif
  if has('win32') && has('gui_running')
    set imactivatefunc=
    set imstatusfunc=
  else
    set imactivatefunc=IM_activatefunc
    set imstatusfunc=IM_statusfunc
    let s:imstatus_active = 0
  endif

  new
  set iminsert=2
  call feedkeys("i\<C-R>=getimstatus()\<CR>\<ESC>", 'nx')
  call assert_equal('1', getline(1))
  set iminsert=0
  call feedkeys("o\<C-R>=getimstatus()\<CR>\<ESC>", 'nx')
  call assert_equal('0', getline(2))
  bw!

  set imactivatefunc=
  set imstatusfunc=
endfunc

func Test_imactivatefunc_imstatusfunc_callback_no_breaks_foldopen()
  CheckScreendump

  let lines =<< trim END
    func IM_activatefunc(active)
    endfunc
    func IM_statusfunc()
      return 0
    endfunc
    set imactivatefunc=IM_activatefunc
    set imstatusfunc=IM_statusfunc
    set foldmethod=marker
    set foldopen=search
    call setline(1, ['{{{', 'abc', '}}}'])
    %foldclose
  END
  call writefile(lines, 'Xscript', 'D')
  let buf = RunVimInTerminal('-S Xscript', {})
  call assert_notequal('abc', term_getline(buf, 2))
  call term_sendkeys(buf, "/abc\n")
  call WaitForAssert({-> assert_equal('abc', term_getline(buf, 2))})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" Test for using an lmap in insert mode
func Test_lmap_in_insert_mode()
  new
  call setline(1, 'abc')
  lmap { w
  set iminsert=1
  call feedkeys('r{', 'xt')
  call assert_equal('wbc', getline(1))
  set iminsert=2
  call feedkeys('$r{', 'xt')
  call assert_equal('wb{', getline(1))
  call setline(1, 'vim web')
  set iminsert=1
  call feedkeys('0f{', 'xt')
  call assert_equal(5, col('.'))
  set iminsert&
  lunmap {
  close!
endfunc

" Test for using CTRL-^ to toggle iminsert in insert mode
func Test_iminsert_toggle()
  CheckGui
  if has('win32')
    CheckFeature multi_byte_ime
  else
    CheckFeature xim
  endif
  if has('gui_running') && !has('win32')
    throw 'Skipped: works only in Win32 GUI version (for some reason)'
  endif
  new
  let save_imdisable = &imdisable
  let save_iminsert = &iminsert
  set noimdisable
  set iminsert=0
  exe "normal i\<C-^>"
  call assert_equal(2, &iminsert)
  exe "normal i\<C-^>"
  call assert_equal(0, &iminsert)
  let &iminsert = save_iminsert
  let &imdisable = save_imdisable
  close!
endfunc

" Test for different ways of setting the 'imactivatefunc' and 'imstatusfunc'
" options
func Test_imactivatefunc_imstatusfunc_callback()
  CheckNotMSWindows
  func IMactivatefunc1(active)
    let g:IMactivatefunc_called += 1
  endfunc
  func IMstatusfunc1()
    let g:IMstatusfunc_called += 1
    return 1
  endfunc
  set iminsert=2

  let lines =<< trim END
    LET g:IMactivatefunc_called = 0
    LET g:IMstatusfunc_called = 0

    #" Test for using a function name
    LET &imactivatefunc = 'g:IMactivatefunc1'
    LET &imstatusfunc = 'g:IMstatusfunc1'
    normal! i

    #" Test for using a function()
    set imactivatefunc=function('g:IMactivatefunc1')
    set imstatusfunc=function('g:IMstatusfunc1')
    normal! i

    #" Using a funcref variable to set 'completefunc'
    VAR Fn1 = function('g:IMactivatefunc1')
    LET &imactivatefunc = Fn1
    VAR Fn2 = function('g:IMstatusfunc1')
    LET &imstatusfunc = Fn2
    normal! i

    #" Using a string(funcref variable) to set 'completefunc'
    LET &imactivatefunc = string(Fn1)
    LET &imstatusfunc = string(Fn2)
    normal! i

    #" Test for using a funcref()
    set imactivatefunc=funcref('g:IMactivatefunc1')
    set imstatusfunc=funcref('g:IMstatusfunc1')
    normal! i

    #" Using a funcref variable to set 'imactivatefunc'
    LET Fn1 = funcref('g:IMactivatefunc1')
    LET &imactivatefunc = Fn1
    LET Fn2 = funcref('g:IMstatusfunc1')
    LET &imstatusfunc = Fn2
    normal! i

    #" Using a string(funcref variable) to set 'imactivatefunc'
    LET &imactivatefunc = string(Fn1)
    LET &imstatusfunc = string(Fn2)
    normal! i

    #" Test for using a lambda function
    VAR optval = "LSTART a LMIDDLE g:IMactivatefunc1(a) LEND"
    LET optval = substitute(optval, ' ', '\\ ', 'g')
    exe "set imactivatefunc=" .. optval
    LET optval = "LSTART LMIDDLE g:IMstatusfunc1() LEND"
    LET optval = substitute(optval, ' ', '\\ ', 'g')
    exe "set imstatusfunc=" .. optval
    normal! i

    #" Set 'imactivatefunc' and 'imstatusfunc' to a lambda expression
    LET &imactivatefunc = LSTART a LMIDDLE g:IMactivatefunc1(a) LEND
    LET &imstatusfunc = LSTART LMIDDLE g:IMstatusfunc1() LEND
    normal! i

    #" Set 'imactivatefunc' and 'imstatusfunc' to a string(lambda expression)
    LET &imactivatefunc = 'LSTART a LMIDDLE g:IMactivatefunc1(a) LEND'
    LET &imstatusfunc = 'LSTART LMIDDLE g:IMstatusfunc1() LEND'
    normal! i

    #" Set 'imactivatefunc' 'imstatusfunc' to a variable with a lambda
    #" expression
    VAR Lambda1 = LSTART a LMIDDLE g:IMactivatefunc1(a) LEND
    VAR Lambda2 = LSTART LMIDDLE g:IMstatusfunc1() LEND
    LET &imactivatefunc = Lambda1
    LET &imstatusfunc = Lambda2
    normal! i

    #" Set 'imactivatefunc' 'imstatusfunc' to a string(variable with a lambda
    #" expression)
    LET &imactivatefunc = string(Lambda1)
    LET &imstatusfunc = string(Lambda2)
    normal! i

    #" Test for clearing the 'completefunc' option
    set imactivatefunc='' imstatusfunc=''
    set imactivatefunc& imstatusfunc&

    set imactivatefunc=g:IMactivatefunc1
    set imstatusfunc=g:IMstatusfunc1
    call assert_fails("set imactivatefunc=function('abc')", "E700:")
    call assert_fails("set imstatusfunc=function('abc')", "E700:")
    call assert_fails("set imactivatefunc=funcref('abc')", "E700:")
    call assert_fails("set imstatusfunc=funcref('abc')", "E700:")
    call assert_fails("LET &imstatusfunc = function('abc')", "E700:")
    call assert_fails("LET &imactivatefunc = function('abc')", "E700:")
    normal! i

    #" set 'imactivatefunc' and 'imstatusfunc' to a non-existing function
    set imactivatefunc=IMactivatefunc1
    set imstatusfunc=IMstatusfunc1
    call assert_fails("set imactivatefunc=function('NonExistingFunc')", 'E700:')
    call assert_fails("set imstatusfunc=function('NonExistingFunc')", 'E700:')
    call assert_fails("LET &imactivatefunc = function('NonExistingFunc')", 'E700:')
    call assert_fails("LET &imstatusfunc = function('NonExistingFunc')", 'E700:')
    normal! i

    call assert_equal(14, g:IMactivatefunc_called)
    call assert_equal(28, g:IMstatusfunc_called)
  END
  call v9.CheckLegacyAndVim9Success(lines)

  " Using Vim9 lambda expression in legacy context should fail
  set imactivatefunc=(a)\ =>\ IMactivatefunc1(a)
  set imstatusfunc=IMstatusfunc1
  call assert_fails('normal! i', 'E117:')
  set imactivatefunc=IMactivatefunc1
  set imstatusfunc=()\ =>\ IMstatusfunc1(a)
  call assert_fails('normal! i', 'E117:')

  " set 'imactivatefunc' and 'imstatusfunc' to a partial with dict. This used
  " to cause a crash.
  func SetIMFunc()
    let params1 = {'activate': function('g:DictActivateFunc')}
    let params2 = {'status': function('g:DictStatusFunc')}
    let &imactivatefunc = params1.activate
    let &imstatusfunc = params2.status
  endfunc
  func g:DictActivateFunc(_) dict
  endfunc
  func g:DictStatusFunc(_) dict
  endfunc
  call SetIMFunc()
  new
  call SetIMFunc()
  bw
  call test_garbagecollect_now()
  new
  set imactivatefunc=
  set imstatusfunc=
  wincmd w
  set imactivatefunc=
  set imstatusfunc=
  :%bw!
  delfunc g:DictActivateFunc
  delfunc g:DictStatusFunc
  delfunc SetIMFunc

  " Vim9 tests
  let lines =<< trim END
    vim9script

    # Test for using function()
    def IMactivatefunc1(active: number): any
      g:IMactivatefunc_called += 1
      return 1
    enddef
    def IMstatusfunc1(): number
      g:IMstatusfunc_called += 1
      return 1
    enddef
    g:IMactivatefunc_called = 0
    g:IMstatusfunc_called = 0
    set iminsert=2
    set imactivatefunc=function('IMactivatefunc1')
    set imstatusfunc=function('IMstatusfunc1')
    normal! i

    set iminsert=0
    set imactivatefunc=
    set imstatusfunc=
  END
  call v9.CheckScriptSuccess(lines)

  " cleanup
  set iminsert=0
  set imactivatefunc&
  set imstatusfunc&
  delfunc IMactivatefunc1
  delfunc IMstatusfunc1
  unlet g:IMactivatefunc_called g:IMstatusfunc_called
  %bw!
endfunc

" vim: shiftwidth=2 sts=2 expandtab
