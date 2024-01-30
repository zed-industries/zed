/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * viminfo.c: viminfo related functions
 */

#include "vim.h"
#include "version.h"

/*
 * Structure used for reading from the viminfo file.
 */
typedef struct
{
    char_u	*vir_line;	// text of the current line
    FILE	*vir_fd;	// file descriptor
    vimconv_T	vir_conv;	// encoding conversion
    int		vir_version;	// viminfo version detected or -1
    garray_T	vir_barlines;	// lines starting with |
} vir_T;

typedef enum {
    BVAL_NR,
    BVAL_STRING,
    BVAL_EMPTY
} btype_T;

typedef struct {
    btype_T	bv_type;
    long	bv_nr;
    char_u	*bv_string;
    char_u	*bv_tofree;	// free later when not NULL
    int		bv_len;		// length of bv_string
    int		bv_allocated;	// bv_string was allocated
} bval_T;

#if defined(FEAT_VIMINFO) || defined(PROTO)

static int  viminfo_errcnt;

/*
 * Find the parameter represented by the given character (eg ''', ':', '"', or
 * '/') in the 'viminfo' option and return a pointer to the string after it.
 * Return NULL if the parameter is not specified in the string.
 */
    static char_u *
find_viminfo_parameter(int type)
{
    char_u  *p;

    for (p = p_viminfo; *p; ++p)
    {
	if (*p == type)
	    return p + 1;
	if (*p == 'n')		    // 'n' is always the last one
	    break;
	p = vim_strchr(p, ',');	    // skip until next ','
	if (p == NULL)		    // hit the end without finding parameter
	    break;
    }
    return NULL;
}

/*
 * Find the parameter represented by the given character (eg ', :, ", or /),
 * and return its associated value in the 'viminfo' string.
 * Only works for number parameters, not for 'r' or 'n'.
 * If the parameter is not specified in the string or there is no following
 * number, return -1.
 */
    int
get_viminfo_parameter(int type)
{
    char_u  *p;

    p = find_viminfo_parameter(type);
    if (p != NULL && VIM_ISDIGIT(*p))
	return atoi((char *)p);
    return -1;
}

/*
 * Get the viminfo file name to use.
 * If "file" is given and not empty, use it (has already been expanded by
 * cmdline functions).
 * Otherwise use "-i file_name", value from 'viminfo' or the default, and
 * expand environment variables.
 * Returns an allocated string.  NULL when out of memory.
 */
    static char_u *
viminfo_filename(char_u *file)
{
    if (file == NULL || *file == NUL)
    {
	if (*p_viminfofile != NUL)
	    file = p_viminfofile;
	else if ((file = find_viminfo_parameter('n')) == NULL || *file == NUL)
	{
#ifdef VIMINFO_FILE2
# ifdef VMS
	    if (mch_getenv((char_u *)"SYS$LOGIN") == NULL)
# else
#  ifdef MSWIN
	    // Use $VIM only if $HOME is the default "C:/".
	    if (STRCMP(vim_getenv((char_u *)"HOME", NULL), "C:/") == 0
		    && mch_getenv((char_u *)"HOME") == NULL)
#  else
	    if (mch_getenv((char_u *)"HOME") == NULL)
#  endif
# endif
	    {
		// don't use $VIM when not available.
		expand_env((char_u *)"$VIM", NameBuff, MAXPATHL);
		if (STRCMP("$VIM", NameBuff) != 0)  // $VIM was expanded
		    file = (char_u *)VIMINFO_FILE2;
		else
		    file = (char_u *)VIMINFO_FILE;
	    }
	    else
#endif
		file = (char_u *)VIMINFO_FILE;
	}
	expand_env(file, NameBuff, MAXPATHL);
	file = NameBuff;
    }
    return vim_strsave(file);
}

/*
 * write string to viminfo file
 * - replace CTRL-V with CTRL-V CTRL-V
 * - replace '\n'   with CTRL-V 'n'
 * - add a '\n' at the end
 *
 * For a long line:
 * - write " CTRL-V <length> \n " in first line
 * - write " < <string> \n "	  in second line
 */
    static void
viminfo_writestring(FILE *fd, char_u *p)
{
    int		c;
    char_u	*s;
    int		len = 0;

    for (s = p; *s != NUL; ++s)
    {
	if (*s == Ctrl_V || *s == '\n')
	    ++len;
	++len;
    }

    // If the string will be too long, write its length and put it in the next
    // line.  Take into account that some room is needed for what comes before
    // the string (e.g., variable name).  Add something to the length for the
    // '<', NL and trailing NUL.
    if (len > LSIZE / 2)
	fprintf(fd, "\026%d\n<", len + 3);

    while ((c = *p++) != NUL)
    {
	if (c == Ctrl_V || c == '\n')
	{
	    putc(Ctrl_V, fd);
	    if (c == '\n')
		c = 'n';
	}
	putc(c, fd);
    }
    putc('\n', fd);
}

/*
 * Write a string in quotes that barline_parse() can read back.
 * Breaks the line in less than LSIZE pieces when needed.
 * Returns remaining characters in the line.
 */
    static int
barline_writestring(FILE *fd, char_u *s, int remaining_start)
{
    char_u *p;
    int	    remaining = remaining_start;
    int	    len = 2;

    // Count the number of characters produced, including quotes.
    for (p = s; *p != NUL; ++p)
    {
	if (*p == NL)
	    len += 2;
	else if (*p == '"' || *p == '\\')
	    len += 2;
	else
	    ++len;
    }
    if (len > remaining - 2)
    {
	fprintf(fd, ">%d\n|<", len);
	remaining = LSIZE - 20;
    }

    putc('"', fd);
    for (p = s; *p != NUL; ++p)
    {
	if (*p == NL)
	{
	    putc('\\', fd);
	    putc('n', fd);
	    --remaining;
	}
	else if (*p == '"' || *p == '\\')
	{
	    putc('\\', fd);
	    putc(*p, fd);
	    --remaining;
	}
	else
	    putc(*p, fd);
	--remaining;

	if (remaining < 3)
	{
	    putc('\n', fd);
	    putc('|', fd);
	    putc('<', fd);
	    // Leave enough space for another continuation.
	    remaining = LSIZE - 20;
	}
    }
    putc('"', fd);
    return remaining - 2;
}

/*
 * Check string read from viminfo file.
 * Remove '\n' at the end of the line.
 * - replace CTRL-V CTRL-V with CTRL-V
 * - replace CTRL-V 'n'    with '\n'
 *
 * Check for a long line as written by viminfo_writestring().
 *
 * Return the string in allocated memory (NULL when out of memory).
 */
    static char_u *
viminfo_readstring(
    vir_T	*virp,
    int		off,		    // offset for virp->vir_line
    int		convert UNUSED)	    // convert the string
{
    char_u	*retval = NULL;
    char_u	*s, *d;
    long	len;

    if (virp->vir_line[off] == Ctrl_V && vim_isdigit(virp->vir_line[off + 1]))
    {
	len = atol((char *)virp->vir_line + off + 1);
	if (len > 0 && len < 1000000)
	    retval = lalloc(len, TRUE);
	if (retval == NULL)
	{
	    // Invalid length, line too long, out of memory?  Skip next line.
	    (void)vim_fgets(virp->vir_line, 10, virp->vir_fd);
	    return NULL;
	}
	(void)vim_fgets(retval, (int)len, virp->vir_fd);
	s = retval + 1;	    // Skip the leading '<'
    }
    else
    {
	retval = vim_strsave(virp->vir_line + off);
	if (retval == NULL)
	    return NULL;
	s = retval;
    }

    // Change CTRL-V CTRL-V to CTRL-V and CTRL-V n to \n in-place.
    d = retval;
    while (*s != NUL && *s != '\n')
    {
	if (s[0] == Ctrl_V && s[1] != NUL)
	{
	    if (s[1] == 'n')
		*d++ = '\n';
	    else
		*d++ = Ctrl_V;
	    s += 2;
	}
	else
	    *d++ = *s++;
    }
    *d = NUL;

    if (convert && virp->vir_conv.vc_type != CONV_NONE && *retval != NUL)
    {
	d = string_convert(&virp->vir_conv, retval, NULL);
	if (d != NULL)
	{
	    vim_free(retval);
	    retval = d;
	}
    }

    return retval;
}

/*
 * Read a line from the viminfo file.
 * Returns TRUE for end-of-file;
 */
    static int
viminfo_readline(vir_T *virp)
{
    return vim_fgets(virp->vir_line, LSIZE, virp->vir_fd);
}

    static int
read_viminfo_bufferlist(
    vir_T	*virp,
    int		writing)
{
    char_u	*tab;
    linenr_T	lnum;
    colnr_T	col;
    buf_T	*buf;
    char_u	*sfname;
    char_u	*xline;

    // Handle long line and escaped characters.
    xline = viminfo_readstring(virp, 1, FALSE);

    // don't read in if there are files on the command-line or if writing:
    if (xline != NULL && !writing && ARGCOUNT == 0
				       && find_viminfo_parameter('%') != NULL)
    {
	// Format is: <fname> Tab <lnum> Tab <col>.
	// Watch out for a Tab in the file name, work from the end.
	lnum = 0;
	col = 0;
	tab = vim_strrchr(xline, '\t');
	if (tab != NULL)
	{
	    *tab++ = '\0';
	    col = (colnr_T)atoi((char *)tab);
	    tab = vim_strrchr(xline, '\t');
	    if (tab != NULL)
	    {
		*tab++ = '\0';
		lnum = atol((char *)tab);
	    }
	}

	// Expand "~/" in the file name at "line + 1" to a full path.
	// Then try shortening it by comparing with the current directory
	expand_env(xline, NameBuff, MAXPATHL);
	sfname = shorten_fname1(NameBuff);

	buf = buflist_new(NameBuff, sfname, (linenr_T)0, BLN_LISTED);
	if (buf != NULL)	// just in case...
	{
	    buf->b_last_cursor.lnum = lnum;
	    buf->b_last_cursor.col = col;
	    buflist_setfpos(buf, curwin, lnum, col, FALSE);
	}
    }
    vim_free(xline);

    return viminfo_readline(virp);
}

/*
 * Return TRUE if "name" is on removable media (depending on 'viminfo').
 */
    static int
removable(char_u *name)
{
    char_u  *p;
    char_u  part[51];
    int	    retval = FALSE;
    size_t  n;

    name = home_replace_save(NULL, name);
    if (name == NULL)
	return FALSE;
    for (p = p_viminfo; *p; )
    {
	copy_option_part(&p, part, 51, ", ");
	if (part[0] == 'r')
	{
	    n = STRLEN(part + 1);
	    if (MB_STRNICMP(part + 1, name, n) == 0)
	    {
		retval = TRUE;
		break;
	    }
	}
    }
    vim_free(name);
    return retval;
}

    static void
write_viminfo_bufferlist(FILE *fp)
{
    buf_T	*buf;
    win_T	*win;
    tabpage_T	*tp;
    char_u	*line;
    int		max_buffers;

    if (find_viminfo_parameter('%') == NULL)
	return;

    // Without a number -1 is returned: do all buffers.
    max_buffers = get_viminfo_parameter('%');

    // Allocate room for the file name, lnum and col.
#define LINE_BUF_LEN (MAXPATHL + 40)
    line = alloc(LINE_BUF_LEN);
    if (line == NULL)
	return;

    FOR_ALL_TAB_WINDOWS(tp, win)
	set_last_cursor(win);

    fputs(_("\n# Buffer list:\n"), fp);
    FOR_ALL_BUFFERS(buf)
    {
	if (buf->b_fname == NULL
		|| !buf->b_p_bl
		|| bt_quickfix(buf)
		|| bt_terminal(buf)
		|| removable(buf->b_ffname))
	    continue;

	if (max_buffers-- == 0)
	    break;
	putc('%', fp);
	home_replace(NULL, buf->b_ffname, line, MAXPATHL, TRUE);
	vim_snprintf_add((char *)line, LINE_BUF_LEN, "\t%ld\t%d",
			(long)buf->b_last_cursor.lnum,
			buf->b_last_cursor.col);
	viminfo_writestring(fp, line);
    }
    vim_free(line);
}

/*
 * Buffers for history read from a viminfo file.  Only valid while reading.
 */
static histentry_T *viminfo_history[HIST_COUNT] =
					       {NULL, NULL, NULL, NULL, NULL};
static int	viminfo_hisidx[HIST_COUNT] = {0, 0, 0, 0, 0};
static int	viminfo_hislen[HIST_COUNT] = {0, 0, 0, 0, 0};
static int	viminfo_add_at_front = FALSE;

/*
 * Translate a history type number to the associated character.
 */
    static int
