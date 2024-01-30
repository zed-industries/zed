" Test glob2regpat()

import './vim9.vim' as v9

func Test_glob2regpat_invalid()
  call assert_equal('^1\.33$', glob2regpat(1.33))
  call v9.CheckDefAndScriptFailure(['echo glob2regpat(1.2)'], ['E1013: Argument 1: type mismatch, expected string but got float', 'E1174: String required for argument 1'])
  call assert_fails('call glob2regpat("}")', 'E219:')
  call assert_fails('call glob2regpat("{")', 'E220:')
endfunc

func Test_glob2regpat_valid()
  call assert_equal('^foo\.', glob2regpat('foo.*'))
  call assert_equal('^foo.$', 'foo?'->glob2regpat())
  call assert_equal('\.vim$', glob2regpat('*.vim'))
  call assert_equal('^[abc]$', glob2regpat('[abc]'))
  call assert_equal('^foo bar$', glob2regpat('foo\ bar'))
  call assert_equal('^foo,bar$', glob2regpat('foo,bar'))
  call assert_equal('^\(foo\|bar\)$', glob2regpat('{foo,bar}'))
  call assert_equal('.*', glob2regpat('**'))

  if exists('+shellslash')
    call assert_equal('^foo[\/].$', glob2regpat('foo\?'))
    call assert_equal('^\(foo[\/]\|bar\|foobar\)$', glob2regpat('{foo\,bar,foobar}'))
    call assert_equal('^[\/]\(foo\|bar[\/]\)$', glob2regpat('\{foo,bar\}'))
    call assert_equal('^[\/][\/]\(foo\|bar[\/][\/]\)$', glob2regpat('\\{foo,bar\\}'))
  else
    call assert_equal('^foo?$', glob2regpat('foo\?'))
    call assert_equal('^\(foo,bar\|foobar\)$', glob2regpat('{foo\,bar,foobar}'))
    call assert_equal('^{foo,bar}$', glob2regpat('\{foo,bar\}'))
    call assert_equal('^\\\(foo\|bar\\\)$', glob2regpat('\\{foo,bar\\}'))
  endif
endfunc

" vim: shiftwidth=2 sts=2 expandtab
