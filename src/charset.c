/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

#include "vim.h"

#if defined(HAVE_WCHAR_H)
# include <wchar.h>	    // for towupper() and towlower()
#endif

static int win_nolbr_chartabsize(chartabsize_T *cts, int *headp);
static unsigned nr2hex(unsigned c);

static int    chartab_initialized = FALSE;

// b_chartab[] is an array of 32 bytes, each bit representing one of the
// characters 0-255.
#define SET_CHARTAB(buf, c) (buf)->b_chartab[(unsigned)(c) >> 3] |= (1 << ((c) & 0x7))
#define RESET_CHARTAB(buf, c) (buf)->b_chartab[(unsigned)(c) >> 3] &= ~(1 << ((c) & 0x7))
#define GET_CHARTAB(buf, c) ((buf)->b_chartab[(unsigned)(c) >> 3] & (1 << ((c) & 0x7)))

// table used below, see init_chartab() for an explanation
static char_u	g_chartab[256];

/*
 * Flags for g_chartab[].
 */
#define CT_CELL_MASK	0x07	// mask: nr of display cells (1, 2 or 4)
#define CT_PRINT_CHAR	0x10	// flag: set for printable chars
#define CT_ID_CHAR	0x20	// flag: set for ID chars
#define CT_FNAME_CHAR	0x40	// flag: set for file name chars

static int in_win_border(win_T *wp, colnr_T vcol);

/*
 * Fill g_chartab[].  Also fills curbuf->b_chartab[] with flags for keyword
 * characters for current buffer.
 *
 * Depends on the option settings 'iskeyword', 'isident', 'isfname',
 * 'isprint' and 'encoding'.
 *
 * The index in g_chartab[] depends on 'encoding':
 * - For non-multi-byte index with the byte (same as the character).
 * - For DBCS index with the first byte.
 * - For UTF-8 index with the character (when first byte is up to 0x80 it is
 *   the same as the character, if the first byte is 0x80 and above it depends
 *   on further bytes).
 *
 * The contents of g_chartab[]:
 * - The lower two bits, masked by CT_CELL_MASK, give the number of display
 *   cells the character occupies (1 or 2).  Not valid for UTF-8 above 0x80.
 * - CT_PRINT_CHAR bit is set when the character is printable (no need to
 *   translate the character before displaying it).  Note that only DBCS
 *   characters can have 2 display cells and still be printable.
 * - CT_FNAME_CHAR bit is set when the character can be in a file name.
 * - CT_ID_CHAR bit is set when the character can be in an identifier.
 *
 * Return FAIL if 'iskeyword', 'isident', 'isfname' or 'isprint' option has an
 * error, OK otherwise.
 */
    int
init_chartab(void)
{
    return buf_init_chartab(curbuf, TRUE);
}

    int
buf_init_chartab(
    buf_T	*buf,
    int		global)		// FALSE: only set buf->b_chartab[]
{
    int		c;
    int		c2;
    char_u	*p;
    int		i;
    int		tilde;
    int		do_isalpha;

    if (global)
    {
	/*
	 * Set the default size for printable characters:
	 * From <Space> to '~' is 1 (printable), others are 2 (not printable).
	 * This also inits all 'isident' and 'isfname' flags to FALSE.
	 */
	c = 0;
	while (c < ' ')
	    g_chartab[c++] = (dy_flags & DY_UHEX) ? 4 : 2;
	while (c <= '~')
	    g_chartab[c++] = 1 + CT_PRINT_CHAR;
	while (c < 256)
	{
	    // UTF-8: bytes 0xa0 - 0xff are printable (latin1)
	    if (enc_utf8 && c >= 0xa0)
		g_chartab[c++] = CT_PRINT_CHAR + 1;
	    // euc-jp characters starting with 0x8e are single width
	    else if (enc_dbcs == DBCS_JPNU && c == 0x8e)
		g_chartab[c++] = CT_PRINT_CHAR + 1;
	    // other double-byte chars can be printable AND double-width
	    else if (enc_dbcs != 0 && MB_BYTE2LEN(c) == 2)
		g_chartab[c++] = CT_PRINT_CHAR + 2;
	    else
		// the rest is unprintable by default
		g_chartab[c++] = (dy_flags & DY_UHEX) ? 4 : 2;
	}

	// Assume that every multi-byte char is a filename character.
	for (c = 1; c < 256; ++c)
	    if ((enc_dbcs != 0 && MB_BYTE2LEN(c) > 1)
		    || (enc_dbcs == DBCS_JPNU && c == 0x8e)
		    || (enc_utf8 && c >= 0xa0))
		g_chartab[c] |= CT_FNAME_CHAR;
    }

    /*
     * Init word char flags all to FALSE
     */
    CLEAR_FIELD(buf->b_chartab);
    if (enc_dbcs != 0)
	for (c = 0; c < 256; ++c)
	{
	    // double-byte characters are probably word characters
	    if (MB_BYTE2LEN(c) == 2)
		SET_CHARTAB(buf, c);
	}

    /*
     * In lisp mode the '-' character is included in keywords.
     */
    if (buf->b_p_lisp)
	SET_CHARTAB(buf, '-');

    // Walk through the 'isident', 'iskeyword', 'isfname' and 'isprint'
    // options Each option is a list of characters, character numbers or
    // ranges, separated by commas, e.g.: "200-210,x,#-178,-"
    for (i = global ? 0 : 3; i <= 3; ++i)
    {
	if (i == 0)
	    p = p_isi;		// first round: 'isident'
	else if (i == 1)
	    p = p_isp;		// second round: 'isprint'
	else if (i == 2)
	    p = p_isf;		// third round: 'isfname'
	else	// i == 3
	    p = buf->b_p_isk;	// fourth round: 'iskeyword'

	while (*p)
	{
	    tilde = FALSE;
	    do_isalpha = FALSE;
	    if (*p == '^' && p[1] != NUL)
	    {
		tilde = TRUE;
		++p;
	    }
	    if (VIM_ISDIGIT(*p))
		c = getdigits(&p);
	    else if (has_mbyte)
		c = mb_ptr2char_adv(&p);
	    else
		c = *p++;
	    c2 = -1;
	    if (*p == '-' && p[1] != NUL)
	    {
		++p;
		if (VIM_ISDIGIT(*p))
		    c2 = getdigits(&p);
		else if (has_mbyte)
		    c2 = mb_ptr2char_adv(&p);
		else
		    c2 = *p++;
	    }
	    if (c <= 0 || c >= 256 || (c2 < c && c2 != -1) || c2 >= 256
						 || !(*p == NUL || *p == ','))
		return FAIL;

	    if (c2 == -1)	// not a range
	    {
		/*
		 * A single '@' (not "@-@"):
		 * Decide on letters being ID/printable/keyword chars with
		 * standard function isalpha(). This takes care of locale for
		 * single-byte characters).
		 */
		if (c == '@')
		{
		    do_isalpha = TRUE;
		    c = 1;
		    c2 = 255;
		}
		else
		    c2 = c;
	    }
	    while (c <= c2)
	    {
		// Use the MB_ functions here, because isalpha() doesn't
		// work properly when 'encoding' is "latin1" and the locale is
		// "C".
		if (!do_isalpha || MB_ISLOWER(c) || MB_ISUPPER(c))
		{
		    if (i == 0)			// (re)set ID flag
		    {
			if (tilde)
			    g_chartab[c] &= ~CT_ID_CHAR;
			else
			    g_chartab[c] |= CT_ID_CHAR;
		    }
		    else if (i == 1)		// (re)set printable
		    {
			if ((c < ' ' || c > '~'
				// For double-byte we keep the cell width, so
				// that we can detect it from the first byte.
			    ) && !(enc_dbcs && MB_BYTE2LEN(c) == 2))
			{
			    if (tilde)
			    {
				g_chartab[c] = (g_chartab[c] & ~CT_CELL_MASK)
					     + ((dy_flags & DY_UHEX) ? 4 : 2);
				g_chartab[c] &= ~CT_PRINT_CHAR;
			    }
			    else
			    {
				g_chartab[c] = (g_chartab[c] & ~CT_CELL_MASK)
									   + 1;
				g_chartab[c] |= CT_PRINT_CHAR;
			    }
			}
		    }
		    else if (i == 2)		// (re)set fname flag
		    {
			if (tilde)
			    g_chartab[c] &= ~CT_FNAME_CHAR;
			else
			    g_chartab[c] |= CT_FNAME_CHAR;
		    }
		    else // i == 3		 (re)set keyword flag
		    {
			if (tilde)
			    RESET_CHARTAB(buf, c);
			else
			    SET_CHARTAB(buf, c);
		    }
		}
		++c;
	    }

	    c = *p;
	    p = skip_to_option_part(p);
	    if (c == ',' && *p == NUL)
		// Trailing comma is not allowed.
		return FAIL;
	}
    }
    chartab_initialized = TRUE;
    return OK;
}

/*
 * Translate any special characters in buf[bufsize] in-place.
 * The result is a string with only printable characters, but if there is not
 * enough room, not all characters will be translated.
 */
    void
