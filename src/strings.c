/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * strings.c: string manipulation functions
 */

#define USING_FLOAT_STUFF
#include "vim.h"

/*
 * Copy "string" into newly allocated memory.
 */
    char_u *
vim_strsave(char_u *string)
{
    char_u	*p;
    size_t	len;

    len = STRLEN(string) + 1;
    p = alloc(len);
    if (p != NULL)
	mch_memmove(p, string, len);
    return p;
}

/*
 * Copy up to "len" bytes of "string" into newly allocated memory and
 * terminate with a NUL.
 * The allocated memory always has size "len + 1", also when "string" is
 * shorter.
 */
    char_u *
vim_strnsave(char_u *string, size_t len)
{
    char_u	*p;

    p = alloc(len + 1);
    if (p == NULL)
	return NULL;

    STRNCPY(p, string, len);
    p[len] = NUL;
    return p;
}

/*
 * Same as vim_strsave(), but any characters found in esc_chars are preceded
 * by a backslash.
 */
    char_u *
vim_strsave_escaped(char_u *string, char_u *esc_chars)
{
    return vim_strsave_escaped_ext(string, esc_chars, '\\', FALSE);
}

/*
 * Same as vim_strsave_escaped(), but when "bsl" is TRUE also escape
 * characters where rem_backslash() would remove the backslash.
 * Escape the characters with "cc".
 */
    char_u *
vim_strsave_escaped_ext(
    char_u	*string,
    char_u	*esc_chars,
    int		cc,
    int		bsl)
{
    char_u	*p;
    char_u	*p2;
    char_u	*escaped_string;
    unsigned	length;
    int		l;

    // First count the number of backslashes required.
    // Then allocate the memory and insert them.
    length = 1;				// count the trailing NUL
    for (p = string; *p; p++)
    {
	if (has_mbyte && (l = (*mb_ptr2len)(p)) > 1)
	{
	    length += l;		// count a multibyte char
	    p += l - 1;
	    continue;
	}
	if (vim_strchr(esc_chars, *p) != NULL || (bsl && rem_backslash(p)))
	    ++length;			// count a backslash
	++length;			// count an ordinary char
    }
    escaped_string = alloc(length);
    if (escaped_string == NULL)
	return NULL;
    p2 = escaped_string;
    for (p = string; *p; p++)
    {
	if (has_mbyte && (l = (*mb_ptr2len)(p)) > 1)
	{
	    mch_memmove(p2, p, (size_t)l);
	    p2 += l;
	    p += l - 1;		// skip multibyte char
	    continue;
	}
	if (vim_strchr(esc_chars, *p) != NULL || (bsl && rem_backslash(p)))
	    *p2++ = cc;
	*p2++ = *p;
    }
    *p2 = NUL;
    return escaped_string;
}

/*
 * Return TRUE when 'shell' has "csh" in the tail.
 */
    int
csh_like_shell(void)
{
    return (strstr((char *)gettail(p_sh), "csh") != NULL);
}

/*
 * Return TRUE when 'shell' has "fish" in the tail.
 */
    static int
fish_like_shell(void)
{
    return (strstr((char *)gettail(p_sh), "fish") != NULL);
}

/*
 * Escape "string" for use as a shell argument with system().
 * This uses single quotes, except when we know we need to use double quotes
 * (MS-DOS and MS-Windows not using PowerShell and without 'shellslash' set).
 * PowerShell also uses a novel escaping for enclosed single quotes - double
 * them up.
 * Escape a newline, depending on the 'shell' option.
 * When "do_special" is TRUE also replace "!", "%", "#" and things starting
 * with "<" like "<cfile>".
 * When "do_newline" is FALSE do not escape newline unless it is csh shell.
 * Returns the result in allocated memory, NULL if we have run out.
 */
    char_u *
vim_strsave_shellescape(char_u *string, int do_special, int do_newline)
{
    unsigned	length;
    char_u	*p;
    char_u	*d;
    char_u	*escaped_string;
    int		l;
    int		csh_like;
    int		fish_like;
    char_u	*shname;
    int		powershell;
# ifdef MSWIN
    int		double_quotes;
# endif

    // Only csh and similar shells expand '!' within single quotes.  For sh and
    // the like we must not put a backslash before it, it will be taken
    // literally.  If do_special is set the '!' will be escaped twice.
    // Csh also needs to have "\n" escaped twice when do_special is set.
    csh_like = csh_like_shell();

    // Fish shell uses '\' as an escape character within single quotes, so '\'
    // itself must be escaped to get a literal '\'.
    fish_like = fish_like_shell();

    // PowerShell uses its own version for quoting single quotes
    shname = gettail(p_sh);
    powershell = strstr((char *)shname, "pwsh") != NULL;
# ifdef MSWIN
    powershell = powershell || strstr((char *)shname, "powershell") != NULL;
    // PowerShell only accepts single quotes so override shellslash.
    double_quotes = !powershell && !p_ssl;
# endif

    // First count the number of extra bytes required.
    length = (unsigned)STRLEN(string) + 3;  // two quotes and a trailing NUL
    for (p = string; *p != NUL; MB_PTR_ADV(p))
    {
# ifdef MSWIN
	if (double_quotes)
	{
	    if (*p == '"')
		++length;		// " -> ""
	}
	else
# endif
	if (*p == '\'')
	{
	    if (powershell)
		length +=2;		// ' => ''
	    else
		length += 3;		// ' => '\''
	}
	if ((*p == '\n' && (csh_like || do_newline))
		|| (*p == '!' && (csh_like || do_special)))
	{
	    ++length;			// insert backslash
	    if (csh_like && do_special)
		++length;		// insert backslash
	}
	if (do_special && find_cmdline_var(p, &l) >= 0)
	{
	    ++length;			// insert backslash
	    p += l - 1;
	}
	if (*p == '\\' && fish_like)
	    ++length;			// insert backslash
    }

    // Allocate memory for the result and fill it.
    escaped_string = alloc(length);
    if (escaped_string != NULL)
    {
	d = escaped_string;

	// add opening quote
# ifdef MSWIN
	if (double_quotes)
	    *d++ = '"';
	else
# endif
	    *d++ = '\'';

	for (p = string; *p != NUL; )
	{
# ifdef MSWIN
	    if (double_quotes)
	    {
		if (*p == '"')
		{
		    *d++ = '"';
		    *d++ = '"';
		    ++p;
		    continue;
		}
	    }
	    else
# endif
	    if (*p == '\'')
	    {
		if (powershell)
		{
		    *d++ = '\'';
		    *d++ = '\'';
		}
		else
		{
		    *d++ = '\'';
		    *d++ = '\\';
		    *d++ = '\'';
		    *d++ = '\'';
		}
		++p;
		continue;
	    }
	    if ((*p == '\n' && (csh_like || do_newline))
		    || (*p == '!' && (csh_like || do_special)))
	    {
		*d++ = '\\';
		if (csh_like && do_special)
		    *d++ = '\\';
		*d++ = *p++;
		continue;
	    }
	    if (do_special && find_cmdline_var(p, &l) >= 0)
	    {
		*d++ = '\\';		// insert backslash
		while (--l >= 0)	// copy the var
		    *d++ = *p++;
		continue;
	    }
	    if (*p == '\\' && fish_like)
	    {
		*d++ = '\\';
		*d++ = *p++;
		continue;
	    }

	    MB_COPY_CHAR(p, d);
	}

	// add terminating quote and finish with a NUL
# ifdef MSWIN
	if (double_quotes)
	    *d++ = '"';
	else
# endif
	    *d++ = '\'';
	*d = NUL;
    }

    return escaped_string;
}

/*
 * Like vim_strsave(), but make all characters uppercase.
 * This uses ASCII lower-to-upper case translation, language independent.
 */
    char_u *
vim_strsave_up(char_u *string)
{
    char_u *p1;

    p1 = vim_strsave(string);
    vim_strup(p1);
    return p1;
}

/*
 * Like vim_strnsave(), but make all characters uppercase.
 * This uses ASCII lower-to-upper case translation, language independent.
 */
    char_u *
vim_strnsave_up(char_u *string, size_t len)
{
    char_u *p1;

    p1 = vim_strnsave(string, len);
    vim_strup(p1);
    return p1;
}

/*
 * ASCII lower-to-upper case translation, language independent.
 */
    void
vim_strup(
    char_u	*p)
{
    char_u  *p2;
    int	    c;

    if (p == NULL)
	return;

    p2 = p;
    while ((c = *p2) != NUL)
	*p2++ = (c < 'a' || c > 'z') ? c : (c - 0x20);
}

#if defined(FEAT_EVAL) || defined(FEAT_SPELL) || defined(PROTO)
/*
 * Make string "s" all upper-case and return it in allocated memory.
 * Handles multi-byte characters as well as possible.
 * Returns NULL when out of memory.
 */
    static char_u *
strup_save(char_u *orig)
{
    char_u	*p;
    char_u	*res;

    res = p = vim_strsave(orig);

    if (res != NULL)
	while (*p != NUL)
	{
	    int		l;

	    if (enc_utf8)
	    {
		int	c, uc;
		int	newl;
		char_u	*s;

		c = utf_ptr2char(p);
		l = utf_ptr2len(p);
		if (c == 0)
		{
		    // overlong sequence, use only the first byte
		    c = *p;
		    l = 1;
		}
		uc = utf_toupper(c);

		// Reallocate string when byte count changes.  This is rare,
		// thus it's OK to do another malloc()/free().
		newl = utf_char2len(uc);
		if (newl != l)
		{
		    s = alloc(STRLEN(res) + 1 + newl - l);
		    if (s == NULL)
		    {
			vim_free(res);
			return NULL;
		    }
		    mch_memmove(s, res, p - res);
		    STRCPY(s + (p - res) + newl, p + l);
		    p = s + (p - res);
		    vim_free(res);
		    res = s;
		}

		utf_char2bytes(uc, p);
		p += newl;
	    }
	    else if (has_mbyte && (l = (*mb_ptr2len)(p)) > 1)
		p += l;		// skip multi-byte character
	    else
	    {
		*p = TOUPPER_LOC(*p); // note that toupper() can be a macro
		p++;
	    }
	}

    return res;
}

/*
 * Make string "s" all lower-case and return it in allocated memory.
 * Handles multi-byte characters as well as possible.
 * Returns NULL when out of memory.
 */
    char_u *
strlow_save(char_u *orig)
{
    char_u	*p;
    char_u	*res;

    res = p = vim_strsave(orig);

    if (res != NULL)
	while (*p != NUL)
	{
	    int		l;

	    if (enc_utf8)
	    {
		int	c, lc;
		int	newl;
		char_u	*s;

		c = utf_ptr2char(p);
		l = utf_ptr2len(p);
		if (c == 0)
		{
		    // overlong sequence, use only the first byte
		    c = *p;
		    l = 1;
		}
		lc = utf_tolower(c);

		// Reallocate string when byte count changes.  This is rare,
		// thus it's OK to do another malloc()/free().
		newl = utf_char2len(lc);
		if (newl != l)
		{
		    s = alloc(STRLEN(res) + 1 + newl - l);
		    if (s == NULL)
		    {
			vim_free(res);
			return NULL;
		    }
		    mch_memmove(s, res, p - res);
		    STRCPY(s + (p - res) + newl, p + l);
		    p = s + (p - res);
		    vim_free(res);
		    res = s;
		}

		utf_char2bytes(lc, p);
		p += newl;
	    }
	    else if (has_mbyte && (l = (*mb_ptr2len)(p)) > 1)
		p += l;		// skip multi-byte character
	    else
	    {
		*p = TOLOWER_LOC(*p); // note that tolower() can be a macro
		p++;
	    }
	}

    return res;
}
#endif

/*
 * delete spaces at the end of a string
 */
    void
del_trailing_spaces(char_u *ptr)
{
    char_u	*q;

    q = ptr + STRLEN(ptr);
    while (--q > ptr && VIM_ISWHITE(q[0]) && q[-1] != '\\' && q[-1] != Ctrl_V)
	*q = NUL;
}

/*
 * Like strncpy(), but always terminate the result with one NUL.
 * "to" must be "len + 1" long!
 */
    void
vim_strncpy(char_u *to, char_u *from, size_t len)
{
    STRNCPY(to, from, len);
    to[len] = NUL;
}

