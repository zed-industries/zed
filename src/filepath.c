/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * filepath.c: dealing with file names and paths.
 */

#include "vim.h"

#ifdef MSWIN
/*
 * Functions for ":8" filename modifier: get 8.3 version of a filename.
 */

/*
 * Get the short path (8.3) for the filename in "fnamep".
 * Only works for a valid file name.
 * When the path gets longer "fnamep" is changed and the allocated buffer
 * is put in "bufp".
 * *fnamelen is the length of "fnamep" and set to 0 for a nonexistent path.
 * Returns OK on success, FAIL on failure.
 */
    static int
get_short_pathname(char_u **fnamep, char_u **bufp, int *fnamelen)
{
    int		l, len;
    WCHAR	*newbuf;
    WCHAR	*wfname;

    len = MAXPATHL;
    newbuf = malloc(len * sizeof(*newbuf));
    if (newbuf == NULL)
	return FAIL;

    wfname = enc_to_utf16(*fnamep, NULL);
    if (wfname == NULL)
    {
	vim_free(newbuf);
	return FAIL;
    }

    l = GetShortPathNameW(wfname, newbuf, len);
    if (l > len - 1)
    {
	// If that doesn't work (not enough space), then save the string
	// and try again with a new buffer big enough.
	WCHAR *newbuf_t = newbuf;
	newbuf = vim_realloc(newbuf, (l + 1) * sizeof(*newbuf));
	if (newbuf == NULL)
	{
	    vim_free(wfname);
	    vim_free(newbuf_t);
	    return FAIL;
	}
	// Really should always succeed, as the buffer is big enough.
	l = GetShortPathNameW(wfname, newbuf, l+1);
    }
    if (l != 0)
    {
	char_u *p = utf16_to_enc(newbuf, NULL);

	if (p != NULL)
	{
	    vim_free(*bufp);
	    *fnamep = *bufp = p;
	}
	else
	{
	    vim_free(wfname);
	    vim_free(newbuf);
	    return FAIL;
	}
    }
    vim_free(wfname);
    vim_free(newbuf);

    *fnamelen = l == 0 ? l : (int)STRLEN(*bufp);
    return OK;
}

/*
 * Get the short path (8.3) for the filename in "fname". The converted
 * path is returned in "bufp".
 *
 * Some of the directories specified in "fname" may not exist. This function
 * will shorten the existing directories at the beginning of the path and then
 * append the remaining non-existing path.
 *
 * fname - Pointer to the filename to shorten.  On return, contains the
 *	   pointer to the shortened pathname
 * bufp -  Pointer to an allocated buffer for the filename.
 * fnamelen - Length of the filename pointed to by fname
 *
 * Returns OK on success (or nothing done) and FAIL on failure (out of memory).
 */
    static int
shortpath_for_invalid_fname(
    char_u	**fname,
    char_u	**bufp,
    int		*fnamelen)
{
    char_u	*short_fname, *save_fname, *pbuf_unused;
    char_u	*endp, *save_endp;
    char_u	ch;
    int		old_len, len;
    int		new_len, sfx_len;
    int		retval = OK;

    // Make a copy
    old_len = *fnamelen;
    save_fname = vim_strnsave(*fname, old_len);
    pbuf_unused = NULL;
    short_fname = NULL;

    endp = save_fname + old_len - 1; // Find the end of the copy
    save_endp = endp;

    /*
     * Try shortening the supplied path till it succeeds by removing one
     * directory at a time from the tail of the path.
     */
    len = 0;
    for (;;)
    {
	// go back one path-separator
	while (endp > save_fname && !after_pathsep(save_fname, endp + 1))
	    --endp;
	if (endp <= save_fname)
	    break;		// processed the complete path

	/*
	 * Replace the path separator with a NUL and try to shorten the
	 * resulting path.
	 */
	ch = *endp;
	*endp = 0;
	short_fname = save_fname;
	len = (int)STRLEN(short_fname) + 1;
	if (get_short_pathname(&short_fname, &pbuf_unused, &len) == FAIL)
	{
	    retval = FAIL;
	    goto theend;
	}
	*endp = ch;	// preserve the string

	if (len > 0)
	    break;	// successfully shortened the path

	// failed to shorten the path. Skip the path separator
	--endp;
    }

    if (len > 0)
    {
	/*
	 * Succeeded in shortening the path. Now concatenate the shortened
	 * path with the remaining path at the tail.
	 */

	// Compute the length of the new path.
	sfx_len = (int)(save_endp - endp) + 1;
	new_len = len + sfx_len;

	*fnamelen = new_len;
	vim_free(*bufp);
	if (new_len > old_len)
	{
	    // There is not enough space in the currently allocated string,
	    // copy it to a buffer big enough.
	    *fname = *bufp = vim_strnsave(short_fname, new_len);
	    if (*fname == NULL)
	    {
		retval = FAIL;
		goto theend;
	    }
	}
	else
	{
	    // Transfer short_fname to the main buffer (it's big enough),
	    // unless get_short_pathname() did its work in-place.
	    *fname = *bufp = save_fname;
	    if (short_fname != save_fname)
		STRNCPY(save_fname, short_fname, len);
	    save_fname = NULL;
	}

	// concat the not-shortened part of the path
	if ((*fname + len) != endp)
	    vim_strncpy(*fname + len, endp, sfx_len);
	(*fname)[new_len] = NUL;
    }

theend:
    vim_free(pbuf_unused);
    vim_free(save_fname);

    return retval;
}

/*
 * Get a pathname for a partial path.
 * Returns OK for success, FAIL for failure.
 */
    static int
shortpath_for_partial(
    char_u	**fnamep,
    char_u	**bufp,
    int		*fnamelen)
{
    int		sepcount, len, tflen;
    char_u	*p;
    char_u	*pbuf, *tfname;
    int		hasTilde;

    // Count up the path separators from the RHS.. so we know which part
    // of the path to return.
    sepcount = 0;
    for (p = *fnamep; p < *fnamep + *fnamelen; MB_PTR_ADV(p))
	if (vim_ispathsep(*p))
	    ++sepcount;

    // Need full path first (use expand_env() to remove a "~/")
    hasTilde = (**fnamep == '~');
    if (hasTilde)
	pbuf = tfname = expand_env_save(*fnamep);
    else
	pbuf = tfname = FullName_save(*fnamep, FALSE);

    len = tflen = (int)STRLEN(tfname);

    if (get_short_pathname(&tfname, &pbuf, &len) == FAIL)
	return FAIL;

    if (len == 0)
    {
	// Don't have a valid filename, so shorten the rest of the
	// path if we can. This CAN give us invalid 8.3 filenames, but
	// there's not a lot of point in guessing what it might be.
	len = tflen;
	if (shortpath_for_invalid_fname(&tfname, &pbuf, &len) == FAIL)
	    return FAIL;
    }

    // Count the paths backward to find the beginning of the desired string.
    for (p = tfname + len - 1; p >= tfname; --p)
    {
	if (has_mbyte)
	    p -= mb_head_off(tfname, p);
	if (vim_ispathsep(*p))
	{
	    if (sepcount == 0 || (hasTilde && sepcount == 1))
		break;
	    else
		sepcount --;
	}
    }
    if (hasTilde)
    {
	--p;
	if (p >= tfname)
	    *p = '~';
	else
	    return FAIL;
    }
    else
	++p;

    // Copy in the string - p indexes into tfname - allocated at pbuf
    vim_free(*bufp);
    *fnamelen = (int)STRLEN(p);
    *bufp = pbuf;
    *fnamep = p;

    return OK;
}
#endif // MSWIN

/*
 * Adjust a filename, according to a string of modifiers.
 * *fnamep must be NUL terminated when called.  When returning, the length is
 * determined by *fnamelen.
 * Returns VALID_ flags or -1 for failure.
 * When there is an error, *fnamep is set to NULL.
 */
    int
modify_fname(
    char_u	*src,		// string with modifiers
    int		tilde_file,	// "~" is a file name, not $HOME
    int		*usedlen,	// characters after src that are used
    char_u	**fnamep,	// file name so far
    char_u	**bufp,		// buffer for allocated file name or NULL
    int		*fnamelen)	// length of fnamep
{
    int		valid = 0;
    char_u	*tail;
    char_u	*s, *p, *pbuf;
    char_u	dirname[MAXPATHL];
    int		c;
    int		has_fullname = 0;
    int		has_homerelative = 0;
#ifdef MSWIN
    char_u	*fname_start = *fnamep;
    int		has_shortname = 0;
#endif

repeat:
    // ":p" - full path/file_name
    if (src[*usedlen] == ':' && src[*usedlen + 1] == 'p')
    {
	has_fullname = 1;

	valid |= VALID_PATH;
	*usedlen += 2;

	// Expand "~/path" for all systems and "~user/path" for Unix and VMS
	if ((*fnamep)[0] == '~'
#if !defined(UNIX) && !(defined(VMS) && defined(USER_HOME))
		&& ((*fnamep)[1] == '/'
# ifdef BACKSLASH_IN_FILENAME
		    || (*fnamep)[1] == '\\'
# endif
		    || (*fnamep)[1] == NUL)
#endif
		&& !(tilde_file && (*fnamep)[1] == NUL)
	   )
	{
	    *fnamep = expand_env_save(*fnamep);
	    vim_free(*bufp);	// free any allocated file name
	    *bufp = *fnamep;
	    if (*fnamep == NULL)
		return -1;
	}

	// When "/." or "/.." is used: force expansion to get rid of it.
	for (p = *fnamep; *p != NUL; MB_PTR_ADV(p))
	{
	    if (vim_ispathsep(*p)
		    && p[1] == '.'
		    && (p[2] == NUL
			|| vim_ispathsep(p[2])
			|| (p[2] == '.'
			    && (p[3] == NUL || vim_ispathsep(p[3])))))
		break;
	}

	// FullName_save() is slow, don't use it when not needed.
	if (*p != NUL || !vim_isAbsName(*fnamep))
	{
	    *fnamep = FullName_save(*fnamep, *p != NUL);
	    vim_free(*bufp);	// free any allocated file name
	    *bufp = *fnamep;
	    if (*fnamep == NULL)
		return -1;
	}

#ifdef MSWIN
# if _WIN32_WINNT >= 0x0500
	if (vim_strchr(*fnamep, '~') != NULL)
	{
	    // Expand 8.3 filename to full path.  Needed to make sure the same
	    // file does not have two different names.
	    // Note: problem does not occur if _WIN32_WINNT < 0x0500.
	    WCHAR *wfname = enc_to_utf16(*fnamep, NULL);
	    WCHAR buf[_MAX_PATH];

	    if (wfname != NULL)
	    {
		if (GetLongPathNameW(wfname, buf, _MAX_PATH))
		{
		    char_u *q = utf16_to_enc(buf, NULL);

		    if (q != NULL)
		    {
			vim_free(*bufp);    // free any allocated file name
			*bufp = *fnamep = q;
		    }
		}
		vim_free(wfname);
	    }
	}
# endif
#endif
	// Append a path separator to a directory.
	if (mch_isdir(*fnamep))
	{
	    // Make room for one or two extra characters.
	    *fnamep = vim_strnsave(*fnamep, STRLEN(*fnamep) + 2);
	    vim_free(*bufp);	// free any allocated file name
	    *bufp = *fnamep;
	    if (*fnamep == NULL)
		return -1;
	    add_pathsep(*fnamep);
	}
    }

    // ":." - path relative to the current directory
    // ":~" - path relative to the home directory
    // ":8" - shortname path - postponed till after
    while (src[*usedlen] == ':'
		  && ((c = src[*usedlen + 1]) == '.' || c == '~' || c == '8'))
    {
	*usedlen += 2;
	if (c == '8')
	{
#ifdef MSWIN
	    has_shortname = 1; // Postpone this.
#endif
	    continue;
	}
	pbuf = NULL;
	// Need full path first (use expand_env() to remove a "~/")
	if (!has_fullname && !has_homerelative)
	{
	    if (**fnamep == '~')
		p = pbuf = expand_env_save(*fnamep);
	    else
		p = pbuf = FullName_save(*fnamep, FALSE);
	}
	else
	    p = *fnamep;

	has_fullname = 0;

	if (p != NULL)
	{
	    if (c == '.')
	    {
		size_t	namelen;

		mch_dirname(dirname, MAXPATHL);
		if (has_homerelative)
		{
		    s = vim_strsave(dirname);
		    if (s != NULL)
		    {
			home_replace(NULL, s, dirname, MAXPATHL, TRUE);
			vim_free(s);
		    }
		}
		namelen = STRLEN(dirname);

		// Do not call shorten_fname() here since it removes the prefix
		// even though the path does not have a prefix.
		if (fnamencmp(p, dirname, namelen) == 0)
		{
		    p += namelen;
		    if (vim_ispathsep(*p))
		    {
			while (*p && vim_ispathsep(*p))
			    ++p;
			*fnamep = p;
			if (pbuf != NULL)
			{
			    // free any allocated file name
			    vim_free(*bufp);
			    *bufp = pbuf;
			    pbuf = NULL;
			}
		    }
		}
	    }
	    else
	    {
		home_replace(NULL, p, dirname, MAXPATHL, TRUE);
		// Only replace it when it starts with '~'
		if (*dirname == '~')
		{
		    s = vim_strsave(dirname);
		    if (s != NULL)
		    {
			*fnamep = s;
			vim_free(*bufp);
			*bufp = s;
			has_homerelative = TRUE;
		    }
		}
	    }
	    vim_free(pbuf);
	}
    }

    tail = gettail(*fnamep);
    *fnamelen = (int)STRLEN(*fnamep);

    // ":h" - head, remove "/file_name", can be repeated
    // Don't remove the first "/" or "c:\"
    while (src[*usedlen] == ':' && src[*usedlen + 1] == 'h')
    {
	valid |= VALID_HEAD;
	*usedlen += 2;
	s = get_past_head(*fnamep);
	while (tail > s && after_pathsep(s, tail))
	    MB_PTR_BACK(*fnamep, tail);
	*fnamelen = (int)(tail - *fnamep);
#ifdef VMS
	if (*fnamelen > 0)
	    *fnamelen += 1; // the path separator is part of the path
#endif
	if (*fnamelen == 0)
	{
	    // Result is empty.  Turn it into "." to make ":cd %:h" work.
	    p = vim_strsave((char_u *)".");
	    if (p == NULL)
		return -1;
	    vim_free(*bufp);
	    *bufp = *fnamep = tail = p;
	    *fnamelen = 1;
	}
	else
	{
	    while (tail > s && !after_pathsep(s, tail))
		MB_PTR_BACK(*fnamep, tail);
	}
    }

    // ":8" - shortname
    if (src[*usedlen] == ':' && src[*usedlen + 1] == '8')
    {
	*usedlen += 2;
#ifdef MSWIN
	has_shortname = 1;
#endif
    }

#ifdef MSWIN
    /*
     * Handle ":8" after we have done 'heads' and before we do 'tails'.
     */
    if (has_shortname)
    {
	// Copy the string if it is shortened by :h and when it wasn't copied
	// yet, because we are going to change it in place.  Avoids changing
	// the buffer name for "%:8".
	if (*fnamelen < (int)STRLEN(*fnamep) || *fnamep == fname_start)
	{
	    p = vim_strnsave(*fnamep, *fnamelen);
	    if (p == NULL)
		return -1;
	    vim_free(*bufp);
	    *bufp = *fnamep = p;
	}

	// Split into two implementations - makes it easier.  First is where
	// there isn't a full name already, second is where there is.
	if (!has_fullname && !vim_isAbsName(*fnamep))
	{
	    if (shortpath_for_partial(fnamep, bufp, fnamelen) == FAIL)
		return -1;
	}
	else
	{
	    int		l = *fnamelen;

	    // Simple case, already have the full-name.
	    // Nearly always shorter, so try first time.
	    if (get_short_pathname(fnamep, bufp, &l) == FAIL)
		return -1;

	    if (l == 0)
	    {
		// Couldn't find the filename, search the paths.
		l = *fnamelen;
		if (shortpath_for_invalid_fname(fnamep, bufp, &l) == FAIL)
		    return -1;
	    }
	    *fnamelen = l;
	}
    }
#endif // MSWIN

    // ":t" - tail, just the basename
    if (src[*usedlen] == ':' && src[*usedlen + 1] == 't')
    {
	*usedlen += 2;
	*fnamelen -= (int)(tail - *fnamep);
	*fnamep = tail;
    }

    // ":e" - extension, can be repeated
    // ":r" - root, without extension, can be repeated
    while (src[*usedlen] == ':'
	    && (src[*usedlen + 1] == 'e' || src[*usedlen + 1] == 'r'))
    {
	// find a '.' in the tail:
	// - for second :e: before the current fname
	// - otherwise: The last '.'
	if (src[*usedlen + 1] == 'e' && *fnamep > tail)
	    s = *fnamep - 2;
	else
	    s = *fnamep + *fnamelen - 1;
	for ( ; s > tail; --s)
	    if (s[0] == '.')
		break;
	if (src[*usedlen + 1] == 'e')		// :e
	{
	    if (s > tail)
	    {
		*fnamelen += (int)(*fnamep - (s + 1));
		*fnamep = s + 1;
#ifdef VMS
		// cut version from the extension
		s = *fnamep + *fnamelen - 1;
		for ( ; s > *fnamep; --s)
		    if (s[0] == ';')
			break;
		if (s > *fnamep)
		    *fnamelen = s - *fnamep;
#endif
	    }
	    else if (*fnamep <= tail)
		*fnamelen = 0;
	}
	else				// :r
	{
	    char_u *limit = *fnamep;

	    if (limit < tail)
		limit = tail;
	    if (s > limit)	// remove one extension
		*fnamelen = (int)(s - *fnamep);
	}
	*usedlen += 2;
    }

    // ":s?pat?foo?" - substitute
    // ":gs?pat?foo?" - global substitute
    if (src[*usedlen] == ':'
	    && (src[*usedlen + 1] == 's'
		|| (src[*usedlen + 1] == 'g' && src[*usedlen + 2] == 's')))
    {
	char_u	    *str;
	char_u	    *pat;
	char_u	    *sub;
	int	    sep;
	char_u	    *flags;
	int	    didit = FALSE;

	flags = (char_u *)"";
	s = src + *usedlen + 2;
	if (src[*usedlen + 1] == 'g')
	{
	    flags = (char_u *)"g";
	    ++s;
	}

	sep = *s++;
	if (sep)
	{
	    // find end of pattern
	    p = vim_strchr(s, sep);
	    if (p != NULL)
	    {
		pat = vim_strnsave(s, p - s);
		if (pat != NULL)
		{
		    s = p + 1;
		    // find end of substitution
		    p = vim_strchr(s, sep);
		    if (p != NULL)
		    {
			sub = vim_strnsave(s, p - s);
			str = vim_strnsave(*fnamep, *fnamelen);
			if (sub != NULL && str != NULL)
			{
			    *usedlen = (int)(p + 1 - src);
			    s = do_string_sub(str, pat, sub, NULL, flags);
			    if (s != NULL)
			    {
				*fnamep = s;
				*fnamelen = (int)STRLEN(s);
				vim_free(*bufp);
				*bufp = s;
				didit = TRUE;
			    }
			}
			vim_free(sub);
			vim_free(str);
		    }
		    vim_free(pat);
		}
	    }
	    // after using ":s", repeat all the modifiers
	    if (didit)
		goto repeat;
	}
    }

    if (src[*usedlen] == ':' && src[*usedlen + 1] == 'S')
    {
	// vim_strsave_shellescape() needs a NUL terminated string.
	c = (*fnamep)[*fnamelen];
	if (c != NUL)
	    (*fnamep)[*fnamelen] = NUL;
	p = vim_strsave_shellescape(*fnamep, FALSE, FALSE);
	if (c != NUL)
	    (*fnamep)[*fnamelen] = c;
	if (p == NULL)
	    return -1;
	vim_free(*bufp);
	*bufp = *fnamep = p;
	*fnamelen = (int)STRLEN(p);
	*usedlen += 2;
    }

    return valid;
}

