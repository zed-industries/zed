/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 * move.c: Functions for moving the cursor and scrolling text.
 *
 * There are two ways to move the cursor:
 * 1. Move the cursor directly, the text is scrolled to keep the cursor in the
 *    window.
 * 2. Scroll the text, the cursor is moved into the text visible in the
 *    window.
 * The 'scrolloff' option makes this a bit complicated.
 */

#include "vim.h"

static int scrolljump_value(void);
static int check_top_offset(void);
static void curs_rows(win_T *wp);

typedef struct
{
    linenr_T	    lnum;	// line number
#ifdef FEAT_DIFF
    int		    fill;	// filler lines
#endif
    int		    height;	// height of added line
} lineoff_T;

static void topline_back(lineoff_T *lp);
static void botline_forw(lineoff_T *lp);

/*
 * Get the number of screen lines skipped with "wp->w_skipcol".
 */
    int
adjust_plines_for_skipcol(win_T *wp)
{
    if (wp->w_skipcol == 0)
	return 0;

    int width = wp->w_width - win_col_off(wp);
    int w2 = width + win_col_off2(wp);
    if (wp->w_skipcol >= width && w2 > 0)
	return (wp->w_skipcol - width) / w2 + 1;

    return 0;
}

/*
 * Return how many lines "lnum" will take on the screen, taking into account
 * whether it is the first line, whether w_skipcol is non-zero and limiting to
 * the window height.
 */
    static int
plines_correct_topline(win_T *wp, linenr_T lnum)
{
    int n;
#ifdef FEAT_DIFF
    if (lnum == wp->w_topline)
	n = plines_win_nofill(wp, lnum, FALSE) + wp->w_topfill;
    else
#endif
	n = plines_win(wp, lnum, FALSE);
    if (lnum == wp->w_topline)
	n -= adjust_plines_for_skipcol(wp);
    if (n > wp->w_height)
	n = wp->w_height;
    return n;
}

/*
 * Compute wp->w_botline for the current wp->w_topline.  Can be called after
 * wp->w_topline changed.
 */
    static void
comp_botline(win_T *wp)
{
    int		n;
    linenr_T	lnum;
    int		done;
#ifdef FEAT_FOLDING
    linenr_T    last;
    int		folded;
#endif

    /*
     * If w_cline_row is valid, start there.
     * Otherwise have to start at w_topline.
     */
    check_cursor_moved(wp);
    if (wp->w_valid & VALID_CROW)
    {
	lnum = wp->w_cursor.lnum;
	done = wp->w_cline_row;
    }
    else
    {
	lnum = wp->w_topline;
	done = 0;
    }

    for ( ; lnum <= wp->w_buffer->b_ml.ml_line_count; ++lnum)
    {
#ifdef FEAT_FOLDING
	last = lnum;
	folded = FALSE;
	if (hasFoldingWin(wp, lnum, NULL, &last, TRUE, NULL))
	{
	    n = 1;
	    folded = TRUE;
	}
	else
#endif
	{
	    n = plines_correct_topline(wp, lnum);
	}
	if (
#ifdef FEAT_FOLDING
		lnum <= wp->w_cursor.lnum && last >= wp->w_cursor.lnum
#else
		lnum == wp->w_cursor.lnum
#endif
	   )
	{
	    wp->w_cline_row = done;
	    wp->w_cline_height = n;
#ifdef FEAT_FOLDING
	    wp->w_cline_folded = folded;
#endif
	    redraw_for_cursorline(wp);
	    wp->w_valid |= (VALID_CROW|VALID_CHEIGHT);
	}
	if (done + n > wp->w_height)
	    break;
	done += n;
#ifdef FEAT_FOLDING
	lnum = last;
#endif
    }

    // wp->w_botline is the line that is just below the window
    wp->w_botline = lnum;
    wp->w_valid |= VALID_BOTLINE|VALID_BOTLINE_AP;

    set_empty_rows(wp, done);
}

/*
 * Redraw when w_cline_row changes and 'relativenumber' or 'cursorline' is
 * set.
 */
    void
redraw_for_cursorline(win_T *wp)
{
    if ((wp->w_p_rnu
#ifdef FEAT_SYN_HL
		|| wp->w_p_cul
#endif
		)
	    && (wp->w_valid & VALID_CROW) == 0
	    && !pum_visible())
    {
	// win_line() will redraw the number column and cursorline only.
	redraw_win_later(wp, UPD_VALID);
    }
}

#ifdef FEAT_SYN_HL
/*
 * Redraw when w_virtcol changes and 'cursorcolumn' is set or 'cursorlineopt'
 * contains "screenline".
 */
    static void
redraw_for_cursorcolumn(win_T *wp)
{
    if ((wp->w_valid & VALID_VIRTCOL) == 0 && !pum_visible())
    {
	// When 'cursorcolumn' is set need to redraw with UPD_SOME_VALID.
	if (wp->w_p_cuc)
	    redraw_win_later(wp, UPD_SOME_VALID);
	// When 'cursorlineopt' contains "screenline" need to redraw with
	// UPD_VALID.
	else if (wp->w_p_cul && (wp->w_p_culopt_flags & CULOPT_SCRLINE))
	    redraw_win_later(wp, UPD_VALID);
    }
}
#endif

/*
 * Calculates how much the 'listchars' "precedes" or 'smoothscroll' "<<<"
 * marker overlaps with buffer text for window "wp".
 * Parameter "extra2" should be the padding on the 2nd line, not the first
 * line.
 * Returns the number of columns of overlap with buffer text, excluding the
 * extra padding on the ledge.
 */
     int
sms_marker_overlap(win_T *wp, int extra2)
{
#if defined(FEAT_LINEBREAK)
    // There is no marker overlap when in showbreak mode, thus no need to
    // account for it.  See wlv_screen_line().
    if (*get_showbreak_value(wp) != NUL)
	return 0;
#endif
    // Overlap when 'list' and 'listchars' "precedes" are set is 1.
    if (wp->w_p_list && wp->w_lcs_chars.prec)
	return 1;

    return extra2 > 3 ? 0 : 3 - extra2;
}

/*
 * Calculates the skipcol offset for window "wp" given how many
 * physical lines we want to scroll down.
 */
    static int
skipcol_from_plines(win_T *wp, int plines_off)
{
    int width1 = wp->w_width - win_col_off(wp);

    int skipcol = 0;
    if (plines_off > 0)
	skipcol += width1;
    if (plines_off > 1)
	skipcol += (width1 + win_col_off2(wp)) * (plines_off - 1);
    return skipcol;
}

/*
 * Set curwin->w_skipcol to zero and redraw later if needed.
 */
    static void
reset_skipcol(void)
{
    if (curwin->w_skipcol == 0)
	return;

    curwin->w_skipcol = 0;

    // Should use the least expensive way that displays all that changed.
    // UPD_NOT_VALID is too expensive, UPD_REDRAW_TOP does not redraw
    // enough when the top line gets another screen line.
    redraw_later(UPD_SOME_VALID);
}

/*
 * Update curwin->w_topline and redraw if necessary.
 * Used to update the screen before printing a message.
 */
    void
update_topline_redraw(void)
{
    update_topline();
    if (must_redraw)
	update_screen(0);
}

/*
 * Update curwin->w_topline to move the cursor onto the screen.
 */
    void
update_topline(void)
{
    long	line_count;
    int		halfheight;
    int		n;
#ifdef FEAT_FOLDING
    linenr_T	lnum;
#endif
    int		check_topline = FALSE;
    int		check_botline = FALSE;
    long	*so_ptr = curwin->w_p_so >= 0 ? &curwin->w_p_so : &p_so;
    int		save_so = *so_ptr;

    // Cursor is updated instead when this is TRUE for 'splitkeep'.
    if (skip_update_topline)
	return;

    // If there is no valid screen and when the window height is zero just use
    // the cursor line.
    if (!screen_valid(TRUE) || curwin->w_height == 0)
    {
	check_cursor_lnum();
	curwin->w_topline = curwin->w_cursor.lnum;
	curwin->w_botline = curwin->w_topline;
	curwin->w_scbind_pos = 1;
	return;
    }

    check_cursor_moved(curwin);
    if (curwin->w_valid & VALID_TOPLINE)
	return;

    // When dragging with the mouse, don't scroll that quickly
    if (mouse_dragging > 0)
	*so_ptr = mouse_dragging - 1;

    linenr_T old_topline = curwin->w_topline;
#ifdef FEAT_DIFF
    int old_topfill = curwin->w_topfill;
#endif

    /*
     * If the buffer is empty, always set topline to 1.
     */
    if (BUFEMPTY())		// special case - file is empty
    {
	if (curwin->w_topline != 1)
	    redraw_later(UPD_NOT_VALID);
	curwin->w_topline = 1;
	curwin->w_botline = 2;
	curwin->w_valid |= VALID_BOTLINE|VALID_BOTLINE_AP;
	curwin->w_scbind_pos = 1;
    }

    /*
     * If the cursor is above or near the top of the window, scroll the window
     * to show the line the cursor is in, with 'scrolloff' context.
     */
    else
    {
	if (curwin->w_topline > 1 || curwin->w_skipcol > 0)
	{
	    // If the cursor is above topline, scrolling is always needed.
	    // If the cursor is far below topline and there is no folding,
	    // scrolling down is never needed.
	    if (curwin->w_cursor.lnum < curwin->w_topline)
		check_topline = TRUE;
	    else if (check_top_offset())
		check_topline = TRUE;
	    else if (curwin->w_skipcol > 0
				 && curwin->w_cursor.lnum == curwin->w_topline)
	    {
		colnr_T vcol;

		// Check that the cursor position is visible.  Add columns for
		// the marker displayed in the top-left if needed.
		getvvcol(curwin, &curwin->w_cursor, &vcol, NULL, NULL);
		int overlap = sms_marker_overlap(curwin, curwin_col_off()
							- curwin_col_off2());
		if (curwin->w_skipcol + overlap > vcol)
		    check_topline = TRUE;
	    }
	}
#ifdef FEAT_DIFF
	    // Check if there are more filler lines than allowed.
	if (!check_topline && curwin->w_topfill > diff_check_fill(curwin,
							   curwin->w_topline))
	    check_topline = TRUE;
#endif

	if (check_topline)
	{
	    halfheight = curwin->w_height / 2 - 1;
	    if (halfheight < 2)
		halfheight = 2;

#ifdef FEAT_FOLDING
	    if (hasAnyFolding(curwin))
	    {
		// Count the number of logical lines between the cursor and
		// topline + scrolloff (approximation of how much will be
		// scrolled).
		n = 0;
		for (lnum = curwin->w_cursor.lnum;
				    lnum < curwin->w_topline + *so_ptr; ++lnum)
		{
		    ++n;
		    // stop at end of file or when we know we are far off
		    if (lnum >= curbuf->b_ml.ml_line_count || n >= halfheight)
			break;
		    (void)hasFolding(lnum, NULL, &lnum);
		}
	    }
	    else
#endif
		n = curwin->w_topline + *so_ptr - curwin->w_cursor.lnum;

	    // If we weren't very close to begin with, we scroll to put the
	    // cursor in the middle of the window.  Otherwise put the cursor
	    // near the top of the window.
	    if (n >= halfheight)
		scroll_cursor_halfway(FALSE, FALSE);
	    else
	    {
		scroll_cursor_top(scrolljump_value(), FALSE);
		check_botline = TRUE;
	    }
	}

	else
	{
#ifdef FEAT_FOLDING
	    // Make sure topline is the first line of a fold.
	    (void)hasFolding(curwin->w_topline, &curwin->w_topline, NULL);
#endif
	    check_botline = TRUE;
	}
    }

    /*
     * If the cursor is below the bottom of the window, scroll the window
     * to put the cursor on the window.
     * When w_botline is invalid, recompute it first, to avoid a redraw later.
     * If w_botline was approximated, we might need a redraw later in a few
     * cases, but we don't want to spend (a lot of) time recomputing w_botline
     * for every small change.
     */
    if (check_botline)
    {
	if (!(curwin->w_valid & VALID_BOTLINE_AP))
	    validate_botline();

	if (curwin->w_botline <= curbuf->b_ml.ml_line_count)
	{
	    if (curwin->w_cursor.lnum < curwin->w_botline)
	    {
	      if (((long)curwin->w_cursor.lnum
					   >= (long)curwin->w_botline - *so_ptr
#ifdef FEAT_FOLDING
			|| hasAnyFolding(curwin)
#endif
			))
	      {
		lineoff_T	loff;

		// Cursor is (a few lines) above botline, check if there are
		// 'scrolloff' window lines below the cursor.  If not, need to
		// scroll.
		n = curwin->w_empty_rows;
		loff.lnum = curwin->w_cursor.lnum;
#ifdef FEAT_FOLDING
		// In a fold go to its last line.
		(void)hasFolding(loff.lnum, NULL, &loff.lnum);
#endif
#ifdef FEAT_DIFF
		loff.fill = 0;
		n += curwin->w_filler_rows;
#endif
		loff.height = 0;
		while (loff.lnum < curwin->w_botline
#ifdef FEAT_DIFF
			&& (loff.lnum + 1 < curwin->w_botline || loff.fill == 0)
#endif
			)
		{
		    n += loff.height;
		    if (n >= *so_ptr)
			break;
		    botline_forw(&loff);
		}
		if (n >= *so_ptr)
		    // sufficient context, no need to scroll
		    check_botline = FALSE;
	      }
	      else
		  // sufficient context, no need to scroll
		  check_botline = FALSE;
	    }
	    if (check_botline)
	    {
#ifdef FEAT_FOLDING
		if (hasAnyFolding(curwin))
		{
		    // Count the number of logical lines between the cursor and
		    // botline - scrolloff (approximation of how much will be
		    // scrolled).
		    line_count = 0;
		    for (lnum = curwin->w_cursor.lnum;
				   lnum >= curwin->w_botline - *so_ptr; --lnum)
		    {
			++line_count;
			// stop at end of file or when we know we are far off
			if (lnum <= 0 || line_count > curwin->w_height + 1)
			    break;
			(void)hasFolding(lnum, &lnum, NULL);
		    }
		}
		else
#endif
		    line_count = curwin->w_cursor.lnum - curwin->w_botline
								 + 1 + *so_ptr;
		if (line_count <= curwin->w_height + 1)
		    scroll_cursor_bot(scrolljump_value(), FALSE);
		else
		    scroll_cursor_halfway(FALSE, FALSE);
	    }
	}
    }
    curwin->w_valid |= VALID_TOPLINE;

    /*
     * Need to redraw when topline changed.
     */
    if (curwin->w_topline != old_topline
#ifdef FEAT_DIFF
	    || curwin->w_topfill != old_topfill
#endif
	    )
    {
	dollar_vcol = -1;
	redraw_later(UPD_VALID);

	// When 'smoothscroll' is not set, should reset w_skipcol.
	if (!curwin->w_p_sms)
	    reset_skipcol();
	else if (curwin->w_skipcol != 0)
	    redraw_later(UPD_SOME_VALID);

	// May need to set w_skipcol when cursor in w_topline.
	if (curwin->w_cursor.lnum == curwin->w_topline)
	    validate_cursor();
    }

    *so_ptr = save_so;
}

