/*    dosish.h
 *
 *    Copyright (C) 1993, 1994, 1996, 1997, 1998, 1999,
 *    2000, 2001, 2002, 2007, by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */
#define ABORT() abort();

#ifndef SH_PATH
#define SH_PATH "/bin/sh"
#endif

#ifdef DJGPP
#  define BIT_BUCKET "nul"
#  define OP_BINARY O_BINARY
#  define PERL_SYS_INIT_BODY(c,v)					\
	 MALLOC_CHECK_TAINT2(*c,*v) Perl_DJGPP_init(c,v); PERLIO_INIT
#  define init_os_extras Perl_init_os_extras
#  define HAS_UTIME
#  define HAS_KILL
   char *djgpp_pathexp (const char*);
   void Perl_DJGPP_init (int *argcp,char ***argvp);
#  if (DJGPP==2 && DJGPP_MINOR < 2)
#    define NO_LOCALECONV_MON_THOUSANDS_SEP
#  endif
#  ifndef PERL_CORE
#    define PERL_FS_VER_FMT	"%d_%d_%d"
#  endif
#  define PERL_FS_VERSION	STRINGIFY(PERL_REVISION) "_" \
				STRINGIFY(PERL_VERSION) "_" \
				STRINGIFY(PERL_SUBVERSION)
#elif defined(WIN32)
#  define PERL_SYS_INIT_BODY(c,v)					\
      MALLOC_CHECK_TAINT2(*c,*v) Perl_win32_init(c,v); PERLIO_INIT
#  define PERL_SYS_TERM_BODY()   Perl_win32_term()
#  define BIT_BUCKET "nul"
#elif defined(NETWARE)
#  define PERL_SYS_INIT_BODY(c,v)					\
    MALLOC_CHECK_TAINT2(*c,*v) Perl_nw5_init(c,v); PERLIO_INIT
#  define BIT_BUCKET "nwnul"
#else
#  define PERL_SYS_INIT_BODY(c,v)		\
    MALLOC_CHECK_TAINT2(*c,*v); PERLIO_INIT
#  define BIT_BUCKET "\\dev\\nul" /* "wanna be like, umm, Newlined, or somethin?" */
#endif

#ifndef PERL_SYS_TERM_BODY
#  define PERL_SYS_TERM_BODY()                         \
    HINTS_REFCNT_TERM; KEYWORD_PLUGIN_MUTEX_TERM;      \
    OP_CHECK_MUTEX_TERM; OP_REFCNT_TERM; PERLIO_TERM;  \
    MALLOC_TERM; LOCALE_TERM; USER_PROP_MUTEX_TERM;    \
    ENV_TERM;
#endif
#define dXSUB_SYS dNOOP

/* USEMYBINMODE
 *	This symbol, if defined, indicates that the program should
 *	use the routine my_binmode(FILE *fp, char iotype, int mode) to insure
 *	that a file is in "binary" mode -- that is, that no translation
 *	of bytes occurs on read or write operations.
 */
#undef USEMYBINMODE

/* Stat_t:
 *	This symbol holds the type used to declare buffers for information
 *	returned by stat().  It's usually just struct stat.  It may be necessary
 *	to include <sys/stat.h> and <sys/types.h> to get any typedef'ed
 *	information.
 */
#if defined(WIN64) || defined(USE_LARGE_FILES)
#  define Stat_t struct _stati64
#else
#  define Stat_t struct stat
#endif

/* USE_STAT_RDEV:
 *	This symbol is defined if this system has a stat structure declaring
 *	st_rdev
 */
#define USE_STAT_RDEV 	/**/

/* ACME_MESS:
 *	This symbol, if defined, indicates that error messages should be 
 *	should be generated in a format that allows the use of the Acme
 *	GUI/editor's autofind feature.
 */
#undef ACME_MESS	/**/

/* ALTERNATE_SHEBANG:
 *	This symbol, if defined, contains a "magic" string which may be used
 *	as the first line of a Perl program designed to be executed directly
 *	by name, instead of the standard Unix #!.  If ALTERNATE_SHEBANG
 *	begins with a character other then #, then Perl will only treat
 *	it as a command line if it finds the string "perl" in the first
 *	word; otherwise it's treated as the first line of code in the script.
 *	(IOW, Perl won't hand off to another interpreter via an alternate
 *	shebang sequence that might be legal Perl code.)
 */
/* #define ALTERNATE_SHEBANG "#!" / **/

#include <signal.h>

/*
 * fwrite1() should be a routine with the same calling sequence as fwrite(),
 * but which outputs all of the bytes requested as a single stream (unlike
 * fwrite() itself, which on some systems outputs several distinct records
 * if the number_of_items parameter is >1).
 */
#define fwrite1 fwrite

#define Fstat(fd,bufptr)   fstat((fd),(bufptr))
#ifdef DJGPP
#   define Fflush(fp)      djgpp_fflush(fp)
#else
#   define Fflush(fp)      fflush(fp)
#endif
#define Mkdir(path,mode)   mkdir((path),(mode))

#ifndef WIN32
#  define Stat(fname,bufptr) stat((fname),(bufptr))
#else
#  define HAS_IOCTL
#  define HAS_UTIME
#  define HAS_KILL
#  define HAS_WAIT
#  define HAS_CHOWN
#endif	/* WIN32 */

/*
 * <rich@phekda.freeserve.co.uk>: The DJGPP port has code that converts
 * the return code of system() into the form that Unixy wait usually
 * returns:
 *
 * - signal number in bits 0-6;
 * - core dump flag in bit 7;
 * - exit code in bits 8-15.
 *
 * Bits 0-7 are always zero for DJGPP, because it uses system().
 * See djgpp.c.
 *
 * POSIX::W* use the W* macros from <sys/wait.h> to decode
 * the return code. Unfortunately the W* macros for DJGPP use
 * a different format than Unixy wait does. So there's a mismatch
 * and, say, WEXITSTATUS($?) will return bogus values.
 *
 * So here we add hack to redefine the W* macros from DJGPP's <sys/wait.h>
 * to work with our return-code conversion.
 */

#ifdef DJGPP

#include <sys/wait.h>

#undef WEXITSTATUS
#undef WIFEXITED
#undef WIFSIGNALED
#undef WIFSTOPPED
#undef WNOHANG
#undef WSTOPSIG
#undef WTERMSIG
#undef WUNTRACED

#define WEXITSTATUS(stat_val) ((stat_val) >> 8)
#define WIFEXITED(stat_val)   0
#define WIFSIGNALED(stat_val) 0
#define WIFSTOPPED(stat_val)  0
#define WNOHANG               0
#define WSTOPSIG(stat_val)    0
#define WTERMSIG(stat_val)    0
#define WUNTRACED             0

#endif

/* Don't go reading from /dev/urandom */
#define PERL_NO_DEV_RANDOM

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
