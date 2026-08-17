/* just define a list of macros to push elements in INC
*  so we can easily use them and change order on demand
*
* list of available INCPUSH macros
* - INCPUSH_APPLLIB_EXP
* - INCPUSH_SITEARCH_EXP
* - INCPUSH_SITELIB_EXP
* - INCPUSH_PERL_VENDORARCH_EXP
* - INCPUSH_PERL_VENDORLIB_EXP
* - INCPUSH_ARCHLIB_EXP
* - INCPUSH_PRIVLIB_EXP
* - INCPUSH_PERL_OTHERLIBDIRS
* - INCPUSH_PERL5LIB
* - INCPUSH_APPLLIB_OLD_EXP
* - INCPUSH_SITELIB_STEM
* - INCPUSH_PERL_VENDORLIB_STEM
* - INCPUSH_PERL_OTHERLIBDIRS_ARCHONLY
*/

#ifndef DEFINE_INC_MACROS

/* protect against multiple inclusions */
#define DEFINE_INC_MACROS 1

#ifdef APPLLIB_EXP
#	define INCPUSH_APPLLIB_EXP  S_incpush_use_sep(aTHX_ STR_WITH_LEN(APPLLIB_EXP), \
		      INCPUSH_ADD_SUB_DIRS|INCPUSH_CAN_RELOCATE);
#endif

#ifdef SITEARCH_EXP
    /* sitearch is always relative to sitelib on Windows for
     * DLL-based path intuition to work correctly */
#  if !defined(WIN32)
#	define INCPUSH_SITEARCH_EXP S_incpush_use_sep(aTHX_ STR_WITH_LEN(SITEARCH_EXP), \
			  INCPUSH_CAN_RELOCATE);
#  endif
#endif

#ifdef SITELIB_EXP
#  if defined(WIN32)
    /* this picks up sitearch as well */
#	  define INCPUSH_SITELIB_EXP s = PerlEnv_sitelib_path(PERL_FS_VERSION, &len); \
		if (s) incpush_use_sep(s, len, INCPUSH_ADD_SUB_DIRS|INCPUSH_CAN_RELOCATE);
#  else
#	  define INCPUSH_SITELIB_EXP S_incpush_use_sep(aTHX_ STR_WITH_LEN(SITELIB_EXP), \
		INCPUSH_CAN_RELOCATE);
#  endif
#endif

#ifdef PERL_VENDORARCH_EXP
    /* vendorarch is always relative to vendorlib on Windows for
     * DLL-based path intuition to work correctly */
#  if !defined(WIN32)
#		define INCPUSH_PERL_VENDORARCH_EXP  S_incpush_use_sep(aTHX_ STR_WITH_LEN(PERL_VENDORARCH_EXP), INCPUSH_CAN_RELOCATE);
#  endif
#endif

#ifdef PERL_VENDORLIB_EXP
#  if defined(WIN32)
    /* this picks up vendorarch as well */
#		define INCPUSH_PERL_VENDORLIB_EXP s = PerlEnv_vendorlib_path(PERL_FS_VERSION, &len); \
			if (s) incpush_use_sep(s, len, INCPUSH_ADD_SUB_DIRS|INCPUSH_CAN_RELOCATE);
#  else
#		define INCPUSH_PERL_VENDORLIB_EXP S_incpush_use_sep(aTHX_ STR_WITH_LEN(PERL_VENDORLIB_EXP), INCPUSH_CAN_RELOCATE);
#  endif
#endif

#ifdef ARCHLIB_EXP
#	define INCPUSH_ARCHLIB_EXP S_incpush_use_sep(aTHX_ STR_WITH_LEN(ARCHLIB_EXP), INCPUSH_CAN_RELOCATE);
#endif

/* used by INCPUSH_PRIVLIB_EXP */
#ifndef PRIVLIB_EXP
#  define PRIVLIB_EXP "/usr/local/lib/perl5:/usr/local/lib/perl"
#endif

#if defined(WIN32)
#	define INCPUSH_PRIVLIB_EXP s = PerlEnv_lib_path(PERL_FS_VERSION, &len); \
    if (s) incpush_use_sep(s, len, INCPUSH_ADD_SUB_DIRS|INCPUSH_CAN_RELOCATE);
#elif defined(NETWARE)
#	define INCPUSH_PRIVLIB_EXP S_incpush_use_sep(aTHX_ PRIVLIB_EXP, 0, INCPUSH_CAN_RELOCATE);
#else
#	define INCPUSH_PRIVLIB_EXP S_incpush_use_sep(aTHX_ STR_WITH_LEN(PRIVLIB_EXP), INCPUSH_CAN_RELOCATE);
#endif

#ifdef PERL_OTHERLIBDIRS
#	define INCPUSH_PERL_OTHERLIBDIRS S_incpush_use_sep(aTHX_ STR_WITH_LEN(PERL_OTHERLIBDIRS), \
		      INCPUSH_ADD_VERSIONED_SUB_DIRS|INCPUSH_NOT_BASEDIR|INCPUSH_CAN_RELOCATE);
#endif


/* submacros for INCPUSH_PERL5LIB */

