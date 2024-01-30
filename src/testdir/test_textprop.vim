" Tests for defining text property types and adding text properties to the
" buffer.

source check.vim
CheckFeature textprop

source screendump.vim
import './vim9.vim' as v9

func Test_proptype_global()
  call prop_type_add('comment', {'highlight': 'Directory', 'priority': 123, 'start_incl': 1, 'end_incl': 1})
  let proptypes = prop_type_list()
  call assert_equal(1, len(proptypes))
  call assert_equal('comment', proptypes[0])

  let proptype = prop_type_get('comment')
  call assert_equal('Directory', proptype['highlight'])
  call assert_equal(123, proptype['priority'])
  call assert_equal(1, proptype['start_incl'])
  call assert_equal(1, proptype['end_incl'])

  call prop_type_delete('comment')
  call assert_equal(0, len(prop_type_list()))

  call prop_type_add('one', {})
  call assert_equal(1, len(prop_type_list()))
  let proptype = 'one'->prop_type_get()
  call assert_false(has_key(proptype, 'highlight'))
  call assert_equal(0, proptype['priority'])
  call assert_equal(0, proptype['start_incl'])
  call assert_equal(0, proptype['end_incl'])

  call prop_type_add('two', {})
  call assert_equal(2, len(prop_type_list()))
  call prop_type_delete('one')
  call assert_equal(1, len(prop_type_list()))
  call prop_type_delete('two')
  call assert_equal(0, len(prop_type_list()))
endfunc

func Test_proptype_buf()
  let bufnr = bufnr('')
  call prop_type_add('comment', #{bufnr: bufnr, highlight: 'Directory', priority: 123, start_incl: 1, end_incl: 1})
  let proptypes = prop_type_list({'bufnr': bufnr})
  call assert_equal(1, len(proptypes))
  call assert_equal('comment', proptypes[0])

  let proptype = prop_type_get('comment', {'bufnr': bufnr})
  call assert_equal('Directory', proptype['highlight'])
  call assert_equal(123, proptype['priority'])
  call assert_equal(1, proptype['start_incl'])
  call assert_equal(1, proptype['end_incl'])

  call prop_type_delete('comment', {'bufnr': bufnr})
  call assert_equal(0, len({'bufnr': bufnr}->prop_type_list()))

  call prop_type_add('one', {'bufnr': bufnr})
  let proptype = prop_type_get('one', {'bufnr': bufnr})
  call assert_false(has_key(proptype, 'highlight'))
  call assert_equal(0, proptype['priority'])
  call assert_equal(0, proptype['start_incl'])
  call assert_equal(0, proptype['end_incl'])

  call prop_type_add('two', {'bufnr': bufnr})
  call assert_equal(2, len(prop_type_list({'bufnr': bufnr})))
  call prop_type_delete('one', {'bufnr': bufnr})
  call assert_equal(1, len(prop_type_list({'bufnr': bufnr})))
  call prop_type_delete('two', {'bufnr': bufnr})
  call assert_equal(0, len(prop_type_list({'bufnr': bufnr})))

  call assert_fails("call prop_type_add('one', {'bufnr': 98764})", "E158:")
endfunc

def Test_proptype_add_remove()
  # add and remove a prop type so that the array is empty
  prop_type_add('local', {bufnr: bufnr('%')})
  prop_type_delete('local', {bufnr: bufnr('%')})
  prop_type_add('global', {highlight: 'ErrorMsg'})
  prop_add(1, 1, {length: 1, type: 'global'})
  redraw

  prop_clear(1)
  prop_type_delete('global')
enddef

def Test_proptype_buf_list()
  new
  var bufnr = bufnr('')
  try
    prop_type_add('global', {})
    prop_type_add('local', {bufnr: bufnr})

    prop_add(1, 1, {type: 'global'})
    prop_add(1, 1, {type: 'local'})

    assert_equal([
      {type: 'local',  type_bufnr: bufnr, id: 0, col: 1, end: 1, length: 0, start: 1},
      {type: 'global', type_bufnr: 0,     id: 0, col: 1, end: 1, length: 0, start: 1},
    ], prop_list(1))
    assert_equal(
      {lnum: 1, id: 0, col: 1, type_bufnr: bufnr, end: 1, type: 'local', length: 0, start: 1},
      prop_find({lnum: 1, type: 'local'}))
    assert_equal(
      {lnum: 1, id: 0, col: 1, type_bufnr: 0, end: 1, type: 'global', length: 0, start: 1},
      prop_find({lnum: 1, type: 'global'}))

    prop_remove({type: 'global'}, 1)
    prop_remove({type: 'local'}, 1)
  finally
    prop_type_delete('global')
    prop_type_delete('local', {bufnr: bufnr})
    bwipe!
  endtry
enddef

func AddPropTypes()
  call prop_type_add('one', {})
  call prop_type_add('two', {})
  call prop_type_add('three', {})
  call prop_type_add('whole', {})
endfunc

func DeletePropTypes()
  call prop_type_delete('one')
  call prop_type_delete('two')
  call prop_type_delete('three')
  call prop_type_delete('whole')
endfunc

func SetupPropsInFirstLine()
  call setline(1, 'one two three')
  call prop_add(1, 1, {'length': 3, 'id': 11, 'type': 'one'})
  eval 1->prop_add(5, {'length': 3, 'id': 12, 'type': 'two'})
  call prop_add(1, 9, {'length': 5, 'id': 13, 'type': 'three'})
  call prop_add(1, 1, {'length': 13, 'id': 14, 'type': 'whole'})
endfunc

func Get_expected_props()
  return [
      \ #{type_bufnr: 0, col: 1, length: 13, id: 14, type: 'whole', start: 1, end: 1},
      \ #{type_bufnr: 0, col: 1, length: 3,  id: 11, type: 'one',   start: 1, end: 1},
      \ #{type_bufnr: 0, col: 5, length: 3,  id: 12, type: 'two',   start: 1, end: 1},
      \ #{type_bufnr: 0, col: 9, length: 5,  id: 13, type: 'three', start: 1, end: 1},
      \ ]
endfunc

func Test_prop_find()
  new
  call setline(1, ['one one one', 'twotwo', 'three', 'fourfour', 'five', 'sixsix'])

  " Add two text props on lines 1 and 5, and one spanning lines 2 to 4.
  call prop_type_add('prop_name', {'highlight': 'Directory'})
  call prop_add(1, 5, {'type': 'prop_name', 'id': 10, 'length': 3})
  call prop_add(2, 4, {'type': 'prop_name', 'id': 11, 'end_lnum': 4, 'end_col': 9})
  call prop_add(5, 4, {'type': 'prop_name', 'id': 12, 'length': 1})

  let expected = [
    \ #{type_bufnr: 0, lnum: 1, col: 5, length: 3, id: 10, type: 'prop_name', start: 1, end: 1},
    \ #{type_bufnr: 0, lnum: 2, col: 4, id: 11, type: 'prop_name', start: 1, end: 0},
    \ #{type_bufnr: 0, lnum: 5, col: 4, length: 1, id: 12, type: 'prop_name', start: 1, end: 1}
    \ ]

  " Starting at line 5 col 1 this should find the prop at line 5 col 4.
  call cursor(5, 1)
  let result = prop_find({'type': 'prop_name'}, 'f')
  call assert_equal(expected[2], result)

  " With skipstart left at false (default), this should find the prop at line
  " 5 col 4.
  let result = prop_find({'type': 'prop_name', 'lnum': 5, 'col': 4}, 'b')
  call assert_equal(expected[2], result)

  " With skipstart set to true, this should skip the prop at line 5 col 4.
  let result = prop_find({'type': 'prop_name', 'lnum': 5, 'col': 4, 'skipstart': 1}, 'b')
  unlet result.length
  call assert_equal(expected[1], result)

  " Search backwards from line 1 col 10 to find the prop on the same line.
  let result = prop_find({'type': 'prop_name', 'lnum': 1, 'col': 10}, 'b')
  call assert_equal(expected[0], result)

  " with skipstart set to false, if the start position is anywhere between the
  " start and end lines of a text prop (searching forward or backward), the
  " result should be the prop on the first line (the line with 'start' set to 1).
  call cursor(3, 1)
  let result = prop_find({'type': 'prop_name'}, 'f')
  unlet result.length
  call assert_equal(expected[1], result)
  let result = prop_find({'type': 'prop_name'}, 'b')
  unlet result.length
  call assert_equal(expected[1], result)

  " with skipstart set to true, if the start position is anywhere between the
  " start and end lines of a text prop (searching forward or backward), all lines
  " of the prop will be skipped.
  let result = prop_find({'type': 'prop_name', 'skipstart': 1}, 'b')
  call assert_equal(expected[0], result)
  let result = prop_find({'type': 'prop_name', 'skipstart': 1}, 'f')
  call assert_equal(expected[2], result)

  " Use skipstart to search through all props with type name 'prop_name'.
  " First forward...
  let lnum = 1
  let col = 1
  let i = 0
  for exp in expected
    let result = prop_find({'type': 'prop_name', 'lnum': lnum, 'col': col, 'skipstart': 1}, 'f')
    if !has_key(exp, "length")
      unlet result.length
    endif
    call assert_equal(exp, result)
    let lnum = result.lnum
    let col = result.col
    let i = i + 1
  endfor

  " ...then backwards.
  let lnum = 6
  let col = 4
  let i = 2
  while i >= 0
    let result = prop_find({'type': 'prop_name', 'lnum': lnum, 'col': col, 'skipstart': 1}, 'b')
    if !has_key(expected[i], "length")
      unlet result.length
    endif
    call assert_equal(expected[i], result)
    let lnum = result.lnum
    let col = result.col
    let i = i - 1
  endwhile

  " Starting from line 6 col 1 search backwards for prop with id 10.
  call cursor(6, 1)
  let result = prop_find({'id': 10, 'skipstart': 1}, 'b')
  call assert_equal(expected[0], result)

  " Starting from line 1 col 1 search forwards for prop with id 12.
  call cursor(1, 1)
  let result = prop_find({'id': 12}, 'f')
  call assert_equal(expected[2], result)

  " Search for a prop with an unknown id.
  let result = prop_find({'id': 999}, 'f')
  call assert_equal({}, result)

  " Search backwards from the proceeding position of the prop with id 11
  " (at line num 2 col 4). This should return an empty dict.
  let result = prop_find({'id': 11, 'lnum': 2, 'col': 3}, 'b')
  call assert_equal({}, result)

  " When lnum is given and col is omitted, use column 1.
  let result = prop_find({'type': 'prop_name', 'lnum': 1}, 'f')
  call assert_equal(expected[0], result)

  " Negative ID is possible, just like prop is not found.
  call assert_equal({}, prop_find({'id': -1}))
  call assert_equal({}, prop_find({'id': -2}))

  call prop_clear(1, 6)

  " Default ID is zero
  call prop_add(5, 4, {'type': 'prop_name', 'length': 1})
  call assert_equal(#{lnum: 5, id: 0, col: 4, type_bufnr: 0, end: 1, type: 'prop_name', length: 1, start: 1}, prop_find({'id': 0}))

  call prop_type_delete('prop_name')
  call prop_clear(1, 6)
  bwipe!
endfunc

def Test_prop_find2()
  # Multiple props per line, start on the first, should find the second.
  new
  ['the quikc bronw fox jumsp over the layz dog']->repeat(2)->setline(1)
  prop_type_add('misspell', {highlight: 'ErrorMsg'})
  for lnum in [1, 2]
    for col in [8, 14, 24, 38]
      prop_add(lnum, col, {type: 'misspell', length: 2})
    endfor
  endfor
  cursor(1, 8)
  var expected = {type_bufnr: 0, lnum: 1, id: 0, col: 14, end: 1, type: 'misspell', length: 2, start: 1}
  var result = prop_find({type: 'misspell', skipstart: true}, 'f')
  assert_equal(expected, result)

  prop_type_delete('misspell')
  bwipe!
enddef

func Test_prop_find_smaller_len_than_match_col()
  new
  call prop_type_add('test', {'highlight': 'ErrorMsg'})
  call setline(1, ['xxxx', 'x'])
  call prop_add(1, 4, {'type': 'test'})
  call assert_equal(
        \ #{type_bufnr: 0, id: 0, lnum: 1, col: 4, type: 'test', length: 0, start: 1, end: 1},
        \ prop_find({'type': 'test', 'lnum': 2, 'col': 1}, 'b'))
  bwipe!
  call prop_type_delete('test')
endfunc

func Test_prop_find_with_both_option_enabled()
  " Initialize
  new
  call AddPropTypes()
  call SetupPropsInFirstLine()
  let props = Get_expected_props()->map({_, v -> extend(v, {'lnum': 1})})
  " Test
  call assert_fails("call prop_find({'both': 1})", 'E968:')
  call assert_fails("call prop_find({'id': 11, 'both': 1})", 'E860:')
  call assert_fails("call prop_find({'type': 'three', 'both': 1})", 'E860:')
  call assert_equal({}, prop_find({'id': 11, 'type': 'three', 'both': 1}))
  call assert_equal({}, prop_find({'id': 130000, 'type': 'one', 'both': 1}))
  call assert_equal(props[2], prop_find({'id': 12, 'type': 'two', 'both': 1}))
  call assert_equal(props[0], prop_find({'id': 14, 'type': 'whole', 'both': 1}))
  " Clean up
  call DeletePropTypes()
  bwipe!
endfunc

func Test_prop_add()
  new
  call AddPropTypes()
  call SetupPropsInFirstLine()
  let expected_props = Get_expected_props()
  call assert_equal(expected_props, prop_list(1))
  call assert_fails("call prop_add(10, 1, {'length': 1, 'id': 14, 'type': 'whole'})", 'E966:')
  call assert_fails("call prop_add(1, 22, {'length': 1, 'id': 14, 'type': 'whole'})", 'E964:')

  " Insert a line above, text props must still be there.
  call append(0, 'empty')
  call assert_equal(expected_props, prop_list(2))
  " Delete a line above, text props must still be there.
  1del
  call assert_equal(expected_props, prop_list(1))

  " Prop without length or end column is zero length
  call prop_clear(1)
  call prop_type_add('included', {'start_incl': 1, 'end_incl': 1})
  call prop_add(1, 5, #{type: 'included'})
  let expected = [#{type_bufnr: 0, col: 5, length: 0, type: 'included', id: 0, start: 1, end: 1}]
  call assert_equal(expected, prop_list(1))

  " Inserting text makes the prop bigger.
  exe "normal 5|ixx\<Esc>"
  let expected = [#{type_bufnr: 0, col: 5, length: 2, type: 'included', id: 0, start: 1, end: 1}]
  call assert_equal(expected, prop_list(1))

  call assert_fails("call prop_add(1, 5, {'type': 'two', 'bufnr': 234343})", 'E158:')

  call DeletePropTypes()
  call prop_type_delete('included')
  bwipe!
endfunc

" Test for the prop_add_list() function
func Test_prop_add_list()
  new
  call AddPropTypes()
  call setline(1, ['one one one', 'two two two', 'six six six', 'ten ten ten'])
  call prop_add_list(#{type: 'one', id: 2},
        \ [[1, 1, 1, 3], [2, 5, 2, 7], [3, 6, 4, 6]])
  call assert_equal([#{id: 2, col: 1, type_bufnr: 0, end: 1, type: 'one',
        \ length: 2, start: 1}], prop_list(1))
  call assert_equal([#{id: 2, col: 5, type_bufnr: 0, end: 1, type: 'one',
        \ length: 2, start: 1}], prop_list(2))
  call assert_equal([#{id: 2, col: 6, type_bufnr: 0, end: 0, type: 'one',
        \ length: 7, start: 1}], prop_list(3))
  call assert_equal([#{id: 2, col: 1, type_bufnr: 0, end: 1, type: 'one',
        \ length: 5, start: 0}], prop_list(4))
  call prop_remove(#{id: 2})
  call assert_equal([], prop_list(1))

  call prop_add_list(#{type: 'one', id: 3},
        \ [[1, 1, 1, 3], [2, 5, 2, 7, 9]])
  call assert_equal([#{id: 3, col: 1, type_bufnr: 0, end: 1, type: 'one',
        \ length: 2, start: 1}], prop_list(1))
  call assert_equal([#{id: 9, col: 5, type_bufnr: 0, end: 1, type: 'one',
        \ length: 2, start: 1}], prop_list(2))

  call assert_fails('call prop_add_list([1, 2], [[1, 1, 3]])', 'E1206:')
  call assert_fails('call prop_add_list({}, {})', 'E1211:')
  call assert_fails('call prop_add_list({}, [[1, 1, 3]])', 'E965:')
  call assert_fails('call prop_add_list(#{type: "abc"}, [[1, 1, 1, 3]])', 'E971:')
  call assert_fails('call prop_add_list(#{type: "one"}, [[]])', 'E474:')
  call assert_fails('call prop_add_list(#{type: "one"}, [[1, 1, 1, 1], {}])', 'E714:')
  call assert_fails('call prop_add_list(#{type: "one"}, [[1, 1, "a"]])', 'E474:')
  call assert_fails('call prop_add_list(#{type: "one"}, [[2, 2]])', 'E474:')
  call assert_fails('call prop_add_list(#{type: "one"}, [[1, 1, 2], [2, 2]])', 'E474:')
  call assert_fails('call prop_add_list(#{type: "one"}, [[1, 1, 1, 2], [4, 1, 5, 2]])', 'E966:')
  call assert_fails('call prop_add_list(#{type: "one"}, [[3, 1, 1, 2]])', 'E966:')
  call assert_fails('call prop_add_list(#{type: "one"}, [[2, 2, 2, 2], [3, 20, 3, 22]])', 'E964:')
  call assert_fails('eval #{type: "one"}->prop_add_list([[2, 2, 2, 2], [3, 20, 3, 22]])', 'E964:')
  call assert_fails('call prop_add_list(test_null_dict(), [[2, 2, 2]])', 'E965:')
  call assert_fails('call prop_add_list(#{type: "one"}, test_null_list())', 'E1298:')
  call assert_fails('call prop_add_list(#{type: "one"}, [test_null_list()])', 'E714:')

  " only one error for multiple wrong values
  call assert_fails('call prop_add_list(#{type: "one"}, [[{}, [], 0z00, 0.3]])', ['E728:', 'E728:'])
  call DeletePropTypes()
  bw!
endfunc

