/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *				GUI/Motif support by Robert Webb
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

#include "vim.h"

// Structure containing all the GUI information
gui_T gui;

#if defined(FEAT_GUI_X11) && !defined(FEAT_GUI_GTK)
# define USE_SET_GUIFONTWIDE
static void set_guifontwide(char_u *font_name);
#endif
static void gui_check_pos(void);
static void gui_reset_scroll_region(void);
static void gui_outstr(char_u *, int);
static int gui_screenchar(int off, int flags, guicolor_T fg, guicolor_T bg, int back);
static int gui_outstr_nowrap(char_u *s, int len, int flags, guicolor_T fg, guicolor_T bg, int back);
static void gui_delete_lines(int row, int count);
static void gui_insert_lines(int row, int count);
static int gui_xy2colrow(int x, int y, int *colp);
#if defined(FEAT_GUI_TABLINE) || defined(PROTO)
static int gui_has_tabline(void);
#endif
static void gui_do_scrollbar(win_T *wp, int which, int enable);
static void gui_update_horiz_scrollbar(int);
static void gui_set_fg_color(char_u *name);
static void gui_set_bg_color(char_u *name);
static void init_gui_options(void);
static win_T *xy2win(int x, int y, mouse_find_T popup);

#ifdef GUI_MAY_FORK
static void gui_do_fork(void);

static int gui_read_child_pipe(int fd);

// Return values for gui_read_child_pipe
enum {
    GUI_CHILD_IO_ERROR,
    GUI_CHILD_OK,
    GUI_CHILD_FAILED
};
#endif

static void gui_attempt_start(void);

static int can_update_cursor = TRUE; // can display the cursor
static int disable_flush = 0;	// If > 0, gui_mch_flush() is disabled.

/*
 * gui_start -- Called when user wants to start the GUI.
 *
 * Careful: This function can be called recursively when there is a ":gui"
 * command in the .gvimrc file.  Only the first call should fork, not the
 * recursive call.
 */
    void
gui_start(char_u *arg UNUSED)
{
    char_u	*old_term;
#ifdef GUI_MAY_FORK
    static int	recursive = 0;
#endif
#if defined(GUI_MAY_SPAWN) && defined(EXPERIMENTAL_GUI_CMD)
    char	*msg = NULL;
#endif

    old_term = vim_strsave(T_NAME);

    settmode(TMODE_COOK);		// stop RAW mode
    if (full_screen)
	cursor_on();			// needed for ":gui" in .vimrc
    full_screen = FALSE;

#ifdef GUI_MAY_FORK
    ++recursive;
    /*
     * Quit the current process and continue in the child.
     * Makes "gvim file" disconnect from the shell it was started in.
     * Don't do this when Vim was started with "-f" or the 'f' flag is present
     * in 'guioptions'.
     * Don't do this when there is a running job, we can only get the status
     * of a child from the parent.
     */
    if (gui.dofork && !vim_strchr(p_go, GO_FORG) && recursive <= 1
# ifdef FEAT_JOB_CHANNEL
	    && !job_any_running()
# endif
	    )
    {
	gui_do_fork();
    }
    else
#endif
#ifdef GUI_MAY_SPAWN
    if (gui.dospawn
# ifdef EXPERIMENTAL_GUI_CMD
	    && gui.dofork
# endif
	    && !vim_strchr(p_go, GO_FORG)
	    && !anyBufIsChanged()
# ifdef FEAT_JOB_CHANNEL
	    && !job_any_running()
# endif
	    )
    {
# ifdef EXPERIMENTAL_GUI_CMD
	msg =
# endif
	    gui_mch_do_spawn(arg);
    }
    else
#endif
    {
#ifdef FEAT_GUI_GTK
	// If there is 'f' in 'guioptions' and specify -g argument,
	// gui_mch_init_check() was not called yet.
	if (gui_mch_init_check() != OK)
	    getout_preserve_modified(1);
#endif
	gui_attempt_start();
    }

    if (!gui.in_use)			// failed to start GUI
    {
	// Back to old term settings
	//
	// FIXME: If we got here because a child process failed and flagged to
	// the parent to resume, and X11 is enabled, this will
	// hit an X11 I/O error and do a longjmp(), leaving recursive
	// permanently set to 1. This is probably not as big a problem as it
	// sounds, because gui_mch_init() in both gui_x11.c and gui_gtk_x11.c
	// return "OK" unconditionally, so it would be very difficult to
	// actually hit this case.
	termcapinit(old_term);
	settmode(TMODE_RAW);		// restart RAW mode
	set_title_defaults();		// set 'title' and 'icon' again
#if defined(GUI_MAY_SPAWN) && defined(EXPERIMENTAL_GUI_CMD)
	if (msg != NULL)
	    emsg(msg);
#endif
    }

    vim_free(old_term);

    // If the GUI started successfully, trigger the GUIEnter event, otherwise
    // the GUIFailed event.
    gui_mch_update();
    apply_autocmds(gui.in_use ? EVENT_GUIENTER : EVENT_GUIFAILED,
						   NULL, NULL, FALSE, curbuf);
#ifdef GUI_MAY_FORK
    --recursive;
#endif
}

/*
 * Set_termname() will call gui_init() to start the GUI.
 * Set the "starting" flag, to indicate that the GUI will start.
 *
 * We don't want to open the GUI shell until after we've read .gvimrc,
 * otherwise we don't know what font we will use, and hence we don't know
 * what size the shell should be.  So if there are errors in the .gvimrc
 * file, they will have to go to the terminal: Set full_screen to FALSE.
 * full_screen will be set to TRUE again by a successful termcapinit().
 */
    static void
gui_attempt_start(void)
{
    static int recursive = 0;

    ++recursive;
    gui.starting = TRUE;

#ifdef FEAT_GUI_GTK
    gui.event_time = GDK_CURRENT_TIME;
#endif

    termcapinit((char_u *)"builtin_gui");
    gui.starting = recursive - 1;

#if defined(FEAT_GUI_GTK) || defined(FEAT_GUI_X11)
    if (gui.in_use)
    {
# ifdef FEAT_EVAL
	Window	x11_window;
	Display	*x11_display;

	if (gui_get_x11_windis(&x11_window, &x11_display) == OK)
	    set_vim_var_nr(VV_WINDOWID, (long)x11_window);
# endif

	// Display error messages in a dialog now.
	display_errors();
    }
#endif
    --recursive;
}

#ifdef GUI_MAY_FORK

// for waitpid()
# if defined(HAVE_SYS_WAIT_H) || defined(HAVE_UNION_WAIT)
#  include <sys/wait.h>
# endif

/*
 * Create a new process, by forking. In the child, start the GUI, and in
 * the parent, exit.
 *
 * If something goes wrong, this will return with gui.in_use still set
 * to FALSE, in which case the caller should continue execution without
 * the GUI.
 *
 * If the child fails to start the GUI, then the child will exit and the
 * parent will return. If the child succeeds, then the parent will exit
 * and the child will return.
 */
    static void
gui_do_fork(void)
{
    int		pipefd[2];	// pipe between parent and child
    int		pipe_error;
    int		status;
    int		exit_status;
    pid_t	pid = -1;

# if defined(FEAT_RELTIME) && defined(PROF_NSEC)
    // a timer is not carried forward
    delete_timer();
# endif

    // Setup a pipe between the child and the parent, so that the parent
    // knows when the child has done the setsid() call and is allowed to
    // exit.
    pipe_error = (pipe(pipefd) < 0);
    pid = fork();
    if (pid < 0)	    // Fork error
    {
	emsg(_(e_failed_to_create_new_process_for_GUI));
	return;
    }
    else if (pid > 0)	    // Parent
    {
	// Give the child some time to do the setsid(), otherwise the
	// exit() may kill the child too (when starting gvim from inside a
	// gvim).
	if (!pipe_error)
	{
	    // The read returns when the child closes the pipe (or when
	    // the child dies for some reason).
	    close(pipefd[1]);
	    status = gui_read_child_pipe(pipefd[0]);
	    if (status == GUI_CHILD_FAILED)
	    {
		// The child failed to start the GUI, so the caller must
		// continue. There may be more error information written
		// to stderr by the child.
# ifdef __NeXT__
		wait4(pid, &exit_status, 0, (struct rusage *)0);
# else
		waitpid(pid, &exit_status, 0);
# endif
		emsg(_(e_the_child_process_failed_to_start_GUI));
		return;
	    }
	    else if (status == GUI_CHILD_IO_ERROR)
	    {
		pipe_error = TRUE;
	    }
	    // else GUI_CHILD_OK: parent exit
	}

	if (pipe_error)
	    ui_delay(301L, TRUE);

	// When swapping screens we may need to go to the next line, e.g.,
	// after a hit-enter prompt and using ":gui".
	if (newline_on_exit)
	    mch_errmsg("\r\n");

	/*
	 * The parent must skip the normal exit() processing, the child
	 * will do it.  For example, GTK messes up signals when exiting.
	 */
	_exit(0);
    }
    // Child

# ifdef FEAT_GUI_GTK
    // Call gtk_init_check() here after fork(). See gui_init_check().
    if (gui_mch_init_check() != OK)
	getout_preserve_modified(1);
# endif

# if defined(HAVE_SETSID) || defined(HAVE_SETPGID)
    /*
     * Change our process group.  On some systems/shells a CTRL-C in the
     * shell where Vim was started would otherwise kill gvim!
     */
#  if defined(HAVE_SETSID)
    (void)setsid();
#  else
    (void)setpgid(0, 0);
#  endif
# endif
    if (!pipe_error)
	close(pipefd[0]);

# if defined(FEAT_GUI_GNOME) && defined(FEAT_SESSION)
    // Tell the session manager our new PID
    gui_mch_forked();
# endif

    // Try to start the GUI
    gui_attempt_start();

    // Notify the parent
    if (!pipe_error)
    {
	if (gui.in_use)
	    write_eintr(pipefd[1], "ok", 3);
	else
	    write_eintr(pipefd[1], "fail", 5);
	close(pipefd[1]);
    }

    // If we failed to start the GUI, exit now.
    if (!gui.in_use)
	getout_preserve_modified(1);
}

/*
 * Read from a pipe assumed to be connected to the child process (this
 * function is called from the parent).
 * Return GUI_CHILD_OK if the child successfully started the GUI,
 * GUY_CHILD_FAILED if the child failed, or GUI_CHILD_IO_ERROR if there was
 * some other error.
 *
 * The file descriptor will be closed before the function returns.
 */
    static int
gui_read_child_pipe(int fd)
{
    long	bytes_read;
# define READ_BUFFER_SIZE 10
    char	buffer[READ_BUFFER_SIZE];

    bytes_read = read_eintr(fd, buffer, READ_BUFFER_SIZE - 1);
# undef READ_BUFFER_SIZE
    close(fd);
    if (bytes_read < 0)
	return GUI_CHILD_IO_ERROR;
    buffer[bytes_read] = NUL;
    if (strcmp(buffer, "ok") == 0)
	return GUI_CHILD_OK;
    return GUI_CHILD_FAILED;
}

#endif // GUI_MAY_FORK

/*
 * Call this when vim starts up, whether or not the GUI is started
 */
    void
gui_prepare(int *argc, char **argv)
{
    gui.in_use = FALSE;		    // No GUI yet (maybe later)
    gui.starting = FALSE;	    // No GUI yet (maybe later)
    gui_mch_prepare(argc, argv);
}

/*
 * Try initializing the GUI and check if it can be started.
 * Used from main() to check early if "vim -g" can start the GUI.
 * Used from gui_init() to prepare for starting the GUI.
 * Returns FAIL or OK.
 */
    int
gui_init_check(void)
{
    static int result = MAYBE;

    if (result != MAYBE)
    {
	if (result == FAIL)
	    emsg(_(e_cannot_start_the_GUI));
	return result;
    }

    gui.shell_created = FALSE;
    gui.dying = FALSE;
    gui.in_focus = TRUE;		// so the guicursor setting works
    gui.dragged_sb = SBAR_NONE;
    gui.dragged_wp = NULL;
    gui.pointer_hidden = FALSE;
    gui.col = 0;
    gui.row = 0;
    gui.num_cols = Columns;
    gui.num_rows = Rows;

    gui.cursor_is_valid = FALSE;
    gui.scroll_region_top = 0;
    gui.scroll_region_bot = Rows - 1;
    gui.scroll_region_left = 0;
    gui.scroll_region_right = Columns - 1;
    gui.highlight_mask = HL_NORMAL;
    gui.char_width = 1;
    gui.char_height = 1;
    gui.char_ascent = 0;
    gui.border_width = 0;

    gui.norm_font = NOFONT;
#ifndef FEAT_GUI_GTK
    gui.bold_font = NOFONT;
    gui.ital_font = NOFONT;
    gui.boldital_font = NOFONT;
# ifdef FEAT_XFONTSET
    gui.fontset = NOFONTSET;
# endif
#endif
    gui.wide_font = NOFONT;
#ifndef FEAT_GUI_GTK
    gui.wide_bold_font = NOFONT;
    gui.wide_ital_font = NOFONT;
    gui.wide_boldital_font = NOFONT;
#endif

#ifdef FEAT_MENU
# ifndef FEAT_GUI_GTK
#  ifdef FONTSET_ALWAYS
    gui.menu_fontset = NOFONTSET;
#  else
    gui.menu_font = NOFONT;
#  endif
# endif
    gui.menu_is_active = TRUE;	    // default: include menu
# ifndef FEAT_GUI_GTK
    gui.menu_height = MENU_DEFAULT_HEIGHT;
    gui.menu_width = 0;
# endif
#endif
#if defined(FEAT_TOOLBAR) && (defined(FEAT_GUI_MOTIF) || defined(FEAT_GUI_HAIKU))
    gui.toolbar_height = 0;
#endif
#ifdef FEAT_BEVAL_TIP
    gui.tooltip_fontset = NOFONTSET;
#endif

    gui.scrollbar_width = gui.scrollbar_height = SB_DEFAULT_WIDTH;
    gui.prev_wrap = -1;

#ifdef FEAT_GUI_GTK
    CLEAR_FIELD(gui.ligatures_map);
#endif

#if defined(ALWAYS_USE_GUI) || defined(VIMDLL)
    result = OK;
#else
# ifdef FEAT_GUI_GTK
    /*
     * Note: Don't call gtk_init_check() before fork, it will be called after
     * the fork. When calling it before fork, it make vim hang for a while.
     * See gui_do_fork().
     * Use a simpler check if the GUI window can probably be opened.
     */
    result = gui.dofork ? gui_mch_early_init_check(TRUE) : gui_mch_init_check();
# else
    result = gui_mch_init_check();
# endif
#endif
    return result;
}

/*
 * This is the call which starts the GUI.
 */
    void
gui_init(void)
{
    win_T	*wp;
    static int	recursive = 0;

    /*
     * It's possible to use ":gui" in a .gvimrc file.  The first half of this
     * function will then be executed at the first call, the rest by the
     * recursive call.  This allow the shell to be opened halfway reading a
     * gvimrc file.
     */
    if (!recursive)
    {
	++recursive;

	clip_init(TRUE);

	// If can't initialize, don't try doing the rest
	if (gui_init_check() == FAIL)
	{
	    --recursive;
	    clip_init(FALSE);
	    return;
	}

	/*
	 * Reset 'paste'.  It's useful in the terminal, but not in the GUI.  It
	 * breaks the Paste toolbar button.
	 */
	set_option_value_give_err((char_u *)"paste", 0L, NULL, 0);

	// Set t_Co to the number of colors: RGB.
	set_color_count(256 * 256 * 256);

	/*
	 * Set up system-wide default menus.
	 */
#if defined(SYS_MENU_FILE) && defined(FEAT_MENU)
	if (vim_strchr(p_go, GO_NOSYSMENU) == NULL)
	{
	    sys_menu = TRUE;
	    do_source((char_u *)SYS_MENU_FILE, FALSE, DOSO_NONE, NULL);
	    sys_menu = FALSE;
	}
#endif

	/*
	 * Switch on the mouse by default, unless the user changed it already.
	 * This can then be changed in the .gvimrc.
	 */
	if (!option_was_set((char_u *)"mouse"))
	    set_string_option_direct((char_u *)"mouse", -1,
					   (char_u *)"a", OPT_FREE, SID_NONE);

	/*
	 * If -U option given, use only the initializations from that file and
	 * nothing else.  Skip all initializations for "-U NONE" or "-u NORC".
	 */
	if (use_gvimrc != NULL)
	{
	    if (STRCMP(use_gvimrc, "NONE") != 0
		    && STRCMP(use_gvimrc, "NORC") != 0
		    && do_source(use_gvimrc, FALSE, DOSO_NONE, NULL) != OK)
		semsg(_(e_cannot_read_from_str), use_gvimrc);
	}
	else
	{
	    /*
	     * Get system wide defaults for gvim, only when file name defined.
	     */
#ifdef SYS_GVIMRC_FILE
	    do_source((char_u *)SYS_GVIMRC_FILE, FALSE, DOSO_NONE, NULL);
#endif

	    /*
	     * Try to read GUI initialization commands from the following
	     * places:
	     * - environment variable GVIMINIT
	     * - the user gvimrc file (~/.gvimrc)
	     * - the second user gvimrc file ($VIM/.gvimrc for Dos)
	     * - the third user gvimrc file ($VIM/.gvimrc for Amiga)
	     * The first that exists is used, the rest is ignored.
	     */
	    if (process_env((char_u *)"GVIMINIT", FALSE) == FAIL
		 && do_source((char_u *)USR_GVIMRC_FILE, TRUE,
						     DOSO_GVIMRC, NULL) == FAIL
#ifdef USR_GVIMRC_FILE2
		 && do_source((char_u *)USR_GVIMRC_FILE2, TRUE,
						     DOSO_GVIMRC, NULL) == FAIL
#endif
#ifdef USR_GVIMRC_FILE3
		 && do_source((char_u *)USR_GVIMRC_FILE3, TRUE,
						     DOSO_GVIMRC, NULL) == FAIL
#endif
				)
	    {
#ifdef USR_GVIMRC_FILE4
		(void)do_source((char_u *)USR_GVIMRC_FILE4, TRUE,
							    DOSO_GVIMRC, NULL);
#endif
	    }

	    /*
	     * Read initialization commands from ".gvimrc" in current
	     * directory.  This is only done if the 'exrc' option is set.
	     * Because of security reasons we disallow shell and write
	     * commands now, except for unix if the file is owned by the user
	     * or 'secure' option has been reset in environment of global
	     * ".gvimrc".
	     * Only do this if GVIMRC_FILE is not the same as USR_GVIMRC_FILE,
	     * USR_GVIMRC_FILE2, USR_GVIMRC_FILE3 or SYS_GVIMRC_FILE.
	     */
	    if (p_exrc)
	    {
#ifdef UNIX
		{
		    stat_T s;

		    // if ".gvimrc" file is not owned by user, set 'secure'
		    // mode
		    if (mch_stat(GVIMRC_FILE, &s) || s.st_uid != getuid())
			secure = p_secure;
		}
#else
		secure = p_secure;
#endif

		if (       fullpathcmp((char_u *)USR_GVIMRC_FILE,
				(char_u *)GVIMRC_FILE, FALSE, TRUE) != FPC_SAME
#ifdef SYS_GVIMRC_FILE
			&& fullpathcmp((char_u *)SYS_GVIMRC_FILE,
				(char_u *)GVIMRC_FILE, FALSE, TRUE) != FPC_SAME
#endif
#ifdef USR_GVIMRC_FILE2
			&& fullpathcmp((char_u *)USR_GVIMRC_FILE2,
				(char_u *)GVIMRC_FILE, FALSE, TRUE) != FPC_SAME
#endif
#ifdef USR_GVIMRC_FILE3
			&& fullpathcmp((char_u *)USR_GVIMRC_FILE3,
				(char_u *)GVIMRC_FILE, FALSE, TRUE) != FPC_SAME
#endif
#ifdef USR_GVIMRC_FILE4
			&& fullpathcmp((char_u *)USR_GVIMRC_FILE4,
				(char_u *)GVIMRC_FILE, FALSE, TRUE) != FPC_SAME
#endif
			)
		    do_source((char_u *)GVIMRC_FILE, TRUE, DOSO_GVIMRC, NULL);

		if (secure == 2)
		    need_wait_return = TRUE;
		secure = 0;
	    }
	}

	if (need_wait_return || msg_didany)
	    wait_return(TRUE);

	--recursive;
    }

    // If recursive call opened the shell, return here from the first call
    if (gui.in_use)
	return;

    /*
     * Create the GUI shell.
     */
    gui.in_use = TRUE;		// Must be set after menus have been set up
    if (gui_mch_init() == FAIL)
	goto error;

    // Avoid a delay for an error message that was printed in the terminal
    // where Vim was started.
    emsg_on_display = FALSE;
    msg_scrolled = 0;
    clear_sb_text(TRUE);
    need_wait_return = FALSE;
    msg_didany = FALSE;

    /*
     * Check validity of any generic resources that may have been loaded.
     */
    if (gui.border_width < 0)
	gui.border_width = 0;

    /*
     * Set up the fonts.  First use a font specified with "-fn" or "-font".
     */
    if (font_argument != NULL)
	set_option_value_give_err((char_u *)"gfn",
					       0L, (char_u *)font_argument, 0);
    if (
#ifdef FEAT_XFONTSET
	    (*p_guifontset == NUL
	     || gui_init_font(p_guifontset, TRUE) == FAIL) &&
#endif
	    gui_init_font(*p_guifont == NUL ? hl_get_font_name()
						  : p_guifont, FALSE) == FAIL)
    {
	emsg(_(e_cannot_start_gui_no_valid_font_found));
	goto error2;
    }
    if (gui_get_wide_font() == FAIL)
	emsg(_(e_guifontwide_invalid));

    gui.num_cols = Columns;
    gui.num_rows = Rows;
    gui_reset_scroll_region();

    // Create initial scrollbars
    FOR_ALL_WINDOWS(wp)
    {
	gui_create_scrollbar(&wp->w_scrollbars[SBAR_LEFT], SBAR_LEFT, wp);
	gui_create_scrollbar(&wp->w_scrollbars[SBAR_RIGHT], SBAR_RIGHT, wp);
    }
    gui_create_scrollbar(&gui.bottom_sbar, SBAR_BOTTOM, NULL);

#ifdef FEAT_MENU
    gui_create_initial_menus(root_menu);
#endif
#ifdef FEAT_SIGN_ICONS
    sign_gui_started();
#endif

    // Configure the desired menu and scrollbars
    gui_init_which_components(NULL);

    // All components of the GUI have been created now
    gui.shell_created = TRUE;

#ifdef FEAT_GUI_MSWIN
    // Set the shell size, adjusted for the screen size.  For GTK this only
    // works after the shell has been opened, thus it is further down.
    // If the window is already maximized (e.g. when --windowid is passed in),
    // we want to use the system-provided dimensions by passing FALSE to
    // mustset. Otherwise, we want to initialize with the default rows/columns.
    if (gui_mch_maximized())
	gui_set_shellsize(FALSE, TRUE, RESIZE_BOTH);
    else
	gui_set_shellsize(TRUE, TRUE, RESIZE_BOTH);
#else
# ifndef FEAT_GUI_GTK
    gui_set_shellsize(FALSE, TRUE, RESIZE_BOTH);
# endif
#endif
#if defined(FEAT_GUI_MOTIF) && defined(FEAT_MENU)
    // Need to set the size of the menubar after all the menus have been
    // created.
    gui_mch_compute_menu_height((Widget)0);
#endif

    /*
     * Actually open the GUI shell.
     */
    if (gui_mch_open() != FAIL)
    {
	maketitle();
	resettitle();

	init_gui_options();
#ifdef FEAT_ARABIC
	// Our GUI can't do bidi.
	p_tbidi = FALSE;
#endif
#if defined(FEAT_GUI_GTK)
	// Give GTK+ a chance to put all widget's into place.
	gui_mch_update();

# ifdef FEAT_MENU
	// If there is no 'm' in 'guioptions' we need to remove the menu now.
	// It was still there to make F10 work.
	if (vim_strchr(p_go, GO_MENUS) == NULL)
	{
	    --gui.starting;
	    gui_mch_enable_menu(FALSE);
	    ++gui.starting;
	    gui_mch_update();
	}
# endif

	// Now make sure the shell fits on the screen.
	if (gui_mch_maximized())
	    gui_set_shellsize(FALSE, TRUE, RESIZE_BOTH);
	else
	    gui_set_shellsize(TRUE, TRUE, RESIZE_BOTH);
#endif
	// When 'lines' was set while starting up the topframe may have to be
	// resized.
	win_new_shellsize();

#ifdef FEAT_BEVAL_GUI
	// Always create the Balloon Evaluation area, but disable it when
	// 'ballooneval' is off.
	if (balloonEval != NULL)
	{
# ifdef FEAT_VARTABS
	    vim_free(balloonEval->vts);
# endif
	    vim_free(balloonEval);
	}
	balloonEvalForTerm = FALSE;
# ifdef FEAT_GUI_GTK
	balloonEval = gui_mch_create_beval_area(gui.drawarea, NULL,
						     &general_beval_cb, NULL);
# else
#  if defined(FEAT_GUI_MOTIF)
	{
	    extern Widget	textArea;
	    balloonEval = gui_mch_create_beval_area(textArea, NULL,
						     &general_beval_cb, NULL);
	}
#  else
#   ifdef FEAT_GUI_MSWIN
	balloonEval = gui_mch_create_beval_area(NULL, NULL,
						     &general_beval_cb, NULL);
#   endif
#  endif
# endif
	if (!p_beval)
	    gui_mch_disable_beval_area(balloonEval);
#endif

#ifndef FEAT_GUI_MSWIN
	// In the GUI modifiers are prepended to keys.
	// Don't do this for MS-Windows yet, it sends CTRL-K without the
	// modifier.
	seenModifyOtherKeys = TRUE;
#endif

#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
	if (!im_xim_isvalid_imactivate())
	    emsg(_(e_value_of_imactivatekey_is_invalid));
#endif
	// When 'cmdheight' was set during startup it may not have taken
	// effect yet.
	if (p_ch != 1L)
	    command_height();

	return;
    }

error2:
#ifdef FEAT_GUI_X11
    // undo gui_mch_init()
    gui_mch_uninit();
#endif

error:
    gui.in_use = FALSE;
    clip_init(FALSE);
}


    void
