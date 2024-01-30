" Helper functions for generating mouse events

" xterm2 and sgr always work, urxvt is optional.
let g:Ttymouse_values = ['xterm2', 'sgr']
if has('mouse_urxvt')
  call add(g:Ttymouse_values, 'urxvt')
endif

" dec doesn't support all the functionality
if has('mouse_dec')
  let g:Ttymouse_dec = ['dec']
else
  let g:Ttymouse_dec = []
endif

" netterm only supports left click
if has('mouse_netterm')
  let g:Ttymouse_netterm = ['netterm']
else
  let g:Ttymouse_netterm = []
endif

" Vim Mouse Codes.
" Used by the GUI and by MS-Windows Consoles.
" Keep these in sync with vim.h
let s:MOUSE_CODE = {
  \ 'BTN_LEFT'    :  0x00,
  \ 'BTN_MIDDLE'  :  0x01,
  \ 'BTN_RIGHT'   :  0x02,
  \ 'BTN_RELEASE' :  0x03,
  \ 'BTN_X1'      : 0x300,
  \ 'BTN_X2'      : 0x400,
  \ 'SCRL_DOWN'   : 0x100,
  \ 'SCRL_UP'     : 0x200,
  \ 'SCRL_LEFT'   : 0x500,
  \ 'SCRL_RIGHT'  : 0x600,
  \ 'MOVE'        : 0x700,
  \ 'MOD_SHIFT'   :  0x04,
  \ 'MOD_ALT'     :  0x08,
  \ 'MOD_CTRL'    :  0x10,
  \ }


" Helper function to emit a terminal escape code.
func TerminalEscapeCode(code, row, col, m)
  if &ttymouse ==# 'xterm2'
    " need to use byte encoding here.
    let str = list2str([a:code + 0x20, a:col + 0x20, a:row + 0x20])
    if has('iconv')
      let bytes = str->iconv('utf-8', 'latin1')
    else
      " Hopefully the numbers are not too big.
      let bytes = str
    endif
    return "\<Esc>[M" .. bytes
  elseif &ttymouse ==# 'sgr'
    return printf("\<Esc>[<%d;%d;%d%s", a:code, a:col, a:row, a:m)
  elseif &ttymouse ==# 'urxvt'
    return printf("\<Esc>[%d;%d;%dM", a:code + 0x20, a:col, a:row)
  endif
endfunc

func DecEscapeCode(code, down, row, col)
    return printf("\<Esc>[%d;%d;%d;%d&w", a:code, a:down, a:row, a:col)
endfunc

func NettermEscapeCode(row, col)
    return printf("\<Esc>}%d,%d\r", a:row, a:col)
endfunc

" Send low level mouse event to MS-Windows consoles or GUI
func MSWinMouseEvent(button, row, col, move, multiclick, modifiers)
    let args = { }
    let args.button = a:button
    " Scroll directions are inverted in the GUI, no idea why.
    if has('gui_running')
      if a:button == s:MOUSE_CODE.SCRL_UP
        let args.button = s:MOUSE_CODE.SCRL_DOWN
      elseif a:button == s:MOUSE_CODE.SCRL_DOWN
        let args.button = s:MOUSE_CODE.SCRL_UP
      elseif a:button == s:MOUSE_CODE.SCRL_LEFT
        let args.button = s:MOUSE_CODE.SCRL_RIGHT
      elseif a:button == s:MOUSE_CODE.SCRL_RIGHT
        let args.button = s:MOUSE_CODE.SCRL_LEFT
      endif
    endif
    let args.row = a:row
    let args.col = a:col
    let args.move = a:move
    let args.multiclick = a:multiclick
    let args.modifiers = a:modifiers
    call test_mswin_event("mouse", args)
    unlet args
endfunc

func MouseLeftClickCode(row, col)
  if &ttymouse ==# 'dec'
    return DecEscapeCode(2, 4, a:row, a:col)
  elseif &ttymouse ==# 'netterm'
    return NettermEscapeCode(a:row, a:col)
  else
    return TerminalEscapeCode(0, a:row, a:col, 'M')
  endif
endfunc

