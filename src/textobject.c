/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * textobject.c: functions for text objects
 */
#include "vim.h"

static int cls(void);
static int skip_chars(int, int);

/*
 * Find the start of the next sentence, searching in the direction specified
 * by the "dir" argument.  The cursor is positioned on the start of the next
 * sentence when found.  If the next sentence is found, return OK.  Return FAIL
 * otherwise.  See ":h sentence" for the precise definition of a "sentence"
 * text object.
 */
    int
findsent(int dir, long count)
{
    pos_T	pos, tpos;
    pos_T	prev_pos;
    int		c;
    int		(*func)(pos_T *);
    int		startlnum;
    int		noskip = FALSE;	    // do not skip blanks
    int		cpo_J;
    int		found_dot;

    pos = curwin->w_cursor;
    if (dir == FORWARD)
	func = incl;
    else
	func = decl;

    while (count--)
    {
	prev_pos = pos;

	/*
	 * if on an empty line, skip up to a non-empty line
	 */
	if (gchar_pos(&pos) == NUL)
	{
	    do
		if ((*func)(&pos) == -1)
		    break;
	    while (gchar_pos(&pos) == NUL);
	    if (dir == FORWARD)
		goto found;
	}
	/*
	 * if on the start of a paragraph or a section and searching forward,
	 * go to the next line
	 */
	else if (dir == FORWARD && pos.col == 0 &&
						startPS(pos.lnum, NUL, FALSE))
	{
	    if (pos.lnum == curbuf->b_ml.ml_line_count)
		return FAIL;
	    ++pos.lnum;
	    goto found;
	}
	else if (dir == BACKWARD)
	    decl(&pos);

	// go back to the previous non-white non-punctuation character
	found_dot = FALSE;
	while (c = gchar_pos(&pos), VIM_ISWHITE(c)
				|| vim_strchr((char_u *)".!?)]\"'", c) != NULL)
	{
	    tpos = pos;
	    if (decl(&tpos) == -1 || (LINEEMPTY(tpos.lnum) && dir == FORWARD))
		break;

	    if (found_dot)
		break;
	    if (vim_strchr((char_u *) ".!?", c) != NULL)
		found_dot = TRUE;

	    if (vim_strchr((char_u *) ")]\"'", c) != NULL
		&& vim_strchr((char_u *) ".!?)]\"'", gchar_pos(&tpos)) == NULL)
		break;

	    decl(&pos);
	}

	// remember the line where the search started
	startlnum = pos.lnum;
	cpo_J = vim_strchr(p_cpo, CPO_ENDOFSENT) != NULL;

	for (;;)		// find end of sentence
	{
	    c = gchar_pos(&pos);
	    if (c == NUL || (pos.col == 0 && startPS(pos.lnum, NUL, FALSE)))
	    {
		if (dir == BACKWARD && pos.lnum != startlnum)
		    ++pos.lnum;
		break;
	    }
	    if (c == '.' || c == '!' || c == '?')
	    {
		tpos = pos;
		do
		    if ((c = inc(&tpos)) == -1)
			break;
		while (vim_strchr((char_u *)")]\"'", c = gchar_pos(&tpos))
			!= NULL);
		if (c == -1  || (!cpo_J && (c == ' ' || c == '\t')) || c == NUL
		    || (cpo_J && (c == ' ' && inc(&tpos) >= 0
			      && gchar_pos(&tpos) == ' ')))
		{
		    pos = tpos;
		    if (gchar_pos(&pos) == NUL) // skip NUL at EOL
			inc(&pos);
		    break;
		}
	    }
	    if ((*func)(&pos) == -1)
	    {
		if (count)
		    return FAIL;
		noskip = TRUE;
		break;
	    }
	}
found:
	    // skip white space
	while (!noskip && ((c = gchar_pos(&pos)) == ' ' || c == '\t'))
	    if (incl(&pos) == -1)
		break;

	if (EQUAL_POS(prev_pos, pos))
	{
	    // didn't actually move, advance one character and try again
	    if ((*func)(&pos) == -1)
	    {
		if (count)
		    return FAIL;
		break;
	    }
	    ++count;
	}
    }

    setpcmark();
    curwin->w_cursor = pos;
    return OK;
}

/*
 * Find the next paragraph or section in direction 'dir'.
 * Paragraphs are currently supposed to be separated by empty lines.
 * If 'what' is NUL we go to the next paragraph.
 * If 'what' is '{' or '}' we go to the next section.
 * If 'both' is TRUE also stop at '}'.
 * Return TRUE if the next paragraph or section was found.
 */
    int
findpar(
    int		*pincl,	    // Return: TRUE if last char is to be included
    int		dir,
    long	count,
    int		what,
    int		both)
{
    linenr_T	curr;
    int		did_skip;   // TRUE after separating lines have been skipped
    int		first;	    // TRUE on first line
    int		posix = (vim_strchr(p_cpo, CPO_PARA) != NULL);
#ifdef FEAT_FOLDING
    linenr_T	fold_first;	// first line of a closed fold
    linenr_T	fold_last;	// last line of a closed fold
    int		fold_skipped;	// TRUE if a closed fold was skipped this
				// iteration
#endif

    curr = curwin->w_cursor.lnum;

    while (count--)
    {
	did_skip = FALSE;
	for (first = TRUE; ; first = FALSE)
	{
	    if (*ml_get(curr) != NUL)
		did_skip = TRUE;

#ifdef FEAT_FOLDING
	    // skip folded lines
	    fold_skipped = FALSE;
	    if (first && hasFolding(curr, &fold_first, &fold_last))
	    {
		curr = ((dir > 0) ? fold_last : fold_first) + dir;
		fold_skipped = TRUE;
	    }
#endif

	    // POSIX has its own ideas of what a paragraph boundary is and it
	    // doesn't match historical Vi: It also stops at a "{" in the
	    // first column and at an empty line.
	    if (!first && did_skip && (startPS(curr, what, both)
			   || (posix && what == NUL && *ml_get(curr) == '{')))
		break;

#ifdef FEAT_FOLDING
	    if (fold_skipped)
		curr -= dir;
#endif
	    if ((curr += dir) < 1 || curr > curbuf->b_ml.ml_line_count)
	    {
		if (count)
		    return FALSE;
		curr -= dir;
		break;
	    }
	}
    }
    setpcmark();
    if (both && *ml_get(curr) == '}')	// include line with '}'
	++curr;
    curwin->w_cursor.lnum = curr;
    if (curr == curbuf->b_ml.ml_line_count && what != '}' && dir == FORWARD)
    {
	char_u *line = ml_get(curr);

	// Put the cursor on the last character in the last line and make the
	// motion inclusive.
	if ((curwin->w_cursor.col = (colnr_T)STRLEN(line)) != 0)
	{
	    --curwin->w_cursor.col;
	    curwin->w_cursor.col -=
			     (*mb_head_off)(line, line + curwin->w_cursor.col);
	    *pincl = TRUE;
	}
    }
    else
	curwin->w_cursor.col = 0;
    return TRUE;
}