/*
 * Like strcat(), but make sure the result fits in "tosize" bytes and is
 * always NUL terminated. "from" and "to" may overlap.
 */
    void
vim_strcat(char_u *to, char_u *from, size_t tosize)
{
    size_t tolen = STRLEN(to);
    size_t fromlen = STRLEN(from);

    if (tolen + fromlen + 1 > tosize)
    {
	mch_memmove(to + tolen, from, tosize - tolen - 1);
	to[tosize - 1] = NUL;
    }
    else
	mch_memmove(to + tolen, from, fromlen + 1);
}

/*
 * A version of strlen() that has a maximum length.
 */
    size_t
vim_strlen_maxlen(char *s, size_t maxlen)
{
    size_t i;
    for (i = 0; i < maxlen; ++i)
	if (s[i] == NUL)
	    break;
    return i;
}

#if (!defined(HAVE_STRCASECMP) && !defined(HAVE_STRICMP)) || defined(PROTO)
/*
 * Compare two strings, ignoring case, using current locale.
 * Doesn't work for multi-byte characters.
 * return 0 for match, < 0 for smaller, > 0 for bigger
 */
    int
vim_stricmp(char *s1, char *s2)
{
    int		i;

    for (;;)
    {
	i = (int)TOLOWER_LOC(*s1) - (int)TOLOWER_LOC(*s2);
	if (i != 0)
	    return i;			    // this character different
	if (*s1 == NUL)
	    break;			    // strings match until NUL
	++s1;
	++s2;
    }
    return 0;				    // strings match
}
#endif

#if (!defined(HAVE_STRNCASECMP) && !defined(HAVE_STRNICMP)) || defined(PROTO)
/*
 * Compare two strings, for length "len", ignoring case, using current locale.
 * Doesn't work for multi-byte characters.
 * return 0 for match, < 0 for smaller, > 0 for bigger
 */
    int
vim_strnicmp(char *s1, char *s2, size_t len)
{
    int		i;

    while (len > 0)
    {
	i = (int)TOLOWER_LOC(*s1) - (int)TOLOWER_LOC(*s2);
	if (i != 0)
	    return i;			    // this character different
	if (*s1 == NUL)
	    break;			    // strings match until NUL
	++s1;
	++s2;
	--len;
    }
    return 0;				    // strings match
}
#endif

/*
 * Search for first occurrence of "c" in "string".
 * Version of strchr() that handles unsigned char strings with characters from
 * 128 to 255 correctly.  It also doesn't return a pointer to the NUL at the
 * end of the string.
 */
    char_u *
vim_strchr(char_u *string, int c)
{
    char_u	*p;
    int		b;

    p = string;
    if (enc_utf8 && c >= 0x80)
    {
	while (*p != NUL)
	{
	    int l = utfc_ptr2len(p);

	    // Avoid matching an illegal byte here.
	    if (utf_ptr2char(p) == c && l > 1)
		return p;
	    p += l;
	}
	return NULL;
    }
    if (enc_dbcs != 0 && c > 255)
    {
	int	n2 = c & 0xff;

	c = ((unsigned)c >> 8) & 0xff;
	while ((b = *p) != NUL)
	{
	    if (b == c && p[1] == n2)
		return p;
	    p += (*mb_ptr2len)(p);
	}
	return NULL;
    }
    if (has_mbyte)
    {
	while ((b = *p) != NUL)
	{
	    if (b == c)
		return p;
	    p += (*mb_ptr2len)(p);
	}
	return NULL;
    }
    while ((b = *p) != NUL)
    {
	if (b == c)
	    return p;
	++p;
    }
    return NULL;
}

/*
 * Version of strchr() that only works for bytes and handles unsigned char
 * strings with characters above 128 correctly. It also doesn't return a
 * pointer to the NUL at the end of the string.
 */
    char_u  *
vim_strbyte(char_u *string, int c)
{
    char_u	*p = string;

    while (*p != NUL)
    {
	if (*p == c)
	    return p;
	++p;
    }
    return NULL;
}

/*
 * Search for last occurrence of "c" in "string".
 * Version of strrchr() that handles unsigned char strings with characters from
 * 128 to 255 correctly.  It also doesn't return a pointer to the NUL at the
 * end of the string.
 * Return NULL if not found.
 * Does not handle multi-byte char for "c"!
 */
    char_u  *
vim_strrchr(char_u *string, int c)
{
    char_u	*retval = NULL;
    char_u	*p = string;

    while (*p)
    {
	if (*p == c)
	    retval = p;
	MB_PTR_ADV(p);
    }
    return retval;
}

/*
 * Vim's version of strpbrk(), in case it's missing.
 * Don't generate a prototype for this, causes problems when it's not used.
 */
#ifndef PROTO
# ifndef HAVE_STRPBRK
#  ifdef vim_strpbrk
#   undef vim_strpbrk
#  endif
    char_u *
vim_strpbrk(char_u *s, char_u *charset)
{
    while (*s)
    {
	if (vim_strchr(charset, *s) != NULL)
	    return s;
	MB_PTR_ADV(s);
    }
    return NULL;
}
# endif
#endif

/*
 * Sort an array of strings.
 */
static int sort_compare(const void *s1, const void *s2);

    static int
sort_compare(const void *s1, const void *s2)
{
    return STRCMP(*(char **)s1, *(char **)s2);
}

    void
sort_strings(
    char_u	**files,
    int		count)
{
    qsort((void *)files, (size_t)count, sizeof(char_u *), sort_compare);
}

#if defined(FEAT_QUICKFIX) || defined(FEAT_SPELL) || defined(PROTO)
/*
 * Return TRUE if string "s" contains a non-ASCII character (128 or higher).
 * When "s" is NULL FALSE is returned.
 */
    int
has_non_ascii(char_u *s)
{
    char_u	*p;

    if (s != NULL)
	for (p = s; *p != NUL; ++p)
	    if (*p >= 128)
		return TRUE;
    return FALSE;
}
#endif

/*
 * Concatenate two strings and return the result in allocated memory.
 * Returns NULL when out of memory.
 */
    char_u  *
concat_str(char_u *str1, char_u *str2)
{
    char_u  *dest;
    size_t  l = str1 == NULL ? 0 : STRLEN(str1);

    dest = alloc(l + (str2 == NULL ? 0 : STRLEN(str2)) + 1L);
    if (dest == NULL)
	return NULL;
    if (str1 == NULL)
	*dest = NUL;
    else
	STRCPY(dest, str1);
    if (str2 != NULL)
	STRCPY(dest + l, str2);
    return dest;
}

#if defined(FEAT_EVAL) || defined(FEAT_RIGHTLEFT) || defined(PROTO)
/*
 * Reverse text into allocated memory.
 * Returns the allocated string, NULL when out of memory.
 */
    char_u *
reverse_text(char_u *s)
{
    size_t len = STRLEN(s);
    char_u *rev = alloc(len + 1);
    if (rev == NULL)
	return NULL;

    for (size_t s_i = 0, rev_i = len; s_i < len; ++s_i)
    {
	if (has_mbyte)
	{
	    int mb_len = (*mb_ptr2len)(s + s_i);
	    rev_i -= mb_len;
	    mch_memmove(rev + rev_i, s + s_i, mb_len);
	    s_i += mb_len - 1;
	}
	else
	    rev[--rev_i] = s[s_i];
    }
    rev[len] = NUL;
    return rev;
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return string "str" in ' quotes, doubling ' characters.
 * If "str" is NULL an empty string is assumed.
 * If "function" is TRUE make it function('string').
 */
    char_u *
string_quote(char_u *str, int function)
{
    unsigned	len;
    char_u	*p, *r, *s;

    len = (function ? 13 : 3);
    if (str != NULL)
    {
	len += (unsigned)STRLEN(str);
	for (p = str; *p != NUL; MB_PTR_ADV(p))
	    if (*p == '\'')
		++len;
    }
    s = r = alloc(len);
    if (r == NULL)
	return NULL;

    if (function)
    {
	STRCPY(r, "function('");
	r += 10;
    }
    else
	*r++ = '\'';
    if (str != NULL)
	for (p = str; *p != NUL; )
	{
	    if (*p == '\'')
		*r++ = '\'';
	    MB_COPY_CHAR(p, r);
	}
    *r++ = '\'';
    if (function)
	*r++ = ')';
    *r++ = NUL;
    return s;
}

/*
 * Count the number of times "needle" occurs in string "haystack". Case is
 * ignored if "ic" is TRUE.
 */
    long
string_count(char_u *haystack, char_u *needle, int ic)
{
    long	n = 0;
    char_u	*p = haystack;
    char_u	*next;

    if (p == NULL || needle == NULL || *needle == NUL)
	return 0;

    if (ic)
    {
	size_t len = STRLEN(needle);

	while (*p != NUL)
	{
	    if (MB_STRNICMP(p, needle, len) == 0)
	    {
		++n;
		p += len;
	    }
	    else
		MB_PTR_ADV(p);
	}
    }
    else
	while ((next = (char_u *)strstr((char *)p, (char *)needle)) != NULL)
	{
	    ++n;
	    p = next + STRLEN(needle);
	}

    return n;
}

/*
 * Make a typval_T of the first character of "input" and store it in "output".
 * Return OK or FAIL.
 */
    static int
copy_first_char_to_tv(char_u *input, typval_T *output)
{
    char_u	buf[MB_MAXBYTES + 1];
    int		len;

    if (input == NULL || output == NULL)
	return FAIL;

    len = has_mbyte ? mb_ptr2len(input) : 1;
    STRNCPY(buf, input, len);
    buf[len] = NUL;
    output->v_type = VAR_STRING;
    output->vval.v_string = vim_strsave(buf);

    return output->vval.v_string == NULL ? FAIL : OK;
}

/*
 * Implementation of map() and filter() for a String. Apply "expr" to every
 * character in string "str" and return the result in "rettv".
 */
    void
string_filter_map(
	char_u		*str,
	filtermap_T	filtermap,
	typval_T	*expr,
	typval_T	*rettv)
{
    char_u	*p;
    typval_T	tv;
    garray_T	ga;
    int		len = 0;
    int		idx = 0;
    int		rem;
    typval_T	newtv;
    funccall_T	*fc;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    // set_vim_var_nr() doesn't set the type
    set_vim_var_type(VV_KEY, VAR_NUMBER);

    // Create one funccall_T for all eval_expr_typval() calls.
    fc = eval_expr_get_funccal(expr, &newtv);

    ga_init2(&ga, sizeof(char), 80);
    for (p = str; *p != NUL; p += len)
    {
	if (copy_first_char_to_tv(p, &tv) == FAIL)
	    break;
	len = (int)STRLEN(tv.vval.v_string);

	set_vim_var_nr(VV_KEY, idx);
	if (filter_map_one(&tv, expr, filtermap, fc, &newtv, &rem) == FAIL
		|| did_emsg)
	{
	    clear_tv(&newtv);
	    clear_tv(&tv);
	    break;
	}
	if (filtermap == FILTERMAP_MAP || filtermap == FILTERMAP_MAPNEW)
	{
	    if (newtv.v_type != VAR_STRING)
	    {
		clear_tv(&newtv);
		clear_tv(&tv);
		emsg(_(e_string_required));
		break;
	    }
	    else
		ga_concat(&ga, newtv.vval.v_string);
	}
	else if (filtermap == FILTERMAP_FOREACH || !rem)
	    ga_concat(&ga, tv.vval.v_string);

	clear_tv(&newtv);
	clear_tv(&tv);

	++idx;
    }
    ga_append(&ga, NUL);
    rettv->vval.v_string = ga.ga_data;
    if (fc != NULL)
	remove_funccal();
}

/*
 * Implementation of reduce() for String "argvars[0]" using the function "expr"
 * starting with the optional initial value "argvars[2]" and return the result
 * in "rettv".
 */
    void
string_reduce(
	typval_T	*argvars,
	typval_T	*expr,
	typval_T	*rettv)
{
    char_u	*p = tv_get_string(&argvars[0]);
    int		len;
    typval_T	argv[3];
    int		r;
    int		called_emsg_start = called_emsg;
    funccall_T	*fc;

    if (argvars[2].v_type == VAR_UNKNOWN)
    {
	if (*p == NUL)
	{
	    semsg(_(e_reduce_of_an_empty_str_with_no_initial_value), "String");
	    return;
	}
	if (copy_first_char_to_tv(p, rettv) == FAIL)
	    return;
	p += STRLEN(rettv->vval.v_string);
    }
    else if (check_for_string_arg(argvars, 2) == FAIL)
	return;
    else
	copy_tv(&argvars[2], rettv);

    // Create one funccall_T for all eval_expr_typval() calls.
    fc = eval_expr_get_funccal(expr, rettv);

    for ( ; *p != NUL; p += len)
    {
	argv[0] = *rettv;
	if (copy_first_char_to_tv(p, &argv[1]) == FAIL)
	    break;
	len = (int)STRLEN(argv[1].vval.v_string);

	r = eval_expr_typval(expr, TRUE, argv, 2, fc, rettv);

	clear_tv(&argv[0]);
	clear_tv(&argv[1]);
	if (r == FAIL || called_emsg != called_emsg_start)
	    return;
    }

    if (fc != NULL)
	remove_funccal();
}

/*
 * Implementation of "byteidx()" and "byteidxcomp()" functions
 */
    static void
byteidx_common(typval_T *argvars, typval_T *rettv, int comp UNUSED)
{
    rettv->vval.v_number = -1;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL
		|| check_for_opt_bool_arg(argvars, 2) == FAIL))
	return;

    char_u *str = tv_get_string_chk(&argvars[0]);
    varnumber_T	idx = tv_get_number_chk(&argvars[1], NULL);
    if (str == NULL || idx < 0)
	return;

    varnumber_T	utf16idx = FALSE;
    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	int error = FALSE;
	utf16idx = tv_get_bool_chk(&argvars[2], &error);
	if (error)
	    return;
	if (utf16idx < 0 || utf16idx > 1)
	{
	    semsg(_(e_using_number_as_bool_nr), utf16idx);
	    return;
	}
    }

    int (*ptr2len)(char_u *);
    if (enc_utf8 && comp)
	ptr2len = utf_ptr2len;
    else
	ptr2len = mb_ptr2len;

    char_u *t = str;
    for ( ; idx > 0; idx--)
    {
	if (*t == NUL)		// EOL reached
	    return;
	if (utf16idx)
	{
	    int clen = ptr2len(t);
	    int c = (clen > 1) ? utf_ptr2char(t) : *t;
	    if (c > 0xFFFF)
		idx--;
	}
	if (idx > 0)
	    t += ptr2len(t);
    }
    rettv->vval.v_number = (varnumber_T)(t - str);
}

