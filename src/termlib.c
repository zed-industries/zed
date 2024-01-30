/* vi:set ts=8 sts=4 sw=4 noet: */
/*
 * The following software is (C) 1984 Peter da Silva, the Mad Australian, in
 * the public domain. It may be re-distributed for any purpose with the
 * inclusion of this notice.
 */

// Modified by Bram Moolenaar for use with VIM - Vi Improved.
// A few bugs removed by Olaf 'Rhialto' Seibert.

// TERMLIB: Terminal independent database.

#include "vim.h"
#include "termlib.pro"

#if !defined(AMIGA) && !defined(VMS)
# include <sgtty.h>
#endif

static int  getent(char *, char *, FILE *, int);
static int  nextent(char *, FILE *, int);
static int  _match(char *, char *);
static char *_addfmt(char *, char *, int);
static char *_find(char *, char *);

/*
 * Global variables for termlib
 */

char	*tent;		      // Pointer to terminal entry, set by tgetent
char	PC = 0;		      // Pad character, default NULL
char	*UP = 0, *BC = 0;     // Pointers to UP and BC strings from database
short	ospeed;		      // Baud rate (1-16, 1=300, 16=19200), as in stty

/*
 * Module: tgetent
 *
 * Purpose: Get termcap entry for <term> into buffer at <tbuf>.
 *
 * Calling conventions: char tbuf[TBUFSZ+], term=canonical name for terminal.
 *
 * Returned values: 1 = success, -1 = can't open file,
 *	    0 = can't find terminal.
 *
 * Notes:
 * - Should probably supply static buffer.
 * - Uses environment variables "TERM" and "TERMCAP". If TERM = term (that is,
 *   if the argument matches the environment) then it looks at TERMCAP.
 * - If TERMCAP begins with a slash, then it assumes this is the file to
 *   search rather than /etc/termcap.
 * - If TERMCAP does not begin with a slash, and it matches TERM, then this is
 *   used as the entry.
 * - This could be simplified considerably for non-UNIX systems.
 */

#ifndef TERMCAPFILE
# ifdef AMIGA
#  define TERMCAPFILE "s:termcap"
# else
#  ifdef VMS
#   define TERMCAPFILE "VIMRUNTIME:termcap"
#  else
#   define TERMCAPFILE "/etc/termcap"
#  endif
# endif
#endif

    int
tgetent(
    char    *tbuf,		// Buffer to hold termcap entry, TBUFSZ bytes max
    char    *term)		// Name of terminal
{
    char    tcbuf[32];		// Temp buffer to handle
    char    *tcptr = tcbuf;	// extended entries
    char    *tcap = TERMCAPFILE; // Default termcap file
    char    *tmp;
    FILE    *termcap;
    int	    retval = 0;
    int	    len;

    if ((tmp = (char *)mch_getenv((char_u *)"TERMCAP")) != NULL)
    {
	if (*tmp == '/')		// TERMCAP = name of termcap file
	{
	    tcap = tmp ;
#if defined(AMIGA)
	    // Convert /usr/share/lib/termcap to usr:share/lib/termcap
	    tcap++;
	    tmp = strchr(tcap, '/');
	    if (tmp)
		*tmp = ':';
#endif
	}
	else				// TERMCAP = termcap entry itself
	{
	    int tlen = strlen(term);

	    while (*tmp && *tmp != ':')		// Check if TERM matches
	    {
		char *nexttmp;

		while (*tmp == '|')
		    tmp++;
		nexttmp  = _find(tmp, ":|");	// Rhialto
		if (tmp+tlen == nexttmp && _match(tmp, term) == tlen)
		{
		    strcpy(tbuf, tmp);
		    tent = tbuf;
		    return 1;
		}
		else
		    tmp = nexttmp;
	    }
	}
    }
    if (!(termcap = mch_fopen(tcap, "r")))
    {
	strcpy(tbuf, tcap);
	return -1;
    }

    len = 0;
    while (getent(tbuf + len, term, termcap, TBUFSZ - len))
    {
	tcptr = tcbuf;				// Rhialto
	if ((term = tgetstr("tc", &tcptr)))	// extended entry
	{
	    rewind(termcap);
	    len = strlen(tbuf);
	}
	else
	{
	    retval = 1;
	    tent = tbuf;	// reset it back to the beginning
	    break;
	}
    }
    fclose(termcap);
    return retval;
}

    static int
