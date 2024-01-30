/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

#ifndef VIM__H
# define VIM__H

#include "protodef.h"

// _WIN32 is defined as 1 when the compilation target is 32-bit or 64-bit.
// Note: If you want to check for 64-bit use the _WIN64 macro.
#if defined(WIN32) || defined(_WIN32)
# define MSWIN
#endif

#if defined(MSWIN) && !defined(PROTO)
# include <io.h>
#endif

// ============ the header file puzzle: order matters =========

#ifdef HAVE_CONFIG_H	// GNU autoconf (or something else) was here
# include "auto/config.h"
# define HAVE_PATHDEF

/*
 * Check if configure correctly managed to find sizeof(int).  If this failed,
 * it becomes zero.  This is likely a problem of not being able to run the
 * test program.  Other items from configure may also be wrong then!
 */
# if (VIM_SIZEOF_INT == 0)
#  error configure did not run properly.  Check auto/config.log.
# endif

# if (defined(__linux__) && !defined(__ANDROID__)) || defined(__CYGWIN__)
// Needed for strptime().  Needs to be done early, since header files can
// include other header files and end up including time.h, where these symbols
// matter for Vim.
// 700 is needed for mkdtemp().
#  ifndef _XOPEN_SOURCE
#   define _XOPEN_SOURCE    700

// On old systems, defining _XOPEN_SOURCE causes _BSD_SOURCE, _SVID_SOURCE
// and/or // _DEFAULT_SOURCE not to be defined, so do that here.  Those are
// needed to include nanosecond-resolution timestamps in struct stat.  On new
// systems, _DEFAULT_SOURCE is needed to avoid warning messages about using
// deprecated _BSD_SOURCE or _SVID_SOURCE.
#   ifndef _BSD_SOURCE
#    define _BSD_SOURCE 1
#   endif
#   ifndef _SVID_SOURCE
#    define _SVID_SOURCE 1
#   endif
#   ifndef _DEFAULT_SOURCE
#    define _DEFAULT_SOURCE 1
#   endif
#  endif
# endif

/*
 * Cygwin may have fchdir() in a newer release, but in most versions it
 * doesn't work well and avoiding it keeps the binary backward compatible.
 */
# if defined(__CYGWIN32__) && defined(HAVE_FCHDIR)
#  undef HAVE_FCHDIR
# endif

// We may need to define the uint32_t on non-Unix system, but using the same
// identifier causes conflicts.  Therefore use UINT32_T.
# define UINT32_TYPEDEF uint32_t
#endif

// for INT_MAX, LONG_MAX et al.
#include <limits.h>

#if !defined(UINT32_TYPEDEF)
# if defined(uint32_t)  // this doesn't catch typedefs, unfortunately
#  define UINT32_TYPEDEF uint32_t
# else
  // Fall back to assuming unsigned int is 32 bit.  If this is wrong then the
  // test in blowfish.c will fail.
#  define UINT32_TYPEDEF unsigned int
# endif
#endif

// user ID of root is usually zero, but not for everybody
#ifdef __TANDEM
# ifndef _TANDEM_SOURCE
#  define _TANDEM_SOURCE
# endif
# include <floss.h>
# define ROOT_UID 65535
# define OLDXAW
# if (_TANDEM_ARCH_ == 2 && __H_Series_RVU >= 621)
#  define SA_ONSTACK_COMPATIBILITY
# endif
#else
# define ROOT_UID 0
#endif

/* Include MAC_OS_X_VERSION_* macros */
#ifdef HAVE_AVAILABILITYMACROS_H
# include <AvailabilityMacros.h>
#endif

/*
 * MACOS_X	    compiling for Mac OS X
 * MACOS_X_DARWIN   integrating the darwin feature into MACOS_X
 */
#if defined(MACOS_X_DARWIN) && !defined(MACOS_X)
# define MACOS_X
#endif
// Unless made through the Makefile enforce GUI on Mac
#if defined(MACOS_X) && !defined(HAVE_CONFIG_H)
# define UNIX
#endif

#if defined(FEAT_GUI_MOTIF) \
    || defined(FEAT_GUI_GTK) \
    || defined(FEAT_GUI_HAIKU) \
    || defined(FEAT_GUI_MSWIN) \
    || defined(FEAT_GUI_PHOTON)
# if !defined(FEAT_GUI) && !defined(NO_X11_INCLUDES)
#  define FEAT_GUI
# endif
#endif

// Check support for rendering options
#ifdef FEAT_GUI
# if defined(FEAT_DIRECTX)
#  define FEAT_RENDER_OPTIONS
# endif
#endif

/*
 * VIM_SIZEOF_INT is used in feature.h, and the system-specific included files
 * need items from feature.h.  Therefore define VIM_SIZEOF_INT here.
 */
#ifdef MSWIN
# define VIM_SIZEOF_INT 4
#endif

#ifdef AMIGA
  // Be conservative about sizeof(int). It could be 4 too.
# ifndef FEAT_GUI_GTK	// avoid problems when generating prototypes
#  ifdef __GNUC__
#   define VIM_SIZEOF_INT	4
#  else
#   define VIM_SIZEOF_INT	2
#  endif
# endif
#endif
#if defined(MACOS_X) && !defined(HAVE_CONFIG_H)
#  define VIM_SIZEOF_INT __SIZEOF_INT__
#endif

#if VIM_SIZEOF_INT < 4 && !defined(PROTO)
# error Vim only works with 32 bit int or larger
#endif

/*
 * #defines for optionals and features
 * Also defines FEAT_TINY, FEAT_NORMAL, etc. when FEAT_HUGE is defined.
 */
#include "feature.h"

#if defined(MACOS_X_DARWIN)
# if defined(FEAT_NORMAL) && !defined(FEAT_CLIPBOARD)
#  define FEAT_CLIPBOARD
# endif
# if defined(FEAT_HUGE) && !defined(FEAT_SOUND) && \
	defined(__clang_major__) && __clang_major__ >= 7 && \
	defined(MAC_OS_X_VERSION_MIN_REQUIRED) && MAC_OS_X_VERSION_MIN_REQUIRED >= 1060
#  define FEAT_SOUND
# endif
# if defined(FEAT_SOUND)
#  define FEAT_SOUND_MACOSX
# endif
#endif

// +x11 is only enabled when it's both available and wanted.
#if defined(HAVE_X11) && defined(WANT_X11)
# define FEAT_X11
#endif

#ifdef NO_X11_INCLUDES
    // In os_mac_conv.c and os_macosx.m NO_X11_INCLUDES is defined to avoid
    // X11 headers.  Disable all X11 related things to avoid conflicts.
# ifdef FEAT_X11
#  undef FEAT_X11
# endif
# ifdef FEAT_GUI_X11
#  undef FEAT_GUI_X11
# endif
# ifdef FEAT_XCLIPBOARD
#  undef FEAT_XCLIPBOARD
# endif
# ifdef FEAT_GUI_MOTIF
#  undef FEAT_GUI_MOTIF
# endif
# ifdef FEAT_GUI_GTK
#  undef FEAT_GUI_GTK
# endif
# ifdef FEAT_BEVAL_TIP
#  undef FEAT_BEVAL_TIP
# endif
# ifdef FEAT_XIM
#  undef FEAT_XIM
# endif
# ifdef FEAT_CLIENTSERVER
#  undef FEAT_CLIENTSERVER
# endif
#endif

// The Mac conversion stuff doesn't work under X11.
#if defined(MACOS_X_DARWIN)
# define MACOS_CONVERT
#endif

// Can't use "PACKAGE" here, conflicts with a Perl include file.
#ifndef VIMPACKAGE
# define VIMPACKAGE	"vim"
#endif

/*
 * Find out if function definitions should include argument types
 */
#ifdef AZTEC_C
# include <functions.h>
#endif

#ifdef SASC
# include <clib/exec_protos.h>
#endif

#ifdef _DCC
# include <clib/exec_protos.h>
#endif

#ifdef __HAIKU__
# include "os_haiku.h"
# define __ARGS(x)  x
#endif

#if (defined(UNIX) || defined(VMS)) \
	&& (!defined(MACOS_X) || defined(HAVE_CONFIG_H))
# include "os_unix.h"	    // bring lots of system header files
#else
  // For all non-Unix systems: use old-fashioned signal().
# define mch_signal(signum, sighandler) signal(signum, sighandler)
#endif

// Mark unused function arguments with UNUSED, so that gcc -Wunused-parameter
// can be used to check for mistakes.
#ifndef UNUSED
# if defined(HAVE_ATTRIBUTE_UNUSED) || defined(__MINGW32__)
#  define UNUSED __attribute__((unused))
# else
#  if defined __has_attribute
#   if __has_attribute(unused)
#    define UNUSED __attribute__((unused))
#   endif
#  endif
# endif
# ifndef UNUSED
#  define UNUSED
# endif
#endif

// Used to check for "sun", "__sun" is used by newer compilers.
#if defined(__sun)
# define SUN_SYSTEM
#endif

// If we're compiling in C++ (currently only KVim), the system
// headers must have the correct prototypes or nothing will build.
// Conversely, our prototypes might clash due to throw() specifiers and
// cause compilation failures even though the headers are correct.  For
// a concrete example, gcc-3.2 enforces exception specifications, and
// glibc-2.2.5 has them in their system headers.
#if !defined(__cplusplus) && defined(UNIX) \
	&& !defined(MACOS_X) // MACOS_X doesn't yet support osdef.h
# include "auto/osdef.h"	// bring missing declarations in
#endif

#ifdef AMIGA
# include "os_amiga.h"
#endif

#ifdef MSWIN
# include "os_win32.h"
#endif

#if defined(MACOS_X)
# include "os_mac.h"
#endif

#ifdef __QNX__
# include "os_qnx.h"
#endif

#ifdef X_LOCALE
# include <X11/Xlocale.h>
#else
# ifdef HAVE_LOCALE_H
#  include <locale.h>
# endif
#endif

/*
 * Maximum length of a path (for non-unix systems) Make it a bit long, to stay
 * on the safe side.  But not too long to put on the stack.
 */
#ifndef MAXPATHL
# ifdef MAXPATHLEN
#  define MAXPATHL  MAXPATHLEN
# else
#  define MAXPATHL  256
# endif
#endif
#ifdef BACKSLASH_IN_FILENAME
# define PATH_ESC_CHARS ((char_u *)" \t\n*?[{`%#'\"|!<")
# define BUFFER_ESC_CHARS ((char_u *)" \t\n*?[`%#'\"|!<")
#else
# ifdef VMS
    // VMS allows a lot of characters in the file name
#  define PATH_ESC_CHARS ((char_u *)" \t\n*?{`\\%#'\"|!")
#  define SHELL_ESC_CHARS ((char_u *)" \t\n*?{`\\%#'|!()&")
# else
#  define PATH_ESC_CHARS ((char_u *)" \t\n*?[{`$\\%#'\"|!<")
#  define SHELL_ESC_CHARS ((char_u *)" \t\n*?[{`$\\%#'\"|!<>();&")
# endif
#  define BUFFER_ESC_CHARS ((char_u *)" \t\n*?[`$\\%#'\"|!<")
#endif

// length of a buffer to store a number in ASCII (64 bits binary + NUL)
#define NUMBUFLEN 65

// flags for vim_str2nr()
#define STR2NR_BIN  0x01
#define STR2NR_OCT  0x02
#define STR2NR_HEX  0x04
#define STR2NR_OOCT 0x08    // Octal with prefix "0o": 0o777
#define STR2NR_ALL (STR2NR_BIN + STR2NR_OCT + STR2NR_HEX + STR2NR_OOCT)
#define STR2NR_NO_OCT (STR2NR_BIN + STR2NR_HEX + STR2NR_OOCT)

#define STR2NR_FORCE 0x80   // only when ONE of the above is used

#define STR2NR_QUOTE 0x10   // ignore embedded single quotes

/*
 * Shorthand for unsigned variables. Many systems, but not all, have u_char
 * already defined, so we use char_u to avoid trouble.
 */
typedef unsigned char	char_u;
typedef unsigned short	short_u;
typedef unsigned int	int_u;

// Make sure long_u is big enough to hold a pointer.
// On Win64, longs are 32 bits and pointers are 64 bits.
// For printf() and scanf(), we need to take care of long_u specifically.
#ifdef _WIN64
typedef unsigned __int64	long_u;
typedef		 __int64	long_i;
# define SCANF_HEX_LONG_U       "%llx"
# define SCANF_DECIMAL_LONG_U   "%llu"
# define PRINTF_HEX_LONG_U      "0x%llx"
#else
typedef unsigned long		long_u;
typedef		 long		long_i;
# define SCANF_HEX_LONG_U       "%lx"
# define SCANF_DECIMAL_LONG_U   "%lu"
# define PRINTF_HEX_LONG_U      "0x%lx"
#endif
#define PRINTF_DECIMAL_LONG_U SCANF_DECIMAL_LONG_U

/*
 * Only systems which use configure will have SIZEOF_OFF_T and VIM_SIZEOF_LONG
 * defined, which is ok since those are the same systems which can have
 * varying sizes for off_t.  The other systems will continue to use "%ld" to
 * print off_t since off_t is simply a typedef to long for them.
 */
#if defined(SIZEOF_OFF_T) && (SIZEOF_OFF_T > VIM_SIZEOF_LONG)
# define LONG_LONG_OFF_T
#endif

/*
 * We use 64-bit file functions here, if available.  E.g. ftello() returns
 * off_t instead of long, which helps if long is 32 bit and off_t is 64 bit.
 * We assume that when fseeko() is available then ftello() is too.
 * Note that Windows has different function names.
 */
#if defined(MSWIN) && !defined(PROTO)
typedef __int64 off_T;
# ifdef __MINGW32__
#  define vim_lseek lseek64
#  define vim_fseek fseeko64
#  define vim_ftell ftello64
# else
#  define vim_lseek _lseeki64
#  define vim_fseek _fseeki64
#  define vim_ftell _ftelli64
# endif
#else
# ifdef PROTO
typedef long off_T;
# else
typedef off_t off_T;
# endif
# ifdef HAVE_FSEEKO
#  define vim_lseek lseek
#  define vim_ftell ftello
#  define vim_fseek fseeko
# else
#  define vim_lseek lseek
#  define vim_ftell ftell
#  define vim_fseek(a, b, c)	fseek(a, (long)b, c)
# endif
#endif

/*
 * The characters and attributes cached for the screen.
 */
typedef char_u schar_T;
typedef unsigned short sattr_T;
#define MAX_TYPENR 65535

/*
 * The u8char_T can hold one decoded UTF-8 character.
 * We use 32 bits, since some Asian characters don't fit in 16 bits.
 */
typedef unsigned int u8char_T;	// int is 32 bits or more

/*
 * The vimlong_T has sizeof(vimlong_T) >= 2 * sizeof(int).
 * One use is simple handling of overflow in int calculations.
 */
#if defined(VMS) && defined(VAX)
// unsupported compiler
typedef long      vimlong_T;
#else
typedef long long vimlong_T;
#endif

#ifndef UNIX		    // For Unix this is included in os_unix.h
# include <stdio.h>
# include <ctype.h>
#endif

#include "ascii.h"
#include "keymap.h"
#include "termdefs.h"
#include "macros.h"

#ifdef LATTICE
# include <sys/types.h>
# include <sys/stat.h>
#endif
#ifdef _DCC
# include <sys/stat.h>
#endif
#if defined(MSWIN)
# include <sys/stat.h>
#endif

