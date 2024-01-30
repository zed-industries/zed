/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * undo.c: multi level undo facility
 *
 * The saved lines are stored in a list of lists (one for each buffer):
 *
 * b_u_oldhead------------------------------------------------+
 *							      |
 *							      V
 *		  +--------------+    +--------------+	  +--------------+
 * b_u_newhead--->| u_header	 |    | u_header     |	  | u_header	 |
 *		  |	uh_next------>|     uh_next------>|	uh_next---->NULL
 *	   NULL<--------uh_prev  |<---------uh_prev  |<---------uh_prev  |
 *		  |	uh_entry |    |     uh_entry |	  |	uh_entry |
 *		  +--------|-----+    +--------|-----+	  +--------|-----+
 *			   |		       |		   |
 *			   V		       V		   V
 *		  +--------------+    +--------------+	  +--------------+
 *		  | u_entry	 |    | u_entry      |	  | u_entry	 |
 *		  |	ue_next  |    |     ue_next  |	  |	ue_next  |
 *		  +--------|-----+    +--------|-----+	  +--------|-----+
 *			   |		       |		   |
 *			   V		       V		   V
 *		  +--------------+	      NULL		  NULL
 *		  | u_entry	 |
 *		  |	ue_next  |
 *		  +--------|-----+
 *			   |
 *			   V
 *			  etc.
 *
 * Each u_entry list contains the information for one undo or redo.
 * curbuf->b_u_curhead points to the header of the last undo (the next redo),
 * or is NULL if nothing has been undone (end of the branch).
 *
 * For keeping alternate undo/redo branches the uh_alt field is used.  Thus at
 * each point in the list a branch may appear for an alternate to redo.  The
 * uh_seq field is numbered sequentially to be able to find a newer or older
 * branch.
 *
 *		   +---------------+	+---------------+
 * b_u_oldhead --->| u_header	   |	| u_header	|
 *		   |   uh_alt_next ---->|   uh_alt_next ----> NULL
 *	   NULL <----- uh_alt_prev |<------ uh_alt_prev |
 *		   |   uh_prev	   |	|   uh_prev	|
 *		   +-----|---------+	+-----|---------+
 *			 |		      |
 *			 V		      V
 *		   +---------------+	+---------------+
 *		   | u_header	   |	| u_header	|
 *		   |   uh_alt_next |	|   uh_alt_next |
 * b_u_newhead --->|   uh_alt_prev |	|   uh_alt_prev |
 *		   |   uh_prev	   |	|   uh_prev	|
 *		   +-----|---------+	+-----|---------+
 *			 |		      |
 *			 V		      V
 *		       NULL		+---------------+    +---------------+
 *					| u_header	|    | u_header      |
 *					|   uh_alt_next ---->|	 uh_alt_next |
 *					|   uh_alt_prev |<------ uh_alt_prev |
 *					|   uh_prev	|    |	 uh_prev     |
 *					+-----|---------+    +-----|---------+
 *					      |			   |
 *					     etc.		  etc.
 *
 *
 * All data is allocated and will all be freed when the buffer is unloaded.
 */

// Uncomment the next line for including the u_check() function.  This warns
// for errors in the debug information.
// #define U_DEBUG 1
#define UH_MAGIC 0x18dade	// value for uh_magic when in use
#define UE_MAGIC 0xabc123	// value for ue_magic when in use

// Size of buffer used for encryption.
#define CRYPT_BUF_SIZE 8192

#include "vim.h"

// Structure passed around between functions.
// Avoids passing cryptstate_T when encryption not available.
typedef struct {
    buf_T	*bi_buf;
    FILE	*bi_fp;
#ifdef FEAT_CRYPT
    cryptstate_T *bi_state;
    char_u	*bi_buffer; // CRYPT_BUF_SIZE, NULL when not buffering
    size_t	bi_used;    // bytes written to/read from bi_buffer
    size_t	bi_avail;   // bytes available in bi_buffer
#endif
} bufinfo_T;


static void u_unch_branch(u_header_T *uhp);
static u_entry_T *u_get_headentry(void);
static void u_getbot(void);
static void u_doit(int count);
static void u_undoredo(int undo);
static void u_undo_end(int did_undo, int absolute);
static void u_freeheader(buf_T *buf, u_header_T *uhp, u_header_T **uhpp);
static void u_freebranch(buf_T *buf, u_header_T *uhp, u_header_T **uhpp);
static void u_freeentries(buf_T *buf, u_header_T *uhp, u_header_T **uhpp);
static void u_freeentry(u_entry_T *, long);
#ifdef FEAT_PERSISTENT_UNDO
# ifdef FEAT_CRYPT
static int undo_flush(bufinfo_T *bi);
# endif
static int undo_read(bufinfo_T *bi, char_u *buffer, size_t size);
static int serialize_uep(bufinfo_T *bi, u_entry_T *uep);
static u_entry_T *unserialize_uep(bufinfo_T *bi, int *error, char_u *file_name);
static void serialize_pos(bufinfo_T *bi, pos_T pos);
static void unserialize_pos(bufinfo_T *bi, pos_T *pos);
static void serialize_visualinfo(bufinfo_T *bi, visualinfo_T *info);
static void unserialize_visualinfo(bufinfo_T *bi, visualinfo_T *info);
#endif
static void u_saveline(linenr_T lnum);

#define U_ALLOC_LINE(size) lalloc(size, FALSE)

// used in undo_end() to report number of added and deleted lines
static long	u_newcount, u_oldcount;

/*
 * When 'u' flag included in 'cpoptions', we behave like vi.  Need to remember
 * the action that "u" should do.
 */
static int	undo_undoes = FALSE;

static int	lastmark = 0;

#if defined(U_DEBUG) || defined(PROTO)
/*
 * Check the undo structures for being valid.  Print a warning when something
 * looks wrong.
 */
static int seen_b_u_curhead;
static int seen_b_u_newhead;
static int header_count;

    static void
u_check_tree(u_header_T *uhp,
	u_header_T *exp_uh_next,
	u_header_T *exp_uh_alt_prev)
{
    u_entry_T *uep;

    if (uhp == NULL)
	return;
    ++header_count;
    if (uhp == curbuf->b_u_curhead && ++seen_b_u_curhead > 1)
    {
	emsg("b_u_curhead found twice (looping?)");
	return;
    }
    if (uhp == curbuf->b_u_newhead && ++seen_b_u_newhead > 1)
    {
	emsg("b_u_newhead found twice (looping?)");
	return;
    }

    if (uhp->uh_magic != UH_MAGIC)
	emsg("uh_magic wrong (may be using freed memory)");
    else
    {
	// Check pointers back are correct.
	if (uhp->uh_next.ptr != exp_uh_next)
	{
	    emsg("uh_next wrong");
	    smsg("expected: 0x%x, actual: 0x%x",
					       exp_uh_next, uhp->uh_next.ptr);
	}
	if (uhp->uh_alt_prev.ptr != exp_uh_alt_prev)
	{
	    emsg("uh_alt_prev wrong");
	    smsg("expected: 0x%x, actual: 0x%x",
				       exp_uh_alt_prev, uhp->uh_alt_prev.ptr);
	}

	// Check the undo tree at this header.
	for (uep = uhp->uh_entry; uep != NULL; uep = uep->ue_next)
	{
	    if (uep->ue_magic != UE_MAGIC)
	    {
		emsg("ue_magic wrong (may be using freed memory)");
		break;
	    }
	}

	// Check the next alt tree.
	u_check_tree(uhp->uh_alt_next.ptr, uhp->uh_next.ptr, uhp);

	// Check the next header in this branch.
	u_check_tree(uhp->uh_prev.ptr, uhp, NULL);
    }
}

    static void
u_check(int newhead_may_be_NULL)
{
    seen_b_u_newhead = 0;
    seen_b_u_curhead = 0;
    header_count = 0;

    u_check_tree(curbuf->b_u_oldhead, NULL, NULL);

    if (seen_b_u_newhead == 0 && curbuf->b_u_oldhead != NULL
	    && !(newhead_may_be_NULL && curbuf->b_u_newhead == NULL))
	semsg("b_u_newhead invalid: 0x%x", curbuf->b_u_newhead);
    if (curbuf->b_u_curhead != NULL && seen_b_u_curhead == 0)
	semsg("b_u_curhead invalid: 0x%x", curbuf->b_u_curhead);
    if (header_count != curbuf->b_u_numhead)
    {
	emsg("b_u_numhead invalid");
	smsg("expected: %ld, actual: %ld",
			       (long)header_count, (long)curbuf->b_u_numhead);
    }
}
#endif

/*
 * Save the current line for both the "u" and "U" command.
 * Careful: may trigger autocommands that reload the buffer.
 * Returns OK or FAIL.
 */
    int
u_save_cursor(void)
{
    return (u_save((linenr_T)(curwin->w_cursor.lnum - 1),
				      (linenr_T)(curwin->w_cursor.lnum + 1)));
}

/*
 * Save the lines between "top" and "bot" for both the "u" and "U" command.
 * "top" may be 0 and "bot" may be curbuf->b_ml.ml_line_count + 1.
 * Careful: may trigger autocommands that reload the buffer.
 * Returns FAIL when lines could not be saved, OK otherwise.
 */
    int
u_save(linenr_T top, linenr_T bot)
{
    if (undo_off)
	return OK;

    if (top >= bot || bot > curbuf->b_ml.ml_line_count + 1)
	return FAIL;	// rely on caller to give an error message

    if (top + 2 == bot)
	u_saveline((linenr_T)(top + 1));

    return (u_savecommon(top, bot, (linenr_T)0, FALSE));
}

/*
 * Save the line "lnum" (used by ":s" and "~" command).
 * The line is replaced, so the new bottom line is lnum + 1.
 * Careful: may trigger autocommands that reload the buffer.
 * Returns FAIL when lines could not be saved, OK otherwise.
 */
    int
u_savesub(linenr_T lnum)
{
    if (undo_off)
	return OK;

    return (u_savecommon(lnum - 1, lnum + 1, lnum + 1, FALSE));
}

/*
 * A new line is inserted before line "lnum" (used by :s command).
 * The line is inserted, so the new bottom line is lnum + 1.
 * Careful: may trigger autocommands that reload the buffer.
 * Returns FAIL when lines could not be saved, OK otherwise.
 */
    int
u_inssub(linenr_T lnum)
{
    if (undo_off)
	return OK;

    return (u_savecommon(lnum - 1, lnum, lnum + 1, FALSE));
}

/*
 * Save the lines "lnum" - "lnum" + nlines (used by delete command).
 * The lines are deleted, so the new bottom line is lnum, unless the buffer
 * becomes empty.
 * Careful: may trigger autocommands that reload the buffer.
 * Returns FAIL when lines could not be saved, OK otherwise.
 */
    int
u_savedel(linenr_T lnum, long nlines)
{
    if (undo_off)
	return OK;

    return (u_savecommon(lnum - 1, lnum + nlines,
		     nlines == curbuf->b_ml.ml_line_count ? 2 : lnum, FALSE));
}

/*
 * Return TRUE when undo is allowed.  Otherwise give an error message and
 * return FALSE.
 */
    int
undo_allowed(void)
{
    // Don't allow changes when 'modifiable' is off.
    if (!curbuf->b_p_ma)
    {
	emsg(_(e_cannot_make_changes_modifiable_is_off));
	return FALSE;
    }

#ifdef HAVE_SANDBOX
    // In the sandbox it's not allowed to change the text.
    if (sandbox != 0)
    {
	emsg(_(e_not_allowed_in_sandbox));
	return FALSE;
    }
#endif

    // Don't allow changes in the buffer while editing the cmdline.  The
    // caller of getcmdline() may get confused.
    if (textlock != 0)
    {
	emsg(_(e_not_allowed_to_change_text_or_change_window));
	return FALSE;
    }

    return TRUE;
}

/*
 * Get the undolevel value for the current buffer.
 */
    static long
get_undolevel(void)
{
    if (curbuf->b_p_ul == NO_LOCAL_UNDOLEVEL)
	return p_ul;
    return curbuf->b_p_ul;
}

/*
 * u_save_line(): save an allocated copy of line "lnum" into "ul".
 * Returns FAIL when out of memory.
 */
    static int
u_save_line(undoline_T *ul, linenr_T lnum)
{
    char_u *line = ml_get(lnum);

    if (curbuf->b_ml.ml_line_len == 0)
    {
	ul->ul_len = 1;
	ul->ul_line = vim_strsave((char_u *)"");
    }
    else
    {
	// This uses the length in the memline, thus text properties are
	// included.
	ul->ul_len = curbuf->b_ml.ml_line_len;
	ul->ul_line = vim_memsave(line, ul->ul_len);
    }
    return ul->ul_line == NULL ? FAIL : OK;
}

#ifdef FEAT_PROP_POPUP
/*
 * return TRUE if line "lnum" has text property "flags".
 */
    static int
has_prop_w_flags(linenr_T lnum, int flags)
{
    char_u  *props;
    int	    i;
    int	    proplen = get_text_props(curbuf, lnum, &props, FALSE);

    for (i = 0; i < proplen; ++i)
    {
	textprop_T prop;

	mch_memmove(&prop, props + i * sizeof prop, sizeof prop);
	if (prop.tp_flags & flags)
	    return TRUE;
    }
    return FALSE;
}
#endif

/*
 * Common code for various ways to save text before a change.
 * "top" is the line above the first changed line.
 * "bot" is the line below the last changed line.
 * "newbot" is the new bottom line.  Use zero when not known.
 * "reload" is TRUE when saving for a buffer reload.
 * Careful: may trigger autocommands that reload the buffer.
 * Returns FAIL when lines could not be saved, OK otherwise.
 */
    int
