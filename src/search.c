/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 * search.c: code for normal mode searching commands
 */

#include "vim.h"

#ifdef FEAT_EVAL
static void set_vv_searchforward(void);
static int first_submatch(regmmatch_T *rp);
#endif
#ifdef FEAT_FIND_ID
static void show_pat_in_path(char_u *, int,
					 int, int, FILE *, linenr_T *, long);
#endif

typedef struct searchstat
{
    int	    cur;	    // current position of found words
    int	    cnt;	    // total count of found words
    int	    exact_match;    // TRUE if matched exactly on specified position
    int	    incomplete;	    // 0: search was fully completed
			    // 1: recomputing was timed out
			    // 2: max count exceeded
    int	    last_maxcount;  // the max count of the last search
} searchstat_T;

static void cmdline_search_stat(int dirc, pos_T *pos, pos_T *cursor_pos, int show_top_bot_msg, char_u *msgbuf, int recompute, int maxcount, long timeout);
static void update_search_stat(int dirc, pos_T *pos, pos_T *cursor_pos, searchstat_T *stat, int recompute, int maxcount, long timeout);

#define SEARCH_STAT_DEF_TIMEOUT 40L
#define SEARCH_STAT_DEF_MAX_COUNT 99
#define SEARCH_STAT_BUF_LEN 12

/*
 * This file contains various searching-related routines. These fall into
 * three groups:
 * 1. string searches (for /, ?, n, and N)
 * 2. character searches within a single line (for f, F, t, T, etc)
 * 3. "other" kinds of searches like the '%' command, and 'word' searches.
 */

/*
 * String searches
 *
 * The string search functions are divided into two levels:
 * lowest:  searchit(); uses an pos_T for starting position and found match.
 * Highest: do_search(); uses curwin->w_cursor; calls searchit().
 *
 * The last search pattern is remembered for repeating the same search.
 * This pattern is shared between the :g, :s, ? and / commands.
 * This is in search_regcomp().
 *
 * The actual string matching is done using a heavily modified version of
 * Henry Spencer's regular expression library.  See regexp.c.
 */

/*
 * Two search patterns are remembered: One for the :substitute command and
 * one for other searches.  last_idx points to the one that was used the last
 * time.
 */
static spat_T spats[2] =
{
    {NULL, TRUE, FALSE, {'/', 0, 0, 0L}},	// last used search pat
    {NULL, TRUE, FALSE, {'/', 0, 0, 0L}}	// last used substitute pat
};

static int last_idx = 0;	// index in spats[] for RE_LAST

static char_u lastc[2] = {NUL, NUL};	// last character searched for
static int lastcdir = FORWARD;		// last direction of character search
static int last_t_cmd = TRUE;		// last search t_cmd
static char_u	lastc_bytes[MB_MAXBYTES + 1];
static int	lastc_bytelen = 1;	// >1 for multi-byte char

// copy of spats[], for keeping the search patterns while executing autocmds
static spat_T	    saved_spats[2];
static char_u	    *saved_mr_pattern = NULL;
# ifdef FEAT_SEARCH_EXTRA
static int	    saved_spats_last_idx = 0;
static int	    saved_spats_no_hlsearch = 0;
# endif

// allocated copy of pattern used by search_regcomp()
static char_u	    *mr_pattern = NULL;

#ifdef FEAT_FIND_ID
/*
 * Type used by find_pattern_in_path() to remember which included files have
 * been searched already.
 */
typedef struct SearchedFile
{
    FILE	*fp;		// File pointer
    char_u	*name;		// Full name of file
    linenr_T	lnum;		// Line we were up to in file
    int		matched;	// Found a match in this file
} SearchedFile;
#endif

/*
 * translate search pattern for vim_regcomp()
 *
 * pat_save == RE_SEARCH: save pat in spats[RE_SEARCH].pat (normal search cmd)
 * pat_save == RE_SUBST: save pat in spats[RE_SUBST].pat (:substitute command)
 * pat_save == RE_BOTH: save pat in both patterns (:global command)
 * pat_use  == RE_SEARCH: use previous search pattern if "pat" is NULL
 * pat_use  == RE_SUBST: use previous substitute pattern if "pat" is NULL
 * pat_use  == RE_LAST: use last used pattern if "pat" is NULL
 * options & SEARCH_HIS: put search string in history
 * options & SEARCH_KEEP: keep previous search pattern
 *
 * returns FAIL if failed, OK otherwise.
 */
    int
search_regcomp(
    char_u	*pat,
    char_u	**used_pat,
    int		pat_save,
    int		pat_use,
    int		options,
    regmmatch_T	*regmatch)	// return: pattern and ignore-case flag
{
    int		magic;
    int		i;

    rc_did_emsg = FALSE;
    magic = magic_isset();

    /*
     * If no pattern given, use a previously defined pattern.
     */
    if (pat == NULL || *pat == NUL)
    {
	if (pat_use == RE_LAST)
	    i = last_idx;
	else
	    i = pat_use;
	if (spats[i].pat == NULL)	// pattern was never defined
	{
	    if (pat_use == RE_SUBST)
		emsg(_(e_no_previous_substitute_regular_expression));
	    else
		emsg(_(e_no_previous_regular_expression));
	    rc_did_emsg = TRUE;
	    return FAIL;
	}
	pat = spats[i].pat;
	magic = spats[i].magic;
	no_smartcase = spats[i].no_scs;
    }
    else if (options & SEARCH_HIS)	// put new pattern in history
	add_to_history(HIST_SEARCH, pat, TRUE, NUL);

    if (used_pat)
	*used_pat = pat;

    vim_free(mr_pattern);
#ifdef FEAT_RIGHTLEFT
    if (curwin->w_p_rl && *curwin->w_p_rlc == 's')
	mr_pattern = reverse_text(pat);
    else
#endif
	mr_pattern = vim_strsave(pat);

    /*
     * Save the currently used pattern in the appropriate place,
     * unless the pattern should not be remembered.
     */
    if (!(options & SEARCH_KEEP)
			       && (cmdmod.cmod_flags & CMOD_KEEPPATTERNS) == 0)
    {
	// search or global command
	if (pat_save == RE_SEARCH || pat_save == RE_BOTH)
	    save_re_pat(RE_SEARCH, pat, magic);
	// substitute or global command
	if (pat_save == RE_SUBST || pat_save == RE_BOTH)
	    save_re_pat(RE_SUBST, pat, magic);
    }

    regmatch->rmm_ic = ignorecase(pat);
    regmatch->rmm_maxcol = 0;
    regmatch->regprog = vim_regcomp(pat, magic ? RE_MAGIC : 0);
    if (regmatch->regprog == NULL)
	return FAIL;
    return OK;
}

/*
 * Get search pattern used by search_regcomp().
 */
    char_u *
get_search_pat(void)
{
    return mr_pattern;
}

    void
save_re_pat(int idx, char_u *pat, int magic)
{
    if (spats[idx].pat == pat)
	return;

    vim_free(spats[idx].pat);
    spats[idx].pat = vim_strsave(pat);
    spats[idx].magic = magic;
    spats[idx].no_scs = no_smartcase;
    last_idx = idx;
#ifdef FEAT_SEARCH_EXTRA
    // If 'hlsearch' set and search pat changed: need redraw.
    if (p_hls)
	redraw_all_later(UPD_SOME_VALID);
    set_no_hlsearch(FALSE);
#endif
}

/*
 * Save the search patterns, so they can be restored later.
 * Used before/after executing autocommands and user functions.
 */
static int save_level = 0;

    void
save_search_patterns(void)
{
    if (save_level++ != 0)
	return;

    saved_spats[0] = spats[0];
    if (spats[0].pat != NULL)
	saved_spats[0].pat = vim_strsave(spats[0].pat);
    saved_spats[1] = spats[1];
    if (spats[1].pat != NULL)
	saved_spats[1].pat = vim_strsave(spats[1].pat);
    if (mr_pattern == NULL)
	saved_mr_pattern = NULL;
    else
	saved_mr_pattern = vim_strsave(mr_pattern);
#ifdef FEAT_SEARCH_EXTRA
    saved_spats_last_idx = last_idx;
    saved_spats_no_hlsearch = no_hlsearch;
#endif
}

    void
restore_search_patterns(void)
{
    if (--save_level != 0)
	return;

    vim_free(spats[0].pat);
    spats[0] = saved_spats[0];
#if defined(FEAT_EVAL)
    set_vv_searchforward();
#endif
    vim_free(spats[1].pat);
    spats[1] = saved_spats[1];
    vim_free(mr_pattern);
    mr_pattern = saved_mr_pattern;
#ifdef FEAT_SEARCH_EXTRA
    last_idx = saved_spats_last_idx;
    set_no_hlsearch(saved_spats_no_hlsearch);
#endif
}

#if defined(EXITFREE) || defined(PROTO)
    void
free_search_patterns(void)
{
    vim_free(spats[0].pat);
    vim_free(spats[1].pat);
    VIM_CLEAR(mr_pattern);
}
#endif

#ifdef FEAT_SEARCH_EXTRA
// copy of spats[RE_SEARCH], for keeping the search patterns while incremental
// searching
static spat_T	    saved_last_search_spat;
static int	    did_save_last_search_spat = 0;
static int	    saved_last_idx = 0;
static int	    saved_no_hlsearch = 0;
static int	    saved_search_match_endcol;
static int	    saved_search_match_lines;

/*
 * Save and restore the search pattern for incremental highlight search
 * feature.
 *
 * It's similar to but different from save_search_patterns() and
 * restore_search_patterns(), because the search pattern must be restored when
 * canceling incremental searching even if it's called inside user functions.
 */
    void
save_last_search_pattern(void)
{
    if (++did_save_last_search_spat != 1)
	// nested call, nothing to do
	return;

    saved_last_search_spat = spats[RE_SEARCH];
    if (spats[RE_SEARCH].pat != NULL)
	saved_last_search_spat.pat = vim_strsave(spats[RE_SEARCH].pat);
    saved_last_idx = last_idx;
    saved_no_hlsearch = no_hlsearch;
}

    void
restore_last_search_pattern(void)
{
    if (--did_save_last_search_spat > 0)
	// nested call, nothing to do
	return;
    if (did_save_last_search_spat != 0)
    {
	iemsg("restore_last_search_pattern() called more often than save_last_search_pattern()");
	return;
    }

    vim_free(spats[RE_SEARCH].pat);
    spats[RE_SEARCH] = saved_last_search_spat;
    saved_last_search_spat.pat = NULL;
# if defined(FEAT_EVAL)
    set_vv_searchforward();
# endif
    last_idx = saved_last_idx;
    set_no_hlsearch(saved_no_hlsearch);
}

/*
 * Save and restore the incsearch highlighting variables.
 * This is required so that calling searchcount() at does not invalidate the
 * incsearch highlighting.
 */
    static void
save_incsearch_state(void)
{
    saved_search_match_endcol = search_match_endcol;
    saved_search_match_lines  = search_match_lines;
}

    static void
restore_incsearch_state(void)
{
    search_match_endcol = saved_search_match_endcol;
    search_match_lines  = saved_search_match_lines;
}

    char_u *
last_search_pattern(void)
{
    return spats[RE_SEARCH].pat;
}
#endif

/*
 * Return TRUE when case should be ignored for search pattern "pat".
 * Uses the 'ignorecase' and 'smartcase' options.
 */
    int
ignorecase(char_u *pat)
{
    return ignorecase_opt(pat, p_ic, p_scs);
}

/*
 * As ignorecase() put pass the "ic" and "scs" flags.
 */
    int
ignorecase_opt(char_u *pat, int ic_in, int scs)
{
    int		ic = ic_in;

    if (ic && !no_smartcase && scs
			    && !(ctrl_x_mode_not_default() && curbuf->b_p_inf))
	ic = !pat_has_uppercase(pat);
    no_smartcase = FALSE;

    return ic;
}

/*
 * Return TRUE if pattern "pat" has an uppercase character.
 */
    int
pat_has_uppercase(char_u *pat)
{
    char_u *p = pat;
    magic_T magic_val = MAGIC_ON;

    // get the magicness of the pattern
    (void)skip_regexp_ex(pat, NUL, magic_isset(), NULL, NULL, &magic_val);

    while (*p != NUL)
    {
	int		l;

	if (has_mbyte && (l = (*mb_ptr2len)(p)) > 1)
	{
	    if (enc_utf8 && utf_isupper(utf_ptr2char(p)))
		return TRUE;
	    p += l;
	}
	else if (*p == '\\' && magic_val <= MAGIC_ON)
	{
	    if (p[1] == '_' && p[2] != NUL)  // skip "\_X"
		p += 3;
	    else if (p[1] == '%' && p[2] != NUL)  // skip "\%X"
		p += 3;
	    else if (p[1] != NUL)  // skip "\X"
		p += 2;
	    else
		p += 1;
	}
	else if ((*p == '%' || *p == '_') && magic_val == MAGIC_ALL)
	{
	    if (p[1] != NUL)  // skip "_X" and %X
		p += 2;
	    else
		p++;
	}
	else if (MB_ISUPPER(*p))
	    return TRUE;
	else
	    ++p;
    }
    return FALSE;
}

#if defined(FEAT_EVAL) || defined(PROTO)
    char_u *
last_csearch(void)
{
    return lastc_bytes;
}

    int
last_csearch_forward(void)
{
    return lastcdir == FORWARD;
}

    int
last_csearch_until(void)
{
    return last_t_cmd == TRUE;
}

    void
set_last_csearch(int c, char_u *s, int len)
{
    *lastc = c;
    lastc_bytelen = len;
    if (len)
	memcpy(lastc_bytes, s, len);
    else
	CLEAR_FIELD(lastc_bytes);
}
#endif

    void
set_csearch_direction(int cdir)
{
    lastcdir = cdir;
}

    void
set_csearch_until(int t_cmd)
{
    last_t_cmd = t_cmd;
}

    char_u *
last_search_pat(void)
{
    return spats[last_idx].pat;
}

/*
 * Reset search direction to forward.  For "gd" and "gD" commands.
 */
    void
reset_search_dir(void)
{
    spats[0].off.dir = '/';
#if defined(FEAT_EVAL)
    set_vv_searchforward();
#endif
}

#if defined(FEAT_EVAL) || defined(FEAT_VIMINFO)
/*
 * Set the last search pattern.  For ":let @/ =" and viminfo.
 * Also set the saved search pattern, so that this works in an autocommand.
 */
    void
set_last_search_pat(
    char_u	*s,
    int		idx,
    int		magic,
    int		setlast)
{
    vim_free(spats[idx].pat);
    // An empty string means that nothing should be matched.
    if (*s == NUL)
	spats[idx].pat = NULL;
    else
	spats[idx].pat = vim_strsave(s);
    spats[idx].magic = magic;
    spats[idx].no_scs = FALSE;
    spats[idx].off.dir = '/';
#if defined(FEAT_EVAL)
    set_vv_searchforward();
#endif
    spats[idx].off.line = FALSE;
    spats[idx].off.end = FALSE;
    spats[idx].off.off = 0;
    if (setlast)
	last_idx = idx;
    if (save_level)
    {
	vim_free(saved_spats[idx].pat);
	saved_spats[idx] = spats[0];
	if (spats[idx].pat == NULL)
	    saved_spats[idx].pat = NULL;
	else
	    saved_spats[idx].pat = vim_strsave(spats[idx].pat);
# ifdef FEAT_SEARCH_EXTRA
	saved_spats_last_idx = last_idx;
# endif
    }
# ifdef FEAT_SEARCH_EXTRA
    // If 'hlsearch' set and search pat changed: need redraw.
    if (p_hls && idx == last_idx && !no_hlsearch)
	redraw_all_later(UPD_SOME_VALID);
# endif
}
#endif

#ifdef FEAT_SEARCH_EXTRA
/*
 * Get a regexp program for the last used search pattern.
 * This is used for highlighting all matches in a window.
 * Values returned in regmatch->regprog and regmatch->rmm_ic.
 */
    void
last_pat_prog(regmmatch_T *regmatch)
{
    if (spats[last_idx].pat == NULL)
    {
	regmatch->regprog = NULL;
	return;
    }
    ++emsg_off;		// So it doesn't beep if bad expr
    (void)search_regcomp((char_u *)"", NULL, 0, last_idx, SEARCH_KEEP, regmatch);
    --emsg_off;
}
#endif

/*
 * Lowest level search function.
 * Search for 'count'th occurrence of pattern "pat" in direction "dir".
 * Start at position "pos" and return the found position in "pos".
 *
 * if (options & SEARCH_MSG) == 0 don't give any messages
 * if (options & SEARCH_MSG) == SEARCH_NFMSG don't give 'notfound' messages
 * if (options & SEARCH_MSG) == SEARCH_MSG give all messages
 * if (options & SEARCH_HIS) put search pattern in history
 * if (options & SEARCH_END) return position at end of match
 * if (options & SEARCH_START) accept match at pos itself
 * if (options & SEARCH_KEEP) keep previous search pattern
 * if (options & SEARCH_FOLD) match only once in a closed fold
 * if (options & SEARCH_PEEK) check for typed char, cancel search
 * if (options & SEARCH_COL) start at pos->col instead of zero
 *
 * Return FAIL (zero) for failure, non-zero for success.
 * When FEAT_EVAL is defined, returns the index of the first matching
 * subpattern plus one; one if there was none.
 */
    int
