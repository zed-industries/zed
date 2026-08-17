/*
 *
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 *
 * (C) Copyright IBM Corp. 1998-2007 - All Rights Reserved
 *
 */

#ifndef __GNOMEGLUE_H
#define __GNOMEGLUE_H

#include <gnome.h>
#include <ft2build.h>
#include FT_FREETYPE_H

#include "unicode/utypes.h"

#include "LETypes.h"
#include "loengine.h"
#include "gsupport.h"
#include "rsurface.h"

typedef void fm_fontMap;

U_CDECL_BEGIN

gs_guiSupport *gs_gnomeGuiSupportOpen();
void gs_gnomeGuiSupportClose(gs_guiSupport *guiSupport);

rs_surface *rs_gnomeRenderingSurfaceOpen(GtkWidget *theWidget);
void rs_gnomeRenderingSurfaceClose(rs_surface *surface);

fm_fontMap *fm_gnomeFontMapOpen(FT_Library engine, const char *fileName, le_int16 pointSize, gs_guiSupport *guiSupport, LEErrorCode *status); 
void fm_fontMapClose(fm_fontMap *fontMap);

le_font *le_scriptCompositeFontOpen(fm_fontMap *fontMap);
void le_fontClose(le_font *font);

U_CDECL_END

#endif
