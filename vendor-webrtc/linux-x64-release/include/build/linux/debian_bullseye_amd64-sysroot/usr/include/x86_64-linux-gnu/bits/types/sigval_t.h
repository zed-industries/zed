#ifndef __sigval_t_defined
#define __sigval_t_defined

#include <bits/types/__sigval_t.h>

/* To avoid sigval_t (not a standard type name) having C++ name
   mangling depending on whether the selected standard includes union
   sigval, it should not be defined at all when using a standard for
   which the sigval name is not reserved; in that case, headers should
   not include <bits/types/sigval_t.h> and should use only the
   internal __sigval_t name.  */
#ifndef __USE_POSIX199309
# error "sigval_t defined for standard not including union sigval"
#endif

typedef __sigval_t sigval_t;

#endif