searchit(
    win_T	*win,		// window to search in; can be NULL for a
				// buffer without a window!
    buf_T	*buf,
    pos_T	*pos,
    pos_T	*end_pos,	// set to end of the match, unless NULL
    int		dir,
    char_u	*pat,
    long	count,
    int		options,
    int		pat_use,	// which pattern to use when "pat" is empty
    searchit_arg_T *extra_arg)	// optional extra arguments, can be NULL
{
    int		found;
    linenr_T	lnum;		// no init to shut up Apollo cc
    colnr_T	col;
    regmmatch_T	regmatch;
    char_u	*ptr;
    colnr_T	matchcol;
    lpos_T	endpos;
    lpos_T	matchpos;
    int		loop;
    pos_T	start_pos;
    int		at_first_line;
    int		extra_col;
    int		start_char_len;
    int		match_ok;
    long	nmatched;
    int		submatch = 0;
    int		first_match = TRUE;
    int		called_emsg_before = called_emsg;
#ifdef FEAT_SEARCH_EXTRA
    int		break_loop = FALSE;
#endif
    linenr_T	stop_lnum = 0;	// stop after this line number when != 0
    int		unused_timeout_flag = FALSE;
    int		*timed_out = &unused_timeout_flag;  // set when timed out.

    if (search_regcomp(pat, NULL, RE_SEARCH, pat_use,
		   (options & (SEARCH_HIS + SEARCH_KEEP)), &regmatch) == FAIL)
    {
	if ((options & SEARCH_MSG) && !rc_did_emsg)
	    semsg(_(e_invalid_search_string_str), mr_pattern);
	return FAIL;
    }

    if (extra_arg != NULL)
    {
	stop_lnum = extra_arg->sa_stop_lnum;
#ifdef FEAT_RELTIME
	if (extra_arg->sa_tm > 0)
	    init_regexp_timeout(extra_arg->sa_tm);
	// Also set the pointer when sa_tm is zero, the caller may have set the
	// timeout.
	timed_out = &extra_arg->sa_timed_out;
#endif
    }

    /*
     * find the string
     */
    do	// loop for count
    {
	// When not accepting a match at the start position set "extra_col" to
	// a non-zero value.  Don't do that when starting at MAXCOL, since
	// MAXCOL + 1 is zero.
	if (pos->col == MAXCOL)
	    start_char_len = 0;
	// Watch out for the "col" being MAXCOL - 2, used in a closed fold.
	else if (has_mbyte
		    && pos->lnum >= 1 && pos->lnum <= buf->b_ml.ml_line_count
						    && pos->col < MAXCOL - 2)
	{
	    ptr = ml_get_buf(buf, pos->lnum, FALSE);
	    if ((int)STRLEN(ptr) <= pos->col)
		start_char_len = 1;
	    else
		start_char_len = (*mb_ptr2len)(ptr + pos->col);
	}
	else
	    start_char_len = 1;
	if (dir == FORWARD)
	{
	    if (options & SEARCH_START)
		extra_col = 0;
	    else
		extra_col = start_char_len;
	}
	else
	{
	    if (options & SEARCH_START)
		extra_col = start_char_len;
	    else
		extra_col = 0;
	}

	start_pos = *pos;	// remember start pos for detecting no match
	found = 0;		// default: not found
	at_first_line = TRUE;	// default: start in first line
	if (pos->lnum == 0)	// correct lnum for when starting in line 0
	{
	    pos->lnum = 1;
	    pos->col = 0;
	    at_first_line = FALSE;  // not in first line now
	}

	/*
	 * Start searching in current line, unless searching backwards and
	 * we're in column 0.
	 * If we are searching backwards, in column 0, and not including the
	 * current position, gain some efficiency by skipping back a line.
	 * Otherwise begin the search in the current line.
	 */
	if (dir == BACKWARD && start_pos.col == 0
					     && (options & SEARCH_START) == 0)
	{
	    lnum = pos->lnum - 1;
	    at_first_line = FALSE;
	}
	else
	    lnum = pos->lnum;

	for (loop = 0; loop <= 1; ++loop)   // loop twice if 'wrapscan' set
	{
	    for ( ; lnum > 0 && lnum <= buf->b_ml.ml_line_count;
					   lnum += dir, at_first_line = FALSE)
	    {
		// Stop after checking "stop_lnum", if it's set.
		if (stop_lnum != 0 && (dir == FORWARD
				       ? lnum > stop_lnum : lnum < stop_lnum))
		    break;
		// Stop after passing the time limit.
		if (*timed_out)
		    break;

		/*
		 * Look for a match somewhere in line "lnum".
		 */
		col = at_first_line && (options & SEARCH_COL) ? pos->col
								 : (colnr_T)0;
		nmatched = vim_regexec_multi(&regmatch, win, buf,
							 lnum, col, timed_out);
		// vim_regexec_multi() may clear "regprog"
		if (regmatch.regprog == NULL)
		    break;
		// Abort searching on an error (e.g., out of stack).
		if (called_emsg > called_emsg_before || *timed_out)
		    break;
		if (nmatched > 0)
		{
		    // match may actually be in another line when using \zs
		    matchpos = regmatch.startpos[0];
		    endpos = regmatch.endpos[0];
#ifdef FEAT_EVAL
		    submatch = first_submatch(&regmatch);
#endif
		    // "lnum" may be past end of buffer for "\n\zs".
		    if (lnum + matchpos.lnum > buf->b_ml.ml_line_count)
			ptr = (char_u *)"";
		    else
			ptr = ml_get_buf(buf, lnum + matchpos.lnum, FALSE);

		    /*
		     * Forward search in the first line: match should be after
		     * the start position. If not, continue at the end of the
		     * match (this is vi compatible) or on the next char.
		     */
		    if (dir == FORWARD && at_first_line)
		    {
			match_ok = TRUE;

			/*
			 * When the match starts in a next line it's certainly
			 * past the start position.
			 * When match lands on a NUL the cursor will be put
			 * one back afterwards, compare with that position,
			 * otherwise "/$" will get stuck on end of line.
			 */
			while (matchpos.lnum == 0
				&& ((options & SEARCH_END) && first_match
				    ?  (nmatched == 1
					&& (int)endpos.col - 1
					     < (int)start_pos.col + extra_col)
				    : ((int)matchpos.col
						  - (ptr[matchpos.col] == NUL)
					    < (int)start_pos.col + extra_col)))
			{
			    /*
			     * If vi-compatible searching, continue at the end
			     * of the match, otherwise continue one position
			     * forward.
			     */
			    if (vim_strchr(p_cpo, CPO_SEARCH) != NULL)
			    {
				if (nmatched > 1)
				{
				    // end is in next line, thus no match in
				    // this line
				    match_ok = FALSE;
				    break;
				}
				matchcol = endpos.col;
				// for empty match: advance one char
				if (matchcol == matchpos.col
						      && ptr[matchcol] != NUL)
				{
				    if (has_mbyte)
					matchcol +=
					  (*mb_ptr2len)(ptr + matchcol);
				    else
					++matchcol;
				}
			    }
			    else
			    {
				// Advance "matchcol" to the next character.
				// This uses rmm_matchcol, the actual start of
				// the match, ignoring "\zs".
				matchcol = regmatch.rmm_matchcol;
				if (ptr[matchcol] != NUL)
				{
				    if (has_mbyte)
					matchcol += (*mb_ptr2len)(ptr
								  + matchcol);
				    else
					++matchcol;
				}
			    }
			    if (matchcol == 0 && (options & SEARCH_START))
				break;
			    if (ptr[matchcol] == NUL
				    || (nmatched = vim_regexec_multi(&regmatch,
					      win, buf, lnum + matchpos.lnum,
					      matchcol, timed_out)) == 0)
			    {
				match_ok = FALSE;
				break;
			    }
			    // vim_regexec_multi() may clear "regprog"
			    if (regmatch.regprog == NULL)
				break;
			    matchpos = regmatch.startpos[0];
			    endpos = regmatch.endpos[0];
# ifdef FEAT_EVAL
			    submatch = first_submatch(&regmatch);
# endif

			    // Need to get the line pointer again, a
			    // multi-line search may have made it invalid.
			    ptr = ml_get_buf(buf, lnum + matchpos.lnum, FALSE);
			}
			if (!match_ok)
			    continue;
		    }
		    if (dir == BACKWARD)
		    {
			/*
			 * Now, if there are multiple matches on this line,
			 * we have to get the last one. Or the last one before
			 * the cursor, if we're on that line.
			 * When putting the new cursor at the end, compare
			 * relative to the end of the match.
			 */
			match_ok = FALSE;
			for (;;)
			{
			    // Remember a position that is before the start
			    // position, we use it if it's the last match in
			    // the line.  Always accept a position after
			    // wrapping around.
			    if (loop
				|| ((options & SEARCH_END)
				    ? (lnum + regmatch.endpos[0].lnum
							      < start_pos.lnum
					|| (lnum + regmatch.endpos[0].lnum
							     == start_pos.lnum
					     && (int)regmatch.endpos[0].col - 1
							< (int)start_pos.col
								+ extra_col))
				    : (lnum + regmatch.startpos[0].lnum
							      < start_pos.lnum
					|| (lnum + regmatch.startpos[0].lnum
							     == start_pos.lnum
					     && (int)regmatch.startpos[0].col
						      < (int)start_pos.col
							      + extra_col))))
			    {
				match_ok = TRUE;
				matchpos = regmatch.startpos[0];
				endpos = regmatch.endpos[0];
# ifdef FEAT_EVAL
				submatch = first_submatch(&regmatch);
# endif
			    }
			    else
				break;

			    /*
			     * We found a valid match, now check if there is
			     * another one after it.
			     * If vi-compatible searching, continue at the end
			     * of the match, otherwise continue one position
			     * forward.
			     */
			    if (vim_strchr(p_cpo, CPO_SEARCH) != NULL)
			    {
				if (nmatched > 1)
				    break;
				matchcol = endpos.col;
				// for empty match: advance one char
				if (matchcol == matchpos.col
						      && ptr[matchcol] != NUL)
				{
				    if (has_mbyte)
					matchcol +=
					  (*mb_ptr2len)(ptr + matchcol);
				    else
					++matchcol;
				}
			    }
			    else
			    {
				// Stop when the match is in a next line.
				if (matchpos.lnum > 0)
				    break;
				matchcol = matchpos.col;
				if (ptr[matchcol] != NUL)
				{
				    if (has_mbyte)
					matchcol +=
					  (*mb_ptr2len)(ptr + matchcol);
				    else
					++matchcol;
				}
			    }
			    if (ptr[matchcol] == NUL
				    || (nmatched = vim_regexec_multi(&regmatch,
					      win, buf, lnum + matchpos.lnum,
					      matchcol, timed_out)) == 0)
			    {
				// If the search timed out, we did find a match
				// but it might be the wrong one, so that's not
				// OK.
				if (*timed_out)
				    match_ok = FALSE;
				break;
			    }
			    // vim_regexec_multi() may clear "regprog"
			    if (regmatch.regprog == NULL)
				break;

			    // Need to get the line pointer again, a
			    // multi-line search may have made it invalid.
			    ptr = ml_get_buf(buf, lnum + matchpos.lnum, FALSE);
			}

			/*
			 * If there is only a match after the cursor, skip
			 * this match.
			 */
			if (!match_ok)
			    continue;
		    }

		    // With the SEARCH_END option move to the last character
		    // of the match.  Don't do it for an empty match, end
		    // should be same as start then.
		    if ((options & SEARCH_END) && !(options & SEARCH_NOOF)
			    && !(matchpos.lnum == endpos.lnum
				&& matchpos.col == endpos.col))
		    {
			// For a match in the first column, set the position
			// on the NUL in the previous line.
			pos->lnum = lnum + endpos.lnum;
			pos->col = endpos.col;
			if (endpos.col == 0)
			{
			    if (pos->lnum > 1)  // just in case
			    {
				--pos->lnum;
				pos->col = (colnr_T)STRLEN(ml_get_buf(buf,
							   pos->lnum, FALSE));
			    }
			}
			else
			{
			    --pos->col;
			    if (has_mbyte
				    && pos->lnum <= buf->b_ml.ml_line_count)
			    {
				ptr = ml_get_buf(buf, pos->lnum, FALSE);
				pos->col -= (*mb_head_off)(ptr, ptr + pos->col);
			    }
			}
			if (end_pos != NULL)
			{
			    end_pos->lnum = lnum + matchpos.lnum;
			    end_pos->col = matchpos.col;
			}
		    }
		    else
		    {
			pos->lnum = lnum + matchpos.lnum;
			pos->col = matchpos.col;
			if (end_pos != NULL)
			{
			    end_pos->lnum = lnum + endpos.lnum;
			    end_pos->col = endpos.col;
			}
		    }
		    pos->coladd = 0;
		    if (end_pos != NULL)
			end_pos->coladd = 0;
		    found = 1;
		    first_match = FALSE;

		    // Set variables used for 'incsearch' highlighting.
		    search_match_lines = endpos.lnum - matchpos.lnum;
		    search_match_endcol = endpos.col;
		    break;
		}
		line_breakcheck();	// stop if ctrl-C typed
		if (got_int)
		    break;

#ifdef FEAT_SEARCH_EXTRA
		// Cancel searching if a character was typed.  Used for
		// 'incsearch'.  Don't check too often, that would slowdown
		// searching too much.
		if ((options & SEARCH_PEEK)
			&& ((lnum - pos->lnum) & 0x3f) == 0
			&& char_avail())
		{
		    break_loop = TRUE;
		    break;
		}
#endif

		if (loop && lnum == start_pos.lnum)
		    break;	    // if second loop, stop where started
	    }
	    at_first_line = FALSE;

	    // vim_regexec_multi() may clear "regprog"
	    if (regmatch.regprog == NULL)
		break;

	    /*
	     * Stop the search if wrapscan isn't set, "stop_lnum" is
	     * specified, after an interrupt, after a match and after looping
	     * twice.
	     */
	    if (!p_ws || stop_lnum != 0 || got_int
			      || called_emsg > called_emsg_before || *timed_out
#ifdef FEAT_SEARCH_EXTRA
			      || break_loop
#endif
			      || found || loop)
		break;

	    /*
	     * If 'wrapscan' is set we continue at the other end of the file.
	     * If 'shortmess' does not contain 's', we give a message, but
	     * only, if we won't show the search stat later anyhow,
	     * (so SEARCH_COUNT must be absent).
	     * This message is also remembered in keep_msg for when the screen
	     * is redrawn. The keep_msg is cleared whenever another message is
	     * written.
	     */
	    if (dir == BACKWARD)    // start second loop at the other end
		lnum = buf->b_ml.ml_line_count;
	    else
		lnum = 1;
	    if (!shortmess(SHM_SEARCH)
		    && shortmess(SHM_SEARCHCOUNT)
		    && (options & SEARCH_MSG))
		give_warning((char_u *)_(dir == BACKWARD
					  ? top_bot_msg : bot_top_msg), TRUE);
	    if (extra_arg != NULL)
		extra_arg->sa_wrapped = TRUE;
	}
	if (got_int || called_emsg > called_emsg_before || *timed_out
#ifdef FEAT_SEARCH_EXTRA
		|| break_loop
#endif
		)
	    break;
    }
    while (--count > 0 && found);   // stop after count matches or no match

#ifdef FEAT_RELTIME
    if (extra_arg != NULL && extra_arg->sa_tm > 0)
	disable_regexp_timeout();
#endif
    vim_regfree(regmatch.regprog);

    if (!found)		    // did not find it
    {
	if (got_int)
	    emsg(_(e_interrupted));
	else if ((options & SEARCH_MSG) == SEARCH_MSG)
	{
	    if (p_ws)
		semsg(_(e_pattern_not_found_str), mr_pattern);
	    else if (lnum == 0)
		semsg(_(e_search_hit_top_without_match_for_str), mr_pattern);
	    else
		semsg(_(e_search_hit_bottom_without_match_for_str), mr_pattern);
	}
	return FAIL;
    }

    // A pattern like "\n\zs" may go past the last line.
    if (pos->lnum > buf->b_ml.ml_line_count)
    {
	pos->lnum = buf->b_ml.ml_line_count;
	pos->col = (int)STRLEN(ml_get_buf(buf, pos->lnum, FALSE));
	if (pos->col > 0)
	    --pos->col;
    }

    return submatch + 1;
}

#if defined(FEAT_EVAL) || defined(FEAT_PROTO)
    void
set_search_direction(int cdir)
{
    spats[0].off.dir = cdir;
}

    static void
set_vv_searchforward(void)
{
    set_vim_var_nr(VV_SEARCHFORWARD, (long)(spats[0].off.dir == '/'));
}

/*
 * Return the number of the first subpat that matched.
 * Return zero if none of them matched.
 */
    static int
first_submatch(regmmatch_T *rp)
{
    int		submatch;

    for (submatch = 1; ; ++submatch)
    {
	if (rp->startpos[submatch].lnum >= 0)
	    break;
	if (submatch == 9)
	{
	    submatch = 0;
	    break;
	}
    }
    return submatch;
}
#endif

/*
 * Highest level string search function.
 * Search for the 'count'th occurrence of pattern 'pat' in direction 'dirc'
 *		  If 'dirc' is 0: use previous dir.
 *    If 'pat' is NULL or empty : use previous string.
 *    If 'options & SEARCH_REV' : go in reverse of previous dir.
 *    If 'options & SEARCH_ECHO': echo the search command and handle options
 *    If 'options & SEARCH_MSG' : may give error message
 *    If 'options & SEARCH_OPT' : interpret optional flags
 *    If 'options & SEARCH_HIS' : put search pattern in history
 *    If 'options & SEARCH_NOOF': don't add offset to position
 *    If 'options & SEARCH_MARK': set previous context mark
 *    If 'options & SEARCH_KEEP': keep previous search pattern
 *    If 'options & SEARCH_START': accept match at curpos itself
 *    If 'options & SEARCH_PEEK': check for typed char, cancel search
 *
 * Careful: If spats[0].off.line == TRUE and spats[0].off.off == 0 this
 * makes the movement linewise without moving the match position.
 *
 * Return 0 for failure, 1 for found, 2 for found and line offset added.
 */
    int
