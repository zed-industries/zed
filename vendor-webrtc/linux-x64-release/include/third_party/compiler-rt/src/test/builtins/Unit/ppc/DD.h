#ifndef __DD_HEADER
#define __DD_HEADER

#include <stdint.h>

typedef union {
	long double ld;
	struct {
		double hi;
		double lo;
	};
} DD;

#endif // __DD_HEADER
