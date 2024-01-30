/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * edit.c: functions for Insert mode
 */

#include "vim.h"

#define BACKSPACE_CHAR		    1
#define BACKSPACE_WORD		    2
#define BACKSPACE_WORD_NOT_SPACE    3
#define BACKSPACE_LINE		    4

// Set when doing something for completion that may call edit() recursively,
// which is not allowed.
static int	compl_busy = FALSE;


static void ins_ctrl_v(void);
static void insert_special(int, int, int);
static void redo_literal(int c);
static void start_arrow_common(pos_T *end_insert_pos, int change);
#ifdef FEAT_SPELL
static void check_spell_redraw(void);
#endif
static void stop_insert(pos_T *end_insert_pos, int esc, int nomove);
static int  echeck_abbr(int);
static void mb_replace_pop_ins(int cc);
static void replace_flush(void);
static void replace_do_bs(int limit_col);
static int del_char_after_col(int limit_col);
static void ins_reg(void);
static void ins_ctrl_g(void);
static void ins_ctrl_hat(void);
static int  ins_esc(long *count, int cmdchar, int nomove);
#ifdef FEAT_RIGHTLEFT
static void ins_ctrl_(void);
#endif
static int ins_start_select(int c);
static void ins_insert(int replaceState);
static void ins_ctrl_o(void);
static void ins_shift(int c, int lastc);
static void ins_del(void);
static int  ins_bs(int c, int mode, int *inserted_space_p);
#if defined(FEAT_GUI_TABLINE) || defined(PROTO)
static void ins_tabline(int c);
#endif
static void ins_left(void);
static void ins_home(int c);
static void ins_end(int c);
static void ins_s_left(void);
static void ins_right(void);
static void ins_s_right(void);
static void ins_up(int startcol);
static void ins_pageup(void);
static void ins_down(int startcol);
static void ins_pagedown(void);
#ifdef FEAT_DND
static void ins_drop(void);
#endif
static int  ins_tab(void);
#ifdef FEAT_DIGRAPHS
static int  ins_digraph(void);
#endif
static int  ins_ctrl_ey(int tc);
#if defined(FEAT_EVAL)
static char_u *do_insert_char_pre(int c);
#endif

static colnr_T	Insstart_textlen;	// length of line when insert started
static colnr_T	Insstart_blank_vcol;	// vcol for first inserted blank
static int	update_Insstart_orig = TRUE; // set Insstart_orig to Insstart

static char_u	*last_insert = NULL;	// the text of the previous insert,
					// K_SPECIAL and CSI are escaped
static int	last_insert_skip; // nr of chars in front of previous insert
static int	new_insert_skip;  // nr of chars in front of current insert
static int	did_restart_edit;	// "restart_edit" when calling edit()

static int	can_cindent;		// may do cindenting on this line

#ifdef FEAT_RIGHTLEFT
static int	revins_on;		// reverse insert mode on
static int	revins_chars;		// how much to skip after edit
static int	revins_legal;		// was the last char 'legal'?
static int	revins_scol;		// start column of revins session
#endif

static int	ins_need_undo;		// call u_save() before inserting a
					// char.  Set when edit() is called.
					// after that arrow_used is used.

static int	dont_sync_undo = FALSE;	// CTRL-G U prevents syncing undo for
					// the next left/right cursor key

/*
 * edit(): Start inserting text.
 *
 * "cmdchar" can be:
 * 'i'	normal insert command
 * 'a'	normal append command
 * K_PS bracketed paste
 * 'R'	replace command
 * 'r'	"r<CR>" command: insert one <CR>.  Note: count can be > 1, for redo,
 *	but still only one <CR> is inserted.  The <Esc> is not used for redo.
 * 'g'	"gI" command.
 * 'V'	"gR" command for Virtual Replace mode.
 * 'v'	"gr" command for single character Virtual Replace mode.
 *
 * This function is not called recursively.  For CTRL-O commands, it returns
 * and lets the caller handle the Normal-mode command.
 *
 * Return TRUE if a CTRL-O command caused the return (insert mode pending).
 */
    int
edit(
    int		cmdchar,
    int		startln,	// if set, insert at start of line
    long	count)
{
    int		c = 0;
    char_u	*ptr;
    int		lastc = 0;
    int		mincol;
    static linenr_T o_lnum = 0;
    int		i;
    int		did_backspace = TRUE;	    // previous char was backspace
    int		line_is_white = FALSE;	    // line is empty before insert
    linenr_T	old_topline = 0;	    // topline before insertion
#ifdef FEAT_DIFF
    int		old_topfill = -1;
#endif
    int		inserted_space = FALSE;     // just inserted a space
    int		replaceState = MODE_REPLACE;
    int		nomove = FALSE;		    // don't move cursor on return
#ifdef FEAT_JOB_CHANNEL
    int		cmdchar_todo = cmdchar;
#endif
#ifdef FEAT_CONCEAL
    int		cursor_line_was_concealed;
#endif

    // Remember whether editing was restarted after CTRL-O.
    did_restart_edit = restart_edit;

    // sleep before redrawing, needed for "CTRL-O :" that results in an
    // error message
    check_for_delay(TRUE);

    // set Insstart_orig to Insstart
    update_Insstart_orig = TRUE;

#ifdef HAVE_SANDBOX
    // Don't allow inserting in the sandbox.
    if (sandbox != 0)
    {
	emsg(_(e_not_allowed_in_sandbox));
	return FALSE;
    }
#endif
    // Don't allow changes in the buffer while editing the cmdline.  The
    // caller of getcmdline() may get confused.
    // Don't allow recursive insert mode when busy with completion.
    if (textlock != 0 || ins_compl_active() || compl_busy || pum_visible())
    {
	emsg(_(e_not_allowed_to_change_text_or_change_window));
	return FALSE;
    }
    ins_compl_clear();	    // clear stuff for CTRL-X mode

    /*
     * Trigger InsertEnter autocommands.  Do not do this for "r<CR>" or "grx".
     */
    if (cmdchar != 'r' && cmdchar != 'v')
    {
	pos_T   save_cursor = curwin->w_cursor;

#ifdef FEAT_EVAL
	if (cmdchar == 'R')
	    ptr = (char_u *)"r";
	else if (cmdchar == 'V')
	    ptr = (char_u *)"v";
	else
	    ptr = (char_u *)"i";
	set_vim_var_string(VV_INSERTMODE, ptr, 1);
	set_vim_var_string(VV_CHAR, NULL, -1);  // clear v:char
#endif
	ins_apply_autocmds(EVENT_INSERTENTER);

	// Check for changed highlighting, e.g. for ModeMsg.
	if (need_highlight_changed)
	    highlight_changed();

	// Make sure the cursor didn't move.  Do call check_cursor_col() in
	// case the text was modified.  Since Insert mode was not started yet
	// a call to check_cursor_col() may move the cursor, especially with
	// the "A" command, thus set State to avoid that. Also check that the
	// line number is still valid (lines may have been deleted).
	// Do not restore if v:char was set to a non-empty string.
	if (!EQUAL_POS(curwin->w_cursor, save_cursor)
#ifdef FEAT_EVAL
		&& *get_vim_var_str(VV_CHAR) == NUL
#endif
		&& save_cursor.lnum <= curbuf->b_ml.ml_line_count)
	{
	    int save_state = State;

	    curwin->w_cursor = save_cursor;
	    State = MODE_INSERT;
	    check_cursor_col();
	    State = save_state;
	}
    }

#ifdef FEAT_CONCEAL
    // Check if the cursor line was concealed before changing State.
    cursor_line_was_concealed = curwin->w_p_cole > 0
						&& conceal_cursor_line(curwin);
#endif

    /*
     * When doing a paste with the middle mouse button, Insstart is set to
     * where the paste started.
     */
    if (where_paste_started.lnum != 0)
	Insstart = where_paste_started;
    else
    {
	Insstart = curwin->w_cursor;
	if (startln)
	    Insstart.col = 0;
    }
    Insstart_textlen = (colnr_T)linetabsize_str(ml_get_curline());
    Insstart_blank_vcol = MAXCOL;
    if (!did_ai)
	ai_col = 0;

    if (cmdchar != NUL && restart_edit == 0)
    {
	ResetRedobuff();
	AppendNumberToRedobuff(count);
	if (cmdchar == 'V' || cmdchar == 'v')
	{
	    // "gR" or "gr" command
	    AppendCharToRedobuff('g');
	    AppendCharToRedobuff((cmdchar == 'v') ? 'r' : 'R');
	}
	else
	{
	    if (cmdchar == K_PS)
		AppendCharToRedobuff('a');
	    else
		AppendCharToRedobuff(cmdchar);
	    if (cmdchar == 'g')		    // "gI" command
		AppendCharToRedobuff('I');
	    else if (cmdchar == 'r')	    // "r<CR>" command
		count = 1;		    // insert only one <CR>
	}
    }

    if (cmdchar == 'R')
    {
	State = MODE_REPLACE;
    }
    else if (cmdchar == 'V' || cmdchar == 'v')
    {
	State = MODE_VREPLACE;
	replaceState = MODE_VREPLACE;
	orig_line_count = curbuf->b_ml.ml_line_count;
	vr_lines_changed = 1;
    }
    else
	State = MODE_INSERT;

    may_trigger_modechanged();
    stop_insert_mode = FALSE;

#ifdef FEAT_CONCEAL
    // Check if the cursor line needs redrawing after changing State.  If
    // 'concealcursor' is "n" it needs to be redrawn without concealing.
    conceal_check_cursor_line(cursor_line_was_concealed);
#endif

    // Need to position cursor again when on a TAB and when on a char with
    // virtual text.
    if (gchar_cursor() == TAB
#ifdef FEAT_PROP_POPUP
	    || curbuf->b_has_textprop
#endif
       )
	curwin->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL);

    /*
     * Enable langmap or IME, indicated by 'iminsert'.
     * Note that IME may enabled/disabled without us noticing here, thus the
     * 'iminsert' value may not reflect what is actually used.  It is updated
     * when hitting <Esc>.
     */
    if (curbuf->b_p_iminsert == B_IMODE_LMAP)
	State |= MODE_LANGMAP;
#ifdef HAVE_INPUT_METHOD
    im_set_active(curbuf->b_p_iminsert == B_IMODE_IM);
#endif

    setmouse();
    clear_showcmd();
#ifdef FEAT_RIGHTLEFT
    // there is no reverse replace mode
    revins_on = (State == MODE_INSERT && p_ri);
    if (revins_on)
	undisplay_dollar();
    revins_chars = 0;
    revins_legal = 0;
    revins_scol = -1;
#endif
    if (!p_ek)
    {
	MAY_WANT_TO_LOG_THIS;

	// Disable bracketed paste mode, we won't recognize the escape
	// sequences.
	out_str(T_BD);

	// Disable modifyOtherKeys, keys with modifiers would cause exiting
	// Insert mode.
	out_str_t_TE();
    }

    /*
     * Handle restarting Insert mode.
     * Don't do this for "CTRL-O ." (repeat an insert): In that case we get
     * here with something in the stuff buffer.
     */
    if (restart_edit != 0 && stuff_empty())
    {
	/*
	 * After a paste we consider text typed to be part of the insert for
	 * the pasted text. You can backspace over the pasted text too.
	 */
	if (where_paste_started.lnum)
	    arrow_used = FALSE;
	else
	    arrow_used = TRUE;
	restart_edit = 0;

	/*
	 * If the cursor was after the end-of-line before the CTRL-O and it is
	 * now at the end-of-line, put it after the end-of-line (this is not
	 * correct in very rare cases).
	 * Also do this if curswant is greater than the current virtual
	 * column.  Eg after "^O$" or "^O80|".
	 */
	validate_virtcol();
	update_curswant();
	if (((ins_at_eol && curwin->w_cursor.lnum == o_lnum)
		    || curwin->w_curswant > curwin->w_virtcol)
		&& *(ptr = ml_get_curline() + curwin->w_cursor.col) != NUL)
	{
	    if (ptr[1] == NUL)
		++curwin->w_cursor.col;
	    else if (has_mbyte)
	    {
		i = (*mb_ptr2len)(ptr);
		if (ptr[i] == NUL)
		    curwin->w_cursor.col += i;
	    }
	}
	ins_at_eol = FALSE;
    }
    else
	arrow_used = FALSE;

    // we are in insert mode now, don't need to start it anymore
    need_start_insertmode = FALSE;

    // Need to save the line for undo before inserting the first char.
    ins_need_undo = TRUE;

    where_paste_started.lnum = 0;
    can_cindent = TRUE;
#ifdef FEAT_FOLDING
    // The cursor line is not in a closed fold, unless 'insertmode' is set or
    // restarting.
    if (!p_im && did_restart_edit == 0)
	foldOpenCursor();
#endif

    /*
     * If 'showmode' is set, show the current (insert/replace/..) mode.
     * A warning message for changing a readonly file is given here, before
     * actually changing anything.  It's put after the mode, if any.
     */
    i = 0;
    if (p_smd && msg_silent == 0)
	i = showmode();

    if (!p_im && did_restart_edit == 0)
	change_warning(i == 0 ? 0 : i + 1);

#ifdef CURSOR_SHAPE
    ui_cursor_shape();		// may show different cursor shape
#endif
#ifdef FEAT_DIGRAPHS
    do_digraph(-1);		// clear digraphs