/*
 * Shorten the path of a file from "~/foo/../.bar/fname" to "~/f/../.b/fname"
 * "trim_len" specifies how many characters to keep for each directory.
 * Must be 1 or more.
 * It's done in-place.
 */
    static void
shorten_dir_len(char_u *str, int trim_len)
{
    char_u	*tail, *s, *d;
    int		skip = FALSE;
    int		dirchunk_len = 0;

    tail = gettail(str);
    d = str;
    for (s = str; ; ++s)
    {
	if (s >= tail)		    // copy the whole tail
	{
	    *d++ = *s;
	    if (*s == NUL)
		break;
	}
	else if (vim_ispathsep(*s))	    // copy '/' and next char
	{
	    *d++ = *s;
	    skip = FALSE;
	    dirchunk_len = 0;
	}
	else if (!skip)
	{
	    *d++ = *s;			// copy next char
	    if (*s != '~' && *s != '.') // and leading "~" and "."
	    {
		++dirchunk_len; // only count word chars for the size

		// keep copying chars until we have our preferred length (or
		// until the above if/else branches move us along)
		if (dirchunk_len >= trim_len)
		    skip = TRUE;
	    }

	    if (has_mbyte)
	    {
		int l = mb_ptr2len(s);

		while (--l > 0)
		    *d++ = *++s;
	    }
	}
    }
}

/*
 * Shorten the path of a file from "~/foo/../.bar/fname" to "~/f/../.b/fname"
 * It's done in-place.
 */
    void
shorten_dir(char_u *str)
{
    shorten_dir_len(str, 1);
}

/*
 * Return TRUE if "fname" is a readable file.
 */
    int
file_is_readable(char_u *fname)
{
    int		fd;

#ifndef O_NONBLOCK
# define O_NONBLOCK 0
#endif
    if (*fname && !mch_isdir(fname)
	      && (fd = mch_open((char *)fname, O_RDONLY | O_NONBLOCK, 0)) >= 0)
    {
	close(fd);
	return TRUE;
    }
    return FALSE;
}

#if defined(FEAT_EVAL) || defined(PROTO)

/*
 * "chdir(dir)" function
 */
    void
f_chdir(typval_T *argvars, typval_T *rettv)
{
    char_u	*cwd;
    cdscope_T	scope = CDSCOPE_GLOBAL;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (argvars[0].v_type != VAR_STRING)
    {
	// Returning an empty string means it failed.
	// No error message, for historic reasons.
	if (in_vim9script())
	    (void) check_for_string_arg(argvars, 0);
	return;
    }

    // Return the current directory
    cwd = alloc(MAXPATHL);
    if (cwd != NULL)
    {
	if (mch_dirname(cwd, MAXPATHL) != FAIL)
	{
#ifdef BACKSLASH_IN_FILENAME
	    slash_adjust(cwd);
#endif
	    rettv->vval.v_string = vim_strsave(cwd);
	}
	vim_free(cwd);
    }

    if (curwin->w_localdir != NULL)
	scope = CDSCOPE_WINDOW;
    else if (curtab->tp_localdir != NULL)
	scope = CDSCOPE_TABPAGE;

    if (!changedir_func(argvars[0].vval.v_string, TRUE, scope))
	// Directory change failed
	VIM_CLEAR(rettv->vval.v_string);
}

/*
 * "delete()" function
 */
    void
f_delete(typval_T *argvars, typval_T *rettv)
{
    char_u	nbuf[NUMBUFLEN];
    char_u	*name;
    char_u	*flags;

    rettv->vval.v_number = -1;
    if (check_restricted() || check_secure())
	return;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL))
	return;

    name = tv_get_string(&argvars[0]);
    if (name == NULL || *name == NUL)
    {
	emsg(_(e_invalid_argument));
	return;
    }

    if (argvars[1].v_type != VAR_UNKNOWN)
	flags = tv_get_string_buf(&argvars[1], nbuf);
    else
	flags = (char_u *)"";

    if (*flags == NUL)
	// delete a file
	rettv->vval.v_number = mch_remove(name) == 0 ? 0 : -1;
    else if (STRCMP(flags, "d") == 0)
	// delete an empty directory
	rettv->vval.v_number = mch_rmdir(name) == 0 ? 0 : -1;
    else if (STRCMP(flags, "rf") == 0)
	// delete a directory recursively
	rettv->vval.v_number = delete_recursive(name);
    else
	semsg(_(e_invalid_expression_str), flags);
}

/*
 * "executable()" function
 */
    void
f_executable(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    // Check in $PATH and also check directly if there is a directory name.
    rettv->vval.v_number = mch_can_exe(tv_get_string(&argvars[0]), NULL, TRUE);
}

/*
 * "exepath()" function
 */
    void
f_exepath(typval_T *argvars, typval_T *rettv)
{
    char_u *p = NULL;

    if (in_vim9script() && check_for_nonempty_string_arg(argvars, 0) == FAIL)
	return;
    (void)mch_can_exe(tv_get_string(&argvars[0]), &p, TRUE);
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = p;
}

/*
 * "filereadable()" function
 */
    void
f_filereadable(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;
    rettv->vval.v_number = file_is_readable(tv_get_string(&argvars[0]));
}

/*
 * Return 0 for not writable, 1 for writable file, 2 for a dir which we have
 * rights to write into.
 */
    void
f_filewritable(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;
    rettv->vval.v_number = filewritable(tv_get_string(&argvars[0]));
}

    static void
findfilendir(
    typval_T	*argvars,
    typval_T	*rettv,
    int		find_what)
{
    char_u	*fname;
    char_u	*fresult = NULL;
    char_u	*path = *curbuf->b_p_path == NUL ? p_path : curbuf->b_p_path;
    char_u	*p;
    char_u	pathbuf[NUMBUFLEN];
    int		count = 1;
    int		first = TRUE;
    int		error = FALSE;

    rettv->vval.v_string = NULL;
    rettv->v_type = VAR_STRING;
    if (in_vim9script()
	    && (check_for_nonempty_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 2) == FAIL)))
	return;

    fname = tv_get_string(&argvars[0]);

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	p = tv_get_string_buf_chk(&argvars[1], pathbuf);
	if (p == NULL)
	    error = TRUE;
	else
	{
	    if (*p != NUL)
		path = p;

	    if (argvars[2].v_type != VAR_UNKNOWN)
		count = (int)tv_get_number_chk(&argvars[2], &error);
	}
    }

    if (count < 0 && rettv_list_alloc(rettv) == FAIL)
	error = TRUE;

    if (*fname != NUL && !error)
    {
	char_u	*file_to_find = NULL;
	char	*search_ctx = NULL;

	do
	{
	    if (rettv->v_type == VAR_STRING || rettv->v_type == VAR_LIST)
		vim_free(fresult);
	    fresult = find_file_in_path_option(first ? fname : NULL,
					       first ? (int)STRLEN(fname) : 0,
					0, first, path,
					find_what,
					curbuf->b_ffname,
					find_what == FINDFILE_DIR
					    ? (char_u *)"" : curbuf->b_p_sua,
					    &file_to_find, &search_ctx);
	    first = FALSE;

	    if (fresult != NULL && rettv->v_type == VAR_LIST)
		list_append_string(rettv->vval.v_list, fresult, -1);

	} while ((rettv->v_type == VAR_LIST || --count > 0) && fresult != NULL);

	vim_free(file_to_find);
	vim_findfile_cleanup(search_ctx);
    }

    if (rettv->v_type == VAR_STRING)
	rettv->vval.v_string = fresult;
}

/*
 * "finddir({fname}[, {path}[, {count}]])" function
 */
    void
f_finddir(typval_T *argvars, typval_T *rettv)
{
    findfilendir(argvars, rettv, FINDFILE_DIR);
}

/*
 * "findfile({fname}[, {path}[, {count}]])" function
 */
    void
f_findfile(typval_T *argvars, typval_T *rettv)
{
    findfilendir(argvars, rettv, FINDFILE_FILE);
}

/*
 * "fnamemodify({fname}, {mods})" function
 */
    void
f_fnamemodify(typval_T *argvars, typval_T *rettv)
{
    char_u	*fname;
    char_u	*mods;
    int		usedlen = 0;
    int		len = 0;
    char_u	*fbuf = NULL;
    char_u	buf[NUMBUFLEN];

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL))
	return;

    fname = tv_get_string_chk(&argvars[0]);
    mods = tv_get_string_buf_chk(&argvars[1], buf);
    if (mods == NULL || fname == NULL)
	fname = NULL;
    else
    {
	len = (int)STRLEN(fname);
	if (mods != NULL && *mods != NUL)
	    (void)modify_fname(mods, FALSE, &usedlen, &fname, &fbuf, &len);
    }

    rettv->v_type = VAR_STRING;
    if (fname == NULL)
	rettv->vval.v_string = NULL;
    else
	rettv->vval.v_string = vim_strnsave(fname, len);
    vim_free(fbuf);
}

/*
 * "getcwd()" function
 *
 * Return the current working directory of a window in a tab page.
 * First optional argument 'winnr' is the window number or -1 and the second
 * optional argument 'tabnr' is the tab page number.
 *
 * If no arguments are supplied, then return the directory of the current
 * window.
 * If only 'winnr' is specified and is not -1 or 0 then return the directory of
 * the specified window.
 * If 'winnr' is 0 then return the directory of the current window.
 * If both 'winnr and 'tabnr' are specified and 'winnr' is -1 then return the
 * directory of the specified tab page.  Otherwise return the directory of the
 * specified window in the specified tab page.
 * If the window or the tab page doesn't exist then return NULL.
 */
    void
