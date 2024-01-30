/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read a list of people who contributed.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

#include "vim.h"

static void cmd_with_count(char *cmd, char_u *bufp, size_t bufsize, long Prenum);
static void win_init(win_T *newp, win_T *oldp, int flags);
static void win_init_some(win_T *newp, win_T *oldp);
static void frame_comp_pos(frame_T *topfrp, int *row, int *col);
static void frame_setheight(frame_T *curfrp, int height);
static void frame_setwidth(frame_T *curfrp, int width);
static void win_exchange(long);
static void win_rotate(int, int);
static void win_totop(int size, int flags);
static void win_equal_rec(win_T *next_curwin, int current, frame_T *topfr, int dir, int col, int row, int width, int height);
static void trigger_winnewpre(void);
static void trigger_winclosed(win_T *win);
static win_T *win_free_mem(win_T *win, int *dirp, tabpage_T *tp);
static frame_T *win_altframe(win_T *win, tabpage_T *tp);
static tabpage_T *alt_tabpage(void);
static win_T *frame2win(frame_T *frp);
static int frame_has_win(frame_T *frp, win_T *wp);
static void win_fix_scroll(int resize);
static void win_fix_cursor(int normal);
static void frame_new_height(frame_T *topfrp, int height, int topfirst, int wfh);
static int frame_fixed_height(frame_T *frp);
static int frame_fixed_width(frame_T *frp);
static void frame_add_statusline(frame_T *frp);
static void frame_new_width(frame_T *topfrp, int width, int leftfirst, int wfw);
static void frame_add_vsep(frame_T *frp);
static int frame_minwidth(frame_T *topfrp, win_T *next_curwin);
static void frame_fix_width(win_T *wp);
static int win_alloc_firstwin(win_T *oldwin);
static void new_frame(win_T *wp);
static tabpage_T *alloc_tabpage(void);
static int leave_tabpage(buf_T *new_curbuf, int trigger_leave_autocmds);
static void enter_tabpage(tabpage_T *tp, buf_T *old_curbuf, int trigger_enter_autocmds, int trigger_leave_autocmds);
static void frame_fix_height(win_T *wp);
static int frame_minheight(frame_T *topfrp, win_T *next_curwin);
static int may_open_tabpage(void);
static int win_enter_ext(win_T *wp, int flags);
static void win_free(win_T *wp, tabpage_T *tp);
static void win_append(win_T *after, win_T *wp);
static void frame_append(frame_T *after, frame_T *frp);
static void frame_insert(frame_T *before, frame_T *frp);
static void frame_remove(frame_T *frp);
static void win_goto_ver(int up, long count);
static void win_goto_hor(int left, long count);
static void frame_add_height(frame_T *frp, int n);
static void last_status_rec(frame_T *fr, int statusline);

static void make_snapshot_rec(frame_T *fr, frame_T **frp);
static void clear_snapshot(tabpage_T *tp, int idx);
static void clear_snapshot_rec(frame_T *fr);
static int check_snapshot_rec(frame_T *sn, frame_T *fr);
static win_T *restore_snapshot_rec(frame_T *sn, frame_T *fr);
static win_T *get_snapshot_curwin(int idx);

static int frame_check_height(frame_T *topfrp, int height);
static int frame_check_width(frame_T *topfrp, int width);

static win_T *win_alloc(win_T *after, int hidden);

#define NOWIN		((win_T *)-1)	// non-existing window

#define ROWS_AVAIL (Rows - p_ch - tabline_height())

// flags for win_enter_ext()
#define WEE_UNDO_SYNC			0x01
#define WEE_CURWIN_INVALID		0x02
#define WEE_TRIGGER_NEW_AUTOCMDS	0x04
#define WEE_TRIGGER_ENTER_AUTOCMDS	0x08
#define WEE_TRIGGER_LEAVE_AUTOCMDS	0x10
#define WEE_ALLOW_PARSE_MESSAGES	0x20

static char *m_onlyone = N_("Already only one window");

// When non-zero splitting a window is forbidden.  Used to avoid that nasty
// autocommands mess up the window structure.
static int split_disallowed = 0;

// When non-zero closing a window is forbidden.  Used to avoid that nasty
// autocommands mess up the window structure.
static int close_disallowed = 0;

/*
 * Disallow changing the window layout (split window, close window, move
 * window).  Resizing is still allowed.
 * Used for autocommands that temporarily use another window and need to
 * make sure the previously selected window is still there.
 * Must be matched with exactly one call to window_layout_unlock()!
 */
    static void
window_layout_lock(void)
{
    ++split_disallowed;
    ++close_disallowed;
}

    static void
window_layout_unlock(void)
{
    --split_disallowed;
    --close_disallowed;
}

/*
 * When the window layout cannot be changed give an error and return TRUE.
 * "cmd" indicates the action being performed and is used to pick the relevant
 * error message.
 */
    int
window_layout_locked(enum CMD_index cmd)
{
    if (split_disallowed > 0 || close_disallowed > 0)
    {
	if (close_disallowed == 0 && cmd == CMD_tabnew)
	    emsg(_(e_cannot_split_window_when_closing_buffer));
	else
	    emsg(_(e_not_allowed_to_change_window_layout_in_this_autocmd));
	return TRUE;
    }
    return FALSE;
}

// #define WIN_DEBUG
#ifdef WIN_DEBUG
/*
 * Call this method to log the current window layout.
 */
    static void
log_frame_layout(frame_T *frame)
{
    ch_log(NULL, "layout %s, wi: %d, he: %d, wwi: %d, whe: %d, id: %d",
	    frame->fr_layout == FR_LEAF ? "LEAF"
				  : frame->fr_layout == FR_ROW ? "ROW" : "COL",
	    frame->fr_width,
	    frame->fr_height,
	    frame->fr_win == NULL ? -1 : frame->fr_win->w_width,
	    frame->fr_win == NULL ? -1 : frame->fr_win->w_height,
	    frame->fr_win == NULL ? -1 : frame->fr_win->w_id);
    if (frame->fr_child != NULL)
    {
	ch_log(NULL, "children");
	log_frame_layout(frame->fr_child);
	if (frame->fr_next != NULL)
	    ch_log(NULL, "END of children");
    }
    if (frame->fr_next != NULL)
	log_frame_layout(frame->fr_next);
}
#endif

/*
 * Return the current window, unless in the cmdline window and "prevwin" is
 * set, then return "prevwin".
 */
    win_T *
prevwin_curwin(void)
{
    // In cmdwin, the alternative buffer should be used.
    return is_in_cmdwin() && prevwin != NULL ? prevwin : curwin;
}

/*
 * If the 'switchbuf' option contains "useopen" or "usetab", then try to jump
 * to a window containing "buf".
 * Returns the pointer to the window that was jumped to or NULL.
 */
    win_T *
swbuf_goto_win_with_buf(buf_T *buf)
{
    win_T   *wp = NULL;

    if (buf == NULL)
	return wp;

    // If 'switchbuf' contains "useopen": jump to first window in the current
    // tab page containing "buf" if one exists.
    if (swb_flags & SWB_USEOPEN)
	wp = buf_jump_open_win(buf);

    // If 'switchbuf' contains "usetab": jump to first window in any tab page
    // containing "buf" if one exists.
    if (wp == NULL && (swb_flags & SWB_USETAB))
	wp = buf_jump_open_tab(buf);

    return wp;
}

/*
 * All CTRL-W window commands are handled here, called from normal_cmd().
 */
    void
do_window(
    int		nchar,
    long	Prenum,
    int		xchar)	    // extra char from ":wincmd gx" or NUL
{
    long	Prenum1;
    win_T	*wp;
    char_u	*ptr;
    linenr_T    lnum = -1;
#ifdef FEAT_FIND_ID
    int		type = FIND_DEFINE;
    int		len;
#endif
    char_u	cbuf[40];

    if (ERROR_IF_ANY_POPUP_WINDOW)
	return;

#define CHECK_CMDWIN \
    do { \
	if (cmdwin_type != 0) \
	{ \
	    emsg(_(e_invalid_in_cmdline_window)); \
	    return; \
	} \
    } while (0)

    Prenum1 = Prenum == 0 ? 1 : Prenum;

    switch (nchar)
    {
// split current window in two parts, horizontally
    case 'S':
    case Ctrl_S:
    case 's':
		CHECK_CMDWIN;
		reset_VIsual_and_resel();	// stop Visual mode
		// When splitting the quickfix window open a new buffer in it,
		// don't replicate the quickfix buffer.
		if (bt_quickfix(curbuf))
		    goto newwindow;
#ifdef FEAT_GUI
		need_mouse_correct = TRUE;
#endif
		(void)win_split((int)Prenum, 0);
		break;

// split current window in two parts, vertically
    case Ctrl_V:
    case 'v':
		CHECK_CMDWIN;
		reset_VIsual_and_resel();	// stop Visual mode
		// When splitting the quickfix window open a new buffer in it,
		// don't replicate the quickfix buffer.
		if (bt_quickfix(curbuf))
		    goto newwindow;
#ifdef FEAT_GUI
		need_mouse_correct = TRUE;
#endif
		(void)win_split((int)Prenum, WSP_VERT);
		break;

// split current window and edit alternate file
    case Ctrl_HAT:
    case '^':
		CHECK_CMDWIN;
		reset_VIsual_and_resel();	// stop Visual mode

		if (buflist_findnr(Prenum == 0
					? curwin->w_alt_fnum : Prenum) == NULL)
		{
		    if (Prenum == 0)
			emsg(_(e_no_alternate_file));
		    else
			semsg(_(e_buffer_nr_not_found), Prenum);
		    break;
		}

		if (!curbuf_locked() && win_split(0, 0) == OK)
		    (void)buflist_getfile(
			    Prenum == 0 ? curwin->w_alt_fnum : Prenum,
			    (linenr_T)0, GETF_ALT, FALSE);
		break;

// open new window
    case Ctrl_N:
    case 'n':
		CHECK_CMDWIN;
		reset_VIsual_and_resel();	// stop Visual mode
newwindow:
		if (Prenum)
		    // window height
		    vim_snprintf((char *)cbuf, sizeof(cbuf) - 5, "%ld", Prenum);
		else
		    cbuf[0] = NUL;
#if defined(FEAT_QUICKFIX)
		if (nchar == 'v' || nchar == Ctrl_V)
		    STRCAT(cbuf, "v");
#endif
		STRCAT(cbuf, "new");
		do_cmdline_cmd(cbuf);
		break;

// quit current window
    case Ctrl_Q:
    case 'q':
		reset_VIsual_and_resel();	// stop Visual mode
		cmd_with_count("quit", cbuf, sizeof(cbuf), Prenum);
		do_cmdline_cmd(cbuf);
		break;

// close current window
    case Ctrl_C:
    case 'c':
		reset_VIsual_and_resel();	// stop Visual mode
		cmd_with_count("close", cbuf, sizeof(cbuf), Prenum);
		do_cmdline_cmd(cbuf);
		break;

#if defined(FEAT_QUICKFIX)
// close preview window
    case Ctrl_Z:
    case 'z':
		CHECK_CMDWIN;
		reset_VIsual_and_resel();	// stop Visual mode
		do_cmdline_cmd((char_u *)"pclose");
		break;

// cursor to preview window
    case 'P':
		FOR_ALL_WINDOWS(wp)
		    if (wp->w_p_pvw)
			break;
		if (wp == NULL)
		    emsg(_(e_there_is_no_preview_window));
		else
		    win_goto(wp);
		break;
#endif

// close all but current window
    case Ctrl_O:
    case 'o':
		CHECK_CMDWIN;
		reset_VIsual_and_resel();	// stop Visual mode
		cmd_with_count("only", cbuf, sizeof(cbuf), Prenum);
		do_cmdline_cmd(cbuf);
		break;

// cursor to next window with wrap around
    case Ctrl_W:
    case 'w':
// cursor to previous window with wrap around
    case 'W':
		CHECK_CMDWIN;
		if (ONE_WINDOW && Prenum != 1)	// just one window
		    beep_flush();
		else
		{
		    if (Prenum)			// go to specified window
		    {
			for (wp = firstwin; --Prenum > 0; )
			{
			    if (wp->w_next == NULL)
				break;
			    else
				wp = wp->w_next;
			}
		    }
		    else
		    {
			if (nchar == 'W')	    // go to previous window
			{
			    wp = curwin->w_prev;
			    if (wp == NULL)
				wp = lastwin;	    // wrap around
			}
			else			    // go to next window
			{
			    wp = curwin->w_next;
			    if (wp == NULL)
				wp = firstwin;	    // wrap around
			}
		    }
		    win_goto(wp);
		}
		break;

// cursor to window below
    case 'j':
    case K_DOWN:
    case Ctrl_J:
		CHECK_CMDWIN;
		win_goto_ver(FALSE, Prenum1);
		break;

// cursor to window above
    case 'k':
    case K_UP:
    case Ctrl_K:
		CHECK_CMDWIN;
		win_goto_ver(TRUE, Prenum1);
		break;

// cursor to left window
    case 'h':
    case K_LEFT:
    case Ctrl_H:
    case K_BS:
		CHECK_CMDWIN;
		win_goto_hor(TRUE, Prenum1);
		break;

// cursor to right window
    case 'l':
    case K_RIGHT:
    case Ctrl_L:
		CHECK_CMDWIN;
		win_goto_hor(FALSE, Prenum1);
		break;

// move window to new tab page
    case 'T':
		CHECK_CMDWIN;
		if (one_window())
		    msg(_(m_onlyone));
		else
		{
		    tabpage_T	*oldtab = curtab;
		    tabpage_T	*newtab;

		    // First create a new tab with the window, then go back to
		    // the old tab and close the window there.
		    wp = curwin;
		    if (win_new_tabpage((int)Prenum) == OK
						     && valid_tabpage(oldtab))
		    {
			newtab = curtab;
			goto_tabpage_tp(oldtab, TRUE, TRUE);
			if (curwin == wp)
			    win_close(curwin, FALSE);
			if (valid_tabpage(newtab))
			    goto_tabpage_tp(newtab, TRUE, TRUE);
		    }
		}
		break;

// cursor to top-left window
    case 't':
    case Ctrl_T:
		win_goto(firstwin);
		break;

// cursor to bottom-right window
    case 'b':
    case Ctrl_B:
		win_goto(lastwin);
		break;

// cursor to last accessed (previous) window
    case 'p':
    case Ctrl_P:
		if (!win_valid(prevwin))
		    beep_flush();
		else
		    win_goto(prevwin);
		break;

// exchange current and next window
    case 'x':
    case Ctrl_X:
		CHECK_CMDWIN;
		win_exchange(Prenum);
		break;

// rotate windows downwards
    case Ctrl_R:
    case 'r':
		CHECK_CMDWIN;
		reset_VIsual_and_resel();	// stop Visual mode
		win_rotate(FALSE, (int)Prenum1);    // downwards
		break;

// rotate windows upwards
    case 'R':
		CHECK_CMDWIN;
		reset_VIsual_and_resel();	// stop Visual mode
		win_rotate(TRUE, (int)Prenum1);	    // upwards
		break;

// move window to the very top/bottom/left/right
    case 'K':
    case 'J':
    case 'H':
    case 'L':
		CHECK_CMDWIN;
		win_totop((int)Prenum,
			((nchar == 'H' || nchar == 'L') ? WSP_VERT : 0)
			| ((nchar == 'H' || nchar == 'K') ? WSP_TOP : WSP_BOT));
		break;

// make all windows the same width and/or height
    case '=':
		{
		    int mod = cmdmod.cmod_split & (WSP_VERT | WSP_HOR);
#ifdef FEAT_GUI
		    need_mouse_correct = TRUE;
#endif
		    win_equal(NULL, FALSE,
			   mod == WSP_VERT ? 'v' : mod == WSP_HOR ? 'h' : 'b');
		}
		break;

// increase current window height
    case '+':
#ifdef FEAT_GUI
		need_mouse_correct = TRUE;
#endif
		win_setheight(curwin->w_height + (int)Prenum1);
		break;

// decrease current window height
    case '-':
#ifdef FEAT_GUI
		need_mouse_correct = TRUE;
#endif
		win_setheight(curwin->w_height - (int)Prenum1);
		break;

// set current window height
    case Ctrl__:
    case '_':
#ifdef FEAT_GUI
		need_mouse_correct = TRUE;
#endif
		win_setheight(Prenum ? (int)Prenum : 9999);
		break;

// increase current window width
    case '>':
#ifdef FEAT_GUI
		need_mouse_correct = TRUE;
#endif
		win_setwidth(curwin->w_width + (int)Prenum1);
		break;

// decrease current window width
    case '<':
#ifdef FEAT_GUI
		need_mouse_correct = TRUE;
#endif
		win_setwidth(curwin->w_width - (int)Prenum1);
		break;

// set current window width
    case '|':
#ifdef FEAT_GUI
		need_mouse_correct = TRUE;
#endif
		win_setwidth(Prenum != 0 ? (int)Prenum : 9999);
		break;

// jump to tag and split window if tag exists (in preview window)
#if defined(FEAT_QUICKFIX)
    case '}':
		CHECK_CMDWIN;
		if (Prenum)
		    g_do_tagpreview = Prenum;
		else
		    g_do_tagpreview = p_pvh;
#endif
		// FALLTHROUGH
    case ']':
    case Ctrl_RSB:
		CHECK_CMDWIN;
		// keep Visual mode, can select words to use as a tag
		if (Prenum)
		    postponed_split = Prenum;
		else
		    postponed_split = -1;
#ifdef FEAT_QUICKFIX
		if (nchar != '}')
		    g_do_tagpreview = 0;
#endif

		// Execute the command right here, required when "wincmd ]"
		// was used in a function.
		do_nv_ident(Ctrl_RSB, NUL);
		postponed_split = 0;
		break;

// edit file name under cursor in a new window
    case 'f':
    case 'F':
    case Ctrl_F:
wingotofile:
		CHECK_CMDWIN;
		if (check_text_or_curbuf_locked(NULL))
		    break;

		ptr = grab_file_name(Prenum1, &lnum);
		if (ptr != NULL)
		{
		    tabpage_T	*oldtab = curtab;
		    win_T	*oldwin = curwin;
#ifdef FEAT_GUI
		    need_mouse_correct = TRUE;
#endif
		    setpcmark();

		    // If 'switchbuf' is set to 'useopen' or 'usetab' and the
		    // file is already opened in a window, then jump to it.
		    wp = NULL;
		    if ((swb_flags & (SWB_USEOPEN | SWB_USETAB))
						&& cmdmod.cmod_tab == 0)
			wp = swbuf_goto_win_with_buf(buflist_findname_exp(ptr));

		    if (wp == NULL && win_split(0, 0) == OK)
		    {
			RESET_BINDING(curwin);
			if (do_ecmd(0, ptr, NULL, NULL, ECMD_LASTL,
						   ECMD_HIDE, NULL) == FAIL)
			{
			    // Failed to open the file, close the window
			    // opened for it.
			    win_close(curwin, FALSE);
			    goto_tabpage_win(oldtab, oldwin);
			}
			else
			    wp = curwin;
		    }

		    if (wp != NULL && nchar == 'F' && lnum >= 0)
		    {
			curwin->w_cursor.lnum = lnum;
			check_cursor_lnum();
			beginline(BL_SOL | BL_FIX);
		    }
		    vim_free(ptr);
		}
		break;

#ifdef FEAT_FIND_ID
// Go to the first occurrence of the identifier under cursor along path in a
// new window -- webb
    case 'i':			    // Go to any match
    case Ctrl_I:
		type = FIND_ANY;
		// FALLTHROUGH
    case 'd':			    // Go to definition, using 'define'
    case Ctrl_D:
		CHECK_CMDWIN;
		if ((len = find_ident_under_cursor(&ptr, FIND_IDENT)) == 0)
		    break;

		// Make a copy, if the line was changed it will be freed.
		ptr = vim_strnsave(ptr, len);
		if (ptr == NULL)
		    break;

		find_pattern_in_path(ptr, 0, len, TRUE,
			Prenum == 0 ? TRUE : FALSE, type,
			Prenum1, ACTION_SPLIT, (linenr_T)1, (linenr_T)MAXLNUM);
		vim_free(ptr);
		curwin->w_set_curswant = TRUE;
		break;
#endif

// Quickfix window only: view the result under the cursor in a new split.
#if defined(FEAT_QUICKFIX)
    case K_KENTER:
    case CAR:
		if (bt_quickfix(curbuf))
		    qf_view_result(TRUE);
		break;
#endif

// CTRL-W g  extended commands
    case 'g':
    case Ctrl_G:
		CHECK_CMDWIN;
#ifdef USE_ON_FLY_SCROLL
		dont_scroll = TRUE;		// disallow scrolling here
#endif
		++no_mapping;
		++allow_keys;   // no mapping for xchar, but allow key codes
		if (xchar == NUL)
		    xchar = plain_vgetc();
		LANGMAP_ADJUST(xchar, TRUE);
		--no_mapping;
		--allow_keys;
		(void)add_to_showcmd(xchar);

		switch (xchar)
		{
#if defined(FEAT_QUICKFIX)
		    case '}':
			xchar = Ctrl_RSB;
			if (Prenum)
			    g_do_tagpreview = Prenum;
			else
			    g_do_tagpreview = p_pvh;
#endif
			// FALLTHROUGH
		    case ']':
		    case Ctrl_RSB:
			// keep Visual mode, can select words to use as a tag
			if (Prenum)
			    postponed_split = Prenum;
			else
			    postponed_split = -1;

			// Execute the command right here, required when
			// "wincmd g}" was used in a function.
			do_nv_ident('g', xchar);
			postponed_split = 0;
			break;

		    case 'f':	    // CTRL-W gf: "gf" in a new tab page
		    case 'F':	    // CTRL-W gF: "gF" in a new tab page
			cmdmod.cmod_tab = tabpage_index(curtab) + 1;
			nchar = xchar;
			goto wingotofile;

		    case 't':	    // CTRL-W gt: go to next tab page
			goto_tabpage((int)Prenum);
			break;

		    case 'T':	    // CTRL-W gT: go to previous tab page
			goto_tabpage(-(int)Prenum1);
			break;

		    case TAB:	    // CTRL-W g<Tab>: go to last used tab page
			if (goto_tabpage_lastused() == FAIL)
			    beep_flush();
			break;

		    default:
			beep_flush();
			break;
		}
		break;

    default:	beep_flush();
		break;
    }
}

/*
 * Figure out the address type for ":wincmd".
 */
    void
get_wincmd_addr_type(char_u *arg, exarg_T *eap)
{
    switch (*arg)
    {
    case 'S':
    case Ctrl_S:
    case 's':
    case Ctrl_N:
    case 'n':
    case 'j':
    case Ctrl_J:
    case 'k':
    case Ctrl_K:
    case 'T':
    case Ctrl_R:
    case 'r':
    case 'R':
    case 'K':
    case 'J':
    case '+':
    case '-':
    case Ctrl__:
    case '_':
    case '|':
    case ']':
    case Ctrl_RSB:
    case 'g':
    case Ctrl_G:
    case Ctrl_V:
    case 'v':
    case 'h':
    case Ctrl_H:
    case 'l':
    case Ctrl_L:
    case 'H':
    case 'L':
    case '>':
    case '<':
#if defined(FEAT_QUICKFIX)
    case '}':
#endif
    case 'f':
    case 'F':
    case Ctrl_F:
#ifdef FEAT_FIND_ID
    case 'i':
    case Ctrl_I:
    case 'd':
    case Ctrl_D:
#endif
		// window size or any count
		eap->addr_type = ADDR_OTHER;
		break;

    case Ctrl_HAT:
    case '^':
		// buffer number
		eap->addr_type = ADDR_BUFFERS;
		break;

    case Ctrl_Q:
    case 'q':
    case Ctrl_C:
    case 'c':
    case Ctrl_O:
    case 'o':
    case Ctrl_W:
    case 'w':
    case 'W':
    case 'x':
    case Ctrl_X:
		// window number
		eap->addr_type = ADDR_WINDOWS;
		break;

#if defined(FEAT_QUICKFIX)
    case Ctrl_Z:
    case 'z':
    case 'P':
#endif
    case 't':
    case Ctrl_T:
    case 'b':
    case Ctrl_B:
    case 'p':
    case Ctrl_P:
    case '=':
    case CAR:
		// no count
		eap->addr_type = ADDR_NONE;
		break;
    }
}

    static void
cmd_with_count(
    char	*cmd,
    char_u	*bufp,
    size_t	bufsize,
    long	Prenum)
{
    if (Prenum > 0)
	vim_snprintf((char *)bufp, bufsize, "%s %ld", cmd, Prenum);
    else
	STRCPY(bufp, cmd);
}

/*
 * If "split_disallowed" is set give an error and return FAIL.
 * Otherwise return OK.
 */
    static int
check_split_disallowed(void)
{
    if (split_disallowed > 0)
    {
	emsg(_(e_cant_split_window_while_closing_another));
	return FAIL;
    }
    if (curwin->w_buffer->b_locked_split)
    {
	emsg(_(e_cannot_split_window_when_closing_buffer));
	return FAIL;
    }
    return OK;
}

/*
 * split the current window, implements CTRL-W s and :split
 *
 * "size" is the height or width for the new window, 0 to use half of current
 * height or width.
 *
 * "flags":
 * WSP_ROOM: require enough room for new window
 * WSP_VERT: vertical split.
 * WSP_TOP:  open window at the top-left of the shell (help window).
 * WSP_BOT:  open window at the bottom-right of the shell (quickfix window).
 * WSP_HELP: creating the help window, keep layout snapshot
 *
 * return FAIL for failure, OK otherwise
 */
    int
win_split(int size, int flags)
{
    if (ERROR_IF_ANY_POPUP_WINDOW)
	return FAIL;

    if (check_split_disallowed() == FAIL)
	return FAIL;

    // When the ":tab" modifier was used open a new tab page instead.
    if (may_open_tabpage() == OK)
	return OK;

    // Add flags from ":vertical", ":topleft" and ":botright".
    flags |= cmdmod.cmod_split;
    if ((flags & WSP_TOP) && (flags & WSP_BOT))
    {
	emsg(_(e_cant_split_topleft_and_botright_at_the_same_time));
	return FAIL;
    }

    // When creating the help window make a snapshot of the window layout.
    // Otherwise clear the snapshot, it's now invalid.
    if (flags & WSP_HELP)
	make_snapshot(SNAP_HELP_IDX);
    else
	clear_snapshot(curtab, SNAP_HELP_IDX);

    return win_split_ins(size, flags, NULL, 0);
}

/*
 * When "new_wp" is NULL: split the current window in two.
 * When "new_wp" is not NULL: insert this window at the far
 * top/left/right/bottom.
 * Return FAIL for failure, OK otherwise.
 */
    int