#if defined(HAVE_ERRNO_H) || defined(MSWIN)
# include <errno.h>
#endif

/*
 * Allow other (non-unix) systems to configure themselves now
 * These are also in os_unix.h, because osdef.sh needs them there.
 */
#ifndef UNIX
// Note: Some systems need both string.h and strings.h (Savage).  If the
// system can't handle this, define NO_STRINGS_WITH_STRING_H.
# ifdef HAVE_STRING_H
#  include <string.h>
# endif
# if defined(HAVE_STRINGS_H) && !defined(NO_STRINGS_WITH_STRING_H)
#  include <strings.h>
# endif
# ifdef HAVE_STAT_H
#  include <stat.h>
# endif
# ifdef HAVE_STDLIB_H
#  include <stdlib.h>
# endif
#endif // NON-UNIX

#include <assert.h>

#ifdef HAVE_STDINT_H
# include <stdint.h>
#endif
#ifdef HAVE_INTTYPES_H
# include <inttypes.h>
#endif
#ifdef HAVE_WCTYPE_H
# include <wctype.h>
#endif
#include <stdarg.h>
// older compilers do not define va_copy
#ifndef va_copy
# define va_copy(dst, src)	((dst) = (src))
#endif

// for offsetof()
#include <stddef.h>

#if defined(HAVE_SYS_SELECT_H) && \
	(!defined(HAVE_SYS_TIME_H) || defined(SYS_SELECT_WITH_SYS_TIME))
# include <sys/select.h>
#endif

#ifndef HAVE_SELECT
# ifdef HAVE_SYS_POLL_H
#  include <sys/poll.h>
# elif defined(MSWIN)
#  define HAVE_SELECT
# else
#  ifdef HAVE_POLL_H
#   include <poll.h>
#  endif
# endif
#endif

#ifdef HAVE_SODIUM
# include <sodium.h>
#endif

// ================ end of the header file puzzle ===============

/*
 * For dynamically loaded imm library. Currently, only for Win32.
 */
#ifdef DYNAMIC_IME
# ifndef FEAT_MBYTE_IME
#  define FEAT_MBYTE_IME
# endif
#endif

/*
 * For dynamically loaded gettext library.  Currently, only for Win32.
 */
#ifdef DYNAMIC_GETTEXT
# ifndef FEAT_GETTEXT
#  define FEAT_GETTEXT
# endif
// These are in os_win32.c
extern char *(*dyn_libintl_gettext)(const char *msgid);
extern char *(*dyn_libintl_ngettext)(const char *msgid, const char *msgid_plural, unsigned long n);
extern char *(*dyn_libintl_bindtextdomain)(const char *domainname, const char *dirname);
extern char *(*dyn_libintl_bind_textdomain_codeset)(const char *domainname, const char *codeset);
extern char *(*dyn_libintl_textdomain)(const char *domainname);
extern int (*dyn_libintl_wputenv)(const wchar_t *envstring);
#endif


/*
 * The _() stuff is for using gettext().  It is a no-op when libintl.h is not
 * found or the +multilang feature is disabled.
 * Use NGETTEXT(single, multi, number) to get plural behavior:
 * - single - message for singular form
 * - multi  - message for plural form
 * - number - the count
 */
#ifdef FEAT_GETTEXT
# ifdef DYNAMIC_GETTEXT
#  define _(x) (*dyn_libintl_gettext)((char *)(x))
#  define NGETTEXT(x, xs, n) (*dyn_libintl_ngettext)((char *)(x), (char *)(xs), (n))
#  define N_(x) x
#  define bindtextdomain(domain, dir) (*dyn_libintl_bindtextdomain)((domain), (dir))
#  define bind_textdomain_codeset(domain, codeset) (*dyn_libintl_bind_textdomain_codeset)((domain), (codeset))
#  if !defined(HAVE_BIND_TEXTDOMAIN_CODESET)
#   define HAVE_BIND_TEXTDOMAIN_CODESET 1
#  endif
#  define textdomain(domain) (*dyn_libintl_textdomain)(domain)
#  define libintl_wputenv(envstring) (*dyn_libintl_wputenv)(envstring)
# else
#  include <libintl.h>
#  define _(x) gettext((char *)(x))
#  define NGETTEXT(x, xs, n) ngettext((x), (xs), (n))
#  ifdef gettext_noop
#   define N_(x) gettext_noop(x)
#  else
#   define N_(x) x
#  endif
# endif
#else
# define _(x) ((char *)(x))
# define NGETTEXT(x, xs, n) (((n) == 1) ? (char *)(x) : (char *)(xs))
# define N_(x) x
# ifdef bindtextdomain
#  undef bindtextdomain
# endif
# define bindtextdomain(x, y) // empty
# ifdef bind_textdomain_codeset
#  undef bind_textdomain_codeset
# endif
# define bind_textdomain_codeset(x, y) // empty
# ifdef textdomain
#  undef textdomain
# endif
# define textdomain(x) // empty
#endif

/*
 * Flags for update_screen().
 * The higher the value, the higher the priority.
 */
#define UPD_VALID_NO_UPDATE	 5  // no new changes, keep the command line if
				    // possible
#define UPD_VALID		10  // buffer not changed, or changes marked
				    // with b_mod_*
#define UPD_INVERTED		20  // redisplay inverted part that changed
#define UPD_INVERTED_ALL	25  // redisplay whole inverted part
#define UPD_REDRAW_TOP		30  // display first w_upd_rows screen lines
#define UPD_SOME_VALID		35  // like UPD_NOT_VALID but may scroll
#define UPD_NOT_VALID		40  // buffer needs complete redraw
#define UPD_CLEAR		50  // screen messed up, clear it

// flags for screen_line()
#define SLF_RIGHTLEFT	1
#define SLF_POPUP	2

#define MB_FILLER_CHAR '<'  // character used when a double-width character
			    // doesn't fit.

/*
 * Flags for w_valid.
 * These are set when something in a window structure becomes invalid, except
 * when the cursor is moved.  Call check_cursor_moved() before testing one of
 * the flags.
 * These are reset when that thing has been updated and is valid again.
 *
 * Every function that invalidates one of these must call one of the
 * invalidate_* functions.
 *
 * w_valid is supposed to be used only in screen.c.  From other files, use the
 * functions that set or reset the flags.
 *
 * VALID_BOTLINE    VALID_BOTLINE_AP
 *     on		on		w_botline valid
 *     off		on		w_botline approximated
 *     off		off		w_botline not valid
 *     on		off		not possible
 */
#define VALID_WROW	0x01	// w_wrow (window row) is valid
#define VALID_WCOL	0x02	// w_wcol (window col) is valid
#define VALID_VIRTCOL	0x04	// w_virtcol (file col) is valid
#define VALID_CHEIGHT	0x08	// w_cline_height and w_cline_folded valid
#define VALID_CROW	0x10	// w_cline_row is valid
#define VALID_BOTLINE	0x20	// w_botine and w_empty_rows are valid
#define VALID_BOTLINE_AP 0x40	// w_botine is approximated
#define VALID_TOPLINE	0x80	// w_topline is valid (for cursor position)

// Values for w_popup_flags.
#define POPF_IS_POPUP	0x01	// this is a popup window
#define POPF_HIDDEN	0x02	// popup is not displayed
#define POPF_HIDDEN_FORCE 0x04	// popup is explicitly set to not be displayed
#define POPF_CURSORLINE	0x08	// popup is highlighting at the cursorline
#define POPF_ON_CMDLINE	0x10	// popup overlaps command line
#define POPF_DRAG	0x20	// popup can be moved by dragging border
#define POPF_DRAGALL	0x40	// popup can be moved by dragging everywhere
#define POPF_RESIZE	0x80	// popup can be resized by dragging
#define POPF_MAPPING	0x100	// mapping keys
#define POPF_INFO	0x200	// used for info of popup menu
#define POPF_INFO_MENU	0x400	// align info popup with popup menu
#define POPF_POSINVERT	0x800	// vertical position can be inverted

// flags used in w_popup_handled
#define POPUP_HANDLED_1	    0x01    // used by mouse_find_win()
#define POPUP_HANDLED_2	    0x02    // used by popup_do_filter()
#define POPUP_HANDLED_3	    0x04    // used by popup_check_cursor_pos()
#define POPUP_HANDLED_4	    0x08    // used by may_update_popup_mask()
#define POPUP_HANDLED_5	    0x10    // used by update_popups()

/*
 * Terminal highlighting attribute bits.
 * Attributes above HL_ALL are used for syntax highlighting.
 */
#define HL_NORMAL		0x00
#define HL_INVERSE		0x01
#define HL_BOLD			0x02
#define HL_ITALIC		0x04
#define HL_UNDERLINE		0x08
#define HL_UNDERCURL		0x10
#define HL_UNDERDOUBLE		0x20
#define HL_UNDERDOTTED		0x40
#define HL_UNDERDASHED		0x80
#define HL_STANDOUT		0x100
#define HL_NOCOMBINE		0x200
#define HL_STRIKETHROUGH	0x400
#define HL_ALL			0x7ff

// special attribute addition: Put message in history
#define MSG_HIST		0x1000

/*
 * Values for State.
 *
 * The lower bits up to 0x80 are used to distinguish normal/visual/op_pending
 * /cmdline/insert/replace/terminal mode.  This is used for mapping.  If none
 * of these bits are set, no mapping is done.  See the comment above do_map().
 * The upper bits are used to distinguish between other states and variants of
 * the base modes.
 */
#define MODE_NORMAL	0x01	// Normal mode, command expected
#define MODE_VISUAL	0x02	// Visual mode - use get_real_state()
#define MODE_OP_PENDING	0x04	// Normal mode, operator is pending - use
				// get_real_state()
#define MODE_CMDLINE	0x08	// Editing the command line
#define MODE_INSERT	0x10	// Insert mode, also for Replace mode
#define MODE_LANGMAP	0x20	// Language mapping, can be combined with
				// MODE_INSERT and MODE_CMDLINE
#define MODE_SELECT	0x40	// Select mode, use get_real_state()
#define MODE_TERMINAL	0x80	// Terminal mode

#define MAP_ALL_MODES	0xff    // all mode bits used for mapping

#define REPLACE_FLAG	0x100	// Replace mode flag
#define MODE_REPLACE	(REPLACE_FLAG | MODE_INSERT)
#define VREPLACE_FLAG	0x200	// Virtual-replace mode flag
#define MODE_VREPLACE	(REPLACE_FLAG | VREPLACE_FLAG | MODE_INSERT)
#define MODE_LREPLACE	(REPLACE_FLAG | MODE_LANGMAP)

#define MODE_NORMAL_BUSY (0x1000 | MODE_NORMAL)
				// Normal mode, busy with a command
#define MODE_HITRETURN	(0x2000 | MODE_NORMAL)
				// waiting for return or command
#define MODE_ASKMORE	0x3000	// Asking if you want --more--
#define MODE_SETWSIZE	0x4000	// window size has changed
#define MODE_EXTERNCMD	0x5000	// executing an external command
#define MODE_SHOWMATCH	(0x6000 | MODE_INSERT) // show matching paren
#define MODE_CONFIRM	0x7000	// ":confirm" prompt
#define MODE_ALL	0xffff

#define MODE_MAX_LENGTH	4	// max mode length used by get_mode(),
				// including the terminating NUL

// directions
#define FORWARD			1
#define BACKWARD		(-1)
#define FORWARD_FILE		3
#define BACKWARD_FILE		(-3)

// return values for functions
#if !(defined(OK) && (OK == 1))
// OK already defined to 1 in MacOS X curses, skip this
# define OK			1
#endif
#define FAIL			0
#define NOTDONE			2   // not OK or FAIL but skipped

// flags for b_flags
#define BF_RECOVERED	0x01	// buffer has been recovered
#define BF_CHECK_RO	0x02	// need to check readonly when loading file
				// into buffer (set by ":e", may be reset by
				// ":buf"
#define BF_NEVERLOADED	0x04	// file has never been loaded into buffer,
				// many variables still need to be set
#define BF_NOTEDITED	0x08	// Set when file name is changed after
				// starting to edit, reset when file is
				// written out.
#define BF_NEW		0x10	// file didn't exist when editing started
#define BF_NEW_W	0x20	// Warned for BF_NEW and file created
#define BF_READERR	0x40	// got errors while reading the file
#define BF_DUMMY	0x80	// dummy buffer, only used internally
#define BF_PRESERVED	0x100	// ":preserve" was used
#define BF_SYN_SET	0x200	// 'syntax' option was set
#define BF_NO_SEA	0x400	// no swap_exists_action (ATTENTION prompt)

// Mask to check for flags that prevent normal writing
#define BF_WRITE_MASK	(BF_NOTEDITED + BF_NEW + BF_READERR)

/*
 * values for xp_context when doing command line completion
 */
#define EXPAND_UNSUCCESSFUL	(-2)
#define EXPAND_OK		(-1)
#define EXPAND_NOTHING		0
#define EXPAND_COMMANDS		1
#define EXPAND_FILES		2
#define EXPAND_DIRECTORIES	3
#define EXPAND_SETTINGS		4
#define EXPAND_BOOL_SETTINGS	5
#define EXPAND_TAGS		6
#define EXPAND_OLD_SETTING	7
#define EXPAND_HELP		8
#define EXPAND_BUFFERS		9
#define EXPAND_EVENTS		10
#define EXPAND_MENUS		11
#define EXPAND_SYNTAX		12
#define EXPAND_HIGHLIGHT	13
#define EXPAND_AUGROUP		14
#define EXPAND_USER_VARS	15
#define EXPAND_MAPPINGS		16
#define EXPAND_TAGS_LISTFILES	17
#define EXPAND_FUNCTIONS	18
#define EXPAND_USER_FUNC	19
#define EXPAND_EXPRESSION	20
#define EXPAND_MENUNAMES	21
#define EXPAND_USER_COMMANDS	22
#define EXPAND_USER_CMD_FLAGS	23
#define EXPAND_USER_NARGS	24
#define EXPAND_USER_COMPLETE	25
#define EXPAND_ENV_VARS		26
#define EXPAND_LANGUAGE		27
#define EXPAND_COLORS		28
#define EXPAND_COMPILER		29
#define EXPAND_USER_DEFINED	30
#define EXPAND_USER_LIST	31
#define EXPAND_SHELLCMD		32
#define EXPAND_CSCOPE		33
#define EXPAND_SIGN		34
#define EXPAND_PROFILE		35
#define EXPAND_BEHAVE		36
#define EXPAND_FILETYPE		37
#define EXPAND_FILES_IN_PATH	38
#define EXPAND_OWNSYNTAX	39
#define EXPAND_LOCALES		40
#define EXPAND_HISTORY		41
#define EXPAND_USER		42
#define EXPAND_SYNTIME		43
#define EXPAND_USER_ADDR_TYPE	44
#define EXPAND_PACKADD		45
#define EXPAND_MESSAGES		46
#define EXPAND_MAPCLEAR		47
#define EXPAND_ARGLIST		48
#define EXPAND_DIFF_BUFFERS	49
#define EXPAND_DISASSEMBLE	50
#define EXPAND_BREAKPOINT	51
#define EXPAND_SCRIPTNAMES	52
#define EXPAND_RUNTIME		53
#define EXPAND_STRING_SETTING	54
#define EXPAND_SETTING_SUBTRACT	55
#define EXPAND_ARGOPT		56
#define EXPAND_TERMINALOPT	57
#define EXPAND_KEYMAP		58