trans_characters(
    char_u	*buf,
    int		bufsize)
{
    int		len;		// length of string needing translation
    int		room;		// room in buffer after string
    char_u	*trs;		// translated character
    int		trs_len;	// length of trs[]

    len = (int)STRLEN(buf);
    room = bufsize - len;
    while (*buf != 0)
    {
	// Assume a multi-byte character doesn't need translation.
	if (has_mbyte && (trs_len = (*mb_ptr2len)(buf)) > 1)
	    len -= trs_len;
	else
	{
	    trs = transchar_byte(*buf);
	    trs_len = (int)STRLEN(trs);
	    if (trs_len > 1)
	    {
		room -= trs_len - 1;
		if (room <= 0)
		    return;
		mch_memmove(buf + trs_len, buf + 1, (size_t)len);
	    }
	    mch_memmove(buf, trs, (size_t)trs_len);
	    --len;
	}
	buf += trs_len;
    }
}

/*
 * Translate a string into allocated memory, replacing special chars with
 * printable chars.  Returns NULL when out of memory.
 */
    char_u *
transstr(char_u *s)
{
    char_u	*res;
    char_u	*p;
    int		l, len, c;
    char_u	hexbuf[11];

    if (has_mbyte)
    {
	// Compute the length of the result, taking account of unprintable
	// multi-byte characters.
	len = 0;
	p = s;
	while (*p != NUL)
	{
	    if ((l = (*mb_ptr2len)(p)) > 1)
	    {
		c = (*mb_ptr2char)(p);
		p += l;
		if (vim_isprintc(c))
		    len += l;
		else
		{
		    transchar_hex(hexbuf, c);
		    len += (int)STRLEN(hexbuf);
		}
	    }
	    else
	    {
		l = byte2cells(*p++);
		if (l > 0)
		    len += l;
		else
		    len += 4;	// illegal byte sequence
	    }
	}
	res = alloc(len + 1);
    }
    else
	res = alloc(vim_strsize(s) + 1);

    if (res == NULL)
	return NULL;

    *res = NUL;
    p = s;
    while (*p != NUL)
    {
	if (has_mbyte && (l = (*mb_ptr2len)(p)) > 1)
	{
	    c = (*mb_ptr2char)(p);
	    if (vim_isprintc(c))
		STRNCAT(res, p, l);	// append printable multi-byte char
	    else
		transchar_hex(res + STRLEN(res), c);
	    p += l;
	}
	else
	    STRCAT(res, transchar_byte(*p++));
    }
    return res;
}

/*
 * Convert the string "str[orglen]" to do ignore-case comparing.  Uses the
 * current locale.
 * When "buf" is NULL returns an allocated string (NULL for out-of-memory).
 * Otherwise puts the result in "buf[buflen]".
 */
    char_u *
str_foldcase(
    char_u	*str,
    int		orglen,
    char_u	*buf,
    int		buflen)
{
    garray_T	ga;
    int		i;
    int		len = orglen;

#define GA_CHAR(i)  ((char_u *)ga.ga_data)[i]
#define GA_PTR(i)   ((char_u *)ga.ga_data + (i))
#define STR_CHAR(i)  (buf == NULL ? GA_CHAR(i) : buf[i])
#define STR_PTR(i)   (buf == NULL ? GA_PTR(i) : buf + (i))

    // Copy "str" into "buf" or allocated memory, unmodified.
    if (buf == NULL)
    {
	ga_init2(&ga, 1, 10);
	if (ga_grow(&ga, len + 1) == FAIL)
	    return NULL;
	mch_memmove(ga.ga_data, str, (size_t)len);
	ga.ga_len = len;
    }
    else
    {
	if (len >= buflen)	    // Ugly!
	    len = buflen - 1;
	mch_memmove(buf, str, (size_t)len);
    }
    if (buf == NULL)
	GA_CHAR(len) = NUL;
    else
	buf[len] = NUL;

    // Make each character lower case.
    i = 0;
    while (STR_CHAR(i) != NUL)
    {
	if (enc_utf8 || (has_mbyte && MB_BYTE2LEN(STR_CHAR(i)) > 1))
	{
	    if (enc_utf8)
	    {
		int	c = utf_ptr2char(STR_PTR(i));
		int	olen = utf_ptr2len(STR_PTR(i));
		int	lc = utf_tolower(c);

		// Only replace the character when it is not an invalid
		// sequence (ASCII character or more than one byte) and
		// utf_tolower() doesn't return the original character.
		if ((c < 0x80 || olen > 1) && c != lc)
		{
		    int	    nlen = utf_char2len(lc);

		    // If the byte length changes need to shift the following
		    // characters forward or backward.
		    if (olen != nlen)
		    {
			if (nlen > olen)
			{
			    if (buf == NULL
				    ? ga_grow(&ga, nlen - olen + 1) == FAIL
				    : len + nlen - olen >= buflen)
			    {
				// out of memory, keep old char
				lc = c;
				nlen = olen;
			    }
			}
			if (olen != nlen)
			{
			    if (buf == NULL)
			    {
				STRMOVE(GA_PTR(i) + nlen, GA_PTR(i) + olen);
				ga.ga_len += nlen - olen;
			    }
			    else
			    {
				STRMOVE(buf + i + nlen, buf + i + olen);
				len += nlen - olen;
			    }
			}
		    }
		    (void)utf_char2bytes(lc, STR_PTR(i));
		}
	    }
	    // skip to next multi-byte char
	    i += (*mb_ptr2len)(STR_PTR(i));
	}
	else
	{
	    if (buf == NULL)
		GA_CHAR(i) = TOLOWER_LOC(GA_CHAR(i));
	    else
		buf[i] = TOLOWER_LOC(buf[i]);
	    ++i;
	}
    }

    if (buf == NULL)
	return (char_u *)ga.ga_data;
    return buf;
}

/*
 * Catch 22: g_chartab[] can't be initialized before the options are
 * initialized, and initializing options may cause transchar() to be called!
 * When chartab_initialized == FALSE don't use g_chartab[].
 * Does NOT work for multi-byte characters, c must be <= 255.
 * Also doesn't work for the first byte of a multi-byte, "c" must be a
 * character!
 */
static char_u	transchar_charbuf[7];

    char_u *
transchar(int c)
{
    return transchar_buf(curbuf, c);
}

    char_u *
transchar_buf(buf_T *buf, int c)
{
    int			i;

    i = 0;
    if (IS_SPECIAL(c))	    // special key code, display as ~@ char
    {
	transchar_charbuf[0] = '~';
	transchar_charbuf[1] = '@';
	i = 2;
	c = K_SECOND(c);
    }

    if ((!chartab_initialized && ((c >= ' ' && c <= '~')))
					|| (c < 256 && vim_isprintc_strict(c)))
    {
	// printable character
	transchar_charbuf[i] = c;
	transchar_charbuf[i + 1] = NUL;
    }
    else
	transchar_nonprint(buf, transchar_charbuf + i, c);
    return transchar_charbuf;
}

/*
 * Like transchar(), but called with a byte instead of a character.  Checks
 * for an illegal UTF-8 byte.  Uses 'fileformat' of the current buffer.
 */
    char_u *
transchar_byte(int c)
{
    return transchar_byte_buf(curbuf, c);
}

/*
 * Like transchar_buf(), but called with a byte instead of a character.  Checks
 * for an illegal UTF-8 byte.  Uses 'fileformat' of "buf", unless it is NULL.
 */
    char_u *
transchar_byte_buf(buf_T *buf, int c)
{
    if (enc_utf8 && c >= 0x80)
    {
	transchar_nonprint(buf, transchar_charbuf, c);
	return transchar_charbuf;
    }
    return transchar_buf(buf, c);
}
/*
 * Convert non-printable character to two or more printable characters in
 * "charbuf[]".  "charbuf" needs to be able to hold five bytes.
 * Does NOT work for multi-byte characters, c must be <= 255.
 */
    void
transchar_nonprint(buf_T *buf, char_u *charbuf, int c)
{
    if (c == NL)
	c = NUL;		// we use newline in place of a NUL
    else if (buf != NULL && c == CAR && get_fileformat(buf) == EOL_MAC)
	c = NL;			// we use CR in place of  NL in this case

    if (dy_flags & DY_UHEX)		// 'display' has "uhex"
	transchar_hex(charbuf, c);

    else if (c <= 0x7f)			// 0x00 - 0x1f and 0x7f
    {
	charbuf[0] = '^';
	charbuf[1] = c ^ 0x40;		// DEL displayed as ^?
	charbuf[2] = NUL;
    }
    else if (enc_utf8)
    {
	transchar_hex(charbuf, c);
    }
    else if (c >= ' ' + 0x80 && c <= '~' + 0x80)    // 0xa0 - 0xfe
    {
	charbuf[0] = '|';
	charbuf[1] = c - 0x80;
	charbuf[2] = NUL;
    }
    else					    // 0x80 - 0x9f and 0xff
    {
	charbuf[0] = '~';
	charbuf[1] = (c - 0x80) ^ 0x40;	// 0xff displayed as ~?
	charbuf[2] = NUL;
    }
}

    void
transchar_hex(char_u *buf, int c)
{
    int		i = 0;

    buf[0] = '<';
    if (c > 255)
    {
	buf[++i] = nr2hex((unsigned)c >> 12);
	buf[++i] = nr2hex((unsigned)c >> 8);
    }
    buf[++i] = nr2hex((unsigned)c >> 4);
    buf[++i] = nr2hex((unsigned)c);
    buf[++i] = '>';
    buf[++i] = NUL;
}

/*
 * Convert the lower 4 bits of byte "c" to its hex character.
 * Lower case letters are used to avoid the confusion of <F1> being 0xf1 or
 * function key 1.
 */
    static unsigned
nr2hex(unsigned c)
{
    if ((c & 0xf) <= 9)
	return (c & 0xf) + '0';
    return (c & 0xf) - 10 + 'a';
}

