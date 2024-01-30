/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *	      OS/2 port by Paul Slootman
 *	      VMS merge by Zoltan Arpadffy
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * os_unix.c -- code for all flavors of Unix (BSD, SYSV, SVR4, POSIX, ...)
 *	     Also for OS/2, using the excellent EMX package!!!
 *	     Also for Atari MiNT.
 *
 * A lot of this file was originally written by Juergen Weigert and later
 * changed beyond recognition.
 */

#include "vim.h"

#ifdef FEAT_MZSCHEME
# include "if_mzsch.h"
#endif

#include "os_unixx.h"	    // unix includes for os_unix.c only

#ifdef USE_XSMP
# include <X11/SM/SMlib.h>
#endif

#ifdef HAVE_SELINUX
# include <selinux/selinux.h>
static int selinux_enabled = -1;
#endif

#ifdef FEAT_XATTR
# include <sys/xattr.h>
#endif

#ifdef HAVE_SMACK
# include <sys/xattr.h>
# include <linux/xattr.h>
# ifndef SMACK_LABEL_LEN
#  define SMACK_LABEL_LEN 1024
# endif
#endif

#ifdef __CYGWIN__
# include <cygwin/version.h>
# include <sys/cygwin.h>	// for cygwin_conv_to_posix_path() and/or
				// for cygwin_conv_path()
# ifdef FEAT_CYGWIN_WIN32_CLIPBOARD
#  define WIN32_LEAN_AND_MEAN
#  include <windows.h>
#  include "winclip.pro"
# endif
#endif

#ifdef FEAT_MOUSE_GPM

# include <gpm.h>

# ifdef DYNAMIC_GPM
#  define Gpm_Open     (*dll_Gpm_Open)
#  define Gpm_Close    (*dll_Gpm_Close)
#  define Gpm_GetEvent (*dll_Gpm_GetEvent)
#  define gpm_flag     (dll_gpm_flag != NULL ? *dll_gpm_flag :  0)
#  define gpm_fd       (dll_gpm_fd   != NULL ? *dll_gpm_fd   : -1)

static int (*dll_Gpm_Open)     (Gpm_Connect *, int);
static int (*dll_Gpm_Close)    (void);
static int (*dll_Gpm_GetEvent) (Gpm_Event *);
static int *dll_gpm_flag;
static int *dll_gpm_fd;

static void *libgpm_hinst;
# endif

// <linux/keyboard.h> contains defines conflicting with "keymap.h",
// I just copied relevant defines here. A cleaner solution would be to put gpm
// code into separate file and include there linux/keyboard.h
// #include <linux/keyboard.h>
# define KG_SHIFT	0
# define KG_CTRL	2
# define KG_ALT		3
# define KG_ALTGR	1
# define KG_SHIFTL	4
# define KG_SHIFTR	5
# define KG_CTRLL	6
# define KG_CTRLR	7
# define KG_CAPSSHIFT	8

static void gpm_close(void);
static int gpm_open(void);
static int mch_gpm_process(void);
#endif

#ifdef FEAT_SYSMOUSE
# include <sys/consio.h>
# include <sys/fbio.h>

static int sysmouse_open(void);
static void sysmouse_close(void);
static void sig_sysmouse SIGPROTOARG;
#endif

/*
 * end of autoconf section. To be extended...
 */

// Are the following #ifdefs still required? And why? Is that for X11?

#if defined(ESIX) || defined(M_UNIX) && !defined(SCO)
# ifdef SIGWINCH
#  undef SIGWINCH
# endif
# ifdef TIOCGWINSZ
#  undef TIOCGWINSZ
# endif
#endif

#if defined(SIGWINDOW) && !defined(SIGWINCH)	// hpux 9.01 has it
# define SIGWINCH SIGWINDOW
#endif

#ifdef FEAT_X11
# include <X11/Xlib.h>
# include <X11/Xutil.h>
# include <X11/Xatom.h>
# ifdef FEAT_XCLIPBOARD
#  include <X11/Intrinsic.h>
#  include <X11/Shell.h>
#  include <X11/StringDefs.h>
static Widget	xterm_Shell = (Widget)0;
static void clip_update(void);
static void xterm_update(void);
# endif

Window	    x11_window = 0;
Display	    *x11_display = NULL;
#endif

static int ignore_sigtstp = FALSE;

static int get_x11_title(int);

static char_u	*oldtitle = NULL;
static volatile sig_atomic_t oldtitle_outdated = FALSE;
static int	unix_did_set_title = FALSE;
static char_u	*oldicon = NULL;
static int	did_set_icon = FALSE;

static void may_core_dump(void);

#ifdef HAVE_UNION_WAIT
typedef union wait waitstatus;
#else
typedef int waitstatus;
#endif
static int  WaitForChar(long msec, int *interrupted, int ignore_input);
static int  WaitForCharOrMouse(long msec, int *interrupted, int ignore_input);
#ifdef VMS
int  RealWaitForChar(int, long, int *, int *interrupted);
#else
static int  RealWaitForChar(int, long, int *, int *interrupted);
#endif

#ifdef FEAT_XCLIPBOARD
static int do_xterm_trace(void);
# define XT_TRACE_DELAY	50	// delay for xterm tracing
#endif

static void handle_resize(void);

#if defined(SIGWINCH)
static void sig_winch SIGPROTOARG;
#endif
#if defined(SIGTSTP)
static void sig_tstp SIGPROTOARG;
// volatile because it is used in signal handler sig_tstp() and
// sigcont_handler().
static volatile sig_atomic_t in_mch_suspend = FALSE;
#endif
#if defined(SIGINT)
static void catch_sigint SIGPROTOARG;
#endif
#if defined(SIGUSR1)
static void catch_sigusr1 SIGPROTOARG;
#endif
#if defined(SIGPWR)
static void catch_sigpwr SIGPROTOARG;
#endif
#if defined(SIGALRM) && defined(FEAT_X11) && !defined(FEAT_GUI_GTK)
# define SET_SIG_ALARM
static void sig_alarm SIGPROTOARG;
// volatile because it is used in signal handler sig_alarm().
static volatile sig_atomic_t sig_alarm_called;
#endif
static void deathtrap SIGPROTOARG;

static void catch_int_signal(void);
static void set_signals(void);
static void catch_signals(void (*func_deadly)(int), void (*func_other)(int));
#ifdef HAVE_SIGPROCMASK
# define SIGSET_DECL(set)	sigset_t set;
# define BLOCK_SIGNALS(set)	block_signals(set)
# define UNBLOCK_SIGNALS(set)	unblock_signals(set)
#else
# define SIGSET_DECL(set)
# define BLOCK_SIGNALS(set)	do { /**/ } while (0)
# define UNBLOCK_SIGNALS(set)	do { /**/ } while (0)
#endif
static int  have_wildcard(int, char_u **);
static int  have_dollars(int, char_u **);

static int save_patterns(int num_pat, char_u **pat, int *num_file, char_u ***file);

#ifndef SIG_ERR
# define SIG_ERR	((sighandler_T)-1)
#endif
#ifndef SIG_HOLD
# define SIG_HOLD	((sighandler_T)-2)
#endif

// volatile because it is used in signal handler sig_winch().
static volatile sig_atomic_t do_resize = FALSE;
// volatile because it is used in signal handler sig_tstp().
static volatile sig_atomic_t got_tstp = FALSE;
static char_u	*extra_shell_arg = NULL;
static int	show_shell_mess = TRUE;
// volatile because it is used in signal handler deathtrap().
static volatile sig_atomic_t deadly_signal = 0;	   // The signal we caught
// volatile because it is used in signal handler deathtrap().
static volatile sig_atomic_t in_mch_delay = FALSE; // sleeping in mch_delay()

#if defined(FEAT_JOB_CHANNEL) && !defined(USE_SYSTEM)
static int dont_check_job_ended = 0;
#endif

// Current terminal mode from mch_settmode().  Can differ from cur_tmode.
static tmode_T mch_cur_tmode = TMODE_COOK;

#ifdef USE_XSMP
typedef struct
{
    SmcConn smcconn;	    // The SM connection ID
    IceConn iceconn;	    // The ICE connection ID
    char *clientid;	    // The client ID for the current smc session
    Bool save_yourself;     // If we're in the middle of a save_yourself
    Bool shutdown;	    // If we're in shutdown mode
} xsmp_config_T;

static xsmp_config_T xsmp;
#endif

#ifdef SYS_SIGLIST_DECLARED
/*
 * I have seen
 *  extern char *_sys_siglist[NSIG];
 * on Linux, NetBSD and Solaris. It contains a nice list of strings
 * that describe the signals. That is nearly what we want here.  But
 * autoconf does only check for sys_siglist (without the underscore), I
 * do not want to change everything today.... jw.
 * This is why AC_DECL_SYS_SIGLIST is commented out in configure.ac.
 */
#endif

static struct signalinfo
{
    int	    sig;	// Signal number, eg. SIGSEGV etc
    char    *name;	// Signal name (not char_u!).
    char    deadly;	// Catch as a deadly signal?
} signal_info[] =
{
#ifdef SIGHUP
    {SIGHUP,	    "HUP",	TRUE},
#endif
#ifdef SIGQUIT
    {SIGQUIT,	    "QUIT",	TRUE},
#endif
#ifdef SIGILL
    {SIGILL,	    "ILL",	TRUE},
#endif
#ifdef SIGTRAP
    {SIGTRAP,	    "TRAP",	TRUE},
#endif
#ifdef SIGABRT
    {SIGABRT,	    "ABRT",	TRUE},
#endif
#ifdef SIGEMT
    {SIGEMT,	    "EMT",	TRUE},
#endif
#ifdef SIGFPE
    {SIGFPE,	    "FPE",	TRUE},
#endif
#ifdef SIGBUS
    {SIGBUS,	    "BUS",	TRUE},
#endif
#if defined(SIGSEGV) && !defined(FEAT_MZSCHEME)
    // MzScheme uses SEGV in its garbage collector
    {SIGSEGV,	    "SEGV",	TRUE},
#endif
#ifdef SIGSYS
    {SIGSYS,	    "SYS",	TRUE},
#endif
#ifdef SIGALRM
    {SIGALRM,	    "ALRM",	FALSE},	// Perl's alarm() can trigger it
#endif
#ifdef SIGTERM
    {SIGTERM,	    "TERM",	TRUE},
#endif
#if defined(SIGVTALRM) && !defined(FEAT_RUBY)
    {SIGVTALRM,	    "VTALRM",	TRUE},
#endif
#if defined(SIGPROF) && !defined(FEAT_MZSCHEME) && !defined(WE_ARE_PROFILING)
    // MzScheme uses SIGPROF for its own needs; On Linux with profiling
    // this makes Vim exit.  WE_ARE_PROFILING is defined in Makefile.
    {SIGPROF,	    "PROF",	TRUE},
#endif
#ifdef SIGXCPU
    {SIGXCPU,	    "XCPU",	TRUE},
#endif
#ifdef SIGXFSZ
    {SIGXFSZ,	    "XFSZ",	TRUE},
#endif
#ifdef SIGUSR1
    {SIGUSR1,	    "USR1",	FALSE},
#endif
#if defined(SIGUSR2) && !defined(FEAT_SYSMOUSE)
    // Used for sysmouse handling
    {SIGUSR2,	    "USR2",	TRUE},
#endif
#ifdef SIGINT
    {SIGINT,	    "INT",	FALSE},
#endif
#ifdef SIGWINCH
    {SIGWINCH,	    "WINCH",	FALSE},
#endif
#ifdef SIGTSTP
    {SIGTSTP,	    "TSTP",	FALSE},
#endif
#ifdef SIGPIPE
    {SIGPIPE,	    "PIPE",	FALSE},
#endif
    {-1,	    "Unknown!", FALSE}
};

    sighandler_T
mch_signal(int sig, sighandler_T func)
{
#if defined(HAVE_SIGACTION) && defined(HAVE_SIGPROCMASK)
    // Modern implementation: use sigaction().
    struct sigaction	sa, old;
    sigset_t		curset;
    int			blocked;

    if (sigprocmask(SIG_BLOCK, NULL, &curset) == -1)
	return SIG_ERR;

    blocked = sigismember(&curset, sig);

    if (func == SIG_HOLD)
    {
	if (blocked)
	    return SIG_HOLD;

	sigemptyset(&curset);
	sigaddset(&curset, sig);

	if (sigaction(sig, NULL, &old) == -1
				|| sigprocmask(SIG_BLOCK, &curset, NULL) == -1)
	    return SIG_ERR;
	return old.sa_handler;
    }

    if (blocked)
    {
	sigemptyset(&curset);
	sigaddset(&curset, sig);

	if (sigprocmask(SIG_UNBLOCK, &curset, NULL) == -1)
	    return SIG_ERR;
    }

    sa.sa_handler = func;
    sigemptyset(&sa.sa_mask);
# ifdef SA_RESTART
    sa.sa_flags = SA_RESTART;
# else
    sa.sa_flags = 0;
# endif
    if (sigaction(sig, &sa, &old) == -1)
	return SIG_ERR;
    return blocked ? SIG_HOLD: old.sa_handler;
#elif defined(HAVE_SIGSET)
    // Using sigset() is preferred above signal().
    return sigset(sig, func);
#else
    // Oldest and most compatible solution.
    return signal(sig, func);
#endif
}

    int
mch_chdir(char *path)
{
    if (p_verbose >= 5)
    {
	verbose_enter();
	smsg("chdir(%s)", path);
	verbose_leave();
    }
#ifdef VMS
    return chdir(vms_fixfilename(path));
#else
    return chdir(path);
#endif
}

// Why is NeXT excluded here (and not in os_unixx.h)?
#if defined(ECHOE) && defined(ICANON) \
    && (defined(HAVE_TERMIO_H) || defined(HAVE_TERMIOS_H)) \
    && !defined(__NeXT__)
# define NEW_TTY_SYSTEM
#endif

/*
 * Write s[len] to the screen (stdout).
 */
    void
mch_write(char_u *s, int len)
{
    vim_ignored = (int)write(1, (char *)s, len);
    if (p_wd)		// Unix is too fast, slow down a bit more
	RealWaitForChar(read_cmd_fd, p_wd, NULL, NULL);
}

/*
 * Function passed to inchar_loop() to handle window resizing.
 * If "check_only" is TRUE: Return whether there was a resize.
 * If "check_only" is FALSE: Deal with the window resized.
 */
    static int
resize_func(int check_only)
{
    if (check_only)
	return do_resize;
    while (do_resize)
    {
#ifdef FEAT_EVAL
	ch_log(NULL, "calling handle_resize() in resize_func()");
#endif
	handle_resize();
    }
    return FALSE;
}

/*
 * mch_inchar(): low level input function.
 * Get a characters from the keyboard.
 * Return the number of characters that are available.
 * If wtime == 0 do not wait for characters.
 * If wtime == n wait a short time for characters.
 * If wtime == -1 wait forever for characters.
 */
    int
mch_inchar(
    char_u	*buf,
    int		maxlen,
    long	wtime,	    // don't use "time", MIPS cannot handle it
    int		tb_change_cnt)
{
    return inchar_loop(buf, maxlen, wtime, tb_change_cnt,
		       WaitForChar, resize_func);
}

    static void
handle_resize(void)
{
    do_resize = FALSE;
    shell_resized();
}

/*
 * Return non-zero if a character is available.
 */
    int
mch_char_avail(void)
{
    return WaitForChar(0L, NULL, FALSE);
}

#if defined(FEAT_TERMINAL) || defined(PROTO)
/*
 * Check for any pending input or messages.
 */
    int
mch_check_messages(void)
{
    return WaitForChar(0L, NULL, TRUE);
}
#endif

#if defined(HAVE_TOTAL_MEM) || defined(PROTO)
# ifdef HAVE_SYS_RESOURCE_H
#  include <sys/resource.h>
# endif
# if defined(HAVE_SYS_SYSCTL_H) && defined(HAVE_SYSCTL)
#  include <sys/sysctl.h>
# endif
# if defined(HAVE_SYS_SYSINFO_H) && defined(HAVE_SYSINFO)
#  include <sys/sysinfo.h>
# endif
# ifdef MACOS_X
#  include <mach/mach_host.h>
#  include <mach/mach_port.h>
# endif

/*
 * Return total amount of memory available in Kbyte.
 * Doesn't change when memory has been allocated.
 */
    long_u
mch_total_mem(int special UNUSED)
{
    long_u	mem = 0;
    long_u	shiftright = 10;  // how much to shift "mem" right for Kbyte

# ifdef MACOS_X
    {
	// Mac (Darwin) way of getting the amount of RAM available
	mach_port_t		host = mach_host_self();
	kern_return_t		kret;
#  ifdef HOST_VM_INFO64
	struct vm_statistics64	vm_stat;
	natural_t		count = HOST_VM_INFO64_COUNT;

	kret = host_statistics64(host, HOST_VM_INFO64,
					     (host_info64_t)&vm_stat, &count);
#  else
	struct vm_statistics	vm_stat;
	natural_t		count = HOST_VM_INFO_COUNT;

	kret = host_statistics(host, HOST_VM_INFO,
					       (host_info_t)&vm_stat, &count);
#  endif
	if (kret == KERN_SUCCESS)
	    // get the amount of user memory by summing each usage
	    mem = (long_u)(vm_stat.free_count + vm_stat.active_count
					    + vm_stat.inactive_count
#  ifdef MAC_OS_X_VERSION_10_9
					    + vm_stat.compressor_page_count
#  endif
					    ) * sysconf(_SC_PAGESIZE);
	mach_port_deallocate(mach_task_self(), host);
    }
# endif

# ifdef HAVE_SYSCTL
    if (mem == 0)
    {
	// BSD way of getting the amount of RAM available.
	int		mib[2];
	size_t		len = sizeof(long_u);
#  ifdef HW_USERMEM64
	long_u		physmem;
#  else
	// sysctl() may return 32 bit or 64 bit, accept both
	union {
	    int_u	u32;
	    long_u	u64;
	} physmem;
#  endif

	mib[0] = CTL_HW;
#  ifdef HW_USERMEM64
	mib[1] = HW_USERMEM64;
#  else
	mib[1] = HW_USERMEM;
#  endif
	if (sysctl(mib, 2, &physmem, &len, NULL, 0) == 0)
	{
#  ifdef HW_USERMEM64
	    mem = (long_u)physmem;
#  else
	    if (len == sizeof(physmem.u64))
		mem = (long_u)physmem.u64;
	    else
		mem = (long_u)physmem.u32;
#  endif
	}
    }
# endif

# if defined(HAVE_SYS_SYSINFO_H) && defined(HAVE_SYSINFO)
    if (mem == 0)
    {
	struct sysinfo sinfo;

	// Linux way of getting amount of RAM available
	if (sysinfo(&sinfo) == 0)
	{
#  ifdef HAVE_SYSINFO_MEM_UNIT
	    // avoid overflow as much as possible
	    while (shiftright > 0 && (sinfo.mem_unit & 1) == 0)
	    {
		sinfo.mem_unit = sinfo.mem_unit >> 1;
		--shiftright;
	    }
	    mem = sinfo.totalram * sinfo.mem_unit;
#  else
	    mem = sinfo.totalram;
#  endif
	}
    }
# endif

# ifdef HAVE_SYSCONF
    if (mem == 0)
    {
	long	    pagesize, pagecount;

	// Solaris way of getting amount of RAM available
	pagesize = sysconf(_SC_PAGESIZE);
	pagecount = sysconf(_SC_PHYS_PAGES);
	if (pagesize > 0 && pagecount > 0)
	{
	    // avoid overflow as much as possible
	    while (shiftright > 0 && (pagesize & 1) == 0)
	    {
		pagesize = (long_u)pagesize >> 1;
		--shiftright;
	    }
	    mem = (long_u)pagesize * pagecount;
	}
    }
# endif

    // Return the minimum of the physical memory and the user limit, because
    // using more than the user limit may cause Vim to be terminated.
# if defined(HAVE_SYS_RESOURCE_H) && defined(HAVE_GETRLIMIT)
    {
	struct rlimit	rlp;

	if (getrlimit(RLIMIT_DATA, &rlp) == 0
		&& rlp.rlim_cur < ((rlim_t)1 << (sizeof(long_u) * 8 - 1))
#  ifdef RLIM_INFINITY
		&& rlp.rlim_cur != RLIM_INFINITY
#  endif
		&& ((long_u)rlp.rlim_cur >> 10) < (mem >> shiftright)
	   )
	{
	    mem = (long_u)rlp.rlim_cur;
	    shiftright = 10;
	}
    }
# endif

    if (mem > 0)
	return mem >> shiftright;
    return (long_u)0x1fffff;
}
#endif

/*
 * "flags": MCH_DELAY_IGNOREINPUT - don't read input
 *	    MCH_DELAY_SETTMODE - use settmode() even for short delays
 */
    void
mch_delay(long msec, int flags)
{
    tmode_T	old_tmode;
    int		call_settmode;
#ifdef FEAT_MZSCHEME
    long	total = msec; // remember original value
#endif

    if (flags & MCH_DELAY_IGNOREINPUT)
    {
	// Go to cooked mode without echo, to allow SIGINT interrupting us
	// here.  But we don't want QUIT to kill us (CTRL-\ used in a
	// shell may produce SIGQUIT).
	// Only do this if sleeping for more than half a second.
	in_mch_delay = TRUE;
	call_settmode = mch_cur_tmode == TMODE_RAW
			       && (msec > 500 || (flags & MCH_DELAY_SETTMODE));
	if (call_settmode)
	{
	    old_tmode = mch_cur_tmode;
	    settmode(TMODE_SLEEP);
	}

	/*
	 * Everybody sleeps in a different way...
	 * Prefer nanosleep(), some versions of usleep() can only sleep up to
	 * one second.
	 */
#ifdef FEAT_MZSCHEME
	do
	{
	    // if total is large enough, wait by portions in p_mzq
	    if (total > p_mzq)
		msec = p_mzq;
	    else
		msec = total;
	    total -= msec;
#endif
#ifdef HAVE_NANOSLEEP
	{
	    struct timespec ts;

	    ts.tv_sec = msec / 1000;
	    ts.tv_nsec = (msec % 1000) * 1000000;
	    (void)nanosleep(&ts, NULL);
	}
#else
# ifdef HAVE_USLEEP
	while (msec >= 1000)
	{
	    usleep((unsigned int)(999 * 1000));
	    msec -= 999;
	}
	usleep((unsigned int)(msec * 1000));
# else
#  ifndef HAVE_SELECT
	poll(NULL, 0, (int)msec);
#  else
	{
	    struct timeval tv;

	    tv.tv_sec = msec / 1000;
	    tv.tv_usec = (msec % 1000) * 1000;
	    // NOTE: Solaris 2.6 has a bug that makes select() hang here.  Get
	    // a patch from Sun to fix this.  Reported by Gunnar Pedersen.
	    select(0, NULL, NULL, NULL, &tv);
	}
#  endif
# endif
#endif
#ifdef FEAT_MZSCHEME
	}
	while (total > 0);
#endif

	if (call_settmode)
	    settmode(old_tmode);
	in_mch_delay = FALSE;
    }
    else
	WaitForChar(msec, NULL, FALSE);
}

#if defined(HAVE_STACK_LIMIT) \
	|| (!defined(HAVE_SIGALTSTACK) && defined(HAVE_SIGSTACK))
# define HAVE_CHECK_STACK_GROWTH
/*
 * Support for checking for an almost-out-of-stack-space situation.
 */

/*
 * Return a pointer to an item on the stack.  Used to find out if the stack
 * grows up or down.
 */
static int stack_grows_downwards;

/*
 * Find out if the stack grows upwards or downwards.
 * "p" points to a variable on the stack of the caller.
 */
    static void
check_stack_growth(char *p)
{
    int		i;

    stack_grows_downwards = (p > (char *)&i);
}
#endif

#if defined(HAVE_STACK_LIMIT) || defined(PROTO)
static char *stack_limit = NULL;

#if defined(_THREAD_SAFE) && defined(HAVE_PTHREAD_NP_H)
# include <pthread.h>
# include <pthread_np.h>
#endif

/*
 * Find out until how var the stack can grow without getting into trouble.
 * Called when starting up and when switching to the signal stack in
 * deathtrap().
 */
    static void
get_stack_limit(void)
{
    struct rlimit	rlp;
    int			i;
    long		lim;

    // Set the stack limit to 15/16 of the allowable size.  Skip this when the
    // limit doesn't fit in a long (rlim_cur might be "long long").
    if (getrlimit(RLIMIT_STACK, &rlp) == 0
	    && rlp.rlim_cur < ((rlim_t)1 << (sizeof(long_u) * 8 - 1))
#  ifdef RLIM_INFINITY
	    && rlp.rlim_cur != RLIM_INFINITY
#  endif
       )
    {
	lim = (long)rlp.rlim_cur;
#if defined(_THREAD_SAFE) && defined(HAVE_PTHREAD_NP_H)
	{
	    pthread_attr_t  attr;
	    size_t	    size;

	    // On FreeBSD the initial thread always has a fixed stack size, no
	    // matter what the limits are set to.  Normally it's 1 Mbyte.
	    pthread_attr_init(&attr);
	    if (pthread_attr_get_np(pthread_self(), &attr) == 0)
	    {
		pthread_attr_getstacksize(&attr, &size);
		if (lim > (long)size)
		    lim = (long)size;
	    }
	    pthread_attr_destroy(&attr);
	}
#endif
	if (stack_grows_downwards)
	{
	    stack_limit = (char *)((long)&i - (lim / 16L * 15L));
	    if (stack_limit >= (char *)&i)
		// overflow, set to 1/16 of current stack position
		stack_limit = (char *)((long)&i / 16L);
	}
	else
	{
	    stack_limit = (char *)((long)&i + (lim / 16L * 15L));
	    if (stack_limit <= (char *)&i)
		stack_limit = NULL;	// overflow
	}
    }
}

/*
 * Return FAIL when running out of stack space.
 * "p" must point to any variable local to the caller that's on the stack.
 */
    int
mch_stackcheck(char *p)
{
    if (stack_limit == NULL)
	return OK;

    if (stack_grows_downwards)
    {
	if (p < stack_limit)
	    return FAIL;
    }
    else if (p > stack_limit)
	return FAIL;
    return OK;
}
#endif

#if defined(HAVE_SIGALTSTACK) || defined(HAVE_SIGSTACK)
/*
 * Support for using the signal stack.
 * This helps when we run out of stack space, which causes a SIGSEGV.  The
 * signal handler then must run on another stack, since the normal stack is
 * completely full.
 */

# ifdef HAVE_SIGALTSTACK
static stack_t sigstk;			// for sigaltstack()
# else
static struct sigstack sigstk;		// for sigstack()
# endif

/*
 * Get a size of signal stack.
 * Preference (if available): sysconf > SIGSTKSZ > guessed size
 */
static long int get_signal_stack_size(void)
{
# ifdef HAVE_SYSCONF_SIGSTKSZ
    long int size = -1;

    // return size only if sysconf doesn't return an error
    if ((size = sysconf(_SC_SIGSTKSZ)) > -1)
	return size;
# endif

# ifdef SIGSTKSZ
    // if sysconf() isn't available or gives error, return SIGSTKSZ
    // if defined
    return SIGSTKSZ;
# endif

    // otherwise guess the size
    return 8000;
}

static char *signal_stack;

    static void
init_signal_stack(void)
{
    if (signal_stack == NULL)
	return;

# ifdef HAVE_SIGALTSTACK
#  ifdef HAVE_SS_BASE
    sigstk.ss_base = signal_stack;
#  else
    sigstk.ss_sp = signal_stack;
#  endif
    sigstk.ss_size = get_signal_stack_size();
    sigstk.ss_flags = 0;
    (void)sigaltstack(&sigstk, NULL);
# else
    sigstk.ss_sp = signal_stack;
    if (stack_grows_downwards)
	sigstk.ss_sp += get_signal_stack_size() - 1;
    sigstk.ss_onstack = 0;
    (void)sigstack(&sigstk, NULL);
# endif
}
#endif

/*
 * We need correct prototypes for a signal function, otherwise mean compilers
 * will barf when the second argument to signal() is ``wrong''.
 * Let me try it with a few tricky defines from my own osdef.h	(jw).
 */
#if defined(SIGWINCH)
    static void
sig_winch SIGDEFARG(sigarg)
{
    // this is not required on all systems, but it doesn't hurt anybody
    mch_signal(SIGWINCH, sig_winch);
    do_resize = TRUE;
}
#endif

#if defined(SIGTSTP)
    static void
sig_tstp SIGDEFARG(sigarg)
{
    // Second time we get called we actually need to suspend
    if (in_mch_suspend)
    {
	mch_signal(SIGTSTP, ignore_sigtstp ? SIG_IGN : SIG_DFL);
	raise(sigarg);
    }
    else
	got_tstp = TRUE;

#if !defined(__ANDROID__) && !defined(__OpenBSD__) && !defined(__DragonFly__)
    // This is not required on all systems.  On some systems (at least Android,
    // OpenBSD, and DragonFlyBSD) this breaks suspending with CTRL-Z.
    mch_signal(SIGTSTP, sig_tstp);
#endif
}
#endif

#if defined(SIGINT)
    static void
catch_sigint SIGDEFARG(sigarg)
{
    // this is not required on all systems, but it doesn't hurt anybody
    mch_signal(SIGINT, catch_sigint);
    got_int = TRUE;
}
#endif

#if defined(SIGUSR1)
    static void
catch_sigusr1 SIGDEFARG(sigarg)
{
    // this is not required on all systems, but it doesn't hurt anybody
    mch_signal(SIGUSR1, catch_sigusr1);
    got_sigusr1 = TRUE;
}
#endif

#if defined(SIGPWR)
    static void
catch_sigpwr SIGDEFARG(sigarg)
{
    // this is not required on all systems, but it doesn't hurt anybody
    mch_signal(SIGPWR, catch_sigpwr);
    /*
     * I'm not sure we get the SIGPWR signal when the system is really going
     * down or when the batteries are almost empty.  Just preserve the swap
     * files and don't exit, that can't do any harm.
     */
    ml_sync_all(FALSE, FALSE);
}
#endif

#ifdef SET_SIG_ALARM
/*
 * signal function for alarm().
 */
    static void
sig_alarm SIGDEFARG(sigarg)
{
    // doesn't do anything, just to break a system call
    sig_alarm_called = TRUE;
}
#endif

#if (defined(HAVE_SETJMP_H) \
	&& ((defined(FEAT_X11) && defined(FEAT_XCLIPBOARD)) \
	    || defined(FEAT_LIBCALL))) \
    || defined(PROTO)
# define USING_SETJMP 1

// argument to SETJMP()
static JMP_BUF lc_jump_env;

# ifdef SIGHASARG
// Caught signal number, 0 when no signal was caught; used for mch_libcall().
// Volatile because it is used in signal handlers.
static volatile sig_atomic_t lc_signal;
# endif

// TRUE when lc_jump_env is valid.
// Volatile because it is used in signal handler deathtrap().
static volatile sig_atomic_t lc_active = FALSE;

/*
 * A simplistic version of setjmp() that only allows one level of using.
 * Used to protect areas where we could crash.
 * Don't call twice before calling mch_endjmp()!.
 *
 * Usage:
 *	mch_startjmp();
 *	if (SETJMP(lc_jump_env) != 0)
 *	{
 *	    mch_didjmp();
 *	    emsg("crash!");
 *	}
 *	else
 *	{
 *	    do_the_work;
 *	    mch_endjmp();
 *	}
 * Note: Can't move SETJMP() here, because a function calling setjmp() must
 * not return before the saved environment is used.
 * Returns OK for normal return, FAIL when the protected code caused a
 * problem and LONGJMP() was used.
 */
    static void
