/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * drawscreen.c: Code for updating all the windows on the screen.
 * This is the top level, drawline.c is the middle and screen.c the lower
 * level.
 *
 * update_screen() is the function that updates all windows and status lines.
 * It is called form the main loop when must_redraw is non-zero.  It may be
 * called from other places when an immediate screen update is needed.
 *
 * The part of the buffer that is displayed in a window is set with:
 * - w_topline (first buffer line in window)
 * - w_topfill (filler lines above the first line)
 * - w_leftcol (leftmost window cell in window),
 * - w_skipcol (skipped window cells of first line)
 *
 * Commands that only move the cursor around in a window, do not need to take
 * action to update the display.  The main loop will check if w_topline is
 * valid and update it (scroll the window) when needed.
 *
 * Commands that scroll a window change w_topline and must call
 * check_cursor() to move the cursor into the visible part of the window, and
 * call redraw_later(UPD_VALID) to have the window displayed by update_screen()
 * later.
 *
 * Commands that change text in the buffer must call changed_bytes() or
 * changed_lines() to mark the area that changed and will require updating
 * later.  The main loop will call update_screen(), which will update each
 * window that shows the changed buffer.  This assumes text above the change
 * can remain displayed as it is.  Text after the change may need updating for
 * scrolling, folding and syntax highlighting.
 *
 * Commands that change how a window is displayed (e.g., setting 'list') or
 * invalidate the contents of a window in another way (e.g., change fold
 * settings), must call redraw_later(UPD_NOT_VALID) to have the whole window
 * redisplayed by update_screen() later.
 *
 * Commands that change how a buffer is displayed (e.g., setting 'tabstop')
 * must call redraw_curbuf_later(UPD_NOT_VALID) to have all the windows for the
 * buffer redisplayed by update_screen() later.
 *
 * Commands that change highlighting and possibly cause a scroll too must call
 * redraw_later(UPD_SOME_VALID) to update the whole window but still use
 * scrolling to avoid redrawing everything.  But the length of displayed lines
 * must not change, use UPD_NOT_VALID then.
 *
 * Commands that move the window position must call redraw_later(UPD_NOT_VALID).
 * TODO: should minimize redrawing by scrolling when possible.
 *
 * Commands that change everything (e.g., resizing the screen) must call
 * redraw_all_later(UPD_NOT_VALID) or redraw_all_later(UPD_CLEAR).
 *
 * Things that are handled indirectly:
 * - When messages scroll the screen up, msg_scrolled will be set and
 *   update_screen() called to redraw.
 */

#include "vim.h"

static void win_update(win_T *wp);
#ifdef FEAT_STL_OPT
static void redraw_custom_statusline(win_T *wp);
#endif
#if defined(FEAT_SEARCH_EXTRA) || defined(FEAT_CLIPBOARD)
static int  did_update_one_window;
#endif

/*
 * Based on the current value of curwin->w_topline, transfer a screenfull
 * of stuff from Filemem to ScreenLines[], and update curwin->w_botline.
 * Return OK when the screen was updated, FAIL if it was not done.
 */
    int
update_screen(int type_arg)
{
    int		type = type_arg;
    win_T	*wp;
    static int	did_intro = FALSE;
#ifdef FEAT_GUI
    int		did_one = FALSE;
    int		did_undraw = FALSE;
    int		gui_cursor_col = 0;
    int		gui_cursor_row = 0;
#endif
    int		no_update = FALSE;
    int		save_pum_will_redraw = pum_will_redraw;

    // Don't do anything if the screen structures are (not yet) valid.
    if (!screen_valid(TRUE))
	return FAIL;

    if (type == UPD_VALID_NO_UPDATE)
    {
	no_update = TRUE;
	type = 0;
    }

#ifdef FEAT_EVAL
    {
	buf_T *buf;

	// Before updating the screen, notify any listeners of changed text.
	FOR_ALL_BUFFERS(buf)
	    invoke_listeners(buf);
    }
#endif

#ifdef FEAT_DIFF
    // May have postponed updating diffs.
    if (need_diff_redraw)
	diff_redraw(TRUE);
#endif

    if (must_redraw)
    {
	if (type < must_redraw)	    // use maximal type
	    type = must_redraw;

	// must_redraw is reset here, so that when we run into some weird
	// reason to redraw while busy redrawing (e.g., asynchronous
	// scrolling), or update_topline() in win_update() will cause a
	// scroll, the screen will be redrawn later or in win_update().
	must_redraw = 0;
    }

    // May need to update w_lines[].
    if (curwin->w_lines_valid == 0 && type < UPD_NOT_VALID
#ifdef FEAT_TERMINAL
	    && !term_do_update_window(curwin)
#endif
		)
	type = UPD_NOT_VALID;

    // Postpone the redrawing when it's not needed and when being called
    // recursively.
    if (!redrawing() || updating_screen)
    {
	redraw_later(type);		// remember type for next time
	must_redraw = type;
	if (type > UPD_INVERTED_ALL)
	    curwin->w_lines_valid = 0;	// don't use w_lines[].wl_size now
	return FAIL;
    }
    updating_screen = TRUE;

#ifdef FEAT_PROP_POPUP
    // Update popup_mask if needed.  This may set w_redraw_top and w_redraw_bot
    // in some windows.
    may_update_popup_mask(type);
#endif

#ifdef FEAT_SYN_HL
    ++display_tick;	    // let syntax code know we're in a next round of
			    // display updating
#endif
    if (no_update)
	++no_win_do_lines_ins;

    // if the screen was scrolled up when displaying a message, scroll it down
    if (msg_scrolled)
    {
	clear_cmdline = TRUE;
	if (type != UPD_CLEAR)
	{
	    if (msg_scrolled > Rows - 5)	    // redrawing is faster
	    {
		type = UPD_NOT_VALID;
		redraw_as_cleared();
	    }
	    else
	    {
		check_for_delay(FALSE);
		if (screen_ins_lines(0, 0, msg_scrolled, (int)Rows, 0, NULL)
								       == FAIL)
		{
		    type = UPD_NOT_VALID;
		    redraw_as_cleared();
		}
		FOR_ALL_WINDOWS(wp)
		{
		    if (wp->w_winrow < msg_scrolled)
		    {
			if (W_WINROW(wp) + wp->w_height > msg_scrolled
				&& wp->w_redr_type < UPD_REDRAW_TOP
				&& wp->w_lines_valid > 0
				&& wp->w_topline == wp->w_lines[0].wl_lnum)
			{
			    wp->w_upd_rows = msg_scrolled - W_WINROW(wp);
			    wp->w_redr_type = UPD_REDRAW_TOP;
			}
			else
			{
			    wp->w_redr_type = UPD_NOT_VALID;
			    if (W_WINROW(wp) + wp->w_height
					 + wp->w_status_height <= msg_scrolled)
				wp->w_redr_status = TRUE;
			}
		    }
		}
		if (!no_update)
		    redraw_cmdline = TRUE;
		redraw_tabline = TRUE;
	    }
	}
	msg_scrolled = 0;
	need_wait_return = FALSE;
    }

    // reset cmdline_row now (may have been changed temporarily)
    compute_cmdrow();

    // Check for changed highlighting
    if (need_highlight_changed)
	highlight_changed();

    if (type == UPD_CLEAR)		// first clear screen
    {
	screenclear();		// will reset clear_cmdline
	type = UPD_NOT_VALID;
	// must_redraw may be set indirectly, avoid another redraw later
	must_redraw = 0;
    }

    if (clear_cmdline)		// going to clear cmdline (done below)
	check_for_delay(FALSE);

#ifdef FEAT_LINEBREAK
    // Force redraw when width of 'number' or 'relativenumber' column
    // changes.
    if (curwin->w_redr_type < UPD_NOT_VALID
	   && curwin->w_nrwidth != ((curwin->w_p_nu || curwin->w_p_rnu)
				    ? number_width(curwin) : 0))
	curwin->w_redr_type = UPD_NOT_VALID;
#endif

    // Only start redrawing if there is really something to do.
    if (type == UPD_INVERTED)
	update_curswant();
    if (curwin->w_redr_type < type
	    && !((type == UPD_VALID
		    && curwin->w_lines[0].wl_valid
#ifdef FEAT_DIFF
		    && curwin->w_topfill == curwin->w_old_topfill
		    && curwin->w_botfill == curwin->w_old_botfill
#endif
		    && curwin->w_topline == curwin->w_lines[0].wl_lnum)
		|| (type == UPD_INVERTED
		    && VIsual_active
		    && curwin->w_old_cursor_lnum == curwin->w_cursor.lnum
		    && curwin->w_old_visual_mode == VIsual_mode
		    && (curwin->w_valid & VALID_VIRTCOL)
		    && curwin->w_old_curswant == curwin->w_curswant)
		))
	curwin->w_redr_type = type;

    // Redraw the tab pages line if needed.
    if (redraw_tabline || type >= UPD_NOT_VALID)
	draw_tabline();

#ifdef FEAT_SYN_HL
    // Correct stored syntax highlighting info for changes in each displayed
    // buffer.  Each buffer must only be done once.
    FOR_ALL_WINDOWS(wp)
    {
	if (wp->w_buffer->b_mod_set)
	{
	    win_T	*wwp;

	    // Check if we already did this buffer.
	    for (wwp = firstwin; wwp != wp; wwp = wwp->w_next)
		if (wwp->w_buffer == wp->w_buffer)
		    break;
	    if (wwp == wp && syntax_present(wp))
		syn_stack_apply_changes(wp->w_buffer);
	}
    }
#endif

    if (pum_redraw_in_same_position())
	// Avoid flicker if the popup menu is going to be redrawn in the same
	// position.
	pum_will_redraw = TRUE;

    // Go from top to bottom through the windows, redrawing the ones that need
    // it.
#if defined(FEAT_SEARCH_EXTRA) || defined(FEAT_CLIPBOARD)
    did_update_one_window = FALSE;
#endif
#ifdef FEAT_SEARCH_EXTRA
    screen_search_hl.rm.regprog = NULL;
#endif
    FOR_ALL_WINDOWS(wp)
    {
	if (wp->w_redr_type != 0)
	{
	    cursor_off();
#ifdef FEAT_GUI
	    if (!did_one)
	    {
		did_one = TRUE;

		// Remove the cursor before starting to do anything, because
		// scrolling may make it difficult to redraw the text under
		// it.
		// Also remove the cursor if it needs to be hidden due to an
		// ongoing cursor-less sleep.
		if (gui.in_use && (wp == curwin || cursor_is_sleeping()))
		{
		    gui_cursor_col = gui.cursor_col;
		    gui_cursor_row = gui.cursor_row;
		    gui_undraw_cursor();
		    did_undraw = TRUE;
		}
	    }
#endif
	    win_update(wp);
	}

	// redraw status line after the window to minimize cursor movement
	if (wp->w_redr_status)
	{
	    cursor_off();
	    win_redr_status(wp, TRUE); // any popup menu will be redrawn below
	}
    }
#if defined(FEAT_SEARCH_EXTRA)
    end_search_hl();
#endif

    // May need to redraw the popup menu.
    pum_will_redraw = save_pum_will_redraw;
    pum_may_redraw();

    // Reset b_mod_set flags.  Going through all windows is probably faster
    // than going through all buffers (there could be many buffers).
    FOR_ALL_WINDOWS(wp)
	wp->w_buffer->b_mod_set = FALSE;

#ifdef FEAT_PROP_POPUP
    // Display popup windows on top of the windows and command line.
    update_popups(win_update);
#endif

#ifdef FEAT_TERMINAL
    FOR_ALL_WINDOWS(wp)
	// If this window contains a terminal, after redrawing all windows, the
	// dirty row range can be reset.
	term_did_update_window(wp);
#endif

    after_updating_screen(TRUE);

    // Clear or redraw the command line.  Done last, because scrolling may
    // mess up the command line.
    if (clear_cmdline || redraw_cmdline || redraw_mode)
	showmode();

    if (no_update)
	--no_win_do_lines_ins;

    // May put up an introductory message when not editing a file
    if (!did_intro)
	maybe_intro_message();
    did_intro = TRUE;

#ifdef FEAT_GUI
    // Redraw the cursor and update the scrollbars when all screen updating is
    // done.
    if (gui.in_use)
    {
	if (did_undraw && !gui_mch_is_blink_off())
	{
	    mch_disable_flush();
	    out_flush();	// required before updating the cursor
	    mch_enable_flush();

	    // Put the GUI position where the cursor was, gui_update_cursor()
	    // uses that.
	    gui.col = gui_cursor_col;
	    gui.row = gui_cursor_row;
	    gui.col = mb_fix_col(gui.col, gui.row);
	    gui_update_cursor(FALSE, FALSE);
	    gui_may_flush();
	    screen_cur_col = gui.col;
	    screen_cur_row = gui.row;
	}
	else
	    out_flush();
	gui_update_scrollbars(FALSE);
    }
#endif
    return OK;
}

/*
 * Return the row for drawing the statusline and the ruler of window "wp".
 */
    int
statusline_row(win_T *wp)
{
#if defined(FEAT_PROP_POPUP)
    // If the window is really zero height the winbar isn't displayed.
    if (wp->w_frame->fr_height == wp->w_status_height && !popup_is_popup(wp))
	return wp->w_winrow;
#endif
    return W_WINROW(wp) + wp->w_height;
}

/*
 * Redraw the status line of window wp.
 *
 * If inversion is possible we use it. Else '=' characters are used.
 * If "ignore_pum" is TRUE, also redraw statusline when the popup menu is
 * displayed.
 */
    void
win_redr_status(win_T *wp, int ignore_pum UNUSED)
{
    int		row;
    char_u	*p;
    int		len;
    int		fillchar;
    int		attr;
    int		this_ru_col;
    static int  busy = FALSE;

    // It's possible to get here recursively when 'statusline' (indirectly)
    // invokes ":redrawstatus".  Simply ignore the call then.
    if (busy)
	return;
    busy = TRUE;

    row = statusline_row(wp);

    wp->w_redr_status = FALSE;
    if (wp->w_status_height == 0)
    {
	// no status line, can only be last window
	redraw_cmdline = TRUE;
    }
    else if (!redrawing()
	    // don't update status line when popup menu is visible and may be
	    // drawn over it, unless it will be redrawn later
	    || (!ignore_pum && pum_visible()))
    {
	// Don't redraw right now, do it later.
	wp->w_redr_status = TRUE;
    }
#ifdef FEAT_STL_OPT
    else if (*p_stl != NUL || *wp->w_p_stl != NUL)
    {
	// redraw custom status line
	redraw_custom_statusline(wp);
    }
#endif
    else
    {
	fillchar = fillchar_status(&attr, wp);

	get_trans_bufname(wp->w_buffer);
	p = NameBuff;
	len = (int)STRLEN(p);

	if ((bt_help(wp->w_buffer)
#ifdef FEAT_QUICKFIX
		    || wp->w_p_pvw
#endif
		    || bufIsChanged(wp->w_buffer)
		    || wp->w_buffer->b_p_ro)
		&& len < MAXPATHL - 1)
	    *(p + len++) = ' ';
	if (bt_help(wp->w_buffer))
	{
	    vim_snprintf((char *)p + len, MAXPATHL - len, "%s", _("[Help]"));
	    len += (int)STRLEN(p + len);
	}
#ifdef FEAT_QUICKFIX
	if (wp->w_p_pvw)
	{
	    vim_snprintf((char *)p + len, MAXPATHL - len, "%s", _("[Preview]"));
	    len += (int)STRLEN(p + len);
	}
#endif
	if (bufIsChanged(wp->w_buffer) && !bt_terminal(wp->w_buffer))
	{
	    vim_snprintf((char *)p + len, MAXPATHL - len, "%s", "[+]");
	    len += (int)STRLEN(p + len);
	}
	if (wp->w_buffer->b_p_ro)
	{
	    vim_snprintf((char *)p + len, MAXPATHL - len, "%s", _("[RO]"));
	    len += (int)STRLEN(p + len);
	}

	this_ru_col = ru_col - (Columns - wp->w_width);
	if (this_ru_col < (wp->w_width + 1) / 2)
	    this_ru_col = (wp->w_width + 1) / 2;
	if (this_ru_col <= 1)
	{
	    p = (char_u *)"<";		// No room for file name!
	    len = 1;
	}
	else if (has_mbyte)
	{
	    int	clen = 0, i;

	    // Count total number of display cells.
	    clen = mb_string2cells(p, -1);

	    // Find first character that will fit.
	    // Going from start to end is much faster for DBCS.
	    for (i = 0; p[i] != NUL && clen >= this_ru_col - 1;
		    i += (*mb_ptr2len)(p + i))
		clen -= (*mb_ptr2cells)(p + i);
	    len = clen;
	    if (i > 0)
	    {
		p = p + i - 1;
		*p = '<';
		++len;
	    }

	}
	else if (len > this_ru_col - 1)
	{
	    p += len - (this_ru_col - 1);
	    *p = '<';
	    len = this_ru_col - 1;
	}

	screen_puts(p, row, wp->w_wincol, attr);
	screen_fill(row, row + 1, len + wp->w_wincol,
			this_ru_col + wp->w_wincol, fillchar, fillchar, attr);

	if (get_keymap_str(wp, (char_u *)"<%s>", NameBuff, MAXPATHL)
		&& (this_ru_col - len) > (int)(STRLEN(NameBuff) + 1))
	    screen_puts(NameBuff, row, (int)(this_ru_col - STRLEN(NameBuff)
						   - 1 + wp->w_wincol), attr);

	win_redr_ruler(wp, TRUE, ignore_pum);

	// Draw the 'showcmd' information if 'showcmdloc' == "statusline".
	if (p_sc && *p_sloc == 's')
	{
	    int	width = MIN(10, this_ru_col - len - 2);

	    if (width > 0)
		screen_puts_len(showcmd_buf, width, row,
				wp->w_wincol + this_ru_col - width - 1, attr);
	}
    }

    /*
     * May need to draw the character below the vertical separator.
     */
    if (wp->w_vsep_width != 0 && wp->w_status_height != 0 && redrawing())
    {
	if (stl_connected(wp))
	    fillchar = fillchar_status(&attr, wp);
	else
	    fillchar = fillchar_vsep(&attr, wp);
	screen_putchar(fillchar, row, W_ENDCOL(wp), attr);
    }
    busy = FALSE;
}

#ifdef FEAT_STL_OPT
/*
 * Redraw the status line according to 'statusline' and take care of any
 * errors encountered.
 */
    static void
redraw_custom_statusline(win_T *wp)
{
    static int	    entered = FALSE;

    // When called recursively return.  This can happen when the statusline
    // contains an expression that triggers a redraw.
    if (entered)
	return;
    entered = TRUE;

    win_redr_custom(wp, FALSE);
    entered = FALSE;
}
#endif

/*
 * Show current status info in ruler and various other places
 * If always is FALSE, only show ruler if position has changed.
 */
    void
showruler(int always)
{
    if (!always && !redrawing())
	return;
    if (pum_visible())
    {
	// Don't redraw right now, do it later.
	curwin->w_redr_status = TRUE;
	return;
    }
#if defined(FEAT_STL_OPT)
    if ((*p_stl != NUL || *curwin->w_p_stl != NUL) && curwin->w_status_height)
	redraw_custom_statusline(curwin);
    else
#endif
	win_redr_ruler(curwin, always, FALSE);

    if (need_maketitle
#ifdef FEAT_STL_OPT
	    || (p_icon && (stl_syntax & STL_IN_ICON))
	    || (p_title && (stl_syntax & STL_IN_TITLE))
#endif
       )
	maketitle();

    // Redraw the tab pages line if needed.
    if (redraw_tabline)
	draw_tabline();
}

    void
win_redr_ruler(win_T *wp, int always, int ignore_pum)
{
#define RULER_BUF_LEN 70
    char_u	buffer[RULER_BUF_LEN];
    int		row;
    int		fillchar;
    int		attr;
    int		empty_line = FALSE;
    colnr_T	virtcol;
    int		i;
    size_t	len;
    int		o;
    int		this_ru_col;
    int		off = 0;
    int		width;

    // If 'ruler' off don't do anything
    if (!p_ru)
	return;

    /*
     * Check if cursor.lnum is valid, since win_redr_ruler() may be called
     * after deleting lines, before cursor.lnum is corrected.
     */
    if (wp->w_cursor.lnum > wp->w_buffer->b_ml.ml_line_count)
	return;

    // Don't draw the ruler while doing insert-completion, it might overwrite
    // the (long) mode message.
    if (wp == lastwin && lastwin->w_status_height == 0)
	if (edit_submode != NULL)
	    return;
    // Don't draw the ruler when the popup menu is visible, it may overlap.
    // Except when the popup menu will be redrawn anyway.
    if (!ignore_pum && pum_visible())
	return;

#ifdef FEAT_STL_OPT
    if (*p_ruf)
    {
	win_redr_custom(wp, TRUE);
	return;
    }
#endif

    /*
     * Check if not in Insert mode and the line is empty (will show "0-1").
     */
    if ((State & MODE_INSERT) == 0
		&& *ml_get_buf(wp->w_buffer, wp->w_cursor.lnum, FALSE) == NUL)
	empty_line = TRUE;

    /*
     * Only draw the ruler when something changed.
     */
    validate_virtcol_win(wp);
    if (       redraw_cmdline
	    || always
	    || wp->w_cursor.lnum != wp->w_ru_cursor.lnum
	    || wp->w_cursor.col != wp->w_ru_cursor.col
	    || wp->w_virtcol != wp->w_ru_virtcol
	    || wp->w_cursor.coladd != wp->w_ru_cursor.coladd
	    || wp->w_topline != wp->w_ru_topline
	    || wp->w_buffer->b_ml.ml_line_count != wp->w_ru_line_count
#ifdef FEAT_DIFF
	    || wp->w_topfill != wp->w_ru_topfill
#endif
	    || empty_line != wp->w_ru_empty)
    {
	cursor_off();
	if (wp->w_status_height)
	{
	    row = statusline_row(wp);
	    fillchar = fillchar_status(&attr, wp);
	    off = wp->w_wincol;
	    width = wp->w_width;
	}
	else
	{
	    row = Rows - 1;
	    fillchar = ' ';
	    attr = 0;
	    width = Columns;
	    off = 0;
	}

	// In list mode virtcol needs to be recomputed
	virtcol = wp->w_virtcol;
	if (wp->w_p_list && wp->w_lcs_chars.tab1 == NUL)
	{
	    wp->w_p_list = FALSE;
	    getvvcol(wp, &wp->w_cursor, NULL, &virtcol, NULL);
	    wp->w_p_list = TRUE;
	}

	/*
	 * Some sprintfs return the length, some return a pointer.
	 * To avoid portability problems we use strlen() here.
	 */
	vim_snprintf((char *)buffer, RULER_BUF_LEN, "%ld,",
		(wp->w_buffer->b_ml.ml_flags & ML_EMPTY)
		    ? 0L
		    : (long)(wp->w_cursor.lnum));
	len = STRLEN(buffer);
	col_print(buffer + len, RULER_BUF_LEN - len,
			empty_line ? 0 : (int)wp->w_cursor.col + 1,
			(int)virtcol + 1);

	/*
	 * Add a "50%" if there is room for it.
	 * On the last line, don't print in the last column (scrolls the
	 * screen up on some terminals).
	 */
	i = (int)STRLEN(buffer);
	get_rel_pos(wp, buffer + i + 1, RULER_BUF_LEN - i - 1);
	o = i + vim_strsize(buffer + i + 1);
	if (wp->w_status_height == 0)	// can't use last char of screen
	    ++o;
	this_ru_col = ru_col - (Columns - width);
	if (this_ru_col < 0)
	    this_ru_col = 0;
	// Never use more than half the window/screen width, leave the other
	// half for the filename.
	if (this_ru_col < (width + 1) / 2)
	    this_ru_col = (width + 1) / 2;
	if (this_ru_col + o < width)
	{
	    // need at least 3 chars left for get_rel_pos() + NUL
	    while (this_ru_col + o < width && RULER_BUF_LEN > i + 4)
	    {
		if (has_mbyte)
		    i += (*mb_char2bytes)(fillchar, buffer + i);
		else
		    buffer[i++] = fillchar;
		++o;
	    }
	    get_rel_pos(wp, buffer + i, RULER_BUF_LEN - i);
	}
	// Truncate at window boundary.
	if (has_mbyte)
	{
	    o = 0;
	    for (i = 0; buffer[i] != NUL; i += (*mb_ptr2len)(buffer + i))
	    {
		o += (*mb_ptr2cells)(buffer + i);
		if (this_ru_col + o > width)
		{
		    buffer[i] = NUL;
		    break;
		}
	    }
	}
	else if (this_ru_col + (int)STRLEN(buffer) > width)
	    buffer[width - this_ru_col] = NUL;

	screen_puts(buffer, row, this_ru_col + off, attr);
	i = redraw_cmdline;
	screen_fill(row, row + 1,
		this_ru_col + off + (int)STRLEN(buffer),
		(off + width),
		fillchar, fillchar, attr);
	// don't redraw the cmdline because of showing the ruler
	redraw_cmdline = i;
	wp->w_ru_cursor = wp->w_cursor;
	wp->w_ru_virtcol = wp->w_virtcol;
	wp->w_ru_empty = empty_line;
	wp->w_ru_topline = wp->w_topline;
	wp->w_ru_line_count = wp->w_buffer->b_ml.ml_line_count;
#ifdef FEAT_DIFF
	wp->w_ru_topfill = wp->w_topfill;
#endif
    }
}

/*
 * To be called when "updating_screen" was set before and now the postponed
 * side effects may take place.
 */
    void
after_updating_screen(int may_resize_shell UNUSED)
{
    updating_screen = FALSE;
#ifdef FEAT_GUI
    if (may_resize_shell)
	gui_may_resize_shell();
#endif
#ifdef FEAT_TERMINAL
    term_check_channel_closed_recently();
#endif

#ifdef HAVE_DROP_FILE
    // If handle_drop() was called while updating_screen was TRUE need to
    // handle the drop now.
    handle_any_postponed_drop();
#endif
}

/*
 * Update all windows that are editing the current buffer.
 */
    void
update_curbuf(int type)
{
    redraw_curbuf_later(type);
    update_screen(type);
}

#if defined(FEAT_MENU) || defined(FEAT_FOLDING)
/*
 * Copy "text" to ScreenLines using "attr".
 * Returns the next screen column.
 */
    static int
text_to_screenline(win_T *wp, char_u *text, int col)
{
    int		off = (int)(current_ScreenLine - ScreenLines);

    if (has_mbyte)
    {
	int	cells;
	int	u8c, u8cc[MAX_MCO];
	int	i;
	int	idx;
	int	c_len;
	char_u	*p;
# ifdef FEAT_ARABIC
	int	prev_c = 0;		// previous Arabic character
	int	prev_c1 = 0;		// first composing char for prev_c
# endif

# ifdef FEAT_RIGHTLEFT
	if (wp->w_p_rl)
	    idx = off;
	else
# endif
	    idx = off + col;

	// Store multibyte characters in ScreenLines[] et al. correctly.
	for (p = text; *p != NUL; )
	{
	    cells = (*mb_ptr2cells)(p);
	    c_len = (*mb_ptr2len)(p);
	    if (col + cells > wp->w_width
# ifdef FEAT_RIGHTLEFT
		    - (wp->w_p_rl ? col : 0)
# endif
		    )
		break;
	    ScreenLines[idx] = *p;
	    if (enc_utf8)
	    {
		u8c = utfc_ptr2char(p, u8cc);
		if (*p < 0x80 && u8cc[0] == 0)
		{
		    ScreenLinesUC[idx] = 0;
#ifdef FEAT_ARABIC
		    prev_c = u8c;
#endif
		}
		else
		{
#ifdef FEAT_ARABIC
		    if (p_arshape && !p_tbidi && ARABIC_CHAR(u8c))
		    {
			// Do Arabic shaping.
			int	pc, pc1, nc;
			int	pcc[MAX_MCO];
			int	firstbyte = *p;

			// The idea of what is the previous and next
			// character depends on 'rightleft'.
			if (wp->w_p_rl)
			{
			    pc = prev_c;
			    pc1 = prev_c1;
			    nc = utf_ptr2char(p + c_len);
			    prev_c1 = u8cc[0];
			}
			else
			{
			    pc = utfc_ptr2char(p + c_len, pcc);
			    nc = prev_c;
			    pc1 = pcc[0];
			}
			prev_c = u8c;

			u8c = arabic_shape(u8c, &firstbyte, &u8cc[0],
								 pc, pc1, nc);
			ScreenLines[idx] = firstbyte;
		    }
		    else
			prev_c = u8c;
#endif
		    // Non-BMP character: display as ? or fullwidth ?.
		    ScreenLinesUC[idx] = u8c;
		    for (i = 0; i < Screen_mco; ++i)
		    {
			ScreenLinesC[i][idx] = u8cc[i];
			if (u8cc[i] == 0)
			    break;
		    }
		}
		if (cells > 1)
		    ScreenLines[idx + 1] = 0;
	    }
	    else if (enc_dbcs == DBCS_JPNU && *p == 0x8e)
		// double-byte single width character
		ScreenLines2[idx] = p[1];
	    else if (cells > 1)
		// double-width character
		ScreenLines[idx + 1] = p[1];
	    col += cells;
	    idx += cells;
	    p += c_len;
	}
    }
    else
    {
	int len = (int)STRLEN(text);

	if (len > wp->w_width - col)
	    len = wp->w_width - col;
	if (len > 0)
	{
#ifdef FEAT_RIGHTLEFT
	    if (wp->w_p_rl)
		mch_memmove(current_ScreenLine, text, len);
	    else
#endif
		mch_memmove(current_ScreenLine + col, text, len);
	    col += len;
	}
    }
    return col;
}
#endif

#ifdef FEAT_MENU
/*
 * Draw the window toolbar.
 */
    static void
redraw_win_toolbar(win_T *wp)
{
    vimmenu_T	*menu;
    int		item_idx = 0;
    int		item_count = 0;
    int		col = 0;
    int		next_col;
    int		off = (int)(current_ScreenLine - ScreenLines);
    int		fill_attr = syn_name2attr((char_u *)"ToolbarLine");
    int		button_attr = syn_name2attr((char_u *)"ToolbarButton");

    vim_free(wp->w_winbar_items);
    FOR_ALL_CHILD_MENUS(wp->w_winbar, menu)
	++item_count;
    wp->w_winbar_items = ALLOC_CLEAR_MULT(winbar_item_T, item_count + 1);

    // TODO: use fewer spaces if there is not enough room
    for (menu = wp->w_winbar->children;
			  menu != NULL && col < wp->w_width; menu = menu->next)
    {
	space_to_screenline(off + col, fill_attr);
	if (++col >= wp->w_width)
	    break;
	if (col > 1)
	{
	    space_to_screenline(off + col, fill_attr);
	    if (++col >= wp->w_width)
		break;
	}

	wp->w_winbar_items[item_idx].wb_startcol = col;
	space_to_screenline(off + col, button_attr);
	if (++col >= wp->w_width)
	    break;

	next_col = text_to_screenline(wp, menu->name, col);
	while (col < next_col)
	{
	    ScreenAttrs[off + col] = button_attr;
	    ++col;
	}
	wp->w_winbar_items[item_idx].wb_endcol = col;
	wp->w_winbar_items[item_idx].wb_menu = menu;
	++item_idx;

	if (col >= wp->w_width)
	    break;
	space_to_screenline(off + col, button_attr);
	++col;
    }
    while (col < wp->w_width)
    {
	space_to_screenline(off + col, fill_attr);
	++col;
    }
    wp->w_winbar_items[item_idx].wb_menu = NULL; // end marker

    screen_line(wp, wp->w_winrow, wp->w_wincol, wp->w_width, wp->w_width, 0);
}
#endif

#if defined(FEAT_FOLDING) || defined(PROTO)
/*
 * Copy "buf[len]" to ScreenLines["off"] and set attributes to "attr".
 */
    static void
copy_text_attr(
    int		off,
    char_u	*buf,
    int		len,
    int		attr)
{
    int		i;

    mch_memmove(ScreenLines + off, buf, (size_t)len);
    if (enc_utf8)
	vim_memset(ScreenLinesUC + off, 0, sizeof(u8char_T) * (size_t)len);
    for (i = 0; i < len; ++i)
	ScreenAttrs[off + i] = attr;
}

/*
 * Display one folded line.
 */
    static void
fold_line(
    win_T	*wp,
    long	fold_count,
    foldinfo_T	*foldinfo,
    linenr_T	lnum,
    int		row)
{
    // Max value of 'foldcolumn' is 12 and maximum number of bytes in a
    // multi-byte character is MAX_MCO.
    char_u	buf[MAX_MCO * 12 + 1];
    pos_T	*top, *bot;
    linenr_T	lnume = lnum + fold_count - 1;
    int		len;
    char_u	*text;
    int		fdc;
    int		col;
    int		txtcol;
    int		off = (int)(current_ScreenLine - ScreenLines);
    int		ri;

    // Build the fold line:
    // 1. Add the cmdwin_type for the command-line window
    // 2. Add the 'foldcolumn'
    // 3. Add the 'number' or 'relativenumber' column
    // 4. Compose the text
    // 5. Add the text
    // 6. set highlighting for the Visual area an other text
    col = 0;

    // 1. Add the cmdwin_type for the command-line window
    // Ignores 'rightleft', this window is never right-left.
    if (wp == cmdwin_win)
    {
	ScreenLines[off] = cmdwin_type;
	ScreenAttrs[off] = HL_ATTR(HLF_AT);
	if (enc_utf8)
	    ScreenLinesUC[off] = 0;
	++col;
    }

#ifdef FEAT_RIGHTLEFT
# define RL_MEMSET(p, v, l) \
    do { \
	if (wp->w_p_rl) \
	    for (ri = 0; ri < (l); ++ri) \
	       ScreenAttrs[off + (wp->w_width - (p) - (l)) + ri] = v; \
	 else \
	    for (ri = 0; ri < (l); ++ri) \
	       ScreenAttrs[off + (p) + ri] = v; \
    } while (0)
#else
# define RL_MEMSET(p, v, l) \
    do { \
	for (ri = 0; ri < l; ++ri) \
	    ScreenAttrs[off + (p) + ri] = v; \
    } while (0)
#endif

    // 2. Add the 'foldcolumn'
    //    Reduce the width when there is not enough space.
    fdc = compute_foldcolumn(wp, col);
    if (fdc > 0)
    {
	char_u	*p;
	int	i;
	int	idx;

	fill_foldcolumn(buf, wp, TRUE, lnum);
	p = buf;
	for (i = 0; i < fdc; i++)
	{
	    int		ch;

	    if (has_mbyte)
		ch = mb_ptr2char_adv(&p);
	    else
		ch = *p++;
#ifdef FEAT_RIGHTLEFT
	    if (wp->w_p_rl)
		idx = off + wp->w_width - i - 1 - col;
	    else
#endif
		idx = off + col + i;
	    if (enc_utf8)
	    {
		if (ch >= 0x80)
		{
		    ScreenLinesUC[idx] = ch;
		    ScreenLinesC[0][idx] = 0;
		    ScreenLines[idx] = 0x80;
		}
		else
		{
		    ScreenLines[idx] = ch;
		    ScreenLinesUC[idx] = 0;
		}
	    }
	    else
		ScreenLines[idx] = ch;
	}

	RL_MEMSET(col, HL_ATTR(HLF_FC), fdc);
	col += fdc;
    }

    // Set all attributes of the 'number' or 'relativenumber' column and the
    // text
    RL_MEMSET(col, HL_ATTR(HLF_FL), wp->w_width - col);

#ifdef FEAT_SIGNS
    // If signs are being displayed, add two spaces.
    if (signcolumn_on(wp))
    {
	len = wp->w_width - col;
	if (len > 0)
	{
	    if (len > 2)
		len = 2;
# ifdef FEAT_RIGHTLEFT
	    if (wp->w_p_rl)
		// the line number isn't reversed
		copy_text_attr(off + wp->w_width - len - col,
					(char_u *)"  ", len, HL_ATTR(HLF_FL));
	    else
# endif
		copy_text_attr(off + col, (char_u *)"  ", len, HL_ATTR(HLF_FL));
	    col += len;
	}
    }
#endif

    // 3. Add the 'number' or 'relativenumber' column
    if (wp->w_p_nu || wp->w_p_rnu)
    {
	len = wp->w_width - col;
	if (len > 0)
	{
	    int	    w = number_width(wp);
	    long    num;
	    char    *fmt = "%*ld ";

	    if (len > w + 1)
		len = w + 1;

	    if (wp->w_p_nu && !wp->w_p_rnu)
		// 'number' + 'norelativenumber'
		num = (long)lnum;
	    else
	    {
		// 'relativenumber', don't use negative numbers
		num = labs((long)get_cursor_rel_lnum(wp, lnum));
		if (num == 0 && wp->w_p_nu && wp->w_p_rnu)
		{
		    // 'number' + 'relativenumber': cursor line shows absolute
		    // line number
		    num = lnum;
		    fmt = "%-*ld ";
		}
	    }

	    sprintf((char *)buf, fmt, w, num);
#ifdef FEAT_RIGHTLEFT
	    if (wp->w_p_rl)
		// the line number isn't reversed
		copy_text_attr(off + wp->w_width - len - col, buf, len,
							     HL_ATTR(HLF_FL));
	    else
#endif
		copy_text_attr(off + col, buf, len, HL_ATTR(HLF_FL));
	    col += len;
	}
    }

    // 4. Compose the folded-line string with 'foldtext', if set.
    text = get_foldtext(wp, lnum, lnume, foldinfo, buf);

    txtcol = col;	// remember where text starts

    // 5. move the text to current_ScreenLine.  Fill up with "fold" from
    //    'fillchars'.
    //    Right-left text is put in columns 0 - number-col, normal text is put
    //    in columns number-col - window-width.
    col = text_to_screenline(wp, text, col);

    // Fill the rest of the line with the fold filler
#ifdef FEAT_RIGHTLEFT
    if (wp->w_p_rl)
	col -= txtcol;
#endif
    while (col < wp->w_width
#ifdef FEAT_RIGHTLEFT
		    - (wp->w_p_rl ? txtcol : 0)
#endif
	    )
    {
	int c = wp->w_fill_chars.fold;

	if (enc_utf8)
	{
	    if (c >= 0x80)
	    {
		ScreenLinesUC[off + col] = c;
		ScreenLinesC[0][off + col] = 0;
		ScreenLines[off + col] = 0x80; // avoid storing zero
	    }
	    else
	    {
		ScreenLinesUC[off + col] = 0;
		ScreenLines[off + col] = c;
	    }
	    col++;
	}
	else
	    ScreenLines[off + col++] = c;
    }

    if (text != buf)
	vim_free(text);

    // 6. set highlighting for the Visual area an other text.
    // If all folded lines are in the Visual area, highlight the line.
    if (VIsual_active && wp->w_buffer == curwin->w_buffer)
    {
	if (LTOREQ_POS(curwin->w_cursor, VIsual))
	{
	    // Visual is after curwin->w_cursor
	    top = &curwin->w_cursor;
	    bot = &VIsual;
	}
	else
	{
	    // Visual is before curwin->w_cursor
	    top = &VIsual;
	    bot = &curwin->w_cursor;
	}
	if (lnum >= top->lnum
		&& lnume <= bot->lnum
		&& (VIsual_mode != 'v'
		    || ((lnum > top->lnum
			    || (lnum == top->lnum
				&& top->col == 0))
			&& (lnume < bot->lnum
			    || (lnume == bot->lnum
				&& (bot->col - (*p_sel == 'e'))
		>= (colnr_T)STRLEN(ml_get_buf(wp->w_buffer, lnume, FALSE)))))))
	{
	    if (VIsual_mode == Ctrl_V)
	    {
		// Visual block mode: highlight the chars part of the block
		if (wp->w_old_cursor_fcol + txtcol < (colnr_T)wp->w_width)
		{
		    if (wp->w_old_cursor_lcol != MAXCOL
			     && wp->w_old_cursor_lcol + txtcol
						       < (colnr_T)wp->w_width)
			len = wp->w_old_cursor_lcol;
		    else
			len = wp->w_width - txtcol;
		    RL_MEMSET(wp->w_old_cursor_fcol + txtcol, HL_ATTR(HLF_V),
					    len - (int)wp->w_old_cursor_fcol);
		}
	    }
	    else
	    {
		// Set all attributes of the text
		RL_MEMSET(txtcol, HL_ATTR(HLF_V), wp->w_width - txtcol);
	    }
	}
    }

#ifdef FEAT_SYN_HL
    // Show colorcolumn in the fold line, but let cursorcolumn override it.
    if (wp->w_p_cc_cols)
    {
	int i = 0;
	int j = wp->w_p_cc_cols[i];
	int old_txtcol = txtcol;

	while (j > -1)
	{
	    txtcol += j;
	    if (wp->w_p_wrap)
		txtcol -= wp->w_skipcol;
	    else
		txtcol -= wp->w_leftcol;
	    if (txtcol >= 0 && txtcol < wp->w_width)
		ScreenAttrs[off + txtcol] = hl_combine_attr(
				    ScreenAttrs[off + txtcol], HL_ATTR(HLF_MC));
	    txtcol = old_txtcol;
	    j = wp->w_p_cc_cols[++i];
	}
    }

    // Show 'cursorcolumn' in the fold line.
    if (wp->w_p_cuc)
    {
	txtcol += wp->w_virtcol;
	if (wp->w_p_wrap)
	    txtcol -= wp->w_skipcol;
	else
	    txtcol -= wp->w_leftcol;
	if (txtcol >= 0 && txtcol < wp->w_width)
	    ScreenAttrs[off + txtcol] = hl_combine_attr(
				 ScreenAttrs[off + txtcol], HL_ATTR(HLF_CUC));
    }
#endif

    screen_line(wp, row + W_WINROW(wp), wp->w_wincol,
						  wp->w_width, wp->w_width, 0);

    // Update w_cline_height and w_cline_folded if the cursor line was
    // updated (saves a call to plines() later).
    if (wp == curwin
	    && lnum <= curwin->w_cursor.lnum
	    && lnume >= curwin->w_cursor.lnum)
    {
	curwin->w_cline_row = row;
	curwin->w_cline_height = 1;
	curwin->w_cline_folded = TRUE;
	curwin->w_valid |= (VALID_CHEIGHT|VALID_CROW);
    }

# ifdef FEAT_CONCEAL
    // When the line was not folded w_wrow may have been set, recompute it.
    if (wp == curwin
	    && wp->w_cursor.lnum >= lnum
	    && wp->w_cursor.lnum <= lnume
	    && conceal_cursor_line(wp))
	curs_columns(TRUE);
# endif
}
#endif

