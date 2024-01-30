" This test is in a separate file, because it usually causes reports for memory
" leaks under valgrind.  That is because when fork/exec fails memory is not
" freed.  Since the process exits right away it's not a real leak.

source check.vim
CheckFeature terminal

source shared.vim

func Test_terminal_redir_fails()
  CheckUnix

  let buf = term_start('xyzabc', {'err_io': 'file', 'err_name': 'Xfile'})
  call TermWait(buf)
  call WaitFor('len(readfile("Xfile")) > 0')
  call assert_match('executing job failed', readfile('Xfile')[0])
  call WaitFor('!&modified')
  call delete('Xfile')
  bwipe
endfunc

" vim: shiftwidth=2 sts=2 expandtab
