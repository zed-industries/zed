This is XPM library compiled for Windows which is intended for use with Vim
'signs' feature.

Libraries in x86 directory were compiled with MSVC6 and MinGW. Proposed
commands to compile Vim are:

If you want to build XPM library by yourself, you may want to use the
following Win32 port:
https://github.com/koron/libXpm-win32

Any version of MSVC starting from version 6.0:
nmake -f Make_mvc.mak GUI=yes CSCOPE=yes NETBEANS=yes XPM=e:\hg\xpm\x86

MinGW:
mingw32-make -f Make_ming.mak GUI=yes CSCOPE=yes XPM=e:/hg/xpm/x86

MinGW 64 for x64:
mingw32-make -f Make_ming.mak GUI=yes ARCH=x86-64 XPM=E:\HG\xpm\x64

Microsoft Visual C++ on x64 (tested with versions 2008 and 2010):
nmake -f Make_mvc.mak GUI=yes CSCOPE=yes XPM=E:\HG\xpm\x64

To test, open some file in Vim and execute commands below:
:exe 'sign define vimxpm icon='.$VIMRUNTIME.'\\vim32x32.xpm'
:exe 'sign place 1 line=1 name=vimxpm file='.expand('%:p')


See COPYRIGHT for XPM licence.

If you have questions please email sergey.khorev@gmail.com.
