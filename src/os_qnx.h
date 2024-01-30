/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

#ifdef __QNXNTO__
# include <sys/procmgr.h>
#endif

#define	USE_TMPNAM

#define POSIX	    // Used by pty.c

#if defined(FEAT_GUI_PHOTON)
extern int is_photon_available;
#endif
