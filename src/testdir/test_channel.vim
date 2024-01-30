" Test for channel and job functions.

" When +channel is supported then +job is too, so we don't check for that.
source check.vim
CheckFeature channel

source shared.vim
source screendump.vim
source view_util.vim

let s:python = PythonProg()
if s:python == ''
  " Can't run this test without Python.
  throw 'Skipped: Python command missing'
endif

" Uncomment the next line to see what happens. Output is in
" src/testdir/channellog.
" Add ch_log() calls where you want to see what happens.
" call ch_logfile('channellog', 'w')

func SetUp()
  if g:testfunc =~ '_ipv6()$'
    let s:localhost = '[::1]:'
    let s:testscript = 'test_channel_6.py'
  elseif g:testfunc =~ '_unix()$'
    let s:localhost = 'unix:Xtestsocket'
    let s:testscript = 'test_channel_unix.py'
  else
    let s:localhost = 'localhost:'
    let s:testscript = 'test_channel.py'
  endif
  let s:chopt = {}
  call ch_log(g:testfunc)

  " Most tests use job_start(), which can be flaky
  let g:test_is_flaky = 1
endfunc

" Run "testfunc" after starting the server and stop the server afterwards.
func s:run_server(testfunc, ...)
  call RunServer(s:testscript, a:testfunc, a:000)
endfunc

" Returns the address of the test server.
func s:address(port)
  if s:localhost =~ '^unix:'
    return s:localhost
  else
    return s:localhost . a:port
  end
endfunc

" Return a list of open files.
" Can be used to make sure no resources leaked.
" Returns an empty list on systems where this is not supported.
func s:get_resources()
  let pid = getpid()

  if executable('lsof')
    return systemlist('lsof -p ' . pid . ' | awk ''$4~/^[0-9]*[rwu]$/&&$5=="REG"{print$NF}''')
  elseif isdirectory('/proc/' . pid . '/fd/')
    return systemlist('readlink /proc/' . pid . '/fd/* | grep -v ''^/dev/''')
  else
    return []
  endif
endfunc

let g:Ch_responseMsg = ''
func Ch_requestHandler(handle, msg)
  let g:Ch_responseHandle = a:handle
  let g:Ch_responseMsg = a:msg
endfunc

func Ch_communicate(port)
  " Avoid dropping messages, since we don't use a callback here.
  let s:chopt.drop = 'never'
  " Also add the noblock flag to try it out.
  let s:chopt.noblock = 1
  let handle = ch_open(s:address(a:port), s:chopt)
  if ch_status(handle) == "fail"
    call assert_report("Can't open channel")
    return
  endif

  " check that getjob without a job is handled correctly
  call assert_equal('no process', string(ch_getjob(handle)))

  let dict = handle->ch_info()
  call assert_true(dict.id != 0)
  call assert_equal('open', dict.status)
  if has_key(dict, 'port')
    " Channels using Unix sockets have no 'port' entry.
    call assert_equal(a:port, string(dict.port))
  end
  call assert_equal('open', dict.sock_status)
  call assert_equal('socket', dict.sock_io)

  " Simple string request and reply.
  call assert_equal('got it', ch_evalexpr(handle, 'hello!'))

  " Malformed command should be ignored.
  call assert_equal('ok', ch_evalexpr(handle, 'malformed1'))
  call assert_equal('ok', ch_evalexpr(handle, 'malformed2'))
  call assert_equal('ok', ch_evalexpr(handle, 'malformed3'))

  " split command should work
  call assert_equal('ok', ch_evalexpr(handle, 'split'))
  call WaitFor('exists("g:split")')
  call assert_equal(123, g:split)

  " string with ][ should work
  call assert_equal('this][that', ch_evalexpr(handle, 'echo this][that'))

  " nothing to read now
  call assert_equal(0, ch_canread(handle))

  " sending three messages quickly then reading should work
  for i in range(3)
    call ch_sendexpr(handle, 'echo hello ' . i)
  endfor
  call assert_equal('hello 0', ch_read(handle)[1])
  call assert_equal('hello 1', ch_read(handle)[1])
  call assert_equal('hello 2', ch_read(handle)[1])

  " Request that triggers sending two ex commands.  These will usually be
  " handled before getting the response, but it's not guaranteed, thus wait a
  " tiny bit for the commands to get executed.
  call assert_equal('ok', ch_evalexpr(handle, 'make change'))
  call WaitForAssert({-> assert_equal("added2", getline("$"))})
  call assert_equal('added1', getline(line('$') - 1))

  " Request command "echoerr 'this is an error'".
  " This will throw an exception, catch it here.
  let caught = 'no'
  try
    call assert_equal('ok', ch_evalexpr(handle, 'echoerr'))
  catch /this is an error/
    let caught = 'yes'
  endtry
  if caught != 'yes'
    call assert_report("Expected exception from error message")
  endif

  " Request command "foo bar", which fails silently.
  call assert_equal('ok', ch_evalexpr(handle, 'bad command'))
  call WaitForAssert({-> assert_match("E492:.*foo bar", v:errmsg)})

  call assert_equal('ok', ch_evalexpr(handle, 'do normal', {'timeout': 100}))
  call WaitForAssert({-> assert_equal('added more', getline('$'))})

  " Send a request with a specific handler.
  call ch_sendexpr(handle, 'hello!', {'callback': 'Ch_requestHandler'})
  call WaitFor('exists("g:Ch_responseHandle")')
  if !exists('g:Ch_responseHandle')
    call assert_report('g:Ch_responseHandle was not set')
  else
    call assert_equal(handle, g:Ch_responseHandle)
    unlet g:Ch_responseHandle
  endif
  call assert_equal('got it', g:Ch_responseMsg)

  let g:Ch_responseMsg = ''
  call ch_sendexpr(handle, 'hello!', {'callback': function('Ch_requestHandler')})
  call WaitFor('exists("g:Ch_responseHandle")')
  if !exists('g:Ch_responseHandle')
    call assert_report('g:Ch_responseHandle was not set')
  else
    call assert_equal(handle, g:Ch_responseHandle)
    unlet g:Ch_responseHandle
  endif
  call assert_equal('got it', g:Ch_responseMsg)

  " Using lambda.
  let g:Ch_responseMsg = ''
  call ch_sendexpr(handle, 'hello!', {'callback': {a, b -> Ch_requestHandler(a, b)}})
  call WaitFor('exists("g:Ch_responseHandle")')
  if !exists('g:Ch_responseHandle')
    call assert_report('g:Ch_responseHandle was not set')
  else
    call assert_equal(handle, g:Ch_responseHandle)
    unlet g:Ch_responseHandle
  endif
  call assert_equal('got it', g:Ch_responseMsg)

  " Collect garbage, tests that our handle isn't collected.
  call test_garbagecollect_now()

  " check setting options (without testing the effect)
  eval handle->ch_setoptions({'callback': 's:NotUsed'})
  call ch_setoptions(handle, {'timeout': 1111})
  call ch_setoptions(handle, {'mode': 'json'})
  call assert_fails("call ch_setoptions(handle, {'waittime': 111})", 'E475:')
  call ch_setoptions(handle, {'callback': ''})
  call ch_setoptions(handle, {'drop': 'never'})
  call ch_setoptions(handle, {'drop': 'auto'})
  call assert_fails("call ch_setoptions(handle, {'drop': 'bad'})", 'E475:')
  call assert_equal(0, ch_setoptions(handle, test_null_dict()))
  call assert_equal(0, ch_setoptions(test_null_channel(), {'drop' : 'never'}))

  " Send an eval request that works.
  call assert_equal('ok', ch_evalexpr(handle, 'eval-works'))
  sleep 10m
  call assert_equal([-1, 'foo123'], ch_evalexpr(handle, 'eval-result'))

  " Send an eval request with special characters.
  call assert_equal('ok', ch_evalexpr(handle, 'eval-special'))
  sleep 10m
  call assert_equal([-2, "foo\x7f\x10\x01bar"], ch_evalexpr(handle, 'eval-result'))

  " Send an eval request to get a line with special characters.
  call setline(3, "a\nb\<CR>c\x01d\x7fe")
  call assert_equal('ok', ch_evalexpr(handle, 'eval-getline'))
  sleep 10m
  call assert_equal([-3, "a\nb\<CR>c\x01d\x7fe"], ch_evalexpr(handle, 'eval-result'))

  " Send an eval request that fails.
  call assert_equal('ok', ch_evalexpr(handle, 'eval-fails'))
  sleep 10m
  call assert_equal([-4, 'ERROR'], ch_evalexpr(handle, 'eval-result'))

  " Send an eval request that works but can't be encoded.
  call assert_equal('ok', ch_evalexpr(handle, 'eval-error'))
  sleep 10m
  call assert_equal([-5, 'ERROR'], ch_evalexpr(handle, 'eval-result'))

  " Send a bad eval request. There will be no response.
  call assert_equal('ok', ch_evalexpr(handle, 'eval-bad'))
  sleep 10m
  call assert_equal([-5, 'ERROR'], ch_evalexpr(handle, 'eval-result'))

  " Send an expr request
  call assert_equal('ok', ch_evalexpr(handle, 'an expr'))
  call WaitForAssert({-> assert_equal('three', getline('$'))})
  call assert_equal('one', getline(line('$') - 2))
  call assert_equal('two', getline(line('$') - 1))

  " Request a redraw, we don't check for the effect.
  call assert_equal('ok', ch_evalexpr(handle, 'redraw'))
  call assert_equal('ok', ch_evalexpr(handle, 'redraw!'))

  call assert_equal('ok', ch_evalexpr(handle, 'empty-request'))

  " Reading while there is nothing available.
  call assert_equal(v:none, ch_read(handle, {'timeout': 0}))
  if exists('*reltimefloat')
    let start = reltime()
    call assert_equal(v:none, ch_read(handle, {'timeout': 333}))
    let elapsed = reltime(start)
    call assert_inrange(0.3, 0.6, reltimefloat(reltime(start)))
  endif

  " Send without waiting for a response, then wait for a response.
  call ch_sendexpr(handle, 'wait a bit')
  let resp = ch_read(handle)
  call assert_equal(type([]), type(resp))
  call assert_equal(type(11), type(resp[0]))
  call assert_equal('waited', resp[1])

  " make the server quit, can't check if this works, should not hang.
  call ch_sendexpr(handle, '!quit!')
endfunc

func Test_communicate()
  call s:run_server('Ch_communicate')
endfunc

func Test_communicate_ipv6()
  CheckIPv6
  call Test_communicate()
endfunc

func Test_communicate_unix()
  CheckUnix
  call Test_communicate()
  call delete('Xtestsocket')
endfunc


" Test that we can open two channels.
func Ch_two_channels(port)
  let handle = ch_open(s:address(a:port), s:chopt)
  call assert_equal(v:t_channel, type(handle))
  if handle->ch_status() == "fail"
    call assert_report("Can't open channel")
    return
  endif

  call assert_equal('got it', ch_evalexpr(handle, 'hello!'))

  let newhandle = ch_open(s:address(a:port), s:chopt)
  if ch_status(newhandle) == "fail"
    call assert_report("Can't open second channel")
    return
  endif
  call assert_equal('got it', ch_evalexpr(newhandle, 'hello!'))
  call assert_equal('got it', ch_evalexpr(handle, 'hello!'))

  call ch_close(handle)
  call assert_equal('got it', ch_evalexpr(newhandle, 'hello!'))

  call ch_close(newhandle)
  call assert_fails("call ch_close(newhandle)", 'E906:')
endfunc

func Test_two_channels()
  eval 'Test_two_channels()'->ch_log()
  call s:run_server('Ch_two_channels')
endfunc

func Test_two_channels_ipv6()
  CheckIPv6
  call Test_two_channels()
endfunc

func Test_two_channels_unix()
  CheckUnix
  call Test_two_channels()
  call delete('Xtestsocket')
endfunc

" Test that a server crash is handled gracefully.
func Ch_server_crash(port)
  let handle = ch_open(s:address(a:port), s:chopt)
  if ch_status(handle) == "fail"
    call assert_report("Can't open channel")
    return
  endif

  call ch_evalexpr(handle, '!crash!')

  sleep 10m
endfunc

func Test_server_crash()
  call s:run_server('Ch_server_crash')
endfunc

func Test_server_crash_ipv6()
  CheckIPv6
  call Test_server_crash()
endfunc

func Test_server_crash_unix()
  CheckUnix
  call Test_server_crash()
  call delete('Xtestsocket')
endfunc

