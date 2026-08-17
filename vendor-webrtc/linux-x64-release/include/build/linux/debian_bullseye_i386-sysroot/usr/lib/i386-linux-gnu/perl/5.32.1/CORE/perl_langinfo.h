/* Replaces <langinfo.h>, and allows our code to work on systems that don't
 * have that. */

#ifndef PERL_LANGINFO_H
#define PERL_LANGINFO_H 1

#include "config.h"

#if defined(HAS_NL_LANGINFO) && defined(I_LANGINFO)
#   include <langinfo.h>
#endif

/* NOTE that this file is parsed by ext/XS-APItest/t/locale.t, so be careful
 * with changes */

/* If foo doesn't exist define it to a negative number. */

#ifndef CODESET
#  define CODESET -1
#endif
#ifndef D_T_FMT
#  define D_T_FMT -2
#endif
#ifndef D_FMT
#  define D_FMT -3
#endif
#ifndef T_FMT
#  define T_FMT -4
#endif
#ifndef T_FMT_AMPM
#  define T_FMT_AMPM -5
#endif
#ifndef AM_STR
#  define AM_STR -6
#endif
#ifndef PM_STR
#  define PM_STR -7
#endif
#ifndef DAY_1
#  define DAY_1 -8
#endif
#ifndef DAY_2
#  define DAY_2 -9
#endif
#ifndef DAY_3
#  define DAY_3 -10
#endif
#ifndef DAY_4
#  define DAY_4 -11
#endif
#ifndef DAY_5
#  define DAY_5 -12
#endif
#ifndef DAY_6
#  define DAY_6 -13
#endif
#ifndef DAY_7
#  define DAY_7 -14
#endif
#ifndef ABDAY_1
#  define ABDAY_1 -15
#endif
#ifndef ABDAY_2
#  define ABDAY_2 -16
#endif
#ifndef ABDAY_3
#  define ABDAY_3 -17
#endif
#ifndef ABDAY_4
#  define ABDAY_4 -18
#endif
#ifndef ABDAY_5
#  define ABDAY_5 -19
#endif
#ifndef ABDAY_6
#  define ABDAY_6 -20
#endif
#ifndef ABDAY_7
#  define ABDAY_7 -21
#endif
#ifndef MON_1
#  define MON_1 -22
#endif
#ifndef MON_2
#  define MON_2 -23
#endif
#ifndef MON_3
#  define MON_3 -24
#endif
#ifndef MON_4
#  define MON_4 -25
#endif
#ifndef MON_5
#  define MON_5 -26
#endif
#ifndef MON_6
#  define MON_6 -27
#endif
#ifndef MON_7
#  define MON_7 -28
#endif
#ifndef MON_8
#  define MON_8 -29
#endif
#ifndef MON_9
#  define MON_9 -30
#endif
#ifndef MON_10
#  define MON_10 -31
#endif
#ifndef MON_11
#  define MON_11 -32
#endif
#ifndef MON_12
#  define MON_12 -33
#endif
#ifndef ABMON_1
#  define ABMON_1 -34
#endif
#ifndef ABMON_2
#  define ABMON_2 -35
#endif
#ifndef ABMON_3
#  define ABMON_3 -36
#endif
#ifndef ABMON_4
#  define ABMON_4 -37
#endif
#ifndef ABMON_5
#  define ABMON_5 -38
#endif
#ifndef ABMON_6
#  define ABMON_6 -39
#endif
#ifndef ABMON_7
#  define ABMON_7 -40
#endif
#ifndef ABMON_8
#  define ABMON_8 -41
#endif
#ifndef ABMON_9
#  define ABMON_9 -42
#endif
#ifndef ABMON_10
#  define ABMON_10 -43
#endif
#ifndef ABMON_11
#  define ABMON_11 -44
#endif
#ifndef ABMON_12
#  define ABMON_12 -45
#endif
#ifndef ERA
#  define ERA -46
#endif
#ifndef ERA_D_FMT
#  define ERA_D_FMT -47
#endif
#ifndef ERA_D_T_FMT
#  define ERA_D_T_FMT -48
#endif
#ifndef ERA_T_FMT
#  define ERA_T_FMT -49
#endif
#ifndef ALT_DIGITS
#  define ALT_DIGITS -50
#endif
#ifndef RADIXCHAR
#  define RADIXCHAR -51
#endif
#ifndef THOUSEP
#  define THOUSEP -52
#endif
#ifndef YESEXPR
#  define YESEXPR -53
#endif
#ifndef YESSTR
#  define YESSTR -54
#endif
#ifndef NOEXPR
#  define NOEXPR -55
#endif
#ifndef NOSTR
#  define NOSTR -56
#endif
#ifndef CRNCYSTR
#  define CRNCYSTR -57
#endif

#endif