/*
 * Return the scrolljump value to use for the current window.
 * When 'scrolljump' is positive use it as-is.
 * When 'scrolljump' is negative use it as a percentage of the window height.
 */
    static int
scrolljump_value(void)
{
    if (p_sj >= 0)
	return (int)p_sj;
    return (curwin->w_height * -p_sj) / 100;
}

/*
 * Return TRUE when there are not 'scrolloff' lines above the cursor for the
 * current window.
 */
    static int
check_top_offset(void)
{
    lineoff_T	loff;
    int		n;
    long	so = get_scrolloff_value();

    if (curwin->w_cursor.lnum < curwin->w_topline + so
#ifdef FEAT_FOLDING
		    || hasAnyFolding(curwin)
#endif
	    )
    {
	loff.lnum = curwin->w_cursor.lnum;
#ifdef FEAT_DIFF
	loff.fill = 0;
	n = curwin->w_topfill;	    // always have this context
#else
	n = 0;
#endif
	// Count the visible screen lines above the cursor line.
	while (n < so)
	{
	    topline_back(&loff);
	    // Stop when included a line above the window.
	    if (loff.lnum < curwin->w_topline
#ifdef FEAT_DIFF
		    || (loff.lnum == curwin->w_topline && loff.fill > 0)
#endif
		    )
		break;
	    n += loff.height;
	}
	if (n < so)
	    return TRUE;
    }
    return FALSE;
}

/*
 * Update w_curswant.
 */
    void
update_curswant_force(void)
{
    validate_virtcol();
    curwin->w_curswant = curwin->w_virtcol
#ifdef FEAT_PROP_POPUP
	- curwin->w_virtcol_first_char
#endif
	;
    curwin->w_set_curswant = FALSE;
}

/*
 * Update w_curswant if w_set_curswant is set.
 */
    void
update_curswant(void)
{
    if (curwin->w_set_curswant)
	update_curswant_force();
}

/*
 * Check if the cursor has moved.  Set the w_valid flag accordingly.
 */
    void
check_cursor_moved(win_T *wp)
{
    if (wp->w_cursor.lnum != wp->w_valid_cursor.lnum)
    {
	wp->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL
				      |VALID_CHEIGHT|VALID_CROW|VALID_TOPLINE
				      |VALID_BOTLINE|VALID_BOTLINE_AP);
	wp->w_valid_cursor = wp->w_cursor;
	wp->w_valid_leftcol = wp->w_leftcol;
	wp->w_valid_skipcol = wp->w_skipcol;
    }
    else if (wp->w_skipcol != wp->w_valid_skipcol)
    {
	wp->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL
				      |VALID_CHEIGHT|VALID_CROW
				      |VALID_BOTLINE|VALID_BOTLINE_AP);
	wp->w_valid_cursor = wp->w_cursor;
	wp->w_valid_leftcol = wp->w_leftcol;
	wp->w_valid_skipcol = wp->w_skipcol;
    }
    else if (wp->w_cursor.col != wp->w_valid_cursor.col
	     || wp->w_leftcol != wp->w_valid_leftcol
	     || wp->w_cursor.coladd != wp->w_valid_cursor.coladd)
    {
	wp->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL);
	wp->w_valid_cursor.col = wp->w_cursor.col;
	wp->w_valid_leftcol = wp->w_leftcol;
	wp->w_valid_cursor.coladd = wp->w_cursor.coladd;
    }
}

/*
 * Call this function when some window settings have changed, which require
 * the cursor position, botline and topline to be recomputed and the window to
 * be redrawn.  E.g, when changing the 'wrap' option or folding.
 */
    void
changed_window_setting(void)
{
    changed_window_setting_win(curwin);
}

    void
changed_window_setting_win(win_T *wp)
{
    wp->w_lines_valid = 0;
    changed_line_abv_curs_win(wp);
    wp->w_valid &= ~(VALID_BOTLINE|VALID_BOTLINE_AP|VALID_TOPLINE);
    redraw_win_later(wp, UPD_NOT_VALID);
}

#if defined(FEAT_PROP_POPUP) || defined(PROTO)
/*
 * Call changed_window_setting_win() for every window containing "buf".
 */
    void
changed_window_setting_buf(buf_T *buf)
{
    tabpage_T	*tp;
    win_T	*wp;

    FOR_ALL_TAB_WINDOWS(tp, wp)
	if (wp->w_buffer == buf)
	    changed_window_setting_win(wp);
}
#endif

/*
 * Set wp->w_topline to a certain number.
 */
    void
set_topline(win_T *wp, linenr_T lnum)
{
#ifdef FEAT_DIFF
    linenr_T prev_topline = wp->w_topline;
#endif

#ifdef FEAT_FOLDING
    // go to first of folded lines
    (void)hasFoldingWin(wp, lnum, &lnum, NULL, TRUE, NULL);
#endif
    // Approximate the value of w_botline
    wp->w_botline += lnum - wp->w_topline;
    if (wp->w_botline > wp->w_buffer->b_ml.ml_line_count + 1)
	wp->w_botline = wp->w_buffer->b_ml.ml_line_count + 1;
    wp->w_topline = lnum;
    wp->w_topline_was_set = TRUE;
#ifdef FEAT_DIFF
    if (lnum != prev_topline)
	// Keep the filler lines when the topline didn't change.
	wp->w_topfill = 0;
#endif
    wp->w_valid &= ~(VALID_WROW|VALID_CROW|VALID_BOTLINE|VALID_TOPLINE);
    // Don't set VALID_TOPLINE here, 'scrolloff' needs to be checked.
    redraw_later(UPD_VALID);
}

/*
 * Call this function when the length of the cursor line (in screen
 * characters) has changed, and the change is before the cursor.
 * If the line length changed the number of screen lines might change,
 * requiring updating w_topline.  That may also invalidate w_crow.
 * Need to take care of w_botline separately!
 */
    void
changed_cline_bef_curs(void)
{
    curwin->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL|VALID_CROW
						|VALID_CHEIGHT|VALID_TOPLINE);
}

    void
changed_cline_bef_curs_win(win_T *wp)
{
    wp->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL|VALID_CROW
						|VALID_CHEIGHT|VALID_TOPLINE);
}

/*
 * Call this function when the length of a line (in screen characters) above
 * the cursor have changed.
 * Need to take care of w_botline separately!
 */
    void
changed_line_abv_curs(void)
{
    curwin->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL|VALID_CROW
						|VALID_CHEIGHT|VALID_TOPLINE);
}

    void
changed_line_abv_curs_win(win_T *wp)
{
    wp->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL|VALID_CROW
						|VALID_CHEIGHT|VALID_TOPLINE);
}

#if defined(FEAT_PROP_POPUP) || defined(PROTO)
/*
 * Display of line has changed for "buf", invalidate cursor position and
 * w_botline.
 */
    void
changed_line_display_buf(buf_T *buf)
{
    win_T *wp;

    FOR_ALL_WINDOWS(wp)
	if (wp->w_buffer == buf)
	    wp->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL
				|VALID_CROW|VALID_CHEIGHT
				|VALID_TOPLINE|VALID_BOTLINE|VALID_BOTLINE_AP);
}
#endif

/*
 * Make sure the value of curwin->w_botline is valid.
 */
    void
validate_botline(void)
{
    validate_botline_win(curwin);
}

/*
 * Make sure the value of wp->w_botline is valid.
 */
    void
validate_botline_win(win_T *wp)
{
    if (!(wp->w_valid & VALID_BOTLINE))
	comp_botline(wp);
}

/*
 * Mark curwin->w_botline as invalid (because of some change in the buffer).
 */
    void
invalidate_botline(void)
{
    curwin->w_valid &= ~(VALID_BOTLINE|VALID_BOTLINE_AP);
}

    void
invalidate_botline_win(win_T *wp)
{
    wp->w_valid &= ~(VALID_BOTLINE|VALID_BOTLINE_AP);
}

    void
approximate_botline_win(
    win_T	*wp)
{
    wp->w_valid &= ~VALID_BOTLINE;
}

/*
 * Return TRUE if curwin->w_wrow and curwin->w_wcol are valid.
 */
    int
cursor_valid(void)
{
    check_cursor_moved(curwin);
    return ((curwin->w_valid & (VALID_WROW|VALID_WCOL)) ==
						      (VALID_WROW|VALID_WCOL));
}

/*
 * Validate cursor position.  Makes sure w_wrow and w_wcol are valid.
 * w_topline must be valid, you may need to call update_topline() first!
 */
    void
validate_cursor(void)
{
    check_cursor_lnum();
    check_cursor_moved(curwin);
    if ((curwin->w_valid & (VALID_WCOL|VALID_WROW)) != (VALID_WCOL|VALID_WROW))
	curs_columns(TRUE);
}

#if defined(FEAT_GUI) || defined(PROTO)
/*
 * validate w_cline_row.
 */
    void
validate_cline_row(void)
{
    /*
     * First make sure that w_topline is valid (after moving the cursor).
     */
    update_topline();
    check_cursor_moved(curwin);
    if (!(curwin->w_valid & VALID_CROW))
	curs_rows(curwin);
}
#endif

/*
 * Compute wp->w_cline_row and wp->w_cline_height, based on the current value
 * of wp->w_topline.
 */
    static void
curs_rows(win_T *wp)
{
    linenr_T	lnum;
    int		i;
    int		all_invalid;
    int		valid;
#ifdef FEAT_FOLDING
    long	fold_count;
#endif

    // Check if wp->w_lines[].wl_size is invalid
    all_invalid = (!redrawing()
			|| wp->w_lines_valid == 0
			|| wp->w_lines[0].wl_lnum > wp->w_topline);
    i = 0;
    wp->w_cline_row = 0;
    for (lnum = wp->w_topline; lnum < wp->w_cursor.lnum; ++i)
    {
	valid = FALSE;
	if (!all_invalid && i < wp->w_lines_valid)
	{
	    if (wp->w_lines[i].wl_lnum < lnum || !wp->w_lines[i].wl_valid)
		continue;		// skip changed or deleted lines
	    if (wp->w_lines[i].wl_lnum == lnum)
	    {
#ifdef FEAT_FOLDING
		// Check for newly inserted lines below this row, in which
		// case we need to check for folded lines.
		if (!wp->w_buffer->b_mod_set
			|| wp->w_lines[i].wl_lastlnum < wp->w_cursor.lnum
			|| wp->w_buffer->b_mod_top
					     > wp->w_lines[i].wl_lastlnum + 1)
#endif
		valid = TRUE;
	    }
	    else if (wp->w_lines[i].wl_lnum > lnum)
		--i;			// hold at inserted lines
	}
	if (valid
	       && (lnum != wp->w_topline || (wp->w_skipcol == 0
#ifdef FEAT_DIFF
					    && !wp->w_p_diff
#endif
	       )))
	{
#ifdef FEAT_FOLDING
	    lnum = wp->w_lines[i].wl_lastlnum + 1;
	    // Cursor inside folded lines, don't count this row
	    if (lnum > wp->w_cursor.lnum)
		break;
#else
	    ++lnum;
#endif
	    wp->w_cline_row += wp->w_lines[i].wl_size;
	}
	else
	{
#ifdef FEAT_FOLDING
	    fold_count = foldedCount(wp, lnum, NULL);
	    if (fold_count)
	    {
		lnum += fold_count;
		if (lnum > wp->w_cursor.lnum)
		    break;
		++wp->w_cline_row;
	    }
	    else
#endif
	    {
		wp->w_cline_row += plines_correct_topline(wp, lnum);
		++lnum;
	    }
	}
    }

    check_cursor_moved(wp);
    if (!(wp->w_valid & VALID_CHEIGHT))
    {
	if (all_invalid
		|| i == wp->w_lines_valid
		|| (i < wp->w_lines_valid
		    && (!wp->w_lines[i].wl_valid
			|| wp->w_lines[i].wl_lnum != wp->w_cursor.lnum)))
	{
#ifdef FEAT_DIFF
	    if (wp->w_cursor.lnum == wp->w_topline)
		wp->w_cline_height = plines_win_nofill(wp, wp->w_cursor.lnum,
							TRUE) + wp->w_topfill;
	    else
#endif
		wp->w_cline_height = plines_win(wp, wp->w_cursor.lnum, TRUE);
#ifdef FEAT_FOLDING
	    wp->w_cline_folded = hasFoldingWin(wp, wp->w_cursor.lnum,
						      NULL, NULL, TRUE, NULL);
#endif
	}
	else if (i > wp->w_lines_valid)
	{
	    // a line that is too long to fit on the last screen line
	    wp->w_cline_height = 0;
#ifdef FEAT_FOLDING
	    wp->w_cline_folded = hasFoldingWin(wp, wp->w_cursor.lnum,
						      NULL, NULL, TRUE, NULL);
#endif
	}
	else
	{
	    wp->w_cline_height = wp->w_lines[i].wl_size;
#ifdef FEAT_FOLDING
	    wp->w_cline_folded = wp->w_lines[i].wl_folded;
#endif
	}
    }

    redraw_for_cursorline(curwin);
    wp->w_valid |= VALID_CROW|VALID_CHEIGHT;
}

