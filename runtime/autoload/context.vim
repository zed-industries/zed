vim9script

# Language:           ConTeXt typesetting engine
# Maintainer:         Nicola Vitacolonna <nvitacolonna@gmail.com>
# Former Maintainers: Nikolai Weibull <now@bitwi.se>
# Latest Revision:    2023 Dec 26

# Typesetting {{{
import autoload './typeset.vim'

export def ConTeXtCmd(path: string): list<string>
  var cmd = ['mtxrun', '--script', 'context', '--nonstopmode', '--autogenerate']
  if !empty(get(g:, 'context_extra_options', ''))
    cmd += g:context_extra_options
  endif
  cmd->add(path)
  return cmd
enddef

export def Typeset(bufname: string, env = {}, Cmd = ConTeXtCmd): bool
  return typeset.TypesetBuffer(bufname, Cmd, env, 'ConTeXt')
enddef

export def JobStatus()
  typeset.JobStatus('ConTeXt')
enddef

export def StopJobs()
  typeset.StopJobs('ConTeXt')
enddef

export def Log(bufname: string)
  execute 'edit' typeset.LogPath(bufname)
enddef
# }}}

# Completion {{{
def BinarySearch(base: string, keywords: list<string>): list<string>
  const pat = '^' .. base
  const len = len(keywords)
  var res = []
  var lft = 0
  var rgt = len

  # Find the leftmost index matching base
  while lft < rgt
    var i = (lft + rgt) / 2
    if keywords[i] < base
      lft = i + 1
    else
      rgt = i
    endif
  endwhile

  while lft < len && keywords[lft] =~ pat
    add(res, keywords[lft])
    lft += 1
  endwhile

  return res
enddef

var isMetaPostBlock = false

var MP_KEYWORDS:  list<string> = []
var CTX_KEYWORDS: list<string> = []

# Complete only MetaPost keywords in MetaPost blocks, and complete only
# ConTeXt keywords otherwise.
export def Complete(findstart: number, base: string): any
  if findstart == 1
    if len(synstack(line("."), 1)) > 0 && synIDattr(synstack(line("."), 1)[0], "name") ==# 'contextMPGraphic'
      isMetaPostBlock = true
      return match(getline('.'), '\S\+\%' .. col('.') .. 'c')
    endif

    # Complete only \commands starting with a backslash
    isMetaPostBlock = false
    var pos = match(getline('.'), '\\\zs\S\+\%' .. col('.') .. 'c')
    return (pos == -1) ? -3 : pos
  endif

  if isMetaPostBlock
    if empty(MP_KEYWORDS)
      MP_KEYWORDS = sort(syntaxcomplete#OmniSyntaxList(['mf\w\+', 'mp\w\+']))
    endif
    return BinarySearch(base, MP_KEYWORDS)
  endif

  if empty(CTX_KEYWORDS)
    CTX_KEYWORDS = sort(syntaxcomplete#OmniSyntaxList([
      'context\w\+', 'texAleph', 'texEtex', 'texLuatex', 'texOmega',
      'texPdftex', 'texTex', 'texXeTeX'
    ]))
  endif
  return BinarySearch(base, CTX_KEYWORDS)
enddef
# }}}

# vim: sw=2 fdm=marker
