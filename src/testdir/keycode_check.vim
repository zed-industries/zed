vim9script

# Script to get various codes that keys send, depending on the protocol used.
#
# Usage:  vim -u NONE -S keycode_check.vim
#
# Author:	Bram Moolenaar
# Last Update:	2022 Nov 15
#
# The codes are stored in the file "keycode_check.json", so that you can
# compare the results of various terminals.
#
# You can select what protocol to enable:
# - None
# - modifyOtherKeys level 2
# - kitty keyboard protocol

# Change directory to where this script is, so that the json file is found
# there.
exe 'cd ' .. expand('<sfile>:h')
echo 'working in directory: ' .. getcwd()

const filename = 'keycode_check.json'

# Dictionary of dictionaries with the results in the form:
# {'xterm': {protocol: 'none', 'Tab': '09', 'S-Tab': '09'},
#  'xterm2': {protocol: 'mok2', 'Tab': '09', 'S-Tab': '09'},
#  'kitty': {protocol: 'kitty', 'Tab': '09', 'S-Tab': '09'},
# }
# The values are in hex form.
var keycodes = {}

if filereadable(filename)
  keycodes = readfile(filename)->join()->json_decode()
else
  # Use some dummy entries to try out with
  keycodes = {
    'xterm': {protocol: 'none', 'Tab': '09', 'S-Tab': '09'},
    'kitty': {protocol: 'kitty', 'Tab': '09', 'S-Tab': '1b5b393b3275'},
    }
endif
var orig_keycodes = deepcopy(keycodes)  # used to detect something changed

# Write the "keycodes" variable in JSON form to "filename".
def WriteKeycodes()
  # If the file already exists move it to become the backup file.
  if filereadable(filename)
    if rename(filename, filename .. '~')
      echoerr $'Renaming {filename} to {filename}~ failed!'
      return
    endif
  endif

  if writefile([json_encode(keycodes)], filename) != 0
    echoerr $'Writing {filename} failed!'
  endif
enddef

# The key entries that we want to list, in this order.
# The first item is displayed in the prompt, the second is the key in
# the keycodes dictionary.
var key_entries = [
	['Tab', 'Tab'],
	['Shift-Tab', 'S-Tab'],
	['Ctrl-Tab', 'C-Tab'],
	['Alt-Tab', 'A-Tab'],
	['Ctrl-I', 'C-I'],
	['Shift-Ctrl-I', 'S-C-I'],
	['Esc', 'Esc'],
	['Shift-Esc', 'S-Esc'],
	['Ctrl-Esc', 'C-Esc'],
	['Alt-Esc', 'A-Esc'],
	['Space', 'Space'],
	['Shift-Space', 'S-Space'],
	['Ctrl-Space', 'C-Space'],
	['Alt-Space', 'A-Space'],
      ]

# Given a terminal name and a item name, return the text to display.
def GetItemDisplay(term: string, item: string): string
  var val = get(keycodes[term], item, '')

  # see if we can pretty-print this one
  var pretty = val
  if val[0 : 1] == '1b'
    pretty = 'ESC'
    var idx = 2

    if val[0 : 3] == '1b5b'
      pretty = 'CSI'
      idx = 4
    endif

    var digits = false
    while idx < len(val)
      var cc = val[idx : idx + 1]
      var nr = str2nr('0x' .. cc, 16)
      idx += 2
      if nr >= char2nr('0') && nr <= char2nr('9')
	if !digits
	  pretty ..= ' '
	endif
	digits = true
	pretty ..= cc[1]
      else
	if nr == char2nr(';') && digits
	  # don't use space between semicolon and digits to keep it short
	  pretty ..= ';'
	else
	  digits = false
	  if nr >= char2nr(' ') && nr <= char2nr('~')
	    # printable character
	    pretty ..= ' ' .. printf('%c', nr)
	  else
	    # non-printable, use hex code
	    pretty = val
	    break
	  endif
	endif
      endif
    endwhile
  endif

  return pretty
enddef


# Action: list the information in "keycodes" in a more or less nice way.
def ActionList()
  var terms = keys(keycodes)
  if len(terms) == 0
    echo 'No terminal results yet'
    return
  endif
  sort(terms)

  var items = ['protocol', 'version', 'kitty', 'modkeys']
	      + key_entries->copy()->map((_, v) => v[1])

  # For each terminal compute the needed width, add two.
  # You may need to increase the terminal width to avoid wrapping.
  var widths = []
  for [idx, term] in items(terms)
    widths[idx] = len(term) + 2
  endfor

  for item in items
    for [idx, term] in items(terms)
      var l = len(GetItemDisplay(term, item))
      if widths[idx] < l + 2
	widths[idx] = l + 2
      endif
    endfor
  endfor

  # Use one column of width 10 for the item name.
  echo "\n"
  echon '          '
  for [idx, term] in items(terms)
    echon printf('%-' .. widths[idx] .. 's', term)
  endfor
  echo "\n"

  for item in items
    echon printf('%8s  ', item)
    for [idx, term] in items(terms)
      echon printf('%-' .. widths[idx] .. 's', GetItemDisplay(term, item))
    endfor
    echo ''
  endfor
  echo "\n"