u_savecommon(
    linenr_T	top,
    linenr_T	bot,
    linenr_T	newbot,
    int		reload)
{
    linenr_T	lnum;
    long	i;
    u_header_T	*uhp;
    u_header_T	*old_curhead;
    u_entry_T	*uep;
    u_entry_T	*prev_uep;
    long	size;

    if (!reload)
    {
	// When making changes is not allowed return FAIL.  It's a crude way
	// to make all change commands fail.
	if (!undo_allowed())
	    return FAIL;

#ifdef FEAT_NETBEANS_INTG
	/*
	 * Netbeans defines areas that cannot be modified.  Bail out here when
	 * trying to change text in a guarded area.
	 */
	if (netbeans_active())
	{
	    if (netbeans_is_guarded(top, bot))
	    {
		emsg(_(e_region_is_guarded_cannot_modify));
		return FAIL;
	    }
	    if (curbuf->b_p_ro)
	    {
		emsg(_(e_netbeans_does_not_allow_changes_in_read_only_files));
		return FAIL;
	    }
	}
#endif
#ifdef FEAT_TERMINAL
	// A change in a terminal buffer removes the highlighting.
	term_change_in_curbuf();
#endif

	/*
	 * Saving text for undo means we are going to make a change.  Give a
	 * warning for a read-only file before making the change, so that the
	 * FileChangedRO event can replace the buffer with a read-write version
	 * (e.g., obtained from a source control system).
	 */
	change_warning(0);
	if (bot > curbuf->b_ml.ml_line_count + 1)
	{
	    // This happens when the FileChangedRO autocommand changes the
	    // file in a way it becomes shorter.
	    emsg(_(e_line_count_changed_unexpectedly));
	    return FAIL;
	}
    }

#ifdef U_DEBUG
    u_check(FALSE);
#endif

#ifdef FEAT_PROP_POPUP
    // Include the line above if a text property continues from it.
    // Include the line below if a text property continues to it.
    if (bot - top > 1)
    {
	if (top > 0 && has_prop_w_flags(top + 1, TP_FLAG_CONT_PREV))
	    --top;
	if (bot <= curbuf->b_ml.ml_line_count
			       && has_prop_w_flags(bot - 1, TP_FLAG_CONT_NEXT))
	{
	    ++bot;
	    if (newbot != 0)
		++newbot;
	}
    }
#endif

    size = bot - top - 1;

    /*
     * If curbuf->b_u_synced == TRUE make a new header.
     */
    if (curbuf->b_u_synced)
    {
	// Need to create new entry in b_changelist.
	curbuf->b_new_change = TRUE;

	if (get_undolevel() >= 0)
	{
	    /*
	     * Make a new header entry.  Do this first so that we don't mess
	     * up the undo info when out of memory.
	     */
	    uhp = U_ALLOC_LINE(sizeof(u_header_T));
	    if (uhp == NULL)
		goto nomem;
#ifdef U_DEBUG
	    uhp->uh_magic = UH_MAGIC;
#endif
	}
	else
	    uhp = NULL;

	/*
	 * If we undid more than we redid, move the entry lists before and
	 * including curbuf->b_u_curhead to an alternate branch.
	 */
	old_curhead = curbuf->b_u_curhead;
	if (old_curhead != NULL)
	{
	    curbuf->b_u_newhead = old_curhead->uh_next.ptr;
	    curbuf->b_u_curhead = NULL;
	}

	/*
	 * free headers to keep the size right
	 */
	while (curbuf->b_u_numhead > get_undolevel()
					       && curbuf->b_u_oldhead != NULL)
	{
	    u_header_T	    *uhfree = curbuf->b_u_oldhead;

	    if (uhfree == old_curhead)
		// Can't reconnect the branch, delete all of it.
		u_freebranch(curbuf, uhfree, &old_curhead);
	    else if (uhfree->uh_alt_next.ptr == NULL)
		// There is no branch, only free one header.
		u_freeheader(curbuf, uhfree, &old_curhead);
	    else
	    {
		// Free the oldest alternate branch as a whole.
		while (uhfree->uh_alt_next.ptr != NULL)
		    uhfree = uhfree->uh_alt_next.ptr;
		u_freebranch(curbuf, uhfree, &old_curhead);
	    }
#ifdef U_DEBUG
	    u_check(TRUE);
#endif
	}

	if (uhp == NULL)		// no undo at all
	{
	    if (old_curhead != NULL)
		u_freebranch(curbuf, old_curhead, NULL);
	    curbuf->b_u_synced = FALSE;
	    return OK;
	}

	uhp->uh_prev.ptr = NULL;
	uhp->uh_next.ptr = curbuf->b_u_newhead;
	uhp->uh_alt_next.ptr = old_curhead;
	if (old_curhead != NULL)
	{
	    uhp->uh_alt_prev.ptr = old_curhead->uh_alt_prev.ptr;
	    if (uhp->uh_alt_prev.ptr != NULL)
		uhp->uh_alt_prev.ptr->uh_alt_next.ptr = uhp;
	    old_curhead->uh_alt_prev.ptr = uhp;
	    if (curbuf->b_u_oldhead == old_curhead)
		curbuf->b_u_oldhead = uhp;
	}
	else
	    uhp->uh_alt_prev.ptr = NULL;
	if (curbuf->b_u_newhead != NULL)
	    curbuf->b_u_newhead->uh_prev.ptr = uhp;

	uhp->uh_seq = ++curbuf->b_u_seq_last;
	curbuf->b_u_seq_cur = uhp->uh_seq;
	uhp->uh_time = vim_time();
	uhp->uh_save_nr = 0;
	curbuf->b_u_time_cur = uhp->uh_time + 1;

	uhp->uh_walk = 0;
	uhp->uh_entry = NULL;
	uhp->uh_getbot_entry = NULL;
	uhp->uh_cursor = curwin->w_cursor;	// save cursor pos. for undo
	if (virtual_active() && curwin->w_cursor.coladd > 0)
	    uhp->uh_cursor_vcol = getviscol();
	else
	    uhp->uh_cursor_vcol = -1;

	// save changed and buffer empty flag for undo
	uhp->uh_flags = (curbuf->b_changed ? UH_CHANGED : 0) +
		       ((curbuf->b_ml.ml_flags & ML_EMPTY) ? UH_EMPTYBUF : 0);

	// save named marks and Visual marks for undo
	mch_memmove(uhp->uh_namedm, curbuf->b_namedm, sizeof(pos_T) * NMARKS);
	uhp->uh_visual = curbuf->b_visual;

	curbuf->b_u_newhead = uhp;
	if (curbuf->b_u_oldhead == NULL)
	    curbuf->b_u_oldhead = uhp;
	++curbuf->b_u_numhead;
    }
    else
    {
	if (get_undolevel() < 0)	// no undo at all
	    return OK;

	/*
	 * When saving a single line, and it has been saved just before, it
	 * doesn't make sense saving it again.  Saves a lot of memory when
	 * making lots of changes inside the same line.
	 * This is only possible if the previous change didn't increase or
	 * decrease the number of lines.
	 * Check the ten last changes.  More doesn't make sense and takes too
	 * long.
	 */
	if (size == 1)
	{
	    uep = u_get_headentry();
	    prev_uep = NULL;
	    for (i = 0; i < 10; ++i)
	    {
		if (uep == NULL)
		    break;

		// If lines have been inserted/deleted we give up.
		// Also when the line was included in a multi-line save.
		if ((curbuf->b_u_newhead->uh_getbot_entry != uep
			    ? (uep->ue_top + uep->ue_size + 1
				!= (uep->ue_bot == 0
				    ? curbuf->b_ml.ml_line_count + 1
				    : uep->ue_bot))
			    : uep->ue_lcount != curbuf->b_ml.ml_line_count)
			|| (uep->ue_size > 1
			    && top >= uep->ue_top
			    && top + 2 <= uep->ue_top + uep->ue_size + 1))
		    break;

		// If it's the same line we can skip saving it again.
		if (uep->ue_size == 1 && uep->ue_top == top)
		{
		    if (i > 0)
		    {
			// It's not the last entry: get ue_bot for the last
			// entry now.  Following deleted/inserted lines go to
			// the re-used entry.
			u_getbot();
			curbuf->b_u_synced = FALSE;

			// Move the found entry to become the last entry.  The
			// order of undo/redo doesn't matter for the entries
			// we move it over, since they don't change the line
			// count and don't include this line.  It does matter
			// for the found entry if the line count is changed by
			// the executed command.
			prev_uep->ue_next = uep->ue_next;
			uep->ue_next = curbuf->b_u_newhead->uh_entry;
			curbuf->b_u_newhead->uh_entry = uep;
		    }

		    // The executed command may change the line count.
		    if (newbot != 0)
			uep->ue_bot = newbot;
		    else if (bot > curbuf->b_ml.ml_line_count)
			uep->ue_bot = 0;
		    else
		    {
			uep->ue_lcount = curbuf->b_ml.ml_line_count;
			curbuf->b_u_newhead->uh_getbot_entry = uep;
		    }
		    return OK;
		}
		prev_uep = uep;
		uep = uep->ue_next;
	    }
	}

	// find line number for ue_bot for previous u_save()
	u_getbot();
    }

#if !defined(UNIX) && !defined(MSWIN)
	/*
	 * With Amiga we can't handle big undo's, because
	 * then u_alloc_line would have to allocate a block larger than 32K
	 */
    if (size >= 8000)
	goto nomem;
#endif

    /*
     * add lines in front of entry list
     */
    uep = U_ALLOC_LINE(sizeof(u_entry_T));
    if (uep == NULL)
	goto nomem;
    CLEAR_POINTER(uep);
#ifdef U_DEBUG
    uep->ue_magic = UE_MAGIC;
#endif

    uep->ue_size = size;
    uep->ue_top = top;
    if (newbot != 0)
	uep->ue_bot = newbot;
    /*
     * Use 0 for ue_bot if bot is below last line.
     * Otherwise we have to compute ue_bot later.
     */
    else if (bot > curbuf->b_ml.ml_line_count)
	uep->ue_bot = 0;
    else
    {
	uep->ue_lcount = curbuf->b_ml.ml_line_count;
	curbuf->b_u_newhead->uh_getbot_entry = uep;
    }

    if (size > 0)
    {
	if ((uep->ue_array = U_ALLOC_LINE(sizeof(undoline_T) * size)) == NULL)
	{
	    u_freeentry(uep, 0L);
	    goto nomem;
	}
	for (i = 0, lnum = top + 1; i < size; ++i)
	{
	    fast_breakcheck();
	    if (got_int)
	    {
		u_freeentry(uep, i);
		return FAIL;
	    }
	    if (u_save_line(&uep->ue_array[i], lnum++) == FAIL)
	    {
		u_freeentry(uep, i);
		goto nomem;
	    }
	}
    }
    else
	uep->ue_array = NULL;
    uep->ue_next = curbuf->b_u_newhead->uh_entry;
    curbuf->b_u_newhead->uh_entry = uep;
    curbuf->b_u_synced = FALSE;
    undo_undoes = FALSE;

#ifdef U_DEBUG
    u_check(FALSE);
#endif
    return OK;

nomem:
    msg_silent = 0;	// must display the prompt
    if (ask_yesno((char_u *)_("No undo possible; continue anyway"), TRUE)
								       == 'y')
    {
	undo_off = TRUE;	    // will be reset when character typed
	return OK;
    }
    do_outofmem_msg((long_u)0);
    return FAIL;
}

#if defined(FEAT_PERSISTENT_UNDO) || defined(PROTO)

# define UF_START_MAGIC	    "Vim\237UnDo\345"  // magic at start of undofile
# define UF_START_MAGIC_LEN	9
# define UF_HEADER_MAGIC	0x5fd0	// magic at start of header
# define UF_HEADER_END_MAGIC	0xe7aa	// magic after last header
# define UF_ENTRY_MAGIC		0xf518	// magic at start of entry
# define UF_ENTRY_END_MAGIC	0x3581	// magic after last entry
# define UF_VERSION		2	// 2-byte undofile version number
# define UF_VERSION_CRYPT	0x8002	// idem, encrypted

// extra fields for header
# define UF_LAST_SAVE_NR	1

// extra fields for uhp
# define UHP_SAVE_NR		1

/*
 * Compute the hash for the current buffer text into hash[UNDO_HASH_SIZE].
 */
    void
u_compute_hash(char_u *hash)
{
    context_sha256_T	ctx;
    linenr_T		lnum;
    char_u		*p;

    sha256_start(&ctx);
    for (lnum = 1; lnum <= curbuf->b_ml.ml_line_count; ++lnum)
    {
	p = ml_get(lnum);
	sha256_update(&ctx, p, (UINT32_T)(STRLEN(p) + 1));
    }
    sha256_finish(&ctx, hash);
}

/*
 * Return an allocated string of the full path of the target undofile.
 * When "reading" is TRUE find the file to read, go over all directories in
 * 'undodir'.
 * When "reading" is FALSE use the first name where the directory exists.
 * Returns NULL when there is no place to write or no file to read.
 */
    static char_u *
u_get_undo_file_name(char_u *buf_ffname, int reading)
{
    char_u	*dirp;
    char_u	dir_name[IOSIZE + 1];
    char_u	*munged_name = NULL;
    char_u	*undo_file_name = NULL;
    int		dir_len;
    char_u	*p;
    stat_T	st;
    char_u	*ffname = buf_ffname;
#ifdef HAVE_READLINK
    char_u	fname_buf[MAXPATHL];
#endif

    if (ffname == NULL)
	return NULL;

#ifdef HAVE_READLINK
    // Expand symlink in the file name, so that we put the undo file with the
    // actual file instead of with the symlink.
    if (resolve_symlink(ffname, fname_buf) == OK)
	ffname = fname_buf;
#endif

    // Loop over 'undodir'.  When reading find the first file that exists.
    // When not reading use the first directory that exists or ".".
    dirp = p_udir;
    while (*dirp != NUL)
    {
	dir_len = copy_option_part(&dirp, dir_name, IOSIZE, ",");
	if (dir_len == 1 && dir_name[0] == '.')
	{
	    // Use same directory as the ffname,
	    // "dir/name" -> "dir/.name.un~"
	    undo_file_name = vim_strnsave(ffname, STRLEN(ffname) + 5);
	    if (undo_file_name == NULL)
		break;
	    p = gettail(undo_file_name);
#ifdef VMS
	    // VMS can not handle more than one dot in the filenames
	    // use "dir/name" -> "dir/_un_name" - add _un_
	    // at the beginning to keep the extension
	    mch_memmove(p + 4,  p, STRLEN(p) + 1);
	    mch_memmove(p, "_un_", 4);

#else
	    // Use same directory as the ffname,
	    // "dir/name" -> "dir/.name.un~"
	    mch_memmove(p + 1, p, STRLEN(p) + 1);
	    *p = '.';
	    STRCAT(p, ".un~");
#endif
	}
	else
	{
	    dir_name[dir_len] = NUL;
	    if (mch_isdir(dir_name))
	    {
		if (munged_name == NULL)
		{
		    munged_name = vim_strsave(ffname);
		    if (munged_name == NULL)
			return NULL;
		    for (p = munged_name; *p != NUL; MB_PTR_ADV(p))
			if (vim_ispathsep(*p))
			    *p = '%';
		}
		undo_file_name = concat_fnames(dir_name, munged_name, TRUE);
	    }
	}

	// When reading check if the file exists.
	if (undo_file_name != NULL && (!reading
			       || mch_stat((char *)undo_file_name, &st) >= 0))
	    break;
	VIM_CLEAR(undo_file_name);
    }

    vim_free(munged_name);
    return undo_file_name;
}

    static void
corruption_error(char *mesg, char_u *file_name)
{
    semsg(_(e_corrupted_undo_file_str_str), mesg, file_name);
}

    static void