/*
 * "byteidx()" function
 */
    void
f_byteidx(typval_T *argvars, typval_T *rettv)
{
    byteidx_common(argvars, rettv, FALSE);
}

/*
 * "byteidxcomp()" function
 */
    void
f_byteidxcomp(typval_T *argvars, typval_T *rettv)
{
    byteidx_common(argvars, rettv, TRUE);
}

/*
 * "charidx()" function
 */
    void
f_charidx(typval_T *argvars, typval_T *rettv)
{
    rettv->vval.v_number = -1;

    if (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL
		|| check_for_opt_bool_arg(argvars, 2) == FAIL
		|| (argvars[2].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 3) == FAIL))
	return;

    char_u *str = tv_get_string_chk(&argvars[0]);
    varnumber_T	idx = tv_get_number_chk(&argvars[1], NULL);
    if (str == NULL || idx < 0)
	return;

    varnumber_T	countcc = FALSE;
    varnumber_T	utf16idx = FALSE;
    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	countcc = tv_get_bool(&argvars[2]);
	if (argvars[3].v_type != VAR_UNKNOWN)
	    utf16idx = tv_get_bool(&argvars[3]);
    }

    int (*ptr2len)(char_u *);
    if (enc_utf8 && countcc)
	ptr2len = utf_ptr2len;
    else
	ptr2len = mb_ptr2len;

    char_u	*p;
    int		len;
    for (p = str, len = 0; utf16idx ? idx >= 0 : p <= str + idx; len++)
    {
	if (*p == NUL)
	{
	    // If the index is exactly the number of bytes or utf-16 code units
	    // in the string then return the length of the string in
	    // characters.
	    if (utf16idx ? (idx == 0) : (p == (str + idx)))
		rettv->vval.v_number = len;
	    return;
	}
	if (utf16idx)
	{
	    idx--;
	    int clen = ptr2len(p);
	    int c = (clen > 1) ? utf_ptr2char(p) : *p;
	    if (c > 0xFFFF)
		idx--;
	}
	p += ptr2len(p);
    }

    rettv->vval.v_number = len > 0 ? len - 1 : 0;
}

/*
 * "str2list()" function
 */
    void
f_str2list(typval_T *argvars, typval_T *rettv)
{
    char_u	*p;
    int		utf8 = FALSE;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL))
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
	utf8 = (int)tv_get_bool_chk(&argvars[1], NULL);

    p = tv_get_string(&argvars[0]);

    if (has_mbyte || utf8)
    {
	int (*ptr2len)(char_u *);
	int (*ptr2char)(char_u *);

	if (utf8 || enc_utf8)
	{
	    ptr2len = utf_ptr2len;
	    ptr2char = utf_ptr2char;
	}
	else
	{
	    ptr2len = mb_ptr2len;
	    ptr2char = mb_ptr2char;
	}

	for ( ; *p != NUL; p += (*ptr2len)(p))
	    list_append_number(rettv->vval.v_list, (*ptr2char)(p));
    }
    else
	for ( ; *p != NUL; ++p)
	    list_append_number(rettv->vval.v_list, *p);
}

/*
 * "str2nr()" function
 */
    void
f_str2nr(typval_T *argvars, typval_T *rettv)
{
    int		base = 10;
    char_u	*p;
    varnumber_T	n;
    int		what = 0;
    int		isneg;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 2) == FAIL)))
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	base = (int)tv_get_number(&argvars[1]);
	if (base != 2 && base != 8 && base != 10 && base != 16)
	{
	    emsg(_(e_invalid_argument));
	    return;
	}
	if (argvars[2].v_type != VAR_UNKNOWN && tv_get_bool(&argvars[2]))
	    what |= STR2NR_QUOTE;
    }

    p = skipwhite(tv_get_string_strict(&argvars[0]));
    isneg = (*p == '-');
    if (*p == '+' || *p == '-')
	p = skipwhite(p + 1);
    switch (base)
    {
	case 2: what |= STR2NR_BIN + STR2NR_FORCE; break;
	case 8: what |= STR2NR_OCT + STR2NR_OOCT + STR2NR_FORCE; break;
	case 16: what |= STR2NR_HEX + STR2NR_FORCE; break;
    }
    vim_str2nr(p, NULL, NULL, what, &n, NULL, 0, FALSE, NULL);
    // Text after the number is silently ignored.
    if (isneg)
	rettv->vval.v_number = -n;
    else
	rettv->vval.v_number = n;

}

/*
 * "strgetchar()" function
 */
    void
f_strgetchar(typval_T *argvars, typval_T *rettv)
{
    char_u	*str;
    int		len;
    int		error = FALSE;
    int		charidx;
    int		byteidx = 0;

    rettv->vval.v_number = -1;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    str = tv_get_string_chk(&argvars[0]);
    if (str == NULL)
	return;
    len = (int)STRLEN(str);
    charidx = (int)tv_get_number_chk(&argvars[1], &error);
    if (error)
	return;

    while (charidx >= 0 && byteidx < len)
    {
	if (charidx == 0)
	{
	    rettv->vval.v_number = mb_ptr2char(str + byteidx);
	    break;
	}
	--charidx;
	byteidx += MB_CPTR2LEN(str + byteidx);
    }
}

/*
 * "stridx()" function
 */
    void
f_stridx(typval_T *argvars, typval_T *rettv)
{
    char_u	buf[NUMBUFLEN];
    char_u	*needle;
    char_u	*haystack;
    char_u	*save_haystack;
    char_u	*pos;
    int		start_idx;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_opt_number_arg(argvars, 2) == FAIL))
	return;

    needle = tv_get_string_chk(&argvars[1]);
    save_haystack = haystack = tv_get_string_buf_chk(&argvars[0], buf);
    rettv->vval.v_number = -1;
    if (needle == NULL || haystack == NULL)
	return;		// type error; errmsg already given

    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	int	    error = FALSE;

	start_idx = (int)tv_get_number_chk(&argvars[2], &error);
	if (error || start_idx >= (int)STRLEN(haystack))
	    return;
	if (start_idx >= 0)
	    haystack += start_idx;
    }

    pos	= (char_u *)strstr((char *)haystack, (char *)needle);
    if (pos != NULL)
	rettv->vval.v_number = (varnumber_T)(pos - save_haystack);
}

/*
 * "string()" function
 */
    void
f_string(typval_T *argvars, typval_T *rettv)
{
    char_u	*tofree;
    char_u	numbuf[NUMBUFLEN];

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = tv2string(&argvars[0], &tofree, numbuf,
								get_copyID());
    // Make a copy if we have a value but it's not in allocated memory.
    if (rettv->vval.v_string != NULL && tofree == NULL)
	rettv->vval.v_string = vim_strsave(rettv->vval.v_string);
}

/*
 * "strlen()" function
 */
    void
f_strlen(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && check_for_string_or_number_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_number = (varnumber_T)(STRLEN(
					      tv_get_string(&argvars[0])));
}

    static void
strchar_common(typval_T *argvars, typval_T *rettv, int skipcc)
{
    char_u		*s = tv_get_string(&argvars[0]);
    varnumber_T		len = 0;
    int			(*func_mb_ptr2char_adv)(char_u **pp);

    func_mb_ptr2char_adv = skipcc ? mb_ptr2char_adv : mb_cptr2char_adv;
    while (*s != NUL)
    {
	func_mb_ptr2char_adv(&s);
	++len;
    }
    rettv->vval.v_number = len;
}

/*
 * "strcharlen()" function
 */
    void
f_strcharlen(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && check_for_string_or_number_arg(argvars, 0) == FAIL)
	return;

    strchar_common(argvars, rettv, TRUE);
}

/*
 * "strchars()" function
 */
    void
f_strchars(typval_T *argvars, typval_T *rettv)
{
    varnumber_T		skipcc = FALSE;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL))
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	int error = FALSE;
	skipcc = tv_get_bool_chk(&argvars[1], &error);
	if (error)
	    return;
	if (skipcc < 0 || skipcc > 1)
	{
	    semsg(_(e_using_number_as_bool_nr), skipcc);
	    return;
	}
    }

    strchar_common(argvars, rettv, skipcc);
}

/*
 * "strutf16len()" function
 */
    void
f_strutf16len(typval_T *argvars, typval_T *rettv)
{
    rettv->vval.v_number = -1;

    if (check_for_string_arg(argvars, 0) == FAIL
	    || check_for_opt_bool_arg(argvars, 1) == FAIL)
	return;

    varnumber_T countcc = FALSE;
    if (argvars[1].v_type != VAR_UNKNOWN)
	countcc = tv_get_bool(&argvars[1]);

    char_u		*s = tv_get_string(&argvars[0]);
    varnumber_T		len = 0;
    int			(*func_mb_ptr2char_adv)(char_u **pp);
    int			ch;

    func_mb_ptr2char_adv = countcc ? mb_cptr2char_adv : mb_ptr2char_adv;
    while (*s != NUL)
    {
	ch = func_mb_ptr2char_adv(&s);
	if (ch > 0xFFFF)
	    ++len;
	++len;
    }
    rettv->vval.v_number = len;
}

/*
 * "strdisplaywidth()" function
 */
    void
f_strdisplaywidth(typval_T *argvars, typval_T *rettv)
{
    char_u	*s;
    int		col = 0;

    rettv->vval.v_number = -1;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL))
	return;

    s = tv_get_string(&argvars[0]);
    if (argvars[1].v_type != VAR_UNKNOWN)
	col = (int)tv_get_number(&argvars[1]);

    rettv->vval.v_number = (varnumber_T)(linetabsize_col(col, s) - col);
}

/*
 * "strwidth()" function
 */
    void
f_strwidth(typval_T *argvars, typval_T *rettv)
{
    char_u	*s;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    s = tv_get_string_strict(&argvars[0]);
    rettv->vval.v_number = (varnumber_T)(mb_string2cells(s, -1));
}

/*
 * "strcharpart()" function
 */
    void
