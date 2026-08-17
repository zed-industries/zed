// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2013, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/
/********************************************************************************
*
* File CINTLTST.H
*
*     Madhu Katragadda               Creation
* Modification History:
*   Date        Name        Description            
*   07/13/99    helena      HPUX 11 CC port.
*********************************************************************************

The main root for C API tests
*/

#ifndef _CINTLTST
#define _CINTLTST

#include "unicode/utypes.h"
#include "unicode/putil.h"
#include "unicode/ctest.h"

#include <stdlib.h>

#ifndef U_USE_DEPRECATED_API
#define U_USE_DEPRECATED_API 1
#endif

U_CFUNC void addAllTests(TestNode** root);

/**
 * Return the path to the icu/source/data/out  directory 
 */
U_CFUNC const char* ctest_dataOutDir(void);

/**
 * Return the path to the icu/source/data/  directory 
 * for out of source builds too returns the source directory
 */
U_CFUNC const char* ctest_dataSrcDir(void);

/**
 * Convert a char string into a UChar string, with unescaping
 * The result buffer has been malloc()'ed (not ctst_malloc) and needs to be free()'ed by the caller.
 */
U_CFUNC UChar* CharsToUChars(const char* chars);

/**
 * Convert a const UChar* into a char*
 * Result is allocated with ctst_malloc and will be freed at the end of the test.
 * @param unichars UChars (null terminated) to be converted
 * @return new char* to the unichars in host format
 */
 
U_CFUNC char *austrdup(const UChar* unichars);

/**
 * Convert a const UChar* into an escaped char*
 * Result is allocated with ctst_malloc and will be freed at the end of the test.
 * @param unichars UChars to be converted
 * @param length length of chars
 * @return new char* to the unichars in host format
 */
U_CFUNC char *aescstrdup(const UChar* unichars, int32_t length);

/**
 * Special memory allocation function for test use. At the end of cintltst, 
 * or every few thousand allocations, memory allocated by this function will be freed.
 * Do not manually free memory returned by this function, and do not retain a pointer
 * outside of a single instruction scope (i.e. long enough to display the value).
 * @see #ctst_freeAll
 */
U_CFUNC void *ctst_malloc(size_t size);

/**
 * Return the path to cintltst's data ( icu/source/data/testdata ) directory.
 * The path may be in the out/ directory.
 * Return value is allocated by ctst_malloc and should not be deleted.
 */
U_CFUNC const char* loadTestData(UErrorCode* err);

/*
 * Returns the path to the icu/source/test/testdata directory.
 * The path is always the source directory.
 * Return value is static and should not be deleted.
 */
U_CFUNC const char* loadSourceTestData(UErrorCode* err);

/**
 * function used to specify the error
 * converts the errorcode to an error descriptive string(const char*)
 * @param status the error code
 */
#define myErrorName(errorCode) u_errorName(errorCode)


/**
 * Call this once to get a consistent timezone. Use ctest_resetTimeZone to set it back to the original value.
 * @param optionalTimeZone Set this to a requested timezone.
 *      Set to NULL to use the standard test timezone (Pacific Time)
 */
U_CFUNC void ctest_setTimeZone(const char *optionalTimeZone, UErrorCode *status);
/**
 * Call this once get back the original timezone
 */
U_CFUNC void ctest_resetTimeZone(void);

/**
 * Call this once get ICU back to its original state with test arguments.
 * This function calls u_cleanup.
 */
U_CFUNC UBool ctest_resetICU(void);

/**
 * Assert that the given UErrorCode succeeds, and return true if it does.
 */
U_CFUNC UBool assertSuccess(const char* msg, UErrorCode* ec);

/**
 * Assert that the given UErrorCode succeeds, and return true if it does.
 * Give data error if UErrorCode fails and possibleDataError is true.
 */
U_CFUNC UBool assertSuccessCheck(const char* msg, UErrorCode* ec, UBool possibleDataError);

/**
 * Assert that the UBool is true, and return true if it does.
 *
 * NOTE: Use 'int condition' rather than 'UBool condition' so the
 * compiler doesn't complain about integral conversion of expressions
 * like 'p != 0'.
 */
U_CFUNC UBool assertTrue(const char* msg, int condition);

/**
 * Assert that the actualString equals the expectedString, and return
 * true if it does.
 */
U_CFUNC UBool assertEquals(const char* msg, const char* expectedString,
                           const char* actualString);

/**
 * Assert that the actualString equals the expectedString, and return
 * true if it does.
 */
U_CFUNC UBool assertUEquals(const char* msg, const UChar* expectedString,
                            const UChar* actualString);

/**
 * Assert that two 64-bit integers are equal, returning true if they are.
 */
U_CFUNC UBool assertIntEquals(const char* msg, int64_t expected, int64_t actual);

/**
 * Assert that the addresses of the two pointers are the same, returning
 * true if they are equal.
 */
U_CFUNC UBool assertPtrEquals(const char* msg, const void* expected, const void* actual);

/**
 * Assert that two doubles are equal, returning true if they are.
 */
U_CFUNC UBool assertDoubleEquals(const char *msg, double expected, double actual);

/*
 * note - isICUVersionBefore and isICUVersionAtLeast have been removed.
 * use log_knownIssue() instead.
 */

#endif
