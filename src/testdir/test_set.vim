" Tests for the :set command

source check.vim

function Test_set_backslash()
  let isk_save = &isk

  set isk=a,b,c
  set isk+=d
  call assert_equal('a,b,c,d', &isk)
  set isk+=\\,e
  call assert_equal('a,b,c,d,\,e', &isk)
  set isk-=e
  call assert_equal('a,b,c,d,\', &isk)
  set isk-=\\
  call assert_equal('a,b,c,d', &isk)

  let &isk = isk_save
endfunction

function Test_set_add()
  let wig_save = &wig

  set wildignore=*.png,
  set wildignore+=*.jpg
  call assert_equal('*.png,*.jpg', &wig)

  let &wig = wig_save
endfunction


" :set, :setlocal, :setglobal without arguments show values of options.
func Test_set_no_arg()
  set textwidth=79
  let a = execute('set')
  call assert_match("^\n--- Options ---\n.*textwidth=79\\>", a)
  set textwidth&

  setlocal textwidth=78
  let a = execute('setlocal')
  call assert_match("^\n--- Local option values ---\n.*textwidth=78\\>", a)
  setlocal textwidth&

  setglobal textwidth=77
  let a = execute('setglobal')
  call assert_match("^\n--- Global option values ---\n.*textwidth=77\\>", a)
  setglobal textwidth&
endfunc

func Test_set_termcap()
  CheckNotGui

  let lines = split(execute('set termcap'), "\n")
  call assert_match('--- Terminal codes ---', lines[0])
  " four columns
  call assert_match('t_..=.*t_..=.*t_..=.*t_..=', lines[1])

  for keys_idx in range(len(lines))
    if lines[keys_idx] =~ '--- Terminal keys ---'
      break
    endif
  endfor
  call assert_true(keys_idx < len(lines))
  " three columns
  call assert_match('<[^>]*> .*<[^>]*> .*<[^>]*> ', lines[keys_idx + 1])

  let more_lines = split(execute('set! termcap'), "\n")
  for i in range(len(more_lines))
    if more_lines[i] =~ '--- Terminal keys ---'
      break
    endif
  endfor
  call assert_true(i < len(more_lines))
  call assert_true(i > keys_idx)
  call assert_true(len(more_lines) - i > len(lines) - keys_idx)
endfunc

" vim: shiftwidth=2 sts=2 expandtab
