
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
 *   file name:  GDIFontInstance.h
 *
 *   created on: 08/09/2000
 *   created by: Eric R. Mader
 */

#ifndef __GDIFONTINSTANCE_H
#define __GDIFONTINSTANCE_H

#include <windows.h>

#include "layout/LETypes.h"
#include "layout/LEFontInstance.h"
#include "RenderingSurface.h"
#include "FontTableCache.h"
#include "cmaps.h"

class GDIFontInstance;

class GDISurface : public RenderingSurface
{
public:
    GDISurface(HDC theHDC);
    virtual ~GDISurface();

    virtual void drawGlyphs(const LEFontInstance *font, const LEGlyphID *glyphs, le_int32 count,
        const float *positions, le_int32 x, le_int32 y, le_int32 width, le_int32 height);

    void setFont(const GDIFontInstance *font);
    HDC  getHDC() const;
    void setHDC(HDC theHDC);

private:
    HDC fHdc;
    const GDIFontInstance *fCurrentFont;
};

inline HDC GDISurface::getHDC() const
{
    return fHdc;
}

class GDIFontInstance : public LEFontInstance, protected FontTableCache
{
protected:
    GDISurface *fSurface;
    HFONT fFont;

    le_int32 fPointSize;
    le_int32 fUnitsPerEM;
    le_int32 fAscent;
    le_int32 fDescent;
    le_int32 fLeading;

    float fDeviceScaleX;
    float fDeviceScaleY;

    CMAPMapper *fMapper;

    virtual const void *readFontTable(LETag tableTag) const;

    virtual LEErrorCode initMapper();

public:
    GDIFontInstance(GDISurface *surface, TCHAR *faceName, le_int16 pointSize, LEErrorCode &status);
    GDIFontInstance(GDISurface *surface, const char *faceName, le_int16 pointSize, LEErrorCode &status);
    //GDIFontInstance(GDISurface *surface, le_int16 pointSize);

    virtual ~GDIFontInstance();

    HFONT getFont() const;

    virtual const void *getFontTable(LETag tableTag) const;

    virtual le_int32 getUnitsPerEM() const;

    virtual le_int32 getAscent() const;

    virtual le_int32 getDescent() const;

    virtual le_int32 getLeading() const;

    virtual LEGlyphID mapCharToGlyph(LEUnicode32 ch) const;

    virtual void getGlyphAdvance(LEGlyphID glyph, LEPoint &advance) const;

    virtual le_bool getGlyphPoint(LEGlyphID glyph, le_int32 pointNumber, LEPoint &point) const;

    float getXPixelsPerEm() const;

    float getYPixelsPerEm() const;

    float getScaleFactorX() const;

    float getScaleFactorY() const;
};

inline HFONT GDIFontInstance::getFont() const
{
    return fFont;
}

inline le_int32 GDIFontInstance::getUnitsPerEM() const
{
    return fUnitsPerEM;
}

inline le_int32 GDIFontInstance::getAscent() const
{
    return fAscent;
}

inline le_int32 GDIFontInstance::getDescent() const
{
    return fDescent;
}

inline le_int32 GDIFontInstance::getLeading() const
{
    return fLeading;
}

inline LEGlyphID GDIFontInstance::mapCharToGlyph(LEUnicode32 ch) const
{
    return fMapper->unicodeToGlyph(ch);
}

inline float GDIFontInstance::getXPixelsPerEm() const
{
    return (float) fPointSize;
}

inline float GDIFontInstance::getYPixelsPerEm() const
{
    return  (float) fPointSize;
}

inline float GDIFontInstance::getScaleFactorX() const
{
    return fDeviceScaleX;
}

inline float GDIFontInstance::getScaleFactorY() const
{
    return fDeviceScaleY;
}

#endif
