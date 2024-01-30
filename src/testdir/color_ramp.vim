" Script to generate a file that shows al 256 xterm colors

new
let lnum = 1

" | in original color pair to see white background.
let trail_bar = "\033[m|"

" ANSI colors
call setline(lnum, 'ANSI background')
let lnum += 1

let s = ''
for nr in range(0, 7)
  let s .= "\033[4" . nr . "m    "
endfor
for nr in range(8, 15)
  let s .= "\033[10" . (nr - 8) . "m    "
endfor
let s .= trail_bar

call setline(lnum, s)
let lnum += 1

" ANSI text colors
call setline(lnum, 'ANSI text')
let lnum += 1

let s = ''
for nr in range(0, 7)
  let s .= "\033[0;3" . nr . "mxxxx"
endfor
for nr in range(8, 15)
  let s .= "\033[0;9" . (nr - 8) . "mxxxx"
endfor
let s .= trail_bar

call setline(lnum, s)
let lnum += 1

" ANSI with bold text
call setline(lnum, 'ANSI bold text')
let lnum += 1

let s = ''
for nr in range(0, 7)
  let s .= "\033[1;3" . nr . "mxxxx"
endfor
for nr in range(8, 15)
  let s .= "\033[1;9" . (nr - 8) . "mxxxx"
endfor
let s .= trail_bar

call setline(lnum, s)
let lnum += 1

" 6 x 6 x 6 color cube
call setline(lnum, 'color cube')
let lnum += 1

for high in range(0, 5)
  let s = ''
  for low in range(0, 35)
    let nr = low + high * 36
    let s .= "\033[48;5;" . (nr + 16) . "m  "
  endfor
  let s .= trail_bar
  call setline(lnum + high, s)
endfor
let lnum += 6

" 24 shades of grey
call setline(lnum, 'grey ramp')
let lnum += 1

let s = ''
for nr in range(0, 23)
    let s .= "\033[48;5;" . (nr + 232) . "m   "
endfor
let s .= trail_bar
call setline(lnum, s)

set binary
write! <sfile>:h/color_ramp.txt
quit
