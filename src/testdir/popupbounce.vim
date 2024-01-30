" Use this script to measure the redrawing performance when a popup is being
" displayed.  Usage with gcc:
"    cd src
"    # Edit Makefile to uncomment PROFILE_CFLAGS and PROFILE_LIBS
"    make reconfig
"    ./vim --clean -S testdir/popupbounce.vim main.c
"    gprof vim gmon.out | vim -

" using line continuation
set nocp

" don't switch screens when quitting, so we can read the frames/sec
set t_te=

let winid = popup_create(['line1', 'line2', 'line3', 'line4'], {
	      \   'line' : 1,
	      \   'col' : 1,
	      \   'zindex' : 101,
	      \ })
redraw

let start = reltime()
let framecount = 0

let line = 1.0
let col = 1
let downwards = 1
let col_inc = 1
let initial_speed = 0.2
let speed = initial_speed
let accel = 1.1
let time = 0.1

let countdown = 0

while 1
  if downwards
    let speed += time * accel
    let line += speed
  else
    let speed -= time * accel
    let line -= speed
  endif

  if line + 3 >= &lines
    let downwards = 0
    let speed = speed * 0.8
    let line = &lines - 3
  endif
  if !downwards && speed < 1.0
    let downwards = 1
    let speed = initial_speed
    if line + 4 > &lines && countdown == 0
      let countdown = 50
    endif
  endif

  let col += col_inc
  if col + 4 >= &columns
    let col_inc = -1
  elseif col <= 1
    let col_inc = 1
  endif

  call popup_move(winid, {'line': float2nr(line), 'col': col})
  redraw
  let framecount += 1
  if countdown > 0
    let countdown -= 1
    if countdown == 0
      break
    endif
  endif

endwhile

let elapsed = reltimefloat(reltime(start))
echomsg framecount .. ' frames in ' .. string(elapsed) .. ' seconds, ' .. string(framecount / elapsed) .. ' frames/sec'

qa
