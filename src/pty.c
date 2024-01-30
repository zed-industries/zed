/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 * The stuff in this file mostly comes from the "screen" program.
 * Included with permission from Juergen Weigert.
 * Copied from "pty.c".  "putenv.c" was used for putenv() in misc2.c.
 *
 * It has been modified to work better with Vim.
 * The parts that are not used in Vim have been deleted.
 * See the "screen" sources for the complete stuff.
 *
 * This specific version is distributed under the Vim license (attribution by
 * Juergen Weigert), the GPL applies to the original version, see the
 * copyright notice below.
 */

// Copyright (c) 1993
//	Juergen Weigert (jnweiger@immd4.informatik.uni-erlangen.de)
//	Michael Schroeder (mlschroe@immd4.informatik.uni-erlangen.de)
// Copyright (c) 1987 Oliver Laumann
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program (see the file COPYING); if not, write to the
// Free Software Foundation, Inc.,
// 59 Temple Place - Suite 330, Boston, MA  02111-1307, USA

#include "vim.h"

#if defined(FEAT_GUI) || defined(FEAT_JOB_CHANNEL)

#include <signal.h>

#ifdef HAVE_SYS_IOCTL_H
# include <sys/ioctl.h>
#endif

#if HAVE_STROPTS_H
# include <sys/types.h>
# ifdef sinix
#  define buf_T __system_buf_t__
# endif
# include <stropts.h>
# ifdef sinix
#  undef buf_T
# endif
# ifdef SUN_SYSTEM
#  include <sys/conf.h>
#  if defined(HAVE_SYS_PTMS_H) && defined(HAVE_SVR4_PTYS)
#   include <sys/ptms.h>
#  endif
# endif
#endif

#ifdef HAVE_UNISTD_H
# include <unistd.h>
#endif

#if HAVE_TERMIO_H
# include <termio.h>
#else
# ifdef HAVE_TERMIOS_H
#  include <termios.h>
# endif
#endif

#if HAVE_SYS_STREAM_H
# include <sys/stream.h>
#endif

#if HAVE_SYS_PTEM_H
# include <sys/ptem.h>
#endif

#if !defined(SUN_SYSTEM) && !defined(VMS)
# include <sys/ioctl.h>
#endif

#if defined(SUN_SYSTEM) && defined(LOCKPTY) && !defined(TIOCEXCL)
# include <sys/ttold.h>
#endif

#ifdef ISC
# include <sys/tty.h>
# include <sys/sioctl.h>
# include <sys/pty.h>
#endif

#ifdef sgi
# include <sys/sysmacros.h>
#endif

#if defined(_INCLUDE_HPUX_SOURCE) && !defined(hpux)
# define hpux
#endif

/*
 * if no PTYRANGE[01] is in the config file, we pick a default
 */
#ifndef PTYRANGE0
# define PTYRANGE0 "qprs"
#endif
#ifndef PTYRANGE1
# define PTYRANGE1 "0123456789abcdef"
#endif

// SVR4 pseudo ttys don't seem to work with SCO-5
#ifdef M_UNIX
# undef HAVE_SVR4_PTYS
#endif

/*
 *  Open all ptys with O_NOCTTY, just to be on the safe side.
 */
#ifndef O_NOCTTY
# define O_NOCTTY 0
#endif

#if defined(HAVE_SVR4_PTYS) || defined(HAVE_POSIX_OPENPT)
// These should be in stdlib.h, but it depends on _XOPEN_SOURCE.
char *ptsname(int);
int unlockpt(int);
int grantpt(int);
int posix_openpt(int flags);
#endif

    static void
initmaster(int f UNUSED)
{
#ifndef VMS
# ifdef POSIX
    tcflush(f, TCIOFLUSH);
# else
#  ifdef TIOCFLUSH
    (void)ioctl(f, TIOCFLUSH, (char *) 0);
#  endif
# endif
# ifdef LOCKPTY
    (void)ioctl(f, TIOCEXCL, (char *) 0);
# endif
#endif
}

/*
 * This causes a hang on some systems, but is required for a properly working
 * pty on others.  Needs to be tuned...
 */
    int
