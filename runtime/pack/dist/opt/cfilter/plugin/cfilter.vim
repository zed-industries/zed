vim9script

# cfilter.vim: Plugin to filter entries from a quickfix/location list
# Last Change: August 16, 2023
# Maintainer: Yegappan Lakshmanan (yegappan AT yahoo DOT com)
# Version: 2.0
#
# Commands to filter the quickfix list:
#   :Cfilter[!] /{pat}/
#       Create a new quickfix list from entries matching {pat} in the current
#       quickfix list. Both the file name and the text of the entries are
#       matched against {pat}. If ! is supplied, then entries not matching
#       {pat} are used. The pattern can be optionally enclosed using one of
#       the following characters: ', ", /. If the pattern is empty, then the
#       last used search pattern is used.
#   :Lfilter[!] /{pat}/
#       Same as :Cfilter but operates on the current location list.
#

def Qf_filter(qf: bool, searchpat: string, bang: string)
  var Xgetlist: func
  var Xsetlist: func
  var cmd: string
  var firstchar: string
  var lastchar: string
  var pat: string
  var title: string
  var Cond: func
  var items: list<any>

  if qf
    Xgetlist = function('getqflist')
    Xsetlist = function('setqflist')
    cmd = $':Cfilter{bang}'
  else
    Xgetlist = function('getloclist', [0])
    Xsetlist = function('setloclist', [0])
    cmd = $':Lfilter{bang}'
  endif

  firstchar = searchpat[0]
  lastchar = searchpat[-1 :]
  if firstchar == lastchar &&
              (firstchar == '/' || firstchar == '"' || firstchar == "'")
    pat = searchpat[1 : -2]
    if pat == ''
      # Use the last search pattern
      pat = @/
    endif
  else
    pat = searchpat
  endif

  if pat == ''
    return
  endif

  if bang == '!'
    Cond = (_, val) => val.text !~# pat && bufname(val.bufnr) !~# pat
  else
    Cond = (_, val) => val.text =~# pat || bufname(val.bufnr) =~# pat
  endif

  items = filter(Xgetlist(), Cond)
  title = $'{cmd} /{pat}/'
  Xsetlist([], ' ', {title: title, items: items})
enddef

command! -nargs=+ -bang Cfilter Qf_filter(true, <q-args>, <q-bang>)
command! -nargs=+ -bang Lfilter Qf_filter(false, <q-args>, <q-bang>)

# vim: shiftwidth=2 sts=2 expandtab
