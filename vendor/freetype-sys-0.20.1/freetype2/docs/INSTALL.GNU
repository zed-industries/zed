This document contains instructions how  to build the FreeType library
on non-Unix systems with  the help of GNU Make.  Note  that if you are
running  Cygwin  or  MinGW/MSYS  in Windows,  you  should  follow  the
instructions in the file `INSTALL.UNIX' instead.


  FreeType 2 includes a powerful and flexible build system that allows
  you to  easily compile it on  a great variety of  platforms from the
  command line.  To do so, just follow these simple instructions.

  1. Install GNU Make
  -------------------

    The FreeType 2 build system relies on many features special to GNU
    Make.

    NEARLY ALL OTHER MAKE TOOLS  FAIL, INCLUDING `BSD MAKE', SO REALLY
    INSTALL A RECENT VERSION OF GNU MAKE ON YOUR SYSTEM!

    Note that  make++, a  make tool written  in Perl,  supports enough
    features of GNU make to compile FreeType.  See

      https://makepp.sourceforge.net

    for more information;  you need version 2.0 or newer, and you must
    pass option `--norc-substitution'.

    Make sure that you are invoking GNU Make from the command line, by
    typing something like:

      make -v

    to display its version number.

    VERSION 3.81 OR NEWER IS NEEDED!


  2. Invoke `make'
  ----------------

    Go to  the root  directory of FreeType  2, then simply  invoke GNU
    Make from the command line.   This will launch the FreeType 2 host
    platform  detection routines.   A summary  will be  displayed, for
    example, on Win32.


      ==============================================================
      FreeType build system -- automatic system detection

      The following settings are used:

        platform                     windows
        compiler                     gcc
        configuration directory      .\builds\windows
        configuration rules          .\builds\windows\w32-gcc.mk

      If this does not correspond to your system or settings please
      remove the file 'config.mk' from this directory then read the
      INSTALL file for help.

      Otherwise, simply type 'make' again to build the library
      or 'make refdoc' to build the API reference (the latter needs
      Python >= 3.5).
      =============================================================


    If the detected settings correspond to your platform and compiler,
    skip to step 5.  Note that if your platform is completely alien to
    the build system, the detected platform will be `ansi'.


  3. Configure the build system for a different compiler
  ------------------------------------------------------

    If the build system correctly detected your platform, but you want
    to use a different compiler  than the one specified in the summary
    (for most platforms, gcc is the default compiler), invoke GNU Make
    with

      make setup <compiler>

    Examples:

      to use Visual C++ on Win32, type:  `make setup visualc'
      to use Borland C++ on Win32, type  `make setup bcc32'
      to use Watcom C++ on Win32, type   `make setup watcom'
      to use Intel C++ on Win32, type    `make setup intelc'
      to use LCC-Win32 on Win32, type:   `make setup lcc'
      to use Watcom C++ on OS/2, type    `make setup watcom'
      to use VisualAge C++ on OS/2, type `make setup visualage'

    The  <compiler> name to  use is  platform-dependent.  The  list of
    available  compilers for  your  system is  available  in the  file
    `builds/<system>/detect.mk'.

    If you  are satisfied  by the new  configuration summary,  skip to
    step 5.


  3a. Use clang instead of gcc
  ----------------------------

    The `clang'  compiler can  use FreeType's setup  for `gcc';  it is
    sufficient to set the `CC' variable, for example

      make CC=clang


  3b. Compiling with a C++ compiler
  ---------------------------------

    FreeType can be built with a C++ compiler, for example

      make CC="g++"

    If `clang++' should  be used it is necessary to  also override the
    `ANSIFLAGS' variable:

      make CC="clang++" ANSIFLAGS=""


  4. Configure the build system for an unknown platform/compiler
  --------------------------------------------------------------

    The auto-detection/setup  phase of the build system  copies a file
    to the current directory under the name `config.mk'.

    For    example,    on    OS/2+gcc,    it   would    simply    copy
    `builds/os2/os2-gcc.mk' to `./config.mk'.

    If for  some reason your  platform isn't correctly  detected, copy
    manually the configuration sub-makefile to `./config.mk' and go to
    step 5.

    Note  that  this file  is  a  sub-Makefile  used to  specify  Make
    variables  for compiler  and linker  invocation during  the build.
    You can  easily create your own  version from one  of the existing
    configuration files,  then copy it to the  current directory under
    the name `./config.mk'.


  5. Build the library
  --------------------

    The auto-detection/setup  phase should have  copied a file  in the
    current  directory,  called  `./config.mk'.   This  file  contains
    definitions of various Make  variables used to invoke the compiler
    and linker during the build.  [It has also generated a file called
    `ftmodule.h'   in  the  objects   directory  (which   is  normally
    `<toplevel>/objs/');  please read  the  file `docs/CUSTOMIZE'  for
    customization of FreeType.]

    To  launch  the build,  simply  invoke  GNU  Make again:  The  top
    Makefile will detect the configuration file and run the build with
    it.  If you have used variables in  step 3, you must use  the same
    variables here, too.


  Final note

    The above instructions build a  _statically_ linked library of the
    font engine in the `objs' directory.   On Windows, you can build a
    DLL  either  with  MinGW  (within an  MSYS  shell,  following  the
    instructions in `INSTALL.UNIX'), or you  use one of the Visual C++
    project files; see  the  subdirectories  of `builds/windows'.  For
    everything else,  you are on  your own,  and you might  follow the
    instructions in `INSTALL.ANY' to create your own Makefiles.

----------------------------------------------------------------------

Copyright (C) 2003-2023 by
David Turner, Robert Wilhelm, and Werner Lemberg.

This  file is  part of  the FreeType  project, and  may only  be used,
modified,  and distributed  under the  terms of  the  FreeType project
license,  LICENSE.TXT.  By  continuing to  use, modify,  or distribute
this file you  indicate that you have read  the license and understand
and accept it fully.


--- end of INSTALL.GNU ---