/*
 * check if the string 's' is a nroff macro that is in option 'opt'
 */
    static int
inmacro(char_u *opt, char_u *s)
{
    char_u	*macro;

    for (macro = opt; macro[0]; ++macro)
    {
	// Accept two characters in the option being equal to two characters
	// in the line.  A space in the option matches with a space in the
	// line or the line having ended.
	if (       (macro[0] == s[0]
		    || (macro[0] == ' '
			&& (s[0] == NUL || s[0] == ' ')))
		&& (macro[1] == s[1]
		    || ((macro[1] == NUL || macro[1] == ' ')
			&& (s[0] == NUL || s[1] == NUL || s[1] == ' '))))
	    break;
	++macro;
	if (macro[0] == NUL)
	    break;
    }
    return (macro[0] != NUL);
}

/*
 * startPS: return TRUE if line 'lnum' is the start of a section or paragraph.
 * If 'para' is '{' or '}' only check for sections.
 * If 'both' is TRUE also stop at '}'
 */
    int
startPS(linenr_T lnum, int para, int both)
{
    char_u	*s;

    s = ml_get(lnum);
    if (*s == para || *s == '\f' || (both && *s == '}'))
	return TRUE;
    if (*s == '.' && (inmacro(p_sections, s + 1) ||
					   (!para && inmacro(p_para, s + 1))))
	return TRUE;
    return FALSE;
}

/*
 * The following routines do the word searches performed by the 'w', 'W',
 * 'b', 'B', 'e', and 'E' commands.
 */

/*
 * To perform these searches, characters are placed into one of three
 * classes, and transitions between classes determine word boundaries.
 *
 * The classes are:
 *
 * 0 - white space
 * 1 - punctuation
 * 2 or higher - keyword characters (letters, digits and underscore)
 */

static int	cls_bigword;	// TRUE for "W", "B" or "E"

/*
 * cls() - returns the class of character at curwin->w_cursor
 *
 * If a 'W', 'B', or 'E' motion is being done (cls_bigword == TRUE), chars
 * from class 2 and higher are reported as class 1 since only white space
 * boundaries are of interest.
 */
    static int
cls(void)
{
    int	    c;

    c = gchar_cursor();
    if (c == ' ' || c == '\t' || c == NUL)
	return 0;
    if (enc_dbcs != 0 && c > 0xFF)
    {
	// If cls_bigword, report multi-byte chars as class 1.
	if (enc_dbcs == DBCS_KOR && cls_bigword)
	    return 1;

	// process code leading/trailing bytes
	return dbcs_class(((unsigned)c >> 8), (c & 0xFF));
    }
    if (enc_utf8)
    {
	c = utf_class(c);
	if (c != 0 && cls_bigword)
	    return 1;
	return c;
    }

    // If cls_bigword is TRUE, report all non-blanks as class 1.
    if (cls_bigword)
	return 1;

    if (vim_iswordc(c))
	return 2;
    return 1;
}


/*
 * fwd_word(count, type, eol) - move forward one word
 *
 * Returns FAIL if the cursor was already at the end of the file.
 * If eol is TRUE, last word stops at end of line (for operators).
 */
    int
fwd_word(
    long	count,
    int		bigword,    // "W", "E" or "B"
    int		eol)
{
    int		sclass;	    // starting class
    int		i;
    int		last_line;

    curwin->w_cursor.coladd = 0;
    cls_bigword = bigword;
    while (--count >= 0)
    {
#ifdef FEAT_FOLDING
	// When inside a range of folded lines, move to the last char of the
	// last line.
	if (hasFolding(curwin->w_cursor.lnum, NULL, &curwin->w_cursor.lnum))
	    coladvance((colnr_T)MAXCOL);
#endif
	sclass = cls();

	/*
	 * We always move at least one character, unless on the last
	 * character in the buffer.
	 */
	last_line = (curwin->w_cursor.lnum == curbuf->b_ml.ml_line_count);
	i = inc_cursor();
	if (i == -1 || (i >= 1 && last_line)) // started at last char in file
	    return FAIL;
	if (i >= 1 && eol && count == 0)      // started at last char in line
	    return OK;

	/*
	 * Go one char past end of current word (if any)
	 */
	if (sclass != 0)
	    while (cls() == sclass)
	    {
		i = inc_cursor();
		if (i == -1 || (i >= 1 && eol && count == 0))
		    return OK;
	    }

	/*
	 * go to next non-white
	 */
	while (cls() == 0)
	{
	    /*
	     * We'll stop if we land on a blank line
	     */
	    if (curwin->w_cursor.col == 0 && *ml_get_curline() == NUL)
		break;

	    i = inc_cursor();
	    if (i == -1 || (i >= 1 && eol && count == 0))
		return OK;
	}
    }
    return OK;
}

/*
 * bck_word() - move backward 'count' words
 *
 * If stop is TRUE and we are already on the start of a word, move one less.
 *
 * Returns FAIL if top of the file was reached.
 */
    int
bck_word(long count, int bigword, int stop)
{
    int		sclass;	    // starting class

    curwin->w_cursor.coladd = 0;
    cls_bigword = bigword;
    while (--count >= 0)
    {
#ifdef FEAT_FOLDING
	// When inside a range of folded lines, move to the first char of the
	// first line.
	if (hasFolding(curwin->w_cursor.lnum, &curwin->w_cursor.lnum, NULL))
	    curwin->w_cursor.col = 0;
#endif
	sclass = cls();
	if (dec_cursor() == -1)		// started at start of file
	    return FAIL;

	if (!stop || sclass == cls() || sclass == 0)
	{
	    /*
	     * Skip white space before the word.
	     * Stop on an empty line.
	     */
	    while (cls() == 0)
	    {
		if (curwin->w_cursor.col == 0
				      && LINEEMPTY(curwin->w_cursor.lnum))
		    goto finished;
		if (dec_cursor() == -1) // hit start of file, stop here
		    return OK;
	    }

	    /*
	     * Move backward to start of this word.
	     */
	    if (skip_chars(cls(), BACKWARD))
		return OK;
	}

	inc_cursor();			// overshot - forward one
finished:
	stop = FALSE;
    }
    adjust_skipcol();
    return OK;
}

/*
 * end_word() - move to the end of the word
 *
 * There is an apparent bug in the 'e' motion of the real vi. At least on the
 * System V Release 3 version for the 80386. Unlike 'b' and 'w', the 'e'
 * motion crosses blank lines. When the real vi crosses a blank line in an
 * 'e' motion, the cursor is placed on the FIRST character of the next
 * non-blank line. The 'E' command, however, works correctly. Since this
 * appears to be a bug, I have not duplicated it here.
 *
 * Returns FAIL if end of the file was reached.
 *
 * If stop is TRUE and we are already on the end of a word, move one less.
 * If empty is TRUE stop on an empty line.
 */
    int
