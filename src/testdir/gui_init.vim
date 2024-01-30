" gvimrc for test_gui_init.vim

if has('gui_motif') || has('gui_gtk2') || has('gui_gtk3')
  set guiheadroom=0
  set guioptions+=p
endif
