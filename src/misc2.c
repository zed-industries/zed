/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * misc2.c: Various functions.
 */
#include "vim.h"

static char_u	*username = NULL; // cached result of mch_get_user_name()

static int coladvance2(pos_T *pos, int addspaces, int finetune, colnr_T wcol);

/*
 * Return TRUE if in the current mode we need to use virtual.
 */
    int
virtual_active(void)
{
    unsigned int cur_ve_flags = get_ve_flags();

    // While an operator is being executed we return "virtual_op", because
    // VIsual_active has already been reset, thus we can't check for "block"
    // being used.
    if (virtual_op != MAYBE)
	return virtual_op;
    return (cur_ve_flags == VE_ALL
	    || ((cur_ve_flags & VE_BLOCK) && VIsual_active
						      && VIsual_mode == Ctrl_V)
	    || ((cur_ve_flags & VE_INSERT) && (State & MODE_INSERT)));
}

/*
 * Get the screen position of the cursor.
 */
    int
getviscol(void)
{
    colnr_T	x;

    getvvcol(curwin, &curwin->w_cursor, &x, NULL, NULL);
    return (int)x;
}

/*
 * Go to column "wcol", and add/insert white space as necessary to get the
 * cursor in that column.
 * The caller must have saved the cursor line for undo!
 */
    int
coladvance_force(colnr_T wcol)
{
    int rc = coladvance2(&curwin->w_cursor, TRUE, FALSE, wcol);

    if (wcol == MAXCOL)
	curwin->w_valid &= ~VALID_VIRTCOL;
    else
    {
	// Virtcol is valid
	curwin->w_valid |= VALID_VIRTCOL;
	curwin->w_virtcol = wcol;
    }
    return rc;
}

/*
 * Get the screen position of character col with a coladd in the cursor line.
 */
    int
getviscol2(colnr_T col, colnr_T coladd UNUSED)
{
    colnr_T	x;
    pos_T	pos;

    pos.lnum = curwin->w_cursor.lnum;
    pos.col = col;
    pos.coladd = coladd;
    getvvcol(curwin, &pos, &x, NULL, NULL);
    return (int)x;
}

/*
 * Try to advance the Cursor to the specified screen column "wantcol".
 * If virtual editing: fine tune the cursor position.
 * Note that all virtual positions off the end of a line should share
 * a curwin->w_cursor.col value (n.b. this is equal to STRLEN(line)),
 * beginning at coladd 0.
 *
 * return OK if desired column is reached, FAIL if not
 */
    int
coladvance(colnr_T wantcol)
{
    int rc = getvpos(&curwin->w_cursor, wantcol);

    if (wantcol == MAXCOL || rc == FAIL)
	curwin->w_valid &= ~VALID_VIRTCOL;
    else if (*ml_get_cursor() != TAB)
    {
	// Virtcol is valid when not on a TAB
	curwin->w_valid |= VALID_VIRTCOL;
	curwin->w_virtcol = wantcol;
    }
    return rc;
}

/*
 * Return in "pos" the position of the cursor advanced to screen column
 * "wantcol".
 * return OK if desired column is reached, FAIL if not
 */
    int
getvpos(pos_T *pos, colnr_T wantcol)
{
    return coladvance2(pos, FALSE, virtual_active(), wantcol);
}

    static int
coladvance2(
    pos_T	*pos,
    int		addspaces,	// change the text to achieve our goal?
    int		finetune,	// change char offset for the exact column
    colnr_T	wcol_arg)	// column to move to (can be negative)
{
    colnr_T	wcol = wcol_arg;
    int		idx;
    char_u	*line;
    colnr_T	col = 0;
    int		csize = 0;
    int		one_more;
#ifdef FEAT_LINEBREAK
    int		head = 0;
#endif

    one_more = (State & MODE_INSERT)
		    || restart_edit != NUL
		    || (VIsual_active && *p_sel != 'o')
		    || ((get_ve_flags() & VE_ONEMORE) && wcol < MAXCOL);
    line = ml_get_buf(curbuf, pos->lnum, FALSE);

    if (wcol >= MAXCOL)
    {
	    idx = (int)STRLEN(line) - 1 + one_more;
	    col = wcol;

	    if ((addspaces || finetune) && !VIsual_active)
	    {
		curwin->w_curswant = linetabsize(curwin, pos->lnum) + one_more;
		if (curwin->w_curswant > 0)
		    --curwin->w_curswant;
	    }
    }
    else
    {
	int		width = curwin->w_width - win_col_off(curwin);
	chartabsize_T	cts;

	if (finetune
		&& curwin->w_p_wrap
		&& curwin->w_width != 0
		&& wcol >= (colnr_T)width
		&& width > 0)
	{
	    csize = linetabsize(curwin, pos->lnum);
	    if (csize > 0)
		csize--;

	    if (wcol / width > (colnr_T)csize / width
		    && ((State & MODE_INSERT) == 0 || (int)wcol > csize + 1))
	    {
		// In case of line wrapping don't move the cursor beyond the
		// right screen edge.  In Insert mode allow going just beyond
		// the last character (like what happens when typing and
		// reaching the right window edge).
		wcol = (csize / width + 1) * width - 1;
	    }
	}

	init_chartabsize_arg(&cts, curwin, pos->lnum, 0, line, line);
	while (cts.cts_vcol <= wcol && *cts.cts_ptr != NUL)
	{
#ifdef FEAT_PROP_POPUP
	    int at_start = cts.cts_ptr == cts.cts_line;
#endif
	    // Count a tab for what it's worth (if list mode not on)
#ifdef FEAT_LINEBREAK
	    csize = win_lbr_chartabsize(&cts, &head);
	    MB_PTR_ADV(cts.cts_ptr);
#else
	    csize = lbr_chartabsize_adv(&cts);
#endif
	    cts.cts_vcol += csize;
#ifdef FEAT_PROP_POPUP
	    if (at_start)
		// do not count the columns for virtual text above
		cts.cts_vcol -= cts.cts_first_char;
#endif
	}
	col = cts.cts_vcol;
	idx = (int)(cts.cts_ptr - line);
	clear_chartabsize_arg(&cts);

	/*
	 * Handle all the special cases.  The virtual_active() check
	 * is needed to ensure that a virtual position off the end of
	 * a line has the correct indexing.  The one_more comparison
	 * replaces an explicit add of one_more later on.
	 */
	if (col > wcol || (!virtual_active() && one_more == 0))
	{
	    idx -= 1;
# ifdef FEAT_LINEBREAK
	    // Don't count the chars from 'showbreak'.
	    csize -= head;
# endif
	    col -= csize;
	}

	if (virtual_active()
		&& addspaces
		&& wcol >= 0
		&& ((col != wcol && col != wcol + 1) || csize > 1))
	{
	    // 'virtualedit' is set: The difference between wcol and col is
	    // filled with spaces.

	    if (line[idx] == NUL)
	    {
		// Append spaces
		int	correct = wcol - col;
		char_u	*newline = alloc(idx + correct + 1);
		int	t;

		if (newline == NULL)
		    return FAIL;

		for (t = 0; t < idx; ++t)
		    newline[t] = line[t];

		for (t = 0; t < correct; ++t)
		    newline[t + idx] = ' ';

		newline[idx + correct] = NUL;

		ml_replace(pos->lnum, newline, FALSE);
		changed_bytes(pos->lnum, (colnr_T)idx);
		idx += correct;
		col = wcol;
	    }
	    else
	    {
		// Break a tab
		int	linelen = (int)STRLEN(line);
		int	correct = wcol - col - csize + 1; // negative!!
		char_u	*newline;
		int	t, s = 0;
		int	v;

		if (-correct > csize)
		    return FAIL;

		newline = alloc(linelen + csize);
		if (newline == NULL)
		    return FAIL;

		for (t = 0; t < linelen; t++)
		{
		    if (t != idx)
			newline[s++] = line[t];
		    else
			for (v = 0; v < csize; v++)
			    newline[s++] = ' ';
		}

		newline[linelen + csize - 1] = NUL;

		ml_replace(pos->lnum, newline, FALSE);
		changed_bytes(pos->lnum, idx);
		idx += (csize - 1 + correct);
		col += correct;
	    }
	}
    }

    if (idx < 0)
	pos->col = 0;
    else
	pos->col = idx;

    pos->coladd = 0;

    if (finetune)
    {
	if (wcol == MAXCOL)
	{
	    // The width of the last character is used to set coladd.
	    if (!one_more)
	    {
		colnr_T	    scol, ecol;

		getvcol(curwin, pos, &scol, NULL, &ecol);
		pos->coladd = ecol - scol;
	    }
	}
	else
	{
	    int b = (int)wcol - (int)col;

	    // The difference between wcol and col is used to set coladd.
	    if (b > 0 && b < (MAXCOL - 2 * curwin->w_width))
		pos->coladd = b;

	    col += b;
	}
    }

    // prevent from moving onto a trail byte
    if (has_mbyte)
	mb_adjustpos(curbuf, pos);

    if (wcol < 0 || col < wcol)
	return FAIL;
    return OK;
}

/*
 * Increment the cursor position.  See inc() for return values.
 */
    int
inc_cursor(void)
{
    return inc(&curwin->w_cursor);
}

/*
 * Increment the line pointer "lp" crossing line boundaries as necessary.
 * Return 1 when going to the next line.
 * Return 2 when moving forward onto a NUL at the end of the line).
 * Return -1 when at the end of file.
 * Return 0 otherwise.
 */
    int
inc(pos_T *lp)
{
    char_u  *p;

    // when searching position may be set to end of a line
    if (lp->col != MAXCOL)
    {
	p = ml_get_pos(lp);
	if (*p != NUL)	// still within line, move to next char (may be NUL)
	{
	    if (has_mbyte)
	    {
		int l = (*mb_ptr2len)(p);

		lp->col += l;
		return ((p[l] != NUL) ? 0 : 2);
	    }
	    lp->col++;
	    lp->coladd = 0;
	    return ((p[1] != NUL) ? 0 : 2);
	}
    }
    if (lp->lnum != curbuf->b_ml.ml_line_count)     // there is a next line
    {
	lp->col = 0;
	lp->lnum++;
	lp->coladd = 0;
	return 1;
    }
    return -1;
}

/*
 * incl(lp): same as inc(), but skip the NUL at the end of non-empty lines
 */
    int
incl(pos_T *lp)
{
    int	    r;

    if ((r = inc(lp)) >= 1 && lp->col)
	r = inc(lp);
    return r;
}

/*
 * dec(p)
 *
 * Decrement the line pointer 'p' crossing line boundaries as necessary.
 * Return 1 when crossing a line, -1 when at start of file, 0 otherwise.
 */
    int
dec_cursor(void)
{
    return dec(&curwin->w_cursor);
}

    int
dec(pos_T *lp)
{
    char_u	*p;

    lp->coladd = 0;
    if (lp->col == MAXCOL)
    {
	// past end of line
	p = ml_get(lp->lnum);
	lp->col = (colnr_T)STRLEN(p);
	if (has_mbyte)
	    lp->col -= (*mb_head_off)(p, p + lp->col);
	return 0;
    }

    if (lp->col > 0)
    {
	// still within line
	lp->col--;
	if (has_mbyte)
	{
	    p = ml_get(lp->lnum);
	    lp->col -= (*mb_head_off)(p, p + lp->col);
	}
	return 0;
    }

    if (lp->lnum > 1)
    {
	// there is a prior line
	lp->lnum--;
	p = ml_get(lp->lnum);
	lp->col = (colnr_T)STRLEN(p);
	if (has_mbyte)
	    lp->col -= (*mb_head_off)(p, p + lp->col);
	return 1;
    }

    // at start of file
    return -1;
}

/*
 * decl(lp): same as dec(), but skip the NUL at the end of non-empty lines
 */
    int