// Values for exmode_active (0 is no exmode)
#define EXMODE_NORMAL		1
#define EXMODE_VIM		2

// Values for nextwild() and ExpandOne().  See ExpandOne() for meaning.
#define WILD_FREE		1
#define WILD_EXPAND_FREE	2
#define WILD_EXPAND_KEEP	3
#define WILD_NEXT		4
#define WILD_PREV		5
#define WILD_ALL		6
#define WILD_LONGEST		7
#define WILD_ALL_KEEP		8
#define WILD_CANCEL		9
#define WILD_APPLY		10
#define WILD_PAGEUP		11
#define WILD_PAGEDOWN		12

#define WILD_LIST_NOTFOUND	    0x01
#define WILD_HOME_REPLACE	    0x02
#define WILD_USE_NL		    0x04
#define WILD_NO_BEEP		    0x08
#define WILD_ADD_SLASH		    0x10
#define WILD_KEEP_ALL		    0x20
#define WILD_SILENT		    0x40
#define WILD_ESCAPE		    0x80
#define WILD_ICASE		    0x100
#define WILD_ALLLINKS		    0x200
#define WILD_IGNORE_COMPLETESLASH   0x400
#define WILD_NOERROR		    0x800  // sets EW_NOERROR
#define WILD_BUFLASTUSED	    0x1000
#define BUF_DIFF_FILTER		    0x2000

// Flags for expand_wildcards()
#define EW_DIR		0x01	// include directory names
#define EW_FILE		0x02	// include file names
#define EW_NOTFOUND	0x04	// include not found names
#define EW_ADDSLASH	0x08	// append slash to directory name
#define EW_KEEPALL	0x10	// keep all matches
#define EW_SILENT	0x20	// don't print "1 returned" from shell
#define EW_EXEC		0x40	// executable files
#define EW_PATH		0x80	// search in 'path' too
#define EW_ICASE	0x100	// ignore case
#define EW_NOERROR	0x200	// no error for bad regexp
#define EW_NOTWILD	0x400	// add match with literal name if exists
#define EW_KEEPDOLLAR	0x800	// do not escape $, $var is expanded
// Note: mostly EW_NOTFOUND and EW_SILENT are mutually exclusive: EW_NOTFOUND
// is used when executing commands and EW_SILENT for interactive expanding.
#define EW_ALLLINKS	0x1000	// also links not pointing to existing file
#define EW_SHELLCMD	0x2000	// called from expand_shellcmd(), don't check
				// if executable is in $PATH
#define EW_DODOT	0x4000	// also files starting with a dot
#define EW_EMPTYOK	0x8000	// no matches is not an error
#define EW_NOTENV	0x10000	// do not expand environment variables

// Flags for find_file_*() functions.
#define FINDFILE_FILE	0	// only files
#define FINDFILE_DIR	1	// only directories
#define FINDFILE_BOTH	2	// files and directories

#define W_ENDCOL(wp)	((wp)->w_wincol + (wp)->w_width)
#ifdef FEAT_MENU
# define W_WINROW(wp)	((wp)->w_winrow + (wp)->w_winbar_height)
#else
# define W_WINROW(wp)	(wp->w_winrow)
#endif

// Values for the find_pattern_in_path() function args 'type' and 'action':
#define FIND_ANY	1
#define FIND_DEFINE	2
#define CHECK_PATH	3

#define ACTION_SHOW	1
#define ACTION_GOTO	2
#define ACTION_SPLIT	3
#define ACTION_SHOW_ALL	4
#define ACTION_EXPAND	5

#ifdef FEAT_SYN_HL
# define SST_MIN_ENTRIES 150	// minimal size for state stack array
# define SST_MAX_ENTRIES 1000	// maximal size for state stack array
# define SST_FIX_STATES	 7	// size of sst_stack[].
# define SST_DIST	 16	// normal distance between entries
# define SST_INVALID	((synstate_T *)-1)	// invalid syn_state pointer

# define HL_CONTAINED	0x01	// not used on toplevel
# define HL_TRANSP	0x02	// has no highlighting
# define HL_ONELINE	0x04	// match within one line only
# define HL_HAS_EOL	0x08	// end pattern that matches with $
# define HL_SYNC_HERE	0x10	// sync point after this item (syncing only)
# define HL_SYNC_THERE	0x20	// sync point at current line (syncing only)
# define HL_MATCH	0x40	// use match ID instead of item ID
# define HL_SKIPNL	0x80	// nextgroup can skip newlines
# define HL_SKIPWHITE	0x100	// nextgroup can skip white space
# define HL_SKIPEMPTY	0x200	// nextgroup can skip empty lines
# define HL_KEEPEND	0x400	// end match always kept
# define HL_EXCLUDENL	0x800	// exclude NL from match
# define HL_DISPLAY	0x1000	// only used for displaying, not syncing
# define HL_FOLD	0x2000	// define fold
# define HL_EXTEND	0x4000	// ignore a keepend
# define HL_MATCHCONT	0x8000	// match continued from previous line
# define HL_TRANS_CONT	0x10000 // transparent item without contains arg
# define HL_CONCEAL	0x20000 // can be concealed
# define HL_CONCEALENDS	0x40000 // can be concealed
#endif

// Values for 'options' argument in do_search() and searchit()
#define SEARCH_REV    0x01  // go in reverse of previous dir.
#define SEARCH_ECHO   0x02  // echo the search command and handle options
#define SEARCH_MSG    0x0c  // give messages (yes, it's not 0x04)
#define SEARCH_NFMSG  0x08  // give all messages except not found
#define SEARCH_OPT    0x10  // interpret optional flags
#define SEARCH_HIS    0x20  // put search pattern in history
#define SEARCH_END    0x40  // put cursor at end of match
#define SEARCH_NOOF   0x80  // don't add offset to position
#define SEARCH_START 0x100  // start search without col offset
#define SEARCH_MARK  0x200  // set previous context mark
#define SEARCH_KEEP  0x400  // keep previous search pattern
#define SEARCH_PEEK  0x800  // peek for typed char, cancel search
#define SEARCH_COL  0x1000  // start at specified column instead of zero

// Values for find_ident_under_cursor()
#define FIND_IDENT	1	// find identifier (word)
#define FIND_STRING	2	// find any string (WORD)
#define FIND_EVAL	4	// include "->", "[]" and "."
#define FIND_NOERROR	8	// no error when no word found

// Values for file_name_in_line()
#define FNAME_MESS	1	// give error message
#define FNAME_EXP	2	// expand to path
#define FNAME_HYP	4	// check for hypertext link
#define FNAME_INCL	8	// apply 'includeexpr'
#define FNAME_REL	16	// ".." and "./" are relative to the (current)
				// file instead of the current directory
#define FNAME_UNESC	32	// remove backslashes used for escaping

// Values for buflist_getfile()
#define GETF_SETMARK	0x01	// set pcmark before jumping
#define GETF_ALT	0x02	// jumping to alternate file (not buf num)
#define GETF_SWITCH	0x04	// respect 'switchbuf' settings when jumping

// Return values of getfile()
#define GETFILE_ERROR	    1	// normal error
#define GETFILE_NOT_WRITTEN 2	// "not written" error
#define GETFILE_SAME_FILE   0	// success, same file
#define GETFILE_OPEN_OTHER (-1)	// success, opened another file
#define GETFILE_UNUSED	    8
#define GETFILE_SUCCESS(x)  ((x) <= 0)

// Values for buflist_new() flags
#define BLN_CURBUF	1	// may re-use curbuf for new buffer
#define BLN_LISTED	2	// put new buffer in buffer list
#define BLN_DUMMY	4	// allocating dummy buffer
#define BLN_NEW		8	// create a new buffer
#define BLN_NOOPT	16	// don't copy options to existing buffer
#define BLN_DUMMY_OK	32	// also find an existing dummy buffer
#define BLN_REUSE	64	// may re-use number from buf_reuse
#define BLN_NOCURWIN	128	// buffer is not associated with curwin

// Values for in_cinkeys()
#define KEY_OPEN_FORW	0x101
#define KEY_OPEN_BACK	0x102
#define KEY_COMPLETE	0x103	// end of completion

// Used for the first argument of do_map()
#define MAPTYPE_MAP	0
#define MAPTYPE_UNMAP	1
#define MAPTYPE_NOREMAP	2

// Values for "noremap" argument of ins_typebuf().  Also used for
// map->m_noremap and menu->noremap[].
#define REMAP_YES	0	// allow remapping
#define REMAP_NONE	(-1)	// no remapping
#define REMAP_SCRIPT	(-2)	// remap script-local mappings only
#define REMAP_SKIP	(-3)	// no remapping for first char

// Values for mch_call_shell() second argument
#define SHELL_FILTER	1	// filtering text
#define SHELL_EXPAND	2	// expanding wildcards
#define SHELL_COOKED	4	// set term to cooked mode
#define SHELL_DOOUT	8	// redirecting output
#define SHELL_SILENT	16	// don't print error returned by command
#define SHELL_READ	32	// read lines and insert into buffer
#define SHELL_WRITE	64	// write lines from buffer

// Values returned by mch_nodetype()
#define NODE_NORMAL	0	// file or directory, check with mch_isdir()
#define NODE_WRITABLE	1	// something we can write to (character
				// device, fifo, socket, ..)
#define NODE_OTHER	2	// non-writable thing (e.g., block device)

// Values for readfile() flags
#define READ_NEW	0x01	// read a file into a new buffer
#define READ_FILTER	0x02	// read filter output
#define READ_STDIN	0x04	// read from stdin
#define READ_BUFFER	0x08	// read from curbuf (converting stdin)
#define READ_DUMMY	0x10	// reading into a dummy buffer
#define READ_KEEP_UNDO	0x20	// keep undo info
#define READ_FIFO	0x40	// read from fifo or socket
#define READ_NOWINENTER 0x80	// do not trigger BufWinEnter
#define READ_NOFILE	0x100	// do not read a file, do trigger BufReadCmd

// Values for change_indent()
#define INDENT_SET	1	// set indent
#define INDENT_INC	2	// increase indent
#define INDENT_DEC	3	// decrease indent

// Values for flags argument for findmatchlimit()
#define FM_BACKWARD	0x01	// search backwards
#define FM_FORWARD	0x02	// search forwards
#define FM_BLOCKSTOP	0x04	// stop at start/end of block
#define FM_SKIPCOMM	0x08	// skip comments

// Values for action argument for do_buffer() and close_buffer()
#define DOBUF_GOTO	0	// go to specified buffer
#define DOBUF_SPLIT	1	// split window and go to specified buffer
#define DOBUF_UNLOAD	2	// unload specified buffer(s)
#define DOBUF_DEL	3	// delete specified buffer(s) from buflist
#define DOBUF_WIPE	4	// delete specified buffer(s) really
#define DOBUF_WIPE_REUSE 5	// like DOBUF_WIPE and keep number for reuse

// Values for start argument for do_buffer()
#define DOBUF_CURRENT	0	// "count" buffer from current buffer
#define DOBUF_FIRST	1	// "count" buffer from first buffer
#define DOBUF_LAST	2	// "count" buffer from last buffer
#define DOBUF_MOD	3	// "count" mod. buffer from current buffer

// Values for flags argument of do_buffer()
#define DOBUF_FORCEIT	1	// :cmd!
#define DOBUF_NOPOPUP	2	// skip popup window buffers

// Values for sub_cmd and which_pat argument for search_regcomp()
// Also used for which_pat argument for searchit()
#define RE_SEARCH	0	// save/use pat in/from search_pattern
#define RE_SUBST	1	// save/use pat in/from subst_pattern
#define RE_BOTH		2	// save pat in both patterns
#define RE_LAST		2	// use last used pattern if "pat" is NULL

// Second argument for vim_regcomp().
#define RE_MAGIC	1	// 'magic' option
#define RE_STRING	2	// match in string instead of buffer text
#define RE_STRICT	4	// don't allow [abc] without ]
#define RE_AUTO		8	// automatic engine selection

#ifdef FEAT_SYN_HL
// values for reg_do_extmatch
# define REX_SET	1	// to allow \z\(...\),
# define REX_USE	2	// to allow \z\1 et al.
# define REX_ALL	(REX_SET | REX_USE)
#endif

// Return values for fullpathcmp()
// Note: can use (fullpathcmp() & FPC_SAME) to check for equal files
#define FPC_SAME	1	// both exist and are the same file.
#define FPC_DIFF	2	// both exist and are different files.
#define FPC_NOTX	4	// both don't exist.
#define FPC_DIFFX	6	// one of them doesn't exist.
#define FPC_SAMEX	7	// both don't exist and file names are same.

// flags for do_ecmd()
#define ECMD_HIDE	0x01	// don't free the current buffer
#define ECMD_SET_HELP	0x02	// set b_help flag of (new) buffer before
				// opening file
#define ECMD_OLDBUF	0x04	// use existing buffer if it exists
#define ECMD_FORCEIT	0x08	// ! used in Ex command
#define ECMD_ADDBUF	0x10	// don't edit, just add to buffer list
#define ECMD_ALTBUF	0x20	// like ECMD_ADDBUF and set the alternate file
#define ECMD_NOWINENTER	0x40	// do not trigger BufWinEnter

// for lnum argument in do_ecmd()
#define ECMD_LASTL	(linenr_T)0	// use last position in loaded file
#define ECMD_LAST	((linenr_T)-1)	// use last position in all files
#define ECMD_ONE	(linenr_T)1	// use first line

// flags for do_cmdline()
#define DOCMD_VERBOSE	0x01	// included command in error message
#define DOCMD_NOWAIT	0x02	// don't call wait_return() and friends
#define DOCMD_REPEAT	0x04	// repeat exec. until getline() returns NULL
#define DOCMD_KEYTYPED	0x08	// don't reset KeyTyped
#define DOCMD_EXCRESET	0x10	// reset exception environment (for debugging)
#define DOCMD_KEEPLINE  0x20	// keep typed line for repeating with "."
#define DOCMD_RANGEOK	0x40	// can use a range without ":" in Vim9 script

// flags for beginline()
#define BL_WHITE	1	// cursor on first non-white in the line
#define BL_SOL		2	// use 'sol' option
#define BL_FIX		4	// don't leave cursor on a NUL

// flags for mf_sync()
#define MFS_ALL		1	// also sync blocks with negative numbers
#define MFS_STOP	2	// stop syncing when a character is available
#define MFS_FLUSH	4	// flushed file to disk
#define MFS_ZERO	8	// only write block 0

// flags for buf_copy_options()
#define BCO_ENTER	1	// going to enter the buffer
#define BCO_ALWAYS	2	// always copy the options
#define BCO_NOHELP	4	// don't touch the help related options

// flags for do_put()
#define PUT_FIXINDENT	1	// make indent look nice
#define PUT_CURSEND	2	// leave cursor after end of new text
#define PUT_CURSLINE	4	// leave cursor on last line of new text
#define PUT_LINE	8	// put register as lines
#define PUT_LINE_SPLIT	16	// split line for linewise register
#define PUT_LINE_FORWARD 32	// put linewise register below Visual sel.
#define PUT_BLOCK_INNER 64      // in block mode, do not add trailing spaces

// flags for set_indent()
#define SIN_CHANGED	1	// call changed_bytes() when line changed
#define SIN_INSERT	2	// insert indent before existing text
#define SIN_UNDO	4	// save line for undo before changing it

// flags for insertchar()
#define INSCHAR_FORMAT	1	// force formatting
#define INSCHAR_DO_COM	2	// format comments
#define INSCHAR_CTRLV	4	// char typed just after CTRL-V
#define INSCHAR_NO_FEX	8	// don't use 'formatexpr'
#define INSCHAR_COM_LIST 16	// format comments with list/2nd line indent

