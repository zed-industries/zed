How to customize the compilation of the library
===============================================

  FreeType  is highly  customizable  to fit  various  needs, and  this
  document  describes  how  it  is  possible  to  select  options  and
  components at compilation time.


I. Configuration macros

  The  file `include/freetype/config/ftoption.h'  contains a  list  of
  commented configuration macros that can  be toggled by developers to
  indicate which features should be active while building the library.

  These  options range  from debug  level to  availability  of certain
  features,   like  native   TrueType  hinting   through   a  bytecode
  interpreter.

  We  invite you  to read  this file  for more  information.   You can
  change the  file's content to suit  your needs, or  override it with
  one of the techniques described below.


II. Modules list

  If you  use GNU make  please edit the top-level  file `modules.cfg'.
  It contains a  list of available FreeType modules  and extensions to
  be compiled.  Change it to suit your own preferences.  Be aware that
  certain modules  depend on  others, as described  in the  file.  GNU
  make  uses `modules.cfg'  to  generate `ftmodule.h'  (in the  object
  directory).

  If you build FreeType in a directory separate from the source files,
  put your  customized `modules.cfg' in  that directory; that  way you
  can keep the source files `clean'.

  If  you don't  use  GNU make  you  have to  manually  edit the  file
  `include/freetype/config/ftmodule.h' (which  is *not* used  with  if
  compiled with GNU make) to add  or remove the drivers and components
  you want  to compile into  the library.  See `INSTALL.ANY'  for more
  information.


III. System interface

  FreeType's  default interface to  the system  (i.e., the  parts that
  deal  with  memory  management   and  i/o  streams)  is  located  in
  `src/base/ftsystem.c'.

  The current  implementation uses standard C library  calls to manage
  memory  and to read  font files.   It is  however possible  to write
  custom implementations to suit specific systems.

  To  tell the  GNU Make-based  build system  to use  a  custom system
  interface, you have to  define the environment variable FTSYS_SRC to
  point to the relevant implementation:

    on Unix:

      ./configure <your options>
      export FTSYS_SRC=foo/my_ftsystem.c
      make
      make install

    on Windows:

      make setup <compiler>
      set FTSYS_SRC=foo/my_ftsystem.c
      make


IV. Overriding default configuration and module headers

  It  is possible  to override  the default  configuration  and module
  headers without  changing the original files.  There  are three ways
  to do that:


  1. With GNU make

    [This is actually a combination of method 2 and 3.]

    Just put your custom `ftoption.h'  file into the objects directory
    (normally `<topdir>/objs' if you build  in the source tree, or the
    directory where  you invoke configure  if you build in  a separate
    directory), which GNU make prefers over the standard location.  No
    action  is  needed  for   `ftmodule.h'  because  it  is  generated
    automatically in the objects directory.

  2. Using the C include path

    Use the  C include path  to ensure that  your own versions  of the
    files are used at compile time when the lines

      #include FT_CONFIG_OPTIONS_H
      #include FT_CONFIG_MODULES_H

    are      compiled.       Their      default      values      being
    <freetype/config/ftoption.h> and <freetype/config/ftmodule.h>, you
    can do something like:

      custom/
        config/
          ftoption.h      => custom options header
          ftmodule.h      => custom modules list

      include/            => normal FreeType 2 include
        ...

    then change the C include path to always give the path to `custom'
    before the FreeType 2 `include'.


  3. Redefining FT_CONFIG_OPTIONS_H and FT_CONFIG_MODULES_H

    Another way to do the same thing is to redefine the macros used to
    name  the configuration  headers.  To  do  so, you  need a  custom
    `ft2build.h' whose content can be as simple as:

      #ifndef FT2_BUILD_MY_PLATFORM_H_
      #define FT2_BUILD_MY_PLATFORM_H_

      #define FT_CONFIG_OPTIONS_H  <custom/my-ftoption.h>
      #define FT_CONFIG_MODULES_H  <custom/my-ftmodule.h>

      #include <freetype/config/ftheader.h>

      #endif /* FT2_BUILD_MY_PLATFORM_H_ */

    Place those files in a separate directory, e.g.,

      custom/
        ft2build.h           => custom version described above
        my-ftoption.h        => custom options header
        my-ftmodule.h        => custom modules list header

    and change  the C include path  to ensure that  `custom' is always
    placed before the FT2 `include' during compilation.

----------------------------------------------------------------------

Copyright (C) 2003-2023 by
David Turner, Robert Wilhelm, and Werner Lemberg.

This  file is  part of  the FreeType  project, and  may only  be used,
modified,  and distributed  under the  terms of  the  FreeType project
license,  LICENSE.TXT.  By  continuing to  use, modify,  or distribute
this file you  indicate that you have read  the license and understand
and accept it fully.


--- end of CUSTOMIZE ---
