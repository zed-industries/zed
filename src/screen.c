/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * screen.c: Lower level code for displaying on the screen.
 *
 * Output to the screen (console, terminal emulator or GUI window) is minimized
 * by remembering what is already on the screen, and only updating the parts
 * that changed.
 *
 * ScreenLines[off]  Contains a copy of the whole screen, as it is currently
 *		     displayed (excluding text written by external commands).
 * ScreenAttrs[off]  Contains the associated attributes.
 * ScreenCols[off]   Contains the virtual columns in the line. -1 means not
 *		     available or before buffer text, MAXCOL means after the
 *		     end of the line.
 *
 * LineOffset[row]   Contains the offset into ScreenLines*[], ScreenAttrs[]
 *		     and ScreenCols[] for each line.
 * LineWraps[row]    Flag for each line whether it wraps to the next line.
 *
 * For double-byte characters, two consecutive bytes in ScreenLines[] can form
 * one character which occupies two display cells.
 * For UTF-8 a multi-byte character is converted to Unicode and stored in
 * ScreenLinesUC[].  ScreenLines[] contains the first byte only.  For an ASCII
 * character without composing chars ScreenLinesUC[] will be 0 and
 * ScreenLinesC[][] is not used.  When the character occupies two display
 * cells the next byte in ScreenLines[] is 0.
 * ScreenLinesC[][] contain up to 'maxcombine' composing characters
 * (drawn on top of the first character).  There is 0 after the last one used.
 * ScreenLines2[] is only used for euc-jp to store the second byte if the
 * first byte is 0x8e (single-width character).
 *
 * The screen_*() functions write to the screen and handle updating
 * ScreenLines[].
 */

#include "vim.h"

/*
 * The attributes that are actually active for writing to the screen.
 */
static int	screen_attr = 0;

static void screen_char_2(unsigned off, int row, int col);
static int screenclear2(int doclear);
static void lineclear(unsigned off, int width, int attr);
static void lineinvalid(unsigned off, int width);
static int win_do_lines(win_T *wp, int row, int line_count, int mayclear, int del, int clear_attr);
static void win_rest_invalid(win_T *wp);
static void msg_pos_mode(void);
static void recording_mode(int attr);

// Ugly global: overrule attribute used by screen_char()
static int screen_char_attr = 0;

#if defined(FEAT_CONCEAL) || defined(PROTO)
/*
 * Return TRUE if the cursor line in window "wp" may be concealed, according
 * to the 'concealcursor' option.
 */
    int
conceal_cursor_line(win_T *wp)
{
    int		c;

    if (*wp->w_p_cocu == NUL)
	return FALSE;
    if (get_real_state() & MODE_VISUAL)
	c = 'v';
    else if (State & MODE_INSERT)
	c = 'i';
    else if (State & MODE_NORMAL)
	c = 'n';
    else if (State & MODE_CMDLINE)
	c = 'c';
    else
	return FALSE;
    return vim_strchr(wp->w_p_cocu, c) != NULL;
}

/*
 * Check if the cursor line needs to be redrawn because of 'concealcursor'.
 * To be called after changing the state, "was_concealed" is the value of
 * "conceal_cursor_line()" before the change.
 * "
 */
    void
conceal_check_cursor_line(int was_concealed)
{
    if (curwin->w_p_cole <= 0 || conceal_cursor_line(curwin) == was_concealed)
	return;

    int wcol = curwin->w_wcol;

    need_cursor_line_redraw = TRUE;
    // Need to recompute cursor column, e.g., when starting Visual mode
    // without concealing.
    curs_columns(TRUE);

    // When concealing now w_wcol will be computed wrong, keep the previous
    // value, it will be updated in win_line().
    if (!was_concealed)
	curwin->w_wcol = wcol;
}
#endif

/*
 * Get 'wincolor' attribute for window "wp".  If not set and "wp" is a popup
 * window then get the "Pmenu" highlight attribute.
 */
    int
get_wcr_attr(win_T *wp)
{
    int wcr_attr = 0;

    if (*wp->w_p_wcr != NUL)
	wcr_attr = syn_name2attr(wp->w_p_wcr);
#ifdef FEAT_PROP_POPUP
    else if (WIN_IS_POPUP(wp))
    {
	if (wp->w_popup_flags & POPF_INFO)
	    wcr_attr = HL_ATTR(HLF_PSI);    // PmenuSel
	else
	    wcr_attr = HL_ATTR(HLF_PNI);    // Pmenu
    }
#endif
    return wcr_attr;
}

/*
 * Call screen_fill() with the columns adjusted for 'rightleft' if needed.
 * Return the new offset.
 */
    static int
screen_fill_end(
	win_T *wp,
	int	c1,
	int	c2,
	int	off,
	int	width,
	int	row,
	int	endrow,
	int	attr)
{
    int	    nn = off + width;

    if (nn > wp->w_width)
	nn = wp->w_width;
#ifdef FEAT_RIGHTLEFT
    if (wp->w_p_rl)
    {
	screen_fill(W_WINROW(wp) + row, W_WINROW(wp) + endrow,
		W_ENDCOL(wp) - nn, (int)W_ENDCOL(wp) - off,
		c1, c2, attr);
    }
    else
#endif
	screen_fill(W_WINROW(wp) + row, W_WINROW(wp) + endrow,
		wp->w_wincol + off, (int)wp->w_wincol + nn,
		c1, c2, attr);
    return nn;
}

/*
 * Clear lines near the end the window and mark the unused lines with "c1".
 * use "c2" as the filler character.
 * When "draw_margin" is TRUE then draw the sign, fold and number columns.
 */
    void
win_draw_end(
    win_T	*wp,
    int		c1,
    int		c2,
    int		draw_margin,
    int		row,
    int		endrow,
    hlf_T	hl)
{
    int		n = 0;
    int		attr = HL_ATTR(hl);
    int		wcr_attr = get_wcr_attr(wp);

    attr = hl_combine_attr(wcr_attr, attr);

    if (draw_margin)
    {
#ifdef FEAT_FOLDING
	int	fdc = compute_foldcolumn(wp, 0);

	if (fdc > 0)
	    // draw the fold column
	    n = screen_fill_end(wp, ' ', ' ', n, fdc,
		      row, endrow, hl_combine_attr(wcr_attr, HL_ATTR(HLF_FC)));
#endif
#ifdef FEAT_SIGNS
	if (signcolumn_on(wp))
	    // draw the sign column
	    n = screen_fill_end(wp, ' ', ' ', n, 2,
		      row, endrow, hl_combine_attr(wcr_attr, HL_ATTR(HLF_SC)));
#endif
	if ((wp->w_p_nu || wp->w_p_rnu)
				  && vim_strchr(p_cpo, CPO_NUMCOL) == NULL)
	    // draw the number column
	    n = screen_fill_end(wp, ' ', ' ', n, number_width(wp) + 1,
		       row, endrow, hl_combine_attr(wcr_attr, HL_ATTR(HLF_N)));
    }

#ifdef FEAT_RIGHTLEFT
    if (wp->w_p_rl)
    {
	screen_fill(W_WINROW(wp) + row, W_WINROW(wp) + endrow,
		wp->w_wincol, W_ENDCOL(wp) - 1 - n,
		c2, c2, attr);
	screen_fill(W_WINROW(wp) + row, W_WINROW(wp) + endrow,
		W_ENDCOL(wp) - 1 - n, W_ENDCOL(wp) - n,
		c1, c2, attr);
    }
    else
#endif
    {
	screen_fill(W_WINROW(wp) + row, W_WINROW(wp) + endrow,
		wp->w_wincol + n, (int)W_ENDCOL(wp),
		c1, c2, attr);
    }

    set_empty_rows(wp, row);
}

#if defined(FEAT_FOLDING) || defined(PROTO)
/*
 * Compute the width of the foldcolumn.  Based on 'foldcolumn' and how much
 * space is available for window "wp", minus "col".
 */
    int
compute_foldcolumn(win_T *wp, int col)
{
    int fdc = wp->w_p_fdc;
    int wmw = wp == curwin && p_wmw == 0 ? 1 : p_wmw;
    int wwidth = wp->w_width;

    if (fdc > wwidth - (col + wmw))
	fdc = wwidth - (col + wmw);
    return fdc;
}

/*
 * Fill the foldcolumn at "p" for window "wp".
 * Only to be called when 'foldcolumn' > 0.
 * Returns the number of bytes stored in 'p'. When non-multibyte characters are
 * used for the fold column markers, this is equal to 'fdc' setting. Otherwise,
 * this will be greater than 'fdc'.
 */
    size_t
fill_foldcolumn(
    char_u	*p,
    win_T	*wp,
    int		closed,		// TRUE of FALSE
    linenr_T	lnum)		// current line number
{
    int		i = 0;
    int		level;
    int		first_level;
    int		empty;
    int		fdc = compute_foldcolumn(wp, 0);
    size_t	byte_counter = 0;
    int		symbol = 0;
    int		len = 0;

    // Init to all spaces.
    vim_memset(p, ' ', MAX_MCO * fdc + 1);

    level = win_foldinfo.fi_level;
    empty = (fdc == 1) ? 0 : 1;

    // If the column is too narrow, we start at the lowest level that
    // fits and use numbers to indicate the depth.
    first_level = level - fdc - closed + 1 + empty;
    if (first_level < 1)
	first_level = 1;

    for (i = 0; i < MIN(fdc, level); i++)
    {
	if (win_foldinfo.fi_lnum == lnum
		&& first_level + i >= win_foldinfo.fi_low_level)
	    symbol = wp->w_fill_chars.foldopen;
	else if (first_level == 1)
	    symbol = wp->w_fill_chars.foldsep;
	else if (first_level + i <= 9)
	    symbol = '0' + first_level + i;
	else
	    symbol = '>';

	len = utf_char2bytes(symbol, &p[byte_counter]);
	byte_counter += len;
	if (first_level + i >= level)
	{
	    i++;
	    break;
	}
    }

    if (closed)
    {
	if (symbol != 0)
	{
	    // rollback length and the character
	    byte_counter -= len;
	    if (len > 1)
		// for a multibyte character, erase all the bytes
		vim_memset(p + byte_counter, ' ', len);
	}
	symbol = wp->w_fill_chars.foldclosed;
	len = utf_char2bytes(symbol, &p[byte_counter]);
	byte_counter += len;
    }

    return MAX(byte_counter + (fdc - i), (size_t)fdc);
}
#endif // FEAT_FOLDING

/*
 * Return if the composing characters at "off_from" and "off_to" differ.
 * Only to be used when ScreenLinesUC[off_from] != 0.
 */
    static int
comp_char_differs(int off_from, int off_to)
{
    int	    i;

    for (i = 0; i < Screen_mco; ++i)
    {
	if (ScreenLinesC[i][off_from] != ScreenLinesC[i][off_to])
	    return TRUE;
	if (ScreenLinesC[i][off_from] == 0)
	    break;
    }
    return FALSE;
}

/*
 * Check whether the given character needs redrawing:
 * - the (first byte of the) character is different
 * - the attributes are different
 * - the character is multi-byte and the next byte is different
 * - the character is two cells wide and the second cell differs.
 */
    static int
char_needs_redraw(int off_from, int off_to, int cols)
{
    if (cols > 0
	    && ((ScreenLines[off_from] != ScreenLines[off_to]
		    || ScreenAttrs[off_from] != ScreenAttrs[off_to])
		|| (enc_dbcs != 0
		    && MB_BYTE2LEN(ScreenLines[off_from]) > 1
		    && (enc_dbcs == DBCS_JPNU && ScreenLines[off_from] == 0x8e
			? ScreenLines2[off_from] != ScreenLines2[off_to]
			: (cols > 1 && ScreenLines[off_from + 1]
						 != ScreenLines[off_to + 1])))
		|| (enc_utf8
		    && (ScreenLinesUC[off_from] != ScreenLinesUC[off_to]
			|| (ScreenLinesUC[off_from] != 0
			    && comp_char_differs(off_from, off_to))
			|| ((*mb_off2cells)(off_from, off_from + cols) > 1
			    && ScreenLines[off_from + 1]
						!= ScreenLines[off_to + 1])))))
	return TRUE;
    return FALSE;
}

#if defined(FEAT_TERMINAL) || defined(PROTO)
/*
 * Return the index in ScreenLines[] for the current screen line.
 */
    int
screen_get_current_line_off(void)
{
    return (int)(current_ScreenLine - ScreenLines);
}
#endif

#ifdef FEAT_PROP_POPUP
/*
 * Return TRUE if this position has a higher level popup or this cell is
 * transparent in the current popup.
 */
    static int
blocked_by_popup(int row, int col)
{
    int off;

    if (!popup_visible)
	return FALSE;
    off = row * screen_Columns + col;
    return popup_mask[off] > screen_zindex || popup_transparent[off];
}
#endif

/*
 * Reset the highlighting.  Used before clearing the screen.
 */
    void
reset_screen_attr(void)
{
#ifdef FEAT_GUI
    if (gui.in_use)
	// Use a code that will reset gui.highlight_mask in
	// gui_stop_highlight().
	screen_attr = HL_ALL + 1;
    else
#endif
	// Use attributes that is very unlikely to appear in text.
	screen_attr = HL_BOLD | HL_UNDERLINE | HL_INVERSE | HL_STRIKETHROUGH;
}

/*
 * Return TRUE if the character at "row" / "col" is under the popup menu and it
 * will be redrawn soon or it is under another popup.
 */
    static int
skip_for_popup(int row, int col)
{
    // Popup windows with zindex higher than POPUPMENU_ZINDEX go on top.
    if (pum_under_menu(row, col, TRUE)
#ifdef FEAT_PROP_POPUP
	    && screen_zindex <= POPUPMENU_ZINDEX
#endif
	    )
	return TRUE;
#ifdef FEAT_PROP_POPUP
    if (blocked_by_popup(row, col))
	return TRUE;
#endif
    return FALSE;
}

/*
 * Move one "cooked" screen line to the screen, but only the characters that
 * have actually changed.  Handle insert/delete character.
 * "coloff" gives the first column on the screen for this line.
 * "endcol" gives the columns where valid characters are.
 * "clear_width" is the width of the window.  It's > 0 if the rest of the line
 * needs to be cleared, negative otherwise.
 * "flags" can have bits:
 * SLF_POPUP	    popup window
 * SLF_RIGHTLEFT    rightleft window:
 *    When TRUE and "clear_width" > 0, clear columns 0 to "endcol"
 *    When FALSE and "clear_width" > 0, clear columns "endcol" to "clear_width"
 */
    void
