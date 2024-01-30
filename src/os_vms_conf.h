/*
 * os_vms_conf.h.  Replaces auto/config.h for VMS
 *
 */

#if defined(__VMS) || defined(__vms)
#if !defined(VMS)
#define VMS
#endif
#endif

#define CASE_INSENSITIVE_FILENAME   // Open VMS is case insensitive
#define SPACE_IN_FILENAME	    // There could be space between user and passwd
#define FNAME_ILLEGAL "|*#?%"       // Illegal characters in a file name
#define BINARY_FILE_IO		    // Use binary fileio
#define USE_GETCWD
#define USE_SYSTEM
#define XPMATTRIBUTES_TYPE XpmAttributes

// Define when terminfo support found
#undef TERMINFO

// Define when termcap.h contains ospeed
// #define HAVE_OSPEED

// Define when termcap.h contains UP, BC and PC
// #define HAVE_UP_BC_PC

// Define when termcap.h defines outfuntype
//#define HAVE_OUTFUNTYPE

// Define when __DATE__ " " __TIME__ can be used
#define HAVE_DATE_TIME

// #undef USEBCOPY
#define USEMEMMOVE
// #undef USEMEMCPY

// Define when "man -s 2" is to be used
// #undef USEMAN_S

// Define to empty if the keyword does not work.
// #undef const

// Define to `int' if <sys/types.h> doesn't define.
// #undef mode_t

// Define to `long' if <sys/types.h> doesn't define.
// #undef off_t

// Define to `long' if <sys/types.h> doesn't define.
// #undef pid_t

// Define to `unsigned' if <sys/types.h> doesn't define.
// #undef size_t

// Define to `int' if <sys/types.h> doesn't define.
// #undef uid_t

// Define to `unsigned int' or other type that is 32 bit.
#define UINT32_T unsigned int

// Define to `int' if <sys/types.h> doesn't define.
// #undef gid_t

// Define to `long' if <sys/types.h> doesn't define.
// #undef ino_t

// Define if you have the nanosleep() function.
// #undef HAVE_NANOSLEEP

// Define if you can safely include both <sys/time.h> and <sys/select.h>.
// #undef SYS_SELECT_WITH_SYS_TIME

// Define if struct sigcontext is present
#define HAVE_SIGCONTEXT

// Define if toupper/tolower only work on lower/uppercase characters
// #define BROKEN_TOUPPER

// Define if tgetstr() has a second argument that is (char *)
// #undef TGETSTR_CHAR_P

// Define if you have the sigset() function.
// #undef HAVE_SIGSET

// Define if you have the setpgid() function.
// #undef HAVE_SETPGID

// Define if you have the setsid() function.
// #undef HAVE_SETSID

// Define if you have the sigset() function.
// #undef HAVE_SIGSET

#define TGETENT_ZERO_ERR
#define HAVE_GETCWD
#define HAVE_STRCSPN
#define HAVE_STRTOL
#define HAVE_TGETENT
#define HAVE_MEMSET
#define HAVE_STRERROR
#define HAVE_FCHOWN
#define HAVE_RENAME
#define HAVE_QSORT
#define HAVE_FSYNC
#define HAVE_GETPWUID
#define HAVE_GETPWNAM
#define	HAVE_STDLIB_H
#define	HAVE_STRING_H
#define	HAVE_ERRNO_H
#define HAVE_OPENDIR
#define HAVE_PUTENV
#define HAVE_SETENV
#define HAVE_SETJMP_H
#define HAVE_MATH_H
#define HAVE_GETTIMEOFDAY
#define HAVE_PWD_H
#define HAVE_NETDB_H
#define	HAVE_DIRENT_H

#undef	HAVE_SYS_NDIR_H
#undef	HAVE_SYS_DIR_H
#undef	HAVE_NDIR_H
#undef	HAVE_SYS_WAIT_H
#undef	HAVE_UNION_WAIT
#undef  HAVE_SYS_SELECT_H
#undef  HAVE_SYS_UTSNAME_H
#undef  HAVE_SYS_SYSTEMINFO_H
#undef  HAVE_TERMCAP_H
#undef	HAVE_SGTTY_H
#undef	HAVE_SYS_IOCTL_H
#undef	HAVE_TERMIO_H
#undef	HAVE_STROPTS_H
#undef	HAVE_SYS_STREAM_H
#undef	HAVE_SYS_PTEM_H
#undef	HAVE_TERMIOS_H
#undef	HAVE_LIBC_H
#undef	HAVE_SYS_STATFS_H
#undef	HAVE_SYS_POLL_H
#undef  HAVE_FCHDIR
#undef  HAVE_LSTAT
#undef  HAVE_STDINT_H

// Default features
#define FEAT_IPV6
#define FEAT_XTERM_SAVE

// Hardware specific
#if defined(__VAX) || defined(VAX)
#undef  HAVE_GETTIMEOFDAY
#undef  HAVE_USLEEP
#undef  HAVE_STRCASECMP
#undef  HAVE_STRINGS_H
#undef  HAVE_SIGSETJMP
#undef  HAVE_ISNAN
#undef  HAVE_XOS_R_H
#define HAVE_NO_LONG_LONG
#define VIM_SIZEOF_INT  4
#define VIM_SIZEOF_LONG 4
#define LONG_LONG_MIN  (-2147483647-1)
#define LONG_LONG_MAX  (2147483647)
#define ULONG_LONG_MAX (4294967295U)

#else // ALPHA, IA64, X86_64
#define HAVE_GETTIMEOFDAY
#define HAVE_USLEEP
#define HAVE_STRCASECMP
#define HAVE_STRINGS_H
#define HAVE_SIGSETJMP
#define HAVE_ISNAN
#undef  HAVE_XOS_R_H
#undef  HAVE_NO_LONG_LONG
#define VIM_SIZEOF_INT  4
#define VIM_SIZEOF_LONG 8
#define LONG_LONG_MIN  (-9223372036854775807-1)
#define LONG_LONG_MAX  (9223372036854775807)
#define ULONG_LONG_MAX (18446744073709551615U)

#if defined(__x86_64) || defined(__x86_64__)
#if !defined(X86_64)
#define X86_64
#endif
#define HAVE_ISNAN
#define HAVE_ISINF
#define HAVE_XOS_R_H
#endif
#endif

// Compiler specific
#if defined(VAXC) || defined(__VAXC)
#undef  HAVE_SELECT
#undef  HAVE_FCNTL_H
#undef  HAVE_UNISTD_H
#undef  HAVE_SYS_TIME_H
#undef  HAVE_LOCALE_H
#define BROKEN_LOCALE
#undef  DYNAMIC_ICONV
#undef	HAVE_STRFTIME
#else   // DECC
#define HAVE_SELECT
#define HAVE_FCNTL_H
#define HAVE_UNISTD_H 1
#define HAVE_SYS_TIME_H
#define HAVE_LOCALE_H
#define BROKEN_LOCALE
#undef  DYNAMIC_ICONV
#define	HAVE_STRFTIME
#endif

#if defined(USE_ICONV)
#define HAVE_ICONV_H
#define HAVE_ICONV
#else
#undef HAVE_ICONV_H
#undef HAVE_ICONV
#endif

// GUI support defines
#if defined(FEAT_GUI_MOTIF) || defined(FEAT_GUI_GTK)
#define X_INCLUDE_GRP_H		// To use getgrgid
#define XUSE_MTSAFE_API
#define HAVE_X11
#define WANT_X11
#ifdef  HAVE_XPM
#define HAVE_X11_XPM_H
#endif
#define USE_FONTSET
#undef  X_LOCALE
#endif
