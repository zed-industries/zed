/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * diff.c: code for diff'ing two, three or four buffers.
 *
 * There are three ways to diff:
 * - Shell out to an external diff program, using files.
 * - Use the compiled-in xdiff library.
 * - Let 'diffexpr' do the work, using files.
 */

#include "vim.h"
#include "xdiff/xdiff.h"

#if defined(FEAT_DIFF) || defined(PROTO)

static int diff_busy = FALSE;	    // using diff structs, don't change them
static int diff_need_update = FALSE; // ex_diffupdate needs to be called

// flags obtained from the 'diffopt' option
#define DIFF_FILLER	0x001	// display filler lines
#define DIFF_IBLANK	0x002	// ignore empty lines
#define DIFF_ICASE	0x004	// ignore case
#define DIFF_IWHITE	0x008	// ignore change in white space
#define DIFF_IWHITEALL	0x010	// ignore all white space changes
#define DIFF_IWHITEEOL	0x020	// ignore change in white space at EOL
#define DIFF_HORIZONTAL	0x040	// horizontal splits
#define DIFF_VERTICAL	0x080	// vertical splits
#define DIFF_HIDDEN_OFF	0x100	// diffoff when hidden
#define DIFF_INTERNAL	0x200	// use internal xdiff algorithm
#define DIFF_CLOSE_OFF	0x400	// diffoff when closing window
#define DIFF_FOLLOWWRAP	0x800	// follow the wrap option
#define ALL_WHITE_DIFF (DIFF_IWHITE | DIFF_IWHITEALL | DIFF_IWHITEEOL)
static int	diff_flags = DIFF_INTERNAL | DIFF_FILLER | DIFF_CLOSE_OFF;

static long diff_algorithm = 0;

#define LBUFLEN 50		// length of line in diff file

static int diff_a_works = MAYBE; // TRUE when "diff -a" works, FALSE when it
				 // doesn't work, MAYBE when not checked yet
#if defined(MSWIN)
static int diff_bin_works = MAYBE; // TRUE when "diff --binary" works, FALSE
				   // when it doesn't work, MAYBE when not
				   // checked yet
#endif

// used for diff input
typedef struct {
    char_u	*din_fname;  // used for external diff
    mmfile_t	din_mmfile;  // used for internal diff
} diffin_T;

// used for diff result
typedef struct {
    char_u	*dout_fname;  // used for external diff
    garray_T	dout_ga;      // used for internal diff
} diffout_T;

// used for recording hunks from xdiff
typedef struct {
    linenr_T lnum_orig;
    long     count_orig;
    linenr_T lnum_new;
    long     count_new;
} diffhunk_T;

// two diff inputs and one result
typedef struct {
    diffin_T	dio_orig;     // original file input
    diffin_T	dio_new;      // new file input
    diffout_T	dio_diff;     // diff result
    int		dio_internal; // using internal diff
} diffio_T;

static int diff_buf_idx(buf_T *buf);
static int diff_buf_idx_tp(buf_T *buf, tabpage_T *tp);
static void diff_mark_adjust_tp(tabpage_T *tp, int idx, linenr_T line1, linenr_T line2, long amount, long amount_after);
static void diff_check_unchanged(tabpage_T *tp, diff_T *dp);
static int diff_check_sanity(tabpage_T *tp, diff_T *dp);
static int check_external_diff(diffio_T *diffio);
static int diff_file(diffio_T *diffio);
static int diff_equal_entry(diff_T *dp, int idx1, int idx2);
static int diff_cmp(char_u *s1, char_u *s2);
#ifdef FEAT_FOLDING
static void diff_fold_update(diff_T *dp, int skip_idx);
#endif
static void diff_read(int idx_orig, int idx_new, diffio_T *dio);
static void diff_copy_entry(diff_T *dprev, diff_T *dp, int idx_orig, int idx_new);
static diff_T *diff_alloc_new(tabpage_T *tp, diff_T *dprev, diff_T *dp);
static int parse_diff_ed(char_u *line, diffhunk_T *hunk);
static int parse_diff_unified(char_u *line, diffhunk_T *hunk);
static int xdiff_out(long start_a, long count_a, long start_b, long count_b, void *priv);

#define FOR_ALL_DIFFBLOCKS_IN_TAB(tp, dp) \
    for ((dp) = (tp)->tp_first_diff; (dp) != NULL; (dp) = (dp)->df_next)

/*
 * Called when deleting or unloading a buffer: No longer make a diff with it.
 */
    void
diff_buf_delete(buf_T *buf)
{
    int		i;
    tabpage_T	*tp;

    FOR_ALL_TABPAGES(tp)
    {
	i = diff_buf_idx_tp(buf, tp);
	if (i != DB_COUNT)
	{
	    tp->tp_diffbuf[i] = NULL;
	    tp->tp_diff_invalid = TRUE;
	    if (tp == curtab)
	    {
		// don't redraw right away, more might change or buffer state
		// is invalid right now
		need_diff_redraw = TRUE;
		redraw_later(UPD_VALID);
	    }
	}
    }
}

/*
 * Check if the current buffer should be added to or removed from the list of
 * diff buffers.
 */
    void
diff_buf_adjust(win_T *win)
{
    win_T	*wp;
    int		i;

    if (!win->w_p_diff)
    {
	// When there is no window showing a diff for this buffer, remove
	// it from the diffs.
	FOR_ALL_WINDOWS(wp)
	    if (wp->w_buffer == win->w_buffer && wp->w_p_diff)
		break;
	if (wp == NULL)
	{
	    i = diff_buf_idx(win->w_buffer);
	    if (i != DB_COUNT)
	    {
		curtab->tp_diffbuf[i] = NULL;
		curtab->tp_diff_invalid = TRUE;
		diff_redraw(TRUE);
	    }
	}
    }
    else
	diff_buf_add(win->w_buffer);
}

/*
 * Add a buffer to make diffs for.
 * Call this when a new buffer is being edited in the current window where
 * 'diff' is set.
 * Marks the current buffer as being part of the diff and requiring updating.
 * This must be done before any autocmd, because a command may use info
 * about the screen contents.
 */
    void
diff_buf_add(buf_T *buf)
{
    int		i;

    if (diff_buf_idx(buf) != DB_COUNT)
	return;		// It's already there.

    for (i = 0; i < DB_COUNT; ++i)
	if (curtab->tp_diffbuf[i] == NULL)
	{
	    curtab->tp_diffbuf[i] = buf;
	    curtab->tp_diff_invalid = TRUE;
	    diff_redraw(TRUE);
	    return;
	}

    semsg(_(e_cannot_diff_more_than_nr_buffers), DB_COUNT);
}

/*
 * Remove all buffers to make diffs for.
 */
    static void
diff_buf_clear(void)
{
    int		i;

    for (i = 0; i < DB_COUNT; ++i)
	if (curtab->tp_diffbuf[i] != NULL)
	{
	    curtab->tp_diffbuf[i] = NULL;
	    curtab->tp_diff_invalid = TRUE;
	    diff_redraw(TRUE);
	}
}

/*
 * Find buffer "buf" in the list of diff buffers for the current tab page.
 * Return its index or DB_COUNT if not found.
 */
    static int
diff_buf_idx(buf_T *buf)
{
    int		idx;

    for (idx = 0; idx < DB_COUNT; ++idx)
	if (curtab->tp_diffbuf[idx] == buf)
	    break;
    return idx;
}

/*
 * Find buffer "buf" in the list of diff buffers for tab page "tp".
 * Return its index or DB_COUNT if not found.
 */
    static int
diff_buf_idx_tp(buf_T *buf, tabpage_T *tp)
{
    int		idx;

    for (idx = 0; idx < DB_COUNT; ++idx)
	if (tp->tp_diffbuf[idx] == buf)
	    break;
    return idx;
}

/*
 * Mark the diff info involving buffer "buf" as invalid, it will be updated
 * when info is requested.
 */
    void
diff_invalidate(buf_T *buf)
{
    tabpage_T	*tp;
    int		i;

    FOR_ALL_TABPAGES(tp)
    {
	i = diff_buf_idx_tp(buf, tp);
	if (i != DB_COUNT)
	{
	    tp->tp_diff_invalid = TRUE;
	    if (tp == curtab)
		diff_redraw(TRUE);
	}
    }
}

/*
 * Called by mark_adjust(): update line numbers in "curbuf".
 */
    void
diff_mark_adjust(
    linenr_T	line1,
    linenr_T	line2,
    long	amount,
    long	amount_after)
{
    int		idx;
    tabpage_T	*tp;

    // Handle all tab pages that use the current buffer in a diff.
    FOR_ALL_TABPAGES(tp)
    {
	idx = diff_buf_idx_tp(curbuf, tp);
	if (idx != DB_COUNT)
	    diff_mark_adjust_tp(tp, idx, line1, line2, amount, amount_after);
    }
}

/*
 * Update line numbers in tab page "tp" for "curbuf" with index "idx".
 * This attempts to update the changes as much as possible:
 * When inserting/deleting lines outside of existing change blocks, create a
 * new change block and update the line numbers in following blocks.
 * When inserting/deleting lines in existing change blocks, update them.
 */
    static void
diff_mark_adjust_tp(
    tabpage_T	*tp,
    int		idx,
    linenr_T	line1,
    linenr_T	line2,
    long	amount,
    long	amount_after)
{
    diff_T	*dp;
    diff_T	*dprev;
    diff_T	*dnext;
    int		i;
    int		inserted, deleted;
    int		n, off;
    linenr_T	last;
    linenr_T	lnum_deleted = line1;	// lnum of remaining deletion
    int		check_unchanged;

    if (diff_internal())
    {
	// Will update diffs before redrawing.  Set _invalid to update the
	// diffs themselves, set _update to also update folds properly just
	// before redrawing.
	// Do update marks here, it is needed for :%diffput.
	tp->tp_diff_invalid = TRUE;
	tp->tp_diff_update = TRUE;
    }

    if (line2 == MAXLNUM)
    {
	// mark_adjust(99, MAXLNUM, 9, 0): insert lines
	inserted = amount;
	deleted = 0;
    }
    else if (amount_after > 0)
    {
	// mark_adjust(99, 98, MAXLNUM, 9): a change that inserts lines
	inserted = amount_after;
	deleted = 0;
    }
    else
    {
	// mark_adjust(98, 99, MAXLNUM, -2): delete lines
	inserted = 0;
	deleted = -amount_after;
    }

    dprev = NULL;
    dp = tp->tp_first_diff;
    for (;;)
    {
	// If the change is after the previous diff block and before the next
	// diff block, thus not touching an existing change, create a new diff
	// block.  Don't do this when ex_diffgetput() is busy.
	if ((dp == NULL || dp->df_lnum[idx] - 1 > line2
		    || (line2 == MAXLNUM && dp->df_lnum[idx] > line1))
		&& (dprev == NULL
		    || dprev->df_lnum[idx] + dprev->df_count[idx] < line1)
		&& !diff_busy)
	{
	    dnext = diff_alloc_new(tp, dprev, dp);
	    if (dnext == NULL)
		return;

	    dnext->df_lnum[idx] = line1;
	    dnext->df_count[idx] = inserted;
	    for (i = 0; i < DB_COUNT; ++i)
		if (tp->tp_diffbuf[i] != NULL && i != idx)
		{
		    if (dprev == NULL)
			dnext->df_lnum[i] = line1;
		    else
			dnext->df_lnum[i] = line1
			    + (dprev->df_lnum[i] + dprev->df_count[i])
			    - (dprev->df_lnum[idx] + dprev->df_count[idx]);
		    dnext->df_count[i] = deleted;
		}
	}

	// if at end of the list, quit
	if (dp == NULL)
	    break;

	/*
	 * Check for these situations:
	 *	  1  2	3
	 *	  1  2	3
	 * line1     2	3  4  5
	 *	     2	3  4  5
	 *	     2	3  4  5
	 * line2     2	3  4  5
	 *		3     5  6
	 *		3     5  6
	 */
	// compute last line of this change
	last = dp->df_lnum[idx] + dp->df_count[idx] - 1;

	// 1. change completely above line1: nothing to do
	if (last >= line1 - 1)
	{
	    // 6. change below line2: only adjust for amount_after; also when
	    // "deleted" became zero when deleted all lines between two diffs
	    if (dp->df_lnum[idx] - (deleted + inserted != 0) > line2)
	    {
		if (amount_after == 0)
		    break;	// nothing left to change
		dp->df_lnum[idx] += amount_after;
	    }
	    else
	    {
		check_unchanged = FALSE;

		// 2. 3. 4. 5.: inserted/deleted lines touching this diff.
		if (deleted > 0)
		{
		    off = 0;
		    if (dp->df_lnum[idx] >= line1)
		    {
			if (last <= line2)
			{
			    // 4. delete all lines of diff
			    if (dp->df_next != NULL
				    && dp->df_next->df_lnum[idx] - 1 <= line2)
			    {
				// delete continues in next diff, only do
				// lines until that one
				n = dp->df_next->df_lnum[idx] - lnum_deleted;
				deleted -= n;
				n -= dp->df_count[idx];
				lnum_deleted = dp->df_next->df_lnum[idx];
			    }
			    else
				n = deleted - dp->df_count[idx];
			    dp->df_count[idx] = 0;
			}
			else
			{
			    // 5. delete lines at or just before top of diff
			    off = dp->df_lnum[idx] - lnum_deleted;
			    n = off;
			    dp->df_count[idx] -= line2 - dp->df_lnum[idx] + 1;
			    check_unchanged = TRUE;
			}
			dp->df_lnum[idx] = line1;
		    }
		    else
		    {
			if (last < line2)
			{
			    // 2. delete at end of diff
			    dp->df_count[idx] -= last - lnum_deleted + 1;
			    if (dp->df_next != NULL
				    && dp->df_next->df_lnum[idx] - 1 <= line2)
			    {
				// delete continues in next diff, only do
				// lines until that one
				n = dp->df_next->df_lnum[idx] - 1 - last;
				deleted -= dp->df_next->df_lnum[idx]
							       - lnum_deleted;
				lnum_deleted = dp->df_next->df_lnum[idx];
			    }
			    else
				n = line2 - last;
			    check_unchanged = TRUE;
			}
			else
			{
			    // 3. delete lines inside the diff
			    n = 0;
			    dp->df_count[idx] -= deleted;
			}
		    }

		    for (i = 0; i < DB_COUNT; ++i)
			if (tp->tp_diffbuf[i] != NULL && i != idx)
			{
			    if (dp->df_lnum[i] > off)
				dp->df_lnum[i] -= off;
			    else
				dp->df_lnum[i] = 1;
			    dp->df_count[i] += n;
			}
		}
		else
		{
		    if (dp->df_lnum[idx] <= line1)
		    {
			// inserted lines somewhere in this diff
			dp->df_count[idx] += inserted;
			check_unchanged = TRUE;
		    }
		    else
			// inserted lines somewhere above this diff
			dp->df_lnum[idx] += inserted;
		}

		if (check_unchanged)
		    // Check if inserted lines are equal, may reduce the
		    // size of the diff.  TODO: also check for equal lines
		    // in the middle and perhaps split the block.
		    diff_check_unchanged(tp, dp);
	    }
	}

	// check if this block touches the previous one, may merge them.
	if (dprev != NULL && dprev->df_lnum[idx] + dprev->df_count[idx]
							  == dp->df_lnum[idx])
	{
	    for (i = 0; i < DB_COUNT; ++i)
		if (tp->tp_diffbuf[i] != NULL)
		    dprev->df_count[i] += dp->df_count[i];
	    dprev->df_next = dp->df_next;
	    vim_free(dp);
	    dp = dprev->df_next;
	}
	else
	{
	    // Advance to next entry.
	    dprev = dp;
	    dp = dp->df_next;
	}
    }

    dprev = NULL;
    dp = tp->tp_first_diff;
    while (dp != NULL)
    {
	// All counts are zero, remove this entry.
	for (i = 0; i < DB_COUNT; ++i)
	    if (tp->tp_diffbuf[i] != NULL && dp->df_count[i] != 0)
		break;
	if (i == DB_COUNT)
	{
	    dnext = dp->df_next;
	    vim_free(dp);
	    dp = dnext;
	    if (dprev == NULL)
		tp->tp_first_diff = dnext;
	    else
		dprev->df_next = dnext;
	}
	else
	{
	    // Advance to next entry.
	    dprev = dp;
	    dp = dp->df_next;
	}

    }

    if (tp == curtab)
    {
	// Don't redraw right away, this updates the diffs, which can be slow.
	need_diff_redraw = TRUE;

	// Need to recompute the scroll binding, may remove or add filler
	// lines (e.g., when adding lines above w_topline). But it's slow when
	// making many changes, postpone until redrawing.
	diff_need_scrollbind = TRUE;
    }
}

