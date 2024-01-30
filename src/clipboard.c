/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * clipboard.c: Functions to handle the clipboard
 */

#include "vim.h"

#ifdef FEAT_CYGWIN_WIN32_CLIPBOARD
# define WIN32_LEAN_AND_MEAN
# include <windows.h>
# include "winclip.pro"
#endif

// Functions for copying and pasting text between applications.
// This is always included in a GUI version, but may also be included when the
// clipboard and mouse is available to a terminal version such as xterm.
// Note: there are some more functions in ops.c that handle selection stuff.
//
// Also note that the majority of functions here deal with the X 'primary'
// (visible - for Visual mode use) selection, and only that. There are no
// versions of these for the 'clipboard' selection, as Visual mode has no use
// for them.

#if defined(FEAT_CLIPBOARD) || defined(PROTO)

/*
 * Selection stuff using Visual mode, for cutting and pasting text to other
 * windows.
 */

/*
 * Call this to initialise the clipboard.  Pass it FALSE if the clipboard code
 * is included, but the clipboard can not be used, or TRUE if the clipboard can
 * be used.  Eg unix may call this with FALSE, then call it again with TRUE if
 * the GUI starts.
 */
    void
clip_init(int can_use)
{
    Clipboard_T *cb;

    cb = &clip_star;
    for (;;)
    {
	cb->available  = can_use;
	cb->owned      = FALSE;
	cb->start.lnum = 0;
	cb->start.col  = 0;
	cb->end.lnum   = 0;
	cb->end.col    = 0;
	cb->state      = SELECT_CLEARED;

	if (cb == &clip_plus)
	    break;
	cb = &clip_plus;
    }
}

/*
 * Check whether the VIsual area has changed, and if so try to become the owner
 * of the selection, and free any old converted selection we may still have
 * lying around.  If the VIsual mode has ended, make a copy of what was
 * selected so we can still give it to others.	Will probably have to make sure
 * this is called whenever VIsual mode is ended.
 */
    void
clip_update_selection(Clipboard_T *clip)
{
    pos_T	    start, end;

    // If visual mode is only due to a redo command ("."), then ignore it
    if (!redo_VIsual_busy && VIsual_active && (State & MODE_NORMAL))
    {
	if (LT_POS(VIsual, curwin->w_cursor))
	{
	    start = VIsual;
	    end = curwin->w_cursor;
	    if (has_mbyte)
		end.col += (*mb_ptr2len)(ml_get_cursor()) - 1;
	}
	else
	{
	    start = curwin->w_cursor;
	    end = VIsual;
	}
	if (!EQUAL_POS(clip->start, start)
		|| !EQUAL_POS(clip->end, end)
		|| clip->vmode != VIsual_mode)
	{
	    clip_clear_selection(clip);
	    clip->start = start;
	    clip->end = end;
	    clip->vmode = VIsual_mode;
	    clip_free_selection(clip);
	    clip_own_selection(clip);
	    clip_gen_set_selection(clip);
	}
    }
}

    static int
clip_gen_own_selection(Clipboard_T *cbd)
{
#ifdef FEAT_XCLIPBOARD
# ifdef FEAT_GUI
    if (gui.in_use)
	return clip_mch_own_selection(cbd);
    else
# endif
	return clip_xterm_own_selection(cbd);
#else
    return clip_mch_own_selection(cbd);
#endif
}

    void
clip_own_selection(Clipboard_T *cbd)
{
    /*
     * Also want to check somehow that we are reading from the keyboard rather
     * than a mapping etc.
     */
#ifdef FEAT_X11
    // Always own the selection, we might have lost it without being
    // notified, e.g. during a ":sh" command.
    if (cbd->available)
    {
	int was_owned = cbd->owned;

	cbd->owned = (clip_gen_own_selection(cbd) == OK);
	if (!was_owned && (cbd == &clip_star || cbd == &clip_plus))
	{
	    // May have to show a different kind of highlighting for the
	    // selected area.  There is no specific redraw command for this,
	    // just redraw all windows on the current buffer.
	    if (cbd->owned
		    && (get_real_state() == MODE_VISUAL
					    || get_real_state() == MODE_SELECT)
		    && (cbd == &clip_star ? clip_isautosel_star()
						      : clip_isautosel_plus())
		    && HL_ATTR(HLF_V) != HL_ATTR(HLF_VNC))
		redraw_curbuf_later(UPD_INVERTED_ALL);
	}
    }
#else
    // Only own the clipboard when we didn't own it yet.
    if (!cbd->owned && cbd->available)
	cbd->owned = (clip_gen_own_selection(cbd) == OK);
#endif
}

    static void
clip_gen_lose_selection(Clipboard_T *cbd)
{
#ifdef FEAT_XCLIPBOARD
# ifdef FEAT_GUI
    if (gui.in_use)
	clip_mch_lose_selection(cbd);
    else
# endif
	clip_xterm_lose_selection(cbd);
#else
    clip_mch_lose_selection(cbd);
#endif
}

    void
clip_lose_selection(Clipboard_T *cbd)
{
#ifdef FEAT_X11
    int	    was_owned = cbd->owned;
#endif
    int     visual_selection = FALSE;

    if (cbd == &clip_star || cbd == &clip_plus)
	visual_selection = TRUE;

    clip_free_selection(cbd);
    cbd->owned = FALSE;
    if (visual_selection)
	clip_clear_selection(cbd);
    clip_gen_lose_selection(cbd);
#ifdef FEAT_X11
    if (visual_selection)
    {
	// May have to show a different kind of highlighting for the selected
	// area.  There is no specific redraw command for this, just redraw all
	// windows on the current buffer.
	if (was_owned
		&& (get_real_state() == MODE_VISUAL
					    || get_real_state() == MODE_SELECT)
		&& (cbd == &clip_star ?
				clip_isautosel_star() : clip_isautosel_plus())
		&& HL_ATTR(HLF_V) != HL_ATTR(HLF_VNC)
		&& !exiting)
	{
	    update_curbuf(UPD_INVERTED_ALL);
	    setcursor();
	    cursor_on();
	    out_flush_cursor(TRUE, FALSE);
	}
    }
#endif
}

    static void
clip_copy_selection(Clipboard_T *clip)
{
    if (VIsual_active && (State & MODE_NORMAL) && clip->available)
    {
	clip_update_selection(clip);
	clip_free_selection(clip);
	clip_own_selection(clip);
	if (clip->owned)
	    clip_get_selection(clip);
	clip_gen_set_selection(clip);
    }
}

/*
 * Save and restore clip_unnamed before doing possibly many changes. This
 * prevents accessing the clipboard very often which might slow down Vim
 * considerably.
 */
static int global_change_count = 0; // if set, inside a start_global_changes
static int clipboard_needs_update = FALSE; // clipboard needs to be updated
static int clip_did_set_selection = TRUE;

/*
 * Save clip_unnamed and reset it.
 */
    void
start_global_changes(void)
{
    if (++global_change_count > 1)
	return;
    clip_unnamed_saved = clip_unnamed;
    clipboard_needs_update = FALSE;

    if (clip_did_set_selection)
    {
	clip_unnamed = 0;
	clip_did_set_selection = FALSE;
    }
}

/*
 * Return TRUE if setting the clipboard was postponed, it already contains the
 * right text.
 */
    static int
is_clipboard_needs_update(void)
{
    return clipboard_needs_update;
}

/*
 * Restore clip_unnamed and set the selection when needed.
 */
    void
end_global_changes(void)
{
    if (--global_change_count > 0)
	// recursive
	return;
    if (!clip_did_set_selection)
    {
	clip_did_set_selection = TRUE;
	clip_unnamed = clip_unnamed_saved;
	clip_unnamed_saved = 0;
	if (clipboard_needs_update)
	{
	    // only store something in the clipboard,
	    // if we have yanked anything to it
	    if (clip_unnamed & CLIP_UNNAMED)
	    {
		clip_own_selection(&clip_star);
		clip_gen_set_selection(&clip_star);
	    }
	    if (clip_unnamed & CLIP_UNNAMED_PLUS)
	    {
		clip_own_selection(&clip_plus);
		clip_gen_set_selection(&clip_plus);
	    }
	}
    }
    clipboard_needs_update = FALSE;
}

/*
 * Called when Visual mode is ended: update the selection.
 */
    void
