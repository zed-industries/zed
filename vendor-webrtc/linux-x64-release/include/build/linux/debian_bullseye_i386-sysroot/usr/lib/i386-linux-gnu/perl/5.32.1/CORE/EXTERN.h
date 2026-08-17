/*    EXTERN.h
 *
 *    Copyright (C) 1991, 1992, 1993, 1995, 1996, 1997, 1998, 1999,
 *    2000, 2001, by Larry Wall and others
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

#  if (defined(WIN32) || defined(__SYMBIAN32__)) && !defined(PERL_STATIC_SYMS)
    /* miniperl should not export anything */
#    if defined(PERL_IS_MINIPERL)
#      define EXT extern
#      define dEXT 
#      define EXTCONST extern const
#      define dEXTCONST const
#    elif defined(PERLDLL) || defined(__SYMBIAN32__)
#      define EXT EXTERN_C __declspec(dllexport)
#      define dEXT 
#      define EXTCONST EXTERN_C __declspec(dllexport) const
#      define dEXTCONST const
#    else
#      define EXT EXTERN_C __declspec(dllimport)
#      define dEXT 
#      define EXTCONST EXTERN_C __declspec(dllimport) const
#      define dEXTCONST const
#    endif
#  else
#    if defined(__CYGWIN__) && defined(USEIMPORTLIB)
#      define EXT extern __declspec(dllimport)
#      define dEXT 
#      define EXTCONST extern __declspec(dllimport) const
#      define dEXTCONST const
#    else
#      define EXT extern
#      define dEXT
#      define EXTCONST extern const
#      define dEXTCONST const
#    endif
#  endif

#undef INIT
#define INIT(x)

#undef DOINIT