end_word(
    long	count,
    int		bigword,
    int		stop,
    int		empty)
{
    int		sclass;	    // starting class

    curwin->w_cursor.coladd = 0;
    cls_bigword = bigword;
    while (--count >= 0)
    {
#ifdef FEAT_FOLDING
	// When inside a range of folded lines, move to the last char of the
	// last line.
	if (hasFolding(curwin->w_cursor.lnum, NULL, &curwin->w_cursor.lnum))
	    coladvance((colnr_T)MAXCOL);
#endif
	sclass = cls();
	if (inc_cursor() == -1)
	    return FAIL;

	/*
	 * If we're in the middle of a word, we just have to move to the end
	 * of it.
	 */
	if (cls() == sclass && sclass != 0)
	{
	    /*
	     * Move forward to end of the current word
	     */
	    if (skip_chars(sclass, FORWARD))
		return FAIL;
	}
	else if (!stop || sclass == 0)
	{
	    /*
	     * We were at the end of a word. Go to the end of the next word.
	     * First skip white space, if 'empty' is TRUE, stop at empty line.
	     */
	    while (cls() == 0)
	    {
		if (empty && curwin->w_cursor.col == 0
					  && LINEEMPTY(curwin->w_cursor.lnum))
		    goto finished;
		if (inc_cursor() == -1)	    // hit end of file, stop here
		    return FAIL;
	    }

	    /*
	     * Move forward to the end of this word.
	     */
	    if (skip_chars(cls(), FORWARD))
		return FAIL;
	}
	dec_cursor();			// overshot - one char backward
finished:
	stop = FALSE;			// we move only one word less
    }
    return OK;
}

/*
 * Move back to the end of the word.
 *
 * Returns FAIL if start of the file was reached.
 */
    int
bckend_word(
    long	count,
    int		bigword,    // TRUE for "B"
    int		eol)	    // TRUE: stop at end of line.
{
    int		sclass;	    // starting class
    int		i;

    curwin->w_cursor.coladd = 0;
    cls_bigword = bigword;
    while (--count >= 0)
    {
	sclass = cls();
	if ((i = dec_cursor()) == -1)
	    return FAIL;
	if (eol && i == 1)
	    return OK;

	/*
	 * Move backward to before the start of this word.
	 */
	if (sclass != 0)
	{
	    while (cls() == sclass)
		if ((i = dec_cursor()) == -1 || (eol && i == 1))
		    return OK;
	}

	/*
	 * Move backward to end of the previous word
	 */
	while (cls() == 0)
	{
	    if (curwin->w_cursor.col == 0 && LINEEMPTY(curwin->w_cursor.lnum))
		break;
	    if ((i = dec_cursor()) == -1 || (eol && i == 1))
		return OK;
	}
    }
    adjust_skipcol();
    return OK;
}

/*
 * Skip a row of characters of the same class.
 * Return TRUE when end-of-file reached, FALSE otherwise.
 */
    static int
skip_chars(int cclass, int dir)
{
    while (cls() == cclass)
	if ((dir == FORWARD ? inc_cursor() : dec_cursor()) == -1)
	    return TRUE;
    return FALSE;
}

/*
 * Go back to the start of the word or the start of white space
 */
    static void
back_in_line(void)
{
    int		sclass;		    // starting class

    sclass = cls();
    for (;;)
    {
	if (curwin->w_cursor.col == 0)	    // stop at start of line
	    break;
	dec_cursor();
	if (cls() != sclass)		    // stop at start of word
	{
	    inc_cursor();
	    break;
	}
    }
}

    static void
find_first_blank(pos_T *posp)
{
    int	    c;

    while (decl(posp) != -1)
    {
	c = gchar_pos(posp);
	if (!VIM_ISWHITE(c))
	{
	    incl(posp);
	    break;
	}
    }
}

/*
 * Skip count/2 sentences and count/2 separating white spaces.
 */
    static void
findsent_forward(
    long    count,
    int	    at_start_sent)	// cursor is at start of sentence
{
    while (count--)
    {
	findsent(FORWARD, 1L);
	if (at_start_sent)
	    find_first_blank(&curwin->w_cursor);
	if (count == 0 || at_start_sent)
	    decl(&curwin->w_cursor);
	at_start_sent = !at_start_sent;
    }
}

/*
 * Find word under cursor, cursor at end.
 * Used while an operator is pending, and in Visual mode.
 */
    int
current_word(
    oparg_T	*oap,
    long	count,
    int		include,	// TRUE: include word and white space
    int		bigword)	// FALSE == word, TRUE == WORD
{
    pos_T	start_pos;
    pos_T	pos;
    int		inclusive = TRUE;
    int		include_white = FALSE;

    cls_bigword = bigword;
    CLEAR_POS(&start_pos);

    // Correct cursor when 'selection' is exclusive
    if (VIsual_active && *p_sel == 'e' && LT_POS(VIsual, curwin->w_cursor))
	dec_cursor();

    /*
     * When Visual mode is not active, or when the VIsual area is only one
     * character, select the word and/or white space under the cursor.
     */
    if (!VIsual_active || EQUAL_POS(curwin->w_cursor, VIsual))
    {
	/*
	 * Go to start of current word or white space.
	 */
	back_in_line();
	start_pos = curwin->w_cursor;

	/*
	 * If the start is on white space, and white space should be included
	 * ("	word"), or start is not on white space, and white space should
	 * not be included ("word"), find end of word.
	 */
	if ((cls() == 0) == include)
	{
	    if (end_word(1L, bigword, TRUE, TRUE) == FAIL)
		return FAIL;
	}
	else
	{
	    /*
	     * If the start is not on white space, and white space should be
	     * included ("word	 "), or start is on white space and white
	     * space should not be included ("	 "), find start of word.
	     * If we end up in the first column of the next line (single char
	     * word) back up to end of the line.
	     */
	    fwd_word(1L, bigword, TRUE);
	    if (curwin->w_cursor.col == 0)
		decl(&curwin->w_cursor);
	    else
		oneleft();

	    if (include)
		include_white = TRUE;
	}

	if (VIsual_active)
	{
	    // should do something when inclusive == FALSE !
	    VIsual = start_pos;
	    redraw_curbuf_later(UPD_INVERTED);	// update the inversion
	}
	else
	{
	    oap->start = start_pos;
	    oap->motion_type = MCHAR;
	}
	--count;
    }

    /*
     * When count is still > 0, extend with more objects.
     */
    while (count > 0)
    {
	inclusive = TRUE;
	if (VIsual_active && LT_POS(curwin->w_cursor, VIsual))
	{
	    /*
	     * In Visual mode, with cursor at start: move cursor back.
	     */
	    if (decl(&curwin->w_cursor) == -1)
		return FAIL;
	    if (include != (cls() != 0))
	    {
		if (bck_word(1L, bigword, TRUE) == FAIL)
		    return FAIL;
	    }
	    else
	    {
		if (bckend_word(1L, bigword, TRUE) == FAIL)
		    return FAIL;
		(void)incl(&curwin->w_cursor);
	    }
	}
	else
	{
	    /*
	     * Move cursor forward one word and/or white area.
	     */
	    if (incl(&curwin->w_cursor) == -1)
		return FAIL;
	    if (include != (cls() == 0))
	    {
		if (fwd_word(1L, bigword, TRUE) == FAIL && count > 1)
		    return FAIL;
		/*
		 * If end is just past a new-line, we don't want to include
		 * the first character on the line.
		 * Put cursor on last char of white.
		 */
		if (oneleft() == FAIL)
		    inclusive = FALSE;
	    }
	    else
	    {
		if (end_word(1L, bigword, TRUE, TRUE) == FAIL)
		    return FAIL;
	    }
	}
	--count;
    }

    if (include_white && (cls() != 0
		 || (curwin->w_cursor.col == 0 && !inclusive)))
    {
	/*
	 * If we don't include white space at the end, move the start
	 * to include some white space there. This makes "daw" work
	 * better on the last word in a sentence (and "2daw" on last-but-one
	 * word).  Also when "2daw" deletes "word." at the end of the line
	 * (cursor is at start of next line).
	 * But don't delete white space at start of line (indent).
	 */
	pos = curwin->w_cursor;	// save cursor position
	curwin->w_cursor = start_pos;
	if (oneleft() == OK)
	{
	    back_in_line();
	    if (cls() == 0 && curwin->w_cursor.col > 0)
	    {
		if (VIsual_active)
		    VIsual = curwin->w_cursor;
		else
		    oap->start = curwin->w_cursor;
	    }
	}
	curwin->w_cursor = pos;	// put cursor back at end
    }

    if (VIsual_active)
    {
	if (*p_sel == 'e' && inclusive && LTOREQ_POS(VIsual, curwin->w_cursor))
	    inc_cursor();
	if (VIsual_mode == 'V')
	{
	    VIsual_mode = 'v';
	    redraw_cmdline = TRUE;		// show mode later
	}
    }
    else
	oap->inclusive = inclusive;

    return OK;
}

