/***************************************************************************/
/*                                                                         */
/*  ftmodule.h                                                             */
/*                                                                         */
/*    Amiga-specific FreeType module selection.                            */
/*                                                                         */
/*  Copyright (C) 2005-2023 by                                             */
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
 * To avoid that all your programs include all FreeType modules,
 * you copy the following piece of source code into your own
 * source file and specify which modules you really need in your
 * application by uncommenting the appropriate lines.
 */
/*
//#define FT_USE_AUTOFIT // autofitter
//#define FT_USE_RASTER  // monochrome rasterizer
//#define FT_USE_SMOOTH  // anti-aliasing rasterizer
//#define FT_USE_TT      // truetype font driver
//#define FT_USE_T1      // type1 font driver
//#define FT_USE_T42     // type42 font driver
//#define FT_USE_T1CID   // cid-keyed type1 font driver  // no cmap support
//#define FT_USE_CFF     // opentype font driver
//#define FT_USE_BDF     // bdf bitmap font driver
//#define FT_USE_PCF     // pcf bitmap font driver
//#define FT_USE_PFR     // pfr font driver
//#define FT_USE_WINFNT  // windows .fnt|.fon bitmap font driver
//#define FT_USE_OTV     // opentype validator
//#define FT_USE_GXV     // truetype gx validator
#include "FT:src/base/ftinit.c"
*/

/* Make sure that the needed support modules are built in.
 * Dependencies can be found by searching for FT_Get_Module.
 */

#ifdef FT_USE_T42
#define FT_USE_TT
#endif

#ifdef FT_USE_TT
#define FT_USE_SFNT
#endif

#ifdef FT_USE_CFF
#define FT_USE_SFNT
#define FT_USE_PSHINT
#define FT_USE_PSNAMES
#endif

#ifdef FT_USE_T1
#define FT_USE_PSAUX
#define FT_USE_PSHINT
#define FT_USE_PSNAMES
#endif

#ifdef FT_USE_T1CID
#define FT_USE_PSAUX
#define FT_USE_PSHINT
#define FT_USE_PSNAMES
#endif

#ifdef FT_USE_PSAUX
#define FT_USE_PSNAMES
#endif

#ifdef FT_USE_SFNT
#define FT_USE_PSNAMES
#endif

/* Now include the modules */

#ifdef FT_USE_AUTOFIT
FT_USE_MODULE( FT_Module_Class, autofit_module_class )
#endif

#ifdef FT_USE_TT
FT_USE_MODULE( FT_Driver_ClassRec, tt_driver_class )
#endif

#ifdef FT_USE_T1
FT_USE_MODULE( FT_Driver_ClassRec, t1_driver_class )
#endif

#ifdef FT_USE_CFF
FT_USE_MODULE( FT_Driver_ClassRec, cff_driver_class )
#endif

#ifdef FT_USE_T1CID
FT_USE_MODULE( FT_Driver_ClassRec, t1cid_driver_class )
#endif

#ifdef FT_USE_PFR
FT_USE_MODULE( FT_Driver_ClassRec, pfr_driver_class )
#endif

#ifdef FT_USE_T42
FT_USE_MODULE( FT_Driver_ClassRec, t42_driver_class )
#endif

#ifdef FT_USE_WINFNT
FT_USE_MODULE( FT_Driver_ClassRec, winfnt_driver_class )
#endif

#ifdef FT_USE_PCF
FT_USE_MODULE( FT_Driver_ClassRec, pcf_driver_class )
#endif

#ifdef FT_USE_PSAUX
FT_USE_MODULE( FT_Module_Class, psaux_module_class )
#endif

#ifdef FT_USE_PSNAMES
FT_USE_MODULE( FT_Module_Class, psnames_module_class )
#endif

#ifdef FT_USE_PSHINT
FT_USE_MODULE( FT_Module_Class, pshinter_module_class )
#endif

#ifdef FT_USE_RASTER
FT_USE_MODULE( FT_Renderer_Class, ft_raster1_renderer_class )
#endif

#ifdef FT_USE_SFNT
FT_USE_MODULE( FT_Module_Class, sfnt_module_class )
#endif

#ifdef FT_USE_SMOOTH
FT_USE_MODULE( FT_Renderer_Class, ft_smooth_renderer_class )
#endif

#ifdef FT_USE_OTV
FT_USE_MODULE( FT_Module_Class, otv_module_class )
#endif

#ifdef FT_USE_BDF
FT_USE_MODULE( FT_Driver_ClassRec, bdf_driver_class )
#endif

#ifdef FT_USE_GXV
FT_USE_MODULE( FT_Module_Class, gxv_module_class )
#endif

/*
Local Variables:
coding: latin-1
End:
*/