hist_type2char(
    int	    type,
    int	    use_question)	    // use '?' instead of '/'
{
    if (type == HIST_CMD)
	return ':';
    if (type == HIST_SEARCH)
    {
	if (use_question)
	    return '?';
	else
	    return '/';
    }
    if (type == HIST_EXPR)
	return '=';
    return '@';
}

/*
 * Prepare for reading the history from the viminfo file.
 * This allocates history arrays to store the read history lines.
 */
    static void
prepare_viminfo_history(int asklen, int writing)
{
    int	    i;
    int	    num;
    int	    type;
    int	    len;
    int	    hislen;

    init_history();
    hislen = get_hislen();
    viminfo_add_at_front = (asklen != 0 && !writing);
    if (asklen > hislen)
	asklen = hislen;

    for (type = 0; type < HIST_COUNT; ++type)
    {
	histentry_T *histentry = get_histentry(type);

	// Count the number of empty spaces in the history list.  Entries read
	// from viminfo previously are also considered empty.  If there are
	// more spaces available than we request, then fill them up.
	for (i = 0, num = 0; i < hislen; i++)
	    if (histentry[i].hisstr == NULL || histentry[i].viminfo)
		num++;
	len = asklen;
	if (num > len)
	    len = num;
	if (len <= 0)
	    viminfo_history[type] = NULL;
	else
	    viminfo_history[type] = LALLOC_MULT(histentry_T, len);
	if (viminfo_history[type] == NULL)
	    len = 0;
	viminfo_hislen[type] = len;
	viminfo_hisidx[type] = 0;
    }
}

/*
 * Accept a line from the viminfo, store it in the history array when it's
 * new.
 */
    static int
read_viminfo_history(vir_T *virp, int writing)
{
    int		type;
    long_u	len;
    char_u	*val = NULL;
    char_u	*p;

    type = hist_char2type(virp->vir_line[0]);
    if (viminfo_hisidx[type] >= viminfo_hislen[type])
	goto done;

    val = viminfo_readstring(virp, 1, TRUE);
    if (val == NULL || *val == NUL)
	goto done;

    int sep = (*val == ' ' ? NUL : *val);

    if (in_history(type, val + (type == HIST_SEARCH), viminfo_add_at_front,
								sep, writing))
	goto done;

    // Need to re-allocate to append the separator byte.
    len = STRLEN(val);
    p = alloc(len + 2);
    if (p == NULL)
	goto done;

    if (type == HIST_SEARCH)
    {
	// Search entry: Move the separator from the first
	// column to after the NUL.
	mch_memmove(p, val + 1, (size_t)len);
	p[len] = sep;
    }
    else
    {
	// Not a search entry: No separator in the viminfo
	// file, add a NUL separator.
	mch_memmove(p, val, (size_t)len + 1);
	p[len + 1] = NUL;
    }
    viminfo_history[type][viminfo_hisidx[type]].hisstr = p;
    viminfo_history[type][viminfo_hisidx[type]].time_set = 0;
    viminfo_history[type][viminfo_hisidx[type]].viminfo = TRUE;
    viminfo_history[type][viminfo_hisidx[type]].hisnum = 0;
    viminfo_hisidx[type]++;

done:
    vim_free(val);
    return viminfo_readline(virp);
}

/*
 * Accept a new style history line from the viminfo, store it in the history
 * array when it's new.
 */
    static void
handle_viminfo_history(
	garray_T    *values,
	int	    writing)
{
    int		type;
    long_u	len;
    char_u	*val;
    char_u	*p;
    bval_T	*vp = (bval_T *)values->ga_data;

    // Check the format:
    // |{bartype},{histtype},{timestamp},{separator},"text"
    if (values->ga_len < 4
	    || vp[0].bv_type != BVAL_NR
	    || vp[1].bv_type != BVAL_NR
	    || (vp[2].bv_type != BVAL_NR && vp[2].bv_type != BVAL_EMPTY)
	    || vp[3].bv_type != BVAL_STRING)
	return;

    type = vp[0].bv_nr;
    if (type >= HIST_COUNT)
	return;

    if (viminfo_hisidx[type] >= viminfo_hislen[type])
	return;

    val = vp[3].bv_string;
    if (val == NULL || *val == NUL)
	return;

    int sep = type == HIST_SEARCH && vp[2].bv_type == BVAL_NR
							? vp[2].bv_nr : NUL;
    int idx;
    int overwrite = FALSE;

    if (in_history(type, val, viminfo_add_at_front, sep, writing))
	return;

    // If lines were written by an older Vim we need to avoid
    // getting duplicates. See if the entry already exists.
    for (idx = 0; idx < viminfo_hisidx[type]; ++idx)
    {
	p = viminfo_history[type][idx].hisstr;
	if (STRCMP(val, p) == 0
		&& (type != HIST_SEARCH || sep == p[STRLEN(p) + 1]))
	{
	    overwrite = TRUE;
	    break;
	}
    }

    if (!overwrite)
    {
	// Need to re-allocate to append the separator byte.
	len = vp[3].bv_len;
	p = alloc(len + 2);
    }
    else
	len = 0; // for picky compilers
    if (p != NULL)
    {
	viminfo_history[type][idx].time_set = vp[1].bv_nr;
	if (!overwrite)
	{
	    mch_memmove(p, val, (size_t)len + 1);
	    // Put the separator after the NUL.
	    p[len + 1] = sep;
	    viminfo_history[type][idx].hisstr = p;
	    viminfo_history[type][idx].hisnum = 0;
	    viminfo_history[type][idx].viminfo = TRUE;
	    viminfo_hisidx[type]++;
	}
    }
}

/*
 * Concatenate history lines from viminfo after the lines typed in this Vim.
 */
    static void
concat_history(int type)
{
    int		idx;
    int		i;
    int		hislen = get_hislen();
    histentry_T *histentry = get_histentry(type);
    int		*hisidx = get_hisidx(type);
    int		*hisnum = get_hisnum(type);

    idx = *hisidx + viminfo_hisidx[type];
    if (idx >= hislen)
	idx -= hislen;
    else if (idx < 0)
	idx = hislen - 1;
    if (viminfo_add_at_front)
	*hisidx = idx;
    else
    {
	if (*hisidx == -1)
	    *hisidx = hislen - 1;
	do
	{
	    if (histentry[idx].hisstr != NULL || histentry[idx].viminfo)
		break;
	    if (++idx == hislen)
		idx = 0;
	} while (idx != *hisidx);
	if (idx != *hisidx && --idx < 0)
	    idx = hislen - 1;
    }
    for (i = 0; i < viminfo_hisidx[type]; i++)
    {
	vim_free(histentry[idx].hisstr);
	histentry[idx].hisstr = viminfo_history[type][i].hisstr;
	histentry[idx].viminfo = TRUE;
	histentry[idx].time_set = viminfo_history[type][i].time_set;
	if (--idx < 0)
	    idx = hislen - 1;
    }
    idx += 1;
    idx %= hislen;
    for (i = 0; i < viminfo_hisidx[type]; i++)
    {
	histentry[idx++].hisnum = ++*hisnum;
	idx %= hislen;
    }
}

    static int
sort_hist(const void *s1, const void *s2)
{
    histentry_T *p1 = *(histentry_T **)s1;
    histentry_T *p2 = *(histentry_T **)s2;

    if (p1->time_set < p2->time_set) return -1;
    if (p1->time_set > p2->time_set) return 1;
    return 0;
}

/*
 * Merge history lines from viminfo and lines typed in this Vim based on the
 * timestamp;
 */
    static void
merge_history(int type)
{
    int		max_len;
    histentry_T **tot_hist;
    histentry_T *new_hist;
    int		i;
    int		len;
    int		hislen = get_hislen();
    histentry_T *histentry = get_histentry(type);
    int		*hisidx = get_hisidx(type);
    int		*hisnum = get_hisnum(type);

    // Make one long list with all entries.
    max_len = hislen + viminfo_hisidx[type];
    tot_hist = ALLOC_MULT(histentry_T *, max_len);
    new_hist = ALLOC_MULT(histentry_T, hislen);
    if (tot_hist == NULL || new_hist == NULL)
    {
	vim_free(tot_hist);
	vim_free(new_hist);
	return;
    }
    for (i = 0; i < viminfo_hisidx[type]; i++)
	tot_hist[i] = &viminfo_history[type][i];
    len = i;
    for (i = 0; i < hislen; i++)
	if (histentry[i].hisstr != NULL)
	    tot_hist[len++] = &histentry[i];

    // Sort the list on timestamp.
    qsort((void *)tot_hist, (size_t)len, sizeof(histentry_T *), sort_hist);

    // Keep the newest ones.
    for (i = 0; i < hislen; i++)
    {
	if (i < len)
	{
	    new_hist[i] = *tot_hist[i];
	    tot_hist[i]->hisstr = NULL;
	    if (new_hist[i].hisnum == 0)
		new_hist[i].hisnum = ++*hisnum;
	}
	else
	    clear_hist_entry(&new_hist[i]);
    }
    *hisidx = (i < len ? i : len) - 1;

    // Free what is not kept.
    for (i = 0; i < viminfo_hisidx[type]; i++)
	vim_free(viminfo_history[type][i].hisstr);
    for (i = 0; i < hislen; i++)
	vim_free(histentry[i].hisstr);
    vim_free(histentry);
    set_histentry(type, new_hist);
    vim_free(tot_hist);
}

/*
 * Finish reading history lines from viminfo.  Not used when writing viminfo.
 */
    static void
finish_viminfo_history(vir_T *virp)
{
    int	type;
    int merge = virp->vir_version >= VIMINFO_VERSION_WITH_HISTORY;

    for (type = 0; type < HIST_COUNT; ++type)
    {
	if (get_histentry(type) == NULL)
	    continue;

	if (merge)
	    merge_history(type);
	else
	    concat_history(type);

	VIM_CLEAR(viminfo_history[type]);
	viminfo_hisidx[type] = 0;
    }
}

/*
 * Write history to viminfo file in "fp".
 * When "merge" is TRUE merge history lines with a previously read viminfo
 * file, data is in viminfo_history[].
 * When "merge" is FALSE just write all history lines.  Used for ":wviminfo!".
 */
    static void
write_viminfo_history(FILE *fp, int merge)
{
    int	    i;
    int	    type;
    int	    num_saved;
    int     round;
    int	    hislen;

    init_history();
    hislen = get_hislen();
    if (hislen == 0)
	return;
    for (type = 0; type < HIST_COUNT; ++type)
    {
	histentry_T *histentry = get_histentry(type);
	int	    *hisidx = get_hisidx(type);

	num_saved = get_viminfo_parameter(hist_type2char(type, FALSE));
	if (num_saved == 0)
	    continue;
	if (num_saved < 0)  // Use default
	    num_saved = hislen;
	fprintf(fp, _("\n# %s History (newest to oldest):\n"),
			    type == HIST_CMD ? _("Command Line") :
			    type == HIST_SEARCH ? _("Search String") :
			    type == HIST_EXPR ? _("Expression") :
			    type == HIST_INPUT ? _("Input Line") :
					_("Debug Line"));
	if (num_saved > hislen)
	    num_saved = hislen;

	// Merge typed and viminfo history:
	// round 1: history of typed commands.
	// round 2: history from recently read viminfo.
	for (round = 1; round <= 2; ++round)
	{
	    if (round == 1)
		// start at newest entry, somewhere in the list
		i = *hisidx;
	    else if (viminfo_hisidx[type] > 0)
		// start at newest entry, first in the list
		i = 0;
	    else
		// empty list
		i = -1;
	    if (i >= 0)
		while (num_saved > 0
			&& !(round == 2 && i >= viminfo_hisidx[type]))
		{
		    char_u  *p;
		    time_t  timestamp;
		    int	    c = NUL;

		    if (round == 1)
		    {
			p = histentry[i].hisstr;
			timestamp = histentry[i].time_set;
		    }
		    else
		    {
			p = viminfo_history[type] == NULL ? NULL
					    : viminfo_history[type][i].hisstr;
			timestamp = viminfo_history[type] == NULL ? 0
					  : viminfo_history[type][i].time_set;
		    }

		    if (p != NULL && (round == 2
				       || !merge
				       || !histentry[i].viminfo))
		    {
			--num_saved;
			fputc(hist_type2char(type, TRUE), fp);
			// For the search history: put the separator in the
			// second column; use a space if there isn't one.
			if (type == HIST_SEARCH)
			{
			    c = p[STRLEN(p) + 1];
			    putc(c == NUL ? ' ' : c, fp);
			}
			viminfo_writestring(fp, p);

			{
			    char    cbuf[NUMBUFLEN];

			    // New style history with a bar line. Format:
			    // |{bartype},{histtype},{timestamp},{separator},"text"
			    if (c == NUL)
				cbuf[0] = NUL;
			    else
				sprintf(cbuf, "%d", c);
			    fprintf(fp, "|%d,%d,%ld,%s,", BARTYPE_HISTORY,
						 type, (long)timestamp, cbuf);
			    barline_writestring(fp, p, LSIZE - 20);
			    putc('\n', fp);
			}
		    }
		    if (round == 1)
		    {
			// Decrement index, loop around and stop when back at
			// the start.
			if (--i < 0)
			    i = hislen - 1;
			if (i == *hisidx)
			    break;
		    }
		    else
		    {
			// Increment index. Stop at the end in the while.
			++i;
		    }
		}
	}
	for (i = 0; i < viminfo_hisidx[type]; ++i)
	    if (viminfo_history[type] != NULL)
		vim_free(viminfo_history[type][i].hisstr);
	VIM_CLEAR(viminfo_history[type]);
	viminfo_hisidx[type] = 0;
    }
}

    static void
