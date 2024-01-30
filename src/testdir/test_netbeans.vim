" Test the netbeans interface.

source check.vim
CheckFeature netbeans_intg

source shared.vim

let s:python = PythonProg()
if s:python == ''
  throw 'Skipped: python program missing'
endif

" Run "testfunc" after starting the server and stop the server afterwards.
func s:run_server(testfunc, ...)
  call RunServer('test_netbeans.py', a:testfunc, a:000)
endfunc

" Wait for an exception (error) to be thrown. This is used to check whether a
" message from the netbeans server causes an error. It takes some time for Vim
" to process a netbeans message. So a sleep is used below to account for this.
func WaitForError(errcode)
  let save_exception = ''
  for i in range(200)
    try
      sleep 5m
    catch
      let save_exception = v:exception
      break
    endtry
  endfor
  call assert_match(a:errcode, save_exception)
endfunc

" Read the "Xnetbeans" file and filter out geometry messages.
func ReadXnetbeans()
  let l = readfile("Xnetbeans")
  " Xnetbeans may include '0:geometry=' messages in the GUI Vim if the window
  " position, size, or z order are changed.  Remove these messages because
  " these messages will break the assert for the output.
  return filter(l, 'v:val !~ "^0:geometry="')
endfunc

func Nb_basic(port)
  call writefile([], "Xnetbeans", 'D')

  " Last line number in the Xnetbeans file. Used to verify the result of the
  " communication with the netbeans server
  let g:last = 0

  " Establish the connection with the netbeans server
  exe 'nbstart :localhost:' .. a:port .. ':bunny'
  call assert_true(has("netbeans_enabled"))
  call WaitFor('len(ReadXnetbeans()) > (g:last + 2)')
  let l = ReadXnetbeans()
  call assert_equal(['AUTH bunny',
        \ '0:version=0 "2.5"',
        \ '0:startupDone=0'], l[-3:])
  let g:last += 3

  " Trying to connect again to netbeans server should fail
  call assert_fails("exe 'nbstart :localhost:' . a:port . ':bunny'", 'E511:')

  " Open the command buffer to communicate with the server
  split Xcmdbuf
  let cmdbufnr = bufnr()
  call WaitFor('len(ReadXnetbeans()) > (g:last + 2)')
  let l = ReadXnetbeans()
  call assert_equal('0:fileOpened=0 "Xcmdbuf" T F',
        \ substitute(l[-3], '".*/', '"', ''))
  call assert_equal('send: 1:putBufferNumber!15 "Xcmdbuf"',
        \ substitute(l[-2], '".*/', '"', ''))
  call assert_equal('1:startDocumentListen!16', l[-1])
  let g:last += 3

  " Keep the command buffer loaded for communication
  hide

  sleep 1m

  " getCursor test
  call writefile(['foo bar', 'foo bar', 'foo bar'], 'Xfile1', 'D')
  split Xfile1
  call cursor(3, 4)
  sleep 10m
  call appendbufline(cmdbufnr, '$', 'getCursor_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 5)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:getCursor/30', '30 -1 3 3 19'], l[-2:])
  let g:last += 5

  " Test for E627
  call appendbufline(cmdbufnr, '$', 'E627_Test')
  call WaitForError('E627:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0 setReadOnly!31', l[-1])
  let g:last += 3

  " Test for E628
  call appendbufline(cmdbufnr, '$', 'E628_Test')
  call WaitForError('E628:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:setReadOnly 32', l[-1])
  let g:last += 3

  " Test for E632
  call appendbufline(cmdbufnr, '$', 'E632_Test')
  call WaitForError('E632:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:getLength/33', '33 0'], l[-2:])
  let g:last += 4

  " Test for E633
  call appendbufline(cmdbufnr, '$', 'E633_Test')
  call WaitForError('E633:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:getText/34', '34 '], l[-2:])
  let g:last += 4

  " Test for E634
  call appendbufline(cmdbufnr, '$', 'E634_Test')
  call WaitForError('E634:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:remove/35 1 1', '35'], l[-2:])
  let g:last += 4

  " Test for E635
  call appendbufline(cmdbufnr, '$', 'E635_Test')
  call WaitForError('E635:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:insert/36 0 "line1\n"', '36'], l[-2:])
  let g:last += 4

  " Test for E636
  call appendbufline(cmdbufnr, '$', 'E636_Test')
  call WaitForError('E636:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:create!37', l[-1])
  let g:last += 3

  " Test for E637
  call appendbufline(cmdbufnr, '$', 'E637_Test')
  call WaitForError('E637:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:startDocumentListen!38', l[-1])
  let g:last += 3

  " Test for E638
  call appendbufline(cmdbufnr, '$', 'E638_Test')
  call WaitForError('E638:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:stopDocumentListen!39', l[-1])
  let g:last += 3

  " Test for E639
  call appendbufline(cmdbufnr, '$', 'E639_Test')
  call WaitForError('E639:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:setTitle!40 "Title"', l[-1])
  let g:last += 3

  " Test for E640
  call appendbufline(cmdbufnr, '$', 'E640_Test')
  call WaitForError('E640:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:initDone!41', l[-1])
  let g:last += 3

  " Test for E641
  call appendbufline(cmdbufnr, '$', 'E641_Test')
  call WaitForError('E641:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:putBufferNumber!42 "XSomeBuf"', l[-1])
  let g:last += 3

  " Test for E642
  call appendbufline(cmdbufnr, '$', 'E642_Test')
  call WaitForError('E642:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 9:putBufferNumber!43 "XInvalidBuf"', l[-1])
  let g:last += 3

  " Test for E643
  call appendbufline(cmdbufnr, '$', 'E643_Test')
  call WaitForError('E643:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:setFullName!44 "XSomeBuf"', l[-1])
  let g:last += 3

  enew!

  " Test for E644
  call appendbufline(cmdbufnr, '$', 'E644_Test')
  call WaitForError('E644:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:editFile!45 "Xfile3"', l[-1])
  let g:last += 3

  " Test for E645 (shown only when verbose > 0)
  call appendbufline(cmdbufnr, '$', 'E645_Test')
  set verbose=1
  call WaitForError('E645:')
  set verbose&
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:setVisible!46 T', l[-1])
  let g:last += 3

  " Test for E646 (shown only when verbose > 0)
  call appendbufline(cmdbufnr, '$', 'E646_Test')
  set verbose=1
  call WaitForError('E646:')
  set verbose&
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:setModified!47 T', l[-1])
  let g:last += 3

  " Test for E647
  call appendbufline(cmdbufnr, '$', 'E647_Test')
  call WaitForError('E647:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:setDot!48 1/1', l[-1])
  let g:last += 3

  " Test for E648
  call appendbufline(cmdbufnr, '$', 'E648_Test')
  call WaitForError('E648:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:close!49', l[-1])
  let g:last += 3

  " Test for E650
  call appendbufline(cmdbufnr, '$', 'E650_Test')
  call WaitForError('E650:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:defineAnnoType!50 1 "abc" "a" "a" 1 1', l[-1])
  let g:last += 3

  " Test for E651
  call appendbufline(cmdbufnr, '$', 'E651_Test')
  call WaitForError('E651:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:addAnno!51 1 1 1 1', l[-1])
  let g:last += 3

  " Test for E652
  call appendbufline(cmdbufnr, '$', 'E652_Test')
  call WaitForError('E652:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:getAnno/52 8', '52 0'], l[-2:])
  let g:last += 4

  " editFile test
  call writefile(['foo bar1', 'foo bar2', 'foo bar3'], 'Xfile3', 'D')
  call appendbufline(cmdbufnr, '$', 'editFile_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal('send: 2:editFile!53 "Xfile3"', l[-2])
  call assert_match('0:fileOpened=0 ".*/Xfile3" T F', l[-1])
  call assert_equal('Xfile3', bufname())
  let g:last += 4

  " getLength test
  call appendbufline(cmdbufnr, '$', 'getLength_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 2:getLength/54', '54 27'], l[-2:])
  let g:last += 4

  " getModified test
  call appendbufline(cmdbufnr, '$', 'getModified_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 2:getModified/55', '55 0'], l[-2:])
  let g:last += 4

  " getText test
  call appendbufline(cmdbufnr, '$', 'getText_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 2:getText/56',
        \ '56 "foo bar1\nfoo bar2\nfoo bar3\n"'], l[-2:])
  let g:last += 4

  " setDot test with lnum/col
  call cursor(1, 1)
  call appendbufline(cmdbufnr, '$', 'setDot_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 2:setDot!57 3/6', l[-1])
  sleep 10m
  call assert_equal([0, 3, 7, 0], getpos('.'))
  let g:last += 3

  " setDot test with an offset
  call cursor(1, 1)
  call appendbufline(cmdbufnr, '$', 'setDot2_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 2:setDot!57 9', l[-1])
  sleep 10m
  call assert_equal([0, 2, 1, 0], getpos('.'))
  let g:last += 3

  " startDocumentListen test
  call appendbufline(cmdbufnr, '$', 'startDocumentListen_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 2:startDocumentListen!58', l[-1])
  let g:last += 3

  " make some changes to the buffer and check whether the netbeans server
  " received the notifications
  call append(2, 'blue sky')
  1d
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_match('2:insert=\d\+ 18 "blue sky"', l[-3])
  call assert_match('2:insert=\d\+ 26 "\\n"', l[-2])
  call assert_match('2:remove=\d\+ 0 9', l[-1])
  let g:last += 3

  " Change case using the ~ command with 'whichwrap' containing '~'
  set whichwrap+=~
  normal 2G$~
  set whichwrap&
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 2)')
  let l = ReadXnetbeans()
  call assert_match('2:remove=\d\+ 16 1', l[-4])
  call assert_match('2:insert=\d\+ 16 "Y"', l[-3])
  call assert_match('2:remove=\d\+ 18 0', l[-2])
  call assert_match('2:insert=\d\+ 18 ""', l[-1])
  let g:last += 4

  " Test for replacing spaces with a tab character using 'softtabstop' and
  " 'noexpandtab'
  setlocal softtabstop=4
  setlocal noexpandtab
  exe "normal I\<Tab>\<Tab>"
  setlocal expandtab&
  setlocal softtabstop&
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 18)')
  let l = ReadXnetbeans()
  call assert_match('2:insert=\d\+ 18 "        foo bar3"', l[-3])
  call assert_match('2:remove=\d\+ 26 8', l[-2])
  call assert_match('2:insert=\d\+ 26 "\t"', l[-1])
  let g:last += 18

  " Test for changing case of multiple lines using ~
  normal ggVG~
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 6)')
  let l = ReadXnetbeans()
  call assert_match('2:remove=\d\+ 0 8', l[-6])
  call assert_match('2:insert=\d\+ 0 "FOO BAR2"', l[-5])
  call assert_match('2:remove=\d\+ 9 8', l[-4])
  call assert_match('2:insert=\d\+ 9 "BLUE SKy"', l[-3])
  call assert_match('2:remove=\d\+ 18 9', l[-2])
  call assert_match('2:insert=\d\+ 18 "\tFOO BAR3"', l[-1])
  let g:last += 6

  " Test for changing case of a visual block using ~
  exe "normal ggw\<C-V>$~"
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 2)')
  let l = ReadXnetbeans()
  call assert_match('2:remove=\d\+ 4 4', l[-2])
  call assert_match('2:insert=\d\+ 4 "bar2"', l[-1])
  let g:last += 2

  " Increment a number using <C-A> in visual mode
  exe "normal! gg$v6\<C-A>"
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 6)')
  let l = ReadXnetbeans()
  call assert_match('2:remove=\d\+ 0 9', l[-4])
  call assert_match('2:insert=\d\+ 0 "FOO bar8"', l[-3])
  call assert_match('2:remove=\d\+ 7 1', l[-2])
  call assert_match('2:insert=\d\+ 7 "8"', l[-1])
  let g:last += 6

  " Decrement a number using <C-X> in visual mode
  exe "normal! gg$v3\<C-X>"
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 6)')
  let l = ReadXnetbeans()
  call assert_match('2:remove=\d\+ 0 9', l[-4])
  call assert_match('2:insert=\d\+ 0 "FOO bar5"', l[-3])
  call assert_match('2:remove=\d\+ 7 1', l[-2])
  call assert_match('2:insert=\d\+ 7 "5"', l[-1])
  let g:last += 6

  " stopDocumentListen test
  call appendbufline(cmdbufnr, '$', 'stopDocumentListen_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 2:stopDocumentListen!59', l[-1])
  let g:last += 3

  " Wait for vim to process the previous netbeans message
  sleep 1m

  " modify the buffer and make sure that the netbeans server is not notified
  call append(2, 'clear sky')
  1d

  " defineAnnoType test
  call appendbufline(cmdbufnr, '$', 'define_anno_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 2:defineAnnoType!60 1 "s1" "x" "=>" blue none', l[-1])
  sleep 1m
  call assert_equal({'name': '1', 'texthl': 'NB_s1', 'text': '=>'},
        \ sign_getdefined()->get(0, {}))
  let g:last += 3

  " defineAnnoType with a long color name
  call appendbufline(cmdbufnr, '$', 'E532_Test')
  call WaitForError('E532:')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 2:defineAnnoType!61 1 "s1" "x" "=>" aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa none', l[-1])
  let g:last += 3

  " addAnno test
  call appendbufline(cmdbufnr, '$', 'add_anno_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 2:addAnno!62 1 1 2/1 0', l[-1])
  sleep 1m
  call assert_equal([{'lnum': 2, 'id': 1, 'name': '1', 'priority': 10,
        \ 'group': ''}], sign_getplaced()[0].signs)
  let g:last += 3

  " getAnno test
  call appendbufline(cmdbufnr, '$', 'get_anno_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 2:getAnno/63 1', '63 2'], l[-2:])
  let g:last += 4

  " removeAnno test
  call appendbufline(cmdbufnr, '$', 'remove_anno_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 2:removeAnno!64 1', l[-1])
  sleep 1m
  call assert_equal([], sign_getplaced())
  let g:last += 3

  " getModified test to get the number of modified buffers
  call appendbufline(cmdbufnr, '$', 'getModifiedAll_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:getModified/65', '65 2'], l[-2:])
  let g:last += 4

  let bufcount = len(getbufinfo())

  " create test to create a new buffer
  call appendbufline(cmdbufnr, '$', 'create_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:create!66', l[-1])
  " Wait for vim to process the previous netbeans message
  sleep 10m
  call assert_equal(bufcount + 1, len(getbufinfo()))
  let g:last += 3

  " setTitle test
  call appendbufline(cmdbufnr, '$', 'setTitle_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:setTitle!67 "Xfile4"', l[-1])
  let g:last += 3

  " setFullName test
  call appendbufline(cmdbufnr, '$', 'setFullName_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 5)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:setFullName!68 "Xfile4"', l[-3])
  call assert_match('0:fileOpened=0 ".*/Xfile4" T F', l[-1])
  call assert_equal('Xfile4', bufname())
  let g:last += 5

  " initDone test
  call appendbufline(cmdbufnr, '$', 'initDone_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:initDone!69', l[-1])
  let g:last += 3

  " setVisible test
  hide enew
  call appendbufline(cmdbufnr, '$', 'setVisible_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:setVisible!70 T', l[-1])
  let g:last += 3

  " setModtime test
  call appendbufline(cmdbufnr, '$', 'setModtime_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:setModtime!71 6', l[-1])
  let g:last += 3

  " insert test
  call appendbufline(cmdbufnr, '$', 'insert_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 3:insert/72 0 "line1\nline2\n"', '72'], l[-2:])
  call assert_equal(['line1', 'line2'], getline(1, '$'))
  let g:last += 4

  " remove test
  call appendbufline(cmdbufnr, '$', 'remove_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 3:remove/73 3 4', '73'], l[-2:])
  call assert_equal(['linine2'], getline(1, '$'))
  let g:last += 4

  " remove with invalid offset
  call appendbufline(cmdbufnr, '$', 'remove_invalid_offset_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 3:remove/74 900 4', '74 !bad position'], l[-2:])
  let g:last += 4

  " remove with invalid count
  call appendbufline(cmdbufnr, '$', 'remove_invalid_count_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 3:remove/75 1 800', '75 !bad count'], l[-2:])
  let g:last += 4

  " guard test
  %d
  call setline(1, ['foo bar', 'foo bar', 'foo bar'])
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 8)')
  let g:last += 8

  call appendbufline(cmdbufnr, '$', 'guard_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:guard!76 8 7', l[-1])
  sleep 1m
  " second line is guarded. Try modifying the line
  call assert_fails('normal 2GIbaz', 'E463:')
  call assert_fails('normal 2GAbaz', 'E463:')
  call assert_fails('normal dd', 'E463:')
  call assert_equal([{'name': '1', 'texthl': 'NB_s1', 'text': '=>'},
        \ {'name': '10000', 'linehl': 'NBGuarded'}],
        \ sign_getdefined())
  let s = sign_getplaced()[0].signs[0]
  call assert_equal(2, s.lnum)
  call assert_equal('10000', s.name)
  let g:last += 3

  " setModified test
  call appendbufline(cmdbufnr, '$', 'setModified_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:setModified!77 T', l[-1])
  sleep 1m
  call assert_equal(1, &modified)
  let g:last += 3

  " clear setModified test
  call appendbufline(cmdbufnr, '$', 'setModifiedClear_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:setModified!77 F', l[-1])
  sleep 1m
  call assert_equal(0, &modified)
  let g:last += 3

  " insertDone test
  let v:statusmsg = ''
  call appendbufline(cmdbufnr, '$', 'insertDone_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:insertDone!78 T F', l[-1])
  sleep 1m
  call assert_match('.*/Xfile4" 3L, 0B', v:statusmsg)
  let g:last += 3

  " saveDone test
  let v:statusmsg = ''
  call appendbufline(cmdbufnr, '$', 'saveDone_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:saveDone!79', l[-1])
  sleep 1m
  call assert_match('.*/Xfile4" 3L, 0B', v:statusmsg)
  let g:last += 3

  " unimplemented command test
  call appendbufline(cmdbufnr, '$', 'invalidcmd_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:invalidcmd!80', l[-1])
  let g:last += 3

  " unimplemented function test
  call appendbufline(cmdbufnr, '$', 'invalidfunc_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 3:invalidfunc/81', '81'], l[-2:])
  let g:last += 4

  " Test for removeAnno cmd failure
  call appendbufline(cmdbufnr, '$', 'removeAnno_fail_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:removeAnno/82 1', '82'], l[-2:])
  let g:last += 4

  " Test for guard cmd failure
  call appendbufline(cmdbufnr, '$', 'guard_fail_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:guard/83 1 1', '83'], l[-2:])
  let g:last += 4

  " Test for save cmd failure
  call appendbufline(cmdbufnr, '$', 'save_fail_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:save/84', '84'], l[-2:])
  let g:last += 4

  " Test for netbeansBuffer cmd failure
  call appendbufline(cmdbufnr, '$', 'netbeansBuffer_fail_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal(['send: 0:netbeansBuffer/85 T', '85'], l[-2:])
  let g:last += 4

  " nbkey test
  call cursor(3, 3)
  nbkey "\<C-F2>"
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal(['3:newDotAndMark=85 18 18',
        \ '3:keyCommand=85 ""\<C-F2>""',
        \ '3:keyAtPos=85 ""\<C-F2>"" 18 3/2'], l[-3:])
  let g:last += 3

  " setExitDelay test
  call appendbufline(cmdbufnr, '$', 'setExitDelay_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:setExitDelay!86 2', l[-1])
  let g:last += 3

  " setReadonly test
  call appendbufline(cmdbufnr, '$', 'setReadOnly_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:setReadOnly!87 T', l[-1])
  sleep 1m
  call assert_equal(1, &readonly)
  let g:last += 3

  " clear setReadonly test
  call appendbufline(cmdbufnr, '$', 'setReadOnlyClear_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:setReadOnly!88 F', l[-1])
  sleep 1m
  call assert_equal(0, &readonly)
  let g:last += 3

  " save test
  call setbufvar(bufnr('Xfile4'), '&modified', 1)
  call appendbufline(cmdbufnr, '$', 'save_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:save!89', l[-1])
  sleep 1m
  call assert_true(filereadable('Xfile4'))
  let g:last += 3

  " close test. Don't use buffer 10 after this
  call appendbufline(cmdbufnr, '$', 'close_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 4)')
  let l = ReadXnetbeans()
  call assert_equal('send: 3:close!90', l[-2])
  call assert_equal('3:killed=90', l[-1])
  call assert_equal(1, winnr('$'))
  let g:last += 4

  " specialKeys test
  call appendbufline(cmdbufnr, '$', 'specialKeys_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:specialKeys!91 "F12 F13 C-F13"', l[-1])
  sleep 1m
  call assert_equal(':nbkey F12<CR>', maparg('<F12>', 'n'))
  call assert_equal(':nbkey F13<CR>', maparg('<F13>', 'n'))
  call assert_equal(':nbkey C-F13<CR>', maparg('<C-F13>', 'n'))
  let g:last += 3

  " Open a buffer not monitored by netbeans
  enew | only!
  nbkey "\<C-F3>"
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 1)')
  let l = ReadXnetbeans()
  call assert_equal('0:fileOpened=0 "" T F', l[-1])
  let g:last += 1

  " Test for writing a netbeans buffer
  call appendbufline(cmdbufnr, '$', 'nbbufwrite_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 5)')
  call assert_fails('write', 'E656:')
  call setline(1, ['one', 'two'])
  call assert_fails('1write!', 'E657:')
  write
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 10)')
  let g:last += 10

  if has('mouse')
    " Test for mouse button release
    let save_mouse = &mouse
    set mouse=a
    call feedkeys("\<LeftMouse>\<LeftRelease>", 'xt')
    let &mouse = save_mouse
    call WaitFor('len(ReadXnetbeans()) >= (g:last + 2)')
    let l = ReadXnetbeans()
    call assert_equal('4:newDotAndMark=93 0 0', l[-2])
    call assert_equal('4:buttonRelease=93 0 1 -1', l[-1])
    let g:last += 2
  endif

  " Test for startAtomic
  call appendbufline(cmdbufnr, '$', 'startAtomic_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:startAtomic!94', l[-1])
  let g:last += 3

  " Test for endAtomic
  call appendbufline(cmdbufnr, '$', 'endAtomic_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_equal('send: 0:endAtomic!95', l[-1])
  let g:last += 3

  " Test for invoking a netbeans key binding
  let special_keys = [
        \ ["\<F1>", 'F1'], ["\<S-F1>", 'S-F1'],
        \ ["\<F2>", 'F2'], ["\<S-F2>", 'S-F2'],
        \ ["\<F3>", 'F3'], ["\<S-F3>", 'S-F3'],
        \ ["\<F4>", 'F4'], ["\<S-F4>", 'S-F4'],
        \ ["\<F5>", 'F5'], ["\<S-F5>", 'S-F5'],
        \ ["\<F6>", 'F6'], ["\<S-F6>", 'S-F6'],
        \ ["\<F7>", 'F7'], ["\<S-F7>", 'S-F7'],
        \ ["\<F8>", 'F8'], ["\<S-F8>", 'S-F8'],
        \ ["\<F9>", 'F9'], ["\<S-F9>", 'S-F9'],
        \ ["\<F11>", 'F11'], ["\<S-F11>", 'S-F11'],
        \ ["\<F12>", 'F12'], ["\<S-F12>", 'S-F12'], ['!', '!']
        \ ]
  for [key, name] in special_keys
    call feedkeys("\<F21>" .. key, 'xt')
    call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
    let l = ReadXnetbeans()
    call assert_match('4:keyCommand=\d\+ "' .. name .. '"', l[-2])
    call assert_match('4:keyAtPos=\d\+ "' .. name .. '" 0 1/0', l[-1])
    let g:last += 3
  endfor
  call feedkeys("\<F21>\<C-S-M-F9>", 'xt')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 3)')
  let l = ReadXnetbeans()
  call assert_match('4:keyCommand=\d\+ "CSM-F9"', l[-2])
  call assert_match('4:keyAtPos=\d\+ "CSM-F9" 0 1/0', l[-1])
  let g:last += 3

  if has('signs') && has('mouse')
    sign define S1 linehl=Search text==>
    sign define S2 linehl=ErrorMsg text=!!
    sign place 10 line=1 name=S1
    sign place 20 line=1 name=S2

    let save_mouse = &mouse
    set mouse=a
    call assert_equal('S2', sign_getplaced()[0].signs[0].name)
    call test_setmouse(1, 1)
    call feedkeys("\<LeftMouse>\<LeftRelease>", 'xt')
    call assert_equal('S1', sign_getplaced()[0].signs[0].name)
    call test_setmouse(1, 1)
    call feedkeys("\<LeftMouse>\<LeftRelease>", 'xt')
    call assert_equal('S2', sign_getplaced()[0].signs[0].name)
    let &mouse = save_mouse

    sign unplace 10
    sign unplace 20
    sign undefine S1
    sign undefine S2
  endif

  " define a large number of annotations
  call appendbufline(cmdbufnr, '$', 'AnnoScale_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 26)')
  let l = ReadXnetbeans()
  call assert_equal('2:defineAnnoType!60 25 "s25" "x" "=>" blue none', l[-1])
  sleep 1m
  call assert_true(len(sign_getdefined()) >= 25)
  let g:last += 26

  " detach
  call appendbufline(cmdbufnr, '$', 'detach_Test')
  call WaitFor('len(ReadXnetbeans()) >= (g:last + 8)')
  call WaitForAssert({-> assert_equal('0:disconnect=97', ReadXnetbeans()[-1])})

  " the connection was closed
  call assert_false(has("netbeans_enabled"))

  " Remove all the signs
  call sign_unplace('*')
  call sign_undefine()

  call delete('Xfile4')
