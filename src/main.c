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

#ifdef MSWIN
    /*
     * MinGW expands command line arguments, which confuses our code to
     * convert when 'encoding' changes.  Get the unexpanded arguments.
     */
    argc = get_cmd_argsW(&argv);
#endif

    // Many variables are in "params" so that we can pass them to invoked
    // functions without a lot of arguments.  "argc" and "argv" are also
    // copied, so that they can be changed.
    CLEAR_FIELD(params);
    params.argc = argc;
    params.argv = argv;
    params.want_full_screen = TRUE;
#ifdef FEAT_EVAL
    params.use_debug_break_level = -1;
#endif
    params.window_count = -1;

    autocmd_init();

#ifdef FEAT_RUBY
    {
	int ruby_stack_start;
	vim_ruby_init((void *)&ruby_stack_start);
    }
#endif

#ifdef FEAT_TCL
    vim_tcl_init(params.argv[0]);
#endif

#ifdef MEM_PROFILE
    atexit(vim_mem_profile_dump);
#endif

#if defined(STARTUPTIME) || defined(FEAT_JOB_CHANNEL)
    // Need to find "--startuptime" and "--log" before actually parsing
    // arguments.
    for (i = 1; i < argc - 1; ++i)
    {
# ifdef STARTUPTIME
	if (STRICMP(argv[i], "--startuptime") == 0 && time_fd == NULL)
	{
	    time_fd = mch_fopen(argv[i + 1], "a");
	    TIME_MSG("--- VIM STARTING ---");
	}
# endif
# ifdef FEAT_EVAL
	if (STRICMP(argv[i], "--log") == 0)
	    ch_logfile((char_u *)(argv[i + 1]), (char_u *)"ao");
# endif
    }
#endif

#ifdef CLEAN_RUNTIMEPATH
    // Need to find "--clean" before actually parsing arguments.
    for (i = 1; i < argc; ++i)
	if (STRICMP(argv[i], "--clean") == 0)
	{
	    params.clean = TRUE;
	    break;
	}
#endif
#ifdef MSWIN
    // Need to find "-register" and "-unregister" before loading any libraries.
    for (i = 1; i < argc; ++i)
	if ((STRICMP(argv[i] + 1, "register") == 0
				    || STRICMP(argv[i] + 1, "unregister") == 0)
		&& (argv[i][0] == '-' || argv[i][0] == '/'))
	{
	    found_register_arg = TRUE;
	    break;
	}
#endif

    /*
     * Various initialisations shared with tests.
     */
    common_init(&params);

#ifdef VIMDLL
    // Check if the current executable file is for the GUI subsystem.
    gui.starting = mch_is_gui_executable();
#elif defined(FEAT_GUI_MSWIN)
    gui.starting = TRUE;
#endif

#ifdef FEAT_CLIENTSERVER
    /*
     * Do the client-server stuff, unless "--servername ''" was used.
     * This may exit Vim if the command was sent to the server.
     */
    exec_on_server(&params);
#endif

    /*
     * Figure out the way to work from the command name argv[0].
     * "vimdiff" starts diff mode, "rvim" sets "restricted", etc.
     */
    parse_command_name(&params);

    /*
     * Process the command line arguments.  File names are put in the global
     * argument list "global_alist".
     */
    command_line_scan(&params);
    TIME_MSG("parsing arguments");

    /*
     * On some systems, when we compile with the GUI, we always use it.  On Mac
     * there is no terminal version, and on Windows we can't fork one off with
     * :gui.
     */
#ifdef ALWAYS_USE_GUI
    gui.starting = TRUE;
#else
# if defined(FEAT_GUI_X11) || defined(FEAT_GUI_GTK)
    /*
     * Check if the GUI can be started.  Reset gui.starting if not.
     * Don't know about other systems, stay on the safe side and don't check.
     */
    if (gui.starting)
    {
	if (gui_init_check() == FAIL)
	{
	    gui.starting = FALSE;

	    // When running "evim" or "gvim -y" we need the menus, exit if we
	    // don't have them.
	    if (params.evim_mode)
		mch_exit(1);
	}
    }
# endif
#endif

    if (GARGCOUNT > 0)
    {
#ifdef EXPAND_FILENAMES
	/*
	 * Expand wildcards in file names.
	 */
	if (!params.literal)
	{
	    start_dir = alloc(MAXPATHL);
	    if (start_dir != NULL)
		mch_dirname(start_dir, MAXPATHL);
	    // Temporarily add '(' and ')' to 'isfname'.  These are valid
	    // filename characters but are excluded from 'isfname' to make
	    // "gf" work on a file name in parentheses (e.g.: see vim.h).
	    do_cmdline_cmd((char_u *)":set isf+=(,)");
	    alist_expand(NULL, 0);
	    do_cmdline_cmd((char_u *)":set isf&");
	    if (start_dir != NULL)
		mch_chdir((char *)start_dir);
	}
#endif
	params.fname = alist_name(&GARGLIST[0]);
    }

#ifdef MSWIN
    {
	// Remember the number of entries in the argument list.  If it changes
	// we don't react on setting 'encoding'.
	set_alist_count();
    }
#endif

#ifdef MSWIN
    if (GARGCOUNT == 1 && params.full_path)
    {
	/*
	 * If there is one filename, fully qualified, we have very probably
	 * been invoked from explorer, so change to the file's directory.
	 * Hint: to avoid this when typing a command use a forward slash.
	 * If the cd fails, it doesn't matter.
	 */
	if (vim_chdirfile(params.fname, "drop") == OK)
	    last_chdir_reason = "drop";
	if (start_dir != NULL)
	    mch_dirname(start_dir, MAXPATHL);
    }
#endif
    TIME_MSG("expanding arguments");

#ifdef FEAT_DIFF
    if (params.diff_mode && params.window_count == -1)
	params.window_count = 0;	// open up to 3 windows
#endif

    // Don't redraw until much later.
    ++RedrawingDisabled;

    /*
     * When listing swap file names, don't do cursor positioning et. al.
     */
    if (recoverymode && params.fname == NULL)
	params.want_full_screen = FALSE;

    /*
     * When certain to start the GUI, don't check terminal capabilities.
     * For GTK we can't be sure, but when started from the desktop it doesn't
     * make sense to try using a terminal.
     */
#if defined(ALWAYS_USE_GUI) || defined(FEAT_GUI_X11) || defined(FEAT_GUI_GTK) \
	|| defined(VIMDLL)
    if (gui.starting
# ifdef FEAT_GUI_GTK
	    && !isatty(2)
# endif
	    )
	params.want_full_screen = FALSE;
#endif

    /*
     * mch_init() sets up the terminal (window) for use.  This must be
     * done after resetting full_screen, otherwise it may move the cursor.
     * Note that we may use mch_exit() before mch_init()!
     */
    mch_init();
    TIME_MSG("shell init");

#ifdef USE_XSMP
    /*
     * For want of anywhere else to do it, try to connect to xsmp here.
     * Fitting it in after gui_mch_init, but before gui_init (via termcapinit).
     * Hijacking -X 'no X connection' to also disable XSMP connection as that
     * has a similar delay upon failure.
     * Only try if SESSION_MANAGER is set to something non-null.
     */
    if (!x_no_connect)
    {
	char *p = getenv("SESSION_MANAGER");

	if (p != NULL && *p != NUL)
	{
	    xsmp_init();
	    TIME_MSG("xsmp init");
	}
    }
#endif

    /*
     * Print a warning if stdout is not a terminal.
     */
    check_tty(&params);

#ifdef _IOLBF
    if (silent_mode)
    {
	// Ensure output works usefully without a tty: buffer lines instead of
	// fully buffered.
	s_vbuf = malloc(BUFSIZ);
	if (s_vbuf != NULL)
	    setvbuf(stdout, s_vbuf, _IOLBF, BUFSIZ);
    }
#endif

    // This message comes before term inits, but after setting "silent_mode"
    // when the input is not a tty. Omit the message with --not-a-term.
    if (GARGCOUNT > 1 && !silent_mode && !is_not_a_term())
	printf(_("%d files to edit\n"), GARGCOUNT);

    if (params.want_full_screen && !silent_mode)
    {
	termcapinit(params.term);	// set terminal name and get terminal
				   // capabilities (will set full_screen)
	screen_start();		// don't know where cursor is now
	TIME_MSG("Termcap init");
    }

    /*
     * Set the default values for the options that use Rows and Columns.
     */
    ui_get_shellsize();		// inits Rows and Columns
    win_init_size();
#ifdef FEAT_DIFF
    // Set the 'diff' option now, so that it can be checked for in a .vimrc
    // file.  There is no buffer yet though.
    if (params.diff_mode)
	diff_win_options(firstwin, FALSE);
#endif

    cmdline_row = Rows - p_ch;
    msg_row = cmdline_row;
    screenalloc(FALSE);		// allocate screen buffers
    set_init_2();
    TIME_MSG("inits 2");

    msg_scroll = TRUE;
    no_wait_return = TRUE;

    init_mappings();		// set up initial mappings

    init_highlight(TRUE, FALSE); // set the default highlight groups
    TIME_MSG("init highlight");

#if defined(FEAT_TERMRESPONSE)
    init_term_props(TRUE);
#endif

#ifdef FEAT_EVAL
    // Set the break level after the terminal is initialized.
    debug_break_level = params.use_debug_break_level;
#endif

    // Reset 'loadplugins' for "-u NONE" before "--cmd" arguments.
    // Allows for setting 'loadplugins' there.
    if (params.use_vimrc != NULL
	    && (STRCMP(params.use_vimrc, "NONE") == 0
		|| STRCMP(params.use_vimrc, "DEFAULTS") == 0))
	p_lpl = FALSE;

    // Execute --cmd arguments.
    exe_pre_commands(&params);

    // Source startup scripts.
    source_startup_scripts(&params);

#ifdef FEAT_MZSCHEME
    /*
     * Newer version of MzScheme (Racket) require earlier (trampolined)
     * initialisation via scheme_main_setup.
     * Implement this by initialising it as early as possible
     * and splitting off remaining Vim main into vim_main2().
     * Do source startup scripts, so that 'mzschemedll' can be set.
     */
    return mzscheme_main();
#else
    return vim_main2();
#endif
}
#endif // NO_VIM_MAIN
#endif // PROTO

/*
 * vim_main2() is needed for FEAT_MZSCHEME, but we define it always to keep
 * things simple.
 * It is also defined when NO_VIM_MAIN is defined, but then it's empty.
 */
    int