"""""""""

func Ch_handler(chan, msg)
  call ch_log('Ch_handler()')
  unlet g:Ch_reply
  let g:Ch_reply = a:msg
endfunc

func Ch_channel_handler(port)
  let handle = ch_open(s:address(a:port), s:chopt)
  if ch_status(handle) == "fail"
    call assert_report("Can't open channel")
    return
  endif

  " Test that it works while waiting on a numbered message.
  call assert_equal('ok', ch_evalexpr(handle, 'call me'))
  call WaitForAssert({-> assert_equal('we called you', g:Ch_reply)})

  " Test that it works while not waiting on a numbered message.
  call ch_sendexpr(handle, 'call me again')
  call WaitForAssert({-> assert_equal('we did call you', g:Ch_reply)})
endfunc

func Test_channel_handler()
  let g:Ch_reply = ""
  let s:chopt.callback = 'Ch_handler'
  call s:run_server('Ch_channel_handler')
  let g:Ch_reply = ""
  let s:chopt.callback = function('Ch_handler')
  call s:run_server('Ch_channel_handler')
endfunc

func Test_channel_handler_ipv6()
  CheckIPv6
  call Test_channel_handler()
endfunc

func Test_channel_handler_unix()
  CheckUnix
  call Test_channel_handler()
  call delete('Xtestsocket')
endfunc

"""""""""

let g:Ch_reply = ''
func Ch_zeroHandler(chan, msg)
  unlet g:Ch_reply
  let g:Ch_reply = a:msg
endfunc

let g:Ch_zero_reply = ''
func Ch_oneHandler(chan, msg)
  unlet g:Ch_zero_reply
  let g:Ch_zero_reply = a:msg
endfunc

func Ch_channel_zero(port)
  let handle = (s:address(a:port))->ch_open(s:chopt)
  if ch_status(handle) == "fail"
    call assert_report("Can't open channel")
    return
  endif

  " Check that eval works.
  call assert_equal('got it', ch_evalexpr(handle, 'hello!'))

  " Check that eval works if a zero id message is sent back.
  let g:Ch_reply = ''
  call assert_equal('sent zero', ch_evalexpr(handle, 'send zero'))
  if s:has_handler
    call WaitForAssert({-> assert_equal('zero index', g:Ch_reply)})
  else
    sleep 20m
    call assert_equal('', g:Ch_reply)
  endif

  " Check that handler works if a zero id message is sent back.
  let g:Ch_reply = ''
  let g:Ch_zero_reply = ''
  call ch_sendexpr(handle, 'send zero', {'callback': 'Ch_oneHandler'})
  call WaitForAssert({-> assert_equal('sent zero', g:Ch_zero_reply)})
  if s:has_handler
    call assert_equal('zero index', g:Ch_reply)
  else
    call assert_equal('', g:Ch_reply)
  endif
endfunc

func Test_zero_reply()
  " Run with channel handler
  let s:has_handler = 1
  let s:chopt.callback = 'Ch_zeroHandler'
  call s:run_server('Ch_channel_zero')
  unlet s:chopt.callback

  " Run without channel handler
  let s:has_handler = 0
  call s:run_server('Ch_channel_zero')
endfunc

func Test_zero_reply_ipv6()
  CheckIPv6
  call Test_zero_reply()
endfunc

func Test_zero_reply_unix()
  CheckUnix
  call Test_zero_reply()
  call delete('Xtestsocket')
endfunc


"""""""""

let g:Ch_reply1 = ""
func Ch_handleRaw1(chan, msg)
  unlet g:Ch_reply1
  let g:Ch_reply1 = a:msg
endfunc

let g:Ch_reply2 = ""
func Ch_handleRaw2(chan, msg)
  unlet g:Ch_reply2
  let g:Ch_reply2 = a:msg
endfunc

let g:Ch_reply3 = ""
func Ch_handleRaw3(chan, msg)
  unlet g:Ch_reply3
  let g:Ch_reply3 = a:msg
endfunc

func Ch_raw_one_time_callback(port)
  let handle = ch_open(s:address(a:port), s:chopt)
  if ch_status(handle) == "fail"
    call assert_report("Can't open channel")
    return
  endif
  call ch_setoptions(handle, {'mode': 'raw'})

  " The messages are sent raw, we do our own JSON strings here.
  call ch_sendraw(handle, "[1, \"hello!\"]\n", {'callback': 'Ch_handleRaw1'})
  call WaitForAssert({-> assert_equal("[1, \"got it\"]", g:Ch_reply1)})
  call ch_sendraw(handle, "[2, \"echo something\"]\n", {'callback': 'Ch_handleRaw2'})
  call ch_sendraw(handle, "[3, \"wait a bit\"]\n", {'callback': 'Ch_handleRaw3'})
  call WaitForAssert({-> assert_equal("[2, \"something\"]", g:Ch_reply2)})
  " wait for the 200 msec delayed reply
  call WaitForAssert({-> assert_equal("[3, \"waited\"]", g:Ch_reply3)})
endfunc

func Test_raw_one_time_callback()
  call s:run_server('Ch_raw_one_time_callback')
endfunc

func Test_raw_one_time_callback_ipv6()
  CheckIPv6
  call Test_raw_one_time_callback()
endfunc

func Test_raw_one_time_callback_unix()
  CheckUnix
  call Test_raw_one_time_callback()
  call delete('Xtestsocket')
endfunc

"""""""""

" Test that trying to connect to a non-existing port fails quickly.
func Test_connect_waittime()
  CheckFunction reltimefloat
  " this is timing sensitive

  let start = reltime()
  let handle = ch_open('localhost:9876', s:chopt)
  if ch_status(handle) != "fail"
    " Oops, port exists.
    call ch_close(handle)
  else
    let elapsed = reltime(start)
    call assert_inrange(0.0, 1.0, reltimefloat(elapsed))
  endif

  " We intend to use a socket that doesn't exist and wait for half a second
  " before giving up.  If the socket does exist it can fail in various ways.
  " Check for "Connection reset by peer" to avoid flakiness.
  let start = reltime()
  try
    let handle = ch_open('localhost:9867', {'waittime': 500})
    if ch_status(handle) != "fail"
      " Oops, port exists.
      call ch_close(handle)
    else
      " Failed connection should wait about 500 msec.  Can be longer if the
      " computer is busy with other things.
      call assert_inrange(0.3, 1.5, reltimefloat(reltime(start)))
    endif
  catch
    if v:exception !~ 'Connection reset by peer'
      call assert_report("Caught exception: " . v:exception)
    endif
  endtry
endfunc