/*
 * Update a single window.
 *
 * This may cause the windows below it also to be redrawn (when clearing the
 * screen or scrolling lines).
 *
 * How the window is redrawn depends on wp->w_redr_type.  Each type also
 * implies the one below it.
 * UPD_NOT_VALID	redraw the whole window
 * UPD_SOME_VALID	redraw the whole window but do scroll when possible
 * UPD_REDRAW_TOP	redraw the top w_upd_rows window lines, otherwise like
 *			UPD_VALID
 * UPD_INVERTED		redraw the changed part of the Visual area
 * UPD_INVERTED_ALL	redraw the whole Visual area
 * UPD_VALID	1. scroll up/down to adjust for a changed w_topline
 *		2. update lines at the top when scrolled down
 *		3. redraw changed text:
 *		   - if wp->w_buffer->b_mod_set set, update lines between
 *		     b_mod_top and b_mod_bot.
 *		   - if wp->w_redraw_top non-zero, redraw lines between
 *		     wp->w_redraw_top and wp->w_redr_bot.
 *		   - continue redrawing when syntax status is invalid.
 *		4. if scrolled up, update lines at the bottom.
 * This results in three areas that may need updating:
 * top:	from first row to top_end (when scrolled down)
 * mid: from mid_start to mid_end (update inversion or changed text)
 * bot: from bot_start to last row (when scrolled up)
 */
    static void