clip_auto_select(void)
{
    if (clip_isautosel_star())
	clip_copy_selection(&clip_star);
    if (clip_isautosel_plus())
	clip_copy_selection(&clip_plus);
}

/*
 * Return TRUE if automatic selection of Visual area is desired for the *
 * register.
 */
    int
clip_isautosel_star(void)
{
    return (
#ifdef FEAT_GUI
	    gui.in_use ? (vim_strchr(p_go, GO_ASEL) != NULL) :
#endif
	    clip_autoselect_star);
}

/*
 * Return TRUE if automatic selection of Visual area is desired for the +
 * register.
 */
    int
clip_isautosel_plus(void)
{
    return (
#ifdef FEAT_GUI
	    gui.in_use ? (vim_strchr(p_go, GO_ASELPLUS) != NULL) :
#endif
	    clip_autoselect_plus);
}


/*
 * Stuff for general mouse selection, without using Visual mode.
 */

/*
 * Compare two screen positions ala strcmp()
 */
    static int
clip_compare_pos(
    int		row1,
    int		col1,
    int		row2,
    int		col2)
{
    if (row1 > row2) return(1);
    if (row1 < row2) return(-1);
    if (col1 > col2) return(1);
    if (col1 < col2) return(-1);
    return(0);
}

// "how" flags for clip_invert_area()
#define CLIP_CLEAR	1
#define CLIP_SET	2
#define CLIP_TOGGLE	3

/*
 * Invert or un-invert a rectangle of the screen.
 * "invert" is true if the result is inverted.
 */
    static void
clip_invert_rectangle(
	Clipboard_T	*cbd UNUSED,
	int		row_arg,
	int		col_arg,
	int		height_arg,
	int		width_arg,
	int		invert)
{
    int		row = row_arg;
    int		col = col_arg;
    int		height = height_arg;
    int		width = width_arg;

#ifdef FEAT_PROP_POPUP
    // this goes on top of all popup windows
    screen_zindex = CLIP_ZINDEX;

    if (col < cbd->min_col)
    {
	width -= cbd->min_col - col;
	col = cbd->min_col;
    }
    if (width > cbd->max_col - col)
	width = cbd->max_col - col;
    if (row < cbd->min_row)
    {
	height -= cbd->min_row - row;
	row = cbd->min_row;
    }
    if (height > cbd->max_row - row + 1)
	height = cbd->max_row - row + 1;
#endif
#ifdef FEAT_GUI
    if (gui.in_use)
	gui_mch_invert_rectangle(row, col, height, width);
    else
#endif
	screen_draw_rectangle(row, col, height, width, invert);
#ifdef FEAT_PROP_POPUP
    screen_zindex = 0;
#endif
}

/*
 * Invert a region of the display between a starting and ending row and column
 * Values for "how":
 * CLIP_CLEAR:  undo inversion
 * CLIP_SET:    set inversion
 * CLIP_TOGGLE: set inversion if pos1 < pos2, undo inversion otherwise.
 * 0: invert (GUI only).
 */
    static void
clip_invert_area(
	Clipboard_T	*cbd,
	int		row1,
	int		col1,
	int		row2,
	int		col2,
	int		how)
{
    int		invert = FALSE;
    int		max_col;

#ifdef FEAT_PROP_POPUP
    max_col = cbd->max_col - 1;
#else
    max_col = Columns - 1;
#endif

    if (how == CLIP_SET)
	invert = TRUE;

    // Swap the from and to positions so the from is always before
    if (clip_compare_pos(row1, col1, row2, col2) > 0)
    {
	int tmp_row, tmp_col;

	tmp_row = row1;
	tmp_col = col1;
	row1	= row2;
	col1	= col2;
	row2	= tmp_row;
	col2	= tmp_col;
    }
    else if (how == CLIP_TOGGLE)
	invert = TRUE;

    // If all on the same line, do it the easy way
    if (row1 == row2)
    {
	clip_invert_rectangle(cbd, row1, col1, 1, col2 - col1, invert);
    }
    else
    {
	// Handle a piece of the first line
	if (col1 > 0)
	{
	    clip_invert_rectangle(cbd, row1, col1, 1,
						  (int)Columns - col1, invert);
	    row1++;
	}

	// Handle a piece of the last line
	if (col2 < max_col)
	{
	    clip_invert_rectangle(cbd, row2, 0, 1, col2, invert);
	    row2--;
	}

	// Handle the rectangle that's left
	if (row2 >= row1)
	    clip_invert_rectangle(cbd, row1, 0, row2 - row1 + 1,
							 (int)Columns, invert);
    }
}

/*
 * Start, continue or end a modeless selection.  Used when editing the
 * command-line, in the cmdline window and when the mouse is in a popup window.
 */
    void
clip_modeless(int button, int is_click, int is_drag)
{
    int		repeat;

    repeat = ((clip_star.mode == SELECT_MODE_CHAR
		|| clip_star.mode == SELECT_MODE_LINE)
					      && (mod_mask & MOD_MASK_2CLICK))
	    || (clip_star.mode == SELECT_MODE_WORD
					     && (mod_mask & MOD_MASK_3CLICK));
    if (is_click && button == MOUSE_RIGHT)
    {
	// Right mouse button: If there was no selection, start one.
	// Otherwise extend the existing selection.
	if (clip_star.state == SELECT_CLEARED)
	    clip_start_selection(mouse_col, mouse_row, FALSE);
	clip_process_selection(button, mouse_col, mouse_row, repeat);
    }
    else if (is_click)
	clip_start_selection(mouse_col, mouse_row, repeat);
    else if (is_drag)
    {
	// Don't try extending a selection if there isn't one.  Happens when
	// button-down is in the cmdline and them moving mouse upwards.
	if (clip_star.state != SELECT_CLEARED)
	    clip_process_selection(button, mouse_col, mouse_row, repeat);
    }
    else // release
	clip_process_selection(MOUSE_RELEASE, mouse_col, mouse_row, FALSE);
}

/*
 * Update the currently selected region by adding and/or subtracting from the
 * beginning or end and inverting the changed area(s).
 */
    static void
clip_update_modeless_selection(
    Clipboard_T    *cb,
    int		    row1,
    int		    col1,
    int		    row2,
    int		    col2)
{
    // See if we changed at the beginning of the selection
    if (row1 != cb->start.lnum || col1 != (int)cb->start.col)
    {
	clip_invert_area(cb, row1, col1, (int)cb->start.lnum, cb->start.col,
								 CLIP_TOGGLE);
	cb->start.lnum = row1;
	cb->start.col  = col1;
    }

    // See if we changed at the end of the selection
    if (row2 != cb->end.lnum || col2 != (int)cb->end.col)
    {
	clip_invert_area(cb, (int)cb->end.lnum, cb->end.col, row2, col2,
								 CLIP_TOGGLE);
	cb->end.lnum = row2;
	cb->end.col  = col2;
    }
}

/*
 * Find the starting and ending positions of the word at the given row and
 * column.  Only white-separated words are recognized here.
 */
#define CHAR_CLASS(c)	(c <= ' ' ? ' ' : vim_iswordc(c))

    static void
clip_get_word_boundaries(Clipboard_T *cb, int row, int col)
{
    int		start_class;
    int		temp_col;
    char_u	*p;
    int		mboff;

    if (row >= screen_Rows || col >= screen_Columns || ScreenLines == NULL)
	return;

    p = ScreenLines + LineOffset[row];
    // Correct for starting in the right half of a double-wide char
    if (enc_dbcs != 0)
	col -= dbcs_screen_head_off(p, p + col);
    else if (enc_utf8 && p[col] == 0)
	--col;
    start_class = CHAR_CLASS(p[col]);

    temp_col = col;
    for ( ; temp_col > 0; temp_col--)
	if (enc_dbcs != 0
		   && (mboff = dbcs_screen_head_off(p, p + temp_col - 1)) > 0)
	    temp_col -= mboff;
	else if (CHAR_CLASS(p[temp_col - 1]) != start_class
		&& !(enc_utf8 && p[temp_col - 1] == 0))
	    break;
    cb->word_start_col = temp_col;

    temp_col = col;
    for ( ; temp_col < screen_Columns; temp_col++)
	if (enc_dbcs != 0 && dbcs_ptr2cells(p + temp_col) == 2)
	    ++temp_col;
	else if (CHAR_CLASS(p[temp_col]) != start_class
		&& !(enc_utf8 && p[temp_col] == 0))
	    break;
    cb->word_end_col = temp_col;
}

/*
 * Find the column position for the last non-whitespace character on the given
 * line at or before start_col.
 */
    static int
clip_get_line_end(Clipboard_T *cbd UNUSED, int row)
{
    int	    i;

    if (row >= screen_Rows || ScreenLines == NULL)
	return 0;
    for (i =
#ifdef FEAT_PROP_POPUP
	    cbd->max_col;
#else
	    screen_Columns;
#endif
			    i > 0; i--)
	if (ScreenLines[LineOffset[row] + i - 1] != ' ')
	    break;
    return i;
}

/*
 * Start the selection
 */
    void
clip_start_selection(int col, int row, int repeated_click)
{
    Clipboard_T	*cb = &clip_star;
#ifdef FEAT_PROP_POPUP
    win_T	*wp;
    int		row_cp = row;
    int		col_cp = col;

    wp = mouse_find_win(&row_cp, &col_cp, FIND_POPUP);
    if (wp != NULL && WIN_IS_POPUP(wp)
				  && popup_is_in_scrollbar(wp, row_cp, col_cp))
	// click or double click in scrollbar does not start a selection
	return;
#endif

    if (cb->state == SELECT_DONE)
	clip_clear_selection(cb);

    row = check_row(row);
    col = check_col(col);
    col = mb_fix_col(col, row);

    cb->start.lnum  = row;
    cb->start.col   = col;
    cb->end	    = cb->start;
    cb->origin_row  = (short_u)cb->start.lnum;
    cb->state	    = SELECT_IN_PROGRESS;
#ifdef FEAT_PROP_POPUP
    if (wp != NULL && WIN_IS_POPUP(wp))
    {
	// Click in a popup window restricts selection to that window,
	// excluding the border.
	cb->min_col = wp->w_wincol + wp->w_popup_border[3];
	cb->max_col = wp->w_wincol + popup_width(wp)
				 - wp->w_popup_border[1] - wp->w_has_scrollbar;
	if (cb->max_col > screen_Columns)
	    cb->max_col = screen_Columns;
	cb->min_row = wp->w_winrow + wp->w_popup_border[0];
	cb->max_row = wp->w_winrow + popup_height(wp) - 1
						   - wp->w_popup_border[2];
    }
    else
    {
	cb->min_col = 0;
	cb->max_col = screen_Columns;
	cb->min_row = 0;
	cb->max_row = screen_Rows;
    }
#endif

    if (repeated_click)
    {
	if (++cb->mode > SELECT_MODE_LINE)
	    cb->mode = SELECT_MODE_CHAR;
    }
    else
	cb->mode = SELECT_MODE_CHAR;

#ifdef FEAT_GUI
    // clear the cursor until the selection is made
    if (gui.in_use)
	gui_undraw_cursor();
#endif

    switch (cb->mode)
    {
	case SELECT_MODE_CHAR:
	    cb->origin_start_col = cb->start.col;
	    cb->word_end_col = clip_get_line_end(cb, (int)cb->start.lnum);
	    break;

	case SELECT_MODE_WORD:
	    clip_get_word_boundaries(cb, (int)cb->start.lnum, cb->start.col);
	    cb->origin_start_col = cb->word_start_col;
	    cb->origin_end_col	 = cb->word_end_col;

	    clip_invert_area(cb, (int)cb->start.lnum, cb->word_start_col,
			    (int)cb->end.lnum, cb->word_end_col, CLIP_SET);
	    cb->start.col = cb->word_start_col;
	    cb->end.col   = cb->word_end_col;
	    break;

	case SELECT_MODE_LINE:
	    clip_invert_area(cb, (int)cb->start.lnum, 0, (int)cb->start.lnum,
			    (int)Columns, CLIP_SET);
	    cb->start.col = 0;
	    cb->end.col   = Columns;
	    break;
    }

    cb->prev = cb->start;

#ifdef DEBUG_SELECTION
    printf("Selection started at (%ld,%d)\n", cb->start.lnum, cb->start.col);
#endif
}

/*
 * Continue processing the selection
 */
    void
clip_process_selection(
    int		button,
    int		col,
    int		row,
    int_u	repeated_click)
{
    Clipboard_T	*cb = &clip_star;
    int		diff;
    int		slen = 1;	// cursor shape width

    if (button == MOUSE_RELEASE)
    {
	if (cb->state != SELECT_IN_PROGRESS)
	    return;

	// Check to make sure we have something selected
	if (cb->start.lnum == cb->end.lnum && cb->start.col == cb->end.col)
	{
#ifdef FEAT_GUI
	    if (gui.in_use)
		gui_update_cursor(FALSE, FALSE);
#endif
	    cb->state = SELECT_CLEARED;
	    return;
	}

#ifdef DEBUG_SELECTION
	printf("Selection ended: (%ld,%d) to (%ld,%d)\n", cb->start.lnum,
		cb->start.col, cb->end.lnum, cb->end.col);
#endif
	if (clip_isautosel_star()
		|| (
#ifdef FEAT_GUI
		    gui.in_use ? (vim_strchr(p_go, GO_ASELML) != NULL) :
#endif
		    clip_autoselectml))
	    clip_copy_modeless_selection(FALSE);
#ifdef FEAT_GUI
	if (gui.in_use)
	    gui_update_cursor(FALSE, FALSE);
#endif

	cb->state = SELECT_DONE;
	return;
    }

    row = check_row(row);
    col = check_col(col);
    col = mb_fix_col(col, row);

    if (col == (int)cb->prev.col && row == cb->prev.lnum && !repeated_click)
	return;

    /*
     * When extending the selection with the right mouse button, swap the
     * start and end if the position is before half the selection
     */
    if (cb->state == SELECT_DONE && button == MOUSE_RIGHT)
    {
	/*
	 * If the click is before the start, or the click is inside the
	 * selection and the start is the closest side, set the origin to the
	 * end of the selection.
	 */
	if (clip_compare_pos(row, col, (int)cb->start.lnum, cb->start.col) < 0
		|| (clip_compare_pos(row, col,
					   (int)cb->end.lnum, cb->end.col) < 0
		    && (((cb->start.lnum == cb->end.lnum
			    && cb->end.col - col > col - cb->start.col))
			|| ((diff = (cb->end.lnum - row) -
						   (row - cb->start.lnum)) > 0
			    || (diff == 0 && col < (int)(cb->start.col +
							 cb->end.col) / 2)))))
	{
	    cb->origin_row = (short_u)cb->end.lnum;
	    cb->origin_start_col = cb->end.col - 1;
	    cb->origin_end_col = cb->end.col;
	}
	else
	{
	    cb->origin_row = (short_u)cb->start.lnum;
	    cb->origin_start_col = cb->start.col;
	    cb->origin_end_col = cb->start.col;
	}
	if (cb->mode == SELECT_MODE_WORD && !repeated_click)
	    cb->mode = SELECT_MODE_CHAR;
    }

    // set state, for when using the right mouse button
    cb->state = SELECT_IN_PROGRESS;

#ifdef DEBUG_SELECTION
    printf("Selection extending to (%d,%d)\n", row, col);
#endif

    if (repeated_click && ++cb->mode > SELECT_MODE_LINE)
	cb->mode = SELECT_MODE_CHAR;

    switch (cb->mode)
    {
	case SELECT_MODE_CHAR:
	    // If we're on a different line, find where the line ends
	    if (row != cb->prev.lnum)
		cb->word_end_col = clip_get_line_end(cb, row);

	    // See if we are before or after the origin of the selection
	    if (clip_compare_pos(row, col, cb->origin_row,
						   cb->origin_start_col) >= 0)
	    {
		if (col >= (int)cb->word_end_col)
		    clip_update_modeless_selection(cb, cb->origin_row,
			    cb->origin_start_col, row, (int)Columns);
		else
		{
		    if (has_mbyte && mb_lefthalve(row, col))
			slen = 2;
		    clip_update_modeless_selection(cb, cb->origin_row,
			    cb->origin_start_col, row, col + slen);
		}
	    }
	    else
	    {
		if (has_mbyte
			&& mb_lefthalve(cb->origin_row, cb->origin_start_col))
		    slen = 2;
		if (col >= (int)cb->word_end_col)
		    clip_update_modeless_selection(cb, row, cb->word_end_col,
			    cb->origin_row, cb->origin_start_col + slen);
		else
		    clip_update_modeless_selection(cb, row, col,
			    cb->origin_row, cb->origin_start_col + slen);
	    }
	    break;

	case SELECT_MODE_WORD:
	    // If we are still within the same word, do nothing
	    if (row == cb->prev.lnum && col >= (int)cb->word_start_col
		    && col < (int)cb->word_end_col && !repeated_click)
		return;

	    // Get new word boundaries
	    clip_get_word_boundaries(cb, row, col);

	    // Handle being after the origin point of selection
	    if (clip_compare_pos(row, col, cb->origin_row,
		    cb->origin_start_col) >= 0)
		clip_update_modeless_selection(cb, cb->origin_row,
			cb->origin_start_col, row, cb->word_end_col);
	    else
		clip_update_modeless_selection(cb, row, cb->word_start_col,
			cb->origin_row, cb->origin_end_col);
	    break;

	case SELECT_MODE_LINE:
	    if (row == cb->prev.lnum && !repeated_click)
		return;

	    if (clip_compare_pos(row, col, cb->origin_row,
		    cb->origin_start_col) >= 0)
		clip_update_modeless_selection(cb, cb->origin_row, 0, row,
			(int)Columns);
	    else
		clip_update_modeless_selection(cb, row, 0, cb->origin_row,
			(int)Columns);
	    break;
    }

    cb->prev.lnum = row;
    cb->prev.col  = col;

#ifdef DEBUG_SELECTION
	printf("Selection is: (%ld,%d) to (%ld,%d)\n", cb->start.lnum,
		cb->start.col, cb->end.lnum, cb->end.col);
#endif
}

# if defined(FEAT_GUI) || defined(PROTO)
/*
 * Redraw part of the selection if character at "row,col" is inside of it.
 * Only used for the GUI.
 */
    void
clip_may_redraw_selection(int row, int col, int len)
{
    int		start = col;
    int		end = col + len;

    if (clip_star.state != SELECT_CLEARED
	    && row >= clip_star.start.lnum
	    && row <= clip_star.end.lnum)
    {
	if (row == clip_star.start.lnum && start < (int)clip_star.start.col)
	    start = clip_star.start.col;
	if (row == clip_star.end.lnum && end > (int)clip_star.end.col)
	    end = clip_star.end.col;
	if (end > start)
	    clip_invert_area(&clip_star, row, start, row, end, 0);
    }
}
# endif

/*
 * Called from outside to clear selected region from the display
 */
    void
clip_clear_selection(Clipboard_T *cbd)
{

    if (cbd->state == SELECT_CLEARED)
	return;

    clip_invert_area(cbd, (int)cbd->start.lnum, cbd->start.col,
				 (int)cbd->end.lnum, cbd->end.col, CLIP_CLEAR);
    cbd->state = SELECT_CLEARED;
}

/*
 * Clear the selection if any lines from "row1" to "row2" are inside of it.
 */
    void
clip_may_clear_selection(int row1, int row2)
{
    if (clip_star.state == SELECT_DONE
	    && row2 >= clip_star.start.lnum
	    && row1 <= clip_star.end.lnum)
	clip_clear_selection(&clip_star);
}

/*
 * Called before the screen is scrolled up or down.  Adjusts the line numbers
 * of the selection.  Call with big number when clearing the screen.
 */
    void
clip_scroll_selection(
    int	    rows)		// negative for scroll down
{
    int	    lnum;

    if (clip_star.state == SELECT_CLEARED)
	return;

    lnum = clip_star.start.lnum - rows;
    if (lnum <= 0)
	clip_star.start.lnum = 0;
    else if (lnum >= screen_Rows)	// scrolled off of the screen
	clip_star.state = SELECT_CLEARED;
    else
	clip_star.start.lnum = lnum;

    lnum = clip_star.end.lnum - rows;
    if (lnum < 0)			// scrolled off of the screen
	clip_star.state = SELECT_CLEARED;
    else if (lnum >= screen_Rows)
	clip_star.end.lnum = screen_Rows - 1;
    else
	clip_star.end.lnum = lnum;
}

/*
 * Copy the currently selected area into the '*' register so it will be
 * available for pasting.
 * When "both" is TRUE also copy to the '+' register.
 */
    void
clip_copy_modeless_selection(int both UNUSED)
{
    char_u	*buffer;
    char_u	*bufp;
    int		row;
    int		start_col;
    int		end_col;
    int		line_end_col;
    int		add_newline_flag = FALSE;
    int		len;
    char_u	*p;
    int		row1 = clip_star.start.lnum;
    int		col1 = clip_star.start.col;
    int		row2 = clip_star.end.lnum;
    int		col2 = clip_star.end.col;

    // Can't use ScreenLines unless initialized
    if (ScreenLines == NULL)
	return;

    /*
     * Make sure row1 <= row2, and if row1 == row2 that col1 <= col2.
     */
    if (row1 > row2)
    {
	row = row1; row1 = row2; row2 = row;
	row = col1; col1 = col2; col2 = row;
    }
    else if (row1 == row2 && col1 > col2)
    {
	row = col1; col1 = col2; col2 = row;
    }
#ifdef FEAT_PROP_POPUP
    if (col1 < clip_star.min_col)
	col1 = clip_star.min_col;
    if (col2 > clip_star.max_col)
	col2 = clip_star.max_col;
    if (row1 > clip_star.max_row || row2 < clip_star.min_row)
	return;
    if (row1 < clip_star.min_row)
	row1 = clip_star.min_row;
    if (row2 > clip_star.max_row)
	row2 = clip_star.max_row;
#endif
    // correct starting point for being on right half of double-wide char
    p = ScreenLines + LineOffset[row1];
    if (enc_dbcs != 0)
	col1 -= (*mb_head_off)(p, p + col1);
    else if (enc_utf8 && p[col1] == 0)
	--col1;

    // Create a temporary buffer for storing the text
    len = (row2 - row1 + 1) * Columns + 1;
    if (enc_dbcs != 0)
	len *= 2;	// max. 2 bytes per display cell
    else if (enc_utf8)
	len *= MB_MAXBYTES;
    buffer = alloc(len);
    if (buffer == NULL)	    // out of memory
	return;

    // Process each row in the selection
    for (bufp = buffer, row = row1; row <= row2; row++)
    {
	if (row == row1)
	    start_col = col1;
	else
#ifdef FEAT_PROP_POPUP
	    start_col = clip_star.min_col;
#else
	    start_col = 0;
#endif

	if (row == row2)
	    end_col = col2;
	else
#ifdef FEAT_PROP_POPUP
	    end_col = clip_star.max_col;
#else
	    end_col = Columns;
#endif

	line_end_col = clip_get_line_end(&clip_star, row);

	// See if we need to nuke some trailing whitespace
	if (end_col >=
#ifdef FEAT_PROP_POPUP
		clip_star.max_col
#else
		Columns
#endif
		    && (row < row2 || end_col > line_end_col))
	{
	    // Get rid of trailing whitespace
	    end_col = line_end_col;
	    if (end_col < start_col)
		end_col = start_col;

	    // If the last line extended to the end, add an extra newline
	    if (row == row2)
		add_newline_flag = TRUE;
	}

	// If after the first row, we need to always add a newline
	if (row > row1 && !LineWraps[row - 1])
	    *bufp++ = NL;

	// Safetey check for in case resizing went wrong
	if (row < screen_Rows && end_col <= screen_Columns)
	{
	    if (enc_dbcs != 0)
	    {
		int	i;

		p = ScreenLines + LineOffset[row];
		for (i = start_col; i < end_col; ++i)
		    if (enc_dbcs == DBCS_JPNU && p[i] == 0x8e)
		    {
			// single-width double-byte char
			*bufp++ = 0x8e;
			*bufp++ = ScreenLines2[LineOffset[row] + i];
		    }
		    else
		    {
			*bufp++ = p[i];
			if (MB_BYTE2LEN(p[i]) == 2)
			    *bufp++ = p[++i];
		    }
	    }
	    else if (enc_utf8)
	    {
		int	off;
		int	i;
		int	ci;

		off = LineOffset[row];
		for (i = start_col; i < end_col; ++i)
		{
		    // The base character is either in ScreenLinesUC[] or
		    // ScreenLines[].
		    if (ScreenLinesUC[off + i] == 0)
			*bufp++ = ScreenLines[off + i];
		    else
		    {
			bufp += utf_char2bytes(ScreenLinesUC[off + i], bufp);
			for (ci = 0; ci < Screen_mco; ++ci)
			{
			    // Add a composing character.
			    if (ScreenLinesC[ci][off + i] == 0)
				break;
			    bufp += utf_char2bytes(ScreenLinesC[ci][off + i],
									bufp);
			}
		    }
		    // Skip right half of double-wide character.
		    if (ScreenLines[off + i + 1] == 0)
			++i;
		}
	    }
	    else
	    {
		STRNCPY(bufp, ScreenLines + LineOffset[row] + start_col,
							 end_col - start_col);
		bufp += end_col - start_col;
	    }
	}
    }

    // Add a newline at the end if the selection ended there
    if (add_newline_flag)
	*bufp++ = NL;

    // First cleanup any old selection and become the owner.
    clip_free_selection(&clip_star);
    clip_own_selection(&clip_star);

    // Yank the text into the '*' register.
    clip_yank_selection(MCHAR, buffer, (long)(bufp - buffer), &clip_star);

    // Make the register contents available to the outside world.
    clip_gen_set_selection(&clip_star);

#ifdef FEAT_X11
    if (both)
    {
	// Do the same for the '+' register.
	clip_free_selection(&clip_plus);
	clip_own_selection(&clip_plus);
	clip_yank_selection(MCHAR, buffer, (long)(bufp - buffer), &clip_plus);
	clip_gen_set_selection(&clip_plus);
    }
#endif
    vim_free(buffer);
}

    void