mch_startjmp(void)
{
# ifdef SIGHASARG
    lc_signal = 0;
# endif
    lc_active = TRUE;
}

    static void
mch_endjmp(void)
{
    lc_active = FALSE;
}

    static void
mch_didjmp(void)
{
# if defined(HAVE_SIGALTSTACK) || defined(HAVE_SIGSTACK)
    // On FreeBSD the signal stack has to be reset after using siglongjmp(),
    // otherwise catching the signal only works once.
    init_signal_stack();
# endif
}
#endif

/*
 * This function handles deadly signals.
 * It tries to preserve any swap files and exit properly.
 * (partly from Elvis).
 * NOTE: Avoid unsafe functions, such as allocating memory, they can result in
 * a deadlock.
 */
    static void
deathtrap SIGDEFARG(sigarg)
{
    static int	entered = 0;	    // count the number of times we got here.
				    // Note: when memory has been corrupted
				    // this may get an arbitrary value!
#ifdef SIGHASARG
    int		i;
#endif

#if defined(USING_SETJMP)
    /*
     * Catch a crash in protected code.
     * Restores the environment saved in lc_jump_env, which looks like
     * SETJMP() returns 1.
     */
    if (lc_active)
    {
# if defined(SIGHASARG)
	lc_signal = sigarg;
# endif
	lc_active = FALSE;	// don't jump again
	LONGJMP(lc_jump_env, 1);
	// NOTREACHED
    }
#endif

#ifdef SIGHASARG
# ifdef SIGQUIT
    // While in mch_delay() we go to cooked mode to allow a CTRL-C to
    // interrupt us.  But in cooked mode we may also get SIGQUIT, e.g., when
    // pressing CTRL-\, but we don't want Vim to exit then.
    if (in_mch_delay && sigarg == SIGQUIT)
	return;
# endif

    // When SIGHUP, SIGQUIT, etc. are blocked: postpone the effect and return
    // here.  This avoids that a non-reentrant function is interrupted, e.g.,
    // free().  Calling free() again may then cause a crash.
    if (entered == 0
	    && (0
# ifdef SIGHUP
		|| sigarg == SIGHUP
# endif
# ifdef SIGQUIT
		|| sigarg == SIGQUIT
# endif
# ifdef SIGTERM
		|| sigarg == SIGTERM
# endif
# ifdef SIGPWR
		|| sigarg == SIGPWR
# endif
# ifdef SIGUSR1
		|| sigarg == SIGUSR1
# endif
# ifdef SIGUSR2
		|| sigarg == SIGUSR2
# endif
		)
	    && !vim_handle_signal(sigarg))
	return;
#endif

    // Remember how often we have been called.
    ++entered;

    // Executing autocommands is likely to use more stack space than we have
    // available in the signal stack.
    block_autocmds();

#ifdef FEAT_EVAL
    // Set the v:dying variable.
    set_vim_var_nr(VV_DYING, (long)entered);
#endif
    v_dying = entered;

#ifdef HAVE_STACK_LIMIT
    // Since we are now using the signal stack, need to reset the stack
    // limit.  Otherwise using a regexp will fail.
    get_stack_limit();
#endif

#if 0
    // This is for opening gdb the moment Vim crashes.
    // You need to manually adjust the file name and Vim executable name.
    // Suggested by SungHyun Nam.
    {
# define VI_GDB_FILE "/tmp/vimgdb"
# define VIM_NAME "/usr/bin/vim"
	FILE *fp = fopen(VI_GDB_FILE, "w");
	if (fp)
	{
	    fprintf(fp,
		    "file %s\n"
		    "attach %d\n"
		    "set height 1000\n"
		    "bt full\n"
		    , VIM_NAME, getpid());
	    fclose(fp);
	    system("xterm -e gdb -x "VI_GDB_FILE);
	    unlink(VI_GDB_FILE);
	}
    }
#endif

#ifdef SIGHASARG
    // try to find the name of this signal
    for (i = 0; signal_info[i].sig != -1; i++)
	if (sigarg == signal_info[i].sig)
	    break;
    deadly_signal = sigarg;
#endif

    full_screen = FALSE;	// don't write message to the GUI, it might be
				// part of the problem...
    /*
     * If something goes wrong after entering here, we may get here again.
     * When this happens, give a message and try to exit nicely (resetting the
     * terminal mode, etc.)
     * When this happens twice, just exit, don't even try to give a message,
     * stack may be corrupt or something weird.
     * When this still happens again (or memory was corrupted in such a way
     * that "entered" was clobbered) use _exit(), don't try freeing resources.
     */
    if (entered >= 3)
    {
	reset_signals();	// don't catch any signals anymore
	may_core_dump();
	if (entered >= 4)
	    _exit(8);
	exit(7);
    }
    if (entered == 2)
    {
	// No translation, it may call malloc().
	OUT_STR("Vim: Double signal, exiting\n");
	out_flush();
	getout(1);
    }

    // No translation, it may call malloc().
#ifdef SIGHASARG
    sprintf((char *)IObuff, "Vim: Caught deadly signal %s\r\n",
							 signal_info[i].name);
#else
    sprintf((char *)IObuff, "Vim: Caught deadly signal\r\n");
#endif

    // Preserve files and exit.  This sets the really_exiting flag to prevent
    // calling free().
    preserve_exit();

    // NOTREACHED

#ifdef NBDEBUG
    reset_signals();
    may_core_dump();
    abort();
#endif
}

/*
 * Invoked after receiving SIGCONT.  We don't know what happened while
 * sleeping, deal with part of that.
 */
    static void
after_sigcont(void)
{
    // Don't change "oldtitle" in a signal handler, set a flag to obtain it
    // again later.
    oldtitle_outdated = TRUE;

    settmode(TMODE_RAW);
    need_check_timestamps = TRUE;
    did_check_timestamps = FALSE;
}

#if defined(SIGCONT)
static void sigcont_handler SIGPROTOARG;

/*
 * With multi-threading, suspending might not work immediately.  Catch the
 * SIGCONT signal, which will be used as an indication whether the suspending
 * has been done or not.
 *
 * On Linux, signal is not always handled immediately either.
 * See https://bugs.launchpad.net/bugs/291373
 * Probably because the signal is handled in another thread.
 *
 * volatile because it is used in signal handler sigcont_handler().
 */
static volatile sig_atomic_t sigcont_received;
static void sigcont_handler SIGPROTOARG;

/*
 * signal handler for SIGCONT
 */
    static void
sigcont_handler SIGDEFARG(sigarg)
{
    if (in_mch_suspend)
    {
	sigcont_received = TRUE;
    }
    else
    {
	// We didn't suspend ourselves, assume we were stopped by a SIGSTOP
	// signal (which can't be intercepted) and get a SIGCONT.  Need to get
	// back to a sane mode. We should redraw, but we can't really do that
	// in a signal handler, do a redraw later.
	after_sigcont();
	redraw_later(UPD_CLEAR);
	cursor_on_force();
	out_flush();
    }
}
#endif

#if defined(FEAT_CLIPBOARD) && defined(FEAT_X11)
# ifdef USE_SYSTEM
static void *clip_star_save = NULL;
static void *clip_plus_save = NULL;
# endif

/*
 * Called when Vim is going to sleep or execute a shell command.
 * We can't respond to requests for the X selections.  Lose them, otherwise
 * other applications will hang.  But first copy the text to cut buffer 0.
 */
    static void
loose_clipboard(void)
{
    if (clip_star.owned || clip_plus.owned)
    {
	x11_export_final_selection();
	if (clip_star.owned)
	    clip_lose_selection(&clip_star);
	if (clip_plus.owned)
	    clip_lose_selection(&clip_plus);
	if (x11_display != NULL)
	    XFlush(x11_display);
    }
}

# ifdef USE_SYSTEM
/*
 * Save clipboard text to restore later.
 */
    static void
save_clipboard(void)
{
    if (clip_star.owned)
	clip_star_save = get_register('*', TRUE);
    if (clip_plus.owned)
	clip_plus_save = get_register('+', TRUE);
}

/*
 * Restore clipboard text if no one own the X selection.
 */
    static void
restore_clipboard(void)
{
    if (clip_star_save != NULL)
    {
	if (!clip_gen_owner_exists(&clip_star))
	    put_register('*', clip_star_save);
	else
	    free_register(clip_star_save);
	clip_star_save = NULL;
    }
    if (clip_plus_save != NULL)
    {
	if (!clip_gen_owner_exists(&clip_plus))
	    put_register('+', clip_plus_save);
	else
	    free_register(clip_plus_save);
	clip_plus_save = NULL;
    }
}
# endif
#endif

/*
 * If the machine has job control, use it to suspend the program,
 * otherwise fake it by starting a new shell.
 */
    void
mch_suspend(void)
{
    if (ignore_sigtstp)
	return;

#if defined(SIGTSTP)
    in_mch_suspend = TRUE;

    out_flush();	    // needed to make cursor visible on some systems
    settmode(TMODE_COOK);
    out_flush();	    // needed to disable mouse on some systems

# if defined(FEAT_CLIPBOARD) && defined(FEAT_X11)
    loose_clipboard();
# endif
# if defined(SIGCONT)
    sigcont_received = FALSE;
# endif

    kill(0, SIGTSTP);	    // send ourselves a STOP signal

# if defined(SIGCONT)
    /*
     * Wait for the SIGCONT signal to be handled. It generally happens
     * immediately, but somehow not all the time, probably because it's handled
     * in another thread. Do not call pause() because there would be race
     * condition which would hang Vim if signal happened in between the test of
     * sigcont_received and the call to pause(). If signal is not yet received,
     * sleep 0, 1, 2, 3 ms. Don't bother waiting further if signal is not
     * received after 1+2+3 ms (not expected to happen).
     */
    {
	long wait_time;

	for (wait_time = 0; !sigcont_received && wait_time <= 3L; wait_time++)
	    mch_delay(wait_time, 0);
    }
# endif
    in_mch_suspend = FALSE;

    after_sigcont();
#else
    suspend_shell();
#endif
}

    void
mch_init(void)
{
    Columns = 80;
    Rows = 24;

    out_flush();

#ifdef SIGTSTP
    // Check whether we were invoked with SIGTSTP set to be ignored. If it is
    // that indicates the shell (or program) that launched us does not support
    // tty job control and thus we should ignore that signal. If invoked as a
    // restricted editor (e.g., as "rvim") SIGTSTP is always ignored.
    ignore_sigtstp = restricted || SIG_IGN == mch_signal(SIGTSTP, SIG_ERR);
#endif
    set_signals();

#ifdef MACOS_CONVERT
    mac_conv_init();
#endif
#ifdef FEAT_CYGWIN_WIN32_CLIPBOARD
    win_clip_init();
#endif
}

    static void
set_signals(void)
{
#if defined(SIGWINCH)
    /*
     * WINDOW CHANGE signal is handled with sig_winch().
     */
    mch_signal(SIGWINCH, sig_winch);
#endif

#ifdef SIGTSTP
    // See mch_init() for the conditions under which we ignore SIGTSTP.
    // In the GUI default TSTP processing is OK.
    // Checking both gui.in_use and gui.starting because gui.in_use is not set
    // at this point (set after menus are displayed), but gui.starting is set.
    mch_signal(SIGTSTP, ignore_sigtstp ? SIG_IGN
# ifdef FEAT_GUI
				: gui.in_use || gui.starting ? SIG_DFL
# endif
				    : sig_tstp);
#endif
#if defined(SIGCONT)
    mch_signal(SIGCONT, sigcont_handler);
#endif
#ifdef SIGPIPE
    /*
     * We want to ignore breaking of PIPEs.
     */
    mch_signal(SIGPIPE, SIG_IGN);
#endif

#ifdef SIGINT
    catch_int_signal();
#endif

#ifdef SIGUSR1
    /*
     * Call user's handler on SIGUSR1
     */
    mch_signal(SIGUSR1, catch_sigusr1);
#endif

    /*
     * Ignore alarm signals (Perl's alarm() generates it).
     */
#ifdef SIGALRM
    mch_signal(SIGALRM, SIG_IGN);
#endif

#ifdef SIGPWR
    /*
     * Catch SIGPWR (power failure?) to preserve the swap files, so that no
     * work will be lost.
     */
    mch_signal(SIGPWR, catch_sigpwr);
#endif

    /*
     * Arrange for other signals to gracefully shutdown Vim.
     */
    catch_signals(deathtrap, SIG_ERR);

#if defined(FEAT_GUI) && defined(SIGHUP)
    /*
     * When the GUI is running, ignore the hangup signal.
     */
    if (gui.in_use)
	mch_signal(SIGHUP, SIG_IGN);
#endif
}

#if defined(SIGINT) || defined(PROTO)
/*
 * Catch CTRL-C (only works while in Cooked mode).
 */
    static void
catch_int_signal(void)
{
    mch_signal(SIGINT, catch_sigint);
}
#endif

    void
reset_signals(void)
{
    catch_signals(SIG_DFL, SIG_DFL);
#if defined(SIGCONT)
    // SIGCONT isn't in the list, because its default action is ignore
    mch_signal(SIGCONT, SIG_DFL);
#endif
}

    static void
catch_signals(
    void (*func_deadly)(int),
    void (*func_other)(int))
{
    int	    i;

    for (i = 0; signal_info[i].sig != -1; i++)
    {
	if (signal_info[i].deadly)
	{
#if defined(HAVE_SIGALTSTACK) && defined(HAVE_SIGACTION)
	    struct sigaction sa;

	    // Setup to use the alternate stack for the signal function.
	    sa.sa_handler = func_deadly;
	    sigemptyset(&sa.sa_mask);
# if defined(__linux__) && defined(_REENTRANT)
	    // On Linux, with glibc compiled for kernel 2.2, there is a bug in
	    // thread handling in combination with using the alternate stack:
	    // pthread library functions try to use the stack pointer to
	    // identify the current thread, causing a SEGV signal, which
	    // recursively calls deathtrap() and hangs.
	    sa.sa_flags = 0;
# else
	    sa.sa_flags = SA_ONSTACK;
# endif
	    sigaction(signal_info[i].sig, &sa, NULL);
#else
# if defined(HAVE_SIGALTSTACK) && defined(HAVE_SIGVEC)
	    struct sigvec sv;

	    // Setup to use the alternate stack for the signal function.
	    sv.sv_handler = func_deadly;
	    sv.sv_mask = 0;
	    sv.sv_flags = SV_ONSTACK;
	    sigvec(signal_info[i].sig, &sv, NULL);
# else
	    mch_signal(signal_info[i].sig, func_deadly);
# endif
#endif
	}
	else if (func_other != SIG_ERR)
	{
	    // Deal with non-deadly signals.
#ifdef SIGTSTP
	    mch_signal(signal_info[i].sig,
		    signal_info[i].sig == SIGTSTP && ignore_sigtstp
						       ? SIG_IGN : func_other);
#else
	    mch_signal(signal_info[i].sig, func_other);
#endif
	}
    }
}

#ifdef HAVE_SIGPROCMASK
    static void
block_signals(sigset_t *set)
{
    sigset_t	newset;
    int		i;

    sigemptyset(&newset);

    for (i = 0; signal_info[i].sig != -1; i++)
	sigaddset(&newset, signal_info[i].sig);

# if defined(SIGCONT)
    // SIGCONT isn't in the list, because its default action is ignore
    sigaddset(&newset, SIGCONT);
# endif

    sigprocmask(SIG_BLOCK, &newset, set);
}

    static void
unblock_signals(sigset_t *set)
{
    sigprocmask(SIG_SETMASK, set, NULL);
}
#endif

/*
 * Handling of SIGHUP, SIGQUIT and SIGTERM:
 * "when" == a signal:       when busy, postpone and return FALSE, otherwise
 *			     return TRUE
 * "when" == SIGNAL_BLOCK:   Going to be busy, block signals
 * "when" == SIGNAL_UNBLOCK: Going to wait, unblock signals, use postponed
 *			     signal
 * Returns TRUE when Vim should exit.
 */
    int
vim_handle_signal(int sig)
{
    static int got_signal = 0;
    static int blocked = TRUE;

    switch (sig)
    {
	case SIGNAL_BLOCK:   blocked = TRUE;
			     break;

	case SIGNAL_UNBLOCK: blocked = FALSE;
			     if (got_signal != 0)
			     {
				 kill(getpid(), got_signal);
				 got_signal = 0;
			     }
			     break;

	default:	     if (!blocked)
				 return TRUE;	// exit!
			     got_signal = sig;
#ifdef SIGPWR
			     if (sig != SIGPWR)
#endif
				 got_int = TRUE;    // break any loops
			     break;
    }
    return FALSE;
}

/*
 * Check_win checks whether we have an interactive stdout.
 */
    int
mch_check_win(int argc UNUSED, char **argv UNUSED)
{
    if (isatty(1))
	return OK;
    return FAIL;
}

/*
 * Return TRUE if the input comes from a terminal, FALSE otherwise.
 */
    int
mch_input_isatty(void)
{
    if (isatty(read_cmd_fd))
	return TRUE;
    return FALSE;
}

#ifdef FEAT_X11

# if defined(ELAPSED_TIMEVAL)

/*
 * Give a message about the elapsed time for opening the X window.
 */
    static void
xopen_message(long elapsed_msec)
{
    smsg(_("Opening the X display took %ld msec"), elapsed_msec);
}
# endif
#endif

#if defined(FEAT_X11)
/*
 * A few functions shared by X11 title and clipboard code.
 */

static int	got_x_error = FALSE;

/*
 * X Error handler, otherwise X just exits!  (very rude) -- webb
 */
    static int
x_error_handler(Display *dpy, XErrorEvent *error_event)
{
    XGetErrorText(dpy, error_event->error_code, (char *)IObuff, IOSIZE);
    STRCAT(IObuff, _("\nVim: Got X error\n"));

    // In the GUI we cannot print a message and continue, because no X calls
    // are allowed here (causes my system to hang).  Silently continuing seems
    // like the best alternative.  Do preserve files, in case we crash.
    ml_sync_all(FALSE, FALSE);

#ifdef FEAT_GUI
    if (!gui.in_use)
#endif
	msg((char *)IObuff);

    return 0;		// NOTREACHED
}

/*
 * Another X Error handler, just used to check for errors.
 */
    static int
x_error_check(Display *dpy UNUSED, XErrorEvent *error_event UNUSED)
{
    got_x_error = TRUE;
    return 0;
}

/*
 * Return TRUE when connection to the X server is desired.
 */
    static int
x_connect_to_server(void)
{
    // No point in connecting if we are exiting or dying.
    if (exiting || v_dying)
	return FALSE;

#if defined(FEAT_CLIENTSERVER)
    if (x_force_connect)
	return TRUE;
#endif
    if (x_no_connect)
	return FALSE;

    // Check for a match with "exclude:" from 'clipboard'.
    if (clip_exclude_prog != NULL)
    {
	// Just in case we get called recursively, return FALSE.  This could
	// happen if vpeekc() is used while executing the prog and it causes a
	// related callback to be invoked.
	if (regprog_in_use(clip_exclude_prog))
	    return FALSE;

	if (vim_regexec_prog(&clip_exclude_prog, FALSE, T_NAME, (colnr_T)0))
	    return FALSE;
    }
    return TRUE;
}

#if defined(FEAT_X11) && defined(FEAT_XCLIPBOARD)
# if defined(USING_SETJMP)
/*
 * An X IO Error handler, used to catch error while opening the display.
 */
    static int
x_IOerror_check(Display *dpy UNUSED)
{
    // This function should not return, it causes exit().  Longjump instead.
    LONGJMP(lc_jump_env, 1);
#  if defined(VMS) || defined(__CYGWIN__)
    return 0;  // avoid the compiler complains about missing return value
#  endif
}
# endif

/*
 * An X IO Error handler, used to catch terminal errors.
 */
static int xterm_dpy_retry_count = 0;

    static int
x_IOerror_handler(Display *dpy UNUSED)
{
    xterm_dpy = NULL;
    xterm_dpy_retry_count = 5;  // Try reconnecting five times
    x11_window = 0;
    x11_display = NULL;
    xterm_Shell = (Widget)0;

    // This function should not return, it causes exit().  Longjump instead.
    LONGJMP(x_jump_env, 1);
# if defined(VMS) || defined(__CYGWIN__)
    return 0;  // avoid the compiler complains about missing return value
# endif
}

/*
 * If the X11 connection was lost try to restore it.
 * Helps when the X11 server was stopped and restarted while Vim was inactive
 * (e.g. through tmux).
 */
    static void
may_restore_clipboard(void)
{
    // No point in restoring the connecting if we are exiting or dying.
    if (!exiting && !v_dying && xterm_dpy_retry_count > 0)
    {
	--xterm_dpy_retry_count;

# ifndef LESSTIF_VERSION
	// This has been reported to avoid Vim getting stuck.
	if (app_context != (XtAppContext)NULL)
	{
	    XtDestroyApplicationContext(app_context);
	    app_context = (XtAppContext)NULL;
	    x11_display = NULL; // freed by XtDestroyApplicationContext()
	}
# endif

	setup_term_clip();
	get_x11_title(FALSE);
    }
}

    void
ex_xrestore(exarg_T *eap)
{
    if (eap->arg != NULL && STRLEN(eap->arg) > 0)
    {
	if (xterm_display_allocated)
	    vim_free(xterm_display);
	xterm_display = (char *)vim_strsave(eap->arg);
	xterm_display_allocated = TRUE;
    }
    smsg(_("restoring display %s"), xterm_display == NULL
		    ? (char *)mch_getenv((char_u *)"DISPLAY") : xterm_display);

    clear_xterm_clip();
    x11_window = 0;
    xterm_dpy_retry_count = 5;  // Try reconnecting five times
    may_restore_clipboard();
}
#endif

/*
 * Test if "dpy" and x11_window are valid by getting the window title.
 * I don't actually want it yet, so there may be a simpler call to use, but
 * this will cause the error handler x_error_check() to be called if anything
 * is wrong, such as the window pointer being invalid (as can happen when the
 * user changes his DISPLAY, but not his WINDOWID) -- webb
 */
    static int
test_x11_window(Display *dpy)
{
    int			(*old_handler)(Display*, XErrorEvent*);
    XTextProperty	text_prop;

    old_handler = XSetErrorHandler(x_error_check);
    got_x_error = FALSE;
    if (XGetWMName(dpy, x11_window, &text_prop))
	XFree((void *)text_prop.value);
    XSync(dpy, False);
    (void)XSetErrorHandler(old_handler);

    if (p_verbose > 0 && got_x_error)
	verb_msg(_("Testing the X display failed"));

    return (got_x_error ? FAIL : OK);
}
#endif


#ifdef FEAT_X11

static int get_x11_thing(int get_title, int test_only);

/*
 * try to get x11 window and display
 *
 * return FAIL for failure, OK otherwise
 */
    static int
get_x11_windis(void)
{
    char	    *winid;
    static int	    result = -1;
#define XD_NONE	 0	// x11_display not set here
#define XD_HERE	 1	// x11_display opened here
#define XD_GUI	 2	// x11_display used from gui.dpy
#define XD_XTERM 3	// x11_display used from xterm_dpy
    static int	    x11_display_from = XD_NONE;
    static int	    did_set_error_handler = FALSE;

    if (!did_set_error_handler)
    {
	// X just exits if it finds an error otherwise!
	(void)XSetErrorHandler(x_error_handler);
	did_set_error_handler = TRUE;
    }

#if defined(FEAT_GUI_X11) || defined(FEAT_GUI_GTK)
    if (gui.in_use)
    {
	/*
	 * If the X11 display was opened here before, for the window where Vim
	 * was started, close that one now to avoid a memory leak.
	 */
	if (x11_display_from == XD_HERE && x11_display != NULL)
	{
	    XCloseDisplay(x11_display);
	    x11_display_from = XD_NONE;
	}
	if (gui_get_x11_windis(&x11_window, &x11_display) == OK)
	{
	    x11_display_from = XD_GUI;
	    return OK;
	}
	x11_display = NULL;
	return FAIL;
    }
    else if (x11_display_from == XD_GUI)
    {
	// GUI must have stopped somehow, clear x11_display
	x11_window = 0;
	x11_display = NULL;
	x11_display_from = XD_NONE;
    }
#endif

    // When started with the "-X" argument, don't try connecting.
    if (!x_connect_to_server())
	return FAIL;

    /*
     * If WINDOWID not set, should try another method to find out
     * what the current window number is. The only code I know for
     * this is very complicated.
     * We assume that zero is invalid for WINDOWID.
     */
    if (x11_window == 0 && (winid = getenv("WINDOWID")) != NULL)
	x11_window = (Window)atol(winid);

#ifdef FEAT_XCLIPBOARD
    if (xterm_dpy == x11_display)
	// x11_display may have been set to xterm_dpy elsewhere
	x11_display_from = XD_XTERM;

    if (xterm_dpy != NULL && x11_window != 0)
    {
	// We may have checked it already, but Gnome terminal can move us to
	// another window, so we need to check every time.
	if (x11_display_from != XD_XTERM)
	{
	    /*
	     * If the X11 display was opened here before, for the window where
	     * Vim was started, close that one now to avoid a memory leak.
	     */
	    if (x11_display_from == XD_HERE && x11_display != NULL)
		XCloseDisplay(x11_display);
	    x11_display = xterm_dpy;
	    x11_display_from = XD_XTERM;
	}
	if (test_x11_window(x11_display) == FAIL)
	{
	    // probably bad $WINDOWID
	    x11_window = 0;
	    x11_display = NULL;
	    x11_display_from = XD_NONE;
	    return FAIL;
	}
	return OK;
    }
#endif

    if (x11_window == 0 || x11_display == NULL)
	result = -1;

    if (result != -1)	    // Have already been here and set this
	return result;	    // Don't do all these X calls again

    if (x11_window != 0 && x11_display == NULL)
    {
#ifdef SET_SIG_ALARM
	sighandler_T sig_save;
#endif
#ifdef ELAPSED_FUNC
	elapsed_T start_tv;

	if (p_verbose > 0)
	    ELAPSED_INIT(start_tv);
#endif

#ifdef SET_SIG_ALARM
	/*
	 * Opening the Display may hang if the DISPLAY setting is wrong, or
	 * the network connection is bad.  Set an alarm timer to get out.
	 */
	sig_alarm_called = FALSE;
	sig_save = mch_signal(SIGALRM, sig_alarm);
	alarm(2);
#endif
	x11_display = XOpenDisplay(NULL);

#ifdef SET_SIG_ALARM
	alarm(0);
	mch_signal(SIGALRM, sig_save);
	if (p_verbose > 0 && sig_alarm_called)
	    verb_msg(_("Opening the X display timed out"));
#endif
	if (x11_display != NULL)
	{
# ifdef ELAPSED_FUNC
	    if (p_verbose > 0)
	    {
		verbose_enter();
		xopen_message(ELAPSED_FUNC(start_tv));
		verbose_leave();
	    }
# endif
	    if (test_x11_window(x11_display) == FAIL)
	    {
		// Maybe window id is bad
		x11_window = 0;
		XCloseDisplay(x11_display);
		x11_display = NULL;
	    }
	    else
		x11_display_from = XD_HERE;
	}
    }
    if (x11_window == 0 || x11_display == NULL)
	return (result = FAIL);

# ifdef FEAT_EVAL
    set_vim_var_nr(VV_WINDOWID, (long)x11_window);
# endif

    return (result = OK);
}

/*
 * Determine original x11 Window Title
 */
    static int
get_x11_title(int test_only)
{
    return get_x11_thing(TRUE, test_only);
}

/*
 * Determine original x11 Window icon
 */
    static int
get_x11_icon(int test_only)
{
    int		retval = FALSE;

    retval = get_x11_thing(FALSE, test_only);

    // could not get old icon, use terminal name
    if (oldicon == NULL && !test_only)
    {
	if (STRNCMP(T_NAME, "builtin_", 8) == 0)
	    oldicon = vim_strsave(T_NAME + 8);
	else
	    oldicon = vim_strsave(T_NAME);
    }

    return retval;
}

    static int
get_x11_thing(
    int		get_title,	// get title string
    int		test_only)
{
    XTextProperty	text_prop;
    int			retval = FALSE;
    Status		status;

    if (get_x11_windis() != OK)
	return FALSE;

    // Get window/icon name if any
    if (get_title)
	status = XGetWMName(x11_display, x11_window, &text_prop);
    else
	status = XGetWMIconName(x11_display, x11_window, &text_prop);

    /*
     * If terminal is xterm, then x11_window may be a child window of the
     * outer xterm window that actually contains the window/icon name, so
     * keep traversing up the tree until a window with a title/icon is
     * found.
     */
    // Previously this was only done for xterm and alike.  I don't see a
    // reason why it would fail for other terminal emulators.
    // if (term_is_xterm)
    Window	    root;
    Window	    parent;
    Window	    win = x11_window;
    Window	   *children;
    unsigned int    num_children;

    while (!status || text_prop.value == NULL)
    {
	if (!XQueryTree(x11_display, win, &root, &parent, &children,
		    &num_children))
	    break;
	if (children)
	    XFree((void *)children);
	if (parent == root || parent == 0)
	    break;

	win = parent;
	if (get_title)
	    status = XGetWMName(x11_display, win, &text_prop);
	else
	    status = XGetWMIconName(x11_display, win, &text_prop);
    }

    if (status && text_prop.value != NULL)
    {
	retval = TRUE;
	if (!test_only)
	{
	    if (get_title)
		vim_free(oldtitle);
	    else
		vim_free(oldicon);
	    if (text_prop.encoding == XA_STRING && !has_mbyte)
	    {
		if (get_title)
		    oldtitle = vim_strsave((char_u *)text_prop.value);
		else
		    oldicon = vim_strsave((char_u *)text_prop.value);
	    }
	    else
	    {
		char    **cl;
		Status  transform_status;
		int	    n = 0;

		transform_status = XmbTextPropertyToTextList(x11_display,
			&text_prop,
			&cl, &n);
		if (transform_status >= Success && n > 0 && cl[0])
		{
		    if (get_title)
			oldtitle = vim_strsave((char_u *) cl[0]);
		    else
			oldicon = vim_strsave((char_u *) cl[0]);
		    XFreeStringList(cl);
		}
		else
		{
		    if (get_title)
			oldtitle = vim_strsave((char_u *)text_prop.value);
		    else
			oldicon = vim_strsave((char_u *)text_prop.value);
		}
	    }
	}
	XFree((void *)text_prop.value);
    }
    return retval;
}

// Xutf8 functions are not available on older systems. Note that on some
// systems X_HAVE_UTF8_STRING may be defined in a header file but
// Xutf8SetWMProperties() is not in the X11 library.  Configure checks for
// that and defines HAVE_XUTF8SETWMPROPERTIES.
#if defined(X_HAVE_UTF8_STRING)
# if X_HAVE_UTF8_STRING && HAVE_XUTF8SETWMPROPERTIES
#  define USE_UTF8_STRING
# endif
#endif

/*
 * Set x11 Window Title
 *
 * get_x11_windis() must be called before this and have returned OK
 */
    static void
set_x11_title(char_u *title)
{
	// XmbSetWMProperties() and Xutf8SetWMProperties() should use a STRING
	// when possible, COMPOUND_TEXT otherwise.  COMPOUND_TEXT isn't
	// supported everywhere and STRING doesn't work for multi-byte titles.
#ifdef USE_UTF8_STRING
    if (enc_utf8)
	Xutf8SetWMProperties(x11_display, x11_window, (const char *)title,
					     NULL, NULL, 0, NULL, NULL, NULL);
    else
#endif
    {
#if XtSpecificationRelease >= 4
# ifdef FEAT_XFONTSET
	XmbSetWMProperties(x11_display, x11_window, (const char *)title,
					     NULL, NULL, 0, NULL, NULL, NULL);
# else
	XTextProperty	text_prop;
	char		*c_title = (char *)title;

	// directly from example 3-18 "basicwin" of Xlib Programming Manual
	(void)XStringListToTextProperty(&c_title, 1, &text_prop);
	XSetWMProperties(x11_display, x11_window, &text_prop,
					     NULL, NULL, 0, NULL, NULL, NULL);
# endif
#else
	XStoreName(x11_display, x11_window, (char *)title);
#endif
    }
    XFlush(x11_display);
}