f_strcharpart(typval_T *argvars, typval_T *rettv)
{
    char_u	*p;
    int		nchar;
    int		nbyte = 0;
    int		charlen;
    int		skipcc = FALSE;
    int		len = 0;
    int		slen;
    int		error = FALSE;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL
		|| check_for_opt_number_arg(argvars, 2) == FAIL
		|| (argvars[2].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 3) == FAIL)))
	return;

    p = tv_get_string(&argvars[0]);
    slen = (int)STRLEN(p);

    nchar = (int)tv_get_number_chk(&argvars[1], &error);
    if (!error)
    {
	if (argvars[2].v_type != VAR_UNKNOWN
					   && argvars[3].v_type != VAR_UNKNOWN)
	{
	    skipcc = tv_get_bool_chk(&argvars[3], &error);
	    if (error)
		return;
	    if (skipcc < 0 || skipcc > 1)
	    {
		semsg(_(e_using_number_as_bool_nr), skipcc);
		return;
	    }
	}

	if (nchar > 0)
	    while (nchar > 0 && nbyte < slen)
	    {
		if (skipcc)
		    nbyte += mb_ptr2len(p + nbyte);
		else
		    nbyte += MB_CPTR2LEN(p + nbyte);
		--nchar;
	    }
	else
	    nbyte = nchar;
	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    charlen = (int)tv_get_number(&argvars[2]);
	    while (charlen > 0 && nbyte + len < slen)
	    {
		int off = nbyte + len;

		if (off < 0)
		    len += 1;
		else
		{
		    if (skipcc)
			len += mb_ptr2len(p + off);
		    else
			len += MB_CPTR2LEN(p + off);
		}
		--charlen;
	    }
	}
	else
	    len = slen - nbyte;    // default: all bytes that are available.
    }

    // Only return the overlap between the specified part and the actual
    // string.
    if (nbyte < 0)
    {
	len += nbyte;
	nbyte = 0;
    }
    else if (nbyte > slen)
	nbyte = slen;
    if (len < 0)
	len = 0;
    else if (nbyte + len > slen)
	len = slen - nbyte;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = vim_strnsave(p + nbyte, len);
}

/*
 * "strpart()" function
 */
    void
f_strpart(typval_T *argvars, typval_T *rettv)
{
    char_u	*p;
    int		n;
    int		len;
    int		slen;
    int		error = FALSE;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL
		|| check_for_opt_number_arg(argvars, 2) == FAIL
		|| (argvars[2].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 3) == FAIL)))
	return;

    p = tv_get_string(&argvars[0]);
    slen = (int)STRLEN(p);

    n = (int)tv_get_number_chk(&argvars[1], &error);
    if (error)
	len = 0;
    else if (argvars[2].v_type != VAR_UNKNOWN)
	len = (int)tv_get_number(&argvars[2]);
    else
	len = slen - n;	    // default len: all bytes that are available.

    // Only return the overlap between the specified part and the actual
    // string.
    if (n < 0)
    {
	len += n;
	n = 0;
    }
    else if (n > slen)
	n = slen;
    if (len < 0)
	len = 0;
    else if (n + len > slen)
	len = slen - n;

    if (argvars[2].v_type != VAR_UNKNOWN && argvars[3].v_type != VAR_UNKNOWN)
    {
	int off;

	// length in characters
	for (off = n; off < slen && len > 0; --len)
	    off += mb_ptr2len(p + off);
	len = off - n;
    }

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = vim_strnsave(p + n, len);
}

/*
 * "strridx()" function
 */
    void
f_strridx(typval_T *argvars, typval_T *rettv)
{
    char_u	buf[NUMBUFLEN];
    char_u	*needle;
    char_u	*haystack;
    char_u	*rest;
    char_u	*lastmatch = NULL;
    int		haystack_len, end_idx;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_opt_number_arg(argvars, 2) == FAIL))
	return;

    needle = tv_get_string_chk(&argvars[1]);
    haystack = tv_get_string_buf_chk(&argvars[0], buf);

    rettv->vval.v_number = -1;
    if (needle == NULL || haystack == NULL)
	return;		// type error; errmsg already given

    haystack_len = (int)STRLEN(haystack);
    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	// Third argument: upper limit for index
	end_idx = (int)tv_get_number_chk(&argvars[2], NULL);
	if (end_idx < 0)
	    return;	// can never find a match
    }
    else
	end_idx = haystack_len;

    if (*needle == NUL)
    {
	// Empty string matches past the end.
	lastmatch = haystack + end_idx;
    }
    else
    {
	for (rest = haystack; *rest != '\0'; ++rest)
	{
	    rest = (char_u *)strstr((char *)rest, (char *)needle);
	    if (rest == NULL || rest > haystack + end_idx)
		break;
	    lastmatch = rest;
	}
    }

    if (lastmatch == NULL)
	rettv->vval.v_number = -1;
    else
	rettv->vval.v_number = (varnumber_T)(lastmatch - haystack);
}

/*
 * "strtrans()" function
 */
    void
f_strtrans(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = transstr(tv_get_string(&argvars[0]));
}


/*
 * "utf16idx()" function
 *
 * Converts a byte or character offset in a string to the corresponding UTF-16
 * code unit offset.
 */
    void
f_utf16idx(typval_T *argvars, typval_T *rettv)
{
    rettv->vval.v_number = -1;

    if (check_for_string_arg(argvars, 0) == FAIL
	    || check_for_opt_number_arg(argvars, 1) == FAIL
	    || check_for_opt_bool_arg(argvars, 2) == FAIL
	    || (argvars[2].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 3) == FAIL))
	    return;

    char_u *str = tv_get_string_chk(&argvars[0]);
    varnumber_T	idx = tv_get_number_chk(&argvars[1], NULL);
    if (str == NULL || idx < 0)
	return;

    varnumber_T	countcc = FALSE;
    varnumber_T	charidx = FALSE;
    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	countcc = tv_get_bool(&argvars[2]);
	if (argvars[3].v_type != VAR_UNKNOWN)
	    charidx = tv_get_bool(&argvars[3]);
    }

    int (*ptr2len)(char_u *);
    if (enc_utf8 && countcc)
	ptr2len = utf_ptr2len;
    else
	ptr2len = mb_ptr2len;

    char_u	*p;
    int		len;
    int		utf16idx = 0;
    for (p = str, len = 0; charidx ? idx >= 0 : p <= str + idx; len++)
    {
	if (*p == NUL)
	{
	    // If the index is exactly the number of bytes or characters in the
	    // string then return the length of the string in utf-16 code
	    // units.
	    if (charidx ? (idx == 0) : (p == (str + idx)))
		rettv->vval.v_number = len;
	    return;
	}
	utf16idx = len;
	int clen = ptr2len(p);
	int c = (clen > 1) ? utf_ptr2char(p) : *p;
	if (c > 0xFFFF)
	    len++;
	p += ptr2len(p);
	if (charidx)
	    idx--;
    }

    rettv->vval.v_number = utf16idx;
}

/*
 * "tolower(string)" function
 */
    void
f_tolower(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = strlow_save(tv_get_string(&argvars[0]));
}

/*
 * "toupper(string)" function
 */
    void
f_toupper(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = strup_save(tv_get_string(&argvars[0]));
}

/*
 * "tr(string, fromstr, tostr)" function
 */
    void
f_tr(typval_T *argvars, typval_T *rettv)
{
    char_u	*in_str;
    char_u	*fromstr;
    char_u	*tostr;
    char_u	*p;
    int		inlen;
    int		fromlen;
    int		tolen;
    int		idx;
    char_u	*cpstr;
    int		cplen;
    int		first = TRUE;
    char_u	buf[NUMBUFLEN];
    char_u	buf2[NUMBUFLEN];
    garray_T	ga;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_string_arg(argvars, 2) == FAIL))
	return;

    in_str = tv_get_string(&argvars[0]);
    fromstr = tv_get_string_buf_chk(&argvars[1], buf);
    tostr = tv_get_string_buf_chk(&argvars[2], buf2);

    // Default return value: empty string.
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;
    if (fromstr == NULL || tostr == NULL)
	    return;		// type error; errmsg already given
    ga_init2(&ga, sizeof(char), 80);

    if (!has_mbyte)
	// not multi-byte: fromstr and tostr must be the same length
	if (STRLEN(fromstr) != STRLEN(tostr))
	{
error:
	    semsg(_(e_invalid_argument_str), fromstr);
	    ga_clear(&ga);
	    return;
	}

    // fromstr and tostr have to contain the same number of chars
    while (*in_str != NUL)
    {
	if (has_mbyte)
	{
	    inlen = (*mb_ptr2len)(in_str);
	    cpstr = in_str;
	    cplen = inlen;
	    idx = 0;
	    for (p = fromstr; *p != NUL; p += fromlen)
	    {
		fromlen = (*mb_ptr2len)(p);
		if (fromlen == inlen && STRNCMP(in_str, p, inlen) == 0)
		{
		    for (p = tostr; *p != NUL; p += tolen)
		    {
			tolen = (*mb_ptr2len)(p);
			if (idx-- == 0)
			{
			    cplen = tolen;
			    cpstr = p;
			    break;
			}
		    }
		    if (*p == NUL)	// tostr is shorter than fromstr
			goto error;
		    break;
		}
		++idx;
	    }

	    if (first && cpstr == in_str)
	    {
		// Check that fromstr and tostr have the same number of
		// (multi-byte) characters.  Done only once when a character
		// of in_str doesn't appear in fromstr.
		first = FALSE;
		for (p = tostr; *p != NUL; p += tolen)
		{
		    tolen = (*mb_ptr2len)(p);
		    --idx;
		}
		if (idx != 0)
		    goto error;
	    }

	    (void)ga_grow(&ga, cplen);
	    mch_memmove((char *)ga.ga_data + ga.ga_len, cpstr, (size_t)cplen);
	    ga.ga_len += cplen;

	    in_str += inlen;
	}
	else
	{
	    // When not using multi-byte chars we can do it faster.
	    p = vim_strchr(fromstr, *in_str);
	    if (p != NULL)
		ga_append(&ga, tostr[p - fromstr]);
	    else
		ga_append(&ga, *in_str);
	    ++in_str;
	}
    }

    // add a terminating NUL
    (void)ga_grow(&ga, 1);
    ga_append(&ga, NUL);

    rettv->vval.v_string = ga.ga_data;
}

/*
 * "trim({expr})" function
 */
    void
f_trim(typval_T *argvars, typval_T *rettv)
{
    char_u	buf1[NUMBUFLEN];
    char_u	buf2[NUMBUFLEN];
    char_u	*head;
    char_u	*mask = NULL;
    char_u	*tail;
    char_u	*prev;
    char_u	*p;
    int		c1;
    int		dir = 0;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 2) == FAIL)))
	return;

    head = tv_get_string_buf_chk(&argvars[0], buf1);
    if (head == NULL)
	return;

    if (check_for_opt_string_arg(argvars, 1) == FAIL)
	return;

    if (argvars[1].v_type == VAR_STRING)
    {
	mask = tv_get_string_buf_chk(&argvars[1], buf2);
	if (*mask == NUL)
	    mask = NULL;

	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    int	error = 0;

	    // leading or trailing characters to trim
	    dir = (int)tv_get_number_chk(&argvars[2], &error);
	    if (error)
		return;
	    if (dir < 0 || dir > 2)
	    {
		semsg(_(e_invalid_argument_str), tv_get_string(&argvars[2]));
		return;
	    }
	}
    }

    if (dir == 0 || dir == 1)
    {
	// Trim leading characters
	while (*head != NUL)
	{
	    c1 = PTR2CHAR(head);
	    if (mask == NULL)
	    {
		if (c1 > ' ' && c1 != 0xa0)
		    break;
	    }
	    else
	    {
		for (p = mask; *p != NUL; MB_PTR_ADV(p))
		    if (c1 == PTR2CHAR(p))
			break;
		if (*p == NUL)
		    break;
	    }
	    MB_PTR_ADV(head);
	}
    }

    tail = head + STRLEN(head);
    if (dir == 0 || dir == 2)
    {
	// Trim trailing characters
	for (; tail > head; tail = prev)
	{
	    prev = tail;
	    MB_PTR_BACK(head, prev);
	    c1 = PTR2CHAR(prev);
	    if (mask == NULL)
	    {
		if (c1 > ' ' && c1 != 0xa0)
		    break;
	    }
	    else
	    {
		for (p = mask; *p != NUL; MB_PTR_ADV(p))
		    if (c1 == PTR2CHAR(p))
			break;
		if (*p == NUL)
		    break;
	    }
	}
    }
    rettv->vval.v_string = vim_strnsave(head, tail - head);
}

