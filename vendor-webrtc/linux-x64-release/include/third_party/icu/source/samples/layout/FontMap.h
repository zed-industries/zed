/*
 ******************************************************************************
 * © 2016 and later: Unicode, Inc. and others.                    *
 * License & terms of use: http://www.unicode.org/copyright.html      *
 ******************************************************************************
 ******************************************************************************
 * Copyright (C) 1998-2003, International Business Machines Corporation and   *
 * others. All Rights Reserved.                                               *
 ******************************************************************************
 */

#ifndef __FONTMAP_H
#define __FONTMAP_H

#include "layout/LETypes.h"
#include "layout/LEScripts.h"
#include "layout/LEFontInstance.h"

#include "GUISupport.h"

#define BUFFER_SIZE 128

class FontMap
{
public:
    FontMap(const char *fileName, le_int16 pointSize, GUISupport *guiSupport, LEErrorCode &status);

    virtual ~FontMap();

    virtual const LEFontInstance *getScriptFont(le_int32 scriptCode, LEErrorCode &status);

    virtual le_int16 getPointSize() const;

    virtual le_int32 getAscent() const;

    virtual le_int32 getDescent() const;

    virtual le_int32 getLeading() const;

protected:
    virtual const LEFontInstance *openFont(const char *fontName, le_int16 pointSize, LEErrorCode &status) = 0;

    char errorMessage[256];

private:
    static char *strip(char *s);
    le_int32 getFontIndex(const char *fontName);
    void getMaxMetrics();

    le_int16 fPointSize;
    le_int32 fFontCount;

    le_int32 fAscent;
    le_int32 fDescent;
    le_int32 fLeading;

    GUISupport *fGUISupport;

    const LEFontInstance *fFontInstances[scriptCodeCount];
    const char *fFontNames[scriptCodeCount];
    le_int32 fFontIndices[scriptCodeCount];
};

inline le_int16 FontMap::getPointSize() const
{
    return fPointSize;
}

#endif