/*
 * Set x11 Window icon
 *
 * get_x11_windis() must be called before this and have returned OK
 */
    static void
set_x11_icon(char_u *icon)
{
    // See above for comments about using X*SetWMProperties().
#ifdef USE_UTF8_STRING
    if (enc_utf8)
	Xutf8SetWMProperties(x11_display, x11_window, NULL, (const char *)icon,
						   NULL, 0, NULL, NULL, NULL);
    else
#endif
    {
#if XtSpecificationRelease >= 4
# ifdef FEAT_XFONTSET
	XmbSetWMProperties(x11_display, x11_window, NULL, (const char *)icon,
						   NULL, 0, NULL, NULL, NULL);
# else
	XTextProperty	text_prop;
	char		*c_icon = (char *)icon;

	(void)XStringListToTextProperty(&c_icon, 1, &text_prop);
	XSetWMProperties(x11_display, x11_window, NULL, &text_prop,
						   NULL, 0, NULL, NULL, NULL);
# endif
#else
	XSetIconName(x11_display, x11_window, (char *)icon);
#endif
    }
    XFlush(x11_display);
}

#else  // FEAT_X11

    static int
get_x11_title(int test_only UNUSED)
{
    return FALSE;
}

    static int
get_x11_icon(int test_only)
{
    if (!test_only)
    {
	if (STRNCMP(T_NAME, "builtin_", 8) == 0)
	    oldicon = vim_strsave(T_NAME + 8);
	else
	    oldicon = vim_strsave(T_NAME);
    }
    return FALSE;
}

#endif // FEAT_X11

    int
mch_can_restore_title(void)
{
    return get_x11_title(TRUE);
}

    int
mch_can_restore_icon(void)
{
    return get_x11_icon(TRUE);
}

/*
 * Set the window title and icon.
 */
    void
mch_settitle(char_u *title, char_u *icon)
{
    int		type = 0;
    static int	recursive = 0;

    if (T_NAME == NULL)	    // no terminal name (yet)
	return;
    if (title == NULL && icon == NULL)	    // nothing to do
	return;

    // When one of the X11 functions causes a deadly signal, we get here again
    // recursively.  Avoid hanging then (something is probably locked).
    if (recursive)
	return;
    ++recursive;

    /*
     * if the window ID and the display is known, we may use X11 calls
     */
#ifdef FEAT_X11
    if (get_x11_windis() == OK)
	type = 1;
#endif
#if defined(FEAT_GUI_PHOTON) \
    || defined(FEAT_GUI_GTK) || defined(FEAT_GUI_HAIKU)
    if (gui.in_use)
	type = 1;
#endif

    /*
     * Note: if "t_ts" is set, title is set with escape sequence rather
     *	     than x11 calls, because the x11 calls don't always work
     */
    if ((type || *T_TS != NUL) && title != NULL)
    {
	if (oldtitle_outdated)
	{
	    oldtitle_outdated = FALSE;
	    VIM_CLEAR(oldtitle);
	}
	if (oldtitle == NULL
#ifdef FEAT_GUI
		&& !gui.in_use
#endif
		)		// first call but not in GUI, save title
	    (void)get_x11_title(FALSE);

	if (*T_TS != NUL)		// it's OK if t_fs is empty
	    term_settitle(title);
#ifdef FEAT_X11
	else
# ifdef FEAT_GUI_GTK
	if (!gui.in_use)		// don't do this if GTK+ is running
# endif
	    set_x11_title(title);		// x11
#endif
#if defined(FEAT_GUI_GTK) || defined(FEAT_GUI_HAIKU) \
	|| defined(FEAT_GUI_PHOTON)
	else
	    gui_mch_settitle(title, icon);
#endif
	unix_did_set_title = TRUE;
    }

    if ((type || *T_CIS != NUL) && icon != NULL)
    {
	if (oldicon == NULL
#ifdef FEAT_GUI
		&& !gui.in_use
#endif
		)		// first call, save icon
	    get_x11_icon(FALSE);

	if (*T_CIS != NUL)
	{
	    out_str(T_CIS);			// set icon start
	    out_str_nf(icon);
	    out_str(T_CIE);			// set icon end
	    out_flush();
	}
#ifdef FEAT_X11
	else
# ifdef FEAT_GUI_GTK
	if (!gui.in_use)		// don't do this if GTK+ is running
# endif
	    set_x11_icon(icon);			// x11
#endif
	did_set_icon = TRUE;
    }
    --recursive;
}

/*
 * Restore the window/icon title.
 * "which" is one of:
 *  SAVE_RESTORE_TITLE only restore title
 *  SAVE_RESTORE_ICON  only restore icon
 *  SAVE_RESTORE_BOTH  restore title and icon
 */
    void
mch_restore_title(int which)
{
    int	do_push_pop = unix_did_set_title || did_set_icon;

    // Only restore the title or icon when it has been set.
    // When using "oldtitle" make a copy, it might be freed halfway.
    char_u *title = ((which & SAVE_RESTORE_TITLE) && unix_did_set_title)
			? (oldtitle ? oldtitle : p_titleold) : NULL;
    char_u *tofree = NULL;
    if (title == oldtitle && oldtitle != NULL)
    {
	tofree = vim_strsave(title);
	if (tofree != NULL)
	    title = tofree;
    }
    mch_settitle(title,
	       ((which & SAVE_RESTORE_ICON) && did_set_icon) ? oldicon : NULL);
    vim_free(tofree);

    if (do_push_pop)
    {
	// pop and push from/to the stack
	term_pop_title(which);
	term_push_title(which);
    }
}


/*
 * Return TRUE if "name" looks like some xterm name.
 * This matches "xterm.*", thus "xterm-256color", "xterm-kitty", etc.
 * Do not consider "xterm-kitty" an xterm, it is not fully xterm compatible,
 * using the "xterm-kitty" terminfo entry should work better.
 * Seiichi Sato mentioned that "mlterm" works like xterm.
 */
    int
vim_is_xterm(char_u *name)
{
    if (name == NULL)
	return FALSE;
    return ((STRNICMP(name, "xterm", 5) == 0
				     && STRNICMP(name, "xterm-kitty", 11) != 0)
		|| STRNICMP(name, "nxterm", 6) == 0
		|| STRNICMP(name, "kterm", 5) == 0
		|| STRNICMP(name, "mlterm", 6) == 0
		|| STRNICMP(name, "rxvt", 4) == 0
		|| STRNICMP(name, "screen.xterm", 12) == 0
		|| STRCMP(name, "builtin_xterm") == 0);
}

#if defined(FEAT_MOUSE_XTERM) || defined(PROTO)
/*
 * Return TRUE if "name" appears to be that of a terminal
 * known to support the xterm-style mouse protocol.
 * Relies on term_is_xterm having been set to its correct value.
 */
    int
use_xterm_like_mouse(char_u *name)
{
    return (name != NULL
	    && (term_is_xterm
		|| STRNICMP(name, "screen", 6) == 0
		|| STRNICMP(name, "tmux", 4) == 0
		|| STRNICMP(name, "gnome", 5) == 0
		|| STRICMP(name, "st") == 0
		|| STRNICMP(name, "st-", 3) == 0
		|| STRNICMP(name, "stterm", 6) == 0));
}
#endif

/*
 * Return non-zero when using an xterm mouse, according to 'ttymouse'.
 * Return 1 for "xterm".
 * Return 2 for "xterm2".
 * Return 3 for "urxvt".
 * Return 4 for "sgr".
 */
    int
use_xterm_mouse(void)
{
    if (ttym_flags == TTYM_SGR)
	return 4;
    if (ttym_flags == TTYM_URXVT)
	return 3;
    if (ttym_flags == TTYM_XTERM2)
	return 2;
    if (ttym_flags == TTYM_XTERM)
	return 1;
    return 0;
}

/*
 * Return TRUE if "name" is an iris-ansi terminal name.
 */
    int
vim_is_iris(char_u *name)
{
    if (name == NULL)
	return FALSE;
    return (STRNICMP(name, "iris-ansi", 9) == 0
	    || STRCMP(name, "builtin_iris-ansi") == 0);
}

#if defined(VMS) || defined(PROTO)
/*
 * Return TRUE if "name" is a vt300-like terminal name.
 */
    int
vim_is_vt300(char_u *name)
{
    if (name == NULL)
	return FALSE;
    // Actually all ANSI compatible terminals should be here.
    // Catch at least VT1xx - VT5xx
    return ((STRNICMP(name, "vt", 2) == 0
			     && vim_strchr((char_u *)"12345", name[2]) != NULL)
	    || STRCMP(name, "builtin_vt320") == 0);
}
#endif

/*
 * Insert user name in s[len].
 * Return OK if a name found.
 */
    int
mch_get_user_name(char_u *s, int len)
{
#ifdef VMS
    vim_strncpy(s, (char_u *)cuserid(NULL), len - 1);
    return OK;
#else
    return mch_get_uname(getuid(), s, len);
#endif
}

/*
 * Insert user name for "uid" in s[len].
 * Return OK if a name found.
 */
    int
mch_get_uname(uid_t uid, char_u *s, int len)
{
#if defined(HAVE_PWD_H) && defined(HAVE_GETPWUID)
    struct passwd   *pw;

    if ((pw = getpwuid(uid)) != NULL
	    && pw->pw_name != NULL && *(pw->pw_name) != NUL)
    {
	vim_strncpy(s, (char_u *)pw->pw_name, len - 1);
	return OK;
    }
#endif
    sprintf((char *)s, "%d", (int)uid);	    // assumes s is long enough
    return FAIL;			    // a number is not a name
}

/*
 * Insert host name is s[len].
 */

#ifdef HAVE_SYS_UTSNAME_H
    void
mch_get_host_name(char_u *s, int len)
{
    struct utsname vutsname;

    if (uname(&vutsname) < 0)
	*s = NUL;
    else
	vim_strncpy(s, (char_u *)vutsname.nodename, len - 1);
}
#else // HAVE_SYS_UTSNAME_H

# ifdef HAVE_SYS_SYSTEMINFO_H
#  define gethostname(nam, len) sysinfo(SI_HOSTNAME, nam, len)
# endif

    void
mch_get_host_name(char_u *s, int len)
{
# ifdef VAXC
    vaxc$gethostname((char *)s, len);
# else
    gethostname((char *)s, len);
# endif
    s[len - 1] = NUL;	// make sure it's terminated
}
#endif // HAVE_SYS_UTSNAME_H

/*
 * return process ID
 */
    long
mch_get_pid(void)
{
    return (long)getpid();
}

/*
 * return TRUE if process "pid" is still running
 */
    int
mch_process_running(long pid)
{
    // If there is no error the process must be running.
    if (kill(pid, 0) == 0)
	return TRUE;
#ifdef ESRCH
    // If the error is ESRCH then the process is not running.
    if (errno == ESRCH)
	return FALSE;
#endif
    // If the process is running and owned by another user we get EPERM.  With
    // other errors the process might be running, assuming it is then.
    return TRUE;
}

#if !defined(HAVE_STRERROR) && defined(USE_GETCWD)
    static char *
strerror(int err)
{
    extern int	    sys_nerr;
    extern char	    *sys_errlist[];
    static char	    er[20];

    if (err > 0 && err < sys_nerr)
	return (sys_errlist[err]);
    sprintf(er, "Error %d", err);
    return er;
}
#endif

/*
 * Get name of current directory into buffer "buf" of length "len" bytes.
 * "len" must be at least PATH_MAX.
 * Return OK for success, FAIL for failure.
 */
    int
mch_dirname(char_u *buf, int len)
{
#if defined(USE_GETCWD)
    if (getcwd((char *)buf, len) == NULL)
    {
	STRCPY(buf, strerror(errno));
	return FAIL;
    }
    return OK;
#else
    return (getwd((char *)buf) != NULL ? OK : FAIL);
#endif
}

/*
 * Get absolute file name into "buf[len]".
 *
 * return FAIL for failure, OK for success
 */
    int
mch_FullName(
    char_u	*fname,
    char_u	*buf,
    int		len,
    int		force)		// also expand when already absolute path
{
    int		l;
#ifdef HAVE_FCHDIR
    int		fd = -1;
    static int	dont_fchdir = FALSE;	// TRUE when fchdir() doesn't work
#endif
    char_u	olddir[MAXPATHL];
    char_u	*p;
    int		retval = OK;
#ifdef __CYGWIN__
    char_u	posix_fname[MAXPATHL];	// Cygwin docs mention MAX_PATH, but
					// it's not always defined
#endif

#ifdef VMS
    fname = vms_fixfilename(fname);
#endif

#ifdef __CYGWIN__
    /*
     * This helps for when "/etc/hosts" is a symlink to "c:/something/hosts".
     */
# if CYGWIN_VERSION_DLL_MAJOR >= 1007
    // Use CCP_RELATIVE to avoid that it sometimes returns a path that ends in
    // a forward slash.
    cygwin_conv_path(CCP_WIN_A_TO_POSIX | CCP_RELATIVE,
		     fname, posix_fname, MAXPATHL);
# else
    cygwin_conv_to_posix_path(fname, posix_fname);
# endif
    fname = posix_fname;
#endif

    // Expand it if forced or not an absolute path.
    // Do not do it for "/file", the result is always "/".
    if ((force || !mch_isFullName(fname))
	    && ((p = vim_strrchr(fname, '/')) == NULL || p != fname))
    {
	/*
	 * If the file name has a path, change to that directory for a moment,
	 * and then get the directory (and get back to where we were).
	 * This will get the correct path name with "../" things.
	 */
	if (p != NULL)
	{
	    if (STRCMP(p, "/..") == 0)
		// for "/path/dir/.." include the "/.."
		p += 3;

#ifdef HAVE_FCHDIR
	    /*
	     * Use fchdir() if possible, it's said to be faster and more
	     * reliable.  But on SunOS 4 it might not work.  Check this by
	     * doing a fchdir() right now.
	     */
	    if (!dont_fchdir)
	    {
		fd = open(".", O_RDONLY | O_EXTRA, 0);
		if (fd >= 0 && fchdir(fd) < 0)
		{
		    close(fd);
		    fd = -1;
		    dont_fchdir = TRUE;	    // don't try again
		}
	    }
#endif

	    // Only change directory when we are sure we can return to where
	    // we are now.  After doing "su" chdir(".") might not work.
	    if (
#ifdef HAVE_FCHDIR
		fd < 0 &&
#endif
			(mch_dirname(olddir, MAXPATHL) == FAIL
					   || mch_chdir((char *)olddir) != 0))
	    {
		p = NULL;	// can't get current dir: don't chdir
		retval = FAIL;
	    }
	    else
	    {
		// The directory is copied into buf[], to be able to remove
		// the file name without changing it (could be a string in
		// read-only memory)
		if (p - fname >= len)
		    retval = FAIL;
		else
		{
		    vim_strncpy(buf, fname, p - fname);
		    if (mch_chdir((char *)buf))
		    {
			// Path does not exist (yet).  For a full path fail,
			// will use the path as-is.  For a relative path use
			// the current directory and append the file name.
			if (mch_isFullName(fname))
			    retval = FAIL;
			else
			    p = NULL;
		    }
		    else if (*p == '/')
			fname = p + 1;
		    else
			fname = p;
		    *buf = NUL;
		}
	    }
	}
	if (mch_dirname(buf, len) == FAIL)
	{
	    retval = FAIL;
	    *buf = NUL;
	}
	if (p != NULL)
	{
#ifdef HAVE_FCHDIR
	    if (fd >= 0)
	    {
		if (p_verbose >= 5)
		{
		    verbose_enter();
		    msg("fchdir() to previous dir");
		    verbose_leave();
		}
		l = fchdir(fd);
	    }
	    else
#endif
		l = mch_chdir((char *)olddir);
	    if (l != 0)
		emsg(_(e_cannot_go_back_to_previous_directory));
	}
#ifdef HAVE_FCHDIR
	if (fd >= 0)
	    close(fd);
#endif

	l = STRLEN(buf);
	if (l >= len - 1)
	    retval = FAIL; // no space for trailing "/"
#ifndef VMS
	else if (l > 0 && buf[l - 1] != '/' && *fname != NUL
						   && STRCMP(fname, ".") != 0)
	    STRCAT(buf, "/");
#endif
    }

    // Catch file names which are too long.
    if (retval == FAIL || (int)(STRLEN(buf) + STRLEN(fname)) >= len)
	return FAIL;

    // Do not append ".", "/dir/." is equal to "/dir".
    if (STRCMP(fname, ".") != 0)
	STRCAT(buf, fname);

    return OK;
}

/*
 * Return TRUE if "fname" does not depend on the current directory.
 */
    int
mch_isFullName(char_u *fname)
{
#ifdef VMS
    return ( fname[0] == '/'	       || fname[0] == '.'	    ||
	     strchr((char *)fname,':') || strchr((char *)fname,'"') ||
	    (strchr((char *)fname,'[') && strchr((char *)fname,']'))||
	    (strchr((char *)fname,'<') && strchr((char *)fname,'>'))   );
#else
    return (*fname == '/' || *fname == '~');
#endif
}

#if defined(USE_FNAME_CASE) || defined(PROTO)
/*
 * Set the case of the file name, if it already exists.  This will cause the
 * file name to remain exactly the same.
 * Only required for file systems where case is ignored and preserved.
 */
    void
fname_case(
    char_u	*name,
    int		len UNUSED)  // buffer size, only used when name gets longer
{
    struct stat st;
    char_u	*slash, *tail;
    DIR		*dirp;
    struct dirent *dp;

    if (mch_lstat((char *)name, &st) < 0)
	return;

    // Open the directory where the file is located.
    slash = vim_strrchr(name, '/');
    if (slash == NULL)
    {
	dirp = opendir(".");
	tail = name;
    }
    else
    {
	*slash = NUL;
	dirp = opendir((char *)name);
	*slash = '/';
	tail = slash + 1;
    }

    if (dirp == NULL)
	return;

    while ((dp = readdir(dirp)) != NULL)
    {
	// Only accept names that differ in case and are the same byte
	// length. TODO: accept different length name.
	if (STRICMP(tail, dp->d_name) == 0
		&& STRLEN(tail) == STRLEN(dp->d_name))
	{
	    char_u	newname[MAXPATHL + 1];
	    struct stat st2;

	    // Verify the inode is equal.
	    vim_strncpy(newname, name, MAXPATHL);
	    vim_strncpy(newname + (tail - name), (char_u *)dp->d_name,
		    MAXPATHL - (tail - name));
	    if (mch_lstat((char *)newname, &st2) >= 0
		    && st.st_ino == st2.st_ino
		    && st.st_dev == st2.st_dev)
	    {
		STRCPY(tail, dp->d_name);
		break;
	    }
	}
    }

    closedir(dirp);
}
#endif

/*
 * Get file permissions for 'name'.
 * Returns -1 when it doesn't exist.
 */
    long
mch_getperm(char_u *name)
{
    struct stat statb;

    // Keep the #ifdef outside of stat(), it may be a macro.
#ifdef VMS
    if (stat((char *)vms_fixfilename(name), &statb))
#else
    if (stat((char *)name, &statb))
#endif
	return -1;
#ifdef __INTERIX
    // The top bit makes the value negative, which means the file doesn't
    // exist.  Remove the bit, we don't use it.
    return statb.st_mode & ~S_ADDACE;
#else
    return statb.st_mode;
#endif
}

/*
 * Set file permission for "name" to "perm".
 * Return FAIL for failure, OK otherwise.
 */
    int
mch_setperm(char_u *name, long perm)
{
    return (chmod((char *)
#ifdef VMS
		    vms_fixfilename(name),
#else
		    name,
#endif
		    (mode_t)perm) == 0 ? OK : FAIL);
}

#if defined(HAVE_FCHMOD) || defined(PROTO)
/*
 * Set file permission for open file "fd" to "perm".
 * Return FAIL for failure, OK otherwise.
 */
    int
mch_fsetperm(int fd, long perm)
{
    return (fchmod(fd, (mode_t)perm) == 0 ? OK : FAIL);
}
#endif

#if defined(HAVE_ACL) || defined(PROTO)
# ifdef HAVE_SYS_ACL_H
#  include <sys/acl.h>
# endif
# ifdef HAVE_SYS_ACCESS_H
#  include <sys/access.h>
# endif

# ifdef HAVE_SOLARIS_ACL
typedef struct vim_acl_solaris_T {
    int acl_cnt;
    aclent_t *acl_entry;
} vim_acl_solaris_T;
# endif

#if defined(HAVE_SELINUX) || defined(PROTO)
/*
 * Copy security info from "from_file" to "to_file".
 */
    void
mch_copy_sec(char_u *from_file, char_u *to_file)
{
    if (from_file == NULL)
	return;

    if (selinux_enabled == -1)
	selinux_enabled = is_selinux_enabled();

    if (selinux_enabled <= 0)
	return;

    // Use "char *" instead of "security_context_t" to avoid a deprecation
    // warning.
    char *from_context = NULL;
    char *to_context = NULL;

    if (getfilecon((char *)from_file, &from_context) < 0)
    {
	// If the filesystem doesn't support extended attributes,
	// the original had no special security context and the
	// target cannot have one either.
	if (errno == EOPNOTSUPP)
	    return;

	msg_puts(_("\nCould not get security context for "));
	msg_outtrans(from_file);
	msg_putchar('\n');
	return;
    }
    if (getfilecon((char *)to_file, &to_context) < 0)
    {
	msg_puts(_("\nCould not get security context for "));
	msg_outtrans(to_file);
	msg_putchar('\n');
	freecon (from_context);
	return ;
    }
    if (strcmp(from_context, to_context) != 0)
    {
	if (setfilecon((char *)to_file, from_context) < 0)
	{
	    msg_puts(_("\nCould not set security context for "));
	    msg_outtrans(to_file);
	    msg_putchar('\n');
	}
    }
    freecon(to_context);
    freecon(from_context);
}
#endif // HAVE_SELINUX

#if defined(HAVE_SMACK) && !defined(PROTO)
/*
 * Copy security info from "from_file" to "to_file".
 */
    void
mch_copy_sec(char_u *from_file, char_u *to_file)
{
    static const char * const smack_copied_attributes[] =
	{
	    XATTR_NAME_SMACK,
	    XATTR_NAME_SMACKEXEC,
	    XATTR_NAME_SMACKMMAP
	};

    char	buffer[SMACK_LABEL_LEN];
    const char	*name;
    int		index;
    int		ret;
    ssize_t	size;

    if (from_file == NULL)
	return;

    size = listxattr((char *)from_file, NULL, 0);
    // not supported or no attributes to copy
    if (errno == ENOTSUP || size == 0)
	return;

    for (index = 0 ; index < (int)(sizeof(smack_copied_attributes)
			      / sizeof(smack_copied_attributes)[0]) ; index++)
    {
	// get the name of the attribute to copy
	name = smack_copied_attributes[index];

	// get the value of the attribute in buffer
	size = getxattr((char*)from_file, name, buffer, sizeof(buffer));
	if (size >= 0)
	{
	    // copy the attribute value of buffer
	    ret = setxattr((char*)to_file, name, buffer, (size_t)size, 0);
	    if (ret < 0)
	    {
		vim_snprintf((char *)IObuff, IOSIZE,
			_("Could not set security context %s for %s"),
			name, to_file);
		msg_outtrans(IObuff);
		msg_putchar('\n');
	    }
	}
	else
	{
	    // what reason of not having the attribute value?
	    switch (errno)
	    {
		case ENOTSUP:
		    // extended attributes aren't supported or enabled
		    // should a message be echoed? not sure...
		    return; // leave because it isn't useful to continue

		case ERANGE:
		default:
		    // no enough size OR unexpected error
		     vim_snprintf((char *)IObuff, IOSIZE,
			    _("Could not get security context %s for %s. Removing it!"),
			    name, from_file);
		    msg_puts((char *)IObuff);
		    msg_putchar('\n');
		    // FALLTHROUGH to remove the attribute

		case ENODATA:
		    // no attribute of this name
		    ret = removexattr((char*)to_file, name);
		    // Silently ignore errors, apparently this happens when
		    // smack is not actually being used.
		    break;
	    }
	}
    }
}
#endif // HAVE_SMACK

#ifdef FEAT_XATTR
/*
 * Copy extended attributes from_file to to_file
 */
    void
mch_copy_xattr(char_u *from_file, char_u *to_file)
{
    char	*xattr_buf;
    size_t	size;
    size_t	tsize;
    ssize_t	keylen, vallen, max_vallen = 0;
    char	*key;
    char	*val = NULL;
    char	*errmsg = NULL;

    if (from_file == NULL)
	return;

    // get the length of the extended attributes
    size = listxattr((char *)from_file, NULL, 0);
    // not supported or no attributes to copy
    if (errno == ENOTSUP || size == 0)
	return;
    xattr_buf = (char*)alloc(size);
    if (xattr_buf == NULL)
	return;
    size = listxattr((char *)from_file, xattr_buf, size);
    tsize = size;

    errno = 0;

    for (int round = 0; round < 2; round++)
    {

	key = xattr_buf;
	if (round == 1)
	    size = tsize;

	while (size > 0)
	{
	    vallen = getxattr((char *)from_file, key,
		    val, round ? max_vallen : 0);
	    // only set the attribute in the second round
	    if (vallen >= 0 && round &&
		setxattr((char *)to_file, key, val, vallen, 0) == 0)
		;
	    else if (errno)
	    {
		switch (errno)
		{
		    case E2BIG:
			errmsg = e_xattr_e2big;
			goto error_exit;
		    case ENOTSUP:
		    case EACCES:
		    case EPERM:
			break;
		    case ERANGE:
			errmsg = e_xattr_erange;
			goto error_exit;
		    default:
			errmsg = e_xattr_other;
			goto error_exit;
		}
	    }

	    if (round == 0 && vallen > max_vallen)
		max_vallen = vallen;

	    // add one for terminating null
	    keylen = STRLEN(key) + 1;
	    size -= keylen;
	    key += keylen;
	}
	if (round)
	    break;

	val = (char*)alloc(max_vallen + 1);
	if (val == NULL)
	    goto error_exit;

    }
error_exit:
    vim_free(xattr_buf);
    vim_free(val);

    if (errmsg != NULL)
	emsg(_(errmsg));
}
#endif

/*
 * Return a pointer to the ACL of file "fname" in allocated memory.
 * Return NULL if the ACL is not available for whatever reason.
 */
    vim_acl_T
mch_get_acl(char_u *fname UNUSED)
{
    vim_acl_T	ret = NULL;
#ifdef HAVE_POSIX_ACL
    ret = (vim_acl_T)acl_get_file((char *)fname, ACL_TYPE_ACCESS);
#else
#ifdef HAVE_SOLARIS_ZFS_ACL
    acl_t *aclent;

    if (acl_get((char *)fname, 0, &aclent) < 0)
	return NULL;
    ret = (vim_acl_T)aclent;
#else
#ifdef HAVE_SOLARIS_ACL
    vim_acl_solaris_T   *aclent;

    aclent = malloc(sizeof(vim_acl_solaris_T));
    if ((aclent->acl_cnt = acl((char *)fname, GETACLCNT, 0, NULL)) < 0)
    {
	free(aclent);
	return NULL;
    }
    aclent->acl_entry = malloc(aclent->acl_cnt * sizeof(aclent_t));
    if (acl((char *)fname, GETACL, aclent->acl_cnt, aclent->acl_entry) < 0)
    {
	free(aclent->acl_entry);
	free(aclent);
	return NULL;
    }
    ret = (vim_acl_T)aclent;
#else
#if defined(HAVE_AIX_ACL)
    int		aclsize;
    struct acl *aclent;

    aclsize = sizeof(struct acl);
    aclent = malloc(aclsize);
    if (statacl((char *)fname, STX_NORMAL, aclent, aclsize) < 0)
    {
	if (errno == ENOSPC)
	{
	    aclsize = aclent->acl_len;
	    aclent = realloc(aclent, aclsize);
	    if (statacl((char *)fname, STX_NORMAL, aclent, aclsize) < 0)
	    {
		free(aclent);
		return NULL;
	    }
	}
	else
	{
	    free(aclent);
	    return NULL;
	}
    }
    ret = (vim_acl_T)aclent;
#endif // HAVE_AIX_ACL
#endif // HAVE_SOLARIS_ACL
#endif // HAVE_SOLARIS_ZFS_ACL
#endif // HAVE_POSIX_ACL
    return ret;
}

/*
 * Set the ACL of file "fname" to "acl" (unless it's NULL).
 */
    void
mch_set_acl(char_u *fname UNUSED, vim_acl_T aclent)
{
    if (aclent == NULL)
	return;
#ifdef HAVE_POSIX_ACL
    acl_set_file((char *)fname, ACL_TYPE_ACCESS, (acl_t)aclent);
#else
#ifdef HAVE_SOLARIS_ZFS_ACL
    acl_set((char *)fname, (acl_t *)aclent);
#else
#ifdef HAVE_SOLARIS_ACL
    acl((char *)fname, SETACL, ((vim_acl_solaris_T *)aclent)->acl_cnt,
	    ((vim_acl_solaris_T *)aclent)->acl_entry);
#else
#ifdef HAVE_AIX_ACL
    chacl((char *)fname, aclent, ((struct acl *)aclent)->acl_len);
#endif // HAVE_AIX_ACL
#endif // HAVE_SOLARIS_ACL
#endif // HAVE_SOLARIS_ZFS_ACL
#endif // HAVE_POSIX_ACL
}

    void
mch_free_acl(vim_acl_T aclent)
{
    if (aclent == NULL)
	return;
#ifdef HAVE_POSIX_ACL
    acl_free((acl_t)aclent);
#else
#ifdef HAVE_SOLARIS_ZFS_ACL
    acl_free((acl_t *)aclent);
#else
#ifdef HAVE_SOLARIS_ACL
    free(((vim_acl_solaris_T *)aclent)->acl_entry);
    free(aclent);
#else
#ifdef HAVE_AIX_ACL
    free(aclent);
#endif // HAVE_AIX_ACL
#endif // HAVE_SOLARIS_ACL
#endif // HAVE_SOLARIS_ZFS_ACL
#endif // HAVE_POSIX_ACL
}
#endif

/*
 * Set hidden flag for "name".
 */
    void
mch_hide(char_u *name UNUSED)
{
    // can't hide a file
}

/*
 * return TRUE if "name" is a directory or a symlink to a directory
 * return FALSE if "name" is not a directory
 * return FALSE for error
 */
    int
mch_isdir(char_u *name)
{
    struct stat statb;

    if (*name == NUL)	    // Some stat()s don't flag "" as an error.
	return FALSE;
    if (stat((char *)name, &statb))
	return FALSE;
    return (S_ISDIR(statb.st_mode) ? TRUE : FALSE);
}

/*
 * return TRUE if "name" is a directory, NOT a symlink to a directory
 * return FALSE if "name" is not a directory
 * return FALSE for error
 */
    int
mch_isrealdir(char_u *name)
{
    struct stat statb;

    if (*name == NUL)	    // Some stat()s don't flag "" as an error.
	return FALSE;
    if (mch_lstat((char *)name, &statb))
	return FALSE;
    return (S_ISDIR(statb.st_mode) ? TRUE : FALSE);
}

