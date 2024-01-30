" Tests for user defined commands

import './vim9.vim' as v9

source check.vim
source screendump.vim

" Test for <mods> in user defined commands
function Test_cmdmods()
  let g:mods = ''

  command! -nargs=* MyCmd let g:mods = '<mods>'

  MyCmd
  call assert_equal('', g:mods)
  aboveleft MyCmd
  call assert_equal('aboveleft', g:mods)
  abo MyCmd
  call assert_equal('aboveleft', g:mods)
  belowright MyCmd
  call assert_equal('belowright', g:mods)
  bel MyCmd
  call assert_equal('belowright', g:mods)
  botright MyCmd
  call assert_equal('botright', g:mods)
  bo MyCmd
  call assert_equal('botright', g:mods)
  browse MyCmd
  call assert_equal('browse', g:mods)
  bro MyCmd
  call assert_equal('browse', g:mods)
  confirm MyCmd
  call assert_equal('confirm', g:mods)
  conf MyCmd
  call assert_equal('confirm', g:mods)
  hide MyCmd
  call assert_equal('hide', g:mods)
  hid MyCmd
  call assert_equal('hide', g:mods)
  keepalt MyCmd
  call assert_equal('keepalt', g:mods)
  keepa MyCmd
  call assert_equal('keepalt', g:mods)
  keepjumps MyCmd
  call assert_equal('keepjumps', g:mods)
  keepj MyCmd
  call assert_equal('keepjumps', g:mods)
  keepmarks MyCmd
  call assert_equal('keepmarks', g:mods)
  kee MyCmd
  call assert_equal('keepmarks', g:mods)
  keeppatterns MyCmd
  call assert_equal('keeppatterns', g:mods)
  keepp MyCmd
  call assert_equal('keeppatterns', g:mods)
  leftabove MyCmd  " results in :aboveleft
  call assert_equal('aboveleft', g:mods)
  lefta MyCmd
  call assert_equal('aboveleft', g:mods)
  lockmarks MyCmd
  call assert_equal('lockmarks', g:mods)
  loc MyCmd
  call assert_equal('lockmarks', g:mods)
  noautocmd MyCmd
  call assert_equal('noautocmd', g:mods)
  noa MyCmd
  call assert_equal('noautocmd', g:mods)
  noswapfile MyCmd
  call assert_equal('noswapfile', g:mods)
  nos MyCmd
  call assert_equal('noswapfile', g:mods)
  rightbelow MyCmd " results in :belowright
  call assert_equal('belowright', g:mods)
  rightb MyCmd
  call assert_equal('belowright', g:mods)
  " sandbox MyCmd
  silent MyCmd
  call assert_equal('silent', g:mods)
  sil MyCmd
  call assert_equal('silent', g:mods)
  silent! MyCmd
  call assert_equal('silent!', g:mods)
  sil! MyCmd
  call assert_equal('silent!', g:mods)
  tab MyCmd
  call assert_equal('tab', g:mods)
  0tab MyCmd
  call assert_equal('0tab', g:mods)
  tab split
  tab MyCmd
  call assert_equal('tab', g:mods)
  1tab MyCmd
  call assert_equal('1tab', g:mods)
  tabprev
  tab MyCmd
  call assert_equal('tab', g:mods)
  2tab MyCmd
  call assert_equal('2tab', g:mods)
  2tabclose
  topleft MyCmd
  call assert_equal('topleft', g:mods)
  to MyCmd
  call assert_equal('topleft', g:mods)
  unsilent MyCmd
  call assert_equal('unsilent', g:mods)
  uns MyCmd
  call assert_equal('unsilent', g:mods)
  verbose MyCmd
  call assert_equal('verbose', g:mods)
  verb MyCmd
  call assert_equal('verbose', g:mods)
  0verbose MyCmd
  call assert_equal('0verbose', g:mods)
  3verbose MyCmd
  call assert_equal('3verbose', g:mods)
  999verbose MyCmd
  call assert_equal('999verbose', g:mods)
  vertical MyCmd
  call assert_equal('vertical', g:mods)
  vert MyCmd
  call assert_equal('vertical', g:mods)
  horizontal MyCmd
  call assert_equal('horizontal', g:mods)
  hor MyCmd
  call assert_equal('horizontal', g:mods)

  aboveleft belowright botright browse confirm hide keepalt keepjumps
	      \ keepmarks keeppatterns lockmarks noautocmd noswapfile silent
	      \ tab topleft unsilent verbose vertical MyCmd

  call assert_equal('browse confirm hide keepalt keepjumps ' .
      \ 'keepmarks keeppatterns lockmarks noswapfile unsilent noautocmd ' .
      \ 'silent verbose aboveleft belowright botright tab topleft vertical',
      \ g:mods)

  let g:mods = ''
  command! -nargs=* MyQCmd let g:mods .= '<q-mods> '

  vertical MyQCmd
  call assert_equal('"vertical" ', g:mods)

  delcommand MyCmd
  delcommand MyQCmd
  unlet g:mods