// flags for open_line()
#define OPENLINE_DELSPACES  0x01    // delete spaces after cursor
#define OPENLINE_DO_COM	    0x02    // format comments
#define OPENLINE_KEEPTRAIL  0x04    // keep trailing spaces
#define OPENLINE_MARKFIX    0x08    // fix mark positions
#define OPENLINE_COM_LIST   0x10    // format comments with list/2nd line indent
#define OPENLINE_FORMAT	    0x20    // formatting long comment

// There are five history tables:
#define HIST_CMD	0	// colon commands
#define HIST_SEARCH	1	// search commands
#define HIST_EXPR	2	// expressions (from entering = register)
#define HIST_INPUT	3	// input() lines
#define HIST_DEBUG	4	// debug commands
#define HIST_COUNT	5	// number of history tables

// The type numbers are fixed for backwards compatibility.
#define BARTYPE_VERSION 1
#define BARTYPE_HISTORY 2
#define BARTYPE_REGISTER 3
#define BARTYPE_MARK 4

#define VIMINFO_VERSION 4
#define VIMINFO_VERSION_WITH_HISTORY 2
#define VIMINFO_VERSION_WITH_REGISTERS 3
#define VIMINFO_VERSION_WITH_MARKS 4

/*
 * Values for do_tag().
 */
#define DT_TAG		1	// jump to newer position or same tag again
#define DT_POP		2	// jump to older position
#define DT_NEXT		3	// jump to next match of same tag
#define DT_PREV		4	// jump to previous match of same tag
#define DT_FIRST	5	// jump to first match of same tag
#define DT_LAST		6	// jump to first match of same tag
#define DT_SELECT	7	// jump to selection from list
#define DT_HELP		8	// like DT_TAG, but no wildcards
#define DT_JUMP		9	// jump to new tag or selection from list
#define DT_CSCOPE	10	// cscope find command (like tjump)
#define DT_LTAG		11	// tag using location list
#define DT_FREE		99	// free cached matches

/*
 * flags for find_tags().
 */
#define TAG_HELP	1	// only search for help tags
#define TAG_NAMES	2	// only return name of tag
#define	TAG_REGEXP	4	// use tag pattern as regexp
#define	TAG_NOIC	8	// don't always ignore case
#ifdef FEAT_CSCOPE
# define TAG_CSCOPE	16	// cscope tag
#endif
#define TAG_VERBOSE	32	// message verbosity
#define TAG_INS_COMP	64	// Currently doing insert completion
#define TAG_KEEP_LANG	128	// keep current language
#define TAG_NO_TAGFUNC	256	// do not use 'tagfunc'

#define TAG_MANY	300	// When finding many tags (for completion),
				// find up to this many tags

/*
 * Types of dialogs passed to do_vim_dialog().
 */
#define VIM_GENERIC	0
#define VIM_ERROR	1
#define VIM_WARNING	2
#define VIM_INFO	3
#define VIM_QUESTION	4
#define VIM_LAST_TYPE	4	// sentinel value

/*
 * Return values for functions like gui_yesnocancel()
 */
#define VIM_YES		2
#define VIM_NO		3
#define VIM_CANCEL	4
#define VIM_ALL		5
#define VIM_DISCARDALL  6

/*
 * arguments for win_split()
 */
#define WSP_ROOM	0x01	// require enough room
#define WSP_VERT	0x02	// split/equalize vertically
#define WSP_HOR		0x04	// equalize horizontally
#define WSP_TOP		0x08	// window at top-left of shell
#define WSP_BOT		0x10	// window at bottom-right of shell
#define WSP_HELP	0x20	// creating the help window
#define WSP_BELOW	0x40	// put new window below/right
#define WSP_ABOVE	0x80	// put new window above/left
#define WSP_NEWLOC	0x100	// don't copy location list

/*
 * arguments for gui_set_shellsize()
 */
#define RESIZE_VERT	1	// resize vertically
#define RESIZE_HOR	2	// resize horizontally
#define RESIZE_BOTH	15	// resize in both directions

/*
 * flags for check_changed()
 */
#define CCGD_AW		1	// do autowrite if buffer was changed
#define CCGD_MULTWIN	2	// check also when several wins for the buf
#define CCGD_FORCEIT	4	// ! used
#define CCGD_ALLBUF	8	// may write all buffers
#define CCGD_EXCMD	16	// may suggest using !

/*
 * "flags" values for option-setting functions.
 * When OPT_GLOBAL and OPT_LOCAL are both missing, set both local and global
 * values, get local value.
 */
#define OPT_FREE	0x01	// free old value if it was allocated
#define OPT_GLOBAL	0x02	// use global value
#define OPT_LOCAL	0x04	// use local value
#define OPT_MODELINE	0x08	// option in modeline
#define OPT_WINONLY	0x10	// only set window-local options
#define OPT_NOWIN	0x20	// don't set window-local options
#define OPT_ONECOLUMN	0x40	// list options one per line
#define OPT_NO_REDRAW	0x80	// ignore redraw flags on option
#define OPT_SKIPRTP	0x100	// "skiprtp" in 'sessionoptions'

// Magic chars used in confirm dialog strings
#define DLG_BUTTON_SEP	'\n'
#define DLG_HOTKEY_CHAR	'&'

// Values for "starting"
#define NO_SCREEN	2	// no screen updating yet
#define NO_BUFFERS	1	// not all buffers loaded yet
//			0	   not starting anymore

// Values for swap_exists_action: what to do when swap file already exists
#define SEA_NONE	0	// don't use dialog
#define SEA_DIALOG	1	// use dialog when possible
#define SEA_QUIT	2	// quit editing the file
#define SEA_RECOVER	3	// recover the file
#define SEA_READONLY	4	// no dialog, mark buffer as read-only

/*
 * Minimal size for block 0 of a swap file.
 * NOTE: This depends on size of struct block0! It's not done with a sizeof(),
 * because struct block0 is defined in memline.c (Sorry).
 * The maximal block size is arbitrary.
 */
#define MIN_SWAP_PAGE_SIZE 1048
#define MAX_SWAP_PAGE_SIZE 50000

// Special values for current_sctx.sc_sid.
#define SID_MODELINE	(-1)	// when using a modeline
#define SID_CMDARG	(-2)	// for "--cmd" argument
#define SID_CARG	(-3)	// for "-c" argument
#define SID_ENV		(-4)	// for sourcing environment variable
#define SID_ERROR	(-5)	// option was reset because of an error
#define SID_NONE	(-6)	// don't set scriptID
#define SID_WINLAYOUT	(-7)	// changing window size

/*
 * Events for autocommands.
 */
enum auto_event
{
    EVENT_BUFADD = 0,		// after adding a buffer to the buffer list
    EVENT_BUFDELETE,		// deleting a buffer from the buffer list
    EVENT_BUFENTER,		// after entering a buffer
    EVENT_BUFFILEPOST,		// after renaming a buffer
    EVENT_BUFFILEPRE,		// before renaming a buffer
    EVENT_BUFHIDDEN,		// just after buffer becomes hidden
    EVENT_BUFLEAVE,		// before leaving a buffer
    EVENT_BUFNEW,		// after creating any buffer
    EVENT_BUFNEWFILE,		// when creating a buffer for a new file
    EVENT_BUFREADCMD,		// read buffer using command
    EVENT_BUFREADPOST,		// after reading a buffer
    EVENT_BUFREADPRE,		// before reading a buffer
    EVENT_BUFUNLOAD,		// just before unloading a buffer
    EVENT_BUFWINENTER,		// after showing a buffer in a window
    EVENT_BUFWINLEAVE,		// just after buffer removed from window
    EVENT_BUFWIPEOUT,		// just before really deleting a buffer
    EVENT_BUFWRITECMD,		// write buffer using command
    EVENT_BUFWRITEPOST,		// after writing a buffer
    EVENT_BUFWRITEPRE,		// before writing a buffer
    EVENT_CMDLINECHANGED,	// command line was modified
    EVENT_CMDLINEENTER,		// after entering the command line
    EVENT_CMDLINELEAVE,		// before leaving the command line
    EVENT_CMDUNDEFINED,		// command undefined
    EVENT_CMDWINENTER,		// after entering the cmdline window
    EVENT_CMDWINLEAVE,		// before leaving the cmdline window
    EVENT_COLORSCHEME,		// after loading a colorscheme
    EVENT_COLORSCHEMEPRE,	// before loading a colorscheme
    EVENT_COMPLETECHANGED,	// after completion popup menu changed
    EVENT_COMPLETEDONE,		// after finishing insert complete
    EVENT_COMPLETEDONEPRE,	// idem, before clearing info
    EVENT_CURSORHOLD,		// cursor in same position for a while
    EVENT_CURSORHOLDI,		// idem, in Insert mode
    EVENT_CURSORMOVED,		// cursor was moved
    EVENT_CURSORMOVEDI,		// cursor was moved in Insert mode
    EVENT_DIFFUPDATED,		// after diffs were updated
    EVENT_DIRCHANGED,		// after user changed directory
    EVENT_DIRCHANGEDPRE,	// before directory changes
    EVENT_ENCODINGCHANGED,	// after changing the 'encoding' option
    EVENT_EXITPRE,		// before exiting
    EVENT_FILEAPPENDCMD,	// append to a file using command
    EVENT_FILEAPPENDPOST,	// after appending to a file
    EVENT_FILEAPPENDPRE,	// before appending to a file
    EVENT_FILECHANGEDRO,	// before first change to read-only file
    EVENT_FILECHANGEDSHELL,	// after shell command that changed file
    EVENT_FILECHANGEDSHELLPOST,	// after (not) reloading changed file
    EVENT_FILEREADCMD,		// read from a file using command
    EVENT_FILEREADPOST,		// after reading a file
    EVENT_FILEREADPRE,		// before reading a file
    EVENT_FILETYPE,		// new file type detected (user defined)
    EVENT_FILEWRITECMD,		// write to a file using command
    EVENT_FILEWRITEPOST,	// after writing a file
    EVENT_FILEWRITEPRE,		// before writing a file
    EVENT_FILTERREADPOST,	// after reading from a filter
    EVENT_FILTERREADPRE,	// before reading from a filter
    EVENT_FILTERWRITEPOST,	// after writing to a filter
    EVENT_FILTERWRITEPRE,	// before writing to a filter
    EVENT_FOCUSGAINED,		// got the focus
    EVENT_FOCUSLOST,		// lost the focus to another app
    EVENT_FUNCUNDEFINED,	// if calling a function which doesn't exist
    EVENT_GUIENTER,		// after starting the GUI
    EVENT_GUIFAILED,		// after starting the GUI failed
    EVENT_INSERTCHANGE,		// when changing Insert/Replace mode
    EVENT_INSERTCHARPRE,	// before inserting a char
    EVENT_INSERTENTER,		// when entering Insert mode
    EVENT_INSERTLEAVEPRE,	// just before leaving Insert mode
    EVENT_INSERTLEAVE,		// just after leaving Insert mode
    EVENT_MENUPOPUP,		// just before popup menu is displayed
    EVENT_MODECHANGED,		// after changing the mode
    EVENT_OPTIONSET,		// option was set
    EVENT_QUICKFIXCMDPOST,	// after :make, :grep etc.
    EVENT_QUICKFIXCMDPRE,	// before :make, :grep etc.
    EVENT_QUITPRE,		// before :quit
    EVENT_REMOTEREPLY,		// upon string reception from a remote vim
    EVENT_SAFESTATE,		// going to wait for a character
    EVENT_SAFESTATEAGAIN,	// still waiting for a character
    EVENT_SESSIONLOADPOST,	// after loading a session file
    EVENT_SHELLCMDPOST,		// after ":!cmd"
    EVENT_SHELLFILTERPOST,	// after ":1,2!cmd", ":w !cmd", ":r !cmd".
    EVENT_SIGUSR1,		// after the SIGUSR1 signal
    EVENT_SOURCECMD,		// sourcing a Vim script using command
    EVENT_SOURCEPRE,		// before sourcing a Vim script
    EVENT_SOURCEPOST,		// after sourcing a Vim script
    EVENT_SPELLFILEMISSING,	// spell file missing
    EVENT_STDINREADPOST,	// after reading from stdin
    EVENT_STDINREADPRE,		// before reading from stdin
    EVENT_SWAPEXISTS,		// found existing swap file
    EVENT_SYNTAX,		// syntax selected
    EVENT_TABCLOSED,		// after closing a tab page
    EVENT_TABENTER,		// after entering a tab page
    EVENT_TABLEAVE,		// before leaving a tab page
    EVENT_TABNEW,		// when entering a new tab page
    EVENT_TERMCHANGED,		// after changing 'term'
    EVENT_TERMINALOPEN,		// after a terminal buffer was created
    EVENT_TERMINALWINOPEN,	// after a terminal buffer was created and
				// entering its window
    EVENT_TERMRESPONSE,		// after setting "v:termresponse"
    EVENT_TERMRESPONSEALL,	// after setting terminal response vars
    EVENT_TEXTCHANGED,		// text was modified not in Insert mode
    EVENT_TEXTCHANGEDI,		// text was modified in Insert mode
    EVENT_TEXTCHANGEDP,		// TextChangedI with popup menu visible
    EVENT_TEXTCHANGEDT,		// text was modified in Terminal mode
    EVENT_TEXTYANKPOST,		// after some text was yanked
    EVENT_USER,			// user defined autocommand
    EVENT_VIMENTER,		// after starting Vim
    EVENT_VIMLEAVE,		// before exiting Vim
    EVENT_VIMLEAVEPRE,		// before exiting Vim and writing .viminfo
    EVENT_VIMRESIZED,		// after Vim window was resized
    EVENT_WINENTER,		// after entering a window
    EVENT_WINLEAVE,		// before leaving a window
    EVENT_WINNEWPRE,		// before creating a new window
    EVENT_WINNEW,		// after creating a new window
    EVENT_WINCLOSED,		// after closing a window
    EVENT_VIMSUSPEND,		// before Vim is suspended
    EVENT_VIMRESUME,		// after Vim is resumed
    EVENT_WINRESIZED,		// after a window was resized
    EVENT_WINSCROLLED,		// after a window was scrolled or resized

    NUM_EVENTS			// MUST be the last one
};

typedef enum auto_event event_T;

/*
 * Values for index in highlight_attr[].
 * When making changes, also update HL_FLAGS below!
 * And update the default value of 'highlight': HIGHLIGHT_INIT in optiondefs.h
 */