write_viminfo_barlines(vir_T *virp, FILE *fp_out)
{
    int		i;
    garray_T	*gap = &virp->vir_barlines;
    int		seen_useful = FALSE;
    char	*line;

    if (gap->ga_len <= 0)
	return;

    fputs(_("\n# Bar lines, copied verbatim:\n"), fp_out);

    // Skip over continuation lines until seeing a useful line.
    for (i = 0; i < gap->ga_len; ++i)
    {
	line = ((char **)(gap->ga_data))[i];
	if (seen_useful || line[1] != '<')
	{
	    fputs(line, fp_out);
	    seen_useful = TRUE;
	}
    }
}

/*
 * Parse a viminfo line starting with '|'.
 * Add each decoded value to "values".
 * Returns TRUE if the next line is to be read after using the parsed values.
 */
    static int
barline_parse(vir_T *virp, char_u *text, garray_T *values)
{
    char_u  *p = text;
    char_u  *nextp = NULL;
    char_u  *buf = NULL;
    bval_T  *value;
    int	    i;
    int	    allocated = FALSE;
    int	    eof;
    char_u  *sconv;
    int	    converted;

    while (*p == ',')
    {
	++p;
	if (ga_grow(values, 1) == FAIL)
	    break;
	value = (bval_T *)(values->ga_data) + values->ga_len;

	if (*p == '>')
	{
	    // Need to read a continuation line.  Put strings in allocated
	    // memory, because virp->vir_line is overwritten.
	    if (!allocated)
	    {
		for (i = 0; i < values->ga_len; ++i)
		{
		    bval_T  *vp = (bval_T *)(values->ga_data) + i;

		    if (vp->bv_type == BVAL_STRING && !vp->bv_allocated)
		    {
			vp->bv_string = vim_strnsave(vp->bv_string, vp->bv_len);
			vp->bv_allocated = TRUE;
		    }
		}
		allocated = TRUE;
	    }

	    if (vim_isdigit(p[1]))
	    {
		size_t len;
		size_t todo;
		size_t n;

		// String value was split into lines that are each shorter
		// than LSIZE:
		//     |{bartype},>{length of "{text}{text2}"}
		//     |<"{text1}
		//     |<{text2}",{value}
		// Length includes the quotes.
		++p;
		len = getdigits(&p);
		buf = alloc((int)(len + 1));
		if (buf == NULL)
		    return TRUE;
		p = buf;
		for (todo = len; todo > 0; todo -= n)
		{
		    eof = viminfo_readline(virp);
		    if (eof || virp->vir_line[0] != '|'
						  || virp->vir_line[1] != '<')
		    {
			// File was truncated or garbled. Read another line if
			// this one starts with '|'.
			vim_free(buf);
			return eof || virp->vir_line[0] == '|';
		    }
		    // Get length of text, excluding |< and NL chars.
		    n = STRLEN(virp->vir_line);
		    while (n > 0 && (virp->vir_line[n - 1] == NL
					     || virp->vir_line[n - 1] == CAR))
			--n;
		    n -= 2;
		    if (n > todo)
		    {
			// more values follow after the string
			nextp = virp->vir_line + 2 + todo;
			n = todo;
		    }
		    mch_memmove(p, virp->vir_line + 2, n);
		    p += n;
		}
		*p = NUL;
		p = buf;
	    }
	    else
	    {
		// Line ending in ">" continues in the next line:
		//     |{bartype},{lots of values},>
		//     |<{value},{value}
		eof = viminfo_readline(virp);
		if (eof || virp->vir_line[0] != '|'
					      || virp->vir_line[1] != '<')
		    // File was truncated or garbled. Read another line if
		    // this one starts with '|'.
		    return eof || virp->vir_line[0] == '|';
		p = virp->vir_line + 2;
	    }
	}

	if (SAFE_isdigit(*p))
	{
	    value->bv_type = BVAL_NR;
	    value->bv_nr = getdigits(&p);
	    ++values->ga_len;
	}
	else if (*p == '"')
	{
	    int	    len = 0;
	    char_u  *s = p;

	    // Unescape special characters in-place.
	    ++p;
	    while (*p != '"')
	    {
		if (*p == NL || *p == NUL)
		    return TRUE;  // syntax error, drop the value
		if (*p == '\\')
		{
		    ++p;
		    if (*p == 'n')
			s[len++] = '\n';
		    else
			s[len++] = *p;
		    ++p;
		}
		else
		    s[len++] = *p++;
	    }
	    ++p;
	    s[len] = NUL;

	    converted = FALSE;
	    value->bv_tofree = NULL;
	    if (virp->vir_conv.vc_type != CONV_NONE && *s != NUL)
	    {
		sconv = string_convert(&virp->vir_conv, s, NULL);
		if (sconv != NULL)
		{
		    if (s == buf)
			// the converted string is stored in bv_string and
			// freed later, also need to free "buf" later
			value->bv_tofree = buf;
		    s = sconv;
		    converted = TRUE;
		}
	    }

	    // Need to copy in allocated memory if the string wasn't allocated
	    // above and we did allocate before, thus vir_line may change.
	    if (s != buf && allocated && !converted)
		s = vim_strsave(s);
	    value->bv_string = s;
	    value->bv_type = BVAL_STRING;
	    value->bv_len = len;
	    value->bv_allocated = allocated || converted;
	    ++values->ga_len;
	    if (nextp != NULL)
	    {
		// values following a long string
		p = nextp;
		nextp = NULL;
	    }
	}
	else if (*p == ',')
	{
	    value->bv_type = BVAL_EMPTY;
	    ++values->ga_len;
	}
	else
	    break;
    }
    return TRUE;
}

    static void
write_viminfo_version(FILE *fp_out)
{
    fprintf(fp_out, "# Viminfo version\n|%d,%d\n\n",
					    BARTYPE_VERSION, VIMINFO_VERSION);
}

    static int
no_viminfo(void)
{
    // "vim -i NONE" does not read or write a viminfo file
    return STRCMP(p_viminfofile, "NONE") == 0;
}

/*
 * Report an error for reading a viminfo file.
 * Count the number of errors.	When there are more than 10, return TRUE.
 */
    static int
viminfo_error(char *errnum, char *message, char_u *line)
{
    vim_snprintf((char *)IObuff, IOSIZE, _("%sviminfo: %s in line: "),
							     errnum, message);
    STRNCAT(IObuff, line, IOSIZE - STRLEN(IObuff) - 1);
    if (IObuff[STRLEN(IObuff) - 1] == '\n')
	IObuff[STRLEN(IObuff) - 1] = NUL;
    emsg((char *)IObuff);
    if (++viminfo_errcnt >= 10)
    {
	emsg(_(e_viminfo_too_many_errors_skipping_rest_of_file));
	return TRUE;
    }
    return FALSE;
}

/*
 * Compare the 'encoding' value in the viminfo file with the current value of
 * 'encoding'.  If different and the 'c' flag is in 'viminfo', setup for
 * conversion of text with iconv() in viminfo_readstring().
 */
    static int
