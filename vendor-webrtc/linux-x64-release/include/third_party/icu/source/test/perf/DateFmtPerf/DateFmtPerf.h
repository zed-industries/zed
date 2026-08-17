/*
***********************************************************************
* © 2016 and later: Unicode, Inc. and others.
* License & terms of use: http://www.unicode.org/copyright.html
***********************************************************************
***********************************************************************
* Copyright (c) 2002-2016,International Business Machines
* Corporation and others.  All Rights Reserved.
***********************************************************************
***********************************************************************
*/

#ifndef _DATEFMTPERF_H
#define _DATEFMTPERF_H

#include "cmemory.h"
#include "unicode/stringpiece.h"
#include "unicode/unistr.h"
#include "unicode/uperf.h"

#include "unicode/dtitvfmt.h"
#include "unicode/utypes.h"
#include "unicode/datefmt.h"
#include "unicode/calendar.h"
#include "unicode/uclean.h"
#include "unicode/brkiter.h"
#include "unicode/numfmt.h"
#include "unicode/coll.h"
#include "util.h"

#include "datedata.h"
#include "breakdata.h"
#include "collationdata.h"

#include <stdlib.h>
#include <string.h>

#include <fstream>

#include <iostream>
using namespace std;

//  Stubs for Windows API functions when building on UNIXes.
//
#if U_PLATFORM_USES_ONLY_WIN32_API
// do nothing
#else
#define _UNICODE
typedef int DWORD;
inline int FoldStringW(DWORD dwMapFlags, const char16_t* lpSrcStr,int cchSrc, char16_t* lpDestStr,int cchDest);
#endif

class BreakItFunction : public UPerfFunction
{
private:
	int num;
	bool wordIteration;

public:

	BreakItFunction(){num = -1;}
	BreakItFunction(int a, bool b){num = a; wordIteration = b;}

	virtual void call(UErrorCode * status)
	{		
		BreakIterator* boundary;

		if(wordIteration)
		{
			for(int i = 0; i < num; i++)
			{
				boundary = BreakIterator::createWordInstance("en", *status);
				boundary->setText(str);

				int32_t start = boundary->first();
				for (int32_t end = boundary->next();
					 end != BreakIterator::DONE;
					 start = end, end = boundary->next())
				{
					printTextRange( *boundary, start, end );
				}
			}
		}

		else // character iteration
		{
			for(int i = 0; i < num; i++)
            {
				boundary = BreakIterator::createCharacterInstance(Locale::getUS(), *status);
				boundary->setText(str);

				int32_t start = boundary->first();
				for (int32_t end = boundary->next();
					 end != BreakIterator::DONE;
					 start = end, end = boundary->next())
				{
					printTextRange( *boundary, start, end );
				}
			}
		}


	}

	virtual long getOperationsPerIteration()
	{
		if(wordIteration) return 125*num;
		else return 355*num;
	}

	void printUnicodeString(const UnicodeString &s) {
		char charBuf[1000];
		s.extract(0, s.length(), charBuf, sizeof(charBuf)-1, 0);   
		charBuf[sizeof(charBuf)-1] = 0;          
		printf("%s", charBuf);
	}


	void printTextRange( BreakIterator& iterator, 
						int32_t start, int32_t end )
	{
		CharacterIterator *strIter = iterator.getText().clone();
		UnicodeString  s;
		strIter->getText(s);
		//printUnicodeString(UnicodeString(s, start, end-start));
		//puts("");
		delete strIter;
	}

	// Print the given string to stdout (for debugging purposes)
	void uprintf(const UnicodeString &str) {
		char *buf = 0;
		int32_t len = str.length();
		int32_t bufLen = len + 16;
		int32_t actualLen;
		buf = new char[bufLen + 1];
		actualLen = str.extract(0, len, buf/*, bufLen*/); // Default codepage conversion
		buf[actualLen] = 0;
		printf("%s", buf);
		delete[] buf;
	}

};

class DateFmtFunction : public UPerfFunction
{

private:
	int num;
    char locale[25];
public:
	
	DateFmtFunction()
	{
		num = -1;
	}

	DateFmtFunction(int a, const char* loc)
	{
		num = a;
        strcpy(locale, loc);
	}

	virtual void call(UErrorCode* status)
	{

		UErrorCode status2 = U_ZERO_ERROR;		
		Calendar *cal;
		TimeZone *zone;
		UnicodeString str;
		UDate date;

		cal = Calendar::createInstance(status2);
		check(status2, "Calendar::createInstance");
		zone = TimeZone::createTimeZone("GMT"); // Create a GMT zone
		cal->adoptTimeZone(zone);
		
		Locale loc(locale);
		DateFormat *fmt;
		fmt = DateFormat::createDateTimeInstance(
								DateFormat::kShort, DateFormat::kFull, loc);

		
		// (dates are imported from datedata.h)
		for(int j = 0; j < num; j++)
			for(int i = 0; i < NUM_DATES; i++)
			{
				cal->clear();
				cal->set(years[i], months[i], days[i]);
				date = cal->getTime(status2);
				check(status2, "Calendar::getTime");

				fmt->setCalendar(*cal);

				// Format the date
				str.remove();
				fmt->format(date, str, status2);

				
				// Display the formatted date string
				//uprintf(str);
				//printf("\n");
				
			}
		
		delete fmt;
		delete cal;
		//u_cleanup();
	}

	virtual long getOperationsPerIteration()
	{
		return NUM_DATES * num;
	}

	// Print the given string to stdout (for debugging purposes)
	void uprintf(const UnicodeString &str) {
		char *buf = 0;
		int32_t len = str.length();
		int32_t bufLen = len + 16;
		int32_t actualLen;
		buf = new char[bufLen + 1];
		actualLen = str.extract(0, len, buf/*, bufLen*/); // Default codepage conversion
		buf[actualLen] = 0;
		printf("%s", buf);
		delete[] buf;
	}

	// Verify that a UErrorCode is successful; exit(1) if not
	void check(UErrorCode& status, const char* msg) {
		if (U_FAILURE(status)) {
			printf("ERROR: %s (%s)\n", u_errorName(status), msg);
			exit(1);
		}
	}

};

class DateFmtCreateFunction : public UPerfFunction
{

private:
        int num;
        char locale[25];
public:

        DateFmtCreateFunction(int a, const char* loc)
        {
                num = a;
                strcpy(locale, loc);
        }

        virtual void call(UErrorCode* /* status */)
        {

                Locale loc(locale);
                DateFormat *fmt;
                // (dates are imported from datedata.h)
                for(int j = 0; j < num; j++) {
                    fmt = DateFormat::createDateTimeInstance(
                            DateFormat::kShort, DateFormat::kFull, loc);
                    delete fmt;
                }
        }

        virtual long getOperationsPerIteration()
        {
                return num;
        }

};

class DateFmtCopyFunction : public UPerfFunction
{

private:
        int num;
    char locale[25];
public:

        DateFmtCopyFunction()
        {
                num = -1;
        }

        DateFmtCopyFunction(int a, const char* loc)
        {
                num = a;
        strcpy(locale, loc);
        }

        virtual void call(UErrorCode* /* status */)
        {
                Locale loc(locale);
                UErrorCode status2 = U_ZERO_ERROR;
                DateFormat *fmt = DateFormat::createDateTimeInstance(
                            DateFormat::kShort, DateFormat::kFull, loc);
                for(int j = 0; j < num; j++) {
                    Format *cp = fmt->clone();
                    delete cp;
                }
                delete fmt;
        }

        virtual long getOperationsPerIteration()
        {
                return num;
        }

        // Verify that a UErrorCode is successful; exit(1) if not
        void check(UErrorCode& status, const char* msg) {
                if (U_FAILURE(status)) {
                        printf("ERROR: %s (%s)\n", u_errorName(status), msg);
                        exit(1);
                }
        }

};

class DIFCreateFunction : public UPerfFunction
{

private:
	int num;
    char locale[25];
public:
	
	DIFCreateFunction()
	{
		num = -1;
	}

	DIFCreateFunction(int a, const char* loc)
	{
		num = a;
        strcpy(locale, loc);
	}

	virtual void call(UErrorCode* /* status */)
	{
		UErrorCode status2 = U_ZERO_ERROR;		
		Calendar *cal;
		TimeZone *zone;

		cal = Calendar::createInstance(status2);
		check(status2, "Calendar::createInstance");
		zone = TimeZone::createTimeZone("GMT"); // Create a GMT zone
		cal->adoptTimeZone(zone);
		
		Locale loc(locale);
                UnicodeString skeleton("yMMMMdHms");

		for(int j = 0; j < num; j++) {
                    DateIntervalFormat* fmt(DateIntervalFormat::createInstance(skeleton, loc, status2));
                    delete fmt;
                }
                delete cal;
	}