do_search(
    oparg_T	    *oap,	// can be NULL
    int		    dirc,	// '/' or '?'
    int		    search_delim, // the delimiter for the search, e.g. '%' in
				  // s%regex%replacement%
    char_u	    *pat,
    long	    count,
    int		    options,
    searchit_arg_T  *sia)	// optional arguments or NULL
{
    pos_T	    pos;	// position of the last match
    char_u	    *searchstr;
    soffset_T	    old_off;
    int		    retval;	// Return value
    char_u	    *p;
    long	    c;
    char_u	    *dircp;
    char_u	    *strcopy = NULL;
    char_u	    *ps;
    char_u	    *msgbuf = NULL;
    size_t	    len;
    int		    has_offset = FALSE;

    /*
     * A line offset is not remembered, this is vi compatible.
     */
    if (spats[0].off.line && vim_strchr(p_cpo, CPO_LINEOFF) != NULL)
    {
	spats[0].off.line = FALSE;
	spats[0].off.off = 0;
    }

    /*
     * Save the values for when (options & SEARCH_KEEP) is used.
     * (there is no "if ()" around this because gcc wants them initialized)
     */
    old_off = spats[0].off;

    pos = curwin->w_cursor;	// start searching at the cursor position

    /*
     * Find out the direction of the search.
     */
    if (dirc == 0)
	dirc = spats[0].off.dir;
    else
    {
	spats[0].off.dir = dirc;
#if defined(FEAT_EVAL)
	set_vv_searchforward();
#endif
    }
    if (options & SEARCH_REV)
    {
#ifdef MSWIN
	// There is a bug in the Visual C++ 2.2 compiler which means that
	// dirc always ends up being '/'
	dirc = (dirc == '/')  ?  '?'  :  '/';
#else
	if (dirc == '/')
	    dirc = '?';
	else
	    dirc = '/';
#endif
    }

#ifdef FEAT_FOLDING
    // If the cursor is in a closed fold, don't find another match in the same
    // fold.
    if (dirc == '/')
    {
	if (hasFolding(pos.lnum, NULL, &pos.lnum))
	    pos.col = MAXCOL - 2;	// avoid overflow when adding 1
    }
    else
    {
	if (hasFolding(pos.lnum, &pos.lnum, NULL))
	    pos.col = 0;
    }
#endif

#ifdef FEAT_SEARCH_EXTRA
    /*
     * Turn 'hlsearch' highlighting back on.
     */
    if (no_hlsearch && !(options & SEARCH_KEEP))
    {
	redraw_all_later(UPD_SOME_VALID);
	set_no_hlsearch(FALSE);
    }
#endif

    /*
     * Repeat the search when pattern followed by ';', e.g. "/foo/;?bar".
     */
    for (;;)
    {
	int		show_top_bot_msg = FALSE;

	searchstr = pat;
	dircp = NULL;
					    // use previous pattern
	if (pat == NULL || *pat == NUL || *pat == search_delim)
	{
	    if (spats[RE_SEARCH].pat == NULL)	    // no previous pattern
	    {
		searchstr = spats[RE_SUBST].pat;
		if (searchstr == NULL)
		{
		    emsg(_(e_no_previous_regular_expression));
		    retval = 0;
		    goto end_do_search;
		}
	    }
	    else
	    {
		// make search_regcomp() use spats[RE_SEARCH].pat
		searchstr = (char_u *)"";
	    }
	}

	if (pat != NULL && *pat != NUL)	// look for (new) offset
	{
	    /*
	     * Find end of regular expression.
	     * If there is a matching '/' or '?', toss it.
	     */
	    ps = strcopy;
	    p = skip_regexp_ex(pat, search_delim, magic_isset(),
							&strcopy, NULL, NULL);
	    if (strcopy != ps)
	    {
		// made a copy of "pat" to change "\?" to "?"
		searchcmdlen += (int)(STRLEN(pat) - STRLEN(strcopy));
		pat = strcopy;
		searchstr = strcopy;
	    }
	    if (*p == search_delim)
	    {
		dircp = p;	// remember where we put the NUL
		*p++ = NUL;
	    }
	    spats[0].off.line = FALSE;
	    spats[0].off.end = FALSE;
	    spats[0].off.off = 0;
	    /*
	     * Check for a line offset or a character offset.
	     * For get_address (echo off) we don't check for a character
	     * offset, because it is meaningless and the 's' could be a
	     * substitute command.
	     */
	    if (*p == '+' || *p == '-' || VIM_ISDIGIT(*p))
		spats[0].off.line = TRUE;
	    else if ((options & SEARCH_OPT)
				      && (*p == 'e' || *p == 's' || *p == 'b'))
	    {
		if (*p == 'e')		// end
		    spats[0].off.end = SEARCH_END;
		++p;
	    }
	    if (VIM_ISDIGIT(*p) || *p == '+' || *p == '-')  // got an offset
	    {
					    // 'nr' or '+nr' or '-nr'
		if (VIM_ISDIGIT(*p) || VIM_ISDIGIT(*(p + 1)))
		    spats[0].off.off = atol((char *)p);
		else if (*p == '-')	    // single '-'
		    spats[0].off.off = -1;
		else			    // single '+'
		    spats[0].off.off = 1;
		++p;
		while (VIM_ISDIGIT(*p))	    // skip number
		    ++p;
	    }

	    // compute length of search command for get_address()
	    searchcmdlen += (int)(p - pat);

	    pat = p;			    // put pat after search command
	}

	if ((options & SEARCH_ECHO) && messaging()
		&& !msg_silent
		&& (!cmd_silent || !shortmess(SHM_SEARCHCOUNT)))
	{
	    char_u	*trunc;
	    char_u	off_buf[40];
	    size_t	off_len = 0;

	    // Compute msg_row early.
	    msg_start();

	    // Get the offset, so we know how long it is.
	    if (!cmd_silent &&
		    (spats[0].off.line || spats[0].off.end || spats[0].off.off))
	    {
		p = off_buf;
		*p++ = dirc;
		if (spats[0].off.end)
		    *p++ = 'e';
		else if (!spats[0].off.line)
		    *p++ = 's';
		if (spats[0].off.off > 0 || spats[0].off.line)
		    *p++ = '+';
		*p = NUL;
		if (spats[0].off.off != 0 || spats[0].off.line)
		    sprintf((char *)p, "%ld", spats[0].off.off);
		off_len = STRLEN(off_buf);
	    }

	    if (*searchstr == NUL)
		p = spats[0].pat;
	    else
		p = searchstr;

	    if (!shortmess(SHM_SEARCHCOUNT) || cmd_silent)
	    {
		// Reserve enough space for the search pattern + offset +
		// search stat.  Use all the space available, so that the
		// search state is right aligned.  If there is not enough space
		// msg_strtrunc() will shorten in the middle.
		if (msg_scrolled != 0 && !cmd_silent)
		    // Use all the columns.
		    len = (int)(Rows - msg_row) * Columns - 1;
		else
		    // Use up to 'showcmd' column.
		    len = (int)(Rows - msg_row - 1) * Columns + sc_col - 1;
		if (len < STRLEN(p) + off_len + SEARCH_STAT_BUF_LEN + 3)
		    len = STRLEN(p) + off_len + SEARCH_STAT_BUF_LEN + 3;
	    }
	    else
		// Reserve enough space for the search pattern + offset.
		len = STRLEN(p) + off_len + 3;

	    vim_free(msgbuf);
	    msgbuf = alloc(len);
	    if (msgbuf != NULL)
	    {
		vim_memset(msgbuf, ' ', len);
		msgbuf[len - 1] = NUL;
		// do not fill the msgbuf buffer, if cmd_silent is set, leave it
		// empty for the search_stat feature.
		if (!cmd_silent)
		{
		    msgbuf[0] = dirc;

		    if (enc_utf8 && utf_iscomposing(utf_ptr2char(p)))
		    {
			// Use a space to draw the composing char on.
			msgbuf[1] = ' ';
			mch_memmove(msgbuf + 2, p, STRLEN(p));
		    }
		    else
			mch_memmove(msgbuf + 1, p, STRLEN(p));
		    if (off_len > 0)
			mch_memmove(msgbuf + STRLEN(p) + 1, off_buf, off_len);

		    trunc = msg_strtrunc(msgbuf, TRUE);
		    if (trunc != NULL)
		    {
			vim_free(msgbuf);
			msgbuf = trunc;
		    }

#ifdef FEAT_RIGHTLEFT
		    // The search pattern could be shown on the right in
		    // rightleft mode, but the 'ruler' and 'showcmd' area use
		    // it too, thus it would be blanked out again very soon.
		    // Show it on the left, but do reverse the text.
		    if (curwin->w_p_rl && *curwin->w_p_rlc == 's')
		    {
			char_u *r;
			size_t pat_len;

			r = reverse_text(msgbuf);
			if (r != NULL)
			{
			    vim_free(msgbuf);
			    msgbuf = r;
			    // move reversed text to beginning of buffer
			    while (*r != NUL && *r == ' ')
				r++;
			    pat_len = msgbuf + STRLEN(msgbuf) - r;
			    mch_memmove(msgbuf, r, pat_len);
			    // overwrite old text
			    if ((size_t)(r - msgbuf) >= pat_len)
				vim_memset(r, ' ', pat_len);
			    else
				vim_memset(msgbuf + pat_len, ' ', r - msgbuf);
			}
		    }
#endif
		    msg_outtrans(msgbuf);
		    msg_clr_eos();
		    msg_check();

		    gotocmdline(FALSE);
		    out_flush();
		    msg_nowait = TRUE;	    // don't wait for this message
		}
	    }
	}

	/*
	 * If there is a character offset, subtract it from the current
	 * position, so we don't get stuck at "?pat?e+2" or "/pat/s-2".
	 * Skip this if pos.col is near MAXCOL (closed fold).
	 * This is not done for a line offset, because then we would not be vi
	 * compatible.
	 */
	if (!spats[0].off.line && spats[0].off.off && pos.col < MAXCOL - 2)
	{
	    if (spats[0].off.off > 0)
	    {
		for (c = spats[0].off.off; c; --c)
		    if (decl(&pos) == -1)
			break;
		if (c)			// at start of buffer
		{
		    pos.lnum = 0;	// allow lnum == 0 here
		    pos.col = MAXCOL;
		}
	    }
	    else
	    {
		for (c = spats[0].off.off; c; ++c)
		    if (incl(&pos) == -1)
			break;
		if (c)			// at end of buffer
		{
		    pos.lnum = curbuf->b_ml.ml_line_count + 1;
		    pos.col = 0;
		}
	    }
	}

	/*
	 * The actual search.
	 */
	c = searchit(curwin, curbuf, &pos, NULL,
					      dirc == '/' ? FORWARD : BACKWARD,
		searchstr, count, spats[0].off.end + (options &
		       (SEARCH_KEEP + SEARCH_PEEK + SEARCH_HIS
			+ SEARCH_MSG + SEARCH_START
			+ ((pat != NULL && *pat == ';') ? 0 : SEARCH_NOOF))),
		RE_LAST, sia);

	if (dircp != NULL)
	    *dircp = search_delim; // restore second '/' or '?' for normal_cmd()

	if (!shortmess(SHM_SEARCH)
		&& ((dirc == '/' && LT_POS(pos, curwin->w_cursor))
			    || (dirc == '?' && LT_POS(curwin->w_cursor, pos))))
	    show_top_bot_msg = TRUE;

	if (c == FAIL)
	{
	    retval = 0;
	    goto end_do_search;
	}
	if (spats[0].off.end && oap != NULL)
	    oap->inclusive = TRUE;  // 'e' includes last character

	retval = 1;		    // pattern found

	/*
	 * Add character and/or line offset
	 */
	if (!(options & SEARCH_NOOF) || (pat != NULL && *pat == ';'))
	{
	    pos_T org_pos = pos;

	    if (spats[0].off.line)	// Add the offset to the line number.
	    {
		c = pos.lnum + spats[0].off.off;
		if (c < 1)
		    pos.lnum = 1;
		else if (c > curbuf->b_ml.ml_line_count)
		    pos.lnum = curbuf->b_ml.ml_line_count;
		else
		    pos.lnum = c;
		pos.col = 0;

		retval = 2;	    // pattern found, line offset added
	    }
	    else if (pos.col < MAXCOL - 2)	// just in case
	    {
		// to the right, check for end of file
		c = spats[0].off.off;
		if (c > 0)
		{
		    while (c-- > 0)
			if (incl(&pos) == -1)
			    break;
		}
		// to the left, check for start of file
		else
		{
		    while (c++ < 0)
			if (decl(&pos) == -1)
			    break;
		}
	    }
	    if (!EQUAL_POS(pos, org_pos))
		has_offset = TRUE;
	}

	// Show [1/15] if 'S' is not in 'shortmess'.
	if ((options & SEARCH_ECHO)
		&& messaging()
		&& !msg_silent
		&& c != FAIL
		&& !shortmess(SHM_SEARCHCOUNT)
		&& msgbuf != NULL)
	     cmdline_search_stat(dirc, &pos, &curwin->w_cursor,
				show_top_bot_msg, msgbuf,
				(count != 1 || has_offset
#ifdef FEAT_FOLDING
				 || (!(fdo_flags & FDO_SEARCH)
				     && hasFolding(curwin->w_cursor.lnum,
								   NULL, NULL))
#endif
				),
				SEARCH_STAT_DEF_MAX_COUNT,
				SEARCH_STAT_DEF_TIMEOUT);

	/*
	 * The search command can be followed by a ';' to do another search.
	 * For example: "/pat/;/foo/+3;?bar"
	 * This is like doing another search command, except:
	 * - The remembered direction '/' or '?' is from the first search.
	 * - When an error happens the cursor isn't moved at all.
	 * Don't do this when called by get_address() (it handles ';' itself).
	 */
	if (!(options & SEARCH_OPT) || pat == NULL || *pat != ';')
	    break;

	dirc = *++pat;
	search_delim = dirc;
	if (dirc != '?' && dirc != '/')
	{
	    retval = 0;
	    emsg(_(e_expected_question_or_slash_after_semicolon));
	    goto end_do_search;
	}
	++pat;
    }

    if (options & SEARCH_MARK)
	setpcmark();
    curwin->w_cursor = pos;
    curwin->w_set_curswant = TRUE;

end_do_search:
    if ((options & SEARCH_KEEP) || (cmdmod.cmod_flags & CMOD_KEEPPATTERNS))
	spats[0].off = old_off;
    vim_free(strcopy);
    vim_free(msgbuf);

    return retval;
}

/*
 * search_for_exact_line(buf, pos, dir, pat)
 *
 * Search for a line starting with the given pattern (ignoring leading
 * white-space), starting from pos and going in direction "dir". "pos" will
 * contain the position of the match found.    Blank lines match only if
 * ADDING is set.  If p_ic is set then the pattern must be in lowercase.
 * Return OK for success, or FAIL if no line found.
 */
    int
search_for_exact_line(
    buf_T	*buf,
    pos_T	*pos,
    int		dir,
    char_u	*pat)
{
    linenr_T	start = 0;
    char_u	*ptr;
    char_u	*p;

    if (buf->b_ml.ml_line_count == 0)
	return FAIL;
    for (;;)
    {
	pos->lnum += dir;
	if (pos->lnum < 1)
	{
	    if (p_ws)
	    {
		pos->lnum = buf->b_ml.ml_line_count;
		if (!shortmess(SHM_SEARCH))
		    give_warning((char_u *)_(top_bot_msg), TRUE);
	    }
	    else
	    {
		pos->lnum = 1;
		break;
	    }
	}
	else if (pos->lnum > buf->b_ml.ml_line_count)
	{
	    if (p_ws)
	    {
		pos->lnum = 1;
		if (!shortmess(SHM_SEARCH))
		    give_warning((char_u *)_(bot_top_msg), TRUE);
	    }
	    else
	    {
		pos->lnum = 1;
		break;
	    }
	}
	if (pos->lnum == start)
	    break;
	if (start == 0)
	    start = pos->lnum;
	ptr = ml_get_buf(buf, pos->lnum, FALSE);
	p = skipwhite(ptr);
	pos->col = (colnr_T) (p - ptr);

	// when adding lines the matching line may be empty but it is not
	// ignored because we are interested in the next line -- Acevedo
	if (compl_status_adding() && !compl_status_sol())
	{
	    if ((p_ic ? MB_STRICMP(p, pat) : STRCMP(p, pat)) == 0)
		return OK;
	}
	else if (*p != NUL)	// ignore empty lines
	{	// expanding lines or words
	    if ((p_ic ? MB_STRNICMP(p, pat, ins_compl_len())
				   : STRNCMP(p, pat, ins_compl_len())) == 0)
		return OK;
	}
    }
    return FAIL;
}

/*
 * Character Searches
 */

/*
 * Search for a character in a line.  If "t_cmd" is FALSE, move to the
 * position of the character, otherwise move to just before the char.
 * Do this "cap->count1" times.
 * Return FAIL or OK.
 */
    int
searchc(cmdarg_T *cap, int t_cmd)
{
    int			c = cap->nchar;	// char to search for
    int			dir = cap->arg;	// TRUE for searching forward
    long		count = cap->count1;	// repeat count
    int			col;
    char_u		*p;
    int			len;
    int			stop = TRUE;

    if (c != NUL)	// normal search: remember args for repeat
    {
	if (!KeyStuffed)    // don't remember when redoing
	{
	    *lastc = c;
	    set_csearch_direction(dir);
	    set_csearch_until(t_cmd);
	    lastc_bytelen = (*mb_char2bytes)(c, lastc_bytes);
	    if (cap->ncharC1 != 0)
	    {
		lastc_bytelen += (*mb_char2bytes)(cap->ncharC1,
			lastc_bytes + lastc_bytelen);
		if (cap->ncharC2 != 0)
		    lastc_bytelen += (*mb_char2bytes)(cap->ncharC2,
			    lastc_bytes + lastc_bytelen);
	    }
	}
    }
    else		// repeat previous search
    {
	if (*lastc == NUL && lastc_bytelen <= 1)
	    return FAIL;
	if (dir)	// repeat in opposite direction
	    dir = -lastcdir;
	else
	    dir = lastcdir;
	t_cmd = last_t_cmd;
	c = *lastc;
	// For multi-byte re-use last lastc_bytes[] and lastc_bytelen.

	// Force a move of at least one char, so ";" and "," will move the
	// cursor, even if the cursor is right in front of char we are looking
	// at.
	if (vim_strchr(p_cpo, CPO_SCOLON) == NULL && count == 1 && t_cmd)
	    stop = FALSE;
    }

    if (dir == BACKWARD)
	cap->oap->inclusive = FALSE;
    else
	cap->oap->inclusive = TRUE;

    p = ml_get_curline();
    col = curwin->w_cursor.col;
    len = (int)STRLEN(p);

    while (count--)
    {
	if (has_mbyte)
	{
	    for (;;)
	    {
		if (dir > 0)
		{
		    col += (*mb_ptr2len)(p + col);
		    if (col >= len)
			return FAIL;
		}
		else
		{
		    if (col == 0)
			return FAIL;
		    col -= (*mb_head_off)(p, p + col - 1) + 1;
		}
		if (lastc_bytelen <= 1)
		{
		    if (p[col] == c && stop)
			break;
		}
		else if (STRNCMP(p + col, lastc_bytes, lastc_bytelen) == 0
								       && stop)
		    break;
		stop = TRUE;
	    }
	}
	else
	{
	    for (;;)
	    {
		if ((col += dir) < 0 || col >= len)
		    return FAIL;
		if (p[col] == c && stop)
		    break;
		stop = TRUE;
	    }
	}
    }

    if (t_cmd)
    {
	// backup to before the character (possibly double-byte)
	col -= dir;
	if (has_mbyte)
	{
	    if (dir < 0)
		// Landed on the search char which is lastc_bytelen long
		col += lastc_bytelen - 1;
	    else
		// To previous char, which may be multi-byte.
		col -= (*mb_head_off)(p, p + col);
	}
    }
    curwin->w_cursor.col = col;

    return OK;
}

/*
 * "Other" Searches
 */

/*
 * findmatch - find the matching paren or brace
 *
 * Improvement over vi: Braces inside quotes are ignored.
 */
    pos_T *
findmatch(oparg_T *oap, int initc)
{
    return findmatchlimit(oap, initc, 0, 0);
}

/*
 * Return TRUE if the character before "linep[col]" equals "ch".
 * Return FALSE if "col" is zero.
 * Update "*prevcol" to the column of the previous character, unless "prevcol"
 * is NULL.
 * Handles multibyte string correctly.
 */
    static int
check_prevcol(
    char_u	*linep,
    int		col,
    int		ch,
    int		*prevcol)
{
    --col;
    if (col > 0 && has_mbyte)
	col -= (*mb_head_off)(linep, linep + col);
    if (prevcol)
	*prevcol = col;
    return (col >= 0 && linep[col] == ch) ? TRUE : FALSE;
}

/*
 * Raw string start is found at linep[startpos.col - 1].
 * Return TRUE if the matching end can be found between startpos and endpos.
 */
    static int
