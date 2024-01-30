" Tests for 'balloonevalterm'.
" A few tests only work in the terminal.

source check.vim
CheckNotGui
CheckFeature balloon_eval_term

source screendump.vim
CheckScreendump

let s:common_script =<< trim [CODE]
  call setline(1, ["one one one", "two tXo two", "three three three"])
  set balloonevalterm balloonexpr=MyBalloonExpr()..s:trailing balloondelay=100
  let s:trailing = '<'  " check that script context is set
  func MyBalloonExpr()
    return "line " .. v:beval_lnum .. " column " .. v:beval_col .. ":\n" .. v:beval_text
  endfun
  redraw
[CODE]

func Test_balloon_eval_term()
  " Use <Ignore> after <MouseMove> to return from vgetc() without removing
  " the balloon.
  let xtra_lines =<< trim [CODE]
    set updatetime=300
    au CursorHold * echo 'hold fired'
    func Trigger()
      call test_setmouse(2, 6)
      call feedkeys("\<MouseMove>\<Ignore>", "xt")
    endfunc
  [CODE]
  call writefile(s:common_script + xtra_lines, 'XTest_beval', 'D')

  " Check that the balloon shows up after a mouse move
  let buf = RunVimInTerminal('-S XTest_beval', {'rows': 10, 'cols': 50})
  call TermWait(buf, 50)
  call term_sendkeys(buf, 'll')
  call term_sendkeys(buf, ":call Trigger()\<CR>")
  call VerifyScreenDump(buf, 'Test_balloon_eval_term_01', {})

  " Make sure the balloon still shows after 'updatetime' passed and CursorHold
  " was triggered.
  call TermWait(buf, 150)
  call VerifyScreenDump(buf, 'Test_balloon_eval_term_01a', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_balloon_eval_term_visual()
  " Use <Ignore> after <MouseMove> to return from vgetc() without removing
  " the balloon.
  call writefile(s:common_script + [
	\ 'call test_setmouse(3, 6)',
	\ 'call feedkeys("3Gevfr\<MouseMove>\<Ignore>", "xt")',
	\ ], 'XTest_beval_visual', 'D')

  " Check that the balloon shows up after a mouse move
  let buf = RunVimInTerminal('-S XTest_beval_visual', {'rows': 10, 'cols': 50})
  call TermWait(buf, 50)
  call VerifyScreenDump(buf, 'Test_balloon_eval_term_02', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" vim: shiftwidth=2 sts=2 expandtab
