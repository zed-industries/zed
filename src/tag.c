/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * Code to handle tags and the tag stack
 */

#include "vim.h"

/*
 * Structure to hold pointers to various items in a tag line.
 */
typedef struct tag_pointers
{
    // filled in by parse_tag_line():
    char_u	*tagname;	// start of tag name (skip "file:")
    char_u	*tagname_end;	// char after tag name
    char_u	*fname;		// first char of file name
    char_u	*fname_end;	// char after file name
    char_u	*command;	// first char of command
    // filled in by parse_match():
    char_u	*command_end;	// first char after command
    char_u	*tag_fname;	// file name of the tags file. This is used
				// when 'tr' is set.
#ifdef FEAT_EMACS_TAGS
    int		is_etag;	// TRUE for emacs tag
#endif
    char_u	*tagkind;	// "kind:" value
    char_u	*tagkind_end;	// end of tagkind
    char_u	*user_data;	// user_data string
    char_u	*user_data_end;	// end of user_data
    linenr_T	tagline;	// "line:" value
} tagptrs_T;

/*
 * Return values used when reading lines from a tags file.
 */
typedef enum
{
    TAGS_READ_SUCCESS = 1,
    TAGS_READ_EOF,
    TAGS_READ_IGNORE,
} tags_read_status_T;

/*
 * States used during a tags search
 */
typedef enum
{
    TS_START,		// at start of file
    TS_LINEAR,		// linear searching forward, till EOF
    TS_BINARY,		// binary searching
    TS_SKIP_BACK,	// skipping backwards
    TS_STEP_FORWARD	// stepping forwards
} tagsearch_state_T;	// Current search state

/*
 * Binary search file offsets in a tags file
 */
typedef struct
{
    off_T	low_offset;	// offset for first char of first line that
				// could match
    off_T	high_offset;	// offset of char after last line that could
				// match
    off_T	curr_offset;	// Current file offset in search range
    off_T	curr_offset_used; // curr_offset used when skipping back
    off_T	match_offset;	// Where the binary search found a tag
    int	low_char;		// first char at low_offset
    int	high_char;		// first char at high_offset
} tagsearch_info_T;

/*
 * Return values used when matching tags against a pattern.
 */
typedef enum
{
    TAG_MATCH_SUCCESS = 1,
    TAG_MATCH_FAIL,
    TAG_MATCH_STOP,
    TAG_MATCH_NEXT
} tagmatch_status_T;

/*
 * Arguments used for matching tags read from a tags file against a pattern.
 */
typedef struct
{
    int	matchoff;		// tag match offset
    int	match_re;		// TRUE if the tag matches a regexp
    int	match_no_ic;		// TRUE if the tag matches with case
    int	has_re;			// regular expression used
    int	sortic;			// tags file sorted ignoring case (foldcase)
    int	sort_error;		// tags file not sorted
} findtags_match_args_T;

/*
 * The matching tags are first stored in one of the hash tables.  In
 * which one depends on the priority of the match.
 * ht_match[] is used to find duplicates, ga_match[] to keep them in sequence.
 * At the end, all the matches from ga_match[] are concatenated, to make a list
 * sorted on priority.
 */
#define MT_ST_CUR	0		// static match in current file
#define MT_GL_CUR	1		// global match in current file
#define MT_GL_OTH	2		// global match in other file
#define MT_ST_OTH	3		// static match in other file
#define MT_IC_OFF	4		// add for icase match
#define MT_RE_OFF	8		// add for regexp match
#define MT_MASK		7		// mask for printing priority
#define MT_COUNT	16

static char	*mt_names[MT_COUNT/2] =
		{"FSC", "F C", "F  ", "FS ", " SC", "  C", "   ", " S "};

#define NOTAGFILE	99		// return value for jumpto_tag
static char_u	*nofile_fname = NULL;	// fname for NOTAGFILE error

static void taglen_advance(int l);

static int jumpto_tag(char_u *lbuf, int forceit, int keep_help);
#ifdef FEAT_EMACS_TAGS
static int parse_tag_line(char_u *lbuf, int is_etag, tagptrs_T *tagp);
#else
static int parse_tag_line(char_u *lbuf, tagptrs_T *tagp);
#endif
static int test_for_static(tagptrs_T *);
static int parse_match(char_u *lbuf, tagptrs_T *tagp);
static char_u *tag_full_fname(tagptrs_T *tagp);
static char_u *expand_tag_fname(char_u *fname, char_u *tag_fname, int expand);
#ifdef FEAT_EMACS_TAGS
static int test_for_current(int, char_u *, char_u *, char_u *, char_u *);
#else
static int test_for_current(char_u *, char_u *, char_u *, char_u *);
#endif
static int find_extra(char_u **pp);
static void print_tag_list(int new_tag, int use_tagstack, int num_matches, char_u **matches);
#if defined(FEAT_QUICKFIX) && defined(FEAT_EVAL)
static int add_llist_tags(char_u *tag, int num_matches, char_u **matches);
#endif
static void tagstack_clear_entry(taggy_T *item);

static char_u	*tagmatchname = NULL;	// name of last used tag

#if defined(FEAT_QUICKFIX)
/*
 * Tag for preview window is remembered separately, to avoid messing up the
 * normal tagstack.
 */
static taggy_T ptag_entry = {NULL, {{0, 0, 0}, 0}, 0, 0, NULL};
#endif

#ifdef FEAT_EVAL
static int  tfu_in_use = FALSE;	    // disallow recursive call of tagfunc
static callback_T tfu_cb;	    // 'tagfunc' callback function
#endif

// Used instead of NUL to separate tag fields in the growarrays.
#define TAG_SEP 0x02

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Reads the 'tagfunc' option value and convert that to a callback value.
 * Invoked when the 'tagfunc' option is set. The option value can be a name of
 * a function (string), or function(<name>) or funcref(<name>) or a lambda.
 */
    char *
did_set_tagfunc(optset_T *args UNUSED)
{
#ifdef FEAT_EVAL
    free_callback(&tfu_cb);
    free_callback(&curbuf->b_tfu_cb);

    if (*curbuf->b_p_tfu == NUL)
	return NULL;

    if (option_set_callback_func(curbuf->b_p_tfu, &tfu_cb) == FAIL)
	return e_invalid_argument;

    copy_callback(&curbuf->b_tfu_cb, &tfu_cb);
#endif

    return NULL;
}
#endif

# if defined(EXITFREE) || defined(PROTO)
    void
free_tagfunc_option(void)
{
# ifdef FEAT_EVAL
    free_callback(&tfu_cb);
# endif
}
# endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Mark the global 'tagfunc' callback with "copyID" so that it is not garbage
 * collected.
 */
    int
set_ref_in_tagfunc(int copyID UNUSED)
{
    int	abort = FALSE;

    abort = set_ref_in_callback(&tfu_cb, copyID);

    return abort;
}

/*
 * Copy the global 'tagfunc' callback function to the buffer-local 'tagfunc'
 * callback for 'buf'.
 */
    void
set_buflocal_tfu_callback(buf_T *buf UNUSED)
{
    free_callback(&buf->b_tfu_cb);
    if (tfu_cb.cb_name != NULL && *tfu_cb.cb_name != NUL)
	copy_callback(&buf->b_tfu_cb, &tfu_cb);
}
#endif

/*
 * Jump to tag; handling of tag commands and tag stack
 *
 * *tag != NUL: ":tag {tag}", jump to new tag, add to tag stack
 *
 * type == DT_TAG:	":tag [tag]", jump to newer position or same tag again
 * type == DT_HELP:	like DT_TAG, but don't use regexp.
 * type == DT_POP:	":pop" or CTRL-T, jump to old position
 * type == DT_NEXT:	jump to next match of same tag
 * type == DT_PREV:	jump to previous match of same tag
 * type == DT_FIRST:	jump to first match of same tag
 * type == DT_LAST:	jump to last match of same tag
 * type == DT_SELECT:	":tselect [tag]", select tag from a list of all matches
 * type == DT_JUMP:	":tjump [tag]", jump to tag or select tag from a list
 * type == DT_CSCOPE:	use cscope to find the tag
 * type == DT_LTAG:	use location list for displaying tag matches
 * type == DT_FREE:	free cached matches
 *
 * for cscope, returns TRUE if we jumped to tag or aborted, FALSE otherwise
 */
    int