/*
 * Validate curwin->w_virtcol only.
 */
    void
validate_virtcol(void)
{
    validate_virtcol_win(curwin);
}

/*
 * Validate wp->w_virtcol only.
 */
    void
validate_virtcol_win(win_T *wp)
{
    check_cursor_moved(wp);

    if (wp->w_valid & VALID_VIRTCOL)
	return;

#ifdef FEAT_PROP_POPUP
    wp->w_virtcol_first_char = 0;
#endif
    getvvcol(wp, &wp->w_cursor, NULL, &(wp->w_virtcol), NULL);
#ifdef FEAT_SYN_HL
    redraw_for_cursorcolumn(wp);
#endif
    wp->w_valid |= VALID_VIRTCOL;
}

/*
 * Validate curwin->w_cline_height only.
 */
    void
validate_cheight(void)
{
    check_cursor_moved(curwin);

    if (curwin->w_valid & VALID_CHEIGHT)
	return;

#ifdef FEAT_DIFF
    if (curwin->w_cursor.lnum == curwin->w_topline)
	curwin->w_cline_height = plines_nofill(curwin->w_cursor.lnum)
	    + curwin->w_topfill;
    else
#endif
	curwin->w_cline_height = plines(curwin->w_cursor.lnum);
#ifdef FEAT_FOLDING
    curwin->w_cline_folded = hasFolding(curwin->w_cursor.lnum, NULL, NULL);
#endif
    curwin->w_valid |= VALID_CHEIGHT;
}

/*
 * Validate w_wcol and w_virtcol only.
 */
    void
validate_cursor_col(void)
{
    colnr_T off;
    colnr_T col;
    int     width;

    validate_virtcol();

    if (curwin->w_valid & VALID_WCOL)
	return;

    col = curwin->w_virtcol;
    off = curwin_col_off();
    col += off;
    width = curwin->w_width - off + curwin_col_off2();

    // long line wrapping, adjust curwin->w_wrow
    if (curwin->w_p_wrap
	    && col >= (colnr_T)curwin->w_width
	    && width > 0)
	// use same formula as what is used in curs_columns()
	col -= ((col - curwin->w_width) / width + 1) * width;
    if (col > (int)curwin->w_leftcol)
	col -= curwin->w_leftcol;
    else
	col = 0;
    curwin->w_wcol = col;

    curwin->w_valid |= VALID_WCOL;
#ifdef FEAT_PROP_POPUP
    curwin->w_flags &= ~WFLAG_WCOL_OFF_ADDED;
#endif
}

/*
 * Compute offset of a window, occupied by absolute or relative line number,
 * fold column and sign column (these don't move when scrolling horizontally).
 */
    int
win_col_off(win_T *wp)
{
    return (((wp->w_p_nu || wp->w_p_rnu) ? number_width(wp) + 1 : 0)
	    + (wp != cmdwin_win ? 0 : 1)
#ifdef FEAT_FOLDING
	    + wp->w_p_fdc
#endif
#ifdef FEAT_SIGNS
	    + (signcolumn_on(wp) ? 2 : 0)
#endif
	   );
}

    int
curwin_col_off(void)
{
    return win_col_off(curwin);
}

/*
 * Return the difference in column offset for the second screen line of a
 * wrapped line.  It's positive if 'number' or 'relativenumber' is on and 'n'
 * is in 'cpoptions'.
 */
    int
win_col_off2(win_T *wp)
{
    if ((wp->w_p_nu || wp->w_p_rnu) && vim_strchr(p_cpo, CPO_NUMCOL) != NULL)
	return number_width(wp) + 1;
    return 0;
}

    int
curwin_col_off2(void)
{
    return win_col_off2(curwin);
}

/*
 * Compute curwin->w_wcol and curwin->w_virtcol.
 * Also updates curwin->w_wrow and curwin->w_cline_row.
 * Also updates curwin->w_leftcol.
 */
    void
curs_columns(
    int		may_scroll)	// when TRUE, may scroll horizontally
{
    int		diff;
    int		extra;		// offset for first screen line
    int		off_left, off_right;
    int		n;
    int		p_lines;
    int		width1;		// text width for first screen line
    int		width2 = 0;	// text width for second and later screen line
    int		new_leftcol;
    colnr_T	startcol;
    colnr_T	endcol;
    colnr_T	prev_skipcol;
    long	so = get_scrolloff_value();
    long	siso = get_sidescrolloff_value();
    int		did_sub_skipcol = FALSE;

    /*
     * First make sure that w_topline is valid (after moving the cursor).
     */
    update_topline();

    /*
     * Next make sure that w_cline_row is valid.
     */
    if (!(curwin->w_valid & VALID_CROW))
	curs_rows(curwin);

#ifdef FEAT_PROP_POPUP
    // will be set by getvvcol() but not reset
    curwin->w_virtcol_first_char = 0;
#endif

    /*
     * Compute the number of virtual columns.
     */
#ifdef FEAT_FOLDING
    if (curwin->w_cline_folded)
	// In a folded line the cursor is always in the first column
	startcol = curwin->w_virtcol = endcol = curwin->w_leftcol;
    else
#endif
	getvvcol(curwin, &curwin->w_cursor,
				&startcol, &(curwin->w_virtcol), &endcol);

    // remove '$' from change command when cursor moves onto it
    if (startcol > dollar_vcol)
	dollar_vcol = -1;

    extra = curwin_col_off();
    curwin->w_wcol = curwin->w_virtcol + extra;
    endcol += extra;

    /*
     * Now compute w_wrow, counting screen lines from w_cline_row.
     */
    curwin->w_wrow = curwin->w_cline_row;

    width1 = curwin->w_width - extra;
    if (width1 <= 0)
    {
	// No room for text, put cursor in last char of window.
	// If not wrapping, the last non-empty line.
	curwin->w_wcol = curwin->w_width - 1;
	if (curwin->w_p_wrap)
	    curwin->w_wrow = curwin->w_height - 1;
	else
	    curwin->w_wrow = curwin->w_height - 1 - curwin->w_empty_rows;
    }
    else if (curwin->w_p_wrap && curwin->w_width != 0)
    {
	width2 = width1 + curwin_col_off2();

	// skip columns that are not visible
	if (curwin->w_cursor.lnum == curwin->w_topline
		&& curwin->w_skipcol > 0
		&& curwin->w_wcol >= curwin->w_skipcol)
	{
	    // Deduct by multiples of width2.  This allows the long line
	    // wrapping formula below to correctly calculate the w_wcol value
	    // when wrapping.
	    if (curwin->w_skipcol <= width1)
		curwin->w_wcol -= width2;
	    else
		curwin->w_wcol -= width2
			       * (((curwin->w_skipcol - width1) / width2) + 1);

	    did_sub_skipcol = TRUE;
	}

	// long line wrapping, adjust curwin->w_wrow
	if (curwin->w_wcol >= curwin->w_width)
	{
	    // this same formula is used in validate_cursor_col()
	    n = (curwin->w_wcol - curwin->w_width) / width2 + 1;
	    curwin->w_wcol -= n * width2;
	    curwin->w_wrow += n;
	}
    }

    // No line wrapping: compute curwin->w_leftcol if scrolling is on and line
    // is not folded.
    // If scrolling is off, curwin->w_leftcol is assumed to be 0
    else if (may_scroll
#ifdef FEAT_FOLDING
	    && !curwin->w_cline_folded
#endif
	    )
    {
#ifdef FEAT_PROP_POPUP
	if (curwin->w_virtcol_first_char > 0)
	{
	    int cols = (curwin->w_width - extra);
	    int rows = cols > 0 ? curwin->w_virtcol_first_char / cols : 1;

	    // each "above" text prop shifts the text one row down
	    curwin->w_wrow += rows;
	    curwin->w_wcol -= rows * cols;
	    endcol -= rows * cols;
	    curwin->w_cline_height = rows + 1;
	}
#endif
	/*
	 * If Cursor is left of the screen, scroll rightwards.
	 * If Cursor is right of the screen, scroll leftwards
	 * If we get closer to the edge than 'sidescrolloff', scroll a little
	 * extra
	 */
	off_left = (int)startcol - (int)curwin->w_leftcol - siso;
	off_right = (int)endcol - (int)(curwin->w_leftcol + curwin->w_width
								- siso) + 1;
	if (off_left < 0 || off_right > 0)
	{
	    if (off_left < 0)
		diff = -off_left;
	    else
		diff = off_right;

	    // When far off or not enough room on either side, put cursor in
	    // middle of window.
	    if (p_ss == 0 || diff >= width1 / 2 || off_right >= off_left)
		new_leftcol = curwin->w_wcol - extra - width1 / 2;
	    else
	    {
		if (diff < p_ss)
		    diff = p_ss;
		if (off_left < 0)
		    new_leftcol = curwin->w_leftcol - diff;
		else
		    new_leftcol = curwin->w_leftcol + diff;
	    }
	    if (new_leftcol < 0)
		new_leftcol = 0;
	    if (new_leftcol != (int)curwin->w_leftcol)
	    {
		curwin->w_leftcol = new_leftcol;
		// screen has to be redrawn with new curwin->w_leftcol
		redraw_later(UPD_NOT_VALID);
	    }
	}
	curwin->w_wcol -= curwin->w_leftcol;
    }
    else if (curwin->w_wcol > (int)curwin->w_leftcol)
	curwin->w_wcol -= curwin->w_leftcol;
    else
	curwin->w_wcol = 0;

#ifdef FEAT_DIFF
    // Skip over filler lines.  At the top use w_topfill, there
    // may be some filler lines above the window.
    if (curwin->w_cursor.lnum == curwin->w_topline)
	curwin->w_wrow += curwin->w_topfill;
    else
	curwin->w_wrow += diff_check_fill(curwin, curwin->w_cursor.lnum);
#endif

    prev_skipcol = curwin->w_skipcol;

    p_lines = 0;

    if ((curwin->w_wrow >= curwin->w_height
		|| ((prev_skipcol > 0
			|| curwin->w_wrow + so >= curwin->w_height)
		    && (p_lines =
#ifdef FEAT_DIFF
			plines_win_nofill
#else
			plines_win
#endif
			(curwin, curwin->w_cursor.lnum, FALSE))
						    - 1 >= curwin->w_height))
	    && curwin->w_height != 0
	    && curwin->w_cursor.lnum == curwin->w_topline
	    && width2 > 0
	    && curwin->w_width != 0)
    {
	// Cursor past end of screen.  Happens with a single line that does
	// not fit on screen.  Find a skipcol to show the text around the
	// cursor.  Avoid scrolling all the time. compute value of "extra":
	// 1: Less than 'scrolloff' lines above
	// 2: Less than 'scrolloff' lines below
	// 3: both of them
	extra = 0;
	if (curwin->w_skipcol + so * width2 > curwin->w_virtcol)
	    extra = 1;
	// Compute last display line of the buffer line that we want at the
	// bottom of the window.
	if (p_lines == 0)
	    p_lines = plines_win(curwin, curwin->w_cursor.lnum, FALSE);
	--p_lines;
	if (p_lines > curwin->w_wrow + so)
	    n = curwin->w_wrow + so;
	else
	    n = p_lines;
	if ((colnr_T)n >= curwin->w_height + curwin->w_skipcol / width2 - so)
	    extra += 2;

	if (extra == 3 || curwin->w_height <= so * 2)
	{
	    // not enough room for 'scrolloff', put cursor in the middle
	    n = curwin->w_virtcol / width2;
	    if (n > curwin->w_height / 2)
		n -= curwin->w_height / 2;
	    else
		n = 0;
	    // don't skip more than necessary
	    if (n > p_lines - curwin->w_height + 1)
		n = p_lines - curwin->w_height + 1;
	    if (n > 0)
		curwin->w_skipcol = width1 + (n - 1) * width2;
	    else
		curwin->w_skipcol = 0;
	}
	else if (extra == 1)
	{
	    // less than 'scrolloff' lines above, decrease skipcol
	    extra = (curwin->w_skipcol + so * width2 - curwin->w_virtcol
				     + width2 - 1) / width2;
	    if (extra > 0)
	    {
		if ((colnr_T)(extra * width2) > curwin->w_skipcol)
		    extra = curwin->w_skipcol / width2;
		curwin->w_skipcol -= extra * width2;
	    }
	}
	else if (extra == 2)
	{
	    // less than 'scrolloff' lines below, increase skipcol
	    endcol = (n - curwin->w_height + 1) * width2;
	    while (endcol > curwin->w_virtcol)
		endcol -= width2;
	    if (endcol > curwin->w_skipcol)
		curwin->w_skipcol = endcol;
	}

	// adjust w_wrow for the changed w_skipcol
	if (did_sub_skipcol)
	    curwin->w_wrow -= (curwin->w_skipcol - prev_skipcol) / width2;
	else
	    curwin->w_wrow -= curwin->w_skipcol / width2;

	if (curwin->w_wrow >= curwin->w_height)
	{
	    // small window, make sure cursor is in it
	    extra = curwin->w_wrow - curwin->w_height + 1;
	    curwin->w_skipcol += extra * width2;
	    curwin->w_wrow -= extra;
	}

	extra = ((int)prev_skipcol - (int)curwin->w_skipcol) / width2;
	if (extra > 0)
	    win_ins_lines(curwin, 0, extra, FALSE, FALSE);
	else if (extra < 0)
	    win_del_lines(curwin, 0, -extra, FALSE, FALSE, 0);
    }
    else if (!curwin->w_p_sms)
	curwin->w_skipcol = 0;
    if (prev_skipcol != curwin->w_skipcol)
	redraw_later(UPD_SOME_VALID);

#ifdef FEAT_SYN_HL
    redraw_for_cursorcolumn(curwin);
#endif
#if defined(FEAT_PROP_POPUP) && defined(FEAT_TERMINAL)
    if (popup_is_popup(curwin) && curbuf->b_term != NULL)
    {
	curwin->w_wrow += popup_top_extra(curwin);
	curwin->w_wcol += popup_left_extra(curwin);
	curwin->w_flags |= WFLAG_WCOL_OFF_ADDED + WFLAG_WROW_OFF_ADDED;
    }
    else
	curwin->w_flags &= ~(WFLAG_WCOL_OFF_ADDED + WFLAG_WROW_OFF_ADDED);
#endif

    // now w_leftcol and w_skipcol are valid, avoid check_cursor_moved()
    // thinking otherwise
    curwin->w_valid_leftcol = curwin->w_leftcol;
    curwin->w_valid_skipcol = curwin->w_skipcol;

    curwin->w_valid |= VALID_WCOL|VALID_WROW|VALID_VIRTCOL;
}