gui_exit(int rc)
{
    // don't free the fonts, it leads to a BUS error
    // richard@whitequeen.com Jul 99
    free_highlight_fonts();
    gui.in_use = FALSE;
    gui_mch_exit(rc);
}

#if defined(FEAT_GUI_GTK) || defined(FEAT_GUI_X11) || defined(FEAT_GUI_MSWIN) \
	|| defined(FEAT_GUI_PHOTON) || defined(PROTO)
# define NEED_GUI_UPDATE_SCREEN 1
/*
 * Called when the GUI shell is closed by the user.  If there are no changed
 * files Vim exits, otherwise there will be a dialog to ask the user what to
 * do.
 * When this function returns, Vim should NOT exit!
 */
    void
gui_shell_closed(void)
{
    cmdmod_T	    save_cmdmod = cmdmod;

    if (before_quit_autocmds(curwin, TRUE, FALSE))
	return;

    // Only exit when there are no changed files
    exiting = TRUE;
# ifdef FEAT_BROWSE
    cmdmod.cmod_flags |= CMOD_BROWSE;
# endif
# if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
    cmdmod.cmod_flags |= CMOD_CONFIRM;
# endif
    // If there are changed buffers, present the user with a dialog if
    // possible, otherwise give an error message.
    if (!check_changed_any(FALSE, FALSE))
	getout(0);

    exiting = FALSE;
    cmdmod = save_cmdmod;
    gui_update_screen();	// redraw, window may show changed buffer
}
#endif

/*
 * Set the font.  "font_list" is a comma separated list of font names.  The
 * first font name that works is used.  If none is found, use the default
 * font.
 * If "fontset" is TRUE, the "font_list" is used as one name for the fontset.
 * Return OK when able to set the font.  When it failed FAIL is returned and
 * the fonts are unchanged.
 */
    int
gui_init_font(char_u *font_list, int fontset UNUSED)
{
#define FONTLEN 320
    char_u	font_name[FONTLEN];
    int		font_list_empty = FALSE;
    int		ret = FAIL;

    if (!gui.in_use)
	return FAIL;

    font_name[0] = NUL;
    if (*font_list == NUL)
	font_list_empty = TRUE;
    else
    {
#ifdef FEAT_XFONTSET
	// When using a fontset, the whole list of fonts is one name.
	if (fontset)
	    ret = gui_mch_init_font(font_list, TRUE);
	else
#endif
	    while (*font_list != NUL)
	    {
		// Isolate one comma separated font name.
		(void)copy_option_part(&font_list, font_name, FONTLEN, ",");

		// Careful!!!  The Win32 version of gui_mch_init_font(), when
		// called with "*" will change p_guifont to the selected font
		// name, which frees the old value.  This makes font_list
		// invalid.  Thus when OK is returned here, font_list must no
		// longer be used!
		if (gui_mch_init_font(font_name, FALSE) == OK)
		{
#ifdef USE_SET_GUIFONTWIDE
		    // If it's a Unicode font, try setting 'guifontwide' to a
		    // similar double-width font.
		    if ((p_guifontwide == NULL || *p_guifontwide == NUL)
				&& strstr((char *)font_name, "10646") != NULL)
			set_guifontwide(font_name);
#endif
		    ret = OK;
		    break;
		}
	    }
    }

    if (ret != OK
	    && STRCMP(font_list, "*") != 0
	    && (font_list_empty || gui.norm_font == NOFONT))
    {
	/*
	 * Couldn't load any font in 'font_list', keep the current font if
	 * there is one.  If 'font_list' is empty, or if there is no current
	 * font, tell gui_mch_init_font() to try to find a font we can load.
	 */
	ret = gui_mch_init_font(NULL, FALSE);
    }

    if (ret == OK)
    {
#ifndef FEAT_GUI_GTK
	// Set normal font as current font
# ifdef FEAT_XFONTSET
	if (gui.fontset != NOFONTSET)
	    gui_mch_set_fontset(gui.fontset);
	else
# endif
	    gui_mch_set_font(gui.norm_font);
#endif
	gui_set_shellsize(FALSE, TRUE, RESIZE_BOTH);
    }

    return ret;
}

#ifdef USE_SET_GUIFONTWIDE
/*
 * Try setting 'guifontwide' to a font twice as wide as "name".
 */
    static void
set_guifontwide(char_u *name)
{
    int		i = 0;
    char_u	wide_name[FONTLEN + 10]; // room for 2 * width and '*'
    char_u	*wp = NULL;
    char_u	*p;
    GuiFont	font;

    wp = wide_name;
    for (p = name; *p != NUL; ++p)
    {
	*wp++ = *p;
	if (*p == '-')
	{
	    ++i;
	    if (i == 6)		// font type: change "--" to "-*-"
	    {
		if (p[1] == '-')
		    *wp++ = '*';
	    }
	    else if (i == 12)	// found the width
	    {
		++p;
		i = getdigits(&p);
		if (i != 0)
		{
		    // Double the width specification.
		    sprintf((char *)wp, "%d%s", i * 2, p);
		    font = gui_mch_get_font(wide_name, FALSE);
		    if (font != NOFONT)
		    {
			gui_mch_free_font(gui.wide_font);
			gui.wide_font = font;
			set_string_option_direct((char_u *)"gfw", -1,
						      wide_name, OPT_FREE, 0);
		    }
		}
		break;
	    }
	}
    }
}
#endif

/*
 * Get the font for 'guifontwide'.
 * Return FAIL for an invalid font name.
 */
    int
gui_get_wide_font(void)
{
    GuiFont	font = NOFONT;
    char_u	font_name[FONTLEN];
    char_u	*p;

    if (!gui.in_use)	    // Can't allocate font yet, assume it's OK.
	return OK;	    // Will give an error message later.

    if (p_guifontwide != NULL && *p_guifontwide != NUL)
    {
	for (p = p_guifontwide; *p != NUL; )
	{
	    // Isolate one comma separated font name.
	    (void)copy_option_part(&p, font_name, FONTLEN, ",");
	    font = gui_mch_get_font(font_name, FALSE);
	    if (font != NOFONT)
		break;
	}
	if (font == NOFONT)
	    return FAIL;
    }

    gui_mch_free_font(gui.wide_font);
#ifdef FEAT_GUI_GTK
    // Avoid unnecessary overhead if 'guifontwide' is equal to 'guifont'.
    if (font != NOFONT && gui.norm_font != NOFONT
			 && pango_font_description_equal(font, gui.norm_font))
    {
	gui.wide_font = NOFONT;
	gui_mch_free_font(font);
    }
    else
#endif
	gui.wide_font = font;
#ifdef FEAT_GUI_MSWIN
    gui_mch_wide_font_changed();
#else
    /*
     * TODO: setup wide_bold_font, wide_ital_font and wide_boldital_font to
     * support those fonts for 'guifontwide'.
     */
#endif
    return OK;
}

#if defined(FEAT_GUI_GTK) || defined(PROTO)
/*
 * Set list of ascii characters that combined can create ligature.
 * Store them in char map for quick access from gui_gtk2_draw_string.
 */
    void
gui_set_ligatures(void)
{
    char_u	*p;

    if (*p_guiligatures != NUL)
    {
	// check for invalid characters
	for (p = p_guiligatures; *p != NUL; ++p)
	    if (*p < 32 || *p > 127)
	    {
		emsg(_(e_ascii_code_not_in_range));
		return;
	    }

	// store valid setting into ligatures_map
	CLEAR_FIELD(gui.ligatures_map);
	for (p = p_guiligatures; *p != NUL; ++p)
	    gui.ligatures_map[*p] = 1;
    }
    else
	CLEAR_FIELD(gui.ligatures_map);
}

/*
 * Adjust the columns to undraw for when the cursor is on ligatures.
 */
    static void
gui_adjust_undraw_cursor_for_ligatures(int *startcol, int *endcol)
{
    int off;

    if (ScreenLines == NULL || *p_guiligatures == NUL)
	return;

    // expand before the cursor for all the chars in gui.ligatures_map
    off = LineOffset[gui.cursor_row] + *startcol;
    if (gui.ligatures_map[ScreenLines[off]])
	while (*startcol > 0 && gui.ligatures_map[ScreenLines[--off]])
	    (*startcol)--;

    // expand after the cursor for all the chars in gui.ligatures_map
    off = LineOffset[gui.cursor_row] + *endcol;
    if (gui.ligatures_map[ScreenLines[off]])
	while (*endcol < ((int)screen_Columns - 1)
				      && gui.ligatures_map[ScreenLines[++off]])
	   (*endcol)++;
}
#endif

    static void
gui_set_cursor(int row, int col)
{
    gui.row = row;
    gui.col = col;
}

/*
 * gui_check_pos - check if the cursor is on the screen.
 */
    static void
gui_check_pos(void)
{
    if (gui.row >= screen_Rows)
	gui.row = screen_Rows - 1;
    if (gui.col >= screen_Columns)
	gui.col = screen_Columns - 1;
    if (gui.cursor_row >= screen_Rows || gui.cursor_col >= screen_Columns)
	gui.cursor_is_valid = FALSE;
}

/*
 * Redraw the cursor if necessary or when forced.
 * Careful: The contents of ScreenLines[] must match what is on the screen,
 * otherwise this goes wrong.  May need to call out_flush() first.
 */
    void
gui_update_cursor(
    int		force,		 // when TRUE, update even when not moved
    int		clear_selection) // clear selection under cursor
{
    int		cur_width = 0;
    int		cur_height = 0;
    int		old_hl_mask;
    cursorentry_T *shape;
    int		id;
#ifdef FEAT_TERMINAL
    guicolor_T	shape_fg = INVALCOLOR;
    guicolor_T	shape_bg = INVALCOLOR;
#endif
    guicolor_T	cfg, cbg, cc;	// cursor fore-/background color
    int		cattr;		// cursor attributes
    int		attr;
    attrentry_T *aep = NULL;

    // Don't update the cursor when halfway busy scrolling or the screen size
    // doesn't match 'columns' and 'lines.  ScreenLines[] isn't valid then.
    if (!can_update_cursor || screen_Columns != gui.num_cols
					       || screen_Rows != gui.num_rows)
	return;

    gui_check_pos();

    if (gui.cursor_is_valid && !force
		&& gui.row == gui.cursor_row && gui.col == gui.cursor_col)
	return;

    gui_undraw_cursor();

    // If a cursor-less sleep is ongoing, leave the cursor invisible
    if (cursor_is_sleeping())
	return;

    if (gui.row < 0)
	return;
#ifdef HAVE_INPUT_METHOD
    if (gui.row != gui.cursor_row || gui.col != gui.cursor_col)
	im_set_position(gui.row, gui.col);
#endif
    gui.cursor_row = gui.row;
    gui.cursor_col = gui.col;

    // Only write to the screen after ScreenLines[] has been initialized
    if (!screen_cleared || ScreenLines == NULL)
	return;

    // Clear the selection if we are about to write over it
    if (clear_selection)
	clip_may_clear_selection(gui.row, gui.row);
    // Check that the cursor is inside the shell (resizing may have made
    // it invalid)
    if (gui.row >= screen_Rows || gui.col >= screen_Columns)
	return;

    gui.cursor_is_valid = TRUE;

    /*
     * How the cursor is drawn depends on the current mode.
     * When in a terminal window use the shape/color specified there.
     */
#ifdef FEAT_TERMINAL
    if (terminal_is_active())
	shape = term_get_cursor_shape(&shape_fg, &shape_bg);
    else
#endif
	shape = &shape_table[get_shape_idx(FALSE)];
    if (State & MODE_LANGMAP)
	id = shape->id_lm;
    else
	id = shape->id;

    // get the colors and attributes for the cursor.  Default is inverted
    cfg = INVALCOLOR;
    cbg = INVALCOLOR;
    cattr = HL_INVERSE;
    gui_mch_set_blinking(shape->blinkwait,
	    shape->blinkon,
	    shape->blinkoff);
    if (shape->blinkwait == 0 || shape->blinkon == 0
	    || shape->blinkoff == 0)
	gui_mch_stop_blink(FALSE);
#ifdef FEAT_TERMINAL
    if (shape_bg != INVALCOLOR)
    {
	cattr = 0;
	cfg = shape_fg;
	cbg = shape_bg;
    }
    else
#endif
	if (id > 0)
	{
	    cattr = syn_id2colors(id, &cfg, &cbg);
#if defined(HAVE_INPUT_METHOD)
	    {
		static int iid;
		guicolor_T fg, bg;

		if (
# if defined(FEAT_GUI_GTK) && defined(FEAT_XIM)
			preedit_get_status()
# else
			im_get_status()
# endif
		   )
		{
		    iid = syn_name2id((char_u *)"CursorIM");
		    if (iid > 0)
		    {
			syn_id2colors(iid, &fg, &bg);
			if (bg != INVALCOLOR)
			    cbg = bg;
			if (fg != INVALCOLOR)
			    cfg = fg;
		    }
		}
	    }
#endif
	}

    /*
     * Get the attributes for the character under the cursor.
     * When no cursor color was given, use the character color.
     */
    attr = ScreenAttrs[LineOffset[gui.row] + gui.col];
    if (attr > HL_ALL)
	aep = syn_gui_attr2entry(attr);
    if (aep != NULL)
    {
	attr = aep->ae_attr;
	if (cfg == INVALCOLOR)
	    cfg = ((attr & HL_INVERSE)  ? aep->ae_u.gui.bg_color
		    : aep->ae_u.gui.fg_color);
	if (cbg == INVALCOLOR)
	    cbg = ((attr & HL_INVERSE)  ? aep->ae_u.gui.fg_color
		    : aep->ae_u.gui.bg_color);
    }
    if (cfg == INVALCOLOR)
	cfg = (attr & HL_INVERSE) ? gui.back_pixel : gui.norm_pixel;
    if (cbg == INVALCOLOR)
	cbg = (attr & HL_INVERSE) ? gui.norm_pixel : gui.back_pixel;

#ifdef FEAT_XIM
    if (aep != NULL)
    {
	xim_bg_color = ((attr & HL_INVERSE) ? aep->ae_u.gui.fg_color
		: aep->ae_u.gui.bg_color);
	xim_fg_color = ((attr & HL_INVERSE) ? aep->ae_u.gui.bg_color
		: aep->ae_u.gui.fg_color);
	if (xim_bg_color == INVALCOLOR)
	    xim_bg_color = (attr & HL_INVERSE) ? gui.norm_pixel
		: gui.back_pixel;
	if (xim_fg_color == INVALCOLOR)
	    xim_fg_color = (attr & HL_INVERSE) ? gui.back_pixel
		: gui.norm_pixel;
    }
    else
    {
	xim_bg_color = (attr & HL_INVERSE) ? gui.norm_pixel
	    : gui.back_pixel;
	xim_fg_color = (attr & HL_INVERSE) ? gui.back_pixel
	    : gui.norm_pixel;
    }
#endif

    attr &= ~HL_INVERSE;
    if (cattr & HL_INVERSE)
    {
	cc = cbg;
	cbg = cfg;
	cfg = cc;
    }
    cattr &= ~HL_INVERSE;

    /*
     * When we don't have window focus, draw a hollow cursor.
     */
    if (!gui.in_focus)
    {
	gui_mch_draw_hollow_cursor(cbg);
	return;
    }

    old_hl_mask = gui.highlight_mask;
    if (shape->shape == SHAPE_BLOCK)
    {
	/*
	 * Draw the text character with the cursor colors.	Use the
	 * character attributes plus the cursor attributes.
	 */
	gui.highlight_mask = (cattr | attr);
	(void)gui_screenchar(LineOffset[gui.row] + gui.col,
		GUI_MON_IS_CURSOR | GUI_MON_NOCLEAR, cfg, cbg, 0);
    }
    else
    {
#if defined(FEAT_RIGHTLEFT)
	int	    col_off = FALSE;
#endif
	/*
	 * First draw the partial cursor, then overwrite with the text
	 * character, using a transparent background.
	 */
	if (shape->shape == SHAPE_VER)
	{
	    cur_height = gui.char_height;
	    cur_width = (gui.char_width * shape->percentage + 99) / 100;
	}
	else
	{
	    cur_height = (gui.char_height * shape->percentage + 99) / 100;
	    cur_width = gui.char_width;
	}
	if (has_mbyte && (*mb_off2cells)(LineOffset[gui.row] + gui.col,
		    LineOffset[gui.row] + screen_Columns) > 1)
	{
	    // Double wide character.
	    if (shape->shape != SHAPE_VER)
		cur_width += gui.char_width;
#ifdef FEAT_RIGHTLEFT
	    if (CURSOR_BAR_RIGHT)
	    {
		// gui.col points to the left half of the character but
		// the vertical line needs to be on the right half.
		// A double-wide horizontal line is also drawn from the
		// right half in gui_mch_draw_part_cursor().
		col_off = TRUE;
		++gui.col;
	    }
#endif
	}
	gui_mch_draw_part_cursor(cur_width, cur_height, cbg);
#if defined(FEAT_RIGHTLEFT)
	if (col_off)
	    --gui.col;
#endif

#ifndef FEAT_GUI_MSWIN	    // doesn't seem to work for MSWindows
	gui.highlight_mask = ScreenAttrs[LineOffset[gui.row] + gui.col];
	(void)gui_screenchar(LineOffset[gui.row] + gui.col,
		GUI_MON_TRS_CURSOR | GUI_MON_NOCLEAR,
		(guicolor_T)0, (guicolor_T)0, 0);
#endif
    }
    gui.highlight_mask = old_hl_mask;
}

#if defined(FEAT_MENU) || defined(PROTO)
    static void
gui_position_menu(void)
{
# if !defined(FEAT_GUI_GTK) && !defined(FEAT_GUI_MOTIF)
    if (gui.menu_is_active && gui.in_use)
	gui_mch_set_menu_pos(0, 0, gui.menu_width, gui.menu_height);
# endif
}
#endif

/*
 * Position the various GUI components (text area, menu).  The vertical
 * scrollbars are NOT handled here.  See gui_update_scrollbars().
 */
    static void
gui_position_components(int total_width UNUSED)
{
    int	    text_area_x;
    int	    text_area_y;
    int	    text_area_width;
    int	    text_area_height;

    // avoid that moving components around generates events
    ++hold_gui_events;

    text_area_x = 0;
    if (gui.which_scrollbars[SBAR_LEFT])
	text_area_x += gui.scrollbar_width;

    text_area_y = 0;
#if defined(FEAT_MENU) && !(defined(FEAT_GUI_GTK) || defined(FEAT_GUI_PHOTON))
    gui.menu_width = total_width;
    if (gui.menu_is_active)
	text_area_y += gui.menu_height;
#endif

#if defined(FEAT_GUI_TABLINE) && (defined(FEAT_GUI_MSWIN) \
	|| defined(FEAT_GUI_MOTIF))
    if (gui_has_tabline())
	text_area_y += gui.tabline_height;
#endif

#if defined(FEAT_TOOLBAR) && (defined(FEAT_GUI_MOTIF) \
	|| defined(FEAT_GUI_HAIKU) || defined(FEAT_GUI_MSWIN))
    if (vim_strchr(p_go, GO_TOOLBAR) != NULL)
    {
# if defined(FEAT_GUI_HAIKU)
	gui_mch_set_toolbar_pos(0, text_area_y,
				gui.menu_width, gui.toolbar_height);
# endif
	text_area_y += gui.toolbar_height;
    }
#endif

#if defined(FEAT_GUI_TABLINE) && defined(FEAT_GUI_HAIKU)
    gui_mch_set_tabline_pos(0, text_area_y,
    gui.menu_width, gui.tabline_height);
    if (gui_has_tabline())
	text_area_y += gui.tabline_height;
#endif

    text_area_width = gui.num_cols * gui.char_width + gui.border_offset * 2;
    text_area_height = gui.num_rows * gui.char_height + gui.border_offset * 2;

    gui_mch_set_text_area_pos(text_area_x,
			      text_area_y,
			      text_area_width,
			      text_area_height
#if defined(FEAT_XIM) && !defined(FEAT_GUI_GTK)
				  + xim_get_status_area_height()
#endif
			      );
#ifdef FEAT_MENU
    gui_position_menu();
#endif
    if (gui.which_scrollbars[SBAR_BOTTOM])
	gui_mch_set_scrollbar_pos(&gui.bottom_sbar,
				  text_area_x,
				  text_area_y + text_area_height
					+ gui_mch_get_scrollbar_ypadding(),
				  text_area_width,
				  gui.scrollbar_height);
    gui.left_sbar_x = 0;
    gui.right_sbar_x = text_area_x + text_area_width
					+ gui_mch_get_scrollbar_xpadding();

    --hold_gui_events;
}

/*
 * Get the width of the widgets and decorations to the side of the text area.
 */
    int
gui_get_base_width(void)
{
    int	    base_width;

    base_width = 2 * gui.border_offset;
    if (gui.which_scrollbars[SBAR_LEFT])
	base_width += gui.scrollbar_width;
    if (gui.which_scrollbars[SBAR_RIGHT])
	base_width += gui.scrollbar_width;
    return base_width;
}

/*
 * Get the height of the widgets and decorations above and below the text area.
 */
    int
gui_get_base_height(void)
{
    int	    base_height;

    base_height = 2 * gui.border_offset;
    if (gui.which_scrollbars[SBAR_BOTTOM])
	base_height += gui.scrollbar_height;
#ifdef FEAT_GUI_GTK
    // We can't take the sizes properly into account until anything is
    // realized.  Therefore we recalculate all the values here just before
    // setting the size. (--mdcki)
#else
# ifdef FEAT_MENU
    if (gui.menu_is_active)
	base_height += gui.menu_height;
# endif
# ifdef FEAT_TOOLBAR
    if (vim_strchr(p_go, GO_TOOLBAR) != NULL)
	base_height += gui.toolbar_height;
# endif
# if defined(FEAT_GUI_TABLINE) && (defined(FEAT_GUI_MSWIN) \
	|| defined(FEAT_GUI_MOTIF) || defined(FEAT_GUI_HAIKU))
    if (gui_has_tabline())
	base_height += gui.tabline_height;
# endif
# if defined(FEAT_GUI_MOTIF) && defined(FEAT_MENU)
    base_height += gui_mch_text_area_extra_height();
# endif
#endif
    return base_height;
}

/*
 * Should be called after the GUI shell has been resized.  Its arguments are
 * the new width and height of the shell in pixels.
 */
    void
gui_resize_shell(int pixel_width, int pixel_height)
{
    static int	busy = FALSE;

    if (!gui.shell_created)	    // ignore when still initializing
	return;

    /*
     * Can't resize the screen while it is being redrawn.  Remember the new
     * size and handle it later.
     */
    if (updating_screen || busy)
    {
	new_pixel_width = pixel_width;
	new_pixel_height = pixel_height;
	return;
    }

again:
    new_pixel_width = 0;
    new_pixel_height = 0;
    busy = TRUE;

#ifdef FEAT_GUI_HAIKU
    vim_lock_screen();
#endif

    // Flush pending output before redrawing
    out_flush();

    gui.num_cols = (pixel_width - gui_get_base_width()) / gui.char_width;
    gui.num_rows = (pixel_height - gui_get_base_height()) / gui.char_height;

    gui_position_components(pixel_width);
    gui_reset_scroll_region();

    /*
     * At the "more" and ":confirm" prompt there is no redraw, put the cursor
     * at the last line here (why does it have to be one row too low?).
     */
    if (State == MODE_ASKMORE || State == MODE_CONFIRM)
	gui.row = gui.num_rows;

    // Only comparing Rows and Columns may be sufficient, but let's stay on
    // the safe side.
    if (gui.num_rows != screen_Rows || gui.num_cols != screen_Columns
	    || gui.num_rows != Rows || gui.num_cols != Columns || gui.force_redraw)
    {
	shell_resized();
	gui.force_redraw = 0;
    }

#ifdef FEAT_GUI_HAIKU
    vim_unlock_screen();
#endif

    gui_update_scrollbars(TRUE);
    gui_update_cursor(FALSE, TRUE);
#if defined(FEAT_XIM) && !defined(FEAT_GUI_GTK)
    xim_set_status_area();
#endif

    busy = FALSE;

    // We may have been called again while redrawing the screen.
    // Need to do it all again with the latest size then.  But only if the size
    // actually changed.
    if (new_pixel_height)
    {
	if (pixel_width == new_pixel_width && pixel_height == new_pixel_height)
	{
	    new_pixel_width = 0;
	    new_pixel_height = 0;
	}
	else
	{
	    pixel_width = new_pixel_width;
	    pixel_height = new_pixel_height;
	    goto again;
	}
    }
}

/*
 * Check if gui_resize_shell() must be called.
 */
    void
gui_may_resize_shell(void)
{
    if (new_pixel_height)
	// careful: gui_resize_shell() may postpone the resize again if we
	// were called indirectly by it
	gui_resize_shell(new_pixel_width, new_pixel_height);
}

    int
gui_get_shellsize(void)
{
    Rows = gui.num_rows;
    Columns = gui.num_cols;
    return OK;
}

/*
 * Set the size of the Vim shell according to Rows and Columns.
 * If "fit_to_display" is TRUE then the size may be reduced to fit the window
 * on the screen.
 * When "mustset" is TRUE the size was set by the user. When FALSE a UI
 * component was added or removed (e.g., a scrollbar).
 */
    void
gui_set_shellsize(
    int		mustset UNUSED,
    int		fit_to_display,
    int		direction)		// RESIZE_HOR, RESIZE_VER
{
    int		base_width;
    int		base_height;
    int		width;
    int		height;
    int		min_width;
    int		min_height;
    int		screen_w;
    int		screen_h;
#ifdef FEAT_GUI_GTK
    int		un_maximize = mustset;
    int		did_adjust = 0;
#endif
    int		x = -1, y = -1;

    if (!gui.shell_created)
	return;

#if defined(MSWIN) || defined(FEAT_GUI_GTK)
    // If not setting to a user specified size and maximized, calculate the
    // number of characters that fit in the maximized window.
    if (!mustset && (vim_strchr(p_go, GO_KEEPWINSIZE) != NULL
						       || gui_mch_maximized()))
    {
	gui_mch_newfont();
	return;
    }
#endif

    base_width = gui_get_base_width();
    base_height = gui_get_base_height();
    if (fit_to_display)
	// Remember the original window position.
	(void)gui_mch_get_winpos(&x, &y);

    width = Columns * gui.char_width + base_width;
    height = Rows * gui.char_height + base_height;

    if (fit_to_display)
    {
	gui_mch_get_screen_dimensions(&screen_w, &screen_h);
	if ((direction & RESIZE_HOR) && width > screen_w)
	{
	    Columns = (screen_w - base_width) / gui.char_width;
	    if (Columns < MIN_COLUMNS)
		Columns = MIN_COLUMNS;
	    width = Columns * gui.char_width + base_width;
#ifdef FEAT_GUI_GTK
	    ++did_adjust;
#endif
	}
	if ((direction & RESIZE_VERT) && height > screen_h)
	{
	    Rows = (screen_h - base_height) / gui.char_height;
	    check_shellsize();
	    height = Rows * gui.char_height + base_height;
#ifdef FEAT_GUI_GTK
	    ++did_adjust;
#endif
	}
#ifdef FEAT_GUI_GTK
	if (did_adjust == 2 || (width + gui.char_width >= screen_w
				     && height + gui.char_height >= screen_h))
	    // don't unmaximize if at maximum size
	    un_maximize = FALSE;
#endif
    }
    limit_screen_size();
    gui.num_cols = Columns;
    gui.num_rows = Rows;

    min_width = base_width + MIN_COLUMNS * gui.char_width;
    min_height = base_height + MIN_LINES * gui.char_height;
    min_height += tabline_height() * gui.char_height;

#ifdef FEAT_GUI_GTK
    if (un_maximize)
    {
	// If the window size is smaller than the screen unmaximize the
	// window, otherwise resizing won't work.
	gui_mch_get_screen_dimensions(&screen_w, &screen_h);
	if ((width + gui.char_width < screen_w
				   || height + gui.char_height * 2 < screen_h)
		&& gui_mch_maximized())
	    gui_mch_unmaximize();
    }
#endif

    gui_mch_set_shellsize(width, height, min_width, min_height,
					  base_width, base_height, direction);

    if (fit_to_display && x >= 0 && y >= 0)
    {
	// Some window managers put the Vim window left of/above the screen.
	// Only change the position if it wasn't already negative before
	// (happens on MS-Windows with a secondary monitor).
	gui_mch_update();
	if (gui_mch_get_winpos(&x, &y) == OK && (x < 0 || y < 0))
	    gui_mch_set_winpos(x < 0 ? 0 : x, y < 0 ? 0 : y);
    }

    gui_position_components(width);
    gui_update_scrollbars(TRUE);
    gui_reset_scroll_region();
}

/*
 * Called when Rows and/or Columns has changed.
 */
    void
gui_new_shellsize(void)
{
    gui_reset_scroll_region();
}

/*
 * Make scroll region cover whole screen.
 */
    static void
gui_reset_scroll_region(void)
{
    gui.scroll_region_top = 0;
    gui.scroll_region_bot = gui.num_rows - 1;
    gui.scroll_region_left = 0;
    gui.scroll_region_right = gui.num_cols - 1;
}

    static void
gui_start_highlight(int mask)
{
    if (mask > HL_ALL)		    // highlight code
	gui.highlight_mask = mask;
    else			    // mask
	gui.highlight_mask |= mask;
}

    void
gui_stop_highlight(int mask)
{
    if (mask > HL_ALL)		    // highlight code
	gui.highlight_mask = HL_NORMAL;
    else			    // mask
	gui.highlight_mask &= ~mask;
}

/*
 * Clear a rectangular region of the screen from text pos (row1, col1) to
 * (row2, col2) inclusive.
 */
    void
gui_clear_block(
    int	    row1,
    int	    col1,
    int	    row2,
    int	    col2)
{
    // Clear the selection if we are about to write over it
    clip_may_clear_selection(row1, row2);

    gui_mch_clear_block(row1, col1, row2, col2);

    // Invalidate cursor if it was in this block
    if (       gui.cursor_row >= row1 && gui.cursor_row <= row2
	    && gui.cursor_col >= col1 && gui.cursor_col <= col2)
	gui.cursor_is_valid = FALSE;
}

/*
 * Write code to update the cursor later.  This avoids the need to flush the
 * output buffer before calling gui_update_cursor().
 */
    void
gui_update_cursor_later(void)
{
    OUT_STR("\033|s");
}

    void
gui_write(
    char_u	*s,
    int		len)
{
    char_u	*p;
    int		arg1 = 0, arg2 = 0;
    int		force_cursor = FALSE;	// force cursor update
    int		force_scrollbar = FALSE;
    static win_T	*old_curwin = NULL;

// #define DEBUG_GUI_WRITE
#ifdef DEBUG_GUI_WRITE
    {
	int i;
	char_u *str;

	printf("gui_write(%d):\n    ", len);
	for (i = 0; i < len; i++)
	    if (s[i] == ESC)
	    {
		if (i != 0)
		    printf("\n    ");
		printf("<ESC>");
	    }
	    else
	    {
		str = transchar_byte(s[i]);
		if (str[0] && str[1])
		    printf("<%s>", (char *)str);
		else
		    printf("%s", (char *)str);
	    }
	printf("\n");
    }
#endif
    while (len)
    {
	if (s[0] == ESC && s[1] == '|')
	{
	    p = s + 2;
	    if (VIM_ISDIGIT(*p) || (*p == '-' && VIM_ISDIGIT(*(p + 1))))
	    {
		arg1 = getdigits(&p);
		if (p > s + len)
		    break;
		if (*p == ';')
		{
		    ++p;
		    arg2 = getdigits(&p);
		    if (p > s + len)
			break;
		}
	    }
	    switch (*p)
	    {
		case 'C':	// Clear screen
		    clip_scroll_selection(9999);
		    gui_mch_clear_all();
		    gui.cursor_is_valid = FALSE;
		    force_scrollbar = TRUE;
		    break;
		case 'M':	// Move cursor
		    gui_set_cursor(arg1, arg2);
		    break;
		case 's':	// force cursor (shape) update
		    force_cursor = TRUE;
		    break;
		case 'R':	// Set scroll region
		    if (arg1 < arg2)
		    {
			gui.scroll_region_top = arg1;
			gui.scroll_region_bot = arg2;
		    }
		    else
		    {
			gui.scroll_region_top = arg2;
			gui.scroll_region_bot = arg1;
		    }
		    break;
		case 'V':	// Set vertical scroll region
		    if (arg1 < arg2)
		    {
			gui.scroll_region_left = arg1;
			gui.scroll_region_right = arg2;
		    }
		    else
		    {
			gui.scroll_region_left = arg2;
			gui.scroll_region_right = arg1;
		    }
		    break;
		case 'd':	// Delete line
		    gui_delete_lines(gui.row, 1);
		    break;
		case 'D':	// Delete lines
		    gui_delete_lines(gui.row, arg1);
		    break;
		case 'i':	// Insert line
		    gui_insert_lines(gui.row, 1);
		    break;
		case 'I':	// Insert lines
		    gui_insert_lines(gui.row, arg1);
		    break;
		case '$':	// Clear to end-of-line
		    gui_clear_block(gui.row, gui.col, gui.row,
							    (int)Columns - 1);
		    break;
		case 'h':	// Turn on highlighting
		    gui_start_highlight(arg1);
		    break;
		case 'H':	// Turn off highlighting
		    gui_stop_highlight(arg1);
		    break;
		case 'f':	// flash the window (visual bell)
		    gui_mch_flash(arg1 == 0 ? 20 : arg1);
		    break;
		default:
		    p = s + 1;	// Skip the ESC
		    break;
	    }
	    len -= (int)(++p - s);
	    s = p;
	}
	else if (s[0] < 0x20		// Ctrl character
#ifdef FEAT_SIGN_ICONS
		&& s[0] != SIGN_BYTE
# ifdef FEAT_NETBEANS_INTG
		&& s[0] != MULTISIGN_BYTE
# endif
#endif
		)
	{
	    if (s[0] == '\n')		// NL
	    {
		gui.col = 0;
		if (gui.row < gui.scroll_region_bot)
		    gui.row++;
		else
		    gui_delete_lines(gui.scroll_region_top, 1);
	    }
	    else if (s[0] == '\r')	// CR
	    {
		gui.col = 0;
	    }
	    else if (s[0] == '\b')	// Backspace
	    {
		if (gui.col)
		    --gui.col;
	    }
	    else if (s[0] == Ctrl_L)	// cursor-right
	    {
		++gui.col;
	    }
	    else if (s[0] == Ctrl_G)	// Beep
	    {
		gui_mch_beep();
	    }
	    // Other Ctrl character: shouldn't happen!

	    --len;	// Skip this char
	    ++s;
	}
	else
	{
	    p = s;
	    while (len > 0 && (
			*p >= 0x20
#ifdef FEAT_SIGN_ICONS
			|| *p == SIGN_BYTE
# ifdef FEAT_NETBEANS_INTG
			|| *p == MULTISIGN_BYTE
# endif
#endif
			))
	    {
		len--;
		p++;
	    }
	    gui_outstr(s, (int)(p - s));
	    s = p;
	}
    }

    // Postponed update of the cursor (won't work if "can_update_cursor" isn't
    // set).
    if (force_cursor)
	gui_update_cursor(TRUE, TRUE);

    // When switching to another window the dragging must have stopped.
    // Required for GTK, dragged_sb isn't reset.
    if (old_curwin != curwin)
	gui.dragged_sb = SBAR_NONE;

    // Update the scrollbars after clearing the screen or when switched
    // to another window.
    // Update the horizontal scrollbar always, it's difficult to check all
    // situations where it might change.
    if (force_scrollbar || old_curwin != curwin)
	gui_update_scrollbars(force_scrollbar);
    else
	gui_update_horiz_scrollbar(FALSE);
    old_curwin = curwin;

    /*
     * We need to make sure this is cleared since GTK doesn't tell us when
     * the user is done dragging.
     */
#if defined(FEAT_GUI_GTK)
    gui.dragged_sb = SBAR_NONE;
#endif

    gui_may_flush();		    // In case vim decides to take a nap
}

/*
 * When ScreenLines[] is invalid, updating the cursor should not be done, it
 * produces wrong results.  Call gui_dont_update_cursor() before that code and
 * gui_can_update_cursor() afterwards.
 */
    void
gui_dont_update_cursor(int undraw)
{
    if (!gui.in_use)
	return;

    // Undraw the cursor now, we probably can't do it after the change.
    if (undraw)
	gui_undraw_cursor();
    can_update_cursor = FALSE;
}

    void
gui_can_update_cursor(void)
{
    can_update_cursor = TRUE;
    // No need to update the cursor right now, there is always more output
    // after scrolling.
}

/*
 * Disable issuing gui_mch_flush().
 */
    void
gui_disable_flush(void)
{
    ++disable_flush;
}

/*
 * Enable issuing gui_mch_flush().
 */
    void
