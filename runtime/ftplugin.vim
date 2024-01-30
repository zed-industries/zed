vim9script noclear

# Vim support file to switch on loading plugins for file types
#
# Maintainer:	The Vim Project <https://github.com/vim/vim>
# Last change:	2023 Aug 10
# Former Maintainer:	Bram Moolenaar <Bram@vim.org>

if exists("g:did_load_ftplugin")
  finish
endif
g:did_load_ftplugin = 1

augroup filetypeplugin
  au FileType * call LoadFTPlugin()
augroup END

if exists('*LoadFTPlugin')
  # No need to define the function again.
  finish
endif

def LoadFTPlugin()
  if exists("b:undo_ftplugin")
    # We assume b:undo_ftplugin is using legacy script syntax
    legacy exe b:undo_ftplugin
    unlet! b:undo_ftplugin b:did_ftplugin
  endif

  var s = expand("<amatch>")
  if s != ""
    if &cpo =~# "S" && exists("b:did_ftplugin")
      # In compatible mode options are reset to the global values, need to
      # set the local values also when a plugin was already used.
      unlet b:did_ftplugin
    endif

    # When there is a dot it is used to separate filetype names.  Thus for
    # "aaa.bbb" load "aaa" and then "bbb".
    for name in split(s, '\.')
      exe 'runtime! ftplugin/' .. name .. '.vim ftplugin/' .. name .. '_*.vim ftplugin/' .. name .. '/*.vim'
    endfor
  endif
enddef
