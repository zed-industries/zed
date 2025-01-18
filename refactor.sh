#!/bin/bash

# Merge process:
#
# * Use mergiraf for merge, with `git merge main -X theirs`
#
#    - Need to use it with a patched tree-sitter-rust. I (Michael)
#      haven't yet uploaded a fork for this, can do if helpful.
#      https://github.com/tree-sitter/tree-sitter-rust/pull/245
#
#    - Watch for newlines between top level decls sometimes disappearing
#
# * Run this script.

re() {
    ruplacer "$1" "$2" crates/ --type *.rs --go
}

re_dry() {
    ruplacer "$1" "$2" crates/ --type *.rs
}

re 'cx\.new_view'                    'cx.new_model'
re 'View<'                           'Model<'

# closure parameters
re ', &mut WindowContext\)'          ', &mut Window, &mut AppContext)'
re ', &mut ViewContext<([^>]+)>\)'   ', &mut Window, &mut ModelContext<$1>)'
re '\(&mut WindowContext\)'          '(&mut Window, &mut AppContext)'
re '\(&mut ViewContext<([^>]+)>\)'   '(&mut Window, &mut ModelContext<$1>)'

# function parameters
re '_cx: &mut WindowContext\)'        '_window: &mut Window, _cx: &mut AppContext)'
re '_cx: &mut ViewContext<([^>]+)>\)' '_window: &mut Window, _cx: &mut ModelContext<$1>)'
re '_cx: &mut WindowContext,'         '_window: &mut Window, _cx: &mut AppContext,'
re '_cx: &mut ViewContext<([^>]+)>,'  '_window: &mut Window, _cx: &mut ModelContext<$1>,'
re 'cx: &mut WindowContext\)'         'window: &mut Window, cx: &mut AppContext)'
re 'cx: &mut ViewContext<([^>]+)>\)'  'window: &mut Window, cx: &mut ModelContext<$1>)'
re 'cx: &mut WindowContext,'          'window: &mut Window, cx: &mut AppContext,'
re 'cx: &mut ViewContext<([^>]+)>,'   'window: &mut Window, cx: &mut ModelContext<$1>,'

re '_cx: &WindowContext\)'            '_window: &Window, _cx: &AppContext)'
re '_cx: &ViewContext<([^>]+)>\)'     '_window: &Window, _cx: &ModelContext<$1>)'
re '_cx: &WindowContext,'             '_window: &Window, _cx: &AppContext,'
re '_cx: &ViewContext<([^>]+)>,'      '_window: &Window, _cx: &ModelContext<$1>,'
re 'cx: &WindowContext\)'             'window: &Window, cx: &AppContext)'
re 'cx: &ViewContext<([^>]+)>\)'      'window: &Window, cx: &ModelContext<$1>)'
re 'cx: &WindowContext,'              'window: &Window, cx: &AppContext,'
re 'cx: &ViewContext<([^>]+)>,'       'window: &Window, cx: &ModelContext<$1>,'

# context methods moved to window
re 'cx\.set_cursor_style\('           'window.set_cursor_style('
re 'cx\.modifiers\('                  'window.modifiers('
re 'cx\.mouse_position\('             'window.mouse_position('

re 'ModelContext<\\' 'ModelContext<'
