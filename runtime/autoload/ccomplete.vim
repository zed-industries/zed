vim9script noclear

# Vim completion script
# Language:	C
# Maintainer:	The Vim Project <https://github.com/vim/vim>
# Last Change:	2023 Aug 10
#		Rewritten in Vim9 script by github user lacygoill
# Former Maintainer:   Bram Moolenaar <Bram@vim.org>

var prepended: string
var grepCache: dict<list<dict<any>>>

# This function is used for the 'omnifunc' option.
export def Complete(findstart: bool, abase: string): any # {{{1
  if findstart
    # Locate the start of the item, including ".", "->" and "[...]".
    var line: string = getline('.')
    var start: number = charcol('.') - 1
    var lastword: number = -1
    while start > 0
      if line[start - 1] =~ '\w'
        --start
      elseif line[start - 1] =~ '\.'
        if lastword == -1
          lastword = start
        endif
        --start
      elseif start > 1 && line[start - 2] == '-'
        && line[start - 1] == '>'
        if lastword == -1
          lastword = start
        endif
        start -= 2
      elseif line[start - 1] == ']'
        # Skip over [...].
        var n: number = 0
        --start
        while start > 0
          --start
          if line[start] == '['
            if n == 0
              break
            endif
            --n
          elseif line[start] == ']'  # nested []
            ++n
          endif
        endwhile
      else
        break
      endif
    endwhile

    # Return the column of the last word, which is going to be changed.
    # Remember the text that comes before it in prepended.
    if lastword == -1
      prepended = ''
      return byteidx(line, start)
    endif
    prepended = line[start : lastword - 1]
    return byteidx(line, lastword)
  endif

  # Return list of matches.

  var base: string = prepended .. abase

  # Don't do anything for an empty base, would result in all the tags in the
  # tags file.
  if base == ''
    return []
  endif

  # init cache for vimgrep to empty
  grepCache = {}

  # Split item in words, keep empty word after "." or "->".
  # "aa" -> ['aa'], "aa." -> ['aa', ''], "aa.bb" -> ['aa', 'bb'], etc.
  # We can't use split, because we need to skip nested [...].
  # "aa[...]" -> ['aa', '[...]'], "aa.bb[...]" -> ['aa', 'bb', '[...]'], etc.
  var items: list<string>
  var s: number = 0
  var arrays: number = 0
  while 1
    var e: number = base->charidx(match(base, '\.\|->\|\[', s))
    if e < 0
      if s == 0 || base[s - 1] != ']'
        items->add(base[s :])
      endif
      break
    endif
    if s == 0 || base[s - 1] != ']'
      items->add(base[s : e - 1])
    endif
    if base[e] == '.'
      # skip over '.'
      s = e + 1
    elseif base[e] == '-'
      # skip over '->'
      s = e + 2
    else
      # Skip over [...].
      var n: number = 0
      s = e
      ++e
      while e < strcharlen(base)
        if base[e] == ']'
          if n == 0
            break
          endif
          --n
        elseif base[e] == '['  # nested [...]
          ++n
        endif
        ++e
      endwhile
      ++e
      items->add(base[s : e - 1])
      ++arrays
      s = e
    endif
  endwhile

  # Find the variable items[0].
  # 1. in current function (like with "gd")
  # 2. in tags file(s) (like with ":tag")
  # 3. in current file (like with "gD")
  var res: list<dict<any>>
  if items[0]->searchdecl(false, true) == 0
    # Found, now figure out the type.
    # TODO: join previous line if it makes sense
    var line: string = getline('.')
    var col: number = charcol('.')
    if line[: col - 1]->stridx(';') >= 0
      # Handle multiple declarations on the same line.
      var col2: number = col - 1
      while line[col2] != ';'
        --col2
      endwhile
      line = line[col2 + 1 :]
      col -= col2
    endif
    if line[: col - 1]->stridx(',') >= 0
      # Handle multiple declarations on the same line in a function
      # declaration.
      var col2: number = col - 1
      while line[col2] != ','
        --col2
      endwhile
      if line[col2 + 1 : col - 1] =~ ' *[^ ][^ ]*  *[^ ]'
        line = line[col2 + 1 :]
        col -= col2
      endif
    endif
    if len(items) == 1
      # Completing one word and it's a local variable: May add '[', '.' or
      # '->'.
      var match: string = items[0]
      var kind: string = 'v'
      if match(line, '\<' .. match .. '\s*\[') > 0
        match ..= '['
      else
        res = line[: col - 1]->Nextitem([''], 0, true)
        if len(res) > 0
          # There are members, thus add "." or "->".
          if match(line, '\*[ \t(]*' .. match .. '\>') > 0
            match ..= '->'
          else
            match ..= '.'
          endif
        endif
      endif
      res = [{match: match, tagline: '', kind: kind, info: line}]
    elseif len(items) == arrays + 1
      # Completing one word and it's a local array variable: build tagline
      # from declaration line
      var match: string = items[0]
      var kind: string = 'v'
      var tagline: string = "\t/^" .. line .. '$/'
      res = [{match: match, tagline: tagline, kind: kind, info: line}]
    else
      # Completing "var.", "var.something", etc.
      res = line[: col - 1]->Nextitem(items[1 :], 0, true)
    endif
  endif

  if len(items) == 1 || len(items) == arrays + 1
    # Only one part, no "." or "->": complete from tags file.
    var tags: list<dict<any>>
    if len(items) == 1
      tags = taglist('^' .. base)
    else
      tags = taglist('^' .. items[0] .. '$')
    endif

    tags
      # Remove members, these can't appear without something in front.
      ->filter((_, v: dict<any>): bool =>
                v->has_key('kind') ? v.kind != 'm' : true)
      # Remove static matches in other files.
      ->filter((_, v: dict<any>): bool =>
                 !v->has_key('static')
              || !v['static']
              || bufnr('%') == bufnr(v['filename']))

    res = res->extend(tags->map((_, v: dict<any>) => Tag2item(v)))
  endif

  if len(res) == 0
    # Find the variable in the tags file(s)
    var diclist: list<dict<any>> = taglist('^' .. items[0] .. '$')
      # Remove members, these can't appear without something in front.
      ->filter((_, v: dict<string>): bool =>
                v->has_key('kind') ? v.kind != 'm' : true)

    res = []
    for i: number in len(diclist)->range()
      # New ctags has the "typeref" field.  Patched version has "typename".
      if diclist[i]->has_key('typename')
        res = res->extend(diclist[i]['typename']->StructMembers(items[1 :], true))
      elseif diclist[i]->has_key('typeref')
        res = res->extend(diclist[i]['typeref']->StructMembers(items[1 :], true))
      endif

      # For a variable use the command, which must be a search pattern that
      # shows the declaration of the variable.
      if diclist[i]['kind'] == 'v'
        var line: string = diclist[i]['cmd']
        if line[: 1] == '/^'
          var col: number = line->charidx(match(line, '\<' .. items[0] .. '\>'))
          res = res->extend(line[2 : col - 1]->Nextitem(items[1 :], 0, true))
        endif
      endif
    endfor
  endif

  if len(res) == 0 && items[0]->searchdecl(true) == 0
    # Found, now figure out the type.
    # TODO: join previous line if it makes sense
    var line: string = getline('.')
    var col: number = charcol('.')
    res = line[: col - 1]->Nextitem(items[1 :], 0, true)
  endif

  # If the last item(s) are [...] they need to be added to the matches.
  var last: number = len(items) - 1
  var brackets: string = ''
  while last >= 0
    if items[last][0] != '['
      break
    endif
    brackets = items[last] .. brackets
    --last
  endwhile

  return res->map((_, v: dict<any>): dict<string> => Tagline2item(v, brackets))