#endif

    /*
     * Get the current length of the redo buffer, those characters have to be
     * skipped if we want to get to the inserted characters.
     */
    ptr = get_inserted();
    if (ptr == NULL)
	new_insert_skip = 0;
    else
    {
	new_insert_skip = (int)STRLEN(ptr);
	vim_free(ptr);
    }

    old_indent = 0;

    /*
     * Main loop in Insert mode: repeat until Insert mode is left.
     */
    for (;;)
    {
#ifdef FEAT_RIGHTLEFT
	if (!revins_legal)
	    revins_scol = -1;	    // reset on illegal motions
	else
	    revins_legal = 0;
#endif
	if (arrow_used)	    // don't repeat insert when arrow key used
	    count = 0;

	if (update_Insstart_orig)
	    Insstart_orig = Insstart;

	if (stop_insert_mode && !ins_compl_active())
	{
	    // ":stopinsert" used or 'insertmode' reset
	    count = 0;
	    goto doESCkey;
	}

	// set curwin->w_curswant for next K_DOWN or K_UP
	if (!arrow_used)
	    curwin->w_set_curswant = TRUE;

	// If there is no typeahead may check for timestamps (e.g., for when a
	// menu invoked a shell command).
	if (stuff_empty())
	{
	    did_check_timestamps = FALSE;
	    if (need_check_timestamps)
		check_timestamps(FALSE);
	}

	/*
	 * When emsg() was called msg_scroll will have been set.
	 */
	msg_scroll = FALSE;

#ifdef FEAT_GUI
	// When 'mousefocus' is set a mouse movement may have taken us to
	// another window.  "need_mouse_correct" may then be set because of an
	// autocommand.
	if (need_mouse_correct)
	    gui_mouse_correct();
#endif

#ifdef FEAT_FOLDING
	// Open fold at the cursor line, according to 'foldopen'.
	if (fdo_flags & FDO_INSERT)
	    foldOpenCursor();
	// Close folds where the cursor isn't, according to 'foldclose'
	if (!char_avail())
	    foldCheckClose();
#endif

#ifdef FEAT_JOB_CHANNEL
	if (bt_prompt(curbuf))
	{
	    init_prompt(cmdchar_todo);
	    cmdchar_todo = NUL;
	}
#endif

	/*
	 * If we inserted a character at the last position of the last line in
	 * the window, scroll the window one line up. This avoids an extra
	 * redraw.
	 * This is detected when the cursor column is smaller after inserting
	 * something.
	 * Don't do this when the topline changed already, it has
	 * already been adjusted (by insertchar() calling open_line())).
	 * Also don't do this when 'smoothscroll' is set, as the window should
	 * then be scrolled by screen lines.
	 */
	if (curbuf->b_mod_set
		&& curwin->w_p_wrap
		&& !curwin->w_p_sms
		&& !did_backspace
		&& curwin->w_topline == old_topline
#ifdef FEAT_DIFF
		&& curwin->w_topfill == old_topfill
#endif
		)
	{
	    mincol = curwin->w_wcol;
	    validate_cursor_col();

	    if (
#ifdef FEAT_VARTABS
		curwin->w_wcol < mincol - tabstop_at(
					  get_nolist_virtcol(), curbuf->b_p_ts,
							 curbuf->b_p_vts_array)
#else
		(int)curwin->w_wcol < mincol - curbuf->b_p_ts
#endif
		    && curwin->w_wrow ==
				   curwin->w_height - 1 - get_scrolloff_value()
		    && (curwin->w_cursor.lnum != curwin->w_topline
#ifdef FEAT_DIFF
			|| curwin->w_topfill > 0
#endif
		    ))
	    {
#ifdef FEAT_DIFF
		if (curwin->w_topfill > 0)
		    --curwin->w_topfill;
		else
#endif
#ifdef FEAT_FOLDING
		if (hasFolding(curwin->w_topline, NULL, &old_topline))
		    set_topline(curwin, old_topline + 1);
		else
#endif
		    set_topline(curwin, curwin->w_topline + 1);
	    }
	}

	// May need to adjust w_topline to show the cursor.
	update_topline();

	did_backspace = FALSE;

	validate_cursor();		// may set must_redraw

	/*
	 * Redraw the display when no characters are waiting.
	 * Also shows mode, ruler and positions cursor.
	 */
	ins_redraw(TRUE);

	if (curwin->w_p_scb)
	    do_check_scrollbind(TRUE);

	if (curwin->w_p_crb)
	    do_check_cursorbind();
	update_curswant();
	old_topline = curwin->w_topline;
#ifdef FEAT_DIFF
	old_topfill = curwin->w_topfill;
#endif

#ifdef USE_ON_FLY_SCROLL
	dont_scroll = FALSE;		// allow scrolling here
#endif
	// May request the keyboard protocol state now.
	may_send_t_RK();

	/*
	 * Get a character for Insert mode.  Ignore K_IGNORE and K_NOP.
	 */
	if (c != K_CURSORHOLD)
	    lastc = c;		// remember the previous char for CTRL-D

	// After using CTRL-G U the next cursor key will not break undo.
	if (dont_sync_undo == MAYBE)
	    dont_sync_undo = TRUE;
	else
	    dont_sync_undo = FALSE;
	if (cmdchar == K_PS)
	    // Got here from normal mode when bracketed paste started.
	    c = K_PS;
	else
	    do
	    {
		c = safe_vgetc();

		if (stop_insert_mode
#ifdef FEAT_TERMINAL
			|| (c == K_IGNORE && term_use_loop())
#endif
		   )
		{
		    // Insert mode ended, possibly from a callback, or a timer
		    // must have opened a terminal window.
		    if (c != K_IGNORE && c != K_NOP)
			vungetc(c);
		    count = 0;
		    nomove = TRUE;
		    ins_compl_prep(ESC);
		    goto doESCkey;
		}
	    } while (c == K_IGNORE || c == K_NOP);

	// Don't want K_CURSORHOLD for the second key, e.g., after CTRL-V.
	did_cursorhold = TRUE;

#ifdef FEAT_RIGHTLEFT
	if (p_hkmap && KeyTyped)
	    c = hkmap(c);		// Hebrew mode mapping
#endif

	// If the window was made so small that nothing shows, make it at least
	// one line and one column when typing.
	if (KeyTyped && !KeyStuffed)
	    win_ensure_size();

	/*
	 * Special handling of keys while the popup menu is visible or wanted
	 * and the cursor is still in the completed word.  Only when there is
	 * a match, skip this when no matches were found.
	 */
	if (ins_compl_active()
		&& pum_wanted()
		&& curwin->w_cursor.col >= ins_compl_col()
		&& ins_compl_has_shown_match())
	{
	    // BS: Delete one character from "compl_leader".
	    if ((c == K_BS || c == Ctrl_H)
			&& curwin->w_cursor.col > ins_compl_col()
			&& (c = ins_compl_bs()) == NUL)
		continue;

	    // When no match was selected or it was edited.
	    if (!ins_compl_used_match())
	    {
		// CTRL-L: Add one character from the current match to
		// "compl_leader".  Except when at the original match and
		// there is nothing to add, CTRL-L works like CTRL-P then.
		if (c == Ctrl_L
			&& (!ctrl_x_mode_line_or_eval()
			    || ins_compl_long_shown_match()))
		{
		    ins_compl_addfrommatch();
		    continue;
		}

		// A non-white character that fits in with the current
		// completion: Add to "compl_leader".
		if (ins_compl_accept_char(c))
		{
#if defined(FEAT_EVAL)
		    // Trigger InsertCharPre.
		    char_u *str = do_insert_char_pre(c);
		    char_u *p;

		    if (str != NULL)
		    {
			for (p = str; *p != NUL; MB_PTR_ADV(p))
			    ins_compl_addleader(PTR2CHAR(p));
			vim_free(str);
		    }
		    else
#endif
			ins_compl_addleader(c);
		    continue;
		}

		// Pressing CTRL-Y selects the current match.  When
		// ins_compl_enter_selects() is set the Enter key does the
		// same.
		if ((c == Ctrl_Y || (ins_compl_enter_selects()
				    && (c == CAR || c == K_KENTER || c == NL)))
			&& stop_arrow() == OK)
		{
		    ins_compl_delete();
		    ins_compl_insert(FALSE);
		}
	    }
	}

	// Prepare for or stop CTRL-X mode.  This doesn't do completion, but
	// it does fix up the text when finishing completion.
	ins_compl_init_get_longest();
	if (ins_compl_prep(c))
	    continue;

	// CTRL-\ CTRL-N goes to Normal mode,
	// CTRL-\ CTRL-G goes to mode selected with 'insertmode',
	// CTRL-\ CTRL-O is like CTRL-O but without moving the cursor.
	if (c == Ctrl_BSL)
	{
	    // may need to redraw when no more chars available now
	    ins_redraw(FALSE);
	    ++no_mapping;
	    ++allow_keys;
	    c = plain_vgetc();
	    --no_mapping;
	    --allow_keys;
	    if (c != Ctrl_N && c != Ctrl_G && c != Ctrl_O)
	    {
		// it's something else
		vungetc(c);
		c = Ctrl_BSL;
	    }
	    else if (c == Ctrl_G && p_im)
		continue;
	    else
	    {
		if (c == Ctrl_O)
		{
		    ins_ctrl_o();
		    ins_at_eol = FALSE;	// cursor keeps its column
		    nomove = TRUE;
		}
		count = 0;
		goto doESCkey;
	    }
	}

#ifdef FEAT_DIGRAPHS
	c = do_digraph(c);
#endif

	if ((c == Ctrl_V || c == Ctrl_Q) && ctrl_x_mode_cmdline())
	    goto docomplete;
	if (c == Ctrl_V || c == Ctrl_Q)
	{
	    ins_ctrl_v();
	    c = Ctrl_V;	// pretend CTRL-V is last typed character
	    continue;
	}

	if (cindent_on() && ctrl_x_mode_none())
	{
	    // A key name preceded by a bang means this key is not to be
	    // inserted.  Skip ahead to the re-indenting below.
	    // A key name preceded by a star means that indenting has to be
	    // done before inserting the key.
	    line_is_white = inindent(0);
	    if (in_cinkeys(c, '!', line_is_white))
		goto force_cindent;
	    if (can_cindent && in_cinkeys(c, '*', line_is_white)
							&& stop_arrow() == OK)
		do_c_expr_indent();
	}

#ifdef FEAT_RIGHTLEFT
	if (curwin->w_p_rl)
	    switch (c)
	    {
		case K_LEFT:	c = K_RIGHT; break;
		case K_S_LEFT:	c = K_S_RIGHT; break;
		case K_C_LEFT:	c = K_C_RIGHT; break;
		case K_RIGHT:	c = K_LEFT; break;
		case K_S_RIGHT: c = K_S_LEFT; break;
		case K_C_RIGHT: c = K_C_LEFT; break;
	    }
#endif

	/*
	 * If 'keymodel' contains "startsel", may start selection.  If it
	 * does, a CTRL-O and c will be stuffed, we need to get these
	 * characters.
	 */
	if (ins_start_select(c))
	    continue;

	/*
	 * The big switch to handle a character in insert mode.
	 */
	switch (c)
	{
	case ESC:	// End input mode
	    if (echeck_abbr(ESC + ABBR_OFF))
		break;
	    // FALLTHROUGH

	case Ctrl_C:	// End input mode
	    if (c == Ctrl_C && cmdwin_type != 0)
	    {
		// Close the cmdline window.
		cmdwin_result = K_IGNORE;
		got_int = FALSE; // don't stop executing autocommands et al.
		nomove = TRUE;
		goto doESCkey;
	    }
#ifdef FEAT_JOB_CHANNEL
	    if (c == Ctrl_C && bt_prompt(curbuf))
	    {
		if (invoke_prompt_interrupt())
		{
		    if (!bt_prompt(curbuf))
			// buffer changed to a non-prompt buffer, get out of
			// Insert mode
			goto doESCkey;
		    break;
		}
	    }
#endif

#ifdef UNIX
do_intr:
#endif
	    // when 'insertmode' set, and not halfway a mapping, don't leave
	    // Insert mode
	    if (goto_im())
	    {
		if (got_int)
		{
		    (void)vgetc();		// flush all buffers
		    got_int = FALSE;
		}
		else
		    vim_beep(BO_IM);
		break;
	    }
doESCkey:
	    /*
	     * This is the ONLY return from edit()!
	     */
	    // Always update o_lnum, so that a "CTRL-O ." that adds a line
	    // still puts the cursor back after the inserted text.
	    if (ins_at_eol && gchar_cursor() == NUL)
		o_lnum = curwin->w_cursor.lnum;

	    if (ins_esc(&count, cmdchar, nomove))
	    {
		// When CTRL-C was typed got_int will be set, with the result
		// that the autocommands won't be executed. When mapped got_int
		// is not set, but let's keep the behavior the same.
		if (cmdchar != 'r' && cmdchar != 'v' && c != Ctrl_C)
		    ins_apply_autocmds(EVENT_INSERTLEAVE);
		did_cursorhold = FALSE;
		curbuf->b_last_changedtick = CHANGEDTICK(curbuf);
		return (c == Ctrl_O);
	    }
	    continue;

	case Ctrl_Z:	// suspend when 'insertmode' set
	    if (!p_im)
		goto normalchar;	// insert CTRL-Z as normal char
	    do_cmdline_cmd((char_u *)"stop");
#ifdef CURSOR_SHAPE
	    ui_cursor_shape();		// may need to update cursor shape
#endif
	    continue;

	case Ctrl_O:	// execute one command
#ifdef FEAT_COMPL_FUNC
	    if (ctrl_x_mode_omni())
		goto docomplete;
#endif
	    if (echeck_abbr(Ctrl_O + ABBR_OFF))
		break;
	    ins_ctrl_o();

	    // don't move the cursor left when 'virtualedit' has "onemore".
	    if (get_ve_flags() & VE_ONEMORE)
	    {
		ins_at_eol = FALSE;
		nomove = TRUE;
	    }
	    count = 0;
	    goto doESCkey;

	case K_INS:	// toggle insert/replace mode
	case K_KINS:
	    ins_insert(replaceState);
	    break;

	case K_SELECT:	// end of Select mode mapping - ignore
	    break;

	case K_HELP:	// Help key works like <ESC> <Help>
	case K_F1:
	case K_XF1:
	    stuffcharReadbuff(K_HELP);
	    if (p_im)
		need_start_insertmode = TRUE;
	    goto doESCkey;

#ifdef FEAT_NETBEANS_INTG
	case K_F21:	// NetBeans command
	    ++no_mapping;		// don't map the next key hits
	    i = plain_vgetc();
	    --no_mapping;
	    netbeans_keycommand(i);
	    break;
#endif

	case K_ZERO:	// Insert the previously inserted text.
	case NUL:
	case Ctrl_A:
	    // For ^@ the trailing ESC will end the insert, unless there is an
	    // error.
	    if (stuff_inserted(NUL, 1L, (c == Ctrl_A)) == FAIL
						   && c != Ctrl_A && !p_im)
		goto doESCkey;		// quit insert mode
	    inserted_space = FALSE;
	    break;

	case Ctrl_R:	// insert the contents of a register
	    ins_reg();
	    auto_format(FALSE, TRUE);
	    inserted_space = FALSE;
	    break;

	case Ctrl_G:	// commands starting with CTRL-G
	    ins_ctrl_g();
	    break;

	case Ctrl_HAT:	// switch input mode and/or langmap
	    ins_ctrl_hat();
	    break;

#ifdef FEAT_RIGHTLEFT
	case Ctrl__:	// switch between languages
	    if (!p_ari)
		goto normalchar;
	    ins_ctrl_();
	    break;
#endif

	case Ctrl_D:	// Make indent one shiftwidth smaller.
#if defined(FEAT_FIND_ID)
	    if (ctrl_x_mode_path_defines())
		goto docomplete;
#endif
	    // FALLTHROUGH

	case Ctrl_T:	// Make indent one shiftwidth greater.
	    if (c == Ctrl_T && ctrl_x_mode_thesaurus())
	    {
		if (has_compl_option(FALSE))
		    goto docomplete;
		break;
	    }

	    ins_shift(c, lastc);
	    auto_format(FALSE, TRUE);
	    inserted_space = FALSE;
	    break;

	case K_DEL:	// delete character under the cursor
	case K_KDEL:
	    ins_del();
	    auto_format(FALSE, TRUE);
	    break;

	case K_BS:	// delete character before the cursor
	case K_S_BS:
	case Ctrl_H:
	    did_backspace = ins_bs(c, BACKSPACE_CHAR, &inserted_space);
	    auto_format(FALSE, TRUE);
	    break;

	case Ctrl_W:	// delete word before the cursor
#ifdef FEAT_JOB_CHANNEL
	    if (bt_prompt(curbuf) && (mod_mask & MOD_MASK_SHIFT) == 0)
	    {
		// In a prompt window CTRL-W is used for window commands.
		// Use Shift-CTRL-W to delete a word.
		stuffcharReadbuff(Ctrl_W);
		restart_edit = 'A';
		nomove = TRUE;
		count = 0;
		goto doESCkey;
	    }
#endif
	    did_backspace = ins_bs(c, BACKSPACE_WORD, &inserted_space);
	    auto_format(FALSE, TRUE);
	    break;

	case Ctrl_U:	// delete all inserted text in current line
# ifdef FEAT_COMPL_FUNC
	    // CTRL-X CTRL-U completes with 'completefunc'.
	    if (ctrl_x_mode_function())
		goto docomplete;
# endif
	    did_backspace = ins_bs(c, BACKSPACE_LINE, &inserted_space);
	    auto_format(FALSE, TRUE);
	    inserted_space = FALSE;
	    break;

	case K_LEFTMOUSE:   // mouse keys
	case K_LEFTMOUSE_NM:
	case K_LEFTDRAG:
	case K_LEFTRELEASE:
	case K_LEFTRELEASE_NM:
	case K_MOUSEMOVE:
	case K_MIDDLEMOUSE:
	case K_MIDDLEDRAG:
	case K_MIDDLERELEASE:
	case K_RIGHTMOUSE:
	case K_RIGHTDRAG:
	case K_RIGHTRELEASE:
	case K_X1MOUSE:
	case K_X1DRAG:
	case K_X1RELEASE:
	case K_X2MOUSE:
	case K_X2DRAG:
	case K_X2RELEASE:
	    ins_mouse(c);
	    break;

	case K_MOUSEDOWN: // Default action for scroll wheel up: scroll up
	    ins_mousescroll(MSCR_DOWN);
	    break;

	case K_MOUSEUP:	// Default action for scroll wheel down: scroll down
	    ins_mousescroll(MSCR_UP);
	    break;

	case K_MOUSELEFT: // Scroll wheel left
	    ins_mousescroll(MSCR_LEFT);
	    break;

	case K_MOUSERIGHT: // Scroll wheel right
	    ins_mousescroll(MSCR_RIGHT);
	    break;

	case K_PS:
	    bracketed_paste(PASTE_INSERT, FALSE, NULL);
	    if (cmdchar == K_PS)
		// invoked from normal mode, bail out
		goto doESCkey;
	    break;
	case K_PE:
	    // Got K_PE without K_PS, ignore.
	    break;

#ifdef FEAT_GUI_TABLINE
	case K_TABLINE:
	case K_TABMENU:
	    ins_tabline(c);
	    break;
#endif

	case K_IGNORE:	// Something mapped to nothing
	    break;

	case K_COMMAND:		    // <Cmd>command<CR>
	case K_SCRIPT_COMMAND:	    // <ScriptCmd>command<CR>
	    {
		do_cmdkey_command(c, 0);

#ifdef FEAT_TERMINAL
		if (term_use_loop())
		    // Started a terminal that gets the input, exit Insert mode.
		    goto doESCkey;
#endif
		if (curbuf->b_u_synced)
		    // The command caused undo to be synced.  Need to save the
		    // line for undo before inserting the next char.
		    ins_need_undo = TRUE;
	    }
	    break;

	case K_CURSORHOLD:	// Didn't type something for a while.
	    ins_apply_autocmds(EVENT_CURSORHOLDI);
	    did_cursorhold = TRUE;
	    // If CTRL-G U was used apply it to the next typed key.
	    if (dont_sync_undo == TRUE)
		dont_sync_undo = MAYBE;
	    break;

#ifdef FEAT_GUI_MSWIN
	    // On MS-Windows ignore <M-F4>, we get it when closing the window
	    // was cancelled.
	case K_F4:
	    if (mod_mask != MOD_MASK_ALT)
		goto normalchar;
	    break;
#endif

#ifdef FEAT_GUI
	case K_VER_SCROLLBAR:
	    ins_scroll();
	    break;

	case K_HOR_SCROLLBAR:
	    ins_horscroll();
	    break;
#endif

	case K_HOME:	// <Home>
	case K_KHOME:
	case K_S_HOME:
	case K_C_HOME:
	    ins_home(c);
	    break;

	case K_END:	// <End>
	case K_KEND:
	case K_S_END:
	case K_C_END:
	    ins_end(c);
	    break;

	case K_LEFT:	// <Left>
	    if (mod_mask & (MOD_MASK_SHIFT|MOD_MASK_CTRL))
		ins_s_left();
	    else
		ins_left();
	    break;

	case K_S_LEFT:	// <S-Left>
	case K_C_LEFT:
	    ins_s_left();
	    break;

	case K_RIGHT:	// <Right>
	    if (mod_mask & (MOD_MASK_SHIFT|MOD_MASK_CTRL))
		ins_s_right();
	    else
		ins_right();
	    break;

	case K_S_RIGHT:	// <S-Right>
	case K_C_RIGHT:
	    ins_s_right();
	    break;

	case K_UP:	// <Up>
	    if (pum_visible())
		goto docomplete;
	    if (mod_mask & MOD_MASK_SHIFT)
		ins_pageup();
	    else
		ins_up(FALSE);
	    break;

	case K_S_UP:	// <S-Up>
	case K_PAGEUP:
	case K_KPAGEUP:
	    if (pum_visible())
		goto docomplete;
	    ins_pageup();
	    break;

	case K_DOWN:	// <Down>
	    if (pum_visible())
		goto docomplete;
	    if (mod_mask & MOD_MASK_SHIFT)
		ins_pagedown();
	    else
		ins_down(FALSE);
	    break;

	case K_S_DOWN:	// <S-Down>
	case K_PAGEDOWN:
	case K_KPAGEDOWN:
	    if (pum_visible())
		goto docomplete;
	    ins_pagedown();
	    break;

#ifdef FEAT_DND
	case K_DROP:	// drag-n-drop event
	    ins_drop();
	    break;
#endif

	case K_S_TAB:	// When not mapped, use like a normal TAB
	    c = TAB;
	    // FALLTHROUGH

	case TAB:	// TAB or Complete patterns along path
#if defined(FEAT_FIND_ID)
	    if (ctrl_x_mode_path_patterns())
		goto docomplete;
#endif
	    inserted_space = FALSE;
	    if (ins_tab())
		goto normalchar;	// insert TAB as a normal char
	    auto_format(FALSE, TRUE);
	    break;

	case K_KENTER:	// <Enter>
	    c = CAR;
	    // FALLTHROUGH
	case CAR:
	case NL:
#if defined(FEAT_QUICKFIX)
	    // In a quickfix window a <CR> jumps to the error under the
	    // cursor.
	    if (bt_quickfix(curbuf) && c == CAR)
	    {
		if (curwin->w_llist_ref == NULL)    // quickfix window
		    do_cmdline_cmd((char_u *)".cc");
		else				    // location list window
		    do_cmdline_cmd((char_u *)".ll");
		break;
	    }
#endif
	    if (cmdwin_type != 0)
	    {
		// Execute the command in the cmdline window.
		cmdwin_result = CAR;
		goto doESCkey;
	    }
#ifdef FEAT_JOB_CHANNEL
	    if (bt_prompt(curbuf))
	    {
		invoke_prompt_callback();
		if (!bt_prompt(curbuf))
		    // buffer changed to a non-prompt buffer, get out of
		    // Insert mode
		    goto doESCkey;
		break;
	    }
#endif
	    if (ins_eol(c) == FAIL && !p_im)
		goto doESCkey;	    // out of memory
	    auto_format(FALSE, FALSE);
	    inserted_space = FALSE;
	    break;

	case Ctrl_K:	    // digraph or keyword completion
	    if (ctrl_x_mode_dictionary())
	    {
		if (has_compl_option(TRUE))
		    goto docomplete;
		break;
	    }
#ifdef FEAT_DIGRAPHS
	    c = ins_digraph();
	    if (c == NUL)
		break;
#endif
	    goto normalchar;

	case Ctrl_X:	// Enter CTRL-X mode
	    ins_ctrl_x();
	    break;

	case Ctrl_RSB:	// Tag name completion after ^X
	    if (!ctrl_x_mode_tags())
		goto normalchar;
	    goto docomplete;

	case Ctrl_F:	// File name completion after ^X
	    if (!ctrl_x_mode_files())
		goto normalchar;
	    goto docomplete;

	case 's':	// Spelling completion after ^X
	case Ctrl_S:
	    if (!ctrl_x_mode_spell())
		goto normalchar;
	    goto docomplete;

	case Ctrl_L:	// Whole line completion after ^X
	    if (!ctrl_x_mode_whole_line())
	    {
		// CTRL-L with 'insertmode' set: Leave Insert mode
		if (p_im)
		{
		    if (echeck_abbr(Ctrl_L + ABBR_OFF))
			break;
		    goto doESCkey;
		}
		goto normalchar;
	    }
	    // FALLTHROUGH

	case Ctrl_P:	// Do previous/next pattern completion
	case Ctrl_N:
	    // if 'complete' is empty then plain ^P is no longer special,
	    // but it is under other ^X modes
	    if (*curbuf->b_p_cpt == NUL
		    && (ctrl_x_mode_normal() || ctrl_x_mode_whole_line())
		    && !compl_status_local())
		goto normalchar;

docomplete:
	    compl_busy = TRUE;
#ifdef FEAT_FOLDING
	    disable_fold_update++;  // don't redraw folds here
#endif
	    if (ins_complete(c, TRUE) == FAIL)
		compl_status_clear();
#ifdef FEAT_FOLDING
	    disable_fold_update--;
#endif
	    compl_busy = FALSE;
	    can_si = may_do_si(); // allow smartindenting
	    break;

	case Ctrl_Y:	// copy from previous line or scroll down
	case Ctrl_E:	// copy from next line	   or scroll up
	    c = ins_ctrl_ey(c);
	    break;

	  default:
#ifdef UNIX
	    if (c == intr_char)		// special interrupt char
		goto do_intr;
#endif

normalchar:
	    /*
	     * Insert a normal character.
	     */
#if defined(FEAT_EVAL)
	    if (!p_paste)
	    {
		// Trigger InsertCharPre.
		char_u *str = do_insert_char_pre(c);
		char_u *p;

		if (str != NULL)
		{
		    if (*str != NUL && stop_arrow() != FAIL)
		    {
			// Insert the new value of v:char literally.
			for (p = str; *p != NUL; MB_PTR_ADV(p))
			{
			    c = PTR2CHAR(p);
			    if (c == CAR || c == K_KENTER || c == NL)
				ins_eol(c);
			    else
				ins_char(c);
			}
			AppendToRedobuffLit(str, -1);
		    }
		    vim_free(str);
		    c = NUL;
		}

		// If the new value is already inserted or an empty string
		// then don't insert any character.
		if (c == NUL)
		    break;
	    }
#endif
	    // Try to perform smart-indenting.
	    ins_try_si(c);

	    if (c == ' ')
	    {
		inserted_space = TRUE;
		if (inindent(0))
		    can_cindent = FALSE;
		if (Insstart_blank_vcol == MAXCOL
			&& curwin->w_cursor.lnum == Insstart.lnum)
		    Insstart_blank_vcol = get_nolist_virtcol();
	    }

	    // Insert a normal character and check for abbreviations on a
	    // special character.  Let CTRL-] expand abbreviations without
	    // inserting it.
	    if (vim_iswordc(c) || (!echeck_abbr(
			// Add ABBR_OFF for characters above 0x100, this is
			// what check_abbr() expects.
				(has_mbyte && c >= 0x100) ? (c + ABBR_OFF) : c)
			&& c != Ctrl_RSB))
	    {
		insert_special(c, FALSE, FALSE);
#ifdef FEAT_RIGHTLEFT
		revins_legal++;
		revins_chars++;
#endif
	    }

	    auto_format(FALSE, TRUE);

#ifdef FEAT_FOLDING
	    // When inserting a character the cursor line must never be in a
	    // closed fold.
	    foldOpenCursor();
#endif
	    break;
	}   // end of switch (c)

	// If typed something may trigger CursorHoldI again.
	if (c != K_CURSORHOLD
#ifdef FEAT_COMPL_FUNC
		// but not in CTRL-X mode, a script can't restore the state
		&& ctrl_x_mode_normal()
#endif
	       )
	    did_cursorhold = FALSE;

	// If the cursor was moved we didn't just insert a space
	if (arrow_used)
	    inserted_space = FALSE;

	if (can_cindent && cindent_on() && ctrl_x_mode_normal())
	{
force_cindent:
	    /*
	     * Indent now if a key was typed that is in 'cinkeys'.
	     */
	    if (in_cinkeys(c, ' ', line_is_white))
	    {
		if (stop_arrow() == OK)
		    // re-indent the current line
		    do_c_expr_indent();
	    }
	}

    }	// for (;;)
    // NOTREACHED
}

    int
