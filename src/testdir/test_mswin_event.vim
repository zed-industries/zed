" Test MS-Windows input event handling.
" Most of this works the same in Windows GUI as well as Windows console.

source check.vim
CheckMSWindows
source mouse.vim

" Helper function for sending a grouped sequence of low level key presses
" The modifier key(s) can be included as VK Key Codes in the sequence
" Keydown events will be sent, to to the end of the group, then keyup events
" will be sent in reverse order to release the keys.
func SendKeyGroup(keygroup)
  for k in a:keygroup
    call test_mswin_event("key", {'event': "keydown", 'keycode': k})
  endfor
  for k in reverse(copy(a:keygroup))
    call test_mswin_event("key", {'event': "keyup", 'keycode': k})
  endfor
endfunc

" Send individual key press and release events.
" the modifiers for the key press can be specified in the modifiers arg.
func SendKeyWithModifiers(key, modifiers)
  let args = { }
  let args.keycode = a:key
  let args.modifiers = a:modifiers
  let args.event = "keydown"
  call test_mswin_event("key", args)
  let args.event = "keyup"
  call test_mswin_event("key", args)
  unlet args
endfunc

" Send an individual key press, without modifiers.
func SendKey(key)
  call SendKeyWithModifiers(a:key, 0)
endfunc

" getcharstr(0) but catch Vim:Interrupt
func Getcharstr()
  try
    let ch = getcharstr(0)
  catch /^Vim:Interrupt$/
    let ch = "\<c-c>"
  endtry
  return ch
endfunc


" Send a string of individual key-press events, without modifiers.
func SendKeyStr(keystring)
  for k in a:keystring
    call SendKey(k)
  endfor
endfunc

" This tells Vim to execute the buffered keys as user commands,
" ie. same as feekdeys with mode X would do.
func ExecuteBufferedKeys()
  if has('gui_running')
    call feedkeys("\<Esc>", 'Lx!')
  else
    call test_mswin_event("key", {'execute': v:true})
  endif
endfunc