vim_main2(void)
{
#ifndef NO_VIM_MAIN
#ifdef FEAT_EVAL
    /*
     * Read all the plugin files.
     * Only when compiled with +eval, since most plugins need it.
     */
    if (p_lpl)
    {
	char_u *rtp_copy = NULL;
	char_u *plugin_pattern = (char_u *)
# if defined(VMS) || defined(AMIGA) // VMS and Amiga don't handle the "**".
		"plugin/*.vim"
# else
		"plugin/**/*.vim"
# endif
		;

	// First add all package directories to 'runtimepath', so that their
	// autoload directories can be found.  Only if not done already with a
	// :packloadall command.
	// Make a copy of 'runtimepath', so that source_runtime does not use
	// the pack directories.
	if (!did_source_packages)
	{
	    rtp_copy = vim_strsave(p_rtp);
	    add_pack_start_dirs();
	}

	source_in_path(rtp_copy == NULL ? p_rtp : rtp_copy, plugin_pattern,
		DIP_ALL | DIP_NOAFTER, NULL);
	TIME_MSG("loading plugins");
	vim_free(rtp_copy);

	// Only source "start" packages if not done already with a :packloadall
	// command.
	if (!did_source_packages)
	    load_start_packages();
	TIME_MSG("loading packages");

	source_runtime(plugin_pattern, DIP_ALL | DIP_AFTER);
	TIME_MSG("loading after plugins");
    }
#endif

#ifdef FEAT_DIFF
    // Decide about window layout for diff mode after reading vimrc.
    if (params.diff_mode && params.window_layout == 0)
    {
	if (diffopt_horizontal())
	    params.window_layout = WIN_HOR;	// use horizontal split
	else
	    params.window_layout = WIN_VER;	// use vertical split
    }
#endif

    /*
     * Recovery mode without a file name: List swap files.
     * This uses the 'dir' option, therefore it must be after the
     * initializations.
     */
    if (recoverymode && params.fname == NULL)
    {
	recover_names(NULL, TRUE, NULL, 0, NULL);
	mch_exit(0);
    }

    /*
     * Set a few option defaults after reading .vimrc files:
     * 'title' and 'icon', Unix: 'shellpipe' and 'shellredir'.
     */
    set_init_3();
    TIME_MSG("inits 3");

    /*
     * "-n" argument: Disable swap file by setting 'updatecount' to 0.
     * Note that this overrides anything from a vimrc file.
     */
    if (params.no_swap_file)
	p_uc = 0;

#ifdef FEAT_GUI
    if (gui.starting)
    {
# if defined(UNIX) || defined(VMS)
	// When something caused a message from a vimrc script, need to output
	// an extra newline before the shell prompt.
	if (did_emsg || msg_didout)
	    putchar('\n');
# endif

	gui_start(NULL);		// will set full_screen to TRUE
	TIME_MSG("starting GUI");

	// When running "evim" or "gvim -y" we need the menus, exit if we
	// don't have them.
	if (!gui.in_use && params.evim_mode)
	    mch_exit(1);
	firstwin->w_prev_height = firstwin->w_height; // may have changed
    }
#endif

#ifdef FEAT_VIMINFO
    /*
     * Read in registers, history etc, but not marks, from the viminfo file.
     * This is where v:oldfiles gets filled.
     */
    if (*p_viminfo != NUL)
    {
	read_viminfo(NULL, VIF_WANT_INFO | VIF_GET_OLDFILES);
	TIME_MSG("reading viminfo");
    }
#endif
#ifdef FEAT_EVAL
    // It's better to make v:oldfiles an empty list than NULL.
    if (get_vim_var_list(VV_OLDFILES) == NULL)
	set_vim_var_list(VV_OLDFILES, list_alloc());
#endif

#ifdef FEAT_QUICKFIX
    /*
     * "-q errorfile": Load the error file now.
     * If the error file can't be read, exit before doing anything else.
     */
    if (params.edit_type == EDIT_QF)
    {
	char_u	*enc = NULL;

	enc = p_menc;
	if (params.use_ef != NULL)
	    set_string_option_direct((char_u *)"ef", -1,
					   params.use_ef, OPT_FREE, SID_CARG);
	vim_snprintf((char *)IObuff, IOSIZE, "cfile %s", p_ef);
	if (qf_init(NULL, p_ef, p_efm, TRUE, IObuff, enc) < 0)
	{
	    out_char('\n');
	    mch_exit(3);
	}
	TIME_MSG("reading errorfile");
    }
#endif

    /*
     * Start putting things on the screen.
     * Scroll screen down before drawing over it
     * Clear screen now, so file message will not be cleared.
     */
    starting = NO_BUFFERS;
    no_wait_return = FALSE;
    if (!exmode_active)
	msg_scroll = FALSE;

#ifdef FEAT_GUI
    /*
     * This seems to be required to make callbacks to be called now, instead
     * of after things have been put on the screen, which then may be deleted
     * when getting a resize callback.
     * For the Mac this handles putting files dropped on the Vim icon to
     * global_alist.
     */
    if (gui.in_use)
    {
	gui_wait_for_chars(50L, typebuf.tb_change_cnt);
	TIME_MSG("GUI delay");
    }
#endif

#if defined(FEAT_GUI_PHOTON) && defined(FEAT_CLIPBOARD)
    qnx_clip_init();
#endif

#if defined(MACOS_X) && defined(FEAT_CLIPBOARD)
    clip_init(TRUE);
#endif

#ifdef FEAT_XCLIPBOARD
    // Start using the X clipboard, unless the GUI was started.
# ifdef FEAT_GUI
    if (!gui.in_use)
# endif
    {
	setup_term_clip();
	TIME_MSG("setup clipboard");
    }
#endif

#ifdef FEAT_CLIENTSERVER
    // Prepare for being a Vim server.
    prepare_server(&params);
#endif

    /*
     * If "-" argument given: Read file from stdin.
     * Do this before starting Raw mode, because it may change things that the
     * writing end of the pipe doesn't like, e.g., in case stdin and stderr
     * are the same terminal: "cat | vim -".
     * Using autocommands here may cause trouble...
     */
    if (params.edit_type == EDIT_STDIN && !recoverymode)
	read_stdin();

#if defined(UNIX) || defined(VMS)
    // When switching screens and something caused a message from a vimrc
    // script, need to output an extra newline on exit.
    if ((did_emsg || msg_didout) && *T_TI != NUL)
	newline_on_exit = TRUE;
#endif

    /*
     * When done something that is not allowed or given an error message call
     * wait_return().  This must be done before starttermcap(), because it may
     * switch to another screen. It must be done after settmode(TMODE_RAW),
     * because we want to react on a single key stroke.
     * Call settmode and starttermcap here, so the T_KS and T_TI may be
     * defined by termcapinit and redefined in .exrc.
     */
    settmode(TMODE_RAW);
    TIME_MSG("setting raw mode");

    if (need_wait_return || msg_didany)
    {
	wait_return(TRUE);
	TIME_MSG("waiting for return");
    }

    starttermcap();	    // start termcap if not done by wait_return()
    TIME_MSG("start termcap");

    setmouse();				// may start using the mouse
    if (scroll_region)
	scroll_region_reset();		// In case Rows changed
    scroll_start();	// may scroll the screen to the right position

#if defined(UNIX) || defined(VMS) || defined(MACOS_X)
    term_push_title(SAVE_RESTORE_BOTH);
#endif

    /*
     * Don't clear the screen when starting in Ex mode, unless using the GUI.
     */
    if (exmode_active
#ifdef FEAT_GUI
			&& !gui.in_use
#endif
					)
	set_must_redraw(UPD_CLEAR);
    else
    {
	screenclear();			// clear screen
	TIME_MSG("clearing screen");
    }

#ifdef FEAT_CRYPT
    if (params.ask_for_key)
    {
	crypt_check_current_method();
	(void)crypt_get_key(TRUE, TRUE);
	TIME_MSG("getting crypt key");
    }
#endif

    no_wait_return = TRUE;

    /*
     * Create the requested number of windows and edit buffers in them.
     * Also does recovery if "recoverymode" set.
     */
    create_windows(&params);
    TIME_MSG("opening buffers");

#ifdef FEAT_EVAL
    // clear v:swapcommand
    set_vim_var_string(VV_SWAPCOMMAND, NULL, -1);
#endif

    // Ex starts at last line of the file
    if (exmode_active)
	curwin->w_cursor.lnum = curbuf->b_ml.ml_line_count;

    apply_autocmds(EVENT_BUFENTER, NULL, NULL, FALSE, curbuf);
    TIME_MSG("BufEnter autocommands");
    setpcmark();

#ifdef FEAT_QUICKFIX
    /*
     * When started with "-q errorfile" jump to first error now.
     */
    if (params.edit_type == EDIT_QF)
    {
	qf_jump(NULL, 0, 0, FALSE);
	TIME_MSG("jump to first error");
    }
#endif

    /*
     * If opened more than one window, start editing files in the other
     * windows.
     */
    edit_buffers(&params, start_dir);
    vim_free(start_dir);

#ifdef FEAT_DIFF
    if (params.diff_mode)
    {
	win_T	*wp;

	// set options in each window for "vimdiff".
	FOR_ALL_WINDOWS(wp)
	    diff_win_options(wp, TRUE);
    }
#endif

    /*
     * Shorten any of the filenames, but only when absolute.
     */
    shorten_fnames(FALSE);

    /*
     * Need to jump to the tag before executing the '-c command'.
     * Makes "vim -c '/return' -t main" work.
     */
    if (params.tagname != NULL)
    {
	swap_exists_did_quit = FALSE;

	vim_snprintf((char *)IObuff, IOSIZE, "ta %s", params.tagname);
	do_cmdline_cmd(IObuff);
	TIME_MSG("jumping to tag");

	// If the user doesn't want to edit the file then we quit here.
	if (swap_exists_did_quit)
	    getout(1);
    }

    // Execute any "+", "-c" and "-S" arguments.
    if (params.n_commands > 0)
	exe_commands(&params);

    // Must come before the may_req_ calls.
    starting = 0;

#if defined(FEAT_TERMRESPONSE)
    // Must be done before redrawing, puts a few characters on the screen.
    check_terminal_behavior();
#endif

    RedrawingDisabled = 0;
    redraw_all_later(UPD_NOT_VALID);
    no_wait_return = FALSE;

    // 'autochdir' has been postponed
    DO_AUTOCHDIR;

#ifdef FEAT_TERMRESPONSE
    // Requesting the termresponse is postponed until here, so that a "-c q"
    // argument doesn't make it appear in the shell Vim was started from.
    may_req_termresponse();

    may_req_bg_color();
#endif

    // start in insert mode
    if (p_im)
	need_start_insertmode = TRUE;

#ifdef FEAT_EVAL
    set_vim_var_nr(VV_VIM_DID_ENTER, 1L);
#endif
    apply_autocmds(EVENT_VIMENTER, NULL, NULL, FALSE, curbuf);
    TIME_MSG("VimEnter autocommands");

#if defined(FEAT_EVAL) && defined(FEAT_CLIPBOARD)
    // Adjust default register name for "unnamed" in 'clipboard'. Can only be
    // done after the clipboard is available and all initial commands that may
    // modify the 'clipboard' setting have run; i.e. just before entering the
    // main loop.
    reset_reg_var();
#endif

#if defined(FEAT_DIFF)
    // When a startup script or session file setup for diff'ing and
    // scrollbind, sync the scrollbind now.
    if (curwin->w_p_diff && curwin->w_p_scb)
    {
	update_topline();
	check_scrollbind((linenr_T)0, 0L);
	TIME_MSG("diff scrollbinding");
    }
#endif

#if defined(MSWIN) && (!defined(FEAT_GUI_MSWIN) || defined(VIMDLL))
# ifdef VIMDLL
    if (!gui.in_use)
# endif
	mch_set_winsize_now();	    // Allow winsize changes from now on
#endif

#if defined(FEAT_GUI)
    // When tab pages were created, may need to update the tab pages line and
    // scrollbars.  This is skipped while creating them.
    if (gui.in_use && first_tabpage->tp_next != NULL)
    {
	out_flush();
	gui_init_which_components(NULL);
	gui_update_scrollbars(TRUE);
    }
    need_mouse_correct = TRUE;
#endif

    // If ":startinsert" command used, stuff a dummy command to be able to
    // call normal_cmd(), which will then start Insert mode.
    if (restart_edit != 0)
	stuffcharReadbuff(K_NOP);

#ifdef FEAT_NETBEANS_INTG
    if (netbeansArg != NULL && strncmp("-nb", netbeansArg, 3) == 0)
    {
# ifdef FEAT_GUI
#  if !defined(FEAT_GUI_X11) && !defined(FEAT_GUI_GTK)  \
		&& !defined(FEAT_GUI_MSWIN)
	if (gui.in_use)
	{
	    mch_errmsg(_("netbeans is not supported with this GUI\n"));
	    mch_exit(2);
	}
#  endif
# endif
	// Tell the client that it can start sending commands.
	netbeans_open(netbeansArg + 3, TRUE);
    }
#endif

    // Redraw at least once, also when 'lazyredraw' is set, to make sure the
    // window title gets updated.
    do_redraw = TRUE;

    TIME_MSG("before starting main loop");

    /*
     * Call the main command loop.  This never returns.
     */
    main_loop(FALSE, FALSE);

#endif // NO_VIM_MAIN

    return 0;
}

/*
 * Initialisation shared by main() and some tests.
 */
    void
common_init(mparm_T *paramp)
{
    estack_init();
    cmdline_init();

    (void)mb_init();	// init mb_bytelen_tab[] to ones
#ifdef FEAT_EVAL
    eval_init();	// init global variables
#endif

#ifdef __QNXNTO__
    qnx_init();		// PhAttach() for clipboard, (and gui)
#endif

    /*
     * Allocate space for the generic buffers (needed for set_init_1() and
     * emsg()).
     */
    if ((IObuff = alloc(IOSIZE)) == NULL
	    || (NameBuff = alloc(MAXPATHL)) == NULL)
	mch_exit(0);
    TIME_MSG("Allocated generic buffers");

#ifdef NBDEBUG
    // Wait a moment for debugging NetBeans.  Must be after allocating
    // NameBuff.
    nbdebug_log_init("SPRO_GVIM_DEBUG", "SPRO_GVIM_DLEVEL");
    nbdebug_wait(WT_ENV | WT_WAIT | WT_STOP, "SPRO_GVIM_WAIT", 20);
    TIME_MSG("NetBeans debug wait");
#endif

#if defined(HAVE_LOCALE_H) || defined(X_LOCALE)
    /*
     * Setup to use the current locale (for ctype() and many other things).
     * NOTE: Translated messages with encodings other than latin1 will not
     * work until set_init_1() has been called!
     */
    init_locale();
    TIME_MSG("locale set");
#endif

#ifdef FEAT_GUI
    gui.dofork = TRUE;		    // default is to use fork()
#endif

    /*
     * Do a first scan of the arguments in "argv[]":
     *   -display or --display
     *   --server...
     *   --socketid
     *   --windowid
     */
    early_arg_scan(paramp);

#if defined(FEAT_GUI)
    // Prepare for possibly starting GUI sometime
    gui_prepare(&paramp->argc, paramp->argv);
    TIME_MSG("GUI prepared");
#endif

#ifdef FEAT_CLIPBOARD
    clip_init(FALSE);		// Initialise clipboard stuff
    TIME_MSG("clipboard setup");
#endif

    /*
     * Check if we have an interactive window.
     * On the Amiga: If there is no window, we open one with a newcli command
     * (needed for :! to * work). mch_check_win() will also handle the -d or
     * -dev argument.
     */
    stdout_isatty = (mch_check_win(paramp->argc, paramp->argv) != FAIL);
    TIME_MSG("window checked");

    /*
     * Allocate the first window and buffer.
     * Can't do anything without it, exit when it fails.
     */
    if (win_alloc_first() == FAIL)
	mch_exit(0);

    init_yank();		// init yank buffers

    alist_init(&global_alist);	// Init the argument list to empty.
    global_alist.id = 0;

    /*
     * Set the default values for the options.
     * NOTE: Non-latin1 translated messages are working only after this,
     * because this is where "has_mbyte" will be set, which is used by
     * msg_outtrans_len_attr().
     * First find out the home directory, needed to expand "~" in options.
     */
    init_homedir();		// find real value of $HOME
    set_init_1(paramp->clean);
    TIME_MSG("inits 1");

#ifdef FEAT_EVAL
    // set v:lang and v:ctype
    set_lang_var();

    // set v:argv
    set_argv_var(paramp->argv, paramp->argc);
#endif

#ifdef FEAT_SIGNS
    init_signs();
#endif
}

/*
 * Return TRUE when the --not-a-term argument was found.
 */
    int
is_not_a_term(void)
{
    return params.not_a_term;
}

/*
 * Return TRUE when the --not-a-term argument was found or the GUI is in use.
 */
    int
is_not_a_term_or_gui(void)
{
    return params.not_a_term
#ifdef FEAT_GUI
			    || gui.in_use
#endif
	;
}

#if defined(EXITFREE) || defined(PROTO)
    void
free_vbuf(void)
{
# ifdef _IOLBF
    if (s_vbuf != NULL)
    {
	setvbuf(stdout, NULL, _IONBF, 0);
	free(s_vbuf);
	s_vbuf = NULL;
    }
# endif
}
#endif

#if defined(FEAT_GUI) || defined(PROTO)
/*
 * If a --gui-dialog-file argument was given return the file name.
 * Otherwise return NULL.
 */
    char_u *
get_gui_dialog_file(void)
{
    return params.gui_dialog_file;
}
#endif

// When TRUE in a safe state when starting to wait for a character.
static int	was_safe = FALSE;
static oparg_T	*current_oap = NULL;

/*
 * Return TRUE if an operator was started but not finished yet.
 * Includes typing a count or a register name.
 */
    int
op_pending(void)
{
    return !(current_oap != NULL
	    && !finish_op
	    && current_oap->prev_opcount == 0
	    && current_oap->prev_count0 == 0
	    && current_oap->op_type == OP_NOP
	    && current_oap->regname == NUL);
}

/*
 * Return whether currently it is safe, assuming it was safe before (high level
 * state didn't change).
 */
    static int
is_safe_now(void)
{
    return stuff_empty()
	&& typebuf.tb_len == 0
	&& scriptin[curscript] == NULL
#ifdef FEAT_EVAL
	&& !debug_mode
#endif
	&& !global_busy;
}

/*
 * Trigger SafeState if currently in s safe state, that is "safe" is TRUE and
 * there is no typeahead.
 */
    void