static char *e_printf = N_(e_insufficient_arguments_for_printf);

/*
 * Get number argument from "idxp" entry in "tvs".  First entry is 1.
 */
    static varnumber_T
tv_nr(typval_T *tvs, int *idxp)
{
    int		idx = *idxp - 1;
    varnumber_T	n = 0;
    int		err = FALSE;

    if (tvs[idx].v_type == VAR_UNKNOWN)
	emsg(_(e_printf));
    else
    {
	++*idxp;
	n = tv_get_number_chk(&tvs[idx], &err);
	if (err)
	    n = 0;
    }
    return n;
}

/*
 * Get string argument from "idxp" entry in "tvs".  First entry is 1.
 * If "tofree" is NULL tv_get_string_chk() is used.  Some types (e.g. List)
 * are not converted to a string.
 * If "tofree" is not NULL echo_string() is used.  All types are converted to
 * a string with the same format as ":echo".  The caller must free "*tofree".
 * Returns NULL for an error.
 */
    static char *
tv_str(typval_T *tvs, int *idxp, char_u **tofree)
{
    int		    idx = *idxp - 1;
    char	    *s = NULL;
    static char_u   numbuf[NUMBUFLEN];

    if (tvs[idx].v_type == VAR_UNKNOWN)
	emsg(_(e_printf));
    else
    {
	++*idxp;
	if (tofree != NULL)
	    s = (char *)echo_string(&tvs[idx], tofree, numbuf, get_copyID());
	else
	    s = (char *)tv_get_string_chk(&tvs[idx]);
    }
    return s;
}

/*
 * Get float argument from "idxp" entry in "tvs".  First entry is 1.
 */
    static double
tv_float(typval_T *tvs, int *idxp)
{
    int		idx = *idxp - 1;
    double	f = 0;

    if (tvs[idx].v_type == VAR_UNKNOWN)
	emsg(_(e_printf));
    else
    {
	++*idxp;
	if (tvs[idx].v_type == VAR_FLOAT)
	    f = tvs[idx].vval.v_float;
	else if (tvs[idx].v_type == VAR_NUMBER)
	    f = (double)tvs[idx].vval.v_number;
	else
	    emsg(_(e_expected_float_argument_for_printf));
    }
    return f;
}

#endif

/*
 * Return the representation of infinity for printf() function:
 * "-inf", "inf", "+inf", " inf", "-INF", "INF", "+INF" or " INF".
 */
    static const char *
infinity_str(int positive,
	     char fmt_spec,
	     int force_sign,
	     int space_for_positive)
{
    static const char *table[] =
    {
	"-inf", "inf", "+inf", " inf",
	"-INF", "INF", "+INF", " INF"
    };
    int idx = positive * (1 + force_sign + force_sign * space_for_positive);

    if (ASCII_ISUPPER(fmt_spec))
	idx += 4;
    return table[idx];
}

/*
 * This code was included to provide a portable vsnprintf() and snprintf().
 * Some systems may provide their own, but we always use this one for
 * consistency.
 *
 * This code is based on snprintf.c - a portable implementation of snprintf
 * by Mark Martinec <mark.martinec@ijs.si>, Version 2.2, 2000-10-06.
 * Included with permission.  It was heavily modified to fit in Vim.
 * The original code, including useful comments, can be found here:
 *	http://www.ijs.si/software/snprintf/
 *
 * This snprintf() only supports the following conversion specifiers:
 * s, c, d, u, o, x, X, p  (and synonyms: i, D, U, O - see below)
 * with flags: '-', '+', ' ', '0' and '#'.
 * An asterisk is supported for field width as well as precision.
 *
 * Limited support for floating point was added: 'f', 'F', 'e', 'E', 'g', 'G'.
 *
 * Length modifiers 'h' (short int) and 'l' (long int) and 'll' (long long int)
 * are supported.  NOTE: for 'll' the argument is varnumber_T or uvarnumber_T.
 *
 * The locale is not used, the string is used as a byte string.  This is only
 * relevant for double-byte encodings where the second byte may be '%'.
 *
 * It is permitted for "str_m" to be zero, and it is permitted to specify NULL
 * pointer for resulting string argument if "str_m" is zero (as per ISO C99).
 *
 * The return value is the number of characters which would be generated
 * for the given input, excluding the trailing NUL. If this value
 * is greater or equal to "str_m", not all characters from the result
 * have been stored in str, output bytes beyond the ("str_m"-1) -th character
 * are discarded. If "str_m" is greater than zero it is guaranteed
 * the resulting string will be NUL-terminated.
 */

/*
 * When va_list is not supported we only define vim_snprintf().
 *
 * vim_vsnprintf_typval() can be invoked with either "va_list" or a list of
 * "typval_T".  When the latter is not used it must be NULL.
 */

// When generating prototypes all of this is skipped, cproto doesn't
// understand this.
#ifndef PROTO

// Like vim_vsnprintf() but append to the string.
    int
vim_snprintf_add(char *str, size_t str_m, const char *fmt, ...)
{
    va_list	ap;
    int		str_l;
    size_t	len = STRLEN(str);
    size_t	space;

    if (str_m <= len)
	space = 0;
    else
	space = str_m - len;
    va_start(ap, fmt);
    str_l = vim_vsnprintf(str + len, space, fmt, ap);
    va_end(ap);
    return str_l;
}

    int
vim_snprintf(char *str, size_t str_m, const char *fmt, ...)
{
    va_list	ap;
    int		str_l;

    va_start(ap, fmt);
    str_l = vim_vsnprintf(str, str_m, fmt, ap);
    va_end(ap);
    return str_l;
}

    int
vim_vsnprintf(
    char	*str,
    size_t	str_m,
    const char	*fmt,
    va_list	ap)
{
    return vim_vsnprintf_typval(str, str_m, fmt, ap, NULL);
}

enum
{
    TYPE_UNKNOWN = -1,
    TYPE_INT,
    TYPE_LONGINT,
    TYPE_LONGLONGINT,
    TYPE_UNSIGNEDINT,
    TYPE_UNSIGNEDLONGINT,
    TYPE_UNSIGNEDLONGLONGINT,
    TYPE_POINTER,
    TYPE_PERCENT,
    TYPE_CHAR,
    TYPE_STRING,
    TYPE_FLOAT
};

/* Types that can be used in a format string
 */
    static int
format_typeof(
    const char	*type)
{
    // allowed values: \0, h, l, L
    char    length_modifier = '\0';

    // current conversion specifier character
    char    fmt_spec = '\0';

    // parse 'h', 'l' and 'll' length modifiers
    if (*type == 'h' || *type == 'l')
    {
	length_modifier = *type;
	type++;
	if (length_modifier == 'l' && *type == 'l')
	{
	    // double l = __int64 / varnumber_T
	    length_modifier = 'L';
	    type++;
	}
    }
    fmt_spec = *type;

    // common synonyms:
    switch (fmt_spec)
    {
	case 'i': fmt_spec = 'd'; break;
	case '*': fmt_spec = 'd'; length_modifier = 'h'; break;
	case 'D': fmt_spec = 'd'; length_modifier = 'l'; break;
	case 'U': fmt_spec = 'u'; length_modifier = 'l'; break;
	case 'O': fmt_spec = 'o'; length_modifier = 'l'; break;
	default: break;
    }

    // get parameter value, do initial processing
    switch (fmt_spec)
    {
	// '%' and 'c' behave similar to 's' regarding flags and field
	// widths
    case '%':
	return TYPE_PERCENT;

    case 'c':
	return TYPE_CHAR;

    case 's':
    case 'S':
	return TYPE_STRING;

    case 'd': case 'u':
    case 'b': case 'B':
    case 'o':
    case 'x': case 'X':
    case 'p':
	{
	    // NOTE: the u, b, o, x, X and p conversion specifiers
	    // imply the value is unsigned;  d implies a signed
	    // value

	    // 0 if numeric argument is zero (or if pointer is
	    // NULL for 'p'), +1 if greater than zero (or nonzero
	    // for unsigned arguments), -1 if negative (unsigned
	    // argument is never negative)

	    if (fmt_spec == 'p')
		return TYPE_POINTER;
	    else if (fmt_spec == 'b' || fmt_spec == 'B')
		return TYPE_UNSIGNEDLONGLONGINT;
	    else if (fmt_spec == 'd')
	    {
		// signed
		switch (length_modifier)
		{
		case '\0':
		case 'h':
		    // char and short arguments are passed as int.
		    return TYPE_INT;
		case 'l':
		    return TYPE_LONGINT;
		case 'L':
		    return TYPE_LONGLONGINT;
		}
	    }
	    else
	    {
		// unsigned
		switch (length_modifier)
		{
		    case '\0':
		    case 'h':
			return TYPE_UNSIGNEDINT;
		    case 'l':
			return TYPE_UNSIGNEDLONGINT;
		    case 'L':
			return TYPE_UNSIGNEDLONGLONGINT;
		}
	    }
	}
	break;

    case 'f':
    case 'F':
    case 'e':
    case 'E':
    case 'g':
    case 'G':
	return TYPE_FLOAT;
    }

    return TYPE_UNKNOWN;
}

    static char *
format_typename(
    const char  *type)
{
    switch (format_typeof(type))
    {
	case TYPE_INT:
	    return _(typename_int);

	case TYPE_LONGINT:
	    return _(typename_longint);

	case TYPE_LONGLONGINT:
	    return _(typename_longlongint);

	case TYPE_UNSIGNEDINT:
	    return _(typename_unsignedint);

	case TYPE_UNSIGNEDLONGINT:
	    return _(typename_unsignedlongint);

	case TYPE_UNSIGNEDLONGLONGINT:
	    return _(typename_unsignedlonglongint);

	case TYPE_POINTER:
	    return _(typename_pointer);

	case TYPE_PERCENT:
	    return _(typename_percent);

	case TYPE_CHAR:
	    return _(typename_char);

	case TYPE_STRING:
	    return _(typename_string);

	case TYPE_FLOAT:
	    return _(typename_float);
    }

    return _(typename_unknown);
}

    static int
adjust_types(
    const char ***ap_types,
    int arg,
    int *num_posarg,
    const char *type)
{
    if (*ap_types == NULL || *num_posarg < arg)
    {
	int	    idx;
	const char  **new_types;

	if (*ap_types == NULL)
	    new_types = ALLOC_CLEAR_MULT(const char *, arg);
	else
	    new_types = vim_realloc((char **)*ap_types,
						arg * sizeof(const char *));

	if (new_types == NULL)
	    return FAIL;

	for (idx = *num_posarg; idx < arg; ++idx)
	    new_types[idx] = NULL;

	*ap_types = new_types;
	*num_posarg = arg;
    }

    if ((*ap_types)[arg - 1] != NULL)
    {
	if ((*ap_types)[arg - 1][0] == '*' || type[0] == '*')
	{
	    const char *pt = type;
	    if (pt[0] == '*')
		pt = (*ap_types)[arg - 1];

	    if (pt[0] != '*')
	    {
		switch (pt[0])
		{
		    case 'd': case 'i': break;
		    default:
			semsg(_(e_positional_num_field_spec_reused_str_str), arg, format_typename((*ap_types)[arg - 1]), format_typename(type));
			return FAIL;
		}
	    }
	}
	else
	{
	    if (format_typeof(type) != format_typeof((*ap_types)[arg - 1]))
	    {
		semsg(_( e_positional_arg_num_type_inconsistent_str_str), arg, format_typename(type), format_typename((*ap_types)[arg - 1]));
		return FAIL;
	    }
	}
    }

    (*ap_types)[arg - 1] = type;

    return OK;
}

    static int
