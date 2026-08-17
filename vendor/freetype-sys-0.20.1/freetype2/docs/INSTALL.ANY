Instructions on how to build FreeType with your own build tool
==============================================================

See  the  file `CUSTOMIZE'  to  learn  how  to customize  FreeType  to
specific environments.


I. Standard procedure
---------------------

  * If you use macro names  for FreeType header files (while mandatory
    in earlier versions,  this is now optional  since FreeType version
    2.6.1) it  is necessary to  disable pre-compiled headers.  This is
    very important for Visual C++, because lines like

      #include FT_FREETYPE_H

    are not  correctly supported  by this compiler  while being  ISO C
    compliant!

  * You need to add the directory `include' to your  include path when
    compiling the library.

  * FreeType 2 is made of several  components; each of them is located
    in    a   subdirectory    of    `freetype/src'.    For    example,
    `freetype/src/truetype/' contains the TrueType font driver.

  * DO NOT COMPILE ALL C FILES!  Rather, compile the following ones.

    -- base components (required)

      src/base/ftsystem.c
      src/base/ftinit.c
      src/base/ftdebug.c

      src/base/ftbase.c

      src/base/ftbbox.c       -- recommended, see <ftbbox.h>
      src/base/ftglyph.c      -- recommended, see <ftglyph.h>

      src/base/ftbdf.c        -- optional, see <ftbdf.h>
      src/base/ftbitmap.c     -- optional, see <ftbitmap.h>
      src/base/ftcid.c        -- optional, see <ftcid.h>
      src/base/ftfstype.c     -- optional
      src/base/ftgasp.c       -- optional, see <ftgasp.h>
      src/base/ftgxval.c      -- optional, see <ftgxval.h>
      src/base/ftmm.c         -- optional, see <ftmm.h>
      src/base/ftotval.c      -- optional, see <ftotval.h>
      src/base/ftpatent.c     -- optional
      src/base/ftpfr.c        -- optional, see <ftpfr.h>
      src/base/ftstroke.c     -- optional, see <ftstroke.h>
      src/base/ftsynth.c      -- optional, see <ftsynth.h>
      src/base/fttype1.c      -- optional, see <t1tables.h>
      src/base/ftwinfnt.c     -- optional, see <ftwinfnt.h>

      src/base/ftmac.c        -- only on the Macintosh

    -- font drivers (optional; at least one is needed)

      src/bdf/bdf.c           -- BDF font driver
      src/cff/cff.c           -- CFF/OpenType font driver
      src/cid/type1cid.c      -- Type 1 CID-keyed font driver
      src/pcf/pcf.c           -- PCF font driver
      src/pfr/pfr.c           -- PFR/TrueDoc font driver
      src/sfnt/sfnt.c         -- SFNT files support
                                 (TrueType & OpenType)
      src/truetype/truetype.c -- TrueType font driver
      src/type1/type1.c       -- Type 1 font driver
      src/type42/type42.c     -- Type 42 font driver
      src/winfonts/winfnt.c   -- Windows FONT / FNT font driver

    -- rasterizers (optional; at least one is needed for vector
       formats)

      src/raster/raster.c     -- monochrome rasterizer
      src/sdf/sdf.c           -- Signed Distance Field driver
      src/smooth/smooth.c     -- anti-aliasing rasterizer

    -- auxiliary modules (optional)

      src/autofit/autofit.c   -- auto hinting module
      src/cache/ftcache.c     -- cache sub-system (in beta)
      src/gzip/ftgzip.c       -- support for compressed fonts (.gz)
      src/lzw/ftlzw.c         -- support for compressed fonts (.Z)
      src/bzip2/ftbzip2.c     -- support for compressed fonts (.bz2)
      src/gxvalid/gxvalid.c   -- TrueTypeGX/AAT table validation
      src/otvalid/otvalid.c   -- OpenType table validation
      src/psaux/psaux.c       -- PostScript Type 1 parsing
      src/pshinter/pshinter.c -- PS hinting module
      src/psnames/psnames.c   -- PostScript glyph names support


    Notes:

      `ftcache.c'  needs `ftglyph.c'
      `ftfstype.c' needs `fttype1.c'
      `ftglyph.c'  needs `ftbitmap.c'
      `ftstroke.c' needs `ftglyph.c'
      `ftsynth.c'  needs `ftbitmap.c'

      `cff.c'      needs `sfnt.c', `pshinter.c', and `psnames.c'
      `truetype.c' needs `sfnt.c' and `psnames.c'
      `type1.c'    needs `psaux.c' `pshinter.c', and `psnames.c'
      `type1cid.c' needs `psaux.c', `pshinter.c', and `psnames.c'
      `type42.c'   needs `truetype.c'

      Please consult the central  `include/freetype/config/ftoption.h'
      configuration file for details on additional libraries necessary
      for some optional features.


  Read the file `CUSTOMIZE' in case  you want to compile only a subset
  of  the  drivers,  renderers,   and  optional  modules;  a  detailed
  description of the various base  extension is given in the top-level
  file `modules.cfg'.

  You are done.  In case of problems, see the archives of the FreeType
  development mailing list.


II. Support for flat-directory compilation
------------------------------------------

  It is  possible to  put all  FreeType 2 source  files into  a single
  directory, with the *exception* of the `include' hierarchy.

  1. Copy all files in current directory

      cp freetype/src/base/*.[hc] .
      cp freetype/src/raster1/*.[hc] .
      cp freetype/src/smooth/*.[hc] .
      etc.

  2. Compile sources

      cc -c -Iinclude -DFT2_BUILD_LIBRARY ftsystem.c
      cc -c -Iinclude -DFT2_BUILD_LIBRARY ftinit.c
      cc -c -Iinclude -DFT2_BUILD_LIBRARY ftdebug.c
      cc -c -Iinclude -DFT2_BUILD_LIBRARY ftbase.c
      etc.

    You don't  need to define  the FT_FLAT_COMPILATION macro  (as this
    was required in previous releases of FreeType 2).

----------------------------------------------------------------------

Copyright (C) 2003-2023 by
David Turner, Robert Wilhelm, and Werner Lemberg.

This  file is  part of  the FreeType  project, and  may only  be used,
modified,  and distributed  under the  terms of  the  FreeType project
license,  LICENSE.TXT.  By  continuing to  use, modify,  or distribute
this file you  indicate that you have read  the license and understand
and accept it fully.


--- end of INSTALL.ANY ---
