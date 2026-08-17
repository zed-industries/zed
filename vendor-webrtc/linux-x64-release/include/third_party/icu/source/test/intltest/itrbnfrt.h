// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
 *******************************************************************************
 * Copyright (C) 1996-2006, International Business Machines Corporation and    *
 * others. All Rights Reserved.                                                *
 *******************************************************************************
 */

#ifndef ITRBNFRT_H
#define ITRBNFRT_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "intltest.h"
#include "unicode/rbnf.h"

class RbnfRoundTripTest : public IntlTest {

  // IntlTest override
  virtual void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par) override;

#if U_HAVE_RBNF
  /**
   * Perform an exhaustive round-trip test on the English spellout rules
   */
  virtual void TestEnglishSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the duration-formatting rules
   */
  virtual void TestDurationsRT();

  /**
   * Perform an exhaustive round-trip test on the Spanish spellout rules
   */
  virtual void TestSpanishSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the French spellout rules
   */
  virtual void TestFrenchSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the Swiss French spellout rules
   */
  virtual void TestSwissFrenchSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the Italian spellout rules
   */
  virtual void TestItalianSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the German spellout rules
   */
  virtual void TestGermanSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the Swedish spellout rules
   */
  virtual void TestSwedishSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the Dutch spellout rules
   */
  virtual void TestDutchSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the Japanese spellout rules
   */
  virtual void TestJapaneseSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the Russian spellout rules
   */
  virtual void TestRussianSpelloutRT();

  /**
   * Perform an exhaustive round-trip test on the Portuguese spellout rules
   */
  virtual void TestPortugueseSpelloutRT();

 protected:
  void doTest(const RuleBasedNumberFormat* formatter,  double lowLimit, double highLimit);

  /* U_HAVE_RBNF */
#else

  void TestRBNFDisabled();

  /* U_HAVE_RBNF */
#endif
};

#endif /* #if !UCONFIG_NO_FORMATTING */

// endif ITRBNFRT_H
#endif
