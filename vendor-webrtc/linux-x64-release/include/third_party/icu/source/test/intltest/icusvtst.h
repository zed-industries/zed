// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/**
 *******************************************************************************
 * Copyright (C) 2001-2003, International Business Machines Corporation and    *
 * others. All Rights Reserved.                                                *
 *******************************************************************************
 *
 *******************************************************************************
 */

#ifndef ICUSVTST_H
#define ICUSVTST_H

#include "unicode/utypes.h"

#if !UCONFIG_NO_SERVICE

#include "intltest.h"

class Integer;

class ICUServiceTest : public IntlTest
{    
 public:
  ICUServiceTest();
  virtual ~ICUServiceTest();

  void runIndexedTest(int32_t index, UBool exec, const char* &name, char* par = nullptr) override;

  void testAPI_One();
  void testAPI_Two();
  void testRBF();
  void testNotification();
  void testLocale();
  void testWrapFactory();
  void testCoverage();

 private:
  UnicodeString& lrmsg(UnicodeString& result, const UnicodeString& message, const UObject* lhs, const UObject* rhs) const;
  void confirmBoolean(const UnicodeString& message, UBool val);
#if 0
  void confirmEqual(const UnicodeString& message, const UObject* lhs, const UObject* rhs);
#else
  void confirmEqual(const UnicodeString& message, const Integer* lhs, const Integer* rhs);
  void confirmEqual(const UnicodeString& message, const UnicodeString* lhs, const UnicodeString* rhs);
  void confirmEqual(const UnicodeString& message, const Locale* lhs, const Locale* rhs);
#endif
  void confirmStringsEqual(const UnicodeString& message, const UnicodeString& lhs, const UnicodeString& rhs);
  void confirmIdentical(const UnicodeString& message, const UObject* lhs, const UObject* rhs);
  void confirmIdentical(const UnicodeString& message, int32_t lhs, int32_t rhs);

  void msgstr(const UnicodeString& message, UObject* obj, UBool err = true);
  void logstr(const UnicodeString& message, UObject* obj) {
        msgstr(message, obj, false);
  }
};


/* UCONFIG_NO_SERVICE */
#endif

/* ICUSVTST_H */
#endif
