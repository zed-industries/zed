#
# Makefile for VIM on Win32, using MinGW cross compiler on Cygwin
#
# Also read INSTALLpc.txt!
#
# This compiles Vim as a Windows application.  If you want Vim to run as a
# Cygwin application use the Makefile (just like on Unix).
#
# The old Make_cyg.mak (maintained by Dan Sharp et al.) was merged into
# Make_cyg_ming.mak.  Note: USEDLL option was removed.
# This file contains Cygwin specific settings. Common settings are contained
# in Make_cyg_ming.mak.
#
# Last updated by Ken Takata.
# Last Change: 2014 Oct 21


# uncomment 'PERL' if you want a perl-enabled version
#PERL=/cygdrive/c/perl

# uncomment 'LUA' if you want a Lua-enabled version
#LUA=/cygdrive/c/lua

# uncomment 'MZSCHEME' if you want a MzScheme-enabled version
#MZSCHEME=/cygdrive/d/plt

# uncomment 'PYTHON' if you want a python-enabled version
#PYTHON=/cygdrive/c/python20

# uncomment 'PYTHON3' if you want a python3-enabled version
#PYTHON3=/cygdrive/c/python31

# uncomment 'TCL' if you want a Tcl-enabled version
#TCL=/cygdrive/c/tcl

# uncomment 'RUBY' if you want a Ruby-enabled version
#RUBY=/cygdribe/c/ruby


# Use MinGW-w64 cross compiler.
# There are two MinGW-w64 packages in Cygwin:
#   32-bit: mingw64-i686-gcc-g++
#   64-bit: mingw64-x86_64-gcc-g++
# You may also need to set 'ARCH' in Make_cyg_ming.mak.
CROSS_COMPILE = i686-w64-mingw32-
#CROSS_COMPILE = x86_64-w64-mingw32-


# Do not change this.
UNDER_CYGWIN = yes
include Make_cyg_ming.mak

# vim: set noet sw=8 ts=8 sts=0 wm=0 tw=0:
