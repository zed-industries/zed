/*
 * nasmint.h
 *
 * Small ersatz subset of <inttypes.h>, deriving the types from
 * <limits.h>.
 *
 * Important: the preprocessor may truncate numbers too large for it.
 * Therefore, test the signed types only ... truncation won't generate
 * a 01111111... bit pattern.
 */

#ifndef NASM_NASMINT_H
#define NASM_NASMINT_H

#include <limits.h>

/*** 64-bit type: __int64, long or long long ***/

/* Some old versions of gcc <limits.h> omit LLONG_MAX */
#ifndef LLONG_MAX
# ifdef __LONG_LONG_MAX__
#  define LLONG_MAX __LONG_LONG_MAX__
# else
#  define LLONG_MAX 0		/* Assume long long is unusable */
# endif
#endif

#ifndef _I64_MAX
# ifdef _MSC_VER
#  define _I64_MAX 9223372036854775807
# else
#  define _I64_MAX 0
# endif
#endif

#if _I64_MAX == 9223372036854775807

/* Windows-based compiler: use __int64 */
typedef signed __int64		int64_t;
typedef unsigned __int64	uint64_t;
#define _scn64			"I64"
#define _pri64			"I64"
#define INT64_C(x)		x ## i64
#define UINT64_C(x)		x ## ui64

#elif LONG_MAX == 9223372036854775807L

/* long is 64 bits */
typedef signed long		int64_t;
typedef unsigned long		uint64_t;
#define _scn64			"l"
#define _pri64			"l"
#define INT64_C(x)		x ## L
#define UINT64_C(x)		x ## UL

#elif LLONG_MAX == 9223372036854775807LL

/* long long is 64 bits */
typedef signed long long	int64_t;
typedef unsigned long long	uint64_t;
#define _scn64			"ll"
#define _pri64			"ll"
#define INT64_C(x)		x ## LL
#define UINT64_C(x)		x ## ULL

#else

#error "Neither long nor long long is 64 bits in size"

#endif

/*** 32-bit type: int or long ***/

#if INT_MAX == 2147483647

/* int is 32 bits */
typedef signed int		int32_t;
typedef unsigned int		uint32_t;
#define _scn32			""
#define _pri32			""
#define INT32_C(x)		x
#define UINT32_C(x)		x ## U

#elif LONG_MAX == 2147483647L

/* long is 32 bits */
typedef signed long		int32_t;
typedef unsigned long		uint32_t;
#define _scn32			"l"
#define _pri32			"l"
#define INT32_C(x)		x ## L
#define UINT32_C(x)		x ## UL

#else

#error "Neither int nor long is 32 bits in size"

#endif

/*** 16-bit size: int or short ***/

#if INT_MAX == 32767

/* int is 16 bits */
typedef signed int		int16_t;
typedef unsigned int		uint16_t;
#define _scn16			""
#define _pri16			""
#define INT16_C(x)		x
#define UINT16_C(x)		x ## U

#elif SHRT_MAX == 32767

/* short is 16 bits */
typedef signed short		int16_t;
typedef unsigned short		uint16_t;
#define _scn16			"h"
#define _pri16			""
#define INT16_C(x)		x
#define UINT16_C(x)		x ## U

#else

#error "Neither short nor int is 16 bits in size"

#endif

/*** 8-bit size: char ***/

#if SCHAR_MAX == 127

/* char is 8 bits */
typedef signed char		int8_t;
typedef unsigned char		uint8_t;
#define _scn8			"hh"
#define _pri8			""
#define INT8_C(x)		x
#define UINT8_C(x)		x ## U

#else

#error "char is not 8 bits in size"

#endif

/* The rest of this is common to all models */

#define PRId8		_pri8  "d"
#define PRId16		_pri16 "d"
#define PRId32		_pri32 "d"
#define PRId64		_pri64 "d"

#define PRIi8		_pri8  "i"
#define PRIi16		_pri16 "i"
#define PRIi32		_pri32 "i"
#define PRIi64		_pri64 "i"

#define PRIo8		_pri8  "o"
#define PRIo16		_pri16 "o"
#define PRIo32		_pri32 "o"
#define PRIo64		_pri64 "o"

#define PRIu8		_pri8  "u"
#define PRIu16		_pri16 "u"
#define PRIu32		_pri32 "u"
#define PRIu64		_pri64 "u"

#define PRIx8		_pri8  "x"
#define PRIx16		_pri16 "x"
#define PRIx32		_pri32 "x"
#define PRIx64		_pri64 "x"

#define PRIX8		_pri8  "X"
#define PRIX16		_pri16 "X"
#define PRIX32		_pri32 "X"
#define PRIX64		_pri64 "X"

#define SCNd8		_scn8  "d"
#define SCNd16		_scn16 "d"
#define SCNd32		_scn32 "d"
#define SCNd64		_scn64 "d"

#define SCNi8		_scn8  "i"
#define SCNi16		_scn16 "i"
#define SCNi32		_scn32 "i"
#define SCNi64		_scn64 "i"

#define SCNo8		_scn8  "o"
#define SCNo16		_scn16 "o"
#define SCNo32		_scn32 "o"
#define SCNo64		_scn64 "o"

#define SCNu8		_scn8  "u"
#define SCNu16		_scn16 "u"
#define SCNu32		_scn32 "u"
#define SCNu64		_scn64 "u"

#define SCNx8		_scn8  "x"
#define SCNx16		_scn16 "x"
#define SCNx32		_scn32 "x"
#define SCNx64		_scn64 "x"

#define INT8_MIN	INT8_C(-128)
#define INT8_MAX	INT8_C(127)
#define UINT8_MAX	UINT8_C(255)

#define INT16_MIN	INT16_C(-32768)
#define INT16_MAX	INT16_C(32767)
#define UINT16_MAX	UINT16_C(65535)

#define INT32_MIN	INT32_C(-2147483648)
#define INT32_MAX	INT32_C(2147483647)
#define UINT32_MAX	UINT32_C(4294967295)

#define INT64_MIN	INT64_C(-9223372036854775808)
#define INT64_MAX	INT64_C(9223372036854775807)
#define UINT64_MAX	UINT64_C(18446744073709551615)

#endif /* NASM_NASMINT_H */