typedef enum
{
    HLF_8 = 0	    // Meta & special keys listed with ":map", text that is
		    // displayed different from what it is
    , HLF_EOB	    // after the last line in the buffer
    , HLF_AT	    // @ characters at end of screen, characters that
		    // don't really exist in the text
    , HLF_D	    // directories in CTRL-D listing
    , HLF_E	    // error messages
    , HLF_H	    // obsolete, ignored
    , HLF_I	    // incremental search
    , HLF_L	    // last search string
    , HLF_LC	    // last search string under cursor
    , HLF_M	    // "--More--" message
    , HLF_CM	    // Mode (e.g., "-- INSERT --")
    , HLF_N	    // line number for ":number" and ":#" commands
    , HLF_LNA	    // LineNrAbove
    , HLF_LNB	    // LineNrBelow
    , HLF_CLN	    // current line number
    , HLF_CLS	    // current line sign column
    , HLF_CLF	    // current line fold
    , HLF_R	    // return to continue message and yes/no questions
    , HLF_S	    // status lines
    , HLF_SNC	    // status lines of not-current windows
    , HLF_C	    // column to separate vertically split windows
    , HLF_T	    // Titles for output from ":set all", ":autocmd" etc.
    , HLF_V	    // Visual mode
    , HLF_VNC	    // Visual mode, autoselecting and not clipboard owner
    , HLF_W	    // warning messages
    , HLF_WM	    // Wildmenu highlight
    , HLF_FL	    // Folded line
    , HLF_FC	    // Fold column
    , HLF_ADD	    // Added diff line
    , HLF_CHD	    // Changed diff line
    , HLF_DED	    // Deleted diff line
    , HLF_TXD	    // Text Changed in diff line
    , HLF_CONCEAL   // Concealed text
    , HLF_SC	    // Sign column
    , HLF_SPB	    // SpellBad
    , HLF_SPC	    // SpellCap
    , HLF_SPR	    // SpellRare
    , HLF_SPL	    // SpellLocal
    , HLF_PNI	    // popup menu normal item
    , HLF_PSI	    // popup menu selected item
    , HLF_PNK	    // popup menu normal item "kind"
    , HLF_PSK	    // popup menu selected item "kind"
    , HLF_PNX	    // popup menu normal item "menu" (extra text)
    , HLF_PSX	    // popup menu selected item "menu" (extra text)
    , HLF_PSB	    // popup menu scrollbar
    , HLF_PST	    // popup menu scrollbar thumb
    , HLF_TP	    // tabpage line
    , HLF_TPS	    // tabpage line selected
    , HLF_TPF	    // tabpage line filler
    , HLF_CUC	    // 'cursorcolumn'
    , HLF_CUL	    // 'cursorline'
    , HLF_MC	    // 'colorcolumn'
    , HLF_QFL	    // quickfix window line currently selected
    , HLF_ST	    // status lines of terminal windows
    , HLF_STNC	    // status lines of not-current terminal windows
    , HLF_COUNT	    // MUST be the last one
} hlf_T;

// The HL_FLAGS must be in the same order as the HLF_ enums!
// When changing this also adjust the default for 'highlight'.
#define HL_FLAGS {'8', '~', '@', 'd', 'e', 'h', 'i', 'l', 'y', 'm', 'M', \
		  'n', 'a', 'b', 'N', 'G', 'O', 'r', 's', 'S', 'c', 't', 'v', 'V', \
		  'w', 'W', 'f', 'F', 'A', 'C', 'D', 'T', '-', '>', \
		  'B', 'P', 'R', 'L', \
		  '+', '=', '[', ']', '{', '}', 'x', 'X', \
		  '*', '#', '_', '!', '.', 'o', 'q', \
		  'z', 'Z'}

/*
 * Boolean constants
 */
#ifndef TRUE
# define FALSE	0	    // note: this is an int, not a long!
# define TRUE	1
#endif

#define MAYBE	2	    // sometimes used for a variant on TRUE

#define LOG_ALWAYS 9	    // must be different from TRUE and FALSE

#ifdef FEAT_JOB_CHANNEL
// If "--log logfile" was used or ch_logfile() was called then log some or all
// terminal output.
# define MAY_WANT_TO_LOG_THIS if (ch_log_output == FALSE) ch_log_output = TRUE;
#else
// no logging support
# define MAY_WANT_TO_LOG_THIS
#endif

#ifndef UINT32_T
typedef UINT32_TYPEDEF UINT32_T;
#endif

/*
 * Operator IDs; The order must correspond to opchars[] in ops.c!
 */
#define OP_NOP		0	// no pending operation
#define OP_DELETE	1	// "d"  delete operator
#define OP_YANK		2	// "y"  yank operator
#define OP_CHANGE	3	// "c"  change operator
#define OP_LSHIFT	4	// "<"  left shift operator
#define OP_RSHIFT	5	// ">"  right shift operator
#define OP_FILTER	6	// "!"  filter operator
#define OP_TILDE	7	// "g~" switch case operator
#define OP_INDENT	8	// "="  indent operator
#define OP_FORMAT	9	// "gq" format operator
#define OP_COLON	10	// ":"  colon operator
#define OP_UPPER	11	// "gU" make upper case operator
#define OP_LOWER	12	// "gu" make lower case operator
#define OP_JOIN		13	// "J"  join operator, only for Visual mode
#define OP_JOIN_NS	14	// "gJ"  join operator, only for Visual mode
#define OP_ROT13	15	// "g?" rot-13 encoding
#define OP_REPLACE	16	// "r"  replace chars, only for Visual mode
#define OP_INSERT	17	// "I"  Insert column, only for Visual mode
#define OP_APPEND	18	// "A"  Append column, only for Visual mode
#define OP_FOLD		19	// "zf" define a fold
#define OP_FOLDOPEN	20	// "zo" open folds
#define OP_FOLDOPENREC	21	// "zO" open folds recursively
#define OP_FOLDCLOSE	22	// "zc" close folds
#define OP_FOLDCLOSEREC	23	// "zC" close folds recursively
#define OP_FOLDDEL	24	// "zd" delete folds
#define OP_FOLDDELREC	25	// "zD" delete folds recursively
#define OP_FORMAT2	26	// "gw" format operator, keeps cursor pos
#define OP_FUNCTION	27	// "g@" call 'operatorfunc'
#define OP_NR_ADD	28	// "<C-A>" Add to the number or alphabetic
				// character (OP_ADD conflicts with Perl)
#define OP_NR_SUB	29	// "<C-X>" Subtract from the number or
				// alphabetic character

/*
 * Motion types, used for operators and for yank/delete registers.
 */
#define MCHAR	0		// character-wise movement/register
#define MLINE	1		// line-wise movement/register
#define MBLOCK	2		// block-wise register

#define MAUTO	0xff		// Decide between MLINE/MCHAR

/*
 * Minimum screen size
 */
#define MIN_COLUMNS	12	// minimal columns for screen
#define MIN_LINES	2	// minimal lines for screen
#define STATUS_HEIGHT	1	// height of a status line under a window
#ifdef FEAT_MENU		// height of a status line under a window
# define WINBAR_HEIGHT(wp)	(wp)->w_winbar_height
# define VISIBLE_HEIGHT(wp)	((wp)->w_height + (wp)->w_winbar_height)
#else
# define WINBAR_HEIGHT(wp)	0
# define VISIBLE_HEIGHT(wp)	(wp)->w_height
#endif
#define QF_WINHEIGHT	10	// default height for quickfix window

/*
 * Buffer sizes
 */
#ifndef CMDBUFFSIZE
# define CMDBUFFSIZE	256	// size of the command processing buffer
#endif

#define LSIZE	    512		// max. size of a line in the tags file

#define IOSIZE	   (1024+1)	// file i/o and sprintf buffer size

#define DIALOG_MSG_SIZE 1000	// buffer size for dialog_msg()

#define MSG_BUF_LEN 480	// length of buffer for small messages
#define MSG_BUF_CLEN  (MSG_BUF_LEN / 6)    // cell length (worst case: utf-8
					   // takes 6 bytes for one cell)

#define FOLD_TEXT_LEN  51	// buffer size for get_foldtext()

// Size of the buffer used for tgetent().  Unfortunately this is largely
// undocumented, some systems use 1024.  Using a buffer that is too small
// causes a buffer overrun and a crash.  Use the maximum known value to stay
// on the safe side.
#define TBUFSZ 2048		// buffer size for termcap entry

/*
 * Maximum length of key sequence to be mapped.
 * Must be able to hold an Amiga resize report.
 */
#define MAXMAPLEN   50

// maximum length of a function name, including SID and NUL
#define MAX_FUNC_NAME_LEN   200

// Size in bytes of the hash used in the undo file.
#define UNDO_HASH_SIZE 32

#ifdef HAVE_FCNTL_H
# include <fcntl.h>
#endif

#ifdef BINARY_FILE_IO
# define WRITEBIN   "wb"	// no CR-LF translation
# define READBIN    "rb"
# define APPENDBIN  "ab"
#else
# define WRITEBIN   "w"
# define READBIN    "r"
# define APPENDBIN  "a"
#endif

/*
 * Cygwin doesn't have a global way of making open() use binary I/O.
 * Use O_BINARY for all open() calls.
 */
#ifdef __CYGWIN__
# define O_EXTRA    O_BINARY
#else
# define O_EXTRA    0
#endif

#ifndef O_NOFOLLOW
# define O_NOFOLLOW 0
#endif

#ifndef W_OK
# define W_OK 2		// for systems that don't have W_OK in unistd.h
#endif
#ifndef R_OK
# define R_OK 4		// for systems that don't have R_OK in unistd.h
#endif

// Allocate memory for one type and cast the returned pointer to have the
// compiler check the types.
#define ALLOC_ONE(type)  (type *)alloc(sizeof(type))
#define ALLOC_ONE_ID(type, id)  (type *)alloc_id(sizeof(type), id)
#define ALLOC_MULT(type, count)  (type *)alloc(sizeof(type) * (count))
#define ALLOC_CLEAR_ONE(type)  (type *)alloc_clear(sizeof(type))
#define ALLOC_CLEAR_ONE_ID(type, id)  (type *)alloc_clear_id(sizeof(type), id)
#define ALLOC_CLEAR_MULT(type, count)  (type *)alloc_clear(sizeof(type) * (count))
#define LALLOC_CLEAR_ONE(type)  (type *)lalloc_clear(sizeof(type), FALSE)
#define LALLOC_CLEAR_MULT(type, count)  (type *)lalloc_clear(sizeof(type) * (count), FALSE)
#define LALLOC_MULT(type, count)  (type *)lalloc(sizeof(type) * (count), FALSE)

#ifdef HAVE_MEMSET
# define vim_memset(ptr, c, size)   memset((ptr), (c), (size))
#else
void *vim_memset(void *, int, size_t);
#endif
#define CLEAR_FIELD(field)  vim_memset(&(field), 0, sizeof(field))
#define CLEAR_POINTER(ptr)  vim_memset((ptr), 0, sizeof(*(ptr)))

/*
 * defines to avoid typecasts from (char_u *) to (char *) and back
 * (vim_strchr() and vim_strrchr() are now in strings.c)
 */
#define STRLEN(s)	    strlen((char *)(s))
#define STRCPY(d, s)	    strcpy((char *)(d), (char *)(s))
#define STRNCPY(d, s, n)    strncpy((char *)(d), (char *)(s), (size_t)(n))
#define STRCMP(d, s)	    strcmp((char *)(d), (char *)(s))
#define STRNCMP(d, s, n)    strncmp((char *)(d), (char *)(s), (size_t)(n))
#ifdef HAVE_STRCASECMP
# define STRICMP(d, s)	    strcasecmp((char *)(d), (char *)(s))
#else
# ifdef HAVE_STRICMP
#  define STRICMP(d, s)	    stricmp((char *)(d), (char *)(s))
# else
#  define STRICMP(d, s)	    vim_stricmp((char *)(d), (char *)(s))
# endif
#endif
#ifdef HAVE_STRCOLL
# define STRCOLL(d, s)     strcoll((char *)(d), (char *)(s))
#else
# define STRCOLL(d, s)     strcmp((char *)(d), (char *)(s))
#endif

// Like strcpy() but allows overlapped source and destination.
#define STRMOVE(d, s)	    mch_memmove((d), (s), STRLEN(s) + 1)

#ifdef HAVE_STRNCASECMP
# define STRNICMP(d, s, n)  strncasecmp((char *)(d), (char *)(s), (size_t)(n))
#else
# ifdef HAVE_STRNICMP
#  define STRNICMP(d, s, n) strnicmp((char *)(d), (char *)(s), (size_t)(n))
# else
#  define STRNICMP(d, s, n) vim_strnicmp((char *)(d), (char *)(s), (size_t)(n))
# endif
#endif

// We need to call mb_stricmp() even when we aren't dealing with a multi-byte
// encoding because mb_stricmp() takes care of all ascii and non-ascii
// encodings, including characters with umlauts in latin1, etc., while
// STRICMP() only handles the system locale version, which often does not
// handle non-ascii properly.

# define MB_STRICMP(d, s)	mb_strnicmp((char_u *)(d), (char_u *)(s), (int)MAXCOL)
# define MB_STRNICMP(d, s, n)	mb_strnicmp((char_u *)(d), (char_u *)(s), (int)(n))

#define STRCAT(d, s)	    strcat((char *)(d), (char *)(s))
#define STRNCAT(d, s, n)    strncat((char *)(d), (char *)(s), (size_t)(n))

#ifdef HAVE_STRPBRK
# define vim_strpbrk(s, cs) (char_u *)strpbrk((char *)(s), (char *)(cs))
#endif

#define OUT_STR(s)		    out_str((char_u *)(s))
#define OUT_STR_NF(s)		    out_str_nf((char_u *)(s))

#ifdef FEAT_GUI
# ifdef FEAT_TERMGUICOLORS
#  define GUI_FUNCTION(f)	    (gui.in_use ? gui_##f : termgui_##f)
#  define GUI_FUNCTION2(f, pixel)   (gui.in_use \
				    ?  ((pixel) != INVALCOLOR \
					? gui_##f((pixel)) \
					: INVALCOLOR) \
				    : termgui_##f((pixel)))
#  define USE_24BIT		    (gui.in_use || p_tgc)
# else
#  define GUI_FUNCTION(f)	    gui_##f
#  define GUI_FUNCTION2(f,pixel)    ((pixel) != INVALCOLOR \
				     ? gui_##f((pixel)) \
				     : INVALCOLOR)
#  define USE_24BIT		    gui.in_use
# endif
#else
# ifdef FEAT_TERMGUICOLORS
#  define GUI_FUNCTION(f)	    termgui_##f
#  define GUI_FUNCTION2(f, pixel)   termgui_##f((pixel))
#  define USE_24BIT		    p_tgc
# endif
#endif
#ifdef FEAT_TERMGUICOLORS
# define IS_CTERM		    (t_colors > 1 || p_tgc)
#else
# define IS_CTERM		    (t_colors > 1)
#endif
#ifdef GUI_FUNCTION
# define GUI_MCH_GET_RGB	    GUI_FUNCTION(mch_get_rgb)
# define GUI_MCH_GET_RGB2(pixel)    GUI_FUNCTION2(mch_get_rgb, (pixel))
# define GUI_MCH_GET_COLOR	    GUI_FUNCTION(mch_get_color)
# define GUI_GET_COLOR		    GUI_FUNCTION(get_color)
#endif

// Prefer using emsgf(), because perror() may send the output to the wrong
// destination and mess up the screen.
#ifdef HAVE_STRERROR
# define PERROR(msg)		    (void)semsg("%s: %s", (char *)(msg), strerror(errno))
#else
# define PERROR(msg)		    do_perror(msg)
#endif

typedef long	linenr_T;		// line number type
typedef int	colnr_T;		// column number type
typedef unsigned short disptick_T;	// display tick type

/*
 * Well, you won't believe it, but some S/390 machines ("host", now also known
 * as zServer) use 31 bit pointers. There are also some newer machines, that
 * use 64 bit pointers. I don't know how to distinguish between 31 and 64 bit
 * machines, so the best way is to assume 31 bits whenever we detect OS/390
 * Unix.
 * With this we restrict the maximum line length to 1073741823. I guess this is
 * not a real problem. BTW:  Longer lines are split.
 */
