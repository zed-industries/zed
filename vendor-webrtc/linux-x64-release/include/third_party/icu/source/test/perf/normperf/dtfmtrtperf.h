// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/*
**********************************************************************
* Copyright (c) 2010-2016,International Business Machines
* Corporation and others.  All Rights Reserved.
**********************************************************************
**********************************************************************
*/

#ifndef _DTFMTRTPERF_H
#define _DTFMTRTPERF_H

#include "unicode/utypes.h"
#include "unicode/uperf.h"
#include "unicode/timezone.h"
#include "unicode/simpletz.h"
#include "unicode/calendar.h"
#include "unicode/strenum.h"
#include "unicode/smpdtfmt.h"
#include "unicode/uchar.h"
#include "unicode/basictz.h"
#include "cmemory.h"
#include "cstring.h"

#include "unicode/uperf.h"
#include "unicode/unistr.h"
#include "unicode/datefmt.h"
#include "unicode/calendar.h"
#include "unicode/uclean.h"
#include "unicode/brkiter.h"
#include "util.h"

static const char* PATTERNS[] = {"z", "zzzz", "Z", "ZZZZ", "v", "vvvv", "V", "VVVV"};
static const int NUM_PATTERNS = UPRV_LENGTHOF(PATTERNS);

#include <iostream>
#include <stdlib.h>
#include <fstream>
#include <string>
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

class DateTimeRoundTripFunction : public UPerfFunction
{
private:
	int nLocales;
public:
	
	DateTimeRoundTripFunction()
	{
		nLocales = 0;
	}

	DateTimeRoundTripFunction(int locs)
	{
		nLocales = locs;
	}

	virtual void call(UErrorCode* status)
	{
        *status = U_ZERO_ERROR;

        SimpleTimeZone unknownZone(-31415, (UnicodeString)"Etc/Unknown");
        int32_t badDstOffset = -1234;
        int32_t badZoneOffset = -2345;

        int32_t testDateData[][3] = {
            {2007, 1, 15},
            {2007, 6, 15},
            {1990, 1, 15},
            {1990, 6, 15},
            {1960, 1, 15},
            {1960, 6, 15},
        };

        Calendar *cal = Calendar::createInstance(*status);
        if (U_FAILURE(*status)) {
            //dataerrln("Calendar::createInstance failed: %s", u_errorName(*status));
            return;
        }

        // Set up rule equivalency test range
        UDate low, high;
        cal->set(1900, UCAL_JANUARY, 1);
        low = cal->getTime(*status);
        cal->set(2040, UCAL_JANUARY, 1);
        high = cal->getTime(*status);
        if (U_FAILURE(*status)) {
            //errln("getTime failed");
            return;
        }

        // Set up test dates
        UDate DATES[UPRV_LENGTHOF(testDateData)/3];
        const int32_t nDates = UPRV_LENGTHOF(testDateData)/3;
        cal->clear();
        for (int32_t i = 0; i < nDates; i++) {
            cal->set(testDateData[i][0], testDateData[i][1], testDateData[i][2]);
            DATES[i] = cal->getTime(*status);
            if (U_FAILURE(*status)) {
                //errln("getTime failed");
                return;
            }
        }

        // Set up test locales
        const Locale testLocales[] = {
            Locale("en"),
            Locale("en_US"),
            Locale("en_AU"),
            Locale("de_DE"),
            Locale("fr"),
            Locale("ja_JP"),
            Locale("ko"),
            Locale("pt"),
            Locale("th_TH"),
            Locale("zh_Hans"),

            Locale("it"),

            Locale("en"),
            Locale("en_US"),
            Locale("en_AU"),
            Locale("de_DE"),
            Locale("fr"),
            Locale("ja_JP"),
            Locale("ko"),
            Locale("pt"),
            Locale("th_TH"),
            Locale("zh_Hans"),
        };

        const Locale *LOCALES;
        LOCALES = testLocales;

        StringEnumeration *tzids = TimeZone::createEnumeration(*status);
        if (U_FAILURE(*status)) {
            //errln("tzids->count failed");
            return;
        }

        // Run the roundtrip test
        for (int32_t locidx = 0; locidx < nLocales; locidx++) {
            for (int32_t patidx = 0; patidx < NUM_PATTERNS; patidx++) {
                SimpleDateFormat *sdf = new SimpleDateFormat((UnicodeString)PATTERNS[patidx], LOCALES[locidx], *status);
                if (U_FAILURE(*status)) {
                    //errcheckln(*status, (UnicodeString)"new SimpleDateFormat failed for pattern " +
                    //    PATTERNS[patidx] + " for locale " + LOCALES[locidx].getName() + " - " + u_errorName(*status));
                    *status = U_ZERO_ERROR;
                    continue;
                }

                tzids->reset(*status);
                const UnicodeString *tzid;
                while ((tzid = tzids->snext(*status))) {
                    TimeZone *tz = TimeZone::createTimeZone(*tzid);

                    for (int32_t datidx = 0; datidx < nDates; datidx++) {
                        UnicodeString tzstr;
                        FieldPosition fpos(FieldPosition::DONT_CARE);

                        // Format
                        sdf->setTimeZone(*tz);
                        sdf->format(DATES[datidx], tzstr, fpos);

                        // Before parse, set unknown zone to SimpleDateFormat instance
                        // just for making sure that it does not depends on the time zone
                        // originally set.
                        sdf->setTimeZone(unknownZone);

                        // Parse
                        ParsePosition pos(0);
                        Calendar *outcal = Calendar::createInstance(unknownZone, *status);
                        if (U_FAILURE(*status)) {
                            //errln("Failed to create an instance of calendar for receiving parse result.");
                            *status = U_ZERO_ERROR;
                            continue;
                        }
                        outcal->set(UCAL_DST_OFFSET, badDstOffset);
                        outcal->set(UCAL_ZONE_OFFSET, badZoneOffset);
                        sdf->parse(tzstr, *outcal, pos);

                        // clean loop
                        delete outcal;

                    }
                    delete tz;
                    // break  time zone loop
                    break;

                }
                delete sdf;
            }
        }
        delete cal;
        delete tzids;

	}

	virtual long getOperationsPerIteration()
	{
		return NUM_PATTERNS * nLocales * 6;
	}
};


class DateTimeRoundTripPerfTest : public UPerfTest
{
private:

public:

	DateTimeRoundTripPerfTest(int32_t argc, const char* argv[], UErrorCode& status);
	~DateTimeRoundTripPerfTest();
	virtual UPerfFunction* runIndexedTest(int32_t index, UBool exec,const char* &name, char* par);

	UPerfFunction* RoundTripLocale1();
	UPerfFunction* RoundTripLocale10();
	UPerfFunction* RoundTripLocale11();
	UPerfFunction* RoundTripLocale21();
};


#endif // DateTimeRoundTripPerfTest