decl(pos_T *lp)
{
    int	    r;

    if ((r = dec(lp)) == 1 && lp->col)
	r = dec(lp);
    return r;
}

/*
 * Get the line number relative to the current cursor position, i.e. the
 * difference between line number and cursor position. Only look for lines that
 * can be visible, folded lines don't count.
 */
    linenr_T
get_cursor_rel_lnum(
    win_T	*wp,
    linenr_T	lnum)		    // line number to get the result for
{
    linenr_T	cursor = wp->w_cursor.lnum;
    linenr_T	retval = 0;

#ifdef FEAT_FOLDING
    if (hasAnyFolding(wp))
    {
	if (lnum > cursor)
	{
	    while (lnum > cursor)
	    {
		(void)hasFoldingWin(wp, lnum, &lnum, NULL, TRUE, NULL);
		// if lnum and cursor are in the same fold,
		// now lnum <= cursor
		if (lnum > cursor)
		    retval++;
		lnum--;
	    }
	}
	else if (lnum < cursor)
	{
	    while (lnum < cursor)
	    {
		(void)hasFoldingWin(wp, lnum, NULL, &lnum, TRUE, NULL);
		// if lnum and cursor are in the same fold,
		// now lnum >= cursor
		if (lnum < cursor)
		    retval--;
		lnum++;
	    }
	}
	// else if (lnum == cursor)
	//     retval = 0;
    }
    else
#endif
	retval = lnum - cursor;

    return retval;
}

/*
 * Make sure "pos.lnum" and "pos.col" are valid in "buf".
 * This allows for the col to be on the NUL byte.
 */
    void
check_pos(buf_T *buf, pos_T *pos)
{
    char_u *line;
    colnr_T len;

    if (pos->lnum > buf->b_ml.ml_line_count)
	pos->lnum = buf->b_ml.ml_line_count;

    if (pos->col > 0)
    {
	line = ml_get_buf(buf, pos->lnum, FALSE);
	len = (colnr_T)STRLEN(line);
	if (pos->col > len)
	    pos->col = len;
    }
}

/*
 * Make sure curwin->w_cursor.lnum is valid.
 */
    void
check_cursor_lnum(void)
{
    if (curwin->w_cursor.lnum > curbuf->b_ml.ml_line_count)
    {
#ifdef FEAT_FOLDING
	// If there is a closed fold at the end of the file, put the cursor in
	// its first line.  Otherwise in the last line.
	if (!hasFolding(curbuf->b_ml.ml_line_count,
						&curwin->w_cursor.lnum, NULL))
#endif
	    curwin->w_cursor.lnum = curbuf->b_ml.ml_line_count;
    }
    if (curwin->w_cursor.lnum <= 0)
	curwin->w_cursor.lnum = 1;
}

/*
 * Make sure curwin->w_cursor.col is valid.
 */
    void
check_cursor_col(void)
{
    check_cursor_col_win(curwin);
}

/*
 * Make sure win->w_cursor.col is valid.
 */
    void
check_cursor_col_win(win_T *win)
{
    colnr_T      len;
    colnr_T      oldcol = win->w_cursor.col;
    colnr_T      oldcoladd = win->w_cursor.col + win->w_cursor.coladd;
    unsigned int cur_ve_flags = get_ve_flags();

    len = (colnr_T)STRLEN(ml_get_buf(win->w_buffer, win->w_cursor.lnum, FALSE));
    if (len == 0)
	win->w_cursor.col = 0;
    else if (win->w_cursor.col >= len)
    {
	// Allow cursor past end-of-line when:
	// - in Insert mode or restarting Insert mode
	// - in Visual mode and 'selection' isn't "old"
	// - 'virtualedit' is set
	if ((State & MODE_INSERT) || restart_edit
		|| (VIsual_active && *p_sel != 'o')
		|| (cur_ve_flags & VE_ONEMORE)
		|| virtual_active())
	    win->w_cursor.col = len;
	else
	{
	    win->w_cursor.col = len - 1;
	    // Move the cursor to the head byte.
	    if (has_mbyte)
		mb_adjustpos(win->w_buffer, &win->w_cursor);
	}
    }
    else if (win->w_cursor.col < 0)
	win->w_cursor.col = 0;

    // If virtual editing is on, we can leave the cursor on the old position,
    // only we must set it to virtual.  But don't do it when at the end of the
    // line.
    if (oldcol == MAXCOL)
	win->w_cursor.coladd = 0;
    else if (cur_ve_flags == VE_ALL)
    {
	if (oldcoladd > win->w_cursor.col)
	{
	    win->w_cursor.coladd = oldcoladd - win->w_cursor.col;

	    // Make sure that coladd is not more than the char width.
	    // Not for the last character, coladd is then used when the cursor
	    // is actually after the last character.
	    if (win->w_cursor.col + 1 < len)
	    {
		int cs, ce;

		getvcol(win, &win->w_cursor, &cs, NULL, &ce);
		if (win->w_cursor.coladd > ce - cs)
		    win->w_cursor.coladd = ce - cs;
	    }
	}
	else
	    // avoid weird number when there is a miscalculation or overflow
	    win->w_cursor.coladd = 0;
    }
}

/*
 * make sure curwin->w_cursor in on a valid character
 */
    void
check_cursor(void)
{
    check_cursor_lnum();
    check_cursor_col();
}

/*
 * Check if VIsual position is valid, correct it if not.
 * Can be called when in Visual mode and a change has been made.
 */
    void
check_visual_pos(void)
{
    if (VIsual.lnum > curbuf->b_ml.ml_line_count)
    {
	VIsual.lnum = curbuf->b_ml.ml_line_count;
	VIsual.col = 0;
	VIsual.coladd = 0;
    }
    else
    {
	int len = (int)STRLEN(ml_get(VIsual.lnum));

	if (VIsual.col > len)
	{
	    VIsual.col = len;
	    VIsual.coladd = 0;
	}
    }
}

/*
 * Make sure curwin->w_cursor is not on the NUL at the end of the line.
 * Allow it when in Visual mode and 'selection' is not "old".
 */
    void
adjust_cursor_col(void)
{
    if (curwin->w_cursor.col > 0
	    && (!VIsual_active || *p_sel == 'o')
	    && gchar_cursor() == NUL)
	--curwin->w_cursor.col;
}

/*
 * Set "curwin->w_leftcol" to "leftcol".
 * Adjust the cursor position if needed.
 * Return TRUE if the cursor was moved.
 */
    int
set_leftcol(colnr_T leftcol)
{
    int		retval = FALSE;

    // Return quickly when there is no change.
    if (curwin->w_leftcol == leftcol)
	return FALSE;
    curwin->w_leftcol = leftcol;

    changed_cline_bef_curs();
    long lastcol = curwin->w_leftcol + curwin->w_width - curwin_col_off() - 1;
    validate_virtcol();

    // If the cursor is right or left of the screen, move it to last or first
    // visible character.
    long siso = get_sidescrolloff_value();
    if (curwin->w_virtcol > (colnr_T)(lastcol - siso))
    {
	retval = TRUE;
	coladvance((colnr_T)(lastcol - siso));
    }
    else if (curwin->w_virtcol < curwin->w_leftcol + siso)
    {
	retval = TRUE;
	(void)coladvance((colnr_T)(curwin->w_leftcol + siso));
    }

    // If the start of the character under the cursor is not on the screen,
    // advance the cursor one more char.  If this fails (last char of the
    // line) adjust the scrolling.
    colnr_T	s, e;
    getvvcol(curwin, &curwin->w_cursor, &s, NULL, &e);
    if (e > (colnr_T)lastcol)
    {
	retval = TRUE;
	coladvance(s - 1);
    }
    else if (s < curwin->w_leftcol)
    {
	retval = TRUE;
	if (coladvance(e + 1) == FAIL)	// there isn't another character
	{
	    curwin->w_leftcol = s;	// adjust w_leftcol instead
	    changed_cline_bef_curs();
	}
    }

    if (retval)
	curwin->w_set_curswant = TRUE;
    redraw_later(UPD_NOT_VALID);
    return retval;
}

/*
 * Isolate one part of a string option where parts are separated with
 * "sep_chars".
 * The part is copied into "buf[maxlen]".
 * "*option" is advanced to the next part.
 * The length is returned.
 */
    int
copy_option_part(
    char_u	**option,
    char_u	*buf,
    int		maxlen,
    char	*sep_chars)
{
    int	    len = 0;
    char_u  *p = *option;

    // skip '.' at start of option part, for 'suffixes'
    if (*p == '.')
	buf[len++] = *p++;
    while (*p != NUL && vim_strchr((char_u *)sep_chars, *p) == NULL)
    {
	/*
	 * Skip backslash before a separator character and space.
	 */
	if (p[0] == '\\' && vim_strchr((char_u *)sep_chars, p[1]) != NULL)
	    ++p;
	if (len < maxlen - 1)
	    buf[len++] = *p;
	++p;
    }
    buf[len] = NUL;

    if (*p != NUL && *p != ',')	// skip non-standard separator
	++p;
    p = skip_to_option_part(p);	// p points to next file name

    *option = p;
    return len;
}

#ifndef HAVE_MEMSET
    void *
vim_memset(void *ptr, int c, size_t size)
{
    char *p = ptr;

    while (size-- > 0)
	*p++ = c;
    return ptr;
}
#endif

/*
 * Vim has its own isspace() function, because on some machines isspace()
 * can't handle characters above 128.
 */
    int
vim_isspace(int x)
{
    return ((x >= 9 && x <= 13) || x == ' ');
}

/************************************************************************
 * functions that use lookup tables for various things, generally to do with
 * special key codes.
 */

/*
 * Some useful tables.
 */

static struct modmasktable
{
    short	mod_mask;	// Bit-mask for particular key modifier
    short	mod_flag;	// Bit(s) for particular key modifier
    char_u	name;		// Single letter name of modifier
} mod_mask_table[] =
{
    {MOD_MASK_ALT,		MOD_MASK_ALT,		(char_u)'M'},
    {MOD_MASK_META,		MOD_MASK_META,		(char_u)'T'},
    {MOD_MASK_CTRL,		MOD_MASK_CTRL,		(char_u)'C'},
    {MOD_MASK_SHIFT,		MOD_MASK_SHIFT,		(char_u)'S'},
    {MOD_MASK_MULTI_CLICK,	MOD_MASK_2CLICK,	(char_u)'2'},
    {MOD_MASK_MULTI_CLICK,	MOD_MASK_3CLICK,	(char_u)'3'},
    {MOD_MASK_MULTI_CLICK,	MOD_MASK_4CLICK,	(char_u)'4'},
#if defined(MACOS_X) || defined(FEAT_GUI_GTK)
    {MOD_MASK_CMD,		MOD_MASK_CMD,		(char_u)'D'},
#endif
    // 'A' must be the last one
    {MOD_MASK_ALT,		MOD_MASK_ALT,		(char_u)'A'},
    {0, 0, NUL}
    // NOTE: when adding an entry, update MAX_KEY_NAME_LEN!
};

/*
 * Shifted key terminal codes and their unshifted equivalent.
 * Don't add mouse codes here, they are handled separately!
 */
#define MOD_KEYS_ENTRY_SIZE 5

