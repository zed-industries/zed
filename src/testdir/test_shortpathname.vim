" Test for shortpathname ':8' extension.
" Only for use on Win32 systems!

set encoding=utf-8
scriptencoding utf-8

source check.vim
CheckMSWindows

func TestIt(file, bits, expected)
  let res = fnamemodify(a:file, a:bits)
  if a:expected != ''
    call assert_equal(substitute(a:expected, '/', '\\', 'g'),
		\ substitute(res, '/', '\\', 'g'),
		\ "'" . a:file . "'->(" . a:bits . ")->'" . res . "'")
  endif
endfunc

func Test_ColonEight()
  let save_dir = getcwd()

  " This could change for CygWin to //cygdrive/c .
  let dir1 = 'c:/x.x.y'
  let trycount = 5
  while 1
    if !filereadable(dir1) && !isdirectory(dir1)
      break
    endif
    if trycount == 1
      call assert_report("Fatal: '" . dir1 . "' exists, cannot run this test")
      return
    endif
    " When tests run in parallel the directory may exist, wait a bit until it
    " is gone.
    sleep 5
    let trycount -= 1
  endwhile

  let file1 = dir1 . '/zz.y.txt'
  let nofile1 = dir1 . '/z.y.txt'
  let dir2 = dir1 . '/VimIsTheGreatestSinceSlicedBread'
  let file2 = dir2 . '/z.txt'
  let nofile2 = dir2 . '/zz.txt'

  call mkdir(dir1, 'D')
  let resdir1 = substitute(fnamemodify(dir1, ':p:8'), '/$', '', '')
  call assert_match('\V\^c:/XX\x\x\x\x~1.Y\$', resdir1)

  let resfile1 = resdir1 . '/ZZY~1.TXT'
  let resnofile1 = resdir1 . '/z.y.txt'
  let resdir2 = resdir1 . '/VIMIST~1'
  let resfile2 = resdir2 . '/z.txt'
  let resnofile2 = resdir2 . '/zz.txt'

  call mkdir(dir2, 'D')
  call writefile([], file1, 'D')
  call writefile([], file2, 'D')

  call TestIt(file1, ':p:8', resfile1)
  call TestIt(nofile1, ':p:8', resnofile1)
  call TestIt(file2, ':p:8', resfile2)
  call TestIt(nofile2, ':p:8', resnofile2)
  call TestIt(nofile2, ':p:8:h', fnamemodify(resnofile2, ':h'))
  call chdir(dir1)
  call TestIt(file1, ':.:8', strpart(resfile1, strlen(resdir1)+1))
  call TestIt(nofile1, ':.:8', strpart(resnofile1, strlen(resdir1)+1))
  call TestIt(file2, ':.:8', strpart(resfile2, strlen(resdir1)+1))
  call TestIt(nofile2, ':.:8', strpart(resnofile2, strlen(resdir1)+1))
  let $HOME=dir1
  call TestIt(file1, ':~:8', '~' . strpart(resfile1, strlen(resdir1)))
  call TestIt(nofile1, ':~:8', '~' . strpart(resnofile1, strlen(resdir1)))
  call TestIt(file2, ':~:8', '~' . strpart(resfile2, strlen(resdir1)))
  call TestIt(nofile2, ':~:8', '~' . strpart(resnofile2, strlen(resdir1)))

  cd c:/

  call chdir(save_dir)
endfunc

func Test_ColonEight_MultiByte()
  let dir = 'Xtest'

  let file = dir . '/日本語のファイル.txt'

  call mkdir(dir, 'D')
  call writefile([], file, 'D')

  let sfile = fnamemodify(file, ':8')

  call assert_notequal(file, sfile)
  call assert_match('\~', sfile)
endfunc

func Test_ColonEight_notexists()
  let non_exists='C:\windows\newfile.txt'
  call assert_equal(non_exists, fnamemodify(non_exists, ':p:8'))
endfunc

" vim: shiftwidth=2 sts=2 expandtab