clip_gen_set_selection(Clipboard_T *cbd)
{
    if (!clip_did_set_selection)
    {
	// Updating postponed, so that accessing the system clipboard won't
	// hang Vim when accessing it many times (e.g. on a :g command).
	if ((cbd == &clip_plus && (clip_unnamed_saved & CLIP_UNNAMED_PLUS))
		|| (cbd == &clip_star && (clip_unnamed_saved & CLIP_UNNAMED)))
	{
	    clipboard_needs_update = TRUE;
	    return;
	}
    }
#ifdef FEAT_XCLIPBOARD
# ifdef FEAT_GUI
    if (gui.in_use)
	clip_mch_set_selection(cbd);
    else
# endif
	clip_xterm_set_selection(cbd);
#else
    clip_mch_set_selection(cbd);
#endif
}

    static void
clip_gen_request_selection(Clipboard_T *cbd)
{
#ifdef FEAT_XCLIPBOARD
# ifdef FEAT_GUI
    if (gui.in_use)
	clip_mch_request_selection(cbd);
    else
# endif
	clip_xterm_request_selection(cbd);
#else
    clip_mch_request_selection(cbd);
#endif
}

#if (defined(FEAT_X11) && defined(FEAT_XCLIPBOARD) && defined(USE_SYSTEM)) \
	|| defined(PROTO)
    static int
clip_x11_owner_exists(Clipboard_T *cbd)
{
    return XGetSelectionOwner(X_DISPLAY, cbd->sel_atom) != None;
}
#endif

#if (defined(FEAT_X11) && defined(USE_SYSTEM)) || defined(PROTO)
    int
clip_gen_owner_exists(Clipboard_T *cbd UNUSED)
{
#ifdef FEAT_XCLIPBOARD
# ifdef FEAT_GUI_GTK
    if (gui.in_use)
	return clip_gtk_owner_exists(cbd);
    else
# endif
	return clip_x11_owner_exists(cbd);
#else
    return TRUE;
#endif
}
#endif

/*
 * Extract the items in the 'clipboard' option and set global values.
 * Return an error message or NULL for success.
 */
    char *
did_set_clipboard(optset_T *args UNUSED)
{
    int		new_unnamed = 0;
    int		new_autoselect_star = FALSE;
    int		new_autoselect_plus = FALSE;
    int		new_autoselectml = FALSE;
    int		new_html = FALSE;
    regprog_T	*new_exclude_prog = NULL;
    char	*errmsg = NULL;
    char_u	*p;

    for (p = p_cb; *p != NUL; )
    {
	// Note: Keep this in sync with p_cb_values.
	if (STRNCMP(p, "unnamed", 7) == 0 && (p[7] == ',' || p[7] == NUL))
	{
	    new_unnamed |= CLIP_UNNAMED;
	    p += 7;
	}
	else if (STRNCMP(p, "unnamedplus", 11) == 0
					    && (p[11] == ',' || p[11] == NUL))
	{
	    new_unnamed |= CLIP_UNNAMED_PLUS;
	    p += 11;
	}
	else if (STRNCMP(p, "autoselect", 10) == 0
					    && (p[10] == ',' || p[10] == NUL))
	{
	    new_autoselect_star = TRUE;
	    p += 10;
	}
	else if (STRNCMP(p, "autoselectplus", 14) == 0
					    && (p[14] == ',' || p[14] == NUL))
	{
	    new_autoselect_plus = TRUE;
	    p += 14;
	}
	else if (STRNCMP(p, "autoselectml", 12) == 0
					    && (p[12] == ',' || p[12] == NUL))
	{
	    new_autoselectml = TRUE;
	    p += 12;
	}
	else if (STRNCMP(p, "html", 4) == 0 && (p[4] == ',' || p[4] == NUL))
	{
	    new_html = TRUE;
	    p += 4;
	}
	else if (STRNCMP(p, "exclude:", 8) == 0 && new_exclude_prog == NULL)
	{
	    p += 8;
	    new_exclude_prog = vim_regcomp(p, RE_MAGIC);
	    if (new_exclude_prog == NULL)
		errmsg = e_invalid_argument;
	    break;
	}
	else
	{
	    errmsg = e_invalid_argument;
	    break;
	}
	if (*p == ',')
	    ++p;
    }
    if (errmsg == NULL)
    {
	if (global_busy)
	    // clip_unnamed will be reset to clip_unnamed_saved
	    // at end_global_changes
	    clip_unnamed_saved = new_unnamed;
	else
	    clip_unnamed = new_unnamed;
	clip_autoselect_star = new_autoselect_star;
	clip_autoselect_plus = new_autoselect_plus;
	clip_autoselectml = new_autoselectml;
	clip_html = new_html;
	vim_regfree(clip_exclude_prog);
	clip_exclude_prog = new_exclude_prog;
#ifdef FEAT_GUI_GTK
	if (gui.in_use)
	{
	    gui_gtk_set_selection_targets((GdkAtom)GDK_SELECTION_PRIMARY);
	    gui_gtk_set_selection_targets((GdkAtom)clip_plus.gtk_sel_atom);
	    gui_gtk_set_dnd_targets();
	}
#endif
    }
    else
	vim_regfree(new_exclude_prog);

    return errmsg;
}

/*
 * Stuff for the X clipboard.  Shared between VMS and Unix.
 */

#if defined(FEAT_XCLIPBOARD) || defined(FEAT_GUI_X11) || defined(PROTO)
# include <X11/Xatom.h>
# include <X11/Intrinsic.h>

/*
 * Open the application context (if it hasn't been opened yet).
 * Used for Motif GUI and the xterm clipboard.
 */
    void
open_app_context(void)
{
    if (app_context == NULL)
    {
	XtToolkitInitialize();
	app_context = XtCreateApplicationContext();
    }
}

static Atom	vim_atom;	// Vim's own special selection format
static Atom	vimenc_atom;	// Vim's extended selection format
static Atom	utf8_atom;
static Atom	compound_text_atom;
static Atom	text_atom;
static Atom	targets_atom;
static Atom	timestamp_atom;	// Used to get a timestamp

    void
x11_setup_atoms(Display *dpy)
{
    vim_atom	       = XInternAtom(dpy, VIM_ATOM_NAME,   False);
    vimenc_atom	       = XInternAtom(dpy, VIMENC_ATOM_NAME,False);
    utf8_atom	       = XInternAtom(dpy, "UTF8_STRING",   False);
    compound_text_atom = XInternAtom(dpy, "COMPOUND_TEXT", False);
    text_atom	       = XInternAtom(dpy, "TEXT",	   False);
    targets_atom       = XInternAtom(dpy, "TARGETS",	   False);
    clip_star.sel_atom = XA_PRIMARY;
    clip_plus.sel_atom = XInternAtom(dpy, "CLIPBOARD",	   False);
    timestamp_atom     = XInternAtom(dpy, "TIMESTAMP",	   False);
}