find_rawstring_end(char_u *linep, pos_T *startpos, pos_T *endpos)
{
    char_u	*p;
    char_u	*delim_copy;
    size_t	delim_len;
    linenr_T	lnum;
    int		found = FALSE;

    for (p = linep + startpos->col + 1; *p && *p != '('; ++p)
	;
    delim_len = (p - linep) - startpos->col - 1;
    delim_copy = vim_strnsave(linep + startpos->col + 1, delim_len);
    if (delim_copy == NULL)
	return FALSE;
    for (lnum = startpos->lnum; lnum <= endpos->lnum; ++lnum)
    {
	char_u *line = ml_get(lnum);

	for (p = line + (lnum == startpos->lnum
					    ? startpos->col + 1 : 0); *p; ++p)
	{
	    if (lnum == endpos->lnum && (colnr_T)(p - line) >= endpos->col)
		break;
	    if (*p == ')' && STRNCMP(delim_copy, p + 1, delim_len) == 0
			  && p[delim_len + 1] == '"')
	    {
		found = TRUE;
		break;
	    }
	}
	if (found)
	    break;
    }
    vim_free(delim_copy);
    return found;
}

/*
 * Check matchpairs option for "*initc".
 * If there is a match set "*initc" to the matching character and "*findc" to
 * the opposite character.  Set "*backwards" to the direction.
 * When "switchit" is TRUE swap the direction.
 */
    static void
find_mps_values(
    int	    *initc,
    int	    *findc,
    int	    *backwards,
    int	    switchit)
{
    char_u	*ptr;

    ptr = curbuf->b_p_mps;
    while (*ptr != NUL)
    {
	if (has_mbyte)
	{
	    char_u *prev;

	    if (mb_ptr2char(ptr) == *initc)
	    {
		if (switchit)
		{
		    *findc = *initc;
		    *initc = mb_ptr2char(ptr + mb_ptr2len(ptr) + 1);
		    *backwards = TRUE;
		}
		else
		{
		    *findc = mb_ptr2char(ptr + mb_ptr2len(ptr) + 1);
		    *backwards = FALSE;
		}
		return;
	    }
	    prev = ptr;
	    ptr += mb_ptr2len(ptr) + 1;
	    if (mb_ptr2char(ptr) == *initc)
	    {
		if (switchit)
		{
		    *findc = *initc;
		    *initc = mb_ptr2char(prev);
		    *backwards = FALSE;
		}
		else
		{
		    *findc = mb_ptr2char(prev);
		    *backwards = TRUE;
		}
		return;
	    }
	    ptr += mb_ptr2len(ptr);
	}
	else
	{
	    if (*ptr == *initc)
	    {
		if (switchit)
		{
		    *backwards = TRUE;
		    *findc = *initc;
		    *initc = ptr[2];
		}
		else
		{
		    *backwards = FALSE;
		    *findc = ptr[2];
		}
		return;
	    }
	    ptr += 2;
	    if (*ptr == *initc)
	    {
		if (switchit)
		{
		    *backwards = FALSE;
		    *findc = *initc;
		    *initc = ptr[-2];
		}
		else
		{
		    *backwards = TRUE;
		    *findc =  ptr[-2];
		}
		return;
	    }
	    ++ptr;
	}
	if (*ptr == ',')
	    ++ptr;
    }
}

/*
 * findmatchlimit -- find the matching paren or brace, if it exists within
 * maxtravel lines of the cursor.  A maxtravel of 0 means search until falling
 * off the edge of the file.
 *
 * "initc" is the character to find a match for.  NUL means to find the
 * character at or after the cursor. Special values:
 * '*'  look for C-style comment / *
 * '/'  look for C-style comment / *, ignoring comment-end
 * '#'  look for preprocessor directives
 * 'R'  look for raw string start: R"delim(text)delim" (only backwards)
 *
 * flags: FM_BACKWARD	search backwards (when initc is '/', '*' or '#')
 *	  FM_FORWARD	search forwards (when initc is '/', '*' or '#')
 *	  FM_BLOCKSTOP	stop at start/end of block ({ or } in column 0)
 *	  FM_SKIPCOMM	skip comments (not implemented yet!)
 *
 * "oap" is only used to set oap->motion_type for a linewise motion, it can be
 * NULL
 */
    pos_T *
findmatchlimit(
    oparg_T	*oap,
    int		initc,
    int		flags,
    int		maxtravel)
{
    static pos_T pos;			// current search position
    int		findc = 0;		// matching brace
    int		c;
    int		count = 0;		// cumulative number of braces
    int		backwards = FALSE;	// init for gcc
    int		raw_string = FALSE;	// search for raw string
    int		inquote = FALSE;	// TRUE when inside quotes
    char_u	*linep;			// pointer to current line
    char_u	*ptr;
    int		do_quotes;		// check for quotes in current line
    int		at_start;		// do_quotes value at start position
    int		hash_dir = 0;		// Direction searched for # things
    int		comment_dir = 0;	// Direction searched for comments
    pos_T	match_pos;		// Where last slash-star was found
    int		start_in_quotes;	// start position is in quotes
    int		traveled = 0;		// how far we've searched so far
    int		ignore_cend = FALSE;    // ignore comment end
    int		cpo_match;		// vi compatible matching
    int		cpo_bsl;		// don't recognize backslashes
    int		match_escaped = 0;	// search for escaped match
    int		dir;			// Direction to search
    int		comment_col = MAXCOL;   // start of / / comment
    int		lispcomm = FALSE;	// inside of Lisp-style comment
    int		lisp = curbuf->b_p_lisp; // engage Lisp-specific hacks ;)

    pos = curwin->w_cursor;
    pos.coladd = 0;
    linep = ml_get(pos.lnum);

    cpo_match = (vim_strchr(p_cpo, CPO_MATCH) != NULL);
    cpo_bsl = (vim_strchr(p_cpo, CPO_MATCHBSL) != NULL);

    // Direction to search when initc is '/', '*' or '#'
    if (flags & FM_BACKWARD)
	dir = BACKWARD;
    else if (flags & FM_FORWARD)
	dir = FORWARD;
    else
	dir = 0;

    /*
     * if initc given, look in the table for the matching character
     * '/' and '*' are special cases: look for start or end of comment.
     * When '/' is used, we ignore running backwards into an star-slash, for
     * "[*" command, we just want to find any comment.
     */
    if (initc == '/' || initc == '*' || initc == 'R')
    {
	comment_dir = dir;
	if (initc == '/')
	    ignore_cend = TRUE;
	backwards = (dir == FORWARD) ? FALSE : TRUE;
	raw_string = (initc == 'R');
	initc = NUL;
    }
    else if (initc != '#' && initc != NUL)
    {
	find_mps_values(&initc, &findc, &backwards, TRUE);
	if (dir)
	    backwards = (dir == FORWARD) ? FALSE : TRUE;
	if (findc == NUL)
	    return NULL;
    }
    else
    {
	/*
	 * Either initc is '#', or no initc was given and we need to look
	 * under the cursor.
	 */
	if (initc == '#')
	{
	    hash_dir = dir;
	}
	else
	{
	    /*
	     * initc was not given, must look for something to match under
	     * or near the cursor.
	     * Only check for special things when 'cpo' doesn't have '%'.
	     */
	    if (!cpo_match)
	    {
		// Are we before or at #if, #else etc.?
		ptr = skipwhite(linep);
		if (*ptr == '#' && pos.col <= (colnr_T)(ptr - linep))
		{
		    ptr = skipwhite(ptr + 1);
		    if (   STRNCMP(ptr, "if", 2) == 0
			|| STRNCMP(ptr, "endif", 5) == 0
			|| STRNCMP(ptr, "el", 2) == 0)
			hash_dir = 1;
		}

		// Are we on a comment?
		else if (linep[pos.col] == '/')
		{
		    if (linep[pos.col + 1] == '*')
		    {
			comment_dir = FORWARD;
			backwards = FALSE;
			pos.col++;
		    }
		    else if (pos.col > 0 && linep[pos.col - 1] == '*')
		    {
			comment_dir = BACKWARD;
			backwards = TRUE;
			pos.col--;
		    }
		}
		else if (linep[pos.col] == '*')
		{
		    if (linep[pos.col + 1] == '/')
		    {
			comment_dir = BACKWARD;
			backwards = TRUE;
		    }
		    else if (pos.col > 0 && linep[pos.col - 1] == '/')
		    {
			comment_dir = FORWARD;
			backwards = FALSE;
		    }
		}
	    }

	    /*
	     * If we are not on a comment or the # at the start of a line, then
	     * look for brace anywhere on this line after the cursor.
	     */
	    if (!hash_dir && !comment_dir)
	    {
		/*
		 * Find the brace under or after the cursor.
		 * If beyond the end of the line, use the last character in
		 * the line.
		 */
		if (linep[pos.col] == NUL && pos.col)
		    --pos.col;
		for (;;)
		{
		    initc = PTR2CHAR(linep + pos.col);
		    if (initc == NUL)
			break;

		    find_mps_values(&initc, &findc, &backwards, FALSE);
		    if (findc)
			break;
		    pos.col += mb_ptr2len(linep + pos.col);
		}
		if (!findc)
		{
		    // no brace in the line, maybe use "  #if" then
		    if (!cpo_match && *skipwhite(linep) == '#')
			hash_dir = 1;
		    else
			return NULL;
		}
		else if (!cpo_bsl)
		{
		    int col, bslcnt = 0;

		    // Set "match_escaped" if there are an odd number of
		    // backslashes.
		    for (col = pos.col; check_prevcol(linep, col, '\\', &col);)
			bslcnt++;
		    match_escaped = (bslcnt & 1);
		}
	    }
	}
	if (hash_dir)
	{
	    /*
	     * Look for matching #if, #else, #elif, or #endif
	     */
	    if (oap != NULL)
		oap->motion_type = MLINE;   // Linewise for this case only
	    if (initc != '#')
	    {
		ptr = skipwhite(skipwhite(linep) + 1);
		if (STRNCMP(ptr, "if", 2) == 0 || STRNCMP(ptr, "el", 2) == 0)
		    hash_dir = 1;
		else if (STRNCMP(ptr, "endif", 5) == 0)
		    hash_dir = -1;
		else
		    return NULL;
	    }
	    pos.col = 0;
	    while (!got_int)
	    {
		if (hash_dir > 0)
		{
		    if (pos.lnum == curbuf->b_ml.ml_line_count)
			break;
		}
		else if (pos.lnum == 1)
		    break;
		pos.lnum += hash_dir;
		linep = ml_get(pos.lnum);
		line_breakcheck();	// check for CTRL-C typed
		ptr = skipwhite(linep);
		if (*ptr != '#')
		    continue;
		pos.col = (colnr_T) (ptr - linep);
		ptr = skipwhite(ptr + 1);
		if (hash_dir > 0)
		{
		    if (STRNCMP(ptr, "if", 2) == 0)
			count++;
		    else if (STRNCMP(ptr, "el", 2) == 0)
		    {
			if (count == 0)
			    return &pos;
		    }
		    else if (STRNCMP(ptr, "endif", 5) == 0)
		    {
			if (count == 0)
			    return &pos;
			count--;
		    }
		}
		else
		{
		    if (STRNCMP(ptr, "if", 2) == 0)
		    {
			if (count == 0)
			    return &pos;
			count--;
		    }
		    else if (initc == '#' && STRNCMP(ptr, "el", 2) == 0)
		    {
			if (count == 0)
			    return &pos;
		    }
		    else if (STRNCMP(ptr, "endif", 5) == 0)
			count++;
		}
	    }
	    return NULL;
	}
    }

#ifdef FEAT_RIGHTLEFT
    // This is just guessing: when 'rightleft' is set, search for a matching
    // paren/brace in the other direction.
    if (curwin->w_p_rl && vim_strchr((char_u *)"()[]{}<>", initc) != NULL)
	backwards = !backwards;
#endif

    do_quotes = -1;
    start_in_quotes = MAYBE;
    CLEAR_POS(&match_pos);

    // backward search: Check if this line contains a single-line comment
    if ((backwards && comment_dir) || lisp)
	comment_col = check_linecomment(linep);
    if (lisp && comment_col != MAXCOL && pos.col > (colnr_T)comment_col)
	lispcomm = TRUE;    // find match inside this comment

    while (!got_int)
    {
	/*
	 * Go to the next position, forward or backward. We could use
	 * inc() and dec() here, but that is much slower
	 */
	if (backwards)
	{
	    // char to match is inside of comment, don't search outside
	    if (lispcomm && pos.col < (colnr_T)comment_col)
		break;
	    if (pos.col == 0)		// at start of line, go to prev. one
	    {
		if (pos.lnum == 1)	// start of file
		    break;
		--pos.lnum;

		if (maxtravel > 0 && ++traveled > maxtravel)
		    break;

		linep = ml_get(pos.lnum);
		pos.col = (colnr_T)STRLEN(linep); // pos.col on trailing NUL
		do_quotes = -1;
		line_breakcheck();

		// Check if this line contains a single-line comment
		if (comment_dir || lisp)
		    comment_col = check_linecomment(linep);
		// skip comment
		if (lisp && comment_col != MAXCOL)
		    pos.col = comment_col;
	    }
	    else
	    {
		--pos.col;
		if (has_mbyte)
		    pos.col -= (*mb_head_off)(linep, linep + pos.col);
	    }
	}
	else				// forward search
	{
	    if (linep[pos.col] == NUL
		    // at end of line, go to next one
		    // For lisp don't search for match in comment
		    || (lisp && comment_col != MAXCOL
					   && pos.col == (colnr_T)comment_col))
	    {
		if (pos.lnum == curbuf->b_ml.ml_line_count  // end of file
			// line is exhausted and comment with it,
			// don't search for match in code
			 || lispcomm)
		    break;
		++pos.lnum;

		if (maxtravel && traveled++ > maxtravel)
		    break;

		linep = ml_get(pos.lnum);
		pos.col = 0;
		do_quotes = -1;
		line_breakcheck();
		if (lisp)   // find comment pos in new line
		    comment_col = check_linecomment(linep);
	    }
	    else
	    {
		if (has_mbyte)
		    pos.col += (*mb_ptr2len)(linep + pos.col);
		else
		    ++pos.col;
	    }
	}

	/*
	 * If FM_BLOCKSTOP given, stop at a '{' or '}' in column 0.
	 */
	if (pos.col == 0 && (flags & FM_BLOCKSTOP)
				       && (linep[0] == '{' || linep[0] == '}'))
	{
	    if (linep[0] == findc && count == 0)	// match!
		return &pos;
	    break;					// out of scope
	}

	if (comment_dir)
	{
	    // Note: comments do not nest, and we ignore quotes in them
	    // TODO: ignore comment brackets inside strings
	    if (comment_dir == FORWARD)
	    {
		if (linep[pos.col] == '*' && linep[pos.col + 1] == '/')
		{
		    pos.col++;
		    return &pos;
		}
	    }
	    else    // Searching backwards
	    {
		/*
		 * A comment may contain / * or / /, it may also start or end
		 * with / * /.	Ignore a / * after / / and after *.
		 */
		if (pos.col == 0)
		    continue;
		else if (raw_string)
		{
		    if (linep[pos.col - 1] == 'R'
			&& linep[pos.col] == '"'
			&& vim_strchr(linep + pos.col + 1, '(') != NULL)
		    {
			// Possible start of raw string. Now that we have the
			// delimiter we can check if it ends before where we
			// started searching, or before the previously found
			// raw string start.
			if (!find_rawstring_end(linep, &pos,
				  count > 0 ? &match_pos : &curwin->w_cursor))
			{
			    count++;
			    match_pos = pos;
			    match_pos.col--;
			}
			linep = ml_get(pos.lnum); // may have been released
		    }
		}
		else if (  linep[pos.col - 1] == '/'
			&& linep[pos.col] == '*'
			&& (pos.col == 1 || linep[pos.col - 2] != '*')
			&& (int)pos.col < comment_col)
		{
		    count++;
		    match_pos = pos;
		    match_pos.col--;
		}
		else if (linep[pos.col - 1] == '*' && linep[pos.col] == '/')
		{
		    if (count > 0)
			pos = match_pos;
		    else if (pos.col > 1 && linep[pos.col - 2] == '/'
					       && (int)pos.col <= comment_col)
			pos.col -= 2;
		    else if (ignore_cend)
			continue;
		    else
			return NULL;
		    return &pos;
		}
	    }
	    continue;
	}

	/*
	 * If smart matching ('cpoptions' does not contain '%'), braces inside
	 * of quotes are ignored, but only if there is an even number of
	 * quotes in the line.
	 */
	if (cpo_match)
	    do_quotes = 0;
	else if (do_quotes == -1)
	{
	    /*
	     * Count the number of quotes in the line, skipping \" and '"'.
	     * Watch out for "\\".
	     */
	    at_start = do_quotes;
	    for (ptr = linep; *ptr; ++ptr)
	    {
		if (ptr == linep + pos.col + backwards)
		    at_start = (do_quotes & 1);
		if (*ptr == '"'
			&& (ptr == linep || ptr[-1] != '\'' || ptr[1] != '\''))
		    ++do_quotes;
		if (*ptr == '\\' && ptr[1] != NUL)
		    ++ptr;
	    }
	    do_quotes &= 1;	    // result is 1 with even number of quotes

	    /*
	     * If we find an uneven count, check current line and previous
	     * one for a '\' at the end.
	     */
	    if (!do_quotes)
	    {
		inquote = FALSE;
		if (ptr[-1] == '\\')
		{
		    do_quotes = 1;
		    if (start_in_quotes == MAYBE)
		    {
			// Do we need to use at_start here?
			inquote = TRUE;
			start_in_quotes = TRUE;
		    }
		    else if (backwards)
			inquote = TRUE;
		}
		if (pos.lnum > 1)
		{
		    ptr = ml_get(pos.lnum - 1);
		    if (*ptr && *(ptr + STRLEN(ptr) - 1) == '\\')
		    {
			do_quotes = 1;
			if (start_in_quotes == MAYBE)
			{
			    inquote = at_start;
			    if (inquote)
				start_in_quotes = TRUE;
			}
			else if (!backwards)
			    inquote = TRUE;
		    }

		    // ml_get() only keeps one line, need to get linep again
		    linep = ml_get(pos.lnum);
		}
	    }
	}
	if (start_in_quotes == MAYBE)
	    start_in_quotes = FALSE;

	/*
	 * If 'smartmatch' is set:
	 *   Things inside quotes are ignored by setting 'inquote'.  If we
	 *   find a quote without a preceding '\' invert 'inquote'.  At the
	 *   end of a line not ending in '\' we reset 'inquote'.
	 *
	 *   In lines with an uneven number of quotes (without preceding '\')
	 *   we do not know which part to ignore. Therefore we only set
	 *   inquote if the number of quotes in a line is even, unless this
	 *   line or the previous one ends in a '\'.  Complicated, isn't it?
	 */
	c = PTR2CHAR(linep + pos.col);
	switch (c)
	{
	case NUL:
	    // at end of line without trailing backslash, reset inquote
	    if (pos.col == 0 || linep[pos.col - 1] != '\\')
	    {
		inquote = FALSE;
		start_in_quotes = FALSE;
	    }
	    break;

	case '"':
	    // a quote that is preceded with an odd number of backslashes is
	    // ignored
	    if (do_quotes)
	    {
		int col;

		for (col = pos.col - 1; col >= 0; --col)
		    if (linep[col] != '\\')
			break;
		if ((((int)pos.col - 1 - col) & 1) == 0)
		{
		    inquote = !inquote;
		    start_in_quotes = FALSE;
		}
	    }
	    break;

	/*
	 * If smart matching ('cpoptions' does not contain '%'):
	 *   Skip things in single quotes: 'x' or '\x'.  Be careful for single
	 *   single quotes, eg jon's.  Things like '\233' or '\x3f' are not
	 *   skipped, there is never a brace in them.
	 *   Ignore this when finding matches for `'.
	 */
	case '\'':
	    if (!cpo_match && initc != '\'' && findc != '\'')
	    {
		if (backwards)
		{
		    if (pos.col > 1)
		    {
			if (linep[pos.col - 2] == '\'')
			{
			    pos.col -= 2;
			    break;
			}
			else if (linep[pos.col - 2] == '\\'
				  && pos.col > 2 && linep[pos.col - 3] == '\'')
			{
			    pos.col -= 3;
			    break;
			}
		    }
		}
		else if (linep[pos.col + 1])	// forward search
		{
		    if (linep[pos.col + 1] == '\\'
			   && linep[pos.col + 2] && linep[pos.col + 3] == '\'')
		    {
			pos.col += 3;
			break;
		    }
		    else if (linep[pos.col + 2] == '\'')
		    {
			pos.col += 2;
			break;
		    }
		}
	    }
	    // FALLTHROUGH

	default:
	    /*
	     * For Lisp skip over backslashed (), {} and [].
	     * (actually, we skip #\( et al)
	     */
	    if (curbuf->b_p_lisp
		    && vim_strchr((char_u *)"{}()[]", c) != NULL
		    && pos.col > 1
		    && check_prevcol(linep, pos.col, '\\', NULL)
		    && check_prevcol(linep, pos.col - 1, '#', NULL))
		break;

	    // Check for match outside of quotes, and inside of
	    // quotes when the start is also inside of quotes.
	    if ((!inquote || start_in_quotes == TRUE)
		    && (c == initc || c == findc))
	    {
		int	col, bslcnt = 0;

		if (!cpo_bsl)
		{
		    for (col = pos.col; check_prevcol(linep, col, '\\', &col);)
			bslcnt++;
		}
		// Only accept a match when 'M' is in 'cpo' or when escaping
		// is what we expect.
		if (cpo_bsl || (bslcnt & 1) == match_escaped)
		{
		    if (c == initc)
			count++;
		    else
		    {
			if (count == 0)
			    return &pos;
			count--;
		    }
		}
	    }
	}
    }

    if (comment_dir == BACKWARD && count > 0)
    {
	pos = match_pos;
	return &pos;
    }
    return (pos_T *)NULL;	// never found it
}