may_trigger_safestate(int safe)
{
    int is_safe = safe && is_safe_now();

#ifdef FEAT_EVAL
    if (was_safe != is_safe)
	// Only log when the state changes, otherwise it happens at nearly
	// every key stroke.
	ch_log(NULL, is_safe ? "SafeState: Start triggering"
					       : "SafeState: Stop triggering");
#endif
    if (is_safe)
	apply_autocmds(EVENT_SAFESTATE, NULL, NULL, FALSE, curbuf);
    was_safe = is_safe;
}

/*
 * Something changed which causes the state possibly to be unsafe, e.g. a
 * character was typed.  It will remain unsafe until the next call to
 * may_trigger_safestate().
 */
    void
state_no_longer_safe(char *reason UNUSED)
{
#ifdef FEAT_EVAL
    if (was_safe)
	ch_log(NULL, "SafeState: reset: %s", reason);
#endif
    was_safe = FALSE;
}

#if defined(FEAT_EVAL) || defined(MESSAGE_QUEUE) || defined(PROTO)
    int
get_was_safe_state(void)
{
    return was_safe;
}
#endif

#if defined(MESSAGE_QUEUE) || defined(PROTO)
/*
 * Invoked when leaving code that invokes callbacks.  Then trigger
 * SafeStateAgain, if it was safe when starting to wait for a character.
 */
    void
may_trigger_safestateagain(void)
{
    if (!was_safe)
    {
	// If the safe state was reset in state_no_longer_safe(), e.g. because
	// of calling feedkeys(), we check if it's now safe again (all keys
	// were consumed).
	was_safe = is_safe_now();
# ifdef FEAT_EVAL
	if (was_safe)
	    ch_log(NULL, "SafeState: undo reset");
# endif
    }
    if (was_safe)
    {
# ifdef FEAT_EVAL
	// Only do this message when another message was given, otherwise we
	// get lots of them.
	if ((did_repeated_msg & REPEATED_MSG_SAFESTATE) == 0)
	{
	    int did = did_repeated_msg;

	    ch_log(NULL,
		      "SafeState: back to waiting, triggering SafeStateAgain");
	    did_repeated_msg = did | REPEATED_MSG_SAFESTATE;
	}
# endif
	apply_autocmds(EVENT_SAFESTATEAGAIN, NULL, NULL, FALSE, curbuf);
    }
# ifdef FEAT_EVAL
    else
	ch_log(NULL,
		  "SafeState: back to waiting, not triggering SafeStateAgain");
# endif
}
#endif

/*
 * Return TRUE if there is any typeahead, pending operator or command.
 */
    int
work_pending(void)
{
    return op_pending() || !is_safe_now();
}


/*
 * Main loop: Execute Normal mode commands until exiting Vim.
 * Also used to handle commands in the command-line window, until the window
 * is closed.
 * Also used to handle ":visual" command after ":global": execute Normal mode
 * commands, return when entering Ex mode.  "noexmode" is TRUE then.
 */
    void
main_loop(
    int		cmdwin,	    // TRUE when working in the command-line window
    int		noexmode)   // TRUE when return on entering Ex mode
{
    oparg_T	oa;		// operator arguments
    oparg_T	*prev_oap;	// operator arguments
    volatile int previous_got_int = FALSE;	// "got_int" was TRUE
#ifdef FEAT_CONCEAL
    // these are static to avoid a compiler warning
    static linenr_T	conceal_old_cursor_line = 0;
    static linenr_T	conceal_new_cursor_line = 0;
    static int		conceal_update_lines = FALSE;
#endif

    prev_oap = current_oap;
    current_oap = &oa;

#if defined(FEAT_X11) && defined(FEAT_XCLIPBOARD)
    // Setup to catch a terminating error from the X server.  Just ignore
    // it, restore the state and continue.  This might not always work
    // properly, but at least we don't exit unexpectedly when the X server
    // exits while Vim is running in a console.
    if (!cmdwin && !noexmode && SETJMP(x_jump_env))
    {
	State = MODE_NORMAL;
	VIsual_active = FALSE;
	got_int = TRUE;
	need_wait_return = FALSE;
	global_busy = FALSE;
	exmode_active = 0;
	skip_redraw = FALSE;
	RedrawingDisabled = 0;
	no_wait_return = 0;
	vgetc_busy = 0;
# ifdef FEAT_EVAL
	emsg_skip = 0;
# endif
	emsg_off = 0;
	setmouse();
	settmode(TMODE_RAW);
	starttermcap();
	scroll_start();
	redraw_later_clear();
    }
#endif

    clear_oparg(&oa);
    while (!cmdwin || cmdwin_result == 0)
    {
	if (stuff_empty())
	{
	    did_check_timestamps = FALSE;
	    if (need_check_timestamps)
		check_timestamps(FALSE);
	    if (need_wait_return)	// if wait_return() still needed ...
		wait_return(FALSE);	// ... call it now
	    if (need_start_insertmode && goto_im() && !VIsual_active)
	    {
		need_start_insertmode = FALSE;
		stuffReadbuff((char_u *)"i");	// start insert mode next
		// skip the fileinfo message now, because it would be shown
		// after insert mode finishes!
		need_fileinfo = FALSE;
	    }
	}

	// Reset "got_int" now that we got back to the main loop.  Except when
	// inside a ":g/pat/cmd" command, then the "got_int" needs to abort
	// the ":g" command.
	// For ":g/pat/vi" we reset "got_int" when used once.  When used
	// a second time we go back to Ex mode and abort the ":g" command.
	if (got_int)
	{
	    if (noexmode && global_busy && !exmode_active && previous_got_int)
	    {
		// Typed two CTRL-C in a row: go back to ex mode as if "Q" was
		// used and keep "got_int" set, so that it aborts ":g".
		exmode_active = EXMODE_NORMAL;
		State = MODE_NORMAL;
	    }
	    else if (!global_busy || !exmode_active)
	    {
		if (!quit_more)
		    (void)vgetc();		// flush all buffers
		got_int = FALSE;
	    }
	    previous_got_int = TRUE;
	}
	else
	    previous_got_int = FALSE;

#ifdef FEAT_EVAL
	// At the toplevel there is no exception handling.  Discard any that
	// may be hanging around (e.g. from "interrupt" at the debug prompt).
	if (did_throw && !ex_normal_busy)
	    discard_current_exception();
#endif

	if (!exmode_active)
	    msg_scroll = FALSE;
	quit_more = FALSE;

	// it's not safe unless may_trigger_safestate_main() is called
	was_safe = FALSE;

	/*
	 * If skip redraw is set (for ":" in wait_return()), don't redraw now.
	 * If there is nothing in the stuff_buffer or do_redraw is TRUE,
	 * update cursor and redraw.
	 */
	if (skip_redraw || exmode_active)
	{
	    skip_redraw = FALSE;
	    setcursor();
	    cursor_on();
	}
	else if (do_redraw || stuff_empty())
	{
#ifdef FEAT_GUI
	    // If ui_breakcheck() was used a resize may have been postponed.
	    gui_may_resize_shell();
#endif
#ifdef HAVE_DROP_FILE
	    // If files were dropped while text was locked or the curbuf was
	    // locked, this would be a good time to handle the drop.
	    handle_any_postponed_drop();
#endif
#ifdef FEAT_CONCEAL
	    if (curwin->w_p_cole == 0)
		conceal_update_lines = FALSE;
#endif

	    // Trigger CursorMoved if the cursor moved.
	    if (!finish_op && (has_cursormoved()
#ifdef FEAT_PROP_POPUP
				|| popup_visible
#endif
#ifdef FEAT_CONCEAL
				|| curwin->w_p_cole > 0
#endif
			      )
		    && !EQUAL_POS(last_cursormoved, curwin->w_cursor))
	    {
		if (has_cursormoved())
		    apply_autocmds(EVENT_CURSORMOVED, NULL, NULL,
							       FALSE, curbuf);
#ifdef FEAT_PROP_POPUP
		if (popup_visible)
		    popup_check_cursor_pos();
#endif
#ifdef FEAT_CONCEAL
		if (curwin->w_p_cole > 0)
		{
		    conceal_old_cursor_line = last_cursormoved.lnum;
		    conceal_new_cursor_line = curwin->w_cursor.lnum;
		    conceal_update_lines = TRUE;
		}
#endif
		last_cursormoved = curwin->w_cursor;
	    }

#if defined(FEAT_CONCEAL)
	    if (conceal_update_lines
		    && (conceal_old_cursor_line != conceal_new_cursor_line
			|| conceal_cursor_line(curwin)
			|| need_cursor_line_redraw))
	    {
		if (conceal_old_cursor_line != conceal_new_cursor_line
			&& conceal_old_cursor_line != 0
			&& conceal_old_cursor_line
						<= curbuf->b_ml.ml_line_count)
		    redrawWinline(curwin, conceal_old_cursor_line);
		redrawWinline(curwin, conceal_new_cursor_line);
		curwin->w_valid &= ~VALID_CROW;
		need_cursor_line_redraw = FALSE;
	    }
#endif

	    // Trigger TextChanged if b:changedtick differs.
	    if (!finish_op && has_textchanged()
		    && curbuf->b_last_changedtick != CHANGEDTICK(curbuf))
	    {
		apply_autocmds(EVENT_TEXTCHANGED, NULL, NULL, FALSE, curbuf);
		curbuf->b_last_changedtick = CHANGEDTICK(curbuf);
	    }

	    // Ensure curwin->w_topline and curwin->w_leftcol are up to date
	    // before triggering a WinScrolled autocommand.
	    update_topline();
	    validate_cursor();

	    if (!finish_op)
		may_trigger_win_scrolled_resized();

	    // If nothing is pending and we are going to wait for the user to
	    // type a character, trigger SafeState.
	    may_trigger_safestate(!op_pending() && restart_edit == 0);

#if defined(FEAT_DIFF)
	    // Updating diffs from changed() does not always work properly,
	    // esp. updating folds.  Do an update just before redrawing if
	    // needed.
	    if (curtab->tp_diff_update || curtab->tp_diff_invalid)
	    {
		ex_diffupdate(NULL);
		curtab->tp_diff_update = FALSE;
	    }

	    // Scroll-binding for diff mode may have been postponed until
	    // here.  Avoids doing it for every change.
	    if (diff_need_scrollbind)
	    {
		check_scrollbind((linenr_T)0, 0L);
		diff_need_scrollbind = FALSE;
	    }
#endif
#if defined(FEAT_FOLDING)
	    // Include a closed fold completely in the Visual area.
	    foldAdjustVisual();
#endif
#ifdef FEAT_FOLDING
	    /*
	     * When 'foldclose' is set, apply 'foldlevel' to folds that don't
	     * contain the cursor.
	     * When 'foldopen' is "all", open the fold(s) under the cursor.
	     * This may mark the window for redrawing.
	     */
	    if (hasAnyFolding(curwin) && !char_avail())
	    {
		foldCheckClose();
		if (fdo_flags & FDO_ALL)
		    foldOpenCursor();
	    }
#endif

	    // Before redrawing, make sure w_topline is correct, and w_leftcol
	    // if lines don't wrap, and w_skipcol if lines wrap.
	    update_topline();
	    validate_cursor();

	    if (VIsual_active)
		update_curbuf(UPD_INVERTED); // update inverted part
	    else if (must_redraw)
	    {
		mch_disable_flush();	// Stop issuing gui_mch_flush().
		update_screen(0);
		mch_enable_flush();
	    }
	    else if (redraw_cmdline || clear_cmdline || redraw_mode)
		showmode();
	    redraw_statuslines();
	    if (need_maketitle)
		maketitle();
#ifdef FEAT_VIMINFO
	    curbuf->b_last_used = vim_time();
#endif
	    // display message after redraw
	    if (keep_msg != NULL)
	    {
		char_u *p = vim_strsave(keep_msg);

		if (p != NULL)
		{
		    // msg_start() will set keep_msg to NULL, make a copy
		    // first.  Don't reset keep_msg, msg_attr_keep() uses it to
		    // check for duplicates.  Never put this message in
		    // history.
		    msg_hist_off = TRUE;
		    msg_attr((char *)p, keep_msg_attr);
		    msg_hist_off = FALSE;
		    vim_free(p);
		}
	    }
	    if (need_fileinfo)		// show file info after redraw
	    {
		fileinfo(FALSE, TRUE, FALSE);
		need_fileinfo = FALSE;
	    }

	    emsg_on_display = FALSE;	// can delete error message now
	    did_emsg = FALSE;
	    msg_didany = FALSE;		// reset lines_left in msg_start()
	    may_clear_sb_text();	// clear scroll-back text on next msg
	    showruler(FALSE);

	    setcursor();
	    cursor_on();

	    do_redraw = FALSE;

#ifdef STARTUPTIME
	    // Now that we have drawn the first screen all the startup stuff
	    // has been done, close any file for startup messages.
	    if (time_fd != NULL)
	    {
		TIME_MSG("first screen update");
		TIME_MSG("--- VIM STARTED ---");
		fclose(time_fd);
		time_fd = NULL;
	    }
#endif
	    // After the first screen update may start triggering WinScrolled
	    // autocmd events.  Store all the scroll positions and sizes now.
	    may_make_initial_scroll_size_snapshot();
	}
#ifdef FEAT_GUI
	if (need_mouse_correct)
	    gui_mouse_correct();
#endif

	// May request the keyboard protocol state now.
	may_send_t_RK();

	// Update w_curswant if w_set_curswant has been set.
	// Postponed until here to avoid computing w_virtcol too often.
	update_curswant();

#ifdef FEAT_EVAL
	/*
	 * May perform garbage collection when waiting for a character, but
	 * only at the very toplevel.  Otherwise we may be using a List or
	 * Dict internally somewhere.
	 * "may_garbage_collect" is reset in vgetc() which is invoked through
	 * do_exmode() and normal_cmd().
	 */
	may_garbage_collect = (!cmdwin && !noexmode);
#endif
	/*
	 * If we're invoked as ex, do a round of ex commands.
	 * Otherwise, get and execute a normal mode command.
	 */
	if (exmode_active)
	{
	    if (noexmode)   // End of ":global/path/visual" commands
		goto theend;
	    do_exmode(exmode_active == EXMODE_VIM);
	}
	else
	{
#ifdef FEAT_TERMINAL
	    if (term_use_loop()
		    && oa.op_type == OP_NOP && oa.regname == NUL
		    && !VIsual_active
		    && !skip_term_loop)
	    {
		// If terminal_loop() returns OK we got a key that is handled
		// in Normal mode.  With FAIL we first need to position the
		// cursor and the screen needs to be redrawn.
		if (terminal_loop(TRUE) == OK)
		    normal_cmd(&oa, TRUE);
	    }
	    else
#endif
	    {
#ifdef FEAT_TERMINAL
		skip_term_loop = FALSE;
#endif
		normal_cmd(&oa, TRUE);
	    }
	}
    }

theend:
    current_oap = prev_oap;
}