endfunc

func Test_nb_basic()
  call ch_log('Test_nb_basic')
  call s:run_server('Nb_basic')
endfunc

func Nb_file_auth(port)
  call delete("Xnetbeans")
  call writefile([], "Xnetbeans", 'D')

  call assert_fails('nbstart =notexist', 'E660:')
  call writefile(['host=localhost', 'port=' . a:port, 'auth=bunny'], 'Xnbauth', 'D')
  if has('unix')
    call setfperm('Xnbauth', "rw-r--r--")
    call assert_fails('nbstart =Xnbauth', 'E668:')
  endif
  call setfperm('Xnbauth', "rw-------")
  exe 'nbstart =Xnbauth'
  call assert_true(has("netbeans_enabled"))

  call WaitFor('len(ReadXnetbeans()) > 2')
  nbclose
  let lines = ReadXnetbeans()
  call assert_equal('AUTH bunny', lines[0])
  call assert_equal('0:version=0 "2.5"', lines[1])
  call assert_equal('0:startupDone=0', lines[2])

  call delete("Xnbauth")
endfunc

func Test_nb_file_auth()
  call ch_log('Test_nb_file_auth')
  call s:run_server('Nb_file_auth')
endfunc

" Test for quitting Vim with an open netbeans connection
func Nb_quit_with_conn(port)
  call delete("Xnetbeans")
  call writefile([], "Xnetbeans", 'D')
  let after =<< trim END
    source shared.vim
    set cpo&vim

    func ReadXnetbeans()
      let l = readfile("Xnetbeans")
      return filter(l, 'v:val !~ "^0:geometry="')
    endfunc

    try
      " Establish the connection with the netbeans server
      exe 'nbstart :localhost:' .. g:port .. ':star'
      call assert_true(has("netbeans_enabled"))
      call WaitFor('len(ReadXnetbeans()) >= 3')
      let l = ReadXnetbeans()
      call assert_equal(['AUTH star',
        \ '0:version=0 "2.5"',
        \ '0:startupDone=0'], l[-3:])

      " Open the command buffer to communicate with the server
      split Xcmdbuf
      call WaitFor('len(ReadXnetbeans()) >= 6')
      let l = ReadXnetbeans()
      call assert_equal('0:fileOpened=0 "Xcmdbuf" T F',
            \ substitute(l[-3], '".*/', '"', ''))
      call assert_equal('send: 1:putBufferNumber!15 "Xcmdbuf"',
            \ substitute(l[-2], '".*/', '"', ''))
      call assert_equal('1:startDocumentListen!16', l[-1])
      sleep 1m

      quit!
      quit!
    finally
      qall!
    endtry
  END
  if RunVim(['let g:port = ' .. a:port], after, '')
    call WaitFor('len(ReadXnetbeans()) >= 9')
    let l = ReadXnetbeans()
    call assert_equal('1:unmodified=16', l[-3])
    call assert_equal('1:killed=16', l[-2])
    call assert_equal('0:disconnect=16', l[-1])
  endif
