/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * cmdhist.c: Functions for the history of the command-line.
 */

#include "vim.h"

static histentry_T *(history[HIST_COUNT]) = {NULL, NULL, NULL, NULL, NULL};
static int	hisidx[HIST_COUNT] = {-1, -1, -1, -1, -1};  // lastused entry
static int	hisnum[HIST_COUNT] = {0, 0, 0, 0, 0};
		    // identifying (unique) number of newest history entry
static int	hislen = 0;		// actual length of history tables

/*
 * Return the length of the history tables
 */
    int
get_hislen(void)
{
    return hislen;
}

/*
 * Return a pointer to a specified history table
 */
    histentry_T *
get_histentry(int hist_type)
{
    return history[hist_type];
}

#if defined(FEAT_VIMINFO) || defined(PROTO)
    void
set_histentry(int hist_type, histentry_T *entry)
{
    history[hist_type] = entry;
}
#endif

    int *
get_hisidx(int hist_type)
{
    return &hisidx[hist_type];
}

#if defined(FEAT_VIMINFO) || defined(PROTO)
    int *
get_hisnum(int hist_type)
{
    return &hisnum[hist_type];
}
#endif

/*
 * Translate a history character to the associated type number.
 */
    int
hist_char2type(int c)
{
    if (c == ':')
	return HIST_CMD;
    if (c == '=')
	return HIST_EXPR;
    if (c == '@')
	return HIST_INPUT;
    if (c == '>')
	return HIST_DEBUG;
    return HIST_SEARCH;	    // must be '?' or '/'
}

/*
 * Table of history names.
 * These names are used in :history and various hist...() functions.
 * It is sufficient to give the significant prefix of a history name.
 */

static char *(history_names[]) =
{
    "cmd",
    "search",
    "expr",
    "input",
    "debug",
    NULL
};

/*
 * Function given to ExpandGeneric() to obtain the possible first
 * arguments of the ":history command.
 */
    char_u *
get_history_arg(expand_T *xp UNUSED, int idx)
{
    char    *short_names = ":=@>?/";
    int	    short_names_count = (int)STRLEN(short_names);
    int	    history_name_count = ARRAY_LENGTH(history_names) - 1;

    if (idx < short_names_count)
    {
	xp->xp_buf[0] = (char_u)short_names[idx];
	xp->xp_buf[1] = NUL;
	return xp->xp_buf;
    }
    if (idx < short_names_count + history_name_count)
	return (char_u *)history_names[idx - short_names_count];
    if (idx == short_names_count + history_name_count)
	return (char_u *)"all";
    return NULL;
}

/*
 * init_history() - Initialize the command line history.
 * Also used to re-allocate the history when the size changes.
 */
    void
init_history(void)
{
    int		newlen;	    // new length of history table
    histentry_T	*temp;
    int		i;
    int		j;
    int		type;

    // If size of history table changed, reallocate it
    newlen = (int)p_hi;
    if (newlen == hislen)		// history length didn't change
	return;

    // history length changed
    for (type = 0; type < HIST_COUNT; ++type)   // adjust the tables
    {
	if (newlen > 0)
	{
	    temp = ALLOC_MULT(histentry_T, newlen);
	    if (temp == NULL)   // out of memory!
	    {
		if (type == 0)  // first one: just keep the old length
		{
		    newlen = hislen;
		    break;
		}
		// Already changed one table, now we can only have zero
		// length for all tables.
		newlen = 0;
		type = -1;
		continue;
	    }
	}
	else
	    temp = NULL;

	if (hisidx[type] < 0)		// there are no entries yet
	{
	    for (i = 0; i < newlen; ++i)
		clear_hist_entry(&temp[i]);
	}
	else if (newlen > hislen)	// array becomes bigger
	{
	    for (i = 0; i <= hisidx[type]; ++i)
		temp[i] = history[type][i];
	    j = i;
	    for ( ; i <= newlen - (hislen - hisidx[type]); ++i)
		clear_hist_entry(&temp[i]);
	    for ( ; j < hislen; ++i, ++j)
		temp[i] = history[type][j];
	}
	else				// array becomes smaller or 0
	{
	    j = hisidx[type];
	    for (i = newlen - 1; ; --i)
	    {
		if (i >= 0)		// copy newest entries
		    temp[i] = history[type][j];
		else			// remove older entries
		    vim_free(history[type][j].hisstr);
		if (--j < 0)
		    j = hislen - 1;
		if (j == hisidx[type])
		    break;
	    }
	    hisidx[type] = newlen - 1;
	}
	vim_free(history[type]);
	history[type] = temp;
    }
    hislen = newlen;
}

    void