/*
 * Allocate a new diff block and link it between "dprev" and "dp".
 */
    static diff_T *
diff_alloc_new(tabpage_T *tp, diff_T *dprev, diff_T *dp)
{
    diff_T	*dnew;

    dnew = ALLOC_ONE(diff_T);
    if (dnew == NULL)
	return NULL;

    dnew->df_next = dp;
    if (dprev == NULL)
	tp->tp_first_diff = dnew;
    else
	dprev->df_next = dnew;
    return dnew;
}

/*
 * Check if the diff block "dp" can be made smaller for lines at the start and
 * end that are equal.  Called after inserting lines.
 * This may result in a change where all buffers have zero lines, the caller
 * must take care of removing it.
 */
    static void
diff_check_unchanged(tabpage_T *tp, diff_T *dp)
{
    int		i_org;
    int		i_new;
    int		off_org, off_new;
    char_u	*line_org;
    int		dir = FORWARD;

    // Find the first buffers, use it as the original, compare the other
    // buffer lines against this one.
    for (i_org = 0; i_org < DB_COUNT; ++i_org)
	if (tp->tp_diffbuf[i_org] != NULL)
	    break;
    if (i_org == DB_COUNT)	// safety check
	return;

    if (diff_check_sanity(tp, dp) == FAIL)
	return;

    // First check lines at the top, then at the bottom.
    off_org = 0;
    off_new = 0;
    for (;;)
    {
	// Repeat until a line is found which is different or the number of
	// lines has become zero.
	while (dp->df_count[i_org] > 0)
	{
	    // Copy the line, the next ml_get() will invalidate it.
	    if (dir == BACKWARD)
		off_org = dp->df_count[i_org] - 1;
	    line_org = vim_strsave(ml_get_buf(tp->tp_diffbuf[i_org],
					dp->df_lnum[i_org] + off_org, FALSE));
	    if (line_org == NULL)
		return;
	    for (i_new = i_org + 1; i_new < DB_COUNT; ++i_new)
	    {
		if (tp->tp_diffbuf[i_new] == NULL)
		    continue;
		if (dir == BACKWARD)
		    off_new = dp->df_count[i_new] - 1;
		// if other buffer doesn't have this line, it was inserted
		if (off_new < 0 || off_new >= dp->df_count[i_new])
		    break;
		if (diff_cmp(line_org, ml_get_buf(tp->tp_diffbuf[i_new],
				   dp->df_lnum[i_new] + off_new, FALSE)) != 0)
		    break;
	    }
	    vim_free(line_org);

	    // Stop when a line isn't equal in all diff buffers.
	    if (i_new != DB_COUNT)
		break;

	    // Line matched in all buffers, remove it from the diff.
	    for (i_new = i_org; i_new < DB_COUNT; ++i_new)
		if (tp->tp_diffbuf[i_new] != NULL)
		{
		    if (dir == FORWARD)
			++dp->df_lnum[i_new];
		    --dp->df_count[i_new];
		}
	}
	if (dir == BACKWARD)
	    break;
	dir = BACKWARD;
    }
}

/*
 * Check if a diff block doesn't contain invalid line numbers.
 * This can happen when the diff program returns invalid results.
 */
    static int
diff_check_sanity(tabpage_T *tp, diff_T *dp)
{
    int		i;

    for (i = 0; i < DB_COUNT; ++i)
	if (tp->tp_diffbuf[i] != NULL)
	    if (dp->df_lnum[i] + dp->df_count[i] - 1
				      > tp->tp_diffbuf[i]->b_ml.ml_line_count)
		return FAIL;
    return OK;
}

/*
 * Mark all diff buffers in the current tab page for redraw.
 */
    void
diff_redraw(
    int		dofold)	    // also recompute the folds
{
    win_T	*wp;
    win_T	*wp_other = NULL;
    int		used_max_fill_other = FALSE;
    int		used_max_fill_curwin = FALSE;
    int		n;

    need_diff_redraw = FALSE;
    FOR_ALL_WINDOWS(wp)
    {
	// when closing windows or wiping buffers skip invalid window
	if (!wp->w_p_diff || !buf_valid(wp->w_buffer))
	    continue;

	redraw_win_later(wp, UPD_SOME_VALID);
	if (wp != curwin)
	    wp_other = wp;
#ifdef FEAT_FOLDING
	if (dofold && foldmethodIsDiff(wp))
	    foldUpdateAll(wp);
#endif
	// A change may have made filler lines invalid, need to take care of
	// that for other windows.
	n = diff_check(wp, wp->w_topline);
	if ((wp != curwin && wp->w_topfill > 0) || n > 0)
	{
	    if (wp->w_topfill > n)
		wp->w_topfill = (n < 0 ? 0 : n);
	    else if (n > 0 && n > wp->w_topfill)
	    {
		wp->w_topfill = n;
		if (wp == curwin)
		    used_max_fill_curwin = TRUE;
		else if (wp_other != NULL)
		    used_max_fill_other = TRUE;
	    }
	    check_topfill(wp, FALSE);
	}
    }

    if (wp_other != NULL && curwin->w_p_scb)
    {
	if (used_max_fill_curwin)
	    // The current window was set to use the maximum number of filler
	    // lines, may need to reduce them.
	    diff_set_topline(wp_other, curwin);
	else if (used_max_fill_other)
	    // The other window was set to use the maximum number of filler
	    // lines, may need to reduce them.
	    diff_set_topline(curwin, wp_other);
    }
}

    static void
clear_diffin(diffin_T *din)
{
    if (din->din_fname == NULL)
	VIM_CLEAR(din->din_mmfile.ptr);
    else
	mch_remove(din->din_fname);
}

    static void
clear_diffout(diffout_T *dout)
{
    if (dout->dout_fname == NULL)
	ga_clear_strings(&dout->dout_ga);
    else
	mch_remove(dout->dout_fname);
}

/*
 * Write buffer "buf" to a memory buffer.
 * Return FAIL for failure.
 */
    static int
diff_write_buffer(buf_T *buf, diffin_T *din)
{
    linenr_T	lnum;
    char_u	*s;
    long	len = 0;
    char_u	*ptr;

    // xdiff requires one big block of memory with all the text.
    for (lnum = 1; lnum <= buf->b_ml.ml_line_count; ++lnum)
	len += (long)STRLEN(ml_get_buf(buf, lnum, FALSE)) + 1;
    ptr = alloc(len);
    if (ptr == NULL)
    {
	// Allocating memory failed.  This can happen, because we try to read
	// the whole buffer text into memory.  Set the failed flag, the diff
	// will be retried with external diff.  The flag is never reset.
	buf->b_diff_failed = TRUE;
	if (p_verbose > 0)
	{
	    verbose_enter();
	    smsg(_("Not enough memory to use internal diff for buffer \"%s\""),
								 buf->b_fname);
	    verbose_leave();
	}
	return FAIL;
    }
    din->din_mmfile.ptr = (char *)ptr;
    din->din_mmfile.size = len;

    len = 0;
    for (lnum = 1; lnum <= buf->b_ml.ml_line_count; ++lnum)
    {
	for (s = ml_get_buf(buf, lnum, FALSE); *s != NUL; )
	{
	    if (diff_flags & DIFF_ICASE)
	    {
		int c;
		int	orig_len;
		char_u	cbuf[MB_MAXBYTES + 1];

		if (*s == NL)
		    c = NUL;
		else
		{
		    // xdiff doesn't support ignoring case, fold-case the text.
		    c = PTR2CHAR(s);
		    c = MB_CASEFOLD(c);
		}
		orig_len = mb_ptr2len(s);
		if (mb_char2bytes(c, cbuf) != orig_len)
		    // TODO: handle byte length difference
		    mch_memmove(ptr + len, s, orig_len);
		else
		    mch_memmove(ptr + len, cbuf, orig_len);

		s += orig_len;
		len += orig_len;
	    }
	    else
	    {
		ptr[len++] = *s == NL ? NUL : *s;
		s++;
	    }
	}
	ptr[len++] = NL;
    }
    return OK;
}

/*
 * Write buffer "buf" to file or memory buffer.
 * Return FAIL for failure.
 */
    static int
diff_write(buf_T *buf, diffin_T *din)
{
    int		r;
    char_u	*save_ff;
    int		save_cmod_flags;

    if (din->din_fname == NULL)
	return diff_write_buffer(buf, din);

    // Always use 'fileformat' set to "unix".
    save_ff = buf->b_p_ff;
    buf->b_p_ff = vim_strsave((char_u *)FF_UNIX);
    save_cmod_flags = cmdmod.cmod_flags;
    // Writing the buffer is an implementation detail of performing the diff,
    // so it shouldn't update the '[ and '] marks.
    cmdmod.cmod_flags |= CMOD_LOCKMARKS;
    r = buf_write(buf, din->din_fname, NULL,
			(linenr_T)1, buf->b_ml.ml_line_count,
			NULL, FALSE, FALSE, FALSE, TRUE);
    cmdmod.cmod_flags = save_cmod_flags;
    free_string_option(buf->b_p_ff);
    buf->b_p_ff = save_ff;
    return r;
}

/*
 * Update the diffs for all buffers involved.
 */
    static void