getent(char *tbuf, char *term, FILE *termcap, int buflen)
{
    char    *tptr;
    int	    tlen = strlen(term);

    while (nextent(tbuf, termcap, buflen))	// For each possible entry
    {
	tptr = tbuf;
	while (*tptr && *tptr != ':')		// : terminates name field
	{
	    char    *nexttptr;

	    while (*tptr == '|')		// | separates names
		tptr++;
	    nexttptr = _find(tptr, ":|");	// Rhialto
	    if (tptr + tlen == nexttptr &&
		_match(tptr, term) == tlen)	// FOUND!
	    {
		tent = tbuf;
		return 1;
	    }
	    else				// Look for next name
		tptr = nexttptr;
	}
    }
    return 0;
}

/*
 * Read 1 entry from TERMCAP file.
 */
    static int
nextent(char *tbuf, FILE *termcap, int buflen)
{
    char *lbuf = tbuf;				// lbuf=line buffer
				// read lines straight into buffer

    while (lbuf < tbuf+buflen &&		// There's room and
	  fgets(lbuf, (int)(tbuf+buflen-lbuf), termcap)) // another line
    {
	int llen = strlen(lbuf);

	if (*lbuf == '#')			// eat comments
	    continue;
	if (lbuf[-1] == ':' &&			// and whitespace
	    lbuf[0] == '\t' &&
	    lbuf[1] == ':')
	{
	    STRMOVE(lbuf, lbuf + 2);
	    llen -= 2;
	}
	if (lbuf[llen-2] == '\\')		// and continuations
	    lbuf += llen-2;
	else
	{
	    lbuf[llen-1]=0;			// no continuation, return
	    return 1;
	}
    }

    return 0;					// ran into end of file
}

/*
 * Module: tgetflag
 *
 * Purpose: returns flag true or false as to the existence of a given entry.
 * used with 'bs', 'am', etc...
 *
 * Calling conventions: id is the 2 character capability id.
 *
 * Returned values: 1 for success, 0 for failure.
 */

    int
tgetflag(char *id)
{
    char    buf[256], *ptr = buf;

    return tgetstr(id, &ptr) ? 1 : 0;
}

/*
 * Module: tgetnum
 *
 * Purpose: get numeric value such as 'li' or 'co' from termcap.
 *
 * Calling conventions: id = 2 character id.
 *
 * Returned values: -1 for failure, else numerical value.
 */

    int
tgetnum(char *id)
{
    char *ptr, buf[256];
    ptr = buf;

    if (tgetstr(id, &ptr))
	return atoi(buf);
    else
	return 0;
}

/*
 * Module: tgetstr
 *
 * Purpose: get terminal capability string from database.
 *
 * Calling conventions: id is the two character capability id.
 *	    (*buf) points into a hold buffer for the
 *	    id. the capability is copied into the buffer
 *	    and (*buf) is advanced to point to the next
 *	    free byte in the buffer.
 *
 * Returned values: 0 = no such entry, otherwise returns original
 *	    (*buf) (now a pointer to the string).
 *
 * Notes
 *	It also decodes certain escape sequences in the buffer.
 *  they should be obvious from the code:
 *	\E = escape.
 *	\n, \r, \t, \f, \b match the 'c' escapes.
 *	^x matches control-x (^@...^_).
 *	\nnn matches nnn octal.
 *	\x, where x is anything else, matches x. I differ
 *  from the standard library here, in that I allow ^: to match
 *  :.
 *
 */

    char *