gui_enable_flush(void)
{
    --disable_flush;
}

/*
 * Issue gui_mch_flush() if it is not disabled.
 */
    void
gui_may_flush(void)
{
    if (disable_flush == 0)
	gui_mch_flush();
}

    static void
gui_outstr(char_u *s, int len)
{
    int	    this_len;
    int	    cells;

    if (len == 0)
	return;

    if (len < 0)
	len = (int)STRLEN(s);

    while (len > 0)
    {
	if (has_mbyte)
	{
	    // Find out how many chars fit in the current line.
	    cells = 0;
	    for (this_len = 0; this_len < len; )
	    {
		cells += (*mb_ptr2cells)(s + this_len);
		if (gui.col + cells > Columns)
		    break;
		this_len += (*mb_ptr2len)(s + this_len);
	    }
	    if (this_len > len)
		this_len = len;	    // don't include following composing char
	}
	else if (gui.col + len > Columns)
	    this_len = Columns - gui.col;
	else
	    this_len = len;

	(void)gui_outstr_nowrap(s, this_len,
					  0, (guicolor_T)0, (guicolor_T)0, 0);
	s += this_len;
	len -= this_len;
	// fill up for a double-width char that doesn't fit.
	if (len > 0 && gui.col < Columns)
	    (void)gui_outstr_nowrap((char_u *)" ", 1,
					  0, (guicolor_T)0, (guicolor_T)0, 0);
	// The cursor may wrap to the next line.
	if (gui.col >= Columns)
	{
	    gui.col = 0;
	    gui.row++;
	}
    }
}

/*
 * Output one character (may be one or two display cells).
 * Caller must check for valid "off".
 * Returns FAIL or OK, just like gui_outstr_nowrap().
 */
    static int
gui_screenchar(
    int		off,	    // Offset from start of screen
    int		flags,
    guicolor_T	fg,	    // colors for cursor
    guicolor_T	bg,	    // colors for cursor
    int		back)	    // backup this many chars when using bold trick
{
    char_u	buf[MB_MAXBYTES + 1];

    // Don't draw right half of a double-width UTF-8 char. "cannot happen"
    if (enc_utf8 && ScreenLines[off] == 0)
	return OK;

    if (enc_utf8 && ScreenLinesUC[off] != 0)
	// Draw UTF-8 multi-byte character.
	return gui_outstr_nowrap(buf, utfc_char2bytes(off, buf),
							 flags, fg, bg, back);

    if (enc_dbcs == DBCS_JPNU && ScreenLines[off] == 0x8e)
    {
	buf[0] = ScreenLines[off];
	buf[1] = ScreenLines2[off];
	return gui_outstr_nowrap(buf, 2, flags, fg, bg, back);
    }

    // Draw non-multi-byte character or DBCS character.
    return gui_outstr_nowrap(ScreenLines + off,
	    enc_dbcs ? (*mb_ptr2len)(ScreenLines + off) : 1,
							 flags, fg, bg, back);
}

#ifdef FEAT_GUI_GTK
/*
 * Output the string at the given screen position.  This is used in place
 * of gui_screenchar() where possible because Pango needs as much context
 * as possible to work nicely.  It's a lot faster as well.
 */
    static int
gui_screenstr(
    int		off,	    // Offset from start of screen
    int		len,	    // string length in screen cells
    int		flags,
    guicolor_T	fg,	    // colors for cursor
    guicolor_T	bg,	    // colors for cursor
    int		back)	    // backup this many chars when using bold trick
{
    char_u  *buf;
    int	    outlen = 0;
    int	    i;
    int	    retval;

    if (len <= 0) // "cannot happen"?
	return OK;

    if (enc_utf8)
    {
	buf = alloc(len * MB_MAXBYTES + 1);
	if (buf == NULL)
	    return OK; // not much we could do here...

	for (i = off; i < off + len; ++i)
	{
	    if (ScreenLines[i] == 0)
		continue; // skip second half of double-width char

	    if (ScreenLinesUC[i] == 0)
		buf[outlen++] = ScreenLines[i];
	    else
		outlen += utfc_char2bytes(i, buf + outlen);
	}

	buf[outlen] = NUL; // only to aid debugging
	retval = gui_outstr_nowrap(buf, outlen, flags, fg, bg, back);
	vim_free(buf);

	return retval;
    }
    else if (enc_dbcs == DBCS_JPNU)
    {
	buf = alloc(len * 2 + 1);
	if (buf == NULL)
	    return OK; // not much we could do here...

	for (i = off; i < off + len; ++i)
	{
	    buf[outlen++] = ScreenLines[i];

	    // handle double-byte single-width char
	    if (ScreenLines[i] == 0x8e)
		buf[outlen++] = ScreenLines2[i];
	    else if (MB_BYTE2LEN(ScreenLines[i]) == 2)
		buf[outlen++] = ScreenLines[++i];
	}

	buf[outlen] = NUL; // only to aid debugging
	retval = gui_outstr_nowrap(buf, outlen, flags, fg, bg, back);
	vim_free(buf);

	return retval;
    }
    else
    {
	return gui_outstr_nowrap(&ScreenLines[off], len,
				 flags, fg, bg, back);
    }
}
#endif // FEAT_GUI_GTK

/*
 * Output the given string at the current cursor position.  If the string is
 * too long to fit on the line, then it is truncated.
 * "flags":
 * GUI_MON_IS_CURSOR should only be used when this function is being called to
 * actually draw (an inverted) cursor.
 * GUI_MON_TRS_CURSOR is used to draw the cursor text with a transparent
 * background.
 * GUI_MON_NOCLEAR is used to avoid clearing the selection when drawing over
 * it.
 * Returns OK, unless "back" is non-zero and using the bold trick, then return
 * FAIL (the caller should start drawing "back" chars back).
 */
    static int
gui_outstr_nowrap(
    char_u	*s,
    int		len,
    int		flags,
    guicolor_T	fg,	    // colors for cursor
    guicolor_T	bg,	    // colors for cursor
    int		back)	    // backup this many chars when using bold trick
{
    long_u	highlight_mask;
    long_u	hl_mask_todo;
    guicolor_T	fg_color;
    guicolor_T	bg_color;
    guicolor_T	sp_color;
#if !defined(FEAT_GUI_GTK)
    GuiFont	font = NOFONT;
    GuiFont	wide_font = NOFONT;
# ifdef FEAT_XFONTSET
    GuiFontset	fontset = NOFONTSET;
# endif
#endif
    attrentry_T	*aep = NULL;
    int		draw_flags;
    int		col = gui.col;
#ifdef FEAT_SIGN_ICONS
    int		draw_sign = FALSE;
    int		signcol = 0;
    char_u	extra[18];
# ifdef FEAT_NETBEANS_INTG
    int		multi_sign = FALSE;
# endif
#endif

    if (len < 0)
	len = (int)STRLEN(s);
    if (len == 0)
	return OK;

#ifdef FEAT_SIGN_ICONS
    if (*s == SIGN_BYTE
# ifdef FEAT_NETBEANS_INTG
	  || *s == MULTISIGN_BYTE
# endif
       )
    {
# ifdef FEAT_NETBEANS_INTG
	if (*s == MULTISIGN_BYTE)
	    multi_sign = TRUE;
# endif
	// draw spaces instead
	if (*curwin->w_p_scl == 'n' && *(curwin->w_p_scl + 1) == 'u' &&
		(curwin->w_p_nu || curwin->w_p_rnu))
	{
	    sprintf((char *)extra, "%*c ", number_width(curwin), ' ');
	    s = extra;
	}
	else
	    s = (char_u *)"  ";
	if (len == 1 && col > 0)
	    --col;
	len = (int)STRLEN(s);
	if (len > 2)
	    // right align sign icon in the number column
	    signcol = col + len - 3;
	else
	    signcol = col;
	draw_sign = TRUE;
	highlight_mask = 0;
    }
    else
#endif
    if (gui.highlight_mask > HL_ALL)
    {
	aep = syn_gui_attr2entry(gui.highlight_mask);
	if (aep == NULL)	    // highlighting not set
	    highlight_mask = 0;
	else
	    highlight_mask = aep->ae_attr;
    }
    else
	highlight_mask = gui.highlight_mask;
    hl_mask_todo = highlight_mask;

#if !defined(FEAT_GUI_GTK)
    // Set the font
    if (aep != NULL && aep->ae_u.gui.font != NOFONT)
	font = aep->ae_u.gui.font;
# ifdef FEAT_XFONTSET
    else if (aep != NULL && aep->ae_u.gui.fontset != NOFONTSET)
	fontset = aep->ae_u.gui.fontset;
# endif
    else
    {
# ifdef FEAT_XFONTSET
	if (gui.fontset != NOFONTSET)
	    fontset = gui.fontset;
	else
# endif
	    if (hl_mask_todo & (HL_BOLD | HL_STANDOUT))
	{
	    if ((hl_mask_todo & HL_ITALIC) && gui.boldital_font != NOFONT)
	    {
		font = gui.boldital_font;
		hl_mask_todo &= ~(HL_BOLD | HL_STANDOUT | HL_ITALIC);
	    }
	    else if (gui.bold_font != NOFONT)
	    {
		font = gui.bold_font;
		hl_mask_todo &= ~(HL_BOLD | HL_STANDOUT);
	    }
	    else
		font = gui.norm_font;
	}
	else if ((hl_mask_todo & HL_ITALIC) && gui.ital_font != NOFONT)
	{
	    font = gui.ital_font;
	    hl_mask_todo &= ~HL_ITALIC;
	}
	else
	    font = gui.norm_font;

	/*
	 * Choose correct wide_font by font.  wide_font should be set with font
	 * at same time in above block.  But it will make many "ifdef" nasty
	 * blocks.  So we do it here.
	 */
	if (font == gui.boldital_font && gui.wide_boldital_font)
	    wide_font = gui.wide_boldital_font;
	else if (font == gui.bold_font && gui.wide_bold_font)
	    wide_font = gui.wide_bold_font;
	else if (font == gui.ital_font && gui.wide_ital_font)
	    wide_font = gui.wide_ital_font;
	else if (font == gui.norm_font && gui.wide_font)
	    wide_font = gui.wide_font;
    }
# ifdef FEAT_XFONTSET
    if (fontset != NOFONTSET)
	gui_mch_set_fontset(fontset);
    else
# endif
	gui_mch_set_font(font);
#endif

    draw_flags = 0;

    // Set the color
    bg_color = gui.back_pixel;
    if ((flags & GUI_MON_IS_CURSOR) && gui.in_focus)
    {
	draw_flags |= DRAW_CURSOR;
	fg_color = fg;
	bg_color = bg;
	sp_color = fg;
    }
    else if (aep != NULL)
    {
	fg_color = aep->ae_u.gui.fg_color;
	if (fg_color == INVALCOLOR)
	    fg_color = gui.norm_pixel;
	bg_color = aep->ae_u.gui.bg_color;
	if (bg_color == INVALCOLOR)
	    bg_color = gui.back_pixel;
	sp_color = aep->ae_u.gui.sp_color;
	if (sp_color == INVALCOLOR)
	    sp_color = fg_color;
    }
    else
    {
	fg_color = gui.norm_pixel;
	sp_color = fg_color;
    }

    if (highlight_mask & (HL_INVERSE | HL_STANDOUT))
    {
	gui_mch_set_fg_color(bg_color);
	gui_mch_set_bg_color(fg_color);
    }
    else
    {
	gui_mch_set_fg_color(fg_color);
	gui_mch_set_bg_color(bg_color);
    }
    gui_mch_set_sp_color(sp_color);

    // Clear the selection if we are about to write over it
    if (!(flags & GUI_MON_NOCLEAR))
	clip_may_clear_selection(gui.row, gui.row);


    // If there's no bold font, then fake it
    if (hl_mask_todo & (HL_BOLD | HL_STANDOUT))
	draw_flags |= DRAW_BOLD;

    /*
     * When drawing bold or italic characters the spill-over from the left
     * neighbor may be destroyed.  Let the caller backup to start redrawing
     * just after a blank.
     */
    if (back != 0 && ((draw_flags & DRAW_BOLD) || (highlight_mask & HL_ITALIC)))
	return FAIL;

#if defined(FEAT_GUI_GTK)
    // If there's no italic font, then fake it.
    // For GTK2, we don't need a different font for italic style.
    if (hl_mask_todo & HL_ITALIC)
	draw_flags |= DRAW_ITALIC;

    // Do we underline the text?
    if (hl_mask_todo & HL_UNDERLINE)
	draw_flags |= DRAW_UNDERL;

#else
    // Do we underline the text?
    if ((hl_mask_todo & HL_UNDERLINE) || (hl_mask_todo & HL_ITALIC))
	draw_flags |= DRAW_UNDERL;
#endif
    // Do we undercurl the text?
    if (hl_mask_todo & HL_UNDERCURL)
	draw_flags |= DRAW_UNDERC;

    // TODO: HL_UNDERDOUBLE, HL_UNDERDOTTED, HL_UNDERDASHED

    // Do we strikethrough the text?
    if (hl_mask_todo & HL_STRIKETHROUGH)
	draw_flags |= DRAW_STRIKE;

    // Do we draw transparently?
    if (flags & GUI_MON_TRS_CURSOR)
	draw_flags |= DRAW_TRANSP;

    /*
     * Draw the text.
     */
#ifdef FEAT_GUI_GTK
    // The value returned is the length in display cells
    len = gui_gtk2_draw_string(gui.row, col, s, len, draw_flags);
#else
    if (enc_utf8)
    {
	int	start;		// index of bytes to be drawn
	int	cells;		// cellwidth of bytes to be drawn
	int	thislen;	// length of bytes to be drawn
	int	cn;		// cellwidth of current char
	int	i;		// index of current char
	int	c;		// current char value
	int	cl;		// byte length of current char
	int	comping;	// current char is composing
	int	scol = col;	// screen column
	int	curr_wide = FALSE;  // use 'guifontwide'
	int	prev_wide = FALSE;
	int	wide_changed;
# ifdef MSWIN
	int	sep_comp = FALSE;   // Don't separate composing chars.
# else
	int	sep_comp = TRUE;    // Separate composing chars.
# endif

	// Break the string at a composing character, it has to be drawn on
	// top of the previous character.
	start = 0;
	cells = 0;
	for (i = 0; i < len; i += cl)
	{
	    c = utf_ptr2char(s + i);
	    cn = utf_char2cells(c);
	    comping = utf_iscomposing(c);
	    if (!comping)	// count cells from non-composing chars
		cells += cn;
	    if (!comping || sep_comp)
	    {
		if (cn > 1
# ifdef FEAT_XFONTSET
			&& fontset == NOFONTSET
# endif
			&& wide_font != NOFONT)
		    curr_wide = TRUE;
		else
		    curr_wide = FALSE;
	    }
	    cl = utf_ptr2len(s + i);
	    if (cl == 0)	// hit end of string
		len = i + cl;	// len must be wrong "cannot happen"

	    wide_changed = curr_wide != prev_wide;

	    // Print the string so far if it's the last character or there is
	    // a composing character.
	    if (i + cl >= len || (comping && sep_comp && i > start)
		    || wide_changed
# if defined(FEAT_GUI_X11)
		    || (cn > 1
#  ifdef FEAT_XFONTSET
			// No fontset: At least draw char after wide char at
			// right position.
			&& fontset == NOFONTSET
#  endif
		       )
# endif
	       )
	    {
		if ((comping && sep_comp) || wide_changed)
		    thislen = i - start;
		else
		    thislen = i - start + cl;
		if (thislen > 0)
		{
		    if (prev_wide)
			gui_mch_set_font(wide_font);
		    gui_mch_draw_string(gui.row, scol, s + start, thislen,
								  draw_flags);
		    if (prev_wide)
			gui_mch_set_font(font);
		    start += thislen;
		}
		scol += cells;
		cells = 0;
		// Adjust to not draw a character which width is changed
		// against with last one.
		if (wide_changed && !(comping && sep_comp))
		{
		    scol -= cn;
		    cl = 0;
		}

# if defined(FEAT_GUI_X11)
		// No fontset: draw a space to fill the gap after a wide char
		//
		if (cn > 1 && (draw_flags & DRAW_TRANSP) == 0
#  ifdef FEAT_XFONTSET
			&& fontset == NOFONTSET
#  endif
			&& !wide_changed)
		    gui_mch_draw_string(gui.row, scol - 1, (char_u *)" ",
							       1, draw_flags);
# endif
	    }
	    // Draw a composing char on top of the previous char.
	    if (comping && sep_comp)
	    {
# if defined(__APPLE_CC__) && TARGET_API_MAC_CARBON
		// Carbon ATSUI autodraws composing char over previous char
		gui_mch_draw_string(gui.row, scol, s + i, cl,
						    draw_flags | DRAW_TRANSP);
# else
		gui_mch_draw_string(gui.row, scol - cn, s + i, cl,
						    draw_flags | DRAW_TRANSP);
# endif
		start = i + cl;
	    }
	    prev_wide = curr_wide;
	}
	// The stuff below assumes "len" is the length in screen columns.
	len = scol - col;
    }
    else
    {
	gui_mch_draw_string(gui.row, col, s, len, draw_flags);
	if (enc_dbcs == DBCS_JPNU)
	{
	    // Get the length in display cells, this can be different from the
	    // number of bytes for "euc-jp".
	    len = mb_string2cells(s, len);
	}
    }
#endif // !FEAT_GUI_GTK

    if (!(flags & (GUI_MON_IS_CURSOR | GUI_MON_TRS_CURSOR)))
	gui.col = col + len;

    // May need to invert it when it's part of the selection.
    if (flags & GUI_MON_NOCLEAR)
	clip_may_redraw_selection(gui.row, col, len);

    if (!(flags & (GUI_MON_IS_CURSOR | GUI_MON_TRS_CURSOR)))
    {
	// Invalidate the old physical cursor position if we wrote over it
	if (gui.cursor_row == gui.row
		&& gui.cursor_col >= col
		&& gui.cursor_col < col + len)
	    gui.cursor_is_valid = FALSE;
    }

#ifdef FEAT_SIGN_ICONS
    if (draw_sign)
	// Draw the sign on top of the spaces.
	gui_mch_drawsign(gui.row, signcol, gui.highlight_mask);
# if defined(FEAT_NETBEANS_INTG) && (defined(FEAT_GUI_X11) \
	|| defined(FEAT_GUI_GTK) || defined(FEAT_GUI_MSWIN))
    if (multi_sign)
	netbeans_draw_multisign_indicator(gui.row);
# endif
#endif

    return OK;
}

/*
 * Undraw the cursor.  This actually redraws the character at the cursor
 * position, plus some more characters when needed.
 */
    void
gui_undraw_cursor(void)
{
    if (!gui.cursor_is_valid)
	return;

    // Always redraw the character just before if there is one, because
    // with some fonts and characters there can be a one pixel overlap.
    int startcol = gui.cursor_col > 0 ? gui.cursor_col - 1 : gui.cursor_col;
    int endcol = gui.cursor_col;

#ifdef FEAT_GUI_GTK
    gui_adjust_undraw_cursor_for_ligatures(&startcol, &endcol);
#endif
    gui_redraw_block(gui.cursor_row, startcol,
	    gui.cursor_row, endcol, GUI_MON_NOCLEAR);

    // Cursor_is_valid is reset when the cursor is undrawn, also reset it
    // here in case it wasn't needed to undraw it.
    gui.cursor_is_valid = FALSE;
}

    void
gui_redraw(
    int		x,
    int		y,
    int		w,
    int		h)
{
    int		row1, col1, row2, col2;

    row1 = Y_2_ROW(y);
    col1 = X_2_COL(x);
    row2 = Y_2_ROW(y + h - 1);
    col2 = X_2_COL(x + w - 1);

    gui_redraw_block(row1, col1, row2, col2, GUI_MON_NOCLEAR);

    /*
     * We may need to redraw the cursor, but don't take it upon us to change
     * its location after a scroll.
     * (maybe be more strict even and test col too?)
     * These things may be outside the update/clipping region and reality may
     * not reflect Vims internal ideas if these operations are clipped away.
     */
    if (gui.row == gui.cursor_row)
	gui_update_cursor(TRUE, TRUE);
}

/*
 * Draw a rectangular block of characters, from row1 to row2 (inclusive) and
 * from col1 to col2 (inclusive).
 */
    void