parse_fmt_types(
    const char  ***ap_types,
    int		*num_posarg,
    const char  *fmt,
    typval_T	*tvs UNUSED
    )
{
    const char	*p = fmt;
    const char	*arg = NULL;

    int		any_pos = 0;
    int		any_arg = 0;
    int		arg_idx;

#define CHECK_POS_ARG do { \
    if (any_pos && any_arg) \
    { \
	semsg(_( e_cannot_mix_positional_and_non_positional_str), fmt); \
	goto error; \
    } \
} while (0);

    if (p == NULL)
	return OK;

    while (*p != NUL)
    {
	if (*p != '%')
	{
	    char    *q = strchr(p + 1, '%');
	    size_t  n = (q == NULL) ? STRLEN(p) : (size_t)(q - p);

	    p += n;
	}
	else
	{
	    // allowed values: \0, h, l, L
	    char	length_modifier = '\0';

	    // variable for positional arg
	    int		pos_arg = -1;
	    const char	*ptype = NULL;

	    p++;  // skip '%'

	    // First check to see if we find a positional
	    // argument specifier
	    ptype = p;

	    while (VIM_ISDIGIT(*ptype))
		++ptype;

	    if (*ptype == '$')
	    {
		if (*p == '0')
		{
		    // 0 flag at the wrong place
		    semsg(_( e_invalid_format_specifier_str), fmt);
		    goto error;
		}

		// Positional argument
		unsigned int uj = *p++ - '0';

		while (VIM_ISDIGIT((int)(*p)))
		    uj = 10 * uj + (unsigned int)(*p++ - '0');
		pos_arg = uj;

		any_pos = 1;
		CHECK_POS_ARG;

		++p;
	    }

	    // parse flags
	    while (*p == '0' || *p == '-' || *p == '+' || *p == ' '
						   || *p == '#' || *p == '\'')
	    {
		switch (*p)
		{
		    case '0': break;
		    case '-': break;
		    case '+': break;
		    case ' ': // If both the ' ' and '+' flags appear, the ' '
			      // flag should be ignored
			      break;
		    case '#': break;
		    case '\'': break;
		}
		p++;
	    }
	    // If the '0' and '-' flags both appear, the '0' flag should be
	    // ignored.

	    // parse field width
	    if (*(arg = p) == '*')
	    {
		p++;

		if (VIM_ISDIGIT((int)(*p)))
		{
		    // Positional argument field width
		    unsigned int uj = *p++ - '0';

		    while (VIM_ISDIGIT((int)(*p)))
			uj = 10 * uj + (unsigned int)(*p++ - '0');

		    if (*p != '$')
		    {
			semsg(_( e_invalid_format_specifier_str), fmt);
			goto error;
		    }
		    else
		    {
			++p;
			any_pos = 1;
			CHECK_POS_ARG;

			if (adjust_types(ap_types, uj, num_posarg, arg) == FAIL)
			    goto error;
		    }
		}
		else
		{
		    any_arg = 1;
		    CHECK_POS_ARG;
		}
	    }
	    else if (VIM_ISDIGIT((int)(*p)))
	    {
		// size_t could be wider than unsigned int; make sure we treat
		// argument like common implementations do
		unsigned int uj = *p++ - '0';

		while (VIM_ISDIGIT((int)(*p)))
		    uj = 10 * uj + (unsigned int)(*p++ - '0');

		if (*p == '$')
		{
		    semsg(_( e_invalid_format_specifier_str), fmt);
		    goto error;
		}
	    }

	    // parse precision
	    if (*p == '.')
	    {
		p++;

		if (*(arg = p) == '*')
		{
		    p++;

		    if (VIM_ISDIGIT((int)(*p)))
		    {
			// Parse precision
			unsigned int uj = *p++ - '0';

			while (VIM_ISDIGIT((int)(*p)))
			    uj = 10 * uj + (unsigned int)(*p++ - '0');

			if (*p == '$')
			{
			    any_pos = 1;
			    CHECK_POS_ARG;

			    ++p;

			    if (adjust_types(ap_types, uj, num_posarg, arg) == FAIL)
				goto error;
			}
			else
			{
			    semsg(_( e_invalid_format_specifier_str), fmt);
			    goto error;
			}
		    }
		    else
		    {
			any_arg = 1;
			CHECK_POS_ARG;
		    }
		}
		else if (VIM_ISDIGIT((int)(*p)))
		{
		    // size_t could be wider than unsigned int; make sure we
		    // treat argument like common implementations do
		    unsigned int uj = *p++ - '0';

		    while (VIM_ISDIGIT((int)(*p)))
			uj = 10 * uj + (unsigned int)(*p++ - '0');

		    if (*p == '$')
		    {
			semsg(_( e_invalid_format_specifier_str), fmt);
			goto error;
		    }
		}
	    }

	    if (pos_arg != -1)
	    {
		any_pos = 1;
		CHECK_POS_ARG;

		ptype = p;
	    }

	    // parse 'h', 'l' and 'll' length modifiers
	    if (*p == 'h' || *p == 'l')
	    {
		length_modifier = *p;
		p++;
		if (length_modifier == 'l' && *p == 'l')
		{
		    // double l = __int64 / varnumber_T
		    // length_modifier = 'L';
		    p++;
		}
	    }

	    switch (*p)
	    {
		// Check for known format specifiers. % is special!
		case 'i':
		case '*':
		case 'd':
		case 'u':
		case 'o':
		case 'D':
		case 'U':
		case 'O':
		case 'x':
		case 'X':
		case 'b':
		case 'B':
		case 'c':
		case 's':
		case 'S':
		case 'p':
		case 'f':
		case 'F':
		case 'e':
		case 'E':
		case 'g':
		case 'G':
		    if (pos_arg != -1)
		    {
			if (adjust_types(ap_types, pos_arg, num_posarg, ptype) == FAIL)
			    goto error;
		    }
		    else
		    {
			any_arg = 1;
			CHECK_POS_ARG;
		    }
		    break;

		default:
		    if (pos_arg != -1)
		    {
			semsg(_( e_cannot_mix_positional_and_non_positional_str), fmt);
			goto error;
		    }
	    }

	    if (*p != NUL)
		p++;     // step over the just processed conversion specifier
	}
    }

    for (arg_idx = 0; arg_idx < *num_posarg; ++arg_idx)
    {
	if ((*ap_types)[arg_idx] == NULL)
	{
	    semsg(_(e_fmt_arg_nr_unused_str), arg_idx + 1, fmt);
	    goto error;
	}

# if defined(FEAT_EVAL)
	if (tvs != NULL && tvs[arg_idx].v_type == VAR_UNKNOWN)
	{
	    semsg(_(e_positional_nr_out_of_bounds_str), arg_idx + 1, fmt);
	    goto error;
	}
# endif
    }

    return OK;

error:
    vim_free((char**)*ap_types);
    *ap_types = NULL;
    *num_posarg = 0;
    return FAIL;
}

    static void
skip_to_arg(
    const char	**ap_types,
    va_list	ap_start,
    va_list	*ap,
    int		*arg_idx,
    int		*arg_cur,
    const char	*fmt)
{
    int		arg_min = 0;

    if (*arg_cur + 1 == *arg_idx)
    {
	++*arg_cur;
	++*arg_idx;
	return;
    }

    if (*arg_cur >= *arg_idx)
    {
	// Reset ap to ap_start and skip arg_idx - 1 types
	va_end(*ap);
	va_copy(*ap, ap_start);
    }
    else
    {
	// Skip over any we should skip
	arg_min = *arg_cur;
    }

    for (*arg_cur = arg_min; *arg_cur < *arg_idx - 1; ++*arg_cur)
    {
	const char *p;

	if (ap_types == NULL || ap_types[*arg_cur] == NULL)
	{
	    siemsg(e_aptypes_is_null_nr_str, *arg_cur, fmt);
	    return;
	}

	p = ap_types[*arg_cur];

	int fmt_type = format_typeof(p);

	// get parameter value, do initial processing
	switch (fmt_type)
	{
	case TYPE_PERCENT:
	case TYPE_UNKNOWN:
	    break;

	case TYPE_CHAR:
	    va_arg(*ap, int);
	    break;

	case TYPE_STRING:
	    va_arg(*ap, char *);
	    break;

	case TYPE_POINTER:
	    va_arg(*ap, void *);
	    break;

	case TYPE_INT:
	    va_arg(*ap, int);
	    break;

	case TYPE_LONGINT:
	    va_arg(*ap, long int);
	    break;

	case TYPE_LONGLONGINT:
	    va_arg(*ap, varnumber_T);
	    break;

	case TYPE_UNSIGNEDINT:
	    va_arg(*ap, unsigned int);
	    break;

	case TYPE_UNSIGNEDLONGINT:
	    va_arg(*ap, unsigned long int);
	    break;

	case TYPE_UNSIGNEDLONGLONGINT:
	    va_arg(*ap, uvarnumber_T);
	    break;

	case TYPE_FLOAT:
	    va_arg(*ap, double);
	    break;
	}
    }

    // Because we know that after we return from this call,
    // a va_arg() call is made, we can pre-emptively
    // increment the current argument index.
    ++*arg_cur;
    ++*arg_idx;

    return;
}

    int