/*
 * X Selection stuff, for cutting and pasting text to other windows.
 */

    static Boolean
clip_x11_convert_selection_cb(
    Widget	w UNUSED,
    Atom	*sel_atom,
    Atom	*target,
    Atom	*type,
    XtPointer	*value,
    long_u	*length,
    int		*format)
{
    static char_u   *save_result = NULL;
    static long_u   save_length = 0;
    char_u	    *string;
    int		    motion_type;
    Clipboard_T    *cbd;
    int		    i;

    if (*sel_atom == clip_plus.sel_atom)
	cbd = &clip_plus;
    else
	cbd = &clip_star;

    if (!cbd->owned)
	return False;	    // Shouldn't ever happen

    // requestor wants to know what target types we support
    if (*target == targets_atom)
    {
	static Atom array[7];

	*value = (XtPointer)array;
	i = 0;
	array[i++] = targets_atom;
	array[i++] = vimenc_atom;
	array[i++] = vim_atom;
	if (enc_utf8)
	    array[i++] = utf8_atom;
	array[i++] = XA_STRING;
	array[i++] = text_atom;
	array[i++] = compound_text_atom;

	*type = XA_ATOM;
	// This used to be: *format = sizeof(Atom) * 8; but that caused
	// crashes on 64 bit machines. (Peter Derr)
	*format = 32;
	*length = i;
	return True;
    }

    if (       *target != XA_STRING
	    && *target != vimenc_atom
	    && (*target != utf8_atom || !enc_utf8)
	    && *target != vim_atom
	    && *target != text_atom
	    && *target != compound_text_atom)
	return False;

    clip_get_selection(cbd);
    motion_type = clip_convert_selection(&string, length, cbd);
    if (motion_type < 0)
	return False;

    // For our own format, the first byte contains the motion type
    if (*target == vim_atom)
	(*length)++;

    // Our own format with encoding: motion 'encoding' NUL text
    if (*target == vimenc_atom)
	*length += STRLEN(p_enc) + 2;

    if (save_length < *length || save_length / 2 >= *length)
	*value = XtRealloc((char *)save_result, (Cardinal)*length + 1);
    else
	*value = save_result;
    if (*value == NULL)
    {
	vim_free(string);
	return False;
    }
    save_result = (char_u *)*value;
    save_length = *length;

    if (*target == XA_STRING || (*target == utf8_atom && enc_utf8))
    {
	mch_memmove(save_result, string, (size_t)(*length));
	*type = *target;
    }
    else if (*target == compound_text_atom || *target == text_atom)
    {
	XTextProperty	text_prop;
	char		*string_nt = (char *)save_result;
	int		conv_result;

	// create NUL terminated string which XmbTextListToTextProperty wants
	mch_memmove(string_nt, string, (size_t)*length);
	string_nt[*length] = NUL;
	conv_result = XmbTextListToTextProperty(X_DISPLAY, &string_nt,
					   1, XCompoundTextStyle, &text_prop);
	if (conv_result != Success)
	{
	    vim_free(string);
	    return False;
	}
	*value = (XtPointer)(text_prop.value);	//    from plain text
	*length = text_prop.nitems;
	*type = compound_text_atom;
	XtFree((char *)save_result);
	save_result = (char_u *)*value;
	save_length = *length;
    }
    else if (*target == vimenc_atom)
    {
	int l = STRLEN(p_enc);

	save_result[0] = motion_type;
	STRCPY(save_result + 1, p_enc);
	mch_memmove(save_result + l + 2, string, (size_t)(*length - l - 2));
	*type = vimenc_atom;
    }
    else
    {
	save_result[0] = motion_type;
	mch_memmove(save_result + 1, string, (size_t)(*length - 1));
	*type = vim_atom;
    }
    *format = 8;	    // 8 bits per char
    vim_free(string);
    return True;
}

    static void
clip_x11_lose_ownership_cb(Widget w UNUSED, Atom *sel_atom)
{
    if (*sel_atom == clip_plus.sel_atom)
	clip_lose_selection(&clip_plus);
    else
	clip_lose_selection(&clip_star);
}

    static void
clip_x11_notify_cb(Widget w UNUSED, Atom *sel_atom UNUSED, Atom *target UNUSED)
{
    // To prevent automatically freeing the selection value.
}

/*
 * Property callback to get a timestamp for XtOwnSelection.
 */
# if (defined(FEAT_X11) && defined(FEAT_XCLIPBOARD)) || defined(PROTO)
    static void
clip_x11_timestamp_cb(
    Widget	w,
    XtPointer	n UNUSED,
    XEvent	*event,
    Boolean	*cont UNUSED)
{
    Atom	    actual_type;
    int		    format;
    unsigned  long  nitems, bytes_after;
    unsigned char   *prop=NULL;
    XPropertyEvent  *xproperty=&event->xproperty;

    // Must be a property notify, state can't be Delete (True), has to be
    // one of the supported selection types.
    if (event->type != PropertyNotify || xproperty->state
	    || (xproperty->atom != clip_star.sel_atom
				    && xproperty->atom != clip_plus.sel_atom))
	return;

    if (XGetWindowProperty(xproperty->display, xproperty->window,
	  xproperty->atom, 0, 0, False, timestamp_atom, &actual_type, &format,
						&nitems, &bytes_after, &prop))
	return;

    if (prop)
	XFree(prop);

    // Make sure the property type is "TIMESTAMP" and it's 32 bits.
    if (actual_type != timestamp_atom || format != 32)
	return;

    // Get the selection, using the event timestamp.
    if (XtOwnSelection(w, xproperty->atom, xproperty->time,
	    clip_x11_convert_selection_cb, clip_x11_lose_ownership_cb,
	    clip_x11_notify_cb) == OK)
    {
	// Set the "owned" flag now, there may have been a call to
	// lose_ownership_cb in between.
	if (xproperty->atom == clip_plus.sel_atom)
	    clip_plus.owned = TRUE;
	else
	    clip_star.owned = TRUE;
    }
}

    void
x11_setup_selection(Widget w)
{
    XtAddEventHandler(w, PropertyChangeMask, False,
	    /*(XtEventHandler)*/clip_x11_timestamp_cb, (XtPointer)NULL);
}
# endif

    static void
clip_x11_request_selection_cb(
    Widget	w UNUSED,
    XtPointer	success,
    Atom	*sel_atom,
    Atom	*type,
    XtPointer	value,
    long_u	*length,
    int		*format)
{
    int		motion_type = MAUTO;
    long_u	len;
    char_u	*p;
    char	**text_list = NULL;
    Clipboard_T	*cbd;
    char_u	*tmpbuf = NULL;

    if (*sel_atom == clip_plus.sel_atom)
	cbd = &clip_plus;
    else
	cbd = &clip_star;

    if (value == NULL || *length == 0)
    {
	clip_free_selection(cbd);	// nothing received, clear register
	*(int *)success = FALSE;
	return;
    }
    p = (char_u *)value;
    len = *length;
    if (*type == vim_atom)
    {
	motion_type = *p++;
	len--;
    }

    else if (*type == vimenc_atom)
    {
	char_u		*enc;
	vimconv_T	conv;
	int		convlen;

	motion_type = *p++;
	--len;

	enc = p;
	p += STRLEN(p) + 1;
	len -= p - enc;

	// If the encoding of the text is different from 'encoding', attempt
	// converting it.
	conv.vc_type = CONV_NONE;
	convert_setup(&conv, enc, p_enc);
	if (conv.vc_type != CONV_NONE)
	{
	    convlen = len;	// Need to use an int here.
	    tmpbuf = string_convert(&conv, p, &convlen);
	    len = convlen;
	    if (tmpbuf != NULL)
		p = tmpbuf;
	    convert_setup(&conv, NULL, NULL);
	}
    }

    else if (*type == compound_text_atom
	    || *type == utf8_atom
	    || (enc_dbcs != 0 && *type == text_atom))
    {
	XTextProperty	text_prop;
	int		n_text = 0;
	int		status;

	text_prop.value = (unsigned char *)value;
	text_prop.encoding = *type;
	text_prop.format = *format;
	text_prop.nitems = len;
#if defined(X_HAVE_UTF8_STRING)
	if (*type == utf8_atom)
	    status = Xutf8TextPropertyToTextList(X_DISPLAY, &text_prop,
							 &text_list, &n_text);
	else
#endif
	    status = XmbTextPropertyToTextList(X_DISPLAY, &text_prop,
							 &text_list, &n_text);
	if (status != Success || n_text < 1)
	{
	    *(int *)success = FALSE;
	    return;
	}
	p = (char_u *)text_list[0];
	len = STRLEN(p);
    }
    clip_yank_selection(motion_type, p, (long)len, cbd);

    if (text_list != NULL)
	XFreeStringList(text_list);
    vim_free(tmpbuf);
    XtFree((char *)value);
    *(int *)success = TRUE;
}

    void
