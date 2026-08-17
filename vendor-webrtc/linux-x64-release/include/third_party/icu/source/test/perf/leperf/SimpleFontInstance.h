/*
 *******************************************************************************
 *
 *   © 2016 and later: Unicode, Inc. and others.
 *   License & terms of use: http://www.unicode.org/copyright.html
 *
 *******************************************************************************
 *******************************************************************************
 *
 *   Copyright (C) 1999-2013, International Business Machines
 *   Corporation and others.  All Rights Reserved.
 *
 *******************************************************************************
 *   file name:  SimpleFontInstance.h
 *
 *   created on: 03/30/2006
 *   created by: Eric R. Mader
 */

#ifndef __SIMPLEFONTINSTANCE_H
#define __SIMPLEFONTINSTANCE_H

#include "layout/LETypes.h"
#include "layout/LEFontInstance.h"

U_NAMESPACE_USE

class SimpleFontInstance : public LEFontInstance
{
private:
    float     fPointSize;
    le_int32  fAscent;
    le_int32  fDescent;

protected:
    const void *readFontTable(LETag tableTag) const;

public:
    SimpleFontInstance(float pointSize, LEErrorCode &status);

    virtual ~SimpleFontInstance();

    virtual const void *getFontTable(LETag tableTag) const;

    virtual le_int32 getUnitsPerEM() const;

    virtual le_int32 getAscent() const;

    virtual le_int32 getDescent() const;

    virtual le_int32 getLeading() const;

    // We really want to inherit this method from the superclass, but some compilers
    // issue a warning if we don't implement it...
    virtual LEGlyphID mapCharToGlyph(LEUnicode32 ch, const LECharMapper *mapper, le_bool filterZeroWidth) const;
    
    // We really want to inherit this method from the superclass, but some compilers
    // issue a warning if we don't implement it...
    virtual LEGlyphID mapCharToGlyph(LEUnicode32 ch, const LECharMapper *mapper) const;

    virtual LEGlyphID mapCharToGlyph(LEUnicode32 ch) const;

    virtual void getGlyphAdvance(LEGlyphID glyph, LEPoint &advance) const;

    virtual le_bool getGlyphPoint(LEGlyphID glyph, le_int32 pointNumber, LEPoint &point) const;

    float getXPixelsPerEm() const;

    float getYPixelsPerEm() const;

    float getScaleFactorX() const;

    float getScaleFactorY() const;

};

#endif