#ifdef __MVS__
# define MAXCOL (0x3fffffffL)		// maximum column number, 30 bits
# define MAXLNUM (0x3fffffffL)		// maximum (invalid) line number
#else
  // MAXCOL used to be INT_MAX, but with 64 bit ints that results in running
  // out of memory when trying to allocate a very long line.
# define MAXCOL  0x7fffffffL		// maximum column number
# define MAXLNUM LONG_MAX		// maximum (invalid) line number
#endif

#define SHOWCMD_COLS 10			// columns needed by shown command

typedef void	    *vim_acl_T;		// dummy to pass an ACL to a function

#ifndef mch_memmove
# define mch_memmove(to, from, len) memmove((char*)(to), (char*)(from), (size_t)(len))
#endif

/*
 * fnamecmp() is used to compare file names.
 * On some systems case in a file name does not matter, on others it does.
 * (this does not account for maximum name lengths and things like "../dir",
 * thus it is not 100% accurate!)
 */
#define fnamecmp(x, y) vim_fnamecmp((char_u *)(x), (char_u *)(y))
#define fnamencmp(x, y, n) vim_fnamencmp((char_u *)(x), (char_u *)(y), (size_t)(n))

#if defined(UNIX) || defined(FEAT_GUI) || defined(VMS) \
	|| defined(FEAT_CLIENTSERVER)
# define USE_INPUT_BUF
#endif

#ifndef EINTR
# define read_eintr(fd, buf, count) vim_read((fd), (buf), (count))
# define write_eintr(fd, buf, count) vim_write((fd), (buf), (count))
#endif

#ifdef MSWIN
// On MS-Windows the third argument isn't size_t.  This matters for Win64,
// where sizeof(size_t)==8, not 4
# define vim_read(fd, buf, count)   read((fd), (char *)(buf), (unsigned int)(count))
# define vim_write(fd, buf, count)  write((fd), (char *)(buf), (unsigned int)(count))
#else
# define vim_read(fd, buf, count)   read((fd), (char *)(buf), (size_t) (count))
# define vim_write(fd, buf, count)  write((fd), (char *)(buf), (size_t) (count))
#endif

/*
 * Enums need a typecast to be used as array index (for Ultrix).
 */
#define HL_ATTR(n)	highlight_attr[(int)(n)]
#define TERM_STR(n)	term_strings[(int)(n)]

/*
 * EXTERN is only defined in main.c.  That's where global variables are
 * actually defined and initialized.
 */
#ifndef EXTERN
# define EXTERN extern
# define INIT(x)
# define INIT2(a, b)
# define INIT3(a, b, c)
# define INIT4(a, b, c, d)
# define INIT5(a, b, c, d, e)
# define INIT6(a, b, c, d, e, f)
#else
# ifndef INIT
#  define INIT(x) x
#  define INIT2(a, b) = {a, b}
#  define INIT3(a, b, c) = {a, b, c}
#  define INIT4(a, b, c, d) = {a, b, c, d}
#  define INIT5(a, b, c, d, e) = {a, b, c, d, e}
#  define INIT6(a, b, c, d, e, f) = {a, b, c, d, e, f}
#  define DO_INIT
# endif
#endif

#define MAX_MCO	6	// maximum value for 'maxcombine'

// Maximum number of bytes in a multi-byte character.  It can be one 32-bit
// character of up to 6 bytes, or one 16-bit character of up to three bytes
// plus six following composing characters of three bytes each.
#define MB_MAXBYTES	21

#if (defined(FEAT_PROFILE) || defined(FEAT_RELTIME)) && !defined(PROTO)
# ifdef MSWIN
typedef LARGE_INTEGER proftime_T;
#  define PROF_TIME_BLANK "           "
#  define PROF_TOTALS_HEADER "count  total (s)   self (s)"
# else
   // Use tv_fsec for fraction of second (micro or nano) of proftime_T
#  if defined(HAVE_TIMER_CREATE) && defined(HAVE_CLOCK_GETTIME)
#   define PROF_NSEC 1
typedef struct timespec proftime_T;
#   define PROF_GET_TIME(tm) clock_gettime(CLOCK_MONOTONIC, tm)
#   define tv_fsec tv_nsec
#   define TV_FSEC_SEC 1000000000L
#   define PROF_TIME_FORMAT "%3ld.%09ld"
#   define PROF_TIME_BLANK "              "
#   define PROF_TOTALS_HEADER "count     total (s)      self (s)"
#  else
typedef struct timeval proftime_T;
#   define PROF_GET_TIME(tm) gettimeofday(tm, NULL)
#   define tv_fsec tv_usec
#   define TV_FSEC_SEC 1000000
#   define PROF_TIME_FORMAT "%3ld.%06ld"
#   define PROF_TIME_BLANK "           "
#   define PROF_TOTALS_HEADER "count  total (s)   self (s)"
#  endif
# endif
#else
typedef int proftime_T;	    // dummy for function prototypes
#endif

// Type of compilation passed to compile_def_function()
typedef enum {
    CT_NONE,	    // use df_instr
    CT_PROFILE,	    // use df_instr_prof
    CT_DEBUG	    // use df_instr_debug, overrules CT_PROFILE
} compiletype_T;

/*
 * When compiling with 32 bit Perl time_t is 32 bits in the Perl code but 64
 * bits elsewhere.  That causes memory corruption.  Define time_T and use it
 * for global variables to avoid that.
 */
#ifdef PROTO
typedef long  time_T;
#else
# ifdef MSWIN
typedef __time64_t  time_T;
# else
typedef time_t	    time_T;
# endif
#endif

#ifdef _WIN64
typedef __int64 sock_T;
#else
typedef int sock_T;
#endif

// Include option.h before structs.h, because the number of window-local and
// buffer-local options is used there.
#include "option.h"	// options and default values

#include "beval.h"	// BalloonEval

// Note that gui.h is included by structs.h

#include "structs.h"	// defines many structures

#include "alloc.h"

// Values for "do_profiling".
#define PROF_NONE	0	// profiling not started
#define PROF_YES	1	// profiling busy
#define PROF_PAUSED	2	// profiling paused


// Codes for mouse button events in lower three bits:
#define MOUSE_LEFT	0x00
#define MOUSE_MIDDLE	0x01
#define MOUSE_RIGHT	0x02
#define MOUSE_RELEASE	0x03

// bit masks for modifiers:
#define MOUSE_SHIFT	0x04
#define MOUSE_ALT	0x08
#define MOUSE_CTRL	0x10

// mouse buttons that are handled like a key press (GUI only)
// Note that the scroll wheel keys are inverted: MOUSE_5 scrolls lines up but
// the result of this is that the window moves down, similarly MOUSE_6 scrolls
// columns left but the window moves right.
#define MOUSE_4	0x100	// scroll wheel down
#define MOUSE_5	0x200	// scroll wheel up

#define MOUSE_X1	0x300 // Mouse-button X1 (6th)
#define MOUSE_X2	0x400 // Mouse-button X2

#define MOUSE_6	0x500	// scroll wheel left
#define MOUSE_7	0x600	// scroll wheel right

#define MOUSE_MOVE 0x700    // report mouse moved

// 0x20 is reserved by xterm
#define MOUSE_DRAG_XTERM   0x40

#define MOUSE_DRAG	(0x40 | MOUSE_RELEASE)

// Lowest button code for using the mouse wheel (xterm only)
#define MOUSEWHEEL_LOW		0x60

#define MOUSE_CLICK_MASK	0x03

#define NUM_MOUSE_CLICKS(code) \
    (((unsigned)((code) & 0xC0) >> 6) + 1)

#define SET_NUM_MOUSE_CLICKS(code, num) \
    ((code) = ((code) & 0x3f) | ((((num) - 1) & 3) << 6))

// Added to mouse column for GUI when 'mousefocus' wants to give focus to a
// window by simulating a click on its status line.  We could use up to 128 *
// 128 = 16384 columns, now it's reduced to 10000.
#define MOUSE_COLOFF 10000

/*
 * jump_to_mouse() returns one of first five these values, possibly with
 * some of the other four added.
 */
#define IN_UNKNOWN		0
#define IN_BUFFER		1
#define IN_STATUS_LINE		2	// on status or command line
#define IN_SEP_LINE		4	// on vertical separator line
#define IN_OTHER_WIN		8	// in other window but can't go there
#define CURSOR_MOVED		0x100
#define MOUSE_FOLD_CLOSE	0x200	// clicked on '-' in fold column
#define MOUSE_FOLD_OPEN		0x400	// clicked on '+' in fold column
#define MOUSE_WINBAR		0x800	// in window toolbar

// flags for jump_to_mouse()
#define MOUSE_FOCUS		0x01	// need to stay in this window
#define MOUSE_MAY_VIS		0x02	// may start Visual mode
#define MOUSE_DID_MOVE		0x04	// only act when mouse has moved
#define MOUSE_SETPOS		0x08	// only set current mouse position
#define MOUSE_MAY_STOP_VIS	0x10	// may stop Visual mode
#define MOUSE_RELEASED		0x20	// button was released

#if defined(UNIX) && defined(HAVE_GETTIMEOFDAY) && defined(HAVE_SYS_TIME_H)
# define CHECK_DOUBLE_CLICK 1	// Checking for double clicks ourselves.
#endif


// defines for eval_vars()
#define VALID_PATH		1
#define VALID_HEAD		2

// Defines for Vim variables.  These must match vimvars[] in evalvars.c!
#define VV_COUNT	0
#define VV_COUNT1	1
#define VV_PREVCOUNT	2
#define VV_ERRMSG	3
#define VV_WARNINGMSG	4
#define VV_STATUSMSG	5
#define VV_SHELL_ERROR	6
#define VV_THIS_SESSION	7
#define VV_VERSION	8
#define VV_LNUM		9
#define VV_TERMRESPONSE	10
#define VV_FNAME	11
#define VV_LANG		12
#define VV_LC_TIME	13
#define VV_CTYPE	14
#define VV_CC_FROM	15
#define VV_CC_TO	16
#define VV_FNAME_IN	17
#define VV_FNAME_OUT	18
#define VV_FNAME_NEW	19
#define VV_FNAME_DIFF	20
#define VV_CMDARG	21
#define VV_FOLDSTART	22
#define VV_FOLDEND	23
#define VV_FOLDDASHES	24
#define VV_FOLDLEVEL	25
#define VV_PROGNAME	26
#define VV_SEND_SERVER	27
#define VV_DYING	28
#define VV_EXCEPTION	29
#define VV_THROWPOINT	30
#define VV_REG		31
#define VV_CMDBANG	32
#define VV_INSERTMODE	33
#define VV_VAL		34
#define VV_KEY		35
#define VV_PROFILING	36
#define VV_FCS_REASON	37
#define VV_FCS_CHOICE	38
#define VV_BEVAL_BUFNR	39
#define VV_BEVAL_WINNR	40
#define VV_BEVAL_WINID	41
#define VV_BEVAL_LNUM	42
#define VV_BEVAL_COL	43
#define VV_BEVAL_TEXT	44
#define VV_SCROLLSTART	45
#define VV_SWAPNAME	46
#define VV_SWAPCHOICE	47
#define VV_SWAPCOMMAND	48
#define VV_CHAR		49
#define VV_MOUSE_WIN	50
#define VV_MOUSE_WINID	51
#define VV_MOUSE_LNUM   52
#define VV_MOUSE_COL	53
#define VV_OP		54
#define VV_SEARCHFORWARD 55
#define VV_HLSEARCH	56
#define VV_OLDFILES	57
#define VV_WINDOWID	58
#define VV_PROGPATH	59
#define VV_COMPLETED_ITEM 60
#define VV_OPTION_NEW   61
#define VV_OPTION_OLD   62
#define VV_OPTION_OLDLOCAL 63
#define VV_OPTION_OLDGLOBAL 64
#define VV_OPTION_COMMAND 65
#define VV_OPTION_TYPE  66
#define VV_ERRORS	67
#define VV_FALSE	68
#define VV_TRUE		69
#define VV_NONE		70
#define VV_NULL		71
#define VV_NUMBERMAX	72
#define VV_NUMBERMIN	73
#define VV_NUMBERSIZE	74
#define VV_VIM_DID_ENTER 75
#define VV_TESTING	76
#define VV_TYPE_NUMBER	77
#define VV_TYPE_STRING	78
#define VV_TYPE_FUNC	79
#define VV_TYPE_LIST	80
#define VV_TYPE_DICT	81
#define VV_TYPE_FLOAT	82
#define VV_TYPE_BOOL	83
#define VV_TYPE_NONE	84
#define VV_TYPE_JOB	85
#define VV_TYPE_CHANNEL	86
#define VV_TYPE_BLOB	87
#define VV_TYPE_CLASS	88
#define VV_TYPE_OBJECT	89
#define VV_TERMRFGRESP	90
#define VV_TERMRBGRESP	91
#define VV_TERMU7RESP	92
#define VV_TERMSTYLERESP 93
#define VV_TERMBLINKRESP 94
#define VV_EVENT	95
#define VV_VERSIONLONG	96
#define VV_ECHOSPACE	97
#define VV_ARGV		98
#define VV_COLLATE      99
#define VV_EXITING	100
#define VV_COLORNAMES   101
#define VV_SIZEOFINT	102
#define VV_SIZEOFLONG	103
#define VV_SIZEOFPOINTER 104
#define VV_MAXCOL	105
#define VV_PYTHON3_VERSION 106
#define VV_TYPE_TYPEALIAS 107
#define VV_LEN		108	// number of v: vars

// used for v_number in VAR_BOOL and VAR_SPECIAL
#define VVAL_FALSE	0L	// VAR_BOOL
#define VVAL_TRUE	1L	// VAR_BOOL
#define VVAL_NONE	2L	// VAR_SPECIAL
#define VVAL_NULL	3L	// VAR_SPECIAL

// Type values for type().
#define VAR_TYPE_NUMBER	    0
#define VAR_TYPE_STRING	    1
#define VAR_TYPE_FUNC	    2
#define VAR_TYPE_LIST	    3
#define VAR_TYPE_DICT	    4
#define VAR_TYPE_FLOAT	    5
#define VAR_TYPE_BOOL	    6
#define VAR_TYPE_NONE	    7
#define VAR_TYPE_JOB	    8
#define VAR_TYPE_CHANNEL    9
#define VAR_TYPE_BLOB	    10
#define VAR_TYPE_INSTR	    11
#define VAR_TYPE_CLASS	    12
#define VAR_TYPE_OBJECT	    13
#define VAR_TYPE_TYPEALIAS  14

#define DICT_MAXNEST 100	// maximum nesting of lists and dicts

#define TABSTOP_MAX 9999

#ifdef FEAT_CLIPBOARD

// VIM_ATOM_NAME is the older Vim-specific selection type for X11.  Still
// supported for when a mix of Vim versions is used. VIMENC_ATOM_NAME includes
// the encoding to support Vims using different 'encoding' values.
# define VIM_ATOM_NAME "_VIM_TEXT"
# define VIMENC_ATOM_NAME "_VIMENC_TEXT"

// Selection states for modeless selection
# define SELECT_CLEARED		0
# define SELECT_IN_PROGRESS	1
# define SELECT_DONE		2

# define SELECT_MODE_CHAR	0
# define SELECT_MODE_WORD	1
# define SELECT_MODE_LINE	2

# ifdef FEAT_GUI_MSWIN
#  ifdef FEAT_OLE
#   define WM_OLE (WM_APP+0)
#  endif
# endif

