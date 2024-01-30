" Test for setting 'encoding' to something else than the terminal uses, then
" setting 'termencoding' to make it work.

" This only works with "iconv".
if !has('iconv')
  throw 'Skipped: iconv feature missing'
endif

source screendump.vim
if !CanRunVimInTerminal()
  throw 'Skipped: cannot make screendumps'
endif

" This Vim is running with 'encoding' "utf-8", the Vim in the terminal is
" running with 'encoding' "euc-jp".  We need to make sure the text is in the
" right encoding, this is a bit tricky.
func Test_termencoding_euc_jp()
  new
  call setline(1, 'E89: バッファ %ld の変更は保存されていません (! で変更を破棄)')
  write ++enc=euc-jp Xeuc_jp.txt
  quit

  call writefile([
	\ 'set encoding=euc-jp',
	\ 'set termencoding=utf-8',
	\ 'scriptencoding utf-8',
	\ 'exe "normal aE83: バッファを作成できないので、他のを使用します...\<Esc>"',
	\ 'split Xeuc_jp.txt',
	\ ], 'XTest_tenc_euc_jp', 'D')
  let buf = RunVimInTerminal('-S XTest_tenc_euc_jp', {'rows': 10})
  call VerifyScreenDump(buf, 'Test_tenc_euc_jp_01', {})

  " clean up
  call StopVimInTerminal(buf)
  call delete('Xeuc_jp.txt')
endfunc

" vim: shiftwidth=2 sts=2 expandtab