diff_try_update(
	diffio_T    *dio,
	int	    idx_orig,
	exarg_T	    *eap)	// "eap" can be NULL
{
    buf_T	*buf;
    int		idx_new;

    if (dio->dio_internal)
    {
	ga_init2(&dio->dio_diff.dout_ga, sizeof(char *), 1000);
    }
    else
    {
	// We need three temp file names.
	dio->dio_orig.din_fname = vim_tempname('o', TRUE);
	dio->dio_new.din_fname = vim_tempname('n', TRUE);
	dio->dio_diff.dout_fname = vim_tempname('d', TRUE);
	if (dio->dio_orig.din_fname == NULL
		|| dio->dio_new.din_fname == NULL
		|| dio->dio_diff.dout_fname == NULL)
	    goto theend;
    }

    // Check external diff is actually working.
    if (!dio->dio_internal && check_external_diff(dio) == FAIL)
	goto theend;

    // :diffupdate!
    if (eap != NULL && eap->forceit)
	for (idx_new = idx_orig; idx_new < DB_COUNT; ++idx_new)
	{
	    buf = curtab->tp_diffbuf[idx_new];
	    if (buf_valid(buf))
		buf_check_timestamp(buf, FALSE);
	}

    // Write the first buffer to a tempfile or mmfile_t.
    buf = curtab->tp_diffbuf[idx_orig];
    if (diff_write(buf, &dio->dio_orig) == FAIL)
	goto theend;

    // Make a difference between the first buffer and every other.
    for (idx_new = idx_orig + 1; idx_new < DB_COUNT; ++idx_new)
    {
	buf = curtab->tp_diffbuf[idx_new];
	if (buf == NULL || buf->b_ml.ml_mfp == NULL)
	    continue; // skip buffer that isn't loaded

	// Write the other buffer and diff with the first one.
	if (diff_write(buf, &dio->dio_new) == FAIL)
	    continue;
	if (diff_file(dio) == FAIL)
	    continue;

	// Read the diff output and add each entry to the diff list.
	diff_read(idx_orig, idx_new, dio);

	clear_diffin(&dio->dio_new);
	clear_diffout(&dio->dio_diff);
    }
    clear_diffin(&dio->dio_orig);

theend:
    vim_free(dio->dio_orig.din_fname);
    vim_free(dio->dio_new.din_fname);
    vim_free(dio->dio_diff.dout_fname);
}

/*
 * Return TRUE if the options are set to use the internal diff library.
 * Note that if the internal diff failed for one of the buffers, the external
 * diff will be used anyway.
 */
    int
diff_internal(void)
{
    return (diff_flags & DIFF_INTERNAL) != 0
#ifdef FEAT_EVAL
	&& *p_dex == NUL
#endif
	;
}

/*
 * Return TRUE if the internal diff failed for one of the diff buffers.
 */
    static int
diff_internal_failed(void)
{
    int idx;

    // Only need to do something when there is another buffer.
    for (idx = 0; idx < DB_COUNT; ++idx)
	if (curtab->tp_diffbuf[idx] != NULL
		&& curtab->tp_diffbuf[idx]->b_diff_failed)
	    return TRUE;
    return FALSE;
}

/*
 * Completely update the diffs for the buffers involved.
 * When using the external "diff" command the buffers are written to a file,
 * also for unmodified buffers (the file could have been produced by
 * autocommands, e.g. the netrw plugin).
 */
    void
ex_diffupdate(exarg_T *eap)	// "eap" can be NULL
{
    int		idx_orig;
    int		idx_new;
    diffio_T	diffio;
    int		had_diffs = curtab->tp_first_diff != NULL;

    if (diff_busy)
    {
	diff_need_update = TRUE;
	return;
    }

    // Delete all diffblocks.
    diff_clear(curtab);
    curtab->tp_diff_invalid = FALSE;

    // Use the first buffer as the original text.
    for (idx_orig = 0; idx_orig < DB_COUNT; ++idx_orig)
	if (curtab->tp_diffbuf[idx_orig] != NULL)
	    break;
    if (idx_orig == DB_COUNT)
	goto theend;

    // Only need to do something when there is another buffer.
    for (idx_new = idx_orig + 1; idx_new < DB_COUNT; ++idx_new)
	if (curtab->tp_diffbuf[idx_new] != NULL)
	    break;
    if (idx_new == DB_COUNT)
	goto theend;

    // Only use the internal method if it did not fail for one of the buffers.
    CLEAR_FIELD(diffio);
    diffio.dio_internal = diff_internal() && !diff_internal_failed();

    diff_try_update(&diffio, idx_orig, eap);
    if (diffio.dio_internal && diff_internal_failed())
    {
	// Internal diff failed, use external diff instead.
	CLEAR_FIELD(diffio);
	diff_try_update(&diffio, idx_orig, eap);
    }

    // force updating cursor position on screen
    curwin->w_valid_cursor.lnum = 0;

theend:
    // A redraw is needed if there were diffs and they were cleared, or there
    // are diffs now, which means they got updated.
    if (had_diffs || curtab->tp_first_diff != NULL)
    {
	diff_redraw(TRUE);
	apply_autocmds(EVENT_DIFFUPDATED, NULL, NULL, FALSE, curbuf);
    }
}

/*
 * Do a quick test if "diff" really works.  Otherwise it looks like there
 * are no differences.  Can't use the return value, it's non-zero when
 * there are differences.
 */
    static int
check_external_diff(diffio_T *diffio)
{
    FILE	*fd;
    int		ok;
    int		io_error = FALSE;

    // May try twice, first with "-a" and then without.
    for (;;)
    {
	ok = FALSE;
	fd = mch_fopen((char *)diffio->dio_orig.din_fname, "w");
	if (fd == NULL)
	    io_error = TRUE;
	else
	{
	    if (fwrite("line1\n", (size_t)6, (size_t)1, fd) != 1)
		io_error = TRUE;
	    fclose(fd);
	    fd = mch_fopen((char *)diffio->dio_new.din_fname, "w");
	    if (fd == NULL)
		io_error = TRUE;
	    else
	    {
		if (fwrite("line2\n", (size_t)6, (size_t)1, fd) != 1)
		    io_error = TRUE;
		fclose(fd);
		fd = NULL;
		if (diff_file(diffio) == OK)
		    fd = mch_fopen((char *)diffio->dio_diff.dout_fname, "r");
		if (fd == NULL)
		    io_error = TRUE;
		else
		{
		    char_u	linebuf[LBUFLEN];

		    for (;;)
		    {
			// For normal diff there must be a line that contains
			// "1c1".  For unified diff "@@ -1 +1 @@".
			if (vim_fgets(linebuf, LBUFLEN, fd))
			    break;
			if (STRNCMP(linebuf, "1c1", 3) == 0
				|| STRNCMP(linebuf, "@@ -1 +1 @@", 11) == 0)
			    ok = TRUE;
		    }
		    fclose(fd);
		}
		mch_remove(diffio->dio_diff.dout_fname);
		mch_remove(diffio->dio_new.din_fname);
	    }
	    mch_remove(diffio->dio_orig.din_fname);
	}

#ifdef FEAT_EVAL
	// When using 'diffexpr' break here.
	if (*p_dex != NUL)
	    break;
#endif

#if defined(MSWIN)
	// If the "-a" argument works, also check if "--binary" works.
	if (ok && diff_a_works == MAYBE && diff_bin_works == MAYBE)
	{
	    diff_a_works = TRUE;
	    diff_bin_works = TRUE;
	    continue;
	}
	if (!ok && diff_a_works == TRUE && diff_bin_works == TRUE)
	{
	    // Tried --binary, but it failed. "-a" works though.
	    diff_bin_works = FALSE;
	    ok = TRUE;
	}
#endif

	// If we checked if "-a" works already, break here.
	if (diff_a_works != MAYBE)
	    break;
	diff_a_works = ok;

	// If "-a" works break here, otherwise retry without "-a".
	if (ok)
	    break;
    }
    if (!ok)
    {
	if (io_error)
	    emsg(_(e_cannot_read_or_write_temp_files));
	emsg(_(e_cannot_create_diffs));
	diff_a_works = MAYBE;
#if defined(MSWIN)
	diff_bin_works = MAYBE;
#endif
	return FAIL;
    }
    return OK;
}

/*
 * Invoke the xdiff function.
 */
    static int
diff_file_internal(diffio_T *diffio)
{
    xpparam_t	    param;
    xdemitconf_t    emit_cfg;
    xdemitcb_t	    emit_cb;

    CLEAR_FIELD(param);
    CLEAR_FIELD(emit_cfg);
    CLEAR_FIELD(emit_cb);

    param.flags = diff_algorithm;

    if (diff_flags & DIFF_IWHITE)
	param.flags |= XDF_IGNORE_WHITESPACE_CHANGE;
    if (diff_flags & DIFF_IWHITEALL)
	param.flags |= XDF_IGNORE_WHITESPACE;
    if (diff_flags & DIFF_IWHITEEOL)
	param.flags |= XDF_IGNORE_WHITESPACE_AT_EOL;
    if (diff_flags & DIFF_IBLANK)
	param.flags |= XDF_IGNORE_BLANK_LINES;

    emit_cfg.ctxlen = 0; // don't need any diff_context here
    emit_cb.priv = &diffio->dio_diff;
    emit_cfg.hunk_func = xdiff_out;
    if (xdl_diff(&diffio->dio_orig.din_mmfile,
		&diffio->dio_new.din_mmfile,
		&param, &emit_cfg, &emit_cb) < 0)
    {
	emsg(_(e_problem_creating_internal_diff));
	return FAIL;
    }
    return OK;
}

/*
 * Make a diff between files "tmp_orig" and "tmp_new", results in "tmp_diff".
 * return OK or FAIL;
 */
    static int
diff_file(diffio_T *dio)
{
    char_u	*cmd;
    size_t	len;
    char_u	*tmp_orig = dio->dio_orig.din_fname;
    char_u	*tmp_new = dio->dio_new.din_fname;
    char_u	*tmp_diff = dio->dio_diff.dout_fname;

#ifdef FEAT_EVAL
    if (*p_dex != NUL)
    {
	// Use 'diffexpr' to generate the diff file.
	eval_diff(tmp_orig, tmp_new, tmp_diff);
	return OK;
    }
    else
#endif
    // Use xdiff for generating the diff.
    if (dio->dio_internal)
	return diff_file_internal(dio);

    len = STRLEN(tmp_orig) + STRLEN(tmp_new)
				+ STRLEN(tmp_diff) + STRLEN(p_srr) + 27;
    cmd = alloc(len);
    if (cmd == NULL)
	return FAIL;

    // We don't want $DIFF_OPTIONS to get in the way.
    if (getenv("DIFF_OPTIONS"))
	vim_setenv((char_u *)"DIFF_OPTIONS", (char_u *)"");

    // Build the diff command and execute it.  Always use -a, binary
    // differences are of no use.  Ignore errors, diff returns
    // non-zero when differences have been found.
    vim_snprintf((char *)cmd, len, "diff %s%s%s%s%s%s%s%s %s",
	    diff_a_works == FALSE ? "" : "-a ",
#if defined(MSWIN)
	    diff_bin_works == TRUE ? "--binary " : "",
#else
	    "",
#endif
	    (diff_flags & DIFF_IWHITE) ? "-b " : "",
	    (diff_flags & DIFF_IWHITEALL) ? "-w " : "",
	    (diff_flags & DIFF_IWHITEEOL) ? "-Z " : "",
	    (diff_flags & DIFF_IBLANK) ? "-B " : "",
	    (diff_flags & DIFF_ICASE) ? "-i " : "",
	    tmp_orig, tmp_new);
    append_redir(cmd, (int)len, p_srr, tmp_diff);
    block_autocmds();	// avoid ShellCmdPost stuff
    (void)call_shell(cmd, SHELL_FILTER|SHELL_SILENT|SHELL_DOOUT);
    unblock_autocmds();
    vim_free(cmd);
    return OK;
}

/*
 * Create a new version of a file from the current buffer and a diff file.
 * The buffer is written to a file, also for unmodified buffers (the file
 * could have been produced by autocommands, e.g. the netrw plugin).
 */
    void