clear_hist_entry(histentry_T *hisptr)
{
    hisptr->hisnum = 0;
    hisptr->viminfo = FALSE;
    hisptr->hisstr = NULL;
    hisptr->time_set = 0;
}

/*
 * Check if command line 'str' is already in history.
 * If 'move_to_front' is TRUE, matching entry is moved to end of history.
 */
    int
in_history(
    int	    type,
    char_u  *str,
    int	    move_to_front,	// Move the entry to the front if it exists
    int	    sep,
    int	    writing)		// ignore entries read from viminfo
{
    int	    i;
    int	    last_i = -1;
    char_u  *p;

    if (hisidx[type] < 0)
	return FALSE;
    i = hisidx[type];
    do
    {
	if (history[type][i].hisstr == NULL)
	    return FALSE;

	// For search history, check that the separator character matches as
	// well.
	p = history[type][i].hisstr;
	if (STRCMP(str, p) == 0
		&& !(writing && history[type][i].viminfo)
		&& (type != HIST_SEARCH || sep == p[STRLEN(p) + 1]))
	{
	    if (!move_to_front)
		return TRUE;
	    last_i = i;
	    break;
	}
	if (--i < 0)
	    i = hislen - 1;
    } while (i != hisidx[type]);

    if (last_i < 0)
	return FALSE;

    str = history[type][i].hisstr;
    while (i != hisidx[type])
    {
	if (++i >= hislen)
	    i = 0;
	history[type][last_i] = history[type][i];
	last_i = i;
    }
    history[type][i].hisnum = ++hisnum[type];
    history[type][i].viminfo = FALSE;
    history[type][i].hisstr = str;
    history[type][i].time_set = vim_time();
    return TRUE;
}

/*
 * Convert history name (from table above) to its HIST_ equivalent.
 * When "name" is empty, return "cmd" history.
 * Returns -1 for unknown history name.
 */
    static int
get_histtype(char_u *name)
{
    int		i;
    int		len = (int)STRLEN(name);

    // No argument: use current history.
    if (len == 0)
	return hist_char2type(get_cmdline_firstc());

    for (i = 0; history_names[i] != NULL; ++i)
	if (STRNICMP(name, history_names[i], len) == 0)
	    return i;

    if (vim_strchr((char_u *)":=@>?/", name[0]) != NULL && name[1] == NUL)
	return hist_char2type(name[0]);

    return -1;
}

static int	last_maptick = -1;	// last seen maptick

/*
 * Add the given string to the given history.  If the string is already in the
 * history then it is moved to the front.  "histype" may be one of he HIST_
 * values.
 */
    void
add_to_history(
    int		histype,
    char_u	*new_entry,
    int		in_map,		// consider maptick when inside a mapping
    int		sep)		// separator character used (search hist)
{
    histentry_T	*hisptr;
    int		len;

    if (hislen == 0)		// no history
	return;

    if ((cmdmod.cmod_flags & CMOD_KEEPPATTERNS) && histype == HIST_SEARCH)
	return;

    // Searches inside the same mapping overwrite each other, so that only
    // the last line is kept.  Be careful not to remove a line that was moved
    // down, only lines that were added.
    if (histype == HIST_SEARCH && in_map)
    {
	if (maptick == last_maptick && hisidx[HIST_SEARCH] >= 0)
	{
	    // Current line is from the same mapping, remove it
	    hisptr = &history[HIST_SEARCH][hisidx[HIST_SEARCH]];
	    vim_free(hisptr->hisstr);
	    clear_hist_entry(hisptr);
	    --hisnum[histype];
	    if (--hisidx[HIST_SEARCH] < 0)
		hisidx[HIST_SEARCH] = hislen - 1;
	}
	last_maptick = -1;
    }

    if (in_history(histype, new_entry, TRUE, sep, FALSE))
	return;

    if (++hisidx[histype] == hislen)
	hisidx[histype] = 0;
    hisptr = &history[histype][hisidx[histype]];
    vim_free(hisptr->hisstr);

    // Store the separator after the NUL of the string.
    len = (int)STRLEN(new_entry);
    hisptr->hisstr = vim_strnsave(new_entry, len + 2);
    if (hisptr->hisstr != NULL)
	hisptr->hisstr[len + 1] = sep;

    hisptr->hisnum = ++hisnum[histype];
    hisptr->viminfo = FALSE;
    hisptr->time_set = vim_time();
    if (histype == HIST_SEARCH && in_map)
	last_maptick = maptick;
}

