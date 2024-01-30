/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

#define EXTERN
#include "vim.h"

#ifdef __CYGWIN__
# include <cygwin/version.h>
# include <sys/cygwin.h>	// for cygwin_conv_to_posix_path() and/or
				// cygwin_conv_path()
# include <limits.h>
#endif

#if defined(MSWIN) && (!defined(FEAT_GUI_MSWIN) || defined(VIMDLL))
# include "iscygpty.h"
#endif

// Values for edit_type.
#define EDIT_NONE   0	    // no edit type yet
#define EDIT_FILE   1	    // file name argument[s] given, use argument list
#define EDIT_STDIN  2	    // read file from stdin
#define EDIT_TAG    3	    // tag name argument given, use tagname
#define EDIT_QF	    4	    // start in quickfix mode

#if (defined(UNIX) || defined(VMS)) && !defined(NO_VIM_MAIN)
static int file_owned(char *fname);
#endif
static void mainerr(int, char_u *);
static void early_arg_scan(mparm_T *parmp);
#ifndef NO_VIM_MAIN
static void usage(void);
static void parse_command_name(mparm_T *parmp);
static void command_line_scan(mparm_T *parmp);
static void check_tty(mparm_T *parmp);
static void read_stdin(void);
static void create_windows(mparm_T *parmp);
static void edit_buffers(mparm_T *parmp, char_u *cwd);
static void exe_pre_commands(mparm_T *parmp);
static void exe_commands(mparm_T *parmp);
static void source_startup_scripts(mparm_T *parmp);
static void main_start_gui(void);
static void check_swap_exists_action(void);
# ifdef FEAT_EVAL
static void set_progpath(char_u *argv0);
# endif
#endif


/*
 * Different types of error messages.
 */
static char *(main_errors[]) =
{
    N_("Unknown option argument"),
#define ME_UNKNOWN_OPTION	0
    N_("Too many edit arguments"),
#define ME_TOO_MANY_ARGS	1
    N_("Argument missing after"),
#define ME_ARG_MISSING		2
    N_("Garbage after option argument"),
#define ME_GARBAGE		3
    N_("Too many \"+command\", \"-c command\" or \"--cmd command\" arguments"),
#define ME_EXTRA_CMD		4
    N_("Invalid argument for"),
#define ME_INVALID_ARG		5
};

#ifndef PROTO		// don't want a prototype for main()

// Various parameters passed between main() and other functions.
static mparm_T	params;

#ifdef _IOLBF
static void *s_vbuf = NULL;		// buffer for setvbuf()
#endif

#ifndef NO_VIM_MAIN	// skip this for unittests

static char_u *start_dir = NULL;	// current working dir on startup

static int has_dash_c_arg = FALSE;

# ifdef VIMDLL
__declspec(dllexport)
# endif
    int
# ifdef MSWIN
VimMain
# else
main
# endif
(int argc, char **argv)
{
#if defined(STARTUPTIME) || defined(CLEAN_RUNTIMEPATH)
    int		i;
#endif

    /*
     * Do any system-specific initialisations.  These can NOT use IObuff or
     * NameBuff.  Thus emsg2() cannot be called!
     */
    mch_early_init();

    // Source startup scripts.
    source_startup_scripts(&params);

#if 0
    /*
     * Newer version of MzScheme (Racket) require earlier (trampolined)
     * initialisation via scheme_main_setup.
     */
    return mzscheme_main();
#else
    return vim_main2();
#endif
}