#if (defined(FEAT_EVAL) || defined(FEAT_PROP_POPUP)) || defined(PROTO)
/*
 * Compute the screen position of text character at "pos" in window "wp"
 * The resulting values are one-based, zero when character is not visible.
 */
    void
textpos2screenpos(
	win_T	*wp,
	pos_T	*pos,
	int	*rowp,	// screen row
	int	*scolp,	// start screen column
	int	*ccolp,	// cursor screen column
	int	*ecolp)	// end screen column
{
    colnr_T	scol = 0, ccol = 0, ecol = 0;
    int		row = 0;
    colnr_T	coloff = 0;

    if (pos->lnum >= wp->w_topline && pos->lnum <= wp->w_botline)
    {
	colnr_T	    col;
	int	    width;
	linenr_T    lnum = pos->lnum;
#ifdef FEAT_FOLDING
	int	    is_folded;

	is_folded = hasFoldingWin(wp, lnum, &lnum, NULL, TRUE, NULL);
#endif
	row = plines_m_win(wp, wp->w_topline, lnum - 1, FALSE);
	// "row" should be the screen line where line "lnum" begins, which can
	// be negative if "lnum" is "w_topline" and "w_skipcol" is non-zero.
	row -= adjust_plines_for_skipcol(wp);

#ifdef FEAT_DIFF
	// Add filler lines above this buffer line.
	row += lnum == wp->w_topline ? wp->w_topfill
				     : diff_check_fill(wp, lnum);
#endif

	colnr_T	off = win_col_off(wp);
#ifdef FEAT_FOLDING
	if (is_folded)
	{
	    row += W_WINROW(wp) + 1;
	    coloff = wp->w_wincol + 1 + off;
	}
	else
#endif
	{
	    getvcol(wp, pos, &scol, &ccol, &ecol);

	    // similar to what is done in validate_cursor_col()
	    col = scol;
	    col += off;
	    width = wp->w_width - off + win_col_off2(wp);

	    // long line wrapping, adjust row
	    if (wp->w_p_wrap
		    && col >= (colnr_T)wp->w_width
		    && width > 0)
	    {
		// use same formula as what is used in curs_columns()
		int rowoff = ((col - wp->w_width) / width + 1);
		col -= rowoff * width;
		row += rowoff;
	    }
	    col -= wp->w_leftcol;
	    if (col >= wp->w_width)
		col = -1;
	    if (col >= 0 && row >= 0 && row < wp->w_height)
	    {
		coloff = col - scol + wp->w_wincol + 1;
		row += W_WINROW(wp) + 1;
	    }
	    else
		// character is out of the window
		row = scol = ccol = ecol = 0;
	}
    }
    *rowp = row;
    *scolp = scol + coloff;
    *ccolp = ccol + coloff;
    *ecolp = ecol + coloff;
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * "screenpos({winid}, {lnum}, {col})" function
 */
    void
f_screenpos(typval_T *argvars UNUSED, typval_T *rettv)
{
    dict_T	*dict;
    win_T	*wp;
    pos_T	pos;
    int		row = 0;
    int		scol = 0, ccol = 0, ecol = 0;

    if (rettv_dict_alloc(rettv) == FAIL)
	return;
    dict = rettv->vval.v_dict;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL
		|| check_for_number_arg(argvars, 2) == FAIL))
	return;

    wp = find_win_by_nr_or_id(&argvars[0]);
    if (wp == NULL)
	return;

    pos.lnum = tv_get_number(&argvars[1]);
    if (pos.lnum > wp->w_buffer->b_ml.ml_line_count)
    {
	semsg(_(e_invalid_line_number_nr), pos.lnum);
	return;
    }
    pos.col = tv_get_number(&argvars[2]) - 1;
    if (pos.col < 0)
	pos.col = 0;
    pos.coladd = 0;
    textpos2screenpos(wp, &pos, &row, &scol, &ccol, &ecol);

    dict_add_number(dict, "row", row);
    dict_add_number(dict, "col", scol);
    dict_add_number(dict, "curscol", ccol);
    dict_add_number(dict, "endcol", ecol);
}

/*
 * Convert a virtual (screen) column to a character column.  The first column
 * is one.  For a multibyte character, the column number of the first byte is
 * returned.
 */
    static int
virtcol2col(win_T *wp, linenr_T lnum, int vcol)
{
    int		offset = vcol2col(wp, lnum, vcol - 1, NULL);
    char_u	*line = ml_get_buf(wp->w_buffer, lnum, FALSE);
    char_u	*p = line + offset;

    if (*p == NUL)
    {
	if (p == line)  // empty line
	    return 0;
	// Move to the first byte of the last char.
	MB_PTR_BACK(line, p);
    }
    return (int)(p - line + 1);
}

/*
 * "virtcol2col({winid}, {lnum}, {col})" function
 */
    void
f_virtcol2col(typval_T *argvars UNUSED, typval_T *rettv)
{
    win_T	*wp;
    linenr_T	lnum;
    int		screencol;
    int		error = FALSE;

    rettv->vval.v_number = -1;

    if (check_for_number_arg(argvars, 0) == FAIL
	    || check_for_number_arg(argvars, 1) == FAIL
	    || check_for_number_arg(argvars, 2) == FAIL)
	return;

    wp = find_win_by_nr_or_id(&argvars[0]);
    if (wp == NULL)
	return;

    lnum = tv_get_number_chk(&argvars[1], &error);
    if (error || lnum < 0 || lnum > wp->w_buffer->b_ml.ml_line_count)
	return;

    screencol = tv_get_number_chk(&argvars[2], &error);
    if (error || screencol < 0)
	return;

    rettv->vval.v_number = virtcol2col(wp, lnum, screencol);
}
#endif

/*
 * Scroll the current window down by "line_count" logical lines.  "CTRL-Y"
 */
    void
scrolldown(
    long	line_count,
    int		byfold UNUSED)	// TRUE: count a closed fold as one line
{
    long	done = 0;	// total # of physical lines done
    int		wrow;
    int		moved = FALSE;
    int		do_sms = curwin->w_p_wrap && curwin->w_p_sms;
    int		width1 = 0;
    int		width2 = 0;

    if (do_sms)
    {
	width1 = curwin->w_width - curwin_col_off();
	width2 = width1 + curwin_col_off2();
    }

#ifdef FEAT_FOLDING
    linenr_T	first;

    // Make sure w_topline is at the first of a sequence of folded lines.
    (void)hasFolding(curwin->w_topline, &curwin->w_topline, NULL);
#endif
    validate_cursor();		// w_wrow needs to be valid
    for (int todo = line_count; todo > 0; --todo)
    {
#ifdef FEAT_DIFF
	if (curwin->w_topfill < diff_check(curwin, curwin->w_topline)
		&& curwin->w_topfill < curwin->w_height - 1)
	{
	    ++curwin->w_topfill;
	    ++done;
	}
	else
#endif
	{
	    // break when at the very top
	    if (curwin->w_topline == 1
				   && (!do_sms || curwin->w_skipcol < width1))
		break;
	    if (do_sms && curwin->w_skipcol >= width1)
	    {
		// scroll a screen line down
		if (curwin->w_skipcol >= width1 + width2)
		    curwin->w_skipcol -= width2;
		else
		    curwin->w_skipcol -= width1;
		redraw_later(UPD_NOT_VALID);
		++done;
	    }
	    else
	    {
		// scroll a text line down
		--curwin->w_topline;
		curwin->w_skipcol = 0;
#ifdef FEAT_DIFF
		curwin->w_topfill = 0;
#endif
#ifdef FEAT_FOLDING
		// A sequence of folded lines only counts for one logical line
		if (hasFolding(curwin->w_topline, &first, NULL))
		{
		    ++done;
		    if (!byfold)
			todo -= curwin->w_topline - first - 1;
		    curwin->w_botline -= curwin->w_topline - first;
		    curwin->w_topline = first;
		}
		else
#endif
		if (do_sms)
		{
		    int size = win_linetabsize(curwin, curwin->w_topline,
				   ml_get(curwin->w_topline), (colnr_T)MAXCOL);
		    if (size > width1)
		    {
			curwin->w_skipcol = width1;
			size -= width1;
			redraw_later(UPD_NOT_VALID);
		    }
		    while (size > width2)
		    {
			curwin->w_skipcol += width2;
			size -= width2;
		    }
		    ++done;
		}
		else
		    done += PLINES_NOFILL(curwin->w_topline);
	    }
	}
	--curwin->w_botline;		// approximate w_botline
	invalidate_botline();
    }
    curwin->w_wrow += done;		// keep w_wrow updated
    curwin->w_cline_row += done;	// keep w_cline_row updated

#ifdef FEAT_DIFF
    if (curwin->w_cursor.lnum == curwin->w_topline)
	curwin->w_cline_row = 0;
    check_topfill(curwin, TRUE);
#endif

    /*
     * Compute the row number of the last row of the cursor line
     * and move the cursor onto the displayed part of the window.
     */
    wrow = curwin->w_wrow;
    if (curwin->w_p_wrap && curwin->w_width != 0)
    {
	validate_virtcol();
	validate_cheight();
	wrow += curwin->w_cline_height - 1 -
	    curwin->w_virtcol / curwin->w_width;
    }
    while (wrow >= curwin->w_height && curwin->w_cursor.lnum > 1)
    {
#ifdef FEAT_FOLDING
	if (hasFolding(curwin->w_cursor.lnum, &first, NULL))
	{
	    --wrow;
	    if (first == 1)
		curwin->w_cursor.lnum = 1;
	    else
		curwin->w_cursor.lnum = first - 1;
	}
	else
#endif
	    wrow -= plines(curwin->w_cursor.lnum--);
	curwin->w_valid &=
	      ~(VALID_WROW|VALID_WCOL|VALID_CHEIGHT|VALID_CROW|VALID_VIRTCOL);
	moved = TRUE;
    }
    if (moved)
    {
#ifdef FEAT_FOLDING
	// Move cursor to first line of closed fold.
	foldAdjustCursor();
#endif
	coladvance(curwin->w_curswant);
    }

    if (curwin->w_cursor.lnum == curwin->w_topline && do_sms)
    {
	long	so = get_scrolloff_value();
	int	scrolloff_cols = so == 0 ? 0 : width1 + (so - 1) * width2;

	// make sure the cursor is in the visible text
	validate_virtcol();
	int col = curwin->w_virtcol - curwin->w_skipcol + scrolloff_cols;
	int row = 0;
	if (col >= width1)
	{
	    col -= width1;
	    ++row;
	}
	if (col > width2 && width2 > 0)
	{
	    row += col / width2;
	    // even so col is not used anymore,
	    // make sure it is correct, just in case
	    col = col % width2;
	}
	if (row >= curwin->w_height)
	{
	    curwin->w_curswant = curwin->w_virtcol
				       - (row - curwin->w_height + 1) * width2;
	    coladvance(curwin->w_curswant);
	}
    }
}

/*
 * Scroll the current window up by "line_count" logical lines.  "CTRL-E"
 */
    void