/*
 * Return number of display cells occupied by byte "b".
 * Caller must make sure 0 <= b <= 255.
 * For multi-byte mode "b" must be the first byte of a character.
 * A TAB is counted as two cells: "^I".
 * For UTF-8 mode this will return 0 for bytes >= 0x80, because the number of
 * cells depends on further bytes.
 */
    int
byte2cells(int b)
{
    if (enc_utf8 && b >= 0x80)
	return 0;
    return (g_chartab[b] & CT_CELL_MASK);
}

/*
 * Return number of display cells occupied by character "c".
 * "c" can be a special key (negative number) in which case 3 or 4 is returned.
 * A TAB is counted as two cells: "^I" or four: "<09>".
 */
    int
char2cells(int c)
{
    if (IS_SPECIAL(c))
	return char2cells(K_SECOND(c)) + 2;
    if (c >= 0x80)
    {
	// UTF-8: above 0x80 need to check the value
	if (enc_utf8)
	    return utf_char2cells(c);
	// DBCS: double-byte means double-width, except for euc-jp with first
	// byte 0x8e
	if (enc_dbcs != 0 && c >= 0x100)
	{
	    if (enc_dbcs == DBCS_JPNU && ((unsigned)c >> 8) == 0x8e)
		return 1;
	    return 2;
	}
    }
    return (g_chartab[c & 0xff] & CT_CELL_MASK);
}

/*
 * Return number of display cells occupied by character at "*p".
 * A TAB is counted as two cells: "^I" or four: "<09>".
 */
    int
ptr2cells(char_u *p)
{
    if (!has_mbyte)
	return byte2cells(*p);

    // For UTF-8 we need to look at more bytes if the first byte is >= 0x80.
    if (enc_utf8 && *p >= 0x80)
	return utf_ptr2cells(p);
    // For DBCS we can tell the cell count from the first byte.
    return (g_chartab[*p] & CT_CELL_MASK);
}

/*
 * Return the number of character cells string "s" will take on the screen,
 * counting TABs as two characters: "^I".
 */
    int
vim_strsize(char_u *s)
{
    return vim_strnsize(s, (int)MAXCOL);
}

/*
 * Return the number of character cells string "s[len]" will take on the
 * screen, counting TABs as two characters: "^I".
 */
    int
vim_strnsize(char_u *s, int len)
{
    int		size = 0;

    while (*s != NUL && --len >= 0)
    {
	int	    l = (*mb_ptr2len)(s);

	size += ptr2cells(s);
	s += l;
	len -= l - 1;
    }

    return size;
}

/*
 * Return the number of characters 'c' will take on the screen, taking
 * into account the size of a tab.
 * Use a define to make it fast, this is used very often!!!
 * Also see getvcol() below.
 */

#ifdef FEAT_VARTABS
# define RET_WIN_BUF_CHARTABSIZE(wp, buf, p, col) \
    if (*(p) == TAB && (!(wp)->w_p_list || (wp)->w_lcs_chars.tab1)) \
    { \
	return tabstop_padding(col, (buf)->b_p_ts, (buf)->b_p_vts_array); \
    } \
    else \
	return ptr2cells(p);
#else
# define RET_WIN_BUF_CHARTABSIZE(wp, buf, p, col) \
    if (*(p) == TAB && (!(wp)->w_p_list || wp->w_lcs_chars.tab1)) \
    { \
	int ts; \
	ts = (buf)->b_p_ts; \
	return (int)(ts - (col % ts)); \
    } \
    else \
	return ptr2cells(p);
#endif

    int
chartabsize(char_u *p, colnr_T col)
{
    RET_WIN_BUF_CHARTABSIZE(curwin, curbuf, p, col)
}

#ifdef FEAT_LINEBREAK
    static int
win_chartabsize(win_T *wp, char_u *p, colnr_T col)
{
    RET_WIN_BUF_CHARTABSIZE(wp, wp->w_buffer, p, col)
}
#endif

/*
 * Return the number of characters the string "s" will take on the screen,
 * taking into account the size of a tab.
 * Does not handle text properties, since "s" is not a buffer line.
 */
    int
linetabsize_str(char_u *s)
{
    return linetabsize_col(0, s);
}

/*
 * Like linetabsize_str(), but "s" starts at column "startcol".
 */
    int
linetabsize_col(int startcol, char_u *s)
{
    chartabsize_T cts;

    init_chartabsize_arg(&cts, curwin, 0, startcol, s, s);
    while (*cts.cts_ptr != NUL)
	cts.cts_vcol += lbr_chartabsize_adv(&cts);
    clear_chartabsize_arg(&cts);
    return (int)cts.cts_vcol;
}

/*
 * Like linetabsize_str(), but for a given window instead of the current one.
 */
    int
win_linetabsize(win_T *wp, linenr_T lnum, char_u *line, colnr_T len)
{
    chartabsize_T cts;

    init_chartabsize_arg(&cts, wp, lnum, 0, line, line);
    win_linetabsize_cts(&cts, len);
    clear_chartabsize_arg(&cts);
    return (int)cts.cts_vcol;
}

/*
 * Return the number of cells line "lnum" of window "wp" will take on the
 * screen, taking into account the size of a tab and text properties.
 */
  int
linetabsize(win_T *wp, linenr_T lnum)
{
    return win_linetabsize(wp, lnum,
		       ml_get_buf(wp->w_buffer, lnum, FALSE), (colnr_T)MAXCOL);
}

    void
win_linetabsize_cts(chartabsize_T *cts, colnr_T len)
{
#ifdef FEAT_PROP_POPUP
    cts->cts_with_trailing = len == MAXCOL;
#endif
    for ( ; *cts->cts_ptr != NUL && (len == MAXCOL || cts->cts_ptr < cts->cts_line + len);
						      MB_PTR_ADV(cts->cts_ptr))
	cts->cts_vcol += win_lbr_chartabsize(cts, NULL);
#ifdef FEAT_PROP_POPUP
    // check for a virtual text at the end of a line or on an empty line
    if (len == MAXCOL && cts->cts_has_prop_with_text && *cts->cts_ptr == NUL)
    {
	(void)win_lbr_chartabsize(cts, NULL);
	cts->cts_vcol += cts->cts_cur_text_width;
	// when properties are above or below the empty line must also be
	// counted
	if (cts->cts_ptr == cts->cts_line && cts->cts_prop_lines > 0)
	    ++cts->cts_vcol;
    }
#endif
}

/*
 * Return TRUE if 'c' is a normal identifier character:
 * Letters and characters from the 'isident' option.
 */
    int
vim_isIDc(int c)
{
    return (c > 0 && c < 0x100 && (g_chartab[c] & CT_ID_CHAR));
}

/*
 * Like vim_isIDc() but not using the 'isident' option: letters, numbers and
 * underscore.
 */
    int
vim_isNormalIDc(int c)
{
    return ASCII_ISALNUM(c) || c == '_';
}

/*
 * return TRUE if 'c' is a keyword character: Letters and characters from
 * 'iskeyword' option for the current buffer.
 * For multi-byte characters mb_get_class() is used (builtin rules).
 */
    int
vim_iswordc(int c)
{
    return vim_iswordc_buf(c, curbuf);
}

    int
vim_iswordc_buf(int c, buf_T *buf)
{
    if (c >= 0x100)
    {
	if (enc_dbcs != 0)
	    return dbcs_class((unsigned)c >> 8, (unsigned)(c & 0xff)) >= 2;
	if (enc_utf8)
	    return utf_class_buf(c, buf) >= 2;
	return FALSE;
    }
    return (c > 0 && GET_CHARTAB(buf, c) != 0);
}

/*
 * Just like vim_iswordc() but uses a pointer to the (multi-byte) character.
 */
    int
vim_iswordp(char_u *p)
{
    return vim_iswordp_buf(p, curbuf);
}

    int
vim_iswordp_buf(char_u *p, buf_T *buf)
{
    int	c = *p;

    if (has_mbyte && MB_BYTE2LEN(c) > 1)
	c = (*mb_ptr2char)(p);
    return vim_iswordc_buf(c, buf);
}

/*
 * Return TRUE if 'c' is a valid file-name character as specified with the
 * 'isfname' option.
 * Assume characters above 0x100 are valid (multi-byte).
 * To be used for commands like "gf".
 */
    int
vim_isfilec(int c)
{
    return (c >= 0x100 || (c > 0 && (g_chartab[c] & CT_FNAME_CHAR)));
}

#if defined(FEAT_SPELL) || defined(PROTO)
/*
 * Return TRUE if 'c' is a valid file-name character, including characters left
 * out of 'isfname' to make "gf" work, such as comma, space, '@', etc.
 */
    int
vim_is_fname_char(int c)
{
    return vim_isfilec(c) || c == ',' || c == ' ' || c == '@';
}
#endif

/*
 * return TRUE if 'c' is a valid file-name character or a wildcard character
 * Assume characters above 0x100 are valid (multi-byte).
 * Explicitly interpret ']' as a wildcard character as mch_has_wildcard("]")
 * returns false.
 */
    int
vim_isfilec_or_wc(int c)
{
    char_u buf[2];

    buf[0] = (char_u)c;
    buf[1] = NUL;
    return vim_isfilec(c) || c == ']' || mch_has_wildcard(buf);
}

/*
 * Return TRUE if 'c' is a printable character.
 * Assume characters above 0x100 are printable (multi-byte), except for
 * Unicode.
 */
    int
