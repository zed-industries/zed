/* vi:set ts=8 sw=8 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *			Visual Workshop integration by Gordon Prieur
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * NetBeans Debugging Tools. What are these tools and why are they important?
 * There are two main tools here. The first tool is a tool for delaying or
 * stopping gvim during startup.  The second tool is a protocol log tool.
 *
 * The startup delay tool is called nbdebug_wait(). This is very important for
 * debugging startup problems because gvim will be started automatically from
 * netbeans and cannot be run directly from a debugger. The only way to debug
 * a gvim started by netbeans is by attaching a debugger to it. Without this
 * tool all startup code will have completed before you can get the pid and
 * attach.
 *
 * The second tool is a log tool.
 *
 * This code must have NBDEBUG defined for it to be compiled into vim/gvim.
 */

#ifdef NBDEBUG

#include "vim.h"

FILE		*nb_debug = NULL;
u_int		 nb_dlevel = 0;		// nb_debug verbosity level

void		 nbdb(char *, ...) ATTRIBUTE_FORMAT_PRINTF(1, 2);

static int	 lookup(char *);
#ifdef USE_NB_ERRORHANDLER
static int	 errorHandler(Display *, XErrorEvent *);
#endif

/*
 * nbdebug_wait	-   This function can be used to delay or stop execution of vim.
 *		    It's normally used to delay startup while attaching a
 *		    debugger to a running process. Since NetBeans starts gvim
 *		    from a background process this is the only way to debug
 *		    startup problems.
 */
	void
nbdebug_wait(
	u_int		 wait_flags,	// tells what to do
	char		*wait_var,	// wait environment variable
	u_int		 wait_secs)	// how many seconds to wait
{

	init_homedir();			// not inited yet
#ifdef USE_WDDUMP
	WDDump(0, 0, 0);
#endif

	// for debugging purposes only
	if (wait_flags & WT_ENV && wait_var && getenv(wait_var) != NULL)
	{
		sleep(atoi(getenv(wait_var)));
	}
	else if (wait_flags & WT_WAIT && lookup("~/.gvimwait"))
	{
		sleep(wait_secs > 0 && wait_secs < 120 ? wait_secs : 20);
	}
	else if (wait_flags & WT_STOP && lookup("~/.gvimstop"))
	{
		int w = 1;
		while (w)
		{
			;
		}
	}
}

	void
nbdebug_log_init(
	char		*log_var,	// env var with log file
	char		*level_var)	// env var with nb_debug level
{
	char		*file;		// possible nb_debug output file
	char		*cp;		// nb_dlevel pointer

	if (log_var && (file = getenv(log_var)) != NULL)
	{
		time_t now;

		nb_debug = fopen(file, "a");
		time(&now);
		fprintf(nb_debug, "%s", get_ctime(now, TRUE));
		if (level_var && (cp = getenv(level_var)) != NULL)
		{
			nb_dlevel = strtoul(cp, NULL, 0);
		}
		else
		{
			nb_dlevel = NB_TRACE;	// default level
		}
#ifdef USE_NB_ERRORHANDLER
		XSetErrorHandler(errorHandler);
#endif
	}

}

	void
nbdbg(char *fmt, ...)
{
	va_list		 ap;

	if (nb_debug != NULL && nb_dlevel & NB_TRACE)
	{
		va_start(ap, fmt);
		vfprintf(nb_debug, fmt, ap);
		va_end(ap);
		fflush(nb_debug);
	}

}

	static int
lookup(char *file)
{
	char		 buf[BUFSIZ];

	expand_env((char_u *) file, (char_u *) buf, BUFSIZ);
	return
#ifndef FEAT_GUI_MSWIN
		(access(buf, F_OK) == 0);
#else
		(access(buf, 0) == 0);
#endif

}

#ifdef USE_NB_ERRORHANDLER
	static int
errorHandler(
	Display		*dpy,
	XErrorEvent	*err)
{
	char		 msg[256];
	char		 buf[256];

	XGetErrorText(dpy, err->error_code, msg, sizeof(msg));
	nbdbg("\n\nNBDEBUG Vim: X Error of failed request: %s\n", msg);

	sprintf(buf, "%d", err->request_code);
	XGetErrorDatabaseText(dpy,
	    "XRequest", buf, "Unknown", msg, sizeof(msg));
	nbdbg("\tMajor opcode of failed request: %d (%s)\n",
	    err->request_code, msg);
	if (err->request_code > 128)
	{
		nbdbg("\tMinor opcode of failed request: %d\n",
		    err->minor_code);
	}

	return 0;
}
#endif


#endif // NBDEBUG