f_getcwd(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp = NULL;
    tabpage_T	*tp = NULL;
    char_u	*cwd;
    int		global = FALSE;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script()
	    && (check_for_opt_number_arg(argvars, 0) == FAIL
		|| (argvars[0].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 1) == FAIL)))
	return;

    if (argvars[0].v_type == VAR_NUMBER
	    && argvars[0].vval.v_number == -1
	    && argvars[1].v_type == VAR_UNKNOWN)
	global = TRUE;
    else
	wp = find_tabwin(&argvars[0], &argvars[1], &tp);

    if (wp != NULL && wp->w_localdir != NULL
					   && argvars[0].v_type != VAR_UNKNOWN)
	rettv->vval.v_string = vim_strsave(wp->w_localdir);
    else if (tp != NULL && tp->tp_localdir != NULL
					   && argvars[0].v_type != VAR_UNKNOWN)
	rettv->vval.v_string = vim_strsave(tp->tp_localdir);
    else if (wp != NULL || tp != NULL || global)
    {
	if (globaldir != NULL && argvars[0].v_type != VAR_UNKNOWN)
	    rettv->vval.v_string = vim_strsave(globaldir);
	else
	{
	    cwd = alloc(MAXPATHL);
	    if (cwd != NULL)
	    {
		if (mch_dirname(cwd, MAXPATHL) != FAIL)
		    rettv->vval.v_string = vim_strsave(cwd);
		vim_free(cwd);
	    }
	}
    }
#ifdef BACKSLASH_IN_FILENAME
    if (rettv->vval.v_string != NULL)
	slash_adjust(rettv->vval.v_string);
#endif
}

/*
 * Convert "st" to file permission string.
 */
    char_u *
getfpermst(stat_T *st, char_u *perm)
{
    char_u	    flags[] = "rwx";
    int		    i;

    for (i = 0; i < 9; i++)
    {
	if (st->st_mode & (1 << (8 - i)))
	    perm[i] = flags[i % 3];
	else
	    perm[i] = '-';
    }
    return perm;
}

/*
 * "getfperm({fname})" function
 */
    void
f_getfperm(typval_T *argvars, typval_T *rettv)
{
    char_u	*fname;
    stat_T	st;
    char_u	*perm = NULL;
    char_u	permbuf[] = "---------";

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    fname = tv_get_string(&argvars[0]);

    rettv->v_type = VAR_STRING;
    if (mch_stat((char *)fname, &st) >= 0)
	perm = vim_strsave(getfpermst(&st, permbuf));
    rettv->vval.v_string = perm;
}

/*
 * "getfsize({fname})" function
 */
    void
f_getfsize(typval_T *argvars, typval_T *rettv)
{
    char_u	*fname;
    stat_T	st;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    fname = tv_get_string(&argvars[0]);
    if (mch_stat((char *)fname, &st) >= 0)
    {
	if (mch_isdir(fname))
	    rettv->vval.v_number = 0;
	else
	{
	    rettv->vval.v_number = (varnumber_T)st.st_size;

	    // non-perfect check for overflow
	    if ((off_T)rettv->vval.v_number != (off_T)st.st_size)
		rettv->vval.v_number = -2;
	}
    }
    else
	  rettv->vval.v_number = -1;
}

/*
 * "getftime({fname})" function
 */
    void
f_getftime(typval_T *argvars, typval_T *rettv)
{
    char_u	*fname;
    stat_T	st;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    fname = tv_get_string(&argvars[0]);
    if (mch_stat((char *)fname, &st) >= 0)
	rettv->vval.v_number = (varnumber_T)st.st_mtime;
    else
	rettv->vval.v_number = -1;
}

/*
 * Convert "st" to file type string.
 */
    char_u *
getftypest(stat_T *st)
{
    char    *t;

    if (S_ISREG(st->st_mode))
	t = "file";
    else if (S_ISDIR(st->st_mode))
	t = "dir";
    else if (S_ISLNK(st->st_mode))
	t = "link";
    else if (S_ISBLK(st->st_mode))
	t = "bdev";
    else if (S_ISCHR(st->st_mode))
	t = "cdev";
    else if (S_ISFIFO(st->st_mode))
	t = "fifo";
    else if (S_ISSOCK(st->st_mode))
	t = "socket";
    else
	t = "other";
    return (char_u*)t;
}

/*
 * "getftype({fname})" function
 */
    void
f_getftype(typval_T *argvars, typval_T *rettv)
{
    char_u	*fname;
    stat_T	st;
    char_u	*type = NULL;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    fname = tv_get_string(&argvars[0]);

    rettv->v_type = VAR_STRING;
    if (mch_lstat((char *)fname, &st) >= 0)
	type = vim_strsave(getftypest(&st));
    rettv->vval.v_string = type;
}

/*
 * "glob()" function
 */
    void
f_glob(typval_T *argvars, typval_T *rettv)
{
    int		options = WILD_SILENT|WILD_USE_NL;
    expand_T	xpc;
    int		error = FALSE;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && (check_for_opt_bool_arg(argvars, 2) == FAIL
			|| (argvars[2].v_type != VAR_UNKNOWN
			    && check_for_opt_bool_arg(argvars, 3) == FAIL)))))
	return;

    // When the optional second argument is non-zero, don't remove matches
    // for 'wildignore' and don't put matches for 'suffixes' at the end.
    rettv->v_type = VAR_STRING;
    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	if (tv_get_bool_chk(&argvars[1], &error))
	    options |= WILD_KEEP_ALL;
	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    if (tv_get_bool_chk(&argvars[2], &error))
		rettv_list_set(rettv, NULL);
	    if (argvars[3].v_type != VAR_UNKNOWN
				    && tv_get_bool_chk(&argvars[3], &error))
		options |= WILD_ALLLINKS;
	}
    }
    if (!error)
    {
	ExpandInit(&xpc);
	xpc.xp_context = EXPAND_FILES;
	if (p_wic)
	    options += WILD_ICASE;
	if (rettv->v_type == VAR_STRING)
	    rettv->vval.v_string = ExpandOne(&xpc, tv_get_string(&argvars[0]),
						     NULL, options, WILD_ALL);
	else if (rettv_list_alloc(rettv) == OK)
	{
	  int i;

	  ExpandOne(&xpc, tv_get_string(&argvars[0]),
						NULL, options, WILD_ALL_KEEP);
	  for (i = 0; i < xpc.xp_numfiles; i++)
	      list_append_string(rettv->vval.v_list, xpc.xp_files[i], -1);

	  ExpandCleanup(&xpc);
	}
    }
    else
	rettv->vval.v_string = NULL;
}

/*
 * "glob2regpat()" function
 */
    void
f_glob2regpat(typval_T *argvars, typval_T *rettv)
{
    char_u	buf[NUMBUFLEN];
    char_u	*pat;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    pat = tv_get_string_buf_chk_strict(&argvars[0], buf, in_vim9script());
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = (pat == NULL)
			 ? NULL : file_pat_to_reg_pat(pat, NULL, NULL, FALSE);
}

/*
 * "globpath()" function
 */
    void
f_globpath(typval_T *argvars, typval_T *rettv)
{
    int		flags = WILD_IGNORE_COMPLETESLASH;
    char_u	buf1[NUMBUFLEN];
    char_u	*file;
    int		error = FALSE;
    garray_T	ga;
    int		i;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_opt_bool_arg(argvars, 2) == FAIL
		|| (argvars[2].v_type != VAR_UNKNOWN
		    && (check_for_opt_bool_arg(argvars, 3) == FAIL
			|| (argvars[3].v_type != VAR_UNKNOWN
			    && check_for_opt_bool_arg(argvars, 4) == FAIL)))))
	return;

    file = tv_get_string_buf_chk(&argvars[1], buf1);

    // When the optional second argument is non-zero, don't remove matches
    // for 'wildignore' and don't put matches for 'suffixes' at the end.
    rettv->v_type = VAR_STRING;
    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	if (tv_get_bool_chk(&argvars[2], &error))
	    flags |= WILD_KEEP_ALL;
	if (argvars[3].v_type != VAR_UNKNOWN)
	{
	    if (tv_get_bool_chk(&argvars[3], &error))
		rettv_list_set(rettv, NULL);
	    if (argvars[4].v_type != VAR_UNKNOWN
				    && tv_get_bool_chk(&argvars[4], &error))
		flags |= WILD_ALLLINKS;
	}
    }
    if (file != NULL && !error)
    {
	ga_init2(&ga, sizeof(char_u *), 10);
	globpath(tv_get_string(&argvars[0]), file, &ga, flags, FALSE);
	if (rettv->v_type == VAR_STRING)
	    rettv->vval.v_string = ga_concat_strings(&ga, "\n");
	else if (rettv_list_alloc(rettv) == OK)
	    for (i = 0; i < ga.ga_len; ++i)
		list_append_string(rettv->vval.v_list,
					    ((char_u **)(ga.ga_data))[i], -1);
	ga_clear_strings(&ga);
    }
    else
	rettv->vval.v_string = NULL;
}

/*
 * "isdirectory()" function
 */
    void
f_isdirectory(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_number = mch_isdir(tv_get_string(&argvars[0]));
}

/*
 * "isabsolutepath()" function
 */
    void
f_isabsolutepath(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_number = mch_isFullName(tv_get_string_strict(&argvars[0]));
}

/*
 * Create the directory in which "dir" is located, and higher levels when
 * needed.
 * Set "created" to the full name of the first created directory.  It will be
 * NULL until that happens.
 * Return OK or FAIL.
 */
    static int
mkdir_recurse(char_u *dir, int prot, char_u **created)
{
    char_u	*p;
    char_u	*updir;
    int		r = FAIL;

    // Get end of directory name in "dir".
    // We're done when it's "/" or "c:/".
    p = gettail_sep(dir);
    if (p <= get_past_head(dir))
	return OK;

    // If the directory exists we're done.  Otherwise: create it.
    updir = vim_strnsave(dir, p - dir);
    if (updir == NULL)
	return FAIL;
    if (mch_isdir(updir))
	r = OK;
    else if (mkdir_recurse(updir, prot, created) == OK)
    {
	r = vim_mkdir_emsg(updir, prot);
	if (r == OK && created != NULL && *created == NULL)
	    *created = FullName_save(updir, FALSE);
    }
    vim_free(updir);
    return r;
}

/*
 * "mkdir()" function
 */
    void
f_mkdir(typval_T *argvars, typval_T *rettv)
{
    char_u	*dir;
    char_u	buf[NUMBUFLEN];
    int		prot = 0755;
    int		defer = FALSE;
    int		defer_recurse = FALSE;
    char_u	*created = NULL;

    rettv->vval.v_number = FAIL;
    if (check_restricted() || check_secure())
	return;

    if (in_vim9script()
	    && (check_for_nonempty_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 2) == FAIL)))
	return;

    dir = tv_get_string_buf(&argvars[0], buf);
    if (*dir == NUL)
	return;

    if (*gettail(dir) == NUL)
	// remove trailing slashes
	*gettail_sep(dir) = NUL;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	char_u *arg2;

	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    prot = (int)tv_get_number_chk(&argvars[2], NULL);
	    if (prot == -1)
		return;
	}
	arg2 = tv_get_string(&argvars[1]);
	defer = vim_strchr(arg2, 'D') != NULL;
	defer_recurse = vim_strchr(arg2, 'R') != NULL;
	if ((defer || defer_recurse) && !can_add_defer())
	    return;

	if (vim_strchr(arg2, 'p') != NULL)
	{
	    if (mch_isdir(dir))
	    {
		// With the "p" flag it's OK if the dir already exists.
		rettv->vval.v_number = OK;
		return;
	    }
	    mkdir_recurse(dir, prot, defer || defer_recurse ? &created : NULL);
	}
    }
    rettv->vval.v_number = vim_mkdir_emsg(dir, prot);

    // Handle "D" and "R": deferred deletion of the created directory.
    if (rettv->vval.v_number == OK
				&& created == NULL && (defer || defer_recurse))
	created = FullName_save(dir, FALSE);
    if (created != NULL)
    {
	typval_T tv[2];

	tv[0].v_type = VAR_STRING;
	tv[0].v_lock = 0;
	tv[0].vval.v_string = created;
	tv[1].v_type = VAR_STRING;
	tv[1].v_lock = 0;
	tv[1].vval.v_string = vim_strsave(
				       (char_u *)(defer_recurse ? "rf" : "d"));
	if (tv[0].vval.v_string == NULL || tv[1].vval.v_string == NULL
		|| add_defer((char_u *)"delete", 2, tv) == FAIL)
	{
	    vim_free(tv[0].vval.v_string);
	    vim_free(tv[1].vval.v_string);
	}
    }
}

/*
 * "pathshorten()" function
 */
    void
f_pathshorten(typval_T *argvars, typval_T *rettv)
{
    char_u	*p;
    int		trim_len = 1;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL))
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	trim_len = (int)tv_get_number(&argvars[1]);
	if (trim_len < 1)
	    trim_len = 1;
    }

    rettv->v_type = VAR_STRING;
    p = tv_get_string_chk(&argvars[0]);

    if (p == NULL)
	rettv->vval.v_string = NULL;
    else
    {
	p = vim_strsave(p);
	rettv->vval.v_string = p;
	if (p != NULL)
	    shorten_dir_len(p, trim_len);
    }
}

/*
 * Common code for readdir_checkitem() and readdirex_checkitem().
 * Either "name" or "dict" is NULL.
 */
    static int
checkitem_common(void *context, char_u *name, dict_T *dict)
{
    typval_T	*expr = (typval_T *)context;
    typval_T	save_val;
    typval_T	rettv;
    typval_T	argv[2];
    int		retval = 0;
    int		error = FALSE;

    prepare_vimvar(VV_VAL, &save_val);
    if (name != NULL)
    {
	set_vim_var_string(VV_VAL, name, -1);
	argv[0].v_type = VAR_STRING;
	argv[0].vval.v_string = name;
    }
    else
    {
	set_vim_var_dict(VV_VAL, dict);
	argv[0].v_type = VAR_DICT;
	argv[0].vval.v_dict = dict;
    }

    if (eval_expr_typval(expr, FALSE, argv, 1, NULL, &rettv) == FAIL)
	goto theend;

    // We want to use -1, but also true/false should be allowed.
    if (rettv.v_type == VAR_SPECIAL || rettv.v_type == VAR_BOOL)
    {
	rettv.v_type = VAR_NUMBER;
	rettv.vval.v_number = rettv.vval.v_number == VVAL_TRUE;
    }
    retval = tv_get_number_chk(&rettv, &error);
    if (error)
	retval = -1;
    clear_tv(&rettv);

theend:
    if (name != NULL)
	set_vim_var_string(VV_VAL, NULL, 0);
    else
	set_vim_var_dict(VV_VAL, NULL);
    restore_vimvar(VV_VAL, &save_val);
    return retval;
}

/*
 * Evaluate "expr" (= "context") for readdir().
 */
    static int
readdir_checkitem(void *context, void *item)
{
    char_u	*name = (char_u *)item;

    return checkitem_common(context, name, NULL);
}

/*
 * Process the keys in the Dict argument to the readdir() and readdirex()
 * functions.  Assumes the Dict argument is the 3rd argument.
 */
    static int
