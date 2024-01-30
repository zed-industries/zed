" This script makes a tags file for help text.
"
" Usage: vim -eX -u doctags.vim

try
  helptags ++t .
  echo 'help tags updated'
catch
  echo v:exception
  echo 'help tags failed update'
endtry
echo ''
qa!