	virtual long getOperationsPerIteration()
	{
		return num;
	}

	// Verify that a UErrorCode is successful; exit(1) if not
	void check(UErrorCode& status, const char* msg) {
		if (U_FAILURE(status)) {
			printf("ERROR: %s (%s)\n", u_errorName(status), msg);
			exit(1);
		}
	}

};

class TimeZoneCreateFunction : public UPerfFunction
{

private:
	int num;
    char locale[25];
public:
	
	TimeZoneCreateFunction()
	{
		num = -1;
	}

	TimeZoneCreateFunction(int a, const char* loc)
	{
		num = a;
        strcpy(locale, loc);
	}

	virtual void call(UErrorCode* /* status */)
	{
		Locale loc(locale);
                UnicodeString tzname("UTC");
		for(int j = 0; j < num; j++) {
                    TimeZone* tz(TimeZone::createTimeZone(tzname));
                    delete tz;
                }
	}

	virtual long getOperationsPerIteration()
	{
		return num;
	}

	// Verify that a UErrorCode is successful; exit(1) if not
	void check(UErrorCode& status, const char* msg) {
		if (U_FAILURE(status)) {
			printf("ERROR: %s (%s)\n", u_errorName(status), msg);
			exit(1);
		}
	}

};

class DTPatternGeneratorCreateFunction : public UPerfFunction
{

private:
	int num;
    char locale[25];
public:
	
	DTPatternGeneratorCreateFunction()
	{
		num = -1;
	}

	DTPatternGeneratorCreateFunction(int a, const char* loc)
	{
		num = a;
        strcpy(locale, loc);
	}

	virtual void call(UErrorCode* /* status */)
	{
		UErrorCode status2 = U_ZERO_ERROR;		
		Locale loc(locale);

		for(int j = 0; j < num; j++) {
                    DateTimePatternGenerator* gen(DateTimePatternGenerator::createInstance(loc, status2));
                    delete gen;
                }
	}

	virtual long getOperationsPerIteration()
	{
		return num;
	}

	// Verify that a UErrorCode is successful; exit(1) if not
	void check(UErrorCode& status, const char* msg) {
		if (U_FAILURE(status)) {
			printf("ERROR: %s (%s)\n", u_errorName(status), msg);
			exit(1);
		}
	}

};

class DTPatternGeneratorCopyFunction : public UPerfFunction
{

private:
	int num;
    char locale[25];
public:
	
	DTPatternGeneratorCopyFunction()
	{
		num = -1;
	}

	DTPatternGeneratorCopyFunction(int a, const char* loc)
	{
		num = a;
        strcpy(locale, loc);
	}

	virtual void call(UErrorCode* /* status */)
	{
		UErrorCode status2 = U_ZERO_ERROR;		
		Locale loc(locale);
                DateTimePatternGenerator* gen(DateTimePatternGenerator::createInstance(loc, status2));

		for(int j = 0; j < num; j++) {
                    DateTimePatternGenerator *cl = gen->clone();
                    delete cl;
                }
                delete gen;
	}

	virtual long getOperationsPerIteration()
	{
		return num;
	}

	// Verify that a UErrorCode is successful; exit(1) if not
	void check(UErrorCode& status, const char* msg) {
		if (U_FAILURE(status)) {
			printf("ERROR: %s (%s)\n", u_errorName(status), msg);
			exit(1);
		}
	}

};

class DTPatternGeneratorBestValueFunction : public UPerfFunction
{

private:
	int num;
    char locale[25];
public:
	
	DTPatternGeneratorBestValueFunction()
	{
		num = -1;
	}

	DTPatternGeneratorBestValueFunction(int a, const char* loc)
	{
		num = a;
        strcpy(locale, loc);
	}

	virtual void call(UErrorCode* /* status */)
	{
		UErrorCode status2 = U_ZERO_ERROR;		
		Locale loc(locale);
                DateTimePatternGenerator* gen(DateTimePatternGenerator::createInstance(loc, status2));
                UnicodeString skeleton("yMMMMdHms");

		for(int j = 0; j < num; j++) {
                    gen->getBestPattern(skeleton, status2);
                }
                check(status2, "getBestPattern");
                delete gen;
	}

