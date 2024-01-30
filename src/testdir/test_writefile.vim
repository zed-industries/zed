" Tests for the writefile() function and some :write commands.

source check.vim
source term_util.vim

func Test_writefile()
  let f = tempname()
  call writefile(["over", "written"], f, "bD")
  call writefile(["hello", "world"], f, "b")
  call writefile(["!", "good"], f, "a")
  call writefile(["morning"], f, "ab")
  call writefile(["", "vimmers"], f, "ab")
  let l = readfile(f)
  call assert_equal("hello", l[0])
  call assert_equal("world!", l[1])
  call assert_equal("good", l[2])
  call assert_equal("morning", l[3])
  call assert_equal("vimmers", l[4])

  call assert_fails('call writefile("text", "Xwffile")', 'E475: Invalid argument: writefile() first argument must be a List or a Blob')
endfunc

func Test_writefile_ignore_regexp_error()
  write Xt[z-a]est.txt
  call delete('Xt[z-a]est.txt')
endfunc

func Test_writefile_fails_gently()
  call assert_fails('call writefile(["test"], "Xwffile", [])', 'E730:')
  call assert_false(filereadable("Xwffile"))
  call delete("Xwffile")

  call assert_fails('call writefile(["test", [], [], [], "tset"], "Xwffile")', 'E730:')
  call assert_false(filereadable("Xwffile"))
  call delete("Xwffile")

  call assert_fails('call writefile([], "Xwffile", [])', 'E730:')
  call assert_false(filereadable("Xwffile"))
  call delete("Xwffile")

  call assert_fails('call writefile([], [])', 'E730:')
endfunc

func Test_writefile_fails_conversion()
  CheckFeature iconv
  if has('sun')
    throw 'Skipped: does not work on SunOS'
  endif
  " Without a backup file the write won't happen if there is a conversion
  " error.
  set nobackup nowritebackup backupdir=. backupskip=
  new
  let contents = ["line one", "line two"]
  call writefile(contents, 'Xwfcfile', 'D')
  edit Xwfcfile
  call setline(1, ["first line", "cannot convert \u010b", "third line"])
  call assert_fails('write ++enc=cp932', 'E513:')
  call assert_equal(contents, readfile('Xwfcfile'))

  " With 'backupcopy' set, if there is a conversion error, the backup file is
  " still created.
  set backupcopy=yes writebackup& backup&
  call delete('Xwfcfile' .. &backupext)
  call assert_fails('write ++enc=cp932', 'E513:')
  call assert_equal(contents, readfile('Xwfcfile'))
  call assert_equal(contents, readfile('Xwfcfile' .. &backupext))
  set backupcopy&
  %bw!

  " Conversion error during write
  new
  call setline(1, ["\U10000000"])
  let output = execute('write! ++enc=utf-16 Xwfcfile')
  call assert_match('CONVERSION ERROR', output)
  let output = execute('write! ++enc=ucs-2 Xwfcfile')
  call assert_match('CONVERSION ERROR', output)
  call delete('Xwfcfilz~')
  call delete('Xwfcfily~')
  %bw!

  call delete('Xwfcfile' .. &backupext)
  bwipe!
  set backup& writebackup& backupdir&vim backupskip&vim
endfunc

func Test_writefile_fails_conversion2()
  CheckFeature iconv
  if has('sun')
    throw 'Skipped: does not work on SunOS'
  endif
  " With a backup file the write happens even if there is a conversion error,
  " but then the backup file must remain
  set nobackup writebackup backupdir=. backupskip=
  let contents = ["line one", "line two"]
  call writefile(contents, 'Xwf2file_conversion_err', 'D')
  edit Xwf2file_conversion_err
  call setline(1, ["first line", "cannot convert \u010b", "third line"])
  set fileencoding=latin1
  let output = execute('write')
  call assert_match('CONVERSION ERROR', output)
  call assert_equal(contents, readfile('Xwf2file_conversion_err~'))

  call delete('Xwf2file_conversion_err~')
  bwipe!
  set backup& writebackup& backupdir&vim backupskip&vim
endfunc

func SetFlag(timer)
  let g:flag = 1
endfunc

func Test_write_quit_split()
  " Prevent exiting by splitting window on file write.
  augroup testgroup
    autocmd BufWritePre * split
  augroup END
  e! Xwqsfile
  call setline(1, 'nothing')
  wq

  if has('timers')
    " timer will not run if "exiting" is still set
    let g:flag = 0
    call timer_start(1, 'SetFlag')
    sleep 50m
    call assert_equal(1, g:flag)
    unlet g:flag
  endif
  au! testgroup
  bwipe Xwqsfile
  call delete('Xwqsfile')
endfunc

func Test_nowrite_quit_split()
  " Prevent exiting by opening a help window.
  e! Xnqsfile
  help
  wincmd w
  exe winnr() . 'q'

  if has('timers')
    " timer will not run if "exiting" is still set
    let g:flag = 0
    call timer_start(1, 'SetFlag')
    sleep 50m
    call assert_equal(1, g:flag)
    unlet g:flag
  endif
  bwipe Xnqsfile