ex_diffpatch(exarg_T *eap)
{
    char_u	*tmp_orig;	// name of original temp file
    char_u	*tmp_new;	// name of patched temp file
    char_u	*buf = NULL;
    size_t	buflen;
    win_T	*old_curwin = curwin;
    char_u	*newname = NULL;	// name of patched file buffer
#ifdef UNIX
    char_u	dirbuf[MAXPATHL];
    char_u	*fullname = NULL;
#endif
#ifdef FEAT_BROWSE
    char_u	*browseFile = NULL;
    int		save_cmod_flags = cmdmod.cmod_flags;
#endif
    stat_T	st;
    char_u	*esc_name = NULL;

#ifdef FEAT_BROWSE
    if (cmdmod.cmod_flags & CMOD_BROWSE)
    {
	browseFile = do_browse(0, (char_u *)_("Patch file"),
			 eap->arg, NULL, NULL,
			 (char_u *)_(BROWSE_FILTER_ALL_FILES), NULL);
	if (browseFile == NULL)
	    return;		// operation cancelled
	eap->arg = browseFile;
	cmdmod.cmod_flags &= ~CMOD_BROWSE; // don't let do_ecmd() browse again
    }
#endif

    // We need two temp file names.
    tmp_orig = vim_tempname('o', FALSE);
    tmp_new = vim_tempname('n', FALSE);
    if (tmp_orig == NULL || tmp_new == NULL)
	goto theend;

    // Write the current buffer to "tmp_orig".
    if (buf_write(curbuf, tmp_orig, NULL,
		(linenr_T)1, curbuf->b_ml.ml_line_count,
				     NULL, FALSE, FALSE, FALSE, TRUE) == FAIL)
	goto theend;

#ifdef UNIX
    // Get the absolute path of the patchfile, changing directory below.
    fullname = FullName_save(eap->arg, FALSE);
#endif
    esc_name = vim_strsave_shellescape(
# ifdef UNIX
		    fullname != NULL ? fullname :
# endif
		    eap->arg, TRUE, TRUE);
    if (esc_name == NULL)
	goto theend;
    buflen = STRLEN(tmp_orig) + STRLEN(esc_name) + STRLEN(tmp_new) + 16;
    buf = alloc(buflen);
    if (buf == NULL)
	goto theend;

#ifdef UNIX
    // Temporarily chdir to /tmp, to avoid patching files in the current
    // directory when the patch file contains more than one patch.  When we
    // have our own temp dir use that instead, it will be cleaned up when we
    // exit (any .rej files created).  Don't change directory if we can't
    // return to the current.
    if (mch_dirname(dirbuf, MAXPATHL) != OK || mch_chdir((char *)dirbuf) != 0)
	dirbuf[0] = NUL;
    else
    {
# ifdef TEMPDIRNAMES
	if (vim_tempdir != NULL)
	    vim_ignored = mch_chdir((char *)vim_tempdir);
	else
# endif
	    vim_ignored = mch_chdir("/tmp");
	shorten_fnames(TRUE);
    }
#endif

#ifdef FEAT_EVAL
    if (*p_pex != NUL)
	// Use 'patchexpr' to generate the new file.
	eval_patch(tmp_orig,
# ifdef UNIX
		fullname != NULL ? fullname :
# endif
		eap->arg, tmp_new);
    else
#endif
    {
	if (check_restricted())
	    goto theend;

	// Build the patch command and execute it.  Ignore errors.  Switch to
	// cooked mode to allow the user to respond to prompts.
	vim_snprintf((char *)buf, buflen, "patch -o %s %s < %s",
						  tmp_new, tmp_orig, esc_name);
	block_autocmds();	// Avoid ShellCmdPost stuff
	(void)call_shell(buf, SHELL_FILTER | SHELL_COOKED);
	unblock_autocmds();
    }

#ifdef UNIX
    if (dirbuf[0] != NUL)
    {
	if (mch_chdir((char *)dirbuf) != 0)
	    emsg(_(e_cannot_go_back_to_previous_directory));
	shorten_fnames(TRUE);
    }
#endif

    // patch probably has written over the screen
    redraw_later(UPD_CLEAR);

    // Delete any .orig or .rej file created.
    STRCPY(buf, tmp_new);
    STRCAT(buf, ".orig");
    mch_remove(buf);
    STRCPY(buf, tmp_new);
    STRCAT(buf, ".rej");
    mch_remove(buf);

    // Only continue if the output file was created.
    if (mch_stat((char *)tmp_new, &st) < 0 || st.st_size == 0)
	emsg(_(e_cannot_read_patch_output));
    else
    {
	if (curbuf->b_fname != NULL)
	{
	    newname = vim_strnsave(curbuf->b_fname,
						  STRLEN(curbuf->b_fname) + 4);
	    if (newname != NULL)
		STRCAT(newname, ".new");
	}

#ifdef FEAT_GUI
	need_mouse_correct = TRUE;
#endif
	// don't use a new tab page, each tab page has its own diffs
	cmdmod.cmod_tab = 0;

	if (win_split(0, (diff_flags & DIFF_VERTICAL) ? WSP_VERT : 0) != FAIL)
	{
	    // Pretend it was a ":split fname" command
	    eap->cmdidx = CMD_split;
	    eap->arg = tmp_new;
	    do_exedit(eap, old_curwin);

	    // check that split worked and editing tmp_new
	    if (curwin != old_curwin && win_valid(old_curwin))
	    {
		// Set 'diff', 'scrollbind' on and 'wrap' off.
		diff_win_options(curwin, TRUE);
		diff_win_options(old_curwin, TRUE);

		if (newname != NULL)
		{
		    // do a ":file filename.new" on the patched buffer
		    eap->arg = newname;
		    ex_file(eap);

		    // Do filetype detection with the new name.
		    if (au_has_group((char_u *)"filetypedetect"))
			do_cmdline_cmd(
				     (char_u *)":doau filetypedetect BufRead");
		}
	    }
	}
    }

theend:
    if (tmp_orig != NULL)
	mch_remove(tmp_orig);
    vim_free(tmp_orig);
    if (tmp_new != NULL)
	mch_remove(tmp_new);
    vim_free(tmp_new);
    vim_free(newname);
    vim_free(buf);
#ifdef UNIX
    vim_free(fullname);
#endif
    vim_free(esc_name);
#ifdef FEAT_BROWSE
    vim_free(browseFile);
    cmdmod.cmod_flags = save_cmod_flags;
#endif
}

/*
 * Split the window and edit another file, setting options to show the diffs.
 */
    void
ex_diffsplit(exarg_T *eap)
{
    win_T	*old_curwin = curwin;
    bufref_T	old_curbuf;

    set_bufref(&old_curbuf, curbuf);
#ifdef FEAT_GUI
    need_mouse_correct = TRUE;
#endif
    // Need to compute w_fraction when no redraw happened yet.
    validate_cursor();
    set_fraction(curwin);

    // don't use a new tab page, each tab page has its own diffs
    cmdmod.cmod_tab = 0;

    if (win_split(0, (diff_flags & DIFF_VERTICAL) ? WSP_VERT : 0) == FAIL)
	return;

    // Pretend it was a ":split fname" command
    eap->cmdidx = CMD_split;
    curwin->w_p_diff = TRUE;
    do_exedit(eap, old_curwin);

    if (curwin == old_curwin)		// split didn't work
	return;

    // Set 'diff', 'scrollbind' on and 'wrap' off.
    diff_win_options(curwin, TRUE);
    if (win_valid(old_curwin))
    {
	diff_win_options(old_curwin, TRUE);

	if (bufref_valid(&old_curbuf))
	    // Move the cursor position to that of the old window.
	    curwin->w_cursor.lnum = diff_get_corresponding_line(
		    old_curbuf.br_buf, old_curwin->w_cursor.lnum);
    }
    // Now that lines are folded scroll to show the cursor at the same
    // relative position.
    scroll_to_fraction(curwin, curwin->w_height);
}

/*
 * Set options to show diffs for the current window.
 */
    void
ex_diffthis(exarg_T *eap UNUSED)
{
    // Set 'diff', 'scrollbind' on and 'wrap' off.
    diff_win_options(curwin, TRUE);
}

    static void
set_diff_option(win_T *wp, int value)
{
    win_T *old_curwin = curwin;

    curwin = wp;
    curbuf = curwin->w_buffer;
    ++curbuf_lock;
    set_option_value_give_err((char_u *)"diff", (long)value, NULL, OPT_LOCAL);
    --curbuf_lock;
    curwin = old_curwin;
    curbuf = curwin->w_buffer;
}

/*
 * Set options in window "wp" for diff mode.
 */
    void
diff_win_options(
    win_T	*wp,
    int		addbuf)		// Add buffer to diff.
{
# ifdef FEAT_FOLDING
    win_T *old_curwin = curwin;

    // close the manually opened folds
    curwin = wp;
    newFoldLevel();
    curwin = old_curwin;
# endif

    // Use 'scrollbind' and 'cursorbind' when available
    if (!wp->w_p_diff)
	wp->w_p_scb_save = wp->w_p_scb;
    wp->w_p_scb = TRUE;
    if (!wp->w_p_diff)
	wp->w_p_crb_save = wp->w_p_crb;
    wp->w_p_crb = TRUE;
    if (!(diff_flags & DIFF_FOLLOWWRAP))
    {
	if (!wp->w_p_diff)
	    wp->w_p_wrap_save = wp->w_p_wrap;
	wp->w_p_wrap = FALSE;
    }
# ifdef FEAT_FOLDING
    if (!wp->w_p_diff)
    {
	if (wp->w_p_diff_saved)
	    free_string_option(wp->w_p_fdm_save);
	wp->w_p_fdm_save = vim_strsave(wp->w_p_fdm);
    }
    set_string_option_direct_in_win(wp, (char_u *)"fdm", -1, (char_u *)"diff",
						       OPT_LOCAL|OPT_FREE, 0);
    if (!wp->w_p_diff)
    {
	wp->w_p_fdc_save = wp->w_p_fdc;
	wp->w_p_fen_save = wp->w_p_fen;
	wp->w_p_fdl_save = wp->w_p_fdl;
    }
    wp->w_p_fdc = diff_foldcolumn;
    wp->w_p_fen = TRUE;
    wp->w_p_fdl = 0;
    foldUpdateAll(wp);
    // make sure topline is not halfway a fold
    changed_window_setting_win(wp);
# endif
    if (vim_strchr(p_sbo, 'h') == NULL)
	do_cmdline_cmd((char_u *)"set sbo+=hor");
    // Save the current values, to be restored in ex_diffoff().
    wp->w_p_diff_saved = TRUE;

    set_diff_option(wp, TRUE);

    if (addbuf)
	diff_buf_add(wp->w_buffer);
    redraw_win_later(wp, UPD_NOT_VALID);
}

/*
 * Set options not to show diffs.  For the current window or all windows.
 * Only in the current tab page.
 */
    void
ex_diffoff(exarg_T *eap)
{
    win_T	*wp;
    int		diffwin = FALSE;

    FOR_ALL_WINDOWS(wp)
    {
	if (eap->forceit ? wp->w_p_diff : wp == curwin)
	{
	    // Set 'diff' off. If option values were saved in
	    // diff_win_options(), restore the ones whose settings seem to have
	    // been left over from diff mode.
	    set_diff_option(wp, FALSE);

	    if (wp->w_p_diff_saved)
	    {

		if (wp->w_p_scb)
		    wp->w_p_scb = wp->w_p_scb_save;
		if (wp->w_p_crb)
		    wp->w_p_crb = wp->w_p_crb_save;
		if (!(diff_flags & DIFF_FOLLOWWRAP))
		{
		    if (!wp->w_p_wrap)
			wp->w_p_wrap = wp->w_p_wrap_save;
		}
#ifdef FEAT_FOLDING
		free_string_option(wp->w_p_fdm);
		wp->w_p_fdm = vim_strsave(
		    *wp->w_p_fdm_save ? wp->w_p_fdm_save : (char_u*)"manual");

		if (wp->w_p_fdc == diff_foldcolumn)
		    wp->w_p_fdc = wp->w_p_fdc_save;
		if (wp->w_p_fdl == 0)
		    wp->w_p_fdl = wp->w_p_fdl_save;

		// Only restore 'foldenable' when 'foldmethod' is not
		// "manual", otherwise we continue to show the diff folds.
		if (wp->w_p_fen)
		    wp->w_p_fen = foldmethodIsManual(wp) ? FALSE
							 : wp->w_p_fen_save;

		foldUpdateAll(wp);
#endif
	    }
	    // remove filler lines
	    wp->w_topfill = 0;

	    // make sure topline is not halfway a fold and cursor is
	    // invalidated
	    changed_window_setting_win(wp);

	    // Note: 'sbo' is not restored, it's a global option.
	    diff_buf_adjust(wp);
	}
	diffwin |= wp->w_p_diff;
    }

    // Also remove hidden buffers from the list.
    if (eap->forceit)
	diff_buf_clear();

    if (!diffwin)
    {
	diff_need_update = FALSE;
	curtab->tp_diff_invalid = FALSE;
	curtab->tp_diff_update = FALSE;
	diff_clear(curtab);
    }

    // Remove "hor" from 'scrollopt' if there are no diff windows left.
    if (!diffwin && vim_strchr(p_sbo, 'h') != NULL)
	do_cmdline_cmd((char_u *)"set sbo-=hor");
}

/*
 * Read the diff output and add each entry to the diff list.
 */
    static void