win_update(win_T *wp)
{
    buf_T	*buf = wp->w_buffer;
    int		type;
    int		top_end = 0;	// Below last row of the top area that needs
				// updating.  0 when no top area updating.
    int		mid_start = 999;// first row of the mid area that needs
				// updating.  999 when no mid area updating.
    int		mid_end = 0;	// Below last row of the mid area that needs
				// updating.  0 when no mid area updating.
    int		bot_start = 999;// first row of the bot area that needs
				// updating.  999 when no bot area updating
    int		scrolled_down = FALSE;	// TRUE when scrolled down when
					// w_topline got smaller a bit
#ifdef FEAT_SEARCH_EXTRA
    int		top_to_mod = FALSE;    // redraw above mod_top
#endif

    int		row;		// current window row to display
    linenr_T	lnum;		// current buffer lnum to display
    int		idx;		// current index in w_lines[]
    int		srow;		// starting row of the current line

    int		eof = FALSE;	// if TRUE, we hit the end of the file
    int		didline = FALSE; // if TRUE, we finished the last line
    int		i;
    long	j;
    static int	recursive = FALSE;	// being called recursively
    linenr_T	old_botline = wp->w_botline;
#ifdef FEAT_CONCEAL
    int		old_wrow = wp->w_wrow;
    int		old_wcol = wp->w_wcol;
#endif
#ifdef FEAT_FOLDING
    long	fold_count;
#endif
#ifdef FEAT_SYN_HL
    // remember what happened to the previous line, to know if
    // check_visual_highlight() can be used
# define DID_NONE 1	// didn't update a line
# define DID_LINE 2	// updated a normal line
# define DID_FOLD 3	// updated a folded line
    int		did_update = DID_NONE;
    linenr_T	syntax_last_parsed = 0;		// last parsed text line
#endif
    linenr_T	mod_top = 0;
    linenr_T	mod_bot = 0;
#if defined(FEAT_SYN_HL) || defined(FEAT_SEARCH_EXTRA)
    int		save_got_int;
#endif

#if defined(FEAT_SEARCH_EXTRA) || defined(FEAT_CLIPBOARD)
    // This needs to be done only for the first window when update_screen() is
    // called.
    if (!did_update_one_window)
    {
	did_update_one_window = TRUE;
# ifdef FEAT_SEARCH_EXTRA
	start_search_hl();
# endif
# ifdef FEAT_CLIPBOARD
	// When Visual area changed, may have to update selection.
	if (clip_star.available && clip_isautosel_star())
	    clip_update_selection(&clip_star);
	if (clip_plus.available && clip_isautosel_plus())
	    clip_update_selection(&clip_plus);
# endif
    }
#endif

    type = wp->w_redr_type;

    if (type == UPD_NOT_VALID)
    {
	wp->w_redr_status = TRUE;
	wp->w_lines_valid = 0;
    }

    // Window frame is zero-height: nothing to draw.
    if (wp->w_height + WINBAR_HEIGHT(wp) == 0
	    || (wp->w_frame->fr_height == wp->w_status_height
#if defined(FEAT_PROP_POPUP)
		&& !popup_is_popup(wp)
#endif
	       ))
    {
	wp->w_redr_type = 0;
	return;
    }

    // Window is zero-width: Only need to draw the separator.
    if (wp->w_width == 0)
    {
	// draw the vertical separator right of this window
	draw_vsep_win(wp, 0);
	wp->w_redr_type = 0;
	return;
    }

#ifdef FEAT_TERMINAL
    // If this window contains a terminal, redraw works completely differently.
    if (term_do_update_window(wp))
    {
	term_update_window(wp);
# ifdef FEAT_MENU
	// Draw the window toolbar, if there is one.
	if (winbar_height(wp) > 0)
	    redraw_win_toolbar(wp);
# endif
	wp->w_redr_type = 0;
	return;
    }
#endif

#ifdef FEAT_SEARCH_EXTRA
    init_search_hl(wp, &screen_search_hl);
#endif

    // Make sure skipcol is valid, it depends on various options and the window
    // width.
    if (wp->w_skipcol > 0)
    {
	int w = 0;
	int width1 = wp->w_width - win_col_off(wp);
	int width2 = width1 + win_col_off2(wp);
	int add = width1;

	while (w < wp->w_skipcol)
	{
	    if (w > 0)
		add = width2;
	    w += add;
	}
	if (w != wp->w_skipcol)
	    // always round down, the higher value may not be valid
	    wp->w_skipcol = w - add;
    }

#ifdef FEAT_LINEBREAK
    // Force redraw when width of 'number' or 'relativenumber' column
    // changes.
    i = (wp->w_p_nu || wp->w_p_rnu) ? number_width(wp) : 0;
    if (wp->w_nrwidth != i)
    {
	type = UPD_NOT_VALID;
	wp->w_nrwidth = i;
    }
    else
#endif

    if (buf->b_mod_set && buf->b_mod_xlines != 0 && wp->w_redraw_top != 0)
    {
	// When there are both inserted/deleted lines and specific lines to be
	// redrawn, w_redraw_top and w_redraw_bot may be invalid, just redraw
	// everything (only happens when redrawing is off for while).
	type = UPD_NOT_VALID;
    }
    else
    {
	// Set mod_top to the first line that needs displaying because of
	// changes.  Set mod_bot to the first line after the changes.
	mod_top = wp->w_redraw_top;
	if (wp->w_redraw_bot != 0)
	    mod_bot = wp->w_redraw_bot + 1;
	else
	    mod_bot = 0;
	if (buf->b_mod_set)
	{
	    if (mod_top == 0 || mod_top > buf->b_mod_top)
	    {
		mod_top = buf->b_mod_top;
#ifdef FEAT_SYN_HL
		// Need to redraw lines above the change that may be included
		// in a pattern match.
		if (syntax_present(wp))
		{
		    mod_top -= buf->b_s.b_syn_sync_linebreaks;
		    if (mod_top < 1)
			mod_top = 1;
		}
#endif
	    }
	    if (mod_bot == 0 || mod_bot < buf->b_mod_bot)
		mod_bot = buf->b_mod_bot;

#ifdef FEAT_SEARCH_EXTRA
	    // When 'hlsearch' is on and using a multi-line search pattern, a
	    // change in one line may make the Search highlighting in a
	    // previous line invalid.  Simple solution: redraw all visible
	    // lines above the change.
	    // Same for a match pattern.
	    if (screen_search_hl.rm.regprog != NULL
		    && re_multiline(screen_search_hl.rm.regprog))
		top_to_mod = TRUE;
	    else
	    {
		matchitem_T *cur = wp->w_match_head;

		while (cur != NULL)
		{
		    if (cur->mit_match.regprog != NULL
				       && re_multiline(cur->mit_match.regprog))
		    {
			top_to_mod = TRUE;
			break;
		    }
		    cur = cur->mit_next;
		}
	    }
#endif
	}

#ifdef FEAT_SEARCH_EXTRA
	if (search_hl_has_cursor_lnum > 0)
	{
	    // CurSearch was used last time, need to redraw the line with it to
	    // avoid having two matches highlighted with CurSearch.
	    if (mod_top == 0 || mod_top > search_hl_has_cursor_lnum)
		mod_top = search_hl_has_cursor_lnum;
	    if (mod_bot == 0 || mod_bot < search_hl_has_cursor_lnum + 1)
		mod_bot = search_hl_has_cursor_lnum + 1;
	}
#endif

#ifdef FEAT_FOLDING
	if (mod_top != 0 && hasAnyFolding(wp))
	{
	    linenr_T	lnumt, lnumb;

	    // A change in a line can cause lines above it to become folded or
	    // unfolded.  Find the top most buffer line that may be affected.
	    // If the line was previously folded and displayed, get the first
	    // line of that fold.  If the line is folded now, get the first
	    // folded line.  Use the minimum of these two.

	    // Find last valid w_lines[] entry above mod_top.  Set lnumt to
	    // the line below it.  If there is no valid entry, use w_topline.
	    // Find the first valid w_lines[] entry below mod_bot.  Set lnumb
	    // to this line.  If there is no valid entry, use MAXLNUM.
	    lnumt = wp->w_topline;
	    lnumb = MAXLNUM;
	    for (i = 0; i < wp->w_lines_valid; ++i)
		if (wp->w_lines[i].wl_valid)
		{
		    if (wp->w_lines[i].wl_lastlnum < mod_top)
			lnumt = wp->w_lines[i].wl_lastlnum + 1;
		    if (lnumb == MAXLNUM && wp->w_lines[i].wl_lnum >= mod_bot)
		    {
			lnumb = wp->w_lines[i].wl_lnum;
			// When there is a fold column it might need updating
			// in the next line ("J" just above an open fold).
			if (compute_foldcolumn(wp, 0) > 0)
			    ++lnumb;
		    }
		}

	    (void)hasFoldingWin(wp, mod_top, &mod_top, NULL, TRUE, NULL);
	    if (mod_top > lnumt)
		mod_top = lnumt;

	    // Now do the same for the bottom line (one above mod_bot).
	    --mod_bot;
	    (void)hasFoldingWin(wp, mod_bot, NULL, &mod_bot, TRUE, NULL);
	    ++mod_bot;
	    if (mod_bot < lnumb)
		mod_bot = lnumb;
	}
#endif

	// When a change starts above w_topline and the end is below
	// w_topline, start redrawing at w_topline.
	// If the end of the change is above w_topline: do like no change was
	// made, but redraw the first line to find changes in syntax.
	if (mod_top != 0 && mod_top < wp->w_topline)
	{
	    if (mod_bot > wp->w_topline)
		mod_top = wp->w_topline;
#ifdef FEAT_SYN_HL
	    else if (syntax_present(wp))
		top_end = 1;
#endif
	}

	// When line numbers are displayed need to redraw all lines below
	// inserted/deleted lines.
	if (mod_top != 0 && buf->b_mod_xlines != 0 && wp->w_p_nu)
	    mod_bot = MAXLNUM;
    }
    wp->w_redraw_top = 0;	// reset for next time
    wp->w_redraw_bot = 0;
#ifdef FEAT_SEARCH_EXTRA
    search_hl_has_cursor_lnum = 0;
#endif


    // When only displaying the lines at the top, set top_end.  Used when
    // window has scrolled down for msg_scrolled.
    if (type == UPD_REDRAW_TOP)
    {
	j = 0;
	for (i = 0; i < wp->w_lines_valid; ++i)
	{
	    j += wp->w_lines[i].wl_size;
	    if (j >= wp->w_upd_rows)
	    {
		top_end = j;
		break;
	    }
	}
	if (top_end == 0)
	    // not found (cannot happen?): redraw everything
	    type = UPD_NOT_VALID;
	else
	    // top area defined, the rest is UPD_VALID
	    type = UPD_VALID;
    }

    // Trick: we want to avoid clearing the screen twice.  screenclear() will
    // set "screen_cleared" to TRUE.  The special value MAYBE (which is still
    // non-zero and thus not FALSE) will indicate that screenclear() was not
    // called.
    if (screen_cleared)
	screen_cleared = MAYBE;

    // If there are no changes on the screen that require a complete redraw,
    // handle three cases:
    // 1: we are off the top of the screen by a few lines: scroll down
    // 2: wp->w_topline is below wp->w_lines[0].wl_lnum: may scroll up
    // 3: wp->w_topline is wp->w_lines[0].wl_lnum: find first entry in
    //    w_lines[] that needs updating.
    if ((type == UPD_VALID || type == UPD_SOME_VALID
			   || type == UPD_INVERTED || type == UPD_INVERTED_ALL)
#ifdef FEAT_DIFF
	    && !wp->w_botfill && !wp->w_old_botfill
#endif
	    )
    {
	if (mod_top != 0
		&& wp->w_topline == mod_top
		&& (!wp->w_lines[0].wl_valid
		    || wp->w_topline == wp->w_lines[0].wl_lnum))
	{
	    // w_topline is the first changed line and window is not scrolled,
	    // the scrolling from changed lines will be done further down.
	}
	else if (wp->w_lines[0].wl_valid
		&& (wp->w_topline < wp->w_lines[0].wl_lnum
#ifdef FEAT_DIFF
		    || (wp->w_topline == wp->w_lines[0].wl_lnum
			&& wp->w_topfill > wp->w_old_topfill)
#endif
		   ))
	{
	    // New topline is above old topline: May scroll down.
#ifdef FEAT_FOLDING
	    if (hasAnyFolding(wp))
	    {
		linenr_T ln;

		// count the number of lines we are off, counting a sequence
		// of folded lines as one
		j = 0;
		for (ln = wp->w_topline; ln < wp->w_lines[0].wl_lnum; ++ln)
		{
		    ++j;
		    if (j >= wp->w_height - 2)
			break;
		    (void)hasFoldingWin(wp, ln, NULL, &ln, TRUE, NULL);
		}
	    }
	    else
#endif
		j = wp->w_lines[0].wl_lnum - wp->w_topline;
	    if (j < wp->w_height - 2)		// not too far off
	    {
		i = plines_m_win(wp, wp->w_topline, wp->w_lines[0].wl_lnum - 1,
									 TRUE);
#ifdef FEAT_DIFF
		// insert extra lines for previously invisible filler lines
		if (wp->w_lines[0].wl_lnum != wp->w_topline)
		    i += diff_check_fill(wp, wp->w_lines[0].wl_lnum)
							  - wp->w_old_topfill;
#endif
		if (i < wp->w_height - 2)	// less than a screen off
		{
		    // Try to insert the correct number of lines.
		    // If not the last window, delete the lines at the bottom.
		    // win_ins_lines may fail when the terminal can't do it.
		    if (i > 0)
			check_for_delay(FALSE);
		    if (win_ins_lines(wp, 0, i, FALSE, wp == firstwin) == OK)
		    {
			if (wp->w_lines_valid != 0)
			{
			    // Need to update rows that are new, stop at the
			    // first one that scrolled down.
			    top_end = i;
			    scrolled_down = TRUE;

			    // Move the entries that were scrolled, disable
			    // the entries for the lines to be redrawn.
			    if ((wp->w_lines_valid += j) > wp->w_height)
				wp->w_lines_valid = wp->w_height;
			    for (idx = wp->w_lines_valid; idx - j >= 0; idx--)
				wp->w_lines[idx] = wp->w_lines[idx - j];
			    while (idx >= 0)
				wp->w_lines[idx--].wl_valid = FALSE;
			}
		    }
		    else
			mid_start = 0;		// redraw all lines
		}
		else
		    mid_start = 0;		// redraw all lines
	    }
	    else
		mid_start = 0;		// redraw all lines
	}
	else
	{
	    // New topline is at or below old topline: May scroll up.
	    // When topline didn't change, find first entry in w_lines[] that
	    // needs updating.

	    // Try to find wp->w_topline in wp->w_lines[].wl_lnum.  The check
	    // for "Rows" is in case "wl_size" is incorrect somehow.
	    j = -1;
	    row = 0;
	    for (i = 0; i < wp->w_lines_valid && i < Rows; i++)
	    {
		if (wp->w_lines[i].wl_valid
			&& wp->w_lines[i].wl_lnum == wp->w_topline)
		{
		    j = i;
		    break;
		}
		row += wp->w_lines[i].wl_size;
	    }
	    if (j == -1)
	    {
		// if wp->w_topline is not in wp->w_lines[].wl_lnum redraw all
		// lines
		mid_start = 0;
	    }
	    else
	    {
		// Try to delete the correct number of lines.
		// wp->w_topline is at wp->w_lines[i].wl_lnum.
#ifdef FEAT_DIFF
		// If the topline didn't change, delete old filler lines,
		// otherwise delete filler lines of the new topline...
		if (wp->w_lines[0].wl_lnum == wp->w_topline)
		    row += wp->w_old_topfill;
		else
		    row += diff_check_fill(wp, wp->w_topline);
		// ... but don't delete new filler lines.
		row -= wp->w_topfill;
#endif
		if (row > Rows)  // just in case
		    row = Rows;
		if (row > 0)
		{
		    check_for_delay(FALSE);
		    if (win_del_lines(wp, 0, row, FALSE, wp == firstwin, 0)
									 == OK)
			bot_start = wp->w_height - row;
		    else
			mid_start = 0;		// redraw all lines
		}
		if ((row == 0 || bot_start < 999) && wp->w_lines_valid != 0)
		{
		    // Skip the lines (below the deleted lines) that are still
		    // valid and don't need redrawing.	Copy their info
		    // upwards, to compensate for the deleted lines.  Set
		    // bot_start to the first row that needs redrawing.
		    bot_start = 0;
		    idx = 0;
		    for (;;)
		    {
			wp->w_lines[idx] = wp->w_lines[j];
			// stop at line that didn't fit, unless it is still
			// valid (no lines deleted)
			if (row > 0 && bot_start + row
				 + (int)wp->w_lines[j].wl_size > wp->w_height)
			{
			    wp->w_lines_valid = idx + 1;
			    break;
			}
			bot_start += wp->w_lines[idx++].wl_size;

			// stop at the last valid entry in w_lines[].wl_size
			if (++j >= wp->w_lines_valid)
			{
			    wp->w_lines_valid = idx;
			    break;
			}
		    }
#ifdef FEAT_DIFF
		    // Correct the first entry for filler lines at the top
		    // when it won't get updated below.
		    if (wp->w_p_diff && bot_start > 0)
			wp->w_lines[0].wl_size =
			    plines_win_nofill(wp, wp->w_topline, TRUE)
							      + wp->w_topfill;
#endif
		}
	    }
	}

	// When starting redraw in the first line, redraw all lines.
	if (mid_start == 0)
	    mid_end = wp->w_height;

	// When win_del_lines() or win_ins_lines() caused the screen to be
	// cleared (only happens for the first window) or when screenclear()
	// was called directly above, "must_redraw" will have been set to
	// UPD_NOT_VALID, need to reset it here to avoid redrawing twice.
	if (screen_cleared == TRUE)
	    must_redraw = 0;
    }
    else
    {
	// Not UPD_VALID or UPD_INVERTED: redraw all lines.
	mid_start = 0;
	mid_end = wp->w_height;
    }

    if (type == UPD_SOME_VALID)
    {
	// UPD_SOME_VALID: redraw all lines.
	mid_start = 0;
	mid_end = wp->w_height;
	type = UPD_NOT_VALID;
    }

    // check if we are updating or removing the inverted part
    if ((VIsual_active && buf == curwin->w_buffer)
	    || (wp->w_old_cursor_lnum != 0 && type != UPD_NOT_VALID))
    {
	linenr_T    from, to;

	if (VIsual_active)
	{
	    if (VIsual_mode != wp->w_old_visual_mode
		|| type == UPD_INVERTED_ALL)
	    {
		// If the type of Visual selection changed, redraw the whole
		// selection.  Also when the ownership of the X selection is
		// gained or lost.
		if (curwin->w_cursor.lnum < VIsual.lnum)
		{
		    from = curwin->w_cursor.lnum;
		    to = VIsual.lnum;
		}
		else
		{
		    from = VIsual.lnum;
		    to = curwin->w_cursor.lnum;
		}
		// redraw more when the cursor moved as well
		if (wp->w_old_cursor_lnum < from)
		    from = wp->w_old_cursor_lnum;
		if (wp->w_old_cursor_lnum > to)
		    to = wp->w_old_cursor_lnum;
		if (wp->w_old_visual_lnum < from)
		    from = wp->w_old_visual_lnum;
		if (wp->w_old_visual_lnum > to)
		    to = wp->w_old_visual_lnum;
	    }
	    else
	    {
		// Find the line numbers that need to be updated: The lines
		// between the old cursor position and the current cursor
		// position.  Also check if the Visual position changed.
		if (curwin->w_cursor.lnum < wp->w_old_cursor_lnum)
		{
		    from = curwin->w_cursor.lnum;
		    to = wp->w_old_cursor_lnum;
		}
		else
		{
		    from = wp->w_old_cursor_lnum;
		    to = curwin->w_cursor.lnum;
		    if (from == 0)	// Visual mode just started
			from = to;
		}

		if (VIsual.lnum != wp->w_old_visual_lnum
					|| VIsual.col != wp->w_old_visual_col)
		{
		    if (wp->w_old_visual_lnum < from
						&& wp->w_old_visual_lnum != 0)
			from = wp->w_old_visual_lnum;
		    if (wp->w_old_visual_lnum > to)
			to = wp->w_old_visual_lnum;
		    if (VIsual.lnum < from)
			from = VIsual.lnum;
		    if (VIsual.lnum > to)
			to = VIsual.lnum;
		}
	    }

	    // If in block mode and changed column or curwin->w_curswant:
	    // update all lines.
	    // First compute the actual start and end column.
	    if (VIsual_mode == Ctrl_V)
	    {
		colnr_T	    fromc, toc;
#if defined(FEAT_LINEBREAK)
		int	    save_ve_flags = curwin->w_ve_flags;

		if (curwin->w_p_lbr)
		    curwin->w_ve_flags = VE_ALL;
#endif
		getvcols(wp, &VIsual, &curwin->w_cursor, &fromc, &toc);
		++toc;
#if defined(FEAT_LINEBREAK)
		curwin->w_ve_flags = save_ve_flags;
#endif
		// Highlight to the end of the line, unless 'virtualedit' has
		// "block".
		if (curwin->w_curswant == MAXCOL)
		{
		    if (get_ve_flags() & VE_BLOCK)
		    {
			pos_T	    pos;
			int	    cursor_above =
					   curwin->w_cursor.lnum < VIsual.lnum;

			// Need to find the longest line.
			toc = 0;
			pos.coladd = 0;
			for (pos.lnum = curwin->w_cursor.lnum; cursor_above
					? pos.lnum <= VIsual.lnum
					: pos.lnum >= VIsual.lnum;
					     pos.lnum += cursor_above ? 1 : -1)
			{
			    colnr_T t;

			    pos.col = (int)STRLEN(ml_get_buf(wp->w_buffer,
							     pos.lnum, FALSE));
			    getvvcol(wp, &pos, NULL, NULL, &t);
			    if (toc < t)
				toc = t;
			}
			++toc;
		    }
		    else
			toc = MAXCOL;
		}

		if (fromc != wp->w_old_cursor_fcol
			|| toc != wp->w_old_cursor_lcol)
		{
		    if (from > VIsual.lnum)
			from = VIsual.lnum;
		    if (to < VIsual.lnum)
			to = VIsual.lnum;
		}
		wp->w_old_cursor_fcol = fromc;
		wp->w_old_cursor_lcol = toc;
	    }
	}
	else
	{
	    // Use the line numbers of the old Visual area.
	    if (wp->w_old_cursor_lnum < wp->w_old_visual_lnum)
	    {
		from = wp->w_old_cursor_lnum;
		to = wp->w_old_visual_lnum;
	    }
	    else
	    {
		from = wp->w_old_visual_lnum;
		to = wp->w_old_cursor_lnum;
	    }
	}

	// There is no need to update lines above the top of the window.
	if (from < wp->w_topline)
	    from = wp->w_topline;

	// If we know the value of w_botline, use it to restrict the update to
	// the lines that are visible in the window.
	if (wp->w_valid & VALID_BOTLINE)
	{
	    if (from >= wp->w_botline)
		from = wp->w_botline - 1;
	    if (to >= wp->w_botline)
		to = wp->w_botline - 1;
	}

	// Find the minimal part to be updated.
	// Watch out for scrolling that made entries in w_lines[] invalid.
	// E.g., CTRL-U makes the first half of w_lines[] invalid and sets
	// top_end; need to redraw from top_end to the "to" line.
	// A middle mouse click with a Visual selection may change the text
	// above the Visual area and reset wl_valid, do count these for
	// mid_end (in srow).
	if (mid_start > 0)
	{
	    lnum = wp->w_topline;
	    idx = 0;
	    srow = 0;
	    if (scrolled_down)
		mid_start = top_end;
	    else
		mid_start = 0;
	    while (lnum < from && idx < wp->w_lines_valid)	// find start
	    {
		if (wp->w_lines[idx].wl_valid)
		    mid_start += wp->w_lines[idx].wl_size;
		else if (!scrolled_down)
		    srow += wp->w_lines[idx].wl_size;
		++idx;
# ifdef FEAT_FOLDING
		if (idx < wp->w_lines_valid && wp->w_lines[idx].wl_valid)
		    lnum = wp->w_lines[idx].wl_lnum;
		else
# endif
		    ++lnum;
	    }
	    srow += mid_start;
	    mid_end = wp->w_height;
	    for ( ; idx < wp->w_lines_valid; ++idx)		// find end
	    {
		if (wp->w_lines[idx].wl_valid
			&& wp->w_lines[idx].wl_lnum >= to + 1)
		{
		    // Only update until first row of this line
		    mid_end = srow;
		    break;
		}
		srow += wp->w_lines[idx].wl_size;
	    }
	}
    }

    if (VIsual_active && buf == curwin->w_buffer)
    {
	wp->w_old_visual_mode = VIsual_mode;
	wp->w_old_cursor_lnum = curwin->w_cursor.lnum;
	wp->w_old_visual_lnum = VIsual.lnum;
	wp->w_old_visual_col = VIsual.col;
	wp->w_old_curswant = curwin->w_curswant;
    }
    else
    {
	wp->w_old_visual_mode = 0;
	wp->w_old_cursor_lnum = 0;
	wp->w_old_visual_lnum = 0;
	wp->w_old_visual_col = 0;
    }

#if defined(FEAT_SYN_HL) || defined(FEAT_SEARCH_EXTRA)
    // reset got_int, otherwise regexp won't work
    save_got_int = got_int;
    got_int = 0;
#endif
#ifdef SYN_TIME_LIMIT
    // Set the time limit to 'redrawtime'.
    redrawtime_limit_set = TRUE;
    init_regexp_timeout(p_rdt);
#endif
#ifdef FEAT_FOLDING
    win_foldinfo.fi_level = 0;
#endif

#ifdef FEAT_MENU
    // Draw the window toolbar, if there is one.
    // TODO: only when needed.
    if (winbar_height(wp) > 0)
	redraw_win_toolbar(wp);
#endif

    lnum = wp->w_topline;   // first line shown in window

    spellvars_T spv;
#ifdef FEAT_SPELL
    // Initialize spell related variables for the first drawn line.
    CLEAR_FIELD(spv);
    if (spell_check_window(wp))
    {
	spv.spv_has_spell = TRUE;
	spv.spv_unchanged = mod_top == 0;
    }
#endif

    // Update all the window rows.
    idx = 0;		// first entry in w_lines[].wl_size
    row = 0;
    srow = 0;
    for (;;)
    {
	// stop updating when reached the end of the window (check for _past_
	// the end of the window is at the end of the loop)
	if (row == wp->w_height)
	{
	    didline = TRUE;
	    break;
	}

	// stop updating when hit the end of the file
	if (lnum > buf->b_ml.ml_line_count)
	{
	    eof = TRUE;
	    break;
	}

	// Remember the starting row of the line that is going to be dealt
	// with.  It is used further down when the line doesn't fit.
	srow = row;

	// Update a line when it is in an area that needs updating, when it
	// has changes or w_lines[idx] is invalid.
	// "bot_start" may be halfway a wrapped line after using
	// win_del_lines(), check if the current line includes it.
	// When syntax folding is being used, the saved syntax states will
	// already have been updated, we can't see where the syntax state is
	// the same again, just update until the end of the window.
	if (row < top_end
		|| (row >= mid_start && row < mid_end)
#ifdef FEAT_SEARCH_EXTRA
		|| top_to_mod
#endif
		|| idx >= wp->w_lines_valid
		|| (row + wp->w_lines[idx].wl_size > bot_start)
		|| (mod_top != 0
		    && (lnum == mod_top
			|| (lnum >= mod_top
			    && (lnum < mod_bot
#ifdef FEAT_SYN_HL
				|| did_update == DID_FOLD
				|| (did_update == DID_LINE
				    && syntax_present(wp)
				    && (
# ifdef FEAT_FOLDING
					(foldmethodIsSyntax(wp)
						      && hasAnyFolding(wp)) ||
# endif
					syntax_check_changed(lnum)))
#endif
#ifdef FEAT_SEARCH_EXTRA
				// match in fixed position might need redraw
				// if lines were inserted or deleted
				|| (wp->w_match_head != NULL
						    && buf->b_mod_xlines != 0)
#endif
				))))
#ifdef FEAT_SYN_HL
		|| (wp->w_p_cul && lnum == wp->w_cursor.lnum)
		|| lnum == wp->w_last_cursorline
#endif
				)
	{
#ifdef FEAT_SEARCH_EXTRA
	    if (lnum == mod_top)
		top_to_mod = FALSE;
#endif

	    // When at start of changed lines: May scroll following lines
	    // up or down to minimize redrawing.
	    // Don't do this when the change continues until the end.
	    // Don't scroll when dollar_vcol >= 0, keep the "$".
	    // Don't scroll when redrawing the top, scrolled already above.
	    if (lnum == mod_top
		    && mod_bot != MAXLNUM
		    && !(dollar_vcol >= 0 && mod_bot == mod_top + 1)
		    && row >= top_end)
	    {
		int		old_rows = 0;
		int		new_rows = 0;
		int		xtra_rows;
		linenr_T	l;

		// Count the old number of window rows, using w_lines[], which
		// should still contain the sizes for the lines as they are
		// currently displayed.
		for (i = idx; i < wp->w_lines_valid; ++i)
		{
		    // Only valid lines have a meaningful wl_lnum.  Invalid
		    // lines are part of the changed area.
		    if (wp->w_lines[i].wl_valid
			    && wp->w_lines[i].wl_lnum == mod_bot)
			break;
		    old_rows += wp->w_lines[i].wl_size;
#ifdef FEAT_FOLDING
		    if (wp->w_lines[i].wl_valid
			    && wp->w_lines[i].wl_lastlnum + 1 == mod_bot)
		    {
			// Must have found the last valid entry above mod_bot.
			// Add following invalid entries.
			++i;
			while (i < wp->w_lines_valid
						  && !wp->w_lines[i].wl_valid)
			    old_rows += wp->w_lines[i++].wl_size;
			break;
		    }
#endif
		}

		if (i >= wp->w_lines_valid)
		{
		    // We can't find a valid line below the changed lines,
		    // need to redraw until the end of the window.
		    // Inserting/deleting lines has no use.
		    bot_start = 0;
		}
		else
		{
		    // Able to count old number of rows: Count new window
		    // rows, and may insert/delete lines
		    j = idx;
		    for (l = lnum; l < mod_bot; ++l)
		    {
#ifdef FEAT_FOLDING
			if (hasFoldingWin(wp, l, NULL, &l, TRUE, NULL))
			    ++new_rows;
			else
#endif
			{
#ifdef FEAT_DIFF
			    if (l == wp->w_topline)
			    {
				int n = plines_win_nofill(wp, l, FALSE)
								+ wp->w_topfill;
				n -= adjust_plines_for_skipcol(wp);
				if (n > wp->w_height)
				    n = wp->w_height;
				new_rows += n;
			    }
			    else
#endif
				new_rows += plines_win(wp, l, TRUE);
			}
			++j;
			if (new_rows > wp->w_height - row - 2)
			{
			    // it's getting too much, must redraw the rest
			    new_rows = 9999;
			    break;
			}
		    }
		    xtra_rows = new_rows - old_rows;
		    if (xtra_rows < 0)
		    {
			// May scroll text up.  If there is not enough
			// remaining text or scrolling fails, must redraw the
			// rest.  If scrolling works, must redraw the text
			// below the scrolled text.
			if (row - xtra_rows >= wp->w_height - 2)
			    mod_bot = MAXLNUM;
			else
			{
			    check_for_delay(FALSE);
			    if (win_del_lines(wp, row,
					  -xtra_rows, FALSE, FALSE, 0) == FAIL)
				mod_bot = MAXLNUM;
			    else
				bot_start = wp->w_height + xtra_rows;
			}
		    }
		    else if (xtra_rows > 0)
		    {
			// May scroll text down.  If there is not enough
			// remaining text of scrolling fails, must redraw the
			// rest.
			if (row + xtra_rows >= wp->w_height - 2)
			    mod_bot = MAXLNUM;
			else
			{
			    check_for_delay(FALSE);
			    if (win_ins_lines(wp, row + old_rows,
					     xtra_rows, FALSE, FALSE) == FAIL)
				mod_bot = MAXLNUM;
			    else if (top_end > row + old_rows)
				// Scrolled the part at the top that requires
				// updating down.
				top_end += xtra_rows;
			}
		    }

		    // When not updating the rest, may need to move w_lines[]
		    // entries.
		    if (mod_bot != MAXLNUM && i != j)
		    {
			if (j < i)
			{
			    int x = row + new_rows;

			    // move entries in w_lines[] upwards
			    for (;;)
			    {
				// stop at last valid entry in w_lines[]
				if (i >= wp->w_lines_valid)
				{
				    wp->w_lines_valid = j;
				    break;
				}
				wp->w_lines[j] = wp->w_lines[i];
				// stop at a line that won't fit
				if (x + (int)wp->w_lines[j].wl_size
							   > wp->w_height)
				{
				    wp->w_lines_valid = j + 1;
				    break;
				}
				x += wp->w_lines[j++].wl_size;
				++i;
			    }
			    if (bot_start > x)
				bot_start = x;
			}
			else // j > i
			{
			    // move entries in w_lines[] downwards
			    j -= i;
			    wp->w_lines_valid += j;
			    if (wp->w_lines_valid > wp->w_height)
				wp->w_lines_valid = wp->w_height;
			    for (i = wp->w_lines_valid; i - j >= idx; --i)
				wp->w_lines[i] = wp->w_lines[i - j];

			    // The w_lines[] entries for inserted lines are
			    // now invalid, but wl_size may be used above.
			    // Reset to zero.
			    while (i >= idx)
			    {
				wp->w_lines[i].wl_size = 0;
				wp->w_lines[i--].wl_valid = FALSE;
			    }
			}
		    }
		}
	    }

#ifdef FEAT_FOLDING
	    // When lines are folded, display one line for all of them.
	    // Otherwise, display normally (can be several display lines when
	    // 'wrap' is on).
	    fold_count = foldedCount(wp, lnum, &win_foldinfo);
	    if (fold_count != 0)
	    {
		fold_line(wp, fold_count, &win_foldinfo, lnum, row);
		++row;
		--fold_count;
		wp->w_lines[idx].wl_folded = TRUE;
		wp->w_lines[idx].wl_lastlnum = lnum + fold_count;
# ifdef FEAT_SYN_HL
		did_update = DID_FOLD;
# endif
# ifdef FEAT_SPELL
		spv.spv_capcol_lnum = 0;
# endif
	    }
	    else
#endif
	    if (idx < wp->w_lines_valid
		    && wp->w_lines[idx].wl_valid
		    && wp->w_lines[idx].wl_lnum == lnum
		    && lnum > wp->w_topline
		    && !(dy_flags & (DY_LASTLINE | DY_TRUNCATE))
		    && !WIN_IS_POPUP(wp)
		    && srow + wp->w_lines[idx].wl_size > wp->w_height
#ifdef FEAT_DIFF
		    && diff_check_fill(wp, lnum) == 0
#endif
		    )
	    {
		// This line is not going to fit.  Don't draw anything here,
		// will draw "@  " lines below.
		row = wp->w_height + 1;
	    }
	    else
	    {
#ifdef FEAT_SEARCH_EXTRA
		prepare_search_hl(wp, &screen_search_hl, lnum);
#endif
#ifdef FEAT_SYN_HL
		// Let the syntax stuff know we skipped a few lines.
		if (syntax_last_parsed != 0 && syntax_last_parsed + 1 < lnum
						       && syntax_present(wp))
		    syntax_end_parsing(wp, syntax_last_parsed + 1);
#endif

		// Display one line.
		row = win_line(wp, lnum, srow, wp->w_height, FALSE, &spv);

#ifdef FEAT_FOLDING
		wp->w_lines[idx].wl_folded = FALSE;
		wp->w_lines[idx].wl_lastlnum = lnum;
#endif
#ifdef FEAT_SYN_HL
		did_update = DID_LINE;
		syntax_last_parsed = lnum;
#endif
	    }

	    wp->w_lines[idx].wl_lnum = lnum;
	    wp->w_lines[idx].wl_valid = TRUE;

	    // Past end of the window or end of the screen. Note that after
	    // resizing wp->w_height may be end up too big. That's a problem
	    // elsewhere, but prevent a crash here.
	    if (row > wp->w_height || row + wp->w_winrow >= Rows)
	    {
		// we may need the size of that too long line later on
		if (dollar_vcol == -1)
		    wp->w_lines[idx].wl_size = plines_win(wp, lnum, TRUE);
		++idx;
		break;
	    }
	    if (dollar_vcol == -1)
		wp->w_lines[idx].wl_size = row - srow;
	    ++idx;
#ifdef FEAT_FOLDING
	    lnum += fold_count + 1;
#else
	    ++lnum;
#endif
	}
	else
	{
	    if (wp->w_p_rnu && wp->w_last_cursor_lnum_rnu != wp->w_cursor.lnum)
	    {
#ifdef FEAT_FOLDING
		// 'relativenumber' set and the cursor moved vertically: The
		// text doesn't need to be drawn, but the number column does.
		fold_count = foldedCount(wp, lnum, &win_foldinfo);
		if (fold_count != 0)
		    fold_line(wp, fold_count, &win_foldinfo, lnum, row);
		else
#endif
		    (void)win_line(wp, lnum, srow, wp->w_height, TRUE, &spv);
	    }

	    // This line does not need to be drawn, advance to the next one.
	    row += wp->w_lines[idx++].wl_size;
	    if (row > wp->w_height)	// past end of screen
		break;
#ifdef FEAT_FOLDING
	    lnum = wp->w_lines[idx - 1].wl_lastlnum + 1;
#else
	    ++lnum;
#endif
#ifdef FEAT_SYN_HL
	    did_update = DID_NONE;
#endif
#ifdef FEAT_SPELL
	    spv.spv_capcol_lnum = 0;
#endif
	}

	if (lnum > buf->b_ml.ml_line_count)
	{
	    eof = TRUE;
	    break;
	}

	// Safety check: if any of the wl_size values is wrong we might go over
	// the end of w_lines[].
	if (idx >= Rows)
	    break;
    }

    // End of loop over all window lines.

#ifdef FEAT_SYN_HL
    // Now that the window has been redrawn with the old and new cursor line,
    // update w_last_cursorline.
    wp->w_last_cursorline = wp->w_p_cul ? wp->w_cursor.lnum : 0;
#endif
    wp->w_last_cursor_lnum_rnu = wp->w_p_rnu ? wp->w_cursor.lnum : 0;

#ifdef FEAT_VTP
    // Rewrite the character at the end of the screen line.
    // See the version that was fixed.
    if (use_vtp() && get_conpty_fix_type() < 1)
    {
	int k;

	for (k = 0; k < Rows; ++k)
	    if (enc_utf8)
		if ((*mb_off2cells)(LineOffset[k] + Columns - 2,
					   LineOffset[k] + screen_Columns) > 1)
		    screen_draw_rectangle(k, Columns - 2, 1, 2, FALSE);
		else
		    screen_draw_rectangle(k, Columns - 1, 1, 1, FALSE);
	    else
		screen_char(LineOffset[k] + Columns - 1, k, Columns - 1);
    }
#endif

    if (idx > wp->w_lines_valid)
	wp->w_lines_valid = idx;

#ifdef FEAT_SYN_HL
    // Let the syntax stuff know we stop parsing here.
    if (syntax_last_parsed != 0 && syntax_present(wp))
	syntax_end_parsing(wp, syntax_last_parsed + 1);
#endif

    // If we didn't hit the end of the file, and we didn't finish the last
    // line we were working on, then the line didn't fit.
    wp->w_empty_rows = 0;
#ifdef FEAT_DIFF
    wp->w_filler_rows = 0;
#endif
    if (!eof && !didline)
    {
	if (lnum == wp->w_topline)
	{
	    // Single line that does not fit!
	    // Don't overwrite it, it can be edited.
	    wp->w_botline = lnum + 1;
	}
#ifdef FEAT_DIFF
	else if (diff_check_fill(wp, lnum) >= wp->w_height - srow)
	{
	    // Window ends in filler lines.
	    wp->w_botline = lnum;
	    wp->w_filler_rows = wp->w_height - srow;
	}
#endif
#ifdef FEAT_PROP_POPUP
	else if (WIN_IS_POPUP(wp))
	{
	    // popup line that doesn't fit is left as-is
	    wp->w_botline = lnum;
	}
#endif
	else if (dy_flags & DY_TRUNCATE)	// 'display' has "truncate"
	{
	    int		scr_row = W_WINROW(wp) + wp->w_height - 1;
	    int		symbol  = wp->w_fill_chars.lastline;
	    int		charlen;
	    char_u	fillbuf[12];  // 2 characters of 6 bytes

	    charlen = mb_char2bytes(symbol, &fillbuf[0]);
	    mb_char2bytes(symbol, &fillbuf[charlen]);

	    // Last line isn't finished: Display "@@@" in the last screen line.
	    screen_puts_len(fillbuf,
			    (wp->w_width > 2 ? 2 : wp->w_width) * charlen,
			    scr_row, wp->w_wincol, HL_ATTR(HLF_AT));
	    screen_fill(scr_row, scr_row + 1,
		    (int)wp->w_wincol + 2, (int)W_ENDCOL(wp),
		    symbol, ' ', HL_ATTR(HLF_AT));
	    set_empty_rows(wp, srow);
	    wp->w_botline = lnum;
	}
	else if (dy_flags & DY_LASTLINE)	// 'display' has "lastline"
	{
	    int start_col = (int)W_ENDCOL(wp) - 3;
	    int symbol    = wp->w_fill_chars.lastline;

	    // Last line isn't finished: Display "@@@" at the end.
	    screen_fill(W_WINROW(wp) + wp->w_height - 1,
		    W_WINROW(wp) + wp->w_height,
		    start_col < wp->w_wincol ? wp->w_wincol : start_col,
		    (int)W_ENDCOL(wp),
		    symbol, symbol, HL_ATTR(HLF_AT));
	    set_empty_rows(wp, srow);
	    wp->w_botline = lnum;
	}
	else
	{
	    win_draw_end(wp, wp->w_fill_chars.lastline, ' ', TRUE,
						   srow, wp->w_height, HLF_AT);
	    wp->w_botline = lnum;
	}
    }
    else
    {
	draw_vsep_win(wp, row);
	if (eof)		// we hit the end of the file
	{
	    wp->w_botline = buf->b_ml.ml_line_count + 1;
#ifdef FEAT_DIFF
	    j = diff_check_fill(wp, wp->w_botline);
	    if (j > 0 && !wp->w_botfill)
	    {
		// Display filler lines at the end of the file.
		if (char2cells(wp->w_fill_chars.diff) > 1)
		    i = '-';
		else
		    i = wp->w_fill_chars.diff;
		if (row + j > wp->w_height)
		    j = wp->w_height - row;
		win_draw_end(wp, i, i, TRUE, row, row + (int)j, HLF_DED);
		row += j;
	    }
#endif
	}
	else if (dollar_vcol == -1)
	    wp->w_botline = lnum;

	// Make sure the rest of the screen is blank.
	// write the "eob" character from 'fillchars' to rows that aren't part
	// of the file.
	if (WIN_IS_POPUP(wp))
	    win_draw_end(wp, ' ', ' ', FALSE, row, wp->w_height, HLF_AT);
	else
	    win_draw_end(wp, wp->w_fill_chars.eob, ' ', FALSE,
						   row, wp->w_height, HLF_EOB);
    }

#ifdef SYN_TIME_LIMIT
    disable_regexp_timeout();
    redrawtime_limit_set = FALSE;
#endif

    // Reset the type of redrawing required, the window has been updated.
    wp->w_redr_type = 0;
#ifdef FEAT_DIFF
    wp->w_old_topfill = wp->w_topfill;
    wp->w_old_botfill = wp->w_botfill;
#endif

    if (dollar_vcol == -1)
    {
	// There is a trick with w_botline.  If we invalidate it on each
	// change that might modify it, this will cause a lot of expensive
	// calls to plines() in update_topline() each time.  Therefore the
	// value of w_botline is often approximated, and this value is used to
	// compute the value of w_topline.  If the value of w_botline was
	// wrong, check that the value of w_topline is correct (cursor is on
	// the visible part of the text).  If it's not, we need to redraw
	// again.  Mostly this just means scrolling up a few lines, so it
	// doesn't look too bad.  Only do this for the current window (where
	// changes are relevant).
	wp->w_valid |= VALID_BOTLINE;
	if (wp == curwin && wp->w_botline != old_botline && !recursive)
	{
	    win_T	*wwp;
#if defined(FEAT_CONCEAL)
	    linenr_T	old_topline = wp->w_topline;
	    int		new_wcol = wp->w_wcol;
#endif
	    recursive = TRUE;
	    curwin->w_valid &= ~VALID_TOPLINE;
	    update_topline();	// may invalidate w_botline again

#if defined(FEAT_CONCEAL)
	    if (old_wcol != new_wcol && (wp->w_valid & (VALID_WCOL|VALID_WROW))
						    != (VALID_WCOL|VALID_WROW))
	    {
		// A win_line() call applied a fix to screen cursor column to
		// accommodate concealment of cursor line, but in this call to
		// update_topline() the cursor's row or column got invalidated.
		// If they are left invalid, setcursor() will recompute them
		// but there won't be any further win_line() call to re-fix the
		// column and the cursor will end up misplaced.  So we call
		// cursor validation now and reapply the fix again (or call
		// win_line() to do it for us).
		validate_cursor();
		if (wp->w_wcol == old_wcol && wp->w_wrow == old_wrow
					       && old_topline == wp->w_topline)
		    wp->w_wcol = new_wcol;
		else
		    redrawWinline(wp, wp->w_cursor.lnum);
	    }
#endif
	    // New redraw either due to updated topline or due to wcol fix.
	    if (wp->w_redr_type != 0)
	    {
		// Don't update for changes in buffer again.
		i = curbuf->b_mod_set;
		curbuf->b_mod_set = FALSE;
		j = curbuf->b_mod_xlines;
		curbuf->b_mod_xlines = 0;
		win_update(curwin);
		curbuf->b_mod_set = i;
		curbuf->b_mod_xlines = j;
	    }
	    // Other windows might have w_redr_type raised in update_topline().
	    must_redraw = 0;
	    FOR_ALL_WINDOWS(wwp)
		if (wwp->w_redr_type > must_redraw)
		    must_redraw = wwp->w_redr_type;
	    recursive = FALSE;
	}
    }

#if defined(FEAT_SYN_HL) || defined(FEAT_SEARCH_EXTRA)
    // restore got_int, unless CTRL-C was hit while redrawing
    if (!got_int)
	got_int = save_got_int;
#endif
}