" Refer to the following page for the virtual key codes:
" https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
let s:VK = {
    \ 'ENTER'      : 0x0D,
    \ 'SPACE'      : 0x20,
    \ 'SHIFT'      : 0x10,
    \ 'LSHIFT'     : 0xA0,
    \ 'RSHIFT'     : 0xA1,
    \ 'CONTROL'    : 0x11,
    \ 'LCONTROL'   : 0xA2,
    \ 'RCONTROL'   : 0xA3,
    \ 'MENU'       : 0x12,
    \ 'ALT'        : 0x12,
    \ 'LMENU'      : 0xA4,
    \ 'LALT'       : 0xA4,
    \ 'RMENU'      : 0xA5,
    \ 'RALT'       : 0xA5,
    \ 'OEM_1'      : 0xBA,
    \ 'OEM_2'      : 0xBF,
    \ 'OEM_3'      : 0xC0,
    \ 'OEM_4'      : 0xDB,
    \ 'OEM_5'      : 0xDC,
    \ 'OEM_6'      : 0xDD,
    \ 'OEM_7'      : 0xDE,
    \ 'OEM_PLUS'   : 0xBB,
    \ 'OEM_COMMA'  : 0xBC,
    \ 'OEM_MINUS'  : 0xBD,
    \ 'OEM_PERIOD' : 0xBE,
    \ 'PRIOR'      : 0x21,
    \ 'NEXT'       : 0x22,
    \ 'END'        : 0x23,
    \ 'HOME'       : 0x24,
    \ 'LEFT'       : 0x25,
    \ 'UP'         : 0x26,
    \ 'RIGHT'      : 0x27,
    \ 'DOWN'       : 0x28,
    \ 'KEY_0'      : 0x30,
    \ 'KEY_1'      : 0x31,
    \ 'KEY_2'      : 0x32,
    \ 'KEY_3'      : 0x33,
    \ 'KEY_4'      : 0x34,
    \ 'KEY_5'      : 0x35,
    \ 'KEY_6'      : 0x36,
    \ 'KEY_7'      : 0x37,
    \ 'KEY_8'      : 0x38,
    \ 'KEY_9'      : 0x39,
    \ 'KEY_A'      : 0x41,
    \ 'KEY_B'      : 0x42,
    \ 'KEY_C'      : 0x43,
    \ 'KEY_D'      : 0x44,
    \ 'KEY_E'      : 0x45,
    \ 'KEY_F'      : 0x46,
    \ 'KEY_G'      : 0x47,
    \ 'KEY_H'      : 0x48,
    \ 'KEY_I'      : 0x49,
    \ 'KEY_J'      : 0x4A,
    \ 'KEY_K'      : 0x4B,
    \ 'KEY_L'      : 0x4C,
    \ 'KEY_M'      : 0x4D,
    \ 'KEY_N'      : 0x4E,
    \ 'KEY_O'      : 0x4F,
    \ 'KEY_P'      : 0x50,
    \ 'KEY_Q'      : 0x51,
    \ 'KEY_R'      : 0x52,
    \ 'KEY_S'      : 0x53,
    \ 'KEY_T'      : 0x54,
    \ 'KEY_U'      : 0x55,
    \ 'KEY_V'      : 0x56,
    \ 'KEY_W'      : 0x57,
    \ 'KEY_X'      : 0x58,
    \ 'KEY_Y'      : 0x59,
    \ 'KEY_Z'      : 0x5A,
    \ 'NUMPAD0'    : 0x60,
    \ 'NUMPAD1'    : 0x61,
    \ 'NUMPAD2'    : 0x62,
    \ 'NUMPAD3'    : 0x63,
    \ 'NUMPAD4'    : 0x64,
    \ 'NUMPAD5'    : 0x65,
    \ 'NUMPAD6'    : 0x66,
    \ 'NUMPAD7'    : 0x67,
    \ 'NUMPAD8'    : 0x68,
    \ 'NUMPAD9'    : 0x69,
    \ 'MULTIPLY'   : 0x6A,
    \ 'ADD'        : 0x6B,
    \ 'SUBTRACT'   : 0x6D,
    \ 'F1'         : 0x70,
    \ 'F2'         : 0x71,
    \ 'F3'         : 0x72,
    \ 'F4'         : 0x73,
    \ 'F5'         : 0x74,
    \ 'F6'         : 0x75,
    \ 'F7'         : 0x76,
    \ 'F8'         : 0x77,
    \ 'F9'         : 0x78,
    \ 'F10'        : 0x79,
    \ 'F11'        : 0x7A,
    \ 'F12'        : 0x7B,
    \ 'DELETE'     : 0x2E,
    \ 'BACK'       : 0x08,
    \ 'ESCAPE'     : 0x1B
    \ }

  let s:MOD_MASK_SHIFT = 0x02
  let s:MOD_MASK_CTRL  = 0x04
  let s:MOD_MASK_ALT   = 0x08

  let s:vim_key_modifiers = [
    \ ["",       0,   []],
    \ ["S-",     2,   [s:VK.LSHIFT]],
    \ ["C-",     4,   [s:VK.LCONTROL]],
    \ ["C-S-",   6,   [s:VK.LCONTROL, s:VK.LSHIFT]],
    \ ["A-",     8,   [s:VK.LMENU]],
    \ ["A-S-",   10,  [s:VK.LMENU, s:VK.LSHIFT]],
    \ ["A-C-",   12,  [s:VK.LMENU, s:VK.LCONTROL]],
    \ ["A-C-S-", 14,  [s:VK.LMENU, s:VK.LCONTROL, s:VK.LSHIFT]],
    \]

  " Assuming Standard US PC Keyboard layout
  let s:test_ascii_key_chars = [
    \ [[s:VK.SPACE], ' '],
    \ [[s:VK.OEM_1], ';'],
    \ [[s:VK.OEM_2], '/'],
    \ [[s:VK.OEM_3], '`'],
    \ [[s:VK.OEM_4], '['],
    \ [[s:VK.OEM_5], '\'],
    \ [[s:VK.OEM_6], ']'],
    \ [[s:VK.OEM_7], ''''],
    \ [[s:VK.OEM_PLUS], '='],
    \ [[s:VK.OEM_COMMA], ','],
    \ [[s:VK.OEM_MINUS], '-'],
    \ [[s:VK.OEM_PERIOD], '.'],
    \ [[s:VK.SHIFT, s:VK.OEM_1], ':'],
    \ [[s:VK.SHIFT, s:VK.OEM_2], '?'],
    \ [[s:VK.SHIFT, s:VK.OEM_3], '~'],
    \ [[s:VK.SHIFT, s:VK.OEM_4], '{'],
    \ [[s:VK.SHIFT, s:VK.OEM_5], '|'],
    \ [[s:VK.SHIFT, s:VK.OEM_6], '}'],
    \ [[s:VK.SHIFT, s:VK.OEM_7], '"'],
    \ [[s:VK.SHIFT, s:VK.OEM_PLUS], '+'],
    \ [[s:VK.SHIFT, s:VK.OEM_COMMA], '<'],
    \ [[s:VK.SHIFT, s:VK.OEM_MINUS], '_'],
    \ [[s:VK.SHIFT, s:VK.OEM_PERIOD], '>'],
    \ [[s:VK.KEY_1], '1'],
    \ [[s:VK.KEY_2], '2'],
    \ [[s:VK.KEY_3], '3'],
    \ [[s:VK.KEY_4], '4'],
    \ [[s:VK.KEY_5], '5'],
    \ [[s:VK.KEY_6], '6'],
    \ [[s:VK.KEY_7], '7'],
    \ [[s:VK.KEY_8], '8'],
    \ [[s:VK.KEY_9], '9'],
    \ [[s:VK.KEY_0], '0'],
    \ [[s:VK.SHIFT, s:VK.KEY_1], '!'],
    \ [[s:VK.SHIFT, s:VK.KEY_2], '@'],
    \ [[s:VK.SHIFT, s:VK.KEY_3], '#'],
    \ [[s:VK.SHIFT, s:VK.KEY_4], '$'],
    \ [[s:VK.SHIFT, s:VK.KEY_5], '%'],
    \ [[s:VK.SHIFT, s:VK.KEY_6], '^'],
    \ [[s:VK.SHIFT, s:VK.KEY_7], '&'],
    \ [[s:VK.SHIFT, s:VK.KEY_8], '*'],
    \ [[s:VK.SHIFT, s:VK.KEY_9], '('],
    \ [[s:VK.SHIFT, s:VK.KEY_0], ')'],
    \ [[s:VK.KEY_A], 'a'],
    \ [[s:VK.KEY_B], 'b'],
    \ [[s:VK.KEY_C], 'c'],
    \ [[s:VK.KEY_D], 'd'],
    \ [[s:VK.KEY_E], 'e'],
    \ [[s:VK.KEY_F], 'f'],
    \ [[s:VK.KEY_G], 'g'],
    \ [[s:VK.KEY_H], 'h'],
    \ [[s:VK.KEY_I], 'i'],
    \ [[s:VK.KEY_J], 'j'],
    \ [[s:VK.KEY_K], 'k'],
    \ [[s:VK.KEY_L], 'l'],
    \ [[s:VK.KEY_M], 'm'],
    \ [[s:VK.KEY_N], 'n'],
    \ [[s:VK.KEY_O], 'o'],
    \ [[s:VK.KEY_P], 'p'],
    \ [[s:VK.KEY_Q], 'q'],
    \ [[s:VK.KEY_R], 'r'],
    \ [[s:VK.KEY_S], 's'],
    \ [[s:VK.KEY_T], 't'],
    \ [[s:VK.KEY_U], 'u'],
    \ [[s:VK.KEY_V], 'v'],
    \ [[s:VK.KEY_W], 'w'],
    \ [[s:VK.KEY_X], 'x'],
    \ [[s:VK.KEY_Y], 'y'],
    \ [[s:VK.KEY_Z], 'z'],
    \ [[s:VK.SHIFT, s:VK.KEY_A], 'A'],
    \ [[s:VK.SHIFT, s:VK.KEY_B], 'B'],
    \ [[s:VK.SHIFT, s:VK.KEY_C], 'C'],
    \ [[s:VK.SHIFT, s:VK.KEY_D], 'D'],
    \ [[s:VK.SHIFT, s:VK.KEY_E], 'E'],
    \ [[s:VK.SHIFT, s:VK.KEY_F], 'F'],
    \ [[s:VK.SHIFT, s:VK.KEY_G], 'G'],
    \ [[s:VK.SHIFT, s:VK.KEY_H], 'H'],
    \ [[s:VK.SHIFT, s:VK.KEY_I], 'I'],
    \ [[s:VK.SHIFT, s:VK.KEY_J], 'J'],
    \ [[s:VK.SHIFT, s:VK.KEY_K], 'K'],
    \ [[s:VK.SHIFT, s:VK.KEY_L], 'L'],
    \ [[s:VK.SHIFT, s:VK.KEY_M], 'M'],
    \ [[s:VK.SHIFT, s:VK.KEY_N], 'N'],
    \ [[s:VK.SHIFT, s:VK.KEY_O], 'O'],
    \ [[s:VK.SHIFT, s:VK.KEY_P], 'P'],
    \ [[s:VK.SHIFT, s:VK.KEY_Q], 'Q'],
    \ [[s:VK.SHIFT, s:VK.KEY_R], 'R'],
    \ [[s:VK.SHIFT, s:VK.KEY_S], 'S'],
    \ [[s:VK.SHIFT, s:VK.KEY_T], 'T'],
    \ [[s:VK.SHIFT, s:VK.KEY_U], 'U'],
    \ [[s:VK.SHIFT, s:VK.KEY_V], 'V'],
    \ [[s:VK.SHIFT, s:VK.KEY_W], 'W'],
    \ [[s:VK.SHIFT, s:VK.KEY_X], 'X'],
    \ [[s:VK.SHIFT, s:VK.KEY_Y], 'Y'],
    \ [[s:VK.SHIFT, s:VK.KEY_Z], 'Z'],
    \ [[s:VK.CONTROL, s:VK.KEY_A], 0x01],
    \ [[s:VK.CONTROL, s:VK.KEY_B], 0x02],
    \ [[s:VK.CONTROL, s:VK.KEY_C], 0x03],
    \ [[s:VK.CONTROL, s:VK.KEY_D], 0x04],
    \ [[s:VK.CONTROL, s:VK.KEY_E], 0x05],
    \ [[s:VK.CONTROL, s:VK.KEY_F], 0x06],
    \ [[s:VK.CONTROL, s:VK.KEY_G], 0x07],
    \ [[s:VK.CONTROL, s:VK.KEY_H], 0x08],
    \ [[s:VK.CONTROL, s:VK.KEY_I], 0x09],
    \ [[s:VK.CONTROL, s:VK.KEY_J], 0x0A],
    \ [[s:VK.CONTROL, s:VK.KEY_K], 0x0B],
    \ [[s:VK.CONTROL, s:VK.KEY_L], 0x0C],
    \ [[s:VK.CONTROL, s:VK.KEY_M], 0x0D],
    \ [[s:VK.CONTROL, s:VK.KEY_N], 0x0E],
    \ [[s:VK.CONTROL, s:VK.KEY_O], 0x0F],
    \ [[s:VK.CONTROL, s:VK.KEY_P], 0x10],
    \ [[s:VK.CONTROL, s:VK.KEY_Q], 0x11],
    \ [[s:VK.CONTROL, s:VK.KEY_R], 0x12],
    \ [[s:VK.CONTROL, s:VK.KEY_S], 0x13],
    \ [[s:VK.CONTROL, s:VK.KEY_T], 0x14],
    \ [[s:VK.CONTROL, s:VK.KEY_U], 0x15],
    \ [[s:VK.CONTROL, s:VK.KEY_V], 0x16],
    \ [[s:VK.CONTROL, s:VK.KEY_W], 0x17],
    \ [[s:VK.CONTROL, s:VK.KEY_X], 0x18],
    \ [[s:VK.CONTROL, s:VK.KEY_Y], 0x19],
    \ [[s:VK.CONTROL, s:VK.KEY_Z], 0x1A],
    \ [[s:VK.CONTROL, s:VK.OEM_4], 0x1B],
    \ [[s:VK.CONTROL, s:VK.OEM_5], 0x1C],
    \ [[s:VK.CONTROL, s:VK.OEM_6], 0x1D],
    \ [[s:VK.CONTROL, s:VK.KEY_6], 0x1E],
    \ [[s:VK.CONTROL, s:VK.OEM_MINUS], 0x1F],
    \ ]

let s:test_extra_key_chars = [
    \ [[s:VK.ALT, s:VK.KEY_1], '±'],
    \ [[s:VK.ALT, s:VK.KEY_2], '²'],
    \ [[s:VK.ALT, s:VK.KEY_3], '³'],
    \ [[s:VK.ALT, s:VK.KEY_4], '´'],
    \ [[s:VK.ALT, s:VK.KEY_5], 'µ'],
    \ [[s:VK.ALT, s:VK.KEY_6], '¶'],
    \ [[s:VK.ALT, s:VK.KEY_7], '·'],
    \ [[s:VK.ALT, s:VK.KEY_8], '¸'],
    \ [[s:VK.ALT, s:VK.KEY_9], '¹'],
    \ [[s:VK.ALT, s:VK.KEY_0], '°'],
    \ [[s:VK.ALT, s:VK.KEY_A], 'á'],
    \ [[s:VK.ALT, s:VK.KEY_B], 'â'],
    \ [[s:VK.ALT, s:VK.KEY_C], 'ã'],
    \ [[s:VK.ALT, s:VK.KEY_D], 'ä'],
    \ [[s:VK.ALT, s:VK.KEY_E], 'å'],
    \ [[s:VK.ALT, s:VK.KEY_F], 'æ'],
    \ [[s:VK.ALT, s:VK.KEY_G], 'ç'],
    \ [[s:VK.ALT, s:VK.KEY_H], 'è'],
    \ [[s:VK.ALT, s:VK.KEY_I], 'é'],
    \ [[s:VK.ALT, s:VK.KEY_J], 'ê'],
    \ [[s:VK.ALT, s:VK.KEY_K], 'ë'],
    \ [[s:VK.ALT, s:VK.KEY_L], 'ì'],
    \ [[s:VK.ALT, s:VK.KEY_M], 'í'],
    \ [[s:VK.ALT, s:VK.KEY_N], 'î'],
    \ [[s:VK.ALT, s:VK.KEY_O], 'ï'],
    \ [[s:VK.ALT, s:VK.KEY_P], 'ð'],
    \ [[s:VK.ALT, s:VK.KEY_Q], 'ñ'],
    \ [[s:VK.ALT, s:VK.KEY_R], 'ò'],
    \ [[s:VK.ALT, s:VK.KEY_S], 'ó'],
    \ [[s:VK.ALT, s:VK.KEY_T], 'ô'],
    \ [[s:VK.ALT, s:VK.KEY_U], 'õ'],
    \ [[s:VK.ALT, s:VK.KEY_V], 'ö'],
    \ [[s:VK.ALT, s:VK.KEY_W], '÷'],
    \ [[s:VK.ALT, s:VK.KEY_X], 'ø'],
    \ [[s:VK.ALT, s:VK.KEY_Y], 'ù'],
    \ [[s:VK.ALT, s:VK.KEY_Z], 'ú'],
    \ ]

func s:LoopTestKeyArray(arr)
  " flush out the typeahead buffer
  while getchar(0)
  endwhile

  for [kcodes, kstr] in a:arr
    " Send as a sequence of key presses.
    call SendKeyGroup(kcodes)
    let ch = Getcharstr()
    " need to deal a bit differently with the non-printable ascii chars < 0x20
    if kstr < 0x20 && index([s:VK.CONTROL, s:VK.LCONTROL, s:VK.RCONTROL], kcodes[0]) >= 0
      call assert_equal(nr2char(kstr), $"{ch}")
    else
      call assert_equal(kstr, $"{ch}")
    endif
    let mod_mask = getcharmod()
    " the mod_mask is zero when no modifiers are used
    " and when the virtual termcap maps the character
    call assert_equal(0, mod_mask, $"key = {kstr}")

    " Send as a single key press with a modifiers mask.
    let modifiers = 0
    let key = kcodes[0]
    for key in kcodes
      if index([s:VK.SHIFT, s:VK.LSHIFT, s:VK.RSHIFT], key) >= 0
        let modifiers = modifiers + s:MOD_MASK_SHIFT
      endif
      if index([s:VK.CONTROL, s:VK.LCONTROL, s:VK.RCONTROL], key) >= 0
        let modifiers = modifiers + s:MOD_MASK_CTRL
      endif
      if index([s:VK.ALT, s:VK.LALT, s:VK.RALT], key) >= 0
        let modifiers = modifiers + s:MOD_MASK_ALT
      endif
    endfor
    call SendKeyWithModifiers(key, modifiers)
    let ch = Getcharstr()
    " need to deal a bit differently with the non-printable ascii chars < 0x20
    if kstr < 0x20 && index([s:VK.CONTROL, s:VK.LCONTROL, s:VK.RCONTROL],  kcodes[0]) >= 0
      call assert_equal(nr2char(kstr), $"{ch}")
    else
      call assert_equal(kstr, $"{ch}")
    endif
    let mod_mask = getcharmod()
    " the mod_mask is zero when no modifiers are used
    " and when the virtual termcap maps the character
    call assert_equal(0, mod_mask, $"key = {kstr}")
  endfor

  " flush out the typeahead buffer
  while getchar(0)
  endwhile

endfunc

" Test MS-Windows key events
func Test_mswin_event_character_keys()
  CheckMSWindows
  new

  call s:LoopTestKeyArray(s:test_ascii_key_chars)

  if !has('gui_running')
    call s:LoopTestKeyArray(s:test_extra_key_chars)
  endif

" Test keyboard codes for digits
" (0x30 - 0x39) : VK_0 - VK_9 are the same as ASCII '0' - '9'
  for kc in range(48, 57)
    call SendKey(kc)
    let ch = Getcharstr()
    call assert_equal(nr2char(kc), ch)
    call SendKeyWithModifiers(kc, 0)
    let ch = Getcharstr()
    call assert_equal(nr2char(kc), ch)
  endfor

" Test keyboard codes for Alt-0 to Alt-9
" Expect +128 from the digit char codes
  for modkey in [s:VK.ALT, s:VK.LALT, s:VK.RALT]
    for kc in range(48, 57)
      call SendKeyGroup([modkey, kc])
      let ch = getchar(0)
      call assert_equal(kc+128, ch)
      call SendKeyWithModifiers(kc, s:MOD_MASK_ALT)
      let ch = getchar(0)
      call assert_equal(kc+128, ch)
    endfor
  endfor

" Test for lowercase 'a' to 'z', VK codes 65(0x41) - 90(0x5A)
" Note: VK_A-VK_Z virtual key codes coincide with uppercase ASCII codes A-Z.
" eg VK_A is 65, and the ASCII character code for uppercase 'A' is also 65.
" Caution: these are interpreted as lowercase when Shift is NOT pressed.
" eg, sending VK_A (65) 'A' Key code without shift modifier, will produce ASCII
" char 'a' (91) as the output.  The ASCII codes for the lowercase letters are
" numbered 32 higher than their uppercase versions.
  for kc in range(65, 90)
    call SendKey(kc)
    let ch = Getcharstr()
    call assert_equal(nr2char(kc + 32), ch)
    call SendKeyWithModifiers(kc, 0)
    let ch = Getcharstr()
    call assert_equal(nr2char(kc + 32), ch)
  endfor

"  Test for Uppercase 'A' - 'Z' keys
"  ie. with VK_SHIFT, expect the keycode = character code.
  for modkey in [s:VK.SHIFT, s:VK.LSHIFT, s:VK.RSHIFT]
    for kc in range(65, 90)
      call SendKeyGroup([modkey, kc])
      let ch = Getcharstr()
      call assert_equal(nr2char(kc), ch)
      call SendKeyWithModifiers(kc, s:MOD_MASK_SHIFT)
      let ch = Getcharstr()
      call assert_equal(nr2char(kc), ch)
    endfor
  endfor

" Test for <Ctrl-A> to <Ctrl-Z> keys
" Expect the unicode characters 0x01 to 0x1A
" Note: Skip C because it triggers an Interrupt (CTRL-C)
"       which causes a test failure
   for modkey in [s:VK.CONTROL, s:VK.LCONTROL, s:VK.RCONTROL]
    for kc in range(65, 90)
      if kc == 67
        continue
      endif
      call SendKeyGroup([modkey, kc])
      let ch = Getcharstr()
      call assert_equal(nr2char(kc - 64), ch)
      call SendKeyWithModifiers(kc, s:MOD_MASK_CTRL)
      let ch = Getcharstr()
      call assert_equal(nr2char(kc - 64), ch)
    endfor
  endfor

  "  Windows intercepts some of these keys in the GUI.
  if !has("gui_running")
  "  Test for <Alt-A> to <Alt-Z> keys
  "  Expect the unicode characters 0xE1 to 0xFA
  "  ie. 160 higher than the lowercase equivalent
    for modkey in [s:VK.ALT, s:VK.LALT, s:VK.RALT]
      for kc in range(65, 90)
        call SendKeyGroup([modkey, kc])
        let ch = getchar(0)
        call assert_equal(kc+160, ch)
        call SendKeyWithModifiers(kc, s:MOD_MASK_ALT)
        let ch = getchar(0)
        call assert_equal(kc+160, ch)
      endfor
    endfor
  endif

endfun

  " Test for Function Keys 'F1' to 'F12'
  " VK codes 112(0x70) - 123(0x7B)
  " Also with ALL permutatios of modifiers; Shift, Ctrl & Alt
func Test_mswin_event_function_keys()

  if has('gui_running')
    let g:test_is_flaky = 1
  endif

  " NOTE: Windows intercepts these combinations in the GUI
  let gui_nogo = ["A-F1", "A-F2", "A-F3", "A-F4", "A-S-F4", "A-C-S-F4",
            \ "A-F5", "A-F6", "A-F7", "A-F8", "A-C-F8", "A-F9",
	    \ "A-F10", "A-F11" , "A-C-F11", "A-C-F12"]

  " flush out the typeahead buffer
  while getchar(0)
  endwhile

  for [mod_str, vim_mod_mask, mod_keycodes] in s:vim_key_modifiers
    for n in range(1, 12)
      let expected_mod_mask = vim_mod_mask
      let kstr = $"{mod_str}F{n}"
      if !has('gui_running') || (has('gui_running') && n != 10
                                             \  && index(gui_nogo, kstr) == -1)
        let keycode = eval('"\<' .. kstr .. '>"')
        " flush out the typeahead buffer
        while getchar(0)
        endwhile
        call SendKeyWithModifiers(111+n, vim_mod_mask)
        let ch = Getcharstr()
        let mod_mask = getcharmod()
        call assert_equal(keycode, $"{ch}", $"key = {kstr}")
        " workaround for the virtual termcap maps changing the character
        "instead of sending Shift
        for mod_key in mod_keycodes
          if index([s:VK.SHIFT, s:VK.LSHIFT, s:VK.RSHIFT], mod_key) >= 0
            let expected_mod_mask -= s:MOD_MASK_SHIFT
            break
          endif
        endfor
        call assert_equal(expected_mod_mask, mod_mask, $"mod = {expected_mod_mask} for key = {kstr}")
      endif
    endfor
  endfor
endfunc

func ExtractModifiers(mod_keycodes)
  let has_shift = 0
  let has_ctrl = 0
  let has_alt = 0
  for mod_key in a:mod_keycodes
    if index([s:VK.SHIFT, s:VK.LSHIFT, s:VK.RSHIFT], mod_key) >= 0
      let has_shift = 1
    endif
    if index([s:VK.CONTROL, s:VK.LCONTROL, s:VK.RCONTROL], mod_key) >= 0
      let has_ctrl = 1
    endif
    if index([s:VK.MENU, s:VK.LMENU, s:VK.RMENU], mod_key) >= 0
      let has_alt = 1
    endif
  endfor
  return [has_shift, has_ctrl, has_alt]
endfunc

  " Test for Movement Keys;
  "    VK_PRIOR 0x21,   VK_NEXT  0x22,
  "    VK_END   0x23,   VK_HOME  0x24,
  "    VK_LEFT  0x25,   VK_UP    0x26,
  "    VK_RIGHT 0x27,   VK_DOWN  0x28
  " With ALL permutations of modifiers; none, Shift, Ctrl & Alt
func Test_mswin_event_movement_keys()

  if has('gui_running')
    let g:test_is_flaky = 1
  endif

  let movement_keys = [
    \ [s:VK.PRIOR, "PageUp"],
    \ [s:VK.NEXT,  "PageDown"],
    \ [s:VK.END,   "End"],
    \ [s:VK.HOME,  "Home"],
    \ [s:VK.LEFT,  "Left"],
    \ [s:VK.UP,    "Up"],
    \ [s:VK.RIGHT, "Right"],
    \ [s:VK.DOWN,  "Down"],
    \ ]

  " flush out the typeahead buffer
  while getchar(0)
  endwhile

  for [mod_str, vim_mod_mask, mod_keycodes] in s:vim_key_modifiers
    for [kcode, kname] in movement_keys
      let exp_mod_mask = vim_mod_mask
      let kstr = $"{mod_str}{kname}"
      let chstr_eval = eval('"\<' .. kstr .. '>"')

      " flush out the typeahead buffer
      while getchar(0)
      endwhile
      execute 'call feedkeys("\<' .. kstr .. '>")'
      let chstr_fk = Getcharstr()
      call assert_equal(chstr_eval, chstr_fk, $"feedkeys = <{kstr}>")

      " flush out the typeahead buffer
      while getchar(0)
      endwhile
      call SendKey(kcode)
      let chstr_alone = Getcharstr()
      let chstr_alone_end = chstr_alone[len(chstr_alone)-2:len(chstr_alone)-1]

      " flush out the typeahead buffer
      while getchar(0)
      endwhile
      call SendKeyGroup(mod_keycodes + [kcode])
      let chstr_mswin = Getcharstr()
      let chstr_mswin_end = chstr_mswin[len(chstr_mswin)-2:len(chstr_mswin)-1]
      let mod_mask = getcharmod()

      " The virtual termcap maps may** change the character and either;
      " - remove the Shift modifier, or
      " - remove the Ctrl modifier if the Shift modifier was not removed.
      let [has_shift, has_ctrl, has_alt] = ExtractModifiers(mod_keycodes)
      if chstr_alone_end != chstr_mswin_end
        if has_shift != 0
          let exp_mod_mask -= s:MOD_MASK_SHIFT
        elseif has_ctrl != 0
	  let exp_mod_mask -= s:MOD_MASK_CTRL
        endif
      endif
      " **Note: The appveyor Windows GUI test environments, from VS2017 on,
      " consistently intercepts the Shift modifier WITHOUT changing the
      " MOVEMENT character.  This issue does not happen in any github actions
      " CI Windows test environments.  Attempted to reproduce this manually
      " on Windows versions;  7, 8.1, 10, 11, Server 2019 and Server 2022, but
      " the issue did not occur on any of those environments.
      " Below is a workaround for the issue.
      if has('gui_running') && has_shift != 0
        if exp_mod_mask != mod_mask && chstr_eval != chstr_mswin
          let kstr_sub = substitute(kstr, "S-", "", "")
          let chstr_eval = eval('"\<' .. kstr_sub .. '>"')
          if exp_mod_mask - s:MOD_MASK_SHIFT == mod_mask
            let exp_mod_mask -= s:MOD_MASK_SHIFT
          elseif has_ctrl != 0 && exp_mod_mask - s:MOD_MASK_CTRL == mod_mask
            let exp_mod_mask -= s:MOD_MASK_CTRL
          endif
        endif
      endif
      call assert_equal(chstr_eval, chstr_mswin, $"key = {kstr}")
      call assert_equal(exp_mod_mask, mod_mask, $"mod_mask for key = {kstr}")
    endfor
  endfor

  bw!
endfunc


" Test for QWERTY Ctrl+- which should result in ^_
" issue #10817
func Test_QWERTY_Ctrl_minus()
  CheckMSWindows
  new

  call SendKeyGroup([s:VK.CONTROL, s:VK.OEM_MINUS])
  let ch = Getcharstr()
  call assert_equal(nr2char(0x1f),ch)

  call SendKey(s:VK.KEY_I)
  call SendKeyGroup([s:VK.CONTROL, s:VK.SUBTRACT])
  call SendKey(s:VK.ESCAPE)
  call ExecuteBufferedKeys()
  call assert_equal('-', getline('$'))

  %d _
  imapclear
  imap <C-_> BINGO
  call SendKey(s:VK.KEY_I)
  call SendKeyGroup([s:VK.CONTROL, s:VK.OEM_MINUS])
  call SendKey(s:VK.ESCAPE)
  call ExecuteBufferedKeys()
  call assert_equal('BINGO', getline('$'))

  %d _
  imapclear
  exec "imap \x1f BILBO"
  call SendKey(s:VK.KEY_I)
  call SendKeyGroup([s:VK.CONTROL, s:VK.OEM_MINUS])
  call SendKey(s:VK.ESCAPE)
  call ExecuteBufferedKeys()
  call assert_equal('BILBO', getline('$'))

  imapclear
  bw!
endfunc

"  Test MS-Windows mouse events
func Test_mswin_event_mouse()
  CheckMSWindows
  new

  set mousemodel=extend
  call test_override('no_query_mouse', 1)
  call WaitForResponses()

  let msg = ''

  call setline(1, ['one two three', 'four five six'])

  " Test mouse movement
  " by default, no mouse move events are generated
  " this setting enables it to generate move events
  set mousemev

  if !has('gui_running')
    " console version needs a button pressed,
    " otherwise it ignores mouse movements.
    call MouseLeftClick(2, 3)
  endif
  call MSWinMouseEvent(0x700, 8, 13, 0, 0, 0)
  if has('gui_running')
    call feedkeys("\<Esc>", 'Lx!')
  endif
  let pos = getmousepos()
  call assert_equal(8, pos.screenrow)
  call assert_equal(13, pos.screencol)

  if !has('gui_running')
    call MouseLeftClick(2, 3)
    call MSWinMouseEvent(0x700, 6, 4, 1, 0, 0)
    let pos = getmousepos()
    call assert_equal(6, pos.screenrow)
    call assert_equal(4, pos.screencol)
  endif

  " test cells vs pixels
  if has('gui_running')
    let args = { }
    let args.row = 9
    let args.col = 5
    let args.move = 1
    let args.cell = 1
    call test_mswin_event("mouse", args)
    call feedkeys("\<Esc>", 'Lx!')
    let pos = getmousepos()
    call assert_equal(9, pos.screenrow)
    call assert_equal(5, pos.screencol)

    let args.cell = 0
    call test_mswin_event("mouse", args)
    call feedkeys("\<Esc>", 'Lx!')
    let pos = getmousepos()
    call assert_equal(1, pos.screenrow)
    call assert_equal(1, pos.screencol)

    unlet args
  endif

  " finish testing mouse movement
  set mousemev&

  " place the cursor using left click and release in normal mode
  call MouseLeftClick(2, 4)
  call MouseLeftRelease(2, 4)
  if has('gui_running')
    call feedkeys("\<Esc>", 'Lx!')
  endif
  call assert_equal([0, 2, 4, 0], getpos('.'))

  " select and yank a word
  let @" = ''
  call MouseLeftClick(1, 9)
  let args = #{button: 0, row: 1, col: 9, multiclick: 1, modifiers: 0}
  call test_mswin_event('mouse', args)
  call MouseLeftRelease(1, 9)
  call feedkeys("y", 'Lx!')
  call assert_equal('three', @")

  " create visual selection using right click
  let @" = ''

  call MouseLeftClick(2 ,6)
  call MouseLeftRelease(2, 6)
  call MouseRightClick(2, 13)
  call MouseRightRelease(2, 13)
  call feedkeys("y", 'Lx!')
  call assert_equal('five six', @")

  " paste using middle mouse button
  let @* = 'abc '
  call feedkeys('""', 'Lx!')
  call MouseMiddleClick(1, 9)
  call MouseMiddleRelease(1, 9)
  if has('gui_running')
    call feedkeys("\<Esc>", 'Lx!')
  endif
  call assert_equal(['one two abc three', 'four five six'], getline(1, '$'))

  " test mouse scrolling (aka touchpad scrolling.)
  %d _
  set scrolloff=0
  call setline(1, range(1, 100))

  " Scroll Down
  call MouseWheelDown(2, 1)
  call MouseWheelDown(2, 1)
  call MouseWheelDown(2, 1)
  call feedkeys("H", 'Lx!')
  call assert_equal(10, line('.'))

  " Scroll Up
  call MouseWheelUp(2, 1)
  call MouseWheelUp(2, 1)
  call feedkeys("H", 'Lx!')
  call assert_equal(4, line('.'))

  " Shift Scroll Down
  call MouseShiftWheelDown(2, 1)
  call feedkeys("H", 'Lx!')
  " should scroll from where it is (4) + visible buffer height - cmdheight
  let shift_scroll_height = line('w$') - line('w0') - &cmdheight
  call assert_equal(4 + shift_scroll_height, line('.'))

  " Shift Scroll Up
  call MouseShiftWheelUp(2, 1)
  call feedkeys("H", 'Lx!')
  call assert_equal(4, line('.'))

  if !has('gui_running')
    " Shift Scroll Down (using MOD)
    call MSWinMouseEvent(0x100, 2, 1, 0, 0, 0x04)
    call feedkeys("H", 'Lx!')
    " should scroll from where it is (4) + visible buffer height - cmdheight
    let shift_scroll_height = line('w$') - line('w0') - &cmdheight
    call assert_equal(4 + shift_scroll_height, line('.'))

    " Shift Scroll Up (using MOD)
    call MSWinMouseEvent(0x200, 2, 1, 0, 0, 0x04)
    call feedkeys("H", 'Lx!')
    call assert_equal(4, line('.'))
  endif

  set scrolloff&

  %d _
  set nowrap
  " make the buffer 500 wide.
  call setline(1, range(10)->join('')->repeat(50))
  " Scroll Right
  call MouseWheelRight(1, 5)
  call MouseWheelRight(1, 10)
  call MouseWheelRight(1, 15)
  call feedkeys('g0', 'Lx!')
  call assert_equal(19, col('.'))

  " Scroll Left
  call MouseWheelLeft(1, 15)
  call MouseWheelLeft(1, 10)
  call feedkeys('g0', 'Lx!')
  call assert_equal(7, col('.'))

  " Shift Scroll Right
  call MouseShiftWheelRight(1, 10)
  call feedkeys('g0', 'Lx!')
  " should scroll from where it is (7) + window width
  call assert_equal(7 + winwidth(0), col('.'))

  " Shift Scroll Left
  call MouseShiftWheelLeft(1, 50)
  call feedkeys('g0', 'Lx!')
  call assert_equal(7, col('.'))
  set wrap&

  %d _
  call setline(1, repeat([repeat('a', 60)], 10))

  " record various mouse events
  let mouseEventNames = [
        \ 'LeftMouse', 'LeftRelease', '2-LeftMouse', '3-LeftMouse',
        \ 'S-LeftMouse', 'A-LeftMouse', 'C-LeftMouse', 'MiddleMouse',
        \ 'MiddleRelease', '2-MiddleMouse', '3-MiddleMouse',
        \ 'S-MiddleMouse', 'A-MiddleMouse', 'C-MiddleMouse',
        \ 'RightMouse', 'RightRelease', '2-RightMouse',
        \ '3-RightMouse', 'S-RightMouse', 'A-RightMouse', 'C-RightMouse',
        \ ]
  let mouseEventCodes = map(copy(mouseEventNames), "'<' .. v:val .. '>'")
  let g:events = []
  for e in mouseEventCodes
    exe 'nnoremap ' .. e .. ' <Cmd>call add(g:events, "' ..
          \ substitute(e, '[<>]', '', 'g') .. '")<CR>'
  endfor

  " Test various mouse buttons
  "(0 - Left, 1 - Middle, 2 - Right,
  " 0x300 - MOUSE_X1/FROM_LEFT_3RD_BUTTON,
  " 0x400 - MOUSE_X2/FROM_LEFT_4TH_BUTTON)
  for button in [0, 1, 2, 0x300, 0x400]
    " Single click
    let args = #{button: button, row: 2, col: 5, multiclick: 0, modifiers: 0}
    call test_mswin_event('mouse', args)
    let args.button = 3
    call test_mswin_event('mouse', args)

    " Double Click
    let args.button = button
    call test_mswin_event('mouse', args)
    let args.multiclick = 1
    call test_mswin_event('mouse', args)
    let args.button = 3
    let args.multiclick = 0
    call test_mswin_event('mouse', args)

    " Triple Click
    let args.button = button
    call test_mswin_event('mouse', args)
    let args.multiclick = 1
    call test_mswin_event('mouse', args)
    call test_mswin_event('mouse', args)
    let args.button = 3
    let args.multiclick = 0
    call test_mswin_event('mouse', args)

    " Shift click
    let args = #{button: button, row: 3, col: 7, multiclick: 0, modifiers: 4}
    call test_mswin_event('mouse', args)
    let args.button = 3
    call test_mswin_event('mouse', args)

    " Alt click
    let args.button = button
    let args.modifiers = 8
    call test_mswin_event('mouse', args)
    let args.button = 3
    call test_mswin_event('mouse', args)

    " Ctrl click
    let args.button = button
    let args.modifiers = 16
    call test_mswin_event('mouse', args)
    let args.button = 3
    call test_mswin_event('mouse', args)

    call feedkeys("\<Esc>", 'Lx!')
  endfor

  if has('gui_running')
    call assert_equal(['LeftMouse', 'LeftRelease', 'LeftMouse',
	\ '2-LeftMouse', 'LeftMouse', '2-LeftMouse', '3-LeftMouse',
	\ 'S-LeftMouse', 'A-LeftMouse', 'C-LeftMouse', 'MiddleMouse',
	\ 'MiddleRelease', 'MiddleMouse', '2-MiddleMouse', 'MiddleMouse',
	\ '2-MiddleMouse', '3-MiddleMouse', 'S-MiddleMouse', 'A-MiddleMouse',
	\ 'C-MiddleMouse', 'RightMouse', 'RightRelease', 'RightMouse',
	\ '2-RightMouse', 'RightMouse', '2-RightMouse', '3-RightMouse',
	\ 'S-RightMouse', 'A-RightMouse', 'C-RightMouse'],
	\ g:events)
  else
    call assert_equal(['MiddleRelease', 'LeftMouse', '2-LeftMouse',
	\ '3-LeftMouse', 'S-LeftMouse', 'MiddleMouse', '2-MiddleMouse',
	\ '3-MiddleMouse', 'MiddleMouse', 'S-MiddleMouse', 'RightMouse',
	\ '2-RightMouse', '3-RightMouse'],
	\ g:events)
  endif

  for e in mouseEventCodes
    exe 'nunmap ' .. e
  endfor

  bw!
  call test_override('no_query_mouse', 0)
  set mousemodel&
endfunc


"  Test MS-Windows test_mswin_event error handling
func Test_mswin_event_error_handling()

  let args = #{button: 0xfff, row: 2, col: 4, move: 0, multiclick: 0, modifiers: 0}
  if !has('gui_running')
    call assert_fails("call test_mswin_event('mouse', args)",'E475:')
  endif
  let args = #{button: 0, row: 2, col: 4, move: 0, multiclick: 0, modifiers: 0}
  call assert_fails("call test_mswin_event('a1b2c3', args)", 'E475:')
  call assert_fails("call test_mswin_event(test_null_string(), {})", 'E475:')

  call assert_fails("call test_mswin_event([], args)", 'E1174:')
  call assert_fails("call test_mswin_event('abc', [])", 'E1206:')

  call assert_false(test_mswin_event('mouse', test_null_dict()))
  let args = #{row: 2, col: 4, multiclick: 0, modifiers: 0}
  call assert_false(test_mswin_event('mouse', args))
  let args = #{button: 0, col: 4, multiclick: 0, modifiers: 0}
  call assert_false(test_mswin_event('mouse', args))
  let args = #{button: 0, row: 2, multiclick: 0, modifiers: 0}
  call assert_false(test_mswin_event('mouse', args))
  let args = #{button: 0, row: 2, col: 4, modifiers: 0}
  call assert_false(test_mswin_event('mouse', args))
  let args = #{button: 0, row: 2, col: 4, multiclick: 0}
  call assert_false(test_mswin_event('mouse', args))

  call assert_false(test_mswin_event('key', test_null_dict()))
  call assert_fails("call test_mswin_event('key', [])", 'E1206:')
  call assert_fails("call test_mswin_event('key', {'event': 'keydown', 'keycode': 0x0})", 'E1291:')
  call assert_fails("call test_mswin_event('key', {'event': 'keydown', 'keycode': [15]})", 'E745:')
  call assert_fails("call test_mswin_event('key', {'event': 'keys', 'keycode': 0x41})", 'E475:')
  call assert_fails("call test_mswin_event('key', {'keycode': 0x41})", 'E417:')
  call assert_fails("call test_mswin_event('key', {'event': 'keydown'})", 'E1291:')

  call assert_fails("sandbox call test_mswin_event('key', {'event': 'keydown', 'keycode': 61 })", 'E48:')

  " flush out the typeahead buffer
  while getchar(0)
  endwhile
endfunc


" vim: shiftwidth=2 sts=2 expandtab