screen_line(
	win_T	*wp,
	int	row,
	int	coloff,
	int	endcol,
	int	clear_width,
	int	flags UNUSED)
{
    unsigned	    off_from;
    unsigned	    off_to;
    unsigned	    max_off_from;
    unsigned	    max_off_to;
    int		    col = 0;
    int		    hl;
    int		    force = FALSE;	// force update rest of the line
    int		    redraw_this		// bool: does character need redraw?
#ifdef FEAT_GUI
				= TRUE	// For GUI when while-loop empty
#endif
				;
    int		    redraw_next;	// redraw_this for next character
#ifdef FEAT_GUI_MSWIN
    int		    changed_this;	// TRUE if character changed
    int		    changed_next;	// TRUE if next character changed
#endif
    int		    clear_next = FALSE;
    int		    char_cells;		// 1: normal char
					// 2: occupies two display cells

    // Check for illegal row and col, just in case.
    if (row >= Rows)
	row = Rows - 1;
    if (endcol > Columns)
	endcol = Columns;

# ifdef FEAT_CLIPBOARD
    clip_may_clear_selection(row, row);
# endif

    off_from = (unsigned)(current_ScreenLine - ScreenLines);
    off_to = LineOffset[row] + coloff;
    max_off_from = off_from + screen_Columns;
    max_off_to = LineOffset[row] + screen_Columns;

#ifdef FEAT_RIGHTLEFT
    if (flags & SLF_RIGHTLEFT)
    {
	// Clear rest first, because it's left of the text.
	if (clear_width > 0)
	{
	    while (col <= endcol && ScreenLines[off_to] == ' '
		    && ScreenAttrs[off_to] == 0
				  && (!enc_utf8 || ScreenLinesUC[off_to] == 0))
	    {
		++off_to;
		++col;
	    }
	    if (col <= endcol)
		screen_fill(row, row + 1, col + coloff,
					    endcol + coloff + 1, ' ', ' ', 0);
	}
	col = endcol + 1;
	off_to = LineOffset[row] + col + coloff;
	off_from += col;
	endcol = (clear_width > 0 ? clear_width : -clear_width);
    }
#endif // FEAT_RIGHTLEFT

#ifdef FEAT_PROP_POPUP
    // First char of a popup window may go on top of the right half of a
    // double-wide character. Clear the left half to avoid it getting the popup
    // window background color.
    if (coloff > 0 && enc_utf8
		   && ScreenLines[off_to] == 0
		   && ScreenLinesUC[off_to - 1] != 0
		   && (*mb_char2cells)(ScreenLinesUC[off_to - 1]) > 1)
    {
	ScreenLines[off_to - 1] = ' ';
	ScreenLinesUC[off_to - 1] = 0;
	screen_char(off_to - 1, row, col + coloff - 1);
    }
#endif

    redraw_next = char_needs_redraw(off_from, off_to, endcol - col);
#ifdef FEAT_GUI_MSWIN
    changed_next = redraw_next;
#endif

    while (col < endcol)
    {
	if (has_mbyte && (col + 1 < endcol))
	    char_cells = (*mb_off2cells)(off_from, max_off_from);
	else
	    char_cells = 1;

	redraw_this = redraw_next;
	redraw_next = force || char_needs_redraw(off_from + char_cells,
			      off_to + char_cells, endcol - col - char_cells);

#ifdef FEAT_GUI
# ifdef FEAT_GUI_MSWIN
	changed_this = changed_next;
	changed_next = redraw_next;
# endif
	// If the next character was bold, then redraw the current character to
	// remove any pixels that might have spilt over into us.  This only
	// happens in the GUI.
	// With MS-Windows antialiasing may also cause pixels to spill over
	// from a previous character, no matter attributes, always redraw if a
	// character changed.
	if (redraw_next && gui.in_use)
	{
# ifndef FEAT_GUI_MSWIN
	    hl = ScreenAttrs[off_to + char_cells];
	    if (hl > HL_ALL)
		hl = syn_attr2attr(hl);
	    if (hl & HL_BOLD)
# endif
		redraw_this = TRUE;
	}
#endif
	// Do not redraw if under the popup menu.
	if (redraw_this && skip_for_popup(row, col + coloff))
	    redraw_this = FALSE;

	if (redraw_this)
	{
	    /*
	     * Special handling when 'xs' termcap flag set (hpterm):
	     * Attributes for characters are stored at the position where the
	     * cursor is when writing the highlighting code.  The
	     * start-highlighting code must be written with the cursor on the
	     * first highlighted character.  The stop-highlighting code must
	     * be written with the cursor just after the last highlighted
	     * character.
	     * Overwriting a character doesn't remove its highlighting.  Need
	     * to clear the rest of the line, and force redrawing it
	     * completely.
	     */
	    if (       p_wiv
		    && !force
#ifdef FEAT_GUI
		    && !gui.in_use
#endif
		    && ScreenAttrs[off_to] != 0
		    && ScreenAttrs[off_from] != ScreenAttrs[off_to])
	    {
		/*
		 * Need to remove highlighting attributes here.
		 */
		windgoto(row, col + coloff);
		out_str(T_CE);		// clear rest of this screen line
		screen_start();		// don't know where cursor is now
		force = TRUE;		// force redraw of rest of the line
		redraw_next = TRUE;	// or else next char would miss out

		/*
		 * If the previous character was highlighted, need to stop
		 * highlighting at this character.
		 */
		if (col + coloff > 0 && ScreenAttrs[off_to - 1] != 0)
		{
		    screen_attr = ScreenAttrs[off_to - 1];
		    term_windgoto(row, col + coloff);
		    screen_stop_highlight();
		}
		else
		    screen_attr = 0;	    // highlighting has stopped
	    }
	    if (enc_dbcs != 0)
	    {
		// Check if overwriting a double-byte with a single-byte or
		// the other way around requires another character to be
		// redrawn.  For UTF-8 this isn't needed, because comparing
		// ScreenLinesUC[] is sufficient.
		if (char_cells == 1
			&& col + 1 < endcol
			&& (*mb_off2cells)(off_to, max_off_to) > 1)
		{
		    // Writing a single-cell character over a double-cell
		    // character: need to redraw the next cell.
		    ScreenLines[off_to + 1] = 0;
		    redraw_next = TRUE;
		}
		else if (char_cells == 2
			&& col + 2 < endcol
			&& (*mb_off2cells)(off_to, max_off_to) == 1
			&& (*mb_off2cells)(off_to + 1, max_off_to) > 1)
		{
		    // Writing the second half of a double-cell character over
		    // a double-cell character: need to redraw the second
		    // cell.
		    ScreenLines[off_to + 2] = 0;
		    redraw_next = TRUE;
		}

		if (enc_dbcs == DBCS_JPNU)
		    ScreenLines2[off_to] = ScreenLines2[off_from];
	    }
	    // When writing a single-width character over a double-width
	    // character and at the end of the redrawn text, need to clear out
	    // the right half of the old character.
	    // Also required when writing the right half of a double-width
	    // char over the left half of an existing one.
	    if (has_mbyte && col + char_cells == endcol
		    && ((char_cells == 1
			    && (*mb_off2cells)(off_to, max_off_to) > 1)
			|| (char_cells == 2
			    && (*mb_off2cells)(off_to, max_off_to) == 1
			    && (*mb_off2cells)(off_to + 1, max_off_to) > 1)))
		clear_next = TRUE;

	    ScreenLines[off_to] = ScreenLines[off_from];
	    if (enc_utf8)
	    {
		ScreenLinesUC[off_to] = ScreenLinesUC[off_from];
		if (ScreenLinesUC[off_from] != 0)
		{
		    int	    i;

		    for (i = 0; i < Screen_mco; ++i)
			ScreenLinesC[i][off_to] = ScreenLinesC[i][off_from];
		}
	    }
	    if (char_cells == 2)
		ScreenLines[off_to + 1] = ScreenLines[off_from + 1];

#if defined(FEAT_GUI) || defined(UNIX)
	    // The bold trick makes a single column of pixels appear in the
	    // next character.  When a bold character is removed, the next
	    // character should be redrawn too.  This happens for our own GUI
	    // and for some xterms.
	    if (
# ifdef FEAT_GUI
		    gui.in_use
# endif
# if defined(FEAT_GUI) && defined(UNIX)
		    ||
# endif
# ifdef UNIX
		    term_is_xterm
# endif
		    )
	    {
		hl = ScreenAttrs[off_to];
		if (hl > HL_ALL)
		    hl = syn_attr2attr(hl);
		if (hl & HL_BOLD)
		    redraw_next = TRUE;
	    }
#endif
#ifdef FEAT_GUI_MSWIN
	    // MS-Windows antialiasing may spill over to the next character,
	    // redraw that one if this one changed, no matter attributes.
	    if (gui.in_use && changed_this)
		redraw_next = TRUE;
#endif
	    ScreenAttrs[off_to] = ScreenAttrs[off_from];

	    // For simplicity set the attributes of second half of a
	    // double-wide character equal to the first half.
	    if (char_cells == 2)
		ScreenAttrs[off_to + 1] = ScreenAttrs[off_from];

	    if (enc_dbcs != 0 && char_cells == 2)
		screen_char_2(off_to, row, col + coloff);
	    else
		screen_char(off_to, row, col + coloff);
	}
	else if (  p_wiv
#ifdef FEAT_GUI
		&& !gui.in_use
#endif
		&& col + coloff > 0)
	{
	    if (ScreenAttrs[off_to] == ScreenAttrs[off_to - 1])
	    {
		/*
		 * Don't output stop-highlight when moving the cursor, it will
		 * stop the highlighting when it should continue.
		 */
		screen_attr = 0;
	    }
	    else if (screen_attr != 0)
		screen_stop_highlight();
	}

	ScreenCols[off_to] = ScreenCols[off_from];
	if (char_cells == 2)
	    ScreenCols[off_to + 1] = ScreenCols[off_from + 1];

	off_to += char_cells;
	off_from += char_cells;
	col += char_cells;
    }

    if (clear_next && !skip_for_popup(row, col + coloff))
    {
	// Clear the second half of a double-wide character of which the left
	// half was overwritten with a single-wide character.
	ScreenLines[off_to] = ' ';
	if (enc_utf8)
	    ScreenLinesUC[off_to] = 0;
	screen_char(off_to, row, col + coloff);
    }

    if (clear_width > 0
#ifdef FEAT_RIGHTLEFT
		    && !(flags & SLF_RIGHTLEFT)
#endif
				   )
    {
#ifdef FEAT_GUI
	int startCol = col;
#endif

	// blank out the rest of the line
	while (col < clear_width && ScreenLines[off_to] == ' '
						  && ScreenAttrs[off_to] == 0
				  && (!enc_utf8 || ScreenLinesUC[off_to] == 0))
	{
	    ScreenCols[off_to] = MAXCOL;
	    ++off_to;
	    ++col;
	}
	if (col < clear_width)
	{
#ifdef FEAT_GUI
	    /*
	     * In the GUI, clearing the rest of the line may leave pixels
	     * behind if the first character cleared was bold.  Some bold
	     * fonts spill over the left.  In this case we redraw the previous
	     * character too.  If we didn't skip any blanks above, then we
	     * only redraw if the character wasn't already redrawn anyway.
	     */
	    if (gui.in_use && (col > startCol || !redraw_this))
	    {
		hl = ScreenAttrs[off_to];
		if (hl > HL_ALL || (hl & HL_BOLD))
		{
		    int prev_cells = 1;

		    if (enc_utf8)
			// for utf-8, ScreenLines[char_offset + 1] == 0 means
			// that its width is 2.
			prev_cells = ScreenLines[off_to - 1] == 0 ? 2 : 1;
		    else if (enc_dbcs != 0)
		    {
			// find previous character by counting from first
			// column and get its width.
			unsigned off = LineOffset[row];
			unsigned max_off = LineOffset[row] + screen_Columns;

			while (off < off_to)
			{
			    prev_cells = (*mb_off2cells)(off, max_off);
			    off += prev_cells;
			}
		    }

		    if (!skip_for_popup(row, col + coloff - prev_cells))
		    {
			if (enc_dbcs != 0 && prev_cells > 1)
			    screen_char_2(off_to - prev_cells, row,
						   col + coloff - prev_cells);
			else
			    screen_char(off_to - prev_cells, row,
						   col + coloff - prev_cells);
		    }
		}
	    }
#endif
	    screen_fill(row, row + 1, col + coloff, clear_width + coloff,
								 ' ', ' ', 0);
	    while (col < clear_width)
	    {
		ScreenCols[off_to++] = MAXCOL;
		++col;
	    }
	}
    }

    if (clear_width > 0
#ifdef FEAT_PROP_POPUP
	    && !(flags & SLF_POPUP)  // no separator for popup window
#endif
	    )
    {
	// For a window that has a right neighbor, draw the separator char
	// right of the window contents.  But not on top of a popup window.
	if (coloff + col < Columns)
	{
	    if (!skip_for_popup(row, col + coloff))
	    {
		int c;

		c = fillchar_vsep(&hl, wp);
		if (ScreenLines[off_to] != (schar_T)c
			|| (enc_utf8 && (int)ScreenLinesUC[off_to]
							!= (c >= 0x80 ? c : 0))
			|| ScreenAttrs[off_to] != hl)
		{
		    ScreenLines[off_to] = c;
		    ScreenAttrs[off_to] = hl;
		    if (enc_utf8)
		    {
			if (c >= 0x80)
			{
			    ScreenLinesUC[off_to] = c;
			    ScreenLinesC[0][off_to] = 0;
			}
			else
			    ScreenLinesUC[off_to] = 0;
		    }
		    screen_char(off_to, row, col + coloff);
		}
	    }
	}
	else
	    LineWraps[row] = FALSE;
    }
}

#if defined(FEAT_RIGHTLEFT) || defined(PROTO)
/*
 * Mirror text "str" for right-left displaying.
 * Only works for single-byte characters (e.g., numbers).
 */
    void
rl_mirror(char_u *str)
{
    char_u	*p1, *p2;
    int		t;

    for (p1 = str, p2 = str + STRLEN(str) - 1; p1 < p2; ++p1, --p2)
    {
	t = *p1;
	*p1 = *p2;
	*p2 = t;
    }
}
#endif

/*
 * Draw the verticap separator right of window "wp" starting with line "row".
 */
    void
draw_vsep_win(win_T *wp, int row)
{
    int		hl;
    int		c;

    if (!wp->w_vsep_width)
	return;

    // draw the vertical separator right of this window
    c = fillchar_vsep(&hl, wp);
    screen_fill(W_WINROW(wp) + row, W_WINROW(wp) + wp->w_height,
	    W_ENDCOL(wp), W_ENDCOL(wp) + 1,
	    c, ' ', hl);
}

/*
 * Return TRUE if the status line of window "wp" is connected to the status
 * line of the window right of it.  If not, then it's a vertical separator.
 * Only call if (wp->w_vsep_width != 0).
 */
    int
stl_connected(win_T *wp)
{
    frame_T	*fr;

    fr = wp->w_frame;
    while (fr->fr_parent != NULL)
    {
	if (fr->fr_parent->fr_layout == FR_COL)
	{
	    if (fr->fr_next != NULL)
		break;
	}
	else
	{
	    if (fr->fr_next != NULL)
		return TRUE;
	}
	fr = fr->fr_parent;
    }
    return FALSE;
}


/*
 * Get the value to show for the language mappings, active 'keymap'.
 */
    int
get_keymap_str(
    win_T	*wp,
    char_u	*fmt,	    // format string containing one %s item
    char_u	*buf,	    // buffer for the result
    int		len)	    // length of buffer
{
    char_u	*p;

    if (wp->w_buffer->b_p_iminsert != B_IMODE_LMAP)
	return FALSE;

#ifdef FEAT_EVAL
    buf_T	*old_curbuf = curbuf;
    win_T	*old_curwin = curwin;
    char_u	*s;

    curbuf = wp->w_buffer;
    curwin = wp;
    STRCPY(buf, "b:keymap_name");	// must be writable
    ++emsg_skip;
    s = p = eval_to_string(buf, FALSE, FALSE);
    --emsg_skip;
    curbuf = old_curbuf;
    curwin = old_curwin;
    if (p == NULL || *p == NUL)
#endif
    {
#ifdef FEAT_KEYMAP
	if (wp->w_buffer->b_kmap_state & KEYMAP_LOADED)
	    p = wp->w_buffer->b_p_keymap;
	else
#endif
	    p = (char_u *)"lang";
    }
    if (vim_snprintf((char *)buf, len, (char *)fmt, p) > len - 1)
	buf[0] = NUL;
#ifdef FEAT_EVAL
    vim_free(s);
#endif
    return buf[0] != NUL;
}

#if defined(FEAT_STL_OPT) || defined(PROTO)
/*
 * Redraw the status line or ruler of window "wp".
 * When "wp" is NULL redraw the tab pages line from 'tabline'.
 */
    void
win_redr_custom(
    win_T	*wp,
    int		draw_ruler)	// TRUE or FALSE
{
    static int	entered = FALSE;
    int		attr;
    int		curattr;
    int		row;
    int		col = 0;
    int		maxwidth;
    int		width;
    int		n;
    int		len;
    int		fillchar;
    char_u	buf[MAXPATHL];
    char_u	*stl;
    char_u	*p;
    char_u	*opt_name;
    int		opt_scope = 0;
    stl_hlrec_T *hltab;
    stl_hlrec_T *tabtab;
    win_T	*ewp;
    int		p_crb_save;

    // There is a tiny chance that this gets called recursively: When
    // redrawing a status line triggers redrawing the ruler or tabline.
    // Avoid trouble by not allowing recursion.
    if (entered)
	return;
    entered = TRUE;

    // setup environment for the task at hand
    if (wp == NULL)
    {
	// Use 'tabline'.  Always at the first line of the screen.
	stl = p_tal;
	row = 0;
	fillchar = ' ';
	attr = HL_ATTR(HLF_TPF);
	maxwidth = Columns;
	opt_name = (char_u *)"tabline";
    }
    else
    {
	row = statusline_row(wp);
	fillchar = fillchar_status(&attr, wp);
	int in_status_line = wp->w_status_height != 0;
	maxwidth = in_status_line ? wp->w_width : Columns;

	if (draw_ruler)
	{
	    stl = p_ruf;
	    opt_name = (char_u *)"rulerformat";
	    // advance past any leading group spec - implicit in ru_col
	    if (*stl == '%')
	    {
		if (*++stl == '-')
		    stl++;
		if (atoi((char *)stl))
		    while (VIM_ISDIGIT(*stl))
			stl++;
		if (*stl++ != '(')
		    stl = p_ruf;
	    }
	    col = ru_col - (Columns - maxwidth);
	    if (col < (maxwidth + 1) / 2)
		col = (maxwidth + 1) / 2;
	    maxwidth -= col;
	    if (!in_status_line)
	    {
		row = Rows - 1;
		--maxwidth;	// writing in last column may cause scrolling
		fillchar = ' ';
		attr = 0;
	    }
	}
	else
	{
	    opt_name = (char_u *)"statusline";
	    if (*wp->w_p_stl != NUL)
	    {
		stl = wp->w_p_stl;
		opt_scope = OPT_LOCAL;
	    }
	    else
		stl = p_stl;
	}

	if (in_status_line)
	    col += wp->w_wincol;
    }

    if (maxwidth <= 0)
	goto theend;

    // Temporarily reset 'cursorbind', we don't want a side effect from moving
    // the cursor away and back.
    ewp = wp == NULL ? curwin : wp;
    p_crb_save = ewp->w_p_crb;
    ewp->w_p_crb = FALSE;

    // Make a copy, because the statusline may include a function call that
    // might change the option value and free the memory.
    stl = vim_strsave(stl);
    width = build_stl_str_hl(ewp, buf, sizeof(buf),
				stl, opt_name, opt_scope,
				fillchar, maxwidth, &hltab, &tabtab);
    vim_free(stl);
    ewp->w_p_crb = p_crb_save;

    // Make all characters printable.
    p = transstr(buf);
    if (p != NULL)
    {
	vim_strncpy(buf, p, sizeof(buf) - 1);
	vim_free(p);
    }

    // fill up with "fillchar"
    len = (int)STRLEN(buf);
    while (width < maxwidth && len < (int)sizeof(buf) - 1)
    {
	len += (*mb_char2bytes)(fillchar, buf + len);
	++width;
    }
    buf[len] = NUL;

    /*
     * Draw each snippet with the specified highlighting.
     */
    curattr = attr;
    p = buf;
    for (n = 0; hltab[n].start != NULL; n++)
    {
	len = (int)(hltab[n].start - p);
	screen_puts_len(p, len, row, col, curattr);
	col += vim_strnsize(p, len);
	p = hltab[n].start;

	if (hltab[n].userhl == 0)
	    curattr = attr;
	else if (hltab[n].userhl < 0)
	    curattr = syn_id2attr(-hltab[n].userhl);
#ifdef FEAT_TERMINAL
	else if (wp != NULL && wp != curwin && bt_terminal(wp->w_buffer)
						   && wp->w_status_height != 0)
	    curattr = highlight_stltermnc[hltab[n].userhl - 1];
	else if (wp != NULL && bt_terminal(wp->w_buffer)
						   && wp->w_status_height != 0)
	    curattr = highlight_stlterm[hltab[n].userhl - 1];
#endif
	else if (wp != NULL && wp != curwin && wp->w_status_height != 0)
	    curattr = highlight_stlnc[hltab[n].userhl - 1];
	else
	    curattr = highlight_user[hltab[n].userhl - 1];
    }
    screen_puts(p, row, col, curattr);

    if (wp == NULL)
    {
	// Fill the TabPageIdxs[] array for clicking in the tab pagesline.
	col = 0;
	len = 0;
	p = buf;
	fillchar = 0;
	for (n = 0; tabtab[n].start != NULL; n++)
	{
	    len += vim_strnsize(p, (int)(tabtab[n].start - p));
	    while (col < len)
		TabPageIdxs[col++] = fillchar;
	    p = tabtab[n].start;
	    fillchar = tabtab[n].userhl;
	}
	while (col < Columns)
	    TabPageIdxs[col++] = fillchar;
    }

theend:
    entered = FALSE;
}

#endif // FEAT_STL_OPT

/*
 * Output a single character directly to the screen and update ScreenLines.
 */
    void
screen_putchar(int c, int row, int col, int attr)
{
    char_u	buf[MB_MAXBYTES + 1];

    if (has_mbyte)
	buf[(*mb_char2bytes)(c, buf)] = NUL;
    else
    {
	buf[0] = c;
	buf[1] = NUL;
    }
    screen_puts(buf, row, col, attr);
}

/*
 * Get a single character directly from ScreenLines into "bytes", which must
 * have a size of "MB_MAXBYTES + 1".
 * If "attrp" is not NULL, return the character's attribute in "*attrp".
 */
    void
screen_getbytes(int row, int col, char_u *bytes, int *attrp)
{
    unsigned off;

    // safety check
    if (ScreenLines == NULL || row >= screen_Rows || col >= screen_Columns)
	return;

    off = LineOffset[row] + col;
    if (attrp != NULL)
	*attrp = ScreenAttrs[off];
    bytes[0] = ScreenLines[off];
    bytes[1] = NUL;

    if (enc_utf8 && ScreenLinesUC[off] != 0)
	bytes[utfc_char2bytes(off, bytes)] = NUL;
    else if (enc_dbcs == DBCS_JPNU && ScreenLines[off] == 0x8e)
    {
	bytes[0] = ScreenLines[off];
	bytes[1] = ScreenLines2[off];
	bytes[2] = NUL;
    }
    else if (enc_dbcs && MB_BYTE2LEN(bytes[0]) > 1)
    {
	bytes[1] = ScreenLines[off + 1];
	bytes[2] = NUL;
    }
}

/*
 * Return TRUE if composing characters for screen posn "off" differs from
 * composing characters in "u8cc".
 * Only to be used when ScreenLinesUC[off] != 0.
 */
    static int
