vim9script

# Extra functionality for displaying Vim help .

# Called when editing the doc/syntax.txt file
export def HighlightGroups()
  var save_cursor = getcurpos()
  var buf: number = bufnr('%')

  var start: number = search('\*highlight-groups\*', 'c')
  var end: number = search('^======')
  for lnum in range(start, end)
    var word: string = getline(lnum)->matchstr('^\w\+\ze\t')
    if word->hlexists()
      var type = 'help-hl-' .. word
      if prop_type_list({bufnr: buf})->index(type) != -1
	# was called before, delete existing properties
	prop_remove({type: type, bufnr: buf})
	prop_type_delete(type, {bufnr: buf})
      endif
      prop_type_add(type, {
	bufnr: buf,
	highlight: word,
	combine: false,
	})
      prop_add(lnum, 1, {length: word->strlen(), type: type})
    endif
  endfor

  setpos('.', save_cursor)
enddef
