" Tests for the :checkpath command

" Test for 'include' without \zs or \ze
func Test_checkpath1()
  call mkdir("Xcheckdir1/dir2", "pR")
  call writefile(['#include    "bar.a"'], 'Xcheckdir1/dir2/foo.a')
  call writefile(['#include    "baz.a"'], 'Xcheckdir1/dir2/bar.a')
  call writefile(['#include    "foo.a"'], 'Xcheckdir1/dir2/baz.a')
  call writefile(['#include    <foo.a>'], 'Xbase.a')

  edit Xbase.a
  set path=Xcheckdir1/dir2
  let res = split(execute("checkpath!"), "\n")
  call assert_equal([
	      \ '--- Included files in path ---',
	      \ 'Xcheckdir1/dir2/foo.a',
	      \ 'Xcheckdir1/dir2/foo.a -->',
	      \ '  Xcheckdir1/dir2/bar.a',
	      \ '  Xcheckdir1/dir2/bar.a -->',
	      \ '    Xcheckdir1/dir2/baz.a',
	      \ '    Xcheckdir1/dir2/baz.a -->',
	      \ '      "foo.a"  (Already listed)'], res)

  enew
  call delete("./Xbase.a")
  set path&
endfunc

func DotsToSlashes()
  return substitute(v:fname, '\.', '/', 'g') . '.b'
endfunc

" Test for 'include' with \zs and \ze
func Test_checkpath2()
  call mkdir("Xcheckdir2/dir2", "pR")
  call writefile(['%inc    /bar/'], 'Xcheckdir2/dir2/foo.b')
  call writefile(['%inc    /baz/'], 'Xcheckdir2/dir2/bar.b')
  call writefile(['%inc    /foo/'], 'Xcheckdir2/dir2/baz.b')
  call writefile(['%inc    /foo/'], 'Xbase.b', 'D')

  let &include='^\s*%inc\s*/\zs[^/]\+\ze'
  let &includeexpr='DotsToSlashes()'

  edit Xbase.b
  set path=Xcheckdir2/dir2
  let res = split(execute("checkpath!"), "\n")
  call assert_equal([
	      \ '--- Included files in path ---',
	      \ 'Xcheckdir2/dir2/foo.b',
	      \ 'Xcheckdir2/dir2/foo.b -->',
	      \ '  Xcheckdir2/dir2/bar.b',
	      \ '  Xcheckdir2/dir2/bar.b -->',
	      \ '    Xcheckdir2/dir2/baz.b',
	      \ '    Xcheckdir2/dir2/baz.b -->',
	      \ '      foo  (Already listed)'], res)

  enew
  set path&
  set include&
  set includeexpr&
endfunc

func StripNewlineChar()
  if v:fname =~ '\n$'
    return v:fname[:-2]
  endif
  return v:fname
endfunc

" Test for 'include' with \zs and no \ze
func Test_checkpath3()
  call mkdir("Xcheckdir3/dir2", "pR")
  call writefile(['%inc    bar.c'], 'Xcheckdir3/dir2/foo.c')
  call writefile(['%inc    baz.c'], 'Xcheckdir3/dir2/bar.c')
  call writefile(['%inc    foo.c'], 'Xcheckdir3/dir2/baz.c')
  call writefile(['%inc    foo.c'], 'Xcheckdir3/dir2/FALSE.c')
  call writefile(['%inc    FALSE.c foo.c'], 'Xbase.c', 'D')

  let &include='^\s*%inc\s*\%([[:upper:]][^[:space:]]*\s\+\)\?\zs\S\+\ze'
  let &includeexpr='StripNewlineChar()'

  edit Xbase.c
  set path=Xcheckdir3/dir2
  let res = split(execute("checkpath!"), "\n")
  call assert_equal([
	      \ '--- Included files in path ---',
	      \ 'Xcheckdir3/dir2/foo.c',
	      \ 'Xcheckdir3/dir2/foo.c -->',
	      \ '  Xcheckdir3/dir2/bar.c',
	      \ '  Xcheckdir3/dir2/bar.c -->',
	      \ '    Xcheckdir3/dir2/baz.c',
	      \ '    Xcheckdir3/dir2/baz.c -->',
	      \ '      foo.c  (Already listed)'], res)

  enew
  set path&
  set include&
  set includeexpr&
endfunc

" Test for invalid regex in 'include' and 'define' options
func Test_checkpath_errors()
  let save_include = &include
  set include=\\%(
  call assert_fails('checkpath', 'E53:')
  let &include = save_include

  let save_define = &define
  set define=\\%(
  call assert_fails('dsearch abc', 'E53:')
  let &define = save_define

  call assert_fails('psearch \%(', 'E53:')
endfunc

" vim: shiftwidth=2 sts=2 expandtab