win_split_ins(
    int		size,
    int		flags,
    win_T	*new_wp,
    int		dir)
{
    win_T	*wp = new_wp;
    win_T	*oldwin;
    int		new_size = size;
    int		i;
    int		need_status = 0;
    int		do_equal = FALSE;
    int		needed;
    int		available;
    int		oldwin_height = 0;
    int		layout;
    frame_T	*frp, *curfrp, *frp2, *prevfrp;
    int		before;
    int		minheight;
    int		wmh1;
    int		did_set_fraction = FALSE;
    int		retval = FAIL;

    // Do not redraw here, curwin->w_buffer may be invalid.
    ++RedrawingDisabled;

    trigger_winnewpre();

    if (flags & WSP_TOP)
	oldwin = firstwin;
    else if (flags & WSP_BOT)
	oldwin = lastwin;
    else
	oldwin = curwin;

    // add a status line when p_ls == 1 and splitting the first window
    if (ONE_WINDOW && p_ls == 1 && oldwin->w_status_height == 0)
    {
	if (VISIBLE_HEIGHT(oldwin) <= p_wmh && new_wp == NULL)
	{
	    emsg(_(e_not_enough_room));
	    goto theend;
	}
	need_status = STATUS_HEIGHT;
    }

#ifdef FEAT_GUI
    // May be needed for the scrollbars that are going to change.
    if (gui.in_use)
	out_flush();
#endif

    if (flags & WSP_VERT)
    {
	int	wmw1;
	int	minwidth;

	layout = FR_ROW;

	/*
	 * Check if we are able to split the current window and compute its
	 * width.
	 */
	// Current window requires at least 1 space.
	wmw1 = (p_wmw == 0 ? 1 : p_wmw);
	needed = wmw1 + 1;
	if (flags & WSP_ROOM)
	    needed += p_wiw - wmw1;
	if (flags & (WSP_BOT | WSP_TOP))
	{
	    minwidth = frame_minwidth(topframe, NOWIN);
	    available = topframe->fr_width;
	    needed += minwidth;
	}
	else if (p_ea)
	{
	    minwidth = frame_minwidth(oldwin->w_frame, NOWIN);
	    prevfrp = oldwin->w_frame;
	    for (frp = oldwin->w_frame->fr_parent; frp != NULL;
							frp = frp->fr_parent)
	    {
		if (frp->fr_layout == FR_ROW)
		    FOR_ALL_FRAMES(frp2, frp->fr_child)
			if (frp2 != prevfrp)
			    minwidth += frame_minwidth(frp2, NOWIN);
		prevfrp = frp;
	    }
	    available = topframe->fr_width;
	    needed += minwidth;
	}
	else
	{
	    minwidth = frame_minwidth(oldwin->w_frame, NOWIN);
	    available = oldwin->w_frame->fr_width;
	    needed += minwidth;
	}
	if (available < needed && new_wp == NULL)
	{
	    emsg(_(e_not_enough_room));
	    goto theend;
	}
	if (new_size == 0)
	    new_size = oldwin->w_width / 2;
	if (new_size > available - minwidth - 1)
	    new_size = available - minwidth - 1;
	if (new_size < wmw1)
	    new_size = wmw1;

	// if it doesn't fit in the current window, need win_equal()
	if (oldwin->w_width - new_size - 1 < p_wmw)
	    do_equal = TRUE;

	// We don't like to take lines for the new window from a
	// 'winfixwidth' window.  Take them from a window to the left or right
	// instead, if possible. Add one for the separator.
	if (oldwin->w_p_wfw)
	    win_setwidth_win(oldwin->w_width + new_size + 1, oldwin);

	// Only make all windows the same width if one of them (except oldwin)
	// is wider than one of the split windows.
	if (!do_equal && p_ea && size == 0 && *p_ead != 'v'
					 && oldwin->w_frame->fr_parent != NULL)
	{
	    frp = oldwin->w_frame->fr_parent->fr_child;
	    while (frp != NULL)
	    {
		if (frp->fr_win != oldwin && frp->fr_win != NULL
			&& (frp->fr_win->w_width > new_size
			    || frp->fr_win->w_width > oldwin->w_width
							      - new_size - 1))
		{
		    do_equal = TRUE;
		    break;
		}
		frp = frp->fr_next;
	    }
	}
    }
    else
    {
	layout = FR_COL;

	/*
	 * Check if we are able to split the current window and compute its
	 * height.
	 */
	// Current window requires at least 1 space.
	wmh1 = (p_wmh == 0 ? 1 : p_wmh) + WINBAR_HEIGHT(curwin);
	needed = wmh1 + STATUS_HEIGHT;
	if (flags & WSP_ROOM)
	    needed += p_wh - wmh1;
	if (flags & (WSP_BOT | WSP_TOP))
	{
	    minheight = frame_minheight(topframe, NOWIN) + need_status;
	    available = topframe->fr_height;
	    needed += minheight;
	}
	else if (p_ea)
	{
	    minheight = frame_minheight(oldwin->w_frame, NOWIN) + need_status;
	    prevfrp = oldwin->w_frame;
	    for (frp = oldwin->w_frame->fr_parent; frp != NULL;
							frp = frp->fr_parent)
	    {
		if (frp->fr_layout == FR_COL)
		    FOR_ALL_FRAMES(frp2, frp->fr_child)
			if (frp2 != prevfrp)
			    minheight += frame_minheight(frp2, NOWIN);
		prevfrp = frp;
	    }
	    available = topframe->fr_height;
	    needed += minheight;
	}
	else
	{
	    minheight = frame_minheight(oldwin->w_frame, NOWIN) + need_status;
	    available = oldwin->w_frame->fr_height;
	    needed += minheight;
	}
	if (available < needed && new_wp == NULL)
	{
	    emsg(_(e_not_enough_room));
	    goto theend;
	}
	oldwin_height = oldwin->w_height;
	if (need_status)
	{
	    oldwin->w_status_height = STATUS_HEIGHT;
	    oldwin_height -= STATUS_HEIGHT;
	}
	if (new_size == 0)
	    new_size = oldwin_height / 2;
	if (new_size > available - minheight - STATUS_HEIGHT)
	    new_size = available - minheight - STATUS_HEIGHT;
	if (new_size < wmh1)
	    new_size = wmh1;

	// if it doesn't fit in the current window, need win_equal()
	if (oldwin_height - new_size - STATUS_HEIGHT < p_wmh)
	    do_equal = TRUE;

	// We don't like to take lines for the new window from a
	// 'winfixheight' window.  Take them from a window above or below
	// instead, if possible.
	if (oldwin->w_p_wfh)
	{
	    // Set w_fraction now so that the cursor keeps the same relative
	    // vertical position using the old height.
	    set_fraction(oldwin);
	    did_set_fraction = TRUE;

	    win_setheight_win(oldwin->w_height + new_size + STATUS_HEIGHT,
								      oldwin);
	    oldwin_height = oldwin->w_height;
	    if (need_status)
		oldwin_height -= STATUS_HEIGHT;
	}

	// Only make all windows the same height if one of them (except oldwin)
	// is higher than one of the split windows.
	if (!do_equal && p_ea && size == 0 && *p_ead != 'h'
	   && oldwin->w_frame->fr_parent != NULL)
	{
	    frp = oldwin->w_frame->fr_parent->fr_child;
	    while (frp != NULL)
	    {
		if (frp->fr_win != oldwin && frp->fr_win != NULL
			&& (frp->fr_win->w_height > new_size
			    || frp->fr_win->w_height > oldwin_height - new_size
							      - STATUS_HEIGHT))
		{
		    do_equal = TRUE;
		    break;
		}
		frp = frp->fr_next;
	    }
	}
    }

    /*
     * allocate new window structure and link it in the window list
     */
    if ((flags & WSP_TOP) == 0
	    && ((flags & WSP_BOT)
		|| (flags & WSP_BELOW)
		|| (!(flags & WSP_ABOVE)
		    && ( (flags & WSP_VERT) ? p_spr : p_sb))))
    {
	// new window below/right of current one
	if (new_wp == NULL)
	    wp = win_alloc(oldwin, FALSE);
	else
	    win_append(oldwin, wp);
    }
    else
    {
	if (new_wp == NULL)
	    wp = win_alloc(oldwin->w_prev, FALSE);
	else
	    win_append(oldwin->w_prev, wp);
    }

    if (new_wp == NULL)
    {
	if (wp == NULL)
	    goto theend;

	new_frame(wp);
	if (wp->w_frame == NULL)
	{
	    win_free(wp, NULL);
	    goto theend;
	}

	// make the contents of the new window the same as the current one
	win_init(wp, curwin, flags);
    }

    /*
     * Reorganise the tree of frames to insert the new window.
     */
    if (flags & (WSP_TOP | WSP_BOT))
    {
	if ((topframe->fr_layout == FR_COL && (flags & WSP_VERT) == 0)
	    || (topframe->fr_layout == FR_ROW && (flags & WSP_VERT) != 0))
	{
	    curfrp = topframe->fr_child;
	    if (flags & WSP_BOT)
		while (curfrp->fr_next != NULL)
		    curfrp = curfrp->fr_next;
	}
	else
	    curfrp = topframe;
	before = (flags & WSP_TOP);
    }
    else
    {
	curfrp = oldwin->w_frame;
	if (flags & WSP_BELOW)
	    before = FALSE;
	else if (flags & WSP_ABOVE)
	    before = TRUE;
	else if (flags & WSP_VERT)
	    before = !p_spr;
	else
	    before = !p_sb;
    }
    if (curfrp->fr_parent == NULL || curfrp->fr_parent->fr_layout != layout)
    {
	// Need to create a new frame in the tree to make a branch.
	frp = ALLOC_CLEAR_ONE(frame_T);
	*frp = *curfrp;
	curfrp->fr_layout = layout;
	frp->fr_parent = curfrp;
	frp->fr_next = NULL;
	frp->fr_prev = NULL;
	curfrp->fr_child = frp;
	curfrp->fr_win = NULL;
	curfrp = frp;
	if (frp->fr_win != NULL)
	    oldwin->w_frame = frp;
	else
	    FOR_ALL_FRAMES(frp, frp->fr_child)
		frp->fr_parent = curfrp;
    }

    if (new_wp == NULL)
	frp = wp->w_frame;
    else
	frp = new_wp->w_frame;
    frp->fr_parent = curfrp->fr_parent;

    // Insert the new frame at the right place in the frame list.
    if (before)
	frame_insert(curfrp, frp);
    else
	frame_append(curfrp, frp);

    // Set w_fraction now so that the cursor keeps the same relative
    // vertical position.
    if (!did_set_fraction)
	set_fraction(oldwin);
    wp->w_fraction = oldwin->w_fraction;

    if (flags & WSP_VERT)
    {
	wp->w_p_scr = curwin->w_p_scr;

	if (need_status)
	{
	    win_new_height(oldwin, oldwin->w_height - 1);
	    oldwin->w_status_height = need_status;
	}
	if (flags & (WSP_TOP | WSP_BOT))
	{
	    // set height and row of new window to full height
	    wp->w_winrow = tabline_height();
	    win_new_height(wp, curfrp->fr_height - (p_ls > 0)
							  - WINBAR_HEIGHT(wp));
	    wp->w_status_height = (p_ls > 0);
	}
	else
	{
	    // height and row of new window is same as current window
	    wp->w_winrow = oldwin->w_winrow;
	    win_new_height(wp, VISIBLE_HEIGHT(oldwin));
	    wp->w_status_height = oldwin->w_status_height;
	}
	frp->fr_height = curfrp->fr_height;

	// "new_size" of the current window goes to the new window, use
	// one column for the vertical separator
	win_new_width(wp, new_size);
	if (before)
	    wp->w_vsep_width = 1;
	else
	{
	    wp->w_vsep_width = oldwin->w_vsep_width;
	    oldwin->w_vsep_width = 1;
	}
	if (flags & (WSP_TOP | WSP_BOT))
	{
	    if (flags & WSP_BOT)
		frame_add_vsep(curfrp);
	    // Set width of neighbor frame
	    frame_new_width(curfrp, curfrp->fr_width
		     - (new_size + ((flags & WSP_TOP) != 0)), flags & WSP_TOP,
								       FALSE);
	}
	else
	    win_new_width(oldwin, oldwin->w_width - (new_size + 1));
	if (before)	// new window left of current one
	{
	    wp->w_wincol = oldwin->w_wincol;
	    oldwin->w_wincol += new_size + 1;
	}
	else		// new window right of current one
	    wp->w_wincol = oldwin->w_wincol + oldwin->w_width + 1;
	frame_fix_width(oldwin);
	frame_fix_width(wp);
    }
    else
    {
	// width and column of new window is same as current window
	if (flags & (WSP_TOP | WSP_BOT))
	{
	    wp->w_wincol = 0;
	    win_new_width(wp, Columns);
	    wp->w_vsep_width = 0;
	}
	else
	{
	    wp->w_wincol = oldwin->w_wincol;
	    win_new_width(wp, oldwin->w_width);
	    wp->w_vsep_width = oldwin->w_vsep_width;
	}
	frp->fr_width = curfrp->fr_width;

	// "new_size" of the current window goes to the new window, use
	// one row for the status line
	win_new_height(wp, new_size);
	int old_status_height = oldwin->w_status_height;
	if (flags & (WSP_TOP | WSP_BOT))
	{
	    int new_fr_height = curfrp->fr_height - new_size
							  + WINBAR_HEIGHT(wp) ;

	    if (!((flags & WSP_BOT) && p_ls == 0))
		new_fr_height -= STATUS_HEIGHT;
	    if (flags & WSP_BOT)
		frame_add_statusline(curfrp);
	    frame_new_height(curfrp, new_fr_height, flags & WSP_TOP, FALSE);
	}
	else
	    win_new_height(oldwin, oldwin_height - (new_size + STATUS_HEIGHT));
	if (before)	// new window above current one
	{
	    wp->w_winrow = oldwin->w_winrow;
	    wp->w_status_height = STATUS_HEIGHT;
	    oldwin->w_winrow += wp->w_height + STATUS_HEIGHT;
	}
	else		// new window below current one
	{
	    wp->w_winrow = oldwin->w_winrow + VISIBLE_HEIGHT(oldwin)
							       + STATUS_HEIGHT;
	    wp->w_status_height = old_status_height;
	    if (!(flags & WSP_BOT))
		oldwin->w_status_height = STATUS_HEIGHT;
	}
	frame_fix_height(wp);
	frame_fix_height(oldwin);
    }

    if (flags & (WSP_TOP | WSP_BOT))
	(void)win_comp_pos();

     // Both windows need redrawing.  Update all status lines, in case they
     // show something related to the window count or position.
    redraw_win_later(wp, UPD_NOT_VALID);
    redraw_win_later(oldwin, UPD_NOT_VALID);
    status_redraw_all();

    if (need_status)
    {
	msg_row = Rows - 1;
	msg_col = sc_col;
	msg_clr_eos_force();	// Old command/ruler may still be there
	comp_col();
	msg_row = Rows - 1;
	msg_col = 0;	// put position back at start of line
    }

    /*
     * equalize the window sizes.
     */
    if (do_equal || dir != 0)
	win_equal(wp, TRUE,
		(flags & WSP_VERT) ? (dir == 'v' ? 'b' : 'h')
		: dir == 'h' ? 'b' : 'v');
    else if (!is_aucmd_win(wp))
	win_fix_scroll(FALSE);

    // Don't change the window height/width to 'winheight' / 'winwidth' if a
    // size was given.
    if (flags & WSP_VERT)
    {
	i = p_wiw;
	if (size != 0)
	    p_wiw = size;

# ifdef FEAT_GUI
	// When 'guioptions' includes 'L' or 'R' may have to add scrollbars.
	if (gui.in_use)
	    gui_init_which_components(NULL);
# endif
    }
    else
    {
	i = p_wh;
	if (size != 0)
	    p_wh = size;
    }

    /*
     * make the new window the current window
     */
    (void)win_enter_ext(wp, WEE_TRIGGER_NEW_AUTOCMDS
		    | WEE_TRIGGER_ENTER_AUTOCMDS | WEE_TRIGGER_LEAVE_AUTOCMDS);
    if (flags & WSP_VERT)
	p_wiw = i;
    else
	p_wh = i;
    retval = OK;

theend:
    if (RedrawingDisabled > 0)
	--RedrawingDisabled;
    return retval;
}


/*
 * Initialize window "newp" from window "oldp".
 * Used when splitting a window and when creating a new tab page.
 * The windows will both edit the same buffer.
 * WSP_NEWLOC may be specified in flags to prevent the location list from
 * being copied.
 */
    static void
win_init(win_T *newp, win_T *oldp, int flags UNUSED)
{
    int		i;

    newp->w_buffer = oldp->w_buffer;
#ifdef FEAT_SYN_HL
    newp->w_s = &(oldp->w_buffer->b_s);
#endif
    oldp->w_buffer->b_nwindows++;
    newp->w_cursor = oldp->w_cursor;
    newp->w_valid = 0;
    newp->w_curswant = oldp->w_curswant;
    newp->w_set_curswant = oldp->w_set_curswant;
    newp->w_topline = oldp->w_topline;
#ifdef FEAT_DIFF
    newp->w_topfill = oldp->w_topfill;
#endif
    newp->w_leftcol = oldp->w_leftcol;
    newp->w_pcmark = oldp->w_pcmark;
    newp->w_prev_pcmark = oldp->w_prev_pcmark;
    newp->w_alt_fnum = oldp->w_alt_fnum;
    newp->w_wrow = oldp->w_wrow;
    newp->w_fraction = oldp->w_fraction;
    newp->w_prev_fraction_row = oldp->w_prev_fraction_row;
    copy_jumplist(oldp, newp);
#ifdef FEAT_QUICKFIX
    if (flags & WSP_NEWLOC)
    {
	// Don't copy the location list.
	newp->w_llist = NULL;
	newp->w_llist_ref = NULL;
    }
    else
	copy_loclist_stack(oldp, newp);
#endif
    newp->w_localdir = (oldp->w_localdir == NULL)
				    ? NULL : vim_strsave(oldp->w_localdir);
    newp->w_prevdir = (oldp->w_prevdir == NULL)
				    ? NULL : vim_strsave(oldp->w_prevdir);

    if (*p_spk != 'c')
    {
	if (*p_spk == 't')
	    newp->w_skipcol = oldp->w_skipcol;
	newp->w_botline = oldp->w_botline;
	newp->w_prev_height = oldp->w_height - WINBAR_HEIGHT(oldp);
	newp->w_prev_winrow = oldp->w_winrow + 2 * WINBAR_HEIGHT(oldp);
    }

    // copy tagstack and folds
    for (i = 0; i < oldp->w_tagstacklen; i++)
    {
	taggy_T	*tag = &newp->w_tagstack[i];
	*tag = oldp->w_tagstack[i];
	if (tag->tagname != NULL)
	    tag->tagname = vim_strsave(tag->tagname);
	if (tag->user_data != NULL)
	    tag->user_data = vim_strsave(tag->user_data);
    }
    newp->w_tagstackidx = oldp->w_tagstackidx;
    newp->w_tagstacklen = oldp->w_tagstacklen;

    // Keep same changelist position in new window.
    newp->w_changelistidx = oldp->w_changelistidx;

#ifdef FEAT_FOLDING
    copyFoldingState(oldp, newp);
#endif

    win_init_some(newp, oldp);
#ifdef FEAT_TERMINAL
    term_update_wincolor(newp);
#endif
}

/*
 * Initialize window "newp" from window "old".
 * Only the essential things are copied.
 */
    static void
win_init_some(win_T *newp, win_T *oldp)
{
    // Use the same argument list.
    newp->w_alist = oldp->w_alist;
    ++newp->w_alist->al_refcount;
    newp->w_arg_idx = oldp->w_arg_idx;

    // copy options from existing window
    win_copy_options(oldp, newp);
}

/*
 * Return TRUE if "win" is a global popup or a popup in the current tab page.
 */
    int
win_valid_popup(win_T *win UNUSED)
{
#ifdef FEAT_PROP_POPUP
    win_T	*wp;

    FOR_ALL_POPUPWINS(wp)
	if (wp == win)
	    return TRUE;
    FOR_ALL_POPUPWINS_IN_TAB(curtab, wp)
	if (wp == win)
	    return TRUE;
#endif
    return FALSE;
}

/*
 * Check if "win" is a pointer to an existing window in the current tab page.
 */
    int
win_valid(win_T *win)
{
    win_T	*wp;

    if (win == NULL)
	return FALSE;
    FOR_ALL_WINDOWS(wp)
	if (wp == win)
	    return TRUE;
    return win_valid_popup(win);
}

/*
 * Find window "id" in the current tab page.
 * Also find popup windows.
 * Return NULL if not found.
 */
    win_T *
win_find_by_id(int id)
{
    win_T   *wp;

    FOR_ALL_WINDOWS(wp)
	if (wp->w_id == id)
	    return wp;
#ifdef FEAT_PROP_POPUP
    FOR_ALL_POPUPWINS(wp)
	if (wp->w_id == id)
	    return wp;
    FOR_ALL_POPUPWINS_IN_TAB(curtab, wp)
	if (wp->w_id == id)
	    return wp;
#endif
    return NULL;
}

/*
 * Check if "win" is a pointer to an existing window in any tab page.
 */
    int
win_valid_any_tab(win_T *win)
{
    win_T	*wp;
    tabpage_T	*tp;

    if (win == NULL)
	return FALSE;
    FOR_ALL_TABPAGES(tp)
    {
	FOR_ALL_WINDOWS_IN_TAB(tp, wp)
	{
	    if (wp == win)
		return TRUE;
	}
#ifdef FEAT_PROP_POPUP
	FOR_ALL_POPUPWINS_IN_TAB(tp, wp)
	    if (wp == win)
		return TRUE;
#endif
    }
    return win_valid_popup(win);
}

/*
 * Return the number of windows.
 */
    int
win_count(void)
{
    win_T	*wp;
    int		count = 0;

    FOR_ALL_WINDOWS(wp)
	++count;
    return count;
}

/*
 * Make "count" windows on the screen.
 * Return actual number of windows on the screen.
 * Must be called when there is just one window, filling the whole screen
 * (excluding the command line).
 */
    int
make_windows(
    int		count,
    int		vertical UNUSED)  // split windows vertically if TRUE
{
    int		maxcount;
    int		todo;

    if (vertical)
    {
	// Each window needs at least 'winminwidth' lines and a separator
	// column.
	maxcount = (curwin->w_width + curwin->w_vsep_width
					     - (p_wiw - p_wmw)) / (p_wmw + 1);
    }
    else
    {
	// Each window needs at least 'winminheight' lines and a status line.
	maxcount = (VISIBLE_HEIGHT(curwin) + curwin->w_status_height
				  - (p_wh - p_wmh)) / (p_wmh + STATUS_HEIGHT);
    }

    if (maxcount < 2)
	maxcount = 2;
    if (count > maxcount)
	count = maxcount;

    /*
     * add status line now, otherwise first window will be too big
     */
    if (count > 1)
	last_status(TRUE);

    /*
     * Don't execute autocommands while creating the windows.  Must do that
     * when putting the buffers in the windows.
     */
    block_autocmds();

    // todo is number of windows left to create
    for (todo = count - 1; todo > 0; --todo)
	if (vertical)
	{
	    if (win_split(curwin->w_width - (curwin->w_width - todo)
			/ (todo + 1) - 1, WSP_VERT | WSP_ABOVE) == FAIL)
		break;
	}
	else
	{
	    if (win_split(curwin->w_height - (curwin->w_height - todo
			    * STATUS_HEIGHT) / (todo + 1)
			- STATUS_HEIGHT, WSP_ABOVE) == FAIL)
		break;
	}

    unblock_autocmds();

    // return actual number of windows
    return (count - todo);
}

/*
 * Exchange current and next window
 */
    static void
win_exchange(long Prenum)
{
    frame_T	*frp;
    frame_T	*frp2;
    win_T	*wp;
    win_T	*wp2;
    int		temp;

    if (ERROR_IF_ANY_POPUP_WINDOW)
	return;
    if (ONE_WINDOW)	    // just one window
    {
	beep_flush();
	return;
    }
    if (text_or_buf_locked())
    {
	beep_flush();
	return;
    }

#ifdef FEAT_GUI
    need_mouse_correct = TRUE;
#endif

    /*
     * find window to exchange with
     */
    if (Prenum)
    {
	frp = curwin->w_frame->fr_parent->fr_child;
	while (frp != NULL && --Prenum > 0)
	    frp = frp->fr_next;
    }
    else if (curwin->w_frame->fr_next != NULL)	// Swap with next
	frp = curwin->w_frame->fr_next;
    else    // Swap last window in row/col with previous
	frp = curwin->w_frame->fr_prev;

    // We can only exchange a window with another window, not with a frame
    // containing windows.
    if (frp == NULL || frp->fr_win == NULL || frp->fr_win == curwin)
	return;
    wp = frp->fr_win;

/*
 * 1. remove curwin from the list. Remember after which window it was in wp2
 * 2. insert curwin before wp in the list
 * if wp != wp2
 *    3. remove wp from the list
 *    4. insert wp after wp2
 * 5. exchange the status line height and vsep width.
 */
    wp2 = curwin->w_prev;
    frp2 = curwin->w_frame->fr_prev;
    if (wp->w_prev != curwin)
    {
	win_remove(curwin, NULL);
	frame_remove(curwin->w_frame);
	win_append(wp->w_prev, curwin);
	frame_insert(frp, curwin->w_frame);
    }
    if (wp != wp2)
    {
	win_remove(wp, NULL);
	frame_remove(wp->w_frame);
	win_append(wp2, wp);
	if (frp2 == NULL)
	    frame_insert(wp->w_frame->fr_parent->fr_child, wp->w_frame);
	else
	    frame_append(frp2, wp->w_frame);
    }
    temp = curwin->w_status_height;
    curwin->w_status_height = wp->w_status_height;
    wp->w_status_height = temp;
    temp = curwin->w_vsep_width;
    curwin->w_vsep_width = wp->w_vsep_width;
    wp->w_vsep_width = temp;

    frame_fix_height(curwin);
    frame_fix_height(wp);
    frame_fix_width(curwin);
    frame_fix_width(wp);

    (void)win_comp_pos();		// recompute window positions

    if (wp->w_buffer != curbuf)
	reset_VIsual_and_resel();
    else if (VIsual_active)
	wp->w_cursor = curwin->w_cursor;

    win_enter(wp, TRUE);
    redraw_all_later(UPD_NOT_VALID);
}

/*
 * rotate windows: if upwards TRUE the second window becomes the first one
 *		   if upwards FALSE the first window becomes the second one
 */
    static void
win_rotate(int upwards, int count)
{
    win_T	*wp1;
    win_T	*wp2;
    frame_T	*frp;
    int		n;

    if (ONE_WINDOW)		// nothing to do
    {
	beep_flush();
	return;
    }

#ifdef FEAT_GUI
    need_mouse_correct = TRUE;
#endif

    // Check if all frames in this row/col have one window.
    FOR_ALL_FRAMES(frp, curwin->w_frame->fr_parent->fr_child)
	if (frp->fr_win == NULL)
	{
	    emsg(_(e_cannot_rotate_when_another_window_is_split));
	    return;
	}

    while (count--)
    {
	if (upwards)		// first window becomes last window
	{
	    // remove first window/frame from the list
	    frp = curwin->w_frame->fr_parent->fr_child;
	    wp1 = frp->fr_win;
	    win_remove(wp1, NULL);
	    frame_remove(frp);

	    // find last frame and append removed window/frame after it
	    for ( ; frp->fr_next != NULL; frp = frp->fr_next)
		;
	    win_append(frp->fr_win, wp1);
	    frame_append(frp, wp1->w_frame);

	    wp2 = frp->fr_win;		// previously last window
	}
	else			// last window becomes first window
	{
	    // find last window/frame in the list and remove it
	    for (frp = curwin->w_frame; frp->fr_next != NULL;
							   frp = frp->fr_next)
		;
	    wp1 = frp->fr_win;
	    wp2 = wp1->w_prev;		    // will become last window
	    win_remove(wp1, NULL);
	    frame_remove(frp);

	    // append the removed window/frame before the first in the list
	    win_append(frp->fr_parent->fr_child->fr_win->w_prev, wp1);
	    frame_insert(frp->fr_parent->fr_child, frp);
	}

	// exchange status height and vsep width of old and new last window
	n = wp2->w_status_height;
	wp2->w_status_height = wp1->w_status_height;
	wp1->w_status_height = n;
	frame_fix_height(wp1);
	frame_fix_height(wp2);
	n = wp2->w_vsep_width;
	wp2->w_vsep_width = wp1->w_vsep_width;
	wp1->w_vsep_width = n;
	frame_fix_width(wp1);
	frame_fix_width(wp2);

	// recompute w_winrow and w_wincol for all windows
	(void)win_comp_pos();
    }

    redraw_all_later(UPD_NOT_VALID);
}

/*
 * Move the current window to the very top/bottom/left/right of the screen.
 */
    static void
win_totop(int size, int flags)
{
    int		dir;
    int		height = curwin->w_height;

    if (ONE_WINDOW)
    {
	beep_flush();
	return;
    }
    if (check_split_disallowed() == FAIL)
	return;

    // Remove the window and frame from the tree of frames.
    (void)winframe_remove(curwin, &dir, NULL);
    win_remove(curwin, NULL);
    last_status(FALSE);	    // may need to remove last status line
    (void)win_comp_pos();   // recompute window positions

    // Split a window on the desired side and put the window there.
    (void)win_split_ins(size, flags, curwin, dir);
    if (!(flags & WSP_VERT))
    {
	win_setheight(height);
	if (p_ea)
	    win_equal(curwin, TRUE, 'v');
    }

#if defined(FEAT_GUI)
    // When 'guioptions' includes 'L' or 'R' may have to remove or add
    // scrollbars.  Have to update them anyway.
    gui_may_update_scrollbars();
#endif
}