tgetstr(char *id, char **buf)
{
    int		len = strlen(id);
    char	*tmp=tent;
    char	*hold;
    int		i;

    do {
	tmp = _find(tmp, ":");			// For each field
	while (*tmp == ':')			// skip empty fields
	    tmp++;
	if (!*tmp)
	    break;

	if (_match(id, tmp) == len)
	{
	    tmp += len;				// find '=' '@' or '#'
	    if (*tmp == '@')			// :xx@: entry for tc
		return 0;			// deleted entry
	    hold= *buf;
	    while (*++tmp && *tmp != ':')	// not at end of field
		{
		switch(*tmp)
		{
		case '\\':			// Expand escapes here
		    switch(*++tmp)
		    {
		    case 0:			// ignore backslashes
			tmp--;			// at end of entry
			break;			// shouldn't happen
		    case 'e':
		    case 'E':			// ESC
			*(*buf)++ = ESC;
			break;
		    case 'n':			// \n
			*(*buf)++ = '\n';
			break;
		    case 'r':			// \r
			*(*buf)++ = '\r';
			break;
		    case 't':			// \t
			*(*buf)++ = '\t';
			break;
		    case 'b':			// \b
			*(*buf)++ = '\b';
			break;
		    case 'f':			// \f
			*(*buf)++ = '\f';
			break;
		    case '0':			// \nnn
		    case '1':
		    case '2':
		    case '3':
		    case '4':
		    case '5':
		    case '6':
		    case '7':
		    case '8':
		    case '9':
			**buf = 0;
			    // get up to three digits
			for (i = 0; i < 3 && VIM_ISDIGIT(*tmp); ++i)
			    **buf = **buf * 8 + *tmp++ - '0';
			(*buf)++;
			tmp--;
			break;
		    default:			// \x, for all other x
			*(*buf)++= *tmp;
		    }
		    break;
		case '^':			// control characters
		    ++tmp;
		    *(*buf)++ = Ctrl_chr(*tmp);
		    break;
		default:
		    *(*buf)++ = *tmp;
		}
	    }
	    *(*buf)++ = 0;
	    return hold;
	}
    } while (*tmp);

    return 0;
}

/*
 * Module: tgoto
 *
 * Purpose: decode cm cursor motion string.
 *
 * Calling conventions: cm is cursor motion string.  line, col, are the
 * desired destination.
 *
 * Returned values: a string pointing to the decoded string, or "OOPS" if it
 * cannot be decoded.
 *
 * Notes
 *	The accepted escapes are:
 *	%d	 as in printf, 0 origin.
 *	%2, %3   like %02d, %03d in printf.
 *	%.	 like %c
 *	%+x	 adds <x> to value, then %.
 *	%>xy     if value>x, adds y. No output.
 *	%i	 increments line& col, no output.
 *	%r	 reverses order of line&col. No output.
 *	%%	 prints as a single %.
 *	%n	 exclusive or row & col with 0140.
 *	%B	 BCD, no output.
 *	%D	 reverse coding (x-2*(x%16)), no output.
 */

    char *
tgoto(
    char    *cm,				// cm string, from termcap
    int	    col,				// column, x position
    int	    line)				// line, y position
{
    char    gx, gy,				// x, y
	*ptr,					// pointer in 'cm'
	reverse = 0,				// reverse flag
	*bufp,					// pointer in returned string
	addup = 0,				// add upline
	addbak = 0,				// add backup
	c;
    static char buffer[32];

    if (!cm)
	return "OOPS";				// Kludge, but standard

    bufp = buffer;
    ptr = cm;

    while (*ptr)
    {
	if ((c = *ptr++) != '%')		// normal char
	{
	    *bufp++ = c;
	}
	else
	{				// % escape
	    switch(c = *ptr++)
	    {
	    case 'd':				// decimal
		bufp = _addfmt(bufp, "%d", line);
		line = col;
		break;
	    case '2':				// 2 digit decimal
		bufp = _addfmt(bufp, "%02d", line);
		line = col;
		break;
	    case '3':				// 3 digit decimal
		bufp = _addfmt(bufp, "%03d", line);
		line = col;
		break;
	    case '>':				// %>xy: if >x, add y
		gx = *ptr++;
		gy = *ptr++;
		if (col>gx) col += gy;
		if (line>gx) line += gy;
		break;
	    case '+':				// %+c: add c
		line += *ptr++;
	    case '.':				// print x/y
		if (line == '\t' ||		// these are
		   line == '\n' ||		// chars that
		   line == '\004' ||		// UNIX hates
		   line == '\0')
		{
		    line++;			// so go to next pos
		    if (reverse == (line == col))
			addup=1;		// and mark UP
		    else
			addbak=1;		// or BC
		}
		*bufp++=line;
		line = col;
		break;
	    case 'r':				// r: reverse
		gx = line;
		line = col;
		col = gx;
		reverse = 1;
		break;
	    case 'i':			// increment (1-origin screen)
		col++;
		line++;
		break;
	    case '%':				// %%=% literally
		*bufp++='%';
		break;
	    case 'n':				// magic DM2500 code
		line ^= 0140;
		col ^= 0140;
		break;
	    case 'B':				// bcd encoding
		line = line/10<<4+line%10;
		col = col/10<<4+col%10;
		break;
	    case 'D':				// magic Delta Data code
		line = line-2*(line&15);
		col = col-2*(col&15);
		break;
	    default:				// Unknown escape
		return "OOPS";
	    }
	}
    }

    if (addup)					// add upline
	if (UP)
	{
	    ptr=UP;
	    while (VIM_ISDIGIT(*ptr) || *ptr == '.')
		ptr++;
	    if (*ptr == '*')
		ptr++;
	    while (*ptr)
		*bufp++ = *ptr++;
	}

    if (addbak)					// add backspace
	if (BC)
	{
	    ptr=BC;
	    while (VIM_ISDIGIT(*ptr) || *ptr == '.')
		ptr++;
	    if (*ptr == '*')
		ptr++;
	    while (*ptr)
		*bufp++ = *ptr++;
	}
	else
	    *bufp++='\b';

    *bufp = 0;

    return(buffer);
}

