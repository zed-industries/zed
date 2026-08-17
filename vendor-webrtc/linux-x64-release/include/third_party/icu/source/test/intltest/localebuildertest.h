// © 2018 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html

#include "intltest.h"
#include "unicode/localebuilder.h"


/**
 * Tests for the LocaleBuilder class
 **/
class LocaleBuilderTest: public IntlTest {
 public:
    LocaleBuilderTest();
    virtual ~LocaleBuilderTest();

    void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par = nullptr ) override;

    void TestAddRemoveUnicodeLocaleAttribute();
    void TestAddRemoveUnicodeLocaleAttributeWellFormed();
    void TestAddUnicodeLocaleAttributeIllFormed();
    void TestLocaleBuilder();
    void TestLocaleBuilderBasic();
    void TestLocaleBuilderBasicWithExtensionsOnDefaultLocale();
    void TestPosixCases();
    void TestSetExtensionOthers();
    void TestSetExtensionPU();
    void TestSetExtensionT();
    void TestSetExtensionU();
    void TestSetExtensionValidateOthersIllFormed();
    void TestSetExtensionValidateOthersWellFormed();
    void TestSetExtensionValidatePUIllFormed();
    void TestSetExtensionValidatePUWellFormed();
    void TestSetExtensionValidateTIllFormed();
    void TestSetExtensionValidateTWellFormed();
    void TestSetExtensionValidateUIllFormed();
    void TestSetExtensionValidateUWellFormed();
    void TestSetLanguageIllFormed();
    void TestSetLanguageWellFormed();
    void TestSetLocale();
    void TestSetRegionIllFormed();
    void TestSetRegionWellFormed();
    void TestSetScriptIllFormed();
    void TestSetScriptWellFormed();
    void TestSetUnicodeLocaleKeywordIllFormedKey();
    void TestSetUnicodeLocaleKeywordIllFormedValue();
    void TestSetUnicodeLocaleKeywordWellFormed();
    void TestSetVariantIllFormed();
    void TestSetVariantWellFormed();

 private:
    void Verify(LocaleBuilder& bld, const char* expected, const char* msg);
};