#if defined(FEAT_EVAL) || defined(PROTO)

/*
 * Get identifier of newest history entry.
 * "histype" may be one of the HIST_ values.
 */
    static int
get_history_idx(int histype)
{
    if (hislen == 0 || histype < 0 || histype >= HIST_COUNT
		    || hisidx[histype] < 0)
	return -1;

    return history[histype][hisidx[histype]].hisnum;
}

/*
 * Calculate history index from a number:
 *   num > 0: seen as identifying number of a history entry
 *   num < 0: relative position in history wrt newest entry
 * "histype" may be one of the HIST_ values.
 */
    static int
calc_hist_idx(int histype, int num)
{
    int		i;
    histentry_T	*hist;
    int		wrapped = FALSE;

    if (hislen == 0 || histype < 0 || histype >= HIST_COUNT
		    || (i = hisidx[histype]) < 0 || num == 0)
	return -1;

    hist = history[histype];
    if (num > 0)
    {
	while (hist[i].hisnum > num)
	    if (--i < 0)
	    {
		if (wrapped)
		    break;
		i += hislen;
		wrapped = TRUE;
	    }
	if (i >= 0 && hist[i].hisnum == num && hist[i].hisstr != NULL)
	    return i;
    }
    else if (-num <= hislen)
    {
	i += num + 1;
	if (i < 0)
	    i += hislen;
	if (hist[i].hisstr != NULL)
	    return i;
    }
    return -1;
}

/*
 * Get a history entry by its index.
 * "histype" may be one of the HIST_ values.
 */
    static char_u *
get_history_entry(int histype, int idx)
{
    idx = calc_hist_idx(histype, idx);
    if (idx >= 0)
	return history[histype][idx].hisstr;
    else
	return (char_u *)"";
}

/*
 * Clear all entries of a history.
 * "histype" may be one of the HIST_ values.
 */
    static int
clr_history(int histype)
{
    int		i;
    histentry_T	*hisptr;

    if (hislen != 0 && histype >= 0 && histype < HIST_COUNT)
    {
	hisptr = history[histype];
	for (i = hislen; i--;)
	{
	    vim_free(hisptr->hisstr);
	    clear_hist_entry(hisptr);
	    hisptr++;
	}
	hisidx[histype] = -1;	// mark history as cleared
	hisnum[histype] = 0;	// reset identifier counter
	return OK;
    }
    return FAIL;
}

/*
 * Remove all entries matching {str} from a history.
 * "histype" may be one of the HIST_ values.
 */
    static int
del_history_entry(int histype, char_u *str)
{
    regmatch_T	regmatch;
    histentry_T	*hisptr;
    int		idx;
    int		i;
    int		last;
    int		found = FALSE;

    if (hislen == 0 || histype < 0 || histype >= HIST_COUNT || *str == NUL
		|| hisidx[histype] < 0)
	return FALSE;

    idx = hisidx[histype];
    regmatch.regprog = vim_regcomp(str, RE_MAGIC + RE_STRING);
    if (regmatch.regprog == NULL)
	return FALSE;

    regmatch.rm_ic = FALSE;	// always match case

    i = last = idx;
    do
    {
	hisptr = &history[histype][i];
	if (hisptr->hisstr == NULL)
	    break;
	if (vim_regexec(&regmatch, hisptr->hisstr, (colnr_T)0))
	{
	    found = TRUE;
	    vim_free(hisptr->hisstr);
	    clear_hist_entry(hisptr);
	}
	else
	{
	    if (i != last)
	    {
		history[histype][last] = *hisptr;
		clear_hist_entry(hisptr);
	    }
	    if (--last < 0)
		last += hislen;
	}
	if (--i < 0)
	    i += hislen;
    } while (i != idx);

    if (history[histype][idx].hisstr == NULL)
	hisidx[histype] = -1;

    vim_regfree(regmatch.regprog);
    return found;
}