u_free_uhp(u_header_T *uhp)
{
    u_entry_T	*nuep;
    u_entry_T	*uep;

    uep = uhp->uh_entry;
    while (uep != NULL)
    {
	nuep = uep->ue_next;
	u_freeentry(uep, uep->ue_size);
	uep = nuep;
    }
    vim_free(uhp);
}

/*
 * Write a sequence of bytes to the undo file.
 * Buffers and encrypts as needed.
 * Returns OK or FAIL.
 */
    static int
undo_write(bufinfo_T *bi, char_u *ptr, size_t len)
{
#ifdef FEAT_CRYPT
    if (bi->bi_buffer != NULL)
    {
	size_t	len_todo = len;
	char_u  *p = ptr;

	while (bi->bi_used + len_todo >= CRYPT_BUF_SIZE)
	{
	    size_t	n = CRYPT_BUF_SIZE - bi->bi_used;

	    mch_memmove(bi->bi_buffer + bi->bi_used, p, n);
	    len_todo -= n;
	    p += n;
	    bi->bi_used = CRYPT_BUF_SIZE;
	    if (undo_flush(bi) == FAIL)
		return FAIL;
	}
	if (len_todo > 0)
	{
	    mch_memmove(bi->bi_buffer + bi->bi_used, p, len_todo);
	    bi->bi_used += len_todo;
	}
	return OK;
    }
#endif
    if (fwrite(ptr, len, (size_t)1, bi->bi_fp) != 1)
	return FAIL;
    return OK;
}

#ifdef FEAT_CRYPT
    static int
undo_flush(bufinfo_T *bi)
{
    if (bi->bi_buffer != NULL && bi->bi_state != NULL && bi->bi_used > 0)
    {
	// Last parameter is only used for sodium encryption and that
	// explicitly disables encryption of undofiles.
	crypt_encode_inplace(bi->bi_state, bi->bi_buffer, bi->bi_used, FALSE);
	if (fwrite(bi->bi_buffer, bi->bi_used, (size_t)1, bi->bi_fp) != 1)
	    return FAIL;
	bi->bi_used = 0;
    }
    return OK;
}
#endif

/*
 * Write "ptr[len]" and crypt the bytes when needed.
 * Returns OK or FAIL.
 */
    static int
fwrite_crypt(bufinfo_T *bi, char_u *ptr, size_t len)
{
#ifdef FEAT_CRYPT
    char_u  *copy;
    char_u  small_buf[100];
    size_t  i;

    if (bi->bi_state != NULL && bi->bi_buffer == NULL)
    {
	// crypting every piece of text separately
	if (len < 100)
	    copy = small_buf;  // no malloc()/free() for short strings
	else
	{
	    copy = lalloc(len, FALSE);
	    if (copy == NULL)
		return 0;
	}
	// Last parameter is only used for sodium encryption and that
	// explicitly disables encryption of undofiles.
	crypt_encode(bi->bi_state, ptr, len, copy, TRUE);
	i = fwrite(copy, len, (size_t)1, bi->bi_fp);
	if (copy != small_buf)
	    vim_free(copy);
	return i == 1 ? OK : FAIL;
    }
#endif
    return undo_write(bi, ptr, len);
}

/*
 * Write a number, MSB first, in "len" bytes.
 * Must match with undo_read_?c() functions.
 * Returns OK or FAIL.
 */
    static int
undo_write_bytes(bufinfo_T *bi, long_u nr, int len)
{
    char_u  buf[8];
    int	    i;
    int	    bufi = 0;

    for (i = len - 1; i >= 0; --i)
	buf[bufi++] = (char_u)(nr >> (i * 8));
    return undo_write(bi, buf, (size_t)len);
}

/*
 * Write the pointer to an undo header.  Instead of writing the pointer itself
 * we use the sequence number of the header.  This is converted back to
 * pointers when reading. */
    static void
put_header_ptr(bufinfo_T *bi, u_header_T *uhp)
{
    undo_write_bytes(bi, (long_u)(uhp != NULL ? uhp->uh_seq : 0), 4);
}

    static int
undo_read_4c(bufinfo_T *bi)
{
#ifdef FEAT_CRYPT
    if (bi->bi_buffer != NULL)
    {
	char_u  buf[4];
	int	n;

	undo_read(bi, buf, (size_t)4);
	n = ((unsigned)buf[0] << 24) + (buf[1] << 16) + (buf[2] << 8) + buf[3];
	return n;
    }
#endif
    return get4c(bi->bi_fp);
}

    static int
undo_read_2c(bufinfo_T *bi)
{
#ifdef FEAT_CRYPT
    if (bi->bi_buffer != NULL)
    {
	char_u  buf[2];
	int	n;

	undo_read(bi, buf, (size_t)2);
	n = (buf[0] << 8) + buf[1];
	return n;
    }
#endif
    return get2c(bi->bi_fp);
}

    static int
undo_read_byte(bufinfo_T *bi)
{
#ifdef FEAT_CRYPT
    if (bi->bi_buffer != NULL)
    {
	char_u  buf[1];

	undo_read(bi, buf, (size_t)1);
	return buf[0];
    }
#endif
    return getc(bi->bi_fp);
}

    static time_t
undo_read_time(bufinfo_T *bi)
{
#ifdef FEAT_CRYPT
    if (bi->bi_buffer != NULL)
    {
	char_u  buf[8];
	time_t	n = 0;
	int	i;

	undo_read(bi, buf, (size_t)8);
	for (i = 0; i < 8; ++i)
	    n = (n << 8) + buf[i];
	return n;
    }
#endif
    return get8ctime(bi->bi_fp);
}

/*
 * Read "buffer[size]" from the undo file.
 * Return OK or FAIL.
 */
    static int
undo_read(bufinfo_T *bi, char_u *buffer, size_t size)
{
    int retval = OK;

#ifdef FEAT_CRYPT
    if (bi->bi_buffer != NULL)
    {
	int	size_todo = (int)size;
	char_u	*p = buffer;

	while (size_todo > 0)
	{
	    size_t n;

	    if (bi->bi_used >= bi->bi_avail)
	    {
		n = fread(bi->bi_buffer, 1, (size_t)CRYPT_BUF_SIZE, bi->bi_fp);
		if (n == 0)
		{
		    retval = FAIL;
		    break;
		}
		bi->bi_avail = n;
		bi->bi_used = 0;
		crypt_decode_inplace(bi->bi_state, bi->bi_buffer, bi->bi_avail, FALSE);
	    }
	    n = size_todo;
	    if (n > bi->bi_avail - bi->bi_used)
		n = bi->bi_avail - bi->bi_used;
	    mch_memmove(p, bi->bi_buffer + bi->bi_used, n);
	    bi->bi_used += n;
	    size_todo -= (int)n;
	    p += n;
	}
    }
    else
#endif
    if (fread(buffer, size, 1, bi->bi_fp) != 1)
	retval = FAIL;

    if (retval == FAIL)
	// Error may be checked for only later.  Fill with zeros,
	// so that the reader won't use garbage.
	vim_memset(buffer, 0, size);
    return retval;
}

/*
 * Read a string of length "len" from "bi->bi_fd".
 * "len" can be zero to allocate an empty line.
 * Decrypt the bytes if needed.
 * Append a NUL.
 * Returns a pointer to allocated memory or NULL for failure.
 */
    static char_u *
read_string_decrypt(bufinfo_T *bi, int len)
{
    char_u  *ptr = alloc(len + 1);

    if (ptr == NULL)
	return NULL;

    if (len > 0 && undo_read(bi, ptr, len) == FAIL)
    {
	vim_free(ptr);
	return NULL;
    }
    // In case there are text properties there already is a NUL, but
    // checking for that is more expensive than just adding a dummy byte.
    ptr[len] = NUL;
#ifdef FEAT_CRYPT
    if (bi->bi_state != NULL && bi->bi_buffer == NULL)
	crypt_decode_inplace(bi->bi_state, ptr, len, FALSE);
#endif
    return ptr;
}

/*
 * Writes the (not encrypted) header and initializes encryption if needed.
 */
    static int
serialize_header(bufinfo_T *bi, char_u *hash)
{
    long	len;
    buf_T	*buf = bi->bi_buf;
    FILE	*fp = bi->bi_fp;
    char_u	time_buf[8];

    // Start writing, first the magic marker and undo info version.
    if (fwrite(UF_START_MAGIC, (size_t)UF_START_MAGIC_LEN, (size_t)1, fp) != 1)
	return FAIL;

    // If the buffer is encrypted then all text bytes following will be
    // encrypted.  Numbers and other info is not crypted.
#ifdef FEAT_CRYPT
    if (*buf->b_p_key != NUL)
    {
	char_u *header;
	int    header_len;

	undo_write_bytes(bi, (long_u)UF_VERSION_CRYPT, 2);
	bi->bi_state = crypt_create_for_writing(crypt_get_method_nr(buf),
					  buf->b_p_key, &header, &header_len);
	if (bi->bi_state == NULL)
	    return FAIL;
	len = (long)fwrite(header, (size_t)header_len, (size_t)1, fp);
	vim_free(header);
	if (len != 1)
	{
	    crypt_free_state(bi->bi_state);
	    bi->bi_state = NULL;
	    return FAIL;
	}

	if (crypt_whole_undofile(crypt_get_method_nr(buf)))
	{
	    bi->bi_buffer = alloc(CRYPT_BUF_SIZE);
	    if (bi->bi_buffer == NULL)
	    {
		crypt_free_state(bi->bi_state);
		bi->bi_state = NULL;
		return FAIL;
	    }
	    bi->bi_used = 0;
	}
    }
    else
#endif
	undo_write_bytes(bi, (long_u)UF_VERSION, 2);


    // Write a hash of the buffer text, so that we can verify it is still the
    // same when reading the buffer text.
    if (undo_write(bi, hash, (size_t)UNDO_HASH_SIZE) == FAIL)
	return FAIL;

    // buffer-specific data
    undo_write_bytes(bi, (long_u)buf->b_ml.ml_line_count, 4);
    len = buf->b_u_line_ptr.ul_line == NULL
				? 0L : (long)STRLEN(buf->b_u_line_ptr.ul_line);
    undo_write_bytes(bi, (long_u)len, 4);
    if (len > 0 && fwrite_crypt(bi, buf->b_u_line_ptr.ul_line, (size_t)len)
								       == FAIL)
	return FAIL;
    undo_write_bytes(bi, (long_u)buf->b_u_line_lnum, 4);
    undo_write_bytes(bi, (long_u)buf->b_u_line_colnr, 4);

    // Undo structures header data
    put_header_ptr(bi, buf->b_u_oldhead);
    put_header_ptr(bi, buf->b_u_newhead);
    put_header_ptr(bi, buf->b_u_curhead);

    undo_write_bytes(bi, (long_u)buf->b_u_numhead, 4);
    undo_write_bytes(bi, (long_u)buf->b_u_seq_last, 4);
    undo_write_bytes(bi, (long_u)buf->b_u_seq_cur, 4);
    time_to_bytes(buf->b_u_time_cur, time_buf);
    undo_write(bi, time_buf, 8);

    // Optional fields.
    undo_write_bytes(bi, 4, 1);
    undo_write_bytes(bi, UF_LAST_SAVE_NR, 1);
    undo_write_bytes(bi, (long_u)buf->b_u_save_nr_last, 4);

    undo_write_bytes(bi, 0, 1);  // end marker

    return OK;
}

    static int
serialize_uhp(bufinfo_T *bi, u_header_T *uhp)
{
    int		i;
    u_entry_T	*uep;
    char_u	time_buf[8];

    if (undo_write_bytes(bi, (long_u)UF_HEADER_MAGIC, 2) == FAIL)
	return FAIL;

    put_header_ptr(bi, uhp->uh_next.ptr);
    put_header_ptr(bi, uhp->uh_prev.ptr);
    put_header_ptr(bi, uhp->uh_alt_next.ptr);
    put_header_ptr(bi, uhp->uh_alt_prev.ptr);
    undo_write_bytes(bi, uhp->uh_seq, 4);
    serialize_pos(bi, uhp->uh_cursor);
    undo_write_bytes(bi, (long_u)uhp->uh_cursor_vcol, 4);
    undo_write_bytes(bi, (long_u)uhp->uh_flags, 2);
    // Assume NMARKS will stay the same.
    for (i = 0; i < NMARKS; ++i)
	serialize_pos(bi, uhp->uh_namedm[i]);
    serialize_visualinfo(bi, &uhp->uh_visual);
    time_to_bytes(uhp->uh_time, time_buf);
    undo_write(bi, time_buf, 8);

    // Optional fields.
    undo_write_bytes(bi, 4, 1);
    undo_write_bytes(bi, UHP_SAVE_NR, 1);
    undo_write_bytes(bi, (long_u)uhp->uh_save_nr, 4);

    undo_write_bytes(bi, 0, 1);  // end marker

    // Write all the entries.
    for (uep = uhp->uh_entry; uep != NULL; uep = uep->ue_next)
    {
	undo_write_bytes(bi, (long_u)UF_ENTRY_MAGIC, 2);
	if (serialize_uep(bi, uep) == FAIL)
	    return FAIL;
    }
    undo_write_bytes(bi, (long_u)UF_ENTRY_END_MAGIC, 2);
    return OK;
}

    static u_header_T *
unserialize_uhp(bufinfo_T *bi, char_u *file_name)
{
    u_header_T	*uhp;
    int		i;
    u_entry_T	*uep, *last_uep;
    int		c;
    int		error;

    uhp = U_ALLOC_LINE(sizeof(u_header_T));
    if (uhp == NULL)
	return NULL;
    CLEAR_POINTER(uhp);
#ifdef U_DEBUG
    uhp->uh_magic = UH_MAGIC;
#endif
    uhp->uh_next.seq = undo_read_4c(bi);
    uhp->uh_prev.seq = undo_read_4c(bi);
    uhp->uh_alt_next.seq = undo_read_4c(bi);
    uhp->uh_alt_prev.seq = undo_read_4c(bi);
    uhp->uh_seq = undo_read_4c(bi);
    if (uhp->uh_seq <= 0)
    {
	corruption_error("uh_seq", file_name);
	vim_free(uhp);
	return NULL;
    }
    unserialize_pos(bi, &uhp->uh_cursor);
    uhp->uh_cursor_vcol = undo_read_4c(bi);
    uhp->uh_flags = undo_read_2c(bi);
    for (i = 0; i < NMARKS; ++i)
	unserialize_pos(bi, &uhp->uh_namedm[i]);
    unserialize_visualinfo(bi, &uhp->uh_visual);
    uhp->uh_time = undo_read_time(bi);

    // Optional fields.
    for (;;)
    {
	int len = undo_read_byte(bi);
	int what;

	if (len == EOF)
	{
	    corruption_error("truncated", file_name);
	    u_free_uhp(uhp);
	    return NULL;
	}
	if (len == 0)
	    break;
	what = undo_read_byte(bi);
	switch (what)
	{
	    case UHP_SAVE_NR:
		uhp->uh_save_nr = undo_read_4c(bi);
		break;
	    default:
		// field not supported, skip
		while (--len >= 0)
		    (void)undo_read_byte(bi);
	}
    }

    // Unserialize the uep list.
    last_uep = NULL;
    while ((c = undo_read_2c(bi)) == UF_ENTRY_MAGIC)
    {
	error = FALSE;
	uep = unserialize_uep(bi, &error, file_name);
	if (last_uep == NULL)
	    uhp->uh_entry = uep;
	else
	    last_uep->ue_next = uep;
	last_uep = uep;
	if (uep == NULL || error)
	{
	    u_free_uhp(uhp);
	    return NULL;
	}
    }
    if (c != UF_ENTRY_END_MAGIC)
    {
	corruption_error("entry end", file_name);
	u_free_uhp(uhp);
	return NULL;
    }

    return uhp;
}