do_tag(
    char_u	*tag,		// tag (pattern) to jump to
    int		type,
    int		count,
    int		forceit,	// :ta with !
    int		verbose)	// print "tag not found" message
{
    taggy_T	*tagstack = curwin->w_tagstack;
    int		tagstackidx = curwin->w_tagstackidx;
    int		tagstacklen = curwin->w_tagstacklen;
    int		cur_match = 0;
    int		cur_fnum = curbuf->b_fnum;
    int		oldtagstackidx = tagstackidx;
    int		prevtagstackidx = tagstackidx;
    int		prev_num_matches;
    int		new_tag = FALSE;
    int		i;
    int		ic;
    int		no_regexp = FALSE;
    int		error_cur_match = 0;
    int		save_pos = FALSE;
    fmark_T	saved_fmark;
#ifdef FEAT_CSCOPE
    int		jumped_to_tag = FALSE;
#endif
    int		new_num_matches;
    char_u	**new_matches;
    int		use_tagstack;
    int		skip_msg = FALSE;
    char_u	*buf_ffname = curbuf->b_ffname;	    // name to use for
						    // priority computation
    int		use_tfu = 1;
    char_u	*tofree = NULL;

    // remember the matches for the last used tag
    static int		num_matches = 0;
    static int		max_num_matches = 0;  // limit used for match search
    static char_u	**matches = NULL;
    static int		flags;

#ifdef FEAT_EVAL
    if (tfu_in_use)
    {
	emsg(_(e_cannot_modify_tag_stack_within_tagfunc));
	return FALSE;
    }
#endif

#ifdef EXITFREE
    if (type == DT_FREE)
    {
	// remove the list of matches
	FreeWild(num_matches, matches);
# ifdef FEAT_CSCOPE
	cs_free_tags();
# endif
	num_matches = 0;
	return FALSE;
    }
#endif

    if (type == DT_HELP)
    {
	type = DT_TAG;
	no_regexp = TRUE;
	use_tfu = 0;
    }

    prev_num_matches = num_matches;
    free_string_option(nofile_fname);
    nofile_fname = NULL;

    CLEAR_POS(&saved_fmark.mark);	// shutup gcc 4.0
    saved_fmark.fnum = 0;

    /*
     * Don't add a tag to the tagstack if 'tagstack' has been reset.
     */
    if ((!p_tgst && *tag != NUL))
    {
	use_tagstack = FALSE;
	new_tag = TRUE;
#if defined(FEAT_QUICKFIX)
	if (g_do_tagpreview != 0)
	{
	    tagstack_clear_entry(&ptag_entry);
	    if ((ptag_entry.tagname = vim_strsave(tag)) == NULL)
		goto end_do_tag;
	}
#endif
    }
    else
    {
#if defined(FEAT_QUICKFIX)
	if (g_do_tagpreview != 0)
	    use_tagstack = FALSE;
	else
#endif
	    use_tagstack = TRUE;

	// new pattern, add to the tag stack
	if (*tag != NUL
		&& (type == DT_TAG || type == DT_SELECT || type == DT_JUMP
#ifdef FEAT_QUICKFIX
		    || type == DT_LTAG
#endif
#ifdef FEAT_CSCOPE
		    || type == DT_CSCOPE
#endif
		    ))
	{
#if defined(FEAT_QUICKFIX)
	    if (g_do_tagpreview != 0)
	    {
		if (ptag_entry.tagname != NULL
			&& STRCMP(ptag_entry.tagname, tag) == 0)
		{
		    // Jumping to same tag: keep the current match, so that
		    // the CursorHold autocommand example works.
		    cur_match = ptag_entry.cur_match;
		    cur_fnum = ptag_entry.cur_fnum;
		}
		else
		{
		    tagstack_clear_entry(&ptag_entry);
		    if ((ptag_entry.tagname = vim_strsave(tag)) == NULL)
			goto end_do_tag;
		}
	    }
	    else
#endif
	    {
		/*
		 * If the last used entry is not at the top, delete all tag
		 * stack entries above it.
		 */
		while (tagstackidx < tagstacklen)
		    tagstack_clear_entry(&tagstack[--tagstacklen]);

		// if the tagstack is full: remove oldest entry
		if (++tagstacklen > TAGSTACKSIZE)
		{
		    tagstacklen = TAGSTACKSIZE;
		    tagstack_clear_entry(&tagstack[0]);
		    for (i = 1; i < tagstacklen; ++i)
			tagstack[i - 1] = tagstack[i];
		    --tagstackidx;
		}

		/*
		 * put the tag name in the tag stack
		 */
		if ((tagstack[tagstackidx].tagname = vim_strsave(tag)) == NULL)
		{
		    curwin->w_tagstacklen = tagstacklen - 1;
		    goto end_do_tag;
		}
		curwin->w_tagstacklen = tagstacklen;

		save_pos = TRUE;	// save the cursor position below
	    }

	    new_tag = TRUE;
	}
	else
	{
	    if (
#if defined(FEAT_QUICKFIX)
		    g_do_tagpreview != 0 ? ptag_entry.tagname == NULL :
#endif
		    tagstacklen == 0)
	    {
		// empty stack
		emsg(_(e_tag_stack_empty));
		goto end_do_tag;
	    }

	    if (type == DT_POP)		// go to older position
	    {
#ifdef FEAT_FOLDING
		int	old_KeyTyped = KeyTyped;
#endif
		if ((tagstackidx -= count) < 0)
		{
		    emsg(_(e_at_bottom_of_tag_stack));
		    if (tagstackidx + count == 0)
		    {
			// We did [num]^T from the bottom of the stack
			tagstackidx = 0;
			goto end_do_tag;
		    }
		    // We weren't at the bottom of the stack, so jump all the
		    // way to the bottom now.
		    tagstackidx = 0;
		}
		else if (tagstackidx >= tagstacklen)    // count == 0?
		{
		    emsg(_(e_at_top_of_tag_stack));
		    goto end_do_tag;
		}

		// Make a copy of the fmark, autocommands may invalidate the
		// tagstack before it's used.
		saved_fmark = tagstack[tagstackidx].fmark;
		if (saved_fmark.fnum != curbuf->b_fnum)
		{
		    /*
		     * Jump to other file. If this fails (e.g. because the
		     * file was changed) keep original position in tag stack.
		     */
		    if (buflist_getfile(saved_fmark.fnum, saved_fmark.mark.lnum,
					       GETF_SETMARK, forceit) == FAIL)
		    {
			tagstackidx = oldtagstackidx;  // back to old posn
			goto end_do_tag;
		    }
		    // An BufReadPost autocommand may jump to the '" mark, but
		    // we don't what that here.
		    curwin->w_cursor.lnum = saved_fmark.mark.lnum;
		}
		else
		{
		    setpcmark();
		    curwin->w_cursor.lnum = saved_fmark.mark.lnum;
		}
		curwin->w_cursor.col = saved_fmark.mark.col;
		curwin->w_set_curswant = TRUE;
		check_cursor();
#ifdef FEAT_FOLDING
		if ((fdo_flags & FDO_TAG) && old_KeyTyped)
		    foldOpenCursor();
#endif

		// remove the old list of matches
		FreeWild(num_matches, matches);
#ifdef FEAT_CSCOPE
		cs_free_tags();
#endif
		num_matches = 0;
		tag_freematch();
		goto end_do_tag;
	    }

	    if (type == DT_TAG
#if defined(FEAT_QUICKFIX)
		    || type == DT_LTAG
#endif
	       )
	    {
#if defined(FEAT_QUICKFIX)
		if (g_do_tagpreview != 0)
		{
		    cur_match = ptag_entry.cur_match;
		    cur_fnum = ptag_entry.cur_fnum;
		}
		else
#endif
		{
		    // ":tag" (no argument): go to newer pattern
		    save_pos = TRUE;	// save the cursor position below
		    if ((tagstackidx += count - 1) >= tagstacklen)
		    {
			/*
			 * Beyond the last one, just give an error message and
			 * go to the last one.  Don't store the cursor
			 * position.
			 */
			tagstackidx = tagstacklen - 1;
			emsg(_(e_at_top_of_tag_stack));
			save_pos = FALSE;
		    }
		    else if (tagstackidx < 0)	// must have been count == 0
		    {
			emsg(_(e_at_bottom_of_tag_stack));
			tagstackidx = 0;
			goto end_do_tag;
		    }
		    cur_match = tagstack[tagstackidx].cur_match;
		    cur_fnum = tagstack[tagstackidx].cur_fnum;
		}
		new_tag = TRUE;
	    }
	    else				// go to other matching tag
	    {
		// Save index for when selection is cancelled.
		prevtagstackidx = tagstackidx;

#if defined(FEAT_QUICKFIX)
		if (g_do_tagpreview != 0)
		{
		    cur_match = ptag_entry.cur_match;
		    cur_fnum = ptag_entry.cur_fnum;
		}
		else
#endif
		{
		    if (--tagstackidx < 0)
			tagstackidx = 0;
		    cur_match = tagstack[tagstackidx].cur_match;
		    cur_fnum = tagstack[tagstackidx].cur_fnum;
		}
		switch (type)
		{
		    case DT_FIRST: cur_match = count - 1; break;
		    case DT_SELECT:
		    case DT_JUMP:
#ifdef FEAT_CSCOPE
		    case DT_CSCOPE:
#endif
		    case DT_LAST:  cur_match = MAXCOL - 1; break;
		    case DT_NEXT:  cur_match += count; break;
		    case DT_PREV:  cur_match -= count; break;
		}
		if (cur_match >= MAXCOL)
		    cur_match = MAXCOL - 1;
		else if (cur_match < 0)
		{
		    emsg(_(e_cannot_go_before_first_matching_tag));
		    skip_msg = TRUE;
		    cur_match = 0;
		    cur_fnum = curbuf->b_fnum;
		}
	    }
	}

#if defined(FEAT_QUICKFIX)
	if (g_do_tagpreview != 0)
	{
	    if (type != DT_SELECT && type != DT_JUMP)
	    {
		ptag_entry.cur_match = cur_match;
		ptag_entry.cur_fnum = cur_fnum;
	    }
	}
	else
#endif
	{
	    /*
	     * For ":tag [arg]" or ":tselect" remember position before the jump.
	     */
	    saved_fmark = tagstack[tagstackidx].fmark;
	    if (save_pos)
	    {
		tagstack[tagstackidx].fmark.mark = curwin->w_cursor;
		tagstack[tagstackidx].fmark.fnum = curbuf->b_fnum;
	    }

	    // Curwin will change in the call to jumpto_tag() if ":stag" was
	    // used or an autocommand jumps to another window; store value of
	    // tagstackidx now.
	    curwin->w_tagstackidx = tagstackidx;
	    if (type != DT_SELECT && type != DT_JUMP)
	    {
		curwin->w_tagstack[tagstackidx].cur_match = cur_match;
		curwin->w_tagstack[tagstackidx].cur_fnum = cur_fnum;
	    }
	}
    }

    // When not using the current buffer get the name of buffer "cur_fnum".
    // Makes sure that the tag order doesn't change when using a remembered
    // position for "cur_match".
    if (cur_fnum != curbuf->b_fnum)
    {
	buf_T *buf = buflist_findnr(cur_fnum);

	if (buf != NULL)
	    buf_ffname = buf->b_ffname;
    }

    /*
     * Repeat searching for tags, when a file has not been found.
     */
    for (;;)
    {
	int	other_name;
	char_u	*name;

	/*
	 * When desired match not found yet, try to find it (and others).
	 */
	if (use_tagstack)
	{
	    // make a copy, the tagstack may change in 'tagfunc'
	    name = vim_strsave(tagstack[tagstackidx].tagname);
	    vim_free(tofree);
	    tofree = name;
	}
#if defined(FEAT_QUICKFIX)
	else if (g_do_tagpreview != 0)
	    name = ptag_entry.tagname;
#endif
	else
	    name = tag;
	other_name = (tagmatchname == NULL || STRCMP(tagmatchname, name) != 0);
	if (new_tag
		|| (cur_match >= num_matches && max_num_matches != MAXCOL)
		|| other_name)
	{
	    if (other_name)
	    {
		vim_free(tagmatchname);
		tagmatchname = vim_strsave(name);
	    }

	    if (type == DT_SELECT || type == DT_JUMP
#if defined(FEAT_QUICKFIX)
		|| type == DT_LTAG
#endif
		)
		cur_match = MAXCOL - 1;
	    if (type == DT_TAG)
		max_num_matches = MAXCOL;
	    else
		max_num_matches = cur_match + 1;

	    // when the argument starts with '/', use it as a regexp
	    if (!no_regexp && *name == '/')
	    {
		flags = TAG_REGEXP;
		++name;
	    }
	    else
		flags = TAG_NOIC;

#ifdef FEAT_CSCOPE
	    if (type == DT_CSCOPE)
		flags = TAG_CSCOPE;
#endif
	    if (verbose)
		flags |= TAG_VERBOSE;

	    if (!use_tfu)
		flags |= TAG_NO_TAGFUNC;

	    if (find_tags(name, &new_num_matches, &new_matches, flags,
					    max_num_matches, buf_ffname) == OK
		    && new_num_matches < max_num_matches)
		max_num_matches = MAXCOL; // If less than max_num_matches
					  // found: all matches found.

	    // A tag function may do anything, which may cause various
	    // information to become invalid.  At least check for the tagstack
	    // to still be the same.
	    if (tagstack != curwin->w_tagstack)
	    {
		emsg(_(e_window_unexpectedly_close_while_searching_for_tags));
		FreeWild(new_num_matches, new_matches);
		break;
	    }

	    // If there already were some matches for the same name, move them
	    // to the start.  Avoids that the order changes when using
	    // ":tnext" and jumping to another file.
	    if (!new_tag && !other_name)
	    {
		int	    j, k;
		int	    idx = 0;
		tagptrs_T   tagp, tagp2;

		// Find the position of each old match in the new list.  Need
		// to use parse_match() to find the tag line.
		for (j = 0; j < num_matches; ++j)
		{
		    parse_match(matches[j], &tagp);
		    for (i = idx; i < new_num_matches; ++i)
		    {
			parse_match(new_matches[i], &tagp2);
			if (STRCMP(tagp.tagname, tagp2.tagname) == 0)
			{
			    char_u *p = new_matches[i];
			    for (k = i; k > idx; --k)
				new_matches[k] = new_matches[k - 1];
			    new_matches[idx++] = p;
			    break;
			}
		    }
		}
	    }
	    FreeWild(num_matches, matches);
	    num_matches = new_num_matches;
	    matches = new_matches;
	}

	if (num_matches <= 0)
	{
	    if (verbose)
		semsg(_(e_tag_not_found_str), name);
#if defined(FEAT_QUICKFIX)
	    g_do_tagpreview = 0;
#endif
	}
	else
	{
	    int ask_for_selection = FALSE;

#ifdef FEAT_CSCOPE
	    if (type == DT_CSCOPE && num_matches > 1)
	    {
		cs_print_tags();
		ask_for_selection = TRUE;
	    }
	    else
#endif
	    if (type == DT_TAG && *tag != NUL)
		// If a count is supplied to the ":tag <name>" command, then
		// jump to count'th matching tag.
		cur_match = count > 0 ? count - 1 : 0;
	    else if (type == DT_SELECT || (type == DT_JUMP && num_matches > 1))
	    {
		print_tag_list(new_tag, use_tagstack, num_matches, matches);
		ask_for_selection = TRUE;
	    }
#if defined(FEAT_QUICKFIX) && defined(FEAT_EVAL)
	    else if (type == DT_LTAG)
	    {
		if (add_llist_tags(tag, num_matches, matches) == FAIL)
		    goto end_do_tag;
		cur_match = 0;		// Jump to the first tag
	    }
#endif

	    if (ask_for_selection == TRUE)
	    {
		/*
		 * Ask to select a tag from the list.
		 */
		i = prompt_for_number(NULL);
		if (i <= 0 || i > num_matches || got_int)
		{
		    // no valid choice: don't change anything
		    if (use_tagstack)
		    {
			tagstack[tagstackidx].fmark = saved_fmark;
			tagstackidx = prevtagstackidx;
		    }
#ifdef FEAT_CSCOPE
		    cs_free_tags();
		    jumped_to_tag = TRUE;
#endif
		    break;
		}
		cur_match = i - 1;
	    }

	    if (cur_match >= num_matches)
	    {
		// Avoid giving this error when a file wasn't found and we're
		// looking for a match in another file, which wasn't found.
		// There will be an emsg("file doesn't exist") below then.
		if ((type == DT_NEXT || type == DT_FIRST)
						      && nofile_fname == NULL)
		{
		    if (num_matches == 1)
			emsg(_(e_there_is_only_one_matching_tag));
		    else
			emsg(_(e_cannot_go_beyond_last_matching_tag));
		    skip_msg = TRUE;
		}
		cur_match = num_matches - 1;
	    }
	    if (use_tagstack)
	    {
		tagptrs_T   tagp;

		tagstack[tagstackidx].cur_match = cur_match;
		tagstack[tagstackidx].cur_fnum = cur_fnum;

		// store user-provided data originating from tagfunc
		if (use_tfu && parse_match(matches[cur_match], &tagp) == OK
			&& tagp.user_data)
		{
		    VIM_CLEAR(tagstack[tagstackidx].user_data);
		    tagstack[tagstackidx].user_data = vim_strnsave(
			  tagp.user_data, tagp.user_data_end - tagp.user_data);
		}

		++tagstackidx;
	    }
#if defined(FEAT_QUICKFIX)
	    else if (g_do_tagpreview != 0)
	    {
		ptag_entry.cur_match = cur_match;
		ptag_entry.cur_fnum = cur_fnum;
	    }
#endif

	    /*
	     * Only when going to try the next match, report that the previous
	     * file didn't exist.  Otherwise an emsg() is given below.
	     */
	    if (nofile_fname != NULL && error_cur_match != cur_match)
		smsg(_("File \"%s\" does not exist"), nofile_fname);


	    ic = (matches[cur_match][0] & MT_IC_OFF);
	    if (type != DT_TAG && type != DT_SELECT && type != DT_JUMP
#ifdef FEAT_CSCOPE
		&& type != DT_CSCOPE
#endif
		&& (num_matches > 1 || ic)
		&& !skip_msg)
	    {
		// Give an indication of the number of matching tags
		sprintf((char *)IObuff, _("tag %d of %d%s"),
				cur_match + 1,
				num_matches,
				max_num_matches != MAXCOL ? _(" or more") : "");
		if (ic)
		    STRCAT(IObuff, _("  Using tag with different case!"));
		if ((num_matches > prev_num_matches || new_tag)
							   && num_matches > 1)
		{
		    if (ic)
			msg_attr((char *)IObuff, HL_ATTR(HLF_W));
		    else
			msg((char *)IObuff);
		    msg_scroll = TRUE;	// don't overwrite this message
		}
		else
		    give_warning(IObuff, ic);
		if (ic && !msg_scrolled && msg_silent == 0)
		{
		    out_flush();
		    ui_delay(1007L, TRUE);
		}
	    }

#if defined(FEAT_EVAL)
	    // Let the SwapExists event know what tag we are jumping to.
	    vim_snprintf((char *)IObuff, IOSIZE, ":ta %s\r", name);
	    set_vim_var_string(VV_SWAPCOMMAND, IObuff, -1);
#endif

	    /*
	     * Jump to the desired match.
	     */
	    i = jumpto_tag(matches[cur_match], forceit, type != DT_CSCOPE);

#if defined(FEAT_EVAL)
	    set_vim_var_string(VV_SWAPCOMMAND, NULL, -1);
#endif

	    if (i == NOTAGFILE)
	    {
		// File not found: try again with another matching tag
		if ((type == DT_PREV && cur_match > 0)
			|| ((type == DT_TAG || type == DT_NEXT
							  || type == DT_FIRST)
			    && (max_num_matches != MAXCOL
					     || cur_match < num_matches - 1)))
		{
		    error_cur_match = cur_match;
		    if (use_tagstack)
			--tagstackidx;
		    if (type == DT_PREV)
			--cur_match;
		    else
		    {
			type = DT_NEXT;
			++cur_match;
		    }
		    continue;
		}
		semsg(_(e_file_str_does_not_exist), nofile_fname);
	    }
	    else
	    {
		// We may have jumped to another window, check that
		// tagstackidx is still valid.
		if (use_tagstack && tagstackidx > curwin->w_tagstacklen)
		    tagstackidx = curwin->w_tagstackidx;
#ifdef FEAT_CSCOPE
		jumped_to_tag = TRUE;
#endif
	    }
	}
	break;
    }

end_do_tag:
    // Only store the new index when using the tagstack and it's valid.
    if (use_tagstack && tagstackidx <= curwin->w_tagstacklen)
	curwin->w_tagstackidx = tagstackidx;
    postponed_split = 0;	// don't split next time
# ifdef FEAT_QUICKFIX
    g_do_tagpreview = 0;	// don't do tag preview next time
# endif

    vim_free(tofree);
#ifdef FEAT_CSCOPE
    return jumped_to_tag;
#else
    return FALSE;
#endif
}

/*
 * List all the matching tags.
 */
    static void
print_tag_list(
    int		new_tag,
    int		use_tagstack,
    int		num_matches,
    char_u	**matches)
{
    taggy_T	*tagstack = curwin->w_tagstack;
    int		tagstackidx = curwin->w_tagstackidx;
    int		i;
    char_u	*p;
    char_u	*command_end;
    tagptrs_T	tagp;
    int		taglen;
    int		attr;

    /*
     * Assume that the first match indicates how long the tags can
     * be, and align the file names to that.
     */
    parse_match(matches[0], &tagp);
    taglen = (int)(tagp.tagname_end - tagp.tagname + 2);
    if (taglen < 18)
	taglen = 18;
    if (taglen > Columns - 25)
	taglen = MAXCOL;
    if (msg_col == 0)
	msg_didout = FALSE;	// overwrite previous message
    msg_start();
    msg_puts_attr(_("  # pri kind tag"), HL_ATTR(HLF_T));
    msg_clr_eos();
    taglen_advance(taglen);
    msg_puts_attr(_("file\n"), HL_ATTR(HLF_T));

    for (i = 0; i < num_matches && !got_int; ++i)
    {
	parse_match(matches[i], &tagp);
	if (!new_tag && (
#if defined(FEAT_QUICKFIX)
		    (g_do_tagpreview != 0
		     && i == ptag_entry.cur_match) ||
#endif
		    (use_tagstack
		     && i == tagstack[tagstackidx].cur_match)))
	    *IObuff = '>';
	else
	    *IObuff = ' ';
	vim_snprintf((char *)IObuff + 1, IOSIZE - 1,
		"%2d %s ", i + 1,
			       mt_names[matches[i][0] & MT_MASK]);
	msg_puts((char *)IObuff);
	if (tagp.tagkind != NULL)
	    msg_outtrans_len(tagp.tagkind,
			  (int)(tagp.tagkind_end - tagp.tagkind));
	msg_advance(13);
	msg_outtrans_len_attr(tagp.tagname,
			   (int)(tagp.tagname_end - tagp.tagname),
						  HL_ATTR(HLF_T));
	msg_putchar(' ');
	taglen_advance(taglen);

	// Find out the actual file name. If it is long, truncate
	// it and put "..." in the middle
	p = tag_full_fname(&tagp);
	if (p != NULL)
	{
	    msg_outtrans_long_attr(p, HL_ATTR(HLF_D));
	    vim_free(p);
	}
	if (msg_col > 0)
	    msg_putchar('\n');
	if (got_int)
	    break;
	msg_advance(15);

	// print any extra fields
	command_end = tagp.command_end;
	if (command_end != NULL)
	{
	    p = command_end + 3;
	    while (*p && *p != '\r' && *p != '\n')
	    {
		while (*p == TAB)
		    ++p;

		// skip "file:" without a value (static tag)
		if (STRNCMP(p, "file:", 5) == 0
					     && vim_isspace(p[5]))
		{
		    p += 5;
		    continue;
		}
		// skip "kind:<kind>" and "<kind>"
		if (p == tagp.tagkind
			|| (p + 5 == tagp.tagkind
				&& STRNCMP(p, "kind:", 5) == 0))
		{
		    p = tagp.tagkind_end;
		    continue;
		}
		// print all other extra fields
		attr = HL_ATTR(HLF_CM);
		while (*p && *p != '\r' && *p != '\n')
		{
		    if (msg_col + ptr2cells(p) >= Columns)
		    {
			msg_putchar('\n');
			if (got_int)
			    break;
			msg_advance(15);
		    }
		    p = msg_outtrans_one(p, attr);
		    if (*p == TAB)
		    {
			msg_puts_attr(" ", attr);
			break;
		    }
		    if (*p == ':')
			attr = 0;
		}
	    }
	    if (msg_col > 15)
	    {
		msg_putchar('\n');
		if (got_int)
		    break;
		msg_advance(15);
	    }
	}
	else
	{
	    for (p = tagp.command;
			      *p && *p != '\r' && *p != '\n'; ++p)
		;
	    command_end = p;
	}

	// Put the info (in several lines) at column 15.
	// Don't display "/^" and "?^".
	p = tagp.command;
	if (*p == '/' || *p == '?')
	{
	    ++p;
	    if (*p == '^')
		++p;
	}
	// Remove leading whitespace from pattern
	while (p != command_end && vim_isspace(*p))
	    ++p;

	while (p != command_end)
	{
	    if (msg_col + (*p == TAB ? 1 : ptr2cells(p)) > Columns)
		msg_putchar('\n');
	    if (got_int)
		break;
	    msg_advance(15);

	    // skip backslash used for escaping a command char or
	    // a backslash
	    if (*p == '\\' && (*(p + 1) == *tagp.command
			    || *(p + 1) == '\\'))
		++p;

	    if (*p == TAB)
	    {
		msg_putchar(' ');
		++p;
	    }
	    else
		p = msg_outtrans_one(p, 0);

	    // don't display the "$/;\"" and "$?;\""
	    if (p == command_end - 2 && *p == '$'
				     && *(p + 1) == *tagp.command)
		break;
	    // don't display matching '/' or '?'
	    if (p == command_end - 1 && *p == *tagp.command
				     && (*p == '/' || *p == '?'))
		break;
	}
	if (msg_col)
	    msg_putchar('\n');
	ui_breakcheck();
    }
    if (got_int)
	got_int = FALSE;	// only stop the listing
}

#if defined(FEAT_QUICKFIX) && defined(FEAT_EVAL)
/*
 * Add the matching tags to the location list for the current
 * window.
 */
    static int