diff_read(
    int		idx_orig,	// idx of original file
    int		idx_new,	// idx of new file
    diffio_T   *dio)		// diff output
{
    FILE	*fd = NULL;
    int		line_idx = 0;
    diff_T	*dprev = NULL;
    diff_T	*dp = curtab->tp_first_diff;
    diff_T	*dn, *dpl;
    diffout_T   *dout = &dio->dio_diff;
    char_u	linebuf[LBUFLEN];   // only need to hold the diff line
    char_u	*line;
    long	off;
    int		i;
    int		notset = TRUE;	    // block "*dp" not set yet
    diffhunk_T	*hunk = NULL;	    // init to avoid gcc warning

    enum {
	DIFF_ED,
	DIFF_UNIFIED,
	DIFF_NONE
    } diffstyle = DIFF_NONE;

    if (dout->dout_fname == NULL)
    {
	diffstyle = DIFF_UNIFIED;
    }
    else
    {
	fd = mch_fopen((char *)dout->dout_fname, "r");
	if (fd == NULL)
	{
	    emsg(_(e_cannot_read_diff_output));
	    return;
	}
    }

    if (!dio->dio_internal)
    {
	hunk = ALLOC_ONE(diffhunk_T);
	if (hunk == NULL)
	{
	    if (fd != NULL)
		fclose(fd);
	    return;
	}
    }

    for (;;)
    {
	if (dio->dio_internal)
	{
	    if (line_idx >= dout->dout_ga.ga_len)
		break;      // did last line
	    hunk = ((diffhunk_T **)dout->dout_ga.ga_data)[line_idx++];
	}
	else
	{
	    if (fd == NULL)
	    {
		if (line_idx >= dout->dout_ga.ga_len)
		    break;	    // did last line
		line = ((char_u **)dout->dout_ga.ga_data)[line_idx++];
	    }
	    else
	    {
		if (vim_fgets(linebuf, LBUFLEN, fd))
		    break;		// end of file
		line = linebuf;
	    }

	    if (diffstyle == DIFF_NONE)
	    {
		// Determine diff style.
		// ed like diff looks like this:
		// {first}[,{last}]c{first}[,{last}]
		// {first}a{first}[,{last}]
		// {first}[,{last}]d{first}
		//
		// unified diff looks like this:
		// --- file1       2018-03-20 13:23:35.783153140 +0100
		// +++ file2       2018-03-20 13:23:41.183156066 +0100
		// @@ -1,3 +1,5 @@
		if (SAFE_isdigit(*line))
		    diffstyle = DIFF_ED;
		else if ((STRNCMP(line, "@@ ", 3) == 0))
		    diffstyle = DIFF_UNIFIED;
		else if ((STRNCMP(line, "--- ", 4) == 0)
			&& (vim_fgets(linebuf, LBUFLEN, fd) == 0)
			&& (STRNCMP(line, "+++ ", 4) == 0)
			&& (vim_fgets(linebuf, LBUFLEN, fd) == 0)
			&& (STRNCMP(line, "@@ ", 3) == 0))
		    diffstyle = DIFF_UNIFIED;
		else
		    // Format not recognized yet, skip over this line.  Cygwin
		    // diff may put a warning at the start of the file.
		    continue;
	    }

	    if (diffstyle == DIFF_ED)
	    {
		if (!SAFE_isdigit(*line))
		    continue;	// not the start of a diff block
		if (parse_diff_ed(line, hunk) == FAIL)
		    continue;
	    }
	    else if (diffstyle == DIFF_UNIFIED)
	    {
		if (STRNCMP(line, "@@ ", 3)  != 0)
		    continue;	// not the start of a diff block
		if (parse_diff_unified(line, hunk) == FAIL)
		    continue;
	    }
	    else
	    {
		emsg(_(e_invalid_diff_format));
		break;
	    }
	}

	// Go over blocks before the change, for which orig and new are equal.
	// Copy blocks from orig to new.
	while (dp != NULL
		&& hunk->lnum_orig > dp->df_lnum[idx_orig]
						      + dp->df_count[idx_orig])
	{
	    if (notset)
		diff_copy_entry(dprev, dp, idx_orig, idx_new);
	    dprev = dp;
	    dp = dp->df_next;
	    notset = TRUE;
	}

	if (dp != NULL
		&& hunk->lnum_orig <= dp->df_lnum[idx_orig]
						       + dp->df_count[idx_orig]
		&& hunk->lnum_orig + hunk->count_orig >= dp->df_lnum[idx_orig])
	{
	    // New block overlaps with existing block(s).
	    // First find last block that overlaps.
	    for (dpl = dp; dpl->df_next != NULL; dpl = dpl->df_next)
		if (hunk->lnum_orig + hunk->count_orig
					     < dpl->df_next->df_lnum[idx_orig])
		    break;

	    // If the newly found block starts before the old one, set the
	    // start back a number of lines.
	    off = dp->df_lnum[idx_orig] - hunk->lnum_orig;
	    if (off > 0)
	    {
		for (i = idx_orig; i < idx_new; ++i)
		    if (curtab->tp_diffbuf[i] != NULL)
			dp->df_lnum[i] -= off;
		dp->df_lnum[idx_new] = hunk->lnum_new;
		dp->df_count[idx_new] = hunk->count_new;
	    }
	    else if (notset)
	    {
		// new block inside existing one, adjust new block
		dp->df_lnum[idx_new] = hunk->lnum_new + off;
		dp->df_count[idx_new] = hunk->count_new - off;
	    }
	    else
		// second overlap of new block with existing block
		dp->df_count[idx_new] += hunk->count_new - hunk->count_orig
		    + dpl->df_lnum[idx_orig] + dpl->df_count[idx_orig]
		    - (dp->df_lnum[idx_orig] + dp->df_count[idx_orig]);

	    // Adjust the size of the block to include all the lines to the
	    // end of the existing block or the new diff, whatever ends last.
	    off = (hunk->lnum_orig + hunk->count_orig)
			 - (dpl->df_lnum[idx_orig] + dpl->df_count[idx_orig]);
	    if (off < 0)
	    {
		// new change ends in existing block, adjust the end if not
		// done already
		if (notset)
		    dp->df_count[idx_new] += -off;
		off = 0;
	    }
	    for (i = idx_orig; i < idx_new; ++i)
		if (curtab->tp_diffbuf[i] != NULL)
		    dp->df_count[i] = dpl->df_lnum[i] + dpl->df_count[i]
						       - dp->df_lnum[i] + off;

	    // Delete the diff blocks that have been merged into one.
	    dn = dp->df_next;
	    dp->df_next = dpl->df_next;
	    while (dn != dp->df_next)
	    {
		dpl = dn->df_next;
		vim_free(dn);
		dn = dpl;
	    }
	}
	else
	{
	    // Allocate a new diffblock.
	    dp = diff_alloc_new(curtab, dprev, dp);
	    if (dp == NULL)
		goto done;

	    dp->df_lnum[idx_orig] = hunk->lnum_orig;
	    dp->df_count[idx_orig] = hunk->count_orig;
	    dp->df_lnum[idx_new] = hunk->lnum_new;
	    dp->df_count[idx_new] = hunk->count_new;

	    // Set values for other buffers, these must be equal to the
	    // original buffer, otherwise there would have been a change
	    // already.
	    for (i = idx_orig + 1; i < idx_new; ++i)
		if (curtab->tp_diffbuf[i] != NULL)
		    diff_copy_entry(dprev, dp, idx_orig, i);
	}
	notset = FALSE;		// "*dp" has been set
    }

    // for remaining diff blocks orig and new are equal
    while (dp != NULL)
    {
	if (notset)
	    diff_copy_entry(dprev, dp, idx_orig, idx_new);
	dprev = dp;
	dp = dp->df_next;
	notset = TRUE;
    }

done:
    if (!dio->dio_internal)
	vim_free(hunk);

    if (fd != NULL)
	fclose(fd);
}

/*
 * Copy an entry at "dp" from "idx_orig" to "idx_new".
 */
    static void
diff_copy_entry(
    diff_T	*dprev,
    diff_T	*dp,
    int		idx_orig,
    int		idx_new)
{
    long	off;

    if (dprev == NULL)
	off = 0;
    else
	off = (dprev->df_lnum[idx_orig] + dprev->df_count[idx_orig])
	    - (dprev->df_lnum[idx_new] + dprev->df_count[idx_new]);
    dp->df_lnum[idx_new] = dp->df_lnum[idx_orig] - off;
    dp->df_count[idx_new] = dp->df_count[idx_orig];
}

/*
 * Clear the list of diffblocks for tab page "tp".
 */
    void
diff_clear(tabpage_T *tp)
{
    diff_T	*p, *next_p;

    for (p = tp->tp_first_diff; p != NULL; p = next_p)
    {
	next_p = p->df_next;
	vim_free(p);
    }
    tp->tp_first_diff = NULL;
}

/*
 * Check diff status for line "lnum" in buffer "buf":
 * Returns 0 for nothing special
 * Returns -1 for a line that should be highlighted as changed.
 * Returns -2 for a line that should be highlighted as added/deleted.
 * Returns > 0 for inserting that many filler lines above it (never happens
 * when 'diffopt' doesn't contain "filler").
 * This should only be used for windows where 'diff' is set.
 */
    int
diff_check(win_T *wp, linenr_T lnum)
{
    int		idx;		// index in tp_diffbuf[] for this buffer
    diff_T	*dp;
    int		maxcount;
    int		i;
    buf_T	*buf = wp->w_buffer;
    int		cmp;

    if (curtab->tp_diff_invalid)
	ex_diffupdate(NULL);		// update after a big change

    if (curtab->tp_first_diff == NULL || !wp->w_p_diff)	// no diffs at all
	return 0;

    // safety check: "lnum" must be a buffer line
    if (lnum < 1 || lnum > buf->b_ml.ml_line_count + 1)
	return 0;

    idx = diff_buf_idx(buf);
    if (idx == DB_COUNT)
	return 0;		// no diffs for buffer "buf"

#ifdef FEAT_FOLDING
    // A closed fold never has filler lines.
    if (hasFoldingWin(wp, lnum, NULL, NULL, TRUE, NULL))
	return 0;
#endif

    // search for a change that includes "lnum" in the list of diffblocks.
    FOR_ALL_DIFFBLOCKS_IN_TAB(curtab, dp)
	if (lnum <= dp->df_lnum[idx] + dp->df_count[idx])
	    break;
    if (dp == NULL || lnum < dp->df_lnum[idx])
	return 0;

    if (lnum < dp->df_lnum[idx] + dp->df_count[idx])
    {
	int	zero = FALSE;

	// Changed or inserted line.  If the other buffers have a count of
	// zero, the lines were inserted.  If the other buffers have the same
	// count, check if the lines are identical.
	cmp = FALSE;
	for (i = 0; i < DB_COUNT; ++i)
	    if (i != idx && curtab->tp_diffbuf[i] != NULL)
	    {
		if (dp->df_count[i] == 0)
		    zero = TRUE;
		else
		{
		    if (dp->df_count[i] != dp->df_count[idx])
			return -1;	    // nr of lines changed.
		    cmp = TRUE;
		}
	    }
	if (cmp)
	{
	    // Compare all lines.  If they are equal the lines were inserted
	    // in some buffers, deleted in others, but not changed.
	    for (i = 0; i < DB_COUNT; ++i)
		if (i != idx && curtab->tp_diffbuf[i] != NULL
						      && dp->df_count[i] != 0)
		    if (!diff_equal_entry(dp, idx, i))
			return -1;
	}
	// If there is no buffer with zero lines then there is no difference
	// any longer.  Happens when making a change (or undo) that removes
	// the difference.  Can't remove the entry here, we might be halfway
	// updating the window.  Just report the text as unchanged.  Other
	// windows might still show the change though.
	if (zero == FALSE)
	    return 0;
	return -2;
    }

    // If 'diffopt' doesn't contain "filler", return 0.
    if (!(diff_flags & DIFF_FILLER))
	return 0;

    // Insert filler lines above the line just below the change.  Will return
    // 0 when this buf had the max count.
    maxcount = 0;
    for (i = 0; i < DB_COUNT; ++i)
	if (curtab->tp_diffbuf[i] != NULL && dp->df_count[i] > maxcount)
	    maxcount = dp->df_count[i];
    return maxcount - dp->df_count[idx];
}

/*
 * Compare two entries in diff "*dp" and return TRUE if they are equal.
 */
    static int
diff_equal_entry(diff_T *dp, int idx1, int idx2)
{
    int		i;
    char_u	*line;
    int		cmp;

    if (dp->df_count[idx1] != dp->df_count[idx2])
	return FALSE;
    if (diff_check_sanity(curtab, dp) == FAIL)
	return FALSE;
    for (i = 0; i < dp->df_count[idx1]; ++i)
    {
	line = vim_strsave(ml_get_buf(curtab->tp_diffbuf[idx1],
					       dp->df_lnum[idx1] + i, FALSE));
	if (line == NULL)
	    return FALSE;
	cmp = diff_cmp(line, ml_get_buf(curtab->tp_diffbuf[idx2],
					       dp->df_lnum[idx2] + i, FALSE));
	vim_free(line);
	if (cmp != 0)
	    return FALSE;
    }
    return TRUE;
}

/*
 * Compare the characters at "p1" and "p2".  If they are equal (possibly
 * ignoring case) return TRUE and set "len" to the number of bytes.
 */
    static int
diff_equal_char(char_u *p1, char_u *p2, int *len)
{
    int l  = (*mb_ptr2len)(p1);

    if (l != (*mb_ptr2len)(p2))
	return FALSE;
    if (l > 1)
    {
	if (STRNCMP(p1, p2, l) != 0
		&& (!enc_utf8
		    || !(diff_flags & DIFF_ICASE)
		    || utf_fold(utf_ptr2char(p1))
						!= utf_fold(utf_ptr2char(p2))))
	    return FALSE;
	*len = l;
    }
    else
    {
	if ((*p1 != *p2)
		&& (!(diff_flags & DIFF_ICASE)
		    || TOLOWER_LOC(*p1) != TOLOWER_LOC(*p2)))
	    return FALSE;
	*len = 1;
    }
    return TRUE;
}

/*
 * Compare strings "s1" and "s2" according to 'diffopt'.
 * Return non-zero when they are different.
 */
    static int
diff_cmp(char_u *s1, char_u *s2)
{
    char_u	*p1, *p2;
    int		l;

    if ((diff_flags & DIFF_IBLANK)
	    && (*skipwhite(s1) == NUL || *skipwhite(s2) == NUL))
	return 0;

    if ((diff_flags & (DIFF_ICASE | ALL_WHITE_DIFF)) == 0)
	return STRCMP(s1, s2);
    if ((diff_flags & DIFF_ICASE) && !(diff_flags & ALL_WHITE_DIFF))
	return MB_STRICMP(s1, s2);

    p1 = s1;
    p2 = s2;

    // Ignore white space changes and possibly ignore case.
    while (*p1 != NUL && *p2 != NUL)
    {
	if (((diff_flags & DIFF_IWHITE)
		    && VIM_ISWHITE(*p1) && VIM_ISWHITE(*p2))
		|| ((diff_flags & DIFF_IWHITEALL)
		    && (VIM_ISWHITE(*p1) || VIM_ISWHITE(*p2))))
	{
	    p1 = skipwhite(p1);
	    p2 = skipwhite(p2);
	}
	else
	{
	    if (!diff_equal_char(p1, p2, &l))
		break;
	    p1 += l;
	    p2 += l;
	}
    }

    // Ignore trailing white space.
    p1 = skipwhite(p1);
    p2 = skipwhite(p2);
    if (*p1 != NUL || *p2 != NUL)
	return 1;
    return 0;
}