scrollup(
    long	line_count,
    int		byfold UNUSED)	// TRUE: count a closed fold as one line
{
    int		do_sms = curwin->w_p_wrap && curwin->w_p_sms;

    if (do_sms
# ifdef FEAT_FOLDING
	    || (byfold && hasAnyFolding(curwin))
# endif
# ifdef FEAT_DIFF
	    || (curwin->w_p_diff && !curwin->w_p_wrap)
# endif
       )
    {
	int	    width1 = curwin->w_width - curwin_col_off();
	int	    width2 = width1 + curwin_col_off2();
	int	    size = 0;
	colnr_T	    prev_skipcol = curwin->w_skipcol;

	if (do_sms)
	    size = linetabsize(curwin, curwin->w_topline);

	// diff mode: first consume "topfill"
	// 'smoothscroll': increase "w_skipcol" until it goes over the end of
	// the line, then advance to the next line.
	// folding: count each sequence of folded lines as one logical line.
	for (int todo = line_count; todo > 0; --todo)
	{
# ifdef FEAT_DIFF
	    if (curwin->w_topfill > 0)
		--curwin->w_topfill;
	    else
# endif
	    {
		linenr_T lnum = curwin->w_topline;

# ifdef FEAT_FOLDING
		if (byfold)
		    // for a closed fold: go to the last line in the fold
		    (void)hasFolding(lnum, NULL, &lnum);
# endif
		if (lnum == curwin->w_topline && do_sms)
		{
		    // 'smoothscroll': increase "w_skipcol" until it goes over
		    // the end of the line, then advance to the next line.
		    int add = curwin->w_skipcol > 0 ? width2 : width1;
		    curwin->w_skipcol += add;
		    if (curwin->w_skipcol >= size)
		    {
			if (lnum == curbuf->b_ml.ml_line_count)
			{
			    // at the last screen line, can't scroll further
			    curwin->w_skipcol -= add;
			    break;
			}
			++lnum;
		    }
		}
		else
		{
		    if (lnum >= curbuf->b_ml.ml_line_count)
			break;
		    ++lnum;
		}

		if (lnum > curwin->w_topline)
		{
		    // approximate w_botline
		    curwin->w_botline += lnum - curwin->w_topline;
		    curwin->w_topline = lnum;
# ifdef FEAT_DIFF
		    curwin->w_topfill = diff_check_fill(curwin, lnum);
# endif
		    curwin->w_skipcol = 0;
		    if (todo > 1 && do_sms)
			size = linetabsize(curwin, curwin->w_topline);
		}
	    }
	}

	if (prev_skipcol > 0 || curwin->w_skipcol > 0)
	    // need to redraw more, because wl_size of the (new) topline may
	    // now be invalid
	    redraw_later(UPD_NOT_VALID);
    }
    else
    {
	curwin->w_topline += line_count;
	curwin->w_botline += line_count;	// approximate w_botline
    }

    if (curwin->w_topline > curbuf->b_ml.ml_line_count)
	curwin->w_topline = curbuf->b_ml.ml_line_count;
    if (curwin->w_botline > curbuf->b_ml.ml_line_count + 1)
	curwin->w_botline = curbuf->b_ml.ml_line_count + 1;

#ifdef FEAT_DIFF
    check_topfill(curwin, FALSE);
#endif

#ifdef FEAT_FOLDING
    if (hasAnyFolding(curwin))
	// Make sure w_topline is at the first of a sequence of folded lines.
	(void)hasFolding(curwin->w_topline, &curwin->w_topline, NULL);
#endif

    curwin->w_valid &= ~(VALID_WROW|VALID_CROW|VALID_BOTLINE);
    if (curwin->w_cursor.lnum < curwin->w_topline)
    {
	curwin->w_cursor.lnum = curwin->w_topline;
	curwin->w_valid &=
	      ~(VALID_WROW|VALID_WCOL|VALID_CHEIGHT|VALID_CROW|VALID_VIRTCOL);
	coladvance(curwin->w_curswant);
    }
    if (curwin->w_cursor.lnum == curwin->w_topline
					    && do_sms && curwin->w_skipcol > 0)
    {
	int	col_off = curwin_col_off();
	int	col_off2 = curwin_col_off2();

	int	width1 = curwin->w_width - col_off;
	int	width2 = width1 + col_off2;
	int	extra2 = col_off - col_off2;
	long	so = get_scrolloff_value();
	int	scrolloff_cols = so == 0 ? 0 : width1 + (so - 1) * width2;
	int	space_cols = (curwin->w_height - 1) * width2;

	// If we have non-zero scrolloff, just ignore the marker as we are
	// going past it anyway.
	int overlap = scrolloff_cols != 0 ? 0
					  : sms_marker_overlap(curwin, extra2);

	// Make sure the cursor is in a visible part of the line, taking
	// 'scrolloff' into account, but using screen lines.
	// If there are not enough screen lines put the cursor in the middle.
	if (scrolloff_cols > space_cols / 2)
	    scrolloff_cols = space_cols / 2;
	validate_virtcol();
	if (curwin->w_virtcol < curwin->w_skipcol + overlap + scrolloff_cols)
	{
	    colnr_T col = curwin->w_virtcol;

	    if (col < width1)
		col += width1;
	    while (col < curwin->w_skipcol + overlap + scrolloff_cols)
		col += width2;
	    curwin->w_curswant = col;
	    coladvance(curwin->w_curswant);

	    // validate_virtcol() marked various things as valid, but after
	    // moving the cursor they need to be recomputed
	    curwin->w_valid &=
	       ~(VALID_WROW|VALID_WCOL|VALID_CHEIGHT|VALID_CROW|VALID_VIRTCOL);
	}
    }
}

/*
 * Called after changing the cursor column: make sure that curwin->w_skipcol is
 * valid for 'smoothscroll'.
 */
    void
adjust_skipcol(void)
{
    if (!curwin->w_p_wrap
	    || !curwin->w_p_sms
	    || curwin->w_cursor.lnum != curwin->w_topline)
	return;

    int	    width1 = curwin->w_width - curwin_col_off();
    if (width1 <= 0)
	return;  // no text will be displayed

    int	    width2 = width1 + curwin_col_off2();
    long    so = get_scrolloff_value();
    int	    scrolloff_cols = so == 0 ? 0 : width1 + (so - 1) * width2;
    int	    scrolled = FALSE;

    validate_cheight();
    if (curwin->w_cline_height == curwin->w_height
	    // w_cline_height may be capped at w_height, check there aren't
	    // actually more lines.
	    && plines_win(curwin, curwin->w_cursor.lnum, FALSE)
							   <= curwin->w_height)
    {
	// the line just fits in the window, don't scroll
	reset_skipcol();
	return;
    }

    validate_virtcol();
    int overlap = sms_marker_overlap(curwin,
					 curwin_col_off() - curwin_col_off2());
    while (curwin->w_skipcol > 0
	    && curwin->w_virtcol < curwin->w_skipcol + overlap + scrolloff_cols)
    {
	// scroll a screen line down
	if (curwin->w_skipcol >= width1 + width2)
	    curwin->w_skipcol -= width2;
	else
	    curwin->w_skipcol -= width1;
	scrolled = TRUE;
    }
    if (scrolled)
    {
	validate_virtcol();
	redraw_later(UPD_NOT_VALID);
	return;  // don't scroll in the other direction now
    }

    int col = curwin->w_virtcol - curwin->w_skipcol + scrolloff_cols;
    int row = 0;
    if (col >= width1)
    {
	col -= width1;
	++row;
    }
    if (col > width2)
    {
	row += col / width2;
	// col may no longer be used, but make
	// sure it is correct anyhow, just in case
	col = col % width2;
    }
    if (row >= curwin->w_height)
    {
	if (curwin->w_skipcol == 0)
	{
	    curwin->w_skipcol += width1;
	    --row;
	}
	if (row >= curwin->w_height)
	    curwin->w_skipcol += (row - curwin->w_height) * width2;
	redraw_later(UPD_NOT_VALID);
    }
}

#ifdef FEAT_DIFF
/*
 * Don't end up with too many filler lines in the window.
 */
    void
check_topfill(
    win_T	*wp,
    int		down)	// when TRUE scroll down when not enough space
{
    int		n;

    if (wp->w_topfill <= 0)
	return;

    n = plines_win_nofill(wp, wp->w_topline, TRUE);
    if (wp->w_topfill + n > wp->w_height)
    {
	if (down && wp->w_topline > 1)
	{
	    --wp->w_topline;
	    wp->w_topfill = 0;
	}
	else
	{
	    wp->w_topfill = wp->w_height - n;
	    if (wp->w_topfill < 0)
		wp->w_topfill = 0;
	}
    }
}

/*
 * Use as many filler lines as possible for w_topline.  Make sure w_topline
 * is still visible.
 */
    static void
max_topfill(void)
{
    int		n;

    n = plines_nofill(curwin->w_topline);
    if (n >= curwin->w_height)
	curwin->w_topfill = 0;
    else
    {
	curwin->w_topfill = diff_check_fill(curwin, curwin->w_topline);
	if (curwin->w_topfill + n > curwin->w_height)
	    curwin->w_topfill = curwin->w_height - n;
    }
}
#endif

/*
 * Scroll the screen one line down, but don't do it if it would move the
 * cursor off the screen.
 */
    void
scrolldown_clamp(void)
{
    int		end_row;
#ifdef FEAT_DIFF
    int		can_fill = (curwin->w_topfill
				< diff_check_fill(curwin, curwin->w_topline));
#endif

    if (curwin->w_topline <= 1
#ifdef FEAT_DIFF
	    && !can_fill
#endif
	    )
	return;

    validate_cursor();	    // w_wrow needs to be valid

    /*
     * Compute the row number of the last row of the cursor line
     * and make sure it doesn't go off the screen. Make sure the cursor
     * doesn't go past 'scrolloff' lines from the screen end.
     */
    end_row = curwin->w_wrow;
#ifdef FEAT_DIFF
    if (can_fill)
	++end_row;
    else
	end_row += plines_nofill(curwin->w_topline - 1);
#else
    end_row += plines(curwin->w_topline - 1);
#endif
    if (curwin->w_p_wrap && curwin->w_width != 0)
    {
	validate_cheight();
	validate_virtcol();
	end_row += curwin->w_cline_height - 1 -
	    curwin->w_virtcol / curwin->w_width;
    }
    if (end_row < curwin->w_height - get_scrolloff_value())
    {
#ifdef FEAT_DIFF
	if (can_fill)
	{
	    ++curwin->w_topfill;
	    check_topfill(curwin, TRUE);
	}
	else
	{
	    --curwin->w_topline;
	    curwin->w_topfill = 0;
	}
#else
	--curwin->w_topline;
#endif
#ifdef FEAT_FOLDING
	(void)hasFolding(curwin->w_topline, &curwin->w_topline, NULL);
#endif
	--curwin->w_botline;	    // approximate w_botline
	curwin->w_valid &= ~(VALID_WROW|VALID_CROW|VALID_BOTLINE);
    }
}

/*
 * Scroll the screen one line up, but don't do it if it would move the cursor
 * off the screen.
 */
    void
scrollup_clamp(void)
{
    int	    start_row;

    if (curwin->w_topline == curbuf->b_ml.ml_line_count
#ifdef FEAT_DIFF
	    && curwin->w_topfill == 0
#endif
	    )
	return;

    validate_cursor();	    // w_wrow needs to be valid

    /*
     * Compute the row number of the first row of the cursor line
     * and make sure it doesn't go off the screen. Make sure the cursor
     * doesn't go before 'scrolloff' lines from the screen start.
     */
#ifdef FEAT_DIFF
    start_row = curwin->w_wrow - plines_nofill(curwin->w_topline)
							  - curwin->w_topfill;
#else
    start_row = curwin->w_wrow - plines(curwin->w_topline);
#endif
    if (curwin->w_p_wrap && curwin->w_width != 0)
    {
	validate_virtcol();
	start_row -= curwin->w_virtcol / curwin->w_width;
    }
    if (start_row >= get_scrolloff_value())
    {
#ifdef FEAT_DIFF
	if (curwin->w_topfill > 0)
	    --curwin->w_topfill;
	else
#endif
	{
#ifdef FEAT_FOLDING
	    (void)hasFolding(curwin->w_topline, NULL, &curwin->w_topline);
#endif
	    ++curwin->w_topline;
	}
	++curwin->w_botline;		// approximate w_botline
	curwin->w_valid &= ~(VALID_WROW|VALID_CROW|VALID_BOTLINE);
    }
}

/*
 * Add one line above "lp->lnum".  This can be a filler line, a closed fold or
 * a (wrapped) text line.  Uses and sets "lp->fill".
 * Returns the height of the added line in "lp->height".
 * Lines above the first one are incredibly high: MAXCOL.
 */
    static void
topline_back_winheight(
    lineoff_T	*lp,
    int		winheight)	// when TRUE limit to window height
{
#ifdef FEAT_DIFF
    if (lp->fill < diff_check_fill(curwin, lp->lnum))
    {
	// Add a filler line.
	++lp->fill;
	lp->height = 1;
    }
    else
#endif
    {
	--lp->lnum;
#ifdef FEAT_DIFF
	lp->fill = 0;
#endif
	if (lp->lnum < 1)
	    lp->height = MAXCOL;
	else
#ifdef FEAT_FOLDING
	    if (hasFolding(lp->lnum, &lp->lnum, NULL))
	    // Add a closed fold
	    lp->height = 1;
	else
#endif
	    lp->height = PLINES_WIN_NOFILL(curwin, lp->lnum, winheight);
    }
}

    static void
topline_back(lineoff_T *lp)
{
    topline_back_winheight(lp, TRUE);
}


/*
 * Add one line below "lp->lnum".  This can be a filler line, a closed fold or
 * a (wrapped) text line.  Uses and sets "lp->fill".
 * Returns the height of the added line in "lp->height".
 * Lines below the last one are incredibly high.
 */
    static void
botline_forw(lineoff_T *lp)
{
#ifdef FEAT_DIFF
    if (lp->fill < diff_check_fill(curwin, lp->lnum + 1))
    {
	// Add a filler line.
	++lp->fill;
	lp->height = 1;
    }
    else
#endif
    {
	++lp->lnum;
#ifdef FEAT_DIFF
	lp->fill = 0;
#endif
	if (lp->lnum > curbuf->b_ml.ml_line_count)
	    lp->height = MAXCOL;
	else
#ifdef FEAT_FOLDING
	    if (hasFolding(lp->lnum, NULL, &lp->lnum))
	    // Add a closed fold
	    lp->height = 1;
	else
#endif
	    lp->height = PLINES_NOFILL(lp->lnum);
    }
}

#ifdef FEAT_DIFF
/*
 * Switch from including filler lines below lp->lnum to including filler
 * lines above loff.lnum + 1.  This keeps pointing to the same line.
 * When there are no filler lines nothing changes.
 */
    static void
botline_topline(lineoff_T *lp)
{
    if (lp->fill > 0)
    {
	++lp->lnum;
	lp->fill = diff_check_fill(curwin, lp->lnum) - lp->fill + 1;
    }
}

