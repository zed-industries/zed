// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 *
 * (C) Copyright IBM Corp. 1998-2014 - All Rights Reserved
 *
 */

#ifndef USING_ICULEHB /* C API not available under HB */

#ifndef __CFONTS_H
#define __CFONTS_H

#include "LETypes.h"
#include "loengine.h"

le_font *le_portableFontOpen(const char *fileName,
							float pointSize,
							LEErrorCode *status);

le_font *le_simpleFontOpen(float pointSize,
						   LEErrorCode *status);

void le_fontClose(le_font *font);

const char *le_getNameString(le_font *font, le_uint16 nameID, le_uint16 platform, le_uint16 encoding, le_uint16 language);

const LEUnicode16 *le_getUnicodeNameString(le_font *font, le_uint16 nameID, le_uint16 platform, le_uint16 encoding, le_uint16 language);

void le_deleteNameString(le_font *font, const char *name);

void le_deleteUnicodeNameString(le_font *font, const LEUnicode16 *name);

le_uint32 le_getFontChecksum(le_font *font);

#endif
#endif
