// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 2003-2008, International Business Machines Corporation 
 * and others. All Rights Reserved.
 ********************************************************************
 * Calendar Case Test is a type of CalendarTest which compares the 
 * behavior of a calendar to a certain set of 'test cases', involving
 * conversion between julian-day to fields and vice versa.
 ********************************************************************/

#ifndef __CalendarCaseTest__
#define __CalendarCaseTest__
 
#include "unicode/utypes.h"

#if !UCONFIG_NO_FORMATTING

#include "unicode/calendar.h"
#include "unicode/smpdtfmt.h"
#include "caltest.h"

class CalendarCaseTest: public CalendarTest {
 public:
  virtual void runIndexedTest( int32_t index, UBool exec, const char* &name, char* par ) override;

  /* Test case struct */
  struct TestCase {
    double julian; // Julian Date
    int32_t era;
    int32_t year;
    int32_t month;
    int32_t day;
    int32_t dayOfWeek;
    int32_t hour;
    int32_t min;
    int32_t sec;
  };
  
  /**
   * @param cases array of items to test.  Terminate with a "-1" for era.
   */
  void doTestCases(const TestCase *cases, Calendar *cal);

 private:
  /**
   * Utility function to test out a specific field
   * @param cal calendar
   * @param field which field
   * @param value expected value
   * @param status err status 
   * @return boolean indicating success (true) or failure (false) of the test.
   */
  UBool checkField(Calendar *cal, UCalendarDateFields field, int32_t value, UErrorCode &status);

 private:
  // test cases
  void IslamicCivil();
  void Hebrew();
  void Indian();
  void Coptic();
  void Ethiopic();
};

#endif
#endif
