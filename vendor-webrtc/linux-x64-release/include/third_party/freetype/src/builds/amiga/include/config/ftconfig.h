/***************************************************************************/
/*                                                                         */
/*  ftconfig.h                                                             */
/*                                                                         */
/*    Amiga-specific configuration file (specification only).              */
/*                                                                         */
/*  Copyright (C) 2005-2024 by                                             */
/*  Werner Lemberg and Detlef Würkner.                                     */
/*                                                                         */
/*  This file is part of the FreeType project, and may only be used,       */
/*  modified, and distributed under the terms of the FreeType project      */
/*  license, LICENSE.TXT.  By continuing to use, modify, or distribute     */
/*  this file you indicate that you have read the license and              */
/*  understand and accept it fully.                                        */
/*                                                                         */
/***************************************************************************/

/*
 * This is an example how to override the default FreeType2 header files
 * with Amiga-specific changes. When the compiler searches this directory
 * before the default directory, we can do some modifications.
 *
 * Here we must change FT_EXPORT_DEF so that SAS/C does
 * generate the needed XDEFs.
 */

#if 0
#define FT_EXPORT_DEF( x )  extern  x
#endif

#undef FT_EXPORT_DEF
#define FT_EXPORT_DEF( x )  x

/* Now include the original file */
#ifndef __MORPHOS__
#ifdef __SASC
#include "FT:include/freetype/config/ftconfig.h"
#else
#include "/FT/include/freetype/config/ftconfig.h"
#endif
#else
/* We must define that, it seems that
 * lib/gcc-lib/ppc-morphos/2.95.3/include/syslimits.h is missing in
 * ppc-morphos-gcc-2.95.3-bin.tgz (gcc for 68k producing MorphOS PPC elf
 * binaries from http://www.morphos.de)
 */
#define _LIBC_LIMITS_H_
#include "/FT/include/freetype/config/ftconfig.h"
#endif

/*
Local Variables:
coding: latin-1
End:
*/