static char_u modifier_keys_table[] =
{
//  mod mask	    with modifier		without modifier
    MOD_MASK_SHIFT, '&', '9',			'@', '1',	// begin
    MOD_MASK_SHIFT, '&', '0',			'@', '2',	// cancel
    MOD_MASK_SHIFT, '*', '1',			'@', '4',	// command
    MOD_MASK_SHIFT, '*', '2',			'@', '5',	// copy
    MOD_MASK_SHIFT, '*', '3',			'@', '6',	// create
    MOD_MASK_SHIFT, '*', '4',			'k', 'D',	// delete char
    MOD_MASK_SHIFT, '*', '5',			'k', 'L',	// delete line
    MOD_MASK_SHIFT, '*', '7',			'@', '7',	// end
    MOD_MASK_CTRL,  KS_EXTRA, (int)KE_C_END,	'@', '7',	// end
    MOD_MASK_SHIFT, '*', '9',			'@', '9',	// exit
    MOD_MASK_SHIFT, '*', '0',			'@', '0',	// find
    MOD_MASK_SHIFT, '#', '1',			'%', '1',	// help
    MOD_MASK_SHIFT, '#', '2',			'k', 'h',	// home
    MOD_MASK_CTRL,  KS_EXTRA, (int)KE_C_HOME,	'k', 'h',	// home
    MOD_MASK_SHIFT, '#', '3',			'k', 'I',	// insert
    MOD_MASK_SHIFT, '#', '4',			'k', 'l',	// left arrow
    MOD_MASK_CTRL,  KS_EXTRA, (int)KE_C_LEFT,	'k', 'l',	// left arrow
    MOD_MASK_SHIFT, '%', 'a',			'%', '3',	// message
    MOD_MASK_SHIFT, '%', 'b',			'%', '4',	// move
    MOD_MASK_SHIFT, '%', 'c',			'%', '5',	// next
    MOD_MASK_SHIFT, '%', 'd',			'%', '7',	// options
    MOD_MASK_SHIFT, '%', 'e',			'%', '8',	// previous
    MOD_MASK_SHIFT, '%', 'f',			'%', '9',	// print
    MOD_MASK_SHIFT, '%', 'g',			'%', '0',	// redo
    MOD_MASK_SHIFT, '%', 'h',			'&', '3',	// replace
    MOD_MASK_SHIFT, '%', 'i',			'k', 'r',	// right arr.
    MOD_MASK_CTRL,  KS_EXTRA, (int)KE_C_RIGHT,	'k', 'r',	// right arr.
    MOD_MASK_SHIFT, '%', 'j',			'&', '5',	// resume
    MOD_MASK_SHIFT, '!', '1',			'&', '6',	// save
    MOD_MASK_SHIFT, '!', '2',			'&', '7',	// suspend
    MOD_MASK_SHIFT, '!', '3',			'&', '8',	// undo
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_UP,	'k', 'u',	// up arrow
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_DOWN,	'k', 'd',	// down arrow

								// vt100 F1
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_XF1,	KS_EXTRA, (int)KE_XF1,
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_XF2,	KS_EXTRA, (int)KE_XF2,
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_XF3,	KS_EXTRA, (int)KE_XF3,
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_XF4,	KS_EXTRA, (int)KE_XF4,

    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F1,	'k', '1',	// F1
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F2,	'k', '2',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F3,	'k', '3',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F4,	'k', '4',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F5,	'k', '5',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F6,	'k', '6',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F7,	'k', '7',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F8,	'k', '8',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F9,	'k', '9',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F10,	'k', ';',	// F10

    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F11,	'F', '1',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F12,	'F', '2',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F13,	'F', '3',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F14,	'F', '4',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F15,	'F', '5',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F16,	'F', '6',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F17,	'F', '7',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F18,	'F', '8',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F19,	'F', '9',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F20,	'F', 'A',

    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F21,	'F', 'B',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F22,	'F', 'C',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F23,	'F', 'D',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F24,	'F', 'E',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F25,	'F', 'F',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F26,	'F', 'G',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F27,	'F', 'H',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F28,	'F', 'I',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F29,	'F', 'J',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F30,	'F', 'K',

    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F31,	'F', 'L',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F32,	'F', 'M',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F33,	'F', 'N',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F34,	'F', 'O',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F35,	'F', 'P',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F36,	'F', 'Q',
    MOD_MASK_SHIFT, KS_EXTRA, (int)KE_S_F37,	'F', 'R',

							    // TAB pseudo code
    MOD_MASK_SHIFT, 'k', 'B',			KS_EXTRA, (int)KE_TAB,

    NUL
};

static struct key_name_entry
{
    int	    key;	// Special key code or ascii value
    char_u  *name;	// Name of key
} key_names_table[] =
{
    {' ',		(char_u *)"Space"},
    {TAB,		(char_u *)"Tab"},
    {K_TAB,		(char_u *)"Tab"},
    {NL,		(char_u *)"NL"},
    {NL,		(char_u *)"NewLine"},	// Alternative name
    {NL,		(char_u *)"LineFeed"},	// Alternative name
    {NL,		(char_u *)"LF"},	// Alternative name
    {CAR,		(char_u *)"CR"},
    {CAR,		(char_u *)"Return"},	// Alternative name
    {CAR,		(char_u *)"Enter"},	// Alternative name
    {K_BS,		(char_u *)"BS"},
    {K_BS,		(char_u *)"BackSpace"},	// Alternative name
    {ESC,		(char_u *)"Esc"},
    {CSI,		(char_u *)"CSI"},
    {K_CSI,		(char_u *)"xCSI"},
    {'|',		(char_u *)"Bar"},
    {'\\',		(char_u *)"Bslash"},
    {K_DEL,		(char_u *)"Del"},
    {K_DEL,		(char_u *)"Delete"},	// Alternative name
    {K_KDEL,		(char_u *)"kDel"},
    {K_UP,		(char_u *)"Up"},
    {K_DOWN,		(char_u *)"Down"},
    {K_LEFT,		(char_u *)"Left"},
    {K_RIGHT,		(char_u *)"Right"},
    {K_XUP,		(char_u *)"xUp"},
    {K_XDOWN,		(char_u *)"xDown"},
    {K_XLEFT,		(char_u *)"xLeft"},
    {K_XRIGHT,		(char_u *)"xRight"},
    {K_PS,		(char_u *)"PasteStart"},
    {K_PE,		(char_u *)"PasteEnd"},

    {K_F1,		(char_u *)"F1"},
    {K_F2,		(char_u *)"F2"},
    {K_F3,		(char_u *)"F3"},
    {K_F4,		(char_u *)"F4"},
    {K_F5,		(char_u *)"F5"},
    {K_F6,		(char_u *)"F6"},
    {K_F7,		(char_u *)"F7"},
    {K_F8,		(char_u *)"F8"},
    {K_F9,		(char_u *)"F9"},
    {K_F10,		(char_u *)"F10"},

    {K_F11,		(char_u *)"F11"},
    {K_F12,		(char_u *)"F12"},
    {K_F13,		(char_u *)"F13"},
    {K_F14,		(char_u *)"F14"},
    {K_F15,		(char_u *)"F15"},
    {K_F16,		(char_u *)"F16"},
    {K_F17,		(char_u *)"F17"},
    {K_F18,		(char_u *)"F18"},
    {K_F19,		(char_u *)"F19"},
    {K_F20,		(char_u *)"F20"},

    {K_F21,		(char_u *)"F21"},
    {K_F22,		(char_u *)"F22"},
    {K_F23,		(char_u *)"F23"},
    {K_F24,		(char_u *)"F24"},
    {K_F25,		(char_u *)"F25"},
    {K_F26,		(char_u *)"F26"},
    {K_F27,		(char_u *)"F27"},
    {K_F28,		(char_u *)"F28"},
    {K_F29,		(char_u *)"F29"},
    {K_F30,		(char_u *)"F30"},

    {K_F31,		(char_u *)"F31"},
    {K_F32,		(char_u *)"F32"},
    {K_F33,		(char_u *)"F33"},
    {K_F34,		(char_u *)"F34"},
    {K_F35,		(char_u *)"F35"},
    {K_F36,		(char_u *)"F36"},
    {K_F37,		(char_u *)"F37"},

    {K_XF1,		(char_u *)"xF1"},
    {K_XF2,		(char_u *)"xF2"},
    {K_XF3,		(char_u *)"xF3"},
    {K_XF4,		(char_u *)"xF4"},

    {K_HELP,		(char_u *)"Help"},
    {K_UNDO,		(char_u *)"Undo"},
    {K_INS,		(char_u *)"Insert"},
    {K_INS,		(char_u *)"Ins"},	// Alternative name
    {K_KINS,		(char_u *)"kInsert"},
    {K_HOME,		(char_u *)"Home"},
    {K_KHOME,		(char_u *)"kHome"},
    {K_XHOME,		(char_u *)"xHome"},
    {K_ZHOME,		(char_u *)"zHome"},
    {K_END,		(char_u *)"End"},
    {K_KEND,		(char_u *)"kEnd"},
    {K_XEND,		(char_u *)"xEnd"},
    {K_ZEND,		(char_u *)"zEnd"},
    {K_PAGEUP,		(char_u *)"PageUp"},
    {K_PAGEDOWN,	(char_u *)"PageDown"},
    {K_KPAGEUP,		(char_u *)"kPageUp"},
    {K_KPAGEDOWN,	(char_u *)"kPageDown"},

    {K_KPLUS,		(char_u *)"kPlus"},
    {K_KMINUS,		(char_u *)"kMinus"},
    {K_KDIVIDE,		(char_u *)"kDivide"},
    {K_KMULTIPLY,	(char_u *)"kMultiply"},
    {K_KENTER,		(char_u *)"kEnter"},
    {K_KPOINT,		(char_u *)"kPoint"},

    {K_K0,		(char_u *)"k0"},
    {K_K1,		(char_u *)"k1"},
    {K_K2,		(char_u *)"k2"},
    {K_K3,		(char_u *)"k3"},
    {K_K4,		(char_u *)"k4"},
    {K_K5,		(char_u *)"k5"},
    {K_K6,		(char_u *)"k6"},
    {K_K7,		(char_u *)"k7"},
    {K_K8,		(char_u *)"k8"},
    {K_K9,		(char_u *)"k9"},

    {'<',		(char_u *)"lt"},

    {K_MOUSE,		(char_u *)"Mouse"},
#ifdef FEAT_MOUSE_NET
    {K_NETTERM_MOUSE,	(char_u *)"NetMouse"},
#endif
#ifdef FEAT_MOUSE_DEC
    {K_DEC_MOUSE,	(char_u *)"DecMouse"},
#endif
#ifdef FEAT_MOUSE_JSB
    {K_JSBTERM_MOUSE,	(char_u *)"JsbMouse"},
#endif
#ifdef FEAT_MOUSE_PTERM
    {K_PTERM_MOUSE,	(char_u *)"PtermMouse"},
#endif
#ifdef FEAT_MOUSE_URXVT
    {K_URXVT_MOUSE,	(char_u *)"UrxvtMouse"},
#endif
    {K_SGR_MOUSE,	(char_u *)"SgrMouse"},
    {K_SGR_MOUSERELEASE, (char_u *)"SgrMouseRelease"},
    {K_LEFTMOUSE,	(char_u *)"LeftMouse"},
    {K_LEFTMOUSE_NM,	(char_u *)"LeftMouseNM"},
    {K_LEFTDRAG,	(char_u *)"LeftDrag"},
    {K_LEFTRELEASE,	(char_u *)"LeftRelease"},
    {K_LEFTRELEASE_NM,	(char_u *)"LeftReleaseNM"},
    {K_MOUSEMOVE,	(char_u *)"MouseMove"},
    {K_MIDDLEMOUSE,	(char_u *)"MiddleMouse"},
    {K_MIDDLEDRAG,	(char_u *)"MiddleDrag"},
    {K_MIDDLERELEASE,	(char_u *)"MiddleRelease"},
    {K_RIGHTMOUSE,	(char_u *)"RightMouse"},
    {K_RIGHTDRAG,	(char_u *)"RightDrag"},
    {K_RIGHTRELEASE,	(char_u *)"RightRelease"},
    {K_MOUSEDOWN,	(char_u *)"ScrollWheelUp"},
    {K_MOUSEUP,		(char_u *)"ScrollWheelDown"},
    {K_MOUSELEFT,	(char_u *)"ScrollWheelRight"},
    {K_MOUSERIGHT,	(char_u *)"ScrollWheelLeft"},
    {K_MOUSEDOWN,	(char_u *)"MouseDown"}, // OBSOLETE: Use
    {K_MOUSEUP,		(char_u *)"MouseUp"},	// ScrollWheelXXX instead
    {K_X1MOUSE,		(char_u *)"X1Mouse"},
    {K_X1DRAG,		(char_u *)"X1Drag"},
    {K_X1RELEASE,		(char_u *)"X1Release"},
    {K_X2MOUSE,		(char_u *)"X2Mouse"},
    {K_X2DRAG,		(char_u *)"X2Drag"},
    {K_X2RELEASE,		(char_u *)"X2Release"},
    {K_DROP,		(char_u *)"Drop"},
    {K_ZERO,		(char_u *)"Nul"},
#ifdef FEAT_EVAL
    {K_SNR,		(char_u *)"SNR"},
#endif
    {K_PLUG,		(char_u *)"Plug"},
    {K_CURSORHOLD,	(char_u *)"CursorHold"},
    {K_IGNORE,		(char_u *)"Ignore"},
    {K_COMMAND,		(char_u *)"Cmd"},
    {K_SCRIPT_COMMAND,	(char_u *)"ScriptCmd"},
    {K_FOCUSGAINED,	(char_u *)"FocusGained"},
    {K_FOCUSLOST,	(char_u *)"FocusLost"},
    {0,			NULL}
    // NOTE: When adding a long name update MAX_KEY_NAME_LEN.
};

