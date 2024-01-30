" Print py3 interface versions for Ubuntu. Part 2.

if 1
  execute 'source' expand('<sfile>:h') .. '/if_ver-cmd.vim'

  echo 'Python 3:'
  PrintVer python3 print(sys.version)
endif
