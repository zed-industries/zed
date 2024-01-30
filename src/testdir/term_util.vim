" Functions about terminal shared by several tests

" Only load this script once.
if exists('*CanRunVimInTerminal')
  finish
endif

source shared.vim

" For most tests we need to be able to run terminal Vim with 256 colors.  On
" MS-Windows the console only has 16 colors and the GUI can't run in a
" terminal.
func CanRunVimInTerminal()
  return has('terminal') && !has('win32')
endfunc

" Skip the rest if there is no terminal feature at all.
if !has('terminal')
  finish
endif

" Stops the shell running in terminal "buf".
func StopShellInTerminal(buf)
  call term_sendkeys(a:buf, "exit\r")
  let job = term_getjob(a:buf)
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  call TermWait(a:buf)
endfunc

" Wrapper around term_wait() to allow more time for re-runs of flaky tests
" The second argument is the minimum time to wait in msec, 10 if omitted.
func TermWait(buf, ...)
  let wait_time = a:0 ? a:1 : 10
  if exists('g:run_nr')
    if g:run_nr == 2
      let wait_time *= 4
    elseif g:run_nr > 2
      let wait_time *= 10
    endif
  endif
  call term_wait(a:buf, wait_time)

  " In case it wasn't set yet.
  let g:test_is_flaky = 1
endfunc

" Run Vim with "arguments" in a new terminal window.
" By default uses a size of 20 lines and 75 columns.
" Returns the buffer number of the terminal.
"
" Options is a dictionary, these items are recognized:
" "keep_t_u7" - when 1 do not make t_u7 empty (resetting t_u7 avoids clearing
"               parts of line 2 and 3 on the display)
" "rows" - height of the terminal window (max. 20)
" "cols" - width of the terminal window (max. 78)
" "statusoff" - number of lines the status is offset from default
" "wait_for_ruler" - if zero then don't wait for ruler to show
" "no_clean" - if non-zero then remove "--clean" from the command
" "cmd"  - run any other command, e.g. "xxd" (used in xxd test)
func RunVimInTerminal(arguments, options)
  " If Vim doesn't exit a swap file remains, causing other tests to fail.
  " Remove it here.
  call delete(".swp")

  if exists('$COLORFGBG')
    " Clear $COLORFGBG to avoid 'background' being set to "dark", which will
    " only be corrected if the response to t_RB is received, which may be too
    " late.
    let $COLORFGBG = ''
  endif

  " Make a horizontal and vertical split, so that we can get exactly the right
  " size terminal window.  Works only when the current window is full width.
  call assert_equal(&columns, winwidth(0))
  split
  vsplit

  " Always do this with 256 colors and a light background.
  set t_Co=256 background=light
  hi Normal ctermfg=NONE ctermbg=NONE

  " Make the window 20 lines high and 75 columns, unless told otherwise or
  " 'termwinsize' is set.
  let rows = get(a:options, 'rows', 20)
  let cols = get(a:options, 'cols', 75)
  let statusoff = get(a:options, 'statusoff', 1)

  if get(a:options, 'keep_t_u7', 0)
    let reset_u7 = ''
  else
    let reset_u7 = ' --cmd "set t_u7=" '
  endif

  if empty(get(a:options, 'cmd', ''))
    let cmd = GetVimCommandCleanTerm() .. reset_u7 .. a:arguments
  else
    let cmd = get(a:options, 'cmd')
  endif

  if get(a:options, 'no_clean', 0)
    let cmd = substitute(cmd, '--clean', '', '')
  endif

  let options = #{curwin: 1}
  if &termwinsize == ''
    let options.term_rows = rows
    let options.term_cols = cols
  endif

  " Accept other options whose name starts with 'term_'.
  call extend(options, filter(copy(a:options), 'v:key =~# "^term_"'))

  let buf = term_start(cmd, options)

  if &termwinsize == ''
    " in the GUI we may end up with a different size, try to set it.
    if term_getsize(buf) != [rows, cols]
      call term_setsize(buf, rows, cols)
    endif
    call assert_equal([rows, cols], term_getsize(buf))
  else
    let rows = term_getsize(buf)[0]
    let cols = term_getsize(buf)[1]
  endif

  call TermWait(buf)

  if get(a:options, 'wait_for_ruler', 1) && empty(get(a:options, 'cmd', ''))
    " Wait for "All" or "Top" of the ruler to be shown in the last line or in
    " the status line of the last window. This can be quite slow (e.g. when
    " using valgrind).
    " If it fails then show the terminal contents for debugging.
    try
      call WaitFor({-> len(term_getline(buf, rows)) >= cols - 1 || len(term_getline(buf, rows - statusoff)) >= cols - 1})
    catch /timed out after/
      let lines = map(range(1, rows), {key, val -> term_getline(buf, val)})
      call assert_report('RunVimInTerminal() failed, screen contents: ' . join(lines, "<NL>"))
    endtry
  endif

  " Starting a terminal to run Vim is always considered flaky.
  let g:test_is_flaky = 1

  return buf
endfunc

" Stop a Vim running in terminal buffer "buf".
func StopVimInTerminal(buf, kill = 1)
  " Using a terminal to run Vim is always considered flaky.
  let g:test_is_flaky = 1

  call assert_equal("running", term_getstatus(a:buf))

  " Wait for all the pending updates to terminal to complete
  call TermWait(a:buf)

  " CTRL-O : works both in Normal mode and Insert mode to start a command line.
  " In Command-line it's inserted, the CTRL-U removes it again.
  call term_sendkeys(a:buf, "\<C-O>:\<C-U>qa!\<cr>")

  " Wait for all the pending updates to terminal to complete
  call TermWait(a:buf)

  " Wait for the terminal to end.
  call WaitForAssert({-> assert_equal("finished", term_getstatus(a:buf))})

  " If the buffer still exists forcefully wipe it.
  if a:kill && bufexists(a:buf)
    exe a:buf .. 'bwipe!'
  endif
endfunc

" Open a terminal with a shell, assign the job to g:job and return the buffer
" number.
func Run_shell_in_terminal(options)
  if has('win32')
    let buf = term_start([&shell, '/k'], a:options)
  else
    let buf = term_start(&shell, a:options)
  endif
  let g:test_is_flaky = 1

  let termlist = term_list()
  call assert_equal(1, len(termlist))
  call assert_equal(buf, termlist[0])

  let g:job = term_getjob(buf)
  call assert_equal(v:t_job, type(g:job))

  let string = string({'job': buf->term_getjob()})
  call assert_match("{'job': 'process \\d\\+ run'}", string)

  " On slower systems it may take a bit of time before the shell is ready to
  " accept keys.  This mainly matters when using term_sendkeys() next.
  call TermWait(buf)

  return buf
endfunc

" Return concatenated lines in terminal.
func Term_getlines(buf, lines)
  return join(map(a:lines, 'term_getline(a:buf, v:val)'), '')
endfunc


" vim: shiftwidth=2 sts=2 expandtab