/*
 * Move window "win1" to below/right of "win2" and make "win1" the current
 * window.  Only works within the same frame!
 */
    void
win_move_after(win_T *win1, win_T *win2)
{
    int		height;

    // check if the arguments are reasonable
    if (win1 == win2)
	return;

    // check if there is something to do
    if (win2->w_next != win1)
    {
	if (win1->w_frame->fr_parent != win2->w_frame->fr_parent)
	{
	    iemsg("Trying to move a window into another frame");
	    return;
	}

	// may need to move the status line/vertical separator of the last
	// window
	if (win1 == lastwin)
	{
	    height = win1->w_prev->w_status_height;
	    win1->w_prev->w_status_height = win1->w_status_height;
	    win1->w_status_height = height;
	    if (win1->w_prev->w_vsep_width == 1)
	    {
		// Remove the vertical separator from the last-but-one window,
		// add it to the last window.  Adjust the frame widths.
		win1->w_prev->w_vsep_width = 0;
		win1->w_prev->w_frame->fr_width -= 1;
		win1->w_vsep_width = 1;
		win1->w_frame->fr_width += 1;
	    }
	}
	else if (win2 == lastwin)
	{
	    height = win1->w_status_height;
	    win1->w_status_height = win2->w_status_height;
	    win2->w_status_height = height;
	    if (win1->w_vsep_width == 1)
	    {
		// Remove the vertical separator from win1, add it to the last
		// window, win2.  Adjust the frame widths.
		win2->w_vsep_width = 1;
		win2->w_frame->fr_width += 1;
		win1->w_vsep_width = 0;
		win1->w_frame->fr_width -= 1;
	    }
	}
	win_remove(win1, NULL);
	frame_remove(win1->w_frame);
	win_append(win2, win1);
	frame_append(win2->w_frame, win1->w_frame);

	(void)win_comp_pos();	// recompute w_winrow for all windows
	redraw_later(UPD_NOT_VALID);
    }
    win_enter(win1, FALSE);
}

/*
 * Make all windows the same height.
 * 'next_curwin' will soon be the current window, make sure it has enough
 * rows.
 */
    void
win_equal(
    win_T	*next_curwin,	// pointer to current window to be or NULL
    int		current,	// do only frame with current window
    int		dir)		// 'v' for vertically, 'h' for horizontally,
				// 'b' for both, 0 for using p_ead
{
    if (dir == 0)
	dir = *p_ead;
    win_equal_rec(next_curwin == NULL ? curwin : next_curwin, current,
		      topframe, dir, 0, tabline_height(),
					   (int)Columns, topframe->fr_height);
    if (!is_aucmd_win(next_curwin))
	win_fix_scroll(TRUE);
}

/*
 * Set a frame to a new position and height, spreading the available room
 * equally over contained frames.
 * The window "next_curwin" (if not NULL) should at least get the size from
 * 'winheight' and 'winwidth' if possible.
 */
    static void
win_equal_rec(
    win_T	*next_curwin,	// pointer to current window to be or NULL
    int		current,	// do only frame with current window
    frame_T	*topfr,		// frame to set size off
    int		dir,		// 'v', 'h' or 'b', see win_equal()
    int		col,		// horizontal position for frame
    int		row,		// vertical position for frame
    int		width,		// new width of frame
    int		height)		// new height of frame
{
    int		n, m;
    int		extra_sep = 0;
    int		wincount, totwincount = 0;
    frame_T	*fr;
    int		next_curwin_size = 0;
    int		room = 0;
    int		new_size;
    int		has_next_curwin = 0;
    int		hnc;

    if (topfr->fr_layout == FR_LEAF)
    {
	// Set the width/height of this frame.
	// Redraw when size or position changes
	if (topfr->fr_height != height || topfr->fr_win->w_winrow != row
		|| topfr->fr_width != width || topfr->fr_win->w_wincol != col
	   )
	{
	    topfr->fr_win->w_winrow = row;
	    frame_new_height(topfr, height, FALSE, FALSE);
	    topfr->fr_win->w_wincol = col;
	    frame_new_width(topfr, width, FALSE, FALSE);
	    redraw_all_later(UPD_NOT_VALID);
	}
    }
    else if (topfr->fr_layout == FR_ROW)
    {
	topfr->fr_width = width;
	topfr->fr_height = height;

	if (dir != 'v')			// equalize frame widths
	{
	    // Compute the maximum number of windows horizontally in this
	    // frame.
	    n = frame_minwidth(topfr, NOWIN);
	    // add one for the rightmost window, it doesn't have a separator
	    if (col + width == Columns)
		extra_sep = 1;
	    else
		extra_sep = 0;
	    totwincount = (n + extra_sep) / (p_wmw + 1);
	    has_next_curwin = frame_has_win(topfr, next_curwin);

	    /*
	     * Compute width for "next_curwin" window and room available for
	     * other windows.
	     * "m" is the minimal width when counting p_wiw for "next_curwin".
	     */
	    m = frame_minwidth(topfr, next_curwin);
	    room = width - m;
	    if (room < 0)
	    {
		next_curwin_size = p_wiw + room;
		room = 0;
	    }
	    else
	    {
		next_curwin_size = -1;
		FOR_ALL_FRAMES(fr, topfr->fr_child)
		{
		    if (!frame_fixed_width(fr))
			continue;
		    // If 'winfixwidth' set keep the window width if possible.
		    // Watch out for this window being the next_curwin.
		    n = frame_minwidth(fr, NOWIN);
		    new_size = fr->fr_width;
		    if (frame_has_win(fr, next_curwin))
		    {
			room += p_wiw - p_wmw;
			next_curwin_size = 0;
			if (new_size < p_wiw)
			    new_size = p_wiw;
		    }
		    else
			// These windows don't use up room.
			totwincount -= (n + (fr->fr_next == NULL
					       ? extra_sep : 0)) / (p_wmw + 1);
		    room -= new_size - n;
		    if (room < 0)
		    {
			new_size += room;
			room = 0;
		    }
		    fr->fr_newwidth = new_size;
		}
		if (next_curwin_size == -1)
		{
		    if (!has_next_curwin)
			next_curwin_size = 0;
		    else if (totwincount > 1
			    && (room + (totwincount - 2))
						  / (totwincount - 1) > p_wiw)
		    {
			// Can make all windows wider than 'winwidth', spread
			// the room equally.
			next_curwin_size = (room + p_wiw
					    + (totwincount - 1) * p_wmw
					    + (totwincount - 1)) / totwincount;
			room -= next_curwin_size - p_wiw;
		    }
		    else
			next_curwin_size = p_wiw;
		}
	    }

	    if (has_next_curwin)
		--totwincount;		// don't count curwin
	}

	FOR_ALL_FRAMES(fr, topfr->fr_child)
	{
	    wincount = 1;
	    if (fr->fr_next == NULL)
		// last frame gets all that remains (avoid roundoff error)
		new_size = width;
	    else if (dir == 'v')
		new_size = fr->fr_width;
	    else if (frame_fixed_width(fr))
	    {
		new_size = fr->fr_newwidth;
		wincount = 0;	    // doesn't count as a sizeable window
	    }
	    else
	    {
		// Compute the maximum number of windows horiz. in "fr".
		n = frame_minwidth(fr, NOWIN);
		wincount = (n + (fr->fr_next == NULL ? extra_sep : 0))
								/ (p_wmw + 1);
		m = frame_minwidth(fr, next_curwin);
		if (has_next_curwin)
		    hnc = frame_has_win(fr, next_curwin);
		else
		    hnc = FALSE;
		if (hnc)	    // don't count next_curwin
		    --wincount;
		if (totwincount == 0)
		    new_size = room;
		else
		    new_size = (wincount * room + ((unsigned)totwincount >> 1))
								/ totwincount;
		if (hnc)	    // add next_curwin size
		{
		    next_curwin_size -= p_wiw - (m - n);
		    if (next_curwin_size < 0)
			next_curwin_size = 0;
		    new_size += next_curwin_size;
		    room -= new_size - next_curwin_size;
		}
		else
		    room -= new_size;
		new_size += n;
	    }

	    // Skip frame that is full width when splitting or closing a
	    // window, unless equalizing all frames.
	    if (!current || dir != 'v' || topfr->fr_parent != NULL
		    || (new_size != fr->fr_width)
		    || frame_has_win(fr, next_curwin))
		win_equal_rec(next_curwin, current, fr, dir, col, row,
							    new_size, height);
	    col += new_size;
	    width -= new_size;
	    totwincount -= wincount;
	}
    }
    else // topfr->fr_layout == FR_COL
    {
	topfr->fr_width = width;
	topfr->fr_height = height;

	if (dir != 'h')			// equalize frame heights
	{
	    // Compute maximum number of windows vertically in this frame.
	    n = frame_minheight(topfr, NOWIN);
	    // add one for the bottom window if it doesn't have a statusline
	    if (row + height == cmdline_row && p_ls == 0)
		extra_sep = 1;
	    else
		extra_sep = 0;
	    totwincount = (n + extra_sep) / (p_wmh + 1);
	    has_next_curwin = frame_has_win(topfr, next_curwin);

	    /*
	     * Compute height for "next_curwin" window and room available for
	     * other windows.
	     * "m" is the minimal height when counting p_wh for "next_curwin".
	     */
	    m = frame_minheight(topfr, next_curwin);
	    room = height - m;
	    if (room < 0)
	    {
		// The room is less than 'winheight', use all space for the
		// current window.
		next_curwin_size = p_wh + room;
		room = 0;
	    }
	    else
	    {
		next_curwin_size = -1;
		FOR_ALL_FRAMES(fr, topfr->fr_child)
		{
		    if (!frame_fixed_height(fr))
			continue;
		    // If 'winfixheight' set keep the window height if
		    // possible.
		    // Watch out for this window being the next_curwin.
		    n = frame_minheight(fr, NOWIN);
		    new_size = fr->fr_height;
		    if (frame_has_win(fr, next_curwin))
		    {
			room += p_wh - p_wmh;
			next_curwin_size = 0;
			if (new_size < p_wh)
			    new_size = p_wh;
		    }
		    else
			// These windows don't use up room.
			totwincount -= (n + (fr->fr_next == NULL
					       ? extra_sep : 0)) / (p_wmh + 1);
		    room -= new_size - n;
		    if (room < 0)
		    {
			new_size += room;
			room = 0;
		    }
		    fr->fr_newheight = new_size;
		}
		if (next_curwin_size == -1)
		{
		    if (!has_next_curwin)
			next_curwin_size = 0;
		    else if (totwincount > 1
			    && (room + (totwincount - 2))
						   / (totwincount - 1) > p_wh)
		    {
			// can make all windows higher than 'winheight',
			// spread the room equally.
			next_curwin_size = (room + p_wh
					   + (totwincount - 1) * p_wmh
					   + (totwincount - 1)) / totwincount;
			room -= next_curwin_size - p_wh;
		    }
		    else
			next_curwin_size = p_wh;
		}
	    }

	    if (has_next_curwin)
		--totwincount;		// don't count curwin
	}

	FOR_ALL_FRAMES(fr, topfr->fr_child)
	{
	    wincount = 1;
	    if (fr->fr_next == NULL)
		// last frame gets all that remains (avoid roundoff error)
		new_size = height;
	    else if (dir == 'h')
		new_size = fr->fr_height;
	    else if (frame_fixed_height(fr))
	    {
		new_size = fr->fr_newheight;
		wincount = 0;	    // doesn't count as a sizeable window
	    }
	    else
	    {
		// Compute the maximum number of windows vert. in "fr".
		n = frame_minheight(fr, NOWIN);
		wincount = (n + (fr->fr_next == NULL ? extra_sep : 0))
								/ (p_wmh + 1);
		m = frame_minheight(fr, next_curwin);
		if (has_next_curwin)
		    hnc = frame_has_win(fr, next_curwin);
		else
		    hnc = FALSE;
		if (hnc)	    // don't count next_curwin
		    --wincount;
		if (totwincount == 0)
		    new_size = room;
		else
		    new_size = (wincount * room + ((unsigned)totwincount >> 1))
								/ totwincount;
		if (hnc)	    // add next_curwin size
		{
		    next_curwin_size -= p_wh - (m - n);
		    new_size += next_curwin_size;
		    room -= new_size - next_curwin_size;
		}
		else
		    room -= new_size;
		new_size += n;
	    }
	    // Skip frame that is full width when splitting or closing a
	    // window, unless equalizing all frames.
	    if (!current || dir != 'h' || topfr->fr_parent != NULL
		    || (new_size != fr->fr_height)
		    || frame_has_win(fr, next_curwin))
		win_equal_rec(next_curwin, current, fr, dir, col, row,
							     width, new_size);
	    row += new_size;
	    height -= new_size;
	    totwincount -= wincount;
	}
    }
}

#ifdef FEAT_JOB_CHANNEL
    void
leaving_window(win_T *win)
{
    // Only matters for a prompt window.
    if (!bt_prompt(win->w_buffer))
	return;

    // When leaving a prompt window stop Insert mode and perhaps restart
    // it when entering that window again.
    win->w_buffer->b_prompt_insert = restart_edit;
    if (restart_edit != 0 && mode_displayed)
	clear_cmdline = TRUE;		// unshow mode later
    restart_edit = NUL;

    // When leaving the window (or closing the window) was done from a
    // callback we need to break out of the Insert mode loop and restart Insert
    // mode when entering the window again.
    if ((State & MODE_INSERT) && !stop_insert_mode)
    {
	stop_insert_mode = TRUE;
	if (win->w_buffer->b_prompt_insert == NUL)
	    win->w_buffer->b_prompt_insert = 'A';
    }
}

    void
entering_window(win_T *win)
{
    // Only matters for a prompt window.
    if (!bt_prompt(win->w_buffer))
	return;

    // When switching to a prompt buffer that was in Insert mode, don't stop
    // Insert mode, it may have been set in leaving_window().
    if (win->w_buffer->b_prompt_insert != NUL)
	stop_insert_mode = FALSE;

    // When entering the prompt window restart Insert mode if we were in Insert
    // mode when we left it and not already in Insert mode.
    if ((State & MODE_INSERT) == 0)
	restart_edit = win->w_buffer->b_prompt_insert;
}
#endif

    static void
win_init_empty(win_T *wp)
{
    redraw_win_later(wp, UPD_NOT_VALID);
    wp->w_lines_valid = 0;
    wp->w_cursor.lnum = 1;
    wp->w_curswant = wp->w_cursor.col = 0;
    wp->w_cursor.coladd = 0;
    wp->w_pcmark.lnum = 1;	// pcmark not cleared but set to line 1
    wp->w_pcmark.col = 0;
    wp->w_prev_pcmark.lnum = 0;
    wp->w_prev_pcmark.col = 0;
    wp->w_topline = 1;
#ifdef FEAT_DIFF
    wp->w_topfill = 0;
#endif
    wp->w_botline = 2;
#if defined(FEAT_SYN_HL) || defined(FEAT_SPELL)
    wp->w_s = &wp->w_buffer->b_s;
#endif
#ifdef FEAT_TERMINAL
    term_reset_wincolor(wp);
#endif
}

/*
 * Init the current window "curwin".
 * Called when a new file is being edited.
 */
    void
curwin_init(void)
{
    win_init_empty(curwin);
}

/*
 * Close all windows for buffer "buf".
 */
    void
close_windows(
    buf_T	*buf,
    int		keep_curwin)	    // don't close "curwin"
{
    win_T	*wp;
    tabpage_T   *tp, *nexttp;
    int		count = tabpage_index(NULL);

    ++RedrawingDisabled;

    for (wp = firstwin; wp != NULL && !ONE_WINDOW; )
    {
	if (wp->w_buffer == buf && (!keep_curwin || wp != curwin)
		&& !(wp->w_closing || wp->w_buffer->b_locked > 0))
	{
	    if (win_close(wp, FALSE) == FAIL)
		// If closing the window fails give up, to avoid looping
		// forever.
		break;

	    // Start all over, autocommands may change the window layout.
	    wp = firstwin;
	}
	else
	    wp = wp->w_next;
    }

    // Also check windows in other tab pages.
    for (tp = first_tabpage; tp != NULL; tp = nexttp)
    {
	nexttp = tp->tp_next;
	if (tp != curtab)
	    FOR_ALL_WINDOWS_IN_TAB(tp, wp)
		if (wp->w_buffer == buf
		    && !(wp->w_closing || wp->w_buffer->b_locked > 0))
		{
		    win_close_othertab(wp, FALSE, tp);

		    // Start all over, the tab page may be closed and
		    // autocommands may change the window layout.
		    nexttp = first_tabpage;
		    break;
		}
    }

    if (RedrawingDisabled > 0)
	--RedrawingDisabled;

    if (count != tabpage_index(NULL))
	apply_autocmds(EVENT_TABCLOSED, NULL, NULL, FALSE, curbuf);
}

/*
 * Return TRUE if the current window is the only window that exists (ignoring
 * "aucmd_win[]").
 * Returns FALSE if there is a window, possibly in another tab page.
 */
    int
last_window(void)
{
    return (one_window() && first_tabpage->tp_next == NULL);
}

/*
 * Return TRUE if there is only one window other than "aucmd_win[]" in the
 * current tab page.
 */
    int
one_window(void)
{
    win_T	*wp;
    int		seen_one = FALSE;

    FOR_ALL_WINDOWS(wp)
    {
	if (!is_aucmd_win(wp))
	{
	    if (seen_one)
		return FALSE;
	    seen_one = TRUE;
	}
    }
    return TRUE;
}

/*
 * Close the possibly last window in a tab page.
 * Returns TRUE when the window was closed already.
 */
    static int
close_last_window_tabpage(
    win_T	*win,
    int		free_buf,
    tabpage_T   *prev_curtab)
{
    if (!ONE_WINDOW)
	return FALSE;

    buf_T	*old_curbuf = curbuf;

    /*
     * Closing the last window in a tab page.  First go to another tab
     * page and then close the window and the tab page.  This avoids that
     * curwin and curtab are invalid while we are freeing memory, they may
     * be used in GUI events.
     * Don't trigger autocommands yet, they may use wrong values, so do
     * that below.
     */
    goto_tabpage_tp(alt_tabpage(), FALSE, TRUE);

    // Safety check: Autocommands may have closed the window when jumping
    // to the other tab page.
    if (valid_tabpage(prev_curtab) && prev_curtab->tp_firstwin == win)
	win_close_othertab(win, free_buf, prev_curtab);
#ifdef FEAT_JOB_CHANNEL
    entering_window(curwin);
#endif
    // Since goto_tabpage_tp above did not trigger *Enter autocommands, do
    // that now.
    apply_autocmds(EVENT_TABCLOSED, NULL, NULL, FALSE, curbuf);
    apply_autocmds(EVENT_WINENTER, NULL, NULL, FALSE, curbuf);
    apply_autocmds(EVENT_TABENTER, NULL, NULL, FALSE, curbuf);
    if (old_curbuf != curbuf)
	apply_autocmds(EVENT_BUFENTER, NULL, NULL, FALSE, curbuf);
    return TRUE;
}

/*
 * Close the buffer of "win" and unload it if "action" is DOBUF_UNLOAD.
 * "action" can also be zero (do nothing) or DOBUF_WIPE.
 * "abort_if_last" is passed to close_buffer(): abort closing if all other
 * windows are closed.
 */
    static void
win_close_buffer(win_T *win, int action, int abort_if_last)
{
#ifdef FEAT_SYN_HL
    // Free independent synblock before the buffer is freed.
    if (win->w_buffer != NULL)
	reset_synblock(win);
#endif

#ifdef FEAT_QUICKFIX
    // When a quickfix/location list window is closed and the buffer is
    // displayed in only one window, then unlist the buffer.
    if (win->w_buffer != NULL && bt_quickfix(win->w_buffer)
					&& win->w_buffer->b_nwindows == 1)
	win->w_buffer->b_p_bl = FALSE;
#endif

    // Close the link to the buffer.
    if (win->w_buffer != NULL)
    {
	bufref_T    bufref;

	set_bufref(&bufref, curbuf);
	win->w_closing = TRUE;
	close_buffer(win, win->w_buffer, action, abort_if_last, TRUE);
	if (win_valid_any_tab(win))
	    win->w_closing = FALSE;
	// Make sure curbuf is valid. It can become invalid if 'bufhidden' is
	// "wipe".
	if (!bufref_valid(&bufref))
	    curbuf = firstbuf;
    }
}

/*
 * Close window "win".  Only works for the current tab page.
 * If "free_buf" is TRUE related buffer may be unloaded.
 *
 * Called by :quit, :close, :xit, :wq and findtag().
 * Returns FAIL when the window was not closed.
 */
    int
win_close(win_T *win, int free_buf)
{
    win_T	*wp;
    int		other_buffer = FALSE;
    int		close_curwin = FALSE;
    int		dir;
    int		help_window = FALSE;
    tabpage_T   *prev_curtab = curtab;
    frame_T	*win_frame = win->w_frame->fr_parent;
#ifdef FEAT_DIFF
    int		had_diffmode = win->w_p_diff;
#endif
#ifdef MESSAGE_QUEUE
    int		did_decrement = FALSE;
#endif

#if defined(FEAT_TERMINAL) && defined(FEAT_PROP_POPUP)
    // Can close a popup window with a terminal if the job has finished.
    if (may_close_term_popup() == OK)
	return OK;
#endif
    if (ERROR_IF_ANY_POPUP_WINDOW)
	return FAIL;

    if (last_window())
    {
	emsg(_(e_cannot_close_last_window));
	return FAIL;
    }
    if (window_layout_locked(CMD_close))
	return FAIL;

    if (win->w_closing || (win->w_buffer != NULL
					       && win->w_buffer->b_locked > 0))
	return FAIL; // window is already being closed
    if (win_unlisted(win))
    {
	emsg(_(e_cannot_close_autocmd_or_popup_window));
	return FAIL;
    }
    if ((is_aucmd_win(firstwin) || is_aucmd_win(lastwin)) && one_window())
    {
	emsg(_(e_cannot_close_window_only_autocmd_window_would_remain));
	return FAIL;
    }

    // When closing the last window in a tab page first go to another tab page
    // and then close the window and the tab page to avoid that curwin and
    // curtab are invalid while we are freeing memory.
    if (close_last_window_tabpage(win, free_buf, prev_curtab))
      return FAIL;

    // When closing the help window, try restoring a snapshot after closing
    // the window.  Otherwise clear the snapshot, it's now invalid.
    if (bt_help(win->w_buffer))
	help_window = TRUE;
    else
	clear_snapshot(curtab, SNAP_HELP_IDX);

    if (win == curwin)
    {
#ifdef FEAT_JOB_CHANNEL
	leaving_window(curwin);
#endif
	/*
	 * Guess which window is going to be the new current window.
	 * This may change because of the autocommands (sigh).
	 */
	wp = frame2win(win_altframe(win, NULL));

	/*
	 * Be careful: If autocommands delete the window or cause this window
	 * to be the last one left, return now.
	 */
	if (wp->w_buffer != curbuf)
	{
	    reset_VIsual_and_resel();	// stop Visual mode

	    other_buffer = TRUE;
	    if (!win_valid(win))
		return FAIL;
	    win->w_closing = TRUE;
	    apply_autocmds(EVENT_BUFLEAVE, NULL, NULL, FALSE, curbuf);
	    if (!win_valid(win))
		return FAIL;
	    win->w_closing = FALSE;
	    if (last_window())
		return FAIL;
	}
	win->w_closing = TRUE;
	apply_autocmds(EVENT_WINLEAVE, NULL, NULL, FALSE, curbuf);
	if (!win_valid(win))
	    return FAIL;
	win->w_closing = FALSE;
	if (last_window())
	    return FAIL;
#ifdef FEAT_EVAL
	// autocmds may abort script processing
	if (aborting())
	    return FAIL;
#endif
    }

#ifdef FEAT_GUI
    // Avoid trouble with scrollbars that are going to be deleted in
    // win_free().
    if (gui.in_use)
	out_flush();
#endif

#ifdef FEAT_PROP_POPUP
    if (popup_win_closed(win) && !win_valid(win))
	return FAIL;
#endif

    // Trigger WinClosed just before starting to free window-related resources.
    trigger_winclosed(win);
    // autocmd may have freed the window already.
    if (!win_valid_any_tab(win))
	return OK;

    win_close_buffer(win, free_buf ? DOBUF_UNLOAD : 0, TRUE);

    if (only_one_window() && win_valid(win) && win->w_buffer == NULL
	    && (last_window() || curtab != prev_curtab
		|| close_last_window_tabpage(win, free_buf, prev_curtab)))
    {
	// Autocommands have closed all windows, quit now.  Restore
	// curwin->w_buffer, otherwise writing viminfo may fail.
	if (curwin->w_buffer == NULL)
	    curwin->w_buffer = curbuf;
	getout(0);
    }

    // Autocommands may have moved to another tab page.
    if (curtab != prev_curtab && win_valid_any_tab(win)
						      && win->w_buffer == NULL)
    {
	// Need to close the window anyway, since the buffer is NULL.
	// Don't trigger autocmds with a NULL buffer.
	block_autocmds();
	win_close_othertab(win, FALSE, prev_curtab);
	unblock_autocmds();
	return FAIL;
    }

    // Autocommands may have closed the window already or closed the only
    // other window.
    if (!win_valid(win) || last_window()
	    || close_last_window_tabpage(win, free_buf, prev_curtab))
	return FAIL;

    // Now we are really going to close the window.  Disallow any autocommand
    // to split a window to avoid trouble.
    // Also bail out of parse_queued_messages() to avoid it tries to update the
    // screen.
    ++split_disallowed;
#ifdef MESSAGE_QUEUE
    ++dont_parse_messages;
#endif

    // Free the memory used for the window and get the window that received
    // the screen space.
    wp = win_free_mem(win, &dir, NULL);

    if (help_window)
    {
	// Closing the help window moves the cursor back to the current window
	// of the snapshot.
	win_T *prev_win = get_snapshot_curwin(SNAP_HELP_IDX);

	if (win_valid(prev_win))
	    wp = prev_win;
    }

    // Make sure curwin isn't invalid.  It can cause severe trouble when
    // printing an error message.  For win_equal() curbuf needs to be valid
    // too.
    if (win == curwin)
    {
	curwin = wp;
#ifdef FEAT_QUICKFIX
	if (wp->w_p_pvw || bt_quickfix(wp->w_buffer))
	{
	    /*
	     * If the cursor goes to the preview or the quickfix window, try
	     * finding another window to go to.
	     */
	    for (;;)
	    {
		if (wp->w_next == NULL)
		    wp = firstwin;
		else
		    wp = wp->w_next;
		if (wp == curwin)
		    break;
		if (!wp->w_p_pvw && !bt_quickfix(wp->w_buffer))
		{
		    curwin = wp;
		    break;
		}
	    }
	}
#endif
	curbuf = curwin->w_buffer;
	close_curwin = TRUE;

	// The cursor position may be invalid if the buffer changed after last
	// using the window.
	check_cursor();
    }

    /*
     * If last window has a status line now and we don't want one, remove the
     * status line.  Do this before win_equal(), because it may change the
     * height of a window
     */
    last_status(FALSE);

    if (p_ea && (*p_ead == 'b' || *p_ead == dir))
	// If the frame of the closed window contains the new current window,
	// only resize that frame.  Otherwise resize all windows.
	win_equal(curwin, curwin->w_frame->fr_parent == win_frame, dir);
    else
    {
	win_comp_pos();
	win_fix_scroll(FALSE);
    }
    if (close_curwin)
    {
	// Pass WEE_ALLOW_PARSE_MESSAGES to decrement dont_parse_messages
	// before autocommands.
#ifdef MESSAGE_QUEUE
	did_decrement =
#else
	(void)
#endif
	    win_enter_ext(wp,
		WEE_CURWIN_INVALID | WEE_TRIGGER_ENTER_AUTOCMDS
		      | WEE_TRIGGER_LEAVE_AUTOCMDS | WEE_ALLOW_PARSE_MESSAGES);
	if (other_buffer)
	    // careful: after this wp and win may be invalid!
	    apply_autocmds(EVENT_BUFENTER, NULL, NULL, FALSE, curbuf);
    }

    --split_disallowed;
#ifdef MESSAGE_QUEUE
    if (!did_decrement)
	--dont_parse_messages;
#endif

    // After closing the help window, try restoring the window layout from
    // before it was opened.
    if (help_window)
	restore_snapshot(SNAP_HELP_IDX, close_curwin);

#ifdef FEAT_DIFF
    // If the window had 'diff' set and now there is only one window left in
    // the tab page with 'diff' set, and "closeoff" is in 'diffopt', then
    // execute ":diffoff!".
    if (diffopt_closeoff() && had_diffmode && curtab == prev_curtab)
    {
	int	diffcount = 0;
	win_T	*dwin;

	FOR_ALL_WINDOWS(dwin)
	    if (dwin->w_p_diff)
		++diffcount;
	if (diffcount == 1)
	    do_cmdline_cmd((char_u *)"diffoff!");
    }
#endif

#if defined(FEAT_GUI)
    // When 'guioptions' includes 'L' or 'R' may have to remove scrollbars.
    if (gui.in_use && !win_hasvertsplit())
	gui_init_which_components(NULL);
#endif

    redraw_all_later(UPD_NOT_VALID);
    return OK;
}

    static void