#if defined(FEAT_NETBEANS_INTG) || defined(FEAT_GUI)
/*
 * Prepare for updating one or more windows.
 * Caller must check for "updating_screen" already set to avoid recursiveness.
 */
    static void
update_prepare(void)
{
    cursor_off();
    updating_screen = TRUE;
#ifdef FEAT_GUI
    // Remove the cursor before starting to do anything, because scrolling may
    // make it difficult to redraw the text under it.
    if (gui.in_use)
	gui_undraw_cursor();
#endif
#ifdef FEAT_SEARCH_EXTRA
    start_search_hl();
#endif
#ifdef FEAT_PROP_POPUP
    // Update popup_mask if needed.
    may_update_popup_mask(must_redraw);
#endif
}

/*
 * Finish updating one or more windows.
 */
    static void
update_finish(void)
{
    if (redraw_cmdline || redraw_mode)
	showmode();

# ifdef FEAT_SEARCH_EXTRA
    end_search_hl();
# endif

    after_updating_screen(TRUE);

# ifdef FEAT_GUI
    // Redraw the cursor and update the scrollbars when all screen updating is
    // done.
    if (gui.in_use)
    {
	out_flush_cursor(FALSE, FALSE);
	gui_update_scrollbars(FALSE);
    }
# endif
}
#endif

