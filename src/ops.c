/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * ops.c: implementation of various operators: op_shift, op_delete, op_tilde,
 *	  op_change, op_yank, do_join
 */

#include "vim.h"

static void shift_block(oparg_T *oap, int amount);
static void	mb_adjust_opend(oparg_T *oap);
static int	do_addsub(int op_type, pos_T *pos, int length, linenr_T Prenum1);
static void	pbyte(pos_T lp, int c);
#define PBYTE(lp, c) pbyte(lp, c)


// Flags for third item in "opchars".
#define OPF_LINES  1	// operator always works on lines
#define OPF_CHANGE 2	// operator changes text

/*
 * The names of operators.
 * IMPORTANT: Index must correspond with defines in vim.h!!!
 * The third field holds OPF_ flags.
 */
static char opchars[][3] =
{
    {NUL, NUL, 0},			// OP_NOP
    {'d', NUL, OPF_CHANGE},		// OP_DELETE
    {'y', NUL, 0},			// OP_YANK
    {'c', NUL, OPF_CHANGE},		// OP_CHANGE
    {'<', NUL, OPF_LINES | OPF_CHANGE},	// OP_LSHIFT
    {'>', NUL, OPF_LINES | OPF_CHANGE},	// OP_RSHIFT
    {'!', NUL, OPF_LINES | OPF_CHANGE},	// OP_FILTER
    {'g', '~', OPF_CHANGE},		// OP_TILDE
    {'=', NUL, OPF_LINES | OPF_CHANGE},	// OP_INDENT
    {'g', 'q', OPF_LINES | OPF_CHANGE},	// OP_FORMAT
    {':', NUL, OPF_LINES},		// OP_COLON
    {'g', 'U', OPF_CHANGE},		// OP_UPPER
    {'g', 'u', OPF_CHANGE},		// OP_LOWER
    {'J', NUL, OPF_LINES | OPF_CHANGE},	// DO_JOIN
    {'g', 'J', OPF_LINES | OPF_CHANGE},	// DO_JOIN_NS
    {'g', '?', OPF_CHANGE},		// OP_ROT13
    {'r', NUL, OPF_CHANGE},		// OP_REPLACE
    {'I', NUL, OPF_CHANGE},		// OP_INSERT
    {'A', NUL, OPF_CHANGE},		// OP_APPEND
    {'z', 'f', OPF_LINES},		// OP_FOLD
    {'z', 'o', OPF_LINES},		// OP_FOLDOPEN
    {'z', 'O', OPF_LINES},		// OP_FOLDOPENREC
    {'z', 'c', OPF_LINES},		// OP_FOLDCLOSE
    {'z', 'C', OPF_LINES},		// OP_FOLDCLOSEREC
    {'z', 'd', OPF_LINES},		// OP_FOLDDEL
    {'z', 'D', OPF_LINES},		// OP_FOLDDELREC
    {'g', 'w', OPF_LINES | OPF_CHANGE},	// OP_FORMAT2
    {'g', '@', OPF_CHANGE},		// OP_FUNCTION
    {Ctrl_A, NUL, OPF_CHANGE},		// OP_NR_ADD
    {Ctrl_X, NUL, OPF_CHANGE},		// OP_NR_SUB
};

/*
 * Translate a command name into an operator type.
 * Must only be called with a valid operator name!
 */
    int
get_op_type(int char1, int char2)
{
    int		i;

    if (char1 == 'r')		// ignore second character
	return OP_REPLACE;
    if (char1 == '~')		// when tilde is an operator
	return OP_TILDE;
    if (char1 == 'g' && char2 == Ctrl_A)	// add
	return OP_NR_ADD;
    if (char1 == 'g' && char2 == Ctrl_X)	// subtract
	return OP_NR_SUB;
    if (char1 == 'z' && char2 == 'y')	// OP_YANK
	return OP_YANK;
    for (i = 0; ; ++i)
    {
	if (opchars[i][0] == char1 && opchars[i][1] == char2)
	    break;
	if (i == (int)ARRAY_LENGTH(opchars) - 1)
	{
	    internal_error("get_op_type()");
	    break;
	}
    }
    return i;
}

/*
 * Return TRUE if operator "op" always works on whole lines.
 */
    static int
op_on_lines(int op)
{
    return opchars[op][2] & OPF_LINES;
}

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
/*
 * Return TRUE if operator "op" changes text.
 */
    int
op_is_change(int op)
{
    return opchars[op][2] & OPF_CHANGE;
}
#endif

/*
 * Get first operator command character.
 * Returns 'g' or 'z' if there is another command character.
 */
    int
get_op_char(int optype)
{
    return opchars[optype][0];
}

/*
 * Get second operator command character.
 */
    int
get_extra_op_char(int optype)
{
    return opchars[optype][1];
}

/*
 * op_shift - handle a shift operation
 */
    void
op_shift(oparg_T *oap, int curs_top, int amount)
{
    long	    i;
    int		    first_char;
    int		    block_col = 0;

    if (u_save((linenr_T)(oap->start.lnum - 1),
				       (linenr_T)(oap->end.lnum + 1)) == FAIL)
	return;

    if (oap->block_mode)
	block_col = curwin->w_cursor.col;

    for (i = oap->line_count; --i >= 0; )
    {
	first_char = *ml_get_curline();
	if (first_char == NUL)				// empty line
	    curwin->w_cursor.col = 0;
	else if (oap->block_mode)
	    shift_block(oap, amount);
	else
	    // Move the line right if it doesn't start with '#', 'smartindent'
	    // isn't set or 'cindent' isn't set or '#' isn't in 'cino'.
	    if (first_char != '#' || !preprocs_left())
		shift_line(oap->op_type == OP_LSHIFT, p_sr, amount, FALSE);
	++curwin->w_cursor.lnum;
    }

    changed_lines(oap->start.lnum, 0, oap->end.lnum + 1, 0L);
    if (oap->block_mode)
    {
	curwin->w_cursor.lnum = oap->start.lnum;
	curwin->w_cursor.col = block_col;
    }
    else if (curs_top)	    // put cursor on first line, for ">>"
    {
	curwin->w_cursor.lnum = oap->start.lnum;
	beginline(BL_SOL | BL_FIX);   // shift_line() may have set cursor.col
    }
    else
	--curwin->w_cursor.lnum;	// put cursor on last line, for ":>"

#ifdef FEAT_FOLDING
    // The cursor line is not in a closed fold
    foldOpenCursor();
#endif


    if (oap->line_count > p_report)
    {
	char	    *op;
	char	    *msg_line_single;
	char	    *msg_line_plural;

	if (oap->op_type == OP_RSHIFT)
	    op = ">";
	else
	    op = "<";
	msg_line_single = NGETTEXT("%ld line %sed %d time",
					     "%ld line %sed %d times", amount);
	msg_line_plural = NGETTEXT("%ld lines %sed %d time",
					    "%ld lines %sed %d times", amount);
	vim_snprintf((char *)IObuff, IOSIZE,
		NGETTEXT(msg_line_single, msg_line_plural, oap->line_count),
		oap->line_count, op, amount);
	msg_attr_keep((char *)IObuff, 0, TRUE);
    }

    if ((cmdmod.cmod_flags & CMOD_LOCKMARKS) == 0)
    {
	// Set "'[" and "']" marks.
	curbuf->b_op_start = oap->start;
	curbuf->b_op_end.lnum = oap->end.lnum;
	curbuf->b_op_end.col = (colnr_T)STRLEN(ml_get(oap->end.lnum));
	if (curbuf->b_op_end.col > 0)
	    --curbuf->b_op_end.col;
    }
}

/*
 * Shift the current line one shiftwidth left (if left != 0) or right
 * leaves cursor on first blank in the line.
 */
    void
shift_line(
    int	left,
    int	round,
    int	amount,
    int call_changed_bytes)	// call changed_bytes()
{
    vimlong_T	count;
    int		i, j;
    int		sw_val = trim_to_int(get_sw_value_indent(curbuf));

    count = get_indent();	// get current indent

    if (round)			// round off indent
    {
	i = count / sw_val;	// number of 'shiftwidth' rounded down
	j = count % sw_val;	// extra spaces
	if (j && left)		// first remove extra spaces
	    --amount;
	if (left)
	{
	    i -= amount;
	    if (i < 0)
		i = 0;
	}
	else
	    i += amount;
	count = (vimlong_T)i * (vimlong_T)sw_val;
    }
    else		// original vi indent
    {
	if (left)
	{
	    count -= (vimlong_T)sw_val * (vimlong_T)amount;
	    if (count < 0)
		count = 0;
	}
	else
	    count += (vimlong_T)sw_val * (vimlong_T)amount;
    }

    // Set new indent
    if (State & VREPLACE_FLAG)
	change_indent(INDENT_SET, trim_to_int(count), FALSE, NUL, call_changed_bytes);
    else
	(void)set_indent(trim_to_int(count), call_changed_bytes ? SIN_CHANGED : 0);
}

/*
 * Shift one line of the current block one shiftwidth right or left.
 * Leaves cursor on first character in block.
 */
    static void
shift_block(oparg_T *oap, int amount)
{
    int			left = (oap->op_type == OP_LSHIFT);
    int			oldstate = State;
    int			total;
    char_u		*newp, *oldp;
    int			oldcol = curwin->w_cursor.col;
    int			sw_val = (int)get_sw_value_indent(curbuf);
    int			ts_val = (int)curbuf->b_p_ts;
    struct block_def	bd;
    int			incr;
    colnr_T		ws_vcol;
    int			added;
    unsigned		new_line_len;	// the length of the line after the
					// block shift
#ifdef FEAT_RIGHTLEFT
    int			old_p_ri = p_ri;

    p_ri = 0;			// don't want revins in indent
#endif

    State = MODE_INSERT;	// don't want MODE_REPLACE for State
    block_prep(oap, &bd, curwin->w_cursor.lnum, TRUE);
    if (bd.is_short)
	return;

    // total is number of screen columns to be inserted/removed
    total = (int)((unsigned)amount * (unsigned)sw_val);
    if ((total / sw_val) != amount)
	return; // multiplication overflow

    oldp = ml_get_curline();

    if (!left)
    {
	int		tabs = 0, spaces = 0;
	chartabsize_T	cts;

	/*
	 *  1. Get start vcol
	 *  2. Total ws vcols
	 *  3. Divvy into TABs & spp
	 *  4. Construct new string
	 */
	total += bd.pre_whitesp; // all virtual WS up to & incl a split TAB
	ws_vcol = bd.start_vcol - bd.pre_whitesp;
	if (bd.startspaces)
	{
	    if (has_mbyte)
	    {
		if ((*mb_ptr2len)(bd.textstart) == 1)
		    ++bd.textstart;
		else
		{
		    ws_vcol = 0;
		    bd.startspaces = 0;
		}
	    }
	    else
		++bd.textstart;
	}

	// TODO: is passing bd.textstart for start of the line OK?
	init_chartabsize_arg(&cts, curwin, curwin->w_cursor.lnum,
				   bd.start_vcol, bd.textstart, bd.textstart);
	for ( ; VIM_ISWHITE(*cts.cts_ptr); )
	{
	    incr = lbr_chartabsize_adv(&cts);
	    total += incr;
	    cts.cts_vcol += incr;
	}
	bd.textstart = cts.cts_ptr;
	bd.start_vcol = cts.cts_vcol;
	clear_chartabsize_arg(&cts);

	// OK, now total=all the VWS reqd, and textstart points at the 1st
	// non-ws char in the block.
#ifdef FEAT_VARTABS
	if (!curbuf->b_p_et)
	    tabstop_fromto(ws_vcol, ws_vcol + total,
				ts_val, curbuf->b_p_vts_array, &tabs, &spaces);
	else
	    spaces = total;
#else
	if (!curbuf->b_p_et)
	    tabs = ((ws_vcol % ts_val) + total) / ts_val; // number of tabs
	if (tabs > 0)
	    spaces = ((ws_vcol % ts_val) + total) % ts_val; // number of spp
	else
	    spaces = total;
#endif
	// if we're splitting a TAB, allow for it
	bd.textcol -= bd.pre_whitesp_c - (bd.startspaces != 0);

	new_line_len = bd.textcol + tabs + spaces + (int)STRLEN(bd.textstart);
	newp = alloc(new_line_len + 1);
	if (newp == NULL)
	    return;
	mch_memmove(newp, oldp, (size_t)bd.textcol);
	vim_memset(newp + bd.textcol, TAB, (size_t)tabs);
	vim_memset(newp + bd.textcol + tabs, ' ', (size_t)spaces);
	// Note that STRMOVE() copies the trailing NUL.
	STRMOVE(newp + bd.textcol + tabs + spaces, bd.textstart);
    }
    else // left
    {
	colnr_T	    destination_col;	// column to which text in block will
					// be shifted
	char_u	    *verbatim_copy_end;	// end of the part of the line which is
					// copied verbatim
	colnr_T	    verbatim_copy_width;// the (displayed) width of this part
					// of line
	unsigned    fill;		// nr of spaces that replace a TAB
	size_t	    block_space_width;
	size_t	    shift_amount;
	char_u	    *non_white = bd.textstart;
	colnr_T	    non_white_col;
	chartabsize_T cts;

	/*
	 * Firstly, let's find the first non-whitespace character that is
	 * displayed after the block's start column and the character's column
	 * number. Also, let's calculate the width of all the whitespace
	 * characters that are displayed in the block and precede the searched
	 * non-whitespace character.
	 */

	// If "bd.startspaces" is set, "bd.textstart" points to the character,
	// the part of which is displayed at the block's beginning. Let's start
	// searching from the next character.
	if (bd.startspaces)
	    MB_PTR_ADV(non_white);

	// The character's column is in "bd.start_vcol".
	non_white_col = bd.start_vcol;

	init_chartabsize_arg(&cts, curwin, curwin->w_cursor.lnum,
				   non_white_col, bd.textstart, non_white);
	while (VIM_ISWHITE(*cts.cts_ptr))
	{
	    incr = lbr_chartabsize_adv(&cts);
	    cts.cts_vcol += incr;
	}
	non_white_col = cts.cts_vcol;
	non_white = cts.cts_ptr;
	clear_chartabsize_arg(&cts);

	block_space_width = non_white_col - oap->start_vcol;
	// We will shift by "total" or "block_space_width", whichever is less.
	shift_amount = (block_space_width < (size_t)total
					 ? block_space_width : (size_t)total);

	// The column to which we will shift the text.
	destination_col = (colnr_T)(non_white_col - shift_amount);

	// Now let's find out how much of the beginning of the line we can
	// reuse without modification.
	verbatim_copy_end = bd.textstart;
	verbatim_copy_width = bd.start_vcol;

	// If "bd.startspaces" is set, "bd.textstart" points to the character
	// preceding the block. We have to subtract its width to obtain its
	// column number.
	if (bd.startspaces)
	    verbatim_copy_width -= bd.start_char_vcols;
	init_chartabsize_arg(&cts, curwin, 0, verbatim_copy_width,
					     bd.textstart, verbatim_copy_end);
	while (cts.cts_vcol < destination_col)
	{
	    incr = lbr_chartabsize(&cts);
	    if (cts.cts_vcol + incr > destination_col)
		break;
	    cts.cts_vcol += incr;
	    MB_PTR_ADV(cts.cts_ptr);
	}
	verbatim_copy_width = cts.cts_vcol;
	verbatim_copy_end = cts.cts_ptr;
	clear_chartabsize_arg(&cts);

	// If "destination_col" is different from the width of the initial
	// part of the line that will be copied, it means we encountered a tab
	// character, which we will have to partly replace with spaces.
	fill = destination_col - verbatim_copy_width;

	// The replacement line will consist of:
	// - the beginning of the original line up to "verbatim_copy_end",
	// - "fill" number of spaces,
	// - the rest of the line, pointed to by non_white.
	new_line_len = (unsigned)(verbatim_copy_end - oldp)
		       + fill
		       + (unsigned)STRLEN(non_white);

	newp = alloc(new_line_len + 1);
	if (newp == NULL)
	    return;
	mch_memmove(newp, oldp, (size_t)(verbatim_copy_end - oldp));
	vim_memset(newp + (verbatim_copy_end - oldp), ' ', (size_t)fill);
	// Note that STRMOVE() copies the trailing NUL.
	STRMOVE(newp + (verbatim_copy_end - oldp) + fill, non_white);
    }
    // replace the line
    added = new_line_len - (int)STRLEN(oldp);
    ml_replace(curwin->w_cursor.lnum, newp, FALSE);
    inserted_bytes(curwin->w_cursor.lnum, bd.textcol, added);
    State = oldstate;
    curwin->w_cursor.col = oldcol;
#ifdef FEAT_RIGHTLEFT
    p_ri = old_p_ri;
#endif
}

/*
 * Insert string "s" (b_insert ? before : after) block :AKelly
 * Caller must prepare for undo.
 */
    static void
block_insert(
    oparg_T		*oap,
    char_u		*s,
    int			b_insert,
    struct block_def	*bdp)
{
    int		ts_val;
    int		count = 0;	// extra spaces to replace a cut TAB
    int		spaces = 0;	// non-zero if cutting a TAB
    colnr_T	offset;		// pointer along new line
    colnr_T	startcol;	// column where insert starts
    unsigned	s_len;		// STRLEN(s)
    char_u	*newp, *oldp;	// new, old lines
    linenr_T	lnum;		// loop var
    int		oldstate = State;

    State = MODE_INSERT;	// don't want MODE_REPLACE for State
    s_len = (unsigned)STRLEN(s);

