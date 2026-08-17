// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
************************************************************************
* Copyright (c) 1997-2003, International Business Machines
* Corporation and others.  All Rights Reserved.
************************************************************************
*/

#ifndef _NORMCONF
#define _NORMCONF

#include "unicode/utypes.h"

#if !UCONFIG_NO_NORMALIZATION

#include "unicode/normalizer2.h"
#include "unicode/normlzr.h"
#include "intltest.h"

typedef struct _FileStream FileStream;

class NormalizerConformanceTest : public IntlTest {
    Normalizer normalizer;
    const Normalizer2 *nfc, *nfd, *nfkc, *nfkd;

 public:
    NormalizerConformanceTest();
    virtual ~NormalizerConformanceTest();

    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par=nullptr) override;

    /**
     * Test the conformance of Normalizer to
     * http://www.unicode.org/Public/UNIDATA/NormalizationTest.txt
     */
    void TestConformance();
    void TestConformance32();
    void TestConformance(FileStream *input, int32_t options);

    // Specific tests for debugging.  These are generally failures taken from
    // the conformance file, but culled out to make debugging easier.
    void TestCase6();

 private:
    FileStream *openNormalizationTestFile(const char *filename);

    /**
     * Verify the conformance of the given line of the Unicode
     * normalization (UTR 15) test suite file.  For each line,
     * there are five columns, corresponding to field[0]..field[4].
     *
     * The following invariants must be true for all conformant implementations
     *  c2 == NFC(c1) == NFC(c2) == NFC(c3)
     *  c3 == NFD(c1) == NFD(c2) == NFD(c3)
     *  c4 == NFKC(c1) == NFKC(c2) == NFKC(c3) == NFKC(c4) == NFKC(c5)
     *  c5 == NFKD(c1) == NFKD(c2) == NFKD(c3) == NFKD(c4) == NFKD(c5)
     *
     * @param field the 5 columns
     * @param line the source line from the test suite file
     * @return true if the test passes
     */
    UBool checkConformance(const UnicodeString* field,
                           const char *line,
                           int32_t options,
                           UErrorCode &status);

    UBool checkNorm(UNormalizationMode mode, int32_t options,
                    const Normalizer2 *norm2,
                    const UnicodeString &s, const UnicodeString &exp,
                    int32_t field);

    void iterativeNorm(const UnicodeString& str,
                       UNormalizationMode mode, int32_t options,
                       UnicodeString& result,
                       int8_t dir);

    /**
     * @param op name of normalization form, e.g., "KC"
     * @param op2 name of test case variant, e.g., "(-1)"
     * @param s string being normalized
     * @param got value received
     * @param exp expected value
     * @param msg description of this test
     * @param return true if got == exp
     */
    UBool assertEqual(const char *op, const char *op2,
                      const UnicodeString& s,
                      const UnicodeString& got,
                      const UnicodeString& exp,
                      const char *msg);

    /**
     * Split a string into pieces based on the given delimiter
     * character.  Then, parse the resultant fields from hex into
     * characters.  That is, "0040 0400;0C00;0899" -> new String[] {
     * "\u0040\u0400", "\u0C00", "\u0899" }.  The output is assumed to
     * be of the proper length already, and exactly output.length
     * fields are parsed.  If there are too few an exception is
     * thrown.  If there are too many the extras are ignored.
     *
     * @param buf scratch buffer
     * @return false upon failure
     */
    UBool hexsplit(const char *s, char delimiter,
                   UnicodeString output[], int32_t outputLength);

    void _testOneLine(const char *line);
    void compare(const UnicodeString& s1,const UnicodeString& s2);
};

#endif /* #if !UCONFIG_NO_NORMALIZATION */

#endif