add_llist_tags(
    char_u	*tag,
    int		num_matches,
    char_u	**matches)
{
    list_T	*list;
    char_u	tag_name[128 + 1];
    char_u	*fname;
    char_u	*cmd;
    int		i;
    char_u	*p;
    tagptrs_T	tagp;

    fname = alloc(MAXPATHL + 1);
    cmd = alloc(CMDBUFFSIZE + 1);
    list = list_alloc();
    if (list == NULL || fname == NULL || cmd == NULL)
    {
	vim_free(cmd);
	vim_free(fname);
	if (list != NULL)
	    list_free(list);
	return FAIL;
    }

    for (i = 0; i < num_matches; ++i)
    {
	int	    len, cmd_len;
	long    lnum;
	dict_T  *dict;

	parse_match(matches[i], &tagp);

	// Save the tag name
	len = (int)(tagp.tagname_end - tagp.tagname);
	if (len > 128)
	    len = 128;
	vim_strncpy(tag_name, tagp.tagname, len);
	tag_name[len] = NUL;

	// Save the tag file name
	p = tag_full_fname(&tagp);
	if (p == NULL)
	    continue;
	vim_strncpy(fname, p, MAXPATHL);
	vim_free(p);

	// Get the line number or the search pattern used to locate
	// the tag.
	lnum = 0;
	if (SAFE_isdigit(*tagp.command))
	    // Line number is used to locate the tag
	    lnum = atol((char *)tagp.command);
	else
	{
	    char_u *cmd_start, *cmd_end;

	    // Search pattern is used to locate the tag

	    // Locate the end of the command
	    cmd_start = tagp.command;
	    cmd_end = tagp.command_end;
	    if (cmd_end == NULL)
	    {
		for (p = tagp.command;
		     *p && *p != '\r' && *p != '\n'; ++p)
		    ;
		cmd_end = p;
	    }

	    // Now, cmd_end points to the character after the
	    // command. Adjust it to point to the last
	    // character of the command.
	    cmd_end--;

	    // Skip the '/' and '?' characters at the
	    // beginning and end of the search pattern.
	    if (*cmd_start == '/' || *cmd_start == '?')
		cmd_start++;

	    if (*cmd_end == '/' || *cmd_end == '?')
		cmd_end--;

	    len = 0;
	    cmd[0] = NUL;

	    // If "^" is present in the tag search pattern, then
	    // copy it first.
	    if (*cmd_start == '^')
	    {
		STRCPY(cmd, "^");
		cmd_start++;
		len++;
	    }

	    // Precede the tag pattern with \V to make it very
	    // nomagic.
	    STRCAT(cmd, "\\V");
	    len += 2;

	    cmd_len = (int)(cmd_end - cmd_start + 1);
	    if (cmd_len > (CMDBUFFSIZE - 5))
		cmd_len = CMDBUFFSIZE - 5;
	    STRNCAT(cmd, cmd_start, cmd_len);
	    len += cmd_len;

	    if (cmd[len - 1] == '$')
	    {
		// Replace '$' at the end of the search pattern
		// with '\$'
		cmd[len - 1] = '\\';
		cmd[len] = '$';
		len++;
	    }

	    cmd[len] = NUL;
	}

	if ((dict = dict_alloc()) == NULL)
	    continue;
	if (list_append_dict(list, dict) == FAIL)
	{
	    vim_free(dict);
	    continue;
	}

	dict_add_string(dict, "text", tag_name);
	dict_add_string(dict, "filename", fname);
	dict_add_number(dict, "lnum", lnum);
	if (lnum == 0)
	    dict_add_string(dict, "pattern", cmd);
    }

    vim_snprintf((char *)IObuff, IOSIZE, "ltag %s", tag);
    set_errorlist(curwin, list, ' ', IObuff, NULL);

    list_free(list);
    vim_free(fname);
    vim_free(cmd);

    return OK;
}
#endif

/*
 * Free cached tags.
 */
    void
tag_freematch(void)
{
    VIM_CLEAR(tagmatchname);
}

    static void
taglen_advance(int l)
{
    if (l == MAXCOL)
    {
	msg_putchar('\n');
	msg_advance(24);
    }
    else
	msg_advance(13 + l);
}

/*
 * Print the tag stack
 */
    void
do_tags(exarg_T *eap UNUSED)
{
    int		i;
    char_u	*name;
    taggy_T	*tagstack = curwin->w_tagstack;
    int		tagstackidx = curwin->w_tagstackidx;
    int		tagstacklen = curwin->w_tagstacklen;

    // Highlight title
    msg_puts_title(_("\n  # TO tag         FROM line  in file/text"));
    for (i = 0; i < tagstacklen; ++i)
    {
	if (tagstack[i].tagname != NULL)
	{
	    name = fm_getname(&(tagstack[i].fmark), 30);
	    if (name == NULL)	    // file name not available
		continue;

	    msg_putchar('\n');
	    vim_snprintf((char *)IObuff, IOSIZE, "%c%2d %2d %-15s %5ld  ",
		i == tagstackidx ? '>' : ' ',
		i + 1,
		tagstack[i].cur_match + 1,
		tagstack[i].tagname,
		tagstack[i].fmark.mark.lnum);
	    msg_outtrans(IObuff);
	    msg_outtrans_attr(name, tagstack[i].fmark.fnum == curbuf->b_fnum
							? HL_ATTR(HLF_D) : 0);
	    vim_free(name);
	}
	out_flush();		    // show one line at a time
    }
    if (tagstackidx == tagstacklen)	// idx at top of stack
	msg_puts("\n>");
}

/*
 * Compare two strings, for length "len", ignoring case the ASCII way.
 * return 0 for match, < 0 for smaller, > 0 for bigger
 * Make sure case is folded to uppercase in comparison (like for 'sort -f')
 */
    static int
tag_strnicmp(char_u *s1, char_u *s2, size_t len)
{
    int		i;

    while (len > 0)
    {
	i = (int)TOUPPER_ASC(*s1) - (int)TOUPPER_ASC(*s2);
	if (i != 0)
	    return i;			// this character different
	if (*s1 == NUL)
	    break;			// strings match until NUL
	++s1;
	++s2;
	--len;
    }
    return 0;				// strings match
}

/*
 * Structure to hold info about the tag pattern being used.
 */
typedef struct
{
    char_u	*pat;		// the pattern
    int		len;		// length of pat[]
    char_u	*head;		// start of pattern head
    int		headlen;	// length of head[]
    regmatch_T	regmatch;	// regexp program, may be NULL
} pat_T;

/*
 * Extract info from the tag search pattern "pats->pat".
 */
    static void
prepare_pats(pat_T *pats, int has_re)
{
    pats->head = pats->pat;
    pats->headlen = pats->len;
    if (has_re)
    {
	// When the pattern starts with '^' or "\\<", binary searching can be
	// used (much faster).
	if (pats->pat[0] == '^')
	    pats->head = pats->pat + 1;
	else if (pats->pat[0] == '\\' && pats->pat[1] == '<')
	    pats->head = pats->pat + 2;
	if (pats->head == pats->pat)
	    pats->headlen = 0;
	else
	    for (pats->headlen = 0; pats->head[pats->headlen] != NUL;
							      ++pats->headlen)
		if (vim_strchr((char_u *)(magic_isset() ? ".[~*\\$" : "\\$"),
					   pats->head[pats->headlen]) != NULL)
		    break;
	if (p_tl != 0 && pats->headlen > p_tl)	// adjust for 'taglength'
	    pats->headlen = p_tl;
    }

    if (has_re)
	pats->regmatch.regprog = vim_regcomp(pats->pat,
						 magic_isset() ? RE_MAGIC : 0);
    else
	pats->regmatch.regprog = NULL;
}

#ifdef FEAT_EVAL
/*
 * Call the user-defined function to generate a list of tags used by
 * find_tags().
 *
 * Return OK if at least 1 tag has been successfully found,
 * NOTDONE if the function returns v:null, and FAIL otherwise.
 */
    static int
find_tagfunc_tags(
    char_u	*pat,		// pattern supplied to the user-defined function
    garray_T	*ga,		// the tags will be placed here
    int		*match_count,	// here the number of tags found will be placed
    int		flags,		// flags from find_tags (TAG_*)
    char_u	*buf_ffname)	// name of buffer for priority
{
    pos_T       save_pos;
    list_T      *taglist;
    listitem_T  *item;
    int		ntags = 0;
    int		result = FAIL;
    typval_T	args[4];
    typval_T	rettv;
    char_u      flagString[4];
    dict_T	*d;
    taggy_T	*tag = &curwin->w_tagstack[curwin->w_tagstackidx];

    if (*curbuf->b_p_tfu == NUL || curbuf->b_tfu_cb.cb_name == NULL
					   || *curbuf->b_tfu_cb.cb_name == NUL)
	return FAIL;

    args[0].v_type = VAR_STRING;
    args[0].vval.v_string = pat;
    args[1].v_type = VAR_STRING;
    args[1].vval.v_string = flagString;

    // create 'info' dict argument
    if ((d = dict_alloc_lock(VAR_FIXED)) == NULL)
	return FAIL;
    if (tag->user_data != NULL)
	dict_add_string(d, "user_data", tag->user_data);
    if (buf_ffname != NULL)
	dict_add_string(d, "buf_ffname", buf_ffname);

    ++d->dv_refcount;
    args[2].v_type = VAR_DICT;
    args[2].vval.v_dict = d;

    args[3].v_type = VAR_UNKNOWN;

    vim_snprintf((char *)flagString, sizeof(flagString),
		 "%s%s%s",
		 g_tag_at_cursor      ? "c": "",
		 flags & TAG_INS_COMP ? "i": "",
		 flags & TAG_REGEXP   ? "r": "");

    save_pos = curwin->w_cursor;
    result = call_callback(&curbuf->b_tfu_cb, 0, &rettv, 3, args);
    curwin->w_cursor = save_pos;	// restore the cursor position
    --d->dv_refcount;

    if (result == FAIL)
	return FAIL;
    if (rettv.v_type == VAR_SPECIAL && rettv.vval.v_number == VVAL_NULL)
    {
	clear_tv(&rettv);
	return NOTDONE;
    }
    if (rettv.v_type != VAR_LIST || !rettv.vval.v_list)
    {
	clear_tv(&rettv);
	emsg(_(e_invalid_return_value_from_tagfunc));
	return FAIL;
    }
    taglist = rettv.vval.v_list;

    FOR_ALL_LIST_ITEMS(taglist, item)
    {
	char_u		*mfp;
	char_u		*res_name, *res_fname, *res_cmd, *res_kind;
	int		len;
	dict_iterator_T	iter;
	char_u		*dict_key;
	typval_T	*tv;
	int		has_extra = 0;
	int		name_only = flags & TAG_NAMES;

	if (item->li_tv.v_type != VAR_DICT)
	{
	    emsg(_(e_invalid_return_value_from_tagfunc));
	    break;
	}

#ifdef FEAT_EMACS_TAGS
	len = 3;
#else
	len = 2;
#endif
	res_name = NULL;
	res_fname = NULL;
	res_cmd = NULL;
	res_kind = NULL;

	dict_iterate_start(&item->li_tv, &iter);
	while (NULL != (dict_key = dict_iterate_next(&iter, &tv)))
	{
	    if (tv->v_type != VAR_STRING || tv->vval.v_string == NULL)
		continue;

	    len += (int)STRLEN(tv->vval.v_string) + 1;   // Space for "\tVALUE"
	    if (!STRCMP(dict_key, "name"))
	    {
		res_name = tv->vval.v_string;
		continue;
	    }
	    if (!STRCMP(dict_key, "filename"))
	    {
		res_fname = tv->vval.v_string;
		continue;
	    }
	    if (!STRCMP(dict_key, "cmd"))
	    {
		res_cmd = tv->vval.v_string;
		continue;
	    }
	    has_extra = 1;
	    if (!STRCMP(dict_key, "kind"))
	    {
		res_kind = tv->vval.v_string;
		continue;
	    }
	    // Other elements will be stored as "\tKEY:VALUE"
	    // Allocate space for the key and the colon
	    len += (int)STRLEN(dict_key) + 1;
	}

	if (has_extra)
	    len += 2;	// need space for ;"

	if (!res_name || !res_fname || !res_cmd)
	{
	    emsg(_(e_invalid_return_value_from_tagfunc));
	    break;
	}

	if (name_only)
	    mfp = vim_strsave(res_name);
	else
	    mfp = alloc(sizeof(char_u) + len + 1);

	if (mfp == NULL)
	    continue;

	if (!name_only)
	{
	    char_u *p = mfp;

	    *p++ = MT_GL_OTH + 1;   // mtt
	    *p++ = TAG_SEP;	    // no tag file name
#ifdef FEAT_EMACS_TAGS
	    *p++ = TAG_SEP;
#endif

	    STRCPY(p, res_name);
	    p += STRLEN(p);

	    *p++ = TAB;
	    STRCPY(p, res_fname);
	    p += STRLEN(p);

	    *p++ = TAB;
	    STRCPY(p, res_cmd);
	    p += STRLEN(p);

	    if (has_extra)
	    {
		STRCPY(p, ";\"");
		p += STRLEN(p);

		if (res_kind)
		{
		    *p++ = TAB;
		    STRCPY(p, res_kind);
		    p += STRLEN(p);
		}

		dict_iterate_start(&item->li_tv, &iter);
		while (NULL != (dict_key = dict_iterate_next(&iter, &tv)))
		{
		    if (tv->v_type != VAR_STRING || tv->vval.v_string == NULL)
			continue;

		    if (!STRCMP(dict_key, "name"))
			continue;
		    if (!STRCMP(dict_key, "filename"))
			continue;
		    if (!STRCMP(dict_key, "cmd"))
			continue;
		    if (!STRCMP(dict_key, "kind"))
			continue;

		    *p++ = TAB;
		    STRCPY(p, dict_key);
		    p += STRLEN(p);
		    STRCPY(p, ":");
		    p += STRLEN(p);
		    STRCPY(p, tv->vval.v_string);
		    p += STRLEN(p);
		}
	    }
	}

	// Add all matches because tagfunc should do filtering.
	if (ga_grow(ga, 1) == OK)
	{
	    ((char_u **)(ga->ga_data))[ga->ga_len++] = mfp;
	    ++ntags;
	    result = OK;
	}
	else
	{
	    vim_free(mfp);
	    break;
	}
    }

    clear_tv(&rettv);

    *match_count = ntags;
    return result;
}
#endif

/*
 * State information used during a tag search
 */
typedef struct
{
    tagsearch_state_T	state;		// tag search state
    int		stop_searching;		// stop when match found or error
    pat_T	*orgpat;		// holds unconverted pattern info
    char_u     *lbuf;			// line buffer
    int		lbuf_size;		// length of lbuf
    char_u	*tag_fname;		// name of the tag file
    FILE	*fp;			// current tags file pointer
    int		flags;			// flags used for tag search
    int		tag_file_sorted;	// !_TAG_FILE_SORTED value
    int		get_searchpat;		// used for 'showfulltag'
    int		help_only;		// only search for help tags
    int		did_open;		// did open a tag file
    int		mincount;		// MAXCOL: find all matches
					// other: minimal number of matches
    int		linear;			// do a linear search
    vimconv_T	vimconv;
#ifdef FEAT_EMACS_TAGS
    int		is_etag;		// current file is emacs style
    char_u	*ebuf;			// additional buffer for etag fname
#endif
#ifdef FEAT_MULTI_LANG
    char_u	help_lang[3];		// lang of current tags file
    int		help_pri;		// help language priority
    char_u	*help_lang_find;	// lang to be found
    int		is_txt;			// flag of file extension
#endif
    int		match_count;		// number of matches found
    garray_T	ga_match[MT_COUNT];	// stores matches in sequence
    hashtab_T	ht_match[MT_COUNT];	// stores matches by key
} findtags_state_T;

/*
 * Initialize the state used by find_tags().
 * Returns OK on success and FAIL on memory allocation failure.
 */
    static int
findtags_state_init(
    findtags_state_T	*st,
    char_u		*pat,
    int			flags,
    int			mincount)
{
    int		mtt;

    st->tag_fname = alloc(MAXPATHL + 1);
    st->fp = NULL;
    st->orgpat = ALLOC_ONE(pat_T);
    st->orgpat->pat = pat;
    st->orgpat->len = (int)STRLEN(pat);
    st->orgpat->regmatch.regprog = NULL;
    st->flags = flags;
    st->tag_file_sorted = NUL;
    st->help_only = (flags & TAG_HELP);
    st->get_searchpat = FALSE;
#ifdef FEAT_MULTI_LANG
    st->help_lang[0] = NUL;
    st->help_pri = 0;
    st->help_lang_find = NULL;
    st->is_txt = FALSE;
#endif
    st->did_open = FALSE;
    st->mincount = mincount;
    st->lbuf_size = LSIZE;
    st->lbuf = alloc(st->lbuf_size);
#ifdef FEAT_EMACS_TAGS
    st->ebuf = alloc(LSIZE);
#endif
    st->match_count = 0;
    st->stop_searching = FALSE;

    for (mtt = 0; mtt < MT_COUNT; ++mtt)
    {
	ga_init2(&st->ga_match[mtt], sizeof(char_u *), 100);
	hash_init(&st->ht_match[mtt]);
    }

    // check for out of memory situation
    if (st->tag_fname == NULL
	    || st->lbuf == NULL
#ifdef FEAT_EMACS_TAGS
	    || st->ebuf == NULL
#endif
       )
	return FAIL;

    return OK;
}

/*
 * Free the state used by find_tags()
 */
    static void
findtags_state_free(findtags_state_T *st)
{
    vim_free(st->tag_fname);
    vim_free(st->lbuf);
    vim_regfree(st->orgpat->regmatch.regprog);
    vim_free(st->orgpat);
#ifdef FEAT_EMACS_TAGS
    vim_free(st->ebuf);
#endif
}

#ifdef FEAT_MULTI_LANG
/*
 * Initialize the language and priority used for searching tags in a Vim help
 * file.
 * Returns TRUE to process the help file for tags and FALSE to skip the file.
 */
    static int
findtags_in_help_init(findtags_state_T *st)
{
    int		i;
    char_u	*s;

    // Keep "en" as the language if the file extension is ".txt"
    if (st->is_txt)
	STRCPY(st->help_lang, "en");
    else
    {
	// Prefer help tags according to 'helplang'.  Put the two-letter
	// language name in help_lang[].
	i = (int)STRLEN(st->tag_fname);
	if (i > 3 && st->tag_fname[i - 3] == '-')
	    vim_strncpy(st->help_lang, st->tag_fname + i - 2, 2);
	else
	    STRCPY(st->help_lang, "en");
    }
    // When searching for a specific language skip tags files for other
    // languages.
    if (st->help_lang_find != NULL
	    && STRICMP(st->help_lang, st->help_lang_find) != 0)
	return FALSE;

    // For CTRL-] in a help file prefer a match with the same language.
    if ((st->flags & TAG_KEEP_LANG)
	    && st->help_lang_find == NULL
	    && curbuf->b_fname != NULL
	    && (i = (int)STRLEN(curbuf->b_fname)) > 4
	    && curbuf->b_fname[i - 1] == 'x'
	    && curbuf->b_fname[i - 4] == '.'
	    && STRNICMP(curbuf->b_fname + i - 3, st->help_lang, 2) == 0)
	st->help_pri = 0;
    else
    {
	// search for the language in 'helplang'
	st->help_pri = 1;
	for (s = p_hlg; *s != NUL; ++s)
	{
	    if (STRNICMP(s, st->help_lang, 2) == 0)
		break;
	    ++st->help_pri;
	    if ((s = vim_strchr(s, ',')) == NULL)
		break;
	}
	if (s == NULL || *s == NUL)
	{
	    // Language not in 'helplang': use last, prefer English, unless
	    // found already.
	    ++st->help_pri;
	    if (STRICMP(st->help_lang, "en") != 0)
		++st->help_pri;
	}
    }

    return TRUE;
}
#endif