// Info about selected text
typedef struct
{
    int		available;	// Is clipboard available?
    int		owned;		// Flag: do we own the selection?
    pos_T	start;		// Start of selected area
    pos_T	end;		// End of selected area
    int		vmode;		// Visual mode character

    // Fields for selection that doesn't use Visual mode
    short_u	origin_row;
    short_u	origin_start_col;
    short_u	origin_end_col;
    short_u	word_start_col;
    short_u	word_end_col;
#ifdef FEAT_PROP_POPUP
    // limits for selection inside a popup window
    short_u	min_col;
    short_u	max_col;
    short_u	min_row;
    short_u	max_row;
#endif

    pos_T	prev;		// Previous position
    short_u	state;		// Current selection state
    short_u	mode;		// Select by char, word, or line.

# if defined(FEAT_GUI_X11) || defined(FEAT_XCLIPBOARD)
    Atom	sel_atom;	// PRIMARY/CLIPBOARD selection ID
# endif

# ifdef FEAT_GUI_GTK
    GdkAtom     gtk_sel_atom;	// PRIMARY/CLIPBOARD selection ID
# endif

# if defined(MSWIN) || defined(FEAT_CYGWIN_WIN32_CLIPBOARD)
    int_u	format;		// Vim's own special clipboard format
    int_u	format_raw;	// Vim's raw text clipboard format
# endif
# ifdef FEAT_GUI_HAIKU
    // No clipboard at the moment. TODO?
# endif
} Clipboard_T;
#else
typedef int Clipboard_T;	// This is required for the prototypes.
#endif

// Use 64-bit stat structure on MS-Windows.
#ifdef MSWIN
typedef struct _stat64 stat_T;
#else
typedef struct stat stat_T;
#endif

#if (defined(__GNUC__) || defined(__clang__)) && !defined(__MINGW32__)
# define ATTRIBUTE_FORMAT_PRINTF(fmt_idx, arg_idx) \
    __attribute__((format(printf, fmt_idx, arg_idx)))
#else
# define ATTRIBUTE_FORMAT_PRINTF(fmt_idx, arg_idx)
#endif

#if defined(__GNUC__) || defined(__clang__)
# define likely(x)	__builtin_expect((x), 1)
# define unlikely(x)	__builtin_expect((x), 0)
# define ATTRIBUTE_COLD	__attribute__((cold))
#else
# define unlikely(x)	(x)
# define likely(x)	(x)
# define ATTRIBUTE_COLD
#endif

typedef enum {
    ASSERT_EQUAL,
    ASSERT_NOTEQUAL,
    ASSERT_MATCH,
    ASSERT_NOTMATCH,
    ASSERT_FAILS,
    ASSERT_OTHER
} assert_type_T;

// Mode for bracketed_paste().
typedef enum {
    PASTE_INSERT,	// insert mode
    PASTE_CMDLINE,	// command line
    PASTE_EX,		// ex mode line
    PASTE_ONE_CHAR	// return first character
} paste_mode_T;

// Argument for flush_buffers().
typedef enum {
    FLUSH_MINIMAL,
    FLUSH_TYPEAHEAD,	// flush current typebuf contents
    FLUSH_INPUT		// flush typebuf and inchar() input
} flush_buffers_T;

// Argument for prepare_tagpreview()
typedef enum {
    USEPOPUP_NONE,
    USEPOPUP_NORMAL,	// use info popup
    USEPOPUP_HIDDEN	// use info popup initially hidden
} use_popup_T;

// Argument for estack_sfile().
typedef enum {
    ESTACK_NONE,
    ESTACK_SFILE,
    ESTACK_STACK,
    ESTACK_SCRIPT,
} estack_arg_T;

// Return value of match_keyprotocol()
typedef enum {
    KEYPROTOCOL_NONE,
    KEYPROTOCOL_MOK2,
    KEYPROTOCOL_KITTY,
    KEYPROTOCOL_FAIL
} keyprot_T;

// errors for when calling a function
typedef enum {
    FCERR_NONE,		// no error
    FCERR_UNKNOWN,	// unknown function
    FCERR_TOOMANY,	// too many arguments
    FCERR_TOOFEW,	// too few arguments
    FCERR_SCRIPT,	// missing script context
    FCERR_DICT,		// missing dict
    FCERR_OTHER,	// another kind of error
    FCERR_DELETED,	// function was deleted
    FCERR_NOTMETHOD,	// function cannot be used as a method
    FCERR_FAILED,	// error while executing the function
} funcerror_T;

/*
 * Type for the callback function that is invoked after an option value is
 * changed to validate and apply the new value.
 *
 * Returns NULL if the option value is valid is successfully applied.
 * Otherwise returns an error message.
 */
typedef char *(*opt_did_set_cb_T)(optset_T *args);

/*
 * Type for the callback function that is invoked when expanding possible
 * string option values during cmdline completion.
 *
 * Strings in returned matches will be managed and freed by caller.
 *
 * Returns OK if the expansion succeeded (numMatches and matches have to be
 * set). Otherwise returns FAIL.
 *
 * Note: If returned FAIL or *numMatches is 0, *matches will NOT be freed by
 * caller.
 */
typedef int (*opt_expand_cb_T)(optexpand_T *args, int *numMatches, char_u ***matches);

// Flags for assignment functions.
#define ASSIGN_VAR	0     // ":var" (nothing special)
#define ASSIGN_FINAL	0x01  // ":final"
#define ASSIGN_CONST	0x02  // ":const"
#define ASSIGN_NO_DECL	0x04  // "name = expr" without ":let"/":const"/":final"
#define ASSIGN_DECL	0x08  // may declare variable if it does not exist
#define ASSIGN_UNPACK	0x10  // using [a, b] = list
#define ASSIGN_NO_MEMBER_TYPE 0x20 // use "any" for list and dict member type
#define ASSIGN_FOR_LOOP 0x40 // assigning to loop variable
#define ASSIGN_INIT	0x80 // not assigning a value, just a declaration
#define ASSIGN_UPDATE_BLOCK_ID 0x100  // update sav_block_id

#include "ex_cmds.h"	    // Ex command defines
#include "spell.h"	    // spell checking stuff

#include "proto.h"	    // function prototypes

// This has to go after the include of proto.h, as proto/gui.pro declares
// functions of these names. The declarations would break if the defines had
// been seen at that stage.  But it must be before globals.h, where error_ga
// is declared.
#if !defined(MSWIN) && !defined(FEAT_GUI_X11) && !defined(FEAT_GUI_HAIKU) \
	&& !defined(FEAT_GUI_GTK) && !defined(PROTO)
# define mch_errmsg(str)	fprintf(stderr, "%s", (str))
# define display_errors()	fflush(stderr)
# define mch_msg(str)		printf("%s", (str))
#else
# define USE_MCH_ERRMSG
#endif

# if defined(FEAT_EVAL) \
	&& (!defined(FEAT_GUI_MSWIN) || !defined(FEAT_MBYTE_IME))
// Whether IME is supported by im_get_status() defined in mbyte.c.
// For Win32 GUI it's in gui_w32.c when FEAT_MBYTE_IME is defined.
# define IME_WITHOUT_XIM
#endif

#if defined(FEAT_XIM) \
	|| defined(IME_WITHOUT_XIM) \
	|| (defined(FEAT_GUI_MSWIN) && defined(FEAT_MBYTE_IME))
// im_set_active() is available
# define HAVE_INPUT_METHOD
#endif

#ifndef FEAT_LINEBREAK
// Without the 'numberwidth' option line numbers are always 7 chars.
# define number_width(x) 7
#endif

// This must come after including proto.h.
// For VMS this is defined in macros.h.
#if !defined(MSWIN) && !defined(VMS)
# define mch_open(n, m, p)	open((n), (m), (p))
# define mch_fopen(n, p)	fopen((n), (p))
#endif

#include "globals.h"	    // global variables and messages
#include "errors.h"	    // error messages

/*
 * If console dialog not supported, but GUI dialog is, use the GUI one.
 */
#if defined(FEAT_GUI_DIALOG) && !defined(FEAT_CON_DIALOG)
# define do_dialog gui_mch_dialog
#endif

/*
 * Default filters for gui_mch_browse().
 * The filters are almost system independent.  Except for the difference
 * between "*" and "*.*" for MSDOS-like systems.
 * NOTE: Motif only uses the very first pattern.  Therefore
 * BROWSE_FILTER_DEFAULT should start with a "*" pattern.
 */
#ifdef FEAT_BROWSE
# ifdef BACKSLASH_IN_FILENAME
#  define BROWSE_FILTER_MACROS \
	(char_u *)N_("Vim macro files (*.vim)\t*.vim\nAll Files (*.*)\t*.*\n")
#  define BROWSE_FILTER_ALL_FILES (char_u *)N_("All Files (*.*)\t*.*\n")
#  define BROWSE_FILTER_DEFAULT \
	(char_u *)N_("All Files (*.*)\t*.*\nC source (*.c, *.h)\t*.c;*.h\nC++ source (*.cpp, *.hpp)\t*.cpp;*.hpp\nVB code (*.bas, *.frm)\t*.bas;*.frm\nVim files (*.vim, _vimrc, _gvimrc)\t*.vim;_vimrc;_gvimrc\n")
# else
#  define BROWSE_FILTER_MACROS \
	(char_u *)N_("Vim macro files (*.vim)\t*.vim\nAll Files (*)\t*\n")
#  define BROWSE_FILTER_ALL_FILES (char_u *)N_("All Files (*)\t*\n")
#  define BROWSE_FILTER_DEFAULT \
	(char_u *)N_("All Files (*)\t*\nC source (*.c, *.h)\t*.c;*.h\nC++ source (*.cpp, *.hpp)\t*.cpp;*.hpp\nVim files (*.vim, _vimrc, _gvimrc)\t*.vim;_vimrc;_gvimrc\n")
# endif
# define BROWSE_SAVE 1	    // flag for do_browse()
# define BROWSE_DIR 2	    // flag for do_browse()
#endif

#ifdef _MSC_VER
// Avoid useless warning "conversion from X to Y of greater size".
 #pragma warning(disable : 4312)
// Avoid warning for old style function declarators
 #pragma warning(disable : 4131)
// Avoid warning for conversion to type with smaller range
 #pragma warning(disable : 4244)
// Avoid warning for conversion to larger size
 #pragma warning(disable : 4306)
// Avoid warning for unreferenced formal parameter
 #pragma warning(disable : 4100)
// Avoid warning for differs in indirection to slightly different base type
 #pragma warning(disable : 4057)
// Avoid warning for constant conditional expression
 #pragma warning(disable : 4127)
// Avoid warning for assignment within conditional
 #pragma warning(disable : 4706)
#endif

// Note: a NULL argument for vim_realloc() is not portable, don't use it.
#if defined(MEM_PROFILE)
# define vim_realloc(ptr, size)  mem_realloc((ptr), (size))
#else
# define vim_realloc(ptr, size)  realloc((ptr), (size))
#endif

/*
 * Return byte length of character that starts with byte "b".
 * Returns 1 for a single-byte character.
 * MB_BYTE2LEN_CHECK() can be used to count a special key as one byte.
 * Don't call MB_BYTE2LEN(b) with b < 0 or b > 255!
 */
#define MB_BYTE2LEN(b)		mb_bytelen_tab[b]
#define MB_BYTE2LEN_CHECK(b)	(((b) < 0 || (b) > 255) ? 1 : mb_bytelen_tab[b])

// properties used in enc_canon_table[] (first three mutually exclusive)
#define ENC_8BIT	0x01
#define ENC_DBCS	0x02
#define ENC_UNICODE	0x04

#define ENC_ENDIAN_B	0x10	    // Unicode: Big endian
#define ENC_ENDIAN_L	0x20	    // Unicode: Little endian

#define ENC_2BYTE	0x40	    // Unicode: UCS-2
#define ENC_4BYTE	0x80	    // Unicode: UCS-4
#define ENC_2WORD	0x100	    // Unicode: UTF-16

#define ENC_LATIN1	0x200	    // Latin1
#define ENC_LATIN9	0x400	    // Latin9
#define ENC_MACROMAN	0x800	    // Mac Roman (not Macro Man! :-)

#ifdef USE_ICONV
# ifndef EILSEQ
#  define EILSEQ 123
# endif
# ifdef DYNAMIC_ICONV
// On Win32 iconv.dll is dynamically loaded.
#  define ICONV_ERRNO (*iconv_errno())
#  define ICONV_E2BIG  7
#  define ICONV_EINVAL 22
#  define ICONV_EILSEQ 42
# else
#  define ICONV_ERRNO errno
#  define ICONV_E2BIG  E2BIG
#  define ICONV_EINVAL EINVAL
#  define ICONV_EILSEQ EILSEQ
# endif
#endif

#define SIGN_BYTE 1	    // byte value used where sign is displayed;
			    // attribute value is sign type

#ifdef FEAT_NETBEANS_INTG
# define MULTISIGN_BYTE 2   // byte value used where sign is displayed if
			    // multiple signs exist on the line
#endif

#if defined(FEAT_GUI) && defined(FEAT_XCLIPBOARD)
# ifdef FEAT_GUI_GTK
   // Avoid using a global variable for the X display.  It's ugly
   // and is likely to cause trouble in multihead environments.
#  define X_DISPLAY	((gui.in_use) ? gui_mch_get_display() : xterm_dpy)
# else
#  define X_DISPLAY	(gui.in_use ? gui.dpy : xterm_dpy)
# endif
#else
# ifdef FEAT_GUI
#  ifdef FEAT_GUI_GTK
#   define X_DISPLAY	((gui.in_use) ? gui_mch_get_display() : (Display *)NULL)
#  else
#   define X_DISPLAY	gui.dpy
#  endif
# else
#  define X_DISPLAY	xterm_dpy
# endif
#endif

#if defined(FEAT_BROWSE) && defined(GTK_CHECK_VERSION)
# if GTK_CHECK_VERSION(2,4,0)
#  define USE_FILE_CHOOSER
# endif
#endif

#ifdef FEAT_GUI_GTK
# if !GTK_CHECK_VERSION(2,14,0)
#  define gtk_widget_get_window(wid)	((wid)->window)
#  define gtk_plug_get_socket_window(wid)	((wid)->socket_window)
#  define gtk_selection_data_get_data(sel)	((sel)->data)
#  define gtk_selection_data_get_data_type(sel)	((sel)->type)
#  define gtk_selection_data_get_format(sel)	((sel)->format)
#  define gtk_selection_data_get_length(sel)	((sel)->length)
#  define gtk_adjustment_set_lower(adj, low) \
    do { (adj)->lower = low; } while (0)
#  define gtk_adjustment_set_upper(adj, up) \
    do { (adj)->upper = up; } while (0)
#  define gtk_adjustment_set_page_size(adj, size) \
    do { (adj)->page_size = size; } while (0)
#  define gtk_adjustment_set_page_increment(adj, inc) \
    do { (adj)->page_increment = inc; } while (0)
#  define gtk_adjustment_set_step_increment(adj, inc) \
    do { (adj)->step_increment = inc; } while (0)
# endif
# if !GTK_CHECK_VERSION(2,16,0)
#  define gtk_selection_data_get_selection(sel)	((sel)->selection)
# endif
# if !GTK_CHECK_VERSION(2,18,0)
#  define gtk_widget_get_allocation(wid, alloc) \
    do { *(alloc) = (wid)->allocation; } while (0)
#  define gtk_widget_set_allocation(wid, alloc) \
    do { (wid)->allocation = *(alloc); } while (0)