ins_need_undo_get(void)
{
    return ins_need_undo;
}

/*
 * Redraw for Insert mode.
 * This is postponed until getting the next character to make '$' in the 'cpo'
 * option work correctly.
 * Only redraw when there are no characters available.  This speeds up
 * inserting sequences of characters (e.g., for CTRL-R).
 */
    void
ins_redraw(int ready)	    // not busy with something
{
#ifdef FEAT_CONCEAL
    linenr_T	conceal_old_cursor_line = 0;
    linenr_T	conceal_new_cursor_line = 0;
    int		conceal_update_lines = FALSE;
#endif

    if (char_avail())
	return;

    // Trigger CursorMoved if the cursor moved.  Not when the popup menu is
    // visible, the command might delete it.
    if (ready && (has_cursormovedI()
# ifdef FEAT_PROP_POPUP
		|| popup_visible
# endif
# if defined(FEAT_CONCEAL)
		|| curwin->w_p_cole > 0
# endif
		)
	    && !EQUAL_POS(last_cursormoved, curwin->w_cursor)
	    && !pum_visible())
    {
# ifdef FEAT_SYN_HL
	// Need to update the screen first, to make sure syntax
	// highlighting is correct after making a change (e.g., inserting
	// a "(".  The autocommand may also require a redraw, so it's done
	// again below, unfortunately.
	if (syntax_present(curwin) && must_redraw)
	    update_screen(0);
# endif
	if (has_cursormovedI())
	{
	    // Make sure curswant is correct, an autocommand may call
	    // getcurpos().
	    update_curswant();
	    ins_apply_autocmds(EVENT_CURSORMOVEDI);
	}
#ifdef FEAT_PROP_POPUP
	if (popup_visible)
	    popup_check_cursor_pos();
#endif
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

    // Trigger TextChangedI if b_changedtick_i differs.
    if (ready && has_textchangedI()
	    && curbuf->b_last_changedtick_i != CHANGEDTICK(curbuf)
	    && !pum_visible())
    {
	aco_save_T	aco;
	varnumber_T	tick = CHANGEDTICK(curbuf);

	// Save and restore curwin and curbuf, in case the autocmd changes
	// them.
	aucmd_prepbuf(&aco, curbuf);
	apply_autocmds(EVENT_TEXTCHANGEDI, NULL, NULL, FALSE, curbuf);
	aucmd_restbuf(&aco);
	curbuf->b_last_changedtick_i = CHANGEDTICK(curbuf);
	if (tick != CHANGEDTICK(curbuf))  // see ins_apply_autocmds()
	    u_save(curwin->w_cursor.lnum,
					(linenr_T)(curwin->w_cursor.lnum + 1));
    }

    // Trigger TextChangedP if b_changedtick_pum differs. When the popupmenu
    // closes TextChangedI will need to trigger for backwards compatibility,
    // thus use different b_last_changedtick* variables.
    if (ready && has_textchangedP()
	    && curbuf->b_last_changedtick_pum != CHANGEDTICK(curbuf)
	    && pum_visible())
    {
	aco_save_T	aco;
	varnumber_T	tick = CHANGEDTICK(curbuf);

	// Save and restore curwin and curbuf, in case the autocmd changes
	// them.
	aucmd_prepbuf(&aco, curbuf);
	apply_autocmds(EVENT_TEXTCHANGEDP, NULL, NULL, FALSE, curbuf);
	aucmd_restbuf(&aco);
	curbuf->b_last_changedtick_pum = CHANGEDTICK(curbuf);
	if (tick != CHANGEDTICK(curbuf))  // see ins_apply_autocmds()
	    u_save(curwin->w_cursor.lnum,
					(linenr_T)(curwin->w_cursor.lnum + 1));
    }

    if (ready)
	may_trigger_win_scrolled_resized();

    // Trigger SafeState if nothing is pending.
    may_trigger_safestate(ready
	    && !ins_compl_active()
	    && !pum_visible());

#if defined(FEAT_CONCEAL)
    if ((conceal_update_lines
	    && (conceal_old_cursor_line != conceal_new_cursor_line
		|| conceal_cursor_line(curwin)))
	    || need_cursor_line_redraw)
    {
	if (conceal_old_cursor_line != conceal_new_cursor_line)
	    redrawWinline(curwin, conceal_old_cursor_line);
	redrawWinline(curwin, conceal_new_cursor_line == 0
			    ? curwin->w_cursor.lnum : conceal_new_cursor_line);
	curwin->w_valid &= ~VALID_CROW;
	need_cursor_line_redraw = FALSE;
    }
#endif
    if (must_redraw)
	update_screen(0);
    else if (clear_cmdline || redraw_cmdline)
	showmode();		// clear cmdline and show mode
    showruler(FALSE);
    setcursor();
    emsg_on_display = FALSE;	// may remove error message now
}

/*
 * Handle a CTRL-V or CTRL-Q typed in Insert mode.
 */
    static void
ins_ctrl_v(void)
{
    int		c;
    int		did_putchar = FALSE;

    // may need to redraw when no more chars available now
    ins_redraw(FALSE);

    if (redrawing() && !char_avail())
    {
	edit_putchar('^', TRUE);
	did_putchar = TRUE;
    }
    AppendToRedobuff((char_u *)CTRL_V_STR);	// CTRL-V

    add_to_showcmd_c(Ctrl_V);

    // Do not change any modifyOtherKeys ESC sequence to a normal key for
    // CTRL-SHIFT-V.
    c = get_literal(mod_mask & MOD_MASK_SHIFT);
    if (did_putchar)
	// when the line fits in 'columns' the '^' is at the start of the next
	// line and will not removed by the redraw
	edit_unputchar();
    clear_showcmd();

    insert_special(c, FALSE, TRUE);
#ifdef FEAT_RIGHTLEFT
    revins_chars++;
    revins_legal++;
#endif
}

/*
 * After getting an ESC or CSI for a literal key: If the typeahead buffer
 * contains a modifyOtherKeys sequence then decode it and return the result.
 * Otherwise return "c".
 * Note that this doesn't wait for characters, they must be in the typeahead
 * buffer already.
 */
    static int
decodeModifyOtherKeys(int c)
{
    char_u  *p = typebuf.tb_buf + typebuf.tb_off;
    int	    idx;
    int	    form = 0;
    int	    argidx = 0;
    int	    arg[2] = {0, 0};

    // Recognize:
    // form 0: {lead}{key};{modifier}u
    // form 1: {lead}27;{modifier};{key}~
    if (typebuf.tb_len >= 4 && (c == CSI || (c == ESC && *p == '[')))
    {
	idx = (*p == '[');
	if (p[idx] == '2' && p[idx + 1] == '7' && p[idx + 2] == ';')
	{
	    form = 1;
	    idx += 3;
	}
	while (idx < typebuf.tb_len && argidx < 2)
	{
	    if (p[idx] == ';')
		++argidx;
	    else if (VIM_ISDIGIT(p[idx]))
		arg[argidx] = arg[argidx] * 10 + (p[idx] - '0');
	    else
		break;
	    ++idx;
	}
	if (idx < typebuf.tb_len
		&& p[idx] == (form == 1 ? '~' : 'u')
		&& argidx == 1)
	{
	    // Match, consume the code.
	    typebuf.tb_off += idx + 1;
	    typebuf.tb_len -= idx + 1;
#if defined(FEAT_CLIENTSERVER) || defined(FEAT_EVAL)
	    if (typebuf.tb_len == 0)
		typebuf_was_filled = FALSE;
#endif

	    mod_mask = decode_modifiers(arg[!form]);
	    c = merge_modifyOtherKeys(arg[form], &mod_mask);
	}
    }

    return c;
}

/*
 * Put a character directly onto the screen.  It's not stored in a buffer.
 * Used while handling CTRL-K, CTRL-V, etc. in Insert mode.
 */
static int  pc_status;
#define PC_STATUS_UNSET	0	// pc_bytes was not set
#define PC_STATUS_RIGHT	1	// right half of double-wide char
#define PC_STATUS_LEFT	2	// left half of double-wide char
#define PC_STATUS_SET	3	// pc_bytes was filled
static char_u pc_bytes[MB_MAXBYTES + 1]; // saved bytes
static int  pc_attr;
static int  pc_row;
static int  pc_col;

    void
edit_putchar(int c, int highlight)
{
    int	    attr;

    if (ScreenLines == NULL)
	return;

    update_topline();	// just in case w_topline isn't valid
    validate_cursor();
    if (highlight)
	attr = HL_ATTR(HLF_8);
    else
	attr = 0;
    pc_row = W_WINROW(curwin) + curwin->w_wrow;
    pc_col = curwin->w_wincol;
    pc_status = PC_STATUS_UNSET;
#ifdef FEAT_RIGHTLEFT
    if (curwin->w_p_rl)
    {
	pc_col += curwin->w_width - 1 - curwin->w_wcol;
	if (has_mbyte)
	{
	    int fix_col = mb_fix_col(pc_col, pc_row);

	    if (fix_col != pc_col)
	    {
		screen_putchar(' ', pc_row, fix_col, attr);
		--curwin->w_wcol;
		pc_status = PC_STATUS_RIGHT;
	    }
	}
    }
    else
#endif
    {
	pc_col += curwin->w_wcol;
	if (mb_lefthalve(pc_row, pc_col))
	    pc_status = PC_STATUS_LEFT;
    }

    // save the character to be able to put it back
    if (pc_status == PC_STATUS_UNSET)
    {
	screen_getbytes(pc_row, pc_col, pc_bytes, &pc_attr);
	pc_status = PC_STATUS_SET;
    }
    screen_putchar(c, pc_row, pc_col, attr);
}

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
/*
 * Set the insert start position for when using a prompt buffer.
 */
    void
set_insstart(linenr_T lnum, int col)
{
    Insstart.lnum = lnum;
    Insstart.col = col;
    Insstart_orig = Insstart;
    Insstart_textlen = Insstart.col;
    Insstart_blank_vcol = MAXCOL;
    arrow_used = FALSE;
}
#endif

/*
 * Undo the previous edit_putchar().
 */
    void
edit_unputchar(void)
{
    if (pc_status != PC_STATUS_UNSET && pc_row >= msg_scrolled)
    {
	if (pc_status == PC_STATUS_RIGHT)
	    ++curwin->w_wcol;
	if (pc_status == PC_STATUS_RIGHT || pc_status == PC_STATUS_LEFT)
	    redrawWinline(curwin, curwin->w_cursor.lnum);
	else
	    screen_puts(pc_bytes, pc_row - msg_scrolled, pc_col, pc_attr);
    }
}

/*
 * Called when "$" is in 'cpoptions': display a '$' at the end of the changed
 * text.  Only works when cursor is in the line that changes.
 */
    void
display_dollar(colnr_T col_arg)
{
    colnr_T col = col_arg < 0 ? 0 : col_arg;
    colnr_T save_col;

    if (!redrawing())
	return;

    cursor_off();
    save_col = curwin->w_cursor.col;
    curwin->w_cursor.col = col;
    if (has_mbyte)
    {
	char_u *p;

	// If on the last byte of a multi-byte move to the first byte.
	p = ml_get_curline();
	curwin->w_cursor.col -= (*mb_head_off)(p, p + col);
    }
    curs_columns(FALSE);	    // recompute w_wrow and w_wcol
    if (curwin->w_wcol < curwin->w_width)
    {
	edit_putchar('$', FALSE);
	dollar_vcol = curwin->w_virtcol;
    }
    curwin->w_cursor.col = save_col;
}

/*
 * Call this function before moving the cursor from the normal insert position
 * in insert mode.
 */
    void
undisplay_dollar(void)
{
    if (dollar_vcol < 0)
	return;

    dollar_vcol = -1;
    redrawWinline(curwin, curwin->w_cursor.lnum);
}

/*
 * Truncate the space at the end of a line.  This is to be used only in an
 * insert mode.  It handles fixing the replace stack for MODE_REPLACE and
 * MODE_VREPLACE modes.
 */
    void
truncate_spaces(char_u *line)
{
    int	    i;

    // find start of trailing white space
    for (i = (int)STRLEN(line) - 1; i >= 0 && VIM_ISWHITE(line[i]); i--)
    {
	if (State & REPLACE_FLAG)
	    replace_join(0);	    // remove a NUL from the replace stack
    }
    line[i + 1] = NUL;
}

/*
 * Backspace the cursor until the given column.  Handles MODE_REPLACE and
 * MODE_VREPLACE modes correctly.  May also be used when not in insert mode at
 * all.  Will attempt not to go before "col" even when there is a composing
 * character.
 */
    void
backspace_until_column(int col)
{
    while ((int)curwin->w_cursor.col > col)
    {
	curwin->w_cursor.col--;
	if (State & REPLACE_FLAG)
	    replace_do_bs(col);
	else if (!del_char_after_col(col))
	    break;
    }
}

/*
 * Like del_char(), but make sure not to go before column "limit_col".
 * Only matters when there are composing characters.
 * Return TRUE when something was deleted.
 */
   static int
del_char_after_col(int limit_col UNUSED)
{
    if (enc_utf8 && limit_col >= 0)
    {
	colnr_T ecol = curwin->w_cursor.col + 1;

	// Make sure the cursor is at the start of a character, but
	// skip forward again when going too far back because of a
	// composing character.
	mb_adjust_cursor();
	while (curwin->w_cursor.col < (colnr_T)limit_col)
	{
	    int l = utf_ptr2len(ml_get_cursor());

	    if (l == 0)  // end of line
		break;
	    curwin->w_cursor.col += l;
	}
	if (*ml_get_cursor() == NUL || curwin->w_cursor.col == ecol)
	    return FALSE;
	del_bytes((long)((int)ecol - curwin->w_cursor.col), FALSE, TRUE);
    }
    else
	(void)del_char(FALSE);
    return TRUE;
}

/*
 * Next character is interpreted literally.
 * A one, two or three digit decimal number is interpreted as its byte value.
 * If one or two digits are entered, the next character is given to vungetc().
 * For Unicode a character > 255 may be returned.
 * If "noReduceKeys" is TRUE do not change any modifyOtherKeys ESC sequence
 * into a normal key, return ESC.
 */
    int
get_literal(int noReduceKeys)
{
    int		cc;
    int		nc;
    int		i;
    int		hex = FALSE;
    int		octal = FALSE;
    int		unicode = 0;

    if (got_int)
	return Ctrl_C;

#ifdef FEAT_GUI
    /*
     * In GUI there is no point inserting the internal code for a special key.
     * It is more useful to insert the string "<KEY>" instead.	This would
     * probably be useful in a text window too, but it would not be
     * vi-compatible (maybe there should be an option for it?) -- webb
     */
    if (gui.in_use)
    {
	++allow_keys;
	if (noReduceKeys)
	    ++no_reduce_keys;
    }
#endif
#ifdef USE_ON_FLY_SCROLL
    dont_scroll = TRUE;		// disallow scrolling here
#endif
    ++no_mapping;		// don't map the next key hits
    cc = 0;
    i = 0;
    for (;;)
    {
	nc = plain_vgetc();
	if ((nc == ESC || nc == CSI) && !noReduceKeys)
	    nc = decodeModifyOtherKeys(nc);

	if ((mod_mask & ~MOD_MASK_SHIFT) != 0)
	    // A character with non-Shift modifiers should not be a valid
	    // character for i_CTRL-V_digit.
	    break;

	if ((State & MODE_CMDLINE) == 0 && MB_BYTE2LEN_CHECK(nc) == 1)
	    add_to_showcmd(nc);
	if (nc == 'x' || nc == 'X')
	    hex = TRUE;
	else if (nc == 'o' || nc == 'O')
	    octal = TRUE;
	else if (nc == 'u' || nc == 'U')
	    unicode = nc;
	else
	{
	    if (hex || unicode != 0)
	    {
		if (!vim_isxdigit(nc))
		    break;
		cc = cc * 16 + hex2nr(nc);
	    }
	    else if (octal)
	    {
		if (nc < '0' || nc > '7')
		    break;
		cc = cc * 8 + nc - '0';
	    }
	    else
	    {
		if (!VIM_ISDIGIT(nc))
		    break;
		cc = cc * 10 + nc - '0';
	    }

	    ++i;
	}

	if (cc > 255 && unicode == 0)
	    cc = 255;		// limit range to 0-255
	nc = 0;

	if (hex)		// hex: up to two chars
	{
	    if (i >= 2)
		break;
	}
	else if (unicode)	// Unicode: up to four or eight chars
	{
	    if ((unicode == 'u' && i >= 4) || (unicode == 'U' && i >= 8))
		break;
	}
	else if (i >= 3)	// decimal or octal: up to three chars
	    break;
    }
    if (i == 0)	    // no number entered
    {
	if (nc == K_ZERO)   // NUL is stored as NL
	{
	    cc = '\n';
	    nc = 0;
	}
	else
	{
	    cc = nc;
	    nc = 0;
	}
    }

    if (cc == 0)	// NUL is stored as NL
	cc = '\n';
    if (enc_dbcs && (cc & 0xff) == 0)
	cc = '?';	// don't accept an illegal DBCS char, the NUL in the
			// second byte will cause trouble!

    --no_mapping;
#ifdef FEAT_GUI
    if (gui.in_use)
    {
	--allow_keys;
	if (noReduceKeys)
	    --no_reduce_keys;
    }
#endif
    if (nc)
    {
	vungetc(nc);
	// A character typed with i_CTRL-V_digit cannot have modifiers.
	mod_mask = 0;
    }
    got_int = FALSE;	    // CTRL-C typed after CTRL-V is not an interrupt
    return cc;
}

/*
 * Insert character, taking care of special keys and mod_mask
 */
    static void
insert_special(
    int	    c,
    int	    allow_modmask,
    int	    ctrlv)	    // c was typed after CTRL-V
{
    char_u  *p;
    int	    len;

    /*
     * Special function key, translate into "<Key>". Up to the last '>' is
     * inserted with ins_str(), so as not to replace characters in replace
     * mode.
     * Only use mod_mask for special keys, to avoid things like <S-Space>,
     * unless 'allow_modmask' is TRUE.
     */
#if defined(MACOS_X) || defined(FEAT_GUI_GTK)
    // Command-key never produces a normal key
    if (mod_mask & MOD_MASK_CMD)
	allow_modmask = TRUE;
#endif
    if (IS_SPECIAL(c) || (mod_mask && allow_modmask))
    {
	p = get_special_key_name(c, mod_mask);
	len = (int)STRLEN(p);
	c = p[len - 1];
	if (len > 2)
	{
	    if (stop_arrow() == FAIL)
		return;
	    p[len - 1] = NUL;
	    ins_str(p);
	    AppendToRedobuffLit(p, -1);
	    ctrlv = FALSE;
	}
    }
    if (stop_arrow() == OK)
	insertchar(c, ctrlv ? INSCHAR_CTRLV : 0, -1);
}

/*
 * Special characters in this context are those that need processing other
 * than the simple insertion that can be performed here. This includes ESC
 * which terminates the insert, and CR/NL which need special processing to
 * open up a new line. This routine tries to optimize insertions performed by
 * the "redo", "undo" or "put" commands, so it needs to know when it should
 * stop and defer processing to the "normal" mechanism.
 * '0' and '^' are special, because they can be followed by CTRL-D.
 */
#define ISSPECIAL(c)	((c) < ' ' || (c) >= DEL || (c) == '0' || (c) == '^')

/*
 * "flags": INSCHAR_FORMAT - force formatting
 *	    INSCHAR_CTRLV  - char typed just after CTRL-V
 *	    INSCHAR_NO_FEX - don't use 'formatexpr'
 *
 *   NOTE: passes the flags value straight through to internal_format() which,
 *	   beside INSCHAR_FORMAT (above), is also looking for these:
 *	    INSCHAR_DO_COM   - format comments
 *	    INSCHAR_COM_LIST - format comments with num list or 2nd line indent
 */
    void
insertchar(
    int		c,			// character to insert or NUL
    int		flags,			// INSCHAR_FORMAT, etc.
    int		second_indent)		// indent for second line if >= 0
{
    int		textwidth;
    char_u	*p;
    int		fo_ins_blank;
    int		force_format = flags & INSCHAR_FORMAT;

    textwidth = comp_textwidth(force_format);
    fo_ins_blank = has_format_option(FO_INS_BLANK);

    /*
     * Try to break the line in two or more pieces when:
     * - Always do this if we have been called to do formatting only.
     * - Always do this when 'formatoptions' has the 'a' flag and the line
     *   ends in white space.
     * - Otherwise:
     *	 - Don't do this if inserting a blank
     *	 - Don't do this if an existing character is being replaced, unless
     *	   we're in MODE_VREPLACE state.
     *	 - Do this if the cursor is not on the line where insert started
     *	 or - 'formatoptions' doesn't have 'l' or the line was not too long
     *	       before the insert.
     *	    - 'formatoptions' doesn't have 'b' or a blank was inserted at or
     *	      before 'textwidth'
     */
    if (textwidth > 0
	    && (force_format
		|| (!VIM_ISWHITE(c)
		    && !((State & REPLACE_FLAG)
			&& !(State & VREPLACE_FLAG)
			&& *ml_get_cursor() != NUL)
		    && (curwin->w_cursor.lnum != Insstart.lnum
			|| ((!has_format_option(FO_INS_LONG)
				|| Insstart_textlen <= (colnr_T)textwidth)
			    && (!fo_ins_blank
				|| Insstart_blank_vcol <= (colnr_T)textwidth
			    ))))))
    {
	// Format with 'formatexpr' when it's set.  Use internal formatting
	// when 'formatexpr' isn't set or it returns non-zero.
#if defined(FEAT_EVAL)
	int     do_internal = TRUE;
	colnr_T virtcol = get_nolist_virtcol()
				  + char2cells(c != NUL ? c : gchar_cursor());

	if (*curbuf->b_p_fex != NUL && (flags & INSCHAR_NO_FEX) == 0
		&& (force_format || virtcol > (colnr_T)textwidth))
	{
	    do_internal = (fex_format(curwin->w_cursor.lnum, 1L, c) != 0);
	    // It may be required to save for undo again, e.g. when setline()
	    // was called.
	    ins_need_undo = TRUE;
	}
	if (do_internal)
#endif
	    internal_format(textwidth, second_indent, flags, c == NUL, c);
    }

    if (c == NUL)	    // only formatting was wanted
	return;

    // Check whether this character should end a comment.
    if (did_ai && c == end_comment_pending)
    {
	char_u  *line;
	char_u	lead_end[COM_MAX_LEN];	    // end-comment string
	int	middle_len, end_len;
	int	i;

	/*
	 * Need to remove existing (middle) comment leader and insert end
	 * comment leader.  First, check what comment leader we can find.
	 */
	i = get_leader_len(line = ml_get_curline(), &p, FALSE, TRUE);
	if (i > 0 && vim_strchr(p, COM_MIDDLE) != NULL)	// Just checking
	{
	    // Skip middle-comment string
	    while (*p && p[-1] != ':')	// find end of middle flags
		++p;
	    middle_len = copy_option_part(&p, lead_end, COM_MAX_LEN, ",");
	    // Don't count trailing white space for middle_len
	    while (middle_len > 0 && VIM_ISWHITE(lead_end[middle_len - 1]))
		--middle_len;

	    // Find the end-comment string
	    while (*p && p[-1] != ':')	// find end of end flags
		++p;
	    end_len = copy_option_part(&p, lead_end, COM_MAX_LEN, ",");

	    // Skip white space before the cursor
	    i = curwin->w_cursor.col;
	    while (--i >= 0 && VIM_ISWHITE(line[i]))
		;
	    i++;

	    // Skip to before the middle leader
	    i -= middle_len;

	    // Check some expected things before we go on
	    if (i >= 0 && lead_end[end_len - 1] == end_comment_pending)
	    {
		// Backspace over all the stuff we want to replace
		backspace_until_column(i);

		// Insert the end-comment string, except for the last
		// character, which will get inserted as normal later.
		ins_bytes_len(lead_end, end_len - 1);
	    }
	}
    }
    end_comment_pending = NUL;

    did_ai = FALSE;
    did_si = FALSE;
    can_si = FALSE;
    can_si_back = FALSE;

    /*
     * If there's any pending input, grab up to INPUT_BUFLEN at once.
     * This speeds up normal text input considerably.
     * Don't do this when 'cindent' or 'indentexpr' is set, because we might
     * need to re-indent at a ':', or any other character (but not what
     * 'paste' is set)..
     * Don't do this when there an InsertCharPre autocommand is defined,
     * because we need to fire the event for every character.
     * Do the check for InsertCharPre before the call to vpeekc() because the
     * InsertCharPre autocommand could change the input buffer.
     */
#ifdef USE_ON_FLY_SCROLL
    dont_scroll = FALSE;		// allow scrolling here
#endif

    if (       !ISSPECIAL(c)
	    && (!has_mbyte || (*mb_char2len)(c) == 1)
	    && !has_insertcharpre()
	    && vpeekc() != NUL
	    && !(State & REPLACE_FLAG)
	    && !cindent_on()
#ifdef FEAT_RIGHTLEFT
	    && !p_ri
#endif
	   )
    {
#define INPUT_BUFLEN 100
	char_u		buf[INPUT_BUFLEN + 1];
	int		i;
	colnr_T		virtcol = 0;

	buf[0] = c;
	i = 1;
	if (textwidth > 0)
	    virtcol = get_nolist_virtcol();
	/*
	 * Stop the string when:
	 * - no more chars available
	 * - finding a special character (command key)
	 * - buffer is full
	 * - running into the 'textwidth' boundary
	 * - need to check for abbreviation: A non-word char after a word-char
	 */
	while (	   (c = vpeekc()) != NUL
		&& !ISSPECIAL(c)
		&& (!has_mbyte || MB_BYTE2LEN_CHECK(c) == 1)
		&& i < INPUT_BUFLEN
		&& (textwidth == 0
		    || (virtcol += byte2cells(buf[i - 1])) < (colnr_T)textwidth)
		&& !(!no_abbr && !vim_iswordc(c) && vim_iswordc(buf[i - 1])))
	{
#ifdef FEAT_RIGHTLEFT
	    c = vgetc();
	    if (p_hkmap && KeyTyped)
		c = hkmap(c);		    // Hebrew mode mapping
	    buf[i++] = c;
#else
	    buf[i++] = vgetc();
#endif
	}

#ifdef FEAT_DIGRAPHS
	do_digraph(-1);			// clear digraphs
	do_digraph(buf[i-1]);		// may be the start of a digraph
#endif
	buf[i] = NUL;
	ins_str(buf);
	if (flags & INSCHAR_CTRLV)
	{
	    redo_literal(*buf);
	    i = 1;
	}
	else
	    i = 0;
	if (buf[i] != NUL)
	    AppendToRedobuffLit(buf + i, -1);
    }
    else
    {
	int		cc;

	if (has_mbyte && (cc = (*mb_char2len)(c)) > 1)
	{
	    char_u	buf[MB_MAXBYTES + 1];

	    (*mb_char2bytes)(c, buf);
	    buf[cc] = NUL;
	    ins_char_bytes(buf, cc);
	    AppendCharToRedobuff(c);
	}
	else
	{
	    ins_char(c);
	    if (flags & INSCHAR_CTRLV)
		redo_literal(c);
	    else
		AppendCharToRedobuff(c);
	}
    }
}

/*
 * Put a character in the redo buffer, for when just after a CTRL-V.
 */
    static void
redo_literal(int c)
{
    char_u	buf[10];

    // Only digits need special treatment.  Translate them into a string of
    // three digits.
    if (VIM_ISDIGIT(c))
    {
	vim_snprintf((char *)buf, sizeof(buf), "%03d", c);
	AppendToRedobuff(buf);
    }
    else
	AppendCharToRedobuff(c);
}

/*
 * start_arrow() is called when an arrow key is used in insert mode.
 * For undo/redo it resembles hitting the <ESC> key.
 */
    void
start_arrow(
    pos_T    *end_insert_pos)		// can be NULL
{
    start_arrow_common(end_insert_pos, TRUE);
}

/*
 * Like start_arrow() but with end_change argument.
 * Will prepare for redo of CTRL-G U if "end_change" is FALSE.
 */
    static void
start_arrow_with_change(
    pos_T    *end_insert_pos,		// can be NULL
    int	      end_change)		// end undoable change
{
    start_arrow_common(end_insert_pos, end_change);
    if (!end_change)
    {
	AppendCharToRedobuff(Ctrl_G);
	AppendCharToRedobuff('U');
    }
}

    static void
start_arrow_common(
    pos_T    *end_insert_pos,		// can be NULL
    int	      end_change)		// end undoable change
{
    if (!arrow_used && end_change)	// something has been inserted
    {
	AppendToRedobuff(ESC_STR);
	stop_insert(end_insert_pos, FALSE, FALSE);
	arrow_used = TRUE;	// this means we stopped the current insert
    }
#ifdef FEAT_SPELL
    check_spell_redraw();
#endif
}

#ifdef FEAT_SPELL
/*
 * If we skipped highlighting word at cursor, do it now.
 * It may be skipped again, thus reset spell_redraw_lnum first.
 */
    static void
check_spell_redraw(void)
{
    if (spell_redraw_lnum != 0)
    {
	linenr_T	lnum = spell_redraw_lnum;

	spell_redraw_lnum = 0;
	redrawWinline(curwin, lnum);
    }
}

#endif

/*
 * stop_arrow() is called before a change is made in insert mode.
 * If an arrow key has been used, start a new insertion.
 * Returns FAIL if undo is impossible, shouldn't insert then.
 */
    int
stop_arrow(void)
{
    if (arrow_used)
    {
	Insstart = curwin->w_cursor;	// new insertion starts here
	if (Insstart.col > Insstart_orig.col && !ins_need_undo)
	    // Don't update the original insert position when moved to the
	    // right, except when nothing was inserted yet.
	    update_Insstart_orig = FALSE;
	Insstart_textlen = (colnr_T)linetabsize_str(ml_get_curline());

	if (u_save_cursor() == OK)
	{
	    arrow_used = FALSE;
	    ins_need_undo = FALSE;
	}

	ai_col = 0;
	if (State & VREPLACE_FLAG)
	{
	    orig_line_count = curbuf->b_ml.ml_line_count;
	    vr_lines_changed = 1;
	}
	ResetRedobuff();
	AppendToRedobuff((char_u *)"1i");   // pretend we start an insertion
	new_insert_skip = 2;
    }
    else if (ins_need_undo)
    {
	if (u_save_cursor() == OK)
	    ins_need_undo = FALSE;
    }

#ifdef FEAT_FOLDING
    // Always open fold at the cursor line when inserting something.
    foldOpenCursor();
#endif

    return (arrow_used || ins_need_undo ? FAIL : OK);
}

/*
 * Do a few things to stop inserting.
 * "end_insert_pos" is where insert ended.  It is NULL when we already jumped
 * to another window/buffer.
 */
    static void
stop_insert(
    pos_T	*end_insert_pos,
    int		esc,			// called by ins_esc()
    int		nomove)			// <c-\><c-o>, don't move cursor
{
    int		cc;
    char_u	*ptr;

    stop_redo_ins();
    replace_flush();		// abandon replace stack

    /*
     * Save the inserted text for later redo with ^@ and CTRL-A.
     * Don't do it when "restart_edit" was set and nothing was inserted,
     * otherwise CTRL-O w and then <Left> will clear "last_insert".
     */
    ptr = get_inserted();
    int added = ptr == NULL ? 0 : (int)STRLEN(ptr) - new_insert_skip;
    if (did_restart_edit == 0 || added > 0)
    {
	vim_free(last_insert);
	last_insert = ptr;
	last_insert_skip = added < 0 ? 0 : new_insert_skip;
    }
    else
	vim_free(ptr);

    if (!arrow_used && end_insert_pos != NULL)
    {
	// Auto-format now.  It may seem strange to do this when stopping an
	// insertion (or moving the cursor), but it's required when appending
	// a line and having it end in a space.  But only do it when something
	// was actually inserted, otherwise undo won't work.
	if (!ins_need_undo && has_format_option(FO_AUTO))
	{
	    pos_T   tpos = curwin->w_cursor;

	    // When the cursor is at the end of the line after a space the
	    // formatting will move it to the following word.  Avoid that by
	    // moving the cursor onto the space.
	    cc = 'x';
	    if (curwin->w_cursor.col > 0 && gchar_cursor() == NUL)
	    {
		dec_cursor();
		cc = gchar_cursor();
		if (!VIM_ISWHITE(cc))
		    curwin->w_cursor = tpos;
	    }

	    auto_format(TRUE, FALSE);

	    if (VIM_ISWHITE(cc))
	    {
		if (gchar_cursor() != NUL)
		    inc_cursor();
		// If the cursor is still at the same character, also keep
		// the "coladd".
		if (gchar_cursor() == NUL
			&& curwin->w_cursor.lnum == tpos.lnum
			&& curwin->w_cursor.col == tpos.col)
		    curwin->w_cursor.coladd = tpos.coladd;
	    }
	}

	// If a space was inserted for auto-formatting, remove it now.
	check_auto_format(TRUE);

	// If we just did an auto-indent, remove the white space from the end
	// of the line, and put the cursor back.
	// Do this when ESC was used or moving the cursor up/down.
	// Check for the old position still being valid, just in case the text
	// got changed unexpectedly.
	if (!nomove && did_ai && (esc || (vim_strchr(p_cpo, CPO_INDENT) == NULL
			&& curwin->w_cursor.lnum != end_insert_pos->lnum))
		&& end_insert_pos->lnum <= curbuf->b_ml.ml_line_count)
	{
	    pos_T	tpos = curwin->w_cursor;

	    curwin->w_cursor = *end_insert_pos;
	    check_cursor_col();  // make sure it is not past the line
	    for (;;)
	    {
		if (gchar_cursor() == NUL && curwin->w_cursor.col > 0)
		    --curwin->w_cursor.col;
		cc = gchar_cursor();
		if (!VIM_ISWHITE(cc))
		    break;
		if (del_char(TRUE) == FAIL)
		    break;  // should not happen
	    }
	    if (curwin->w_cursor.lnum != tpos.lnum)
		curwin->w_cursor = tpos;
	    else
	    {
		// reset tpos, could have been invalidated in the loop above
		tpos = curwin->w_cursor;
		tpos.col++;
		if (cc != NUL && gchar_pos(&tpos) == NUL)
		    ++curwin->w_cursor.col;	// put cursor back on the NUL
	    }

	    // <C-S-Right> may have started Visual mode, adjust the position for
	    // deleted characters.
	    if (VIsual_active)
		check_visual_pos();
	}
    }
    did_ai = FALSE;
    did_si = FALSE;
    can_si = FALSE;
    can_si_back = FALSE;

    // Set '[ and '] to the inserted text.  When end_insert_pos is NULL we are
    // now in a different buffer.
    if (end_insert_pos != NULL)
    {
	curbuf->b_op_start = Insstart;
	curbuf->b_op_start_orig = Insstart_orig;
	curbuf->b_op_end = *end_insert_pos;
    }
}

/*
 * Set the last inserted text to a single character.
 * Used for the replace command.
 */
    void
set_last_insert(int c)
{
    char_u	*s;

    vim_free(last_insert);
    last_insert = alloc(MB_MAXBYTES * 3 + 5);
    if (last_insert == NULL)
	return;

    s = last_insert;
    // Use the CTRL-V only when entering a special char
    if (c < ' ' || c == DEL)
	*s++ = Ctrl_V;
    s = add_char2buf(c, s);
    *s++ = ESC;
    *s++ = NUL;
    last_insert_skip = 0;
}

#if defined(EXITFREE) || defined(PROTO)
    void
free_last_insert(void)
{
    VIM_CLEAR(last_insert);
}
#endif

/*
 * Add character "c" to buffer "s".  Escape the special meaning of K_SPECIAL
 * and CSI.  Handle multi-byte characters.
 * Returns a pointer to after the added bytes.
 */
    char_u *
add_char2buf(int c, char_u *s)
{
    char_u	temp[MB_MAXBYTES + 1];
    int		i;
    int		len;

    len = (*mb_char2bytes)(c, temp);
    for (i = 0; i < len; ++i)
    {
	c = temp[i];
	// Need to escape K_SPECIAL and CSI like in the typeahead buffer.
	if (c == K_SPECIAL)
	{
	    *s++ = K_SPECIAL;
	    *s++ = KS_SPECIAL;
	    *s++ = KE_FILLER;
	}
#ifdef FEAT_GUI
	else if (c == CSI)
	{
	    *s++ = CSI;
	    *s++ = KS_EXTRA;
	    *s++ = (int)KE_CSI;
	}
#endif
	else
	    *s++ = c;
    }
    return s;
}

/*
 * move cursor to start of line
 * if flags & BL_WHITE	move to first non-white
 * if flags & BL_SOL	move to first non-white if startofline is set,
 *			    otherwise keep "curswant" column
 * if flags & BL_FIX	don't leave the cursor on a NUL.
 */
    void
beginline(int flags)
{
    if ((flags & BL_SOL) && !p_sol)
	coladvance(curwin->w_curswant);
    else
    {
	curwin->w_cursor.col = 0;
	curwin->w_cursor.coladd = 0;

	if (flags & (BL_WHITE | BL_SOL))
	{
	    char_u  *ptr;

	    for (ptr = ml_get_curline(); VIM_ISWHITE(*ptr)
			       && !((flags & BL_FIX) && ptr[1] == NUL); ++ptr)
		++curwin->w_cursor.col;
	}
	curwin->w_set_curswant = TRUE;
    }
    adjust_skipcol();
}

/*
 * oneright oneleft cursor_down cursor_up
 *
 * Move one char {right,left,down,up}.
 * Doesn't move onto the NUL past the end of the line, unless it is allowed.
 * Return OK when successful, FAIL when we hit a line of file boundary.
 */

    int
oneright(void)
{
    char_u	*ptr;
    int		l;

    if (virtual_active())
    {
	pos_T	prevpos = curwin->w_cursor;

	// Adjust for multi-wide char (excluding TAB)
	ptr = ml_get_cursor();
	coladvance(getviscol() + ((*ptr != TAB
					  && vim_isprintc((*mb_ptr2char)(ptr)))
		    ? ptr2cells(ptr) : 1));
	curwin->w_set_curswant = TRUE;
	// Return OK if the cursor moved, FAIL otherwise (at window edge).
	return (prevpos.col != curwin->w_cursor.col
		    || prevpos.coladd != curwin->w_cursor.coladd) ? OK : FAIL;
    }

    ptr = ml_get_cursor();
    if (*ptr == NUL)
	return FAIL;	    // already at the very end

    if (has_mbyte)
	l = (*mb_ptr2len)(ptr);
    else
	l = 1;

    // move "l" bytes right, but don't end up on the NUL, unless 'virtualedit'
    // contains "onemore".
    if (ptr[l] == NUL && (get_ve_flags() & VE_ONEMORE) == 0)
	return FAIL;
    curwin->w_cursor.col += l;

    curwin->w_set_curswant = TRUE;
    adjust_skipcol();
    return OK;
}

    int
oneleft(void)
{
    if (virtual_active())
    {
#ifdef FEAT_LINEBREAK
	int width;
#endif
	int v = getviscol();

	if (v == 0)
	    return FAIL;

#ifdef FEAT_LINEBREAK
	// We might get stuck on 'showbreak', skip over it.
	width = 1;
	for (;;)
	{
	    coladvance(v - width);
	    // getviscol() is slow, skip it when 'showbreak' is empty,
	    // 'breakindent' is not set and there are no multi-byte
	    // characters
	    if ((*get_showbreak_value(curwin) == NUL && !curwin->w_p_bri
					     && !has_mbyte) || getviscol() < v)
		break;
	    ++width;
	}
#else
	coladvance(v - 1);
#endif

	if (curwin->w_cursor.coladd == 1)
	{
	    char_u *ptr;

	    // Adjust for multi-wide char (not a TAB)
	    ptr = ml_get_cursor();
	    if (*ptr != TAB && vim_isprintc((*mb_ptr2char)(ptr))
							 && ptr2cells(ptr) > 1)
		curwin->w_cursor.coladd = 0;
	}

	curwin->w_set_curswant = TRUE;
	adjust_skipcol();
	return OK;
    }

    if (curwin->w_cursor.col == 0)
	return FAIL;

    curwin->w_set_curswant = TRUE;
    --curwin->w_cursor.col;

    // if the character on the left of the current cursor is a multi-byte
    // character, move to its first byte
    if (has_mbyte)
	mb_adjust_cursor();
    adjust_skipcol();
    return OK;
}

/*
 * Move the cursor up "n" lines in window "wp".
 * Takes care of closed folds.
 */
    void
cursor_up_inner(win_T *wp, long n)
{
    linenr_T	lnum = wp->w_cursor.lnum;

    if (n >= lnum)
	lnum = 1;
    else
#ifdef FEAT_FOLDING
	if (hasAnyFolding(wp))
    {
	/*
	 * Count each sequence of folded lines as one logical line.
	 */
	// go to the start of the current fold
	(void)hasFoldingWin(wp, lnum, &lnum, NULL, TRUE, NULL);

	while (n--)
	{
	    // move up one line
	    --lnum;
	    if (lnum <= 1)
		break;
	    // If we entered a fold, move to the beginning, unless in
	    // Insert mode or when 'foldopen' contains "all": it will open
	    // in a moment.
	    if (n > 0 || !((State & MODE_INSERT) || (fdo_flags & FDO_ALL)))
		(void)hasFoldingWin(wp, lnum, &lnum, NULL, TRUE, NULL);
	}
	if (lnum < 1)
	    lnum = 1;
    }
    else
#endif
	lnum -= n;

    wp->w_cursor.lnum = lnum;
}

    int
cursor_up(
    long	n,
    int		upd_topline)	    // When TRUE: update topline
{
    // This fails if the cursor is already in the first line or the count is
    // larger than the line number and '-' is in 'cpoptions'
    linenr_T lnum = curwin->w_cursor.lnum;
    if (n > 0 && (lnum <= 1
		       || (n >= lnum && vim_strchr(p_cpo, CPO_MINUS) != NULL)))
	return FAIL;
    cursor_up_inner(curwin, n);

    // try to advance to the column we want to be at
    coladvance(curwin->w_curswant);

    if (upd_topline)
	update_topline();	// make sure curwin->w_topline is valid

    return OK;
}

/*
 * Move the cursor down "n" lines in window "wp".
 * Takes care of closed folds.
 */
    void
cursor_down_inner(win_T *wp, long n)
{
    linenr_T	lnum = wp->w_cursor.lnum;
    linenr_T	line_count = wp->w_buffer->b_ml.ml_line_count;

    if (lnum + n >= line_count)
	lnum = line_count;
    else
#ifdef FEAT_FOLDING
	if (hasAnyFolding(wp))
    {
	linenr_T	last;

	// count each sequence of folded lines as one logical line
	while (n--)
	{
	    // Move to last line of fold, will fail if it's the end-of-file.
	    if (hasFoldingWin(wp, lnum, NULL, &last, TRUE, NULL))
		lnum = last + 1;
	    else
		++lnum;
	    if (lnum >= line_count)
		break;
	}
	if (lnum > line_count)
	    lnum = line_count;
    }
    else
#endif
	lnum += n;

    wp->w_cursor.lnum = lnum;
}

/*
 * Cursor down a number of logical lines.
 */
    int
cursor_down(
    long	n,
    int		upd_topline)	    // When TRUE: update topline
{
    linenr_T	lnum = curwin->w_cursor.lnum;
    linenr_T	line_count = curwin->w_buffer->b_ml.ml_line_count;
    // This fails if the cursor is already in the last line or would move
    // beyond the last line and '-' is in 'cpoptions'
    if (n > 0
	    && (lnum >= line_count
		|| (lnum + n > line_count
				     && vim_strchr(p_cpo, CPO_MINUS) != NULL)))
	return FAIL;
    cursor_down_inner(curwin, n);

    // try to advance to the column we want to be at
    coladvance(curwin->w_curswant);

    if (upd_topline)
	update_topline();	// make sure curwin->w_topline is valid

    return OK;
}

/*
 * Stuff the last inserted text in the read buffer.
 * Last_insert actually is a copy of the redo buffer, so we
 * first have to remove the command.
 */
    int
stuff_inserted(
    int	    c,		// Command character to be inserted
    long    count,	// Repeat this many times
    int	    no_esc)	// Don't add an ESC at the end
{
    char_u	*esc_ptr;
    char_u	*ptr;
    char_u	*last_ptr;
    char_u	last = NUL;

    ptr = get_last_insert();
    if (ptr == NULL)
    {
	emsg(_(e_no_inserted_text_yet));
	return FAIL;
    }

    // may want to stuff the command character, to start Insert mode
    if (c != NUL)
	stuffcharReadbuff(c);
    if ((esc_ptr = vim_strrchr(ptr, ESC)) != NULL)
	*esc_ptr = NUL;	    // remove the ESC

    // when the last char is either "0" or "^" it will be quoted if no ESC
    // comes after it OR if it will inserted more than once and "ptr"
    // starts with ^D.	-- Acevedo
    last_ptr = (esc_ptr ? esc_ptr : ptr + STRLEN(ptr)) - 1;
    if (last_ptr >= ptr && (*last_ptr == '0' || *last_ptr == '^')
	    && (no_esc || (*ptr == Ctrl_D && count > 1)))
    {
	last = *last_ptr;
	*last_ptr = NUL;
    }

    do
    {
	stuffReadbuff(ptr);
	// a trailing "0" is inserted as "<C-V>048", "^" as "<C-V>^"
	if (last)
	    stuffReadbuff(
		       (char_u *)(last == '0' ? "\026\060\064\070" : "\026^"));
    }
    while (--count > 0);

    if (last)
	*last_ptr = last;

    if (esc_ptr != NULL)
	*esc_ptr = ESC;	    // put the ESC back

    // may want to stuff a trailing ESC, to get out of Insert mode
    if (!no_esc)
	stuffcharReadbuff(ESC);

    return OK;
}

    char_u *
get_last_insert(void)
{
    if (last_insert == NULL)
	return NULL;
    return last_insert + last_insert_skip;
}

/*
 * Get last inserted string, and remove trailing <Esc>.
 * Returns pointer to allocated memory (must be freed) or NULL.
 */
    char_u *
get_last_insert_save(void)
{
    char_u	*s;
    int		len;

    if (last_insert == NULL)
	return NULL;
    s = vim_strsave(last_insert + last_insert_skip);
    if (s == NULL)
	return NULL;

    len = (int)STRLEN(s);
    if (len > 0 && s[len - 1] == ESC)	// remove trailing ESC
	s[len - 1] = NUL;
    return s;
}

/*
 * Check the word in front of the cursor for an abbreviation.
 * Called when the non-id character "c" has been entered.
 * When an abbreviation is recognized it is removed from the text and
 * the replacement string is inserted in typebuf.tb_buf[], followed by "c".
 */
    static int
echeck_abbr(int c)
{
    // Don't check for abbreviation in paste mode, when disabled and just
    // after moving around with cursor keys.
    if (p_paste || no_abbr || arrow_used)
	return FALSE;

    return check_abbr(c, ml_get_curline(), curwin->w_cursor.col,
		curwin->w_cursor.lnum == Insstart.lnum ? Insstart.col : 0);
}

/*
 * replace-stack functions
 *
 * When replacing characters, the replaced characters are remembered for each
 * new character.  This is used to re-insert the old text when backspacing.
 *
 * There is a NUL headed list of characters for each character that is
 * currently in the file after the insertion point.  When BS is used, one NUL
 * headed list is put back for the deleted character.
 *
 * For a newline, there are two NUL headed lists.  One contains the characters
 * that the NL replaced.  The extra one stores the characters after the cursor
 * that were deleted (always white space).
 *
 * Replace_offset is normally 0, in which case replace_push will add a new
 * character at the end of the stack.  If replace_offset is not 0, that many
 * characters will be left on the stack above the newly inserted character.
 */

static char_u	*replace_stack = NULL;
static long	replace_stack_nr = 0;	    // next entry in replace stack
static long	replace_stack_len = 0;	    // max. number of entries

    void
replace_push(
    int	    c)	    // character that is replaced (NUL is none)
{
    char_u  *p;

    if (replace_stack_nr < replace_offset)	// nothing to do
	return;
    if (replace_stack_len <= replace_stack_nr)
    {
	replace_stack_len += 50;
	p = ALLOC_MULT(char_u, replace_stack_len);
	if (p == NULL)	    // out of memory
	{
	    replace_stack_len -= 50;
	    return;
	}
	if (replace_stack != NULL)
	{
	    mch_memmove(p, replace_stack,
				 (size_t)(replace_stack_nr * sizeof(char_u)));
	    vim_free(replace_stack);
	}
	replace_stack = p;
    }
    p = replace_stack + replace_stack_nr - replace_offset;
    if (replace_offset)
	mch_memmove(p + 1, p, (size_t)(replace_offset * sizeof(char_u)));
    *p = c;
    ++replace_stack_nr;
}

/*
 * Push a character onto the replace stack.  Handles a multi-byte character in
 * reverse byte order, so that the first byte is popped off first.
 * Return the number of bytes done (includes composing characters).
 */
    int
replace_push_mb(char_u *p)
{
    int l = (*mb_ptr2len)(p);
    int j;

    for (j = l - 1; j >= 0; --j)
	replace_push(p[j]);
    return l;
}

/*
 * Pop one item from the replace stack.
 * return -1 if stack empty
 * return replaced character or NUL otherwise
 */
    static int
replace_pop(void)
{
    if (replace_stack_nr == 0)
	return -1;
    return (int)replace_stack[--replace_stack_nr];
}

/*
 * Join the top two items on the replace stack.  This removes to "off"'th NUL
 * encountered.
 */
    void
replace_join(
    int	    off)	// offset for which NUL to remove
{
    int	    i;

    for (i = replace_stack_nr; --i >= 0; )
	if (replace_stack[i] == NUL && off-- <= 0)
	{
	    --replace_stack_nr;
	    mch_memmove(replace_stack + i, replace_stack + i + 1,
					      (size_t)(replace_stack_nr - i));
	    return;
	}
}

/*
 * Pop bytes from the replace stack until a NUL is found, and insert them
 * before the cursor.  Can only be used in MODE_REPLACE or MODE_VREPLACE state.
 */
    static void
replace_pop_ins(void)
{
    int	    cc;
    int	    oldState = State;

    State = MODE_NORMAL;			// don't want MODE_REPLACE here
    while ((cc = replace_pop()) > 0)
    {
	mb_replace_pop_ins(cc);
	dec_cursor();
    }
    State = oldState;
}

/*
 * Insert bytes popped from the replace stack. "cc" is the first byte.  If it
 * indicates a multi-byte char, pop the other bytes too.
 */
    static void
mb_replace_pop_ins(int cc)
{
    int		n;
    char_u	buf[MB_MAXBYTES + 1];
    int		i;
    int		c;

    if (has_mbyte && (n = MB_BYTE2LEN(cc)) > 1)
    {
	buf[0] = cc;
	for (i = 1; i < n; ++i)
	    buf[i] = replace_pop();
	ins_bytes_len(buf, n);
    }
    else
	ins_char(cc);

    if (enc_utf8)
	// Handle composing chars.
	for (;;)
	{
	    c = replace_pop();
	    if (c == -1)	    // stack empty
		break;
	    if ((n = MB_BYTE2LEN(c)) == 1)
	    {
		// Not a multi-byte char, put it back.
		replace_push(c);
		break;
	    }

	    buf[0] = c;
	    for (i = 1; i < n; ++i)
		buf[i] = replace_pop();
	    if (utf_iscomposing(utf_ptr2char(buf)))
		ins_bytes_len(buf, n);
	    else
	    {
		// Not a composing char, put it back.
		for (i = n - 1; i >= 0; --i)
		    replace_push(buf[i]);
		break;
	    }

	}
}

/*
 * make the replace stack empty
 * (called when exiting replace mode)
 */
    static void
replace_flush(void)
{
    VIM_CLEAR(replace_stack);
    replace_stack_len = 0;
    replace_stack_nr = 0;
}

/*
 * Handle doing a BS for one character.
 * cc < 0: replace stack empty, just move cursor
 * cc == 0: character was inserted, delete it
 * cc > 0: character was replaced, put cc (first byte of original char) back
 * and check for more characters to be put back
 * When "limit_col" is >= 0, don't delete before this column.  Matters when
 * using composing characters, use del_char_after_col() instead of del_char().
 */
    static void
replace_do_bs(int limit_col)
{
    int		cc;
    int		orig_len = 0;
    int		ins_len;
    int		orig_vcols = 0;
    colnr_T	start_vcol;
    char_u	*p;
    int		i;
    int		vcol;

    cc = replace_pop();
    if (cc > 0)
    {
#ifdef FEAT_PROP_POPUP
	size_t	len_before = 0;  // init to shut up GCC

	if (curbuf->b_has_textprop)
	{
	    // Do not adjust text properties for individual delete and insert
	    // operations, do it afterwards on the resulting text.
	    len_before = STRLEN(ml_get_curline());
	    ++text_prop_frozen;
	}
#endif
	if (State & VREPLACE_FLAG)
	{
	    // Get the number of screen cells used by the character we are
	    // going to delete.
	    getvcol(curwin, &curwin->w_cursor, NULL, &start_vcol, NULL);
	    orig_vcols = chartabsize(ml_get_cursor(), start_vcol);
	}
	if (has_mbyte)
	{
	    (void)del_char_after_col(limit_col);
	    if (State & VREPLACE_FLAG)
		orig_len = (int)STRLEN(ml_get_cursor());
	    replace_push(cc);
	}
	else
	{
	    pchar_cursor(cc);
	    if (State & VREPLACE_FLAG)
		orig_len = (int)STRLEN(ml_get_cursor()) - 1;
	}
	replace_pop_ins();

	if (State & VREPLACE_FLAG)
	{
	    // Get the number of screen cells used by the inserted characters
	    p = ml_get_cursor();
	    ins_len = (int)STRLEN(p) - orig_len;
	    vcol = start_vcol;
	    for (i = 0; i < ins_len; ++i)
	    {
		vcol += chartabsize(p + i, vcol);
		i += (*mb_ptr2len)(p) - 1;
	    }
	    vcol -= start_vcol;

	    // Delete spaces that were inserted after the cursor to keep the
	    // text aligned.
	    curwin->w_cursor.col += ins_len;
	    while (vcol > orig_vcols && gchar_cursor() == ' ')
	    {
		del_char(FALSE);
		++orig_vcols;
	    }
	    curwin->w_cursor.col -= ins_len;
	}

	// mark the buffer as changed and prepare for displaying
	changed_bytes(curwin->w_cursor.lnum, curwin->w_cursor.col);

#ifdef FEAT_PROP_POPUP
	if (curbuf->b_has_textprop)
	{
	    size_t len_now = STRLEN(ml_get_curline());

	    --text_prop_frozen;
	    adjust_prop_columns(curwin->w_cursor.lnum, curwin->w_cursor.col,
					   (int)(len_now - len_before), 0);
	}
#endif
    }
    else if (cc == 0)
	(void)del_char_after_col(limit_col);
}

#if defined(FEAT_RIGHTLEFT) || defined(PROTO)
/*
 * Map Hebrew keyboard when in hkmap mode.
 */
    int
hkmap(int c)
{
    if (p_hkmapp)   // phonetic mapping, by Ilya Dogolazky
    {
	enum {hALEF=0, BET, GIMEL, DALET, HEI, VAV, ZAIN, HET, TET, IUD,
	    KAFsofit, hKAF, LAMED, MEMsofit, MEM, NUNsofit, NUN, SAMEH, AIN,
	    PEIsofit, PEI, ZADIsofit, ZADI, KOF, RESH, hSHIN, TAV};
	static char_u map[26] =
	    {(char_u)hALEF/*a*/, (char_u)BET  /*b*/, (char_u)hKAF    /*c*/,
	     (char_u)DALET/*d*/, (char_u)-1   /*e*/, (char_u)PEIsofit/*f*/,
	     (char_u)GIMEL/*g*/, (char_u)HEI  /*h*/, (char_u)IUD     /*i*/,
	     (char_u)HET  /*j*/, (char_u)KOF  /*k*/, (char_u)LAMED   /*l*/,
	     (char_u)MEM  /*m*/, (char_u)NUN  /*n*/, (char_u)SAMEH   /*o*/,
	     (char_u)PEI  /*p*/, (char_u)-1   /*q*/, (char_u)RESH    /*r*/,
	     (char_u)ZAIN /*s*/, (char_u)TAV  /*t*/, (char_u)TET     /*u*/,
	     (char_u)VAV  /*v*/, (char_u)hSHIN/*w*/, (char_u)-1      /*x*/,
	     (char_u)AIN  /*y*/, (char_u)ZADI /*z*/};

	if (c == 'N' || c == 'M' || c == 'P' || c == 'C' || c == 'Z')
	    return (int)(map[CharOrd(c)] - 1 + p_aleph);
							    // '-1'='sofit'
	else if (c == 'x')
	    return 'X';
	else if (c == 'q')
	    return '\''; // {geresh}={'}
	else if (c == 246)
	    return ' ';  // \"o --> ' ' for a german keyboard
	else if (c == 228)
	    return ' ';  // \"a --> ' '      -- / --
	else if (c == 252)
	    return ' ';  // \"u --> ' '      -- / --
	// NOTE: islower() does not do the right thing for us on Linux so we
	// do this the same was as 5.7 and previous, so it works correctly on
	// all systems.  Specifically, the e.g. Delete and Arrow keys are
	// munged and won't work if e.g. searching for Hebrew text.
	else if (c >= 'a' && c <= 'z')
	    return (int)(map[CharOrdLow(c)] + p_aleph);
	else
	    return c;
    }
    else
    {
	switch (c)
	{
	    case '`':	return ';';
	    case '/':	return '.';
	    case '\'':	return ',';
	    case 'q':	return '/';
	    case 'w':	return '\'';

			// Hebrew letters - set offset from 'a'
	    case ',':	c = '{'; break;
	    case '.':	c = 'v'; break;
	    case ';':	c = 't'; break;
	    default: {
			 static char str[] = "zqbcxlsjphmkwonu ydafe rig";

			 if (c < 'a' || c > 'z')
			     return c;
			 c = str[CharOrdLow(c)];
			 break;
		     }
	}

	return (int)(CharOrdLow(c) + p_aleph);
    }
}
#endif

    static void
ins_reg(void)
{
    int		need_redraw = FALSE;
    int		regname;
    int		literally = 0;
    int		vis_active = VIsual_active;

    /*
     * If we are going to wait for a character, show a '"'.
     */
    pc_status = PC_STATUS_UNSET;
    if (redrawing() && !char_avail())
    {
	// may need to redraw when no more chars available now
	ins_redraw(FALSE);

	edit_putchar('"', TRUE);
	add_to_showcmd_c(Ctrl_R);
    }

#ifdef USE_ON_FLY_SCROLL
    dont_scroll = TRUE;		// disallow scrolling here
#endif

    /*
     * Don't map the register name. This also prevents the mode message to be
     * deleted when ESC is hit.
     */
    ++no_mapping;
    ++allow_keys;
    regname = plain_vgetc();
    LANGMAP_ADJUST(regname, TRUE);
    if (regname == Ctrl_R || regname == Ctrl_O || regname == Ctrl_P)
    {
	// Get a third key for literal register insertion
	literally = regname;
	add_to_showcmd_c(literally);
	regname = plain_vgetc();
	LANGMAP_ADJUST(regname, TRUE);
    }
    --no_mapping;
    --allow_keys;

#ifdef FEAT_EVAL
    // Don't call u_sync() while typing the expression or giving an error
    // message for it. Only call it explicitly.
    ++no_u_sync;
    if (regname == '=')
    {
	pos_T	curpos = curwin->w_cursor;
# ifdef HAVE_INPUT_METHOD
	int	im_on = im_get_status();
# endif
	// Sync undo when evaluating the expression calls setline() or
	// append(), so that it can be undone separately.
	u_sync_once = 2;

	regname = get_expr_register();

	// Cursor may be moved back a column.
	curwin->w_cursor = curpos;
	check_cursor();
# ifdef HAVE_INPUT_METHOD
	// Restore the Input Method.
	if (im_on)
	    im_set_active(TRUE);
# endif
    }
    if (regname == NUL || !valid_yank_reg(regname, FALSE))
    {
	vim_beep(BO_REG);
	need_redraw = TRUE;	// remove the '"'
    }
    else
    {
#endif
	if (literally == Ctrl_O || literally == Ctrl_P)
	{
	    // Append the command to the redo buffer.
	    AppendCharToRedobuff(Ctrl_R);
	    AppendCharToRedobuff(literally);
	    AppendCharToRedobuff(regname);

	    do_put(regname, NULL, BACKWARD, 1L,
		 (literally == Ctrl_P ? PUT_FIXINDENT : 0) | PUT_CURSEND);
	}
	else if (insert_reg(regname, literally) == FAIL)
	{
	    vim_beep(BO_REG);
	    need_redraw = TRUE;	// remove the '"'
	}
	else if (stop_insert_mode)
	    // When the '=' register was used and a function was invoked that
	    // did ":stopinsert" then stuff_empty() returns FALSE but we won't
	    // insert anything, need to remove the '"'
	    need_redraw = TRUE;

#ifdef FEAT_EVAL
    }
    --no_u_sync;
    if (u_sync_once == 1)
	ins_need_undo = TRUE;
    u_sync_once = 0;
#endif
    clear_showcmd();

    // If the inserted register is empty, we need to remove the '"'
    if (need_redraw || stuff_empty())
	edit_unputchar();

    // Disallow starting Visual mode here, would get a weird mode.
    if (!vis_active && VIsual_active)
	end_visual_mode();
}

/*
 * CTRL-G commands in Insert mode.
 */
    static void
ins_ctrl_g(void)
{
    int		c;

    // Right after CTRL-X the cursor will be after the ruler.
    setcursor();

    /*
     * Don't map the second key. This also prevents the mode message to be
     * deleted when ESC is hit.
     */
    ++no_mapping;
    ++allow_keys;
    c = plain_vgetc();
    --no_mapping;
    --allow_keys;
    switch (c)
    {
	// CTRL-G k and CTRL-G <Up>: cursor up to Insstart.col
	case K_UP:
	case Ctrl_K:
	case 'k': ins_up(TRUE);
		  break;

	// CTRL-G j and CTRL-G <Down>: cursor down to Insstart.col
	case K_DOWN:
	case Ctrl_J:
	case 'j': ins_down(TRUE);
		  break;

	// CTRL-G u: start new undoable edit
	case 'u': u_sync(TRUE);
		  ins_need_undo = TRUE;

		  // Need to reset Insstart, esp. because a BS that joins
		  // a line to the previous one must save for undo.
		  update_Insstart_orig = FALSE;
		  Insstart = curwin->w_cursor;
		  break;

	// CTRL-G U: do not break undo with the next char
	case 'U':
		  // Allow one left/right cursor movement with the next char,
		  // without breaking undo.
		  dont_sync_undo = MAYBE;
		  break;

	case ESC:
		  // Esc after CTRL-G cancels it.
		  break;

	// Unknown CTRL-G command, reserved for future expansion.
	default:  vim_beep(BO_CTRLG);
    }
}

/*
 * CTRL-^ in Insert mode.
 */
    static void
ins_ctrl_hat(void)
{
    if (map_to_exists_mode((char_u *)"", MODE_LANGMAP, FALSE))
    {
	// ":lmap" mappings exists, Toggle use of ":lmap" mappings.
	if (State & MODE_LANGMAP)
	{
	    curbuf->b_p_iminsert = B_IMODE_NONE;
	    State &= ~MODE_LANGMAP;
	}
	else
	{
	    curbuf->b_p_iminsert = B_IMODE_LMAP;
	    State |= MODE_LANGMAP;
#ifdef HAVE_INPUT_METHOD
	    im_set_active(FALSE);
#endif
	}
    }
#ifdef HAVE_INPUT_METHOD
    else
    {
	// There are no ":lmap" mappings, toggle IM
	if (im_get_status())
	{
	    curbuf->b_p_iminsert = B_IMODE_NONE;
	    im_set_active(FALSE);
	}
	else
	{
	    curbuf->b_p_iminsert = B_IMODE_IM;
	    State &= ~MODE_LANGMAP;
	    im_set_active(TRUE);
	}
    }
#endif
    set_iminsert_global();
    showmode();
#ifdef FEAT_GUI
    // may show different cursor shape or color
    if (gui.in_use)
	gui_update_cursor(TRUE, FALSE);
#endif
#if defined(FEAT_KEYMAP)
    // Show/unshow value of 'keymap' in status lines.
    status_redraw_curbuf();
#endif
}

/*
 * Handle ESC in insert mode.
 * Returns TRUE when leaving insert mode, FALSE when going to repeat the
 * insert.
 */
    static int
ins_esc(
    long	*count,
    int		cmdchar,
    int		nomove)	    // don't move cursor
{
    int		temp;
    static int	disabled_redraw = FALSE;
#ifdef FEAT_CONCEAL
    // Remember if the cursor line was concealed before changing State.
    int		cursor_line_was_concealed = curwin->w_p_cole > 0
						&& conceal_cursor_line(curwin);
#endif

#ifdef FEAT_SPELL
    check_spell_redraw();
#endif

    temp = curwin->w_cursor.col;
    if (disabled_redraw)
    {
	if (RedrawingDisabled > 0)
	    --RedrawingDisabled;
	disabled_redraw = FALSE;
    }
    if (!arrow_used)
    {
	/*
	 * Don't append the ESC for "r<CR>" and "grx".
	 * When 'insertmode' is set only CTRL-L stops Insert mode.  Needed for
	 * when "count" is non-zero.
	 */
	if (cmdchar != 'r' && cmdchar != 'v')
	    AppendToRedobuff(p_im ? (char_u *)"\014" : ESC_STR);

	/*
	 * Repeating insert may take a long time.  Check for
	 * interrupt now and then.
	 */
	if (*count > 0)
	{
	    line_breakcheck();
	    if (got_int)
		*count = 0;
	}

	if (--*count > 0)	// repeat what was typed
	{
	    // Vi repeats the insert without replacing characters.
	    if (vim_strchr(p_cpo, CPO_REPLCNT) != NULL)
		State &= ~REPLACE_FLAG;

	    (void)start_redo_ins();
	    if (cmdchar == 'r' || cmdchar == 'v')
		stuffRedoReadbuff(ESC_STR);	// no ESC in redo buffer
	    ++RedrawingDisabled;
	    disabled_redraw = TRUE;
	    return FALSE;	// repeat the insert
	}
	stop_insert(&curwin->w_cursor, TRUE, nomove);
	undisplay_dollar();
    }

    if (cmdchar != 'r' && cmdchar != 'v')
	ins_apply_autocmds(EVENT_INSERTLEAVEPRE);

    // When an autoindent was removed, curswant stays after the
    // indent
    if (restart_edit == NUL && (colnr_T)temp == curwin->w_cursor.col)
	curwin->w_set_curswant = TRUE;

    // Remember the last Insert position in the '^ mark.
    if ((cmdmod.cmod_flags & CMOD_KEEPJUMPS) == 0)
	curbuf->b_last_insert = curwin->w_cursor;

    /*
     * The cursor should end up on the last inserted character.
     * Don't do it for CTRL-O, unless past the end of the line.
     */
    if (!nomove
	    && (curwin->w_cursor.col != 0
		|| curwin->w_cursor.coladd > 0)
	    && (restart_edit == NUL
		   || (gchar_cursor() == NUL && !VIsual_active))
#ifdef FEAT_RIGHTLEFT
	    && !revins_on
#endif
				      )
    {
	if (curwin->w_cursor.coladd > 0 || get_ve_flags() == VE_ALL)
	{
	    oneleft();
	    if (restart_edit != NUL)
		++curwin->w_cursor.coladd;
	}
	else
	{
	    --curwin->w_cursor.col;
	    curwin->w_valid &= ~(VALID_WCOL|VALID_VIRTCOL);
	    // Correct cursor for multi-byte character.
	    if (has_mbyte)
		mb_adjust_cursor();
	}
    }

#ifdef HAVE_INPUT_METHOD
    // Disable IM to allow typing English directly for Normal mode commands.
    // When ":lmap" is enabled don't change 'iminsert' (IM can be enabled as
    // well).
    if (!(State & MODE_LANGMAP))
	im_save_status(&curbuf->b_p_iminsert);
    im_set_active(FALSE);
#endif

    State = MODE_NORMAL;
    may_trigger_modechanged();
    // need to position cursor again when on a TAB and when on a char with
    // virtual text.
    if (gchar_cursor() == TAB
#ifdef FEAT_PROP_POPUP
	    || curbuf->b_has_textprop
#endif
       )
	curwin->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL);

    setmouse();
#ifdef CURSOR_SHAPE
    ui_cursor_shape();		// may show different cursor shape
#endif
    if (!p_ek)
    {
	MAY_WANT_TO_LOG_THIS;

	// Re-enable bracketed paste mode.
	out_str_t_BE();

	// Re-enable modifyOtherKeys.
	out_str_t_TI();
    }
#ifdef FEAT_CONCEAL
    // Check if the cursor line needs redrawing after changing State.  If
    // 'concealcursor' is "i" it needs to be redrawn without concealing.
    conceal_check_cursor_line(cursor_line_was_concealed);
#endif

    // When recording or for CTRL-O, need to display the new mode.
    // Otherwise remove the mode message.
    if (reg_recording != 0 || restart_edit != NUL)
	showmode();
    else if (p_smd && (got_int || !skip_showmode()))
	msg("");

    return TRUE;	    // exit Insert mode
}

