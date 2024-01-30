" Test for expanding dllpath options

func s:test_expand_dllpath(optname)
  let $TEST_EXPAND_DLLPATH = '/dllpath/lib' . substitute(a:optname, '\zedll$', '.', '')
  execute 'let dllpath_save = &' . a:optname
  try
    execute 'set ' . a:optname . '=$TEST_EXPAND_DLLPATH'
    execute 'call assert_equal("' . $TEST_EXPAND_DLLPATH . '", &' . a:optname . ')'

    execute 'set ' . a:optname . '=~' . $TEST_EXPAND_DLLPATH
    let home = substitute($HOME, '\\', '/', 'g')
    execute 'call assert_equal("' . home . $TEST_EXPAND_DLLPATH . '", &' . a:optname . ')'
  finally
    execute 'let &' . a:optname . ' = dllpath_save'
    let $TEST_EXPAND_DLLPATH = ''
  endtry
endfunc

func s:generate_test_if_exists(optname)
  if exists('+' . a:optname)
    execute join([
          \ 'func Test_expand_' . a:optname . '()',
          \ '  call s:test_expand_dllpath("' . a:optname . '")',
          \ 'endfunc'
          \ ], "\n")
  endif
endfunc

call s:generate_test_if_exists('luadll')
call s:generate_test_if_exists('perldll')
call s:generate_test_if_exists('pythondll')
call s:generate_test_if_exists('pythonthreedll')
call s:generate_test_if_exists('rubydll')
call s:generate_test_if_exists('tcldll')

" vim: shiftwidth=2 sts=2 expandtab