    for (lnum = oap->start.lnum + 1; lnum <= oap->end.lnum; lnum++)
    {
	block_prep(oap, bdp, lnum, TRUE);
	if (bdp->is_short && b_insert)
	    continue;	// OP_INSERT, line ends before block start

	oldp = ml_get(lnum);

	if (b_insert)
	{
	    ts_val = bdp->start_char_vcols;
	    spaces = bdp->startspaces;
	    if (spaces != 0)
		count = ts_val - 1; // we're cutting a TAB
	    offset = bdp->textcol;
	}
	else // append
	{
	    ts_val = bdp->end_char_vcols;
	    if (!bdp->is_short) // spaces = padding after block
	    {
		spaces = (bdp->endspaces ? ts_val - bdp->endspaces : 0);
		if (spaces != 0)
		    count = ts_val - 1; // we're cutting a TAB
		offset = bdp->textcol + bdp->textlen - (spaces != 0);
	    }
	    else // spaces = padding to block edge
	    {
		// if $ used, just append to EOL (ie spaces==0)
		if (!bdp->is_MAX)
		    spaces = (oap->end_vcol - bdp->end_vcol) + 1;
		count = spaces;
		offset = bdp->textcol + bdp->textlen;
	    }
	}

	if (has_mbyte && spaces > 0)
	    // avoid copying part of a multi-byte character
	    offset -= (*mb_head_off)(oldp, oldp + offset);

	if (spaces < 0)  // can happen when the cursor was moved
	    spaces = 0;

	// Make sure the allocated size matches what is actually copied below.
	newp = alloc(STRLEN(oldp) + spaces + s_len
		    + (spaces > 0 && !bdp->is_short ? ts_val - spaces : 0)
								  + count + 1);
	if (newp == NULL)
	    continue;

	// copy up to shifted part
	mch_memmove(newp, oldp, (size_t)offset);
	oldp += offset;

	// insert pre-padding
	vim_memset(newp + offset, ' ', (size_t)spaces);
	startcol = offset + spaces;

	// copy the new text
	mch_memmove(newp + startcol, s, (size_t)s_len);
	offset += s_len;

	if (spaces > 0 && !bdp->is_short)
	{
	    if (*oldp == TAB)
	    {
		// insert post-padding
		vim_memset(newp + offset + spaces, ' ',
						    (size_t)(ts_val - spaces));
		// we're splitting a TAB, don't copy it
		oldp++;
		// We allowed for that TAB, remember this now
		count++;
	    }
	    else
		// Not a TAB, no extra spaces
		count = spaces;
	}

	if (spaces > 0)
	    offset += count;
	STRMOVE(newp + offset, oldp);

	ml_replace(lnum, newp, FALSE);

	if (b_insert)
	    // correct any text properties
	    inserted_bytes(lnum, startcol, s_len);

	if (lnum == oap->end.lnum)
	{
	    // Set "']" mark to the end of the block instead of the end of
	    // the insert in the first line.
	    curbuf->b_op_end.lnum = oap->end.lnum;
	    curbuf->b_op_end.col = offset;
	}
    } // for all lnum

    changed_lines(oap->start.lnum + 1, 0, oap->end.lnum + 1, 0L);

    State = oldstate;
}

/*
 * Handle a delete operation.
 *
 * Return FAIL if undo failed, OK otherwise.
 */
    int
op_delete(oparg_T *oap)
{
    int			n;
    linenr_T		lnum;
    char_u		*ptr;
    char_u		*newp, *oldp;
    struct block_def	bd;
    linenr_T		old_lcount = curbuf->b_ml.ml_line_count;
    int			did_yank = FALSE;

    if (curbuf->b_ml.ml_flags & ML_EMPTY)	    // nothing to do
	return OK;

    // Nothing to delete, return here.	Do prepare undo, for op_change().
    if (oap->empty)
	return u_save_cursor();

    if (!curbuf->b_p_ma)
    {
	emsg(_(e_cannot_make_changes_modifiable_is_off));
	return FAIL;
    }

    if (VIsual_select && oap->is_VIsual)
	// use register given with CTRL_R, defaults to zero
	oap->regname = VIsual_select_reg;

#ifdef FEAT_CLIPBOARD
    adjust_clip_reg(&oap->regname);
#endif

    if (has_mbyte)
	mb_adjust_opend(oap);

    /*
     * Imitate the strange Vi behaviour: If the delete spans more than one
     * line and motion_type == MCHAR and the result is a blank line, make the
     * delete linewise.  Don't do this for the change command or Visual mode.
     */
    if (       oap->motion_type == MCHAR
	    && !oap->is_VIsual
	    && !oap->block_mode
	    && oap->line_count > 1
	    && oap->motion_force == NUL
	    && oap->op_type == OP_DELETE)
    {
	ptr = ml_get(oap->end.lnum) + oap->end.col;
	if (*ptr != NUL)
	    ptr += oap->inclusive;
	ptr = skipwhite(ptr);
	if (*ptr == NUL && inindent(0))
	    oap->motion_type = MLINE;
    }

    /*
     * Check for trying to delete (e.g. "D") in an empty line.
     * Note: For the change operator it is ok.
     */
    if (       oap->motion_type == MCHAR
	    && oap->line_count == 1
	    && oap->op_type == OP_DELETE
	    && *ml_get(oap->start.lnum) == NUL)
    {
	/*
	 * It's an error to operate on an empty region, when 'E' included in
	 * 'cpoptions' (Vi compatible).
	 */
	if (virtual_op)
	    // Virtual editing: Nothing gets deleted, but we set the '[ and ']
	    // marks as if it happened.
	    goto setmarks;
	if (vim_strchr(p_cpo, CPO_EMPTYREGION) != NULL)
	    beep_flush();
	return OK;
    }

    /*
     * Do a yank of whatever we're about to delete.
     * If a yank register was specified, put the deleted text into that
     * register.  For the black hole register '_' don't yank anything.
     */
    if (oap->regname != '_')
    {
	if (oap->regname != 0)
	{
	    // check for read-only register
	    if (!valid_yank_reg(oap->regname, TRUE))
	    {
		beep_flush();
		return OK;
	    }
	    get_yank_register(oap->regname, TRUE); // yank into specif'd reg.
	    if (op_yank(oap, TRUE, FALSE) == OK)   // yank without message
		did_yank = TRUE;
	}
	else
	    reset_y_append(); // not appending to unnamed register

	/*
	 * Put deleted text into register 1 and shift number registers if the
	 * delete contains a line break, or when using a specific operator (Vi
	 * compatible)
	 */
	if (oap->motion_type == MLINE || oap->line_count > 1
							   || oap->use_reg_one)
	{
	    shift_delete_registers();
	    if (op_yank(oap, TRUE, FALSE) == OK)
		did_yank = TRUE;
	}

	// Yank into small delete register when no named register specified
	// and the delete is within one line.
	if ((
#ifdef FEAT_CLIPBOARD
	    ((clip_unnamed & CLIP_UNNAMED) && oap->regname == '*') ||
	    ((clip_unnamed & CLIP_UNNAMED_PLUS) && oap->regname == '+') ||
#endif
	    oap->regname == 0) && oap->motion_type != MLINE
						      && oap->line_count == 1)
	{
	    oap->regname = '-';
	    get_yank_register(oap->regname, TRUE);
	    if (op_yank(oap, TRUE, FALSE) == OK)
		did_yank = TRUE;
	    oap->regname = 0;
	}

	/*
	 * If there's too much stuff to fit in the yank register, then get a
	 * confirmation before doing the delete. This is crude, but simple.
	 * And it avoids doing a delete of something we can't put back if we
	 * want.
	 */
	if (!did_yank)
	{
	    int msg_silent_save = msg_silent;

	    msg_silent = 0;	// must display the prompt
	    n = ask_yesno((char_u *)_("cannot yank; delete anyway"), TRUE);
	    msg_silent = msg_silent_save;
	    if (n != 'y')
	    {
		emsg(_(e_command_aborted));
		return FAIL;
	    }
	}

#if defined(FEAT_EVAL)
	if (did_yank && has_textyankpost())
	    yank_do_autocmd(oap, get_y_current());
#endif
    }

    /*
     * block mode delete
     */
    if (oap->block_mode)
    {
	if (u_save((linenr_T)(oap->start.lnum - 1),
			       (linenr_T)(oap->end.lnum + 1)) == FAIL)
	    return FAIL;

	for (lnum = curwin->w_cursor.lnum; lnum <= oap->end.lnum; ++lnum)
	{
	    block_prep(oap, &bd, lnum, TRUE);
	    if (bd.textlen == 0)	// nothing to delete
		continue;

	    // Adjust cursor position for tab replaced by spaces and 'lbr'.
	    if (lnum == curwin->w_cursor.lnum)
	    {
		curwin->w_cursor.col = bd.textcol + bd.startspaces;
		curwin->w_cursor.coladd = 0;
	    }

	    // "n" == number of chars deleted
	    // If we delete a TAB, it may be replaced by several characters.
	    // Thus the number of characters may increase!
	    n = bd.textlen - bd.startspaces - bd.endspaces;
	    oldp = ml_get(lnum);
	    newp = alloc(STRLEN(oldp) + 1 - n);
	    if (newp == NULL)
		continue;
	    // copy up to deleted part
	    mch_memmove(newp, oldp, (size_t)bd.textcol);
	    // insert spaces
	    vim_memset(newp + bd.textcol, ' ',
				     (size_t)(bd.startspaces + bd.endspaces));
	    // copy the part after the deleted part
	    oldp += bd.textcol + bd.textlen;
	    STRMOVE(newp + bd.textcol + bd.startspaces + bd.endspaces, oldp);
	    // replace the line
	    ml_replace(lnum, newp, FALSE);

#ifdef FEAT_PROP_POPUP
	    if (curbuf->b_has_textprop && n != 0)
		adjust_prop_columns(lnum, bd.textcol, -n, 0);
#endif
	}

	check_cursor_col();
	changed_lines(curwin->w_cursor.lnum, curwin->w_cursor.col,
						       oap->end.lnum + 1, 0L);
	oap->line_count = 0;	    // no lines deleted
    }
    else if (oap->motion_type == MLINE)
    {
	if (oap->op_type == OP_CHANGE)
	{
	    // Delete the lines except the first one.  Temporarily move the
	    // cursor to the next line.  Save the current line number, if the
	    // last line is deleted it may be changed.
	    if (oap->line_count > 1)
	    {
		lnum = curwin->w_cursor.lnum;
		++curwin->w_cursor.lnum;
		del_lines((long)(oap->line_count - 1), TRUE);
		curwin->w_cursor.lnum = lnum;
	    }
	    if (u_save_cursor() == FAIL)
		return FAIL;
	    if (curbuf->b_p_ai)		    // don't delete indent
	    {
		beginline(BL_WHITE);	    // cursor on first non-white
		did_ai = TRUE;		    // delete the indent when ESC hit
		ai_col = curwin->w_cursor.col;
	    }
	    else
		beginline(0);		    // cursor in column 0
	    truncate_line(FALSE);   // delete the rest of the line
				    // leave cursor past last char in line
	    if (oap->line_count > 1)
		u_clearline();	    // "U" command not possible after "2cc"
	}
	else
	{
	    del_lines(oap->line_count, TRUE);
	    beginline(BL_WHITE | BL_FIX);
	    u_clearline();	// "U" command not possible after "dd"
	}
    }
    else
    {
	if (virtual_op)
	{
	    int		endcol = 0;

	    // For virtualedit: break the tabs that are partly included.
	    if (gchar_pos(&oap->start) == '\t')
	    {
		if (u_save_cursor() == FAIL)	// save first line for undo
		    return FAIL;
		if (oap->line_count == 1)
		    endcol = getviscol2(oap->end.col, oap->end.coladd);
		coladvance_force(getviscol2(oap->start.col, oap->start.coladd));
		oap->start = curwin->w_cursor;
		if (oap->line_count == 1)
		{
		    coladvance(endcol);
		    oap->end.col = curwin->w_cursor.col;
		    oap->end.coladd = curwin->w_cursor.coladd;
		    curwin->w_cursor = oap->start;
		}
	    }

	    // Break a tab only when it's included in the area.
	    if (gchar_pos(&oap->end) == '\t'
				     && (int)oap->end.coladd < oap->inclusive)
	    {
		// save last line for undo
		if (u_save((linenr_T)(oap->end.lnum - 1),
				       (linenr_T)(oap->end.lnum + 1)) == FAIL)
		    return FAIL;
		curwin->w_cursor = oap->end;
		coladvance_force(getviscol2(oap->end.col, oap->end.coladd));
		oap->end = curwin->w_cursor;
		curwin->w_cursor = oap->start;
	    }
	    if (has_mbyte)
		mb_adjust_opend(oap);
	}

	if (oap->line_count == 1)	// delete characters within one line
	{
	    if (u_save_cursor() == FAIL)	// save line for undo
		return FAIL;

	    // if 'cpoptions' contains '$', display '$' at end of change
	    if (       vim_strchr(p_cpo, CPO_DOLLAR) != NULL
		    && oap->op_type == OP_CHANGE
		    && oap->end.lnum == curwin->w_cursor.lnum
		    && !oap->is_VIsual)
		display_dollar(oap->end.col - !oap->inclusive);

	    n = oap->end.col - oap->start.col + 1 - !oap->inclusive;

	    if (virtual_op)
	    {
		// fix up things for virtualedit-delete:
		// break the tabs which are going to get in our way
		char_u		*curline = ml_get_curline();
		int		len = (int)STRLEN(curline);

		if (oap->end.coladd != 0
			&& (int)oap->end.col >= len - 1
			&& !(oap->start.coladd && (int)oap->end.col >= len - 1))
		    n++;
		// Delete at least one char (e.g, when on a control char).
		if (n == 0 && oap->start.coladd != oap->end.coladd)
		    n = 1;

		// When deleted a char in the line, reset coladd.
		if (gchar_cursor() != NUL)
		    curwin->w_cursor.coladd = 0;
	    }
	    (void)del_bytes((long)n, !virtual_op,
			    oap->op_type == OP_DELETE && !oap->is_VIsual);
	}
	else				// delete characters between lines
	{
	    pos_T   curpos;

	    // save deleted and changed lines for undo
	    if (u_save((linenr_T)(curwin->w_cursor.lnum - 1),
		 (linenr_T)(curwin->w_cursor.lnum + oap->line_count)) == FAIL)
		return FAIL;

	    truncate_line(TRUE);	// delete from cursor to end of line

	    curpos = curwin->w_cursor;	// remember curwin->w_cursor
	    ++curwin->w_cursor.lnum;
	    del_lines((long)(oap->line_count - 2), FALSE);

	    // delete from start of line until op_end
	    n = (oap->end.col + 1 - !oap->inclusive);
	    curwin->w_cursor.col = 0;
	    (void)del_bytes((long)n, !virtual_op,
			    oap->op_type == OP_DELETE && !oap->is_VIsual);
	    curwin->w_cursor = curpos;	// restore curwin->w_cursor
	    (void)do_join(2, FALSE, FALSE, FALSE, FALSE);
	}
	if (oap->op_type == OP_DELETE)
	    auto_format(FALSE, TRUE);
    }

    msgmore(curbuf->b_ml.ml_line_count - old_lcount);

setmarks:
    if ((cmdmod.cmod_flags & CMOD_LOCKMARKS) == 0)
    {
	if (oap->block_mode)
	{
	    curbuf->b_op_end.lnum = oap->end.lnum;
	    curbuf->b_op_end.col = oap->start.col;
	}
	else
	    curbuf->b_op_end = oap->start;
	curbuf->b_op_start = oap->start;
    }

    return OK;
}

/*
 * Adjust end of operating area for ending on a multi-byte character.
 * Used for deletion.
 */
    static void
mb_adjust_opend(oparg_T *oap)
{
    char_u	*p;

    if (!oap->inclusive)
	return;

    p = ml_get(oap->end.lnum);
    oap->end.col += mb_tail_off(p, p + oap->end.col);
}

/*
 * Replace the character under the cursor with "c".
 * This takes care of multi-byte characters.
 */
    static void
replace_character(int c)
{
    int n = State;

    State = MODE_REPLACE;
    ins_char(c);
    State = n;
    // Backup to the replaced character.
    dec_cursor();
}

/*
 * Replace a whole area with one character.
 */
    int