/*
 * Serialize "uep".
 */
    static int
serialize_uep(
    bufinfo_T	*bi,
    u_entry_T	*uep)
{
    int		i;
    size_t	len;

    undo_write_bytes(bi, (long_u)uep->ue_top, 4);
    undo_write_bytes(bi, (long_u)uep->ue_bot, 4);
    undo_write_bytes(bi, (long_u)uep->ue_lcount, 4);
    undo_write_bytes(bi, (long_u)uep->ue_size, 4);
    for (i = 0; i < uep->ue_size; ++i)
    {
	// Text is written without the text properties, since we cannot restore
	// the text property types.
	len = STRLEN(uep->ue_array[i].ul_line);
	if (undo_write_bytes(bi, (long_u)len, 4) == FAIL)
	    return FAIL;
	if (len > 0 && fwrite_crypt(bi, uep->ue_array[i].ul_line, len) == FAIL)
	    return FAIL;
    }
    return OK;
}

    static u_entry_T *
unserialize_uep(bufinfo_T *bi, int *error, char_u *file_name)
{
    int		i;
    u_entry_T	*uep;
    undoline_T	*array = NULL;
    char_u	*line;
    int		line_len;

    uep = U_ALLOC_LINE(sizeof(u_entry_T));
    if (uep == NULL)
	return NULL;
    CLEAR_POINTER(uep);
#ifdef U_DEBUG
    uep->ue_magic = UE_MAGIC;
#endif
    uep->ue_top = undo_read_4c(bi);
    uep->ue_bot = undo_read_4c(bi);
    uep->ue_lcount = undo_read_4c(bi);
    uep->ue_size = undo_read_4c(bi);
    if (uep->ue_size > 0)
    {
	if (uep->ue_size < LONG_MAX / (int)sizeof(char_u *))
	    array = U_ALLOC_LINE(sizeof(undoline_T) * uep->ue_size);
	if (array == NULL)
	{
	    *error = TRUE;
	    return uep;
	}
	vim_memset(array, 0, sizeof(undoline_T) * uep->ue_size);
    }
    uep->ue_array = array;

    for (i = 0; i < uep->ue_size; ++i)
    {
	line_len = undo_read_4c(bi);
	if (line_len >= 0)
	    line = read_string_decrypt(bi, line_len);
	else
	{
	    line = NULL;
	    corruption_error("line length", file_name);
	}
	if (line == NULL)
	{
	    *error = TRUE;
	    return uep;
	}
	array[i].ul_line = line;
	array[i].ul_len = line_len + 1;
    }
    return uep;
}

/*
 * Serialize "pos".
 */
    static void
serialize_pos(bufinfo_T *bi, pos_T pos)
{
    undo_write_bytes(bi, (long_u)pos.lnum, 4);
    undo_write_bytes(bi, (long_u)pos.col, 4);
    undo_write_bytes(bi, (long_u)pos.coladd, 4);
}

/*
 * Unserialize the pos_T at the current position.
 */
    static void
unserialize_pos(bufinfo_T *bi, pos_T *pos)
{
    pos->lnum = undo_read_4c(bi);
    if (pos->lnum < 0)
	pos->lnum = 0;
    pos->col = undo_read_4c(bi);
    if (pos->col < 0)
	pos->col = 0;
    pos->coladd = undo_read_4c(bi);
    if (pos->coladd < 0)
	pos->coladd = 0;
}

/*
 * Serialize "info".
 */
    static void
serialize_visualinfo(bufinfo_T *bi, visualinfo_T *info)
{
    serialize_pos(bi, info->vi_start);
    serialize_pos(bi, info->vi_end);
    undo_write_bytes(bi, (long_u)info->vi_mode, 4);
    undo_write_bytes(bi, (long_u)info->vi_curswant, 4);
}

/*
 * Unserialize the visualinfo_T at the current position.
 */
    static void
unserialize_visualinfo(bufinfo_T *bi, visualinfo_T *info)
{
    unserialize_pos(bi, &info->vi_start);
    unserialize_pos(bi, &info->vi_end);
    info->vi_mode = undo_read_4c(bi);
    info->vi_curswant = undo_read_4c(bi);
}

/*
 * Write the undo tree in an undo file.
 * When "name" is not NULL, use it as the name of the undo file.
 * Otherwise use buf->b_ffname to generate the undo file name.
 * "buf" must never be null, buf->b_ffname is used to obtain the original file
 * permissions.
 * "forceit" is TRUE for ":wundo!", FALSE otherwise.
 * "hash[UNDO_HASH_SIZE]" must be the hash value of the buffer text.
 */
    void
u_write_undo(
    char_u	*name,
    int		forceit,
    buf_T	*buf,
    char_u	*hash)
{
    u_header_T	*uhp;
    char_u	*file_name;
    int		mark;
#ifdef U_DEBUG
    int		headers_written = 0;
#endif
    int		fd;
    FILE	*fp = NULL;
    int		perm;
    int		write_ok = FALSE;
#ifdef UNIX
    int		st_old_valid = FALSE;
    stat_T	st_old;
    stat_T	st_new;
#endif
    bufinfo_T	bi;

    CLEAR_FIELD(bi);

    if (name == NULL)
    {
	file_name = u_get_undo_file_name(buf->b_ffname, FALSE);
	if (file_name == NULL)
	{
	    if (p_verbose > 0)
	    {
		verbose_enter();
		smsg(
		   _("Cannot write undo file in any directory in 'undodir'"));
		verbose_leave();
	    }
	    return;
	}
    }
    else
	file_name = name;

    /*
     * Decide about the permission to use for the undo file.  If the buffer
     * has a name use the permission of the original file.  Otherwise only
     * allow the user to access the undo file.
     */
    perm = 0600;
    if (buf->b_ffname != NULL)
    {
#ifdef UNIX
	if (mch_stat((char *)buf->b_ffname, &st_old) >= 0)
	{
	    perm = st_old.st_mode;
	    st_old_valid = TRUE;
	}
#else
	perm = mch_getperm(buf->b_ffname);
	if (perm < 0)
	    perm = 0600;
#endif
    }

    // strip any s-bit and executable bit
    perm = perm & 0666;

    // If the undo file already exists, verify that it actually is an undo
    // file, and delete it.
    if (mch_getperm(file_name) >= 0)
    {
	if (name == NULL || !forceit)
	{
	    // Check we can read it and it's an undo file.
	    fd = mch_open((char *)file_name, O_RDONLY|O_EXTRA, 0);
	    if (fd < 0)
	    {
		if (name != NULL || p_verbose > 0)
		{
		    if (name == NULL)
			verbose_enter();
		    smsg(
		      _("Will not overwrite with undo file, cannot read: %s"),
								   file_name);
		    if (name == NULL)
			verbose_leave();
		}
		goto theend;
	    }
	    else
	    {
		char_u	mbuf[UF_START_MAGIC_LEN];
		int	len;

		len = read_eintr(fd, mbuf, UF_START_MAGIC_LEN);
		close(fd);
		if (len < UF_START_MAGIC_LEN
		      || memcmp(mbuf, UF_START_MAGIC, UF_START_MAGIC_LEN) != 0)
		{
		    if (name != NULL || p_verbose > 0)
		    {
			if (name == NULL)
			    verbose_enter();
			smsg(
			_("Will not overwrite, this is not an undo file: %s"),
								   file_name);
			if (name == NULL)
			    verbose_leave();
		    }
		    goto theend;
		}
	    }
	}
	mch_remove(file_name);
    }

    // If there is no undo information at all, quit here after deleting any
    // existing undo file.
    if (buf->b_u_numhead == 0 && buf->b_u_line_ptr.ul_line == NULL)
    {
	if (p_verbose > 0)
	    verb_msg(_("Skipping undo file write, nothing to undo"));
	goto theend;
    }

    fd = mch_open((char *)file_name,
			    O_CREAT|O_EXTRA|O_WRONLY|O_EXCL|O_NOFOLLOW, perm);
    if (fd < 0)
    {
	semsg(_(e_cannot_open_undo_file_for_writing_str), file_name);
	goto theend;
    }
    (void)mch_setperm(file_name, perm);
    if (p_verbose > 0)
    {
	verbose_enter();
	smsg(_("Writing undo file: %s"), file_name);
	verbose_leave();
    }

#ifdef U_DEBUG
    // Check there is no problem in undo info before writing.
    u_check(FALSE);
#endif

#ifdef UNIX
    /*
     * Try to set the group of the undo file same as the original file. If
     * this fails, set the protection bits for the group same as the
     * protection bits for others.
     */
    if (st_old_valid
	    && mch_stat((char *)file_name, &st_new) >= 0
	    && st_new.st_gid != st_old.st_gid
# ifdef HAVE_FCHOWN  // sequent-ptx lacks fchown()
	    && fchown(fd, (uid_t)-1, st_old.st_gid) != 0
# endif
       )
	mch_setperm(file_name, (perm & 0707) | ((perm & 07) << 3));
# if defined(HAVE_SELINUX) || defined(HAVE_SMACK)
    if (buf->b_ffname != NULL)
	mch_copy_sec(buf->b_ffname, file_name);
# endif
#endif

    fp = fdopen(fd, "w");
    if (fp == NULL)
    {
	semsg(_(e_cannot_open_undo_file_for_writing_str), file_name);
	close(fd);
	mch_remove(file_name);
	goto theend;
    }

    // Undo must be synced.
    u_sync(TRUE);

    /*
     * Write the header.  Initializes encryption, if enabled.
     */
    bi.bi_buf = buf;
    bi.bi_fp = fp;
    if (serialize_header(&bi, hash) == FAIL)
	goto write_error;

    /*
     * Iteratively serialize UHPs and their UEPs from the top down.
     */
    mark = ++lastmark;
    uhp = buf->b_u_oldhead;
    while (uhp != NULL)
    {
	// Serialize current UHP if we haven't seen it
	if (uhp->uh_walk != mark)
	{
	    uhp->uh_walk = mark;
#ifdef U_DEBUG
	    ++headers_written;
#endif
	    if (serialize_uhp(&bi, uhp) == FAIL)
		goto write_error;
	}

	// Now walk through the tree - algorithm from undo_time().
	if (uhp->uh_prev.ptr != NULL && uhp->uh_prev.ptr->uh_walk != mark)
	    uhp = uhp->uh_prev.ptr;
	else if (uhp->uh_alt_next.ptr != NULL
				     && uhp->uh_alt_next.ptr->uh_walk != mark)
	    uhp = uhp->uh_alt_next.ptr;
	else if (uhp->uh_next.ptr != NULL && uhp->uh_alt_prev.ptr == NULL
					 && uhp->uh_next.ptr->uh_walk != mark)
	    uhp = uhp->uh_next.ptr;
	else if (uhp->uh_alt_prev.ptr != NULL)
	    uhp = uhp->uh_alt_prev.ptr;
	else
	    uhp = uhp->uh_next.ptr;
    }

    if (undo_write_bytes(&bi, (long_u)UF_HEADER_END_MAGIC, 2) == OK)
	write_ok = TRUE;
#ifdef U_DEBUG
    if (headers_written != buf->b_u_numhead)
    {
	semsg("Written %ld headers, ...", headers_written);
	semsg("... but numhead is %ld", buf->b_u_numhead);
    }
#endif

#ifdef FEAT_CRYPT
    if (bi.bi_state != NULL && undo_flush(&bi) == FAIL)
	write_ok = FALSE;
#endif

#if defined(UNIX) && defined(HAVE_FSYNC)
    if (p_fs && fflush(fp) == 0 && vim_fsync(fd) != 0)
	write_ok = FALSE;
#endif

write_error:
    fclose(fp);
    if (!write_ok)
	semsg(_(e_write_error_in_undo_file_str), file_name);

#if defined(MSWIN)
    // Copy file attributes; for systems where this can only be done after
    // closing the file.
    if (buf->b_ffname != NULL)
	(void)mch_copy_file_attribute(buf->b_ffname, file_name);
#endif
#ifdef HAVE_ACL
    if (buf->b_ffname != NULL)
    {
	vim_acl_T	    acl;

	// For systems that support ACL: get the ACL from the original file.
	acl = mch_get_acl(buf->b_ffname);
	mch_set_acl(file_name, acl);
	mch_free_acl(acl);
    }
#endif

theend:
#ifdef FEAT_CRYPT
    if (bi.bi_state != NULL)
	crypt_free_state(bi.bi_state);
    vim_free(bi.bi_buffer);
#endif
    if (file_name != name)
	vim_free(file_name);
}

/*
 * Load the undo tree from an undo file.
 * If "name" is not NULL use it as the undo file name.  This also means being
 * a bit more verbose.
 * Otherwise use curbuf->b_ffname to generate the undo file name.
 * "hash[UNDO_HASH_SIZE]" must be the hash value of the buffer text.
 */
    void