/*
 * Find sentence(s) under the cursor, cursor at end.
 * When Visual active, extend it by one or more sentences.
 */
    int
current_sent(oparg_T *oap, long count, int include)
{
    pos_T	start_pos;
    pos_T	pos;
    int		start_blank;
    int		c;
    int		at_start_sent;
    long	ncount;

    start_pos = curwin->w_cursor;
    pos = start_pos;
    findsent(FORWARD, 1L);	// Find start of next sentence.

    /*
     * When the Visual area is bigger than one character: Extend it.
     */
    if (VIsual_active && !EQUAL_POS(start_pos, VIsual))
    {
extend:
	if (LT_POS(start_pos, VIsual))
	{
	    /*
	     * Cursor at start of Visual area.
	     * Find out where we are:
	     * - in the white space before a sentence
	     * - in a sentence or just after it
	     * - at the start of a sentence
	     */
	    at_start_sent = TRUE;
	    decl(&pos);
	    while (LT_POS(pos, curwin->w_cursor))
	    {
		c = gchar_pos(&pos);
		if (!VIM_ISWHITE(c))
		{
		    at_start_sent = FALSE;
		    break;
		}
		incl(&pos);
	    }
	    if (!at_start_sent)
	    {
		findsent(BACKWARD, 1L);
		if (EQUAL_POS(curwin->w_cursor, start_pos))
		    at_start_sent = TRUE;  // exactly at start of sentence
		else
		    // inside a sentence, go to its end (start of next)
		    findsent(FORWARD, 1L);
	    }
	    if (include)	// "as" gets twice as much as "is"
		count *= 2;
	    while (count--)
	    {
		if (at_start_sent)
		    find_first_blank(&curwin->w_cursor);
		c = gchar_cursor();
		if (!at_start_sent || (!include && !VIM_ISWHITE(c)))
		    findsent(BACKWARD, 1L);
		at_start_sent = !at_start_sent;
	    }
	}
	else
	{
	    /*
	     * Cursor at end of Visual area.
	     * Find out where we are:
	     * - just before a sentence
	     * - just before or in the white space before a sentence
	     * - in a sentence
	     */
	    incl(&pos);
	    at_start_sent = TRUE;
	    // not just before a sentence
	    if (!EQUAL_POS(pos, curwin->w_cursor))
	    {
		at_start_sent = FALSE;
		while (LT_POS(pos, curwin->w_cursor))
		{
		    c = gchar_pos(&pos);
		    if (!VIM_ISWHITE(c))
		    {
			at_start_sent = TRUE;
			break;
		    }
		    incl(&pos);
		}
		if (at_start_sent)	// in the sentence
		    findsent(BACKWARD, 1L);
		else		// in/before white before a sentence
		    curwin->w_cursor = start_pos;
	    }

	    if (include)	// "as" gets twice as much as "is"
		count *= 2;
	    findsent_forward(count, at_start_sent);
	    if (*p_sel == 'e')
		++curwin->w_cursor.col;
	}
	return OK;
    }

    /*
     * If the cursor started on a blank, check if it is just before the start
     * of the next sentence.
     */
    while (c = gchar_pos(&pos), VIM_ISWHITE(c))	// VIM_ISWHITE() is a macro
	incl(&pos);
    if (EQUAL_POS(pos, curwin->w_cursor))
    {
	start_blank = TRUE;
	find_first_blank(&start_pos);	// go back to first blank
    }
    else
    {
	start_blank = FALSE;
	findsent(BACKWARD, 1L);
	start_pos = curwin->w_cursor;
    }
    if (include)
	ncount = count * 2;
    else
    {
	ncount = count;
	if (start_blank)
	    --ncount;
    }
    if (ncount > 0)
	findsent_forward(ncount, TRUE);
    else
	decl(&curwin->w_cursor);

    if (include)
    {
	/*
	 * If the blank in front of the sentence is included, exclude the
	 * blanks at the end of the sentence, go back to the first blank.
	 * If there are no trailing blanks, try to include leading blanks.
	 */
	if (start_blank)
	{
	    find_first_blank(&curwin->w_cursor);
	    c = gchar_pos(&curwin->w_cursor);	// VIM_ISWHITE() is a macro
	    if (VIM_ISWHITE(c))
		decl(&curwin->w_cursor);
	}
	else if (c = gchar_cursor(), !VIM_ISWHITE(c))
	    find_first_blank(&start_pos);
    }

    if (VIsual_active)
    {
	// Avoid getting stuck with "is" on a single space before a sentence.
	if (EQUAL_POS(start_pos, curwin->w_cursor))
	    goto extend;
	if (*p_sel == 'e')
	    ++curwin->w_cursor.col;
	VIsual = start_pos;
	VIsual_mode = 'v';
	redraw_cmdline = TRUE;		// show mode later
	redraw_curbuf_later(UPD_INVERTED);	// update the inversion
    }
    else
    {
	// include a newline after the sentence, if there is one
	if (incl(&curwin->w_cursor) == -1)
	    oap->inclusive = TRUE;
	else
	    oap->inclusive = FALSE;
	oap->start = start_pos;
	oap->motion_type = MCHAR;
    }
    return OK;
}

