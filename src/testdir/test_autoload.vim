" Tests for autoload

set runtimepath=./sautest

func Test_autoload_dict_func()
  let g:loaded_foo_vim = 0
  let g:called_foo_bar_echo = 0
  call g:foo#bar.echo()
  call assert_equal(1, g:loaded_foo_vim)
  call assert_equal(1, g:called_foo_bar_echo)

  eval 'bar'->g:foo#addFoo()->assert_equal('barfoo')

  " empty name works in legacy script
  call assert_equal('empty', foo#())
endfunc

func Test_source_autoload()
  let g:loaded_sourced_vim = 0
  source sautest/autoload/sourced.vim
  call assert_equal(1, g:loaded_sourced_vim)
endfunc

func Test_autoload_vim9script()
  call assert_equal('some', auto9#Getsome())
  call assert_equal(49, auto9#Add42(7))
endfunc


" vim: shiftwidth=2 sts=2 expandtab
