" Test for "rvim" or "vim -Z"

source shared.vim

"if has('win32') && has('gui')
"  " Win32 GUI shows a dialog instead of displaying the error in the last line.
"  finish
"endif

func Test_restricted_mode()
  let lines =<< trim END
    if has('lua')
      call assert_fails('lua print("Hello, Vim!")', 'E981:')
      call assert_fails('luado return "hello"', 'E981:')
      call assert_fails('luafile somefile', 'E981:')
      call assert_fails('call luaeval("expression")', 'E145:')
    endif

    if has('mzscheme')
      call assert_fails('mzscheme statement', 'E981:')
      call assert_fails('mzfile somefile', 'E981:')
      call assert_fails('call mzeval("expression")', 'E145:')
    endif

    if has('perl')
      " TODO: how to make Safe mode fail?
      " call assert_fails('perl system("ls")', 'E981:')
      " call assert_fails('perldo system("hello")', 'E981:')
      " call assert_fails('perlfile somefile', 'E981:')
      " call assert_fails('call perleval("system(\"ls\")")', 'E145:')
    endif

    if has('python')
      call assert_fails('python print "hello"', 'E981:')
      call assert_fails('pydo return "hello"', 'E981:')
      call assert_fails('pyfile somefile', 'E981:')
      call assert_fails('call pyeval("expression")', 'E145:')
    endif

    if has('python3')
      call assert_fails('py3 print "hello"', 'E981:')
      call assert_fails('py3do return "hello"', 'E981:')
      call assert_fails('py3file somefile', 'E981:')
      call assert_fails('call py3eval("expression")', 'E145:')
    endif

    if has('ruby')
      call assert_fails('ruby print "Hello"', 'E981:')
      call assert_fails('rubydo print "Hello"', 'E981:')
      call assert_fails('rubyfile somefile', 'E981:')
    endif

    if has('tcl')
      call assert_fails('tcl puts "Hello"', 'E981:')
      call assert_fails('tcldo puts "Hello"', 'E981:')
      call assert_fails('tclfile somefile', 'E981:')
    endif

    if has('clientserver')
      call assert_fails('let s=remote_peek(10)', 'E145:')
      call assert_fails('let s=remote_read(10)', 'E145:')
      call assert_fails('let s=remote_send("vim", "abc")', 'E145:')
      call assert_fails('let s=server2client(10, "abc")', 'E145:')
    endif

    if has('terminal')
      call assert_fails('terminal', 'E145:')
      call assert_fails('call term_start("vim")', 'E145:')
      call assert_fails('call term_dumpwrite(1, "Xfile")', 'E145:')
    endif

    if has('channel')
      call assert_fails("call ch_logfile('Xlog')", 'E145:')
      call assert_fails("call ch_open('localhost:8765')", 'E145:')
    endif

    if has('job')
      call assert_fails("call job_start('vim')", 'E145:')
    endif

    if has('unix') && has('libcall')
      call assert_fails("echo libcall('libc.so', 'getenv', 'HOME')", 'E145:')
    endif
    call assert_fails("call rename('a', 'b')", 'E145:')
    call assert_fails("call delete('Xfile')", 'E145:')
    call assert_fails("call mkdir('Xdir')", 'E145:')
    call assert_fails('!ls', 'E145:')
    call assert_fails('shell', 'E145:')
    call assert_fails('stop', 'E145:')
    call assert_fails('exe "normal \<C-Z>"', 'E145:')
    set insertmode
    call assert_fails('call feedkeys("\<C-Z>", "xt")', 'E145:')
    set insertmode&
    call assert_fails('suspend', 'E145:')
    call assert_fails('call system("ls")', 'E145:')
    call assert_fails('call systemlist("ls")', 'E145:')
    if has('unix')
      call assert_fails('cd `pwd`', 'E145:')
    endif

    call writefile(v:errors, 'Xresult')
    qa!
  END
  call writefile(lines, 'Xrestricted', 'D')
  if RunVim([], [], '-Z --clean -S Xrestricted')
    call assert_equal([], readfile('Xresult'))
  endif
  call delete('Xresult')
  if has('unix') && RunVimPiped([], [], '--clean -S Xrestricted', 'SHELL=/bin/false ')
    call assert_equal([], readfile('Xresult'))
  endif
  call delete('Xresult')
  if has('unix') && RunVimPiped([], [], '--clean -S Xrestricted', 'SHELL=/sbin/nologin')
    call assert_equal([], readfile('Xresult'))
  endif

  call delete('Xresult')
endfunc

" vim: shiftwidth=2 sts=2 expandtab