#ifdef FEAT_EVAL
/*
 * Use the function set in 'tagfunc' (if configured and enabled) to get the
 * tags.
 * Return OK if at least 1 tag has been successfully found, NOTDONE if the
 * 'tagfunc' is not used or the 'tagfunc' returns v:null and FAIL otherwise.
 */
    static int
findtags_apply_tfu(findtags_state_T *st, char_u *pat, char_u *buf_ffname)
{
    int		use_tfu = ((st->flags & TAG_NO_TAGFUNC) == 0);
    int		retval;

    if (!use_tfu || tfu_in_use || *curbuf->b_p_tfu == NUL)
	return NOTDONE;

    tfu_in_use = TRUE;
    retval = find_tagfunc_tags(pat, st->ga_match, &st->match_count,
						st->flags, buf_ffname);
    tfu_in_use = FALSE;

    return retval;
}
#endif

#ifdef FEAT_EMACS_TAGS
/*
 * Stack for included emacs-tags file.
 * It has a fixed size, to truncate cyclic includes. jw
 */
# define INCSTACK_SIZE 42
static struct
{
    FILE	*fp;
    char_u	*etag_fname;
} incstack[INCSTACK_SIZE];
static int incstack_idx = 0;	// index in incstack

/*
 * Free the emacs include tags file stack.
 */
    static void
emacs_tags_incstack_free(void)
{
    while (incstack_idx)
    {
	--incstack_idx;
	fclose(incstack[incstack_idx].fp);
	incstack[incstack_idx].fp = NULL;
	VIM_CLEAR(incstack[incstack_idx].etag_fname);
    }
}

/*
 * Emacs tags line with CTRL-L: New file name on next line.
 * The file name is followed by a ','.  Remember etag file name in ebuf.
 * The FILE pointer to the tags file is stored in "st->fp".  If another tags
 * file is included, then the FILE pointer to the new tags file is stored in
 * "st->fp". The old file pointer is saved in incstack.
 */
    static void
emacs_tags_new_filename(findtags_state_T *st)
{
    char_u	*p;
    char_u	*fullpath_ebuf;

    if (vim_fgets(st->ebuf, LSIZE, st->fp))
	return;

    for (p = st->ebuf; *p && *p != ','; p++)
	;
    *p = NUL;

    // check for an included tags file.
    // atoi(p+1) is the number of bytes before the next ^L unless it is an
    // include statement. Skip the included tags file if it exceeds the
    // maximum.
    if (STRNCMP(p + 1, "include", 7) != 0 || incstack_idx >= INCSTACK_SIZE)
	return;

    // Save current "fp" and "tag_fname" in the stack.
    incstack[incstack_idx].etag_fname = vim_strsave(st->tag_fname);
    if (incstack[incstack_idx].etag_fname == NULL)
	return;

    incstack[incstack_idx].fp = st->fp;
    st->fp = NULL;

    // Figure out "tag_fname" and "fp" to use for
    // included file.
    fullpath_ebuf = expand_tag_fname(st->ebuf, st->tag_fname, FALSE);
    if (fullpath_ebuf != NULL)
    {
	st->fp = mch_fopen((char *)fullpath_ebuf, "r");
	if (st->fp != NULL)
	{
	    if (STRLEN(fullpath_ebuf) > LSIZE)
		semsg(_(e_tag_file_path_truncated_for_str), st->ebuf);
	    vim_strncpy(st->tag_fname, fullpath_ebuf, MAXPATHL);
	    ++incstack_idx;
	    st->is_etag = FALSE; // we can include anything
	}
	vim_free(fullpath_ebuf);
    }
    if (st->fp == NULL)
    {
	// Can't open the included file, skip it and
	// restore old value of "fp".
	st->fp = incstack[incstack_idx].fp;
	vim_free(incstack[incstack_idx].etag_fname);
    }
}

/*
 * Reached the end of an emacs-style tags file. If this is an included tags
 * file, then pop it from the incstack and continue processing the parent tags
 * file. Otherwise, processed all the tags.
 * Returns TRUE if an included tags file is popped and processing should
 * continue with the parent tags file. Returns FALSE to stop processing tags.
 */
    static int
emacs_tags_file_eof(findtags_state_T *st)
{
    if (!incstack_idx)	// reached end of file. stop processing.
	return FALSE;

    // reached the end of an included tags file. pop it.
    --incstack_idx;
    fclose(st->fp);	// end of this file ...
    st->fp = incstack[incstack_idx].fp;
    STRCPY(st->tag_fname, incstack[incstack_idx].etag_fname);
    vim_free(incstack[incstack_idx].etag_fname);
    st->is_etag = TRUE;	// (only etags can include)

    return TRUE;
}

/*
 * Parse a line from an emacs-style tags file.
 * Returns OK if the line is parsed successfully, returns FAIL if the line is
 * not terminated by a newline.
 */
    static int
emacs_tags_parse_line(char_u *lbuf, tagptrs_T *tagp)
{
    char_u	*p_7f;
    char_u	*p;

    // There are two formats for an emacs tag line:
    // 1:  struct EnvBase ^?EnvBase^A139,4627
    // 2: #define	ARPB_WILD_WORLD ^?153,5194
    p_7f = vim_strchr(lbuf, 0x7f);
    if (p_7f == NULL)
    {
etag_fail:
	if (vim_strchr(lbuf, '\n') != NULL)
	    return FAIL;

	// Truncated line.  Ignore it.
	if (p_verbose >= 5)
	{
	    verbose_enter();
	    msg(_("Ignoring long line in tags file"));
	    verbose_leave();
	}
	tagp->command = lbuf;
	tagp->tagname = lbuf;
	tagp->tagname_end = lbuf;
	return OK;
    }

    // Find ^A.  If not found the line number is after the 0x7f
    p = vim_strchr(p_7f, Ctrl_A);
    if (p == NULL)
	p = p_7f + 1;
    else
	++p;

    if (!VIM_ISDIGIT(*p))	    // check for start of line number
	goto etag_fail;
    tagp->command = p;

    if (p[-1] == Ctrl_A)	    // first format: explicit tagname given
    {
	tagp->tagname = p_7f + 1;
	tagp->tagname_end = p - 1;
    }
    else			    // second format: isolate tagname
    {
	// find end of tagname
	for (p = p_7f - 1; !vim_iswordc(*p); --p)
	    if (p == lbuf)
		goto etag_fail;
	tagp->tagname_end = p + 1;
	while (p >= lbuf && vim_iswordc(*p))
	    --p;
	tagp->tagname = p + 1;
    }

    return OK;
}
#endif

/*
 * Read the next line from a tags file.
 * Returns TAGS_READ_SUCCESS if a tags line is successfully read and should be
 * processed.
 * Returns TAGS_READ_EOF if the end of file is reached.
 * Returns TAGS_READ_IGNORE if the current line should be ignored (used when
 * reached end of a emacs included tags file)
 */
    static tags_read_status_T
findtags_get_next_line(findtags_state_T *st, tagsearch_info_T *sinfo_p)
{
    int		eof;
    off_T	offset;

    // For binary search: compute the next offset to use.
    if (st->state == TS_BINARY)
    {
	offset = sinfo_p->low_offset + ((sinfo_p->high_offset
						- sinfo_p->low_offset) / 2);
	if (offset == sinfo_p->curr_offset)
	    return TAGS_READ_EOF; // End the binary search without a match.
	else
	    sinfo_p->curr_offset = offset;
    }

    // Skipping back (after a match during binary search).
    else if (st->state == TS_SKIP_BACK)
    {
	sinfo_p->curr_offset -= st->lbuf_size * 2;
	if (sinfo_p->curr_offset < 0)
	{
	    sinfo_p->curr_offset = 0;
	    rewind(st->fp);
	    st->state = TS_STEP_FORWARD;
	}
    }

    // When jumping around in the file, first read a line to find the
    // start of the next line.
    if (st->state == TS_BINARY || st->state == TS_SKIP_BACK)
    {
	// Adjust the search file offset to the correct position
	sinfo_p->curr_offset_used = sinfo_p->curr_offset;
	vim_ignored = vim_fseek(st->fp, sinfo_p->curr_offset, SEEK_SET);
	eof = vim_fgets(st->lbuf, st->lbuf_size, st->fp);
	if (!eof && sinfo_p->curr_offset != 0)
	{
	    sinfo_p->curr_offset = vim_ftell(st->fp);
	    if (sinfo_p->curr_offset == sinfo_p->high_offset)
	    {
		// oops, gone a bit too far; try from low offset
		vim_ignored = vim_fseek(st->fp, sinfo_p->low_offset, SEEK_SET);
		sinfo_p->curr_offset = sinfo_p->low_offset;
	    }
	    eof = vim_fgets(st->lbuf, st->lbuf_size, st->fp);
	}
	// skip empty and blank lines
	while (!eof && vim_isblankline(st->lbuf))
	{
	    sinfo_p->curr_offset = vim_ftell(st->fp);
	    eof = vim_fgets(st->lbuf, st->lbuf_size, st->fp);
	}
	if (eof)
	{
	    // Hit end of file.  Skip backwards.
	    st->state = TS_SKIP_BACK;
	    sinfo_p->match_offset = vim_ftell(st->fp);
	    sinfo_p->curr_offset = sinfo_p->curr_offset_used;
	    return TAGS_READ_IGNORE;
	}
    }
    // Not jumping around in the file: Read the next line.
    else
    {
	// skip empty and blank lines
	do
	{
#ifdef FEAT_CSCOPE
	    if (st->flags & TAG_CSCOPE)
		eof = cs_fgets(st->lbuf, st->lbuf_size);
	    else
#endif
		eof = vim_fgets(st->lbuf, st->lbuf_size, st->fp);
	} while (!eof && vim_isblankline(st->lbuf));

	if (eof)
	{
#ifdef FEAT_EMACS_TAGS
	    if (emacs_tags_file_eof(st) == TRUE)
		// an included tags file. Continue processing the parent
		// tags file.
		return TAGS_READ_IGNORE;
#endif
	    return TAGS_READ_EOF;
	}
    }

    return TAGS_READ_SUCCESS;
}

/*
 * Parse a tags file header line in "st->lbuf".
 * Returns TRUE if the current line in st->lbuf is not a tags header line and
 * should be parsed as a regular tag line. Returns FALSE if the line is a
 * header line and the next header line should be read.
 */
    static int
findtags_hdr_parse(findtags_state_T *st)
{
    char_u	*p;

    // Header lines in a tags file start with "!_TAG_"
    if (STRNCMP(st->lbuf, "!_TAG_", 6) != 0)
	// Non-header item before the header, e.g. "!" itself.
	return TRUE;

    // Process the header line.
    if (STRNCMP(st->lbuf, "!_TAG_FILE_SORTED\t", 18) == 0)
	st->tag_file_sorted = st->lbuf[18];
    if (STRNCMP(st->lbuf, "!_TAG_FILE_ENCODING\t", 20) == 0)
    {
	// Prepare to convert every line from the specified encoding to
	// 'encoding'.
	for (p = st->lbuf + 20; *p > ' ' && *p < 127; ++p)
	    ;
	*p = NUL;
	convert_setup(&st->vimconv, st->lbuf + 20, p_enc);
    }

    // Read the next line.  Unrecognized flags are ignored.
    return FALSE;
}

/*
 * Handler to initialize the state when starting to process a new tags file.
 * Called in the TS_START state when finding tags from a tags file.
 * Returns TRUE if the line read from the tags file should be parsed and
 * FALSE if the line should be ignored.
 */
    static int
findtags_start_state_handler(
    findtags_state_T	*st,
    int			*sortic,
    tagsearch_info_T	*sinfo_p)
{
#ifdef FEAT_CSCOPE
    int		use_cscope = (st->flags & TAG_CSCOPE);
#endif
    int		noic = (st->flags & TAG_NOIC);
    off_T	filesize;

    // The header ends when the line sorts below "!_TAG_".  When case is
    // folded lower case letters sort before "_".
    if (STRNCMP(st->lbuf, "!_TAG_", 6) <= 0
	    || (st->lbuf[0] == '!' && ASCII_ISLOWER(st->lbuf[1])))
	return findtags_hdr_parse(st);

    // Headers ends.

    // When there is no tag head, or ignoring case, need to do a
    // linear search.
    // When no "!_TAG_" is found, default to binary search.  If
    // the tag file isn't sorted, the second loop will find it.
    // When "!_TAG_FILE_SORTED" found: start binary search if
    // flag set.
    // For cscope, it's always linear.
# ifdef FEAT_CSCOPE
    if (st->linear || use_cscope)
# else
    if (st->linear)
# endif
	st->state = TS_LINEAR;
    else if (st->tag_file_sorted == NUL)
	st->state = TS_BINARY;
    else if (st->tag_file_sorted == '1')
	st->state = TS_BINARY;
    else if (st->tag_file_sorted == '2')
    {
	st->state = TS_BINARY;
	*sortic = TRUE;
	st->orgpat->regmatch.rm_ic = (p_ic || !noic);
    }
    else
	st->state = TS_LINEAR;

    if (st->state == TS_BINARY && st->orgpat->regmatch.rm_ic && !*sortic)
    {
	// Binary search won't work for ignoring case, use linear
	// search.
	st->linear = TRUE;
	st->state = TS_LINEAR;
    }

    // When starting a binary search, get the size of the file and
    // compute the first offset.
    if (st->state == TS_BINARY)
    {
	if (vim_fseek(st->fp, 0L, SEEK_END) != 0)
	    // can't seek, don't use binary search
	    st->state = TS_LINEAR;
	else
	{
	    // Get the tag file size (don't use mch_fstat(), it's
	    // not portable).  Don't use lseek(), it doesn't work
	    // properly on MacOS Catalina.
	    filesize = vim_ftell(st->fp);
	    vim_ignored = vim_fseek(st->fp, 0L, SEEK_SET);

	    // Calculate the first read offset in the file.  Start
	    // the search in the middle of the file.
	    sinfo_p->low_offset = 0;
	    sinfo_p->low_char = 0;
	    sinfo_p->high_offset = filesize;
	    sinfo_p->curr_offset = 0;
	    sinfo_p->high_char = 0xff;
	}
	return FALSE;
    }

    return TRUE;
}

/*
 * Parse a tag line read from a tags file.
 * Also compares the tag name in "tagpp->tagname" with a search pattern in
 * "st->orgpat->head" as a quick check if the tag may match.
 * Returns:
 * - TAG_MATCH_SUCCESS if the tag may match
 * - TAG_MATCH_FAIL if the tag doesn't match
 * - TAG_MATCH_NEXT to look for the next matching tag (used in a binary search)
 * - TAG_MATCH_STOP if all the tags are processed without a match. Uses the
 *   values in "margs" for doing the comparison.
 */
    static tagmatch_status_T