readdirex_dict_arg(typval_T *argvars, int *cmp)
{
    char_u     *compare;

    if (check_for_nonnull_dict_arg(argvars, 2) == FAIL)
	return FAIL;

    if (dict_has_key(argvars[2].vval.v_dict, "sort"))
	compare = dict_get_string(argvars[2].vval.v_dict, "sort", FALSE);
    else
    {
	semsg(_(e_dictionary_key_str_required), "sort");
	return FAIL;
    }

    if (STRCMP(compare, (char_u *) "none") == 0)
	*cmp = READDIR_SORT_NONE;
    else if (STRCMP(compare, (char_u *) "case") == 0)
	*cmp = READDIR_SORT_BYTE;
    else if (STRCMP(compare, (char_u *) "icase") == 0)
	*cmp = READDIR_SORT_IC;
    else if (STRCMP(compare, (char_u *) "collate") == 0)
	*cmp = READDIR_SORT_COLLATE;
    return OK;
}

/*
 * "readdir()" function
 */
    void
f_readdir(typval_T *argvars, typval_T *rettv)
{
    typval_T	*expr;
    int		ret;
    char_u	*path;
    char_u	*p;
    garray_T	ga;
    int		i;
    int		sort = READDIR_SORT_BYTE;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_dict_arg(argvars, 2) == FAIL)))
	return;

    path = tv_get_string(&argvars[0]);
    expr = &argvars[1];

    if (argvars[1].v_type != VAR_UNKNOWN && argvars[2].v_type != VAR_UNKNOWN &&
	    readdirex_dict_arg(argvars, &sort) == FAIL)
	return;

    ret = readdir_core(&ga, path, FALSE, (void *)expr,
	    (expr->v_type == VAR_UNKNOWN) ? NULL : readdir_checkitem, sort);
    if (ret == OK)
    {
	for (i = 0; i < ga.ga_len; i++)
	{
	    p = ((char_u **)ga.ga_data)[i];
	    list_append_string(rettv->vval.v_list, p, -1);
	}
    }
    ga_clear_strings(&ga);
}

/*
 * Evaluate "expr" (= "context") for readdirex().
 */
    static int
readdirex_checkitem(void *context, void *item)
{
    dict_T	*dict = (dict_T*)item;

    return checkitem_common(context, NULL, dict);
}

/*
 * "readdirex()" function
 */
    void
f_readdirex(typval_T *argvars, typval_T *rettv)
{
    typval_T	*expr;
    int		ret;
    char_u	*path;
    garray_T	ga;
    int		i;
    int		sort = READDIR_SORT_BYTE;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_dict_arg(argvars, 2) == FAIL)))
	return;

    path = tv_get_string(&argvars[0]);
    expr = &argvars[1];

    if (argvars[1].v_type != VAR_UNKNOWN && argvars[2].v_type != VAR_UNKNOWN &&
	    readdirex_dict_arg(argvars, &sort) == FAIL)
	return;

    ret = readdir_core(&ga, path, TRUE, (void *)expr,
	    (expr->v_type == VAR_UNKNOWN) ? NULL : readdirex_checkitem, sort);
    if (ret == OK)
    {
	for (i = 0; i < ga.ga_len; i++)
	{
	    dict_T  *dict = ((dict_T**)ga.ga_data)[i];
	    list_append_dict(rettv->vval.v_list, dict);
	    dict_unref(dict);
	}
    }
    ga_clear(&ga);
}

/*
 * "readfile()" function
 */
    static void
read_file_or_blob(typval_T *argvars, typval_T *rettv, int always_blob)
{
    int		binary = FALSE;
    int		blob = always_blob;
    int		failed = FALSE;
    char_u	*fname;
    FILE	*fd;
    char_u	buf[(IOSIZE/256)*256];	// rounded to avoid odd + 1
    int		io_size = sizeof(buf);
    int		readlen;		// size of last fread()
    char_u	*prev	 = NULL;	// previously read bytes, if any
    long	prevlen  = 0;		// length of data in prev
    long	prevsize = 0;		// size of prev buffer
    long	maxline  = MAXLNUM;
    long	cnt	 = 0;
    char_u	*p;			// position in buf
    char_u	*start;			// start of current line
    off_T	offset = 0;
    off_T	size = -1;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	if (always_blob)
	{
	    offset = (off_T)tv_get_number(&argvars[1]);
	    if (argvars[2].v_type != VAR_UNKNOWN)
		size = (off_T)tv_get_number(&argvars[2]);
	}
	else
	{
	    if (STRCMP(tv_get_string(&argvars[1]), "b") == 0)
		binary = TRUE;
	    if (STRCMP(tv_get_string(&argvars[1]), "B") == 0)
		blob = TRUE;

	    if (argvars[2].v_type != VAR_UNKNOWN)
		maxline = (long)tv_get_number(&argvars[2]);
	}
    }

    if ((blob ? rettv_blob_alloc(rettv) : rettv_list_alloc(rettv)) == FAIL)
	return;

    // Always open the file in binary mode, library functions have a mind of
    // their own about CR-LF conversion.
    fname = tv_get_string(&argvars[0]);

    if (mch_isdir(fname))
    {
	semsg(_(e_str_is_directory), fname);
	return;
    }
    if (*fname == NUL || (fd = mch_fopen((char *)fname, READBIN)) == NULL)
    {
	semsg(_(e_cant_open_file_str),
			       *fname == NUL ? (char_u *)_("<empty>") : fname);
	return;
    }

    if (blob)
    {
	if (read_blob(fd, rettv, offset, size) == FAIL)
	    semsg(_(e_cant_read_file_str), fname);
	fclose(fd);
	return;
    }

    while (cnt < maxline || maxline < 0)
    {
	readlen = (int)fread(buf, 1, io_size, fd);

	// This for loop processes what was read, but is also entered at end
	// of file so that either:
	// - an incomplete line gets written
	// - a "binary" file gets an empty line at the end if it ends in a
	//   newline.
	for (p = buf, start = buf;
		p < buf + readlen || (readlen <= 0 && (prevlen > 0 || binary));
		++p)
	{
	    if (readlen <= 0 || *p == '\n')
	    {
		listitem_T  *li;
		char_u	    *s	= NULL;
		long_u	    len = p - start;

		// Finished a line.  Remove CRs before NL.
		if (readlen > 0 && !binary)
		{
		    while (len > 0 && start[len - 1] == '\r')
			--len;
		    // removal may cross back to the "prev" string
		    if (len == 0)
			while (prevlen > 0 && prev[prevlen - 1] == '\r')
			    --prevlen;
		}
		if (prevlen == 0)
		    s = vim_strnsave(start, len);
		else
		{
		    // Change "prev" buffer to be the right size.  This way
		    // the bytes are only copied once, and very long lines are
		    // allocated only once.
		    if ((s = vim_realloc(prev, prevlen + len + 1)) != NULL)
		    {
			mch_memmove(s + prevlen, start, len);
			s[prevlen + len] = NUL;
			prev = NULL; // the list will own the string
			prevlen = prevsize = 0;
		    }
		}
		if (s == NULL)
		{
		    do_outofmem_msg((long_u) prevlen + len + 1);
		    failed = TRUE;
		    break;
		}

		if ((li = listitem_alloc()) == NULL)
		{
		    vim_free(s);
		    failed = TRUE;
		    break;
		}
		li->li_tv.v_type = VAR_STRING;
		li->li_tv.v_lock = 0;
		li->li_tv.vval.v_string = s;
		list_append(rettv->vval.v_list, li);

		start = p + 1; // step over newline
		if ((++cnt >= maxline && maxline >= 0) || readlen <= 0)
		    break;
	    }
	    else if (*p == NUL)
		*p = '\n';
	    // Check for utf8 "bom"; U+FEFF is encoded as EF BB BF.  Do this
	    // when finding the BF and check the previous two bytes.
	    else if (*p == 0xbf && enc_utf8 && !binary)
	    {
		// Find the two bytes before the 0xbf.	If p is at buf, or buf
		// + 1, these may be in the "prev" string.
		char_u back1 = p >= buf + 1 ? p[-1]
				     : prevlen >= 1 ? prev[prevlen - 1] : NUL;
		char_u back2 = p >= buf + 2 ? p[-2]
			  : p == buf + 1 && prevlen >= 1 ? prev[prevlen - 1]
			  : prevlen >= 2 ? prev[prevlen - 2] : NUL;

		if (back2 == 0xef && back1 == 0xbb)
		{
		    char_u *dest = p - 2;

		    // Usually a BOM is at the beginning of a file, and so at
		    // the beginning of a line; then we can just step over it.
		    if (start == dest)
			start = p + 1;
		    else
		    {
			// have to shuffle buf to close gap
			int adjust_prevlen = 0;

			if (dest < buf)
			{
			    // must be 1 or 2
			    adjust_prevlen = (int)(buf - dest);
			    dest = buf;
			}
			if (readlen > p - buf + 1)
			    mch_memmove(dest, p + 1, readlen - (p - buf) - 1);
			readlen -= 3 - adjust_prevlen;
			prevlen -= adjust_prevlen;
			p = dest - 1;
		    }
		}
	    }
	} // for

	if (failed || (cnt >= maxline && maxline >= 0) || readlen <= 0)
	    break;
	if (start < p)
	{
	    // There's part of a line in buf, store it in "prev".
	    if (p - start + prevlen >= prevsize)
	    {
		// need bigger "prev" buffer
		char_u *newprev;

		// A common use case is ordinary text files and "prev" gets a
		// fragment of a line, so the first allocation is made
		// small, to avoid repeatedly 'allocing' large and
		// 'reallocing' small.
		if (prevsize == 0)
		    prevsize = (long)(p - start);
		else
		{
		    long grow50pc = (prevsize * 3) / 2;
		    long growmin  = (long)((p - start) * 2 + prevlen);
		    prevsize = grow50pc > growmin ? grow50pc : growmin;
		}
		newprev = vim_realloc(prev, prevsize);
		if (newprev == NULL)
		{
		    do_outofmem_msg((long_u)prevsize);
		    failed = TRUE;
		    break;
		}
		prev = newprev;
	    }
	    // Add the line part to end of "prev".
	    mch_memmove(prev + prevlen, start, p - start);
	    prevlen += (long)(p - start);
	}
    } // while

    // For a negative line count use only the lines at the end of the file,
    // free the rest.
    if (!failed && maxline < 0)
	while (cnt > -maxline)
	{
	    listitem_remove(rettv->vval.v_list, rettv->vval.v_list->lv_first);
	    --cnt;
	}

    if (failed)
    {
	// an empty list is returned on error
	list_free(rettv->vval.v_list);
	rettv_list_alloc(rettv);
    }

    vim_free(prev);
    fclose(fd);
}

/*
 * "readblob()" function
 */
    void
f_readblob(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 2) == FAIL)))
	return;

    read_file_or_blob(argvars, rettv, TRUE);
}

/*
 * "readfile()" function
 */
    void
f_readfile(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_nonempty_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 2) == FAIL)))
	return;

    read_file_or_blob(argvars, rettv, FALSE);
}

/*
 * "resolve()" function
 */
    void
f_resolve(typval_T *argvars, typval_T *rettv)
{
    char_u	*p;
#ifdef HAVE_READLINK
    char_u	*buf = NULL;
#endif

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    p = tv_get_string(&argvars[0]);
#ifdef FEAT_SHORTCUT
    {
	char_u	*v = NULL;

	v = mch_resolve_path(p, TRUE);
	if (v != NULL)
	    rettv->vval.v_string = v;
	else
	    rettv->vval.v_string = vim_strsave(p);
    }
#else
# ifdef HAVE_READLINK
    {
	char_u	*cpy;
	int	len;
	char_u	*remain = NULL;
	char_u	*q;
	int	is_relative_to_current = FALSE;
	int	has_trailing_pathsep = FALSE;
	int	limit = 100;

	p = vim_strsave(p);
	if (p == NULL)
	    goto fail;
	if (p[0] == '.' && (vim_ispathsep(p[1])
				   || (p[1] == '.' && (vim_ispathsep(p[2])))))
	    is_relative_to_current = TRUE;

	len = STRLEN(p);
	if (len > 1 && after_pathsep(p, p + len))
	{
	    has_trailing_pathsep = TRUE;
	    p[len - 1] = NUL; // the trailing slash breaks readlink()
	}

	q = getnextcomp(p);
	if (*q != NUL)
	{
	    // Separate the first path component in "p", and keep the
	    // remainder (beginning with the path separator).
	    remain = vim_strsave(q - 1);
	    q[-1] = NUL;
	}

	buf = alloc(MAXPATHL + 1);
	if (buf == NULL)
	{
	    vim_free(p);
	    goto fail;
	}

	for (;;)
	{
	    for (;;)
	    {
		len = readlink((char *)p, (char *)buf, MAXPATHL);
		if (len <= 0)
		    break;
		buf[len] = NUL;

		if (limit-- == 0)
		{
		    vim_free(p);
		    vim_free(remain);
		    emsg(_(e_too_many_symbolic_links_cycle));
		    rettv->vval.v_string = NULL;
		    goto fail;
		}

		// Ensure that the result will have a trailing path separator
		// if the argument has one.
		if (remain == NULL && has_trailing_pathsep)
		    add_pathsep(buf);

		// Separate the first path component in the link value and
		// concatenate the remainders.
		q = getnextcomp(vim_ispathsep(*buf) ? buf + 1 : buf);
		if (*q != NUL)
		{
		    if (remain == NULL)
			remain = vim_strsave(q - 1);
		    else
		    {
			cpy = concat_str(q - 1, remain);
			if (cpy != NULL)
			{
			    vim_free(remain);
			    remain = cpy;
			}
		    }
		    q[-1] = NUL;
		}

		q = gettail(p);
		if (q > p && *q == NUL)
		{
		    // Ignore trailing path separator.
		    p[q - p - 1] = NUL;
		    q = gettail(p);
		}
		if (q > p && !mch_isFullName(buf))
		{
		    // symlink is relative to directory of argument
		    cpy = alloc(STRLEN(p) + STRLEN(buf) + 1);
		    if (cpy != NULL)
		    {
			STRCPY(cpy, p);
			STRCPY(gettail(cpy), buf);
			vim_free(p);
			p = cpy;
		    }
		}
		else
		{
		    vim_free(p);
		    p = vim_strsave(buf);
		}
	    }

	    if (remain == NULL)
		break;

	    // Append the first path component of "remain" to "p".
	    q = getnextcomp(remain + 1);
	    len = q - remain - (*q != NUL);
	    cpy = vim_strnsave(p, STRLEN(p) + len);
	    if (cpy != NULL)
	    {
		STRNCAT(cpy, remain, len);
		vim_free(p);
		p = cpy;
	    }
	    // Shorten "remain".
	    if (*q != NUL)
		STRMOVE(remain, q - 1);
	    else
		VIM_CLEAR(remain);
	}

	// If the result is a relative path name, make it explicitly relative to
	// the current directory if and only if the argument had this form.
	if (!vim_ispathsep(*p))
	{
	    if (is_relative_to_current
		    && *p != NUL
		    && !(p[0] == '.'
			&& (p[1] == NUL
			    || vim_ispathsep(p[1])
			    || (p[1] == '.'
				&& (p[2] == NUL
				    || vim_ispathsep(p[2]))))))
	    {
		// Prepend "./".
		cpy = concat_str((char_u *)"./", p);
		if (cpy != NULL)
		{
		    vim_free(p);
		    p = cpy;
		}
	    }
	    else if (!is_relative_to_current)
	    {
		// Strip leading "./".
		q = p;
		while (q[0] == '.' && vim_ispathsep(q[1]))
		    q += 2;
		if (q > p)
		    STRMOVE(p, p + 2);
	    }
	}

	// Ensure that the result will have no trailing path separator
	// if the argument had none.  But keep "/" or "//".
	if (!has_trailing_pathsep)
	{
	    q = p + STRLEN(p);
	    if (after_pathsep(p, q))
		*gettail_sep(p) = NUL;
	}

	rettv->vval.v_string = p;
    }
# else
    rettv->vval.v_string = vim_strsave(p);
# endif
#endif

    simplify_filename(rettv->vval.v_string);

#ifdef HAVE_READLINK
fail:
    vim_free(buf);
#endif
    rettv->v_type = VAR_STRING;
}