op_replace(oparg_T *oap, int c)
{
    int			n, numc;
    int			num_chars;
    char_u		*newp, *oldp;
    size_t		oldlen;
    struct block_def	bd;
    char_u		*after_p = NULL;
    int			had_ctrl_v_cr = FALSE;

    if ((curbuf->b_ml.ml_flags & ML_EMPTY ) || oap->empty)
	return OK;	    // nothing to do

    if (c == REPLACE_CR_NCHAR)
    {
	had_ctrl_v_cr = TRUE;
	c = CAR;
    }
    else if (c == REPLACE_NL_NCHAR)
    {
	had_ctrl_v_cr = TRUE;
	c = NL;
    }

    if (has_mbyte)
	mb_adjust_opend(oap);

    if (u_save((linenr_T)(oap->start.lnum - 1),
				       (linenr_T)(oap->end.lnum + 1)) == FAIL)
	return FAIL;

    /*
     * block mode replace
     */
    if (oap->block_mode)
    {
	bd.is_MAX = (curwin->w_curswant == MAXCOL);
	for ( ; curwin->w_cursor.lnum <= oap->end.lnum; ++curwin->w_cursor.lnum)
	{
	    curwin->w_cursor.col = 0;  // make sure cursor position is valid
	    block_prep(oap, &bd, curwin->w_cursor.lnum, TRUE);
	    if (bd.textlen == 0 && (!virtual_op || bd.is_MAX))
		continue;	    // nothing to replace

	    // n == number of extra chars required
	    // If we split a TAB, it may be replaced by several characters.
	    // Thus the number of characters may increase!
	    // If the range starts in virtual space, count the initial
	    // coladd offset as part of "startspaces"
	    if (virtual_op && bd.is_short && *bd.textstart == NUL)
	    {
		pos_T vpos;

		vpos.lnum = curwin->w_cursor.lnum;
		getvpos(&vpos, oap->start_vcol);
		bd.startspaces += vpos.coladd;
		n = bd.startspaces;
	    }
	    else
		// allow for pre spaces
		n = (bd.startspaces ? bd.start_char_vcols - 1 : 0);

	    // allow for post spp
	    n += (bd.endspaces
		    && !bd.is_oneChar
		    && bd.end_char_vcols > 0) ? bd.end_char_vcols - 1 : 0;
	    // Figure out how many characters to replace.
	    numc = oap->end_vcol - oap->start_vcol + 1;
	    if (bd.is_short && (!virtual_op || bd.is_MAX))
		numc -= (oap->end_vcol - bd.end_vcol) + 1;

	    // A double-wide character can be replaced only up to half the
	    // times.
	    if ((*mb_char2cells)(c) > 1)
	    {
		if ((numc & 1) && !bd.is_short)
		{
		    ++bd.endspaces;
		    ++n;
		}
		numc = numc / 2;
	    }

	    // Compute bytes needed, move character count to num_chars.
	    num_chars = numc;
	    numc *= (*mb_char2len)(c);
	    // oldlen includes textlen, so don't double count
	    n += numc - bd.textlen;

	    oldp = ml_get_curline();
	    oldlen = STRLEN(oldp);
	    newp = alloc(oldlen + 1 + n);
	    if (newp == NULL)
		continue;
	    vim_memset(newp, NUL, (size_t)(oldlen + 1 + n));
	    // copy up to deleted part
	    mch_memmove(newp, oldp, (size_t)bd.textcol);
	    oldp += bd.textcol + bd.textlen;
	    // insert pre-spaces
	    vim_memset(newp + bd.textcol, ' ', (size_t)bd.startspaces);
	    // insert replacement chars CHECK FOR ALLOCATED SPACE
	    // REPLACE_CR_NCHAR/REPLACE_NL_NCHAR is used for entering CR
	    // literally.
	    if (had_ctrl_v_cr || (c != '\r' && c != '\n'))
	    {
		if (has_mbyte)
		{
		    n = (int)STRLEN(newp);
		    while (--num_chars >= 0)
			n += (*mb_char2bytes)(c, newp + n);
		}
		else
		    vim_memset(newp + STRLEN(newp), c, (size_t)numc);
		if (!bd.is_short)
		{
		    // insert post-spaces
		    vim_memset(newp + STRLEN(newp), ' ', (size_t)bd.endspaces);
		    // copy the part after the changed part
		    STRMOVE(newp + STRLEN(newp), oldp);
		}
	    }
	    else
	    {
		// Replacing with \r or \n means splitting the line.
		after_p = alloc(oldlen + 1 + n - STRLEN(newp));
		if (after_p != NULL)
		    STRMOVE(after_p, oldp);
	    }
	    // replace the line
	    ml_replace(curwin->w_cursor.lnum, newp, FALSE);
	    if (after_p != NULL)
	    {
		ml_append(curwin->w_cursor.lnum++, after_p, 0, FALSE);
		appended_lines_mark(curwin->w_cursor.lnum, 1L);
		oap->end.lnum++;
		vim_free(after_p);
	    }
	}
    }
    else
    {
	/*
	 * MCHAR and MLINE motion replace.
	 */
	if (oap->motion_type == MLINE)
	{
	    oap->start.col = 0;
	    curwin->w_cursor.col = 0;
	    oap->end.col = (colnr_T)STRLEN(ml_get(oap->end.lnum));
	    if (oap->end.col)
		--oap->end.col;
	}
	else if (!oap->inclusive)
	    dec(&(oap->end));

	while (LTOREQ_POS(curwin->w_cursor, oap->end))
	{
	    int done = FALSE;

	    n = gchar_cursor();
	    if (n != NUL)
	    {
		int new_byte_len = (*mb_char2len)(c);
		int old_byte_len = mb_ptr2len(ml_get_cursor());

		if (new_byte_len > 1 || old_byte_len > 1)
		{
		    // This is slow, but it handles replacing a single-byte
		    // with a multi-byte and the other way around.
		    if (curwin->w_cursor.lnum == oap->end.lnum)
			oap->end.col += new_byte_len - old_byte_len;
		    replace_character(c);
		    done = TRUE;
		}
		else
		{
		    if (n == TAB)
		    {
			int end_vcol = 0;

			if (curwin->w_cursor.lnum == oap->end.lnum)
			{
			    // oap->end has to be recalculated when
			    // the tab breaks
			    end_vcol = getviscol2(oap->end.col,
							     oap->end.coladd);
			}
			coladvance_force(getviscol());
			if (curwin->w_cursor.lnum == oap->end.lnum)
			    getvpos(&oap->end, end_vcol);
		    }
		    // with "coladd" set may move to just after a TAB
		    if (gchar_cursor() != NUL)
		    {
			PBYTE(curwin->w_cursor, c);
			done = TRUE;
		    }
		}
	    }
	    if (!done && virtual_op && curwin->w_cursor.lnum == oap->end.lnum)
	    {
		int virtcols = oap->end.coladd;

		if (curwin->w_cursor.lnum == oap->start.lnum
			&& oap->start.col == oap->end.col && oap->start.coladd)
		    virtcols -= oap->start.coladd;

		// oap->end has been trimmed so it's effectively inclusive;
		// as a result an extra +1 must be counted so we don't
		// trample the NUL byte.
		coladvance_force(getviscol2(oap->end.col, oap->end.coladd) + 1);
		curwin->w_cursor.col -= (virtcols + 1);
		for (; virtcols >= 0; virtcols--)
		{
		    if ((*mb_char2len)(c) > 1)
		       replace_character(c);
		    else
			PBYTE(curwin->w_cursor, c);
		   if (inc(&curwin->w_cursor) == -1)
		       break;
		}
	    }

	    // Advance to next character, stop at the end of the file.
	    if (inc_cursor() == -1)
		break;
	}
    }

    curwin->w_cursor = oap->start;
    check_cursor();
    changed_lines(oap->start.lnum, oap->start.col, oap->end.lnum + 1, 0L);

    if ((cmdmod.cmod_flags & CMOD_LOCKMARKS) == 0)
    {
	// Set "'[" and "']" marks.
	curbuf->b_op_start = oap->start;
	curbuf->b_op_end = oap->end;
    }

    return OK;
}

static int swapchars(int op_type, pos_T *pos, int length);

/*
 * Handle the (non-standard vi) tilde operator.  Also for "gu", "gU" and "g?".
 */
    static void
op_tilde(oparg_T *oap)
{
    pos_T		pos;
    struct block_def	bd;
    int			did_change = FALSE;

    if (u_save((linenr_T)(oap->start.lnum - 1),
				       (linenr_T)(oap->end.lnum + 1)) == FAIL)
	return;

    pos = oap->start;
    if (oap->block_mode)		    // Visual block mode
    {
	for (; pos.lnum <= oap->end.lnum; ++pos.lnum)
	{
	    int one_change;

	    block_prep(oap, &bd, pos.lnum, FALSE);
	    pos.col = bd.textcol;
	    one_change = swapchars(oap->op_type, &pos, bd.textlen);
	    did_change |= one_change;

#ifdef FEAT_NETBEANS_INTG
	    if (netbeans_active() && one_change)
	    {
		char_u *ptr;

		netbeans_removed(curbuf, pos.lnum, bd.textcol,
							    (long)bd.textlen);
		// get the line now, it may have been flushed
		ptr = ml_get_buf(curbuf, pos.lnum, FALSE);
		netbeans_inserted(curbuf, pos.lnum, bd.textcol,
						&ptr[bd.textcol], bd.textlen);
	    }
#endif
	}
	if (did_change)
	    changed_lines(oap->start.lnum, 0, oap->end.lnum + 1, 0L);
    }
    else				    // not block mode
    {
	if (oap->motion_type == MLINE)
	{
	    oap->start.col = 0;
	    pos.col = 0;
	    oap->end.col = (colnr_T)STRLEN(ml_get(oap->end.lnum));
	    if (oap->end.col)
		--oap->end.col;
	}
	else if (!oap->inclusive)
	    dec(&(oap->end));

	if (pos.lnum == oap->end.lnum)
	    did_change = swapchars(oap->op_type, &pos,
						  oap->end.col - pos.col + 1);
	else
	    for (;;)
	    {
		did_change |= swapchars(oap->op_type, &pos,
				pos.lnum == oap->end.lnum ? oap->end.col + 1:
					   (int)STRLEN(ml_get_pos(&pos)));
		if (LTOREQ_POS(oap->end, pos) || inc(&pos) == -1)
		    break;
	    }
	if (did_change)
	{
	    changed_lines(oap->start.lnum, oap->start.col, oap->end.lnum + 1,
									  0L);
#ifdef FEAT_NETBEANS_INTG
	    if (netbeans_active())
	    {
		char_u *ptr;
		int count;

		pos = oap->start;
		while (pos.lnum < oap->end.lnum)
		{
		    ptr = ml_get_buf(curbuf, pos.lnum, FALSE);
		    count = (int)STRLEN(ptr) - pos.col;
		    netbeans_removed(curbuf, pos.lnum, pos.col, (long)count);
		    // get the line again, it may have been flushed
		    ptr = ml_get_buf(curbuf, pos.lnum, FALSE);
		    netbeans_inserted(curbuf, pos.lnum, pos.col,
							&ptr[pos.col], count);
		    pos.col = 0;
		    pos.lnum++;
		}
		count = oap->end.col - pos.col + 1;
		netbeans_removed(curbuf, pos.lnum, pos.col, (long)count);
		// get the line again, it may have been flushed
		ptr = ml_get_buf(curbuf, pos.lnum, FALSE);
		netbeans_inserted(curbuf, pos.lnum, pos.col,
							&ptr[pos.col], count);
	    }
#endif
	}
    }

    if (!did_change && oap->is_VIsual)
	// No change: need to remove the Visual selection
	redraw_curbuf_later(UPD_INVERTED);

    if ((cmdmod.cmod_flags & CMOD_LOCKMARKS) == 0)
    {
	// Set '[ and '] marks.
	curbuf->b_op_start = oap->start;
	curbuf->b_op_end = oap->end;
    }

    if (oap->line_count > p_report)
	smsg(NGETTEXT("%ld line changed", "%ld lines changed",
					    oap->line_count), oap->line_count);
}

/*
 * Invoke swapchar() on "length" bytes at position "pos".
 * "pos" is advanced to just after the changed characters.
 * "length" is rounded up to include the whole last multi-byte character.
 * Also works correctly when the number of bytes changes.
 * Returns TRUE if some character was changed.
 */
    static int
swapchars(int op_type, pos_T *pos, int length)
{
    int todo;
    int	did_change = 0;

    for (todo = length; todo > 0; --todo)
    {
	if (has_mbyte)
	{
	    int len = (*mb_ptr2len)(ml_get_pos(pos));

	    // we're counting bytes, not characters
	    if (len > 0)
		todo -= len - 1;
	}
	did_change |= swapchar(op_type, pos);
	if (inc(pos) == -1)    // at end of file
	    break;
    }
    return did_change;
}

/*
 * If op_type == OP_UPPER: make uppercase,
 * if op_type == OP_LOWER: make lowercase,
 * if op_type == OP_ROT13: do rot13 encoding,
 * else swap case of character at 'pos'
 * returns TRUE when something actually changed.
 */
    int
swapchar(int op_type, pos_T *pos)
{
    int	    c;
    int	    nc;

    c = gchar_pos(pos);

    // Only do rot13 encoding for ASCII characters.
    if (c >= 0x80 && op_type == OP_ROT13)
	return FALSE;

    if (op_type == OP_UPPER && c == 0xdf
		      && (enc_latin1like || STRCMP(p_enc, "iso-8859-2") == 0))
    {
	pos_T   sp = curwin->w_cursor;

	// Special handling of German sharp s: change to "SS".
	curwin->w_cursor = *pos;
	del_char(FALSE);
	ins_char('S');
	ins_char('S');
	curwin->w_cursor = sp;
	inc(pos);
    }

    if (enc_dbcs != 0 && c >= 0x100)	// No lower/uppercase letter
	return FALSE;
    nc = c;
    if (MB_ISLOWER(c))
    {
	if (op_type == OP_ROT13)
	    nc = ROT13(c, 'a');
	else if (op_type != OP_LOWER)
	    nc = MB_TOUPPER(c);
    }
    else if (MB_ISUPPER(c))
    {
	if (op_type == OP_ROT13)
	    nc = ROT13(c, 'A');
	else if (op_type != OP_UPPER)
	    nc = MB_TOLOWER(c);
    }
    if (nc != c)
    {
	if (enc_utf8 && (c >= 0x80 || nc >= 0x80))
	{
	    pos_T   sp = curwin->w_cursor;

	    curwin->w_cursor = *pos;
	    // don't use del_char(), it also removes composing chars
	    del_bytes(utf_ptr2len(ml_get_cursor()), FALSE, FALSE);
	    ins_char(nc);
	    curwin->w_cursor = sp;
	}
	else
	    PBYTE(*pos, nc);
	return TRUE;
    }
    return FALSE;
}

/*
 * op_insert - Insert and append operators for Visual mode.
 */
    void
op_insert(oparg_T *oap, long count1)
{
    long		ins_len, pre_textlen = 0;
    char_u		*firstline, *ins_text;
    colnr_T		ind_pre_col = 0, ind_post_col;
    int			ind_pre_vcol = 0, ind_post_vcol = 0;
    struct block_def	bd;
    int			i;
    pos_T		t1;
    pos_T		start_insert;
			// offset when cursor was moved in insert mode
    int			offset = 0;

    // edit() changes this - record it for OP_APPEND
    bd.is_MAX = (curwin->w_curswant == MAXCOL);

    // vis block is still marked. Get rid of it now.
    curwin->w_cursor.lnum = oap->start.lnum;
    update_screen(UPD_INVERTED);

    if (oap->block_mode)
    {
	// When 'virtualedit' is used, need to insert the extra spaces before
	// doing block_prep().  When only "block" is used, virtual edit is
	// already disabled, but still need it when calling
	// coladvance_force().
	// coladvance_force() uses get_ve_flags() to get the 'virtualedit'
	// state for the current window.  To override that state, we need to
	// set the window-local value of ve_flags rather than the global value.
	if (curwin->w_cursor.coladd > 0)
	{
	    int		old_ve_flags = curwin->w_ve_flags;

	    if (u_save_cursor() == FAIL)
		return;

	    curwin->w_ve_flags = VE_ALL;
	    coladvance_force(oap->op_type == OP_APPEND
					   ? oap->end_vcol + 1 : getviscol());
	    if (oap->op_type == OP_APPEND)
		--curwin->w_cursor.col;
	    curwin->w_ve_flags = old_ve_flags;
	}
	// Get the info about the block before entering the text
	block_prep(oap, &bd, oap->start.lnum, TRUE);
	// Get indent information
	ind_pre_col = (colnr_T)getwhitecols_curline();
	ind_pre_vcol = get_indent();
	firstline = ml_get(oap->start.lnum) + bd.textcol;

	if (oap->op_type == OP_APPEND)
	    firstline += bd.textlen;
	pre_textlen = (long)STRLEN(firstline);
    }

    if (oap->op_type == OP_APPEND)
    {
	if (oap->block_mode && curwin->w_cursor.coladd == 0)
	{
	    // Move the cursor to the character right of the block.
	    curwin->w_set_curswant = TRUE;
	    while (*ml_get_cursor() != NUL
		    && (curwin->w_cursor.col < bd.textcol + bd.textlen))
		++curwin->w_cursor.col;
	    if (bd.is_short && !bd.is_MAX)
	    {
		// First line was too short, make it longer and adjust the
		// values in "bd".
		if (u_save_cursor() == FAIL)
		    return;
		for (i = 0; i < bd.endspaces; ++i)
		    ins_char(' ');
		bd.textlen += bd.endspaces;
	    }
	}
	else
	{
	    curwin->w_cursor = oap->end;
	    check_cursor_col();

	    // Works just like an 'i'nsert on the next character.
	    if (!LINEEMPTY(curwin->w_cursor.lnum)
		    && oap->start_vcol != oap->end_vcol)
		inc_cursor();
	}
    }

    t1 = oap->start;
    start_insert = curwin->w_cursor;
    (void)edit(NUL, FALSE, (linenr_T)count1);

    // When a tab was inserted, and the characters in front of the tab
    // have been converted to a tab as well, the column of the cursor
    // might have actually been reduced, so need to adjust here.
    if (t1.lnum == curbuf->b_op_start_orig.lnum
	    && LT_POS(curbuf->b_op_start_orig, t1))
	oap->start = curbuf->b_op_start_orig;

    // If user has moved off this line, we don't know what to do, so do
    // nothing.
    // Also don't repeat the insert when Insert mode ended with CTRL-C.
    if (curwin->w_cursor.lnum != oap->start.lnum || got_int)
	return;

    if (oap->block_mode)
    {
	struct block_def	bd2;
	int			did_indent = FALSE;
	size_t			len;
	int			add;

	// If indent kicked in, the firstline might have changed
	// but only do that, if the indent actually increased.
	ind_post_col = (colnr_T)getwhitecols_curline();
	if (curbuf->b_op_start.col > ind_pre_col && ind_post_col > ind_pre_col)
	{
	    bd.textcol += ind_post_col - ind_pre_col;
	    ind_post_vcol = get_indent();
	    bd.start_vcol += ind_post_vcol - ind_pre_vcol;
	    did_indent = TRUE;
	}

	// The user may have moved the cursor before inserting something, try
	// to adjust the block for that.  But only do it, if the difference
	// does not come from indent kicking in.
	if (oap->start.lnum == curbuf->b_op_start_orig.lnum
						  && !bd.is_MAX && !did_indent)
	{
	    int t = getviscol2(curbuf->b_op_start_orig.col,
					       curbuf->b_op_start_orig.coladd);

	    if (oap->op_type == OP_INSERT
		    && oap->start.col + oap->start.coladd
			    != curbuf->b_op_start_orig.col
					      + curbuf->b_op_start_orig.coladd)
	    {
		oap->start.col = curbuf->b_op_start_orig.col;
		pre_textlen -= t - oap->start_vcol;
		oap->start_vcol = t;
	    }
	    else if (oap->op_type == OP_APPEND
		    && oap->start.col + oap->start.coladd
			    >= curbuf->b_op_start_orig.col
					      + curbuf->b_op_start_orig.coladd)
	    {
		oap->start.col = curbuf->b_op_start_orig.col;
		// reset pre_textlen to the value of OP_INSERT
		pre_textlen += bd.textlen;
		pre_textlen -= t - oap->start_vcol;
		oap->start_vcol = t;
		oap->op_type = OP_INSERT;
	    }
	}

	// Spaces and tabs in the indent may have changed to other spaces and
	// tabs.  Get the starting column again and correct the length.
	// Don't do this when "$" used, end-of-line will have changed.
	//
	// if indent was added and the inserted text was after the indent,
	// correct the selection for the new indent.
	if (did_indent && bd.textcol - ind_post_col > 0)
	{
	    oap->start.col += ind_post_col - ind_pre_col;
	    oap->start_vcol += ind_post_vcol - ind_pre_vcol;
	    oap->end.col += ind_post_col - ind_pre_col;
	    oap->end_vcol += ind_post_vcol - ind_pre_vcol;
	}
	block_prep(oap, &bd2, oap->start.lnum, TRUE);
	if (did_indent && bd.textcol - ind_post_col > 0)
	{
	    // undo for where "oap" is used below
	    oap->start.col -= ind_post_col - ind_pre_col;
	    oap->start_vcol -= ind_post_vcol - ind_pre_vcol;
	    oap->end.col -= ind_post_col - ind_pre_col;
	    oap->end_vcol -= ind_post_vcol - ind_pre_vcol;
	}
	if (!bd.is_MAX || bd2.textlen < bd.textlen)
	{
	    if (oap->op_type == OP_APPEND)
	    {
		pre_textlen += bd2.textlen - bd.textlen;
		if (bd2.endspaces)
		    --bd2.textlen;
	    }
	    bd.textcol = bd2.textcol;
	    bd.textlen = bd2.textlen;
	}

	/*
	 * Subsequent calls to ml_get() flush the firstline data - take a
	 * copy of the required string.
	 */
	firstline = ml_get(oap->start.lnum);
	len = STRLEN(firstline);
	add = bd.textcol;
	if (oap->op_type == OP_APPEND)
	{
	    add += bd.textlen;
	    // account for pressing cursor in insert mode when '$' was used
	    if (bd.is_MAX
		&& (start_insert.lnum == Insstart.lnum
					   && start_insert.col > Insstart.col))
	    {
		offset = (start_insert.col - Insstart.col);
		add -= offset;
		if (oap->end_vcol > offset)
		    oap->end_vcol -= (offset + 1);
		else
		    // moved outside of the visual block, what to do?
		    return;
	    }
	}
	if ((size_t)add > len)
	    firstline += len;  // short line, point to the NUL
	else
	    firstline += add;
	if (pre_textlen >= 0 && (ins_len =
			   (long)STRLEN(firstline) - pre_textlen - offset) > 0)
	{
	    ins_text = vim_strnsave(firstline, ins_len);
	    if (ins_text != NULL)
	    {
		// block handled here
		if (u_save(oap->start.lnum,
					 (linenr_T)(oap->end.lnum + 1)) == OK)
		    block_insert(oap, ins_text, (oap->op_type == OP_INSERT),
									 &bd);

		curwin->w_cursor.col = oap->start.col;
		check_cursor();
		vim_free(ins_text);
	    }
	}
    }
}