u_read_undo(char_u *name, char_u *hash, char_u *orig_name UNUSED)
{
    char_u	*file_name;
    FILE	*fp;
    long	version, str_len;
    undoline_T	line_ptr;
    linenr_T	line_lnum;
    colnr_T	line_colnr;
    linenr_T	line_count;
    long	num_head = 0;
    long	old_header_seq, new_header_seq, cur_header_seq;
    long	seq_last, seq_cur;
    long	last_save_nr = 0;
    short	old_idx = -1, new_idx = -1, cur_idx = -1;
    long	num_read_uhps = 0;
    time_t	seq_time;
    int		i, j;
    int		c;
    u_header_T	*uhp;
    u_header_T	**uhp_table = NULL;
    char_u	read_hash[UNDO_HASH_SIZE];
    char_u	magic_buf[UF_START_MAGIC_LEN];
#ifdef U_DEBUG
    int		*uhp_table_used;
#endif
#ifdef UNIX
    stat_T	st_orig;
    stat_T	st_undo;
#endif
    bufinfo_T	bi;

    CLEAR_FIELD(bi);
    line_ptr.ul_len = 0;
    line_ptr.ul_line = NULL;

    if (name == NULL)
    {
	file_name = u_get_undo_file_name(curbuf->b_ffname, TRUE);
	if (file_name == NULL)
	    return;

#ifdef UNIX
	// For safety we only read an undo file if the owner is equal to the
	// owner of the text file or equal to the current user.
	if (mch_stat((char *)orig_name, &st_orig) >= 0
		&& mch_stat((char *)file_name, &st_undo) >= 0
		&& st_orig.st_uid != st_undo.st_uid
		&& st_undo.st_uid != getuid())
	{
	    if (p_verbose > 0)
	    {
		verbose_enter();
		smsg(_("Not reading undo file, owner differs: %s"),
								   file_name);
		verbose_leave();
	    }
	    return;
	}
#endif
    }
    else
	file_name = name;

    if (p_verbose > 0)
    {
	verbose_enter();
	smsg(_("Reading undo file: %s"), file_name);
	verbose_leave();
    }

    fp = mch_fopen((char *)file_name, "r");
    if (fp == NULL)
    {
	if (name != NULL || p_verbose > 0)
	    semsg(_(e_cannot_open_undo_file_for_reading_str), file_name);
	goto error;
    }
    bi.bi_buf = curbuf;
    bi.bi_fp = fp;

    /*
     * Read the undo file header.
     */
    if (fread(magic_buf, UF_START_MAGIC_LEN, 1, fp) != 1
		|| memcmp(magic_buf, UF_START_MAGIC, UF_START_MAGIC_LEN) != 0)
    {
	semsg(_(e_not_an_undo_file_str), file_name);
	goto error;
    }
    version = get2c(fp);
    if (version == UF_VERSION_CRYPT)
    {
#ifdef FEAT_CRYPT
	if (*curbuf->b_p_key == NUL)
	{
	    semsg(_(e_non_encrypted_file_has_encrypted_undo_file_str),
								    file_name);
	    goto error;
	}
	bi.bi_state = crypt_create_from_file(fp, curbuf->b_p_key);
	if (bi.bi_state == NULL)
	{
	    semsg(_(e_undo_file_decryption_failed), file_name);
	    goto error;
	}
	if (crypt_whole_undofile(bi.bi_state->method_nr))
	{
	    bi.bi_buffer = alloc(CRYPT_BUF_SIZE);
	    if (bi.bi_buffer == NULL)
	    {
		crypt_free_state(bi.bi_state);
		bi.bi_state = NULL;
		goto error;
	    }
	    bi.bi_avail = 0;
	    bi.bi_used = 0;
	}
#else
	semsg(_(e_undo_file_is_encrypted_str), file_name);
	goto error;
#endif
    }
    else if (version != UF_VERSION)
    {
	semsg(_(e_incompatible_undo_file_str), file_name);
	goto error;
    }

    if (undo_read(&bi, read_hash, (size_t)UNDO_HASH_SIZE) == FAIL)
    {
	corruption_error("hash", file_name);
	goto error;
    }
    line_count = (linenr_T)undo_read_4c(&bi);
    if (memcmp(hash, read_hash, UNDO_HASH_SIZE) != 0
				  || line_count != curbuf->b_ml.ml_line_count)
    {
	if (p_verbose > 0 || name != NULL)
	{
	    if (name == NULL)
		verbose_enter();
	    give_warning((char_u *)
		      _("File contents changed, cannot use undo info"), TRUE);
	    if (name == NULL)
		verbose_leave();
	}
	goto error;
    }

    // Read undo data for "U" command.
    str_len = undo_read_4c(&bi);
    if (str_len < 0)
	goto error;
    if (str_len > 0)
    {
	line_ptr.ul_line = read_string_decrypt(&bi, str_len);
	line_ptr.ul_len = str_len + 1;
    }
    line_lnum = (linenr_T)undo_read_4c(&bi);
    line_colnr = (colnr_T)undo_read_4c(&bi);
    if (line_lnum < 0 || line_colnr < 0)
    {
	corruption_error("line lnum/col", file_name);
	goto error;
    }

    // Begin general undo data
    old_header_seq = undo_read_4c(&bi);
    new_header_seq = undo_read_4c(&bi);
    cur_header_seq = undo_read_4c(&bi);
    num_head = undo_read_4c(&bi);
    seq_last = undo_read_4c(&bi);
    seq_cur = undo_read_4c(&bi);
    seq_time = undo_read_time(&bi);

    // Optional header fields.
    for (;;)
    {
	int len = undo_read_byte(&bi);
	int what;

	if (len == 0 || len == EOF)
	    break;
	what = undo_read_byte(&bi);
	switch (what)
	{
	    case UF_LAST_SAVE_NR:
		last_save_nr = undo_read_4c(&bi);
		break;
	    default:
		// field not supported, skip
		while (--len >= 0)
		    (void)undo_read_byte(&bi);
	}
    }

    // uhp_table will store the freshly created undo headers we allocate
    // until we insert them into curbuf. The table remains sorted by the
    // sequence numbers of the headers.
    // When there are no headers uhp_table is NULL.
    if (num_head > 0)
    {
	if (num_head < LONG_MAX / (long)sizeof(u_header_T *))
	    uhp_table = U_ALLOC_LINE(num_head * sizeof(u_header_T *));
	if (uhp_table == NULL)
	    goto error;
    }

    while ((c = undo_read_2c(&bi)) == UF_HEADER_MAGIC)
    {
	if (num_read_uhps >= num_head)
	{
	    corruption_error("num_head too small", file_name);
	    goto error;
	}

	uhp = unserialize_uhp(&bi, file_name);
	if (uhp == NULL)
	    goto error;
	uhp_table[num_read_uhps++] = uhp;
    }

    if (num_read_uhps != num_head)
    {
	corruption_error("num_head", file_name);
	goto error;
    }
    if (c != UF_HEADER_END_MAGIC)
    {
	corruption_error("end marker", file_name);
	goto error;
    }

#ifdef U_DEBUG
    uhp_table_used = alloc_clear(sizeof(int) * num_head + 1);
# define SET_FLAG(j) ++uhp_table_used[j]
#else
# define SET_FLAG(j)
#endif

    // We have put all of the headers into a table. Now we iterate through the
    // table and swizzle each sequence number we have stored in uh_*_seq into
    // a pointer corresponding to the header with that sequence number.
    for (i = 0; i < num_head; i++)
    {
	uhp = uhp_table[i];
	if (uhp == NULL)
	    continue;
	for (j = 0; j < num_head; j++)
	    if (uhp_table[j] != NULL && i != j
			      && uhp_table[i]->uh_seq == uhp_table[j]->uh_seq)
	    {
		corruption_error("duplicate uh_seq", file_name);
		goto error;
	    }
	for (j = 0; j < num_head; j++)
	    if (uhp_table[j] != NULL
				  && uhp_table[j]->uh_seq == uhp->uh_next.seq)
	    {
		uhp->uh_next.ptr = uhp_table[j];
		SET_FLAG(j);
		break;
	    }
	for (j = 0; j < num_head; j++)
	    if (uhp_table[j] != NULL
				  && uhp_table[j]->uh_seq == uhp->uh_prev.seq)
	    {
		uhp->uh_prev.ptr = uhp_table[j];
		SET_FLAG(j);
		break;
	    }
	for (j = 0; j < num_head; j++)
	    if (uhp_table[j] != NULL
			      && uhp_table[j]->uh_seq == uhp->uh_alt_next.seq)
	    {
		uhp->uh_alt_next.ptr = uhp_table[j];
		SET_FLAG(j);
		break;
	    }
	for (j = 0; j < num_head; j++)
	    if (uhp_table[j] != NULL
			      && uhp_table[j]->uh_seq == uhp->uh_alt_prev.seq)
	    {
		uhp->uh_alt_prev.ptr = uhp_table[j];
		SET_FLAG(j);
		break;
	    }
	if (old_header_seq > 0 && old_idx < 0 && uhp->uh_seq == old_header_seq)
	{
	    old_idx = i;
	    SET_FLAG(i);
	}
	if (new_header_seq > 0 && new_idx < 0 && uhp->uh_seq == new_header_seq)
	{
	    new_idx = i;
	    SET_FLAG(i);
	}
	if (cur_header_seq > 0 && cur_idx < 0 && uhp->uh_seq == cur_header_seq)
	{
	    cur_idx = i;
	    SET_FLAG(i);
	}
    }

    // Now that we have read the undo info successfully, free the current undo
    // info and use the info from the file.
    u_blockfree(curbuf);
    curbuf->b_u_oldhead = old_idx < 0 ? NULL : uhp_table[old_idx];
    curbuf->b_u_newhead = new_idx < 0 ? NULL : uhp_table[new_idx];
    curbuf->b_u_curhead = cur_idx < 0 ? NULL : uhp_table[cur_idx];
    curbuf->b_u_line_ptr = line_ptr;
    curbuf->b_u_line_lnum = line_lnum;
    curbuf->b_u_line_colnr = line_colnr;
    curbuf->b_u_numhead = num_head;
    curbuf->b_u_seq_last = seq_last;
    curbuf->b_u_seq_cur = seq_cur;
    curbuf->b_u_time_cur = seq_time;
    curbuf->b_u_save_nr_last = last_save_nr;
    curbuf->b_u_save_nr_cur = last_save_nr;

    curbuf->b_u_synced = TRUE;
    vim_free(uhp_table);

#ifdef U_DEBUG
    for (i = 0; i < num_head; ++i)
	if (uhp_table_used[i] == 0)
	    semsg("uhp_table entry %ld not used, leaking memory", i);
    vim_free(uhp_table_used);
    u_check(TRUE);
#endif

    if (name != NULL)
	smsg(_("Finished reading undo file %s"), file_name);
    goto theend;

error:
    vim_free(line_ptr.ul_line);
    if (uhp_table != NULL)
    {
	for (i = 0; i < num_read_uhps; i++)
	    if (uhp_table[i] != NULL)
		u_free_uhp(uhp_table[i]);
	vim_free(uhp_table);
    }

theend:
#ifdef FEAT_CRYPT
    if (bi.bi_state != NULL)
	crypt_free_state(bi.bi_state);
    vim_free(bi.bi_buffer);
#endif
    if (fp != NULL)
	fclose(fp);
    if (file_name != name)
	vim_free(file_name);
    return;
}

#endif // FEAT_PERSISTENT_UNDO


/*
 * If 'cpoptions' contains 'u': Undo the previous undo or redo (vi compatible).
 * If 'cpoptions' does not contain 'u': Always undo.
 */
    void
u_undo(int count)
{
    /*
     * If we get an undo command while executing a macro, we behave like the
     * original vi. If this happens twice in one macro the result will not
     * be compatible.
     */
    if (curbuf->b_u_synced == FALSE)
    {
	u_sync(TRUE);
	count = 1;
    }

    if (vim_strchr(p_cpo, CPO_UNDO) == NULL)
	undo_undoes = TRUE;
    else
	undo_undoes = !undo_undoes;
    u_doit(count);
}

/*
 * If 'cpoptions' contains 'u': Repeat the previous undo or redo.
 * If 'cpoptions' does not contain 'u': Always redo.
 */
    void
u_redo(int count)
{
    if (vim_strchr(p_cpo, CPO_UNDO) == NULL)
	undo_undoes = FALSE;
    u_doit(count);
}

/*
 * Undo or redo, depending on 'undo_undoes', 'count' times.
 */
    static void
u_doit(int startcount)
{
    int count = startcount;

    if (!undo_allowed())
	return;

    u_newcount = 0;
    u_oldcount = 0;
    if (curbuf->b_ml.ml_flags & ML_EMPTY)
	u_oldcount = -1;
    while (count--)
    {
	// Do the change warning now, so that it triggers FileChangedRO when
	// needed.  This may cause the file to be reloaded, that must happen
	// before we do anything, because it may change curbuf->b_u_curhead
	// and more.
	change_warning(0);

	if (undo_undoes)
	{
	    if (curbuf->b_u_curhead == NULL)		// first undo
		curbuf->b_u_curhead = curbuf->b_u_newhead;
	    else if (get_undolevel() > 0)		// multi level undo
		// get next undo
		curbuf->b_u_curhead = curbuf->b_u_curhead->uh_next.ptr;
	    // nothing to undo
	    if (curbuf->b_u_numhead == 0 || curbuf->b_u_curhead == NULL)
	    {
		// stick curbuf->b_u_curhead at end
		curbuf->b_u_curhead = curbuf->b_u_oldhead;
		beep_flush();
		if (count == startcount - 1)
		{
		    msg(_("Already at oldest change"));
		    return;
		}
		break;
	    }

	    u_undoredo(TRUE);
	}
	else
	{
	    if (curbuf->b_u_curhead == NULL || get_undolevel() <= 0)
	    {
		beep_flush();	// nothing to redo
		if (count == startcount - 1)
		{
		    msg(_("Already at newest change"));
		    return;
		}
		break;
	    }

	    u_undoredo(FALSE);

	    // Advance for next redo.  Set "newhead" when at the end of the
	    // redoable changes.
	    if (curbuf->b_u_curhead->uh_prev.ptr == NULL)
		curbuf->b_u_newhead = curbuf->b_u_curhead;
	    curbuf->b_u_curhead = curbuf->b_u_curhead->uh_prev.ptr;
	}
    }
    u_undo_end(undo_undoes, FALSE);
}

/*
 * Undo or redo over the timeline.
 * When "step" is negative go back in time, otherwise goes forward in time.
 * When "sec" is FALSE make "step" steps, when "sec" is TRUE use "step" as
 * seconds.
 * When "file" is TRUE use "step" as a number of file writes.
 * When "absolute" is TRUE use "step" as the sequence number to jump to.
 * "sec" must be FALSE then.
 */
    void