/*
 * "tempname()" function
 */
    void
f_tempname(typval_T *argvars UNUSED, typval_T *rettv)
{
    static int	x = 'A';

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = vim_tempname(x, FALSE);

    // Advance 'x' to use A-Z and 0-9, so that there are at least 34 different
    // names.  Skip 'I' and 'O', they are used for shell redirection.
    do
    {
	if (x == 'Z')
	    x = '0';
	else if (x == '9')
	    x = 'A';
	else
	    ++x;
    } while (x == 'I' || x == 'O');
}

/*
 * "writefile()" function
 */
    void
f_writefile(typval_T *argvars, typval_T *rettv)
{
    int		binary = FALSE;
    int		append = FALSE;
    int		defer = FALSE;
#ifdef HAVE_FSYNC
    int		do_fsync = p_fs;
#endif
    char_u	*fname;
    FILE	*fd;
    int		ret = 0;
    listitem_T	*li;
    list_T	*list = NULL;
    blob_T	*blob = NULL;

    rettv->vval.v_number = -1;
    if (check_secure())
	return;

    if (in_vim9script()
	    && (check_for_list_or_blob_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_opt_string_arg(argvars, 2) == FAIL))
	return;

    if (argvars[0].v_type == VAR_LIST)
    {
	list = argvars[0].vval.v_list;
	if (list == NULL)
	    return;
	CHECK_LIST_MATERIALIZE(list);
	FOR_ALL_LIST_ITEMS(list, li)
	    if (tv_get_string_chk(&li->li_tv) == NULL)
		return;
    }
    else if (argvars[0].v_type == VAR_BLOB)
    {
	blob = argvars[0].vval.v_blob;
	if (blob == NULL)
	    return;
    }
    else
    {
	semsg(_(e_invalid_argument_str),
		_("writefile() first argument must be a List or a Blob"));
	return;
    }

    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	char_u *arg2 = tv_get_string_chk(&argvars[2]);

	if (arg2 == NULL)
	    return;
	if (vim_strchr(arg2, 'b') != NULL)
	    binary = TRUE;
	if (vim_strchr(arg2, 'a') != NULL)
	    append = TRUE;
	if (vim_strchr(arg2, 'D') != NULL)
	    defer = TRUE;
#ifdef HAVE_FSYNC
	if (vim_strchr(arg2, 's') != NULL)
	    do_fsync = TRUE;
	else if (vim_strchr(arg2, 'S') != NULL)
	    do_fsync = FALSE;
#endif
    }

    fname = tv_get_string_chk(&argvars[1]);
    if (fname == NULL)
	return;

    if (defer && !can_add_defer())
	return;

    // Always open the file in binary mode, library functions have a mind of
    // their own about CR-LF conversion.
    if (*fname == NUL || (fd = mch_fopen((char *)fname,
				      append ? APPENDBIN : WRITEBIN)) == NULL)
    {
	semsg(_(e_cant_create_file_str),
			       *fname == NUL ? (char_u *)_("<empty>") : fname);
	ret = -1;
    }
    else
    {
	if (defer)
	{
	    typval_T tv;

	    tv.v_type = VAR_STRING;
	    tv.v_lock = 0;
	    tv.vval.v_string = FullName_save(fname, FALSE);
	    if (tv.vval.v_string == NULL
		    || add_defer((char_u *)"delete", 1, &tv) == FAIL)
	    {
		ret = -1;
		fclose(fd);
		(void)mch_remove(fname);
	    }
	}

	if (ret == 0)
	{
	    if (blob)
	    {
		if (write_blob(fd, blob) == FAIL)
		    ret = -1;
	    }
	    else
	    {
		if (write_list(fd, list, binary) == FAIL)
		    ret = -1;
	    }
#ifdef HAVE_FSYNC
	    if (ret == 0 && do_fsync)
		// Ignore the error, the user wouldn't know what to do about
		// it.  May happen for a device.
		vim_ignored = vim_fsync(fileno(fd));
#endif
	    fclose(fd);
	}
    }

    rettv->vval.v_number = ret;
}

#endif // FEAT_EVAL

#if defined(FEAT_BROWSE) || defined(PROTO)
/*
 * Generic browse function.  Calls gui_mch_browse() when possible.
 * Later this may pop-up a non-GUI file selector (external command?).
 */
    char_u *
do_browse(
    int		flags,		// BROWSE_SAVE and BROWSE_DIR
    char_u	*title,		// title for the window
    char_u	*dflt,		// default file name (may include directory)
    char_u	*ext,		// extension added
    char_u	*initdir,	// initial directory, NULL for current dir or
				// when using path from "dflt"
    char_u	*filter,	// file name filter
    buf_T	*buf)		// buffer to read/write for
{
    char_u		*fname;
    static char_u	*last_dir = NULL;    // last used directory
    char_u		*tofree = NULL;
    int			save_cmod_flags = cmdmod.cmod_flags;

    // Must turn off browse to avoid that autocommands will get the
    // flag too!
    cmdmod.cmod_flags &= ~CMOD_BROWSE;

    if (title == NULL || *title == NUL)
    {
	if (flags & BROWSE_DIR)
	    title = (char_u *)_("Select Directory dialog");
	else if (flags & BROWSE_SAVE)
	    title = (char_u *)_("Save File dialog");
	else
	    title = (char_u *)_("Open File dialog");
    }

    // When no directory specified, use default file name, default dir, buffer
    // dir, last dir or current dir
    if ((initdir == NULL || *initdir == NUL) && dflt != NULL && *dflt != NUL)
    {
	if (mch_isdir(dflt))		// default file name is a directory
	{
	    initdir = dflt;
	    dflt = NULL;
	}
	else if (gettail(dflt) != dflt)	// default file name includes a path
	{
	    tofree = vim_strsave(dflt);
	    if (tofree != NULL)
	    {
		initdir = tofree;
		*gettail(initdir) = NUL;
		dflt = gettail(dflt);
	    }
	}
    }

    if (initdir == NULL || *initdir == NUL)
    {
	// When 'browsedir' is a directory, use it
	if (STRCMP(p_bsdir, "last") != 0
		&& STRCMP(p_bsdir, "buffer") != 0
		&& STRCMP(p_bsdir, "current") != 0
		&& mch_isdir(p_bsdir))
	    initdir = p_bsdir;
	// When saving or 'browsedir' is "buffer", use buffer fname
	else if (((flags & BROWSE_SAVE) || *p_bsdir == 'b')
		&& buf != NULL && buf->b_ffname != NULL)
	{
	    if (dflt == NULL || *dflt == NUL)
		dflt = gettail(curbuf->b_ffname);
	    tofree = vim_strsave(curbuf->b_ffname);
	    if (tofree != NULL)
	    {
		initdir = tofree;
		*gettail(initdir) = NUL;
	    }
	}
	// When 'browsedir' is "last", use dir from last browse
	else if (*p_bsdir == 'l')
	    initdir = last_dir;
	// When 'browsedir is "current", use current directory.  This is the
	// default already, leave initdir empty.
    }

# ifdef FEAT_GUI
    if (gui.in_use)		// when this changes, also adjust f_has()!
    {
	if (filter == NULL
#  ifdef FEAT_EVAL
		&& (filter = get_var_value((char_u *)"b:browsefilter")) == NULL
		&& (filter = get_var_value((char_u *)"g:browsefilter")) == NULL
#  endif
	)
	    filter = BROWSE_FILTER_DEFAULT;
	if (flags & BROWSE_DIR)
	{
#  if defined(FEAT_GUI_GTK) || defined(MSWIN)
	    // For systems that have a directory dialog.
	    fname = gui_mch_browsedir(title, initdir);
#  else
	    // Generic solution for selecting a directory: select a file and
	    // remove the file name.
	    fname = gui_mch_browse(0, title, dflt, ext, initdir, (char_u *)"");
#  endif
#  if !defined(FEAT_GUI_GTK)
	    // Win32 adds a dummy file name, others return an arbitrary file
	    // name.  GTK+ 2 returns only the directory,
	    if (fname != NULL && *fname != NUL && !mch_isdir(fname))
	    {
		// Remove the file name.
		char_u	    *tail = gettail_sep(fname);

		if (tail == fname)
		    *tail++ = '.';	// use current dir
		*tail = NUL;
	    }
#  endif
	}
	else
	    fname = gui_mch_browse(flags & BROWSE_SAVE,
			       title, dflt, ext, initdir, (char_u *)_(filter));

	// We hang around in the dialog for a while, the user might do some
	// things to our files.  The Win32 dialog allows deleting or renaming
	// a file, check timestamps.
	need_check_timestamps = TRUE;
	did_check_timestamps = FALSE;
    }
    else
# endif
    {
	// TODO: non-GUI file selector here
	emsg(_(e_sorry_no_file_browser_in_console_mode));
	fname = NULL;
    }

    // keep the directory for next time
    if (fname != NULL)
    {
	vim_free(last_dir);
	last_dir = vim_strsave(fname);
	if (last_dir != NULL && !(flags & BROWSE_DIR))
	{
	    *gettail(last_dir) = NUL;
	    if (*last_dir == NUL)
	    {
		// filename only returned, must be in current dir
		vim_free(last_dir);
		last_dir = alloc(MAXPATHL);
		if (last_dir != NULL)
		    mch_dirname(last_dir, MAXPATHL);
	    }
	}
    }

    vim_free(tofree);
    cmdmod.cmod_flags = save_cmod_flags;

    return fname;
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)

/*
 * "browse(save, title, initdir, default)" function
 */
    void
f_browse(typval_T *argvars UNUSED, typval_T *rettv)
{
# ifdef FEAT_BROWSE
    int		save;
    char_u	*title;
    char_u	*initdir;
    char_u	*defname;
    char_u	buf[NUMBUFLEN];
    char_u	buf2[NUMBUFLEN];
    int		error = FALSE;

    if (in_vim9script()
	    && (check_for_bool_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_string_arg(argvars, 2) == FAIL
		|| check_for_string_arg(argvars, 3) == FAIL))
	return;

    save = (int)tv_get_bool_chk(&argvars[0], &error);
    title = tv_get_string_chk(&argvars[1]);
    initdir = tv_get_string_buf_chk(&argvars[2], buf);
    defname = tv_get_string_buf_chk(&argvars[3], buf2);

    if (error || title == NULL || initdir == NULL || defname == NULL)
	rettv->vval.v_string = NULL;
    else
	rettv->vval.v_string =
		 do_browse(save ? BROWSE_SAVE : 0,
				 title, defname, NULL, initdir, NULL, curbuf);
# else
    rettv->vval.v_string = NULL;
# endif
    rettv->v_type = VAR_STRING;
}

/*
 * "browsedir(title, initdir)" function
 */
    void
f_browsedir(typval_T *argvars UNUSED, typval_T *rettv)
{
# ifdef FEAT_BROWSE
    char_u	*title;
    char_u	*initdir;
    char_u	buf[NUMBUFLEN];

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL))
	return;

    title = tv_get_string_chk(&argvars[0]);
    initdir = tv_get_string_buf_chk(&argvars[1], buf);

    if (title == NULL || initdir == NULL)
	rettv->vval.v_string = NULL;
    else
	rettv->vval.v_string = do_browse(BROWSE_DIR,
				    title, NULL, NULL, initdir, NULL, curbuf);
# else
    rettv->vval.v_string = NULL;
# endif
    rettv->v_type = VAR_STRING;
}

#endif // FEAT_EVAL

/*
 * Replace home directory by "~" in each space or comma separated file name in
 * 'src'.
 * If anything fails (except when out of space) dst equals src.
 */
    void
home_replace(
    buf_T	*buf,	// when not NULL, check for help files
    char_u	*src,	// input file name
    char_u	*dst,	// where to put the result
    int		dstlen,	// maximum length of the result
    int		one)	// if TRUE, only replace one file name, include
			// spaces and commas in the file name.
{
    size_t	dirlen = 0, envlen = 0;
    size_t	len;
    char_u	*homedir_env, *homedir_env_orig;
    char_u	*p;

    if (src == NULL)
    {
	*dst = NUL;
	return;
    }

    /*
     * If the file is a help file, remove the path completely.
     */
    if (buf != NULL && buf->b_help)
    {
	vim_snprintf((char *)dst, dstlen, "%s", gettail(src));
	return;
    }

    /*
     * We check both the value of the $HOME environment variable and the
     * "real" home directory.
     */
    if (homedir != NULL)
	dirlen = STRLEN(homedir);

#ifdef VMS
    homedir_env_orig = homedir_env = mch_getenv((char_u *)"SYS$LOGIN");
#else
    homedir_env_orig = homedir_env = mch_getenv((char_u *)"HOME");
#endif
#ifdef MSWIN
    if (homedir_env == NULL)
	homedir_env_orig = homedir_env = mch_getenv((char_u *)"USERPROFILE");
#endif
    // Empty is the same as not set.
    if (homedir_env != NULL && *homedir_env == NUL)
	homedir_env = NULL;

    if (homedir_env != NULL && *homedir_env == '~')
    {
	int	usedlen = 0;
	int	flen;
	char_u	*fbuf = NULL;

	flen = (int)STRLEN(homedir_env);
	(void)modify_fname((char_u *)":p", FALSE, &usedlen,
						  &homedir_env, &fbuf, &flen);
	flen = (int)STRLEN(homedir_env);
	if (flen > 0 && vim_ispathsep(homedir_env[flen - 1]))
	    // Remove the trailing / that is added to a directory.
	    homedir_env[flen - 1] = NUL;
    }

    if (homedir_env != NULL)
	envlen = STRLEN(homedir_env);

    if (!one)
	src = skipwhite(src);
    while (*src && dstlen > 0)
    {
	/*
	 * Here we are at the beginning of a file name.
	 * First, check to see if the beginning of the file name matches
	 * $HOME or the "real" home directory. Check that there is a '/'
	 * after the match (so that if e.g. the file is "/home/pieter/bla",
	 * and the home directory is "/home/piet", the file does not end up
	 * as "~er/bla" (which would seem to indicate the file "bla" in user
	 * er's home directory)).
	 */
	p = homedir;
	len = dirlen;
	for (;;)
	{
	    if (   len
		&& fnamencmp(src, p, len) == 0
		&& (vim_ispathsep(src[len])
		    || (!one && (src[len] == ',' || src[len] == ' '))
		    || src[len] == NUL))
	    {
		src += len;
		if (--dstlen > 0)
		    *dst++ = '~';

		// Do not add directory separator into dst, because dst is
		// expected to just return the directory name without the
		// directory separator '/'.
		break;
	    }
	    if (p == homedir_env)
		break;
	    p = homedir_env;
	    len = envlen;
	}

	// if (!one) skip to separator: space or comma
	while (*src && (one || (*src != ',' && *src != ' ')) && --dstlen > 0)
	    *dst++ = *src++;
	// skip separator
	while ((*src == ' ' || *src == ',') && --dstlen > 0)
	    *dst++ = *src++;
    }
    // if (dstlen == 0) out of space, what to do???

    *dst = NUL;

    if (homedir_env != homedir_env_orig)
	vim_free(homedir_env);
}

