" Test for Vim9 script with failures, causing memory leaks to be reported.
" The leaks happen after a fork() and can be ignored.

source check.vim

def Test_assignment()
  if !has('channel')
    CheckFeature channel
  else
    var chan1: channel
    var job1: job
    var job2: job = job_start('willfail')
  endif
enddef

" Unclear why this test causes valgrind to report problems.
def Test_job_info_return_type()
  if !has('job')
    CheckFeature job
  else
    var job: job = job_start(&shell)
    var jobs = job_info()
    assert_equal('list<job>', typename(jobs))
    assert_equal('dict<any>', typename(job_info(jobs[0])))
    job_stop(job)
  endif
enddef

" Using "idx" from a legacy global function does not work.
" This caused a crash when called from legacy context.
" This creates a dict that contains a partial that refers to the dict, causing
" valgrind to report "possibly leaked memory".
func Test_partial_call_fails()
  let lines =<< trim END
      vim9script

      var l = ['a', 'b', 'c']
      def Iter(container: any): any
        var idx = -1
        var obj = {state: container}
        def g:NextItem__(self: dict<any>): any
          ++idx
          return self.state[idx]
        enddef
        obj.__next__ = function('g:NextItem__', [obj])
        return obj
      enddef

      var it = Iter(l)
      echo it.__next__()
  END
  call writefile(lines, 'XpartialCall', 'D')
  let caught = 'no'
  try
    source XpartialCall
  catch /E1248:/
    let caught = 'yes'
  endtry
  call assert_equal('yes', caught)
  delfunc g:NextItem__
endfunc