endfunction

func SaveCmdArgs(...)
  let g:args = a:000
endfunc

func Test_f_args()
  command -nargs=* TestFArgs call SaveCmdArgs(<f-args>)

  TestFArgs
  call assert_equal([], g:args)

  TestFArgs one two three
  call assert_equal(['one', 'two', 'three'], g:args)

  TestFArgs one\\two three
  call assert_equal(['one\two', 'three'], g:args)

  TestFArgs one\ two three
  call assert_equal(['one two', 'three'], g:args)

  TestFArgs one\"two three
  call assert_equal(['one\"two', 'three'], g:args)

  delcommand TestFArgs
endfunc

func Test_q_args()
  command -nargs=* TestQArgs call SaveCmdArgs(<q-args>)

  TestQArgs
  call assert_equal([''], g:args)

  TestQArgs one two three
  call assert_equal(['one two three'], g:args)

  TestQArgs one\\two three
  call assert_equal(['one\\two three'], g:args)

  TestQArgs one\ two three
  call assert_equal(['one\ two three'], g:args)

  TestQArgs one\"two three
  call assert_equal(['one\"two three'], g:args)

  delcommand TestQArgs
endfunc

func Test_reg_arg()
  command -nargs=* -reg TestRegArg call SaveCmdArgs("<reg>", "<register>")

  TestRegArg
  call assert_equal(['', ''], g:args)

  TestRegArg x
  call assert_equal(['x', 'x'], g:args)

  delcommand TestRegArg
endfunc

func Test_no_arg()
  command -nargs=* TestNoArg call SaveCmdArgs("<args>", "<>", "<x>", "<lt>")

  TestNoArg
  call assert_equal(['', '<>', '<x>', '<'], g:args)

  TestNoArg one
  call assert_equal(['one', '<>', '<x>', '<'], g:args)

  delcommand TestNoArg
endfunc

func Test_range_arg()
  command -range TestRangeArg call SaveCmdArgs(<range>, <line1>, <line2>)
  new
  call setline(1, range(100))
  let lnum = line('.')

  TestRangeArg
  call assert_equal([0, lnum, lnum], g:args)

  99TestRangeArg
  call assert_equal([1, 99, 99], g:args)

  88,99TestRangeArg
  call assert_equal([2, 88, 99], g:args)

  call assert_fails('102TestRangeArg', 'E16:')

  bwipe!
  delcommand TestRangeArg
endfunc

func Test_Ambiguous()
  command Doit let g:didit = 'yes'
  command Dothat let g:didthat = 'also'
  call assert_fails('Do', 'E464:')
  Doit
  call assert_equal('yes', g:didit)
  Dothat
  call assert_equal('also', g:didthat)
  unlet g:didit
  unlet g:didthat

  delcommand Doit
  Do
  call assert_equal('also', g:didthat)
  delcommand Dothat

  call assert_fails("\x4ei\041", ' you demand a ')
endfunc

func Test_redefine_on_reload()
  call writefile(['command ExistingCommand echo "yes"'], 'Xcommandexists', 'D')
  call assert_equal(0, exists(':ExistingCommand'))
  source Xcommandexists
  call assert_equal(2, exists(':ExistingCommand'))
  " Redefining a command when reloading a script is OK.
  source Xcommandexists
  call assert_equal(2, exists(':ExistingCommand'))

  " But redefining in another script is not OK.
  call writefile(['command ExistingCommand echo "yes"'], 'Xcommandexists2', 'D')
  call assert_fails('source Xcommandexists2', 'E174:')

  " And defining twice in one script is not OK.
  delcommand ExistingCommand
  call assert_equal(0, exists(':ExistingCommand'))
  call writefile([
	\ 'command ExistingCommand echo "yes"',
	\ 'command ExistingCommand echo "no"',
	\ ], 'Xcommandexists')
  call assert_fails('source Xcommandexists', 'E174:')
  call assert_equal(2, exists(':ExistingCommand'))

  delcommand ExistingCommand