/*
 * Check if line[] contains a / / comment.
 * Return MAXCOL if not, otherwise return the column.
 */
    int
check_linecomment(char_u *line)
{
    char_u  *p;

    p = line;
    // skip Lispish one-line comments
    if (curbuf->b_p_lisp)
    {
	if (vim_strchr(p, ';') != NULL) // there may be comments
	{
	    int in_str = FALSE;	// inside of string

	    p = line;		// scan from start
	    while ((p = vim_strpbrk(p, (char_u *)"\";")) != NULL)
	    {
		if (*p == '"')
		{
		    if (in_str)
		    {
			if (*(p - 1) != '\\') // skip escaped quote
			    in_str = FALSE;
		    }
		    else if (p == line || ((p - line) >= 2
				      // skip #\" form
				      && *(p - 1) != '\\' && *(p - 2) != '#'))
			in_str = TRUE;
		}
		else if (!in_str && ((p - line) < 2
				    || (*(p - 1) != '\\' && *(p - 2) != '#'))
			       && !is_pos_in_string(line, (colnr_T)(p - line)))
		    break;	// found!
		++p;
	    }
	}
	else
	    p = NULL;
    }
    else
	while ((p = vim_strchr(p, '/')) != NULL)
	{
	    // Accept a double /, unless it's preceded with * and followed by
	    // *, because * / / * is an end and start of a C comment.  Only
	    // accept the position if it is not inside a string.
	    if (p[1] == '/' && (p == line || p[-1] != '*' || p[2] != '*')
			       && !is_pos_in_string(line, (colnr_T)(p - line)))
		break;
	    ++p;
	}

    if (p == NULL)
	return MAXCOL;
    return (int)(p - line);
}

/*
 * Move cursor briefly to character matching the one under the cursor.
 * Used for Insert mode and "r" command.
 * Show the match only if it is visible on the screen.
 * If there isn't a match, then beep.
 */
    void
showmatch(
    int		c)	    // char to show match for
{
    pos_T	*lpos, save_cursor;
    pos_T	mpos;
    colnr_T	vcol;
    long	save_so;
    long	save_siso;
#ifdef CURSOR_SHAPE
    int		save_state;
#endif
    colnr_T	save_dollar_vcol;
    char_u	*p;
    long	*so = curwin->w_p_so >= 0 ? &curwin->w_p_so : &p_so;
    long	*siso = curwin->w_p_siso >= 0 ? &curwin->w_p_siso : &p_siso;

    /*
     * Only show match for chars in the 'matchpairs' option.
     */
    // 'matchpairs' is "x:y,x:y"
    for (p = curbuf->b_p_mps; *p != NUL; ++p)
    {
#ifdef FEAT_RIGHTLEFT
	if (PTR2CHAR(p) == c && (curwin->w_p_rl ^ p_ri))
	    break;
#endif
	p += mb_ptr2len(p) + 1;
	if (PTR2CHAR(p) == c
#ifdef FEAT_RIGHTLEFT
		&& !(curwin->w_p_rl ^ p_ri)
#endif
	   )
	    break;
	p += mb_ptr2len(p);
	if (*p == NUL)
	    return;
    }
    if (*p == NUL)
	return;

    if ((lpos = findmatch(NULL, NUL)) == NULL)	    // no match, so beep
    {
	vim_beep(BO_MATCH);
	return;
    }

    if (lpos->lnum < curwin->w_topline || lpos->lnum >= curwin->w_botline)
	return;

    if (!curwin->w_p_wrap)
	getvcol(curwin, lpos, NULL, &vcol, NULL);

    int col_visible = (curwin->w_p_wrap
	    || (vcol >= curwin->w_leftcol
		&& vcol < curwin->w_leftcol + curwin->w_width));
    if (!col_visible)
	return;

    mpos = *lpos;    // save the pos, update_screen() may change it
    save_cursor = curwin->w_cursor;
    save_so = *so;
    save_siso = *siso;
    // Handle "$" in 'cpo': If the ')' is typed on top of the "$",
    // stop displaying the "$".
    if (dollar_vcol >= 0 && dollar_vcol == curwin->w_virtcol)
	dollar_vcol = -1;
    ++curwin->w_virtcol;	// do display ')' just before "$"
    update_screen(UPD_VALID);	// show the new char first

    save_dollar_vcol = dollar_vcol;
#ifdef CURSOR_SHAPE
    save_state = State;
    State = MODE_SHOWMATCH;
    ui_cursor_shape();		// may show different cursor shape
#endif
    curwin->w_cursor = mpos;	// move to matching char
    *so = 0;			// don't use 'scrolloff' here
    *siso = 0;			// don't use 'sidescrolloff' here
    showruler(FALSE);
    setcursor();
    cursor_on();		// make sure that the cursor is shown
    out_flush_cursor(TRUE, FALSE);

    // Restore dollar_vcol(), because setcursor() may call curs_rows()
    // which resets it if the matching position is in a previous line
    // and has a higher column number.
    dollar_vcol = save_dollar_vcol;

    /*
     * brief pause, unless 'm' is present in 'cpo' and a character is
     * available.
     */
    if (vim_strchr(p_cpo, CPO_SHOWMATCH) != NULL)
	ui_delay(p_mat * 100L + 8, TRUE);
    else if (!char_avail())
	ui_delay(p_mat * 100L + 9, FALSE);
    curwin->w_cursor = save_cursor;	// restore cursor position
    *so = save_so;
    *siso = save_siso;
#ifdef CURSOR_SHAPE
    State = save_state;
    ui_cursor_shape();		// may show different cursor shape
#endif
}

/*
 * Check if the pattern is zero-width.
 * If move is TRUE, check from the beginning of the buffer, else from position
 * "cur".
 * "direction" is FORWARD or BACKWARD.
 * Returns TRUE, FALSE or -1 for failure.
 */
    static int
is_zero_width(char_u *pattern, int move, pos_T *cur, int direction)
{
    regmmatch_T	regmatch;
    int		nmatched = 0;
    int		result = -1;
    pos_T	pos;
    int		called_emsg_before = called_emsg;
    int		flag = 0;

    if (pattern == NULL)
	pattern = spats[last_idx].pat;

    if (search_regcomp(pattern, NULL, RE_SEARCH, RE_SEARCH,
					      SEARCH_KEEP, &regmatch) == FAIL)
	return -1;

    // init startcol correctly
    regmatch.startpos[0].col = -1;
    // move to match
    if (move)
    {
	CLEAR_POS(&pos);
    }
    else
    {
	pos = *cur;
	// accept a match at the cursor position
	flag = SEARCH_START;
    }

    if (searchit(curwin, curbuf, &pos, NULL, direction, pattern, 1,
				  SEARCH_KEEP + flag, RE_SEARCH, NULL) != FAIL)
    {
	// Zero-width pattern should match somewhere, then we can check if
	// start and end are in the same position.
	do
	{
	    regmatch.startpos[0].col++;
	    nmatched = vim_regexec_multi(&regmatch, curwin, curbuf,
			       pos.lnum, regmatch.startpos[0].col, NULL);
	    if (nmatched != 0)
		break;
	} while (regmatch.regprog != NULL
		&& direction == FORWARD ? regmatch.startpos[0].col < pos.col
				      : regmatch.startpos[0].col > pos.col);

	if (called_emsg == called_emsg_before)
	{
	    result = (nmatched != 0
		&& regmatch.startpos[0].lnum == regmatch.endpos[0].lnum
		&& regmatch.startpos[0].col == regmatch.endpos[0].col);
	}
    }

    vim_regfree(regmatch.regprog);
    return result;
}


/*
 * Find next search match under cursor, cursor at end.
 * Used while an operator is pending, and in Visual mode.
 */
    int
current_search(
    long	count,
    int		forward)	// TRUE for forward, FALSE for backward
{
    pos_T	start_pos;	// start position of the pattern match
    pos_T	end_pos;	// end position of the pattern match
    pos_T	orig_pos;	// position of the cursor at beginning
    pos_T	pos;		// position after the pattern
    int		i;
    int		dir;
    int		result;		// result of various function calls
    char_u	old_p_ws = p_ws;
    int		flags = 0;
    pos_T	save_VIsual = VIsual;
    int		zero_width;
    int		skip_first_backward;

    // Correct cursor when 'selection' is exclusive
    if (VIsual_active && *p_sel == 'e' && LT_POS(VIsual, curwin->w_cursor))
	dec_cursor();

    // When searching forward and the cursor is at the start of the Visual
    // area, skip the first search backward, otherwise it doesn't move.
    skip_first_backward = forward && VIsual_active
					   && LT_POS(curwin->w_cursor, VIsual);

    orig_pos = pos = curwin->w_cursor;
    if (VIsual_active)
    {
	if (forward)
	    incl(&pos);
	else
	    decl(&pos);
    }

    // Is the pattern is zero-width?, this time, don't care about the direction
    zero_width = is_zero_width(spats[last_idx].pat, TRUE, &curwin->w_cursor,
								      FORWARD);
    if (zero_width == -1)
	return FAIL;  // pattern not found

    /*
     * The trick is to first search backwards and then search forward again,
     * so that a match at the current cursor position will be correctly
     * captured.  When "forward" is false do it the other way around.
     */
    for (i = 0; i < 2; i++)
    {
	if (forward)
	{
	    if (i == 0 && skip_first_backward)
		continue;
	    dir = i;
	}
	else
	    dir = !i;

	flags = 0;
	if (!dir && !zero_width)
	    flags = SEARCH_END;
	end_pos = pos;

	// wrapping should not occur in the first round
	if (i == 0)
	    p_ws = FALSE;

	result = searchit(curwin, curbuf, &pos, &end_pos,
		(dir ? FORWARD : BACKWARD),
		spats[last_idx].pat, (long) (i ? count : 1),
		SEARCH_KEEP | flags, RE_SEARCH, NULL);

	p_ws = old_p_ws;

	// First search may fail, but then start searching from the
	// beginning of the file (cursor might be on the search match)
	// except when Visual mode is active, so that extending the visual
	// selection works.
	if (i == 1 && !result) // not found, abort
	{
	    curwin->w_cursor = orig_pos;
	    if (VIsual_active)
		VIsual = save_VIsual;
	    return FAIL;
	}
	else if (i == 0 && !result)
	{
	    if (forward)
	    {
		// try again from start of buffer
		CLEAR_POS(&pos);
	    }
	    else
	    {
		// try again from end of buffer
		// searching backwards, so set pos to last line and col
		pos.lnum = curwin->w_buffer->b_ml.ml_line_count;
		pos.col  = (colnr_T)STRLEN(
				ml_get(curwin->w_buffer->b_ml.ml_line_count));
	    }
	}
    }

    start_pos = pos;

    if (!VIsual_active)
	VIsual = start_pos;

    // put the cursor after the match
    curwin->w_cursor = end_pos;
    if (LT_POS(VIsual, end_pos) && forward)
    {
	if (skip_first_backward)
	    // put the cursor on the start of the match
	    curwin->w_cursor = pos;
	else
	    // put the cursor on last character of match
	    dec_cursor();
    }
    else if (VIsual_active && LT_POS(curwin->w_cursor, VIsual) && forward)
	curwin->w_cursor = pos;   // put the cursor on the start of the match
    VIsual_active = TRUE;
    VIsual_mode = 'v';

    if (*p_sel == 'e')
    {
	// Correction for exclusive selection depends on the direction.
	if (forward && LTOREQ_POS(VIsual, curwin->w_cursor))
	    inc_cursor();
	else if (!forward && LTOREQ_POS(curwin->w_cursor, VIsual))
	    inc(&VIsual);
    }

#ifdef FEAT_FOLDING
    if (fdo_flags & FDO_SEARCH && KeyTyped)
	foldOpenCursor();
#endif

    may_start_select('c');
    setmouse();
#ifdef FEAT_CLIPBOARD
    // Make sure the clipboard gets updated.  Needed because start and
    // end are still the same, and the selection needs to be owned
    clip_star.vmode = NUL;
#endif
    redraw_curbuf_later(UPD_INVERTED);
    showmode();

    return OK;
}

/*
 * return TRUE if line 'lnum' is empty or has white chars only.
 */
    int
linewhite(linenr_T lnum)
{
    char_u  *p;

    p = skipwhite(ml_get(lnum));
    return (*p == NUL);
}

/*
 * Add the search count "[3/19]" to "msgbuf".
 * See update_search_stat() for other arguments.
 */
    static void
cmdline_search_stat(
    int		dirc,
    pos_T	*pos,
    pos_T	*cursor_pos,
    int		show_top_bot_msg,
    char_u	*msgbuf,
    int		recompute,
    int		maxcount,
    long	timeout)
{
    searchstat_T stat;

    update_search_stat(dirc, pos, cursor_pos, &stat, recompute, maxcount,
								      timeout);
    if (stat.cur <= 0)
	return;

    char	t[SEARCH_STAT_BUF_LEN];
    size_t	len;

#ifdef FEAT_RIGHTLEFT
    if (curwin->w_p_rl && *curwin->w_p_rlc == 's')
    {
	if (stat.incomplete == 1)
	    vim_snprintf(t, SEARCH_STAT_BUF_LEN, "[?/??]");
	else if (stat.cnt > maxcount && stat.cur > maxcount)
	    vim_snprintf(t, SEARCH_STAT_BUF_LEN, "[>%d/>%d]",
		    maxcount, maxcount);
	else if (stat.cnt > maxcount)
	    vim_snprintf(t, SEARCH_STAT_BUF_LEN, "[>%d/%d]",
		    maxcount, stat.cur);
	else
	    vim_snprintf(t, SEARCH_STAT_BUF_LEN, "[%d/%d]",
		    stat.cnt, stat.cur);
    }
    else
#endif
    {
	if (stat.incomplete == 1)
	    vim_snprintf(t, SEARCH_STAT_BUF_LEN, "[?/??]");
	else if (stat.cnt > maxcount && stat.cur > maxcount)
	    vim_snprintf(t, SEARCH_STAT_BUF_LEN, "[>%d/>%d]",
		    maxcount, maxcount);
	else if (stat.cnt > maxcount)
	    vim_snprintf(t, SEARCH_STAT_BUF_LEN, "[%d/>%d]",
		    stat.cur, maxcount);
	else
	    vim_snprintf(t, SEARCH_STAT_BUF_LEN, "[%d/%d]",
		    stat.cur, stat.cnt);
    }

    len = STRLEN(t);
    if (show_top_bot_msg && len + 2 < SEARCH_STAT_BUF_LEN)
    {
	mch_memmove(t + 2, t, len);
	t[0] = 'W';
	t[1] = ' ';
	len += 2;
    }

    size_t msgbuf_len = STRLEN(msgbuf);
    if (len > msgbuf_len)
	len = msgbuf_len;
    mch_memmove(msgbuf + msgbuf_len - len, t, len);

    if (dirc == '?' && stat.cur == maxcount + 1)
	stat.cur = -1;

    // keep the message even after redraw, but don't put in history
    msg_hist_off = TRUE;
    give_warning(msgbuf, FALSE);
    msg_hist_off = FALSE;
}

/*
 * Add the search count information to "stat".
 * "stat" must not be NULL.
 * When "recompute" is TRUE always recompute the numbers.
 * dirc == 0: don't find the next/previous match (only set the result to "stat")
 * dirc == '/': find the next match
 * dirc == '?': find the previous match
 */
    static void