	virtual long getOperationsPerIteration()
	{
		return num;
	}

	// Verify that a UErrorCode is successful; exit(1) if not
	void check(UErrorCode& status, const char* msg) {
		if (U_FAILURE(status)) {
			printf("ERROR: %s (%s)\n", u_errorName(status), msg);
			exit(1);
		}
	}

};

class NumFmtFunction : public UPerfFunction
{

private:
	int num;
    char locale[25];
public:
	
	NumFmtFunction()
	{
		num = -1;
	}

	NumFmtFunction(int a, const char* loc)
	{
		num = a;
        strcpy(locale, loc);
	}

	virtual void call(UErrorCode* status2)
	{
        Locale loc(locale);
        UErrorCode status = U_ZERO_ERROR;
        
        // Create a number formatter for the locale
        NumberFormat *fmt = NumberFormat::createInstance(loc, status);

        // Parse a string.  The string uses the digits '0' through '9'
        // and the decimal separator '.', standard in the US locale
        
        for(int i = 0; i < num; i++)
        {
            UnicodeString str("9876543210.123");
            Formattable result;
            fmt->parse(str, result, status);

            //uprintf(formattableToString(result));
            //printf("\n");

            // Take the number parsed above, and use the formatter to
            // format it.
            str.remove(); // format() will APPEND to this string
            fmt->format(result, str, status);

            //uprintf(str);
            //printf("\n");
        }

        delete fmt; // Release the storage used by the formatter
    }

    enum {
        U_SPACE=0x20,
        U_DQUOTE=0x22,
        U_COMMA=0x2c,
        U_LEFT_SQUARE_BRACKET=0x5b,
        U_BACKSLASH=0x5c,
        U_RIGHT_SQUARE_BRACKET=0x5d,
        U_SMALL_U=0x75
    };

    // Create a display string for a formattable
    UnicodeString formattableToString(const Formattable& f) {
        switch (f.getType()) {
        case Formattable::kDate:
            // TODO: Finish implementing this
            return UNICODE_STRING_SIMPLE("Formattable_DATE_TBD");
        case Formattable::kDouble:
            {
                char buf[256];
                sprintf(buf, "%gD", f.getDouble());
                return UnicodeString(buf, "");
            }
        case Formattable::kLong:
        case Formattable::kInt64:
            {
                char buf[256];
                sprintf(buf, "%ldL", f.getLong());
                return UnicodeString(buf, "");
            }
        case Formattable::kString:
            return UnicodeString((char16_t)U_DQUOTE).append(f.getString()).append((char16_t)U_DQUOTE);
        case Formattable::kArray:
            {
                int32_t i, count;
                const Formattable* array = f.getArray(count);
                UnicodeString result((char16_t)U_LEFT_SQUARE_BRACKET);
                for (i=0; i<count; ++i) {
                    if (i > 0) {
                        (result += (char16_t)U_COMMA) += (char16_t)U_SPACE;
                    }
                    result += formattableToString(array[i]);
                }
                result += (char16_t)U_RIGHT_SQUARE_BRACKET;
                return result;
            }
        default:
            return UNICODE_STRING_SIMPLE("INVALID_Formattable");
        }
    }

	virtual long getOperationsPerIteration()
	{
		return num;
	}
    
    // Print the given string to stdout using the UTF-8 converter (for debugging purposes only)
    void uprintf(const UnicodeString &str) {
        char stackBuffer[100];
        char *buf = 0;

        int32_t bufLen = str.extract(0, 0x7fffffff, stackBuffer, sizeof(stackBuffer), "UTF-8");
        if(bufLen < sizeof(stackBuffer)) {
            buf = stackBuffer;
        } else {
            buf = new char[bufLen + 1];
            bufLen = str.extract(0, 0x7fffffff, buf, bufLen + 1, "UTF-8");
        }
        printf("%s", buf);
        if(buf != stackBuffer) {
            delete[] buf;
        }
    }
};



#define NUM_STRING "9876543210.123"
#define NUM_NUM 9876543210.123
class StdioNumFmtFunction : public UPerfFunction
{

 private:
  int num;
  char locale[25];
 public:
	
  StdioNumFmtFunction()
    {
      num = -1;
    }

  StdioNumFmtFunction(int a, const char* loc)
    {
      num = a;
      strcpy(locale, loc);
    }