"""""""""

func Test_raw_pipe()
  " Add a dummy close callback to avoid that messages are dropped when calling
  " ch_canread().
  " Also test the non-blocking option.
  let job = job_start(s:python . " test_channel_pipe.py",
	\ {'mode': 'raw', 'drop': 'never', 'noblock': 1})
  call assert_equal(v:t_job, type(job))
  call assert_equal("run", job_status(job))

  call assert_equal("open", ch_status(job))
  call assert_equal("open", ch_status(job), {"part": "out"})
  call assert_equal("open", ch_status(job), {"part": "err"})
  call assert_fails('call ch_status(job, {"in_mode": "raw"})', 'E475:')
  call assert_fails('call ch_status(job, {"part": "in"})', 'E475:')

  let dict = ch_info(job)
  call assert_true(dict.id != 0)
  call assert_equal('open', dict.status)
  call assert_equal('open', dict.out_status)
  call assert_equal('RAW', dict.out_mode)
  call assert_equal('pipe', dict.out_io)
  call assert_equal('open', dict.err_status)
  call assert_equal('RAW', dict.err_mode)
  call assert_equal('pipe', dict.err_io)

  try
    " For a change use the job where a channel is expected.
    call ch_sendraw(job, "echo something\n")
    let msg = ch_readraw(job)
    call assert_equal("something\n", substitute(msg, "\r", "", 'g'))

    call ch_sendraw(job, "double this\n")
    let g:handle = job->job_getchannel()
    call WaitFor('g:handle->ch_canread()')
    unlet g:handle
    let msg = ch_readraw(job)
    call assert_equal("this\nAND this\n", substitute(msg, "\r", "", 'g'))

    let g:Ch_reply = ""
    call ch_sendraw(job, "double this\n", {'callback': 'Ch_handler'})
    call WaitForAssert({-> assert_equal("this\nAND this\n", substitute(g:Ch_reply, "\r", "", 'g'))})

    call assert_fails("let i = ch_evalraw(job, '2 + 2', {'callback' : 'abc'})", 'E917:')
    call assert_fails("let i = ch_evalexpr(job, '2 + 2')", 'E912:')
    call assert_fails("let i = ch_evalraw(job, '2 + 2', {'drop' : ''})", 'E475:')
    call assert_fails("let i = ch_evalraw(test_null_job(), '2 + 2')", 'E906:')

    let reply = job->ch_evalraw("quit\n", {'timeout': 100})
    call assert_equal("Goodbye!\n", substitute(reply, "\r", "", 'g'))
  finally
    call job_stop(job)
  endtry

  let g:Ch_job = job
  call WaitForAssert({-> assert_equal("dead", job_status(g:Ch_job))})
  let info = job->job_info()
  call assert_equal("dead", info.status)
  call assert_equal("term", info.stoponexit)
  call assert_equal(2, len(info.cmd))
  call assert_equal("test_channel_pipe.py", info.cmd[1])

  let found = 0
  for j in job_info()
    if j == job
      let found += 1
    endif
  endfor
  call assert_equal(1, found)

  call assert_fails("call job_stop('abc')", 'E475:')
  call assert_fails("call job_stop(job, [])", 'E730:')
  call assert_fails("call job_stop(test_null_job())", 'E916:')

  " Try to use the job and channel where a number is expected. This is not
  " related to testing the raw pipe. This test is here just to reuse the
  " already created job/channel.
  let ch = job_getchannel(job)
  call assert_fails('let i = job + 1', 'E910:')
  call assert_fails('let j = ch + 1', 'E913:')
  call assert_fails('echo 2.0 == job', 'E911:')
  call assert_fails('echo 2.0 == ch', 'E914:')
endfunc

func Test_raw_pipe_blob()
  " Add a dummy close callback to avoid that messages are dropped when calling
  " ch_canread().
  " Also test the non-blocking option.
  let job = job_start(s:python . " test_channel_pipe.py",
	\ {'mode': 'raw', 'drop': 'never', 'noblock': 1})
  call assert_equal(v:t_job, type(job))
  call assert_equal("run", job_status(job))

  call assert_equal("open", ch_status(job))
  call assert_equal("open", ch_status(job), {"part": "out"})

  try
    " Create a blob with the echo command and write it.
    let blob = 0z00
    let cmd = "echo something\n"
    for i in range(0, len(cmd) - 1)
      let blob[i] = char2nr(cmd[i])
    endfor
    call assert_equal(len(cmd), len(blob))
    call ch_sendraw(job, blob)

    " Read a blob with the reply.
    let msg = job->ch_readblob()
    let expected = 'something'
    for i in range(0, len(expected) - 1)
      call assert_equal(char2nr(expected[i]), msg[i])
    endfor

    let reply = ch_evalraw(job, "quit\n", {'timeout': 100})
    call assert_equal("Goodbye!\n", substitute(reply, "\r", "", 'g'))
  finally
    call job_stop(job)
  endtry

  let g:Ch_job = job
  call WaitForAssert({-> assert_equal("dead", job_status(g:Ch_job))})
  let info = job_info(job)
  call assert_equal("dead", info.status)
endfunc

func Test_nl_pipe()
  let job = job_start([s:python, "test_channel_pipe.py"])
  call assert_equal("run", job_status(job))
  try
    let handle = job_getchannel(job)
    call ch_sendraw(handle, "echo something\n")
    call assert_equal("something", handle->ch_readraw())

    call ch_sendraw(handle, "echoerr wrong\n")
    call assert_equal("wrong", ch_readraw(handle, {'part': 'err'}))

    call ch_sendraw(handle, "double this\n")
    call assert_equal("this", ch_readraw(handle))
    call assert_equal("AND this", ch_readraw(handle))

    call ch_sendraw(handle, "split this line\n")
    call assert_equal("this linethis linethis line", handle->ch_read())

    let reply = ch_evalraw(handle, "quit\n")
    call assert_equal("Goodbye!", reply)
  finally
    call job_stop(job)
  endtry
endfunc

func Stop_g_job()
  call job_stop(g:job)
  if has('win32')
    " On MS-Windows the server must close the file handle before we are able
    " to delete the file.
    call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
    sleep 10m
  endif
endfunc

func Test_nl_read_file()
  call writefile(['echo something', 'echoerr wrong', 'double this'], 'Xinput', 'D')
  let g:job = job_start(s:python . " test_channel_pipe.py",
	\ {'in_io': 'file', 'in_name': 'Xinput'})
  call assert_equal("run", job_status(g:job))
  try
    let handle = job_getchannel(g:job)
    call assert_equal("something", ch_readraw(handle))
    call assert_equal("wrong", ch_readraw(handle, {'part': 'err'}))
    call assert_equal("this", ch_readraw(handle))
    call assert_equal("AND this", ch_readraw(handle))
  finally
    call Stop_g_job()
  endtry
  call assert_fails("echo ch_read(test_null_channel(), {'callback' : 'abc'})", 'E475:')
endfunc

func Test_nl_write_out_file()
  let g:job = job_start(s:python . " test_channel_pipe.py",
	\ {'out_io': 'file', 'out_name': 'Xoutput'})
  call assert_equal("run", job_status(g:job))
  try
    let handle = job_getchannel(g:job)
    call ch_sendraw(handle, "echo line one\n")
    call ch_sendraw(handle, "echo line two\n")
    call ch_sendraw(handle, "double this\n")
    call WaitForAssert({-> assert_equal(['line one', 'line two', 'this', 'AND this'], readfile('Xoutput'))})
  finally
    call Stop_g_job()
    call assert_equal(-1, match(s:get_resources(), '\(^\|/\)Xoutput$'))
    call delete('Xoutput')
  endtry
endfunc

func Test_nl_write_err_file()
  let g:job = job_start(s:python . " test_channel_pipe.py",
	\ {'err_io': 'file', 'err_name': 'Xoutput'})
  call assert_equal("run", job_status(g:job))
  try
    let handle = job_getchannel(g:job)
    call ch_sendraw(handle, "echoerr line one\n")
    call ch_sendraw(handle, "echoerr line two\n")
    call ch_sendraw(handle, "doubleerr this\n")
    call WaitForAssert({-> assert_equal(['line one', 'line two', 'this', 'AND this'], readfile('Xoutput'))})
  finally
    call Stop_g_job()
    call delete('Xoutput')
  endtry
endfunc

func Test_nl_write_both_file()
  let g:job = job_start(s:python . " test_channel_pipe.py",
	\ {'out_io': 'file', 'out_name': 'Xoutput', 'err_io': 'out'})
  call assert_equal("run", job_status(g:job))
  try
    let handle = job_getchannel(g:job)
    call ch_sendraw(handle, "echoerr line one\n")
    call ch_sendraw(handle, "echo line two\n")
    call ch_sendraw(handle, "double this\n")
    call ch_sendraw(handle, "doubleerr that\n")
    call WaitForAssert({-> assert_equal(['line one', 'line two', 'this', 'AND this', 'that', 'AND that'], readfile('Xoutput'))})
  finally
    call Stop_g_job()
    call assert_equal(-1, match(s:get_resources(), '\(^\|/\)Xoutput$'))
    call delete('Xoutput')
  endtry
endfunc

func BufCloseCb(ch)
  let g:Ch_bufClosed = 'yes'
endfunc

func Run_test_pipe_to_buffer(use_name, nomod, do_msg)
  let g:Ch_bufClosed = 'no'
  let options = {'out_io': 'buffer', 'close_cb': 'BufCloseCb'}
  let expected = ['', 'line one', 'line two', 'this', 'AND this', 'Goodbye!']
  if a:use_name
    let options['out_name'] = 'pipe-output'
    if a:do_msg
      let expected[0] = 'Reading from channel output...'
    else
      let options['out_msg'] = 0
      call remove(expected, 0)
    endif
  else
    sp pipe-output
    let options['out_buf'] = bufnr('%')
    quit
    call remove(expected, 0)
  endif
  if a:nomod
    let options['out_modifiable'] = 0
  endif
  let job = job_start(s:python . " test_channel_pipe.py", options)
  call assert_equal("run", job_status(job))
  try
    let handle = job_getchannel(job)
    call ch_sendraw(handle, "echo line one\n")
    call ch_sendraw(handle, "echo line two\n")
    call ch_sendraw(handle, "double this\n")
    call ch_sendraw(handle, "quit\n")
    sp pipe-output
    call WaitFor('line("$") == ' . len(expected) . ' && g:Ch_bufClosed == "yes"')
    call assert_equal(expected, getline(1, '$'))
    if a:nomod
      call assert_equal(0, &modifiable)
    else
      call assert_equal(1, &modifiable)
    endif
    call assert_equal('yes', g:Ch_bufClosed)
    bwipe!
  finally
    call job_stop(job)
  endtry
endfunc

func Test_pipe_to_buffer_name()
  call Run_test_pipe_to_buffer(1, 0, 1)
endfunc

func Test_pipe_to_buffer_nr()
  call Run_test_pipe_to_buffer(0, 0, 1)
endfunc

func Test_pipe_to_buffer_name_nomod()
  call Run_test_pipe_to_buffer(1, 1, 1)
endfunc

func Test_pipe_to_buffer_name_nomsg()
  call Run_test_pipe_to_buffer(1, 0, 1)
endfunc

func Test_close_output_buffer()
  let g:test_is_flaky = 1
  enew!
  let test_lines = ['one', 'two']
  call setline(1, test_lines)
  let options = {'out_io': 'buffer'}
  let options['out_name'] = 'buffer-output'
  let options['out_msg'] = 0
  split buffer-output
  let job = job_start(s:python . " test_channel_write.py", options)
  call assert_equal("run", job_status(job))
  try
    call WaitForAssert({-> assert_equal(3, line('$'))})
    quit!
    sleep 100m
    " Make sure the write didn't happen to the wrong buffer.
    call assert_equal(test_lines, getline(1, line('$')))
    call assert_equal(-1, bufwinnr('buffer-output'))
    sbuf buffer-output
    call assert_notequal(-1, bufwinnr('buffer-output'))
    sleep 100m
    close  " no more writes
    bwipe!
  finally
    call job_stop(job)
  endtry
endfunc

func Run_test_pipe_err_to_buffer(use_name, nomod, do_msg)
  let options = {'err_io': 'buffer'}
  let expected = ['', 'line one', 'line two', 'this', 'AND this']
  if a:use_name
    let options['err_name'] = 'pipe-err'
    if a:do_msg
      let expected[0] = 'Reading from channel error...'
    else
      let options['err_msg'] = 0
      call remove(expected, 0)
    endif
  else
    sp pipe-err
    let options['err_buf'] = bufnr('%')
    quit
    call remove(expected, 0)
  endif
  if a:nomod
    let options['err_modifiable'] = 0
  endif
  let job = job_start(s:python . " test_channel_pipe.py", options)
  call assert_equal("run", job_status(job))
  try
    let handle = job_getchannel(job)
    call ch_sendraw(handle, "echoerr line one\n")
    call ch_sendraw(handle, "echoerr line two\n")
    call ch_sendraw(handle, "doubleerr this\n")
    call ch_sendraw(handle, "quit\n")
    sp pipe-err
    call WaitForAssert({-> assert_equal(expected, getline(1, '$'))})
    if a:nomod
      call assert_equal(0, &modifiable)
    else
      call assert_equal(1, &modifiable)
    endif
    bwipe!
  finally
    call job_stop(job)
  endtry
endfunc

func Test_pipe_err_to_buffer_name()
  call Run_test_pipe_err_to_buffer(1, 0, 1)
endfunc

func Test_pipe_err_to_buffer_nr()
  call Run_test_pipe_err_to_buffer(0, 0, 1)
endfunc

func Test_pipe_err_to_buffer_name_nomod()
  call Run_test_pipe_err_to_buffer(1, 1, 1)
endfunc

func Test_pipe_err_to_buffer_name_nomsg()
  call Run_test_pipe_err_to_buffer(1, 0, 0)
endfunc

func Test_pipe_both_to_buffer()
  let job = job_start(s:python . " test_channel_pipe.py",
	\ {'out_io': 'buffer', 'out_name': 'pipe-err', 'err_io': 'out'})
  call assert_equal("run", job_status(job))
  let handle = job_getchannel(job)
  call assert_equal(bufnr('pipe-err'), ch_getbufnr(handle, 'out'))
  call assert_equal(bufnr('pipe-err'), ch_getbufnr(handle, 'err'))
  try
    call ch_sendraw(handle, "echo line one\n")
    call ch_sendraw(handle, "echoerr line two\n")
    call ch_sendraw(handle, "double this\n")
    call ch_sendraw(handle, "doubleerr that\n")
    call ch_sendraw(handle, "quit\n")
    sp pipe-err
    call WaitForAssert({-> assert_equal(['Reading from channel output...', 'line one', 'line two', 'this', 'AND this', 'that', 'AND that', 'Goodbye!'], getline(1, '$'))})
    bwipe!
  finally
    call job_stop(job)
  endtry
endfunc

func Run_test_pipe_from_buffer(use_name)
  sp pipe-input
  call setline(1, ['echo one', 'echo two', 'echo three'])
  let options = {'in_io': 'buffer', 'block_write': 1}
  if a:use_name
    let options['in_name'] = 'pipe-input'
  else
    let options['in_buf'] = bufnr('%')
  endif

  let job = job_start(s:python . " test_channel_pipe.py", options)
  call assert_equal("run", job_status(job))
  if has('unix') && !a:use_name
    call assert_equal(bufnr('%'), ch_getbufnr(job, 'in'))
  endif
  try
    let handle = job_getchannel(job)
    call assert_equal('one', ch_read(handle))
    call assert_equal('two', ch_read(handle))
    call assert_equal('three', ch_read(handle))
    bwipe!
  finally
    call job_stop(job)
  endtry
endfunc

func Test_pipe_from_buffer_name()
  call Run_test_pipe_from_buffer(1)
endfunc

func Test_pipe_from_buffer_nr()
  call Run_test_pipe_from_buffer(0)
endfunc

func Run_pipe_through_sort(all, use_buffer)
  CheckExecutable sort
  let g:test_is_flaky = 1

  let options = {'out_io': 'buffer', 'out_name': 'sortout'}
  if a:use_buffer
    split sortin
    call setline(1, ['ccc', 'aaa', 'ddd', 'bbb', 'eee'])
    let options.in_io = 'buffer'
    let options.in_name = 'sortin'
  endif
  if !a:all
    let options.in_top = 2
    let options.in_bot = 4
  endif
  let job = job_start('sort', options)

  if !a:use_buffer
    call assert_equal("run", job_status(job))
    call ch_sendraw(job, "ccc\naaa\nddd\nbbb\neee\n")
    eval job->ch_close_in()
  endif

  call WaitForAssert({-> assert_equal("dead", job_status(job))})

  sp sortout
  call WaitFor('line("$") > 3')
  call assert_equal('Reading from channel output...', getline(1))
  if a:all
    call assert_equal(['aaa', 'bbb', 'ccc', 'ddd', 'eee'], getline(2, 6))
  else
    call assert_equal(['aaa', 'bbb', 'ddd'], getline(2, 4))
  endif

  call job_stop(job)
  if a:use_buffer
    bwipe! sortin
  endif
  bwipe! sortout
endfunc

func Test_pipe_through_sort_all()
  call Run_pipe_through_sort(1, 1)
endfunc

func Test_pipe_through_sort_some()
  call Run_pipe_through_sort(0, 1)
endfunc

func Test_pipe_through_sort_feed()
  call Run_pipe_through_sort(1, 0)
endfunc

func Test_pipe_to_nameless_buffer()
  let job = job_start(s:python . " test_channel_pipe.py",
	\ {'out_io': 'buffer'})
  call assert_equal("run", job_status(job))
  try
    let handle = job_getchannel(job)
    call ch_sendraw(handle, "echo line one\n")
    call ch_sendraw(handle, "echo line two\n")
    exe handle->ch_getbufnr("out") .. 'sbuf'
    call WaitFor('line("$") >= 3')
    call assert_equal(['Reading from channel output...', 'line one', 'line two'], getline(1, '$'))
    bwipe!
  finally
    call job_stop(job)
  endtry
endfunc

func Test_pipe_to_buffer_json()
  CheckFunction reltimefloat

  let job = job_start(s:python . " test_channel_pipe.py",
	\ {'out_io': 'buffer', 'out_mode': 'json'})
  call assert_equal("run", job_status(job))
  try
    let handle = job_getchannel(job)
    call ch_sendraw(handle, "echo [0, \"hello\"]\n")
    call ch_sendraw(handle, "echo [-2, 12.34]\n")
    exe ch_getbufnr(handle, "out") . 'sbuf'
    call WaitFor('line("$") >= 3')
    call assert_equal(['Reading from channel output...', '[0,"hello"]', '[-2,12.34]'], getline(1, '$'))
    bwipe!
  finally
    call job_stop(job)
  endtry
endfunc

" Wait a little while for the last line, minus "offset", to equal "line".
func s:wait_for_last_line(line, offset)
  for i in range(100)
    if getline(line('$') - a:offset) == a:line
      break
    endif
    sleep 10m
  endfor
endfunc

func Test_pipe_io_two_buffers()
  " Create two buffers, one to read from and one to write to.
  split pipe-output
  set buftype=nofile
  split pipe-input
  set buftype=nofile

  let job = job_start(s:python . " test_channel_pipe.py",
	\ {'in_io': 'buffer', 'in_name': 'pipe-input', 'in_top': 0,
	\  'out_io': 'buffer', 'out_name': 'pipe-output',
	\  'block_write': 1})
  call assert_equal("run", job_status(job))
  try
    exe "normal Gaecho hello\<CR>"
    exe bufwinnr('pipe-output') . "wincmd w"
    call s:wait_for_last_line('hello', 0)
    call assert_equal('hello', getline('$'))

    exe bufwinnr('pipe-input') . "wincmd w"
    exe "normal Gadouble this\<CR>"
    exe bufwinnr('pipe-output') . "wincmd w"
    call s:wait_for_last_line('AND this', 0)
    call assert_equal('this', getline(line('$') - 1))
    call assert_equal('AND this', getline('$'))

    bwipe!
    exe bufwinnr('pipe-input') . "wincmd w"
    bwipe!
  finally
    call job_stop(job)
  endtry
endfunc

func Test_pipe_io_one_buffer()
  " Create one buffer to read from and to write to.
  split pipe-io
  set buftype=nofile

  let job = job_start(s:python . " test_channel_pipe.py",
	\ {'in_io': 'buffer', 'in_name': 'pipe-io', 'in_top': 0,
	\  'out_io': 'buffer', 'out_name': 'pipe-io',
	\  'block_write': 1})
  call assert_equal("run", job_status(job))
  try
    exe "normal Goecho hello\<CR>"
    call s:wait_for_last_line('hello', 1)
    call assert_equal('hello', getline(line('$') - 1))

    exe "normal Gadouble this\<CR>"
    call s:wait_for_last_line('AND this', 1)
    call assert_equal('this', getline(line('$') - 2))
    call assert_equal('AND this', getline(line('$') - 1))

    bwipe!
  finally
    call job_stop(job)
  endtry
endfunc

func Test_write_to_buffer_and_scroll()
  CheckScreendump

  let lines =<< trim END
      new Xscrollbuffer
      call setline(1, range(1, 200))
      $
      redraw
      wincmd w
      call deletebufline('Xscrollbuffer', 1, '$')
      if has('win32')
	let cmd = ['cmd', '/c', 'echo sometext']
      else
	let cmd = [&shell, &shellcmdflag, 'echo sometext']
      endif
      call job_start(cmd, #{out_io: 'buffer', out_name: 'Xscrollbuffer'})
  END
  call writefile(lines, 'XtestBufferScroll', 'D')
  let buf = RunVimInTerminal('-S XtestBufferScroll', #{rows: 10})
  call TermWait(buf, 50)
  call VerifyScreenDump(buf, 'Test_job_buffer_scroll_1', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_pipe_null()
  " We cannot check that no I/O works, we only check that the job starts
  " properly.
  let job = job_start(s:python . " test_channel_pipe.py something",
	\ {'in_io': 'null'})
  call assert_equal("run", job_status(job))
  try
    call assert_equal('something', ch_read(job))
  finally
    call job_stop(job)
  endtry

  let job = job_start(s:python . " test_channel_pipe.py err-out",
	\ {'out_io': 'null'})
  call assert_equal("run", job_status(job))
  try
    call assert_equal('err-out', ch_read(job, {"part": "err"}))
  finally
    call job_stop(job)
  endtry

  let job = job_start(s:python . " test_channel_pipe.py something",
	\ {'err_io': 'null'})
  call assert_equal("run", job_status(job))
  try
    call assert_equal('something', ch_read(job))
  finally
    call job_stop(job)
  endtry

  " This causes spurious leak errors with valgrind.
  if !RunningWithValgrind()
    let job = job_start(s:python . " test_channel_pipe.py something",
          \ {'out_io': 'null', 'err_io': 'out'})
    call assert_equal("run", job_status(job))
    call job_stop(job)

    let job = job_start(s:python . " test_channel_pipe.py something",
          \ {'in_io': 'null', 'out_io': 'null', 'err_io': 'null'})
    call assert_equal("run", job_status(job))
    call assert_equal('channel fail', string(job_getchannel(job)))
    call assert_equal('fail', ch_status(job))
    call assert_equal('no process', string(test_null_job()))
    call assert_equal('channel fail', string(test_null_channel()))
    call job_stop(job)
  endif
endfunc

func Test_pipe_to_buffer_raw()
  let options = {'out_mode': 'raw', 'out_io': 'buffer', 'out_name': 'testout'}
  split testout
  let job = job_start([s:python, '-c',
        \ 'import sys; [sys.stdout.write(".") and sys.stdout.flush() for _ in range(10000)]'], options)
  " the job may be done quickly, also accept "dead"
  call assert_match('^\%(dead\|run\)$', job_status(job))
  call WaitFor('len(join(getline(1, "$"), "")) >= 10000')
  try
    let totlen = 0
    for line in getline(1, '$')
      call assert_equal('', substitute(line, '^\.*', '', ''))
      let totlen += len(line)
    endfor
    call assert_equal(10000, totlen)
  finally
    call job_stop(job)
    bwipe!
  endtry
endfunc

func Test_reuse_channel()
  let job = job_start(s:python . " test_channel_pipe.py")
  call assert_equal("run", job_status(job))
  let handle = job_getchannel(job)
  try
    call ch_sendraw(handle, "echo something\n")
    call assert_equal("something", ch_readraw(handle))
  finally
    call job_stop(job)
  endtry

  let job = job_start(s:python . " test_channel_pipe.py", {'channel': handle})
  call assert_equal("run", job_status(job))
  let handle = job_getchannel(job)
  try
    call ch_sendraw(handle, "echo again\n")
    call assert_equal("again", ch_readraw(handle))
  finally
    call job_stop(job)
  endtry
endfunc

func Test_out_cb()
  let g:test_is_flaky = 1
  let dict = {'thisis': 'dict: '}
  func dict.outHandler(chan, msg) dict
    if type(a:msg) == v:t_string
      let g:Ch_outmsg = self.thisis . a:msg
    else
      let g:Ch_outobj = a:msg
    endif
  endfunc
  func dict.errHandler(chan, msg) dict
    let g:Ch_errmsg = self.thisis . a:msg
  endfunc
  let job = job_start(s:python . " test_channel_pipe.py",
	\ {'out_cb': dict.outHandler,
	\  'out_mode': 'json',
	\  'err_cb': dict.errHandler,
	\  'err_mode': 'json'})
  call assert_equal("run", job_status(job))
  call test_garbagecollect_now()
  try
    let g:Ch_outmsg = ''
    let g:Ch_errmsg = ''
    call ch_sendraw(job, "echo [0, \"hello\"]\n")
    call ch_sendraw(job, "echoerr [0, \"there\"]\n")
    call WaitForAssert({-> assert_equal("dict: hello", g:Ch_outmsg)})
    call WaitForAssert({-> assert_equal("dict: there", g:Ch_errmsg)})

    " Receive a json object split in pieces
    let g:Ch_outobj = ''
    call ch_sendraw(job, "echosplit [0, {\"one\": 1,| \"tw|o\": 2, \"three\": 3|}]\n")
    " For unknown reasons this can be very slow on Mac.
    " Increase the timeout on every run.
    if g:run_nr == 1
      let timeout = 5000
    elseif g:run_nr == 2
      let timeout = 10000
    elseif g:run_nr == 3
      let timeout = 20000
    else
      let timeout = 40000
    endif
    call WaitForAssert({-> assert_equal({'one': 1, 'two': 2, 'three': 3}, g:Ch_outobj)}, timeout)
  finally
    call job_stop(job)
  endtry
endfunc

func Test_out_close_cb()
  let s:counter = 1
  let g:Ch_msg1 = ''
  let g:Ch_closemsg = 0
  func! OutHandler(chan, msg)
    if s:counter == 1
      let g:Ch_msg1 = a:msg
    endif
    let s:counter += 1
  endfunc
  func! CloseHandler(chan)
    let g:Ch_closemsg = s:counter
    let s:counter += 1
  endfunc
  let job = job_start(s:python . " test_channel_pipe.py quit now",
	\ {'out_cb': 'OutHandler',
	\  'close_cb': 'CloseHandler'})
  " the job may be done quickly, also accept "dead"
  call assert_match('^\%(dead\|run\)$', job_status(job))
  try
    call WaitForAssert({-> assert_equal('quit', g:Ch_msg1)})
    call WaitForAssert({-> assert_equal(2, g:Ch_closemsg)})
  finally
    call job_stop(job)
    delfunc OutHandler
    delfunc CloseHandler
  endtry
endfunc

func Test_read_in_close_cb()
  let g:Ch_received = ''
  func! CloseHandler(chan)
    let g:Ch_received = ch_read(a:chan)
  endfunc
  let job = job_start(s:python . " test_channel_pipe.py quit now",
	\ {'close_cb': 'CloseHandler'})
  " the job may be done quickly, also accept "dead"
  call assert_match('^\%(dead\|run\)$', job_status(job))
  try
    call WaitForAssert({-> assert_equal('quit', g:Ch_received)})
  finally
    call job_stop(job)
    delfunc CloseHandler
  endtry
endfunc

" Use channel in NL mode but received text does not end in NL.
func Test_read_in_close_cb_incomplete()
  let g:Ch_received = ''
  func! CloseHandler(chan)
    while ch_status(a:chan, {'part': 'out'}) == 'buffered'
      let g:Ch_received .= ch_read(a:chan)
    endwhile
  endfunc
  let job = job_start(s:python . " test_channel_pipe.py incomplete",
	\ {'close_cb': 'CloseHandler'})
  " the job may be done quickly, also accept "dead"
  call assert_match('^\%(dead\|run\)$', job_status(job))
  try
    call WaitForAssert({-> assert_equal('incomplete', g:Ch_received)})
  finally
    call job_stop(job)
    delfunc CloseHandler
  endtry
endfunc

func Test_out_cb_lambda()
  let job = job_start(s:python . " test_channel_pipe.py",
        \ {'out_cb': {ch, msg -> execute("let g:Ch_outmsg = 'lambda: ' . msg")},
        \  'out_mode': 'json',
        \  'err_cb': {ch, msg -> execute(":let g:Ch_errmsg = 'lambda: ' . msg")},
        \  'err_mode': 'json'})
  call assert_equal("run", job_status(job))
  try
    let g:Ch_outmsg = ''
    let g:Ch_errmsg = ''
    call ch_sendraw(job, "echo [0, \"hello\"]\n")
    call ch_sendraw(job, "echoerr [0, \"there\"]\n")
    call WaitForAssert({-> assert_equal("lambda: hello", g:Ch_outmsg)})
    call WaitForAssert({-> assert_equal("lambda: there", g:Ch_errmsg)})
  finally
    call job_stop(job)
  endtry
endfunc

func Test_close_and_exit_cb()
  let g:test_is_flaky = 1
  let g:retdict = {'ret': {}}
  func g:retdict.close_cb(ch) dict
    let self.ret['close_cb'] = a:ch->ch_getjob()->job_status()
  endfunc
  func g:retdict.exit_cb(job, status) dict
    let self.ret['exit_cb'] = job_status(a:job)
  endfunc

  let job = job_start([&shell, &shellcmdflag, 'echo'],
        \ {'close_cb': g:retdict.close_cb,
        \  'exit_cb': g:retdict.exit_cb})
  " the job may be done quickly, also accept "dead"
  call assert_match('^\%(dead\|run\)$', job_status(job))
  call WaitForAssert({-> assert_equal(2, len(g:retdict.ret))})
  call assert_match('^\%(dead\|run\)$', g:retdict.ret['close_cb'])
  call assert_equal('dead', g:retdict.ret['exit_cb'])
  unlet g:retdict
endfunc

""""""""""