vim_isprintc(int c)
{
    if (enc_utf8 && c >= 0x100)
	return utf_printable(c);
    return (c >= 0x100 || (c > 0 && (g_chartab[c] & CT_PRINT_CHAR)));
}

/*
 * Strict version of vim_isprintc(c), don't return TRUE if "c" is the head
 * byte of a double-byte character.
 */
    int
vim_isprintc_strict(int c)
{
    if (enc_dbcs != 0 && c < 0x100 && MB_BYTE2LEN(c) > 1)
	return FALSE;
    if (enc_utf8 && c >= 0x100)
	return utf_printable(c);
    return (c >= 0x100 || (c > 0 && (g_chartab[c] & CT_PRINT_CHAR)));
}

/*
 * Prepare the structure passed to chartabsize functions.
 * "line" is the start of the line, "ptr" is the first relevant character.
 * When "lnum" is zero do not use text properties that insert text.
 */
    void
init_chartabsize_arg(
	chartabsize_T	*cts,
	win_T		*wp,
	linenr_T	lnum UNUSED,
	colnr_T		col,
	char_u		*line,
	char_u		*ptr)
{
    CLEAR_POINTER(cts);
    cts->cts_win = wp;
    cts->cts_vcol = col;
    cts->cts_line = line;
    cts->cts_ptr = ptr;
#ifdef FEAT_LINEBREAK
    cts->cts_bri_size = -1;
#endif
#ifdef FEAT_PROP_POPUP
    if (lnum > 0 && !ignore_text_props)
    {
	char_u	*prop_start;
	int	count;

	count = get_text_props(wp->w_buffer, lnum, &prop_start, FALSE);
	cts->cts_text_prop_count = count;
	if (count > 0)
	{
	    // Make a copy of the properties, so that they are properly
	    // aligned.  Make it twice as long for the sorting below.
	    cts->cts_text_props = ALLOC_MULT(textprop_T, count * 2);
	    if (cts->cts_text_props == NULL)
		cts->cts_text_prop_count = 0;
	    else
	    {
		int	i;

		mch_memmove(cts->cts_text_props + count, prop_start,
						   count * sizeof(textprop_T));
		for (i = 0; i < count; ++i)
		{
		    textprop_T *tp = cts->cts_text_props + i + count;
		    if (tp->tp_id < 0
				     && text_prop_type_valid(wp->w_buffer, tp))
		    {
			cts->cts_has_prop_with_text = TRUE;
			break;
		    }
		}
		if (!cts->cts_has_prop_with_text)
		{
		    // won't use the text properties, free them
		    VIM_CLEAR(cts->cts_text_props);
		    cts->cts_text_prop_count = 0;
		}
		else
		{
		    int	    *text_prop_idxs;

		    // Need to sort the array to get any truncation right.
		    // Do the sorting in the second part of the array, then
		    // move the sorted props to the first part of the array.
		    text_prop_idxs = ALLOC_MULT(int, count);
		    if (text_prop_idxs != NULL)
		    {
			for (i = 0; i < count; ++i)
			    text_prop_idxs[i] = i + count;
			sort_text_props(curbuf, cts->cts_text_props,
							text_prop_idxs, count);
			// Here we want the reverse order.
			for (i = 0; i < count; ++i)
			    cts->cts_text_props[count - i - 1] =
					cts->cts_text_props[text_prop_idxs[i]];
			vim_free(text_prop_idxs);
		    }
		}
	    }
	}
    }
#endif
}

/*
 * Free any allocated item in "cts".
 */
    void
clear_chartabsize_arg(chartabsize_T *cts UNUSED)
{
#ifdef FEAT_PROP_POPUP
    if (cts->cts_text_prop_count > 0)
    {
	VIM_CLEAR(cts->cts_text_props);
	cts->cts_text_prop_count = 0;
    }
#endif
}

/*
 * Like chartabsize(), but also check for line breaks on the screen and text
 * properties that insert text.
 */
    int
lbr_chartabsize(chartabsize_T *cts)
{
#if defined(FEAT_LINEBREAK) || defined(FEAT_PROP_POPUP)
    if (1
# ifdef FEAT_LINEBREAK
	&& !curwin->w_p_lbr && *get_showbreak_value(curwin) == NUL
							   && !curwin->w_p_bri
# endif
# ifdef FEAT_PROP_POPUP
	&& !cts->cts_has_prop_with_text
#endif
       )
    {
#endif
	if (curwin->w_p_wrap)
	    return win_nolbr_chartabsize(cts, NULL);
	RET_WIN_BUF_CHARTABSIZE(curwin, curbuf, cts->cts_ptr, cts->cts_vcol)
#if defined(FEAT_LINEBREAK) || defined(FEAT_PROP_POPUP)
    }
    return win_lbr_chartabsize(cts, NULL);
#endif
}

/*
 * Call lbr_chartabsize() and advance the pointer.
 */
    int
lbr_chartabsize_adv(chartabsize_T *cts)
{
    int		retval;

    retval = lbr_chartabsize(cts);
    MB_PTR_ADV(cts->cts_ptr);
    return retval;
}

/*
 * Return the screen size of the character indicated by "cts".
 * "cts->cts_cur_text_width" is set to the extra size for a text property that
 * inserts text.
 * This function is used very often, keep it fast!!!!
 *
 * If "headp" not NULL, set "*headp" to the size of 'showbreak'/'breakindent'
 * included in the return value.
 * When "cts->cts_max_head_vcol" is positive, only count in "*headp" the size
 * of 'showbreak'/'breakindent' before "cts->cts_max_head_vcol".
 * When "cts->cts_max_head_vcol" is negative, only count in "*headp" the size
 * of 'showbreak'/'breakindent' before where cursor should be placed.
 *
 * Warning: "*headp" may not be set if it's 0, init to 0 before calling.
 */
    int