undo_time(
    long	step,
    int		sec,
    int		file,
    int		absolute)
{
    long	    target;
    long	    closest;
    long	    closest_start;
    long	    closest_seq = 0;
    long	    val;
    u_header_T	    *uhp = NULL;
    u_header_T	    *last;
    int		    mark;
    int		    nomark = 0;  // shut up compiler
    int		    round;
    int		    dosec = sec;
    int		    dofile = file;
    int		    above = FALSE;
    int		    did_undo = TRUE;

    if (text_locked())
    {
	text_locked_msg();
	return;
    }

    // First make sure the current undoable change is synced.
    if (curbuf->b_u_synced == FALSE)
	u_sync(TRUE);

    u_newcount = 0;
    u_oldcount = 0;
    if (curbuf->b_ml.ml_flags & ML_EMPTY)
	u_oldcount = -1;

    // "target" is the node below which we want to be.
    // Init "closest" to a value we can't reach.
    if (absolute)
    {
	target = step;
	closest = -1;
    }
    else
    {
	if (dosec)
	    target = (long)(curbuf->b_u_time_cur) + step;
	else if (dofile)
	{
	    if (step < 0)
	    {
		// Going back to a previous write. If there were changes after
		// the last write, count that as moving one file-write, so
		// that ":earlier 1f" undoes all changes since the last save.
		uhp = curbuf->b_u_curhead;
		if (uhp != NULL)
		    uhp = uhp->uh_next.ptr;
		else
		    uhp = curbuf->b_u_newhead;
		if (uhp != NULL && uhp->uh_save_nr != 0)
		    // "uh_save_nr" was set in the last block, that means
		    // there were no changes since the last write
		    target = curbuf->b_u_save_nr_cur + step;
		else
		    // count the changes since the last write as one step
		    target = curbuf->b_u_save_nr_cur + step + 1;
		if (target <= 0)
		    // Go to before first write: before the oldest change. Use
		    // the sequence number for that.
		    dofile = FALSE;
	    }
	    else
	    {
		// Moving forward to a newer write.
		target = curbuf->b_u_save_nr_cur + step;
		if (target > curbuf->b_u_save_nr_last)
		{
		    // Go to after last write: after the latest change. Use
		    // the sequence number for that.
		    target = curbuf->b_u_seq_last + 1;
		    dofile = FALSE;
		}
	    }
	}
	else
	    target = curbuf->b_u_seq_cur + step;
	if (step < 0)
	{
	    if (target < 0)
		target = 0;
	    closest = -1;
	}
	else
	{
	    if (dosec)
		closest = (long)(vim_time() + 1);
	    else if (dofile)
		closest = curbuf->b_u_save_nr_last + 2;
	    else
		closest = curbuf->b_u_seq_last + 2;
	    if (target >= closest)
		target = closest - 1;
	}
    }
    closest_start = closest;
    closest_seq = curbuf->b_u_seq_cur;

    // When "target" is 0; Back to origin.
    if (target == 0)
    {
	mark = lastmark;  // avoid that GCC complains
	goto target_zero;
    }

    /*
     * May do this twice:
     * 1. Search for "target", update "closest" to the best match found.
     * 2. If "target" not found search for "closest".
     *
     * When using the closest time we use the sequence number in the second
     * round, because there may be several entries with the same time.
     */
    for (round = 1; round <= 2; ++round)
    {
	// Find the path from the current state to where we want to go.  The
	// desired state can be anywhere in the undo tree, need to go all over
	// it.  We put "nomark" in uh_walk where we have been without success,
	// "mark" where it could possibly be.
	mark = ++lastmark;
	nomark = ++lastmark;

	if (curbuf->b_u_curhead == NULL)	// at leaf of the tree
	    uhp = curbuf->b_u_newhead;
	else
	    uhp = curbuf->b_u_curhead;

	while (uhp != NULL)
	{
	    uhp->uh_walk = mark;
	    if (dosec)
		val = (long)(uhp->uh_time);
	    else if (dofile)
		val = uhp->uh_save_nr;
	    else
		val = uhp->uh_seq;

	    if (round == 1 && !(dofile && val == 0))
	    {
		// Remember the header that is closest to the target.
		// It must be at least in the right direction (checked with
		// "b_u_seq_cur").  When the timestamp is equal find the
		// highest/lowest sequence number.
		if ((step < 0 ? uhp->uh_seq <= curbuf->b_u_seq_cur
			      : uhp->uh_seq > curbuf->b_u_seq_cur)
			&& ((dosec && val == closest)
			    ? (step < 0
				? uhp->uh_seq < closest_seq
				: uhp->uh_seq > closest_seq)
			    : closest == closest_start
				|| (val > target
				    ? (closest > target
					? val - target <= closest - target
					: val - target <= target - closest)
				    : (closest > target
					? target - val <= closest - target
					: target - val <= target - closest))))
		{
		    closest = val;
		    closest_seq = uhp->uh_seq;
		}
	    }

	    // Quit searching when we found a match.  But when searching for a
	    // time we need to continue looking for the best uh_seq.
	    if (target == val && !dosec)
	    {
		target = uhp->uh_seq;
		break;
	    }

	    // go down in the tree if we haven't been there
	    if (uhp->uh_prev.ptr != NULL && uhp->uh_prev.ptr->uh_walk != nomark
					 && uhp->uh_prev.ptr->uh_walk != mark)
		uhp = uhp->uh_prev.ptr;

	    // go to alternate branch if we haven't been there
	    else if (uhp->uh_alt_next.ptr != NULL
		    && uhp->uh_alt_next.ptr->uh_walk != nomark
		    && uhp->uh_alt_next.ptr->uh_walk != mark)
		uhp = uhp->uh_alt_next.ptr;

	    // go up in the tree if we haven't been there and we are at the
	    // start of alternate branches
	    else if (uhp->uh_next.ptr != NULL && uhp->uh_alt_prev.ptr == NULL
		    && uhp->uh_next.ptr->uh_walk != nomark
		    && uhp->uh_next.ptr->uh_walk != mark)
	    {
		// If still at the start we don't go through this change.
		if (uhp == curbuf->b_u_curhead)
		    uhp->uh_walk = nomark;
		uhp = uhp->uh_next.ptr;
	    }

	    else
	    {
		// need to backtrack; mark this node as useless
		uhp->uh_walk = nomark;
		if (uhp->uh_alt_prev.ptr != NULL)
		    uhp = uhp->uh_alt_prev.ptr;
		else
		    uhp = uhp->uh_next.ptr;
	    }
	}

	if (uhp != NULL)    // found it
	    break;

	if (absolute)
	{
	    semsg(_(e_undo_number_nr_not_found), step);
	    return;
	}

	if (closest == closest_start)
	{
	    if (step < 0)
		msg(_("Already at oldest change"));
	    else
		msg(_("Already at newest change"));
	    return;
	}

	target = closest_seq;
	dosec = FALSE;
	dofile = FALSE;
	if (step < 0)
	    above = TRUE;	// stop above the header
    }

target_zero:
    // If we found it: Follow the path to go to where we want to be.
    if (uhp != NULL || target == 0)
    {
	/*
	 * First go up the tree as much as needed.
	 */
	while (!got_int)
	{
	    // Do the change warning now, for the same reason as above.
	    change_warning(0);

	    uhp = curbuf->b_u_curhead;
	    if (uhp == NULL)
		uhp = curbuf->b_u_newhead;
	    else
		uhp = uhp->uh_next.ptr;
	    if (uhp == NULL || (target > 0 && uhp->uh_walk != mark)
					 || (uhp->uh_seq == target && !above))
		break;
	    curbuf->b_u_curhead = uhp;
	    u_undoredo(TRUE);
	    if (target > 0)
		uhp->uh_walk = nomark;	// don't go back down here
	}

	// When back to origin, redo is not needed.
	if (target > 0)
	{
	    /*
	     * And now go down the tree (redo), branching off where needed.
	     */
	    while (!got_int)
	    {
		// Do the change warning now, for the same reason as above.
		change_warning(0);

		uhp = curbuf->b_u_curhead;
		if (uhp == NULL)
		    break;

		// Go back to the first branch with a mark.
		while (uhp->uh_alt_prev.ptr != NULL
				     && uhp->uh_alt_prev.ptr->uh_walk == mark)
		    uhp = uhp->uh_alt_prev.ptr;

		// Find the last branch with a mark, that's the one.
		last = uhp;
		while (last->uh_alt_next.ptr != NULL
				    && last->uh_alt_next.ptr->uh_walk == mark)
		    last = last->uh_alt_next.ptr;
		if (last != uhp)
		{
		    // Make the used branch the first entry in the list of
		    // alternatives to make "u" and CTRL-R take this branch.
		    while (uhp->uh_alt_prev.ptr != NULL)
			uhp = uhp->uh_alt_prev.ptr;
		    if (last->uh_alt_next.ptr != NULL)
			last->uh_alt_next.ptr->uh_alt_prev.ptr =
							last->uh_alt_prev.ptr;
		    last->uh_alt_prev.ptr->uh_alt_next.ptr =
							last->uh_alt_next.ptr;
		    last->uh_alt_prev.ptr = NULL;
		    last->uh_alt_next.ptr = uhp;
		    uhp->uh_alt_prev.ptr = last;

		    if (curbuf->b_u_oldhead == uhp)
			curbuf->b_u_oldhead = last;
		    uhp = last;
		    if (uhp->uh_next.ptr != NULL)
			uhp->uh_next.ptr->uh_prev.ptr = uhp;
		}
		curbuf->b_u_curhead = uhp;

		if (uhp->uh_walk != mark)
		    break;	    // must have reached the target

		// Stop when going backwards in time and didn't find the exact
		// header we were looking for.
		if (uhp->uh_seq == target && above)
		{
		    curbuf->b_u_seq_cur = target - 1;
		    break;
		}

		u_undoredo(FALSE);

		// Advance "curhead" to below the header we last used.  If it
		// becomes NULL then we need to set "newhead" to this leaf.
		if (uhp->uh_prev.ptr == NULL)
		    curbuf->b_u_newhead = uhp;
		curbuf->b_u_curhead = uhp->uh_prev.ptr;
		did_undo = FALSE;

		if (uhp->uh_seq == target)	// found it!
		    break;

		uhp = uhp->uh_prev.ptr;
		if (uhp == NULL || uhp->uh_walk != mark)
		{
		    // Need to redo more but can't find it...
		    internal_error("undo_time()");
		    break;
		}
	    }
	}
    }
    u_undo_end(did_undo, absolute);
}

/*
 * u_undoredo: common code for undo and redo
 *
 * The lines in the file are replaced by the lines in the entry list at
 * curbuf->b_u_curhead. The replaced lines in the file are saved in the entry
 * list for the next undo/redo.
 *
 * When "undo" is TRUE we go up in the tree, when FALSE we go down.
 */
    static void