#ifdef FEAT_RIGHTLEFT
/*
 * Toggle language: hkmap and revins_on.
 * Move to end of reverse inserted text.
 */
    static void
ins_ctrl_(void)
{
    if (revins_on && revins_chars && revins_scol >= 0)
    {
	while (gchar_cursor() != NUL && revins_chars--)
	    ++curwin->w_cursor.col;
    }
    p_ri = !p_ri;
    revins_on = (State == MODE_INSERT && p_ri);
    if (revins_on)
    {
	revins_scol = curwin->w_cursor.col;
	revins_legal++;
	revins_chars = 0;
	undisplay_dollar();
    }
    else
	revins_scol = -1;
    p_hkmap = curwin->w_p_rl ^ p_ri;    // be consistent!
    showmode();
}
#endif

/*
 * If 'keymodel' contains "startsel", may start selection.
 * Returns TRUE when a CTRL-O and other keys stuffed.
 */
    static int
ins_start_select(int c)
{
    if (!km_startsel)
	return FALSE;
    switch (c)
    {
	case K_KHOME:
	case K_KEND:
	case K_PAGEUP:
	case K_KPAGEUP:
	case K_PAGEDOWN:
	case K_KPAGEDOWN:
# ifdef MACOS_X
	case K_LEFT:
	case K_RIGHT:
	case K_UP:
	case K_DOWN:
	case K_END:
	case K_HOME:
# endif
	    if (!(mod_mask & MOD_MASK_SHIFT))
		break;
	    // FALLTHROUGH
	case K_S_LEFT:
	case K_S_RIGHT:
	case K_S_UP:
	case K_S_DOWN:
	case K_S_END:
	case K_S_HOME:
	    // Start selection right away, the cursor can move with CTRL-O when
	    // beyond the end of the line.
	    start_selection();

	    // Execute the key in (insert) Select mode.
	    stuffcharReadbuff(Ctrl_O);
	    if (mod_mask)
	    {
		char_u	    buf[4];

		buf[0] = K_SPECIAL;
		buf[1] = KS_MODIFIER;
		buf[2] = mod_mask;
		buf[3] = NUL;
		stuffReadbuff(buf);
	    }
	    stuffcharReadbuff(c);
	    return TRUE;
    }
    return FALSE;
}