trigger_winnewpre(void)
{
    window_layout_lock();
    apply_autocmds(EVENT_WINNEWPRE, NULL, NULL, FALSE, NULL);
    window_layout_unlock();
}

    static void
trigger_winclosed(win_T *win)
{
    static int	recursive = FALSE;
    char_u	winid[NUMBUFLEN];

    if (recursive)
	return;
    recursive = TRUE;
    vim_snprintf((char *)winid, sizeof(winid), "%d", win->w_id);
    apply_autocmds(EVENT_WINCLOSED, winid, winid, FALSE, win->w_buffer);
    recursive = FALSE;
}

/*
 * Make a snapshot of all the window scroll positions and sizes of the current
 * tab page.
 */
    void
snapshot_windows_scroll_size(void)
{
    win_T *wp;
    FOR_ALL_WINDOWS(wp)
    {
	wp->w_last_topline = wp->w_topline;
#ifdef FEAT_DIFF
	wp->w_last_topfill = wp->w_topfill;
#endif
	wp->w_last_leftcol = wp->w_leftcol;
	wp->w_last_skipcol = wp->w_skipcol;
	wp->w_last_width = wp->w_width;
	wp->w_last_height = wp->w_height;
    }
}

static int did_initial_scroll_size_snapshot = FALSE;

    void
may_make_initial_scroll_size_snapshot(void)
{
    if (!did_initial_scroll_size_snapshot)
    {
	did_initial_scroll_size_snapshot = TRUE;
	snapshot_windows_scroll_size();
    }
}

#ifdef FEAT_EVAL
/*
 * Create a dictionary with information about size and scroll changes in a
 * window.
 * Returns the dictionary with refcount set to one.
 * Returns NULL when out of memory.
 */
    static dict_T *
make_win_info_dict(
	int width,
	int height,
	int topline,
# ifdef FEAT_DIFF
	int topfill,
# endif
	int leftcol,
	int skipcol)
{
    dict_T *d = dict_alloc();
    if (d == NULL)
	return NULL;
    d->dv_refcount = 1;

    // not actually looping, for breaking out on error
    while (1)
    {
	typval_T tv;
	tv.v_lock = 0;
	tv.v_type = VAR_NUMBER;

	tv.vval.v_number = width;
	if (dict_add_tv(d, "width", &tv) == FAIL)
	    break;
	tv.vval.v_number = height;
	if (dict_add_tv(d, "height", &tv) == FAIL)
	    break;
	tv.vval.v_number = topline;
	if (dict_add_tv(d, "topline", &tv) == FAIL)
	    break;
#ifdef FEAT_DIFF
	tv.vval.v_number = topfill;
#else
	tv.vval.v_number = 0;
#endif
	if (dict_add_tv(d, "topfill", &tv) == FAIL)
	    break;
	tv.vval.v_number = leftcol;
	if (dict_add_tv(d, "leftcol", &tv) == FAIL)
	    break;
	tv.vval.v_number = skipcol;
	if (dict_add_tv(d, "skipcol", &tv) == FAIL)
	    break;
	return d;
    }
    dict_unref(d);
    return NULL;
}
#endif

// Return values of check_window_scroll_resize():
#define CWSR_SCROLLED	1  // at least one window scrolled
#define CWSR_RESIZED	2  // at least one window size changed

/*
 * This function is used for three purposes:
 * 1. Goes over all windows in the current tab page and returns:
 *	0				no scrolling and no size changes found
 *	CWSR_SCROLLED			at least one window scrolled
 *	CWSR_RESIZED			at least one window changed size
 *	CWSR_SCROLLED + CWSR_RESIZED	both
 *    "size_count" is set to the nr of windows with size changes.
 *    "first_scroll_win" is set to the first window with any relevant changes.
 *    "first_size_win" is set to the first window with size changes.
 *
 * 2. When the first three arguments are NULL and "winlist" is not NULL,
 *    "winlist" is set to the list of window IDs with size changes.
 *
 * 3. When the first three arguments are NULL and "v_event" is not NULL,
 *    information about changed windows is added to "v_event".
 */
    static int
check_window_scroll_resize(
	int	*size_count,
	win_T	**first_scroll_win,
	win_T	**first_size_win,
	list_T	*winlist UNUSED,
	dict_T	*v_event UNUSED)
{
    int result = 0;
#ifdef FEAT_EVAL
    int listidx = 0;
    int tot_width = 0;
    int tot_height = 0;
    int tot_topline = 0;
# ifdef FEAT_DIFF
    int tot_topfill = 0;
# endif
    int tot_leftcol = 0;
    int tot_skipcol = 0;
#endif

    win_T *wp;
    FOR_ALL_WINDOWS(wp)
    {
	int size_changed = wp->w_last_width != wp->w_width
					  || wp->w_last_height != wp->w_height;
	if (size_changed)
	{
	    result |= CWSR_RESIZED;
#ifdef FEAT_EVAL
	    if (winlist != NULL)
	    {
		// Add this window to the list of changed windows.
		typval_T tv;
		tv.v_lock = 0;
		tv.v_type = VAR_NUMBER;
		tv.vval.v_number = wp->w_id;
		list_set_item(winlist, listidx++, &tv);
	    }
	    else
#endif
		if (size_count != NULL)
	    {
		++*size_count;
		if (*first_size_win == NULL)
		    *first_size_win = wp;
		// For WinScrolled the first window with a size change is used
		// even when it didn't scroll.
		if (*first_scroll_win == NULL)
		    *first_scroll_win = wp;
	    }
	}

	int scroll_changed = wp->w_last_topline != wp->w_topline
#ifdef FEAT_DIFF
				|| wp->w_last_topfill != wp->w_topfill
#endif
				|| wp->w_last_leftcol != wp->w_leftcol
				|| wp->w_last_skipcol != wp->w_skipcol;
	if (scroll_changed)
	{
	    result |= CWSR_SCROLLED;
	    if (first_scroll_win != NULL && *first_scroll_win == NULL)
		*first_scroll_win = wp;
	}

#ifdef FEAT_EVAL
	if ((size_changed || scroll_changed) && v_event != NULL)
	{
	    // Add info about this window to the v:event dictionary.
	    int width = wp->w_width - wp->w_last_width;
	    int height = wp->w_height - wp->w_last_height;
	    int topline = wp->w_topline - wp->w_last_topline;
#ifdef FEAT_DIFF
	    int topfill = wp->w_topfill - wp->w_last_topfill;
#endif
	    int leftcol = wp->w_leftcol - wp->w_last_leftcol;
	    int skipcol = wp->w_skipcol - wp->w_last_skipcol;
	    dict_T *d = make_win_info_dict(width, height, topline,
#ifdef FEAT_DIFF
							    topfill,
#endif
							    leftcol, skipcol);
	    if (d == NULL)
		break;
	    char winid[NUMBUFLEN];
	    vim_snprintf(winid, sizeof(winid), "%d", wp->w_id);
	    if (dict_add_dict(v_event, winid, d) == FAIL)
	    {
		dict_unref(d);
		break;
	    }
	    --d->dv_refcount;

	    tot_width += abs(width);
	    tot_height += abs(height);
	    tot_topline += abs(topline);
#ifdef FEAT_DIFF
	    tot_topfill += abs(topfill);
#endif
	    tot_leftcol += abs(leftcol);
	    tot_skipcol += abs(skipcol);
	}
#endif
    }

#ifdef FEAT_EVAL
    if (v_event != NULL)
    {
	dict_T *alldict = make_win_info_dict(tot_width, tot_height, tot_topline,
# ifdef FEAT_DIFF
						    tot_topfill,
# endif
						    tot_leftcol, tot_skipcol);
	if (alldict != NULL)
	{
	    if (dict_add_dict(v_event, "all", alldict) == FAIL)
		dict_unref(alldict);
	    else
		--alldict->dv_refcount;
	}
    }
#endif

    return result;
}

/*
 * Trigger WinScrolled and/or WinResized if any window in the current tab page
 * scrolled or changed size.
 */
    void
may_trigger_win_scrolled_resized(void)
{
    static int	    recursive = FALSE;
    int		    do_resize = has_winresized();
    int		    do_scroll = has_winscrolled();

    // Do not trigger WinScrolled or WinResized recursively.  Do not trigger
    // before the initial snapshot of the w_last_ values was made.
    if (recursive
	    || !(do_scroll || do_resize)
	    || !did_initial_scroll_size_snapshot)
	return;

    int size_count = 0;
    win_T *first_scroll_win = NULL, *first_size_win = NULL;
    int cwsr = check_window_scroll_resize(&size_count,
					   &first_scroll_win, &first_size_win,
					   NULL, NULL);
    int trigger_resize = do_resize && size_count > 0;
    int trigger_scroll = do_scroll && cwsr != 0;
    if (!trigger_resize && !trigger_scroll)
	return;  // no relevant changes
#ifdef FEAT_EVAL
    list_T *windows_list = NULL;
    if (trigger_resize)
    {
	// Create the list for v:event.windows before making the snapshot.
	windows_list = list_alloc_with_items(size_count);
	(void)check_window_scroll_resize(NULL, NULL, NULL, windows_list, NULL);
    }

    dict_T *scroll_dict = NULL;
    if (trigger_scroll)
    {
	// Create the dict with entries for v:event before making the snapshot.
	scroll_dict = dict_alloc();
	if (scroll_dict != NULL)
	{
	    scroll_dict->dv_refcount = 1;
	    (void)check_window_scroll_resize(NULL, NULL, NULL, NULL,
								  scroll_dict);
	}
    }
#endif

    // WinScrolled/WinResized are triggered only once, even when multiple
    // windows scrolled or changed size.  Store the current values before
    // triggering the event, if a scroll or resize happens as a side effect
    // then WinScrolled/WinResized is triggered for that later.
    snapshot_windows_scroll_size();

    // "curwin" may be different from the actual current window, make
    // sure it can be restored.
    window_layout_lock();
    recursive = TRUE;

    // If both are to be triggered do WinResized first.
    if (trigger_resize)
    {
#ifdef FEAT_EVAL
	save_v_event_T  save_v_event;
	dict_T		*v_event = get_v_event(&save_v_event);

	if (dict_add_list(v_event, "windows", windows_list) == OK)
	{
	    dict_set_items_ro(v_event);
#endif
	    char_u winid[NUMBUFLEN];
	    vim_snprintf((char *)winid, sizeof(winid), "%d",
							 first_size_win->w_id);
	    apply_autocmds(EVENT_WINRESIZED, winid, winid, FALSE,
						     first_size_win->w_buffer);
#ifdef FEAT_EVAL
	}
	restore_v_event(v_event, &save_v_event);
#endif
    }

    if (trigger_scroll
#ifdef FEAT_EVAL
	    && scroll_dict != NULL
#endif
	    )
    {
#ifdef FEAT_EVAL
	save_v_event_T  save_v_event;
	dict_T		*v_event = get_v_event(&save_v_event);

	// Move the entries from scroll_dict to v_event.
	dict_extend(v_event, scroll_dict, (char_u *)"move", NULL);
	dict_set_items_ro(v_event);
	dict_unref(scroll_dict);
#endif
	char_u winid[NUMBUFLEN];
	vim_snprintf((char *)winid, sizeof(winid), "%d",
						       first_scroll_win->w_id);
	apply_autocmds(EVENT_WINSCROLLED, winid, winid, FALSE,
						   first_scroll_win->w_buffer);
#ifdef FEAT_EVAL
	restore_v_event(v_event, &save_v_event);
#endif
    }

    recursive = FALSE;
    window_layout_unlock();
}

/*
 * Close window "win" in tab page "tp", which is not the current tab page.
 * This may be the last window in that tab page and result in closing the tab,
 * thus "tp" may become invalid!
 * Caller must check if buffer is hidden and whether the tabline needs to be
 * updated.
 */
    void
win_close_othertab(win_T *win, int free_buf, tabpage_T *tp)
{
    win_T	*wp;
    int		dir;
    tabpage_T   *ptp = NULL;
    int		free_tp = FALSE;

    // Get here with win->w_buffer == NULL when win_close() detects the tab
    // page changed.
    if (win->w_closing || (win->w_buffer != NULL
					       && win->w_buffer->b_locked > 0))
	return; // window is already being closed

    // Trigger WinClosed just before starting to free window-related resources.
    trigger_winclosed(win);
    // autocmd may have freed the window already.
    if (!win_valid_any_tab(win))
	return;

    if (win->w_buffer != NULL)
	// Close the link to the buffer.
	close_buffer(win, win->w_buffer, free_buf ? DOBUF_UNLOAD : 0,
								 FALSE, TRUE);

    // Careful: Autocommands may have closed the tab page or made it the
    // current tab page.
    for (ptp = first_tabpage; ptp != NULL && ptp != tp; ptp = ptp->tp_next)
	;
    if (ptp == NULL || tp == curtab)
    {
	// If the buffer was removed from the window we have to give it any
	// buffer.
	if (win_valid_any_tab(win) && win->w_buffer == NULL)
	{
	    win->w_buffer = firstbuf;
	    ++firstbuf->b_nwindows;
	    win_init_empty(win);
	}
	return;
    }

    // Autocommands may have closed the window already.
    for (wp = tp->tp_firstwin; wp != NULL && wp != win; wp = wp->w_next)
	;
    if (wp == NULL)
	return;

    // When closing the last window in a tab page remove the tab page.
    if (tp->tp_firstwin == tp->tp_lastwin)
    {
	int	h = tabline_height();

	if (tp == first_tabpage)
	    first_tabpage = tp->tp_next;
	else
	{
	    for (ptp = first_tabpage; ptp != NULL && ptp->tp_next != tp;
							   ptp = ptp->tp_next)
		;
	    if (ptp == NULL)
	    {
		internal_error("win_close_othertab()");
		return;
	    }
	    ptp->tp_next = tp->tp_next;
	}
	free_tp = TRUE;
	redraw_tabline = TRUE;
	if (h != tabline_height())
	    shell_new_rows();
    }

    // Free the memory used for the window.
    win_free_mem(win, &dir, tp);

    if (free_tp)
	free_tabpage(tp);
}

/*
 * Free the memory used for a window.
 * Returns a pointer to the window that got the freed up space.
 */
    static win_T *
win_free_mem(
    win_T	*win,
    int		*dirp,		// set to 'v' or 'h' for direction if 'ea'
    tabpage_T	*tp)		// tab page "win" is in, NULL for current
{
    frame_T	*frp;
    win_T	*wp;
    tabpage_T	*win_tp = tp == NULL ? curtab : tp;

    // Remove the window and its frame from the tree of frames.
    frp = win->w_frame;
    wp = winframe_remove(win, dirp, tp);
    vim_free(frp);
    win_free(win, tp);

    // When deleting the current window in the tab, select a new current
    // window.
    if (win == win_tp->tp_curwin)
	win_tp->tp_curwin = wp;

    return wp;
}

#if defined(EXITFREE) || defined(PROTO)
    void
win_free_all(void)
{
    int		dummy;

    // avoid an error for switching tabpage with the cmdline window open
    cmdwin_type = 0;
    cmdwin_buf = NULL;
    cmdwin_win = NULL;

    while (first_tabpage->tp_next != NULL)
	tabpage_close(TRUE);

    for (int i = 0; i < AUCMD_WIN_COUNT; ++i)
	if (aucmd_win[i].auc_win != NULL)
	{
	    (void)win_free_mem(aucmd_win[i].auc_win, &dummy, NULL);
	    aucmd_win[i].auc_win = NULL;
	}

    while (firstwin != NULL)
	(void)win_free_mem(firstwin, &dummy, NULL);

    // No window should be used after this. Set curwin to NULL to crash
    // instead of using freed memory.
    curwin = NULL;
}
#endif

/*
 * Remove a window and its frame from the tree of frames.
 * Returns a pointer to the window that got the freed up space.
 */
    win_T *
winframe_remove(
    win_T	*win,
    int		*dirp UNUSED,	// set to 'v' or 'h' for direction if 'ea'
    tabpage_T	*tp)		// tab page "win" is in, NULL for current
{
    frame_T	*frp, *frp2, *frp3;
    frame_T	*frp_close = win->w_frame;
    win_T	*wp;

    /*
     * If there is only one window there is nothing to remove.
     */
    if (tp == NULL ? ONE_WINDOW : tp->tp_firstwin == tp->tp_lastwin)
	return NULL;

    /*
     * Remove the window from its frame.
     */
    frp2 = win_altframe(win, tp);
    wp = frame2win(frp2);

    // Remove this frame from the list of frames.
    frame_remove(frp_close);

    if (frp_close->fr_parent->fr_layout == FR_COL)
    {
	// When 'winfixheight' is set, try to find another frame in the column
	// (as close to the closed frame as possible) to distribute the height
	// to.
	if (frp2->fr_win != NULL && frp2->fr_win->w_p_wfh)
	{
	    frp = frp_close->fr_prev;
	    frp3 = frp_close->fr_next;
	    while (frp != NULL || frp3 != NULL)
	    {
		if (frp != NULL)
		{
		    if (!frame_fixed_height(frp))
		    {
			frp2 = frp;
			wp = frame2win(frp2);
			break;
		    }
		    frp = frp->fr_prev;
		}
		if (frp3 != NULL)
		{
		    if (frp3->fr_win != NULL && !frp3->fr_win->w_p_wfh)
		    {
			frp2 = frp3;
			wp = frp3->fr_win;
			break;
		    }
		    frp3 = frp3->fr_next;
		}
	    }
	}
	frame_new_height(frp2, frp2->fr_height + frp_close->fr_height,
			    frp2 == frp_close->fr_next ? TRUE : FALSE, FALSE);
	*dirp = 'v';
    }
    else
    {
	// When 'winfixwidth' is set, try to find another frame in the column
	// (as close to the closed frame as possible) to distribute the width
	// to.
	if (frp2->fr_win != NULL && frp2->fr_win->w_p_wfw)
	{
	    frp = frp_close->fr_prev;
	    frp3 = frp_close->fr_next;
	    while (frp != NULL || frp3 != NULL)
	    {
		if (frp != NULL)
		{
		    if (!frame_fixed_width(frp))
		    {
			frp2 = frp;
			wp = frame2win(frp2);
			break;
		    }
		    frp = frp->fr_prev;
		}
		if (frp3 != NULL)
		{
		    if (frp3->fr_win != NULL && !frp3->fr_win->w_p_wfw)
		    {
			frp2 = frp3;
			wp = frp3->fr_win;
			break;
		    }
		    frp3 = frp3->fr_next;
		}
	    }
	}
	frame_new_width(frp2, frp2->fr_width + frp_close->fr_width,
			    frp2 == frp_close->fr_next ? TRUE : FALSE, FALSE);
	*dirp = 'h';
    }

    // If rows/columns go to a window below/right its positions need to be
    // updated.  Can only be done after the sizes have been updated.
    if (frp2 == frp_close->fr_next)
    {
	int row = win->w_winrow;
	int col = win->w_wincol;

	frame_comp_pos(frp2, &row, &col);
    }

    if (frp2->fr_next == NULL && frp2->fr_prev == NULL)
    {
	// There is no other frame in this list, move its info to the parent
	// and remove it.
	frp2->fr_parent->fr_layout = frp2->fr_layout;
	frp2->fr_parent->fr_child = frp2->fr_child;
	FOR_ALL_FRAMES(frp, frp2->fr_child)
	    frp->fr_parent = frp2->fr_parent;
	frp2->fr_parent->fr_win = frp2->fr_win;
	if (frp2->fr_win != NULL)
	    frp2->fr_win->w_frame = frp2->fr_parent;
	frp = frp2->fr_parent;
	if (topframe->fr_child == frp2)
	    topframe->fr_child = frp;
	vim_free(frp2);

	frp2 = frp->fr_parent;
	if (frp2 != NULL && frp2->fr_layout == frp->fr_layout)
	{
	    // The frame above the parent has the same layout, have to merge
	    // the frames into this list.
	    if (frp2->fr_child == frp)
		frp2->fr_child = frp->fr_child;
	    frp->fr_child->fr_prev = frp->fr_prev;
	    if (frp->fr_prev != NULL)
		frp->fr_prev->fr_next = frp->fr_child;
	    for (frp3 = frp->fr_child; ; frp3 = frp3->fr_next)
	    {
		frp3->fr_parent = frp2;
		if (frp3->fr_next == NULL)
		{
		    frp3->fr_next = frp->fr_next;
		    if (frp->fr_next != NULL)
			frp->fr_next->fr_prev = frp3;
		    break;
		}
	    }
	    if (topframe->fr_child == frp)
		topframe->fr_child = frp2;
	    vim_free(frp);
	}
    }

    return wp;
}

/*
 * Return a pointer to the frame that will receive the empty screen space that
 * is left over after "win" is closed.
 *
 * If 'splitbelow' or 'splitright' is set, the space goes above or to the left
 * by default.  Otherwise, the free space goes below or to the right.  The
 * result is that opening a window and then immediately closing it will
 * preserve the initial window layout.  The 'wfh' and 'wfw' settings are
 * respected when possible.
 */
    static frame_T *
win_altframe(
    win_T	*win,
    tabpage_T	*tp)		// tab page "win" is in, NULL for current
{
    frame_T	*frp;
    frame_T	*other_fr, *target_fr;

    if (tp == NULL ? ONE_WINDOW : tp->tp_firstwin == tp->tp_lastwin)
	return alt_tabpage()->tp_curwin->w_frame;

    frp = win->w_frame;

    if (frp->fr_prev == NULL)
	return frp->fr_next;
    if (frp->fr_next == NULL)
	return frp->fr_prev;

    // By default the next window will get the space that was abandoned by this
    // window
    target_fr = frp->fr_next;
    other_fr  = frp->fr_prev;

    // If this is part of a column of windows and 'splitbelow' is true then the
    // previous window will get the space.
    if (frp->fr_parent != NULL && frp->fr_parent->fr_layout == FR_COL && p_sb)
    {
	target_fr = frp->fr_prev;
	other_fr  = frp->fr_next;
    }

    // If this is part of a row of windows, and 'splitright' is true then the
    // previous window will get the space.
    if (frp->fr_parent != NULL && frp->fr_parent->fr_layout == FR_ROW && p_spr)
    {
	target_fr = frp->fr_prev;
	other_fr  = frp->fr_next;
    }

    // If 'wfh' or 'wfw' is set for the target and not for the alternate
    // window, reverse the selection.
    if (frp->fr_parent != NULL && frp->fr_parent->fr_layout == FR_ROW)
    {
	if (frame_fixed_width(target_fr) && !frame_fixed_width(other_fr))
	    target_fr = other_fr;
    }
    else
    {
	if (frame_fixed_height(target_fr) && !frame_fixed_height(other_fr))
	    target_fr = other_fr;
    }

    return target_fr;
}

/*
 * Return the tabpage that will be used if the current one is closed.
 */
    static tabpage_T *
alt_tabpage(void)
{
    tabpage_T	*tp;

    // Use the next tab page if possible.
    if (curtab->tp_next != NULL)
	return curtab->tp_next;

    // Find the last but one tab page.
    for (tp = first_tabpage; tp->tp_next != curtab; tp = tp->tp_next)
	;
    return tp;
}

/*
 * Find the left-upper window in frame "frp".
 */
    static win_T *
frame2win(frame_T *frp)
{
    while (frp->fr_win == NULL)
	frp = frp->fr_child;
    return frp->fr_win;
}

/*
 * Return TRUE if frame "frp" contains window "wp".
 */
    static int
frame_has_win(frame_T *frp, win_T *wp)
{
    frame_T	*p;

    if (frp->fr_layout == FR_LEAF)
	return frp->fr_win == wp;

    FOR_ALL_FRAMES(p, frp->fr_child)
	if (frame_has_win(p, wp))
	    return TRUE;
    return FALSE;
}

/*
 * Set a new height for a frame.  Recursively sets the height for contained
 * frames and windows.  Caller must take care of positions.
 */
    static void
frame_new_height(
    frame_T	*topfrp,
    int		height,
    int		topfirst,	// resize topmost contained frame first
    int		wfh)		// obey 'winfixheight' when there is a choice;
				// may cause the height not to be set
{
    frame_T	*frp;
    int		extra_lines;
    int		h;

    if (topfrp->fr_win != NULL)
    {
	// Simple case: just one window.
	win_new_height(topfrp->fr_win,
				    height - topfrp->fr_win->w_status_height
					      - WINBAR_HEIGHT(topfrp->fr_win));
    }
    else if (topfrp->fr_layout == FR_ROW)
    {
	do
	{
	    // All frames in this row get the same new height.
	    FOR_ALL_FRAMES(frp, topfrp->fr_child)
	    {
		frame_new_height(frp, height, topfirst, wfh);
		if (frp->fr_height > height)
		{
		    // Could not fit the windows, make the whole row higher.
		    height = frp->fr_height;
		    break;
		}
	    }
	}
	while (frp != NULL);
    }
    else    // fr_layout == FR_COL
    {
	// Complicated case: Resize a column of frames.  Resize the bottom
	// frame first, frames above that when needed.

	frp = topfrp->fr_child;
	if (wfh)
	    // Advance past frames with one window with 'wfh' set.
	    while (frame_fixed_height(frp))
	    {
		frp = frp->fr_next;
		if (frp == NULL)
		    return;	    // no frame without 'wfh', give up
	    }
	if (!topfirst)
	{
	    // Find the bottom frame of this column
	    while (frp->fr_next != NULL)
		frp = frp->fr_next;
	    if (wfh)
		// Advance back for frames with one window with 'wfh' set.
		while (frame_fixed_height(frp))
		    frp = frp->fr_prev;
	}

	extra_lines = height - topfrp->fr_height;
	if (extra_lines < 0)
	{
	    // reduce height of contained frames, bottom or top frame first
	    while (frp != NULL)
	    {
		h = frame_minheight(frp, NULL);
		if (frp->fr_height + extra_lines < h)
		{
		    extra_lines += frp->fr_height - h;
		    frame_new_height(frp, h, topfirst, wfh);
		}
		else
		{
		    frame_new_height(frp, frp->fr_height + extra_lines,
							       topfirst, wfh);
		    break;
		}
		if (topfirst)
		{
		    do
			frp = frp->fr_next;
		    while (wfh && frp != NULL && frame_fixed_height(frp));
		}
		else
		{
		    do
			frp = frp->fr_prev;
		    while (wfh && frp != NULL && frame_fixed_height(frp));
		}
		// Increase "height" if we could not reduce enough frames.
		if (frp == NULL)
		    height -= extra_lines;
	    }
	}
	else if (extra_lines > 0)
	{
	    // increase height of bottom or top frame
	    frame_new_height(frp, frp->fr_height + extra_lines, topfirst, wfh);
	}
    }
    topfrp->fr_height = height;
}

/*
 * Return TRUE if height of frame "frp" should not be changed because of
 * the 'winfixheight' option.
 */
    static int