#if defined(USE_XSMP) || defined(FEAT_GUI) || defined(PROTO)
/*
 * Exit, but leave behind swap files for modified buffers.
 */
    void
getout_preserve_modified(int exitval)
{
# if defined(SIGHUP) && defined(SIG_IGN)
    // Ignore SIGHUP, because a dropped connection causes a read error, which
    // makes Vim exit and then handling SIGHUP causes various reentrance
    // problems.
    mch_signal(SIGHUP, SIG_IGN);
# endif

    ml_close_notmod();		    // close all not-modified buffers
    ml_sync_all(FALSE, FALSE);	    // preserve all swap files
    ml_close_all(FALSE);	    // close all memfiles, without deleting
    getout(exitval);		    // exit Vim properly
}
#endif


/*
 * Exit properly.  This is the only way to exit Vim after startup has
 * succeeded.  We are certain to exit here, no way to abort it.
 */
    void
getout(int exitval)
{
    exiting = TRUE;
#if defined(FEAT_EVAL)
    ch_log(NULL, "Exiting...");
#endif

    // When running in Ex mode an error causes us to exit with a non-zero exit
    // code.  POSIX requires this, although it's not 100% clear from the
    // standard.
    if (exmode_active)
	exitval += ex_exitval;

#ifdef FEAT_EVAL
    set_vim_var_type(VV_EXITING, VAR_NUMBER);
    set_vim_var_nr(VV_EXITING, exitval);
#endif

    // Position the cursor on the last screen line, below all the text
    if (!is_not_a_term_or_gui())
	windgoto((int)Rows - 1, 0);

#ifdef FEAT_EVAL
    // Invoked all deferred functions in the function stack.
    invoke_all_defer();
#endif

#if defined(FEAT_EVAL) || defined(FEAT_SYN_HL)
    // Optionally print hashtable efficiency.
    hash_debug_results();
#endif

#ifdef FEAT_GUI
    msg_didany = FALSE;
#endif

    if (v_dying <= 1)
    {
	tabpage_T	*tp;
	tabpage_T	*next_tp;
	buf_T		*buf;
	win_T		*wp;
	int		unblock = 0;

	// Trigger BufWinLeave for all windows, but only once per buffer.
	for (tp = first_tabpage; tp != NULL; tp = next_tp)
	{
	    next_tp = tp->tp_next;
	    FOR_ALL_WINDOWS_IN_TAB(tp, wp)
	    {
		if (wp->w_buffer == NULL || !buf_valid(wp->w_buffer))
		    // Autocmd must have close the buffer already, skip.
		    continue;
		buf = wp->w_buffer;
		if (CHANGEDTICK(buf) != -1)
		{
		    bufref_T bufref;

		    set_bufref(&bufref, buf);
		    apply_autocmds(EVENT_BUFWINLEAVE, buf->b_fname,
						    buf->b_fname, FALSE, buf);
		    if (bufref_valid(&bufref))
			CHANGEDTICK(buf) = -1;  // note we did it already

		    // start all over, autocommands may mess up the lists
		    next_tp = first_tabpage;
		    break;
		}
	    }
	}

	// Trigger BufUnload for buffers that are loaded
	FOR_ALL_BUFFERS(buf)
	    if (buf->b_ml.ml_mfp != NULL)
	    {
		bufref_T bufref;

		set_bufref(&bufref, buf);
		apply_autocmds(EVENT_BUFUNLOAD, buf->b_fname, buf->b_fname,
								  FALSE, buf);
		if (!bufref_valid(&bufref))
		    // autocmd deleted the buffer
		    break;
	    }

	// deathtrap() blocks autocommands, but we do want to trigger
	// VimLeavePre.
	if (is_autocmd_blocked())
	{
	    unblock_autocmds();
	    ++unblock;
	}
	apply_autocmds(EVENT_VIMLEAVEPRE, NULL, NULL, FALSE, curbuf);
	if (unblock)
	    block_autocmds();
    }

#ifdef FEAT_VIMINFO
    if (*p_viminfo != NUL)
	// Write out the registers, history, marks etc, to the viminfo file
	write_viminfo(NULL, FALSE);
#endif

    if (v_dying <= 1)
    {
	int	unblock = 0;

	// deathtrap() blocks autocommands, but we do want to trigger VimLeave.
	if (is_autocmd_blocked())
	{
	    unblock_autocmds();
	    ++unblock;
	}
	apply_autocmds(EVENT_VIMLEAVE, NULL, NULL, FALSE, curbuf);
	if (unblock)
	    block_autocmds();
    }

#ifdef FEAT_PROFILE
    profile_dump();
#endif

    if (did_emsg
#ifdef FEAT_GUI
	    || (gui.in_use && msg_didany && p_verbose > 0)
#endif
	    )
    {
	// give the user a chance to read the (error) message
	no_wait_return = FALSE;
	wait_return(FALSE);
    }

    // Position the cursor again, the autocommands may have moved it
    if (!is_not_a_term_or_gui())
	windgoto((int)Rows - 1, 0);

#ifdef FEAT_JOB_CHANNEL
    job_stop_on_exit();
#endif
#ifdef FEAT_LUA
    lua_end();
#endif
#ifdef FEAT_MZSCHEME
    mzscheme_end();
#endif
#ifdef FEAT_TCL
    tcl_end();
#endif
#ifdef FEAT_RUBY
    ruby_end();
#endif
#ifdef FEAT_PYTHON
    python_end();
#endif
#ifdef FEAT_PYTHON3
    python3_end();
#endif
#ifdef FEAT_PERL
    perl_end();
#endif
#if defined(USE_ICONV) && defined(DYNAMIC_ICONV)
    iconv_end();
#endif
#ifdef FEAT_NETBEANS_INTG
    netbeans_end();
#endif
#ifdef FEAT_CSCOPE
    cs_end();
#endif
#ifdef FEAT_EVAL
    if (garbage_collect_at_exit)
	garbage_collect(FALSE);
#endif
#ifdef MSWIN
    free_cmd_argsW();
#endif

    mch_exit(exitval);
}

/*
 * Get the name of the display, before gui_prepare() removes it from
 * argv[].  Used for the xterm-clipboard display.
 *
 * Also find the --server... arguments and --socketid and --windowid
 */
    static void
early_arg_scan(mparm_T *parmp UNUSED)
{
#if defined(FEAT_XCLIPBOARD) || defined(FEAT_CLIENTSERVER) \
	|| !defined(FEAT_NETBEANS_INTG)
    int		argc = parmp->argc;
    char	**argv = parmp->argv;
    int		i;

    for (i = 1; i < argc; i++)
    {
	if (STRCMP(argv[i], "--") == 0)
	    break;
# ifdef FEAT_XCLIPBOARD
	else if (STRICMP(argv[i], "-display") == 0
#  if defined(FEAT_GUI_GTK)
		|| STRICMP(argv[i], "--display") == 0
#  endif
		)
	{
	    if (i == argc - 1)
		mainerr_arg_missing((char_u *)argv[i]);
	    xterm_display = argv[++i];
	}
# endif
# ifdef FEAT_CLIENTSERVER
	else if (STRICMP(argv[i], "--servername") == 0)
	{
	    if (i == argc - 1)
		mainerr_arg_missing((char_u *)argv[i]);
	    parmp->serverName_arg = (char_u *)argv[++i];
	}
	else if (STRICMP(argv[i], "--serverlist") == 0)
	    parmp->serverArg = TRUE;
	else if (STRNICMP(argv[i], "--remote", 8) == 0)
	{
	    parmp->serverArg = TRUE;
#  ifdef FEAT_GUI
	    if (strstr(argv[i], "-wait") != 0)
		// don't fork() when starting the GUI to edit files ourself
		gui.dofork = FALSE;
#  endif
	}
# endif

# if defined(FEAT_GUI_GTK) || defined(FEAT_GUI_MSWIN)
#  ifdef FEAT_GUI_MSWIN
	else if (STRICMP(argv[i], "--windowid") == 0)
#  else
	else if (STRICMP(argv[i], "--socketid") == 0)
#  endif
	{
	    long_u	id;
	    int		count;

	    if (i == argc - 1)
		mainerr_arg_missing((char_u *)argv[i]);
	    if (STRNICMP(argv[i+1], "0x", 2) == 0)
		count = sscanf(&(argv[i + 1][2]), SCANF_HEX_LONG_U, &id);
	    else
		count = sscanf(argv[i + 1], SCANF_DECIMAL_LONG_U, &id);
	    if (count != 1)
		mainerr(ME_INVALID_ARG, (char_u *)argv[i]);
	    else
#  ifdef FEAT_GUI_MSWIN
		win_socket_id = id;
#  else
		gtk_socket_id = id;
#  endif
	    i++;
	}
# endif
# ifdef FEAT_GUI_GTK
	else if (STRICMP(argv[i], "--echo-wid") == 0)
	    echo_wid_arg = TRUE;
# endif
# ifndef FEAT_NETBEANS_INTG
	else if (strncmp(argv[i], "-nb", (size_t)3) == 0)
	{
	    mch_errmsg(_("'-nb' cannot be used: not enabled at compile time\n"));
	    mch_exit(2);
	}
# endif

    }
#endif
}

#ifndef NO_VIM_MAIN
/*
 * Get a (optional) count for a Vim argument.
 */
    static int
get_number_arg(
    char_u	*p,	    // pointer to argument
    int		*idx,	    // index in argument, is incremented
    int		def)	    // default value
{
    if (vim_isdigit(p[*idx]))
    {
	def = atoi((char *)&(p[*idx]));
	while (vim_isdigit(p[*idx]))
	    *idx = *idx + 1;
    }
    return def;
}

/*
 * Check for: [r][e][g][vi|vim|view][diff][ex[im]]  (sort of)
 * If the executable name starts with "r" we disable shell commands.
 * If the next character is "e" we run in Easy mode.
 * If the next character is "g" we run the GUI version.
 * If the next characters are "view" we start in readonly mode.
 * If the next characters are "diff" or "vimdiff" we start in diff mode.
 * If the next characters are "ex" we start in Ex mode.  If it's followed
 * by "im" use improved Ex mode.
 */
    static void
parse_command_name(mparm_T *parmp)
{
    char_u	*initstr;

    initstr = gettail((char_u *)parmp->argv[0]);

#ifdef FEAT_EVAL
    set_vim_var_string(VV_PROGNAME, initstr, -1);
    set_progpath((char_u *)parmp->argv[0]);
#endif

    if (TOLOWER_ASC(initstr[0]) == 'r')
    {
	restricted = TRUE;
	++initstr;
    }

    // Use evim mode for "evim" and "egvim", not for "editor".
    if (TOLOWER_ASC(initstr[0]) == 'e'
	    && (TOLOWER_ASC(initstr[1]) == 'v'
		|| TOLOWER_ASC(initstr[1]) == 'g'))
    {
#ifdef FEAT_GUI
	gui.starting = TRUE;
#endif
	parmp->evim_mode = TRUE;
	++initstr;
    }

    // "gvim" starts the GUI.  Also accept "Gvim" for MS-Windows.
    if (TOLOWER_ASC(initstr[0]) == 'g')
    {
	main_start_gui();
#ifdef FEAT_GUI
	++initstr;
#endif
#ifdef GUI_MAY_SPAWN
	gui.dospawn = FALSE;	// No need to spawn a new process.
#endif
    }
#ifdef GUI_MAY_SPAWN
    else
	gui.dospawn = TRUE;	// Not "gvim". Need to spawn gvim.exe.
#endif


    if (STRNICMP(initstr, "view", 4) == 0)
    {
	readonlymode = TRUE;
	curbuf->b_p_ro = TRUE;
	p_uc = 10000;			// don't update very often
	initstr += 4;
    }
    else if (STRNICMP(initstr, "vim", 3) == 0)
	initstr += 3;

    // Catch "[r][g]vimdiff" and "[r][g]viewdiff".
    if (STRICMP(initstr, "diff") == 0)
    {
#ifdef FEAT_DIFF
	parmp->diff_mode = TRUE;
#else
	mch_errmsg(_("This Vim was not compiled with the diff feature."));
	mch_errmsg("\n");
	mch_exit(2);
#endif
    }

    // Checking for "ex" here may catch some weird names, such as "vimex" or
    // "viewex", we assume the user knows that.
    if (STRNICMP(initstr, "ex", 2) == 0)
    {
	if (STRNICMP(initstr + 2, "im", 2) == 0)
	    exmode_active = EXMODE_VIM;
	else
	    exmode_active = EXMODE_NORMAL;
	change_compatible(TRUE);	// set 'compatible'
    }
}

/*
 * Scan the command line arguments.
 */
    static void