gui_redraw_block(
    int		row1,
    int		col1,
    int		row2,
    int		col2,
    int		flags)	// flags for gui_outstr_nowrap()
{
    int		old_row, old_col;
    long_u	old_hl_mask;
    int		off;
    sattr_T	first_attr;
    int		idx, len;
    int		back, nback;
    int		orig_col1, orig_col2;

    // Don't try to update when ScreenLines is not valid
    if (!screen_cleared || ScreenLines == NULL)
	return;

    // Don't try to draw outside the shell!
    // Check everything, strange values may be caused by a big border width
    col1 = check_col(col1);
    col2 = check_col(col2);
    row1 = check_row(row1);
    row2 = check_row(row2);

    // Remember where our cursor was
    old_row = gui.row;
    old_col = gui.col;
    old_hl_mask = gui.highlight_mask;
    orig_col1 = col1;
    orig_col2 = col2;

    for (gui.row = row1; gui.row <= row2; gui.row++)
    {
	// When only half of a double-wide character is in the block, include
	// the other half.
	col1 = orig_col1;
	col2 = orig_col2;
	off = LineOffset[gui.row];
	if (enc_dbcs != 0)
	{
	    if (col1 > 0)
		col1 -= dbcs_screen_head_off(ScreenLines + off,
						    ScreenLines + off + col1);
	    col2 += dbcs_screen_tail_off(ScreenLines + off,
						    ScreenLines + off + col2);
	}
	else if (enc_utf8)
	{
	    if (ScreenLines[off + col1] == 0)
	    {
		if (col1 > 0)
		    --col1;
		else
		    // FIXME: how can the first character ever be zero?
		    siemsg("NUL in ScreenLines in row %ld", (long)gui.row);
	    }
#ifdef FEAT_GUI_GTK
	    if (col2 + 1 < Columns && ScreenLines[off + col2 + 1] == 0)
		++col2;
#endif
	}
	gui.col = col1;
	off = LineOffset[gui.row] + gui.col;
	len = col2 - col1 + 1;

	// Find how many chars back this highlighting starts, or where a space
	// is.  Needed for when the bold trick is used
	for (back = 0; back < col1; ++back)
	    if (ScreenAttrs[off - 1 - back] != ScreenAttrs[off]
		    || ScreenLines[off - 1 - back] == ' ')
		break;

	// Break it up in strings of characters with the same attributes.
	// Print UTF-8 characters individually.
	while (len > 0)
	{
	    first_attr = ScreenAttrs[off];
	    gui.highlight_mask = first_attr;
#if !defined(FEAT_GUI_GTK)
	    if (enc_utf8 && ScreenLinesUC[off] != 0)
	    {
		// output multi-byte character separately
		nback = gui_screenchar(off, flags,
					  (guicolor_T)0, (guicolor_T)0, back);
		if (gui.col < Columns && ScreenLines[off + 1] == 0)
		    idx = 2;
		else
		    idx = 1;
	    }
	    else if (enc_dbcs == DBCS_JPNU && ScreenLines[off] == 0x8e)
	    {
		// output double-byte, single-width character separately
		nback = gui_screenchar(off, flags,
					  (guicolor_T)0, (guicolor_T)0, back);
		idx = 1;
	    }
	    else
#endif
	    {
#ifdef FEAT_GUI_GTK
		for (idx = 0; idx < len; ++idx)
		{
		    if (enc_utf8 && ScreenLines[off + idx] == 0)
			continue; // skip second half of double-width char
		    if (ScreenAttrs[off + idx] != first_attr)
			break;
		}
		// gui_screenstr() takes care of multibyte chars
		nback = gui_screenstr(off, idx, flags,
				      (guicolor_T)0, (guicolor_T)0, back);
#else
		for (idx = 0; idx < len && ScreenAttrs[off + idx] == first_attr;
									idx++)
		{
		    // Stop at a multi-byte Unicode character.
		    if (enc_utf8 && ScreenLinesUC[off + idx] != 0)
			break;
		    if (enc_dbcs == DBCS_JPNU)
		    {
			// Stop at a double-byte single-width char.
			if (ScreenLines[off + idx] == 0x8e)
			    break;
			if (len > 1 && (*mb_ptr2len)(ScreenLines
							    + off + idx) == 2)
			    ++idx;  // skip second byte of double-byte char
		    }
		}
		nback = gui_outstr_nowrap(ScreenLines + off, idx, flags,
					  (guicolor_T)0, (guicolor_T)0, back);
#endif
	    }
	    if (nback == FAIL)
	    {
		// Must back up to start drawing where a bold or italic word
		// starts.
		off -= back;
		len += back;
		gui.col -= back;
	    }
	    else
	    {
		off += idx;
		len -= idx;
	    }
	    back = 0;
	}
    }

    // Put the cursor back where it was
    gui.row = old_row;
    gui.col = old_col;
    gui.highlight_mask = (int)old_hl_mask;
}

    static void
gui_delete_lines(int row, int count)
{
    if (count <= 0)
	return;

    if (row + count > gui.scroll_region_bot)
	// Scrolled out of region, just blank the lines out
	gui_clear_block(row, gui.scroll_region_left,
			      gui.scroll_region_bot, gui.scroll_region_right);
    else
    {
	gui_mch_delete_lines(row, count);

	// If the cursor was in the deleted lines it's now gone.  If the
	// cursor was in the scrolled lines adjust its position.
	if (gui.cursor_row >= row
		&& gui.cursor_col >= gui.scroll_region_left
		&& gui.cursor_col <= gui.scroll_region_right)
	{
	    if (gui.cursor_row < row + count)
		gui.cursor_is_valid = FALSE;
	    else if (gui.cursor_row <= gui.scroll_region_bot)
		gui.cursor_row -= count;
	}
    }
}

    static void
gui_insert_lines(int row, int count)
{
    if (count <= 0)
	return;

    if (row + count > gui.scroll_region_bot)
	// Scrolled out of region, just blank the lines out
	gui_clear_block(row, gui.scroll_region_left,
			      gui.scroll_region_bot, gui.scroll_region_right);
    else
    {
	gui_mch_insert_lines(row, count);

	if (gui.cursor_row >= gui.row
		&& gui.cursor_col >= gui.scroll_region_left
		&& gui.cursor_col <= gui.scroll_region_right)
	{
	    if (gui.cursor_row <= gui.scroll_region_bot - count)
		gui.cursor_row += count;
	    else if (gui.cursor_row <= gui.scroll_region_bot)
		gui.cursor_is_valid = FALSE;
	}
    }
}

#ifdef FEAT_TIMERS
/*
 * Passed to ui_wait_for_chars_or_timer(), ignoring extra arguments.
 */
    static int
gui_wait_for_chars_3(
    long wtime,
    int *interrupted UNUSED,
    int ignore_input UNUSED)
{
    return gui_mch_wait_for_chars(wtime);
}
#endif

/*
 * Returns OK if a character was found to be available within the given time,
 * or FAIL otherwise.
 */
    static int
gui_wait_for_chars_or_timer(
	long wtime,
	int *interrupted UNUSED,
	int ignore_input UNUSED)
{
#ifdef FEAT_TIMERS
    return ui_wait_for_chars_or_timer(wtime, gui_wait_for_chars_3,
						    interrupted, ignore_input);
#else
    return gui_mch_wait_for_chars(wtime);
#endif
}

/*
 * The main GUI input routine.	Waits for a character from the keyboard.
 * "wtime" == -1    Wait forever.
 * "wtime" == 0	    Don't wait.
 * "wtime" > 0	    Wait wtime milliseconds for a character.
 *
 * Returns the number of characters read or zero when timed out or interrupted.
 * "buf" may be NULL, in which case a non-zero number is returned if characters
 * are available.
 */
    static int
gui_wait_for_chars_buf(
    char_u	*buf,
    int		maxlen,
    long	wtime,	    // don't use "time", MIPS cannot handle it
    int		tb_change_cnt)
{
    int	    retval;

#ifdef FEAT_MENU
    // If we're going to wait a bit, update the menus and mouse shape for the
    // current State.
    if (wtime != 0)
	gui_update_menus(0);
#endif

    gui_mch_update();
    if (input_available())	// Got char, return immediately
    {
	if (buf != NULL && !typebuf_changed(tb_change_cnt))
	    return read_from_input_buf(buf, (long)maxlen);
	return 0;
    }
    if (wtime == 0)		// Don't wait for char
	return FAIL;

    // Before waiting, flush any output to the screen.
    gui_mch_flush();

    // Blink while waiting for a character.
    gui_mch_start_blink();

    // Common function to loop until "wtime" is met, while handling timers and
    // other callbacks.
    retval = inchar_loop(buf, maxlen, wtime, tb_change_cnt,
			 gui_wait_for_chars_or_timer, NULL);

    gui_mch_stop_blink(TRUE);

    return retval;
}

/*
 * Wait for a character from the keyboard without actually reading it.
 * Also deals with timers.
 * wtime == -1	    Wait forever.
 * wtime == 0	    Don't wait.
 * wtime > 0	    Wait wtime milliseconds for a character.
 * Returns OK if a character was found to be available within the given time,
 * or FAIL otherwise.
 */
    int
gui_wait_for_chars(long wtime, int tb_change_cnt)
{
    return gui_wait_for_chars_buf(NULL, 0, wtime, tb_change_cnt);
}

/*
 * Equivalent of mch_inchar() for the GUI.
 */
    int
gui_inchar(
    char_u  *buf,
    int	    maxlen,
    long    wtime,		// milliseconds
    int	    tb_change_cnt)
{
    return gui_wait_for_chars_buf(buf, maxlen, wtime, tb_change_cnt);
}

/*
 * Fill p[4] with mouse coordinates encoded for check_termcode().
 */
    static void
fill_mouse_coord(char_u *p, int col, int row)
{
    p[0] = (char_u)(col / 128 + ' ' + 1);
    p[1] = (char_u)(col % 128 + ' ' + 1);
    p[2] = (char_u)(row / 128 + ' ' + 1);
    p[3] = (char_u)(row % 128 + ' ' + 1);
}

/*
 * Generic mouse support function.  Add a mouse event to the input buffer with
 * the given properties.
 *  button	    --- may be any of MOUSE_LEFT, MOUSE_MIDDLE, MOUSE_RIGHT,
 *			MOUSE_X1, MOUSE_X2
 *			MOUSE_DRAG, or MOUSE_RELEASE.
 *			MOUSE_4 and MOUSE_5 are used for vertical scroll wheel,
 *			MOUSE_6 and MOUSE_7 for horizontal scroll wheel.
 *  x, y	    --- Coordinates of mouse in pixels.
 *  repeated_click  --- TRUE if this click comes only a short time after a
 *			previous click.
 *  modifiers	    --- Bit field which may be any of the following modifiers
 *			or'ed together: MOUSE_SHIFT | MOUSE_CTRL | MOUSE_ALT.
 * This function will ignore drag events where the mouse has not moved to a new
 * character.
 */
    void
gui_send_mouse_event(
    int	    button,
    int	    x,
    int	    y,
    int	    repeated_click,
    int_u   modifiers)
{
    static int	    prev_row = 0, prev_col = 0;
    static int	    prev_button = -1;
    static int	    num_clicks = 1;
    char_u	    string[10];
    enum key_extra  button_char;
    int		    row, col;
#ifdef FEAT_CLIPBOARD
    int		    checkfor;
    int		    did_clip = FALSE;
#endif

    /*
     * Scrolling may happen at any time, also while a selection is present.
     */
    switch (button)
    {
	case MOUSE_MOVE:
	    button_char = KE_MOUSEMOVE_XY;
	    goto button_set;
	case MOUSE_X1:
	    button_char = KE_X1MOUSE;
	    goto button_set;
	case MOUSE_X2:
	    button_char = KE_X2MOUSE;
	    goto button_set;
	case MOUSE_4:
	    button_char = KE_MOUSEDOWN;
	    goto button_set;
	case MOUSE_5:
	    button_char = KE_MOUSEUP;
	    goto button_set;
	case MOUSE_6:
	    button_char = KE_MOUSELEFT;
	    goto button_set;
	case MOUSE_7:
	    button_char = KE_MOUSERIGHT;
button_set:
	    {
		// Don't put events in the input queue now.
		if (hold_gui_events)
		    return;

		row = gui_xy2colrow(x, y, &col);
		// Don't report a mouse move unless moved to a
		// different character position.
		if (button == MOUSE_MOVE)
		{
		    if (row == prev_row && col == prev_col)
			return;
		    else
		    {
			prev_row = row >= 0 ? row : 0;
			prev_col = col;
		    }
		}

		string[3] = CSI;
		string[4] = KS_EXTRA;
		string[5] = (int)button_char;

		// Pass the pointer coordinates of the scroll event so that we
		// know which window to scroll.
		string[6] = (char_u)(col / 128 + ' ' + 1);
		string[7] = (char_u)(col % 128 + ' ' + 1);
		string[8] = (char_u)(row / 128 + ' ' + 1);
		string[9] = (char_u)(row % 128 + ' ' + 1);

		if (modifiers == 0)
		    add_to_input_buf(string + 3, 7);
		else
		{
		    string[0] = CSI;
		    string[1] = KS_MODIFIER;
		    string[2] = 0;
		    if (modifiers & MOUSE_SHIFT)
			string[2] |= MOD_MASK_SHIFT;
		    if (modifiers & MOUSE_CTRL)
			string[2] |= MOD_MASK_CTRL;
		    if (modifiers & MOUSE_ALT)
			string[2] |= MOD_MASK_ALT;
		    add_to_input_buf(string, 10);
		}
		return;
	    }
    }

#ifdef FEAT_CLIPBOARD
    // If a clipboard selection is in progress, handle it
    if (clip_star.state == SELECT_IN_PROGRESS)
    {
	clip_process_selection(button, X_2_COL(x), Y_2_ROW(y), repeated_click);

	// A release event may still need to be sent if the position is equal.
	row = gui_xy2colrow(x, y, &col);
	if (button != MOUSE_RELEASE || row != prev_row || col != prev_col)
	    return;
    }

    // Determine which mouse settings to look for based on the current mode
    switch (get_real_state())
    {
	case MODE_NORMAL_BUSY:
	case MODE_OP_PENDING:
# ifdef FEAT_TERMINAL
	case MODE_TERMINAL:
# endif
	case MODE_NORMAL:	checkfor = MOUSE_NORMAL; break;
	case MODE_VISUAL:	checkfor = MOUSE_VISUAL; break;
	case MODE_SELECT:	checkfor = MOUSE_VISUAL; break;
	case MODE_REPLACE:
	case MODE_REPLACE | MODE_LANGMAP:
	case MODE_VREPLACE:
	case MODE_VREPLACE | MODE_LANGMAP:
	case MODE_INSERT:
	case MODE_INSERT | MODE_LANGMAP:
				checkfor = MOUSE_INSERT; break;
	case MODE_ASKMORE:
	case MODE_HITRETURN:	// At the more- and hit-enter prompt pass the
				// mouse event for a click on or below the
				// message line.
				if (Y_2_ROW(y) >= msg_row)
				    checkfor = MOUSE_NORMAL;
				else
				    checkfor = MOUSE_RETURN;
				break;

	    /*
	     * On the command line, use the clipboard selection on all lines
	     * but the command line.  But not when pasting.
	     */
	case MODE_CMDLINE:
	case MODE_CMDLINE | MODE_LANGMAP:
	    if (Y_2_ROW(y) < cmdline_row && button != MOUSE_MIDDLE)
		checkfor = MOUSE_NONE;
	    else
		checkfor = MOUSE_COMMAND;
	    break;

	default:
	    checkfor = MOUSE_NONE;
	    break;
    };

    /*
     * Allow clipboard selection of text on the command line in "normal"
     * modes.  Don't do this when dragging the status line, or extending a
     * Visual selection.
     */
    if ((State == MODE_NORMAL || State == MODE_NORMAL_BUSY
						      || (State & MODE_INSERT))
	    && Y_2_ROW(y) >= topframe->fr_height + firstwin->w_winrow
	    && button != MOUSE_DRAG
# ifdef FEAT_MOUSESHAPE
	    && !drag_status_line
	    && !drag_sep_line
# endif
	    )
	checkfor = MOUSE_NONE;

    /*
     * Use modeless selection when holding CTRL and SHIFT pressed.
     */
    if ((modifiers & MOUSE_CTRL) && (modifiers & MOUSE_SHIFT))
	checkfor = MOUSE_NONEF;

    /*
     * In Ex mode, always use modeless selection.
     */
    if (exmode_active)
	checkfor = MOUSE_NONE;

    /*
     * If the mouse settings say to not use the mouse, use the modeless
     * selection.  But if Visual is active, assume that only the Visual area
     * will be selected.
     * Exception: On the command line, both the selection is used and a mouse
     * key is sent.
     */
    if (!mouse_has(checkfor) || checkfor == MOUSE_COMMAND)
    {
	// Don't do modeless selection in Visual mode.
	if (checkfor != MOUSE_NONEF && VIsual_active && (State & MODE_NORMAL))
	    return;

	/*
	 * When 'mousemodel' is "popup", shift-left is translated to right.
	 * But not when also using Ctrl.
	 */
	if (mouse_model_popup() && button == MOUSE_LEFT
		&& (modifiers & MOUSE_SHIFT) && !(modifiers & MOUSE_CTRL))
	{
	    button = MOUSE_RIGHT;
	    modifiers &= ~ MOUSE_SHIFT;
	}

	// If the selection is done, allow the right button to extend it.
	// If the selection is cleared, allow the right button to start it
	// from the cursor position.
	if (button == MOUSE_RIGHT)
	{
	    if (clip_star.state == SELECT_CLEARED)
	    {
		if (State & MODE_CMDLINE)
		{
		    col = msg_col;
		    row = msg_row;
		}
		else
		{
		    col = curwin->w_wcol;
		    row = curwin->w_wrow + W_WINROW(curwin);
		}
		clip_start_selection(col, row, FALSE);
	    }
	    clip_process_selection(button, X_2_COL(x), Y_2_ROW(y),
							      repeated_click);
	    did_clip = TRUE;
	}
	// Allow the left button to start the selection
	else if (button == MOUSE_LEFT)
	{
	    clip_start_selection(X_2_COL(x), Y_2_ROW(y), repeated_click);
	    did_clip = TRUE;
	}

	// Always allow pasting
	if (button != MOUSE_MIDDLE)
	{
	    if (!mouse_has(checkfor) || button == MOUSE_RELEASE)
		return;
	    if (checkfor != MOUSE_COMMAND)
		button = MOUSE_LEFT;
	}
	repeated_click = FALSE;
    }

    if (clip_star.state != SELECT_CLEARED && !did_clip)
	clip_clear_selection(&clip_star);
#endif

    // Don't put events in the input queue now.
    if (hold_gui_events)
	return;

    row = gui_xy2colrow(x, y, &col);

    /*
     * If we are dragging and the mouse hasn't moved far enough to be on a
     * different character, then don't send an event to vim.
     */
    if (button == MOUSE_DRAG)
    {
	if (row == prev_row && col == prev_col)
	    return;
	// Dragging above the window, set "row" to -1 to cause a scroll.
	if (y < 0)
	    row = -1;
    }

    /*
     * If topline has changed (window scrolled) since the last click, reset
     * repeated_click, because we don't want starting Visual mode when
     * clicking on a different character in the text.
     */
    if (curwin->w_topline != gui_prev_topline
#ifdef FEAT_DIFF
	    || curwin->w_topfill != gui_prev_topfill
#endif
	    )
	repeated_click = FALSE;

    string[0] = CSI;	// this sequence is recognized by check_termcode()
    string[1] = KS_MOUSE;
    string[2] = KE_FILLER;
    if (button != MOUSE_DRAG && button != MOUSE_RELEASE)
    {
	if (repeated_click)
	{
	    /*
	     * Handle multiple clicks.	They only count if the mouse is still
	     * pointing at the same character.
	     */
	    if (button != prev_button || row != prev_row || col != prev_col)
		num_clicks = 1;
	    else if (++num_clicks > 4)
		num_clicks = 1;
	}
	else
	    num_clicks = 1;
	prev_button = button;
	gui_prev_topline = curwin->w_topline;
#ifdef FEAT_DIFF
	gui_prev_topfill = curwin->w_topfill;
#endif

	string[3] = (char_u)(button | 0x20);
	SET_NUM_MOUSE_CLICKS(string[3], num_clicks);
    }
    else
	string[3] = (char_u)button;

    string[3] |= modifiers;
    fill_mouse_coord(string + 4, col, row);
    add_to_input_buf(string, 8);

    if (row < 0)
	prev_row = 0;
    else
	prev_row = row;
    prev_col = col;

    /*
     * We need to make sure this is cleared since GTK doesn't tell us when
     * the user is done dragging.
     */
#if defined(FEAT_GUI_GTK)
    gui.dragged_sb = SBAR_NONE;
#endif
}

/*
 * Convert x and y coordinate to column and row in text window.
 * Corrects for multi-byte character.
 * returns column in "*colp" and row as return value;
 */
    static int
gui_xy2colrow(int x, int y, int *colp)
{
    int		col = check_col(X_2_COL(x));
    int		row = check_row(Y_2_ROW(y));

    *colp = mb_fix_col(col, row);
    return row;
}

#if defined(FEAT_MENU) || defined(PROTO)
/*
 * Callback function for when a menu entry has been selected.
 */
    void
gui_menu_cb(vimmenu_T *menu)
{
    char_u  bytes[sizeof(long_u)];

    // Don't put events in the input queue now.
    if (hold_gui_events)
	return;

    bytes[0] = CSI;
    bytes[1] = KS_MENU;
    bytes[2] = KE_FILLER;
    add_to_input_buf(bytes, 3);
    add_long_to_buf((long_u)menu, bytes);
    add_to_input_buf_csi(bytes, sizeof(long_u));
}
#endif

static int	prev_which_scrollbars[3];

/*
 * Set which components are present.
 * If "oldval" is not NULL, "oldval" is the previous value, the new value is
 * in p_go.
 */
    void
