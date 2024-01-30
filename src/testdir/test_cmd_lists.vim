" Test to verify that the cmd list in runtime/doc/index.txt contains all of
" the commands in src/ex_cmds.h. It doesn't map the other way round because
" index.txt contains some shorthands like :!! which are useful to list, but
" they don't exist as an independent entry in src/ex_cmds.h.
"
" Currently this just checks for existence, and we aren't checking for whether
" they are sorted in the index, or whether the substring needed (e.g.
" 'defc[ompile]') is correct or not.

func Test_cmd_lists()

  " Create a list of the commands in ex_cmds.h:CMD_index.
  enew!
  read ../ex_cmds.h
  1,/^enum CMD_index$/d
  call search('^};$')
  .,$d
  v/^EXCMD/d
  %s/^.*"\(\S\+\)".*$/\1/
  " Special case ':*' because it's represented as ':star'
  %s/^\*$/star/
  sort u
  let l:command_list = getline(1, '$')

  " Verify that the ':help ex-cmd-index' list contains all known commands.
  enew!
  if filereadable('../../doc/index.txt')
    " unpacked MS-Windows zip archive
    read ../../doc/index.txt
  else
    read ../../runtime/doc/index.txt
  endif
  call search('\*ex-cmd-index\*')
  1,.d
  v/^|:/d
  %s/^|:\(\S*\)|.*/\1/
  sort u
  norm gg
  let l:missing_cmds = []
  for cmd in l:command_list
    " Reserved Vim 9 commands or other script-only syntax aren't useful to
    " document as Ex commands.
    let l:vim9cmds = [
          \ 'abstract',
          \ 'class',
          \ 'endclass',
          \ 'endenum',
          \ 'endinterface',
          \ 'enum',
          \ 'interface',
          \ 'public',
          \ 'static',
          \ 'this',
          \ 'type',
          \ '++',
          \ '--',
          \ '{',
          \ '}']
    if index(l:vim9cmds, cmd) != -1
      continue
    endif

    if search('^\V' .. cmd .. '\v$', 'cW') == 0
      call add(l:missing_cmds, ':' .. cmd)
    endif
  endfor
  call assert_equal(0, len(l:missing_cmds), "Missing commands from `:help ex-cmd-index`: " .. string(l:missing_cmds))
endfunc

" vim: shiftwidth=2 sts=2 expandtab