endfunc

func Test_CmdUndefined()
  call assert_fails('Doit', 'E492:')
  au CmdUndefined Doit :command Doit let g:didit = 'yes'
  Doit
  call assert_equal('yes', g:didit)
  delcommand Doit

  call assert_fails('Dothat', 'E492:')
  au CmdUndefined * let g:didnot = 'yes'
  call assert_fails('Dothat', 'E492:')
  call assert_equal('yes', g:didnot)
endfunc

func Test_CmdErrors()
  call assert_fails('com! docmd :', 'E183:')
  call assert_fails('com! \<Tab> :', 'E182:')
  call assert_fails('com! _ :', 'E182:')
  call assert_fails('com! X :', 'E841:')
  call assert_fails('com! - DoCmd :', 'E175:')
  call assert_fails('com! -xxx DoCmd :', 'E181:')
  call assert_fails('com! -addr DoCmd :', 'E179:')
  call assert_fails('com! -addr=asdf DoCmd :', 'E180:')
  call assert_fails('com! -complete DoCmd :', 'E179:')
  call assert_fails('com! -complete=xxx DoCmd :', 'E180:')
  call assert_fails('com! -complete=custom DoCmd :', 'E467:')
  call assert_fails('com! -complete=customlist DoCmd :', 'E467:')
  call assert_fails('com! -complete=behave,CustomComplete DoCmd :', 'E468:')
  call assert_fails('com! -nargs=x DoCmd :', 'E176:')
  call assert_fails('com! -count=1 -count=2 DoCmd :', 'E177:')
  call assert_fails('com! -count=x DoCmd :', 'E178:')
  call assert_fails('com! -range=x DoCmd :', 'E178:')

  com! -complete=file DoCmd :
  call assert_match('E1208:', v:warningmsg)
  let v:warningmsg = ''
  com! -nargs=0 -complete=file DoCmd :
  call assert_match('E1208:', v:warningmsg)

  let lines =<< trim END
      vim9script
      com! -complete=file DoCmd :
  END
  call v9.CheckScriptFailure(lines, 'E1208', 2)

  let lines =<< trim END
      vim9script
      com! -nargs=0 -complete=file DoCmd :
  END
  call v9.CheckScriptFailure(lines, 'E1208', 2)

  com! -nargs=0 DoCmd :
  call assert_fails('DoCmd x', 'E488:')

  com! -nargs=1 DoCmd :
  call assert_fails('DoCmd', 'E471:')

  com! -nargs=+ DoCmd :
  call assert_fails('DoCmd', 'E471:')

  call assert_fails('com DoCmd :', 'E174:')
  comclear
  call assert_fails('delcom DoCmd', 'E184:')

  " These used to leak memory
  call assert_fails('com! -complete=custom,CustomComplete _ :', 'E182:')
  call assert_fails('com! -complete=custom,CustomComplete docmd :', 'E183:')
  call assert_fails('com! -complete=custom,CustomComplete -xxx DoCmd :', 'E181:')
endfunc

func CustomComplete(A, L, P)
  return "January\nFebruary\nMars\n"
endfunc

func CustomCompleteList(A, L, P)
  return [ "Monday", "Tuesday", "Wednesday", {}, test_null_string()]
endfunc