gui_init_which_components(char_u *oldval UNUSED)
{
#ifdef FEAT_GUI_DARKTHEME
    static int	prev_dark_theme = -1;
    int		using_dark_theme = FALSE;
#endif
#ifdef FEAT_MENU
    static int	prev_menu_is_active = -1;
#endif
#ifdef FEAT_TOOLBAR
    static int	prev_toolbar = -1;
    int		using_toolbar = FALSE;
#endif
#ifdef FEAT_GUI_TABLINE
    int		using_tabline;
#endif
#if defined(FEAT_MENU)
    static int	prev_tearoff = -1;
    int		using_tearoff = FALSE;
#endif

    char_u	*p;
    int		i;
#ifdef FEAT_MENU
    int		grey_old, grey_new;
    char_u	*temp;
#endif
    win_T	*wp;
    int		need_set_size;
    int		fix_size;

#ifdef FEAT_MENU
    if (oldval != NULL && gui.in_use)
    {
	/*
	 * Check if the menus go from grey to non-grey or vice versa.
	 */
	grey_old = (vim_strchr(oldval, GO_GREY) != NULL);
	grey_new = (vim_strchr(p_go, GO_GREY) != NULL);
	if (grey_old != grey_new)
	{
	    temp = p_go;
	    p_go = oldval;
	    gui_update_menus(MENU_ALL_MODES);
	    p_go = temp;
	}
    }
    gui.menu_is_active = FALSE;
#endif

    for (i = 0; i < 3; i++)
	gui.which_scrollbars[i] = FALSE;
    for (p = p_go; *p; p++)
	switch (*p)
	{
	    case GO_LEFT:
		gui.which_scrollbars[SBAR_LEFT] = TRUE;
		break;
	    case GO_RIGHT:
		gui.which_scrollbars[SBAR_RIGHT] = TRUE;
		break;
	    case GO_VLEFT:
		if (win_hasvertsplit())
		    gui.which_scrollbars[SBAR_LEFT] = TRUE;
		break;
	    case GO_VRIGHT:
		if (win_hasvertsplit())
		    gui.which_scrollbars[SBAR_RIGHT] = TRUE;
		break;
	    case GO_BOT:
		gui.which_scrollbars[SBAR_BOTTOM] = TRUE;
		break;
#ifdef FEAT_GUI_DARKTHEME
	    case GO_DARKTHEME:
		using_dark_theme = TRUE;
		break;
#endif
#ifdef FEAT_MENU
	    case GO_MENUS:
		gui.menu_is_active = TRUE;
		break;
#endif
	    case GO_GREY:
		// make menu's have grey items, ignored here
		break;
#ifdef FEAT_TOOLBAR
	    case GO_TOOLBAR:
		using_toolbar = TRUE;
		break;
#endif
	    case GO_TEAROFF:
#if defined(FEAT_MENU)
		using_tearoff = TRUE;
#endif
		break;
	    default:
		// Ignore options that are not supported
		break;
	}

    if (!gui.in_use)
	return;

    need_set_size = 0;
    fix_size = FALSE;

#ifdef FEAT_GUI_DARKTHEME
    if (using_dark_theme != prev_dark_theme)
    {
	gui_mch_set_dark_theme(using_dark_theme);
	prev_dark_theme = using_dark_theme;
    }
#endif

#ifdef FEAT_GUI_TABLINE
    // Update the GUI tab line, it may appear or disappear.  This may
    // cause the non-GUI tab line to disappear or appear.
    using_tabline = gui_has_tabline();
    if (!gui_mch_showing_tabline() != !using_tabline)
    {
	// We don't want a resize event change "Rows" here, save and
	// restore it.  Resizing is handled below.
	i = Rows;
	gui_update_tabline();
	Rows = i;
	need_set_size |= RESIZE_VERT;
	if (using_tabline)
	    fix_size = TRUE;
	if (!gui_use_tabline())
	    redraw_tabline = TRUE;    // may draw non-GUI tab line
    }
#endif

    for (i = 0; i < 3; i++)
    {
	// The scrollbar needs to be updated when it is shown/unshown and
	// when switching tab pages.  But the size only changes when it's
	// shown/unshown.  Thus we need two places to remember whether a
	// scrollbar is there or not.
	if (gui.which_scrollbars[i] != prev_which_scrollbars[i]
		|| gui.which_scrollbars[i]
		!= curtab->tp_prev_which_scrollbars[i])
	{
	    if (i == SBAR_BOTTOM)
		gui_mch_enable_scrollbar(&gui.bottom_sbar,
			gui.which_scrollbars[i]);
	    else
	    {
		FOR_ALL_WINDOWS(wp)
		    gui_do_scrollbar(wp, i, gui.which_scrollbars[i]);
	    }
	    if (gui.which_scrollbars[i] != prev_which_scrollbars[i])
	    {
		if (i == SBAR_BOTTOM)
		    need_set_size |= RESIZE_VERT;
		else
		    need_set_size |= RESIZE_HOR;
		if (gui.which_scrollbars[i])
		    fix_size = TRUE;
	    }
	}
	curtab->tp_prev_which_scrollbars[i] = gui.which_scrollbars[i];
	prev_which_scrollbars[i] = gui.which_scrollbars[i];
    }

#ifdef FEAT_MENU
    if (gui.menu_is_active != prev_menu_is_active)
    {
	// We don't want a resize event change "Rows" here, save and
	// restore it.  Resizing is handled below.
	i = Rows;
	gui_mch_enable_menu(gui.menu_is_active);
	Rows = i;
	prev_menu_is_active = gui.menu_is_active;
	need_set_size |= RESIZE_VERT;
	if (gui.menu_is_active)
	    fix_size = TRUE;
    }
#endif

#ifdef FEAT_TOOLBAR
    if (using_toolbar != prev_toolbar)
    {
	gui_mch_show_toolbar(using_toolbar);
	prev_toolbar = using_toolbar;
	need_set_size |= RESIZE_VERT;
	if (using_toolbar)
	    fix_size = TRUE;
    }
#endif
#if defined(FEAT_MENU) && !(defined(MSWIN) && !defined(FEAT_TEAROFF))
    if (using_tearoff != prev_tearoff)
    {
	gui_mch_toggle_tearoffs(using_tearoff);
	prev_tearoff = using_tearoff;
    }
#endif
    if (need_set_size != 0)
    {
#ifdef FEAT_GUI_GTK
	long    prev_Columns = Columns;
	long    prev_Rows = Rows;
#endif
	// Adjust the size of the window to make the text area keep the
	// same size and to avoid that part of our window is off-screen
	// and a scrollbar can't be used, for example.
	gui_set_shellsize(FALSE, fix_size, need_set_size);

#ifdef FEAT_GUI_GTK
	// GTK has the annoying habit of sending us resize events when
	// changing the window size ourselves.  This mostly happens when
	// waiting for a character to arrive, quite unpredictably, and may
	// change Columns and Rows when we don't want it.  Wait for a
	// character here to avoid this effect.
	// If you remove this, please test this command for resizing
	// effects (with optional left scrollbar): ":vsp|q|vsp|q|vsp|q".
	// Don't do this while starting up though.
	// Don't change Rows when adding menu/toolbar/tabline.
	// Don't change Columns when adding vertical toolbar.
	if (!gui.starting && need_set_size != (RESIZE_VERT | RESIZE_HOR))
	    (void)char_avail();
	if ((need_set_size & RESIZE_VERT) == 0)
	    Rows = prev_Rows;
	if ((need_set_size & RESIZE_HOR) == 0)
	    Columns = prev_Columns;
#endif
    }
    // When the console tabline appears or disappears the window positions
    // change.
    if (firstwin->w_winrow != tabline_height())
	shell_new_rows();	// recompute window positions and heights
}

#if defined(FEAT_GUI_TABLINE) || defined(PROTO)
/*
 * Return TRUE if the GUI is taking care of the tabline.
 * It may still be hidden if 'showtabline' is zero.
 */
    int
gui_use_tabline(void)
{
    return gui.in_use && vim_strchr(p_go, GO_TABLINE) != NULL;
}

/*
 * Return TRUE if the GUI is showing the tabline.
 * This uses 'showtabline'.
 */
    static int
gui_has_tabline(void)
{
    if (!gui_use_tabline()
	    || p_stal == 0
	    || (p_stal == 1 && first_tabpage->tp_next == NULL))
	return FALSE;
    return TRUE;
}

/*
 * Update the tabline.
 * This may display/undisplay the tabline and update the labels.
 */
    void
gui_update_tabline(void)
{
    int	    showit = gui_has_tabline();
    int	    shown = gui_mch_showing_tabline();

    if (!gui.starting && starting == 0)
    {
	// Updating the tabline uses direct GUI commands, flush
	// outstanding instructions first. (esp. clear screen)
	out_flush();

	if (!showit != !shown)
	    gui_mch_show_tabline(showit);
	if (showit != 0)
	    gui_mch_update_tabline();

	// When the tabs change from hidden to shown or from shown to
	// hidden the size of the text area should remain the same.
	if (!showit != !shown)
	    gui_set_shellsize(FALSE, showit, RESIZE_VERT);
    }
}

/*
 * Get the label or tooltip for tab page "tp" into NameBuff[].
 */
    void
get_tabline_label(
    tabpage_T	*tp,
    int		tooltip)	// TRUE: get tooltip
{
    int		modified = FALSE;
    char_u	buf[40];
    int		wincount;
    win_T	*wp;
    char_u	**opt;

    // Use 'guitablabel' or 'guitabtooltip' if it's set.
    opt = (tooltip ? &p_gtt : &p_gtl);
    if (**opt != NUL)
    {
	char_u	res[MAXPATHL];
	tabpage_T *save_curtab;
	char_u	*opt_name = (char_u *)(tooltip ? "guitabtooltip"
							     : "guitablabel");

	printer_page_num = tabpage_index(tp);
# ifdef FEAT_EVAL
	set_vim_var_nr(VV_LNUM, printer_page_num);
# endif
	// It's almost as going to the tabpage, but without autocommands.
	curtab->tp_firstwin = firstwin;
	curtab->tp_lastwin = lastwin;
	curtab->tp_curwin = curwin;
	save_curtab = curtab;
	curtab = tp;
	topframe = curtab->tp_topframe;
	firstwin = curtab->tp_firstwin;
	lastwin = curtab->tp_lastwin;
	curwin = curtab->tp_curwin;
	curbuf = curwin->w_buffer;

	// Can't use NameBuff directly, build_stl_str_hl() uses it.
	build_stl_str_hl(curwin, res, MAXPATHL, *opt, opt_name, 0,
						 0, (int)Columns, NULL, NULL);
	STRCPY(NameBuff, res);

	// Back to the original curtab.
	curtab = save_curtab;
	topframe = curtab->tp_topframe;
	firstwin = curtab->tp_firstwin;
	lastwin = curtab->tp_lastwin;
	curwin = curtab->tp_curwin;
	curbuf = curwin->w_buffer;
    }

    // If 'guitablabel'/'guitabtooltip' is not set or the result is empty then
    // use a default label.
    if (**opt == NUL || *NameBuff == NUL)
    {
	// Get the buffer name into NameBuff[] and shorten it.
	get_trans_bufname(tp == curtab ? curbuf : tp->tp_curwin->w_buffer);
	if (!tooltip)
	    shorten_dir(NameBuff);

	wp = (tp == curtab) ? firstwin : tp->tp_firstwin;
	for (wincount = 0; wp != NULL; wp = wp->w_next, ++wincount)
	    if (bufIsChanged(wp->w_buffer))
		modified = TRUE;
	if (modified || wincount > 1)
	{
	    if (wincount > 1)
		vim_snprintf((char *)buf, sizeof(buf), "%d", wincount);
	    else
		buf[0] = NUL;
	    if (modified)
		STRCAT(buf, "+");
	    STRCAT(buf, " ");
	    STRMOVE(NameBuff + STRLEN(buf), NameBuff);
	    mch_memmove(NameBuff, buf, STRLEN(buf));
	}
    }
}

/*
 * Send the event for clicking to select tab page "nr".
 * Returns TRUE if it was done, FALSE when skipped because we are already at
 * that tab page or the cmdline window is open.
 */
    int
send_tabline_event(int nr)
{
    char_u string[3];

    if (nr == tabpage_index(curtab))
	return FALSE;

    // Don't put events in the input queue now.
    if (hold_gui_events || cmdwin_type != 0)
    {
	// Set it back to the current tab page.
	gui_mch_set_curtab(tabpage_index(curtab));
	return FALSE;
    }

    string[0] = CSI;
    string[1] = KS_TABLINE;
    string[2] = KE_FILLER;
    add_to_input_buf(string, 3);
    string[0] = nr;
    add_to_input_buf_csi(string, 1);
    return TRUE;
}

/*
 * Send a tabline menu event
 */
    void
send_tabline_menu_event(int tabidx, int event)
{
    char_u	    string[3];

    // Don't put events in the input queue now.
    if (hold_gui_events)
	return;

    // Cannot close the last tabpage.
    if (event == TABLINE_MENU_CLOSE && first_tabpage->tp_next == NULL)
	return;

    string[0] = CSI;
    string[1] = KS_TABMENU;
    string[2] = KE_FILLER;
    add_to_input_buf(string, 3);
    string[0] = tabidx;
    string[1] = (char_u)(long)event;
    add_to_input_buf_csi(string, 2);
}

#endif

/*
 * Scrollbar stuff:
 */

/*
 * Remove all scrollbars.  Used before switching to another tab page.
 */
    void
gui_remove_scrollbars(void)
{
    int	    i;
    win_T   *wp;

    for (i = 0; i < 3; i++)
    {
	if (i == SBAR_BOTTOM)
	    gui_mch_enable_scrollbar(&gui.bottom_sbar, FALSE);
	else
	{
	    FOR_ALL_WINDOWS(wp)
		gui_do_scrollbar(wp, i, FALSE);
	}
	curtab->tp_prev_which_scrollbars[i] = -1;
    }
}

    void
gui_create_scrollbar(scrollbar_T *sb, int type, win_T *wp)
{
    static int	sbar_ident = 0;

    sb->ident = sbar_ident++;	// No check for too big, but would it happen?
    sb->wp = wp;
    sb->type = type;
    sb->value = 0;
    sb->size = 1;
    sb->max = 1;
    sb->top = 0;
    sb->height = 0;
    sb->width = 0;
    sb->status_height = 0;
    gui_mch_create_scrollbar(sb, (wp == NULL) ? SBAR_HORIZ : SBAR_VERT);
}

/*
 * Find the scrollbar with the given index.
 */
    scrollbar_T *
gui_find_scrollbar(long ident)
{
    win_T	*wp;

    if (gui.bottom_sbar.ident == ident)
	return &gui.bottom_sbar;
    FOR_ALL_WINDOWS(wp)
    {
	if (wp->w_scrollbars[SBAR_LEFT].ident == ident)
	    return &wp->w_scrollbars[SBAR_LEFT];
	if (wp->w_scrollbars[SBAR_RIGHT].ident == ident)
	    return &wp->w_scrollbars[SBAR_RIGHT];
    }
    return NULL;
}

/*
 * For most systems: Put a code in the input buffer for a dragged scrollbar.
 *
 * For Win32, Macintosh and GTK+ 2:
 * Scrollbars seem to grab focus and vim doesn't read the input queue until
 * you stop dragging the scrollbar.  We get here each time the scrollbar is
 * dragged another pixel, but as far as the rest of vim goes, it thinks
 * we're just hanging in the call to DispatchMessage() in
 * process_message().  The DispatchMessage() call that hangs was passed a
 * mouse button click event in the scrollbar window. -- webb.
 *
 * Solution: Do the scrolling right here.  But only when allowed.
 * Ignore the scrollbars while executing an external command or when there
 * are still characters to be processed.
 */
    void
gui_drag_scrollbar(scrollbar_T *sb, long value, int still_dragging)
{
    win_T	*wp;
    int		sb_num;
#ifdef USE_ON_FLY_SCROLL
    colnr_T	old_leftcol = curwin->w_leftcol;
    linenr_T	old_topline = curwin->w_topline;
# ifdef FEAT_DIFF
    int		old_topfill = curwin->w_topfill;
# endif
#else
    char_u	bytes[sizeof(long_u)];
    int		byte_count;
#endif

    if (sb == NULL)
	return;

    // Don't put events in the input queue now.
    if (hold_gui_events)
	return;

    if (cmdwin_type != 0 && sb->wp != cmdwin_win)
	return;

    if (still_dragging)
    {
	if (sb->wp == NULL)
	    gui.dragged_sb = SBAR_BOTTOM;
	else if (sb == &sb->wp->w_scrollbars[SBAR_LEFT])
	    gui.dragged_sb = SBAR_LEFT;
	else
	    gui.dragged_sb = SBAR_RIGHT;
	gui.dragged_wp = sb->wp;
    }
    else
    {
	gui.dragged_sb = SBAR_NONE;
#ifdef FEAT_GUI_GTK
	// Keep the "dragged_wp" value until after the scrolling, for when the
	// mouse button is released.  GTK2 doesn't send the button-up event.
	gui.dragged_wp = NULL;
#endif
    }

    // Vertical sbar info is kept in the first sbar (the left one)
    if (sb->wp != NULL)
	sb = &sb->wp->w_scrollbars[0];

    /*
     * Check validity of value
     */
    if (value < 0)
	value = 0;
#ifdef SCROLL_PAST_END
    else if (value > sb->max)
	value = sb->max;
#else
    if (value > sb->max - sb->size + 1)
	value = sb->max - sb->size + 1;
#endif

    sb->value = value;

#ifdef USE_ON_FLY_SCROLL
    // When not allowed to do the scrolling right now, return.
    // This also checked input_available(), but that causes the first click in
    // a scrollbar to be ignored when Vim doesn't have focus.
    if (dont_scroll)
	return;
#endif
    // Disallow scrolling the current window when the completion popup menu is
    // visible.
    if ((sb->wp == NULL || sb->wp == curwin) && pum_visible())
	return;

#ifdef FEAT_RIGHTLEFT
    if (sb->wp == NULL && curwin->w_p_rl)
    {
	value = sb->max + 1 - sb->size - value;
	if (value < 0)
	    value = 0;
    }
#endif

    if (sb->wp != NULL)		// vertical scrollbar
    {
	sb_num = 0;
	for (wp = firstwin; wp != sb->wp && wp != NULL; wp = wp->w_next)
	    sb_num++;
	if (wp == NULL)
	    return;

#ifdef USE_ON_FLY_SCROLL
	current_scrollbar = sb_num;
	scrollbar_value = value;
	if (State & MODE_NORMAL)
	{
	    gui_do_scroll();
	    setcursor();
	}
	else if (State & MODE_INSERT)
	{
	    ins_scroll();
	    setcursor();
	}
	else if (State & MODE_CMDLINE)
	{
	    if (msg_scrolled == 0)
	    {
		gui_do_scroll();
		redrawcmdline();
	    }
	}
# ifdef FEAT_FOLDING
	// Value may have been changed for closed fold.
	sb->value = sb->wp->w_topline - 1;
# endif

	// When dragging one scrollbar and there is another one at the other
	// side move the thumb of that one too.
	if (gui.which_scrollbars[SBAR_RIGHT] && gui.which_scrollbars[SBAR_LEFT])
	    gui_mch_set_scrollbar_thumb(
		    &sb->wp->w_scrollbars[
			    sb == &sb->wp->w_scrollbars[SBAR_RIGHT]
						    ? SBAR_LEFT : SBAR_RIGHT],
		    sb->value, sb->size, sb->max);

#else
	bytes[0] = CSI;
	bytes[1] = KS_VER_SCROLLBAR;
	bytes[2] = KE_FILLER;
	bytes[3] = (char_u)sb_num;
	byte_count = 4;
#endif
    }
    else
    {
#ifdef USE_ON_FLY_SCROLL
	scrollbar_value = value;

	if (State & MODE_NORMAL)
	    do_mousescroll_horiz(scrollbar_value);
	else if (State & MODE_INSERT)
	    ins_horscroll();
	else if (State & MODE_CMDLINE)
	{
	    if (msg_scrolled == 0)
	    {
		do_mousescroll_horiz(scrollbar_value);
		redrawcmdline();
	    }
	}
	if (old_leftcol != curwin->w_leftcol)
	{
	    updateWindow(curwin);   // update window, status and cmdline
	    setcursor();
	}
#else
	bytes[0] = CSI;
	bytes[1] = KS_HOR_SCROLLBAR;
	bytes[2] = KE_FILLER;
	byte_count = 3;
#endif
    }

#ifdef USE_ON_FLY_SCROLL
    /*
     * synchronize other windows, as necessary according to 'scrollbind'
     */
    if (curwin->w_p_scb
	    && ((sb->wp == NULL && curwin->w_leftcol != old_leftcol)
		|| (sb->wp == curwin && (curwin->w_topline != old_topline
# ifdef FEAT_DIFF
					   || curwin->w_topfill != old_topfill
# endif
			))))
    {
	do_check_scrollbind(TRUE);
	// need to update the window right here
	FOR_ALL_WINDOWS(wp)
	    if (wp->w_redr_type > 0)
		updateWindow(wp);
	setcursor();
    }
    out_flush_cursor(FALSE, TRUE);
#else
    add_to_input_buf(bytes, byte_count);
    add_long_to_buf((long_u)value, bytes);
    add_to_input_buf_csi(bytes, sizeof(long_u));
#endif
}