/*
 * Return 1 if "name" is an executable file, 0 if not or it doesn't exist.
 */
    static int
executable_file(char_u *name)
{
    struct stat	st;

    if (stat((char *)name, &st))
	return 0;
#ifdef VMS
    // Like on Unix system file can have executable rights but not necessarily
    // be an executable, but on Unix is not a default for an ordinary file to
    // have an executable flag - on VMS it is in most cases.
    // Therefore, this check does not have any sense - let keep us to the
    // conventions instead:
    // *.COM and *.EXE files are the executables - the rest are not. This is
    // not ideal but better than it was.
    int vms_executable = 0;
    if (S_ISREG(st.st_mode) && mch_access((char *)name, X_OK) == 0)
    {
	if (strstr(vms_tolower((char*)name),".exe") != NULL
		|| strstr(vms_tolower((char*)name),".com")!= NULL)
	    vms_executable = 1;
    }
    return vms_executable;
#else
    return S_ISREG(st.st_mode) && mch_access((char *)name, X_OK) == 0;
#endif
}

/*
 * Return TRUE if "name" can be found in $PATH and executed, FALSE if not.
 * If "use_path" is FALSE only check if "name" is executable.
 * Return -1 if unknown.
 */
    int
mch_can_exe(char_u *name, char_u **path, int use_path)
{
    char_u	*buf;
    char_u	*p, *e;
    int		retval;

    // When "use_path" is false and if it's an absolute or relative path don't
    // need to use $PATH.
    if (!use_path || gettail(name) != name)
    {
	// There must be a path separator, files in the current directory
	// can't be executed.
	if ((use_path || gettail(name) != name) && executable_file(name))
	{
	    if (path != NULL)
	    {
		if (name[0] != '/')
		    *path = FullName_save(name, TRUE);
		else
		    *path = vim_strsave(name);
	    }
	    return TRUE;
	}
	return FALSE;
    }

    p = (char_u *)getenv("PATH");
    if (p == NULL || *p == NUL)
	return -1;
    buf = alloc(STRLEN(name) + STRLEN(p) + 2);
    if (buf == NULL)
	return -1;

    /*
     * Walk through all entries in $PATH to check if "name" exists there and
     * is an executable file.
     */
    for (;;)
    {
	e = (char_u *)strchr((char *)p, ':');
	if (e == NULL)
	    e = p + STRLEN(p);
	if (e - p <= 1)		// empty entry means current dir
	    STRCPY(buf, "./");
	else
	{
	    vim_strncpy(buf, p, e - p);
	    add_pathsep(buf);
	}
	STRCAT(buf, name);
	retval = executable_file(buf);
	if (retval == 1)
	{
	    if (path != NULL)
	    {
		if (buf[0] != '/')
		    *path = FullName_save(buf, TRUE);
		else
		    *path = vim_strsave(buf);
	    }
	    break;
	}

	if (*e != ':')
	    break;
	p = e + 1;
    }

    vim_free(buf);
    return retval;
}

/*
 * Check what "name" is:
 * NODE_NORMAL: file or directory (or doesn't exist)
 * NODE_WRITABLE: writable device, socket, fifo, etc.
 * NODE_OTHER: non-writable things
 */
    int
mch_nodetype(char_u *name)
{
    struct stat	st;

    if (stat((char *)name, &st))
	return NODE_NORMAL;
    if (S_ISREG(st.st_mode) || S_ISDIR(st.st_mode))
	return NODE_NORMAL;
    if (S_ISBLK(st.st_mode))	// block device isn't writable
	return NODE_OTHER;
    // Everything else is writable?
    return NODE_WRITABLE;
}

    void
mch_early_init(void)
{
#ifdef HAVE_CHECK_STACK_GROWTH
    int			i;

    check_stack_growth((char *)&i);

# ifdef HAVE_STACK_LIMIT
    get_stack_limit();
# endif

#endif

    /*
     * Setup an alternative stack for signals.  Helps to catch signals when
     * running out of stack space.
     * Use of sigaltstack() is preferred, it's more portable.
     * Ignore any errors.
     */
#if defined(HAVE_SIGALTSTACK) || defined(HAVE_SIGSTACK)
    signal_stack = alloc(get_signal_stack_size());
    init_signal_stack();
#endif
}

#if defined(EXITFREE) || defined(PROTO)
    void
mch_free_mem(void)
{
# if defined(FEAT_CLIPBOARD) && defined(FEAT_X11)
    if (clip_star.owned)
	clip_lose_selection(&clip_star);
    if (clip_plus.owned)
	clip_lose_selection(&clip_plus);
# endif
# if defined(FEAT_X11) && defined(FEAT_XCLIPBOARD)
    if (xterm_Shell != (Widget)0)
	XtDestroyWidget(xterm_Shell);
#  ifndef LESSTIF_VERSION
    // Lesstif crashes here, lose some memory
    if (xterm_dpy != NULL)
	XtCloseDisplay(xterm_dpy);
    if (app_context != (XtAppContext)NULL)
    {
	XtDestroyApplicationContext(app_context);
#   ifdef FEAT_X11
	x11_display = NULL; // freed by XtDestroyApplicationContext()
#   endif
    }
#  endif
# endif
# if defined(FEAT_X11)
    if (x11_display != NULL
#  ifdef FEAT_XCLIPBOARD
	    && x11_display != xterm_dpy
#  endif
	    )
	XCloseDisplay(x11_display);
# endif
# if defined(HAVE_SIGALTSTACK) || defined(HAVE_SIGSTACK)
    VIM_CLEAR(signal_stack);
# endif
    vim_free(oldtitle);
    vim_free(oldicon);
}
#endif

/*
 * Output a newline when exiting.
 * Make sure the newline goes to the same stream as the text.
 */
    static void
exit_scroll(void)
{
    if (silent_mode)
	return;
    if (newline_on_exit || msg_didout)
    {
	if (msg_use_printf())
	{
	    if (info_message)
		mch_msg("\n");
	    else
		mch_errmsg("\r\n");
	}
	else
	    out_char('\n');
    }
    else if (!is_not_a_term())
    {
	restore_cterm_colors();		// get original colors back
	msg_clr_eos_force();		// clear the rest of the display
	windgoto((int)Rows - 1, 0);	// may have moved the cursor
    }
}

#ifdef USE_GCOV_FLUSH
# if (defined(__GNUC__) \
	    && ((__GNUC__ == 11 && __GNUC_MINOR__ >= 1) || (__GNUC__ >= 12))) \
	|| (defined(__clang__) && (__clang_major__ >= 12))
extern void __gcov_dump(void);
extern void __gcov_reset(void);
#  define __gcov_flush() do { __gcov_dump(); __gcov_reset(); } while (0)
# else
extern void __gcov_flush(void);
# endif
#endif

    void
mch_exit(int r)
{
    exiting = TRUE;

#if defined(FEAT_X11) && defined(FEAT_CLIPBOARD)
    x11_export_final_selection();
#endif

#ifdef FEAT_GUI
    if (!gui.in_use)
#endif
    {
	settmode(TMODE_COOK);
	if (!is_not_a_term())
	{
	    // restore xterm title and icon name
	    mch_restore_title(SAVE_RESTORE_BOTH);
	    term_pop_title(SAVE_RESTORE_BOTH);
	}

	/*
	 * When t_ti is not empty but it doesn't cause swapping terminal
	 * pages, need to output a newline when msg_didout is set.  But when
	 * t_ti does swap pages it should not go to the shell page.  Do this
	 * before stoptermcap().
	 */
	if (swapping_screen() && !newline_on_exit)
	    exit_scroll();

	// Stop termcap: May need to check for T_CRV response, which
	// requires RAW mode.
	stoptermcap();

	/*
	 * A newline is only required after a message in the alternate screen.
	 * This is set to TRUE by wait_return().
	 */
	if (!swapping_screen() || newline_on_exit)
	    exit_scroll();

	// Cursor may have been switched off without calling starttermcap()
	// when doing "vim -u vimrc" and vimrc contains ":q".
	if (full_screen)
	    cursor_on();
    }
    out_flush();
    ml_close_all(TRUE);		// remove all memfiles

#ifdef USE_GCOV_FLUSH
    // Flush coverage info before possibly being killed by a deadly signal.
    __gcov_flush();
#endif

    may_core_dump();
#ifdef FEAT_GUI
    if (gui.in_use)
	gui_exit(r);
#endif

#ifdef MACOS_CONVERT
    mac_conv_cleanup();
#endif

#ifdef __QNX__
    // A core dump won't be created if the signal handler
    // doesn't return, so we can't call exit()
    if (deadly_signal != 0)
	return;
#endif

#ifdef FEAT_NETBEANS_INTG
    netbeans_send_disconnect();
#endif

#ifdef EXITFREE
    free_all_mem();
#endif

    exit(r);
}

    static void
may_core_dump(void)
{
    if (deadly_signal != 0)
    {
	mch_signal(deadly_signal, SIG_DFL);
	kill(getpid(), deadly_signal);	// Die using the signal we caught
    }
}

#ifndef VMS

/*
 * Get the file descriptor to use for tty operations.
 */
    static int
get_tty_fd(int fd)
{
    int		tty_fd = fd;

#if defined(HAVE_SVR4_PTYS) && defined(SUN_SYSTEM)
    // On SunOS: Get the terminal parameters from "fd", or the slave device of
    // "fd" when it is a master device.
    if (mch_isatty(fd) > 1)
    {
	char *name;

	name = ptsname(fd);
	if (name == NULL)
	    return -1;

	tty_fd = open(name, O_RDONLY | O_NOCTTY | O_EXTRA, 0);
	if (tty_fd < 0)
	    return -1;
    }
#endif
    return tty_fd;
}

    static int
mch_tcgetattr(int fd, void *term)
{
    int		tty_fd;
    int		retval = -1;

    tty_fd = get_tty_fd(fd);
    if (tty_fd < 0)
	return -1;

#ifdef NEW_TTY_SYSTEM
# ifdef HAVE_TERMIOS_H
    retval = tcgetattr(tty_fd, (struct termios *)term);
# else
    retval = ioctl(tty_fd, TCGETA, (struct termio *)term);
# endif
#else
    // for "old" tty systems
    retval = ioctl(tty_fd, TIOCGETP, (struct sgttyb *)term);
#endif
    if (tty_fd != fd)
	close(tty_fd);
    return retval;
}

    void
mch_settmode(tmode_T tmode)
{
    static int first = TRUE;

#ifdef NEW_TTY_SYSTEM
# ifdef HAVE_TERMIOS_H
    static struct termios told;
	   struct termios tnew;
# else
    static struct termio told;
	   struct termio tnew;
# endif

    if (first)
    {
	first = FALSE;
	mch_tcgetattr(read_cmd_fd, &told);
    }

    tnew = told;
    if (tmode == TMODE_RAW)
    {
	// ~ICRNL enables typing ^V^M
	// ~IXON disables CTRL-S stopping output, so that it can be mapped.
	tnew.c_iflag &= ~(ICRNL | IXON);
	tnew.c_lflag &= ~(ICANON | ECHO | ISIG | ECHOE
# if defined(IEXTEN)
		    | IEXTEN	    // IEXTEN enables typing ^V on SOLARIS
# endif
				);
# ifdef ONLCR
	// Don't map NL -> CR NL, we do it ourselves.
	// Also disable expanding tabs if possible.
#  ifdef XTABS
	tnew.c_oflag &= ~(ONLCR | XTABS);
#  else
#   ifdef TAB3
	tnew.c_oflag &= ~(ONLCR | TAB3);
#   else
	tnew.c_oflag &= ~ONLCR;
#   endif
#  endif
# endif
	tnew.c_cc[VMIN] = 1;		// return after 1 char
	tnew.c_cc[VTIME] = 0;		// don't wait
    }
    else if (tmode == TMODE_SLEEP)
    {
	// Also reset ICANON here, otherwise on Solaris select() won't see
	// typeahead characters.
	tnew.c_lflag &= ~(ICANON | ECHO);
	tnew.c_cc[VMIN] = 1;		// return after 1 char
	tnew.c_cc[VTIME] = 0;		// don't wait
    }

# if defined(HAVE_TERMIOS_H)
    {
	int	n = 10;

	// A signal may cause tcsetattr() to fail (e.g., SIGCONT).  Retry a
	// few times.
	while (tcsetattr(read_cmd_fd, TCSANOW, &tnew) == -1
						   && errno == EINTR && n > 0)
	    --n;
    }
# else
    ioctl(read_cmd_fd, TCSETA, &tnew);
# endif

#else
    /*
     * for "old" tty systems
     */
# ifndef TIOCSETN
#  define TIOCSETN TIOCSETP	// for hpux 9.0
# endif
    static struct sgttyb ttybold;
	   struct sgttyb ttybnew;

    if (first)
    {
	first = FALSE;
	mch_tcgetattr(read_cmd_fd, &ttybold);
    }

    ttybnew = ttybold;
    if (tmode == TMODE_RAW)
    {
	ttybnew.sg_flags &= ~(CRMOD | ECHO);
	ttybnew.sg_flags |= RAW;
    }
    else if (tmode == TMODE_SLEEP)
	ttybnew.sg_flags &= ~(ECHO);
    ioctl(read_cmd_fd, TIOCSETN, &ttybnew);
#endif
    mch_cur_tmode = tmode;
}

/*
 * Try to get the code for "t_kb" from the stty setting
 *
 * Even if termcap claims a backspace key, the user's setting *should*
 * prevail.  stty knows more about reality than termcap does, and if
 * somebody's usual erase key is DEL (which, for most BSD users, it will
 * be), they're going to get really annoyed if their erase key starts
 * doing forward deletes for no reason. (Eric Fischer)
 */
    void
get_stty(void)
{
    ttyinfo_T	info;
    char_u	buf[2];
    char_u	*p;

    if (get_tty_info(read_cmd_fd, &info) != OK)
	return;

    intr_char = info.interrupt;
    buf[0] = info.backspace;
    buf[1] = NUL;
    add_termcode((char_u *)"kb", buf, FALSE);

    // If <BS> and <DEL> are now the same, redefine <DEL>.
    p = find_termcode((char_u *)"kD");
    if (p != NULL && p[0] == buf[0] && p[1] == buf[1])
	do_fixdel(NULL);
}

/*
 * Obtain the characters that Backspace and Enter produce on "fd".
 * Returns OK or FAIL.
 */
    int
get_tty_info(int fd, ttyinfo_T *info)
{
#ifdef NEW_TTY_SYSTEM
# ifdef HAVE_TERMIOS_H
    struct termios keys;
# else
    struct termio keys;
# endif

    if (mch_tcgetattr(fd, &keys) != -1)
    {
	info->backspace = keys.c_cc[VERASE];
	info->interrupt = keys.c_cc[VINTR];
	if (keys.c_iflag & ICRNL)
	    info->enter = NL;
	else
	    info->enter = CAR;
	if (keys.c_oflag & ONLCR)
	    info->nl_does_cr = TRUE;
	else
	    info->nl_does_cr = FALSE;
	return OK;
    }
#else
    // for "old" tty systems
    struct sgttyb keys;

    if (mch_tcgetattr(fd, &keys) != -1)
    {
	info->backspace = keys.sg_erase;
	info->interrupt = keys.sg_kill;
	info->enter = CAR;
	info->nl_does_cr = TRUE;
	return OK;
    }
#endif
    return FAIL;
}

#endif // VMS

static int	mouse_ison = FALSE;

/*
 * Set mouse clicks on or off and possible enable mouse movement events.
 */
    void
mch_setmouse(int on)
{
#ifdef FEAT_BEVAL_TERM
    static int	bevalterm_ison = FALSE;
#endif
    int		xterm_mouse_vers;

#if defined(FEAT_X11) && defined(FEAT_XCLIPBOARD)
    if (!on)
	// Make sure not tracing mouse movements.  Important when a button-down
	// was received but no release yet.
	stop_xterm_trace();
#endif

    if (on == mouse_ison
#ifdef FEAT_BEVAL_TERM
	    && p_bevalterm == bevalterm_ison
#endif
	    )
	// return quickly if nothing to do
	return;

    xterm_mouse_vers = use_xterm_mouse();

#ifdef FEAT_MOUSE_URXVT
    if (ttym_flags == TTYM_URXVT)
    {
	out_str_nf((char_u *)(on ? "\033[?1015h" : "\033[?1015l"));
	mouse_ison = on;
    }
#endif

    if (T_CXM != NULL && *T_CXM != NUL)
    {
	term_enable_mouse(on);
    }
    else if (ttym_flags == TTYM_SGR)
    {
	// SGR mode supports columns above 223
	out_str_nf((char_u *)(on ? "\033[?1006h" : "\033[?1006l"));
	mouse_ison = on;
    }

#ifdef FEAT_BEVAL_TERM
    if (bevalterm_ison != (p_bevalterm && on))
    {
	bevalterm_ison = (p_bevalterm && on);
	if (xterm_mouse_vers > 1 && !bevalterm_ison)
	    // disable mouse movement events, enabling is below
	    out_str_nf((char_u *)("\033[?1003l"));
    }
#endif

    if (xterm_mouse_vers > 0)
    {
	if (on)	// enable mouse events, use mouse tracking if available
	    out_str_nf((char_u *)
		       (xterm_mouse_vers > 1
			? (
#ifdef FEAT_BEVAL_TERM
			    bevalterm_ison ? "\033[?1003h" :
#endif
			      "\033[?1002h")
			: "\033[?1000h"));
	else	// disable mouse events, could probably always send the same
	    out_str_nf((char_u *)
		       (xterm_mouse_vers > 1 ? "\033[?1002l" : "\033[?1000l"));
	mouse_ison = on;
    }

#ifdef FEAT_MOUSE_DEC
    else if (ttym_flags == TTYM_DEC)
    {
	if (on)	// enable mouse events
	    out_str_nf((char_u *)"\033[1;2'z\033[1;3'{");
	else	// disable mouse events
	    out_str_nf((char_u *)"\033['z");
	mouse_ison = on;
    }
#endif

#ifdef FEAT_MOUSE_GPM
    else
    {
	if (on)
	{
	    if (gpm_open())
		mouse_ison = TRUE;
	}
	else
	{
	    gpm_close();
	    mouse_ison = FALSE;
	}
    }
#endif

#ifdef FEAT_SYSMOUSE
    else
    {
	if (on)
	{
	    if (sysmouse_open() == OK)
		mouse_ison = TRUE;
	}
	else
	{
	    sysmouse_close();
	    mouse_ison = FALSE;
	}
    }
#endif

#ifdef FEAT_MOUSE_JSB
    else
    {
	if (on)
	{
	    // D - Enable Mouse up/down messages
	    // L - Enable Left Button Reporting
	    // M - Enable Middle Button Reporting
	    // R - Enable Right Button Reporting
	    // K - Enable SHIFT and CTRL key Reporting
	    // + - Enable Advanced messaging of mouse moves and up/down messages
	    // Q - Quiet No Ack
	    // # - Numeric value of mouse pointer required
	    //	  0 = Multiview 2000 cursor, used as standard
	    //	  1 = Windows Arrow
	    //	  2 = Windows I Beam
	    //	  3 = Windows Hour Glass
	    //	  4 = Windows Cross Hair
	    //	  5 = Windows UP Arrow
# ifdef JSBTERM_MOUSE_NONADVANCED
	    // Disables full feedback of pointer movements
	    out_str_nf((char_u *)"\033[0~ZwLMRK1Q\033\\");
# else
	    out_str_nf((char_u *)"\033[0~ZwLMRK+1Q\033\\");
# endif
	    mouse_ison = TRUE;
	}
	else
	{
	    out_str_nf((char_u *)"\033[0~ZwQ\033\\");
	    mouse_ison = FALSE;
	}
    }
#endif
#ifdef FEAT_MOUSE_PTERM
    else
    {
	// 1 = button press, 6 = release, 7 = drag, 1h...9l = right button
	if (on)
	    out_str_nf("\033[>1h\033[>6h\033[>7h\033[>1h\033[>9l");
	else
	    out_str_nf("\033[>1l\033[>6l\033[>7l\033[>1l\033[>9h");
	mouse_ison = on;
    }
#endif
}

#if defined(FEAT_BEVAL_TERM) || defined(PROTO)
/*
 * Called when 'balloonevalterm' changed.
 */
    void
mch_bevalterm_changed(void)
{
    mch_setmouse(mouse_ison);
}
#endif

/*
 * Set the mouse termcode, depending on the 'term' and 'ttymouse' options.
 */
    void
check_mouse_termcode(void)
{
# ifdef FEAT_MOUSE_XTERM
    if (use_xterm_mouse()
# ifdef FEAT_MOUSE_URXVT
	    && use_xterm_mouse() != 3
# endif
#  ifdef FEAT_GUI
	    && !gui.in_use
#  endif
	    )
    {
	set_mouse_termcode(KS_MOUSE, (char_u *)(term_is_8bit(T_NAME)
							? "\233M" : "\033[M"));
	if (*p_mouse != NUL)
	{
	    // force mouse off and maybe on to send possibly new mouse
	    // activation sequence to the xterm, with(out) drag tracing.
	    mch_setmouse(FALSE);
	    setmouse();
	}
    }
    else
	del_mouse_termcode(KS_MOUSE);
# endif

# ifdef FEAT_MOUSE_GPM
    if (!use_xterm_mouse()
#  ifdef FEAT_GUI
	    && !gui.in_use
#  endif
	    )
	set_mouse_termcode(KS_GPM_MOUSE, (char_u *)"\033MG");
    else
	del_mouse_termcode(KS_GPM_MOUSE);
# endif

# ifdef FEAT_SYSMOUSE
    if (!use_xterm_mouse()
#  ifdef FEAT_GUI
	    && !gui.in_use
#  endif
	    )
	set_mouse_termcode(KS_MOUSE, (char_u *)"\033MS");
# endif

# ifdef FEAT_MOUSE_JSB
    // Conflicts with xterm mouse: "\033[" and "\033[M" ???
    if (!use_xterm_mouse()
#  ifdef FEAT_GUI
	    && !gui.in_use
#  endif
	    )
	set_mouse_termcode(KS_JSBTERM_MOUSE, (char_u *)"\033[0~zw");
    else
	del_mouse_termcode(KS_JSBTERM_MOUSE);
# endif

# ifdef FEAT_MOUSE_NET
    // There is no conflict, but one may type "ESC }" from Insert mode.  Don't
    // define it in the GUI or when using an xterm.
    if (!use_xterm_mouse()
#  ifdef FEAT_GUI
	    && !gui.in_use
#  endif
	    )
	set_mouse_termcode(KS_NETTERM_MOUSE, (char_u *)"\033}");
    else
	del_mouse_termcode(KS_NETTERM_MOUSE);
# endif

# ifdef FEAT_MOUSE_DEC
    // Conflicts with xterm mouse: "\033[" and "\033[M"
    if (!use_xterm_mouse()
#  ifdef FEAT_GUI
	    && !gui.in_use
#  endif
	    )
	set_mouse_termcode(KS_DEC_MOUSE, (char_u *)(term_is_8bit(T_NAME)
							  ? "\233" : "\033["));
    else
	del_mouse_termcode(KS_DEC_MOUSE);
# endif
# ifdef FEAT_MOUSE_PTERM
    // same conflict as the dec mouse
    if (!use_xterm_mouse()
#  ifdef FEAT_GUI
	    && !gui.in_use
#  endif
	    )
	set_mouse_termcode(KS_PTERM_MOUSE, (char_u *)"\033[");
    else
	del_mouse_termcode(KS_PTERM_MOUSE);
# endif
# ifdef FEAT_MOUSE_URXVT
    if (use_xterm_mouse() == 3
#  ifdef FEAT_GUI
	    && !gui.in_use
#  endif
	    )
    {
	set_mouse_termcode(KS_URXVT_MOUSE, (char_u *)(term_is_8bit(T_NAME)
						      ? "\233*M" : "\033[*M"));

	if (*p_mouse != NUL)
	{
	    mch_setmouse(FALSE);
	    setmouse();
	}
    }
    else
	del_mouse_termcode(KS_URXVT_MOUSE);
# endif
    if (use_xterm_mouse() == 4
# ifdef FEAT_GUI
	    && !gui.in_use
# endif
	    )
    {
	set_mouse_termcode(KS_SGR_MOUSE, (char_u *)(term_is_8bit(T_NAME)
						    ? "\233<*M" : "\033[<*M"));

	set_mouse_termcode(KS_SGR_MOUSE_RELEASE, (char_u *)(term_is_8bit(T_NAME)
						    ? "\233<*m" : "\033[<*m"));

	if (*p_mouse != NUL)
	{
	    mch_setmouse(FALSE);
	    setmouse();
	}
    }
    else
    {
	del_mouse_termcode(KS_SGR_MOUSE);
	del_mouse_termcode(KS_SGR_MOUSE_RELEASE);
    }
}

#ifndef VMS

/*
 * Try to get the current window size:
 * 1. with an ioctl(), most accurate method
 * 2. from the environment variables LINES and COLUMNS
 * 3. from the termcap
 * 4. keep using the old values
 * Return OK when size could be determined, FAIL otherwise.
 */
    int
mch_get_shellsize(void)
{
    long	rows = 0;
    long	columns = 0;
    char_u	*p;

    /*
     * 1. try using an ioctl. It is the most accurate method.
     *
     * Try using TIOCGWINSZ first, some systems that have it also define
     * TIOCGSIZE but don't have a struct ttysize.
     */
# ifdef TIOCGWINSZ
    {
	struct winsize	ws;
	int fd = 1;

	// When stdout is not a tty, use stdin for the ioctl().
	if (!isatty(fd) && isatty(read_cmd_fd))
	    fd = read_cmd_fd;
	if (ioctl(fd, TIOCGWINSZ, &ws) == 0)
	{
	    columns = ws.ws_col;
	    rows = ws.ws_row;
#  ifdef FEAT_EVAL
	    ch_log(NULL, "Got size with TIOCGWINSZ: %ld x %ld", columns, rows);
#  endif
	}
    }
# else // TIOCGWINSZ
#  ifdef TIOCGSIZE
    {
	struct ttysize	ts;
	int fd = 1;

	// When stdout is not a tty, use stdin for the ioctl().
	if (!isatty(fd) && isatty(read_cmd_fd))
	    fd = read_cmd_fd;
	if (ioctl(fd, TIOCGSIZE, &ts) == 0)
	{
	    columns = ts.ts_cols;
	    rows = ts.ts_lines;
#  ifdef FEAT_EVAL
	    ch_log(NULL, "Got size with TIOCGSIZE: %ld x %ld", columns, rows);
#  endif
	}
    }
#  endif // TIOCGSIZE
# endif // TIOCGWINSZ

    /*
     * 2. get size from environment
     *    When being POSIX compliant ('|' flag in 'cpoptions') this overrules
     *    the ioctl() values!
     */
    if (columns == 0 || rows == 0 || vim_strchr(p_cpo, CPO_TSIZE) != NULL)
    {
	if ((p = (char_u *)getenv("LINES")))
	{
	    rows = atoi((char *)p);
#  ifdef FEAT_EVAL
	    ch_log(NULL, "Got 'lines' from $LINES: %ld", rows);
#  endif
	}
	if ((p = (char_u *)getenv("COLUMNS")))
	{
	    columns = atoi((char *)p);
#  ifdef FEAT_EVAL
	    ch_log(NULL, "Got 'columns' from $COLUMNS: %ld", columns);
#  endif
	}
    }

#ifdef HAVE_TGETENT
    /*
     * 3. try reading "co" and "li" entries from termcap
     */
    if (columns == 0 || rows == 0)
    {
	getlinecol(&columns, &rows);
# ifdef FEAT_EVAL
	ch_log(NULL, "Got size from termcap: %ld x %ld", columns, rows);
# endif
    }
#endif

    /*
     * 4. If everything fails, use the old values
     */
    if (columns <= 0 || rows <= 0)
	return FAIL;

    Rows = rows;
    Columns = columns;
    limit_screen_size();
    return OK;
}

#if defined(FEAT_TERMINAL) || defined(PROTO)
/*
 * Report the windows size "rows" and "cols" to tty "fd".
 */
    int
mch_report_winsize(int fd, int rows, int cols)
{
    int		tty_fd;
    int		retval = -1;

    tty_fd = get_tty_fd(fd);
    if (tty_fd < 0)
	return FAIL;

# if defined(TIOCSWINSZ)
    struct winsize ws;

    ws.ws_col = cols;
    ws.ws_row = rows;
    ws.ws_xpixel = cols * 5;
    ws.ws_ypixel = rows * 10;
    retval = ioctl(tty_fd, TIOCSWINSZ, &ws);
    ch_log(NULL, "ioctl(TIOCSWINSZ) %s", retval == 0 ? "success" : "failed");
# elif defined(TIOCSSIZE)
    struct ttysize ts;

    ts.ts_cols = cols;
    ts.ts_lines = rows;
    retval = ioctl(tty_fd, TIOCSSIZE, &ts);
    ch_log(NULL, "ioctl(TIOCSSIZE) %s", retval == 0 ? "success" : "failed");
# endif
    if (tty_fd != fd)
	close(tty_fd);
    return retval == 0 ? OK : FAIL;
}
#endif

/*
 * Try to set the window size to Rows and Columns.
 */
    void
mch_set_shellsize(void)
{
    if (*T_CWS)
    {
	/*
	 * NOTE: if you get an error here that term_set_winsize() is
	 * undefined, check the output of configure.  It could probably not
	 * find a ncurses, termcap or termlib library.
	 */
	term_set_winsize((int)Rows, (int)Columns);
	out_flush();
	screen_start();			// don't know where cursor is now
    }
}

#endif // VMS

/*
 * Rows and/or Columns has changed.
 */
    void
mch_new_shellsize(void)
{
    // Nothing to do.
}

/*
 * Wait for process "child" to end.
 * Return "child" if it exited properly, <= 0 on error.
 */
    static pid_t
wait4pid(pid_t child, waitstatus *status)
{
    pid_t wait_pid = 0;
    long delay_msec = 1;

    while (wait_pid != child)
    {
	// When compiled with Python threads are probably used, in which case
	// wait() sometimes hangs for no obvious reason.  Use waitpid()
	// instead and loop (like the GUI). Also needed for other interfaces,
	// they might call system().
# ifdef __NeXT__
	wait_pid = wait4(child, status, WNOHANG, (struct rusage *)0);
# else
	wait_pid = waitpid(child, status, WNOHANG);
# endif
	if (wait_pid == 0)
	{
	    // Wait for 1 to 10 msec before trying again.
	    mch_delay(delay_msec, MCH_DELAY_IGNOREINPUT | MCH_DELAY_SETTMODE);
	    if (++delay_msec > 10)
		delay_msec = 10;
	    continue;
	}
	if (wait_pid <= 0
# ifdef ECHILD
		&& errno == ECHILD
# endif
	   )
	    break;
    }
    return wait_pid;
}

#if !defined(USE_SYSTEM) || defined(FEAT_JOB_CHANNEL)
/*
 * Set the environment for a child process.
 */
    static void