endfunc

func Test_nb_quit_with_conn()
  " Exiting Vim with a netbeans connection doesn't work properly on
  " MS-Windows.
  CheckUnix
  call s:run_server('Nb_quit_with_conn')
endfunc

func Nb_bwipe_buffer(port)
  call delete("Xnetbeans")
  call writefile([], "Xnetbeans")

  " Last line number in the Xnetbeans file. Used to verify the result of the
  " communication with the netbeans server
  let g:last = 0

  " Establish the connection with the netbeans server
  exe 'nbstart :localhost:' .. a:port .. ':bunny'
  call WaitFor('len(ReadXnetbeans()) > (g:last + 2)')
  let l = ReadXnetbeans()
  call assert_equal(['AUTH bunny',
        \ '0:version=0 "2.5"',
        \ '0:startupDone=0'], l[-3:])
  let g:last += 3

  " Open the command buffer to communicate with the server
  split Xcmdbuf
  call WaitFor('len(ReadXnetbeans()) > (g:last + 2)')
  let l = ReadXnetbeans()
  call assert_equal('0:fileOpened=0 "Xcmdbuf" T F',
        \ substitute(l[-3], '".*/', '"', ''))
  call assert_equal('send: 1:putBufferNumber!15 "Xcmdbuf"',
        \ substitute(l[-2], '".*/', '"', ''))
  call assert_equal('1:startDocumentListen!16', l[-1])
  let g:last += 3

  sleep 10m
endfunc

" This test used to reference a buffer after it was freed leading to an ASAN
" error.
func Test_nb_bwipe_buffer()
  call s:run_server('Nb_bwipe_buffer')
  silent! %bwipe!
  sleep 100m
  nbclose
endfunc

" vim: shiftwidth=2 sts=2 expandtab