/*
 * Return the number of filler lines above "lnum".
 */
    int
diff_check_fill(win_T *wp, linenr_T lnum)
{
    int		n;

    // be quick when there are no filler lines
    if (!(diff_flags & DIFF_FILLER))
	return 0;
    n = diff_check(wp, lnum);
    if (n <= 0)
	return 0;
    return n;
}

/*
 * Set the topline of "towin" to match the position in "fromwin", so that they
 * show the same diff'ed lines.
 */
    void
diff_set_topline(win_T *fromwin, win_T *towin)
{
    buf_T	*frombuf = fromwin->w_buffer;
    linenr_T	lnum = fromwin->w_topline;
    int		fromidx;
    int		toidx;
    diff_T	*dp;
    int		max_count;
    int		i;

    fromidx = diff_buf_idx(frombuf);
    if (fromidx == DB_COUNT)
	return;		// safety check

    if (curtab->tp_diff_invalid)
	ex_diffupdate(NULL);		// update after a big change

    towin->w_topfill = 0;

    // search for a change that includes "lnum" in the list of diffblocks.
    FOR_ALL_DIFFBLOCKS_IN_TAB(curtab, dp)
	if (lnum <= dp->df_lnum[fromidx] + dp->df_count[fromidx])
	    break;
    if (dp == NULL)
    {
	// After last change, compute topline relative to end of file; no
	// filler lines.
	towin->w_topline = towin->w_buffer->b_ml.ml_line_count
				       - (frombuf->b_ml.ml_line_count - lnum);
    }
    else
    {
	// Find index for "towin".
	toidx = diff_buf_idx(towin->w_buffer);
	if (toidx == DB_COUNT)
	    return;		// safety check

	towin->w_topline = lnum + (dp->df_lnum[toidx] - dp->df_lnum[fromidx]);
	if (lnum >= dp->df_lnum[fromidx])
	{
	    // Inside a change: compute filler lines. With three or more
	    // buffers we need to know the largest count.
	    max_count = 0;
	    for (i = 0; i < DB_COUNT; ++i)
		if (curtab->tp_diffbuf[i] != NULL
					       && max_count < dp->df_count[i])
		    max_count = dp->df_count[i];

	    if (dp->df_count[toidx] == dp->df_count[fromidx])
	    {
		// same number of lines: use same filler count
		towin->w_topfill = fromwin->w_topfill;
	    }
	    else if (dp->df_count[toidx] > dp->df_count[fromidx])
	    {
		if (lnum == dp->df_lnum[fromidx] + dp->df_count[fromidx])
		{
		    // more lines in towin and fromwin doesn't show diff
		    // lines, only filler lines
		    if (max_count - fromwin->w_topfill >= dp->df_count[toidx])
		    {
			// towin also only shows filler lines
			towin->w_topline = dp->df_lnum[toidx]
						       + dp->df_count[toidx];
			towin->w_topfill = fromwin->w_topfill;
		    }
		    else
			// towin still has some diff lines to show
			towin->w_topline = dp->df_lnum[toidx]
					     + max_count - fromwin->w_topfill;
		}
	    }
	    else if (towin->w_topline >= dp->df_lnum[toidx]
							+ dp->df_count[toidx])
	    {
		// less lines in towin and no diff lines to show: compute
		// filler lines
		towin->w_topline = dp->df_lnum[toidx] + dp->df_count[toidx];
		if (diff_flags & DIFF_FILLER)
		{
		    if (lnum == dp->df_lnum[fromidx] + dp->df_count[fromidx])
			// fromwin is also out of diff lines
			towin->w_topfill = fromwin->w_topfill;
		    else
			// fromwin has some diff lines
			towin->w_topfill = dp->df_lnum[fromidx]
							   + max_count - lnum;
		}
	    }
	}
    }

    // safety check (if diff info gets outdated strange things may happen)
    towin->w_botfill = FALSE;
    if (towin->w_topline > towin->w_buffer->b_ml.ml_line_count)
    {
	towin->w_topline = towin->w_buffer->b_ml.ml_line_count;
	towin->w_botfill = TRUE;
    }
    if (towin->w_topline < 1)
    {
	towin->w_topline = 1;
	towin->w_topfill = 0;
    }

    // When w_topline changes need to recompute w_botline and cursor position
    invalidate_botline_win(towin);
    changed_line_abv_curs_win(towin);

    check_topfill(towin, FALSE);
#ifdef FEAT_FOLDING
    (void)hasFoldingWin(towin, towin->w_topline, &towin->w_topline,
							    NULL, TRUE, NULL);
#endif
}

/*
 * This is called when 'diffopt' is changed.
 */
    int
diffopt_changed(void)
{
    char_u	*p;
    int		diff_context_new = 6;
    int		diff_flags_new = 0;
    int		diff_foldcolumn_new = 2;
    long	diff_algorithm_new = 0;
    long	diff_indent_heuristic = 0;
    tabpage_T	*tp;

    p = p_dip;
    while (*p != NUL)
    {
	// Note: Keep this in sync with p_dip_values
	if (STRNCMP(p, "filler", 6) == 0)
	{
	    p += 6;
	    diff_flags_new |= DIFF_FILLER;
	}
	else if (STRNCMP(p, "context:", 8) == 0 && VIM_ISDIGIT(p[8]))
	{
	    p += 8;
	    diff_context_new = getdigits(&p);
	}
	else if (STRNCMP(p, "iblank", 6) == 0)
	{
	    p += 6;
	    diff_flags_new |= DIFF_IBLANK;
	}
	else if (STRNCMP(p, "icase", 5) == 0)
	{
	    p += 5;
	    diff_flags_new |= DIFF_ICASE;
	}
	else if (STRNCMP(p, "iwhiteall", 9) == 0)
	{
	    p += 9;
	    diff_flags_new |= DIFF_IWHITEALL;
	}
	else if (STRNCMP(p, "iwhiteeol", 9) == 0)
	{
	    p += 9;
	    diff_flags_new |= DIFF_IWHITEEOL;
	}
	else if (STRNCMP(p, "iwhite", 6) == 0)
	{
	    p += 6;
	    diff_flags_new |= DIFF_IWHITE;
	}
	else if (STRNCMP(p, "horizontal", 10) == 0)
	{
	    p += 10;
	    diff_flags_new |= DIFF_HORIZONTAL;
	}
	else if (STRNCMP(p, "vertical", 8) == 0)
	{
	    p += 8;
	    diff_flags_new |= DIFF_VERTICAL;
	}
	else if (STRNCMP(p, "foldcolumn:", 11) == 0 && VIM_ISDIGIT(p[11]))
	{
	    p += 11;
	    diff_foldcolumn_new = getdigits(&p);
	}
	else if (STRNCMP(p, "hiddenoff", 9) == 0)
	{
	    p += 9;
	    diff_flags_new |= DIFF_HIDDEN_OFF;
	}
	else if (STRNCMP(p, "closeoff", 8) == 0)
	{
	    p += 8;
	    diff_flags_new |= DIFF_CLOSE_OFF;
	}
	else if (STRNCMP(p, "followwrap", 10) == 0)
	{
	    p += 10;
	    diff_flags_new |= DIFF_FOLLOWWRAP;
	}
	else if (STRNCMP(p, "indent-heuristic", 16) == 0)
	{
	    p += 16;
	    diff_indent_heuristic = XDF_INDENT_HEURISTIC;
	}
	else if (STRNCMP(p, "internal", 8) == 0)
	{
	    p += 8;
	    diff_flags_new |= DIFF_INTERNAL;
	}
	else if (STRNCMP(p, "algorithm:", 10) == 0)
	{
	    // Note: Keep this in sync with p_dip_algorithm_values.
	    p += 10;
	    if (STRNCMP(p, "myers", 5) == 0)
	    {
		p += 5;
		diff_algorithm_new = 0;
	    }
	    else if (STRNCMP(p, "minimal", 7) == 0)
	    {
		p += 7;
		diff_algorithm_new = XDF_NEED_MINIMAL;
	    }
	    else if (STRNCMP(p, "patience", 8) == 0)
	    {
		p += 8;
		diff_algorithm_new = XDF_PATIENCE_DIFF;
	    }
	    else if (STRNCMP(p, "histogram", 9) == 0)
	    {
		p += 9;
		diff_algorithm_new = XDF_HISTOGRAM_DIFF;
	    }
	    else
		return FAIL;
	}

	if (*p != ',' && *p != NUL)
	    return FAIL;
	if (*p == ',')
	    ++p;
    }

    diff_algorithm_new |= diff_indent_heuristic;

    // Can't have both "horizontal" and "vertical".
    if ((diff_flags_new & DIFF_HORIZONTAL) && (diff_flags_new & DIFF_VERTICAL))
	return FAIL;

    // If flags were added or removed, or the algorithm was changed, need to
    // update the diff.
    if (diff_flags != diff_flags_new || diff_algorithm != diff_algorithm_new)
	FOR_ALL_TABPAGES(tp)
	    tp->tp_diff_invalid = TRUE;

    diff_flags = diff_flags_new;
    diff_context = diff_context_new == 0 ? 1 : diff_context_new;
    diff_foldcolumn = diff_foldcolumn_new;
    diff_algorithm = diff_algorithm_new;

    diff_redraw(TRUE);

    // recompute the scroll binding with the new option value, may
    // remove or add filler lines
    check_scrollbind((linenr_T)0, 0L);

    return OK;
}

/*
 * Return TRUE if 'diffopt' contains "horizontal".
 */
    int
diffopt_horizontal(void)
{
    return (diff_flags & DIFF_HORIZONTAL) != 0;
}

/*
 * Return TRUE if 'diffopt' contains "hiddenoff".
 */
    int
diffopt_hiddenoff(void)
{
    return (diff_flags & DIFF_HIDDEN_OFF) != 0;
}

/*
 * Return TRUE if 'diffopt' contains "closeoff".
 */
    int
diffopt_closeoff(void)
{
    return (diff_flags & DIFF_CLOSE_OFF) != 0;
}

/*
 * Find the difference within a changed line.
 * Returns TRUE if the line was added, no other buffer has it.
 */
    int
diff_find_change(
    win_T	*wp,
    linenr_T	lnum,
    int		*startp,	// first char of the change
    int		*endp)		// last char of the change
{
    char_u	*line_org;
    char_u	*line_new;
    int		i;
    int		si_org, si_new;
    int		ei_org, ei_new;
    diff_T	*dp;
    int		idx;
    int		off;
    int		added = TRUE;
    char_u	*p1, *p2;
    int		l;

    // Make a copy of the line, the next ml_get() will invalidate it.
    line_org = vim_strsave(ml_get_buf(wp->w_buffer, lnum, FALSE));
    if (line_org == NULL)
	return FALSE;

    idx = diff_buf_idx(wp->w_buffer);
    if (idx == DB_COUNT)	// cannot happen
    {
	vim_free(line_org);
	return FALSE;
    }

    // search for a change that includes "lnum" in the list of diffblocks.
    FOR_ALL_DIFFBLOCKS_IN_TAB(curtab, dp)
	if (lnum <= dp->df_lnum[idx] + dp->df_count[idx])
	    break;
    if (dp == NULL || diff_check_sanity(curtab, dp) == FAIL)
    {
	vim_free(line_org);
	return FALSE;
    }

    off = lnum - dp->df_lnum[idx];

    for (i = 0; i < DB_COUNT; ++i)
	if (curtab->tp_diffbuf[i] != NULL && i != idx)
	{
	    // Skip lines that are not in the other change (filler lines).
	    if (off >= dp->df_count[i])
		continue;
	    added = FALSE;
	    line_new = ml_get_buf(curtab->tp_diffbuf[i],
						 dp->df_lnum[i] + off, FALSE);

	    // Search for start of difference
	    si_org = si_new = 0;
	    while (line_org[si_org] != NUL)
	    {
		if (((diff_flags & DIFF_IWHITE)
			    && VIM_ISWHITE(line_org[si_org])
					      && VIM_ISWHITE(line_new[si_new]))
			|| ((diff_flags & DIFF_IWHITEALL)
			    && (VIM_ISWHITE(line_org[si_org])
					    || VIM_ISWHITE(line_new[si_new]))))
		{
		    si_org = (int)(skipwhite(line_org + si_org) - line_org);
		    si_new = (int)(skipwhite(line_new + si_new) - line_new);
		}
		else
		{
		    if (!diff_equal_char(line_org + si_org, line_new + si_new,
									   &l))
			break;
		    si_org += l;
		    si_new += l;
		}
	    }
	    if (has_mbyte)
	    {
		// Move back to first byte of character in both lines (may
		// have "nn^" in line_org and "n^ in line_new).
		si_org -= (*mb_head_off)(line_org, line_org + si_org);
		si_new -= (*mb_head_off)(line_new, line_new + si_new);
	    }
	    if (*startp > si_org)
		*startp = si_org;

	    // Search for end of difference, if any.
	    if (line_org[si_org] != NUL || line_new[si_new] != NUL)
	    {
		ei_org = (int)STRLEN(line_org);
		ei_new = (int)STRLEN(line_new);
		while (ei_org >= *startp && ei_new >= si_new
						&& ei_org >= 0 && ei_new >= 0)
		{
		    if (((diff_flags & DIFF_IWHITE)
				&& VIM_ISWHITE(line_org[ei_org])
					      && VIM_ISWHITE(line_new[ei_new]))
			    || ((diff_flags & DIFF_IWHITEALL)
				&& (VIM_ISWHITE(line_org[ei_org])
					    || VIM_ISWHITE(line_new[ei_new]))))
		    {
			while (ei_org >= *startp
					     && VIM_ISWHITE(line_org[ei_org]))
			    --ei_org;
			while (ei_new >= si_new
					     && VIM_ISWHITE(line_new[ei_new]))
			    --ei_new;
		    }
		    else
		    {
			p1 = line_org + ei_org;
			p2 = line_new + ei_new;
			p1 -= (*mb_head_off)(line_org, p1);
			p2 -= (*mb_head_off)(line_new, p2);
			if (!diff_equal_char(p1, p2, &l))
			    break;
			ei_org -= l;
			ei_new -= l;
		    }
		}
		if (*endp < ei_org)
		    *endp = ei_org;
	    }
	}

    vim_free(line_org);
    return added;
}