/*
 * Like home_replace, store the replaced string in allocated memory.
 * When something fails, NULL is returned.
 */
    char_u  *
home_replace_save(
    buf_T	*buf,	// when not NULL, check for help files
    char_u	*src)	// input file name
{
    char_u	*dst;
    unsigned	len;

    len = 3;			// space for "~/" and trailing NUL
    if (src != NULL)		// just in case
	len += (unsigned)STRLEN(src);
    dst = alloc(len);
    if (dst != NULL)
	home_replace(buf, src, dst, len, TRUE);
    return dst;
}

/*
 * Compare two file names and return:
 * FPC_SAME   if they both exist and are the same file.
 * FPC_SAMEX  if they both don't exist and have the same file name.
 * FPC_DIFF   if they both exist and are different files.
 * FPC_NOTX   if they both don't exist.
 * FPC_DIFFX  if one of them doesn't exist.
 * For the first name environment variables are expanded if "expandenv" is
 * TRUE.
 */
    int
fullpathcmp(
    char_u *s1,
    char_u *s2,
    int	    checkname,		// when both don't exist, check file names
    int	    expandenv)
{
#ifdef UNIX
    char_u	    exp1[MAXPATHL];
    char_u	    full1[MAXPATHL];
    char_u	    full2[MAXPATHL];
    stat_T	    st1, st2;
    int		    r1, r2;

    if (expandenv)
	expand_env(s1, exp1, MAXPATHL);
    else
	vim_strncpy(exp1, s1, MAXPATHL - 1);
    r1 = mch_stat((char *)exp1, &st1);
    r2 = mch_stat((char *)s2, &st2);
    if (r1 != 0 && r2 != 0)
    {
	// if mch_stat() doesn't work, may compare the names
	if (checkname)
	{
	    if (fnamecmp(exp1, s2) == 0)
		return FPC_SAMEX;
	    r1 = vim_FullName(exp1, full1, MAXPATHL, FALSE);
	    r2 = vim_FullName(s2, full2, MAXPATHL, FALSE);
	    if (r1 == OK && r2 == OK && fnamecmp(full1, full2) == 0)
		return FPC_SAMEX;
	}
	return FPC_NOTX;
    }
    if (r1 != 0 || r2 != 0)
	return FPC_DIFFX;
    if (st1.st_dev == st2.st_dev && st1.st_ino == st2.st_ino)
	return FPC_SAME;
    return FPC_DIFF;
#else
    char_u  *exp1;		// expanded s1
    char_u  *full1;		// full path of s1
    char_u  *full2;		// full path of s2
    int	    retval = FPC_DIFF;
    int	    r1, r2;

    // allocate one buffer to store three paths (alloc()/free() is slow!)
    if ((exp1 = alloc(MAXPATHL * 3)) != NULL)
    {
	full1 = exp1 + MAXPATHL;
	full2 = full1 + MAXPATHL;

	if (expandenv)
	    expand_env(s1, exp1, MAXPATHL);
	else
	    vim_strncpy(exp1, s1, MAXPATHL - 1);
	r1 = vim_FullName(exp1, full1, MAXPATHL, FALSE);
	r2 = vim_FullName(s2, full2, MAXPATHL, FALSE);

	// If vim_FullName() fails, the file probably doesn't exist.
	if (r1 != OK && r2 != OK)
	{
	    if (checkname && fnamecmp(exp1, s2) == 0)
		retval = FPC_SAMEX;
	    else
		retval = FPC_NOTX;
	}
	else if (r1 != OK || r2 != OK)
	    retval = FPC_DIFFX;
	else if (fnamecmp(full1, full2))
	    retval = FPC_DIFF;
	else
	    retval = FPC_SAME;
	vim_free(exp1);
    }
    return retval;
#endif
}

/*
 * Get the tail of a path: the file name.
 * When the path ends in a path separator the tail is the NUL after it.
 * Fail safe: never returns NULL.
 */
    char_u *
gettail(char_u *fname)
{
    char_u  *p1, *p2;

    if (fname == NULL)
	return (char_u *)"";
    for (p1 = p2 = get_past_head(fname); *p2; )	// find last part of path
    {
	if (vim_ispathsep_nocolon(*p2))
	    p1 = p2 + 1;
	MB_PTR_ADV(p2);
    }
    return p1;
}

/*
 * Get pointer to tail of "fname", including path separators.  Putting a NUL
 * here leaves the directory name.  Takes care of "c:/" and "//".
 * Always returns a valid pointer.
 */
    char_u *
gettail_sep(char_u *fname)
{
    char_u	*p;
    char_u	*t;

    p = get_past_head(fname);	// don't remove the '/' from "c:/file"
    t = gettail(fname);
    while (t > p && after_pathsep(fname, t))
	--t;
#ifdef VMS
    // path separator is part of the path
    ++t;
#endif
    return t;
}

/*
 * get the next path component (just after the next path separator).
 */
    char_u *
getnextcomp(char_u *fname)
{
    while (*fname && !vim_ispathsep(*fname))
	MB_PTR_ADV(fname);
    if (*fname)
	++fname;
    return fname;
}

/*
 * Get a pointer to one character past the head of a path name.
 * Unix: after "/"; DOS: after "c:\"; Amiga: after "disk:/"; Mac: no head.
 * If there is no head, path is returned.
 */
    char_u *
get_past_head(char_u *path)
{
    char_u  *retval;

#if defined(MSWIN)
    // may skip "c:"
    if (SAFE_isalpha(path[0]) && path[1] == ':')
	retval = path + 2;
    else
	retval = path;
#else
# if defined(AMIGA)
    // may skip "label:"
    retval = vim_strchr(path, ':');
    if (retval == NULL)
	retval = path;
# else	// Unix
    retval = path;
# endif
#endif

    while (vim_ispathsep(*retval))
	++retval;

    return retval;
}

/*
 * Return TRUE if 'c' is a path separator.
 * Note that for MS-Windows this includes the colon.
 */
    int
vim_ispathsep(int c)
{
#ifdef UNIX
    return (c == '/');	    // UNIX has ':' inside file names
#else
# ifdef BACKSLASH_IN_FILENAME
    return (c == ':' || c == '/' || c == '\\');
# else
#  ifdef VMS
    // server"user passwd"::device:[full.path.name]fname.extension;version"
    return (c == ':' || c == '[' || c == ']' || c == '/'
	    || c == '<' || c == '>' || c == '"' );
#  else
    return (c == ':' || c == '/');
#  endif // VMS
# endif
#endif
}

/*
 * Like vim_ispathsep(c), but exclude the colon for MS-Windows.
 */
    int
vim_ispathsep_nocolon(int c)
{
    return vim_ispathsep(c)
#ifdef BACKSLASH_IN_FILENAME
	&& c != ':'
#endif
	;
}

/*
 * Return TRUE if the directory of "fname" exists, FALSE otherwise.
 * Also returns TRUE if there is no directory name.
 * "fname" must be writable!.
 */
    int
dir_of_file_exists(char_u *fname)
{
    char_u	*p;
    int		c;
    int		retval;

    p = gettail_sep(fname);
    if (p == fname)
	return TRUE;
    c = *p;
    *p = NUL;
    retval = mch_isdir(fname);
    *p = c;
    return retval;
}

/*
 * Versions of fnamecmp() and fnamencmp() that handle '/' and '\' equally
 * and deal with 'fileignorecase'.
 */
    int
vim_fnamecmp(char_u *x, char_u *y)
{
#ifdef BACKSLASH_IN_FILENAME
    return vim_fnamencmp(x, y, MAXPATHL);
#else
    if (p_fic)
	return MB_STRICMP(x, y);
    return STRCMP(x, y);
#endif
}

    int
vim_fnamencmp(char_u *x, char_u *y, size_t len)
{
#ifdef BACKSLASH_IN_FILENAME
    char_u	*px = x;
    char_u	*py = y;
    int		cx = NUL;
    int		cy = NUL;

    while (len > 0)
    {
	cx = PTR2CHAR(px);
	cy = PTR2CHAR(py);
	if (cx == NUL || cy == NUL
	    || ((p_fic ? MB_TOLOWER(cx) != MB_TOLOWER(cy) : cx != cy)
		&& !(cx == '/' && cy == '\\')
		&& !(cx == '\\' && cy == '/')))
	    break;
	len -= mb_ptr2len(px);
	px += mb_ptr2len(px);
	py += mb_ptr2len(py);
    }
    if (len == 0)
	return 0;
    return (cx - cy);
#else
    if (p_fic)
	return MB_STRNICMP(x, y, len);
    return STRNCMP(x, y, len);
#endif
}

/*
 * Concatenate file names fname1 and fname2 into allocated memory.
 * Only add a '/' or '\\' when 'sep' is TRUE and it is necessary.
 */
    char_u  *
concat_fnames(char_u *fname1, char_u *fname2, int sep)
{
    char_u  *dest;

    dest = alloc(STRLEN(fname1) + STRLEN(fname2) + 3);
    if (dest == NULL)
	return NULL;

    STRCPY(dest, fname1);
    if (sep)
	add_pathsep(dest);
    STRCAT(dest, fname2);
    return dest;
}

/*
 * Add a path separator to a file name, unless it already ends in a path
 * separator.
 */
    void
add_pathsep(char_u *p)
{
    if (*p != NUL && !after_pathsep(p, p + STRLEN(p)))
	STRCAT(p, PATHSEPSTR);
}

/*
 * FullName_save - Make an allocated copy of a full file name.
 * Returns NULL when out of memory.
 */
    char_u  *
FullName_save(
    char_u	*fname,
    int		force)		// force expansion, even when it already looks
				// like a full path name
{
    char_u	*buf;
    char_u	*new_fname = NULL;

    if (fname == NULL)
	return NULL;

    buf = alloc(MAXPATHL);
    if (buf == NULL)
	return NULL;

    if (vim_FullName(fname, buf, MAXPATHL, force) != FAIL)
	new_fname = vim_strsave(buf);
    else
	new_fname = vim_strsave(fname);
    vim_free(buf);
    return new_fname;
}

/*
 * return TRUE if "fname" exists.
 */
    int
vim_fexists(char_u *fname)
{
    stat_T st;

    if (mch_stat((char *)fname, &st))
	return FALSE;
    return TRUE;
}

/*
 * Invoke expand_wildcards() for one pattern.
 * Expand items like "%:h" before the expansion.
 * Returns OK or FAIL.
 */
    int
expand_wildcards_eval(
    char_u	 **pat,		// pointer to input pattern
    int		  *num_file,	// resulting number of files
    char_u	***file,	// array of resulting files
    int		   flags)	// EW_DIR, etc.
{
    int		ret = FAIL;
    char_u	*eval_pat = NULL;
    char_u	*exp_pat = *pat;
    char	*ignored_msg;
    int		usedlen;
    int		is_cur_alt_file = *exp_pat == '%' || *exp_pat == '#';
    int		star_follows = FALSE;

    if (is_cur_alt_file || *exp_pat == '<')
    {
	++emsg_off;
	eval_pat = eval_vars(exp_pat, exp_pat, &usedlen,
					       NULL, &ignored_msg, NULL, TRUE);
	--emsg_off;
	if (eval_pat != NULL)
	{
	    star_follows = STRCMP(exp_pat + usedlen, "*") == 0;
	    exp_pat = concat_str(eval_pat, exp_pat + usedlen);
	}
    }

    if (exp_pat != NULL)
	ret = expand_wildcards(1, &exp_pat, num_file, file, flags);

    if (eval_pat != NULL)
    {
	if (*num_file == 0 && is_cur_alt_file && star_follows)
	{
	    // Expanding "%" or "#" and the file does not exist: Add the
	    // pattern anyway (without the star) so that this works for remote
	    // files and non-file buffer names.
	    *file = ALLOC_ONE(char_u *);
	    if (*file != NULL)
	    {
		**file = eval_pat;
		eval_pat = NULL;
		*num_file = 1;
		ret = OK;
	    }
	}
	vim_free(exp_pat);
	vim_free(eval_pat);
    }

    return ret;
}

/*
 * Expand wildcards.  Calls gen_expand_wildcards() and removes files matching
 * 'wildignore'.
 * Returns OK or FAIL.  When FAIL then "num_files" won't be set.
 */
    int
expand_wildcards(
    int		   num_pat,	// number of input patterns
    char_u	 **pat,		// array of input patterns
    int		  *num_files,	// resulting number of files
    char_u	***files,	// array of resulting files
    int		   flags)	// EW_DIR, etc.
{
    int		retval;
    int		i, j;
    char_u	*p;
    int		non_suf_match;	// number without matching suffix

    retval = gen_expand_wildcards(num_pat, pat, num_files, files, flags);

    // When keeping all matches, return here
    if ((flags & EW_KEEPALL) || retval == FAIL)
	return retval;

    /*
     * Remove names that match 'wildignore'.
     */
    if (*p_wig)
    {
	char_u	*ffname;

	// check all files in (*files)[]
	for (i = 0; i < *num_files; ++i)
	{
	    ffname = FullName_save((*files)[i], FALSE);
	    if (ffname == NULL)		// out of memory
		break;
# ifdef VMS
	    vms_remove_version(ffname);
# endif
	    if (match_file_list(p_wig, (*files)[i], ffname))
	    {
		// remove this matching file from the list
		vim_free((*files)[i]);
		for (j = i; j + 1 < *num_files; ++j)
		    (*files)[j] = (*files)[j + 1];
		--*num_files;
		--i;
	    }
	    vim_free(ffname);
	}

	// If the number of matches is now zero, we fail.
	if (*num_files == 0)
	{
	    VIM_CLEAR(*files);
	    return FAIL;
	}
    }

    /*
     * Move the names where 'suffixes' match to the end.
     * Skip when interrupted, the result probably won't be used.
     */
    if (*num_files > 1 && !got_int)
    {
	non_suf_match = 0;
	for (i = 0; i < *num_files; ++i)
	{
	    if (!match_suffix((*files)[i]))
	    {
		/*
		 * Move the name without matching suffix to the front
		 * of the list.
		 */
		p = (*files)[i];
		for (j = i; j > non_suf_match; --j)
		    (*files)[j] = (*files)[j - 1];
		(*files)[non_suf_match++] = p;
	    }
	}
    }

    return retval;
}