endfunc

func Test_writefile_sync_arg()
  " This doesn't check if fsync() works, only that the argument is accepted.
  call writefile(['one'], 'Xtest', 'sD')
  call writefile(['two'], 'Xtest', 'S')
endfunc

func Test_writefile_sync_dev_stdout()
  CheckUnix
  if filewritable('/dev/stdout')
    " Just check that this doesn't cause an error.
    call writefile(['one'], '/dev/stdout')
  else
    throw 'Skipped: /dev/stdout is not writable'
  endif
endfunc

func Test_writefile_autowrite()
  set autowrite
  new
  next Xa Xb Xc
  call setline(1, 'aaa')
  next
  call assert_equal(['aaa'], readfile('Xa'))
  call setline(1, 'bbb')
  call assert_fails('edit XX')
  call assert_false(filereadable('Xb'))

  set autowriteall
  edit XX
  call assert_equal(['bbb'], readfile('Xb'))

  bwipe!
  call delete('Xa')
  call delete('Xb')
  set noautowrite
endfunc

func Test_writefile_autowrite_nowrite()
  set autowrite
  new
  next Xa Xb Xc
  set buftype=nowrite
  call setline(1, 'aaa')
  let buf = bufnr('%')
  " buffer contents silently lost
  edit XX
  call assert_false(filereadable('Xa'))
  rewind
  call assert_equal('', getline(1))

  bwipe!
  set noautowrite
endfunc

" Test for ':w !<cmd>' to pipe lines from the current buffer to an external
" command.
func Test_write_pipe_to_cmd()
  CheckUnix
  new
  call setline(1, ['L1', 'L2', 'L3', 'L4'])
  2,3w !cat > Xptfile
  call assert_equal(['L2', 'L3'], readfile('Xptfile'))
  close!
  call delete('Xptfile')
endfunc

" Test for :saveas
func Test_saveas()
  call assert_fails('saveas', 'E471:')
  call writefile(['L1'], 'Xsafile')
  new Xsafile
  new
  call setline(1, ['L1'])
  call assert_fails('saveas Xsafile', 'E139:')
  close!
  enew | only
  call delete('Xsafile')

  " :saveas should detect and set the file type.
  syntax on
  saveas! Xsaveas.pl
  call assert_equal('perl', &filetype)
  syntax off
  %bw!
  call delete('Xsaveas.pl')

  " :saveas fails for "nofile" buffer
  set buftype=nofile
  call assert_fails('saveas Xsafile', 'E676: No matching autocommands for buftype=nofile buffer')

  bwipe!
endfunc

func Test_write_errors()
  " Test for writing partial buffer
  call writefile(['L1', 'L2', 'L3'], 'Xwefile')
  new Xwefile
  call assert_fails('1,2write', 'E140:')
  close!

  call assert_fails('w > Xtest', 'E494:')
 
  " Try to overwrite a directory
  if has('unix')
    call mkdir('Xwerdir1')
    call assert_fails('write Xwerdir1', 'E17:')
    call delete('Xwerdir1', 'd')
  endif

  " Test for :wall for a buffer with no name
  enew | only
  call setline(1, ['L1'])
  call assert_fails('wall', 'E141:')
  enew!

  " Test for writing a 'readonly' file
  new Xwefile
  set readonly
  call assert_fails('write', 'E45:')
  close

  " Test for writing to a read-only file
  new Xwefile
  call setfperm('Xwefile', 'r--r--r--')
  call assert_fails('write', 'E505:')
  call setfperm('Xwefile', 'rw-rw-rw-')
  close

  call delete('Xwefile')

  call writefile(test_null_list(), 'Xwefile')
  call assert_false(filereadable('Xwefile'))
  call writefile(test_null_blob(), 'Xwefile')
  call assert_false(filereadable('Xwefile'))
  call assert_fails('call writefile([], "")', 'E482:')

  " very long file name
  let long_fname = repeat('n', 5000)
  call assert_fails('exe "w " .. long_fname', 'E75:')
  call assert_fails('call writefile([], long_fname)', 'E482:')

  " Test for writing to a block device on Unix-like systems
  if has('unix') && getfperm('/dev/loop0') != ''
        \ && getftype('/dev/loop0') == 'bdev' && !IsRoot()
    new
    edit /dev/loop0
    call assert_fails('write', 'E503: ')
    call assert_fails('write!', 'E503: ')
    close!
  endif
endfunc