func MouseLeftClick(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_LEFT, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseLeftClickCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseMiddleClickCode(row, col)
  if &ttymouse ==# 'dec'
    return DecEscapeCode(4, 2, a:row, a:col)
  else
    return TerminalEscapeCode(1, a:row, a:col, 'M')
  endif
endfunc

func MouseMiddleClick(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_MIDDLE, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseMiddleClickCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseRightClickCode(row, col)
  if &ttymouse ==# 'dec'
    return DecEscapeCode(6, 1, a:row, a:col)
  else
    return TerminalEscapeCode(2, a:row, a:col, 'M')
  endif
endfunc

func MouseRightClick(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_RIGHT, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseRightClickCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseCtrlLeftClickCode(row, col)
  let ctrl = 0x10
  return TerminalEscapeCode(0 + ctrl, a:row, a:col, 'M')
endfunc

func MouseCtrlLeftClick(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_LEFT, a:row, a:col, 0, 0,
                                                         \ s:MOUSE_CODE.MOD_CTRL)
  else
    call feedkeys(MouseCtrlLeftClickCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseCtrlRightClickCode(row, col)
  let ctrl = 0x10
  return TerminalEscapeCode(2 + ctrl, a:row, a:col, 'M')
endfunc

func MouseCtrlRightClick(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_RIGHT, a:row, a:col, 0, 0,
                                                       \ s:MOUSE_CODE.MOD_CTRL)
  else
    call feedkeys(MouseCtrlRightClickCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseAltLeftClickCode(row, col)
  let alt = 0x8
  return TerminalEscapeCode(0 + alt, a:row, a:col, 'M')
endfunc

func MouseAltLeftClick(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_LEFT, a:row, a:col, 0, 0,
                                                       \ s:MOUSE_CODE.MOD_ALT)
  else
    call feedkeys(MouseAltLeftClickCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseAltRightClickCode(row, col)
  let alt = 0x8
  return TerminalEscapeCode(2 + alt, a:row, a:col, 'M')
endfunc

func MouseAltRightClick(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_RIGHT, a:row, a:col, 0, 0,
                                                       \ s:MOUSE_CODE.MOD_ALT)
  else
    call feedkeys(MouseAltRightClickCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseLeftReleaseCode(row, col)
  if &ttymouse ==# 'dec'
    return DecEscapeCode(3, 0, a:row, a:col)
  elseif &ttymouse ==# 'netterm'
    return ''
  else
    return TerminalEscapeCode(3, a:row, a:col, 'm')
  endif
endfunc

func MouseLeftRelease(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_RELEASE, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseLeftReleaseCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseMiddleReleaseCode(row, col)
  if &ttymouse ==# 'dec'
    return DecEscapeCode(5, 0, a:row, a:col)
  else
    return TerminalEscapeCode(3, a:row, a:col, 'm')
  endif
endfunc

func MouseMiddleRelease(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_RELEASE, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseMiddleReleaseCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseRightReleaseCode(row, col)
  if &ttymouse ==# 'dec'
    return DecEscapeCode(7, 0, a:row, a:col)
  else
    return TerminalEscapeCode(3, a:row, a:col, 'm')
  endif
endfunc

func MouseRightRelease(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_RELEASE, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseRightReleaseCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseLeftDragCode(row, col)
  if &ttymouse ==# 'dec'
    return DecEscapeCode(1, 4, a:row, a:col)
  else
    return TerminalEscapeCode(0x20, a:row, a:col, 'M')
  endif
endfunc

func MouseLeftDrag(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.BTN_LEFT, a:row, a:col, 1, 0, 0)
  else
    call feedkeys(MouseLeftDragCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseWheelUpCode(row, col)
  return TerminalEscapeCode(0x40, a:row, a:col, 'M')
endfunc

func MouseWheelUp(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.SCRL_UP, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseWheelUpCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseWheelDownCode(row, col)
  return TerminalEscapeCode(0x41, a:row, a:col, 'M')
endfunc

func MouseWheelDown(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.SCRL_DOWN, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseWheelDownCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseWheelLeftCode(row, col)
  return TerminalEscapeCode(0x42, a:row, a:col, 'M')
endfunc

func MouseWheelLeft(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.SCRL_LEFT, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseWheelLeftCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseWheelRightCode(row, col)
  return TerminalEscapeCode(0x43, a:row, a:col, 'M')
endfunc

func MouseWheelRight(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.SCRL_RIGHT, a:row, a:col, 0, 0, 0)
  else
    call feedkeys(MouseWheelRightCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseShiftWheelUpCode(row, col)
  " todo feed shift mod.
  return TerminalEscapeCode(0x40, a:row, a:col, 'M')
endfunc

func MouseShiftWheelUp(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.SCRL_UP, a:row, a:col, 0, 0,
                                                      \ s:MOUSE_CODE.MOD_SHIFT)
  else
    call feedkeys(MouseShiftWheelUpCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseShiftWheelDownCode(row, col)
  " todo feed shift mod.
  return TerminalEscapeCode(0x41, a:row, a:col, 'M')
endfunc

func MouseShiftWheelDown(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.SCRL_DOWN, a:row, a:col, 0, 0,
                                                      \ s:MOUSE_CODE.MOD_SHIFT)
  else
    call feedkeys(MouseShiftWheelDownCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseShiftWheelLeftCode(row, col)
  " todo feed shift mod.
  return TerminalEscapeCode(0x42, a:row, a:col, 'M')
endfunc

func MouseShiftWheelLeft(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.SCRL_LEFT, a:row, a:col, 0, 0,
                                                      \ s:MOUSE_CODE.MOD_SHIFT)
  else
    call feedkeys(MouseShiftWheelLeftCode(a:row, a:col), 'Lx!')
  endif
endfunc

func MouseShiftWheelRightCode(row, col)
	" todo feed shift mod.
  return TerminalEscapeCode(0x43, a:row, a:col, 'M')
endfunc

func MouseShiftWheelRight(row, col)
  if has('win32')
    call MSWinMouseEvent(s:MOUSE_CODE.SCRL_RIGHT, a:row, a:col, 0, 0,
                                                      \ s:MOUSE_CODE.MOD_SHIFT)
  else
    call feedkeys(MouseShiftWheelRightCode(a:row, a:col), 'Lx!')
  endif
endfunc

" vim: shiftwidth=2 sts=2 expandtab