frame_fixed_height(frame_T *frp)
{
    // frame with one window: fixed height if 'winfixheight' set.
    if (frp->fr_win != NULL)
	return frp->fr_win->w_p_wfh;

    if (frp->fr_layout == FR_ROW)
    {
	// The frame is fixed height if one of the frames in the row is fixed
	// height.
	FOR_ALL_FRAMES(frp, frp->fr_child)
	    if (frame_fixed_height(frp))
		return TRUE;
	return FALSE;
    }

    // frp->fr_layout == FR_COL: The frame is fixed height if all of the
    // frames in the row are fixed height.
    FOR_ALL_FRAMES(frp, frp->fr_child)
	if (!frame_fixed_height(frp))
	    return FALSE;
    return TRUE;
}

/*
 * Return TRUE if width of frame "frp" should not be changed because of
 * the 'winfixwidth' option.
 */
    static int
frame_fixed_width(frame_T *frp)
{
    // frame with one window: fixed width if 'winfixwidth' set.
    if (frp->fr_win != NULL)
	return frp->fr_win->w_p_wfw;

    if (frp->fr_layout == FR_COL)
    {
	// The frame is fixed width if one of the frames in the row is fixed
	// width.
	FOR_ALL_FRAMES(frp, frp->fr_child)
	    if (frame_fixed_width(frp))
		return TRUE;
	return FALSE;
    }

    // frp->fr_layout == FR_ROW: The frame is fixed width if all of the
    // frames in the row are fixed width.
    FOR_ALL_FRAMES(frp, frp->fr_child)
	if (!frame_fixed_width(frp))
	    return FALSE;
    return TRUE;
}

/*
 * Add a status line to windows at the bottom of "frp".
 * Note: Does not check if there is room!
 */
    static void
frame_add_statusline(frame_T *frp)
{
    win_T	*wp;

    if (frp->fr_layout == FR_LEAF)
    {
	wp = frp->fr_win;
	wp->w_status_height = STATUS_HEIGHT;
    }
    else if (frp->fr_layout == FR_ROW)
    {
	// Handle all the frames in the row.
	FOR_ALL_FRAMES(frp, frp->fr_child)
	    frame_add_statusline(frp);
    }
    else // frp->fr_layout == FR_COL
    {
	// Only need to handle the last frame in the column.
	for (frp = frp->fr_child; frp->fr_next != NULL; frp = frp->fr_next)
	    ;
	frame_add_statusline(frp);
    }
}

/*
 * Set width of a frame.  Handles recursively going through contained frames.
 * May remove separator line for windows at the right side (for win_close()).
 */
    static void
frame_new_width(
    frame_T	*topfrp,
    int		width,
    int		leftfirst,	// resize leftmost contained frame first
    int		wfw)		// obey 'winfixwidth' when there is a choice;
				// may cause the width not to be set
{
    frame_T	*frp;
    int		extra_cols;
    int		w;
    win_T	*wp;

    if (topfrp->fr_layout == FR_LEAF)
    {
	// Simple case: just one window.
	wp = topfrp->fr_win;
	// Find out if there are any windows right of this one.
	for (frp = topfrp; frp->fr_parent != NULL; frp = frp->fr_parent)
	    if (frp->fr_parent->fr_layout == FR_ROW && frp->fr_next != NULL)
		break;
	if (frp->fr_parent == NULL)
	    wp->w_vsep_width = 0;
	win_new_width(wp, width - wp->w_vsep_width);
    }
    else if (topfrp->fr_layout == FR_COL)
    {
	do
	{
	    // All frames in this column get the same new width.
	    FOR_ALL_FRAMES(frp, topfrp->fr_child)
	    {
		frame_new_width(frp, width, leftfirst, wfw);
		if (frp->fr_width > width)
		{
		    // Could not fit the windows, make whole column wider.
		    width = frp->fr_width;
		    break;
		}
	    }
	} while (frp != NULL);
    }
    else    // fr_layout == FR_ROW
    {
	// Complicated case: Resize a row of frames.  Resize the rightmost
	// frame first, frames left of it when needed.

	frp = topfrp->fr_child;
	if (wfw)
	    // Advance past frames with one window with 'wfw' set.
	    while (frame_fixed_width(frp))
	    {
		frp = frp->fr_next;
		if (frp == NULL)
		    return;	    // no frame without 'wfw', give up
	    }
	if (!leftfirst)
	{
	    // Find the rightmost frame of this row
	    while (frp->fr_next != NULL)
		frp = frp->fr_next;
	    if (wfw)
		// Advance back for frames with one window with 'wfw' set.
		while (frame_fixed_width(frp))
		    frp = frp->fr_prev;
	}

	extra_cols = width - topfrp->fr_width;
	if (extra_cols < 0)
	{
	    // reduce frame width, rightmost frame first
	    while (frp != NULL)
	    {
		w = frame_minwidth(frp, NULL);
		if (frp->fr_width + extra_cols < w)
		{
		    extra_cols += frp->fr_width - w;
		    frame_new_width(frp, w, leftfirst, wfw);
		}
		else
		{
		    frame_new_width(frp, frp->fr_width + extra_cols,
							      leftfirst, wfw);
		    break;
		}
		if (leftfirst)
		{
		    do
			frp = frp->fr_next;
		    while (wfw && frp != NULL && frame_fixed_width(frp));
		}
		else
		{
		    do
			frp = frp->fr_prev;
		    while (wfw && frp != NULL && frame_fixed_width(frp));
		}
		// Increase "width" if we could not reduce enough frames.
		if (frp == NULL)
		    width -= extra_cols;
	    }
	}
	else if (extra_cols > 0)
	{
	    // increase width of rightmost frame
	    frame_new_width(frp, frp->fr_width + extra_cols, leftfirst, wfw);
	}
    }
    topfrp->fr_width = width;
}

/*
 * Add the vertical separator to windows at the right side of "frp".
 * Note: Does not check if there is room!
 */
    static void
frame_add_vsep(frame_T *frp)
{
    win_T	*wp;

    if (frp->fr_layout == FR_LEAF)
    {
	wp = frp->fr_win;
	if (wp->w_vsep_width == 0)
	{
	    if (wp->w_width > 0)	// don't make it negative
		--wp->w_width;
	    wp->w_vsep_width = 1;
	}
    }
    else if (frp->fr_layout == FR_COL)
    {
	// Handle all the frames in the column.
	FOR_ALL_FRAMES(frp, frp->fr_child)
	    frame_add_vsep(frp);
    }
    else // frp->fr_layout == FR_ROW
    {
	// Only need to handle the last frame in the row.
	frp = frp->fr_child;
	while (frp->fr_next != NULL)
	    frp = frp->fr_next;
	frame_add_vsep(frp);
    }
}

/*
 * Set frame width from the window it contains.
 */
    static void
frame_fix_width(win_T *wp)
{
    wp->w_frame->fr_width = wp->w_width + wp->w_vsep_width;
}

/*
 * Set frame height from the window it contains.
 */
    static void
frame_fix_height(win_T *wp)
{
    wp->w_frame->fr_height = VISIBLE_HEIGHT(wp) + wp->w_status_height;
}

/*
 * Compute the minimal height for frame "topfrp".
 * Uses the 'winminheight' option.
 * When "next_curwin" isn't NULL, use p_wh for this window.
 * When "next_curwin" is NOWIN, don't use at least one line for the current
 * window.
 */
    static int
frame_minheight(frame_T *topfrp, win_T *next_curwin)
{
    frame_T	*frp;
    int		m;
    int		n;

    if (topfrp->fr_win != NULL)
    {
	if (topfrp->fr_win == next_curwin)
	    m = p_wh + topfrp->fr_win->w_status_height;
	else
	{
	    // window: minimal height of the window plus status line
	    m = p_wmh + topfrp->fr_win->w_status_height;
	    if (topfrp->fr_win == curwin && next_curwin == NULL)
	    {
		// Current window is minimal one line high and WinBar is
		// visible.
		if (p_wmh == 0)
		    ++m;
		m += WINBAR_HEIGHT(curwin);
	    }
	}
    }
    else if (topfrp->fr_layout == FR_ROW)
    {
	// get the minimal height from each frame in this row
	m = 0;
	FOR_ALL_FRAMES(frp, topfrp->fr_child)
	{
	    n = frame_minheight(frp, next_curwin);
	    if (n > m)
		m = n;
	}
    }
    else
    {
	// Add up the minimal heights for all frames in this column.
	m = 0;
	FOR_ALL_FRAMES(frp, topfrp->fr_child)
	    m += frame_minheight(frp, next_curwin);
    }

    return m;
}

/*
 * Compute the minimal width for frame "topfrp".
 * When "next_curwin" isn't NULL, use p_wiw for this window.
 * When "next_curwin" is NOWIN, don't use at least one column for the current
 * window.
 */
    static int
frame_minwidth(
    frame_T	*topfrp,
    win_T	*next_curwin)	// use p_wh and p_wiw for next_curwin
{
    frame_T	*frp;
    int		m, n;

    if (topfrp->fr_win != NULL)
    {
	if (topfrp->fr_win == next_curwin)
	    m = p_wiw + topfrp->fr_win->w_vsep_width;
	else
	{
	    // window: minimal width of the window plus separator column
	    m = p_wmw + topfrp->fr_win->w_vsep_width;
	    // Current window is minimal one column wide
	    if (p_wmw == 0 && topfrp->fr_win == curwin && next_curwin == NULL)
		++m;
	}
    }
    else if (topfrp->fr_layout == FR_COL)
    {
	// get the minimal width from each frame in this column
	m = 0;
	FOR_ALL_FRAMES(frp, topfrp->fr_child)
	{
	    n = frame_minwidth(frp, next_curwin);
	    if (n > m)
		m = n;
	}
    }
    else
    {
	// Add up the minimal widths for all frames in this row.
	m = 0;
	FOR_ALL_FRAMES(frp, topfrp->fr_child)
	    m += frame_minwidth(frp, next_curwin);
    }

    return m;
}


/*
 * Try to close all windows except current one.
 * Buffers in the other windows become hidden if 'hidden' is set, or '!' is
 * used and the buffer was modified.
 *
 * Used by ":bdel" and ":only".
 */
    void
close_others(
    int		message,
    int		forceit)	    // always hide all other windows
{
    win_T	*wp;
    win_T	*nextwp;
    int		r;

    if (one_window())
    {
	if (message && !autocmd_busy)
	    msg(_(m_onlyone));
	return;
    }

    // Be very careful here: autocommands may change the window layout.
    for (wp = firstwin; win_valid(wp); wp = nextwp)
    {
	nextwp = wp->w_next;
	if (wp == curwin)		// don't close current window
	    continue;

	// autoccommands messed this one up
	if (!buf_valid(wp->w_buffer) && win_valid(wp))
	{
	    wp->w_buffer = NULL;
	    win_close(wp, 0);
	    continue;
	}
	// Check if it's allowed to abandon this window
	r = can_abandon(wp->w_buffer, forceit);
	if (!win_valid(wp))		// autocommands messed wp up
	{
	    nextwp = firstwin;
	    continue;
	}
	if (!r)
	{
#if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
	    if (message && (p_confirm
			 || (cmdmod.cmod_flags & CMOD_CONFIRM)) && p_write)
	    {
		dialog_changed(wp->w_buffer, FALSE);
		if (!win_valid(wp))		// autocommands messed wp up
		{
		    nextwp = firstwin;
		    continue;
		}
	    }
	    if (bufIsChanged(wp->w_buffer))
#endif
		continue;
	}
	win_close(wp, !buf_hide(wp->w_buffer) && !bufIsChanged(wp->w_buffer));
    }

    if (message && !ONE_WINDOW)
	emsg(_(e_other_window_contains_changes));
}

/*
 * Store the relevant window pointers for tab page "tp".  To be used before
 * use_tabpage().
 */
    void
unuse_tabpage(tabpage_T *tp)
{
    tp->tp_topframe = topframe;
    tp->tp_firstwin = firstwin;
    tp->tp_lastwin = lastwin;
    tp->tp_curwin = curwin;
}

/*
 * Set the relevant pointers to use tab page "tp".  May want to call
 * unuse_tabpage() first.
 */
    void
use_tabpage(tabpage_T *tp)
{
    curtab = tp;
    topframe = curtab->tp_topframe;
    firstwin = curtab->tp_firstwin;
    lastwin = curtab->tp_lastwin;
    curwin = curtab->tp_curwin;
}

/*
 * Allocate the first window and put an empty buffer in it.
 * Called from main().
 * Return FAIL when something goes wrong (out of memory).
 */
    int
win_alloc_first(void)
{
    if (win_alloc_firstwin(NULL) == FAIL)
	return FAIL;

    first_tabpage = alloc_tabpage();
    if (first_tabpage == NULL)
	return FAIL;
    curtab = first_tabpage;
    unuse_tabpage(first_tabpage);

    return OK;
}

/*
 * Allocate and init a window that is not a regular window.
 * This can only be done after the first window is fully initialized, thus it
 * can't be in win_alloc_first().
 */
    win_T *
win_alloc_popup_win(void)
{
    win_T *wp;

    wp = win_alloc(NULL, TRUE);
    if (wp == NULL)
	return NULL;
    // We need to initialize options with something, using the current
    // window makes most sense.
    win_init_some(wp, curwin);

    RESET_BINDING(wp);
    new_frame(wp);
    return wp;
}

/*
 * Initialize window "wp" to display buffer "buf".
 */
    void
win_init_popup_win(win_T *wp, buf_T *buf)
{
    wp->w_buffer = buf;
    ++buf->b_nwindows;
    win_init_empty(wp); // set cursor and topline to safe values

    // Make sure w_localdir and globaldir are NULL to avoid a chdir() in
    // win_enter_ext().
    VIM_CLEAR(wp->w_localdir);
}

/*
 * Allocate the first window or the first window in a new tab page.
 * When "oldwin" is NULL create an empty buffer for it.
 * When "oldwin" is not NULL copy info from it to the new window.
 * Return FAIL when something goes wrong (out of memory).
 */
    static int
win_alloc_firstwin(win_T *oldwin)
{
    curwin = win_alloc(NULL, FALSE);
    if (curwin == NULL)
	return FAIL;
    if (oldwin == NULL)
    {
	// Very first window, need to create an empty buffer for it and
	// initialize from scratch.
	curbuf = buflist_new(NULL, NULL, 1L, BLN_LISTED);
	if (curwin == NULL || curbuf == NULL)
	    return FAIL;
	curwin->w_buffer = curbuf;
#ifdef FEAT_SYN_HL
	curwin->w_s = &(curbuf->b_s);
#endif
	curbuf->b_nwindows = 1;	// there is one window
	curwin->w_alist = &global_alist;
	curwin_init();		// init current window
    }
    else
    {
	// First window in new tab page, initialize it from "oldwin".
	win_init(curwin, oldwin, 0);

	// We don't want cursor- and scroll-binding in the first window.
	RESET_BINDING(curwin);
    }

    new_frame(curwin);
    if (curwin->w_frame == NULL)
	return FAIL;
    topframe = curwin->w_frame;
    topframe->fr_width = Columns;
    topframe->fr_height = Rows - p_ch;

    return OK;
}

/*
 * Create a frame for window "wp".
 */
    static void
new_frame(win_T *wp)
{
    frame_T *frp = ALLOC_CLEAR_ONE(frame_T);

    wp->w_frame = frp;
    if (frp == NULL)
	return;
    frp->fr_layout = FR_LEAF;
    frp->fr_win = wp;
}

/*
 * Initialize the window and frame size to the maximum.
 */
    void
win_init_size(void)
{
    firstwin->w_height = ROWS_AVAIL;
    firstwin->w_prev_height = ROWS_AVAIL;
    topframe->fr_height = ROWS_AVAIL;
    firstwin->w_width = Columns;
    topframe->fr_width = Columns;
}

/*
 * Allocate a new tabpage_T and init the values.
 * Returns NULL when out of memory.
 */
    static tabpage_T *
alloc_tabpage(void)
{
    tabpage_T	*tp;
# ifdef FEAT_GUI
    int		i;
# endif


    tp = ALLOC_CLEAR_ONE(tabpage_T);
    if (tp == NULL)
	return NULL;

# ifdef FEAT_EVAL
    // init t: variables
    tp->tp_vars = dict_alloc_id(aid_newtabpage_tvars);
    if (tp->tp_vars == NULL)
    {
	vim_free(tp);
	return NULL;
    }
    init_var_dict(tp->tp_vars, &tp->tp_winvar, VAR_SCOPE);
# endif

# ifdef FEAT_GUI
    for (i = 0; i < 3; i++)
	tp->tp_prev_which_scrollbars[i] = -1;
# endif
# ifdef FEAT_DIFF
    tp->tp_diff_invalid = TRUE;
# endif
    tp->tp_ch_used = p_ch;

    return tp;
}

    void
free_tabpage(tabpage_T *tp)
{
    int idx;

# ifdef FEAT_DIFF
    diff_clear(tp);
# endif
# ifdef FEAT_PROP_POPUP
    while (tp->tp_first_popupwin != NULL)
	popup_close_tabpage(tp, tp->tp_first_popupwin->w_id, TRUE);
#endif
    for (idx = 0; idx < SNAP_COUNT; ++idx)
	clear_snapshot(tp, idx);
#ifdef FEAT_EVAL
    vars_clear(&tp->tp_vars->dv_hashtab);	// free all t: variables
    hash_init(&tp->tp_vars->dv_hashtab);
    unref_var_dict(tp->tp_vars);
#endif

    if (tp == lastused_tabpage)
	lastused_tabpage = NULL;

    vim_free(tp->tp_localdir);
    vim_free(tp->tp_prevdir);

#ifdef FEAT_PYTHON
    python_tabpage_free(tp);
#endif

#ifdef FEAT_PYTHON3
    python3_tabpage_free(tp);
#endif

    vim_free(tp);
}

/*
 * Create a new Tab page with one window.
 * It will edit the current buffer, like after ":split".
 * When "after" is 0 put it just after the current Tab page.
 * Otherwise put it just before tab page "after".
 * Return FAIL or OK.
 */
    int
win_new_tabpage(int after)
{
    tabpage_T	*tp = curtab;
    tabpage_T	*prev_tp = curtab;
    tabpage_T	*newtp;
    int		n;

    if (cmdwin_type != 0)
    {
	emsg(_(e_invalid_in_cmdline_window));
	return FAIL;
    }
    if (window_layout_locked(CMD_tabnew))
	return FAIL;

    newtp = alloc_tabpage();
    if (newtp == NULL)
	return FAIL;

    // Remember the current windows in this Tab page.
    if (leave_tabpage(curbuf, TRUE) == FAIL)
    {
	vim_free(newtp);
	return FAIL;
    }
    curtab = newtp;

    newtp->tp_localdir = (tp->tp_localdir == NULL)
				    ? NULL : vim_strsave(tp->tp_localdir);

    trigger_winnewpre();

    // Create a new empty window.
    if (win_alloc_firstwin(tp->tp_curwin) == OK)
    {
	// Make the new Tab page the new topframe.
	if (after == 1)
	{
	    // New tab page becomes the first one.
	    newtp->tp_next = first_tabpage;
	    first_tabpage = newtp;
	}
	else
	{
	    if (after > 0)
	    {
		// Put new tab page before tab page "after".
		n = 2;
		for (tp = first_tabpage; tp->tp_next != NULL
					       && n < after; tp = tp->tp_next)
		    ++n;
	    }
	    newtp->tp_next = tp->tp_next;
	    tp->tp_next = newtp;
	}
	newtp->tp_firstwin = newtp->tp_lastwin = newtp->tp_curwin = curwin;

	win_init_size();
	firstwin->w_winrow = tabline_height();
	firstwin->w_prev_winrow = firstwin->w_winrow;
	win_comp_scroll(curwin);

	newtp->tp_topframe = topframe;
	last_status(FALSE);

	lastused_tabpage = prev_tp;

#if defined(FEAT_GUI)
	// When 'guioptions' includes 'L' or 'R' may have to remove or add
	// scrollbars.  Have to update them anyway.
	gui_may_update_scrollbars();
#endif
#ifdef FEAT_JOB_CHANNEL
	entering_window(curwin);
#endif

	redraw_all_later(UPD_NOT_VALID);
	apply_autocmds(EVENT_WINNEW, NULL, NULL, FALSE, curbuf);
	apply_autocmds(EVENT_WINENTER, NULL, NULL, FALSE, curbuf);
	apply_autocmds(EVENT_TABNEW, NULL, NULL, FALSE, curbuf);
	apply_autocmds(EVENT_TABENTER, NULL, NULL, FALSE, curbuf);
	return OK;
    }

    // Failed, get back the previous Tab page
    enter_tabpage(curtab, curbuf, TRUE, TRUE);
    return FAIL;
}

/*
 * Open a new tab page if ":tab cmd" was used.  It will edit the same buffer,
 * like with ":split".
 * Returns OK if a new tab page was created, FAIL otherwise.
 */
    static int
may_open_tabpage(void)
{
    int		n = (cmdmod.cmod_tab == 0)
				       ? postponed_split_tab : cmdmod.cmod_tab;

    if (n == 0)
	return FAIL;

    cmdmod.cmod_tab = 0;	    // reset it to avoid doing it twice
    postponed_split_tab = 0;
    return win_new_tabpage(n);
}

/*
 * Create up to "maxcount" tabpages with empty windows.
 * Returns the number of resulting tab pages.
 */
    int
make_tabpages(int maxcount)
{
    int		count = maxcount;
    int		todo;

    // Limit to 'tabpagemax' tabs.
    if (count > p_tpm)
	count = p_tpm;

    /*
     * Don't execute autocommands while creating the tab pages.  Must do that
     * when putting the buffers in the windows.
     */
    block_autocmds();

    for (todo = count - 1; todo > 0; --todo)
	if (win_new_tabpage(0) == FAIL)
	    break;

    unblock_autocmds();

    // return actual number of tab pages
    return (count - todo);
}

/*
 * Return TRUE when "tpc" points to a valid tab page.
 */
    int
valid_tabpage(tabpage_T *tpc)
{
    tabpage_T	*tp;

    FOR_ALL_TABPAGES(tp)
	if (tp == tpc)
	    return TRUE;
    return FALSE;
}

/*
 * Return TRUE when "tpc" points to a valid tab page and at least one window is
 * valid.
 */
    int
valid_tabpage_win(tabpage_T *tpc)
{
    tabpage_T	*tp;
    win_T	*wp;

    FOR_ALL_TABPAGES(tp)
    {
	if (tp == tpc)
	{
	    FOR_ALL_WINDOWS_IN_TAB(tp, wp)
	    {
		if (win_valid_any_tab(wp))
		    return TRUE;
	    }
	    return FALSE;
	}
    }
    // shouldn't happen
    return FALSE;
}

/*
 * Close tabpage "tab", assuming it has no windows in it.
 * There must be another tabpage or this will crash.
 */
    void
close_tabpage(tabpage_T *tab)
{
    tabpage_T	*ptp;

    if (tab == first_tabpage)
    {
	first_tabpage = tab->tp_next;
	ptp = first_tabpage;
    }
    else
    {
	for (ptp = first_tabpage; ptp != NULL && ptp->tp_next != tab;
							    ptp = ptp->tp_next)
	    ;
	assert(ptp != NULL);
	ptp->tp_next = tab->tp_next;
    }

    goto_tabpage_tp(ptp, FALSE, FALSE);
    free_tabpage(tab);
}

/*
 * Find tab page "n" (first one is 1).  Returns NULL when not found.
 */
    tabpage_T *
find_tabpage(int n)
{
    tabpage_T	*tp;
    int		i = 1;

    if (n == 0)
	return curtab;

    for (tp = first_tabpage; tp != NULL && i != n; tp = tp->tp_next)
	++i;
    return tp;
}

/*
 * Get index of tab page "tp".  First one has index 1.
 * When not found returns number of tab pages plus one.
 */
    int
tabpage_index(tabpage_T *ftp)
{
    int		i = 1;
    tabpage_T	*tp;

    for (tp = first_tabpage; tp != NULL && tp != ftp; tp = tp->tp_next)
	++i;
    return i;
}

/*
 * Prepare for leaving the current tab page.
 * When autocommands change "curtab" we don't leave the tab page and return
 * FAIL.
 * Careful: When OK is returned need to get a new tab page very very soon!
 */
    static int
leave_tabpage(
    buf_T	*new_curbuf UNUSED,    // what is going to be the new curbuf,
				       // NULL if unknown
    int		trigger_leave_autocmds)
{
    tabpage_T	*tp = curtab;

#ifdef FEAT_JOB_CHANNEL
    leaving_window(curwin);
#endif
    reset_VIsual_and_resel();	// stop Visual mode
    if (trigger_leave_autocmds)
    {
	if (new_curbuf != curbuf)
	{
	    apply_autocmds(EVENT_BUFLEAVE, NULL, NULL, FALSE, curbuf);
	    if (curtab != tp)
		return FAIL;
	}
	apply_autocmds(EVENT_WINLEAVE, NULL, NULL, FALSE, curbuf);
	if (curtab != tp)
	    return FAIL;
	apply_autocmds(EVENT_TABLEAVE, NULL, NULL, FALSE, curbuf);
	if (curtab != tp)
	    return FAIL;
    }

    reset_dragwin();
#if defined(FEAT_GUI)
    // Remove the scrollbars.  They may be added back later.
    if (gui.in_use)
	gui_remove_scrollbars();
#endif
    tp->tp_curwin = curwin;
    tp->tp_prevwin = prevwin;
    tp->tp_firstwin = firstwin;
    tp->tp_lastwin = lastwin;
    tp->tp_old_Rows = Rows;
    if (tp->tp_old_Columns != -1)
	tp->tp_old_Columns = Columns;
    firstwin = NULL;
    lastwin = NULL;
    return OK;
}

/*
 * Start using tab page "tp".
 * Only to be used after leave_tabpage() or freeing the current tab page.
 * Only trigger *Enter autocommands when trigger_enter_autocmds is TRUE.
 * Only trigger *Leave autocommands when trigger_leave_autocmds is TRUE.
 */
    static void
enter_tabpage(
    tabpage_T	*tp,
    buf_T	*old_curbuf UNUSED,
    int		trigger_enter_autocmds,
    int		trigger_leave_autocmds)
{
    int		row;
    int		old_off = tp->tp_firstwin->w_winrow;
    win_T	*next_prevwin = tp->tp_prevwin;
    tabpage_T	*last_tab = curtab;

    use_tabpage(tp);

    // We would like doing the TabEnter event first, but we don't have a
    // valid current window yet, which may break some commands.
    // This triggers autocommands, thus may make "tp" invalid.
    (void)win_enter_ext(tp->tp_curwin, WEE_CURWIN_INVALID
		  | (trigger_enter_autocmds ? WEE_TRIGGER_ENTER_AUTOCMDS : 0)
		  | (trigger_leave_autocmds ? WEE_TRIGGER_LEAVE_AUTOCMDS : 0));
    prevwin = next_prevwin;

    last_status(FALSE);		// status line may appear or disappear
    row = win_comp_pos();	// recompute w_winrow for all windows
#ifdef FEAT_DIFF
    diff_need_scrollbind = TRUE;
#endif

    // Use the stored value of p_ch, so that it can be different for each tab
    // page.
    if (p_ch != curtab->tp_ch_used)
	clear_cmdline = TRUE;
    p_ch = curtab->tp_ch_used;

    // When cmdheight is changed in a tab page with '<C-w>-', cmdline_row is
    // changed but p_ch and tp_ch_used are not changed. Thus we also need to
    // check cmdline_row.
    if (row < cmdline_row && cmdline_row <= Rows - p_ch)
	clear_cmdline = TRUE;

    // If there was a click in a window, it won't be usable for a following
    // drag.
    reset_dragwin();

    // The tabpage line may have appeared or disappeared, may need to resize
    // the frames for that.  When the Vim window was resized need to update
    // frame sizes too.
    if (curtab->tp_old_Rows != Rows || (old_off != firstwin->w_winrow
#ifdef FEAT_GUI_TABLINE
			    && !gui_use_tabline()
#endif
		))
	shell_new_rows();
    if (curtab->tp_old_Columns != Columns)
    {
	if (starting == 0)
	{
	    shell_new_columns();	// update window widths
	    curtab->tp_old_Columns = Columns;
	}
	else
	    curtab->tp_old_Columns = -1;  // update window widths later
    }

    lastused_tabpage = last_tab;

#if defined(FEAT_GUI)
    // When 'guioptions' includes 'L' or 'R' may have to remove or add
    // scrollbars.  Have to update them anyway.
    gui_may_update_scrollbars();
#endif

    // Apply autocommands after updating the display, when 'rows' and
    // 'columns' have been set correctly.
    if (trigger_enter_autocmds)
    {
	apply_autocmds(EVENT_TABENTER, NULL, NULL, FALSE, curbuf);
	if (old_curbuf != curbuf)
	    apply_autocmds(EVENT_BUFENTER, NULL, NULL, FALSE, curbuf);
    }

    redraw_all_later(UPD_NOT_VALID);
}