screen_comp_differs(int off, int *u8cc)
{
    int	    i;

    for (i = 0; i < Screen_mco; ++i)
    {
	if (ScreenLinesC[i][off] != (u8char_T)u8cc[i])
	    return TRUE;
	if (u8cc[i] == 0)
	    break;
    }
    return FALSE;
}

/*
 * Put string '*text' on the screen at position 'row' and 'col', with
 * attributes 'attr', and update ScreenLines[] and ScreenAttrs[].
 * Note: only outputs within one row, message is truncated at screen boundary!
 * Note: if ScreenLines[], row and/or col is invalid, nothing is done.
 */
    void
screen_puts(
    char_u	*text,
    int		row,
    int		col,
    int		attr)
{
    screen_puts_len(text, -1, row, col, attr);
}

/*
 * Like screen_puts(), but output "text[len]".  When "len" is -1 output up to
 * a NUL.
 */
    void
screen_puts_len(
    char_u	*text,
    int		textlen,
    int		row,
    int		col,
    int		attr_arg)
{
    int		attr = attr_arg;
    unsigned	off;
    char_u	*ptr = text;
    int		len = textlen;
    int		c;
    unsigned	max_off;
    int		mbyte_blen = 1;
    int		mbyte_cells = 1;
    int		u8c = 0;
    int		u8cc[MAX_MCO];
    int		clear_next_cell = FALSE;
#ifdef FEAT_ARABIC
    int		prev_c = 0;		// previous Arabic character
    int		pc, nc, nc1;
    int		pcc[MAX_MCO];
#endif
    int		force_redraw_this;
    int		force_redraw_next = FALSE;
    int		need_redraw;

    // Safety check. The check for negative row and column is to fix issue
    // #4102. TODO: find out why row/col could be negative.
    if (ScreenLines == NULL
	    || row >= screen_Rows || row < 0
	    || col >= screen_Columns || col < 0)
	return;
    off = LineOffset[row] + col;

    // When drawing over the right half of a double-wide char clear out the
    // left half.  Only needed in a terminal.
    if (has_mbyte && col > 0 && col < screen_Columns
#ifdef FEAT_GUI
	    && !gui.in_use
#endif
	    && mb_fix_col(col, row) != col)
    {
	if (!skip_for_popup(row, col - 1))
	{
	    ScreenLines[off - 1] = ' ';
	    ScreenAttrs[off - 1] = 0;
	    if (enc_utf8)
	    {
		ScreenLinesUC[off - 1] = 0;
		ScreenLinesC[0][off - 1] = 0;
	    }
	    // redraw the previous cell, make it empty
	    screen_char(off - 1, row, col - 1);
	}
	// force the cell at "col" to be redrawn
	force_redraw_next = TRUE;
    }

    max_off = LineOffset[row] + screen_Columns;
    while (col < screen_Columns
	    && (len < 0 || (int)(ptr - text) < len)
	    && *ptr != NUL)
    {
	c = *ptr;
	// check if this is the first byte of a multibyte
	if (has_mbyte)
	{
	    mbyte_blen = enc_utf8 && len > 0
			     ? utfc_ptr2len_len(ptr, (int)((text + len) - ptr))
			     : (*mb_ptr2len)(ptr);
	    if (enc_dbcs == DBCS_JPNU && c == 0x8e)
		mbyte_cells = 1;
	    else if (enc_dbcs != 0)
		mbyte_cells = mbyte_blen;
	    else	// enc_utf8
	    {
		u8c = len >= 0
		      ? utfc_ptr2char_len(ptr, u8cc, (int)((text + len) - ptr))
		      : utfc_ptr2char(ptr, u8cc);
		mbyte_cells = utf_char2cells(u8c);
#ifdef FEAT_ARABIC
		if (p_arshape && !p_tbidi && ARABIC_CHAR(u8c))
		{
		    // Do Arabic shaping.
		    if (len >= 0 && (int)(ptr - text) + mbyte_blen >= len)
		    {
			// Past end of string to be displayed.
			nc = NUL;
			nc1 = NUL;
		    }
		    else
		    {
			nc = len >= 0
				 ? utfc_ptr2char_len(ptr + mbyte_blen, pcc,
					(int)((text + len) - ptr - mbyte_blen))
				 : utfc_ptr2char(ptr + mbyte_blen, pcc);
			nc1 = pcc[0];
		    }
		    pc = prev_c;
		    prev_c = u8c;
		    u8c = arabic_shape(u8c, &c, &u8cc[0], nc, nc1, pc);
		}
		else
		    prev_c = u8c;
#endif
		if (col + mbyte_cells > screen_Columns)
		{
		    // Only 1 cell left, but character requires 2 cells:
		    // display a '>' in the last column to avoid wrapping.
		    c = '>';
		    mbyte_cells = 1;
		}
	    }
	}

	force_redraw_this = force_redraw_next;
	force_redraw_next = FALSE;

	need_redraw = ScreenLines[off] != c
		|| (mbyte_cells == 2
		    && ScreenLines[off + 1] != (enc_dbcs ? ptr[1] : 0))
		|| (enc_dbcs == DBCS_JPNU
		    && c == 0x8e
		    && ScreenLines2[off] != ptr[1])
		|| (enc_utf8
		    && (ScreenLinesUC[off] !=
				(u8char_T)(c < 0x80 && u8cc[0] == 0 ? 0 : u8c)
			|| (ScreenLinesUC[off] != 0
					  && screen_comp_differs(off, u8cc))))
		|| ScreenAttrs[off] != attr
		|| exmode_active;

	if ((need_redraw || force_redraw_this) && !skip_for_popup(row, col))
	{
#if defined(FEAT_GUI) || defined(UNIX)
	    // The bold trick makes a single row of pixels appear in the next
	    // character.  When a bold character is removed, the next
	    // character should be redrawn too.  This happens for our own GUI
	    // and for some xterms.
	    if (need_redraw && ScreenLines[off] != ' ' && (
# ifdef FEAT_GUI
		    gui.in_use
# endif
# if defined(FEAT_GUI) && defined(UNIX)
		    ||
# endif
# ifdef UNIX
		    term_is_xterm
# endif
		    ))
	    {
		int	n = ScreenAttrs[off];

		if (n > HL_ALL)
		    n = syn_attr2attr(n);
		if (n & HL_BOLD)
		    force_redraw_next = TRUE;
	    }
#endif
	    // When at the end of the text and overwriting a two-cell
	    // character with a one-cell character, need to clear the next
	    // cell.  Also when overwriting the left half of a two-cell char
	    // with the right half of a two-cell char.  Do this only once
	    // (mb_off2cells() may return 2 on the right half).
	    if (clear_next_cell)
		clear_next_cell = FALSE;
	    else if (has_mbyte
		    && (len < 0 ? ptr[mbyte_blen] == NUL
					     : ptr + mbyte_blen >= text + len)
		    && ((mbyte_cells == 1 && (*mb_off2cells)(off, max_off) > 1)
			|| (mbyte_cells == 2
			    && (*mb_off2cells)(off, max_off) == 1
			    && (*mb_off2cells)(off + 1, max_off) > 1)))
		clear_next_cell = TRUE;

	    // Make sure we never leave a second byte of a double-byte behind,
	    // it confuses mb_off2cells().
	    if (enc_dbcs
		    && ((mbyte_cells == 1 && (*mb_off2cells)(off, max_off) > 1)
			|| (mbyte_cells == 2
			    && (*mb_off2cells)(off, max_off) == 1
			    && (*mb_off2cells)(off + 1, max_off) > 1)))
		ScreenLines[off + mbyte_blen] = 0;
	    ScreenLines[off] = c;
	    ScreenAttrs[off] = attr;
	    ScreenCols[off] = -1;
	    if (enc_utf8)
	    {
		if (c < 0x80 && u8cc[0] == 0)
		    ScreenLinesUC[off] = 0;
		else
		{
		    int	    i;

		    ScreenLinesUC[off] = u8c;
		    for (i = 0; i < Screen_mco; ++i)
		    {
			ScreenLinesC[i][off] = u8cc[i];
			if (u8cc[i] == 0)
			    break;
		    }
		}
		if (mbyte_cells == 2)
		{
		    ScreenLines[off + 1] = 0;
		    ScreenAttrs[off + 1] = attr;
		    ScreenCols[off + 1] = -1;
		}
		screen_char(off, row, col);
	    }
	    else if (mbyte_cells == 2)
	    {
		ScreenLines[off + 1] = ptr[1];
		ScreenAttrs[off + 1] = attr;
		ScreenCols[off + 1] = -1;
		screen_char_2(off, row, col);
	    }
	    else if (enc_dbcs == DBCS_JPNU && c == 0x8e)
	    {
		ScreenLines2[off] = ptr[1];
		screen_char(off, row, col);
	    }
	    else
		screen_char(off, row, col);
	}
	if (has_mbyte)
	{
	    off += mbyte_cells;
	    col += mbyte_cells;
	    ptr += mbyte_blen;
	    if (clear_next_cell)
	    {
		// This only happens at the end, display one space next.
		// Keep the attribute from before.
		ptr = (char_u *)" ";
		len = -1;
		attr = ScreenAttrs[off];
	    }
	}
	else
	{
	    ++off;
	    ++col;
	    ++ptr;
	}
    }

    // If we detected the next character needs to be redrawn, but the text
    // doesn't extend up to there, update the character here.
    if (force_redraw_next && col < screen_Columns && !skip_for_popup(row, col))
    {
	if (enc_dbcs != 0 && dbcs_off2cells(off, max_off) > 1)
	    screen_char_2(off, row, col);
	else
	    screen_char(off, row, col);
    }
}

#if defined(FEAT_SEARCH_EXTRA) || defined(PROTO)
/*
 * Prepare for 'hlsearch' highlighting.
 */
    void
start_search_hl(void)
{
    if (!p_hls || no_hlsearch)
	return;

    end_search_hl();  // just in case it wasn't called before
    last_pat_prog(&screen_search_hl.rm);
    screen_search_hl.attr = HL_ATTR(HLF_L);
}

/*
 * Clean up for 'hlsearch' highlighting.
 */
    void
end_search_hl(void)
{
    if (screen_search_hl.rm.regprog == NULL)
	return;

    vim_regfree(screen_search_hl.rm.regprog);
    screen_search_hl.rm.regprog = NULL;
}
#endif

      static void
screen_start_highlight(int attr)
{
    attrentry_T *aep = NULL;

    screen_attr = attr;
    if (!full_screen
#ifdef MSWIN
	    || !termcap_active
#endif
       )
	return;

#ifdef FEAT_GUI
    if (gui.in_use)
    {
	char	buf[20];

	// The GUI handles this internally.
	sprintf(buf, "\033|%dh", attr);
	OUT_STR(buf);
	return;
    }
#endif

    if (attr > HL_ALL)				// special HL attr.
    {
	if (IS_CTERM)
	    aep = syn_cterm_attr2entry(attr);
	else
	    aep = syn_term_attr2entry(attr);
	if (aep == NULL)	    // did ":syntax clear"
	    attr = 0;
	else
	    attr = aep->ae_attr;
    }
#if defined(FEAT_VTP) && defined(FEAT_TERMGUICOLORS)
    if (use_vtp())
    {
	guicolor_T  defguifg, defguibg;
	int	    defctermfg, defctermbg;

	// If FG and BG are unset, the color is undefined when
	// BOLD+INVERSE. Use Normal as the default value.
	get_default_console_color(&defctermfg, &defctermbg, &defguifg,
		&defguibg);

	if (p_tgc)
	{
	    if (aep == NULL || COLOR_INVALID(aep->ae_u.cterm.fg_rgb))
		term_fg_rgb_color(defguifg);
	    if (aep == NULL || COLOR_INVALID(aep->ae_u.cterm.bg_rgb))
		term_bg_rgb_color(defguibg);
	}
	else if (t_colors >= 256)
	{
	    if (aep == NULL || aep->ae_u.cterm.fg_color == 0)
		term_fg_color(defctermfg);
	    if (aep == NULL || aep->ae_u.cterm.bg_color == 0)
		term_bg_color(defctermbg);
	}
    }
#endif
    if ((attr & HL_BOLD) && *T_MD != NUL)	// bold
	out_str(T_MD);
    else if (aep != NULL && cterm_normal_fg_bold && (
#ifdef FEAT_TERMGUICOLORS
		p_tgc && aep->ae_u.cterm.fg_rgb != CTERMCOLOR
		? aep->ae_u.cterm.fg_rgb != INVALCOLOR
		:
#endif
		t_colors > 1 && aep->ae_u.cterm.fg_color))
	// If the Normal FG color has BOLD attribute and the new HL
	// has a FG color defined, clear BOLD.
	out_str(T_ME);
    if ((attr & HL_STANDOUT) && *T_SO != NUL)	// standout
	out_str(T_SO);
    if ((attr & HL_UNDERCURL) && *T_UCS != NUL) // undercurl
	out_str(T_UCS);
    if ((attr & HL_UNDERDOUBLE) && *T_USS != NUL) // double underline
	out_str(T_USS);
    if ((attr & HL_UNDERDOTTED) && *T_DS != NUL) // dotted underline
	out_str(T_DS);
    if ((attr & HL_UNDERDASHED) && *T_CDS != NUL) // dashed underline
	out_str(T_CDS);
    if (((attr & HL_UNDERLINE)	    // underline or undercurl, etc.
		|| ((attr & HL_UNDERCURL) && *T_UCS == NUL)
		|| ((attr & HL_UNDERDOUBLE) && *T_USS == NUL)
		|| ((attr & HL_UNDERDOTTED) && *T_DS == NUL)
		|| ((attr & HL_UNDERDASHED) && *T_CDS == NUL))
	    && *T_US != NUL)
	out_str(T_US);
    if ((attr & HL_ITALIC) && *T_CZH != NUL)	// italic
	out_str(T_CZH);
    if ((attr & HL_INVERSE) && *T_MR != NUL)	// inverse (reverse)
	out_str(T_MR);
    if ((attr & HL_STRIKETHROUGH) && *T_STS != NUL)	// strike
	out_str(T_STS);

    /*
     * Output the color or start string after bold etc., in case the
     * bold etc. override the color setting.
     */
    if (aep != NULL)
    {
	if (aep->ae_u.cterm.font > 0 && aep->ae_u.cterm.font < 12)
		term_font(aep->ae_u.cterm.font);
#ifdef FEAT_TERMGUICOLORS
	// When 'termguicolors' is set but fg or bg is unset,
	// fall back to the cterm colors.   This helps for SpellBad,
	// where the GUI uses a red undercurl.
	if (p_tgc && aep->ae_u.cterm.fg_rgb != CTERMCOLOR)
	{
	    if (aep->ae_u.cterm.fg_rgb != INVALCOLOR)
		term_fg_rgb_color(aep->ae_u.cterm.fg_rgb);
	}
	else
#endif
	    if (t_colors > 1)
	    {
		if (aep->ae_u.cterm.fg_color)
		    term_fg_color(aep->ae_u.cterm.fg_color - 1);
	    }
#ifdef FEAT_TERMGUICOLORS
	if (p_tgc && aep->ae_u.cterm.bg_rgb != CTERMCOLOR)
	{
	    if (aep->ae_u.cterm.bg_rgb != INVALCOLOR)
		term_bg_rgb_color(aep->ae_u.cterm.bg_rgb);
	}
	else
#endif
	    if (t_colors > 1)
	    {
		if (aep->ae_u.cterm.bg_color)
		    term_bg_color(aep->ae_u.cterm.bg_color - 1);
	    }
#ifdef FEAT_TERMGUICOLORS
	if (p_tgc && aep->ae_u.cterm.ul_rgb != CTERMCOLOR)
	{
	    if (aep->ae_u.cterm.ul_rgb != INVALCOLOR)
		term_ul_rgb_color(aep->ae_u.cterm.ul_rgb);
	}
	else
#endif
	    if (t_colors > 1)
	    {
		if (aep->ae_u.cterm.ul_color)
		    term_ul_color(aep->ae_u.cterm.ul_color - 1);
	    }

	if (!IS_CTERM)
	{
	    if (aep->ae_u.term.start != NULL)
		out_str(aep->ae_u.term.start);
	}
    }
}

      void
screen_stop_highlight(void)
{
    int	    do_ME = FALSE;	    // output T_ME code
#if defined(FEAT_VTP) && defined(FEAT_TERMGUICOLORS)
    int	    do_ME_fg = FALSE, do_ME_bg = FALSE;
#endif

    if (screen_attr != 0
#ifdef MSWIN
			&& termcap_active
#endif
					   )
    {
#ifdef FEAT_GUI
	if (gui.in_use)
	{
	    char	buf[20];

	    // use internal GUI code
	    sprintf(buf, "\033|%dH", screen_attr);
	    OUT_STR(buf);
	}
	else
#endif
	{
	    int is_under;

	    if (screen_attr > HL_ALL)			// special HL attr.
	    {
		attrentry_T *aep;

		if (IS_CTERM)
		{
		    /*
		     * Assume that t_me restores the original colors!
		     */
		    aep = syn_cterm_attr2entry(screen_attr);
		    if (aep != NULL && ((
#ifdef FEAT_TERMGUICOLORS
			    p_tgc && aep->ae_u.cterm.fg_rgb != CTERMCOLOR
				? aep->ae_u.cterm.fg_rgb != INVALCOLOR
# ifdef FEAT_VTP
				    ? !(do_ME_fg = TRUE) : (do_ME_fg = FALSE)
# endif
				:
#endif
				aep->ae_u.cterm.fg_color) || (
#ifdef FEAT_TERMGUICOLORS
			    p_tgc && aep->ae_u.cterm.bg_rgb != CTERMCOLOR
				? aep->ae_u.cterm.bg_rgb != INVALCOLOR
# ifdef FEAT_VTP
				    ? !(do_ME_bg = TRUE) : (do_ME_bg = FALSE)
# endif
				:
#endif
				aep->ae_u.cterm.bg_color)))
			do_ME = TRUE;
#if defined(FEAT_VTP) && defined(FEAT_TERMGUICOLORS)
		    if (use_vtp())
		    {
			if (do_ME_fg && do_ME_bg)
			    do_ME = TRUE;

			// FG and BG cannot be separated in T_ME, which is not
			// efficient.
			if (!do_ME && do_ME_fg)
			    out_str((char_u *)"\033|39m"); // restore FG
			if (!do_ME && do_ME_bg)
			    out_str((char_u *)"\033|49m"); // restore BG
		    }
		    else
		    {
			// Process FG and BG at once.
			if (!do_ME)
			    do_ME = do_ME_fg | do_ME_bg;
		    }
#endif
		}
		else
		{
		    aep = syn_term_attr2entry(screen_attr);
		    if (aep != NULL && aep->ae_u.term.stop != NULL)
		    {
			if (STRCMP(aep->ae_u.term.stop, T_ME) == 0)
			    do_ME = TRUE;
			else
			    out_str(aep->ae_u.term.stop);
		    }
		}
		if (aep == NULL)	    // did ":syntax clear"
		    screen_attr = 0;
		else
		    screen_attr = aep->ae_attr;
	    }

	    /*
	     * Often all ending-codes are equal to T_ME.  Avoid outputting the
	     * same sequence several times.
	     */
	    if (screen_attr & HL_STANDOUT)
	    {
		if (STRCMP(T_SE, T_ME) == 0)
		    do_ME = TRUE;
		else
		    out_str(T_SE);
	    }
	    is_under = (screen_attr & (HL_UNDERCURL
			  | HL_UNDERDOUBLE | HL_UNDERDOTTED | HL_UNDERDASHED));
	    if (is_under && *T_UCE != NUL)
	    {
		if (STRCMP(T_UCE, T_ME) == 0)
		    do_ME = TRUE;
		else
		    out_str(T_UCE);
	    }
	    if ((screen_attr & HL_UNDERLINE) || (is_under && *T_UCE == NUL))
	    {
		if (STRCMP(T_UE, T_ME) == 0)
		    do_ME = TRUE;
		else
		    out_str(T_UE);
	    }
	    if (screen_attr & HL_ITALIC)
	    {
		if (STRCMP(T_CZR, T_ME) == 0)
		    do_ME = TRUE;
		else
		    out_str(T_CZR);
	    }
	    if (screen_attr & HL_STRIKETHROUGH)
	    {
		if (STRCMP(T_STE, T_ME) == 0)
		    do_ME = TRUE;
		else
		    out_str(T_STE);
	    }
	    if (do_ME || (screen_attr & (HL_BOLD | HL_INVERSE)))
		out_str(T_ME);

#ifdef FEAT_TERMGUICOLORS
	    if (p_tgc)
	    {
		if (cterm_normal_fg_gui_color != INVALCOLOR)
		    term_fg_rgb_color(cterm_normal_fg_gui_color);
		if (cterm_normal_bg_gui_color != INVALCOLOR)
		    term_bg_rgb_color(cterm_normal_bg_gui_color);
		if (cterm_normal_ul_gui_color != INVALCOLOR)
		    term_ul_rgb_color(cterm_normal_ul_gui_color);
	    }
	    else
#endif
	    {
		if (t_colors > 1)
		{
		    // set Normal cterm colors
		    if (cterm_normal_fg_color != 0)
			term_fg_color(cterm_normal_fg_color - 1);
		    if (cterm_normal_bg_color != 0)
			term_bg_color(cterm_normal_bg_color - 1);
		    if (cterm_normal_ul_color != 0)
			term_ul_color(cterm_normal_ul_color - 1);
		    if (cterm_normal_fg_bold)
			out_str(T_MD);
		}
	    }
	}
    }
    screen_attr = 0;
}