/*
 * Remove an indexed entry from a history.
 * "histype" may be one of the HIST_ values.
 */
    static int
del_history_idx(int histype, int idx)
{
    int	    i, j;

    i = calc_hist_idx(histype, idx);
    if (i < 0)
	return FALSE;
    idx = hisidx[histype];
    vim_free(history[histype][i].hisstr);

    // When deleting the last added search string in a mapping, reset
    // last_maptick, so that the last added search string isn't deleted again.
    if (histype == HIST_SEARCH && maptick == last_maptick && i == idx)
	last_maptick = -1;

    while (i != idx)
    {
	j = (i + 1) % hislen;
	history[histype][i] = history[histype][j];
	i = j;
    }
    clear_hist_entry(&history[histype][i]);
    if (--i < 0)
	i += hislen;
    hisidx[histype] = i;
    return TRUE;
}

/*
 * "histadd()" function
 */
    void
f_histadd(typval_T *argvars UNUSED, typval_T *rettv)
{
    int		histype;
    char_u	*str;
    char_u	buf[NUMBUFLEN];

    rettv->vval.v_number = FALSE;
    if (check_secure())
	return;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL))
	return;

    str = tv_get_string_chk(&argvars[0]);	// NULL on type error
    histype = str != NULL ? get_histtype(str) : -1;
    if (histype < 0)
	return;

    str = tv_get_string_buf(&argvars[1], buf);
    if (*str == NUL)
	return;

    init_history();
    add_to_history(histype, str, FALSE, NUL);
    rettv->vval.v_number = TRUE;
}

/*
 * "histdel()" function
 */
    void
f_histdel(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
    int		n;
    char_u	buf[NUMBUFLEN];
    char_u	*str;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_or_number_arg(argvars, 1) == FAIL))
	return;

    str = tv_get_string_chk(&argvars[0]);	// NULL on type error
    if (str == NULL)
	n = 0;
    else if (argvars[1].v_type == VAR_UNKNOWN)
	// only one argument: clear entire history
	n = clr_history(get_histtype(str));
    else if (argvars[1].v_type == VAR_NUMBER)
	// index given: remove that entry
	n = del_history_idx(get_histtype(str),
					  (int)tv_get_number(&argvars[1]));
    else
	// string given: remove all matching entries
	n = del_history_entry(get_histtype(str),
				      tv_get_string_buf(&argvars[1], buf));
    rettv->vval.v_number = n;
}

/*
 * "histget()" function
 */
    void
f_histget(typval_T *argvars UNUSED, typval_T *rettv)
{
    int		type;
    int		idx;
    char_u	*str;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL))
	return;

    str = tv_get_string_chk(&argvars[0]);	// NULL on type error
    if (str == NULL)
	rettv->vval.v_string = NULL;
    else
    {
	type = get_histtype(str);
	if (argvars[1].v_type == VAR_UNKNOWN)
	    idx = get_history_idx(type);
	else
	    idx = (int)tv_get_number_chk(&argvars[1], NULL);
						    // -1 on type error
	rettv->vval.v_string = vim_strsave(get_history_entry(type, idx));
    }
    rettv->v_type = VAR_STRING;
}

/*
 * "histnr()" function
 */
    void
f_histnr(typval_T *argvars UNUSED, typval_T *rettv)
{
    int		i;
    char_u	*histname;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    histname = tv_get_string_chk(&argvars[0]);
    i = histname == NULL ? HIST_CMD - 1 : get_histtype(histname);
    if (i >= HIST_CMD && i < HIST_COUNT)
	i = get_history_idx(i);
    else
	i = -1;
    rettv->vval.v_number = i;
}
#endif // FEAT_EVAL