#if defined(FEAT_NETBEANS_INTG) || defined(PROTO)
    void
update_debug_sign(buf_T *buf, linenr_T lnum)
{
    win_T	*wp;
    int		doit = FALSE;

# ifdef FEAT_FOLDING
    win_foldinfo.fi_level = 0;
# endif

    // update/delete a specific sign
    redraw_buf_line_later(buf, lnum);

    // check if it resulted in the need to redraw a window
    FOR_ALL_WINDOWS(wp)
	if (wp->w_redr_type != 0)
	    doit = TRUE;

    // Return when there is nothing to do, screen updating is already
    // happening (recursive call), messages on the screen or still starting up.
    if (!doit || updating_screen
	    || State == MODE_ASKMORE || State == MODE_HITRETURN
	    || msg_scrolled
#ifdef FEAT_GUI
	    || gui.starting
#endif
	    || starting)
	return;

    // update all windows that need updating
    update_prepare();

    FOR_ALL_WINDOWS(wp)
    {
	if (wp->w_redr_type != 0)
	    win_update(wp);
	if (wp->w_redr_status)
	    win_redr_status(wp, FALSE);
    }

    update_finish();
}
#endif

#if defined(FEAT_GUI) || defined(PROTO)
/*
 * Update a single window, its status line and maybe the command line msg.
 * Used for the GUI scrollbar.
 */
    void
