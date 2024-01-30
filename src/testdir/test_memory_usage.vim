" Tests for memory usage.

source check.vim
CheckFeature terminal
CheckNotGui

" Skip tests on Travis CI ASAN build because it's difficult to estimate memory
" usage.
CheckNotAsan

source shared.vim

func s:pick_nr(str) abort
  return substitute(a:str, '[^0-9]', '', 'g') * 1
endfunc

if has('win32')
  if !executable('wmic')
    throw 'Skipped: wmic program missing'
  endif
  func s:memory_usage(pid) abort
    let cmd = printf('wmic process where processid=%d get WorkingSetSize', a:pid)
    return s:pick_nr(system(cmd)) / 1024
  endfunc
elseif has('unix')
  if !executable('ps')
    throw 'Skipped: ps program missing'
  endif
  func s:memory_usage(pid) abort
    return s:pick_nr(system('ps -o rss= -p ' . a:pid))
  endfunc
else
  throw 'Skipped: not win32 or unix'
endif

" Wait for memory usage to level off.
func s:monitor_memory_usage(pid) abort
  let proc = {}
  let proc.pid = a:pid
  let proc.hist = []
  let proc.max = 0

  func proc.op() abort
    " Check the last 200ms.
    let val = s:memory_usage(self.pid)
    if self.max < val
      let self.max = val
    endif
    call add(self.hist, val)
    if len(self.hist) < 20
      return 0
    endif
    let sample = remove(self.hist, 0)
    return len(uniq([sample] + self.hist)) == 1
  endfunc

  call WaitFor({-> proc.op()}, 10000)
  return {'last': get(proc.hist, -1), 'max': proc.max}
endfunc

let s:term_vim = {}

func s:term_vim.start(...) abort
  let self.buf = term_start([GetVimProg()] + a:000)
  let self.job = term_getjob(self.buf)
  call WaitFor({-> job_status(self.job) ==# 'run'})
  let self.pid = job_info(self.job).process

  " running an external command may fail once in a while
  let g:test_is_flaky = 1
endfunc

func s:term_vim.stop() abort
  call term_sendkeys(self.buf, ":qall!\<CR>")
  call WaitFor({-> job_status(self.job) ==# 'dead'})
  exe self.buf . 'bwipe!'
endfunc

func s:vim_new() abort
  return copy(s:term_vim)
endfunc

func Test_memory_func_capture_vargs()
  " Case: if a local variable captures a:000, funccall object will be free
  " just after it finishes.
  let testfile = 'Xtest.vim'
  let lines =<< trim END
        func s:f(...)
          let x = a:000
        endfunc
        for _ in range(10000)
          call s:f(0)
        endfor
  END
  call writefile(lines, testfile, 'D')

  let vim = s:vim_new()
  call vim.start('--clean', '-c', 'set noswapfile', testfile)
  let before = s:monitor_memory_usage(vim.pid).last

  call term_sendkeys(vim.buf, ":so %\<CR>")
  call WaitFor({-> term_getcursor(vim.buf)[0] == 1})
  let after = s:monitor_memory_usage(vim.pid)

  " Estimate the limit of max usage as 2x initial usage.
  " The lower limit can fluctuate a bit, use 97%.
  call assert_inrange(before * 97 / 100, 2 * before, after.max)

  " In this case, garbage collecting is not needed.
  " The value might fluctuate a bit, allow for 3% tolerance below and 5% above.
  " Based on various test runs.
  let lower = after.last * 97 / 100
  let upper = after.last * 105 / 100
  call assert_inrange(lower, upper, after.max)

  call vim.stop()
endfunc

func Test_memory_func_capture_lvars()
  " Case: if a local variable captures l: dict, funccall object will not be
  " free until garbage collector runs, but after that memory usage doesn't
  " increase so much even when rerun Xtest.vim since system memory caches.
  let testfile = 'Xtest.vim'
  let lines =<< trim END
        func s:f()
          let x = l:
        endfunc
        for _ in range(10000)
          call s:f()
        endfor
  END
  call writefile(lines, testfile, 'D')

  let vim = s:vim_new()
  call vim.start('--clean', '-c', 'set noswapfile', testfile)
  let before = s:monitor_memory_usage(vim.pid).last

  call term_sendkeys(vim.buf, ":so %\<CR>")
  call WaitFor({-> term_getcursor(vim.buf)[0] == 1})
  let after = s:monitor_memory_usage(vim.pid)

  " Rerun Xtest.vim.
  for _ in range(3)
    call term_sendkeys(vim.buf, ":so %\<CR>")
    call WaitFor({-> term_getcursor(vim.buf)[0] == 1})
    let last = s:monitor_memory_usage(vim.pid).last
  endfor

  " The usage may be a bit less than the last value, use 80%.
  " Allow for 20% tolerance at the upper limit.  That's very permissive, but
  " otherwise the test fails sometimes.  On Cirrus CI with FreeBSD we need to
  " be even much more permissive.
  if has('bsd')
    let multiplier = 19
  else
    let multiplier = 12
  endif
  let lower = before * 8 / 10
  let upper = (after.max + (after.last - before)) * multiplier / 10
  call assert_inrange(lower, upper, last)

  call vim.stop()
endfunc

" vim: shiftwidth=2 sts=2 expandtab
