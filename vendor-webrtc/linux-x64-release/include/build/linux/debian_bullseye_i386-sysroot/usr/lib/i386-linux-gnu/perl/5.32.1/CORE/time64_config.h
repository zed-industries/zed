#ifndef PERL_TIME64_CONFIG_H_
#    define PERL_TIME64_CONFIG_H_

/* Configuration
   -------------
   Define as appropriate for your system.
   Sensible defaults provided.
*/

/* Debugging
   TIME_64_DEBUG
   Define if you want debugging messages
*/
/* #define TIME_64_DEBUG */


/* INT_64_T
   A numeric type to store time and others. 
   Must be defined.
*/
#define INT_64_T                NV


/* USE_TM64
   Should we use a 64 bit safe replacement for tm?  This will
   let you go past year 2 billion but the struct will be incompatible
   with tm.  Conversion functions will be provided.
*/
#define USE_TM64


/* Availability of system functions.

   HAS_GMTIME_R
   Define if your system has gmtime_r()

   HAS_LOCALTIME_R
   Define if your system has localtime_r()

   HAS_TIMEGM
   Define if your system has timegm(), a GNU extension.
*/
/* Set in config.h */


/* Details of non-standard tm struct elements.

   HAS_TM_TM_GMTOFF
   True if your tm struct has a "tm_gmtoff" element.
   A BSD extension.

   HAS_TM_TM_ZONE
   True if your tm struct has a "tm_zone" element.
   A BSD extension.
*/
/* Set in config.h */


/* USE_SYSTEM_LOCALTIME
   USE_SYSTEM_GMTIME
   Should we use the system functions if the time is inside their range?
   Your system localtime() is probably more accurate, but our gmtime() is
   fast and safe.  Except on VMS, where we need the homegrown gmtime()
   override to shift between UTC and local for the vmsish 'time' pragma.
*/
#define USE_SYSTEM_LOCALTIME
#ifdef VMS
#  define USE_SYSTEM_GMTIME
#endif


/* SYSTEM_LOCALTIME_MAX
   SYSTEM_LOCALTIME_MIN
   SYSTEM_GMTIME_MAX
   SYSTEM_GMTIME_MIN
   Maximum and minimum values your system's gmtime() and localtime()
   can handle.  We will use your system functions if the time falls
   inside these ranges.
*/
#define SYSTEM_LOCALTIME_MAX    CAT2(LOCALTIME_MAX,.0)
#define SYSTEM_LOCALTIME_MIN    CAT2(LOCALTIME_MIN,.0)
#define SYSTEM_GMTIME_MAX       CAT2(GMTIME_MAX,.0)
#define SYSTEM_GMTIME_MIN       CAT2(GMTIME_MIN,.0)

#endif /* PERL_TIME64_CONFIG_H_ */
