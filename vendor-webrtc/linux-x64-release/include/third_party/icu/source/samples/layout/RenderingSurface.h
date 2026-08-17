/*
 *******************************************************************************
 *
 *   © 2016 and later: Unicode, Inc. and others.
 *   License & terms of use: http://www.unicode.org/copyright.html
 *
 *******************************************************************************
 *******************************************************************************
 *
 *   Copyright (C) 1999-2003, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 *
 *******************************************************************************
 *   file name:  RenderingFontInstance.h
 *
 *   created on: 02/20/2003
 *   created by: Eric R. Mader
 */

#ifndef __RENDERINGSURFACE_H
#define __RENDERINGSURFACE_H

#include "layout/LETypes.h"
#include "layout/LEFontInstance.h"

class RenderingSurface
{
public:
    RenderingSurface() {};
    virtual ~RenderingSurface() {};

    virtual void drawGlyphs(const LEFontInstance *font, const LEGlyphID *glyphs, le_int32 count,
                    const float *positions, le_int32 x, le_int32 y, le_int32 width, le_int32 height) = 0;
};

#endif