#define KEY_NAMES_TABLE_LEN ARRAY_LENGTH(key_names_table)

/*
 * Return the modifier mask bit (MOD_MASK_*) which corresponds to the given
 * modifier name ('S' for Shift, 'C' for Ctrl etc).
 */
    static int
name_to_mod_mask(int c)
{
    int	    i;

    c = TOUPPER_ASC(c);
    for (i = 0; mod_mask_table[i].mod_mask != 0; i++)
	if (c == mod_mask_table[i].name)
	    return mod_mask_table[i].mod_flag;
    return 0;
}

/*
 * Check if if there is a special key code for "key" that includes the
 * modifiers specified.
 */
    int
simplify_key(int key, int *modifiers)
{
    int	    i;
    int	    key0;
    int	    key1;

    if (!(*modifiers & (MOD_MASK_SHIFT | MOD_MASK_CTRL | MOD_MASK_ALT
#ifdef FEAT_GUI_GTK
	    | MOD_MASK_CMD
#endif
    )))
	return key;

    // TAB is a special case
    if (key == TAB && (*modifiers & MOD_MASK_SHIFT))
    {
	*modifiers &= ~MOD_MASK_SHIFT;
	return K_S_TAB;
    }
    key0 = KEY2TERMCAP0(key);
    key1 = KEY2TERMCAP1(key);
    for (i = 0; modifier_keys_table[i] != NUL; i += MOD_KEYS_ENTRY_SIZE)
    {
	if (key0 == modifier_keys_table[i + 3]
		&& key1 == modifier_keys_table[i + 4]
		&& (*modifiers & modifier_keys_table[i]))
	{
	    *modifiers &= ~modifier_keys_table[i];
	    return TERMCAP2KEY(modifier_keys_table[i + 1],
		    modifier_keys_table[i + 2]);
	}
    }
    return key;
}

/*
 * Change <xHome> to <Home>, <xUp> to <Up>, etc.
 */
    int
handle_x_keys(int key)
{
    switch (key)
    {
	case K_XUP:	return K_UP;
	case K_XDOWN:	return K_DOWN;
	case K_XLEFT:	return K_LEFT;
	case K_XRIGHT:	return K_RIGHT;
	case K_XHOME:	return K_HOME;
	case K_ZHOME:	return K_HOME;
	case K_XEND:	return K_END;
	case K_ZEND:	return K_END;
	case K_XF1:	return K_F1;
	case K_XF2:	return K_F2;
	case K_XF3:	return K_F3;
	case K_XF4:	return K_F4;
	case K_S_XF1:	return K_S_F1;
	case K_S_XF2:	return K_S_F2;
	case K_S_XF3:	return K_S_F3;
	case K_S_XF4:	return K_S_F4;
    }
    return key;
}

/*
 * Return a string which contains the name of the given key when the given
 * modifiers are down.
 */
    char_u *
get_special_key_name(int c, int modifiers)
{
    static char_u string[MAX_KEY_NAME_LEN + 1];

    int	    i, idx;
    int	    table_idx;
    char_u  *s;

    string[0] = '<';
    idx = 1;

    // Key that stands for a normal character.
    if (IS_SPECIAL(c) && KEY2TERMCAP0(c) == KS_KEY)
	c = KEY2TERMCAP1(c);

    /*
     * Translate shifted special keys into unshifted keys and set modifier.
     * Same for CTRL and ALT modifiers.
     */
    if (IS_SPECIAL(c))
    {
	for (i = 0; modifier_keys_table[i] != 0; i += MOD_KEYS_ENTRY_SIZE)
	    if (       KEY2TERMCAP0(c) == (int)modifier_keys_table[i + 1]
		    && (int)KEY2TERMCAP1(c) == (int)modifier_keys_table[i + 2])
	    {
		modifiers |= modifier_keys_table[i];
		c = TERMCAP2KEY(modifier_keys_table[i + 3],
						   modifier_keys_table[i + 4]);
		break;
	    }
    }

    // try to find the key in the special key table
    table_idx = find_special_key_in_table(c);

    /*
     * When not a known special key, and not a printable character, try to
     * extract modifiers.
     */
    if (c > 0 && (*mb_char2len)(c) == 1)
    {
	if (table_idx < 0
		&& (!vim_isprintc(c) || (c & 0x7f) == ' ')
		&& (c & 0x80))
	{
	    c &= 0x7f;
	    modifiers |= MOD_MASK_ALT;
	    // try again, to find the un-alted key in the special key table
	    table_idx = find_special_key_in_table(c);
	}
	if (table_idx < 0 && !vim_isprintc(c) && c < ' ')
	{
	    c += '@';
	    modifiers |= MOD_MASK_CTRL;
	}
    }

    // translate the modifier into a string
    for (i = 0; mod_mask_table[i].name != 'A'; i++)
	if ((modifiers & mod_mask_table[i].mod_mask)
						== mod_mask_table[i].mod_flag)
	{
	    string[idx++] = mod_mask_table[i].name;
	    string[idx++] = (char_u)'-';
	}

    if (table_idx < 0)		// unknown special key, may output t_xx
    {
	if (IS_SPECIAL(c))
	{
	    string[idx++] = 't';
	    string[idx++] = '_';
	    string[idx++] = KEY2TERMCAP0(c);
	    string[idx++] = KEY2TERMCAP1(c);
	}
	// Not a special key, only modifiers, output directly
	else
	{
	    if (has_mbyte && (*mb_char2len)(c) > 1)
		idx += (*mb_char2bytes)(c, string + idx);
	    else if (vim_isprintc(c))
		string[idx++] = c;
	    else
	    {
		s = transchar(c);
		while (*s)
		    string[idx++] = *s++;
	    }
	}
    }
    else		// use name of special key
    {
	size_t len = STRLEN(key_names_table[table_idx].name);

	if (len + idx + 2 <= MAX_KEY_NAME_LEN)
	{
	    STRCPY(string + idx, key_names_table[table_idx].name);
	    idx += (int)len;
	}
    }
    string[idx++] = '>';
    string[idx] = NUL;
    return string;
}

/*
 * Try translating a <> name at "(*srcp)[]" to "dst[]".
 * Return the number of characters added to "dst[]", zero for no match.
 * If there is a match, "srcp" is advanced to after the <> name.
 * "dst[]" must be big enough to hold the result (up to six characters)!
 */
    int
trans_special(
    char_u	**srcp,
    char_u	*dst,
    int		flags,		// FSK_ values
    int		escape_ks,	// escape K_SPECIAL bytes in the character
    int		*did_simplify)  // FSK_SIMPLIFY and found <C-H> or <A-x>
{
    int		modifiers = 0;
    int		key;

    key = find_special_key(srcp, &modifiers, flags, did_simplify);
    if (key == 0)
	return 0;

    return special_to_buf(key, modifiers, escape_ks, dst);
}

/*
 * Put the character sequence for "key" with "modifiers" into "dst" and return
 * the resulting length.
 * When "escape_ks" is TRUE escape K_SPECIAL bytes in the character.
 * The sequence is not NUL terminated.
 * This is how characters in a string are encoded.
 */
    int
special_to_buf(int key, int modifiers, int escape_ks, char_u *dst)
{
    int		dlen = 0;

    // Put the appropriate modifier in a string
    if (modifiers != 0)
    {
	dst[dlen++] = K_SPECIAL;
	dst[dlen++] = KS_MODIFIER;
	dst[dlen++] = modifiers;
    }

    if (IS_SPECIAL(key))
    {
	dst[dlen++] = K_SPECIAL;
	dst[dlen++] = KEY2TERMCAP0(key);
	dst[dlen++] = KEY2TERMCAP1(key);
    }
    else if (escape_ks)
	dlen = (int)(add_char2buf(key, dst + dlen) - dst);
    else if (has_mbyte)
	dlen += (*mb_char2bytes)(key, dst + dlen);
    else
	dst[dlen++] = key;

    return dlen;
}

/*
 * Try translating a <> name at "(*srcp)[]", return the key and put modifiers
 * in "modp".
 * "srcp" is advanced to after the <> name.
 * returns 0 if there is no match.
 */
    int
find_special_key(
    char_u	**srcp,
    int		*modp,
    int		flags,		// FSK_ values
    int		*did_simplify)  // found <C-H> or <A-x>
{
    char_u	*last_dash;
    char_u	*end_of_name;
    char_u	*src;
    char_u	*bp;
    int		in_string = flags & FSK_IN_STRING;
    int		modifiers;
    int		bit;
    int		key;
    uvarnumber_T	n;
    int		l;

    src = *srcp;
    if (src[0] != '<')
	return 0;
    if (src[1] == '*')	    // <*xxx>: do not simplify
	++src;

    // Find end of modifier list
    last_dash = src;
    for (bp = src + 1; *bp == '-' || vim_isNormalIDc(*bp); bp++)
    {
	if (*bp == '-')
	{
	    last_dash = bp;
	    if (bp[1] != NUL)
	    {
		if (has_mbyte)
		    l = mb_ptr2len(bp + 1);
		else
		    l = 1;
		// Anything accepted, like <C-?>.
		// <C-"> or <M-"> are not special in strings as " is
		// the string delimiter. With a backslash it works: <M-\">
		if (!(in_string && bp[1] == '"') && bp[l + 1] == '>')
		    bp += l;
		else if (in_string && bp[1] == '\\' && bp[2] == '"'
							   && bp[3] == '>')
		    bp += 2;
	    }
	}
	if (bp[0] == 't' && bp[1] == '_' && bp[2] && bp[3])
	    bp += 3;	// skip t_xx, xx may be '-' or '>'
	else if (STRNICMP(bp, "char-", 5) == 0)
	{
	    vim_str2nr(bp + 5, NULL, &l, STR2NR_ALL, NULL, NULL, 0, TRUE, NULL);
	    if (l == 0)
	    {
		emsg(_(e_invalid_argument));
		return 0;
	    }
	    bp += l + 5;
	    break;
	}
    }

    if (*bp == '>')	// found matching '>'
    {
	end_of_name = bp + 1;

	// Which modifiers are given?
	modifiers = 0x0;
	for (bp = src + 1; bp < last_dash; bp++)
	{
	    if (*bp != '-')
	    {
		bit = name_to_mod_mask(*bp);
		if (bit == 0x0)
		    break;	// Illegal modifier name
		modifiers |= bit;
	    }
	}

	/*
	 * Legal modifier name.
	 */
	if (bp >= last_dash)
	{
	    if (STRNICMP(last_dash + 1, "char-", 5) == 0
						 && VIM_ISDIGIT(last_dash[6]))
	    {
		// <Char-123> or <Char-033> or <Char-0x33>
		vim_str2nr(last_dash + 6, NULL, &l, STR2NR_ALL, NULL,
							    &n, 0, TRUE, NULL);
		if (l == 0)
		{
		    emsg(_(e_invalid_argument));
		    return 0;
		}
		key = (int)n;
	    }
	    else
	    {
		int off = 1;

		// Modifier with single letter, or special key name.
		if (in_string && last_dash[1] == '\\' && last_dash[2] == '"')
		    off = 2;
		if (has_mbyte)
		    l = mb_ptr2len(last_dash + off);
		else
		    l = 1;
		if (modifiers != 0 && last_dash[l + off] == '>')
		    key = PTR2CHAR(last_dash + off);
		else
		{
		    key = get_special_key_code(last_dash + off);
		    if (!(flags & FSK_KEEP_X_KEY))
			key = handle_x_keys(key);
		}
	    }

	    /*
	     * get_special_key_code() may return NUL for invalid
	     * special key name.
	     */
	    if (key != NUL)
	    {
		/*
		 * Only use a modifier when there is no special key code that
		 * includes the modifier.
		 */
		key = simplify_key(key, &modifiers);

		if ((flags & FSK_KEYCODE) == 0)
		{
		    // don't want keycode, use single byte code
		    if (key == K_BS)
			key = BS;
		    else if (key == K_DEL || key == K_KDEL)
			key = DEL;
		}
		else if (key == 27
			&& (flags & FSK_FROM_PART) != 0
			&& (kitty_protocol_state == KKPS_ENABLED
			    || kitty_protocol_state == KKPS_DISABLED))
		{
		    // Using the Kitty key protocol, which uses K_ESC for an
		    // Esc character.  For the simplified keys use the Esc
		    // character and set did_simplify, then in the
		    // non-simplified keys use K_ESC.
		    if ((flags & FSK_SIMPLIFY) != 0)
		    {
			if (did_simplify != NULL)
			    *did_simplify = TRUE;
		    }
		    else
			key = K_ESC;
		}

		// Normal Key with modifier: Try to make a single byte code.
		if (!IS_SPECIAL(key))
		    key = extract_modifiers(key, &modifiers,
					   flags & FSK_SIMPLIFY, did_simplify);

		*modp = modifiers;
		*srcp = end_of_name;
		return key;
	    }
	}
    }
    return 0;
}