win_lbr_chartabsize(
	chartabsize_T	*cts,
	int		*headp UNUSED)
{
    win_T	*wp = cts->cts_win;
#if defined(FEAT_PROP_POPUP) || defined(FEAT_LINEBREAK)
    char_u	*line = cts->cts_line; // start of the line
#endif
    char_u	*s = cts->cts_ptr;
    colnr_T	vcol = cts->cts_vcol;
#ifdef FEAT_LINEBREAK
    int		size;
    int		mb_added = 0;
    int		n;
    char_u	*sbr;
    int		no_sbr = FALSE;
#endif

#if defined(FEAT_PROP_POPUP)
    cts->cts_cur_text_width = 0;
    cts->cts_first_char = 0;
#endif

#if defined(FEAT_LINEBREAK) || defined(FEAT_PROP_POPUP)
    /*
     * No 'linebreak', 'showbreak', 'breakindent' and text properties that
     * insert text: return quickly.
     */
    if (1
# ifdef FEAT_LINEBREAK
	    && !wp->w_p_lbr && !wp->w_p_bri && *get_showbreak_value(wp) == NUL
# endif
# ifdef FEAT_PROP_POPUP
	    && !cts->cts_has_prop_with_text
# endif
	    )
#endif
    {
	if (wp->w_p_wrap)
	    return win_nolbr_chartabsize(cts, headp);
	RET_WIN_BUF_CHARTABSIZE(wp, wp->w_buffer, s, vcol)
    }

#if defined(FEAT_LINEBREAK) || defined(FEAT_PROP_POPUP)
    int has_lcs_eol = wp->w_p_list && wp->w_lcs_chars.eol != NUL;

    /*
     * First get the normal size, without 'linebreak' or text properties
     */
    size = win_chartabsize(wp, s, vcol);
    if (*s == NUL && !has_lcs_eol)
	size = 0;  // NUL is not displayed
# ifdef FEAT_LINEBREAK
    int is_doublewidth = has_mbyte && size == 2 && MB_BYTE2LEN(*s) > 1;
# endif

# ifdef FEAT_PROP_POPUP
    if (cts->cts_has_prop_with_text)
    {
	int	    tab_size = size;
	int	    charlen = *s == NUL ? 1 : mb_ptr2len(s);
	int	    i;
	int	    col = (int)(s - line);
	garray_T    *gap = &wp->w_buffer->b_textprop_text;

	// The "$" for 'list' mode will go between the EOL and
	// the text prop, account for that.
	if (has_lcs_eol)
	{
	    ++vcol;
	    --size;
	}

	for (i = 0; i < cts->cts_text_prop_count; ++i)
	{
	    textprop_T	*tp = cts->cts_text_props + i;
	    int		col_off = win_col_off(wp);

	    // Watch out for the text being deleted.  "cts_text_props" is a
	    // copy, the text prop may actually have been removed from the line.
	    if (tp->tp_id < 0
		    && ((tp->tp_col - 1 >= col
					     && tp->tp_col - 1 < col + charlen)
		       || (tp->tp_col == MAXCOL
			   && ((tp->tp_flags & TP_FLAG_ALIGN_ABOVE)
				? col == 0
				: s[0] == NUL && cts->cts_with_trailing)))
		    && -tp->tp_id - 1 < gap->ga_len)
	    {
		char_u *p = ((char_u **)gap->ga_data)[-tp->tp_id - 1];

		if (p != NULL)
		{
		    int	cells;

		    if (tp->tp_col == MAXCOL)
		    {
			int n_extra = (int)STRLEN(p);

			cells = text_prop_position(wp, tp, vcol,
			     (vcol + size) % (wp->w_width - col_off) + col_off,
					      &n_extra, &p, NULL, NULL, FALSE);
#  ifdef FEAT_LINEBREAK
			no_sbr = TRUE;  // don't use 'showbreak' now
#  endif
		    }
		    else
			cells = vim_strsize(p);
		    cts->cts_cur_text_width += cells;
		    if (tp->tp_flags & TP_FLAG_ALIGN_ABOVE)
			cts->cts_first_char += cells;
		    else
			size += cells;
		    cts->cts_start_incl = tp->tp_flags & TP_FLAG_START_INCL;
		    if (*s == TAB)
		    {
			// tab size changes because of the inserted text
			size -= tab_size;
			tab_size = win_chartabsize(wp, s, vcol + size);
			size += tab_size;
		    }
		    if (tp->tp_col == MAXCOL && (tp->tp_flags
				& (TP_FLAG_ALIGN_ABOVE | TP_FLAG_ALIGN_BELOW)))
			// count extra line for property above/below
			++cts->cts_prop_lines;
		}
	    }
	    if (tp->tp_col != MAXCOL && tp->tp_col - 1 > col)
		break;
	}
	if (has_lcs_eol)
	{
	    --vcol;
	    ++size;
	}
    }
# endif

# ifdef FEAT_LINEBREAK
    if (is_doublewidth && wp->w_p_wrap && in_win_border(wp, vcol + size - 2))
    {
	++size;		// Count the ">" in the last column.
	mb_added = 1;
    }

    /*
     * May have to add something for 'breakindent' and/or 'showbreak'
     * string at the start of a screen line.
     */
    int head = mb_added;
    sbr = no_sbr ? empty_option : get_showbreak_value(wp);
    // When "size" is 0, no new screen line is started.
    if (size > 0 && wp->w_p_wrap && (*sbr != NUL || wp->w_p_bri))
    {
	int	col_off_prev = win_col_off(wp);
	int	width2 = wp->w_width - col_off_prev + win_col_off2(wp);
	colnr_T	wcol = vcol + col_off_prev;
#  ifdef FEAT_PROP_POPUP
	wcol -= wp->w_virtcol_first_char;
#  endif
	colnr_T	max_head_vcol = cts->cts_max_head_vcol;
	int	added = 0;

	// cells taken by 'showbreak'/'breakindent' before current char
	int	head_prev = 0;
	if (wcol >= wp->w_width)
	{
	    wcol -= wp->w_width;
	    col_off_prev = wp->w_width - width2;
	    if (wcol >= width2 && width2 > 0)
		wcol %= width2;
	    if (*sbr != NUL)
		head_prev += vim_strsize(sbr);
	    if (wp->w_p_bri)
	    {
		if (cts->cts_bri_size < 0)
		    cts->cts_bri_size = get_breakindent_win(wp, line);
		head_prev += cts->cts_bri_size;
	    }
	    if (wcol < head_prev)
	    {
		head_prev -= wcol;
		wcol += head_prev;
		added += head_prev;
		if (max_head_vcol <= 0 || vcol < max_head_vcol)
		    head += head_prev;
	    }
	    else
		head_prev = 0;
	    wcol += col_off_prev;
	}

	if (wcol + size > wp->w_width)
	{
	    // cells taken by 'showbreak'/'breakindent' halfway current char
	    int	head_mid = 0;
	    if (*sbr != NUL)
		head_mid += vim_strsize(sbr);
	    if (wp->w_p_bri)
	    {
		if (cts->cts_bri_size < 0)
		    cts->cts_bri_size = get_breakindent_win(wp, line);
		head_mid += cts->cts_bri_size;
	    }
	    if (head_mid > 0 && wcol + size > wp->w_width)
	    {
		// Calculate effective window width.
		int prev_rem = wp->w_width - wcol;
		int width = width2 - head_mid;

		if (width <= 0)
		    width = 1;
		// Divide "size - prev_rem" by "width", rounding up.
		int cnt = (size - prev_rem + width - 1) / width;
		added += cnt * head_mid;

		if (max_head_vcol == 0 || vcol + size + added < max_head_vcol)
		    head += cnt * head_mid;
		else if (max_head_vcol > vcol + head_prev + prev_rem)
		    head += (max_head_vcol - (vcol + head_prev + prev_rem)
					     + width2 - 1) / width2 * head_mid;
#  ifdef FEAT_PROP_POPUP
		else if (max_head_vcol < 0)
		{
		    int off = 0;
		    if (*s != NUL
			     && ((State & MODE_NORMAL) || cts->cts_start_incl))
			off += cts->cts_cur_text_width;
		    if (off >= prev_rem)
			head += (1 + (off - prev_rem) / width) * head_mid;
		}
#  endif
	    }
	}

	size += added;
    }

    if (headp != NULL)
	*headp = head;

    int need_lbr = FALSE;
    /*
     * If 'linebreak' set check at a blank before a non-blank if the line
     * needs a break here.
     */
    if (wp->w_p_lbr && wp->w_p_wrap && wp->w_width != 0
	    && VIM_ISBREAK((int)s[0]) && !VIM_ISBREAK((int)s[1]))
    {
	char_u	*t = cts->cts_line;
	while (VIM_ISBREAK((int)t[0]))
	    t++;
	// 'linebreak' is only needed when not in leading whitespace.
	need_lbr = s >= t;
    }
    if (need_lbr)
    {
	/*
	 * Count all characters from first non-blank after a blank up to next
	 * non-blank after a blank.
	 */
	int numberextra = win_col_off(wp);
	colnr_T col_adj = size - 1;
	colnr_T colmax = (colnr_T)(wp->w_width - numberextra - col_adj);
	if (vcol >= colmax)
	{
	    colmax += col_adj;
	    n = colmax +  win_col_off2(wp);
	    if (n > 0)
		colmax += (((vcol - colmax) / n) + 1) * n - col_adj;
	}

	colnr_T vcol2 = vcol;
	for (;;)
	{
	    char_u *ps = s;
	    MB_PTR_ADV(s);
	    int c = *s;
	    if (!(c != NUL
		    && (VIM_ISBREAK(c)
			|| (!VIM_ISBREAK(c)
			       && (vcol2 == vcol || !VIM_ISBREAK((int)*ps))))))
		break;

	    vcol2 += win_chartabsize(wp, s, vcol2);
	    if (vcol2 >= colmax)		// doesn't fit
	    {
		size = colmax - vcol + col_adj;
		break;
	    }
	}
    }

#  ifdef FEAT_PROP_POPUP
    size += cts->cts_first_char;
#  endif
    return size;
# endif
#endif
}

/*
 * Like win_lbr_chartabsize(), except that we know 'linebreak' is off, 'wrap'
 * is on and there are no properties that insert text.  This means we need to
 * check for a double-byte character that doesn't fit at the end of the screen
 * line.
 * Only uses "cts_win", "cts_ptr" and "cts_vcol" from "cts".
 */
    static int
win_nolbr_chartabsize(
	chartabsize_T	*cts,
	int		*headp)
{
    win_T	*wp = cts->cts_win;
    char_u	*s = cts->cts_ptr;
    colnr_T	col = cts->cts_vcol;
    int		n;

    if (*s == TAB && (!wp->w_p_list || wp->w_lcs_chars.tab1))
    {
# ifdef FEAT_VARTABS
	return tabstop_padding(col, wp->w_buffer->b_p_ts,
				    wp->w_buffer->b_p_vts_array);
# else
	n = wp->w_buffer->b_p_ts;
	return (int)(n - (col % n));
# endif
    }
    n = ptr2cells(s);
    // Add one cell for a double-width character in the last column of the
    // window, displayed with a ">".
    if (n == 2 && MB_BYTE2LEN(*s) > 1 && in_win_border(wp, col))
    {
	if (headp != NULL)
	    *headp = 1;
	return 3;
    }
    return n;
}

/*
 * Return TRUE if virtual column "vcol" is in the rightmost column of window
 * "wp".
 */
    static int
in_win_border(win_T *wp, colnr_T vcol)
{
    int		width1;		// width of first line (after line number)
    int		width2;		// width of further lines

    if (wp->w_width == 0)	// there is no border
	return FALSE;
    width1 = wp->w_width - win_col_off(wp);
    if ((int)vcol < width1 - 1)
	return FALSE;
    if ((int)vcol == width1 - 1)
	return TRUE;
    width2 = width1 + win_col_off2(wp);
    if (width2 <= 0)
	return FALSE;
    return ((vcol - width1) % width2 == width2 - 1);
}

/*
 * Get virtual column number of pos.
 *  start: on the first position of this character (TAB, ctrl)
 * cursor: where the cursor is on this character (first char, except for TAB)
 *    end: on the last position of this character (TAB, ctrl)
 *
 * This is used very often, keep it fast!
 */
    void