updateWindow(win_T *wp)
{
    // return if already busy updating
    if (updating_screen)
	return;

    update_prepare();

#ifdef FEAT_CLIPBOARD
    // When Visual area changed, may have to update selection.
    if (clip_star.available && clip_isautosel_star())
	clip_update_selection(&clip_star);
    if (clip_plus.available && clip_isautosel_plus())
	clip_update_selection(&clip_plus);
#endif

    win_update(wp);

    // When the screen was cleared redraw the tab pages line.
    if (redraw_tabline)
	draw_tabline();

    if (wp->w_redr_status || p_ru
# ifdef FEAT_STL_OPT
	    || *p_stl != NUL || *wp->w_p_stl != NUL
# endif
	    )
	win_redr_status(wp, FALSE);

#ifdef FEAT_PROP_POPUP
    // Display popup windows on top of everything.
    update_popups(win_update);
#endif

    update_finish();
}
#endif

/*
 * Redraw as soon as possible.  When the command line is not scrolled redraw
 * right away and restore what was on the command line.
 * Return a code indicating what happened.
 */
    int
redraw_asap(int type)
{
    int		rows;
    int		cols = screen_Columns;
    int		r;
    int		ret = 0;
    schar_T	*screenline;	// copy from ScreenLines[]
    sattr_T	*screenattr;	// copy from ScreenAttrs[]
    int		i;
    u8char_T	*screenlineUC = NULL;	// copy from ScreenLinesUC[]
    u8char_T	*screenlineC[MAX_MCO];	// copy from ScreenLinesC[][]
    schar_T	*screenline2 = NULL;	// copy from ScreenLines2[]

    redraw_later(type);
    if (msg_scrolled
	    || (State != MODE_NORMAL && State != MODE_NORMAL_BUSY)
	    || exiting)
	return ret;

    // Allocate space to save the text displayed in the command line area.
    rows = screen_Rows - cmdline_row;
    screenline = LALLOC_MULT(schar_T, rows * cols);
    screenattr = LALLOC_MULT(sattr_T, rows * cols);
    if (screenline == NULL || screenattr == NULL)
	ret = 2;
    if (enc_utf8)
    {
	screenlineUC = LALLOC_MULT(u8char_T, rows * cols);
	if (screenlineUC == NULL)
	    ret = 2;
	for (i = 0; i < p_mco; ++i)
	{
	    screenlineC[i] = LALLOC_MULT(u8char_T, rows * cols);
	    if (screenlineC[i] == NULL)
		ret = 2;
	}
    }
    if (enc_dbcs == DBCS_JPNU)
    {
	screenline2 = LALLOC_MULT(schar_T, rows * cols);
	if (screenline2 == NULL)
	    ret = 2;
    }

    if (ret != 2)
    {
	// Save the text displayed in the command line area.
	for (r = 0; r < rows; ++r)
	{
	    mch_memmove(screenline + r * cols,
			ScreenLines + LineOffset[cmdline_row + r],
			(size_t)cols * sizeof(schar_T));
	    mch_memmove(screenattr + r * cols,
			ScreenAttrs + LineOffset[cmdline_row + r],
			(size_t)cols * sizeof(sattr_T));
	    if (enc_utf8)
	    {
		mch_memmove(screenlineUC + r * cols,
			    ScreenLinesUC + LineOffset[cmdline_row + r],
			    (size_t)cols * sizeof(u8char_T));
		for (i = 0; i < p_mco; ++i)
		    mch_memmove(screenlineC[i] + r * cols,
				ScreenLinesC[i] + LineOffset[cmdline_row + r],
				(size_t)cols * sizeof(u8char_T));
	    }
	    if (enc_dbcs == DBCS_JPNU)
		mch_memmove(screenline2 + r * cols,
			    ScreenLines2 + LineOffset[cmdline_row + r],
			    (size_t)cols * sizeof(schar_T));
	}

	update_screen(0);
	ret = 3;

	if (must_redraw == 0)
	{
	    int	off = (int)(current_ScreenLine - ScreenLines);

	    // Restore the text displayed in the command line area.
	    for (r = 0; r < rows; ++r)
	    {
		mch_memmove(current_ScreenLine,
			    screenline + r * cols,
			    (size_t)cols * sizeof(schar_T));
		mch_memmove(ScreenAttrs + off,
			    screenattr + r * cols,
			    (size_t)cols * sizeof(sattr_T));
		if (enc_utf8)
		{
		    mch_memmove(ScreenLinesUC + off,
				screenlineUC + r * cols,
				(size_t)cols * sizeof(u8char_T));
		    for (i = 0; i < p_mco; ++i)
			mch_memmove(ScreenLinesC[i] + off,
				    screenlineC[i] + r * cols,
				    (size_t)cols * sizeof(u8char_T));
		}
		if (enc_dbcs == DBCS_JPNU)
		    mch_memmove(ScreenLines2 + off,
				screenline2 + r * cols,
				(size_t)cols * sizeof(schar_T));
		screen_line(curwin, cmdline_row + r, 0, cols, cols, 0);
	    }
	    ret = 4;
	}
    }

    vim_free(screenline);
    vim_free(screenattr);
    if (enc_utf8)
    {
	vim_free(screenlineUC);
	for (i = 0; i < p_mco; ++i)
	    vim_free(screenlineC[i]);
    }
    if (enc_dbcs == DBCS_JPNU)
	vim_free(screenline2);

    // Show the intro message when appropriate.
    maybe_intro_message();

    setcursor();

    return ret;
}

/*
 * Invoked after an asynchronous callback is called.
 * If an echo command was used the cursor needs to be put back where
 * it belongs. If highlighting was changed a redraw is needed.
 * If "call_update_screen" is FALSE don't call update_screen() when at the
 * command line.
 * If "redraw_message" is TRUE.
 */
    void
redraw_after_callback(int call_update_screen, int do_message)
{
    ++redrawing_for_callback;

    if (State == MODE_HITRETURN || State == MODE_ASKMORE
	    || State == MODE_SETWSIZE || State == MODE_EXTERNCMD
	    || State == MODE_CONFIRM || exmode_active)
    {
	if (do_message)
	    repeat_message();
    }
    else if (State & MODE_CMDLINE)
    {
	if (pum_visible())
	    cmdline_pum_display();

	// Don't redraw when in prompt_for_number().
	if (cmdline_row > 0)
	{
	    // Redrawing only works when the screen didn't scroll. Don't clear
	    // wildmenu entries.
	    if (msg_scrolled == 0
		    && wild_menu_showing == 0
		    && call_update_screen)
		update_screen(0);

	    // Redraw in the same position, so that the user can continue
	    // editing the command.
	    redrawcmdline_ex(FALSE);
	}
    }
    else if (State & (MODE_NORMAL | MODE_INSERT | MODE_TERMINAL))
    {
	update_topline();
	validate_cursor();

	// keep the command line if possible
	update_screen(UPD_VALID_NO_UPDATE);
	setcursor();

	if (msg_scrolled == 0)
	{
	    // don't want a hit-enter prompt when something else is displayed
	    msg_didany = FALSE;
	    need_wait_return = FALSE;
	}
    }
    cursor_on();
#ifdef FEAT_GUI
    if (gui.in_use && !gui_mch_is_blink_off())
	// Don't update the cursor when it is blinking and off to avoid
	// flicker.
	out_flush_cursor(FALSE, FALSE);
    else
#endif
	out_flush();

    --redrawing_for_callback;
}

/*
 * Redraw the current window later, with update_screen(type).
 * Set must_redraw only if not already set to a higher value.
 * E.g. if must_redraw is UPD_CLEAR, type UPD_NOT_VALID will do nothing.
 */
    void
redraw_later(int type)
{
    redraw_win_later(curwin, type);
}

    void
redraw_win_later(
    win_T	*wp,
    int		type)
{
    if (!exiting && !redraw_not_allowed && wp->w_redr_type < type)
    {
	wp->w_redr_type = type;
	if (type >= UPD_NOT_VALID)
	    wp->w_lines_valid = 0;
	if (must_redraw < type)	// must_redraw is the maximum of all windows
	    must_redraw = type;
    }
}

/*
 * Force a complete redraw later.  Also resets the highlighting.  To be used
 * after executing a shell command that messes up the screen.
 */
    void
redraw_later_clear(void)
{
    redraw_all_later(UPD_CLEAR);
    reset_screen_attr();
}

/*
 * Mark all windows to be redrawn later.  Except popup windows.
 */
    void
redraw_all_later(int type)
{
    win_T	*wp;

    FOR_ALL_WINDOWS(wp)
	redraw_win_later(wp, type);
    // This may be needed when switching tabs.
    set_must_redraw(type);
}

#if 0  // not actually used yet, it probably should
/*
 * Mark all windows, including popup windows, to be redrawn.
 */
    void
redraw_all_windows_later(int type)
{
    redraw_all_later(type);
#ifdef FEAT_PROP_POPUP
    popup_redraw_all();		// redraw all popup windows
#endif
}
#endif

/*
 * Set "must_redraw" to "type" unless it already has a higher value
 * or it is currently not allowed.
 */
    void
set_must_redraw(int type)
{
    if (!redraw_not_allowed && must_redraw < type)
	must_redraw = type;
}

/*
 * Mark all windows that are editing the current buffer to be updated later.
 */
    void
redraw_curbuf_later(int type)
{
    redraw_buf_later(curbuf, type);
}

    void
redraw_buf_later(buf_T *buf, int type)
{
    win_T	*wp;

    FOR_ALL_WINDOWS(wp)
    {
	if (wp->w_buffer == buf)
	    redraw_win_later(wp, type);
    }
#if defined(FEAT_TERMINAL) && defined(FEAT_PROP_POPUP)
    // terminal in popup window is not in list of windows
    if (curwin->w_buffer == buf)
	redraw_win_later(curwin, type);
#endif
}

#if defined(FEAT_SIGNS) || defined(PROTO)
    void
redraw_buf_line_later(buf_T *buf, linenr_T lnum)
{
    win_T	*wp;

    FOR_ALL_WINDOWS(wp)
	if (wp->w_buffer == buf && lnum >= wp->w_topline
						  && lnum < wp->w_botline)
	    redrawWinline(wp, lnum);
}
#endif

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
    void
redraw_buf_and_status_later(buf_T *buf, int type)
{
    win_T	*wp;

    if (wild_menu_showing != 0)
	// Don't redraw while the command line completion is displayed, it
	// would disappear.
	return;
    FOR_ALL_WINDOWS(wp)
    {
	if (wp->w_buffer == buf)
	{
	    redraw_win_later(wp, type);
	    wp->w_redr_status = TRUE;
	}
    }
}
#endif

/*
 * mark all status lines for redraw; used after first :cd
 */
    void
status_redraw_all(void)
{
    win_T	*wp;

    FOR_ALL_WINDOWS(wp)
	if (wp->w_status_height)
	{
	    wp->w_redr_status = TRUE;
	    redraw_later(UPD_VALID);
	}
}

/*
 * mark all status lines of the current buffer for redraw
 */
    void
status_redraw_curbuf(void)
{
    win_T	*wp;

    FOR_ALL_WINDOWS(wp)
	if (wp->w_status_height != 0 && wp->w_buffer == curbuf)
	{
	    wp->w_redr_status = TRUE;
	    redraw_later(UPD_VALID);
	}
}

/*
 * Redraw all status lines that need to be redrawn.
 */
    void
redraw_statuslines(void)
{
    win_T	*wp;

    FOR_ALL_WINDOWS(wp)
	if (wp->w_redr_status)
	    win_redr_status(wp, FALSE);
    if (redraw_tabline)
	draw_tabline();
}

/*
 * Redraw all status lines at the bottom of frame "frp".
 */
    void
win_redraw_last_status(frame_T *frp)
{
    if (frp->fr_layout == FR_LEAF)
	frp->fr_win->w_redr_status = TRUE;
    else if (frp->fr_layout == FR_ROW)
    {
	FOR_ALL_FRAMES(frp, frp->fr_child)
	    win_redraw_last_status(frp);
    }
    else // frp->fr_layout == FR_COL
    {
	frp = frp->fr_child;
	while (frp->fr_next != NULL)
	    frp = frp->fr_next;
	win_redraw_last_status(frp);
    }
}

/*
 * Changed something in the current window, at buffer line "lnum", that
 * requires that line and possibly other lines to be redrawn.
 * Used when entering/leaving Insert mode with the cursor on a folded line.
 * Used to remove the "$" from a change command.
 * Note that when also inserting/deleting lines w_redraw_top and w_redraw_bot
 * may become invalid and the whole window will have to be redrawn.
 */
    void
redrawWinline(
    win_T	*wp,
    linenr_T	lnum)
{
    if (wp->w_redraw_top == 0 || wp->w_redraw_top > lnum)
	wp->w_redraw_top = lnum;
    if (wp->w_redraw_bot == 0 || wp->w_redraw_bot < lnum)
	wp->w_redraw_bot = lnum;
    redraw_win_later(wp, UPD_VALID);
}
