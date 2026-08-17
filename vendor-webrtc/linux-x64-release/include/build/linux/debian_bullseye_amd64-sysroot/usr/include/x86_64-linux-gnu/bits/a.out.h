#ifndef __A_OUT_GNU_H__
# error "Never use <bits/a.out.h> directly; include <a.out.h> instead."
#endif

#ifdef __x86_64__

/* Signal to users of this header that this architecture really doesn't
   support a.out binary format.  */
#define __NO_A_OUT_SUPPORT 1

#endif