vim_vsnprintf_typval(
    char	*str,
    size_t	str_m,
    const char	*fmt,
    va_list	ap_start,
    typval_T	*tvs)
{
    size_t	str_l = 0;
    const char	*p = fmt;
    int		arg_cur = 0;
    int		num_posarg = 0;
    int		arg_idx = 1;
    va_list	ap;
    const char	**ap_types = NULL;

    if (parse_fmt_types(&ap_types, &num_posarg, fmt, tvs) == FAIL)
	return 0;

    va_copy(ap, ap_start);

    if (p == NULL)
	p = "";
    while (*p != NUL)
    {
	if (*p != '%')
	{
	    char    *q = strchr(p + 1, '%');
	    size_t  n = (q == NULL) ? STRLEN(p) : (size_t)(q - p);

	    // Copy up to the next '%' or NUL without any changes.
	    if (str_l < str_m)
	    {
		size_t avail = str_m - str_l;

		mch_memmove(str + str_l, p, n > avail ? avail : n);
	    }
	    p += n;
	    str_l += n;
	}
	else
	{
	    size_t  min_field_width = 0, precision = 0;
	    int	    zero_padding = 0, precision_specified = 0, justify_left = 0;
	    int	    alternate_form = 0, force_sign = 0;

	    // If both the ' ' and '+' flags appear, the ' ' flag should be
	    // ignored.
	    int	    space_for_positive = 1;

	    // allowed values: \0, h, l, L
	    char    length_modifier = '\0';

	    // temporary buffer for simple numeric->string conversion
# define TMP_LEN 350	// On my system 1e308 is the biggest number possible.
			// That sounds reasonable to use as the maximum
			// printable.
	    char    tmp[TMP_LEN];

	    // string address in case of string argument
	    const char  *str_arg = NULL;

	    // natural field width of arg without padding and sign
	    size_t  str_arg_l;

	    // unsigned char argument value - only defined for c conversion.
	    // N.B. standard explicitly states the char argument for the c
	    // conversion is unsigned
	    unsigned char uchar_arg;

	    // number of zeros to be inserted for numeric conversions as
	    // required by the precision or minimal field width
	    size_t  number_of_zeros_to_pad = 0;

	    // index into tmp where zero padding is to be inserted
	    size_t  zero_padding_insertion_ind = 0;

	    // current conversion specifier character
	    char    fmt_spec = '\0';

	    // buffer for 's' and 'S' specs
	    char_u  *tofree = NULL;

	    // variables for positional arg
	    int	    pos_arg = -1;
	    const char	*ptype;


	    p++;  // skip '%'

	    // First check to see if we find a positional
	    // argument specifier
	    ptype = p;

	    while (VIM_ISDIGIT(*ptype))
		++ptype;

	    if (*ptype == '$')
	    {
		// Positional argument
		unsigned int uj = *p++ - '0';

		while (VIM_ISDIGIT((int)(*p)))
		    uj = 10 * uj + (unsigned int)(*p++ - '0');
		pos_arg = uj;

		++p;
	    }

	    // parse flags
	    while (*p == '0' || *p == '-' || *p == '+' || *p == ' '
						   || *p == '#' || *p == '\'')
	    {
		switch (*p)
		{
		    case '0': zero_padding = 1; break;
		    case '-': justify_left = 1; break;
		    case '+': force_sign = 1; space_for_positive = 0; break;
		    case ' ': force_sign = 1;
			      // If both the ' ' and '+' flags appear, the ' '
			      // flag should be ignored
			      break;
		    case '#': alternate_form = 1; break;
		    case '\'': break;
		}
		p++;
	    }
	    // If the '0' and '-' flags both appear, the '0' flag should be
	    // ignored.

	    // parse field width
	    if (*p == '*')
	    {
		int j;

		p++;

		if (VIM_ISDIGIT((int)(*p)))
		{
		    // Positional argument field width
		    unsigned int uj = *p++ - '0';

		    while (VIM_ISDIGIT((int)(*p)))
			uj = 10 * uj + (unsigned int)(*p++ - '0');
		    arg_idx = uj;

		    ++p;
		}

		j =
# if defined(FEAT_EVAL)
		    tvs != NULL ? tv_nr(tvs, &arg_idx) :
# endif
			(skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
				     &arg_cur, fmt),
			va_arg(ap, int));

		if (j >= 0)
		    min_field_width = j;
		else
		{
		    min_field_width = -j;
		    justify_left = 1;
		}
	    }
	    else if (VIM_ISDIGIT((int)(*p)))
	    {
		// size_t could be wider than unsigned int; make sure we treat
		// argument like common implementations do
		unsigned int uj = *p++ - '0';

		while (VIM_ISDIGIT((int)(*p)))
		    uj = 10 * uj + (unsigned int)(*p++ - '0');
		min_field_width = uj;
	    }

	    // parse precision
	    if (*p == '.')
	    {
		p++;
		precision_specified = 1;

		if (VIM_ISDIGIT((int)(*p)))
		{
		    // size_t could be wider than unsigned int; make sure we
		    // treat argument like common implementations do
		    unsigned int uj = *p++ - '0';

		    while (VIM_ISDIGIT((int)(*p)))
			uj = 10 * uj + (unsigned int)(*p++ - '0');
		    precision = uj;
		}
		else if (*p == '*')
		{
		    int j;

		    p++;

		    if (VIM_ISDIGIT((int)(*p)))
		    {
			// positional argument
			unsigned int uj = *p++ - '0';

			while (VIM_ISDIGIT((int)(*p)))
			    uj = 10 * uj + (unsigned int)(*p++ - '0');
			arg_idx = uj;

			++p;
		    }

		    j =
# if defined(FEAT_EVAL)
			tvs != NULL ? tv_nr(tvs, &arg_idx) :
# endif
			    (skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
					 &arg_cur, fmt),
			    va_arg(ap, int));

		    if (j >= 0)
			precision = j;
		    else
		    {
			precision_specified = 0;
			precision = 0;
		    }
		}
	    }

	    // parse 'h', 'l' and 'll' length modifiers
	    if (*p == 'h' || *p == 'l')
	    {
		length_modifier = *p;
		p++;
		if (length_modifier == 'l' && *p == 'l')
		{
		    // double l = __int64 / varnumber_T
		    length_modifier = 'L';
		    p++;
		}
	    }
	    fmt_spec = *p;

	    // common synonyms:
	    switch (fmt_spec)
	    {
		case 'i': fmt_spec = 'd'; break;
		case 'D': fmt_spec = 'd'; length_modifier = 'l'; break;
		case 'U': fmt_spec = 'u'; length_modifier = 'l'; break;
		case 'O': fmt_spec = 'o'; length_modifier = 'l'; break;
		default: break;
	    }

# if defined(FEAT_EVAL)
	    switch (fmt_spec)
	    {
		case 'd': case 'u': case 'o': case 'x': case 'X':
		    if (tvs != NULL && length_modifier == '\0')
			length_modifier = 'L';
	    }
# endif

	    if (pos_arg != -1)
		arg_idx = pos_arg;

	    // get parameter value, do initial processing
	    switch (fmt_spec)
	    {
		// '%' and 'c' behave similar to 's' regarding flags and field
		// widths
	    case '%':
	    case 'c':
	    case 's':
	    case 'S':
		str_arg_l = 1;
		switch (fmt_spec)
		{
		case '%':
		    str_arg = p;
		    break;

		case 'c':
		    {
			int j;

			j =
# if defined(FEAT_EVAL)
			    tvs != NULL ? tv_nr(tvs, &arg_idx) :
# endif
				(skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
					     &arg_cur, fmt),
				va_arg(ap, int));

			// standard demands unsigned char
			uchar_arg = (unsigned char)j;
			str_arg = (char *)&uchar_arg;
			break;
		    }

		case 's':
		case 'S':
		    str_arg =
# if defined(FEAT_EVAL)
				tvs != NULL ? tv_str(tvs, &arg_idx, &tofree) :
# endif
				    (skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
						 &arg_cur, fmt),
				    va_arg(ap, char *));

		    if (str_arg == NULL)
		    {
			str_arg = "[NULL]";
			str_arg_l = 6;
		    }
		    // make sure not to address string beyond the specified
		    // precision !!!
		    else if (!precision_specified)
			str_arg_l = strlen(str_arg);
		    // truncate string if necessary as requested by precision
		    else if (precision == 0)
			str_arg_l = 0;
		    else
		    {
			// Don't put the #if inside memchr(), it can be a
			// macro.
			// memchr on HP does not like n > 2^31  !!!
			char *q = memchr(str_arg, '\0',
				  precision <= (size_t)0x7fffffffL ? precision
						       : (size_t)0x7fffffffL);

			str_arg_l = (q == NULL) ? precision
						      : (size_t)(q - str_arg);
		    }
		    if (fmt_spec == 'S')
		    {
			char_u	*p1;
			size_t	i;
			int	cell;

			for (i = 0, p1 = (char_u *)str_arg; *p1;
							  p1 += mb_ptr2len(p1))
			{
			    cell = mb_ptr2cells(p1);
			    if (precision_specified && i + cell > precision)
				break;
			    i += cell;
			}

			str_arg_l = p1 - (char_u *)str_arg;
			if (min_field_width != 0)
			    min_field_width += str_arg_l - i;
		    }
		    break;

		default:
		    break;
		}
		break;

	    case 'd': case 'u':
	    case 'b': case 'B':
	    case 'o':
	    case 'x': case 'X':
	    case 'p':
		{
		    // NOTE: the u, b, o, x, X and p conversion specifiers
		    // imply the value is unsigned;  d implies a signed
		    // value

		    // 0 if numeric argument is zero (or if pointer is
		    // NULL for 'p'), +1 if greater than zero (or nonzero
		    // for unsigned arguments), -1 if negative (unsigned
		    // argument is never negative)
		    int arg_sign = 0;

		    // only set for length modifier h, or for no length
		    // modifiers
		    int int_arg = 0;
		    unsigned int uint_arg = 0;

		    // only set for length modifier l
		    long int long_arg = 0;
		    unsigned long int ulong_arg = 0;

		    // only set for length modifier ll
		    varnumber_T llong_arg = 0;
		    uvarnumber_T ullong_arg = 0;

		    // only set for b conversion
		    uvarnumber_T bin_arg = 0;

		    // pointer argument value -only defined for p
		    // conversion
		    void *ptr_arg = NULL;

		    if (fmt_spec == 'p')
		    {
			length_modifier = '\0';
			ptr_arg =
# if defined(FEAT_EVAL)
				 tvs != NULL ? (void *)tv_str(tvs, &arg_idx,
									NULL) :
# endif
					(skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
						     &arg_cur, fmt),
					va_arg(ap, void *));

			if (ptr_arg != NULL)
			    arg_sign = 1;
		    }
		    else if (fmt_spec == 'b' || fmt_spec == 'B')
		    {
			bin_arg =
# if defined(FEAT_EVAL)
				    tvs != NULL ?
					   (uvarnumber_T)tv_nr(tvs, &arg_idx) :
# endif
					(skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
						     &arg_cur, fmt),
					va_arg(ap, uvarnumber_T));

			if (bin_arg != 0)
			    arg_sign = 1;
		    }
		    else if (fmt_spec == 'd')
		    {
			// signed
			switch (length_modifier)
			{
			case '\0':
			case 'h':
			    // char and short arguments are passed as int.
			    int_arg =
# if defined(FEAT_EVAL)
					tvs != NULL ? tv_nr(tvs, &arg_idx) :
# endif
					    (skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
							 &arg_cur, fmt),
					    va_arg(ap, int));

			    if (int_arg > 0)
				arg_sign =  1;
			    else if (int_arg < 0)
				arg_sign = -1;
			    break;
			case 'l':
			    long_arg =
# if defined(FEAT_EVAL)
					tvs != NULL ? tv_nr(tvs, &arg_idx) :
# endif
					    (skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
							 &arg_cur, fmt),
					    va_arg(ap, long int));

			    if (long_arg > 0)
				arg_sign =  1;
			    else if (long_arg < 0)
				arg_sign = -1;
			    break;
			case 'L':
			    llong_arg =
# if defined(FEAT_EVAL)
					tvs != NULL ? tv_nr(tvs, &arg_idx) :
# endif
					    (skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
							 &arg_cur, fmt),
					    va_arg(ap, varnumber_T));

			    if (llong_arg > 0)
				arg_sign =  1;
			    else if (llong_arg < 0)
				arg_sign = -1;
			    break;
			}
		    }
		    else
		    {
			// unsigned
			switch (length_modifier)
			{
			    case '\0':
			    case 'h':
				uint_arg =
# if defined(FEAT_EVAL)
					    tvs != NULL ? (unsigned)
							tv_nr(tvs, &arg_idx) :
# endif
						(skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
							     &arg_cur, fmt),
						va_arg(ap, unsigned int));

				if (uint_arg != 0)
				    arg_sign = 1;
				break;
			    case 'l':
				ulong_arg =
# if defined(FEAT_EVAL)
					    tvs != NULL ? (unsigned long)
							tv_nr(tvs, &arg_idx) :
# endif
						(skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
							     &arg_cur, fmt),
						va_arg(ap, unsigned long int));

				if (ulong_arg != 0)
				    arg_sign = 1;
				break;
			    case 'L':
				ullong_arg =
# if defined(FEAT_EVAL)
					    tvs != NULL ? (uvarnumber_T)
							tv_nr(tvs, &arg_idx) :