command_line_scan(mparm_T *parmp)
{
    int		argc = parmp->argc;
    char	**argv = parmp->argv;
    int		argv_idx;		// index in argv[n][]
    int		had_minmin = FALSE;	// found "--" argument
    int		want_argument;		// option argument with argument
    int		c;
    char_u	*p = NULL;
    long	n;

    --argc;
    ++argv;
    argv_idx = 1;	    // active option letter is argv[0][argv_idx]
    while (argc > 0)
    {
	/*
	 * "+" or "+{number}" or "+/{pat}" or "+{command}" argument.
	 */
	if (argv[0][0] == '+' && !had_minmin)
	{
	    if (parmp->n_commands >= MAX_ARG_CMDS)
		mainerr(ME_EXTRA_CMD, NULL);
	    argv_idx = -1;	    // skip to next argument
	    if (argv[0][1] == NUL)
		parmp->commands[parmp->n_commands++] = (char_u *)"$";
	    else
		parmp->commands[parmp->n_commands++] = (char_u *)&(argv[0][1]);
	}

	/*
	 * Optional argument.
	 */
	else if (argv[0][0] == '-' && !had_minmin)
	{
	    want_argument = FALSE;
	    c = argv[0][argv_idx++];
#ifdef VMS
	    /*
	     * VMS only uses upper case command lines.  Interpret "-X" as "-x"
	     * and "-/X" as "-X".
	     */
	    if (c == '/')
	    {
		c = argv[0][argv_idx++];
		c = TOUPPER_ASC(c);
	    }
	    else
		c = TOLOWER_ASC(c);
#endif
	    switch (c)
	    {
	    case NUL:		// "vim -"  read from stdin
				// "ex -" silent mode
		if (exmode_active)
		    silent_mode = TRUE;
		else
		{
		    if (parmp->edit_type != EDIT_NONE)
			mainerr(ME_TOO_MANY_ARGS, (char_u *)argv[0]);
		    parmp->edit_type = EDIT_STDIN;
		    read_cmd_fd = 2;	// read from stderr instead of stdin
		}
		argv_idx = -1;		// skip to next argument
		break;

	    case '-':		// "--" don't take any more option arguments
				// "--help" give help message
				// "--version" give version message
				// "--clean" clean context
				// "--literal" take files literally
				// "--startuptime fname" write timing info
				// "--log fname" start logging early
				// "--nofork" don't fork
				// "--not-a-term" don't warn for not a term
				// "--gui-dialog-file fname" write dialog text
				// "--ttyfail" exit if not a term
				// "--noplugin[s]" skip plugins
				// "--cmd <cmd>" execute cmd before vimrc
		if (STRICMP(argv[0] + argv_idx, "help") == 0)
		    usage();
		else if (STRICMP(argv[0] + argv_idx, "version") == 0)
		{
		    Columns = 80;	// need to init Columns
		    info_message = TRUE; // use mch_msg(), not mch_errmsg()
#if defined(FEAT_GUI) && !defined(ALWAYS_USE_GUI) && !defined(VIMDLL)
		    gui.starting = FALSE; // not starting GUI, will exit
#endif
		    list_version();
		    msg_putchar('\n');
		    msg_didout = FALSE;
		    mch_exit(0);
		}
		else if (STRNICMP(argv[0] + argv_idx, "clean", 5) == 0)
		{
		    parmp->use_vimrc = (char_u *)"DEFAULTS";
#ifdef FEAT_GUI
		    use_gvimrc = (char_u *)"NONE";
#endif
		    parmp->clean = TRUE;
		    set_option_value_give_err((char_u *)"vif",
						      0L, (char_u *)"NONE", 0);
		}
		else if (STRNICMP(argv[0] + argv_idx, "literal", 7) == 0)
		{
#ifdef EXPAND_FILENAMES
		    parmp->literal = TRUE;
#endif
		}
		else if (STRNICMP(argv[0] + argv_idx, "nofork", 6) == 0)
		{
#ifdef FEAT_GUI
		    gui.dofork = FALSE;	// don't fork() when starting GUI
#endif
		}
		else if (STRNICMP(argv[0] + argv_idx, "noplugin", 8) == 0)
		    p_lpl = FALSE;
		else if (STRNICMP(argv[0] + argv_idx, "not-a-term", 10) == 0)
		    parmp->not_a_term = TRUE;
		else if (STRNICMP(argv[0] + argv_idx, "gui-dialog-file", 15)
									 == 0)
		{
		    want_argument = TRUE;
		    argv_idx += 15;
		}
		else if (STRNICMP(argv[0] + argv_idx, "ttyfail", 7) == 0)
		    parmp->tty_fail = TRUE;
		else if (STRNICMP(argv[0] + argv_idx, "cmd", 3) == 0)
		{
		    want_argument = TRUE;
		    argv_idx += 3;
		}
		else if (STRNICMP(argv[0] + argv_idx, "startuptime", 11) == 0)
		{
		    want_argument = TRUE;
		    argv_idx += 11;
		}
		else if (STRNICMP(argv[0] + argv_idx, "log", 3) == 0)
		{
		    want_argument = TRUE;
		    argv_idx += 3;
		}
#ifdef FEAT_CLIENTSERVER
		else if (STRNICMP(argv[0] + argv_idx, "serverlist", 10) == 0)
		    ; // already processed -- no arg
		else if (STRNICMP(argv[0] + argv_idx, "servername", 10) == 0
		       || STRNICMP(argv[0] + argv_idx, "serversend", 10) == 0)
		{
		    // already processed -- snatch the following arg
		    if (argc > 1)
		    {
			--argc;
			++argv;
		    }
		}
#endif
#if defined(FEAT_GUI_GTK) || defined(FEAT_GUI_MSWIN)
# ifdef FEAT_GUI_GTK
		else if (STRNICMP(argv[0] + argv_idx, "socketid", 8) == 0)
# else
		else if (STRNICMP(argv[0] + argv_idx, "windowid", 8) == 0)
# endif
		{
		    // already processed -- snatch the following arg
		    if (argc > 1)
		    {
			--argc;
			++argv;
		    }
		}
#endif
#ifdef FEAT_GUI_GTK
		else if (STRNICMP(argv[0] + argv_idx, "echo-wid", 8) == 0)
		{
		    // already processed, skip
		}
#endif
		else
		{
		    if (argv[0][argv_idx])
			mainerr(ME_UNKNOWN_OPTION, (char_u *)argv[0]);
		    had_minmin = TRUE;
		}
		if (!want_argument)
		    argv_idx = -1;	// skip to next argument
		break;

	    case 'A':		// "-A" start in Arabic mode
#ifdef FEAT_ARABIC
		set_option_value_give_err((char_u *)"arabic", 1L, NULL, 0);
#else
		mch_errmsg(_(e_arabic_cannot_be_used_not_enabled_at_compile_time));
		mch_exit(2);
#endif
		break;

	    case 'b':		// "-b" binary mode
		// Needs to be effective before expanding file names, because
		// for Win32 this makes us edit a shortcut file itself,
		// instead of the file it links to.
		set_options_bin(curbuf->b_p_bin, 1, 0);
		curbuf->b_p_bin = 1;	    // binary file I/O
		break;

	    case 'C':		// "-C"  Compatible
		change_compatible(TRUE);
		has_dash_c_arg = TRUE;
		break;

	    case 'e':		// "-e" Ex mode
		exmode_active = EXMODE_NORMAL;
		break;

	    case 'E':		// "-E" Improved Ex mode
		exmode_active = EXMODE_VIM;
		break;

	    case 'f':		// "-f"  GUI: run in foreground.  Amiga: open
				// window directly, not with newcli
#ifdef FEAT_GUI
		gui.dofork = FALSE;	// don't fork() when starting GUI
#endif
		break;

	    case 'g':		// "-g" start GUI
		main_start_gui();
		break;

	    case 'F':		// "-F" was for Farsi mode
		mch_errmsg(_(e_farsi_support_has_been_removed));
		mch_exit(2);
		break;

	    case '?':		// "-?" give help message (for MS-Windows)
	    case 'h':		// "-h" give help message
#ifdef FEAT_GUI_GNOME
		// Tell usage() to exit for "gvim".
		gui.starting = FALSE;
#endif
		usage();
		break;

	    case 'H':		// "-H" start in Hebrew mode: rl + hkmap set
#ifdef FEAT_RIGHTLEFT
		p_hkmap = TRUE;
		set_option_value_give_err((char_u *)"rl", 1L, NULL, 0);
#else
		mch_errmsg(_(e_hebrew_cannot_be_used_not_enabled_at_compile_time));
		mch_exit(2);
#endif
		break;

	    case 'l':		// "-l" lisp mode, 'lisp' and 'showmatch' on
		set_option_value_give_err((char_u *)"lisp", 1L, NULL, 0);
		p_sm = TRUE;
		break;

	    case 'M':		// "-M"  no changes or writing of files
		reset_modifiable();
		// FALLTHROUGH

	    case 'm':		// "-m"  no writing of files
		p_write = FALSE;
		break;

	    case 'y':		// "-y"  easy mode
#ifdef FEAT_GUI
		gui.starting = TRUE;	// start GUI a bit later
#endif
		parmp->evim_mode = TRUE;
		break;

	    case 'N':		// "-N"  Nocompatible
		change_compatible(FALSE);
		break;

	    case 'n':		// "-n" no swap file
#ifdef FEAT_NETBEANS_INTG
		// checking for "-nb", netbeans parameters
		if (argv[0][argv_idx] == 'b')
		{
		    netbeansArg = argv[0];
		    argv_idx = -1;	    // skip to next argument
		}
		else
#endif
		parmp->no_swap_file = TRUE;
		break;

	    case 'p':		// "-p[N]" open N tab pages
#ifdef TARGET_API_MAC_OSX
		// For some reason on MacOS X, an argument like:
		// -psn_0_10223617 is passed in when invoke from Finder
		// or with the 'open' command
		if (argv[0][argv_idx] == 's')
		{
		    argv_idx = -1; // bypass full -psn
		    main_start_gui();
		    break;
		}
#endif
		// default is 0: open window for each file
		parmp->window_count = get_number_arg((char_u *)argv[0],
								&argv_idx, 0);
		parmp->window_layout = WIN_TABS;
		break;

	    case 'o':		// "-o[N]" open N horizontal split windows
		// default is 0: open window for each file
		parmp->window_count = get_number_arg((char_u *)argv[0],
								&argv_idx, 0);
		parmp->window_layout = WIN_HOR;
		break;

		case 'O':	// "-O[N]" open N vertical split windows
		// default is 0: open window for each file
		parmp->window_count = get_number_arg((char_u *)argv[0],
								&argv_idx, 0);
		parmp->window_layout = WIN_VER;
		break;

#ifdef FEAT_QUICKFIX
	    case 'q':		// "-q" QuickFix mode
		if (parmp->edit_type != EDIT_NONE)
		    mainerr(ME_TOO_MANY_ARGS, (char_u *)argv[0]);
		parmp->edit_type = EDIT_QF;
		if (argv[0][argv_idx])		// "-q{errorfile}"
		{
		    parmp->use_ef = (char_u *)argv[0] + argv_idx;
		    argv_idx = -1;
		}
		else if (argc > 1)		// "-q {errorfile}"
		    want_argument = TRUE;
		break;
#endif

	    case 'R':		// "-R" readonly mode
		readonlymode = TRUE;
		curbuf->b_p_ro = TRUE;
		p_uc = 10000;			// don't update very often
		break;

	    case 'r':		// "-r" recovery mode
	    case 'L':		// "-L" recovery mode
		recoverymode = 1;
		break;

	    case 's':
		if (exmode_active)	// "-s" silent (batch) mode
		    silent_mode = TRUE;
		else		// "-s {scriptin}" read from script file
		    want_argument = TRUE;
		break;

	    case 't':		// "-t {tag}" or "-t{tag}" jump to tag
		if (parmp->edit_type != EDIT_NONE)
		    mainerr(ME_TOO_MANY_ARGS, (char_u *)argv[0]);
		parmp->edit_type = EDIT_TAG;
		if (argv[0][argv_idx])		// "-t{tag}"
		{
		    parmp->tagname = (char_u *)argv[0] + argv_idx;
		    argv_idx = -1;
		}
		else				// "-t {tag}"
		    want_argument = TRUE;
		break;

#ifdef FEAT_EVAL
	    case 'D':		// "-D"		Debugging
		parmp->use_debug_break_level = 9999;
		break;
#endif
#ifdef FEAT_DIFF
	    case 'd':		// "-d"		'diff'
# ifdef AMIGA
		// check for "-dev {device}"
		if (argv[0][argv_idx] == 'e' && argv[0][argv_idx + 1] == 'v')
		    want_argument = TRUE;
		else
# endif
		    parmp->diff_mode = TRUE;
		break;
#endif
	    case 'V':		// "-V{N}"	Verbose level
		// default is 10: a little bit verbose
		p_verbose = get_number_arg((char_u *)argv[0], &argv_idx, 10);
		if (argv[0][argv_idx] != NUL)
		{
		    set_option_value_give_err((char_u *)"verbosefile",
					  0L, (char_u *)argv[0] + argv_idx, 0);
		    argv_idx = (int)STRLEN(argv[0]);
		}
		break;

	    case 'v':		// "-v"  Vi-mode (as if called "vi")
		exmode_active = 0;
#if defined(FEAT_GUI) && !defined(VIMDLL)
		gui.starting = FALSE;	// don't start GUI
#endif
		break;

	    case 'w':		// "-w{number}"	set window height
				// "-w {scriptout}"	write to script
		if (vim_isdigit(((char_u *)argv[0])[argv_idx]))
		{
		    n = get_number_arg((char_u *)argv[0], &argv_idx, 10);
		    set_option_value_give_err((char_u *)"window", n, NULL, 0);
		    break;
		}
		want_argument = TRUE;
		break;

#ifdef FEAT_CRYPT
	    case 'x':		// "-x"  encrypted reading/writing of files
		parmp->ask_for_key = TRUE;
		break;
#endif

	    case 'X':		// "-X"  don't connect to X server
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_X11)
		x_no_connect = TRUE;
#endif
		break;

	    case 'Z':		// "-Z"  restricted mode
		restricted = TRUE;
		break;

	    case 'c':		// "-c{command}" or "-c {command}" execute
				// command
		if (argv[0][argv_idx] != NUL)
		{
		    if (parmp->n_commands >= MAX_ARG_CMDS)
			mainerr(ME_EXTRA_CMD, NULL);
		    parmp->commands[parmp->n_commands++] = (char_u *)argv[0]
								   + argv_idx;
		    argv_idx = -1;
		    break;
		}
		// FALLTHROUGH
	    case 'S':		// "-S {file}" execute Vim script
	    case 'i':		// "-i {viminfo}" use for viminfo
#ifndef FEAT_DIFF
	    case 'd':		// "-d {device}" device (for Amiga)
#endif
	    case 'T':		// "-T {terminal}" terminal name
	    case 'u':		// "-u {vimrc}" vim inits file
	    case 'U':		// "-U {gvimrc}" gvim inits file
	    case 'W':		// "-W {scriptout}" overwrite
#ifdef FEAT_GUI_MSWIN
	    case 'P':		// "-P {parent title}" MDI parent
#endif
		want_argument = TRUE;
		break;

	    default:
		mainerr(ME_UNKNOWN_OPTION, (char_u *)argv[0]);
	    }

	    /*
	     * Handle option arguments with argument.
	     */
	    if (want_argument)
	    {
		/*
		 * Check for garbage immediately after the option letter.
		 */
		if (argv[0][argv_idx] != NUL)
		    mainerr(ME_GARBAGE, (char_u *)argv[0]);

		--argc;
		if (argc < 1 && c != 'S')  // -S has an optional argument
		    mainerr_arg_missing((char_u *)argv[0]);
		++argv;
		argv_idx = -1;

		switch (c)
		{
		case 'c':	// "-c {command}" execute command
		case 'S':	// "-S {file}" execute Vim script
		    if (parmp->n_commands >= MAX_ARG_CMDS)
			mainerr(ME_EXTRA_CMD, NULL);
		    if (c == 'S')
		    {
			char	*a;

			if (argc < 1)
			    // "-S" without argument: use default session file
			    // name.
			    a = SESSION_FILE;
			else if (argv[0][0] == '-')
			{
			    // "-S" followed by another option: use default
			    // session file name.
			    a = SESSION_FILE;
			    ++argc;
			    --argv;
			}
			else
			    a = argv[0];
			p = alloc(STRLEN(a) + 4);
			if (p == NULL)
			    mch_exit(2);
			sprintf((char *)p, "so %s", a);
			parmp->cmds_tofree[parmp->n_commands] = TRUE;
			parmp->commands[parmp->n_commands++] = p;
		    }
		    else
			parmp->commands[parmp->n_commands++] =
							    (char_u *)argv[0];
		    break;

		case '-':
		    if (argv[-1][2] == 'c')
		    {
			// "--cmd {command}" execute command
			if (parmp->n_pre_commands >= MAX_ARG_CMDS)
			    mainerr(ME_EXTRA_CMD, NULL);
			parmp->pre_commands[parmp->n_pre_commands++] =
							    (char_u *)argv[0];
		    }
		    // --gui-dialog-file fname
		    if (argv[-1][2] == 'g')
		    {
			// without GUI ignore the argument
#ifdef FEAT_GUI
			parmp->gui_dialog_file = (char_u *)argv[0];
#endif
		    }

		    // "--startuptime <file>" already handled
		    // "--log <file>" already handled
		    break;

	    //	case 'd':   -d {device} is handled in mch_check_win() for the
	    //		    Amiga

#ifdef FEAT_QUICKFIX
		case 'q':	// "-q {errorfile}" QuickFix mode
		    parmp->use_ef = (char_u *)argv[0];
		    break;
#endif

		case 'i':	// "-i {viminfo}" use for viminfo
		    set_option_value_give_err((char_u *)"vif",
						     0L, (char_u *)argv[0], 0);
		    break;

		case 's':	// "-s {scriptin}" read from script file
		    if (scriptin[0] != NULL)
		    {
scripterror:
			mch_errmsg(_("Attempt to open script file again: \""));
			mch_errmsg(argv[-1]);
			mch_errmsg(" ");
			mch_errmsg(argv[0]);
			mch_errmsg("\"\n");
			mch_exit(2);
		    }
		    if ((scriptin[0] = mch_fopen(argv[0], READBIN)) == NULL)
		    {
			mch_errmsg(_("Cannot open for reading: \""));
			mch_errmsg(argv[0]);
			mch_errmsg("\"\n");
			mch_exit(2);
		    }
		    if (save_typebuf() == FAIL)
			mch_exit(2);	// out of memory
		    break;

		case 't':	// "-t {tag}"
		    parmp->tagname = (char_u *)argv[0];
		    break;

		case 'T':	// "-T {terminal}" terminal name
		    /*
		     * The -T term argument is always available and when
		     * HAVE_TERMLIB is supported it overrides the environment
		     * variable TERM.
		     */
#ifdef FEAT_GUI
		    if (term_is_gui((char_u *)argv[0]))
			gui.starting = TRUE;	// start GUI a bit later
		    else
#endif
			parmp->term = (char_u *)argv[0];
		    break;

		case 'u':	// "-u {vimrc}" vim inits file
		    parmp->use_vimrc = (char_u *)argv[0];
		    break;

		case 'U':	// "-U {gvimrc}" gvim inits file
#ifdef FEAT_GUI
		    use_gvimrc = (char_u *)argv[0];
#endif
		    break;

		case 'w':	// "-w {nr}" 'window' value
				// "-w {scriptout}" append to script file
		    if (vim_isdigit(*((char_u *)argv[0])))
		    {
			argv_idx = 0;
			n = get_number_arg((char_u *)argv[0], &argv_idx, 10);
			set_option_value_give_err((char_u *)"window",
								   n, NULL, 0);
			argv_idx = -1;
			break;
		    }
		    // FALLTHROUGH
		case 'W':	// "-W {scriptout}" overwrite script file
		    if (scriptout != NULL)
			goto scripterror;
		    if ((scriptout = mch_fopen(argv[0],
				    c == 'w' ? APPENDBIN : WRITEBIN)) == NULL)
		    {
			mch_errmsg(_("Cannot open for script output: \""));
			mch_errmsg(argv[0]);
			mch_errmsg("\"\n");
			mch_exit(2);
		    }
		    break;

#ifdef FEAT_GUI_MSWIN
		case 'P':		// "-P {parent title}" MDI parent
		    gui_mch_set_parent(argv[0]);
		    break;
#endif
		}
	    }
	}

	/*
	 * File name argument.
	 */
	else
	{
	    argv_idx = -1;	    // skip to next argument

	    // Check for only one type of editing.
	    if (parmp->edit_type != EDIT_NONE && parmp->edit_type != EDIT_FILE)
		mainerr(ME_TOO_MANY_ARGS, (char_u *)argv[0]);
	    parmp->edit_type = EDIT_FILE;

#ifdef MSWIN
	    // Remember if the argument was a full path before changing
	    // slashes to backslashes.
	    if (argv[0][0] != NUL && argv[0][1] == ':' && argv[0][2] == '\\')
		parmp->full_path = TRUE;
#endif

	    // Add the file to the global argument list.
	    if (ga_grow(&global_alist.al_ga, 1) == FAIL
		    || (p = vim_strsave((char_u *)argv[0])) == NULL)
		mch_exit(2);
#ifdef FEAT_DIFF
	    if (parmp->diff_mode && mch_isdir(p) && GARGCOUNT > 0
				      && !mch_isdir(alist_name(&GARGLIST[0])))
	    {
		char_u	    *r;

		r = concat_fnames(p, gettail(alist_name(&GARGLIST[0])), TRUE);
		if (r != NULL)
		{
		    vim_free(p);
		    p = r;
		}
	    }
#endif
#ifdef __CYGWIN__
	    /*
	     * If vim is invoked by non-Cygwin tools, convert away any
	     * DOS paths, so things like .swp files are created correctly.
	     * Look for evidence of non-Cygwin paths before we bother.
	     * This is only for when using the Unix files.
	     */
	    if (vim_strpbrk(p, "\\:") != NULL && !path_with_url(p))
	    {
		char posix_path[MAXPATHL];

# if CYGWIN_VERSION_DLL_MAJOR >= 1007
		cygwin_conv_path(CCP_WIN_A_TO_POSIX, p, posix_path, MAXPATHL);
# else
		cygwin_conv_to_posix_path(p, posix_path);
# endif
		vim_free(p);
		p = vim_strsave((char_u *)posix_path);
		if (p == NULL)
		    mch_exit(2);
	    }
#endif

#ifdef USE_FNAME_CASE
	    // Make the case of the file name match the actual file.
	    fname_case(p, 0);
#endif

	    alist_add(&global_alist, p,
#ifdef EXPAND_FILENAMES
		    parmp->literal ? 2 : 0	// add buffer nr after exp.
#else
		    2		// add buffer number now and use curbuf
#endif
		    );

#ifdef MSWIN
	    {
		// Remember this argument has been added to the argument list.
		// Needed when 'encoding' is changed.
		used_file_arg(argv[0], parmp->literal, parmp->full_path,
# ifdef FEAT_DIFF
							    parmp->diff_mode
# else
							    FALSE
# endif
							    );
	    }
#endif
	}

	/*
	 * If there are no more letters after the current "-", go to next
	 * argument.  argv_idx is set to -1 when the current argument is to be
	 * skipped.
	 */
	if (argv_idx <= 0 || argv[0][argv_idx] == NUL)
	{
	    --argc;
	    ++argv;
	    argv_idx = 1;
	}
    }