function ExitCbWipe(job, status)
  exe g:wipe_buf 'bw!'
endfunction

" This caused a crash, because messages were handled while peeking for a
" character.
func Test_exit_cb_wipes_buf()
  CheckFeature timers
  set cursorline lazyredraw
  call test_override('redraw_flag', 1)
  new
  let g:wipe_buf = bufnr('')

  let job = job_start(has('win32') ? 'cmd /c echo:' : ['true'],
	\ {'exit_cb': 'ExitCbWipe'})
  let timer = timer_start(300, {-> feedkeys("\<Esc>", 'nt')}, {'repeat': 5})
  call feedkeys(repeat('g', 1000) . 'o', 'ntx!')
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  call timer_stop(timer)

  set nocursorline nolazyredraw
  unlet g:wipe_buf
  call test_override('ALL', 0)
endfunc

""""""""""

let g:Ch_unletResponse = ''
func s:UnletHandler(handle, msg)
  let g:Ch_unletResponse = a:msg
  unlet s:channelfd
endfunc

" Test that "unlet handle" in a handler doesn't crash Vim.
func Ch_unlet_handle(port)
  let s:channelfd = ch_open(s:address(a:port), s:chopt)
  eval s:channelfd->ch_sendexpr("test", {'callback': function('s:UnletHandler')})
  call WaitForAssert({-> assert_equal('what?', g:Ch_unletResponse)})
endfunc

func Test_unlet_handle()
  call s:run_server('Ch_unlet_handle')
endfunc

func Test_unlet_handle_ipv6()
  CheckIPv6
  call Test_unlet_handle()
endfunc

""""""""""