/*
 * Return TRUE if "fname" matches with an entry in 'suffixes'.
 */
    int
match_suffix(char_u *fname)
{
    int		fnamelen, setsuflen;
    char_u	*setsuf;
#define MAXSUFLEN 30	    // maximum length of a file suffix
    char_u	suf_buf[MAXSUFLEN];

    fnamelen = (int)STRLEN(fname);
    setsuflen = 0;
    for (setsuf = p_su; *setsuf; )
    {
	setsuflen = copy_option_part(&setsuf, suf_buf, MAXSUFLEN, ".,");
	if (setsuflen == 0)
	{
	    char_u *tail = gettail(fname);

	    // empty entry: match name without a '.'
	    if (vim_strchr(tail, '.') == NULL)
	    {
		setsuflen = 1;
		break;
	    }
	}
	else
	{
	    if (fnamelen >= setsuflen
		    && fnamencmp(suf_buf, fname + fnamelen - setsuflen,
						  (size_t)setsuflen) == 0)
		break;
	    setsuflen = 0;
	}
    }
    return (setsuflen != 0);
}

#ifdef VIM_BACKTICK

/*
 * Return TRUE if we can expand this backtick thing here.
 */
    static int
vim_backtick(char_u *p)
{
    return (*p == '`' && *(p + 1) != NUL && *(p + STRLEN(p) - 1) == '`');
}

/*
 * Expand an item in `backticks` by executing it as a command.
 * Currently only works when pat[] starts and ends with a `.
 * Returns number of file names found, -1 if an error is encountered.
 */
    static int
expand_backtick(
    garray_T	*gap,
    char_u	*pat,
    int		flags)	// EW_* flags
{
    char_u	*p;
    char_u	*cmd;
    char_u	*buffer;
    int		cnt = 0;
    int		i;

    // Create the command: lop off the backticks.
    cmd = vim_strnsave(pat + 1, STRLEN(pat) - 2);
    if (cmd == NULL)
	return -1;

#ifdef FEAT_EVAL
    if (*cmd == '=')	    // `={expr}`: Expand expression
	buffer = eval_to_string(cmd + 1, TRUE, FALSE);
    else
#endif
	buffer = get_cmd_output(cmd, NULL,
				(flags & EW_SILENT) ? SHELL_SILENT : 0, NULL);
    vim_free(cmd);
    if (buffer == NULL)
	return -1;

    cmd = buffer;
    while (*cmd != NUL)
    {
	cmd = skipwhite(cmd);		// skip over white space
	p = cmd;
	while (*p != NUL && *p != '\r' && *p != '\n') // skip over entry
	    ++p;
	// add an entry if it is not empty
	if (p > cmd)
	{
	    i = *p;
	    *p = NUL;
	    addfile(gap, cmd, flags);
	    *p = i;
	    ++cnt;
	}
	cmd = p;
	while (*cmd != NUL && (*cmd == '\r' || *cmd == '\n'))
	    ++cmd;
    }

    vim_free(buffer);
    return cnt;
}
#endif // VIM_BACKTICK

#if defined(MSWIN)
/*
 * File name expansion code for MS-DOS, Win16 and Win32.  It's here because
 * it's shared between these systems.
 */

/*
 * comparison function for qsort in dos_expandpath()
 */
    static int
pstrcmp(const void *a, const void *b)
{
    return (pathcmp(*(char **)a, *(char **)b, -1));
}

/*
 * Recursively expand one path component into all matching files and/or
 * directories.  Adds matches to "gap".  Handles "*", "?", "[a-z]", "**", etc.
 * Return the number of matches found.
 * "path" has backslashes before chars that are not to be expanded, starting
 * at "path[wildoff]".
 * Return the number of matches found.
 * NOTE: much of this is identical to unix_expandpath(), keep in sync!
 */
    static int
dos_expandpath(
    garray_T	*gap,
    char_u	*path,
    int		wildoff,
    int		flags,		// EW_* flags
    int		didstar)	// expanded "**" once already
{
    char_u	*buf;
    char_u	*path_end;
    char_u	*p, *s, *e;
    int		start_len = gap->ga_len;
    char_u	*pat;
    regmatch_T	regmatch;
    int		starts_with_dot;
    int		matches;
    int		len;
    int		starstar = FALSE;
    static int	stardepth = 0;	    // depth for "**" expansion
    HANDLE		hFind = INVALID_HANDLE_VALUE;
    WIN32_FIND_DATAW    wfb;
    WCHAR		*wn = NULL;	// UCS-2 name, NULL when not used.
    char_u		*matchname;
    int			ok;
    char_u		*p_alt;

    // Expanding "**" may take a long time, check for CTRL-C.
    if (stardepth > 0)
    {
	ui_breakcheck();
	if (got_int)
	    return 0;
    }

    // Make room for file name.  When doing encoding conversion the actual
    // length may be quite a bit longer, thus use the maximum possible length.
    buf = alloc(MAXPATHL);
    if (buf == NULL)
	return 0;

    /*
     * Find the first part in the path name that contains a wildcard or a ~1.
     * Copy it into buf, including the preceding characters.
     */
    p = buf;
    s = buf;
    e = NULL;
    path_end = path;
    while (*path_end != NUL)
    {
	// May ignore a wildcard that has a backslash before it; it will
	// be removed by rem_backslash() or file_pat_to_reg_pat() below.
	if (path_end >= path + wildoff && rem_backslash(path_end))
	    *p++ = *path_end++;
	else if (*path_end == '\\' || *path_end == ':' || *path_end == '/')
	{
	    if (e != NULL)
		break;
	    s = p + 1;
	}
	else if (path_end >= path + wildoff
			 && vim_strchr((char_u *)"*?[~", *path_end) != NULL)
	    e = p;
	if (has_mbyte)
	{
	    len = (*mb_ptr2len)(path_end);
	    STRNCPY(p, path_end, len);
	    p += len;
	    path_end += len;
	}
	else
	    *p++ = *path_end++;
    }
    e = p;
    *e = NUL;

    // now we have one wildcard component between s and e
    // Remove backslashes between "wildoff" and the start of the wildcard
    // component.
    for (p = buf + wildoff; p < s; ++p)
	if (rem_backslash(p))
	{
	    STRMOVE(p, p + 1);
	    --e;
	    --s;
	}

    // Check for "**" between "s" and "e".
    for (p = s; p < e; ++p)
	if (p[0] == '*' && p[1] == '*')
	    starstar = TRUE;

    starts_with_dot = *s == '.';
    pat = file_pat_to_reg_pat(s, e, NULL, FALSE);
    if (pat == NULL)
    {
	vim_free(buf);
	return 0;
    }

    // compile the regexp into a program
    if (flags & (EW_NOERROR | EW_NOTWILD))
	++emsg_silent;
    regmatch.rm_ic = TRUE;		// Always ignore case
    regmatch.regprog = vim_regcomp(pat, RE_MAGIC);
    if (flags & (EW_NOERROR | EW_NOTWILD))
	--emsg_silent;
    vim_free(pat);

    if (regmatch.regprog == NULL && (flags & EW_NOTWILD) == 0)
    {
	vim_free(buf);
	return 0;
    }

    // remember the pattern or file name being looked for
    matchname = vim_strsave(s);

    // If "**" is by itself, this is the first time we encounter it and more
    // is following then find matches without any directory.
    if (!didstar && stardepth < 100 && starstar && e - s == 2
							  && *path_end == '/')
    {
	STRCPY(s, path_end + 1);
	++stardepth;
	(void)dos_expandpath(gap, buf, (int)(s - buf), flags, TRUE);
	--stardepth;
    }

    // Scan all files in the directory with "dir/ *.*"
    STRCPY(s, "*.*");
    wn = enc_to_utf16(buf, NULL);
    if (wn != NULL)
	hFind = FindFirstFileW(wn, &wfb);
    ok = (hFind != INVALID_HANDLE_VALUE);

    while (ok)
    {
	p = utf16_to_enc(wfb.cFileName, NULL);   // p is allocated here

	if (p == NULL)
	    break;  // out of memory

	// Do not use the alternate filename when the file name ends in '~',
	// because it picks up backup files: short name for "foo.vim~" is
	// "foo~1.vim", which matches "*.vim".
	if (*wfb.cAlternateFileName == NUL || p[STRLEN(p) - 1] == '~')
	    p_alt = NULL;
	else
	    p_alt = utf16_to_enc(wfb.cAlternateFileName, NULL);

	// Ignore entries starting with a dot, unless when asked for.  Accept
	// all entries found with "matchname".
	if ((p[0] != '.' || starts_with_dot
			 || ((flags & EW_DODOT)
			     && p[1] != NUL && (p[1] != '.' || p[2] != NUL)))
		&& (matchname == NULL
		  || (regmatch.regprog != NULL
		      && (vim_regexec(&regmatch, p, (colnr_T)0)
			 || (p_alt != NULL
				&& vim_regexec(&regmatch, p_alt, (colnr_T)0))))
		  || ((flags & EW_NOTWILD)
		     && fnamencmp(path + (s - buf), p, e - s) == 0)))
	{
	    STRCPY(s, p);
	    len = (int)STRLEN(buf);

	    if (starstar && stardepth < 100
			  && (wfb.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY))
	    {
		// For "**" in the pattern first go deeper in the tree to
		// find matches.
		STRCPY(buf + len, "/**");
		STRCPY(buf + len + 3, path_end);
		++stardepth;
		(void)dos_expandpath(gap, buf, len + 1, flags, TRUE);
		--stardepth;
	    }

	    STRCPY(buf + len, path_end);
	    if (mch_has_exp_wildcard(path_end))
	    {
		// need to expand another component of the path
		// remove backslashes for the remaining components only
		(void)dos_expandpath(gap, buf, len + 1, flags, FALSE);
	    }
	    else
	    {
		// no more wildcards, check if there is a match
		// remove backslashes for the remaining components only
		if (*path_end != 0)
		    backslash_halve(buf + len + 1);
		if (mch_getperm(buf) >= 0)	// add existing file
		    addfile(gap, buf, flags);
	    }
	}

	vim_free(p_alt);
	vim_free(p);
	ok = FindNextFileW(hFind, &wfb);
    }

    FindClose(hFind);
    vim_free(wn);
    vim_free(buf);
    vim_regfree(regmatch.regprog);
    vim_free(matchname);

    matches = gap->ga_len - start_len;
    if (matches > 0)
	qsort(((char_u **)gap->ga_data) + start_len, (size_t)matches,
						   sizeof(char_u *), pstrcmp);
    return matches;
}

    int
mch_expandpath(
    garray_T	*gap,
    char_u	*path,
    int		flags)		// EW_* flags
{
    return dos_expandpath(gap, path, 0, flags, FALSE);
}
#endif // MSWIN

#if (defined(UNIX) && !defined(VMS)) || defined(USE_UNIXFILENAME) \
	|| defined(PROTO)
/*
 * Unix style wildcard expansion code.
 * It's here because it's used both for Unix and Mac.
 */
    static int
pstrcmp(const void *a, const void *b)
{
    return (pathcmp(*(char **)a, *(char **)b, -1));
}

/*
 * Recursively expand one path component into all matching files and/or
 * directories.  Adds matches to "gap".  Handles "*", "?", "[a-z]", "**", etc.
 * "path" has backslashes before chars that are not to be expanded, starting
 * at "path + wildoff".
 * Return the number of matches found.
 * NOTE: much of this is identical to dos_expandpath(), keep in sync!
 */
    int
unix_expandpath(
    garray_T	*gap,
    char_u	*path,
    int		wildoff,
    int		flags,		// EW_* flags
    int		didstar)	// expanded "**" once already
{
    char_u	*buf;
    char_u	*path_end;
    char_u	*p, *s, *e;
    int		start_len = gap->ga_len;
    char_u	*pat;
    regmatch_T	regmatch;
    int		starts_with_dot;
    int		matches;
    int		len;
    int		starstar = FALSE;
    static int	stardepth = 0;	    // depth for "**" expansion

    DIR		*dirp;
    struct dirent *dp;

    // Expanding "**" may take a long time, check for CTRL-C.
    if (stardepth > 0)
    {
	ui_breakcheck();
	if (got_int)
	    return 0;
    }

    // make room for file name (a bit too much to stay on the safe side)
    size_t buflen = STRLEN(path) + MAXPATHL;
    buf = alloc(buflen);
    if (buf == NULL)
	return 0;

    /*
     * Find the first part in the path name that contains a wildcard.
     * When EW_ICASE is set every letter is considered to be a wildcard.
     * Copy it into "buf", including the preceding characters.
     */
    p = buf;
    s = buf;
    e = NULL;
    path_end = path;
    while (*path_end != NUL)
    {
	// May ignore a wildcard that has a backslash before it; it will
	// be removed by rem_backslash() or file_pat_to_reg_pat() below.
	if (path_end >= path + wildoff && rem_backslash(path_end))
	    *p++ = *path_end++;
	else if (*path_end == '/')
	{
	    if (e != NULL)
		break;
	    s = p + 1;
	}
	else if (path_end >= path + wildoff
			 && (vim_strchr((char_u *)"*?[{~$", *path_end) != NULL
			     || (!p_fic && (flags & EW_ICASE)
					  && vim_isalpha(PTR2CHAR(path_end)))))
	    e = p;
	if (has_mbyte)
	{
	    len = (*mb_ptr2len)(path_end);
	    STRNCPY(p, path_end, len);
	    p += len;
	    path_end += len;
	}
	else
	    *p++ = *path_end++;
    }
    e = p;
    *e = NUL;

    // Now we have one wildcard component between "s" and "e".
    // Remove backslashes between "wildoff" and the start of the wildcard
    // component.
    for (p = buf + wildoff; p < s; ++p)
	if (rem_backslash(p))
	{
	    STRMOVE(p, p + 1);
	    --e;
	    --s;
	}

    // Check for "**" between "s" and "e".
    for (p = s; p < e; ++p)
	if (p[0] == '*' && p[1] == '*')
	    starstar = TRUE;

    // convert the file pattern to a regexp pattern
    starts_with_dot = *s == '.';
    pat = file_pat_to_reg_pat(s, e, NULL, FALSE);
    if (pat == NULL)
    {
	vim_free(buf);
	return 0;
    }

    // compile the regexp into a program
    if (flags & EW_ICASE)
	regmatch.rm_ic = TRUE;		// 'wildignorecase' set
    else
	regmatch.rm_ic = p_fic;	// ignore case when 'fileignorecase' is set
    if (flags & (EW_NOERROR | EW_NOTWILD))
	++emsg_silent;
    regmatch.regprog = vim_regcomp(pat, RE_MAGIC);
    if (flags & (EW_NOERROR | EW_NOTWILD))
	--emsg_silent;
    vim_free(pat);

    if (regmatch.regprog == NULL && (flags & EW_NOTWILD) == 0)
    {
	vim_free(buf);
	return 0;
    }

    // If "**" is by itself, this is the first time we encounter it and more
    // is following then find matches without any directory.
    if (!didstar && stardepth < 100 && starstar && e - s == 2
							  && *path_end == '/')
    {
	STRCPY(s, path_end + 1);
	++stardepth;
	(void)unix_expandpath(gap, buf, (int)(s - buf), flags, TRUE);
	--stardepth;
    }

    // open the directory for scanning
    *s = NUL;
    dirp = opendir(*buf == NUL ? "." : (char *)buf);

    // Find all matching entries
    if (dirp != NULL)
    {
	while (!got_int)
	{
	    dp = readdir(dirp);
	    if (dp == NULL)
		break;
	    if ((dp->d_name[0] != '.' || starts_with_dot
			|| ((flags & EW_DODOT)
			    && dp->d_name[1] != NUL
			    && (dp->d_name[1] != '.' || dp->d_name[2] != NUL)))
		 && ((regmatch.regprog != NULL && vim_regexec(&regmatch,
					     (char_u *)dp->d_name, (colnr_T)0))
		   || ((flags & EW_NOTWILD)
		     && fnamencmp(path + (s - buf), dp->d_name, e - s) == 0)))
	    {
		vim_strncpy(s, (char_u *)dp->d_name, buflen - (s - buf) - 1);
		len = STRLEN(buf);

		if (starstar && stardepth < 100)
		{
		    // For "**" in the pattern first go deeper in the tree to
		    // find matches.
		    vim_snprintf((char *)buf + len, buflen - len,
							    "/**%s", path_end);
		    ++stardepth;
		    (void)unix_expandpath(gap, buf, len + 1, flags, TRUE);
		    --stardepth;
		}

		vim_snprintf((char *)buf + len, buflen - len, "%s", path_end);
		if (mch_has_exp_wildcard(path_end)) // handle more wildcards
		{
		    // need to expand another component of the path
		    // remove backslashes for the remaining components only
		    (void)unix_expandpath(gap, buf, len + 1, flags, FALSE);
		}
		else
		{
		    stat_T  sb;

		    // no more wildcards, check if there is a match
		    // remove backslashes for the remaining components only
		    if (*path_end != NUL)
			backslash_halve(buf + len + 1);
		    // add existing file or symbolic link
		    if ((flags & EW_ALLLINKS) ? mch_lstat((char *)buf, &sb) >= 0
						      : mch_getperm(buf) >= 0)
		    {
#ifdef MACOS_CONVERT
			size_t precomp_len = STRLEN(buf)+1;
			char_u *precomp_buf =
			    mac_precompose_path(buf, precomp_len, &precomp_len);

			if (precomp_buf)
			{
			    mch_memmove(buf, precomp_buf, precomp_len);
			    vim_free(precomp_buf);
			}
#endif
			addfile(gap, buf, flags);
		    }
		}
	    }
	}

	closedir(dirp);
    }

    vim_free(buf);
    vim_regfree(regmatch.regprog);

    // When interrupted the matches probably won't be used and sorting can be
    // slow, thus skip it.
    matches = gap->ga_len - start_len;
    if (matches > 0 && !got_int)
	qsort(((char_u **)gap->ga_data) + start_len, matches,
						   sizeof(char_u *), pstrcmp);
    return matches;
}
#endif