#ifdef FEAT_EVAL
    // If there is a "+123" or "-c" command, set v:swapcommand to the first
    // one.
    if (parmp->n_commands > 0)
    {
	p = alloc(STRLEN(parmp->commands[0]) + 3);
	if (p != NULL)
	{
	    sprintf((char *)p, ":%s\r", parmp->commands[0]);
	    set_vim_var_string(VV_SWAPCOMMAND, p, -1);
	    vim_free(p);
	}
    }
#endif
}

/*
 * Print a warning if stdout is not a terminal.
 * When starting in Ex mode and commands come from a file, set silent_mode.
 */
    static void
check_tty(mparm_T *parmp)
{
    int		input_isatty;		// is active input a terminal?

    input_isatty = mch_input_isatty();
    if (exmode_active)
    {
	if (!input_isatty)
	    silent_mode = TRUE;
    }
    else if (parmp->want_full_screen && (!stdout_isatty || !input_isatty)
#ifdef FEAT_GUI
	    // don't want the delay when started from the desktop
	    && !gui.starting
#endif
	    && !parmp->not_a_term)
    {
#ifdef NBDEBUG
	/*
	 * This shouldn't be necessary. But if I run netbeans with the log
	 * output coming to the console and XOpenDisplay fails, I get vim
	 * trying to start with input/output to my console tty.  This fills my
	 * input buffer so fast I can't even kill the process in under 2
	 * minutes (and it beeps continuously the whole time :-)
	 */
	if (netbeans_active() && (!stdout_isatty || !input_isatty))
	{
	    mch_errmsg(_("Vim: Error: Failure to start gvim from NetBeans\n"));
	    exit(1);
	}
#endif
#if defined(MSWIN) && (!defined(FEAT_GUI_MSWIN) || defined(VIMDLL))
	if (
# ifdef VIMDLL
	    !gui.starting &&
# endif
	    is_cygpty_used())
	{
# if defined(HAVE_BIND_TEXTDOMAIN_CODESET) \
	&& defined(FEAT_GETTEXT)
	    char    *s, *tofree = NULL;

	    // Set the encoding of the error message based on $LC_ALL or
	    // other environment variables instead of 'encoding'.
	    // Note that the message is shown on a Cygwin terminal (e.g.
	    // mintty) which encoding is based on $LC_ALL or etc., not the
	    // current codepage used by normal Win32 console programs.
	    tofree = s = (char *)enc_locale_env(NULL);
	    if (s == NULL)
		s = "utf-8";	// Use "utf-8" by default.
	    (void)bind_textdomain_codeset(VIMPACKAGE, s);
	    vim_free(tofree);
# endif
	    mch_errmsg(_("Vim: Error: This version of Vim does not run in a Cygwin terminal\n"));
	    exit(1);
	}
#endif
	if (!stdout_isatty)
	    mch_errmsg(_("Vim: Warning: Output is not to a terminal\n"));
	if (!input_isatty)
	    mch_errmsg(_("Vim: Warning: Input is not from a terminal\n"));
	out_flush();
	if (parmp->tty_fail && (!stdout_isatty || !input_isatty))
	    exit(1);
	if (scriptin[0] == NULL)
	    ui_delay(2005L, TRUE);
	TIME_MSG("Warning delay");
    }
}

/*
 * Read text from stdin.
 */
    static void
read_stdin(void)
{
    int	    i;

    // When getting the ATTENTION prompt here, use a dialog
    swap_exists_action = SEA_DIALOG;

    no_wait_return = TRUE;
    i = msg_didany;
    set_buflisted(TRUE);

    // Create memfile and read from stdin.
    (void)open_buffer(TRUE, NULL, 0);

    no_wait_return = FALSE;
    msg_didany = i;
    TIME_MSG("reading stdin");

    check_swap_exists_action();

#if !(defined(AMIGA) || defined(MACOS_X))
    // Dup stdin from stderr to read commands from, so that shell commands
    // work.
    // TODO: why is this needed, even though readfile() has done this?
    close(0);
    vim_ignored = dup(2);
#endif
}

/*
 * Create the requested number of windows and edit buffers in them.
 * Also does recovery if "recoverymode" set.
 */
    static void
create_windows(mparm_T *parmp UNUSED)
{
    int		dorewind;
    int		done = 0;

    /*
     * Create the number of windows that was requested.
     */
    if (parmp->window_count == -1)	// was not set
	parmp->window_count = 1;
    if (parmp->window_count == 0)
	parmp->window_count = GARGCOUNT;
    if (parmp->window_count > 1)
    {
	// Don't change the windows if there was a command in .vimrc that
	// already split some windows
	if (parmp->window_layout == 0)
	    parmp->window_layout = WIN_HOR;
	if (parmp->window_layout == WIN_TABS)
	{
	    parmp->window_count = make_tabpages(parmp->window_count);
	    TIME_MSG("making tab pages");
	}
	else if (firstwin->w_next == NULL)
	{
	    parmp->window_count = make_windows(parmp->window_count,
					     parmp->window_layout == WIN_VER);
	    TIME_MSG("making windows");
	}
	else
	    parmp->window_count = win_count();
    }
    else
	parmp->window_count = 1;

    if (recoverymode)			// do recover
    {
	msg_scroll = TRUE;		// scroll message up
	ml_recover(TRUE);
	if (curbuf->b_ml.ml_mfp == NULL) // failed
	    getout(1);
	do_modelines(0);		// do modelines
    }
    else
    {
	/*
	 * Open a buffer for windows that don't have one yet.
	 * Commands in the .vimrc might have loaded a file or split the window.
	 * Watch out for autocommands that delete a window.
	 */
	/*
	 * Don't execute Win/Buf Enter/Leave autocommands here
	 */
	++autocmd_no_enter;
	++autocmd_no_leave;
	dorewind = TRUE;
	while (done++ < 1000)
	{
	    if (dorewind)
	    {
		if (parmp->window_layout == WIN_TABS)
		    goto_tabpage(1);
		else
		    curwin = firstwin;
	    }
	    else if (parmp->window_layout == WIN_TABS)
	    {
		if (curtab->tp_next == NULL)
		    break;
		goto_tabpage(0);
	    }
	    else
	    {
		if (curwin->w_next == NULL)
		    break;
		curwin = curwin->w_next;
	    }
	    dorewind = FALSE;
	    curbuf = curwin->w_buffer;
	    if (curbuf->b_ml.ml_mfp == NULL)
	    {
#ifdef FEAT_FOLDING
		// Set 'foldlevel' to 'foldlevelstart' if it's not negative.
		if (p_fdls >= 0)
		    curwin->w_p_fdl = p_fdls;
#endif
		// When getting the ATTENTION prompt here, use a dialog
		swap_exists_action = SEA_DIALOG;

		set_buflisted(TRUE);

		// create memfile, read file
		(void)open_buffer(FALSE, NULL, 0);

		if (swap_exists_action == SEA_QUIT)
		{
		    if (got_int || only_one_window())
		    {
			// abort selected or quit and only one window
			did_emsg = FALSE;   // avoid hit-enter prompt
			getout(1);
		    }
		    // We can't close the window, it would disturb what
		    // happens next.  Clear the file name and set the arg
		    // index to -1 to delete it later.
		    setfname(curbuf, NULL, NULL, FALSE);
		    curwin->w_arg_idx = -1;
		    swap_exists_action = SEA_NONE;
		}
		else
		    handle_swap_exists(NULL);
		dorewind = TRUE;		// start again
	    }
	    ui_breakcheck();
	    if (got_int)
	    {
		(void)vgetc();	// only break the file loading, not the rest
		break;
	    }
	}
	if (parmp->window_layout == WIN_TABS)
	    goto_tabpage(1);
	else
	    curwin = firstwin;
	curbuf = curwin->w_buffer;
	--autocmd_no_enter;
	--autocmd_no_leave;
    }
}