set_child_environment(
	long	rows,
	long	columns,
	char	*term,
	int	is_terminal UNUSED)
{
# ifdef HAVE_SETENV
    char	envbuf[50];
# else
    static char	envbuf_Term[30];
    static char	envbuf_Rows[20];
    static char	envbuf_Lines[20];
    static char	envbuf_Columns[20];
    static char	envbuf_Colors[20];
#  ifdef FEAT_TERMINAL
    static char	envbuf_Version[20];
#  endif
#  ifdef FEAT_CLIENTSERVER
    static char	envbuf_Servername[60];
#  endif
# endif

# ifdef HAVE_SETENV
    setenv("TERM", term, 1);
    sprintf((char *)envbuf, "%ld", rows);
    setenv("ROWS", (char *)envbuf, 1);
    sprintf((char *)envbuf, "%ld", rows);
    setenv("LINES", (char *)envbuf, 1);
    sprintf((char *)envbuf, "%ld", columns);
    setenv("COLUMNS", (char *)envbuf, 1);
    sprintf((char *)envbuf, "%d", t_colors);
    setenv("COLORS", (char *)envbuf, 1);
#  ifdef FEAT_TERMINAL
    if (is_terminal)
    {
	sprintf((char *)envbuf, "%ld",  (long)get_vim_var_nr(VV_VERSION));
	setenv("VIM_TERMINAL", (char *)envbuf, 1);
    }
#  endif
#  ifdef FEAT_CLIENTSERVER
    setenv("VIM_SERVERNAME", serverName == NULL ? "" : (char *)serverName, 1);
#  endif
# else
    /*
     * Putenv does not copy the string, it has to remain valid.
     * Use a static array to avoid losing allocated memory.
     * This won't work well when running multiple children...
     */
    vim_snprintf(envbuf_Term, sizeof(envbuf_Term), "TERM=%s", term);
    putenv(envbuf_Term);
    vim_snprintf(envbuf_Rows, sizeof(envbuf_Rows), "ROWS=%ld", rows);
    putenv(envbuf_Rows);
    vim_snprintf(envbuf_Lines, sizeof(envbuf_Lines), "LINES=%ld", rows);
    putenv(envbuf_Lines);
    vim_snprintf(envbuf_Columns, sizeof(envbuf_Columns),
						       "COLUMNS=%ld", columns);
    putenv(envbuf_Columns);
    vim_snprintf(envbuf_Colors, sizeof(envbuf_Colors), "COLORS=%ld", t_colors);
    putenv(envbuf_Colors);
#  ifdef FEAT_TERMINAL
    if (is_terminal)
    {
	vim_snprintf(envbuf_Version, sizeof(envbuf_Version),
			 "VIM_TERMINAL=%ld", (long)get_vim_var_nr(VV_VERSION));
	putenv(envbuf_Version);
    }
#  endif
#  ifdef FEAT_CLIENTSERVER
    vim_snprintf(envbuf_Servername, sizeof(envbuf_Servername),
	    "VIM_SERVERNAME=%s", serverName == NULL ? "" : (char *)serverName);
    putenv(envbuf_Servername);
#  endif
# endif
}

    static void
set_default_child_environment(int is_terminal)
{
    set_child_environment(Rows, Columns, "dumb", is_terminal);
}
#endif

#if defined(FEAT_GUI) || defined(FEAT_JOB_CHANNEL)
/*
 * Open a PTY, with FD for the master and slave side.
 * When failing "pty_master_fd" and "pty_slave_fd" are -1.
 * When successful both file descriptors are stored and the allocated pty name
 * is stored in both "*name1" and "*name2".
 */
    static void
open_pty(int *pty_master_fd, int *pty_slave_fd, char_u **name1, char_u **name2)
{
    char	*tty_name;

    if (name1 != NULL)
	*name1 = NULL;
    if (name2 != NULL)
	*name2 = NULL;

    *pty_master_fd = mch_openpty(&tty_name);	    // open pty
    if (*pty_master_fd < 0)
	return;

    // Leaving out O_NOCTTY may lead to waitpid() always returning
    // 0 on Mac OS X 10.7 thereby causing freezes. Let's assume
    // adding O_NOCTTY always works when defined.
#ifdef O_NOCTTY
    *pty_slave_fd = open(tty_name, O_RDWR | O_NOCTTY | O_EXTRA, 0);
#else
    *pty_slave_fd = open(tty_name, O_RDWR | O_EXTRA, 0);
#endif
    if (*pty_slave_fd < 0)
    {
	close(*pty_master_fd);
	*pty_master_fd = -1;
    }
    else
    {
	if (name1 != NULL)
	    *name1 = vim_strsave((char_u *)tty_name);
	if (name2 != NULL)
	    *name2 = vim_strsave((char_u *)tty_name);
    }
}
#endif

/*
 * Send SIGINT to a child process if "c" is an interrupt character.
 */
    static void
may_send_sigint(int c UNUSED, pid_t pid UNUSED, pid_t wpid UNUSED)
{
# ifdef SIGINT
    if (c == Ctrl_C || c == intr_char)
    {
#  ifdef HAVE_SETSID
	kill(-pid, SIGINT);
#  else
	kill(0, SIGINT);
#  endif
	if (wpid > 0)
	    kill(wpid, SIGINT);
    }
# endif
}

#if !defined(USE_SYSTEM) || defined(FEAT_TERMINAL) || defined(PROTO)

/*
 * Parse "cmd" and return the result in "argvp" which is an allocated array of
 * pointers, the last one is NULL.
 * The "sh_tofree" and "shcf_tofree" must be later freed by the caller.
 */
    int
unix_build_argv(
	char_u *cmd,
	char ***argvp,
	char_u **sh_tofree,
	char_u **shcf_tofree)
{
    char	**argv = NULL;
    int		argc;

    *sh_tofree = vim_strsave(p_sh);
    if (*sh_tofree == NULL)		// out of memory
	return FAIL;

    if (mch_parse_cmd(*sh_tofree, TRUE, &argv, &argc) == FAIL)
	return FAIL;
    *argvp = argv;

    if (cmd != NULL)
    {
	char_u	*s;
	char_u	*p;

	if (extra_shell_arg != NULL)
	    argv[argc++] = (char *)extra_shell_arg;

	// Break 'shellcmdflag' into white separated parts.  This doesn't
	// handle quoted strings, they are very unlikely to appear.
	*shcf_tofree = alloc(STRLEN(p_shcf) + 1);
	if (*shcf_tofree == NULL)    // out of memory
	    return FAIL;
	s = *shcf_tofree;
	p = p_shcf;
	while (*p != NUL)
	{
	    argv[argc++] = (char *)s;
	    while (*p && *p != ' ' && *p != TAB)
		*s++ = *p++;
	    *s++ = NUL;
	    p = skipwhite(p);
	}

	argv[argc++] = (char *)cmd;
    }
    argv[argc] = NULL;
    return OK;
}
#endif

#if defined(FEAT_GUI) && defined(FEAT_TERMINAL)
/*
 * Use a terminal window to run a shell command in.
 */
    static int
mch_call_shell_terminal(
    char_u	*cmd,
    int		options UNUSED)	// SHELL_*, see vim.h
{
    jobopt_T	opt;
    char	**argv = NULL;
    char_u	*tofree1 = NULL;
    char_u	*tofree2 = NULL;
    int		retval = -1;
    buf_T	*buf;
    job_T	*job;
    aco_save_T	aco;
    oparg_T	oa;		// operator arguments

    if (unix_build_argv(cmd, &argv, &tofree1, &tofree2) == FAIL)
	goto theend;

    init_job_options(&opt);
    ch_log(NULL, "starting terminal for system command '%s'", cmd);
    buf = term_start(NULL, argv, &opt, TERM_START_SYSTEM);
    if (buf == NULL)
	goto theend;

    job = term_getjob(buf->b_term);
    ++job->jv_refcount;

    // Find a window to make "buf" curbuf.
    aucmd_prepbuf(&aco, buf);
    if (curbuf == buf)
    {
	// Only when managed to find a window for "buf",
	clear_oparg(&oa);
	while (term_use_loop())
	{
	    if (oa.op_type == OP_NOP && oa.regname == NUL && !VIsual_active)
	    {
		// If terminal_loop() returns OK we got a key that is handled
		// in Normal model. We don't do redrawing anyway.
		if (terminal_loop(TRUE) == OK)
		    normal_cmd(&oa, TRUE);
	    }
	    else
		normal_cmd(&oa, TRUE);
	}
	retval = job->jv_exitval;
	ch_log(NULL, "system command finished");

	job_unref(job);

	// restore curwin/curbuf and a few other things
	aucmd_restbuf(&aco);
    }

    // Only require pressing Enter when redrawing, to avoid that system() gets
    // the hit-enter prompt even though it didn't output anything.
    if (RedrawingDisabled == 0)
	wait_return(TRUE);
    do_buffer(DOBUF_WIPE, DOBUF_FIRST, FORWARD, buf->b_fnum, TRUE);

theend:
    vim_free(argv);
    vim_free(tofree1);
    vim_free(tofree2);
    return retval;
}
#endif

#ifdef USE_SYSTEM
/*
 * Use system() to start the shell: simple but slow.
 */
    static int
mch_call_shell_system(
    char_u	*cmd,
    int		options)	// SHELL_*, see vim.h
{
#ifdef VMS
    char	*ifn = NULL;
    char	*ofn = NULL;
#endif
    tmode_T	tmode = cur_tmode;
    char_u	*newcmd;	// only needed for unix
    int		x;

    out_flush();

    if (options & SHELL_COOKED)
	settmode(TMODE_COOK);	    // set to normal mode

# if defined(FEAT_CLIPBOARD) && defined(FEAT_X11)
    save_clipboard();
    loose_clipboard();
# endif

    if (cmd == NULL)
	x = system((char *)p_sh);
    else
    {
# ifdef VMS
	if (ofn = strchr((char *)cmd, '>'))
	    *ofn++ = '\0';
	if (ifn = strchr((char *)cmd, '<'))
	{
	    char *p;

	    *ifn++ = '\0';
	    p = strchr(ifn,' '); // chop off any trailing spaces
	    if (p)
		*p = '\0';
	}
	if (ofn)
	    x = vms_sys((char *)cmd, ofn, ifn);
	else
	    x = system((char *)cmd);
# else
	newcmd = alloc(STRLEN(p_sh)
		+ (extra_shell_arg == NULL ? 0 : STRLEN(extra_shell_arg))
		+ STRLEN(p_shcf) + STRLEN(cmd) + 4);
	if (newcmd == NULL)
	    x = 0;
	else
	{
	    sprintf((char *)newcmd, "%s %s %s %s", p_sh,
		    extra_shell_arg == NULL ? "" : (char *)extra_shell_arg,
		    (char *)p_shcf,
		    (char *)cmd);
	    x = system((char *)newcmd);
	    vim_free(newcmd);
	}
# endif
    }
# ifdef VMS
    x = vms_sys_status(x);
# endif
    if (emsg_silent)
	;
    else if (x == 127)
	msg_puts(_("\nCannot execute shell sh\n"));
    else if (x && !(options & SHELL_SILENT))
    {
	msg_puts(_("\nshell returned "));
	msg_outnum((long)x);
	msg_putchar('\n');
    }

    if (tmode == TMODE_RAW)
    {
	// The shell may have messed with the mode, always set it.
	cur_tmode = TMODE_UNKNOWN;
	settmode(TMODE_RAW);	// set to raw mode
    }
    resettitle();
# if defined(FEAT_CLIPBOARD) && defined(FEAT_X11)
    restore_clipboard();
# endif
    return x;
}

#else // USE_SYSTEM

# define EXEC_FAILED 122    // Exit code when shell didn't execute.  Don't use
			    // 127, some shells use that already
# define OPEN_NULL_FAILED 123 // Exit code if /dev/null can't be opened

/*
 * Don't use system(), use fork()/exec().
 */
    static int
mch_call_shell_fork(
    char_u	*cmd,
    int		options)	// SHELL_*, see vim.h
{
    tmode_T	tmode = cur_tmode;
    pid_t	pid;
    pid_t	wpid = 0;
    pid_t	wait_pid = 0;
# ifdef HAVE_UNION_WAIT
    union wait	status;
# else
    int		status = -1;
# endif
    int		retval = -1;
    char	**argv = NULL;
    char_u	*tofree1 = NULL;
    char_u	*tofree2 = NULL;
    int		i;
    int		pty_master_fd = -1;	    // for pty's
# ifdef FEAT_GUI
    int		pty_slave_fd = -1;
# endif
    int		fd_toshell[2];		// for pipes
    int		fd_fromshell[2];
    int		pipe_error = FALSE;
    int		did_settmode = FALSE;	// settmode(TMODE_RAW) called

    out_flush();
    if (options & SHELL_COOKED)
	settmode(TMODE_COOK);		// set to normal mode
    if (tmode == TMODE_RAW)
	// The shell may have messed with the mode, always set it later.
	cur_tmode = TMODE_UNKNOWN;

    if (unix_build_argv(cmd, &argv, &tofree1, &tofree2) == FAIL)
	goto error;

    /*
     * For the GUI, when writing the output into the buffer and when reading
     * input from the buffer: Try using a pseudo-tty to get the stdin/stdout
     * of the executed command into the Vim window.  Or use a pipe.
     */
    if ((options & (SHELL_READ|SHELL_WRITE))
# ifdef FEAT_GUI
	    || (gui.in_use && show_shell_mess)
# endif
		    )
    {
# ifdef FEAT_GUI
	/*
	 * Try to open a master pty.
	 * If this works, open the slave pty.
	 * If the slave can't be opened, close the master pty.
	 */
	if (p_guipty && !(options & (SHELL_READ|SHELL_WRITE)))
	    open_pty(&pty_master_fd, &pty_slave_fd, NULL, NULL);
	/*
	 * If not opening a pty or it didn't work, try using pipes.
	 */
	if (pty_master_fd < 0)
# endif
	{
	    pipe_error = (pipe(fd_toshell) < 0);
	    if (!pipe_error)			    // pipe create OK
	    {
		pipe_error = (pipe(fd_fromshell) < 0);
		if (pipe_error)			    // pipe create failed
		{
		    close(fd_toshell[0]);
		    close(fd_toshell[1]);
		}
	    }
	    if (pipe_error)
	    {
		msg_puts(_("\nCannot create pipes\n"));
		out_flush();
	    }
	}
    }

    if (!pipe_error)			// pty or pipe opened or not used
    {
	SIGSET_DECL(curset)
	BLOCK_SIGNALS(&curset);
	pid = fork();	// maybe we should use vfork()
	if (pid == -1)
	{
	    UNBLOCK_SIGNALS(&curset);

	    msg_puts(_("\nCannot fork\n"));
	    if ((options & (SHELL_READ|SHELL_WRITE))
# ifdef FEAT_GUI
		|| (gui.in_use && show_shell_mess)
# endif
		    )
	    {
# ifdef FEAT_GUI
		if (pty_master_fd >= 0)		// close the pseudo tty
		{
		    close(pty_master_fd);
		    close(pty_slave_fd);
		}
		else				// close the pipes
# endif
		{
		    close(fd_toshell[0]);
		    close(fd_toshell[1]);
		    close(fd_fromshell[0]);
		    close(fd_fromshell[1]);
		}
	    }
	}
	else if (pid == 0)	// child
	{
	    reset_signals();		// handle signals normally
	    UNBLOCK_SIGNALS(&curset);

# ifdef FEAT_EVAL
	    if (ch_log_active())
	    {
		ch_log(NULL, "closing channel log in the child process");
		ch_logfile((char_u *)"", (char_u *)"");
	    }
# endif

	    if (!show_shell_mess || (options & SHELL_EXPAND))
	    {
		int fd;

		/*
		 * Don't want to show any message from the shell.  Can't just
		 * close stdout and stderr though, because some systems will
		 * break if you try to write to them after that, so we must
		 * use dup() to replace them with something else -- webb
		 * Connect stdin to /dev/null too, so ":n `cat`" doesn't hang,
		 * waiting for input.
		 */
		fd = open("/dev/null", O_RDWR | O_EXTRA, 0);
		fclose(stdin);
		fclose(stdout);
		fclose(stderr);

		/*
		 * If any of these open()'s and dup()'s fail, we just continue
		 * anyway.  It's not fatal, and on most systems it will make
		 * no difference at all.  On a few it will cause the execvp()
		 * to exit with a non-zero status even when the completion
		 * could be done, which is nothing too serious.  If the open()
		 * or dup() failed we'd just do the same thing ourselves
		 * anyway -- webb
		 */
		if (fd >= 0)
		{
		    vim_ignored = dup(fd); // To replace stdin  (fd 0)
		    vim_ignored = dup(fd); // To replace stdout (fd 1)
		    vim_ignored = dup(fd); // To replace stderr (fd 2)

		    // Don't need this now that we've duplicated it
		    close(fd);
		}
	    }
	    else if ((options & (SHELL_READ|SHELL_WRITE))
# ifdef FEAT_GUI
		    || gui.in_use
# endif
		    )
	    {

# ifdef HAVE_SETSID
		// Create our own process group, so that the child and all its
		// children can be kill()ed.  Don't do this when using pipes,
		// because stdin is not a tty, we would lose /dev/tty.
		if (p_stmp)
		{
		    (void)setsid();
#  if defined(SIGHUP)
		    // When doing "!xterm&" and 'shell' is bash: the shell
		    // will exit and send SIGHUP to all processes in its
		    // group, killing the just started process.  Ignore SIGHUP
		    // to avoid that. (suggested by Simon Schubert)
		    mch_signal(SIGHUP, SIG_IGN);
#  endif
		}
# endif
# ifdef FEAT_GUI
		if (pty_slave_fd >= 0)
		{
		    // push stream discipline modules
		    if (options & SHELL_COOKED)
			setup_slavepty(pty_slave_fd);
#  ifdef TIOCSCTTY
		    // Try to become controlling tty (probably doesn't work,
		    // unless run by root)
		    ioctl(pty_slave_fd, TIOCSCTTY, (char *)NULL);
#  endif
		}
# endif
		set_default_child_environment(FALSE);

		/*
		 * stderr is only redirected when using the GUI, so that a
		 * program like gpg can still access the terminal to get a
		 * passphrase using stderr.
		 */
# ifdef FEAT_GUI
		if (pty_master_fd >= 0)
		{
		    close(pty_master_fd);   // close master side of pty

		    // set up stdin/stdout/stderr for the child
		    close(0);
		    vim_ignored = dup(pty_slave_fd);
		    close(1);
		    vim_ignored = dup(pty_slave_fd);
		    if (gui.in_use)
		    {
			close(2);
			vim_ignored = dup(pty_slave_fd);
		    }

		    close(pty_slave_fd);    // has been dupped, close it now
		}
		else
# endif
		{
		    // set up stdin for the child
		    close(fd_toshell[1]);
		    close(0);
		    vim_ignored = dup(fd_toshell[0]);
		    close(fd_toshell[0]);

		    // set up stdout for the child
		    close(fd_fromshell[0]);
		    close(1);
		    vim_ignored = dup(fd_fromshell[1]);
		    close(fd_fromshell[1]);

# ifdef FEAT_GUI
		    if (gui.in_use)
		    {
			// set up stderr for the child
			close(2);
			vim_ignored = dup(1);
		    }
# endif
		}
	    }

	    /*
	     * There is no type cast for the argv, because the type may be
	     * different on different machines. This may cause a warning
	     * message with strict compilers, don't worry about it.
	     * Call _exit() instead of exit() to avoid closing the connection
	     * to the X server (esp. with GTK, which uses atexit()).
	     */
	    execvp(argv[0], argv);
	    _exit(EXEC_FAILED);	    // exec failed, return failure code
	}
	else			// parent
	{
	    /*
	     * While child is running, ignore terminating signals.
	     * Do catch CTRL-C, so that "got_int" is set.
	     */
	    catch_signals(SIG_IGN, SIG_ERR);
	    catch_int_signal();
	    UNBLOCK_SIGNALS(&curset);
# ifdef FEAT_JOB_CHANNEL
	    ++dont_check_job_ended;
# endif
	    /*
	     * For the GUI we redirect stdin, stdout and stderr to our window.
	     * This is also used to pipe stdin/stdout to/from the external
	     * command.
	     */
	    if ((options & (SHELL_READ|SHELL_WRITE))
# ifdef FEAT_GUI
		    || (gui.in_use && show_shell_mess)
# endif
	       )
	    {
# define BUFLEN 100		// length for buffer, pseudo tty limit is 128
		char_u	    buffer[BUFLEN + 1];
		int	    buffer_off = 0;	// valid bytes in buffer[]
		char_u	    ta_buf[BUFLEN + 1];	// TypeAHead
		int	    ta_len = 0;		// valid bytes in ta_buf[]
		int	    len;
		int	    p_more_save;
		int	    old_State;
		int	    c;
		int	    toshell_fd;
		int	    fromshell_fd;
		garray_T    ga;
		int	    noread_cnt;
# ifdef ELAPSED_FUNC
		elapsed_T   start_tv;
# endif

# ifdef FEAT_GUI
		if (pty_master_fd >= 0)
		{
		    fromshell_fd = pty_master_fd;
		    toshell_fd = dup(pty_master_fd);
		}
		else
# endif
		{
		    close(fd_toshell[0]);
		    close(fd_fromshell[1]);
		    toshell_fd = fd_toshell[1];
		    fromshell_fd = fd_fromshell[0];
		}

		/*
		 * Write to the child if there are typed characters.
		 * Read from the child if there are characters available.
		 *   Repeat the reading a few times if more characters are
		 *   available. Need to check for typed keys now and then, but
		 *   not too often (delays when no chars are available).
		 * This loop is quit if no characters can be read from the pty
		 * (WaitForChar detected special condition), or there are no
		 * characters available and the child has exited.
		 * Only check if the child has exited when there is no more
		 * output. The child may exit before all the output has
		 * been printed.
		 *
		 * Currently this busy loops!
		 * This can probably dead-lock when the write blocks!
		 */
		p_more_save = p_more;
		p_more = FALSE;
		old_State = State;
		State = MODE_EXTERNCMD;	// don't redraw at window resize

		if ((options & SHELL_WRITE) && toshell_fd >= 0)
		{
		    // Fork a process that will write the lines to the
		    // external program.
		    if ((wpid = fork()) == -1)
		    {
			msg_puts(_("\nCannot fork\n"));
		    }
		    else if (wpid == 0) // child
		    {
			linenr_T    lnum = curbuf->b_op_start.lnum;
			int	    written = 0;
			char_u	    *lp = ml_get(lnum);
			size_t	    l;

			close(fromshell_fd);
			for (;;)
			{
			    l = STRLEN(lp + written);
			    if (l == 0)
				len = 0;
			    else if (lp[written] == NL)
				// NL -> NUL translation
				len = write(toshell_fd, "", (size_t)1);
			    else
			    {
				char_u	*s = vim_strchr(lp + written, NL);

				len = write(toshell_fd, (char *)lp + written,
					   s == NULL ? l
					      : (size_t)(s - (lp + written)));
			    }
			    if (len == (int)l)
			    {
				// Finished a line, add a NL, unless this line
				// should not have one.
				if (lnum != curbuf->b_op_end.lnum
					|| (!curbuf->b_p_bin
					    && curbuf->b_p_fixeol)
					|| (lnum != curbuf->b_no_eol_lnum
					    && (lnum != curbuf->b_ml.ml_line_count
						    || curbuf->b_p_eol)))
				    vim_ignored = write(toshell_fd, "\n",
								   (size_t)1);
				++lnum;
				if (lnum > curbuf->b_op_end.lnum)
				{
				    // finished all the lines, close pipe
				    close(toshell_fd);
				    break;
				}
				lp = ml_get(lnum);
				written = 0;
			    }
			    else if (len > 0)
				written += len;
			}
			_exit(0);
		    }
		    else // parent
		    {
			close(toshell_fd);
			toshell_fd = -1;
		    }
		}

		if (options & SHELL_READ)
		    ga_init2(&ga, 1, BUFLEN);

		noread_cnt = 0;
# ifdef ELAPSED_FUNC
		ELAPSED_INIT(start_tv);
# endif
		for (;;)
		{
		    /*
		     * Check if keys have been typed, write them to the child
		     * if there are any.
		     * Don't do this if we are expanding wild cards (would eat
		     * typeahead).
		     * Don't do this when filtering and terminal is in cooked
		     * mode, the shell command will handle the I/O.  Avoids
		     * that a typed password is echoed for ssh or gpg command.
		     * Don't get characters when the child has already
		     * finished (wait_pid == 0).
		     * Don't read characters unless we didn't get output for a
		     * while (noread_cnt > 4), avoids that ":r !ls" eats
		     * typeahead.
		     */
		    len = 0;
		    if (!(options & SHELL_EXPAND)
			    && ((options &
					 (SHELL_READ|SHELL_WRITE|SHELL_COOKED))
				      != (SHELL_READ|SHELL_WRITE|SHELL_COOKED)
# ifdef FEAT_GUI
						    || gui.in_use
# endif
						    )
			    && wait_pid == 0
			    && (ta_len > 0 || noread_cnt > 4))
		    {
		      if (ta_len == 0)
		      {
			  // Get extra characters when we don't have any.
			  // Reset the counter and timer.
			  noread_cnt = 0;
# ifdef ELAPSED_FUNC
			  ELAPSED_INIT(start_tv);
# endif
			  len = ui_inchar(ta_buf, BUFLEN, 10L, 0);
		      }
		      if (ta_len > 0 || len > 0)
		      {
			/*
			 * For pipes:
			 * Check for CTRL-C: send interrupt signal to child.
			 * Check for CTRL-D: EOF, close pipe to child.
			 */
			if (len == 1 && (pty_master_fd < 0 || cmd != NULL))
			{
			    /*
			     * Send SIGINT to the child's group or all
			     * processes in our group.
			     */
			    may_send_sigint(ta_buf[ta_len], pid, wpid);

			    if (pty_master_fd < 0 && toshell_fd >= 0
					       && ta_buf[ta_len] == Ctrl_D)
			    {
				close(toshell_fd);
				toshell_fd = -1;
			    }
			}

			// Remove Vim-specific codes from the input.
			len = term_replace_keycodes(ta_buf, ta_len, len);

			/*
			 * For pipes: echo the typed characters.
			 * For a pty this does not seem to work.
			 */
			if (pty_master_fd < 0)
			{
			    for (i = ta_len; i < ta_len + len; ++i)
			    {
				if (ta_buf[i] == '\n' || ta_buf[i] == '\b')
				    msg_putchar(ta_buf[i]);
				else if (has_mbyte)
				{
				    int l = (*mb_ptr2len)(ta_buf + i);

				    msg_outtrans_len(ta_buf + i, l);
				    i += l - 1;
				}
				else
				    msg_outtrans_len(ta_buf + i, 1);
			    }
			    windgoto(msg_row, msg_col);
			    out_flush();
			}

			ta_len += len;

			/*
			 * Write the characters to the child, unless EOF has
			 * been typed for pipes.  Write one character at a
			 * time, to avoid losing too much typeahead.
			 * When writing buffer lines, drop the typed
			 * characters (only check for CTRL-C).
			 */
			if (options & SHELL_WRITE)
			    ta_len = 0;
			else if (toshell_fd >= 0)
			{
			    len = write(toshell_fd, (char *)ta_buf, (size_t)1);
			    if (len > 0)
			    {
				ta_len -= len;
				mch_memmove(ta_buf, ta_buf + len, ta_len);
			    }
			}
		      }
		    }

		    if (got_int)
		    {
			// CTRL-C sends a signal to the child, we ignore it
			// ourselves
#  ifdef HAVE_SETSID
			kill(-pid, SIGINT);
#  else
			kill(0, SIGINT);
#  endif
			if (wpid > 0)
			    kill(wpid, SIGINT);
			got_int = FALSE;
		    }

		    /*
		     * Check if the child has any characters to be printed.
		     * Read them and write them to our window.	Repeat this as
		     * long as there is something to do, avoid the 10ms wait
		     * for mch_inchar(), or sending typeahead characters to
		     * the external process.
		     * TODO: This should handle escape sequences, compatible
		     * to some terminal (vt52?).
		     */
		    ++noread_cnt;
		    while (RealWaitForChar(fromshell_fd, 10L, NULL, NULL))
		    {
			len = read_eintr(fromshell_fd, buffer
				+ buffer_off, (size_t)(BUFLEN - buffer_off)
				);
			if (len <= 0)		    // end of file or error
			    goto finished;

			noread_cnt = 0;
			if (options & SHELL_READ)
			{
			    // Do NUL -> NL translation, append NL separated
			    // lines to the current buffer.
			    for (i = 0; i < len; ++i)
			    {
				if (buffer[i] == NL)
				    append_ga_line(&ga);
				else if (buffer[i] == NUL)
				    ga_append(&ga, NL);
				else
				    ga_append(&ga, buffer[i]);
			    }
			}
			else if (has_mbyte)
			{
			    int		l;
			    char_u	*p;

			    len += buffer_off;
			    buffer[len] = NUL;

			    // Check if the last character in buffer[] is
			    // incomplete, keep these bytes for the next
			    // round.
			    for (p = buffer; p < buffer + len; p += l)
			    {
				l = MB_CPTR2LEN(p);
				if (l == 0)
				    l = 1;  // NUL byte?
				else if (MB_BYTE2LEN(*p) != l)
				    break;
			    }
			    if (p == buffer)	// no complete character
			    {
				// avoid getting stuck at an illegal byte
				if (len >= 12)
				    ++p;
				else
				{
				    buffer_off = len;
				    continue;
				}
			    }
			    c = *p;
			    *p = NUL;
			    msg_puts((char *)buffer);
			    if (p < buffer + len)
			    {
				*p = c;
				buffer_off = (buffer + len) - p;
				mch_memmove(buffer, p, buffer_off);
				continue;
			    }
			    buffer_off = 0;
			}
			else
			{
			    buffer[len] = NUL;
			    msg_puts((char *)buffer);
			}

			windgoto(msg_row, msg_col);
			cursor_on();
			out_flush();
			if (got_int)
			    break;

# ifdef ELAPSED_FUNC
			if (wait_pid == 0)
			{
			    long	msec = ELAPSED_FUNC(start_tv);

			    // Avoid that we keep looping here without
			    // checking for a CTRL-C for a long time.  Don't
			    // break out too often to avoid losing typeahead.
			    if (msec > 2000)
			    {
				noread_cnt = 5;
				break;
			    }
			}
# endif
		    }

		    // If we already detected the child has finished, continue
		    // reading output for a short while.  Some text may be
		    // buffered.
		    if (wait_pid == pid)
		    {
			if (noread_cnt < 5)
			    continue;
			break;
		    }

		    /*
		     * Check if the child still exists, before checking for
		     * typed characters (otherwise we would lose typeahead).
		     */
# ifdef __NeXT__
		    wait_pid = wait4(pid, &status, WNOHANG, (struct rusage *)0);
# else
		    wait_pid = waitpid(pid, &status, WNOHANG);
# endif
		    if ((wait_pid == (pid_t)-1 && errno == ECHILD)
			    || (wait_pid == pid && WIFEXITED(status)))
		    {
			// Don't break the loop yet, try reading more
			// characters from "fromshell_fd" first.  When using
			// pipes there might still be something to read and
			// then we'll break the loop at the "break" above.
			wait_pid = pid;
		    }
		    else
			wait_pid = 0;

# if defined(FEAT_XCLIPBOARD) && defined(FEAT_X11)
		    // Handle any X events, e.g. serving the clipboard.
		    clip_update();
# endif
		}
finished:
		p_more = p_more_save;
		if (options & SHELL_READ)
		{
		    if (ga.ga_len > 0)
		    {
			append_ga_line(&ga);
			// remember that the NL was missing
			curbuf->b_no_eol_lnum = curwin->w_cursor.lnum;
		    }
		    else
			curbuf->b_no_eol_lnum = 0;
		    ga_clear(&ga);
		}

		/*
		 * Give all typeahead that wasn't used back to ui_inchar().
		 */
		if (ta_len)
		    ui_inchar_undo(ta_buf, ta_len);
		State = old_State;
		if (toshell_fd >= 0)
		    close(toshell_fd);
		close(fromshell_fd);
	    }
# if defined(FEAT_XCLIPBOARD) && defined(FEAT_X11)
	    else
	    {
		long delay_msec = 1;

		if (tmode == TMODE_RAW)
		    // Possibly disables modifyOtherKeys, so that the system
		    // can recognize CTRL-C.
		    out_str_t_TE();

		/*
		 * Similar to the loop above, but only handle X events, no
		 * I/O.
		 */
		for (;;)
		{
		    if (got_int)
		    {
			// CTRL-C sends a signal to the child, we ignore it
			// ourselves
#  ifdef HAVE_SETSID
			kill(-pid, SIGINT);
#  else
			kill(0, SIGINT);
#  endif
			got_int = FALSE;
		    }
# ifdef __NeXT__
		    wait_pid = wait4(pid, &status, WNOHANG, (struct rusage *)0);
# else
		    wait_pid = waitpid(pid, &status, WNOHANG);
# endif
		    if ((wait_pid == (pid_t)-1 && errno == ECHILD)
			    || (wait_pid == pid && WIFEXITED(status)))
		    {
			wait_pid = pid;
			break;
		    }

		    // Handle any X events, e.g. serving the clipboard.
		    clip_update();

		    // Wait for 1 to 10 msec. 1 is faster but gives the child
		    // less time, gradually wait longer.
		    mch_delay(delay_msec,
				   MCH_DELAY_IGNOREINPUT | MCH_DELAY_SETTMODE);
		    if (++delay_msec > 10)
			delay_msec = 10;
		}

		if (tmode == TMODE_RAW)
		    // possibly enables modifyOtherKeys again
		    out_str_t_TI();
	    }
# endif

	    /*
	     * Wait until our child has exited.
	     * Ignore wait() returning pids of other children and returning
	     * because of some signal like SIGWINCH.
	     * Don't wait if wait_pid was already set above, indicating the
	     * child already exited.
	     */
	    if (wait_pid != pid)
		(void)wait4pid(pid, &status);

# ifdef FEAT_GUI
	    // Close slave side of pty.  Only do this after the child has
	    // exited, otherwise the child may hang when it tries to write on
	    // the pty.
	    if (pty_master_fd >= 0)
		close(pty_slave_fd);
# endif

	    // Make sure the child that writes to the external program is
	    // dead.
	    if (wpid > 0)
	    {
		kill(wpid, SIGKILL);
		wait4pid(wpid, NULL);
	    }

# ifdef FEAT_JOB_CHANNEL
	    --dont_check_job_ended;
# endif

	    /*
	     * Set to raw mode right now, otherwise a CTRL-C after
	     * catch_signals() will kill Vim.
	     */
	    if (tmode == TMODE_RAW)
		settmode(TMODE_RAW);
	    did_settmode = TRUE;
	    set_signals();

	    if (WIFEXITED(status))
	    {
		// LINTED avoid "bitwise operation on signed value"
		retval = WEXITSTATUS(status);
		if (retval != 0 && !emsg_silent)
		{
		    if (retval == EXEC_FAILED)
		    {
			msg_puts(_("\nCannot execute shell "));
			msg_outtrans(p_sh);
			msg_putchar('\n');
		    }
		    else if (!(options & SHELL_SILENT))
		    {
			msg_puts(_("\nshell returned "));
			msg_outnum((long)retval);
			msg_putchar('\n');
		    }
		}
	    }
	    else
		msg_puts(_("\nCommand terminated\n"));
	}
    }

error:
    if (!did_settmode)
	if (tmode == TMODE_RAW)
	    settmode(TMODE_RAW);	// set to raw mode
    resettitle();
    vim_free(argv);
    vim_free(tofree1);
    vim_free(tofree2);

    return retval;
}
#endif // USE_SYSTEM

    int