/*
 * Return TRUE if "p" contains what looks like an environment variable.
 * Allowing for escaping.
 */
    static int
has_env_var(char_u *p)
{
    for ( ; *p; MB_PTR_ADV(p))
    {
	if (*p == '\\' && p[1] != NUL)
	    ++p;
	else if (vim_strchr((char_u *)
#if defined(MSWIN)
				    "$%"
#else
				    "$"
#endif
					, *p) != NULL)
	    return TRUE;
    }
    return FALSE;
}

#ifdef SPECIAL_WILDCHAR
/*
 * Return TRUE if "p" contains a special wildcard character, one that Vim
 * cannot expand, requires using a shell.
 */
    static int
has_special_wildchar(char_u *p)
{
    for ( ; *p; MB_PTR_ADV(p))
    {
	// Disallow line break characters.
	if (*p == '\r' || *p == '\n')
	    break;
	// Allow for escaping.
	if (*p == '\\' && p[1] != NUL && p[1] != '\r' && p[1] != '\n')
	    ++p;
	else if (vim_strchr((char_u *)SPECIAL_WILDCHAR, *p) != NULL)
	{
	    // A { must be followed by a matching }.
	    if (*p == '{' && vim_strchr(p, '}') == NULL)
		continue;
	    // A quote and backtick must be followed by another one.
	    if ((*p == '`' || *p == '\'') && vim_strchr(p, *p) == NULL)
		continue;
	    return TRUE;
	}
    }
    return FALSE;
}
#endif

/*
 * Generic wildcard expansion code.
 *
 * Characters in "pat" that should not be expanded must be preceded with a
 * backslash. E.g., "/path\ with\ spaces/my\*star*"
 *
 * Return FAIL when no single file was found.  In this case "num_file" is not
 * set, and "file" may contain an error message.
 * Return OK when some files found.  "num_file" is set to the number of
 * matches, "file" to the array of matches.  Call FreeWild() later.
 */
    int
gen_expand_wildcards(
    int		num_pat,	// number of input patterns
    char_u	**pat,		// array of input patterns
    int		*num_file,	// resulting number of files
    char_u	***file,	// array of resulting files
    int		flags)		// EW_* flags
{
    int			i;
    garray_T		ga;
    char_u		*p;
    static int		recursive = FALSE;
    int			add_pat;
    int			retval = OK;
    int			did_expand_in_path = FALSE;

    /*
     * expand_env() is called to expand things like "~user".  If this fails,
     * it calls ExpandOne(), which brings us back here.  In this case, always
     * call the machine specific expansion function, if possible.  Otherwise,
     * return FAIL.
     */
    if (recursive)
#ifdef SPECIAL_WILDCHAR
	return mch_expand_wildcards(num_pat, pat, num_file, file, flags);
#else
	return FAIL;
#endif

#ifdef SPECIAL_WILDCHAR
    /*
     * If there are any special wildcard characters which we cannot handle
     * here, call machine specific function for all the expansion.  This
     * avoids starting the shell for each argument separately.
     * For `=expr` do use the internal function.
     */
    for (i = 0; i < num_pat; i++)
    {
	if (has_special_wildchar(pat[i])
# ifdef VIM_BACKTICK
		&& !(vim_backtick(pat[i]) && pat[i][1] == '=')
# endif
	   )
	    return mch_expand_wildcards(num_pat, pat, num_file, file, flags);
    }
#endif

    recursive = TRUE;

    /*
     * The matching file names are stored in a growarray.  Init it empty.
     */
    ga_init2(&ga, sizeof(char_u *), 30);

    for (i = 0; i < num_pat && !got_int; ++i)
    {
	add_pat = -1;
	p = pat[i];

#ifdef VIM_BACKTICK
	if (vim_backtick(p))
	{
	    add_pat = expand_backtick(&ga, p, flags);
	    if (add_pat == -1)
		retval = FAIL;
	}
	else
#endif
	{
	    /*
	     * First expand environment variables, "~/" and "~user/".
	     */
	    if ((has_env_var(p) && !(flags & EW_NOTENV)) || *p == '~')
	    {
		p = expand_env_save_opt(p, TRUE);
		if (p == NULL)
		    p = pat[i];
#ifdef UNIX
		/*
		 * On Unix, if expand_env() can't expand an environment
		 * variable, use the shell to do that.  Discard previously
		 * found file names and start all over again.
		 */
		else if (has_env_var(p) || *p == '~')
		{
		    vim_free(p);
		    ga_clear_strings(&ga);
		    i = mch_expand_wildcards(num_pat, pat, num_file, file,
							 flags|EW_KEEPDOLLAR);
		    recursive = FALSE;
		    return i;
		}
#endif
	    }

	    /*
	     * If there are wildcards or case-insensitive expansion is
	     * required: Expand file names and add each match to the list.  If
	     * there is no match, and EW_NOTFOUND is given, add the pattern.
	     * Otherwise: Add the file name if it exists or when EW_NOTFOUND is
	     * given.
	     */
	    if (mch_has_exp_wildcard(p) || (flags & EW_ICASE))
	    {
		if ((flags & EW_PATH)
			&& !mch_isFullName(p)
			&& !(p[0] == '.'
			    && (vim_ispathsep(p[1])
				|| (p[1] == '.' && vim_ispathsep(p[2]))))
		   )
		{
		    // :find completion where 'path' is used.
		    // Recursiveness is OK here.
		    recursive = FALSE;
		    add_pat = expand_in_path(&ga, p, flags);
		    recursive = TRUE;
		    did_expand_in_path = TRUE;
		}
		else
		    add_pat = mch_expandpath(&ga, p, flags);
	    }
	}

	if (add_pat == -1 || (add_pat == 0 && (flags & EW_NOTFOUND)))
	{
	    char_u	*t = backslash_halve_save(p);

	    // When EW_NOTFOUND is used, always add files and dirs.  Makes
	    // "vim c:/" work.
	    if (flags & EW_NOTFOUND)
		addfile(&ga, t, flags | EW_DIR | EW_FILE);
	    else
		addfile(&ga, t, flags);

	    if (t != p)
		vim_free(t);
	}

	if (did_expand_in_path && ga.ga_len > 0 && (flags & EW_PATH))
	    uniquefy_paths(&ga, p);
	if (p != pat[i])
	    vim_free(p);
    }

    // When returning FAIL the array must be freed here.
    if (retval == FAIL)
	ga_clear_strings(&ga);

    *num_file = ga.ga_len;
    *file = (ga.ga_data != NULL) ? (char_u **)ga.ga_data
						  : (char_u **)_("no matches");

    recursive = FALSE;

    return ((flags & EW_EMPTYOK) || ga.ga_data != NULL) ? retval : FAIL;
}

/*
 * Add a file to a file list.  Accepted flags:
 * EW_DIR	add directories
 * EW_FILE	add files
 * EW_EXEC	add executable files
 * EW_NOTFOUND	add even when it doesn't exist
 * EW_ADDSLASH	add slash after directory name
 * EW_ALLLINKS	add symlink also when the referred file does not exist
 */
    void
addfile(
    garray_T	*gap,
    char_u	*f,	// filename
    int		flags)
{
    char_u	*p;
    int		isdir;
    stat_T	sb;

    // if the file/dir/link doesn't exist, may not add it
    if (!(flags & EW_NOTFOUND) && ((flags & EW_ALLLINKS)
			? mch_lstat((char *)f, &sb) < 0 : mch_getperm(f) < 0))
	return;

#ifdef FNAME_ILLEGAL
    // if the file/dir contains illegal characters, don't add it
    if (vim_strpbrk(f, (char_u *)FNAME_ILLEGAL) != NULL)
	return;
#endif

    isdir = mch_isdir(f);
    if ((isdir && !(flags & EW_DIR)) || (!isdir && !(flags & EW_FILE)))
	return;

    // If the file isn't executable, may not add it.  Do accept directories.
    // When invoked from expand_shellcmd() do not use $PATH.
    if (!isdir && (flags & EW_EXEC)
			     && !mch_can_exe(f, NULL, !(flags & EW_SHELLCMD)))
	return;

    // Make room for another item in the file list.
    if (ga_grow(gap, 1) == FAIL)
	return;

    p = alloc(STRLEN(f) + 1 + isdir);
    if (p == NULL)
	return;

    STRCPY(p, f);
#ifdef BACKSLASH_IN_FILENAME
    slash_adjust(p);
#endif
    /*
     * Append a slash or backslash after directory names if none is present.
     */
    if (isdir && (flags & EW_ADDSLASH))
	add_pathsep(p);
    ((char_u **)gap->ga_data)[gap->ga_len++] = p;
}

/*
 * Free the list of files returned by expand_wildcards() or other expansion
 * functions.
 */
    void
FreeWild(int count, char_u **files)
{
    if (count <= 0 || files == NULL)
	return;
    while (count--)
	vim_free(files[count]);
    vim_free(files);
}

/*
 * Compare path "p[]" to "q[]".
 * If "maxlen" >= 0 compare "p[maxlen]" to "q[maxlen]"
 * Return value like strcmp(p, q), but consider path separators.
 */
    int
pathcmp(const char *p, const char *q, int maxlen)
{
    int		i, j;
    int		c1, c2;
    const char	*s = NULL;

    for (i = 0, j = 0; maxlen < 0 || (i < maxlen && j < maxlen);)
    {
	c1 = PTR2CHAR((char_u *)p + i);
	c2 = PTR2CHAR((char_u *)q + j);

	// End of "p": check if "q" also ends or just has a slash.
	if (c1 == NUL)
	{
	    if (c2 == NUL)  // full match
		return 0;
	    s = q;
	    i = j;
	    break;
	}

	// End of "q": check if "p" just has a slash.
	if (c2 == NUL)
	{
	    s = p;
	    break;
	}

	if ((p_fic ? MB_TOUPPER(c1) != MB_TOUPPER(c2) : c1 != c2)
#ifdef BACKSLASH_IN_FILENAME
		// consider '/' and '\\' to be equal
		&& !((c1 == '/' && c2 == '\\')
		    || (c1 == '\\' && c2 == '/'))
#endif
		)
	{
	    if (vim_ispathsep(c1))
		return -1;
	    if (vim_ispathsep(c2))
		return 1;
	    return p_fic ? MB_TOUPPER(c1) - MB_TOUPPER(c2)
		    : c1 - c2;  // no match
	}

	i += mb_ptr2len((char_u *)p + i);
	j += mb_ptr2len((char_u *)q + j);
    }
    if (s == NULL)	// "i" or "j" ran into "maxlen"
	return 0;

    c1 = PTR2CHAR((char_u *)s + i);
    c2 = PTR2CHAR((char_u *)s + i + mb_ptr2len((char_u *)s + i));
    // ignore a trailing slash, but not "//" or ":/"
    if (c2 == NUL
	    && i > 0
	    && !after_pathsep((char_u *)s, (char_u *)s + i)
#ifdef BACKSLASH_IN_FILENAME
	    && (c1 == '/' || c1 == '\\')
#else
	    && c1 == '/'
#endif
       )
	return 0;   // match with trailing slash
    if (s == q)
	return -1;	    // no match
    return 1;
}

/*
 * Return TRUE if "name" is a full (absolute) path name or URL.
 */
    int
vim_isAbsName(char_u *name)
{
    return (path_with_url(name) != 0 || mch_isFullName(name));
}

/*
 * Get absolute file name into buffer "buf[len]".
 *
 * return FAIL for failure, OK otherwise
 */
    int
vim_FullName(
    char_u	*fname,
    char_u	*buf,
    int		len,
    int		force)	    // force expansion even when already absolute
{
    int		retval = OK;
    int		url;

    *buf = NUL;
    if (fname == NULL)
	return FAIL;

    url = path_with_url(fname);
    if (!url)
	retval = mch_FullName(fname, buf, len, force);
    if (url || retval == FAIL)
    {
	// something failed; use the file name (truncate when too long)
	vim_strncpy(buf, fname, len - 1);
    }
#if defined(MSWIN)
    slash_adjust(buf);
#endif
    return retval;
}