/*
 * Some keys are used with Ctrl without Shift and are still expected to be
 * mapped as if Shift was pressed:
 * CTRL-2 is CTRL-@
 * CTRL-6 is CTRL-^
 * CTRL-- is CTRL-_
 * Also, unless no_reduce_keys is set then <C-H> and <C-h> mean the same thing,
 * use "H".
 * Returns the possibly adjusted key.
 */
    int
may_adjust_key_for_ctrl(int modifiers, int key)
{
    if ((modifiers & MOD_MASK_CTRL) == 0)
	return key;

    if (ASCII_ISALPHA(key))
    {
#ifdef FEAT_TERMINAL
	check_no_reduce_keys();  // may update the no_reduce_keys flag
#endif
	return no_reduce_keys == 0 ? TOUPPER_ASC(key) : key;
    }
    if (key == '2')
	return '@';
    if (key == '6')
	return '^';
    if (key == '-')
	return '_';

    // On a Belgian keyboard AltGr $ is ']', on other keyboards '$' can only be
    // obtained with Shift.  Assume that '$' without shift implies a Belgian
    // keyboard, where CTRL-$ means CTRL-].
    if (key == '$' && (modifiers & MOD_MASK_SHIFT) == 0)
	return ']';

    return key;
}

/*
 * Some keys already have Shift included, pass them as normal keys.
 * When Ctrl is also used <C-H> and <C-S-H> are different, but <C-S-{> should
 * be <C-{>.  Same for <C-S-}> and <C-S-|>.
 * Also for <A-S-a> and <M-S-a>.
 * This includes all printable ASCII characters except a-z.
 * Digits are included because with AZERTY the Shift key is used to get them.
 */
    int
may_remove_shift_modifier(int modifiers, int key)
{
    if ((modifiers == MOD_MASK_SHIFT
		|| modifiers == (MOD_MASK_SHIFT | MOD_MASK_ALT)
#ifdef FEAT_GUI_GTK
		|| modifiers == (MOD_MASK_SHIFT | MOD_MASK_CMD)
#endif
		|| modifiers == (MOD_MASK_SHIFT | MOD_MASK_META))
	    && ((key >= '!' && key <= '/')
		|| (key >= ':' && key <= 'Z')
		|| vim_isdigit(key)
		|| (key >= '[' && key <= '`')
		|| (key >= '{' && key <= '~')))
	return modifiers & ~MOD_MASK_SHIFT;

    if (modifiers == (MOD_MASK_SHIFT | MOD_MASK_CTRL)
		&& (key == '{' || key == '}' || key == '|'))
	return modifiers & ~MOD_MASK_SHIFT;

    return modifiers;
}

/*
 * Try to include modifiers in the key.
 * Changes "Shift-a" to 'A', "Alt-A" to 0xc0, etc.
 * When "simplify" is FALSE don't do Ctrl and Alt.
 * When "simplify" is TRUE and Ctrl or Alt is removed from modifiers set
 * "did_simplify" when it's not NULL.
 */
    int
extract_modifiers(int key, int *modp, int simplify, int *did_simplify)
{
    int	modifiers = *modp;

#ifdef MACOS_X
    // Command-key really special, no fancynest
    if (!(modifiers & MOD_MASK_CMD))
#endif
    if ((modifiers & MOD_MASK_SHIFT) && ASCII_ISALPHA(key))
    {
	key = TOUPPER_ASC(key);
	// With <C-S-a> we keep the shift modifier.
	// With <S-a>, <A-S-a> and <S-A> we don't keep the shift modifier.
	if (simplify || modifiers == MOD_MASK_SHIFT
		|| modifiers == (MOD_MASK_SHIFT | MOD_MASK_ALT)
		|| modifiers == (MOD_MASK_SHIFT | MOD_MASK_META))
	    modifiers &= ~MOD_MASK_SHIFT;
    }

    // <C-H> and <C-h> mean the same thing, always use "H"
    if ((modifiers & MOD_MASK_CTRL) && ASCII_ISALPHA(key))
	key = TOUPPER_ASC(key);

    if (simplify && (modifiers & MOD_MASK_CTRL)
	    && ((key >= '?' && key <= '_') || ASCII_ISALPHA(key)))
    {
	key = Ctrl_chr(key);
	modifiers &= ~MOD_MASK_CTRL;
	// <C-@> is <Nul>
	if (key == NUL)
	    key = K_ZERO;
	if (did_simplify != NULL)
	    *did_simplify = TRUE;
    }

#ifdef MACOS_X
    // Command-key really special, no fancynest
    if (!(modifiers & MOD_MASK_CMD))
#endif
    if (simplify && (modifiers & MOD_MASK_ALT) && key < 0x80
	    && !enc_dbcs)		// avoid creating a lead byte
    {
	key |= 0x80;
	modifiers &= ~MOD_MASK_ALT;	// remove the META modifier
	if (did_simplify != NULL)
	    *did_simplify = TRUE;
    }

    *modp = modifiers;
    return key;
}

/*
 * Try to find key "c" in the special key table.
 * Return the index when found, -1 when not found.
 */
    int
find_special_key_in_table(int c)
{
    int	    i;

    for (i = 0; key_names_table[i].name != NULL; i++)
	if (c == key_names_table[i].key)
	    break;
    if (key_names_table[i].name == NULL)
	i = -1;
    return i;
}

/*
 * Find the special key with the given name (the given string does not have to
 * end with NUL, the name is assumed to end before the first non-idchar).
 * If the name starts with "t_" the next two characters are interpreted as a
 * termcap name.
 * Return the key code, or 0 if not found.
 */
    int
get_special_key_code(char_u *name)
{
    char_u  *table_name;
    char_u  string[3];
    int	    i, j;

    /*
     * If it's <t_xx> we get the code for xx from the termcap
     */
    if (name[0] == 't' && name[1] == '_' && name[2] != NUL && name[3] != NUL)
    {
	string[0] = name[2];
	string[1] = name[3];
	string[2] = NUL;
	if (add_termcap_entry(string, FALSE) == OK)
	    return TERMCAP2KEY(name[2], name[3]);
    }
    else
	for (i = 0; key_names_table[i].name != NULL; i++)
	{
	    table_name = key_names_table[i].name;
	    for (j = 0; vim_isNormalIDc(name[j]) && table_name[j] != NUL; j++)
		if (TOLOWER_ASC(table_name[j]) != TOLOWER_ASC(name[j]))
		    break;
	    if (!vim_isNormalIDc(name[j]) && table_name[j] == NUL)
		return key_names_table[i].key;
	}
    return 0;
}

    char_u *
get_key_name(int i)
{
    if (i >= (int)KEY_NAMES_TABLE_LEN)
	return NULL;
    return  key_names_table[i].name;
}

/*
 * Return the current end-of-line type: EOL_DOS, EOL_UNIX or EOL_MAC.
 */
    int
get_fileformat(buf_T *buf)
{
    int		c = *buf->b_p_ff;

    if (buf->b_p_bin || c == 'u')
	return EOL_UNIX;
    if (c == 'm')
	return EOL_MAC;
    return EOL_DOS;
}

/*
 * Like get_fileformat(), but override 'fileformat' with "p" for "++opt=val"
 * argument.
 */
    int
get_fileformat_force(
    buf_T	*buf,
    exarg_T	*eap)	    // can be NULL!
{
    int		c;

    if (eap != NULL && eap->force_ff != 0)
	c = eap->force_ff;
    else
    {
	if ((eap != NULL && eap->force_bin != 0)
			       ? (eap->force_bin == FORCE_BIN) : buf->b_p_bin)
	    return EOL_UNIX;
	c = *buf->b_p_ff;
    }
    if (c == 'u')
	return EOL_UNIX;
    if (c == 'm')
	return EOL_MAC;
    return EOL_DOS;
}

/*
 * Set the current end-of-line type to EOL_DOS, EOL_UNIX or EOL_MAC.
 * Sets both 'textmode' and 'fileformat'.
 * Note: Does _not_ set global value of 'textmode'!
 */
    void
set_fileformat(
    int		t,
    int		opt_flags)	// OPT_LOCAL and/or OPT_GLOBAL
{
    char	*p = NULL;

    switch (t)
    {
    case EOL_DOS:
	p = FF_DOS;
	curbuf->b_p_tx = TRUE;
	break;
    case EOL_UNIX:
	p = FF_UNIX;
	curbuf->b_p_tx = FALSE;
	break;
    case EOL_MAC:
	p = FF_MAC;
	curbuf->b_p_tx = FALSE;
	break;
    }
    if (p != NULL)
	set_string_option_direct((char_u *)"ff", -1, (char_u *)p,
						     OPT_FREE | opt_flags, 0);

    // This may cause the buffer to become (un)modified.
    check_status(curbuf);
    redraw_tabline = TRUE;
    need_maketitle = TRUE;	    // set window title later
}

/*
 * Return the default fileformat from 'fileformats'.
 */
    int
default_fileformat(void)
{
    switch (*p_ffs)
    {
	case 'm':   return EOL_MAC;
	case 'd':   return EOL_DOS;
    }
    return EOL_UNIX;
}

/*
 * Call shell.	Calls mch_call_shell, with 'shellxquote' added.
 */
    int
