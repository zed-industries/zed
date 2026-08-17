#ifndef LIBDISASM_QWORD_H
#define LIBDISASM_QWORD_H

#include <stdint.h>

/* platform independent data types */

#ifdef _MSC_VER
	typedef __int64         qword_t;
#else
	typedef int64_t         qword_t;
#endif

#endif