enddef

# Convert the literal string after "raw key input" into hex form.
def Literal2hex(code: string): string
  var hex = ''
  for i in range(len(code))
    hex ..= printf('%02x', char2nr(code[i]))
  endfor
  return hex
enddef

def GetTermName(): string
  var name = input('Enter the name of the terminal: ')
  return name
enddef

# Gather key codes for terminal "name".
def DoTerm(name: string)
  var proto = inputlist([$'What protocol to enable for {name}:',
			 '1. None',
			 '2. modifyOtherKeys level 2',
			 '3. kitty',
			])
  echo "\n"
  &t_TE = "\<Esc>[>4;m"
  var proto_name = 'unknown'
  if proto == 1
    # Request the XTQMODKEYS value and request the kitty keyboard protocol status.
    &t_TI = "\<Esc>[?4m" .. "\<Esc>[?u"
    proto_name = 'none'
  elseif proto == 2
    # Enable modifyOtherKeys level 2 and request the XTQMODKEYS value.
    &t_TI = "\<Esc>[>4;2m" .. "\<Esc>[?4m"
    proto_name = 'mok2'
  elseif proto == 3
    # Enable Kitty keyboard protocol and request the status.
    &t_TI = "\<Esc>[>1u" .. "\<Esc>[?u"
    proto_name = 'kitty'
  else
    echoerr 'invalid protocol choice'
    return
  endif

  # Append the request for the version response, this is used to check we have
  # the results.
  &t_TI ..= "\<Esc>[>c"

  # Pattern that matches the line with the version response.
  const version_pattern = "\<Esc>\\[>\\d\\+;\\d\\+;\\d*c"

  # Pattern that matches the XTQMODKEYS response:
  #    CSI > 4;Pv m
  # where Pv indicates the modifyOtherKeys level
  const modkeys_pattern = "\<Esc>\\[>4;\\dm"

  # Pattern that matches the line with the status.  Currently what terminals
  # return for the Kitty keyboard protocol.
  const kitty_status_pattern = "\<Esc>\\[?\\d\\+u"

  ch_logfile('keylog', 'w')

  # executing a dummy shell command will output t_TI
  !echo >/dev/null

  # Wait until the log file has the version response.
  var startTime = reltime()
  var seenVersion = false
  while !seenVersion
    var log = readfile('keylog')
    if len(log) > 2
      for line in log
	if line =~ 'raw key input'
	  var code = substitute(line, '.*raw key input: "\([^"]*\).*', '\1', '')
	  if code =~ version_pattern
	    seenVersion = true
	    echo 'Found the version response'
	    break
	  endif
	endif
      endfor
    endif
    if reltime(startTime)->reltimefloat() > 3
      # break out after three seconds
      break
    endif
  endwhile

  echo 'seenVersion: ' seenVersion

  # Prepare the terminal entry, set protocol and clear status and version.
  if !has_key(keycodes, name)
    keycodes[name] = {}
  endif
  keycodes[name]['protocol'] = proto_name
  keycodes[name]['version'] = ''
  keycodes[name]['kitty'] = ''
  keycodes[name]['modkeys'] = ''

  # Check the log file for a status and the version response
  ch_logfile('', '')
  var log = readfile('keylog')
  delete('keylog')

  for line in log
    if line =~ 'raw key input'
      var code = substitute(line, '.*raw key input: "\([^"]*\).*', '\1', '')

      # Check for the XTQMODKEYS response.
      if code =~ modkeys_pattern
	var modkeys = substitute(code, '.*\(' .. modkeys_pattern .. '\).*', '\1', '')
	# We could get the level out of the response, but showing the response
	# itself provides more information.
	# modkeys = substitute(modkeys, '.*4;\(\d\)m', '\1', '')

	if keycodes[name]['modkeys'] != ''
	  echomsg 'Another modkeys found after ' .. keycodes[name]['modkeys']
	endif
	keycodes[name]['modkeys'] = modkeys
      endif

      # Check for kitty keyboard protocol status
      if code =~ kitty_status_pattern
	var status = substitute(code, '.*\(' .. kitty_status_pattern .. '\).*', '\1', '')
	# use the response itself as the status
	status = Literal2hex(status)

	if keycodes[name]['kitty'] != ''
	  echomsg 'Another status found after ' .. keycodes[name]['kitty']
	endif
	keycodes[name]['kitty'] = status
      endif

      if code =~ version_pattern
	var version = substitute(code, '.*\(' .. version_pattern .. '\).*', '\1', '')
	keycodes[name]['version'] = Literal2hex(version)
	break
      endif
    endif
  endfor

  echo "For Alt to work you may need to press the Windows/Super key as well"
  echo "When a key press doesn't get to Vim (e.g. when using Alt) press x"

  # The log of ignored typeahead is left around for debugging, start with an
  # empty file here.
  delete('keylog-ignore')

  for entry in key_entries
    # Consume any typeahead.  Wait a bit for any responses to arrive.
    ch_logfile('keylog-ignore', 'a')
    while 1
      sleep 100m
      if getchar(1) == 0
	break
      endif
      while getchar(1) != 0
	getchar()
      endwhile
    endwhile
    ch_logfile('', '')

    ch_logfile('keylog', 'w')
    echo $'Press the {entry[0]} key (q to quit):'
    var r = getcharstr()
    ch_logfile('', '')
    if r == 'q'
      break
    endif

    log = readfile('keylog')
    delete('keylog')
    if len(log) < 2
      echoerr 'failed to read result'
      return
    endif
    var done = false
    for line in log
      if line =~ 'raw key input'
	var code = substitute(line, '.*raw key input: "\([^"]*\).*', '\1', '')

	# Remove any version termresponse
	code = substitute(code, version_pattern, '', 'g')

	# Remove any XTGETTCAP replies.
	const cappat = "\<Esc>P[01]+\\k\\+=\\x*\<Esc>\\\\"
	code = substitute(code, cappat, '', 'g')

	# Remove any kitty status reply
	code = substitute(code, kitty_status_pattern, '', 'g')
	if code == ''
	  continue
	endif

	# Convert the literal bytes into hex.  If 'x' was pressed then clear
	# the entry.
	var hex = ''
	if code != 'x'
	  hex = Literal2hex(code)
	endif

	keycodes[name][entry[1]] = hex
	done = true
	break
      endif
    endfor
    if !done
      echo 'Code not found in log'
    endif
  endfor