func Test_CmdCompletion()
  call feedkeys(":com -\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"com -addr bang bar buffer complete count keepscript nargs range register', @:)

  call feedkeys(":com -nargs=0 -\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"com -nargs=0 -addr bang bar buffer complete count keepscript nargs range register', @:)

  call feedkeys(":com -nargs=\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"com -nargs=* + 0 1 ?', @:)

  call feedkeys(":com -addr=\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"com -addr=arguments buffers lines loaded_buffers other quickfix tabs windows', @:)

  call feedkeys(":com -complete=co\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"com -complete=color command compiler', @:)

  " try completion for unsupported argument values
  call feedkeys(":com -newarg=\<Tab>\<C-B>\"\<CR>", 'tx')
  call assert_equal("\"com -newarg=\t", @:)

  " command completion after the name in a user defined command
  call feedkeys(":com MyCmd chist\<Tab>\<C-B>\"\<CR>", 'tx')
  call assert_equal("\"com MyCmd chistory", @:)

  " delete the Check commands to avoid them showing up
  call feedkeys(":com Check\<C-A>\<C-B>\"\<CR>", 'tx')
  let cmds = substitute(@:, '"com ', '', '')->split()
  for cmd in cmds
    exe 'delcommand ' .. cmd
  endfor
  delcommand MissingFeature

  command! DoCmd1 :
  command! DoCmd2 :
  call feedkeys(":com \<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"com DoCmd1 DoCmd2', @:)

  call feedkeys(":DoC\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"DoCmd1 DoCmd2', @:)

  call feedkeys(":delcom DoC\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"delcom DoCmd1 DoCmd2', @:)

  " try argument completion for a command without completion
  call feedkeys(":DoCmd1 \<Tab>\<C-B>\"\<CR>", 'tx')
  call assert_equal("\"DoCmd1 \t", @:)

  delcom DoCmd1
  call feedkeys(":delcom DoC\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"delcom DoCmd2', @:)

  call feedkeys(":com DoC\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"com DoCmd2', @:)

  delcom DoCmd2
  call feedkeys(":delcom DoC\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"delcom DoC', @:)

  call feedkeys(":com DoC\<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"com DoC', @:)

  com! -nargs=1 -complete=behave DoCmd :
  call feedkeys(":DoCmd \<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"DoCmd mswin xterm', @:)

  " Test for file name completion
  com! -nargs=1 -complete=file DoCmd :
  call feedkeys(":DoCmd READM\<Tab>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"DoCmd README.txt', @:)

  " Test for buffer name completion
  com! -nargs=1 -complete=buffer DoCmd :
  let bnum = bufadd('BufForUserCmd')
  call setbufvar(bnum, '&buflisted', 1)
  call feedkeys(":DoCmd BufFor\<Tab>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"DoCmd BufForUserCmd', @:)
  bwipe BufForUserCmd
  call feedkeys(":DoCmd BufFor\<Tab>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"DoCmd BufFor', @:)

  com! -nargs=* -complete=custom,CustomComplete DoCmd :
  call feedkeys(":DoCmd \<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"DoCmd January February Mars', @:)

  com! -nargs=? -complete=customlist,CustomCompleteList DoCmd :
  call feedkeys(":DoCmd \<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"DoCmd Monday Tuesday Wednesday', @:)

  com! -nargs=+ -complete=custom,CustomCompleteList DoCmd :
  call assert_fails("call feedkeys(':DoCmd \<C-D>', 'tx')", 'E730:')

  com! -nargs=+ -complete=customlist,CustomComp DoCmd :
  call assert_fails("call feedkeys(':DoCmd \<C-D>', 'tx')", 'E117:')

  " custom completion without a function
  com! -nargs=? -complete=custom, DoCmd
  call assert_beeps("call feedkeys(':DoCmd \t', 'tx')")

  " custom completion failure with the wrong function
  com! -nargs=? -complete=custom,min DoCmd
  call assert_fails("call feedkeys(':DoCmd \t', 'tx')", 'E118:')

  " custom completion for a pattern with a backslash
  let g:ArgLead = ''
  func! CustCompl(A, L, P)
    let g:ArgLead = a:A
    return ['one', 'two', 'three']
  endfunc
  com! -nargs=? -complete=customlist,CustCompl DoCmd
  call feedkeys(":DoCmd a\\\t", 'xt')
  call assert_equal('a\', g:ArgLead)
  delfunc CustCompl

  delcom DoCmd
endfunc

func CallExecute(A, L, P)
  " Drop first '\n'
  return execute('echo "hi"')[1:]
endfunc

func Test_use_execute_in_completion()
  command! -nargs=* -complete=custom,CallExecute DoExec :
  call feedkeys(":DoExec \<C-A>\<C-B>\"\<CR>", 'tx')
  call assert_equal('"DoExec hi', @:)
  delcommand DoExec
endfunc

func Test_addr_all()
  command! -addr=lines DoSomething let g:a1 = <line1> | let g:a2 = <line2>
  %DoSomething
  call assert_equal(1, g:a1)
  call assert_equal(line('$'), g:a2)

  command! -addr=arguments DoSomething let g:a1 = <line1> | let g:a2 = <line2>
  args one two three
  %DoSomething
  call assert_equal(1, g:a1)
  call assert_equal(3, g:a2)

  command! -addr=buffers DoSomething let g:a1 = <line1> | let g:a2 = <line2>
  %DoSomething
  for low in range(1, bufnr('$'))
    if buflisted(low)
      break
    endif
  endfor
  call assert_equal(low, g:a1)
  call assert_equal(bufnr('$'), g:a2)

  command! -addr=loaded_buffers DoSomething let g:a1 = <line1> | let g:a2 = <line2>
  %DoSomething
  for low in range(1, bufnr('$'))
    if bufloaded(low)
      break
    endif
  endfor
  call assert_equal(low, g:a1)
  for up in range(bufnr('$'), 1, -1)
    if bufloaded(up)
      break
    endif
  endfor
  call assert_equal(up, g:a2)

  command! -addr=windows DoSomething  let g:a1 = <line1> | let g:a2 = <line2>
  new
  %DoSomething
  call assert_equal(1, g:a1)
  call assert_equal(winnr('$'), g:a2)
  bwipe

  command! -addr=tabs DoSomething  let g:a1 = <line1> | let g:a2 = <line2>
  tabnew
  %DoSomething
  call assert_equal(1, g:a1)
  call assert_equal(len(gettabinfo()), g:a2)
  bwipe

  command! -addr=other DoSomething  let g:a1 = <line1> | let g:a2 = <line2>
  DoSomething
  call assert_equal(line('.'), g:a1)
  call assert_equal(line('.'), g:a2)
  %DoSomething
  call assert_equal(1, g:a1)
  call assert_equal(line('$'), g:a2)

  delcommand DoSomething
endfunc

func Test_command_list()
  command! DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0                        :",
        \           execute('command DoCmd'))

  " Test with various -range= and -count= argument values.
  command! -range DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    .                   :",
        \           execute('command DoCmd'))
  command! -range=% DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    %                   :",
        \           execute('command! DoCmd'))
  command! -range=2 DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    2                   :",
        \           execute('command DoCmd'))
  command! -count=2 DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    2c ?                :",
        \           execute('command DoCmd'))

  " Test with various -addr= argument values.
  command! -addr=lines DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    .                   :",
        \           execute('command DoCmd'))
  command! -addr=arguments DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    .  arg              :",
        \           execute('command DoCmd'))
  command! -addr=buffers DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    .  buf              :",
        \           execute('command DoCmd'))
  command! -addr=loaded_buffers DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    .  load             :",
        \           execute('command DoCmd'))
  command! -addr=windows DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    .  win              :",
        \           execute('command DoCmd'))
  command! -addr=tabs DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    .  tab              :",
        \           execute('command DoCmd'))
  command! -addr=other DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0    .  ?                :",
        \           execute('command DoCmd'))

  " Test with various -complete= argument values (non-exhaustive list)
  command! -nargs=1 -complete=arglist DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             1            arglist     :",
        \           execute('command DoCmd'))
  command! -nargs=* -complete=augroup DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             *            augroup     :",
        \           execute('command DoCmd'))
  command! -nargs=? -complete=custom,CustomComplete DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             ?            custom      :",
        \           execute('command DoCmd'))
  command! -nargs=+ -complete=customlist,CustomComplete DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             +            customlist  :",
        \           execute('command DoCmd'))

  " Test with various -narg= argument values.
  command! -nargs=0 DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0                        :",
        \           execute('command DoCmd'))
  command! -nargs=1 DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             1                        :",
        \           execute('command DoCmd'))
  command! -nargs=* DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             *                        :",
        \           execute('command DoCmd'))
  command! -nargs=? DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             ?                        :",
        \           execute('command DoCmd'))
  command! -nargs=+ DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             +                        :",
        \           execute('command DoCmd'))

  " Test with other arguments.
  command! -bang DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n!   DoCmd             0                        :",
        \           execute('command DoCmd'))
  command! -bar DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n|   DoCmd             0                        :",
        \           execute('command DoCmd'))
  command! -register DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n\"   DoCmd             0                        :",
        \           execute('command DoCmd'))
  command! -buffer DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\nb   DoCmd             0                        :"
        \        .. "\n\"   DoCmd             0                        :",
        \           execute('command DoCmd'))
  comclear

  " Test with many args.
  command! -bang -bar -register -buffer -nargs=+ -complete=environment -addr=windows -count=3 DoCmd :
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n!\"b|DoCmd             +    3c win  environment :",
        \           execute('command DoCmd'))
  comclear

  " Test with special characters in command definition.
  command! DoCmd :<cr><tab><c-d>
  call assert_equal("\n    Name              Args Address Complete    Definition"
        \        .. "\n    DoCmd             0                        :<CR><Tab><C-D>",
        \           execute('command DoCmd'))

  " Test output in verbose mode.
  command! -nargs=+ -complete=customlist,SomeFunc DoCmd :ls
  call assert_match("^\n"
        \        .. "    Name              Args Address Complete    Definition\n"
        \        .. "    DoCmd             +            customlist,SomeFunc :ls\n"
        \        .. "\tLast set from .*/test_usercommands.vim line \\d\\+$",
        \           execute('verbose command DoCmd'))

  comclear
  call assert_equal("\nNo user-defined commands found", execute(':command Xxx'))
  call assert_equal("\nNo user-defined commands found", execute('command'))