/*
 * Go to tab page "n".  For ":tab N" and "Ngt".
 * When "n" is 9999 go to the last tab page.
 */
    void
goto_tabpage(int n)
{
    tabpage_T	*tp = NULL;  // shut up compiler
    tabpage_T	*ttp;
    int		i;

    if (text_locked())
    {
	// Not allowed when editing the command line.
	text_locked_msg();
	return;
    }

    // If there is only one it can't work.
    if (first_tabpage->tp_next == NULL)
    {
	if (n > 1)
	    beep_flush();
	return;
    }

    if (n == 0)
    {
	// No count, go to next tab page, wrap around end.
	if (curtab->tp_next == NULL)
	    tp = first_tabpage;
	else
	    tp = curtab->tp_next;
    }
    else if (n < 0)
    {
	// "gT": go to previous tab page, wrap around end.  "N gT" repeats
	// this N times.
	ttp = curtab;
	for (i = n; i < 0; ++i)
	{
	    for (tp = first_tabpage; tp->tp_next != ttp && tp->tp_next != NULL;
		    tp = tp->tp_next)
		;
	    ttp = tp;
	}
    }
    else if (n == 9999)
    {
	// Go to last tab page.
	for (tp = first_tabpage; tp->tp_next != NULL; tp = tp->tp_next)
	    ;
    }
    else
    {
	// Go to tab page "n".
	tp = find_tabpage(n);
	if (tp == NULL)
	{
	    beep_flush();
	    return;
	}
    }

    goto_tabpage_tp(tp, TRUE, TRUE);

#ifdef FEAT_GUI_TABLINE
    if (gui_use_tabline())
	gui_mch_set_curtab(tabpage_index(curtab));
#endif
}

/*
 * Go to tabpage "tp".
 * Only trigger *Enter autocommands when trigger_enter_autocmds is TRUE.
 * Only trigger *Leave autocommands when trigger_leave_autocmds is TRUE.
 * Note: doesn't update the GUI tab.
 */
    void
goto_tabpage_tp(
    tabpage_T	*tp,
    int		trigger_enter_autocmds,
    int		trigger_leave_autocmds)
{
    if (trigger_enter_autocmds || trigger_leave_autocmds)
	CHECK_CMDWIN;

    // Don't repeat a message in another tab page.
    set_keep_msg(NULL, 0);

    skip_win_fix_scroll = TRUE;
    if (tp != curtab && leave_tabpage(tp->tp_curwin->w_buffer,
					trigger_leave_autocmds) == OK)
    {
	if (valid_tabpage(tp))
	    enter_tabpage(tp, curbuf, trigger_enter_autocmds,
		    trigger_leave_autocmds);
	else
	    enter_tabpage(curtab, curbuf, trigger_enter_autocmds,
		    trigger_leave_autocmds);
    }
    skip_win_fix_scroll = FALSE;
}

/*
 * Go to the last accessed tab page, if there is one.
 * Return OK or FAIL
 */
    int
goto_tabpage_lastused(void)
{
    if (!valid_tabpage(lastused_tabpage))
	return FAIL;

    goto_tabpage_tp(lastused_tabpage, TRUE, TRUE);
    return OK;
}

/*
 * Enter window "wp" in tab page "tp".
 * Also updates the GUI tab.
 */
    void
goto_tabpage_win(tabpage_T *tp, win_T *wp)
{
    goto_tabpage_tp(tp, TRUE, TRUE);
    if (curtab == tp && win_valid(wp))
    {
	win_enter(wp, TRUE);
# ifdef FEAT_GUI_TABLINE
	if (gui_use_tabline())
	    gui_mch_set_curtab(tabpage_index(curtab));
# endif
    }
}

/*
 * Move the current tab page to after tab page "nr".
 */
    void
tabpage_move(int nr)
{
    int		n = 1;
    tabpage_T	*tp, *tp_dst;

    if (first_tabpage->tp_next == NULL)
	return;

    if (tabpage_move_disallowed)
	return;

    for (tp = first_tabpage; tp->tp_next != NULL && n < nr; tp = tp->tp_next)
	++n;

    if (tp == curtab || (nr > 0 && tp->tp_next != NULL
						    && tp->tp_next == curtab))
	return;

    tp_dst = tp;

    // Remove the current tab page from the list of tab pages.
    if (curtab == first_tabpage)
	first_tabpage = curtab->tp_next;
    else
    {
	FOR_ALL_TABPAGES(tp)
	    if (tp->tp_next == curtab)
		break;
	if (tp == NULL)	// "cannot happen"
	    return;
	tp->tp_next = curtab->tp_next;
    }

    // Re-insert it at the specified position.
    if (nr <= 0)
    {
	curtab->tp_next = first_tabpage;
	first_tabpage = curtab;
    }
    else
    {
	curtab->tp_next = tp_dst->tp_next;
	tp_dst->tp_next = curtab;
    }

    // Need to redraw the tabline.  Tab page contents doesn't change.
    redraw_tabline = TRUE;
}


/*
 * Go to another window.
 * When jumping to another buffer, stop Visual mode.  Do this before
 * changing windows so we can yank the selection into the '*' register.
 * (note: this may trigger ModeChanged autocommand!)
 * When jumping to another window on the same buffer, adjust its cursor
 * position to keep the same Visual area.
 */
    void
win_goto(win_T *wp)
{
#ifdef FEAT_CONCEAL
    win_T	*owp = curwin;
#endif

#ifdef FEAT_PROP_POPUP
    if (ERROR_IF_ANY_POPUP_WINDOW)
	return;
    if (popup_is_popup(wp))
    {
	emsg(_(e_not_allowed_to_enter_popup_window));
	return;
    }
#endif
    if (text_or_buf_locked())
    {
	beep_flush();
	return;
    }

    if (wp->w_buffer != curbuf)
	// careful: triggers ModeChanged autocommand
	reset_VIsual_and_resel();
    else if (VIsual_active)
	wp->w_cursor = curwin->w_cursor;

    // autocommand may have made wp invalid
    if (!win_valid(wp))
	return;

#ifdef FEAT_GUI
    need_mouse_correct = TRUE;
#endif
    win_enter(wp, TRUE);

#ifdef FEAT_CONCEAL
    // Conceal cursor line in previous window, unconceal in current window.
    if (win_valid(owp) && owp->w_p_cole > 0 && !msg_scrolled)
	redrawWinline(owp, owp->w_cursor.lnum);
    if (curwin->w_p_cole > 0 && !msg_scrolled)
	need_cursor_line_redraw = TRUE;
#endif
}

#if defined(FEAT_PERL) || defined(PROTO)
/*
 * Find window number "winnr" (counting top to bottom).
 */
    win_T *
win_find_nr(int winnr)
{
    win_T	*wp;

    FOR_ALL_WINDOWS(wp)
	if (--winnr == 0)
	    break;
    return wp;
}
#endif

#if ((defined(FEAT_PYTHON) || defined(FEAT_PYTHON3))) || defined(PROTO)
/*
 * Find the tabpage for window "win".
 */
    tabpage_T *
win_find_tabpage(win_T *win)
{
    win_T	*wp;
    tabpage_T	*tp;

    FOR_ALL_TAB_WINDOWS(tp, wp)
	    if (wp == win)
		return tp;
    return NULL;
}
#endif

/*
 * Get the above or below neighbor window of the specified window.
 *   up - TRUE for the above neighbor
 *   count - nth neighbor window
 * Returns the specified window if the neighbor is not found.
 */
    win_T *
win_vert_neighbor(tabpage_T *tp, win_T *wp, int up, long count)
{
    frame_T	*fr;
    frame_T	*nfr;
    frame_T	*foundfr;

#ifdef FEAT_PROP_POPUP
    if (popup_is_popup(wp))
	// popups don't have neighbors.
	return NULL;
#endif
    foundfr = wp->w_frame;
    while (count--)
    {
	/*
	 * First go upwards in the tree of frames until we find a upwards or
	 * downwards neighbor.
	 */
	fr = foundfr;
	for (;;)
	{
	    if (fr == tp->tp_topframe)
		goto end;
	    if (up)
		nfr = fr->fr_prev;
	    else
		nfr = fr->fr_next;
	    if (fr->fr_parent->fr_layout == FR_COL && nfr != NULL)
		break;
	    fr = fr->fr_parent;
	}

	/*
	 * Now go downwards to find the bottom or top frame in it.
	 */
	for (;;)
	{
	    if (nfr->fr_layout == FR_LEAF)
	    {
		foundfr = nfr;
		break;
	    }
	    fr = nfr->fr_child;
	    if (nfr->fr_layout == FR_ROW)
	    {
		// Find the frame at the cursor row.
		while (fr->fr_next != NULL
			&& frame2win(fr)->w_wincol + fr->fr_width
					 <= wp->w_wincol + wp->w_wcol)
		    fr = fr->fr_next;
	    }
	    if (nfr->fr_layout == FR_COL && up)
		while (fr->fr_next != NULL)
		    fr = fr->fr_next;
	    nfr = fr;
	}
    }
end:
    return foundfr != NULL ? foundfr->fr_win : NULL;
}

/*
 * Move to window above or below "count" times.
 */
    static void
win_goto_ver(
    int		up,		// TRUE to go to win above
    long	count)
{
    win_T	*win;

#ifdef FEAT_PROP_POPUP
    if (ERROR_IF_TERM_POPUP_WINDOW)
	return;
#endif
    win = win_vert_neighbor(curtab, curwin, up, count);
    if (win != NULL)
	win_goto(win);
}

/*
 * Get the left or right neighbor window of the specified window.
 *   left - TRUE for the left neighbor
 *   count - nth neighbor window
 * Returns the specified window if the neighbor is not found.
 */
    win_T *
win_horz_neighbor(tabpage_T *tp, win_T *wp, int left, long count)
{
    frame_T	*fr;
    frame_T	*nfr;
    frame_T	*foundfr;

#ifdef FEAT_PROP_POPUP
    if (popup_is_popup(wp))
	// popups don't have neighbors.
	return NULL;
#endif
    foundfr = wp->w_frame;
    while (count--)
    {
	/*
	 * First go upwards in the tree of frames until we find a left or
	 * right neighbor.
	 */
	fr = foundfr;
	for (;;)
	{
	    if (fr == tp->tp_topframe)
		goto end;
	    if (left)
		nfr = fr->fr_prev;
	    else
		nfr = fr->fr_next;
	    if (fr->fr_parent->fr_layout == FR_ROW && nfr != NULL)
		break;
	    fr = fr->fr_parent;
	}

	/*
	 * Now go downwards to find the leftmost or rightmost frame in it.
	 */
	for (;;)
	{
	    if (nfr->fr_layout == FR_LEAF)
	    {
		foundfr = nfr;
		break;
	    }
	    fr = nfr->fr_child;
	    if (nfr->fr_layout == FR_COL)
	    {
		// Find the frame at the cursor row.
		while (fr->fr_next != NULL
			&& frame2win(fr)->w_winrow + fr->fr_height
					 <= wp->w_winrow + wp->w_wrow)
		    fr = fr->fr_next;
	    }
	    if (nfr->fr_layout == FR_ROW && left)
		while (fr->fr_next != NULL)
		    fr = fr->fr_next;
	    nfr = fr;
	}
    }
end:
    return foundfr != NULL ? foundfr->fr_win : NULL;
}

/*
 * Move to left or right window.
 */
    static void
win_goto_hor(
    int		left,		// TRUE to go to left win
    long	count)
{
    win_T	*win;

#ifdef FEAT_PROP_POPUP
    if (ERROR_IF_TERM_POPUP_WINDOW)
	return;
#endif
    win = win_horz_neighbor(curtab, curwin, left, count);
    if (win != NULL)
	win_goto(win);
}

/*
 * Make window "wp" the current window.
 */
    void
win_enter(win_T *wp, int undo_sync)
{
    (void)win_enter_ext(wp, (undo_sync ? WEE_UNDO_SYNC : 0)
		    | WEE_TRIGGER_ENTER_AUTOCMDS | WEE_TRIGGER_LEAVE_AUTOCMDS);
}

/*
 * Used after making another window the current one: change directory if
 * needed.
 */
    static void
fix_current_dir(void)
{
    if (curwin->w_localdir != NULL || curtab->tp_localdir != NULL)
    {
	char_u	*dirname;

	// Window or tab has a local directory: Save current directory as
	// global directory (unless that was done already) and change to the
	// local directory.
	if (globaldir == NULL)
	{
	    char_u	cwd[MAXPATHL];

	    if (mch_dirname(cwd, MAXPATHL) == OK)
		globaldir = vim_strsave(cwd);
	}
	if (curwin->w_localdir != NULL)
	    dirname = curwin->w_localdir;
	else
	    dirname = curtab->tp_localdir;

	if (mch_chdir((char *)dirname) == 0)
	{
	    last_chdir_reason = NULL;
	    shorten_fnames(TRUE);
	}
    }
    else if (globaldir != NULL)
    {
	// Window doesn't have a local directory and we are not in the global
	// directory: Change to the global directory.
	vim_ignored = mch_chdir((char *)globaldir);
	VIM_CLEAR(globaldir);
	last_chdir_reason = NULL;
	shorten_fnames(TRUE);
    }
}

/*
 * Make window "wp" the current window.
 * Can be called with "flags" containing WEE_CURWIN_INVALID, which means that
 * curwin has just been closed and isn't valid.
 * Returns TRUE when dont_parse_messages was decremented.
 */
    static int
win_enter_ext(win_T *wp, int flags)
{
    int		other_buffer = FALSE;
    int		curwin_invalid = (flags & WEE_CURWIN_INVALID);
    int		did_decrement = FALSE;

    if (wp == curwin && curwin_invalid == 0)	// nothing to do
	return FALSE;

#ifdef FEAT_JOB_CHANNEL
    if (curwin_invalid == 0)
	leaving_window(curwin);
#endif

    if (curwin_invalid == 0 && (flags & WEE_TRIGGER_LEAVE_AUTOCMDS))
    {
	/*
	 * Be careful: If autocommands delete the window, return now.
	 */
	if (wp->w_buffer != curbuf)
	{
	    apply_autocmds(EVENT_BUFLEAVE, NULL, NULL, FALSE, curbuf);
	    other_buffer = TRUE;
	    if (!win_valid(wp))
		return FALSE;
	}
	apply_autocmds(EVENT_WINLEAVE, NULL, NULL, FALSE, curbuf);
	if (!win_valid(wp))
	    return FALSE;
#ifdef FEAT_EVAL
	// autocmds may abort script processing
	if (aborting())
	    return FALSE;
#endif
    }

    // sync undo before leaving the current buffer
    if ((flags & WEE_UNDO_SYNC) && curbuf != wp->w_buffer)
	u_sync(FALSE);

    // Might need to scroll the old window before switching, e.g., when the
    // cursor was moved.
    if (*p_spk == 'c' && curwin_invalid == 0)
	update_topline();

    // may have to copy the buffer options when 'cpo' contains 'S'
    if (wp->w_buffer != curbuf)
	buf_copy_options(wp->w_buffer, BCO_ENTER | BCO_NOHELP);

    if (curwin_invalid == 0)
    {
	prevwin = curwin;	// remember for CTRL-W p
	curwin->w_redr_status = TRUE;
    }
    else if (wp == prevwin)
	prevwin = NULL;		// don't want it to be the new curwin

    curwin = wp;
    curbuf = wp->w_buffer;
    check_cursor();
    if (!virtual_active())
	curwin->w_cursor.coladd = 0;
    if (*p_spk == 'c')		// assume cursor position needs updating
	changed_line_abv_curs();
    else
	// Make sure the cursor position is valid, either by moving the cursor
	// or by scrolling the text.
	win_fix_cursor(
		get_real_state() & (MODE_NORMAL|MODE_CMDLINE|MODE_TERMINAL));

    // Now it is OK to parse messages again, which may be needed in
    // autocommands.
#ifdef MESSAGE_QUEUE
    if (flags & WEE_ALLOW_PARSE_MESSAGES)
    {
	--dont_parse_messages;
	did_decrement = TRUE;
    }
#endif

    fix_current_dir();

#ifdef FEAT_JOB_CHANNEL
    entering_window(curwin);
#endif
    // Careful: autocommands may close the window and make "wp" invalid
    if (flags & WEE_TRIGGER_NEW_AUTOCMDS)
	apply_autocmds(EVENT_WINNEW, NULL, NULL, FALSE, curbuf);
    if (flags & WEE_TRIGGER_ENTER_AUTOCMDS)
    {
	apply_autocmds(EVENT_WINENTER, NULL, NULL, FALSE, curbuf);
	if (other_buffer)
	    apply_autocmds(EVENT_BUFENTER, NULL, NULL, FALSE, curbuf);
    }

    maketitle();
    curwin->w_redr_status = TRUE;
#ifdef FEAT_TERMINAL
    if (bt_terminal(curwin->w_buffer))
	// terminal is likely in another mode
	redraw_mode = TRUE;
#endif
    redraw_tabline = TRUE;
    if (restart_edit)
	redraw_later(UPD_VALID);	// causes status line redraw

    // set window height to desired minimal value
    if (curwin->w_height < p_wh && !curwin->w_p_wfh
#ifdef FEAT_PROP_POPUP
	    && !popup_is_popup(curwin)
#endif
	    )
	win_setheight((int)p_wh);
    else if (curwin->w_height == 0)
	win_setheight(1);

    // set window width to desired minimal value
    if (curwin->w_width < p_wiw && !curwin->w_p_wfw)
	win_setwidth((int)p_wiw);

    setmouse();			// in case jumped to/from help buffer

    // Change directories when the 'acd' option is set.
    DO_AUTOCHDIR;

    return did_decrement;
}

/*
 * Jump to the first open window that contains buffer "buf", if one exists.
 * Returns a pointer to the window found, otherwise NULL.
 */
    win_T *
buf_jump_open_win(buf_T *buf)
{
    win_T	*wp = NULL;

    if (curwin->w_buffer == buf)
	wp = curwin;
    else
	FOR_ALL_WINDOWS(wp)
	    if (wp->w_buffer == buf)
		break;
    if (wp != NULL)
	win_enter(wp, FALSE);
    return wp;
}

/*
 * Jump to the first open window in any tab page that contains buffer "buf",
 * if one exists. First search in the windows present in the current tab page.
 * Returns a pointer to the window found, otherwise NULL.
 */
    win_T *
buf_jump_open_tab(buf_T *buf)
{
    win_T	*wp = buf_jump_open_win(buf);
    tabpage_T	*tp;

    if (wp != NULL)
	return wp;

    FOR_ALL_TABPAGES(tp)
	if (tp != curtab)
	{
	    FOR_ALL_WINDOWS_IN_TAB(tp, wp)
		if (wp->w_buffer == buf)
		    break;
	    if (wp != NULL)
	    {
		goto_tabpage_win(tp, wp);
		if (curwin != wp)
		    wp = NULL;	// something went wrong
		break;
	    }
	}
    return wp;
}

static int last_win_id = LOWEST_WIN_ID - 1;

/*
 * Allocate a window structure and link it in the window list when "hidden" is
 * FALSE.
 */
    static win_T *
win_alloc(win_T *after, int hidden)
{
    win_T	*new_wp;

    /*
     * allocate window structure and linesizes arrays
     */
    new_wp = ALLOC_CLEAR_ONE(win_T);
    if (new_wp == NULL)
	return NULL;

    if (win_alloc_lines(new_wp) == FAIL)
    {
	vim_free(new_wp);
	return NULL;
    }

    new_wp->w_id = ++last_win_id;

#ifdef FEAT_EVAL
    // init w: variables
    new_wp->w_vars = dict_alloc_id(aid_newwin_wvars);
    if (new_wp->w_vars == NULL)
    {
	win_free_lsize(new_wp);
	vim_free(new_wp);
	return NULL;
    }
    init_var_dict(new_wp->w_vars, &new_wp->w_winvar, VAR_SCOPE);
#endif

    // Don't execute autocommands while the window is not properly
    // initialized yet.  gui_create_scrollbar() may trigger a FocusGained
    // event.
    block_autocmds();

    /*
     * link the window in the window list
     */
    if (!hidden)
	win_append(after, new_wp);
    new_wp->w_wincol = 0;
    new_wp->w_width = Columns;

    // position the display and the cursor at the top of the file.
    new_wp->w_topline = 1;
#ifdef FEAT_DIFF
    new_wp->w_topfill = 0;
#endif
    new_wp->w_botline = 2;
    new_wp->w_cursor.lnum = 1;
    new_wp->w_scbind_pos = 1;

    // use global option value for global-local options
    new_wp->w_allbuf_opt.wo_so = new_wp->w_p_so = -1;
    new_wp->w_allbuf_opt.wo_siso = new_wp->w_p_siso = -1;

    // We won't calculate w_fraction until resizing the window
    new_wp->w_fraction = 0;
    new_wp->w_prev_fraction_row = -1;

#ifdef FEAT_GUI
    if (gui.in_use)
    {
	gui_create_scrollbar(&new_wp->w_scrollbars[SBAR_LEFT],
		SBAR_LEFT, new_wp);
	gui_create_scrollbar(&new_wp->w_scrollbars[SBAR_RIGHT],
		SBAR_RIGHT, new_wp);
    }
#endif
#ifdef FEAT_FOLDING
    foldInitWin(new_wp);
#endif
    unblock_autocmds();
#ifdef FEAT_SEARCH_EXTRA
    new_wp->w_next_match_id = 1000;  // up to 1000 can be picked by the user
#endif
    return new_wp;
}

/*
 * Remove window 'wp' from the window list and free the structure.
 */
    static void
win_free(
    win_T	*wp,
    tabpage_T	*tp)		// tab page "win" is in, NULL for current
{
    int		i;
    buf_T	*buf;
    wininfo_T	*wip;

#ifdef FEAT_FOLDING
    clearFolding(wp);
#endif

    // reduce the reference count to the argument list.
    alist_unlink(wp->w_alist);

    // Don't execute autocommands while the window is halfway being deleted.
    // gui_mch_destroy_scrollbar() may trigger a FocusGained event.
    block_autocmds();

#ifdef FEAT_LUA
    lua_window_free(wp);
#endif

#ifdef FEAT_MZSCHEME
    mzscheme_window_free(wp);
#endif

#ifdef FEAT_PERL
    perl_win_free(wp);
#endif

#ifdef FEAT_PYTHON
    python_window_free(wp);
#endif

#ifdef FEAT_PYTHON3
    python3_window_free(wp);
#endif

#ifdef FEAT_TCL
    tcl_window_free(wp);
#endif

#ifdef FEAT_RUBY
    ruby_window_free(wp);
#endif

    clear_winopt(&wp->w_onebuf_opt);
    clear_winopt(&wp->w_allbuf_opt);

    vim_free(wp->w_lcs_chars.multispace);
    vim_free(wp->w_lcs_chars.leadmultispace);

#ifdef FEAT_EVAL
    vars_clear(&wp->w_vars->dv_hashtab);	// free all w: variables
    hash_init(&wp->w_vars->dv_hashtab);
    unref_var_dict(wp->w_vars);
#endif

    {
	tabpage_T	*ttp;

	if (prevwin == wp)
	    prevwin = NULL;
	FOR_ALL_TABPAGES(ttp)
	    if (ttp->tp_prevwin == wp)
		ttp->tp_prevwin = NULL;
    }
    win_free_lsize(wp);

    for (i = 0; i < wp->w_tagstacklen; ++i)
    {
	vim_free(wp->w_tagstack[i].tagname);
	vim_free(wp->w_tagstack[i].user_data);
    }
    vim_free(wp->w_localdir);
    vim_free(wp->w_prevdir);

    // Remove the window from the b_wininfo lists, it may happen that the
    // freed memory is re-used for another window.
    FOR_ALL_BUFFERS(buf)
	FOR_ALL_BUF_WININFO(buf, wip)
	    if (wip->wi_win == wp)
	    {
		wininfo_T	*wip2;

		// If there already is an entry with "wi_win" set to NULL it
		// must be removed, it would never be used.
		// Skip "wip" itself, otherwise Coverity complains.
		FOR_ALL_BUF_WININFO(buf, wip2)
		    if (wip2 != wip && wip2->wi_win == NULL)
		    {
			if (wip2->wi_next != NULL)
			    wip2->wi_next->wi_prev = wip2->wi_prev;
			if (wip2->wi_prev == NULL)
			    buf->b_wininfo = wip2->wi_next;
			else
			    wip2->wi_prev->wi_next = wip2->wi_next;
			free_wininfo(wip2);
			break;
		    }

		wip->wi_win = NULL;
	    }

#ifdef FEAT_SEARCH_EXTRA
    clear_matches(wp);
#endif

    free_jumplist(wp);

#ifdef FEAT_QUICKFIX
    qf_free_all(wp);
#endif

#ifdef FEAT_GUI
    if (gui.in_use)
    {
	gui_mch_destroy_scrollbar(&wp->w_scrollbars[SBAR_LEFT]);
	gui_mch_destroy_scrollbar(&wp->w_scrollbars[SBAR_RIGHT]);
    }
#endif // FEAT_GUI

#ifdef FEAT_MENU
    remove_winbar(wp);
#endif
#ifdef FEAT_PROP_POPUP
    free_callback(&wp->w_close_cb);
    free_callback(&wp->w_filter_cb);
    for (i = 0; i < 4; ++i)
	VIM_CLEAR(wp->w_border_highlight[i]);
    vim_free(wp->w_scrollbar_highlight);
    vim_free(wp->w_thumb_highlight);
    vim_free(wp->w_popup_title);
    list_unref(wp->w_popup_mask);
    vim_free(wp->w_popup_mask_cells);
#endif

#ifdef FEAT_SYN_HL
    vim_free(wp->w_p_cc_cols);
#endif

    if (win_valid_any_tab(wp))
	win_remove(wp, tp);
    if (autocmd_busy)
    {
	wp->w_next = au_pending_free_win;
	au_pending_free_win = wp;
    }
    else
	vim_free(wp);

    unblock_autocmds();
}

/*
 * Return TRUE if "wp" is not in the list of windows: the autocmd window or a
 * popup window.
 */
    int
win_unlisted(win_T *wp)
{
    return is_aucmd_win(wp) || WIN_IS_POPUP(wp);
}

#if defined(FEAT_PROP_POPUP) || defined(PROTO)
/*
 * Free a popup window.  This does not take the window out of the window list
 * and assumes there is only one toplevel frame, no split.
 */
    void
win_free_popup(win_T *win)
{
    if (win->w_buffer != NULL)
    {
	if (bt_popup(win->w_buffer))
	    win_close_buffer(win, DOBUF_WIPE_REUSE, FALSE);
	else
	    close_buffer(win, win->w_buffer, 0, FALSE, FALSE);
    }
# if defined(FEAT_TIMERS)
    // the timer may have been cleared, making the pointer invalid
    if (timer_valid(win->w_popup_timer))
	stop_timer(win->w_popup_timer);
# endif
    vim_free(win->w_frame);
    win_free(win, NULL);
}
#endif

/*
 * Append window "wp" in the window list after window "after".
 */
    static void
win_append(win_T *after, win_T *wp)
{
    win_T	*before;

    if (after == NULL)	    // after NULL is in front of the first
	before = firstwin;
    else
	before = after->w_next;

    wp->w_next = before;
    wp->w_prev = after;
    if (after == NULL)
	firstwin = wp;
    else
	after->w_next = wp;
    if (before == NULL)
	lastwin = wp;
    else
	before->w_prev = wp;
}