/*
 * <Insert> key in Insert mode: toggle insert/replace mode.
 */
    static void
ins_insert(int replaceState)
{
#ifdef FEAT_EVAL
    set_vim_var_string(VV_INSERTMODE,
		   (char_u *)((State & REPLACE_FLAG) ? "i"
			    : replaceState == MODE_VREPLACE ? "v" : "r"), 1);
#endif
    ins_apply_autocmds(EVENT_INSERTCHANGE);
    if (State & REPLACE_FLAG)
	State = MODE_INSERT | (State & MODE_LANGMAP);
    else
	State = replaceState | (State & MODE_LANGMAP);
    may_trigger_modechanged();
    AppendCharToRedobuff(K_INS);
    showmode();
#ifdef CURSOR_SHAPE
    ui_cursor_shape();		// may show different cursor shape
#endif
}

/*
 * Pressed CTRL-O in Insert mode.
 */
    static void
ins_ctrl_o(void)
{
    restart_VIsual_select = 0;
    if (State & VREPLACE_FLAG)
	restart_edit = 'V';
    else if (State & REPLACE_FLAG)
	restart_edit = 'R';
    else
	restart_edit = 'I';
    if (virtual_active())
	ins_at_eol = FALSE;	// cursor always keeps its column
    else
	ins_at_eol = (gchar_cursor() == NUL);
}

