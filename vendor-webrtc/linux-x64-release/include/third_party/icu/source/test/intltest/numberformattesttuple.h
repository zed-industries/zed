// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
*******************************************************************************
* Copyright (C) 2015, International Business Machines Corporation and         *
* others. All Rights Reserved.                                                *
*******************************************************************************
*/
#ifndef _NUMBER_FORMAT_TEST_TUPLE
#define _NUMBER_FORMAT_TEST_TUPLE

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/decimfmt.h"
#include "unicode/ucurr.h"

#define NFTT_GET_FIELD(tuple, fieldName, defaultValue) ((tuple).fieldName##Flag ? (tuple).fieldName : (defaultValue))

U_NAMESPACE_USE

enum ENumberFormatTestTupleField {
    kLocale,
    kCurrency,
    kPattern,
    kFormat,
    kOutput,
    kComment,
    kMinIntegerDigits,
    kMaxIntegerDigits,
    kMinFractionDigits,
    kMaxFractionDigits,
    kMinGroupingDigits,
    kBreaks,
    kUseSigDigits,
    kMinSigDigits,
    kMaxSigDigits,
    kUseGrouping,
    kMultiplier,
    kRoundingIncrement,
    kFormatWidth,
    kPadCharacter,
    kUseScientific,
    kGrouping,
    kGrouping2,
    kRoundingMode,
    kCurrencyUsage,
    kMinimumExponentDigits,
    kExponentSignAlwaysShown,
    kDecimalSeparatorAlwaysShown,
    kPadPosition,
    kPositivePrefix,
    kPositiveSuffix,
    kNegativePrefix,
    kNegativeSuffix,
    kSignAlwaysShown,
    kLocalizedPattern,
    kToPattern,
    kToLocalizedPattern,
    kStyle,
    kParse,
    kLenient,
    kPlural,
    kParseIntegerOnly,
    kDecimalPatternMatchRequired,
    kParseCaseSensitive,
    kParseNoExponent,
    kOutputCurrency,
    kNumberFormatTestTupleFieldCount
};

/**
 * NumberFormatTestTuple represents the data for a single data driven test.
 * It consist of named fields each of which may or may not be set. Each field
 * has a particular meaning in the test. For more information on what each
 * field means and how the data drive tests work, please see
 * https://docs.google.com/document/d/1T2P0p953_Lh1pRwo-5CuPVrHlIBa_wcXElG-Hhg_WHM/edit?usp=sharing
 * Each field is optional. That is, a certain field may be unset for a given
 * test. The UBool fields ending in "Flag" indicate whether the corresponding
 * field is set or not. true means set; false means unset. An unset field
 * generally means that the corresponding setter method is not called on
 * the NumberFormat object.
 */

class NumberFormatTestTuple {
public:
    Locale locale;
    UnicodeString currency;
    UnicodeString pattern;
    UnicodeString format;
    UnicodeString output;
    UnicodeString comment;
    int32_t minIntegerDigits;
    int32_t maxIntegerDigits;
    int32_t minFractionDigits;
    int32_t maxFractionDigits;
    int32_t minGroupingDigits;
    UnicodeString breaks;
    int32_t useSigDigits;
    int32_t minSigDigits;
    int32_t maxSigDigits;
    int32_t useGrouping;
    int32_t multiplier;
    double roundingIncrement;
    int32_t formatWidth;
    UnicodeString padCharacter;
    int32_t useScientific;
    int32_t grouping;
    int32_t grouping2;
    DecimalFormat::ERoundingMode roundingMode;
    UCurrencyUsage currencyUsage;
    int32_t minimumExponentDigits;
    int32_t exponentSignAlwaysShown;
    int32_t decimalSeparatorAlwaysShown;
    DecimalFormat::EPadPosition padPosition;
    UnicodeString positivePrefix;
    UnicodeString positiveSuffix;
    UnicodeString negativePrefix;
    UnicodeString negativeSuffix;
    int32_t signAlwaysShown;
    UnicodeString localizedPattern;
    UnicodeString toPattern;
    UnicodeString toLocalizedPattern;
    UNumberFormatStyle style;
    UnicodeString parse;
    int32_t lenient;
    UnicodeString plural;
    int32_t parseIntegerOnly;
    int32_t decimalPatternMatchRequired;
    int32_t parseNoExponent;
    int32_t parseCaseSensitive;
    UnicodeString outputCurrency;

    UBool localeFlag;
    UBool currencyFlag;
    UBool patternFlag;
    UBool formatFlag;
    UBool outputFlag;
    UBool commentFlag;
    UBool minIntegerDigitsFlag;
    UBool maxIntegerDigitsFlag;
    UBool minFractionDigitsFlag;
    UBool maxFractionDigitsFlag;
    UBool minGroupingDigitsFlag;
    UBool breaksFlag;
    UBool useSigDigitsFlag;
    UBool minSigDigitsFlag;
    UBool maxSigDigitsFlag;
    UBool useGroupingFlag;
    UBool multiplierFlag;
    UBool roundingIncrementFlag;
    UBool formatWidthFlag;
    UBool padCharacterFlag;
    UBool useScientificFlag;
    UBool groupingFlag;
    UBool grouping2Flag;
    UBool roundingModeFlag;
    UBool currencyUsageFlag;
    UBool minimumExponentDigitsFlag;
    UBool exponentSignAlwaysShownFlag;
    UBool decimalSeparatorAlwaysShownFlag;
    UBool padPositionFlag;
    UBool positivePrefixFlag;
    UBool positiveSuffixFlag;
    UBool negativePrefixFlag;
    UBool negativeSuffixFlag;
    UBool signAlwaysShownFlag;
    UBool localizedPatternFlag;
    UBool toPatternFlag;
    UBool toLocalizedPatternFlag;
    UBool styleFlag;
    UBool parseFlag;
    UBool lenientFlag;
    UBool pluralFlag;
    UBool parseIntegerOnlyFlag;
    UBool decimalPatternMatchRequiredFlag;
    UBool parseNoExponentFlag;
    UBool parseCaseSensitiveFlag;
    UBool outputCurrencyFlag;

    NumberFormatTestTuple() {
        clear();
    }

    /**
     * Sets a particular field using the string representation of that field.
     * @param field the field to set.
     * @param fieldValue the string representation of the field value.
     * @param status error returned here such as when the string representation
     *  of the field value cannot be parsed.
     * @return true on success or false if an error was set in status.
     */
    UBool setField(
            ENumberFormatTestTupleField field,
            const UnicodeString &fieldValue,
            UErrorCode &status);
    /**
     * Clears a particular field.
     * @param field the field to clear.
     * @param status error set here.
     * @return true on success or false if error was set.
     */
    UBool clearField(
            ENumberFormatTestTupleField field,
            UErrorCode &status);
    /**
     * Clears every field.
     */
    void clear();

    /**
     * Returns the string representation of the test case this object
     * currently represents.
     * @param appendTo the result appended here.
     * @return appendTo
     */
    UnicodeString &toString(UnicodeString &appendTo) const;

    /**
     * Converts the name of a field to the corresponding enum value.
     * @param name the name of the field as a string.
     * @return the corresponding enum value or kNumberFormatTestFieldCount
     *   if name does not map to any recognized field name.
     */
    static ENumberFormatTestTupleField getFieldByName(const UnicodeString &name);
private:
    const void *getFieldAddress(int32_t fieldId) const;
    void *getMutableFieldAddress(int32_t fieldId);
    void setFlag(int32_t fieldId, UBool value);
    UBool isFlag(int32_t fieldId) const;
};

#endif /* !UCONFIG_NO_FORMATTING */
#endif