/*
 * Remove a window from the window list.
 */
    void
win_remove(
    win_T	*wp,
    tabpage_T	*tp)		// tab page "win" is in, NULL for current
{
    if (wp->w_prev != NULL)
	wp->w_prev->w_next = wp->w_next;
    else if (tp == NULL)
	firstwin = curtab->tp_firstwin = wp->w_next;
    else
	tp->tp_firstwin = wp->w_next;

    if (wp->w_next != NULL)
	wp->w_next->w_prev = wp->w_prev;
    else if (tp == NULL)
	lastwin = curtab->tp_lastwin = wp->w_prev;
    else
	tp->tp_lastwin = wp->w_prev;
}

/*
 * Append frame "frp" in a frame list after frame "after".
 */
    static void
frame_append(frame_T *after, frame_T *frp)
{
    frp->fr_next = after->fr_next;
    after->fr_next = frp;
    if (frp->fr_next != NULL)
	frp->fr_next->fr_prev = frp;
    frp->fr_prev = after;
}

/*
 * Insert frame "frp" in a frame list before frame "before".
 */
    static void
frame_insert(frame_T *before, frame_T *frp)
{
    frp->fr_next = before;
    frp->fr_prev = before->fr_prev;
    before->fr_prev = frp;
    if (frp->fr_prev != NULL)
	frp->fr_prev->fr_next = frp;
    else
	frp->fr_parent->fr_child = frp;
}

/*
 * Remove a frame from a frame list.
 */
    static void
frame_remove(frame_T *frp)
{
    if (frp->fr_prev != NULL)
	frp->fr_prev->fr_next = frp->fr_next;
    else
	frp->fr_parent->fr_child = frp->fr_next;
    if (frp->fr_next != NULL)
	frp->fr_next->fr_prev = frp->fr_prev;
}

/*
 * Allocate w_lines[] for window "wp".
 * Return FAIL for failure, OK for success.
 */
    int
win_alloc_lines(win_T *wp)
{
    wp->w_lines_valid = 0;
    wp->w_lines = ALLOC_CLEAR_MULT(wline_T, Rows);
    if (wp->w_lines == NULL)
	return FAIL;
    return OK;
}

/*
 * free lsize arrays for a window
 */
    void
win_free_lsize(win_T *wp)
{
    // TODO: why would wp be NULL here?
    if (wp != NULL)
	VIM_CLEAR(wp->w_lines);
}

/*
 * Called from win_new_shellsize() after Rows changed.
 * This only does the current tab page, others must be done when made active.
 */
    void
shell_new_rows(void)
{
    int		h = (int)ROWS_AVAIL;

    if (firstwin == NULL)	// not initialized yet
	return;
    if (h < frame_minheight(topframe, NULL))
	h = frame_minheight(topframe, NULL);

    // First try setting the heights of windows with 'winfixheight'.  If
    // that doesn't result in the right height, forget about that option.
    frame_new_height(topframe, h, FALSE, TRUE);
    if (!frame_check_height(topframe, h))
	frame_new_height(topframe, h, FALSE, FALSE);

    (void)win_comp_pos();		// recompute w_winrow and w_wincol
    compute_cmdrow();
    curtab->tp_ch_used = p_ch;

    if (!skip_win_fix_scroll)
	win_fix_scroll(TRUE);

#if 0
    // Disabled: don't want making the screen smaller make a window larger.
    if (p_ea)
	win_equal(curwin, FALSE, 'v');
#endif
}

/*
 * Called from win_new_shellsize() after Columns changed.
 */
    void
shell_new_columns(void)
{
    if (firstwin == NULL)	// not initialized yet
	return;

    // First try setting the widths of windows with 'winfixwidth'.  If that
    // doesn't result in the right width, forget about that option.
    frame_new_width(topframe, (int)Columns, FALSE, TRUE);
    if (!frame_check_width(topframe, Columns))
	frame_new_width(topframe, (int)Columns, FALSE, FALSE);

    (void)win_comp_pos();		// recompute w_winrow and w_wincol
#if 0
    // Disabled: don't want making the screen smaller make a window larger.
    if (p_ea)
	win_equal(curwin, FALSE, 'h');
#endif
}

/*
 * Save the size of all windows in "gap".
 */
    void
win_size_save(garray_T *gap)
{
    win_T	*wp;

    ga_init2(gap, sizeof(int), 1);
    if (ga_grow(gap, win_count() * 2 + 1) == FAIL)
	return;

    // first entry is the total lines available for windows
    ((int *)gap->ga_data)[gap->ga_len++] = ROWS_AVAIL - last_stl_height(FALSE);

    FOR_ALL_WINDOWS(wp)
    {
	((int *)gap->ga_data)[gap->ga_len++] =
	    wp->w_width + wp->w_vsep_width;
	((int *)gap->ga_data)[gap->ga_len++] = wp->w_height;
    }
}

/*
 * Restore window sizes, but only if the number of windows is still the same
 * and total lines available for windows didn't change.
 * Does not free the growarray.
 */
    void
win_size_restore(garray_T *gap)
{
    win_T	*wp;
    int		i, j;

    if (win_count() * 2 + 1 == gap->ga_len
	    && ((int *)gap->ga_data)[0] == ROWS_AVAIL - last_stl_height(FALSE))
    {
	// The order matters, because frames contain other frames, but it's
	// difficult to get right. The easy way out is to do it twice.
	for (j = 0; j < 2; ++j)
	{
	    i = 1;
	    FOR_ALL_WINDOWS(wp)
	    {
		frame_setwidth(wp->w_frame, ((int *)gap->ga_data)[i++]);
		win_setheight_win(((int *)gap->ga_data)[i++], wp);
	    }
	}
	// recompute the window positions
	(void)win_comp_pos();
    }
}

/*
 * Update the position for all windows, using the width and height of the
 * frames.
 * Returns the row just after the last window.
 */
    int
win_comp_pos(void)
{
    int		row = tabline_height();
    int		col = 0;

    frame_comp_pos(topframe, &row, &col);
    return row;
}

/*
 * Update the position of the windows in frame "topfrp", using the width and
 * height of the frames.
 * "*row" and "*col" are the top-left position of the frame.  They are updated
 * to the bottom-right position plus one.
 */
    static void
frame_comp_pos(frame_T *topfrp, int *row, int *col)
{
    win_T	*wp;
    frame_T	*frp;
    int		startcol;
    int		startrow;
    int		h;

    wp = topfrp->fr_win;
    if (wp != NULL)
    {
	if (wp->w_winrow != *row || wp->w_wincol != *col)
	{
	    // position changed, redraw
	    wp->w_winrow = *row;
	    wp->w_wincol = *col;
	    redraw_win_later(wp, UPD_NOT_VALID);
	    wp->w_redr_status = TRUE;
	}
	// WinBar will not show if the window height is zero
	h = VISIBLE_HEIGHT(wp) + wp->w_status_height;
	*row += h > topfrp->fr_height ? topfrp->fr_height : h;
	*col += wp->w_width + wp->w_vsep_width;
    }
    else
    {
	startrow = *row;
	startcol = *col;
	FOR_ALL_FRAMES(frp, topfrp->fr_child)
	{
	    if (topfrp->fr_layout == FR_ROW)
		*row = startrow;	// all frames are at the same row
	    else
		*col = startcol;	// all frames are at the same col
	    frame_comp_pos(frp, row, col);
	}
    }
}

/*
 * Make the current window show at least one line and one column.
 */
    void
win_ensure_size(void)
{
    if (curwin->w_height == 0)
	win_setheight(1);
    if (curwin->w_width == 0)
	win_setwidth(1);
}

/*
 * Set current window height and take care of repositioning other windows to
 * fit around it.
 */
    void
win_setheight(int height)
{
    win_setheight_win(height, curwin);
}

/*
 * Set the window height of window "win" and take care of repositioning other
 * windows to fit around it.
 */
    void
win_setheight_win(int height, win_T *win)
{
    int		row;

    if (win == curwin)
    {
	// Always keep current window at least one line high, even when
	// 'winminheight' is zero.
	if (height < p_wmh)
	    height = p_wmh;
	if (height == 0)
	    height = 1;
	height += WINBAR_HEIGHT(curwin);
    }

    frame_setheight(win->w_frame, height + win->w_status_height);

    // recompute the window positions
    row = win_comp_pos();

    /*
     * If there is extra space created between the last window and the command
     * line, clear it.
     */
    if (full_screen && msg_scrolled == 0 && row < cmdline_row)
	screen_fill(row, cmdline_row, 0, (int)Columns, ' ', ' ', 0);
    cmdline_row = row;
    msg_row = row;
    msg_col = 0;

    win_fix_scroll(TRUE);

    redraw_all_later(UPD_NOT_VALID);
}

/*
 * Set the height of a frame to "height" and take care that all frames and
 * windows inside it are resized.  Also resize frames on the left and right if
 * the are in the same FR_ROW frame.
 *
 * Strategy:
 * If the frame is part of a FR_COL frame, try fitting the frame in that
 * frame.  If that doesn't work (the FR_COL frame is too small), recursively
 * go to containing frames to resize them and make room.
 * If the frame is part of a FR_ROW frame, all frames must be resized as well.
 * Check for the minimal height of the FR_ROW frame.
 * At the top level we can also use change the command line height.
 */
    static void
frame_setheight(frame_T *curfrp, int height)
{
    int		room;		// total number of lines available
    int		take;		// number of lines taken from other windows
    int		room_cmdline;	// lines available from cmdline
    int		run;
    frame_T	*frp;
    int		h;
    int		room_reserved;

    // If the height already is the desired value, nothing to do.
    if (curfrp->fr_height == height)
	return;

    if (curfrp->fr_parent == NULL)
    {
	// topframe: can only change the command line height
	if (height > ROWS_AVAIL)
	    height = ROWS_AVAIL;
	if (height > 0)
	    frame_new_height(curfrp, height, FALSE, FALSE);
    }
    else if (curfrp->fr_parent->fr_layout == FR_ROW)
    {
	// Row of frames: Also need to resize frames left and right of this
	// one.  First check for the minimal height of these.
	h = frame_minheight(curfrp->fr_parent, NULL);
	if (height < h)
	    height = h;
	frame_setheight(curfrp->fr_parent, height);
    }
    else
    {
	/*
	 * Column of frames: try to change only frames in this column.
	 */
	/*
	 * Do this twice:
	 * 1: compute room available, if it's not enough try resizing the
	 *    containing frame.
	 * 2: compute the room available and adjust the height to it.
	 * Try not to reduce the height of a window with 'winfixheight' set.
	 */
	for (run = 1; run <= 2; ++run)
	{
	    room = 0;
	    room_reserved = 0;
	    FOR_ALL_FRAMES(frp, curfrp->fr_parent->fr_child)
	    {
		if (frp != curfrp
			&& frp->fr_win != NULL
			&& frp->fr_win->w_p_wfh)
		    room_reserved += frp->fr_height;
		room += frp->fr_height;
		if (frp != curfrp)
		    room -= frame_minheight(frp, NULL);
	    }
	    if (curfrp->fr_width != Columns)
		room_cmdline = 0;
	    else
	    {
		room_cmdline = Rows - p_ch - (lastwin->w_winrow
						+ VISIBLE_HEIGHT(lastwin)
						+ lastwin->w_status_height);
		if (room_cmdline < 0)
		    room_cmdline = 0;
	    }

	    if (height <= room + room_cmdline)
		break;
	    if (run == 2 || curfrp->fr_width == Columns)
	    {
		height = room + room_cmdline;
		break;
	    }
	    frame_setheight(curfrp->fr_parent, height
		+ frame_minheight(curfrp->fr_parent, NOWIN) - (int)p_wmh - 1);
	}

	/*
	 * Compute the number of lines we will take from others frames (can be
	 * negative!).
	 */
	take = height - curfrp->fr_height;

	// If there is not enough room, also reduce the height of a window
	// with 'winfixheight' set.
	if (height > room + room_cmdline - room_reserved)
	    room_reserved = room + room_cmdline - height;
	// If there is only a 'winfixheight' window and making the
	// window smaller, need to make the other window taller.
	if (take < 0 && room - curfrp->fr_height < room_reserved)
	    room_reserved = 0;

	if (take > 0 && room_cmdline > 0)
	{
	    // use lines from cmdline first
	    if (take < room_cmdline)
		room_cmdline = take;
	    take -= room_cmdline;
	    topframe->fr_height += room_cmdline;
	}

	/*
	 * set the current frame to the new height
	 */
	frame_new_height(curfrp, height, FALSE, FALSE);

	/*
	 * First take lines from the frames after the current frame.  If
	 * that is not enough, takes lines from frames above the current
	 * frame.
	 */
	for (run = 0; run < 2; ++run)
	{
	    if (run == 0)
		frp = curfrp->fr_next;	// 1st run: start with next window
	    else
		frp = curfrp->fr_prev;	// 2nd run: start with prev window
	    while (frp != NULL && take != 0)
	    {
		h = frame_minheight(frp, NULL);
		if (room_reserved > 0
			&& frp->fr_win != NULL
			&& frp->fr_win->w_p_wfh)
		{
		    if (room_reserved >= frp->fr_height)
			room_reserved -= frp->fr_height;
		    else
		    {
			if (frp->fr_height - room_reserved > take)
			    room_reserved = frp->fr_height - take;
			take -= frp->fr_height - room_reserved;
			frame_new_height(frp, room_reserved, FALSE, FALSE);
			room_reserved = 0;
		    }
		}
		else
		{
		    if (frp->fr_height - take < h)
		    {
			take -= frp->fr_height - h;
			frame_new_height(frp, h, FALSE, FALSE);
		    }
		    else
		    {
			frame_new_height(frp, frp->fr_height - take,
								FALSE, FALSE);
			take = 0;
		    }
		}
		if (run == 0)
		    frp = frp->fr_next;
		else
		    frp = frp->fr_prev;
	    }
	}
    }
}

/*
 * Set current window width and take care of repositioning other windows to
 * fit around it.
 */
    void
win_setwidth(int width)
{
    win_setwidth_win(width, curwin);
}

    void
win_setwidth_win(int width, win_T *wp)
{
    // Always keep current window at least one column wide, even when
    // 'winminwidth' is zero.
    if (wp == curwin)
    {
	if (width < p_wmw)
	    width = p_wmw;
	if (width == 0)
	    width = 1;
    }
    else if (width < 0)
	width = 0;

    frame_setwidth(wp->w_frame, width + wp->w_vsep_width);

    // recompute the window positions
    (void)win_comp_pos();

    redraw_all_later(UPD_NOT_VALID);
}

/*
 * Set the width of a frame to "width" and take care that all frames and
 * windows inside it are resized.  Also resize frames above and below if the
 * are in the same FR_ROW frame.
 *
 * Strategy is similar to frame_setheight().
 */
    static void
frame_setwidth(frame_T *curfrp, int width)
{
    int		room;		// total number of lines available
    int		take;		// number of lines taken from other windows
    int		run;
    frame_T	*frp;
    int		w;
    int		room_reserved;

    // If the width already is the desired value, nothing to do.
    if (curfrp->fr_width == width)
	return;

    if (curfrp->fr_parent == NULL)
	// topframe: can't change width
	return;

    if (curfrp->fr_parent->fr_layout == FR_COL)
    {
	// Column of frames: Also need to resize frames above and below of
	// this one.  First check for the minimal width of these.
	w = frame_minwidth(curfrp->fr_parent, NULL);
	if (width < w)
	    width = w;
	frame_setwidth(curfrp->fr_parent, width);
    }
    else
    {
	/*
	 * Row of frames: try to change only frames in this row.
	 *
	 * Do this twice:
	 * 1: compute room available, if it's not enough try resizing the
	 *    containing frame.
	 * 2: compute the room available and adjust the width to it.
	 */
	for (run = 1; run <= 2; ++run)
	{
	    room = 0;
	    room_reserved = 0;
	    FOR_ALL_FRAMES(frp, curfrp->fr_parent->fr_child)
	    {
		if (frp != curfrp
			&& frp->fr_win != NULL
			&& frp->fr_win->w_p_wfw)
		    room_reserved += frp->fr_width;
		room += frp->fr_width;
		if (frp != curfrp)
		    room -= frame_minwidth(frp, NULL);
	    }

	    if (width <= room)
		break;
	    if (run == 2 || curfrp->fr_height >= ROWS_AVAIL)
	    {
		width = room;
		break;
	    }
	    frame_setwidth(curfrp->fr_parent, width
		 + frame_minwidth(curfrp->fr_parent, NOWIN) - (int)p_wmw - 1);
	}

	/*
	 * Compute the number of lines we will take from others frames (can be
	 * negative!).
	 */
	take = width - curfrp->fr_width;

	// If there is not enough room, also reduce the width of a window
	// with 'winfixwidth' set.
	if (width > room - room_reserved)
	    room_reserved = room - width;
	// If there is only a 'winfixwidth' window and making the
	// window smaller, need to make the other window narrower.
	if (take < 0 && room - curfrp->fr_width < room_reserved)
	    room_reserved = 0;

	/*
	 * set the current frame to the new width
	 */
	frame_new_width(curfrp, width, FALSE, FALSE);

	/*
	 * First take lines from the frames right of the current frame.  If
	 * that is not enough, takes lines from frames left of the current
	 * frame.
	 */
	for (run = 0; run < 2; ++run)
	{
	    if (run == 0)
		frp = curfrp->fr_next;	// 1st run: start with next window
	    else
		frp = curfrp->fr_prev;	// 2nd run: start with prev window
	    while (frp != NULL && take != 0)
	    {
		w = frame_minwidth(frp, NULL);
		if (room_reserved > 0
			&& frp->fr_win != NULL
			&& frp->fr_win->w_p_wfw)
		{
		    if (room_reserved >= frp->fr_width)
			room_reserved -= frp->fr_width;
		    else
		    {
			if (frp->fr_width - room_reserved > take)
			    room_reserved = frp->fr_width - take;
			take -= frp->fr_width - room_reserved;
			frame_new_width(frp, room_reserved, FALSE, FALSE);
			room_reserved = 0;
		    }
		}
		else
		{
		    if (frp->fr_width - take < w)
		    {
			take -= frp->fr_width - w;
			frame_new_width(frp, w, FALSE, FALSE);
		    }
		    else
		    {
			frame_new_width(frp, frp->fr_width - take,
								FALSE, FALSE);
			take = 0;
		    }
		}
		if (run == 0)
		    frp = frp->fr_next;
		else
		    frp = frp->fr_prev;
	    }
	}
    }
}

/*
 * Check 'winminheight' for a valid value and reduce it if needed.
 */
    void
win_setminheight(void)
{
    int		room;
    int		needed;
    int		first = TRUE;

    // loop until there is a 'winminheight' that is possible
    while (p_wmh > 0)
    {
	room = Rows - p_ch;
	needed = min_rows() - 1;  // 1 was added for the cmdline
	if (room >= needed)
	    break;
	--p_wmh;
	if (first)
	{
	    emsg(_(e_not_enough_room));
	    first = FALSE;
	}
    }
}

/*
 * Check 'winminwidth' for a valid value and reduce it if needed.
 */
    void
win_setminwidth(void)
{
    int		room;
    int		needed;
    int		first = TRUE;

    // loop until there is a 'winminheight' that is possible
    while (p_wmw > 0)
    {
	room = Columns;
	needed = frame_minwidth(topframe, NULL);
	if (room >= needed)
	    break;
	--p_wmw;
	if (first)
	{
	    emsg(_(e_not_enough_room));
	    first = FALSE;
	}
    }
}

/*
 * Status line of dragwin is dragged "offset" lines down (negative is up).
 */
    void
win_drag_status_line(win_T *dragwin, int offset)
{
    frame_T	*curfr;
    frame_T	*fr;
    int		room;
    int		row;
    int		up;	// if TRUE, drag status line up, otherwise down
    int		n;

    fr = dragwin->w_frame;
    curfr = fr;
    if (fr != topframe)		// more than one window
    {
	fr = fr->fr_parent;
	// When the parent frame is not a column of frames, its parent should
	// be.
	if (fr->fr_layout != FR_COL)
	{
	    curfr = fr;
	    if (fr != topframe)	// only a row of windows, may drag statusline
		fr = fr->fr_parent;
	}
    }

    // If this is the last frame in a column, may want to resize the parent
    // frame instead (go two up to skip a row of frames).
    while (curfr != topframe && curfr->fr_next == NULL)
    {
	if (fr != topframe)
	    fr = fr->fr_parent;
	curfr = fr;
	if (fr != topframe)
	    fr = fr->fr_parent;
    }

    if (offset < 0) // drag up
    {
	up = TRUE;
	offset = -offset;
	// sum up the room of the current frame and above it
	if (fr == curfr)
	{
	    // only one window
	    room = fr->fr_height - frame_minheight(fr, NULL);
	}
	else
	{
	    room = 0;
	    for (fr = fr->fr_child; ; fr = fr->fr_next)
	    {
		room += fr->fr_height - frame_minheight(fr, NULL);
		if (fr == curfr)
		    break;
	    }
	}
	fr = curfr->fr_next;		// put fr at frame that grows
    }
    else    // drag down
    {
	up = FALSE;
	/*
	 * Only dragging the last status line can reduce p_ch.
	 */
	room = Rows - cmdline_row;
	if (curfr->fr_next == NULL)
	    --room;
	else
	    room -= p_ch;
	if (room < 0)
	    room = 0;
	// sum up the room of frames below of the current one
	FOR_ALL_FRAMES(fr, curfr->fr_next)
	    room += fr->fr_height - frame_minheight(fr, NULL);
	fr = curfr;			// put fr at window that grows
    }

    if (room < offset)		// Not enough room
	offset = room;		// Move as far as we can
    if (offset <= 0)
	return;

    /*
     * Grow frame fr by "offset" lines.
     * Doesn't happen when dragging the last status line up.
     */
    if (fr != NULL)
	frame_new_height(fr, fr->fr_height + offset, up, FALSE);

    if (up)
	fr = curfr;		// current frame gets smaller
    else
	fr = curfr->fr_next;	// next frame gets smaller

    /*
     * Now make the other frames smaller.
     */
    while (fr != NULL && offset > 0)
    {
	n = frame_minheight(fr, NULL);
	if (fr->fr_height - offset <= n)
	{
	    offset -= fr->fr_height - n;
	    frame_new_height(fr, n, !up, FALSE);
	}
	else
	{
	    frame_new_height(fr, fr->fr_height - offset, !up, FALSE);
	    break;
	}
	if (up)
	    fr = fr->fr_prev;
	else
	    fr = fr->fr_next;
    }
    row = win_comp_pos();
    screen_fill(row, cmdline_row, 0, (int)Columns, ' ', ' ', 0);
    cmdline_row = row;
    p_ch = MAX(Rows - cmdline_row, 1);
    curtab->tp_ch_used = p_ch;

    win_fix_scroll(TRUE);

    redraw_all_later(UPD_SOME_VALID);
    showmode();
}

/*
 * Separator line of dragwin is dragged "offset" lines right (negative is left).
 */
    void
win_drag_vsep_line(win_T *dragwin, int offset)
{
    frame_T	*curfr;
    frame_T	*fr;
    int		room;
    int		left;	// if TRUE, drag separator line left, otherwise right
    int		n;

    fr = dragwin->w_frame;
    if (fr == topframe)		// only one window (cannot happen?)
	return;
    curfr = fr;
    fr = fr->fr_parent;
    // When the parent frame is not a row of frames, its parent should be.
    if (fr->fr_layout != FR_ROW)
    {
	if (fr == topframe)	// only a column of windows (cannot happen?)
	    return;
	curfr = fr;
	fr = fr->fr_parent;
    }

    // If this is the last frame in a row, may want to resize a parent
    // frame instead.
    while (curfr->fr_next == NULL)
    {
	if (fr == topframe)
	    break;
	curfr = fr;
	fr = fr->fr_parent;
	if (fr != topframe)
	{
	    curfr = fr;
	    fr = fr->fr_parent;
	}
    }

    if (offset < 0) // drag left
    {
	left = TRUE;
	offset = -offset;
	// sum up the room of the current frame and left of it
	room = 0;
	for (fr = fr->fr_child; ; fr = fr->fr_next)
	{
	    room += fr->fr_width - frame_minwidth(fr, NULL);
	    if (fr == curfr)
		break;
	}
	fr = curfr->fr_next;		// put fr at frame that grows
    }
    else    // drag right
    {
	left = FALSE;
	// sum up the room of frames right of the current one
	room = 0;
	FOR_ALL_FRAMES(fr, curfr->fr_next)
	    room += fr->fr_width - frame_minwidth(fr, NULL);
	fr = curfr;			// put fr at window that grows
    }

    if (room < offset)		// Not enough room
	offset = room;		// Move as far as we can
    if (offset <= 0)		// No room at all, quit.
	return;
    if (fr == NULL)
	// This can happen when calling win_move_separator() on the rightmost
	// window.  Just don't do anything.
	return;

    // grow frame fr by offset lines
    frame_new_width(fr, fr->fr_width + offset, left, FALSE);

    // shrink other frames: current and at the left or at the right
    if (left)
	fr = curfr;		// current frame gets smaller
    else
	fr = curfr->fr_next;	// next frame gets smaller

    while (fr != NULL && offset > 0)
    {
	n = frame_minwidth(fr, NULL);
	if (fr->fr_width - offset <= n)
	{
	    offset -= fr->fr_width - n;
	    frame_new_width(fr, n, !left, FALSE);
	}
	else
	{
	    frame_new_width(fr, fr->fr_width - offset, !left, FALSE);
	    break;
	}
	if (left)
	    fr = fr->fr_prev;
	else
	    fr = fr->fr_next;
    }
    (void)win_comp_pos();
    redraw_all_later(UPD_NOT_VALID);
}

#define FRACTION_MULT	16384L

/*
 * Set wp->w_fraction for the current w_wrow and w_height.
 * Has no effect when the window is less than two lines.
 */
    void
set_fraction(win_T *wp)
{
    if (wp->w_height > 1)
	// When cursor is in the first line the percentage is computed as if
	// it's halfway that line.  Thus with two lines it is 25%, with three
	// lines 17%, etc.  Similarly for the last line: 75%, 83%, etc.
	wp->w_fraction = ((long)wp->w_wrow * FRACTION_MULT
				     + FRACTION_MULT / 2) / (long)wp->w_height;
}

/*
 * Handle scroll position, depending on 'splitkeep'.  Replaces the
 * scroll_to_fraction() call from win_new_height() if 'splitkeep' is "screen"
 * or "topline".  Instead we iterate over all windows in a tabpage and
 * calculate the new scroll position.
 * TODO: Ensure this also works with wrapped lines.
 * Requires a not fully visible cursor line to be allowed at the bottom of
 * a window("zb"), probably only when 'smoothscroll' is also set.
 */
    static void
win_fix_scroll(int resize)
{
    if (*p_spk == 'c')
	return;  // 'splitkeep' is "cursor"

    skip_update_topline = TRUE;
    win_T	*wp;
    FOR_ALL_WINDOWS(wp)
    {
	// Skip when window height has not changed.
	if (wp->w_height != wp->w_prev_height)
	{
	    // Cursor position in this window may now be invalid.  It is kept
	    // potentially invalid until the window is made the current window.
	    wp->w_do_win_fix_cursor = TRUE;

	    // If window has moved update botline to keep the same screenlines.
	    if (*p_spk == 's' && wp->w_winrow != wp->w_prev_winrow
		      && wp->w_botline - 1 <= wp->w_buffer->b_ml.ml_line_count)
	    {
		int diff = (wp->w_winrow - wp->w_prev_winrow)
					  + (wp->w_height - wp->w_prev_height);
		linenr_T lnum = wp->w_cursor.lnum;
		wp->w_cursor.lnum = wp->w_botline - 1;

		//  Add difference in height and row to botline.
		if (diff > 0)
		    cursor_down_inner(wp, diff);
		else
		    cursor_up_inner(wp, -diff);

		// Scroll to put the new cursor position at the bottom of the
		// screen.
		wp->w_fraction = FRACTION_MULT;
		scroll_to_fraction(wp, wp->w_prev_height);

		wp->w_cursor.lnum = lnum;
	    }
	    else if (wp == curwin)
		wp->w_valid &= ~VALID_CROW;

	    invalidate_botline_win(wp);
	    validate_botline_win(wp);
	}
	wp->w_prev_height = wp->w_height;
	wp->w_prev_winrow = wp->w_winrow;
    }
    skip_update_topline = FALSE;
    // Ensure cursor is valid when not in normal mode or when resized.
    if (!(get_real_state() & (MODE_NORMAL|MODE_CMDLINE|MODE_TERMINAL)))
	win_fix_cursor(FALSE);
    else if (resize)
	win_fix_cursor(TRUE);
}