endfunc

" Test for a custom user completion returning the wrong value type
func Test_usercmd_custom()
  func T1(a, c, p)
    return "a\nb\n"
  endfunc
  command -nargs=* -complete=customlist,T1 TCmd1
  call assert_fails('call feedkeys(":TCmd1 \<C-A>\<C-B>\"\<CR>", "xt")', 'E1303: Custom list completion function does not return a List but a string')
  call assert_equal('"TCmd1 ', @:)
  delcommand TCmd1
  delfunc T1

  func T2(a, c, p)
    return {}
  endfunc
  command -nargs=* -complete=customlist,T2 TCmd2
  call assert_fails('call feedkeys(":TCmd2 \<C-A>\<C-B>\"\<CR>", "xt")', 'E1303: Custom list completion function does not return a List but a dict')
  call assert_equal('"TCmd2 ', @:)
  delcommand TCmd2
  delfunc T2
endfunc

func Test_usercmd_with_block()
  command DoSomething {
        g:didit = 'yes'  # comment
        # comment line
        g:didmore = 'more'
      }
  DoSomething
  call assert_equal('yes', g:didit)
  call assert_equal('more', g:didmore)
  unlet g:didit
  unlet g:didmore
  delcommand DoSomething

  command DoMap {
	echo [1, 2, 3]->map((_, v) => v + 1)
    }
  DoMap
  delcommand DoMap

  let lines =<< trim END
      command DoesNotEnd {
         echo 'hello'
  END
  call v9.CheckScriptFailure(lines, 'E1026:')
  delcommand DoesNotEnd

  let lines =<< trim END
      command HelloThere {
         echo 'hello' | echo 'there'
        }
      HelloThere
  END
  call v9.CheckScriptSuccess(lines)
  delcommand HelloThere

  let lines =<< trim END
      command EchoCond {
          const test: string = true
              ? 'true'
              : 'false'
          g:result = test
      }
      EchoCond
  END
  call v9.CheckScriptSuccess(lines)
  call assert_equal('true', g:result)
  unlet g:result

  call feedkeys(":EchoCond\<CR>", 'xt')
  call assert_equal('true', g:result)

  delcommand EchoCond
  unlet g:result

  let lines =<< trim END
      command BadCommand {
         echo  {
         'key': 'value',
          }
          }
      BadCommand
  END
  call v9.CheckScriptFailure(lines, 'E1128:')
  delcommand BadCommand

  let lines =<< trim END
	  vim9script
    command Cmd {
        g:result = [1,
        2]
    }
    Cmd
  END
  call v9.CheckScriptSuccess(lines)
  call assert_equal([1, 2], g:result)
  delcommand Cmd
	unlet! g:result

  let lines =<< trim END
		vim9script
		command Cmd {
			g:result = and(0x80,
			0x80)
    }
    Cmd
  END
  call v9.CheckScriptSuccess(lines)
  call assert_equal(128, g:result)
  delcommand Cmd
	unlet! g:result

endfunc

func Test_delcommand_buffer()
  command Global echo 'global'
  command -buffer OneBuffer echo 'one'
  new
  command -buffer TwoBuffer echo 'two'
  call assert_equal(0, exists(':OneBuffer'))
  call assert_equal(2, exists(':Global'))
  call assert_equal(2, exists(':TwoBuffer'))
  delcommand -buffer TwoBuffer
  call assert_equal(0, exists(':TwoBuffer'))
  call assert_fails('delcommand -buffer Global', 'E1237:')
  call assert_fails('delcommand -buffer OneBuffer', 'E1237:')
  bwipe!
  call assert_equal(2, exists(':OneBuffer'))
  delcommand -buffer OneBuffer
  call assert_equal(0, exists(':OneBuffer'))
  call assert_fails('delcommand -buffer Global', 'E1237:')
  delcommand Global
  call assert_equal(0, exists(':Global'))
endfunc

def Test_count_with_quotes()
  command -count GetCount g:nr = <count>
  execute("GetCount 1'2")
  assert_equal(12, g:nr)
  execute("GetCount 1'234'567")
  assert_equal(1'234'567, g:nr)

  execute("GetCount 1'234'567'890'123'456'789'012")
  assert_equal(v:sizeoflong == 8 ? 9223372036854775807 : 2147483647, g:nr)

  # TODO: test with negative number once this is supported

  assert_fails("GetCount '12", "E488:")
  assert_fails("GetCount 12'", "E488:")
  assert_fails("GetCount 1''2", "E488:")

  assert_fails(":1'2GetCount", 'E492:')
  new
  setline(1, 'text')
  normal ma
  execute(":1, 'aprint")
  bwipe!

  unlet g:nr
  delcommand GetCount
enddef

func DefCmd(name)
  if len(a:name) > 30
    return
  endif
  exe 'command ' .. a:name .. ' call DefCmd("' .. a:name .. 'x")'
  echo a:name
  exe a:name
endfunc

func Test_recursive_define()
  call DefCmd('Command')

  let name = 'Command'
  while len(name) <= 30
    exe 'delcommand ' .. name
    let name ..= 'x'
  endwhile
endfunc

" Test for using buffer-local ambiguous user-defined commands
func Test_buflocal_ambiguous_usercmd()
  new
  command -buffer -nargs=1 -complete=sign TestCmd1 echo "Hello"
  command -buffer -nargs=1 -complete=sign TestCmd2 echo "World"

  call assert_fails("call feedkeys(':TestCmd\<CR>', 'xt')", 'E464:')
  call feedkeys(":TestCmd \<Tab>\<C-B>\"\<CR>", 'xt')
  call assert_equal('"TestCmd ', @:)

  delcommand TestCmd1
  delcommand TestCmd2
  bw!
endfunc

" Test for using buffer-local user command from cmdwin.
func Test_buflocal_usercmd_cmdwin()
  new
  command -buffer TestCmd edit Test
  " This used to crash Vim
  call assert_fails("norm q::TestCmd\<CR>", 'E11:')
  bw!
endfunc

" Test for using a multibyte character in a user command
func Test_multibyte_in_usercmd()
  command SubJapanesePeriodToDot exe "%s/\u3002/./g"
  new
  call setline(1, "Hello\u3002")
  SubJapanesePeriodToDot
  call assert_equal('Hello.', getline(1))
  bw!
  delcommand SubJapanesePeriodToDot
endfunc

" Declaring a variable in a {} uses Vim9 script rules, even when defined in a
" legacy script.
func Test_block_declaration_legacy_script()
  let lines =<< trim END
      command -range Rename {
                     var save = @a
                     @a = 'something'
                     g:someExpr = @a
                     @a = save
                }
  END
  call writefile(lines, 'Xlegacy', 'D')
  source Xlegacy

  let lines =<< trim END
      let @a = 'saved'
      Rename
      call assert_equal('something', g:someExpr)
      call assert_equal('saved', @a)

      let g:someExpr = 'xxx'
      let @a = 'also'
      Rename
      call assert_equal('something', g:someExpr)
      call assert_equal('also', @a)
  END
  call writefile(lines, 'Xother', 'D')
  source Xother

  unlet g:someExpr
  delcommand Rename
endfunc

func Test_comclear_while_listing()
  call CheckRunVimInTerminal()

  let lines =<< trim END
      set nocompatible
      comclear
      for i in range(1, 999)
        exe 'command ' .. 'Foo' .. i .. ' bar'
      endfor
      au CmdlineLeave : call timer_start(0, {-> execute('comclear')})
  END
  call writefile(lines, 'Xcommandclear', 'D')
  let buf = RunVimInTerminal('-S Xcommandclear', {'rows': 10})

  " this was using freed memory
  call term_sendkeys(buf, ":command\<CR>")
  call TermWait(buf, 50)
  call term_sendkeys(buf, "j")
  call TermWait(buf, 50)
  call term_sendkeys(buf, "G")
  call term_sendkeys(buf, "\<CR>")

  call StopVimInTerminal(buf)
endfunc


" vim: shiftwidth=2 sts=2 expandtab