let g:Ch_unletResponse = ''
func Ch_CloseHandler(handle, msg)
  let g:Ch_unletResponse = a:msg
  eval s:channelfd->ch_close()
endfunc

" Test that "unlet handle" in a handler doesn't crash Vim.
func Ch_close_handle(port)
  let s:channelfd = ch_open(s:address(a:port), s:chopt)
  call ch_sendexpr(s:channelfd, "test", {'callback': function('Ch_CloseHandler')})
  call WaitForAssert({-> assert_equal('what?', g:Ch_unletResponse)})
endfunc

func Test_close_handle()
  call s:run_server('Ch_close_handle')
endfunc

func Test_close_handle_ipv6()
  CheckIPv6
  call Test_close_handle()
endfunc

""""""""""

func Ch_open_ipv6(port)
  let handle = ch_open(s:address(a:port), s:chopt)
  call assert_notequal('fail', ch_status(handle))
endfunc

func Test_open_ipv6()
  CheckIPv6
  call s:run_server('Ch_open_ipv6')
endfunc

""""""""""

func Test_open_fail()
  call assert_fails("let ch = ch_open('noserver')", 'E475:')
  echo ch
  let d = ch
  call assert_fails("let ch = ch_open('noserver', 10)", 'E1206:')
  call assert_fails("let ch = ch_open('localhost:-1')", 'E475:')
  call assert_fails("let ch = ch_open('localhost:65537')", 'E475:')
  call assert_fails("let ch = ch_open('localhost:8765', {'timeout' : -1})",
        \ 'E474:')
  call assert_fails("let ch = ch_open('localhost:8765', {'axby' : 1})",
        \ 'E475:')
  call assert_fails("let ch = ch_open('localhost:8765', {'mode' : 'abc'})",
        \ 'E475:')
  call assert_fails("let ch = ch_open('localhost:8765', {'part' : 'out'})",
        \ 'E475:')
  call assert_fails("let ch = ch_open('[::]')", 'E475:')
  call assert_fails("let ch = ch_open('[::.80')", 'E475:')
  call assert_fails("let ch = ch_open('[::]8080')", 'E475:')
endfunc

func Test_ch_info_fail()
  call assert_fails("let x = ch_info(10)", 'E475:')
endfunc

""""""""""

func Ch_open_delay(port)
  " Wait up to a second for the port to open.
  let s:chopt.waittime = 1000
  let channel = ch_open(s:address(a:port), s:chopt)
  if ch_status(channel) == "fail"
    call assert_report("Can't open channel")
    return
  endif
  call assert_equal('got it', channel->ch_evalexpr('hello!'))
  call ch_close(channel)
endfunc

func Test_open_delay()
  " This fails on BSD (e.g. Cirrus-CI), why?
  CheckNotBSD
  " The server will wait half a second before creating the port.
  call s:run_server('Ch_open_delay', 'delay')
endfunc

func Test_open_delay_ipv6()
  CheckIPv6
  " This fails on BSD (e.g. Cirrus-CI), why?
  CheckNotBSD
  call Test_open_delay()
endfunc

"""""""""

function MyFunction(a,b,c)
  let g:Ch_call_ret = [a:a, a:b, a:c]
endfunc

function Ch_test_call(port)
  let handle = ch_open(s:address(a:port), s:chopt)
  if ch_status(handle) == "fail"
    call assert_report("Can't open channel")
    return
  endif

  let g:Ch_call_ret = []
  call assert_equal('ok', ch_evalexpr(handle, 'call-func'))
  call WaitForAssert({-> assert_equal([1, 2, 3], g:Ch_call_ret)})

  call assert_fails("let i = ch_evalexpr(handle, '2 + 2', {'callback' : 'abc'})", 'E917:')
  call assert_fails("let i = ch_evalexpr(handle, '2 + 2', {'drop' : ''})", 'E475:')
  call assert_fails("let i = ch_evalexpr(test_null_job(), '2 + 2')", 'E906:')
endfunc

func Test_call()
  call s:run_server('Ch_test_call')
endfunc

func Test_call_ipv6()
  CheckIPv6
  call Test_call()
endfunc

func Test_call_unix()
  CheckUnix
  call Test_call()
  call delete('Xtestsocket')
endfunc

"""""""""

let g:Ch_job_exit_ret = 'not yet'
function MyExitCb(job, status)
  let g:Ch_job_exit_ret = 'done'
endfunc

function Ch_test_exit_callback(port)
  eval g:currentJob->job_setoptions({'exit_cb': 'MyExitCb'})
  let g:Ch_exit_job = g:currentJob
  call assert_equal('MyExitCb', job_info(g:currentJob)['exit_cb'])
endfunc

func Test_exit_callback()
  call s:run_server('Ch_test_exit_callback')

  " wait up to a second for the job to exit
  for i in range(100)
    if g:Ch_job_exit_ret == 'done'
      break
    endif
    sleep 10m
    " calling job_status() triggers the callback
    call job_status(g:Ch_exit_job)
  endfor

  call assert_equal('done', g:Ch_job_exit_ret)
  call assert_equal('dead', job_info(g:Ch_exit_job).status)
  unlet g:Ch_exit_job
endfunc

function MyExitTimeCb(job, status)
  if job_info(a:job).process == g:exit_cb_val.process
    let g:exit_cb_val.end = reltime(g:exit_cb_val.start)
  endif
  call Resume()
endfunction

func Test_exit_callback_interval()
  CheckFunction reltimefloat
  let g:test_is_flaky = 1

  let g:exit_cb_val = {'start': reltime(), 'end': 0, 'process': 0}
  let job = [s:python, '-c', 'import time;time.sleep(0.5)']->job_start({'exit_cb': 'MyExitTimeCb'})
  let g:exit_cb_val.process = job_info(job).process
  try
    call WaitFor('type(g:exit_cb_val.end) != v:t_number || g:exit_cb_val.end != 0')
  catch
    call add(v:errors, "Job status: " .. string(job->job_info()))
    throw v:exception
  endtry
  let elapsed = reltimefloat(g:exit_cb_val.end)
  call assert_inrange(0.5, 1.0, elapsed)

  " case: unreferenced job, using timer
  if !has('timers')
    return
  endif

  let g:exit_cb_val = {'start': reltime(), 'end': 0, 'process': 0}
  let g:job = job_start([s:python, '-c', 'import time;time.sleep(0.5)'], {'exit_cb': 'MyExitTimeCb'})
  let g:exit_cb_val.process = job_info(g:job).process
  unlet g:job
  call Standby(1000)
  if type(g:exit_cb_val.end) != v:t_number || g:exit_cb_val.end != 0
    let elapsed = reltimefloat(g:exit_cb_val.end)
  else
    let elapsed = 1.0
  endif
  call assert_inrange(0.5, 1.0, elapsed)
endfunc

"""""""""

let g:Ch_close_ret = 'alive'
function MyCloseCb(ch)
  let g:Ch_close_ret = 'closed'
endfunc

function Ch_test_close_callback(port)
  let handle = ch_open(s:address(a:port), s:chopt)
  if ch_status(handle) == "fail"
    call assert_report("Can't open channel")
    return
  endif
  call ch_setoptions(handle, {'close_cb': 'MyCloseCb'})

  call assert_equal('', ch_evalexpr(handle, 'close me'))
  call WaitForAssert({-> assert_equal('closed', g:Ch_close_ret)})
endfunc

func Test_close_callback()
  call s:run_server('Ch_test_close_callback')
endfunc

func Test_close_callback_ipv6()
  CheckIPv6
  call Test_close_callback()
endfunc

func Test_close_callback_unix()
  CheckUnix
  call Test_close_callback()
  call delete('Xtestsocket')
endfunc

function Ch_test_close_partial(port)
  let handle = ch_open(s:address(a:port), s:chopt)
  if ch_status(handle) == "fail"
    call assert_report("Can't open channel")
    return
  endif
  let g:Ch_d = {}
  func g:Ch_d.closeCb(ch) dict
    let self.close_ret = 'closed'
  endfunc
  call ch_setoptions(handle, {'close_cb': g:Ch_d.closeCb})

  call assert_equal('', ch_evalexpr(handle, 'close me'))
  call WaitForAssert({-> assert_equal('closed', g:Ch_d.close_ret)})
  unlet g:Ch_d
endfunc

func Test_close_partial()
  call s:run_server('Ch_test_close_partial')
endfunc

func Test_close_partial_ipv6()
  CheckIPv6
  call Test_close_partial()
endfunc

func Test_close_partial_unix()
  CheckUnix
  call Test_close_partial()
  call delete('Xtestsocket')
endfunc

func Test_job_start_fails()
  " this was leaking memory
  call assert_fails("call job_start([''])", "E474:")
  call assert_fails('call job_start($x)', 'E474:')
  call assert_fails('call job_start("")', 'E474:')
  call assert_fails('call job_start("ls", {"out_io" : "abc"})', 'E475:')
  call assert_fails('call job_start("ls", {"err_io" : "abc"})', 'E475:')
  call assert_fails('call job_start("ls", [])', 'E715:')
  call assert_fails("call job_start('ls', {'in_top' : -1})", 'E475:')
  call assert_fails("call job_start('ls', {'in_bot' : -1})", 'E475:')
  call assert_fails("call job_start('ls', {'channel' : -1})", 'E475:')
  call assert_fails("call job_start('ls', {'callback' : -1})", 'E921:')
  call assert_fails("call job_start('ls', {'out_cb' : -1})", 'E921:')
  call assert_fails("call job_start('ls', {'err_cb' : -1})", 'E921:')
  call assert_fails("call job_start('ls', {'close_cb' : -1})", 'E921:')
  call assert_fails("call job_start('ls', {'exit_cb' : -1})", 'E921:')
  call assert_fails("call job_start('ls', {'term_name' : []})", 'E475:')
  call assert_fails("call job_start('ls', {'term_finish' : 'run'})", 'E475:')
  call assert_fails("call job_start('ls', {'term_api' : []})", 'E475:')
  call assert_fails("call job_start('ls', {'stoponexit' : []})", 'E730:')
  call assert_fails("call job_start('ls', {'in_io' : 'file'})", 'E920:')
  call assert_fails("call job_start('ls', {'out_io' : 'file'})", 'E920:')
  call assert_fails("call job_start('ls', {'err_io' : 'file'})", 'E920:')
  call assert_fails("call job_start('ls', {'in_mode' : 'abc'})", 'E475:')
  call assert_fails("call job_start('ls', {'out_mode' : 'abc'})", 'E475:')
  call assert_fails("call job_start('ls', {'err_mode' : 'abc'})", 'E475:')
  call assert_fails("call job_start('ls',
        \ {'in_io' : 'buffer', 'in_buf' : 99999})", 'E86:')
  call assert_fails("call job_start('ls',
        \ {'out_io' : 'buffer', 'out_buf' : 99999})", 'E86:')
  call assert_fails("call job_start('ls',
        \ {'err_io' : 'buffer', 'err_buf' : 99999})", 'E86:')

  call assert_fails("call job_start('ls',
        \ {'in_io' : 'buffer', 'in_buf' : -1})", 'E475:')
  call assert_fails("call job_start('ls',
        \ {'out_io' : 'buffer', 'out_buf' : -1})", 'E475:')
  call assert_fails("call job_start('ls',
        \ {'err_io' : 'buffer', 'err_buf' : -1})", 'E475:')

  let cmd = has('win32') ? "cmd /c dir" : "ls"

  set nomodifiable
  call assert_fails("call job_start(cmd,
        \ {'out_io' : 'buffer', 'out_buf' :" .. bufnr() .. "})", 'E21:')
  call assert_fails("call job_start(cmd,
        \ {'err_io' : 'buffer', 'err_buf' :" .. bufnr() .. "})", 'E21:')
  set modifiable

  call assert_fails("call job_start(cmd, {'in_io' : 'buffer'})", 'E915:')

  edit! XXX
  let bnum = bufnr()
  enew
  call assert_fails("call job_start(cmd,
        \ {'in_io' : 'buffer', 'in_buf' : bnum})", 'E918:')

  " Empty job tests
  " This was crashing on MS-Windows.
  call assert_fails('let job = job_start([""])', 'E474:')
  call assert_fails('let job = job_start(["   "])', 'E474:')
  call assert_fails('let job = job_start("")', 'E474:')
  call assert_fails('let job = job_start("   ")', 'E474:')
  call assert_fails('let job = job_start(["ls", []])', 'E730:')
  call assert_fails('call job_setoptions(test_null_job(), {})', 'E916:')
  %bw!