viminfo_encoding(vir_T *virp)
{
    char_u	*p;
    int		i;

    if (get_viminfo_parameter('c') != 0)
    {
	p = vim_strchr(virp->vir_line, '=');
	if (p != NULL)
	{
	    // remove trailing newline
	    ++p;
	    for (i = 0; vim_isprintc(p[i]); ++i)
		;
	    p[i] = NUL;

	    convert_setup(&virp->vir_conv, p, p_enc);
	}
    }
    return viminfo_readline(virp);
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Restore global vars that start with a capital from the viminfo file
 */
    static int
read_viminfo_varlist(vir_T *virp, int writing)
{
    char_u	*tab;
    int		type = VAR_NUMBER;
    typval_T	tv;
    funccal_entry_T funccal_entry;

    if (!writing && (find_viminfo_parameter('!') != NULL))
    {
	tab = vim_strchr(virp->vir_line + 1, '\t');
	if (tab != NULL)
	{
	    *tab++ = '\0';	// isolate the variable name
	    switch (*tab)
	    {
		case 'S': type = VAR_STRING; break;
		case 'F': type = VAR_FLOAT; break;
		case 'D': type = VAR_DICT; break;
		case 'L': type = VAR_LIST; break;
		case 'B': type = VAR_BLOB; break;
		case 'X': type = VAR_SPECIAL; break;
	    }

	    tab = vim_strchr(tab, '\t');
	    if (tab != NULL)
	    {
		tv.v_type = type;
		if (type == VAR_STRING || type == VAR_DICT
			|| type == VAR_LIST || type == VAR_BLOB)
		    tv.vval.v_string = viminfo_readstring(virp,
				       (int)(tab - virp->vir_line + 1), TRUE);
		else if (type == VAR_FLOAT)
		    (void)string2float(tab + 1, &tv.vval.v_float, FALSE);
		else
		{
		    tv.vval.v_number = atol((char *)tab + 1);
		    if (type == VAR_SPECIAL && (tv.vval.v_number == VVAL_FALSE
					     || tv.vval.v_number == VVAL_TRUE))
			tv.v_type = VAR_BOOL;
		}
		if (type == VAR_DICT || type == VAR_LIST)
		{
		    typval_T *etv = eval_expr(tv.vval.v_string, NULL);

		    if (etv == NULL)
			// Failed to parse back the dict or list, use it as a
			// string.
			tv.v_type = VAR_STRING;
		    else
		    {
			vim_free(tv.vval.v_string);
			tv = *etv;
			vim_free(etv);
		    }
		}
		else if (type == VAR_BLOB)
		{
		    blob_T *blob = string2blob(tv.vval.v_string);

		    if (blob == NULL)
			// Failed to parse back the blob, use it as a string.
			tv.v_type = VAR_STRING;
		    else
		    {
			vim_free(tv.vval.v_string);
			tv.v_type = VAR_BLOB;
			tv.vval.v_blob = blob;
		    }
		}

		// when in a function use global variables
		save_funccal(&funccal_entry);
		set_var(virp->vir_line + 1, &tv, FALSE);
		restore_funccal();

		if (tv.v_type == VAR_STRING)
		    vim_free(tv.vval.v_string);
		else if (tv.v_type == VAR_DICT || tv.v_type == VAR_LIST ||
			tv.v_type == VAR_BLOB)
		    clear_tv(&tv);
	    }
	}
    }

    return viminfo_readline(virp);
}

/*
 * Write global vars that start with a capital to the viminfo file
 */
    static void
write_viminfo_varlist(FILE *fp)
{
    hashtab_T	*gvht = get_globvar_ht();
    hashitem_T	*hi;
    dictitem_T	*this_var;
    int		todo;
    char	*s = "";
    char_u	*p;
    char_u	*tofree;
    char_u	numbuf[NUMBUFLEN];

    if (find_viminfo_parameter('!') == NULL)
	return;

    fputs(_("\n# global variables:\n"), fp);

    todo = (int)gvht->ht_used;
    FOR_ALL_HASHTAB_ITEMS(gvht, hi, todo)
    {
	if (!HASHITEM_EMPTY(hi))
	{
	    --todo;
	    this_var = HI2DI(hi);
	    if (var_flavour(this_var->di_key) == VAR_FLAVOUR_VIMINFO)
	    {
		switch (this_var->di_tv.v_type)
		{
		    case VAR_STRING:  s = "STR"; break;
		    case VAR_NUMBER:  s = "NUM"; break;
		    case VAR_FLOAT:   s = "FLO"; break;
		    case VAR_DICT:
			  {
			      dict_T	*di = this_var->di_tv.vval.v_dict;
			      int	copyID = get_copyID();

			      s = "DIC";
			      if (di != NULL && !set_ref_in_ht(
						 &di->dv_hashtab, copyID, NULL)
				      && di->dv_copyID == copyID)
				  // has a circular reference, can't turn the
				  // value into a string
				  continue;
			      break;
			  }
		    case VAR_LIST:
			  {
			      list_T	*l = this_var->di_tv.vval.v_list;
			      int	copyID = get_copyID();

			      s = "LIS";
			      if (l != NULL && !set_ref_in_list_items(
							       l, copyID, NULL)
				      && l->lv_copyID == copyID)
				  // has a circular reference, can't turn the
				  // value into a string
				  continue;
			      break;
			  }
		    case VAR_BLOB:    s = "BLO"; break;
		    case VAR_BOOL:    s = "XPL"; break;  // backwards compat.
		    case VAR_SPECIAL: s = "XPL"; break;

		    case VAR_UNKNOWN:
		    case VAR_ANY:
		    case VAR_VOID:
		    case VAR_FUNC:
		    case VAR_PARTIAL:
		    case VAR_JOB:
		    case VAR_CHANNEL:
		    case VAR_INSTR:
		    case VAR_CLASS:
		    case VAR_OBJECT:
		    case VAR_TYPEALIAS:
				      continue;
		}
		fprintf(fp, "!%s\t%s\t", this_var->di_key, s);
		if (this_var->di_tv.v_type == VAR_BOOL
				      || this_var->di_tv.v_type == VAR_SPECIAL)
		{
		    // do not use "v:true" but "1"
		    sprintf((char *)numbuf, "%ld",
					  (long)this_var->di_tv.vval.v_number);
		    p = numbuf;
		    tofree = NULL;
		}
		else
		    p = echo_string(&this_var->di_tv, &tofree, numbuf, 0);
		if (p != NULL)
		    viminfo_writestring(fp, p);
		vim_free(tofree);
	    }
	}
    }
}
#endif // FEAT_EVAL

    static int
read_viminfo_sub_string(vir_T *virp, int force)
{
    if (force || get_old_sub() == NULL)
	set_old_sub(viminfo_readstring(virp, 1, TRUE));
    return viminfo_readline(virp);
}

    static void
write_viminfo_sub_string(FILE *fp)
{
    char_u *old_sub = get_old_sub();

    if (get_viminfo_parameter('/') == 0 || old_sub == NULL)
	return;

    fputs(_("\n# Last Substitute String:\n$"), fp);
    viminfo_writestring(fp, old_sub);
}

/*
 * Functions relating to reading/writing the search pattern from viminfo
 */

    static int
read_viminfo_search_pattern(vir_T *virp, int force)
{
    char_u	*lp;
    int		idx = -1;
    int		magic = FALSE;
    int		no_scs = FALSE;
    int		off_line = FALSE;
    int		off_end = 0;
    long	off = 0;
    int		setlast = FALSE;
#ifdef FEAT_SEARCH_EXTRA
    static int	hlsearch_on = FALSE;
#endif
    char_u	*val;
    spat_T	*spat;

    // Old line types:
    // "/pat", "&pat": search/subst. pat
    // "~/pat", "~&pat": last used search/subst. pat
    // New line types:
    // "~h", "~H": hlsearch highlighting off/on
    // "~<magic><smartcase><line><end><off><last><which>pat"
    // <magic>: 'm' off, 'M' on
    // <smartcase>: 's' off, 'S' on
    // <line>: 'L' line offset, 'l' char offset
    // <end>: 'E' from end, 'e' from start
    // <off>: decimal, offset
    // <last>: '~' last used pattern
    // <which>: '/' search pat, '&' subst. pat
    lp = virp->vir_line;
    if (lp[0] == '~' && (lp[1] == 'm' || lp[1] == 'M'))	// new line type
    {
	if (lp[1] == 'M')		// magic on
	    magic = TRUE;
	if (lp[2] == 's')
	    no_scs = TRUE;
	if (lp[3] == 'L')
	    off_line = TRUE;
	if (lp[4] == 'E')
	    off_end = SEARCH_END;
	lp += 5;
	off = getdigits(&lp);
    }
    if (lp[0] == '~')		// use this pattern for last-used pattern
    {
	setlast = TRUE;
	lp++;
    }
    if (lp[0] == '/')
	idx = RE_SEARCH;
    else if (lp[0] == '&')
	idx = RE_SUBST;
#ifdef FEAT_SEARCH_EXTRA
    else if (lp[0] == 'h')	// ~h: 'hlsearch' highlighting off
	hlsearch_on = FALSE;
    else if (lp[0] == 'H')	// ~H: 'hlsearch' highlighting on
	hlsearch_on = TRUE;
#endif
    if (idx >= 0)
    {
	spat = get_spat(idx);
	if (force || spat->pat == NULL)
	{
	    val = viminfo_readstring(virp, (int)(lp - virp->vir_line + 1),
									TRUE);
	    if (val != NULL)
	    {
		set_last_search_pat(val, idx, magic, setlast);
		vim_free(val);
		spat->no_scs = no_scs;
		spat->off.line = off_line;
		spat->off.end = off_end;
		spat->off.off = off;
#ifdef FEAT_SEARCH_EXTRA
		if (setlast)
		    set_no_hlsearch(!hlsearch_on);
#endif
	    }
	}
    }
    return viminfo_readline(virp);
}

    static void
wvsp_one(
    FILE	*fp,	// file to write to
    int		idx,	// spats[] index
    char	*s,	// search pat
    int		sc)	// dir char
{
    spat_T	*spat = get_spat(idx);
    if (spat->pat == NULL)
	return;

    fprintf(fp, _("\n# Last %sSearch Pattern:\n~"), s);
    // off.dir is not stored, it's reset to forward
    fprintf(fp, "%c%c%c%c%ld%s%c",
	    spat->magic    ? 'M' : 'm',	// magic
	    spat->no_scs   ? 's' : 'S',	// smartcase
	    spat->off.line ? 'L' : 'l',	// line offset
	    spat->off.end  ? 'E' : 'e',	// offset from end
	    spat->off.off,			// offset
	    get_spat_last_idx() == idx ? "~" : "",	// last used pat
	    sc);
    viminfo_writestring(fp, spat->pat);
}

    static void
write_viminfo_search_pattern(FILE *fp)
{
    if (get_viminfo_parameter('/') == 0)
	return;

#ifdef FEAT_SEARCH_EXTRA
    fprintf(fp, "\n# hlsearch on (H) or off (h):\n~%c",
	    (no_hlsearch || find_viminfo_parameter('h') != NULL) ? 'h' : 'H');
#endif
    wvsp_one(fp, RE_SEARCH, "", '/');
    wvsp_one(fp, RE_SUBST, _("Substitute "), '&');
}

/*
 * Functions relating to reading/writing registers from viminfo
 */

static yankreg_T *y_read_regs = NULL;

#define REG_PREVIOUS 1
#define REG_EXEC 2

/*
 * Prepare for reading viminfo registers when writing viminfo later.
 */
    static void
prepare_viminfo_registers(void)
{
     y_read_regs = ALLOC_CLEAR_MULT(yankreg_T, NUM_REGISTERS);
}

    static void
finish_viminfo_registers(void)
{
    int		i;
    int		j;

    if (y_read_regs == NULL)
	return;

    for (i = 0; i < NUM_REGISTERS; ++i)
	if (y_read_regs[i].y_array != NULL)
	{
	    for (j = 0; j < y_read_regs[i].y_size; j++)
		vim_free(y_read_regs[i].y_array[j]);
	    vim_free(y_read_regs[i].y_array);
	}
    VIM_CLEAR(y_read_regs);
}

    static int
read_viminfo_register(vir_T *virp, int force)
{
    int		eof;
    int		do_it = TRUE;
    int		size;
    int		limit;
    int		i;
    int		set_prev = FALSE;
    char_u	*str;
    char_u	**array = NULL;
    int		new_type = MCHAR; // init to shut up compiler
    colnr_T	new_width = 0; // init to shut up compiler
    yankreg_T	*y_current_p;

    // We only get here (hopefully) if line[0] == '"'
    str = virp->vir_line + 1;

    // If the line starts with "" this is the y_previous register.
    if (*str == '"')
    {
	set_prev = TRUE;
	str++;
    }

    if (!ASCII_ISALNUM(*str) && *str != '-')
    {
	if (viminfo_error("E577: ", _(e_illegal_register_name), virp->vir_line))
	    return TRUE;	// too many errors, pretend end-of-file
	do_it = FALSE;
    }
    get_yank_register(*str++, FALSE);
    y_current_p = get_y_current();
    if (!force && y_current_p->y_array != NULL)
	do_it = FALSE;

    if (*str == '@')
    {
	// "x@: register x used for @@
	if (force || get_execreg_lastc() == NUL)
	    set_execreg_lastc(str[-1]);
    }

    size = 0;
    limit = 100;	// Optimized for registers containing <= 100 lines
    if (do_it)
    {
	// Build the new register in array[].
	// y_array is kept as-is until done.
	// The "do_it" flag is reset when something is wrong, in which case
	// array[] needs to be freed.
	if (set_prev)
	    set_y_previous(y_current_p);
	array = ALLOC_MULT(char_u *, limit);
	str = skipwhite(skiptowhite(str));
	if (STRNCMP(str, "CHAR", 4) == 0)
	    new_type = MCHAR;
	else if (STRNCMP(str, "BLOCK", 5) == 0)
	    new_type = MBLOCK;
	else
	    new_type = MLINE;
	// get the block width; if it's missing we get a zero, which is OK
	str = skipwhite(skiptowhite(str));
	new_width = getdigits(&str);
    }

    while (!(eof = viminfo_readline(virp))
		    && (virp->vir_line[0] == TAB || virp->vir_line[0] == '<'))
    {
	if (do_it)
	{
	    if (size == limit)
	    {
		char_u **new_array = (char_u **)
					   alloc(limit * 2 * sizeof(char_u *));

		if (new_array == NULL)
		{
		    do_it = FALSE;
		    break;
		}
		for (i = 0; i < limit; i++)
		    new_array[i] = array[i];
		vim_free(array);
		array = new_array;
		limit *= 2;
	    }
	    str = viminfo_readstring(virp, 1, TRUE);
	    if (str != NULL)
		array[size++] = str;
	    else
		// error, don't store the result
		do_it = FALSE;
	}
    }

    if (do_it)
    {
	// free y_array[]
	for (i = 0; i < y_current_p->y_size; i++)
	    vim_free(y_current_p->y_array[i]);
	vim_free(y_current_p->y_array);

	y_current_p->y_type = new_type;
	y_current_p->y_width = new_width;
	y_current_p->y_size = size;
	y_current_p->y_time_set = 0;
	if (size == 0)
	{
	    y_current_p->y_array = NULL;
	}
	else
	{
	    // Move the lines from array[] to y_array[].
	    y_current_p->y_array = ALLOC_MULT(char_u *, size);
	    for (i = 0; i < size; i++)
	    {
		if (y_current_p->y_array == NULL)
		    vim_free(array[i]);
		else
		    y_current_p->y_array[i] = array[i];
	    }
	}
    }
    else
    {
	// Free array[] if it was filled.
	for (i = 0; i < size; i++)
	    vim_free(array[i]);
    }
    vim_free(array);

    return eof;
}

/*
 * Accept a new style register line from the viminfo, store it when it's new.
 */
    static void
handle_viminfo_register(garray_T *values, int force)
{
    bval_T	*vp = (bval_T *)values->ga_data;
    int		flags;
    int		name;
    int		type;
    int		linecount;
    int		width;
    time_t	timestamp;
    yankreg_T	*y_ptr;
    yankreg_T	*y_regs_p = get_y_regs();
    int		i;

    // Check the format:
    // |{bartype},{flags},{name},{type},
    //      {linecount},{width},{timestamp},"line1","line2"
    if (values->ga_len < 6
	    || vp[0].bv_type != BVAL_NR
	    || vp[1].bv_type != BVAL_NR
	    || vp[2].bv_type != BVAL_NR
	    || vp[3].bv_type != BVAL_NR
	    || vp[4].bv_type != BVAL_NR
	    || vp[5].bv_type != BVAL_NR)
	return;
    flags = vp[0].bv_nr;
    name = vp[1].bv_nr;
    if (name < 0 || name >= NUM_REGISTERS)
	return;
    type = vp[2].bv_nr;
    if (type != MCHAR && type != MLINE && type != MBLOCK)
	return;
    linecount = vp[3].bv_nr;
    if (values->ga_len < 6 + linecount)
	return;
    width = vp[4].bv_nr;
    if (width < 0)
	return;

    if (y_read_regs != NULL)
	// Reading viminfo for merging and writing.  Store the register
	// content, don't update the current registers.
	y_ptr = &y_read_regs[name];
    else
	y_ptr = &y_regs_p[name];

    // Do not overwrite unless forced or the timestamp is newer.
    timestamp = (time_t)vp[5].bv_nr;
    if (y_ptr->y_array != NULL && !force
			 && (timestamp == 0 || y_ptr->y_time_set > timestamp))
	return;

    if (y_ptr->y_array != NULL)
	for (i = 0; i < y_ptr->y_size; i++)
	    vim_free(y_ptr->y_array[i]);
    vim_free(y_ptr->y_array);

    if (y_read_regs == NULL)
    {
	if (flags & REG_PREVIOUS)
	    set_y_previous(y_ptr);
	if ((flags & REG_EXEC) && (force || get_execreg_lastc() == NUL))
	    set_execreg_lastc(get_register_name(name));
    }
    y_ptr->y_type = type;
    y_ptr->y_width = width;
    y_ptr->y_size = linecount;
    y_ptr->y_time_set = timestamp;
    if (linecount == 0)
    {
	y_ptr->y_array = NULL;
	return;
    }
    y_ptr->y_array = ALLOC_MULT(char_u *, linecount);
    if (y_ptr->y_array == NULL)
    {
	y_ptr->y_size = 0; // ensure object state is consistent
	return;
    }
    for (i = 0; i < linecount; i++)
    {
	if (vp[i + 6].bv_allocated)
	{
	    y_ptr->y_array[i] = vp[i + 6].bv_string;
	    vp[i + 6].bv_string = NULL;
	}
	else if (vp[i + 6].bv_type != BVAL_STRING)
	{
	    free(y_ptr->y_array);
	    y_ptr->y_array = NULL;
	}
	else
	    y_ptr->y_array[i] = vim_strsave(vp[i + 6].bv_string);
    }
}

    static void
write_viminfo_registers(FILE *fp)
{
    int		i, j;
    char_u	*type;
    char_u	c;
    int		num_lines;
    int		max_num_lines;
    int		max_kbyte;
    long	len;
    yankreg_T	*y_ptr;
    yankreg_T	*y_regs_p = get_y_regs();;

    fputs(_("\n# Registers:\n"), fp);

    // Get '<' value, use old '"' value if '<' is not found.
    max_num_lines = get_viminfo_parameter('<');
    if (max_num_lines < 0)
	max_num_lines = get_viminfo_parameter('"');
    if (max_num_lines == 0)
	return;
    max_kbyte = get_viminfo_parameter('s');
    if (max_kbyte == 0)
	return;

    for (i = 0; i < NUM_REGISTERS; i++)
    {
#ifdef FEAT_CLIPBOARD
	// Skip '*'/'+' register, we don't want them back next time
	if (i == STAR_REGISTER || i == PLUS_REGISTER)
	    continue;
#endif
#ifdef FEAT_DND
	// Neither do we want the '~' register
	if (i == TILDE_REGISTER)
	    continue;
#endif
	// When reading viminfo for merging and writing: Use the register from
	// viminfo if it's newer.
	if (y_read_regs != NULL
		&& y_read_regs[i].y_array != NULL
		&& (y_regs_p[i].y_array == NULL ||
			    y_read_regs[i].y_time_set > y_regs_p[i].y_time_set))
	    y_ptr = &y_read_regs[i];
	else if (y_regs_p[i].y_array == NULL)
	    continue;
	else
	    y_ptr = &y_regs_p[i];

	// Skip empty registers.
	num_lines = y_ptr->y_size;
	if (num_lines == 0
		|| (num_lines == 1 && y_ptr->y_type == MCHAR
					&& *y_ptr->y_array[0] == NUL))
	    continue;

	if (max_kbyte > 0)
	{
	    // Skip register if there is more text than the maximum size.
	    len = 0;
	    for (j = 0; j < num_lines; j++)
		len += (long)STRLEN(y_ptr->y_array[j]) + 1L;
	    if (len > (long)max_kbyte * 1024L)
		continue;
	}

	switch (y_ptr->y_type)
	{
	    case MLINE:
		type = (char_u *)"LINE";
		break;
	    case MCHAR:
		type = (char_u *)"CHAR";
		break;
	    case MBLOCK:
		type = (char_u *)"BLOCK";
		break;
	    default:
		semsg(_(e_unknown_register_type_nr), y_ptr->y_type);
		type = (char_u *)"LINE";
		break;
	}
	if (get_y_previous() == &y_regs_p[i])
	    fprintf(fp, "\"");
	c = get_register_name(i);
	fprintf(fp, "\"%c", c);
	if (c == get_execreg_lastc())
	    fprintf(fp, "@");
	fprintf(fp, "\t%s\t%d\n", type, (int)y_ptr->y_width);

	// If max_num_lines < 0, then we save ALL the lines in the register
	if (max_num_lines > 0 && num_lines > max_num_lines)
	    num_lines = max_num_lines;
	for (j = 0; j < num_lines; j++)
	{
	    putc('\t', fp);
	    viminfo_writestring(fp, y_ptr->y_array[j]);
	}

	{
	    int	    flags = 0;
	    int	    remaining;

	    // New style with a bar line. Format:
	    // |{bartype},{flags},{name},{type},
	    //      {linecount},{width},{timestamp},"line1","line2"
	    // flags: REG_PREVIOUS - register is y_previous
	    //	      REG_EXEC - used for @@
	    if (get_y_previous() == &y_regs_p[i])
		flags |= REG_PREVIOUS;
	    if (c == get_execreg_lastc())
		flags |= REG_EXEC;
	    fprintf(fp, "|%d,%d,%d,%d,%d,%d,%ld", BARTYPE_REGISTER, flags,
		    i, y_ptr->y_type, num_lines, (int)y_ptr->y_width,
		    (long)y_ptr->y_time_set);
	    // 11 chars for type/flags/name/type, 3 * 20 for numbers
	    remaining = LSIZE - 71;
	    for (j = 0; j < num_lines; j++)
	    {
		putc(',', fp);
		--remaining;
		remaining = barline_writestring(fp, y_ptr->y_array[j],
								   remaining);
	    }
	    putc('\n', fp);
	}
    }
}

/*
 * Functions relating to reading/writing marks from viminfo
 */

static xfmark_T *vi_namedfm = NULL;
static xfmark_T *vi_jumplist = NULL;
static int vi_jumplist_len = 0;

    static void
write_one_mark(FILE *fp_out, int c, pos_T *pos)
{
    if (pos->lnum != 0)
	fprintf(fp_out, "\t%c\t%ld\t%d\n", c, (long)pos->lnum, (int)pos->col);
}

    static void
write_buffer_marks(buf_T *buf, FILE *fp_out)
{
    int		i;
    pos_T	pos;

    home_replace(NULL, buf->b_ffname, IObuff, IOSIZE, TRUE);
    fprintf(fp_out, "\n> ");
    viminfo_writestring(fp_out, IObuff);

    // Write the last used timestamp as the lnum of the non-existing mark '*'.
    // Older Vims will ignore it and/or copy it.
    pos.lnum = (linenr_T)buf->b_last_used;
    pos.col = 0;
    write_one_mark(fp_out, '*', &pos);

    write_one_mark(fp_out, '"', &buf->b_last_cursor);
    write_one_mark(fp_out, '^', &buf->b_last_insert);
    write_one_mark(fp_out, '.', &buf->b_last_change);
    // changelist positions are stored oldest first
    for (i = 0; i < buf->b_changelistlen; ++i)
    {
	// skip duplicates
	if (i == 0 || !EQUAL_POS(buf->b_changelist[i - 1],
							 buf->b_changelist[i]))
	    write_one_mark(fp_out, '+', &buf->b_changelist[i]);
    }
    for (i = 0; i < NMARKS; i++)
	write_one_mark(fp_out, 'a' + i, &buf->b_namedm[i]);
}

/*
 * Return TRUE if marks for "buf" should not be written.
 */
    static int
skip_for_viminfo(buf_T *buf)
{
    return bt_terminal(buf) || removable(buf->b_ffname);
}

/*
 * Write all the named marks for all buffers.
 * When "buflist" is not NULL fill it with the buffers for which marks are to
 * be written.
 */
    static void
write_viminfo_marks(FILE *fp_out, garray_T *buflist)
{
    buf_T	*buf;
    int		is_mark_set;
    int		i;
    win_T	*win;
    tabpage_T	*tp;

    // Set b_last_cursor for the all buffers that have a window.
    FOR_ALL_TAB_WINDOWS(tp, win)
	set_last_cursor(win);

    fputs(_("\n# History of marks within files (newest to oldest):\n"), fp_out);
    FOR_ALL_BUFFERS(buf)
    {
	// Only write something if buffer has been loaded and at least one
	// mark is set.
	if (buf->b_marks_read)
	{
	    if (buf->b_last_cursor.lnum != 0)
		is_mark_set = TRUE;
	    else
	    {
		is_mark_set = FALSE;
		for (i = 0; i < NMARKS; i++)
		    if (buf->b_namedm[i].lnum != 0)
		    {
			is_mark_set = TRUE;
			break;
		    }
	    }
	    if (is_mark_set && buf->b_ffname != NULL
		      && buf->b_ffname[0] != NUL
		      && !skip_for_viminfo(buf))
	    {
		if (buflist == NULL)
		    write_buffer_marks(buf, fp_out);
		else if (ga_grow(buflist, 1) == OK)
		    ((buf_T **)buflist->ga_data)[buflist->ga_len++] = buf;
	    }
	}
    }
}

    static void
write_one_filemark(
    FILE	*fp,
    xfmark_T	*fm,
    int		c1,
    int		c2)
{
    char_u	*name;

    if (fm->fmark.mark.lnum == 0)	// not set
	return;

    if (fm->fmark.fnum != 0)		// there is a buffer
	name = buflist_nr2name(fm->fmark.fnum, TRUE, FALSE);
    else
	name = fm->fname;		// use name from .viminfo
    if (name != NULL && *name != NUL)
    {
	fprintf(fp, "%c%c  %ld  %ld  ", c1, c2, (long)fm->fmark.mark.lnum,
						    (long)fm->fmark.mark.col);
	viminfo_writestring(fp, name);

	// Barline: |{bartype},{name},{lnum},{col},{timestamp},{filename}
	// size up to filename: 8 + 3 * 20
	fprintf(fp, "|%d,%d,%ld,%ld,%ld,", BARTYPE_MARK, c2,
		(long)fm->fmark.mark.lnum, (long)fm->fmark.mark.col,
		(long)fm->time_set);
	barline_writestring(fp, name, LSIZE - 70);
	putc('\n', fp);
    }

    if (fm->fmark.fnum != 0)
	vim_free(name);
}

    static void
write_viminfo_filemarks(FILE *fp)
{
    int		i;
    char_u	*name;
    buf_T	*buf;
    xfmark_T	*namedfm_p = get_namedfm();
    xfmark_T	*fm;
    int		vi_idx;
    int		idx;

    if (get_viminfo_parameter('f') == 0)
	return;

    fputs(_("\n# File marks:\n"), fp);

    // Write the filemarks 'A - 'Z
    for (i = 0; i < NMARKS; i++)
    {
	if (vi_namedfm != NULL
			&& (vi_namedfm[i].time_set > namedfm_p[i].time_set))
	    fm = &vi_namedfm[i];
	else
	    fm = &namedfm_p[i];
	write_one_filemark(fp, fm, '\'', i + 'A');
    }

    // Find a mark that is the same file and position as the cursor.
    // That one, or else the last one is deleted.
    // Move '0 to '1, '1 to '2, etc. until the matching one or '9
    // Set the '0 mark to current cursor position.
    if (curbuf->b_ffname != NULL && !skip_for_viminfo(curbuf))
    {
	name = buflist_nr2name(curbuf->b_fnum, TRUE, FALSE);
	for (i = NMARKS; i < NMARKS + EXTRA_MARKS - 1; ++i)
	    if (namedfm_p[i].fmark.mark.lnum == curwin->w_cursor.lnum
		    && (namedfm_p[i].fname == NULL
			    ? namedfm_p[i].fmark.fnum == curbuf->b_fnum
			    : (name != NULL
				    && STRCMP(name, namedfm_p[i].fname) == 0)))
		break;
	vim_free(name);

	vim_free(namedfm_p[i].fname);
	for ( ; i > NMARKS; --i)
	    namedfm_p[i] = namedfm_p[i - 1];
	namedfm_p[NMARKS].fmark.mark = curwin->w_cursor;
	namedfm_p[NMARKS].fmark.fnum = curbuf->b_fnum;
	namedfm_p[NMARKS].fname = NULL;
	namedfm_p[NMARKS].time_set = vim_time();
    }

    // Write the filemarks '0 - '9.  Newest (highest timestamp) first.
    vi_idx = NMARKS;
    idx = NMARKS;
    for (i = NMARKS; i < NMARKS + EXTRA_MARKS; i++)
    {
	xfmark_T *vi_fm = vi_namedfm != NULL ? &vi_namedfm[vi_idx] : NULL;

	if (vi_fm != NULL
		&& vi_fm->fmark.mark.lnum != 0
		&& (vi_fm->time_set > namedfm_p[idx].time_set
		    || namedfm_p[idx].fmark.mark.lnum == 0))
	{
	    fm = vi_fm;
	    ++vi_idx;
	}
	else
	{
	    fm = &namedfm_p[idx++];
	    if (vi_fm != NULL
		  && vi_fm->fmark.mark.lnum == fm->fmark.mark.lnum
		  && vi_fm->time_set == fm->time_set
		  && ((vi_fm->fmark.fnum != 0
			  && vi_fm->fmark.fnum == fm->fmark.fnum)
		      || (vi_fm->fname != NULL
			  && fm->fname != NULL
			  && STRCMP(vi_fm->fname, fm->fname) == 0)))
		++vi_idx;  // skip duplicate
	}
	write_one_filemark(fp, fm, '\'', i - NMARKS + '0');
    }

    // Write the jumplist with -'
    fputs(_("\n# Jumplist (newest first):\n"), fp);
    setpcmark();	// add current cursor position
    cleanup_jumplist(curwin, FALSE);
    vi_idx = 0;
    idx = curwin->w_jumplistlen - 1;
    for (i = 0; i < JUMPLISTSIZE; ++i)
    {
	xfmark_T	*vi_fm;

	fm = idx >= 0 ? &curwin->w_jumplist[idx] : NULL;
	vi_fm = (vi_jumplist != NULL && vi_idx < vi_jumplist_len)
					? &vi_jumplist[vi_idx] : NULL;
	if (fm == NULL && vi_fm == NULL)
	    break;
	if (fm == NULL || (vi_fm != NULL && fm->time_set < vi_fm->time_set))
	{
	    fm = vi_fm;
	    ++vi_idx;
	}
	else
	    --idx;
	if (fm->fmark.fnum == 0
		|| ((buf = buflist_findnr(fm->fmark.fnum)) != NULL
		    && !skip_for_viminfo(buf)))
	    write_one_filemark(fp, fm, '-', '\'');
    }
}

/*
 * Compare functions for qsort() below, that compares b_last_used.
 */
    int
buf_compare(const void *s1, const void *s2)
{
    buf_T *buf1 = *(buf_T **)s1;
    buf_T *buf2 = *(buf_T **)s2;

    if (buf1->b_last_used == buf2->b_last_used)
	return 0;
    return buf1->b_last_used > buf2->b_last_used ? -1 : 1;
}

/*
 * Handle marks in the viminfo file:
 * fp_out != NULL: copy marks, in time order with buffers in "buflist".
 * fp_out == NULL && (flags & VIF_WANT_MARKS): read marks for curbuf
 * fp_out == NULL && (flags & VIF_ONLY_CURBUF): bail out after curbuf marks
 * fp_out == NULL && (flags & VIF_GET_OLDFILES | VIF_FORCEIT): fill v:oldfiles
 */
    static void
copy_viminfo_marks(
    vir_T	*virp,
    FILE	*fp_out,
    garray_T	*buflist,
    int		eof,
    int		flags)
{
    char_u	*line = virp->vir_line;
    buf_T	*buf;
    int		num_marked_files;
    int		load_marks;
    int		copy_marks_out;
    char_u	*str;
    int		i;
    char_u	*p;
    char_u	*name_buf;
    pos_T	pos;
#ifdef FEAT_EVAL
    list_T	*list = NULL;
#endif
    int		count = 0;
    int		buflist_used = 0;
    buf_T	*buflist_buf = NULL;

    if ((name_buf = alloc(LSIZE)) == NULL)
	return;
    *name_buf = NUL;

    if (fp_out != NULL && buflist->ga_len > 0)
    {
	// Sort the list of buffers on b_last_used.
	qsort(buflist->ga_data, (size_t)buflist->ga_len,
						sizeof(buf_T *), buf_compare);
	buflist_buf = ((buf_T **)buflist->ga_data)[0];
    }

#ifdef FEAT_EVAL
    if (fp_out == NULL && (flags & (VIF_GET_OLDFILES | VIF_FORCEIT)))
    {
	list = list_alloc();
	if (list != NULL)
	    set_vim_var_list(VV_OLDFILES, list);
    }
#endif

    num_marked_files = get_viminfo_parameter('\'');
    while (!eof && (count < num_marked_files || fp_out == NULL))
    {
	if (line[0] != '>')
	{
	    if (line[0] != '\n' && line[0] != '\r' && line[0] != '#')
	    {
		if (viminfo_error("E576: ", _(e_nonr_missing_gt), line))
		    break;	// too many errors, return now
	    }
	    eof = vim_fgets(line, LSIZE, virp->vir_fd);
	    continue;		// Skip this dud line
	}

	// Handle long line and translate escaped characters.
	// Find file name, set str to start.
	// Ignore leading and trailing white space.
	str = skipwhite(line + 1);
	str = viminfo_readstring(virp, (int)(str - virp->vir_line), FALSE);
	if (str == NULL)
	    continue;
	p = str + STRLEN(str);
	while (p != str && (*p == NUL || vim_isspace(*p)))
	    p--;
	if (*p)
	    p++;
	*p = NUL;

#ifdef FEAT_EVAL
	if (list != NULL)
	    list_append_string(list, str, -1);
#endif

	// If fp_out == NULL, load marks for current buffer.
	// If fp_out != NULL, copy marks for buffers not in buflist.
	load_marks = copy_marks_out = FALSE;
	if (fp_out == NULL)
	{
	    if ((flags & VIF_WANT_MARKS) && curbuf->b_ffname != NULL)
	    {
		if (*name_buf == NUL)	    // only need to do this once
		    home_replace(NULL, curbuf->b_ffname, name_buf, LSIZE, TRUE);
		if (fnamecmp(str, name_buf) == 0)
		    load_marks = TRUE;
	    }
	}
	else // fp_out != NULL
	{
	    // This is slow if there are many buffers!!
	    FOR_ALL_BUFFERS(buf)
		if (buf->b_ffname != NULL)
		{
		    home_replace(NULL, buf->b_ffname, name_buf, LSIZE, TRUE);
		    if (fnamecmp(str, name_buf) == 0)
			break;
		}

	    // Copy marks if the buffer has not been loaded.
	    if (buf == NULL || !buf->b_marks_read)
	    {
		int	did_read_line = FALSE;

		if (buflist_buf != NULL)
		{
		    // Read the next line.  If it has the "*" mark compare the
		    // time stamps.  Write entries from "buflist" that are
		    // newer.
		    if (!viminfo_readline(virp) && line[0] == TAB)
		    {
			did_read_line = TRUE;
			if (line[1] == '*')
			{
			    long	ltime;

			    sscanf((char *)line + 2, "%ld ", &ltime);
			    while ((time_T)ltime < buflist_buf->b_last_used)
			    {
				write_buffer_marks(buflist_buf, fp_out);
				if (++count >= num_marked_files)
				    break;
				if (++buflist_used == buflist->ga_len)
				{
				    buflist_buf = NULL;
				    break;
				}
				buflist_buf =
				   ((buf_T **)buflist->ga_data)[buflist_used];
			    }
			}
			else
			{
			    // No timestamp, must be written by an older Vim.
			    // Assume all remaining buffers are older than
			    // ours.
			    while (count < num_marked_files
					    && buflist_used < buflist->ga_len)
			    {
				buflist_buf = ((buf_T **)buflist->ga_data)
							     [buflist_used++];
				write_buffer_marks(buflist_buf, fp_out);
				++count;
			    }
			    buflist_buf = NULL;
			}

			if (count >= num_marked_files)
			{
			    vim_free(str);
			    break;
			}
		    }
		}

		fputs("\n> ", fp_out);
		viminfo_writestring(fp_out, str);
		if (did_read_line)
		    fputs((char *)line, fp_out);

		count++;
		copy_marks_out = TRUE;
	    }
	}
	vim_free(str);

	pos.coladd = 0;
	while (!(eof = viminfo_readline(virp)) && line[0] == TAB)
	{
	    if (load_marks)
	    {
		if (line[1] != NUL)
		{
		    unsigned u;

		    sscanf((char *)line + 2, "%ld %u", &pos.lnum, &u);
		    pos.col = u;
		    switch (line[1])
		    {
			case '"': curbuf->b_last_cursor = pos; break;
			case '^': curbuf->b_last_insert = pos; break;
			case '.': curbuf->b_last_change = pos; break;
			case '+':
				  // changelist positions are stored oldest
				  // first
				  if (curbuf->b_changelistlen == JUMPLISTSIZE)
				      // list is full, remove oldest entry
				      mch_memmove(curbuf->b_changelist,
					    curbuf->b_changelist + 1,
					    sizeof(pos_T) * (JUMPLISTSIZE - 1));
				  else
				      ++curbuf->b_changelistlen;
				  curbuf->b_changelist[
					   curbuf->b_changelistlen - 1] = pos;
				  break;

				  // Using the line number for the last-used
				  // timestamp.
			case '*': curbuf->b_last_used = pos.lnum; break;

			default:  if ((i = line[1] - 'a') >= 0 && i < NMARKS)
				      curbuf->b_namedm[i] = pos;
		    }
		}
	    }
	    else if (copy_marks_out)
		fputs((char *)line, fp_out);
	}

	if (load_marks)
	{
	    win_T	*wp;

	    FOR_ALL_WINDOWS(wp)
	    {
		if (wp->w_buffer == curbuf)
		    wp->w_changelistidx = curbuf->b_changelistlen;
	    }
	    if (flags & VIF_ONLY_CURBUF)
		break;
	}
    }

    if (fp_out != NULL)
	// Write any remaining entries from buflist.
	while (count < num_marked_files && buflist_used < buflist->ga_len)
	{
	    buflist_buf = ((buf_T **)buflist->ga_data)[buflist_used++];
	    write_buffer_marks(buflist_buf, fp_out);
	    ++count;
	}

    vim_free(name_buf);
}

/*
 * Read marks for the current buffer from the viminfo file, when we support
 * buffer marks and the buffer has a name.
 */
    void
check_marks_read(void)
{
    if (!curbuf->b_marks_read && get_viminfo_parameter('\'') > 0
						  && curbuf->b_ffname != NULL)
	read_viminfo(NULL, VIF_WANT_MARKS | VIF_ONLY_CURBUF);

    // Always set b_marks_read; needed when 'viminfo' is changed to include
    // the ' parameter after opening a buffer.
    curbuf->b_marks_read = TRUE;
}

    static int
read_viminfo_filemark(vir_T *virp, int force)
{
    char_u	*str;
    xfmark_T	*namedfm_p = get_namedfm();
    xfmark_T	*fm;
    int		i;

    // We only get here if line[0] == '\'' or '-'.
    // Illegal mark names are ignored (for future expansion).
    str = virp->vir_line + 1;
    if (*str <= 127
	    && ((*virp->vir_line == '\''
				       && (VIM_ISDIGIT(*str) || SAFE_isupper(*str)))
	     || (*virp->vir_line == '-' && *str == '\'')))
    {
	if (*str == '\'')
	{
	    // If the jumplist isn't full insert fmark as oldest entry
	    if (curwin->w_jumplistlen == JUMPLISTSIZE)
		fm = NULL;
	    else
	    {
		for (i = curwin->w_jumplistlen; i > 0; --i)
		    curwin->w_jumplist[i] = curwin->w_jumplist[i - 1];
		++curwin->w_jumplistidx;
		++curwin->w_jumplistlen;
		fm = &curwin->w_jumplist[0];
		fm->fmark.mark.lnum = 0;
		fm->fname = NULL;
	    }
	}
	else if (VIM_ISDIGIT(*str))
	    fm = &namedfm_p[*str - '0' + NMARKS];
	else
	    fm = &namedfm_p[*str - 'A'];
	if (fm != NULL && (fm->fmark.mark.lnum == 0 || force))
	{
	    str = skipwhite(str + 1);
	    fm->fmark.mark.lnum = getdigits(&str);
	    str = skipwhite(str);
	    fm->fmark.mark.col = getdigits(&str);
	    fm->fmark.mark.coladd = 0;
	    fm->fmark.fnum = 0;
	    str = skipwhite(str);
	    vim_free(fm->fname);
	    fm->fname = viminfo_readstring(virp, (int)(str - virp->vir_line),
								       FALSE);
	    fm->time_set = 0;
	}
    }
    return vim_fgets(virp->vir_line, LSIZE, virp->vir_fd);
}

/*
 * Prepare for reading viminfo marks when writing viminfo later.
 */
    static void
prepare_viminfo_marks(void)
{
    vi_namedfm = ALLOC_CLEAR_MULT(xfmark_T, NMARKS + EXTRA_MARKS);
    vi_jumplist = ALLOC_CLEAR_MULT(xfmark_T, JUMPLISTSIZE);
    vi_jumplist_len = 0;
}

    static void
finish_viminfo_marks(void)
{
    int		i;

    if (vi_namedfm != NULL)
    {
	for (i = 0; i < NMARKS + EXTRA_MARKS; ++i)
	    vim_free(vi_namedfm[i].fname);
	VIM_CLEAR(vi_namedfm);
    }
    if (vi_jumplist != NULL)
    {
	for (i = 0; i < vi_jumplist_len; ++i)
	    vim_free(vi_jumplist[i].fname);
	VIM_CLEAR(vi_jumplist);
    }
}

/*
 * Accept a new style mark line from the viminfo, store it when it's new.
 */
    static void
handle_viminfo_mark(garray_T *values, int force)
{
    bval_T	*vp = (bval_T *)values->ga_data;
    int		name;
    linenr_T	lnum;
    colnr_T	col;
    time_t	timestamp;
    xfmark_T	*fm = NULL;

    // Check the format:
    // |{bartype},{name},{lnum},{col},{timestamp},{filename}
    if (values->ga_len < 5
	    || vp[0].bv_type != BVAL_NR
	    || vp[1].bv_type != BVAL_NR
	    || vp[2].bv_type != BVAL_NR
	    || vp[3].bv_type != BVAL_NR
	    || vp[4].bv_type != BVAL_STRING)
	return;

    name = vp[0].bv_nr;
    if (name != '\'' && !VIM_ISDIGIT(name) && !ASCII_ISUPPER(name))
	return;
    lnum = vp[1].bv_nr;
    col = vp[2].bv_nr;
    if (lnum <= 0 || col < 0)
	return;
    timestamp = (time_t)vp[3].bv_nr;

    if (name == '\'')
    {
	if (vi_jumplist != NULL)
	{
	    if (vi_jumplist_len < JUMPLISTSIZE)
		fm = &vi_jumplist[vi_jumplist_len++];
	}
	else
	{
	    int idx;
	    int i;

	    // If we have a timestamp insert it in the right place.
	    if (timestamp != 0)
	    {
		for (idx = curwin->w_jumplistlen - 1; idx >= 0; --idx)
		    if (curwin->w_jumplist[idx].time_set < timestamp)
		    {
			++idx;
			break;
		    }
		// idx cannot be zero now
		if (idx < 0 && curwin->w_jumplistlen < JUMPLISTSIZE)
		    // insert as the oldest entry
		    idx = 0;
	    }
	    else if (curwin->w_jumplistlen < JUMPLISTSIZE)
		// insert as oldest entry
		idx = 0;
	    else
		idx = -1;

	    if (idx >= 0)
	    {
		if (curwin->w_jumplistlen == JUMPLISTSIZE)
		{
		    // Drop the oldest entry.
		    --idx;
		    vim_free(curwin->w_jumplist[0].fname);
		    for (i = 0; i < idx; ++i)
			curwin->w_jumplist[i] = curwin->w_jumplist[i + 1];
		}
		else
		{
		    // Move newer entries forward.
		    for (i = curwin->w_jumplistlen; i > idx; --i)
			curwin->w_jumplist[i] = curwin->w_jumplist[i - 1];
		    ++curwin->w_jumplistidx;
		    ++curwin->w_jumplistlen;
		}
		fm = &curwin->w_jumplist[idx];
		fm->fmark.mark.lnum = 0;
		fm->fname = NULL;
		fm->time_set = 0;
	    }
	}
    }
    else
    {
	int		idx;
	xfmark_T	*namedfm_p = get_namedfm();

	if (VIM_ISDIGIT(name))
	{
	    if (vi_namedfm != NULL)
		idx = name - '0' + NMARKS;
	    else
	    {
		int i;

		// Do not use the name from the viminfo file, insert in time
		// order.
		for (idx = NMARKS; idx < NMARKS + EXTRA_MARKS; ++idx)
		    if (namedfm_p[idx].time_set < timestamp)
			break;
		if (idx == NMARKS + EXTRA_MARKS)
		    // All existing entries are newer.
		    return;
		i = NMARKS + EXTRA_MARKS - 1;

		vim_free(namedfm_p[i].fname);
		for ( ; i > idx; --i)
		    namedfm_p[i] = namedfm_p[i - 1];
		namedfm_p[idx].fname = NULL;
	    }
	}
	else
	    idx = name - 'A';
	if (vi_namedfm != NULL)
	    fm = &vi_namedfm[idx];
	else
	    fm = &namedfm_p[idx];
    }

    if (fm != NULL)
    {
	if (vi_namedfm != NULL || fm->fmark.mark.lnum == 0
					  || fm->time_set < timestamp || force)
	{
	    fm->fmark.mark.lnum = lnum;
	    fm->fmark.mark.col = col;
	    fm->fmark.mark.coladd = 0;
	    fm->fmark.fnum = 0;
	    vim_free(fm->fname);
	    if (vp[4].bv_allocated)
	    {
		fm->fname = vp[4].bv_string;
		vp[4].bv_string = NULL;
	    }
	    else
		fm->fname = vim_strsave(vp[4].bv_string);
	    fm->time_set = timestamp;
	}
    }
}

    static int
read_viminfo_barline(vir_T *virp, int got_encoding, int force, int writing)
{
    char_u	*p = virp->vir_line + 1;
    int		bartype;
    garray_T	values;
    bval_T	*vp;
    int		i;
    int		read_next = TRUE;

    // The format is: |{bartype},{value},...
    // For a very long string:
    //     |{bartype},>{length of "{text}{text2}"}
    //     |<{text1}
    //     |<{text2},{value}
    // For a long line not using a string
    //     |{bartype},{lots of values},>
    //     |<{value},{value}
    if (*p == '<')
    {
	// Continuation line of an unrecognized item.
	if (writing)
	    ga_copy_string(&virp->vir_barlines, virp->vir_line);
    }
    else
    {
	ga_init2(&values, sizeof(bval_T), 20);
	bartype = getdigits(&p);
	switch (bartype)
	{
	    case BARTYPE_VERSION:
		// Only use the version when it comes before the encoding.
		// If it comes later it was copied by a Vim version that
		// doesn't understand the version.
		if (!got_encoding)
		{
		    read_next = barline_parse(virp, p, &values);
		    vp = (bval_T *)values.ga_data;
		    if (values.ga_len > 0 && vp->bv_type == BVAL_NR)
			virp->vir_version = vp->bv_nr;
		}
		break;

	    case BARTYPE_HISTORY:
		read_next = barline_parse(virp, p, &values);
		handle_viminfo_history(&values, writing);
		break;

	    case BARTYPE_REGISTER:
		read_next = barline_parse(virp, p, &values);
		handle_viminfo_register(&values, force);
		break;

	    case BARTYPE_MARK:
		read_next = barline_parse(virp, p, &values);
		handle_viminfo_mark(&values, force);
		break;

	    default:
		// copy unrecognized line (for future use)
		if (writing)
		    ga_copy_string(&virp->vir_barlines, virp->vir_line);
	}
	for (i = 0; i < values.ga_len; ++i)
	{
	    vp = (bval_T *)values.ga_data + i;
	    if (vp->bv_type == BVAL_STRING && vp->bv_allocated)
		vim_free(vp->bv_string);
	    vim_free(vp->bv_tofree);
	}
	ga_clear(&values);
    }

    if (read_next)
	return viminfo_readline(virp);
    return FALSE;
}

/*
 * read_viminfo_up_to_marks() -- Only called from do_viminfo().  Reads in the
 * first part of the viminfo file which contains everything but the marks that
 * are local to a file.  Returns TRUE when end-of-file is reached. -- webb
 */
    static int
read_viminfo_up_to_marks(
    vir_T	*virp,
    int		forceit,
    int		writing)
{
    int		eof;
    buf_T	*buf;
    int		got_encoding = FALSE;

    prepare_viminfo_history(forceit ? 9999 : 0, writing);

    eof = viminfo_readline(virp);
    while (!eof && virp->vir_line[0] != '>')
    {
	switch (virp->vir_line[0])
	{
		// Characters reserved for future expansion, ignored now
	    case '+': // "+40 /path/dir file", for running vim without args
	    case '^': // to be defined
	    case '<': // long line - ignored
		// A comment or empty line.
	    case NUL:
	    case '\r':
	    case '\n':
	    case '#':
		eof = viminfo_readline(virp);
		break;
	    case '|':
		eof = read_viminfo_barline(virp, got_encoding,
							    forceit, writing);
		break;
	    case '*': // "*encoding=value"
		got_encoding = TRUE;
		eof = viminfo_encoding(virp);
		break;
	    case '!': // global variable
#ifdef FEAT_EVAL
		eof = read_viminfo_varlist(virp, writing);
#else
		eof = viminfo_readline(virp);
#endif
		break;
	    case '%': // entry for buffer list
		eof = read_viminfo_bufferlist(virp, writing);
		break;
	    case '"':
		// When registers are in bar lines skip the old style register
		// lines.
		if (virp->vir_version < VIMINFO_VERSION_WITH_REGISTERS)
		    eof = read_viminfo_register(virp, forceit);
		else
		    do {
			eof = viminfo_readline(virp);
		    } while (!eof && (virp->vir_line[0] == TAB
						|| virp->vir_line[0] == '<'));
		break;
	    case '/':	    // Search string
	    case '&':	    // Substitute search string
	    case '~':	    // Last search string, followed by '/' or '&'
		eof = read_viminfo_search_pattern(virp, forceit);
		break;
	    case '$':
		eof = read_viminfo_sub_string(virp, forceit);
		break;
	    case ':':
	    case '?':
	    case '=':
	    case '@':
		// When history is in bar lines skip the old style history
		// lines.
		if (virp->vir_version < VIMINFO_VERSION_WITH_HISTORY)
		    eof = read_viminfo_history(virp, writing);
		else
		    eof = viminfo_readline(virp);
		break;
	    case '-':
	    case '\'':
		// When file marks are in bar lines skip the old style lines.
		if (virp->vir_version < VIMINFO_VERSION_WITH_MARKS)
		    eof = read_viminfo_filemark(virp, forceit);
		else
		    eof = viminfo_readline(virp);
		break;
	    default:
		if (viminfo_error("E575: ", _(e_illegal_starting_char),
			    virp->vir_line))
		    eof = TRUE;
		else
		    eof = viminfo_readline(virp);
		break;
	}
    }

    // Finish reading history items.
    if (!writing)
	finish_viminfo_history(virp);

    // Change file names to buffer numbers for fmarks.
    FOR_ALL_BUFFERS(buf)
	fmarks_check_names(buf);

    return eof;
}

/*
 * do_viminfo() -- Should only be called from read_viminfo() & write_viminfo().
 */
    static void
do_viminfo(FILE *fp_in, FILE *fp_out, int flags)
{
    int		eof = FALSE;
    vir_T	vir;
    int		merge = FALSE;
    int		do_copy_marks = FALSE;
    garray_T	buflist;

    if ((vir.vir_line = alloc(LSIZE)) == NULL)
	return;
    vir.vir_fd = fp_in;
    vir.vir_conv.vc_type = CONV_NONE;
    ga_init2(&vir.vir_barlines, sizeof(char_u *), 100);
    vir.vir_version = -1;

    if (fp_in != NULL)
    {
	if (flags & VIF_WANT_INFO)
	{
	    if (fp_out != NULL)
	    {
		// Registers and marks are read and kept separate from what
		// this Vim is using.  They are merged when writing.
		prepare_viminfo_registers();
		prepare_viminfo_marks();
	    }

	    eof = read_viminfo_up_to_marks(&vir,
					 flags & VIF_FORCEIT, fp_out != NULL);
	    merge = TRUE;
	}
	else if (flags != 0)
	    // Skip info, find start of marks
	    while (!(eof = viminfo_readline(&vir))
		    && vir.vir_line[0] != '>')
		;

	do_copy_marks = (flags & (VIF_WANT_MARKS | VIF_ONLY_CURBUF
					    | VIF_GET_OLDFILES | VIF_FORCEIT));
    }

    if (fp_out != NULL)
    {
	// Write the info:
	fprintf(fp_out, _("# This viminfo file was generated by Vim %s.\n"),
							  VIM_VERSION_MEDIUM);
	fputs(_("# You may edit it if you're careful!\n\n"), fp_out);
	write_viminfo_version(fp_out);
	fputs(_("# Value of 'encoding' when this file was written\n"), fp_out);
	fprintf(fp_out, "*encoding=%s\n\n", p_enc);
	write_viminfo_search_pattern(fp_out);
	write_viminfo_sub_string(fp_out);
	write_viminfo_history(fp_out, merge);
	write_viminfo_registers(fp_out);
	finish_viminfo_registers();
#ifdef FEAT_EVAL
	write_viminfo_varlist(fp_out);
#endif
	write_viminfo_filemarks(fp_out);
	finish_viminfo_marks();
	write_viminfo_bufferlist(fp_out);
	write_viminfo_barlines(&vir, fp_out);

	if (do_copy_marks)
	    ga_init2(&buflist, sizeof(buf_T *), 50);
	write_viminfo_marks(fp_out, do_copy_marks ? &buflist : NULL);
    }

    if (do_copy_marks)
    {
	copy_viminfo_marks(&vir, fp_out, &buflist, eof, flags);
	if (fp_out != NULL)
	    ga_clear(&buflist);
    }

    vim_free(vir.vir_line);
    if (vir.vir_conv.vc_type != CONV_NONE)
	convert_setup(&vir.vir_conv, NULL, NULL);
    ga_clear_strings(&vir.vir_barlines);
}

/*
 * read_viminfo() -- Read the viminfo file.  Registers etc. which are already
 * set are not over-written unless "flags" includes VIF_FORCEIT. -- webb
 */
    int
read_viminfo(
    char_u	*file,	    // file name or NULL to use default name
    int		flags)	    // VIF_WANT_INFO et al.
{
    FILE	*fp;
    char_u	*fname;
    stat_T	st;		// mch_stat() of existing viminfo file

    if (no_viminfo())
	return FAIL;

    fname = viminfo_filename(file);	// get file name in allocated buffer
    if (fname == NULL)
	return FAIL;
    fp = mch_fopen((char *)fname, READBIN);

    if (p_verbose > 0)
    {
	verbose_enter();
	smsg(_("Reading viminfo file \"%s\"%s%s%s%s"),
		fname,
		(flags & VIF_WANT_INFO) ? _(" info") : "",
		(flags & VIF_WANT_MARKS) ? _(" marks") : "",
		(flags & VIF_GET_OLDFILES) ? _(" oldfiles") : "",
		fp == NULL ? _(" FAILED") : "");
	verbose_leave();
    }

    vim_free(fname);
    if (fp == NULL)
	return FAIL;
    if (mch_fstat(fileno(fp), &st) < 0 || S_ISDIR(st.st_mode))
    {
	fclose(fp);
	return FAIL;
    }

    viminfo_errcnt = 0;
    do_viminfo(fp, NULL, flags);

    fclose(fp);
    return OK;
}

/*
 * Write the viminfo file.  The old one is read in first so that effectively a
 * merge of current info and old info is done.  This allows multiple vims to
 * run simultaneously, without losing any marks etc.
 * If "forceit" is TRUE, then the old file is not read in, and only internal
 * info is written to the file.
 */
    void
write_viminfo(char_u *file, int forceit)
{
    char_u	*fname;
    FILE	*fp_in = NULL;	// input viminfo file, if any
    FILE	*fp_out = NULL;	// output viminfo file
    char_u	*tempname = NULL;	// name of temp viminfo file
    stat_T	st_new;		// mch_stat() of potential new file
    stat_T	st_old;		// mch_stat() of existing viminfo file
#if defined(UNIX) || defined(VMS)
    mode_t	umask_save;
#endif
#ifdef UNIX
    int		shortname = FALSE;	// use 8.3 file name
#endif
#ifdef MSWIN
    int		hidden = FALSE;
#endif

    if (no_viminfo())
	return;

    fname = viminfo_filename(file);	// may set to default if NULL
    if (fname == NULL)
	return;

    fp_in = mch_fopen((char *)fname, READBIN);
    if (fp_in == NULL)
    {
	int fd;

	// if it does exist, but we can't read it, don't try writing
	if (mch_stat((char *)fname, &st_new) == 0)
	    goto end;

	// Create the new .viminfo non-accessible for others, because it may
	// contain text from non-accessible documents. It is up to the user to
	// widen access (e.g. to a group). This may also fail if there is a
	// race condition, then just give up.
	fd = mch_open((char *)fname,
			    O_CREAT|O_EXTRA|O_EXCL|O_WRONLY|O_NOFOLLOW, 0600);
	if (fd < 0)
	    goto end;
	fp_out = fdopen(fd, WRITEBIN);
    }
    else
    {
	// There is an existing viminfo file.  Create a temporary file to
	// write the new viminfo into, in the same directory as the
	// existing viminfo file, which will be renamed once all writing is
	// successful.
	if (mch_fstat(fileno(fp_in), &st_old) < 0
		|| S_ISDIR(st_old.st_mode)
#ifdef UNIX
		// For Unix we check the owner of the file.  It's not very nice
		// to overwrite a user's viminfo file after a "su root", with a
		// viminfo file that the user can't read.
		|| (getuid() != ROOT_UID
		    && !(st_old.st_uid == getuid()
			    ? (st_old.st_mode & 0200)
			    : (st_old.st_gid == getgid()
				    ? (st_old.st_mode & 0020)
				    : (st_old.st_mode & 0002))))
#endif
		)
	{
	    int	tt = msg_didany;

	    // avoid a wait_return() for this message, it's annoying
	    semsg(_(e_viminfo_file_is_not_writable_str), fname);
	    msg_didany = tt;
	    fclose(fp_in);
	    goto end;
	}
#ifdef MSWIN
	// Get the file attributes of the existing viminfo file.
	hidden = mch_ishidden(fname);
#endif

	// Make tempname, find one that does not exist yet.
	// Beware of a race condition: If someone logs out and all Vim
	// instances exit at the same time a temp file might be created between
	// stat() and open().  Use mch_open() with O_EXCL to avoid that.
	// May try twice: Once normal and once with shortname set, just in
	// case somebody puts his viminfo file in an 8.3 filesystem.
	for (;;)
	{
	    int		next_char = 'z';
	    char_u	*wp;

	    tempname = buf_modname(
#ifdef UNIX
				    shortname,
#else
				    FALSE,
#endif
				    fname,
#ifdef VMS
				    (char_u *)"-tmp",
#else
				    (char_u *)".tmp",
#endif
				    FALSE);
	    if (tempname == NULL)		// out of memory
		break;

	    // Try a series of names.  Change one character, just before
	    // the extension.  This should also work for an 8.3
	    // file name, when after adding the extension it still is
	    // the same file as the original.
	    wp = tempname + STRLEN(tempname) - 5;
	    if (wp < gettail(tempname))	    // empty file name?
		wp = gettail(tempname);
	    for (;;)
	    {
		// Check if tempfile already exists.  Never overwrite an
		// existing file!
		if (mch_stat((char *)tempname, &st_new) == 0)
		{
#ifdef UNIX
		    // Check if tempfile is same as original file.  May happen
		    // when modname() gave the same file back.  E.g.  silly
		    // link, or file name-length reached.  Try again with
		    // shortname set.
		    if (!shortname && st_new.st_dev == st_old.st_dev
						&& st_new.st_ino == st_old.st_ino)
		    {
			VIM_CLEAR(tempname);
			shortname = TRUE;
			break;
		    }
#endif
		}
		else
		{
		    // Try creating the file exclusively.  This may fail if
		    // another Vim tries to do it at the same time.
#ifdef VMS
		    // fdopen() fails for some reason
		    umask_save = umask(077);
		    fp_out = mch_fopen((char *)tempname, WRITEBIN);
		    (void)umask(umask_save);
#else
		    int	fd;

		    // Use mch_open() to be able to use O_NOFOLLOW and set file
		    // protection:
		    // Unix: same as original file, but strip s-bit.  Reset
		    // umask to avoid it getting in the way.
		    // Others: r&w for user only.
# ifdef UNIX
		    umask_save = umask(0);
		    fd = mch_open((char *)tempname,
			    O_CREAT|O_EXTRA|O_EXCL|O_WRONLY|O_NOFOLLOW,
					(int)((st_old.st_mode & 0777) | 0600));
		    (void)umask(umask_save);
# else
		    fd = mch_open((char *)tempname,
			     O_CREAT|O_EXTRA|O_EXCL|O_WRONLY|O_NOFOLLOW, 0600);
# endif
		    if (fd < 0)
		    {
			fp_out = NULL;
# ifdef EEXIST
			// Avoid trying lots of names while the problem is lack
			// of permission, only retry if the file already
			// exists.
			if (errno != EEXIST)
			    break;
# endif
		    }
		    else
			fp_out = fdopen(fd, WRITEBIN);
#endif // VMS
		    if (fp_out != NULL)
			break;
		}

		// Assume file exists, try again with another name.
		if (next_char == 'a' - 1)
		{
		    // They all exist?  Must be something wrong! Don't write
		    // the viminfo file then.
		    semsg(_(e_too_many_viminfo_temp_files_like_str), tempname);
		    break;
		}
		*wp = next_char;
		--next_char;
	    }

	    if (tempname != NULL)
		break;
	    // continue if shortname was set
	}

#if defined(UNIX) && defined(HAVE_FCHOWN)
	if (tempname != NULL && fp_out != NULL)
	{
		stat_T	tmp_st;

	    // Make sure the original owner can read/write the tempfile and
	    // otherwise preserve permissions, making sure the group matches.
	    if (mch_stat((char *)tempname, &tmp_st) >= 0)
	    {
		if (st_old.st_uid != tmp_st.st_uid)
		    // Changing the owner might fail, in which case the
		    // file will now be owned by the current user, oh well.
		    vim_ignored = fchown(fileno(fp_out), st_old.st_uid, -1);
		if (st_old.st_gid != tmp_st.st_gid
			&& fchown(fileno(fp_out), -1, st_old.st_gid) == -1)
		    // can't set the group to what it should be, remove
		    // group permissions
		    (void)mch_setperm(tempname, 0600);
	    }
	    else
		// can't stat the file, set conservative permissions
		(void)mch_setperm(tempname, 0600);
	}
#endif
    }

    // Check if the new viminfo file can be written to.
    if (fp_out == NULL)
    {
	semsg(_(e_cant_write_viminfo_file_str),
		       (fp_in == NULL || tempname == NULL) ? fname : tempname);
	if (fp_in != NULL)
	    fclose(fp_in);
	goto end;
    }

    if (p_verbose > 0)
    {
	verbose_enter();
	smsg(_("Writing viminfo file \"%s\""), fname);
	verbose_leave();
    }

    viminfo_errcnt = 0;
    do_viminfo(fp_in, fp_out, forceit ? 0 : (VIF_WANT_INFO | VIF_WANT_MARKS));

    if (fclose(fp_out) == EOF)
	++viminfo_errcnt;

    if (fp_in != NULL)
    {
	fclose(fp_in);

	// In case of an error keep the original viminfo file.  Otherwise
	// rename the newly written file.  Give an error if that fails.
	if (viminfo_errcnt == 0)
	{
	    if (vim_rename(tempname, fname) == -1)
	    {
		++viminfo_errcnt;
		semsg(_(e_cant_rename_viminfo_file_to_str), fname);
	    }
# ifdef MSWIN
	    // If the viminfo file was hidden then also hide the new file.
	    else if (hidden)
		mch_hide(fname);
# endif
	}
	if (viminfo_errcnt > 0)
	    mch_remove(tempname);
    }

end:
    vim_free(fname);
    vim_free(tempname);
}

/*
 * ":rviminfo" and ":wviminfo".
 */
    void
ex_viminfo(
    exarg_T	*eap)
{
    char_u	*save_viminfo;

    save_viminfo = p_viminfo;
    if (*p_viminfo == NUL)
	p_viminfo = (char_u *)"'100";
    if (eap->cmdidx == CMD_rviminfo)
    {
	if (read_viminfo(eap->arg, VIF_WANT_INFO | VIF_WANT_MARKS
				  | (eap->forceit ? VIF_FORCEIT : 0)) == FAIL)
	    emsg(_(e_cannot_open_viminfo_file_for_reading));
    }
    else
	write_viminfo(eap->arg, eap->forceit);
    p_viminfo = save_viminfo;
}

#endif // FEAT_VIMINFO