func Test_prop_remove()
  new
  call AddPropTypes()
  call SetupPropsInFirstLine()
  let props = Get_expected_props()
  call assert_equal(props, prop_list(1))

  " remove by id
  call assert_equal(1, {'id': 12}->prop_remove(1))
  unlet props[2]
  call assert_equal(props, prop_list(1))

  " remove by type
  call assert_equal(1, prop_remove({'type': 'one'}, 1))
  unlet props[1]
  call assert_equal(props, prop_list(1))

  " remove from unknown buffer
  call assert_fails("call prop_remove({'type': 'one', 'bufnr': 123456}, 1)", 'E158:')

  call DeletePropTypes()
  bwipe!

  new
  call AddPropTypes()
  call SetupPropsInFirstLine()
  call prop_add(1, 6, {'length': 2, 'id': 11, 'type': 'three'})
  let props = Get_expected_props()
  call insert(props, #{type_bufnr: 0, col: 6, length: 2, id: 11, type: 'three', start: 1, end: 1}, 3)
  call assert_equal(props, prop_list(1))
  call assert_equal(1, prop_remove({'type': 'three', 'id': 11, 'both': 1, 'all': 1}, 1))
  unlet props[3]
  call assert_equal(props, prop_list(1))

  call assert_fails("call prop_remove({'id': 11, 'both': 1})", 'E860:')
  call assert_fails("call prop_remove({'type': 'three', 'both': 1})", 'E860:')

  call DeletePropTypes()
  bwipe!

  new
  call AddPropTypes()
  call SetupPropsInFirstLine()
  let props = Get_expected_props() " [whole, one, two, three]
  call assert_equal(props, prop_list(1))

  " remove one by types
  call assert_equal(1, prop_remove({'types': ['one', 'two', 'three']}, 1))
  unlet props[1] " [whole, two, three]
  call assert_equal(props, prop_list(1))

  " remove 'all' by types
  call assert_equal(2, prop_remove({'types': ['three', 'whole'], 'all': 1}, 1))
  unlet props[0] " [two, three]
  unlet props[1] " [three]
  call assert_equal(props, prop_list(1))

  " remove none by types
  call assert_equal(0, prop_remove({'types': ['three', 'whole'], 'all': 1}, 1))
  call assert_equal(props, prop_list(1))

  " no types
  call assert_fails("call prop_remove({'types': []}, 1)", 'E968:')
  call assert_fails("call prop_remove({'types': ['not_a_real_type']}, 1)", 'E971:')

  " only one of types and type can be supplied
  call assert_fails("call prop_remove({'type': 'one', 'types': ['three'], 'all': 1}, 1)", 'E1295:')

  call DeletePropTypes()
  bwipe!
endfunc

def Test_prop_add_vim9()
  prop_type_add('comment', {
      highlight: 'Directory',
      priority: 123,
      start_incl: true,
      end_incl: true,
      combine: false,
    })
  prop_type_delete('comment')
enddef

def Test_prop_remove_vim9()
  new
  g:AddPropTypes()
  g:SetupPropsInFirstLine()
  assert_equal(1, prop_remove({type: 'three', id: 13, both: true, all: true}))
  g:DeletePropTypes()
  bwipe!
enddef

func SetupOneLine()
  call setline(1, 'xonex xtwoxx')
  normal gg0
  call AddPropTypes()
  call prop_add(1, 2, {'length': 3, 'id': 11, 'type': 'one'})
  call prop_add(1, 8, {'length': 3, 'id': 12, 'type': 'two'})
  let expected = [
	\ #{type_bufnr: 0, col: 2, length: 3, id: 11, type: 'one', start: 1, end: 1},
	\ #{type_bufnr: 0, col: 8, length: 3, id: 12, type: 'two', start: 1, end: 1},
	\]
  call assert_equal(expected, prop_list(1))
  return expected
endfunc

func Test_prop_add_remove_buf()
  new
  let bufnr = bufnr('')
  call AddPropTypes()
  for lnum in range(1, 4)
    call setline(lnum, 'one two three')
  endfor
  wincmd w
  for lnum in range(1, 4)
    call prop_add(lnum, 1, {'length': 3, 'id': 11, 'type': 'one', 'bufnr': bufnr})
    call prop_add(lnum, 5, {'length': 3, 'id': 12, 'type': 'two', 'bufnr': bufnr})
    call prop_add(lnum, 11, {'length': 3, 'id': 13, 'type': 'three', 'bufnr': bufnr})
  endfor

  let props = [
	\ #{type_bufnr: 0, col: 1, length: 3, id: 11, type: 'one', start: 1, end: 1},
	\ #{type_bufnr: 0, col: 5, length: 3, id: 12, type: 'two', start: 1, end: 1},
	\ #{type_bufnr: 0, col: 11, length: 3, id: 13, type: 'three', start: 1, end: 1},
	\]
  call assert_equal(props, prop_list(1, {'bufnr': bufnr}))

  " remove by id
  let before_props = deepcopy(props)
  unlet props[1]

  call prop_remove({'id': 12, 'bufnr': bufnr}, 1)
  call assert_equal(props, prop_list(1, {'bufnr': bufnr}))
  call assert_equal(before_props, prop_list(2, {'bufnr': bufnr}))
  call assert_equal(before_props, prop_list(3, {'bufnr': bufnr}))
  call assert_equal(before_props, prop_list(4, {'bufnr': bufnr}))

  call prop_remove({'id': 12, 'bufnr': bufnr}, 3, 4)
  call assert_equal(props, prop_list(1, {'bufnr': bufnr}))
  call assert_equal(before_props, prop_list(2, {'bufnr': bufnr}))
  call assert_equal(props, prop_list(3, {'bufnr': bufnr}))
  call assert_equal(props, prop_list(4, {'bufnr': bufnr}))

  call prop_remove({'id': 12, 'bufnr': bufnr})
  for lnum in range(1, 4)
    call assert_equal(props, prop_list(lnum, {'bufnr': bufnr}))
  endfor

  " remove by type
  let before_props = deepcopy(props)
  unlet props[0]

  call prop_remove({'type': 'one', 'bufnr': bufnr}, 1)
  call assert_equal(props, prop_list(1, {'bufnr': bufnr}))
  call assert_equal(before_props, prop_list(2, {'bufnr': bufnr}))
  call assert_equal(before_props, prop_list(3, {'bufnr': bufnr}))
  call assert_equal(before_props, prop_list(4, {'bufnr': bufnr}))

  call prop_remove({'type': 'one', 'bufnr': bufnr}, 3, 4)
  call assert_equal(props, prop_list(1, {'bufnr': bufnr}))
  call assert_equal(before_props, prop_list(2, {'bufnr': bufnr}))
  call assert_equal(props, prop_list(3, {'bufnr': bufnr}))
  call assert_equal(props, prop_list(4, {'bufnr': bufnr}))

  call prop_remove({'type': 'one', 'bufnr': bufnr})
  for lnum in range(1, 4)
    call assert_equal(props, prop_list(lnum, {'bufnr': bufnr}))
  endfor

  call DeletePropTypes()
  wincmd w
  bwipe!
endfunc

func Test_prop_backspace()
  new
  set bs=2
  let expected = SetupOneLine() " 'xonex xtwoxx'

  exe "normal 0li\<BS>\<Esc>fxli\<BS>\<Esc>"
  call assert_equal('one xtwoxx', getline(1))
  let expected[0].col = 1
  let expected[1].col = 6
  call assert_equal(expected, prop_list(1))

  call DeletePropTypes()
  bwipe!
  set bs&
endfunc

func Test_prop_change()
  new
  let expected = SetupOneLine() " 'xonex xtwoxx'

  " Characterwise.
  exe "normal 7|c$\<Esc>"
  call assert_equal('xonex ', getline(1))
  call assert_equal(expected[:0], prop_list(1))
  " Linewise.
  exe "normal cc\<Esc>"
  call assert_equal('', getline(1))
  call assert_equal([], prop_list(1))

  call DeletePropTypes()
  bwipe!
  set bs&
endfunc

func Test_prop_replace()
  new
  set bs=2
  let expected = SetupOneLine() " 'xonex xtwoxx'

  exe "normal 0Ryyy\<Esc>"
  call assert_equal('yyyex xtwoxx', getline(1))
  call assert_equal(expected, prop_list(1))

  exe "normal ftRyy\<BS>"
  call assert_equal('yyyex xywoxx', getline(1))
  call assert_equal(expected, prop_list(1))

  exe "normal 0fwRyy\<BS>"
  call assert_equal('yyyex xyyoxx', getline(1))
  call assert_equal(expected, prop_list(1))

  exe "normal 0foRyy\<BS>\<BS>"
  call assert_equal('yyyex xyyoxx', getline(1))
  call assert_equal(expected, prop_list(1))

  " Replace three 1-byte chars with three 2-byte ones.
  exe "normal 0l3rø"
  call assert_equal('yøøøx xyyoxx', getline(1))
  let expected[0].length += 3
  let expected[1].col += 3
  call assert_equal(expected, prop_list(1))

  call DeletePropTypes()
  bwipe!
  set bs&
endfunc

func Test_prop_open_line()
  new

  " open new line, props stay in top line
  let expected = SetupOneLine() " 'xonex xtwoxx'
  exe "normal o\<Esc>"
  call assert_equal('xonex xtwoxx', getline(1))
  call assert_equal('', getline(2))
  call assert_equal(expected, prop_list(1))
  call DeletePropTypes()

  " move all props to next line
  let expected = SetupOneLine() " 'xonex xtwoxx'
  exe "normal 0i\<CR>\<Esc>"
  call assert_equal('', getline(1))
  call assert_equal('xonex xtwoxx', getline(2))
  call assert_equal(expected, prop_list(2))
  call DeletePropTypes()

  " split just before prop, move all props to next line
  let expected = SetupOneLine() " 'xonex xtwoxx'
  exe "normal 0li\<CR>\<Esc>"
  call assert_equal('x', getline(1))
  call assert_equal('onex xtwoxx', getline(2))
  let expected[0].col -= 1
  let expected[1].col -= 1
  call assert_equal(expected, prop_list(2))
  call DeletePropTypes()

  " split inside prop, split first prop
  let expected = SetupOneLine() " 'xonex xtwoxx'
  exe "normal 0lli\<CR>\<Esc>"
  call assert_equal('xo', getline(1))
  call assert_equal('nex xtwoxx', getline(2))
  let exp_first = [deepcopy(expected[0])]
  let exp_first[0].length = 1
  let exp_first[0].end = 0
  call assert_equal(exp_first, prop_list(1))
  let expected[0].col = 1
  let expected[0].length = 2
  let expected[0].start = 0
  let expected[1].col -= 2
  call assert_equal(expected, prop_list(2))
  call DeletePropTypes()

  " split just after first prop, second prop move to next line
  let expected = SetupOneLine() " 'xonex xtwoxx'
  exe "normal 0fea\<CR>\<Esc>"
  call assert_equal('xone', getline(1))
  call assert_equal('x xtwoxx', getline(2))
  let exp_first = expected[0:0]
  call assert_equal(exp_first, prop_list(1))
  let expected = expected[1:1]
  let expected[0].col -= 4
  call assert_equal(expected, prop_list(2))
  call DeletePropTypes()

  " split at the space character with 'ai' active, the leading space is removed
  " in the second line and the prop is shifted accordingly.
  let expected = SetupOneLine() " 'xonex xtwoxx'
  set ai
  exe "normal 6|i\<CR>\<Esc>"
  call assert_equal('xonex', getline(1))
  call assert_equal('xtwoxx', getline(2))
  let expected[1].col -= 6
  call assert_equal(expected, prop_list(1) + prop_list(2))
  set ai&
  call DeletePropTypes()

  bwipe!
  set bs&
endfunc

func Test_prop_put()
  new
  let expected = SetupOneLine() " 'xonex xtwoxx'

  let @a = 'new'
  " insert just after the prop
  normal 03l"ap
  " insert inside the prop
  normal 02l"ap
  " insert just before the prop
  normal 0"ap

  call assert_equal('xnewonnewenewx xtwoxx', getline(1))
  let expected[0].col += 3
  let expected[0].length += 3
  let expected[1].col += 9
  call assert_equal(expected, prop_list(1))

  " Visually select 4 chars in the prop and put "AB" to replace them
  let @a = 'AB'
  normal 05lv3l"ap
  call assert_equal('xnewoABenewx xtwoxx', getline(1))
  let expected[0].length -= 2
  let expected[1].col -= 2
  call assert_equal(expected, prop_list(1))

  call DeletePropTypes()
  bwipe!
endfunc

func Test_prop_clear()
  new
  call AddPropTypes()
  call SetupPropsInFirstLine()
  call assert_equal(Get_expected_props(), prop_list(1))

  eval 1->prop_clear()
  call assert_equal([], 1->prop_list())

  call DeletePropTypes()
  bwipe!
endfunc

func Test_prop_clear_buf()
  new
  call AddPropTypes()
  call SetupPropsInFirstLine()
  let bufnr = bufnr('')
  wincmd w
  call assert_equal(Get_expected_props(), prop_list(1, {'bufnr': bufnr}))

  call prop_clear(1, 1, {'bufnr': bufnr})
  call assert_equal([], prop_list(1, {'bufnr': bufnr}))

  wincmd w
  call DeletePropTypes()
  bwipe!
endfunc

func Test_prop_setline()
  new
  call AddPropTypes()
  call SetupPropsInFirstLine()
  call assert_equal(Get_expected_props(), prop_list(1))

  call setline(1, 'foobar')
  call assert_equal([], prop_list(1))

  call DeletePropTypes()
  bwipe!
endfunc

func Test_prop_setbufline()
  new
  call AddPropTypes()
  call SetupPropsInFirstLine()
  let bufnr = bufnr('')
  wincmd w
  call assert_equal(Get_expected_props(), prop_list(1, {'bufnr': bufnr}))

  call setbufline(bufnr, 1, 'foobar')
  call assert_equal([], prop_list(1, {'bufnr': bufnr}))

  wincmd w
  call DeletePropTypes()
  bwipe!
endfunc

func Test_prop_substitute()
  new
  " Set first line to 'one two three'
  call AddPropTypes()
  call SetupPropsInFirstLine()
  let expected_props = Get_expected_props()
  call assert_equal(expected_props, prop_list(1))

  " Change "n" in "one" to XX: 'oXXe two three'
  s/n/XX/
  let expected_props[0].length += 1
  let expected_props[1].length += 1
  let expected_props[2].col += 1
  let expected_props[3].col += 1
  call assert_equal(expected_props, prop_list(1))

  " Delete "t" in "two" and "three" to XX: 'oXXe wo hree'
  s/t//g
  let expected_props[0].length -= 2
  let expected_props[2].length -= 1
  let expected_props[3].length -= 1
  let expected_props[3].col -= 1
  call assert_equal(expected_props, prop_list(1))

  " Split the line by changing w to line break: 'oXXe ', 'o hree'
  " The long prop is split and spans both lines.
  " The props on "two" and "three" move to the next line.
  s/w/\r/
  let new_props = [
	\ copy(expected_props[0]),
	\ copy(expected_props[2]),
	\ copy(expected_props[3]),
	\ ]
  let expected_props[0].length = 5
  let expected_props[0].end = 0
  unlet expected_props[3]
  unlet expected_props[2]
  call assert_equal(expected_props, prop_list(1))

  let new_props[0].length = 6
  let new_props[0].start = 0
  let new_props[1].col = 1
  let new_props[1].length = 1
  let new_props[2].col = 3
  call assert_equal(new_props, prop_list(2))

  call DeletePropTypes()
  bwipe!
endfunc

func Test_prop_change_indent()
  call prop_type_add('comment', {'highlight': 'Directory'})
  new
  call setline(1, ['    xxx', 'yyyyy'])
  call prop_add(2, 2, {'length': 2, 'type': 'comment'})
  let expect = #{type_bufnr: 0, col: 2, length: 2, type: 'comment', start: 1, end: 1, id: 0}
  call assert_equal([expect], prop_list(2))

  set shiftwidth=3
  normal 2G>>
  call assert_equal('   yyyyy', getline(2))
  let expect.col += 3
  call assert_equal([expect], prop_list(2))

  normal 2G==
  call assert_equal('    yyyyy', getline(2))
  let expect.col = 6
  call assert_equal([expect], prop_list(2))

  call prop_clear(2)
  call prop_add(2, 2, {'length': 5, 'type': 'comment'})
  let expect.col = 2
  let expect.length = 5
  call assert_equal([expect], prop_list(2))

  normal 2G<<
  call assert_equal(' yyyyy', getline(2))
  let expect.length = 2
  call assert_equal([expect], prop_list(2))

  set shiftwidth&
  call prop_type_delete('comment')
endfunc

" Setup a three line prop in lines 2 - 4.
" Add short props in line 1 and 5.
func Setup_three_line_prop()
  new
  call setline(1, ['one', 'twotwo', 'three', 'fourfour', 'five'])
  call prop_add(1, 2, {'length': 1, 'type': 'comment'})
  call prop_add(2, 4, {'end_lnum': 4, 'end_col': 5, 'type': 'comment'})
  call prop_add(5, 2, {'length': 1, 'type': 'comment'})
endfunc

func Test_prop_multiline()
  eval 'comment'->prop_type_add({'highlight': 'Directory'})
  new
  call setline(1, ['xxxxxxx', 'yyyyyyyyy', 'zzzzzzzz'])

  " start halfway line 1, end halfway line 3
  call prop_add(1, 3, {'end_lnum': 3, 'end_col': 5, 'type': 'comment'})
  let expect1 = #{type_bufnr: 0, col: 3, length: 6, type: 'comment', start: 1, end: 0, id: 0}
  call assert_equal([expect1], prop_list(1))
  let expect2 = #{type_bufnr: 0, col: 1, length: 10, type: 'comment', start: 0, end: 0, id: 0}
  call assert_equal([expect2], prop_list(2))
  let expect3 = #{type_bufnr: 0, col: 1, length: 4, type: 'comment', start: 0, end: 1, id: 0}
  call assert_equal([expect3], prop_list(3))
  call prop_clear(1, 3)

  " include all three lines
  call prop_add(1, 1, {'end_lnum': 3, 'end_col': 999, 'type': 'comment'})
  let expect1.col = 1
  let expect1.length = 8
  call assert_equal([expect1], prop_list(1))
  call assert_equal([expect2], prop_list(2))
  let expect3.length = 9
  call assert_equal([expect3], prop_list(3))
  call prop_clear(1, 3)

  bwipe!

  " Test deleting the first line of a multi-line prop.
  call Setup_three_line_prop()
  let expect_short = #{type_bufnr: 0, col: 2, length: 1, type: 'comment', start: 1, end: 1, id: 0}
  call assert_equal([expect_short], prop_list(1))
  let expect2 = #{type_bufnr: 0, col: 4, length: 4, type: 'comment', start: 1, end: 0, id: 0}
  call assert_equal([expect2], prop_list(2))
  2del
  call assert_equal([expect_short], prop_list(1))
  let expect2 = #{type_bufnr: 0, col: 1, length: 6, type: 'comment', start: 1, end: 0, id: 0}
  call assert_equal([expect2], prop_list(2))
  bwipe!

  " Test deleting the last line of a multi-line prop.
  call Setup_three_line_prop()
  let expect3 = #{type_bufnr: 0, col: 1, length: 6, type: 'comment', start: 0, end: 0, id: 0}
  call assert_equal([expect3], prop_list(3))
  let expect4 = #{type_bufnr: 0, col: 1, length: 4, type: 'comment', start: 0, end: 1, id: 0}
  call assert_equal([expect4], prop_list(4))
  4del
  let expect3.end = 1
  call assert_equal([expect3], prop_list(3))
  call assert_equal([expect_short], prop_list(4))
  bwipe!

  " Test appending a line below the multi-line text prop start.
  call Setup_three_line_prop()
  let expect2 = #{type_bufnr: 0, col: 4, length: 4, type: 'comment', start: 1, end: 0, id: 0}
  call assert_equal([expect2], prop_list(2))
  call append(2, "new line")
  call assert_equal([expect2], prop_list(2))
  let expect3 = #{type_bufnr: 0, col: 1, length: 9, type: 'comment', start: 0, end: 0, id: 0}
  call assert_equal([expect3], prop_list(3))
  bwipe!

  call prop_type_delete('comment')
endfunc

func Run_test_with_line2byte(add_props)
  new
  setlocal ff=unix
  if a:add_props
    call prop_type_add('textprop', #{highlight: 'Search'})
  endif
  " Add a text prop to every fourth line and then change every fifth line so
  " that it causes a data block split a few times.
  for nr in range(1, 1000)
    call setline(nr, 'some longer text here')
    if a:add_props && nr % 4 == 0
      call prop_add(nr, 13, #{type: 'textprop', length: 4})
    endif
  endfor
  let expected = 22 * 997 + 1
  call assert_equal(expected, line2byte(998))

  for nr in range(1, 1000, 5)
    exe nr .. "s/longer/much more/"
    let expected += 3
    call assert_equal(expected, line2byte(998), 'line ' .. nr)
  endfor

  if a:add_props
    call prop_type_delete('textprop')
  endif
  bwipe!
endfunc

func Test_prop_line2byte()
  call prop_type_add('comment', {'highlight': 'Directory'})
  new
  call setline(1, ['line1', 'second line', ''])
  set ff=unix
  call assert_equal(19, line2byte(3))
  call prop_add(1, 1, {'end_col': 3, 'type': 'comment'})
  call assert_equal(19, line2byte(3))
  bwipe!

  new
  setlocal ff=unix
  call setline(1, range(500))
  call assert_equal(1491, line2byte(401))
  call prop_add(2, 1, {'type': 'comment'})
  call prop_add(222, 1, {'type': 'comment'})
  call assert_equal(1491, line2byte(401))
  call prop_remove({'type': 'comment'})
  call assert_equal(1491, line2byte(401))
  bwipe!

  new
  setlocal ff=unix
  call setline(1, range(520))
  call assert_equal(1491, line2byte(401))
  call prop_add(2, 1, {'type': 'comment'})
  call assert_equal(1491, line2byte(401))
  2delete
  call assert_equal(1489, line2byte(400))
  bwipe!

  " Add many lines so that the data block is split.
  " With and without props should give the same result.
  call Run_test_with_line2byte(0)
  call Run_test_with_line2byte(1)

  call prop_type_delete('comment')
endfunc

func Test_prop_byte2line()
  new
  set ff=unix
  call setline(1, ['one one', 'two two', 'three three', 'four four', 'five'])
  call assert_equal(4, byte2line(line2byte(4)))
  call assert_equal(5, byte2line(line2byte(5)))

  call prop_type_add('prop', {'highlight': 'Directory'})
  call prop_add(3, 1, {'length': 5, 'type': 'prop'})
  call assert_equal(4, byte2line(line2byte(4)))
  call assert_equal(5, byte2line(line2byte(5)))

  bwipe!
  call prop_type_delete('prop')
endfunc

func Test_prop_goto_byte()
  new
  call setline(1, '')
  call setline(2, 'two three')
  call setline(3, '')
  call setline(4, 'four five')

  call prop_type_add('testprop', {'highlight': 'Directory'})
  call search('^two')
  call prop_add(line('.'), col('.'), {
        \ 'length': len('two'),
        \ 'type':   'testprop'
        \ })

  call search('two \zsthree')
  let expected_pos = line2byte(line('.')) + col('.') - 1
  exe expected_pos .. 'goto'
  let actual_pos = line2byte(line('.')) + col('.') - 1
  eval actual_pos->assert_equal(expected_pos)

  call search('four \zsfive')
  let expected_pos = line2byte(line('.')) + col('.') - 1
  exe expected_pos .. 'goto'
  let actual_pos = line2byte(line('.')) + col('.') - 1
  eval actual_pos->assert_equal(expected_pos)

  call prop_type_delete('testprop')
  bwipe!
endfunc

func Test_prop_undo()
  new
  call prop_type_add('comment', {'highlight': 'Directory'})
  call setline(1, ['oneone', 'twotwo', 'three'])
  " Set 'undolevels' to break changes into undo-able pieces.
  set ul&

  call prop_add(1, 3, {'end_col': 5, 'type': 'comment'})
  let expected = [#{type_bufnr: 0, col: 3, length: 2, id: 0, type: 'comment', start: 1, end: 1}]
  call assert_equal(expected, prop_list(1))

  " Insert a character, then undo.
  exe "normal 0lllix\<Esc>"
  set ul&
  let expected[0].length = 3
  call assert_equal(expected, prop_list(1))
  undo
  let expected[0].length = 2
  call assert_equal(expected, prop_list(1))

  " Delete a character, then undo
  exe "normal 0lllx"
  set ul&
  let expected[0].length = 1
  call assert_equal(expected, prop_list(1))
  undo
  let expected[0].length = 2
  call assert_equal(expected, prop_list(1))

  " Delete the line, then undo
  1d
  set ul&
  call assert_equal([], prop_list(1))
  undo
  call assert_equal(expected, prop_list(1))

  " Insert a character, delete two characters, then undo with "U"
  exe "normal 0lllix\<Esc>"
  set ul&
  let expected[0].length = 3
  call assert_equal(expected, prop_list(1))
  exe "normal 0lllxx"
  set ul&
  let expected[0].length = 1
  call assert_equal(expected, prop_list(1))
  normal U
  let expected[0].length = 2
  call assert_equal(expected, prop_list(1))

  " substitute a word, then undo
  call setline(1, 'the number 123 is highlighted.')
  call prop_add(1, 12, {'length': 3, 'type': 'comment'})
  let expected = [#{type_bufnr: 0, col: 12, length: 3, id: 0, type: 'comment', start: 1, end: 1} ]
  call assert_equal(expected, prop_list(1))
  set ul&
  1s/number/foo
  let expected[0].col = 9
  call assert_equal(expected, prop_list(1))
  undo
  let expected[0].col = 12
  call assert_equal(expected, prop_list(1))
  call prop_clear(1)

  " substitute with backslash
  call setline(1, 'the number 123 is highlighted.')
  call prop_add(1, 12, {'length': 3, 'type': 'comment'})
  let expected = [#{type_bufnr: 0, col: 12, length: 3, id: 0, type: 'comment', start: 1, end: 1} ]
  call assert_equal(expected, prop_list(1))
  1s/the/\The
  call assert_equal(expected, prop_list(1))
  1s/^/\\
  let expected[0].col += 1
  call assert_equal(expected, prop_list(1))
  1s/^/\~
  let expected[0].col += 1
  call assert_equal(expected, prop_list(1))
  1s/123/12\\3
  let expected[0].length += 1
  call assert_equal(expected, prop_list(1))
  call prop_clear(1)

  bwipe!
  call prop_type_delete('comment')
endfunc

func Test_prop_delete_text()
  new
  call prop_type_add('comment', {'highlight': 'Directory'})
  call setline(1, ['oneone', 'twotwo', 'three'])

  " zero length property
  call prop_add(1, 3, {'type': 'comment'})
  let expected = [#{type_bufnr: 0, col: 3, length: 0, id: 0, type: 'comment', start: 1, end: 1} ]
  call assert_equal(expected, prop_list(1))

  " delete one char moves the property
  normal! x
  let expected = [#{type_bufnr: 0, col: 2, length: 0, id: 0, type: 'comment', start: 1, end: 1} ]
  call assert_equal(expected, prop_list(1))

  " delete char of the property has no effect
  normal! lx
  let expected = [#{type_bufnr: 0, col: 2, length: 0, id: 0, type: 'comment', start: 1, end: 1} ]
  call assert_equal(expected, prop_list(1))

  " delete more chars moves property to first column, is not deleted
  normal! 0xxxx
  let expected = [#{type_bufnr: 0, col: 1, length: 0, id: 0, type: 'comment', start: 1, end: 1} ]
  call assert_equal(expected, prop_list(1))

  bwipe!
  call prop_type_delete('comment')
endfunc

" screenshot test with textprop highlighting
func Test_textprop_screenshot_various()
  CheckScreendump
  " The Vim running in the terminal needs to use utf-8.
  if g:orig_encoding != 'utf-8'
    throw 'Skipped: not using utf-8'
  endif
  call writefile([
	\ "call setline(1, ["
	\	.. "'One two',"
	\	.. "'Numbér 123 änd thœn 4¾7.',"
	\	.. "'--aa--bb--cc--dd--',"
	\	.. "'// comment with error in it',"
	\	.. "'first line',"
	\	.. "'  second line  ',"
	\	.. "'third line',"
	\	.. "'   fourth line',"
	\	.. "])",
	\ "hi NumberProp ctermfg=blue",
	\ "hi LongProp ctermbg=yellow",
	\ "hi BackgroundProp ctermbg=lightgrey",
	\ "hi UnderlineProp cterm=underline",
	\ "call prop_type_add('number', {'highlight': 'NumberProp'})",
	\ "call prop_type_add('long', {'highlight': 'NumberProp'})",
	\ "call prop_type_change('long', {'highlight': 'LongProp'})",
	\ "call prop_type_add('start', {'highlight': 'NumberProp', 'start_incl': 1})",
	\ "call prop_type_add('end', {'highlight': 'NumberProp', 'end_incl': 1})",
	\ "call prop_type_add('both', {'highlight': 'NumberProp', 'start_incl': 1, 'end_incl': 1})",
	\ "call prop_type_add('background', {'highlight': 'BackgroundProp', 'combine': 0})",
	\ "call prop_type_add('backgroundcomb', {'highlight': 'NumberProp', 'combine': 1})",
	\ "eval 'backgroundcomb'->prop_type_change({'highlight': 'BackgroundProp'})",
	\ "call prop_type_add('error', {'highlight': 'UnderlineProp'})",
	\ "call prop_add(1, 4, {'end_lnum': 3, 'end_col': 3, 'type': 'long'})",
	\ "call prop_add(2, 9, {'length': 3, 'type': 'number'})",
	\ "call prop_add(2, 24, {'length': 4, 'type': 'number'})",
	\ "call prop_add(3, 3, {'length': 2, 'type': 'number'})",
	\ "call prop_add(3, 7, {'length': 2, 'type': 'start'})",
	\ "call prop_add(3, 11, {'length': 2, 'type': 'end'})",
	\ "call prop_add(3, 15, {'length': 2, 'type': 'both'})",
	\ "call prop_add(4, 6, {'length': 3, 'type': 'background'})",
	\ "call prop_add(4, 12, {'length': 10, 'type': 'backgroundcomb'})",
	\ "call prop_add(4, 17, {'length': 5, 'type': 'error'})",
	\ "call prop_add(5, 7, {'length': 4, 'type': 'long'})",
	\ "call prop_add(6, 1, {'length': 8, 'type': 'long'})",
	\ "call prop_add(8, 1, {'length': 1, 'type': 'long'})",
	\ "call prop_add(8, 11, {'length': 4, 'type': 'long'})",
	\ "set number cursorline",
	\ "hi clear SpellBad",
	\ "set spell",
	\ "syn match Comment '//.*'",
	\ "hi Comment ctermfg=green",
	\ "normal 3G0llix\<Esc>lllix\<Esc>lllix\<Esc>lllix\<Esc>lllix\<Esc>lllix\<Esc>lllix\<Esc>lllix\<Esc>",
	\ "normal 3G0lli\<BS>\<Esc>",
	\ "normal 6G0i\<BS>\<Esc>",
	\ "normal 3J",
	\ "normal 3G",
	\], 'XtestProp', 'D')
  let buf = RunVimInTerminal('-S XtestProp', {'rows': 8})
  call VerifyScreenDump(buf, 'Test_textprop_01', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_textprop_hl_override()
  CheckScreendump

  let lines =<< trim END
      call setline(1, ['One one one one one', 'Two two two two two', 'Three three three three'])
      hi OverProp ctermfg=blue ctermbg=yellow
      hi CursorLine cterm=bold,underline ctermfg=red ctermbg=green
      hi Vsual ctermfg=cyan ctermbg=grey
      call prop_type_add('under', #{highlight: 'OverProp'})
      call prop_type_add('over', #{highlight: 'OverProp', override: 1})
      call prop_add(1, 5, #{type: 'under', length: 4})
      call prop_add(1, 13, #{type: 'over', length: 4})
      call prop_add(2, 5, #{type: 'under', length: 4})
      call prop_add(2, 13, #{type: 'over', length: 4})
      call prop_add(3, 5, #{type: 'under', length: 4})
      call prop_add(3, 13, #{type: 'over', length: 4})
      set cursorline
      2
  END
  call writefile(lines, 'XtestOverProp', 'D')
  let buf = RunVimInTerminal('-S XtestOverProp', {'rows': 8})
  call VerifyScreenDump(buf, 'Test_textprop_hl_override_1', {})

  call term_sendkeys(buf, "3Gllv$hh")
  call VerifyScreenDump(buf, 'Test_textprop_hl_override_2', {})
  call term_sendkeys(buf, "\<Esc>")

  " clean up
  call StopVimInTerminal(buf)
endfunc

func RunTestVisualBlock(width, dump)
  call writefile([
	\ "call setline(1, ["
	\	.. "'xxxxxxxxx 123 x',"
	\	.. "'xxxxxxxx 123 x',"
	\	.. "'xxxxxxx 123 x',"
	\	.. "'xxxxxx 123 x',"
	\	.. "'xxxxx 123 x',"
	\	.. "'xxxx 123 xx',"
	\	.. "'xxx 123 xxx',"
	\	.. "'xx 123 xxxx',"
	\	.. "'x 123 xxxxx',"
	\	.. "' 123 xxxxxx',"
	\	.. "])",
	\ "hi SearchProp ctermbg=yellow",
	\ "call prop_type_add('search', {'highlight': 'SearchProp'})",
	\ "call prop_add(1, 11, {'length': 3, 'type': 'search'})",
	\ "call prop_add(2, 10, {'length': 3, 'type': 'search'})",
	\ "call prop_add(3, 9, {'length': 3, 'type': 'search'})",
	\ "call prop_add(4, 8, {'length': 3, 'type': 'search'})",
	\ "call prop_add(5, 7, {'length': 3, 'type': 'search'})",
	\ "call prop_add(6, 6, {'length': 3, 'type': 'search'})",
	\ "call prop_add(7, 5, {'length': 3, 'type': 'search'})",
	\ "call prop_add(8, 4, {'length': 3, 'type': 'search'})",
	\ "call prop_add(9, 3, {'length': 3, 'type': 'search'})",
	\ "call prop_add(10, 2, {'length': 3, 'type': 'search'})",
	\ "normal 1G6|\<C-V>" .. repeat('l', a:width - 1) .. "10jx",
	\], 'XtestPropVis', 'D')
  let buf = RunVimInTerminal('-S XtestPropVis', {'rows': 12})
  call VerifyScreenDump(buf, 'Test_textprop_vis_' .. a:dump, {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" screenshot test with Visual block mode operations
func Test_textprop_screenshot_visual()
  CheckScreendump

  " Delete two columns while text props are three chars wide.
  call RunTestVisualBlock(2, '01')

  " Same, but delete four columns
  call RunTestVisualBlock(4, '02')
endfunc

func Test_textprop_after_tab()
  CheckScreendump

  let lines =<< trim END
       call setline(1, [
             \ "\txxx",
             \ "x\txxx",
             \ ])
       hi SearchProp ctermbg=yellow
       call prop_type_add('search', {'highlight': 'SearchProp'})
       call prop_add(1, 2, {'length': 3, 'type': 'search'})
       call prop_add(2, 3, {'length': 3, 'type': 'search'})
  END
  call writefile(lines, 'XtextPropTab', 'D')
  let buf = RunVimInTerminal('-S XtextPropTab', {'rows': 6})
  call VerifyScreenDump(buf, 'Test_textprop_tab', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_textprop_nesting()
  CheckScreendump

  let lines =<< trim END
      vim9script
      var lines =<< trim LINESEND

          const func: func.IFunction = ({
              setLoading
            }) => {
      LINESEND
      setline(1, lines)
      prop_type_add('prop_add_test', {highlight: "ErrorMsg"})
      prop_add(2, 31, {type: 'prop_add_test', end_lnum: 4, end_col: 2})
      var text = 'text long enough to wrap line, text long enough to wrap line, text long enough to wrap line...'
      prop_add(2, 0, {type: 'prop_add_test', text_wrap: 'truncate', text_align: 'after', text: text})
  END
  call writefile(lines, 'XtextpropNesting', 'D')
  let buf = RunVimInTerminal('-S XtextpropNesting', {'rows': 8})
  call VerifyScreenDump(buf, 'Test_textprop_nesting', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_textprop_nowrap_scrolled()
  CheckScreendump

  let lines =<< trim END
       vim9script
       set nowrap
       setline(1, 'The number 123 is smaller than 4567.' .. repeat('X', &columns))
       prop_type_add('number', {'highlight': 'ErrorMsg'})
       prop_add(1, 12, {'length': 3, 'type': 'number'})
       prop_add(1, 32, {'length': 4, 'type': 'number'})
       feedkeys('gg20zl', 'nxt')
  END
  call writefile(lines, 'XtestNowrap', 'D')
  let buf = RunVimInTerminal('-S XtestNowrap', {'rows': 6})
  call VerifyScreenDump(buf, 'Test_textprop_nowrap_01', {})

  call term_sendkeys(buf, "$")
  call VerifyScreenDump(buf, 'Test_textprop_nowrap_02', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_textprop_text_priority()
  CheckScreendump

  let lines =<< trim END
      call setline(1, "function( call, argument, here )")

      call prop_type_add('one', #{highlight: 'Error'})
      call prop_type_add('two', #{highlight: 'Function'})
      call prop_type_add('three', #{highlight: 'DiffChange'})
      call prop_type_add('arg', #{highlight: 'Search'})

      call prop_add(1, 27, #{type: 'arg', length: len('here')})
      call prop_add(1, 27, #{type: 'three', text: 'three: '})
      call prop_add(1, 11, #{type: 'one', text: 'one: '})
      call prop_add(1, 11, #{type: 'arg', length: len('call')})
      call prop_add(1, 17, #{type: 'two', text: 'two: '})
      call prop_add(1, 17, #{type: 'arg', length: len('argument')})
  END
  call writefile(lines, 'XtestPropPrio', 'D')
  let buf = RunVimInTerminal('-S XtestPropPrio', {'rows': 5})
  call VerifyScreenDump(buf, 'Test_prop_at_same_pos', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_textprop_in_empty_popup()
  CheckScreendump

  let lines =<< trim END
    vim9script

    hi def link FilterMenuMatch Constant
    prop_type_add('FilterMenuMatch', {
      highlight: "FilterMenuMatch",
      override: true,
      priority: 1000,
      combine: true,
    })

    var winid = popup_create([{text: "hello", props: [
      {col: 1, length: 1, type: 'FilterMenuMatch'},
      {col: 2, length: 1, type: 'FilterMenuMatch'},
    ]}], {
      minwidth: 20,
      minheight: 10,
      cursorline: false,
      highlight: "None",
      border: [],
    })

    win_execute(winid, "setl nu cursorline cursorlineopt=both")
    popup_settext(winid, [])
    redraw
  END
  call writefile(lines, 'XtestPropEmptyPopup', 'D')
  let buf = RunVimInTerminal('-S XtestPropEmptyPopup', #{rows: 20, cols: 40})
  call VerifyScreenDump(buf, 'Test_prop_in_empty_popup', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

func Test_textprop_with_syntax()
  CheckScreendump

  let lines =<< trim END
       call setline(1, [
             \ "(abc)",
             \ ])
       syn match csParens "[()]" display
       hi! link csParens MatchParen

       call prop_type_add('TPTitle', #{ highlight: 'Title' })
       call prop_add(1, 2, #{type: 'TPTitle', end_col: 5})
  END
  call writefile(lines, 'XtestPropSyn', 'D')
  let buf = RunVimInTerminal('-S XtestPropSyn', {'rows': 6})
  call VerifyScreenDump(buf, 'Test_textprop_syn_1', {})

  " clean up
  call StopVimInTerminal(buf)
endfunc

" Adding a text property to a new buffer should not fail
func Test_textprop_empty_buffer()
  call prop_type_add('comment', {'highlight': 'Search'})
  new
  call prop_add(1, 1, {'type': 'comment'})
  close
  call prop_type_delete('comment')
endfunc

" Adding a text property with invalid highlight should be ignored.
func Test_textprop_invalid_highlight()
  call assert_fails("call prop_type_add('dni', {'highlight': 'DoesNotExist'})", 'E970:')
  new
  call setline(1, ['asdf', 'asdf'])
  call prop_add(1, 1, {'length': 4, 'type': 'dni'})
  redraw
  bwipe!
  call prop_type_delete('dni')
endfunc

" Adding a text property to an empty buffer and then editing another
func Test_textprop_empty_buffer_next()
  call prop_type_add("xxx", {})
  call prop_add(1, 1, {"type": "xxx"})
  next X
  call prop_type_delete('xxx')
endfunc

func Test_textprop_remove_from_buf()
  new
  let buf = bufnr('')
  call prop_type_add('one', {'bufnr': buf})
  call prop_add(1, 1, {'type': 'one', 'id': 234})
  file x
  edit y
  call prop_remove({'id': 234, 'bufnr': buf}, 1)
  call prop_type_delete('one', {'bufnr': buf})
  bwipe! x
  close
endfunc

func Test_textprop_in_unloaded_buf()
  edit Xaaa
  call setline(1, 'aaa')
  write
  edit Xbbb
  call setline(1, 'bbb')
  write
  let bnr = bufnr('')
  edit Xaaa

  call prop_type_add('ErrorMsg', #{highlight:'ErrorMsg'})
  call assert_fails("call prop_add(1, 1, #{end_lnum: 1, endcol: 2, type: 'ErrorMsg', bufnr: bnr})", 'E275:')
  exe 'buf ' .. bnr
  call assert_equal('bbb', getline(1))
  call assert_equal(0, prop_list(1)->len())

  bwipe! Xaaa
  bwipe! Xbbb
  cal delete('Xaaa')
  cal delete('Xbbb')
endfunc

func Test_proptype_substitute2()
  new
  " text_prop.vim
  call setline(1, [
        \ 'The   num  123 is smaller than 4567.',
        \ '123 The number 123 is smaller than 4567.',
        \ '123 The number 123 is smaller than 4567.'])

  call prop_type_add('number', {'highlight': 'ErrorMsg'})

  call prop_add(1, 12, {'length': 3, 'type': 'number'})
  call prop_add(2, 1, {'length': 3, 'type': 'number'})
  call prop_add(3, 36, {'length': 4, 'type': 'number'})
  set ul&
  let expected = [
        \ #{type_bufnr: 0, id: 0, col: 13, end: 1, type: 'number', length: 3, start: 1},
        \ #{type_bufnr: 0, id: 0, col: 1,  end: 1, type: 'number', length: 3, start: 1},
        \ #{type_bufnr: 0, id: 0, col: 50, end: 1, type: 'number', length: 4, start: 1}]

  " TODO
  if 0
    " Add some text in between
    %s/\s\+/   /g
    call assert_equal(expected, prop_list(1) + prop_list(2) + prop_list(3))

    " remove some text
    :1s/[a-z]\{3\}//g
    let expected = [{'id': 0, 'col': 10, 'end': 1, 'type': 'number', 'length': 3, 'start': 1}]
    call assert_equal(expected, prop_list(1))
  endif

  call prop_type_delete('number')
  bwipe!
endfunc

" This was causing property corruption.
func Test_proptype_substitute3()
  new
  call setline(1, ['abcxxx', 'def'])
  call prop_type_add("test", {"highlight": "Search"})
  call prop_add(1, 2, {"end_lnum": 2, "end_col": 2, "type": "test"})
  %s/x\+$//
  redraw

  call prop_type_delete('test')
  bwipe!
endfunc

func Test_proptype_substitute_join()
  new
  call setline(1, [
        \ 'This is some end',
        \ 'start is highlighted end',
        \ 'some is highlighted',
        \ 'start is also highlighted'])

  call prop_type_add('number', {'highlight': 'ErrorMsg'})

  call prop_add(1, 6, {'length': 2, 'type': 'number'})
  call prop_add(2, 7, {'length': 2, 'type': 'number'})
  call prop_add(3, 6, {'length': 2, 'type': 'number'})
  call prop_add(4, 7, {'length': 2, 'type': 'number'})
  " The highlighted "is" in line 1, 2 and 4 is kept and adjusted.
  " The highlighted "is" in line 3 is deleted.
  let expected = [
        \ #{type_bufnr: 0, id: 0, col: 6, end: 1, type: 'number', length: 2, start: 1},
        \ #{type_bufnr: 0, id: 0, col: 21, end: 1, type: 'number', length: 2, start: 1},
        \ #{type_bufnr: 0, id: 0, col: 43, end: 1, type: 'number', length: 2, start: 1}]

  s/end\nstart/joined/
  s/end\n.*\nstart/joined/
  call assert_equal('This is some joined is highlighted joined is also highlighted', getline(1))
  call assert_equal(expected, prop_list(1))

  call prop_type_delete('number')
  bwipe!
endfunc

func SaveOptions()
  let d = #{tabstop: &tabstop,
	  \ softtabstop: &softtabstop,
	  \ shiftwidth: &shiftwidth,
	  \ expandtab: &expandtab,
	  \ foldmethod: '"' .. &foldmethod .. '"',
	  \ }
  return d
endfunc

func RestoreOptions(dict)
  for name in keys(a:dict)
    exe 'let &' .. name .. ' = ' .. a:dict[name]
  endfor
endfunc

func Test_textprop_noexpandtab()
  new
  let save_dict = SaveOptions()

  set tabstop=8
  set softtabstop=4
  set shiftwidth=4
  set noexpandtab
  set foldmethod=marker

  call feedkeys("\<esc>\<esc>0Ca\<cr>\<esc>\<up>", "tx")
  call prop_type_add('test', {'highlight': 'ErrorMsg'})
  call prop_add(1, 1, {'end_col': 2, 'type': 'test'})
  call feedkeys("0i\<tab>", "tx")
  call prop_remove({'type': 'test'})
  call prop_add(1, 2, {'end_col': 3, 'type': 'test'})
  call feedkeys("A\<left>\<tab>", "tx")
  call prop_remove({'type': 'test'})
  try
    " It is correct that this does not pass
    call prop_add(1, 6, {'end_col': 7, 'type': 'test'})
    " Has already collapsed here, start_col:6 does not result in an error
    call feedkeys("A\<left>\<tab>", "tx")
  catch /^Vim\%((\a\+)\)\=:E964/
  endtry
  call prop_remove({'type': 'test'})
  call prop_type_delete('test')

  call RestoreOptions(save_dict)
  bwipe!
endfunc

func Test_textprop_noexpandtab_redraw()
  new
  let save_dict = SaveOptions()

  set tabstop=8
  set softtabstop=4
  set shiftwidth=4
  set noexpandtab
  set foldmethod=marker

  call feedkeys("\<esc>\<esc>0Ca\<cr>\<space>\<esc>\<up>", "tx")
  call prop_type_add('test', {'highlight': 'ErrorMsg'})
  call prop_add(1, 1, {'end_col': 2, 'type': 'test'})
  call feedkeys("0i\<tab>", "tx")
  " Internally broken at the next line
  call feedkeys("A\<left>\<tab>", "tx")
  redraw
  " Index calculation failed internally on next line
  call prop_add(1, 1, {'end_col': 2, 'type': 'test'})
  call prop_remove({'type': 'test', 'all': v:true})
  call prop_type_delete('test')
  call prop_type_delete('test')

  call RestoreOptions(save_dict)
  bwipe!
endfunc

func Test_textprop_ins_str()
  new
  call setline(1, 'just some text')
  call prop_type_add('test', {'highlight': 'ErrorMsg'})
  call prop_add(1, 1, {'end_col': 2, 'type': 'test'})
  call assert_equal([#{type_bufnr: 0, id: 0, col: 1, end: 1, type: 'test', length: 1, start: 1}], prop_list(1))

  call feedkeys("foi\<F8>\<Esc>", "tx")
  call assert_equal('just s<F8>ome text', getline(1))
  call assert_equal([#{type_bufnr: 0, id: 0, col: 1, end: 1, type: 'test', length: 1, start: 1}], prop_list(1))

  bwipe!
  call prop_remove({'type': 'test'})
  call prop_type_delete('test')
endfunc

func Test_find_prop_later_in_line()
  new
  call prop_type_add('test', {'highlight': 'ErrorMsg'})
  call setline(1, 'just some text')
  call prop_add(1, 1, {'length': 4, 'type': 'test'})
  call prop_add(1, 10, {'length': 3, 'type': 'test'})

  call assert_equal(
        \ #{type_bufnr: 0, id: 0, lnum: 1, col: 10, end: 1, type: 'test', length: 3, start: 1},
        \ prop_find(#{type: 'test', lnum: 1, col: 6}))

  bwipe!
  call prop_type_delete('test')
endfunc

func Test_find_zerowidth_prop_sol()
  new
  call prop_type_add('test', {'highlight': 'ErrorMsg'})
  call setline(1, 'just some text')
  call prop_add(1, 1, {'length': 0, 'type': 'test'})

  call assert_equal(
        \ #{type_bufnr: 0, id: 0, lnum: 1, col: 1, end: 1, type: 'test', length: 0, start: 1},
        \ prop_find(#{type: 'test', lnum: 1}))

  bwipe!
  call prop_type_delete('test')
endfunc

" Test for passing invalid arguments to prop_xxx() functions
func Test_prop_func_invalid_args()
  call assert_fails('call prop_clear(1, 2, [])', 'E715:')
  call assert_fails('call prop_clear(-1, 2)', 'E16:')
  call assert_fails('call prop_find(test_null_dict())', 'E1297:')
  call assert_fails('call prop_find({"bufnr" : []})', 'E730:')
  call assert_fails('call prop_find({})', 'E968:')
  call assert_fails('call prop_find({}, "x")', 'E474:')
  call assert_fails('call prop_find({"lnum" : -2})', 'E16:')
  call assert_fails('call prop_list(1, [])', 'E1206:')
  call assert_fails('call prop_list(-1, {})', 'E16:')
  call assert_fails('call prop_remove([])', 'E1206:')
  call assert_fails('call prop_remove({}, -2)', 'E16:')
  call assert_fails('call prop_remove({})', 'E968:')
  call assert_fails('call prop_type_add([], {})', 'E730:')
  call assert_fails("call prop_type_change('long', {'xyz' : 10})", 'E971:')
  call assert_fails("call prop_type_delete([])", 'E730:')
  call assert_fails("call prop_type_delete('xyz', [])", 'E715:')
  call assert_fails("call prop_type_get([])", 'E730:')
  call assert_fails("call prop_type_get('', [])", 'E475:')
  call assert_fails("call prop_type_list([])", 'E715:')
  call assert_fails("call prop_type_add('yyy', 'not_a_dict')", 'E715:')
  call assert_fails("call prop_add(1, 5, {'type':'missing_type', 'length':1})", 'E971:')
  call assert_fails("call prop_add(1, 5, {'type': ''})", 'E971:')
  call assert_fails('call prop_add(1, 1, 0)', 'E1206:')

  new
  call setline(1, ['first', 'second'])
  call prop_type_add('xxx', {})

  call assert_fails("call prop_type_add('xxx', {})", 'E969:')
  call assert_fails("call prop_add(2, 0, {'type': 'xxx'})", 'E964:')
  call assert_fails("call prop_add(2, 3, {'type': 'xxx', 'end_lnum':1})", 'E475:')
  call assert_fails("call prop_add(2, 3, {'type': 'xxx', 'end_lnum':3})", 'E966:')
  call assert_fails("call prop_add(2, 3, {'type': 'xxx', 'length':-1})", 'E475:')
  call assert_fails("call prop_add(2, 3, {'type': 'xxx', 'end_col':0})", 'E475:')
  call assert_fails("call prop_add(2, 3, {'length':1})", 'E965:')

  call prop_type_delete('xxx')
  bwipe!
endfunc

func Test_prop_split_join()
  new
  call prop_type_add('test', {'highlight': 'ErrorMsg'})
  call setline(1, 'just some text')
  call prop_add(1, 6, {'length': 4, 'type': 'test'})

  " Split in middle of "some"
  execute "normal! 8|i\<CR>"
  call assert_equal(
        \ [#{type_bufnr: 0, id: 0, col: 6, end: 0, type: 'test', length: 2, start: 1}],
        \ prop_list(1))
  call assert_equal(
        \ [#{type_bufnr: 0, id: 0, col: 1, end: 1, type: 'test', length: 2, start: 0}],
        \ prop_list(2))

  " Join the two lines back together
  normal! 1GJ
  call assert_equal([#{type_bufnr: 0, id: 0, col: 6, end: 1, type: 'test', length: 5, start: 1}], prop_list(1))

  bwipe!
  call prop_type_delete('test')
endfunc

func Test_prop_increment_decrement()
  new
  call prop_type_add('test', {'highlight': 'ErrorMsg'})
  call setline(1, 'its 998 times')
  call prop_add(1, 5, {'length': 3, 'type': 'test'})

  exe "normal! 0f9\<C-A>"
  eval getline(1)->assert_equal('its 999 times')
  eval prop_list(1)->assert_equal([
        \ #{type_bufnr: 0, id: 0, col: 5, end: 1, type: 'test', length: 3, start: 1}])

  exe "normal! 0f9\<C-A>"
  eval getline(1)->assert_equal('its 1000 times')
  eval prop_list(1)->assert_equal([
        \ #{type_bufnr: 0, id: 0, col: 5, end: 1, type: 'test', length: 4, start: 1}])

  bwipe!
  call prop_type_delete('test')
endfunc

func Test_prop_block_insert()
  new
  call prop_type_add('test', {'highlight': 'ErrorMsg'})
  call setline(1, ['one ', 'two '])
  call prop_add(1, 1, {'length': 3, 'type': 'test'})
  call prop_add(2, 1, {'length': 3, 'type': 'test'})

  " insert "xx" in the first column of both lines
  exe "normal! gg0\<C-V>jIxx\<Esc>"
  eval getline(1, 2)->assert_equal(['xxone ', 'xxtwo '])
  let expected = [#{type_bufnr: 0, id: 0, col: 3, end: 1, type: 'test', length: 3, start: 1}]
  eval prop_list(1)->assert_equal(expected)
  eval prop_list(2)->assert_equal(expected)

  " insert "yy" inside the text props to make them longer
  exe "normal! gg03l\<C-V>jIyy\<Esc>"
  eval getline(1, 2)->assert_equal(['xxoyyne ', 'xxtyywo '])
  let expected[0].length = 5
  eval prop_list(1)->assert_equal(expected)
  eval prop_list(2)->assert_equal(expected)

  " insert "zz" after the text props, text props don't change
  exe "normal! gg07l\<C-V>jIzz\<Esc>"
  eval getline(1, 2)->assert_equal(['xxoyynezz ', 'xxtyywozz '])
  eval prop_list(1)->assert_equal(expected)
  eval prop_list(2)->assert_equal(expected)

  bwipe!
  call prop_type_delete('test')
endfunc

" this was causing an ml_get error because w_botline was wrong
func Test_prop_one_line_window()
  enew
  call range(2)->setline(1)
  call prop_type_add('testprop', {})
  call prop_add(1, 1, {'type': 'testprop'})
  call popup_create('popup', {'textprop': 'testprop'})
  $
  new
  wincmd _
  call feedkeys("\r", 'xt')
  redraw

  call popup_clear()
  call prop_type_delete('testprop')
  close
  bwipe!
endfunc

def Test_prop_column_zero_error()
  prop_type_add('proptype', {highlight: 'Search'})
  var caught = false
  try
    popup_create([{
            text: 'a',
            props: [{col: 0, length: 1, type: 'type'}],
     }], {})
  catch /E964:/
    caught = true
  endtry
  assert_true(caught)

  popup_clear()
  prop_type_delete('proptype')
enddef

" This was calling ml_append_int() and copy a text property from a previous
" line at the wrong moment.  Exact text length matters.
def Test_prop_splits_data_block()
  new
  var lines: list<string> = [repeat('x', 35)]->repeat(41)
			+ [repeat('!', 35)]
			+ [repeat('x', 35)]->repeat(56)
  lines->setline(1)
  prop_type_add('someprop', {highlight: 'ErrorMsg'})
  prop_add(1, 27, {end_lnum: 1, end_col: 70, type: 'someprop'})
  prop_remove({type: 'someprop'}, 1)
  prop_add(35, 22, {end_lnum: 43, end_col: 43, type: 'someprop'})
  prop_remove({type: 'someprop'}, 35, 43)
  assert_equal([], prop_list(42))

  bwipe!
  prop_type_delete('someprop')
enddef

" This was calling ml_delete_int() and try to change text properties.
def Test_prop_add_delete_line()
  new
  var a = 10
  var b = 20
  repeat([''], a)->append('$')
  prop_type_add('Test', {highlight: 'ErrorMsg'})
  for lnum in range(1, a)
    for col in range(1, b)
      prop_add(1, 1, {end_lnum: lnum, end_col: col, type: 'Test'})
    endfor
  endfor

  # check deleting lines is OK
  :5del
  :1del
  :$del

  prop_type_delete('Test')
  bwipe!
enddef

" This test is to detect a regression related to #10430. It is not an attempt
" fully cover deleting lines in the presence of multi-line properties.
def Test_delete_line_within_multiline_prop()
  new
  setline(1, '# Top.')
  append(1, ['some_text = """', 'A string.', '"""', '# Bottom.'])
  prop_type_add('Identifier', {'highlight': 'ModeMsg', 'priority': 0, 'combine': 0, 'start_incl': 0, 'end_incl': 0})
  prop_type_add('String', {'highlight': 'MoreMsg', 'priority': 0, 'combine': 0, 'start_incl': 0, 'end_incl': 0})
  prop_add(2, 1, {'type': 'Identifier', 'end_lnum': 2, 'end_col': 9})
  prop_add(2, 13, {'type': 'String', 'end_lnum': 4, 'end_col': 4})

  # The property for line 3 should extend into the previous and next lines.
  var props = prop_list(3)
  var prop = props[0]
  assert_equal(1, len(props))
  assert_equal(0, prop['start'])
  assert_equal(0, prop['end'])

  # This deletion should run without raising an exception.
  try
    :2 del
  catch
    assert_report('Line delete should have worked, but it raised an error.')
  endtry

  # The property for line 2 (was 3) should no longer extend into the previous
  # line.
  props = prop_list(2)
  prop = props[0]
  assert_equal(1, len(props))
  assert_equal(1, prop['start'], 'Property was not changed to start within the line.')

  # This deletion should run without raising an exception.
  try
    :3 del
  catch
    assert_report('Line delete should have worked, but it raised an error.')
  endtry

  # The property for line 2 (originally 3) should no longer extend into the next
  # line.
  props = prop_list(2)
  prop = props[0]
  assert_equal(1, len(props))
  assert_equal(1, prop['end'], 'Property was not changed to end within the line.')

  prop_type_delete('Identifier')
  prop_type_delete('String')
  bwip!
enddef

func Test_prop_in_linebreak()
  CheckRunVimInTerminal

  let lines =<< trim END
    set breakindent linebreak breakat+=]
    call printf('%s]%s', repeat('x', 50), repeat('x', 70))->setline(1)
    call prop_type_add('test', #{highlight: 'MatchParen'})
    call prop_add(1, 51, #{length: 1, type: 'test'})
    func AddMatch()
      syntax on
      syntax match xTest /.*/
      hi link xTest Comment
      set signcolumn=yes
    endfunc
  END
  call writefile(lines, 'XscriptPropLinebreak', 'D')
  let buf = RunVimInTerminal('-S XscriptPropLinebreak', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_prop_linebreak_1', {})

  call term_sendkeys(buf, ":call AddMatch()\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_linebreak_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_linebreak()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      set linebreak
      setline(1, 'one twoword')
      prop_type_add('test', {highlight: 'Special'})
      prop_add(1, 4, {text: ': virtual text', type: 'test'})
  END
  call writefile(lines, 'XscriptPropWithLinebreak', 'D')
  let buf = RunVimInTerminal('-S XscriptPropWithLinebreak', #{rows: 6, cols: 50})
  call VerifyScreenDump(buf, 'Test_prop_with_linebreak_1', {})
  call term_sendkeys(buf, "iasdf asdf asdf asdf asdf as\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_with_linebreak_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_wrap()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      set linebreak
      setline(1, 'asdf '->repeat(15))
      prop_type_add('test', {highlight: 'Special'})
      prop_add(1, 43, {text: 'some virtual text', type: 'test'})
      normal G$
  END
  call writefile(lines, 'XscriptPropWithWrap', 'D')
  let buf = RunVimInTerminal('-S XscriptPropWithWrap', #{rows: 6, cols: 50})
  call VerifyScreenDump(buf, 'Test_prop_with_wrap_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_after_tab()
  CheckRunVimInTerminal

  let lines =<< trim END
    set breakindent linebreak breakat+=]
    call setline(1, "\t[xxx]")
    call prop_type_add('test', #{highlight: 'ErrorMsg'})
    call prop_add(1, 2, #{length: 1, type: 'test'})
  END
  call writefile(lines, 'XscriptPropAfterTab', 'D')
  let buf = RunVimInTerminal('-S XscriptPropAfterTab', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_prop_after_tab', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_before_tab()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ["\tx"]->repeat(6))
      call prop_type_add('test', #{highlight: 'Search'})
      call prop_add(1, 1, #{type: 'test', text: '123'})
      call prop_add(2, 1, #{type: 'test', text: '1234567'})
      call prop_add(3, 1, #{type: 'test', text: '12345678'})
      call prop_add(4, 1, #{type: 'test', text: '123456789'})
      call prop_add(5, 2, #{type: 'test', text: 'ABC'})
      call prop_add(6, 3, #{type: 'test', text: 'ABC'})
      normal gg0
  END
  call writefile(lines, 'XscriptPropBeforeTab', 'D')
  let buf = RunVimInTerminal('-S XscriptPropBeforeTab', #{rows: 8})
  call VerifyScreenDump(buf, 'Test_prop_before_tab_01', {})
  call term_sendkeys(buf, "$")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_02', {})
  call term_sendkeys(buf, "j0")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_03', {})
  call term_sendkeys(buf, "$")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_04', {})
  call term_sendkeys(buf, "j0")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_05', {})
  call term_sendkeys(buf, "$")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_06', {})
  call term_sendkeys(buf, "j0")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_07', {})
  call term_sendkeys(buf, "$")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_08', {})
  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_09', {})
  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_10', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_after_linebreak()
  CheckRunVimInTerminal

  let lines =<< trim END
      set linebreak wrap
      call printf('%s+(%s)', 'x'->repeat(&columns / 2), 'x'->repeat(&columns / 2))->setline(1)
      call prop_type_add('test', #{highlight: 'ErrorMsg'})
      call prop_add(1, (&columns / 2) + 2, #{length: 1, type: 'test'})
  END
  call writefile(lines, 'XscriptPropAfterLinebreak', 'D')
  let buf = RunVimInTerminal('-S XscriptPropAfterLinebreak', #{rows: 10})
  call VerifyScreenDump(buf, 'Test_prop_after_linebreak', {})

  call StopVimInTerminal(buf)
endfunc

" Buffer number of 0 should be ignored, as if the parameter wasn't passed.
def Test_prop_bufnr_zero()
  new
  try
    var bufnr = bufnr('')
    setline(1, 'hello')
    prop_type_add('bufnr-global', {highlight: 'ErrorMsg'})
    prop_type_add('bufnr-buffer', {highlight: 'StatusLine', bufnr: bufnr})

    prop_add(1, 1, {type: 'bufnr-global', length: 1})
    prop_add(1, 2, {type: 'bufnr-buffer', length: 1})

    var list = prop_list(1)
    assert_equal([
       {id: 0, col: 1, type_bufnr: 0,         end: 1, type: 'bufnr-global', length: 1, start: 1},
       {id: 0, col: 2, type_bufnr: bufnr, end: 1, type: 'bufnr-buffer', length: 1, start: 1},
    ], list)

    assert_equal(
      {highlight: 'ErrorMsg', end_incl: 0, start_incl: 0, priority: 0, combine: 1},
      prop_type_get('bufnr-global', {bufnr: list[0].type_bufnr}))

    assert_equal(
      {highlight: 'StatusLine', end_incl: 0, start_incl: 0, priority: 0, bufnr: bufnr, combine: 1},
      prop_type_get('bufnr-buffer', {bufnr: list[1].type_bufnr}))
  finally
    bwipe!
    prop_type_delete('bufnr-global')
  endtry
enddef

" Tests for the prop_list() function
func Test_prop_list()
  let lines =<< trim END
    new
    call g:AddPropTypes()
    call setline(1, repeat([repeat('a', 60)], 10))
    call prop_add(1, 4, {'type': 'one', 'id': 5, 'end_col': 6})
    call prop_add(1, 5, {'type': 'two', 'id': 10, 'end_col': 7})
    call prop_add(3, 12, {'type': 'one', 'id': 20, 'end_col': 14})
    call prop_add(3, 13, {'type': 'two', 'id': 10, 'end_col': 15})
    call prop_add(5, 20, {'type': 'one', 'id': 10, 'end_col': 22})
    call prop_add(5, 21, {'type': 'two', 'id': 20, 'end_col': 23})
    call assert_equal([
          \ {'id': 5, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1},
          \ {'id': 10, 'col': 5, 'type_bufnr': 0, 'end': 1,
          \  'type': 'two', 'length': 2, 'start': 1}], prop_list(1))
    #" text properties between a few lines
    call assert_equal([
          \ {'lnum': 3, 'id': 20, 'col': 12, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1},
          \ {'lnum': 3, 'id': 10, 'col': 13, 'type_bufnr': 0, 'end': 1,
          \  'type': 'two', 'length': 2, 'start': 1},
          \ {'lnum': 5, 'id': 10, 'col': 20, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1},
          \ {'lnum': 5, 'id': 20, 'col': 21, 'type_bufnr': 0, 'end': 1,
          \  'type': 'two', 'length': 2, 'start': 1}],
          \ prop_list(2, {'end_lnum': 5}))
    #" text properties across all the lines
    call assert_equal([
          \ {'lnum': 1, 'id': 5, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1},
          \ {'lnum': 3, 'id': 20, 'col': 12, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1},
          \ {'lnum': 5, 'id': 10, 'col': 20, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1}],
          \ prop_list(1, {'types': ['one'], 'end_lnum': -1}))
    #" text properties with the specified identifier
    call assert_equal([
          \ {'lnum': 3, 'id': 20, 'col': 12, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1},
          \ {'lnum': 5, 'id': 20, 'col': 21, 'type_bufnr': 0, 'end': 1,
          \  'type': 'two', 'length': 2, 'start': 1}],
          \ prop_list(1, {'ids': [20], 'end_lnum': 10}))
    #" text properties of the specified type and id
    call assert_equal([
          \ {'lnum': 1, 'id': 10, 'col': 5, 'type_bufnr': 0, 'end': 1,
          \  'type': 'two', 'length': 2, 'start': 1},
          \ {'lnum': 3, 'id': 10, 'col': 13, 'type_bufnr': 0, 'end': 1,
          \  'type': 'two', 'length': 2, 'start': 1}],
          \ prop_list(1, {'types': ['two'], 'ids': [10], 'end_lnum': 20}))
    call assert_equal([], prop_list(1, {'ids': [40, 50], 'end_lnum': 10}))
    call assert_equal([], prop_list(6, {'end_lnum': 10}))
    call assert_equal([], prop_list(2, {'end_lnum': 2}))
    #" error cases
    call assert_fails("echo prop_list(1, {'end_lnum': -20})", 'E16:')
    call assert_fails("echo prop_list(4, {'end_lnum': 2})", 'E16:')
    call assert_fails("echo prop_list(1, {'end_lnum': '$'})", 'E889:')
    call assert_fails("echo prop_list(1, {'types': ['blue'], 'end_lnum': 10})",
          \ 'E971:')
    call assert_fails("echo prop_list(1, {'types': ['one', 'blue'],
          \ 'end_lnum': 10})", 'E971:')
    call assert_fails("echo prop_list(1, {'types': ['one', 10],
          \ 'end_lnum': 10})", 'E928:')
    call assert_fails("echo prop_list(1, {'types': ['']})", 'E971:')
    call assert_equal([], prop_list(2, {'types': []}))
    call assert_equal([], prop_list(2, {'types': test_null_list()}))
    call assert_fails("call prop_list(1, {'types': {}})", 'E714:')
    call assert_fails("call prop_list(1, {'types': 'one'})", 'E714:')
    call assert_equal([], prop_list(2, {'types': ['one'],
          \ 'ids': test_null_list()}))
    call assert_equal([], prop_list(2, {'types': ['one'], 'ids': []}))
    call assert_fails("call prop_list(1, {'types': ['one'], 'ids': {}})",
          \ 'E714:')
    call assert_fails("call prop_list(1, {'types': ['one'], 'ids': 10})",
          \ 'E714:')
    call assert_fails("call prop_list(1, {'types': ['one'], 'ids': [[]]})",
          \ 'E745:')
    call assert_fails("call prop_list(1, {'types': ['one'], 'ids': [10, []]})",
          \ 'E745:')

    #" get text properties from a non-current buffer
    wincmd w
    call assert_equal([
          \ {'lnum': 1, 'id': 5, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \ 'type': 'one', 'length': 2, 'start': 1},
          \ {'lnum': 1, 'id': 10, 'col': 5, 'type_bufnr': 0, 'end': 1,
          \ 'type': 'two', 'length': 2, 'start': 1},
          \ {'lnum': 3, 'id': 20, 'col': 12, 'type_bufnr': 0, 'end': 1,
          \ 'type': 'one', 'length': 2, 'start': 1},
          \ {'lnum': 3, 'id': 10, 'col': 13, 'type_bufnr': 0, 'end': 1,
          \ 'type': 'two', 'length': 2, 'start': 1}],
          \ prop_list(1, {'bufnr': winbufnr(1), 'end_lnum': 4}))
    wincmd w

    #" get text properties after clearing all the properties
    call prop_clear(1, line('$'))
    call assert_equal([], prop_list(1, {'end_lnum': 10}))

    call prop_add(2, 4, {'type': 'one', 'id': 5, 'end_col': 6})
    call prop_add(2, 4, {'type': 'two', 'id': 10, 'end_col': 6})
    call prop_add(2, 4, {'type': 'three', 'id': 15, 'end_col': 6})
    #" get text properties with a list of types
    call assert_equal([
          \ {'id': 10, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \  'type': 'two', 'length': 2, 'start': 1},
          \ {'id': 5, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1}],
          \ prop_list(2, {'types': ['one', 'two']}))
    call assert_equal([
          \ {'id': 15, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \  'type': 'three', 'length': 2, 'start': 1},
          \ {'id': 5, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1}],
          \ prop_list(2, {'types': ['one', 'three']}))
    #" get text properties with a list of identifiers
    call assert_equal([
          \ {'id': 10, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \  'type': 'two', 'length': 2, 'start': 1},
          \ {'id': 5, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1}],
          \ prop_list(2, {'ids': [5, 10, 20]}))
    call prop_clear(1, line('$'))
    call assert_equal([], prop_list(2, {'types': ['one', 'two']}))
    call assert_equal([], prop_list(2, {'ids': [5, 10, 20]}))

    #" get text properties from a hidden buffer
    edit! Xaaa
    call setline(1, repeat([repeat('b', 60)], 10))
    call prop_add(1, 4, {'type': 'one', 'id': 5, 'end_col': 6})
    call prop_add(4, 8, {'type': 'two', 'id': 10, 'end_col': 10})
    VAR bnr = bufnr()
    hide edit Xbbb
    call assert_equal([
          \ {'lnum': 1, 'id': 5, 'col': 4, 'type_bufnr': 0, 'end': 1,
          \  'type': 'one', 'length': 2, 'start': 1},
          \ {'lnum': 4, 'id': 10, 'col': 8, 'type_bufnr': 0, 'end': 1,
          \  'type': 'two', 'length': 2, 'start': 1}],
          \ prop_list(1, {'bufnr': bnr,
          \ 'types': ['one', 'two'], 'ids': [5, 10], 'end_lnum': -1}))
    #" get text properties from an unloaded buffer
    bunload! Xaaa
    call assert_equal([], prop_list(1, {'bufnr': bnr, 'end_lnum': -1}))

    call g:DeletePropTypes()
    :%bw!
  END
  call v9.CheckLegacyAndVim9Success(lines)
endfunc

func Test_prop_find_prev_on_same_line()
  new

  call setline(1, 'the quikc bronw fox jumsp over the layz dog')
  call prop_type_add('misspell', #{highlight: 'ErrorMsg'})
  for col in [8, 14, 24, 38]
    call prop_add(1, col, #{type: 'misspell', length: 2})
  endfor

  call cursor(1, 18)
  let expected = [
    \ #{lnum: 1, id: 0, col: 14, end: 1, type: 'misspell', type_bufnr: 0, length: 2, start: 1},
    \ #{lnum: 1, id: 0, col: 24, end: 1, type: 'misspell', type_bufnr: 0, length: 2, start: 1}
    \ ]

  let result = prop_find(#{type: 'misspell'}, 'b')
  call assert_equal(expected[0], result)
  let result = prop_find(#{type: 'misspell'}, 'f')
  call assert_equal(expected[1], result)

  call prop_type_delete('misspell')
  bwipe!
endfunc

func Test_prop_spell()
  new
  set spell
  call AddPropTypes()

  call setline(1, ["helo world", "helo helo helo"])
  call prop_add(1, 1, #{type: 'one', length: 4})
  call prop_add(1, 6, #{type: 'two', length: 5})
  call prop_add(2, 1, #{type: 'three', length: 4})
  call prop_add(2, 6, #{type: 'three', length: 4})
  call prop_add(2, 11, #{type: 'three', length: 4})

  " The first prop over 'helo' increases its length after the word is corrected
  " to 'Hello', the second one is shifted to the right.
  let expected = [
      \ {'id': 0, 'col': 1, 'type_bufnr': 0, 'end': 1, 'type': 'one',
      \ 'length': 5, 'start': 1},
      \ {'id': 0, 'col': 7, 'type_bufnr': 0, 'end': 1, 'type': 'two',
      \ 'length': 5, 'start': 1}
      \ ]
  call feedkeys("z=1\<CR>", 'xt')

  call assert_equal('Hello world', getline(1))
  call assert_equal(expected, prop_list(1))

  " Repeat the replacement done by z=
  spellrepall

  let expected = [
      \ {'id': 0, 'col': 1, 'type_bufnr': 0, 'end': 1, 'type': 'three',
      \ 'length': 5, 'start': 1},
      \ {'id': 0, 'col': 7, 'type_bufnr': 0, 'end': 1, 'type': 'three',
      \ 'length': 5, 'start': 1},
      \ {'id': 0, 'col': 13, 'type_bufnr': 0, 'end': 1, 'type': 'three',
      \ 'length': 5, 'start': 1}
      \ ]
  call assert_equal('Hello Hello Hello', getline(2))
  call assert_equal(expected, prop_list(2))

  call DeletePropTypes()
  set spell&
  bwipe!
endfunc

func Test_prop_shift_block()
  new
  call AddPropTypes()

  call setline(1, ['some     highlighted text']->repeat(2))
  call prop_add(1, 10, #{type: 'one', length: 11})
  call prop_add(2, 10, #{type: 'two', length: 11})

  call cursor(1, 1)
  call feedkeys("5l\<c-v>>", 'nxt')
  call cursor(2, 1)
  call feedkeys("5l\<c-v><", 'nxt')

  let expected = [
      \ {'lnum': 1, 'id': 0, 'col': 8, 'type_bufnr': 0, 'end': 1, 'type': 'one',
      \ 'length': 11, 'start' : 1},
      \ {'lnum': 2, 'id': 0, 'col': 6, 'type_bufnr': 0, 'end': 1, 'type': 'two',
      \ 'length': 11, 'start' : 1}
      \ ]
  call assert_equal(expected, prop_list(1, #{end_lnum: 2}))

  call DeletePropTypes()
  bwipe!
endfunc

func Test_prop_insert_multiline()
  new
  call AddPropTypes()

  call setline(1, ['foobar', 'barbaz'])
  call prop_add(1, 4, #{end_lnum: 2, end_col: 4, type: 'one'})

  call feedkeys("1Goquxqux\<Esc>", 'nxt')
  call feedkeys("2GOquxqux\<Esc>", 'nxt')

  let lines =<< trim END
      foobar
      quxqux
      quxqux
      barbaz
  END
  call assert_equal(lines, getline(1, '$'))
  let expected = [
      \ {'lnum': 1, 'id': 0, 'col': 4, 'type_bufnr': 0, 'end': 0, 'type': 'one',
      \ 'length': 4 , 'start': 1},
      \ {'lnum': 2, 'id': 0, 'col': 1, 'type_bufnr': 0, 'end': 0, 'type': 'one',
      \ 'length': 7, 'start': 0},
      \ {'lnum': 3, 'id': 0, 'col': 1, 'type_bufnr': 0, 'end': 0, 'type': 'one',
      \ 'length': 7, 'start': 0},
      \ {'lnum': 4, 'id': 0, 'col': 1, 'type_bufnr': 0, 'end': 1, 'type': 'one',
      \ 'length': 3, 'start': 0}
      \ ]
  call assert_equal(expected, prop_list(1, #{end_lnum: 10}))

  call DeletePropTypes()
  bwipe!
endfunc

func Test_prop_blockwise_change()
  new
  call AddPropTypes()

  call setline(1, ['foooooo', 'bar', 'baaaaz'])
  call prop_add(1, 1, #{end_col: 3, type: 'one'})
  call prop_add(2, 1, #{end_col: 3, type: 'two'})
  call prop_add(3, 1, #{end_col: 3, type: 'three'})

  " Replace the first two columns with '123', since 'start_incl' is false the
  " prop is not extended.
  call feedkeys("gg\<c-v>2jc123\<Esc>", 'nxt')

  let lines =<< trim END
      123oooooo
      123ar
      123aaaaz
  END
  call assert_equal(lines, getline(1, '$'))
  let expected = [
      \ {'lnum': 1, 'id': 0, 'col': 4, 'type_bufnr': 0, 'end': 1, 'type': 'one',
      \ 'length': 1, 'start': 1},
      \ {'lnum': 2, 'id': 0, 'col': 4, 'type_bufnr': 0, 'end': 1, 'type': 'two',
      \ 'length': 1, 'start': 1},
      \ {'lnum': 3, 'id': 0, 'col': 4, 'type_bufnr': 0, 'end': 1 ,
      \ 'type': 'three', 'length': 1, 'start': 1}
      \ ]
  call assert_equal(expected, prop_list(1, #{end_lnum: 10}))

  call DeletePropTypes()
  bwipe!
endfunc

func Do_test_props_do_not_affect_byte_offsets(ff, increment)
  new
  let lcount = 410

  " File format affects byte-offset calculations, so make sure it is known.
  exec 'setlocal fileformat=' . a:ff

  " Fill the buffer with varying length lines. We need a suitably large number
  " to force Vim code through paths where previous error have occurred. This
  " is more 'art' than 'science'.
  let text = 'a'
  call setline(1, text)
  let offsets = [1]
  for idx in range(lcount)
      call add(offsets, offsets[idx] + len(text) + a:increment)
      if (idx % 6) == 0
          let text = text . 'a'
      endif
      call append(line('$'), text)
  endfor

  " Set a property that spans a few lines to cause Vim's internal buffer code
  " to perform a reasonable amount of rearrangement.
  call prop_type_add('one', {'highlight': 'ErrorMsg'})
  call prop_add(1, 1, {'type': 'one', 'end_lnum': 6, 'end_col': 2})

  for idx in range(lcount)
      let boff = line2byte(idx + 1)
      call assert_equal(offsets[idx], boff, 'Bad byte offset at line ' . (idx + 1))
  endfor

  call prop_type_delete('one')
  bwipe!
endfunc

func Test_props_do_not_affect_byte_offsets()
  call Do_test_props_do_not_affect_byte_offsets('unix', 1)
endfunc

func Test_props_do_not_affect_byte_offsets_dos()
  call Do_test_props_do_not_affect_byte_offsets('dos', 2)
endfunc

func Test_props_do_not_affect_byte_offsets_editline()
  new
  let lcount = 410

  " File format affects byte-offset calculations, so make sure it is known.
  setlocal fileformat=unix

  " Fill the buffer with varying length lines. We need a suitably large number
  " to force Vim code through paths where previous error have occurred. This
  " is more 'art' than 'science'.
  let text = 'aa'
  call setline(1, text)
  let offsets = [1]
  for idx in range(lcount)
      call add(offsets, offsets[idx] + len(text) + 1)
      if (idx % 6) == 0
          let text = text . 'a'
      endif
      call append(line('$'), text)
  endfor

  " Set a property that just covers the first line. When this test was
  " developed, this did not trigger a byte-offset error.
  call prop_type_add('one', {'highlight': 'ErrorMsg'})
  call prop_add(1, 1, {'type': 'one', 'end_lnum': 1, 'end_col': 3})

  for idx in range(lcount)
      let boff = line2byte(idx + 1)
      call assert_equal(offsets[idx], boff,
          \ 'Confounding bad byte offset at line ' . (idx + 1))
  endfor

  " Insert text in the middle of the first line, keeping the property
  " unchanged.
  :1
  normal aHello
  for idx in range(1, lcount)
      let offsets[idx] = offsets[idx] + 5
  endfor

  for idx in range(lcount)
      let boff = line2byte(idx + 1)
      call assert_equal(offsets[idx], boff,
          \ 'Bad byte offset at line ' . (idx + 1))
  endfor

  call prop_type_delete('one')
  bwipe!
endfunc

func Test_prop_inserts_text()
  CheckRunVimInTerminal

  " Just a basic check for now
  let lines =<< trim END
      call setline(1, 'insert some text here and other text there and some more text after wrapping')
      call prop_type_add('someprop', #{highlight: 'ErrorMsg'})
      call prop_type_add('otherprop', #{highlight: 'Search'})
      call prop_type_add('moreprop', #{highlight: 'DiffAdd'})
      call prop_add(1, 18, #{type: 'someprop', text: 'SOME '})
      call prop_add(1, 38, #{type: 'otherprop', text: "OTHER\t"})
      call prop_add(1, 69, #{type: 'moreprop', text: 'MORE '})
      normal $

      call setline(2, 'prepost')
      call prop_type_add('multibyte', #{highlight: 'Visual'})
      call prop_add(2, 4, #{type: 'multibyte', text: 'söme和平téxt'})

      call setline(3, 'Foo foo = { 1, 2 };')
      call prop_type_add('testprop', #{highlight: 'Comment'})
      call prop_add(3, 13, #{type: 'testprop', text: '.x='})
      call prop_add(3, 16, #{type: 'testprop', text: '.y='})

      call setline(4, '')
      call prop_add(4, 1, #{type: 'someprop', text: 'empty line'})

      call setline(5, 'look highlight')
      call prop_type_add('nohi', #{})
      call prop_add(5, 6, #{type: 'nohi', text: 'no '})
  END
  call writefile(lines, 'XscriptPropsWithText', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithText', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_1', {})

  call term_sendkeys(buf, ":set signcolumn=yes\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_2', {})

  call term_sendkeys(buf, "2G$")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_3', {})

  call term_sendkeys(buf, "3Gf1")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_4', {})
  call term_sendkeys(buf, "f2")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_5', {})

  call term_sendkeys(buf, "4G")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_6', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_inserts_text_highlight()
  CheckRunVimInTerminal

  " Just a basic check for now
  let lines =<< trim END
      call setline(1, 'insert some text (here) and there')
      call prop_type_add('someprop', #{highlight: 'ErrorMsg'})
      let bef_prop = prop_add(1, 18, #{type: 'someprop', text: 'BEFORE'})
      set hlsearch
      let thematch = matchaddpos("DiffAdd", [[1, 18]])
      func DoAfter()
        call prop_remove(#{id: g:bef_prop})
        call prop_add(1, 19, #{type: 'someprop', text: 'AFTER'})
        let g:thematch = matchaddpos("DiffAdd", [[1, 18]])
        let @/ = ''
      endfunc
  END
  call writefile(lines, 'XscriptPropsWithHighlight', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithHighlight', #{rows: 6, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_hi_1', {})
  call term_sendkeys(buf, "/text (he\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_hi_2', {})
  call term_sendkeys(buf, ":call matchdelete(thematch)\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_hi_3', {})

  call term_sendkeys(buf, ":call DoAfter()\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_hi_4', {})
  call term_sendkeys(buf, "/text (he\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_hi_5', {})
  call term_sendkeys(buf, ":call matchdelete(thematch)\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_hi_6', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_inserts_text_normal_gM()
  CheckRunVimInTerminal

  let lines =<< trim END
    call setline(1, '123456789')
    call prop_type_add('theprop', #{highlight: 'Special'})
    call prop_add(1, 3, {'type': 'theprop', 'text': 'bbb'})
    call prop_add(1, 8, {'type': 'theprop', 'text': 'bbb'})
  END
  call writefile(lines, 'XscriptPropsNormal_gM', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsNormal_gM', #{rows: 3, cols: 60})
  call term_sendkeys(buf, "gM")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_normal_gM', {})

  call StopVimInTerminal(buf)
endfunc

func Run_test_prop_inserts_text_normal_gj_gk(cmd)
  CheckRunVimInTerminal

  let lines =<< trim END
    call setline(1, repeat([repeat('a', 55)], 2))
    call prop_type_add('theprop', {})
    call prop_add(1, 41, {'type': 'theprop', 'text': repeat('b', 10)})
    call prop_add(2, 41, {'type': 'theprop', 'text': repeat('b', 10)})
  END
  let lines = insert(lines, a:cmd)
  call writefile(lines, 'XscriptPropsNormal_gj_gk', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsNormal_gj_gk', #{rows: 6, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_normal_gj_gk_1', {})
  call term_sendkeys(buf, "gj")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_normal_gj_gk_2', {})
  call term_sendkeys(buf, "gj")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_normal_gj_gk_3', {})
  call term_sendkeys(buf, "gj")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_normal_gj_gk_4', {})
  call term_sendkeys(buf, "gk")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_normal_gj_gk_5', {})
  call term_sendkeys(buf, "gk")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_normal_gj_gk_6', {})
  call term_sendkeys(buf, "gk")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_normal_gj_gk_7', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_inserts_text_normal_gj_gk()
  call Run_test_prop_inserts_text_normal_gj_gk('')
  call Run_test_prop_inserts_text_normal_gj_gk('set virtualedit=all')
endfunc

func Test_prop_inserts_text_visual_block()
  CheckRunVimInTerminal

  let lines =<< trim END
    call setline(1, repeat(['123456789'], 4))
    call prop_type_add('theprop', #{highlight: 'Special'})
    call prop_add(2, 2, {'type': 'theprop', 'text': '-口-'})
    call prop_add(3, 3, {'type': 'theprop', 'text': '口'})
  END
  call writefile(lines, 'XscriptPropsVisualBlock', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsVisualBlock', #{rows: 6, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_visual_block_1', {})
  call term_sendkeys(buf, "\<C-V>3jl")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_visual_block_2', {})
  call term_sendkeys(buf, "l")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_visual_block_3', {})
  call term_sendkeys(buf, "4l")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_visual_block_4', {})
  call term_sendkeys(buf, "Ol")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_visual_block_5', {})
  call term_sendkeys(buf, "l")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_visual_block_6', {})
  call term_sendkeys(buf, "l")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_visual_block_7', {})

  call StopVimInTerminal(buf)
endfunc

func Run_test_prop_inserts_text_showbreak(cmd)
  CheckRunVimInTerminal

  let lines =<< trim END
    highlight! link LineNr Normal
    setlocal number showbreak=+ breakindent breakindentopt=shift:2
    setlocal scrolloff=0 smoothscroll
    call setline(1, repeat('a', 28))
    call prop_type_add('theprop', #{highlight: 'Special'})
    call prop_add(1, 28, #{type: 'theprop', text: repeat('123', 23)})
    normal! $
  END
  let lines = insert(lines, a:cmd)
  call writefile(lines, 'XscriptPropsShowbreak', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsShowbreak', #{rows: 6, cols: 30})
  call term_sendkeys(buf, ":set noruler\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_1', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_2', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_3', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_4', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_5', {})
  call term_sendkeys(buf, "zbi")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_6', {})
  call term_sendkeys(buf, "\<BS>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_7', {})
  call term_sendkeys(buf, "\<Esc>l")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_8', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_9', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_10', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_11', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_12', {})
  call term_sendkeys(buf, "023x$")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_13', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_14', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_15', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_16', {})
  call term_sendkeys(buf, "zbi")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_17', {})
  call term_sendkeys(buf, "\<C-U>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_18', {})
  call term_sendkeys(buf, "\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_19', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_20', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_21', {})
  call term_sendkeys(buf, "zbx")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_22', {})
  call term_sendkeys(buf, "26ia\<Esc>a")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_23', {})
  call term_sendkeys(buf, "\<C-\>\<C-O>:setlocal breakindentopt=\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_showbreak_24', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_inserts_text_showbreak()
  call Run_test_prop_inserts_text_showbreak('')
  " because of 'breakindent' the screendumps are the same
  call Run_test_prop_inserts_text_showbreak('set cpoptions+=n')
endfunc

func Test_prop_before_tab_skipcol()
  CheckRunVimInTerminal

  let lines =<< trim END
    setlocal list listchars=tab:<-> scrolloff=0 smoothscroll
    call setline(1, repeat("\t", 4) .. 'a')
    call prop_type_add('theprop', #{highlight: 'Special'})
    call prop_add(1, 4, #{type: 'theprop', text: repeat('12', 32)})
    normal! $
  END
  call writefile(lines, 'XscriptPropsBeforeTabSkipcol', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsBeforeTabSkipcol', #{rows: 6, cols: 30})
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_1', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_2', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_3', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_4', {})
  call term_sendkeys(buf, "zbh")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_5', {})
  call term_sendkeys(buf, "i")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_6', {})
  call term_sendkeys(buf, "\<C-O>:setlocal nolist\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_7', {})
  call term_sendkeys(buf, "\<Esc>l")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_8', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_9', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_10', {})
  call term_sendkeys(buf, "\<C-E>")
  call VerifyScreenDump(buf, 'Test_prop_before_tab_skipcol_11', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_inserts_text_before_linebreak()
  CheckRunVimInTerminal

  let lines =<< trim END
    setlocal linebreak showbreak=+ breakindent breakindentopt=shift:2
    call setline(1, repeat('a', 50) .. ' ' .. repeat('c', 45))
    call prop_type_add('theprop', #{highlight: 'Special'})
    call prop_add(1, 51, #{type: 'theprop', text: repeat('b', 10)})
    normal! $
  END
  call writefile(lines, 'XscriptPropsBeforeLinebreak', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsBeforeLinebreak', #{rows: 6, cols: 50})
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_before_linebreak_1', {})
  call term_sendkeys(buf, '05x$')
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_before_linebreak_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_inserts_text_before_double_width_wrap()
  CheckRunVimInTerminal

  let lines =<< trim END
    call setline(1, repeat('a', 40) .. '口' .. '12345')
    call prop_type_add('theprop', #{highlight: 'Special'})
    call prop_add(1, 41, #{type: 'theprop', text: repeat('b', 9)})
    normal! $
  END
  call writefile(lines, 'XscriptPropsBeforeDoubleWidthWrap', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsBeforeDoubleWidthWrap', #{rows: 3, cols: 50})
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_before_double_width_wrap_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_inserts_text_lcs_extends()
  CheckRunVimInTerminal

  let lines =<< trim END
    setlocal nowrap list listchars=extends:!
    call setline(1, repeat('a', &columns + 1))
    call prop_type_add('theprop', #{highlight: 'Special'})
    call prop_add(1, &columns + 2, #{type: 'theprop', text: 'bbb'})
  END
  call writefile(lines, 'XscriptPropsListExtends', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsListExtends', #{rows: 3, cols: 50})
  call term_sendkeys(buf, '20l')
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_lcs_extends_1', {})
  call term_sendkeys(buf, 'zl')
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_lcs_extends_2', {})
  call term_sendkeys(buf, 'zl')
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_lcs_extends_3', {})
  call term_sendkeys(buf, 'zl')
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_lcs_extends_4', {})
  call term_sendkeys(buf, 'zl')
  call VerifyScreenDump(buf, 'Test_prop_inserts_text_lcs_extends_5', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_add_with_text_fails()
  call prop_type_add('failing', #{highlight: 'ErrorMsg'})
  call assert_fails("call prop_add(1, 0, #{type: 'failing', text: 'X', end_lnum: 1})", 'E1305:')
  call assert_fails("call prop_add(1, 0, #{type: 'failing', text: 'X', end_col: 1})", 'E1305:')
  call assert_fails("call prop_add(1, 0, #{type: 'failing', text: 'X', length: 1})", 'E1305:')

  call prop_type_delete('failing')
endfunc

func Test_props_with_text_right_align_twice()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ["some text some text some text some text", 'line two'])
      call prop_type_add('MyErrorText', #{highlight: 'ErrorMsg'})
      call prop_type_add('MyPadding', #{highlight: 'DiffChange'})
      call prop_add(1, 0, #{type: 'MyPadding', text: ' nothing here', text_wrap: 'wrap'})
      call prop_add(1, 0, #{type: 'MyErrorText', text: 'Some error', text_wrap: 'wrap', text_align: 'right'})
      call prop_add(1, 0, #{type: 'MyErrorText', text: 'Another error', text_wrap: 'wrap', text_align: 'right'})
      normal G$
  END
  call writefile(lines, 'XscriptPropsRightAlign', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsRightAlign', #{rows: 8})
  call VerifyScreenDump(buf, 'Test_prop_right_align_twice_1', {})

  call term_sendkeys(buf, "ggisome more text\<Esc>G$")
  call VerifyScreenDump(buf, 'Test_prop_right_align_twice_2', {})

  call term_sendkeys(buf, ":set signcolumn=yes\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_right_align_twice_3', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_after()
  CheckRunVimInTerminal

  let lines =<< trim END
      set showbreak=+++
      set breakindent
      call setline(1, '   some text here and other text there')
      call prop_type_add('rightprop', #{highlight: 'ErrorMsg'})
      call prop_type_add('afterprop', #{highlight: 'Search'})
      call prop_type_add('belowprop', #{highlight: 'DiffAdd'})
      call prop_add(1, 0, #{type: 'rightprop', text: ' RIGHT ', text_align: 'right'})
      call prop_add(1, 0, #{type: 'afterprop', text: "\tAFTER\t", text_align: 'after'})
      call prop_add(1, 0, #{type: 'belowprop', text: ' BELOW ', text_align: 'below'})
      call prop_add(1, 0, #{type: 'belowprop', text: ' ALSO BELOW ', text_align: 'below'})

      call setline(2, 'Last line.')
      call prop_add(2, 0, #{type: 'afterprop', text: ' After Last ', text_align: 'after'})
      normal G$

      call setline(3, 'right here')
      call prop_add(3, 0, #{type: 'rightprop', text: 'söme和平téxt', text_align: 'right'})
  END
  call writefile(lines, 'XscriptPropsWithTextAfter', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithTextAfter', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_1', {})

  call StopVimInTerminal(buf)

  call assert_fails('call prop_add(1, 2, #{text: "yes", text_align: "right", type: "some"})', 'E1294:')
endfunc

func Test_props_with_text_after_and_list()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['one', 'two'])
      prop_type_add('test', {highlight: 'Special'})
      prop_add(1, 0, {
          type: 'test',
          text: range(50)->join(' '),
          text_align: 'after',
          text_padding_left: 3
      })
      prop_add(1, 0, {
          type: 'test',
          text: range(50)->join('-'),
          text_align: 'after',
          text_padding_left: 5
      })
      prop_add(1, 0, {
          type: 'test',
          text: range(50)->join('.'),
          text_align: 'after',
          text_padding_left: 1
      })
      normal G$
  END
  call writefile(lines, 'XscriptPropsAfter', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsAfter', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_props_after_1', {})

  call term_sendkeys(buf, ":set list\<CR>")
  call VerifyScreenDump(buf, 'Test_props_after_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_after_below_trunc()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      edit foobar
      set showbreak=+++
      setline(1, ['onasdf asdf asdf asdf asd fas df', 'two'])
      prop_type_add('test', {highlight: 'Special'})
      prop_add(1, 0, {
          type: 'test',
          text: 'the quick brown fox jumps over the lazy dog',
          text_align: 'after',
      })
      prop_type_add('another', {highlight: 'DiffChange'})
      prop_add(1, 0, {
          type: 'another',
          text: 'the quick brown fox jumps over the lazy dog',
          text_align: 'below',
          text_padding_left: 4,
      })
      normal G$
  END
  call writefile(lines, 'XscriptPropsAfterTrunc', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsAfterTrunc', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_below_trunc_1', {})

  call term_sendkeys(buf, ":set number\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_below_trunc_2', {})

  call term_sendkeys(buf, ":set cursorline\<CR>gg")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_below_trunc_3', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_text_below_after_empty()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script

      setline(1, ['vim9script', '', 'three', ''])

      # Add text prop below empty line 2 with padding.
      prop_type_add('test', {highlight: 'ErrorMsg'})
      prop_add(2, 0, {
           type: 'test',
           text: 'The quick brown fox jumps over the lazy dog',
           text_align: 'below',
           text_padding_left: 1,
      })

      # Add text prop below empty line 4 without padding.
      prop_type_add('other', {highlight: 'DiffChange'})
      prop_add(4, 0, {
           type: 'other',
           text: 'The slow fox bumps into the lazy dog',
           text_align: 'below',
           text_padding_left: 0,
      })
  END
  call writefile(lines, 'XscriptPropBelowAfterEmpty', 'D')
  let buf = RunVimInTerminal('-S XscriptPropBelowAfterEmpty', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_below_after_empty_1', {})

  call term_sendkeys(buf, ":set number\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_below_after_empty_2', {})

  call term_sendkeys(buf, ":set nowrap\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_below_after_empty_3', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_text_above_below_empty()
  CheckRunVimInTerminal

  let lines =<< trim END
      setlocal number
      call setline(1, ['11111111', '', '333333333', '', '55555555555'])

      let vt = 'test'
      call prop_type_add(vt, {'highlight': 'ToDo'})
      for ln in range(1, line('$'))
        call prop_add(ln, 0, {'type': vt, 'text': '---', 'text_align': 'above'})
        call prop_add(ln, 0, {'type': vt, 'text': '+++', 'text_align': 'below'})
      endfor
      normal G
  END
  call writefile(lines, 'XscriptPropAboveBelowEmpty', 'D')
  let buf = RunVimInTerminal('-S XscriptPropAboveBelowEmpty', #{rows: 16, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_above_below_empty_1', {})

  call term_sendkeys(buf, ":set list\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_above_below_empty_2', {})

  call term_sendkeys(buf, ":set nolist\<CR>")
  call term_sendkeys(buf, ":set colorcolumn=10\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_above_below_empty_3', {})

  call term_sendkeys(buf, ":set colorcolumn=\<CR>")
  call term_sendkeys(buf, ":set relativenumber\<CR>")
  call term_sendkeys(buf, ":\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_above_below_empty_4', {})

  call term_sendkeys(buf, "kk")
  call VerifyScreenDump(buf, 'Test_prop_above_below_empty_5', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_multibyte_below()
  CheckRunVimInTerminal

  let lines =<< trim END
      setlocal number
      call setline(1, ['©', '©', '©'])

      let vt = 'test'
      call prop_type_add(vt, {'highlight': 'ToDo'})
      for ln in range(1, line('$'))
        call prop_add(ln, 0, {'type': vt, 'text': '+++', 'text_align': 'below'})
      endfor
      normal G
  END
  call writefile(lines, 'XscriptPropMultibyteBelow', 'D')
  let buf = RunVimInTerminal('-S XscriptPropMultibyteBelow', #{rows: 10, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_multibyte_below_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_text_below_rightleft()
  CheckRunVimInTerminal
  CheckFeature rightleft

  let lines =<< trim END
    setlocal number rightleft
    call setline(1, 'abcde')
    call prop_type_add('theprop', #{highlight: 'Special'})
    call prop_add(1, 0, #{type: 'theprop', text: '12345', text_align: 'below'})
  END
  call writefile(lines, 'XscriptPropBelowRightleft', 'D')
  let buf = RunVimInTerminal('-S XscriptPropBelowRightleft', #{rows: 6, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_below_rightleft_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_text_above_empty()
  CheckRunVimInTerminal

  " check the cursor is in the correct line
  let lines =<< trim END
      setlocal number
      call setline(1, ['11111111', '', '333333333', '', '55555555555'])

      let vt = 'test'
      call prop_type_add(vt, {'highlight': 'ToDo'})
      for ln in range(1, line('$'))
        call prop_add(ln, 0, {'type': vt, 'text': '---', 'text_align': 'above'})
      endfor
      normal G
  END
  call writefile(lines, 'XscriptPropAboveEmpty', 'D')
  let buf = RunVimInTerminal('-S XscriptPropAboveEmpty', #{rows: 16, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_above_empty_1', {})

  call term_sendkeys(buf, ":set list\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_above_empty_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_text_below_after_match()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script

      setline(1, ['vim9script', 'some text'])
      set signcolumn=yes
      matchaddpos('Search', [[1, 10]])
      prop_type_add('test', {highlight: 'Error'})
      prop_add(1, 0, {
          type: 'test',
          text: 'The quick brown fox',
          text_align: 'below'
      })
  END
  call writefile(lines, 'XscriptPropsBelow', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsBelow', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_below_after_match_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_after_joined()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ['one', 'two', 'three', 'four'])
      call prop_type_add('afterprop', #{highlight: 'Search'})
      call prop_add(1, 0, #{type: 'afterprop', text: ' ONE', text_align: 'after'})
      call prop_add(4, 0, #{type: 'afterprop', text: ' FOUR', text_align: 'after'})
      normal ggJ
      normal GkJ

      call setline(3, ['a', 'b', 'c', 'd', 'e', 'f'])
      call prop_add(3, 0, #{type: 'afterprop', text: ' AAA', text_align: 'after'})
      call prop_add(5, 0, #{type: 'afterprop', text: ' CCC', text_align: 'after'})
      call prop_add(7, 0, #{type: 'afterprop', text: ' EEE', text_align: 'after'})
      call prop_add(8, 0, #{type: 'afterprop', text: ' FFF', text_align: 'after'})
      normal 3G6J
  END
  call writefile(lines, 'XscriptPropsWithTextAfterJoined', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithTextAfterJoined', #{rows: 6, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_joined_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_after_truncated()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ['one two three four five six seven'])
      call prop_type_add('afterprop', #{highlight: 'Search'})
      call prop_add(1, 0, #{type: 'afterprop', text: ' ONE and TWO and THREE and FOUR and FIVE'})

      call setline(2, ['one two three four five six seven'])
      call prop_add(2, 0, #{type: 'afterprop', text: ' one AND two AND three AND four AND five', text_align: 'right'})

      call setline(3, ['one two three four five six seven'])
      call prop_add(3, 0, #{type: 'afterprop', text: ' one AND two AND three AND four AND five lets wrap after some more text', text_align: 'below'})

      call setline(4, ['cursor here'])
      normal 4Gfh
  END
  call writefile(lines, 'XscriptPropsWithTextAfterTrunc', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithTextAfterTrunc', #{rows: 9, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_trunc_1', {})

  call term_sendkeys(buf, ":37vsp\<CR>gg")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_trunc_2', {})

  call term_sendkeys(buf, ":36wincmd |\<CR>")
  call term_sendkeys(buf, "2G$")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_trunc_3', {})

  call term_sendkeys(buf, ":33wincmd |\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_trunc_4', {})

  call term_sendkeys(buf, ":18wincmd |\<CR>")
  call term_sendkeys(buf, "0fx")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_trunc_5', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_after_truncated_and_ambiwidth_is_double()
  CheckRunVimInTerminal

  let lines =<< trim END
      set ambiwidth=double
      call setline(1, ['one two three four five six seven'])
      call prop_type_add('afterprop', #{highlight: 'Search'})
      call prop_add(1, 0, #{type: 'afterprop', text: ' ONE and TWO and THREE and FOUR and FIVE'})

      call setline(2, ['one two three four five six seven'])
      call prop_add(2, 0, #{type: 'afterprop', text: ' one AND two AND three AND four AND five', text_align: 'right'})

      call setline(3, ['one two three four five six seven'])
      call prop_add(3, 0, #{type: 'afterprop', text: ' one AND two AND three AND four AND five lets wrap after some more text', text_align: 'below'})

      call setline(4, ['cursor here'])
      normal 4Gfh
  END
  call writefile(lines, 'XscriptPropsWithTextAfterTrunc-and-ambiwidth-is-double', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithTextAfterTrunc-and-ambiwidth-is-double', #{rows: 9, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_trunc_ambiw_d_1', {})

  call StopVimInTerminal(buf)
endfunc


func Test_props_with_text_after_truncated_not_utf8()
  CheckRunVimInTerminal

  let lines =<< trim END
      set enc=cp932 tenc=utf-8
      call setline(1, ['one two three four five six seven'])
      call prop_type_add('afterprop', #{highlight: 'Search'})
      call prop_add(1, 0, #{type: 'afterprop', text: ' ONE and TWO and THREE and FOUR and FIVE'})

      call setline(2, ['one two three four five six seven'])
      call prop_add(2, 0, #{type: 'afterprop', text: ' one AND two AND three AND four AND five', text_align: 'right'})

      call setline(3, ['one two three four five six seven'])
      call prop_add(3, 0, #{type: 'afterprop', text: ' one AND two AND three AND four AND five lets wrap after some more text', text_align: 'below'})

      call setline(4, ['cursor here'])
      normal 4Gfh
  END
  call writefile(lines, 'XscriptPropsWithTextAfterTrunc-enc-is-not-utf8', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithTextAfterTrunc-enc-is-not-utf8', #{rows: 9, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_trunc_not_utf8', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_empty_line()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ['', 'aaa', '', 'bbbbbb'])
      call prop_type_add('prop1', #{highlight: 'Search'})
      call prop_add(1, 1, #{type: 'prop1', text: repeat('X', &columns)})
      call prop_add(3, 1, #{type: 'prop1', text: repeat('X', &columns + 1)})
      normal gg0
  END
  call writefile(lines, 'XscriptPropsWithTextEmptyLine', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithTextEmptyLine', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_empty_line_1', {})
  call term_sendkeys(buf, "$")
  call VerifyScreenDump(buf, 'Test_prop_with_text_empty_line_2', {})
  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_prop_with_text_empty_line_3', {})
  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_prop_with_text_empty_line_4', {})
  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_prop_with_text_empty_line_5', {})
  call term_sendkeys(buf, "0\<C-V>2l2k")
  call VerifyScreenDump(buf, 'Test_prop_with_text_empty_line_6', {})
  call term_sendkeys(buf, "\<Esc>/aaa\\n\\%V\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_empty_line_7', {})
  call term_sendkeys(buf, "3ggic")
  call VerifyScreenDump(buf, 'Test_prop_with_text_empty_line_8', {})
  call term_sendkeys(buf, "\<Esc>/aaa\\nc\\%V\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_empty_line_9', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_after_wraps()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ['one two three four five six seven'])
      call prop_type_add('afterprop', #{highlight: 'Search'})
      call prop_add(1, 0, #{type: 'afterprop', text: ' ONE and TWO and THREE and FOUR and FIVE', text_wrap: 'wrap'})

      call setline(2, ['one two three four five six seven'])
      call prop_add(2, 0, #{type: 'afterprop', text: ' one AND two AND three AND four AND five', text_align: 'right', text_wrap: 'wrap'})

      call setline(3, ['one two three four five six seven'])
      call prop_add(3, 0, #{type: 'afterprop', text: ' one AND two AND three AND four AND five lets wrap after some more text', text_align: 'below', text_wrap: 'wrap'})

      call setline(4, ['cursor here'])
      normal 4Gfh
  END
  call writefile(lines, 'XscriptPropsWithTextAfterWraps', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithTextAfterWraps', #{rows: 9, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_wraps_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_after_nowrap()
  CheckRunVimInTerminal

  let lines =<< trim END
      set nowrap
      call setline(1, ['one', 'two', 'three', 'four'])
      call prop_type_add('belowprop', #{highlight: 'ErrorMsg'})
      call prop_type_add('anotherprop', #{highlight: 'Search'})
      call prop_type_add('someprop', #{highlight: 'DiffChange'})
      call prop_add(1, 0, #{type: 'belowprop', text: ' Below the line ', text_align: 'below'})
      call prop_add(2, 0, #{type: 'anotherprop', text: 'another', text_align: 'below'})
      call prop_add(2, 0, #{type: 'belowprop', text: 'One More Here', text_align: 'below'})
      call prop_add(1, 0, #{type: 'someprop', text: 'right here', text_align: 'right'})
      call prop_add(1, 0, #{type: 'someprop', text: ' After the text', text_align: 'after'})
      normal 3G$

      call prop_add(3, 0, #{type: 'anotherprop', text: 'right aligned', text_align: 'right'})
      call prop_add(3, 0, #{type: 'anotherprop', text: 'also right aligned', text_align: 'right'})
      hi CursorLine ctermbg=lightgrey
  END
  call writefile(lines, 'XscriptPropsAfterNowrap', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsAfterNowrap', #{rows: 12, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_nowrap_1', {})

  call term_sendkeys(buf, ":set signcolumn=yes foldcolumn=3 cursorline\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_nowrap_2', {})

  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_nowrap_3', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_text_below_cul()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script

      setline(1, ['some text', 'last line'])
      set cursorline nowrap
      prop_type_add('test', {highlight: 'DiffChange'})
      prop_add(1, 0, {
          type: 'test',
          text: 'The quick brown fox jumps over the lazy dog',
          text_align: 'below',
          text_padding_left: 4,
      })
  END
  call writefile(lines, 'XscriptPropsBelowCurline', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsBelowCurline', #{rows: 6, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_below_cul_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_below_nowrap()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      edit foobar
      set nowrap
      set showbreak=+++\ 
      setline(1, ['onasdf asdf asdf sdf df asdf asdf e asdf asdf asdf asdf asd fas df', 'two'])
      prop_type_add('test', {highlight: 'Special'})
      prop_add(1, 0, {
          type: 'test',
          text: 'the quick brown fox jumps over the lazy dog',
          text_align: 'after'
      })
      prop_add(1, 0, {
          type: 'test',
          text: 'the quick brown fox jumps over the lazy dog',
          text_align: 'below'
      })
      normal G$
  END
  call writefile(lines, 'XscriptPropsBelowNowrap', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsBelowNowrap', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_below_nowrap_1', {})

  call term_sendkeys(buf, "gg$")
  call VerifyScreenDump(buf, 'Test_prop_with_text_below_nowrap_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_above()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ['one two', 'three four', 'five six'])
      call prop_type_add('above1', #{highlight: 'Search'})
      call prop_type_add('above2', #{highlight: 'DiffChange'})
      call prop_type_add('below', #{highlight: 'DiffAdd'})
      call prop_add(1, 0, #{type: 'above1', text: 'first thing above', text_align: 'above'})
      call prop_add(1, 0, #{type: 'above2', text: 'second thing above', text_align: 'above'})
      call prop_add(3, 0, #{type: 'above1', text: 'another thing', text_align: 'above', text_padding_left: 3})

      normal gglllj
      func AddPropBelow()
        call prop_add(1, 0, #{type: 'below', text: 'below', text_align: 'below'})
      endfunc
      func AddLongPropAbove()
        3,4delete
        set wrap
        call prop_add(1, 0, #{type: 'above1', text: range(50)->join(' '), text_align: 'above', text_padding_left: 2})
      endfunc
  END
  call writefile(lines, 'XscriptPropsWithTextAbove', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsWithTextAbove', #{rows: 9, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_1', {})

  call term_sendkeys(buf, "ggg$")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_1a', {})
  call term_sendkeys(buf, "g0")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_1b', {})

  call term_sendkeys(buf, ":set showbreak=>>\<CR>")
  call term_sendkeys(buf, "ggll")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_1c', {})
  call term_sendkeys(buf, ":set showbreak=\<CR>")

  call term_sendkeys(buf, "ggI")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_2', {})
  call term_sendkeys(buf, "inserted \<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_3', {})

  call term_sendkeys(buf, ":set number signcolumn=yes\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_4', {})

  call term_sendkeys(buf, ":set nowrap\<CR>gg$j")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_5', {})

  call term_sendkeys(buf, ":call AddPropBelow()\<CR>")
  call term_sendkeys(buf, "ggve")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_6', {})
  call term_sendkeys(buf, "V")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_7', {})

  call term_sendkeys(buf, "\<Esc>ls\<CR>\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_8', {})

  call term_sendkeys(buf, ":call AddLongPropAbove()\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_above_9', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_with_text_above_padding()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script

      setlocal tabstop=8 noexpandtab
      setline(1, ['', 'sky is blue', 'ocean is blue'])
      prop_type_add('DiagVirtualText', {highlight: 'Search', override: true})
      prop_add(3, 0, {text: "┌─ start", text_align: "above",
               type: 'DiagVirtualText',
               text_padding_left: 200})
  END
  call writefile(lines, 'XscriptAbovePadding', 'D')
  let buf = RunVimInTerminal('-S XscriptAbovePadding', #{rows: 8})
  call VerifyScreenDump(buf, 'Test_prop_above_padding_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_above_with_indent()
  new
  call setline(1, ['first line', '    second line', '    line below'])
  setlocal cindent
  call prop_type_add('indented', #{highlight: 'Search'})
  call prop_add(3, 0, #{type: 'indented', text: 'here', text_align: 'above', text_padding_left: 4})
  call assert_equal('    line below', getline(3))

  exe "normal 3G2|a\<CR>"
  call assert_equal('  ', getline(3))
  call assert_equal('    line below', getline(4))

  bwipe!
  call prop_type_delete('indented')
endfunc

func Test_prop_above_with_number()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['one one one', 'two two two', 'three three three'])
      set number cpo+=n
      prop_type_add('test', {highlight: 'DiffChange'})
      prop_add(2, 0, {
          text:  'above the text',
          type: 'test',
          text_align: 'above',
      })
      def g:OneMore()
        prop_add(2, 0, {
            text:  'also above the text',
            type: 'test',
            text_align: 'above',
        })
      enddef
  END
  call writefile(lines, 'XscriptPropAboveNr', 'D')
  let buf = RunVimInTerminal('-S XscriptPropAboveNr', #{rows: 8})
  call VerifyScreenDump(buf, 'Test_prop_above_number_1', {})

  call term_sendkeys(buf, ":call OneMore()\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_above_number_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_above_with_linebreak()
  CheckRunVimInTerminal

  let lines =<< trim END
    setlocal linebreak breakindent breakindentopt=shift:4
    call setline(1, ["a b", "c d"])
    call prop_type_add('theprop' , #{highlight: 'Special'})
    call prop_add(1, 0, #{type: 'theprop', text: '123', text_align: 'above'})
    normal! 2gg$
  END
  call writefile(lines, 'XscriptPropAboveLinebreak', 'D')
  let buf = RunVimInTerminal('-S XscriptPropAboveLinebreak', #{rows: 6})
  call VerifyScreenDump(buf, 'Test_prop_above_linebreak_1', {})
  call term_sendkeys(buf, 'k')
  call VerifyScreenDump(buf, 'Test_prop_above_linebreak_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_above_and_before()
  CheckRunVimInTerminal

  let lines =<< trim END
    setlocal linebreak breakindent breakindentopt=shift:2
    call setline(1, ["a", "  b c"])
    call prop_type_add('theprop' , #{highlight: 'Special'})
    call prop_add(2, 0, #{type: 'theprop', text: '  123', text_align: 'above'})
    call prop_add(2, 4, #{type: 'theprop', text: ': 456'} )
    normal! 2gg$
  END
  call writefile(lines, 'XscriptPropAboveAndBefore', 'D')
  let buf = RunVimInTerminal('-S XscriptPropAboveAndBefore', #{rows: 6})
  call VerifyScreenDump(buf, 'Test_prop_above_and_before_1', {})
  call term_sendkeys(buf, 'h')
  call VerifyScreenDump(buf, 'Test_prop_above_and_before_2', {})
  call term_sendkeys(buf, 'h')
  call VerifyScreenDump(buf, 'Test_prop_above_and_before_3', {})
  call term_sendkeys(buf, 'h')
  call VerifyScreenDump(buf, 'Test_prop_above_and_before_4', {})
  call term_sendkeys(buf, 'h')
  call VerifyScreenDump(buf, 'Test_prop_above_and_before_5', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_below_split_line()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['one one one', 'two two two', 'three three three'])
      prop_type_add('test', {highlight: 'Search'})
      prop_add(2, 0, {
          text:  '└─ Virtual text below the 2nd line',
          type: 'test',
          text_align: 'below',
          text_padding_left: 3
      })
  END
  call writefile(lines, 'XscriptPropBelowSpitLine', 'D')
  let buf = RunVimInTerminal('-S XscriptPropBelowSpitLine', #{rows: 8})
  call term_sendkeys(buf, "2GA\<CR>xx")
  call VerifyScreenDump(buf, 'Test_prop_below_split_line_1', {})

  call term_sendkeys(buf, "\<Esc>:set number\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_below_split_line_2', {})

  call term_sendkeys(buf, ":set nowrap\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_below_split_line_3', {})

  call StopVimInTerminal(buf)
endfunc

func Test_prop_above_below_smoothscroll()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, range(1, 10)->mapnew((_, v) => '" line ' .. v))

      set smoothscroll wrap
      call prop_type_add('mytype', {highlight: 'DiffChange'})
      call prop_add(3, 0, {text: "insert above", type: "mytype", text_align: 'above'})
      call prop_add(5, 0, {text: "insert above 1", type: "mytype", text_align: 'above'})
      call prop_add(5, 0, {text: "insert above 2", type: "mytype", text_align: 'above'})
      call prop_add(7, 0, {text: "insert below", type: "mytype", text_align: 'below'})
      call prop_add(9, 0, {text: "insert below 1", type: "mytype", text_align: 'below'})
      call prop_add(9, 0, {text: "insert below 2", type: "mytype", text_align: 'below'})
  END
  call writefile(lines, 'XscriptPropsSmoothscroll', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsSmoothscroll', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_above_below_smoothscroll_1', {})

  for nr in range(2, 16)
    call term_sendkeys(buf, "\<C-E>")
    call VerifyScreenDump(buf, 'Test_prop_above_below_smoothscroll_' .. nr, {})
  endfor

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_override()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, 'some text here')
      hi Likethis ctermfg=blue ctermbg=cyan
      prop_type_add('prop', {highlight: 'Likethis', override: true})
      prop_add(1, 6, {type: 'prop', text: ' inserted '})
      hi CursorLine cterm=underline ctermbg=lightgrey
      set cursorline
  END
  call writefile(lines, 'XscriptPropsOverride', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsOverride', #{rows: 6, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_override_1', {})

  call term_sendkeys(buf, ":set nocursorline\<CR>")
  call term_sendkeys(buf, "0llvfr")
  call VerifyScreenDump(buf, 'Test_prop_with_text_override_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_CursorMoved()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ['this is line one', 'this is line two', 'three', 'four', 'five'])

      call prop_type_add('prop', #{highlight: 'Error'})
      let g:long_text = repeat('x', &columns * 2)

      let g:prop_id = v:null
      func! Update()
        if line('.') == 1
          if g:prop_id == v:null
            let g:prop_id = prop_add(1, 0, #{type: 'prop', text_wrap: 'wrap', text: g:long_text})
          endif
        elseif g:prop_id != v:null
          call prop_remove(#{id: g:prop_id})
          let g:prop_id = v:null
        endif
      endfunc

      autocmd CursorMoved * call Update()
  END
  call writefile(lines, 'XscriptPropsCursorMovec', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsCursorMovec', #{rows: 8, cols: 60})
  call term_sendkeys(buf, "gg0w")
  call VerifyScreenDump(buf, 'Test_prop_with_text_cursormoved_1', {})

  call term_sendkeys(buf, "j")
  call VerifyScreenDump(buf, 'Test_prop_with_text_cursormoved_2', {})

  " back to the first state
  call term_sendkeys(buf, "k")
  call VerifyScreenDump(buf, 'Test_prop_with_text_cursormoved_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_props_with_text_after_split_join()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ['1122'])
      call prop_type_add('belowprop', #{highlight: 'ErrorMsg'})
      call prop_add(1, 0, #{type: 'belowprop', text: ' Below the line ', text_align: 'below'})
      exe "normal f2i\<CR>\<Esc>"

      func AddMore()
        call prop_type_add('another', #{highlight: 'Search'})
        call prop_add(1, 0, #{type: 'another', text: ' after the text ', text_align: 'after'})
        call prop_add(1, 0, #{type: 'another', text: ' right here', text_align: 'right'})
      endfunc
  END
  call writefile(lines, 'XscriptPropsAfterSplitJoin', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsAfterSplitJoin', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_join_split_1', {})

  call term_sendkeys(buf, "ggJ")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_join_split_2', {})

  call term_sendkeys(buf, ":call AddMore()\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_join_split_3', {})

  call term_sendkeys(buf, "ggf s\<CR>\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_join_split_4', {})

  call term_sendkeys(buf, "ggJ")
  call VerifyScreenDump(buf, 'Test_prop_with_text_after_join_split_5', {})

  call StopVimInTerminal(buf)
endfunc

func Test_removed_prop_with_text_cleans_up_array()
  new
  call setline(1, 'some text here')
  call prop_type_add('some', #{highlight: 'ErrorMsg'})
  let id1 = prop_add(1, 5, #{type: 'some', text: "SOME"})
  call assert_equal(-1, id1)
  let id2 = prop_add(1, 10, #{type: 'some', text: "HERE"})
  call assert_equal(-2, id2)

  " removing the props resets the index
  call prop_remove(#{id: id1})
  call prop_remove(#{id: id2})
  let id1 = prop_add(1, 5, #{type: 'some', text: "SOME"})
  call assert_equal(-1, id1)

  call prop_type_delete('some')
  bwipe!
endfunc

def Test_insert_text_before_virtual_text()
  new foobar
  setline(1, '12345678')
  prop_type_add('test', {highlight: 'Search'})
  prop_add(1, 5, {
    type: 'test',
    text: ' virtual text '
    })
  normal! f4axyz
  normal! f5iXYZ
  assert_equal('1234xyzXYZ5678', getline(1))

  prop_type_delete('test')
  bwipe!
enddef

func Test_insert_text_start_incl()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['text one text two', '', 'function(arg)'])

      prop_type_add('propincl', {highlight: 'NonText', start_incl: true})
      prop_add(1, 6, {type: 'propincl', text: 'after '})
      cursor(1, 6)
      prop_type_add('propnotincl', {highlight: 'NonText', start_incl: false})
      prop_add(1, 15, {type: 'propnotincl', text: 'before '})

      set cindent sw=4
      prop_type_add('argname', {highlight: 'DiffChange', start_incl: true})
      prop_add(3, 10, {type: 'argname', text: 'arg: '})
  END
  call writefile(lines, 'XscriptPropsStartIncl', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsStartIncl', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_insert_start_incl_1', {})

  call term_sendkeys(buf, "i")
  call VerifyScreenDump(buf, 'Test_prop_insert_start_incl_2', {})
  call term_sendkeys(buf, "xx\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_insert_start_incl_3', {})

  call term_sendkeys(buf, "2wi")
  call VerifyScreenDump(buf, 'Test_prop_insert_start_incl_4', {})
  call term_sendkeys(buf, "yy\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_insert_start_incl_5', {})

  call term_sendkeys(buf, "3Gfai\<CR>\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_insert_start_incl_6', {})
  call term_sendkeys(buf, ">>")
  call VerifyScreenDump(buf, 'Test_prop_insert_start_incl_7', {})
  call term_sendkeys(buf, "<<<<")
  call VerifyScreenDump(buf, 'Test_prop_insert_start_incl_8', {})

  call StopVimInTerminal(buf)
endfunc

func Test_insert_text_list_mode()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['This is a line with quite a bit of text here.',
                  'second line', 'third line'])
      set list listchars+=extends:»
      prop_type_add('Prop1', {highlight: 'Error'})
      prop_add(1, 0, {
          type: 'Prop1',
          text: 'The quick brown fox jumps over the lazy dog',
          text_align: 'right'
      })
  END
  call writefile(lines, 'XscriptPropsListMode', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsListMode', #{rows: 8, cols: 60})
  call term_sendkeys(buf, "ggj")
  call VerifyScreenDump(buf, 'Test_prop_insert_list_mode_1', {})

  call term_sendkeys(buf, ":set nowrap\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_insert_list_mode_2', {})

  call term_sendkeys(buf, "ggd32l")
  call VerifyScreenDump(buf, 'Test_prop_insert_list_mode_3', {})

  call StopVimInTerminal(buf)
endfunc

func Test_insert_text_with_padding()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['Some text to add virtual text to.',
                  'second line',
                  'Another line with some text to make the wrap.'])
      prop_type_add('theprop', {highlight: 'DiffChange'})
      prop_add(1, 0, {
          type: 'theprop',
          text: 'after',
          text_align: 'after',
          text_padding_left: 3,
      })
      prop_add(1, 0, {
          type: 'theprop',
          text: 'right aligned',
          text_align: 'right',
          text_padding_left: 5,
      })
      prop_add(1, 0, {
          type: 'theprop',
          text: 'below the line',
          text_align: 'below',
          text_padding_left: 4,
      })
      prop_add(3, 0, {
          type: 'theprop',
          text: 'rightmost',
          text_align: 'right',
          text_padding_left: 6,
          text_wrap: 'wrap',
      })
  END
  call writefile(lines, 'XscriptPropsPadded', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsPadded', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_text_with_padding_1', {})

  call term_sendkeys(buf, "ggixxxxxxxxxx\<Esc>")
  call term_sendkeys(buf, "3Gix\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_text_with_padding_2', {})

  call term_sendkeys(buf, "ggix\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_text_with_padding_3', {})

  call term_sendkeys(buf, ":set list\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_text_with_padding_4', {})

  call StopVimInTerminal(buf)
endfunc

func Test_long_text_below_with_padding()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['first line', 'second line'])
      prop_type_add('theprop', {highlight: 'DiffChange'})
      prop_add(1, 0, {
          type: 'theprop',
          text: 'after '->repeat(20),
          text_align: 'below',
          text_padding_left: 3,
      })
      prop_add(1, 0, {
          type: 'theprop',
          text: 'more '->repeat(20),
          text_align: 'below',
          text_padding_left: 30,
      })
      normal 2Gw
  END
  call writefile(lines, 'XlongTextBelowWithPadding', 'D')
  let buf = RunVimInTerminal('-S XlongTextBelowWithPadding', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_long_text_with_padding_1', {})

  call term_sendkeys(buf, ":set list\<CR>")
  call VerifyScreenDump(buf, 'Test_long_text_with_padding_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_text_after_nowrap()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['first line', range(80)->join(' '), 'third', 'fourth'])
      set nowrap
      prop_type_add('theprop', {highlight: 'DiffChange'})
      prop_add(1, 0, {
          type: 'theprop',
          text: 'right after the text '->repeat(3),
          text_align: 'after',
          text_padding_left: 2,
      })
      prop_add(1, 0, {
          type: 'theprop',
          text: 'in the middle '->repeat(4),
          text_align: 'after',
          text_padding_left: 3,
      })
      prop_add(1, 0, {
          type: 'theprop',
          text: 'the last one '->repeat(3),
          text_align: 'after',
          text_padding_left: 1,
      })
      normal 2Gw
      def g:ChangeText()
        prop_clear(1)
        set list
        prop_add(1, 0, {
            type: 'theprop',
            text: 'just after txt '->repeat(3),
            text_align: 'after',
            text_padding_left: 2,
        })
        prop_add(1, 0, {
            type: 'theprop',
            text: 'in the middle '->repeat(4),
            text_align: 'after',
            text_padding_left: 1,
        })
      enddef
  END
  call writefile(lines, 'XTextAfterNowrap', 'D')
  let buf = RunVimInTerminal('-S XTextAfterNowrap', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_text_after_nowrap_1', {})

  call term_sendkeys(buf, "30w")
  call VerifyScreenDump(buf, 'Test_text_after_nowrap_2', {})

  call term_sendkeys(buf, "22w")
  call VerifyScreenDump(buf, 'Test_text_after_nowrap_3', {})

  call term_sendkeys(buf, "$")
  call VerifyScreenDump(buf, 'Test_text_after_nowrap_4', {})

  call term_sendkeys(buf, "0")
  call term_sendkeys(buf, ":call ChangeText()\<CR>")
  call VerifyScreenDump(buf, 'Test_text_after_nowrap_5', {})

  call StopVimInTerminal(buf)
endfunc

func Test_text_after_nowrap_list()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script

      set nowrap
      set listchars+=extends:>
      set list
      setline(1, ['some text here', '', 'last line'])

      prop_type_add('test', {highlight: 'DiffChange'})
      prop_add(1, 0, {
          type: 'test',
          text: 'The quick brown fox jumps.',
          text_padding_left: 2,
      })
      prop_add(1, 0, {
          type: 'test',
          text: '■ The fox jumps over the lazy dog.',
          text_padding_left: 2,
      })
      prop_add(1, 0, {
          type: 'test',
          text: '■ The lazy dog.',
          text_padding_left: 2,
      })
      normal 3G$
  END
  call writefile(lines, 'XTextAfterNowrapList', 'D')
  let buf = RunVimInTerminal('-S XTextAfterNowrapList', #{rows: 6, cols: 60})
  call VerifyScreenDump(buf, 'Test_text_after_nowrap_list_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_text_below_nowrap()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['first line', 'second line '->repeat(50), 'third', 'fourth'])
      set nowrap number
      prop_type_add('theprop', {highlight: 'DiffChange'})
      prop_add(1, 0, {
          type: 'theprop',
          text: 'one below the text '->repeat(5),
          text_align: 'below',
          text_padding_left: 2,
      })
      prop_add(1, 0, {
          type: 'theprop',
          text: 'two below the text '->repeat(5),
          text_align: 'below',
          text_padding_left: 2,
      })
      normal 2Gw
  END
  call writefile(lines, 'XTextBelowNowrap', 'D')
  let buf = RunVimInTerminal('-S XTextBelowNowrap', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_text_below_nowrap_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_virtual_text_in_popup_highlight()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script

      # foreground highlight only, popup background is used
      prop_type_add('Prop1', {'highlight': 'SpecialKey'})
      # foreground and background highlight, popup background is not used
      prop_type_add('Prop2', {'highlight': 'DiffDelete'})

      var popupText = [{
        text: 'Some text',
        props: [
                    {
                      col: 1,
                      type: 'Prop1',
                      text: ' + '
                    },
                    {
                      col: 6,
                      type: 'Prop2',
                      text: ' x '
                    },
                ]
          }]
      var popupArgs = {
            line: 3,
            col: 20,
            maxwidth: 80,
            highlight: 'PMenu',
            border: [],
            borderchars: [' '],
          }

      popup_create(popupText, popupArgs)
  END
  call writefile(lines, 'XscriptVirtualHighlight', 'D')
  let buf = RunVimInTerminal('-S XscriptVirtualHighlight', #{rows: 8})
  call VerifyScreenDump(buf, 'Test_virtual_text_in_popup_highlight_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_insert_text_change_arg()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['SetErrorCode( 10, 20 )', 'second line'])
      prop_type_add('param', {highlight: 'DiffChange', start_incl: 1})
      prop_type_add('padd', {highlight: 'NonText', start_incl: 1})
      prop_add(1, 15, {
          type: 'param',
          text: 'id:',
      })
      prop_add(1, 15, {
          type: 'padd',
          text: '-',
      })
      prop_add(1, 19, {
          type: 'param',
          text: 'id:',
      })
      prop_add(1, 19, {
          type: 'padd',
          text: '-',
      })
  END
  call writefile(lines, 'XscriptPropsChange', 'D')
  let buf = RunVimInTerminal('-S XscriptPropsChange', #{rows: 5, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_text_change_arg_1', {})

  call term_sendkeys(buf, "ggf1cw1234\<Esc>")
  call VerifyScreenDump(buf, 'Test_prop_text_change_arg_2', {})

  call StopVimInTerminal(buf)
endfunc

def Test_textprop_in_quickfix_window()
  enew!
  var prop_type = 'my_prop'
  prop_type_add(prop_type, {})

  for lnum in range(1, 10)
    setline(lnum, 'hello world')
  endfor

  cgetbuffer
  copen

  var bufnr = bufnr()
  for lnum in range(1, line('$', bufnr->bufwinid()))
    prop_add(lnum, 1, {
      id: 1000 + lnum,
      type: prop_type,
      bufnr: bufnr,
    })
  endfor

  prop_type_delete(prop_type)
  cclose
  bwipe!
enddef

func Test_text_prop_delete_updates()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script

      setline(1, ['some text', 'more text', 'the end'])
      prop_type_add('test', {highlight: 'DiffChange'})
      prop_add(1, 0, {
          type: 'test',
          text: 'The quick brown fox jumps over the lazy dog',
          text_align: 'below',
          text_padding_left: 3,
      })
      prop_add(1, 0, {
          type: 'test',
          text: 'The quick brown fox jumps over the lazy dog',
          text_align: 'below',
          text_padding_left: 5,
      })

      normal! G
  END
  call writefile(lines, 'XtextPropDelete', 'D')
  let buf = RunVimInTerminal('-S XtextPropDelete', #{rows: 10, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_delete_updates_1', {})

  " Check that after deleting the text prop type the text properties using
  " this type no longer show and are not counted for cursor positioning.
  call term_sendkeys(buf, ":call prop_type_delete('test')\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_delete_updates_2', {})

  call term_sendkeys(buf, "ggj")
  call VerifyScreenDump(buf, 'Test_prop_delete_updates_3', {})

  call StopVimInTerminal(buf)
endfunc

func Test_text_prop_diff_mode()
  CheckRunVimInTerminal

  let lines =<< trim END
      call setline(1, ['9000', '0009', '0009', '9000', '0009'])

      let type = 'test'
      call prop_type_add(type, {})
      let text = '<text>'
      call prop_add(1, 1, {'type': type, 'text': text})
      call prop_add(2, 0, {'type': type, 'text': text, 'text_align': 'after'})
      call prop_add(3, 0, {'type': type, 'text': text, 'text_align': 'right'})
      call prop_add(4, 0, {'type': type, 'text': text, 'text_align': 'above'})
      call prop_add(5, 0, {'type': type, 'text': text, 'text_align': 'below'})
      set diff

      vnew
      call setline(1, ['000', '000', '000', '000', '000'])
      set diff
  END
  call writefile(lines, 'XtextPropDiff', 'D')
  let buf = RunVimInTerminal('-S XtextPropDiff', #{rows: 10, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_diff_mode_1', {})

  call term_sendkeys(buf, ":windo set number\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_diff_mode_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_error_when_using_negative_id()
  call prop_type_add('test1', #{highlight: 'ErrorMsg'})
  call prop_add(1, 1, #{type: 'test1', text: 'virtual'})
  call assert_fails("call prop_add(1, 1, #{type: 'test1', length: 1, id: -1})", 'E1293:')

  call prop_type_delete('test1')
endfunc

func Test_error_after_using_negative_id()
  " This needs to run a separate Vim instance because the
  " "did_use_negative_pop_id" will be set.
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script

      setline(1, ['one', 'two', 'three'])
      prop_type_add('test_1', {highlight: 'Error'})
      prop_type_add('test_2', {highlight: 'WildMenu'})

      prop_add(3, 1, {
          type: 'test_1',
          length: 5,
          id: -1
      })

      def g:AddTextprop()
          prop_add(1, 0, {
              type: 'test_2',
              text: 'The quick fox',
              text_padding_left: 2
          })
      enddef
  END
  call writefile(lines, 'XtextPropError', 'D')
  let buf = RunVimInTerminal('-S XtextPropError', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_negative_error_1', {})

  call term_sendkeys(buf, ":call AddTextprop()\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_negative_error_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_modify_text_before_prop()
  CheckRunVimInTerminal

  let lines =<< trim END
      vim9script
      setline(1, ['test_words', 'second line', 'third line', 'fourth line'])
      set number
      prop_type_add('text', {highlight: 'DiffChange'})
      prop_type_add('below', {highlight: 'NonText'})
      prop_add(1, 11, {type: 'text', text: repeat('a', 65)})
      prop_add(1, 0, {type: 'below', text: repeat('a', 65), text_align: 'below'})
  END
  call writefile(lines, 'XtextPropModifyBefore', 'D')
  let buf = RunVimInTerminal('-S XtextPropModifyBefore', #{rows: 5, cols: 60})
  call VerifyScreenDump(buf, 'Test_modify_text_before_prop_1', {})

  call term_sendkeys(buf, "xxia\<Esc>")
  call VerifyScreenDump(buf, 'Test_modify_text_before_prop_2', {})

  call StopVimInTerminal(buf)
endfunc

func Test_overlong_textprop_above_crash()
  CheckRunVimInTerminal

  let lines =<< trim END
  vim9script
  prop_type_add('PropType', {highlight: 'Error'})
  setline(1, ['xxx ', 'yyy'])
  prop_add(1, 0, {
      type: 'PropType',
      text: 'the quick brown fox jumps over the lazy dog. the quick brown fox jumps over the lazy dog. the quick brown fox jumps over the lazy dog.',
      text_align: 'above',
      text_wrap: 'wrap',
  })
  END
  call writefile(lines, 'XtextPropLongAbove', 'D')
  let buf = RunVimInTerminal('-S XtextPropLongAbove', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_long_above_1', {})

  call StopVimInTerminal(buf)
endfunc

func Test_text_prop_list_hl_and_sign_highlight()
  CheckRunVimInTerminal

  let lines =<< trim END
    func Test()
        split Xbuffer
        call setline(1, ['one', "\ttab", '        space', 'three', 'four', 'five'])
        call prop_type_add('Prop1', #{highlight: 'Search', override: v:true})
        sign define sign1 text=>> linehl=DiffAdd
        sign place 10 line=2 name=sign1
        sign place 20 line=3 name=sign1
        call prop_add(1, 1, #{end_lnum: 4, end_col: 5, type: 'Prop1'})
        sign place 30 line=5 name=sign1
    endfunc
    call Test()
  END
  call writefile(lines, 'XtextPropSignTab', 'D')
  let buf = RunVimInTerminal('-S XtextPropSignTab', #{rows: 8, cols: 60})
  call VerifyScreenDump(buf, 'Test_prop_sign_tab_1', {})

  call term_sendkeys(buf, ":setl list listchars=eol:¶,tab:>-\<CR>")
  call VerifyScreenDump(buf, 'Test_prop_sign_tab_2', {})

  call StopVimInTerminal(buf)
endfunc

" Test for getting the virtual text properties
func Test_virtual_text_get()
  new foobar
  call setline(1, '12345678')
  call prop_type_add('test', #{highlight: 'Search'})
  call prop_add(1, 2, #{type: 'test', text: ' virtual text1 '})
  call prop_add(1, 3, #{type: 'test'})
  call prop_add(1, 0, #{type: 'test', text: ' virtual text2 ',
        \               text_align: 'right'})
  call prop_add(1, 5, #{type: 'test'})
  call prop_add(1, 6, #{type: 'test', text: ' virtual text3 ',
        \               text_wrap: 'wrap'})

  let p = prop_list(1, #{end_lnum: -1})
  call assert_equal(
        \ #{lnum: 1, col: 2, type_bufnr: 0, end: 1,
        \   type: 'test', start: 1,
        \   text: ' virtual text1 '}, p[0])
  call assert_equal(
        \ #{lnum: 1, id: 0, col: 3, type_bufnr: 0, end: 1,
        \   type: 'test', length: 0, start: 1}, p[1])
  call assert_equal(
        \ #{lnum: 1, id: 0, col: 5, type_bufnr: 0, end: 1,
        \   type: 'test', length: 0, start: 1}, p[2])
  call assert_equal(
        \ #{lnum: 1, col: 6, type_bufnr: 0, end: 1, type: 'test',
        \   text_wrap: 'wrap', start: 1, text: ' virtual text3 '},
        \  p[3])
  call assert_equal('right', p[4].text_align)

  call prop_type_delete('test')
  bwipe!
endfunc

" This used to throw: E967
func Test_textprop_notype_join()
  new Xtextprop_no_type_join
  call setline(1, range(1, 3))
  call cursor(1, 1)
  let name = 'a'
  call prop_type_add(name, {})
  call prop_add(line('.'), col('.'), { 'type': name })
  call prop_type_delete(name, {})
  join
  call assert_equal(["1 2", "3"], getline(1, '$'))

  bwipe!
endfunc

" vim: shiftwidth=2 sts=2 expandtab