/*
 * Find block under the cursor, cursor at end.
 * "what" and "other" are two matching parenthesis/brace/etc.
 */
    int
current_block(
    oparg_T	*oap,
    long	count,
    int		include,	// TRUE == include white space
    int		what,		// '(', '{', etc.
    int		other)		// ')', '}', etc.
{
    pos_T	old_pos;
    pos_T	*pos = NULL;
    pos_T	start_pos;
    pos_T	*end_pos;
    pos_T	old_start, old_end;
    char_u	*save_cpo;
    int		sol = FALSE;		// '{' at start of line

    old_pos = curwin->w_cursor;
    old_end = curwin->w_cursor;		// remember where we started
    old_start = old_end;

    /*
     * If we start on '(', '{', ')', '}', etc., use the whole block inclusive.
     */
    if (!VIsual_active || EQUAL_POS(VIsual, curwin->w_cursor))
    {
	setpcmark();
	if (what == '{')		// ignore indent
	    while (inindent(1))
		if (inc_cursor() != 0)
		    break;
	if (gchar_cursor() == what)
	    // cursor on '(' or '{', move cursor just after it
	    ++curwin->w_cursor.col;
    }
    else if (LT_POS(VIsual, curwin->w_cursor))
    {
	old_start = VIsual;
	curwin->w_cursor = VIsual;	    // cursor at low end of Visual
    }
    else
	old_end = VIsual;

    /*
     * Search backwards for unclosed '(', '{', etc..
     * Put this position in start_pos.
     * Ignore quotes here.  Keep the "M" flag in 'cpo', as that is what the
     * user wants.
     */
    save_cpo = p_cpo;
    p_cpo = (char_u *)(vim_strchr(p_cpo, CPO_MATCHBSL) != NULL ? "%M" : "%");
    if ((pos = findmatch(NULL, what)) != NULL)
    {
	while (count-- > 0)
	{
	    if ((pos = findmatch(NULL, what)) == NULL)
		break;
	    curwin->w_cursor = *pos;
	    start_pos = *pos;   // the findmatch for end_pos will overwrite *pos
	}
    }
    else
    {
	while (count-- > 0)
	{
	    if ((pos = findmatchlimit(NULL, what, FM_FORWARD, 0)) == NULL)
		break;
	    curwin->w_cursor = *pos;
	    start_pos = *pos;   // the findmatch for end_pos will overwrite *pos
	}
    }
    p_cpo = save_cpo;

    /*
     * Search for matching ')', '}', etc.
     * Put this position in curwin->w_cursor.
     */
    if (pos == NULL || (end_pos = findmatch(NULL, other)) == NULL)
    {
	curwin->w_cursor = old_pos;
	return FAIL;
    }
    curwin->w_cursor = *end_pos;

    /*
     * Try to exclude the '(', '{', ')', '}', etc. when "include" is FALSE.
     * If the ending '}', ')' or ']' is only preceded by indent, skip that
     * indent.  But only if the resulting area is not smaller than what we
     * started with.
     */
    while (!include)
    {
	incl(&start_pos);
	sol = (curwin->w_cursor.col == 0);
	decl(&curwin->w_cursor);
	while (inindent(1))
	{
	    sol = TRUE;
	    if (decl(&curwin->w_cursor) != 0)
		break;
	}

	/*
	 * In Visual mode, when resulting area is empty
	 * i.e. there is no inner block to select, abort.
	 */
	if (EQUAL_POS(start_pos, *end_pos) && VIsual_active)
	{
	    curwin->w_cursor = old_pos;
	    return FAIL;
	}

	/*
	 * In Visual mode, when the resulting area is not bigger than what we
	 * started with, extend it to the next block, and then exclude again.
	 * Don't try to expand the area if the area is empty.
	 */
	if (!LT_POS(start_pos, old_start) && !LT_POS(old_end, curwin->w_cursor)
		&& !EQUAL_POS(start_pos, curwin->w_cursor)
		&& VIsual_active)
	{
	    curwin->w_cursor = old_start;
	    decl(&curwin->w_cursor);
	    if ((pos = findmatch(NULL, what)) == NULL)
	    {
		curwin->w_cursor = old_pos;
		return FAIL;
	    }
	    start_pos = *pos;
	    curwin->w_cursor = *pos;
	    if ((end_pos = findmatch(NULL, other)) == NULL)
	    {
		curwin->w_cursor = old_pos;
		return FAIL;
	    }
	    curwin->w_cursor = *end_pos;
	}
	else
	    break;
    }

    if (VIsual_active)
    {
	if (*p_sel == 'e')
	    inc(&curwin->w_cursor);
	if (sol && gchar_cursor() != NUL)
	    inc(&curwin->w_cursor);	// include the line break
	VIsual = start_pos;
	VIsual_mode = 'v';
	redraw_curbuf_later(UPD_INVERTED);	// update the inversion
	showmode();
    }
    else
    {
	oap->start = start_pos;
	oap->motion_type = MCHAR;
	oap->inclusive = FALSE;
	if (sol)
	    incl(&curwin->w_cursor);
	else if (LTOREQ_POS(start_pos, curwin->w_cursor))
	    // Include the character under the cursor.
	    oap->inclusive = TRUE;
	else
	    // End is before the start (no text in between <>, [], etc.): don't
	    // operate on any text.
	    curwin->w_cursor = start_pos;
    }

    return OK;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return TRUE if the cursor is on a "<aaa>" tag.  Ignore "<aaa/>".
 * When "end_tag" is TRUE return TRUE if the cursor is on "</aaa>".
 */
    static int
in_html_tag(
    int		end_tag)
{
    char_u	*line = ml_get_curline();
    char_u	*p;
    int		c;
    int		lc = NUL;
    pos_T	pos;

    if (enc_dbcs)
    {
	char_u	*lp = NULL;

	// We search forward until the cursor, because searching backwards is
	// very slow for DBCS encodings.
	for (p = line; p < line + curwin->w_cursor.col; MB_PTR_ADV(p))
	    if (*p == '>' || *p == '<')
	    {
		lc = *p;
		lp = p;
	    }
	if (*p != '<')	    // check for '<' under cursor
	{
	    if (lc != '<')
		return FALSE;
	    p = lp;
	}
    }
    else
    {
	for (p = line + curwin->w_cursor.col; p > line; )
	{
	    if (*p == '<')	// find '<' under/before cursor
		break;
	    MB_PTR_BACK(line, p);
	    if (*p == '>')	// find '>' before cursor
		break;
	}
	if (*p != '<')
	    return FALSE;
    }

    pos.lnum = curwin->w_cursor.lnum;
    pos.col = (colnr_T)(p - line);

    MB_PTR_ADV(p);
    if (end_tag)
	// check that there is a '/' after the '<'
	return *p == '/';

    // check that there is no '/' after the '<'
    if (*p == '/')
	return FALSE;

    // check that the matching '>' is not preceded by '/'
    for (;;)
    {
	if (inc(&pos) < 0)
	    return FALSE;
	c = *ml_get_pos(&pos);
	if (c == '>')
	    break;
	lc = c;
    }
    return lc != '/';
}

/*
 * Find tag block under the cursor, cursor at end.
 */
    int
current_tagblock(
    oparg_T	*oap,
    long	count_arg,
    int		include)	// TRUE == include white space
{
    long	count = count_arg;
    long	n;
    pos_T	old_pos;
    pos_T	start_pos;
    pos_T	end_pos;
    pos_T	old_start, old_end;
    char_u	*spat, *epat;
    char_u	*p;
    char_u	*cp;
    int		len;
    int		r;
    int		do_include = include;
    int		save_p_ws = p_ws;
    int		retval = FAIL;
    int		is_inclusive = TRUE;

    p_ws = FALSE;

    old_pos = curwin->w_cursor;
    old_end = curwin->w_cursor;		    // remember where we started
    old_start = old_end;
    if (!VIsual_active || *p_sel == 'e')
	decl(&old_end);			    // old_end is inclusive

    /*
     * If we start on "<aaa>" select that block.
     */
    if (!VIsual_active || EQUAL_POS(VIsual, curwin->w_cursor))
    {
	setpcmark();

	// ignore indent
	while (inindent(1))
	    if (inc_cursor() != 0)
		break;

	if (in_html_tag(FALSE))
	{
	    // cursor on start tag, move to its '>'
	    while (*ml_get_cursor() != '>')
		if (inc_cursor() < 0)
		    break;
	}
	else if (in_html_tag(TRUE))
	{
	    // cursor on end tag, move to just before it
	    while (*ml_get_cursor() != '<')
		if (dec_cursor() < 0)
		    break;
	    dec_cursor();
	    old_end = curwin->w_cursor;
	}
    }
    else if (LT_POS(VIsual, curwin->w_cursor))
    {
	old_start = VIsual;
	curwin->w_cursor = VIsual;	    // cursor at low end of Visual
    }
    else
	old_end = VIsual;

again:
    /*
     * Search backwards for unclosed "<aaa>".
     * Put this position in start_pos.
     */
    for (n = 0; n < count; ++n)
    {
	if (do_searchpair((char_u *)"<[^ \t>/!]\\+\\%(\\_s\\_[^>]\\{-}[^/]>\\|$\\|\\_s\\=>\\)",
		    (char_u *)"",
		    (char_u *)"</[^>]*>", BACKWARD, NULL, 0,
						  NULL, (linenr_T)0, 0L) <= 0)
	{
	    curwin->w_cursor = old_pos;
	    goto theend;
	}
    }
    start_pos = curwin->w_cursor;

    /*
     * Search for matching "</aaa>".  First isolate the "aaa".
     */
    inc_cursor();
    p = ml_get_cursor();
    for (cp = p; *cp != NUL && *cp != '>' && !VIM_ISWHITE(*cp); MB_PTR_ADV(cp))
	;
    len = (int)(cp - p);
    if (len == 0)
    {
	curwin->w_cursor = old_pos;
	goto theend;
    }
    spat = alloc(len + 39);
    epat = alloc(len + 9);
    if (spat == NULL || epat == NULL)
    {
	vim_free(spat);
	vim_free(epat);
	curwin->w_cursor = old_pos;
	goto theend;
    }
    sprintf((char *)spat, "<%.*s\\>\\%%(\\_s\\_[^>]\\{-}\\_[^/]>\\|\\_s\\?>\\)\\c", len, p);
    sprintf((char *)epat, "</%.*s>\\c", len, p);

    r = do_searchpair(spat, (char_u *)"", epat, FORWARD, NULL,
						    0, NULL, (linenr_T)0, 0L);

    vim_free(spat);
    vim_free(epat);

    if (r < 1 || LT_POS(curwin->w_cursor, old_end))
    {
	// Can't find other end or it's before the previous end.  Could be a
	// HTML tag that doesn't have a matching end.  Search backwards for
	// another starting tag.
	count = 1;
	curwin->w_cursor = start_pos;
	goto again;
    }

    if (do_include)
    {
	// Include up to the '>'.
	while (*ml_get_cursor() != '>')
	    if (inc_cursor() < 0)
		break;
    }
    else
    {
	char_u *c = ml_get_cursor();

	// Exclude the '<' of the end tag.
	// If the closing tag is on new line, do not decrement cursor, but
	// make operation exclusive, so that the linefeed will be selected
	if (*c == '<' && !VIsual_active && curwin->w_cursor.col == 0)
	    // do not decrement cursor
	    is_inclusive = FALSE;
	else if (*c == '<')
	    dec_cursor();
    }
    end_pos = curwin->w_cursor;

    if (!do_include)
    {
	// Exclude the start tag.
	curwin->w_cursor = start_pos;
	while (inc_cursor() >= 0)
	    if (*ml_get_cursor() == '>')
	    {
		inc_cursor();
		start_pos = curwin->w_cursor;
		break;
	    }
	curwin->w_cursor = end_pos;

	// If we are in Visual mode and now have the same text as before set
	// "do_include" and try again.
	if (VIsual_active && EQUAL_POS(start_pos, old_start)
						&& EQUAL_POS(end_pos, old_end))
	{
	    do_include = TRUE;
	    curwin->w_cursor = old_start;
	    count = count_arg;
	    goto again;
	}
    }

    if (VIsual_active)
    {
	// If the end is before the start there is no text between tags, select
	// the char under the cursor.
	if (LT_POS(end_pos, start_pos))
	    curwin->w_cursor = start_pos;
	else if (*p_sel == 'e')
	    inc_cursor();
	VIsual = start_pos;
	VIsual_mode = 'v';
	redraw_curbuf_later(UPD_INVERTED);	// update the inversion
	showmode();
    }
    else
    {
	oap->start = start_pos;
	oap->motion_type = MCHAR;
	if (LT_POS(end_pos, start_pos))
	{
	    // End is before the start: there is no text between tags; operate
	    // on an empty area.
	    curwin->w_cursor = start_pos;
	    oap->inclusive = FALSE;
	}
	else
	    oap->inclusive = is_inclusive;
    }
    retval = OK;

theend:
    p_ws = save_p_ws;
    return retval;
}
#endif

    int
current_par(
    oparg_T	*oap,
    long	count,
    int		include,	// TRUE == include white space
    int		type)		// 'p' for paragraph, 'S' for section
{
    linenr_T	start_lnum;
    linenr_T	end_lnum;
    int		white_in_front;
    int		dir;
    int		start_is_white;
    int		prev_start_is_white;
    int		retval = OK;
    int		do_white = FALSE;
    int		t;
    int		i;

    if (type == 'S')	    // not implemented yet
	return FAIL;

    start_lnum = curwin->w_cursor.lnum;

    /*
     * When visual area is more than one line: extend it.
     */
    if (VIsual_active && start_lnum != VIsual.lnum)
    {
extend:
	if (start_lnum < VIsual.lnum)
	    dir = BACKWARD;
	else
	    dir = FORWARD;
	for (i = count; --i >= 0; )
	{
	    if (start_lnum ==
			   (dir == BACKWARD ? 1 : curbuf->b_ml.ml_line_count))
	    {
		retval = FAIL;
		break;
	    }

	    prev_start_is_white = -1;
	    for (t = 0; t < 2; ++t)
	    {
		start_lnum += dir;
		start_is_white = linewhite(start_lnum);
		if (prev_start_is_white == start_is_white)
		{
		    start_lnum -= dir;
		    break;
		}
		for (;;)
		{
		    if (start_lnum == (dir == BACKWARD
					    ? 1 : curbuf->b_ml.ml_line_count))
			break;
		    if (start_is_white != linewhite(start_lnum + dir)
			    || (!start_is_white
				    && startPS(start_lnum + (dir > 0
							     ? 1 : 0), 0, 0)))
			break;
		    start_lnum += dir;
		}
		if (!include)
		    break;
		if (start_lnum == (dir == BACKWARD
					    ? 1 : curbuf->b_ml.ml_line_count))
		    break;
		prev_start_is_white = start_is_white;
	    }
	}
	curwin->w_cursor.lnum = start_lnum;
	curwin->w_cursor.col = 0;
	return retval;
    }

    /*
     * First move back to the start_lnum of the paragraph or white lines
     */
    white_in_front = linewhite(start_lnum);
    while (start_lnum > 1)
    {
	if (white_in_front)	    // stop at first white line
	{
	    if (!linewhite(start_lnum - 1))
		break;
	}
	else		// stop at first non-white line of start of paragraph
	{
	    if (linewhite(start_lnum - 1) || startPS(start_lnum, 0, 0))
		break;
	}
	--start_lnum;
    }

    /*
     * Move past the end of any white lines.
     */
    end_lnum = start_lnum;
    while (end_lnum <= curbuf->b_ml.ml_line_count && linewhite(end_lnum))
	++end_lnum;

    --end_lnum;
    i = count;
    if (!include && white_in_front)
	--i;
    while (i--)
    {
	if (end_lnum == curbuf->b_ml.ml_line_count)
	    return FAIL;

	if (!include)
	    do_white = linewhite(end_lnum + 1);

	if (include || !do_white)
	{
	    ++end_lnum;
	    /*
	     * skip to end of paragraph
	     */
	    while (end_lnum < curbuf->b_ml.ml_line_count
		    && !linewhite(end_lnum + 1)
		    && !startPS(end_lnum + 1, 0, 0))
		++end_lnum;
	}

	if (i == 0 && white_in_front && include)
	    break;

	/*
	 * skip to end of white lines after paragraph
	 */
	if (include || do_white)
	    while (end_lnum < curbuf->b_ml.ml_line_count
						   && linewhite(end_lnum + 1))
		++end_lnum;
    }

    /*
     * If there are no empty lines at the end, try to find some empty lines at
     * the start (unless that has been done already).
     */
    if (!white_in_front && !linewhite(end_lnum) && include)
	while (start_lnum > 1 && linewhite(start_lnum - 1))
	    --start_lnum;

    if (VIsual_active)
    {
	// Problem: when doing "Vipipip" nothing happens in a single white
	// line, we get stuck there.  Trap this here.
	if (VIsual_mode == 'V' && start_lnum == curwin->w_cursor.lnum)
	    goto extend;
	if (VIsual.lnum != start_lnum)
	{
	    VIsual.lnum = start_lnum;
	    VIsual.col = 0;
	}
	VIsual_mode = 'V';
	redraw_curbuf_later(UPD_INVERTED);	// update the inversion
	showmode();
    }
    else
    {
	oap->start.lnum = start_lnum;
	oap->start.col = 0;
	oap->motion_type = MLINE;
    }
    curwin->w_cursor.lnum = end_lnum;
    curwin->w_cursor.col = 0;

    return OK;
}

/*
 * Search quote char from string line[col].
 * Quote character escaped by one of the characters in "escape" is not counted
 * as a quote.
 * Returns column number of "quotechar" or -1 when not found.
 */
    static int
find_next_quote(
    char_u	*line,
    int		col,
    int		quotechar,
    char_u	*escape)	// escape characters, can be NULL
{
    int		c;

    for (;;)
    {
	c = line[col];
	if (c == NUL)
	    return -1;
	else if (escape != NULL && vim_strchr(escape, c))
	{
	    ++col;
	    if (line[col] == NUL)
		return -1;
	}
	else if (c == quotechar)
	    break;
	if (has_mbyte)
	    col += (*mb_ptr2len)(line + col);
	else
	    ++col;
    }
    return col;
}

/*
 * Search backwards in "line" from column "col_start" to find "quotechar".
 * Quote character escaped by one of the characters in "escape" is not counted
 * as a quote.
 * Return the found column or zero.
 */
    static int
find_prev_quote(
    char_u	*line,
    int		col_start,
    int		quotechar,
    char_u	*escape)	// escape characters, can be NULL
{
    int		n;

    while (col_start > 0)
    {
	--col_start;
	col_start -= (*mb_head_off)(line, line + col_start);
	n = 0;
	if (escape != NULL)
	    while (col_start - n > 0 && vim_strchr(escape,
					     line[col_start - n - 1]) != NULL)
	    ++n;
	if (n & 1)
	    col_start -= n;	// uneven number of escape chars, skip it
	else if (line[col_start] == quotechar)
	    break;
    }
    return col_start;
}

/*
 * Find quote under the cursor, cursor at end.
 * Returns TRUE if found, else FALSE.
 */
    int
current_quote(
    oparg_T	*oap,
    long	count,
    int		include,	// TRUE == include quote char
    int		quotechar)	// Quote character
{
    char_u	*line = ml_get_curline();
    int		col_end;
    int		col_start = curwin->w_cursor.col;
    int		inclusive = FALSE;
    int		vis_empty = TRUE;	// Visual selection <= 1 char
    int		vis_bef_curs = FALSE;	// Visual starts before cursor
    int		did_exclusive_adj = FALSE;  // adjusted pos for 'selection'
    int		inside_quotes = FALSE;	// Looks like "i'" done before
    int		selected_quote = FALSE;	// Has quote inside selection
    int		i;
    int		restore_vis_bef = FALSE; // restore VIsual on abort

    // When 'selection' is "exclusive" move the cursor to where it would be
    // with 'selection' "inclusive", so that the logic is the same for both.
    // The cursor then is moved forward after adjusting the area.
    if (VIsual_active)
    {
	// this only works within one line
	if (VIsual.lnum != curwin->w_cursor.lnum)
	    return FALSE;

	vis_bef_curs = LT_POS(VIsual, curwin->w_cursor);
	vis_empty = EQUAL_POS(VIsual, curwin->w_cursor);
	if (*p_sel == 'e')
	{
	    if (vis_bef_curs)
	    {
		dec_cursor();
		did_exclusive_adj = TRUE;
	    }
	    else if (!vis_empty)
	    {
		dec(&VIsual);
		did_exclusive_adj = TRUE;
	    }
	    vis_empty = EQUAL_POS(VIsual, curwin->w_cursor);
	    if (!vis_bef_curs && !vis_empty)
	    {
		// VIsual needs to be the start of Visual selection.
		pos_T t = curwin->w_cursor;

		curwin->w_cursor = VIsual;
		VIsual = t;
		vis_bef_curs = TRUE;
		restore_vis_bef = TRUE;
	    }
	}
    }

    if (!vis_empty)
    {
	// Check if the existing selection exactly spans the text inside
	// quotes.
	if (vis_bef_curs)
	{
	    inside_quotes = VIsual.col > 0
			&& line[VIsual.col - 1] == quotechar
			&& line[curwin->w_cursor.col] != NUL
			&& line[curwin->w_cursor.col + 1] == quotechar;
	    i = VIsual.col;
	    col_end = curwin->w_cursor.col;
	}
	else
	{
	    inside_quotes = curwin->w_cursor.col > 0
			&& line[curwin->w_cursor.col - 1] == quotechar
			&& line[VIsual.col] != NUL
			&& line[VIsual.col + 1] == quotechar;
	    i = curwin->w_cursor.col;
	    col_end = VIsual.col;
	}

	// Find out if we have a quote in the selection.
	while (i <= col_end)
	{
	    // check for going over the end of the line, which can happen if
	    // the line was changed after the Visual area was selected.
	    if (line[i] == NUL)
		break;
	    if (line[i++] == quotechar)
	    {
		selected_quote = TRUE;
		break;
	    }
	}
    }

    if (!vis_empty && line[col_start] == quotechar)
    {
	// Already selecting something and on a quote character.  Find the
	// next quoted string.
	if (vis_bef_curs)
	{
	    // Assume we are on a closing quote: move to after the next
	    // opening quote.
	    col_start = find_next_quote(line, col_start + 1, quotechar, NULL);
	    if (col_start < 0)
		goto abort_search;
	    col_end = find_next_quote(line, col_start + 1, quotechar,
							      curbuf->b_p_qe);
	    if (col_end < 0)
	    {
		// We were on a starting quote perhaps?
		col_end = col_start;
		col_start = curwin->w_cursor.col;
	    }
	}
	else
	{
	    col_end = find_prev_quote(line, col_start, quotechar, NULL);
	    if (line[col_end] != quotechar)
		goto abort_search;
	    col_start = find_prev_quote(line, col_end, quotechar,
							      curbuf->b_p_qe);
	    if (line[col_start] != quotechar)
	    {
		// We were on an ending quote perhaps?
		col_start = col_end;
		col_end = curwin->w_cursor.col;
	    }
	}
    }
    else

    if (line[col_start] == quotechar || !vis_empty)
    {
	int	first_col = col_start;

	if (!vis_empty)
	{
	    if (vis_bef_curs)
		first_col = find_next_quote(line, col_start, quotechar, NULL);
	    else
		first_col = find_prev_quote(line, col_start, quotechar, NULL);
	}

	// The cursor is on a quote, we don't know if it's the opening or
	// closing quote.  Search from the start of the line to find out.
	// Also do this when there is a Visual area, a' may leave the cursor
	// in between two strings.
	col_start = 0;
	for (;;)
	{
	    // Find open quote character.
	    col_start = find_next_quote(line, col_start, quotechar, NULL);
	    if (col_start < 0 || col_start > first_col)
		goto abort_search;
	    // Find close quote character.
	    col_end = find_next_quote(line, col_start + 1, quotechar,
							      curbuf->b_p_qe);
	    if (col_end < 0)
		goto abort_search;
	    // If is cursor between start and end quote character, it is
	    // target text object.
	    if (col_start <= first_col && first_col <= col_end)
		break;
	    col_start = col_end + 1;
	}
    }
    else
    {
	// Search backward for a starting quote.
	col_start = find_prev_quote(line, col_start, quotechar, curbuf->b_p_qe);
	if (line[col_start] != quotechar)
	{
	    // No quote before the cursor, look after the cursor.
	    col_start = find_next_quote(line, col_start, quotechar, NULL);
	    if (col_start < 0)
		goto abort_search;
	}

	// Find close quote character.
	col_end = find_next_quote(line, col_start + 1, quotechar,
							      curbuf->b_p_qe);
	if (col_end < 0)
	    goto abort_search;
    }

    // When "include" is TRUE, include spaces after closing quote or before
    // the starting quote.
    if (include)
    {
	if (VIM_ISWHITE(line[col_end + 1]))
	    while (VIM_ISWHITE(line[col_end + 1]))
		++col_end;
	else
	    while (col_start > 0 && VIM_ISWHITE(line[col_start - 1]))
		--col_start;
    }

    // Set start position.  After vi" another i" must include the ".
    // For v2i" include the quotes.
    if (!include && count < 2 && (vis_empty || !inside_quotes))
	++col_start;
    curwin->w_cursor.col = col_start;
    if (VIsual_active)
    {
	// Set the start of the Visual area when the Visual area was empty, we
	// were just inside quotes or the Visual area didn't start at a quote
	// and didn't include a quote.
	if (vis_empty
		|| (vis_bef_curs
		    && !selected_quote
		    && (inside_quotes
			|| (line[VIsual.col] != quotechar
			    && (VIsual.col == 0
				|| line[VIsual.col - 1] != quotechar)))))
	{
	    VIsual = curwin->w_cursor;
	    redraw_curbuf_later(UPD_INVERTED);
	}
    }
    else
    {
	oap->start = curwin->w_cursor;
	oap->motion_type = MCHAR;
    }

    // Set end position.
    curwin->w_cursor.col = col_end;
    if ((include || count > 1 // After vi" another i" must include the ".
		|| (!vis_empty && inside_quotes)
	) && inc_cursor() == 2)
	inclusive = TRUE;
    if (VIsual_active)
    {
	if (vis_empty || vis_bef_curs)
	{
	    // decrement cursor when 'selection' is not exclusive
	    if (*p_sel != 'e')
		dec_cursor();
	}
	else
	{
	    // Cursor is at start of Visual area.  Set the end of the Visual
	    // area when it was just inside quotes or it didn't end at a
	    // quote.
	    if (inside_quotes
		    || (!selected_quote
			&& line[VIsual.col] != quotechar
			&& (line[VIsual.col] == NUL
			    || line[VIsual.col + 1] != quotechar)))
	    {
		dec_cursor();
		VIsual = curwin->w_cursor;
	    }
	    curwin->w_cursor.col = col_start;
	}
	if (VIsual_mode == 'V')
	{
	    VIsual_mode = 'v';
	    redraw_cmdline = TRUE;		// show mode later
	}
    }
    else
    {
	// Set inclusive and other oap's flags.
	oap->inclusive = inclusive;
    }

    return OK;

abort_search:
    if (VIsual_active && *p_sel == 'e')
    {
	if (did_exclusive_adj)
	    inc_cursor();
	if (restore_vis_bef)
	{
	    pos_T t = curwin->w_cursor;

	    curwin->w_cursor = VIsual;
	    VIsual = t;
	}
    }
    return FALSE;
}