clip_x11_request_selection(
    Widget	myShell,
    Display	*dpy,
    Clipboard_T	*cbd)
{
    XEvent	event;
    Atom	type;
    static int	success;
    int		i;
    time_t	start_time;
    int		timed_out = FALSE;

    for (i = 0; i < 6; i++)
    {
	switch (i)
	{
	    case 0:  type = vimenc_atom;	break;
	    case 1:  type = vim_atom;		break;
	    case 2:  type = utf8_atom;		break;
	    case 3:  type = compound_text_atom; break;
	    case 4:  type = text_atom;		break;
	    default: type = XA_STRING;
	}
	if (type == utf8_atom
# if defined(X_HAVE_UTF8_STRING)
		&& !enc_utf8
# endif
		)
	    // Only request utf-8 when 'encoding' is utf8 and
	    // Xutf8TextPropertyToTextList is available.
	    continue;
	success = MAYBE;
	XtGetSelectionValue(myShell, cbd->sel_atom, type,
	    clip_x11_request_selection_cb, (XtPointer)&success, CurrentTime);

	// Make sure the request for the selection goes out before waiting for
	// a response.
	XFlush(dpy);

	/*
	 * Wait for result of selection request, otherwise if we type more
	 * characters, then they will appear before the one that requested the
	 * paste!  Don't worry, we will catch up with any other events later.
	 */
	start_time = time(NULL);
	while (success == MAYBE)
	{
	    if (XCheckTypedEvent(dpy, PropertyNotify, &event)
		    || XCheckTypedEvent(dpy, SelectionNotify, &event)
		    || XCheckTypedEvent(dpy, SelectionRequest, &event))
	    {
		// This is where clip_x11_request_selection_cb() should be
		// called.  It may actually happen a bit later, so we loop
		// until "success" changes.
		// We may get a SelectionRequest here and if we don't handle
		// it we hang.  KDE klipper does this, for example.
		// We need to handle a PropertyNotify for large selections.
		XtDispatchEvent(&event);
		continue;
	    }

	    // Time out after 2 to 3 seconds to avoid that we hang when the
	    // other process doesn't respond.  Note that the SelectionNotify
	    // event may still come later when the selection owner comes back
	    // to life and the text gets inserted unexpectedly.  Don't know
	    // why that happens or how to avoid that :-(.
	    if (time(NULL) > start_time + 2)
	    {
		timed_out = TRUE;
		break;
	    }

	    // Do we need this?  Probably not.
	    XSync(dpy, False);

	    // Wait for 1 msec to avoid that we eat up all CPU time.
	    ui_delay(1L, TRUE);
	}

	if (success == TRUE)
	    return;

	// don't do a retry with another type after timing out, otherwise we
	// hang for 15 seconds.
	if (timed_out)
	    break;
    }

    // Final fallback position - use the X CUT_BUFFER0 store
    yank_cut_buffer0(dpy, cbd);
}

    void
clip_x11_lose_selection(Widget myShell, Clipboard_T *cbd)
{
    XtDisownSelection(myShell, cbd->sel_atom,
				XtLastTimestampProcessed(XtDisplay(myShell)));
}

    int
clip_x11_own_selection(Widget myShell, Clipboard_T *cbd)
{
    // When using the GUI we have proper timestamps, use the one of the last
    // event.  When in the console we don't get events (the terminal gets
    // them), Get the time by a zero-length append, clip_x11_timestamp_cb will
    // be called with the current timestamp.
#ifdef FEAT_GUI
    if (gui.in_use)
    {
	if (XtOwnSelection(myShell, cbd->sel_atom,
	       XtLastTimestampProcessed(XtDisplay(myShell)),
	       clip_x11_convert_selection_cb, clip_x11_lose_ownership_cb,
	       clip_x11_notify_cb) == False)
	    return FAIL;
    }
    else
#endif
    {
	if (!XChangeProperty(XtDisplay(myShell), XtWindow(myShell),
		  cbd->sel_atom, timestamp_atom, 32, PropModeAppend, NULL, 0))
	    return FAIL;
    }
    // Flush is required in a terminal as nothing else is doing it.
    XFlush(XtDisplay(myShell));
    return OK;
}

/*
 * Send the current selection to the clipboard.  Do nothing for X because we
 * will fill in the selection only when requested by another app.
 */
    void
clip_x11_set_selection(Clipboard_T *cbd UNUSED)
{
}

#endif

#if defined(FEAT_XCLIPBOARD) || defined(FEAT_GUI_X11) \
    || defined(FEAT_GUI_GTK) || defined(PROTO)
/*
 * Get the contents of the X CUT_BUFFER0 and put it in "cbd".
 */
    void
yank_cut_buffer0(Display *dpy, Clipboard_T *cbd)
{
    int		nbytes = 0;
    char_u	*buffer = (char_u *)XFetchBuffer(dpy, &nbytes, 0);

    if (nbytes > 0)
    {
	int  done = FALSE;

	// CUT_BUFFER0 is supposed to be always latin1.  Convert to 'enc' when
	// using a multi-byte encoding.  Conversion between two 8-bit
	// character sets usually fails and the text might actually be in
	// 'enc' anyway.
	if (has_mbyte)
	{
	    char_u	*conv_buf;
	    vimconv_T	vc;

	    vc.vc_type = CONV_NONE;
	    if (convert_setup(&vc, (char_u *)"latin1", p_enc) == OK)
	    {
		conv_buf = string_convert(&vc, buffer, &nbytes);
		if (conv_buf != NULL)
		{
		    clip_yank_selection(MCHAR, conv_buf, (long)nbytes, cbd);
		    vim_free(conv_buf);
		    done = TRUE;
		}
		convert_setup(&vc, NULL, NULL);
	    }
	}
	if (!done)  // use the text without conversion
	    clip_yank_selection(MCHAR, buffer, (long)nbytes, cbd);
	XFree((void *)buffer);
	if (p_verbose > 0)
	{
	    verbose_enter();
	    verb_msg(_("Used CUT_BUFFER0 instead of empty selection"));
	    verbose_leave();
	}
    }
}
#endif

/*
 * SELECTION / PRIMARY ('*')
 *
 * Text selection stuff that uses the GUI selection register '*'.  When using a
 * GUI this may be text from another window, otherwise it is the last text we
 * had highlighted with VIsual mode.  With mouse support, clicking the middle
 * button performs the paste, otherwise you will need to do <"*p>. "
 * If not under X, it is synonymous with the clipboard register '+'.
 *
 * X CLIPBOARD ('+')
 *
 * Text selection stuff that uses the GUI clipboard register '+'.
 * Under X, this matches the standard cut/paste buffer CLIPBOARD selection.
 * It will be used for unnamed cut/pasting is 'clipboard' contains "unnamed",
 * otherwise you will need to do <"+p>. "
 * If not under X, it is synonymous with the selection register '*'.
 */

/*
 * Routine to export any final X selection we had to the environment
 * so that the text is still available after Vim has exited. X selections
 * only exist while the owning application exists, so we write to the
 * permanent (while X runs) store CUT_BUFFER0.
 * Dump the CLIPBOARD selection if we own it (it's logically the more
 * 'permanent' of the two), otherwise the PRIMARY one.
 * For now, use a hard-coded sanity limit of 1Mb of data.
 */
#if (defined(FEAT_X11) && defined(FEAT_CLIPBOARD)) || defined(PROTO)
    void