/*
 * Reset the colors for a cterm.  Used when leaving Vim.
 * The machine specific code may override this again.
 */
    void
reset_cterm_colors(void)
{
    if (!IS_CTERM)
	return;

    // set Normal cterm colors
#ifdef FEAT_TERMGUICOLORS
    if (p_tgc ? (cterm_normal_fg_gui_color != INVALCOLOR
		|| cterm_normal_bg_gui_color != INVALCOLOR)
	    : (cterm_normal_fg_color > 0 || cterm_normal_bg_color > 0))
#else
	if (cterm_normal_fg_color > 0 || cterm_normal_bg_color > 0)
#endif
	{
	    out_str(T_OP);
	    screen_attr = -1;
	}
    if (cterm_normal_fg_bold)
    {
	out_str(T_ME);
	screen_attr = -1;
    }
}

/*
 * Put character ScreenLines["off"] on the screen at position "row" and "col",
 * using the attributes from ScreenAttrs["off"].
 */
    void
screen_char(unsigned off, int row, int col)
{
    int		attr;

    // Check for illegal values, just in case (could happen just after
    // resizing).
    if (row >= screen_Rows || col >= screen_Columns)
	return;

    // Outputting a character in the last cell on the screen may scroll the
    // screen up.  Only do it when the "xn" termcap property is set, otherwise
    // mark the character invalid (update it when scrolled up).
    if (*T_XN == NUL
	    && row == screen_Rows - 1 && col == screen_Columns - 1
#ifdef FEAT_RIGHTLEFT
	    // account for first command-line character in rightleft mode
	    && !cmdmsg_rl
#endif
       )
    {
	ScreenAttrs[off] = (sattr_T)-1;
	ScreenCols[off] = -1;
	return;
    }

    /*
     * Stop highlighting first, so it's easier to move the cursor.
     */
    if (screen_char_attr != 0)
	attr = screen_char_attr;
    else
	attr = ScreenAttrs[off];
    if (screen_attr != attr)
	screen_stop_highlight();

    windgoto(row, col);

    if (screen_attr != attr)
	screen_start_highlight(attr);

    if (enc_utf8 && ScreenLinesUC[off] != 0)
    {
	char_u	    buf[MB_MAXBYTES + 1];

	if (utf_ambiguous_width(ScreenLinesUC[off]))
	{
	    if (*p_ambw == 'd'
#ifdef FEAT_GUI
		    && !gui.in_use
#endif
		    )
	    {
		// Clear the two screen cells. If the character is actually
		// single width it won't change the second cell.
		out_str((char_u *)"  ");
		term_windgoto(row, col);
	    }
	    // not sure where the cursor is after drawing the ambiguous width
	    // character
	    screen_cur_col = 9999;
	}
	else if (utf_char2cells(ScreenLinesUC[off]) > 1)
	    ++screen_cur_col;

	// Convert the UTF-8 character to bytes and write it.
	buf[utfc_char2bytes(off, buf)] = NUL;
	out_str(buf);
    }
    else
    {
	out_flush_check();
	out_char(ScreenLines[off]);
	// double-byte character in single-width cell
	if (enc_dbcs == DBCS_JPNU && ScreenLines[off] == 0x8e)
	    out_char(ScreenLines2[off]);
    }

    screen_cur_col++;
}

/*
 * Used for enc_dbcs only: Put one double-wide character at ScreenLines["off"]
 * on the screen at position 'row' and 'col'.
 * The attributes of the first byte is used for all.  This is required to
 * output the two bytes of a double-byte character with nothing in between.
 */
    static void
screen_char_2(unsigned off, int row, int col)
{
    // Check for illegal values (could be wrong when screen was resized).
    if (off + 1 >= (unsigned)(screen_Rows * screen_Columns))
	return;

    // Outputting the last character on the screen may scroll the screen up.
    // Don't to it!  Mark the character invalid (update it when scrolled up)
    if (row == screen_Rows - 1 && col >= screen_Columns - 2)
    {
	ScreenAttrs[off] = (sattr_T)-1;
	ScreenCols[off] = -1;
	return;
    }

    // Output the first byte normally (positions the cursor), then write the
    // second byte directly.
    screen_char(off, row, col);
    out_char(ScreenLines[off + 1]);
    ++screen_cur_col;
}

/*
 * Draw a rectangle of the screen, inverted when "invert" is TRUE.
 * This uses the contents of ScreenLines[] and doesn't change it.
 */
    void
screen_draw_rectangle(
    int		row,
    int		col,
    int		height,
    int		width,
    int		invert)
{
    int		r, c;
    int		off;
    int		max_off;

    // Can't use ScreenLines unless initialized
    if (ScreenLines == NULL)
	return;

    if (invert)
	screen_char_attr = HL_INVERSE;
    for (r = row; r < row + height; ++r)
    {
	off = LineOffset[r];
	max_off = off + screen_Columns;
	for (c = col; c < col + width; ++c)
	{
	    if (enc_dbcs != 0 && dbcs_off2cells(off + c, max_off) > 1)
	    {
		if (!skip_for_popup(r, c))
		    screen_char_2(off + c, r, c);
		++c;
	    }
	    else
	    {
		if (!skip_for_popup(r, c))
		    screen_char(off + c, r, c);
		if (utf_off2cells(off + c, max_off) > 1)
		    ++c;
	    }
	}
    }
    screen_char_attr = 0;
}

/*
 * Redraw the characters for a vertically split window.
 */
    static void
redraw_block(int row, int end, win_T *wp)
{
    int		col;
    int		width;

# ifdef FEAT_CLIPBOARD
    clip_may_clear_selection(row, end - 1);
# endif

    if (wp == NULL)
    {
	col = 0;
	width = Columns;
    }
    else
    {
	col = wp->w_wincol;
	width = wp->w_width;
    }
    screen_draw_rectangle(row, col, end - row, width, FALSE);
}

    void
space_to_screenline(int off, int attr)
{
    ScreenLines[off] = ' ';
    ScreenAttrs[off] = attr;
    ScreenCols[off] = -1;
    if (enc_utf8)
	ScreenLinesUC[off] = 0;
}

/*
 * Fill the screen from "start_row" to "end_row" (exclusive), from "start_col"
 * to "end_col" (exclusive) with character "c1" in first column followed by
 * "c2" in the other columns.  Use attributes "attr".
 */
    void
screen_fill(
	int	start_row,
	int	end_row,
	int	start_col,
	int	end_col,
	int	c1,
	int	c2,
	int	attr)
{
    int	    row;
    int	    col;
    int	    off;
    int	    end_off;
    int	    did_delete;
    int	    c;
    int	    norm_term;
#if defined(FEAT_GUI) || defined(UNIX)
    int	    force_next = FALSE;
#endif

    if (end_row > screen_Rows)		// safety check
	end_row = screen_Rows;
    if (end_col > screen_Columns)	// safety check
	end_col = screen_Columns;
    if (ScreenLines == NULL
	    || start_row >= end_row
	    || start_col >= end_col)	// nothing to do
	return;

    // it's a "normal" terminal when not in a GUI or cterm
    norm_term = (
#ifdef FEAT_GUI
	    !gui.in_use &&
#endif
	    !IS_CTERM);
    for (row = start_row; row < end_row; ++row)
    {
	if (has_mbyte
#ifdef FEAT_GUI
		&& !gui.in_use
#endif
	   )
	{
	    // When drawing over the right half of a double-wide char clear
	    // out the left half.  When drawing over the left half of a
	    // double wide-char clear out the right half.  Only needed in a
	    // terminal.
	    if (start_col > 0 && mb_fix_col(start_col, row) != start_col)
		screen_puts_len((char_u *)" ", 1, row, start_col - 1, 0);
	    if (end_col < screen_Columns && mb_fix_col(end_col, row) != end_col)
		screen_puts_len((char_u *)" ", 1, row, end_col, 0);
	}
	/*
	 * Try to use delete-line termcap code, when no attributes or in a
	 * "normal" terminal, where a bold/italic space is just a
	 * space.
	 */
	did_delete = FALSE;
	if (c2 == ' '
		&& end_col == Columns
		&& can_clear(T_CE)
		&& (attr == 0
		    || (norm_term
			&& attr <= HL_ALL
			&& ((attr & ~(HL_BOLD | HL_ITALIC)) == 0))))
	{
	    /*
	     * check if we really need to clear something
	     */
	    col = start_col;
	    if (c1 != ' ')			// don't clear first char
		++col;

	    off = LineOffset[row] + col;
	    end_off = LineOffset[row] + end_col;

	    // skip blanks (used often, keep it fast!)
	    if (enc_utf8)
		while (off < end_off && ScreenLines[off] == ' '
			  && ScreenAttrs[off] == 0 && ScreenLinesUC[off] == 0)
		    ++off;
	    else
		while (off < end_off && ScreenLines[off] == ' '
						     && ScreenAttrs[off] == 0)
		    ++off;
	    if (off < end_off)		// something to be cleared
	    {
		col = off - LineOffset[row];
		screen_stop_highlight();
		term_windgoto(row, col);// clear rest of this screen line
		out_str(T_CE);
		screen_start();		// don't know where cursor is now
		col = end_col - col;
		while (col--)		// clear chars in ScreenLines
		{
		    space_to_screenline(off, 0);
		    ++off;
		}
	    }
	    did_delete = TRUE;		// the chars are cleared now
	}

	off = LineOffset[row] + start_col;
	c = c1;
	for (col = start_col; col < end_col; ++col)
	{
	    if ((ScreenLines[off] != c
		    || (enc_utf8 && (int)ScreenLinesUC[off]
						       != (c >= 0x80 ? c : 0))
		    || ScreenAttrs[off] != attr
		    || must_redraw == UPD_CLEAR  // screen clear pending
#if defined(FEAT_GUI) || defined(UNIX)
		    || force_next
#endif
		    )
		    // Skip if under a(nother) popup.
		    && !skip_for_popup(row, col))
	    {
#if defined(FEAT_GUI) || defined(UNIX)
		// The bold trick may make a single row of pixels appear in
		// the next character.  When a bold character is removed, the
		// next character should be redrawn too.  This happens for our
		// own GUI and for some xterms.
		if (
# ifdef FEAT_GUI
			gui.in_use
# endif
# if defined(FEAT_GUI) && defined(UNIX)
			||
# endif
# ifdef UNIX
			term_is_xterm
# endif
		   )
		{
		    if (ScreenLines[off] != ' '
			    && (ScreenAttrs[off] > HL_ALL
				|| ScreenAttrs[off] & HL_BOLD))
			force_next = TRUE;
		    else
			force_next = FALSE;
		}
#endif // FEAT_GUI || defined(UNIX)
		ScreenLines[off] = c;
		if (enc_utf8)
		{
		    if (c >= 0x80)
		    {
			ScreenLinesUC[off] = c;
			ScreenLinesC[0][off] = 0;
		    }
		    else
			ScreenLinesUC[off] = 0;
		}
		ScreenAttrs[off] = attr;
		if (!did_delete || c != ' ')
		    screen_char(off, row, col);
	    }
	    ScreenCols[off] = -1;
	    ++off;
	    if (col == start_col)
	    {
		if (did_delete)
		    break;
		c = c2;
	    }
	}
	if (end_col == Columns)
	    LineWraps[row] = FALSE;
	if (row == Rows - 1)		// overwritten the command line
	{
	    redraw_cmdline = TRUE;
	    if (start_col == 0 && end_col == Columns
		    && c1 == ' ' && c2 == ' ' && attr == 0)
		clear_cmdline = FALSE;	// command line has been cleared
	    if (start_col == 0)
		mode_displayed = FALSE; // mode cleared or overwritten
	}
    }
}

/*
 * Check if there should be a delay.  Used before clearing or redrawing the
 * screen or the command line.
 */
    void
check_for_delay(int check_msg_scroll)
{
    if ((emsg_on_display || (check_msg_scroll && msg_scroll))
	    && !did_wait_return
	    && emsg_silent == 0
	    && !in_assert_fails)
    {
	out_flush();
	ui_delay(1006L, TRUE);
	emsg_on_display = FALSE;
	if (check_msg_scroll)
	    msg_scroll = FALSE;
    }
}

/*
 * Init TabPageIdxs[] to zero: Clicking outside of tabs has no effect.
 */
    static void
clear_TabPageIdxs(void)
{
    int		scol;

    for (scol = 0; scol < Columns; ++scol)
	TabPageIdxs[scol] = 0;
}

/*
 * screen_valid -  allocate screen buffers if size changed
 *   If "doclear" is TRUE: clear screen if it has been resized.
 *	Returns TRUE if there is a valid screen to write to.
 *	Returns FALSE when starting up and screen not initialized yet.
 */
    int
screen_valid(int doclear)
{
    screenalloc(doclear);	   // allocate screen buffers if size changed
    return (ScreenLines != NULL);
}

/*
 * Resize the shell to Rows and Columns.
 * Allocate ScreenLines[] and associated items.
 *
 * There may be some time between setting Rows and Columns and (re)allocating
 * ScreenLines[].  This happens when starting up and when (manually) changing
 * the shell size.  Always use screen_Rows and screen_Columns to access items
 * in ScreenLines[].  Use Rows and Columns for positioning text etc. where the
 * final size of the shell is needed.
 */
    void
