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

#ifndef __GDIFONTMAP_H
#define __GDIFONTMAP_H

#include <windows.h>

#include "unicode/uscript.h"

#include "layout/LETypes.h"
#include "layout/LEFontInstance.h"

#include "FontMap.h"
#include "GUISupport.h"
#include "GDIFontInstance.h"

#define BUFFER_SIZE 128

class GDIFontMap : public FontMap
{
public:
    GDIFontMap(GDISurface *surface, const char *fileName, le_int16 pointSize, GUISupport *guiSupport, LEErrorCode &status);

    virtual ~GDIFontMap();

protected:
    virtual const LEFontInstance *openFont(const char *fontName, le_int16 pointSize, LEErrorCode &status);

private:
    GDISurface *fSurface;
};

#endif