/*
 * Scrollbar stuff:
 */

/*
 * Called when something in the window layout has changed.
 */
    void
gui_may_update_scrollbars(void)
{
    if (gui.in_use && starting == 0)
    {
	out_flush();
	gui_init_which_components(NULL);
	gui_update_scrollbars(TRUE);
    }
    need_mouse_correct = TRUE;
}

    void
gui_update_scrollbars(
    int		force)	    // Force all scrollbars to get updated
{
    win_T	*wp;
    scrollbar_T	*sb;
    long	val, size, max;		// need 32 bits here
    int		which_sb;
    int		h, y;
    static win_T *prev_curwin = NULL;

    // Update the horizontal scrollbar
    gui_update_horiz_scrollbar(force);

#ifndef MSWIN
    // Return straight away if there is neither a left nor right scrollbar.
    // On MS-Windows this is required anyway for scrollwheel messages.
    if (!gui.which_scrollbars[SBAR_LEFT] && !gui.which_scrollbars[SBAR_RIGHT])
	return;
#endif

    /*
     * Don't want to update a scrollbar while we're dragging it.  But if we
     * have both a left and right scrollbar, and we drag one of them, we still
     * need to update the other one.
     */
    if (!force && (gui.dragged_sb == SBAR_LEFT || gui.dragged_sb == SBAR_RIGHT)
	    && gui.which_scrollbars[SBAR_LEFT]
	    && gui.which_scrollbars[SBAR_RIGHT])
    {
	/*
	 * If we have two scrollbars and one of them is being dragged, just
	 * copy the scrollbar position from the dragged one to the other one.
	 */
	which_sb = SBAR_LEFT + SBAR_RIGHT - gui.dragged_sb;
	if (gui.dragged_wp != NULL)
	    gui_mch_set_scrollbar_thumb(
		    &gui.dragged_wp->w_scrollbars[which_sb],
		    gui.dragged_wp->w_scrollbars[0].value,
		    gui.dragged_wp->w_scrollbars[0].size,
		    gui.dragged_wp->w_scrollbars[0].max);
    }

    // avoid that moving components around generates events
    ++hold_gui_events;

    FOR_ALL_WINDOWS(wp)
    {
	if (wp->w_buffer == NULL)	// just in case
	    continue;
	// Skip a scrollbar that is being dragged.
	if (!force && (gui.dragged_sb == SBAR_LEFT
					     || gui.dragged_sb == SBAR_RIGHT)
		&& gui.dragged_wp == wp)
	    continue;

#ifdef SCROLL_PAST_END
	max = wp->w_buffer->b_ml.ml_line_count - 1;
#else
	max = wp->w_buffer->b_ml.ml_line_count + wp->w_height - 2;
#endif
	if (max < 0)			// empty buffer
	    max = 0;
	val = wp->w_topline - 1;
	size = wp->w_height;
#ifdef SCROLL_PAST_END
	if (val > max)			// just in case
	    val = max;
#else
	if (size > max + 1)		// just in case
	    size = max + 1;
	if (val > max - size + 1)
	    val = max - size + 1;
#endif
	if (val < 0)			// minimal value is 0
	    val = 0;

	/*
	 * Scrollbar at index 0 (the left one) contains all the information.
	 * It would be the same info for left and right so we just store it for
	 * one of them.
	 */
	sb = &wp->w_scrollbars[0];

	/*
	 * Note: no check for valid w_botline.	If it's not valid the
	 * scrollbars will be updated later anyway.
	 */
	if (size < 1 || wp->w_botline - 2 > max)
	{
	    /*
	     * This can happen during changing files.  Just don't update the
	     * scrollbar for now.
	     */
	    sb->height = 0;	    // Force update next time
	    if (gui.which_scrollbars[SBAR_LEFT])
		gui_do_scrollbar(wp, SBAR_LEFT, FALSE);
	    if (gui.which_scrollbars[SBAR_RIGHT])
		gui_do_scrollbar(wp, SBAR_RIGHT, FALSE);
	    continue;
	}
	if (force || sb->height != wp->w_height
	    || sb->top != wp->w_winrow
	    || sb->status_height != wp->w_status_height
	    || sb->width != wp->w_width
	    || prev_curwin != curwin)
	{
	    // Height, width or position of scrollbar has changed.  For
	    // vertical split: curwin changed.
	    sb->height = wp->w_height;
	    sb->top = wp->w_winrow;
	    sb->status_height = wp->w_status_height;
	    sb->width = wp->w_width;

	    // Calculate height and position in pixels
	    h = (sb->height + sb->status_height) * gui.char_height;
	    y = sb->top * gui.char_height + gui.border_offset;
#if defined(FEAT_MENU) && !defined(FEAT_GUI_GTK) && !defined(FEAT_GUI_MOTIF) && !defined(FEAT_GUI_PHOTON)
	    if (gui.menu_is_active)
		y += gui.menu_height;
#endif

#if defined(FEAT_TOOLBAR) && (defined(FEAT_GUI_MSWIN) \
	|| defined(FEAT_GUI_HAIKU))
	    if (vim_strchr(p_go, GO_TOOLBAR) != NULL)
		y += gui.toolbar_height;
#endif

#if defined(FEAT_GUI_TABLINE) && defined(FEAT_GUI_MSWIN) || defined(FEAT_GUI_HAIKU)
	    if (gui_has_tabline())
		y += gui.tabline_height;
#endif

	    if (wp->w_winrow == 0)
	    {
		// Height of top scrollbar includes width of top border
		h += gui.border_offset;
		y -= gui.border_offset;
	    }
	    if (gui.which_scrollbars[SBAR_LEFT])
	    {
		gui_mch_set_scrollbar_pos(&wp->w_scrollbars[SBAR_LEFT],
					  gui.left_sbar_x, y,
					  gui.scrollbar_width, h);
		gui_do_scrollbar(wp, SBAR_LEFT, TRUE);
	    }
	    if (gui.which_scrollbars[SBAR_RIGHT])
	    {
		gui_mch_set_scrollbar_pos(&wp->w_scrollbars[SBAR_RIGHT],
					  gui.right_sbar_x, y,
					  gui.scrollbar_width, h);
		gui_do_scrollbar(wp, SBAR_RIGHT, TRUE);
	    }
	}

	if (force || sb->value != val || sb->size != size || sb->max != max)
	{
	    // Thumb of scrollbar has moved
	    sb->value = val;
	    sb->size = size;
	    sb->max = max;
	    if (gui.which_scrollbars[SBAR_LEFT]
		    && (gui.dragged_sb != SBAR_LEFT || gui.dragged_wp != wp))
		gui_mch_set_scrollbar_thumb(&wp->w_scrollbars[SBAR_LEFT],
					    val, size, max);
	    if (gui.which_scrollbars[SBAR_RIGHT]
		    && (gui.dragged_sb != SBAR_RIGHT || gui.dragged_wp != wp))
		gui_mch_set_scrollbar_thumb(&wp->w_scrollbars[SBAR_RIGHT],
					    val, size, max);
	}
    }

    // update the title, it may show the scroll position
    maketitle();

    prev_curwin = curwin;
    --hold_gui_events;
}

/*
 * Enable or disable a scrollbar.
 * Check for scrollbars for vertically split windows which are not enabled
 * sometimes.
 */
    static void
gui_do_scrollbar(
    win_T	*wp,
    int		which,	    // SBAR_LEFT or SBAR_RIGHT
    int		enable)	    // TRUE to enable scrollbar
{
    int		midcol = curwin->w_wincol + curwin->w_width / 2;
    int		has_midcol = (wp->w_wincol <= midcol
				     && wp->w_wincol + wp->w_width >= midcol);

    // Only enable scrollbars that contain the middle column of the current
    // window.
    if (gui.which_scrollbars[SBAR_RIGHT] != gui.which_scrollbars[SBAR_LEFT])
    {
	// Scrollbars only on one side.  Don't enable scrollbars that don't
	// contain the middle column of the current window.
	if (!has_midcol)
	    enable = FALSE;
    }
    else
    {
	// Scrollbars on both sides.  Don't enable scrollbars that neither
	// contain the middle column of the current window nor are on the far
	// side.
	if (midcol > Columns / 2)
	{
	    if (which == SBAR_LEFT ? wp->w_wincol != 0 : !has_midcol)
		enable = FALSE;
	}
	else
	{
	    if (which == SBAR_RIGHT ? wp->w_wincol + wp->w_width != Columns
								: !has_midcol)
		enable = FALSE;
	}
    }
    gui_mch_enable_scrollbar(&wp->w_scrollbars[which], enable);
}

/*
 * Scroll a window according to the values set in the globals
 * "current_scrollbar" and "scrollbar_value".
 * Return TRUE if the cursor in the current window moved or FALSE otherwise.
 * may eventually cause a redraw using updateWindow
 */
    int
gui_do_scroll(void)
{
    win_T	*wp, *save_wp;
    int		i;
    long	nlines;
    pos_T	old_cursor;
    linenr_T	old_topline;
#ifdef FEAT_DIFF
    int		old_topfill;
#endif

    for (wp = firstwin, i = 0; i < current_scrollbar; wp = W_NEXT(wp), i++)
	if (wp == NULL)
	    break;
    if (wp == NULL)
	// Couldn't find window
	return FALSE;
    // don't redraw, LineOffset and similar are not valid!
    if (exmode_active)
	return FALSE;

    /*
     * Compute number of lines to scroll.  If zero, nothing to do.
     */
    nlines = (long)scrollbar_value + 1 - (long)wp->w_topline;
    if (nlines == 0)
	return FALSE;

    save_wp = curwin;
    old_topline = wp->w_topline;
#ifdef FEAT_DIFF
    old_topfill = wp->w_topfill;
#endif
    old_cursor = wp->w_cursor;
    curwin = wp;
    curbuf = wp->w_buffer;
    if (nlines < 0)
	scrolldown(-nlines, gui.dragged_wp == NULL);
    else
	scrollup(nlines, gui.dragged_wp == NULL);
    // Reset dragged_wp after using it.  "dragged_sb" will have been reset for
    // the mouse-up event already, but we still want it to behave like when
    // dragging.  But not the next click in an arrow.
    if (gui.dragged_sb == SBAR_NONE)
	gui.dragged_wp = NULL;

    if (old_topline != wp->w_topline
#ifdef FEAT_DIFF
	    || old_topfill != wp->w_topfill
#endif
	    )
    {
	if (get_scrolloff_value() != 0)
	{
	    cursor_correct();		// fix window for 'so'
	    update_topline();		// avoid up/down jump
	}
	if (old_cursor.lnum != wp->w_cursor.lnum)
	    coladvance(wp->w_curswant);
	wp->w_scbind_pos = wp->w_topline;
    }

    // Make sure wp->w_leftcol and wp->w_skipcol are correct.
    validate_cursor();

    curwin = save_wp;
    curbuf = save_wp->w_buffer;

    /*
     * Don't call updateWindow() when nothing has changed (it will overwrite
     * the status line!).
     */
    if (old_topline != wp->w_topline
	    || wp->w_redr_type != 0
#ifdef FEAT_DIFF
	    || old_topfill != wp->w_topfill
#endif
	    )
    {
	int type = UPD_VALID;

	if (pum_visible())
	{
	    type = UPD_NOT_VALID;
	    wp->w_lines_valid = 0;
	}

	// Don't set must_redraw here, it may cause the popup menu to
	// disappear when losing focus after a scrollbar drag.
	if (wp->w_redr_type < type)
	    wp->w_redr_type = type;
	mch_disable_flush();
	updateWindow(wp);   // update window, status line, and cmdline
	mch_enable_flush();
    }

    // May need to redraw the popup menu.
    if (pum_visible())
	pum_redraw();

    return (wp == curwin && !EQUAL_POS(curwin->w_cursor, old_cursor));
}

/*
 * Horizontal scrollbar stuff:
 */
    static void
gui_update_horiz_scrollbar(int force)
{
    long	value, size, max;

    if (!gui.which_scrollbars[SBAR_BOTTOM])
	return;

    if (!force && gui.dragged_sb == SBAR_BOTTOM)
	return;

    if (!force && curwin->w_p_wrap && gui.prev_wrap)
	return;

    /*
     * It is possible for the cursor to be invalid if we're in the middle of
     * something (like changing files).  If so, don't do anything for now.
     */
    if (curwin->w_cursor.lnum > curbuf->b_ml.ml_line_count)
    {
	gui.bottom_sbar.value = -1;
	return;
    }

    size = curwin->w_width;
    if (curwin->w_p_wrap)
    {
	value = 0;
#ifdef SCROLL_PAST_END
	max = 0;
#else
	max = curwin->w_width - 1;
#endif
    }
    else
    {
	value = curwin->w_leftcol;
	max = scroll_line_len(ui_find_longest_lnum());

	if (virtual_active())
	{
	    // May move the cursor even further to the right.
	    if (curwin->w_virtcol >= (colnr_T)max)
		max = curwin->w_virtcol;
	}

#ifndef SCROLL_PAST_END
	max += curwin->w_width - 1;
#endif
	// The line number isn't scrolled, thus there is less space when
	// 'number' or 'relativenumber' is set (also for 'foldcolumn').
	size -= curwin_col_off();
#ifndef SCROLL_PAST_END
	max -= curwin_col_off();
#endif
    }

#ifndef SCROLL_PAST_END
    if (value > max - size + 1)
	value = max - size + 1;	    // limit the value to allowable range
#endif

#ifdef FEAT_RIGHTLEFT
    if (curwin->w_p_rl)
    {
	value = max + 1 - size - value;
	if (value < 0)
	{
	    size += value;
	    value = 0;
	}
    }
#endif
    if (!force && value == gui.bottom_sbar.value && size == gui.bottom_sbar.size
						&& max == gui.bottom_sbar.max)
	return;

    gui.bottom_sbar.value = value;
    gui.bottom_sbar.size = size;
    gui.bottom_sbar.max = max;
    gui.prev_wrap = curwin->w_p_wrap;

    gui_mch_set_scrollbar_thumb(&gui.bottom_sbar, value, size, max);
}

/*
 * Check that none of the colors are the same as the background color
 */
    void
gui_check_colors(void)
{
    if (gui.norm_pixel == gui.back_pixel || gui.norm_pixel == INVALCOLOR)
    {
	gui_set_bg_color((char_u *)"White");
	if (gui.norm_pixel == gui.back_pixel || gui.norm_pixel == INVALCOLOR)
	    gui_set_fg_color((char_u *)"Black");
    }
}

    static void
gui_set_fg_color(char_u *name)
{
    gui.norm_pixel = gui_get_color(name);
    hl_set_fg_color_name(vim_strsave(name));
}

    static void
gui_set_bg_color(char_u *name)
{
    gui.back_pixel = gui_get_color(name);
    hl_set_bg_color_name(vim_strsave(name));
}

/*
 * Allocate a color by name.
 * Returns INVALCOLOR and gives an error message when failed.
 */
    guicolor_T
gui_get_color(char_u *name)
{
    guicolor_T	t;

    if (*name == NUL)
	return INVALCOLOR;
    t = gui_mch_get_color(name);

    int is_none = STRCMP(name, "none") == 0;
    if (t == INVALCOLOR
#if defined(FEAT_GUI_X11) || defined(FEAT_GUI_GTK)
	    && (gui.in_use || is_none)
#endif
	    )
    {
	if (is_none)
	    emsg(_(e_cannot_use_color_none_did_you_mean_none));
	else
	    semsg(_(e_cannot_allocate_color_str), name);
    }
    return t;
}

/*
 * Return the grey value of a color (range 0-255).
 */
    int
gui_get_lightness(guicolor_T pixel)
{
    long_u	rgb = (long_u)gui_mch_get_rgb(pixel);

    return  (int)(  (((rgb >> 16) & 0xff) * 299)
		   + (((rgb >> 8) & 0xff) * 587)
		   +  ((rgb	  & 0xff) * 114)) / 1000;
}

    char_u *
gui_bg_default(void)
{
    if (gui_get_lightness(gui.back_pixel) < 127)
	return (char_u *)"dark";
    return (char_u *)"light";
}

/*
 * Option initializations that can only be done after opening the GUI window.
 */
    static void
init_gui_options(void)
{
    // Set the 'background' option according to the lightness of the
    // background color, unless the user has set it already.
    if (!option_was_set((char_u *)"bg") && STRCMP(p_bg, gui_bg_default()) != 0)
    {
	set_option_value_give_err((char_u *)"bg", 0L, gui_bg_default(), 0);
	highlight_changed();
    }
}

#if defined(FEAT_GUI_X11) || defined(PROTO)
    void
gui_new_scrollbar_colors(void)
{
    win_T	*wp;

    // Nothing to do if GUI hasn't started yet.
    if (!gui.in_use)
	return;

    FOR_ALL_WINDOWS(wp)
    {
	gui_mch_set_scrollbar_colors(&(wp->w_scrollbars[SBAR_LEFT]));
	gui_mch_set_scrollbar_colors(&(wp->w_scrollbars[SBAR_RIGHT]));
    }
    gui_mch_set_scrollbar_colors(&gui.bottom_sbar);
}
#endif

/*
 * Call this when focus has changed.
 */
    void
gui_focus_change(int in_focus)
{
/*
 * Skip this code to avoid drawing the cursor when debugging and switching
 * between the debugger window and gvim.
 */
#if 1
    gui.in_focus = in_focus;
    out_flush_cursor(TRUE, FALSE);

# ifdef FEAT_XIM
    xim_set_focus(in_focus);
# endif

    // Put events in the input queue only when allowed.
    // ui_focus_change() isn't called directly, because it invokes
    // autocommands and that must not happen asynchronously.
    if (!hold_gui_events)
    {
	char_u  bytes[3];

	bytes[0] = CSI;
	bytes[1] = KS_EXTRA;
	bytes[2] = in_focus ? (int)KE_FOCUSGAINED : (int)KE_FOCUSLOST;
	add_to_input_buf(bytes, 3);
    }
#endif
}

/*
 * When mouse moved: apply 'mousefocus'.
 * Also updates the mouse pointer shape.
 */
    static void
gui_mouse_focus(int x, int y)
{
    win_T	*wp;
    char_u	st[8];

#ifdef FEAT_MOUSESHAPE
    // Get window pointer, and update mouse shape as well.
    wp = xy2win(x, y, IGNORE_POPUP);
#endif

    // Only handle this when 'mousefocus' set and ...
    if (p_mousef
	    && !hold_gui_events		// not holding events
	    && (State & (MODE_NORMAL | MODE_INSERT))// Normal/Visual/Insert mode
	    && State != MODE_HITRETURN	// but not hit-return prompt
	    && msg_scrolled == 0	// no scrolled message
	    && !need_mouse_correct	// not moving the pointer
	    && gui.in_focus)		// gvim in focus
    {
	// Don't move the mouse when it's left or right of the Vim window
	if (x < 0 || x > Columns * gui.char_width)
	    return;
#ifndef FEAT_MOUSESHAPE
	wp = xy2win(x, y, IGNORE_POPUP);
#endif
	if (wp == curwin || wp == NULL)
	    return;	// still in the same old window, or none at all

	// Ignore position in the tab pages line.
	if (Y_2_ROW(y) < tabline_height())
	    return;

	/*
	 * Format a mouse click on status line input,
	 * ala gui_send_mouse_event(0, x, y, 0, 0);
	 * Trick: Use a column number -1, so that get_pseudo_mouse_code() will
	 * generate a K_LEFTMOUSE_NM key code.
	 */
	if (finish_op)
	{
	    // abort the current operator first
	    st[0] = ESC;
	    add_to_input_buf(st, 1);
	}
	st[0] = CSI;
	st[1] = KS_MOUSE;
	st[2] = KE_FILLER;
	st[3] = (char_u)MOUSE_LEFT;
	fill_mouse_coord(st + 4,
		wp->w_wincol == 0 ? -1 : wp->w_wincol + MOUSE_COLOFF,
		wp->w_height + W_WINROW(wp));

	add_to_input_buf(st, 8);
	st[3] = (char_u)MOUSE_RELEASE;
	add_to_input_buf(st, 8);
#ifdef FEAT_GUI_GTK
	// Need to wake up the main loop
	if (gtk_main_level() > 0)
	    gtk_main_quit();
#endif
    }
}

/*
 * Called when the mouse moved (but not when dragging).
 */
    void
gui_mouse_moved(int x, int y)
{
    // Ignore this while still starting up.
    if (!gui.in_use || gui.starting)
	return;

    // apply 'mousefocus' and pointer shape
    gui_mouse_focus(x, y);

    if (p_mousemev
#ifdef FEAT_PROP_POPUP
	|| popup_uses_mouse_move
#endif
   )
	// Generate a mouse-moved event. For a <MouseMove> mapping. Or so the
	// popup can perhaps be closed, just like in the terminal.
	gui_send_mouse_event(MOUSE_MOVE, x, y, FALSE, 0);
}

/*
 * Get the window where the mouse pointer is on.
 * Returns NULL if not found.
 */
    win_T *
