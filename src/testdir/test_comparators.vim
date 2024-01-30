" Test for comparators

function Test_Comparators()
  try
    let oldisident=&isident
    set isident+=#
    call assert_equal(1, 1 is#1)
  finally
    let &isident=oldisident
  endtry
endfunction

" vim: shiftwidth=2 sts=2 expandtab