findtags_parse_line(
    findtags_state_T		*st,
    tagptrs_T			*tagpp,
    findtags_match_args_T	*margs,
    tagsearch_info_T		*sinfo_p)
{
    int		status;
    int		i;
    int		cmplen;
    int		tagcmp;

    // Figure out where the different strings are in this line.
    // For "normal" tags: Do a quick check if the tag matches.
    // This speeds up tag searching a lot!
    if (st->orgpat->headlen
#ifdef FEAT_EMACS_TAGS
	    && !st->is_etag
#endif
       )
    {
	CLEAR_FIELD(*tagpp);
	tagpp->tagname = st->lbuf;
	tagpp->tagname_end = vim_strchr(st->lbuf, TAB);
	if (tagpp->tagname_end == NULL)
	    // Corrupted tag line.
	    return TAG_MATCH_FAIL;

	// Skip this line if the length of the tag is different and
	// there is no regexp, or the tag is too short.
	cmplen = (int)(tagpp->tagname_end - tagpp->tagname);
	if (p_tl != 0 && cmplen > p_tl)	    // adjust for 'taglength'
	    cmplen = p_tl;
	if ((st->flags & TAG_REGEXP) && st->orgpat->headlen < cmplen)
	    cmplen = st->orgpat->headlen;
	else if (st->state == TS_LINEAR && st->orgpat->headlen != cmplen)
	    return TAG_MATCH_NEXT;

	if (st->state == TS_BINARY)
	{
	    // Simplistic check for unsorted tags file.
	    i = (int)tagpp->tagname[0];
	    if (margs->sortic)
		i = (int)TOUPPER_ASC(tagpp->tagname[0]);
	    if (i < sinfo_p->low_char || i > sinfo_p->high_char)
		margs->sort_error = TRUE;

	    // Compare the current tag with the searched tag.
	    if (margs->sortic)
		tagcmp = tag_strnicmp(tagpp->tagname, st->orgpat->head,
							(size_t)cmplen);
	    else
		tagcmp = STRNCMP(tagpp->tagname, st->orgpat->head, cmplen);

	    // A match with a shorter tag means to search forward.
	    // A match with a longer tag means to search backward.
	    if (tagcmp == 0)
	    {
		if (cmplen < st->orgpat->headlen)
		    tagcmp = -1;
		else if (cmplen > st->orgpat->headlen)
		    tagcmp = 1;
	    }

	    if (tagcmp == 0)
	    {
		// We've located the tag, now skip back and search
		// forward until the first matching tag is found.
		st->state = TS_SKIP_BACK;
		sinfo_p->match_offset = sinfo_p->curr_offset;
		return TAG_MATCH_NEXT;
	    }
	    if (tagcmp < 0)
	    {
		sinfo_p->curr_offset = vim_ftell(st->fp);
		if (sinfo_p->curr_offset < sinfo_p->high_offset)
		{
		    sinfo_p->low_offset = sinfo_p->curr_offset;
		    if (margs->sortic)
			sinfo_p->low_char = TOUPPER_ASC(tagpp->tagname[0]);
		    else
			sinfo_p->low_char = tagpp->tagname[0];
		    return TAG_MATCH_NEXT;
		}
	    }
	    if (tagcmp > 0 && sinfo_p->curr_offset != sinfo_p->high_offset)
	    {
		sinfo_p->high_offset = sinfo_p->curr_offset;
		if (margs->sortic)
		    sinfo_p->high_char = TOUPPER_ASC(tagpp->tagname[0]);
		else
		    sinfo_p->high_char = tagpp->tagname[0];
		return TAG_MATCH_NEXT;
	    }

	    // No match yet and are at the end of the binary search.
	    return TAG_MATCH_STOP;
	}
	else if (st->state == TS_SKIP_BACK)
	{
	    if (MB_STRNICMP(tagpp->tagname, st->orgpat->head, cmplen) != 0)
		st->state = TS_STEP_FORWARD;
	    else
		// Have to skip back more.  Restore the curr_offset
		// used, otherwise we get stuck at a long line.
		sinfo_p->curr_offset = sinfo_p->curr_offset_used;
	    return TAG_MATCH_NEXT;
	}
	else if (st->state == TS_STEP_FORWARD)
	{
	    if (MB_STRNICMP(tagpp->tagname, st->orgpat->head, cmplen) != 0)
	    {
		if ((off_T)vim_ftell(st->fp) > sinfo_p->match_offset)
		    return TAG_MATCH_STOP;	// past last match
		else
		    return TAG_MATCH_NEXT;	// before first match
	    }
	}
	else
	    // skip this match if it can't match
	    if (MB_STRNICMP(tagpp->tagname, st->orgpat->head, cmplen) != 0)
		return TAG_MATCH_NEXT;

	// Can be a matching tag, isolate the file name and command.
	tagpp->fname = tagpp->tagname_end + 1;
	tagpp->fname_end = vim_strchr(tagpp->fname, TAB);
	if (tagpp->fname_end == NULL)
	    status = FAIL;
	else
	{
	    tagpp->command = tagpp->fname_end + 1;
	    status = OK;
	}
    }
    else
	status = parse_tag_line(st->lbuf,
#ifdef FEAT_EMACS_TAGS
		st->is_etag,
#endif
		tagpp);

    if (status == FAIL)
	return TAG_MATCH_FAIL;

#ifdef FEAT_EMACS_TAGS
    if (st->is_etag)
	tagpp->fname = st->ebuf;
#endif

    return TAG_MATCH_SUCCESS;
}

/*
 * Initialize the structure used for tag matching.
 */
    static void
findtags_matchargs_init(findtags_match_args_T *margs, int flags)
{
    margs->matchoff = 0;			// match offset
    margs->match_re = FALSE;			// match with regexp
    margs->match_no_ic = FALSE;			// matches with case
    margs->has_re = (flags & TAG_REGEXP);	// regexp used
    margs->sortic = FALSE;			// tag file sorted in nocase
    margs->sort_error = FALSE;			// tags file not sorted
}

/*
 * Compares the tag name in "tagpp->tagname" with a search pattern in
 * "st->orgpat->pat".
 * Returns TRUE if the tag matches, FALSE if the tag doesn't match.
 * Uses the values in "margs" for doing the comparison.
 */
    static int
findtags_match_tag(
    findtags_state_T	*st,
    tagptrs_T		*tagpp,
    findtags_match_args_T *margs)
{
    int		match = FALSE;
    int		cmplen;

    // First try matching with the pattern literally (also when it is
    // a regexp).
    cmplen = (int)(tagpp->tagname_end - tagpp->tagname);
    if (p_tl != 0 && cmplen > p_tl)	    // adjust for 'taglength'
	cmplen = p_tl;
    // if tag length does not match, don't try comparing
    if (st->orgpat->len != cmplen)
	match = FALSE;
    else
    {
	if (st->orgpat->regmatch.rm_ic)
	{
	    match =
		(MB_STRNICMP(tagpp->tagname, st->orgpat->pat, cmplen) == 0);
	    if (match)
		margs->match_no_ic =
		    (STRNCMP(tagpp->tagname, st->orgpat->pat, cmplen) == 0);
	}
	else
	    match = (STRNCMP(tagpp->tagname, st->orgpat->pat, cmplen) == 0);
    }

    // Has a regexp: Also find tags matching regexp.
    margs->match_re = FALSE;
    if (!match && st->orgpat->regmatch.regprog != NULL)
    {
	int	cc;

	cc = *tagpp->tagname_end;
	*tagpp->tagname_end = NUL;
	match = vim_regexec(&st->orgpat->regmatch, tagpp->tagname, (colnr_T)0);
	if (match)
	{
	    margs->matchoff = (int)(st->orgpat->regmatch.startp[0] -
							tagpp->tagname);
	    if (st->orgpat->regmatch.rm_ic)
	    {
		st->orgpat->regmatch.rm_ic = FALSE;
		margs->match_no_ic = vim_regexec(&st->orgpat->regmatch,
			tagpp->tagname, (colnr_T)0);
		st->orgpat->regmatch.rm_ic = TRUE;
	    }
	}
	*tagpp->tagname_end = cc;
	margs->match_re = TRUE;
    }

    return match;
}

/*
 * Convert the encoding of a line read from a tags file in "st->lbuf".
 * Converting the pattern from 'enc' to the tags file encoding doesn't work,
 * because characters are not recognized. The converted line is saved in
 * st->lbuf.
 */
    static void
findtags_string_convert(findtags_state_T *st)
{
    char_u	*conv_line;
    int		len;

    conv_line = string_convert(&st->vimconv, st->lbuf, NULL);
    if (conv_line == NULL)
	return;

    // Copy or swap lbuf and conv_line.
    len = (int)STRLEN(conv_line) + 1;
    if (len > st->lbuf_size)
    {
	vim_free(st->lbuf);
	st->lbuf = conv_line;
	st->lbuf_size = len;
    }
    else
    {
	STRCPY(st->lbuf, conv_line);
	vim_free(conv_line);
    }
}

/*
 * Add a matching tag found in a tags file to st->ht_match and st->ga_match.
 * Returns OK if successfully added the match and FAIL on memory allocation
 * failure.
 */
    static int
findtags_add_match(
    findtags_state_T	*st,
    tagptrs_T		*tagpp,
    findtags_match_args_T   *margs,
    char_u		*buf_ffname,
    hash_T		*hash)
{
#ifdef FEAT_CSCOPE
    int		use_cscope = (st->flags & TAG_CSCOPE);
#endif
    int		name_only = (st->flags & TAG_NAMES);
    int		mtt;
    int		len = 0;
    int		is_current;		// file name matches
    int		is_static;		// current tag line is static
    char_u	*mfp;
    char_u	*p;
    char_u	*s;

#ifdef FEAT_CSCOPE
    if (use_cscope)
    {
	// Don't change the ordering, always use the same table.
	mtt = MT_GL_OTH;
    }
    else
#endif
    {
	// Decide in which array to store this match.
	is_current = test_for_current(
#ifdef FEAT_EMACS_TAGS
		st->is_etag,
#endif
		tagpp->fname, tagpp->fname_end, st->tag_fname, buf_ffname);
#ifdef FEAT_EMACS_TAGS
	is_static = FALSE;
	if (!st->is_etag)	// emacs tags are never static
#endif
	    is_static = test_for_static(tagpp);

	// decide in which of the sixteen tables to store this
	// match
	if (is_static)
	{
	    if (is_current)
		mtt = MT_ST_CUR;
	    else
		mtt = MT_ST_OTH;
	}
	else
	{
	    if (is_current)
		mtt = MT_GL_CUR;
	    else
		mtt = MT_GL_OTH;
	}
	if (st->orgpat->regmatch.rm_ic && !margs->match_no_ic)
	    mtt += MT_IC_OFF;
	if (margs->match_re)
	    mtt += MT_RE_OFF;
    }

    // Add the found match in ht_match[mtt] and ga_match[mtt].
    // Store the info we need later, which depends on the kind of
    // tags we are dealing with.
    if (st->help_only)
    {
#ifdef FEAT_MULTI_LANG
# define ML_EXTRA 3
#else
# define ML_EXTRA 0
#endif
	// Append the help-heuristic number after the tagname, for
	// sorting it later.  The heuristic is ignored for
	// detecting duplicates.
	// The format is {tagname}@{lang}NUL{heuristic}NUL
	*tagpp->tagname_end = NUL;
	len = (int)(tagpp->tagname_end - tagpp->tagname);
	mfp = alloc(sizeof(char_u) + len + 10 + ML_EXTRA + 1);
	if (mfp != NULL)
	{
	    int heuristic;

	    p = mfp;
	    STRCPY(p, tagpp->tagname);
#ifdef FEAT_MULTI_LANG
	    p[len] = '@';
	    STRCPY(p + len + 1, st->help_lang);
#endif

	    heuristic = help_heuristic(tagpp->tagname,
				margs->match_re ? margs->matchoff : 0,
				!margs->match_no_ic);
#ifdef FEAT_MULTI_LANG
	    heuristic += st->help_pri;
#endif
	    sprintf((char *)p + len + 1 + ML_EXTRA, "%06d",
		    heuristic);
	}
	*tagpp->tagname_end = TAB;
    }
    else if (name_only)
    {
	if (st->get_searchpat)
	{
	    char_u *temp_end = tagpp->command;

	    if (*temp_end == '/')
		while (*temp_end && *temp_end != '\r'
			&& *temp_end != '\n'
			&& *temp_end != '$')
		    temp_end++;

	    if (tagpp->command + 2 < temp_end)
	    {
		len = (int)(temp_end - tagpp->command - 2);
		mfp = alloc(len + 2);
		if (mfp != NULL)
		    vim_strncpy(mfp, tagpp->command + 2, len);
	    }
	    else
		mfp = NULL;
	    st->get_searchpat = FALSE;
	}
	else
	{
	    len = (int)(tagpp->tagname_end - tagpp->tagname);
	    mfp = alloc(sizeof(char_u) + len + 1);
	    if (mfp != NULL)
		vim_strncpy(mfp, tagpp->tagname, len);

	    // if wanted, re-read line to get long form too
	    if (State & MODE_INSERT)
		st->get_searchpat = p_sft;
	}
    }
    else
    {
	size_t tag_fname_len = STRLEN(st->tag_fname);
#ifdef FEAT_EMACS_TAGS
	size_t ebuf_len = 0;
#endif

	// Save the tag in a buffer.
	// Use 0x02 to separate fields (Can't use NUL because the
	// hash key is terminated by NUL, or Ctrl_A because that is
	// part of some Emacs tag files -- see parse_tag_line).
	// Emacs tag: <mtt><tag_fname><0x02><ebuf><0x02><lbuf><NUL>
	// other tag: <mtt><tag_fname><0x02><0x02><lbuf><NUL>
	// without Emacs tags: <mtt><tag_fname><0x02><lbuf><NUL>
	// Here <mtt> is the "mtt" value plus 1 to avoid NUL.
	len = (int)tag_fname_len + (int)STRLEN(st->lbuf) + 3;
#ifdef FEAT_EMACS_TAGS
	if (st->is_etag)
	{
	    ebuf_len = STRLEN(st->ebuf);
	    len += (int)ebuf_len + 1;
	}
	else
	    ++len;
#endif
	mfp = alloc(sizeof(char_u) + len + 1);
	if (mfp != NULL)
	{
	    p = mfp;
	    p[0] = mtt + 1;
	    STRCPY(p + 1, st->tag_fname);
#ifdef BACKSLASH_IN_FILENAME
	    // Ignore differences in slashes, avoid adding
	    // both path/file and path\file.
	    slash_adjust(p + 1);
#endif
	    p[tag_fname_len + 1] = TAG_SEP;
	    s = p + 1 + tag_fname_len + 1;
#ifdef FEAT_EMACS_TAGS
	    if (st->is_etag)
	    {
		STRCPY(s, st->ebuf);
		s[ebuf_len] = TAG_SEP;
		s += ebuf_len + 1;
	    }
	    else
		*s++ = TAG_SEP;
#endif
	    STRCPY(s, st->lbuf);
	}
    }

    if (mfp != NULL)
    {
	hashitem_T	*hi;

	// Don't add identical matches.
	// Add all cscope tags, because they are all listed.
	// "mfp" is used as a hash key, there is a NUL byte to end
	// the part that matters for comparing, more bytes may
	// follow after it.  E.g. help tags store the priority
	// after the NUL.
#ifdef FEAT_CSCOPE
	if (use_cscope)
	    ++*hash;
	else
#endif
	    *hash = hash_hash(mfp);
	hi = hash_lookup(&st->ht_match[mtt], mfp, *hash);
	if (HASHITEM_EMPTY(hi))
	{
	    if (hash_add_item(&st->ht_match[mtt], hi, mfp, *hash) == FAIL
		    || ga_grow(&st->ga_match[mtt], 1) == FAIL)
	    {
		// Out of memory! Just forget about the rest.
		st->stop_searching = TRUE;
		return FAIL;
	    }

	    ((char_u **)(st->ga_match[mtt].ga_data))
		[st->ga_match[mtt].ga_len++] = mfp;
	    st->match_count++;
	}
	else
	    // duplicate tag, drop it
	    vim_free(mfp);
    }

    return OK;
}

/*
 * Read and get all the tags from file st->tag_fname.
 * Sets "st->stop_searching" to TRUE to stop searching for additional tags.
 */
    static void
findtags_get_all_tags(
    findtags_state_T		*st,
    findtags_match_args_T	*margs,
    char_u			*buf_ffname)
{
    tagptrs_T		tagp;
    tagsearch_info_T	search_info;
    int			retval;
#ifdef FEAT_CSCOPE
    int			use_cscope = (st->flags & TAG_CSCOPE);
#endif
    hash_T		hash = 0;

    // This is only to avoid a compiler warning for using search_info
    // uninitialised.
    CLEAR_FIELD(search_info);

    // Read and parse the lines in the file one by one
    for (;;)
    {
	// check for CTRL-C typed, more often when jumping around
	if (st->state == TS_BINARY || st->state == TS_SKIP_BACK)
	    line_breakcheck();
	else
	    fast_breakcheck();
	if ((st->flags & TAG_INS_COMP))	// Double brackets for gcc
	    ins_compl_check_keys(30, FALSE);
	if (got_int || ins_compl_interrupted())
	{
	    st->stop_searching = TRUE;
	    break;
	}
	// When mincount is TAG_MANY, stop when enough matches have been
	// found (for completion).
	if (st->mincount == TAG_MANY && st->match_count >= TAG_MANY)
	{
	    st->stop_searching = TRUE;
	    break;
	}
	if (st->get_searchpat)
	    goto line_read_in;

	retval = findtags_get_next_line(st, &search_info);
	if (retval == TAGS_READ_IGNORE)
	    continue;
	if (retval == TAGS_READ_EOF)
	    break;

line_read_in:

	if (st->vimconv.vc_type != CONV_NONE)
	    findtags_string_convert(st);

#ifdef FEAT_EMACS_TAGS
	// Emacs tags line with CTRL-L: New file name on next line.
	// The file name is followed by a ','.
	// Remember etag file name in ebuf.
	if (*st->lbuf == Ctrl_L
# ifdef FEAT_CSCOPE
		&& !use_cscope
# endif
	   )
	{
	    st->is_etag = TRUE;		// in case at the start
	    st->state = TS_LINEAR;
	    emacs_tags_new_filename(st);
	    continue;
	}
#endif

	// When still at the start of the file, check for Emacs tags file
	// format, and for "not sorted" flag.
	if (st->state == TS_START)
	{
	    if (findtags_start_state_handler(st, &margs->sortic, &search_info) == FALSE)
		continue;
	}

	// When the line is too long the NUL will not be in the
	// last-but-one byte (see vim_fgets()).
	// Has been reported for Mozilla JS with extremely long names.
	// In that case we need to increase lbuf_size.
	if (st->lbuf[st->lbuf_size - 2] != NUL
#ifdef FEAT_CSCOPE
		&& !use_cscope
#endif
	   )
	{
	    st->lbuf_size *= 2;
	    vim_free(st->lbuf);
	    st->lbuf = alloc(st->lbuf_size);
	    if (st->lbuf == NULL)
	    {
		if (st->fp != NULL)
		    fclose(st->fp);
		st->fp = NULL;
		st->stop_searching = TRUE;
		return;
	    }

	    if (st->state == TS_STEP_FORWARD || st->state == TS_LINEAR)
		// Seek to the same position to read the same line again
		vim_ignored = vim_fseek(st->fp, search_info.curr_offset,
								     SEEK_SET);
	    // this will try the same thing again, make sure the offset is
	    // different
	    search_info.curr_offset = 0;
	    continue;
	}

	retval = findtags_parse_line(st, &tagp, margs, &search_info);
	if (retval == TAG_MATCH_NEXT)
	    continue;
	if (retval == TAG_MATCH_STOP)
	    break;
	if (retval == TAG_MATCH_FAIL)
	{
	    semsg(_(e_format_error_in_tags_file_str), st->tag_fname);
#ifdef FEAT_CSCOPE
	    if (!use_cscope)
#endif
		semsg(_("Before byte %ld"), (long)vim_ftell(st->fp));
	    st->stop_searching = TRUE;
	    return;
	}

	// If a match is found, add it to ht_match[] and ga_match[].
	if (findtags_match_tag(st, &tagp, margs))
	{
	    if (findtags_add_match(st, &tagp, margs, buf_ffname, &hash)
								== FAIL)
		break;
	}
    } // forever
}

