" Settings for test script execution
" Always use "sh", don't use the value of "$SHELL".
set shell=sh

" Only when the +eval feature is present.
if 1
  " While some tests overwrite $HOME to prevent them from polluting user files,
  " we need to remember the original value so that we can tell external systems
  " where to ask about their own user settings.
  let g:tester_HOME = $HOME
endif

source setup.vim
