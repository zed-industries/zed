// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/***************************************************************************
*
*   Copyright (C) 1998-2013, International Business Machines
*   Corporation and others.  All Rights Reserved.
*
************************************************************************/


#ifndef __CMAPS_H
#define __CMAPS_H

#include "layout/LETypes.h"
//#include "letest.h"
#include "sfnt.h"

class CMAPMapper
{
public:
    virtual LEGlyphID unicodeToGlyph(LEUnicode32 unicode32) const = 0;

    virtual ~CMAPMapper();

    static CMAPMapper *createUnicodeMapper(const CMAPTable *cmap);

protected:
    CMAPMapper(const CMAPTable *cmap);

    CMAPMapper() {};

private:
    const CMAPTable *fcmap;
};

class CMAPFormat4Mapper : public CMAPMapper
{
public:
    CMAPFormat4Mapper(const CMAPTable *cmap, const CMAPFormat4Encoding *header);

    virtual ~CMAPFormat4Mapper();

    virtual LEGlyphID unicodeToGlyph(LEUnicode32 unicode32) const;

protected:
    CMAPFormat4Mapper() {};

private:
    le_uint16       fEntrySelector;
    le_uint16       fRangeShift;
    const le_uint16 *fEndCodes;
    const le_uint16 *fStartCodes;
    const le_uint16 *fIdDelta;
    const le_uint16 *fIdRangeOffset;
};

class CMAPGroupMapper : public CMAPMapper
{
public:
    CMAPGroupMapper(const CMAPTable *cmap, const CMAPGroup *groups, le_uint32 nGroups);

    virtual ~CMAPGroupMapper();

    virtual LEGlyphID unicodeToGlyph(LEUnicode32 unicode32) const;

protected:
    CMAPGroupMapper() {};

private:
    le_int32 fPower;
    le_int32 fRangeOffset;
    const CMAPGroup *fGroups;
};

inline CMAPMapper::CMAPMapper(const CMAPTable *cmap)
    : fcmap(cmap)
{
    // nothing else to do
}

inline CMAPMapper::~CMAPMapper()
{
    LE_DELETE_ARRAY(fcmap);
}

#endif