/*
 * Search for tags matching "st->orgpat->pat" in the "st->tag_fname" tags file.
 * Information needed to search for the tags is in the "st" state structure.
 * The matching tags are returned in "st". If an error is encountered, then
 * "st->stop_searching" is set to TRUE.
 */
    static void
findtags_in_file(findtags_state_T *st, char_u *buf_ffname)
{
    findtags_match_args_T margs;
#ifdef FEAT_CSCOPE
    int		use_cscope = (st->flags & TAG_CSCOPE);
#endif

    st->vimconv.vc_type = CONV_NONE;
    st->tag_file_sorted = NUL;
    st->fp = NULL;
    findtags_matchargs_init(&margs, st->flags);

    // A file that doesn't exist is silently ignored.  Only when not a
    // single file is found, an error message is given (further on).
#ifdef FEAT_CSCOPE
    if (use_cscope)
	st->fp = NULL;	    // avoid GCC warning
    else
#endif
    {
#ifdef FEAT_MULTI_LANG
	if (curbuf->b_help)
	{
	    if (!findtags_in_help_init(st))
		return;
	}
#endif

	st->fp = mch_fopen((char *)st->tag_fname, "r");
	if (st->fp == NULL)
	    return;

	if (p_verbose >= 5)
	{
	    verbose_enter();
	    smsg(_("Searching tags file %s"), st->tag_fname);
	    verbose_leave();
	}
    }
    st->did_open = TRUE;	// remember that we found at least one file

    st->state = TS_START;	// we're at the start of the file
#ifdef FEAT_EMACS_TAGS
    st->is_etag = FALSE;	// default is: not emacs style
#endif

    // Read and parse the lines in the file one by one
    findtags_get_all_tags(st, &margs, buf_ffname);

    if (st->fp != NULL)
    {
	fclose(st->fp);
	st->fp = NULL;
    }
#ifdef FEAT_EMACS_TAGS
    emacs_tags_incstack_free();
#endif
    if (st->vimconv.vc_type != CONV_NONE)
	convert_setup(&st->vimconv, NULL, NULL);

    if (margs.sort_error)
	semsg(_(e_tags_file_not_sorted_str), st->tag_fname);

    // Stop searching if sufficient tags have been found.
    if (st->match_count >= st->mincount)
	st->stop_searching = TRUE;
}

/*
 * Copy the tags found by find_tags() to "matchesp".
 * Returns the number of matches copied.
 */
    static int
findtags_copy_matches(findtags_state_T *st, char_u ***matchesp)
{
    int		name_only = (st->flags & TAG_NAMES);
    char_u	**matches;
    int		mtt;
    int		i;
    char_u	*mfp;
    char_u	*p;

    if (st->match_count > 0)
	matches = ALLOC_MULT(char_u *, st->match_count);
    else
	matches = NULL;
    st->match_count = 0;
    for (mtt = 0; mtt < MT_COUNT; ++mtt)
    {
	for (i = 0; i < st->ga_match[mtt].ga_len; ++i)
	{
	    mfp = ((char_u **)(st->ga_match[mtt].ga_data))[i];
	    if (matches == NULL)
		vim_free(mfp);
	    else
	    {
		if (!name_only)
		{
		    // Change mtt back to zero-based.
		    *mfp = *mfp - 1;

		    // change the TAG_SEP back to NUL
		    for (p = mfp + 1; *p != NUL; ++p)
			if (*p == TAG_SEP)
			    *p = NUL;
		}
		matches[st->match_count++] = mfp;
	    }
	}

	ga_clear(&st->ga_match[mtt]);
	hash_clear(&st->ht_match[mtt]);
    }

    *matchesp = matches;
    return st->match_count;
}

/*
 * find_tags() - search for tags in tags files
 *
 * Return FAIL if search completely failed (*num_matches will be 0, *matchesp
 * will be NULL), OK otherwise.
 *
 * Priority depending on which type of tag is recognized:
 *  6.	A static or global tag with a full matching tag for the current file.
 *  5.	A global tag with a full matching tag for another file.
 *  4.	A static tag with a full matching tag for another file.
 *  3.	A static or global tag with an ignore-case matching tag for the
 *	current file.
 *  2.	A global tag with an ignore-case matching tag for another file.
 *  1.	A static tag with an ignore-case matching tag for another file.
 *
 * Tags in an emacs-style tags file are always global.
 *
 * flags:
 * TAG_HELP	  only search for help tags
 * TAG_NAMES	  only return name of tag
 * TAG_REGEXP	  use "pat" as a regexp
 * TAG_NOIC	  don't always ignore case
 * TAG_KEEP_LANG  keep language
 * TAG_CSCOPE	  use cscope results for tags
 * TAG_NO_TAGFUNC do not call the 'tagfunc' function
 */
    int
find_tags(
    char_u	*pat,			// pattern to search for
    int		*num_matches,		// return: number of matches found
    char_u	***matchesp,		// return: array of matches found
    int		flags,
    int		mincount,		// MAXCOL: find all matches
					// other: minimal number of matches
    char_u	*buf_ffname)		// name of buffer for priority
{
    findtags_state_T	st;
    tagname_T	tn;			// info for get_tagfname()
    int		first_file;		// trying first tag file
    int		retval = FAIL;		// return value
    int		round;

    int		save_emsg_off;

    int		help_save;
#ifdef FEAT_MULTI_LANG
    int		i;
    char_u	*saved_pat = NULL;		// copy of pat[]
#endif

    int		findall = (mincount == MAXCOL || mincount == TAG_MANY);
						// find all matching tags
    int		has_re = (flags & TAG_REGEXP);	// regexp used
    int		noic = (flags & TAG_NOIC);
#ifdef FEAT_CSCOPE
    int		use_cscope = (flags & TAG_CSCOPE);
#endif
    int		verbose = (flags & TAG_VERBOSE);
    int		save_p_ic = p_ic;

    /*
     * Change the value of 'ignorecase' according to 'tagcase' for the
     * duration of this function.
     */
    switch (curbuf->b_tc_flags ? curbuf->b_tc_flags : tc_flags)
    {
	case TC_FOLLOWIC:		 break;
	case TC_IGNORE:    p_ic = TRUE;  break;
	case TC_MATCH:     p_ic = FALSE; break;
	case TC_FOLLOWSCS: p_ic = ignorecase(pat); break;
	case TC_SMART:     p_ic = ignorecase_opt(pat, TRUE, TRUE); break;
    }

    help_save = curbuf->b_help;

    if (findtags_state_init(&st, pat, flags, mincount) == FAIL)
	goto findtag_end;

#ifdef FEAT_CSCOPE
    STRCPY(st.tag_fname, "from cscope");	// for error messages
#endif

    /*
     * Initialize a few variables
     */
    if (st.help_only)				// want tags from help file
	curbuf->b_help = TRUE;			// will be restored later
#ifdef FEAT_CSCOPE
    else if (use_cscope)
    {
	// Make sure we don't mix help and cscope, confuses Coverity.
	st.help_only = FALSE;
	curbuf->b_help = FALSE;
    }
#endif

#ifdef FEAT_MULTI_LANG
    if (curbuf->b_help)
    {
	// When "@ab" is specified use only the "ab" language, otherwise
	// search all languages.
	if (st.orgpat->len > 3 && pat[st.orgpat->len - 3] == '@'
				&& ASCII_ISALPHA(pat[st.orgpat->len - 2])
				&& ASCII_ISALPHA(pat[st.orgpat->len - 1]))
	{
	    saved_pat = vim_strnsave(pat, st.orgpat->len - 3);
	    if (saved_pat != NULL)
	    {
		st.help_lang_find = &pat[st.orgpat->len - 2];
		st.orgpat->pat = saved_pat;
		st.orgpat->len -= 3;
	    }
	}
    }
#endif
    if (p_tl != 0 && st.orgpat->len > p_tl)	// adjust for 'taglength'
	st.orgpat->len = p_tl;

    save_emsg_off = emsg_off;
    emsg_off = TRUE;  // don't want error for invalid RE here
    prepare_pats(st.orgpat, has_re);
    emsg_off = save_emsg_off;
    if (has_re && st.orgpat->regmatch.regprog == NULL)
	goto findtag_end;

#ifdef FEAT_EVAL
    retval = findtags_apply_tfu(&st, pat, buf_ffname);
    if (retval != NOTDONE)
	goto findtag_end;

    // re-initialize the default return value
    retval = FAIL;
#endif

#ifdef FEAT_MULTI_LANG
    // Set a flag if the file extension is .txt
    if ((flags & TAG_KEEP_LANG)
	    && st.help_lang_find == NULL
	    && curbuf->b_fname != NULL
	    && (i = (int)STRLEN(curbuf->b_fname)) > 4
	    && STRICMP(curbuf->b_fname + i - 4, ".txt") == 0)
	st.is_txt = TRUE;
#endif

    /*
     * When finding a specified number of matches, first try with matching
     * case, so binary search can be used, and try ignore-case matches in a
     * second loop.
     * When finding all matches, 'tagbsearch' is off, or there is no fixed
     * string to look for, ignore case right away to avoid going though the
     * tags files twice.
     * When the tag file is case-fold sorted, it is either one or the other.
     * Only ignore case when TAG_NOIC not used or 'ignorecase' set.
     */
    st.orgpat->regmatch.rm_ic = ((p_ic || !noic)
			&& (findall || st.orgpat->headlen == 0 || !p_tbs));
    for (round = 1; round <= 2; ++round)
    {
	st.linear = (st.orgpat->headlen == 0 || !p_tbs || round == 2);

      /*
       * Try tag file names from tags option one by one.
       */
      for (first_file = TRUE;
#ifdef FEAT_CSCOPE
	    use_cscope ||
#endif
		get_tagfname(&tn, first_file, st.tag_fname) == OK;
							   first_file = FALSE)
      {
	  findtags_in_file(&st, buf_ffname);
	  if (st.stop_searching
#ifdef FEAT_CSCOPE
		  || use_cscope
#endif
	     )
	  {
	      retval = OK;
	      break;
	  }
      } // end of for-each-file loop

#ifdef FEAT_CSCOPE
	if (!use_cscope)
#endif
	    tagname_free(&tn);

	// stop searching when already did a linear search, or when TAG_NOIC
	// used, and 'ignorecase' not set or already did case-ignore search
	if (st.stop_searching || st.linear || (!p_ic && noic) ||
						st.orgpat->regmatch.rm_ic)
	    break;
# ifdef FEAT_CSCOPE
	if (use_cscope)
	    break;
# endif

	// try another time while ignoring case
	st.orgpat->regmatch.rm_ic = TRUE;
    }

    if (!st.stop_searching)
    {
	if (!st.did_open && verbose)	// never opened any tags file
	    emsg(_(e_no_tags_file));
	retval = OK;		// It's OK even when no tag found
    }

findtag_end:
    findtags_state_free(&st);

    /*
     * Move the matches from the ga_match[] arrays into one list of
     * matches.  When retval == FAIL, free the matches.
     */
    if (retval == FAIL)
	st.match_count = 0;

    *num_matches = findtags_copy_matches(&st, matchesp);

    curbuf->b_help = help_save;
#ifdef FEAT_MULTI_LANG
    vim_free(saved_pat);
#endif

    p_ic = save_p_ic;

    return retval;
}

static garray_T tag_fnames = GA_EMPTY;

/*
 * Callback function for finding all "tags" and "tags-??" files in
 * 'runtimepath' doc directories.
 */
    static void
found_tagfile_cb(char_u *fname, void *cookie UNUSED)
{
    if (ga_grow(&tag_fnames, 1) == FAIL)
	return;

    char_u	*tag_fname = vim_strsave(fname);

#ifdef BACKSLASH_IN_FILENAME
    slash_adjust(tag_fname);
#endif
    simplify_filename(tag_fname);
    ((char_u **)(tag_fnames.ga_data))[tag_fnames.ga_len++] = tag_fname;
}

#if defined(EXITFREE) || defined(PROTO)
    void
free_tag_stuff(void)
{
    ga_clear_strings(&tag_fnames);
    if (curwin != NULL)
	do_tag(NULL, DT_FREE, 0, 0, 0);
    tag_freematch();

# if defined(FEAT_QUICKFIX)
    tagstack_clear_entry(&ptag_entry);
# endif
}
#endif

/*
 * Get the next name of a tag file from the tag file list.
 * For help files, use "tags" file only.
 *
 * Return FAIL if no more tag file names, OK otherwise.
 */
    int
get_tagfname(
    tagname_T	*tnp,	// holds status info
    int		first,	// TRUE when first file name is wanted
    char_u	*buf)	// pointer to buffer of MAXPATHL chars
{
    char_u		*fname = NULL;
    char_u		*r_ptr;
    int			i;

    if (first)
	CLEAR_POINTER(tnp);

    if (curbuf->b_help)
    {
	/*
	 * For help files it's done in a completely different way:
	 * Find "doc/tags" and "doc/tags-??" in all directories in
	 * 'runtimepath'.
	 */
	if (first)
	{
	    ga_clear_strings(&tag_fnames);
	    ga_init2(&tag_fnames, sizeof(char_u *), 10);
	    do_in_runtimepath((char_u *)
#ifdef FEAT_MULTI_LANG
# ifdef VMS
		    // Functions decc$to_vms() and decc$translate_vms() crash
		    // on some VMS systems with wildcards "??".  Seems ECO
		    // patches do fix the problem in C RTL, but we can't use
		    // an #ifdef for that.
		    "doc/tags doc/tags-*"
# else
		    "doc/tags doc/tags-??"
# endif
#else
		    "doc/tags"
#endif
					   , DIP_ALL, found_tagfile_cb, NULL);
	}

	if (tnp->tn_hf_idx >= tag_fnames.ga_len)
	{
	    // Not found in 'runtimepath', use 'helpfile', if it exists and
	    // wasn't used yet, replacing "help.txt" with "tags".
	    if (tnp->tn_hf_idx > tag_fnames.ga_len || *p_hf == NUL)
		return FAIL;
	    ++tnp->tn_hf_idx;
	    STRCPY(buf, p_hf);
	    STRCPY(gettail(buf), "tags");
#ifdef BACKSLASH_IN_FILENAME
	    slash_adjust(buf);
#endif
	    simplify_filename(buf);

	    for (i = 0; i < tag_fnames.ga_len; ++i)
		if (STRCMP(buf, ((char_u **)(tag_fnames.ga_data))[i]) == 0)
		    return FAIL; // avoid duplicate file names
	}
	else
	    vim_strncpy(buf, ((char_u **)(tag_fnames.ga_data))[
					     tnp->tn_hf_idx++], MAXPATHL - 1);
	return OK;
    }

    if (first)
    {
	// Init.  We make a copy of 'tags', because autocommands may change
	// the value without notifying us.
	tnp->tn_tags = vim_strsave((*curbuf->b_p_tags != NUL)
						 ? curbuf->b_p_tags : p_tags);
	if (tnp->tn_tags == NULL)
	    return FAIL;
	tnp->tn_np = tnp->tn_tags;
    }

    /*
     * Loop until we have found a file name that can be used.
     * There are two states:
     * tnp->tn_did_filefind_init == FALSE: setup for next part in 'tags'.
     * tnp->tn_did_filefind_init == TRUE: find next file in this part.
     */
    for (;;)
    {
	if (tnp->tn_did_filefind_init)
	{
	    fname = vim_findfile(tnp->tn_search_ctx);
	    if (fname != NULL)
		break;

	    tnp->tn_did_filefind_init = FALSE;
	}
	else
	{
	    char_u  *filename = NULL;

	    // Stop when used all parts of 'tags'.
	    if (*tnp->tn_np == NUL)
	    {
		vim_findfile_cleanup(tnp->tn_search_ctx);
		tnp->tn_search_ctx = NULL;
		return FAIL;
	    }

	    /*
	     * Copy next file name into buf.
	     */
	    buf[0] = NUL;
	    (void)copy_option_part(&tnp->tn_np, buf, MAXPATHL - 1, " ,");

	    r_ptr = vim_findfile_stopdir(buf);
	    // move the filename one char forward and truncate the
	    // filepath with a NUL
	    filename = gettail(buf);
	    STRMOVE(filename + 1, filename);
	    *filename++ = NUL;

	    tnp->tn_search_ctx = vim_findfile_init(buf, filename,
		    r_ptr, 100,
		    FALSE,	   // don't free visited list
		    FINDFILE_FILE, // we search for a file
		    tnp->tn_search_ctx, TRUE, curbuf->b_ffname);
	    if (tnp->tn_search_ctx != NULL)
		tnp->tn_did_filefind_init = TRUE;
	}
    }

    STRCPY(buf, fname);
    vim_free(fname);
    return OK;
}

/*
 * Free the contents of a tagname_T that was filled by get_tagfname().
 */
    void
tagname_free(tagname_T *tnp)
{
    vim_free(tnp->tn_tags);
    vim_findfile_cleanup(tnp->tn_search_ctx);
    tnp->tn_search_ctx = NULL;
    ga_clear_strings(&tag_fnames);
}

/*
 * Parse one line from the tags file. Find start/end of tag name, start/end of
 * file name and start of search pattern.
 *
 * If is_etag is TRUE, tagp->fname and tagp->fname_end are not set.
 *
 * Return FAIL if there is a format error in this line, OK otherwise.
 */
    static int
parse_tag_line(
    char_u	*lbuf,		// line to be parsed
#ifdef FEAT_EMACS_TAGS
    int		is_etag,
#endif
    tagptrs_T	*tagp)
{
    char_u	*p;

#ifdef FEAT_EMACS_TAGS
    if (is_etag)
	// emacs-style tag file
	return emacs_tags_parse_line(lbuf, tagp);
#endif

    // Isolate the tagname, from lbuf up to the first white
    tagp->tagname = lbuf;
    p = vim_strchr(lbuf, TAB);
    if (p == NULL)
	return FAIL;
    tagp->tagname_end = p;

    // Isolate file name, from first to second white space
    if (*p != NUL)
	++p;
    tagp->fname = p;
    p = vim_strchr(p, TAB);
    if (p == NULL)
	return FAIL;
    tagp->fname_end = p;

    // find start of search command, after second white space
    if (*p != NUL)
	++p;
    if (*p == NUL)
	return FAIL;
    tagp->command = p;

    return OK;
}

