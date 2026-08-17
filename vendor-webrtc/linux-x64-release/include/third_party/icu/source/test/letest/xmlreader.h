// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 *
 * (C) Copyright IBM Corp. 1998-2014 - All Rights Reserved
 *
 */

#ifndef __XMLREADER_H
#define __XMLREADER_H

#include "layout/LETypes.h"
#include "letest.h"

typedef void (*TestCaseCallback) (const char *testID,
								  const char *fontName,
								  const char *fontVersion,
								  const char *fontChecksum,
								  le_int32 scriptCode,
								  le_int32 languageCode,
								  const LEUnicode *text,
								  le_int32 charCount,
								  TestResult *expected);

U_CAPI void readTestFile(const char *testFilePath, TestCaseCallback callback);

#endif