  virtual void call(UErrorCode* status2)
  {
    Locale loc(locale);
    UErrorCode status = U_ZERO_ERROR;
        
    // Parse a string.  The string uses the digits '0' through '9'
    // and the decimal separator '.', standard in the US locale

    double result;
    char outbuf[500];
    const char *str = NUM_STRING;
        
    for(int i = 0; i < num; i++)
      {
        if(sscanf(str, "%lg", &result)!=1) {
          cout << "Failed Stdio: failed to sscanf" << endl;
          *status2 = U_PARSE_ERROR;
          return;
        }

        sprintf(outbuf, "%lg", result);
      }
    
    if(result!=NUM_NUM) {
      cout << "Failed Stdio: sscanf got wrong result, expected " << NUM_NUM << " got " << result << endl;
      *status2 = U_PARSE_ERROR;
    }
    if(strcmp(str,NUM_STRING)) {
      cout << "Failed Stdio: sprintf got wrong result, expected " << NUM_STRING << " got " << str << endl;
      *status2 = U_PARSE_ERROR;
    }
  }
 
  virtual long getOperationsPerIteration()
  {
    return num;
  }
    
};

class CollationFunction : public UPerfFunction
{

private:
	int num;
    char locale[25];
	UnicodeString *collation_strings;

	/**
	 * Unescape the strings
	 */
	void init() {
        uint32_t listSize = UPRV_LENGTHOF(collation_strings_escaped);
		collation_strings = new UnicodeString[listSize];
		for(uint32_t k=0;k<listSize;k++) {
			collation_strings[k] = collation_strings_escaped[k].unescape();
		}
		UnicodeString shorty((UChar32)0x12345);
	}
public:
	
	CollationFunction()
	{
		num = -1;

		init();
	}

	~CollationFunction() {
		delete [] collation_strings;
	}

	CollationFunction(int a, const char* loc)
	{
		num = a;
        strcpy(locale, loc);
		init();
	}

	virtual void call(UErrorCode* status2)
	{
        uint32_t listSize = UPRV_LENGTHOF(collation_strings_escaped);
        UErrorCode status = U_ZERO_ERROR; 
        Collator *coll = Collator::createInstance(Locale(locale), status);
        
        for(int k = 0; k < num; k++)
        {
            uint32_t i, j;
            for(i=listSize-1; i>=1; i--) {
                for(j=0; j<i; j++) {
                    if(coll->compare(collation_strings[j], collation_strings[j+1]) == UCOL_LESS) {
                    //cout << "Success!" << endl;
                     }
                }
            }
         }
        delete coll; 
    }

	virtual long getOperationsPerIteration()
	{
		return num;
	}
};

class DateFormatPerfTest : public UPerfTest
{
private:

public:

	DateFormatPerfTest(int32_t argc, const char* argv[], UErrorCode& status);
	~DateFormatPerfTest();
	virtual UPerfFunction* runIndexedTest(int32_t index, UBool exec,const char* &name, char* par);

	UPerfFunction* DateFmt250();
	UPerfFunction* DateFmt10000();
	UPerfFunction* DateFmt100000();
	UPerfFunction* DateFmtCreate250();
	UPerfFunction* DateFmtCreate10000();
	UPerfFunction* DateFmtCopy250();
	UPerfFunction* DateFmtCopy10000();
	UPerfFunction* BreakItWord250();
	UPerfFunction* BreakItWord10000();
	UPerfFunction* BreakItChar250();
	UPerfFunction* BreakItChar10000();
    UPerfFunction* NumFmt10000();
    UPerfFunction* NumFmt100000();
    UPerfFunction* Collation10000();
    UPerfFunction* Collation100000();
    UPerfFunction* DIFCreate250();
    UPerfFunction* DIFCreate10000();
    UPerfFunction* TimeZoneCreate250();
    UPerfFunction* TimeZoneCreate10000();
    UPerfFunction* DTPatternGeneratorCreate250();
    UPerfFunction* DTPatternGeneratorCreate10000();
    UPerfFunction* DTPatternGeneratorCopy250();
    UPerfFunction* DTPatternGeneratorCopy10000();
    UPerfFunction* DTPatternGeneratorBestValue250();
    UPerfFunction* DTPatternGeneratorBestValue10000();
};

#endif // DateFmtPerf