/*
 * If the cursor is on an indent, ^T/^D insert/delete one
 * shiftwidth.	Otherwise ^T/^D behave like a "<<" or ">>".
 * Always round the indent to 'shiftwidth', this is compatible
 * with vi.  But vi only supports ^T and ^D after an
 * autoindent, we support it everywhere.
 */
    static void
ins_shift(int c, int lastc)
{
    if (stop_arrow() == FAIL)
	return;
    AppendCharToRedobuff(c);

    /*
     * 0^D and ^^D: remove all indent.
     */
    if (c == Ctrl_D && (lastc == '0' || lastc == '^')
						  && curwin->w_cursor.col > 0)
    {
	--curwin->w_cursor.col;
	(void)del_char(FALSE);		// delete the '^' or '0'
	// In Replace mode, restore the characters that '^' or '0' replaced.
	if (State & REPLACE_FLAG)
	    replace_pop_ins();
	if (lastc == '^')
	    old_indent = get_indent();	// remember curr. indent
	change_indent(INDENT_SET, 0, TRUE, 0, TRUE);
    }
    else
	change_indent(c == Ctrl_D ? INDENT_DEC : INDENT_INC, 0, TRUE, 0, TRUE);

    if (did_ai && *skipwhite(ml_get_curline()) != NUL)
	did_ai = FALSE;
    did_si = FALSE;
    can_si = FALSE;
    can_si_back = FALSE;
    can_cindent = FALSE;	// no cindenting after ^D or ^T
}

    static void
ins_del(void)
{
    int	    temp;

    if (stop_arrow() == FAIL)
	return;
    if (gchar_cursor() == NUL)		// delete newline
    {
	temp = curwin->w_cursor.col;
	if (!can_bs(BS_EOL)		// only if "eol" included
		|| do_join(2, FALSE, TRUE, FALSE, FALSE) == FAIL)
	    vim_beep(BO_BS);
	else
	{
	    curwin->w_cursor.col = temp;
	    // Adjust orig_line_count in case more lines have been deleted than
	    // have been added. That makes sure, that open_line() later
	    // can access all buffer lines correctly
	    if (State & VREPLACE_FLAG &&
		    orig_line_count > curbuf->b_ml.ml_line_count)
		orig_line_count = curbuf->b_ml.ml_line_count;
	}
    }
    else if (del_char(FALSE) == FAIL)  // delete char under cursor
	vim_beep(BO_BS);
    did_ai = FALSE;
    did_si = FALSE;
    can_si = FALSE;
    can_si_back = FALSE;
    AppendCharToRedobuff(K_DEL);
}

/*
 * Delete one character for ins_bs().
 */
    static void
ins_bs_one(colnr_T *vcolp)
{
    dec_cursor();
    getvcol(curwin, &curwin->w_cursor, vcolp, NULL, NULL);
    if (State & REPLACE_FLAG)
    {
	// Don't delete characters before the insert point when in
	// Replace mode
	if (curwin->w_cursor.lnum != Insstart.lnum
		|| curwin->w_cursor.col >= Insstart.col)
	    replace_do_bs(-1);
    }
    else
	(void)del_char(FALSE);
}

/*
 * Handle Backspace, delete-word and delete-line in Insert mode.
 * Return TRUE when backspace was actually used.
 */
    static int
