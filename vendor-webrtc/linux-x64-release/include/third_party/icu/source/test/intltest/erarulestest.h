// © 2018 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html

#ifndef ERARULESTEST_H_
#define ERARULESTEST_H_

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "intltest.h"

class EraRulesTest : public IntlTest {
public:
    void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par = nullptr) override;

private:
    void testAPIs();
    void testJapanese();
};

#endif /* #if !UCONFIG_NO_FORMATTING */
#endif /* ERARULESTEST_H_ */