#  define gtk_widget_get_has_window(wid)	!GTK_WIDGET_NO_WINDOW(wid)
#  define gtk_widget_get_sensitive(wid)	GTK_WIDGET_SENSITIVE(wid)
#  define gtk_widget_get_visible(wid)	GTK_WIDGET_VISIBLE(wid)
#  define gtk_widget_has_focus(wid)	GTK_WIDGET_HAS_FOCUS(wid)
#  define gtk_widget_set_window(wid, win) \
    do { (wid)->window = (win); } while (0)
#  define gtk_widget_set_can_default(wid, can) \
    do { if (can) \
	    { GTK_WIDGET_SET_FLAGS(wid, GTK_CAN_DEFAULT); } \
	else \
	    { GTK_WIDGET_UNSET_FLAGS(wid, GTK_CAN_DEFAULT); } } while (0)
#  define gtk_widget_set_can_focus(wid, can) \
    do { if (can) \
	    { GTK_WIDGET_SET_FLAGS(wid, GTK_CAN_FOCUS); } \
	else \
	    { GTK_WIDGET_UNSET_FLAGS(wid, GTK_CAN_FOCUS); } } while (0)
#  define gtk_widget_set_visible(wid, vis) \
    do { if (vis) \
	    { gtk_widget_show(wid); } \
	else \
	    { gtk_widget_hide(wid); } } while (0)
# endif
# if !GTK_CHECK_VERSION(2,20,0)
#  define gtk_widget_get_mapped(wid)	GTK_WIDGET_MAPPED(wid)
#  define gtk_widget_get_realized(wid)	GTK_WIDGET_REALIZED(wid)
#  define gtk_widget_set_mapped(wid, map) \
    do { if (map) \
	    { GTK_WIDGET_SET_FLAGS(wid, GTK_MAPPED); } \
	else \
	    { GTK_WIDGET_UNSET_FLAGS(wid, GTK_MAPPED); } } while (0)
#  define gtk_widget_set_realized(wid, rea) \
    do { if (rea) \
	    { GTK_WIDGET_SET_FLAGS(wid, GTK_REALIZED); } \
	else \
	    { GTK_WIDGET_UNSET_FLAGS(wid, GTK_REALIZED); } } while (0)
# endif
#endif

#ifndef FEAT_NETBEANS_INTG
# undef NBDEBUG
#endif
#ifdef NBDEBUG // Netbeans debugging.
# include "nbdebug.h"
#else
# define nbdebug(a)
#endif

#ifdef IN_PERL_FILE
  /*
   * Avoid clashes between Perl and Vim namespace.
   */
# undef STRLEN
# undef FF
# undef OP_DELETE
# undef OP_JOIN
  // remove MAX and MIN, included by glib.h, redefined by sys/param.h
# ifdef MAX
#  undef MAX
# endif
# ifdef MIN
#  undef MIN
# endif
  // We use _() for gettext(), Perl uses it for function prototypes...
# ifdef _
#  undef _
# endif
# ifdef DEBUG
#  undef DEBUG
# endif
# ifdef _DEBUG
#  undef _DEBUG
# endif
# ifdef instr
#  undef instr
# endif
  // bool may cause trouble on some old versions of Mac OS X but is required
  // on a few other systems and for Perl
# if (defined(MACOS_X) && !defined(MAC_OS_X_VERSION_10_6)) \
				       && defined(bool) && !defined(FEAT_PERL)
#  undef bool
# endif

#endif

// values for vim_handle_signal() that are not a signal
#define SIGNAL_BLOCK	(-1)
#define SIGNAL_UNBLOCK  (-2)
#if !defined(UNIX) && !defined(VMS)
# define vim_handle_signal(x) 0
#endif

// flags for skip_vimgrep_pat()
#define VGR_GLOBAL	1
#define VGR_NOJUMP	2
#define VGR_FUZZY	4

// behavior for bad character, "++bad=" argument
#define BAD_REPLACE	'?'	// replace it with '?' (default)
#define BAD_KEEP	(-1)	// leave it
#define BAD_DROP	(-2)	// erase it

// last argument for do_source()
#define DOSO_NONE	0
#define DOSO_VIMRC	1	// loading vimrc file
#define DOSO_GVIMRC	2	// loading gvimrc file

// flags for read_viminfo() and children
#define VIF_WANT_INFO	    1	// load non-mark info
#define VIF_WANT_MARKS	    2	// load file marks
#define VIF_ONLY_CURBUF	    4	// bail out after loading marks for curbuf
#define VIF_FORCEIT	    8	// overwrite info already read
#define VIF_GET_OLDFILES    16	// load v:oldfiles

// flags for buf_freeall()
#define BFA_DEL		 1	// buffer is going to be deleted
#define BFA_WIPE	 2	// buffer is going to be wiped out
#define BFA_KEEP_UNDO	 4	// do not free undo information
#define BFA_IGNORE_ABORT 8	// do not abort for aborting()

// direction for nv_mousescroll() and ins_mousescroll()
#define MSCR_DOWN	0	// DOWN must be FALSE
#define MSCR_UP		1
#define MSCR_LEFT	(-1)
#define MSCR_RIGHT	(-2)

#define KEYLEN_PART_KEY (-1)	// keylen value for incomplete key-code
#define KEYLEN_PART_MAP (-2)	// keylen value for incomplete mapping
#define KEYLEN_REMOVED  9999	// keylen value for removed sequence

// Return values from win32_fileinfo().
#define FILEINFO_OK	     0
#define FILEINFO_ENC_FAIL    1	// enc_to_utf16() failed
#define FILEINFO_READ_FAIL   2	// CreateFile() failed
#define FILEINFO_INFO_FAIL   3	// GetFileInformationByHandle() failed

// Return value from get_option_value_strict
#define SOPT_BOOL	0x01	// Boolean option
#define SOPT_NUM	0x02	// Number option
#define SOPT_STRING	0x04	// String option
#define SOPT_GLOBAL	0x08	// Option has global value
#define SOPT_WIN	0x10	// Option has window-local value
#define SOPT_BUF	0x20	// Option has buffer-local value
#define SOPT_UNSET	0x40	// Option does not have local value set

// Option types for various functions in option.c
#define SREQ_GLOBAL	0	// Request global option
#define SREQ_WIN	1	// Request window-local option
#define SREQ_BUF	2	// Request buffer-local option

// Flags for get_reg_contents
#define GREG_NO_EXPR	1	// Do not allow expression register
#define GREG_EXPR_SRC	2	// Return expression itself for "=" register
#define GREG_LIST	4	// Return list

// Character used as separated in autoload function/variable names.
#define AUTOLOAD_CHAR '#'

#ifdef FEAT_JOB_CHANNEL
# define MAX_OPEN_CHANNELS 10
#else
# define MAX_OPEN_CHANNELS 0
#endif

#if defined(MSWIN)
# define MAX_NAMED_PIPE_SIZE 65535
#endif

// Options for json_encode() and json_decode.
#define JSON_JS		1   // use JS instead of JSON
#define JSON_NO_NONE	2   // v:none item not allowed
#define JSON_NL		4   // append a NL

// Used for flags of do_in_path()
#define DIP_ALL	    0x01	// all matches, not just the first one
#define DIP_DIR	    0x02	// find directories instead of files.
#define DIP_ERR	    0x04	// give an error message when none found.
#define DIP_START   0x08	// also use "start" directory in 'packpath'
#define DIP_OPT	    0x10	// also use "opt" directory in 'packpath'
#define DIP_NORTP   0x20	// do not use 'runtimepath'
#define DIP_NOAFTER 0x40	// skip "after" directories
#define DIP_AFTER   0x80	// only use "after" directories

// Lowest number used for window ID. Cannot have this many windows.
#define LOWEST_WIN_ID 1000

// Used by the garbage collector.
#define COPYID_INC 2
#define COPYID_MASK (~0x1)

// Values for trans_function_name() argument:
#define TFN_INT		0x01	// internal function name OK
#define TFN_QUIET	0x02	// no error messages
#define TFN_NO_AUTOLOAD	0x04	// do not use script autoloading
#define TFN_NO_DEREF	0x08	// do not dereference a Funcref
#define TFN_READ_ONLY	0x10	// will not change the var
#define TFN_NO_DECL	0x20	// only used for GLV_NO_DECL
#define TFN_COMPILING	0x40	// only used for GLV_COMPILING
#define TFN_NEW_FUNC	0x80	// defining a new function
#define TFN_ASSIGN_WITH_OP 0x100  // only for GLV_ASSIGN_WITH_OP
#define TFN_IN_CLASS	0x200	// function in a class

// Values for get_lval() flags argument:
#define GLV_QUIET	TFN_QUIET	// no error messages
#define GLV_NO_AUTOLOAD	TFN_NO_AUTOLOAD	// do not use script autoloading
#define GLV_READ_ONLY	TFN_READ_ONLY	// will not change the var
#define GLV_NO_DECL	TFN_NO_DECL	// assignment without :var or :let
#define GLV_COMPILING	TFN_COMPILING	// variable may be defined later
#define GLV_ASSIGN_WITH_OP TFN_ASSIGN_WITH_OP // assignment with operator
#define GLV_PREFER_FUNC	0x10000		// prefer function above variable
#define GLV_FOR_LOOP	0x20000		// assigning to a loop variable

#define DO_NOT_FREE_CNT 99999	// refcount for dict or list that should not
				// be freed.

// fixed buffer length for fname_trans_sid()
#define FLEN_FIXED 40

// flags for find_name_end()
#define FNE_INCL_BR	1	// include [] in name
#define FNE_CHECK_START	2	// check name starts with valid character
#define FNE_ALLOW_CURLY	4	// always allow curly braces name

// BSD is supposed to cover FreeBSD and similar systems.
#if (defined(SUN_SYSTEM) || defined(BSD) || defined(__FreeBSD_kernel__)) \
	&& (defined(S_ISCHR) || defined(S_IFCHR))
# define OPEN_CHR_FILES
#endif

// stat macros
#ifndef S_ISDIR
# ifdef S_IFDIR
#  define S_ISDIR(m)	(((m) & S_IFMT) == S_IFDIR)
# else
#  define S_ISDIR(m)	0
# endif
#endif
#ifndef S_ISREG
# ifdef S_IFREG
#  define S_ISREG(m)	(((m) & S_IFMT) == S_IFREG)
# else
#  define S_ISREG(m)	0
# endif
#endif
#ifndef S_ISBLK
# ifdef S_IFBLK
#  define S_ISBLK(m)	(((m) & S_IFMT) == S_IFBLK)
# else
#  define S_ISBLK(m)	0
# endif
#endif
#ifndef S_ISSOCK
# ifdef S_IFSOCK
#  define S_ISSOCK(m)	(((m) & S_IFMT) == S_IFSOCK)
# else
#  define S_ISSOCK(m)	0
# endif
#endif
#ifndef S_ISFIFO
# ifdef S_IFIFO
#  define S_ISFIFO(m)	(((m) & S_IFMT) == S_IFIFO)
# else
#  define S_ISFIFO(m)	0
# endif
#endif
#ifndef S_ISCHR
# ifdef S_IFCHR
#  define S_ISCHR(m)	(((m) & S_IFMT) == S_IFCHR)
# else
#  define S_ISCHR(m)	0
# endif
#endif
#ifndef S_ISLNK
# ifdef S_IFLNK
#  define S_ISLNK(m)	(((m) & S_IFMT) == S_IFLNK)
# else
#  define S_ISLNK(m)	0
# endif
#endif

#if defined(HAVE_GETTIMEOFDAY) && defined(HAVE_SYS_TIME_H)
# define ELAPSED_TIMEVAL
# define ELAPSED_INIT(v) gettimeofday(&(v), NULL)
# define ELAPSED_FUNC(v) elapsed(&(v))
typedef struct timeval elapsed_T;
long elapsed(struct timeval *start_tv);
#elif defined(MSWIN)
# define ELAPSED_TICKCOUNT
# define ELAPSED_INIT(v) v = GetTickCount()
# define ELAPSED_FUNC(v) elapsed(v)
# ifdef PROTO
typedef int DWORD;
# endif
typedef DWORD elapsed_T;
# ifndef PROTO
long elapsed(DWORD start_tick);
# endif
#endif

// Replacement for nchar used by nv_replace().
#define REPLACE_CR_NCHAR    (-1)
#define REPLACE_NL_NCHAR    (-2)

// flags for term_start()
#define TERM_START_NOJOB	1
#define TERM_START_FORCEIT	2
#define TERM_START_SYSTEM	4

// Used for icon/title save and restore.
#define SAVE_RESTORE_TITLE	1
#define SAVE_RESTORE_ICON	2
#define SAVE_RESTORE_BOTH	(SAVE_RESTORE_TITLE | SAVE_RESTORE_ICON)

// Flags for adjust_prop_columns()
#define APC_SAVE_FOR_UNDO	1   // call u_savesub() before making changes
#define APC_SUBSTITUTE		2   // text is replaced, not inserted
#define APC_INDENT		4   // changing indent

#define CLIP_ZINDEX 32000

// Flags for replace_termcodes()
#define REPTERM_FROM_PART	1
#define REPTERM_DO_LT		2
#define REPTERM_SPECIAL		4
#define REPTERM_NO_SIMPLIFY	8

// Flags for find_special_key()
#define FSK_KEYCODE	0x01	// prefer key code, e.g. K_DEL instead of DEL
#define FSK_KEEP_X_KEY	0x02	// don't translate xHome to Home key
#define FSK_IN_STRING	0x04	// TRUE in string, double quote is escaped
#define FSK_SIMPLIFY	0x08	// simplify <C-H> and <A-x>
#define FSK_FROM_PART	0x10	// left-hand-side of mapping

// Flags for the readdirex function, how to sort the result
#define READDIR_SORT_NONE	0  // do not sort
#define READDIR_SORT_BYTE	1  // sort by byte order (strcmp), default
#define READDIR_SORT_IC		2  // sort ignoring case (strcasecmp)
#define READDIR_SORT_COLLATE	3  // sort according to collation (strcoll)

// Flags for mch_delay.
#define MCH_DELAY_IGNOREINPUT	1
#define MCH_DELAY_SETTMODE	2

// Flags for eval_variable().
#define EVAL_VAR_VERBOSE	1   // may give error message
#define EVAL_VAR_NOAUTOLOAD	2   // do not use script autoloading
#define EVAL_VAR_IMPORT		4   // may return special variable for import
#define EVAL_VAR_NO_FUNC	8   // do not look for a function

// Maximum number of characters that can be fuzzy matched
#define MAX_FUZZY_MATCHES	256

// flags for equal_type()
#define ETYPE_ARG_UNKNOWN 1

// flags used by user commands and :autocmd
#define UC_BUFFER	1	// -buffer: local to current buffer
#define UC_VIM9		2	// {} argument: Vim9 syntax.

// flags used by vim_strsave_fnameescape()
#define VSE_NONE	0
#define VSE_SHELL	1	// escape for a shell command
#define VSE_BUFFER	2	// escape for a ":buffer" command

// Flags used by find_func_even_dead()
#define FFED_IS_GLOBAL	1	// "g:" was used
#define FFED_NO_GLOBAL	2	// only check for script-local functions

#define MAX_LSHIFT_BITS (varnumber_T)((sizeof(uvarnumber_T) * 8) - 1)

// Flags used by "class_flags" of define_function()
#define CF_CLASS	1	// inside a class
#define CF_INTERFACE	2	// inside an interface
#define CF_ABSTRACT_METHOD	4	// inside an abstract class

#endif // VIM__H