mch_call_shell(
    char_u	*cmd,
    int		options)	// SHELL_*, see vim.h
{
#ifdef FEAT_EVAL
    ch_log(NULL, "executing shell command: %s", cmd);
#endif
#if defined(FEAT_GUI) && defined(FEAT_TERMINAL)
    if (gui.in_use && vim_strchr(p_go, GO_TERMINAL) != NULL
					      && (options & SHELL_SILENT) == 0)
	return mch_call_shell_terminal(cmd, options);
#endif
#ifdef USE_SYSTEM
    return mch_call_shell_system(cmd, options);
#else
    return mch_call_shell_fork(cmd, options);
#endif
}

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
    void
mch_job_start(char **argv, job_T *job, jobopt_T *options, int is_terminal)
{
    pid_t	pid;
    int		fd_in[2] = {-1, -1};	// for stdin
    int		fd_out[2] = {-1, -1};	// for stdout
    int		fd_err[2] = {-1, -1};	// for stderr
    int		pty_master_fd = -1;
    int		pty_slave_fd = -1;
    channel_T	*channel = NULL;
    int		use_null_for_in = options->jo_io[PART_IN] == JIO_NULL;
    int		use_null_for_out = options->jo_io[PART_OUT] == JIO_NULL;
    int		use_null_for_err = options->jo_io[PART_ERR] == JIO_NULL;
    int		use_file_for_in = options->jo_io[PART_IN] == JIO_FILE;
    int		use_file_for_out = options->jo_io[PART_OUT] == JIO_FILE;
    int		use_file_for_err = options->jo_io[PART_ERR] == JIO_FILE;
    int		use_buffer_for_in = options->jo_io[PART_IN] == JIO_BUFFER;
    int		use_out_for_err = options->jo_io[PART_ERR] == JIO_OUT;
    SIGSET_DECL(curset)

    if (use_out_for_err && use_null_for_out)
	use_null_for_err = TRUE;

    // default is to fail
    job->jv_status = JOB_FAILED;

    if (options->jo_pty
	    && (!(use_file_for_in || use_null_for_in)
		|| !(use_file_for_out || use_null_for_out)
		|| !(use_out_for_err || use_file_for_err || use_null_for_err)))
	open_pty(&pty_master_fd, &pty_slave_fd,
					    &job->jv_tty_out, &job->jv_tty_in);

    // TODO: without the channel feature connect the child to /dev/null?
    // Open pipes for stdin, stdout, stderr.
    if (use_file_for_in)
    {
	char_u *fname = options->jo_io_name[PART_IN];

	fd_in[0] = mch_open((char *)fname, O_RDONLY, 0);
	if (fd_in[0] < 0)
	{
	    semsg(_(e_cant_open_file_str), fname);
	    goto failed;
	}
    }
    else
	// When writing buffer lines to the input don't use the pty, so that
	// the pipe can be closed when all lines were written.
	if (!use_null_for_in && (pty_master_fd < 0 || use_buffer_for_in)
							    && pipe(fd_in) < 0)
	    goto failed;

    if (use_file_for_out)
    {
	char_u *fname = options->jo_io_name[PART_OUT];

	fd_out[1] = mch_open((char *)fname, O_WRONLY | O_CREAT | O_TRUNC, 0644);
	if (fd_out[1] < 0)
	{
	    semsg(_(e_cant_open_file_str), fname);
	    goto failed;
	}
    }
    else if (!use_null_for_out && pty_master_fd < 0 && pipe(fd_out) < 0)
	goto failed;

    if (use_file_for_err)
    {
	char_u *fname = options->jo_io_name[PART_ERR];

	fd_err[1] = mch_open((char *)fname, O_WRONLY | O_CREAT | O_TRUNC, 0600);
	if (fd_err[1] < 0)
	{
	    semsg(_(e_cant_open_file_str), fname);
	    goto failed;
	}
    }
    else if (!use_out_for_err && !use_null_for_err
				      && pty_master_fd < 0 && pipe(fd_err) < 0)
	goto failed;

    if (!use_null_for_in || !use_null_for_out || !use_null_for_err)
    {
	if (options->jo_set & JO_CHANNEL)
	{
	    channel = options->jo_channel;
	    if (channel != NULL)
		++channel->ch_refcount;
	}
	else
	    channel = add_channel();
	if (channel == NULL)
	    goto failed;
	if (job->jv_tty_out != NULL)
	    ch_log(channel, "using pty %s on fd %d",
					       job->jv_tty_out, pty_master_fd);
    }

    BLOCK_SIGNALS(&curset);
    pid = fork();	// maybe we should use vfork()
    if (pid == -1)
    {
	// failed to fork
	UNBLOCK_SIGNALS(&curset);
	goto failed;
    }
    if (pid == 0)
    {
	int	null_fd = -1;
	int	stderr_works = TRUE;

	// child
	reset_signals();		// handle signals normally
	UNBLOCK_SIGNALS(&curset);

# ifdef FEAT_EVAL
	if (ch_log_active())
	    // close the log file in the child
	    ch_logfile((char_u *)"", (char_u *)"");
# endif

# ifdef HAVE_SETSID
	// Create our own process group, so that the child and all its
	// children can be kill()ed.  Don't do this when using pipes,
	// because stdin is not a tty, we would lose /dev/tty.
	(void)setsid();
# endif

# ifdef FEAT_TERMINAL
	if (options->jo_term_rows > 0)
	{
	    char *term = (char *)T_NAME;

#ifdef FEAT_GUI
	    if (term_is_gui(T_NAME))
		// In the GUI 'term' is not what we want, use $TERM.
		term = getenv("TERM");
#endif
	    // Use 'term' or $TERM if it starts with "xterm", otherwise fall
	    // back to "xterm" or "xterm-color".
	    if (term == NULL || *term == NUL || STRNCMP(term, "xterm", 5) != 0)
	    {
		if (t_colors >= 256)
		    // TODO: should we check this name is supported?
		    term = "xterm-256color";
		else if (t_colors > 16)
		    term = "xterm-color";
		else
		    term = "xterm";
	    }
	    set_child_environment(
		    (long)options->jo_term_rows,
		    (long)options->jo_term_cols,
		    term,
		    is_terminal);
	}
	else
# endif
	    set_default_child_environment(is_terminal);

	if (options->jo_env != NULL)
	{
	    dict_T	*dict = options->jo_env;
	    hashitem_T	*hi;
	    int		todo = (int)dict->dv_hashtab.ht_used;

	    FOR_ALL_HASHTAB_ITEMS(&dict->dv_hashtab, hi, todo)
		if (!HASHITEM_EMPTY(hi))
		{
		    typval_T *item = &dict_lookup(hi)->di_tv;

		    vim_setenv(hi->hi_key, tv_get_string(item));
		    --todo;
		}
	}

	if (use_null_for_in || use_null_for_out || use_null_for_err)
	{
	    null_fd = open("/dev/null", O_RDWR | O_EXTRA, 0);
	    if (null_fd < 0)
	    {
		perror("opening /dev/null failed");
		_exit(OPEN_NULL_FAILED);
	    }
	}

	if (pty_slave_fd >= 0)
	{
	    // push stream discipline modules
	    setup_slavepty(pty_slave_fd);
#  ifdef TIOCSCTTY
	    // Try to become controlling tty (probably doesn't work,
	    // unless run by root)
	    ioctl(pty_slave_fd, TIOCSCTTY, (char *)NULL);
#  endif
	}

	// set up stdin for the child
	close(0);
	if (use_null_for_in && null_fd >= 0)
	    vim_ignored = dup(null_fd);
	else if (fd_in[0] < 0)
	    vim_ignored = dup(pty_slave_fd);
	else
	    vim_ignored = dup(fd_in[0]);

	// set up stderr for the child
	close(2);
	if (use_null_for_err && null_fd >= 0)
	{
	    vim_ignored = dup(null_fd);
	    stderr_works = FALSE;
	}
	else if (use_out_for_err)
	    vim_ignored = dup(fd_out[1]);
	else if (fd_err[1] < 0)
	    vim_ignored = dup(pty_slave_fd);
	else
	    vim_ignored = dup(fd_err[1]);

	// set up stdout for the child
	close(1);
	if (use_null_for_out && null_fd >= 0)
	    vim_ignored = dup(null_fd);
	else if (fd_out[1] < 0)
	    vim_ignored = dup(pty_slave_fd);
	else
	    vim_ignored = dup(fd_out[1]);

	if (fd_in[0] >= 0)
	    close(fd_in[0]);
	if (fd_in[1] >= 0)
	    close(fd_in[1]);
	if (fd_out[0] >= 0)
	    close(fd_out[0]);
	if (fd_out[1] >= 0)
	    close(fd_out[1]);
	if (fd_err[0] >= 0)
	    close(fd_err[0]);
	if (fd_err[1] >= 0)
	    close(fd_err[1]);
	if (pty_master_fd >= 0)
	{
	    close(pty_master_fd); // not used in the child
	    close(pty_slave_fd);  // was duped above
	}

	if (null_fd >= 0)
	    close(null_fd);

	if (options->jo_cwd != NULL && mch_chdir((char *)options->jo_cwd) != 0)
	    _exit(EXEC_FAILED);

	// See above for type of argv.
	execvp(argv[0], argv);

	if (stderr_works)
	    perror("executing job failed");
# ifdef EXITFREE
	// calling free_all_mem() here causes problems. Ignore valgrind
	// reporting possibly leaked memory.
# endif
	_exit(EXEC_FAILED);	    // exec failed, return failure code
    }

    // parent
    UNBLOCK_SIGNALS(&curset);

    job->jv_pid = pid;
    job->jv_status = JOB_STARTED;
    job->jv_channel = channel;  // ch_refcount was set above

    if (pty_master_fd >= 0)
	close(pty_slave_fd); // not used in the parent
    // close child stdin, stdout and stderr
    if (fd_in[0] >= 0)
	close(fd_in[0]);
    if (fd_out[1] >= 0)
	close(fd_out[1]);
    if (fd_err[1] >= 0)
	close(fd_err[1]);
    if (channel != NULL)
    {
	int in_fd = INVALID_FD;
	int out_fd = INVALID_FD;
	int err_fd = INVALID_FD;

	if (!(use_file_for_in || use_null_for_in))
	    in_fd = fd_in[1] >= 0 ? fd_in[1] : pty_master_fd;

	if (!(use_file_for_out || use_null_for_out))
	    out_fd = fd_out[0] >= 0 ? fd_out[0] : pty_master_fd;

	// When using pty_master_fd only set it for stdout, do not duplicate
	// it for stderr, it only needs to be read once.
	if (!(use_out_for_err || use_file_for_err || use_null_for_err))
	{
	    if (fd_err[0] >= 0)
		err_fd = fd_err[0];
	    else if (out_fd != pty_master_fd)
		err_fd = pty_master_fd;
	}

	channel_set_pipes(channel, in_fd, out_fd, err_fd);
	channel_set_job(channel, job, options);
    }
    else
    {
	if (fd_in[1] >= 0)
	    close(fd_in[1]);
	if (fd_out[0] >= 0)
	    close(fd_out[0]);
	if (fd_err[0] >= 0)
	    close(fd_err[0]);
	if (pty_master_fd >= 0)
	    close(pty_master_fd);
    }

    // success!
    return;

failed:
    channel_unref(channel);
    if (fd_in[0] >= 0)
	close(fd_in[0]);
    if (fd_in[1] >= 0)
	close(fd_in[1]);
    if (fd_out[0] >= 0)
	close(fd_out[0]);
    if (fd_out[1] >= 0)
	close(fd_out[1]);
    if (fd_err[0] >= 0)
	close(fd_err[0]);
    if (fd_err[1] >= 0)
	close(fd_err[1]);
    if (pty_master_fd >= 0)
	close(pty_master_fd);
    if (pty_slave_fd >= 0)
	close(pty_slave_fd);
}

    static char_u *
get_signal_name(int sig)
{
    int		i;
    char_u	numbuf[NUMBUFLEN];

    if (sig == SIGKILL)
	return vim_strsave((char_u *)"kill");

    for (i = 0; signal_info[i].sig != -1; i++)
	if (sig == signal_info[i].sig)
	    return strlow_save((char_u *)signal_info[i].name);

    vim_snprintf((char *)numbuf, NUMBUFLEN, "%d", sig);
    return vim_strsave(numbuf);
}

    char *
mch_job_status(job_T *job)
{
# ifdef HAVE_UNION_WAIT
    union wait	status;
# else
    int		status = -1;
# endif
    pid_t	wait_pid = 0;

# ifdef __NeXT__
    wait_pid = wait4(job->jv_pid, &status, WNOHANG, (struct rusage *)0);
# else
    wait_pid = waitpid(job->jv_pid, &status, WNOHANG);
# endif
    if (wait_pid == -1)
    {
	int waitpid_errno = errno;
	if (waitpid_errno == ECHILD && mch_process_running(job->jv_pid))
	    // The process is alive, but it was probably reparented (for
	    // example by ptrace called by a debugger like lldb or gdb).
	    // Note: This assumes that process IDs are not reused.
	    return "run";

	// process must have exited
	if (job->jv_status < JOB_ENDED)
	    ch_log(job->jv_channel, "Job no longer exists: %s",
						      strerror(waitpid_errno));
	goto return_dead;
    }
    if (wait_pid == 0)
	return "run";
    if (WIFEXITED(status))
    {
	// LINTED avoid "bitwise operation on signed value"
	job->jv_exitval = WEXITSTATUS(status);
	if (job->jv_status < JOB_ENDED)
	    ch_log(job->jv_channel, "Job exited with %d", job->jv_exitval);
	goto return_dead;
    }
    if (WIFSIGNALED(status))
    {
	job->jv_exitval = -1;
	job->jv_termsig = get_signal_name(WTERMSIG(status));
	if (job->jv_status < JOB_ENDED && job->jv_termsig != NULL)
	    ch_log(job->jv_channel, "Job terminated by signal \"%s\"",
							      job->jv_termsig);
	goto return_dead;
    }
    return "run";

return_dead:
    if (job->jv_status < JOB_ENDED)
	job->jv_status = JOB_ENDED;
    return "dead";
}

    job_T *
mch_detect_ended_job(job_T *job_list)
{
# ifdef HAVE_UNION_WAIT
    union wait	status;
# else
    int		status = -1;
# endif
    pid_t	wait_pid = 0;
    job_T	*job;

# ifndef USE_SYSTEM
    // Do not do this when waiting for a shell command to finish, we would get
    // the exit value here (and discard it), the exit value obtained there
    // would then be wrong.
    if (dont_check_job_ended > 0)
	return NULL;
# endif

# ifdef __NeXT__
    wait_pid = wait4(-1, &status, WNOHANG, (struct rusage *)0);
# else
    wait_pid = waitpid(-1, &status, WNOHANG);
# endif
    if (wait_pid <= 0)
	// no process ended
	return NULL;
    for (job = job_list; job != NULL; job = job->jv_next)
    {
	if (job->jv_pid == wait_pid)
	{
	    if (WIFEXITED(status))
		// LINTED avoid "bitwise operation on signed value"
		job->jv_exitval = WEXITSTATUS(status);
	    else if (WIFSIGNALED(status))
	    {
		job->jv_exitval = -1;
		job->jv_termsig = get_signal_name(WTERMSIG(status));
	    }
	    if (job->jv_status < JOB_ENDED)
	    {
		ch_log(job->jv_channel, "Job ended");
		job->jv_status = JOB_ENDED;
	    }
	    return job;
	}
    }
    return NULL;
}

/*
 * Send a (deadly) signal to "job".
 * Return FAIL if "how" is not a valid name.
 */
    int
mch_signal_job(job_T *job, char_u *how)
{
    int	    sig = -1;

    if (*how == NUL || STRCMP(how, "term") == 0)
	sig = SIGTERM;
    else if (STRCMP(how, "hup") == 0)
	sig = SIGHUP;
    else if (STRCMP(how, "quit") == 0)
	sig = SIGQUIT;
    else if (STRCMP(how, "int") == 0)
	sig = SIGINT;
    else if (STRCMP(how, "kill") == 0)
	sig = SIGKILL;
#ifdef SIGWINCH
    else if (STRCMP(how, "winch") == 0)
	sig = SIGWINCH;
#endif
    else if (SAFE_isdigit(*how))
	sig = atoi((char *)how);
    else
	return FAIL;

    // Never kill ourselves!
    if (job->jv_pid != 0)
    {
	// TODO: have an option to only kill the process, not the group?
	kill(-job->jv_pid, sig);
	kill(job->jv_pid, sig);
    }

    return OK;
}

/*
 * Clear the data related to "job".
 */
    void
mch_clear_job(job_T *job)
{
    // call waitpid because child process may become zombie
# ifdef __NeXT__
    (void)wait4(job->jv_pid, NULL, WNOHANG, (struct rusage *)0);
# else
    (void)waitpid(job->jv_pid, NULL, WNOHANG);
# endif
}
#endif

#if defined(FEAT_TERMINAL) || defined(PROTO)
    int
mch_create_pty_channel(job_T *job, jobopt_T *options)
{
    int		pty_master_fd = -1;
    int		pty_slave_fd = -1;
    channel_T	*channel;

    open_pty(&pty_master_fd, &pty_slave_fd, &job->jv_tty_out, &job->jv_tty_in);
    if (pty_master_fd < 0 || pty_slave_fd < 0)
	return FAIL;
    close(pty_slave_fd);

    channel = add_channel();
    if (channel == NULL)
    {
	close(pty_master_fd);
	return FAIL;
    }
    if (job->jv_tty_out != NULL)
	ch_log(channel, "using pty %s on fd %d",
					       job->jv_tty_out, pty_master_fd);
    job->jv_channel = channel;  // ch_refcount was set by add_channel()
    channel->ch_keep_open = TRUE;

    // Only set the pty_master_fd for stdout, do not duplicate it for stderr,
    // it only needs to be read once.
    channel_set_pipes(channel, pty_master_fd, pty_master_fd, INVALID_FD);
    channel_set_job(channel, job, options);
    return OK;
}
#endif

/*
 * Check for CTRL-C typed by reading all available characters.
 * In cooked mode we should get SIGINT, no need to check.
 */
    void
mch_breakcheck(int force)
{
    if ((mch_cur_tmode == TMODE_RAW || force)
			       && RealWaitForChar(read_cmd_fd, 0L, NULL, NULL))
	fill_input_buf(FALSE);
}

/*
 * Wait "msec" msec until a character is available from the mouse, keyboard,
 * from inbuf[].
 * "msec" == -1 will block forever.
 * Invokes timer callbacks when needed.
 * When "ignore_input" is TRUE even check for pending input when input is
 * already available.
 * "interrupted" (if not NULL) is set to TRUE when no character is available
 * but something else needs to be done.
 * Returns TRUE when a character is available.
 * When a GUI is being used, this will never get called -- webb
 */
    static int
WaitForChar(long msec, int *interrupted, int ignore_input)
{
#ifdef FEAT_TIMERS
    return ui_wait_for_chars_or_timer(
		    msec, WaitForCharOrMouse, interrupted, ignore_input) == OK;
#else
    return WaitForCharOrMouse(msec, interrupted, ignore_input);
#endif
}

/*
 * Wait "msec" msec until a character is available from the mouse or keyboard
 * or from inbuf[].
 * "msec" == -1 will block forever.
 * for "ignore_input" see WaitForCharOr().
 * "interrupted" (if not NULL) is set to TRUE when no character is available
 * but something else needs to be done.
 * When a GUI is being used, this will never get called -- webb
 */
    static int
WaitForCharOrMouse(long msec, int *interrupted, int ignore_input)
{
#ifdef FEAT_MOUSE_GPM
    int		gpm_process_wanted;
#endif
#ifdef FEAT_XCLIPBOARD
    int		rest;
#endif
    int		avail;

    if (!ignore_input && input_available())	    // something in inbuf[]
	return 1;

#if defined(FEAT_MOUSE_DEC)
    // May need to query the mouse position.
    if (WantQueryMouse)
    {
	WantQueryMouse = FALSE;
	if (!no_query_mouse_for_testing)
	    mch_write((char_u *)"\033[1'|", 5);
    }
#endif

    /*
     * For FEAT_MOUSE_GPM and FEAT_XCLIPBOARD we loop here to process mouse
     * events.  This is a bit complicated, because they might both be defined.
     */
#if defined(FEAT_MOUSE_GPM) || defined(FEAT_XCLIPBOARD)
# ifdef FEAT_XCLIPBOARD
    rest = 0;
    if (do_xterm_trace())
	rest = msec;
# endif
    do
    {
# ifdef FEAT_XCLIPBOARD
	if (rest != 0)
	{
	    msec = XT_TRACE_DELAY;
	    if (rest >= 0 && rest < XT_TRACE_DELAY)
		msec = rest;
	    if (rest >= 0)
		rest -= msec;
	}
# endif
# ifdef FEAT_SOUND_MACOSX
	// Invoke any pending sound callbacks.
	process_cfrunloop();
# endif
# ifdef FEAT_SOUND_CANBERRA
	// Invoke any pending sound callbacks.
	if (has_sound_callback_in_queue())
	    invoke_sound_callback();
# endif
# ifdef FEAT_MOUSE_GPM
	gpm_process_wanted = 0;
	avail = RealWaitForChar(read_cmd_fd, msec,
					     &gpm_process_wanted, interrupted);
	if (!avail && !gpm_process_wanted)
# else
	avail = RealWaitForChar(read_cmd_fd, msec, NULL, interrupted);
	if (!avail)
# endif
	{
	    if (!ignore_input && input_available())
		return 1;
# ifdef FEAT_XCLIPBOARD
	    if (rest == 0 || !do_xterm_trace())
# endif
		break;
	}
    }
    while (FALSE
# ifdef FEAT_MOUSE_GPM
	   || (gpm_process_wanted && mch_gpm_process() == 0)
# endif
# ifdef FEAT_XCLIPBOARD
	   || (!avail && rest != 0)
# endif
	  )
	;

#else
    avail = RealWaitForChar(read_cmd_fd, msec, NULL, interrupted);
#endif
    return avail;
}

#ifndef VMS
/*
 * Wait "msec" msec until a character is available from file descriptor "fd".
 * "msec" == 0 will check for characters once.
 * "msec" == -1 will block until a character is available.
 * When a GUI is being used, this will not be used for input -- webb
 * Or when a Linux GPM mouse event is waiting.
 * Or when a clientserver message is on the queue.
 * "interrupted" (if not NULL) is set to TRUE when no character is available
 * but something else needs to be done.
 */
    static int
RealWaitForChar(int fd, long msec, int *check_for_gpm UNUSED, int *interrupted)
{
    int		ret;
    int		result;
#if defined(FEAT_XCLIPBOARD) || defined(USE_XSMP) || defined(FEAT_MZSCHEME)
    static int	busy = FALSE;

    // May retry getting characters after an event was handled.
# define MAY_LOOP

# ifdef ELAPSED_FUNC
    // Remember at what time we started, so that we know how much longer we
    // should wait after being interrupted.
    long	start_msec = msec;
    elapsed_T	start_tv;

    if (msec > 0)
	ELAPSED_INIT(start_tv);
# endif

    // Handle being called recursively.  This may happen for the session
    // manager stuff, it may save the file, which does a breakcheck.
    if (busy)
	return 0;
#endif

#ifdef MAY_LOOP
    for (;;)
#endif
    {
#ifdef MAY_LOOP
	int		finished = TRUE; // default is to 'loop' just once
# ifdef FEAT_MZSCHEME
	int		mzquantum_used = FALSE;
# endif
#endif
#ifndef HAVE_SELECT
			// each channel may use in, out and err
	struct pollfd   fds[6 + 3 * MAX_OPEN_CHANNELS];
	int		nfd;
# ifdef FEAT_XCLIPBOARD
	int		xterm_idx = -1;
# endif
# ifdef FEAT_MOUSE_GPM
	int		gpm_idx = -1;
# endif
# ifdef USE_XSMP
	int		xsmp_idx = -1;
# endif
	int		towait = (int)msec;

# ifdef FEAT_MZSCHEME
	mzvim_check_threads();
	if (mzthreads_allowed() && p_mzq > 0 && (msec < 0 || msec > p_mzq))
	{
	    towait = (int)p_mzq;    // don't wait longer than 'mzquantum'
	    mzquantum_used = TRUE;
	}
# endif
	fds[0].fd = fd;
	fds[0].events = POLLIN;
	nfd = 1;

# ifdef FEAT_XCLIPBOARD
	may_restore_clipboard();
	if (xterm_Shell != (Widget)0)
	{
	    xterm_idx = nfd;
	    fds[nfd].fd = ConnectionNumber(xterm_dpy);
	    fds[nfd].events = POLLIN;
	    nfd++;
	}
# endif
# ifdef FEAT_MOUSE_GPM
	if (check_for_gpm != NULL && gpm_flag && gpm_fd >= 0)
	{
	    gpm_idx = nfd;
	    fds[nfd].fd = gpm_fd;
	    fds[nfd].events = POLLIN;
	    nfd++;
	}
# endif
# ifdef USE_XSMP
	if (xsmp_icefd != -1)
	{
	    xsmp_idx = nfd;
	    fds[nfd].fd = xsmp_icefd;
	    fds[nfd].events = POLLIN;
	    nfd++;
	}
# endif
#ifdef FEAT_JOB_CHANNEL
	nfd = channel_poll_setup(nfd, &fds, &towait);
#endif
	if (interrupted != NULL)
	    *interrupted = FALSE;

	ret = poll(fds, nfd, towait);

	result = ret > 0 && (fds[0].revents & POLLIN);
	if (result == 0 && interrupted != NULL && ret > 0)
	    *interrupted = TRUE;

# ifdef FEAT_MZSCHEME
	if (ret == 0 && mzquantum_used)
	    // MzThreads scheduling is required and timeout occurred
	    finished = FALSE;
# endif

# ifdef FEAT_XCLIPBOARD
	if (xterm_Shell != (Widget)0 && (fds[xterm_idx].revents & POLLIN))
	{
	    xterm_update();      // Maybe we should hand out clipboard
	    if (--ret == 0 && !input_available())
		// Try again
		finished = FALSE;
	}
# endif
# ifdef FEAT_MOUSE_GPM
	if (gpm_idx >= 0 && (fds[gpm_idx].revents & POLLIN))
	    *check_for_gpm = 1;
# endif
# ifdef USE_XSMP
	if (xsmp_idx >= 0 && (fds[xsmp_idx].revents & (POLLIN | POLLHUP)))
	{
	    if (fds[xsmp_idx].revents & POLLIN)
	    {
		busy = TRUE;
		xsmp_handle_requests();
		busy = FALSE;
	    }
	    else if (fds[xsmp_idx].revents & POLLHUP)
	    {
		if (p_verbose > 0)
		    verb_msg(_("XSMP lost ICE connection"));
		xsmp_close();
	    }
	    if (--ret == 0)
		finished = FALSE;	// Try again
	}
# endif
#ifdef FEAT_JOB_CHANNEL
	// also call when ret == 0, we may be polling a keep-open channel
	if (ret >= 0)
	    channel_poll_check(ret, &fds);
#endif

#else // HAVE_SELECT

	struct timeval  tv;
	struct timeval	*tvp;
	// These are static because they can take 8 Kbyte each and cause the
	// signal stack to run out with -O3.
	static fd_set	rfds, wfds, efds;
	int		maxfd;
	long		towait = msec;

# ifdef FEAT_MZSCHEME
	mzvim_check_threads();
	if (mzthreads_allowed() && p_mzq > 0 && (msec < 0 || msec > p_mzq))
	{
	    towait = p_mzq;	// don't wait longer than 'mzquantum'
	    mzquantum_used = TRUE;
	}
# endif

	if (towait >= 0)
	{
	    tv.tv_sec = towait / 1000;
	    tv.tv_usec = (towait % 1000) * (1000000/1000);
	    tvp = &tv;
	}
	else
	    tvp = NULL;

	/*
	 * Select on ready for reading and exceptional condition (end of file).
	 */
select_eintr:
	FD_ZERO(&rfds);
	FD_ZERO(&wfds);
	FD_ZERO(&efds);
	FD_SET(fd, &rfds);
# ifndef __QNX__
	// For QNX select() always returns 1 if this is set.  Why?
	FD_SET(fd, &efds);
# endif
	maxfd = fd;

# ifdef FEAT_XCLIPBOARD
	may_restore_clipboard();
	if (xterm_Shell != (Widget)0)
	{
	    FD_SET(ConnectionNumber(xterm_dpy), &rfds);
	    if (maxfd < ConnectionNumber(xterm_dpy))
		maxfd = ConnectionNumber(xterm_dpy);

	    // An event may have already been read but not handled.  In
	    // particularly, XFlush may cause this.
	    xterm_update();
	}
# endif
# ifdef FEAT_MOUSE_GPM
	if (check_for_gpm != NULL && gpm_flag && gpm_fd >= 0)
	{
	    FD_SET(gpm_fd, &rfds);
	    FD_SET(gpm_fd, &efds);
	    if (maxfd < gpm_fd)
		maxfd = gpm_fd;
	}
# endif
# ifdef USE_XSMP
	if (xsmp_icefd != -1)
	{
	    FD_SET(xsmp_icefd, &rfds);
	    FD_SET(xsmp_icefd, &efds);
	    if (maxfd < xsmp_icefd)
		maxfd = xsmp_icefd;
	}
# endif
# ifdef FEAT_JOB_CHANNEL
	maxfd = channel_select_setup(maxfd, &rfds, &wfds, &tv, &tvp);
# endif
	if (interrupted != NULL)
	    *interrupted = FALSE;

	ret = select(maxfd + 1, SELECT_TYPE_ARG234 &rfds,
		      SELECT_TYPE_ARG234 &wfds, SELECT_TYPE_ARG234 &efds, tvp);
	result = ret > 0 && FD_ISSET(fd, &rfds);
	if (result)
	    --ret;
	else if (interrupted != NULL && ret > 0)
	    *interrupted = TRUE;

# ifdef EINTR
	if (ret == -1 && errno == EINTR)
	{
	    // Check whether the EINTR is caused by SIGTSTP
	    if (got_tstp && !in_mch_suspend)
	    {
		exarg_T ea;

		ea.forceit = TRUE;
		ex_stop(&ea);
		got_tstp = FALSE;
	    }

	    // Check whether window has been resized, EINTR may be caused by
	    // SIGWINCH.
	    if (do_resize)
	    {
#  ifdef FEAT_EVAL
		ch_log(NULL, "calling handle_resize() in RealWaitForChar()");
#  endif
		handle_resize();
	    }

	    // Interrupted by a signal, need to try again.  We ignore msec
	    // here, because we do want to check even after a timeout if
	    // characters are available.  Needed for reading output of an
	    // external command after the process has finished.
	    goto select_eintr;
	}
# endif
# ifdef __TANDEM
	if (ret == -1 && errno == ENOTSUP)
	{
	    FD_ZERO(&rfds);
	    FD_ZERO(&efds);
	    ret = 0;
	}
# endif
# ifdef FEAT_MZSCHEME
	if (ret == 0 && mzquantum_used)
	    // loop if MzThreads must be scheduled and timeout occurred
	    finished = FALSE;
# endif

# ifdef FEAT_XCLIPBOARD
	if (ret > 0 && xterm_Shell != (Widget)0
		&& FD_ISSET(ConnectionNumber(xterm_dpy), &rfds))
	{
	    xterm_update();	      // Maybe we should hand out clipboard
	    // continue looping when we only got the X event and the input
	    // buffer is empty
	    if (--ret == 0 && !input_available())
	    {
		// Try again
		finished = FALSE;
	    }
	}
# endif
# ifdef FEAT_MOUSE_GPM
	if (ret > 0 && check_for_gpm != NULL && gpm_flag && gpm_fd >= 0)
	{
	    if (FD_ISSET(gpm_fd, &efds))
		gpm_close();
	    else if (FD_ISSET(gpm_fd, &rfds))
		*check_for_gpm = 1;
	}
# endif
# ifdef USE_XSMP
	if (ret > 0 && xsmp_icefd != -1)
	{
	    if (FD_ISSET(xsmp_icefd, &efds))
	    {
		if (p_verbose > 0)
		    verb_msg(_("XSMP lost ICE connection"));
		xsmp_close();
		if (--ret == 0)
		    finished = FALSE;   // keep going if event was only one
	    }
	    else if (FD_ISSET(xsmp_icefd, &rfds))
	    {
		busy = TRUE;
		xsmp_handle_requests();
		busy = FALSE;
		if (--ret == 0)
		    finished = FALSE;   // keep going if event was only one
	    }
	}
# endif
#ifdef FEAT_JOB_CHANNEL
	// also call when ret == 0, we may be polling a keep-open channel
	if (ret >= 0)
	    (void)channel_select_check(ret, &rfds, &wfds);
#endif

#endif // HAVE_SELECT

#ifdef MAY_LOOP
	if (finished || msec == 0)
	    break;

# ifdef FEAT_CLIENTSERVER
	if (server_waiting())
	    break;
# endif

	// We're going to loop around again, find out for how long
	if (msec > 0)
	{
# ifdef ELAPSED_FUNC
	    // Compute remaining wait time.
	    msec = start_msec - ELAPSED_FUNC(start_tv);
# else
	    // Guess we got interrupted halfway.
	    msec = msec / 2;
# endif
	    if (msec <= 0)
		break;	// waited long enough
	}
#endif
    }

    return result;
}