getvcol(
    win_T	*wp,
    pos_T	*pos,
    colnr_T	*start,
    colnr_T	*cursor,
    colnr_T	*end)
{
    colnr_T	vcol;
    char_u	*ptr;		// points to current char
    char_u	*line;		// start of the line
    int		incr;
    int		head;
#ifdef FEAT_VARTABS
    int		*vts = wp->w_buffer->b_p_vts_array;
#endif
    int		ts = wp->w_buffer->b_p_ts;
    int		c;
    chartabsize_T cts;
#ifdef FEAT_PROP_POPUP
    int		on_NUL = FALSE;
#endif

    vcol = 0;
    line = ptr = ml_get_buf(wp->w_buffer, pos->lnum, FALSE);

    init_chartabsize_arg(&cts, wp, pos->lnum, 0, line, line);
    cts.cts_max_head_vcol = -1;

    /*
     * This function is used very often, do some speed optimizations.
     * When 'list', 'linebreak', 'showbreak' and 'breakindent' are not set
     * and there are no text properties with "text" use a simple loop.
     * Also use this when 'list' is set but tabs take their normal size.
     */
    if ((!wp->w_p_list || wp->w_lcs_chars.tab1 != NUL)
#ifdef FEAT_LINEBREAK
	    && !wp->w_p_lbr && *get_showbreak_value(wp) == NUL && !wp->w_p_bri
#endif
#ifdef FEAT_PROP_POPUP
	    && !cts.cts_has_prop_with_text
#endif
       )
    {
	for (;;)
	{
	    head = 0;
	    c = *ptr;
	    // make sure we don't go past the end of the line
	    if (c == NUL)
	    {
		incr = 1;	// NUL at end of line only takes one column
		break;
	    }
	    // A tab gets expanded, depending on the current column
	    if (c == TAB)
#ifdef FEAT_VARTABS
		incr = tabstop_padding(vcol, ts, vts);
#else
		incr = ts - (vcol % ts);
#endif
	    else
	    {
		if (has_mbyte)
		{
		    // For utf-8, if the byte is >= 0x80, need to look at
		    // further bytes to find the cell width.
		    if (enc_utf8 && c >= 0x80)
			incr = utf_ptr2cells(ptr);
		    else
			incr = g_chartab[c] & CT_CELL_MASK;

		    // If a double-cell char doesn't fit at the end of a line
		    // it wraps to the next line, it's like this char is three
		    // cells wide.
		    if (incr == 2 && wp->w_p_wrap && MB_BYTE2LEN(*ptr) > 1
			    && in_win_border(wp, vcol))
		    {
			++incr;
			head = 1;
		    }
		}
		else
		    incr = g_chartab[c] & CT_CELL_MASK;
	    }

	    char_u *next_ptr = ptr + (*mb_ptr2len)(ptr);
	    if (next_ptr - line > pos->col) // character at pos->col
		break;

	    vcol += incr;
	    ptr = next_ptr;
	}
    }
    else
    {
	for (;;)
	{
	    // A tab gets expanded, depending on the current column.
	    // Other things also take up space.
	    head = 0;
	    incr = win_lbr_chartabsize(&cts, &head);
	    // make sure we don't go past the end of the line
	    if (*cts.cts_ptr == NUL)
	    {
		incr = 1;	// NUL at end of line only takes one column
#ifdef FEAT_PROP_POPUP
		if (cts.cts_cur_text_width > 0)
		    incr = cts.cts_cur_text_width;
		on_NUL = TRUE;
#endif
		break;
	    }
#ifdef FEAT_PROP_POPUP
	    if (cursor == &wp->w_virtcol && cts.cts_ptr == cts.cts_line)
		// do not count the virtual text above for w_curswant
		wp->w_virtcol_first_char = cts.cts_first_char;
#endif

	    char_u *next_ptr = cts.cts_ptr + (*mb_ptr2len)(cts.cts_ptr);
	    if (next_ptr - line > pos->col) // character at pos->col
		break;

	    cts.cts_vcol += incr;
	    cts.cts_ptr = next_ptr;
	}
	vcol = cts.cts_vcol;
	ptr = cts.cts_ptr;
    }
    clear_chartabsize_arg(&cts);

    if (start != NULL)
	*start = vcol + head;
    if (end != NULL)
	*end = vcol + incr - 1;
    if (cursor != NULL)
    {
	if (*ptr == TAB
		&& (State & MODE_NORMAL)
		&& !wp->w_p_list
		&& !virtual_active()
		&& !(VIsual_active
				&& (*p_sel == 'e' || LTOREQ_POS(*pos, VIsual)))
		)
	    *cursor = vcol + incr - 1;	    // cursor at end
	else
	{
#ifdef FEAT_PROP_POPUP
	    // in Insert mode, if "start_incl" is true the text gets inserted
	    // after the virtual text, thus add its width
	    if (((State & MODE_INSERT) == 0 || cts.cts_start_incl) && !on_NUL)
		// cursor is after inserted text, unless on the NUL
		vcol += cts.cts_cur_text_width;
	    else
		// insertion also happens after the "above" virtual text
		vcol += cts.cts_first_char;
#endif
	    *cursor = vcol + head;	    // cursor at start
	}
    }
}

/*
 * Get virtual cursor column in the current window, pretending 'list' is off.
 */
    colnr_T
getvcol_nolist(pos_T *posp)
{
    int		list_save = curwin->w_p_list;
    colnr_T	vcol;

    curwin->w_p_list = FALSE;
    if (posp->coladd)
	getvvcol(curwin, posp, NULL, &vcol, NULL);
    else
	getvcol(curwin, posp, NULL, &vcol, NULL);
    curwin->w_p_list = list_save;
    return vcol;
}

/*
 * Get virtual column in virtual mode.
 */
    void
getvvcol(
    win_T	*wp,
    pos_T	*pos,
    colnr_T	*start,
    colnr_T	*cursor,
    colnr_T	*end)
{
    colnr_T	col;
    colnr_T	coladd;
    colnr_T	endadd;
    char_u	*ptr;

    if (virtual_active())
    {
	// For virtual mode, only want one value
	getvcol(wp, pos, &col, NULL, NULL);

	coladd = pos->coladd;
	endadd = 0;
	// Cannot put the cursor on part of a wide character.
	ptr = ml_get_buf(wp->w_buffer, pos->lnum, FALSE);
	if (pos->col < (colnr_T)STRLEN(ptr))
	{
	    int c = (*mb_ptr2char)(ptr + pos->col);

	    if (c != TAB && vim_isprintc(c))
	    {
		endadd = (colnr_T)(char2cells(c) - 1);
		if (coladd > endadd)	// past end of line
		    endadd = 0;
		else
		    coladd = 0;
	    }
	}
	col += coladd;
	if (start != NULL)
	    *start = col;
	if (cursor != NULL)
	    *cursor = col;
	if (end != NULL)
	    *end = col + endadd;
    }
    else
	getvcol(wp, pos, start, cursor, end);
}

/*
 * Get the leftmost and rightmost virtual column of pos1 and pos2.
 * Used for Visual block mode.
 */
    void
getvcols(
    win_T	*wp,
    pos_T	*pos1,
    pos_T	*pos2,
    colnr_T	*left,
    colnr_T	*right)
{
    colnr_T	from1, from2, to1, to2;

    if (LT_POSP(pos1, pos2))
    {
	getvvcol(wp, pos1, &from1, NULL, &to1);
	getvvcol(wp, pos2, &from2, NULL, &to2);
    }
    else
    {
	getvvcol(wp, pos2, &from1, NULL, &to1);
	getvvcol(wp, pos1, &from2, NULL, &to2);
    }
    if (from2 < from1)
	*left = from2;
    else
	*left = from1;
    if (to2 > to1)
    {
	if (*p_sel == 'e' && from2 - 1 >= to1)
	    *right = from2 - 1;
	else
	    *right = to2;
    }
    else
	*right = to1;
}

/*
 * Skip over ' ' and '\t'.
 */
    char_u *