enddef

# Action: Add key codes for a new terminal.
def ActionAdd()
  var name = input('Enter name of the terminal: ')
  echo "\n"
  if index(keys(keycodes), name) >= 0
    echoerr $'Terminal {name} already exists'
    return
  endif

  DoTerm(name)
enddef

# Action: Replace key codes for an already known terminal.
def ActionReplace()
  var terms = keys(keycodes)
  if len(terms) == 0
    echo 'No terminal results yet'
    return
  endif

  var choice = inputlist(['Select:'] + terms->copy()->map((idx, arg) => (idx + 1) .. ': ' .. arg))
  echo "\n"
  if choice > 0 && choice <= len(terms)
    DoTerm(terms[choice - 1])
  else
    echo 'invalid index'
  endif
enddef

# Action: Clear key codes for an already known terminal.
def ActionClear()
  var terms = keys(keycodes)
  if len(terms) == 0
    echo 'No terminal results yet'
    return
  endif

  var choice = inputlist(['Select:'] + terms->copy()->map((idx, arg) => (idx + 1) .. ': ' .. arg))
  echo "\n"
  if choice > 0 && choice <= len(terms)
    remove(keycodes, terms[choice - 1])
  else
    echo 'invalid index'
  endif
enddef

# Action: Quit, possibly after saving the results first.
def ActionQuit()
  # If nothing was changed just quit
  if keycodes == orig_keycodes
    quit
  endif

  while true
    var res = input("Save the changed key codes (y/n)? ")
    if res == 'n'
      quit
    endif
    if res == 'y'
      WriteKeycodes()
      quit
    endif
    echo 'invalid reply'
  endwhile
enddef

# The main loop
while true
  var action = inputlist(['Select operation:',
			'1. List results',
			'2. Add results for a new terminal',
			'3. Replace results',
			'4. Clear results',
			'5. Quit',
		      ])
  echo "\n"
  if action == 1
    ActionList()
  elseif action == 2
    ActionAdd()
  elseif action == 3
    ActionReplace()
  elseif action == 4
    ActionClear()
  elseif action == 5
    ActionQuit()
  endif
endwhile