enddef

def GetAddition( # {{{1
    line: string,
    match: string,
    memarg: list<dict<any>>,
    bracket: bool): string
  # Guess if the item is an array.
  if bracket && match(line, match .. '\s*\[') > 0
    return '['
  endif

  # Check if the item has members.
  if SearchMembers(memarg, [''], false)->len() > 0
    # If there is a '*' before the name use "->".
    if match(line, '\*[ \t(]*' .. match .. '\>') > 0
      return '->'
    else
      return '.'
    endif
  endif
  return ''
enddef

def Tag2item(val: dict<any>): dict<any> # {{{1
# Turn the tag info "val" into an item for completion.
# "val" is is an item in the list returned by taglist().
# If it is a variable we may add "." or "->".  Don't do it for other types,
# such as a typedef, by not including the info that GetAddition() uses.
  var res: dict<any> = {match: val['name']}

  res['extra'] = Tagcmd2extra(val['cmd'], val['name'], val['filename'])

  var s: string = Dict2info(val)
  if s != ''
    res['info'] = s
  endif

  res['tagline'] = ''
  if val->has_key('kind')
    var kind: string = val['kind']
    res['kind'] = kind
    if kind == 'v'
      res['tagline'] = "\t" .. val['cmd']
      res['dict'] = val
    elseif kind == 'f'
      res['match'] = val['name'] .. '('
    endif
  endif

  return res