#if defined(FEAT_FOLDING) || defined(PROTO)
/*
 * Return TRUE if line "lnum" is not close to a diff block, this line should
 * be in a fold.
 * Return FALSE if there are no diff blocks at all in this window.
 */
    int
diff_infold(win_T *wp, linenr_T lnum)
{
    int		i;
    int		idx = -1;
    int		other = FALSE;
    diff_T	*dp;

    // Return if 'diff' isn't set.
    if (!wp->w_p_diff)
	return FALSE;

    for (i = 0; i < DB_COUNT; ++i)
    {
	if (curtab->tp_diffbuf[i] == wp->w_buffer)
	    idx = i;
	else if (curtab->tp_diffbuf[i] != NULL)
	    other = TRUE;
    }

    // return here if there are no diffs in the window
    if (idx == -1 || !other)
	return FALSE;

    if (curtab->tp_diff_invalid)
	ex_diffupdate(NULL);		// update after a big change

    // Return if there are no diff blocks.  All lines will be folded.
    if (curtab->tp_first_diff == NULL)
	return TRUE;

    FOR_ALL_DIFFBLOCKS_IN_TAB(curtab, dp)
    {
	// If this change is below the line there can't be any further match.
	if (dp->df_lnum[idx] - diff_context > lnum)
	    break;
	// If this change ends before the line we have a match.
	if (dp->df_lnum[idx] + dp->df_count[idx] + diff_context > lnum)
	    return FALSE;
    }
    return TRUE;
}
#endif

/*
 * "dp" and "do" commands.
 */
    void
nv_diffgetput(int put, long count)
{
    exarg_T	ea;
    char_u	buf[30];

#ifdef FEAT_JOB_CHANNEL
    if (bt_prompt(curbuf))
    {
	vim_beep(BO_OPER);
	return;
    }
#endif
    if (count == 0)
	ea.arg = (char_u *)"";
    else
    {
	vim_snprintf((char *)buf, 30, "%ld", count);
	ea.arg = buf;
    }
    if (put)
	ea.cmdidx = CMD_diffput;
    else
	ea.cmdidx = CMD_diffget;
    ea.addr_count = 0;
    ea.line1 = curwin->w_cursor.lnum;
    ea.line2 = curwin->w_cursor.lnum;
    ex_diffgetput(&ea);
}

/*
 * Return TRUE if "diff" appears in the list of diff blocks of the current tab.
 */
    static int
valid_diff(diff_T *diff)
{
    diff_T	*dp;

    FOR_ALL_DIFFBLOCKS_IN_TAB(curtab, dp)
	if (dp == diff)
	    return TRUE;
    return FALSE;
}

/*
 * ":diffget"
 * ":diffput"
 */
    void
ex_diffgetput(exarg_T *eap)
{
    linenr_T	lnum;
    int		count;
    linenr_T	off = 0;
    diff_T	*dp;
    diff_T	*dprev;
    diff_T	*dfree;
    int		idx_cur;
    int		idx_other;
    int		idx_from;
    int		idx_to;
    int		i;
    int		added;
    char_u	*p;
    aco_save_T	aco;
    buf_T	*buf;
    int		start_skip, end_skip;
    int		new_count;
    int		buf_empty;
    int		found_not_ma = FALSE;

    // Find the current buffer in the list of diff buffers.
    idx_cur = diff_buf_idx(curbuf);
    if (idx_cur == DB_COUNT)
    {
	emsg(_(e_current_buffer_is_not_in_diff_mode));
	return;
    }

    if (*eap->arg == NUL)
    {
	// No argument: Find the other buffer in the list of diff buffers.
	for (idx_other = 0; idx_other < DB_COUNT; ++idx_other)
	    if (curtab->tp_diffbuf[idx_other] != curbuf
		    && curtab->tp_diffbuf[idx_other] != NULL)
	    {
		if (eap->cmdidx != CMD_diffput
				     || curtab->tp_diffbuf[idx_other]->b_p_ma)
		    break;
		found_not_ma = TRUE;
	    }
	if (idx_other == DB_COUNT)
	{
	    if (found_not_ma)
		emsg(_(e_no_other_buffer_in_diff_mode_is_modifiable));
	    else
		emsg(_(e_no_other_buffer_in_diff_mode));
	    return;
	}

	// Check that there isn't a third buffer in the list
	for (i = idx_other + 1; i < DB_COUNT; ++i)
	    if (curtab->tp_diffbuf[i] != curbuf
		    && curtab->tp_diffbuf[i] != NULL
		    && (eap->cmdidx != CMD_diffput || curtab->tp_diffbuf[i]->b_p_ma))
	    {
		emsg(_(e_more_than_two_buffers_in_diff_mode_dont_know_which_one_to_use));
		return;
	    }
    }
    else
    {
	// Buffer number or pattern given.  Ignore trailing white space.
	p = eap->arg + STRLEN(eap->arg);
	while (p > eap->arg && VIM_ISWHITE(p[-1]))
	    --p;
	for (i = 0; vim_isdigit(eap->arg[i]) && eap->arg + i < p; ++i)
	    ;
	if (eap->arg + i == p)	    // digits only
	    i = atol((char *)eap->arg);
	else
	{
	    i = buflist_findpat(eap->arg, p, FALSE, TRUE, FALSE);
	    if (i < 0)
		return;		// error message already given
	}
	buf = buflist_findnr(i);
	if (buf == NULL)
	{
	    semsg(_(e_cant_find_buffer_str), eap->arg);
	    return;
	}
	if (buf == curbuf)
	    return;		// nothing to do
	idx_other = diff_buf_idx(buf);
	if (idx_other == DB_COUNT)
	{
	    semsg(_(e_buffer_str_is_not_in_diff_mode), eap->arg);
	    return;
	}
    }

    diff_busy = TRUE;

    // When no range given include the line above or below the cursor.
    if (eap->addr_count == 0)
    {
	// Make it possible that ":diffget" on the last line gets line below
	// the cursor line when there is no difference above the cursor.
	if (eap->cmdidx == CMD_diffget
		&& eap->line1 == curbuf->b_ml.ml_line_count
		&& diff_check(curwin, eap->line1) == 0
		&& (eap->line1 == 1 || diff_check(curwin, eap->line1 - 1) == 0))
	    ++eap->line2;
	else if (eap->line1 > 0)
	    --eap->line1;
    }

    if (eap->cmdidx == CMD_diffget)
    {
	idx_from = idx_other;
	idx_to = idx_cur;
    }
    else
    {
	idx_from = idx_cur;
	idx_to = idx_other;
	// Need to make the other buffer the current buffer to be able to make
	// changes in it.
	// Set curwin/curbuf to buf and save a few things.
	aucmd_prepbuf(&aco, curtab->tp_diffbuf[idx_other]);
	if (curbuf != curtab->tp_diffbuf[idx_other])
	    // Could not find a window for this buffer, the rest is likely to
	    // fail.
	    goto theend;
    }

    // May give the warning for a changed buffer here, which can trigger the
    // FileChangedRO autocommand, which may do nasty things and mess
    // everything up.
    if (!curbuf->b_changed)
    {
	change_warning(0);
	if (diff_buf_idx(curbuf) != idx_to)
	{
	    emsg(_(e_buffer_changed_unexpectedly));
	    goto theend;
	}
    }

    dprev = NULL;
    for (dp = curtab->tp_first_diff; dp != NULL; )
    {
	if (dp->df_lnum[idx_cur] > eap->line2 + off)
	    break;	// past the range that was specified

	dfree = NULL;
	lnum = dp->df_lnum[idx_to];
	count = dp->df_count[idx_to];
	if (dp->df_lnum[idx_cur] + dp->df_count[idx_cur] > eap->line1 + off
		&& u_save(lnum - 1, lnum + count) != FAIL)
	{
	    // Inside the specified range and saving for undo worked.
	    start_skip = 0;
	    end_skip = 0;
	    if (eap->addr_count > 0)
	    {
		// A range was specified: check if lines need to be skipped.
		start_skip = eap->line1 + off - dp->df_lnum[idx_cur];
		if (start_skip > 0)
		{
		    // range starts below start of current diff block
		    if (start_skip > count)
		    {
			lnum += count;
			count = 0;
		    }
		    else
		    {
			count -= start_skip;
			lnum += start_skip;
		    }
		}
		else
		    start_skip = 0;

		end_skip = dp->df_lnum[idx_cur] + dp->df_count[idx_cur] - 1
							 - (eap->line2 + off);
		if (end_skip > 0)
		{
		    // range ends above end of current/from diff block
		    if (idx_cur == idx_from)	// :diffput
		    {
			i = dp->df_count[idx_cur] - start_skip - end_skip;
			if (count > i)
			    count = i;
		    }
		    else			// :diffget
		    {
			count -= end_skip;
			end_skip = dp->df_count[idx_from] - start_skip - count;
			if (end_skip < 0)
			    end_skip = 0;
		    }
		}
		else
		    end_skip = 0;
	    }

	    buf_empty = BUFEMPTY();
	    added = 0;
	    for (i = 0; i < count; ++i)
	    {
		// remember deleting the last line of the buffer
		buf_empty = curbuf->b_ml.ml_line_count == 1;
		if (ml_delete(lnum) == OK)
		    --added;
	    }
	    for (i = 0; i < dp->df_count[idx_from] - start_skip - end_skip; ++i)
	    {
		linenr_T nr;

		nr = dp->df_lnum[idx_from] + start_skip + i;
		if (nr > curtab->tp_diffbuf[idx_from]->b_ml.ml_line_count)
		    break;
		p = vim_strsave(ml_get_buf(curtab->tp_diffbuf[idx_from],
								  nr, FALSE));
		if (p != NULL)
		{
		    ml_append(lnum + i - 1, p, 0, FALSE);
		    vim_free(p);
		    ++added;
		    if (buf_empty && curbuf->b_ml.ml_line_count == 2)
		    {
			// Added the first line into an empty buffer, need to
			// delete the dummy empty line.
			buf_empty = FALSE;
			ml_delete((linenr_T)2);
		    }
		}
	    }
	    new_count = dp->df_count[idx_to] + added;
	    dp->df_count[idx_to] = new_count;

	    if (start_skip == 0 && end_skip == 0)
	    {
		// Check if there are any other buffers and if the diff is
		// equal in them.
		for (i = 0; i < DB_COUNT; ++i)
		    if (curtab->tp_diffbuf[i] != NULL && i != idx_from
								&& i != idx_to
			    && !diff_equal_entry(dp, idx_from, i))
			break;
		if (i == DB_COUNT)
		{
		    // delete the diff entry, the buffers are now equal here
		    dfree = dp;
		    dp = dp->df_next;
		    if (dprev == NULL)
			curtab->tp_first_diff = dp;
		    else
			dprev->df_next = dp;
		}
	    }

	    if (added != 0)
	    {
		// Adjust marks.  This will change the following entries!
		mark_adjust(lnum, lnum + count - 1, (long)MAXLNUM, (long)added);
		if (curwin->w_cursor.lnum >= lnum)
		{
		    // Adjust the cursor position if it's in/after the changed
		    // lines.
		    if (curwin->w_cursor.lnum >= lnum + count)
			curwin->w_cursor.lnum += added;
		    else if (added < 0)
			curwin->w_cursor.lnum = lnum;
		}
	    }
	    changed_lines(lnum, 0, lnum + count, (long)added);

	    if (dfree != NULL)
	    {
		// Diff is deleted, update folds in other windows.
#ifdef FEAT_FOLDING
		diff_fold_update(dfree, idx_to);
#endif
		vim_free(dfree);
	    }

	    // mark_adjust() may have made "dp" invalid.  We don't know where
	    // to continue then, bail out.
	    if (added != 0 && !valid_diff(dp))
		break;

	    if (dfree == NULL)
		// mark_adjust() may have changed the count in a wrong way
		dp->df_count[idx_to] = new_count;

	    // When changing the current buffer, keep track of line numbers
	    if (idx_cur == idx_to)
		off += added;
	}

	// If before the range or not deleted, go to next diff.
	if (dfree == NULL)
	{
	    dprev = dp;
	    dp = dp->df_next;
	}
    }

    // restore curwin/curbuf and a few other things
    if (eap->cmdidx != CMD_diffget)
    {
	// Syncing undo only works for the current buffer, but we change
	// another buffer.  Sync undo if the command was typed.  This isn't
	// 100% right when ":diffput" is used in a function or mapping.
	if (KeyTyped)
	    u_sync(FALSE);
	aucmd_restbuf(&aco);
    }

theend:
    diff_busy = FALSE;
    if (diff_need_update)
	ex_diffupdate(NULL);

    // Check that the cursor is on a valid character and update its
    // position.  When there were filler lines the topline has become
    // invalid.
    check_cursor();
    changed_line_abv_curs();

    if (diff_need_update)
	// redraw already done by ex_diffupdate()
	diff_need_update = FALSE;
    else
    {
	// Also need to redraw the other buffers.
	diff_redraw(FALSE);
	apply_autocmds(EVENT_DIFFUPDATED, NULL, NULL, FALSE, curbuf);
    }
}

