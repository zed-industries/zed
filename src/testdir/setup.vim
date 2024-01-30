" Common preparations for running tests.

" Only load this once.
if 1

  " When using xterm version 377 the response to the modifyOtherKeys status
  " interferes with some tests.  Remove the request from the t_TI termcap
  " entry.
  let &t_TI = substitute(&t_TI, "\<Esc>\\[?4m", '', '')

  if exists('s:did_load')
    finish
  endif
  let s:did_load = 1
endif

" Make sure 'runtimepath' and 'packpath' does not include $HOME.
set rtp=$VIM/vimfiles,$VIMRUNTIME,$VIM/vimfiles/after
if has('packages')
  let &packpath = &rtp
endif

" Only when the +eval feature is present.
if 1
  " Make sure the .Xauthority file can be found after changing $HOME.
  if $XAUTHORITY == ''
    let $XAUTHORITY = $HOME . '/.Xauthority'
  endif

  " Avoid storing shell history.
  let $HISTFILE = ""

  " Have current $HOME available as $ORIGHOME.  $HOME is used for option
  " defaults before we get here, and test_mksession checks that.
  let $ORIGHOME = $HOME

  " Make sure $HOME does not get read or written.
  " It must exist, gnome tries to create $HOME/.gnome2
  let $HOME = getcwd() . '/XfakeHOME'
  if !isdirectory($HOME)
    call mkdir($HOME)
  endif
endif