x11_export_final_selection(void)
{
    Display	*dpy;
    char_u	*str = NULL;
    long_u	len = 0;
    int		motion_type = -1;

# ifdef FEAT_GUI
    if (gui.in_use)
	dpy = X_DISPLAY;
    else
# endif
# ifdef FEAT_XCLIPBOARD
	dpy = xterm_dpy;
# else
	return;
# endif

    // Get selection to export
    if (clip_plus.owned)
	motion_type = clip_convert_selection(&str, &len, &clip_plus);
    else if (clip_star.owned)
	motion_type = clip_convert_selection(&str, &len, &clip_star);

    // Check it's OK
    if (dpy != NULL && str != NULL && motion_type >= 0
					       && len < 1024*1024 && len > 0)
    {
	int ok = TRUE;

	// The CUT_BUFFER0 is supposed to always contain latin1.  Convert from
	// 'enc' when it is a multi-byte encoding.  When 'enc' is an 8-bit
	// encoding conversion usually doesn't work, so keep the text as-is.
	if (has_mbyte)
	{
	    vimconv_T	vc;

	    vc.vc_type = CONV_NONE;
	    if (convert_setup(&vc, p_enc, (char_u *)"latin1") == OK)
	    {
		int	intlen = len;
		char_u	*conv_str;

		vc.vc_fail = TRUE;
		conv_str = string_convert(&vc, str, &intlen);
		len = intlen;
		if (conv_str != NULL)
		{
		    vim_free(str);
		    str = conv_str;
		}
		else
		{
		    ok = FALSE;
		}
		convert_setup(&vc, NULL, NULL);
	    }
	    else
	    {
		ok = FALSE;
	    }
	}

	// Do not store the string if conversion failed.  Better to use any
	// other selection than garbled text.
	if (ok)
	{
	    XStoreBuffer(dpy, (char *)str, (int)len, 0);
	    XFlush(dpy);
	}
    }

    vim_free(str);
}
#endif

    void
clip_free_selection(Clipboard_T *cbd)
{
    yankreg_T *y_ptr = get_y_current();

    if (cbd == &clip_plus)
	set_y_current(get_y_register(PLUS_REGISTER));
    else
	set_y_current(get_y_register(STAR_REGISTER));
    free_yank_all();
    get_y_current()->y_size = 0;
    set_y_current(y_ptr);
}

/*
 * Get the selected text and put it in register '*' or '+'.
 */
    void
clip_get_selection(Clipboard_T *cbd)
{
    yankreg_T	*old_y_previous, *old_y_current;
    pos_T	old_cursor;
    pos_T	old_visual;
    int		old_visual_mode;
    colnr_T	old_curswant;
    int		old_set_curswant;
    pos_T	old_op_start, old_op_end;
    oparg_T	oa;
    cmdarg_T	ca;

    if (cbd->owned)
    {
	if ((cbd == &clip_plus
		&& get_y_register(PLUS_REGISTER)->y_array != NULL)
		|| (cbd == &clip_star
		    && get_y_register(STAR_REGISTER)->y_array != NULL))
	    return;

	// Avoid triggering autocmds such as TextYankPost.
	block_autocmds();

	// Get the text between clip_star.start & clip_star.end
	old_y_previous = get_y_previous();
	old_y_current = get_y_current();
	old_cursor = curwin->w_cursor;
	old_curswant = curwin->w_curswant;
	old_set_curswant = curwin->w_set_curswant;
	old_op_start = curbuf->b_op_start;
	old_op_end = curbuf->b_op_end;
	old_visual = VIsual;
	old_visual_mode = VIsual_mode;
	clear_oparg(&oa);
	oa.regname = (cbd == &clip_plus ? '+' : '*');
	oa.op_type = OP_YANK;
	CLEAR_FIELD(ca);
	ca.oap = &oa;
	ca.cmdchar = 'y';
	ca.count1 = 1;
	ca.retval = CA_NO_ADJ_OP_END;
	do_pending_operator(&ca, 0, TRUE);

	// restore things
	set_y_previous(old_y_previous);
	set_y_current(old_y_current);
	curwin->w_cursor = old_cursor;
	changed_cline_bef_curs();   // need to update w_virtcol et al
	curwin->w_curswant = old_curswant;
	curwin->w_set_curswant = old_set_curswant;
	curbuf->b_op_start = old_op_start;
	curbuf->b_op_end = old_op_end;
	VIsual = old_visual;
	VIsual_mode = old_visual_mode;

	unblock_autocmds();
    }
    else if (!is_clipboard_needs_update())
    {
	clip_free_selection(cbd);

	// Try to get selected text from another window
	clip_gen_request_selection(cbd);
    }
}

/*
 * Convert from the GUI selection string into the '*'/'+' register.
 */
    void
clip_yank_selection(
    int		type,
    char_u	*str,
    long	len,
    Clipboard_T *cbd)
{
    yankreg_T *y_ptr;

    if (cbd == &clip_plus)
	y_ptr = get_y_register(PLUS_REGISTER);
    else
	y_ptr = get_y_register(STAR_REGISTER);

    clip_free_selection(cbd);

    str_to_reg(y_ptr, type, str, len, -1, FALSE);
}

/*
 * Convert the '*'/'+' register into a GUI selection string returned in *str
 * with length *len.
 * Returns the motion type, or -1 for failure.
 */
    int
clip_convert_selection(char_u **str, long_u *len, Clipboard_T *cbd)
{
    char_u	*p;
    int		lnum;
    int		i, j;
    int_u	eolsize;
    yankreg_T	*y_ptr;

    if (cbd == &clip_plus)
	y_ptr = get_y_register(PLUS_REGISTER);
    else
	y_ptr = get_y_register(STAR_REGISTER);

# ifdef USE_CRNL
    eolsize = 2;
# else
    eolsize = 1;
# endif

    *str = NULL;
    *len = 0;
    if (y_ptr->y_array == NULL)
	return -1;

    for (i = 0; i < y_ptr->y_size; i++)
	*len += (long_u)STRLEN(y_ptr->y_array[i]) + eolsize;

    // Don't want newline character at end of last line if we're in MCHAR mode.
    if (y_ptr->y_type == MCHAR && *len >= eolsize)
	*len -= eolsize;

    p = *str = alloc(*len + 1);	// add one to avoid zero
    if (p == NULL)
	return -1;
    lnum = 0;
    for (i = 0, j = 0; i < (int)*len; i++, j++)
    {
	if (y_ptr->y_array[lnum][j] == '\n')
	    p[i] = NUL;
	else if (y_ptr->y_array[lnum][j] == NUL)
	{
# ifdef USE_CRNL
	    p[i++] = '\r';
# endif
	    p[i] = '\n';
	    lnum++;
	    j = -1;
	}
	else
	    p[i] = y_ptr->y_array[lnum][j];
    }
    return y_ptr->y_type;
}

/*
 * When "regname" is a clipboard register, obtain the selection.  If it's not
 * available return zero, otherwise return "regname".
 */
    int
may_get_selection(int regname)
{
    if (regname == '*')
    {
	if (!clip_star.available)
	    regname = 0;
	else
	    clip_get_selection(&clip_star);
    }
    else if (regname == '+')
    {
	if (!clip_plus.available)
	    regname = 0;
	else
	    clip_get_selection(&clip_plus);
    }
    return regname;
}

/*
 * If we have written to a clipboard register, send the text to the clipboard.
 */
    void
may_set_selection(void)
{
    if ((get_y_current() == get_y_register(STAR_REGISTER))
	    && clip_star.available)
    {
	clip_own_selection(&clip_star);
	clip_gen_set_selection(&clip_star);
    }
    else if ((get_y_current() == get_y_register(PLUS_REGISTER))
	    && clip_plus.available)
    {
	clip_own_selection(&clip_plus);
	clip_gen_set_selection(&clip_plus);
    }
}

/*
 * Adjust the register name pointed to with "rp" for the clipboard being
 * used always and the clipboard being available.
 */
    void
adjust_clip_reg(int *rp)
{
    // If no reg. specified, and "unnamed" or "unnamedplus" is in 'clipboard',
    // use '*' or '+' reg, respectively. "unnamedplus" prevails.
    if (*rp == 0 && (clip_unnamed != 0 || clip_unnamed_saved != 0))
    {
	if (clip_unnamed != 0)
	    *rp = ((clip_unnamed & CLIP_UNNAMED_PLUS) && clip_plus.available)
								  ? '+' : '*';
	else
	    *rp = ((clip_unnamed_saved & CLIP_UNNAMED_PLUS)
					   && clip_plus.available) ? '+' : '*';
    }
    if (!clip_star.available && *rp == '*')
	*rp = 0;
    if (!clip_plus.available && *rp == '+')
	*rp = 0;
}

#endif // FEAT_CLIPBOARD