endfunc

func Test_job_stop_immediately()
  " With valgrind this causes spurious leak reports
  CheckNotValgrind

  let g:job = job_start([s:python, '-c', 'import time;time.sleep(10)'])
  try
    eval g:job->job_stop()
    call WaitForAssert({-> assert_equal('dead', job_status(g:job))})
  finally
    call job_stop(g:job, 'kill')
    unlet g:job
  endtry
endfunc

func Test_null_job_eval()
  call assert_fails('eval test_null_job()->eval()', 'E121:')
endfunc

" This was leaking memory.
func Test_partial_in_channel_cycle()
  let d = {}
  let d.a = function('string', [d])
  try
    let d.b = ch_open('nowhere:123', {'close_cb': d.a})
    call test_garbagecollect_now()
  catch
    call assert_exception('E901:')
  endtry
  unlet d
endfunc

func Test_using_freed_memory()
  let g:a = job_start(['ls'])
  sleep 10m
  call test_garbagecollect_now()
endfunc

func Test_collapse_buffers()
  let g:test_is_flaky = 1
  CheckExecutable cat

  sp test_channel.vim
  let g:linecount = line('$')
  close
  split testout
  1,$delete
  call job_start('cat test_channel.vim', {'out_io': 'buffer', 'out_name': 'testout'})
  call WaitForAssert({-> assert_inrange(g:linecount, g:linecount + 1, line('$'))})
  bwipe!
endfunc

func Test_write_to_deleted_buffer()
  CheckExecutable echo
  CheckFeature quickfix

  let job = job_start('echo hello', {'out_io': 'buffer', 'out_name': 'test_buffer', 'out_msg': 0})
  let bufnr = bufnr('test_buffer')
  call WaitForAssert({-> assert_equal(['hello'], getbufline(bufnr, 1, '$'))})
  call assert_equal('nofile', getbufvar(bufnr, '&buftype'))
  call assert_equal('hide', getbufvar(bufnr, '&bufhidden'))

  bdel test_buffer
  call assert_equal([], getbufline(bufnr, 1, '$'))

  let job = job_start('echo hello', {'out_io': 'buffer', 'out_name': 'test_buffer', 'out_msg': 0})
  call WaitForAssert({-> assert_equal(['hello'], getbufline(bufnr, 1, '$'))})
  call assert_equal('nofile', getbufvar(bufnr, '&buftype'))
  call assert_equal('hide', getbufvar(bufnr, '&bufhidden'))

  bwipe! test_buffer
endfunc

func Test_cmd_parsing()
  CheckUnix

  call assert_false(filereadable("file with space"))
  let job = job_start('touch "file with space"')
  call WaitForAssert({-> assert_true(filereadable("file with space"))})
  call delete("file with space")

  let job = job_start('touch file\ with\ space')
  call WaitForAssert({-> assert_true(filereadable("file with space"))})
  call delete("file with space")
endfunc

func Test_raw_passes_nul()
  CheckExecutable cat

  " Test lines from the job containing NUL are stored correctly in a buffer.
  new
  call setline(1, ["asdf\nasdf", "xxx\n", "\nyyy"])
  w! Xtestread
  bwipe!
  split testout
  1,$delete
  call job_start('cat Xtestread', {'out_io': 'buffer', 'out_name': 'testout'})
  call WaitFor('line("$") > 2')
  call assert_equal("asdf\nasdf", getline(1))
  call assert_equal("xxx\n", getline(2))
  call assert_equal("\nyyy", getline(3))

  call delete('Xtestread')
  bwipe!

  " Test lines from a buffer with NUL bytes are written correctly to the job.
  new mybuffer
  call setline(1, ["asdf\nasdf", "xxx\n", "\nyyy"])
  let g:Ch_job = job_start('cat', {'in_io': 'buffer', 'in_name': 'mybuffer', 'out_io': 'file', 'out_name': 'Xtestwrite'})
  call WaitForAssert({-> assert_equal("dead", job_status(g:Ch_job))})
  bwipe!
  split Xtestwrite
  call assert_equal("asdf\nasdf", getline(1))
  call assert_equal("xxx\n", getline(2))
  call assert_equal("\nyyy", getline(3))
  call assert_equal(-1, match(s:get_resources(), '\(^\|/\)Xtestwrite$'))

  call delete('Xtestwrite')
  bwipe!
endfunc

func Test_read_nonl_line()
  let g:linecount = 0
  let arg = 'import sys;sys.stdout.write("1\n2\n3")'
  call job_start([s:python, '-c', arg], {'callback': {-> execute('let g:linecount += 1')}})
  call WaitForAssert({-> assert_equal(3, g:linecount)})
  unlet g:linecount
endfunc

func Test_read_nonl_in_close_cb()
  func s:close_cb(ch)
    while ch_status(a:ch) == 'buffered'
      let g:out .= ch_read(a:ch)
    endwhile
  endfunc

  let g:out = ''
  let arg = 'import sys;sys.stdout.write("1\n2\n3")'
  call job_start([s:python, '-c', arg], {'close_cb': function('s:close_cb')})
  call test_garbagecollect_now()
  call WaitForAssert({-> assert_equal('123', g:out)})
  unlet g:out
  delfunc s:close_cb
endfunc

func Test_read_from_terminated_job()
  let g:linecount = 0
  let arg = 'import os,sys;os.close(1);sys.stderr.write("test\n")'
  call job_start([s:python, '-c', arg], {'callback': {-> execute('let g:linecount += 1')}})
  call WaitForAssert({-> assert_equal(1, g:linecount)})
  call test_garbagecollect_now()
  unlet g:linecount
endfunc

func Test_job_start_windows()
  CheckMSWindows

  " Check that backslash in $COMSPEC is handled properly.
  let g:echostr = ''
  let cmd = $COMSPEC . ' /c echo 123'
  let job = job_start(cmd, {'callback': {ch,msg -> execute(":let g:echostr .= msg")}})
  let info = job_info(job)
  call assert_equal([$COMSPEC, '/c', 'echo', '123'], info.cmd)

  call WaitForAssert({-> assert_equal("123", g:echostr)})
  unlet g:echostr
endfunc

func Test_env()
  let g:envstr = ''
  if has('win32')
    let cmd = ['cmd', '/c', 'echo %FOO%']
  else
    let cmd = [&shell, &shellcmdflag, 'echo $FOO']
  endif
  call assert_fails('call job_start(cmd, {"env": 1})', 'E475:')
  let job = job_start(cmd, {'callback': {ch,msg -> execute(":let g:envstr .= msg")}, 'env': {'FOO': 'bar'}})
  if WaitForAssert({-> assert_equal("bar", g:envstr)}, 500) != 0
    call add(v:errors, "Job status: " .. string(job->job_info()))
  endif
  unlet g:envstr
endfunc

func Test_cwd()
  let g:test_is_flaky = 1
  let g:envstr = ''
  if has('win32')
    let expect = $TEMP
    let cmd = ['cmd', '/c', 'echo %CD%']
  else
    let expect = $HOME
    let cmd = ['pwd']
  endif
  let job = job_start(cmd, {'callback': {ch,msg -> execute(":let g:envstr .= msg")}, 'cwd': expect})
  try
    call WaitForAssert({-> assert_notequal("", g:envstr)})
    " There may be a trailing slash or not, ignore it
    let expect = substitute(expect, '[/\\]$', '', '')
    let g:envstr = substitute(g:envstr, '[/\\]$', '', '')
    " on CI there can be /private prefix or not, ignore it
    if $CI != '' && stridx(expect, '/private/') == 0
      let expect = expect[8:]
    endif
    if $CI != '' && stridx(g:envstr, '/private/') == 0
      let g:envstr = g:envstr[8:]
    endif
    call assert_equal(expect, g:envstr)
  finally
    call job_stop(job)
    unlet g:envstr
  endtry
endfunc

function Ch_test_close_lambda(port)
  let handle = ch_open(s:address(a:port), s:chopt)
  if ch_status(handle) == "fail"
    call assert_report("Can't open channel")
    return
  endif
  let g:Ch_close_ret = ''
  call ch_setoptions(handle, {'close_cb': {ch -> execute("let g:Ch_close_ret = 'closed'")}})
  call test_garbagecollect_now()

  call assert_equal('', ch_evalexpr(handle, 'close me'))
  call WaitForAssert({-> assert_equal('closed', g:Ch_close_ret)})
endfunc

func Test_close_lambda()
  call s:run_server('Ch_test_close_lambda')
endfunc

func Test_close_lambda_ipv6()
  CheckIPv6
  call Test_close_lambda()
endfunc

func Test_close_lambda_unix()
  CheckUnix
  call Test_close_lambda()
  call delete('Xtestsocket')
endfunc

func s:test_list_args(cmd, out, remove_lf)
  try
    let g:out = ''
    let job = job_start([s:python, '-c', a:cmd], {'callback': {ch, msg -> execute('let g:out .= msg')}, 'out_mode': 'raw'})
    try
      call WaitFor('"" != g:out')
    catch
      call add(v:errors, "Job status: " .. string(job->job_info()))
      throw v:exception
    endtry
    if has('win32')
      let g:out = substitute(g:out, '\r', '', 'g')
    endif
    if a:remove_lf
      let g:out = substitute(g:out, '\n$', '', 'g')
    endif
    call assert_equal(a:out, g:out)
  finally
    call job_stop(job)
    unlet g:out
  endtry
endfunc

func Test_list_args()
  call s:test_list_args('import sys;sys.stdout.write("hello world")', "hello world", 0)
  call s:test_list_args('import sys;sys.stdout.write("hello\nworld")', "hello\nworld", 0)
  call s:test_list_args('import sys;sys.stdout.write(''hello\nworld'')', "hello\nworld", 0)
  call s:test_list_args('import sys;sys.stdout.write(''hello"world'')', "hello\"world", 0)
  call s:test_list_args('import sys;sys.stdout.write(''hello^world'')', "hello^world", 0)
  call s:test_list_args('import sys;sys.stdout.write("hello&&world")', "hello&&world", 0)
  call s:test_list_args('import sys;sys.stdout.write(''hello\\world'')', "hello\\world", 0)
  call s:test_list_args('import sys;sys.stdout.write(''hello\\\\world'')', "hello\\\\world", 0)
  call s:test_list_args('import sys;sys.stdout.write("hello\"world\"")', 'hello"world"', 0)
  call s:test_list_args('import sys;sys.stdout.write("h\"ello worl\"d")', 'h"ello worl"d', 0)
  call s:test_list_args('import sys;sys.stdout.write("h\"e\\\"llo wor\\\"l\"d")', 'h"e\"llo wor\"l"d', 0)
  call s:test_list_args('import sys;sys.stdout.write("h\"e\\\"llo world")', 'h"e\"llo world', 0)
  call s:test_list_args('import sys;sys.stdout.write("hello\tworld")', "hello\tworld", 0)

  " tests which not contain spaces in the argument
  call s:test_list_args('print("hello\nworld")', "hello\nworld", 1)
  call s:test_list_args('print(''hello\nworld'')', "hello\nworld", 1)
  call s:test_list_args('print(''hello"world'')', "hello\"world", 1)
  call s:test_list_args('print(''hello^world'')', "hello^world", 1)
  call s:test_list_args('print("hello&&world")', "hello&&world", 1)
  call s:test_list_args('print(''hello\\world'')', "hello\\world", 1)
  call s:test_list_args('print(''hello\\\\world'')', "hello\\\\world", 1)
  call s:test_list_args('print("hello\"world\"")', 'hello"world"', 1)
  call s:test_list_args('print("hello\tworld")', "hello\tworld", 1)
endfunc