u_undoredo(int undo)
{
    undoline_T	*newarray = NULL;
    linenr_T	oldsize;
    linenr_T	newsize;
    linenr_T	top, bot;
    linenr_T	lnum;
    linenr_T	newlnum = MAXLNUM;
    pos_T	new_curpos = curwin->w_cursor;
    long	i;
    u_entry_T	*uep, *nuep;
    u_entry_T	*newlist = NULL;
    int		old_flags;
    int		new_flags;
    pos_T	namedm[NMARKS];
    visualinfo_T visualinfo;
    int		empty_buffer;		    // buffer became empty
    u_header_T	*curhead = curbuf->b_u_curhead;

    // Don't want autocommands using the undo structures here, they are
    // invalid till the end.
    block_autocmds();

#ifdef U_DEBUG
    u_check(FALSE);
#endif
    old_flags = curhead->uh_flags;
    new_flags = (curbuf->b_changed ? UH_CHANGED : 0) +
	       ((curbuf->b_ml.ml_flags & ML_EMPTY) ? UH_EMPTYBUF : 0);
    setpcmark();

    /*
     * save marks before undo/redo
     */
    mch_memmove(namedm, curbuf->b_namedm, sizeof(pos_T) * NMARKS);
    visualinfo = curbuf->b_visual;
    curbuf->b_op_start.lnum = curbuf->b_ml.ml_line_count;
    curbuf->b_op_start.col = 0;
    curbuf->b_op_end.lnum = 0;
    curbuf->b_op_end.col = 0;

    for (uep = curhead->uh_entry; uep != NULL; uep = nuep)
    {
	top = uep->ue_top;
	bot = uep->ue_bot;
	if (bot == 0)
	    bot = curbuf->b_ml.ml_line_count + 1;
	if (top > curbuf->b_ml.ml_line_count || top >= bot
				      || bot > curbuf->b_ml.ml_line_count + 1)
	{
	    unblock_autocmds();
	    iemsg(e_u_undo_line_numbers_wrong);
	    changed();		// don't want UNCHANGED now
	    return;
	}

	oldsize = bot - top - 1;    // number of lines before undo
	newsize = uep->ue_size;	    // number of lines after undo

	// Decide about the cursor position, depending on what text changed.
	// Don't set it yet, it may be invalid if lines are going to be added.
	if (top < newlnum)
	{
	    // If the saved cursor is somewhere in this undo block, move it to
	    // the remembered position.  Makes "gwap" put the cursor back
	    // where it was.
	    lnum = curhead->uh_cursor.lnum;
	    if (lnum >= top && lnum <= top + newsize + 1)
	    {
		new_curpos = curhead->uh_cursor;
		newlnum = new_curpos.lnum - 1;
	    }
	    else
	    {
		// Use the first line that actually changed.  Avoids that
		// undoing auto-formatting puts the cursor in the previous
		// line.
		for (i = 0; i < newsize && i < oldsize; ++i)
		{
		    char_u *p = ml_get(top + 1 + i);

		    if (curbuf->b_ml.ml_line_len != uep->ue_array[i].ul_len
			    || memcmp(uep->ue_array[i].ul_line, p,
						curbuf->b_ml.ml_line_len) != 0)
			break;
		}
		if (i == newsize && newlnum == MAXLNUM && uep->ue_next == NULL)
		{
		    newlnum = top;
		    new_curpos.lnum = newlnum + 1;
		}
		else if (i < newsize)
		{
		    newlnum = top + i;
		    new_curpos.lnum = newlnum + 1;
		}
	    }
	}

	empty_buffer = FALSE;

	/*
	 * Delete the lines between top and bot and save them in newarray.
	 */
	if (oldsize > 0)
	{
	    if ((newarray = U_ALLOC_LINE(sizeof(undoline_T) * oldsize)) == NULL)
	    {
		do_outofmem_msg((long_u)(sizeof(undoline_T) * oldsize));

		// We have messed up the entry list, repair is impossible.
		// we have to free the rest of the list.
		while (uep != NULL)
		{
		    nuep = uep->ue_next;
		    u_freeentry(uep, uep->ue_size);
		    uep = nuep;
		}
		break;
	    }
	    // delete backwards, it goes faster in most cases
	    for (lnum = bot - 1, i = oldsize; --i >= 0; --lnum)
	    {
		// what can we do when we run out of memory?
		if (u_save_line(&newarray[i], lnum) == FAIL)
		    do_outofmem_msg((long_u)0);
		// remember we deleted the last line in the buffer, and a
		// dummy empty line will be inserted
		if (curbuf->b_ml.ml_line_count == 1)
		    empty_buffer = TRUE;
		ml_delete_flags(lnum, ML_DEL_UNDO);
	    }
	}
	else
	    newarray = NULL;

	// make sure the cursor is on a valid line after the deletions
	check_cursor_lnum();

	/*
	 * Insert the lines in u_array between top and bot.
	 */
	if (newsize)
	{
	    for (lnum = top, i = 0; i < newsize; ++i, ++lnum)
	    {
		// If the file is empty, there is an empty line 1 that we
		// should get rid of, by replacing it with the new line.
		if (empty_buffer && lnum == 0)
		    ml_replace_len((linenr_T)1, uep->ue_array[i].ul_line,
					  uep->ue_array[i].ul_len, TRUE, TRUE);
		else
		    ml_append_flags(lnum, uep->ue_array[i].ul_line,
			     (colnr_T)uep->ue_array[i].ul_len, ML_APPEND_UNDO);
		vim_free(uep->ue_array[i].ul_line);
	    }
	    vim_free((char_u *)uep->ue_array);
	}

	// adjust marks
	if (oldsize != newsize)
	{
	    mark_adjust(top + 1, top + oldsize, (long)MAXLNUM,
					       (long)newsize - (long)oldsize);
	    if (curbuf->b_op_start.lnum > top + oldsize)
		curbuf->b_op_start.lnum += newsize - oldsize;
	    if (curbuf->b_op_end.lnum > top + oldsize)
		curbuf->b_op_end.lnum += newsize - oldsize;
	}
	if (oldsize > 0 || newsize > 0)
	{
	    changed_lines(top + 1, 0, bot, newsize - oldsize);
#ifdef FEAT_SPELL
	    // When text has been changed, possibly the start of the next line
	    // may have SpellCap that should be removed or it needs to be
	    // displayed.  Schedule the next line for redrawing just in case.
	    if (spell_check_window(curwin) && bot <= curbuf->b_ml.ml_line_count)
		redrawWinline(curwin, bot);
#endif
	}

	// Set the '[ mark.
	if (top + 1 < curbuf->b_op_start.lnum)
	    curbuf->b_op_start.lnum = top + 1;
	// Set the '] mark.
	if (newsize == 0 && top + 1 > curbuf->b_op_end.lnum)
	    curbuf->b_op_end.lnum = top + 1;
	else if (top + newsize > curbuf->b_op_end.lnum)
	    curbuf->b_op_end.lnum = top + newsize;

	u_newcount += newsize;
	u_oldcount += oldsize;
	uep->ue_size = oldsize;
	uep->ue_array = newarray;
	uep->ue_bot = top + newsize + 1;

	/*
	 * insert this entry in front of the new entry list
	 */
	nuep = uep->ue_next;
	uep->ue_next = newlist;
	newlist = uep;
    }

    // Ensure the '[ and '] marks are within bounds.
    if (curbuf->b_op_start.lnum > curbuf->b_ml.ml_line_count)
	 curbuf->b_op_start.lnum = curbuf->b_ml.ml_line_count;
    if (curbuf->b_op_end.lnum > curbuf->b_ml.ml_line_count)
	 curbuf->b_op_end.lnum = curbuf->b_ml.ml_line_count;

    // Set the cursor to the desired position.  Check that the line is valid.
    curwin->w_cursor = new_curpos;
    check_cursor_lnum();

    curhead->uh_entry = newlist;
    curhead->uh_flags = new_flags;
    if ((old_flags & UH_EMPTYBUF) && BUFEMPTY())
	curbuf->b_ml.ml_flags |= ML_EMPTY;
    if (old_flags & UH_CHANGED)
	changed();
    else
#ifdef FEAT_NETBEANS_INTG
	// per netbeans undo rules, keep it as modified
	if (!isNetbeansModified(curbuf))
#endif
	unchanged(curbuf, FALSE, TRUE);

    /*
     * restore marks from before undo/redo
     */
    for (i = 0; i < NMARKS; ++i)
    {
	if (curhead->uh_namedm[i].lnum != 0)
	    curbuf->b_namedm[i] = curhead->uh_namedm[i];
	if (namedm[i].lnum != 0)
	    curhead->uh_namedm[i] = namedm[i];
	else
	    curhead->uh_namedm[i].lnum = 0;
    }
    if (curhead->uh_visual.vi_start.lnum != 0)
    {
	curbuf->b_visual = curhead->uh_visual;
	curhead->uh_visual = visualinfo;
    }

    /*
     * If the cursor is only off by one line, put it at the same position as
     * before starting the change (for the "o" command).
     * Otherwise the cursor should go to the first undone line.
     */
    if (curhead->uh_cursor.lnum + 1 == curwin->w_cursor.lnum
						 && curwin->w_cursor.lnum > 1)
	--curwin->w_cursor.lnum;
    if (curwin->w_cursor.lnum <= curbuf->b_ml.ml_line_count)
    {
	if (curhead->uh_cursor.lnum == curwin->w_cursor.lnum)
	{
	    curwin->w_cursor.col = curhead->uh_cursor.col;
	    if (virtual_active() && curhead->uh_cursor_vcol >= 0)
		coladvance((colnr_T)curhead->uh_cursor_vcol);
	    else
		curwin->w_cursor.coladd = 0;
	}
	else
	    beginline(BL_SOL | BL_FIX);
    }
    else
    {
	// We get here with the current cursor line being past the end (eg
	// after adding lines at the end of the file, and then undoing it).
	// check_cursor() will move the cursor to the last line.  Move it to
	// the first column here.
	curwin->w_cursor.col = 0;
	curwin->w_cursor.coladd = 0;
    }

    // Make sure the cursor is on an existing line and column.
    check_cursor();

    // Remember where we are for "g-" and ":earlier 10s".
    curbuf->b_u_seq_cur = curhead->uh_seq;
    if (undo)
    {
	// We are below the previous undo.  However, to make ":earlier 1s"
	// work we compute this as being just above the just undone change.
	if (curhead->uh_next.ptr != NULL)
	    curbuf->b_u_seq_cur = curhead->uh_next.ptr->uh_seq;
	else
	    curbuf->b_u_seq_cur = 0;
    }

    // Remember where we are for ":earlier 1f" and ":later 1f".
    if (curhead->uh_save_nr != 0)
    {
	if (undo)
	    curbuf->b_u_save_nr_cur = curhead->uh_save_nr - 1;
	else
	    curbuf->b_u_save_nr_cur = curhead->uh_save_nr;
    }

    // The timestamp can be the same for multiple changes, just use the one of
    // the undone/redone change.
    curbuf->b_u_time_cur = curhead->uh_time;

    unblock_autocmds();
#ifdef U_DEBUG
    u_check(FALSE);
#endif
}

/*
 * If we deleted or added lines, report the number of less/more lines.
 * Otherwise, report the number of changes (this may be incorrect
 * in some cases, but it's better than nothing).
 */
    static void
u_undo_end(
    int		did_undo,	// just did an undo
    int		absolute)	// used ":undo N"
{
    char	*msgstr;
    u_header_T	*uhp;
    char_u	msgbuf[80];

#ifdef FEAT_FOLDING
    if ((fdo_flags & FDO_UNDO) && KeyTyped)
	foldOpenCursor();
#endif

    if (global_busy	    // no messages now, wait until global is finished
	    || !messaging())  // 'lazyredraw' set, don't do messages now
	return;

    if (curbuf->b_ml.ml_flags & ML_EMPTY)
	--u_newcount;

    u_oldcount -= u_newcount;
    if (u_oldcount == -1)
	msgstr = N_("more line");
    else if (u_oldcount < 0)
	msgstr = N_("more lines");
    else if (u_oldcount == 1)
	msgstr = N_("line less");
    else if (u_oldcount > 1)
	msgstr = N_("fewer lines");
    else
    {
	u_oldcount = u_newcount;
	if (u_newcount == 1)
	    msgstr = N_("change");
	else
	    msgstr = N_("changes");
    }

    if (curbuf->b_u_curhead != NULL)
    {
	// For ":undo N" we prefer a "after #N" message.
	if (absolute && curbuf->b_u_curhead->uh_next.ptr != NULL)
	{
	    uhp = curbuf->b_u_curhead->uh_next.ptr;
	    did_undo = FALSE;
	}
	else if (did_undo)
	    uhp = curbuf->b_u_curhead;
	else
	    uhp = curbuf->b_u_curhead->uh_next.ptr;
    }
    else
	uhp = curbuf->b_u_newhead;

    if (uhp == NULL)
	*msgbuf = NUL;
    else
	add_time(msgbuf, sizeof(msgbuf), uhp->uh_time);

#ifdef FEAT_CONCEAL
    {
	win_T	*wp;

	FOR_ALL_WINDOWS(wp)
	{
	    if (wp->w_buffer == curbuf && wp->w_p_cole > 0)
		redraw_win_later(wp, UPD_NOT_VALID);
	}
    }
#endif
    if (VIsual_active)
	check_pos(curbuf, &VIsual);

    smsg_attr_keep(0, _("%ld %s; %s #%ld  %s"),
	    u_oldcount < 0 ? -u_oldcount : u_oldcount,
	    _(msgstr),
	    did_undo ? _("before") : _("after"),
	    uhp == NULL ? 0L : uhp->uh_seq,
	    msgbuf);
}

/*
 * u_sync: stop adding to the current entry list
 */
    void
u_sync(
    int	    force)	// Also sync when no_u_sync is set.
{
    // Skip it when already synced or syncing is disabled.
    if (curbuf->b_u_synced || (!force && no_u_sync > 0))
	return;
#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
    if (p_imst == IM_ON_THE_SPOT && im_is_preediting())
	return;		    // XIM is busy, don't break an undo sequence
#endif
    if (get_undolevel() < 0)
	curbuf->b_u_synced = TRUE;  // no entries, nothing to do
    else
    {
	u_getbot();		    // compute ue_bot of previous u_save
	curbuf->b_u_curhead = NULL;
    }
}

/*
 * ":undolist": List the leafs of the undo tree
 */
    void
ex_undolist(exarg_T *eap UNUSED)
{
    garray_T	ga;
    u_header_T	*uhp;
    int		mark;
    int		nomark;
    int		changes = 1;
    int		i;

    /*
     * 1: walk the tree to find all leafs, put the info in "ga".
     * 2: sort the lines
     * 3: display the list
     */
    mark = ++lastmark;
    nomark = ++lastmark;
    ga_init2(&ga, sizeof(char *), 20);

    uhp = curbuf->b_u_oldhead;
    while (uhp != NULL)
    {
	if (uhp->uh_prev.ptr == NULL && uhp->uh_walk != nomark
						      && uhp->uh_walk != mark)
	{
	    if (ga_grow(&ga, 1) == FAIL)
		break;
	    vim_snprintf((char *)IObuff, IOSIZE, "%6ld %7d  ",
							uhp->uh_seq, changes);
	    add_time(IObuff + STRLEN(IObuff), IOSIZE - STRLEN(IObuff),
								uhp->uh_time);
	    if (uhp->uh_save_nr > 0)
	    {
		while (STRLEN(IObuff) < 33)
		    STRCAT(IObuff, " ");
		vim_snprintf_add((char *)IObuff, IOSIZE,
						   "  %3ld", uhp->uh_save_nr);
	    }
	    ((char_u **)(ga.ga_data))[ga.ga_len++] = vim_strsave(IObuff);
	}

	uhp->uh_walk = mark;

	// go down in the tree if we haven't been there
	if (uhp->uh_prev.ptr != NULL && uhp->uh_prev.ptr->uh_walk != nomark
					 && uhp->uh_prev.ptr->uh_walk != mark)
	{
	    uhp = uhp->uh_prev.ptr;
	    ++changes;
	}

	// go to alternate branch if we haven't been there
	else if (uhp->uh_alt_next.ptr != NULL
		&& uhp->uh_alt_next.ptr->uh_walk != nomark
		&& uhp->uh_alt_next.ptr->uh_walk != mark)
	    uhp = uhp->uh_alt_next.ptr;

	// go up in the tree if we haven't been there and we are at the
	// start of alternate branches
	else if (uhp->uh_next.ptr != NULL && uhp->uh_alt_prev.ptr == NULL
		&& uhp->uh_next.ptr->uh_walk != nomark
		&& uhp->uh_next.ptr->uh_walk != mark)
	{
	    uhp = uhp->uh_next.ptr;
	    --changes;
	}

	else
	{
	    // need to backtrack; mark this node as done
	    uhp->uh_walk = nomark;
	    if (uhp->uh_alt_prev.ptr != NULL)
		uhp = uhp->uh_alt_prev.ptr;
	    else
	    {
		uhp = uhp->uh_next.ptr;
		--changes;
	    }
	}
    }

    if (ga.ga_len == 0)
	msg(_("Nothing to undo"));
    else
    {
	sort_strings((char_u **)ga.ga_data, ga.ga_len);

	msg_start();
	msg_puts_attr(_("number changes  when               saved"),
							      HL_ATTR(HLF_T));
	for (i = 0; i < ga.ga_len && !got_int; ++i)
	{
	    msg_putchar('\n');
	    if (got_int)
		break;
	    msg_puts(((char **)ga.ga_data)[i]);
	}
	msg_end();

	ga_clear_strings(&ga);
    }
}

/*
 * ":undojoin": continue adding to the last entry list
 */
    void
ex_undojoin(exarg_T *eap UNUSED)
{
    if (curbuf->b_u_newhead == NULL)
	return;		    // nothing changed before
    if (curbuf->b_u_curhead != NULL)
    {
	emsg(_(e_undojoin_is_not_allowed_after_undo));
	return;
    }
    if (!curbuf->b_u_synced)
	return;		    // already unsynced
    if (get_undolevel() < 0)
	return;		    // no entries, nothing to do
    else
	// Append next change to the last entry
	curbuf->b_u_synced = FALSE;
}

/*
 * Called after writing or reloading the file and setting b_changed to FALSE.
 * Now an undo means that the buffer is modified.
 */
    void
u_unchanged(buf_T *buf)
{
    u_unch_branch(buf->b_u_oldhead);
    buf->b_did_warn = FALSE;
}

/*
 * After reloading a buffer which was saved for 'undoreload': Find the first
 * line that was changed and set the cursor there.
 */
    void
u_find_first_changed(void)
{
    u_header_T	*uhp = curbuf->b_u_newhead;
    u_entry_T   *uep;
    linenr_T	lnum;

    if (curbuf->b_u_curhead != NULL || uhp == NULL)
	return;  // undid something in an autocmd?

    // Check that the last undo block was for the whole file.
    uep = uhp->uh_entry;
    if (uep->ue_top != 0 || uep->ue_bot != 0)
	return;

    for (lnum = 1; lnum < curbuf->b_ml.ml_line_count
					      && lnum <= uep->ue_size; ++lnum)
    {
	char_u *p = ml_get_buf(curbuf, lnum, FALSE);

	if (uep->ue_array[lnum - 1].ul_len != curbuf->b_ml.ml_line_len
		|| memcmp(p, uep->ue_array[lnum - 1].ul_line, uep->ue_array[lnum - 1].ul_len) != 0)
	{
	    CLEAR_POS(&(uhp->uh_cursor));
	    uhp->uh_cursor.lnum = lnum;
	    return;
	}
    }
    if (curbuf->b_ml.ml_line_count != uep->ue_size)
    {
	// lines added or deleted at the end, put the cursor there
	CLEAR_POS(&(uhp->uh_cursor));
	uhp->uh_cursor.lnum = lnum;
    }
}