update_search_stat(
    int			dirc,
    pos_T		*pos,
    pos_T		*cursor_pos,
    searchstat_T	*stat,
    int			recompute,
    int			maxcount,
    long		timeout UNUSED)
{
    int		    save_ws = p_ws;
    int		    wraparound = FALSE;
    pos_T	    p = (*pos);
    static pos_T    lastpos = {0, 0, 0};
    static int	    cur = 0;
    static int	    cnt = 0;
    static int	    exact_match = FALSE;
    static int	    incomplete = 0;
    static int	    last_maxcount = SEARCH_STAT_DEF_MAX_COUNT;
    static int	    chgtick = 0;
    static char_u   *lastpat = NULL;
    static buf_T    *lbuf = NULL;
#ifdef FEAT_RELTIME
    proftime_T  start;
#endif

    CLEAR_POINTER(stat);

    if (dirc == 0 && !recompute && !EMPTY_POS(lastpos))
    {
	stat->cur = cur;
	stat->cnt = cnt;
	stat->exact_match = exact_match;
	stat->incomplete = incomplete;
	stat->last_maxcount = last_maxcount;
	return;
    }
    last_maxcount = maxcount;

    wraparound = ((dirc == '?' && LT_POS(lastpos, p))
	       || (dirc == '/' && LT_POS(p, lastpos)));

    // If anything relevant changed the count has to be recomputed.
    // MB_STRNICMP ignores case, but we should not ignore case.
    // Unfortunately, there is no MB_STRNICMP function.
    // XXX: above comment should be "no MB_STRCMP function" ?
    if (!(chgtick == CHANGEDTICK(curbuf)
	&& MB_STRNICMP(lastpat, spats[last_idx].pat, STRLEN(lastpat)) == 0
	&& STRLEN(lastpat) == STRLEN(spats[last_idx].pat)
	&& EQUAL_POS(lastpos, *cursor_pos)
	&& lbuf == curbuf) || wraparound || cur < 0
	    || (maxcount > 0 && cur > maxcount) || recompute)
    {
	cur = 0;
	cnt = 0;
	exact_match = FALSE;
	incomplete = 0;
	CLEAR_POS(&lastpos);
	lbuf = curbuf;
    }

    // when searching backwards and having jumped to the first occurrence,
    // cur must remain greater than 1
    if (EQUAL_POS(lastpos, *cursor_pos) && !wraparound
		&& (dirc == 0 || dirc == '/' ? cur < cnt : cur > 1))
	cur += dirc == 0 ? 0 : dirc == '/' ? 1 : -1;
    else
    {
	int	done_search = FALSE;
	pos_T	endpos = {0, 0, 0};

	p_ws = FALSE;
#ifdef FEAT_RELTIME
	if (timeout > 0)
	    profile_setlimit(timeout, &start);
#endif
	while (!got_int && searchit(curwin, curbuf, &lastpos, &endpos,
			 FORWARD, NULL, 1, SEARCH_KEEP, RE_LAST, NULL) != FAIL)
	{
	    done_search = TRUE;
#ifdef FEAT_RELTIME
	    // Stop after passing the time limit.
	    if (timeout > 0 && profile_passed_limit(&start))
	    {
		incomplete = 1;
		break;
	    }
#endif
	    cnt++;
	    if (LTOREQ_POS(lastpos, p))
	    {
		cur = cnt;
		if (LT_POS(p, endpos))
		    exact_match = TRUE;
	    }
	    fast_breakcheck();
	    if (maxcount > 0 && cnt > maxcount)
	    {
		incomplete = 2;    // max count exceeded
		break;
	    }
	}
	if (got_int)
	    cur = -1; // abort
	if (done_search)
	{
	    vim_free(lastpat);
	    lastpat = vim_strsave(spats[last_idx].pat);
	    chgtick = CHANGEDTICK(curbuf);
	    lbuf = curbuf;
	    lastpos = p;
	}
    }
    stat->cur = cur;
    stat->cnt = cnt;
    stat->exact_match = exact_match;
    stat->incomplete = incomplete;
    stat->last_maxcount = last_maxcount;
    p_ws = save_ws;
}

#if defined(FEAT_FIND_ID) || defined(PROTO)

/*
 * Get line "lnum" and copy it into "buf[LSIZE]".
 * The copy is made because the regexp may make the line invalid when using a
 * mark.
 */
    static char_u *
get_line_and_copy(linenr_T lnum, char_u *buf)
{
    char_u *line = ml_get(lnum);

    vim_strncpy(buf, line, LSIZE - 1);
    return buf;
}

/*
 * Find identifiers or defines in included files.
 * If p_ic && compl_status_sol() then ptr must be in lowercase.
 */
    void
find_pattern_in_path(
    char_u	*ptr,		// pointer to search pattern
    int		dir UNUSED,	// direction of expansion
    int		len,		// length of search pattern
    int		whole,		// match whole words only
    int		skip_comments,	// don't match inside comments
    int		type,		// Type of search; are we looking for a type?
				// a macro?
    long	count,
    int		action,		// What to do when we find it
    linenr_T	start_lnum,	// first line to start searching
    linenr_T	end_lnum)	// last line for searching
{
    SearchedFile *files;		// Stack of included files
    SearchedFile *bigger;		// When we need more space
    int		max_path_depth = 50;
    long	match_count = 1;

    char_u	*pat;
    char_u	*new_fname;
    char_u	*curr_fname = curbuf->b_fname;
    char_u	*prev_fname = NULL;
    linenr_T	lnum;
    int		depth;
    int		depth_displayed;	// For type==CHECK_PATH
    int		old_files;
    int		already_searched;
    char_u	*file_line;
    char_u	*line;
    char_u	*p;
    char_u	save_char;
    int		define_matched;
    regmatch_T	regmatch;
    regmatch_T	incl_regmatch;
    regmatch_T	def_regmatch;
    int		matched = FALSE;
    int		did_show = FALSE;
    int		found = FALSE;
    int		i;
    char_u	*already = NULL;
    char_u	*startp = NULL;
    char_u	*inc_opt = NULL;
#if defined(FEAT_QUICKFIX)
    win_T	*curwin_save = NULL;
#endif

    regmatch.regprog = NULL;
    incl_regmatch.regprog = NULL;
    def_regmatch.regprog = NULL;

    file_line = alloc(LSIZE);
    if (file_line == NULL)
	return;

    if (type != CHECK_PATH && type != FIND_DEFINE
	    // when CONT_SOL is set compare "ptr" with the beginning of the
	    // line is faster than quote_meta/regcomp/regexec "ptr" -- Acevedo
	    && !compl_status_sol())
    {
	pat = alloc(len + 5);
	if (pat == NULL)
	    goto fpip_end;
	sprintf((char *)pat, whole ? "\\<%.*s\\>" : "%.*s", len, ptr);
	// ignore case according to p_ic, p_scs and pat
	regmatch.rm_ic = ignorecase(pat);
	regmatch.regprog = vim_regcomp(pat, magic_isset() ? RE_MAGIC : 0);
	vim_free(pat);
	if (regmatch.regprog == NULL)
	    goto fpip_end;
    }
    inc_opt = (*curbuf->b_p_inc == NUL) ? p_inc : curbuf->b_p_inc;
    if (*inc_opt != NUL)
    {
	incl_regmatch.regprog = vim_regcomp(inc_opt,
						 magic_isset() ? RE_MAGIC : 0);
	if (incl_regmatch.regprog == NULL)
	    goto fpip_end;
	incl_regmatch.rm_ic = FALSE;	// don't ignore case in incl. pat.
    }
    if (type == FIND_DEFINE && (*curbuf->b_p_def != NUL || *p_def != NUL))
    {
	def_regmatch.regprog = vim_regcomp(*curbuf->b_p_def == NUL
			   ? p_def : curbuf->b_p_def,
						 magic_isset() ? RE_MAGIC : 0);
	if (def_regmatch.regprog == NULL)
	    goto fpip_end;
	def_regmatch.rm_ic = FALSE;	// don't ignore case in define pat.
    }
    files = lalloc_clear(max_path_depth * sizeof(SearchedFile), TRUE);
    if (files == NULL)
	goto fpip_end;
    old_files = max_path_depth;
    depth = depth_displayed = -1;

    lnum = start_lnum;
    if (end_lnum > curbuf->b_ml.ml_line_count)
	end_lnum = curbuf->b_ml.ml_line_count;
    if (lnum > end_lnum)		// do at least one line
	lnum = end_lnum;
    line = get_line_and_copy(lnum, file_line);

    for (;;)
    {
	if (incl_regmatch.regprog != NULL
		&& vim_regexec(&incl_regmatch, line, (colnr_T)0))
	{
	    char_u *p_fname = (curr_fname == curbuf->b_fname)
					      ? curbuf->b_ffname : curr_fname;

	    if (inc_opt != NULL && strstr((char *)inc_opt, "\\zs") != NULL)
		// Use text from '\zs' to '\ze' (or end) of 'include'.
		new_fname = find_file_name_in_path(incl_regmatch.startp[0],
		       (int)(incl_regmatch.endp[0] - incl_regmatch.startp[0]),
				 FNAME_EXP|FNAME_INCL|FNAME_REL, 1L, p_fname);
	    else
		// Use text after match with 'include'.
		new_fname = file_name_in_line(incl_regmatch.endp[0], 0,
			     FNAME_EXP|FNAME_INCL|FNAME_REL, 1L, p_fname, NULL);
	    already_searched = FALSE;
	    if (new_fname != NULL)
	    {
		// Check whether we have already searched in this file
		for (i = 0;; i++)
		{
		    if (i == depth + 1)
			i = old_files;
		    if (i == max_path_depth)
			break;
		    if (fullpathcmp(new_fname, files[i].name, TRUE, TRUE)
								    & FPC_SAME)
		    {
			if (type != CHECK_PATH
				&& action == ACTION_SHOW_ALL
				&& files[i].matched)
			{
			    msg_putchar('\n');	    // cursor below last one
			    if (!got_int)	    // don't display if 'q'
						    // typed at "--more--"
						    // message
			    {
				msg_home_replace_hl(new_fname);
				msg_puts(_(" (includes previously listed match)"));
				prev_fname = NULL;
			    }
			}
			VIM_CLEAR(new_fname);
			already_searched = TRUE;
			break;
		    }
		}
	    }

	    if (type == CHECK_PATH && (action == ACTION_SHOW_ALL
				 || (new_fname == NULL && !already_searched)))
	    {
		if (did_show)
		    msg_putchar('\n');	    // cursor below last one
		else
		{
		    gotocmdline(TRUE);	    // cursor at status line
		    msg_puts_title(_("--- Included files "));
		    if (action != ACTION_SHOW_ALL)
			msg_puts_title(_("not found "));
		    msg_puts_title(_("in path ---\n"));
		}
		did_show = TRUE;
		while (depth_displayed < depth && !got_int)
		{
		    ++depth_displayed;
		    for (i = 0; i < depth_displayed; i++)
			msg_puts("  ");
		    msg_home_replace(files[depth_displayed].name);
		    msg_puts(" -->\n");
		}
		if (!got_int)		    // don't display if 'q' typed
					    // for "--more--" message
		{
		    for (i = 0; i <= depth_displayed; i++)
			msg_puts("  ");
		    if (new_fname != NULL)
		    {
			// using "new_fname" is more reliable, e.g., when
			// 'includeexpr' is set.
			msg_outtrans_attr(new_fname, HL_ATTR(HLF_D));
		    }
		    else
		    {
			/*
			 * Isolate the file name.
			 * Include the surrounding "" or <> if present.
			 */
			if (inc_opt != NULL
				   && strstr((char *)inc_opt, "\\zs") != NULL)
			{
			    // pattern contains \zs, use the match
			    p = incl_regmatch.startp[0];
			    i = (int)(incl_regmatch.endp[0]
						   - incl_regmatch.startp[0]);
			}
			else
			{
			    // find the file name after the end of the match
			    for (p = incl_regmatch.endp[0];
						  *p && !vim_isfilec(*p); p++)
				;
			    for (i = 0; vim_isfilec(p[i]); i++)
				;
			}

			if (i == 0)
			{
			    // Nothing found, use the rest of the line.
			    p = incl_regmatch.endp[0];
			    i = (int)STRLEN(p);
			}
			// Avoid checking before the start of the line, can
			// happen if \zs appears in the regexp.
			else if (p > line)
			{
			    if (p[-1] == '"' || p[-1] == '<')
			    {
				--p;
				++i;
			    }
			    if (p[i] == '"' || p[i] == '>')
				++i;
			}
			save_char = p[i];
			p[i] = NUL;
			msg_outtrans_attr(p, HL_ATTR(HLF_D));
			p[i] = save_char;
		    }

		    if (new_fname == NULL && action == ACTION_SHOW_ALL)
		    {
			if (already_searched)
			    msg_puts(_("  (Already listed)"));
			else
			    msg_puts(_("  NOT FOUND"));
		    }
		}
		out_flush();	    // output each line directly
	    }

	    if (new_fname != NULL)
	    {
		// Push the new file onto the file stack
		if (depth + 1 == old_files)
		{
		    bigger = ALLOC_MULT(SearchedFile, max_path_depth * 2);
		    if (bigger != NULL)
		    {
			for (i = 0; i <= depth; i++)
			    bigger[i] = files[i];
			for (i = depth + 1; i < old_files + max_path_depth; i++)
			{
			    bigger[i].fp = NULL;
			    bigger[i].name = NULL;
			    bigger[i].lnum = 0;
			    bigger[i].matched = FALSE;
			}
			for (i = old_files; i < max_path_depth; i++)
			    bigger[i + max_path_depth] = files[i];
			old_files += max_path_depth;
			max_path_depth *= 2;
			vim_free(files);
			files = bigger;
		    }
		}
		if ((files[depth + 1].fp = mch_fopen((char *)new_fname, "r"))
								    == NULL)
		    vim_free(new_fname);
		else
		{
		    if (++depth == old_files)
		    {
			/*
			 * lalloc() for 'bigger' must have failed above.  We
			 * will forget one of our already visited files now.
			 */
			vim_free(files[old_files].name);
			++old_files;
		    }
		    files[depth].name = curr_fname = new_fname;
		    files[depth].lnum = 0;
		    files[depth].matched = FALSE;
		    if (action == ACTION_EXPAND)
		    {
			msg_hist_off = TRUE;	// reset in msg_trunc_attr()
			vim_snprintf((char*)IObuff, IOSIZE,
				_("Scanning included file: %s"),
				(char *)new_fname);
			msg_trunc_attr((char *)IObuff, TRUE, HL_ATTR(HLF_R));
		    }
		    else if (p_verbose >= 5)
		    {
			verbose_enter();
			smsg(_("Searching included file %s"),
							   (char *)new_fname);
			verbose_leave();
		    }

		}
	    }
	}
	else
	{
	    /*
	     * Check if the line is a define (type == FIND_DEFINE)
	     */
	    p = line;
search_line:
	    define_matched = FALSE;
	    if (def_regmatch.regprog != NULL
			      && vim_regexec(&def_regmatch, line, (colnr_T)0))
	    {
		/*
		 * Pattern must be first identifier after 'define', so skip
		 * to that position before checking for match of pattern.  Also
		 * don't let it match beyond the end of this identifier.
		 */
		p = def_regmatch.endp[0];
		while (*p && !vim_iswordc(*p))
		    p++;
		define_matched = TRUE;
	    }

	    /*
	     * Look for a match.  Don't do this if we are looking for a
	     * define and this line didn't match define_prog above.
	     */
	    if (def_regmatch.regprog == NULL || define_matched)
	    {
		if (define_matched || compl_status_sol())
		{
		    // compare the first "len" chars from "ptr"
		    startp = skipwhite(p);
		    if (p_ic)
			matched = !MB_STRNICMP(startp, ptr, len);
		    else
			matched = !STRNCMP(startp, ptr, len);
		    if (matched && define_matched && whole
						  && vim_iswordc(startp[len]))
			matched = FALSE;
		}
		else if (regmatch.regprog != NULL
			 && vim_regexec(&regmatch, line, (colnr_T)(p - line)))
		{
		    matched = TRUE;
		    startp = regmatch.startp[0];
		    /*
		     * Check if the line is not a comment line (unless we are
		     * looking for a define).  A line starting with "# define"
		     * is not considered to be a comment line.
		     */
		    if (!define_matched && skip_comments)
		    {
			if ((*line != '#' ||
				STRNCMP(skipwhite(line + 1), "define", 6) != 0)
				&& get_leader_len(line, NULL, FALSE, TRUE))
			    matched = FALSE;

			/*
			 * Also check for a "/ *" or "/ /" before the match.
			 * Skips lines like "int backwards;  / * normal index
			 * * /" when looking for "normal".
			 * Note: Doesn't skip "/ *" in comments.
			 */
			p = skipwhite(line);
			if (matched
				|| (p[0] == '/' && p[1] == '*') || p[0] == '*')
			    for (p = line; *p && p < startp; ++p)
			    {
				if (matched
					&& p[0] == '/'
					&& (p[1] == '*' || p[1] == '/'))
				{
				    matched = FALSE;
				    // After "//" all text is comment
				    if (p[1] == '/')
					break;
				    ++p;
				}
				else if (!matched && p[0] == '*' && p[1] == '/')
				{
				    // Can find match after "* /".
				    matched = TRUE;
				    ++p;
				}
			    }
		    }
		}
	    }
	}
	if (matched)
	{
	    if (action == ACTION_EXPAND)
	    {
		int	cont_s_ipos = FALSE;
		int	add_r;
		char_u	*aux;

		if (depth == -1 && lnum == curwin->w_cursor.lnum)
		    break;
		found = TRUE;
		aux = p = startp;
		if (compl_status_adding())
		{
		    p += ins_compl_len();
		    if (vim_iswordp(p))
			goto exit_matched;
		    p = find_word_start(p);
		}
		p = find_word_end(p);
		i = (int)(p - aux);

		if (compl_status_adding() && i == ins_compl_len())
		{
		    // IOSIZE > compl_length, so the STRNCPY works
		    STRNCPY(IObuff, aux, i);

		    // Get the next line: when "depth" < 0  from the current
		    // buffer, otherwise from the included file.  Jump to
		    // exit_matched when past the last line.
		    if (depth < 0)
		    {
			if (lnum >= end_lnum)
			    goto exit_matched;
			line = get_line_and_copy(++lnum, file_line);
		    }
		    else if (vim_fgets(line = file_line,
						      LSIZE, files[depth].fp))
			goto exit_matched;

		    // we read a line, set "already" to check this "line" later
		    // if depth >= 0 we'll increase files[depth].lnum far
		    // below  -- Acevedo
		    already = aux = p = skipwhite(line);
		    p = find_word_start(p);
		    p = find_word_end(p);
		    if (p > aux)
		    {
			if (*aux != ')' && IObuff[i-1] != TAB)
			{
			    if (IObuff[i-1] != ' ')
				IObuff[i++] = ' ';
			    // IObuf =~ "\(\k\|\i\).* ", thus i >= 2
			    if (p_js
				&& (IObuff[i-2] == '.'
				    || (vim_strchr(p_cpo, CPO_JOINSP) == NULL
					&& (IObuff[i-2] == '?'
					    || IObuff[i-2] == '!'))))
				IObuff[i++] = ' ';
			}
			// copy as much as possible of the new word
			if (p - aux >= IOSIZE - i)
			    p = aux + IOSIZE - i - 1;
			STRNCPY(IObuff + i, aux, p - aux);
			i += (int)(p - aux);
			cont_s_ipos = TRUE;
		    }
		    IObuff[i] = NUL;
		    aux = IObuff;

		    if (i == ins_compl_len())
			goto exit_matched;
		}

		add_r = ins_compl_add_infercase(aux, i, p_ic,
			curr_fname == curbuf->b_fname ? NULL : curr_fname,
			dir, cont_s_ipos);
		if (add_r == OK)
		    // if dir was BACKWARD then honor it just once
		    dir = FORWARD;
		else if (add_r == FAIL)
		    break;
	    }
	    else if (action == ACTION_SHOW_ALL)
	    {
		found = TRUE;
		if (!did_show)
		    gotocmdline(TRUE);		// cursor at status line
		if (curr_fname != prev_fname)
		{
		    if (did_show)
			msg_putchar('\n');	// cursor below last one
		    if (!got_int)		// don't display if 'q' typed
						// at "--more--" message
			msg_home_replace_hl(curr_fname);
		    prev_fname = curr_fname;
		}
		did_show = TRUE;
		if (!got_int)
		    show_pat_in_path(line, type, TRUE, action,
			    (depth == -1) ? NULL : files[depth].fp,
			    (depth == -1) ? &lnum : &files[depth].lnum,
			    match_count++);

		// Set matched flag for this file and all the ones that
		// include it
		for (i = 0; i <= depth; ++i)
		    files[i].matched = TRUE;
	    }
	    else if (--count <= 0)
	    {
		found = TRUE;
		if (depth == -1 && lnum == curwin->w_cursor.lnum
#if defined(FEAT_QUICKFIX)
						      && g_do_tagpreview == 0
#endif
						      )
		    emsg(_(e_match_is_on_current_line));
		else if (action == ACTION_SHOW)
		{
		    show_pat_in_path(line, type, did_show, action,
			(depth == -1) ? NULL : files[depth].fp,
			(depth == -1) ? &lnum : &files[depth].lnum, 1L);
		    did_show = TRUE;
		}
		else
		{
#ifdef FEAT_GUI
		    need_mouse_correct = TRUE;
#endif
#if defined(FEAT_QUICKFIX)
		    // ":psearch" uses the preview window
		    if (g_do_tagpreview != 0)
		    {
			curwin_save = curwin;
			prepare_tagpreview(TRUE, TRUE, FALSE);
		    }
#endif
		    if (action == ACTION_SPLIT)
		    {
			if (win_split(0, 0) == FAIL)
			    break;
			RESET_BINDING(curwin);
		    }
		    if (depth == -1)
		    {
			// match in current file
#if defined(FEAT_QUICKFIX)
			if (g_do_tagpreview != 0)
			{
			    if (!win_valid(curwin_save))
				break;
			    if (!GETFILE_SUCCESS(getfile(
					   curwin_save->w_buffer->b_fnum, NULL,
						     NULL, TRUE, lnum, FALSE)))
				break;	// failed to jump to file
			}
			else
#endif
			    setpcmark();
			curwin->w_cursor.lnum = lnum;
			check_cursor();
		    }
		    else
		    {
			if (!GETFILE_SUCCESS(getfile(
					0, files[depth].name, NULL, TRUE,
						    files[depth].lnum, FALSE)))
			    break;	// failed to jump to file
			// autocommands may have changed the lnum, we don't
			// want that here
			curwin->w_cursor.lnum = files[depth].lnum;
		    }
		}
		if (action != ACTION_SHOW)
		{
		    curwin->w_cursor.col = (colnr_T)(startp - line);
		    curwin->w_set_curswant = TRUE;
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
# ifdef FEAT_PROP_POPUP
		else if (WIN_IS_POPUP(curwin))
		    // can't keep focus in popup window
		    win_enter(firstwin, TRUE);
# endif
#endif
		break;
	    }
exit_matched:
	    matched = FALSE;
	    // look for other matches in the rest of the line if we
	    // are not at the end of it already
	    if (def_regmatch.regprog == NULL
		    && action == ACTION_EXPAND
		    && !compl_status_sol()
		    && *startp != NUL
		    && *(p = startp + mb_ptr2len(startp)) != NUL)
		goto search_line;
	}
	line_breakcheck();
	if (action == ACTION_EXPAND)
	    ins_compl_check_keys(30, FALSE);
	if (got_int || ins_compl_interrupted())
	    break;

	/*
	 * Read the next line.  When reading an included file and encountering
	 * end-of-file, close the file and continue in the file that included
	 * it.
	 */
	while (depth >= 0 && !already
		&& vim_fgets(line = file_line, LSIZE, files[depth].fp))
	{
	    fclose(files[depth].fp);
	    --old_files;
	    files[old_files].name = files[depth].name;
	    files[old_files].matched = files[depth].matched;
	    --depth;
	    curr_fname = (depth == -1) ? curbuf->b_fname
				       : files[depth].name;
	    if (depth < depth_displayed)
		depth_displayed = depth;
	}
	if (depth >= 0)		// we could read the line
	{
	    files[depth].lnum++;
	    // Remove any CR and LF from the line.
	    i = (int)STRLEN(line);
	    if (i > 0 && line[i - 1] == '\n')
		line[--i] = NUL;
	    if (i > 0 && line[i - 1] == '\r')
		line[--i] = NUL;
	}
	else if (!already)
	{
	    if (++lnum > end_lnum)
		break;
	    line = get_line_and_copy(lnum, file_line);
	}
	already = NULL;
    }
    // End of big for (;;) loop.

    // Close any files that are still open.
    for (i = 0; i <= depth; i++)
    {
	fclose(files[i].fp);
	vim_free(files[i].name);
    }
    for (i = old_files; i < max_path_depth; i++)
	vim_free(files[i].name);
    vim_free(files);

    if (type == CHECK_PATH)
    {
	if (!did_show)
	{
	    if (action != ACTION_SHOW_ALL)
		msg(_("All included files were found"));
	    else
		msg(_("No included files"));
	}
    }
    else if (!found && action != ACTION_EXPAND)
    {
	if (got_int || ins_compl_interrupted())
	    emsg(_(e_interrupted));
	else if (type == FIND_DEFINE)
	    emsg(_(e_couldnt_find_definition));
	else
	    emsg(_(e_couldnt_find_pattern));
    }
    if (action == ACTION_SHOW || action == ACTION_SHOW_ALL)
	msg_end();

fpip_end:
    vim_free(file_line);
    vim_regfree(regmatch.regprog);
    vim_regfree(incl_regmatch.regprog);
    vim_regfree(def_regmatch.regprog);
}

    static void
show_pat_in_path(
    char_u  *line,
    int	    type,
    int	    did_show,
    int	    action,
    FILE    *fp,
    linenr_T *lnum,
    long    count)
{
    char_u  *p;

    if (did_show)
	msg_putchar('\n');	// cursor below last one
    else if (!msg_silent)
	gotocmdline(TRUE);	// cursor at status line
    if (got_int)		// 'q' typed at "--more--" message
	return;
    for (;;)
    {
	p = line + STRLEN(line) - 1;
	if (fp != NULL)
	{
	    // We used fgets(), so get rid of newline at end
	    if (p >= line && *p == '\n')
		--p;
	    if (p >= line && *p == '\r')
		--p;
	    *(p + 1) = NUL;
	}
	if (action == ACTION_SHOW_ALL)
	{
	    sprintf((char *)IObuff, "%3ld: ", count);	// show match nr
	    msg_puts((char *)IObuff);
	    sprintf((char *)IObuff, "%4ld", *lnum);	// show line nr
						// Highlight line numbers
	    msg_puts_attr((char *)IObuff, HL_ATTR(HLF_N));
	    msg_puts(" ");
	}
	msg_prt_line(line, FALSE);
	out_flush();			// show one line at a time

	// Definition continues until line that doesn't end with '\'
	if (got_int || type != FIND_DEFINE || p < line || *p != '\\')
	    break;

	if (fp != NULL)
	{
	    if (vim_fgets(line, LSIZE, fp)) // end of file
		break;
	    ++*lnum;
	}
	else
	{
	    if (++*lnum > curbuf->b_ml.ml_line_count)
		break;
	    line = ml_get(*lnum);
	}
	msg_putchar('\n');
    }
}
#endif

#ifdef FEAT_VIMINFO
/*
 * Return the last used search pattern at "idx".
 */
    spat_T *
get_spat(int idx)
{
    return &spats[idx];
}

/*
 * Return the last used search pattern index.
 */
    int
get_spat_last_idx(void)
{
    return last_idx;
}
#endif

#if defined(FEAT_EVAL) || defined(FEAT_PROTO)
/*
 * "searchcount()" function
 */
    void
f_searchcount(typval_T *argvars, typval_T *rettv)
{
    pos_T		pos = curwin->w_cursor;
    char_u		*pattern = NULL;
    int			maxcount = SEARCH_STAT_DEF_MAX_COUNT;
    long		timeout = SEARCH_STAT_DEF_TIMEOUT;
    int			recompute = TRUE;
    searchstat_T	stat;

    if (rettv_dict_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_dict_arg(argvars, 0) == FAIL)
	return;

    if (shortmess(SHM_SEARCHCOUNT))	// 'shortmess' contains 'S' flag
	recompute = TRUE;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	dict_T		*dict;
	dictitem_T	*di;
	listitem_T	*li;
	int		error = FALSE;

	if (check_for_nonnull_dict_arg(argvars, 0) == FAIL)
	    return;
	dict = argvars[0].vval.v_dict;
	di = dict_find(dict, (char_u *)"timeout", -1);
	if (di != NULL)
	{
	    timeout = (long)tv_get_number_chk(&di->di_tv, &error);
	    if (error)
		return;
	}
	di = dict_find(dict, (char_u *)"maxcount", -1);
	if (di != NULL)
	{
	    maxcount = (int)tv_get_number_chk(&di->di_tv, &error);
	    if (error)
		return;
	}
	recompute = dict_get_bool(dict, "recompute", recompute);
	di = dict_find(dict, (char_u *)"pattern", -1);
	if (di != NULL)
	{
	    pattern = tv_get_string_chk(&di->di_tv);
	    if (pattern == NULL)
		return;
	}
	di = dict_find(dict, (char_u *)"pos", -1);
	if (di != NULL)
	{
	    if (di->di_tv.v_type != VAR_LIST)
	    {
		semsg(_(e_invalid_argument_str), "pos");
		return;
	    }
	    if (list_len(di->di_tv.vval.v_list) != 3)
	    {
		semsg(_(e_invalid_argument_str), "List format should be [lnum, col, off]");
		return;
	    }
	    li = list_find(di->di_tv.vval.v_list, 0L);
	    if (li != NULL)
	    {
		pos.lnum = tv_get_number_chk(&li->li_tv, &error);
		if (error)
		    return;
	    }
	    li = list_find(di->di_tv.vval.v_list, 1L);
	    if (li != NULL)
	    {
		pos.col = tv_get_number_chk(&li->li_tv, &error) - 1;
		if (error)
		    return;
	    }
	    li = list_find(di->di_tv.vval.v_list, 2L);
	    if (li != NULL)
	    {
		pos.coladd = tv_get_number_chk(&li->li_tv, &error);
		if (error)
		    return;
	    }
	}
    }

    save_last_search_pattern();
#ifdef FEAT_SEARCH_EXTRA
    save_incsearch_state();
#endif
    if (pattern != NULL)
    {
	if (*pattern == NUL)
	    goto the_end;
	vim_free(spats[last_idx].pat);
	spats[last_idx].pat = vim_strsave(pattern);
    }
    if (spats[last_idx].pat == NULL || *spats[last_idx].pat == NUL)
	goto the_end;	// the previous pattern was never defined

    update_search_stat(0, &pos, &pos, &stat, recompute, maxcount, timeout);

    dict_add_number(rettv->vval.v_dict, "current", stat.cur);
    dict_add_number(rettv->vval.v_dict, "total", stat.cnt);
    dict_add_number(rettv->vval.v_dict, "exact_match", stat.exact_match);
    dict_add_number(rettv->vval.v_dict, "incomplete", stat.incomplete);
    dict_add_number(rettv->vval.v_dict, "maxcount", stat.last_maxcount);

the_end:
    restore_last_search_pattern();
#ifdef FEAT_SEARCH_EXTRA
    restore_incsearch_state();
#endif
}
#endif