#ifdef FEAT_FOLDING
/*
 * Update folds for all diff buffers for entry "dp".
 * Skip buffer with index "skip_idx".
 * When there are no diffs, all folds are removed.
 */
    static void
diff_fold_update(diff_T *dp, int skip_idx)
{
    int		i;
    win_T	*wp;

    FOR_ALL_WINDOWS(wp)
	for (i = 0; i < DB_COUNT; ++i)
	    if (curtab->tp_diffbuf[i] == wp->w_buffer && i != skip_idx)
		foldUpdate(wp, dp->df_lnum[i],
					    dp->df_lnum[i] + dp->df_count[i]);
}
#endif

/*
 * Return TRUE if buffer "buf" is in diff-mode.
 */
    int
diff_mode_buf(buf_T *buf)
{
    tabpage_T	*tp;

    FOR_ALL_TABPAGES(tp)
	if (diff_buf_idx_tp(buf, tp) != DB_COUNT)
	    return TRUE;
    return FALSE;
}

/*
 * Move "count" times in direction "dir" to the next diff block.
 * Return FAIL if there isn't such a diff block.
 */
    int
diff_move_to(int dir, long count)
{
    int		idx;
    linenr_T	lnum = curwin->w_cursor.lnum;
    diff_T	*dp;

    idx = diff_buf_idx(curbuf);
    if (idx == DB_COUNT || curtab->tp_first_diff == NULL)
	return FAIL;

    if (curtab->tp_diff_invalid)
	ex_diffupdate(NULL);		// update after a big change

    if (curtab->tp_first_diff == NULL)		// no diffs today
	return FAIL;

    while (--count >= 0)
    {
	// Check if already before first diff.
	if (dir == BACKWARD && lnum <= curtab->tp_first_diff->df_lnum[idx])
	    break;

	for (dp = curtab->tp_first_diff; ; dp = dp->df_next)
	{
	    if (dp == NULL)
		break;
	    if ((dir == FORWARD && lnum < dp->df_lnum[idx])
		    || (dir == BACKWARD
			&& (dp->df_next == NULL
			    || lnum <= dp->df_next->df_lnum[idx])))
	    {
		lnum = dp->df_lnum[idx];
		break;
	    }
	}
    }

    // don't end up past the end of the file
    if (lnum > curbuf->b_ml.ml_line_count)
	lnum = curbuf->b_ml.ml_line_count;

    // When the cursor didn't move at all we fail.
    if (lnum == curwin->w_cursor.lnum)
	return FAIL;

    setpcmark();
    curwin->w_cursor.lnum = lnum;
    curwin->w_cursor.col = 0;

    return OK;
}

/*
 * Return the line number in the current window that is closest to "lnum1" in
 * "buf1" in diff mode.
 */
    static linenr_T
diff_get_corresponding_line_int(
    buf_T	*buf1,
    linenr_T	lnum1)
{
    int		idx1;
    int		idx2;
    diff_T	*dp;
    int		baseline = 0;

    idx1 = diff_buf_idx(buf1);
    idx2 = diff_buf_idx(curbuf);
    if (idx1 == DB_COUNT || idx2 == DB_COUNT || curtab->tp_first_diff == NULL)
	return lnum1;

    if (curtab->tp_diff_invalid)
	ex_diffupdate(NULL);		// update after a big change

    if (curtab->tp_first_diff == NULL)		// no diffs today
	return lnum1;

    FOR_ALL_DIFFBLOCKS_IN_TAB(curtab, dp)
    {
	if (dp->df_lnum[idx1] > lnum1)
	    return lnum1 - baseline;
	if ((dp->df_lnum[idx1] + dp->df_count[idx1]) > lnum1)
	{
	    // Inside the diffblock
	    baseline = lnum1 - dp->df_lnum[idx1];
	    if (baseline > dp->df_count[idx2])
		baseline = dp->df_count[idx2];

	    return dp->df_lnum[idx2] + baseline;
	}
	if (    (dp->df_lnum[idx1] == lnum1)
	     && (dp->df_count[idx1] == 0)
	     && (dp->df_lnum[idx2] <= curwin->w_cursor.lnum)
	     && ((dp->df_lnum[idx2] + dp->df_count[idx2])
						      > curwin->w_cursor.lnum))
	    /*
	     * Special case: if the cursor is just after a zero-count
	     * block (i.e. all filler) and the target cursor is already
	     * inside the corresponding block, leave the target cursor
	     * unmoved. This makes repeated CTRL-W W operations work
	     * as expected.
	     */
	    return curwin->w_cursor.lnum;
	baseline = (dp->df_lnum[idx1] + dp->df_count[idx1])
				   - (dp->df_lnum[idx2] + dp->df_count[idx2]);
    }

    // If we get here then the cursor is after the last diff
    return lnum1 - baseline;
}

/*
 * Return the line number in the current window that is closest to "lnum1" in
 * "buf1" in diff mode.  Checks the line number to be valid.
 */
    linenr_T
diff_get_corresponding_line(buf_T *buf1, linenr_T lnum1)
{
    linenr_T lnum = diff_get_corresponding_line_int(buf1, lnum1);

    // don't end up past the end of the file
    if (lnum > curbuf->b_ml.ml_line_count)
	return curbuf->b_ml.ml_line_count;
    return lnum;
}

/*
 * For line "lnum" in the current window find the equivalent lnum in window
 * "wp", compensating for inserted/deleted lines.
 */
    linenr_T
diff_lnum_win(linenr_T lnum, win_T *wp)
{
    diff_T	*dp;
    int		idx;
    int		i;
    linenr_T	n;

    idx = diff_buf_idx(curbuf);
    if (idx == DB_COUNT)		// safety check
	return (linenr_T)0;

    if (curtab->tp_diff_invalid)
	ex_diffupdate(NULL);		// update after a big change

    // search for a change that includes "lnum" in the list of diffblocks.
    FOR_ALL_DIFFBLOCKS_IN_TAB(curtab, dp)
	if (lnum <= dp->df_lnum[idx] + dp->df_count[idx])
	    break;

    // When after the last change, compute relative to the last line number.
    if (dp == NULL)
	return wp->w_buffer->b_ml.ml_line_count
					- (curbuf->b_ml.ml_line_count - lnum);

    // Find index for "wp".
    i = diff_buf_idx(wp->w_buffer);
    if (i == DB_COUNT)			// safety check
	return (linenr_T)0;

    n = lnum + (dp->df_lnum[i] - dp->df_lnum[idx]);
    if (n > dp->df_lnum[i] + dp->df_count[i])
	n = dp->df_lnum[i] + dp->df_count[i];
    return n;
}

/*
 * Handle an ED style diff line.
 * Return FAIL if the line does not contain diff info.
 */
    static int
parse_diff_ed(
	char_u	    *line,
	diffhunk_T  *hunk)
{
    char_u *p;
    long    f1, l1, f2, l2;
    int	    difftype;

    // The line must be one of three formats:
    // change: {first}[,{last}]c{first}[,{last}]
    // append: {first}a{first}[,{last}]
    // delete: {first}[,{last}]d{first}
    p = line;
    f1 = getdigits(&p);
    if (*p == ',')
    {
	++p;
	l1 = getdigits(&p);
    }
    else
	l1 = f1;
    if (*p != 'a' && *p != 'c' && *p != 'd')
	return FAIL;		// invalid diff format
    difftype = *p++;
    f2 = getdigits(&p);
    if (*p == ',')
    {
	++p;
	l2 = getdigits(&p);
    }
    else
	l2 = f2;
    if (l1 < f1 || l2 < f2)
	return FAIL;

    if (difftype == 'a')
    {
	hunk->lnum_orig = f1 + 1;
	hunk->count_orig = 0;
    }
    else
    {
	hunk->lnum_orig = f1;
	hunk->count_orig = l1 - f1 + 1;
    }
    if (difftype == 'd')
    {
	hunk->lnum_new = f2 + 1;
	hunk->count_new = 0;
    }
    else
    {
	hunk->lnum_new = f2;
	hunk->count_new = l2 - f2 + 1;
    }
    return OK;
}

/*
 * Parses unified diff with zero(!) context lines.
 * Return FAIL if there is no diff information in "line".
 */
    static int
parse_diff_unified(
	char_u	    *line,
	diffhunk_T  *hunk)
{
    char_u *p;
    long    oldline, oldcount, newline, newcount;

    // Parse unified diff hunk header:
    // @@ -oldline,oldcount +newline,newcount @@
    p = line;
    if (*p++ == '@' && *p++ == '@' && *p++ == ' ' && *p++ == '-')
    {
	oldline = getdigits(&p);
	if (*p == ',')
	{
	    ++p;
	    oldcount = getdigits(&p);
	}
	else
	    oldcount = 1;
	if (*p++ == ' ' && *p++ == '+')
	{
	    newline = getdigits(&p);
	    if (*p == ',')
	    {
		++p;
		newcount = getdigits(&p);
	    }
	    else
		newcount = 1;
	}
	else
	    return FAIL;	// invalid diff format

	if (oldcount == 0)
	    oldline += 1;
	if (newcount == 0)
	    newline += 1;
	if (newline == 0)
	    newline = 1;

	hunk->lnum_orig = oldline;
	hunk->count_orig = oldcount;
	hunk->lnum_new = newline;
	hunk->count_new = newcount;

	return OK;
    }

    return FAIL;
}

/*
 * Callback function for the xdl_diff() function.
 * Stores the diff output in a grow array.
 */
    static int
xdiff_out(
	long start_a,
	long count_a,
	long start_b,
	long count_b,
	void *priv)
{
    diffout_T	*dout = (diffout_T *)priv;
    diffhunk_T *p = ALLOC_ONE(diffhunk_T);

    if (p == NULL)
	return -1;

    if (ga_grow(&dout->dout_ga, 1) == FAIL)
    {
	vim_free(p);
	return -1;
    }

    p->lnum_orig  = start_a + 1;
    p->count_orig = count_a;
    p->lnum_new   = start_b + 1;
    p->count_new  = count_b;
    ((diffhunk_T **)dout->dout_ga.ga_data)[dout->dout_ga.ga_len++] = p;
    return 0;
}

#endif	// FEAT_DIFF

#if defined(FEAT_EVAL) || defined(PROTO)

/*
 * "diff_filler()" function
 */
    void
f_diff_filler(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
#ifdef FEAT_DIFF
    if (in_vim9script() && check_for_lnum_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_number = diff_check_fill(curwin, tv_get_lnum(argvars));
#endif
}

/*
 * "diff_hlID()" function
 */
    void
f_diff_hlID(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
#ifdef FEAT_DIFF
    linenr_T		lnum;
    static linenr_T	prev_lnum = 0;
    static varnumber_T	changedtick = 0;
    static int		fnum = 0;
    static int		change_start = 0;
    static int		change_end = 0;
    static hlf_T	hlID = (hlf_T)0;
    int			filler_lines;
    int			col;

    if (in_vim9script()
	    && (check_for_lnum_arg(argvars,0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    lnum = tv_get_lnum(argvars);
    if (lnum < 0)	// ignore type error in {lnum} arg
	lnum = 0;
    if (lnum != prev_lnum
	    || changedtick != CHANGEDTICK(curbuf)
	    || fnum != curbuf->b_fnum)
    {
	// New line, buffer, change: need to get the values.
	filler_lines = diff_check(curwin, lnum);
	if (filler_lines < 0)
	{
	    if (filler_lines == -1)
	    {
		change_start = MAXCOL;
		change_end = -1;
		if (diff_find_change(curwin, lnum, &change_start, &change_end))
		    hlID = HLF_ADD;	// added line
		else
		    hlID = HLF_CHD;	// changed line
	    }
	    else
		hlID = HLF_ADD;	// added line
	}
	else
	    hlID = (hlf_T)0;
	prev_lnum = lnum;
	changedtick = CHANGEDTICK(curbuf);
	fnum = curbuf->b_fnum;
    }

    if (hlID == HLF_CHD || hlID == HLF_TXD)
    {
	col = tv_get_number(&argvars[1]) - 1; // ignore type error in {col}
	if (col >= change_start && col <= change_end)
	    hlID = HLF_TXD;			// changed text
	else
	    hlID = HLF_CHD;			// changed line
    }
    rettv->vval.v_number = hlID == (hlf_T)0 ? 0 : (int)hlID;
#endif
}

#endif