setup_slavepty(int fd)
{
    if (fd < 0)
	return 0;
#if defined(I_PUSH) && defined(HAVE_SVR4_PTYS) && !defined(sgi) \
	&& !defined(linux) && !defined(__osf__) && !defined(M_UNIX)
# if defined(HAVE_SYS_PTEM_H) || defined(hpux)
    if (ioctl(fd, I_PUSH, "ptem") != 0)
	return -1;
# endif
    if (ioctl(fd, I_PUSH, "ldterm") != 0)
	return -1;
# ifdef SUN_SYSTEM
    if (ioctl(fd, I_PUSH, "ttcompat") != 0)
	return -1;
# endif
#endif
    return 0;
}

#if defined(HAVE_POSIX_OPENPT) && !defined(PTY_DONE)
#define PTY_DONE
    int
mch_openpty(char **ttyn)
{
    int			f;
    char		*m;
    sighandler_T	sigcld;
    static char TtyName[32];  // used for opening a new pty-pair

    if ((f = posix_openpt(O_RDWR | O_NOCTTY | O_EXTRA)) == -1)
	return -1;

    // SIGCHLD set to SIG_DFL for grantpt() because it fork()s and
    // exec()s pt_chmod
    sigcld = mch_signal(SIGCHLD, SIG_DFL);
    if ((m = ptsname(f)) == NULL || grantpt(f) || unlockpt(f))
    {
	mch_signal(SIGCHLD, sigcld);
	close(f);
	return -1;
    }
    mch_signal(SIGCHLD, sigcld);
    vim_strncpy((char_u *)TtyName, (char_u *)m, sizeof(TtyName) - 1);
    initmaster(f);
    *ttyn = TtyName;
    return f;
}
#endif

#if defined(OSX) && !defined(PTY_DONE)
#define PTY_DONE
    int
mch_openpty(char **ttyn)
{
    int		f;
    static char TtyName[32];

    if ((f = open_controlling_pty(TtyName)) < 0)
	return -1;
    initmaster(f);
    *ttyn = TtyName;
    return f;
}
#endif

#if (defined(sequent) || defined(_SEQUENT_)) && defined(HAVE_GETPSEUDOTTY) \
	&& !defined(PTY_DONE)
#define PTY_DONE
    int
mch_openpty(char **ttyn)
{
    char	*m, *s;
    int		f;
    // used for opening a new pty-pair:
    static char PtyName[32];
    static char TtyName[32];

    if ((f = getpseudotty(&s, &m)) < 0)
	return -1;
#ifdef _SEQUENT_
    fvhangup(s);
#endif
    vim_strncpy((char_u *)PtyName, (char_u *)m, sizeof(PtyName) - 1);
    vim_strncpy((char_u *)TtyName, (char_u *)s, sizeof(TtyName) - 1);
    initmaster(f);
    *ttyn = TtyName;
    return f;
}
#endif

#if defined(MIPS) && defined(HAVE_DEV_PTC) && !defined(PTY_DONE)
#define PTY_DONE
    int
mch_openpty(char **ttyn)
{
    int		f;
    stat_T	buf;
    // used for opening a new pty-pair:
    static char TtyName[32];

    if ((f = open("/dev/ptc", O_RDWR | O_NOCTTY | O_NONBLOCK | O_EXTRA, 0)) < 0)
	return -1;
    if (mch_fstat(f, &buf) < 0)
    {
	close(f);
	return -1;
    }
    sprintf(TtyName, "/dev/ttyq%d", minor(buf.st_rdev));
    initmaster(f);
    *ttyn = TtyName;
    return f;
}
#endif

#if defined(HAVE_SVR4_PTYS) && !defined(PTY_DONE) && !defined(hpux) \
	    && !(defined(MACOS_X) && !defined(MAC_OS_X_VERSION_10_6))

// NOTE: Even though HPUX can have /dev/ptmx, the code below doesn't work!
// Same for Mac OS X Leopard (10.5).
#define PTY_DONE
    int
mch_openpty(char **ttyn)
{
    int			f;
    char		*m;
    sighandler_T	sigcld;
    // used for opening a new pty-pair:
    static char TtyName[32];

    if ((f = open("/dev/ptmx", O_RDWR | O_NOCTTY | O_EXTRA, 0)) == -1)
	return -1;

    /*
     * SIGCHLD set to SIG_DFL for grantpt() because it fork()s and
     * exec()s pt_chmod
     */
    sigcld = mch_signal(SIGCHLD, SIG_DFL);
    if ((m = ptsname(f)) == NULL || grantpt(f) || unlockpt(f))
    {
	mch_signal(SIGCHLD, sigcld);
	close(f);
	return -1;
    }
    mch_signal(SIGCHLD, sigcld);
    vim_strncpy((char_u *)TtyName, (char_u *)m, sizeof(TtyName) - 1);
    initmaster(f);
    *ttyn = TtyName;
    return f;
}
#endif

