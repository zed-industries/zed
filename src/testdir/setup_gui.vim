" Common preparations for running GUI tests.

let g:x11_based_gui = has('gui_motif')
	\ || has('gui_gtk2') || has('gui_gnome') || has('gui_gtk3')

" Reasons for 'skipped'.
let g:not_supported   = "Skipped: Feature/Option not supported by this GUI: "
let g:not_hosted      = "Skipped: Test not hosted by the system/environment"

" For KDE set a font, empty 'guifont' may cause a hang.
func GUISetUpCommon()
  if has("gui_kde")
    set guifont=Courier\ 10\ Pitch/8/-1/5/50/0/0/0/0/0
  endif

  " Gnome insists on creating $HOME/.gnome2/, set $HOME to avoid changing the
  " actual home directory.  But avoid triggering fontconfig by setting the
  " cache directory.  Only needed for Unix.
  if $XDG_CACHE_HOME == '' && exists('g:tester_HOME')
    let $XDG_CACHE_HOME = g:tester_HOME . '/.cache'
  endif
  call mkdir('Xhome')
  let $HOME = fnamemodify('Xhome', ':p')
endfunc

func GUITearDownCommon()
  call delete('Xhome', 'rf')
endfunc

" Ignore the "failed to create input context" error.
call test_ignore_error('E285')
