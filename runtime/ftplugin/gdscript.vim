vim9script

# Vim filetype plugin file
# Language: gdscript (Godot game engine scripting language)
# Maintainer: Maxim Kim <habamax@gmail.com>
# Website: https://github.com/habamax/vim-gdscript

if exists("b:did_ftplugin") | finish | endif

b:did_ftplugin = 1
b:undo_ftplugin = 'setlocal cinkeys<'
      \ .. '| setlocal indentkeys<'
      \ .. '| setlocal commentstring<'
      \ .. '| setlocal suffixesadd<'
      \ .. '| setlocal foldexpr<'
      \ .. '| setlocal foldignore<'

setlocal cinkeys-=0#
setlocal indentkeys-=0#
setlocal suffixesadd=.gd
setlocal commentstring=#\ %s
setlocal foldignore=
setlocal foldexpr=GDScriptFoldLevel()


def GDScriptFoldLevel(): string
    var line = getline(v:lnum)
    if line =~? '^\s*$'
        return "-1"
    endif

    var sw = shiftwidth()
    var indent = indent(v:lnum) / sw
    var indent_next = indent(nextnonblank(v:lnum + 1)) / sw

    if indent_next > indent && line =~ ':\s*$'
        return $">{indent_next}"
    else
        return $"{indent}"
    endif
enddef


if !exists("g:no_plugin_maps")
    # Next/Previous section
    def NextSection(back: bool, cnt: number)
        for n in range(cnt)
            search('^\s*func\s', back ? 'bW' : 'W')
        endfor
    enddef

    nnoremap <silent><buffer> ]] <scriptcmd>NextSection(false, v:count1)<CR>
    nnoremap <silent><buffer> [[ <scriptcmd>NextSection(true, v:count1)<CR>
    xmap <buffer><expr> ]] $'<C-\><C-N>{v:count1}]]m>gv'
    xmap <buffer><expr> [[ $'<C-\><C-N>{v:count1}[[m>gv'
    b:undo_ftplugin ..=
          \    " | silent exe 'unmap <buffer> [['"
          \ .. " | silent exe 'unmap <buffer> ]]'"
endif
