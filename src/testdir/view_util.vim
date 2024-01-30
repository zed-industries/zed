" Functions about view shared by several tests

" Only load this script once.
if exists('*Screenline')
  finish
endif

" Get line "lnum" as displayed on the screen.
" Trailing white space is trimmed.
func Screenline(lnum)
  let chars = []
  for c in range(1, winwidth(0))
    call add(chars, nr2char(screenchar(a:lnum, c)))
  endfor
  let line = join(chars, '')
  return matchstr(line, '^.\{-}\ze\s*$')
endfunc

" Get text on the screen, including composing characters.
" ScreenLines(lnum, width) or
" ScreenLines([start, end], width)
func ScreenLines(lnum, width) abort
  redraw!
  if type(a:lnum) == v:t_list
    let start = a:lnum[0]
    let end = a:lnum[1]
  else
    let start = a:lnum
    let end = a:lnum
  endif
  let lines = []
  for l in range(start, end)
    let lines += [join(map(range(1, a:width), 'screenstring(l, v:val)'), '')]
  endfor
  return lines
endfunc

func ScreenAttrs(lnum, width) abort
  redraw!
  if type(a:lnum) == v:t_list
    let start = a:lnum[0]
    let end = a:lnum[1]
  else
    let start = a:lnum
    let end = a:lnum
  endif
  let attrs = []
  for l in range(start, end)
    let attrs += [map(range(1, a:width), 'screenattr(l, v:val)')]
  endfor
  return attrs
endfunc

" Create a new window with the requested size and fix it.
func NewWindow(height, width) abort
  exe a:height . 'new'
  exe a:width . 'vsp'
  set winfixwidth winfixheight
  redraw!
endfunc

func CloseWindow() abort
  bw!
  redraw!
endfunc


" When using RunVimInTerminal() we expect modifyOtherKeys level 2 to be enabled
" automatically.  The key + modifier Escape codes must then use the
" modifyOtherKeys encoding.  They are recognized anyway, thus it's safer to use
" than the raw code.

" Return the modifyOtherKeys level 2 encoding for "key" with "modifier"
" (number value, e.g. CTRL is 5).
func GetEscCodeCSI27(key, modifier)
  let key = printf("%d", char2nr(a:key))
  let mod = printf("%d", a:modifier)
  return "\<Esc>[27;" .. mod .. ';' .. key .. '~'
endfunc

" Return the modifyOtherKeys level 2 encoding for "key" with "modifier"
" (character value, e.g. CTRL is "C").
func GetEscCodeWithModifier(modifier, key)
  let modifier = get({'C': 5}, a:modifier, '')
  if modifier == ''
    echoerr 'Unknown modifier: ' .. a:modifier
  endif
  return GetEscCodeCSI27(a:key, modifier)
endfunc

" Return the kitty keyboard protocol encoding for "key" with "modifier"
" (number value, e.g. CTRL is 5).
func GetEscCodeCSIu(key, modifier)
  let key = printf("%d", char2nr(a:key))
  let mod = printf("%d", a:modifier)
  return "\<Esc>[" .. key .. ';' .. mod .. 'u'
endfunc

" Return the kitty keyboard protocol encoding for a function key:
" CSI {key}
" CSS 1;{modifier} {key}
func GetEscCodeFunckey(key, modifier)
  if a:modifier == 0
    return "\<Esc>[" .. a:key
  endif

  let mod = printf("%d", a:modifier)
  return "\<Esc>[1;".. mod .. a:key
endfunc

" Return the kitty keyboard protocol encoding for "key" without a modifier.
" Used for the Escape key.
func GetEscCodeCSIuWithoutModifier(key)
  let key = printf("%d", char2nr(a:key))
  return "\<Esc>[" .. key .. 'u'
endfunc