/*
 * Fuzzy string matching
 *
 * Ported from the lib_fts library authored by Forrest Smith.
 * https://github.com/forrestthewoods/lib_fts/tree/master/code
 *
 * The following blog describes the fuzzy matching algorithm:
 * https://www.forrestthewoods.com/blog/reverse_engineering_sublime_texts_fuzzy_match/
 *
 * Each matching string is assigned a score. The following factors are checked:
 *   - Matched letter
 *   - Unmatched letter
 *   - Consecutively matched letters
 *   - Proximity to start
 *   - Letter following a separator (space, underscore)
 *   - Uppercase letter following lowercase (aka CamelCase)
 *
 * Matched letters are good. Unmatched letters are bad. Matching near the start
 * is good. Matching the first letter in the middle of a phrase is good.
 * Matching the uppercase letters in camel case entries is good.
 *
 * The score assigned for each factor is explained below.
 * File paths are different from file names. File extensions may be ignorable.
 * Single words care about consecutive matches but not separators or camel
 * case.
 *   Score starts at 100
 *   Matched letter: +0 points
 *   Unmatched letter: -1 point
 *   Consecutive match bonus: +15 points
 *   First letter bonus: +15 points
 *   Separator bonus: +30 points
 *   Camel case bonus: +30 points
 *   Unmatched leading letter: -5 points (max: -15)
 *
 * There is some nuance to this. Scores don’t have an intrinsic meaning. The
 * score range isn’t 0 to 100. It’s roughly [50, 150]. Longer words have a
 * lower minimum score due to unmatched letter penalty. Longer search patterns
 * have a higher maximum score due to match bonuses.
 *
 * Separator and camel case bonus is worth a LOT. Consecutive matches are worth
 * quite a bit.
 *
 * There is a penalty if you DON’T match the first three letters. Which
 * effectively rewards matching near the start. However there’s no difference
 * in matching between the middle and end.
 *
 * There is not an explicit bonus for an exact match. Unmatched letters receive
 * a penalty. So shorter strings and closer matches are worth more.
 */
typedef struct
{
    int		idx;		// used for stable sort
    listitem_T	*item;
    int		score;
    list_T	*lmatchpos;
} fuzzyItem_T;

// bonus for adjacent matches; this is higher than SEPARATOR_BONUS so that
// matching a whole word is preferred.
#define SEQUENTIAL_BONUS 40
// bonus if match occurs after a path separator
#define PATH_SEPARATOR_BONUS 30
// bonus if match occurs after a word separator
#define WORD_SEPARATOR_BONUS 25
// bonus if match is uppercase and prev is lower
#define CAMEL_BONUS 30
// bonus if the first letter is matched
#define FIRST_LETTER_BONUS 15
// penalty applied for every letter in str before the first match
#define LEADING_LETTER_PENALTY (-5)
// maximum penalty for leading letters
#define MAX_LEADING_LETTER_PENALTY (-15)
// penalty for every letter that doesn't match
#define UNMATCHED_LETTER_PENALTY (-1)
// penalty for gap in matching positions (-2 * k)
#define GAP_PENALTY	(-2)
// Score for a string that doesn't fuzzy match the pattern
#define SCORE_NONE	(-9999)

#define FUZZY_MATCH_RECURSION_LIMIT	10

/*
 * Compute a score for a fuzzy matched string. The matching character locations
 * are in 'matches'.
 */
    static int
fuzzy_match_compute_score(
	char_u		*str,
	int		strSz,
	int_u		*matches,
	int		numMatches)
{
    int		score;
    int		penalty;
    int		unmatched;
    int		i;
    char_u	*p = str;
    int_u	sidx = 0;

    // Initialize score
    score = 100;

    // Apply leading letter penalty
    penalty = LEADING_LETTER_PENALTY * matches[0];
    if (penalty < MAX_LEADING_LETTER_PENALTY)
	penalty = MAX_LEADING_LETTER_PENALTY;
    score += penalty;

    // Apply unmatched penalty
    unmatched = strSz - numMatches;
    score += UNMATCHED_LETTER_PENALTY * unmatched;

    // Apply ordering bonuses
    for (i = 0; i < numMatches; ++i)
    {
	int_u	currIdx = matches[i];

	if (i > 0)
	{
	    int_u	prevIdx = matches[i - 1];

	    // Sequential
	    if (currIdx == (prevIdx + 1))
		score += SEQUENTIAL_BONUS;
	    else
		score += GAP_PENALTY * (currIdx - prevIdx);
	}

	// Check for bonuses based on neighbor character value
	if (currIdx > 0)
	{
	    // Camel case
	    int	neighbor = ' ';
	    int	curr;

	    if (has_mbyte)
	    {
		while (sidx < currIdx)
		{
		    neighbor = (*mb_ptr2char)(p);
		    MB_PTR_ADV(p);
		    sidx++;
		}
		curr = (*mb_ptr2char)(p);
	    }
	    else
	    {
		neighbor = str[currIdx - 1];
		curr = str[currIdx];
	    }

	    if (vim_islower(neighbor) && vim_isupper(curr))
		score += CAMEL_BONUS;

	    // Bonus if the match follows a separator character
	    if (neighbor == '/' || neighbor == '\\')
		score += PATH_SEPARATOR_BONUS;
	    else if (neighbor == ' ' || neighbor == '_')
		score += WORD_SEPARATOR_BONUS;
	}
	else
	{
	    // First letter
	    score += FIRST_LETTER_BONUS;
	}
    }
    return score;
}

/*
 * Perform a recursive search for fuzzy matching 'fuzpat' in 'str'.
 * Return the number of matching characters.
 */
    static int