#if defined(_AIX) && defined(HAVE_DEV_PTC) && !defined(PTY_DONE)
#define PTY_DONE

#ifdef _IBMR2
static int aixhack = -1;
#endif

    int
mch_openpty(char **ttyn)
{
    int		f;
    // used for opening a new pty-pair:
    static char TtyName[32];

    // a dumb looking loop replaced by mycrofts code:
    if ((f = open("/dev/ptc", O_RDWR | O_NOCTTY | O_EXTRA)) < 0)
	return -1;
    vim_strncpy((char_u *)TtyName, (char_u *)ttyname(f), sizeof(TtyName) - 1);
    if (geteuid() != ROOT_UID && mch_access(TtyName, R_OK | W_OK))
    {
	close(f);
	return -1;
    }
    initmaster(f);
# ifdef _IBMR2
    if (aixhack >= 0)
	close(aixhack);
    if ((aixhack = open(TtyName, O_RDWR | O_NOCTTY | O_EXTRA, 0)) < 0)
    {
	close(f);
	return -1;
    }
# endif
    *ttyn = TtyName;
    return f;
}
#endif

#ifndef PTY_DONE

# ifdef hpux
static char PtyProto[] = "/dev/ptym/ptyXY";
static char TtyProto[] = "/dev/pty/ttyXY";
# else
#  ifdef __HAIKU__
static char PtyProto[] = "/dev/pt/XY";
static char TtyProto[] = "/dev/tt/XY";
#  else
static char PtyProto[] = "/dev/ptyXY";
static char TtyProto[] = "/dev/ttyXY";
#  endif
# endif

    int
mch_openpty(char **ttyn)
{
    char	*p, *q, *l, *d;
    int		f;
    // used for opening a new pty-pair:
    static char PtyName[32];
    static char TtyName[32];

    strcpy(PtyName, PtyProto);
    strcpy(TtyName, TtyProto);
    for (p = PtyName; *p != 'X'; p++)
	;
    for (q = TtyName; *q != 'X'; q++)
	;
    for (l = PTYRANGE0; (*p = *l) != '\0'; l++)
    {
	for (d = PTYRANGE1; (p[1] = *d) != '\0'; d++)
	{
	    if ((f = open(PtyName, O_RDWR | O_NOCTTY | O_EXTRA, 0)) == -1)
		continue;
	    q[0] = *l;
	    q[1] = *d;
	    if (geteuid() != ROOT_UID && mch_access(TtyName, R_OK | W_OK))
	    {
		close(f);
		continue;
	    }
#if defined(SUN_SYSTEM) && defined(TIOCGPGRP) && !defined(SUNOS3)
	    // Hack to ensure that the slave side of the pty is
	    // unused. May not work in anything other than SunOS4.1
	    {
		int pgrp;

		// tcgetpgrp does not work (uses TIOCGETPGRP)!
		if (ioctl(f, TIOCGPGRP, (char *)&pgrp) != -1 || errno != EIO)
		{
		    close(f);
		    continue;
		}
	    }
#endif
	    initmaster(f);
	    *ttyn = TtyName;
	    return f;
	}
    }
    return -1;
}
#endif

/*
 * Call isatty(fd), except for SunOS where it's done differently.
 */
    int
mch_isatty(int fd)
{
# if defined(I_STR) && defined(HAVE_SYS_PTMS_H) && defined(HAVE_SVR4_PTYS) \
	&& defined(SUN_SYSTEM)
    // On SunOS, isatty() for /dev/ptmx returns false or sometimes can hang up
    // in the inner ioctl(), and therefore first determine whether "fd" is a
    // master device.
    struct strioctl istr;

    istr.ic_cmd = ISPTM;
    istr.ic_timout = 0;
    istr.ic_dp = NULL;
    istr.ic_len = 0;

    if (ioctl(fd, I_STR, &istr) == 0)
	// Trick: return 2 in order to advice the caller that "fd" is a master
	// device. cf. src/os_unix.c:get_tty_fd()
	return 2;
# endif
    return isatty(fd);
}

#endif // FEAT_GUI || FEAT_JOB_CHANNEL
