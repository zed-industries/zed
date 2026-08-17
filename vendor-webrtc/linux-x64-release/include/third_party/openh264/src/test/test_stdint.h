#ifndef TEST_STDINT_H
#define TEST_STDINT_H

#ifndef _MSC_VER

#include <stdint.h>

#else

typedef signed char      int8_t  ;
typedef unsigned char    uint8_t ;
typedef short            int16_t ;
typedef unsigned short   uint16_t;
typedef int              int32_t ;
typedef unsigned int     uint32_t;
typedef __int64          int64_t ;
typedef unsigned __int64 uint64_t;
typedef short            int_least16_t;

#endif

#endif //TEST_STDINT_H