skipwhite(char_u *q)
{
    char_u	*p = q;

    while (VIM_ISWHITE(*p))
	++p;
    return p;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * skip over ' ', '\t' and '\n'.
 */
    char_u *
skipwhite_and_nl(char_u *q)
{
    char_u	*p = q;

    while (VIM_ISWHITE(*p) || *p == NL)
	++p;
    return p;
}
#endif

/*
 * getwhitecols: return the number of whitespace
 * columns (bytes) at the start of a given line
 */
    int
getwhitecols_curline(void)
{
    return getwhitecols(ml_get_curline());
}

    int
getwhitecols(char_u *p)
{
    return skipwhite(p) - p;
}

/*
 * skip over digits
 */
    char_u *
skipdigits(char_u *q)
{
    char_u	*p = q;

    while (VIM_ISDIGIT(*p))	// skip to next non-digit
	++p;
    return p;
}

#if defined(FEAT_SYN_HL) || defined(FEAT_SPELL) || defined(PROTO)
/*
 * skip over binary digits
 */
    char_u *
skipbin(char_u *q)
{
    char_u	*p = q;

    while (vim_isbdigit(*p))	// skip to next non-digit
	++p;
    return p;
}

/*
 * skip over digits and hex characters
 */
    char_u *
skiphex(char_u *q)
{
    char_u	*p = q;

    while (vim_isxdigit(*p))	// skip to next non-digit
	++p;
    return p;
}
#endif

/*
 * skip to bin digit (or NUL after the string)
 */
    char_u *
skiptobin(char_u *q)
{
    char_u	*p = q;

    while (*p != NUL && !vim_isbdigit(*p))	// skip to next digit
	++p;
    return p;
}

/*
 * skip to digit (or NUL after the string)
 */
    char_u *
skiptodigit(char_u *q)
{
    char_u	*p = q;

    while (*p != NUL && !VIM_ISDIGIT(*p))	// skip to next digit
	++p;
    return p;
}

/*
 * skip to hex character (or NUL after the string)
 */
    char_u *
skiptohex(char_u *q)
{
    char_u	*p = q;

    while (*p != NUL && !vim_isxdigit(*p))	// skip to next digit
	++p;
    return p;
}

/*
 * Variant of isdigit() that can handle characters > 0x100.
 * We don't use isdigit() here, because on some systems it also considers
 * superscript 1 to be a digit.
 * Use the VIM_ISDIGIT() macro for simple arguments.
 */
    int
vim_isdigit(int c)
{
    return (c >= '0' && c <= '9');
}

/*
 * Variant of isxdigit() that can handle characters > 0x100.
 * We don't use isxdigit() here, because on some systems it also considers
 * superscript 1 to be a digit.
 */
    int
vim_isxdigit(int c)
{
    return (c >= '0' && c <= '9')
	|| (c >= 'a' && c <= 'f')
	|| (c >= 'A' && c <= 'F');
}

/*
 * Corollary of vim_isdigit and vim_isxdigit() that can handle
 * characters > 0x100.
 */
    int
vim_isbdigit(int c)
{
    return (c == '0' || c == '1');
}

    static int
vim_isodigit(int c)
{
    return (c >= '0' && c <= '7');
}

/*
 * Vim's own character class functions.  These exist because many library
 * islower()/toupper() etc. do not work properly: they crash when used with
 * invalid values or can't handle latin1 when the locale is C.
 * Speed is most important here.
 */
#define LATIN1LOWER 'l'
#define LATIN1UPPER 'U'

static char_u latin1flags[257] = "                                                                 UUUUUUUUUUUUUUUUUUUUUUUUUU      llllllllllllllllllllllllll                                                                     UUUUUUUUUUUUUUUUUUUUUUU UUUUUUUllllllllllllllllllllllll llllllll";
static char_u latin1upper[257] = "                                 !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`ABCDEFGHIJKLMNOPQRSTUVWXYZ{|}~\x7f\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8a\x8b\x8c\x8d\x8e\x8f\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9a\x9b\x9c\x9d\x9e\x9f\xa0\xa1\xa2\xa3\xa4\xa5\xa6\xa7\xa8\xa9\xaa\xab\xac\xad\xae\xaf\xb0\xb1\xb2\xb3\xb4\xb5\xb6\xb7\xb8\xb9\xba\xbb\xbc\xbd\xbe\xbf\xc0\xc1\xc2\xc3\xc4\xc5\xc6\xc7\xc8\xc9\xca\xcb\xcc\xcd\xce\xcf\xd0\xd1\xd2\xd3\xd4\xd5\xd6\xd7\xd8\xd9\xda\xdb\xdc\xdd\xde\xdf\xc0\xc1\xc2\xc3\xc4\xc5\xc6\xc7\xc8\xc9\xca\xcb\xcc\xcd\xce\xcf\xd0\xd1\xd2\xd3\xd4\xd5\xd6\xf7\xd8\xd9\xda\xdb\xdc\xdd\xde\xff";
static char_u latin1lower[257] = "                                 !\"#$%&'()*+,-./0123456789:;<=>?@abcdefghijklmnopqrstuvwxyz[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~\x7f\x80\x81\x82\x83\x84\x85\x86\x87\x88\x89\x8a\x8b\x8c\x8d\x8e\x8f\x90\x91\x92\x93\x94\x95\x96\x97\x98\x99\x9a\x9b\x9c\x9d\x9e\x9f\xa0\xa1\xa2\xa3\xa4\xa5\xa6\xa7\xa8\xa9\xaa\xab\xac\xad\xae\xaf\xb0\xb1\xb2\xb3\xb4\xb5\xb6\xb7\xb8\xb9\xba\xbb\xbc\xbd\xbe\xbf\xe0\xe1\xe2\xe3\xe4\xe5\xe6\xe7\xe8\xe9\xea\xeb\xec\xed\xee\xef\xf0\xf1\xf2\xf3\xf4\xf5\xf6\xd7\xf8\xf9\xfa\xfb\xfc\xfd\xfe\xdf\xe0\xe1\xe2\xe3\xe4\xe5\xe6\xe7\xe8\xe9\xea\xeb\xec\xed\xee\xef\xf0\xf1\xf2\xf3\xf4\xf5\xf6\xf7\xf8\xf9\xfa\xfb\xfc\xfd\xfe\xff";

    int
vim_islower(int c)
{
    if (c <= '@')
	return FALSE;
    if (c >= 0x80)
    {
	if (enc_utf8)
	    return utf_islower(c);
	if (c >= 0x100)
	{
#ifdef HAVE_ISWLOWER
	    if (has_mbyte)
		return iswlower(c);
#endif
	    // islower() can't handle these chars and may crash
	    return FALSE;
	}
	if (enc_latin1like)
	    return (latin1flags[c] & LATIN1LOWER) == LATIN1LOWER;
    }
    return SAFE_islower(c);
}

    int
vim_isupper(int c)
{
    if (c <= '@')
	return FALSE;
    if (c >= 0x80)
    {
	if (enc_utf8)
	    return utf_isupper(c);
	if (c >= 0x100)
	{
#ifdef HAVE_ISWUPPER
	    if (has_mbyte)
		return iswupper(c);
#endif
	    // islower() can't handle these chars and may crash
	    return FALSE;
	}
	if (enc_latin1like)
	    return (latin1flags[c] & LATIN1UPPER) == LATIN1UPPER;
    }
    return SAFE_isupper(c);
}

    int
vim_isalpha(int c)
{
    return vim_islower(c) || vim_isupper(c);
}

    int
vim_toupper(int c)
{
    if (c <= '@')
	return c;
    if (c >= 0x80 || !(cmp_flags & CMP_KEEPASCII))
    {
	if (enc_utf8)
	    return utf_toupper(c);
	if (c >= 0x100)
	{
#ifdef HAVE_TOWUPPER
	    if (has_mbyte)
		return towupper(c);
#endif
	    // toupper() can't handle these chars and may crash
	    return c;
	}
	if (enc_latin1like)
	    return latin1upper[c];
    }
    if (c < 0x80 && (cmp_flags & CMP_KEEPASCII))
	return TOUPPER_ASC(c);
    return TOUPPER_LOC(c);
}

    int
vim_tolower(int c)
{
    if (c <= '@')
	return c;
    if (c >= 0x80 || !(cmp_flags & CMP_KEEPASCII))
    {
	if (enc_utf8)
	    return utf_tolower(c);
	if (c >= 0x100)
	{
#ifdef HAVE_TOWLOWER
	    if (has_mbyte)
		return towlower(c);
#endif
	    // tolower() can't handle these chars and may crash
	    return c;
	}
	if (enc_latin1like)
	    return latin1lower[c];
    }
    if (c < 0x80 && (cmp_flags & CMP_KEEPASCII))
	return TOLOWER_ASC(c);
    return TOLOWER_LOC(c);
}

/*
 * skiptowhite: skip over text until ' ' or '\t' or NUL.
 */
    char_u *
skiptowhite(char_u *p)
{
    while (*p != ' ' && *p != '\t' && *p != NUL)
	++p;
    return p;
}

/*
 * skiptowhite_esc: Like skiptowhite(), but also skip escaped chars
 */
    char_u *
skiptowhite_esc(char_u *p)
{
    while (*p != ' ' && *p != '\t' && *p != NUL)
    {
	if ((*p == '\\' || *p == Ctrl_V) && *(p + 1) != NUL)
	    ++p;
	++p;
    }
    return p;
}

/*
 * Get a number from a string and skip over it.
 * Note: the argument is a pointer to a char_u pointer!
 */
    long
getdigits(char_u **pp)
{
    char_u	*p;
    long	retval;

    p = *pp;
    retval = atol((char *)p);
    if (*p == '-')		// skip negative sign
	++p;
    p = skipdigits(p);		// skip to next non-digit
    *pp = p;
    return retval;
}

/*
 * Like getdigits() but allow for embedded single quotes.
 */
    long
getdigits_quoted(char_u **pp)
{
    char_u	*p = *pp;
    long	retval = 0;

    if (*p == '-')
	++p;
    while (VIM_ISDIGIT(*p))
    {
	if (retval >= LONG_MAX / 10 - 10)
	    retval = LONG_MAX;
	else
	    retval = retval * 10 - '0' + *p;
	++p;
	if (in_vim9script() && *p == '\'' && VIM_ISDIGIT(p[1]))
	    ++p;
    }
    if (**pp == '-')
    {
	if (retval == LONG_MAX)
	    retval = LONG_MIN;
	else
	    retval = -retval;
    }
    *pp = p;
    return retval;
}

/*
 * Return TRUE if "lbuf" is empty or only contains blanks.
 */
    int
vim_isblankline(char_u *lbuf)
{
    char_u	*p;

    p = skipwhite(lbuf);
    return (*p == NUL || *p == '\r' || *p == '\n');
}

/*
 * Convert a string into a long and/or unsigned long, taking care of
 * hexadecimal, octal, and binary numbers.  Accepts a '-' sign.
 * If "prep" is not NULL, returns a flag to indicate the type of the number:
 *  0	    decimal
 *  '0'	    octal
 *  'O'	    octal
 *  'o'	    octal
 *  'B'	    bin
 *  'b'	    bin
 *  'X'	    hex
 *  'x'	    hex
 * If "len" is not NULL, the length of the number in characters is returned.
 * If "nptr" is not NULL, the signed result is returned in it.
 * If "unptr" is not NULL, the unsigned result is returned in it.
 * If "what" contains STR2NR_BIN recognize binary numbers
 * If "what" contains STR2NR_OCT recognize octal numbers
 * If "what" contains STR2NR_HEX recognize hex numbers
 * If "what" contains STR2NR_FORCE always assume bin/oct/hex.
 * If "what" contains STR2NR_QUOTE ignore embedded single quotes
 * If maxlen > 0, check at a maximum maxlen chars.
 * If strict is TRUE, check the number strictly. return *len = 0 if fail.
 */
    void
vim_str2nr(
    char_u		*start,
    int			*prep,	    // return: type of number 0 = decimal, 'x'
				    // or 'X' is hex, '0', 'o' or 'O' is octal,
				    // 'b' or 'B' is bin
    int			*len,	    // return: detected length of number
    int			what,	    // what numbers to recognize
    varnumber_T		*nptr,	    // return: signed result
    uvarnumber_T	*unptr,	    // return: unsigned result
    int			maxlen,     // max length of string to check
    int			strict,     // check strictly
    int			*overflow)  // when not NULL set to TRUE for overflow
{
    char_u	    *ptr = start;
    int		    pre = 0;		// default is decimal
    int		    negative = FALSE;
    uvarnumber_T    un = 0;
    int		    n;

    if (len != NULL)
	*len = 0;

    if (ptr[0] == '-')
    {
	negative = TRUE;
	++ptr;
    }

    // Recognize hex, octal, and bin.
    if (ptr[0] == '0' && ptr[1] != '8' && ptr[1] != '9'
					       && (maxlen == 0 || maxlen > 1))
    {
	pre = ptr[1];
	if ((what & STR2NR_HEX)
		&& (pre == 'X' || pre == 'x') && vim_isxdigit(ptr[2])
		&& (maxlen == 0 || maxlen > 2))
	    // hexadecimal
	    ptr += 2;
	else if ((what & STR2NR_BIN)
		&& (pre == 'B' || pre == 'b') && vim_isbdigit(ptr[2])
		&& (maxlen == 0 || maxlen > 2))
	    // binary
	    ptr += 2;
	else if ((what & STR2NR_OOCT)
		&& (pre == 'O' || pre == 'o') && vim_isodigit(ptr[2])
		&& (maxlen == 0 || maxlen > 2))
	    // octal with prefix "0o"
	    ptr += 2;
	else
	{
	    // decimal or octal, default is decimal
	    pre = 0;
	    if (what & STR2NR_OCT)
	    {
		// Don't interpret "0", "08" or "0129" as octal.
		for (n = 1; n != maxlen && VIM_ISDIGIT(ptr[n]); ++n)
		{
		    if (ptr[n] > '7')
		    {
			pre = 0;	// can't be octal
			break;
		    }
		    pre = '0';	// assume octal
		}
	    }
	}
    }

    // Do the conversion manually to avoid sscanf() quirks.
    n = 1;
    if (pre == 'B' || pre == 'b'
			     || ((what & STR2NR_BIN) && (what & STR2NR_FORCE)))
    {
	// bin
	if (pre != 0)
	    n += 2;	    // skip over "0b"
	while ('0' <= *ptr && *ptr <= '1')
	{
	    // avoid ubsan error for overflow
	    if (un <= UVARNUM_MAX / 2)
		un = 2 * un + (uvarnumber_T)(*ptr - '0');
	    else
	    {
		un = UVARNUM_MAX;
		if (overflow != NULL)
		    *overflow = TRUE;
	    }
	    ++ptr;
	    if (n++ == maxlen)
		break;
	    if ((what & STR2NR_QUOTE) && *ptr == '\''
					     && '0' <= ptr[1] && ptr[1] <= '1')
	    {
		++ptr;
		if (n++ == maxlen)
		    break;
	    }
	}
    }
    else if (pre == 'O' || pre == 'o' ||
		pre == '0' || ((what & STR2NR_OCT) && (what & STR2NR_FORCE)))
    {
	// octal
	if (pre != 0 && pre != '0')
	    n += 2;	    // skip over "0o"
	while ('0' <= *ptr && *ptr <= '7')
	{
	    // avoid ubsan error for overflow
	    if (un <= UVARNUM_MAX / 8)
		un = 8 * un + (uvarnumber_T)(*ptr - '0');
	    else
	    {
		un = UVARNUM_MAX;
		if (overflow != NULL)
		    *overflow = TRUE;
	    }
	    ++ptr;
	    if (n++ == maxlen)
		break;
	    if ((what & STR2NR_QUOTE) && *ptr == '\''
					     && '0' <= ptr[1] && ptr[1] <= '7')
	    {
		++ptr;
		if (n++ == maxlen)
		    break;
	    }
	}
    }
    else if (pre != 0 || ((what & STR2NR_HEX) && (what & STR2NR_FORCE)))
    {
	// hex
	if (pre != 0)
	    n += 2;	    // skip over "0x"
	while (vim_isxdigit(*ptr))
	{
	    // avoid ubsan error for overflow
	    if (un <= UVARNUM_MAX / 16)
		un = 16 * un + (uvarnumber_T)hex2nr(*ptr);
	    else
	    {
		un = UVARNUM_MAX;
		if (overflow != NULL)
		    *overflow = TRUE;
	    }
	    ++ptr;
	    if (n++ == maxlen)
		break;
	    if ((what & STR2NR_QUOTE) && *ptr == '\'' && vim_isxdigit(ptr[1]))
	    {
		++ptr;
		if (n++ == maxlen)
		    break;
	    }
	}
    }
    else
    {
	// decimal
	while (VIM_ISDIGIT(*ptr))
	{
	    uvarnumber_T    digit = (uvarnumber_T)(*ptr - '0');

	    // avoid ubsan error for overflow
	    if (un < UVARNUM_MAX / 10
		    || (un == UVARNUM_MAX / 10 && digit <= UVARNUM_MAX % 10))
		un = 10 * un + digit;
	    else
	    {
		un = UVARNUM_MAX;
		if (overflow != NULL)
		    *overflow = TRUE;
	    }
	    ++ptr;
	    if (n++ == maxlen)
		break;
	    if ((what & STR2NR_QUOTE) && *ptr == '\'' && VIM_ISDIGIT(ptr[1]))
	    {
		++ptr;
		if (n++ == maxlen)
		    break;
	    }
	}
    }

    // Check for an alphanumeric character immediately following, that is
    // most likely a typo.
    if (strict && n - 1 != maxlen && ASCII_ISALNUM(*ptr))
	return;

    if (prep != NULL)
	*prep = pre;
    if (len != NULL)
	*len = (int)(ptr - start);
    if (nptr != NULL)
    {
	if (negative)   // account for leading '-' for decimal numbers
	{
	    // avoid ubsan error for overflow
	    if (un > VARNUM_MAX)
	    {
		*nptr = VARNUM_MIN;
		if (overflow != NULL)
		    *overflow = TRUE;
	    }
	    else
		*nptr = -(varnumber_T)un;
	}
	else
	{
	    // prevent a large unsigned number to become negative
	    if (un > VARNUM_MAX)
	    {
		un = VARNUM_MAX;
		if (overflow != NULL)
		    *overflow = TRUE;
	    }
	    *nptr = (varnumber_T)un;
	}
    }
    if (unptr != NULL)
	*unptr = un;
}

/*
 * Return the value of a single hex character.
 * Only valid when the argument is '0' - '9', 'A' - 'F' or 'a' - 'f'.
 */
    int
hex2nr(int c)
{
    if (c >= 'a' && c <= 'f')
	return c - 'a' + 10;
    if (c >= 'A' && c <= 'F')
	return c - 'A' + 10;
    return c - '0';
}

/*
 * Convert two hex characters to a byte.
 * Return -1 if one of the characters is not hex.
 */
    int
hexhex2nr(char_u *p)
{
    if (!vim_isxdigit(p[0]) || !vim_isxdigit(p[1]))
	return -1;
    return (hex2nr(p[0]) << 4) + hex2nr(p[1]);
}

/*
 * Return TRUE if "str" starts with a backslash that should be removed.
 * For MS-DOS, MSWIN and OS/2 this is only done when the character after the
 * backslash is not a normal file name character.
 * '$' is a valid file name character, we don't remove the backslash before
 * it.  This means it is not possible to use an environment variable after a
 * backslash.  "C:\$VIM\doc" is taken literally, only "$VIM\doc" works.
 * Although "\ name" is valid, the backslash in "Program\ files" must be
 * removed.  Assume a file name doesn't start with a space.
 * For multi-byte names, never remove a backslash before a non-ascii
 * character, assume that all multi-byte characters are valid file name
 * characters.
 */
    int
rem_backslash(char_u *str)
{
#ifdef BACKSLASH_IN_FILENAME
    return (str[0] == '\\'
	    && str[1] < 0x80
	    && (str[1] == ' '
		|| (str[1] != NUL
		    && str[1] != '*'
		    && str[1] != '?'
		    && !vim_isfilec(str[1]))));
#else
    return (str[0] == '\\' && str[1] != NUL);
#endif
}

/*
 * Halve the number of backslashes in a file name argument.
 * For MS-DOS we only do this if the character after the backslash
 * is not a normal file character.
 */
    void
backslash_halve(char_u *p)
{
    for ( ; *p; ++p)
	if (rem_backslash(p))
	    STRMOVE(p, p + 1);
}

/*
 * backslash_halve() plus save the result in allocated memory.
 * However, returns "p" when out of memory.
 */
    char_u *
backslash_halve_save(char_u *p)
{
    char_u	*res;

    res = vim_strsave(p);
    if (res == NULL)
	return p;
    backslash_halve(res);
    return res;
}
