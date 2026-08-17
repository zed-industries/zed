// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/************************************************************************
 * COPYRIGHT:
 * Copyright (c) 2015, International Business Machines Corporation
 * and others. All Rights Reserved.
 ************************************************************************/

#ifndef _DATADRIVENNUMBERFORMATTESTSUITE_H__
#define _DATADRIVENNUMBERFORMATTESTSUITE_H__

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/uobject.h"
#include "unicode/unistr.h"
#include "numberformattesttuple.h"
#include "intltest.h"
#include "cmemory.h"

struct UCHARBUF;
class IntlTest;

/**
 * Performs various in-depth test on NumberFormat
 **/
class DataDrivenNumberFormatTestSuite : public IntlTest {

 public:
     DataDrivenNumberFormatTestSuite() {
         for (int32_t i = 0; i < UPRV_LENGTHOF(fPreviousFormatters); ++i) {
             fPreviousFormatters[i] = nullptr;
         }
     }

     /**
      * Runs the data driven test suite.
      *
      * @param fileName is the name of the file in the source/test/testdata.
      *  This should be just a filename such as "numberformattest.txt"
      * @param runAllTests If true, runs every test in fileName. if false,
      *  skips the tests that are known to break for ICU4C.
      */
     void run(const char *fileName, UBool runAllTests);
     virtual ~DataDrivenNumberFormatTestSuite();
 protected:
    /**
     * Subclasses override this method to test formatting numbers.
     * Subclasses must not override both isFormatPass methods.
     * @param tuple the test data for current test. The format method can
     *   assume that the format and output fields are populated.
     * @param appendErrorMessage any message describing failures appended
     *   here.
     * @param status any error returned here.
     * @return true if test passed or false if test failed.
     */
    virtual UBool isFormatPass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status);

    
    /**
     * Subclasses override this method to test formatting numbers.
     * Along with copy and assignment operators.
     * @param tuple the test data for current test. The format method can
     *   assume that the format and output fields are populated.
     * @param somePreviousFormatter A pointer to a previous formatter
     *  that the test framework owns. This formatter changes as tests
     *  are run. Subclasses should initialize a formatter and assign
     *  the newly initialized formatter to this formatter. In this way,
     *  assignment gets tested with multiple previous states.
     * @param appendErrorMessage any message describing failures appended
     *   here.
     * @param status any error returned here.
     * @return true if test passed or false if test failed.
     */
    virtual UBool isFormatPass(
            const NumberFormatTestTuple &tuple,
            UObject *somePreviousFormatter,
            UnicodeString &appendErrorMessage,
            UErrorCode &status);
    /**
     * If subclass is testing formatting with copy and assignment, it
     * needs to override this method to return a newly allocated formatter.
     */
    virtual UObject *newFormatter(UErrorCode &status);

    /**
     * Tests toPattern method.
     */
    virtual UBool isToPatternPass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status);
    /**
     * Test parsing.
     */
    virtual UBool isParsePass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status);

    /**
     * Test parsing with currency.
     */
    virtual UBool isParseCurrencyPass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status);

    /**
     * Test plural selection.
     */
    virtual UBool isSelectPass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status);
 private:
    UnicodeString fFileLine;
    int32_t fFileLineNumber;
    UnicodeString fFileTestName;
    NumberFormatTestTuple fTuple;
    int32_t fFormatTestNumber;
    UObject *fPreviousFormatters[13];

    void setTupleField(UErrorCode &);
    int32_t splitBy(
            UnicodeString *columnValues,
            int32_t columnValueCount,
            char16_t delimiter);
    void showError(const char *message);
    void showFailure(const UnicodeString &message);
    void showLineInfo();
    UBool breaksC();
    UBool readLine(UCHARBUF *f, UErrorCode &);
    UBool isPass(
            const NumberFormatTestTuple &tuple,
            UnicodeString &appendErrorMessage,
            UErrorCode &status);
};
#endif /* !UCONFIG_NO_FORMATTING */
#endif // _DATADRIVENNUMBERFORMATTESTSUITE_