/*
 * Expand a path into all matching files and/or directories.  Handles "*",
 * "?", "[a-z]", "**", etc.
 * "path" has backslashes before chars that are not to be expanded.
 * Returns the number of matches found.
 */
    int
mch_expandpath(
    garray_T	*gap,
    char_u	*path,
    int		flags)		// EW_* flags
{
    return unix_expandpath(gap, path, 0, flags, FALSE);
}

/*
 * mch_expand_wildcards() - this code does wild-card pattern matching using
 * the shell
 *
 * return OK for success, FAIL for error (you may lose some memory) and put
 * an error message in *file.
 *
 * num_pat is number of input patterns
 * pat is array of pointers to input patterns
 * num_file is pointer to number of matched file names
 * file is pointer to array of pointers to matched file names
 */

#ifndef SEEK_SET
# define SEEK_SET 0
#endif
#ifndef SEEK_END
# define SEEK_END 2
#endif

#define SHELL_SPECIAL (char_u *)"\t \"&'$;<>()\\|"

    int
mch_expand_wildcards(
    int		   num_pat,
    char_u	 **pat,
    int		  *num_file,
    char_u	***file,
    int		   flags)	// EW_* flags
{
    int		i;
    size_t	len;
    long	llen;
    char_u	*p;
    int		dir;

    /*
     * This is the non-OS/2 implementation (really Unix).
     */
    int		j;
    char_u	*tempname;
    char_u	*command;
    FILE	*fd;
    char_u	*buffer;
#define STYLE_ECHO	0	// use "echo", the default
#define STYLE_GLOB	1	// use "glob", for csh
#define STYLE_VIMGLOB	2	// use "vimglob", for Posix sh
#define STYLE_PRINT	3	// use "print -N", for zsh
#define STYLE_BT	4	// `cmd` expansion, execute the pattern directly
#define STYLE_GLOBSTAR	5	// use extended shell glob for bash (this uses extended
				// globbing functionality using globstar, needs bash > 4)
    int		shell_style = STYLE_ECHO;
    int		check_spaces;
    static int	did_find_nul = FALSE;
    int		ampersand = FALSE;
		// vimglob() function to define for Posix shell
    static char *sh_vimglob_func = "vimglob() { while [ $# -ge 1 ]; do echo \"$1\"; shift; done }; vimglob >";
		// vimglob() function with globstar setting enabled, only for bash >= 4.X
    static char *sh_globstar_opt = "[[ ${BASH_VERSINFO[0]} -ge 4 ]] && shopt -s globstar; ";

    *num_file = 0;	// default: no files found
    *file = NULL;

    /*
     * If there are no wildcards, just copy the names to allocated memory.
     * Saves a lot of time, because we don't have to start a new shell.
     */
    if (!have_wildcard(num_pat, pat))
	return save_patterns(num_pat, pat, num_file, file);

# ifdef HAVE_SANDBOX
    // Don't allow any shell command in the sandbox.
    if (sandbox != 0 && check_secure())
	return FAIL;
# endif

    /*
     * Don't allow the use of backticks in secure and restricted mode.
     */
    if (secure || restricted)
	for (i = 0; i < num_pat; ++i)
	    if (vim_strchr(pat[i], '`') != NULL
		    && (check_restricted() || check_secure()))
		return FAIL;

    /*
     * get a name for the temp file
     */
    if ((tempname = vim_tempname('o', FALSE)) == NULL)
    {
	emsg(_(e_cant_get_temp_file_name));
	return FAIL;
    }

    /*
     * Let the shell expand the patterns and write the result into the temp
     * file.
     * STYLE_BT:	NL separated
     *	    If expanding `cmd` execute it directly.
     * STYLE_GLOB:	NUL separated
     *	    If we use *csh, "glob" will work better than "echo".
     * STYLE_PRINT:	NL or NUL separated
     *	    If we use *zsh, "print -N" will work better than "glob".
     * STYLE_VIMGLOB:	NL separated
     *	    If we use *sh*, we define "vimglob()".
     * STYLE_GLOBSTAR:	NL separated
     *	    If we use *bash*, we define "vimglob() and enable globstar option".
     * STYLE_ECHO:	space separated.
     *	    A shell we don't know, stay safe and use "echo".
     */
    if (num_pat == 1 && *pat[0] == '`'
	    && (len = STRLEN(pat[0])) > 2
	    && *(pat[0] + len - 1) == '`')
	shell_style = STYLE_BT;
    else if ((len = STRLEN(p_sh)) >= 3)
    {
	if (STRCMP(p_sh + len - 3, "csh") == 0)
	    shell_style = STYLE_GLOB;
	else if (STRCMP(p_sh + len - 3, "zsh") == 0)
	    shell_style = STYLE_PRINT;
    }
    if (shell_style == STYLE_ECHO)
    {
       if (strstr((char *)gettail(p_sh), "bash") != NULL)
	    shell_style = STYLE_GLOBSTAR;
       else if (strstr((char *)gettail(p_sh), "sh") != NULL)
	    shell_style = STYLE_VIMGLOB;
    }

    // Compute the length of the command.  We need 2 extra bytes: for the
    // optional '&' and for the NUL.
    // Worst case: "unset nonomatch; print -N >" plus two is 29
    len = STRLEN(tempname) + 29;
    if (shell_style == STYLE_VIMGLOB)
	len += STRLEN(sh_vimglob_func);
    else if (shell_style == STYLE_GLOBSTAR)
	len += STRLEN(sh_vimglob_func)
	     + STRLEN(sh_globstar_opt);

    for (i = 0; i < num_pat; ++i)
    {
	// Count the length of the patterns in the same way as they are put in
	// "command" below.
#ifdef USE_SYSTEM
	len += STRLEN(pat[i]) + 3;	// add space and two quotes
#else
	++len;				// add space
	for (j = 0; pat[i][j] != NUL; ++j)
	{
	    if (vim_strchr(SHELL_SPECIAL, pat[i][j]) != NULL)
		++len;		// may add a backslash
	    ++len;
	}
#endif
    }
    command = alloc(len);
    if (command == NULL)
    {
	// out of memory
	vim_free(tempname);
	return FAIL;
    }

    /*
     * Build the shell command:
     * - Set $nonomatch depending on EW_NOTFOUND (hopefully the shell
     *	 recognizes this).
     * - Add the shell command to print the expanded names.
     * - Add the temp file name.
     * - Add the file name patterns.
     */
    if (shell_style == STYLE_BT)
    {
	// change `command; command& ` to (command; command )
	STRCPY(command, "(");
	STRCAT(command, pat[0] + 1);		// exclude first backtick
	p = command + STRLEN(command) - 1;
	*p-- = ')';				// remove last backtick
	while (p > command && VIM_ISWHITE(*p))
	    --p;
	if (*p == '&')				// remove trailing '&'
	{
	    ampersand = TRUE;
	    *p = ' ';
	}
	STRCAT(command, ">");
    }
    else
    {
	STRCPY(command, "");
	if (shell_style == STYLE_GLOB)
	{
	    // Assume the nonomatch option is valid only for csh like shells,
	    // otherwise, this may set the positional parameters for the shell,
	    // e.g. "$*".
	    if (flags & EW_NOTFOUND)
		STRCAT(command, "set nonomatch; ");
	    else
		STRCAT(command, "unset nonomatch; ");
	}
	if (shell_style == STYLE_GLOB)
	    STRCAT(command, "glob >");
	else if (shell_style == STYLE_PRINT)
	    STRCAT(command, "print -N >");
	else if (shell_style == STYLE_VIMGLOB)
	    STRCAT(command, sh_vimglob_func);
	else if (shell_style == STYLE_GLOBSTAR)
	{
	    STRCAT(command, sh_globstar_opt);
	    STRCAT(command, sh_vimglob_func);
	}
	else
	    STRCAT(command, "echo >");
    }

    STRCAT(command, tempname);

    if (shell_style != STYLE_BT)
	for (i = 0; i < num_pat; ++i)
	{
	    // When using system() always add extra quotes, because the shell
	    // is started twice.  Otherwise put a backslash before special
	    // characters, except inside ``.
#ifdef USE_SYSTEM
	    STRCAT(command, " \"");
	    STRCAT(command, pat[i]);
	    STRCAT(command, "\"");
#else
	    int intick = FALSE;

	    p = command + STRLEN(command);
	    *p++ = ' ';
	    for (j = 0; pat[i][j] != NUL; ++j)
	    {
		if (pat[i][j] == '`')
		    intick = !intick;
		else if (pat[i][j] == '\\' && pat[i][j + 1] != NUL)
		{
		    // Remove a backslash, take char literally.  But keep
		    // backslash inside backticks, before a special character
		    // and before a backtick.
		    if (intick
			  || vim_strchr(SHELL_SPECIAL, pat[i][j + 1]) != NULL
			  || pat[i][j + 1] == '`')
			*p++ = '\\';
		    ++j;
		}
		else if (!intick
			 && ((flags & EW_KEEPDOLLAR) == 0 || pat[i][j] != '$')
			      && vim_strchr(SHELL_SPECIAL, pat[i][j]) != NULL)
		    // Put a backslash before a special character, but not
		    // when inside ``. And not for $var when EW_KEEPDOLLAR is
		    // set.
		    *p++ = '\\';

		// Copy one character.
		*p++ = pat[i][j];
	    }
	    *p = NUL;
#endif
	}
    if (flags & EW_SILENT)
	show_shell_mess = FALSE;
    if (ampersand)
	STRCAT(command, "&");		// put the '&' after the redirection

    /*
     * Using zsh -G: If a pattern has no matches, it is just deleted from
     * the argument list, otherwise zsh gives an error message and doesn't
     * expand any other pattern.
     */
    if (shell_style == STYLE_PRINT)
	extra_shell_arg = (char_u *)"-G";   // Use zsh NULL_GLOB option

    /*
     * If we use -f then shell variables set in .cshrc won't get expanded.
     * vi can do it, so we will too, but it is only necessary if there is a "$"
     * in one of the patterns, otherwise we can still use the fast option.
     */
    else if (shell_style == STYLE_GLOB && !have_dollars(num_pat, pat))
	extra_shell_arg = (char_u *)"-f";	// Use csh fast option

    /*
     * execute the shell command
     */
    i = call_shell(command, SHELL_EXPAND | SHELL_SILENT);

    // When running in the background, give it some time to create the temp
    // file, but don't wait for it to finish.
    if (ampersand)
	mch_delay(10L, MCH_DELAY_IGNOREINPUT);

    extra_shell_arg = NULL;		// cleanup
    show_shell_mess = TRUE;
    vim_free(command);

    if (i != 0)				// mch_call_shell() failed
    {
	mch_remove(tempname);
	vim_free(tempname);
	/*
	 * With interactive completion, the error message is not printed.
	 * However with USE_SYSTEM, I don't know how to turn off error messages
	 * from the shell, so screen may still get messed up -- webb.
	 */
#ifndef USE_SYSTEM
	if (!(flags & EW_SILENT))
#endif
	{
	    redraw_later_clear();	// probably messed up screen
	    msg_putchar('\n');		// clear bottom line quickly
	    cmdline_row = Rows - 1;	// continue on last line
#ifdef USE_SYSTEM
	    if (!(flags & EW_SILENT))
#endif
	    {
		msg(_(e_cannot_expand_wildcards));
		msg_start();		// don't overwrite this message
	    }
	}
	// If a `cmd` expansion failed, don't list `cmd` as a match, even when
	// EW_NOTFOUND is given
	if (shell_style == STYLE_BT)
	    return FAIL;
	goto notfound;
    }

    /*
     * read the names from the file into memory
     */
    fd = fopen((char *)tempname, READBIN);
    if (fd == NULL)
    {
	// Something went wrong, perhaps a file name with a special char.
	if (!(flags & EW_SILENT))
	{
	    msg(_(e_cannot_expand_wildcards));
	    msg_start();		// don't overwrite this message
	}
	vim_free(tempname);
	goto notfound;
    }
    fseek(fd, 0L, SEEK_END);
    llen = ftell(fd);			// get size of temp file
    fseek(fd, 0L, SEEK_SET);
    if (llen < 0)
	// just in case ftell() would fail
	buffer = NULL;
    else
	buffer = alloc(llen + 1);
    if (buffer == NULL)
    {
	// out of memory
	mch_remove(tempname);
	vim_free(tempname);
	fclose(fd);
	return FAIL;
    }
    len = llen;
    i = fread((char *)buffer, 1, len, fd);
    fclose(fd);
    mch_remove(tempname);
    if (i != (int)len)
    {
	// unexpected read error
	semsg(_(e_cant_read_file_str), tempname);
	vim_free(tempname);
	vim_free(buffer);
	return FAIL;
    }
    vim_free(tempname);

# ifdef __CYGWIN__
    // Translate <CR><NL> into <NL>.  Caution, buffer may contain NUL.
    p = buffer;
    for (i = 0; i < (int)len; ++i)
	if (!(buffer[i] == CAR && buffer[i + 1] == NL))
	    *p++ = buffer[i];
    len = p - buffer;
# endif


    // file names are separated with Space
    if (shell_style == STYLE_ECHO)
    {
	buffer[len] = '\n';		// make sure the buffer ends in NL
	p = buffer;
	for (i = 0; *p != '\n'; ++i)	// count number of entries
	{
	    while (*p != ' ' && *p != '\n')
		++p;
	    p = skipwhite(p);		// skip to next entry
	}
    }
    // file names are separated with NL
    else if (shell_style == STYLE_BT ||
	    shell_style == STYLE_VIMGLOB ||
	    shell_style == STYLE_GLOBSTAR)
    {
	buffer[len] = NUL;		// make sure the buffer ends in NUL
	p = buffer;
	for (i = 0; *p != NUL; ++i)	// count number of entries
	{
	    while (*p != '\n' && *p != NUL)
		++p;
	    if (*p != NUL)
		++p;
	    p = skipwhite(p);		// skip leading white space
	}
    }
    // file names are separated with NUL
    else
    {
	/*
	 * Some versions of zsh use spaces instead of NULs to separate
	 * results.  Only do this when there is no NUL before the end of the
	 * buffer, otherwise we would never be able to use file names with
	 * embedded spaces when zsh does use NULs.
	 * When we found a NUL once, we know zsh is OK, set did_find_nul and
	 * don't check for spaces again.
	 */
	check_spaces = FALSE;
	if (shell_style == STYLE_PRINT && !did_find_nul)
	{
	    // If there is a NUL, set did_find_nul, else set check_spaces
	    buffer[len] = NUL;
	    if (len && (int)STRLEN(buffer) < (int)len)
		did_find_nul = TRUE;
	    else
		check_spaces = TRUE;
	}

	/*
	 * Make sure the buffer ends with a NUL.  For STYLE_PRINT there
	 * already is one, for STYLE_GLOB it needs to be added.
	 */
	if (len && buffer[len - 1] == NUL)
	    --len;
	else
	    buffer[len] = NUL;
	i = 0;
	for (p = buffer; p < buffer + len; ++p)
	    if (*p == NUL || (*p == ' ' && check_spaces))   // count entry
	    {
		++i;
		*p = NUL;
	    }
	if (len)
	    ++i;			// count last entry
    }
    if (i == 0)
    {
	/*
	 * Can happen when using /bin/sh and typing ":e $NO_SUCH_VAR^I".
	 * /bin/sh will happily expand it to nothing rather than returning an
	 * error; and hey, it's good to check anyway -- webb.
	 */
	vim_free(buffer);
	goto notfound;
    }
    *num_file = i;
    *file = ALLOC_MULT(char_u *, i);
    if (*file == NULL)
    {
	// out of memory
	vim_free(buffer);
	return FAIL;
    }

    /*
     * Isolate the individual file names.
     */
    p = buffer;
    for (i = 0; i < *num_file; ++i)
    {
	(*file)[i] = p;
	// Space or NL separates
	if (shell_style == STYLE_ECHO || shell_style == STYLE_BT
		|| shell_style == STYLE_VIMGLOB || shell_style == STYLE_GLOBSTAR)
	{
	    while (!(shell_style == STYLE_ECHO && *p == ' ')
						   && *p != '\n' && *p != NUL)
		++p;
	    if (p == buffer + len)		// last entry
		*p = NUL;
	    else
	    {
		*p++ = NUL;
		p = skipwhite(p);		// skip to next entry
	    }
	}
	else		// NUL separates
	{
	    while (*p && p < buffer + len)	// skip entry
		++p;
	    ++p;				// skip NUL
	}
    }

    /*
     * Move the file names to allocated memory.
     */
    for (j = 0, i = 0; i < *num_file; ++i)
    {
	// Require the files to exist.	Helps when using /bin/sh
	if (!(flags & EW_NOTFOUND) && mch_getperm((*file)[i]) < 0)
	    continue;

	// check if this entry should be included
	dir = (mch_isdir((*file)[i]));
	if ((dir && !(flags & EW_DIR)) || (!dir && !(flags & EW_FILE)))
	    continue;

	// Skip files that are not executable if we check for that.
	if (!dir && (flags & EW_EXEC)
		    && !mch_can_exe((*file)[i], NULL, !(flags & EW_SHELLCMD)))
	    continue;

	p = alloc(STRLEN((*file)[i]) + 1 + dir);
	if (p)
	{
	    STRCPY(p, (*file)[i]);
	    if (dir)
		add_pathsep(p);	    // add '/' to a directory name
	    (*file)[j++] = p;
	}
    }
    vim_free(buffer);
    *num_file = j;

    if (*num_file == 0)	    // rejected all entries
    {
	VIM_CLEAR(*file);
	goto notfound;
    }

    return OK;

notfound:
    if (flags & EW_NOTFOUND)
	return save_patterns(num_pat, pat, num_file, file);
    return FAIL;
}

#endif // VMS

    static int
save_patterns(
    int		num_pat,
    char_u	**pat,
    int		*num_file,
    char_u	***file)
{
    int		i;
    char_u	*s;

    *file = ALLOC_MULT(char_u *, num_pat);
    if (*file == NULL)
	return FAIL;
    for (i = 0; i < num_pat; i++)
    {
	s = vim_strsave(pat[i]);
	if (s != NULL)
	    // Be compatible with expand_filename(): halve the number of
	    // backslashes.
	    backslash_halve(s);
	(*file)[i] = s;
    }
    *num_file = num_pat;
    return OK;
}

/*
 * Return TRUE if the string "p" contains a wildcard that mch_expandpath() can
 * expand.
 */
    int
mch_has_exp_wildcard(char_u *p)
{
    for ( ; *p; MB_PTR_ADV(p))
    {
	if (*p == '\\' && p[1] != NUL)
	    ++p;
	else
	    if (vim_strchr((char_u *)
#ifdef VMS
				    "*?%"
#else
				    "*?[{'"
#endif
						, *p) != NULL)
	    return TRUE;
    }
    return FALSE;
}

/*
 * Return TRUE if the string "p" contains a wildcard.
 * Don't recognize '~' at the end as a wildcard.
 */
    int
mch_has_wildcard(char_u *p)
{
    for ( ; *p; MB_PTR_ADV(p))
    {
	if (*p == '\\' && p[1] != NUL)
	    ++p;
	else
	    if (vim_strchr((char_u *)
#ifdef VMS
				    "*?%$"
#else
				    "*?[{`'$"
#endif
						, *p) != NULL
		|| (*p == '~' && p[1] != NUL))
	    return TRUE;
    }
    return FALSE;
}

    static int
have_wildcard(int num, char_u **file)
{
    int	    i;

    for (i = 0; i < num; i++)
	if (mch_has_wildcard(file[i]))
	    return 1;
    return 0;
}

    static int
have_dollars(int num, char_u **file)
{
    int	    i;

    for (i = 0; i < num; i++)
	if (vim_strchr(file[i], '$') != NULL)
	    return TRUE;
    return FALSE;
}

#if !defined(HAVE_RENAME) || defined(PROTO)
/*
 * Scaled-down version of rename(), which is missing in Xenix.
 * This version can only move regular files and will fail if the
 * destination exists.
 */
    int
mch_rename(const char *src, const char *dest)
{
    struct stat	    st;

    if (stat(dest, &st) >= 0)	    // fail if destination exists
	return -1;
    if (link(src, dest) != 0)	    // link file to new name
	return -1;
    if (mch_remove(src) == 0)	    // delete link to old name
	return 0;
    return -1;
}
#endif // !HAVE_RENAME

#if defined(FEAT_MOUSE_GPM) || defined(PROTO)
# if defined(DYNAMIC_GPM) || defined(PROTO)
/*
 * Initialize Gpm's symbols for dynamic linking.
 * Must be called only if libgpm_hinst is NULL.
 */
    static int
load_libgpm(void)
{
    libgpm_hinst = dlopen("libgpm.so", RTLD_LAZY|RTLD_GLOBAL);

    if (libgpm_hinst == NULL)
    {
	if (p_verbose > 0)
	    smsg_attr(HL_ATTR(HLF_W),
			       _("Could not load gpm library: %s"), dlerror());
	return FAIL;
    }

    if (
	    (dll_Gpm_Open     = dlsym(libgpm_hinst, "Gpm_Open"))     == NULL
	||  (dll_Gpm_Close    = dlsym(libgpm_hinst, "Gpm_Close"))    == NULL
	||  (dll_Gpm_GetEvent = dlsym(libgpm_hinst, "Gpm_GetEvent")) == NULL
	||  (dll_gpm_flag     = dlsym(libgpm_hinst, "gpm_flag"))     == NULL
	||  (dll_gpm_fd       = dlsym(libgpm_hinst, "gpm_fd"))       == NULL
      )
    {
	semsg(_(e_could_not_load_library_str_str), "gpm", dlerror());
	dlclose(libgpm_hinst);
	libgpm_hinst = NULL;
	dll_gpm_flag = NULL;
	dll_gpm_fd   = NULL;
	return FAIL;
    }
    return OK;
}

    int
gpm_available(void)
{
    return libgpm_hinst != NULL || load_libgpm() == OK;
}
# endif // DYNAMIC_GPM

/*
 * Initializes connection with gpm (if it isn't already opened)
 * Return 1 if succeeded (or connection already opened), 0 if failed
 */
    static int
gpm_open(void)
{
    static Gpm_Connect gpm_connect; // Must it be kept till closing ?

#ifdef DYNAMIC_GPM
    if (!gpm_available())
	return 0;
#endif

    if (gpm_flag)
	return 1; // already open

    gpm_connect.eventMask = (GPM_UP | GPM_DRAG | GPM_DOWN);
    gpm_connect.defaultMask = ~GPM_HARD;
    // Default handling for mouse move
    gpm_connect.minMod = 0; // Handle any modifier keys
    gpm_connect.maxMod = 0xffff;
    if (Gpm_Open(&gpm_connect, 0) > 0)
    {
	// gpm library tries to handling TSTP causes
	// problems. Anyways, we close connection to Gpm whenever
	// we are going to suspend or starting an external process
	// so we shouldn't  have problem with this
# ifdef SIGTSTP
	mch_signal(SIGTSTP, restricted ? SIG_IGN : sig_tstp);
# endif
	return 1; // succeed
    }
    if (gpm_fd == -2)
	Gpm_Close(); // We don't want to talk to xterm via gpm
    return 0;
}

/*
 * Returns TRUE if the GPM mouse is enabled.
 */
    int
gpm_enabled(void)
{
    return gpm_flag && gpm_fd >= 0;
}

/*
 * Closes connection to gpm
 */
    static void
gpm_close(void)
{
    if (gpm_enabled())
	Gpm_Close();
}

/*
 * Reads gpm event and adds special keys to input buf. Returns length of
 * generated key sequence.
 * This function is styled after gui_send_mouse_event().
 */
    static int
mch_gpm_process(void)
{
    int			button;
    static Gpm_Event	gpm_event;
    char_u		string[6];
    int_u		vim_modifiers;
    int			row,col;
    unsigned char	buttons_mask;
    unsigned char	gpm_modifiers;
    static unsigned char old_buttons = 0;

    Gpm_GetEvent(&gpm_event);

#ifdef FEAT_GUI
    // Don't put events in the input queue now.
    if (hold_gui_events)
	return 0;
#endif

    row = gpm_event.y - 1;
    col = gpm_event.x - 1;

    string[0] = ESC; // Our termcode
    string[1] = 'M';
    string[2] = 'G';
    switch (GPM_BARE_EVENTS(gpm_event.type))
    {
	case GPM_DRAG:
	    string[3] = MOUSE_DRAG;
	    break;
	case GPM_DOWN:
	    buttons_mask = gpm_event.buttons & ~old_buttons;
	    old_buttons = gpm_event.buttons;
	    switch (buttons_mask)
	    {
		case GPM_B_LEFT:
		    button = MOUSE_LEFT;
		    break;
		case GPM_B_MIDDLE:
		    button = MOUSE_MIDDLE;
		    break;
		case GPM_B_RIGHT:
		    button = MOUSE_RIGHT;
		    break;
		default:
		    return 0;
		    // Don't know what to do. Can more than one button be
		    // reported in one event?
	    }
	    string[3] = (char_u)(button | 0x20);
	    SET_NUM_MOUSE_CLICKS(string[3], gpm_event.clicks + 1);
	    break;
	case GPM_UP:
	    string[3] = MOUSE_RELEASE;
	    old_buttons &= ~gpm_event.buttons;
	    break;
	default:
	    return 0;
    }
    // This code is based on gui_x11_mouse_cb in gui_x11.c
    gpm_modifiers = gpm_event.modifiers;
    vim_modifiers = 0x0;
    // I ignore capslock stats. Aren't we all just hate capslock mixing with
    // Vim commands ? Besides, gpm_event.modifiers is unsigned char, and
    // K_CAPSSHIFT is defined 8, so it probably isn't even reported
    if (gpm_modifiers & ((1 << KG_SHIFT) | (1 << KG_SHIFTR) | (1 << KG_SHIFTL)))
	vim_modifiers |= MOUSE_SHIFT;

    if (gpm_modifiers & ((1 << KG_CTRL) | (1 << KG_CTRLR) | (1 << KG_CTRLL)))
	vim_modifiers |= MOUSE_CTRL;
    if (gpm_modifiers & ((1 << KG_ALT) | (1 << KG_ALTGR)))
	vim_modifiers |= MOUSE_ALT;
    string[3] |= vim_modifiers;
    string[4] = (char_u)(col + ' ' + 1);
    string[5] = (char_u)(row + ' ' + 1);
    add_to_input_buf(string, 6);
    return 6;
}
#endif // FEAT_MOUSE_GPM

