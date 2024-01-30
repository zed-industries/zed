" Settings for test script execution
" Always use "COMMAND.COM", don't use the value of "$SHELL".
set shell=c:\COMMAND.COM shellquote= shellxquote= shellcmdflag=/c shellredir=>
" This is used only when the +eval feature is available.
if executable("cmd.exe")
   set shell=cmd.exe
endif

source setup.vim