fuzzy_match_recursive(
	char_u		*fuzpat,
	char_u		*str,
	int_u		strIdx,
	int		*outScore,
	char_u		*strBegin,
	int		strLen,
	int_u		*srcMatches,
	int_u		*matches,
	int		maxMatches,
	int		nextMatch,
	int		*recursionCount)
{
    // Recursion params
    int		recursiveMatch = FALSE;
    int_u	bestRecursiveMatches[MAX_FUZZY_MATCHES];
    int		bestRecursiveScore = 0;
    int		first_match;
    int		matched;

    // Count recursions
    ++*recursionCount;
    if (*recursionCount >= FUZZY_MATCH_RECURSION_LIMIT)
	return 0;

    // Detect end of strings
    if (*fuzpat == NUL || *str == NUL)
	return 0;

    // Loop through fuzpat and str looking for a match
    first_match = TRUE;
    while (*fuzpat != NUL && *str != NUL)
    {
	int	c1;
	int	c2;

	c1 = PTR2CHAR(fuzpat);
	c2 = PTR2CHAR(str);

	// Found match
	if (vim_tolower(c1) == vim_tolower(c2))
	{
	    // Supplied matches buffer was too short
	    if (nextMatch >= maxMatches)
		return 0;

	    int		recursiveScore = 0;
	    int_u	recursiveMatches[MAX_FUZZY_MATCHES];
	    CLEAR_FIELD(recursiveMatches);

	    // "Copy-on-Write" srcMatches into matches
	    if (first_match && srcMatches)
	    {
		memcpy(matches, srcMatches, nextMatch * sizeof(srcMatches[0]));
		first_match = FALSE;
	    }

	    // Recursive call that "skips" this match
	    char_u *next_char = str + (has_mbyte ? (*mb_ptr2len)(str) : 1);
	    if (fuzzy_match_recursive(fuzpat, next_char, strIdx + 1,
			&recursiveScore, strBegin, strLen, matches,
			recursiveMatches,
			ARRAY_LENGTH(recursiveMatches),
			nextMatch, recursionCount))
	    {
		// Pick best recursive score
		if (!recursiveMatch || recursiveScore > bestRecursiveScore)
		{
		    memcpy(bestRecursiveMatches, recursiveMatches,
			    MAX_FUZZY_MATCHES * sizeof(recursiveMatches[0]));
		    bestRecursiveScore = recursiveScore;
		}
		recursiveMatch = TRUE;
	    }

	    // Advance
	    matches[nextMatch++] = strIdx;
	    if (has_mbyte)
		MB_PTR_ADV(fuzpat);
	    else
		++fuzpat;
	}
	if (has_mbyte)
	    MB_PTR_ADV(str);
	else
	    ++str;
	strIdx++;
    }

    // Determine if full fuzpat was matched
    matched = *fuzpat == NUL ? TRUE : FALSE;

    // Calculate score
    if (matched)
	*outScore = fuzzy_match_compute_score(strBegin, strLen, matches,
		nextMatch);

    // Return best result
    if (recursiveMatch && (!matched || bestRecursiveScore > *outScore))
    {
	// Recursive score is better than "this"
	memcpy(matches, bestRecursiveMatches, maxMatches * sizeof(matches[0]));
	*outScore = bestRecursiveScore;
	return nextMatch;
    }
    else if (matched)
	return nextMatch;	// "this" score is better than recursive

    return 0;		// no match
}

/*
 * fuzzy_match()
 *
 * Performs exhaustive search via recursion to find all possible matches and
 * match with highest score.
 * Scores values have no intrinsic meaning.  Possible score range is not
 * normalized and varies with pattern.
 * Recursion is limited internally (default=10) to prevent degenerate cases
 * (pat_arg="aaaaaa" str="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").
 * Uses char_u for match indices. Therefore patterns are limited to
 * MAX_FUZZY_MATCHES characters.
 *
 * Returns TRUE if "pat_arg" matches "str". Also returns the match score in
 * "outScore" and the matching character positions in "matches".
 */
    int
fuzzy_match(
	char_u		*str,
	char_u		*pat_arg,
	int		matchseq,
	int		*outScore,
	int_u		*matches,
	int		maxMatches)
{
    int		recursionCount = 0;
    int		len = MB_CHARLEN(str);
    char_u	*save_pat;
    char_u	*pat;
    char_u	*p;
    int		complete = FALSE;
    int		score = 0;
    int		numMatches = 0;
    int		matchCount;

    *outScore = 0;

    save_pat = vim_strsave(pat_arg);
    if (save_pat == NULL)
	return FALSE;
    pat = save_pat;
    p = pat;

    // Try matching each word in 'pat_arg' in 'str'
    while (TRUE)
    {
	if (matchseq)
	    complete = TRUE;
	else
	{
	    // Extract one word from the pattern (separated by space)
	    p = skipwhite(p);
	    if (*p == NUL)
		break;
	    pat = p;
	    while (*p != NUL && !VIM_ISWHITE(PTR2CHAR(p)))
	    {
		if (has_mbyte)
		    MB_PTR_ADV(p);
		else
		    ++p;
	    }
	    if (*p == NUL)		// processed all the words
		complete = TRUE;
	    *p = NUL;
	}

	score = 0;
	recursionCount = 0;
	matchCount = fuzzy_match_recursive(pat, str, 0, &score, str, len, NULL,
				matches + numMatches, maxMatches - numMatches,
				0, &recursionCount);
	if (matchCount == 0)
	{
	    numMatches = 0;
	    break;
	}

	// Accumulate the match score and the number of matches
	*outScore += score;
	numMatches += matchCount;

	if (complete)
	    break;

	// try matching the next word
	++p;
    }

    vim_free(save_pat);
    return numMatches != 0;
}

#if defined(FEAT_EVAL) || defined(FEAT_PROTO)
/*
 * Sort the fuzzy matches in the descending order of the match score.
 * For items with same score, retain the order using the index (stable sort)
 */
    static int
fuzzy_match_item_compare(const void *s1, const void *s2)
{
    int		v1 = ((fuzzyItem_T *)s1)->score;
    int		v2 = ((fuzzyItem_T *)s2)->score;
    int		idx1 = ((fuzzyItem_T *)s1)->idx;
    int		idx2 = ((fuzzyItem_T *)s2)->idx;

    return v1 == v2 ? (idx1 - idx2) : v1 > v2 ? -1 : 1;
}

/*
 * Fuzzy search the string 'str' in a list of 'items' and return the matching
 * strings in 'fmatchlist'.
 * If 'matchseq' is TRUE, then for multi-word search strings, match all the
 * words in sequence.
 * If 'items' is a list of strings, then search for 'str' in the list.
 * If 'items' is a list of dicts, then either use 'key' to lookup the string
 * for each item or use 'item_cb' Funcref function to get the string.
 * If 'retmatchpos' is TRUE, then return a list of positions where 'str'
 * matches for each item.
 */
    static void
fuzzy_match_in_list(
	list_T		*l,
	char_u		*str,
	int		matchseq,
	char_u		*key,
	callback_T	*item_cb,
	int		retmatchpos,
	list_T		*fmatchlist,
	long		max_matches)
{
    long	len;
    fuzzyItem_T	*items;
    listitem_T	*li;
    long	i = 0;
    long	match_count = 0;
    int_u	matches[MAX_FUZZY_MATCHES];

    len = list_len(l);
    if (len == 0)
	return;
    if (max_matches > 0 && len > max_matches)
	len = max_matches;

    items = ALLOC_CLEAR_MULT(fuzzyItem_T, len);
    if (items == NULL)
	return;

    // For all the string items in items, get the fuzzy matching score
    FOR_ALL_LIST_ITEMS(l, li)
    {
	int		score;
	char_u		*itemstr;
	typval_T	rettv;

	if (max_matches > 0 && match_count >= max_matches)
	    break;

	itemstr = NULL;
	rettv.v_type = VAR_UNKNOWN;
	if (li->li_tv.v_type == VAR_STRING)	// list of strings
	    itemstr = li->li_tv.vval.v_string;
	else if (li->li_tv.v_type == VAR_DICT
				&& (key != NULL || item_cb->cb_name != NULL))
	{
	    // For a dict, either use the specified key to lookup the string or
	    // use the specified callback function to get the string.
	    if (key != NULL)
		itemstr = dict_get_string(li->li_tv.vval.v_dict,
							   (char *)key, FALSE);
	    else
	    {
		typval_T	argv[2];

		// Invoke the supplied callback (if any) to get the dict item
		li->li_tv.vval.v_dict->dv_refcount++;
		argv[0].v_type = VAR_DICT;
		argv[0].vval.v_dict = li->li_tv.vval.v_dict;
		argv[1].v_type = VAR_UNKNOWN;
		if (call_callback(item_cb, -1, &rettv, 1, argv) != FAIL)
		{
		    if (rettv.v_type == VAR_STRING)
			itemstr = rettv.vval.v_string;
		}
		dict_unref(li->li_tv.vval.v_dict);
	    }
	}

	if (itemstr != NULL
		&& fuzzy_match(itemstr, str, matchseq, &score, matches,
							MAX_FUZZY_MATCHES))
	{
	    items[match_count].idx = match_count;
	    items[match_count].item = li;
	    items[match_count].score = score;

	    // Copy the list of matching positions in itemstr to a list, if
	    // 'retmatchpos' is set.
	    if (retmatchpos)
	    {
		int	j = 0;
		char_u	*p;

		items[match_count].lmatchpos = list_alloc();
		if (items[match_count].lmatchpos == NULL)
		    goto done;

		p = str;
		while (*p != NUL)
		{
		    if (!VIM_ISWHITE(PTR2CHAR(p)) || matchseq)
		    {
			if (list_append_number(items[match_count].lmatchpos,
				    matches[j]) == FAIL)
			    goto done;
			j++;
		    }
		    if (has_mbyte)
			MB_PTR_ADV(p);
		    else
			++p;
		}
	    }
	    ++match_count;
	}
	clear_tv(&rettv);
    }

    if (match_count > 0)
    {
	list_T		*retlist;

	// Sort the list by the descending order of the match score
	qsort((void *)items, (size_t)match_count, sizeof(fuzzyItem_T),
		fuzzy_match_item_compare);

	// For matchfuzzy(), return a list of matched strings.
	//	    ['str1', 'str2', 'str3']
	// For matchfuzzypos(), return a list with three items.
	// The first item is a list of matched strings. The second item
	// is a list of lists where each list item is a list of matched
	// character positions. The third item is a list of matching scores.
	//	[['str1', 'str2', 'str3'], [[1, 3], [1, 3], [1, 3]]]
	if (retmatchpos)
	{
	    li = list_find(fmatchlist, 0);
	    if (li == NULL || li->li_tv.vval.v_list == NULL)
		goto done;
	    retlist = li->li_tv.vval.v_list;
	}
	else
	    retlist = fmatchlist;

	// Copy the matching strings with a valid score to the return list
	for (i = 0; i < match_count; i++)
	{
	    if (items[i].score == SCORE_NONE)
		break;
	    list_append_tv(retlist, &items[i].item->li_tv);
	}

	// next copy the list of matching positions
	if (retmatchpos)
	{
	    li = list_find(fmatchlist, -2);
	    if (li == NULL || li->li_tv.vval.v_list == NULL)
		goto done;
	    retlist = li->li_tv.vval.v_list;

	    for (i = 0; i < match_count; i++)
	    {
		if (items[i].score == SCORE_NONE)
		    break;
		if (items[i].lmatchpos != NULL
		      && list_append_list(retlist, items[i].lmatchpos) == FAIL)
		    goto done;
	    }

	    // copy the matching scores
	    li = list_find(fmatchlist, -1);
	    if (li == NULL || li->li_tv.vval.v_list == NULL)
		goto done;
	    retlist = li->li_tv.vval.v_list;
	    for (i = 0; i < match_count; i++)
	    {
		if (items[i].score == SCORE_NONE)
		    break;
		if (list_append_number(retlist, items[i].score) == FAIL)
		    goto done;
	    }
	}
    }

done:
    vim_free(items);
}

/*
 * Do fuzzy matching. Returns the list of matched strings in 'rettv'.
 * If 'retmatchpos' is TRUE, also returns the matching character positions.
 */
    static void
do_fuzzymatch(typval_T *argvars, typval_T *rettv, int retmatchpos)
{
    callback_T	cb;
    char_u	*key = NULL;
    int		ret;
    int		matchseq = FALSE;
    long	max_matches = 0;

    if (in_vim9script()
	    && (check_for_list_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_opt_dict_arg(argvars, 2) == FAIL))
	return;

    CLEAR_POINTER(&cb);

    // validate and get the arguments
    if (argvars[0].v_type != VAR_LIST || argvars[0].vval.v_list == NULL)
    {
	semsg(_(e_argument_of_str_must_be_list),
			     retmatchpos ? "matchfuzzypos()" : "matchfuzzy()");
	return;
    }
    if (argvars[1].v_type != VAR_STRING
	    || argvars[1].vval.v_string == NULL)
    {
	semsg(_(e_invalid_argument_str), tv_get_string(&argvars[1]));
	return;
    }

    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	dict_T		*d;
	dictitem_T	*di;

	if (check_for_nonnull_dict_arg(argvars, 2) == FAIL)
	    return;

	// To search a dict, either a callback function or a key can be
	// specified.
	d = argvars[2].vval.v_dict;
	if ((di = dict_find(d, (char_u *)"key", -1)) != NULL)
	{
	    if (di->di_tv.v_type != VAR_STRING
		    || di->di_tv.vval.v_string == NULL
		    || *di->di_tv.vval.v_string == NUL)
	    {
		semsg(_(e_invalid_argument_str), tv_get_string(&di->di_tv));
		return;
	    }
	    key = tv_get_string(&di->di_tv);
	}
	else if ((di = dict_find(d, (char_u *)"text_cb", -1)) != NULL)
	{
	    cb = get_callback(&di->di_tv);
	    if (cb.cb_name == NULL)
	    {
		semsg(_(e_invalid_value_for_argument_str), "text_cb");
		return;
	    }
	}

	if ((di = dict_find(d, (char_u *)"limit", -1)) != NULL)
	{
	    if (di->di_tv.v_type != VAR_NUMBER)
	    {
		semsg(_(e_invalid_argument_str), tv_get_string(&di->di_tv));
		return;
	    }
	    max_matches = (long)tv_get_number_chk(&di->di_tv, NULL);
	}

	if (dict_has_key(d, "matchseq"))
	    matchseq = TRUE;
    }

    // get the fuzzy matches
    ret = rettv_list_alloc(rettv);
    if (ret == FAIL)
	goto done;
    if (retmatchpos)
    {
	list_T	*l;

	// For matchfuzzypos(), a list with three items are returned. First
	// item is a list of matching strings, the second item is a list of
	// lists with matching positions within each string and the third item
	// is the list of scores of the matches.
	l = list_alloc();
	if (l == NULL)
	    goto done;
	if (list_append_list(rettv->vval.v_list, l) == FAIL)
	{
	    vim_free(l);
	    goto done;
	}
	l = list_alloc();
	if (l == NULL)
	    goto done;
	if (list_append_list(rettv->vval.v_list, l) == FAIL)
	{
	    vim_free(l);
	    goto done;
	}
	l = list_alloc();
	if (l == NULL)
	    goto done;
	if (list_append_list(rettv->vval.v_list, l) == FAIL)
	{
	    vim_free(l);
	    goto done;
	}
    }

    fuzzy_match_in_list(argvars[0].vval.v_list, tv_get_string(&argvars[1]),
	    matchseq, key, &cb, retmatchpos, rettv->vval.v_list, max_matches);

done:
    free_callback(&cb);
}

/*
 * "matchfuzzy()" function
 */
    void
f_matchfuzzy(typval_T *argvars, typval_T *rettv)
{
    do_fuzzymatch(argvars, rettv, FALSE);
}

/*
 * "matchfuzzypos()" function
 */
    void
f_matchfuzzypos(typval_T *argvars, typval_T *rettv)
{
    do_fuzzymatch(argvars, rettv, TRUE);
}
#endif

/*
 * Same as fuzzy_match_item_compare() except for use with a string match
 */
    static int
fuzzy_match_str_compare(const void *s1, const void *s2)
{
    int		v1 = ((fuzmatch_str_T *)s1)->score;
    int		v2 = ((fuzmatch_str_T *)s2)->score;
    int		idx1 = ((fuzmatch_str_T *)s1)->idx;
    int		idx2 = ((fuzmatch_str_T *)s2)->idx;

    return v1 == v2 ? (idx1 - idx2) : v1 > v2 ? -1 : 1;
}

/*
 * Sort fuzzy matches by score
 */
    static void
fuzzy_match_str_sort(fuzmatch_str_T *fm, int sz)
{
    // Sort the list by the descending order of the match score
    qsort((void *)fm, (size_t)sz, sizeof(fuzmatch_str_T),
	    fuzzy_match_str_compare);
}

/*
 * Same as fuzzy_match_item_compare() except for use with a function name
 * string match. <SNR> functions should be sorted to the end.
 */
    static int
fuzzy_match_func_compare(const void *s1, const void *s2)
{
    int		v1 = ((fuzmatch_str_T *)s1)->score;
    int		v2 = ((fuzmatch_str_T *)s2)->score;
    int		idx1 = ((fuzmatch_str_T *)s1)->idx;
    int		idx2 = ((fuzmatch_str_T *)s2)->idx;
    char_u	*str1 = ((fuzmatch_str_T *)s1)->str;
    char_u	*str2 = ((fuzmatch_str_T *)s2)->str;

    if (*str1 != '<' && *str2 == '<') return -1;
    if (*str1 == '<' && *str2 != '<') return 1;
    return v1 == v2 ? (idx1 - idx2) : v1 > v2 ? -1 : 1;
}

/*
 * Sort fuzzy matches of function names by score.
 * <SNR> functions should be sorted to the end.
 */
    static void
fuzzy_match_func_sort(fuzmatch_str_T *fm, int sz)
{
    // Sort the list by the descending order of the match score
    qsort((void *)fm, (size_t)sz, sizeof(fuzmatch_str_T),
		fuzzy_match_func_compare);
}

/*
 * Fuzzy match 'pat' in 'str'. Returns 0 if there is no match. Otherwise,
 * returns the match score.
 */
    int
fuzzy_match_str(char_u *str, char_u *pat)
{
    int		score = 0;
    int_u	matchpos[MAX_FUZZY_MATCHES];

    if (str == NULL || pat == NULL)
	return 0;

    fuzzy_match(str, pat, TRUE, &score, matchpos,
				sizeof(matchpos) / sizeof(matchpos[0]));

    return score;
}

/*
 * Free an array of fuzzy string matches "fuzmatch[count]".
 */
    void
fuzmatch_str_free(fuzmatch_str_T *fuzmatch, int count)
{
    int i;

    if (fuzmatch == NULL)
	return;
    for (i = 0; i < count; ++i)
	vim_free(fuzmatch[i].str);
    vim_free(fuzmatch);
}

/*
 * Copy a list of fuzzy matches into a string list after sorting the matches by
 * the fuzzy score. Frees the memory allocated for 'fuzmatch'.
 * Returns OK on success and FAIL on memory allocation failure.
 */
    int
fuzzymatches_to_strmatches(
	fuzmatch_str_T	*fuzmatch,
	char_u		***matches,
	int		count,
	int		funcsort)
{
    int		i;

    if (count <= 0)
	return OK;

    *matches = ALLOC_MULT(char_u *, count);
    if (*matches == NULL)
    {
	fuzmatch_str_free(fuzmatch, count);
	return FAIL;
    }

    // Sort the list by the descending order of the match score
    if (funcsort)
	fuzzy_match_func_sort((void *)fuzmatch, (size_t)count);
    else
	fuzzy_match_str_sort((void *)fuzmatch, (size_t)count);

    for (i = 0; i < count; i++)
	(*matches)[i] = fuzmatch[i].str;
    vim_free(fuzmatch);

    return OK;
}