" Test for writing to a file which is modified after Vim read it
func Test_write_file_mtime()
  CheckEnglish
  CheckRunVimInTerminal

  " First read the file into a buffer
  call writefile(["Line1", "Line2"], 'Xwfmfile', 'D')
  let old_ftime = getftime('Xwfmfile')
  let buf = RunVimInTerminal('Xwfmfile', #{rows : 10})
  call TermWait(buf)
  call term_sendkeys(buf, ":set noswapfile\<CR>")
  call TermWait(buf)

  " Modify the file directly.  Make sure the file modification time is
  " different. Note that on Linux/Unix, the file is considered modified
  " outside, only if the difference is 2 seconds or more
  sleep 1
  call writefile(["Line3", "Line4"], 'Xwfmfile')
  let new_ftime = getftime('Xwfmfile')
  while new_ftime - old_ftime < 2
    sleep 100m
    call writefile(["Line3", "Line4"], 'Xwfmfile')
    let new_ftime = getftime('Xwfmfile')
  endwhile

  " Try to overwrite the file and check for the prompt
  call term_sendkeys(buf, ":w\<CR>")
  call TermWait(buf)
  call WaitForAssert({-> assert_equal("WARNING: The file has been changed since reading it!!!", term_getline(buf, 9))})
  call assert_equal("Do you really want to write to it (y/n)?",
        \ term_getline(buf, 10))
  call term_sendkeys(buf, "n\<CR>")
  call TermWait(buf)
  call assert_equal(new_ftime, getftime('Xwfmfile'))
  call term_sendkeys(buf, ":w\<CR>")
  call TermWait(buf)
  call term_sendkeys(buf, "y\<CR>")
  call TermWait(buf)
  call WaitForAssert({-> assert_equal('Line2', readfile('Xwfmfile')[1])})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" Test for an autocmd unloading a buffer during a write command
func Test_write_autocmd_unloadbuf_lockmark()
  augroup WriteTest
    autocmd BufWritePre Xwaufile enew | write
  augroup END
  e Xwaufile
  call assert_fails('lockmarks write', ['E32:', 'E203:'])
  augroup WriteTest
    au!
  augroup END
  augroup! WriteTest
endfunc

" Test for writing a buffer with 'acwrite' but without autocmds
func Test_write_acwrite_error()
  new Xwaefile
  call setline(1, ['line1', 'line2', 'line3'])
  set buftype=acwrite
  call assert_fails('write', 'E676:')
  call assert_fails('1,2write!', 'E676:')
  call assert_fails('w >>', 'E676:')
  close!
endfunc

" Test for adding and removing lines from an autocmd when writing a buffer
func Test_write_autocmd_add_remove_lines()
  new Xwaafile
  call setline(1, ['aaa', 'bbb', 'ccc', 'ddd'])

  " Autocmd deleting lines from the file when writing a partial file
  augroup WriteTest2
    au!
    autocmd FileWritePre Xwaafile 1,2d
  augroup END
  call assert_fails('2,3w!', 'E204:')

  " Autocmd adding lines to a file when writing a partial file
  augroup WriteTest2
    au!
    autocmd FileWritePre Xwaafile call append(0, ['xxx', 'yyy'])
  augroup END
  %d
  call setline(1, ['aaa', 'bbb', 'ccc', 'ddd'])
  1,2w!
  call assert_equal(['xxx', 'yyy', 'aaa', 'bbb'], readfile('Xwaafile'))

  " Autocmd deleting lines from the file when writing the whole file
  augroup WriteTest2
    au!
    autocmd BufWritePre Xwaafile 1,2d
  augroup END
  %d
  call setline(1, ['aaa', 'bbb', 'ccc', 'ddd'])
  w
  call assert_equal(['ccc', 'ddd'], readfile('Xwaafile'))

  augroup WriteTest2
    au!
  augroup END
  augroup! WriteTest2

  close!
  call delete('Xwaafile')
endfunc

" Test for writing to a readonly file
func Test_write_readonly()
  call writefile([], 'Xwrofile', 'D')
  call setfperm('Xwrofile', "r--------")
  edit Xwrofile
  set noreadonly backupskip=
  call assert_fails('write', 'E505:')
  let save_cpo = &cpo
  set cpo+=W
  call assert_fails('write!', 'E504:')
  let &cpo = save_cpo
  call setline(1, ['line1'])
  write!
  call assert_equal(['line1'], readfile('Xwrofile'))

  " Auto-saving a readonly file should fail with 'autowriteall'
  %bw!
  e Xwrofile
  set noreadonly autowriteall
  call setline(1, ['aaaa'])
  call assert_fails('n', 'E505:')
  set cpo+=W
  call assert_fails('n', 'E504:')
  set cpo-=W
  set autowriteall&

  set backupskip&
  %bw!
endfunc

" Test for 'patchmode'
func Test_patchmode()
  call writefile(['one'], 'Xpafile', 'D')
  set patchmode=.orig nobackup backupskip= writebackup
  new Xpafile
  call setline(1, 'two')
  " first write should create the .orig file
  write
  call assert_equal(['one'], readfile('Xpafile.orig'))
  call setline(1, 'three')
  " subsequent writes should not create/modify the .orig file
  write
  call assert_equal(['one'], readfile('Xpafile.orig'))

  " use 'patchmode' with 'nobackup' and 'nowritebackup' to create an empty
  " original file
  call delete('Xpafile')
  call delete('Xpafile.orig')
  %bw!
  set patchmode=.orig nobackup nowritebackup
  edit Xpafile
  call setline(1, ['xxx'])
  write
  call assert_equal(['xxx'], readfile('Xpafile'))
  call assert_equal([], readfile('Xpafile.orig'))

  set patchmode& backup& backupskip& writebackup&
  call delete('Xpafile.orig')
endfunc

" Test for writing to a file in a readonly directory
" NOTE: if you run tests as root this will fail.  Don't run tests as root!
func Test_write_readonly_dir()
  " On MS-Windows, modifying files in a read-only directory is allowed.
  CheckUnix
  " Root can do it too.
  CheckNotRoot

  call mkdir('Xrodir/', 'R')
  call writefile(['one'], 'Xrodir/Xfile1')
  call setfperm('Xrodir', 'r-xr--r--')
  " try to create a new file in the directory
  new Xrodir/Xfile2
  call setline(1, 'two')
  call assert_fails('write', 'E212:')
  " try to create a backup file in the directory
  edit! Xrodir/Xfile1
  set backupdir=./Xrodir backupskip=
  set patchmode=.orig
  call assert_fails('write', 'E509:')
  call setfperm('Xrodir', 'rwxr--r--')
  set backupdir& backupskip& patchmode&
endfunc

" Test for writing a file using invalid file encoding
func Test_write_invalid_encoding()
  new
  call setline(1, 'abc')
  call assert_fails('write ++enc=axbyc Xiefile', 'E213:')
  close!
endfunc

" Tests for reading and writing files with conversion for Win32.
func Test_write_file_encoding()
  CheckMSWindows
  let save_encoding = &encoding
  let save_fileencodings = &fileencodings
  set encoding=latin1 fileencodings&
  let text =<< trim END
    1 utf-8 text: Ð”Ð»Ñ Vim version 6.2.  ÐŸÐ¾ÑÐ»ÐµÐ´Ð½ÐµÐµ Ð¸Ð·Ð¼ÐµÐ½ÐµÐ½Ð¸Ðµ: 1970 Jan 01
    2 cp1251 text: Äëÿ Vim version 6.2.  Ïîñëåäíåå èçìåíåíèå: 1970 Jan 01
    3 cp866 text: „«ï Vim version 6.2.  ®á«¥¤­¥¥ ¨§¬¥­¥­¨¥: 1970 Jan 01
  END
  call writefile(text, 'Xwfefile', 'D')
  edit Xwfefile

  " write tests:
  " combine three values for 'encoding' with three values for 'fileencoding'
  " also write files for read tests
  call cursor(1, 1)
  set encoding=utf-8
  .w! ++enc=utf-8 Xwfetest
  .w ++enc=cp1251 >> Xwfetest
  .w ++enc=cp866 >> Xwfetest
  .w! ++enc=utf-8 Xutf8
  let expected =<< trim END
    1 utf-8 text: Ð”Ð»Ñ Vim version 6.2.  ÐŸÐ¾ÑÐ»ÐµÐ´Ð½ÐµÐµ Ð¸Ð·Ð¼ÐµÐ½ÐµÐ½Ð¸Ðµ: 1970 Jan 01
    1 utf-8 text: Äëÿ Vim version 6.2.  Ïîñëåäíåå èçìåíåíèå: 1970 Jan 01
    1 utf-8 text: „«ï Vim version 6.2.  ®á«¥¤­¥¥ ¨§¬¥­¥­¨¥: 1970 Jan 01
  END
  call assert_equal(expected, readfile('Xwfetest'))

  call cursor(2, 1)
  set encoding=cp1251
  .w! ++enc=utf-8 Xwfetest
  .w ++enc=cp1251 >> Xwfetest
  .w ++enc=cp866 >> Xwfetest
  .w! ++enc=cp1251 Xcp1251
  let expected =<< trim END
    2 cp1251 text: Ð”Ð»Ñ Vim version 6.2.  ÐŸÐ¾ÑÐ»ÐµÐ´Ð½ÐµÐµ Ð¸Ð·Ð¼ÐµÐ½ÐµÐ½Ð¸Ðµ: 1970 Jan 01
    2 cp1251 text: Äëÿ Vim version 6.2.  Ïîñëåäíåå èçìåíåíèå: 1970 Jan 01
    2 cp1251 text: „«ï Vim version 6.2.  ®á«¥¤­¥¥ ¨§¬¥­¥­¨¥: 1970 Jan 01
  END
  call assert_equal(expected, readfile('Xwfetest'))

  call cursor(3, 1)
  set encoding=cp866
  .w! ++enc=utf-8 Xwfetest
  .w ++enc=cp1251 >> Xwfetest
  .w ++enc=cp866 >> Xwfetest
  .w! ++enc=cp866 Xcp866
  let expected =<< trim END
    3 cp866 text: Ð”Ð»Ñ Vim version 6.2.  ÐŸÐ¾ÑÐ»ÐµÐ´Ð½ÐµÐµ Ð¸Ð·Ð¼ÐµÐ½ÐµÐ½Ð¸Ðµ: 1970 Jan 01
    3 cp866 text: Äëÿ Vim version 6.2.  Ïîñëåäíåå èçìåíåíèå: 1970 Jan 01
    3 cp866 text: „«ï Vim version 6.2.  ®á«¥¤­¥¥ ¨§¬¥­¥­¨¥: 1970 Jan 01
  END
  call assert_equal(expected, readfile('Xwfetest'))

  " read three 'fileencoding's with utf-8 'encoding'
  set encoding=utf-8 fencs=utf-8,cp1251
  e Xutf8
  .w! ++enc=utf-8 Xwfetest
  e Xcp1251
  .w ++enc=utf-8 >> Xwfetest
  set fencs=utf-8,cp866
  e Xcp866
  .w ++enc=utf-8 >> Xwfetest
  let expected =<< trim END
    1 utf-8 text: Ð”Ð»Ñ Vim version 6.2.  ÐŸÐ¾ÑÐ»ÐµÐ´Ð½ÐµÐµ Ð¸Ð·Ð¼ÐµÐ½ÐµÐ½Ð¸Ðµ: 1970 Jan 01
    2 cp1251 text: Ð”Ð»Ñ Vim version 6.2.  ÐŸÐ¾ÑÐ»ÐµÐ´Ð½ÐµÐµ Ð¸Ð·Ð¼ÐµÐ½ÐµÐ½Ð¸Ðµ: 1970 Jan 01
    3 cp866 text: Ð”Ð»Ñ Vim version 6.2.  ÐŸÐ¾ÑÐ»ÐµÐ´Ð½ÐµÐµ Ð¸Ð·Ð¼ÐµÐ½ÐµÐ½Ð¸Ðµ: 1970 Jan 01
  END
  call assert_equal(expected, readfile('Xwfetest'))

  " read three 'fileencoding's with cp1251 'encoding'
  set encoding=utf-8 fencs=utf-8,cp1251
  e Xutf8
  .w! ++enc=cp1251 Xwfetest
  e Xcp1251
  .w ++enc=cp1251 >> Xwfetest
  set fencs=utf-8,cp866
  e Xcp866
  .w ++enc=cp1251 >> Xwfetest
  let expected =<< trim END
    1 utf-8 text: Äëÿ Vim version 6.2.  Ïîñëåäíåå èçìåíåíèå: 1970 Jan 01
    2 cp1251 text: Äëÿ Vim version 6.2.  Ïîñëåäíåå èçìåíåíèå: 1970 Jan 01
    3 cp866 text: Äëÿ Vim version 6.2.  Ïîñëåäíåå èçìåíåíèå: 1970 Jan 01
  END
  call assert_equal(expected, readfile('Xwfetest'))

  " read three 'fileencoding's with cp866 'encoding'
  set encoding=cp866 fencs=utf-8,cp1251
  e Xutf8
  .w! ++enc=cp866 Xwfetest
  e Xcp1251
  .w ++enc=cp866 >> Xwfetest
  set fencs=utf-8,cp866
  e Xcp866
  .w ++enc=cp866 >> Xwfetest
  let expected =<< trim END
    1 utf-8 text: „«ï Vim version 6.2.  ®á«¥¤­¥¥ ¨§¬¥­¥­¨¥: 1970 Jan 01
    2 cp1251 text: „«ï Vim version 6.2.  ®á«¥¤­¥¥ ¨§¬¥­¥­¨¥: 1970 Jan 01
    3 cp866 text: „«ï Vim version 6.2.  ®á«¥¤­¥¥ ¨§¬¥­¥­¨¥: 1970 Jan 01
  END
  call assert_equal(expected, readfile('Xwfetest'))

  call delete('Xwfetest')
  call delete('Xutf8')
  call delete('Xcp1251')
  call delete('Xcp866')
  let &encoding = save_encoding
  let &fileencodings = save_fileencodings
  %bw!
endfunc

" Test for writing and reading a file starting with a BOM.
" Byte Order Mark (BOM) character for various encodings is below:
"     UTF-8      : EF BB BF
"     UTF-16 (BE): FE FF
"     UTF-16 (LE): FF FE
"     UTF-32 (BE): 00 00 FE FF
"     UTF-32 (LE): FF FE 00 00
func Test_readwrite_file_with_bom()
  let utf8_bom = "\xEF\xBB\xBF"
  let utf16be_bom = "\xFE\xFF"
  let utf16le_bom = "\xFF\xFE"
  let utf32be_bom = "\n\n\xFE\xFF"
  let utf32le_bom = "\xFF\xFE\n\n"
  let save_fileencoding = &fileencoding
  set cpoptions+=S

  " Check that editing a latin1 file doesn't see a BOM
  call writefile(["\xFE\xFElatin-1"], 'Xrwtest1', 'D')
  edit Xrwtest1
  call assert_equal('latin1', &fileencoding)
  call assert_equal(0, &bomb)
  set fenc=latin1
  write Xrwfile2
  call assert_equal(["\xFE\xFElatin-1", ''], readfile('Xrwfile2', 'b'))
  set bomb fenc=latin1
  write Xrwtest3
  call assert_equal(["\xFE\xFElatin-1", ''], readfile('Xrwtest3', 'b'))
  set bomb&

  " Check utf-8 BOM
  %bw!
  call writefile([utf8_bom .. "utf-8"], 'Xrwtest1')
  edit! Xrwtest1
  call assert_equal('utf-8', &fileencoding)
  call assert_equal(1, &bomb)
  call assert_equal('utf-8', getline(1))
  set fenc=latin1
  write! Xrwfile2
  call assert_equal(['utf-8', ''], readfile('Xrwfile2', 'b'))
  set fenc=utf-8
  w! Xrwtest3
  call assert_equal([utf8_bom .. "utf-8", ''], readfile('Xrwtest3', 'b'))

  " Check utf-8 with an error (will fall back to latin-1)
  %bw!
  call writefile([utf8_bom .. "utf-8\x80err"], 'Xrwtest1')
  edit! Xrwtest1
  call assert_equal('latin1', &fileencoding)
  call assert_equal(0, &bomb)
  call assert_equal("\xC3\xAF\xC2\xBB\xC2\xBFutf-8\xC2\x80err", getline(1))
  set fenc=latin1
  write! Xrwfile2
  call assert_equal([utf8_bom .. "utf-8\x80err", ''], readfile('Xrwfile2', 'b'))
  set fenc=utf-8
  w! Xrwtest3
  call assert_equal(["\xC3\xAF\xC2\xBB\xC2\xBFutf-8\xC2\x80err", ''],
        \ readfile('Xrwtest3', 'b'))

  " Check ucs-2 BOM
  %bw!
  call writefile([utf16be_bom .. "\nu\nc\ns\n-\n2\n"], 'Xrwtest1')
  edit! Xrwtest1
  call assert_equal('utf-16', &fileencoding)
  call assert_equal(1, &bomb)
  call assert_equal('ucs-2', getline(1))
  set fenc=latin1
  write! Xrwfile2
  call assert_equal(["ucs-2", ''], readfile('Xrwfile2', 'b'))
  set fenc=ucs-2
  w! Xrwtest3
  call assert_equal([utf16be_bom .. "\nu\nc\ns\n-\n2\n", ''],
        \ readfile('Xrwtest3', 'b'))

  " Check ucs-2le BOM
  %bw!
  call writefile([utf16le_bom .. "u\nc\ns\n-\n2\nl\ne\n"], 'Xrwtest1')
  " Need to add a NUL byte after the NL byte
  call writefile(0z00, 'Xrwtest1', 'a')
  edit! Xrwtest1
  call assert_equal('utf-16le', &fileencoding)
  call assert_equal(1, &bomb)
  call assert_equal('ucs-2le', getline(1))
  set fenc=latin1
  write! Xrwfile2
  call assert_equal(["ucs-2le", ''], readfile('Xrwfile2', 'b'))
  set fenc=ucs-2le
  w! Xrwtest3
  call assert_equal([utf16le_bom .. "u\nc\ns\n-\n2\nl\ne\n", "\n"],
        \ readfile('Xrwtest3', 'b'))

  " Check ucs-4 BOM
  %bw!
  call writefile([utf32be_bom .. "\n\n\nu\n\n\nc\n\n\ns\n\n\n-\n\n\n4\n\n\n"], 'Xrwtest1')
  edit! Xrwtest1
  call assert_equal('ucs-4', &fileencoding)
  call assert_equal(1, &bomb)
  call assert_equal('ucs-4', getline(1))
  set fenc=latin1
  write! Xrwfile2
  call assert_equal(["ucs-4", ''], readfile('Xrwfile2', 'b'))
  set fenc=ucs-4
  w! Xrwtest3
  call assert_equal([utf32be_bom .. "\n\n\nu\n\n\nc\n\n\ns\n\n\n-\n\n\n4\n\n\n", ''], readfile('Xrwtest3', 'b'))

  " Check ucs-4le BOM
  %bw!
  call writefile([utf32le_bom .. "u\n\n\nc\n\n\ns\n\n\n-\n\n\n4\n\n\nl\n\n\ne\n\n\n"], 'Xrwtest1')
  " Need to add three NUL bytes after the NL byte
  call writefile(0z000000, 'Xrwtest1', 'a')
  edit! Xrwtest1
  call assert_equal('ucs-4le', &fileencoding)
  call assert_equal(1, &bomb)
  call assert_equal('ucs-4le', getline(1))
  set fenc=latin1
  write! Xrwfile2
  call assert_equal(["ucs-4le", ''], readfile('Xrwfile2', 'b'))
  set fenc=ucs-4le
  w! Xrwtest3
  call assert_equal([utf32le_bom .. "u\n\n\nc\n\n\ns\n\n\n-\n\n\n4\n\n\nl\n\n\ne\n\n\n", "\n\n\n"], readfile('Xrwtest3', 'b'))

  set cpoptions-=S
  let &fileencoding = save_fileencoding
  call delete('Xrwfile2')
  call delete('Xrwtest3')
  %bw!
endfunc

func Test_read_write_bin()
  " write file missing EOL
  call writefile(['noeol'], "XNoEolSetEol", 'bSD')
  call assert_equal(0z6E6F656F6C, readfile('XNoEolSetEol', 'B'))

  " when file is read 'eol' is off
  set nofixeol
  e! ++ff=unix XNoEolSetEol
  call assert_equal(0, &eol)

  " writing with 'eol' set adds the newline
  setlocal eol
  w
  call assert_equal(0z6E6F656F6C0A, readfile('XNoEolSetEol', 'B'))

  set ff& fixeol&
  bwipe! XNoEolSetEol
endfunc

" Test for the 'backupcopy' option when writing files
func Test_backupcopy()
  CheckUnix
  set backupskip=
  " With the default 'backupcopy' setting, saving a symbolic link file
  " should not break the link.
  set backupcopy&
  call writefile(['1111'], 'Xbcfile1')
  silent !ln -s Xbcfile1 Xbcfile2
  new Xbcfile2
  call setline(1, ['2222'])
  write
  close
  call assert_equal(['2222'], readfile('Xbcfile1'))
  call assert_equal('Xbcfile1', resolve('Xbcfile2'))
  call assert_equal('link', getftype('Xbcfile2'))
  call delete('Xbcfile1')
  call delete('Xbcfile2')

  " With the 'backupcopy' set to 'breaksymlink', saving a symbolic link file
  " should break the link.
  set backupcopy=yes,breaksymlink
  call writefile(['1111'], 'Xbcfile1')
  silent !ln -s Xbcfile1 Xbcfile2
  new Xbcfile2
  call setline(1, ['2222'])
  write
  close
  call assert_equal(['1111'], readfile('Xbcfile1'))
  call assert_equal(['2222'], readfile('Xbcfile2'))
  call assert_equal('Xbcfile2', resolve('Xbcfile2'))
  call assert_equal('file', getftype('Xbcfile2'))
  call delete('Xbcfile1')
  call delete('Xbcfile2')
  set backupcopy&

  " With the default 'backupcopy' setting, saving a hard link file
  " should not break the link.
  set backupcopy&
  call writefile(['1111'], 'Xbcfile1')
  silent !ln Xbcfile1 Xbcfile2
  new Xbcfile2
  call setline(1, ['2222'])
  write
  close
  call assert_equal(['2222'], readfile('Xbcfile1'))
  call delete('Xbcfile1')
  call delete('Xbcfile2')

  " With the 'backupcopy' set to 'breaksymlink', saving a hard link file
  " should break the link.
  set backupcopy=yes,breakhardlink
  call writefile(['1111'], 'Xbcfile1')
  silent !ln Xbcfile1 Xbcfile2
  new Xbcfile2
  call setline(1, ['2222'])
  write
  call assert_equal(['1111'], readfile('Xbcfile1'))
  call assert_equal(['2222'], readfile('Xbcfile2'))
  call delete('Xbcfile1')
  call delete('Xbcfile2')

  " If a backup file is already present, then a slightly modified filename
  " should be used as the backup file. Try with 'backupcopy' set to 'yes' and
  " 'no'.
  %bw
  call writefile(['aaaa'], 'Xbcfile')
  call writefile(['bbbb'], 'Xbcfile.bak')
  set backupcopy=yes backupext=.bak
  new Xbcfile
  call setline(1, ['cccc'])
  write
  close
  call assert_equal(['cccc'], readfile('Xbcfile'))
  call assert_equal(['bbbb'], readfile('Xbcfile.bak'))
  set backupcopy=no backupext=.bak
  new Xbcfile
  call setline(1, ['dddd'])
  write
  close
  call assert_equal(['dddd'], readfile('Xbcfile'))
  call assert_equal(['bbbb'], readfile('Xbcfile.bak'))
  call delete('Xbcfile')
  call delete('Xbcfile.bak')

  " Write to a device file (in Unix-like systems) which cannot be backed up.
  if has('unix')
    set writebackup backupcopy=yes nobackup
    new
    call setline(1, ['aaaa'])
    let output = execute('write! /dev/null')
    call assert_match('"/dev/null" \[Device]', output)
    close
    set writebackup backupcopy=no nobackup
    new
    call setline(1, ['aaaa'])
    let output = execute('write! /dev/null')
    call assert_match('"/dev/null" \[Device]', output)
    close
    set backup writebackup& backupcopy&
    new
    call setline(1, ['aaaa'])
    let output = execute('write! /dev/null')
    call assert_match('"/dev/null" \[Device]', output)
    close
  endif

  set backupcopy& backupskip& backupext& backup&
endfunc

" Test for writing a file with 'encoding' set to 'utf-16'
func Test_write_utf16()
  new
  call setline(1, ["\U00010001"])
  write ++enc=utf-16 Xw16file
  bw!
  call assert_equal(0zD800DC01, readfile('Xw16file', 'B')[0:3])
  call delete('Xw16file')
endfunc

" Test for trying to save a backup file when the backup file is a symbolic
" link to the original file. The backup file should not be modified.
func Test_write_backup_symlink()
  CheckUnix
  call mkdir('Xbackup')
  let save_backupdir = &backupdir
  set backupdir=.,./Xbackup
  call writefile(['1111'], 'Xwbsfile', 'D')
  silent !ln -s Xwbsfile Xwbsfile.bak

  new Xwbsfile
  set backup backupcopy=yes backupext=.bak
  write
  call assert_equal('link', getftype('Xwbsfile.bak'))
  call assert_equal('Xwbsfile', resolve('Xwbsfile.bak'))
  " backup file should be created in the 'backup' directory
  if !has('bsd')
    " This check fails on FreeBSD
    call assert_true(filereadable('./Xbackup/Xwbsfile.bak'))
  endif
  set backup& backupcopy& backupext&
  %bw

  call delete('Xwbsfile.bak')
  call delete('Xbackup', 'rf')
  let &backupdir = save_backupdir
endfunc

" Test for ':write ++bin' and ':write ++nobin'
func Test_write_binary_file()
  " create a file without an eol/eof character
  call writefile(0z616161, 'Xwbfile1', 'bD')
  new Xwbfile1
  write ++bin Xwbfile2
  write ++nobin Xwbfile3
  call assert_equal(0z616161, readblob('Xwbfile2'))
  if has('win32')
    call assert_equal(0z6161610D.0A, readblob('Xwbfile3'))
  else
    call assert_equal(0z6161610A, readblob('Xwbfile3'))
  endif
  call delete('Xwbfile2')
  call delete('Xwbfile3')
endfunc

func DoWriteDefer()
  call writefile(['some text'], 'XdeferDelete', 'D')
  call assert_equal(['some text'], readfile('XdeferDelete'))
endfunc

def DefWriteDefer()
  writefile(['some text'], 'XdefdeferDelete', 'D')
  assert_equal(['some text'], readfile('XdefdeferDelete'))
enddef

func Test_write_with_deferred_delete()
  call DoWriteDefer()
  call assert_equal('', glob('XdeferDelete'))
  call DefWriteDefer()
  call assert_equal('', glob('XdefdeferDelete'))
endfunc

func DoWriteFile()
  call writefile(['text'], 'Xthefile', 'D')
  cd ..
endfunc

func Test_write_defer_delete_chdir()
  let dir = getcwd()
  call DoWriteFile()
  call assert_notequal(dir, getcwd())
  call chdir(dir)
  call assert_equal('', glob('Xthefile'))
endfunc

" Check that buffer is written before triggering QuitPre
func Test_wq_quitpre_autocommand()
  edit Xsomefile
  call setline(1, 'hello')
  split
  let g:seq = []
  augroup Testing
    au QuitPre * call add(g:seq, 'QuitPre - ' .. (&modified ? 'modified' : 'not modified'))
    au BufWritePost * call add(g:seq, 'written')
  augroup END
  wq
  call assert_equal(['written', 'QuitPre - not modified'], g:seq)

  augroup Testing
    au!
  augroup END
  bwipe!
  unlet g:seq
  call delete('Xsomefile')
endfunc

func Test_write_with_xattr_support()
  CheckLinux
  CheckFeature xattr
  CheckExecutable setfattr

  let contents = ["file with xattrs", "line two"]
  call writefile(contents, 'Xwattr.txt', 'D')
  " write a couple of xattr
  call system('setfattr -n user.cookie -v chocolate Xwattr.txt')
  call system('setfattr -n user.frieda -v bar Xwattr.txt')
  call system('setfattr -n user.empty Xwattr.txt')

  set backupcopy=no writebackup& backup&
  sp Xwattr.txt
  w
  $r! getfattr -d %
  let expected = ['file with xattrs', 'line two', '# file: Xwattr.txt', 'user.cookie="chocolate"', 'user.empty=""', 'user.frieda="bar"', '']
  call assert_equal(expected, getline(1,'$'))

  set backupcopy&
  bw!
endfunc

" vim: shiftwidth=2 sts=2 expandtab