/*
 * If opened more than one window, start editing files in the other
 * windows.  make_windows() has already opened the windows.
 */
    static void
edit_buffers(
    mparm_T	*parmp,
    char_u	*cwd)			// current working dir
{
    int		arg_idx;		// index in argument list
    int		i;
    int		advance = TRUE;
    win_T	*win;
    char_u	*p_shm_save = NULL;

    /*
     * Don't execute Win/Buf Enter/Leave autocommands here
     */
    ++autocmd_no_enter;
    ++autocmd_no_leave;

    // When w_arg_idx is -1 remove the window (see create_windows()).
    if (curwin->w_arg_idx == -1)
    {
	win_close(curwin, TRUE);
	advance = FALSE;
    }

    arg_idx = 1;
    for (i = 1; i < parmp->window_count; ++i)
    {
	if (cwd != NULL)
	    mch_chdir((char *)cwd);
	// When w_arg_idx is -1 remove the window (see create_windows()).
	if (curwin->w_arg_idx == -1)
	{
	    ++arg_idx;
	    win_close(curwin, TRUE);
	    advance = FALSE;
	    continue;
	}

	if (advance)
	{
	    if (parmp->window_layout == WIN_TABS)
	    {
		if (curtab->tp_next == NULL)	// just checking
		    break;
		goto_tabpage(0);
		// Temporarily reset 'shm' option to not print fileinfo when
		// loading the other buffers. This would overwrite the already
		// existing fileinfo for the first tab.
		if (i == 1)
		{
		    char buf[100];

		    p_shm_save = vim_strsave(p_shm);
		    vim_snprintf(buf, 100, "F%s", p_shm);
		    set_option_value_give_err((char_u *)"shm",
							 0L, (char_u *)buf, 0);
		}
	    }
	    else
	    {
		if (curwin->w_next == NULL)	// just checking
		    break;
		win_enter(curwin->w_next, FALSE);
	    }
	}
	advance = TRUE;

	// Only open the file if there is no file in this window yet (that can
	// happen when .vimrc contains ":sall").
	if (curbuf == firstwin->w_buffer || curbuf->b_ffname == NULL)
	{
	    curwin->w_arg_idx = arg_idx;
	    // Edit file from arg list, if there is one.  When "Quit" selected
	    // at the ATTENTION prompt close the window.
	    swap_exists_did_quit = FALSE;
	    (void)do_ecmd(0, arg_idx < GARGCOUNT
			  ? alist_name(&GARGLIST[arg_idx]) : NULL,
			  NULL, NULL, ECMD_LASTL, ECMD_HIDE, curwin);
	    if (swap_exists_did_quit)
	    {
		// abort or quit selected
		if (got_int || only_one_window())
		{
		    // abort selected and only one window
		    did_emsg = FALSE;   // avoid hit-enter prompt
		    getout(1);
		}
		win_close(curwin, TRUE);
		advance = FALSE;
	    }
	    if (arg_idx == GARGCOUNT - 1)
		arg_had_last = TRUE;
	    ++arg_idx;
	}
	ui_breakcheck();
	if (got_int)
	{
	    (void)vgetc();	// only break the file loading, not the rest
	    break;
	}
    }

    if (p_shm_save != NULL)
    {
	set_option_value_give_err((char_u *)"shm", 0L, p_shm_save, 0);
	vim_free(p_shm_save);
    }

    if (parmp->window_layout == WIN_TABS)
	goto_tabpage(1);
    --autocmd_no_enter;

    // make the first window the current window
    win = firstwin;
#if defined(FEAT_QUICKFIX)
    // Avoid making a preview window the current window.
    while (win->w_p_pvw)
    {
	win = win->w_next;
	if (win == NULL)
	{
	    win = firstwin;
	    break;
	}
    }
#endif
    win_enter(win, FALSE);

    --autocmd_no_leave;
    TIME_MSG("editing files in windows");
    if (parmp->window_count > 1 && parmp->window_layout != WIN_TABS)
	win_equal(curwin, FALSE, 'b');	// adjust heights
}

/*
 * Execute the commands from --cmd arguments "cmds[cnt]".
 */
    static void
exe_pre_commands(mparm_T *parmp)
{
    char_u	**cmds = parmp->pre_commands;
    int		cnt = parmp->n_pre_commands;
    int		i;
    ESTACK_CHECK_DECLARATION;

    if (cnt <= 0)
	return;

    curwin->w_cursor.lnum = 0; // just in case..
    estack_push(ETYPE_ARGS, (char_u *)_("pre-vimrc command line"), 0);
    ESTACK_CHECK_SETUP;
# ifdef FEAT_EVAL
    current_sctx.sc_sid = SID_CMDARG;
# endif
    for (i = 0; i < cnt; ++i)
	do_cmdline_cmd(cmds[i]);
    ESTACK_CHECK_NOW;
    estack_pop();
# ifdef FEAT_EVAL
    current_sctx.sc_sid = 0;
# endif
    TIME_MSG("--cmd commands");
}

/*
 * Execute "+", "-c" and "-S" arguments.
 */
    static void
exe_commands(mparm_T *parmp)
{
    int		i;
    ESTACK_CHECK_DECLARATION;

    /*
     * We start commands on line 0, make "vim +/pat file" match a
     * pattern on line 1.  But don't move the cursor when an autocommand
     * with g`" was used.
     */
    msg_scroll = TRUE;
    if (parmp->tagname == NULL && curwin->w_cursor.lnum <= 1)
	curwin->w_cursor.lnum = 0;
    estack_push(ETYPE_ARGS, (char_u *)"command line", 0);
    ESTACK_CHECK_SETUP;
#ifdef FEAT_EVAL
    current_sctx.sc_sid = SID_CARG;
    current_sctx.sc_seq = 0;
#endif
    for (i = 0; i < parmp->n_commands; ++i)
    {
	do_cmdline_cmd(parmp->commands[i]);
	if (parmp->cmds_tofree[i])
	    vim_free(parmp->commands[i]);
    }
    ESTACK_CHECK_NOW;
    estack_pop();
#ifdef FEAT_EVAL
    current_sctx.sc_sid = 0;
#endif
    if (curwin->w_cursor.lnum == 0)
	curwin->w_cursor.lnum = 1;

    if (!exmode_active)
	msg_scroll = FALSE;

#ifdef FEAT_QUICKFIX
    // When started with "-q errorfile" jump to first error again.
    if (parmp->edit_type == EDIT_QF)
	qf_jump(NULL, 0, 0, FALSE);
#endif
    TIME_MSG("executing command arguments");
}

/*
 * Source startup scripts.
 */
    static void
source_startup_scripts(mparm_T *parmp)
{
    int		i;

    /*
     * For "evim" source evim.vim first of all, so that the user can overrule
     * any things he doesn't like.
     */
    if (parmp->evim_mode)
    {
	(void)do_source((char_u *)EVIM_FILE, FALSE, DOSO_NONE, NULL);
	TIME_MSG("source evim file");
    }

    /*
     * If -u argument given, use only the initializations from that file and
     * nothing else.
     */
    if (parmp->use_vimrc != NULL)
    {
	if (STRCMP(parmp->use_vimrc, "DEFAULTS") == 0)
	{
	    if (do_source((char_u *)VIM_DEFAULTS_FILE, FALSE, DOSO_NONE, NULL)
									 != OK)
		emsg(_(e_failed_to_source_defaults));
	}
	else if (STRCMP(parmp->use_vimrc, "NONE") == 0
				     || STRCMP(parmp->use_vimrc, "NORC") == 0)
	{
#ifdef FEAT_GUI
	    if (use_gvimrc == NULL)	    // don't load gvimrc either
		use_gvimrc = parmp->use_vimrc;
#endif
	}
	else
	{
	    if (do_source(parmp->use_vimrc, FALSE, DOSO_NONE, NULL) != OK)
		semsg(_(e_cannot_read_from_str_2), parmp->use_vimrc);
	}
    }
    else if (!silent_mode)
    {
#ifdef AMIGA
	struct Process	*proc = (struct Process *)FindTask(0L);
	APTR		save_winptr = proc->pr_WindowPtr;

	// Avoid a requester here for a volume that doesn't exist.
	proc->pr_WindowPtr = (APTR)-1L;
#endif

	/*
	 * Get system wide defaults, if the file name is defined.
	 */
#ifdef SYS_VIMRC_FILE
	(void)do_source((char_u *)SYS_VIMRC_FILE, FALSE, DOSO_NONE, NULL);
#endif
#ifdef MACOS_X
	(void)do_source((char_u *)"$VIMRUNTIME/macmap.vim", FALSE,
							      DOSO_NONE, NULL);
#endif

	/*
	 * Try to read initialization commands from the following places:
	 * - environment variable VIMINIT
	 * - user vimrc file (s:.vimrc for Amiga, ~/.vimrc otherwise)
	 * - second user vimrc file ($VIM/.vimrc for Dos)
	 * - environment variable EXINIT
	 * - user exrc file (s:.exrc for Amiga, ~/.exrc otherwise)
	 * - second user exrc file ($VIM/.exrc for Dos)
	 * The first that exists is used, the rest is ignored.
	 */
	if (process_env((char_u *)"VIMINIT", TRUE) != OK)
	{
	    if (do_source((char_u *)USR_VIMRC_FILE, TRUE,
						      DOSO_VIMRC, NULL) == FAIL
#ifdef USR_VIMRC_FILE2
		&& do_source((char_u *)USR_VIMRC_FILE2, TRUE,
						      DOSO_VIMRC, NULL) == FAIL
#endif
#ifdef USR_VIMRC_FILE3
		&& do_source((char_u *)USR_VIMRC_FILE3, TRUE,
						      DOSO_VIMRC, NULL) == FAIL
#endif
#ifdef USR_VIMRC_FILE4
		&& do_source((char_u *)USR_VIMRC_FILE4, TRUE,
						      DOSO_VIMRC, NULL) == FAIL
#endif
		&& process_env((char_u *)"EXINIT", FALSE) == FAIL
		&& do_source((char_u *)USR_EXRC_FILE, FALSE,
						       DOSO_NONE, NULL) == FAIL
#ifdef USR_EXRC_FILE2
		&& do_source((char_u *)USR_EXRC_FILE2, FALSE,
						       DOSO_NONE, NULL) == FAIL
#endif
		&& !has_dash_c_arg)
	    {
		// When no .vimrc file was found: source defaults.vim.
		if (do_source((char_u *)VIM_DEFAULTS_FILE, FALSE, DOSO_NONE,
								 NULL) == FAIL)
		    emsg(_(e_failed_to_source_defaults));
	    }
	}

	/*
	 * Read initialization commands from ".vimrc" or ".exrc" in current
	 * directory.  This is only done if the 'exrc' option is set.
	 * Because of security reasons we disallow shell and write commands
	 * now, except for Unix if the file is owned by the user or 'secure'
	 * option has been reset in environment of global ".exrc" or ".vimrc".
	 * Only do this if VIMRC_FILE is not the same as USR_VIMRC_FILE or
	 * SYS_VIMRC_FILE.
	 */
	if (p_exrc)
	{
#if defined(UNIX) || defined(VMS)
	    // If ".vimrc" file is not owned by user, set 'secure' mode.
	    if (!file_owned(VIMRC_FILE))
#endif
		secure = p_secure;

	    i = FAIL;
	    if (fullpathcmp((char_u *)USR_VIMRC_FILE,
				(char_u *)VIMRC_FILE, FALSE, TRUE) != FPC_SAME
#ifdef USR_VIMRC_FILE2
		    && fullpathcmp((char_u *)USR_VIMRC_FILE2,
				(char_u *)VIMRC_FILE, FALSE, TRUE) != FPC_SAME
#endif
#ifdef USR_VIMRC_FILE3
		    && fullpathcmp((char_u *)USR_VIMRC_FILE3,
				(char_u *)VIMRC_FILE, FALSE, TRUE) != FPC_SAME
#endif
#ifdef SYS_VIMRC_FILE
		    && fullpathcmp((char_u *)SYS_VIMRC_FILE,
				(char_u *)VIMRC_FILE, FALSE, TRUE) != FPC_SAME
#endif
				)
		i = do_source((char_u *)VIMRC_FILE, TRUE, DOSO_VIMRC, NULL);

	    if (i == FAIL)
	    {
#if defined(UNIX) || defined(VMS)
		// if ".exrc" is not owned by user set 'secure' mode
		if (!file_owned(EXRC_FILE))
		    secure = p_secure;
		else
		    secure = 0;
#endif
		if (	   fullpathcmp((char_u *)USR_EXRC_FILE,
				(char_u *)EXRC_FILE, FALSE, TRUE) != FPC_SAME
#ifdef USR_EXRC_FILE2
			&& fullpathcmp((char_u *)USR_EXRC_FILE2,
				(char_u *)EXRC_FILE, FALSE, TRUE) != FPC_SAME
#endif
				)
		    (void)do_source((char_u *)EXRC_FILE, FALSE,
							      DOSO_NONE, NULL);
	    }
	}
	if (secure == 2)
	    need_wait_return = TRUE;
	secure = 0;
#ifdef AMIGA
	proc->pr_WindowPtr = save_winptr;
#endif
    }
    TIME_MSG("sourcing vimrc file(s)");
}

/*
 * Setup to start using the GUI.  Exit with an error when not available.
 */
    static void
main_start_gui(void)
{
#ifdef FEAT_GUI
    gui.starting = TRUE;	// start GUI a bit later
#else
    mch_errmsg(_(e_gui_cannot_be_used_not_enabled_at_compile_time));
    mch_errmsg("\n");
    mch_exit(2);
#endif
}

#endif  // NO_VIM_MAIN