/*
 * op_change - handle a change operation
 *
 * return TRUE if edit() returns because of a CTRL-O command
 */
    int
op_change(oparg_T *oap)
{
    colnr_T		l;
    int			retval;
    long		offset;
    linenr_T		linenr;
    long		ins_len;
    long		pre_textlen = 0;
    long		pre_indent = 0;
    char_u		*firstline;
    char_u		*ins_text, *newp, *oldp;
    struct block_def	bd;

    l = oap->start.col;
    if (oap->motion_type == MLINE)
    {
	l = 0;
	can_si = may_do_si();	// Like opening a new line, do smart indent
    }

    // First delete the text in the region.  In an empty buffer only need to
    // save for undo
    if (curbuf->b_ml.ml_flags & ML_EMPTY)
    {
	if (u_save_cursor() == FAIL)
	    return FALSE;
    }
    else if (op_delete(oap) == FAIL)
	return FALSE;

    if ((l > curwin->w_cursor.col) && !LINEEMPTY(curwin->w_cursor.lnum)
							 && !virtual_op)
	inc_cursor();

    // check for still on same line (<CR> in inserted text meaningless)
    // skip blank lines too
    if (oap->block_mode)
    {
	// Add spaces before getting the current line length.
	if (virtual_op && (curwin->w_cursor.coladd > 0
						    || gchar_cursor() == NUL))
	    coladvance_force(getviscol());
	firstline = ml_get(oap->start.lnum);
	pre_textlen = (long)STRLEN(firstline);
	pre_indent = (long)getwhitecols(firstline);
	bd.textcol = curwin->w_cursor.col;
    }

    if (oap->motion_type == MLINE)
	fix_indent();

    // Reset finish_op now, don't want it set inside edit().
    int save_finish_op = finish_op;
    finish_op = FALSE;

    retval = edit(NUL, FALSE, (linenr_T)1);

    finish_op = save_finish_op;

    /*
     * In Visual block mode, handle copying the new text to all lines of the
     * block.
     * Don't repeat the insert when Insert mode ended with CTRL-C.
     */
    if (oap->block_mode && oap->start.lnum != oap->end.lnum && !got_int)
    {
	// Auto-indenting may have changed the indent.  If the cursor was past
	// the indent, exclude that indent change from the inserted text.
	firstline = ml_get(oap->start.lnum);
	if (bd.textcol > (colnr_T)pre_indent)
	{
	    long new_indent = (long)getwhitecols(firstline);

	    pre_textlen += new_indent - pre_indent;
	    bd.textcol += new_indent - pre_indent;
	}

	ins_len = (long)STRLEN(firstline) - pre_textlen;
	if (ins_len > 0)
	{
	    // Subsequent calls to ml_get() flush the firstline data - take a
	    // copy of the inserted text.
	    if ((ins_text = alloc(ins_len + 1)) != NULL)
	    {
		vim_strncpy(ins_text, firstline + bd.textcol, (size_t)ins_len);
		for (linenr = oap->start.lnum + 1; linenr <= oap->end.lnum;
								     linenr++)
		{
		    block_prep(oap, &bd, linenr, TRUE);
		    if (!bd.is_short || virtual_op)
		    {
			pos_T vpos;

			// If the block starts in virtual space, count the
			// initial coladd offset as part of "startspaces"
			if (bd.is_short)
			{
			    vpos.lnum = linenr;
			    (void)getvpos(&vpos, oap->start_vcol);
			}
			else
			    vpos.coladd = 0;
			oldp = ml_get(linenr);
			newp = alloc(STRLEN(oldp) + vpos.coladd + ins_len + 1);
			if (newp == NULL)
			    continue;
			// copy up to block start
			mch_memmove(newp, oldp, (size_t)bd.textcol);
			offset = bd.textcol;
			vim_memset(newp + offset, ' ', (size_t)vpos.coladd);
			offset += vpos.coladd;
			mch_memmove(newp + offset, ins_text, (size_t)ins_len);
			offset += ins_len;
			oldp += bd.textcol;
			STRMOVE(newp + offset, oldp);
			ml_replace(linenr, newp, FALSE);
#ifdef FEAT_PROP_POPUP
			// Shift the properties for linenr as edit() would do.
			if (curbuf->b_has_textprop)
			    adjust_prop_columns(linenr, bd.textcol,
						     vpos.coladd + ins_len, 0);
#endif
		    }
		}
		check_cursor();

		changed_lines(oap->start.lnum + 1, 0, oap->end.lnum + 1, 0L);
	    }
	    vim_free(ins_text);
	}
    }
    auto_format(FALSE, TRUE);

    return retval;
}

/*
 * When the cursor is on the NUL past the end of the line and it should not be
 * there move it left.
 */
    void
adjust_cursor_eol(void)
{
    unsigned int cur_ve_flags = get_ve_flags();

    int adj_cursor = (curwin->w_cursor.col > 0
				&& gchar_cursor() == NUL
				&& (cur_ve_flags & VE_ONEMORE) == 0
				&& !(restart_edit || (State & MODE_INSERT)));
    if (!adj_cursor)
	return;

    // Put the cursor on the last character in the line.
    dec_cursor();

    if (cur_ve_flags == VE_ALL)
    {
	colnr_T	    scol, ecol;

	// Coladd is set to the width of the last character.
	getvcol(curwin, &curwin->w_cursor, &scol, NULL, &ecol);
	curwin->w_cursor.coladd = ecol - scol + 1;
    }
}

/*
 * If "process" is TRUE and the line begins with a comment leader (possibly
 * after some white space), return a pointer to the text after it. Put a boolean
 * value indicating whether the line ends with an unclosed comment in
 * "is_comment".
 * line - line to be processed,
 * process - if FALSE, will only check whether the line ends with an unclosed
 *	     comment,
 * include_space - whether to also skip space following the comment leader,
 * is_comment - will indicate whether the current line ends with an unclosed
 *		comment.
 */
    char_u *
skip_comment(
    char_u   *line,
    int      process,
    int	     include_space,
    int      *is_comment)
{
    char_u *comment_flags = NULL;
    int    lead_len;
    int    leader_offset = get_last_leader_offset(line, &comment_flags);

    *is_comment = FALSE;
    if (leader_offset != -1)
    {
	// Let's check whether the line ends with an unclosed comment.
	// If the last comment leader has COM_END in flags, there's no comment.
	while (*comment_flags)
	{
	    if (*comment_flags == COM_END
		    || *comment_flags == ':')
		break;
	    ++comment_flags;
	}
	if (*comment_flags != COM_END)
	    *is_comment = TRUE;
    }

    if (process == FALSE)
	return line;

    lead_len = get_leader_len(line, &comment_flags, FALSE, include_space);

    if (lead_len == 0)
	return line;

    // Find:
    // - COM_END,
    // - colon,
    // whichever comes first.
    while (*comment_flags)
    {
	if (*comment_flags == COM_END
		|| *comment_flags == ':')
	    break;
	++comment_flags;
    }

    // If we found a colon, it means that we are not processing a line
    // starting with a closing part of a three-part comment. That's good,
    // because we don't want to remove those as this would be annoying.
    if (*comment_flags == ':' || *comment_flags == NUL)
	line += lead_len;

    return line;
}

/*
 * Join 'count' lines (minimal 2) at the cursor position.
 * When "save_undo" is TRUE save lines for undo first.
 * Set "use_formatoptions" to FALSE when e.g. processing backspace and comment
 * leaders should not be removed.
 * When setmark is TRUE, sets the '[ and '] mark, else, the caller is expected
 * to set those marks.
 *
 * return FAIL for failure, OK otherwise
 */
    int
do_join(
    long    count,
    int	    insert_space,
    int	    save_undo,
    int	    use_formatoptions UNUSED,
    int	    setmark)
{
    char_u	*curr = NULL;
    char_u      *curr_start = NULL;
    char_u	*cend;
    char_u	*newp;
    size_t	newp_len;
    char_u	*spaces;	// number of spaces inserted before a line
    int		endcurr1 = NUL;
    int		endcurr2 = NUL;
    int		currsize = 0;	// size of the current line
    int		sumsize = 0;	// size of the long new line
    linenr_T	t;
    colnr_T	col = 0;
    int		ret = OK;
    int		*comments = NULL;
    int		remove_comments = (use_formatoptions == TRUE)
				  && has_format_option(FO_REMOVE_COMS);
    int		prev_was_comment;
#ifdef FEAT_PROP_POPUP
    int		propcount = 0;	// number of props over all joined lines
    int		props_remaining;
#endif

    if (save_undo && u_save((linenr_T)(curwin->w_cursor.lnum - 1),
			    (linenr_T)(curwin->w_cursor.lnum + count)) == FAIL)
	return FAIL;

    // Allocate an array to store the number of spaces inserted before each
    // line.  We will use it to pre-compute the length of the new line and the
    // proper placement of each original line in the new one.
    spaces = lalloc_clear(count, TRUE);
    if (spaces == NULL)
	return FAIL;
    if (remove_comments)
    {
	comments = lalloc_clear(count * sizeof(int), TRUE);
	if (comments == NULL)
	{
	    vim_free(spaces);
	    return FAIL;
	}
    }

    /*
     * Don't move anything yet, just compute the final line length
     * and setup the array of space strings lengths
     * This loops forward over the joined lines.
     */
    for (t = 0; t < count; ++t)
    {
	curr = curr_start = ml_get((linenr_T)(curwin->w_cursor.lnum + t));
#ifdef FEAT_PROP_POPUP
	propcount += count_props((linenr_T) (curwin->w_cursor.lnum + t),
							t > 0, t + 1 == count);
#endif
	if (t == 0 && setmark && (cmdmod.cmod_flags & CMOD_LOCKMARKS) == 0)
	{
	    // Set the '[ mark.
	    curwin->w_buffer->b_op_start.lnum = curwin->w_cursor.lnum;
	    curwin->w_buffer->b_op_start.col  = (colnr_T)STRLEN(curr);
	}
	if (remove_comments)
	{
	    // We don't want to remove the comment leader if the
	    // previous line is not a comment.
	    if (t > 0 && prev_was_comment)
	    {

		char_u *new_curr = skip_comment(curr, TRUE, insert_space,
							   &prev_was_comment);
		comments[t] = (int)(new_curr - curr);
		curr = new_curr;
	    }
	    else
		curr = skip_comment(curr, FALSE, insert_space,
							   &prev_was_comment);
	}

	if (insert_space && t > 0)
	{
	    curr = skipwhite(curr);
	    if (*curr != NUL && *curr != ')'
		    && sumsize != 0 && endcurr1 != TAB
		    && (!has_format_option(FO_MBYTE_JOIN)
			|| (mb_ptr2char(curr) < 0x100 && endcurr1 < 0x100))
		    && (!has_format_option(FO_MBYTE_JOIN2)
			|| (mb_ptr2char(curr) < 0x100
			    && !(enc_utf8 && utf_eat_space(endcurr1)))
			|| (endcurr1 < 0x100
			    && !(enc_utf8 && utf_eat_space(mb_ptr2char(curr)))))
	       )
	    {
		// don't add a space if the line is ending in a space
		if (endcurr1 == ' ')
		    endcurr1 = endcurr2;
		else
		    ++spaces[t];
		// extra space when 'joinspaces' set and line ends in '.'
		if (       p_js
			&& (endcurr1 == '.'
			    || (vim_strchr(p_cpo, CPO_JOINSP) == NULL
				&& (endcurr1 == '?' || endcurr1 == '!'))))
		    ++spaces[t];
	    }
	}
	currsize = (int)STRLEN(curr);
	sumsize += currsize + spaces[t];
	endcurr1 = endcurr2 = NUL;
	if (insert_space && currsize > 0)
	{
	    if (has_mbyte)
	    {
		cend = curr + currsize;
		MB_PTR_BACK(curr, cend);
		endcurr1 = (*mb_ptr2char)(cend);
		if (cend > curr)
		{
		    MB_PTR_BACK(curr, cend);
		    endcurr2 = (*mb_ptr2char)(cend);
		}
	    }
	    else
	    {
		endcurr1 = *(curr + currsize - 1);
		if (currsize > 1)
		    endcurr2 = *(curr + currsize - 2);
	    }
	}
	line_breakcheck();
	if (got_int)
	{
	    ret = FAIL;
	    goto theend;
	}
    }

    // store the column position before last line
    col = sumsize - currsize - spaces[count - 1];

    // allocate the space for the new line
    newp_len = sumsize + 1;
#ifdef FEAT_PROP_POPUP
    newp_len += propcount * sizeof(textprop_T);
#endif
    newp = alloc(newp_len);
    if (newp == NULL)
    {
	ret = FAIL;
	goto theend;
    }
    cend = newp + sumsize;
    *cend = 0;

    /*
     * Move affected lines to the new long one.
     * This loops backwards over the joined lines, including the original line.
     *
     * Move marks from each deleted line to the joined line, adjusting the
     * column.  This is not Vi compatible, but Vi deletes the marks, thus that
     * should not really be a problem.
     */
#ifdef FEAT_PROP_POPUP
    props_remaining = propcount;
#endif
    for (t = count - 1; ; --t)
    {
	int spaces_removed;

	cend -= currsize;
	mch_memmove(cend, curr, (size_t)currsize);

	if (spaces[t] > 0)
	{
	    cend -= spaces[t];
	    vim_memset(cend, ' ', (size_t)(spaces[t]));
	}

	// If deleting more spaces than adding, the cursor moves no more than
	// what is added if it is inside these spaces.
	spaces_removed = (curr - curr_start) - spaces[t];

	mark_col_adjust(curwin->w_cursor.lnum + t, (colnr_T)0, -t,
			 (long)(cend - newp - spaces_removed), spaces_removed);
#ifdef FEAT_PROP_POPUP
	prepend_joined_props(newp + sumsize + 1, propcount, &props_remaining,
		curwin->w_cursor.lnum + t, t == count - 1,
		(long)(cend - newp), spaces_removed);
#endif
	if (t == 0)
	    break;
	curr = curr_start = ml_get((linenr_T)(curwin->w_cursor.lnum + t - 1));
	if (remove_comments)
	    curr += comments[t - 1];
	if (insert_space && t > 1)
	    curr = skipwhite(curr);
	currsize = (int)STRLEN(curr);
    }

    ml_replace_len(curwin->w_cursor.lnum, newp, (colnr_T)newp_len, TRUE, FALSE);

    if (setmark && (cmdmod.cmod_flags & CMOD_LOCKMARKS) == 0)
    {
	// Set the '] mark.
	curwin->w_buffer->b_op_end.lnum = curwin->w_cursor.lnum;
	curwin->w_buffer->b_op_end.col  = (colnr_T)sumsize;
    }

    // Only report the change in the first line here, del_lines() will report
    // the deleted line.
    changed_lines(curwin->w_cursor.lnum, currsize,
					       curwin->w_cursor.lnum + 1, 0L);
    /*
     * Delete following lines. To do this we move the cursor there
     * briefly, and then move it back. After del_lines() the cursor may
     * have moved up (last line deleted), so the current lnum is kept in t.
     */
    t = curwin->w_cursor.lnum;
    ++curwin->w_cursor.lnum;
    del_lines(count - 1, FALSE);
    curwin->w_cursor.lnum = t;

    /*
     * Set the cursor column:
     * Vi compatible: use the column of the first join
     * vim:	      use the column of the last join
     */
    curwin->w_cursor.col =
		    (vim_strchr(p_cpo, CPO_JOINCOL) != NULL ? currsize : col);
    check_cursor_col();

    curwin->w_cursor.coladd = 0;
    curwin->w_set_curswant = TRUE;

theend:
    vim_free(spaces);
    if (remove_comments)
	vim_free(comments);
    return ret;
}