ins_bs(
    int		c,
    int		mode,
    int		*inserted_space_p)
{
    linenr_T	lnum;
    int		cc;
    int		temp = 0;	    // init for GCC
    colnr_T	save_col;
    colnr_T	mincol;
    int		did_backspace = FALSE;
    int		in_indent;
    int		oldState;
    int		cpc[MAX_MCO];	    // composing characters
    int		call_fix_indent = FALSE;

    /*
     * can't delete anything in an empty file
     * can't backup past first character in buffer
     * can't backup past starting point unless 'backspace' > 1
     * can backup to a previous line if 'backspace' == 0
     */
    if (       BUFEMPTY()
	    || (
#ifdef FEAT_RIGHTLEFT
		!revins_on &&
#endif
		((curwin->w_cursor.lnum == 1 && curwin->w_cursor.col == 0)
		    || (!can_bs(BS_START)
			&& ((arrow_used
#ifdef FEAT_JOB_CHANNEL
				&& !bt_prompt(curbuf)
#endif
			) || (curwin->w_cursor.lnum == Insstart_orig.lnum
				&& curwin->w_cursor.col <= Insstart_orig.col)))
		    || (!can_bs(BS_INDENT) && !arrow_used && ai_col > 0
					 && curwin->w_cursor.col <= ai_col)
		    || (!can_bs(BS_EOL) && curwin->w_cursor.col == 0))))
    {
	vim_beep(BO_BS);
	return FALSE;
    }

    if (stop_arrow() == FAIL)
	return FALSE;
    in_indent = inindent(0);
    if (in_indent)
	can_cindent = FALSE;
    end_comment_pending = NUL;	// After BS, don't auto-end comment
#ifdef FEAT_RIGHTLEFT
    if (revins_on)	    // put cursor after last inserted char
	inc_cursor();
#endif

    // Virtualedit:
    //	BACKSPACE_CHAR eats a virtual space
    //	BACKSPACE_WORD eats all coladd
    //	BACKSPACE_LINE eats all coladd and keeps going
    if (curwin->w_cursor.coladd > 0)
    {
	if (mode == BACKSPACE_CHAR)
	{
	    --curwin->w_cursor.coladd;
	    return TRUE;
	}
	if (mode == BACKSPACE_WORD)
	{
	    curwin->w_cursor.coladd = 0;
	    return TRUE;
	}
	curwin->w_cursor.coladd = 0;
    }

    /*
     * Delete newline!
     */
    if (curwin->w_cursor.col == 0)
    {
	lnum = Insstart.lnum;
	if (curwin->w_cursor.lnum == lnum
#ifdef FEAT_RIGHTLEFT
			|| revins_on
#endif
				    )
	{
	    if (u_save((linenr_T)(curwin->w_cursor.lnum - 2),
			       (linenr_T)(curwin->w_cursor.lnum + 1)) == FAIL)
		return FALSE;
	    --Insstart.lnum;
	    Insstart.col = (colnr_T)STRLEN(ml_get(Insstart.lnum));
	}
	/*
	 * In replace mode:
	 * cc < 0: NL was inserted, delete it
	 * cc >= 0: NL was replaced, put original characters back
	 */
	cc = -1;
	if (State & REPLACE_FLAG)
	    cc = replace_pop();	    // returns -1 if NL was inserted
	/*
	 * In replace mode, in the line we started replacing, we only move the
	 * cursor.
	 */
	if ((State & REPLACE_FLAG) && curwin->w_cursor.lnum <= lnum)
	{
	    dec_cursor();
	}
	else
	{
	    if (!(State & VREPLACE_FLAG)
				   || curwin->w_cursor.lnum > orig_line_count)
	    {
		temp = gchar_cursor();	// remember current char
		--curwin->w_cursor.lnum;

		// When "aw" is in 'formatoptions' we must delete the space at
		// the end of the line, otherwise the line will be broken
		// again when auto-formatting.
		if (has_format_option(FO_AUTO)
					   && has_format_option(FO_WHITE_PAR))
		{
		    char_u  *ptr = ml_get_buf(curbuf, curwin->w_cursor.lnum,
									TRUE);
		    int	    len;

		    len = (int)STRLEN(ptr);
		    if (len > 0 && ptr[len - 1] == ' ')
			ptr[len - 1] = NUL;
		}

		(void)do_join(2, FALSE, FALSE, FALSE, FALSE);
		if (temp == NUL && gchar_cursor() != NUL)
		    inc_cursor();
	    }
	    else
		dec_cursor();

	    /*
	     * In MODE_REPLACE mode we have to put back the text that was
	     * replaced by the NL. On the replace stack is first a
	     * NUL-terminated sequence of characters that were deleted and then
	     * the characters that NL replaced.
	     */
	    if (State & REPLACE_FLAG)
	    {
		/*
		 * Do the next ins_char() in MODE_NORMAL state, to
		 * prevent ins_char() from replacing characters and
		 * avoiding showmatch().
		 */
		oldState = State;
		State = MODE_NORMAL;
		/*
		 * restore characters (blanks) deleted after cursor
		 */
		while (cc > 0)
		{
		    save_col = curwin->w_cursor.col;
		    mb_replace_pop_ins(cc);
		    curwin->w_cursor.col = save_col;
		    cc = replace_pop();
		}
		// restore the characters that NL replaced
		replace_pop_ins();
		State = oldState;
	    }
	}
	did_ai = FALSE;
    }
    else
    {
	/*
	 * Delete character(s) before the cursor.
	 */
#ifdef FEAT_RIGHTLEFT
	if (revins_on)		// put cursor on last inserted char
	    dec_cursor();
#endif
	mincol = 0;
						// keep indent
	if (mode == BACKSPACE_LINE
		&& (curbuf->b_p_ai || cindent_on())
#ifdef FEAT_RIGHTLEFT
		&& !revins_on
#endif
			    )
	{
	    save_col = curwin->w_cursor.col;
	    beginline(BL_WHITE);
	    if (curwin->w_cursor.col < save_col)
	    {
		mincol = curwin->w_cursor.col;
		// should now fix the indent to match with the previous line
		call_fix_indent = TRUE;
	    }
	    curwin->w_cursor.col = save_col;
	}

	/*
	 * Handle deleting one 'shiftwidth' or 'softtabstop'.
	 */
	if (	   mode == BACKSPACE_CHAR
		&& ((p_sta && in_indent)
		    || ((get_sts_value() != 0
#ifdef FEAT_VARTABS
			|| tabstop_count(curbuf->b_p_vsts_array)
#endif
			)
			&& curwin->w_cursor.col > 0
			&& (*(ml_get_cursor() - 1) == TAB
			    || (*(ml_get_cursor() - 1) == ' '
				&& (!*inserted_space_p
				    || arrow_used))))))
	{
	    int		ts;
	    colnr_T	vcol;
	    colnr_T	want_vcol;
	    colnr_T	start_vcol;

	    *inserted_space_p = FALSE;
	    // Compute the virtual column where we want to be.  Since
	    // 'showbreak' may get in the way, need to get the last column of
	    // the previous character.
	    getvcol(curwin, &curwin->w_cursor, &vcol, NULL, NULL);
	    start_vcol = vcol;
	    dec_cursor();
	    getvcol(curwin, &curwin->w_cursor, NULL, NULL, &want_vcol);
	    inc_cursor();
#ifdef FEAT_VARTABS
	    if (p_sta && in_indent)
	    {
		ts = (int)get_sw_value(curbuf);
		want_vcol = (want_vcol / ts) * ts;
	    }
	    else
		want_vcol = tabstop_start(want_vcol, get_sts_value(),
						       curbuf->b_p_vsts_array);
#else
	    if (p_sta && in_indent)
		ts = (int)get_sw_value(curbuf);
	    else
		ts = (int)get_sts_value();
	    want_vcol = (want_vcol / ts) * ts;
#endif

	    // delete characters until we are at or before want_vcol
	    while (vcol > want_vcol && curwin->w_cursor.col > 0
		    && (cc = *(ml_get_cursor() - 1), VIM_ISWHITE(cc)))
		ins_bs_one(&vcol);

	    // insert extra spaces until we are at want_vcol
	    while (vcol < want_vcol)
	    {
		// Remember the first char we inserted
		if (curwin->w_cursor.lnum == Insstart_orig.lnum
				   && curwin->w_cursor.col < Insstart_orig.col)
		    Insstart_orig.col = curwin->w_cursor.col;

		if (State & VREPLACE_FLAG)
		    ins_char(' ');
		else
		{
		    ins_str((char_u *)" ");
		    if ((State & REPLACE_FLAG))
			replace_push(NUL);
		}
		getvcol(curwin, &curwin->w_cursor, &vcol, NULL, NULL);
	    }

	    // If we are now back where we started delete one character.  Can
	    // happen when using 'sts' and 'linebreak'.
	    if (vcol >= start_vcol)
		ins_bs_one(&vcol);
	}

	/*
	 * Delete up to starting point, start of line or previous word.
	 */
	else
	{
	    int cclass = 0, prev_cclass = 0;

	    if (has_mbyte)
		cclass = mb_get_class(ml_get_cursor());
	    do
	    {
#ifdef FEAT_RIGHTLEFT
		if (!revins_on) // put cursor on char to be deleted
#endif
		    dec_cursor();

		cc = gchar_cursor();
		// look multi-byte character class
		if (has_mbyte)
		{
		    prev_cclass = cclass;
		    cclass = mb_get_class(ml_get_cursor());
		}

		// start of word?
		if (mode == BACKSPACE_WORD && !vim_isspace(cc))
		{
		    mode = BACKSPACE_WORD_NOT_SPACE;
		    temp = vim_iswordc(cc);
		}
		// end of word?
		else if (mode == BACKSPACE_WORD_NOT_SPACE
			&& ((vim_isspace(cc) || vim_iswordc(cc) != temp)
			|| prev_cclass != cclass))
		{
#ifdef FEAT_RIGHTLEFT
		    if (!revins_on)
#endif
			inc_cursor();
#ifdef FEAT_RIGHTLEFT
		    else if (State & REPLACE_FLAG)
			dec_cursor();
#endif
		    break;
		}
		if (State & REPLACE_FLAG)
		    replace_do_bs(-1);
		else
		{
		    if (enc_utf8 && p_deco)
			(void)utfc_ptr2char(ml_get_cursor(), cpc);
		    (void)del_char(FALSE);
		    /*
		     * If there are combining characters and 'delcombine' is set
		     * move the cursor back.  Don't back up before the base
		     * character.
		     */
		    if (enc_utf8 && p_deco && cpc[0] != NUL)
			inc_cursor();
#ifdef FEAT_RIGHTLEFT
		    if (revins_chars)
		    {
			revins_chars--;
			revins_legal++;
		    }
		    if (revins_on && gchar_cursor() == NUL)
			break;
#endif
		}
		// Just a single backspace?:
		if (mode == BACKSPACE_CHAR)
		    break;
	    } while (
#ifdef FEAT_RIGHTLEFT
		    revins_on ||
#endif
		    (curwin->w_cursor.col > mincol
		    &&  (can_bs(BS_NOSTOP)
			|| (curwin->w_cursor.lnum != Insstart_orig.lnum
			|| curwin->w_cursor.col != Insstart_orig.col)
		    )));
	}
	did_backspace = TRUE;
    }
    did_si = FALSE;
    can_si = FALSE;
    can_si_back = FALSE;
    if (curwin->w_cursor.col <= 1)
	did_ai = FALSE;

    if (call_fix_indent)
	fix_indent();

    /*
     * It's a little strange to put backspaces into the redo
     * buffer, but it makes auto-indent a lot easier to deal
     * with.
     */
    AppendCharToRedobuff(c);

    // If deleted before the insertion point, adjust it
    if (curwin->w_cursor.lnum == Insstart_orig.lnum
				  && curwin->w_cursor.col < Insstart_orig.col)
	Insstart_orig.col = curwin->w_cursor.col;

    // vi behaviour: the cursor moves backward but the character that
    //		     was there remains visible
    // Vim behaviour: the cursor moves backward and the character that
    //		      was there is erased from the screen.
    // We can emulate the vi behaviour by pretending there is a dollar
    // displayed even when there isn't.
    //  --pkv Sun Jan 19 01:56:40 EST 2003
    if (vim_strchr(p_cpo, CPO_BACKSPACE) != NULL && dollar_vcol == -1)
	dollar_vcol = curwin->w_virtcol;

#ifdef FEAT_FOLDING
    // When deleting a char the cursor line must never be in a closed fold.
    // E.g., when 'foldmethod' is indent and deleting the first non-white
    // char before a Tab.
    if (did_backspace)
	foldOpenCursor();
#endif

    return did_backspace;
}

/*
 * Handle receiving P_PS: start paste mode.  Inserts the following text up to
 * P_PE literally.
 * When "drop" is TRUE then consume the text and drop it.
 */
    int
bracketed_paste(paste_mode_T mode, int drop, garray_T *gap)
{
    int		c;
    char_u	buf[NUMBUFLEN + MB_MAXBYTES];
    int		idx = 0;
    char_u	*end = find_termcode((char_u *)"PE");
    int		ret_char = -1;
    int		save_allow_keys = allow_keys;
    int		save_paste = p_paste;

    // If the end code is too long we can't detect it, read everything.
    if (end != NULL && STRLEN(end) >= NUMBUFLEN)
	end = NULL;
    ++no_mapping;
    allow_keys = 0;
    if (!p_paste)
	// Also have the side effects of setting 'paste' to make it work much
	// faster.
	set_option_value_give_err((char_u *)"paste", TRUE, NULL, 0);

    for (;;)
    {
	// When the end is not defined read everything there is.
	if (end == NULL && vpeekc() == NUL)
	    break;
	do
	    c = vgetc();
	while (c == K_IGNORE || c == K_VER_SCROLLBAR || c == K_HOR_SCROLLBAR);

	if (c == NUL || got_int || (ex_normal_busy > 0 && c == Ctrl_C))
	    // When CTRL-C was encountered the typeahead will be flushed and we
	    // won't get the end sequence.  Except when using ":normal".
	    break;

	if (has_mbyte)
	    idx += (*mb_char2bytes)(c, buf + idx);
	else
	    buf[idx++] = c;
	buf[idx] = NUL;
	if (end != NULL && STRNCMP(buf, end, idx) == 0)
	{
	    if (end[idx] == NUL)
		break; // Found the end of paste code.
	    continue;
	}
	if (!drop)
	{
	    switch (mode)
	    {
		case PASTE_CMDLINE:
		    put_on_cmdline(buf, idx, TRUE);
		    break;

		case PASTE_EX:
		    // add one for the NUL that is going to be appended
		    if (gap != NULL && ga_grow(gap, idx + 1) == OK)
		    {
			mch_memmove((char *)gap->ga_data + gap->ga_len,
							     buf, (size_t)idx);
			gap->ga_len += idx;
		    }
		    break;

		case PASTE_INSERT:
		    if (stop_arrow() == OK)
		    {
			c = buf[0];
			if (idx == 1 && (c == CAR || c == K_KENTER || c == NL))
			    ins_eol(c);
			else
			{
			    ins_char_bytes(buf, idx);
			    AppendToRedobuffLit(buf, idx);
			}
		    }
		    break;

		case PASTE_ONE_CHAR:
		    if (ret_char == -1)
		    {
			if (has_mbyte)
			    ret_char = (*mb_ptr2char)(buf);
			else
			    ret_char = buf[0];
		    }
		    break;
	    }
	}
	idx = 0;
    }

    --no_mapping;
    allow_keys = save_allow_keys;
    if (!save_paste)
	set_option_value_give_err((char_u *)"paste", FALSE, NULL, 0);

    return ret_char;
}

#if defined(FEAT_GUI_TABLINE) || defined(PROTO)
    static void
ins_tabline(int c)
{
    // We will be leaving the current window, unless closing another tab.
    if (c != K_TABMENU || current_tabmenu != TABLINE_MENU_CLOSE
		|| (current_tab != 0 && current_tab != tabpage_index(curtab)))
    {
	undisplay_dollar();
	start_arrow(&curwin->w_cursor);
	can_cindent = TRUE;
    }

    if (c == K_TABLINE)
	goto_tabpage(current_tab);
    else
    {
	handle_tabmenu();
	redraw_statuslines();	// will redraw the tabline when needed
    }
}
#endif

#if defined(FEAT_GUI) || defined(PROTO)
    void
ins_scroll(void)
{
    pos_T	tpos;

    undisplay_dollar();
    tpos = curwin->w_cursor;
    if (gui_do_scroll())
    {
	start_arrow(&tpos);
	can_cindent = TRUE;
    }
}

    void
ins_horscroll(void)
{
    pos_T	tpos;

    undisplay_dollar();
    tpos = curwin->w_cursor;
    if (do_mousescroll_horiz(scrollbar_value))
    {
	start_arrow(&tpos);
	can_cindent = TRUE;
    }
}
#endif

    static void
ins_left(void)
{
    pos_T	tpos;
    int		end_change = dont_sync_undo == FALSE; // end undoable change

#ifdef FEAT_FOLDING
    if ((fdo_flags & FDO_HOR) && KeyTyped)
	foldOpenCursor();
#endif
    undisplay_dollar();
    tpos = curwin->w_cursor;
    if (oneleft() == OK)
    {
#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
	// Only call start_arrow() when not busy with preediting, it will
	// break undo.  K_LEFT is inserted in im_correct_cursor().
	if (p_imst == IM_OVER_THE_SPOT || !im_is_preediting())
#endif
	{
	    start_arrow_with_change(&tpos, end_change);
	    if (!end_change)
		AppendCharToRedobuff(K_LEFT);
	}
#ifdef FEAT_RIGHTLEFT
	// If exit reversed string, position is fixed
	if (revins_scol != -1 && (int)curwin->w_cursor.col >= revins_scol)
	    revins_legal++;
	revins_chars++;
#endif
    }

    /*
     * if 'whichwrap' set for cursor in insert mode may go to
     * previous line
     */
    else if (vim_strchr(p_ww, '[') != NULL && curwin->w_cursor.lnum > 1)
    {
	// always break undo when moving upwards/downwards, else undo may break
	start_arrow(&tpos);
	--(curwin->w_cursor.lnum);
	coladvance((colnr_T)MAXCOL);
	curwin->w_set_curswant = TRUE;	// so we stay at the end
    }
    else
	vim_beep(BO_CRSR);
    dont_sync_undo = FALSE;
}

    static void
ins_home(int c)
{
    pos_T	tpos;

#ifdef FEAT_FOLDING
    if ((fdo_flags & FDO_HOR) && KeyTyped)
	foldOpenCursor();
#endif
    undisplay_dollar();
    tpos = curwin->w_cursor;
    if (c == K_C_HOME)
	curwin->w_cursor.lnum = 1;
    curwin->w_cursor.col = 0;
    curwin->w_cursor.coladd = 0;
    curwin->w_curswant = 0;
    start_arrow(&tpos);
}

    static void
ins_end(int c)
{
    pos_T	tpos;

#ifdef FEAT_FOLDING
    if ((fdo_flags & FDO_HOR) && KeyTyped)
	foldOpenCursor();
#endif
    undisplay_dollar();
    tpos = curwin->w_cursor;
    if (c == K_C_END)
	curwin->w_cursor.lnum = curbuf->b_ml.ml_line_count;
    coladvance((colnr_T)MAXCOL);
    curwin->w_curswant = MAXCOL;

    start_arrow(&tpos);
}

    static void
ins_s_left(void)
{
    int end_change = dont_sync_undo == FALSE; // end undoable change
#ifdef FEAT_FOLDING
    if ((fdo_flags & FDO_HOR) && KeyTyped)
	foldOpenCursor();
#endif
    undisplay_dollar();
    if (curwin->w_cursor.lnum > 1 || curwin->w_cursor.col > 0)
    {
	start_arrow_with_change(&curwin->w_cursor, end_change);
	if (!end_change)
	    AppendCharToRedobuff(K_S_LEFT);
	(void)bck_word(1L, FALSE, FALSE);
	curwin->w_set_curswant = TRUE;
    }
    else
	vim_beep(BO_CRSR);
    dont_sync_undo = FALSE;
}

    static void
ins_right(void)
{
    int end_change = dont_sync_undo == FALSE; // end undoable change

#ifdef FEAT_FOLDING
    if ((fdo_flags & FDO_HOR) && KeyTyped)
	foldOpenCursor();
#endif
    undisplay_dollar();
    if (gchar_cursor() != NUL || virtual_active())
    {
	start_arrow_with_change(&curwin->w_cursor, end_change);
	if (!end_change)
	    AppendCharToRedobuff(K_RIGHT);
	curwin->w_set_curswant = TRUE;
	if (virtual_active())
	    oneright();
	else
	{
	    if (has_mbyte)
		curwin->w_cursor.col += (*mb_ptr2len)(ml_get_cursor());
	    else
		++curwin->w_cursor.col;
	}

#ifdef FEAT_RIGHTLEFT
	revins_legal++;
	if (revins_chars)
	    revins_chars--;
#endif
    }
    // if 'whichwrap' set for cursor in insert mode, may move the
    // cursor to the next line
    else if (vim_strchr(p_ww, ']') != NULL
	    && curwin->w_cursor.lnum < curbuf->b_ml.ml_line_count)
    {
	start_arrow(&curwin->w_cursor);
	curwin->w_set_curswant = TRUE;
	++curwin->w_cursor.lnum;
	curwin->w_cursor.col = 0;
    }
    else
	vim_beep(BO_CRSR);
    dont_sync_undo = FALSE;
}

    static void
ins_s_right(void)
{
    int end_change = dont_sync_undo == FALSE; // end undoable change
#ifdef FEAT_FOLDING
    if ((fdo_flags & FDO_HOR) && KeyTyped)
	foldOpenCursor();
#endif
    undisplay_dollar();
    if (curwin->w_cursor.lnum < curbuf->b_ml.ml_line_count
	    || gchar_cursor() != NUL)
    {
	start_arrow_with_change(&curwin->w_cursor, end_change);
	if (!end_change)
	    AppendCharToRedobuff(K_S_RIGHT);
	(void)fwd_word(1L, FALSE, 0);
	curwin->w_set_curswant = TRUE;
    }
    else
	vim_beep(BO_CRSR);
    dont_sync_undo = FALSE;
}

    static void
ins_up(
    int		startcol)	// when TRUE move to Insstart.col
{
    pos_T	tpos;
    linenr_T	old_topline = curwin->w_topline;
#ifdef FEAT_DIFF
    int		old_topfill = curwin->w_topfill;
#endif

    undisplay_dollar();
    tpos = curwin->w_cursor;
    if (cursor_up(1L, TRUE) == OK)
    {
	if (startcol)
	    coladvance(getvcol_nolist(&Insstart));
	if (old_topline != curwin->w_topline
#ifdef FEAT_DIFF
		|| old_topfill != curwin->w_topfill
#endif
		)
	    redraw_later(UPD_VALID);
	start_arrow(&tpos);
	can_cindent = TRUE;
    }
    else
	vim_beep(BO_CRSR);
}

    static void