/*
 * Switch from including filler lines above lp->lnum to including filler
 * lines below loff.lnum - 1.  This keeps pointing to the same line.
 * When there are no filler lines nothing changes.
 */
    static void
topline_botline(lineoff_T *lp)
{
    if (lp->fill > 0)
    {
	lp->fill = diff_check_fill(curwin, lp->lnum) - lp->fill + 1;
	--lp->lnum;
    }
}
#endif

/*
 * Recompute topline to put the cursor at the top of the window.
 * Scroll at least "min_scroll" lines.
 * If "always" is TRUE, always set topline (for "zt").
 */
    void
scroll_cursor_top(int min_scroll, int always)
{
    int		scrolled = 0;
    int		extra = 0;
    int		used;
    int		i;
    linenr_T	top;		// just above displayed lines
    linenr_T	bot;		// just below displayed lines
    linenr_T	old_topline = curwin->w_topline;
    int		old_skipcol = curwin->w_skipcol;
#ifdef FEAT_DIFF
    linenr_T	old_topfill = curwin->w_topfill;
#endif
    linenr_T	new_topline;
    int		off = get_scrolloff_value();

    if (mouse_dragging > 0)
	off = mouse_dragging - 1;

    /*
     * Decrease topline until:
     * - it has become 1
     * - (part of) the cursor line is moved off the screen or
     * - moved at least 'scrolljump' lines and
     * - at least 'scrolloff' lines above and below the cursor
     */
    validate_cheight();
    used = curwin->w_cline_height; // includes filler lines above
    if (curwin->w_cursor.lnum < curwin->w_topline)
	scrolled = used;

#ifdef FEAT_FOLDING
    if (hasFolding(curwin->w_cursor.lnum, &top, &bot))
    {
	--top;
	++bot;
    }
    else
#endif
    {
	top = curwin->w_cursor.lnum - 1;
	bot = curwin->w_cursor.lnum + 1;
    }
    new_topline = top + 1;

#ifdef FEAT_DIFF
    // "used" already contains the number of filler lines above, don't add it
    // again.
    // Hide filler lines above cursor line by adding them to "extra".
    extra += diff_check_fill(curwin, curwin->w_cursor.lnum);
#endif

    /*
     * Check if the lines from "top" to "bot" fit in the window.  If they do,
     * set new_topline and advance "top" and "bot" to include more lines.
     */
    while (top > 0)
    {
#ifdef FEAT_FOLDING
	if (hasFolding(top, &top, NULL))
	    // count one logical line for a sequence of folded lines
	    i = 1;
	else
#endif
	    i = PLINES_NOFILL(top);
	if (top < curwin->w_topline)
	    scrolled += i;

	// If scrolling is needed, scroll at least 'sj' lines.
	if ((new_topline >= curwin->w_topline || scrolled > min_scroll)
		&& extra >= off)
	    break;

	used += i;
	if (extra + i <= off && bot < curbuf->b_ml.ml_line_count)
	{
#ifdef FEAT_FOLDING
	    if (hasFolding(bot, NULL, &bot))
		// count one logical line for a sequence of folded lines
		++used;
	    else
#endif
		used += plines(bot);
	}
	if (used > curwin->w_height)
	    break;

	extra += i;
	new_topline = top;
	--top;
	++bot;
    }

    /*
     * If we don't have enough space, put cursor in the middle.
     * This makes sure we get the same position when using "k" and "j"
     * in a small window.
     */
    if (used > curwin->w_height)
	scroll_cursor_halfway(FALSE, FALSE);
    else
    {
	/*
	 * If "always" is FALSE, only adjust topline to a lower value, higher
	 * value may happen with wrapping lines.
	 */
	if (new_topline < curwin->w_topline || always)
	    curwin->w_topline = new_topline;
	if (curwin->w_topline > curwin->w_cursor.lnum)
	    curwin->w_topline = curwin->w_cursor.lnum;
#ifdef FEAT_DIFF
	curwin->w_topfill = diff_check_fill(curwin, curwin->w_topline);
	if (curwin->w_topfill > 0 && extra > off)
	{
	    curwin->w_topfill -= extra - off;
	    if (curwin->w_topfill < 0)
		curwin->w_topfill = 0;
	}
	check_topfill(curwin, FALSE);
#endif
	if (curwin->w_topline != old_topline)
	    reset_skipcol();
	else if (curwin->w_topline == curwin->w_cursor.lnum)
	{
	    validate_virtcol();
	    if (curwin->w_skipcol >= curwin->w_virtcol)
		// TODO: if the line doesn't fit may optimize w_skipcol instead
		// of making it zero
		reset_skipcol();
	}
	if (curwin->w_topline != old_topline
		|| curwin->w_skipcol != old_skipcol
#ifdef FEAT_DIFF
		|| curwin->w_topfill != old_topfill
#endif
		)
	    curwin->w_valid &=
		      ~(VALID_WROW|VALID_CROW|VALID_BOTLINE|VALID_BOTLINE_AP);
	curwin->w_valid |= VALID_TOPLINE;
    }
}

/*
 * Set w_empty_rows and w_filler_rows for window "wp", having used up "used"
 * screen lines for text lines.
 */
    void
set_empty_rows(win_T *wp, int used)
{
#ifdef FEAT_DIFF
    wp->w_filler_rows = 0;
#endif
    if (used == 0)
	wp->w_empty_rows = 0;	// single line that doesn't fit
    else
    {
	wp->w_empty_rows = wp->w_height - used;
#ifdef FEAT_DIFF
	if (wp->w_botline <= wp->w_buffer->b_ml.ml_line_count)
	{
	    wp->w_filler_rows = diff_check_fill(wp, wp->w_botline);
	    if (wp->w_empty_rows > wp->w_filler_rows)
		wp->w_empty_rows -= wp->w_filler_rows;
	    else
	    {
		wp->w_filler_rows = wp->w_empty_rows;
		wp->w_empty_rows = 0;
	    }
	}
#endif
    }
}

/*
 * Recompute topline to put the cursor at the bottom of the window.
 * When scrolling scroll at least "min_scroll" lines.
 * If "set_topbot" is TRUE, set topline and botline first (for "zb").
 * This is messy stuff!!!
 */
    void
scroll_cursor_bot(int min_scroll, int set_topbot)
{
    int		used;
    int		scrolled = 0;
    int		extra = 0;
    int		i;
    linenr_T	line_count;
    linenr_T	old_topline = curwin->w_topline;
    int		old_skipcol = curwin->w_skipcol;
    lineoff_T	loff;
    lineoff_T	boff;
#ifdef FEAT_DIFF
    int		old_topfill = curwin->w_topfill;
    int		fill_below_window;
#endif
    linenr_T	old_botline = curwin->w_botline;
    linenr_T	old_valid = curwin->w_valid;
    int		old_empty_rows = curwin->w_empty_rows;
    linenr_T	cln;		    // Cursor Line Number
    long	so = get_scrolloff_value();
    int		do_sms = curwin->w_p_wrap && curwin->w_p_sms;

    cln = curwin->w_cursor.lnum;
    if (set_topbot)
    {
	int set_skipcol = FALSE;

	used = 0;
	curwin->w_botline = cln + 1;
#ifdef FEAT_DIFF
	loff.fill = 0;
#endif
	for (curwin->w_topline = curwin->w_botline;
		curwin->w_topline > 1;
		curwin->w_topline = loff.lnum)
	{
	    loff.lnum = curwin->w_topline;
	    topline_back_winheight(&loff, FALSE);
	    if (loff.height == MAXCOL)
		break;
	    if (used + loff.height > curwin->w_height)
	    {
		if (do_sms)
		{
		    // 'smoothscroll' and 'wrap' are set.  The above line is
		    // too long to show in its entirety, so we show just a part
		    // of it.
		    if (used < curwin->w_height)
		    {
			int plines_offset = used + loff.height
							    - curwin->w_height;
			used = curwin->w_height;
#ifdef FEAT_DIFF
			curwin->w_topfill = loff.fill;
#endif
			curwin->w_topline = loff.lnum;
			curwin->w_skipcol = skipcol_from_plines(
							curwin, plines_offset);
			set_skipcol = TRUE;
		    }
		}
		break;
	    }
	    used += loff.height;
#ifdef FEAT_DIFF
	    curwin->w_topfill = loff.fill;
#endif
	}
	set_empty_rows(curwin, used);
	curwin->w_valid |= VALID_BOTLINE|VALID_BOTLINE_AP;
	if (curwin->w_topline != old_topline
#ifdef FEAT_DIFF
		|| curwin->w_topfill != old_topfill
#endif
		|| set_skipcol
		|| curwin->w_skipcol != 0)
	{
	    curwin->w_valid &= ~(VALID_WROW|VALID_CROW);
	    if (set_skipcol)
		redraw_later(UPD_NOT_VALID);
	    else
		reset_skipcol();
	}
    }
    else
	validate_botline();

    // The lines of the cursor line itself are always used.
#ifdef FEAT_DIFF
    used = plines_nofill(cln);
#else
    validate_cheight();
    used = curwin->w_cline_height;
#endif

    // If the cursor is on or below botline, we will at least scroll by the
    // height of the cursor line, which is "used".  Correct for empty lines,
    // which are really part of botline.
    if (cln >= curwin->w_botline)
    {
	scrolled = used;
	if (cln == curwin->w_botline)
	    scrolled -= curwin->w_empty_rows;
	if (do_sms)
	{
	    // 'smoothscroll' and 'wrap' are set.
	    // Calculate how many screen lines the current top line of window
	    // occupies. If it is occupying more than the entire window, we
	    // need to scroll the additional clipped lines to scroll past the
	    // top line before we can move on to the other lines.
	    int top_plines =
#ifdef FEAT_DIFF
			    plines_win_nofill
#else
			    plines_win
#endif
					(curwin, curwin->w_topline, FALSE);
	    int skip_lines = 0;
	    int width1 = curwin->w_width - curwin_col_off();
	    if (width1 > 0)
	    {
		int width2 = width1 + curwin_col_off2();
		// similar formula is used in curs_columns()
		if (curwin->w_skipcol > width1)
		    skip_lines += (curwin->w_skipcol - width1) / width2 + 1;
		else if (curwin->w_skipcol > 0)
		    skip_lines = 1;

		top_plines -= skip_lines;
		if (top_plines > curwin->w_height)
		{
		    scrolled += (top_plines - curwin->w_height);
		}
	    }
	}
    }

    /*
     * Stop counting lines to scroll when
     * - hitting start of the file
     * - scrolled nothing or at least 'sj' lines
     * - at least 'scrolloff' lines below the cursor
     * - lines between botline and cursor have been counted
     */
#ifdef FEAT_FOLDING
    if (!hasFolding(curwin->w_cursor.lnum, &loff.lnum, &boff.lnum))
#endif
    {
	loff.lnum = cln;
	boff.lnum = cln;
    }
#ifdef FEAT_DIFF
    loff.fill = 0;
    boff.fill = 0;
    fill_below_window = diff_check_fill(curwin, curwin->w_botline)
						      - curwin->w_filler_rows;
#endif

    while (loff.lnum > 1)
    {
	// Stop when scrolled nothing or at least "min_scroll", found "extra"
	// context for 'scrolloff' and counted all lines below the window.
	if ((((scrolled <= 0 || scrolled >= min_scroll)
		    && extra >= (mouse_dragging > 0 ? mouse_dragging - 1 : so))
		    || boff.lnum + 1 > curbuf->b_ml.ml_line_count)
		&& loff.lnum <= curwin->w_botline
#ifdef FEAT_DIFF
		&& (loff.lnum < curwin->w_botline
		    || loff.fill >= fill_below_window)
#endif
		)
	    break;

	// Add one line above
	topline_back(&loff);
	if (loff.height == MAXCOL)
	    used = MAXCOL;
	else
	    used += loff.height;
	if (used > curwin->w_height)
	    break;
	if (loff.lnum >= curwin->w_botline
#ifdef FEAT_DIFF
		&& (loff.lnum > curwin->w_botline
		    || loff.fill <= fill_below_window)
#endif
		)
	{
	    // Count screen lines that are below the window.
	    scrolled += loff.height;
	    if (loff.lnum == curwin->w_botline
#ifdef FEAT_DIFF
			    && loff.fill == 0
#endif
		    )
		scrolled -= curwin->w_empty_rows;
	}

	if (boff.lnum < curbuf->b_ml.ml_line_count)
	{
	    // Add one line below
	    botline_forw(&boff);
	    used += boff.height;
	    if (used > curwin->w_height)
		break;
	    if (extra < ( mouse_dragging > 0 ? mouse_dragging - 1 : so)
		    || scrolled < min_scroll)
	    {
		extra += boff.height;
		if (boff.lnum >= curwin->w_botline
#ifdef FEAT_DIFF
			|| (boff.lnum + 1 == curwin->w_botline
			    && boff.fill > curwin->w_filler_rows)
#endif
		   )
		{
		    // Count screen lines that are below the window.
		    scrolled += boff.height;
		    if (boff.lnum == curwin->w_botline
#ifdef FEAT_DIFF
			    && boff.fill == 0
#endif
			    )
			scrolled -= curwin->w_empty_rows;
		}
	    }
	}
    }

    // curwin->w_empty_rows is larger, no need to scroll
    if (scrolled <= 0)
	line_count = 0;
    // more than a screenfull, don't scroll but redraw
    else if (used > curwin->w_height)
	line_count = used;
    // scroll minimal number of lines
    else
    {
	line_count = 0;
#ifdef FEAT_DIFF
	boff.fill = curwin->w_topfill;
#endif
	boff.lnum = curwin->w_topline - 1;
	for (i = 0; i < scrolled && boff.lnum < curwin->w_botline; )
	{
	    botline_forw(&boff);
	    i += boff.height;
	    ++line_count;
	}
	if (i < scrolled)	// below curwin->w_botline, don't scroll
	    line_count = 9999;
    }

    /*
     * Scroll up if the cursor is off the bottom of the screen a bit.
     * Otherwise put it at 1/2 of the screen.
     */
    if (line_count >= curwin->w_height && line_count > min_scroll)
	scroll_cursor_halfway(FALSE, TRUE);
    else if (line_count > 0)
    {
	if (do_sms)
	    scrollup(scrolled, TRUE);  // TODO
	else
	    scrollup(line_count, TRUE);
    }

    /*
     * If topline didn't change we need to restore w_botline and w_empty_rows
     * (we changed them).
     * If topline did change, update_screen() will set botline.
     */
    if (curwin->w_topline == old_topline
	    && curwin->w_skipcol == old_skipcol
	    && set_topbot)
    {
	curwin->w_botline = old_botline;
	curwin->w_empty_rows = old_empty_rows;
	curwin->w_valid = old_valid;
    }
    curwin->w_valid |= VALID_TOPLINE;
}