enddef

def Dict2info(dict: dict<any>): string # {{{1
# Use all the items in dictionary for the "info" entry.
  var info: string = ''
  for k: string in dict->keys()->sort()
    info  ..= k .. repeat(' ', 10 - strlen(k))
    if k == 'cmd'
      info ..= dict['cmd']
        ->matchstr('/^\s*\zs.*\ze$/')
        ->substitute('\\\(.\)', '\1', 'g')
    else
      var dictk: any = dict[k]
      if typename(dictk) != 'string'
        info ..= dictk->string()
      else
        info ..= dictk
      endif
    endif
    info ..= "\n"
  endfor
  return info
enddef

def ParseTagline(line: string): dict<any> # {{{1
# Parse a tag line and return a dictionary with items like taglist()
  var l: list<string> = split(line, "\t")
  var d: dict<any>
  if len(l) >= 3
    d['name'] = l[0]
    d['filename'] = l[1]
    d['cmd'] = l[2]
    var n: number = 2
    if l[2] =~ '^/'
      # Find end of cmd, it may contain Tabs.
      while n < len(l) && l[n] !~ '/;"$'
        ++n
        d['cmd'] ..= '  ' .. l[n]
      endwhile
    endif
    for i: number in range(n + 1, len(l) - 1)
      if l[i] == 'file:'
        d['static'] = 1
      elseif l[i] !~ ':'
        d['kind'] = l[i]
      else
        d[l[i]->matchstr('[^:]*')] = l[i]->matchstr(':\zs.*')
      endif
    endfor
  endif

  return d
enddef

def Tagline2item(val: dict<any>, brackets: string): dict<string> # {{{1
# Turn a match item "val" into an item for completion.
# "val['match']" is the matching item.
# "val['tagline']" is the tagline in which the last part was found.
  var line: string = val['tagline']
  var add: string = GetAddition(line, val['match'], [val], brackets == '')
  var res: dict<string> = {word: val['match'] .. brackets .. add}

  if val->has_key('info')
    # Use info from Tag2item().
    res['info'] = val['info']
  else
    # Parse the tag line and add each part to the "info" entry.
    var s: string = ParseTagline(line)->Dict2info()
    if s != ''
      res['info'] = s
    endif
  endif

  if val->has_key('kind')
    res['kind'] = val['kind']
  elseif add == '('
    res['kind'] = 'f'
  else
    var s: string = line->matchstr('\t\(kind:\)\=\zs\S\ze\(\t\|$\)')
    if s != ''
      res['kind'] = s
    endif
  endif

  if val->has_key('extra')
    res['menu'] = val['extra']
    return res
  endif

  # Isolate the command after the tag and filename.
  var s: string = line->matchstr('[^\t]*\t[^\t]*\t\zs\(/^.*$/\|[^\t]*\)\ze\(;"\t\|\t\|$\)')
  if s != ''
    res['menu'] = s->Tagcmd2extra(val['match'], line->matchstr('[^\t]*\t\zs[^\t]*\ze\t'))
  endif
  return res
enddef

def Tagcmd2extra( # {{{1
    cmd: string,
    name: string,
    fname: string): string
