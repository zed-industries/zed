" Test whether glob()/globpath() return correct results with certain escaped
" characters.

source check.vim

func SetUp()
  " consistent sorting of file names
  set nofileignorecase
endfunction

function Test_glob()
  " This test fails on Windows because of the special characters in the
  " filenames. Disable the test on non-Unix systems for now.
  CheckUnix

  " Execute these commands in the sandbox, so that using the shell fails.
  " Setting 'shell' to an invalid name causes a memory leak.
  sandbox call assert_equal("", glob('Xxx\{'))
  sandbox call assert_equal("", 'Xxx\$'->glob())
  w! Xxx\{
  w! Xxx\$
  sandbox call assert_equal("Xxx{", glob('Xxx\{'))
  sandbox call assert_equal("Xxx$", glob('Xxx\$'))
  call delete('Xxx{')
  call delete('Xxx$')
endfunction

function Test_globpath()
  sandbox call assert_equal("sautest/autoload/globone.vim\nsautest/autoload/globtwo.vim",
        \ globpath('sautest/autoload', 'glob*.vim'))
  sandbox call assert_equal(['sautest/autoload/globone.vim', 'sautest/autoload/globtwo.vim'],
        \ 'glob*.vim'->globpath('sautest/autoload', 0, 1))
endfunction

" vim: shiftwidth=2 sts=2 expandtab