func Test_keep_pty_open()
  CheckUnix

  let job = job_start(s:python . ' -c "import time;time.sleep(0.2)"',
        \ {'out_io': 'null', 'err_io': 'null', 'pty': 1})
  let elapsed = WaitFor({-> job_status(job) ==# 'dead'})
  call assert_inrange(200, 1000, elapsed)
  call job_stop(job)
endfunc

func Test_job_start_in_timer()
  CheckFeature timers
  CheckFunction reltimefloat

  func OutCb(chan, msg)
    let g:val += 1
  endfunc

  func ExitCb(job, status)
    let g:val += 1
    call Resume()
  endfunc

  func TimerCb(timer)
    if has('win32')
      let cmd = ['cmd', '/c', 'echo.']
    else
      let cmd = ['echo']
    endif
    let g:job = job_start(cmd, {'out_cb': 'OutCb', 'exit_cb': 'ExitCb'})
    call substitute(repeat('a', 100000), '.', '', 'g')
  endfunc

  " We should be interrupted before 'updatetime' elapsed.
  let g:val = 0
  call timer_start(1, 'TimerCb')
  let elapsed = Standby(&ut)
  call assert_inrange(1, &ut / 2, elapsed)

  " Wait for both OutCb() and ExitCb() to have been called before deleting
  " them.
  call WaitForAssert({-> assert_equal(2, g:val)})
  call job_stop(g:job)

  delfunc OutCb
  delfunc ExitCb
  delfunc TimerCb
  unlet! g:val
  unlet! g:job
endfunc

func Test_raw_large_data()
  try
    let g:out = ''
    let job = job_start(s:python . " test_channel_pipe.py",
          \ {'mode': 'raw', 'drop': 'never', 'noblock': 1,
          \  'callback': {ch, msg -> execute('let g:out .= msg')}})

    let outlen = 79999
    let want = repeat('X', outlen) . "\n"
    eval job->ch_sendraw(want)
    call WaitFor({-> len(g:out) >= outlen}, 10000)
    call WaitForAssert({-> assert_equal("dead", job_status(job))})
    call assert_equal(want, substitute(g:out, '\r', '', 'g'))
  finally
    call job_stop(job)
    unlet g:out
  endtry
endfunc

func Test_no_hang_windows()
  CheckMSWindows

  try
    let job = job_start(s:python . " test_channel_pipe.py busy",
          \ {'mode': 'raw', 'drop': 'never', 'noblock': 0})
    call assert_fails('call ch_sendraw(job, repeat("X", 80000))', 'E631:')
  finally
    call job_stop(job)
  endtry
endfunc

func Test_job_exitval_and_termsig()
  CheckUnix

  " Terminate job normally
  let cmd = ['echo']
  let job = job_start(cmd)
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  let info = job_info(job)
  call assert_equal(0, info.exitval)
  call assert_equal("", info.termsig)

  " Terminate job by signal
  let cmd = ['sleep', '10']
  let job = job_start(cmd)
  " 10m usually works but 50m is needed when running Valgrind
  sleep 50m
  call job_stop(job)
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  let info = job_info(job)
  call assert_equal(-1, info.exitval)
  call assert_equal("term", info.termsig)
endfunc

func Test_job_tty_in_out()
  CheckUnix

  call writefile(['test'], 'Xtestin', 'D')
  let in_opts = [{},
        \ {'in_io': 'null'},
        \ {'in_io': 'file', 'in_name': 'Xtestin'}]
  let out_opts = [{},
        \ {'out_io': 'null'},
        \ {'out_io': 'file', 'out_name': 'Xtestout'}]
  let err_opts = [{},
        \ {'err_io': 'null'},
        \ {'err_io': 'file', 'err_name': 'Xtesterr'},
        \ {'err_io': 'out'}]
  let opts = []

  for in_opt in in_opts
    let x = copy(in_opt)
    for out_opt in out_opts
      let x = extend(copy(x), out_opt)
      for err_opt in err_opts
        let x = extend(copy(x), err_opt)
        let opts += [extend({'pty': 1}, x)]
      endfor
    endfor
  endfor

  for opt in opts
    let job = job_start('echo', opt)
    let info = job_info(job)
    let msg = printf('option={"in_io": "%s", "out_io": "%s", "err_io": "%s"}',
          \ get(opt, 'in_io', 'tty'),
          \ get(opt, 'out_io', 'tty'),
          \ get(opt, 'err_io', 'tty'))

    if !has_key(opt, 'in_io') || !has_key(opt, 'out_io') || !has_key(opt, 'err_io')
      call assert_notequal('', info.tty_in, msg)
    else
      call assert_equal('', info.tty_in, msg)
    endif
    call assert_equal(info.tty_in, info.tty_out, msg)

    call WaitForAssert({-> assert_equal('dead', job_status(job))})
  endfor

  call delete('Xtestout')
  call delete('Xtesterr')
endfunc

" Do this last, it stops any channel log.
func Test_zz_nl_err_to_out_pipe()

  eval 'Xlog'->ch_logfile()
  call ch_log('Test_zz_nl_err_to_out_pipe()')
  let job = job_start(s:python . " test_channel_pipe.py", {'err_io': 'out'})
  call assert_equal("run", job_status(job))
  try
    let handle = job_getchannel(job)
    call ch_sendraw(handle, "echo something\n")
    call assert_equal("something", ch_readraw(handle))

    call ch_sendraw(handle, "echoerr wrong\n")
    call assert_equal("wrong", ch_readraw(handle))
  finally
    call job_stop(job)
    call ch_logfile('')
    let loglines = readfile('Xlog')
    call assert_true(len(loglines) > 10)
    let found_test = 0
    let found_send = 0
    let found_recv = 0
    let found_stop = 0
    for l in loglines
      if l =~ 'Test_zz_nl_err_to_out_pipe'
	let found_test = 1
      endif
      if l =~ 'SEND on.*echo something'
	let found_send = 1
      endif
      if l =~ 'RECV on.*something'
	let found_recv = 1
      endif
      if l =~ 'Stopping job with'
	let found_stop = 1
      endif
    endfor
    call assert_equal(1, found_test)
    call assert_equal(1, found_send)
    call assert_equal(1, found_recv)
    call assert_equal(1, found_stop)
    " On MS-Windows need to sleep for a moment to be able to delete the file.
    sleep 10m
    call delete('Xlog')
  endtry
endfunc

" Do this last, it stops any channel log.
func Test_zz_ch_log()
  call ch_logfile('Xlog', 'w')
  call ch_log('hello there')
  call ch_log('%s%s')
  call ch_logfile('')
  let text = readfile('Xlog')
  call assert_match("start log session", text[0])
  call assert_match("ch_log(): hello there", text[1])
  call assert_match("%s%s", text[2])
  call mkdir("Xchlogdir1", 'D')
  call assert_fails("call ch_logfile('Xchlogdir1')", 'E484:')

  call delete('Xlog')
endfunc

func Test_issue_5150()
  if has('win32')
    let cmd = 'cmd /c pause'
  else
    let cmd = 'grep foo'
  endif

  let g:job = job_start(cmd, {})
  sleep 50m  " give the job time to start
  call job_stop(g:job)
  call WaitForAssert({-> assert_equal(-1, job_info(g:job).exitval)})

  let g:job = job_start(cmd, {})
  sleep 50m
  call job_stop(g:job, 'term')
  call WaitForAssert({-> assert_equal(-1, job_info(g:job).exitval)})

  let g:job = job_start(cmd, {})
  sleep 50m
  call job_stop(g:job, 'kill')
  call WaitForAssert({-> assert_equal(-1, job_info(g:job).exitval)})
endfunc

func Test_issue_5485()
  let $VAR1 = 'global'
  let g:Ch_reply = ""
  let l:job = job_start([&shell, &shellcmdflag, has('win32') ? 'echo %VAR1% %VAR2%' : 'echo $VAR1 $VAR2'], {'env': {'VAR1': 'local', 'VAR2': 'local'}, 'callback': 'Ch_handler'})
  let g:Ch_job = l:job
  call WaitForAssert({-> assert_equal("local local", trim(g:Ch_reply))})
  unlet $VAR1
endfunc

func Test_job_trailing_space_unix()
  CheckUnix
  CheckExecutable cat

  let job = job_start("cat ", #{in_io: 'null'})
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  call assert_equal(0, job_info(job).exitval)

  call delete('Xtestsocket')
endfunc

func Test_ch_getbufnr()
  let ch = test_null_channel()
  call assert_equal(-1, ch_getbufnr(ch, 'in'))
  call assert_equal(-1, ch_getbufnr(ch, 'out'))
  call assert_equal(-1, ch_getbufnr(ch, 'err'))
  call assert_equal(-1, ch_getbufnr(ch, ''))
endfunc

" Test for unsupported options passed to ch_status()
func Test_invalid_job_chan_options()
  let ch = test_null_channel()
  let invalid_opts = [
        \ {'in_io' : 'null'},
        \ {'out_io' : 'null'},
        \ {'err_io' : 'null'},
        \ {'mode' : 'json'},
        \ {'out_mode' : 'json'},
        \ {'err_mode' : 'json'},
        \ {'noblock' : 1},
        \ {'in_name' : '/a/b'},
        \ {'pty' : 1},
        \ {'in_buf' : 1},
        \ {'out_buf' : 1},
        \ {'err_buf' : 1},
        \ {'out_modifiable' : 1},
        \ {'err_modifiable' : 1},
        \ {'out_msg' : 1},
        \ {'err_msg' : 1},
        \ {'in_top' : 1},
        \ {'in_bot' : 1},
        \ {'channel' : ch},
        \ {'callback' : ''},
        \ {'out_cb' : ''},
        \ {'err_cb' : ''},
        \ {'close_cb' : ''},
        \ {'exit_cb' : ''},
        \ {'term_opencmd' : ''},
        \ {'eof_chars' : ''},
        \ {'term_rows' : 10},
        \ {'term_cols' : 10},
        \ {'vertical' : 0},
        \ {'curwin' : 1},
        \ {'bufnr' : 1},
        \ {'hidden' : 0},
        \ {'norestore' : 0},
        \ {'term_kill' : 'kill'},
        \ {'tty_type' : ''},
        \ {'term_highlight' : ''},
        \ {'env' : {}},
        \ {'cwd' : ''},
        \ {'timeout' : 0},
        \ {'out_timeout' : 0},
        \ {'err_timeout' : 0},
        \ {'id' : 0},
        \ {'stoponexit' : ''},
        \ {'block_write' : 1}
        \ ]
  if has('gui')
    call add(invalid_opts, {'ansi_colors' : []})
  endif

  for opt in invalid_opts
    call assert_fails("let x = ch_status(ch, opt)", 'E475:')
  endfor
  call assert_equal('fail', ch_status(ch, test_null_dict()))
endfunc

" Test for passing the command and the arguments as List on MS-Windows
func Test_job_with_list_args()
  CheckMSWindows

  enew!
  let bnum = bufnr()
  let job = job_start(['cmd', '/c', 'echo', 'Hello', 'World'], {'out_io' : 'buffer', 'out_buf' : bnum})
  call WaitForAssert({-> assert_equal("dead", job_status(job))})
  call assert_equal('Hello World', getline(1))
  %bw!
endfunc

func ExitCb_cb_with_input(job, status)
  call feedkeys(":\<C-u>echo input('', 'default')\<CR>\<CR>", 'nx')
  call assert_equal('default', Screenline(&lines))
  let g:wait_exit_cb = 0
endfunc

func Test_cb_with_input()
  let g:wait_exit_cb = 1

  if has('win32')
    let cmd = 'cmd /c echo "Vim''s test"'
  else
    let cmd = 'echo "Vim''s test"'
  endif

  let job = job_start(cmd, {'out_cb': 'ExitCb_cb_with_input'})
  call WaitFor({-> job_status(job) == "dead"})
  call WaitForAssert({-> assert_equal(0, g:wait_exit_cb)})

  unlet g:wait_exit_cb
endfunc

function s:HandleBufEnter() abort
  let queue = []
  let job = job_start(['date'], {'callback': { j, d -> add(queue, d) }})
  while empty(queue)
    sleep! 10m
  endwhile
endfunction

func Test_parse_messages_in_autocmd()
  CheckUnix

  " Check that in the BufEnter autocommand events are being handled
  augroup bufenterjob
    autocmd!
    autocmd BufEnter Xbufenterjob call s:HandleBufEnter()
  augroup END

  only
  split Xbufenterjob
  wincmd p
  redraw

  close
  augroup bufenterjob
    autocmd!
  augroup END
endfunc

func Test_job_start_with_invalid_argument()
  call assert_fails('call job_start([0zff])', 'E976:')
endfunc

" Process requests received from the LSP server
func LspProcessServerRequests(chan, msg)
  if a:msg['method'] == 'server-req-in-middle'
        \ && a:msg['params']['text'] == 'server-req'
    call ch_sendexpr(a:chan, #{method: 'server-req-in-middle-resp',
          \ id: a:msg['id'], params: #{text: 'client-resp'}})
  endif
endfunc

" LSP channel message callback function
func LspCb(chan, msg)
  call add(g:lspNotif, a:msg)
  if a:msg->has_key('method')
    call LspProcessServerRequests(a:chan, a:msg)
  endif
endfunc

" LSP one-time message callback function (used for ch_sendexpr())
func LspOtCb(chan, msg)
  call add(g:lspOtMsgs, a:msg)
  if a:msg->has_key('method')
    call LspProcessServerRequests(a:chan, a:msg)
  endif
endfunc

" Test for the 'lsp' channel mode
func LspTests(port)
  " call ch_logfile('Xlspclient.log', 'w')
  let ch = ch_open(s:localhost .. a:port, #{mode: 'lsp', callback: 'LspCb'})
  if ch_status(ch) == "fail"
    call assert_report("Can't open the lsp channel")
    return
  endif

  " check for channel information
  let info = ch_info(ch)
  call assert_equal('LSP', info.sock_mode)

  " Evaluate an expression
  let resp = ch_evalexpr(ch, #{method: 'simple-rpc', params: [10, 20]})
  call assert_false(empty(resp))
  call assert_equal(#{id: 1, jsonrpc: '2.0', result: 'simple-rpc'}, resp)

  " Evaluate an expression. While waiting for the response, a notification
  " message is delivered.
  let g:lspNotif = []
  let resp = ch_evalexpr(ch, #{method: 'rpc-with-notif', params: {'v': 10}})
  call assert_false(empty(resp))
  call assert_equal(#{id: 2, jsonrpc: '2.0', result: 'rpc-with-notif-resp'},
        \ resp)
  call assert_equal([#{jsonrpc: '2.0', result: 'rpc-with-notif-notif'}],
        \ g:lspNotif)

  " Wrong payload notification test
  let g:lspNotif = []
  let r = ch_sendexpr(ch, #{method: 'wrong-payload', params: {}})
  call assert_equal({}, r)
  " Send a ping to wait for all the notification messages to arrive
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  call assert_equal([#{jsonrpc: '2.0', result: 'wrong-payload'}], g:lspNotif)

  " Test for receiving a response with incorrect 'id' and additional
  " notification messages while evaluating an expression.
  let g:lspNotif = []
  let resp = ch_evalexpr(ch, #{method: 'rpc-resp-incorrect-id',
        \ params: {'a': [1, 2]}})
  call assert_false(empty(resp))
  call assert_equal(#{id: 4, jsonrpc: '2.0',
        \ result: 'rpc-resp-incorrect-id-4'}, resp)
  call assert_equal([#{jsonrpc: '2.0', result: 'rpc-resp-incorrect-id-1'},
        \ #{jsonrpc: '2.0', result: 'rpc-resp-incorrect-id-2'},
        \ #{jsonrpc: '2.0', id: 1, result: 'rpc-resp-incorrect-id-3'}],
        \ g:lspNotif)

  " simple notification test
  let g:lspNotif = []
  call ch_sendexpr(ch, #{method: 'simple-notif', params: [#{a: 10, b: []}]})
  " Send a ping to wait for all the notification messages to arrive
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  call assert_equal([#{jsonrpc: '2.0', result: 'simple-notif'}], g:lspNotif)

  " multiple notifications test
  let g:lspNotif = []
  call ch_sendexpr(ch, #{method: 'multi-notif', params: [#{a: {}, b: {}}]})
  " Send a ping to wait for all the notification messages to arrive
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  call assert_equal([#{jsonrpc: '2.0', result: 'multi-notif1'},
        \ #{jsonrpc: '2.0', result: 'multi-notif2'}], g:lspNotif)

  " Test for sending a message with an identifier.
  let g:lspNotif = []
  call ch_sendexpr(ch, #{method: 'msg-with-id', id: 93, params: #{s: 'str'}})
  " Send a ping to wait for all the notification messages to arrive
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  call assert_equal([#{jsonrpc: '2.0', id: 93, result: 'msg-with-id'}],
        \ g:lspNotif)

  " Test for setting the 'id' value in a request message
  let resp = ch_evalexpr(ch, #{method: 'ping', id: 1, params: {}})
  call assert_equal(#{id: 8, jsonrpc: '2.0', result: 'alive'}, resp)

  " Test for using a one time callback function to process a response
  let g:lspOtMsgs = []
  let r = ch_sendexpr(ch, #{method: 'msg-specific-cb', params: {}},
        \ #{callback: 'LspOtCb'})
  call assert_equal(9, r.id)
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  call assert_equal([#{id: 9, jsonrpc: '2.0', result: 'msg-specific-cb'}],
        \ g:lspOtMsgs)

  " Test for generating a request message from the other end (server)
  let g:lspNotif = []
  call ch_sendexpr(ch, #{method: 'server-req', params: #{}})
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  call assert_equal([{'id': 201, 'jsonrpc': '2.0',
        \ 'result': {'method': 'checkhealth', 'params': {'a': 20}}}],
        \ g:lspNotif)

  " Test for sending a message without an id
  let g:lspNotif = []
  call ch_sendexpr(ch, #{method: 'echo', params: #{s: 'msg-without-id'}})
  " Send a ping to wait for all the notification messages to arrive
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  call assert_equal([#{jsonrpc: '2.0', result:
        \ #{method: 'echo', jsonrpc: '2.0', params: #{s: 'msg-without-id'}}}],
        \ g:lspNotif)

  " Test for sending a notification message with an id
  let g:lspNotif = []
  call ch_sendexpr(ch, #{method: 'echo', id: 110, params: #{s: 'msg-with-id'}})
  " Send a ping to wait for all the notification messages to arrive
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  call assert_equal([#{jsonrpc: '2.0', result:
        \ #{method: 'echo', jsonrpc: '2.0', id: 110,
        \ params: #{s: 'msg-with-id'}}}], g:lspNotif)

  " Test for processing the extra fields in the HTTP header
  let resp = ch_evalexpr(ch, #{method: 'extra-hdr-fields', params: {}})
  call assert_equal({'id': 14, 'jsonrpc': '2.0', 'result': 'extra-hdr-fields'},
        \ resp)

  " Test for processing delayed payload
  let resp = ch_evalexpr(ch, #{method: 'delayed-payload', params: {}})
  call assert_equal({'id': 15, 'jsonrpc': '2.0', 'result': 'delayed-payload'},
        \ resp)

  " Test for processing a HTTP header without the Content-Length field
  let resp = ch_evalexpr(ch, #{method: 'hdr-without-len', params: {}},
        \ #{timeout: 200})
  call assert_equal({}, resp)
  " send a ping to make sure communication still works
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)

  " Test for processing a HTTP header with wrong length
  let resp = ch_evalexpr(ch, #{method: 'hdr-with-wrong-len', params: {}},
        \ #{timeout: 200})
  call assert_equal({}, resp)
  " send a ping to make sure communication still works
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)

  " Test for processing a HTTP header with negative length
  let resp = ch_evalexpr(ch, #{method: 'hdr-with-negative-len', params: {}},
        \ #{timeout: 200})
  call assert_equal({}, resp)
  " send a ping to make sure communication still works
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)

  " Test for an empty header
  let resp = ch_evalexpr(ch, #{method: 'empty-header', params: {}},
        \ #{timeout: 200})
  call assert_equal({}, resp)
  " send a ping to make sure communication still works
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)

  " Test for an empty payload
  let resp = ch_evalexpr(ch, #{method: 'empty-payload', params: {}},
        \ #{timeout: 200})
  call assert_equal({}, resp)
  " send a ping to make sure communication still works
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)

  " Test for a large payload
  let content = repeat('abcdef', 11000)
  let resp = ch_evalexpr(ch, #{method: 'large-payload',
        \ params: #{text: content}})
  call assert_equal(#{jsonrpc: '2.0', id: 26, result:
        \ #{method: 'large-payload', jsonrpc: '2.0', id: 26,
        \ params: #{text: content}}}, resp)
  " send a ping to make sure communication still works
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)

  " Test for processing a request message from the server while the client
  " is waiting for a response with the same identifier (sync-rpc)
  let g:lspNotif = []
  let resp = ch_evalexpr(ch, #{method: 'server-req-in-middle',
        \ params: #{text: 'client-req'}})
  call assert_equal(#{jsonrpc: '2.0', id: 28,
        \ result: #{text: 'server-resp'}}, resp)
  call assert_equal([
        \ #{id: -1, jsonrpc: '2.0', method: 'server-req-in-middle',
        \   params: #{text: 'server-notif'}},
        \ #{id: 28, jsonrpc: '2.0', method: 'server-req-in-middle',
        \   params: #{text: 'server-req'}}], g:lspNotif)

  " Test for processing a request message from the server while the client
  " is waiting for a response with the same identifier (async-rpc using the
  " channel callback function)
  let g:lspNotif = []
  call ch_sendexpr(ch, #{method: 'server-req-in-middle', id: 500,
        \ params: #{text: 'client-req'}})
  " Send three pings to wait for all the notification messages to arrive
  for i in range(3)
    call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  endfor
  call assert_equal([
        \ #{id: -1, jsonrpc: '2.0', method: 'server-req-in-middle',
        \   params: #{text: 'server-notif'}},
        \ #{id: 500, jsonrpc: '2.0', method: 'server-req-in-middle',
        \   params: #{text: 'server-req'}},
        \ #{id: 500, jsonrpc: '2.0', result: #{text: 'server-resp'}}
        \ ], g:lspNotif)

  " Test for processing a request message from the server while the client
  " is waiting for a response with the same identifier (async-rpc using a
  " one-time callback function)
  let g:lspNotif = []
  let g:lspOtMsgs = []
  call ch_sendexpr(ch, #{method: 'server-req-in-middle',
        \ params: #{text: 'client-req'}}, #{callback: 'LspOtCb'})
  " Send a ping to wait for all the notification messages to arrive
  for i in range(3)
    call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  endfor
  call assert_equal([
        \ #{id: 32, jsonrpc: '2.0', result: #{text: 'server-resp'}}],
        \ g:lspOtMsgs)
  call assert_equal([
        \ #{id: -1, jsonrpc: '2.0', method: 'server-req-in-middle',
        \ params: #{text: 'server-notif'}},
        \ #{id: 32, jsonrpc: '2.0', method: 'server-req-in-middle',
        \ params: {'text': 'server-req'}}], g:lspNotif)

  " Test for invoking an unsupported method
  let resp = ch_evalexpr(ch, #{method: 'xyz', params: {}}, #{timeout: 200})
  call assert_equal({}, resp)

  " Test for sending a message without a callback function. Notification
  " message should be dropped but RPC response should not be dropped.
  call ch_setoptions(ch, #{callback: ''})
  let g:lspNotif = []
  call ch_sendexpr(ch, #{method: 'echo', params: #{s: 'no-callback'}})
  " Send a ping to wait for all the notification messages to arrive
  call assert_equal('alive', ch_evalexpr(ch, #{method: 'ping'}).result)
  call assert_equal([], g:lspNotif)
  " Restore the callback function
  call ch_setoptions(ch, #{callback: 'LspCb'})

  " " Test for sending a raw message
  " let g:lspNotif = []
  " let s = "Content-Length: 62\r\n"
  " let s ..= "Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n"
  " let s ..= "\r\n"
  " let s ..= '{"method":"echo","jsonrpc":"2.0","params":{"m":"raw-message"}}'
  " call ch_sendraw(ch, s)
  " call ch_evalexpr(ch, #{method: 'ping'})
  " call assert_equal([{'jsonrpc': '2.0',
  "       \ 'result': {'method': 'echo', 'jsonrpc': '2.0',
  "       \ 'params': {'m': 'raw-message'}}}], g:lspNotif)

  " Invalid arguments to ch_evalexpr() and ch_sendexpr()
  call assert_fails('call ch_sendexpr(ch, #{method: "cookie", id: "cookie"})',
        \ 'E475:')
  call assert_fails('call ch_evalexpr(ch, #{method: "ping", id: [{}]})', 'E475:')
  call assert_fails('call ch_evalexpr(ch, [1, 2, 3])', 'E1206:')
  call assert_fails('call ch_sendexpr(ch, "abc")', 'E1206:')
  call assert_fails('call ch_evalexpr(ch, #{method: "ping"}, #{callback: "LspOtCb"})', 'E917:')
  " call ch_logfile('', 'w')
endfunc

func Test_channel_lsp_mode()
  call RunServer('test_channel_lsp.py', 'LspTests', [])
endfunc

" vim: shiftwidth=2 sts=2 expandtab