/*
 * Check if tagname is a static tag
 *
 * Static tags produced by the older ctags program have the format:
 *	'file:tag  file  /pattern'.
 * This is only recognized when both occurrence of 'file' are the same, to
 * avoid recognizing "string::string" or ":exit".
 *
 * Static tags produced by the new ctags program have the format:
 *	'tag  file  /pattern/;"<Tab>file:'	    "
 *
 * Return TRUE if it is a static tag and adjust *tagname to the real tag.
 * Return FALSE if it is not a static tag.
 */
    static int
test_for_static(tagptrs_T *tagp)
{
    char_u	*p;

    /*
     * Check for new style static tag ":...<Tab>file:[<Tab>...]"
     */
    p = tagp->command;
    while ((p = vim_strchr(p, '\t')) != NULL)
    {
	++p;
	if (STRNCMP(p, "file:", 5) == 0)
	    return TRUE;
    }

    return FALSE;
}

/*
 * Returns the length of a matching tag line.
 */
    static size_t
matching_line_len(char_u *lbuf)
{
    char_u	*p = lbuf + 1;

    // does the same thing as parse_match()
    p += STRLEN(p) + 1;
#ifdef FEAT_EMACS_TAGS
    p += STRLEN(p) + 1;
#endif
    return (p - lbuf) + STRLEN(p);
}

/*
 * Parse a line from a matching tag.  Does not change the line itself.
 *
 * The line that we get looks like this:
 * Emacs tag: <mtt><tag_fname><NUL><ebuf><NUL><lbuf>
 * other tag: <mtt><tag_fname><NUL><NUL><lbuf>
 * without Emacs tags: <mtt><tag_fname><NUL><lbuf>
 *
 * Return OK or FAIL.
 */
    static int
parse_match(
    char_u	*lbuf,	    // input: matching line
    tagptrs_T	*tagp)	    // output: pointers into the line
{
    int		retval;
    char_u	*p;
    char_u	*pc, *pt;

    tagp->tag_fname = lbuf + 1;
    lbuf += STRLEN(tagp->tag_fname) + 2;
#ifdef FEAT_EMACS_TAGS
    if (*lbuf)
    {
	tagp->is_etag = TRUE;
	tagp->fname = lbuf;
	lbuf += STRLEN(lbuf);
	tagp->fname_end = lbuf++;
    }
    else
    {
	tagp->is_etag = FALSE;
	++lbuf;
    }
#endif

    // Find search pattern and the file name for non-etags.
    retval = parse_tag_line(lbuf,
#ifdef FEAT_EMACS_TAGS
			tagp->is_etag,
#endif
			tagp);

    tagp->tagkind = NULL;
    tagp->user_data = NULL;
    tagp->tagline = 0;
    tagp->command_end = NULL;

    if (retval != OK)
	return retval;

    // Try to find a kind field: "kind:<kind>" or just "<kind>"
    p = tagp->command;
    if (find_extra(&p) == OK)
    {
	if (p > tagp->command && p[-1] == '|')
	    tagp->command_end = p - 1;  // drop trailing bar
	else
	    tagp->command_end = p;
	p += 2;	// skip ";\""
	if (*p++ == TAB)
	    // Accept ASCII alphabetic kind characters and any multi-byte
	    // character.
	    while (ASCII_ISALPHA(*p) || mb_ptr2len(p) > 1)
	    {
		if (STRNCMP(p, "kind:", 5) == 0)
		    tagp->tagkind = p + 5;
		else if (STRNCMP(p, "user_data:", 10) == 0)
		    tagp->user_data = p + 10;
		else if (STRNCMP(p, "line:", 5) == 0)
		    tagp->tagline = atoi((char *)p + 5);
		if (tagp->tagkind != NULL && tagp->user_data != NULL)
		    break;
		pc = vim_strchr(p, ':');
		pt = vim_strchr(p, '\t');
		if (pc == NULL || (pt != NULL && pc > pt))
		    tagp->tagkind = p;
		if (pt == NULL)
		    break;
		p = pt;
		MB_PTR_ADV(p);
	    }
    }
    if (tagp->tagkind != NULL)
    {
	for (p = tagp->tagkind;
		*p && *p != '\t' && *p != '\r' && *p != '\n'; MB_PTR_ADV(p))
	    ;
	tagp->tagkind_end = p;
    }
    if (tagp->user_data != NULL)
    {
	for (p = tagp->user_data;
		*p && *p != '\t' && *p != '\r' && *p != '\n'; MB_PTR_ADV(p))
	    ;
	tagp->user_data_end = p;
    }
    return retval;
}

/*
 * Find out the actual file name of a tag.  Concatenate the tags file name
 * with the matching tag file name.
 * Returns an allocated string or NULL (out of memory).
 */
    static char_u *
tag_full_fname(tagptrs_T *tagp)
{
    char_u	*fullname;
    int		c;

#ifdef FEAT_EMACS_TAGS
    if (tagp->is_etag)
	c = 0;	    // to shut up GCC
    else
#endif
    {
	c = *tagp->fname_end;
	*tagp->fname_end = NUL;
    }
    fullname = expand_tag_fname(tagp->fname, tagp->tag_fname, FALSE);

#ifdef FEAT_EMACS_TAGS
    if (!tagp->is_etag)
#endif
	*tagp->fname_end = c;

    return fullname;
}

/*
 * Jump to a tag that has been found in one of the tag files
 *
 * returns OK for success, NOTAGFILE when file not found, FAIL otherwise.
 */
    static int
jumpto_tag(
    char_u	*lbuf_arg,	// line from the tags file for this tag
    int		forceit,	// :ta with !
    int		keep_help)	// keep help flag (FALSE for cscope)
{
    optmagic_T	save_magic_overruled;
    int		save_p_ws, save_p_scs, save_p_ic;
    linenr_T	save_lnum;
    char_u	*str;
    char_u	*pbuf;			// search pattern buffer
    char_u	*pbuf_end;
    char_u	*tofree_fname = NULL;
    char_u	*fname;
    tagptrs_T	tagp;
    int		retval = FAIL;
    int		getfile_result = GETFILE_UNUSED;
    int		search_options;
#ifdef FEAT_SEARCH_EXTRA
    int		save_no_hlsearch;
#endif
#if defined(FEAT_QUICKFIX)
    win_T	*curwin_save = NULL;
#endif
    char_u	*full_fname = NULL;
#ifdef FEAT_FOLDING
    int		old_KeyTyped = KeyTyped;    // getting the file may reset it
#endif
    size_t	len;
    char_u	*lbuf;

    // Make a copy of the line, it can become invalid when an autocommand calls
    // back here recursively.
    len = matching_line_len(lbuf_arg) + 1;
    lbuf = alloc(len);
    if (lbuf != NULL)
	mch_memmove(lbuf, lbuf_arg, len);

    pbuf = alloc(LSIZE);

    // parse the match line into the tagp structure
    if (pbuf == NULL || lbuf == NULL || parse_match(lbuf, &tagp) == FAIL)
    {
	tagp.fname_end = NULL;
	goto erret;
    }

    // truncate the file name, so it can be used as a string
    *tagp.fname_end = NUL;
    fname = tagp.fname;

    // copy the command to pbuf[], remove trailing CR/NL
    str = tagp.command;
    for (pbuf_end = pbuf; *str && *str != '\n' && *str != '\r'; )
    {
#ifdef FEAT_EMACS_TAGS
	if (tagp.is_etag && *str == ',')// stop at ',' after line number
	    break;
#endif
	*pbuf_end++ = *str++;
	if (pbuf_end - pbuf + 1 >= LSIZE)
	    break;
    }
    *pbuf_end = NUL;

#ifdef FEAT_EMACS_TAGS
    if (!tagp.is_etag)
#endif
    {
	/*
	 * Remove the "<Tab>fieldname:value" stuff; we don't need it here.
	 */
	str = pbuf;
	if (find_extra(&str) == OK)
	{
	    pbuf_end = str;
	    *pbuf_end = NUL;
	}
    }

    /*
     * Expand file name, when needed (for environment variables).
     * If 'tagrelative' option set, may change file name.
     */
    fname = expand_tag_fname(fname, tagp.tag_fname, TRUE);
    if (fname == NULL)
	goto erret;
    tofree_fname = fname;	// free() it later

    /*
     * Check if the file with the tag exists before abandoning the current
     * file.  Also accept a file name for which there is a matching BufReadCmd
     * autocommand event (e.g., http://sys/file).
     */
    if (mch_getperm(fname) < 0 && !has_autocmd(EVENT_BUFREADCMD, fname, NULL))
    {
	retval = NOTAGFILE;
	vim_free(nofile_fname);
	nofile_fname = vim_strsave(fname);
	if (nofile_fname == NULL)
	    nofile_fname = empty_option;
	goto erret;
    }

    ++RedrawingDisabled;

#ifdef FEAT_GUI
    need_mouse_correct = TRUE;
#endif

#if defined(FEAT_QUICKFIX)
    if (g_do_tagpreview != 0)
    {
	postponed_split = 0;	// don't split again below
	curwin_save = curwin;	// Save current window

	/*
	 * If we are reusing a window, we may change dir when
	 * entering it (autocommands) so turn the tag filename
	 * into a fullpath
	 */
	if (!curwin->w_p_pvw)
	{
	    full_fname = FullName_save(fname, FALSE);
	    fname = full_fname;

	    /*
	     * Make the preview window the current window.
	     * Open a preview window when needed.
	     */
	    prepare_tagpreview(TRUE, TRUE, FALSE);
	}
    }

    // If it was a CTRL-W CTRL-] command split window now.  For ":tab tag"
    // open a new tab page.
    if (postponed_split && (swb_flags & (SWB_USEOPEN | SWB_USETAB)))
    {
	buf_T *existing_buf = buflist_findname_exp(fname);

	if (existing_buf != NULL)
	{
	    // If 'switchbuf' is set jump to the window containing "buf".
	    if (swbuf_goto_win_with_buf(existing_buf) != NULL)
		// We've switched to the buffer, the usual loading of the file
		// must be skipped.
		getfile_result = GETFILE_SAME_FILE;
	}
    }
    if (getfile_result == GETFILE_UNUSED
				  && (postponed_split || cmdmod.cmod_tab != 0))
    {
	if (win_split(postponed_split > 0 ? postponed_split : 0,
						postponed_split_flags) == FAIL)
	{
	    if (RedrawingDisabled > 0)
		--RedrawingDisabled;
	    goto erret;
	}
	RESET_BINDING(curwin);
    }
#endif

    if (keep_help)
    {
	// A :ta from a help file will keep the b_help flag set.  For ":ptag"
	// we need to use the flag from the window where we came from.
#if defined(FEAT_QUICKFIX)
	if (g_do_tagpreview != 0)
	    keep_help_flag = bt_help(curwin_save->w_buffer);
	else
#endif
	    keep_help_flag = curbuf->b_help;
    }

    if (getfile_result == GETFILE_UNUSED)
	// Careful: getfile() may trigger autocommands and call jumpto_tag()
	// recursively.
	getfile_result = getfile(0, fname, NULL, TRUE, (linenr_T)0, forceit);
    keep_help_flag = FALSE;

    if (GETFILE_SUCCESS(getfile_result))	// got to the right file
    {
	curwin->w_set_curswant = TRUE;
	postponed_split = 0;

	save_magic_overruled = magic_overruled;
	magic_overruled = OPTION_MAGIC_OFF;	// always execute with 'nomagic'
#ifdef FEAT_SEARCH_EXTRA
	// Save value of no_hlsearch, jumping to a tag is not a real search
	save_no_hlsearch = no_hlsearch;
#endif
#if defined(FEAT_PROP_POPUP) && defined(FEAT_QUICKFIX)
	// getfile() may have cleared options, apply 'previewpopup' again.
	if (g_do_tagpreview != 0 && *p_pvp != NUL)
	    parse_previewpopup(curwin);
#endif

	/*
	 * If 'cpoptions' contains 't', store the search pattern for the "n"
	 * command.  If 'cpoptions' does not contain 't', the search pattern
	 * is not stored.
	 */
	if (vim_strchr(p_cpo, CPO_TAGPAT) != NULL)
	    search_options = 0;
	else
	    search_options = SEARCH_KEEP;

	/*
	 * If the command is a search, try here.
	 *
	 * Reset 'smartcase' for the search, since the search pattern was not
	 * typed by the user.
	 * Only use do_search() when there is a full search command, without
	 * anything following.
	 */
	str = pbuf;
	if (pbuf[0] == '/' || pbuf[0] == '?')
	    str = skip_regexp(pbuf + 1, pbuf[0], FALSE) + 1;
	if (str > pbuf_end - 1)	// search command with nothing following
	{
	    save_p_ws = p_ws;
	    save_p_ic = p_ic;
	    save_p_scs = p_scs;
	    p_ws = TRUE;	// need 'wrapscan' for backward searches
	    p_ic = FALSE;	// don't ignore case now
	    p_scs = FALSE;
	    save_lnum = curwin->w_cursor.lnum;
	    if (tagp.tagline > 0)
		// start search before line from "line:" field
		curwin->w_cursor.lnum = tagp.tagline - 1;
	    else
		// start search before first line
		curwin->w_cursor.lnum = 0;
	    if (do_search(NULL, pbuf[0], pbuf[0], pbuf + 1, (long)1,
							 search_options, NULL))
		retval = OK;
	    else
	    {
		int	found = 1;
		int	cc;

		/*
		 * try again, ignore case now
		 */
		p_ic = TRUE;
		if (!do_search(NULL, pbuf[0], pbuf[0], pbuf + 1, (long)1,
							 search_options, NULL))
		{
		    /*
		     * Failed to find pattern, take a guess: "^func  ("
		     */
		    found = 2;
		    (void)test_for_static(&tagp);
		    cc = *tagp.tagname_end;
		    *tagp.tagname_end = NUL;
		    sprintf((char *)pbuf, "^%s\\s\\*(", tagp.tagname);
		    if (!do_search(NULL, '/', '/', pbuf, (long)1,
							 search_options, NULL))
		    {
			// Guess again: "^char * \<func  ("
			sprintf((char *)pbuf, "^\\[#a-zA-Z_]\\.\\*\\<%s\\s\\*(",
								tagp.tagname);
			if (!do_search(NULL, '/', '/', pbuf, (long)1,
							 search_options, NULL))
			    found = 0;
		    }
		    *tagp.tagname_end = cc;
		}
		if (found == 0)
		{
		    emsg(_(e_cannot_find_tag_pattern));
		    curwin->w_cursor.lnum = save_lnum;
		}
		else
		{
		    /*
		     * Only give a message when really guessed, not when 'ic'
		     * is set and match found while ignoring case.
		     */
		    if (found == 2 || !save_p_ic)
		    {
			msg(_(e_couldnt_find_tag_just_guessing));
			if (!msg_scrolled && msg_silent == 0)
			{
			    out_flush();
			    ui_delay(1010L, TRUE);
			}
		    }
		    retval = OK;
		}
	    }
	    p_ws = save_p_ws;
	    p_ic = save_p_ic;
	    p_scs = save_p_scs;

	    // A search command may have positioned the cursor beyond the end
	    // of the line.  May need to correct that here.
	    check_cursor();
	}
	else
	{
	    int		save_secure = secure;

	    // Setup the sandbox for executing the command from the tags file.
	    secure = 1;
#ifdef HAVE_SANDBOX
	    ++sandbox;
#endif
	    curwin->w_cursor.lnum = 1;		// start command in line 1
	    do_cmdline_cmd(pbuf);
	    retval = OK;

	    // When the command has done something that is not allowed make
	    // sure the error message can be seen.
	    if (secure == 2)
		wait_return(TRUE);
	    secure = save_secure;
#ifdef HAVE_SANDBOX
	    --sandbox;
#endif
	}

	magic_overruled = save_magic_overruled;
#ifdef FEAT_SEARCH_EXTRA
	// restore no_hlsearch when keeping the old search pattern
	if (search_options)
	    set_no_hlsearch(save_no_hlsearch);
#endif

	// Return OK if jumped to another file (at least we found the file!).
	if (getfile_result == GETFILE_OPEN_OTHER)
	    retval = OK;

	if (retval == OK)
	{
	    /*
	     * For a help buffer: Put the cursor line at the top of the window,
	     * the help subject will be below it.
	     */
	    if (curbuf->b_help)
		set_topline(curwin, curwin->w_cursor.lnum);
#ifdef FEAT_FOLDING
	    if ((fdo_flags & FDO_TAG) && old_KeyTyped)
		foldOpenCursor();
#endif
	}

#if defined(FEAT_QUICKFIX)
	if (g_do_tagpreview != 0
			   && curwin != curwin_save && win_valid(curwin_save))
	{
	    // Return cursor to where we were
	    validate_cursor();
	    redraw_later(UPD_VALID);
	    win_enter(curwin_save, TRUE);
	}
#endif

	if (RedrawingDisabled > 0)
	    --RedrawingDisabled;
    }
    else
    {
	if (RedrawingDisabled > 0)
	    --RedrawingDisabled;
	got_int = FALSE;  // don't want entering window to fail

	if (postponed_split)		// close the window
	{
	    win_close(curwin, FALSE);
	    postponed_split = 0;
	}
#if defined(FEAT_QUICKFIX) && defined(FEAT_PROP_POPUP)
	else if (WIN_IS_POPUP(curwin))
	{
	    win_T   *wp = curwin;

	    if (win_valid(curwin_save))
		win_enter(curwin_save, TRUE);
	    popup_close(wp->w_id, FALSE);
	}
#endif
    }
#if defined(FEAT_QUICKFIX) && defined(FEAT_PROP_POPUP)
    if (WIN_IS_POPUP(curwin))
	// something went wrong, still in popup, but it can't have focus
	win_enter(firstwin, TRUE);
#endif

erret:
#if defined(FEAT_QUICKFIX)
    g_do_tagpreview = 0; // For next time
#endif
    vim_free(lbuf);
    vim_free(pbuf);
    vim_free(tofree_fname);
    vim_free(full_fname);

    return retval;
}

