" Tests for the sound feature

source check.vim
source shared.vim

CheckFeature sound

func PlayCallback(id, result)
  let g:playcallback_count += 1
  let g:id = a:id
  let g:result = a:result
endfunc

func Test_play_event()
  if has('win32')
    throw 'Skipped: Playing event with callback is not supported on Windows'
  endif
  let g:playcallback_count = 0
  let g:id = 0
  let event_name = 'bell'
  if has('osx')
      let event_name = 'Tink'
  endif
  let id = event_name->sound_playevent('PlayCallback')
  if id == 0
    throw 'Skipped: bell event not available'
  endif

  " Stop it quickly, avoid annoying the user.
  sleep 20m
  eval id->sound_stop()
  call WaitForAssert({-> assert_equal(id, g:id)})
  call assert_equal(1, g:result)  " sound was aborted
  call assert_equal(1, g:playcallback_count)
endfunc

func Test_play_silent()
  let fname = fnamemodify('silent.wav', '%p')
  let g:playcallback_count = 0

  " play without callback
  let id1 = sound_playfile(fname)
  if id1 == 0
    throw 'Skipped: playing a sound is not working'
  endif

  " play until the end
  let id2 = fname->sound_playfile('PlayCallback')
  call assert_true(id2 > 0)
  call WaitForAssert({-> assert_equal(id2, g:id)})
  call assert_equal(0, g:result)
  call assert_equal(1, g:playcallback_count)

  let id2 = sound_playfile(fname, 'PlayCallback')
  call assert_true(id2 > 0)
  sleep 20m
  call sound_clear()
  call WaitForAssert({-> assert_equal(id2, g:id)})
  call assert_equal(1, g:result)  " sound was aborted
  call assert_equal(2, g:playcallback_count)

  " Play 2 sounds almost at the same time to exercise
  " code with multiple callbacks in the callback list.
  call sound_playfile(fname, 'PlayCallback')
  call sound_playfile(fname, 'PlayCallback')
  call WaitForAssert({-> assert_equal(4, g:playcallback_count)})

  " recursive use was causing a crash
  func PlayAgain(id, fname)
    let g:id = a:id
    let g:id_again = sound_playfile(a:fname)
  endfunc

  let id3 = sound_playfile(fname, {id, res -> PlayAgain(id, fname)})
  call assert_true(id3 > 0)
  sleep 50m
  call sound_clear()
  call WaitForAssert({-> assert_true(g:id > 0)})
endfunc

func Test_play_event_error()
  " FIXME: sound_playevent() doesn't return 0 in case of error on Windows.
  if !has('win32')
    call assert_equal(0, sound_playevent(''))
    call assert_equal(0, sound_playevent(test_null_string()))
    call assert_equal(0, sound_playevent('doesnotexist'))
    call assert_equal(0, sound_playevent('doesnotexist', 'doesnotexist'))
    call assert_equal(0, sound_playevent(test_null_string(), test_null_string()))
    call assert_equal(0, sound_playevent(test_null_string(), test_null_function()))
  endif

  call assert_equal(0, sound_playfile(''))
  call assert_equal(0, sound_playfile(test_null_string()))
  call assert_equal(0, sound_playfile('doesnotexist'))
  call assert_equal(0, sound_playfile('doesnotexist', 'doesnotexist'))
  call assert_equal(0, sound_playfile(test_null_string(), test_null_string()))
  call assert_equal(0, sound_playfile(test_null_string(), test_null_function()))
endfunc

" vim: shiftwidth=2 sts=2 expandtab