#ifdef FEAT_SYSMOUSE
/*
 * Initialize connection with sysmouse.
 * Let virtual console inform us with SIGUSR2 for pending sysmouse
 * output, any sysmouse output than will be processed via sig_sysmouse().
 * Return OK if succeeded, FAIL if failed.
 */
    static int
sysmouse_open(void)
{
    struct mouse_info   mouse;

    mouse.operation = MOUSE_MODE;
    mouse.u.mode.mode = 0;
    mouse.u.mode.signal = SIGUSR2;
    if (ioctl(1, CONS_MOUSECTL, &mouse) == -1)
	return FAIL;

    mch_signal(SIGUSR2, sig_sysmouse);
    mouse.operation = MOUSE_SHOW;
    ioctl(1, CONS_MOUSECTL, &mouse);
    return OK;
}

/*
 * Stop processing SIGUSR2 signals, and also make sure that
 * virtual console do not send us any sysmouse related signal.
 */
    static void
sysmouse_close(void)
{
    struct mouse_info	mouse;

    mch_signal(SIGUSR2, restricted ? SIG_IGN : SIG_DFL);
    mouse.operation = MOUSE_MODE;
    mouse.u.mode.mode = 0;
    mouse.u.mode.signal = 0;
    ioctl(1, CONS_MOUSECTL, &mouse);
}

/*
 * Gets info from sysmouse and adds special keys to input buf.
 */
    static void
sig_sysmouse SIGDEFARG(sigarg)
{
    struct mouse_info	mouse;
    struct video_info	video;
    char_u		string[6];
    int			row, col;
    int			button;
    int			buttons;
    static int		oldbuttons = 0;

#ifdef FEAT_GUI
    // Don't put events in the input queue now.
    if (hold_gui_events)
	return;
#endif

    mouse.operation = MOUSE_GETINFO;
    if (ioctl(1, FBIO_GETMODE, &video.vi_mode) != -1
	    && ioctl(1, FBIO_MODEINFO, &video) != -1
	    && ioctl(1, CONS_MOUSECTL, &mouse) != -1
	    && video.vi_cheight > 0 && video.vi_cwidth > 0)
    {
	row = mouse.u.data.y / video.vi_cheight;
	col = mouse.u.data.x / video.vi_cwidth;
	buttons = mouse.u.data.buttons;
	string[0] = ESC; // Our termcode
	string[1] = 'M';
	string[2] = 'S';
	if (oldbuttons == buttons && buttons != 0)
	{
	    button = MOUSE_DRAG;
	}
	else
	{
	    switch (buttons)
	    {
		case 0:
		    button = MOUSE_RELEASE;
		    break;
		case 1:
		    button = MOUSE_LEFT;
		    break;
		case 2:
		    button = MOUSE_MIDDLE;
		    break;
		case 4:
		    button = MOUSE_RIGHT;
		    break;
		default:
		    return;
	    }
	    oldbuttons = buttons;
	}
	string[3] = (char_u)(button);
	string[4] = (char_u)(col + ' ' + 1);
	string[5] = (char_u)(row + ' ' + 1);
	add_to_input_buf(string, 6);
    }
    return;
}
#endif // FEAT_SYSMOUSE

#if defined(FEAT_LIBCALL) || defined(PROTO)
typedef char_u * (*STRPROCSTR)(char_u *);
typedef char_u * (*INTPROCSTR)(int);
typedef int (*STRPROCINT)(char_u *);
typedef int (*INTPROCINT)(int);

/*
 * Call a DLL routine which takes either a string or int param
 * and returns an allocated string.
 */
    int
mch_libcall(
    char_u	*libname,
    char_u	*funcname,
    char_u	*argstring,	// NULL when using a argint
    int		argint,
    char_u	**string_result, // NULL when using number_result
    int		*number_result)
{
# if defined(USE_DLOPEN)
    void	*hinstLib;
    char	*dlerr = NULL;
# else
    shl_t	hinstLib;
# endif
    STRPROCSTR	ProcAdd;
    INTPROCSTR	ProcAddI;
    char_u	*retval_str = NULL;
    int		retval_int = 0;
    int		success = FALSE;

    /*
     * Get a handle to the DLL module.
     */
# if defined(USE_DLOPEN)
    // First clear any error, it's not cleared by the dlopen() call.
    (void)dlerror();

    hinstLib = dlopen((char *)libname, RTLD_LAZY
#  ifdef RTLD_LOCAL
	    | RTLD_LOCAL
#  endif
	    );
    if (hinstLib == NULL)
    {
	// "dlerr" must be used before dlclose()
	dlerr = dlerror();
	if (dlerr != NULL)
	    semsg(_("dlerror = \"%s\""), dlerr);
    }
# else
    hinstLib = shl_load((const char*)libname, BIND_IMMEDIATE|BIND_VERBOSE, 0L);
# endif

    // If the handle is valid, try to get the function address.
    if (hinstLib != NULL)
    {
# ifdef USING_SETJMP
	/*
	 * Catch a crash when calling the library function.  For example when
	 * using a number where a string pointer is expected.
	 */
	mch_startjmp();
	if (SETJMP(lc_jump_env) != 0)
	{
	    success = FALSE;
#  if defined(USE_DLOPEN)
	    dlerr = NULL;
#  endif
	    mch_didjmp();
	}
	else
# endif
	{
	    retval_str = NULL;
	    retval_int = 0;

	    if (argstring != NULL)
	    {
# if defined(USE_DLOPEN)
		*(void **)(&ProcAdd) = dlsym(hinstLib, (const char *)funcname);
		dlerr = dlerror();
# else
		if (shl_findsym(&hinstLib, (const char *)funcname,
					TYPE_PROCEDURE, (void *)&ProcAdd) < 0)
		    ProcAdd = NULL;
# endif
		if ((success = (ProcAdd != NULL
# if defined(USE_DLOPEN)
			    && dlerr == NULL
# endif
			    )))
		{
		    if (string_result == NULL)
			retval_int = ((STRPROCINT)(void *)ProcAdd)(argstring);
		    else
			retval_str = (ProcAdd)(argstring);
		}
	    }
	    else
	    {
# if defined(USE_DLOPEN)
		*(void **)(&ProcAddI) = dlsym(hinstLib, (const char *)funcname);
		dlerr = dlerror();
# else
		if (shl_findsym(&hinstLib, (const char *)funcname,
				       TYPE_PROCEDURE, (void *)&ProcAddI) < 0)
		    ProcAddI = NULL;
# endif
		if ((success = (ProcAddI != NULL
# if defined(USE_DLOPEN)
			    && dlerr == NULL
# endif
			    )))
		{
		    if (string_result == NULL)
			retval_int = ((INTPROCINT)(void *)ProcAddI)(argint);
		    else
			retval_str = (ProcAddI)(argint);
		}
	    }

	    // Save the string before we free the library.
	    // Assume that a "1" or "-1" result is an illegal pointer.
	    if (string_result == NULL)
		*number_result = retval_int;
	    else if (retval_str != NULL
		    && retval_str != (char_u *)1
		    && retval_str != (char_u *)-1)
		*string_result = vim_strsave(retval_str);
	}

# ifdef USING_SETJMP
	mch_endjmp();
#  ifdef SIGHASARG
	if (lc_signal != 0)
	{
	    int i;

	    // try to find the name of this signal
	    for (i = 0; signal_info[i].sig != -1; i++)
		if (lc_signal == signal_info[i].sig)
		    break;
	    semsg(_(e_got_sig_str_in_libcall), signal_info[i].name);
	}
#  endif
# endif

# if defined(USE_DLOPEN)
	// "dlerr" must be used before dlclose()
	if (dlerr != NULL)
	    semsg(_("dlerror = \"%s\""), dlerr);

	// Free the DLL module.
	(void)dlclose(hinstLib);
# else
	(void)shl_unload(hinstLib);
# endif
    }

    if (!success)
    {
	semsg(_(e_library_call_failed_for_str), funcname);
	return FAIL;
    }

    return OK;
}
#endif

#if (defined(FEAT_X11) && defined(FEAT_XCLIPBOARD)) || defined(PROTO)
static int	xterm_trace = -1;	// default: disabled
static int	xterm_button;

/*
 * Setup a dummy window for X selections in a terminal.
 */
    void
setup_term_clip(void)
{
    int		z = 0;
    char	*strp = "";
    Widget	AppShell;

    if (!x_connect_to_server())
	return;

    open_app_context();
    if (app_context != NULL && xterm_Shell == (Widget)0)
    {
	int (*oldhandler)(Display*, XErrorEvent*);
# if defined(USING_SETJMP)
	int (*oldIOhandler)(Display*);
# endif
# ifdef ELAPSED_FUNC
	elapsed_T start_tv;

	if (p_verbose > 0)
	    ELAPSED_INIT(start_tv);
# endif

	// Ignore X errors while opening the display
	oldhandler = XSetErrorHandler(x_error_check);

# if defined(USING_SETJMP)
	// Ignore X IO errors while opening the display
	oldIOhandler = XSetIOErrorHandler(x_IOerror_check);
	mch_startjmp();
	if (SETJMP(lc_jump_env) != 0)
	{
	    mch_didjmp();
	    xterm_dpy = NULL;
	}
	else
# endif
	{
	    xterm_dpy = XtOpenDisplay(app_context, xterm_display,
		    "vim_xterm", "Vim_xterm", NULL, 0, &z, &strp);
	    if (xterm_dpy != NULL)
		xterm_dpy_retry_count = 0;
# if defined(USING_SETJMP)
	    mch_endjmp();
# endif
	}

# if defined(USING_SETJMP)
	// Now handle X IO errors normally.
	(void)XSetIOErrorHandler(oldIOhandler);
# endif
	// Now handle X errors normally.
	(void)XSetErrorHandler(oldhandler);

	if (xterm_dpy == NULL)
	{
	    if (p_verbose > 0)
		verb_msg(_("Opening the X display failed"));
	    return;
	}

	// Catch terminating error of the X server connection.
	(void)XSetIOErrorHandler(x_IOerror_handler);

# ifdef ELAPSED_FUNC
	if (p_verbose > 0)
	{
	    verbose_enter();
	    xopen_message(ELAPSED_FUNC(start_tv));
	    verbose_leave();
	}
# endif

	// Create a Shell to make converters work.
	AppShell = XtVaAppCreateShell("vim_xterm", "Vim_xterm",
		applicationShellWidgetClass, xterm_dpy,
		NULL);
	if (AppShell == (Widget)0)
	    return;
	xterm_Shell = XtVaCreatePopupShell("VIM",
		topLevelShellWidgetClass, AppShell,
		XtNmappedWhenManaged, 0,
		XtNwidth, 1,
		XtNheight, 1,
		NULL);
	if (xterm_Shell == (Widget)0)
	    return;

	x11_setup_atoms(xterm_dpy);
	x11_setup_selection(xterm_Shell);
	if (x11_display == NULL)
	    x11_display = xterm_dpy;

	XtRealizeWidget(xterm_Shell);
	XSync(xterm_dpy, False);
	xterm_update();
    }
    if (xterm_Shell != (Widget)0)
    {
	clip_init(TRUE);
	if (x11_window == 0 && (strp = getenv("WINDOWID")) != NULL)
	    x11_window = (Window)atol(strp);
	// Check if $WINDOWID is valid.
	if (test_x11_window(xterm_dpy) == FAIL)
	    x11_window = 0;
	if (x11_window != 0)
	    xterm_trace = 0;
    }
}

    void
start_xterm_trace(int button)
{
    if (x11_window == 0 || xterm_trace < 0 || xterm_Shell == (Widget)0)
	return;
    xterm_trace = 1;
    xterm_button = button;
    do_xterm_trace();
}


    void
stop_xterm_trace(void)
{
    if (xterm_trace < 0)
	return;
    xterm_trace = 0;
}

/*
 * Query the xterm pointer and generate mouse termcodes if necessary
 * return TRUE if dragging is active, else FALSE
 */
    static int
do_xterm_trace(void)
{
    Window		root, child;
    int			root_x, root_y;
    int			win_x, win_y;
    int			row, col;
    int_u		mask_return;
    char_u		buf[50];
    char_u		*strp;
    long		got_hints;
    static char_u	*mouse_code;
    static char_u	mouse_name[2] = {KS_MOUSE, KE_FILLER};
    static int		prev_row = 0, prev_col = 0;
    static XSizeHints	xterm_hints;

    if (xterm_trace <= 0)
	return FALSE;

    if (xterm_trace == 1)
    {
	// Get the hints just before tracking starts.  The font size might
	// have changed recently.
	if (!XGetWMNormalHints(xterm_dpy, x11_window, &xterm_hints, &got_hints)
		|| !(got_hints & PResizeInc)
		|| xterm_hints.width_inc <= 1
		|| xterm_hints.height_inc <= 1)
	{
	    xterm_trace = -1;  // Not enough data -- disable tracing
	    return FALSE;
	}

	// Rely on the same mouse code for the duration of this
	mouse_code = find_termcode(mouse_name);
	prev_row = mouse_row;
	prev_col = mouse_col;
	xterm_trace = 2;

	// Find the offset of the chars, there might be a scrollbar on the
	// left of the window and/or a menu on the top (eterm etc.)
	XQueryPointer(xterm_dpy, x11_window, &root, &child, &root_x, &root_y,
		      &win_x, &win_y, &mask_return);
	xterm_hints.y = win_y - (xterm_hints.height_inc * mouse_row)
			      - (xterm_hints.height_inc / 2);
	if (xterm_hints.y <= xterm_hints.height_inc / 2)
	    xterm_hints.y = 2;
	xterm_hints.x = win_x - (xterm_hints.width_inc * mouse_col)
			      - (xterm_hints.width_inc / 2);
	if (xterm_hints.x <= xterm_hints.width_inc / 2)
	    xterm_hints.x = 2;
	return TRUE;
    }
    if (mouse_code == NULL || STRLEN(mouse_code) > 45)
    {
	xterm_trace = 0;
	return FALSE;
    }

    XQueryPointer(xterm_dpy, x11_window, &root, &child, &root_x, &root_y,
		  &win_x, &win_y, &mask_return);

    row = check_row((win_y - xterm_hints.y) / xterm_hints.height_inc);
    col = check_col((win_x - xterm_hints.x) / xterm_hints.width_inc);
    if (row == prev_row && col == prev_col)
	return TRUE;

    STRCPY(buf, mouse_code);
    strp = buf + STRLEN(buf);
    *strp++ = (xterm_button | MOUSE_DRAG) & ~0x20;
    *strp++ = (char_u)(col + ' ' + 1);
    *strp++ = (char_u)(row + ' ' + 1);
    *strp = 0;
    add_to_input_buf(buf, STRLEN(buf));

    prev_row = row;
    prev_col = col;
    return TRUE;
}

# if defined(FEAT_GUI) || defined(FEAT_XCLIPBOARD) || defined(PROTO)
/*
 * Destroy the display, window and app_context.  Required for GTK.
 */
    void
clear_xterm_clip(void)
{
    if (xterm_Shell != (Widget)0)
    {
	XtDestroyWidget(xterm_Shell);
	xterm_Shell = (Widget)0;
    }
    if (xterm_dpy != NULL)
    {
#  if 0
	// Lesstif and Solaris crash here, lose some memory
	XtCloseDisplay(xterm_dpy);
#  endif
	if (x11_display == xterm_dpy)
	    x11_display = NULL;
	xterm_dpy = NULL;
    }
#  if 0
    if (app_context != (XtAppContext)NULL)
    {
	// Lesstif and Solaris crash here, lose some memory
	XtDestroyApplicationContext(app_context);
	app_context = (XtAppContext)NULL;
    }
#  endif
}
# endif

/*
 * Catch up with GUI or X events.
 */
    static void
clip_update(void)
{
# ifdef FEAT_GUI
    if (gui.in_use)
	gui_mch_update();
    else
# endif
    if (xterm_Shell != (Widget)0)
	xterm_update();
}

/*
 * Catch up with any queued X events.  This may put keyboard input into the
 * input buffer, call resize call-backs, trigger timers etc.  If there is
 * nothing in the X event queue (& no timers pending), then we return
 * immediately.
 */
    static void
xterm_update(void)
{
    XEvent event;

    for (;;)
    {
	XtInputMask mask = XtAppPending(app_context);

	if (mask == 0 || vim_is_input_buf_full())
	    break;

	if (mask & XtIMXEvent)
	{
	    // There is an event to process.
	    XtAppNextEvent(app_context, &event);
#ifdef FEAT_CLIENTSERVER
	    {
		XPropertyEvent *e = (XPropertyEvent *)&event;

		if (e->type == PropertyNotify && e->window == commWindow
		   && e->atom == commProperty && e->state == PropertyNewValue)
		    serverEventProc(xterm_dpy, &event, 0);
	    }
#endif
	    XtDispatchEvent(&event);
	}
	else
	{
	    // There is something else than an event to process.
	    XtAppProcessEvent(app_context, mask);
	}
    }
}

    int
clip_xterm_own_selection(Clipboard_T *cbd)
{
    if (xterm_Shell != (Widget)0)
	return clip_x11_own_selection(xterm_Shell, cbd);
    return FAIL;
}

    void
clip_xterm_lose_selection(Clipboard_T *cbd)
{
    if (xterm_Shell != (Widget)0)
	clip_x11_lose_selection(xterm_Shell, cbd);
}

    void
clip_xterm_request_selection(Clipboard_T *cbd)
{
    if (xterm_Shell != (Widget)0)
	clip_x11_request_selection(xterm_Shell, xterm_dpy, cbd);
}

    void
clip_xterm_set_selection(Clipboard_T *cbd)
{
    clip_x11_set_selection(cbd);
}
#endif


#if defined(USE_XSMP) || defined(PROTO)
/*
 * Code for X Session Management Protocol.
 */

# if defined(FEAT_GUI) && defined(USE_XSMP_INTERACT)
/*
 * This is our chance to ask the user if they want to save,
 * or abort the logout
 */
    static void
xsmp_handle_interaction(SmcConn smc_conn, SmPointer client_data UNUSED)
{
    int		save_cmod_flags;
    int		cancel_shutdown = False;

    save_cmod_flags = cmdmod.cmod_flags;
    cmdmod.cmod_flags |= CMOD_CONFIRM;
    if (check_changed_any(FALSE, FALSE))
	// Mustn't logout
	cancel_shutdown = True;
    cmdmod.cmod_flags = save_cmod_flags;
    setcursor();		// position cursor
    out_flush();

    // Done interaction
    SmcInteractDone(smc_conn, cancel_shutdown);

    // Finish off
    // Only end save-yourself here if we're not cancelling shutdown;
    // we'll get a cancelled callback later in which we'll end it.
    // Hopefully get around glitchy SMs (like GNOME-1)
    if (!cancel_shutdown)
    {
	xsmp.save_yourself = False;
	SmcSaveYourselfDone(smc_conn, True);
    }
}
# endif

/*
 * Callback that starts save-yourself.
 */
    static void
xsmp_handle_save_yourself(
    SmcConn	smc_conn,
    SmPointer	client_data UNUSED,
    int		save_type UNUSED,
    Bool	shutdown,
    int		interact_style UNUSED,
    Bool	fast UNUSED)
{
    // Handle already being in saveyourself
    if (xsmp.save_yourself)
	SmcSaveYourselfDone(smc_conn, True);
    xsmp.save_yourself = True;
    xsmp.shutdown = shutdown;

    // First up, preserve all files
    out_flush();
    ml_sync_all(FALSE, FALSE);	// preserve all swap files

    if (p_verbose > 0)
	verb_msg(_("XSMP handling save-yourself request"));

# if defined(FEAT_GUI) && defined(USE_XSMP_INTERACT)
    // Now see if we can ask about unsaved files
    if (shutdown && !fast && gui.in_use)
	// Need to interact with user, but need SM's permission
	SmcInteractRequest(smc_conn, SmDialogError,
					xsmp_handle_interaction, client_data);
    else
# endif
    {
	// Can stop the cycle here
	SmcSaveYourselfDone(smc_conn, True);
	xsmp.save_yourself = False;
    }
}


/*
 * Callback to warn us of imminent death.
 */
    static void
xsmp_die(SmcConn smc_conn UNUSED, SmPointer client_data UNUSED)
{
    xsmp_close();

    // quit quickly leaving swapfiles for modified buffers behind
    getout_preserve_modified(0);
}


/*
 * Callback to tell us that save-yourself has completed.
 */
    static void
xsmp_save_complete(
    SmcConn	smc_conn UNUSED,
    SmPointer	client_data UNUSED)
{
    xsmp.save_yourself = False;
}


/*
 * Callback to tell us that an instigated shutdown was cancelled
 * (maybe even by us)
 */
    static void
xsmp_shutdown_cancelled(
    SmcConn	smc_conn,
    SmPointer	client_data UNUSED)
{
    if (xsmp.save_yourself)
	SmcSaveYourselfDone(smc_conn, True);
    xsmp.save_yourself = False;
    xsmp.shutdown = False;
}


/*
 * Callback to tell us that a new ICE connection has been established.
 */
    static void
xsmp_ice_connection(
    IceConn	iceConn,
    IcePointer	clientData UNUSED,
    Bool	opening,
    IcePointer	*watchData UNUSED)
{
    // Intercept creation of ICE connection fd
    if (opening)
    {
	xsmp_icefd = IceConnectionNumber(iceConn);
	IceRemoveConnectionWatch(xsmp_ice_connection, NULL);
    }
}


// Handle any ICE processing that's required; return FAIL if SM lost
    int
xsmp_handle_requests(void)
{
    Bool rep;

    if (IceProcessMessages(xsmp.iceconn, NULL, &rep)
						 == IceProcessMessagesIOError)
    {
	// Lost ICE
	if (p_verbose > 0)
	    verb_msg(_("XSMP lost ICE connection"));
	xsmp_close();
	return FAIL;
    }
    else
	return OK;
}

static int dummy;

// Set up X Session Management Protocol
    void
xsmp_init(void)
{
    char		errorstring[80];
    SmcCallbacks	smcallbacks;
#if 0
    SmPropValue		smname;
    SmProp		smnameprop;
    SmProp		*smprops[1];
#endif

    if (p_verbose > 0)
	verb_msg(_("XSMP opening connection"));

    xsmp.save_yourself = xsmp.shutdown = False;

    // Set up SM callbacks - must have all, even if they're not used
    smcallbacks.save_yourself.callback = xsmp_handle_save_yourself;
    smcallbacks.save_yourself.client_data = NULL;
    smcallbacks.die.callback = xsmp_die;
    smcallbacks.die.client_data = NULL;
    smcallbacks.save_complete.callback = xsmp_save_complete;
    smcallbacks.save_complete.client_data = NULL;
    smcallbacks.shutdown_cancelled.callback = xsmp_shutdown_cancelled;
    smcallbacks.shutdown_cancelled.client_data = NULL;

    // Set up a watch on ICE connection creations.  The "dummy" argument is
    // apparently required for FreeBSD (we get a BUS error when using NULL).
    if (IceAddConnectionWatch(xsmp_ice_connection, &dummy) == 0)
    {
	if (p_verbose > 0)
	    verb_msg(_("XSMP ICE connection watch failed"));
	return;
    }

    // Create an SM connection
    xsmp.smcconn = SmcOpenConnection(
	    NULL,
	    NULL,
	    SmProtoMajor,
	    SmProtoMinor,
	    SmcSaveYourselfProcMask | SmcDieProcMask
		     | SmcSaveCompleteProcMask | SmcShutdownCancelledProcMask,
	    &smcallbacks,
	    NULL,
	    &xsmp.clientid,
	    sizeof(errorstring) - 1,
	    errorstring);
    if (xsmp.smcconn == NULL)
    {
	if (p_verbose > 0)
	{
	    char errorreport[132];

	    // If the message is too long it might not be NUL terminated.  Add
	    // a NUL at the end to make sure we don't go over the end.
	    errorstring[sizeof(errorstring) - 1] = NUL;
	    vim_snprintf(errorreport, sizeof(errorreport),
			 _("XSMP SmcOpenConnection failed: %s"), errorstring);
	    verb_msg(errorreport);
	}
	return;
    }
    xsmp.iceconn = SmcGetIceConnection(xsmp.smcconn);

#if 0
    // ID ourselves
    smname.value = "vim";
    smname.length = 3;
    smnameprop.name = "SmProgram";
    smnameprop.type = "SmARRAY8";
    smnameprop.num_vals = 1;
    smnameprop.vals = &smname;

    smprops[0] = &smnameprop;
    SmcSetProperties(xsmp.smcconn, 1, smprops);
#endif
}


// Shut down XSMP comms.
    void
xsmp_close(void)
{
    if (xsmp_icefd == -1)
	return;

    SmcCloseConnection(xsmp.smcconn, 0, NULL);
    if (xsmp.clientid != NULL)
	free(xsmp.clientid);
    xsmp.clientid = NULL;
    xsmp_icefd = -1;
}
#endif // USE_XSMP

#if defined(FEAT_RELTIME) || defined(PROTO)
# if defined(PROF_NSEC) || defined(PROTO)
/*
 * Implement timeout with timer_create() and timer_settime().
 */
static volatile sig_atomic_t timeout_flag = FALSE;
static timer_t		     timer_id;
static int		     timer_created = FALSE;

/*
 * Callback for when the timer expires.
 */
    static void
set_flag(union sigval _unused UNUSED)
{
    timeout_flag = TRUE;
}

/*
 * Stop any active timeout.
 */
    void
stop_timeout(void)
{
    static struct itimerspec disarm = {{0, 0}, {0, 0}};

    if (timer_created)
    {
	int ret = timer_settime(timer_id, 0, &disarm, NULL);

	if (ret < 0)
	    semsg(_(e_could_not_clear_timeout_str), strerror(errno));
    }

    // Clear the current timeout flag; any previous timeout should be
    // considered _not_ triggered.
    timeout_flag = FALSE;
}

/*
 * Start the timeout timer.
 *
 * The return value is a pointer to a flag that is initialised to FALSE. If the
 * timeout expires, the flag is set to TRUE. This will only return pointers to
 * static memory; i.e. any pointer returned by this function may always be
 * safely dereferenced.
 *
 * This function is not expected to fail, but if it does it will still return a
 * valid flag pointer; the flag will remain stuck as FALSE .
 */
    volatile sig_atomic_t *
start_timeout(long msec)
{
    struct itimerspec interval = {
	    {0, 0},					// Do not repeat.
	    {msec / 1000, (msec % 1000) * 1000000}};	// Timeout interval
    int ret;

    // This is really the caller's responsibility, but let's make sure the
    // previous timer has been stopped.
    stop_timeout();

    if (!timer_created)
    {
	struct sigevent action = {0};

	action.sigev_notify = SIGEV_THREAD;
	action.sigev_notify_function = set_flag;
	ret = timer_create(CLOCK_MONOTONIC, &action, &timer_id);
	if (ret < 0)
	{
	    semsg(_(e_could_not_set_timeout_str), strerror(errno));
	    return &timeout_flag;
	}
	timer_created = TRUE;
    }

# ifdef FEAT_EVAL
    ch_log(NULL, "setting timeout timer to %d sec %ld nsec",
	       (int)interval.it_value.tv_sec, (long)interval.it_value.tv_nsec);
# endif
    ret = timer_settime(timer_id, 0, &interval, NULL);
    if (ret < 0)
	semsg(_(e_could_not_set_timeout_str), strerror(errno));

    return &timeout_flag;
}

/*
 * To be used before fork/exec: delete any created timer.
 */
    void
delete_timer(void)
{
    if (!timer_created)
	return;

    timer_delete(timer_id);
    timer_created = FALSE;
}

# else // PROF_NSEC

/*
 * Implement timeout with setitimer()
 */
static struct sigaction		prev_sigaction;
static volatile sig_atomic_t	timeout_flag	     = FALSE;
static int			timer_active	     = FALSE;
static int			timer_handler_active = FALSE;
static volatile sig_atomic_t	alarm_pending	     = FALSE;

/*
 * Handle SIGALRM for a timeout.
 */
    static void
set_flag SIGDEFARG(sigarg)
{
    if (alarm_pending)
	alarm_pending = FALSE;
    else
	timeout_flag = TRUE;
}

/*
 * Stop any active timeout.
 */
    void
stop_timeout(void)
{
    static struct itimerval disarm = {{0, 0}, {0, 0}};
    int			    ret;

    if (timer_active)
    {
	timer_active = FALSE;
	ret = setitimer(ITIMER_REAL, &disarm, NULL);
	if (ret < 0)
	    // Should only get here as a result of coding errors.
	    semsg(_(e_could_not_clear_timeout_str), strerror(errno));
    }

    if (timer_handler_active)
    {
	timer_handler_active = FALSE;
	ret = sigaction(SIGALRM, &prev_sigaction, NULL);
	if (ret < 0)
	    // Should only get here as a result of coding errors.
	    semsg(_(e_could_not_reset_handler_for_timeout_str),
							      strerror(errno));
    }
    timeout_flag = FALSE;
}

/*
 * Start the timeout timer.
 *
 * The return value is a pointer to a flag that is initialised to FALSE. If the
 * timeout expires, the flag is set to TRUE. This will only return pointers to
 * static memory; i.e. any pointer returned by this function may always be
 * safely dereferenced.
 *
 * This function is not expected to fail, but if it does it will still return a
 * valid flag pointer; the flag will remain stuck as FALSE .
 */
    volatile sig_atomic_t *
start_timeout(long msec)
{
    struct itimerval	interval = {
	    {0, 0},				    // Do not repeat.
	    {msec / 1000, (msec % 1000) * 1000}};   // Timeout interval
    struct sigaction	handle_alarm;
    int			ret;
    sigset_t		sigs;
    sigset_t		saved_sigs;

    // This is really the caller's responsibility, but let's make sure the
    // previous timer has been stopped.
    stop_timeout();

    // There is a small chance that SIGALRM is pending and so the handler must
    // ignore it on the first call.
    alarm_pending = FALSE;
    ret = sigemptyset(&sigs);
    ret = ret == 0 ? sigaddset(&sigs, SIGALRM) : ret;
    ret = ret == 0 ? sigprocmask(SIG_BLOCK, &sigs, &saved_sigs) : ret;
    timeout_flag = FALSE;
    ret = ret == 0 ? sigpending(&sigs) : ret;
    if (ret == 0)
    {
	alarm_pending = sigismember(&sigs, SIGALRM);
	ret = sigprocmask(SIG_SETMASK, &saved_sigs, NULL);
    }
    if (unlikely(ret != 0 || alarm_pending < 0))
    {
	// Just catching coding errors. Write an error message, but carry on.
	semsg(_(e_could_not_check_for_pending_sigalrm_str), strerror(errno));
	alarm_pending = FALSE;
    }

    // Set up the alarm handler first.
    ret = sigemptyset(&handle_alarm.sa_mask);
    handle_alarm.sa_handler = set_flag;
    handle_alarm.sa_flags = 0;
    ret = ret == 0 ?  sigaction(SIGALRM, &handle_alarm, &prev_sigaction) : ret;
    if (ret < 0)
    {
	// Should only get here as a result of coding errors.
	semsg(_(e_could_not_set_handler_for_timeout_str), strerror(errno));
	return &timeout_flag;
    }
    timer_handler_active = TRUE;

    // Set up the interval timer once the alarm handler is in place.
    ret = setitimer(ITIMER_REAL, &interval, NULL);
    if (ret < 0)
    {
	// Should only get here as a result of coding errors.
	semsg(_(e_could_not_set_timeout_str), strerror(errno));
	stop_timeout();
	return &timeout_flag;
    }

    timer_active = TRUE;
    return &timeout_flag;
}
# endif // PROF_NSEC
#endif  // FEAT_RELTIME
