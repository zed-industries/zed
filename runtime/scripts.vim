" Vim support file to detect file types in scripts
"
" Maintainer:	The Vim Project <https://github.com/vim/vim>
" Last Change:	2023 Aug 27
" Former Maintainer:	Bram Moolenaar <Bram@vim.org>

" This file is called by an autocommand for every file that has just been
" loaded into a buffer.  It checks if the type of file can be recognized by
" the file contents.  The autocommand is in $VIMRUNTIME/filetype.vim.


" Bail out when a FileType autocommand has already set the filetype.
if did_filetype()
  finish
endif

" Load the user defined scripts file first
" Only do this when the FileType autocommand has not been triggered yet
if exists("myscriptsfile") && filereadable(expand(myscriptsfile))
  execute "source " . myscriptsfile
  if did_filetype()
    finish
  endif
endif

" The main code is in a compiled function for speed.
call dist#script#DetectFiletype()
