/****************************************************************************
 *
 * dlgwrap.c
 *
 *   Wrapper file for the 'dlg' library (body only)
 *
 * Copyright (C) 2020-2023 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */


#include <ft2build.h>
#include FT_CONFIG_OPTIONS_H


#ifdef FT_DEBUG_LOGGING
#define DLG_STATIC
#include "dlg.c"
#else
  /* ANSI C doesn't like empty source files */
  typedef int  dlg_dummy_;
#endif


/* END */
