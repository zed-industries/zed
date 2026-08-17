#ifndef __wint_t_defined
#define __wint_t_defined 1

/* Some versions of stddef.h provide wint_t, even though neither the
   C nor C++ standards, nor POSIX, specifies this.  We assume that
   stddef.h will define the macro _WINT_T if and only if it provides
   wint_t, and conversely, that it will avoid providing wint_t if
   _WINT_T is already defined.  */
#ifndef _WINT_T
#define _WINT_T 1

/* Integral type unchanged by default argument promotions that can
   hold any value corresponding to members of the extended character
   set, as well as at least one value that does not correspond to any
   member of the extended character set.  */
#ifndef __WINT_TYPE__
# define __WINT_TYPE__ unsigned int
#endif

typedef __WINT_TYPE__ wint_t;

#endif /* _WINT_T */
#endif /* bits/types/wint_t.h */
