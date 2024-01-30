" Test for MzScheme interface and mzeval() function

source check.vim
CheckFeature mzscheme

func MzRequire()
  redir => l:mzversion
  mz (version)
  redir END
  if strpart(l:mzversion, 1, 1) < "4"
    " MzScheme versions < 4.x:
    mz (require (prefix vim- vimext))
  else
    " newer versions:
    mz (require (prefix-in vim- 'vimext))
    mz (require r5rs)
  endif
endfunc

func Test_mzscheme()
  new
  let lines =<< trim END
    1 line 1
    2 line 2
    3 line 3
  END
  call setline(1, lines)

  call MzRequire()
  mz (define l '("item0" "dictionary with list OK" "item2"))
  mz (define h (make-hash))
  mz (hash-set! h "list" l)

  call cursor(1, 1)
  " change buffer contents
  mz (vim-set-buff-line (vim-eval "line('.')") "1 changed line 1")
  call assert_equal('1 changed line 1', getline(1))

  " scalar test
  let tmp_string = mzeval('"string"')
  let tmp_1000 = '1000'->mzeval()
  call assert_equal('string1000', tmp_string .. tmp_1000)

  " dictionary containing a list
  call assert_equal('dictionary with list OK', mzeval("h")["list"][1])

  call cursor(2, 1)
  " circular list (at the same time test lists containing lists)
  mz (set-car! (cddr l) l)
  let l2 = mzeval("h")["list"]
  call assert_equal(l2[2], l2)

  " funcrefs
  mz (define vim:max (vim-eval "function('max')"))
  mz (define m (vim:max '(1 100 8)))
  let m = mzeval('m')
  call assert_equal(100, m)

  close!
endfunc

" vim: shiftwidth=2 sts=2 expandtab
