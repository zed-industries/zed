/*
 *******************************************************************************
 * © 2016 and later: Unicode, Inc. and others.
 * License & terms of use: http://www.unicode.org/copyright.html
 *******************************************************************************
 ******************************************************************************
 * Copyright (C) 1998-2006, International Business Machines Corporation and   *
 * others. All Rights Reserved.                                               *
 ******************************************************************************
 */

#ifndef __GNOMEFONTMAP_H
#define __GNOMEFONTMAP_H

#include <ft2build.h>
#include FT_FREETYPE_H

#include "unicode/uscript.h"

#include "layout/LETypes.h"
#include "layout/LEFontInstance.h"

#include "GUISupport.h"
#include "FontMap.h"

#define BUFFER_SIZE 128

class GnomeFontMap : public FontMap
{
 public:
    GnomeFontMap(FT_Library engine, const char *fileName, le_int16 pointSize, GUISupport *guiSupport, LEErrorCode &status);

    virtual ~GnomeFontMap();

 protected:
    virtual const LEFontInstance *openFont(const char *fontName, le_int16 pointSize, LEErrorCode &status);

 private:
    FT_Library fEngine;
};

#endif
