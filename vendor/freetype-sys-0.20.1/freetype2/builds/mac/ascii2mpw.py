#!/usr/bin/env python
import sys
import string

if len( sys.argv ) == 1 :
  for asc_line in sys.stdin.readlines():
    mpw_line = string.replace(asc_line, "\\xA5", "\245")
    mpw_line = string.replace(mpw_line, "\\xB6", "\266")
    mpw_line = string.replace(mpw_line, "\\xC4", "\304")
    mpw_line = string.replace(mpw_line, "\\xC5", "\305")
    mpw_line = string.replace(mpw_line, "\\xFF", "\377")
    mpw_line = string.replace(mpw_line, "\n",   "\r")
    mpw_line = string.replace(mpw_line, "\\n",   "\n")
    sys.stdout.write(mpw_line)
elif sys.argv[1] == "-r" :
  for mpw_line in sys.stdin.readlines():
    asc_line = string.replace(mpw_line, "\n",   "\\n")
    asc_line = string.replace(asc_line, "\r",   "\n")
    asc_line = string.replace(asc_line, "\245", "\\xA5")
    asc_line = string.replace(asc_line, "\266", "\\xB6")
    asc_line = string.replace(asc_line, "\304", "\\xC4")
    asc_line = string.replace(asc_line, "\305", "\\xC5")
    asc_line = string.replace(asc_line, "\377", "\\xFF")
    sys.stdout.write(asc_line)