screenalloc(int doclear)
{
    int		    new_row, old_row;
#ifdef FEAT_GUI
    int		    old_Rows;
#endif
    win_T	    *wp;
    int		    outofmem = FALSE;
    int		    len;
    schar_T	    *new_ScreenLines;
    u8char_T	    *new_ScreenLinesUC = NULL;
    u8char_T	    *new_ScreenLinesC[MAX_MCO];
    schar_T	    *new_ScreenLines2 = NULL;
    sattr_T	    *new_ScreenAttrs;
    colnr_T	    *new_ScreenCols;
    unsigned	    *new_LineOffset;
    char_u	    *new_LineWraps;
    short	    *new_TabPageIdxs;
#ifdef FEAT_PROP_POPUP
    short	    *new_popup_mask;
    short	    *new_popup_mask_next;
    char	    *new_popup_transparent;
#endif
    tabpage_T	    *tp;
    static int	    entered = FALSE;		// avoid recursiveness
    static int	    done_outofmem_msg = FALSE;	// did outofmem message
    int		    retry_count = 0;
    int		    found_null;

retry:
    /*
     * Allocation of the screen buffers is done only when the size changes and
     * when Rows and Columns have been set and we have started doing full
     * screen stuff.
     */
    if ((ScreenLines != NULL
		&& Rows == screen_Rows
		&& Columns == screen_Columns
		&& enc_utf8 == (ScreenLinesUC != NULL)
		&& (enc_dbcs == DBCS_JPNU) == (ScreenLines2 != NULL)
		&& p_mco == Screen_mco)
	    || Rows == 0
	    || Columns == 0
	    || (!full_screen && ScreenLines == NULL))
	return;

    /*
     * It's possible that we produce an out-of-memory message below, which
     * will cause this function to be called again.  To break the loop, just
     * return here.
     */
    if (entered)
	return;
    entered = TRUE;

    /*
     * Note that the window sizes are updated before reallocating the arrays,
     * thus we must not redraw here!
     */
    ++RedrawingDisabled;

    win_new_shellsize();    // fit the windows in the new sized shell

#ifdef FEAT_GUI_HAIKU
    vim_lock_screen();  // be safe, put it here
#endif

    comp_col();		// recompute columns for shown command and ruler

    /*
     * We're changing the size of the screen.
     * - Allocate new arrays for ScreenLines and ScreenAttrs.
     * - Move lines from the old arrays into the new arrays, clear extra
     *	 lines (unless the screen is going to be cleared).
     * - Free the old arrays.
     *
     * If anything fails, make ScreenLines NULL, so we don't do anything!
     * Continuing with the old ScreenLines may result in a crash, because the
     * size is wrong.
     */
    FOR_ALL_TAB_WINDOWS(tp, wp)
	win_free_lsize(wp);
    for (int i = 0; i < AUCMD_WIN_COUNT; ++i)
	if (aucmd_win[i].auc_win != NULL)
	    win_free_lsize(aucmd_win[i].auc_win);
#ifdef FEAT_PROP_POPUP
    // global popup windows
    FOR_ALL_POPUPWINS(wp)
	win_free_lsize(wp);
    // tab-local popup windows
    FOR_ALL_TABPAGES(tp)
	FOR_ALL_POPUPWINS_IN_TAB(tp, wp)
	    win_free_lsize(wp);
#endif

    new_ScreenLines = LALLOC_MULT(schar_T, (Rows + 1) * Columns);
    vim_memset(new_ScreenLinesC, 0, sizeof(u8char_T *) * MAX_MCO);
    if (enc_utf8)
    {
	new_ScreenLinesUC = LALLOC_MULT(u8char_T, (Rows + 1) * Columns);
	for (int i = 0; i < p_mco; ++i)
	    new_ScreenLinesC[i] = LALLOC_CLEAR_MULT(u8char_T,
							 (Rows + 1) * Columns);
    }
    if (enc_dbcs == DBCS_JPNU)
	new_ScreenLines2 = LALLOC_MULT(schar_T, (Rows + 1) * Columns);
    new_ScreenAttrs = LALLOC_MULT(sattr_T, (Rows + 1) * Columns);
    // Clear ScreenCols to avoid a warning for uninitialized memory in
    // jump_to_mouse().
    new_ScreenCols = LALLOC_CLEAR_MULT(colnr_T, (Rows + 1) * Columns);
    new_LineOffset = LALLOC_MULT(unsigned, Rows);
    new_LineWraps = LALLOC_MULT(char_u, Rows);
    new_TabPageIdxs = LALLOC_MULT(short, Columns);
#ifdef FEAT_PROP_POPUP
    new_popup_mask = LALLOC_MULT(short, Rows * Columns);
    new_popup_mask_next = LALLOC_MULT(short, Rows * Columns);
    new_popup_transparent = LALLOC_MULT(char, Rows * Columns);
#endif

    FOR_ALL_TAB_WINDOWS(tp, wp)
    {
	if (win_alloc_lines(wp) == FAIL)
	{
	    outofmem = TRUE;
	    goto give_up;
	}
    }
    for (int i = 0; i < AUCMD_WIN_COUNT; ++i)
	if (aucmd_win[i].auc_win != NULL
		&& aucmd_win[i].auc_win->w_lines == NULL
		&& win_alloc_lines(aucmd_win[i].auc_win) == FAIL)
	{
	    outofmem = TRUE;
	    break;
	}
#ifdef FEAT_PROP_POPUP
    // global popup windows
    FOR_ALL_POPUPWINS(wp)
	if (win_alloc_lines(wp) == FAIL)
	{
	    outofmem = TRUE;
	    goto give_up;
	}
    // tab-local popup windows
    FOR_ALL_TABPAGES(tp)
	FOR_ALL_POPUPWINS_IN_TAB(tp, wp)
	    if (win_alloc_lines(wp) == FAIL)
	    {
		outofmem = TRUE;
		goto give_up;
	    }
#endif

give_up:
    found_null = FALSE;
    for (int i = 0; i < p_mco; ++i)
	if (new_ScreenLinesC[i] == NULL)
	{
	    found_null = TRUE;
	    break;
	}
    if (new_ScreenLines == NULL
	    || (enc_utf8 && (new_ScreenLinesUC == NULL || found_null))
	    || (enc_dbcs == DBCS_JPNU && new_ScreenLines2 == NULL)
	    || new_ScreenAttrs == NULL
	    || new_ScreenCols == NULL
	    || new_LineOffset == NULL
	    || new_LineWraps == NULL
	    || new_TabPageIdxs == NULL
#ifdef FEAT_PROP_POPUP
	    || new_popup_mask == NULL
	    || new_popup_mask_next == NULL
	    || new_popup_transparent == NULL
#endif
	    || outofmem)
    {
	if (ScreenLines != NULL || !done_outofmem_msg)
	{
	    // guess the size
	    do_outofmem_msg((long_u)((Rows + 1) * Columns));

	    // Remember we did this to avoid getting outofmem messages over
	    // and over again.
	    done_outofmem_msg = TRUE;
	}
	VIM_CLEAR(new_ScreenLines);
	VIM_CLEAR(new_ScreenLinesUC);
	for (int i = 0; i < p_mco; ++i)
	    VIM_CLEAR(new_ScreenLinesC[i]);
	VIM_CLEAR(new_ScreenLines2);
	VIM_CLEAR(new_ScreenAttrs);
	VIM_CLEAR(new_ScreenCols);
	VIM_CLEAR(new_LineOffset);
	VIM_CLEAR(new_LineWraps);
	VIM_CLEAR(new_TabPageIdxs);
#ifdef FEAT_PROP_POPUP
	VIM_CLEAR(new_popup_mask);
	VIM_CLEAR(new_popup_mask_next);
	VIM_CLEAR(new_popup_transparent);
#endif
    }
    else
    {
	done_outofmem_msg = FALSE;

	for (new_row = 0; new_row < Rows; ++new_row)
	{
	    new_LineOffset[new_row] = new_row * Columns;
	    new_LineWraps[new_row] = FALSE;

	    /*
	     * If the screen is not going to be cleared, copy as much as
	     * possible from the old screen to the new one and clear the rest
	     * (used when resizing the window at the "--more--" prompt or when
	     * executing an external command, for the GUI).
	     */
	    if (!doclear)
	    {
		(void)vim_memset(new_ScreenLines + new_row * Columns,
				      ' ', (size_t)Columns * sizeof(schar_T));
		if (enc_utf8)
		{
		    (void)vim_memset(new_ScreenLinesUC + new_row * Columns,
				       0, (size_t)Columns * sizeof(u8char_T));
		    for (int i = 0; i < p_mco; ++i)
			(void)vim_memset(new_ScreenLinesC[i]
							  + new_row * Columns,
				       0, (size_t)Columns * sizeof(u8char_T));
		}
		if (enc_dbcs == DBCS_JPNU)
		    (void)vim_memset(new_ScreenLines2 + new_row * Columns,
				       0, (size_t)Columns * sizeof(schar_T));
		(void)vim_memset(new_ScreenAttrs + new_row * Columns,
					0, (size_t)Columns * sizeof(sattr_T));
		(void)vim_memset(new_ScreenCols + new_row * Columns,
					0, (size_t)Columns * sizeof(colnr_T));
		old_row = new_row + (screen_Rows - Rows);
		if (old_row >= 0 && ScreenLines != NULL)
		{
		    if (screen_Columns < Columns)
			len = screen_Columns;
		    else
			len = Columns;
		    // When switching to utf-8 don't copy characters, they
		    // may be invalid now.  Also when p_mco changes.
		    if (!(enc_utf8 && ScreenLinesUC == NULL)
						       && p_mco == Screen_mco)
			mch_memmove(new_ScreenLines + new_LineOffset[new_row],
				ScreenLines + LineOffset[old_row],
				(size_t)len * sizeof(schar_T));
		    if (enc_utf8 && ScreenLinesUC != NULL
						       && p_mco == Screen_mco)
		    {
			mch_memmove(new_ScreenLinesUC + new_LineOffset[new_row],
				ScreenLinesUC + LineOffset[old_row],
				(size_t)len * sizeof(u8char_T));
			for (int i = 0; i < p_mco; ++i)
			    mch_memmove(new_ScreenLinesC[i]
						    + new_LineOffset[new_row],
				ScreenLinesC[i] + LineOffset[old_row],
				(size_t)len * sizeof(u8char_T));
		    }
		    if (enc_dbcs == DBCS_JPNU && ScreenLines2 != NULL)
			mch_memmove(new_ScreenLines2 + new_LineOffset[new_row],
				ScreenLines2 + LineOffset[old_row],
				(size_t)len * sizeof(schar_T));
		    mch_memmove(new_ScreenAttrs + new_LineOffset[new_row],
			    ScreenAttrs + LineOffset[old_row],
			    (size_t)len * sizeof(sattr_T));
		    mch_memmove(new_ScreenCols + new_LineOffset[new_row],
			    ScreenAttrs + LineOffset[old_row],
			    (size_t)len * sizeof(colnr_T));
		}
	    }
	}
	// Use the last line of the screen for the current line.
	current_ScreenLine = new_ScreenLines + Rows * Columns;

#ifdef FEAT_PROP_POPUP
	vim_memset(new_popup_mask, 0, Rows * Columns * sizeof(short));
	vim_memset(new_popup_transparent, 0, Rows * Columns * sizeof(char));
#endif
    }

    free_screenlines();

    // NOTE: this may result in all pointers to become NULL.
    ScreenLines = new_ScreenLines;
    ScreenLinesUC = new_ScreenLinesUC;
    for (int i = 0; i < p_mco; ++i)
	ScreenLinesC[i] = new_ScreenLinesC[i];
    Screen_mco = p_mco;
    ScreenLines2 = new_ScreenLines2;
    ScreenAttrs = new_ScreenAttrs;
    ScreenCols = new_ScreenCols;
    LineOffset = new_LineOffset;
    LineWraps = new_LineWraps;
    TabPageIdxs = new_TabPageIdxs;
#ifdef FEAT_PROP_POPUP
    popup_mask = new_popup_mask;
    popup_mask_next = new_popup_mask_next;
    popup_transparent = new_popup_transparent;
    popup_mask_refresh = TRUE;
#endif

    // It's important that screen_Rows and screen_Columns reflect the actual
    // size of ScreenLines[].  Set them before calling anything.
#ifdef FEAT_GUI
    old_Rows = screen_Rows;
#endif
    screen_Rows = Rows;
    screen_Columns = Columns;

    set_must_redraw(UPD_CLEAR);	// need to clear the screen later
    if (doclear)
	screenclear2(TRUE);
#ifdef FEAT_GUI
    else if (gui.in_use
	    && !gui.starting
	    && ScreenLines != NULL
	    && old_Rows != Rows)
    {
	gui_redraw_block(0, 0, (int)Rows - 1, (int)Columns - 1, 0);

	// Adjust the position of the cursor, for when executing an external
	// command.
	if (msg_row >= Rows)		// Rows got smaller
	    msg_row = Rows - 1;		// put cursor at last row
	else if (Rows > old_Rows)	// Rows got bigger
	    msg_row += Rows - old_Rows; // put cursor in same place
	if (msg_col >= Columns)		// Columns got smaller
	    msg_col = Columns - 1;	// put cursor at last column
    }
#endif
    clear_TabPageIdxs();

#ifdef FEAT_GUI_HAIKU
    vim_unlock_screen();
#endif

    entered = FALSE;
    if (RedrawingDisabled > 0)
	--RedrawingDisabled;

    /*
     * Do not apply autocommands more than 3 times to avoid an endless loop
     * in case applying autocommands always changes Rows or Columns.
     */
    if (starting == 0 && ++retry_count <= 3)
    {
	apply_autocmds(EVENT_VIMRESIZED, NULL, NULL, FALSE, curbuf);
	// In rare cases, autocommands may have altered Rows or Columns,
	// jump back to check if we need to allocate the screen again.
	goto retry;
    }
}

    void
free_screenlines(void)
{
    int		i;

    VIM_CLEAR(ScreenLinesUC);
    for (i = 0; i < Screen_mco; ++i)
	VIM_CLEAR(ScreenLinesC[i]);
    VIM_CLEAR(ScreenLines2);
    VIM_CLEAR(ScreenLines);
    VIM_CLEAR(ScreenAttrs);
    VIM_CLEAR(ScreenCols);
    VIM_CLEAR(LineOffset);
    VIM_CLEAR(LineWraps);
    VIM_CLEAR(TabPageIdxs);
#ifdef FEAT_PROP_POPUP
    VIM_CLEAR(popup_mask);
    VIM_CLEAR(popup_mask_next);
    VIM_CLEAR(popup_transparent);
#endif
}

/*
 * Clear the screen.
 * May delay if there is something the user should read.
 * Allocated the screen for resizing if needed.
 * Returns TRUE when the screen was actually cleared, FALSE if all display
 * cells were marked for updating.
 */
    int
screenclear(void)
{
    check_for_delay(FALSE);
    screenalloc(FALSE);		    // allocate screen buffers if size changed
    return screenclear2(TRUE);	    // clear the screen
}

/*
 * Do not clear the screen but mark everything for redraw.
 */
    void
redraw_as_cleared(void)
{
    screenclear2(FALSE);
}

    static int
screenclear2(int doclear)
{
    int	    i;
    int	    did_clear = FALSE;

    if (starting == NO_SCREEN || ScreenLines == NULL
#ifdef FEAT_GUI
	    || (gui.in_use && gui.starting)
#endif
	    )
	return FALSE;

#ifdef FEAT_GUI
    if (!gui.in_use)
#endif
	screen_attr = -1;	// force setting the Normal colors
    screen_stop_highlight();	// don't want highlighting here

#ifdef FEAT_CLIPBOARD
    // disable selection without redrawing it
    clip_scroll_selection(9999);
#endif

    // blank out ScreenLines
    for (i = 0; i < Rows; ++i)
    {
	lineclear(LineOffset[i], (int)Columns, 0);
	LineWraps[i] = FALSE;
    }

    if (doclear && can_clear(T_CL))
    {
	out_str(T_CL);		// clear the display
	did_clear = TRUE;
	clear_cmdline = FALSE;
	mode_displayed = FALSE;
    }
    else
    {
	// can't clear the screen, mark all chars with invalid attributes
	for (i = 0; i < Rows; ++i)
	    lineinvalid(LineOffset[i], (int)Columns);
	clear_cmdline = TRUE;
    }

    screen_cleared = TRUE;	// can use contents of ScreenLines now

    win_rest_invalid(firstwin);	// redraw all regular windows
    redraw_cmdline = TRUE;
    redraw_tabline = TRUE;
    if (must_redraw == UPD_CLEAR)	// no need to clear again
	must_redraw = UPD_NOT_VALID;
    msg_scrolled = 0;		// compute_cmdrow() uses this
    compute_cmdrow();
#ifdef FEAT_PROP_POPUP
    popup_redraw_all();		// redraw all popup windows
#endif
    msg_row = cmdline_row;	// put cursor on last line for messages
    msg_col = 0;
    screen_start();		// don't know where cursor is now
    msg_didany = FALSE;
    msg_didout = FALSE;

    return did_clear;
}

/*
 * Clear one line in ScreenLines.
 */
    static void
lineclear(unsigned off, int width, int attr)
{
    (void)vim_memset(ScreenLines + off, ' ', (size_t)width * sizeof(schar_T));
    if (enc_utf8)
	(void)vim_memset(ScreenLinesUC + off, 0,
					  (size_t)width * sizeof(u8char_T));
    (void)vim_memset(ScreenAttrs + off, attr, (size_t)width * sizeof(sattr_T));
    (void)vim_memset(ScreenCols + off, -1, (size_t)width * sizeof(colnr_T));
}

/*
 * Mark one line in ScreenLines invalid by setting the attributes to an
 * invalid value.
 */
    static void
lineinvalid(unsigned off, int width)
{
    (void)vim_memset(ScreenAttrs + off, -1, (size_t)width * sizeof(sattr_T));
    (void)vim_memset(ScreenCols + off, -1, (size_t)width * sizeof(colnr_T));
}

/*
 * To be called when characters were sent to the terminal directly, outputting
 * test on "screen_lnum".
 */
    void
line_was_clobbered(int screen_lnum)
{
    lineinvalid(LineOffset[screen_lnum], (int)Columns);
}

/*
 * Copy part of a Screenline for vertically split window "wp".
 */
    static void
linecopy(int to, int from, win_T *wp)
{
    unsigned	off_to = LineOffset[to] + wp->w_wincol;
    unsigned	off_from = LineOffset[from] + wp->w_wincol;

    mch_memmove(ScreenLines + off_to, ScreenLines + off_from,
	    wp->w_width * sizeof(schar_T));
    if (enc_utf8)
    {
	int	i;

	mch_memmove(ScreenLinesUC + off_to, ScreenLinesUC + off_from,
		wp->w_width * sizeof(u8char_T));
	for (i = 0; i < p_mco; ++i)
	    mch_memmove(ScreenLinesC[i] + off_to, ScreenLinesC[i] + off_from,
		    wp->w_width * sizeof(u8char_T));
    }
    if (enc_dbcs == DBCS_JPNU)
	mch_memmove(ScreenLines2 + off_to, ScreenLines2 + off_from,
		wp->w_width * sizeof(schar_T));
    mch_memmove(ScreenAttrs + off_to, ScreenAttrs + off_from,
	    wp->w_width * sizeof(sattr_T));
    mch_memmove(ScreenCols + off_to, ScreenCols + off_from,
	    wp->w_width * sizeof(colnr_T));
}

/*
 * Return TRUE if clearing with term string "p" would work.
 * It can't work when the string is empty or it won't set the right background.
 * Don't clear to end-of-line when there are popups, it may cause flicker.
 */
    int
can_clear(char_u *p)
{
    return (*p != NUL && (t_colors <= 1
#ifdef FEAT_GUI
		|| gui.in_use
#endif
#ifdef FEAT_TERMGUICOLORS
		|| (p_tgc && cterm_normal_bg_gui_color == INVALCOLOR)
		|| (!p_tgc && cterm_normal_bg_color == 0)
#else
		|| cterm_normal_bg_color == 0
#endif
		|| *T_UT != NUL)
#ifdef FEAT_PROP_POPUP
	    && !(p == T_CE && popup_visible)
#endif
	    );
}

/*
 * Reset cursor position. Use whenever cursor was moved because of outputting
 * something directly to the screen (shell commands) or a terminal control
 * code.
 */
    void
screen_start(void)
{
    screen_cur_row = screen_cur_col = 9999;
}

/*
 * Move the cursor to position "row","col" in the screen.
 * This tries to find the most efficient way to move, minimizing the number of
 * characters sent to the terminal.
 */
    void