/*
 * Make sure the cursor position is valid for 'splitkeep'.
 * If it is not, put the cursor position in the jumplist and move it.
 * If we are not in normal mode ("normal" is FALSE), make it valid by scrolling
 * instead.
 */
    static void
win_fix_cursor(int normal)
{
    win_T	*wp = curwin;

    if (skip_win_fix_cursor
	    || !wp->w_do_win_fix_cursor
	    || wp->w_buffer->b_ml.ml_line_count < wp->w_height)
	return;

    wp->w_do_win_fix_cursor = FALSE;
    // Determine valid cursor range.
    long so = MIN(wp->w_height / 2, get_scrolloff_value());
    linenr_T lnum = wp->w_cursor.lnum;

    wp->w_cursor.lnum = wp->w_topline;
    cursor_down_inner(wp, so);
    linenr_T top = wp->w_cursor.lnum;

    wp->w_cursor.lnum = wp->w_botline - 1;
    cursor_up_inner(wp, so);
    linenr_T bot = wp->w_cursor.lnum;

    wp->w_cursor.lnum = lnum;

    // Check if cursor position is above or below valid cursor range.
    linenr_T nlnum = 0;
    if (lnum > bot && (wp->w_botline - wp->w_buffer->b_ml.ml_line_count) != 1)
	nlnum = bot;
    else if (lnum < top && wp->w_topline != 1)
	nlnum = (so == wp->w_height / 2) ? bot : top;

    if (nlnum != 0)  // Cursor is invalid for current scroll position.
    {
	if (normal)  // Save to jumplist and set cursor to avoid scrolling.
	{
	    setmark('\'');
	    wp->w_cursor.lnum = nlnum;
	}
	else  // Scroll instead when not in normal mode.
	{
	    wp->w_fraction = (nlnum == bot) ? FRACTION_MULT : 0;
	    scroll_to_fraction(wp, wp->w_prev_height);
	    validate_botline();
	}
    }
}

/*
 * Set the height of a window.
 * "height" excludes any window toolbar.
 * This takes care of the things inside the window, not what happens to the
 * window position, the frame or to other windows.
 */
    void
win_new_height(win_T *wp, int height)
{
    int		prev_height = wp->w_height;

    // Don't want a negative height.  Happens when splitting a tiny window.
    // Will equalize heights soon to fix it.
    if (height < 0)
	height = 0;
    if (wp->w_height == height)
	return;	    // nothing to do

    if (wp->w_height > 0)
    {
	if (wp == curwin && *p_spk == 'c')
	    // w_wrow needs to be valid. When setting 'laststatus' this may
	    // call win_new_height() recursively.
	    validate_cursor();
	if (wp->w_height != prev_height)
	    return;  // Recursive call already changed the size, bail out here
		     //	to avoid the following to mess things up.
	if (wp->w_wrow != wp->w_prev_fraction_row)
	    set_fraction(wp);
    }

    wp->w_height = height;
    wp->w_redr_status = TRUE;
    win_comp_scroll(wp);

    // There is no point in adjusting the scroll position when exiting.  Some
    // values might be invalid.
    if (!exiting && *p_spk == 'c')
    {
	wp->w_skipcol = 0;
	scroll_to_fraction(wp, prev_height);
    }
}

    void
scroll_to_fraction(win_T *wp, int prev_height)
{
    linenr_T	lnum;
    int		sline, line_size;
    int		height = wp->w_height;

    // Don't change w_topline in any of these cases:
    // - window height is 0
    // - 'scrollbind' is set and this isn't the current window
    // - window height is sufficient to display the whole buffer and first line
    //   is visible.
    if (height > 0
	   && (!wp->w_p_scb || wp == curwin)
	   && (height < wp->w_buffer->b_ml.ml_line_count || wp->w_topline > 1))
    {
	/*
	 * Find a value for w_topline that shows the cursor at the same
	 * relative position in the window as before (more or less).
	 */
	lnum = wp->w_cursor.lnum;
	if (lnum < 1)		// can happen when starting up
	    lnum = 1;
	wp->w_wrow = ((long)wp->w_fraction * (long)height - 1L)
							       / FRACTION_MULT;
	line_size = plines_win_col(wp, lnum, (long)(wp->w_cursor.col)) - 1;
	sline = wp->w_wrow - line_size;

	if (sline >= 0)
	{
	    // Make sure the whole cursor line is visible, if possible.
	    int rows = plines_win(wp, lnum, FALSE);

	    if (sline > wp->w_height - rows)
	    {
		sline = wp->w_height - rows;
		wp->w_wrow -= rows - line_size;
	    }
	}

	if (sline < 0)
	{
	    /*
	     * Cursor line would go off top of screen if w_wrow was this high.
	     * Make cursor line the first line in the window.  If not enough
	     * room use w_skipcol;
	     */
	    wp->w_wrow = line_size;
	    if (wp->w_wrow >= wp->w_height
				       && (wp->w_width - win_col_off(wp)) > 0)
	    {
		wp->w_skipcol += wp->w_width - win_col_off(wp);
		--wp->w_wrow;
		while (wp->w_wrow >= wp->w_height)
		{
		    wp->w_skipcol += wp->w_width - win_col_off(wp)
							   + win_col_off2(wp);
		    --wp->w_wrow;
		}
	    }
	}
	else if (sline > 0)
	{
	    while (sline > 0 && lnum > 1)
	    {
#ifdef FEAT_FOLDING
		hasFoldingWin(wp, lnum, &lnum, NULL, TRUE, NULL);
		if (lnum == 1)
		{
		    // first line in buffer is folded
		    line_size = 1;
		    --sline;
		    break;
		}
#endif
		--lnum;
#ifdef FEAT_DIFF
		if (lnum == wp->w_topline)
		    line_size = plines_win_nofill(wp, lnum, TRUE)
							      + wp->w_topfill;
		else
#endif
		    line_size = plines_win(wp, lnum, TRUE);
		sline -= line_size;
	    }

	    if (sline < 0)
	    {
		/*
		 * Line we want at top would go off top of screen.  Use next
		 * line instead.
		 */
#ifdef FEAT_FOLDING
		hasFoldingWin(wp, lnum, NULL, &lnum, TRUE, NULL);
#endif
		lnum++;
		wp->w_wrow -= line_size + sline;
	    }
	    else if (sline > 0)
	    {
		// First line of file reached, use that as topline.
		lnum = 1;
		wp->w_wrow -= sline;
	    }
	}
	set_topline(wp, lnum);
    }

    if (wp == curwin)
	curs_columns(FALSE);	// validate w_wrow

    if (prev_height > 0)
	wp->w_prev_fraction_row = wp->w_wrow;

    redraw_win_later(wp, UPD_SOME_VALID);
    invalidate_botline_win(wp);
}

/*
 * Set the width of a window.
 */
    void
win_new_width(win_T *wp, int width)
{
    // Should we give an error if width < 0?
    wp->w_width = width < 0 ? 0 : width;
    wp->w_lines_valid = 0;
    changed_line_abv_curs_win(wp);
    invalidate_botline_win(wp);

    if (wp == curwin && *p_spk == 'c')
	curs_columns(TRUE);	// validate w_wrow

    redraw_win_later(wp, UPD_NOT_VALID);
    wp->w_redr_status = TRUE;
}

    void
win_comp_scroll(win_T *wp)
{
#if defined(FEAT_EVAL)
    int old_w_p_scr = wp->w_p_scr;
#endif

    wp->w_p_scr = ((unsigned)wp->w_height >> 1);
    if (wp->w_p_scr == 0)
	wp->w_p_scr = 1;
#if defined(FEAT_EVAL)
    if (wp->w_p_scr != old_w_p_scr)
    {
	// Used by "verbose set scroll".
	wp->w_p_script_ctx[WV_SCROLL].sc_sid = SID_WINLAYOUT;
	wp->w_p_script_ctx[WV_SCROLL].sc_lnum = 0;
    }
#endif
}

/*
 * Command_height: called whenever p_ch has been changed.
 */
    void
command_height(void)
{
    int		h;
    frame_T	*frp;
    int		old_p_ch = curtab->tp_ch_used;

    // Use the value of p_ch that we remembered.  This is needed for when the
    // GUI starts up, we can't be sure in what order things happen.  And when
    // p_ch was changed in another tab page.
    curtab->tp_ch_used = p_ch;

    // If the space for the command line is already more than 'cmdheight' there
    // is nothing to do (window size must have decreased).
    // Note: this makes curtab->tp_ch_used unreliable
    if (p_ch > old_p_ch && cmdline_row <= Rows - p_ch)
	return;

    // Update cmdline_row to what it should be: just below the last window.
    cmdline_row = topframe->fr_height + tabline_height();

    // old_p_ch may be unreliable, because of the early return above, so
    // set old_p_ch to what it would be, so that the windows get resized
    // properly for the new value.
    old_p_ch = Rows - cmdline_row;

    // Find bottom frame with width of screen.
    frp = lastwin->w_frame;
    while (frp->fr_width != Columns && frp->fr_parent != NULL)
	frp = frp->fr_parent;

    // Avoid changing the height of a window with 'winfixheight' set.
    while (frp->fr_prev != NULL && frp->fr_layout == FR_LEAF
						      && frp->fr_win->w_p_wfh)
	frp = frp->fr_prev;

    if (starting != NO_SCREEN)
    {
	cmdline_row = Rows - p_ch;

	if (p_ch > old_p_ch)		    // p_ch got bigger
	{
	    while (p_ch > old_p_ch)
	    {
		if (frp == NULL)
		{
		    emsg(_(e_not_enough_room));
		    p_ch = old_p_ch;
		    curtab->tp_ch_used = p_ch;
		    cmdline_row = Rows - p_ch;
		    break;
		}
		h = frp->fr_height - frame_minheight(frp, NULL);
		if (h > p_ch - old_p_ch)
		    h = p_ch - old_p_ch;
		old_p_ch += h;
		frame_add_height(frp, -h);
		frp = frp->fr_prev;
	    }

	    // Recompute window positions.
	    (void)win_comp_pos();

	    // clear the lines added to cmdline
	    if (full_screen)
		screen_fill(cmdline_row, (int)Rows, 0,
						   (int)Columns, ' ', ' ', 0);
	    msg_row = cmdline_row;
	    redraw_cmdline = TRUE;
	    return;
	}

	if (msg_row < cmdline_row)
	    msg_row = cmdline_row;
	redraw_cmdline = TRUE;
    }
    frame_add_height(frp, (int)(old_p_ch - p_ch));

    // Recompute window positions.
    if (frp != lastwin->w_frame)
	(void)win_comp_pos();
}

/*
 * Resize frame "frp" to be "n" lines higher (negative for less high).
 * Also resize the frames it is contained in.
 */
    static void
frame_add_height(frame_T *frp, int n)
{
    frame_new_height(frp, frp->fr_height + n, FALSE, FALSE);
    for (;;)
    {
	frp = frp->fr_parent;
	if (frp == NULL)
	    break;
	frp->fr_height += n;
    }
}

/*
 * Add or remove a status line for the bottom window(s), according to the
 * value of 'laststatus'.
 */
    void
last_status(
    int		morewin)	// pretend there are two or more windows
{
    // Don't make a difference between horizontal or vertical split.
    last_status_rec(topframe, last_stl_height(morewin) > 0);
}

    static void
last_status_rec(frame_T *fr, int statusline)
{
    frame_T	*fp;
    win_T	*wp;

    if (fr->fr_layout == FR_LEAF)
    {
	wp = fr->fr_win;
	if (wp->w_status_height != 0 && !statusline)
	{
	    // remove status line
	    win_new_height(wp, wp->w_height + 1);
	    wp->w_status_height = 0;
	    comp_col();
	}
	else if (wp->w_status_height == 0 && statusline)
	{
	    // Find a frame to take a line from.
	    fp = fr;
	    while (fp->fr_height <= frame_minheight(fp, NULL))
	    {
		if (fp == topframe)
		{
		    emsg(_(e_not_enough_room));
		    return;
		}
		// In a column of frames: go to frame above.  If already at
		// the top or in a row of frames: go to parent.
		if (fp->fr_parent->fr_layout == FR_COL && fp->fr_prev != NULL)
		    fp = fp->fr_prev;
		else
		    fp = fp->fr_parent;
	    }
	    wp->w_status_height = 1;
	    if (fp != fr)
	    {
		frame_new_height(fp, fp->fr_height - 1, FALSE, FALSE);
		frame_fix_height(wp);
		(void)win_comp_pos();
	    }
	    else
		win_new_height(wp, wp->w_height - 1);
	    comp_col();
	    redraw_all_later(UPD_SOME_VALID);
	}
	// Set prev_height when difference is due to 'laststatus'.
	if (abs(wp->w_height - wp->w_prev_height) == 1)
	    wp->w_prev_height = wp->w_height;
    }
    else if (fr->fr_layout == FR_ROW)
    {
	// vertically split windows, set status line for each one
	FOR_ALL_FRAMES(fp, fr->fr_child)
	    last_status_rec(fp, statusline);
    }
    else
    {
	// horizontally split window, set status line for last one
	for (fp = fr->fr_child; fp->fr_next != NULL; fp = fp->fr_next)
	    ;
	last_status_rec(fp, statusline);
    }
}

/*
 * Return the number of lines used by the tab page line.
 */
    int
tabline_height(void)
{
#ifdef FEAT_GUI_TABLINE
    // When the GUI has the tabline then this always returns zero.
    if (gui_use_tabline())
	return 0;
#endif
    switch (p_stal)
    {
	case 0: return 0;
	case 1: return (first_tabpage->tp_next == NULL) ? 0 : 1;
    }
    return 1;
}

/*
 * Return the height of the last window's statusline.
 */
    int
last_stl_height(
    int		morewin)	// pretend there are two or more windows
{
    return (p_ls == 2 || (p_ls == 1 && (morewin || !ONE_WINDOW)))
		? STATUS_HEIGHT : 0;
}

/*
 * Return the minimal number of rows that is needed on the screen to display
 * the current number of windows.
 */
    int
min_rows(void)
{
    int		total;
    tabpage_T	*tp;
    int		n;

    if (firstwin == NULL)	// not initialized yet
	return MIN_LINES;

    total = 0;
    FOR_ALL_TABPAGES(tp)
    {
	n = frame_minheight(tp->tp_topframe, NULL);
	if (total < n)
	    total = n;
    }
    total += tabline_height();
    total += 1;		// count the room for the command line
    return total;
}

/*
 * Return TRUE if there is only one window and only one tab page, not
 * counting a help or preview window, unless it is the current window.
 * Does not count unlisted windows.
 */
    int
only_one_window(void)
{
    int		count = 0;
    win_T	*wp;

#if defined(FEAT_PROP_POPUP)
    // If the current window is a popup then there always is another window.
    if (popup_is_popup(curwin))
	return FALSE;
#endif

    // If there is another tab page there always is another window.
    if (first_tabpage->tp_next != NULL)
	return FALSE;

    FOR_ALL_WINDOWS(wp)
	if (wp->w_buffer != NULL
		&& (!((bt_help(wp->w_buffer) && !bt_help(curbuf))
# ifdef FEAT_QUICKFIX
		    || wp->w_p_pvw
# endif
	     ) || wp == curwin) && !is_aucmd_win(wp))
	    ++count;
    return (count <= 1);
}

/*
 * Implementation of check_lnums() and check_lnums_nested().
 */
    static void
check_lnums_both(int do_curwin, int nested)
{
    win_T	*wp;
    tabpage_T	*tp;

    FOR_ALL_TAB_WINDOWS(tp, wp)
	if ((do_curwin || wp != curwin) && wp->w_buffer == curbuf)
	{
	    int need_adjust;

	    if (!nested)
	    {
		// save the original cursor position and topline
		wp->w_save_cursor.w_cursor_save = wp->w_cursor;
		wp->w_save_cursor.w_topline_save = wp->w_topline;
	    }

	    need_adjust = wp->w_cursor.lnum > curbuf->b_ml.ml_line_count;
	    if (need_adjust)
		wp->w_cursor.lnum = curbuf->b_ml.ml_line_count;
	    if (need_adjust || !nested)
		// save the (corrected) cursor position
		wp->w_save_cursor.w_cursor_corr = wp->w_cursor;

	    need_adjust = wp->w_topline > curbuf->b_ml.ml_line_count;
	    if (need_adjust)
		wp->w_topline = curbuf->b_ml.ml_line_count;
	    if (need_adjust || !nested)
		// save the (corrected) topline
		wp->w_save_cursor.w_topline_corr = wp->w_topline;
	}
}

/*
 * Correct the cursor line number in other windows.  Used after changing the
 * current buffer, and before applying autocommands.
 * When "do_curwin" is TRUE, also check current window.
 */
    void
check_lnums(int do_curwin)
{
    check_lnums_both(do_curwin, FALSE);
}

/*
 * Like check_lnums() but for when check_lnums() was already called.
 */
    void
check_lnums_nested(int do_curwin)
{
    check_lnums_both(do_curwin, TRUE);
}

/*
 * Reset cursor and topline to its stored values from check_lnums().
 * check_lnums() must have been called first!
 */
    void
reset_lnums(void)
{
    win_T	*wp;
    tabpage_T	*tp;

    FOR_ALL_TAB_WINDOWS(tp, wp)
	if (wp->w_buffer == curbuf)
	{
	    // Restore the value if the autocommand didn't change it and it was
	    // set.
	    // Note: This triggers e.g. on BufReadPre, when the buffer is not yet
	    //       loaded, so cannot validate the buffer line
	    if (EQUAL_POS(wp->w_save_cursor.w_cursor_corr, wp->w_cursor)
				  && wp->w_save_cursor.w_cursor_save.lnum != 0)
		wp->w_cursor = wp->w_save_cursor.w_cursor_save;
	    if (wp->w_save_cursor.w_topline_corr == wp->w_topline
				  && wp->w_save_cursor.w_topline_save != 0)
		wp->w_topline = wp->w_save_cursor.w_topline_save;
	   if (wp->w_save_cursor.w_topline_save > wp->w_buffer->b_ml.ml_line_count)
		wp->w_valid &= ~VALID_TOPLINE;
	}
}

/*
 * A snapshot of the window sizes, to restore them after closing the help
 * or other window.
 * Only these fields are used:
 * fr_layout
 * fr_width
 * fr_height
 * fr_next
 * fr_child
 * fr_win (only valid for the old curwin, NULL otherwise)
 */

/*
 * Create a snapshot of the current frame sizes.
 * "idx" is SNAP_HELP_IDX or SNAP_AUCMD_IDX.
 */
    void
make_snapshot(int idx)
{
    clear_snapshot(curtab, idx);
    make_snapshot_rec(topframe, &curtab->tp_snapshot[idx]);
}

    static void
make_snapshot_rec(frame_T *fr, frame_T **frp)
{
    *frp = ALLOC_CLEAR_ONE(frame_T);
    if (*frp == NULL)
	return;
    (*frp)->fr_layout = fr->fr_layout;
    (*frp)->fr_width = fr->fr_width;
    (*frp)->fr_height = fr->fr_height;
    if (fr->fr_next != NULL)
	make_snapshot_rec(fr->fr_next, &((*frp)->fr_next));
    if (fr->fr_child != NULL)
	make_snapshot_rec(fr->fr_child, &((*frp)->fr_child));
    if (fr->fr_layout == FR_LEAF && fr->fr_win == curwin)
	(*frp)->fr_win = curwin;
}

/*
 * Remove any existing snapshot.
 */
    static void
clear_snapshot(tabpage_T *tp, int idx)
{
    clear_snapshot_rec(tp->tp_snapshot[idx]);
    tp->tp_snapshot[idx] = NULL;
}

    static void
clear_snapshot_rec(frame_T *fr)
{
    if (fr == NULL)
	return;
    clear_snapshot_rec(fr->fr_next);
    clear_snapshot_rec(fr->fr_child);
    vim_free(fr);
}

/*
 * Traverse a snapshot to find the previous curwin.
 */
    static win_T *
get_snapshot_curwin_rec(frame_T *ft)
{
    win_T	*wp;

    if (ft->fr_next != NULL)
    {
	if ((wp = get_snapshot_curwin_rec(ft->fr_next)) != NULL)
	    return wp;
    }
    if (ft->fr_child != NULL)
    {
	if ((wp = get_snapshot_curwin_rec(ft->fr_child)) != NULL)
	    return wp;
    }

    return ft->fr_win;
}

/*
 * Return the current window stored in the snapshot or NULL.
 */
    static win_T *
get_snapshot_curwin(int idx)
{
    if (curtab->tp_snapshot[idx] == NULL)
	return NULL;

    return get_snapshot_curwin_rec(curtab->tp_snapshot[idx]);
}

/*
 * Restore a previously created snapshot, if there is any.
 * This is only done if the screen size didn't change and the window layout is
 * still the same.
 * "idx" is SNAP_HELP_IDX or SNAP_AUCMD_IDX.
 */
    void
restore_snapshot(
    int		idx,
    int		close_curwin)	    // closing current window
{
    win_T	*wp;

    if (curtab->tp_snapshot[idx] != NULL
	    && curtab->tp_snapshot[idx]->fr_width == topframe->fr_width
	    && curtab->tp_snapshot[idx]->fr_height == topframe->fr_height
	    && check_snapshot_rec(curtab->tp_snapshot[idx], topframe) == OK)
    {
	wp = restore_snapshot_rec(curtab->tp_snapshot[idx], topframe);
	win_comp_pos();
	if (wp != NULL && close_curwin)
	    win_goto(wp);
	redraw_all_later(UPD_NOT_VALID);
    }
    clear_snapshot(curtab, idx);
}

/*
 * Check if frames "sn" and "fr" have the same layout, same following frames
 * and same children.  And the window pointer is valid.
 */
    static int
check_snapshot_rec(frame_T *sn, frame_T *fr)
{
    if (sn->fr_layout != fr->fr_layout
	    || (sn->fr_next == NULL) != (fr->fr_next == NULL)
	    || (sn->fr_child == NULL) != (fr->fr_child == NULL)
	    || (sn->fr_next != NULL
		&& check_snapshot_rec(sn->fr_next, fr->fr_next) == FAIL)
	    || (sn->fr_child != NULL
		&& check_snapshot_rec(sn->fr_child, fr->fr_child) == FAIL)
	    || (sn->fr_win != NULL && !win_valid(sn->fr_win)))
	return FAIL;
    return OK;
}

/*
 * Copy the size of snapshot frame "sn" to frame "fr".  Do the same for all
 * following frames and children.
 * Returns a pointer to the old current window, or NULL.
 */
    static win_T *
restore_snapshot_rec(frame_T *sn, frame_T *fr)
{
    win_T	*wp = NULL;
    win_T	*wp2;

    fr->fr_height = sn->fr_height;
    fr->fr_width = sn->fr_width;
    if (fr->fr_layout == FR_LEAF)
    {
	frame_new_height(fr, fr->fr_height, FALSE, FALSE);
	frame_new_width(fr, fr->fr_width, FALSE, FALSE);
	wp = sn->fr_win;
    }
    if (sn->fr_next != NULL)
    {
	wp2 = restore_snapshot_rec(sn->fr_next, fr->fr_next);
	if (wp2 != NULL)
	    wp = wp2;
    }
    if (sn->fr_child != NULL)
    {
	wp2 = restore_snapshot_rec(sn->fr_child, fr->fr_child);
	if (wp2 != NULL)
	    wp = wp2;
    }
    return wp;
}

#if defined(FEAT_GUI) || defined(PROTO)
/*
 * Return TRUE if there is any vertically split window.
 */
    int
win_hasvertsplit(void)
{
    frame_T	*fr;

    if (topframe->fr_layout == FR_ROW)
	return TRUE;

    if (topframe->fr_layout == FR_COL)
	FOR_ALL_FRAMES(fr, topframe->fr_child)
	    if (fr->fr_layout == FR_ROW)
		return TRUE;

    return FALSE;
}
#endif

#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3) || defined(PROTO)
    int
get_win_number(win_T *wp, win_T *first_win)
{
    int		i = 1;
    win_T	*w;

    for (w = first_win; w != NULL && w != wp; w = W_NEXT(w))
	++i;

    if (w == NULL)
	return 0;
    else
	return i;
}

    int
get_tab_number(tabpage_T *tp UNUSED)
{
    int		i = 1;
    tabpage_T	*t;

    for (t = first_tabpage; t != NULL && t != tp; t = t->tp_next)
	++i;

    if (t == NULL)
	return 0;
    else
	return i;
}
#endif

/*
 * Return TRUE if "topfrp" and its children are at the right height.
 */
    static int
frame_check_height(frame_T *topfrp, int height)
{
    frame_T *frp;

    if (topfrp->fr_height != height)
	return FALSE;

    if (topfrp->fr_layout == FR_ROW)
	FOR_ALL_FRAMES(frp, topfrp->fr_child)
	    if (frp->fr_height != height)
		return FALSE;

    return TRUE;
}

/*
 * Return TRUE if "topfrp" and its children are at the right width.
 */
    static int
frame_check_width(frame_T *topfrp, int width)
{
    frame_T *frp;

    if (topfrp->fr_width != width)
	return FALSE;

    if (topfrp->fr_layout == FR_COL)
	FOR_ALL_FRAMES(frp, topfrp->fr_child)
	    if (frp->fr_width != width)
		return FALSE;

    return TRUE;
}

#if defined(FEAT_SYN_HL) || defined(PROTO)
/*
 * Simple int comparison function for use with qsort()
 */
    static int
int_cmp(const void *a, const void *b)
{
    return *(const int *)a - *(const int *)b;
}

/*
 * Handle setting 'colorcolumn' or 'textwidth' in window "wp".
 * Returns error message, NULL if it's OK.
 */
    char *
check_colorcolumn(win_T *wp)
{
    char_u	*s;
    int		col;
    int		count = 0;
    int		color_cols[256];
    int		i;
    int		j = 0;

    if (wp->w_buffer == NULL)
	return NULL;  // buffer was closed

    for (s = wp->w_p_cc; *s != NUL && count < 255;)
    {
	if (*s == '-' || *s == '+')
	{
	    // -N and +N: add to 'textwidth'
	    col = (*s == '-') ? -1 : 1;
	    ++s;
	    if (!VIM_ISDIGIT(*s))
		return e_invalid_argument;
	    col = col * getdigits(&s);
	    if (wp->w_buffer->b_p_tw == 0)
		goto skip;  // 'textwidth' not set, skip this item
	    col += wp->w_buffer->b_p_tw;
	    if (col < 0)
		goto skip;
	}
	else if (VIM_ISDIGIT(*s))
	    col = getdigits(&s);
	else
	    return e_invalid_argument;
	color_cols[count++] = col - 1;  // 1-based to 0-based
skip:
	if (*s == NUL)
	    break;
	if (*s != ',')
	    return e_invalid_argument;
	if (*++s == NUL)
	    return e_invalid_argument;  // illegal trailing comma as in "set cc=80,"
    }

    vim_free(wp->w_p_cc_cols);
    if (count == 0)
	wp->w_p_cc_cols = NULL;
    else
    {
	wp->w_p_cc_cols = ALLOC_MULT(int, count + 1);
	if (wp->w_p_cc_cols != NULL)
	{
	    // sort the columns for faster usage on screen redraw inside
	    // win_line()
	    qsort(color_cols, count, sizeof(int), int_cmp);

	    for (i = 0; i < count; ++i)
		// skip duplicates
		if (j == 0 || wp->w_p_cc_cols[j - 1] != color_cols[i])
		    wp->w_p_cc_cols[j++] = color_cols[i];
	    wp->w_p_cc_cols[j] = -1;  // end marker
	}
    }

    return NULL;  // no error
}
#endif