#ifdef FEAT_LINEBREAK
/*
 * Reset 'linebreak' and take care of side effects.
 * Returns the previous value, to be passed to restore_lbr().
 */
    static int
reset_lbr(void)
{
    if (!curwin->w_p_lbr)
	return FALSE;
    // changing 'linebreak' may require w_virtcol to be updated
    curwin->w_p_lbr = FALSE;
    curwin->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL);
    return TRUE;
}

/*
 * Restore 'linebreak' and take care of side effects.
 */
    static void
restore_lbr(int lbr_saved)
{
    if (curwin->w_p_lbr || !lbr_saved)
	return;

    // changing 'linebreak' may require w_virtcol to be updated
    curwin->w_p_lbr = TRUE;
    curwin->w_valid &= ~(VALID_WROW|VALID_WCOL|VALID_VIRTCOL);
}
#endif

/*
 * prepare a few things for block mode yank/delete/tilde
 *
 * for delete:
 * - textlen includes the first/last char to be (partly) deleted
 * - start/endspaces is the number of columns that are taken by the
 *   first/last deleted char minus the number of columns that have to be
 *   deleted.
 * for yank and tilde:
 * - textlen includes the first/last char to be wholly yanked
 * - start/endspaces is the number of columns of the first/last yanked char
 *   that are to be yanked.
 */
    void
block_prep(
    oparg_T		*oap,
    struct block_def	*bdp,
    linenr_T		lnum,
    int			is_del)
{
    int		incr = 0;
    char_u	*pend;
    char_u	*pstart;
    char_u	*line;
    char_u	*prev_pstart;
    char_u	*prev_pend;
    chartabsize_T cts;
#ifdef FEAT_LINEBREAK
		// Avoid a problem with unwanted linebreaks in block mode.
    int		lbr_saved = reset_lbr();
#endif

    bdp->startspaces = 0;
    bdp->endspaces = 0;
    bdp->textlen = 0;
    bdp->start_vcol = 0;
    bdp->end_vcol = 0;
    bdp->is_short = FALSE;
    bdp->is_oneChar = FALSE;
    bdp->pre_whitesp = 0;
    bdp->pre_whitesp_c = 0;
    bdp->end_char_vcols = 0;
    bdp->start_char_vcols = 0;

    line = ml_get(lnum);
    prev_pstart = line;
    init_chartabsize_arg(&cts, curwin, lnum, bdp->start_vcol, line, line);
    while (cts.cts_vcol < oap->start_vcol && *cts.cts_ptr != NUL)
    {
	// Count a tab for what it's worth (if list mode not on)
	incr = lbr_chartabsize(&cts);
	cts.cts_vcol += incr;
	if (VIM_ISWHITE(*cts.cts_ptr))
	{
	    bdp->pre_whitesp += incr;
	    bdp->pre_whitesp_c++;
	}
	else
	{
	    bdp->pre_whitesp = 0;
	    bdp->pre_whitesp_c = 0;
	}
	prev_pstart = cts.cts_ptr;
	MB_PTR_ADV(cts.cts_ptr);
    }
    bdp->start_vcol = cts.cts_vcol;
    pstart = cts.cts_ptr;
    clear_chartabsize_arg(&cts);

    bdp->start_char_vcols = incr;
    if (bdp->start_vcol < oap->start_vcol)	// line too short
    {
	bdp->end_vcol = bdp->start_vcol;
	bdp->is_short = TRUE;
	if (!is_del || oap->op_type == OP_APPEND)
	    bdp->endspaces = oap->end_vcol - oap->start_vcol + 1;
    }
    else
    {
	// notice: this converts partly selected Multibyte characters to
	// spaces, too.
	bdp->startspaces = bdp->start_vcol - oap->start_vcol;
	if (is_del && bdp->startspaces)
	    bdp->startspaces = bdp->start_char_vcols - bdp->startspaces;
	pend = pstart;
	bdp->end_vcol = bdp->start_vcol;
	if (bdp->end_vcol > oap->end_vcol)	// it's all in one character
	{
	    bdp->is_oneChar = TRUE;
	    if (oap->op_type == OP_INSERT)
		bdp->endspaces = bdp->start_char_vcols - bdp->startspaces;
	    else if (oap->op_type == OP_APPEND)
	    {
		bdp->startspaces += oap->end_vcol - oap->start_vcol + 1;
		bdp->endspaces = bdp->start_char_vcols - bdp->startspaces;
	    }
	    else
	    {
		bdp->startspaces = oap->end_vcol - oap->start_vcol + 1;
		if (is_del && oap->op_type != OP_LSHIFT)
		{
		    // just putting the sum of those two into
		    // bdp->startspaces doesn't work for Visual replace,
		    // so we have to split the tab in two
		    bdp->startspaces = bdp->start_char_vcols
					- (bdp->start_vcol - oap->start_vcol);
		    bdp->endspaces = bdp->end_vcol - oap->end_vcol - 1;
		}
	    }
	}
	else
	{
	    init_chartabsize_arg(&cts, curwin, lnum, bdp->end_vcol,
								  line, pend);
	    prev_pend = pend;
	    while (cts.cts_vcol <= oap->end_vcol && *cts.cts_ptr != NUL)
	    {
		// count a tab for what it's worth (if list mode not on)
		prev_pend = cts.cts_ptr;
		incr = lbr_chartabsize_adv(&cts);
		cts.cts_vcol += incr;
	    }
	    bdp->end_vcol = cts.cts_vcol;
	    pend = cts.cts_ptr;
	    clear_chartabsize_arg(&cts);

	    if (bdp->end_vcol <= oap->end_vcol
		    && (!is_del
			|| oap->op_type == OP_APPEND
			|| oap->op_type == OP_REPLACE)) // line too short
	    {
		bdp->is_short = TRUE;
		// Alternative: include spaces to fill up the block.
		// Disadvantage: can lead to trailing spaces when the line is
		// short where the text is put
		// if (!is_del || oap->op_type == OP_APPEND)
		if (oap->op_type == OP_APPEND || virtual_op)
		    bdp->endspaces = oap->end_vcol - bdp->end_vcol
							     + oap->inclusive;
		else
		    bdp->endspaces = 0; // replace doesn't add characters
	    }
	    else if (bdp->end_vcol > oap->end_vcol)
	    {
		bdp->endspaces = bdp->end_vcol - oap->end_vcol - 1;
		if (!is_del && bdp->endspaces)
		{
		    bdp->endspaces = incr - bdp->endspaces;
		    if (pend != pstart)
			pend = prev_pend;
		}
	    }
	}
	bdp->end_char_vcols = incr;
	if (is_del && bdp->startspaces)
	    pstart = prev_pstart;
	bdp->textlen = (int)(pend - pstart);
    }
    bdp->textcol = (colnr_T) (pstart - line);
    bdp->textstart = pstart;
#ifdef FEAT_LINEBREAK
    restore_lbr(lbr_saved);
#endif
}

/*
 * Handle the add/subtract operator.
 */
    void
op_addsub(
    oparg_T	*oap,
    linenr_T	Prenum1,	    // Amount of add/subtract
    int		g_cmd)		    // was g<c-a>/g<c-x>
{
    pos_T		pos;
    struct block_def	bd;
    int			change_cnt = 0;
    linenr_T		amount = Prenum1;

   // do_addsub() might trigger re-evaluation of 'foldexpr' halfway, when the
   // buffer is not completely updated yet. Postpone updating folds until before
   // the call to changed_lines().
#ifdef FEAT_FOLDING
   disable_fold_update++;
#endif

    if (!VIsual_active)
    {
	pos = curwin->w_cursor;
	if (u_save_cursor() == FAIL)
	{
#ifdef FEAT_FOLDING
	    disable_fold_update--;
#endif
	    return;
	}
	change_cnt = do_addsub(oap->op_type, &pos, 0, amount);
#ifdef FEAT_FOLDING
	disable_fold_update--;
#endif
	if (change_cnt)
	    changed_lines(pos.lnum, 0, pos.lnum + 1, 0L);
    }
    else
    {
	int	one_change;
	int	length;
	pos_T	startpos;

	if (u_save((linenr_T)(oap->start.lnum - 1),
					(linenr_T)(oap->end.lnum + 1)) == FAIL)
	{
#ifdef FEAT_FOLDING
	    disable_fold_update--;
#endif
	    return;
	}

	pos = oap->start;
	for (; pos.lnum <= oap->end.lnum; ++pos.lnum)
	{
	    if (oap->block_mode)		    // Visual block mode
	    {
		block_prep(oap, &bd, pos.lnum, FALSE);
		pos.col = bd.textcol;
		length = bd.textlen;
	    }
	    else if (oap->motion_type == MLINE)
	    {
		curwin->w_cursor.col = 0;
		pos.col = 0;
		length = (colnr_T)STRLEN(ml_get(pos.lnum));
	    }
	    else // oap->motion_type == MCHAR
	    {
		if (pos.lnum == oap->start.lnum && !oap->inclusive)
		    dec(&(oap->end));
		length = (colnr_T)STRLEN(ml_get(pos.lnum));
		pos.col = 0;
		if (pos.lnum == oap->start.lnum)
		{
		    pos.col += oap->start.col;
		    length -= oap->start.col;
		}
		if (pos.lnum == oap->end.lnum)
		{
		    length = (int)STRLEN(ml_get(oap->end.lnum));
		    if (oap->end.col >= length)
			oap->end.col = length - 1;
		    length = oap->end.col - pos.col + 1;
		}
	    }
	    one_change = do_addsub(oap->op_type, &pos, length, amount);
	    if (one_change)
	    {
		// Remember the start position of the first change.
		if (change_cnt == 0)
		    startpos = curbuf->b_op_start;
		++change_cnt;
	    }

#ifdef FEAT_NETBEANS_INTG
	    if (netbeans_active() && one_change)
	    {
		char_u *ptr;

		netbeans_removed(curbuf, pos.lnum, pos.col, (long)length);
		ptr = ml_get_buf(curbuf, pos.lnum, FALSE);
		netbeans_inserted(curbuf, pos.lnum, pos.col,
						&ptr[pos.col], length);
	    }
#endif
	    if (g_cmd && one_change)
		amount += Prenum1;
	}

#ifdef FEAT_FOLDING
	disable_fold_update--;
#endif
	if (change_cnt)
	    changed_lines(oap->start.lnum, 0, oap->end.lnum + 1, 0L);

	if (!change_cnt && oap->is_VIsual)
	    // No change: need to remove the Visual selection
	    redraw_curbuf_later(UPD_INVERTED);

	// Set '[ mark if something changed. Keep the last end
	// position from do_addsub().
	if (change_cnt > 0 && (cmdmod.cmod_flags & CMOD_LOCKMARKS) == 0)
	    curbuf->b_op_start = startpos;

	if (change_cnt > p_report)
	    smsg(NGETTEXT("%d line changed", "%d lines changed",
						      change_cnt), change_cnt);
    }
}

/*
 * Add or subtract 'Prenum1' from a number in a line
 * op_type is OP_NR_ADD or OP_NR_SUB
 *
 * Returns TRUE if some character was changed.
 */
    static int