windgoto(int row, int col)
{
    sattr_T	    *p;
    int		    i;
    int		    plan;
    int		    cost;
    int		    wouldbe_col;
    int		    noinvcurs;
    char_u	    *bs;
    int		    goto_cost;
    int		    attr;

#define GOTO_COST   7	// assume a term_windgoto() takes about 7 chars
#define HIGHL_COST  5	// assume unhighlight takes 5 chars

#define PLAN_LE	    1
#define PLAN_CR	    2
#define PLAN_NL	    3
#define PLAN_WRITE  4
    // Can't use ScreenLines unless initialized
    if (ScreenLines == NULL)
	return;
    if (col == screen_cur_col && row == screen_cur_row)
	return;

    // Check for valid position.
    if (row < 0)	// window without text lines?
	row = 0;
    if (row >= screen_Rows)
	row = screen_Rows - 1;
    if (col >= screen_Columns)
	col = screen_Columns - 1;

    // check if no cursor movement is allowed in highlight mode
    if (screen_attr && *T_MS == NUL)
	noinvcurs = HIGHL_COST;
    else
	noinvcurs = 0;
    goto_cost = GOTO_COST + noinvcurs;

    /*
     * Plan how to do the positioning:
     * 1. Use CR to move it to column 0, same row.
     * 2. Use T_LE to move it a few columns to the left.
     * 3. Use NL to move a few lines down, column 0.
     * 4. Move a few columns to the right with T_ND or by writing chars.
     *
     * Don't do this if the cursor went beyond the last column, the cursor
     * position is unknown then (some terminals wrap, some don't )
     *
     * First check if the highlighting attributes allow us to write
     * characters to move the cursor to the right.
     */
    if (row >= screen_cur_row && screen_cur_col < Columns)
    {
	/*
	 * If the cursor is in the same row, bigger col, we can use CR
	 * or T_LE.
	 */
	bs = NULL;			    // init for GCC
	attr = screen_attr;
	if (row == screen_cur_row && col < screen_cur_col)
	{
	    // "le" is preferred over "bc", because "bc" is obsolete
	    if (*T_LE)
		bs = T_LE;		    // "cursor left"
	    else
		bs = T_BC;		    // "backspace character (old)
	    if (*bs)
		cost = (screen_cur_col - col) * (int)STRLEN(bs);
	    else
		cost = 999;
	    if (col + 1 < cost)	    // using CR is less characters
	    {
		plan = PLAN_CR;
		wouldbe_col = 0;
		cost = 1;		    // CR is just one character
	    }
	    else
	    {
		plan = PLAN_LE;
		wouldbe_col = col;
	    }
	    if (noinvcurs)		    // will stop highlighting
	    {
		cost += noinvcurs;
		attr = 0;
	    }
	}

	/*
	 * If the cursor is above where we want to be, we can use CR LF.
	 */
	else if (row > screen_cur_row)
	{
	    plan = PLAN_NL;
	    wouldbe_col = 0;
	    cost = (row - screen_cur_row) * 2;  // CR LF
	    if (noinvcurs)		    // will stop highlighting
	    {
		cost += noinvcurs;
		attr = 0;
	    }
	}

	/*
	 * If the cursor is in the same row, smaller col, just use write.
	 */
	else
	{
	    plan = PLAN_WRITE;
	    wouldbe_col = screen_cur_col;
	    cost = 0;
	}

	/*
	 * Check if any characters that need to be written have the
	 * correct attributes.  Also avoid UTF-8 characters.
	 */
	i = col - wouldbe_col;
	if (i > 0)
	    cost += i;
	if (cost < goto_cost && i > 0)
	{
	    /*
	     * Check if the attributes are correct without additionally
	     * stopping highlighting.
	     */
	    p = ScreenAttrs + LineOffset[row] + wouldbe_col;
	    while (i && *p++ == attr)
		--i;
	    if (i != 0)
	    {
		/*
		 * Try if it works when highlighting is stopped here.
		 */
		if (*--p == 0)
		{
		    cost += noinvcurs;
		    while (i && *p++ == 0)
			--i;
		}
		if (i != 0)
		    cost = 999;	// different attributes, don't do it
	    }
	    if (enc_utf8)
	    {
		// Don't use an UTF-8 char for positioning, it's slow.
		for (i = wouldbe_col; i < col; ++i)
		    if (ScreenLinesUC[LineOffset[row] + i] != 0)
		    {
			cost = 999;
			break;
		    }
	    }
	}

	/*
	 * We can do it without term_windgoto()!
	 */
	if (cost < goto_cost)
	{
	    if (plan == PLAN_LE)
	    {
		if (noinvcurs)
		    screen_stop_highlight();
		while (screen_cur_col > col)
		{
		    out_str(bs);
		    --screen_cur_col;
		}
	    }
	    else if (plan == PLAN_CR)
	    {
		if (noinvcurs)
		    screen_stop_highlight();
		out_char('\r');
		screen_cur_col = 0;
	    }
	    else if (plan == PLAN_NL)
	    {
		if (noinvcurs)
		    screen_stop_highlight();
		while (screen_cur_row < row)
		{
		    out_char('\n');
		    ++screen_cur_row;
		}
		screen_cur_col = 0;
	    }

	    i = col - screen_cur_col;
	    if (i > 0)
	    {
		/*
		 * Use cursor-right if it's one character only.  Avoids
		 * removing a line of pixels from the last bold char, when
		 * using the bold trick in the GUI.
		 */
		if (T_ND[0] != NUL && T_ND[1] == NUL)
		{
		    while (i-- > 0)
			out_char(*T_ND);
		}
		else
		{
		    int	off;

		    off = LineOffset[row] + screen_cur_col;
		    while (i-- > 0)
		    {
			if (ScreenAttrs[off] != screen_attr)
			    screen_stop_highlight();
			out_flush_check();
			out_char(ScreenLines[off]);
			if (enc_dbcs == DBCS_JPNU
				&& ScreenLines[off] == 0x8e)
			    out_char(ScreenLines2[off]);
			++off;
		    }
		}
	    }
	}
    }
    else
	cost = 999;

    if (cost >= goto_cost)
    {
	if (noinvcurs)
	    screen_stop_highlight();
	if (row == screen_cur_row && (col > screen_cur_col)
		&& *T_CRI != NUL)
	    term_cursor_right(col - screen_cur_col);
	else
	    term_windgoto(row, col);
    }
    screen_cur_row = row;
    screen_cur_col = col;
}

/*
 * Set cursor to its position in the current window.
 */
    void
setcursor(void)
{
    setcursor_mayforce(FALSE);
}

/*
 * Set cursor to its position in the current window.
 * When "force" is TRUE also when not redrawing.
 */
    void
setcursor_mayforce(int force)
{
    if (force || redrawing())
    {
	validate_cursor();
	windgoto(W_WINROW(curwin) + curwin->w_wrow,
		curwin->w_wincol + (
#ifdef FEAT_RIGHTLEFT
		// With 'rightleft' set and the cursor on a double-wide
		// character, position it on the leftmost column.
		curwin->w_p_rl ? ((int)curwin->w_width - curwin->w_wcol
		    - ((has_mbyte
			   && (*mb_ptr2cells)(ml_get_cursor()) == 2
			   && vim_isprintc(gchar_cursor())) ? 2 : 1)) :
#endif
							    curwin->w_wcol));
    }
}


/*
 * Insert 'line_count' lines at 'row' in window 'wp'.
 * If 'invalid' is TRUE the wp->w_lines[].wl_lnum is invalidated.
 * If 'mayclear' is TRUE the screen will be cleared if it is faster than
 * scrolling.
 * Returns FAIL if the lines are not inserted, OK for success.
 */
    int
win_ins_lines(
    win_T	*wp,
    int		row,
    int		line_count,
    int		invalid,
    int		mayclear)
{
    int		did_delete;
    int		nextrow;
    int		lastrow;
    int		retval;

    if (invalid)
	wp->w_lines_valid = 0;

    // with only a few lines it's not worth the effort
    if (wp->w_height < 5)
	return FAIL;

    // with the popup menu visible this might not work correctly
    if (pum_visible())
	return FAIL;

    if (line_count > wp->w_height - row)
	line_count = wp->w_height - row;

    retval = win_do_lines(wp, row, line_count, mayclear, FALSE, 0);
    if (retval != MAYBE)
	return retval;

    /*
     * If there is a next window or a status line, we first try to delete the
     * lines at the bottom to avoid messing what is after the window.
     * If this fails and there are following windows, don't do anything to
     * avoid messing up those windows, better just redraw.
     */
    did_delete = FALSE;
    if (wp->w_next != NULL || wp->w_status_height)
    {
	if (screen_del_lines(0, W_WINROW(wp) + wp->w_height - line_count,
				  line_count, (int)Rows, FALSE, 0, NULL) == OK)
	    did_delete = TRUE;
	else if (wp->w_next)
	    return FAIL;
    }
    /*
     * if no lines deleted, blank the lines that will end up below the window
     */
    if (!did_delete)
    {
	wp->w_redr_status = TRUE;
	redraw_cmdline = TRUE;
	nextrow = W_WINROW(wp) + wp->w_height + wp->w_status_height;
	lastrow = nextrow + line_count;
	if (lastrow > Rows)
	    lastrow = Rows;
	screen_fill(nextrow - line_count, lastrow - line_count,
		  wp->w_wincol, (int)W_ENDCOL(wp),
		  ' ', ' ', 0);
    }

    if (screen_ins_lines(0, W_WINROW(wp) + row, line_count, (int)Rows, 0, NULL)
								      == FAIL)
    {
	// deletion will have messed up other windows
	if (did_delete)
	{
	    wp->w_redr_status = TRUE;
	    win_rest_invalid(W_NEXT(wp));
	}
	return FAIL;
    }

    return OK;
}

/*
 * Delete "line_count" window lines at "row" in window "wp".
 * If "invalid" is TRUE curwin->w_lines[] is invalidated.
 * If "mayclear" is TRUE the screen will be cleared if it is faster than
 * scrolling
 * Return OK for success, FAIL if the lines are not deleted.
 */
    int
win_del_lines(
    win_T	*wp,
    int		row,
    int		line_count,
    int		invalid,
    int		mayclear,
    int		clear_attr)	    // for clearing lines
{
    int		retval;

    if (invalid)
	wp->w_lines_valid = 0;

    if (line_count > wp->w_height - row)
	line_count = wp->w_height - row;

    retval = win_do_lines(wp, row, line_count, mayclear, TRUE, clear_attr);
    if (retval != MAYBE)
	return retval;

    if (screen_del_lines(0, W_WINROW(wp) + row, line_count,
				   (int)Rows, FALSE, clear_attr, NULL) == FAIL)
	return FAIL;

    /*
     * If there are windows or status lines below, try to put them at the
     * correct place. If we can't do that, they have to be redrawn.
     */
    if (wp->w_next || wp->w_status_height || cmdline_row < Rows - 1)
    {
	if (screen_ins_lines(0, W_WINROW(wp) + wp->w_height - line_count,
			      line_count, (int)Rows, clear_attr, NULL) == FAIL)
	{
	    wp->w_redr_status = TRUE;
	    win_rest_invalid(wp->w_next);
	}
    }
    /*
     * If this is the last window and there is no status line, redraw the
     * command line later.
     */
    else
	redraw_cmdline = TRUE;
    return OK;
}

/*
 * Common code for win_ins_lines() and win_del_lines().
 * Returns OK or FAIL when the work has been done.
 * Returns MAYBE when not finished yet.
 */
    static int
win_do_lines(
    win_T	*wp,
    int		row,
    int		line_count,
    int		mayclear,
    int		del,
    int		clear_attr)
{
    int		retval;

    if (!redrawing() || line_count <= 0)
	return FAIL;

    // When inserting lines would result in loss of command output, just redraw
    // the lines.
    if (no_win_do_lines_ins && !del)
	return FAIL;

    // only a few lines left: redraw is faster
    if (mayclear && Rows - line_count < 5 && wp->w_width == Columns)
    {
	if (!no_win_do_lines_ins)
	    screenclear();	    // will set wp->w_lines_valid to 0
	return FAIL;
    }

#ifdef FEAT_PROP_POPUP
    // this doesn't work when there are popups visible
    if (popup_visible)
	return FAIL;
#endif

    // Delete all remaining lines
    if (row + line_count >= wp->w_height)
    {
	screen_fill(W_WINROW(wp) + row, W_WINROW(wp) + wp->w_height,
		wp->w_wincol, (int)W_ENDCOL(wp),
		' ', ' ', 0);
	return OK;
    }

    /*
     * When scrolling, the message on the command line should be cleared,
     * otherwise it will stay there forever.
     * Don't do this when avoiding to insert lines.
     */
    if (!no_win_do_lines_ins)
	clear_cmdline = TRUE;

    /*
     * If the terminal can set a scroll region, use that.
     * Always do this in a vertically split window.  This will redraw from
     * ScreenLines[] when t_CV isn't defined.  That's faster than using
     * win_line().
     * Don't use a scroll region when we are going to redraw the text, writing
     * a character in the lower right corner of the scroll region may cause a
     * scroll-up .
     */
    if (scroll_region || wp->w_width != Columns)
    {
	if (scroll_region && (wp->w_width == Columns || *T_CSV != NUL))
	    scroll_region_set(wp, row);
	if (del)
	    retval = screen_del_lines(W_WINROW(wp) + row, 0, line_count,
				    wp->w_height - row, FALSE, clear_attr, wp);
	else
	    retval = screen_ins_lines(W_WINROW(wp) + row, 0, line_count,
					   wp->w_height - row, clear_attr, wp);
	if (scroll_region && (wp->w_width == Columns || *T_CSV != NUL))
	    scroll_region_reset();
	return retval;
    }

    if (wp->w_next != NULL && p_tf) // don't delete/insert on fast terminal
	return FAIL;

    return MAYBE;
}

/*
 * window 'wp' and everything after it is messed up, mark it for redraw
 */
    static void
win_rest_invalid(win_T *wp)
{
    while (wp != NULL)
    {
	redraw_win_later(wp, UPD_NOT_VALID);
	wp->w_redr_status = TRUE;
	wp = wp->w_next;
    }
    redraw_cmdline = TRUE;
}

/*
 * The rest of the routines in this file perform screen manipulations. The
 * given operation is performed physically on the screen. The corresponding
 * change is also made to the internal screen image. In this way, the editor
 * anticipates the effect of editing changes on the appearance of the screen.
 * That way, when we call screenupdate a complete redraw isn't usually
 * necessary. Another advantage is that we can keep adding code to anticipate
 * screen changes, and in the meantime, everything still works.
 */

/*
 * types for inserting or deleting lines
 */
#define USE_T_CAL   1
#define USE_T_CDL   2
#define USE_T_AL    3
#define USE_T_CE    4
#define USE_T_DL    5
#define USE_T_SR    6
#define USE_NL	    7
#define USE_T_CD    8
#define USE_REDRAW  9

/*
 * insert lines on the screen and update ScreenLines[]
 * "end" is the line after the scrolled part. Normally it is Rows.
 * When scrolling region used "off" is the offset from the top for the region.
 * "row" and "end" are relative to the start of the region.
 *
 * return FAIL for failure, OK for success.
 */
    int
screen_ins_lines(
    int		off,
    int		row,
    int		line_count,
    int		end,
    int		clear_attr,
    win_T	*wp)	    // NULL or window to use width from
{
    int		i;
    int		j;
    unsigned	temp;
    int		cursor_row;
    int		cursor_col = 0;
    int		type;
    int		result_empty;
    int		can_ce = can_clear(T_CE);

    /*
     * FAIL if
     * - there is no valid screen
     * - the line count is less than one
     * - the line count is more than 'ttyscroll'
     * - "end" is more than "Rows" (safety check, should not happen)
     * - redrawing for a callback and there is a modeless selection
     * - there is a popup window
     */
     if (!screen_valid(TRUE)
	     || line_count <= 0 || line_count > p_ttyscroll
	     || end > Rows
#ifdef FEAT_CLIPBOARD
	     || (clip_star.state != SELECT_CLEARED
						 && redrawing_for_callback > 0)
#endif
#ifdef FEAT_PROP_POPUP
	     || popup_visible
#endif
	     )
	return FAIL;

    /*
     * There are seven ways to insert lines:
     * 0. When in a vertically split window and t_CV isn't set, redraw the
     *    characters from ScreenLines[].
     * 1. Use T_CD (clear to end of display) if it exists and the result of
     *	  the insert is just empty lines
     * 2. Use T_CAL (insert multiple lines) if it exists and T_AL is not
     *	  present or line_count > 1. It looks better if we do all the inserts
     *	  at once.
     * 3. Use T_CDL (delete multiple lines) if it exists and the result of the
     *	  insert is just empty lines and T_CE is not present or line_count >
     *	  1.
     * 4. Use T_AL (insert line) if it exists.
     * 5. Use T_CE (erase line) if it exists and the result of the insert is
     *	  just empty lines.
     * 6. Use T_DL (delete line) if it exists and the result of the insert is
     *	  just empty lines.
     * 7. Use T_SR (scroll reverse) if it exists and inserting at row 0 and
     *	  the 'da' flag is not set or we have clear line capability.
     * 8. redraw the characters from ScreenLines[].
     *
     * Careful: In a hpterm scroll reverse doesn't work as expected, it moves
     * the scrollbar for the window. It does have insert line, use that if it
     * exists.
     */
    result_empty = (row + line_count >= end);
    if (wp != NULL && wp->w_width != Columns && *T_CSV == NUL)
    {
	// Avoid that lines are first cleared here and then redrawn, which
	// results in many characters updated twice.  This happens with CTRL-F
	// in a vertically split window.  With line-by-line scrolling
	// USE_REDRAW should be faster.
	if (line_count > 3)
	    return FAIL;
	type = USE_REDRAW;
    }
    else if (can_clear(T_CD) && result_empty)
	type = USE_T_CD;
    else if (*T_CAL != NUL && (line_count > 1 || *T_AL == NUL))
	type = USE_T_CAL;
    else if (*T_CDL != NUL && result_empty && (line_count > 1 || !can_ce))
	type = USE_T_CDL;
    else if (*T_AL != NUL)
	type = USE_T_AL;
    else if (can_ce && result_empty)
	type = USE_T_CE;
    else if (*T_DL != NUL && result_empty)
	type = USE_T_DL;
    else if (*T_SR != NUL && row == 0 && (*T_DA == NUL || can_ce))
	type = USE_T_SR;
    else
	return FAIL;

    /*
     * For clearing the lines screen_del_lines() is used. This will also take
     * care of t_db if necessary.
     */
    if (type == USE_T_CD || type == USE_T_CDL ||
					 type == USE_T_CE || type == USE_T_DL)
	return screen_del_lines(off, row, line_count, end, FALSE, 0, wp);

    /*
     * If text is retained below the screen, first clear or delete as many
     * lines at the bottom of the window as are about to be inserted so that
     * the deleted lines won't later surface during a screen_del_lines.
     */
    if (*T_DB)
	screen_del_lines(off, end - line_count, line_count, end, FALSE, 0, wp);

#ifdef FEAT_CLIPBOARD
    // Remove a modeless selection when inserting lines halfway the screen
    // or not the full width of the screen.
    if (off + row > 0 || (wp != NULL && wp->w_width != Columns))
	clip_clear_selection(&clip_star);
    else
	clip_scroll_selection(-line_count);
#endif

#ifdef FEAT_GUI_HAIKU
    vim_lock_screen();
#endif

#ifdef FEAT_GUI
    // Don't update the GUI cursor here, ScreenLines[] is invalid until the
    // scrolling is actually carried out.
    gui_dont_update_cursor(row + off <= gui.cursor_row);
#endif

    if (wp != NULL && wp->w_wincol != 0 && *T_CSV != NUL && *T_CCS == NUL)
	cursor_col = wp->w_wincol;

    if (*T_CCS != NUL)	   // cursor relative to region
	cursor_row = row;
    else
	cursor_row = row + off;

    /*
     * Shift LineOffset[] line_count down to reflect the inserted lines.
     * Clear the inserted lines in ScreenLines[].
     */
    row += off;
    end += off;
    for (i = 0; i < line_count; ++i)
    {
	if (wp != NULL && wp->w_width != Columns)
	{
	    // need to copy part of a line
	    j = end - 1 - i;
	    while ((j -= line_count) >= row)
		linecopy(j + line_count, j, wp);
	    j += line_count;
	    if (can_clear((char_u *)" "))
		lineclear(LineOffset[j] + wp->w_wincol, wp->w_width,
								   clear_attr);
	    else
		lineinvalid(LineOffset[j] + wp->w_wincol, wp->w_width);
	    LineWraps[j] = FALSE;
	}
	else
	{
	    j = end - 1 - i;
	    temp = LineOffset[j];
	    while ((j -= line_count) >= row)
	    {
		LineOffset[j + line_count] = LineOffset[j];
		LineWraps[j + line_count] = LineWraps[j];
	    }
	    LineOffset[j + line_count] = temp;
	    LineWraps[j + line_count] = FALSE;
	    if (can_clear((char_u *)" "))
		lineclear(temp, (int)Columns, clear_attr);
	    else
		lineinvalid(temp, (int)Columns);
	}
    }

#ifdef FEAT_GUI_HAIKU
    vim_unlock_screen();
#endif

    screen_stop_highlight();
    windgoto(cursor_row, cursor_col);
    if (clear_attr != 0)
	screen_start_highlight(clear_attr);

    // redraw the characters
    if (type == USE_REDRAW)
	redraw_block(row, end, wp);
    else if (type == USE_T_CAL)
    {
	term_append_lines(line_count);
	screen_start();		// don't know where cursor is now
    }
    else
    {
	for (i = 0; i < line_count; i++)
	{
	    if (type == USE_T_AL)
	    {
		if (i && cursor_row != 0)
		    windgoto(cursor_row, cursor_col);
		out_str(T_AL);
	    }
	    else  // type == USE_T_SR
		out_str(T_SR);
	    screen_start();	    // don't know where cursor is now
	}
    }

    /*
     * With scroll-reverse and 'da' flag set we need to clear the lines that
     * have been scrolled down into the region.
     */
    if (type == USE_T_SR && *T_DA)
    {
	for (i = 0; i < line_count; ++i)
	{
	    windgoto(off + i, cursor_col);
	    out_str(T_CE);
	    screen_start();	    // don't know where cursor is now
	}
    }

#ifdef FEAT_GUI
    gui_can_update_cursor();
    if (gui.in_use)
	out_flush();	// always flush after a scroll
#endif
    return OK;
}