call_shell(char_u *cmd, int opt)
{
    char_u	*ncmd;
    int		retval;
#ifdef FEAT_PROFILE
    proftime_T	wait_time;
#endif

    if (p_verbose > 3)
    {
	verbose_enter();
	smsg(_("Calling shell to execute: \"%s\""), cmd == NULL ? p_sh : cmd);
	msg_putchar_attr('\n', 0);
	cursor_on();
	verbose_leave();
    }

#ifdef FEAT_PROFILE
    if (do_profiling == PROF_YES)
	prof_child_enter(&wait_time);
#endif

    if (*p_sh == NUL)
    {
	emsg(_(e_shell_option_is_empty));
	retval = -1;
    }
    else
    {
#ifdef FEAT_GUI_MSWIN
	// Don't hide the pointer while executing a shell command.
	gui_mch_mousehide(FALSE);
#endif
#ifdef FEAT_GUI
	++hold_gui_events;
#endif
	// The external command may update a tags file, clear cached tags.
	tag_freematch();

	if (cmd == NULL || *p_sxq == NUL)
	    retval = mch_call_shell(cmd, opt);
	else
	{
	    char_u *ecmd = cmd;

	    if (*p_sxe != NUL && *p_sxq == '(')
	    {
		ecmd = vim_strsave_escaped_ext(cmd, p_sxe, '^', FALSE);
		if (ecmd == NULL)
		    ecmd = cmd;
	    }
	    ncmd = alloc(STRLEN(ecmd) + STRLEN(p_sxq) * 2 + 1);
	    if (ncmd != NULL)
	    {
		STRCPY(ncmd, p_sxq);
		STRCAT(ncmd, ecmd);
		// When 'shellxquote' is ( append ).
		// When 'shellxquote' is "( append )".
		STRCAT(ncmd, *p_sxq == '(' ? (char_u *)")"
		    : *p_sxq == '"' && *(p_sxq+1) == '(' ? (char_u *)")\""
		    : p_sxq);
		retval = mch_call_shell(ncmd, opt);
		vim_free(ncmd);
	    }
	    else
		retval = -1;
	    if (ecmd != cmd)
		vim_free(ecmd);
	}
#ifdef FEAT_GUI
	--hold_gui_events;
#endif
	/*
	 * Check the window size, in case it changed while executing the
	 * external command.
	 */
	shell_resized_check();
    }

#ifdef FEAT_EVAL
    set_vim_var_nr(VV_SHELL_ERROR, (long)retval);
# ifdef FEAT_PROFILE
    if (do_profiling == PROF_YES)
	prof_child_exit(&wait_time);
# endif
#endif

    return retval;
}

/*
 * MODE_VISUAL, MODE_SELECT and MODE_OP_PENDING State are never set, they are
 * equal to MODE_NORMAL State with a condition.  This function returns the real
 * State.
 */
    int
get_real_state(void)
{
    if (State & MODE_NORMAL)
    {
	if (VIsual_active)
	{
	    if (VIsual_select)
		return MODE_SELECT;
	    return MODE_VISUAL;
	}
	else if (finish_op)
	    return MODE_OP_PENDING;
    }
    return State;
}

/*
 * Return TRUE if "p" points to just after a path separator.
 * Takes care of multi-byte characters.
 * "b" must point to the start of the file name
 */
    int
after_pathsep(char_u *b, char_u *p)
{
    return p > b && vim_ispathsep(p[-1])
			     && (!has_mbyte || (*mb_head_off)(b, p - 1) == 0);
}

/*
 * Return TRUE if file names "f1" and "f2" are in the same directory.
 * "f1" may be a short name, "f2" must be a full path.
 */
    int
same_directory(char_u *f1, char_u *f2)
{
    char_u	ffname[MAXPATHL];
    char_u	*t1;
    char_u	*t2;

    // safety check
    if (f1 == NULL || f2 == NULL)
	return FALSE;

    (void)vim_FullName(f1, ffname, MAXPATHL, FALSE);
    t1 = gettail_sep(ffname);
    t2 = gettail_sep(f2);
    return (t1 - ffname == t2 - f2
	     && pathcmp((char *)ffname, (char *)f2, (int)(t1 - ffname)) == 0);
}

#if defined(FEAT_SESSION) || defined(FEAT_AUTOCHDIR) \
	|| defined(MSWIN) || defined(FEAT_GUI_GTK) \
	|| defined(FEAT_NETBEANS_INTG) \
	|| defined(PROTO)
/*
 * Change to a file's directory.
 * Caller must call shorten_fnames()!
 * Return OK or FAIL.
 */
    int
vim_chdirfile(char_u *fname, char *trigger_autocmd)
{
    char_u	old_dir[MAXPATHL];
    char_u	new_dir[MAXPATHL];

    if (mch_dirname(old_dir, MAXPATHL) != OK)
	*old_dir = NUL;

    vim_strncpy(new_dir, fname, MAXPATHL - 1);
    *gettail_sep(new_dir) = NUL;

    if (pathcmp((char *)old_dir, (char *)new_dir, -1) == 0)
	// nothing to do
	return OK;

    if (trigger_autocmd != NULL)
	trigger_DirChangedPre((char_u *)trigger_autocmd, new_dir);

    if (mch_chdir((char *)new_dir) != 0)
	return FAIL;

    if (trigger_autocmd != NULL)
	apply_autocmds(EVENT_DIRCHANGED, (char_u *)trigger_autocmd,
						       new_dir, FALSE, curbuf);
    return OK;
}
#endif

#if defined(STAT_IGNORES_SLASH) || defined(PROTO)
/*
 * Check if "name" ends in a slash and is not a directory.
 * Used for systems where stat() ignores a trailing slash on a file name.
 * The Vim code assumes a trailing slash is only ignored for a directory.
 */
    static int
illegal_slash(const char *name)
{
    if (name[0] == NUL)
	return FALSE;	    // no file name is not illegal
    if (name[strlen(name) - 1] != '/')
	return FALSE;	    // no trailing slash
    if (mch_isdir((char_u *)name))
	return FALSE;	    // trailing slash for a directory
    return TRUE;
}

/*
 * Special implementation of mch_stat() for Solaris.
 */
    int
vim_stat(const char *name, stat_T *stp)
{
    // On Solaris stat() accepts "file/" as if it was "file".  Return -1 if
    // the name ends in "/" and it's not a directory.
    return illegal_slash(name) ? -1 : stat(name, stp);
}
#endif

#if defined(CURSOR_SHAPE) || defined(PROTO)

/*
 * Handling of cursor and mouse pointer shapes in various modes.
 */

cursorentry_T shape_table[SHAPE_IDX_COUNT] =
{
    // The values will be filled in from the 'guicursor' and 'mouseshape'
    // defaults when Vim starts.
    // Adjust the SHAPE_IDX_ defines when making changes!
    {0,	0, 0, 700L, 400L, 250L, 0, 0, "n", SHAPE_CURSOR+SHAPE_MOUSE},
    {0,	0, 0, 700L, 400L, 250L, 0, 0, "v", SHAPE_CURSOR+SHAPE_MOUSE},
    {0,	0, 0, 700L, 400L, 250L, 0, 0, "i", SHAPE_CURSOR+SHAPE_MOUSE},
    {0,	0, 0, 700L, 400L, 250L, 0, 0, "r", SHAPE_CURSOR+SHAPE_MOUSE},
    {0,	0, 0, 700L, 400L, 250L, 0, 0, "c", SHAPE_CURSOR+SHAPE_MOUSE},
    {0,	0, 0, 700L, 400L, 250L, 0, 0, "ci", SHAPE_CURSOR+SHAPE_MOUSE},
    {0,	0, 0, 700L, 400L, 250L, 0, 0, "cr", SHAPE_CURSOR+SHAPE_MOUSE},
    {0,	0, 0, 700L, 400L, 250L, 0, 0, "o", SHAPE_CURSOR+SHAPE_MOUSE},
    {0,	0, 0, 700L, 400L, 250L, 0, 0, "ve", SHAPE_CURSOR+SHAPE_MOUSE},
    {0,	0, 0,   0L,   0L,   0L, 0, 0, "e", SHAPE_MOUSE},
    {0,	0, 0,   0L,   0L,   0L, 0, 0, "s", SHAPE_MOUSE},
    {0,	0, 0,   0L,   0L,   0L, 0, 0, "sd", SHAPE_MOUSE},
    {0,	0, 0,   0L,   0L,   0L, 0, 0, "vs", SHAPE_MOUSE},
    {0,	0, 0,   0L,   0L,   0L, 0, 0, "vd", SHAPE_MOUSE},
    {0,	0, 0,   0L,   0L,   0L, 0, 0, "m", SHAPE_MOUSE},
    {0,	0, 0,   0L,   0L,   0L, 0, 0, "ml", SHAPE_MOUSE},
    {0,	0, 0, 100L, 100L, 100L, 0, 0, "sm", SHAPE_CURSOR},
};

# ifdef FEAT_MOUSESHAPE
/*
 * Table with names for mouse shapes.  Keep in sync with all the tables for
 * mch_set_mouse_shape()!.
 */
static char *mshape_names[] =
{
    "arrow",	// default, must be the first one
    "blank",	// hidden
    "beam",
    "updown",
    "udsizing",
    "leftright",
    "lrsizing",
    "busy",
    "no",
    "crosshair",
    "hand1",
    "hand2",
    "pencil",
    "question",
    "rightup-arrow",
    "up-arrow",
    NULL
};

#  define MSHAPE_NAMES_COUNT  (ARRAY_LENGTH(mshape_names) - 1)
# endif

/*
 * Parse the 'guicursor' option ("what" is SHAPE_CURSOR) or 'mouseshape'
 * ("what" is SHAPE_MOUSE).
 * Returns error message for an illegal option, NULL otherwise.
 */
    char *