do_addsub(
    int		op_type,
    pos_T	*pos,
    int		length,
    linenr_T	Prenum1)
{
    int		col;
    char_u	*buf1;
    char_u	buf2[NUMBUFLEN];
    int		pre;		// 'X'/'x': hex; '0': octal; 'B'/'b': bin
    static int	hexupper = FALSE;	// 0xABC
    uvarnumber_T	n;
    uvarnumber_T	oldn;
    char_u	*ptr;
    int		c;
    int		todel;
    int		do_hex;
    int		do_oct;
    int		do_bin;
    int		do_alpha;
    int		do_unsigned;
    int		firstdigit;
    int		subtract;
    int		negative = FALSE;
    int		was_positive = TRUE;
    int		visual = VIsual_active;
    int		did_change = FALSE;
    pos_T	save_cursor = curwin->w_cursor;
    int		maxlen = 0;
    pos_T	startpos;
    pos_T	endpos;
    colnr_T	save_coladd = 0;

    do_hex = (vim_strchr(curbuf->b_p_nf, 'x') != NULL);	// "heX"
    do_oct = (vim_strchr(curbuf->b_p_nf, 'o') != NULL);	// "Octal"
    do_bin = (vim_strchr(curbuf->b_p_nf, 'b') != NULL);	// "Bin"
    do_alpha = (vim_strchr(curbuf->b_p_nf, 'p') != NULL);	// "alPha"
    do_unsigned = (vim_strchr(curbuf->b_p_nf, 'u') != NULL);	// "Unsigned"

    if (virtual_active())
    {
	save_coladd = pos->coladd;
	pos->coladd = 0;
    }

    curwin->w_cursor = *pos;
    ptr = ml_get(pos->lnum);
    col = pos->col;

    if (*ptr == NUL || col + !!save_coladd >= (int)STRLEN(ptr))
	goto theend;

    /*
     * First check if we are on a hexadecimal number, after the "0x".
     */
    if (!VIsual_active)
    {
	if (do_bin)
	    while (col > 0 && vim_isbdigit(ptr[col]))
	    {
		--col;
		if (has_mbyte)
		    col -= (*mb_head_off)(ptr, ptr + col);
	    }

	if (do_hex)
	    while (col > 0 && vim_isxdigit(ptr[col]))
	    {
		--col;
		if (has_mbyte)
		    col -= (*mb_head_off)(ptr, ptr + col);
	    }

	if (       do_bin
		&& do_hex
		&& ! ((col > 0
		    && (ptr[col] == 'X'
			|| ptr[col] == 'x')
		    && ptr[col - 1] == '0'
		    && (!has_mbyte ||
			!(*mb_head_off)(ptr, ptr + col - 1))
		    && vim_isxdigit(ptr[col + 1]))))
	{

	    // In case of binary/hexadecimal pattern overlap match, rescan

	    col = pos->col;

	    while (col > 0 && vim_isdigit(ptr[col]))
	    {
		col--;
		if (has_mbyte)
		    col -= (*mb_head_off)(ptr, ptr + col);
	    }
	}

	if ((       do_hex
		&& col > 0
		&& (ptr[col] == 'X'
		    || ptr[col] == 'x')
		&& ptr[col - 1] == '0'
		&& (!has_mbyte ||
		    !(*mb_head_off)(ptr, ptr + col - 1))
		&& vim_isxdigit(ptr[col + 1])) ||
	    (       do_bin
		&& col > 0
		&& (ptr[col] == 'B'
		    || ptr[col] == 'b')
		&& ptr[col - 1] == '0'
		&& (!has_mbyte ||
		    !(*mb_head_off)(ptr, ptr + col - 1))
		&& vim_isbdigit(ptr[col + 1])))
	{
	    // Found hexadecimal or binary number, move to its start.
	    --col;
	    if (has_mbyte)
		col -= (*mb_head_off)(ptr, ptr + col);
	}
	else
	{
	    /*
	     * Search forward and then backward to find the start of number.
	     */
	    col = pos->col;

	    while (ptr[col] != NUL
		    && !vim_isdigit(ptr[col])
		    && !(do_alpha && ASCII_ISALPHA(ptr[col])))
		col += mb_ptr2len(ptr + col);

	    while (col > 0
		    && vim_isdigit(ptr[col - 1])
		    && !(do_alpha && ASCII_ISALPHA(ptr[col])))
	    {
		--col;
		if (has_mbyte)
		    col -= (*mb_head_off)(ptr, ptr + col);
	    }
	}
    }

    if (visual)
    {
	while (ptr[col] != NUL && length > 0
		&& !vim_isdigit(ptr[col])
		&& !(do_alpha && ASCII_ISALPHA(ptr[col])))
	{
	    int mb_len = mb_ptr2len(ptr + col);

	    col += mb_len;
	    length -= mb_len;
	}

	if (length == 0)
	    goto theend;

	if (col > pos->col && ptr[col - 1] == '-'
		&& (!has_mbyte || !(*mb_head_off)(ptr, ptr + col - 1))
		&& !do_unsigned)
	{
	    negative = TRUE;
	    was_positive = FALSE;
	}
    }

    /*
     * If a number was found, and saving for undo works, replace the number.
     */
    firstdigit = ptr[col];
    if (!VIM_ISDIGIT(firstdigit) && !(do_alpha && ASCII_ISALPHA(firstdigit)))
    {
	beep_flush();
	goto theend;
    }

    if (do_alpha && ASCII_ISALPHA(firstdigit))
    {
	// decrement or increment alphabetic character
	if (op_type == OP_NR_SUB)
	{
	    if (CharOrd(firstdigit) < Prenum1)
	    {
		if (SAFE_isupper(firstdigit))
		    firstdigit = 'A';
		else
		    firstdigit = 'a';
	    }
	    else
		firstdigit -= Prenum1;
	}
	else
	{
	    if (26 - CharOrd(firstdigit) - 1 < Prenum1)
	    {
		if (SAFE_isupper(firstdigit))
		    firstdigit = 'Z';
		else
		    firstdigit = 'z';
	    }
	    else
		firstdigit += Prenum1;
	}
	curwin->w_cursor.col = col;
	if (!did_change)
	    startpos = curwin->w_cursor;
	did_change = TRUE;
	(void)del_char(FALSE);
	ins_char(firstdigit);
	endpos = curwin->w_cursor;
	curwin->w_cursor.col = col;
    }
    else
    {
	pos_T	save_pos;
	int	i;

	if (col > 0 && ptr[col - 1] == '-'
		&& (!has_mbyte ||
		    !(*mb_head_off)(ptr, ptr + col - 1))
		&& !visual
		&& !do_unsigned)
	{
	    // negative number
	    --col;
	    negative = TRUE;
	}
	// get the number value (unsigned)
	if (visual && VIsual_mode != 'V')
	    maxlen = (curbuf->b_visual.vi_curswant == MAXCOL
		    ? (int)STRLEN(ptr) - col
		    : length);

	int overflow = FALSE;
	vim_str2nr(ptr + col, &pre, &length,
		0 + (do_bin ? STR2NR_BIN : 0)
		    + (do_oct ? STR2NR_OCT : 0)
		    + (do_hex ? STR2NR_HEX : 0),
		NULL, &n, maxlen, FALSE, &overflow);

	// ignore leading '-' for hex and octal and bin numbers
	if (pre && negative)
	{
	    ++col;
	    --length;
	    negative = FALSE;
	}
	// add or subtract
	subtract = FALSE;
	if (op_type == OP_NR_SUB)
	    subtract ^= TRUE;
	if (negative)
	    subtract ^= TRUE;

	oldn = n;
	if (!overflow)  // if number is too big don't add/subtract
	{
	    if (subtract)
		n -= (uvarnumber_T)Prenum1;
	    else
		n += (uvarnumber_T)Prenum1;
	}

	// handle wraparound for decimal numbers
	if (!pre)
	{
	    if (subtract)
	    {
		if (n > oldn)
		{
		    n = 1 + (n ^ (uvarnumber_T)-1);
		    negative ^= TRUE;
		}
	    }
	    else
	    {
		// add
		if (n < oldn)
		{
		    n = (n ^ (uvarnumber_T)-1);
		    negative ^= TRUE;
		}
	    }
	    if (n == 0)
		negative = FALSE;
	}

	if (do_unsigned && negative)
	{
	    if (subtract)
		// sticking at zero.
		n = (uvarnumber_T)0;
	    else
		// sticking at 2^64 - 1.
		n = (uvarnumber_T)(-1);
	    negative = FALSE;
	}

	if (visual && !was_positive && !negative && col > 0)
	{
	    // need to remove the '-'
	    col--;
	    length++;
	}

	/*
	 * Delete the old number.
	 */
	curwin->w_cursor.col = col;
	if (!did_change)
	    startpos = curwin->w_cursor;
	did_change = TRUE;
	todel = length;
	c = gchar_cursor();
	/*
	 * Don't include the '-' in the length, only the length of the
	 * part after it is kept the same.
	 */
	if (c == '-')
	    --length;

	save_pos = curwin->w_cursor;
	for (i = 0; i < todel; ++i)
	{
	    if (c < 0x100 && SAFE_isalpha(c))
	    {
		if (SAFE_isupper(c))
		    hexupper = TRUE;
		else
		    hexupper = FALSE;
	    }
	    inc_cursor();
	    c = gchar_cursor();
	}
	curwin->w_cursor = save_pos;

	/*
	 * Prepare the leading characters in buf1[].
	 * When there are many leading zeros it could be very long.
	 * Allocate a bit too much.
	 */
	buf1 = alloc(length + NUMBUFLEN);
	if (buf1 == NULL)
	    goto theend;
	ptr = buf1;
	if (negative && (!visual || was_positive))
	    *ptr++ = '-';
	if (pre)
	{
	    *ptr++ = '0';
	    --length;
	}
	if (pre == 'b' || pre == 'B' ||
	    pre == 'x' || pre == 'X')
	{
	    *ptr++ = pre;
	    --length;
	}

	/*
	 * Put the number characters in buf2[].
	 */
	if (pre == 'b' || pre == 'B')
	{
	    int bit = 0;
	    int bits = sizeof(uvarnumber_T) * 8;

	    // leading zeros
	    for (bit = bits; bit > 0; bit--)
		if ((n >> (bit - 1)) & 0x1) break;

	    for (i = 0; bit > 0 && i < (NUMBUFLEN - 1); bit--)
		buf2[i++] = ((n >> (bit - 1)) & 0x1) ? '1' : '0';

	    buf2[i] = '\0';
	}
	else if (pre == 0)
	    vim_snprintf((char *)buf2, NUMBUFLEN, "%llu", n);
	else if (pre == '0')
	    vim_snprintf((char *)buf2, NUMBUFLEN, "%llo", n);
	else if (pre && hexupper)
	    vim_snprintf((char *)buf2, NUMBUFLEN, "%llX", n);
	else
	    vim_snprintf((char *)buf2, NUMBUFLEN, "%llx", n);
	length -= (int)STRLEN(buf2);

	/*
	 * Adjust number of zeros to the new number of digits, so the
	 * total length of the number remains the same.
	 * Don't do this when
	 * the result may look like an octal number.
	 */
	if (firstdigit == '0' && !(do_oct && pre == 0))
	    while (length-- > 0)
		*ptr++ = '0';
	*ptr = NUL;

	STRCAT(buf1, buf2);

	// Insert just after the first character to be removed, so that any
	// text properties will be adjusted.  Then delete the old number
	// afterwards.
	save_pos = curwin->w_cursor;
	if (todel > 0)
	    inc_cursor();
	ins_str(buf1);		// insert the new number
	vim_free(buf1);

	// del_char() will also mark line needing displaying
	if (todel > 0)
	{
	    int bytes_after = (int)STRLEN(ml_get_curline())
							- curwin->w_cursor.col;

	    // Delete the one character before the insert.
	    curwin->w_cursor = save_pos;
	    (void)del_char(FALSE);
	    curwin->w_cursor.col = (colnr_T)(STRLEN(ml_get_curline())
								- bytes_after);
	    --todel;
	}
	while (todel-- > 0)
	    (void)del_char(FALSE);

	endpos = curwin->w_cursor;
	if (did_change && curwin->w_cursor.col)
	    --curwin->w_cursor.col;
    }

    if (did_change && (cmdmod.cmod_flags & CMOD_LOCKMARKS) == 0)
    {
	// set the '[ and '] marks
	curbuf->b_op_start = startpos;
	curbuf->b_op_end = endpos;
	if (curbuf->b_op_end.col > 0)
	    --curbuf->b_op_end.col;
    }

theend:
    if (visual)
	curwin->w_cursor = save_cursor;
    else if (did_change)
	curwin->w_set_curswant = TRUE;
    else if (virtual_active())
	curwin->w_cursor.coladd = save_coladd;

    return did_change;
}

    void
clear_oparg(oparg_T *oap)
{
    CLEAR_POINTER(oap);
}

/*
 *  Count the number of bytes, characters and "words" in a line.
 *
 *  "Words" are counted by looking for boundaries between non-space and
 *  space characters.  (it seems to produce results that match 'wc'.)
 *
 *  Return value is byte count; word count for the line is added to "*wc".
 *  Char count is added to "*cc".
 *
 *  The function will only examine the first "limit" characters in the
 *  line, stopping if it encounters an end-of-line (NUL byte).  In that
 *  case, eol_size will be added to the character count to account for
 *  the size of the EOL character.
 */
    static varnumber_T
line_count_info(
    char_u	*line,
    varnumber_T	*wc,
    varnumber_T	*cc,
    varnumber_T	limit,
    int		eol_size)
{
    varnumber_T	i;
    varnumber_T	words = 0;
    varnumber_T	chars = 0;
    int		is_word = 0;

    for (i = 0; i < limit && line[i] != NUL; )
    {
	if (is_word)
	{
	    if (vim_isspace(line[i]))
	    {
		words++;
		is_word = 0;
	    }
	}
	else if (!vim_isspace(line[i]))
	    is_word = 1;
	++chars;
	i += (*mb_ptr2len)(line + i);
    }

    if (is_word)
	words++;
    *wc += words;

    // Add eol_size if the end of line was reached before hitting limit.
    if (i < limit && line[i] == NUL)
    {
	i += eol_size;
	chars += eol_size;
    }
    *cc += chars;
    return i;
}

/*
 * Give some info about the position of the cursor (for "g CTRL-G").
 * In Visual mode, give some info about the selected region.  (In this case,
 * the *_count_cursor variables store running totals for the selection.)
 * When "dict" is not NULL store the info there instead of showing it.
 */
    void
cursor_pos_info(dict_T *dict)
{
    char_u	*p;
    char_u	buf1[50];
    char_u	buf2[40];
    linenr_T	lnum;
    varnumber_T	byte_count = 0;
    varnumber_T	bom_count  = 0;
    varnumber_T	byte_count_cursor = 0;
    varnumber_T	char_count = 0;
    varnumber_T	char_count_cursor = 0;
    varnumber_T	word_count = 0;
    varnumber_T	word_count_cursor = 0;
    int		eol_size;
    varnumber_T	last_check = 100000L;
    long	line_count_selected = 0;
    pos_T	min_pos, max_pos;
    oparg_T	oparg;
    struct block_def	bd;

    /*
     * Compute the length of the file in characters.
     */
    if (curbuf->b_ml.ml_flags & ML_EMPTY)
    {
	if (dict == NULL)
	{
	    msg(_(no_lines_msg));
	    return;
	}
    }
    else
    {
	if (get_fileformat(curbuf) == EOL_DOS)
	    eol_size = 2;
	else
	    eol_size = 1;

	if (VIsual_active)
	{
	    if (LT_POS(VIsual, curwin->w_cursor))
	    {
		min_pos = VIsual;
		max_pos = curwin->w_cursor;
	    }
	    else
	    {
		min_pos = curwin->w_cursor;
		max_pos = VIsual;
	    }
	    if (*p_sel == 'e' && max_pos.col > 0)
		--max_pos.col;

	    if (VIsual_mode == Ctrl_V)
	    {
#ifdef FEAT_LINEBREAK
		char_u * saved_sbr = p_sbr;
		char_u * saved_w_sbr = curwin->w_p_sbr;

		// Make 'sbr' empty for a moment to get the correct size.
		p_sbr = empty_option;
		curwin->w_p_sbr = empty_option;
#endif
		oparg.is_VIsual = 1;
		oparg.block_mode = TRUE;
		oparg.op_type = OP_NOP;
		getvcols(curwin, &min_pos, &max_pos,
					  &oparg.start_vcol, &oparg.end_vcol);
#ifdef FEAT_LINEBREAK
		p_sbr = saved_sbr;
		curwin->w_p_sbr = saved_w_sbr;
#endif
		if (curwin->w_curswant == MAXCOL)
		    oparg.end_vcol = MAXCOL;
		// Swap the start, end vcol if needed
		if (oparg.end_vcol < oparg.start_vcol)
		{
		    oparg.end_vcol += oparg.start_vcol;
		    oparg.start_vcol = oparg.end_vcol - oparg.start_vcol;
		    oparg.end_vcol -= oparg.start_vcol;
		}
	    }
	    line_count_selected = max_pos.lnum - min_pos.lnum + 1;
	}

	for (lnum = 1; lnum <= curbuf->b_ml.ml_line_count; ++lnum)
	{
	    // Check for a CTRL-C every 100000 characters.
	    if (byte_count > last_check)
	    {
		ui_breakcheck();
		if (got_int)
		    return;
		last_check = byte_count + 100000L;
	    }

	    // Do extra processing for VIsual mode.
	    if (VIsual_active
		    && lnum >= min_pos.lnum && lnum <= max_pos.lnum)
	    {
		char_u	    *s = NULL;
		long	    len = 0L;

		switch (VIsual_mode)
		{
		    case Ctrl_V:
			virtual_op = virtual_active();
			block_prep(&oparg, &bd, lnum, 0);
			virtual_op = MAYBE;
			s = bd.textstart;
			len = (long)bd.textlen;
			break;
		    case 'V':
			s = ml_get(lnum);
			len = MAXCOL;
			break;
		    case 'v':
			{
			    colnr_T start_col = (lnum == min_pos.lnum)
							   ? min_pos.col : 0;
			    colnr_T end_col = (lnum == max_pos.lnum)
				      ? max_pos.col - start_col + 1 : MAXCOL;

			    s = ml_get(lnum) + start_col;
			    len = end_col;
			}
			break;
		}
		if (s != NULL)
		{
		    byte_count_cursor += line_count_info(s, &word_count_cursor,
					   &char_count_cursor, len, eol_size);
		    if (lnum == curbuf->b_ml.ml_line_count
			    && !curbuf->b_p_eol
			    && (curbuf->b_p_bin || !curbuf->b_p_fixeol)
			    && (long)STRLEN(s) < len)
			byte_count_cursor -= eol_size;
		}
	    }
	    else
	    {
		// In non-visual mode, check for the line the cursor is on
		if (lnum == curwin->w_cursor.lnum)
		{
		    word_count_cursor += word_count;
		    char_count_cursor += char_count;
		    byte_count_cursor = byte_count +
			line_count_info(ml_get(lnum),
				&word_count_cursor, &char_count_cursor,
				(varnumber_T)(curwin->w_cursor.col + 1),
				eol_size);
		}
	    }
	    // Add to the running totals
	    byte_count += line_count_info(ml_get(lnum), &word_count,
					 &char_count, (varnumber_T)MAXCOL,
					 eol_size);
	}

	// Correction for when last line doesn't have an EOL.
	if (!curbuf->b_p_eol && (curbuf->b_p_bin || !curbuf->b_p_fixeol))
	    byte_count -= eol_size;

	if (dict == NULL)
	{
	    if (VIsual_active)
	    {
		if (VIsual_mode == Ctrl_V && curwin->w_curswant < MAXCOL)
		{
		    getvcols(curwin, &min_pos, &max_pos, &min_pos.col,
								    &max_pos.col);
		    vim_snprintf((char *)buf1, sizeof(buf1), _("%ld Cols; "),
			    (long)(oparg.end_vcol - oparg.start_vcol + 1));
		}
		else
		    buf1[0] = NUL;

		if (char_count_cursor == byte_count_cursor
						    && char_count == byte_count)
		    vim_snprintf((char *)IObuff, IOSIZE,
			    _("Selected %s%ld of %ld Lines; %lld of %lld Words; %lld of %lld Bytes"),
			    buf1, line_count_selected,
			    (long)curbuf->b_ml.ml_line_count,
			    word_count_cursor,
			    word_count,
			    byte_count_cursor,
			    byte_count);
		else
		    vim_snprintf((char *)IObuff, IOSIZE,
			    _("Selected %s%ld of %ld Lines; %lld of %lld Words; %lld of %lld Chars; %lld of %lld Bytes"),
			    buf1, line_count_selected,
			    (long)curbuf->b_ml.ml_line_count,
			    word_count_cursor,
			    word_count,
			    char_count_cursor,
			    char_count,
			    byte_count_cursor,
			    byte_count);
	    }
	    else
	    {
		p = ml_get_curline();
		validate_virtcol();
		col_print(buf1, sizeof(buf1), (int)curwin->w_cursor.col + 1,
			(int)curwin->w_virtcol + 1);
		col_print(buf2, sizeof(buf2), (int)STRLEN(p),
							   linetabsize_str(p));

		if (char_count_cursor == byte_count_cursor
			&& char_count == byte_count)
		    vim_snprintf((char *)IObuff, IOSIZE,
			_("Col %s of %s; Line %ld of %ld; Word %lld of %lld; Byte %lld of %lld"),
			(char *)buf1, (char *)buf2,
			(long)curwin->w_cursor.lnum,
			(long)curbuf->b_ml.ml_line_count,
			word_count_cursor, word_count,
			byte_count_cursor, byte_count);
		else
		    vim_snprintf((char *)IObuff, IOSIZE,
			_("Col %s of %s; Line %ld of %ld; Word %lld of %lld; Char %lld of %lld; Byte %lld of %lld"),
			(char *)buf1, (char *)buf2,
			(long)curwin->w_cursor.lnum,
			(long)curbuf->b_ml.ml_line_count,
			word_count_cursor, word_count,
			char_count_cursor, char_count,
			byte_count_cursor, byte_count);
	    }
	}

	bom_count = bomb_size();
	if (dict == NULL && bom_count > 0)
	{
	    size_t len = STRLEN(IObuff);

	    vim_snprintf((char *)IObuff + len, IOSIZE - len,
				 _("(+%lld for BOM)"), bom_count);
	}
	if (dict == NULL)
	{
	    // Don't shorten this message, the user asked for it.
	    p = p_shm;
	    p_shm = (char_u *)"";
	    msg((char *)IObuff);
	    p_shm = p;
	}
    }
#if defined(FEAT_EVAL)
    if (dict != NULL)
    {
	dict_add_number(dict, "words", word_count);
	dict_add_number(dict, "chars", char_count);
	dict_add_number(dict, "bytes", byte_count + bom_count);
	dict_add_number(dict, VIsual_active ? "visual_bytes" : "cursor_bytes",
		byte_count_cursor);
	dict_add_number(dict, VIsual_active ? "visual_chars" : "cursor_chars",
		char_count_cursor);
	dict_add_number(dict, VIsual_active ? "visual_words" : "cursor_words",
		word_count_cursor);
    }
#endif
}