# endif
						(skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
							     &arg_cur, fmt),
						va_arg(ap, uvarnumber_T));

				if (ullong_arg != 0)
				    arg_sign = 1;
				break;
			}
		    }

		    str_arg = tmp;
		    str_arg_l = 0;

		    // NOTE:
		    //   For d, i, u, o, x, and X conversions, if precision is
		    //   specified, the '0' flag should be ignored. This is so
		    //   with Solaris 2.6, Digital UNIX 4.0, HPUX 10, Linux,
		    //   FreeBSD, NetBSD; but not with Perl.
		    if (precision_specified)
			zero_padding = 0;
		    if (fmt_spec == 'd')
		    {
			if (force_sign && arg_sign >= 0)
			    tmp[str_arg_l++] = space_for_positive ? ' ' : '+';
			// leave negative numbers for sprintf to handle, to
			// avoid handling tricky cases like (short int)-32768
		    }
		    else if (alternate_form)
		    {
			if (arg_sign != 0
				     && (fmt_spec == 'b' || fmt_spec == 'B'
				      || fmt_spec == 'x' || fmt_spec == 'X') )
			{
			    tmp[str_arg_l++] = '0';
			    tmp[str_arg_l++] = fmt_spec;
			}
			// alternate form should have no effect for p
			// conversion, but ...
		    }

		    zero_padding_insertion_ind = str_arg_l;
		    if (!precision_specified)
			precision = 1;   // default precision is 1
		    if (precision == 0 && arg_sign == 0)
		    {
			// When zero value is formatted with an explicit
			// precision 0, the resulting formatted string is
			// empty (d, i, u, b, B, o, x, X, p).
		    }
		    else
		    {
			char	f[6];
			int	f_l = 0;

			// construct a simple format string for sprintf
			f[f_l++] = '%';
			if (!length_modifier)
			    ;
			else if (length_modifier == 'L')
			{
# ifdef MSWIN
			    f[f_l++] = 'I';
			    f[f_l++] = '6';
			    f[f_l++] = '4';
# else
			    f[f_l++] = 'l';
			    f[f_l++] = 'l';
# endif
			}
			else
			    f[f_l++] = length_modifier;
			f[f_l++] = fmt_spec;
			f[f_l++] = '\0';

			if (fmt_spec == 'p')
			    str_arg_l += sprintf(tmp + str_arg_l, f, ptr_arg);
			else if (fmt_spec == 'b' || fmt_spec == 'B')
			{
			    char	    b[8 * sizeof(uvarnumber_T)];
			    size_t	    b_l = 0;
			    uvarnumber_T    bn = bin_arg;

			    do
			    {
				b[sizeof(b) - ++b_l] = '0' + (bn & 0x1);
				bn >>= 1;
			    }
			    while (bn != 0);

			    memcpy(tmp + str_arg_l, b + sizeof(b) - b_l, b_l);
			    str_arg_l += b_l;
			}
			else if (fmt_spec == 'd')
			{
			    // signed
			    switch (length_modifier)
			    {
			    case '\0': str_arg_l += sprintf(
						 tmp + str_arg_l, f,
						 int_arg);
				       break;
			    case 'h': str_arg_l += sprintf(
						 tmp + str_arg_l, f,
						 (short)int_arg);
				      break;
			    case 'l': str_arg_l += sprintf(
						tmp + str_arg_l, f, long_arg);
				      break;
			    case 'L': str_arg_l += sprintf(
					       tmp + str_arg_l, f, llong_arg);
				      break;
			    }
			}
			else
			{
			    // unsigned
			    switch (length_modifier)
			    {
			    case '\0': str_arg_l += sprintf(
						tmp + str_arg_l, f,
						uint_arg);
				       break;
			    case 'h': str_arg_l += sprintf(
						tmp + str_arg_l, f,
						(unsigned short)uint_arg);
				      break;
			    case 'l': str_arg_l += sprintf(
					       tmp + str_arg_l, f, ulong_arg);
				      break;
			    case 'L': str_arg_l += sprintf(
					      tmp + str_arg_l, f, ullong_arg);
				      break;
			    }
			}

			// include the optional minus sign and possible
			// "0x" in the region before the zero padding
			// insertion point
			if (zero_padding_insertion_ind < str_arg_l
				&& tmp[zero_padding_insertion_ind] == '-')
			    zero_padding_insertion_ind++;
			if (zero_padding_insertion_ind + 1 < str_arg_l
				&& tmp[zero_padding_insertion_ind]   == '0'
				&& (tmp[zero_padding_insertion_ind + 1] == 'x'
				 || tmp[zero_padding_insertion_ind + 1] == 'X'))
			    zero_padding_insertion_ind += 2;
		    }

		    {
			size_t num_of_digits = str_arg_l
						 - zero_padding_insertion_ind;

			if (alternate_form && fmt_spec == 'o'
				// unless zero is already the first
				// character
				&& !(zero_padding_insertion_ind < str_arg_l
				    && tmp[zero_padding_insertion_ind] == '0'))
			{
			    // assure leading zero for alternate-form
			    // octal numbers
			    if (!precision_specified
					     || precision < num_of_digits + 1)
			    {
				// precision is increased to force the
				// first character to be zero, except if a
				// zero value is formatted with an
				// explicit precision of zero
				precision = num_of_digits + 1;
			    }
			}
			// zero padding to specified precision?
			if (num_of_digits < precision)
			    number_of_zeros_to_pad = precision - num_of_digits;
		    }
		    // zero padding to specified minimal field width?
		    if (!justify_left && zero_padding)
		    {
			int n = (int)(min_field_width - (str_arg_l
						    + number_of_zeros_to_pad));
			if (n > 0)
			    number_of_zeros_to_pad += n;
		    }
		    break;
		}

	    case 'f':
	    case 'F':
	    case 'e':
	    case 'E':
	    case 'g':
	    case 'G':
		{
		    // Floating point.
		    double	f;
		    double	abs_f;
		    char	format[40];
		    int		l;
		    int		remove_trailing_zeroes = FALSE;

		    f =
# if defined(FEAT_EVAL)
			tvs != NULL ? tv_float(tvs, &arg_idx) :
# endif
			    (skip_to_arg(ap_types, ap_start, &ap, &arg_idx,
					 &arg_cur, fmt),
			    va_arg(ap, double));

		    abs_f = f < 0 ? -f : f;

		    if (fmt_spec == 'g' || fmt_spec == 'G')
		    {
			// Would be nice to use %g directly, but it prints
			// "1.0" as "1", we don't want that.
			if ((abs_f >= 0.001 && abs_f < 10000000.0)
							      || abs_f == 0.0)
			    fmt_spec = ASCII_ISUPPER(fmt_spec) ? 'F' : 'f';
			else
			    fmt_spec = fmt_spec == 'g' ? 'e' : 'E';
			remove_trailing_zeroes = TRUE;
		    }

		    if ((fmt_spec == 'f' || fmt_spec == 'F') &&
# ifdef VAX
			    abs_f > 1.0e38
# else
			    abs_f > 1.0e307
# endif
			    )
		    {
			// Avoid a buffer overflow
			STRCPY(tmp, infinity_str(f > 0.0, fmt_spec,
					      force_sign, space_for_positive));
			str_arg_l = STRLEN(tmp);
			zero_padding = 0;
		    }
		    else
		    {
			if (isnan(f))
			{
			    // Not a number: nan or NAN
			    STRCPY(tmp, ASCII_ISUPPER(fmt_spec) ? "NAN"
								      : "nan");
			    str_arg_l = 3;
			    zero_padding = 0;
			}
			else if (isinf(f))
			{
			    STRCPY(tmp, infinity_str(f > 0.0, fmt_spec,
					      force_sign, space_for_positive));
			    str_arg_l = STRLEN(tmp);
			    zero_padding = 0;
			}
			else
			{
			    // Regular float number
			    format[0] = '%';
			    l = 1;
			    if (force_sign)
				format[l++] = space_for_positive ? ' ' : '+';
			    if (precision_specified)
			    {
				size_t max_prec = TMP_LEN - 10;

				// Make sure we don't get more digits than we
				// have room for.
				if ((fmt_spec == 'f' || fmt_spec == 'F')
								&& abs_f > 1.0)
				    max_prec -= (size_t)log10(abs_f);
				if (precision > max_prec)
				    precision = max_prec;
				l += sprintf(format + l, ".%d", (int)precision);
			    }
			    format[l] = fmt_spec == 'F' ? 'f' : fmt_spec;
			    format[l + 1] = NUL;

			    str_arg_l = sprintf(tmp, format, f);
			}

			if (remove_trailing_zeroes)
			{
			    int i;
			    char *tp;

			    // Using %g or %G: remove superfluous zeroes.
			    if (fmt_spec == 'f' || fmt_spec == 'F')
				tp = tmp + str_arg_l - 1;
			    else
			    {
				tp = (char *)vim_strchr((char_u *)tmp,
						 fmt_spec == 'e' ? 'e' : 'E');
				if (tp != NULL)
				{
				    // Remove superfluous '+' and leading
				    // zeroes from the exponent.
				    if (tp[1] == '+')
				    {
					// Change "1.0e+07" to "1.0e07"
					STRMOVE(tp + 1, tp + 2);
					--str_arg_l;
				    }
				    i = (tp[1] == '-') ? 2 : 1;
				    while (tp[i] == '0')
				    {
					// Change "1.0e07" to "1.0e7"
					STRMOVE(tp + i, tp + i + 1);
					--str_arg_l;
				    }
				    --tp;
				}
			    }

			    if (tp != NULL && !precision_specified)
				// Remove trailing zeroes, but keep the one
				// just after a dot.
				while (tp > tmp + 2 && *tp == '0'
							     && tp[-1] != '.')
				{
				    STRMOVE(tp, tp + 1);
				    --tp;
				    --str_arg_l;
				}
			}
			else
			{
			    char *tp;

			    // Be consistent: some printf("%e") use 1.0e+12
			    // and some 1.0e+012.  Remove one zero in the last
			    // case.
			    tp = (char *)vim_strchr((char_u *)tmp,
						 fmt_spec == 'e' ? 'e' : 'E');
			    if (tp != NULL && (tp[1] == '+' || tp[1] == '-')
					  && tp[2] == '0'
					  && vim_isdigit(tp[3])
					  && vim_isdigit(tp[4]))
			    {
				STRMOVE(tp + 2, tp + 3);
				--str_arg_l;
			    }
			}
		    }
		    if (zero_padding && min_field_width > str_arg_l
					      && (tmp[0] == '-' || force_sign))
		    {
			// padding 0's should be inserted after the sign
			number_of_zeros_to_pad = min_field_width - str_arg_l;
			zero_padding_insertion_ind = 1;
		    }
		    str_arg = tmp;
		    break;
		}

	    default:
		// unrecognized conversion specifier, keep format string
		// as-is
		zero_padding = 0;  // turn zero padding off for non-numeric
				   // conversion
		justify_left = 1;
		min_field_width = 0;		    // reset flags

		// discard the unrecognized conversion, just keep *
		// the unrecognized conversion character
		str_arg = p;
		str_arg_l = 0;
		if (*p != NUL)
		    str_arg_l++;  // include invalid conversion specifier
				  // unchanged if not at end-of-string
		break;
	    }

	    if (*p != NUL)
		p++;     // step over the just processed conversion specifier

	    // insert padding to the left as requested by min_field_width;
	    // this does not include the zero padding in case of numerical
	    // conversions
	    if (!justify_left)
	    {
		// left padding with blank or zero
		int pn = (int)(min_field_width - (str_arg_l + number_of_zeros_to_pad));

		if (pn > 0)
		{
		    if (str_l < str_m)
		    {
			size_t avail = str_m - str_l;

			vim_memset(str + str_l, zero_padding ? '0' : ' ',
					     (size_t)pn > avail ? avail
								: (size_t)pn);
		    }
		    str_l += pn;
		}
	    }

	    // zero padding as requested by the precision or by the minimal
	    // field width for numeric conversions required?
	    if (number_of_zeros_to_pad == 0)
	    {
		// will not copy first part of numeric right now, *
		// force it to be copied later in its entirety
		zero_padding_insertion_ind = 0;
	    }
	    else
	    {
		// insert first part of numerics (sign or '0x') before zero
		// padding
		int zn = (int)zero_padding_insertion_ind;

		if (zn > 0)
		{
		    if (str_l < str_m)
		    {
			size_t avail = str_m - str_l;

			mch_memmove(str + str_l, str_arg,
					     (size_t)zn > avail ? avail
								: (size_t)zn);
		    }
		    str_l += zn;
		}

		// insert zero padding as requested by the precision or min
		// field width
		zn = (int)number_of_zeros_to_pad;
		if (zn > 0)
		{
		    if (str_l < str_m)
		    {
			size_t avail = str_m - str_l;

			vim_memset(str + str_l, '0',
					     (size_t)zn > avail ? avail
								: (size_t)zn);
		    }
		    str_l += zn;
		}
	    }

	    // insert formatted string
	    // (or as-is conversion specifier for unknown conversions)
	    {
		int sn = (int)(str_arg_l - zero_padding_insertion_ind);

		if (sn > 0)
		{
		    if (str_l < str_m)
		    {
			size_t avail = str_m - str_l;

			mch_memmove(str + str_l,
				str_arg + zero_padding_insertion_ind,
				(size_t)sn > avail ? avail : (size_t)sn);
		    }
		    str_l += sn;
		}
	    }

	    // insert right padding
	    if (justify_left)
	    {
		// right blank padding to the field width
		int pn = (int)(min_field_width
				      - (str_arg_l + number_of_zeros_to_pad));

		if (pn > 0)
		{
		    if (str_l < str_m)
		    {
			size_t avail = str_m - str_l;

			vim_memset(str + str_l, ' ',
					     (size_t)pn > avail ? avail
								: (size_t)pn);
		    }
		    str_l += pn;
		}
	    }
	    vim_free(tofree);
	}
    }

    if (str_m > 0)
    {
	// make sure the string is nul-terminated even at the expense of
	// overwriting the last character (shouldn't happen, but just in case)
	//
	str[str_l <= str_m - 1 ? str_l : str_m - 1] = '\0';
    }

    if (tvs != NULL && tvs[num_posarg != 0 ? num_posarg : arg_idx - 1].v_type != VAR_UNKNOWN)
	emsg(_(e_too_many_arguments_to_printf));

    vim_free((char*)ap_types);
    va_end(ap);

    // Return the number of characters formatted (excluding trailing nul
    // character), that is, the number of characters that would have been
    // written to the buffer if it were large enough.
    return (int)str_l;
}

#endif // PROTO