/*
 * Delete lines on the screen and update ScreenLines[].
 * "end" is the line after the scrolled part. Normally it is Rows.
 * When scrolling region used "off" is the offset from the top for the region.
 * "row" and "end" are relative to the start of the region.
 *
 * Return OK for success, FAIL if the lines are not deleted.
 */
    int
screen_del_lines(
    int		off,
    int		row,
    int		line_count,
    int		end,
    int		force,		// even when line_count > p_ttyscroll
    int		clear_attr,	// used for clearing lines
    win_T	*wp)		// NULL or window to use width from
{
    int		j;
    int		i;
    unsigned	temp;
    int		cursor_row;
    int		cursor_col = 0;
    int		cursor_end;
    int		result_empty;	// result is empty until end of region
    int		can_delete;	// deleting line codes can be used
    int		type;

    /*
     * FAIL if
     * - there is no valid screen
     * - the screen has to be redrawn completely
     * - the line count is less than one
     * - the line count is more than 'ttyscroll'
     * - "end" is more than "Rows" (safety check, should not happen)
     * - redrawing for a callback and there is a modeless selection
     */
    if (!screen_valid(TRUE)
	    || line_count <= 0
	    || (!force && line_count > p_ttyscroll)
	    || end > Rows
#ifdef FEAT_CLIPBOARD
	    || (clip_star.state != SELECT_CLEARED && redrawing_for_callback > 0)
#endif
       )
	return FAIL;

    /*
     * Check if the rest of the current region will become empty.
     */
    result_empty = row + line_count >= end;

    /*
     * We can delete lines only when 'db' flag not set or when 'ce' option
     * available.
     */
    can_delete = (*T_DB == NUL || can_clear(T_CE));

    /*
     * There are six ways to delete lines:
     * 0. When in a vertically split window and t_CV isn't set, redraw the
     *    characters from ScreenLines[].
     * 1. Use T_CD if it exists and the result is empty.
     * 2. Use newlines if row == 0 and count == 1 or T_CDL does not exist.
     * 3. Use T_CDL (delete multiple lines) if it exists and line_count > 1 or
     *	  none of the other ways work.
     * 4. Use T_CE (erase line) if the result is empty.
     * 5. Use T_DL (delete line) if it exists.
     * 6. redraw the characters from ScreenLines[].
     */
    if (wp != NULL && wp->w_width != Columns && *T_CSV == NUL)
    {
	// Avoid that lines are first cleared here and then redrawn, which
	// results in many characters updated twice.  This happens with CTRL-F
	// in a vertically split window.  With line-by-line scrolling
	// USE_REDRAW should be faster.
	if (line_count > 3)
	    return FAIL;
	type = USE_REDRAW;
    }
    else if (can_clear(T_CD) && result_empty)
	type = USE_T_CD;
    else if (row == 0 && (
#ifndef AMIGA
	// On the Amiga, somehow '\n' on the last line doesn't always scroll
	// up, so use delete-line command
			    line_count == 1 ||
#endif
						*T_CDL == NUL))
	type = USE_NL;
    else if (*T_CDL != NUL && line_count > 1 && can_delete)
	type = USE_T_CDL;
    else if (can_clear(T_CE) && result_empty
	    && (wp == NULL || wp->w_width == Columns))
	type = USE_T_CE;
    else if (*T_DL != NUL && can_delete)
	type = USE_T_DL;
    else if (*T_CDL != NUL && can_delete)
	type = USE_T_CDL;
    else
	return FAIL;

#ifdef FEAT_CLIPBOARD
    // Remove a modeless selection when deleting lines halfway the screen or
    // not the full width of the screen.
    if (off + row > 0 || (wp != NULL && wp->w_width != Columns))
	clip_clear_selection(&clip_star);
    else
	clip_scroll_selection(line_count);
#endif

#ifdef FEAT_GUI_HAIKU
    vim_lock_screen();
#endif

#ifdef FEAT_GUI
    // Don't update the GUI cursor here, ScreenLines[] is invalid until the
    // scrolling is actually carried out.
    gui_dont_update_cursor(gui.cursor_row >= row + off
						&& gui.cursor_row < end + off);
#endif

    if (wp != NULL && wp->w_wincol != 0 && *T_CSV != NUL && *T_CCS == NUL)
	cursor_col = wp->w_wincol;

    if (*T_CCS != NUL)	    // cursor relative to region
    {
	cursor_row = row;
	cursor_end = end;
    }
    else
    {
	cursor_row = row + off;
	cursor_end = end + off;
    }

    /*
     * Now shift LineOffset[] line_count up to reflect the deleted lines.
     * Clear the inserted lines in ScreenLines[].
     */
    row += off;
    end += off;
    for (i = 0; i < line_count; ++i)
    {
	if (wp != NULL && wp->w_width != Columns)
	{
	    // need to copy part of a line
	    j = row + i;
	    while ((j += line_count) <= end - 1)
		linecopy(j - line_count, j, wp);
	    j -= line_count;
	    if (can_clear((char_u *)" "))
		lineclear(LineOffset[j] + wp->w_wincol, wp->w_width,
								   clear_attr);
	    else
		lineinvalid(LineOffset[j] + wp->w_wincol, wp->w_width);
	    LineWraps[j] = FALSE;
	}
	else
	{
	    // whole width, moving the line pointers is faster
	    j = row + i;
	    temp = LineOffset[j];
	    while ((j += line_count) <= end - 1)
	    {
		LineOffset[j - line_count] = LineOffset[j];
		LineWraps[j - line_count] = LineWraps[j];
	    }
	    LineOffset[j - line_count] = temp;
	    LineWraps[j - line_count] = FALSE;
	    if (can_clear((char_u *)" "))
		lineclear(temp, (int)Columns, clear_attr);
	    else
		lineinvalid(temp, (int)Columns);
	}
    }

#ifdef FEAT_GUI_HAIKU
    vim_unlock_screen();
#endif

    if (screen_attr != clear_attr)
	screen_stop_highlight();
    if (clear_attr != 0)
	screen_start_highlight(clear_attr);

    // redraw the characters
    if (type == USE_REDRAW)
	redraw_block(row, end, wp);
    else if (type == USE_T_CD)	// delete the lines
    {
	windgoto(cursor_row, cursor_col);
	out_str(T_CD);
	screen_start();			// don't know where cursor is now
    }
    else if (type == USE_T_CDL)
    {
	windgoto(cursor_row, cursor_col);
	term_delete_lines(line_count);
	screen_start();			// don't know where cursor is now
    }
    /*
     * Deleting lines at top of the screen or scroll region: Just scroll
     * the whole screen (scroll region) up by outputting newlines on the
     * last line.
     */
    else if (type == USE_NL)
    {
	windgoto(cursor_end - 1, cursor_col);
	for (i = line_count; --i >= 0; )
	    out_char('\n');		// cursor will remain on same line
    }
    else
    {
	for (i = line_count; --i >= 0; )
	{
	    if (type == USE_T_DL)
	    {
		windgoto(cursor_row, cursor_col);
		out_str(T_DL);		// delete a line
	    }
	    else // type == USE_T_CE
	    {
		windgoto(cursor_row + i, cursor_col);
		out_str(T_CE);		// erase a line
	    }
	    screen_start();		// don't know where cursor is now
	}
    }

    /*
     * If the 'db' flag is set, we need to clear the lines that have been
     * scrolled up at the bottom of the region.
     */
    if (*T_DB && (type == USE_T_DL || type == USE_T_CDL))
    {
	for (i = line_count; i > 0; --i)
	{
	    windgoto(cursor_end - i, cursor_col);
	    out_str(T_CE);		// erase a line
	    screen_start();		// don't know where cursor is now
	}
    }

#ifdef FEAT_GUI
    gui_can_update_cursor();
    if (gui.in_use)
	out_flush();	// always flush after a scroll
#endif

    return OK;
}

/*
 * Return TRUE when postponing displaying the mode message: when not redrawing
 * or inside a mapping.
 */
    int
skip_showmode(void)
{
    // Call char_avail() only when we are going to show something, because it
    // takes a bit of time.  redrawing() may also call char_avail().
    if (global_busy
	    || msg_silent != 0
	    || !redrawing()
	    || (char_avail() && !KeyTyped))
    {
	redraw_mode = TRUE;		// show mode later
	return TRUE;
    }
    return FALSE;
}

/*
 * Show the current mode and ruler.
 *
 * If clear_cmdline is TRUE, clear the rest of the cmdline.
 * If clear_cmdline is FALSE there may be a message there that needs to be
 * cleared only if a mode is shown.
 * If redraw_mode is TRUE show or clear the mode.
 * Return the length of the message (0 if no message).
 */
    int
showmode(void)
{
    int		need_clear;
    int		length = 0;
    int		do_mode;
    int		attr;
    int		nwr_save;
    int		sub_attr;

    do_mode = p_smd && msg_silent == 0
	    && ((State & MODE_INSERT)
		|| restart_edit != NUL
		|| VIsual_active);
    if (do_mode || reg_recording != 0)
    {
	if (skip_showmode())
	    return 0;		// show mode later

	nwr_save = need_wait_return;

	// wait a bit before overwriting an important message
	check_for_delay(FALSE);

	// if the cmdline is more than one line high, erase top lines
	need_clear = clear_cmdline;
	if (clear_cmdline && cmdline_row < Rows - 1)
	    msg_clr_cmdline();			// will reset clear_cmdline

	// Position on the last line in the window, column 0
	msg_pos_mode();
	cursor_off();
	attr = HL_ATTR(HLF_CM);			// Highlight mode
	if (do_mode)
	{
	    msg_puts_attr("--", attr);
#if defined(FEAT_XIM)
	    if (
# ifdef FEAT_GUI_GTK
		    preedit_get_status()
# else
		    im_get_status()
# endif
	       )
# ifdef FEAT_GUI_GTK // most of the time, it's not XIM being used
		msg_puts_attr(" IM", attr);
# else
		msg_puts_attr(" XIM", attr);
# endif
#endif
	    // CTRL-X in Insert mode
	    if (edit_submode != NULL && !shortmess(SHM_COMPLETIONMENU))
	    {
		// These messages can get long, avoid a wrap in a narrow
		// window.  Prefer showing edit_submode_extra.
		length = (Rows - msg_row) * Columns - 3;
		if (edit_submode_extra != NULL)
		    length -= vim_strsize(edit_submode_extra);
		if (length > 0)
		{
		    if (edit_submode_pre != NULL)
			length -= vim_strsize(edit_submode_pre);
		    if (length - vim_strsize(edit_submode) > 0)
		    {
			if (edit_submode_pre != NULL)
			    msg_puts_attr((char *)edit_submode_pre, attr);
			msg_puts_attr((char *)edit_submode, attr);
		    }
		    if (edit_submode_extra != NULL)
		    {
			msg_puts_attr(" ", attr);  // add a space in between
			if ((int)edit_submode_highl < (int)HLF_COUNT)
			    sub_attr = HL_ATTR(edit_submode_highl);
			else
			    sub_attr = attr;
			msg_puts_attr((char *)edit_submode_extra, sub_attr);
		    }
		}
	    }
	    else
	    {
		if (State & VREPLACE_FLAG)
		    msg_puts_attr(_(" VREPLACE"), attr);
		else if (State & REPLACE_FLAG)
		    msg_puts_attr(_(" REPLACE"), attr);
		else if (State & MODE_INSERT)
		{
#ifdef FEAT_RIGHTLEFT
		    if (p_ri)
			msg_puts_attr(_(" REVERSE"), attr);
#endif
		    msg_puts_attr(_(" INSERT"), attr);
		}
		else if (restart_edit == 'I' || restart_edit == 'i' ||
			restart_edit == 'a' || restart_edit == 'A')
		    msg_puts_attr(_(" (insert)"), attr);
		else if (restart_edit == 'R')
		    msg_puts_attr(_(" (replace)"), attr);
		else if (restart_edit == 'V')
		    msg_puts_attr(_(" (vreplace)"), attr);
#ifdef FEAT_RIGHTLEFT
		if (p_hkmap)
		    msg_puts_attr(_(" Hebrew"), attr);
#endif
#ifdef FEAT_KEYMAP
		if (State & MODE_LANGMAP)
		{
# ifdef FEAT_ARABIC
		    if (curwin->w_p_arab)
			msg_puts_attr(_(" Arabic"), attr);
		    else
# endif
			if (get_keymap_str(curwin, (char_u *)" (%s)",
							   NameBuff, MAXPATHL))
			    msg_puts_attr((char *)NameBuff, attr);
		}
#endif
		if ((State & MODE_INSERT) && p_paste)
		    msg_puts_attr(_(" (paste)"), attr);

		if (VIsual_active)
		{
		    char *p;

		    // Don't concatenate separate words to avoid translation
		    // problems.
		    switch ((VIsual_select ? 4 : 0)
			    + (VIsual_mode == Ctrl_V) * 2
			    + (VIsual_mode == 'V'))
		    {
			case 0:	p = N_(" VISUAL"); break;
			case 1: p = N_(" VISUAL LINE"); break;
			case 2: p = N_(" VISUAL BLOCK"); break;
			case 4: p = N_(" SELECT"); break;
			case 5: p = N_(" SELECT LINE"); break;
			default: p = N_(" SELECT BLOCK"); break;
		    }
		    msg_puts_attr(_(p), attr);
		}
		msg_puts_attr(" --", attr);
	    }

	    need_clear = TRUE;
	}
	if (reg_recording != 0
		&& edit_submode == NULL)    // otherwise it gets too long
	{
	    recording_mode(attr);
	    need_clear = TRUE;
	}

	mode_displayed = TRUE;
	if (need_clear || clear_cmdline || redraw_mode)
	    msg_clr_eos();
	msg_didout = FALSE;		// overwrite this message
	length = msg_col;
	msg_col = 0;
	need_wait_return = nwr_save;	// never ask for hit-return for this
    }
    else if (clear_cmdline && msg_silent == 0)
	// Clear the whole command line.  Will reset "clear_cmdline".
	msg_clr_cmdline();
    else if (redraw_mode)
    {
	msg_pos_mode();
	msg_clr_eos();
    }

    // In Visual mode the size of the selected area must be redrawn.
    if (VIsual_active)
	clear_showcmd();

    // If the last window has no status line, the ruler is after the mode
    // message and must be redrawn
    if (redrawing() && lastwin->w_status_height == 0)
	win_redr_ruler(lastwin, TRUE, FALSE);

    redraw_cmdline = FALSE;
    redraw_mode = FALSE;
    clear_cmdline = FALSE;

    return length;
}

/*
 * Position for a mode message.
 */
    static void
msg_pos_mode(void)
{
    msg_col = 0;
    msg_row = Rows - 1;
}

/*
 * Delete mode message.  Used when ESC is typed which is expected to end
 * Insert mode (but Insert mode didn't end yet!).
 * Caller should check "mode_displayed".
 */
    void
unshowmode(int force)
{
    /*
     * Don't delete it right now, when not redrawing or inside a mapping.
     */
    if (!redrawing() || (!force && char_avail() && !KeyTyped))
	redraw_cmdline = TRUE;		// delete mode later
    else
	clearmode();
}

/*
 * Clear the mode message.
 */
    void
clearmode(void)
{
    int save_msg_row = msg_row;
    int save_msg_col = msg_col;

    msg_pos_mode();
    if (reg_recording != 0)
	recording_mode(HL_ATTR(HLF_CM));
    msg_clr_eos();

    msg_col = save_msg_col;
    msg_row = save_msg_row;
}

    static void
recording_mode(int attr)
{
    msg_puts_attr(_("recording"), attr);
    if (shortmess(SHM_RECORDING))
	return;

    char s[4];

    sprintf(s, " @%c", reg_recording);
    msg_puts_attr(s, attr);
}

/*
 * Draw the tab pages line at the top of the Vim window.
 */
    void
draw_tabline(void)
{
    int		tabcount = 0;
    tabpage_T	*tp;
    int		tabwidth;
    int		col = 0;
    int		scol = 0;
    int		attr;
    win_T	*wp;
    win_T	*cwp;
    int		wincount;
    int		modified;
    int		c;
    int		len;
    int		attr_sel = HL_ATTR(HLF_TPS);
    int		attr_nosel = HL_ATTR(HLF_TP);
    int		attr_fill = HL_ATTR(HLF_TPF);
    char_u	*p;
    int		room;
    int		use_sep_chars = (t_colors < 8
#ifdef FEAT_GUI
					    && !gui.in_use
#endif
#ifdef FEAT_TERMGUICOLORS
					    && !p_tgc
#endif
					    );

    if (ScreenLines == NULL)
	return;
    redraw_tabline = FALSE;

#ifdef FEAT_GUI_TABLINE
    // Take care of a GUI tabline.
    if (gui_use_tabline())
    {
	gui_update_tabline();
	return;
    }
#endif

    if (tabline_height() < 1)
	return;

#if defined(FEAT_STL_OPT)
    clear_TabPageIdxs();

    // Use the 'tabline' option if it's set.
    if (*p_tal != NUL)
	win_redr_custom(NULL, FALSE);
    else
#endif
    {
	FOR_ALL_TABPAGES(tp)
	    ++tabcount;

	tabwidth = (Columns - 1 + tabcount / 2) / tabcount;
	if (tabwidth < 6)
	    tabwidth = 6;

	attr = attr_nosel;
	tabcount = 0;
	for (tp = first_tabpage; tp != NULL && col < Columns - 4;
							     tp = tp->tp_next)
	{
	    scol = col;

	    if (tp->tp_topframe == topframe)
		attr = attr_sel;
	    if (use_sep_chars && col > 0)
		screen_putchar('|', 0, col++, attr);

	    if (tp->tp_topframe != topframe)
		attr = attr_nosel;

	    screen_putchar(' ', 0, col++, attr);

	    if (tp == curtab)
	    {
		cwp = curwin;
		wp = firstwin;
	    }
	    else
	    {
		cwp = tp->tp_curwin;
		wp = tp->tp_firstwin;
	    }

	    modified = FALSE;
	    for (wincount = 0; wp != NULL; wp = wp->w_next, ++wincount)
		if (bufIsChanged(wp->w_buffer))
		    modified = TRUE;
	    if (modified || wincount > 1)
	    {
		if (wincount > 1)
		{
		    vim_snprintf((char *)NameBuff, MAXPATHL, "%d", wincount);
		    len = (int)STRLEN(NameBuff);
		    if (col + len >= Columns - 3)
			break;
		    screen_puts_len(NameBuff, len, 0, col,
#if defined(FEAT_SYN_HL)
					 hl_combine_attr(attr, HL_ATTR(HLF_T))
#else
					 attr
#endif
					       );
		    col += len;
		}
		if (modified)
		    screen_puts_len((char_u *)"+", 1, 0, col++, attr);
		screen_putchar(' ', 0, col++, attr);
	    }

	    room = scol - col + tabwidth - 1;
	    if (room > 0)
	    {
		// Get buffer name in NameBuff[]
		get_trans_bufname(cwp->w_buffer);
		shorten_dir(NameBuff);
		len = vim_strsize(NameBuff);
		p = NameBuff;
		if (has_mbyte)
		    while (len > room)
		    {
			len -= ptr2cells(p);
			MB_PTR_ADV(p);
		    }
		else if (len > room)
		{
		    p += len - room;
		    len = room;
		}
		if (len > Columns - col - 1)
		    len = Columns - col - 1;

		screen_puts_len(p, (int)STRLEN(p), 0, col, attr);
		col += len;
	    }
	    screen_putchar(' ', 0, col++, attr);

	    // Store the tab page number in TabPageIdxs[], so that
	    // jump_to_mouse() knows where each one is.
	    ++tabcount;
	    while (scol < col)
		TabPageIdxs[scol++] = tabcount;
	}

	if (use_sep_chars)
	    c = '_';
	else
	    c = ' ';
	screen_fill(0, 1, col, (int)Columns, c, c, attr_fill);

	// Draw the 'showcmd' information if 'showcmdloc' == "tabline".
	if (p_sc && *p_sloc == 't')
	{
	    int	width = MIN(10, (int)Columns - col - (tabcount > 1) * 3);

	    if (width > 0)
		screen_puts_len(showcmd_buf, width, 0, (int)Columns
			    - width - (tabcount > 1) * 2, attr_nosel);
	}

	// Put an "X" for closing the current tab if there are several.
	if (tabcount > 1)
	{
	    screen_putchar('X', 0, (int)Columns - 1, attr_nosel);
	    TabPageIdxs[Columns - 1] = -999;
	}
    }

    // Reset the flag here again, in case evaluating 'tabline' causes it to be
    // set.
    redraw_tabline = FALSE;
}