/*
 * Recompute topline to put the cursor halfway the window
 * If "atend" is TRUE, also put it halfway at the end of the file.
 */
    void
scroll_cursor_halfway(int atend, int prefer_above)
{
    int		above = 0;
    linenr_T	topline;
    colnr_T	skipcol = 0;
#ifdef FEAT_DIFF
    int		topfill = 0;
#endif
    int		below = 0;
    int		used;
    lineoff_T	loff;
    lineoff_T	boff;
#ifdef FEAT_DIFF
    linenr_T	old_topline = curwin->w_topline;
#endif

#ifdef FEAT_PROP_POPUP
    // if the width changed this needs to be updated first
    may_update_popup_position();
#endif
    loff.lnum = boff.lnum = curwin->w_cursor.lnum;
#ifdef FEAT_FOLDING
    (void)hasFolding(loff.lnum, &loff.lnum, &boff.lnum);
#endif
#ifdef FEAT_DIFF
    used = plines_nofill(loff.lnum);
    loff.fill = 0;
    boff.fill = 0;
#else
    used = plines(loff.lnum);
#endif
    topline = loff.lnum;

    int want_height;
    int do_sms = curwin->w_p_wrap && curwin->w_p_sms;
    if (do_sms)
    {
	// 'smoothscroll' and 'wrap' are set
	if (atend)
	{
	    want_height = (curwin->w_height - used) / 2;
	    used = 0;
	}
	else
	    want_height = curwin->w_height;
    }

    while (topline > 1)
    {
	// If using smoothscroll, we can precisely scroll to the
	// exact point where the cursor is halfway down the screen.
	if (do_sms)
	{
	    topline_back_winheight(&loff, FALSE);
	    if (loff.height == MAXCOL)
		break;
	    used += loff.height;
	    if (!atend && boff.lnum < curbuf->b_ml.ml_line_count)
	    {
		botline_forw(&boff);
		used += boff.height;
	    }
	    if (used > want_height)
	    {
		if (used - loff.height < want_height)
		{
		    topline = loff.lnum;
#ifdef FEAT_DIFF
		    topfill = loff.fill;
#endif
		    skipcol = skipcol_from_plines(curwin, used - want_height);
		}
		break;
	    }
	    topline = loff.lnum;
#ifdef FEAT_DIFF
	    topfill = loff.fill;
#endif
	    continue;
	}

	// If not using smoothscroll, we have to iteratively find how many
	// lines to scroll down to roughly fit the cursor.
	// This may not be right in the middle if the lines'
	// physical height > 1 (e.g. 'wrap' is on).

	// Depending on "prefer_above" we add a line above or below first.
	// Loop twice to avoid duplicating code.
	int done = FALSE;
	for (int round = 1; round <= 2; ++round)
	{
	    if (prefer_above ? (round == 2 && below < above)
			     : (round == 1 && below <= above))
	    {
		// add a line below the cursor
		if (boff.lnum < curbuf->b_ml.ml_line_count)
		{
		    botline_forw(&boff);
		    used += boff.height;
		    if (used > curwin->w_height)
		    {
			done = TRUE;
			break;
		    }
		    below += boff.height;
		}
		else
		{
		    ++below;	    // count a "~" line
		    if (atend)
			++used;
		}
	    }

	    if (prefer_above ? (round == 1 && below >= above)
			     : (round == 1 && below > above))
	    {
		// add a line above the cursor
		topline_back(&loff);
		if (loff.height == MAXCOL)
		    used = MAXCOL;
		else
		    used += loff.height;
		if (used > curwin->w_height)
		{
		    done = TRUE;
		    break;
		}
		above += loff.height;
		topline = loff.lnum;
#ifdef FEAT_DIFF
		topfill = loff.fill;
#endif
	    }
	}
	if (done)
	    break;
    }

#ifdef FEAT_FOLDING
    if (!hasFolding(topline, &curwin->w_topline, NULL))
#endif
    {
	if (curwin->w_topline != topline
		|| skipcol != 0
		|| curwin->w_skipcol != 0)
	{
	    curwin->w_topline = topline;
	    if (skipcol != 0)
	    {
		curwin->w_skipcol = skipcol;
		redraw_later(UPD_NOT_VALID);
	    }
	    else if (do_sms)
		reset_skipcol();
	}
    }
#ifdef FEAT_DIFF
    curwin->w_topfill = topfill;
    if (old_topline > curwin->w_topline + curwin->w_height)
	curwin->w_botfill = FALSE;
    check_topfill(curwin, FALSE);
#endif
    curwin->w_valid &= ~(VALID_WROW|VALID_CROW|VALID_BOTLINE|VALID_BOTLINE_AP);
    curwin->w_valid |= VALID_TOPLINE;
}

/*
 * Correct the cursor position so that it is in a part of the screen at least
 * 'scrolloff' lines from the top and bottom, if possible.
 * If not possible, put it at the same position as scroll_cursor_halfway().
 * When called topline must be valid!
 */
    void
cursor_correct(void)
{
    int		above = 0;	    // screen lines above topline
    linenr_T	topline;
    int		below = 0;	    // screen lines below botline
    linenr_T	botline;
    int		above_wanted, below_wanted;
    linenr_T	cln;		    // Cursor Line Number
    int		max_off;
    long	so = get_scrolloff_value();

    /*
     * How many lines we would like to have above/below the cursor depends on
     * whether the first/last line of the file is on screen.
     */
    above_wanted = so;
    below_wanted = so;
    if (mouse_dragging > 0)
    {
	above_wanted = mouse_dragging - 1;
	below_wanted = mouse_dragging - 1;
    }
    if (curwin->w_topline == 1)
    {
	above_wanted = 0;
	max_off = curwin->w_height / 2;
	if (below_wanted > max_off)
	    below_wanted = max_off;
    }
    validate_botline();
    if (curwin->w_botline == curbuf->b_ml.ml_line_count + 1
	    && mouse_dragging == 0)
    {
	below_wanted = 0;
	max_off = (curwin->w_height - 1) / 2;
	if (above_wanted > max_off)
	    above_wanted = max_off;
    }

    /*
     * If there are sufficient file-lines above and below the cursor, we can
     * return now.
     */
    cln = curwin->w_cursor.lnum;
    if (cln >= curwin->w_topline + above_wanted
	    && cln < curwin->w_botline - below_wanted
#ifdef FEAT_FOLDING
	    && !hasAnyFolding(curwin)
#endif
	    )
	return;

    if (curwin->w_p_sms && !curwin->w_p_wrap)
    {
	// 'smoothscroll' is active
	if (curwin->w_cline_height == curwin->w_height)
	{
	    // The cursor line just fits in the window, don't scroll.
	    reset_skipcol();
	    return;
	}
	// TODO: If the cursor line doesn't fit in the window then only adjust
	// w_skipcol.
    }

    /*
     * Narrow down the area where the cursor can be put by taking lines from
     * the top and the bottom until:
     * - the desired context lines are found
     * - the lines from the top is past the lines from the bottom
     */
    topline = curwin->w_topline;
    botline = curwin->w_botline - 1;
#ifdef FEAT_DIFF
    // count filler lines as context
    above = curwin->w_topfill;
    below = curwin->w_filler_rows;
#endif
    while ((above < above_wanted || below < below_wanted) && topline < botline)
    {
	if (below < below_wanted && (below <= above || above >= above_wanted))
	{
#ifdef FEAT_FOLDING
	    if (hasFolding(botline, &botline, NULL))
		++below;
	    else
#endif
		below += plines(botline);
	    --botline;
	}
	if (above < above_wanted && (above < below || below >= below_wanted))
	{
#ifdef FEAT_FOLDING
	    if (hasFolding(topline, NULL, &topline))
		++above;
	    else
#endif
		above += PLINES_NOFILL(topline);
#ifdef FEAT_DIFF
	    // Count filler lines below this line as context.
	    if (topline < botline)
		above += diff_check_fill(curwin, topline + 1);
#endif
	    ++topline;
	}
    }
    if (topline == botline || botline == 0)
	curwin->w_cursor.lnum = topline;
    else if (topline > botline)
	curwin->w_cursor.lnum = botline;
    else
    {
	if (cln < topline && curwin->w_topline > 1)
	{
	    curwin->w_cursor.lnum = topline;
	    curwin->w_valid &=
			    ~(VALID_WROW|VALID_WCOL|VALID_CHEIGHT|VALID_CROW);
	}
	if (cln > botline && curwin->w_botline <= curbuf->b_ml.ml_line_count)
	{
	    curwin->w_cursor.lnum = botline;
	    curwin->w_valid &=
			    ~(VALID_WROW|VALID_WCOL|VALID_CHEIGHT|VALID_CROW);
	}
    }
    curwin->w_valid |= VALID_TOPLINE;
}

static void get_scroll_overlap(lineoff_T *lp, int dir);

/*
 * Move screen "count" pages up ("dir" is BACKWARD) or down ("dir" is FORWARD)
 * and update the screen.
 *
 * Return FAIL for failure, OK otherwise.
 */
    int