/*
 * Module: tputs
 *
 * Purpose: decode padding information
 *
 * Calling conventions: cp = string to be padded, affcnt = # of items affected
 *	(lines, characters, whatever), outc = routine to output 1 character.
 *
 * Returned values: none
 *
 * Notes
 *	cp has padding information ahead of it, in the form
 *  nnnTEXT or nnn*TEXT. nnn is the number of milliseconds to delay,
 *  and may be a decimal (nnn.mmm). If the asterisk is given, then
 *  the delay is multiplied by afcnt. The delay is produced by outputting
 *  a number of nulls (or other padding char) after printing the
 *  TEXT.
 *
 */

long _bauds[16]={
    0,	50, 75,	110,
    134,    150,    200,    300,
    600,    1200,   1800,   2400,
    4800,   9600,   19200,  19200 };

    int
tputs(
    char *cp,				// string to print
    int affcnt,				// Number of lines affected
    void (*outc)(unsigned int))		// routine to output 1 character
{
    long    frac,			// 10^(#digits after decimal point)
	counter,			// digits
	atol(const char *);

    if (VIM_ISDIGIT(*cp))
    {
	counter = 0;
	frac = 1000;
	while (VIM_ISDIGIT(*cp))
	    counter = counter * 10L + (long)(*cp++ - '0');
	if (*cp == '.')
	    while (VIM_ISDIGIT(*++cp))
	    {
		counter = counter * 10L + (long)(*cp++ - '0');
		frac = frac * 10;
	    }
	if (*cp!='*')			// multiply by affected lines
	{
	    if (affcnt>1) affcnt = 1;
	}
	else
	    cp++;

	// Calculate number of characters for padding counter/frac ms delay
	if (ospeed)
	    counter = (counter * _bauds[ospeed] * (long)affcnt) / frac;

	while (*cp)			// output string
	    (*outc)(*cp++);
	if (ospeed)
	    while (counter--)		// followed by pad characters
		(*outc)(PC);
    }
    else
	while (*cp)
	    (*outc)(*cp++);
    return 0;
}

/*
 * Module: tutil.c
 *
 * Purpose: Utility routines for TERMLIB functions.
 * Returns length of text common to s1 and s2.
 */
    static int
_match(char *s1, char *s2)
{
    int i = 0;

    while (s1[i] && s1[i] == s2[i])
	i++;

    return i;
}

/*
 * finds next c in s that's a member of set, returns pointer
 */
    static char *
_find(char *s, char *set)
{
    for (; *s; s++)
    {
	char	*ptr = set;

	while (*ptr && *s != *ptr)
	    ptr++;

	if (*ptr)
	    return s;
    }

    return s;
}

/*
 * add val to buf according to format fmt
 */
    static char *
_addfmt(char *buf, char *fmt, int val)
{
    sprintf(buf, fmt, val);
    while (*buf)
	buf++;
    return buf;
}