/*
 * Handle indent and format operators and visual mode ":".
 */
    static void
op_colon(oparg_T *oap)
{
    stuffcharReadbuff(':');
    if (oap->is_VIsual)
	stuffReadbuff((char_u *)"'<,'>");
    else
    {
	// Make the range look nice, so it can be repeated.
	if (oap->start.lnum == curwin->w_cursor.lnum)
	    stuffcharReadbuff('.');
	else
	    stuffnumReadbuff((long)oap->start.lnum);

#ifdef FEAT_FOLDING
	// When using !! on a closed fold the range ".!" works best to operate
	// on, it will be made the whole closed fold later.
	linenr_T endOfStartFold = oap->start.lnum;
	(void)hasFolding(oap->start.lnum, NULL, &endOfStartFold);
#endif
	if (oap->end.lnum != oap->start.lnum
#ifdef FEAT_FOLDING
		&& oap->end.lnum != endOfStartFold
#endif
	   )
	{
	    // Make it a range with the end line.
	    stuffcharReadbuff(',');
	    if (oap->end.lnum == curwin->w_cursor.lnum)
		stuffcharReadbuff('.');
	    else if (oap->end.lnum == curbuf->b_ml.ml_line_count)
		stuffcharReadbuff('$');
	    else if (oap->start.lnum == curwin->w_cursor.lnum
#ifdef FEAT_FOLDING
		    // do not use ".+number" for a closed fold, it would count
		    // folded lines twice
		    && !hasFolding(oap->end.lnum, NULL, NULL)
#endif
		    )
	    {
		stuffReadbuff((char_u *)".+");
		stuffnumReadbuff((long)oap->line_count - 1);
	    }
	    else
		stuffnumReadbuff((long)oap->end.lnum);
	}
    }
    if (oap->op_type != OP_COLON)
	stuffReadbuff((char_u *)"!");
    if (oap->op_type == OP_INDENT)
    {
	if (*get_equalprg() == NUL)
	    stuffReadbuff((char_u *)"indent");
	else
	    stuffReadbuff(get_equalprg());
	stuffReadbuff((char_u *)"\n");
    }
    else if (oap->op_type == OP_FORMAT)
    {
	if (*curbuf->b_p_fp != NUL)
	    stuffReadbuff(curbuf->b_p_fp);
	else if (*p_fp != NUL)
	    stuffReadbuff(p_fp);
	else
	    stuffReadbuff((char_u *)"fmt");
	stuffReadbuff((char_u *)"\n']");
    }

    // do_cmdline() does the rest
}

// callback function for 'operatorfunc'
static callback_T opfunc_cb;

/*
 * Process the 'operatorfunc' option value.
 * Returns OK or FAIL.
 */
    char *
did_set_operatorfunc(optset_T *args UNUSED)
{
    if (option_set_callback_func(p_opfunc, &opfunc_cb) == FAIL)
	return e_invalid_argument;

    return NULL;
}

#if defined(EXITFREE) || defined(PROTO)
    void
