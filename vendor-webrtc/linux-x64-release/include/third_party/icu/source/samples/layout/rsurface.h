/*
 *
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 *
 * (C) Copyright IBM Corp. 1998-2007 - All Rights Reserved
 *
 */

#ifndef __RSURFACE_H
#define __RSURFACE_H

#include "loengine.h"

typedef void rs_surface;

U_CDECL_BEGIN

void rs_drawGlyphs(rs_surface *surface, const le_font *font, const LEGlyphID *glyphs, le_int32 count,
                   const float *positions, le_int32 x, le_int32 y, le_int32 width, le_int32 height);

U_CDECL_END

#endif
