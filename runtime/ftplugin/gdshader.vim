vim9script

# Vim filetype plugin file
# Language: Godot shading language
# Maintainer: Maxim Kim <habamax@gmail.com>
# Website: https://github.com/habamax/vim-gdscript

if exists("b:did_ftplugin") | finish | endif
b:did_ftplugin = 1

b:undo_ftplugin = 'setlocal suffixesadd<'

setlocal suffixesadd=.gdshader
