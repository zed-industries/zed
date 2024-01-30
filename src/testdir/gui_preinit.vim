" vimrc for test_gui_init.vim

" Note that this flag must be added in the .vimrc file, before switching on
" syntax or filetype recognition (when the |gvimrc| file is sourced the system
" menu has already been loaded; the ":syntax on" and ":filetype on" commands
" load the menu too).
set guioptions+=M