# Turn a command from a tag line to something that is useful in the menu
  var x: string
  if cmd =~ '^/^'
    # The command is a search command, useful to see what it is.
    x = cmd
      ->matchstr('^/^\s*\zs.*\ze$/')
      ->substitute('\<' .. name .. '\>', '@@', '')
      ->substitute('\\\(.\)', '\1', 'g')
      .. ' - ' .. fname
  elseif cmd =~ '^\d*$'
    # The command is a line number, the file name is more useful.
    x = fname .. ' - ' .. cmd
  else
    # Not recognized, use command and file name.
    x = cmd .. ' - ' .. fname
  endif
  return x
enddef

def Nextitem( # {{{1
    lead: string,
    items: list<string>,
    depth: number,
    all: bool): list<dict<string>>
# Find composing type in "lead" and match items[0] with it.
# Repeat this recursively for items[1], if it's there.
# When resolving typedefs "depth" is used to avoid infinite recursion.
# Return the list of matches.

  # Use the text up to the variable name and split it in tokens.
  var tokens: list<string> = split(lead, '\s\+\|\<')

  # Try to recognize the type of the variable.  This is rough guessing...
  var res: list<dict<string>>
  for tidx: number in len(tokens)->range()

    # Skip tokens starting with a non-ID character.
    if tokens[tidx] !~ '^\h'
      continue
    endif

    # Recognize "struct foobar" and "union foobar".
    # Also do "class foobar" when it's C++ after all (doesn't work very well
    # though).
    if (tokens[tidx] == 'struct'
      || tokens[tidx] == 'union'
      || tokens[tidx] == 'class')
      && tidx + 1 < len(tokens)
      res = StructMembers(tokens[tidx] .. ':' .. tokens[tidx + 1], items, all)
      break
    endif

    # TODO: add more reserved words
    if ['int', 'short', 'char', 'float',
        'double', 'static', 'unsigned', 'extern']->index(tokens[tidx]) >= 0
      continue
    endif

    # Use the tags file to find out if this is a typedef.
    var diclist: list<dict<any>> = taglist('^' .. tokens[tidx] .. '$')
    for tagidx: number in len(diclist)->range()
      var item: dict<any> = diclist[tagidx]

      # New ctags has the "typeref" field.  Patched version has "typename".
      if item->has_key('typeref')
        res = res->extend(item['typeref']->StructMembers(items, all))
        continue
      endif
      if item->has_key('typename')
        res = res->extend(item['typename']->StructMembers(items, all))
        continue
      endif

      # Only handle typedefs here.
      if item['kind'] != 't'
        continue
      endif

      # Skip matches local to another file.
      if item->has_key('static') && item['static']
        && bufnr('%') != bufnr(item['filename'])
        continue
      endif

      # For old ctags we recognize "typedef struct aaa" and
      # "typedef union bbb" in the tags file command.
      var cmd: string = item['cmd']
      var ei: number = cmd->charidx(matchend(cmd, 'typedef\s\+'))
      if ei > 1
        var cmdtokens: list<string> = cmd[ei :]->split('\s\+\|\<')
        if len(cmdtokens) > 1
          if cmdtokens[0] == 'struct'
            || cmdtokens[0] == 'union'
            || cmdtokens[0] == 'class'
            var name: string = ''
            # Use the first identifier after the "struct" or "union"
            for ti: number in (len(cmdtokens) - 1)->range()
              if cmdtokens[ti] =~ '^\w'
                name = cmdtokens[ti]
                break
              endif
            endfor
            if name != ''
              res = res->extend(StructMembers(cmdtokens[0] .. ':' .. name, items, all))
            endif
          elseif depth < 10
            # Could be "typedef other_T some_T".
            res = res->extend(cmdtokens[0]->Nextitem(items, depth + 1, all))
          endif
        endif
      endif
    endfor
    if len(res) > 0
      break
    endif
  endfor

  return res
enddef

def StructMembers( # {{{1
    atypename: string,
    items: list<string>,
    all: bool): list<dict<string>>

