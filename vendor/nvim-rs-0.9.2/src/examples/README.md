Several examples on how to use `nvim-rs` to implement RPC plugins for neovim.
Documentation can be found on
[`docs.rs`](https://docs.rs/nvim-rs/latest/nvim_rs/examples/index.html).


### Integration

To integrate an PRC plugin (i.e. a child process of neovim, as  opposed to e.g.
a GUI that embeds neovim as a child process) some scripting on the neovim side
is necessary. The following will use the `scorched_earth` binary from the
corresponding example, but can be used as a template for other plugins. To
understand the usage of `$rtp`, please read the [neovim
docs](https://neovim.io/doc/user/options.html#'runtimepath').

First, put the following into `$rtp/autoload/scorched_earth.vim`:

```vim
if ! exists('s:jobid')
  let s:jobid = 0
endif

let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')

if ! exists('g:scorched_earth_program')
  let g:scorched_earth_program = s:scriptdir . '/target/release/neovim-scorched-earth'
endif

function! scorchedEarth#init()
  call scorchedEarth#connect()
endfunction

function! scorchedEarth#connect()
  let result = s:StartJob()

  if 0 == result
    echoerr "scortched earth: cannot start rpc process"
  elseif -1 == result
    echoerr "scortched earth: rpc process is not executable"
  else
    let s:jobid = result
    call s:ConfigureJob(result)
  endif
endfunction

function! scorchedEarth#reset()
  let s:jobid = 0
endfunction

function! s:ConfigureJob(jobid)
  augroup scortchedEarth
    " clear all previous autocommands
    autocmd!

    autocmd VimLeavePre * :call s:StopJob()

    autocmd InsertChange * :call s:NotifyInsertChange()
    autocmd InsertEnter * :call s:NotifyInsertEnter()
    autocmd InsertLeave * :call s:NotifyInsertLeave()

    autocmd CursorMovedI * :call s:NotifyCursorMovedI()
  augroup END
endfunction

function! s:NotifyCursorMovedI()
  let [ bufnum, lnum, column, off ] = getpos('.')
  call rpcnotify(s:jobid, 'cursor-moved-i', lnum, column)
endfunction

function! s:NotifyInsertChange()
  let [ bufnum, lnum, column, off ] = getpos('.')
  call rpcnotify(s:jobid, 'insert-change', v:insertmode, lnum, column)
endfunction

function! s:NotifyInsertEnter()
  let [ bufnum, lnum, column, off ] = getpos('.')
  call rpcnotify(s:jobid, 'insert-enter', v:insertmode, lnum, column)
endfunction

function! s:NotifyInsertLeave()
  call rpcnotify(s:jobid, 'insert-leave')
endfunction

function! s:OnStderr(id, data, event) dict
  echom 'scorched earth: stderr: ' . join(a:data, "\n")
endfunction

function! s:StartJob()
  if 0 == s:jobid
    let id = jobstart([g:scorched_earth_program], { 'rpc': v:true, 'on_stderr': function('s:OnStderr') })
    return id
  else
    return 0
  endif
endfunction

function! s:StopJob()
  if 0 < s:jobid
    augroup scortchedEarth
      autocmd!    " clear all previous autocommands
    augroup END

    call rpcnotify(s:jobid, 'quit')
    let result = jobwait(s:jobid, 500)

    if -1 == result
      " kill the job
      call jobstop(s:jobid)
    endif

    " reset job id back to zero
    let s:jobid = 0
  endif
endfunction

call color#highlight('default ScorchedEarth', 'dddddd', '550000', 'bold', '', '')
```

This sets up the commands to actually run the plugin, register some
`autocommand`s, and stopping the plugin when shutting down neovim. The variable
`g:scorched_earth_program` needs to point to the binary of the plugin.

Secondly, to provide a command from neovim to run the plugin, put the following
into `$rtp/plugin/scorched_earth.vim`:

```vim
command! -nargs=0 ScorchedEarthConnect call scorchedEarth#connect()
```

Now you can start the plugin from neovim by running `:ScorchedEarthConnect`.
