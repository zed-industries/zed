#include <time.h>
#include "time64_config.h"

#ifndef PERL_TIME64_H_
#    define PERL_TIME64_H_


/* Set our custom types */
typedef INT_64_T        Int64;
typedef Int64           Time64_T;
typedef I32             Year;


/* A copy of the tm struct but with a 64 bit year */
struct TM64 {
        int     tm_sec;
        int     tm_min;
        int     tm_hour;
        int     tm_mday;
        int     tm_mon;
        Year    tm_year;
        int     tm_wday;
        int     tm_yday;
        int     tm_isdst;

#ifdef HAS_TM_TM_GMTOFF
        long    tm_gmtoff;
#endif

#ifdef HAS_TM_TM_ZONE
/* If glibc is defined or we are on QNX, use const.
 * Otherwise, if we are on android, use const but
 * not with g++.
 */
#  if defined(__GLIBC__) || (defined(__ANDROID__) && !defined(__cplusplus)) \
    || defined(__QNX__) || defined(__CYGWIN__)
        const
#  endif
        char    *tm_zone;
#endif
};


/* Decide which tm struct to use */
#ifdef USE_TM64
#define TM      TM64
#else
#define TM      tm
#endif


/* Declare functions */
struct TM *Perl_gmtime64_r    (const Time64_T *, struct TM *);
struct TM *Perl_localtime64_r (const Time64_T *, struct TM *);

#endif