# Search for members of structure "typename" in tags files.
# Return a list with resulting matches.
# Each match is a dictionary with "match" and "tagline" entries.
# When "all" is true find all, otherwise just return 1 if there is any member.

  # Todo: What about local structures?
  var fnames: string = tagfiles()
    ->map((_, v: string) => escape(v, ' \#%'))
    ->join()
  if fnames == ''
    return []
  endif

  var typename: string = atypename
  var qflist: list<dict<any>>
  var cached: number = 0
  var n: string
  if !all
    n = '1'  # stop at first found match
    if grepCache->has_key(typename)
      qflist = grepCache[typename]
      cached = 1
    endif
  else
    n = ''
  endif
  if !cached
    while 1
      execute 'silent! keepjumps noautocmd '
        .. n .. 'vimgrep ' .. '/\t' .. typename .. '\(\t\|$\)/j '
        .. fnames

      qflist = getqflist()
      if len(qflist) > 0 || match(typename, '::') < 0
        break
      endif
      # No match for "struct:context::name", remove "context::" and try again.
      typename = typename->substitute(':[^:]*::', ':', '')
    endwhile

    if !all
      # Store the result to be able to use it again later.
      grepCache[typename] = qflist
    endif
  endif

  # Skip over [...] items
  var idx: number = 0
  var target: string
  while 1
    if idx >= len(items)
      target = ''  # No further items, matching all members
      break
    endif
    if items[idx][0] != '['
      target = items[idx]
      break
    endif
    ++idx
  endwhile
  # Put matching members in matches[].
  var matches: list<dict<string>>
  for l: dict<any> in qflist
    var memb: string = l['text']->matchstr('[^\t]*')
    if memb =~ '^' .. target
      # Skip matches local to another file.
      if match(l['text'], "\tfile:") < 0
        || bufnr('%') == l['text']->matchstr('\t\zs[^\t]*')->bufnr()
        var item: dict<string> = {match: memb, tagline: l['text']}

        # Add the kind of item.
        var s: string = l['text']->matchstr('\t\(kind:\)\=\zs\S\ze\(\t\|$\)')
        if s != ''
          item['kind'] = s
          if s == 'f'
            item['match'] = memb .. '('
          endif
        endif

        matches->add(item)
      endif
    endif
  endfor

  if len(matches) > 0
    # Skip over next [...] items
    ++idx
    while 1
      if idx >= len(items)
        return matches  # No further items, return the result.
      endif
      if items[idx][0] != '['
        break
      endif
      ++idx
    endwhile

    # More items following.  For each of the possible members find the
    # matching following members.
    return SearchMembers(matches, items[idx :], all)
  endif

  # Failed to find anything.
  return []
enddef

def SearchMembers( # {{{1
    matches: list<dict<any>>,
    items: list<string>,
    all: bool): list<dict<string>>

# For matching members, find matches for following items.
# When "all" is true find all, otherwise just return 1 if there is any member.
  var res: list<dict<string>>
  for i: number in len(matches)->range()
    var typename: string = ''
    var line: string
    if matches[i]->has_key('dict')
      if matches[i]['dict']->has_key('typename')
        typename = matches[i]['dict']['typename']
      elseif matches[i]['dict']->has_key('typeref')
        typename = matches[i]['dict']['typeref']
      endif
      line = "\t" .. matches[i]['dict']['cmd']
    else
      line = matches[i]['tagline']
      var eb: number = matchend(line, '\ttypename:')
      var e: number = charidx(line, eb)
      if e < 0
        eb = matchend(line, '\ttyperef:')
        e = charidx(line, eb)
      endif
      if e > 0
        # Use typename field
        typename = line->matchstr('[^\t]*', eb)
      endif
    endif

    if typename != ''
      res = res->extend(StructMembers(typename, items, all))
    else
      # Use the search command (the declaration itself).
      var sb: number = line->match('\t\zs/^')
      var s: number = charidx(line, sb)
      if s > 0
        var e: number = line
          ->charidx(match(line, '\<' .. matches[i]['match'] .. '\>', sb))
        if e > 0
          res = res->extend(line[s : e - 1]->Nextitem(items, 0, all))
        endif
      endif
    endif
    if !all && len(res) > 0
      break
    endif
  endfor
  return res
enddef
#}}}1

# vim: noet sw=2 sts=2
