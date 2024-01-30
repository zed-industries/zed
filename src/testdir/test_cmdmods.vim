" Test for all command modifiers in

def Test_cmdmods_array()
  # Get all the command modifiers from ex_cmds.h.
  var lines = readfile('../ex_cmds.h')->filter((_, l) => l =~ 'ex_wrongmodifier,')
  var cmds = lines->map((_, v) => substitute(v, '.*"\(\k*\)".*', '\1', ''))

  # :hide is both a command and a modifier
  cmds->extend(['hide'])

  # Get the entries of cmdmods[] in ex_docmd.c
  edit ../ex_docmd.c
  var top = search('^} cmdmods[') + 1
  var bot = search('^};') - 1
  lines = getline(top, bot)
  var mods = lines->map((_, v) => substitute(v, '.*"\(\k*\)".*', '\1', ''))

  # Add the other commands that use ex_wrongmodifier.
  mods->extend([
                'endclass',
                'endenum',
                'endinterface',
                'public',
                'static',
                'this',
              ])

  # Check the lists are equal.  Convert them to a dict to get a clearer error
  # message.
  var cmds_dict = {}
  for v in cmds
    cmds_dict[v] = 1
  endfor
  var mods_dict = {}
  for v in mods
    mods_dict[v] = 1
  endfor
  assert_equal(cmds_dict, mods_dict)

  bwipe!
enddef


" vim: shiftwidth=2 sts=2 expandtab

