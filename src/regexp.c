/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * Handling of regular expressions: vim_regcomp(), vim_regexec(), vim_regsub()
 */

// By default: do not create debugging logs or files related to regular
// expressions, even when compiling with -DDEBUG.
// Uncomment the second line to get the regexp debugging.
#undef DEBUG
// #define DEBUG

#include "vim.h"

#ifdef DEBUG
// show/save debugging data when BT engine is used
# define BT_REGEXP_DUMP
// save the debugging data to a file instead of displaying it
# define BT_REGEXP_LOG
# define BT_REGEXP_DEBUG_LOG
# define BT_REGEXP_DEBUG_LOG_NAME	"bt_regexp_debug.log"
#endif

#ifdef FEAT_RELTIME
static sig_atomic_t dummy_timeout_flag = 0;
static volatile sig_atomic_t *timeout_flag = &dummy_timeout_flag;
#endif

/*
 * Magic characters have a special meaning, they don't match literally.
 * Magic characters are negative.  This separates them from literal characters
 * (possibly multi-byte).  Only ASCII characters can be Magic.
 */
#define Magic(x)	((int)(x) - 256)
#define un_Magic(x)	((x) + 256)
#define is_Magic(x)	((x) < 0)

    static int
no_Magic(int x)
{
    if (is_Magic(x))
	return un_Magic(x);
    return x;
}

    static int
toggle_Magic(int x)
{
    if (is_Magic(x))
	return un_Magic(x);
    return Magic(x);
}

#ifdef FEAT_RELTIME
static int timeout_nesting = 0;

/*
 * Start a timer that will cause the regexp to abort after "msec".
 * This doesn't work well recursively.  In case it happens anyway, the first
 * set timeout will prevail, nested ones are ignored.
 * The caller must make sure there is a matching disable_regexp_timeout() call!
 */
    void
init_regexp_timeout(long msec)
{
    if (timeout_nesting == 0)
	timeout_flag = start_timeout(msec);
    ++timeout_nesting;
}

    void