#if defined(FEAT_CRYPT) || defined(PROTO)
/*
 * Very specific function to remove the value in ":set key=val" from the
 * history.
 */
    void
remove_key_from_history(void)
{
    char_u	*p;
    int		i;

    i = hisidx[HIST_CMD];
    if (i < 0)
	return;
    p = history[HIST_CMD][i].hisstr;
    if (p == NULL)
	return;

    for ( ; *p; ++p)
	if (STRNCMP(p, "key", 3) == 0 && !SAFE_isalpha(p[3]))
	{
	    p = vim_strchr(p + 3, '=');
	    if (p == NULL)
		break;
	    ++p;
	    for (i = 0; p[i] && !VIM_ISWHITE(p[i]); ++i)
		if (p[i] == '\\' && p[i + 1])
		    ++i;
	    STRMOVE(p, p + i);
	    --p;
	}
}
#endif

/*
 * :history command - print a history
 */
    void
ex_history(exarg_T *eap)
{
    histentry_T	*hist;
    int		histype1 = HIST_CMD;
    int		histype2 = HIST_CMD;
    int		hisidx1 = 1;
    int		hisidx2 = -1;
    int		idx;
    int		i, j, k;
    char_u	*end;
    char_u	*arg = eap->arg;

    if (hislen == 0)
    {
	msg(_("'history' option is zero"));
	return;
    }

    if (!(VIM_ISDIGIT(*arg) || *arg == '-' || *arg == ','))
    {
	end = arg;
	while (ASCII_ISALPHA(*end)
		|| vim_strchr((char_u *)":=@>/?", *end) != NULL)
	    end++;
	i = *end;
	*end = NUL;
	histype1 = get_histtype(arg);
	if (histype1 == -1)
	{
	    if (STRNICMP(arg, "all", STRLEN(arg)) == 0)
	    {
		histype1 = 0;
		histype2 = HIST_COUNT-1;
	    }
	    else
	    {
		*end = i;
		semsg(_(e_trailing_characters_str), arg);
		return;
	    }
	}
	else
	    histype2 = histype1;
	*end = i;
    }
    else
	end = arg;
    if (!get_list_range(&end, &hisidx1, &hisidx2) || *end != NUL)
    {
	if (*end != NUL)
	    semsg(_(e_trailing_characters_str), end);
	else
	    semsg(_(e_val_too_large), arg);
	return;
    }

    for (; !got_int && histype1 <= histype2; ++histype1)
    {
	STRCPY(IObuff, "\n      #  ");
	STRCAT(STRCAT(IObuff, history_names[histype1]), " history");
	msg_puts_title((char *)IObuff);
	idx = hisidx[histype1];
	hist = history[histype1];
	j = hisidx1;
	k = hisidx2;
	if (j < 0)
	    j = (-j > hislen) ? 0 : hist[(hislen+j+idx+1) % hislen].hisnum;
	if (k < 0)
	    k = (-k > hislen) ? 0 : hist[(hislen+k+idx+1) % hislen].hisnum;
	if (idx >= 0 && j <= k)
	    for (i = idx + 1; !got_int; ++i)
	    {
		if (i == hislen)
		    i = 0;
		if (hist[i].hisstr != NULL
			&& hist[i].hisnum >= j && hist[i].hisnum <= k)
		{
		    msg_putchar('\n');
		    sprintf((char *)IObuff, "%c%6d  ", i == idx ? '>' : ' ',
							      hist[i].hisnum);
		    if (vim_strsize(hist[i].hisstr) > (int)Columns - 10)
			trunc_string(hist[i].hisstr, IObuff + STRLEN(IObuff),
			     (int)Columns - 10, IOSIZE - (int)STRLEN(IObuff));
		    else
			STRCAT(IObuff, hist[i].hisstr);
		    msg_outtrans(IObuff);
		    out_flush();
		}
		if (i == idx)
		    break;
	    }
    }
}