ins_pageup(void)
{
    pos_T	tpos;

    undisplay_dollar();

    if (mod_mask & MOD_MASK_CTRL)
    {
	// <C-PageUp>: tab page back
	if (first_tabpage->tp_next != NULL)
	{
	    start_arrow(&curwin->w_cursor);
	    goto_tabpage(-1);
	}
	return;
    }

    tpos = curwin->w_cursor;
    if (onepage(BACKWARD, 1L) == OK)
    {
	start_arrow(&tpos);
	can_cindent = TRUE;
    }
    else
	vim_beep(BO_CRSR);
}

    static void
ins_down(
    int		startcol)	// when TRUE move to Insstart.col
{
    pos_T	tpos;
    linenr_T	old_topline = curwin->w_topline;
#ifdef FEAT_DIFF
    int		old_topfill = curwin->w_topfill;
#endif

    undisplay_dollar();
    tpos = curwin->w_cursor;
    if (cursor_down(1L, TRUE) == OK)
    {
	if (startcol)
	    coladvance(getvcol_nolist(&Insstart));
	if (old_topline != curwin->w_topline
#ifdef FEAT_DIFF
		|| old_topfill != curwin->w_topfill
#endif
		)
	    redraw_later(UPD_VALID);
	start_arrow(&tpos);
	can_cindent = TRUE;
    }
    else
	vim_beep(BO_CRSR);
}

    static void
ins_pagedown(void)
{
    pos_T	tpos;

    undisplay_dollar();

    if (mod_mask & MOD_MASK_CTRL)
    {
	// <C-PageDown>: tab page forward
	if (first_tabpage->tp_next != NULL)
	{
	    start_arrow(&curwin->w_cursor);
	    goto_tabpage(0);
	}
	return;
    }

    tpos = curwin->w_cursor;
    if (onepage(FORWARD, 1L) == OK)
    {
	start_arrow(&tpos);
	can_cindent = TRUE;
    }
    else
	vim_beep(BO_CRSR);
}

#ifdef FEAT_DND
    static void
ins_drop(void)
{
    do_put('~', NULL, BACKWARD, 1L, PUT_CURSEND);
}
#endif

/*
 * Handle TAB in Insert or Replace mode.
 * Return TRUE when the TAB needs to be inserted like a normal character.
 */
    static int
ins_tab(void)
{
    int		ind;
    int		i;
    int		temp;

    if (Insstart_blank_vcol == MAXCOL && curwin->w_cursor.lnum == Insstart.lnum)
	Insstart_blank_vcol = get_nolist_virtcol();
    if (echeck_abbr(TAB + ABBR_OFF))
	return FALSE;

    ind = inindent(0);
    if (ind)
	can_cindent = FALSE;

    /*
     * When nothing special, insert TAB like a normal character.
     */
    if (!curbuf->b_p_et
#ifdef FEAT_VARTABS
	    && !(p_sta && ind
		// These five lines mean 'tabstop' != 'shiftwidth'
		&& ((tabstop_count(curbuf->b_p_vts_array) > 1)
		    || (tabstop_count(curbuf->b_p_vts_array) == 1
			&& tabstop_first(curbuf->b_p_vts_array)
						       != get_sw_value(curbuf))
		    || (tabstop_count(curbuf->b_p_vts_array) == 0
			&& curbuf->b_p_ts != get_sw_value(curbuf))))
	    && tabstop_count(curbuf->b_p_vsts_array) == 0
#else
	    && !(p_sta && ind && curbuf->b_p_ts != get_sw_value(curbuf))
#endif
	    && get_sts_value() == 0)
	return TRUE;

    if (stop_arrow() == FAIL)
	return TRUE;

    did_ai = FALSE;
    did_si = FALSE;
    can_si = FALSE;
    can_si_back = FALSE;
    AppendToRedobuff((char_u *)"\t");

#ifdef FEAT_VARTABS
    if (p_sta && ind)		// insert tab in indent, use 'shiftwidth'
    {
	temp = (int)get_sw_value(curbuf);
	temp -= get_nolist_virtcol() % temp;
    }
    else if (tabstop_count(curbuf->b_p_vsts_array) > 0 || curbuf->b_p_sts != 0)
				// use 'softtabstop' when set
	temp = tabstop_padding(get_nolist_virtcol(), get_sts_value(),
						     curbuf->b_p_vsts_array);
    else			// otherwise use 'tabstop'
	temp = tabstop_padding(get_nolist_virtcol(), curbuf->b_p_ts,
						     curbuf->b_p_vts_array);
#else
    if (p_sta && ind)		// insert tab in indent, use 'shiftwidth'
	temp = (int)get_sw_value(curbuf);
    else if (curbuf->b_p_sts != 0) // use 'softtabstop' when set
	temp = (int)get_sts_value();
    else			// otherwise use 'tabstop'
	temp = (int)curbuf->b_p_ts;
    temp -= get_nolist_virtcol() % temp;
#endif

    /*
     * Insert the first space with ins_char().	It will delete one char in
     * replace mode.  Insert the rest with ins_str(); it will not delete any
     * chars.  For MODE_VREPLACE state, we use ins_char() for all characters.
     */
    ins_char(' ');
    while (--temp > 0)
    {
	if (State & VREPLACE_FLAG)
	    ins_char(' ');
	else
	{
	    ins_str((char_u *)" ");
	    if (State & REPLACE_FLAG)	    // no char replaced
		replace_push(NUL);
	}
    }

    /*
     * When 'expandtab' not set: Replace spaces by TABs where possible.
     */
#ifdef FEAT_VARTABS
    if (!curbuf->b_p_et && (tabstop_count(curbuf->b_p_vsts_array) > 0
			    || get_sts_value() > 0
			    || (p_sta && ind)))
#else
    if (!curbuf->b_p_et && (get_sts_value() || (p_sta && ind)))
#endif
    {
	char_u		*ptr;
	char_u		*saved_line = NULL;	// init for GCC
	pos_T		pos;
	pos_T		fpos;
	pos_T		*cursor;
	colnr_T		want_vcol, vcol;
	int		change_col = -1;
	int		save_list = curwin->w_p_list;
	char_u		*tab = (char_u *)"\t";
	chartabsize_T	cts;

	/*
	 * Get the current line.  For MODE_VREPLACE state, don't make real
	 * changes yet, just work on a copy of the line.
	 */
	if (State & VREPLACE_FLAG)
	{
	    pos = curwin->w_cursor;
	    cursor = &pos;
	    saved_line = vim_strsave(ml_get_curline());
	    if (saved_line == NULL)
		return FALSE;
	    ptr = saved_line + pos.col;
	}
	else
	{
	    ptr = ml_get_cursor();
	    cursor = &curwin->w_cursor;
	}

	// When 'L' is not in 'cpoptions' a tab always takes up 'ts' spaces.
	if (vim_strchr(p_cpo, CPO_LISTWM) == NULL)
	    curwin->w_p_list = FALSE;

	// Find first white before the cursor
	fpos = curwin->w_cursor;
	while (fpos.col > 0 && VIM_ISWHITE(ptr[-1]))
	{
	    --fpos.col;
	    --ptr;
	}

	// In Replace mode, don't change characters before the insert point.
	if ((State & REPLACE_FLAG)
		&& fpos.lnum == Insstart.lnum
		&& fpos.col < Insstart.col)
	{
	    ptr += Insstart.col - fpos.col;
	    fpos.col = Insstart.col;
	}

	// compute virtual column numbers of first white and cursor
	getvcol(curwin, &fpos, &vcol, NULL, NULL);
	getvcol(curwin, cursor, &want_vcol, NULL, NULL);

	init_chartabsize_arg(&cts, curwin, 0, vcol, tab, tab);

	// Use as many TABs as possible.  Beware of 'breakindent', 'showbreak'
	// and 'linebreak' adding extra virtual columns.
	while (VIM_ISWHITE(*ptr))
	{
	    i = lbr_chartabsize(&cts);
	    if (cts.cts_vcol + i > want_vcol)
		break;
	    if (*ptr != TAB)
	    {
		*ptr = TAB;
		if (change_col < 0)
		{
		    change_col = fpos.col;  // Column of first change
		    // May have to adjust Insstart
		    if (fpos.lnum == Insstart.lnum && fpos.col < Insstart.col)
			Insstart.col = fpos.col;
		}
	    }
	    ++fpos.col;
	    ++ptr;
	    cts.cts_vcol += i;
	}
	vcol = cts.cts_vcol;
	clear_chartabsize_arg(&cts);

	if (change_col >= 0)
	{
	    int		    repl_off = 0;

	    // Skip over the spaces we need.
	    init_chartabsize_arg(&cts, curwin, 0, vcol, ptr, ptr);
	    while (cts.cts_vcol < want_vcol && *cts.cts_ptr == ' ')
	    {
		cts.cts_vcol += lbr_chartabsize(&cts);
		++cts.cts_ptr;
		++repl_off;
	    }
	    ptr = cts.cts_ptr;
	    vcol = cts.cts_vcol;
	    clear_chartabsize_arg(&cts);

	    if (vcol > want_vcol)
	    {
		// Must have a char with 'showbreak' just before it.
		--ptr;
		--repl_off;
	    }
	    fpos.col += repl_off;

	    // Delete following spaces.
	    i = cursor->col - fpos.col;
	    if (i > 0)
	    {
#ifdef FEAT_PROP_POPUP
		if (!(State & VREPLACE_FLAG))
		{
		    char_u  *newp;
		    int	    col;

		    newp = alloc(curbuf->b_ml.ml_line_len - i);
		    if (newp == NULL)
			return FALSE;

		    col = ptr - curbuf->b_ml.ml_line_ptr;
		    if (col > 0)
			mch_memmove(newp, ptr - col, col);
		    mch_memmove(newp + col, ptr + i,
					   curbuf->b_ml.ml_line_len - col - i);

		    if (curbuf->b_ml.ml_flags & (ML_LINE_DIRTY | ML_ALLOCATED))
			vim_free(curbuf->b_ml.ml_line_ptr);
		    curbuf->b_ml.ml_line_ptr = newp;
		    curbuf->b_ml.ml_line_len -= i;
		    curbuf->b_ml.ml_flags =
			   (curbuf->b_ml.ml_flags | ML_LINE_DIRTY) & ~ML_EMPTY;
		}
		else
#endif
		    STRMOVE(ptr, ptr + i);
		// correct replace stack.
		if ((State & REPLACE_FLAG) && !(State & VREPLACE_FLAG))
		    for (temp = i; --temp >= 0; )
			replace_join(repl_off);
	    }
#ifdef FEAT_NETBEANS_INTG
	    if (netbeans_active())
	    {
		netbeans_removed(curbuf, fpos.lnum, cursor->col, (long)(i + 1));
		netbeans_inserted(curbuf, fpos.lnum, cursor->col,
							   (char_u *)"\t", 1);
	    }
#endif
	    cursor->col -= i;

	    /*
	     * In MODE_VREPLACE state, we haven't changed anything yet.  Do it
	     * now by backspacing over the changed spacing and then inserting
	     * the new spacing.
	     */
	    if (State & VREPLACE_FLAG)
	    {
		// Backspace from real cursor to change_col
		backspace_until_column(change_col);

		// Insert each char in saved_line from changed_col to
		// ptr-cursor
		ins_bytes_len(saved_line + change_col,
						    cursor->col - change_col);
	    }
	}

	if (State & VREPLACE_FLAG)
	    vim_free(saved_line);
	curwin->w_p_list = save_list;
    }

    return FALSE;
}

/*
 * Handle CR or NL in insert mode.
 * Return FAIL when out of memory or can't undo.
 */
    int
ins_eol(int c)
{
    int	    i;

    if (echeck_abbr(c + ABBR_OFF))
	return OK;
    if (stop_arrow() == FAIL)
	return FAIL;
    undisplay_dollar();

    /*
     * Strange Vi behaviour: In Replace mode, typing a NL will not delete the
     * character under the cursor.  Only push a NUL on the replace stack,
     * nothing to put back when the NL is deleted.
     */
    if ((State & REPLACE_FLAG) && !(State & VREPLACE_FLAG))
	replace_push(NUL);

    /*
     * In MODE_VREPLACE state, a NL replaces the rest of the line, and starts
     * replacing the next line, so we push all of the characters left on the
     * line onto the replace stack.  This is not done here though, it is done
     * in open_line().
     */

    // Put cursor on NUL if on the last char and coladd is 1 (happens after
    // CTRL-O).
    if (virtual_active() && curwin->w_cursor.coladd > 0)
	coladvance(getviscol());

#ifdef FEAT_RIGHTLEFT
    // NL in reverse insert will always start in the end of
    // current line.
    if (revins_on)
	curwin->w_cursor.col += (colnr_T)STRLEN(ml_get_cursor());
#endif

    AppendToRedobuff(NL_STR);
    i = open_line(FORWARD,
	    has_format_option(FO_RET_COMS) ? OPENLINE_DO_COM : 0, old_indent,
	    NULL);
    old_indent = 0;
    can_cindent = TRUE;
#ifdef FEAT_FOLDING
    // When inserting a line the cursor line must never be in a closed fold.
    foldOpenCursor();
#endif

    return i;
}

#ifdef FEAT_DIGRAPHS
/*
 * Handle digraph in insert mode.
 * Returns character still to be inserted, or NUL when nothing remaining to be
 * done.
 */
    static int
ins_digraph(void)
{
    int	    c;
    int	    cc;
    int	    did_putchar = FALSE;

    pc_status = PC_STATUS_UNSET;
    if (redrawing() && !char_avail())
    {
	// may need to redraw when no more chars available now
	ins_redraw(FALSE);

	edit_putchar('?', TRUE);
	did_putchar = TRUE;
	add_to_showcmd_c(Ctrl_K);
    }

#ifdef USE_ON_FLY_SCROLL
    dont_scroll = TRUE;		// disallow scrolling here
#endif

    // don't map the digraph chars. This also prevents the
    // mode message to be deleted when ESC is hit
    ++no_mapping;
    ++allow_keys;
    c = plain_vgetc();
    --no_mapping;
    --allow_keys;
    if (did_putchar)
	// when the line fits in 'columns' the '?' is at the start of the next
	// line and will not be removed by the redraw
	edit_unputchar();

    if (IS_SPECIAL(c) || mod_mask)	    // special key
    {
	clear_showcmd();
	insert_special(c, TRUE, FALSE);
	return NUL;
    }
    if (c != ESC)
    {
	did_putchar = FALSE;
	if (redrawing() && !char_avail())
	{
	    // may need to redraw when no more chars available now
	    ins_redraw(FALSE);

	    if (char2cells(c) == 1)
	    {
		ins_redraw(FALSE);
		edit_putchar(c, TRUE);
		did_putchar = TRUE;
	    }
	    add_to_showcmd_c(c);
	}
	++no_mapping;
	++allow_keys;
	cc = plain_vgetc();
	--no_mapping;
	--allow_keys;
	if (did_putchar)
	    // when the line fits in 'columns' the '?' is at the start of the
	    // next line and will not be removed by a redraw
	    edit_unputchar();
	if (cc != ESC)
	{
	    AppendToRedobuff((char_u *)CTRL_V_STR);
	    c = digraph_get(c, cc, TRUE);
	    clear_showcmd();
	    return c;
	}
    }
    clear_showcmd();
    return NUL;
}
#endif

/*
 * Handle CTRL-E and CTRL-Y in Insert mode: copy char from other line.
 * Returns the char to be inserted, or NUL if none found.
 */
    int
ins_copychar(linenr_T lnum)
{
    int		    c;
    char_u	    *ptr, *prev_ptr;
    char_u	    *line;
    chartabsize_T   cts;

    if (lnum < 1 || lnum > curbuf->b_ml.ml_line_count)
    {
	vim_beep(BO_COPY);
	return NUL;
    }

    // try to advance to the cursor column
    validate_virtcol();
    line = ml_get(lnum);
    prev_ptr = line;
    init_chartabsize_arg(&cts, curwin, lnum, 0, line, line);
    while (cts.cts_vcol < curwin->w_virtcol && *cts.cts_ptr != NUL)
    {
	prev_ptr = cts.cts_ptr;
	cts.cts_vcol += lbr_chartabsize_adv(&cts);
    }
    if (cts.cts_vcol > curwin->w_virtcol)
	ptr = prev_ptr;
    else
	ptr = cts.cts_ptr;
    clear_chartabsize_arg(&cts);

    c = (*mb_ptr2char)(ptr);
    if (c == NUL)
	vim_beep(BO_COPY);
    return c;
}

/*
 * CTRL-Y or CTRL-E typed in Insert mode.
 */
    static int
ins_ctrl_ey(int tc)
{
    int	    c = tc;

    if (ctrl_x_mode_scroll())
    {
	if (c == Ctrl_Y)
	    scrolldown_clamp();
	else
	    scrollup_clamp();
	redraw_later(UPD_VALID);
    }
    else
    {
	c = ins_copychar(curwin->w_cursor.lnum + (c == Ctrl_Y ? -1 : 1));
	if (c != NUL)
	{
	    long	tw_save;

	    // The character must be taken literally, insert like it
	    // was typed after a CTRL-V, and pretend 'textwidth'
	    // wasn't set.  Digits, 'o' and 'x' are special after a
	    // CTRL-V, don't use it for these.
	    if (c < 256 && !SAFE_isalnum(c))
		AppendToRedobuff((char_u *)CTRL_V_STR);	// CTRL-V
	    tw_save = curbuf->b_p_tw;
	    curbuf->b_p_tw = -1;
	    insert_special(c, TRUE, FALSE);
	    curbuf->b_p_tw = tw_save;
#ifdef FEAT_RIGHTLEFT
	    revins_chars++;
	    revins_legal++;
#endif
	    c = Ctrl_V;	// pretend CTRL-V is last character
	    auto_format(FALSE, TRUE);
	}
    }
    return c;
}

/*
 * Get the value that w_virtcol would have when 'list' is off.
 * Unless 'cpo' contains the 'L' flag.
 */
    colnr_T
get_nolist_virtcol(void)
{
    // check validity of cursor in current buffer
    if (curwin->w_buffer == NULL
	|| curwin->w_buffer->b_ml.ml_mfp == NULL
	|| curwin->w_cursor.lnum > curwin->w_buffer->b_ml.ml_line_count)
	return 0;
    if (curwin->w_p_list && vim_strchr(p_cpo, CPO_LISTWM) == NULL)
	return getvcol_nolist(&curwin->w_cursor);
    validate_virtcol();
    return curwin->w_virtcol;
}

#if defined(FEAT_EVAL)
/*
 * Handle the InsertCharPre autocommand.
 * "c" is the character that was typed.
 * Return a pointer to allocated memory with the replacement string.
 * Return NULL to continue inserting "c".
 */
    static char_u *
do_insert_char_pre(int c)
{
    char_u	*res;
    char_u	buf[MB_MAXBYTES + 1];
    int		save_State = State;

    // Return quickly when there is nothing to do.
    if (!has_insertcharpre())
	return NULL;

    if (c == Ctrl_RSB)
	return NULL;

    if (has_mbyte)
	buf[(*mb_char2bytes)(c, buf)] = NUL;
    else
    {
	buf[0] = c;
	buf[1] = NUL;
    }

    // Lock the text to avoid weird things from happening.
    ++textlock;
    set_vim_var_string(VV_CHAR, buf, -1);  // set v:char

    res = NULL;
    if (ins_apply_autocmds(EVENT_INSERTCHARPRE))
    {
	// Get the value of v:char.  It may be empty or more than one
	// character.  Only use it when changed, otherwise continue with the
	// original character to avoid breaking autoindent.
	if (STRCMP(buf, get_vim_var_str(VV_CHAR)) != 0)
	    res = vim_strsave(get_vim_var_str(VV_CHAR));
    }

    set_vim_var_string(VV_CHAR, NULL, -1);  // clear v:char
    --textlock;

    // Restore the State, it may have been changed.
    State = save_State;

    return res;
}
#endif

    int
get_can_cindent(void)
{
    return can_cindent;
}

    void
set_can_cindent(int val)
{
    can_cindent = val;
}

/*
 * Trigger "event" and take care of fixing undo.
 */
    int
ins_apply_autocmds(event_T event)
{
    varnumber_T	tick = CHANGEDTICK(curbuf);
    int r;

    r = apply_autocmds(event, NULL, NULL, FALSE, curbuf);

    // If u_savesub() was called then we are not prepared to start
    // a new line.  Call u_save() with no contents to fix that.
    // Except when leaving Insert mode.
    if (event != EVENT_INSERTLEAVE && tick != CHANGEDTICK(curbuf))
	u_save(curwin->w_cursor.lnum, (linenr_T)(curwin->w_cursor.lnum + 1));

    return r;
}
