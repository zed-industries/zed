/*    unixish.h
 *
 *    Copyright (C) 1993, 1994, 1995, 1996, 1997, 1999, 2000, 2001, 2002,
 *    2003, 2006, 2007, by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

/*
 * The following symbols are defined if your operating system supports
 * functions by that name.  All Unixes I know of support them, thus they
 * are not checked by the configuration script, but are directly defined
 * here.
 */

#ifndef PERL_MICRO

/* HAS_IOCTL:
 *	This symbol, if defined, indicates that the ioctl() routine is
 *	available to set I/O characteristics
 */
#define	HAS_IOCTL		/**/
 
/* HAS_UTIME:
 *	This symbol, if defined, indicates that the routine utime() is
 *	available to update the access and modification times of files.
 */
#define HAS_UTIME		/**/

/* HAS_GROUP
 *	This symbol, if defined, indicates that the getgrnam() and
 *	getgrgid() routines are available to get group entries.
 *	The getgrent() has a separate definition, HAS_GETGRENT.
 */
#define HAS_GROUP		/**/

/* HAS_PASSWD
 *	This symbol, if defined, indicates that the getpwnam() and
 *	getpwuid() routines are available to get password entries.
 *	The getpwent() has a separate definition, HAS_GETPWENT.
 */
#define HAS_PASSWD		/**/

#define HAS_KILL
#define HAS_WAIT

#endif /* !PERL_MICRO */
  
/* USEMYBINMODE
 *	This symbol, if defined, indicates that the program should
 *	use the routine my_binmode(FILE *fp, char iotype) to insure
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
#define Stat_t struct stat

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

/* UNLINK_ALL_VERSIONS:
 *	This symbol, if defined, indicates that the program should arrange
 *	to remove all versions of a file if unlink() is called.  This is
 *	probably only relevant for VMS.
 */
/* #define UNLINK_ALL_VERSIONS		/ **/

/* VMS:
 *	This symbol, if defined, indicates that the program is running under
 *	VMS.  It is currently automatically set by cpps running under VMS,
 *	and is included here for completeness only.
 */
/* #define VMS		/ **/

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

# include <signal.h>

#ifndef SIGABRT
#    define SIGABRT SIGILL
#endif
#ifndef SIGILL
#    define SIGILL 6         /* blech */
#endif
#define ABORT() kill(PerlProc_getpid(),SIGABRT);

/*
 * fwrite1() should be a routine with the same calling sequence as fwrite(),
 * but which outputs all of the bytes requested as a single stream (unlike
 * fwrite() itself, which on some systems outputs several distinct records
 * if the number_of_items parameter is >1).
 */
#define fwrite1 fwrite

#define Stat(fname,bufptr) stat((fname),(bufptr))

#ifdef __amigaos4__
int afstat(int fd, struct stat *statb);
#  define Fstat(fd,bufptr) afstat((fd),(bufptr))
#endif

#ifndef Fstat
#  define Fstat(fd,bufptr)   fstat((fd),(bufptr))
#endif

#define Fflush(fp)         fflush(fp)
#define Mkdir(path,mode)   mkdir((path),(mode))

#if defined(__amigaos4__)
#  define PERL_SYS_INIT_BODY(c,v)					\
	MALLOC_CHECK_TAINT2(*c,*v) PERL_FPU_INIT; PERLIO_INIT; MALLOC_INIT; amigaos4_init_fork_array(); amigaos4_init_environ_sema();
#  define PERL_SYS_TERM_BODY()                         \
    HINTS_REFCNT_TERM; KEYWORD_PLUGIN_MUTEX_TERM;      \
    OP_CHECK_MUTEX_TERM; OP_REFCNT_TERM; PERLIO_TERM;  \
    MALLOC_TERM; LOCALE_TERM; USER_PROP_MUTEX_TERM;    \
    ENV_TERM;                                          \
    amigaos4_dispose_fork_array();
#endif

#ifndef PERL_SYS_INIT_BODY
#  define PERL_SYS_INIT_BODY(c,v)					\
	MALLOC_CHECK_TAINT2(*c,*v) PERL_FPU_INIT; PERLIO_INIT; MALLOC_INIT
#endif

#ifndef PERL_SYS_TERM_BODY
#  define PERL_SYS_TERM_BODY()                         \
    HINTS_REFCNT_TERM; KEYWORD_PLUGIN_MUTEX_TERM;      \
    OP_CHECK_MUTEX_TERM; OP_REFCNT_TERM; PERLIO_TERM;  \
    MALLOC_TERM; LOCALE_TERM; USER_PROP_MUTEX_TERM;    \
    ENV_TERM;

#endif

#define BIT_BUCKET "/dev/null"

#define dXSUB_SYS dNOOP

#ifndef NO_ENVIRON_ARRAY
#define USE_ENVIRON_ARRAY
#endif

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