disable_regexp_timeout(void)
{
    if (timeout_nesting == 0)
	iemsg("disable_regexp_timeout() called without active timer");
    else if (--timeout_nesting == 0)
    {
	stop_timeout();
	timeout_flag = &dummy_timeout_flag;
    }
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
# ifdef FEAT_RELTIME
static sig_atomic_t *saved_timeout_flag;
# endif

/*
 * Used at the debug prompt: disable the timeout so that expression evaluation
 * can used patterns.
 * Must be followed by calling restore_timeout_for_debugging().
 */
    void
save_timeout_for_debugging(void)
{
# ifdef FEAT_RELTIME
    saved_timeout_flag = (sig_atomic_t *)timeout_flag;
    timeout_flag = &dummy_timeout_flag;
# endif
}

    void
restore_timeout_for_debugging(void)
{
# ifdef FEAT_RELTIME
    timeout_flag = saved_timeout_flag;
# endif
}
#endif

/*
 * The first byte of the BT regexp internal "program" is actually this magic
 * number; the start node begins in the second byte.  It's used to catch the
 * most severe mutilation of the program by the caller.
 */

#define REGMAGIC	0234

/*
 * Utility definitions.
 */
#define UCHARAT(p)	((int)*(char_u *)(p))

// Used for an error (down from) vim_regcomp(): give the error message, set
// rc_did_emsg and return NULL
#define EMSG_RET_NULL(m) return (emsg((m)), rc_did_emsg = TRUE, (void *)NULL)
#define IEMSG_RET_NULL(m) return (iemsg((m)), rc_did_emsg = TRUE, (void *)NULL)
#define EMSG_RET_FAIL(m) return (emsg((m)), rc_did_emsg = TRUE, FAIL)
#define EMSG2_RET_NULL(m, c) return (semsg((const char *)(m), (c) ? "" : "\\"), rc_did_emsg = TRUE, (void *)NULL)
#define EMSG3_RET_NULL(m, c, a) return (semsg((const char *)(m), (c) ? "" : "\\", (a)), rc_did_emsg = TRUE, (void *)NULL)
#define EMSG2_RET_FAIL(m, c) return (semsg((const char *)(m), (c) ? "" : "\\"), rc_did_emsg = TRUE, FAIL)
#define EMSG_ONE_RET_NULL EMSG2_RET_NULL(_(e_invalid_item_in_str_brackets), reg_magic == MAGIC_ALL)


#define MAX_LIMIT	(32767L << 16L)

#define NOT_MULTI	0
#define MULTI_ONE	1
#define MULTI_MULT	2

// return values for regmatch()
#define RA_FAIL		1	// something failed, abort
#define RA_CONT		2	// continue in inner loop
#define RA_BREAK	3	// break inner loop
#define RA_MATCH	4	// successful match
#define RA_NOMATCH	5	// didn't match

/*
 * Return NOT_MULTI if c is not a "multi" operator.
 * Return MULTI_ONE if c is a single "multi" operator.
 * Return MULTI_MULT if c is a multi "multi" operator.
 */
    static int
re_multi_type(int c)
{
    if (c == Magic('@') || c == Magic('=') || c == Magic('?'))
	return MULTI_ONE;
    if (c == Magic('*') || c == Magic('+') || c == Magic('{'))
	return MULTI_MULT;
    return NOT_MULTI;
}

static char_u		*reg_prev_sub = NULL;

/*
 * REGEXP_INRANGE contains all characters which are always special in a []
 * range after '\'.
 * REGEXP_ABBR contains all characters which act as abbreviations after '\'.
 * These are:
 *  \n	- New line (NL).
 *  \r	- Carriage Return (CR).
 *  \t	- Tab (TAB).
 *  \e	- Escape (ESC).
 *  \b	- Backspace (Ctrl_H).
 *  \d  - Character code in decimal, eg \d123
 *  \o	- Character code in octal, eg \o80
 *  \x	- Character code in hex, eg \x4a
 *  \u	- Multibyte character code, eg \u20ac
 *  \U	- Long multibyte character code, eg \U12345678
 */
static char_u REGEXP_INRANGE[] = "]^-n\\";
static char_u REGEXP_ABBR[] = "nrtebdoxuU";

/*
 * Translate '\x' to its control character, except "\n", which is Magic.
 */
    static int
backslash_trans(int c)
{
    switch (c)
    {
	case 'r':   return CAR;
	case 't':   return TAB;
	case 'e':   return ESC;
	case 'b':   return BS;
    }
    return c;
}

/*
 * Check for a character class name "[:name:]".  "pp" points to the '['.
 * Returns one of the CLASS_ items. CLASS_NONE means that no item was
 * recognized.  Otherwise "pp" is advanced to after the item.
 */
    static int
get_char_class(char_u **pp)
{
    static const char *(class_names[]) =
    {
	"alnum:]",
#define CLASS_ALNUM 0
	"alpha:]",
#define CLASS_ALPHA 1
	"blank:]",
#define CLASS_BLANK 2
	"cntrl:]",
#define CLASS_CNTRL 3
	"digit:]",
#define CLASS_DIGIT 4
	"graph:]",
#define CLASS_GRAPH 5
	"lower:]",
#define CLASS_LOWER 6
	"print:]",
#define CLASS_PRINT 7
	"punct:]",
#define CLASS_PUNCT 8
	"space:]",
#define CLASS_SPACE 9
	"upper:]",
#define CLASS_UPPER 10
	"xdigit:]",
#define CLASS_XDIGIT 11
	"tab:]",
#define CLASS_TAB 12
	"return:]",
#define CLASS_RETURN 13
	"backspace:]",
#define CLASS_BACKSPACE 14
	"escape:]",
#define CLASS_ESCAPE 15
	"ident:]",
#define CLASS_IDENT 16
	"keyword:]",
#define CLASS_KEYWORD 17
	"fname:]",
#define CLASS_FNAME 18
    };
#define CLASS_NONE 99
    int i;

    if ((*pp)[1] == ':')
    {
	for (i = 0; i < (int)ARRAY_LENGTH(class_names); ++i)
	    if (STRNCMP(*pp + 2, class_names[i], STRLEN(class_names[i])) == 0)
	    {
		*pp += STRLEN(class_names[i]) + 2;
		return i;
	    }
    }
    return CLASS_NONE;
}

/*
 * Specific version of character class functions.
 * Using a table to keep this fast.
 */
static short	class_tab[256];

#define	    RI_DIGIT	0x01
#define	    RI_HEX	0x02
#define	    RI_OCTAL	0x04
#define	    RI_WORD	0x08
#define	    RI_HEAD	0x10
#define	    RI_ALPHA	0x20
#define	    RI_LOWER	0x40
#define	    RI_UPPER	0x80
#define	    RI_WHITE	0x100

    static void
init_class_tab(void)
{
    int		i;
    static int	done = FALSE;

    if (done)
	return;

    for (i = 0; i < 256; ++i)
    {
	if (i >= '0' && i <= '7')
	    class_tab[i] = RI_DIGIT + RI_HEX + RI_OCTAL + RI_WORD;
	else if (i >= '8' && i <= '9')
	    class_tab[i] = RI_DIGIT + RI_HEX + RI_WORD;
	else if (i >= 'a' && i <= 'f')
	    class_tab[i] = RI_HEX + RI_WORD + RI_HEAD + RI_ALPHA + RI_LOWER;
	else if (i >= 'g' && i <= 'z')
	    class_tab[i] = RI_WORD + RI_HEAD + RI_ALPHA + RI_LOWER;
	else if (i >= 'A' && i <= 'F')
	    class_tab[i] = RI_HEX + RI_WORD + RI_HEAD + RI_ALPHA + RI_UPPER;
	else if (i >= 'G' && i <= 'Z')
	    class_tab[i] = RI_WORD + RI_HEAD + RI_ALPHA + RI_UPPER;
	else if (i == '_')
	    class_tab[i] = RI_WORD + RI_HEAD;
	else
	    class_tab[i] = 0;
    }
    class_tab[' '] |= RI_WHITE;
    class_tab['\t'] |= RI_WHITE;
    done = TRUE;
}

#define ri_digit(c)	((c) < 0x100 && (class_tab[c] & RI_DIGIT))
#define ri_hex(c)	((c) < 0x100 && (class_tab[c] & RI_HEX))
#define ri_octal(c)	((c) < 0x100 && (class_tab[c] & RI_OCTAL))
#define ri_word(c)	((c) < 0x100 && (class_tab[c] & RI_WORD))
#define ri_head(c)	((c) < 0x100 && (class_tab[c] & RI_HEAD))
#define ri_alpha(c)	((c) < 0x100 && (class_tab[c] & RI_ALPHA))
#define ri_lower(c)	((c) < 0x100 && (class_tab[c] & RI_LOWER))
#define ri_upper(c)	((c) < 0x100 && (class_tab[c] & RI_UPPER))
#define ri_white(c)	((c) < 0x100 && (class_tab[c] & RI_WHITE))

// flags for regflags
#define RF_ICASE    1	// ignore case
#define RF_NOICASE  2	// don't ignore case
#define RF_HASNL    4	// can match a NL
#define RF_ICOMBINE 8	// ignore combining characters
#define RF_LOOKBH   16	// uses "\@<=" or "\@<!"

/*
 * Global work variables for vim_regcomp().
 */

static char_u	*regparse;	// Input-scan pointer.
static int	regnpar;	// () count.
static int	wants_nfa;	// regex should use NFA engine
#ifdef FEAT_SYN_HL
static int	regnzpar;	// \z() count.
static int	re_has_z;	// \z item detected
#endif
static unsigned	regflags;	// RF_ flags for prog
#if defined(FEAT_SYN_HL) || defined(PROTO)
static int	had_eol;	// TRUE when EOL found by vim_regcomp()
#endif

static magic_T	reg_magic;	// magicness of the pattern

static int	reg_string;	// matching with a string instead of a buffer
				// line
static int	reg_strict;	// "[abc" is illegal

/*
 * META contains all characters that may be magic, except '^' and '$'.
 */

// META[] is used often enough to justify turning it into a table.
static char_u META_flags[] = {
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//		   %  &     (  )  *  +	      .
    0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0,
//     1  2  3	4  5  6  7  8  9	<  =  >  ?
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1,
//  @  A     C	D     F     H  I     K	L  M	 O
    1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1,
//  P	     S	   U  V  W  X	  Z  [		 _
    1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1,
//     a     c	d     f     h  i     k	l  m  n  o
    0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1,
//  p	     s	   u  v  w  x	  z  {	|     ~
    1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1
};

static int	curchr;		// currently parsed character
// Previous character.  Note: prevchr is sometimes -1 when we are not at the
// start, eg in /[ ^I]^ the pattern was never found even if it existed,
// because ^ was taken to be magic -- webb
static int	prevchr;
static int	prevprevchr;	// previous-previous character
static int	nextchr;	// used for ungetchr()

// arguments for reg()
#define REG_NOPAREN	0	// toplevel reg()
#define REG_PAREN	1	// \(\)
#define REG_ZPAREN	2	// \z(\)
#define REG_NPAREN	3	// \%(\)

typedef struct
{
     char_u	*regparse;
     int	prevchr_len;
     int	curchr;
     int	prevchr;
     int	prevprevchr;
     int	nextchr;
     int	at_start;
     int	prev_at_start;
     int	regnpar;
} parse_state_T;

static void	initchr(char_u *);
static int	getchr(void);
static void	skipchr_keepstart(void);
static int	peekchr(void);
static void	skipchr(void);
static void	ungetchr(void);
static long	gethexchrs(int maxinputlen);
static long	getoctchrs(void);
static long	getdecchrs(void);
static int	coll_get_char(void);
static int	prog_magic_wrong(void);
static int	cstrncmp(char_u *s1, char_u *s2, int *n);
static char_u	*cstrchr(char_u *, int);
static int	re_mult_next(char *what);
static int	reg_iswordc(int);
#ifdef FEAT_EVAL
static void report_re_switch(char_u *pat);
#endif

static regengine_T bt_regengine;
static regengine_T nfa_regengine;

/*
 * Return TRUE if compiled regular expression "prog" can match a line break.
 */
    int
re_multiline(regprog_T *prog)
{
    return (prog->regflags & RF_HASNL);
}

/*
 * Check for an equivalence class name "[=a=]".  "pp" points to the '['.
 * Returns a character representing the class. Zero means that no item was
 * recognized.  Otherwise "pp" is advanced to after the item.
 */
    static int
get_equi_class(char_u **pp)
{
    int		c;
    int		l = 1;
    char_u	*p = *pp;

    if (p[1] == '=' && p[2] != NUL)
    {
	if (has_mbyte)
	    l = (*mb_ptr2len)(p + 2);
	if (p[l + 2] == '=' && p[l + 3] == ']')
	{
	    if (has_mbyte)
		c = mb_ptr2char(p + 2);
	    else
		c = p[2];
	    *pp += l + 4;
	    return c;
	}
    }
    return 0;
}

/*
 * Check for a collating element "[.a.]".  "pp" points to the '['.
 * Returns a character. Zero means that no item was recognized.  Otherwise
 * "pp" is advanced to after the item.
 * Currently only single characters are recognized!
 */
    static int
get_coll_element(char_u **pp)
{
    int		c;
    int		l = 1;
    char_u	*p = *pp;

    if (p[0] != NUL && p[1] == '.' && p[2] != NUL)
    {
	if (has_mbyte)
	    l = (*mb_ptr2len)(p + 2);
	if (p[l + 2] == '.' && p[l + 3] == ']')
	{
	    if (has_mbyte)
		c = mb_ptr2char(p + 2);
	    else
		c = p[2];
	    *pp += l + 4;
	    return c;
	}
    }
    return 0;
}

static int reg_cpo_lit; // 'cpoptions' contains 'l' flag
static int reg_cpo_bsl; // 'cpoptions' contains '\' flag

    static void
get_cpo_flags(void)
{
    reg_cpo_lit = vim_strchr(p_cpo, CPO_LITERAL) != NULL;
    reg_cpo_bsl = vim_strchr(p_cpo, CPO_BACKSL) != NULL;
}

/*
 * Skip over a "[]" range.
 * "p" must point to the character after the '['.
 * The returned pointer is on the matching ']', or the terminating NUL.
 */
    static char_u *
skip_anyof(char_u *p)
{
    int		l;

    if (*p == '^')	// Complement of range.
	++p;
    if (*p == ']' || *p == '-')
	++p;
    while (*p != NUL && *p != ']')
    {
	if (has_mbyte && (l = (*mb_ptr2len)(p)) > 1)
	    p += l;
	else
	    if (*p == '-')
	    {
		++p;
		if (*p != ']' && *p != NUL)
		    MB_PTR_ADV(p);
	    }
	else if (*p == '\\'
		&& !reg_cpo_bsl
		&& (vim_strchr(REGEXP_INRANGE, p[1]) != NULL
		    || (!reg_cpo_lit && vim_strchr(REGEXP_ABBR, p[1]) != NULL)))
	    p += 2;
	else if (*p == '[')
	{
	    if (get_char_class(&p) == CLASS_NONE
		    && get_equi_class(&p) == 0
		    && get_coll_element(&p) == 0
		    && *p != NUL)
		++p; // it is not a class name and not NUL
	}
	else
	    ++p;
    }

    return p;
}

/*
 * Skip past regular expression.
 * Stop at end of "startp" or where "delim" is found ('/', '?', etc).
 * Take care of characters with a backslash in front of it.
 * Skip strings inside [ and ].
 */
    char_u *
skip_regexp(
    char_u	*startp,
    int		delim,
    int		magic)
{
    return skip_regexp_ex(startp, delim, magic, NULL, NULL, NULL);
}

/*
 * Call skip_regexp() and when the delimiter does not match give an error and
 * return NULL.
 */
    char_u *
skip_regexp_err(
    char_u	*startp,
    int		delim,
    int		magic)
{
    char_u *p = skip_regexp(startp, delim, magic);

    if (*p != delim)
    {
	semsg(_(e_missing_delimiter_after_search_pattern_str), startp);
	return NULL;
    }
    return p;
}

/*
 * skip_regexp() with extra arguments:
 * When "newp" is not NULL and "dirc" is '?', make an allocated copy of the
 * expression and change "\?" to "?".  If "*newp" is not NULL the expression
 * is changed in-place.
 * If a "\?" is changed to "?" then "dropped" is incremented, unless NULL.
 * If "magic_val" is not NULL, returns the effective magicness of the pattern
 */
    char_u *
skip_regexp_ex(
    char_u	*startp,
    int		dirc,
    int		magic,
    char_u	**newp,
    int		*dropped,
    magic_T	*magic_val)
{
    magic_T	mymagic;
    char_u	*p = startp;

    if (magic)
	mymagic = MAGIC_ON;
    else
	mymagic = MAGIC_OFF;
    get_cpo_flags();

    for (; p[0] != NUL; MB_PTR_ADV(p))
    {
	if (p[0] == dirc)	// found end of regexp
	    break;
	if ((p[0] == '[' && mymagic >= MAGIC_ON)
		|| (p[0] == '\\' && p[1] == '[' && mymagic <= MAGIC_OFF))
	{
	    p = skip_anyof(p + 1);
	    if (p[0] == NUL)
		break;
	}
	else if (p[0] == '\\' && p[1] != NUL)
	{
	    if (dirc == '?' && newp != NULL && p[1] == '?')
	    {
		// change "\?" to "?", make a copy first.
		if (*newp == NULL)
		{
		    *newp = vim_strsave(startp);
		    if (*newp != NULL)
			p = *newp + (p - startp);
		}
		if (dropped != NULL)
		    ++*dropped;
		if (*newp != NULL)
		    STRMOVE(p, p + 1);
		else
		    ++p;
	    }
	    else
		++p;    // skip next character
	    if (*p == 'v')
		mymagic = MAGIC_ALL;
	    else if (*p == 'V')
		mymagic = MAGIC_NONE;
	}
    }
    if (magic_val != NULL)
	*magic_val = mymagic;
    return p;
}

/*
 * Functions for getting characters from the regexp input.
 */
static int	prevchr_len;	// byte length of previous char
static int	at_start;	// True when on the first character
static int	prev_at_start;  // True when on the second character

/*
 * Start parsing at "str".
 */
    static void
initchr(char_u *str)
{
    regparse = str;
    prevchr_len = 0;
    curchr = prevprevchr = prevchr = nextchr = -1;
    at_start = TRUE;
    prev_at_start = FALSE;
}

/*
 * Save the current parse state, so that it can be restored and parsing
 * starts in the same state again.
 */
    static void
save_parse_state(parse_state_T *ps)
{
    ps->regparse = regparse;
    ps->prevchr_len = prevchr_len;
    ps->curchr = curchr;
    ps->prevchr = prevchr;
    ps->prevprevchr = prevprevchr;
    ps->nextchr = nextchr;
    ps->at_start = at_start;
    ps->prev_at_start = prev_at_start;
    ps->regnpar = regnpar;
}

/*
 * Restore a previously saved parse state.
 */
    static void
restore_parse_state(parse_state_T *ps)
{
    regparse = ps->regparse;
    prevchr_len = ps->prevchr_len;
    curchr = ps->curchr;
    prevchr = ps->prevchr;
    prevprevchr = ps->prevprevchr;
    nextchr = ps->nextchr;
    at_start = ps->at_start;
    prev_at_start = ps->prev_at_start;
    regnpar = ps->regnpar;
}


/*
 * Get the next character without advancing.
 */
    static int
peekchr(void)
{
    static int	after_slash = FALSE;

    if (curchr != -1)
	return curchr;

    switch (curchr = regparse[0])
    {
	case '.':
	case '[':
	case '~':
	    // magic when 'magic' is on
	    if (reg_magic >= MAGIC_ON)
		curchr = Magic(curchr);
	    break;
	case '(':
	case ')':
	case '{':
	case '%':
	case '+':
	case '=':
	case '?':
	case '@':
	case '!':
	case '&':
	case '|':
	case '<':
	case '>':
	case '#':	// future ext.
	case '"':	// future ext.
	case '\'':	// future ext.
	case ',':	// future ext.
	case '-':	// future ext.
	case ':':	// future ext.
	case ';':	// future ext.
	case '`':	// future ext.
	case '/':	// Can't be used in / command
			// magic only after "\v"
	    if (reg_magic == MAGIC_ALL)
		curchr = Magic(curchr);
	    break;
	case '*':
	    // * is not magic as the very first character, eg "?*ptr", when
	    // after '^', eg "/^*ptr" and when after "\(", "\|", "\&".  But
	    // "\(\*" is not magic, thus must be magic if "after_slash"
	    if (reg_magic >= MAGIC_ON
		    && !at_start
		    && !(prev_at_start && prevchr == Magic('^'))
		    && (after_slash
			|| (prevchr != Magic('(')
			    && prevchr != Magic('&')
			    && prevchr != Magic('|'))))
		curchr = Magic('*');
	    break;
	case '^':
	    // '^' is only magic as the very first character and if it's after
	    // "\(", "\|", "\&' or "\n"
	    if (reg_magic >= MAGIC_OFF
		    && (at_start
			|| reg_magic == MAGIC_ALL
			|| prevchr == Magic('(')
			|| prevchr == Magic('|')
			|| prevchr == Magic('&')
			|| prevchr == Magic('n')
			|| (no_Magic(prevchr) == '('
			    && prevprevchr == Magic('%'))))
	    {
		curchr = Magic('^');
		at_start = TRUE;
		prev_at_start = FALSE;
	    }
	    break;
	case '$':
	    // '$' is only magic as the very last char and if it's in front of
	    // either "\|", "\)", "\&", or "\n"
	    if (reg_magic >= MAGIC_OFF)
	    {
		char_u *p = regparse + 1;
		int is_magic_all = (reg_magic == MAGIC_ALL);

		// ignore \c \C \m \M \v \V and \Z after '$'
		while (p[0] == '\\' && (p[1] == 'c' || p[1] == 'C'
			    || p[1] == 'm' || p[1] == 'M'
			    || p[1] == 'v' || p[1] == 'V' || p[1] == 'Z'))
		{
		    if (p[1] == 'v')
			is_magic_all = TRUE;
		    else if (p[1] == 'm' || p[1] == 'M' || p[1] == 'V')
			is_magic_all = FALSE;
		    p += 2;
		}
		if (p[0] == NUL
			|| (p[0] == '\\'
			    && (p[1] == '|' || p[1] == '&' || p[1] == ')'
				|| p[1] == 'n'))
			|| (is_magic_all
			    && (p[0] == '|' || p[0] == '&' || p[0] == ')'))
			|| reg_magic == MAGIC_ALL)
		    curchr = Magic('$');
	    }
	    break;
	case '\\':
	    {
		int c = regparse[1];

		if (c == NUL)
		    curchr = '\\';	// trailing '\'
		else if (c <= '~' && META_flags[c])
		{
		    /*
		     * META contains everything that may be magic sometimes,
		     * except ^ and $ ("\^" and "\$" are only magic after
		     * "\V").  We now fetch the next character and toggle its
		     * magicness.  Therefore, \ is so meta-magic that it is
		     * not in META.
		     */
		    curchr = -1;
		    prev_at_start = at_start;
		    at_start = FALSE;	// be able to say "/\*ptr"
		    ++regparse;
		    ++after_slash;
		    peekchr();
		    --regparse;
		    --after_slash;
		    curchr = toggle_Magic(curchr);
		}
		else if (vim_strchr(REGEXP_ABBR, c))
		{
		    /*
		     * Handle abbreviations, like "\t" for TAB -- webb
		     */
		    curchr = backslash_trans(c);
		}
		else if (reg_magic == MAGIC_NONE && (c == '$' || c == '^'))
		    curchr = toggle_Magic(c);
		else
		{
		    /*
		     * Next character can never be (made) magic?
		     * Then backslashing it won't do anything.
		     */
		    if (has_mbyte)
			curchr = (*mb_ptr2char)(regparse + 1);
		    else
			curchr = c;
		}
		break;
	    }

	default:
	    if (has_mbyte)
		curchr = (*mb_ptr2char)(regparse);
    }

    return curchr;
}

/*
 * Eat one lexed character.  Do this in a way that we can undo it.
 */
    static void
skipchr(void)
{
    // peekchr() eats a backslash, do the same here
    if (*regparse == '\\')
	prevchr_len = 1;
    else
	prevchr_len = 0;
    if (regparse[prevchr_len] != NUL)
    {
	if (enc_utf8)
	    // exclude composing chars that mb_ptr2len does include
	    prevchr_len += utf_ptr2len(regparse + prevchr_len);
	else if (has_mbyte)
	    prevchr_len += (*mb_ptr2len)(regparse + prevchr_len);
	else
	    ++prevchr_len;
    }
    regparse += prevchr_len;
    prev_at_start = at_start;
    at_start = FALSE;
    prevprevchr = prevchr;
    prevchr = curchr;
    curchr = nextchr;	    // use previously unget char, or -1
    nextchr = -1;
}

/*
 * Skip a character while keeping the value of prev_at_start for at_start.
 * prevchr and prevprevchr are also kept.
 */
    static void
skipchr_keepstart(void)
{
    int as = prev_at_start;
    int pr = prevchr;
    int prpr = prevprevchr;

    skipchr();
    at_start = as;
    prevchr = pr;
    prevprevchr = prpr;
}

/*
 * Get the next character from the pattern. We know about magic and such, so
 * therefore we need a lexical analyzer.
 */
    static int
getchr(void)
{
    int chr = peekchr();

    skipchr();
    return chr;
}

/*
 * put character back.  Works only once!
 */
    static void
ungetchr(void)
{
    nextchr = curchr;
    curchr = prevchr;
    prevchr = prevprevchr;
    at_start = prev_at_start;
    prev_at_start = FALSE;

    // Backup regparse, so that it's at the same position as before the
    // getchr().
    regparse -= prevchr_len;
}

/*
 * Get and return the value of the hex string at the current position.
 * Return -1 if there is no valid hex number.
 * The position is updated:
 *     blahblah\%x20asdf
 *	   before-^ ^-after
 * The parameter controls the maximum number of input characters. This will be
 * 2 when reading a \%x20 sequence and 4 when reading a \%u20AC sequence.
 */
    static long
gethexchrs(int maxinputlen)
{
    long_u	nr = 0;
    int		c;
    int		i;

    for (i = 0; i < maxinputlen; ++i)
    {
	c = regparse[0];
	if (!vim_isxdigit(c))
	    break;
	nr <<= 4;
	nr |= hex2nr(c);
	++regparse;
    }

    if (i == 0)
	return -1;
    return (long)nr;
}

/*
 * Get and return the value of the decimal string immediately after the
 * current position. Return -1 for invalid.  Consumes all digits.
 */
    static long
getdecchrs(void)
{
    long_u	nr = 0;
    int		c;
    int		i;

    for (i = 0; ; ++i)
    {
	c = regparse[0];
	if (c < '0' || c > '9')
	    break;
	nr *= 10;
	nr += c - '0';
	++regparse;
	curchr = -1; // no longer valid
    }

    if (i == 0)
	return -1;
    return (long)nr;
}

/*
 * get and return the value of the octal string immediately after the current
 * position. Return -1 for invalid, or 0-255 for valid. Smart enough to handle
 * numbers > 377 correctly (for example, 400 is treated as 40) and doesn't
 * treat 8 or 9 as recognised characters. Position is updated:
 *     blahblah\%o210asdf
 *	   before-^  ^-after
 */
    static long
getoctchrs(void)
{
    long_u	nr = 0;
    int		c;
    int		i;

    for (i = 0; i < 3 && nr < 040; ++i)
    {
	c = regparse[0];
	if (c < '0' || c > '7')
	    break;
	nr <<= 3;
	nr |= hex2nr(c);
	++regparse;
    }

    if (i == 0)
	return -1;
    return (long)nr;
}

/*
 * read_limits - Read two integers to be taken as a minimum and maximum.
 * If the first character is '-', then the range is reversed.
 * Should end with 'end'.  If minval is missing, zero is default, if maxval is
 * missing, a very big number is the default.
 */
    static int
read_limits(long *minval, long *maxval)
{
    int		reverse = FALSE;
    char_u	*first_char;
    long	tmp;

    if (*regparse == '-')
    {
	// Starts with '-', so reverse the range later
	regparse++;
	reverse = TRUE;
    }
    first_char = regparse;
    *minval = getdigits(&regparse);
    if (*regparse == ',')	    // There is a comma
    {
	if (vim_isdigit(*++regparse))
	    *maxval = getdigits(&regparse);
	else
	    *maxval = MAX_LIMIT;
    }
    else if (VIM_ISDIGIT(*first_char))
	*maxval = *minval;	    // It was \{n} or \{-n}
    else
	*maxval = MAX_LIMIT;	    // It was \{} or \{-}
    if (*regparse == '\\')
	regparse++;	// Allow either \{...} or \{...\}
    if (*regparse != '}')
	EMSG2_RET_FAIL(_(e_syntax_error_in_str_curlies),
						       reg_magic == MAGIC_ALL);

    /*
     * Reverse the range if there was a '-', or make sure it is in the right
     * order otherwise.
     */
    if ((!reverse && *minval > *maxval) || (reverse && *minval < *maxval))
    {
	tmp = *minval;
	*minval = *maxval;
	*maxval = tmp;
    }
    skipchr();		// let's be friends with the lexer again
    return OK;
}

/*
 * vim_regexec and friends
 */

/*
 * Global work variables for vim_regexec().
 */

static void	cleanup_subexpr(void);
#ifdef FEAT_SYN_HL
static void	cleanup_zsubexpr(void);
#endif
static int	match_with_backref(linenr_T start_lnum, colnr_T start_col, linenr_T end_lnum, colnr_T end_col, int *bytelen);

/*
 * Sometimes need to save a copy of a line.  Since alloc()/free() is very
 * slow, we keep one allocated piece of memory and only re-allocate it when
 * it's too small.  It's freed in bt_regexec_both() when finished.
 */
static char_u	*reg_tofree = NULL;
static unsigned	reg_tofreelen;

/*
 * Structure used to store the execution state of the regex engine.
 * Which ones are set depends on whether a single-line or multi-line match is
 * done:
 *			single-line		multi-line
 * reg_match		&regmatch_T		NULL
 * reg_mmatch		NULL			&regmmatch_T
 * reg_startp		reg_match->startp	<invalid>
 * reg_endp		reg_match->endp		<invalid>
 * reg_startpos		<invalid>		reg_mmatch->startpos
 * reg_endpos		<invalid>		reg_mmatch->endpos
 * reg_win		NULL			window in which to search
 * reg_buf		curbuf			buffer in which to search
 * reg_firstlnum	<invalid>		first line in which to search
 * reg_maxline		0			last line nr
 * reg_line_lbr		FALSE or TRUE		FALSE
 */
typedef struct {
    regmatch_T		*reg_match;
    regmmatch_T		*reg_mmatch;

    char_u		**reg_startp;
    char_u		**reg_endp;
    lpos_T		*reg_startpos;
    lpos_T		*reg_endpos;

    win_T		*reg_win;
    buf_T		*reg_buf;
    linenr_T		reg_firstlnum;
    linenr_T		reg_maxline;
    int			reg_line_lbr;	// "\n" in string is line break

    // The current match-position is stord in these variables:
    linenr_T	lnum;		// line number, relative to first line
    char_u	*line;		// start of current line
    char_u	*input;		// current input, points into "line"

    int	need_clear_subexpr;	// subexpressions still need to be cleared
#ifdef FEAT_SYN_HL
    int	need_clear_zsubexpr;	// extmatch subexpressions still need to be
				// cleared
#endif

    // Internal copy of 'ignorecase'.  It is set at each call to vim_regexec().
    // Normally it gets the value of "rm_ic" or "rmm_ic", but when the pattern
    // contains '\c' or '\C' the value is overruled.
    int			reg_ic;

    // Similar to "reg_ic", but only for 'combining' characters.  Set with \Z
    // flag in the regexp.  Defaults to false, always.
    int			reg_icombine;

    // Copy of "rmm_maxcol": maximum column to search for a match.  Zero when
    // there is no maximum.
    colnr_T		reg_maxcol;

    // State for the NFA engine regexec.
    int nfa_has_zend;	    // NFA regexp \ze operator encountered.
    int nfa_has_backref;    // NFA regexp \1 .. \9 encountered.
    int nfa_nsubexpr;	    // Number of sub expressions actually being used
			    // during execution. 1 if only the whole match
			    // (subexpr 0) is used.
    // listid is global, so that it increases on recursive calls to
    // nfa_regmatch(), which means we don't have to clear the lastlist field of
    // all the states.
    int nfa_listid;
    int nfa_alt_listid;

#ifdef FEAT_SYN_HL
    int nfa_has_zsubexpr;   // NFA regexp has \z( ), set zsubexpr.
#endif
} regexec_T;

static regexec_T	rex;
static int		rex_in_use = FALSE;

/*
 * Return TRUE if character 'c' is included in 'iskeyword' option for
 * "reg_buf" buffer.
 */
    static int
reg_iswordc(int c)
{
    return vim_iswordc_buf(c, rex.reg_buf);
}

/*
 * Get pointer to the line "lnum", which is relative to "reg_firstlnum".
 */
    static char_u *
reg_getline(linenr_T lnum)
{
    // when looking behind for a match/no-match lnum is negative.  But we
    // can't go before line 1
    if (rex.reg_firstlnum + lnum < 1)
	return NULL;
    if (lnum > rex.reg_maxline)
	// Must have matched the "\n" in the last line.
	return (char_u *)"";
    return ml_get_buf(rex.reg_buf, rex.reg_firstlnum + lnum, FALSE);
}

#ifdef FEAT_SYN_HL
static char_u	*reg_startzp[NSUBEXP];	// Workspace to mark beginning
static char_u	*reg_endzp[NSUBEXP];	//   and end of \z(...\) matches
static lpos_T	reg_startzpos[NSUBEXP];	// idem, beginning pos
static lpos_T	reg_endzpos[NSUBEXP];	// idem, end pos
#endif

// TRUE if using multi-line regexp.
#define REG_MULTI	(rex.reg_match == NULL)

#ifdef FEAT_SYN_HL
/*
 * Create a new extmatch and mark it as referenced once.
 */
    static reg_extmatch_T *
make_extmatch(void)
{
    reg_extmatch_T	*em;

    em = ALLOC_CLEAR_ONE(reg_extmatch_T);
    if (em != NULL)
	em->refcnt = 1;
    return em;
}

/*
 * Add a reference to an extmatch.
 */
    reg_extmatch_T *
ref_extmatch(reg_extmatch_T *em)
{
    if (em != NULL)
	em->refcnt++;
    return em;
}

/*
 * Remove a reference to an extmatch.  If there are no references left, free
 * the info.
 */
    void
unref_extmatch(reg_extmatch_T *em)
{
    int i;

    if (em != NULL && --em->refcnt <= 0)
    {
	for (i = 0; i < NSUBEXP; ++i)
	    vim_free(em->matches[i]);
	vim_free(em);
    }
}
#endif

/*
 * Get class of previous character.
 */
    static int
reg_prev_class(void)
{
    if (rex.input > rex.line)
	return mb_get_class_buf(rex.input - 1
		       - (*mb_head_off)(rex.line, rex.input - 1), rex.reg_buf);
    return -1;
}

/*
 * Return TRUE if the current rex.input position matches the Visual area.
 */
    static int
reg_match_visual(void)
{
    pos_T	top, bot;
    linenr_T    lnum;
    colnr_T	col;
    win_T	*wp = rex.reg_win == NULL ? curwin : rex.reg_win;
    int		mode;
    colnr_T	start, end;
    colnr_T	start2, end2;
    colnr_T	cols;
    colnr_T	curswant;

    // Check if the buffer is the current buffer and not using a string.
    if (rex.reg_buf != curbuf || VIsual.lnum == 0 || !REG_MULTI)
	return FALSE;

    if (VIsual_active)
    {
	if (LT_POS(VIsual, wp->w_cursor))
	{
	    top = VIsual;
	    bot = wp->w_cursor;
	}
	else
	{
	    top = wp->w_cursor;
	    bot = VIsual;
	}
	mode = VIsual_mode;
	curswant = wp->w_curswant;
    }
    else
    {
	if (LT_POS(curbuf->b_visual.vi_start, curbuf->b_visual.vi_end))
	{
	    top = curbuf->b_visual.vi_start;
	    bot = curbuf->b_visual.vi_end;
	}
	else
	{
	    top = curbuf->b_visual.vi_end;
	    bot = curbuf->b_visual.vi_start;
	}
	// a substitue command may have
	// removed some lines
	if (bot.lnum > curbuf->b_ml.ml_line_count)
	    bot.lnum = curbuf->b_ml.ml_line_count;
	mode = curbuf->b_visual.vi_mode;
	curswant = curbuf->b_visual.vi_curswant;
    }
    lnum = rex.lnum + rex.reg_firstlnum;
    if (lnum < top.lnum || lnum > bot.lnum)
	return FALSE;

    col = (colnr_T)(rex.input - rex.line);
    if (mode == 'v')
    {
	if ((lnum == top.lnum && col < top.col)
		|| (lnum == bot.lnum && col >= bot.col + (*p_sel != 'e')))
	    return FALSE;
    }
    else if (mode == Ctrl_V)
    {
	getvvcol(wp, &top, &start, NULL, &end);
	getvvcol(wp, &bot, &start2, NULL, &end2);
	if (start2 < start)
	    start = start2;
	if (end2 > end)
	    end = end2;
	if (top.col == MAXCOL || bot.col == MAXCOL || curswant == MAXCOL)
	    end = MAXCOL;

	// getvvcol() flushes rex.line, need to get it again
	rex.line = reg_getline(rex.lnum);
	rex.input = rex.line + col;

	cols = win_linetabsize(wp, rex.reg_firstlnum + rex.lnum, rex.line, col);
	if (cols < start || cols > end - (*p_sel == 'e'))
	    return FALSE;
    }
    return TRUE;
}

/*
 * Check the regexp program for its magic number.
 * Return TRUE if it's wrong.
 */
    static int
prog_magic_wrong(void)
{
    regprog_T	*prog;

    prog = REG_MULTI ? rex.reg_mmatch->regprog : rex.reg_match->regprog;
    if (prog->engine == &nfa_regengine)
	// For NFA matcher we don't check the magic
	return FALSE;

    if (UCHARAT(((bt_regprog_T *)prog)->program) != REGMAGIC)
    {
	iemsg(e_corrupted_regexp_program);
	return TRUE;
    }
    return FALSE;
}

/*
 * Cleanup the subexpressions, if this wasn't done yet.
 * This construction is used to clear the subexpressions only when they are
 * used (to increase speed).
 */
    static void
cleanup_subexpr(void)
{
    if (!rex.need_clear_subexpr)
	return;

    if (REG_MULTI)
    {
	// Use 0xff to set lnum to -1
	vim_memset(rex.reg_startpos, 0xff, sizeof(lpos_T) * NSUBEXP);
	vim_memset(rex.reg_endpos, 0xff, sizeof(lpos_T) * NSUBEXP);
    }
    else
    {
	vim_memset(rex.reg_startp, 0, sizeof(char_u *) * NSUBEXP);
	vim_memset(rex.reg_endp, 0, sizeof(char_u *) * NSUBEXP);
    }
    rex.need_clear_subexpr = FALSE;
}

#ifdef FEAT_SYN_HL
    static void
cleanup_zsubexpr(void)
{
    if (!rex.need_clear_zsubexpr)
	return;

    if (REG_MULTI)
    {
	// Use 0xff to set lnum to -1
	vim_memset(reg_startzpos, 0xff, sizeof(lpos_T) * NSUBEXP);
	vim_memset(reg_endzpos, 0xff, sizeof(lpos_T) * NSUBEXP);
    }
    else
    {
	vim_memset(reg_startzp, 0, sizeof(char_u *) * NSUBEXP);
	vim_memset(reg_endzp, 0, sizeof(char_u *) * NSUBEXP);
    }
    rex.need_clear_zsubexpr = FALSE;
}
#endif

/*
 * Advance rex.lnum, rex.line and rex.input to the next line.
 */
    static void
reg_nextline(void)
{
    rex.line = reg_getline(++rex.lnum);
    rex.input = rex.line;
    fast_breakcheck();
}

/*
 * Check whether a backreference matches.
 * Returns RA_FAIL, RA_NOMATCH or RA_MATCH.
 * If "bytelen" is not NULL, it is set to the byte length of the match in the
 * last line.
 */
    static int
match_with_backref(
    linenr_T start_lnum,
    colnr_T  start_col,
    linenr_T end_lnum,
    colnr_T  end_col,
    int	     *bytelen)
{
    linenr_T	clnum = start_lnum;
    colnr_T	ccol = start_col;
    int		len;
    char_u	*p;

    if (bytelen != NULL)
	*bytelen = 0;
    for (;;)
    {
	// Since getting one line may invalidate the other, need to make copy.
	// Slow!
	if (rex.line != reg_tofree)
	{
	    len = (int)STRLEN(rex.line);
	    if (reg_tofree == NULL || len >= (int)reg_tofreelen)
	    {
		len += 50;	// get some extra
		vim_free(reg_tofree);
		reg_tofree = alloc(len);
		if (reg_tofree == NULL)
		    return RA_FAIL; // out of memory!
		reg_tofreelen = len;
	    }
	    STRCPY(reg_tofree, rex.line);
	    rex.input = reg_tofree + (rex.input - rex.line);
	    rex.line = reg_tofree;
	}

	// Get the line to compare with.
	p = reg_getline(clnum);
	if (clnum == end_lnum)
	    len = end_col - ccol;
	else
	    len = (int)STRLEN(p + ccol);

	if (cstrncmp(p + ccol, rex.input, &len) != 0)
	    return RA_NOMATCH;  // doesn't match
	if (bytelen != NULL)
	    *bytelen += len;
	if (clnum == end_lnum)
	    break;		// match and at end!
	if (rex.lnum >= rex.reg_maxline)
	    return RA_NOMATCH;  // text too short

	// Advance to next line.
	reg_nextline();
	if (bytelen != NULL)
	    *bytelen = 0;
	++clnum;
	ccol = 0;
	if (got_int)
	    return RA_FAIL;
    }

    // found a match!  Note that rex.line may now point to a copy of the line,
    // that should not matter.
    return RA_MATCH;
}

/*
 * Used in a place where no * or \+ can follow.
 */
    static int
re_mult_next(char *what)
{
    if (re_multi_type(peekchr()) == MULTI_MULT)
    {
       semsg(_(e_nfa_regexp_cannot_repeat_str), what);
       rc_did_emsg = TRUE;
       return FAIL;
    }
    return OK;
}

typedef struct
{
    int a, b, c;
} decomp_T;


// 0xfb20 - 0xfb4f
static decomp_T decomp_table[0xfb4f-0xfb20+1] =
{
    {0x5e2,0,0},		// 0xfb20	alt ayin
    {0x5d0,0,0},		// 0xfb21	alt alef
    {0x5d3,0,0},		// 0xfb22	alt dalet
    {0x5d4,0,0},		// 0xfb23	alt he
    {0x5db,0,0},		// 0xfb24	alt kaf
    {0x5dc,0,0},		// 0xfb25	alt lamed
    {0x5dd,0,0},		// 0xfb26	alt mem-sofit
    {0x5e8,0,0},		// 0xfb27	alt resh
    {0x5ea,0,0},		// 0xfb28	alt tav
    {'+', 0, 0},		// 0xfb29	alt plus
    {0x5e9, 0x5c1, 0},		// 0xfb2a	shin+shin-dot
    {0x5e9, 0x5c2, 0},		// 0xfb2b	shin+sin-dot
    {0x5e9, 0x5c1, 0x5bc},	// 0xfb2c	shin+shin-dot+dagesh
    {0x5e9, 0x5c2, 0x5bc},	// 0xfb2d	shin+sin-dot+dagesh
    {0x5d0, 0x5b7, 0},		// 0xfb2e	alef+patah
    {0x5d0, 0x5b8, 0},		// 0xfb2f	alef+qamats
    {0x5d0, 0x5b4, 0},		// 0xfb30	alef+hiriq
    {0x5d1, 0x5bc, 0},		// 0xfb31	bet+dagesh
    {0x5d2, 0x5bc, 0},		// 0xfb32	gimel+dagesh
    {0x5d3, 0x5bc, 0},		// 0xfb33	dalet+dagesh
    {0x5d4, 0x5bc, 0},		// 0xfb34	he+dagesh
    {0x5d5, 0x5bc, 0},		// 0xfb35	vav+dagesh
    {0x5d6, 0x5bc, 0},		// 0xfb36	zayin+dagesh
    {0xfb37, 0, 0},		// 0xfb37 -- UNUSED
    {0x5d8, 0x5bc, 0},		// 0xfb38	tet+dagesh
    {0x5d9, 0x5bc, 0},		// 0xfb39	yud+dagesh
    {0x5da, 0x5bc, 0},		// 0xfb3a	kaf sofit+dagesh
    {0x5db, 0x5bc, 0},		// 0xfb3b	kaf+dagesh
    {0x5dc, 0x5bc, 0},		// 0xfb3c	lamed+dagesh
    {0xfb3d, 0, 0},		// 0xfb3d -- UNUSED
    {0x5de, 0x5bc, 0},		// 0xfb3e	mem+dagesh
    {0xfb3f, 0, 0},		// 0xfb3f -- UNUSED
    {0x5e0, 0x5bc, 0},		// 0xfb40	nun+dagesh
    {0x5e1, 0x5bc, 0},		// 0xfb41	samech+dagesh
    {0xfb42, 0, 0},		// 0xfb42 -- UNUSED
    {0x5e3, 0x5bc, 0},		// 0xfb43	pe sofit+dagesh
    {0x5e4, 0x5bc,0},		// 0xfb44	pe+dagesh
    {0xfb45, 0, 0},		// 0xfb45 -- UNUSED
    {0x5e6, 0x5bc, 0},		// 0xfb46	tsadi+dagesh
    {0x5e7, 0x5bc, 0},		// 0xfb47	qof+dagesh
    {0x5e8, 0x5bc, 0},		// 0xfb48	resh+dagesh
    {0x5e9, 0x5bc, 0},		// 0xfb49	shin+dagesh
    {0x5ea, 0x5bc, 0},		// 0xfb4a	tav+dagesh
    {0x5d5, 0x5b9, 0},		// 0xfb4b	vav+holam
    {0x5d1, 0x5bf, 0},		// 0xfb4c	bet+rafe
    {0x5db, 0x5bf, 0},		// 0xfb4d	kaf+rafe
    {0x5e4, 0x5bf, 0},		// 0xfb4e	pe+rafe
    {0x5d0, 0x5dc, 0}		// 0xfb4f	alef-lamed
};

    static void
mb_decompose(int c, int *c1, int *c2, int *c3)
{
    decomp_T d;

    if (c >= 0xfb20 && c <= 0xfb4f)
    {
	d = decomp_table[c - 0xfb20];
	*c1 = d.a;
	*c2 = d.b;
	*c3 = d.c;
    }
    else
    {
	*c1 = c;
	*c2 = *c3 = 0;
    }
}

/*
 * Compare two strings, ignore case if rex.reg_ic set.
 * Return 0 if strings match, non-zero otherwise.
 * Correct the length "*n" when composing characters are ignored.
 */
    static int
cstrncmp(char_u *s1, char_u *s2, int *n)
{
    int		result;

    if (!rex.reg_ic)
	result = STRNCMP(s1, s2, *n);
    else
	result = MB_STRNICMP(s1, s2, *n);

    // if it failed and it's utf8 and we want to combineignore:
    if (result != 0 && enc_utf8 && rex.reg_icombine)
    {
	char_u	*str1, *str2;
	int	c1, c2, c11, c12;
	int	junk;

	// we have to handle the strcmp ourselves, since it is necessary to
	// deal with the composing characters by ignoring them:
	str1 = s1;
	str2 = s2;
	c1 = c2 = 0;
	while ((int)(str1 - s1) < *n)
	{
	    c1 = mb_ptr2char_adv(&str1);
	    c2 = mb_ptr2char_adv(&str2);

	    // Decompose the character if necessary, into 'base' characters.
	    // Currently hard-coded for Hebrew, Arabic to be done...
	    if (c1 != c2 && (!rex.reg_ic || utf_fold(c1) != utf_fold(c2)))
	    {
		// decomposition necessary?
		mb_decompose(c1, &c11, &junk, &junk);
		mb_decompose(c2, &c12, &junk, &junk);
		c1 = c11;
		c2 = c12;
		if (c11 != c12
			    && (!rex.reg_ic || utf_fold(c11) != utf_fold(c12)))
		    break;
	    }
	}
	result = c2 - c1;
	if (result == 0)
	    *n = (int)(str2 - s2);
    }

    return result;
}

/*
 * cstrchr: This function is used a lot for simple searches, keep it fast!
 */
    static char_u *
cstrchr(char_u *s, int c)
{
    char_u	*p;
    int		cc;

    if (!rex.reg_ic || (!enc_utf8 && mb_char2len(c) > 1))
	return vim_strchr(s, c);

    // tolower() and toupper() can be slow, comparing twice should be a lot
    // faster (esp. when using MS Visual C++!).
    // For UTF-8 need to use folded case.
    if (enc_utf8 && c > 0x80)
	cc = utf_fold(c);
    else
	 if (MB_ISUPPER(c))
	cc = MB_TOLOWER(c);
    else if (MB_ISLOWER(c))
	cc = MB_TOUPPER(c);
    else
	return vim_strchr(s, c);

    if (has_mbyte)
    {
	for (p = s; *p != NUL; p += (*mb_ptr2len)(p))
	{
	    if (enc_utf8 && c > 0x80)
	    {
		int uc = utf_ptr2char(p);

		// Do not match an illegal byte.  E.g. 0xff matches 0xc3 0xbf,
		// not 0xff.
		if ((uc < 0x80 || uc != *p) && utf_fold(uc) == cc)
		    return p;
	    }
	    else if (*p == c || *p == cc)
		return p;
	}
    }
    else
	// Faster version for when there are no multi-byte characters.
	for (p = s; *p != NUL; ++p)
	    if (*p == c || *p == cc)
		return p;

    return NULL;
}

////////////////////////////////////////////////////////////////
//		      regsub stuff			      //
////////////////////////////////////////////////////////////////

typedef void (*fptr_T)(int *, int);

static int vim_regsub_both(char_u *source, typval_T *expr, char_u *dest, int destlen, int flags);

    static void
do_upper(int *d, int c)
{
    *d = MB_TOUPPER(c);
}

    static void
do_lower(int *d, int c)
{
    *d = MB_TOLOWER(c);
}

/*
 * regtilde(): Replace tildes in the pattern by the old pattern.
 *
 * Short explanation of the tilde: It stands for the previous replacement
 * pattern.  If that previous pattern also contains a ~ we should go back a
 * step further...  But we insert the previous pattern into the current one
 * and remember that.
 * This still does not handle the case where "magic" changes.  So require the
 * user to keep his hands off of "magic".
 *
 * The tildes are parsed once before the first call to vim_regsub().
 */
    char_u *
regtilde(char_u *source, int magic)
{
    char_u	*newsub = source;
    char_u	*p;

    for (p = newsub; *p; ++p)
    {
	if ((*p == '~' && magic) || (*p == '\\' && *(p + 1) == '~' && !magic))
	{
	    if (reg_prev_sub != NULL)
	    {
		// length = len(newsub) - 1 + len(prev_sub) + 1
		// Avoid making the text longer than MAXCOL, it will cause
		// trouble at some point.
		size_t	prevsublen = STRLEN(reg_prev_sub);
		size_t  newsublen = STRLEN(newsub);
		if (prevsublen > MAXCOL || newsublen > MAXCOL
					    || newsublen + prevsublen > MAXCOL)
		{
		    emsg(_(e_resulting_text_too_long));
		    break;
		}

		char_u *tmpsub = alloc(newsublen + prevsublen);
		if (tmpsub != NULL)
		{
		    // copy prefix
		    size_t prefixlen = p - newsub;	// not including ~
		    mch_memmove(tmpsub, newsub, prefixlen);
		    // interpret tilde
		    mch_memmove(tmpsub + prefixlen, reg_prev_sub,
							       prevsublen);
		    // copy postfix
		    if (!magic)
			++p;			// back off backslash
		    STRCPY(tmpsub + prefixlen + prevsublen, p + 1);

		    if (newsub != source)	// allocated newsub before
			vim_free(newsub);
		    newsub = tmpsub;
		    p = newsub + prefixlen + prevsublen;
		}
	    }
	    else if (magic)
		STRMOVE(p, p + 1);	// remove '~'
	    else
		STRMOVE(p, p + 2);	// remove '\~'
	    --p;
	}
	else
	{
	    if (*p == '\\' && p[1])		// skip escaped characters
		++p;
	    if (has_mbyte)
		p += (*mb_ptr2len)(p) - 1;
	}
    }

    // Store a copy of newsub  in reg_prev_sub.  It is always allocated,
    // because recursive calls may make the returned string invalid.
    vim_free(reg_prev_sub);
    reg_prev_sub = vim_strsave(newsub);

    return newsub;
}

#ifdef FEAT_EVAL
static int can_f_submatch = FALSE;	// TRUE when submatch() can be used

// These pointers are used for reg_submatch().  Needed for when the
// substitution string is an expression that contains a call to substitute()
// and submatch().
typedef struct {
    regmatch_T	*sm_match;
    regmmatch_T	*sm_mmatch;
    linenr_T	sm_firstlnum;
    linenr_T	sm_maxline;
    int		sm_line_lbr;
} regsubmatch_T;

static regsubmatch_T rsm;  // can only be used when can_f_submatch is TRUE
#endif

#ifdef FEAT_EVAL

/*
 * Put the submatches in "argv[argskip]" which is a list passed into
 * call_func() by vim_regsub_both().
 */
    static int
fill_submatch_list(int argc UNUSED, typval_T *argv, int argskip, ufunc_T *fp)
{
    listitem_T	*li;
    int		i;
    char_u	*s;
    typval_T	*listarg = argv + argskip;

    if (!has_varargs(fp) && fp->uf_args.ga_len <= argskip)
	// called function doesn't take a submatches argument
	return argskip;

    // Relies on sl_list to be the first item in staticList10_T.
    init_static_list((staticList10_T *)(listarg->vval.v_list));

    // There are always 10 list items in staticList10_T.
    li = listarg->vval.v_list->lv_first;
    for (i = 0; i < 10; ++i)
    {
	s = rsm.sm_match->startp[i];
	if (s == NULL || rsm.sm_match->endp[i] == NULL)
	    s = NULL;
	else
	    s = vim_strnsave(s, rsm.sm_match->endp[i] - s);
	li->li_tv.v_type = VAR_STRING;
	li->li_tv.vval.v_string = s;
	li = li->li_next;
    }
    return argskip + 1;
}

    static void
clear_submatch_list(staticList10_T *sl)
{
    int i;

    for (i = 0; i < 10; ++i)
	vim_free(sl->sl_items[i].li_tv.vval.v_string);
}
#endif

/*
 * vim_regsub() - perform substitutions after a vim_regexec() or
 * vim_regexec_multi() match.
 *
 * If "flags" has REGSUB_COPY really copy into "dest[destlen]".
 * Otherwise nothing is copied, only compute the length of the result.
 *
 * If "flags" has REGSUB_MAGIC then behave like 'magic' is set.
 *
 * If "flags" has REGSUB_BACKSLASH a backslash will be removed later, need to
 * double them to keep them, and insert a backslash before a CR to avoid it
 * being replaced with a line break later.
 *
 * Note: The matched text must not change between the call of
 * vim_regexec()/vim_regexec_multi() and vim_regsub()!  It would make the back
 * references invalid!
 *
 * Returns the size of the replacement, including terminating NUL.
 */
    int
vim_regsub(
    regmatch_T	*rmp,
    char_u	*source,
    typval_T	*expr,
    char_u	*dest,
    int		destlen,
    int		flags)
{
    int		result;
    regexec_T	rex_save;
    int		rex_in_use_save = rex_in_use;

    if (rex_in_use)
	// Being called recursively, save the state.
	rex_save = rex;
    rex_in_use = TRUE;

    rex.reg_match = rmp;
    rex.reg_mmatch = NULL;
    rex.reg_maxline = 0;
    rex.reg_buf = curbuf;
    rex.reg_line_lbr = TRUE;
    result = vim_regsub_both(source, expr, dest, destlen, flags);

    rex_in_use = rex_in_use_save;
    if (rex_in_use)
	rex = rex_save;

    return result;
}

    int
vim_regsub_multi(
    regmmatch_T	*rmp,
    linenr_T	lnum,
    char_u	*source,
    char_u	*dest,
    int		destlen,
    int		flags)
{
    int		result;
    regexec_T	rex_save;
    int		rex_in_use_save = rex_in_use;

    if (rex_in_use)
	// Being called recursively, save the state.
	rex_save = rex;
    rex_in_use = TRUE;

    rex.reg_match = NULL;
    rex.reg_mmatch = rmp;
    rex.reg_buf = curbuf;	// always works on the current buffer!
    rex.reg_firstlnum = lnum;
    rex.reg_maxline = curbuf->b_ml.ml_line_count - lnum;
    rex.reg_line_lbr = FALSE;
    result = vim_regsub_both(source, NULL, dest, destlen, flags);

    rex_in_use = rex_in_use_save;
    if (rex_in_use)
	rex = rex_save;

    return result;
}

#if defined(FEAT_EVAL) || defined(PROTO)
// When nesting more than a couple levels it's probably a mistake.
# define MAX_REGSUB_NESTING 4
static char_u   *eval_result[MAX_REGSUB_NESTING] = {NULL, NULL, NULL, NULL};

# if defined(EXITFREE) || defined(PROTO)
    void
free_resub_eval_result(void)
{
    int i;

    for (i = 0; i < MAX_REGSUB_NESTING; ++i)
	VIM_CLEAR(eval_result[i]);
}
# endif
#endif

    static int
vim_regsub_both(
    char_u	*source,
    typval_T	*expr,
    char_u	*dest,
    int		destlen,
    int		flags)
{
    char_u	*src;
    char_u	*dst;
    char_u	*s;
    int		c;
    int		cc;
    int		no = -1;
    fptr_T	func_all = (fptr_T)NULL;
    fptr_T	func_one = (fptr_T)NULL;
    linenr_T	clnum = 0;	// init for GCC
    int		len = 0;	// init for GCC
#ifdef FEAT_EVAL
    static int  nesting = 0;
    int		nested;
#endif
    int		copy = flags & REGSUB_COPY;

    // Be paranoid...
    if ((source == NULL && expr == NULL) || dest == NULL)
    {
	iemsg(e_null_argument);
	return 0;
    }
    if (prog_magic_wrong())
	return 0;
#ifdef FEAT_EVAL
    if (nesting == MAX_REGSUB_NESTING)
    {
	emsg(_(e_substitute_nesting_too_deep));
	return 0;
    }
    nested = nesting;
#endif
    src = source;
    dst = dest;

    /*
     * When the substitute part starts with "\=" evaluate it as an expression.
     */
    if (expr != NULL || (source[0] == '\\' && source[1] == '='))
    {
#ifdef FEAT_EVAL
	// To make sure that the length doesn't change between checking the
	// length and copying the string, and to speed up things, the
	// resulting string is saved from the call with
	// "flags & REGSUB_COPY" == 0 to the call with
	// "flags & REGSUB_COPY" != 0.
	if (copy)
	{
	    if (eval_result[nested] != NULL &&
		    (int)STRLEN(eval_result[nested]) < destlen)
	    {
		STRCPY(dest, eval_result[nested]);
		dst += STRLEN(eval_result[nested]);
		VIM_CLEAR(eval_result[nested]);
	    }
	}
	else
	{
	    int		    prev_can_f_submatch = can_f_submatch;
	    regsubmatch_T   rsm_save;

	    VIM_CLEAR(eval_result[nested]);

	    // The expression may contain substitute(), which calls us
	    // recursively.  Make sure submatch() gets the text from the first
	    // level.
	    if (can_f_submatch)
		rsm_save = rsm;
	    can_f_submatch = TRUE;
	    rsm.sm_match = rex.reg_match;
	    rsm.sm_mmatch = rex.reg_mmatch;
	    rsm.sm_firstlnum = rex.reg_firstlnum;
	    rsm.sm_maxline = rex.reg_maxline;
	    rsm.sm_line_lbr = rex.reg_line_lbr;

	    // Although unlikely, it is possible that the expression invokes a
	    // substitute command (it might fail, but still).  Therefore keep
	    // an array of eval results.
	    ++nesting;

	    if (expr != NULL)
	    {
		typval_T	argv[2];
		char_u		buf[NUMBUFLEN];
		typval_T	rettv;
		staticList10_T	matchList;
		funcexe_T	funcexe;

		rettv.v_type = VAR_STRING;
		rettv.vval.v_string = NULL;
		argv[0].v_type = VAR_LIST;
		argv[0].vval.v_list = &matchList.sl_list;
		matchList.sl_list.lv_len = 0;
		CLEAR_FIELD(funcexe);
		funcexe.fe_argv_func = fill_submatch_list;
		funcexe.fe_evaluate = TRUE;
		if (expr->v_type == VAR_FUNC)
		{
		    s = expr->vval.v_string;
		    call_func(s, -1, &rettv, 1, argv, &funcexe);
		}
		else if (expr->v_type == VAR_PARTIAL)
		{
		    partial_T   *partial = expr->vval.v_partial;

		    s = partial_name(partial);
		    funcexe.fe_partial = partial;
		    call_func(s, -1, &rettv, 1, argv, &funcexe);
		}
		else if (expr->v_type == VAR_INSTR)
		{
		    exe_typval_instr(expr, &rettv);
		}
		if (matchList.sl_list.lv_len > 0)
		    // fill_submatch_list() was called
		    clear_submatch_list(&matchList);

		if (rettv.v_type == VAR_UNKNOWN)
		    // something failed, no need to report another error
		    eval_result[nested] = NULL;
		else
		{
		    eval_result[nested] = tv_get_string_buf_chk(&rettv, buf);
		    if (eval_result[nested] != NULL)
			eval_result[nested] = vim_strsave(eval_result[nested]);
		}
		clear_tv(&rettv);
	    }
	    else if (substitute_instr != NULL)
		// Execute instructions from ISN_SUBSTITUTE.
		eval_result[nested] = exe_substitute_instr();
	    else
		eval_result[nested] = eval_to_string(source + 2, TRUE, FALSE);
	    --nesting;

	    if (eval_result[nested] != NULL)
	    {
		int had_backslash = FALSE;

		for (s = eval_result[nested]; *s != NUL; MB_PTR_ADV(s))
		{
		    // Change NL to CR, so that it becomes a line break,
		    // unless called from vim_regexec_nl().
		    // Skip over a backslashed character.
		    if (*s == NL && !rsm.sm_line_lbr)
			*s = CAR;
		    else if (*s == '\\' && s[1] != NUL)
		    {
			++s;
			/* Change NL to CR here too, so that this works:
			 * :s/abc\\\ndef/\="aaa\\\nbbb"/  on text:
			 *   abc\
			 *   def
			 * Not when called from vim_regexec_nl().
			 */
			if (*s == NL && !rsm.sm_line_lbr)
			    *s = CAR;
			had_backslash = TRUE;
		    }
		}
		if (had_backslash && (flags & REGSUB_BACKSLASH))
		{
		    // Backslashes will be consumed, need to double them.
		    s = vim_strsave_escaped(eval_result[nested], (char_u *)"\\");
		    if (s != NULL)
		    {
			vim_free(eval_result[nested]);
			eval_result[nested] = s;
		    }
		}

		dst += STRLEN(eval_result[nested]);
	    }

	    can_f_submatch = prev_can_f_submatch;
	    if (can_f_submatch)
		rsm = rsm_save;
	}
#endif
    }
    else
      while ((c = *src++) != NUL)
      {
	if (c == '&' && (flags & REGSUB_MAGIC))
	    no = 0;
	else if (c == '\\' && *src != NUL)
	{
	    if (*src == '&' && !(flags & REGSUB_MAGIC))
	    {
		++src;
		no = 0;
	    }
	    else if ('0' <= *src && *src <= '9')
	    {
		no = *src++ - '0';
	    }
	    else if (vim_strchr((char_u *)"uUlLeE", *src))
	    {
		switch (*src++)
		{
		case 'u':   func_one = do_upper;
			    continue;
		case 'U':   func_all = do_upper;
			    continue;
		case 'l':   func_one = do_lower;
			    continue;
		case 'L':   func_all = do_lower;
			    continue;
		case 'e':
		case 'E':   func_one = func_all = (fptr_T)NULL;
			    continue;
		}
	    }
	}
	if (no < 0)	      // Ordinary character.
	{
	    if (c == K_SPECIAL && src[0] != NUL && src[1] != NUL)
	    {
		// Copy a special key as-is.
		if (copy)
		{
		    if (dst + 3 > dest + destlen)
		    {
			iemsg("vim_regsub_both(): not enough space");
			return 0;
		    }
		    *dst++ = c;
		    *dst++ = *src++;
		    *dst++ = *src++;
		}
		else
		{
		    dst += 3;
		    src += 2;
		}
		continue;
	    }

	    if (c == '\\' && *src != NUL)
	    {
		// Check for abbreviations -- webb
		switch (*src)
		{
		    case 'r':	c = CAR;	++src;	break;
		    case 'n':	c = NL;		++src;	break;
		    case 't':	c = TAB;	++src;	break;
		 // Oh no!  \e already has meaning in subst pat :-(
		 // case 'e':   c = ESC;	++src;	break;
		    case 'b':	c = Ctrl_H;	++src;	break;

		    // If "backslash" is TRUE the backslash will be removed
		    // later.  Used to insert a literal CR.
		    default:	if (flags & REGSUB_BACKSLASH)
				{
				    if (copy)
				    {
					if (dst + 1 > dest + destlen)
					{
					    iemsg("vim_regsub_both(): not enough space");
					    return 0;
					}
					*dst = '\\';
				    }
				    ++dst;
				}
				c = *src++;
		}
	    }
	    else if (has_mbyte)
		c = mb_ptr2char(src - 1);

	    // Write to buffer, if copy is set.
	    if (func_one != (fptr_T)NULL)
	    {
		func_one(&cc, c);
		func_one = NULL;
	    }
	    else if (func_all != (fptr_T)NULL)
		func_all(&cc, c);
	    else // just copy
		cc = c;

	    if (has_mbyte)
	    {
		int totlen = mb_ptr2len(src - 1);
		int charlen = mb_char2len(cc);

		if (copy)
		{
		    if (dst + charlen > dest + destlen)
		    {
			iemsg("vim_regsub_both(): not enough space");
			return 0;
		    }
		    mb_char2bytes(cc, dst);
		}
		dst += charlen - 1;
		if (enc_utf8)
		{
		    int clen = utf_ptr2len(src - 1);

		    // If the character length is shorter than "totlen", there
		    // are composing characters; copy them as-is.
		    if (clen < totlen)
		    {
			if (copy)
			{
			    if (dst + totlen - clen > dest + destlen)
			    {
				iemsg("vim_regsub_both(): not enough space");
				return 0;
			    }
			    mch_memmove(dst + 1, src - 1 + clen,
						     (size_t)(totlen - clen));
			}
			dst += totlen - clen;
		    }
		}
		src += totlen - 1;
	    }
	    else if (copy)
	    {
		if (dst + 1 > dest + destlen)
		{
		    iemsg("vim_regsub_both(): not enough space");
		    return 0;
		}
		*dst = cc;
	    }
	    dst++;
	}
	else
	{
	    if (REG_MULTI)
	    {
		clnum = rex.reg_mmatch->startpos[no].lnum;
		if (clnum < 0 || rex.reg_mmatch->endpos[no].lnum < 0)
		    s = NULL;
		else
		{
		    s = reg_getline(clnum) + rex.reg_mmatch->startpos[no].col;
		    if (rex.reg_mmatch->endpos[no].lnum == clnum)
			len = rex.reg_mmatch->endpos[no].col
					    - rex.reg_mmatch->startpos[no].col;
		    else
			len = (int)STRLEN(s);
		}
	    }
	    else
	    {
		s = rex.reg_match->startp[no];
		if (rex.reg_match->endp[no] == NULL)
		    s = NULL;
		else
		    len = (int)(rex.reg_match->endp[no] - s);
	    }
	    if (s != NULL)
	    {
		for (;;)
		{
		    if (len == 0)
		    {
			if (REG_MULTI)
			{
			    if (rex.reg_mmatch->endpos[no].lnum == clnum)
				break;
			    if (copy)
			    {
				if (dst + 1 > dest + destlen)
				{
				    iemsg("vim_regsub_both(): not enough space");
				    return 0;
				}
				*dst = CAR;
			    }
			    ++dst;
			    s = reg_getline(++clnum);
			    if (rex.reg_mmatch->endpos[no].lnum == clnum)
				len = rex.reg_mmatch->endpos[no].col;
			    else
				len = (int)STRLEN(s);
			}
			else
			    break;
		    }
		    else if (*s == NUL) // we hit NUL.
		    {
			if (copy)
			    iemsg(e_damaged_match_string);
			goto exit;
		    }
		    else
		    {
			if ((flags & REGSUB_BACKSLASH)
						  && (*s == CAR || *s == '\\'))
			{
			    /*
			     * Insert a backslash in front of a CR, otherwise
			     * it will be replaced by a line break.
			     * Number of backslashes will be halved later,
			     * double them here.
			     */
			    if (copy)
			    {
				if (dst + 2 > dest + destlen)
				{
				    iemsg("vim_regsub_both(): not enough space");
				    return 0;
				}
				dst[0] = '\\';
				dst[1] = *s;
			    }
			    dst += 2;
			}
			else
			{
			    if (has_mbyte)
				c = mb_ptr2char(s);
			    else
				c = *s;

			    if (func_one != (fptr_T)NULL)
			    {
				func_one(&cc, c);
				func_one = NULL;
			    }
			    else if (func_all != (fptr_T)NULL)
				func_all(&cc, c);
			    else // just copy
				cc = c;

			    if (has_mbyte)
			    {
				int l;
				int charlen;

				// Copy composing characters separately, one
				// at a time.
				if (enc_utf8)
				    l = utf_ptr2len(s) - 1;
				else
				    l = mb_ptr2len(s) - 1;

				s += l;
				len -= l;
				charlen = mb_char2len(cc);
				if (copy)
				{
				    if (dst + charlen > dest + destlen)
				    {
					iemsg("vim_regsub_both(): not enough space");
					return 0;
				    }
				    mb_char2bytes(cc, dst);
				}
				dst += charlen - 1;
			    }
			    else if (copy)
			    {
				if (dst + 1 > dest + destlen)
				{
				    iemsg("vim_regsub_both(): not enough space");
				    return 0;
				}
				*dst = cc;
			    }
			    dst++;
			}

			++s;
			--len;
		    }
		}
	    }
	    no = -1;
	}
      }
    if (copy)
	*dst = NUL;

exit:
    return (int)((dst - dest) + 1);
}

#ifdef FEAT_EVAL
/*
 * Call reg_getline() with the line numbers from the submatch.  If a
 * substitute() was used the reg_maxline and other values have been
 * overwritten.
 */
    static char_u *
reg_getline_submatch(linenr_T lnum)
{
    char_u *s;
    linenr_T save_first = rex.reg_firstlnum;
    linenr_T save_max = rex.reg_maxline;

    rex.reg_firstlnum = rsm.sm_firstlnum;
    rex.reg_maxline = rsm.sm_maxline;

    s = reg_getline(lnum);

    rex.reg_firstlnum = save_first;
    rex.reg_maxline = save_max;
    return s;
}

/*
 * Used for the submatch() function: get the string from the n'th submatch in
 * allocated memory.
 * Returns NULL when not in a ":s" command and for a non-existing submatch.
 */
    char_u *
reg_submatch(int no)
{
    char_u	*retval = NULL;
    char_u	*s;
    int		len;
    int		round;
    linenr_T	lnum;

    if (!can_f_submatch || no < 0)
	return NULL;

    if (rsm.sm_match == NULL)
    {
	/*
	 * First round: compute the length and allocate memory.
	 * Second round: copy the text.
	 */
	for (round = 1; round <= 2; ++round)
	{
	    lnum = rsm.sm_mmatch->startpos[no].lnum;
	    if (lnum < 0 || rsm.sm_mmatch->endpos[no].lnum < 0)
		return NULL;

	    s = reg_getline_submatch(lnum);
	    if (s == NULL)  // anti-crash check, cannot happen?
		break;
	    s += rsm.sm_mmatch->startpos[no].col;
	    if (rsm.sm_mmatch->endpos[no].lnum == lnum)
	    {
		// Within one line: take form start to end col.
		len = rsm.sm_mmatch->endpos[no].col
					  - rsm.sm_mmatch->startpos[no].col;
		if (round == 2)
		    vim_strncpy(retval, s, len);
		++len;
	    }
	    else
	    {
		// Multiple lines: take start line from start col, middle
		// lines completely and end line up to end col.
		len = (int)STRLEN(s);
		if (round == 2)
		{
		    STRCPY(retval, s);
		    retval[len] = '\n';
		}
		++len;
		++lnum;
		while (lnum < rsm.sm_mmatch->endpos[no].lnum)
		{
		    s = reg_getline_submatch(lnum++);
		    if (round == 2)
			STRCPY(retval + len, s);
		    len += (int)STRLEN(s);
		    if (round == 2)
			retval[len] = '\n';
		    ++len;
		}
		if (round == 2)
		    STRNCPY(retval + len, reg_getline_submatch(lnum),
					     rsm.sm_mmatch->endpos[no].col);
		len += rsm.sm_mmatch->endpos[no].col;
		if (round == 2)
		    retval[len] = NUL;
		++len;
	    }

	    if (retval == NULL)
	    {
		retval = alloc(len);
		if (retval == NULL)
		    return NULL;
	    }
	}
    }
    else
    {
	s = rsm.sm_match->startp[no];
	if (s == NULL || rsm.sm_match->endp[no] == NULL)
	    retval = NULL;
	else
	    retval = vim_strnsave(s, rsm.sm_match->endp[no] - s);
    }

    return retval;
}

/*
 * Used for the submatch() function with the optional non-zero argument: get
 * the list of strings from the n'th submatch in allocated memory with NULs
 * represented in NLs.
 * Returns a list of allocated strings.  Returns NULL when not in a ":s"
 * command, for a non-existing submatch and for any error.
 */
    list_T *
reg_submatch_list(int no)
{
    char_u	*s;
    linenr_T	slnum;
    linenr_T	elnum;
    colnr_T	scol;
    colnr_T	ecol;
    int		i;
    list_T	*list;
    int		error = FALSE;

    if (!can_f_submatch || no < 0)
	return NULL;

    if (rsm.sm_match == NULL)
    {
	slnum = rsm.sm_mmatch->startpos[no].lnum;
	elnum = rsm.sm_mmatch->endpos[no].lnum;
	if (slnum < 0 || elnum < 0)
	    return NULL;

	scol = rsm.sm_mmatch->startpos[no].col;
	ecol = rsm.sm_mmatch->endpos[no].col;

	list = list_alloc();
	if (list == NULL)
	    return NULL;

	s = reg_getline_submatch(slnum) + scol;
	if (slnum == elnum)
	{
	    if (list_append_string(list, s, ecol - scol) == FAIL)
		error = TRUE;
	}
	else
	{
	    if (list_append_string(list, s, -1) == FAIL)
		error = TRUE;
	    for (i = 1; i < elnum - slnum; i++)
	    {
		s = reg_getline_submatch(slnum + i);
		if (list_append_string(list, s, -1) == FAIL)
		    error = TRUE;
	    }
	    s = reg_getline_submatch(elnum);
	    if (list_append_string(list, s, ecol) == FAIL)
		error = TRUE;
	}
    }
    else
    {
	s = rsm.sm_match->startp[no];
	if (s == NULL || rsm.sm_match->endp[no] == NULL)
	    return NULL;
	list = list_alloc();
	if (list == NULL)
	    return NULL;
	if (list_append_string(list, s,
				 (int)(rsm.sm_match->endp[no] - s)) == FAIL)
	    error = TRUE;
    }

    if (error)
    {
	list_free(list);
	return NULL;
    }
    ++list->lv_refcount;
    return list;
}
#endif

/*
 * Initialize the values used for matching against multiple lines
 */
    static void
init_regexec_multi(
	regmmatch_T	*rmp,
	win_T		*win,	// window in which to search or NULL
	buf_T		*buf,	// buffer in which to search
	linenr_T	lnum)	// nr of line to start looking for match
{
    rex.reg_match = NULL;
    rex.reg_mmatch = rmp;
    rex.reg_buf = buf;
    rex.reg_win = win;
    rex.reg_firstlnum = lnum;
    rex.reg_maxline = rex.reg_buf->b_ml.ml_line_count - lnum;
    rex.reg_line_lbr = FALSE;
    rex.reg_ic = rmp->rmm_ic;
    rex.reg_icombine = FALSE;
    rex.reg_maxcol = rmp->rmm_maxcol;
}

#include "regexp_bt.c"

static regengine_T bt_regengine =
{
    bt_regcomp,
    bt_regfree,
    bt_regexec_nl,
    bt_regexec_multi
#ifdef DEBUG
    ,(char_u *)""
#endif
};

#include "regexp_nfa.c"

static regengine_T nfa_regengine =
{
    nfa_regcomp,
    nfa_regfree,
    nfa_regexec_nl,
    nfa_regexec_multi
#ifdef DEBUG
    ,(char_u *)""
#endif
};

// Which regexp engine to use? Needed for vim_regcomp().
// Must match with 'regexpengine'.
static int regexp_engine = 0;

#ifdef DEBUG
static char_u regname[][30] = {
		    "AUTOMATIC Regexp Engine",
		    "BACKTRACKING Regexp Engine",
		    "NFA Regexp Engine"
			    };
#endif

/*
 * Compile a regular expression into internal code.
 * Returns the program in allocated memory.
 * Use vim_regfree() to free the memory.
 * Returns NULL for an error.
 */
    regprog_T *
vim_regcomp(char_u *expr_arg, int re_flags)
{
    regprog_T   *prog = NULL;
    char_u	*expr = expr_arg;
    int		called_emsg_before;

    regexp_engine = p_re;

    // Check for prefix "\%#=", that sets the regexp engine
    if (STRNCMP(expr, "\\%#=", 4) == 0)
    {
	int newengine = expr[4] - '0';

	if (newengine == AUTOMATIC_ENGINE
	    || newengine == BACKTRACKING_ENGINE
	    || newengine == NFA_ENGINE)
	{
	    regexp_engine = expr[4] - '0';
	    expr += 5;
#ifdef DEBUG
	    smsg("New regexp mode selected (%d): %s",
					   regexp_engine, regname[newengine]);
#endif
	}
	else
	{
	    emsg(_(e_percent_hash_can_only_be_followed_by_zero_one_two_automatic_engine_will_be_used));
	    regexp_engine = AUTOMATIC_ENGINE;
	}
    }
#ifdef DEBUG
    bt_regengine.expr = expr;
    nfa_regengine.expr = expr;
#endif
    // reg_iswordc() uses rex.reg_buf
    rex.reg_buf = curbuf;

    /*
     * First try the NFA engine, unless backtracking was requested.
     */
    called_emsg_before = called_emsg;
    if (regexp_engine != BACKTRACKING_ENGINE)
	prog = nfa_regengine.regcomp(expr,
		re_flags + (regexp_engine == AUTOMATIC_ENGINE ? RE_AUTO : 0));
    else
	prog = bt_regengine.regcomp(expr, re_flags);

    // Check for error compiling regexp with initial engine.
    if (prog == NULL)
    {
#ifdef BT_REGEXP_DEBUG_LOG
	if (regexp_engine == BACKTRACKING_ENGINE)   // debugging log for BT engine
	{
	    FILE *f;
	    f = fopen(BT_REGEXP_DEBUG_LOG_NAME, "a");
	    if (f)
	    {
		fprintf(f, "Syntax error in \"%s\"\n", expr);
		fclose(f);
	    }
	    else
		semsg("(NFA) Could not open \"%s\" to write !!!",
			BT_REGEXP_DEBUG_LOG_NAME);
	}
#endif
	/*
	 * If the NFA engine failed, try the backtracking engine.
	 * The NFA engine also fails for patterns that it can't handle well
	 * but are still valid patterns, thus a retry should work.
	 * But don't try if an error message was given.
	 */
	if (regexp_engine == AUTOMATIC_ENGINE
					  && called_emsg == called_emsg_before)
	{
	    regexp_engine = BACKTRACKING_ENGINE;
#ifdef FEAT_EVAL
	    report_re_switch(expr);
#endif
	    prog = bt_regengine.regcomp(expr, re_flags);
	}
    }

    if (prog != NULL)
    {
	// Store the info needed to call regcomp() again when the engine turns
	// out to be very slow when executing it.
	prog->re_engine = regexp_engine;
	prog->re_flags  = re_flags;
    }

    return prog;
}

/*
 * Free a compiled regexp program, returned by vim_regcomp().
 */
    void
vim_regfree(regprog_T *prog)
{
    if (prog != NULL)
	prog->engine->regfree(prog);
}

#if defined(EXITFREE) || defined(PROTO)
    void
free_regexp_stuff(void)
{
    ga_clear(&regstack);
    ga_clear(&backpos);
    vim_free(reg_tofree);
    vim_free(reg_prev_sub);
}
#endif

#ifdef FEAT_EVAL
    static void
report_re_switch(char_u *pat)
{
    if (p_verbose > 0)
    {
	verbose_enter();
	msg_puts(_("Switching to backtracking RE engine for pattern: "));
	msg_puts((char *)pat);
	verbose_leave();
    }
}
#endif

#if defined(FEAT_X11) || defined(PROTO)
/*
 * Return whether "prog" is currently being executed.
 */
    int
regprog_in_use(regprog_T *prog)
{
    return prog->re_in_use;
}
#endif

/*
 * Match a regexp against a string.
 * "rmp->regprog" must be a compiled regexp as returned by vim_regcomp().
 * Note: "rmp->regprog" may be freed and changed.
 * Uses curbuf for line count and 'iskeyword'.
 * When "nl" is TRUE consider a "\n" in "line" to be a line break.
 *
 * Return TRUE if there is a match, FALSE if not.
 */
    static int
vim_regexec_string(
    regmatch_T	*rmp,
    char_u	*line,  // string to match against
    colnr_T	col,    // column to start looking for match
    int		nl)
{
    int		result;
    regexec_T	rex_save;
    int		rex_in_use_save = rex_in_use;

    // Cannot use the same prog recursively, it contains state.
    if (rmp->regprog->re_in_use)
    {
	emsg(_(e_cannot_use_pattern_recursively));
	return FALSE;
    }
    rmp->regprog->re_in_use = TRUE;

    if (rex_in_use)
	// Being called recursively, save the state.
	rex_save = rex;
    rex_in_use = TRUE;

    rex.reg_startp = NULL;
    rex.reg_endp = NULL;
    rex.reg_startpos = NULL;
    rex.reg_endpos = NULL;

    result = rmp->regprog->engine->regexec_nl(rmp, line, col, nl);
    rmp->regprog->re_in_use = FALSE;

    // NFA engine aborted because it's very slow.
    if (rmp->regprog->re_engine == AUTOMATIC_ENGINE
					       && result == NFA_TOO_EXPENSIVE)
    {
	int    save_p_re = p_re;
	int    re_flags = rmp->regprog->re_flags;
	char_u *pat = vim_strsave(((nfa_regprog_T *)rmp->regprog)->pattern);

	p_re = BACKTRACKING_ENGINE;
	vim_regfree(rmp->regprog);
	if (pat != NULL)
	{
#ifdef FEAT_EVAL
	    report_re_switch(pat);
#endif
	    rmp->regprog = vim_regcomp(pat, re_flags);
	    if (rmp->regprog != NULL)
	    {
		rmp->regprog->re_in_use = TRUE;
		result = rmp->regprog->engine->regexec_nl(rmp, line, col, nl);
		rmp->regprog->re_in_use = FALSE;
	    }
	    vim_free(pat);
	}

	p_re = save_p_re;
    }

    rex_in_use = rex_in_use_save;
    if (rex_in_use)
	rex = rex_save;

    return result > 0;
}

#if defined(FEAT_SPELL) || defined(FEAT_EVAL) || defined(FEAT_X11) || defined(PROTO)
/*
 * Note: "*prog" may be freed and changed.
 * Return TRUE if there is a match, FALSE if not.
 */
    int
vim_regexec_prog(
    regprog_T	**prog,
    int		ignore_case,
    char_u	*line,
    colnr_T	col)
{
    int		r;
    regmatch_T	regmatch;

    regmatch.regprog = *prog;
    regmatch.rm_ic = ignore_case;
    r = vim_regexec_string(&regmatch, line, col, FALSE);
    *prog = regmatch.regprog;
    return r;
}
#endif

/*
 * Note: "rmp->regprog" may be freed and changed.
 * Return TRUE if there is a match, FALSE if not.
 */
    int
vim_regexec(regmatch_T *rmp, char_u *line, colnr_T col)
{
    return vim_regexec_string(rmp, line, col, FALSE);
}

/*
 * Like vim_regexec(), but consider a "\n" in "line" to be a line break.
 * Note: "rmp->regprog" may be freed and changed.
 * Return TRUE if there is a match, FALSE if not.
 */
    int
vim_regexec_nl(regmatch_T *rmp, char_u *line, colnr_T col)
{
    return vim_regexec_string(rmp, line, col, TRUE);
}

/*
 * Match a regexp against multiple lines.
 * "rmp->regprog" must be a compiled regexp as returned by vim_regcomp().
 * Note: "rmp->regprog" may be freed and changed, even set to NULL.
 * Uses curbuf for line count and 'iskeyword'.
 *
 * Return zero if there is no match.  Return number of lines contained in the
 * match otherwise.
 */
    long
vim_regexec_multi(
    regmmatch_T *rmp,
    win_T       *win,		// window in which to search or NULL
    buf_T       *buf,		// buffer in which to search
    linenr_T	lnum,		// nr of line to start looking for match
    colnr_T	col,		// column to start looking for match
    int		*timed_out)	// flag is set when timeout limit reached
{
    int		result;
    regexec_T	rex_save;
    int		rex_in_use_save = rex_in_use;

    // Cannot use the same prog recursively, it contains state.
    if (rmp->regprog->re_in_use)
    {
	emsg(_(e_cannot_use_pattern_recursively));
	return FALSE;
    }
    rmp->regprog->re_in_use = TRUE;

    if (rex_in_use)
	// Being called recursively, save the state.
	rex_save = rex;
    rex_in_use = TRUE;

    result = rmp->regprog->engine->regexec_multi(
				      rmp, win, buf, lnum, col, timed_out);
    rmp->regprog->re_in_use = FALSE;

    // NFA engine aborted because it's very slow.
    if (rmp->regprog->re_engine == AUTOMATIC_ENGINE
					       && result == NFA_TOO_EXPENSIVE)
    {
	int    save_p_re = p_re;
	int    re_flags = rmp->regprog->re_flags;
	char_u *pat = vim_strsave(((nfa_regprog_T *)rmp->regprog)->pattern);

	p_re = BACKTRACKING_ENGINE;
	if (pat != NULL)
	{
	    regprog_T *prev_prog = rmp->regprog;

#ifdef FEAT_EVAL
	    report_re_switch(pat);
#endif
#ifdef FEAT_SYN_HL
	    // checking for \z misuse was already done when compiling for NFA,
	    // allow all here
	    reg_do_extmatch = REX_ALL;
#endif
	    rmp->regprog = vim_regcomp(pat, re_flags);
#ifdef FEAT_SYN_HL
	    reg_do_extmatch = 0;
#endif
	    if (rmp->regprog == NULL)
	    {
		// Somehow compiling the pattern failed now, put back the
		// previous one to avoid "regprog" becoming NULL.
		rmp->regprog = prev_prog;
	    }
	    else
	    {
		vim_regfree(prev_prog);

		rmp->regprog->re_in_use = TRUE;
		result = rmp->regprog->engine->regexec_multi(
				      rmp, win, buf, lnum, col, timed_out);
		rmp->regprog->re_in_use = FALSE;
	    }
	    vim_free(pat);
	}
	p_re = save_p_re;
    }

    rex_in_use = rex_in_use_save;
    if (rex_in_use)
	rex = rex_save;

    return result <= 0 ? 0 : result;
}