/*
 * Get buffer name for "buf" into NameBuff[].
 * Takes care of special buffer names and translates special characters.
 */
    void
get_trans_bufname(buf_T *buf)
{
    if (buf_spname(buf) != NULL)
	vim_strncpy(NameBuff, buf_spname(buf), MAXPATHL - 1);
    else
	home_replace(buf, buf->b_fname, NameBuff, MAXPATHL, TRUE);
    trans_characters(NameBuff, MAXPATHL);
}

/*
 * Get the character to use in a status line.  Get its attributes in "*attr".
 */
    int
fillchar_status(int *attr, win_T *wp)
{
    int fill;

#ifdef FEAT_TERMINAL
    if (bt_terminal(wp->w_buffer))
    {
	if (wp == curwin)
	{
	    *attr = HL_ATTR(HLF_ST);
	    fill = wp->w_fill_chars.stl;
	}
	else
	{
	    *attr = HL_ATTR(HLF_STNC);
	    fill = wp->w_fill_chars.stlnc;
	}
    }
    else
#endif
    if (wp == curwin)
    {
	*attr = HL_ATTR(HLF_S);
	fill = wp->w_fill_chars.stl;
    }
    else
    {
	*attr = HL_ATTR(HLF_SNC);
	fill = wp->w_fill_chars.stlnc;
    }
    return fill;
}

/*
 * Get the character to use in a separator between vertically split windows.
 * Get its attributes in "*attr".
 */
    int
fillchar_vsep(int *attr, win_T *wp)
{
    *attr = HL_ATTR(HLF_C);
    if (*attr == 0 && wp->w_fill_chars.vert == ' ')
	return '|';
    else
	return wp->w_fill_chars.vert;
}

/*
 * Return TRUE if redrawing should currently be done.
 */
    int
redrawing(void)
{
#ifdef FEAT_EVAL
    if (disable_redraw_for_testing)
	return 0;
    else
#endif
	return ((RedrawingDisabled == 0
#ifdef FEAT_EVAL
		    || ignore_redraw_flag_for_testing
#endif
		) && !(p_lz && char_avail() && !KeyTyped && !do_redraw));
}

/*
 * Return TRUE if printing messages should currently be done.
 */
    int
messaging(void)
{
    return (!(p_lz && char_avail() && !KeyTyped));
}

/*
 * Compute columns for ruler and shown command. 'sc_col' is also used to
 * decide what the maximum length of a message on the status line can be.
 * If there is a status line for the last window, 'sc_col' is independent
 * of 'ru_col'.
 */

#define COL_RULER 17	    // columns needed by standard ruler

    void
comp_col(void)
{
    int last_has_status = last_stl_height(FALSE) > 0;

    sc_col = 0;
    ru_col = 0;
    if (p_ru)
    {
#ifdef FEAT_STL_OPT
	ru_col = (ru_wid ? ru_wid : COL_RULER) + 1;
#else
	ru_col = COL_RULER + 1;
#endif
	// no last status line, adjust sc_col
	if (!last_has_status)
	    sc_col = ru_col;
    }
    if (p_sc)
    {
	sc_col += SHOWCMD_COLS;
	if (!p_ru || last_has_status)	    // no need for separating space
	    ++sc_col;
    }
    sc_col = Columns - sc_col;
    ru_col = Columns - ru_col;
    if (sc_col <= 0)		// screen too narrow, will become a mess
	sc_col = 1;
    if (ru_col <= 0)
	ru_col = 1;
#ifdef FEAT_EVAL
    set_vim_var_nr(VV_ECHOSPACE, sc_col - 1);
#endif
}

#if defined(FEAT_LINEBREAK) || defined(PROTO)
/*
 * Return the width of the 'number' and 'relativenumber' column.
 * Caller may need to check if 'number' or 'relativenumber' is set.
 * Otherwise it depends on 'numberwidth' and the line count.
 */
    int
number_width(win_T *wp)
{
    int		n;
    linenr_T	lnum;

    if (wp->w_p_rnu && !wp->w_p_nu)
	// cursor line shows "0"
	lnum = wp->w_height;
    else
	// cursor line shows absolute line number
	lnum = wp->w_buffer->b_ml.ml_line_count;

    if (lnum == wp->w_nrwidth_line_count && wp->w_nuw_cached == wp->w_p_nuw)
	return wp->w_nrwidth_width;
    wp->w_nrwidth_line_count = lnum;

    n = 0;
    do
    {
	lnum /= 10;
	++n;
    } while (lnum > 0);

    // 'numberwidth' gives the minimal width plus one
    if (n < wp->w_p_nuw - 1)
	n = wp->w_p_nuw - 1;

# ifdef FEAT_SIGNS
    // If 'signcolumn' is set to 'number' and there is a sign to display, then
    // the minimal width for the number column is 2.
    if (n < 2 && get_first_valid_sign(wp) != NULL
	    && (*wp->w_p_scl == 'n' && *(wp->w_p_scl + 1) == 'u'))
	n = 2;
# endif

    wp->w_nrwidth_width = n;
    wp->w_nuw_cached = wp->w_p_nuw;
    return n;
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return the current cursor column. This is the actual position on the
 * screen. First column is 0.
 */
    int
screen_screencol(void)
{
    return screen_cur_col;
}

/*
 * Return the current cursor row. This is the actual position on the screen.
 * First row is 0.
 */
    int
screen_screenrow(void)
{
    return screen_cur_row;
}
#endif

/*
 * Calls mb_ptr2char_adv(p) and returns the character.
 * If "p" starts with "\x", "\u" or "\U" the hex or unicode value is used.
 */
    static int
get_encoded_char_adv(char_u **p)
{
    char_u *s = *p;

    if (s[0] == '\\' && (s[1] == 'x' || s[1] == 'u' || s[1] == 'U'))
    {
	varnumber_T num = 0;
	int	    bytes;
	int	    n;

	for (bytes = s[1] == 'x' ? 1 : s[1] == 'u' ? 2 : 4; bytes > 0; --bytes)
	{
	    *p += 2;
	    n = hexhex2nr(*p);
	    if (n < 0)
		return 0;
	    num = num * 256 + n;
	}
	*p += 2;
	return num;
    }
    return mb_ptr2char_adv(p);
}

struct charstab
{
    int	    *cp;
    char    *name;
};
static fill_chars_T fill_chars;
static struct charstab filltab[] =
{
    {&fill_chars.stl,		"stl"},
    {&fill_chars.stlnc,		"stlnc"},
    {&fill_chars.vert,		"vert"},
    {&fill_chars.fold,		"fold"},
    {&fill_chars.foldopen,	"foldopen"},
    {&fill_chars.foldclosed,	"foldclose"},
    {&fill_chars.foldsep,	"foldsep"},
    {&fill_chars.diff,		"diff"},
    {&fill_chars.eob,		"eob"},
    {&fill_chars.lastline,	"lastline"},
};
static lcs_chars_T lcs_chars;
static struct charstab lcstab[] =
{
    {&lcs_chars.eol,		"eol"},
    {&lcs_chars.ext,		"extends"},
    {&lcs_chars.nbsp,		"nbsp"},
    {&lcs_chars.prec,		"precedes"},
    {&lcs_chars.space,		"space"},
    {&lcs_chars.tab2,		"tab"},
    {&lcs_chars.trail,		"trail"},
    {&lcs_chars.lead,		"lead"},
#ifdef FEAT_CONCEAL
    {&lcs_chars.conceal,	"conceal"},
#else
    {NULL,			"conceal"},
#endif
    {NULL,			"multispace"},
    {NULL,			"leadmultispace"},
};

    static char *
field_value_err(char *errbuf, size_t errbuflen, char *fmt, char *field)
{
    if (errbuf == NULL)
	return "";
    vim_snprintf(errbuf, errbuflen, _(fmt), field);
    return errbuf;
}

/*
 * Handle setting 'listchars' or 'fillchars'.
 * "value" points to either the global or the window-local value.
 * "is_listchars" is TRUE for "listchars" and FALSE for "fillchars".
 * When "apply" is FALSE do not store the flags, only check for errors.
 * Assume monocell characters.
 * Returns error message, NULL if it's OK.
 */
    static char *
set_chars_option(win_T *wp, char_u *value, int is_listchars, int apply,
						char *errbuf, size_t errbuflen)
{
    int	    round, i, len, entries;
    char_u  *p, *s;
    int	    c1 = 0, c2 = 0, c3 = 0;
    char_u  *last_multispace = NULL;  // Last occurrence of "multispace:"
    char_u  *last_lmultispace = NULL; // Last occurrence of "leadmultispace:"
    int	    multispace_len = 0;	      // Length of lcs-multispace string
    int	    lead_multispace_len = 0;  // Length of lcs-leadmultispace string

    struct charstab *tab;

    if (is_listchars)
    {
	tab = lcstab;
	CLEAR_FIELD(lcs_chars);
	entries = ARRAY_LENGTH(lcstab);
	if (wp->w_p_lcs[0] == NUL)
	    value = p_lcs;  // local value is empty, use the global value
    }
    else
    {
	tab = filltab;
	entries = ARRAY_LENGTH(filltab);
	if (wp->w_p_fcs[0] == NUL)
	    value = p_fcs;  // local value is empty, us the global value
    }

    // first round: check for valid value, second round: assign values
    for (round = 0; round <= (apply ? 1 : 0); ++round)
    {
	if (round > 0)
	{
	    // After checking that the value is valid: set defaults.
	    if (is_listchars)
	    {
		for (i = 0; i < entries; ++i)
		    if (tab[i].cp != NULL)
			*(tab[i].cp) = NUL;
		lcs_chars.tab1 = NUL;
		lcs_chars.tab3 = NUL;

		if (multispace_len > 0)
		{
		    lcs_chars.multispace = ALLOC_MULT(int, multispace_len + 1);
		    if (lcs_chars.multispace != NULL)
			lcs_chars.multispace[multispace_len] = NUL;
		}
		else
		    lcs_chars.multispace = NULL;

		if (lead_multispace_len > 0)
		{
		    lcs_chars.leadmultispace =
				      ALLOC_MULT(int, lead_multispace_len + 1);
		    lcs_chars.leadmultispace[lead_multispace_len] = NUL;
		}
		else
		    lcs_chars.leadmultispace = NULL;
	    }
	    else
	    {
		fill_chars.stl = ' ';
		fill_chars.stlnc = ' ';
		fill_chars.vert = ' ';
		fill_chars.fold = '-';
		fill_chars.foldopen = '-';
		fill_chars.foldclosed = '+';
		fill_chars.foldsep = '|';
		fill_chars.diff = '-';
		fill_chars.eob = '~';
		fill_chars.lastline = '@';
	    }
	}
	p = value;
	while (*p)
	{
	    for (i = 0; i < entries; ++i)
	    {
		len = (int)STRLEN(tab[i].name);
		if (!(STRNCMP(p, tab[i].name, len) == 0 && p[len] == ':'))
		    continue;

		if (is_listchars && strcmp(tab[i].name, "multispace") == 0)
		{
		    s = p + len + 1;
		    if (round == 0)
		    {
			// Get length of lcs-multispace string in first round
			last_multispace = p;
			multispace_len = 0;
			while (*s != NUL && *s != ',')
			{
			    c1 = get_encoded_char_adv(&s);
			    if (char2cells(c1) > 1)
				return field_value_err(errbuf, errbuflen,
					 e_wrong_character_width_for_field_str,
					 tab[i].name);
			    ++multispace_len;
			}
			if (multispace_len == 0)
			    // lcs-multispace cannot be an empty string
			    return field_value_err(errbuf, errbuflen,
				    e_wrong_number_of_characters_for_field_str,
				    tab[i].name);
			p = s;
		    }
		    else
		    {
			int multispace_pos = 0;

			while (*s != NUL && *s != ',')
			{
			    c1 = get_encoded_char_adv(&s);
			    if (p == last_multispace)
				lcs_chars.multispace[multispace_pos++] = c1;
			}
			p = s;
		    }
		    break;
		}

		if (is_listchars && strcmp(tab[i].name, "leadmultispace") == 0)
		{
		    s = p + len + 1;
		    if (round == 0)
		    {
			// get length of lcs-leadmultispace string in first
			// round
			last_lmultispace = p;
			lead_multispace_len = 0;
			while (*s != NUL && *s != ',')
			{
			    c1 = get_encoded_char_adv(&s);
			    if (char2cells(c1) > 1)
				return field_value_err(errbuf, errbuflen,
					 e_wrong_character_width_for_field_str,
					 tab[i].name);
			    ++lead_multispace_len;
			}
			if (lead_multispace_len == 0)
			    // lcs-leadmultispace cannot be an empty string
			    return field_value_err(errbuf, errbuflen,
				    e_wrong_number_of_characters_for_field_str,
				    tab[i].name);
			p = s;
		    }
		    else
		    {
			int multispace_pos = 0;

			while (*s != NUL && *s != ',')
			{
			    c1 = get_encoded_char_adv(&s);
			    if (p == last_lmultispace)
				lcs_chars.leadmultispace[multispace_pos++] = c1;
			}
			p = s;
		    }
		    break;
		}

		c2 = c3 = 0;
		s = p + len + 1;
		if (*s == NUL)
		    return field_value_err(errbuf, errbuflen,
				    e_wrong_number_of_characters_for_field_str,
				    tab[i].name);
		c1 = get_encoded_char_adv(&s);
		if (char2cells(c1) > 1)
		    return field_value_err(errbuf, errbuflen,
					 e_wrong_character_width_for_field_str,
					 tab[i].name);
		if (tab[i].cp == &lcs_chars.tab2)
		{
		    if (*s == NUL)
			return field_value_err(errbuf, errbuflen,
				    e_wrong_number_of_characters_for_field_str,
				    tab[i].name);
		    c2 = get_encoded_char_adv(&s);
		    if (char2cells(c2) > 1)
			return field_value_err(errbuf, errbuflen,
					 e_wrong_character_width_for_field_str,
					 tab[i].name);
		    if (!(*s == ',' || *s == NUL))
		    {
			c3 = get_encoded_char_adv(&s);
			if (char2cells(c3) > 1)
			    return field_value_err(errbuf, errbuflen,
					 e_wrong_character_width_for_field_str,
					 tab[i].name);
		    }
		}

		if (*s == ',' || *s == NUL)
		{
		    if (round > 0)
		    {
			if (tab[i].cp == &lcs_chars.tab2)
			{
			    lcs_chars.tab1 = c1;
			    lcs_chars.tab2 = c2;
			    lcs_chars.tab3 = c3;
			}
			else if (tab[i].cp != NULL)
			    *(tab[i].cp) = c1;

		    }
		    p = s;
		    break;
		}
		else
		    return field_value_err(errbuf, errbuflen,
				    e_wrong_number_of_characters_for_field_str,
				    tab[i].name);
	    }

	    if (i == entries)
		return e_invalid_argument;

	    if (*p == ',')
		++p;
	}
    }

    if (apply)
    {
	if (is_listchars)
	{
	    vim_free(wp->w_lcs_chars.multispace);
	    vim_free(wp->w_lcs_chars.leadmultispace);
	    wp->w_lcs_chars = lcs_chars;
	}
	else
	{
	    wp->w_fill_chars = fill_chars;
	}
    }

    return NULL;	// no error
}

/*
 * Handle the new value of 'fillchars'.
 */
    char *
set_fillchars_option(win_T *wp, char_u *val, int apply, char *errbuf,
							      size_t errbuflen)
{
    return set_chars_option(wp, val, FALSE, apply, errbuf, errbuflen);
}

/*
 * Handle the new value of 'listchars'.
 */
    char *
set_listchars_option(win_T *wp, char_u *val, int apply, char *errbuf,
							      size_t errbuflen)
{
    return set_chars_option(wp, val, TRUE, apply, errbuf, errbuflen);
}

/*
 * Function given to ExpandGeneric() to obtain possible arguments of the
 * 'fillchars' option.
 */
    char_u *
get_fillchars_name(expand_T *xp UNUSED, int idx)
{
    if (idx >= (int)(sizeof(filltab) / sizeof(filltab[0])))
	return NULL;

    return (char_u*)filltab[idx].name;
}

/*
 * Function given to ExpandGeneric() to obtain possible arguments of the
 * 'listchars' option.
 */
    char_u *
get_listchars_name(expand_T *xp UNUSED, int idx)
{
    if (idx >= (int)(sizeof(lcstab) / sizeof(lcstab[0])))
	return NULL;

    return (char_u*)lcstab[idx].name;
}

/*
 * Check all global and local values of 'listchars' and 'fillchars'.
 * Return an untranslated error messages if any of them is invalid, NULL
 * otherwise.
 */
    char *
check_chars_options(void)
{
    tabpage_T   *tp;
    win_T	    *wp;

    if (set_listchars_option(curwin, p_lcs, FALSE, NULL, 0) != NULL)
	return e_conflicts_with_value_of_listchars;
    if (set_fillchars_option(curwin, p_fcs, FALSE, NULL, 0) != NULL)
	return e_conflicts_with_value_of_fillchars;
    FOR_ALL_TAB_WINDOWS(tp, wp)
    {
	if (set_listchars_option(wp, wp->w_p_lcs, FALSE, NULL, 0) != NULL)
	    return e_conflicts_with_value_of_listchars;
	if (set_fillchars_option(wp, wp->w_p_fcs, FALSE, NULL, 0) != NULL)
	    return e_conflicts_with_value_of_fillchars;
    }
    return NULL;
}
