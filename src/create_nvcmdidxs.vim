" This script generates the table nv_cmd_idx[] which contains the index in
" nv_cmds[] table (normal.c) for each of the command character supported in
" normal/visual mode.
" This is used to speed up the command lookup in nv_cmds[].
"
" Script should be run using "make nvcmdidxs", every time the nv_cmds[] table
" in src/nv_cmds.h changes.
"
" This is written in legacy Vim script so that it can be run by a slightly
" older Vim version.

" Generate the table of normal/visual mode command characters and their
" corresponding index.
let cmd = 'create_nvcmdidxs'
if has('unix')
  let cmd = './' .. cmd
endif
let nv_cmdtbl = systemlist(cmd)->map({i, ch -> {'idx': i, 'cmdchar': ch}})

" sort the table by the command character
call sort(nv_cmdtbl, {a, b -> a.cmdchar - b.cmdchar})

" Compute the highest index upto which the command character can be directly
" used as an index.
let nv_max_linear = 0
for i in range(nv_cmdtbl->len())
  if i != nv_cmdtbl[i].cmdchar
    let nv_max_linear = i - 1
    break
  endif
endfor

" Generate a header file with the table
let output =<< trim END
  /*
   * Automatically generated code by the create_nvcmdidxs.vim script.
   *
   * Table giving the index in nv_cmds[] to lookup based on
   * the command character.
   */

  // nv_cmd_idx[<normal mode command character>] => nv_cmds[] index
  static const unsigned short nv_cmd_idx[] =
  {
END

" Add each command character in comment and the corresponding index
let output += nv_cmdtbl->map({_, v ->
      \ printf('  /* %5d */ %3d,', v.cmdchar, v.idx)})

let output += ['};', '',
      \ '// The highest index for which',
      \ '// nv_cmds[idx].cmd_char == nv_cmd_idx[nv_cmds[idx].cmd_char]']

let output += ['static const int nv_max_linear = ' .. nv_max_linear .. ';']

call writefile(output, "nv_cmdidxs.h")
quit

" vim: shiftwidth=2 sts=2 expandtab