gui_mouse_window(mouse_find_T popup)
{
    int		x, y;

    if (!(gui.in_use && (p_mousef || popup == FIND_POPUP)))
	return NULL;
    gui_mch_getmouse(&x, &y);

    // Only use the mouse when it's on the Vim window
    if (x >= 0 && x <= Columns * gui.char_width
	    && y >= 0 && Y_2_ROW(y) >= tabline_height())
	return xy2win(x, y, popup);
    return NULL;
}

/*
 * Called when mouse should be moved to window with focus.
 */
    void
gui_mouse_correct(void)
{
    win_T	*wp = NULL;

    need_mouse_correct = FALSE;

    wp = gui_mouse_window(IGNORE_POPUP);
    if (wp == curwin || wp == NULL)
	return;

    // If in other than current window
    validate_cline_row();
    gui_mch_setmouse((int)W_ENDCOL(curwin) * gui.char_width - 3,
	    (W_WINROW(curwin) + curwin->w_wrow) * gui.char_height
	    + (gui.char_height) / 2);
}

/*
 * Find window where the mouse pointer "x" / "y" coordinate is in.
 * As a side effect update the shape of the mouse pointer.
 */
    static win_T *
xy2win(int x, int y, mouse_find_T popup)
{
    int		row;
    int		col;
    win_T	*wp;

    row = Y_2_ROW(y);
    col = X_2_COL(x);
    if (row < 0 || col < 0)		// before first window
	return NULL;
    wp = mouse_find_win(&row, &col, popup);
    if (wp == NULL)
	return NULL;
#ifdef FEAT_MOUSESHAPE
    if (State == MODE_HITRETURN || State == MODE_ASKMORE)
    {
	if (Y_2_ROW(y) >= msg_row)
	    update_mouseshape(SHAPE_IDX_MOREL);
	else
	    update_mouseshape(SHAPE_IDX_MORE);
    }
    else if (row > wp->w_height)	// below status line
	update_mouseshape(SHAPE_IDX_CLINE);
    else if (!(State & MODE_CMDLINE) && wp->w_vsep_width > 0 && col == wp->w_width
	    && (row != wp->w_height || !stl_connected(wp)) && msg_scrolled == 0)
	update_mouseshape(SHAPE_IDX_VSEP);
    else if (!(State & MODE_CMDLINE) && wp->w_status_height > 0
				  && row == wp->w_height && msg_scrolled == 0)
	update_mouseshape(SHAPE_IDX_STATUS);
    else
	update_mouseshape(-2);
#endif
    return wp;
}

/*
 * ":gui" and ":gvim": Change from the terminal version to the GUI version.
 * File names may be given to redefine the args list.
 */
    void
ex_gui(exarg_T *eap)
{
    char_u	*arg = eap->arg;

    /*
     * Check for "-f" argument: foreground, don't fork.
     * Also don't fork when started with "gvim -f".
     * Do fork when using "gui -b".
     */
    if (arg[0] == '-'
	    && (arg[1] == 'f' || arg[1] == 'b')
	    && (arg[2] == NUL || VIM_ISWHITE(arg[2])))
    {
	gui.dofork = (arg[1] == 'b');
	eap->arg = skipwhite(eap->arg + 2);
    }
    if (!gui.in_use)
    {
#if defined(VIMDLL) && !defined(EXPERIMENTAL_GUI_CMD)
	if (!gui.starting)
	{
	    emsg(_(e_gui_cannot_be_used_not_enabled_at_compile_time));
	    return;
	}
#endif
	// Clear the command.  Needed for when forking+exiting, to avoid part
	// of the argument ending up after the shell prompt.
	msg_clr_eos_force();
#ifdef GUI_MAY_SPAWN
	if (!ends_excmd2(eap->cmd, eap->arg))
	    gui_start(eap->arg);
	else
#endif
	    gui_start(NULL);
#ifdef FEAT_JOB_CHANNEL
	channel_gui_register_all();
#endif
    }
    if (!ends_excmd2(eap->cmd, eap->arg))
	ex_next(eap);
}

#if ((defined(FEAT_GUI_X11) || defined(FEAT_GUI_GTK) \
	    || defined(FEAT_GUI_MSWIN) || defined(FEAT_GUI_PHOTON) \
	    || defined(FEAT_GUI_HAIKU)) \
	    && defined(FEAT_TOOLBAR)) || defined(PROTO)
/*
 * This is shared between Haiku, Motif, and GTK.
 */

/*
 * Callback function for do_in_runtimepath().
 */
    static void
gfp_setname(char_u *fname, void *cookie)
{
    char_u	*gfp_buffer = cookie;

    if (STRLEN(fname) >= MAXPATHL)
	*gfp_buffer = NUL;
    else
	STRCPY(gfp_buffer, fname);
}

/*
 * Find the path of bitmap "name" with extension "ext" in 'runtimepath'.
 * Return FAIL for failure and OK if buffer[MAXPATHL] contains the result.
 */
    int
gui_find_bitmap(char_u *name, char_u *buffer, char *ext)
{
    if (STRLEN(name) > MAXPATHL - 14)
	return FAIL;
    vim_snprintf((char *)buffer, MAXPATHL, "bitmaps/%s.%s", name, ext);
    if (do_in_runtimepath(buffer, 0, gfp_setname, buffer) == FAIL
							    || *buffer == NUL)
	return FAIL;
    return OK;
}

# if !defined(FEAT_GUI_GTK) || defined(PROTO)
/*
 * Given the name of the "icon=" argument, try finding the bitmap file for the
 * icon.  If it is an absolute path name, use it as it is.  Otherwise append
 * "ext" and search for it in 'runtimepath'.
 * The result is put in "buffer[MAXPATHL]".  If something fails "buffer"
 * contains "name".
 */
    void
gui_find_iconfile(char_u *name, char_u *buffer, char *ext)
{
    char_u	buf[MAXPATHL + 1];

    expand_env(name, buffer, MAXPATHL);
    if (!mch_isFullName(buffer) && gui_find_bitmap(buffer, buf, ext) == OK)
	STRCPY(buffer, buf);
}
# endif
#endif

#if defined(FEAT_GUI_GTK) || defined(FEAT_GUI_X11)|| defined(FEAT_GUI_HAIKU) \
	|| defined(PROTO)
    void
display_errors(void)
{
    char_u	*p;

    if (isatty(2))
    {
	fflush(stderr);
	return;
    }

    if (error_ga.ga_data == NULL)
	return;

    // avoid putting up a message box with blanks only
    for (p = (char_u *)error_ga.ga_data; *p != NUL; ++p)
	if (!SAFE_isspace(*p))
	{
	    // Truncate a very long message, it will go off-screen.
	    if (STRLEN(p) > 2000)
		STRCPY(p + 2000 - 14, "...(truncated)");
	    (void)do_dialog(VIM_ERROR, (char_u *)_("Error"),
		    p, (char_u *)_("&Ok"), 1, NULL, FALSE);
	    break;
	}
    ga_clear(&error_ga);
}
#endif

#if defined(NO_CONSOLE_INPUT) || defined(PROTO)
/*
 * Return TRUE if still starting up and there is no place to enter text.
 * For GTK and X11 we check if stderr is not a tty, which means we were
 * (probably) started from the desktop.  Also check stdin, "vim >& file" does
 * allow typing on stdin.
 */
    int
no_console_input(void)
{
    return ((!gui.in_use || gui.starting)
# ifndef NO_CONSOLE
	    && !isatty(0) && !isatty(2)
# endif
	    );
}
#endif

#if defined(FIND_REPLACE_DIALOG) \
	|| defined(NEED_GUI_UPDATE_SCREEN) \
	|| defined(PROTO)
/*
 * Update the current window and the screen.
 */
    void
gui_update_screen(void)
{
# ifdef FEAT_CONCEAL
    linenr_T	conceal_old_cursor_line = 0;
    linenr_T	conceal_new_cursor_line = 0;
    int		conceal_update_lines = FALSE;
# endif

    update_topline();
    validate_cursor();

    // Trigger CursorMoved if the cursor moved.
    if (!finish_op && (has_cursormoved()
# ifdef FEAT_PROP_POPUP
		|| popup_visible
# endif
# ifdef FEAT_CONCEAL
		|| curwin->w_p_cole > 0
# endif
		) && !EQUAL_POS(last_cursormoved, curwin->w_cursor))
    {
	if (has_cursormoved())
	    apply_autocmds(EVENT_CURSORMOVED, NULL, NULL, FALSE, curbuf);
# ifdef FEAT_PROP_POPUP
	if (popup_visible)
	    popup_check_cursor_pos();
# endif
# ifdef FEAT_CONCEAL
	if (curwin->w_p_cole > 0)
	{
	    conceal_old_cursor_line = last_cursormoved.lnum;
	    conceal_new_cursor_line = curwin->w_cursor.lnum;
	    conceal_update_lines = TRUE;
	}
# endif
	last_cursormoved = curwin->w_cursor;
    }

    if (!finish_op)
	may_trigger_win_scrolled_resized();

# ifdef FEAT_CONCEAL
    if (conceal_update_lines
	    && (conceal_old_cursor_line != conceal_new_cursor_line
		|| conceal_cursor_line(curwin)
		|| need_cursor_line_redraw))
    {
	if (conceal_old_cursor_line != conceal_new_cursor_line)
	    redrawWinline(curwin, conceal_old_cursor_line);
	redrawWinline(curwin, conceal_new_cursor_line);
	curwin->w_valid &= ~VALID_CROW;
	need_cursor_line_redraw = FALSE;
    }
# endif
    update_screen(0);	// may need to update the screen
    setcursor();
    out_flush_cursor(TRUE, FALSE);
}
#endif

#if defined(FIND_REPLACE_DIALOG) || defined(PROTO)
/*
 * Get the text to use in a find/replace dialog.  Uses the last search pattern
 * if the argument is empty.
 * Returns an allocated string.
 */
    char_u *
get_find_dialog_text(
    char_u	*arg,
    int		*wwordp,	// return: TRUE if \< \> found
    int		*mcasep)	// return: TRUE if \C found
{
    char_u	*text;

    if (*arg == NUL)
	text = last_search_pat();
    else
	text = arg;
    if (text != NULL)
    {
	text = vim_strsave(text);
	if (text != NULL)
	{
	    int len = (int)STRLEN(text);
	    int i;

	    // Remove "\V"
	    if (len >= 2 && STRNCMP(text, "\\V", 2) == 0)
	    {
		mch_memmove(text, text + 2, (size_t)(len - 1));
		len -= 2;
	    }

	    // Recognize "\c" and "\C" and remove.
	    if (len >= 2 && *text == '\\' && (text[1] == 'c' || text[1] == 'C'))
	    {
		*mcasep = (text[1] == 'C');
		mch_memmove(text, text + 2, (size_t)(len - 1));
		len -= 2;
	    }

	    // Recognize "\<text\>" and remove.
	    if (len >= 4
		    && STRNCMP(text, "\\<", 2) == 0
		    && STRNCMP(text + len - 2, "\\>", 2) == 0)
	    {
		*wwordp = TRUE;
		mch_memmove(text, text + 2, (size_t)(len - 4));
		text[len - 4] = NUL;
	    }

	    // Recognize "\/" or "\?" and remove.
	    for (i = 0; i + 1 < len; ++i)
		if (text[i] == '\\' && (text[i + 1] == '/'
						       || text[i + 1] == '?'))
		{
		    mch_memmove(text + i, text + i + 1, (size_t)(len - i));
		    --len;
		}
	}
    }
    return text;
}

/*
 * Handle the press of a button in the find-replace dialog.
 * Return TRUE when something was added to the input buffer.
 */
    int
gui_do_findrepl(
    int		flags,		// one of FRD_REPLACE, FRD_FINDNEXT, etc.
    char_u	*find_text,
    char_u	*repl_text,
    int		down)		// Search downwards.
{
    garray_T	ga;
    int		i;
    int		type = (flags & FRD_TYPE_MASK);
    char_u	*p;
    regmatch_T	regmatch;
    int		save_did_emsg = did_emsg;
    static int  busy = FALSE;

    // When the screen is being updated we should not change buffers and
    // windows structures, it may cause freed memory to be used.  Also don't
    // do this recursively (pressing "Find" quickly several times).
    if (updating_screen || busy)
	return FALSE;

    // refuse replace when text cannot be changed
    if ((type == FRD_REPLACE || type == FRD_REPLACEALL) && text_locked())
	return FALSE;

    busy = TRUE;

    ga_init2(&ga, 1, 100);
    if (type == FRD_REPLACEALL)
	ga_concat(&ga, (char_u *)"%s/");

    ga_concat(&ga, (char_u *)"\\V");
    if (flags & FRD_MATCH_CASE)
	ga_concat(&ga, (char_u *)"\\C");
    else
	ga_concat(&ga, (char_u *)"\\c");
    if (flags & FRD_WHOLE_WORD)
	ga_concat(&ga, (char_u *)"\\<");
    // escape slash and backslash
    p = vim_strsave_escaped(find_text, (char_u *)"/\\");
    if (p != NULL)
	ga_concat(&ga, p);
    vim_free(p);
    if (flags & FRD_WHOLE_WORD)
	ga_concat(&ga, (char_u *)"\\>");

    if (type == FRD_REPLACEALL)
    {
	ga_concat(&ga, (char_u *)"/");
	// Escape slash and backslash.
	// Also escape tilde and ampersand if 'magic' is set.
	p = vim_strsave_escaped(repl_text,
				p_magic ? (char_u *)"/\\~&" : (char_u *)"/\\");
	if (p != NULL)
	    ga_concat(&ga, p);
	vim_free(p);
	ga_concat(&ga, (char_u *)"/g");
    }
    ga_append(&ga, NUL);

    if (type == FRD_REPLACE)
    {
	// Do the replacement when the text at the cursor matches.  Thus no
	// replacement is done if the cursor was moved!
	regmatch.regprog = vim_regcomp(ga.ga_data, RE_MAGIC + RE_STRING);
	regmatch.rm_ic = 0;
	if (regmatch.regprog != NULL)
	{
	    p = ml_get_cursor();
	    if (vim_regexec_nl(&regmatch, p, (colnr_T)0)
						   && regmatch.startp[0] == p)
	    {
		// Clear the command line to remove any old "No match"
		// error.
		msg_end_prompt();

		if (u_save_cursor() == OK)
		{
		    // A button was pressed thus undo should be synced.
		    u_sync(FALSE);

		    del_bytes((long)(regmatch.endp[0] - regmatch.startp[0]),
								FALSE, FALSE);
		    ins_str(repl_text);
		}
	    }
	    else
		msg(_("No match at cursor, finding next"));
	    vim_regfree(regmatch.regprog);
	}
    }

    if (type == FRD_REPLACEALL)
    {
	// A button was pressed, thus undo should be synced.
	u_sync(FALSE);
	do_cmdline_cmd(ga.ga_data);
    }
    else
    {
	int searchflags = SEARCH_MSG + SEARCH_MARK;

	// Search for the next match.
	// Don't skip text under cursor for single replace.
	if (type == FRD_REPLACE)
	    searchflags += SEARCH_START;
	i = msg_scroll;
	if (down)
	{
	    (void)do_search(NULL, '/', '/', ga.ga_data, 1L, searchflags, NULL);
	}
	else
	{
	    // We need to escape '?' if and only if we are searching in the up
	    // direction
	    p = vim_strsave_escaped(ga.ga_data, (char_u *)"?");
	    if (p != NULL)
		(void)do_search(NULL, '?', '?', p, 1L, searchflags, NULL);
	    vim_free(p);
	}

	msg_scroll = i;	    // don't let an error message set msg_scroll
    }

    // Don't want to pass did_emsg to other code, it may cause disabling
    // syntax HL if we were busy redrawing.
    did_emsg = save_did_emsg;

    if (State & (MODE_NORMAL | MODE_INSERT))
    {
	gui_update_screen();		// update the screen
	msg_didout = 0;			// overwrite any message
	need_wait_return = FALSE;	// don't wait for return
    }

    vim_free(ga.ga_data);
    busy = FALSE;
    return (ga.ga_len > 0);
}

#endif

#if defined(HAVE_DROP_FILE) || defined(PROTO)
/*
 * Jump to the window at specified point (x, y).
 */
    static void
gui_wingoto_xy(int x, int y)
{
    int		row = Y_2_ROW(y);
    int		col = X_2_COL(x);
    win_T	*wp;

    if (row < 0 || col < 0)
	return;

    wp = mouse_find_win(&row, &col, FAIL_POPUP);
    if (wp != NULL && wp != curwin)
	win_goto(wp);
}

/*
 * Function passed to handle_drop() for the actions to be done after the
 * argument list has been updated.
 */
    static void
drop_callback(void *cookie)
{
    char_u	*p = cookie;
    int		do_shorten = FALSE;

    // If Shift held down, change to first file's directory.  If the first
    // item is a directory, change to that directory (and let the explorer
    // plugin show the contents).
    if (p != NULL)
    {
	if (mch_isdir(p))
	{
	    if (mch_chdir((char *)p) == 0)
		do_shorten = TRUE;
	}
	else if (vim_chdirfile(p, "drop") == OK)
	    do_shorten = TRUE;
	vim_free(p);
	if (do_shorten)
	{
	    shorten_fnames(TRUE);
	    last_chdir_reason = "drop";
	}
    }

    // Update the screen display
    update_screen(UPD_NOT_VALID);
# ifdef FEAT_MENU
    gui_update_menus(0);
# endif
    maketitle();
    setcursor();
    out_flush_cursor(FALSE, FALSE);
}

/*
 * Process file drop.  Mouse cursor position, key modifiers, name of files
 * and count of files are given.  Argument "fnames[count]" has full pathnames
 * of dropped files, they will be freed in this function, and caller can't use
 * fnames after call this function.
 */
    void
gui_handle_drop(
    int		x UNUSED,
    int		y UNUSED,
    int_u	modifiers,
    char_u	**fnames,
    int		count)
{
    int		i;
    char_u	*p;
    static int	entered = FALSE;

    /*
     * This function is called by event handlers.  Just in case we get a
     * second event before the first one is handled, ignore the second one.
     * Not sure if this can ever happen, just in case.
     */
    if (entered)
	return;
    entered = TRUE;

    /*
     * When the cursor is at the command line, add the file names to the
     * command line, don't edit the files.
     */
    if (State & MODE_CMDLINE)
    {
	shorten_filenames(fnames, count);
	for (i = 0; i < count; ++i)
	{
	    if (fnames[i] != NULL)
	    {
		if (i > 0)
		    add_to_input_buf((char_u*)" ", 1);

		// We don't know what command is used thus we can't be sure
		// about which characters need to be escaped.  Only escape the
		// most common ones.
# ifdef BACKSLASH_IN_FILENAME
		p = vim_strsave_escaped(fnames[i], (char_u *)" \t\"|");
# else
		p = vim_strsave_escaped(fnames[i], (char_u *)"\\ \t\"|");
# endif
		if (p != NULL)
		    add_to_input_buf_csi(p, (int)STRLEN(p));
		vim_free(p);
		vim_free(fnames[i]);
	    }
	}
	vim_free(fnames);
    }
    else
    {
	// Go to the window under mouse cursor, then shorten given "fnames" by
	// current window, because a window can have local current dir.
	gui_wingoto_xy(x, y);
	shorten_filenames(fnames, count);

	// If Shift held down, remember the first item.
	if ((modifiers & MOUSE_SHIFT) != 0)
	    p = vim_strsave(fnames[0]);
	else
	    p = NULL;

	// Handle the drop, :edit or :split to get to the file.  This also
	// frees fnames[].  Skip this if there is only one item, it's a
	// directory and Shift is held down.
	if (count == 1 && (modifiers & MOUSE_SHIFT) != 0
						     && mch_isdir(fnames[0]))
	{
	    vim_free(fnames[0]);
	    vim_free(fnames);
	    vim_free(p);
	}
	else
	    handle_drop(count, fnames, (modifiers & MOUSE_CTRL) != 0,
						     drop_callback, (void *)p);
    }

    entered = FALSE;
}
#endif

/*
 * Check if "key" is to interrupt us.  Handles a key that has not had modifiers
 * applied yet.
 * Return the key with modifiers applied if so, NUL if not.
 */
    int
check_for_interrupt(int key, int modifiers_arg)
{
    int modifiers = modifiers_arg;
    int c = merge_modifyOtherKeys(key, &modifiers);

    if ((c == Ctrl_C && ctrl_c_interrupts)
#ifdef UNIX
	    || (intr_char != Ctrl_C && c == intr_char)
#endif
	    )
    {
	got_int = TRUE;
	return c;
    }
    return NUL;
}

/*
 * If the "--gui-log-file fname" argument is given write the dialog title and
 * message to a file and return TRUE.  Otherwise return FALSE.
 * When there is any problem opening the file or writing to the file this is
 * ignored, showing the dialog might get the test to get stuck.
 */
    int
gui_dialog_log(char_u *title, char_u *message)
{
    char_u  *fname = get_gui_dialog_file();
    FILE    *fd;

    if (fname == NULL)
	return FALSE;

    fd = mch_fopen((char *)fname, "a");
    if (fd != NULL)
    {
	fprintf(fd, "%s: %s\n", title, message);
	fclose(fd);
    }
    return TRUE;
}
