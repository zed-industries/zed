/*
 *
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 *
 * (C) Copyright IBM Corp. 1998-2007 - All Rights Reserved
 *
 */

#ifndef __GDIGLUE_H
#define __GDIGLUE_H

#include <windows.h>

#include "unicode/utypes.h"

#include "LETypes.h"
#include "loengine.h"
#include "gsupport.h"
#include "rsurface.h"

typedef void fm_fontMap;

U_CDECL_BEGIN

gs_guiSupport *gs_gdiGuiSupportOpen();
void gs_gdiGuiSupportClose(gs_guiSupport *guiSupport);

rs_surface *rs_gdiRenderingSurfaceOpen(HDC hdc);
void rs_gdiRenderingSurfaceSetHDC(rs_surface *surface, HDC hdc);
void rs_gdiRenderingSurfaceClose(rs_surface *surface);

fm_fontMap *fm_gdiFontMapOpen(rs_surface *surface, const char *fileName, le_int16 pointSize, gs_guiSupport *guiSupport, LEErrorCode *status); 
void fm_fontMapClose(fm_fontMap *fontMap);

le_font *le_scriptCompositeFontOpen(fm_fontMap *fontMap);
void le_fontClose(le_font *font);

U_CDECL_END

#endif
