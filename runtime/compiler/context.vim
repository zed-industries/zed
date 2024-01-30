vim9script

# Language:           ConTeXt typesetting engine
# Maintainer:         Nicola Vitacolonna <nvitacolonna@gmail.com>
# Former Maintainers: Nikolai Weibull <now@bitwi.se>
# Latest Revision:    2023 Dec 26

if exists("g:current_compiler")
  finish
endif

import autoload '../autoload/context.vim'

if exists(":CompilerSet") != 2 # Older Vim always used :setlocal
  command -nargs=* CompilerSet setlocal <args>
endif

g:current_compiler = 'context'

if get(b:, 'context_ignore_makefile', get(g:, 'context_ignore_makefile', 0)) ||
  (!filereadable('Makefile') && !filereadable('makefile'))
  &l:makeprg =  join(context.ConTeXtCmd(shellescape(expand('%:p:t'))), ' ')
else
  g:current_compiler = 'make'
endif

const context_errorformat = join([
  "%-Popen source%.%#> %f",
  "%-Qclose source%.%#> %f",
  "%-Popen source%.%#name '%f'",
  "%-Qclose source%.%#name '%f'",
  "tex %trror%.%#error on line %l in file %f: %m",
  "%Elua %trror%.%#error on line %l in file %f:",
  "%+Emetapost %#> error: %#",
  "%Emetafun%.%#error: %m",
  "! error: %#%m",
  "%-C %#",
  "%C! %m",
  "%Z[ctxlua]%m",
  "%+C<*> %.%#",
  "%-C%.%#",
  "%Z...%m",
  "%-Zno-error",
  "%-G%.%#"], ",")

execute 'CompilerSet errorformat=' .. escape(context_errorformat, ' ')

# vim: sw=2 fdm=marker