free_operatorfunc_option(void)
{
# ifdef FEAT_EVAL
    free_callback(&opfunc_cb);
# endif
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Mark the global 'operatorfunc' callback with "copyID" so that it is not
 * garbage collected.
 */
    int
set_ref_in_opfunc(int copyID UNUSED)
{
    int abort = FALSE;

    abort = set_ref_in_callback(&opfunc_cb, copyID);

    return abort;
}
#endif

/*
 * Handle the "g@" operator: call 'operatorfunc'.
 */
    static void
op_function(oparg_T *oap UNUSED)
{
#ifdef FEAT_EVAL
    typval_T	argv[2];
    pos_T	orig_start = curbuf->b_op_start;
    pos_T	orig_end = curbuf->b_op_end;
    typval_T	rettv;

    if (*p_opfunc == NUL)
	emsg(_(e_operatorfunc_is_empty));
    else
    {
	// Set '[ and '] marks to text to be operated on.
	curbuf->b_op_start = oap->start;
	curbuf->b_op_end = oap->end;
	if (oap->motion_type != MLINE && !oap->inclusive)
	    // Exclude the end position.
	    decl(&curbuf->b_op_end);

	argv[0].v_type = VAR_STRING;
	if (oap->block_mode)
	    argv[0].vval.v_string = (char_u *)"block";
	else if (oap->motion_type == MLINE)
	    argv[0].vval.v_string = (char_u *)"line";
	else
	    argv[0].vval.v_string = (char_u *)"char";
	argv[1].v_type = VAR_UNKNOWN;

	// Reset virtual_op so that 'virtualedit' can be changed in the
	// function.
	int save_virtual_op = virtual_op;
	virtual_op = MAYBE;

	// Reset finish_op so that mode() returns the right value.
	int save_finish_op = finish_op;
	finish_op = FALSE;

	if (call_callback(&opfunc_cb, 0, &rettv, 1, argv) != FAIL)
	    clear_tv(&rettv);

	virtual_op = save_virtual_op;
	finish_op = save_finish_op;
	if (cmdmod.cmod_flags & CMOD_LOCKMARKS)
	{
	    curbuf->b_op_start = orig_start;
	    curbuf->b_op_end = orig_end;
	}
    }
#else
    emsg(_(e_eval_feature_not_available));
#endif
}

/*
 * Calculate start/end virtual columns for operating in block mode.
 */
    static void
get_op_vcol(
    oparg_T	*oap,
    colnr_T	redo_VIsual_vcol,
    int		initial)    // when TRUE adjust position for 'selectmode'
{
    colnr_T	    start, end;

    if (VIsual_mode != Ctrl_V
	    || (!initial && oap->end.col < curwin->w_width))
	return;

    oap->block_mode = TRUE;

    // prevent from moving onto a trail byte
    if (has_mbyte)
	mb_adjustpos(curwin->w_buffer, &oap->end);

    getvvcol(curwin, &(oap->start), &oap->start_vcol, NULL, &oap->end_vcol);

    if (!redo_VIsual_busy)
    {
	getvvcol(curwin, &(oap->end), &start, NULL, &end);

	if (start < oap->start_vcol)
	    oap->start_vcol = start;
	if (end > oap->end_vcol)
	{
	    if (initial && *p_sel == 'e' && start >= 1
				    && start - 1 >= oap->end_vcol)
		oap->end_vcol = start - 1;
	    else
		oap->end_vcol = end;
	}
    }

    // if '$' was used, get oap->end_vcol from longest line
    if (curwin->w_curswant == MAXCOL)
    {
	curwin->w_cursor.col = MAXCOL;
	oap->end_vcol = 0;
	for (curwin->w_cursor.lnum = oap->start.lnum;
		curwin->w_cursor.lnum <= oap->end.lnum;
					++curwin->w_cursor.lnum)
	{
	    getvvcol(curwin, &curwin->w_cursor, NULL, NULL, &end);
	    if (end > oap->end_vcol)
		oap->end_vcol = end;
	}
    }
    else if (redo_VIsual_busy)
	oap->end_vcol = oap->start_vcol + redo_VIsual_vcol - 1;
    // Correct oap->end.col and oap->start.col to be the
    // upper-left and lower-right corner of the block area.
    //
    // (Actually, this does convert column positions into character
    // positions)
    curwin->w_cursor.lnum = oap->end.lnum;
    coladvance(oap->end_vcol);
    oap->end = curwin->w_cursor;

    curwin->w_cursor = oap->start;
    coladvance(oap->start_vcol);
    oap->start = curwin->w_cursor;
}

// Information for redoing the previous Visual selection.
typedef struct {
    int		rv_mode;	// 'v', 'V', or Ctrl-V
    linenr_T	rv_line_count;	// number of lines
    colnr_T	rv_vcol;	// number of cols or end column
    long	rv_count;	// count for Visual operator
    int		rv_arg;		// extra argument
} redo_VIsual_T;

    static int
is_ex_cmdchar(cmdarg_T *cap)
{
    return cap->cmdchar == ':'
	|| cap->cmdchar == K_COMMAND
	|| cap->cmdchar == K_SCRIPT_COMMAND;
}

/*
 * Handle an operator after Visual mode or when the movement is finished.
 * "gui_yank" is true when yanking text for the clipboard.
 */
    void
do_pending_operator(cmdarg_T *cap, int old_col, int gui_yank)
{
    oparg_T	*oap = cap->oap;
    pos_T	old_cursor;
    int		empty_region_error;
    int		restart_edit_save;
#ifdef FEAT_LINEBREAK
    int		lbr_saved = curwin->w_p_lbr;
#endif

    // The visual area is remembered for redo
    static redo_VIsual_T   redo_VIsual = {NUL, 0, 0, 0,0};

    int		    include_line_break = FALSE;

#if defined(FEAT_CLIPBOARD)
    // Yank the visual area into the GUI selection register before we operate
    // on it and lose it forever.
    // Don't do it if a specific register was specified, so that ""x"*P works.
    // This could call do_pending_operator() recursively, but that's OK
    // because gui_yank will be TRUE for the nested call.
    if ((clip_star.available || clip_plus.available)
	    && oap->op_type != OP_NOP
	    && !gui_yank
	    && VIsual_active
	    && !redo_VIsual_busy
	    && oap->regname == 0)
	clip_auto_select();
#endif
    old_cursor = curwin->w_cursor;

    // If an operation is pending, handle it...
    if ((finish_op || VIsual_active) && oap->op_type != OP_NOP)
    {
	// Yank can be redone when 'y' is in 'cpoptions', but not when yanking
	// for the clipboard.
	int	redo_yank = vim_strchr(p_cpo, CPO_YANK) != NULL && !gui_yank;

#ifdef FEAT_LINEBREAK
	// Avoid a problem with unwanted linebreaks in block mode.
	(void)reset_lbr();
#endif
	oap->is_VIsual = VIsual_active;
	if (oap->motion_force == 'V')
	    oap->motion_type = MLINE;
	else if (oap->motion_force == 'v')
	{
	    // If the motion was linewise, "inclusive" will not have been set.
	    // Use "exclusive" to be consistent.  Makes "dvj" work nice.
	    if (oap->motion_type == MLINE)
		oap->inclusive = FALSE;
	    // If the motion already was characterwise, toggle "inclusive"
	    else if (oap->motion_type == MCHAR)
		oap->inclusive = !oap->inclusive;
	    oap->motion_type = MCHAR;
	}
	else if (oap->motion_force == Ctrl_V)
	{
	    // Change line- or characterwise motion into Visual block mode.
	    if (!VIsual_active)
	    {
		VIsual_active = TRUE;
		VIsual = oap->start;
	    }
	    VIsual_mode = Ctrl_V;
	    VIsual_select = FALSE;
	    VIsual_reselect = FALSE;
	}

	// Only redo yank when 'y' flag is in 'cpoptions'.
	// Never redo "zf" (define fold).
	if ((redo_yank || oap->op_type != OP_YANK)
		&& ((!VIsual_active || oap->motion_force)
		    // Also redo Operator-pending Visual mode mappings
		    || (VIsual_active
			    && is_ex_cmdchar(cap) && oap->op_type != OP_COLON))
		&& cap->cmdchar != 'D'
#ifdef FEAT_FOLDING
		&& oap->op_type != OP_FOLD
		&& oap->op_type != OP_FOLDOPEN
		&& oap->op_type != OP_FOLDOPENREC
		&& oap->op_type != OP_FOLDCLOSE
		&& oap->op_type != OP_FOLDCLOSEREC
		&& oap->op_type != OP_FOLDDEL
		&& oap->op_type != OP_FOLDDELREC
#endif
		)
	{
	    prep_redo(oap->regname, cap->count0,
		    get_op_char(oap->op_type), get_extra_op_char(oap->op_type),
		    oap->motion_force, cap->cmdchar, cap->nchar);
	    if (cap->cmdchar == '/' || cap->cmdchar == '?') // was a search
	    {
		// If 'cpoptions' does not contain 'r', insert the search
		// pattern to really repeat the same command.
		if (vim_strchr(p_cpo, CPO_REDO) == NULL)
		    AppendToRedobuffLit(cap->searchbuf, -1);
		AppendToRedobuff(NL_STR);
	    }
	    else if (is_ex_cmdchar(cap))
	    {
		// do_cmdline() has stored the first typed line in
		// "repeat_cmdline".  When several lines are typed repeating
		// won't be possible.
		if (repeat_cmdline == NULL)
		    ResetRedobuff();
		else
		{
		    if (cap->cmdchar == ':')
			AppendToRedobuffLit(repeat_cmdline, -1);
		    else
			AppendToRedobuffSpec(repeat_cmdline);
		    AppendToRedobuff(NL_STR);
		    VIM_CLEAR(repeat_cmdline);
		}
	    }
	}

	if (redo_VIsual_busy)
	{
	    // Redo of an operation on a Visual area. Use the same size from
	    // redo_VIsual.rv_line_count and redo_VIsual.rv_vcol.
	    oap->start = curwin->w_cursor;
	    curwin->w_cursor.lnum += redo_VIsual.rv_line_count - 1;
	    if (curwin->w_cursor.lnum > curbuf->b_ml.ml_line_count)
		curwin->w_cursor.lnum = curbuf->b_ml.ml_line_count;
	    VIsual_mode = redo_VIsual.rv_mode;
	    if (redo_VIsual.rv_vcol == MAXCOL || VIsual_mode == 'v')
	    {
		if (VIsual_mode == 'v')
		{
		    if (redo_VIsual.rv_line_count <= 1)
		    {
			validate_virtcol();
			curwin->w_curswant =
				     curwin->w_virtcol + redo_VIsual.rv_vcol - 1;
		    }
		    else
			curwin->w_curswant = redo_VIsual.rv_vcol;
		}
		else
		{
		    curwin->w_curswant = MAXCOL;
		}
		coladvance(curwin->w_curswant);
	    }
	    cap->count0 = redo_VIsual.rv_count;
	    if (redo_VIsual.rv_count != 0)
		cap->count1 = redo_VIsual.rv_count;
	    else
		cap->count1 = 1;
	}
	else if (VIsual_active)
	{
	    if (!gui_yank)
	    {
		// Save the current VIsual area for '< and '> marks, and "gv"
		curbuf->b_visual.vi_start = VIsual;
		curbuf->b_visual.vi_end = curwin->w_cursor;
		curbuf->b_visual.vi_mode = VIsual_mode;
		restore_visual_mode();
		curbuf->b_visual.vi_curswant = curwin->w_curswant;
#ifdef FEAT_EVAL
		curbuf->b_visual_mode_eval = VIsual_mode;
#endif
	    }

	    // In Select mode, a linewise selection is operated upon like a
	    // characterwise selection.
	    // Special case: gH<Del> deletes the last line.
	    if (VIsual_select && VIsual_mode == 'V'
					    && cap->oap->op_type != OP_DELETE)
	    {
		if (LT_POS(VIsual, curwin->w_cursor))
		{
		    VIsual.col = 0;
		    curwin->w_cursor.col =
			       (colnr_T)STRLEN(ml_get(curwin->w_cursor.lnum));
		}
		else
		{
		    curwin->w_cursor.col = 0;
		    VIsual.col = (colnr_T)STRLEN(ml_get(VIsual.lnum));
		}
		VIsual_mode = 'v';
	    }
	    // If 'selection' is "exclusive", backup one character for
	    // charwise selections.
	    else if (VIsual_mode == 'v')
		include_line_break = unadjust_for_sel();

	    oap->start = VIsual;
	    if (VIsual_mode == 'V')
	    {
		oap->start.col = 0;
		oap->start.coladd = 0;
	    }
	}

	// Set oap->start to the first position of the operated text, oap->end
	// to the end of the operated text.  w_cursor is equal to oap->start.
	if (LT_POS(oap->start, curwin->w_cursor))
	{
#ifdef FEAT_FOLDING
	    // Include folded lines completely.
	    if (!VIsual_active)
	    {
		if (hasFolding(oap->start.lnum, &oap->start.lnum, NULL))
		    oap->start.col = 0;
		if ((curwin->w_cursor.col > 0 || oap->inclusive
						  || oap->motion_type == MLINE)
			&& hasFolding(curwin->w_cursor.lnum, NULL,
						      &curwin->w_cursor.lnum))
		    curwin->w_cursor.col = (colnr_T)STRLEN(ml_get_curline());
	    }
#endif
	    oap->end = curwin->w_cursor;
	    curwin->w_cursor = oap->start;

	    // w_virtcol may have been updated; if the cursor goes back to its
	    // previous position w_virtcol becomes invalid and isn't updated
	    // automatically.
	    curwin->w_valid &= ~VALID_VIRTCOL;
	}
	else
	{
#ifdef FEAT_FOLDING
	    // Include folded lines completely.
	    if (!VIsual_active && oap->motion_type == MLINE)
	    {
		if (hasFolding(curwin->w_cursor.lnum, &curwin->w_cursor.lnum,
									NULL))
		    curwin->w_cursor.col = 0;
		if (hasFolding(oap->start.lnum, NULL, &oap->start.lnum))
		    oap->start.col = (colnr_T)STRLEN(ml_get(oap->start.lnum));
	    }
#endif
	    oap->end = oap->start;
	    oap->start = curwin->w_cursor;
	}

	// Just in case lines were deleted that make the position invalid.
	check_pos(curwin->w_buffer, &oap->end);
	oap->line_count = oap->end.lnum - oap->start.lnum + 1;

	// Set "virtual_op" before resetting VIsual_active.
	virtual_op = virtual_active();

	if (VIsual_active || redo_VIsual_busy)
	{
	    get_op_vcol(oap, redo_VIsual.rv_vcol, TRUE);

	    if (!redo_VIsual_busy && !gui_yank)
	    {
		// Prepare to reselect and redo Visual: this is based on the
		// size of the Visual text
		resel_VIsual_mode = VIsual_mode;
		if (curwin->w_curswant == MAXCOL)
		    resel_VIsual_vcol = MAXCOL;
		else
		{
		    if (VIsual_mode != Ctrl_V)
			getvvcol(curwin, &(oap->end),
						  NULL, NULL, &oap->end_vcol);
		    if (VIsual_mode == Ctrl_V || oap->line_count <= 1)
		    {
			if (VIsual_mode != Ctrl_V)
			    getvvcol(curwin, &(oap->start),
						&oap->start_vcol, NULL, NULL);
			resel_VIsual_vcol = oap->end_vcol - oap->start_vcol + 1;
		    }
		    else
			resel_VIsual_vcol = oap->end_vcol;
		}
		resel_VIsual_line_count = oap->line_count;
	    }

	    // can't redo yank (unless 'y' is in 'cpoptions') and ":"
	    if ((redo_yank || oap->op_type != OP_YANK)
		    && oap->op_type != OP_COLON
#ifdef FEAT_FOLDING
		    && oap->op_type != OP_FOLD
		    && oap->op_type != OP_FOLDOPEN
		    && oap->op_type != OP_FOLDOPENREC
		    && oap->op_type != OP_FOLDCLOSE
		    && oap->op_type != OP_FOLDCLOSEREC
		    && oap->op_type != OP_FOLDDEL
		    && oap->op_type != OP_FOLDDELREC
#endif
		    && oap->motion_force == NUL
		    )
	    {
		// Prepare for redoing.  Only use the nchar field for "r",
		// otherwise it might be the second char of the operator.
		if (cap->cmdchar == 'g' && (cap->nchar == 'n'
							|| cap->nchar == 'N'))
		    prep_redo(oap->regname, cap->count0,
			    get_op_char(oap->op_type),
			    get_extra_op_char(oap->op_type),
			    oap->motion_force, cap->cmdchar, cap->nchar);
		else if (!is_ex_cmdchar(cap))
		{
		    int opchar = get_op_char(oap->op_type);
		    int extra_opchar = get_extra_op_char(oap->op_type);
		    int nchar = oap->op_type == OP_REPLACE ? cap->nchar : NUL;

		    // reverse what nv_replace() did
		    if (nchar == REPLACE_CR_NCHAR)
			nchar = CAR;
		    else if (nchar == REPLACE_NL_NCHAR)
			nchar = NL;

		    if (opchar == 'g' && extra_opchar == '@')
			// also repeat the count for 'operatorfunc'
			prep_redo_num2(oap->regname, 0L, NUL, 'v',
				     cap->count0, opchar, extra_opchar, nchar);
		    else
			prep_redo(oap->regname, 0L, NUL, 'v',
						  opchar, extra_opchar, nchar);
		}
		if (!redo_VIsual_busy)
		{
		    redo_VIsual.rv_mode = resel_VIsual_mode;
		    redo_VIsual.rv_vcol = resel_VIsual_vcol;
		    redo_VIsual.rv_line_count = resel_VIsual_line_count;
		    redo_VIsual.rv_count = cap->count0;
		    redo_VIsual.rv_arg = cap->arg;
		}
	    }

	    // oap->inclusive defaults to TRUE.
	    // If oap->end is on a NUL (empty line) oap->inclusive becomes
	    // FALSE.  This makes "d}P" and "v}dP" work the same.
	    if (oap->motion_force == NUL || oap->motion_type == MLINE)
		oap->inclusive = TRUE;
	    if (VIsual_mode == 'V')
		oap->motion_type = MLINE;
	    else
	    {
		oap->motion_type = MCHAR;
		if (VIsual_mode != Ctrl_V && *ml_get_pos(&(oap->end)) == NUL
			&& (include_line_break || !virtual_op))
		{
		    oap->inclusive = FALSE;
		    // Try to include the newline, unless it's an operator
		    // that works on lines only.
		    if (*p_sel != 'o'
			    && !op_on_lines(oap->op_type)
			    && oap->end.lnum < curbuf->b_ml.ml_line_count)
		    {
			++oap->end.lnum;
			oap->end.col = 0;
			oap->end.coladd = 0;
			++oap->line_count;
		    }
		}
	    }

	    redo_VIsual_busy = FALSE;

	    // Switch Visual off now, so screen updating does
	    // not show inverted text when the screen is redrawn.
	    // With OP_YANK and sometimes with OP_COLON and OP_FILTER there is
	    // no screen redraw, so it is done here to remove the inverted
	    // part.
	    if (!gui_yank)
	    {
		VIsual_active = FALSE;
		setmouse();
		mouse_dragging = 0;
		may_clear_cmdline();
		if ((oap->op_type == OP_YANK
			    || oap->op_type == OP_COLON
			    || oap->op_type == OP_FUNCTION
			    || oap->op_type == OP_FILTER)
			&& oap->motion_force == NUL)
		{
#ifdef FEAT_LINEBREAK
		    // make sure redrawing is correct
		    restore_lbr(lbr_saved);
#endif
		    redraw_curbuf_later(UPD_INVERTED);
		}
	    }
	}

	// Include the trailing byte of a multi-byte char.
	if (has_mbyte && oap->inclusive)
	{
	    int		l;

	    l = (*mb_ptr2len)(ml_get_pos(&oap->end));
	    if (l > 1)
		oap->end.col += l - 1;
	}
	curwin->w_set_curswant = TRUE;

	// oap->empty is set when start and end are the same.  The inclusive
	// flag affects this too, unless yanking and the end is on a NUL.
	oap->empty = (oap->motion_type == MCHAR
		    && (!oap->inclusive
			|| (oap->op_type == OP_YANK
			    && gchar_pos(&oap->end) == NUL))
		    && EQUAL_POS(oap->start, oap->end)
		    && !(virtual_op && oap->start.coladd != oap->end.coladd));
	// For delete, change and yank, it's an error to operate on an
	// empty region, when 'E' included in 'cpoptions' (Vi compatible).
	empty_region_error = (oap->empty
				&& vim_strchr(p_cpo, CPO_EMPTYREGION) != NULL);

	// Force a redraw when operating on an empty Visual region, when
	// 'modifiable is off or creating a fold.
	if (oap->is_VIsual && (oap->empty || !curbuf->b_p_ma
#ifdef FEAT_FOLDING
		    || oap->op_type == OP_FOLD
#endif
		    ))
	{
#ifdef FEAT_LINEBREAK
	    restore_lbr(lbr_saved);
#endif
	    redraw_curbuf_later(UPD_INVERTED);
	}

	// If the end of an operator is in column one while oap->motion_type
	// is MCHAR and oap->inclusive is FALSE, we put op_end after the last
	// character in the previous line. If op_start is on or before the
	// first non-blank in the line, the operator becomes linewise
	// (strange, but that's the way vi does it).
	if (	   oap->motion_type == MCHAR
		&& oap->inclusive == FALSE
		&& !(cap->retval & CA_NO_ADJ_OP_END)
		&& oap->end.col == 0
		&& (!oap->is_VIsual || *p_sel == 'o')
		&& !oap->block_mode
		&& oap->line_count > 1)
	{
	    oap->end_adjusted = TRUE;	    // remember that we did this
	    --oap->line_count;
	    --oap->end.lnum;
	    if (inindent(0))
		oap->motion_type = MLINE;
	    else
	    {
		oap->end.col = (colnr_T)STRLEN(ml_get(oap->end.lnum));
		if (oap->end.col)
		{
		    --oap->end.col;
		    oap->inclusive = TRUE;
		}
	    }
	}
	else
	    oap->end_adjusted = FALSE;

	switch (oap->op_type)
	{
	case OP_LSHIFT:
	case OP_RSHIFT:
	    op_shift(oap, TRUE, oap->is_VIsual ? (int)cap->count1 : 1);
	    auto_format(FALSE, TRUE);
	    break;

	case OP_JOIN_NS:
	case OP_JOIN:
	    if (oap->line_count < 2)
		oap->line_count = 2;
	    if (curwin->w_cursor.lnum + oap->line_count - 1 >
						   curbuf->b_ml.ml_line_count)
		beep_flush();
	    else
	    {
		(void)do_join(oap->line_count, oap->op_type == OP_JOIN,
							    TRUE, TRUE, TRUE);
		auto_format(FALSE, TRUE);
	    }
	    break;

	case OP_DELETE:
	    VIsual_reselect = FALSE;	    // don't reselect now
	    if (empty_region_error)
	    {
		vim_beep(BO_OPER);
		CancelRedo();
	    }
	    else
	    {
		(void)op_delete(oap);
		// save cursor line for undo if it wasn't saved yet
		if (oap->motion_type == MLINE && has_format_option(FO_AUTO)
						      && u_save_cursor() == OK)
		    auto_format(FALSE, TRUE);
	    }
	    break;

	case OP_YANK:
	    if (empty_region_error)
	    {
		if (!gui_yank)
		{
		    vim_beep(BO_OPER);
		    CancelRedo();
		}
	    }
	    else
	    {
#ifdef FEAT_LINEBREAK
		restore_lbr(lbr_saved);
#endif
		oap->excl_tr_ws = cap->cmdchar == 'z';
		(void)op_yank(oap, FALSE, !gui_yank);
	    }
	    check_cursor_col();
	    break;

	case OP_CHANGE:
	    VIsual_reselect = FALSE;	    // don't reselect now
	    if (empty_region_error)
	    {
		vim_beep(BO_OPER);
		CancelRedo();
	    }
	    else
	    {
		// This is a new edit command, not a restart.  Need to
		// remember it to make 'insertmode' work with mappings for
		// Visual mode.  But do this only once and not when typed and
		// 'insertmode' isn't set.
		if (p_im || !KeyTyped)
		    restart_edit_save = restart_edit;
		else
		    restart_edit_save = 0;
		restart_edit = 0;
#ifdef FEAT_LINEBREAK
		// Restore linebreak, so that when the user edits it looks as
		// before.
		restore_lbr(lbr_saved);
#endif
		// trigger TextChangedI
		curbuf->b_last_changedtick_i = CHANGEDTICK(curbuf);

		if (op_change(oap))	// will call edit()
		    cap->retval |= CA_COMMAND_BUSY;
		if (restart_edit == 0)
		    restart_edit = restart_edit_save;
	    }
	    break;

	case OP_FILTER:
	    if (vim_strchr(p_cpo, CPO_FILTER) != NULL)
		AppendToRedobuff((char_u *)"!\r");  // use any last used !cmd
	    else
		bangredo = TRUE;    // do_bang() will put cmd in redo buffer
	    // FALLTHROUGH

	case OP_INDENT:
	case OP_COLON:

	    // If 'equalprg' is empty, do the indenting internally.
	    if (oap->op_type == OP_INDENT && *get_equalprg() == NUL)
	    {
		if (curbuf->b_p_lisp)
		{
#ifdef FEAT_EVAL
		    if (use_indentexpr_for_lisp())
			op_reindent(oap, get_expr_indent);
		    else
#endif
			op_reindent(oap, get_lisp_indent);
		    break;
		}
		op_reindent(oap,
#ifdef FEAT_EVAL
			*curbuf->b_p_inde != NUL ? get_expr_indent :
#endif
			    get_c_indent);
		break;
	    }

	    op_colon(oap);
	    break;

	case OP_TILDE:
	case OP_UPPER:
	case OP_LOWER:
	case OP_ROT13:
	    if (empty_region_error)
	    {
		vim_beep(BO_OPER);
		CancelRedo();
	    }
	    else
		op_tilde(oap);
	    check_cursor_col();
	    break;

	case OP_FORMAT:
#if defined(FEAT_EVAL)
	    if (*curbuf->b_p_fex != NUL)
		op_formatexpr(oap);	// use expression
	    else
#endif
	    {
		if (*p_fp != NUL || *curbuf->b_p_fp != NUL)
		    op_colon(oap);		// use external command
		else
		    op_format(oap, FALSE);	// use internal function
	    }
	    break;
	case OP_FORMAT2:
	    op_format(oap, TRUE);	// use internal function
	    break;

	case OP_FUNCTION:
	    {
		redo_VIsual_T   save_redo_VIsual = redo_VIsual;

#ifdef FEAT_LINEBREAK
		// Restore linebreak, so that when the user edits it looks as
		// before.
		restore_lbr(lbr_saved);
#endif
		// call 'operatorfunc'
		op_function(oap);

		// Restore the info for redoing Visual mode, the function may
		// invoke another operator and unintentionally change it.
		redo_VIsual = save_redo_VIsual;
		break;
	    }

	case OP_INSERT:
	case OP_APPEND:
	    VIsual_reselect = FALSE;	// don't reselect now
	    if (empty_region_error)
	    {
		vim_beep(BO_OPER);
		CancelRedo();
	    }
	    else
	    {
		// This is a new edit command, not a restart.  Need to
		// remember it to make 'insertmode' work with mappings for
		// Visual mode.  But do this only once.
		restart_edit_save = restart_edit;
		restart_edit = 0;
#ifdef FEAT_LINEBREAK
		// Restore linebreak, so that when the user edits it looks as
		// before.
		restore_lbr(lbr_saved);
#endif
		// trigger TextChangedI
		curbuf->b_last_changedtick_i = CHANGEDTICK(curbuf);

		op_insert(oap, cap->count1);
#ifdef FEAT_LINEBREAK
		// Reset linebreak, so that formatting works correctly.
		(void)reset_lbr();
#endif

		// TODO: when inserting in several lines, should format all
		// the lines.
		auto_format(FALSE, TRUE);

		if (restart_edit == 0)
		    restart_edit = restart_edit_save;
		else
		    cap->retval |= CA_COMMAND_BUSY;
	    }
	    break;

	case OP_REPLACE:
	    VIsual_reselect = FALSE;	// don't reselect now
	    if (empty_region_error)
	    {
		vim_beep(BO_OPER);
		CancelRedo();
	    }
	    else
	    {
#ifdef FEAT_LINEBREAK
		// Restore linebreak, so that when the user edits it looks as
		// before.
		restore_lbr(lbr_saved);
#endif
		op_replace(oap, cap->nchar);
	    }
	    break;

#ifdef FEAT_FOLDING
	case OP_FOLD:
	    VIsual_reselect = FALSE;	// don't reselect now
	    foldCreate(oap->start.lnum, oap->end.lnum);
	    break;

	case OP_FOLDOPEN:
	case OP_FOLDOPENREC:
	case OP_FOLDCLOSE:
	case OP_FOLDCLOSEREC:
	    VIsual_reselect = FALSE;	// don't reselect now
	    opFoldRange(oap->start.lnum, oap->end.lnum,
		    oap->op_type == OP_FOLDOPEN
					    || oap->op_type == OP_FOLDOPENREC,
		    oap->op_type == OP_FOLDOPENREC
					  || oap->op_type == OP_FOLDCLOSEREC,
					  oap->is_VIsual);
	    break;

	case OP_FOLDDEL:
	case OP_FOLDDELREC:
	    VIsual_reselect = FALSE;	// don't reselect now
	    deleteFold(oap->start.lnum, oap->end.lnum,
			       oap->op_type == OP_FOLDDELREC, oap->is_VIsual);
	    break;
#endif
	case OP_NR_ADD:
	case OP_NR_SUB:
	    if (empty_region_error)
	    {
		vim_beep(BO_OPER);
		CancelRedo();
	    }
	    else
	    {
		VIsual_active = TRUE;
#ifdef FEAT_LINEBREAK
		restore_lbr(lbr_saved);
#endif
		op_addsub(oap, cap->count1, redo_VIsual.rv_arg);
		VIsual_active = FALSE;
	    }
	    check_cursor_col();
	    break;
	default:
	    clearopbeep(oap);
	}
	virtual_op = MAYBE;
	if (!gui_yank)
	{
	    // if 'sol' not set, go back to old column for some commands
	    if (!p_sol && oap->motion_type == MLINE && !oap->end_adjusted
		    && (oap->op_type == OP_LSHIFT || oap->op_type == OP_RSHIFT
						|| oap->op_type == OP_DELETE))
	    {
#ifdef FEAT_LINEBREAK
		(void)reset_lbr();
#endif
		coladvance(curwin->w_curswant = old_col);
	    }
	}
	else
	{
	    curwin->w_cursor = old_cursor;
	}
	oap->block_mode = FALSE;
	clearop(oap);
	motion_force = NUL;
    }
#ifdef FEAT_LINEBREAK
    restore_lbr(lbr_saved);
#endif
}

// put byte 'c' at position 'lp', but
// verify, that the position to place
// is actually safe
    static void
pbyte(pos_T lp, int c)
{
    char_u *p = ml_get_buf(curbuf, lp.lnum, TRUE);
    int	len = curbuf->b_ml.ml_line_len;

    // safety check
    if (lp.col >= len)
	lp.col = (len > 1 ? len - 2 : 0);
    *(p + lp.col) = c;
}