/*
 * Get an environment variable and execute it as Ex commands.
 * Returns FAIL if the environment variable was not executed, OK otherwise.
 */
    int
process_env(
    char_u	*env,
    int		is_viminit) // when TRUE, called for VIMINIT
{
    char_u	*initstr;
    sctx_T	save_current_sctx;
    ESTACK_CHECK_DECLARATION;

    if ((initstr = mch_getenv(env)) == NULL || *initstr == NUL)
	return FAIL;

    if (is_viminit)
	vimrc_found(NULL, NULL);
    estack_push(ETYPE_ENV, env, 0);
    ESTACK_CHECK_SETUP;
    save_current_sctx = current_sctx;
    current_sctx.sc_version = 1;
#ifdef FEAT_EVAL
    current_sctx.sc_sid = SID_ENV;
    current_sctx.sc_seq = 0;
    current_sctx.sc_lnum = 0;
#endif

    do_cmdline_cmd(initstr);

    ESTACK_CHECK_NOW;
    estack_pop();
    current_sctx = save_current_sctx;
    return OK;
}

#if (defined(UNIX) || defined(VMS)) && !defined(NO_VIM_MAIN)
/*
 * Return TRUE if we are certain the user owns the file "fname".
 * Used for ".vimrc" and ".exrc".
 * Use both stat() and lstat() for extra security.
 */
    static int
file_owned(char *fname)
{
    stat_T	s;
# ifdef UNIX
    uid_t	uid = getuid();
# else	 // VMS
    uid_t	uid = ((getgid() << 16) | getuid());
# endif

    return !(mch_stat(fname, &s) != 0 || s.st_uid != uid
# ifdef HAVE_LSTAT
	    || mch_lstat(fname, &s) != 0 || s.st_uid != uid
# endif
	    );
}
#endif

/*
 * Give an error message main_errors["n"] and exit.
 */
    static void
mainerr(
    int		n,	// one of the ME_ defines
    char_u	*str)	// extra argument or NULL
{
#if defined(UNIX) || defined(VMS)
    reset_signals();		// kill us with CTRL-C here, if you like
#endif

    // If this is a Windows GUI executable, show an error dialog box.
#ifdef VIMDLL
    gui.in_use = mch_is_gui_executable();
#endif
#ifdef FEAT_GUI_MSWIN
    gui.starting = FALSE;   // Needed to show as error.
#endif

    init_longVersion();
    mch_errmsg(longVersion);
    mch_errmsg("\n");
    mch_errmsg(_(main_errors[n]));
    if (str != NULL)
    {
	mch_errmsg(": \"");
	mch_errmsg((char *)str);
	mch_errmsg("\"");
    }
    mch_errmsg(_("\nMore info with: \"vim -h\"\n"));

    mch_exit(1);
}

    void
mainerr_arg_missing(char_u *str)
{
    mainerr(ME_ARG_MISSING, str);
}

#ifndef NO_VIM_MAIN
/*
 * print a message with three spaces prepended and '\n' appended.
 */
    static void
main_msg(char *s)
{
    mch_msg("   ");
    mch_msg(s);
    mch_msg("\n");
}

/*
 * Print messages for "vim -h" or "vim --help" and exit.
 */
    static void
usage(void)
{
    int		i;
    static char	*(use[]) =
    {
	N_("[file ..]       edit specified file(s)"),
	N_("-               read text from stdin"),
	N_("-t tag          edit file where tag is defined"),
#ifdef FEAT_QUICKFIX
	N_("-q [errorfile]  edit file with first error")
#endif
    };

#if defined(UNIX) || defined(VMS)
    reset_signals();		// kill us with CTRL-C here, if you like
#endif

    init_longVersion();
    mch_msg(longVersion);
    mch_msg(_("\n\nUsage:"));
    for (i = 0; ; ++i)
    {
	mch_msg(_(" vim [arguments] "));
	mch_msg(_(use[i]));
	if (i == ARRAY_LENGTH(use) - 1)
	    break;
	mch_msg(_("\n   or:"));
    }
#ifdef VMS
    mch_msg(_("\nWhere case is ignored prepend / to make flag upper case"));
#endif

    mch_msg(_("\n\nArguments:\n"));
    main_msg(_("--\t\t\tOnly file names after this"));
#ifdef EXPAND_FILENAMES
    main_msg(_("--literal\t\tDon't expand wildcards"));
#endif
#ifdef FEAT_OLE
    main_msg(_("-register\t\tRegister this gvim for OLE"));
    main_msg(_("-unregister\t\tUnregister gvim for OLE"));
#endif
#ifdef FEAT_GUI
    main_msg(_("-g\t\t\tRun using GUI (like \"gvim\")"));
    main_msg(_("-f  or  --nofork\tForeground: Don't fork when starting GUI"));
#endif
    main_msg(_("-v\t\t\tVi mode (like \"vi\")"));
    main_msg(_("-e\t\t\tEx mode (like \"ex\")"));
    main_msg(_("-E\t\t\tImproved Ex mode"));
    main_msg(_("-s\t\t\tSilent (batch) mode (only for \"ex\")"));
#ifdef FEAT_DIFF
    main_msg(_("-d\t\t\tDiff mode (like \"vimdiff\")"));
#endif
    main_msg(_("-y\t\t\tEasy mode (like \"evim\", modeless)"));
    main_msg(_("-R\t\t\tReadonly mode (like \"view\")"));
    main_msg(_("-Z\t\t\tRestricted mode (like \"rvim\")"));
    main_msg(_("-m\t\t\tModifications (writing files) not allowed"));
    main_msg(_("-M\t\t\tModifications in text not allowed"));
    main_msg(_("-b\t\t\tBinary mode"));
    main_msg(_("-l\t\t\tLisp mode"));
    main_msg(_("-C\t\t\tCompatible with Vi: 'compatible'"));
    main_msg(_("-N\t\t\tNot fully Vi compatible: 'nocompatible'"));
    main_msg(_("-V[N][fname]\t\tBe verbose [level N] [log messages to fname]"));
#ifdef FEAT_EVAL
    main_msg(_("-D\t\t\tDebugging mode"));
#endif
    main_msg(_("-n\t\t\tNo swap file, use memory only"));
    main_msg(_("-r\t\t\tList swap files and exit"));
    main_msg(_("-r (with file name)\tRecover crashed session"));
    main_msg(_("-L\t\t\tSame as -r"));
#ifdef AMIGA
    main_msg(_("-f\t\t\tDon't use newcli to open window"));
    main_msg(_("-dev <device>\t\tUse <device> for I/O"));
#endif
#ifdef FEAT_ARABIC
    main_msg(_("-A\t\t\tStart in Arabic mode"));
#endif
#ifdef FEAT_RIGHTLEFT
    main_msg(_("-H\t\t\tStart in Hebrew mode"));
#endif
    main_msg(_("-T <terminal>\tSet terminal type to <terminal>"));
    main_msg(_("--not-a-term\t\tSkip warning for input/output not being a terminal"));
#ifdef FEAT_GUI
    main_msg(_("--gui-dialog-file {fname}  For testing: write dialog text"));
#endif
    main_msg(_("--ttyfail\t\tExit if input or output is not a terminal"));
    main_msg(_("-u <vimrc>\t\tUse <vimrc> instead of any .vimrc"));
#ifdef FEAT_GUI
    main_msg(_("-U <gvimrc>\t\tUse <gvimrc> instead of any .gvimrc"));
#endif
    main_msg(_("--noplugin\t\tDon't load plugin scripts"));
    main_msg(_("-p[N]\t\tOpen N tab pages (default: one for each file)"));
    main_msg(_("-o[N]\t\tOpen N windows (default: one for each file)"));
    main_msg(_("-O[N]\t\tLike -o but split vertically"));
    main_msg(_("+\t\t\tStart at end of file"));
    main_msg(_("+<lnum>\t\tStart at line <lnum>"));
    main_msg(_("--cmd <command>\tExecute <command> before loading any vimrc file"));
    main_msg(_("-c <command>\t\tExecute <command> after loading the first file"));
    main_msg(_("-S <session>\t\tSource file <session> after loading the first file"));
    main_msg(_("-s <scriptin>\tRead Normal mode commands from file <scriptin>"));
    main_msg(_("-w <scriptout>\tAppend all typed commands to file <scriptout>"));
    main_msg(_("-W <scriptout>\tWrite all typed commands to file <scriptout>"));
#ifdef FEAT_CRYPT
    main_msg(_("-x\t\t\tEdit encrypted files"));
#endif
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_X11)
# if defined(FEAT_GUI_X11) && !defined(FEAT_GUI_GTK)
    main_msg(_("-display <display>\tConnect Vim to this particular X-server"));
# endif
    main_msg(_("-X\t\t\tDo not connect to X server"));
#endif
#ifdef FEAT_CLIENTSERVER
    main_msg(_("--remote <files>\tEdit <files> in a Vim server if possible"));
    main_msg(_("--remote-silent <files>  Same, don't complain if there is no server"));
    main_msg(_("--remote-wait <files>  As --remote but wait for files to have been edited"));
    main_msg(_("--remote-wait-silent <files>  Same, don't complain if there is no server"));
    main_msg(_("--remote-tab[-wait][-silent] <files>  As --remote but use tab page per file"));
    main_msg(_("--remote-send <keys>\tSend <keys> to a Vim server and exit"));
    main_msg(_("--remote-expr <expr>\tEvaluate <expr> in a Vim server and print result"));
    main_msg(_("--serverlist\t\tList available Vim server names and exit"));
    main_msg(_("--servername <name>\tSend to/become the Vim server <name>"));
#endif
#ifdef STARTUPTIME
    main_msg(_("--startuptime <file>\tWrite startup timing messages to <file>"));
#endif
#ifdef FEAT_JOB_CHANNEL
    main_msg(_("--log <file>\t\tStart logging to <file> early"));
#endif
#ifdef FEAT_VIMINFO
    main_msg(_("-i <viminfo>\t\tUse <viminfo> instead of .viminfo"));
#endif
    main_msg(_("--clean\t\t'nocompatible', Vim defaults, no plugins, no viminfo"));
    main_msg(_("-h  or  --help\tPrint Help (this message) and exit"));
    main_msg(_("--version\t\tPrint version information and exit"));

#ifdef FEAT_GUI_X11
# ifdef FEAT_GUI_MOTIF
    mch_msg(_("\nArguments recognised by gvim (Motif version):\n"));
# endif
    main_msg(_("-display <display>\tRun Vim on <display>"));
    main_msg(_("-iconic\t\tStart Vim iconified"));
    main_msg(_("-background <color>\tUse <color> for the background (also: -bg)"));
    main_msg(_("-foreground <color>\tUse <color> for normal text (also: -fg)"));
    main_msg(_("-font <font>\t\tUse <font> for normal text (also: -fn)"));
    main_msg(_("-boldfont <font>\tUse <font> for bold text"));
    main_msg(_("-italicfont <font>\tUse <font> for italic text"));
    main_msg(_("-geometry <geom>\tUse <geom> for initial geometry (also: -geom)"));
    main_msg(_("-borderwidth <width>\tUse a border width of <width> (also: -bw)"));
    main_msg(_("-scrollbarwidth <width>  Use a scrollbar width of <width> (also: -sw)"));
    main_msg(_("-reverse\t\tUse reverse video (also: -rv)"));
    main_msg(_("+reverse\t\tDon't use reverse video (also: +rv)"));
    main_msg(_("-xrm <resource>\tSet the specified resource"));
#endif // FEAT_GUI_X11
#ifdef FEAT_GUI_GTK
    mch_msg(_("\nArguments recognised by gvim (GTK+ version):\n"));
    main_msg(_("-background <color>\tUse <color> for the background (also: -bg)"));
    main_msg(_("-foreground <color>\tUse <color> for normal text (also: -fg)"));
    main_msg(_("-font <font>\t\tUse <font> for normal text (also: -fn)"));
    main_msg(_("-geometry <geom>\tUse <geom> for initial geometry (also: -geom)"));
    main_msg(_("-iconic\t\tStart Vim iconified"));
    main_msg(_("-reverse\t\tUse reverse video (also: -rv)"));
    main_msg(_("-display <display>\tRun Vim on <display> (also: --display)"));
    main_msg(_("--role <role>\tSet a unique role to identify the main window"));
    main_msg(_("--socketid <xid>\tOpen Vim inside another GTK widget"));
    main_msg(_("--echo-wid\t\tMake gvim echo the Window ID on stdout"));
#endif
#ifdef FEAT_GUI_MSWIN
# ifdef VIMDLL
    if (gui.starting)
# endif
    {
	main_msg(_("-P <parent title>\tOpen Vim inside parent application"));
	main_msg(_("--windowid <HWND>\tOpen Vim inside another win32 widget"));
    }
#endif

#ifdef FEAT_GUI_GNOME
    // Gnome gives extra messages for --help if we continue, but not for -h.
    if (gui.starting)
    {
	mch_msg("\n");
	gui.dofork = FALSE;
    }
    else
#endif
	mch_exit(0);
}

/*
 * Check the result of the ATTENTION dialog:
 * When "Quit" selected, exit Vim.
 * When "Recover" selected, recover the file.
 */
    static void
check_swap_exists_action(void)
{
    if (swap_exists_action == SEA_QUIT)
	getout(1);
    handle_swap_exists(NULL);
}

#endif // NO_VIM_MAIN

#if !defined(NO_VIM_MAIN) && defined(FEAT_EVAL)
    static void
set_progpath(char_u *argv0)
{
    char_u *val = argv0;

# ifdef MSWIN
    // A relative path containing a "/" will become invalid when using ":cd",
    // turn it into a full path.
    // On MS-Windows "vim" should be expanded to "vim.exe", thus always do
    // this.
    char_u *path = NULL;

    if (mch_can_exe(argv0, &path, FALSE) && path != NULL)
	val = path;
# else
    char_u	buf[MAXPATHL + 1];
#  ifdef PROC_EXE_LINK
    char	linkbuf[MAXPATHL + 1];
    ssize_t	len;

    len = readlink(PROC_EXE_LINK, linkbuf, MAXPATHL);
    if (len > 0)
    {
	linkbuf[len] = NUL;
	val = (char_u *)linkbuf;
    }
#  endif

    if (!mch_isFullName(val))
    {
	if (gettail(val) != val
			   && vim_FullName(val, buf, MAXPATHL, TRUE) != FAIL)
	    val = buf;
    }
# endif

    set_vim_var_string(VV_PROGPATH, val, -1);

# ifdef MSWIN
    vim_free(path);
# endif
}

#endif // NO_VIM_MAIN
