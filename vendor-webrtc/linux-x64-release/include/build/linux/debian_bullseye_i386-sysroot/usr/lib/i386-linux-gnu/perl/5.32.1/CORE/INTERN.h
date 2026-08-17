/*    INTERN.h
 *
 *    Copyright (C) 1991, 1992, 1993, 1995, 1996, 1998, 2000, 2001,
 *    by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

/*
 * EXT  designates a global var which is defined in perl.h
 * dEXT designates a global var which is defined in another
 *      file, so we can't count on finding it in perl.h
 *      (this practice should be avoided).
 */
#undef EXT
#undef dEXT
#undef EXTCONST
#undef dEXTCONST

#  if (defined(WIN32) && defined(__MINGW32__) && ! defined(PERL_IS_MINIPERL)) \
     || defined(__SYMBIAN32__)
#    ifdef __cplusplus
#      define EXT	__declspec(dllexport)
#      define dEXT
#      define EXTCONST	__declspec(dllexport) extern const
#      define dEXTCONST	const
#    else
#      define EXT	__declspec(dllexport)
#      define dEXT
#      define EXTCONST	__declspec(dllexport) const
#      define dEXTCONST	const
#    endif
#  else
#    ifdef __cplusplus
#      define EXT
#      define dEXT
#      define EXTCONST EXTERN_C const
#      define dEXTCONST const
#    else
#      define EXT
#      define dEXT
#      define EXTCONST const
#      define dEXTCONST const
#    endif
#  endif

#undef INIT
#define INIT(x) = x

#define DOINIT