/*
 * If "expand" is TRUE, expand wildcards in fname.
 * If 'tagrelative' option set, change fname (name of file containing tag)
 * according to tag_fname (name of tag file containing fname).
 * Returns a pointer to allocated memory (or NULL when out of memory).
 */
    static char_u *
expand_tag_fname(char_u *fname, char_u *tag_fname, int expand)
{
    char_u	*p;
    char_u	*retval;
    char_u	*expanded_fname = NULL;
    expand_T	xpc;

    /*
     * Expand file name (for environment variables) when needed.
     */
    if (expand && mch_has_wildcard(fname))
    {
	ExpandInit(&xpc);
	xpc.xp_context = EXPAND_FILES;
	expanded_fname = ExpandOne(&xpc, fname, NULL,
			    WILD_LIST_NOTFOUND|WILD_SILENT, WILD_EXPAND_FREE);
	if (expanded_fname != NULL)
	    fname = expanded_fname;
    }

    if ((p_tr || curbuf->b_help)
	    && !vim_isAbsName(fname)
	    && (p = gettail(tag_fname)) != tag_fname)
    {
	retval = alloc(MAXPATHL);
	if (retval != NULL)
	{
	    STRCPY(retval, tag_fname);
	    vim_strncpy(retval + (p - tag_fname), fname,
					      MAXPATHL - (p - tag_fname) - 1);
	    /*
	     * Translate names like "src/a/../b/file.c" into "src/b/file.c".
	     */
	    simplify_filename(retval);
	}
    }
    else
	retval = vim_strsave(fname);

    vim_free(expanded_fname);

    return retval;
}

/*
 * Check if we have a tag for the buffer with name "buf_ffname".
 * This is a bit slow, because of the full path compare in fullpathcmp().
 * Return TRUE if tag for file "fname" if tag file "tag_fname" is for current
 * file.
 */
    static int
test_for_current(
#ifdef FEAT_EMACS_TAGS
    int	    is_etag,
#endif
    char_u  *fname,
    char_u  *fname_end,
    char_u  *tag_fname,
    char_u  *buf_ffname)
{
    int	    c;
    int	    retval = FALSE;
    char_u  *fullname;

    if (buf_ffname != NULL)	// if the buffer has a name
    {
#ifdef FEAT_EMACS_TAGS
	if (is_etag)
	    c = 0;	    // to shut up GCC
	else
#endif
	{
	    c = *fname_end;
	    *fname_end = NUL;
	}
	fullname = expand_tag_fname(fname, tag_fname, TRUE);
	if (fullname != NULL)
	{
	    retval = (fullpathcmp(fullname, buf_ffname, TRUE, TRUE) & FPC_SAME);
	    vim_free(fullname);
	}
#ifdef FEAT_EMACS_TAGS
	if (!is_etag)
#endif
	    *fname_end = c;
    }

    return retval;
}

/*
 * Find the end of the tagaddress.
 * Return OK if ";\"" is following, FAIL otherwise.
 */
    static int
find_extra(char_u **pp)
{
    char_u	*str = *pp;
    char_u	first_char = **pp;

    // Repeat for addresses separated with ';'
    for (;;)
    {
	if (VIM_ISDIGIT(*str))
	    str = skipdigits(str + 1);
	else if (*str == '/' || *str == '?')
	{
	    str = skip_regexp(str + 1, *str, FALSE);
	    if (*str != first_char)
		str = NULL;
	    else
		++str;
	}
	else
	{
	    // not a line number or search string, look for terminator.
	    str = (char_u *)strstr((char *)str, "|;\"");
	    if (str != NULL)
	    {
		++str;
		break;
	    }

	}
	if (str == NULL || *str != ';'
		  || !(VIM_ISDIGIT(str[1]) || str[1] == '/' || str[1] == '?'))
	    break;
	++str;	// skip ';'
	first_char = *str;
    }

    if (str != NULL && STRNCMP(str, ";\"", 2) == 0)
    {
	*pp = str;
	return OK;
    }
    return FAIL;
}

/*
 * Free a single entry in a tag stack
 */
    static void
tagstack_clear_entry(taggy_T *item)
{
    VIM_CLEAR(item->tagname);
    VIM_CLEAR(item->user_data);
}

    int
expand_tags(
    int		tagnames,	// expand tag names
    char_u	*pat,
    int		*num_file,
    char_u	***file)
{
    int		i;
    int		extra_flag;
    char_u	*name_buf;
    size_t	name_buf_size = 100;
    tagptrs_T	t_p;
    int		ret;

    name_buf = alloc(name_buf_size);
    if (name_buf == NULL)
	return FAIL;

    if (tagnames)
	extra_flag = TAG_NAMES;
    else
	extra_flag = 0;
    if (pat[0] == '/')
	ret = find_tags(pat + 1, num_file, file,
		TAG_REGEXP | extra_flag | TAG_VERBOSE | TAG_NO_TAGFUNC,
		TAG_MANY, curbuf->b_ffname);
    else
	ret = find_tags(pat, num_file, file,
	      TAG_REGEXP | extra_flag | TAG_VERBOSE | TAG_NO_TAGFUNC | TAG_NOIC,
		TAG_MANY, curbuf->b_ffname);
    if (ret == OK && !tagnames)
    {
	 // Reorganize the tags for display and matching as strings of:
	 // "<tagname>\0<kind>\0<filename>\0"
	 for (i = 0; i < *num_file; i++)
	 {
	     size_t	len;

	     parse_match((*file)[i], &t_p);
	     len = t_p.tagname_end - t_p.tagname;
	     if (len > name_buf_size - 3)
	     {
		 char_u *buf;

		 name_buf_size = len + 3;
		 buf = vim_realloc(name_buf, name_buf_size);
		 if (buf == NULL)
		 {
		     vim_free(name_buf);
		     return FAIL;
		 }
		 name_buf = buf;
	     }

	     mch_memmove(name_buf, t_p.tagname, len);
	     name_buf[len++] = 0;
	     name_buf[len++] = (t_p.tagkind != NULL && *t_p.tagkind)
							  ? *t_p.tagkind : 'f';
	     name_buf[len++] = 0;
	     mch_memmove((*file)[i] + len, t_p.fname,
						    t_p.fname_end - t_p.fname);
	     (*file)[i][len + (t_p.fname_end - t_p.fname)] = 0;
	     mch_memmove((*file)[i], name_buf, len);
	}
    }

    vim_free(name_buf);
    return ret;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Add a tag field to the dictionary "dict".
 * Return OK or FAIL.
 */
    static int
add_tag_field(
    dict_T  *dict,
    char    *field_name,
    char_u  *start,		// start of the value
    char_u  *end)		// after the value; can be NULL
{
    char_u	*buf;
    int		len = 0;
    int		retval;

    // check that the field name doesn't exist yet
    if (dict_has_key(dict, field_name))
    {
	if (p_verbose > 0)
	{
	    verbose_enter();
	    smsg(_("Duplicate field name: %s"), field_name);
	    verbose_leave();
	}
	return FAIL;
    }
    buf = alloc(MAXPATHL);
    if (buf == NULL)
	return FAIL;
    if (start != NULL)
    {
	if (end == NULL)
	{
	    end = start + STRLEN(start);
	    while (end > start && (end[-1] == '\r' || end[-1] == '\n'))
		--end;
	}
	len = (int)(end - start);
	if (len > MAXPATHL - 1)
	    len = MAXPATHL - 1;
	vim_strncpy(buf, start, len);
    }
    buf[len] = NUL;
    retval = dict_add_string(dict, field_name, buf);
    vim_free(buf);
    return retval;
}

/*
 * Add the tags matching the specified pattern "pat" to the list "list"
 * as a dictionary. Use "buf_fname" for priority, unless NULL.
 */
    int
get_tags(list_T *list, char_u *pat, char_u *buf_fname)
{
    int		num_matches, i, ret;
    char_u	**matches, *p;
    char_u	*full_fname;
    dict_T	*dict;
    tagptrs_T	tp;
    long	is_static;

    ret = find_tags(pat, &num_matches, &matches,
				TAG_REGEXP | TAG_NOIC, (int)MAXCOL, buf_fname);
    if (ret != OK || num_matches <= 0)
	return ret;

    for (i = 0; i < num_matches; ++i)
    {
	if (parse_match(matches[i], &tp) == FAIL)
	{
	    vim_free(matches[i]);
	    continue;
	}

	is_static = test_for_static(&tp);

	// Skip pseudo-tag lines.
	if (STRNCMP(tp.tagname, "!_TAG_", 6) == 0)
	{
	    vim_free(matches[i]);
	    continue;
	}

	if ((dict = dict_alloc()) == NULL)
	{
	    ret = FAIL;
	    vim_free(matches[i]);
	    break;
	}
	if (list_append_dict(list, dict) == FAIL)
	    ret = FAIL;

	full_fname = tag_full_fname(&tp);
	if (add_tag_field(dict, "name", tp.tagname, tp.tagname_end) == FAIL
		|| add_tag_field(dict, "filename", full_fname,
		    NULL) == FAIL
		|| add_tag_field(dict, "cmd", tp.command,
		    tp.command_end) == FAIL
		|| add_tag_field(dict, "kind", tp.tagkind,
		    tp.tagkind_end) == FAIL
		|| dict_add_number(dict, "static", is_static) == FAIL)
	    ret = FAIL;

	vim_free(full_fname);

	if (tp.command_end != NULL)
	{
	    for (p = tp.command_end + 3;
		    *p != NUL && *p != '\n' && *p != '\r'; MB_PTR_ADV(p))
	    {
		if (p == tp.tagkind || (p + 5 == tp.tagkind
			    && STRNCMP(p, "kind:", 5) == 0))
		    // skip "kind:<kind>" and "<kind>"
		    p = tp.tagkind_end - 1;
		else if (STRNCMP(p, "file:", 5) == 0)
		    // skip "file:" (static tag)
		    p += 4;
		else if (!VIM_ISWHITE(*p))
		{
		    char_u	*s, *n;
		    int	len;

		    // Add extra field as a dict entry.  Fields are
		    // separated by Tabs.
		    n = p;
		    while (*p != NUL && *p >= ' ' && *p < 127 && *p != ':')
			++p;
		    len = (int)(p - n);
		    if (*p == ':' && len > 0)
		    {
			s = ++p;
			while (*p != NUL && *p >= ' ')
			    ++p;
			n[len] = NUL;
			if (add_tag_field(dict, (char *)n, s, p) == FAIL)
			    ret = FAIL;
			n[len] = ':';
		    }
		    else
			// Skip field without colon.
			while (*p != NUL && *p >= ' ')
			    ++p;
		    if (*p == NUL)
			break;
		}
	    }
	}

	vim_free(matches[i]);
    }
    vim_free(matches);
    return ret;
}

/*
 * Return information about 'tag' in dict 'retdict'.
 */
    static void
get_tag_details(taggy_T *tag, dict_T *retdict)
{
    list_T	*pos;
    fmark_T	*fmark;

    dict_add_string(retdict, "tagname", tag->tagname);
    dict_add_number(retdict, "matchnr", tag->cur_match + 1);
    dict_add_number(retdict, "bufnr", tag->cur_fnum);
    if (tag->user_data)
	dict_add_string(retdict, "user_data", tag->user_data);

    if ((pos = list_alloc_id(aid_tagstack_from)) == NULL)
	return;
    dict_add_list(retdict, "from", pos);

    fmark = &tag->fmark;
    list_append_number(pos,
			(varnumber_T)(fmark->fnum != -1 ? fmark->fnum : 0));
    list_append_number(pos, (varnumber_T)fmark->mark.lnum);
    list_append_number(pos, (varnumber_T)(fmark->mark.col == MAXCOL ?
					MAXCOL : fmark->mark.col + 1));
    list_append_number(pos, (varnumber_T)fmark->mark.coladd);
}

/*
 * Return the tag stack entries of the specified window 'wp' in dictionary
 * 'retdict'.
 */
    void
get_tagstack(win_T *wp, dict_T *retdict)
{
    list_T	*l;
    int		i;
    dict_T	*d;

    dict_add_number(retdict, "length", wp->w_tagstacklen);
    dict_add_number(retdict, "curidx", wp->w_tagstackidx + 1);
    l = list_alloc_id(aid_tagstack_items);
    if (l == NULL)
	return;
    dict_add_list(retdict, "items", l);

    for (i = 0; i < wp->w_tagstacklen; i++)
    {
	if ((d = dict_alloc_id(aid_tagstack_details)) == NULL)
	    return;
	list_append_dict(l, d);

	get_tag_details(&wp->w_tagstack[i], d);
    }
}

/*
 * Free all the entries in the tag stack of the specified window
 */
    static void
tagstack_clear(win_T *wp)
{
    int i;

    // Free the current tag stack
    for (i = 0; i < wp->w_tagstacklen; ++i)
	tagstack_clear_entry(&wp->w_tagstack[i]);
    wp->w_tagstacklen = 0;
    wp->w_tagstackidx = 0;
}

/*
 * Remove the oldest entry from the tag stack and shift the rest of
 * the entries to free up the top of the stack.
 */
    static void
tagstack_shift(win_T *wp)
{
    taggy_T	*tagstack = wp->w_tagstack;
    int		i;

    tagstack_clear_entry(&tagstack[0]);
    for (i = 1; i < wp->w_tagstacklen; ++i)
	tagstack[i - 1] = tagstack[i];
    wp->w_tagstacklen--;
}

/*
 * Push a new item to the tag stack
 */
    static void
tagstack_push_item(
	win_T	*wp,
	char_u	*tagname,
	int	cur_fnum,
	int	cur_match,
	pos_T	mark,
	int	fnum,
	char_u  *user_data)
{
    taggy_T	*tagstack = wp->w_tagstack;
    int		idx = wp->w_tagstacklen;	// top of the stack

    // if the tagstack is full: remove the oldest entry
    if (idx >= TAGSTACKSIZE)
    {
	tagstack_shift(wp);
	idx = TAGSTACKSIZE - 1;
    }

    wp->w_tagstacklen++;
    tagstack[idx].tagname = tagname;
    tagstack[idx].cur_fnum = cur_fnum;
    tagstack[idx].cur_match = cur_match;
    if (tagstack[idx].cur_match < 0)
	tagstack[idx].cur_match = 0;
    tagstack[idx].fmark.mark = mark;
    tagstack[idx].fmark.fnum = fnum;
    tagstack[idx].user_data = user_data;
}

/*
 * Add a list of items to the tag stack in the specified window
 */
    static void
tagstack_push_items(win_T *wp, list_T *l)
{
    listitem_T	*li;
    dictitem_T	*di;
    dict_T	*itemdict;
    char_u	*tagname;
    pos_T	mark;
    int		fnum;

    // Add one entry at a time to the tag stack
    FOR_ALL_LIST_ITEMS(l, li)
    {
	if (li->li_tv.v_type != VAR_DICT || li->li_tv.vval.v_dict == NULL)
	    continue;				// Skip non-dict items
	itemdict = li->li_tv.vval.v_dict;

	// parse 'from' for the cursor position before the tag jump
	if ((di = dict_find(itemdict, (char_u *)"from", -1)) == NULL)
	    continue;
	if (list2fpos(&di->di_tv, &mark, &fnum, NULL, FALSE) != OK)
	    continue;
	if ((tagname = dict_get_string(itemdict, "tagname", TRUE)) == NULL)
	    continue;

	if (mark.col > 0)
	    mark.col--;
	tagstack_push_item(wp, tagname,
		(int)dict_get_number(itemdict, "bufnr"),
		(int)dict_get_number(itemdict, "matchnr") - 1,
		mark, fnum,
		dict_get_string(itemdict, "user_data", TRUE));
    }
}

/*
 * Set the current index in the tag stack. Valid values are between 0
 * and the stack length (inclusive).
 */
    static void
tagstack_set_curidx(win_T *wp, int curidx)
{
    wp->w_tagstackidx = curidx;
    if (wp->w_tagstackidx < 0)			// sanity check
	wp->w_tagstackidx = 0;
    if (wp->w_tagstackidx > wp->w_tagstacklen)
	wp->w_tagstackidx = wp->w_tagstacklen;
}

/*
 * Set the tag stack entries of the specified window.
 * 'action' is set to one of:
 *	'a' for append
 *	'r' for replace
 *	't' for truncate
 */
    int
set_tagstack(win_T *wp, dict_T *d, int action)
{
    dictitem_T	*di;
    list_T	*l = NULL;

#ifdef FEAT_EVAL
    // not allowed to alter the tag stack entries from inside tagfunc
    if (tfu_in_use)
    {
	emsg(_(e_cannot_modify_tag_stack_within_tagfunc));
	return FAIL;
    }
#endif

    if ((di = dict_find(d, (char_u *)"items", -1)) != NULL)
    {
	if (di->di_tv.v_type != VAR_LIST)
	{
	    emsg(_(e_list_required));
	    return FAIL;
	}
	l = di->di_tv.vval.v_list;
    }

    if ((di = dict_find(d, (char_u *)"curidx", -1)) != NULL)
	tagstack_set_curidx(wp, (int)tv_get_number(&di->di_tv) - 1);

    if (action == 't')		    // truncate the stack
    {
	taggy_T	*tagstack = wp->w_tagstack;
	int	tagstackidx = wp->w_tagstackidx;
	int	tagstacklen = wp->w_tagstacklen;

	// delete all the tag stack entries above the current entry
	while (tagstackidx < tagstacklen)
	    tagstack_clear_entry(&tagstack[--tagstacklen]);
	wp->w_tagstacklen = tagstacklen;
    }

    if (l != NULL)
    {
	if (action == 'r')		// replace the stack
	    tagstack_clear(wp);

	tagstack_push_items(wp, l);
	// set the current index after the last entry
	wp->w_tagstackidx = wp->w_tagstacklen;
    }

    return OK;
}
#endif