parse_shape_opt(int what)
{
    char_u	*modep;
    char_u	*colonp;
    char_u	*commap;
    char_u	*slashp;
    char_u	*p, *endp;
    int		idx = 0;		// init for GCC
    int		all_idx;
    int		len;
    int		i;
    long	n;
    int		found_ve = FALSE;	// found "ve" flag
    int		round;

    /*
     * First round: check for errors; second round: do it for real.
     */
    for (round = 1; round <= 2; ++round)
    {
	/*
	 * Repeat for all comma separated parts.
	 */
#ifdef FEAT_MOUSESHAPE
	if (what == SHAPE_MOUSE)
	    modep = p_mouseshape;
	else
#endif
	    modep = p_guicursor;
	while (*modep != NUL)
	{
	    colonp = vim_strchr(modep, ':');
	    commap = vim_strchr(modep, ',');

	    if (colonp == NULL || (commap != NULL && commap < colonp))
		return e_missing_colon_2;
	    if (colonp == modep)
		return e_illegal_mode;

	    /*
	     * Repeat for all mode's before the colon.
	     * For the 'a' mode, we loop to handle all the modes.
	     */
	    all_idx = -1;
	    while (modep < colonp || all_idx >= 0)
	    {
		if (all_idx < 0)
		{
		    // Find the mode.
		    if (modep[1] == '-' || modep[1] == ':')
			len = 1;
		    else
			len = 2;
		    if (len == 1 && TOLOWER_ASC(modep[0]) == 'a')
			all_idx = SHAPE_IDX_COUNT - 1;
		    else
		    {
			for (idx = 0; idx < SHAPE_IDX_COUNT; ++idx)
			    if (STRNICMP(modep, shape_table[idx].name, len)
									 == 0)
				break;
			if (idx == SHAPE_IDX_COUNT
				   || (shape_table[idx].used_for & what) == 0)
			    return e_illegal_mode;
			if (len == 2 && modep[0] == 'v' && modep[1] == 'e')
			    found_ve = TRUE;
		    }
		    modep += len + 1;
		}

		if (all_idx >= 0)
		    idx = all_idx--;
		else if (round == 2)
		{
#ifdef FEAT_MOUSESHAPE
		    if (what == SHAPE_MOUSE)
		    {
			// Set the default, for the missing parts
			shape_table[idx].mshape = 0;
		    }
		    else
#endif
		    {
			// Set the defaults, for the missing parts
			shape_table[idx].shape = SHAPE_BLOCK;
			shape_table[idx].blinkwait = 700L;
			shape_table[idx].blinkon = 400L;
			shape_table[idx].blinkoff = 250L;
		    }
		}

		// Parse the part after the colon
		for (p = colonp + 1; *p && *p != ','; )
		{
#ifdef FEAT_MOUSESHAPE
		    if (what == SHAPE_MOUSE)
		    {
			for (i = 0; ; ++i)
			{
			    if (mshape_names[i] == NULL)
			    {
				if (!VIM_ISDIGIT(*p))
				    return e_illegal_mouseshape;
				if (round == 2)
				    shape_table[idx].mshape =
					      getdigits(&p) + MSHAPE_NUMBERED;
				else
				    (void)getdigits(&p);
				break;
			    }
			    len = (int)STRLEN(mshape_names[i]);
			    if (STRNICMP(p, mshape_names[i], len) == 0)
			    {
				if (round == 2)
				    shape_table[idx].mshape = i;
				p += len;
				break;
			    }
			}
		    }
		    else // if (what == SHAPE_MOUSE)
#endif
		    {
			/*
			 * First handle the ones with a number argument.
			 */
			i = *p;
			len = 0;
			if (STRNICMP(p, "ver", 3) == 0)
			    len = 3;
			else if (STRNICMP(p, "hor", 3) == 0)
			    len = 3;
			else if (STRNICMP(p, "blinkwait", 9) == 0)
			    len = 9;
			else if (STRNICMP(p, "blinkon", 7) == 0)
			    len = 7;
			else if (STRNICMP(p, "blinkoff", 8) == 0)
			    len = 8;
			if (len != 0)
			{
			    p += len;
			    if (!VIM_ISDIGIT(*p))
				return e_digit_expected;
			    n = getdigits(&p);
			    if (len == 3)   // "ver" or "hor"
			    {
				if (n == 0)
				    return e_illegal_percentage;
				if (round == 2)
				{
				    if (TOLOWER_ASC(i) == 'v')
					shape_table[idx].shape = SHAPE_VER;
				    else
					shape_table[idx].shape = SHAPE_HOR;
				    shape_table[idx].percentage = n;
				}
			    }
			    else if (round == 2)
			    {
				if (len == 9)
				    shape_table[idx].blinkwait = n;
				else if (len == 7)
				    shape_table[idx].blinkon = n;
				else
				    shape_table[idx].blinkoff = n;
			    }
			}
			else if (STRNICMP(p, "block", 5) == 0)
			{
			    if (round == 2)
				shape_table[idx].shape = SHAPE_BLOCK;
			    p += 5;
			}
			else	// must be a highlight group name then
			{
			    endp = vim_strchr(p, '-');
			    if (commap == NULL)		    // last part
			    {
				if (endp == NULL)
				    endp = p + STRLEN(p);   // find end of part
			    }
			    else if (endp > commap || endp == NULL)
				endp = commap;
			    slashp = vim_strchr(p, '/');
			    if (slashp != NULL && slashp < endp)
			    {
				// "group/langmap_group"
				i = syn_check_group(p, (int)(slashp - p));
				p = slashp + 1;
			    }
			    if (round == 2)
			    {
				shape_table[idx].id = syn_check_group(p,
							     (int)(endp - p));
				shape_table[idx].id_lm = shape_table[idx].id;
				if (slashp != NULL && slashp < endp)
				    shape_table[idx].id = i;
			    }
			    p = endp;
			}
		    } // if (what != SHAPE_MOUSE)

		    if (*p == '-')
			++p;
		}
	    }
	    modep = p;
	    if (*modep == ',')
		++modep;
	}
    }

    // If the 's' flag is not given, use the 'v' cursor for 's'
    if (!found_ve)
    {
#ifdef FEAT_MOUSESHAPE
	if (what == SHAPE_MOUSE)
	{
	    shape_table[SHAPE_IDX_VE].mshape = shape_table[SHAPE_IDX_V].mshape;
	}
	else
#endif
	{
	    shape_table[SHAPE_IDX_VE].shape = shape_table[SHAPE_IDX_V].shape;
	    shape_table[SHAPE_IDX_VE].percentage =
					 shape_table[SHAPE_IDX_V].percentage;
	    shape_table[SHAPE_IDX_VE].blinkwait =
					  shape_table[SHAPE_IDX_V].blinkwait;
	    shape_table[SHAPE_IDX_VE].blinkon =
					    shape_table[SHAPE_IDX_V].blinkon;
	    shape_table[SHAPE_IDX_VE].blinkoff =
					   shape_table[SHAPE_IDX_V].blinkoff;
	    shape_table[SHAPE_IDX_VE].id = shape_table[SHAPE_IDX_V].id;
	    shape_table[SHAPE_IDX_VE].id_lm = shape_table[SHAPE_IDX_V].id_lm;
	}
    }

    return NULL;
}

# if defined(MCH_CURSOR_SHAPE) || defined(FEAT_GUI) \
	|| defined(FEAT_MOUSESHAPE) || defined(PROTO)
/*
 * Return the index into shape_table[] for the current mode.
 * When "mouse" is TRUE, consider indexes valid for the mouse pointer.
 */
    int
get_shape_idx(int mouse)
{
#ifdef FEAT_MOUSESHAPE
    if (mouse && (State == MODE_HITRETURN || State == MODE_ASKMORE))
    {
# ifdef FEAT_GUI
	int x, y;
	gui_mch_getmouse(&x, &y);
	if (Y_2_ROW(y) == Rows - 1)
	    return SHAPE_IDX_MOREL;
# endif
	return SHAPE_IDX_MORE;
    }
    if (mouse && drag_status_line)
	return SHAPE_IDX_SDRAG;
    if (mouse && drag_sep_line)
	return SHAPE_IDX_VDRAG;
#endif
    if (!mouse && State == MODE_SHOWMATCH)
	return SHAPE_IDX_SM;
    if (State & VREPLACE_FLAG)
	return SHAPE_IDX_R;
    if (State & REPLACE_FLAG)
	return SHAPE_IDX_R;
    if (State & MODE_INSERT)
	return SHAPE_IDX_I;
    if (State & MODE_CMDLINE)
    {
	if (cmdline_at_end())
	    return SHAPE_IDX_C;
	if (cmdline_overstrike())
	    return SHAPE_IDX_CR;
	return SHAPE_IDX_CI;
    }
    if (finish_op)
	return SHAPE_IDX_O;
    if (VIsual_active)
    {
	if (*p_sel == 'e')
	    return SHAPE_IDX_VE;
	else
	    return SHAPE_IDX_V;
    }
    return SHAPE_IDX_N;
}
#endif

# if defined(FEAT_MOUSESHAPE) || defined(PROTO)
static int current_mouse_shape = 0;

/*
 * Set the mouse shape:
 * If "shape" is -1, use shape depending on the current mode,
 * depending on the current state.
 * If "shape" is -2, only update the shape when it's CLINE or STATUS (used
 * when the mouse moves off the status or command line).
 */
    void
update_mouseshape(int shape_idx)
{
    int new_mouse_shape;

    // Only works in GUI mode.
    if (!gui.in_use || gui.starting)
	return;

    // Postpone the updating when more is to come.  Speeds up executing of
    // mappings.
    if (shape_idx == -1 && char_avail())
    {
	postponed_mouseshape = TRUE;
	return;
    }

    // When ignoring the mouse don't change shape on the statusline.
    if (*p_mouse == NUL
	    && (shape_idx == SHAPE_IDX_CLINE
		|| shape_idx == SHAPE_IDX_STATUS
		|| shape_idx == SHAPE_IDX_VSEP))
	shape_idx = -2;

    if (shape_idx == -2
	    && current_mouse_shape != shape_table[SHAPE_IDX_CLINE].mshape
	    && current_mouse_shape != shape_table[SHAPE_IDX_STATUS].mshape
	    && current_mouse_shape != shape_table[SHAPE_IDX_VSEP].mshape)
	return;
    if (shape_idx < 0)
	new_mouse_shape = shape_table[get_shape_idx(TRUE)].mshape;
    else
	new_mouse_shape = shape_table[shape_idx].mshape;
    if (new_mouse_shape != current_mouse_shape)
    {
	mch_set_mouse_shape(new_mouse_shape);
	current_mouse_shape = new_mouse_shape;
    }
    postponed_mouseshape = FALSE;
}
# endif

#endif // CURSOR_SHAPE

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Mainly for tests: get the name of the current mouse shape.
 */
    void
f_getmouseshape(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;
# if defined(FEAT_MOUSESHAPE) || defined(PROTO)
    if (current_mouse_shape >= 0
			      && current_mouse_shape < (int)MSHAPE_NAMES_COUNT)
	rettv->vval.v_string = vim_strsave(
				  (char_u *)mshape_names[current_mouse_shape]);
# endif
}
#endif



/*
 * Change directory to "new_dir".  Search 'cdpath' for relative directory
 * names.
 */
    int
vim_chdir(char_u *new_dir)
{
    char_u	*dir_name;
    int		r;
    char_u	*file_to_find = NULL;
    char	*search_ctx = NULL;

    dir_name = find_directory_in_path(new_dir, (int)STRLEN(new_dir),
		     FNAME_MESS, curbuf->b_ffname, &file_to_find, &search_ctx);
    vim_free(file_to_find);
    vim_findfile_cleanup(search_ctx);
    if (dir_name == NULL)
	return -1;
    r = mch_chdir((char *)dir_name);
    vim_free(dir_name);
    return r;
}

/*
 * Get user name from machine-specific function.
 * Returns the user name in "buf[len]".
 * Some systems are quite slow in obtaining the user name (Windows NT), thus
 * cache the result.
 * Returns OK or FAIL.
 */
    int
get_user_name(char_u *buf, int len)
{
    if (username == NULL)
    {
	if (mch_get_user_name(buf, len) == FAIL)
	    return FAIL;
	username = vim_strsave(buf);
    }
    else
	vim_strncpy(buf, username, len - 1);
    return OK;
}

#if defined(EXITFREE) || defined(PROTO)
/*
 * Free the memory allocated by get_user_name()
 */
    void
free_username(void)
{
    vim_free(username);
}
#endif

#ifndef HAVE_QSORT
/*
 * Our own qsort(), for systems that don't have it.
 * It's simple and slow.  From the K&R C book.
 */
    void
qsort(
    void	*base,
    size_t	elm_count,
    size_t	elm_size,
    int (*cmp)(const void *, const void *))
{
    char_u	*buf;
    char_u	*p1;
    char_u	*p2;
    int		i, j;
    int		gap;

    buf = alloc(elm_size);
    if (buf == NULL)
	return;

    for (gap = elm_count / 2; gap > 0; gap /= 2)
	for (i = gap; i < elm_count; ++i)
	    for (j = i - gap; j >= 0; j -= gap)
	    {
		// Compare the elements.
		p1 = (char_u *)base + j * elm_size;
		p2 = (char_u *)base + (j + gap) * elm_size;
		if ((*cmp)((void *)p1, (void *)p2) <= 0)
		    break;
		// Exchange the elements.
		mch_memmove(buf, p1, elm_size);
		mch_memmove(p1, p2, elm_size);
		mch_memmove(p2, buf, elm_size);
	    }

    vim_free(buf);
}
#endif

/*
 * The putenv() implementation below comes from the "screen" program.
 * Included with permission from Juergen Weigert.
 * See pty.c for the copyright notice.
 */

/*
 *  putenv  --	put value into environment
 *
 *  Usage:  i = putenv (string)
 *    int i;
 *    char  *string;
 *
 *  where string is of the form <name>=<value>.
 *  Putenv returns 0 normally, -1 on error (not enough core for malloc).
 *
 *  Putenv may need to add a new name into the environment, or to
 *  associate a value longer than the current value with a particular
 *  name.  So, to make life simpler, putenv() copies your entire
 *  environment into the heap (i.e. malloc()) from the stack
 *  (i.e. where it resides when your process is initiated) the first
 *  time you call it.
 *
 *  (history removed, not very interesting.  See the "screen" sources.)
 */

