/****************************************************************************
 *
 * ft2build.h
 *
 *   FreeType 2 build and setup macros (development version).
 *
 * Copyright (C) 1996-2023 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */


 /*
  * This is a development version of <ft2build.h> to build the library in
  * debug mode.  Its only difference to the default version is that it
  * includes a local `ftoption.h' header file with different settings for
  * many configuration macros.
  *
  * To use it, simply ensure that the directory containing this file is
  * scanned by the compiler before the default FreeType header directory.
  *
  */

#ifndef FT2BUILD_H_
#define FT2BUILD_H_

#define FT_CONFIG_MODULES_H  <ftmodule.h>
#define FT_CONFIG_OPTIONS_H  <ftoption.h>

#include <freetype/config/ftheader.h>

#endif /* FT2BUILD_H_ */


/* END */
