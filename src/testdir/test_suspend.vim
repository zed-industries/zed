" Test :suspend

source check.vim
source term_util.vim
source shared.vim

func CheckSuspended(buf, fileExists)
  call WaitForAssert({-> assert_match('[$#] $', term_getline(a:buf, '.'))})

  if a:fileExists
    call assert_equal(['foo'], readfile('Xfoo'))
  else
    " Without 'autowrite', buffer should not be written.
    call assert_equal(0, filereadable('Xfoo'))
  endif

  call term_sendkeys(a:buf, "fg\<CR>\<C-L>")
  call WaitForAssert({-> assert_equal('  1 foo', term_getline(a:buf, '.'))})
endfunc

func Test_suspend()
  CheckFeature terminal
  CheckExecutable /bin/sh

  " Somehow the modifyOtherKeys response may get to the terminal when using
  " Mac OS.  Make t_RK and 'keyprotocol' empty to avoid that.
  set t_RK= keyprotocol=

  call WaitForResponses()

  let buf = term_start('/bin/sh')
  " Wait for shell prompt.
  call WaitForAssert({-> assert_match('[$#] $', term_getline(buf, '.'))})

  call term_sendkeys(buf, v:progpath
        \               . " --clean -X"
        \               . " -c 'set nu keyprotocol='"
        \               . " -c 'call setline(1, \"foo\")'"
        \               . " Xfoo\<CR>")
  " Cursor in terminal buffer should be on first line in spawned vim.
  call WaitForAssert({-> assert_equal('  1 foo', term_getline(buf, '.'))})

  for suspend_cmd in [":suspend\<CR>",
        \             ":stop\<CR>",
        \             ":suspend!\<CR>",
        \             ":stop!\<CR>",
        \             "\<C-Z>"]
    " Suspend and wait for shell prompt.
    call term_sendkeys(buf, suspend_cmd)
    call CheckSuspended(buf, 0)
  endfor

  " Test that :suspend! with 'autowrite' writes content of buffers if modified.
  call term_sendkeys(buf, ":set autowrite\<CR>")
  call assert_equal(0, filereadable('Xfoo'))
  call term_sendkeys(buf, ":suspend\<CR>")
  " Wait for shell prompt.
  call CheckSuspended(buf, 1)

  " Quit gracefully to dump coverage information.
  call term_sendkeys(buf, ":qall!\<CR>")
  call TermWait(buf)
  " Wait until Vim actually exited and shell shows a prompt
  call WaitForAssert({-> assert_match('[$#] $', term_getline(buf, '.'))})
  call StopShellInTerminal(buf)

  exe buf . 'bwipe!'
  call delete('Xfoo')
endfunc

func Test_suspend_autocmd()
  CheckFeature terminal
  CheckExecutable /bin/sh

  " Somehow the modifyOtherKeys response may get to the terminal when using
  " Mac OS.  Make t_RK and 'keyprotocol' empty to avoid that.
  set t_RK= keyprotocol=

  call WaitForResponses()

  let buf = term_start('/bin/sh', #{term_rows: 6})
  " Wait for shell prompt.
  call WaitForAssert({-> assert_match('[$#] $', term_getline(buf, '.'))})

  call term_sendkeys(buf, v:progpath
        \               . " --clean -X"
        \               . " -c 'set nu keyprotocol='"
        \               . " -c 'let g:count = 0'"
        \               . " -c 'au VimSuspend * let g:count += 1'"
        \               . " -c 'au VimResume * let g:count += 1'"
        \               . " -c 'call setline(1, \"foo\")'"
        \               . " Xfoo\<CR>")
  " Cursor in terminal buffer should be on first line in spawned vim.
  call WaitForAssert({-> assert_equal('  1 foo', term_getline(buf, '.'))})

  for suspend_cmd in [":suspend\<CR>",
        \             ":stop\<CR>",
        \             ":suspend!\<CR>",
        \             ":stop!\<CR>",
        \             "\<C-Z>"]
    " Suspend and wait for shell prompt.  Then "fg" will restore Vim.
    call term_sendkeys(buf, suspend_cmd)
    call CheckSuspended(buf, 0)
  endfor

  call term_sendkeys(buf, ":echo g:count\<CR>")
  call TermWait(buf)
  call WaitForAssert({-> assert_match('^10', term_getline(buf, 6))})

  " Quit gracefully to dump coverage information.
  call term_sendkeys(buf, ":qall!\<CR>")
  call TermWait(buf)
  " Wait until Vim actually exited and shell shows a prompt
  call WaitForAssert({-> assert_match('[$#] $', term_getline(buf, '.'))})
  call StopShellInTerminal(buf)

  exe buf . 'bwipe!'
  call delete('Xfoo')
endfunc

" vim: shiftwidth=2 sts=2 expandtab