#if !defined(HAVE_SETENV) && !defined(HAVE_PUTENV)

#define EXTRASIZE 5		// increment to add to env. size

static int  envsize = -1;	// current size of environment
extern char **environ;		// the global which is your env.

static int  findenv(char *name); // look for a name in the env.
static int  newenv(void);	// copy env. from stack to heap
static int  moreenv(void);	// incr. size of env.

    int
putenv(const char *string)
{
    int	    i;
    char    *p;

    if (envsize < 0)
    {				// first time putenv called
	if (newenv() < 0)	// copy env. to heap
	    return -1;
    }

    i = findenv((char *)string); // look for name in environment

    if (i < 0)
    {				// name must be added
	for (i = 0; environ[i]; i++);
	if (i >= (envsize - 1))
	{			// need new slot
	    if (moreenv() < 0)
		return -1;
	}
	p = alloc(strlen(string) + 1);
	if (p == NULL)		// not enough core
	    return -1;
	environ[i + 1] = 0;	// new end of env.
    }
    else
    {				// name already in env.
	p = vim_realloc(environ[i], strlen(string) + 1);
	if (p == NULL)
	    return -1;
    }
    sprintf(p, "%s", string);	// copy into env.
    environ[i] = p;

    return 0;
}

    static int
findenv(char *name)
{
    char    *namechar, *envchar;
    int	    i, found;

    found = 0;
    for (i = 0; environ[i] && !found; i++)
    {
	envchar = environ[i];
	namechar = name;
	while (*namechar && *namechar != '=' && (*namechar == *envchar))
	{
	    namechar++;
	    envchar++;
	}
	found = ((*namechar == '\0' || *namechar == '=') && *envchar == '=');
    }
    return found ? i - 1 : -1;
}

    static int
newenv(void)
{
    char    **env, *elem;
    int	    i, esize;

    for (i = 0; environ[i]; i++)
	;

    esize = i + EXTRASIZE + 1;
    env = ALLOC_MULT(char *, esize);
    if (env == NULL)
	return -1;

    for (i = 0; environ[i]; i++)
    {
	elem = alloc(strlen(environ[i]) + 1);
	if (elem == NULL)
	    return -1;
	env[i] = elem;
	strcpy(elem, environ[i]);
    }

    env[i] = 0;
    environ = env;
    envsize = esize;
    return 0;
}

    static int
moreenv(void)
{
    int	    esize;
    char    **env;

    esize = envsize + EXTRASIZE;
    env = vim_realloc((char *)environ, esize * sizeof (*env));
    if (env == 0)
	return -1;
    environ = env;
    envsize = esize;
    return 0;
}

# ifdef USE_VIMPTY_GETENV
/*
 * Used for mch_getenv() for Mac.
 */
    char_u *
vimpty_getenv(const char_u *string)
{
    int i;
    char_u *p;

    if (envsize < 0)
	return NULL;

    i = findenv((char *)string);

    if (i < 0)
	return NULL;

    p = vim_strchr((char_u *)environ[i], '=');
    return (p + 1);
}
# endif

#endif // !defined(HAVE_SETENV) && !defined(HAVE_PUTENV)

#if defined(FEAT_EVAL) || defined(FEAT_SPELL) || defined(PROTO)
/*
 * Return 0 for not writable, 1 for writable file, 2 for a dir which we have
 * rights to write into.
 */
    int
filewritable(char_u *fname)
{
    int		retval = 0;
#if defined(UNIX) || defined(VMS)
    int		perm = 0;
#endif

#if defined(UNIX) || defined(VMS)
    perm = mch_getperm(fname);
#endif
    if (
# ifdef MSWIN
	    mch_writable(fname) &&
# else
# if defined(UNIX) || defined(VMS)
	    (perm & 0222) &&
#  endif
# endif
	    mch_access((char *)fname, W_OK) == 0
       )
    {
	++retval;
	if (mch_isdir(fname))
	    ++retval;
    }
    return retval;
}
#endif

#if defined(FEAT_SPELL) || defined(FEAT_PERSISTENT_UNDO) || defined(PROTO)
/*
 * Read 2 bytes from "fd" and turn them into an int, MSB first.
 * Returns -1 when encountering EOF.
 */
    int
get2c(FILE *fd)
{
    int		c, n;

    n = getc(fd);
    if (n == EOF) return -1;
    c = getc(fd);
    if (c == EOF) return -1;
    return (n << 8) + c;
}

/*
 * Read 3 bytes from "fd" and turn them into an int, MSB first.
 * Returns -1 when encountering EOF.
 */
    int
get3c(FILE *fd)
{
    int		c, n;

    n = getc(fd);
    if (n == EOF) return -1;
    c = getc(fd);
    if (c == EOF) return -1;
    n = (n << 8) + c;
    c = getc(fd);
    if (c == EOF) return -1;
    return (n << 8) + c;
}

/*
 * Read 4 bytes from "fd" and turn them into an int, MSB first.
 * Returns -1 when encountering EOF.
 */
    int
get4c(FILE *fd)
{
    int		c;
    // Use unsigned rather than int otherwise result is undefined
    // when left-shift sets the MSB.
    unsigned	n;

    c = getc(fd);
    if (c == EOF) return -1;
    n = (unsigned)c;
    c = getc(fd);
    if (c == EOF) return -1;
    n = (n << 8) + (unsigned)c;
    c = getc(fd);
    if (c == EOF) return -1;
    n = (n << 8) + (unsigned)c;
    c = getc(fd);
    if (c == EOF) return -1;
    n = (n << 8) + (unsigned)c;
    return (int)n;
}

/*
 * Read a string of length "cnt" from "fd" into allocated memory.
 * Returns NULL when out of memory or unable to read that many bytes.
 */
    char_u *
read_string(FILE *fd, int cnt)
{
    char_u	*str;
    int		i;
    int		c;

    // allocate memory
    str = alloc(cnt + 1);
    if (str == NULL)
	return NULL;

    // Read the string.  Quit when running into the EOF.
    for (i = 0; i < cnt; ++i)
    {
	c = getc(fd);
	if (c == EOF)
	{
	    vim_free(str);
	    return NULL;
	}
	str[i] = c;
    }
    str[i] = NUL;
    return str;
}

/*
 * Write a number to file "fd", MSB first, in "len" bytes.
 */
    int
put_bytes(FILE *fd, long_u nr, int len)
{
    int	    i;

    for (i = len - 1; i >= 0; --i)
	if (putc((int)(nr >> (i * 8)), fd) == EOF)
	    return FAIL;
    return OK;
}

#endif

#ifndef PROTO  // proto is defined in vim.h
# ifdef ELAPSED_TIMEVAL
/*
 * Return time in msec since "start_tv".
 */
    long
elapsed(struct timeval *start_tv)
{
    struct timeval  now_tv;

    gettimeofday(&now_tv, NULL);
    return (now_tv.tv_sec - start_tv->tv_sec) * 1000L
	 + (now_tv.tv_usec - start_tv->tv_usec) / 1000L;
}
# endif

# ifdef ELAPSED_TICKCOUNT
/*
 * Return time in msec since "start_tick".
 */
    long
elapsed(DWORD start_tick)
{
    DWORD	now = GetTickCount();

    return (long)now - (long)start_tick;
}
# endif
#endif

#if defined(FEAT_JOB_CHANNEL) \
	|| (defined(UNIX) && (!defined(USE_SYSTEM) \
	|| (defined(FEAT_GUI) && defined(FEAT_TERMINAL)))) \
	|| defined(PROTO)
/*
 * Parse "cmd" and put the white-separated parts in "argv".
 * "argv" is an allocated array with "argc" entries and room for 4 more.
 * Returns FAIL when out of memory.
 */
    int
mch_parse_cmd(char_u *cmd, int use_shcf, char ***argv, int *argc)
{
    int		i;
    char_u	*p, *d;
    int		inquote;

    /*
     * Do this loop twice:
     * 1: find number of arguments
     * 2: separate them and build argv[]
     */
    for (i = 1; i <= 2; ++i)
    {
	p = skipwhite(cmd);
	inquote = FALSE;
	*argc = 0;
	while (*p != NUL)
	{
	    if (i == 2)
		(*argv)[*argc] = (char *)p;
	    ++*argc;
	    d = p;
	    while (*p != NUL && (inquote || (*p != ' ' && *p != TAB)))
	    {
		if (p[0] == '"')
		    // quotes surrounding an argument and are dropped
		    inquote = !inquote;
		else
		{
		    if (rem_backslash(p))
		    {
			// First pass: skip over "\ " and "\"".
			// Second pass: Remove the backslash.
			++p;
		    }
		    if (i == 2)
			*d++ = *p;
		}
		++p;
	    }
	    if (*p == NUL)
	    {
		if (i == 2)
		    *d++ = NUL;
		break;
	    }
	    if (i == 2)
		*d++ = NUL;
	    p = skipwhite(p + 1);
	}
	if (*argv == NULL)
	{
	    if (use_shcf)
	    {
		// Account for possible multiple args in p_shcf.
		p = p_shcf;
		for (;;)
		{
		    p = skiptowhite(p);
		    if (*p == NUL)
			break;
		    ++*argc;
		    p = skipwhite(p);
		}
	    }

	    *argv = ALLOC_MULT(char *, *argc + 4);
	    if (*argv == NULL)	    // out of memory
		return FAIL;
	}
    }
    return OK;
}

/*
 * Build "argv[argc]" from the string "cmd".
 * "argv[argc]" is set to NULL;
 * Return FAIL when out of memory.
 */
    int
build_argv_from_string(char_u *cmd, char ***argv, int *argc)
{
    char_u	*cmd_copy;
    int		i;

    // Make a copy, parsing will modify "cmd".
    cmd_copy = vim_strsave(cmd);
    if (cmd_copy == NULL
	    || mch_parse_cmd(cmd_copy, FALSE, argv, argc) == FAIL)
    {
	vim_free(cmd_copy);
	return FAIL;
    }
    for (i = 0; i < *argc; i++)
	(*argv)[i] = (char *)vim_strsave((char_u *)(*argv)[i]);
    (*argv)[*argc] = NULL;
    vim_free(cmd_copy);
    return OK;
}

# if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
/*
 * Build "argv[argc]" from the list "l".
 * "argv[argc]" is set to NULL;
 * Return FAIL when out of memory.
 */
    int
build_argv_from_list(list_T *l, char ***argv, int *argc)
{
    listitem_T  *li;
    char_u	*s;

    // Pass argv[] to mch_call_shell().
    *argv = ALLOC_MULT(char *, l->lv_len + 1);
    if (*argv == NULL)
	return FAIL;
    *argc = 0;
    FOR_ALL_LIST_ITEMS(l, li)
    {
	s = tv_get_string_chk(&li->li_tv);
	if (s == NULL)
	{
	    int i;

	    for (i = 0; i < *argc; ++i)
		VIM_CLEAR((*argv)[i]);
	    (*argv)[0] = NULL;
	    return FAIL;
	}
	(*argv)[*argc] = (char *)vim_strsave(s);
	*argc += 1;
    }
    (*argv)[*argc] = NULL;
    return OK;
}
# endif
#endif

/*
 * Change the behavior of vterm.
 * 0: As usual.
 * 1: Windows 10 version 1809
 *      The bug causes unstable handling of ambiguous width character.
 * 2: Windows 10 version 1903 & 1909
 *      Use the wrong result because each result is different.
 * 3: Windows 10 insider preview (current latest logic)
 */
    int
get_special_pty_type(void)
{
#ifdef MSWIN
    return get_conpty_type();
#else
    return 0;
#endif
}
