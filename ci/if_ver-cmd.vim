" Provide 'PrintVer' command to print the interface versions.

func s:print_ver(lang, ...)
  if has(a:lang)
    exec a:lang join(a:000)
  else
    echo 'N/A'
  endif
  echo ''
endfunc

command -nargs=+ PrintVer call <SID>print_ver(<f-args>)