onepage(int dir, long count)
{
    long	n;
    int		retval = OK;
    lineoff_T	loff;
    linenr_T	old_topline = curwin->w_topline;
    long	so = get_scrolloff_value();

    if (curbuf->b_ml.ml_line_count == 1)    // nothing to do
    {
	beep_flush();
	return FAIL;
    }

    for ( ; count > 0; --count)
    {
	validate_botline();
	/*
	 * It's an error to move a page up when the first line is already on
	 * the screen.	It's an error to move a page down when the last line
	 * is on the screen and the topline is 'scrolloff' lines from the
	 * last line.
	 */
	if (dir == FORWARD
		? ((curwin->w_topline >= curbuf->b_ml.ml_line_count - so)
		    && curwin->w_botline > curbuf->b_ml.ml_line_count)
		: (curwin->w_topline == 1
#ifdef FEAT_DIFF
		    && curwin->w_topfill ==
				    diff_check_fill(curwin, curwin->w_topline)
#endif
		    ))
	{
	    beep_flush();
	    retval = FAIL;
	    break;
	}

#ifdef FEAT_DIFF
	loff.fill = 0;
#endif
	if (dir == FORWARD)
	{
	    if (ONE_WINDOW && p_window > 0 && p_window < Rows - 1)
	    {
		// Vi compatible scrolling
		if (p_window <= 2)
		    ++curwin->w_topline;
		else
		    curwin->w_topline += p_window - 2;
		if (curwin->w_topline > curbuf->b_ml.ml_line_count)
		    curwin->w_topline = curbuf->b_ml.ml_line_count;
		curwin->w_cursor.lnum = curwin->w_topline;
	    }
	    else if (curwin->w_botline > curbuf->b_ml.ml_line_count)
	    {
		// at end of file
		curwin->w_topline = curbuf->b_ml.ml_line_count;
#ifdef FEAT_DIFF
		curwin->w_topfill = 0;
#endif
		curwin->w_valid &= ~(VALID_WROW|VALID_CROW);
	    }
	    else
	    {
		// For the overlap, start with the line just below the window
		// and go upwards.
		loff.lnum = curwin->w_botline;
#ifdef FEAT_DIFF
		loff.fill = diff_check_fill(curwin, loff.lnum)
						      - curwin->w_filler_rows;
#endif
		get_scroll_overlap(&loff, -1);
		curwin->w_topline = loff.lnum;
#ifdef FEAT_DIFF
		curwin->w_topfill = loff.fill;
		check_topfill(curwin, FALSE);
#endif
		curwin->w_cursor.lnum = curwin->w_topline;
		curwin->w_valid &= ~(VALID_WCOL|VALID_CHEIGHT|VALID_WROW|
				   VALID_CROW|VALID_BOTLINE|VALID_BOTLINE_AP);
	    }
	}
	else	// dir == BACKWARDS
	{
#ifdef FEAT_DIFF
	    if (curwin->w_topline == 1)
	    {
		// Include max number of filler lines
		max_topfill();
		continue;
	    }
#endif
	    if (ONE_WINDOW && p_window > 0 && p_window < Rows - 1)
	    {
		// Vi compatible scrolling (sort of)
		if (p_window <= 2)
		    --curwin->w_topline;
		else
		    curwin->w_topline -= p_window - 2;
		if (curwin->w_topline < 1)
		    curwin->w_topline = 1;
		curwin->w_cursor.lnum = curwin->w_topline + p_window - 1;
		if (curwin->w_cursor.lnum > curbuf->b_ml.ml_line_count)
		    curwin->w_cursor.lnum = curbuf->b_ml.ml_line_count;
		continue;
	    }

	    // Find the line at the top of the window that is going to be the
	    // line at the bottom of the window.  Make sure this results in
	    // the same line as before doing CTRL-F.
	    loff.lnum = curwin->w_topline - 1;
#ifdef FEAT_DIFF
	    loff.fill = diff_check_fill(curwin, loff.lnum + 1)
							  - curwin->w_topfill;
#endif
	    get_scroll_overlap(&loff, 1);

	    if (loff.lnum >= curbuf->b_ml.ml_line_count)
	    {
		loff.lnum = curbuf->b_ml.ml_line_count;
#ifdef FEAT_DIFF
		loff.fill = 0;
	    }
	    else
	    {
		botline_topline(&loff);
#endif
	    }
	    curwin->w_cursor.lnum = loff.lnum;

	    // Find the line just above the new topline to get the right line
	    // at the bottom of the window.
	    n = 0;
	    while (n <= curwin->w_height && loff.lnum >= 1)
	    {
		topline_back(&loff);
		if (loff.height == MAXCOL)
		    n = MAXCOL;
		else
		    n += loff.height;
	    }
	    if (loff.lnum < 1)			// at begin of file
	    {
		curwin->w_topline = 1;
#ifdef FEAT_DIFF
		max_topfill();
#endif
		curwin->w_valid &= ~(VALID_WROW|VALID_CROW|VALID_BOTLINE);
	    }
	    else
	    {
		// Go two lines forward again.
#ifdef FEAT_DIFF
		topline_botline(&loff);
#endif
		botline_forw(&loff);
		botline_forw(&loff);
#ifdef FEAT_DIFF
		botline_topline(&loff);
#endif
#ifdef FEAT_FOLDING
		// We're at the wrong end of a fold now.
		(void)hasFolding(loff.lnum, &loff.lnum, NULL);
#endif

		// Always scroll at least one line.  Avoid getting stuck on
		// very long lines.
		if (loff.lnum >= curwin->w_topline
#ifdef FEAT_DIFF
			&& (loff.lnum > curwin->w_topline
			    || loff.fill >= curwin->w_topfill)
#endif
			)
		{
#ifdef FEAT_DIFF
		    // First try using the maximum number of filler lines.  If
		    // that's not enough, backup one line.
		    loff.fill = curwin->w_topfill;
		    if (curwin->w_topfill < diff_check_fill(curwin,
							   curwin->w_topline))
			max_topfill();
		    if (curwin->w_topfill == loff.fill)
#endif
		    {
			--curwin->w_topline;
#ifdef FEAT_DIFF
			curwin->w_topfill = 0;
#endif
			curwin->w_valid &= ~(VALID_WROW|VALID_CROW);
		    }
		    comp_botline(curwin);
		    curwin->w_cursor.lnum = curwin->w_botline - 1;
		    curwin->w_valid &=
			    ~(VALID_WCOL|VALID_CHEIGHT|VALID_WROW|VALID_CROW);
		}
		else
		{
		    curwin->w_topline = loff.lnum;
#ifdef FEAT_DIFF
		    curwin->w_topfill = loff.fill;
		    check_topfill(curwin, FALSE);
#endif
		    curwin->w_valid &= ~(VALID_WROW|VALID_CROW|VALID_BOTLINE);
		}
	    }
	}
    }
#ifdef FEAT_FOLDING
    foldAdjustCursor();
#endif
    cursor_correct();
    check_cursor_col();
    if (retval == OK)
	beginline(BL_SOL | BL_FIX);
    curwin->w_valid &= ~(VALID_WCOL|VALID_WROW|VALID_VIRTCOL);

    if (retval == OK && dir == FORWARD)
    {
	// Avoid the screen jumping up and down when 'scrolloff' is non-zero.
	// But make sure we scroll at least one line (happens with mix of long
	// wrapping lines and non-wrapping line).
	if (check_top_offset())
	{
	    scroll_cursor_top(1, FALSE);
	    if (curwin->w_topline <= old_topline
				  && old_topline < curbuf->b_ml.ml_line_count)
	    {
		curwin->w_topline = old_topline + 1;
#ifdef FEAT_FOLDING
		(void)hasFolding(curwin->w_topline, &curwin->w_topline, NULL);
#endif
	    }
	}
#ifdef FEAT_FOLDING
	else if (curwin->w_botline > curbuf->b_ml.ml_line_count)
	    (void)hasFolding(curwin->w_topline, &curwin->w_topline, NULL);
#endif
    }

    redraw_later(UPD_VALID);
    return retval;
}

/*
 * Decide how much overlap to use for page-up or page-down scrolling.
 * This is symmetric, so that doing both keeps the same lines displayed.
 * Three lines are examined:
 *
 *  before CTRL-F	    after CTRL-F / before CTRL-B
 *     etc.			l1
 *  l1 last but one line	------------
 *  l2 last text line		l2 top text line
 *  -------------		l3 second text line
 *  l3				   etc.
 */
    static void
get_scroll_overlap(lineoff_T *lp, int dir)
{
    int		h1, h2, h3, h4;
    int		min_height = curwin->w_height - 2;
    lineoff_T	loff0, loff1, loff2;

#ifdef FEAT_DIFF
    if (lp->fill > 0)
	lp->height = 1;
    else
	lp->height = plines_nofill(lp->lnum);
#else
    lp->height = plines(lp->lnum);
#endif
    h1 = lp->height;
    if (h1 > min_height)
	return;		// no overlap

    loff0 = *lp;
    if (dir > 0)
	botline_forw(lp);
    else
	topline_back(lp);
    h2 = lp->height;
    if (h2 == MAXCOL || h2 + h1 > min_height)
    {
	*lp = loff0;	// no overlap
	return;
    }

    loff1 = *lp;
    if (dir > 0)
	botline_forw(lp);
    else
	topline_back(lp);
    h3 = lp->height;
    if (h3 == MAXCOL || h3 + h2 > min_height)
    {
	*lp = loff0;	// no overlap
	return;
    }

    loff2 = *lp;
    if (dir > 0)
	botline_forw(lp);
    else
	topline_back(lp);
    h4 = lp->height;
    if (h4 == MAXCOL || h4 + h3 + h2 > min_height || h3 + h2 + h1 > min_height)
	*lp = loff1;	// 1 line overlap
    else
	*lp = loff2;	// 2 lines overlap
}

/*
 * Scroll 'scroll' lines up or down.
 */
    void
halfpage(int flag, linenr_T Prenum)
{
    long	scrolled = 0;
    int		i;
    int		n;
    int		room;

    if (Prenum)
	curwin->w_p_scr = (Prenum > curwin->w_height) ?
						curwin->w_height : Prenum;
    n = (curwin->w_p_scr <= curwin->w_height) ?
				    curwin->w_p_scr : curwin->w_height;

    update_topline();
    validate_botline();
    room = curwin->w_empty_rows;
#ifdef FEAT_DIFF
    room += curwin->w_filler_rows;
#endif
    if (flag)
    {
	/*
	 * scroll the text up
	 */
	while (n > 0 && curwin->w_botline <= curbuf->b_ml.ml_line_count)
	{
#ifdef FEAT_DIFF
	    if (curwin->w_topfill > 0)
	    {
		i = 1;
		--n;
		--curwin->w_topfill;
	    }
	    else
#endif
	    {
		i = PLINES_NOFILL(curwin->w_topline);
		n -= i;
		if (n < 0 && scrolled > 0)
		    break;
#ifdef FEAT_FOLDING
		(void)hasFolding(curwin->w_topline, NULL, &curwin->w_topline);
#endif
		++curwin->w_topline;
#ifdef FEAT_DIFF
		curwin->w_topfill = diff_check_fill(curwin, curwin->w_topline);
#endif

		if (curwin->w_cursor.lnum < curbuf->b_ml.ml_line_count)
		{
		    ++curwin->w_cursor.lnum;
		    curwin->w_valid &=
				    ~(VALID_VIRTCOL|VALID_CHEIGHT|VALID_WCOL);
		}
	    }
	    curwin->w_valid &= ~(VALID_CROW|VALID_WROW);
	    scrolled += i;

	    /*
	     * Correct w_botline for changed w_topline.
	     * Won't work when there are filler lines.
	     */
#ifdef FEAT_DIFF
	    if (curwin->w_p_diff)
		curwin->w_valid &= ~(VALID_BOTLINE|VALID_BOTLINE_AP);
	    else
#endif
	    {
		room += i;
		do
		{
		    i = plines(curwin->w_botline);
		    if (i > room)
			break;
#ifdef FEAT_FOLDING
		    (void)hasFolding(curwin->w_botline, NULL,
							  &curwin->w_botline);
#endif
		    ++curwin->w_botline;
		    room -= i;
		} while (curwin->w_botline <= curbuf->b_ml.ml_line_count);
	    }
	}

	/*
	 * When hit bottom of the file: move cursor down.
	 */
	if (n > 0)
	{
# ifdef FEAT_FOLDING
	    if (hasAnyFolding(curwin))
	    {
		while (--n >= 0
			&& curwin->w_cursor.lnum < curbuf->b_ml.ml_line_count)
		{
		    (void)hasFolding(curwin->w_cursor.lnum, NULL,
						      &curwin->w_cursor.lnum);
		    ++curwin->w_cursor.lnum;
		}
	    }
	    else
# endif
		curwin->w_cursor.lnum += n;
	    check_cursor_lnum();
	}
    }
    else
    {
	/*
	 * scroll the text down
	 */
	while (n > 0 && curwin->w_topline > 1)
	{
#ifdef FEAT_DIFF
	    if (curwin->w_topfill < diff_check_fill(curwin, curwin->w_topline))
	    {
		i = 1;
		--n;
		++curwin->w_topfill;
	    }
	    else
#endif
	    {
		i = PLINES_NOFILL(curwin->w_topline - 1);
		n -= i;
		if (n < 0 && scrolled > 0)
		    break;
		--curwin->w_topline;
#ifdef FEAT_FOLDING
		(void)hasFolding(curwin->w_topline, &curwin->w_topline, NULL);
#endif
#ifdef FEAT_DIFF
		curwin->w_topfill = 0;
#endif
	    }
	    curwin->w_valid &= ~(VALID_CROW|VALID_WROW|
					      VALID_BOTLINE|VALID_BOTLINE_AP);
	    scrolled += i;
	    if (curwin->w_cursor.lnum > 1)
	    {
		--curwin->w_cursor.lnum;
		curwin->w_valid &= ~(VALID_VIRTCOL|VALID_CHEIGHT|VALID_WCOL);
	    }
	}

	/*
	 * When hit top of the file: move cursor up.
	 */
	if (n > 0)
	{
	    if (curwin->w_cursor.lnum <= (linenr_T)n)
		curwin->w_cursor.lnum = 1;
	    else
# ifdef FEAT_FOLDING
	    if (hasAnyFolding(curwin))
	    {
		while (--n >= 0 && curwin->w_cursor.lnum > 1)
		{
		    --curwin->w_cursor.lnum;
		    (void)hasFolding(curwin->w_cursor.lnum,
						&curwin->w_cursor.lnum, NULL);
		}
	    }
	    else
# endif
		curwin->w_cursor.lnum -= n;
	}
    }
# ifdef FEAT_FOLDING
    // Move cursor to first line of closed fold.
    foldAdjustCursor();
# endif
#ifdef FEAT_DIFF
    check_topfill(curwin, !flag);
#endif
    cursor_correct();
    beginline(BL_SOL | BL_FIX);
    redraw_later(UPD_VALID);
}

    void
do_check_cursorbind(void)
{
    static win_T	*prev_curwin = NULL;
    static pos_T	prev_cursor = {0, 0, 0};

    if (curwin == prev_curwin && EQUAL_POS(curwin->w_cursor, prev_cursor))
	return;
    prev_curwin = curwin;
    prev_cursor = curwin->w_cursor;

    linenr_T	line = curwin->w_cursor.lnum;
    colnr_T	col = curwin->w_cursor.col;
    colnr_T	coladd = curwin->w_cursor.coladd;
    colnr_T	curswant = curwin->w_curswant;
    int		set_curswant = curwin->w_set_curswant;
    win_T	*old_curwin = curwin;
    buf_T	*old_curbuf = curbuf;
    int		old_VIsual_select = VIsual_select;
    int		old_VIsual_active = VIsual_active;

    /*
     * loop through the cursorbound windows
     */
    VIsual_select = VIsual_active = 0;
    FOR_ALL_WINDOWS(curwin)
    {
	curbuf = curwin->w_buffer;
	// skip original window and windows with 'nocursorbind'
	if (curwin != old_curwin && curwin->w_p_crb)
	{
# ifdef FEAT_DIFF
	    if (curwin->w_p_diff)
		curwin->w_cursor.lnum =
				 diff_get_corresponding_line(old_curbuf, line);
	    else
# endif
		curwin->w_cursor.lnum = line;
	    curwin->w_cursor.col = col;
	    curwin->w_cursor.coladd = coladd;
	    curwin->w_curswant = curswant;
	    curwin->w_set_curswant = set_curswant;

	    // Make sure the cursor is in a valid position.  Temporarily set
	    // "restart_edit" to allow the cursor to be beyond the EOL.
	    int restart_edit_save = restart_edit;
	    restart_edit = 'a';
	    check_cursor();

	    // Avoid a scroll here for the cursor position, 'scrollbind' is
	    // more important.
	    if (!curwin->w_p_scb)
		validate_cursor();

	    restart_edit = restart_edit_save;
	    // Correct cursor for multi-byte character.
	    if (has_mbyte)
		mb_adjust_cursor();
	    redraw_later(UPD_VALID);

	    // Only scroll when 'scrollbind' hasn't done this.
	    if (!curwin->w_p_scb)
		update_topline();
	    curwin->w_redr_status = TRUE;
	}
    }

    /*
     * reset current-window
     */
    VIsual_select = old_VIsual_select;
    VIsual_active = old_VIsual_active;
    curwin = old_curwin;
    curbuf = old_curbuf;
}