/*
 * Increase the write count, store it in the last undo header, what would be
 * used for "u".
 */
    void
u_update_save_nr(buf_T *buf)
{
    u_header_T	*uhp;

    ++buf->b_u_save_nr_last;
    buf->b_u_save_nr_cur = buf->b_u_save_nr_last;
    uhp = buf->b_u_curhead;
    if (uhp != NULL)
	uhp = uhp->uh_next.ptr;
    else
	uhp = buf->b_u_newhead;
    if (uhp != NULL)
	uhp->uh_save_nr = buf->b_u_save_nr_last;
}

    static void
u_unch_branch(u_header_T *uhp)
{
    u_header_T	*uh;

    for (uh = uhp; uh != NULL; uh = uh->uh_prev.ptr)
    {
	uh->uh_flags |= UH_CHANGED;
	if (uh->uh_alt_next.ptr != NULL)
	    u_unch_branch(uh->uh_alt_next.ptr);	    // recursive
    }
}

/*
 * Get pointer to last added entry.
 * If it's not valid, give an error message and return NULL.
 */
    static u_entry_T *
u_get_headentry(void)
{
    if (curbuf->b_u_newhead == NULL || curbuf->b_u_newhead->uh_entry == NULL)
    {
	iemsg(e_undo_list_corrupt);
	return NULL;
    }
    return curbuf->b_u_newhead->uh_entry;
}

/*
 * u_getbot(): compute the line number of the previous u_save
 *		It is called only when b_u_synced is FALSE.
 */
    static void
u_getbot(void)
{
    u_entry_T	*uep;
    linenr_T	extra;

    uep = u_get_headentry();	// check for corrupt undo list
    if (uep == NULL)
	return;

    uep = curbuf->b_u_newhead->uh_getbot_entry;
    if (uep != NULL)
    {
	/*
	 * the new ue_bot is computed from the number of lines that has been
	 * inserted (0 - deleted) since calling u_save. This is equal to the
	 * old line count subtracted from the current line count.
	 */
	extra = curbuf->b_ml.ml_line_count - uep->ue_lcount;
	uep->ue_bot = uep->ue_top + uep->ue_size + 1 + extra;
	if (uep->ue_bot < 1 || uep->ue_bot > curbuf->b_ml.ml_line_count)
	{
	    iemsg(e_undo_line_missing);
	    uep->ue_bot = uep->ue_top + 1;  // assume all lines deleted, will
					    // get all the old lines back
					    // without deleting the current
					    // ones
	}

	curbuf->b_u_newhead->uh_getbot_entry = NULL;
    }

    curbuf->b_u_synced = TRUE;
}

/*
 * Free one header "uhp" and its entry list and adjust the pointers.
 */
    static void
u_freeheader(
    buf_T	    *buf,
    u_header_T	    *uhp,
    u_header_T	    **uhpp)	// if not NULL reset when freeing this header
{
    u_header_T	    *uhap;

    // When there is an alternate redo list free that branch completely,
    // because we can never go there.
    if (uhp->uh_alt_next.ptr != NULL)
	u_freebranch(buf, uhp->uh_alt_next.ptr, uhpp);

    if (uhp->uh_alt_prev.ptr != NULL)
	uhp->uh_alt_prev.ptr->uh_alt_next.ptr = NULL;

    // Update the links in the list to remove the header.
    if (uhp->uh_next.ptr == NULL)
	buf->b_u_oldhead = uhp->uh_prev.ptr;
    else
	uhp->uh_next.ptr->uh_prev.ptr = uhp->uh_prev.ptr;

    if (uhp->uh_prev.ptr == NULL)
	buf->b_u_newhead = uhp->uh_next.ptr;
    else
	for (uhap = uhp->uh_prev.ptr; uhap != NULL;
						 uhap = uhap->uh_alt_next.ptr)
	    uhap->uh_next.ptr = uhp->uh_next.ptr;

    u_freeentries(buf, uhp, uhpp);
}

/*
 * Free an alternate branch and any following alternate branches.
 */
    static void
u_freebranch(
    buf_T	    *buf,
    u_header_T	    *uhp,
    u_header_T	    **uhpp)	// if not NULL reset when freeing this header
{
    u_header_T	    *tofree, *next;

    // If this is the top branch we may need to use u_freeheader() to update
    // all the pointers.
    if (uhp == buf->b_u_oldhead)
    {
	while (buf->b_u_oldhead != NULL)
	    u_freeheader(buf, buf->b_u_oldhead, uhpp);
	return;
    }

    if (uhp->uh_alt_prev.ptr != NULL)
	uhp->uh_alt_prev.ptr->uh_alt_next.ptr = NULL;

    next = uhp;
    while (next != NULL)
    {
	tofree = next;
	if (tofree->uh_alt_next.ptr != NULL)
	    u_freebranch(buf, tofree->uh_alt_next.ptr, uhpp);   // recursive
	next = tofree->uh_prev.ptr;
	u_freeentries(buf, tofree, uhpp);
    }
}

/*
 * Free all the undo entries for one header and the header itself.
 * This means that "uhp" is invalid when returning.
 */
    static void
u_freeentries(
    buf_T	    *buf,
    u_header_T	    *uhp,
    u_header_T	    **uhpp)	// if not NULL reset when freeing this header
{
    u_entry_T	    *uep, *nuep;

    // Check for pointers to the header that become invalid now.
    if (buf->b_u_curhead == uhp)
	buf->b_u_curhead = NULL;
    if (buf->b_u_newhead == uhp)
	buf->b_u_newhead = NULL;  // freeing the newest entry
    if (uhpp != NULL && uhp == *uhpp)
	*uhpp = NULL;

    for (uep = uhp->uh_entry; uep != NULL; uep = nuep)
    {
	nuep = uep->ue_next;
	u_freeentry(uep, uep->ue_size);
    }

#ifdef U_DEBUG
    uhp->uh_magic = 0;
#endif
    vim_free((char_u *)uhp);
    --buf->b_u_numhead;
}

/*
 * free entry 'uep' and 'n' lines in uep->ue_array[]
 */
    static void
u_freeentry(u_entry_T *uep, long n)
{
    while (n > 0)
	vim_free(uep->ue_array[--n].ul_line);
    vim_free((char_u *)uep->ue_array);
#ifdef U_DEBUG
    uep->ue_magic = 0;
#endif
    vim_free((char_u *)uep);
}

/*
 * invalidate the undo buffer; called when storage has already been released
 */
    void
u_clearall(buf_T *buf)
{
    buf->b_u_newhead = buf->b_u_oldhead = buf->b_u_curhead = NULL;
    buf->b_u_synced = TRUE;
    buf->b_u_numhead = 0;
    buf->b_u_line_ptr.ul_line = NULL;
    buf->b_u_line_ptr.ul_len = 0;
    buf->b_u_line_lnum = 0;
}

/*
 * Save the line "lnum" for the "U" command.
 */
    static void
u_saveline(linenr_T lnum)
{
    if (lnum == curbuf->b_u_line_lnum)	    // line is already saved
	return;
    if (lnum < 1 || lnum > curbuf->b_ml.ml_line_count) // should never happen
	return;
    u_clearline();
    curbuf->b_u_line_lnum = lnum;
    if (curwin->w_cursor.lnum == lnum)
	curbuf->b_u_line_colnr = curwin->w_cursor.col;
    else
	curbuf->b_u_line_colnr = 0;
    if (u_save_line(&curbuf->b_u_line_ptr, lnum) == FAIL)
	do_outofmem_msg((long_u)0);
}

/*
 * clear the line saved for the "U" command
 * (this is used externally for crossing a line while in insert mode)
 */
    void
u_clearline(void)
{
    if (curbuf->b_u_line_ptr.ul_line == NULL)
	return;

    VIM_CLEAR(curbuf->b_u_line_ptr.ul_line);
    curbuf->b_u_line_ptr.ul_len = 0;
    curbuf->b_u_line_lnum = 0;
}

/*
 * Implementation of the "U" command.
 * Differentiation from vi: "U" can be undone with the next "U".
 * We also allow the cursor to be in another line.
 * Careful: may trigger autocommands that reload the buffer.
 */
    void
u_undoline(void)
{
    colnr_T	t;
    undoline_T  oldp;

    if (undo_off)
	return;

    if (curbuf->b_u_line_ptr.ul_line == NULL
			|| curbuf->b_u_line_lnum > curbuf->b_ml.ml_line_count)
    {
	beep_flush();
	return;
    }

    // first save the line for the 'u' command
    if (u_savecommon(curbuf->b_u_line_lnum - 1,
		       curbuf->b_u_line_lnum + 1, (linenr_T)0, FALSE) == FAIL)
	return;
    if (u_save_line(&oldp, curbuf->b_u_line_lnum) == FAIL)
    {
	do_outofmem_msg((long_u)0);
	return;
    }
    ml_replace_len(curbuf->b_u_line_lnum, curbuf->b_u_line_ptr.ul_line,
				     curbuf->b_u_line_ptr.ul_len, TRUE, FALSE);
    changed_bytes(curbuf->b_u_line_lnum, 0);
    curbuf->b_u_line_ptr = oldp;

    t = curbuf->b_u_line_colnr;
    if (curwin->w_cursor.lnum == curbuf->b_u_line_lnum)
	curbuf->b_u_line_colnr = curwin->w_cursor.col;
    curwin->w_cursor.col = t;
    curwin->w_cursor.lnum = curbuf->b_u_line_lnum;
    check_cursor_col();
}

/*
 * Free all allocated memory blocks for the buffer 'buf'.
 */
    void
u_blockfree(buf_T *buf)
{
    while (buf->b_u_oldhead != NULL)
	u_freeheader(buf, buf->b_u_oldhead, NULL);
    vim_free(buf->b_u_line_ptr.ul_line);
}

/*
 * Check if the 'modified' flag is set, or 'ff' has changed (only need to
 * check the first character, because it can only be "dos", "unix" or "mac").
 * "nofile" and "scratch" type buffers are considered to always be unchanged.
 * Also considers a buffer changed when a terminal window contains a running
 * job.
 */
    int
bufIsChanged(buf_T *buf)
{
#ifdef FEAT_TERMINAL
    if (term_job_running_not_none(buf->b_term))
	return TRUE;
#endif
    return bufIsChangedNotTerm(buf);
}

/*
 * Return TRUE if any buffer has changes.  Also buffers that are not written.
 */
    int
anyBufIsChanged(void)
{
    buf_T *buf;

    FOR_ALL_BUFFERS(buf)
	if (bufIsChanged(buf))
	    return TRUE;
    return FALSE;
}

/*
 * Like bufIsChanged() but ignoring a terminal window.
 */
    int
bufIsChangedNotTerm(buf_T *buf)
{
    // In a "prompt" buffer we do respect 'modified', so that we can control
    // closing the window by setting or resetting that option.
    return (!bt_dontwrite(buf) || bt_prompt(buf))
	&& (buf->b_changed || file_ff_differs(buf, TRUE));
}

    int
curbufIsChanged(void)
{
    return bufIsChanged(curbuf);
}

#if defined(FEAT_EVAL) || defined(PROTO)

/*
 * For undotree(): Append the list of undo blocks at "first_uhp" to "list".
 * Recursive.
 */
    static void
u_eval_tree(buf_T *buf, u_header_T *first_uhp, list_T *list)
{
    u_header_T  *uhp = first_uhp;
    dict_T	*dict;

    while (uhp != NULL)
    {
	dict = dict_alloc();
	if (dict == NULL)
	    return;
	dict_add_number(dict, "seq", uhp->uh_seq);
	dict_add_number(dict, "time", (long)uhp->uh_time);
	if (uhp == buf->b_u_newhead)
	    dict_add_number(dict, "newhead", 1);
	if (uhp == buf->b_u_curhead)
	    dict_add_number(dict, "curhead", 1);
	if (uhp->uh_save_nr > 0)
	    dict_add_number(dict, "save", uhp->uh_save_nr);

	if (uhp->uh_alt_next.ptr != NULL)
	{
	    list_T	*alt_list = list_alloc();

	    if (alt_list != NULL)
	    {
		// Recursive call to add alternate undo tree.
		u_eval_tree(buf, uhp->uh_alt_next.ptr, alt_list);
		dict_add_list(dict, "alt", alt_list);
	    }
	}

	list_append_dict(list, dict);
	uhp = uhp->uh_prev.ptr;
    }
}

/*
 * "undofile(name)" function
 */
    void
f_undofile(typval_T *argvars UNUSED, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_STRING;
#ifdef FEAT_PERSISTENT_UNDO
    {
	char_u *fname = tv_get_string(&argvars[0]);

	if (*fname == NUL)
	{
	    // If there is no file name there will be no undo file.
	    rettv->vval.v_string = NULL;
	}
	else
	{
	    char_u *ffname = FullName_save(fname, TRUE);

	    if (ffname != NULL)
		rettv->vval.v_string = u_get_undo_file_name(ffname, FALSE);
	    vim_free(ffname);
	}
    }
#else
    rettv->vval.v_string = NULL;
#endif
}
#ifdef FEAT_PERSISTENT_UNDO
/*
 * Reset undofile option and delete the undofile
 */
    void
u_undofile_reset_and_delete(buf_T *buf)
{
    char_u *file_name;

    if (!buf->b_p_udf)
	return;

    file_name = u_get_undo_file_name(buf->b_ffname, TRUE);
    if (file_name != NULL)
    {
	mch_remove(file_name);
	vim_free(file_name);
    }

    set_option_value_give_err((char_u *)"undofile", 0L, NULL, OPT_LOCAL);
}
 #endif

/*
 * "undotree(expr)" function
 */
    void
f_undotree(typval_T *argvars UNUSED, typval_T *rettv)
{
    if (in_vim9script() && check_for_opt_buffer_arg(argvars, 0) == FAIL)
	return;

    if (rettv_dict_alloc(rettv) == FAIL)
	return;

    typval_T	*tv = &argvars[0];
    buf_T	*buf = tv->v_type == VAR_UNKNOWN ? curbuf : get_buf_arg(tv);
    if (buf == NULL)
	return;

    dict_T *dict = rettv->vval.v_dict;

    dict_add_number(dict, "synced", (long)buf->b_u_synced);
    dict_add_number(dict, "seq_last", buf->b_u_seq_last);
    dict_add_number(dict, "save_last", buf->b_u_save_nr_last);
    dict_add_number(dict, "seq_cur", buf->b_u_seq_cur);
    dict_add_number(dict, "time_cur", (long)buf->b_u_time_cur);
    dict_add_number(dict, "save_cur", buf->b_u_save_nr_cur);

    list_T *list = list_alloc();
    if (list != NULL)
    {
	u_eval_tree(buf, buf->b_u_oldhead, list);
	dict_add_list(dict, "entries", list);
    }
}

#endif