#if defined(PERL_USE_SAFE_PUTENV) && ! defined(HAS_UNSETENV)
# 	define _INCPUSH_PERL5LIB_IF	if (perl5lib && *perl5lib != '\0')
#else
# 	define _INCPUSH_PERL5LIB_IF	if (perl5lib)
#endif

#ifndef VMS
/*
 * It isn't possible to delete an environment variable with
 * PERL_USE_SAFE_PUTENV set unless unsetenv() is also available, so in that
 * case we treat PERL5LIB as undefined if it has a zero-length value.
 */
# define _INCPUSH_PERL5LIB_ADD _INCPUSH_PERL5LIB_IF incpush_use_sep(perl5lib, 0, INCPUSH_ADD_OLD_VERS|INCPUSH_NOT_BASEDIR);
#else
/* VMS */
	/* Treat PERL5?LIB as a possible search list logical name -- the
	 * "natural" VMS idiom for a Unix path string.  We allow each
	 * element to be a set of |-separated directories for compatibility.
	 */
# define _INCPUSH_PERL5LIB_ADD char buf[256]; \
	int idx = 0; \
	if (vmstrnenv("PERL5LIB",buf,0,NULL,0)) \
	    do { \
		incpush_use_sep(buf, 0, \
				INCPUSH_ADD_OLD_VERS|INCPUSH_NOT_BASEDIR); \
	    } while (vmstrnenv("PERL5LIB",buf,++idx,NULL,0));
#endif

/* this macro is special and use submacros from above */
#define INCPUSH_PERL5LIB if (!TAINTING_get) { _INCPUSH_PERL5LIB_ADD }

/* Use the ~-expanded versions of APPLLIB (undocumented),
    SITELIB and VENDORLIB for older versions
*/
#ifdef APPLLIB_EXP
#	define INCPUSH_APPLLIB_OLD_EXP S_incpush_use_sep(aTHX_ STR_WITH_LEN(APPLLIB_EXP), \
	    INCPUSH_ADD_OLD_VERS|INCPUSH_NOT_BASEDIR|INCPUSH_CAN_RELOCATE);
#endif

#if defined(SITELIB_STEM) && defined(PERL_INC_VERSION_LIST)
    /* Search for version-specific dirs below here */
#	define INCPUSH_SITELIB_STEM   S_incpush_use_sep(aTHX_ STR_WITH_LEN(SITELIB_STEM), \
		      INCPUSH_ADD_OLD_VERS|INCPUSH_CAN_RELOCATE);
#endif


#if defined(PERL_VENDORLIB_STEM) && defined(PERL_INC_VERSION_LIST)
    /* Search for version-specific dirs below here */
#	define INCPUSH_PERL_VENDORLIB_STEM    S_incpush_use_sep(aTHX_ STR_WITH_LEN(PERL_VENDORLIB_STEM), \
		      INCPUSH_ADD_OLD_VERS|INCPUSH_CAN_RELOCATE);
#endif

#ifdef PERL_OTHERLIBDIRS
#	define INCPUSH_PERL_OTHERLIBDIRS_ARCHONLY  S_incpush_use_sep(aTHX_ STR_WITH_LEN(PERL_OTHERLIBDIRS), \
		      INCPUSH_ADD_OLD_VERS|INCPUSH_ADD_ARCHONLY_SUB_DIRS|INCPUSH_CAN_RELOCATE);
#endif


/* define all undefined macros... */
#ifndef INCPUSH_APPLLIB_EXP
#define INCPUSH_APPLLIB_EXP
#endif
#ifndef INCPUSH_SITEARCH_EXP
#define INCPUSH_SITEARCH_EXP
#endif
#ifndef INCPUSH_SITELIB_EXP
#define INCPUSH_SITELIB_EXP
#endif
#ifndef INCPUSH_PERL_VENDORARCH_EXP
#define INCPUSH_PERL_VENDORARCH_EXP
#endif
#ifndef INCPUSH_PERL_VENDORLIB_EXP
#define INCPUSH_PERL_VENDORLIB_EXP
#endif
#ifndef INCPUSH_ARCHLIB_EXP
#define INCPUSH_ARCHLIB_EXP
#endif
#ifndef INCPUSH_PRIVLIB_EXP
#define INCPUSH_PRIVLIB_EXP
#endif
#ifndef INCPUSH_PERL_OTHERLIBDIRS
#define INCPUSH_PERL_OTHERLIBDIRS
#endif
#ifndef INCPUSH_PERL5LIB
#define INCPUSH_PERL5LIB
#endif
#ifndef INCPUSH_APPLLIB_OLD_EXP
#define INCPUSH_APPLLIB_OLD_EXP
#endif
#ifndef INCPUSH_SITELIB_STEM
#define INCPUSH_SITELIB_STEM
#endif
#ifndef INCPUSH_PERL_VENDORLIB_STEM
#define INCPUSH_PERL_VENDORLIB_STEM
#endif
#ifndef INCPUSH_PERL_OTHERLIBDIRS_ARCHONLY
#define INCPUSH_PERL_OTHERLIBDIRS_ARCHONLY
#endif

#endif /* DEFINE_INC_MACROS */
