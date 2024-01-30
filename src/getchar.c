/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * getchar.c: Code related to getting a character from the user or a script
 * file, manipulations with redo buffer and stuff buffer.
 */

#include "vim.h"

/*
 * These buffers are used for storing:
 * - stuffed characters: A command that is translated into another command.
 * - redo characters: will redo the last change.
 * - recorded characters: for the "q" command.
 *
 * The bytes are stored like in the typeahead buffer:
 * - K_SPECIAL introduces a special key (two more bytes follow).  A literal
 *   K_SPECIAL is stored as K_SPECIAL KS_SPECIAL KE_FILLER.
 * - CSI introduces a GUI termcap code (also when gui.in_use is FALSE,
 *   otherwise switching the GUI on would make mappings invalid).
 *   A literal CSI is stored as CSI KS_EXTRA KE_CSI.
 * These translations are also done on multi-byte characters!
 *
 * Escaping CSI bytes is done by the system-specific input functions, called
 * by ui_inchar().
 * Escaping K_SPECIAL is done by inchar().
 * Un-escaping is done by vgetc().
 */

#define MINIMAL_SIZE 20			// minimal size for b_str

static buffheader_T redobuff = {{NULL, {NUL}}, NULL, 0, 0};
static buffheader_T old_redobuff = {{NULL, {NUL}}, NULL, 0, 0};
static buffheader_T recordbuff = {{NULL, {NUL}}, NULL, 0, 0};

static int typeahead_char = 0;		// typeahead char that's not flushed

/*
 * When block_redo is TRUE the redo buffer will not be changed.
 * Used by edit() to repeat insertions.
 */
static int	block_redo = FALSE;

static int	KeyNoremap = 0;	    // remapping flags

/*
 * Variables used by vgetorpeek() and flush_buffers().
 *
 * typebuf.tb_buf[] contains all characters that are not consumed yet.
 * typebuf.tb_buf[typebuf.tb_off] is the first valid character.
 * typebuf.tb_buf[typebuf.tb_off + typebuf.tb_len - 1] is the last valid char.
 * typebuf.tb_buf[typebuf.tb_off + typebuf.tb_len] must be NUL.
 * The head of the buffer may contain the result of mappings, abbreviations
 * and @a commands.  The length of this part is typebuf.tb_maplen.
 * typebuf.tb_silent is the part where <silent> applies.
 * After the head are characters that come from the terminal.
 * typebuf.tb_no_abbr_cnt is the number of characters in typebuf.tb_buf that
 * should not be considered for abbreviations.
 * Some parts of typebuf.tb_buf may not be mapped. These parts are remembered
 * in typebuf.tb_noremap[], which is the same length as typebuf.tb_buf and
 * contains RM_NONE for the characters that are not to be remapped.
 * typebuf.tb_noremap[typebuf.tb_off] is the first valid flag.
 * (typebuf has been put in globals.h, because check_termcode() needs it).
 */
#define RM_YES		0	// tb_noremap: remap
#define RM_NONE		1	// tb_noremap: don't remap
#define RM_SCRIPT	2	// tb_noremap: remap local script mappings
#define RM_ABBR		4	// tb_noremap: don't remap, do abbrev.

// typebuf.tb_buf has three parts: room in front (for result of mappings), the
// middle for typeahead and room for new characters (which needs to be 3 *
// MAXMAPLEN for the Amiga).
#define TYPELEN_INIT	(5 * (MAXMAPLEN + 3))
static char_u	typebuf_init[TYPELEN_INIT];	// initial typebuf.tb_buf
static char_u	noremapbuf_init[TYPELEN_INIT];	// initial typebuf.tb_noremap

static int	last_recorded_len = 0;	// number of last recorded chars

#ifdef FEAT_EVAL
mapblock_T	*last_used_map = NULL;
int		last_used_sid = -1;
#endif

static int	read_readbuf(buffheader_T *buf, int advance);
static void	init_typebuf(void);
static void	may_sync_undo(void);
static void	free_typebuf(void);
static void	closescript(void);
static void	updatescript(int c);
static int	vgetorpeek(int);
static int	inchar(char_u *buf, int maxlen, long wait_time);

/*
 * Free and clear a buffer.
 */
    static void
free_buff(buffheader_T *buf)
{
    buffblock_T	*p, *np;

    for (p = buf->bh_first.b_next; p != NULL; p = np)
    {
	np = p->b_next;
	vim_free(p);
    }
    buf->bh_first.b_next = NULL;
    buf->bh_curr = NULL;
}

/*
 * Return the contents of a buffer as a single string.
 * K_SPECIAL and CSI in the returned string are escaped.
 */
    static char_u *
get_buffcont(
    buffheader_T	*buffer,
    int			dozero)	    // count == zero is not an error
{
    long_u	    count = 0;
    char_u	    *p = NULL;
    char_u	    *p2;
    char_u	    *str;
    buffblock_T *bp;

    // compute the total length of the string
    for (bp = buffer->bh_first.b_next; bp != NULL; bp = bp->b_next)
	count += (long_u)STRLEN(bp->b_str);

    if ((count > 0 || dozero) && (p = alloc(count + 1)) != NULL)
    {
	p2 = p;
	for (bp = buffer->bh_first.b_next; bp != NULL; bp = bp->b_next)
	    for (str = bp->b_str; *str; )
		*p2++ = *str++;
	*p2 = NUL;
    }
    return p;
}

/*
 * Return the contents of the record buffer as a single string
 * and clear the record buffer.
 * K_SPECIAL and CSI in the returned string are escaped.
 */
    char_u *
get_recorded(void)
{
    char_u	*p;
    size_t	len;

    p = get_buffcont(&recordbuff, TRUE);
    free_buff(&recordbuff);

    /*
     * Remove the characters that were added the last time, these must be the
     * (possibly mapped) characters that stopped the recording.
     */
    len = STRLEN(p);
    if ((int)len >= last_recorded_len)
    {
	len -= last_recorded_len;
	p[len] = NUL;
    }

    /*
     * When stopping recording from Insert mode with CTRL-O q, also remove the
     * CTRL-O.
     */
    if (len > 0 && restart_edit != 0 && p[len - 1] == Ctrl_O)
	p[len - 1] = NUL;

    return (p);
}

/*
 * Return the contents of the redo buffer as a single string.
 * K_SPECIAL and CSI in the returned string are escaped.
 */
    char_u *
get_inserted(void)
{
    return get_buffcont(&redobuff, FALSE);
}

/*
 * Add string "s" after the current block of buffer "buf".
 * K_SPECIAL and CSI should have been escaped already.
 */
    static void
add_buff(
    buffheader_T	*buf,
    char_u		*s,
    long		slen)	// length of "s" or -1
{
    buffblock_T *p;
    long_u	    len;

    if (slen < 0)
	slen = (long)STRLEN(s);
    if (slen == 0)				// don't add empty strings
	return;

    if (buf->bh_first.b_next == NULL)	// first add to list
    {
	buf->bh_space = 0;
	buf->bh_curr = &(buf->bh_first);
    }
    else if (buf->bh_curr == NULL)	// buffer has already been read
    {
	iemsg(e_add_to_internal_buffer_that_was_already_read_from);
	return;
    }
    else if (buf->bh_index != 0)
	mch_memmove(buf->bh_first.b_next->b_str,
		    buf->bh_first.b_next->b_str + buf->bh_index,
		    STRLEN(buf->bh_first.b_next->b_str + buf->bh_index) + 1);
    buf->bh_index = 0;

    if (buf->bh_space >= (int)slen)
    {
	len = (long_u)STRLEN(buf->bh_curr->b_str);
	vim_strncpy(buf->bh_curr->b_str + len, s, (size_t)slen);
	buf->bh_space -= slen;
    }
    else
    {
	if (slen < MINIMAL_SIZE)
	    len = MINIMAL_SIZE;
	else
	    len = slen;
	p = alloc(offsetof(buffblock_T, b_str) + len + 1);
	if (p == NULL)
	    return; // no space, just forget it
	buf->bh_space = (int)(len - slen);
	vim_strncpy(p->b_str, s, (size_t)slen);

	p->b_next = buf->bh_curr->b_next;
	buf->bh_curr->b_next = p;
	buf->bh_curr = p;
    }
}

/*
 * Delete "slen" bytes from the end of "buf".
 * Only works when it was just added.
 */
    static void
delete_buff_tail(buffheader_T *buf, int slen)
{
    int len;

    if (buf->bh_curr == NULL)
	return;  // nothing to delete
    len = (int)STRLEN(buf->bh_curr->b_str);
    if (len < slen)
	return;

    buf->bh_curr->b_str[len - slen] = NUL;
    buf->bh_space += slen;
}

/*
 * Add number "n" to buffer "buf".
 */
    static void
add_num_buff(buffheader_T *buf, long n)
{
    char_u	number[32];

    sprintf((char *)number, "%ld", n);
    add_buff(buf, number, -1L);
}

/*
 * Add character 'c' to buffer "buf".
 * Translates special keys, NUL, CSI, K_SPECIAL and multibyte characters.
 */
    static void
add_char_buff(buffheader_T *buf, int c)
{
    char_u	bytes[MB_MAXBYTES + 1];
    int		len;
    int		i;
    char_u	temp[4];

    if (IS_SPECIAL(c))
	len = 1;
    else
	len = (*mb_char2bytes)(c, bytes);
    for (i = 0; i < len; ++i)
    {
	if (!IS_SPECIAL(c))
	    c = bytes[i];

	if (IS_SPECIAL(c) || c == K_SPECIAL || c == NUL)
	{
	    // translate special key code into three byte sequence
	    temp[0] = K_SPECIAL;
	    temp[1] = K_SECOND(c);
	    temp[2] = K_THIRD(c);
	    temp[3] = NUL;
	}
#ifdef FEAT_GUI
	else if (c == CSI)
	{
	    // Translate a CSI to a CSI - KS_EXTRA - KE_CSI sequence
	    temp[0] = CSI;
	    temp[1] = KS_EXTRA;
	    temp[2] = (int)KE_CSI;
	    temp[3] = NUL;
	}
#endif
	else
	{
	    temp[0] = c;
	    temp[1] = NUL;
	}
	add_buff(buf, temp, -1L);
    }
}

// First read ahead buffer. Used for translated commands.
static buffheader_T readbuf1 = {{NULL, {NUL}}, NULL, 0, 0};

// Second read ahead buffer. Used for redo.
static buffheader_T readbuf2 = {{NULL, {NUL}}, NULL, 0, 0};

/*
 * Get one byte from the read buffers.  Use readbuf1 one first, use readbuf2
 * if that one is empty.
 * If advance == TRUE go to the next char.
 * No translation is done K_SPECIAL and CSI are escaped.
 */
    static int
read_readbuffers(int advance)
{
    int c;

    c = read_readbuf(&readbuf1, advance);
    if (c == NUL)
	c = read_readbuf(&readbuf2, advance);
    return c;
}

    static int
read_readbuf(buffheader_T *buf, int advance)
{
    char_u	c;
    buffblock_T	*curr;

    if (buf->bh_first.b_next == NULL)  // buffer is empty
	return NUL;

    curr = buf->bh_first.b_next;
    c = curr->b_str[buf->bh_index];

    if (advance)
    {
	if (curr->b_str[++buf->bh_index] == NUL)
	{
	    buf->bh_first.b_next = curr->b_next;
	    vim_free(curr);
	    buf->bh_index = 0;
	}
    }
    return c;
}

/*
 * Prepare the read buffers for reading (if they contain something).
 */
    static void
start_stuff(void)
{
    if (readbuf1.bh_first.b_next != NULL)
    {
	readbuf1.bh_curr = &(readbuf1.bh_first);
	readbuf1.bh_space = 0;
    }
    if (readbuf2.bh_first.b_next != NULL)
    {
	readbuf2.bh_curr = &(readbuf2.bh_first);
	readbuf2.bh_space = 0;
    }
}

/*
 * Return TRUE if the stuff buffer is empty.
 */
    int
stuff_empty(void)
{
    return (readbuf1.bh_first.b_next == NULL
	 && readbuf2.bh_first.b_next == NULL);
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return TRUE if readbuf1 is empty.  There may still be redo characters in
 * redbuf2.
 */
    int
readbuf1_empty(void)
{
    return (readbuf1.bh_first.b_next == NULL);
}
#endif

/*
 * Set a typeahead character that won't be flushed.
 */
    void
typeahead_noflush(int c)
{
    typeahead_char = c;
}

/*
 * Remove the contents of the stuff buffer and the mapped characters in the
 * typeahead buffer (used in case of an error).  If "flush_typeahead" is true,
 * flush all typeahead characters (used when interrupted by a CTRL-C).
 */
    void
flush_buffers(flush_buffers_T flush_typeahead)
{
    init_typebuf();

    start_stuff();
    while (read_readbuffers(TRUE) != NUL)
	;

    if (flush_typeahead == FLUSH_MINIMAL)
    {
	// remove mapped characters at the start only
	typebuf.tb_off += typebuf.tb_maplen;
	typebuf.tb_len -= typebuf.tb_maplen;
#if defined(FEAT_CLIENTSERVER) || defined(FEAT_EVAL)
	if (typebuf.tb_len == 0)
	    typebuf_was_filled = FALSE;
#endif
    }
    else
    {
	// remove typeahead
	if (flush_typeahead == FLUSH_INPUT)
	    // We have to get all characters, because we may delete the first
	    // part of an escape sequence.  In an xterm we get one char at a
	    // time and we have to get them all.
	    while (inchar(typebuf.tb_buf, typebuf.tb_buflen - 1, 10L) != 0)
		;
	typebuf.tb_off = MAXMAPLEN;
	typebuf.tb_len = 0;
#if defined(FEAT_CLIENTSERVER) || defined(FEAT_EVAL)
	// Reset the flag that text received from a client or from feedkeys()
	// was inserted in the typeahead buffer.
	typebuf_was_filled = FALSE;
#endif
    }
    typebuf.tb_maplen = 0;
    typebuf.tb_silent = 0;
    cmd_silent = FALSE;
    typebuf.tb_no_abbr_cnt = 0;
    if (++typebuf.tb_change_cnt == 0)
	typebuf.tb_change_cnt = 1;
}

/*
 * The previous contents of the redo buffer is kept in old_redobuffer.
 * This is used for the CTRL-O <.> command in insert mode.
 */
    void
ResetRedobuff(void)
{
    if (block_redo)
	return;

    free_buff(&old_redobuff);
    old_redobuff = redobuff;
    redobuff.bh_first.b_next = NULL;
}

/*
 * Discard the contents of the redo buffer and restore the previous redo
 * buffer.
 */
    void
CancelRedo(void)
{
    if (block_redo)
	return;

    free_buff(&redobuff);
    redobuff = old_redobuff;
    old_redobuff.bh_first.b_next = NULL;
    start_stuff();
    while (read_readbuffers(TRUE) != NUL)
	;
}

/*
 * Save redobuff and old_redobuff to save_redobuff and save_old_redobuff.
 * Used before executing autocommands and user functions.
 */
    void
saveRedobuff(save_redo_T *save_redo)
{
    char_u	*s;

    save_redo->sr_redobuff = redobuff;
    redobuff.bh_first.b_next = NULL;
    save_redo->sr_old_redobuff = old_redobuff;
    old_redobuff.bh_first.b_next = NULL;

    // Make a copy, so that ":normal ." in a function works.
    s = get_buffcont(&save_redo->sr_redobuff, FALSE);
    if (s == NULL)
	return;

    add_buff(&redobuff, s, -1L);
    vim_free(s);
}

/*
 * Restore redobuff and old_redobuff from save_redobuff and save_old_redobuff.
 * Used after executing autocommands and user functions.
 */
    void
restoreRedobuff(save_redo_T *save_redo)
{
    free_buff(&redobuff);
    redobuff = save_redo->sr_redobuff;
    free_buff(&old_redobuff);
    old_redobuff = save_redo->sr_old_redobuff;
}

/*
 * Append "s" to the redo buffer.
 * K_SPECIAL and CSI should already have been escaped.
 */
    void
AppendToRedobuff(char_u *s)
{
    if (!block_redo)
	add_buff(&redobuff, s, -1L);
}

/*
 * Append to Redo buffer literally, escaping special characters with CTRL-V.
 * K_SPECIAL and CSI are escaped as well.
 */
    void
AppendToRedobuffLit(
    char_u	*str,
    int		len)	    // length of "str" or -1 for up to the NUL
{
    char_u	*s = str;
    int		c;
    char_u	*start;

    if (block_redo)
	return;

    while (len < 0 ? *s != NUL : s - str < len)
    {
	// Put a string of normal characters in the redo buffer (that's
	// faster).
	start = s;
	while (*s >= ' ' && *s < DEL && (len < 0 || s - str < len))
	    ++s;

	// Don't put '0' or '^' as last character, just in case a CTRL-D is
	// typed next.
	if (*s == NUL && (s[-1] == '0' || s[-1] == '^'))
	    --s;
	if (s > start)
	    add_buff(&redobuff, start, (long)(s - start));

	if (*s == NUL || (len >= 0 && s - str >= len))
	    break;

	// Handle a special or multibyte character.
	if (has_mbyte)
	    // Handle composing chars separately.
	    c = mb_cptr2char_adv(&s);
	else
	    c = *s++;
	if (c < ' ' || c == DEL || (*s == NUL && (c == '0' || c == '^')))
	    add_char_buff(&redobuff, Ctrl_V);

	// CTRL-V '0' must be inserted as CTRL-V 048
	if (*s == NUL && c == '0')
	    add_buff(&redobuff, (char_u *)"048", 3L);
	else
	    add_char_buff(&redobuff, c);
    }
}

/*
 * Append "s" to the redo buffer, leaving 3-byte special key codes unmodified
 * and escaping other K_SPECIAL and CSI bytes.
 */
    void
AppendToRedobuffSpec(char_u *s)
{
    if (block_redo)
	return;

    while (*s != NUL)
    {
	if (*s == K_SPECIAL && s[1] != NUL && s[2] != NUL)
	{
	    // Insert special key literally.
	    add_buff(&redobuff, s, 3L);
	    s += 3;
	}
	else
	    add_char_buff(&redobuff, mb_cptr2char_adv(&s));
    }
}

/*
 * Append a character to the redo buffer.
 * Translates special keys, NUL, CSI, K_SPECIAL and multibyte characters.
 */
    void
AppendCharToRedobuff(int c)
{
    if (!block_redo)
	add_char_buff(&redobuff, c);
}

/*
 * Append a number to the redo buffer.
 */
    void
AppendNumberToRedobuff(long n)
{
    if (!block_redo)
	add_num_buff(&redobuff, n);
}

/*
 * Append string "s" to the stuff buffer.
 * CSI and K_SPECIAL must already have been escaped.
 */
    void
stuffReadbuff(char_u *s)
{
    add_buff(&readbuf1, s, -1L);
}

/*
 * Append string "s" to the redo stuff buffer.
 * CSI and K_SPECIAL must already have been escaped.
 */
    void
stuffRedoReadbuff(char_u *s)
{
    add_buff(&readbuf2, s, -1L);
}

    static void
stuffReadbuffLen(char_u *s, long len)
{
    add_buff(&readbuf1, s, len);
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Stuff "s" into the stuff buffer, leaving special key codes unmodified and
 * escaping other K_SPECIAL and CSI bytes.
 * Change CR, LF and ESC into a space.
 */
    void
stuffReadbuffSpec(char_u *s)
{
    int c;

    while (*s != NUL)
    {
	if (*s == K_SPECIAL && s[1] != NUL && s[2] != NUL)
	{
	    // Insert special key literally.
	    stuffReadbuffLen(s, 3L);
	    s += 3;
	}
	else
	{
	    c = mb_cptr2char_adv(&s);
	    if (c == CAR || c == NL || c == ESC)
		c = ' ';
	    stuffcharReadbuff(c);
	}
    }
}
#endif

/*
 * Append a character to the stuff buffer.
 * Translates special keys, NUL, CSI, K_SPECIAL and multibyte characters.
 */
    void
stuffcharReadbuff(int c)
{
    add_char_buff(&readbuf1, c);
}

/*
 * Append a number to the stuff buffer.
 */
    void
stuffnumReadbuff(long n)
{
    add_num_buff(&readbuf1, n);
}

/*
 * Stuff a string into the typeahead buffer, such that edit() will insert it
 * literally ("literally" TRUE) or interpret is as typed characters.
 */
    void
stuffescaped(char_u *arg, int literally)
{
    int		c;
    char_u	*start;

    while (*arg != NUL)
    {
	// Stuff a sequence of normal ASCII characters, that's fast.  Also
	// stuff K_SPECIAL to get the effect of a special key when "literally"
	// is TRUE.
	start = arg;
	while ((*arg >= ' ' && *arg < DEL)
		|| (*arg == K_SPECIAL && !literally))
	    ++arg;
	if (arg > start)
	    stuffReadbuffLen(start, (long)(arg - start));

	// stuff a single special character
	if (*arg != NUL)
	{
	    if (has_mbyte)
		c = mb_cptr2char_adv(&arg);
	    else
		c = *arg++;
	    if (literally && ((c < ' ' && c != TAB) || c == DEL))
		stuffcharReadbuff(Ctrl_V);
	    stuffcharReadbuff(c);
	}
    }
}

/*
 * Read a character from the redo buffer.  Translates K_SPECIAL, CSI and
 * multibyte characters.
 * The redo buffer is left as it is.
 * If init is TRUE, prepare for redo, return FAIL if nothing to redo, OK
 * otherwise.
 * If old is TRUE, use old_redobuff instead of redobuff.
 */
    static int
read_redo(int init, int old_redo)
{
    static buffblock_T	*bp;
    static char_u	*p;
    int			c;
    int			n;
    char_u		buf[MB_MAXBYTES + 1];
    int			i;

    if (init)
    {
	if (old_redo)
	    bp = old_redobuff.bh_first.b_next;
	else
	    bp = redobuff.bh_first.b_next;
	if (bp == NULL)
	    return FAIL;
	p = bp->b_str;
	return OK;
    }
    if ((c = *p) != NUL)
    {
	// Reverse the conversion done by add_char_buff()
	// For a multi-byte character get all the bytes and return the
	// converted character.
	if (has_mbyte && (c != K_SPECIAL || p[1] == KS_SPECIAL))
	    n = MB_BYTE2LEN_CHECK(c);
	else
	    n = 1;
	for (i = 0; ; ++i)
	{
	    if (c == K_SPECIAL) // special key or escaped K_SPECIAL
	    {
		c = TO_SPECIAL(p[1], p[2]);
		p += 2;
	    }
#ifdef FEAT_GUI
	    if (c == CSI)	// escaped CSI
		p += 2;
#endif
	    if (*++p == NUL && bp->b_next != NULL)
	    {
		bp = bp->b_next;
		p = bp->b_str;
	    }
	    buf[i] = c;
	    if (i == n - 1)	// last byte of a character
	    {
		if (n != 1)
		    c = (*mb_ptr2char)(buf);
		break;
	    }
	    c = *p;
	    if (c == NUL)	// cannot happen?
		break;
	}
    }

    return c;
}

/*
 * Copy the rest of the redo buffer into the stuff buffer (in a slow way).
 * If old_redo is TRUE, use old_redobuff instead of redobuff.
 * The escaped K_SPECIAL and CSI are copied without translation.
 */
    static void
copy_redo(int old_redo)
{
    int	    c;

    while ((c = read_redo(FALSE, old_redo)) != NUL)
	add_char_buff(&readbuf2, c);
}

/*
 * Stuff the redo buffer into readbuf2.
 * Insert the redo count into the command.
 * If "old_redo" is TRUE, the last but one command is repeated
 * instead of the last command (inserting text). This is used for
 * CTRL-O <.> in insert mode
 *
 * return FAIL for failure, OK otherwise
 */
    int
start_redo(long count, int old_redo)
{
    int	    c;

    // init the pointers; return if nothing to redo
    if (read_redo(TRUE, old_redo) == FAIL)
	return FAIL;

    c = read_redo(FALSE, old_redo);

#ifdef FEAT_EVAL
    if (c == K_SID)
    {
	// Copy the <SID>{sid}; sequence
	add_char_buff(&readbuf2, c);
	for (;;)
	{
	    c = read_redo(FALSE, old_redo);
	    add_char_buff(&readbuf2, c);
	    if (!SAFE_isdigit(c))
		break;
	}
	c = read_redo(FALSE, old_redo);
    }
#endif

    // copy the buffer name, if present
    if (c == '"')
    {
	add_buff(&readbuf2, (char_u *)"\"", 1L);
	c = read_redo(FALSE, old_redo);

	// if a numbered buffer is used, increment the number
	if (c >= '1' && c < '9')
	    ++c;
	add_char_buff(&readbuf2, c);

	// the expression register should be re-evaluated
	if (c == '=')
	{
	    add_char_buff(&readbuf2, CAR);
	    cmd_silent = TRUE;
	}

	c = read_redo(FALSE, old_redo);
    }

    if (c == 'v')   // redo Visual
    {
	VIsual = curwin->w_cursor;
	VIsual_active = TRUE;
	VIsual_select = FALSE;
	VIsual_reselect = TRUE;
	redo_VIsual_busy = TRUE;
	c = read_redo(FALSE, old_redo);
    }

    // try to enter the count (in place of a previous count)
    if (count)
    {
	while (VIM_ISDIGIT(c))	// skip "old" count
	    c = read_redo(FALSE, old_redo);
	add_num_buff(&readbuf2, count);
    }

    // copy the rest from the redo buffer into the stuff buffer
    add_char_buff(&readbuf2, c);
    copy_redo(old_redo);
    return OK;
}

/*
 * Repeat the last insert (R, o, O, a, A, i or I command) by stuffing
 * the redo buffer into readbuf2.
 * return FAIL for failure, OK otherwise
 */
    int
start_redo_ins(void)
{
    int	    c;

    if (read_redo(TRUE, FALSE) == FAIL)
	return FAIL;
    start_stuff();

    // skip the count and the command character
    while ((c = read_redo(FALSE, FALSE)) != NUL)
    {
	if (vim_strchr((char_u *)"AaIiRrOo", c) != NULL)
	{
	    if (c == 'O' || c == 'o')
		add_buff(&readbuf2, NL_STR, -1L);
	    break;
	}
    }

    // copy the typed text from the redo buffer into the stuff buffer
    copy_redo(FALSE);
    block_redo = TRUE;
    return OK;
}

    void
stop_redo_ins(void)
{
    block_redo = FALSE;
}

/*
 * Initialize typebuf.tb_buf to point to typebuf_init.
 * alloc() cannot be used here: In out-of-memory situations it would
 * be impossible to type anything.
 */
    static void
init_typebuf(void)
{
    if (typebuf.tb_buf != NULL)
	return;

    typebuf.tb_buf = typebuf_init;
    typebuf.tb_noremap = noremapbuf_init;
    typebuf.tb_buflen = TYPELEN_INIT;
    typebuf.tb_len = 0;
    typebuf.tb_off = MAXMAPLEN + 4;
    typebuf.tb_change_cnt = 1;
}

/*
 * Returns TRUE when keys cannot be remapped.
 */
    int
noremap_keys(void)
{
    return KeyNoremap & (RM_NONE|RM_SCRIPT);
}

/*
 * Insert a string in position 'offset' in the typeahead buffer (for "@r"
 * and ":normal" command, vgetorpeek() and check_termcode()).
 *
 * If "noremap" is REMAP_YES, new string can be mapped again.
 * If "noremap" is REMAP_NONE, new string cannot be mapped again.
 * If "noremap" is REMAP_SKIP, first char of new string cannot be mapped again,
 * but abbreviations are allowed.
 * If "noremap" is REMAP_SCRIPT, new string cannot be mapped again, except for
 *			script-local mappings.
 * If "noremap" is > 0, that many characters of the new string cannot be mapped.
 *
 * If "nottyped" is TRUE, the string does not return KeyTyped (don't use when
 * "offset" is non-zero!).
 *
 * If "silent" is TRUE, cmd_silent is set when the characters are obtained.
 *
 * return FAIL for failure, OK otherwise
 */
    int
ins_typebuf(
    char_u	*str,
    int		noremap,
    int		offset,
    int		nottyped,
    int		silent)
{
    char_u	*s1, *s2;
    int		newlen;
    int		addlen;
    int		i;
    int		newoff;
    int		val;
    int		nrm;

    init_typebuf();
    if (++typebuf.tb_change_cnt == 0)
	typebuf.tb_change_cnt = 1;
    state_no_longer_safe("ins_typebuf()");

    addlen = (int)STRLEN(str);

    if (offset == 0 && addlen <= typebuf.tb_off)
    {
	/*
	 * Easy case: there is room in front of typebuf.tb_buf[typebuf.tb_off]
	 */
	typebuf.tb_off -= addlen;
	mch_memmove(typebuf.tb_buf + typebuf.tb_off, str, (size_t)addlen);
    }
    else if (typebuf.tb_len == 0 && typebuf.tb_buflen
					       >= addlen + 3 * (MAXMAPLEN + 4))
    {
	/*
	 * Buffer is empty and string fits in the existing buffer.
	 * Leave some space before and after, if possible.
	 */
	typebuf.tb_off = (typebuf.tb_buflen - addlen - 3 * (MAXMAPLEN + 4)) / 2;
	mch_memmove(typebuf.tb_buf + typebuf.tb_off, str, (size_t)addlen);
    }
    else
    {
	int extra;

	/*
	 * Need to allocate a new buffer.
	 * In typebuf.tb_buf there must always be room for 3 * (MAXMAPLEN + 4)
	 * characters.  We add some extra room to avoid having to allocate too
	 * often.
	 */
	newoff = MAXMAPLEN + 4;
	extra = addlen + newoff + 4 * (MAXMAPLEN + 4);
	if (typebuf.tb_len > 2147483647 - extra)
	{
	    // string is getting too long for a 32 bit int
	    emsg(_(e_command_too_complex));    // also calls flush_buffers
	    setcursor();
	    return FAIL;
	}
	newlen = typebuf.tb_len + extra;
	s1 = alloc(newlen);
	if (s1 == NULL)		    // out of memory
	    return FAIL;
	s2 = alloc(newlen);
	if (s2 == NULL)		    // out of memory
	{
	    vim_free(s1);
	    return FAIL;
	}
	typebuf.tb_buflen = newlen;

	// copy the old chars, before the insertion point
	mch_memmove(s1 + newoff, typebuf.tb_buf + typebuf.tb_off,
							      (size_t)offset);
	// copy the new chars
	mch_memmove(s1 + newoff + offset, str, (size_t)addlen);
	// copy the old chars, after the insertion point, including the	NUL at
	// the end
	mch_memmove(s1 + newoff + offset + addlen,
				     typebuf.tb_buf + typebuf.tb_off + offset,
				       (size_t)(typebuf.tb_len - offset + 1));
	if (typebuf.tb_buf != typebuf_init)
	    vim_free(typebuf.tb_buf);
	typebuf.tb_buf = s1;

	mch_memmove(s2 + newoff, typebuf.tb_noremap + typebuf.tb_off,
							      (size_t)offset);
	mch_memmove(s2 + newoff + offset + addlen,
		   typebuf.tb_noremap + typebuf.tb_off + offset,
					   (size_t)(typebuf.tb_len - offset));
	if (typebuf.tb_noremap != noremapbuf_init)
	    vim_free(typebuf.tb_noremap);
	typebuf.tb_noremap = s2;

	typebuf.tb_off = newoff;
    }
    typebuf.tb_len += addlen;

    // If noremap == REMAP_SCRIPT: do remap script-local mappings.
    if (noremap == REMAP_SCRIPT)
	val = RM_SCRIPT;
    else if (noremap == REMAP_SKIP)
	val = RM_ABBR;
    else
	val = RM_NONE;

    /*
     * Adjust typebuf.tb_noremap[] for the new characters:
     * If noremap == REMAP_NONE or REMAP_SCRIPT: new characters are
     *			(sometimes) not remappable
     * If noremap == REMAP_YES: all the new characters are mappable
     * If noremap  > 0: "noremap" characters are not remappable, the rest
     *			mappable
     */
    if (noremap == REMAP_SKIP)
	nrm = 1;
    else if (noremap < 0)
	nrm = addlen;
    else
	nrm = noremap;
    for (i = 0; i < addlen; ++i)
	typebuf.tb_noremap[typebuf.tb_off + i + offset] =
						  (--nrm >= 0) ? val : RM_YES;

    // tb_maplen and tb_silent only remember the length of mapped and/or
    // silent mappings at the start of the buffer, assuming that a mapped
    // sequence doesn't result in typed characters.
    if (nottyped || typebuf.tb_maplen > offset)
	typebuf.tb_maplen += addlen;
    if (silent || typebuf.tb_silent > offset)
    {
	typebuf.tb_silent += addlen;
	cmd_silent = TRUE;
    }
    if (typebuf.tb_no_abbr_cnt && offset == 0)	// and not used for abbrev.s
	typebuf.tb_no_abbr_cnt += addlen;

    return OK;
}

/*
 * Put character "c" back into the typeahead buffer.
 * Can be used for a character obtained by vgetc() that needs to be put back.
 * Uses cmd_silent, KeyTyped and KeyNoremap to restore the flags belonging to
 * the char.
 * Returns the length of what was inserted.
 */
    int
ins_char_typebuf(int c, int modifiers)
{
    char_u	buf[MB_MAXBYTES * 3 + 4];
    int		len = special_to_buf(c, modifiers, TRUE, buf);

    buf[len] = NUL;
    (void)ins_typebuf(buf, KeyNoremap, 0, !KeyTyped, cmd_silent);
    return len;
}

/*
 * Return TRUE if the typeahead buffer was changed (while waiting for a
 * character to arrive).  Happens when a message was received from a client or
 * from feedkeys().
 * But check in a more generic way to avoid trouble: When "typebuf.tb_buf"
 * changed it was reallocated and the old pointer can no longer be used.
 * Or "typebuf.tb_off" may have been changed and we would overwrite characters
 * that was just added.
 */
    int
typebuf_changed(
    int		tb_change_cnt)	// old value of typebuf.tb_change_cnt
{
    return (tb_change_cnt != 0 && (typebuf.tb_change_cnt != tb_change_cnt
#if defined(FEAT_CLIENTSERVER) || defined(FEAT_EVAL)
	    || typebuf_was_filled
#endif
	   ));
}

/*
 * Return TRUE if there are no characters in the typeahead buffer that have
 * not been typed (result from a mapping or come from ":normal").
 */
    int
typebuf_typed(void)
{
    return typebuf.tb_maplen == 0;
}

/*
 * Return the number of characters that are mapped (or not typed).
 */
    int
typebuf_maplen(void)
{
    return typebuf.tb_maplen;
}

/*
 * remove "len" characters from typebuf.tb_buf[typebuf.tb_off + offset]
 */
    void
del_typebuf(int len, int offset)
{
    int	    i;

    if (len == 0)
	return;		// nothing to do

    typebuf.tb_len -= len;

    /*
     * Easy case: Just increase typebuf.tb_off.
     */
    if (offset == 0 && typebuf.tb_buflen - (typebuf.tb_off + len)
							 >= 3 * MAXMAPLEN + 3)
	typebuf.tb_off += len;
    /*
     * Have to move the characters in typebuf.tb_buf[] and typebuf.tb_noremap[]
     */
    else
    {
	i = typebuf.tb_off + offset;
	/*
	 * Leave some extra room at the end to avoid reallocation.
	 */
	if (typebuf.tb_off > MAXMAPLEN)
	{
	    mch_memmove(typebuf.tb_buf + MAXMAPLEN,
			     typebuf.tb_buf + typebuf.tb_off, (size_t)offset);
	    mch_memmove(typebuf.tb_noremap + MAXMAPLEN,
			 typebuf.tb_noremap + typebuf.tb_off, (size_t)offset);
	    typebuf.tb_off = MAXMAPLEN;
	}
	// adjust typebuf.tb_buf (include the NUL at the end)
	mch_memmove(typebuf.tb_buf + typebuf.tb_off + offset,
						     typebuf.tb_buf + i + len,
				       (size_t)(typebuf.tb_len - offset + 1));
	// adjust typebuf.tb_noremap[]
	mch_memmove(typebuf.tb_noremap + typebuf.tb_off + offset,
						 typebuf.tb_noremap + i + len,
					   (size_t)(typebuf.tb_len - offset));
    }

    if (typebuf.tb_maplen > offset)		// adjust tb_maplen
    {
	if (typebuf.tb_maplen < offset + len)
	    typebuf.tb_maplen = offset;
	else
	    typebuf.tb_maplen -= len;
    }
    if (typebuf.tb_silent > offset)		// adjust tb_silent
    {
	if (typebuf.tb_silent < offset + len)
	    typebuf.tb_silent = offset;
	else
	    typebuf.tb_silent -= len;
    }
    if (typebuf.tb_no_abbr_cnt > offset)	// adjust tb_no_abbr_cnt
    {
	if (typebuf.tb_no_abbr_cnt < offset + len)
	    typebuf.tb_no_abbr_cnt = offset;
	else
	    typebuf.tb_no_abbr_cnt -= len;
    }

#if defined(FEAT_CLIENTSERVER) || defined(FEAT_EVAL)
    // Reset the flag that text received from a client or from feedkeys()
    // was inserted in the typeahead buffer.
    typebuf_was_filled = FALSE;
#endif
    if (++typebuf.tb_change_cnt == 0)
	typebuf.tb_change_cnt = 1;
}

/*
 * Write typed characters to script file.
 * If recording is on put the character in the recordbuffer.
 */
    static void
gotchars(char_u *chars, int len)
{
    char_u		*s = chars;
    int			i;
    static char_u	buf[4];
    static int		buflen = 0;
    int			todo = len;

    while (todo--)
    {
	buf[buflen++] = *s++;

	// When receiving a special key sequence, store it until we have all
	// the bytes and we can decide what to do with it.
	if (buflen == 1 && buf[0] == K_SPECIAL)
	    continue;
	if (buflen == 2)
	    continue;
	if (buflen == 3 && buf[1] == KS_EXTRA
		       && (buf[2] == KE_FOCUSGAINED || buf[2] == KE_FOCUSLOST))
	{
	    // Drop K_FOCUSGAINED and K_FOCUSLOST, they are not useful in a
	    // recording.
	    buflen = 0;
	    continue;
	}

	// Handle one byte at a time; no translation to be done.
	for (i = 0; i < buflen; ++i)
	    updatescript(buf[i]);

	if (reg_recording != 0)
	{
	    buf[buflen] = NUL;
	    add_buff(&recordbuff, buf, (long)buflen);
	    // remember how many chars were last recorded
	    last_recorded_len += buflen;
	}
	buflen = 0;
    }
    may_sync_undo();

#ifdef FEAT_EVAL
    // output "debug mode" message next time in debug mode
    debug_did_msg = FALSE;
#endif

    // Since characters have been typed, consider the following to be in
    // another mapping.  Search string will be kept in history.
    ++maptick;
}

/*
 * Record an <Ignore> key.
 */
    void
gotchars_ignore(void)
{
    char_u nop_buf[3] = { K_SPECIAL, KS_EXTRA, KE_IGNORE };
    gotchars(nop_buf, 3);
}

/*
 * Undo the last gotchars() for "len" bytes.  To be used when putting a typed
 * character back into the typeahead buffer, thus gotchars() will be called
 * again.
 * Only affects recorded characters.
 */
    void
ungetchars(int len)
{
    if (reg_recording == 0)
	return;

    delete_buff_tail(&recordbuff, len);
    last_recorded_len -= len;
}

/*
 * Sync undo.  Called when typed characters are obtained from the typeahead
 * buffer, or when a menu is used.
 * Do not sync:
 * - In Insert mode, unless cursor key has been used.
 * - While reading a script file.
 * - When no_u_sync is non-zero.
 */
    static void
may_sync_undo(void)
{
    if ((!(State & (MODE_INSERT | MODE_CMDLINE)) || arrow_used)
					       && scriptin[curscript] == NULL)
	u_sync(FALSE);
}

/*
 * Make "typebuf" empty and allocate new buffers.
 * Returns FAIL when out of memory.
 */
    static int
alloc_typebuf(void)
{
    typebuf.tb_buf = alloc(TYPELEN_INIT);
    typebuf.tb_noremap = alloc(TYPELEN_INIT);
    if (typebuf.tb_buf == NULL || typebuf.tb_noremap == NULL)
    {
	free_typebuf();
	return FAIL;
    }
    typebuf.tb_buflen = TYPELEN_INIT;
    typebuf.tb_off = MAXMAPLEN + 4;  // can insert without realloc
    typebuf.tb_len = 0;
    typebuf.tb_maplen = 0;
    typebuf.tb_silent = 0;
    typebuf.tb_no_abbr_cnt = 0;
    if (++typebuf.tb_change_cnt == 0)
	typebuf.tb_change_cnt = 1;
#if defined(FEAT_CLIENTSERVER) || defined(FEAT_EVAL)
    typebuf_was_filled = FALSE;
#endif
    return OK;
}

/*
 * Free the buffers of "typebuf".
 */
    static void
free_typebuf(void)
{
    if (typebuf.tb_buf == typebuf_init)
	internal_error("Free typebuf 1");
    else
	VIM_CLEAR(typebuf.tb_buf);
    if (typebuf.tb_noremap == noremapbuf_init)
	internal_error("Free typebuf 2");
    else
	VIM_CLEAR(typebuf.tb_noremap);
}

/*
 * When doing ":so! file", the current typeahead needs to be saved, and
 * restored when "file" has been read completely.
 */
static typebuf_T saved_typebuf[NSCRIPT];

    int
save_typebuf(void)
{
    init_typebuf();
    saved_typebuf[curscript] = typebuf;
    // If out of memory: restore typebuf and close file.
    if (alloc_typebuf() == FAIL)
    {
	closescript();
	return FAIL;
    }
    return OK;
}

static int old_char = -1;	// character put back by vungetc()
static int old_mod_mask;	// mod_mask for ungotten character
static int old_mouse_row;	// mouse_row related to old_char
static int old_mouse_col;	// mouse_col related to old_char
static int old_KeyStuffed;	// whether old_char was stuffed

static int can_get_old_char(void)
{
    // If the old character was not stuffed and characters have been added to
    // the stuff buffer, need to first get the stuffed characters instead.
    return old_char != -1 && (old_KeyStuffed || stuff_empty());
}

/*
 * Save all three kinds of typeahead, so that the user must type at a prompt.
 */
    void
save_typeahead(tasave_T *tp)
{
    tp->save_typebuf = typebuf;
    tp->typebuf_valid = (alloc_typebuf() == OK);
    if (!tp->typebuf_valid)
	typebuf = tp->save_typebuf;

    tp->old_char = old_char;
    tp->old_mod_mask = old_mod_mask;
    old_char = -1;

    tp->save_readbuf1 = readbuf1;
    readbuf1.bh_first.b_next = NULL;
    tp->save_readbuf2 = readbuf2;
    readbuf2.bh_first.b_next = NULL;
# ifdef USE_INPUT_BUF
    tp->save_inputbuf = get_input_buf();
# endif
}

/*
 * Restore the typeahead to what it was before calling save_typeahead().
 * The allocated memory is freed, can only be called once!
 * When "overwrite" is FALSE input typed later is kept.
 */
    void
restore_typeahead(tasave_T *tp, int overwrite UNUSED)
{
    if (tp->typebuf_valid)
    {
	free_typebuf();
	typebuf = tp->save_typebuf;
    }

    old_char = tp->old_char;
    old_mod_mask = tp->old_mod_mask;

    free_buff(&readbuf1);
    readbuf1 = tp->save_readbuf1;
    free_buff(&readbuf2);
    readbuf2 = tp->save_readbuf2;
# ifdef USE_INPUT_BUF
    set_input_buf(tp->save_inputbuf, overwrite);
# endif
}

/*
 * Open a new script file for the ":source!" command.
 */
    void
openscript(
    char_u	*name,
    int		directly)	// when TRUE execute directly
{
    if (curscript + 1 == NSCRIPT)
    {
	emsg(_(e_scripts_nested_too_deep));
	return;
    }

    // Disallow sourcing a file in the sandbox, the commands would be executed
    // later, possibly outside of the sandbox.
    if (check_secure())
	return;

#ifdef FEAT_EVAL
    if (ignore_script)
	// Not reading from script, also don't open one.  Warning message?
	return;
#endif

    if (scriptin[curscript] != NULL)	// already reading script
	++curscript;
				// use NameBuff for expanded name
    expand_env(name, NameBuff, MAXPATHL);
    if ((scriptin[curscript] = mch_fopen((char *)NameBuff, READBIN)) == NULL)
    {
	semsg(_(e_cant_open_file_str), name);
	if (curscript)
	    --curscript;
	return;
    }
    if (save_typebuf() == FAIL)
	return;

    /*
     * Execute the commands from the file right now when using ":source!"
     * after ":global" or ":argdo" or in a loop.  Also when another command
     * follows.  This means the display won't be updated.  Don't do this
     * always, "make test" would fail.
     */
    if (directly)
    {
	oparg_T	oa;
	int	oldcurscript;
	int	save_State = State;
	int	save_restart_edit = restart_edit;
	int	save_insertmode = p_im;
	int	save_finish_op = finish_op;
	int	save_msg_scroll = msg_scroll;

	State = MODE_NORMAL;
	msg_scroll = FALSE;	// no msg scrolling in Normal mode
	restart_edit = 0;	// don't go to Insert mode
	p_im = FALSE;		// don't use 'insertmode'
	clear_oparg(&oa);
	finish_op = FALSE;

	oldcurscript = curscript;
	do
	{
	    update_topline_cursor();	// update cursor position and topline
	    normal_cmd(&oa, FALSE);	// execute one command
	    (void)vpeekc();		// check for end of file
	}
	while (scriptin[oldcurscript] != NULL);

	State = save_State;
	msg_scroll = save_msg_scroll;
	restart_edit = save_restart_edit;
	p_im = save_insertmode;
	finish_op = save_finish_op;
    }
}

/*
 * Close the currently active input script.
 */
    static void
closescript(void)
{
    free_typebuf();
    typebuf = saved_typebuf[curscript];

    fclose(scriptin[curscript]);
    scriptin[curscript] = NULL;
    if (curscript > 0)
	--curscript;
}

#if defined(EXITFREE) || defined(PROTO)
    void
close_all_scripts(void)
{
    while (scriptin[0] != NULL)
	closescript();
}
#endif

/*
 * Return TRUE when reading keys from a script file.
 */
    int
using_script(void)
{
    return scriptin[curscript] != NULL;
}

/*
 * This function is called just before doing a blocking wait.  Thus after
 * waiting 'updatetime' for a character to arrive.
 */
    void
before_blocking(void)
{
    updatescript(0);
#ifdef FEAT_EVAL
    if (may_garbage_collect)
	garbage_collect(FALSE);
#endif
}

/*
 * updatescript() is called when a character can be written into the script
 * file or when we have waited some time for a character (c == 0)
 *
 * All the changed memfiles are synced if c == 0 or when the number of typed
 * characters reaches 'updatecount' and 'updatecount' is non-zero.
 */
    static void
updatescript(int c)
{
    static int	    count = 0;

    if (c && scriptout)
	putc(c, scriptout);
    if (c == 0 || (p_uc > 0 && ++count >= p_uc))
    {
	ml_sync_all(c == 0, TRUE);
	count = 0;
    }
}

/*
 * Convert "c_arg" plus "modifiers" to merge the effect of modifyOtherKeys into
 * the character.  Also for when the Kitty key protocol is used.
 */
    int
merge_modifyOtherKeys(int c_arg, int *modifiers)
{
    int c = c_arg;

    // CTRL only uses the lower 5 bits of the character.
    if (*modifiers & MOD_MASK_CTRL)
    {
	if ((c >= '`' && c <= 0x7f) || (c >= '@' && c <= '_'))
	{
	    c &= 0x1f;
	    if (c == NUL)
		c = K_ZERO;
	}
	else if (c == '6')
	    // CTRL-6 is equivalent to CTRL-^
	    c = 0x1e;
#ifdef FEAT_GUI_GTK
	// These mappings look arbitrary at the first glance, but in fact
	// resemble quite exactly the behaviour of the GTK+ 1.2 GUI on my
	// machine.  The only difference is BS vs. DEL for CTRL-8 (makes
	// more sense and is consistent with usual terminal behaviour).
	else if (c == '2')
	    c = NUL;
	else if (c >= '3' && c <= '7')
	    c = c ^ 0x28;
	else if (c == '8')
	    c = BS;
	else if (c == '?')
	    c = DEL;
#endif
	if (c != c_arg)
	    *modifiers &= ~MOD_MASK_CTRL;
    }

    // Alt/Meta sets the 8th bit of the character.
    if ((*modifiers & (MOD_MASK_META | MOD_MASK_ALT))
	    && c >= 0 && c <= 127)
    {
	// Some terminals (esp. Kitty) do not include Shift in the character.
	// Apply it here to get consistency across terminals.  Only do ASCII
	// letters, for other characters it depends on the keyboard layout.
	if ((*modifiers & MOD_MASK_SHIFT) && c >= 'a' && c <= 'z')
	{
	    c += 'a' - 'A';
	    *modifiers &= ~MOD_MASK_SHIFT;
	}
	c += 0x80;
	*modifiers &= ~(MOD_MASK_META | MOD_MASK_ALT);
    }

    return c;
}

/*
 * Get the next input character.
 * Can return a special key or a multi-byte character.
 * Can return NUL when called recursively, use safe_vgetc() if that's not
 * wanted.
 * This translates escaped K_SPECIAL and CSI bytes to a K_SPECIAL or CSI byte.
 * Collects the bytes of a multibyte character into the whole character.
 * Returns the modifiers in the global "mod_mask".
 */
    int
vgetc(void)
{
    int		c, c2;
    int		n;
    char_u	buf[MB_MAXBYTES + 1];
    int		i;

#ifdef FEAT_EVAL
    // Do garbage collection when garbagecollect() was called previously and
    // we are now at the toplevel.
    if (may_garbage_collect && want_garbage_collect)
	garbage_collect(FALSE);
#endif

    /*
     * If a character was put back with vungetc, it was already processed.
     * Return it directly.
     */
    if (can_get_old_char())
    {
	c = old_char;
	old_char = -1;
	mod_mask = old_mod_mask;
	mouse_row = old_mouse_row;
	mouse_col = old_mouse_col;
    }
    else
    {
	// number of characters recorded from the last vgetc() call
	static int	last_vgetc_recorded_len = 0;

	mod_mask = 0;
	vgetc_mod_mask = 0;
	vgetc_char = 0;

	// last_recorded_len can be larger than last_vgetc_recorded_len
	// if peeking records more
	last_recorded_len -= last_vgetc_recorded_len;

	for (;;)		// this is done twice if there are modifiers
	{
	    int did_inc = FALSE;

	    // No mapping after modifier has been read, using an input method
	    // and when a popup window has disabled mapping.
	    if (mod_mask
#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
		    || im_is_preediting()
#endif
#if defined(FEAT_PROP_POPUP)
		    || popup_no_mapping()
#endif
		    )
	    {
		++no_mapping;
		++allow_keys;
		// mod_mask value may change, remember we did the increment
		did_inc = TRUE;
	    }
	    c = vgetorpeek(TRUE);
	    if (did_inc)
	    {
		--no_mapping;
		--allow_keys;
	    }

	    // Get two extra bytes for special keys, handle modifiers.
	    if (c == K_SPECIAL
#ifdef FEAT_GUI
		    || c == CSI
#endif
	       )
	    {
		int	    save_allow_keys = allow_keys;

		++no_mapping;
		allow_keys = 0;		// make sure BS is not found
		c2 = vgetorpeek(TRUE);	// no mapping for these chars
		c = vgetorpeek(TRUE);
		--no_mapping;
		allow_keys = save_allow_keys;
		if (c2 == KS_MODIFIER)
		{
		    mod_mask = c;
		    continue;
		}
		c = TO_SPECIAL(c2, c);

		// K_ESC is used to avoid ambiguity with the single Esc
		// character that might be the start of an escape sequence.
		// Convert it back to a single Esc here.
		if (c == K_ESC)
		    c = ESC;

#if defined(FEAT_GUI_MSWIN) && defined(FEAT_MENU) && defined(FEAT_TEAROFF)
		// Handle K_TEAROFF here, the caller of vgetc() doesn't need to
		// know that a menu was torn off
		if (
# ifdef VIMDLL
		    gui.in_use &&
# endif
		    c == K_TEAROFF)
		{
		    char_u	name[200];
		    int		j;

		    // get menu path, it ends with a <CR>
		    for (j = 0; (c = vgetorpeek(TRUE)) != '\r'; )
		    {
			name[j] = c;
			if (j < 199)
			    ++j;
		    }
		    name[j] = NUL;
		    gui_make_tearoff(name);
		    continue;
		}
#endif
#if defined(FEAT_GUI) && defined(FEAT_GUI_GTK) && defined(FEAT_MENU)
		// GTK: <F10> normally selects the menu, but it's passed until
		// here to allow mapping it.  Intercept and invoke the GTK
		// behavior if it's not mapped.
		if (c == K_F10 && gui.menubar != NULL)
		{
		    gtk_menu_shell_select_first(
					   GTK_MENU_SHELL(gui.menubar), FALSE);
		    continue;
		}
#endif
#ifdef FEAT_GUI
		// Handle focus event here, so that the caller doesn't need to
		// know about it.  Return K_IGNORE so that we loop once (needed
		// if 'lazyredraw' is set).
		if (c == K_FOCUSGAINED || c == K_FOCUSLOST)
		{
		    ui_focus_change(c == K_FOCUSGAINED);
		    c = K_IGNORE;
		}

		// Translate K_CSI to CSI.  The special key is only used to
		// avoid it being recognized as the start of a special key.
		if (c == K_CSI)
		    c = CSI;
#endif
#ifdef FEAT_EVAL
		if (c == K_SID)
		{
		    int	    j;

		    // Handle <SID>{sid};  Do up to 20 digits for safety.
		    last_used_sid = 0;
		    for (j = 0; j < 20 && SAFE_isdigit(c = vgetorpeek(TRUE)); ++j)
			last_used_sid = last_used_sid * 10 + (c - '0');
		    last_used_map = NULL;
		    continue;
		}
#endif
	    }

	    // a keypad or special function key was not mapped, use it like
	    // its ASCII equivalent
	    switch (c)
	    {
		case K_KPLUS:	c = '+'; break;
		case K_KMINUS:	c = '-'; break;
		case K_KDIVIDE:	c = '/'; break;
		case K_KMULTIPLY: c = '*'; break;
		case K_KENTER:	c = CAR; break;
		case K_KPOINT:
#ifdef MSWIN
				// Can be either '.' or a ',',
				// depending on the type of keypad.
				c = MapVirtualKey(VK_DECIMAL, 2); break;
#else
				c = '.'; break;
#endif
		case K_K0:	c = '0'; break;
		case K_K1:	c = '1'; break;
		case K_K2:	c = '2'; break;
		case K_K3:	c = '3'; break;
		case K_K4:	c = '4'; break;
		case K_K5:	c = '5'; break;
		case K_K6:	c = '6'; break;
		case K_K7:	c = '7'; break;
		case K_K8:	c = '8'; break;
		case K_K9:	c = '9'; break;

		case K_XHOME:
		case K_ZHOME:	if (mod_mask == MOD_MASK_SHIFT)
				{
				    c = K_S_HOME;
				    mod_mask = 0;
				}
				else if (mod_mask == MOD_MASK_CTRL)
				{
				    c = K_C_HOME;
				    mod_mask = 0;
				}
				else
				    c = K_HOME;
				break;
		case K_XEND:
		case K_ZEND:	if (mod_mask == MOD_MASK_SHIFT)
				{
				    c = K_S_END;
				    mod_mask = 0;
				}
				else if (mod_mask == MOD_MASK_CTRL)
				{
				    c = K_C_END;
				    mod_mask = 0;
				}
				else
				    c = K_END;
				break;

		case K_XUP:	c = K_UP; break;
		case K_XDOWN:	c = K_DOWN; break;
		case K_XLEFT:	c = K_LEFT; break;
		case K_XRIGHT:	c = K_RIGHT; break;
	    }

	    // For a multi-byte character get all the bytes and return the
	    // converted character.
	    // Note: This will loop until enough bytes are received!
	    if (has_mbyte && (n = MB_BYTE2LEN_CHECK(c)) > 1)
	    {
		++no_mapping;
		buf[0] = c;
		for (i = 1; i < n; ++i)
		{
		    buf[i] = vgetorpeek(TRUE);
		    if (buf[i] == K_SPECIAL
#ifdef FEAT_GUI
			    || (buf[i] == CSI)
#endif
			    )
		    {
			// Must be a K_SPECIAL - KS_SPECIAL - KE_FILLER
			// sequence, which represents a K_SPECIAL (0x80),
			// or a CSI - KS_EXTRA - KE_CSI sequence, which
			// represents a CSI (0x9B),
			// or a K_SPECIAL - KS_EXTRA - KE_CSI, which is CSI
			// too.
			c = vgetorpeek(TRUE);
			if (vgetorpeek(TRUE) == KE_CSI && c == KS_EXTRA)
			    buf[i] = CSI;
		    }
		}
		--no_mapping;
		c = (*mb_ptr2char)(buf);
	    }

	    if (vgetc_char == 0)
	    {
		vgetc_mod_mask = mod_mask;
		vgetc_char = c;
	    }

	    break;
	}

	last_vgetc_recorded_len = last_recorded_len;
    }

#ifdef FEAT_EVAL
    /*
     * In the main loop "may_garbage_collect" can be set to do garbage
     * collection in the first next vgetc().  It's disabled after that to
     * avoid internally used Lists and Dicts to be freed.
     */
    may_garbage_collect = FALSE;
#endif

#ifdef FEAT_BEVAL_TERM
    if (c != K_MOUSEMOVE && c != K_IGNORE && c != K_CURSORHOLD)
    {
	// Don't trigger 'balloonexpr' unless only the mouse was moved.
	bevalexpr_due_set = FALSE;
	ui_remove_balloon();
    }
#endif
#ifdef FEAT_PROP_POPUP
    // Only filter keys that do not come from ":normal".  Keys from feedkeys()
    // are filtered.
    if ((!ex_normal_busy || in_feedkeys) && popup_do_filter(c))
    {
	if (c == Ctrl_C)
	    got_int = FALSE;  // avoid looping
	c = K_IGNORE;
    }
#endif

    // Need to process the character before we know it's safe to do something
    // else.
    if (c != K_IGNORE)
	state_no_longer_safe("key typed");

    return c;
}

/*
 * Like vgetc(), but never return a NUL when called recursively, get a key
 * directly from the user (ignoring typeahead).
 */
    int
safe_vgetc(void)
{
    int	c;

    c = vgetc();
    if (c == NUL)
	c = get_keystroke();
    return c;
}

/*
 * Like safe_vgetc(), but loop to handle K_IGNORE.
 * Also ignore scrollbar events.
 * Does not handle bracketed paste - do not use the result for commands.
 */
    static int
plain_vgetc_nopaste(void)
{
    int c;

    do
	c = safe_vgetc();
    while (c == K_IGNORE
	    || c == K_VER_SCROLLBAR || c == K_HOR_SCROLLBAR
	    || c == K_MOUSEMOVE);
    return c;
}

/*
 * Like safe_vgetc(), but loop to handle K_IGNORE.
 * Also ignore scrollbar events.
 */
    int
plain_vgetc(void)
{
    int c = plain_vgetc_nopaste();

    if (c == K_PS)
	// Only handle the first pasted character.  Drop the rest, since we
	// don't know what to do with it.
	c = bracketed_paste(PASTE_ONE_CHAR, FALSE, NULL);

    return c;
}

/*
 * Check if a character is available, such that vgetc() will not block.
 * If the next character is a special character or multi-byte, the returned
 * character is not valid!.
 * Returns NUL if no character is available.
 */
    int
vpeekc(void)
{
    if (can_get_old_char())
	return old_char;
    return vgetorpeek(FALSE);
}

#if defined(FEAT_TERMRESPONSE) || defined(FEAT_TERMINAL) || defined(PROTO)
/*
 * Like vpeekc(), but don't allow mapping.  Do allow checking for terminal
 * codes.
 */
    int
vpeekc_nomap(void)
{
    int		c;

    ++no_mapping;
    ++allow_keys;
    c = vpeekc();
    --no_mapping;
    --allow_keys;
    return c;
}
#endif

/*
 * Check if any character is available, also half an escape sequence.
 * Trick: when no typeahead found, but there is something in the typeahead
 * buffer, it must be an ESC that is recognized as the start of a key code.
 */
    int
vpeekc_any(void)
{
    int		c;

    c = vpeekc();
    if (c == NUL && typebuf.tb_len > 0)
	c = ESC;
    return c;
}

/*
 * Call vpeekc() without causing anything to be mapped.
 * Return TRUE if a character is available, FALSE otherwise.
 */
    int
char_avail(void)
{
    int	    retval;

#ifdef FEAT_EVAL
    // When test_override("char_avail", 1) was called pretend there is no
    // typeahead.
    if (disable_char_avail_for_testing)
	return FALSE;
#endif
    ++no_mapping;
    retval = vpeekc();
    --no_mapping;
    return (retval != NUL);
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * "getchar()" and "getcharstr()" functions
 */
    static void
getchar_common(typval_T *argvars, typval_T *rettv)
{
    varnumber_T		n;
    int			error = FALSE;

    if (in_vim9script() && check_for_opt_bool_arg(argvars, 0) == FAIL)
	return;

#ifdef MESSAGE_QUEUE
    // vpeekc() used to check for messages, but that caused problems, invoking
    // a callback where it was not expected.  Some plugins use getchar(1) in a
    // loop to await a message, therefore make sure we check for messages here.
    parse_queued_messages();
#endif

    // Position the cursor.  Needed after a message that ends in a space.
    windgoto(msg_row, msg_col);

    ++no_mapping;
    ++allow_keys;
    for (;;)
    {
	if (argvars[0].v_type == VAR_UNKNOWN)
	    // getchar(): blocking wait.
	    n = plain_vgetc_nopaste();
	else if (tv_get_bool_chk(&argvars[0], &error))
	    // getchar(1): only check if char avail
	    n = vpeekc_any();
	else if (error || vpeekc_any() == NUL)
	    // illegal argument or getchar(0) and no char avail: return zero
	    n = 0;
	else
	    // getchar(0) and char avail() != NUL: get a character.
	    // Note that vpeekc_any() returns K_SPECIAL for K_IGNORE.
	    n = safe_vgetc();

	if (n == K_IGNORE || n == K_MOUSEMOVE
		|| n == K_VER_SCROLLBAR || n == K_HOR_SCROLLBAR)
	    continue;
	break;
    }
    --no_mapping;
    --allow_keys;

    set_vim_var_nr(VV_MOUSE_WIN, 0);
    set_vim_var_nr(VV_MOUSE_WINID, 0);
    set_vim_var_nr(VV_MOUSE_LNUM, 0);
    set_vim_var_nr(VV_MOUSE_COL, 0);

    rettv->vval.v_number = n;
    if (n != 0 && (IS_SPECIAL(n) || mod_mask != 0))
    {
	char_u		temp[10];   // modifier: 3, mbyte-char: 6, NUL: 1
	int		i = 0;

	// Turn a special key into three bytes, plus modifier.
	if (mod_mask != 0)
	{
	    temp[i++] = K_SPECIAL;
	    temp[i++] = KS_MODIFIER;
	    temp[i++] = mod_mask;
	}
	if (IS_SPECIAL(n))
	{
	    temp[i++] = K_SPECIAL;
	    temp[i++] = K_SECOND(n);
	    temp[i++] = K_THIRD(n);
	}
	else if (has_mbyte)
	    i += (*mb_char2bytes)(n, temp + i);
	else
	    temp[i++] = n;
	temp[i++] = NUL;
	rettv->v_type = VAR_STRING;
	rettv->vval.v_string = vim_strsave(temp);

	if (is_mouse_key(n))
	{
	    int		row = mouse_row;
	    int		col = mouse_col;
	    win_T	*win;
	    linenr_T	lnum;
	    win_T	*wp;
	    int		winnr = 1;

	    if (row >= 0 && col >= 0)
	    {
		// Find the window at the mouse coordinates and compute the
		// text position.
		win = mouse_find_win(&row, &col, FIND_POPUP);
		if (win == NULL)
		    return;
		(void)mouse_comp_pos(win, &row, &col, &lnum, NULL);
#ifdef FEAT_PROP_POPUP
		if (WIN_IS_POPUP(win))
		    winnr = 0;
		else
#endif
		    for (wp = firstwin; wp != win && wp != NULL;
							       wp = wp->w_next)
			++winnr;
		set_vim_var_nr(VV_MOUSE_WIN, winnr);
		set_vim_var_nr(VV_MOUSE_WINID, win->w_id);
		set_vim_var_nr(VV_MOUSE_LNUM, lnum);
		set_vim_var_nr(VV_MOUSE_COL, col + 1);
	    }
	}
    }
}

/*
 * "getchar()" function
 */
    void
f_getchar(typval_T *argvars, typval_T *rettv)
{
    getchar_common(argvars, rettv);
}

/*
 * "getcharstr()" function
 */
    void
f_getcharstr(typval_T *argvars, typval_T *rettv)
{
    getchar_common(argvars, rettv);

    if (rettv->v_type != VAR_NUMBER)
	return;

    char_u		temp[7];   // mbyte-char: 6, NUL: 1
    varnumber_T	n = rettv->vval.v_number;
    int		i = 0;

    if (n != 0)
    {
	if (has_mbyte)
	    i += (*mb_char2bytes)(n, temp + i);
	else
	    temp[i++] = n;
    }
    temp[i++] = NUL;
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = vim_strsave(temp);
}

/*
 * "getcharmod()" function
 */
    void
f_getcharmod(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->vval.v_number = mod_mask;
}
#endif // FEAT_EVAL

#if defined(MESSAGE_QUEUE) || defined(PROTO)
# define MAX_REPEAT_PARSE 8

/*
 * Process messages that have been queued for netbeans or clientserver.
 * Also check if any jobs have ended.
 * These functions can call arbitrary Vim script and should only be called when
 * it is safe to do so.
 */
    void
parse_queued_messages(void)
{
    int	    old_curwin_id;
    int	    old_curbuf_fnum;
    int	    i;
    int	    save_may_garbage_collect = may_garbage_collect;
    static int entered = 0;
    int	    was_safe = get_was_safe_state();

    // Do not handle messages while redrawing, because it may cause buffers to
    // change or be wiped while they are being redrawn.
    // Also bail out when parsing messages was explicitly disabled.
    if (updating_screen || dont_parse_messages)
	return;

    // If memory allocation fails during startup we'll exit but curbuf or
    // curwin could be NULL.
    if (curbuf == NULL || curwin == NULL)
       return;

    old_curbuf_fnum = curbuf->b_fnum;
    old_curwin_id = curwin->w_id;

    ++entered;

    // may_garbage_collect is set in main_loop() to do garbage collection when
    // blocking to wait on a character.  We don't want that while parsing
    // messages, a callback may invoke vgetc() while lists and dicts are in use
    // in the call stack.
    may_garbage_collect = FALSE;

    // Loop when a job ended, but don't keep looping forever.
    for (i = 0; i < MAX_REPEAT_PARSE; ++i)
    {
	// For Win32 mch_breakcheck() does not check for input, do it here.
# if (defined(MSWIN) || defined(__HAIKU__)) && defined(FEAT_JOB_CHANNEL)
	channel_handle_events(FALSE);
# endif

# ifdef FEAT_NETBEANS_INTG
	// Process the queued netbeans messages.
	netbeans_parse_messages();
# endif
# ifdef FEAT_JOB_CHANNEL
	// Write any buffer lines still to be written.
	channel_write_any_lines();

	// Process the messages queued on channels.
	channel_parse_messages();
# endif
# if defined(FEAT_CLIENTSERVER) && defined(FEAT_X11)
	// Process the queued clientserver messages.
	server_parse_messages();
# endif
# ifdef FEAT_JOB_CHANNEL
	// Check if any jobs have ended.  If so, repeat the above to handle
	// changes, e.g. stdin may have been closed.
	if (job_check_ended())
	    continue;
# endif
# ifdef FEAT_TERMINAL
	free_unused_terminals();
# endif

# ifdef FEAT_SOUND_MACOSX
	process_cfrunloop();
# endif
# ifdef FEAT_SOUND_CANBERRA
	if (has_sound_callback_in_queue())
	    invoke_sound_callback();
# endif
#ifdef SIGUSR1
	if (got_sigusr1)
	{
	    apply_autocmds(EVENT_SIGUSR1, NULL, NULL, FALSE, curbuf);
	    got_sigusr1 = FALSE;
	}
#endif
	break;
    }

    // When not nested we'll go back to waiting for a typed character.  If it
    // was safe before then this triggers a SafeStateAgain autocommand event.
    if (entered == 1 && was_safe)
	may_trigger_safestateagain();

    may_garbage_collect = save_may_garbage_collect;

    // If the current window or buffer changed we need to bail out of the
    // waiting loop.  E.g. when a job exit callback closes the terminal window.
    if (curwin->w_id != old_curwin_id || curbuf->b_fnum != old_curbuf_fnum)
	ins_char_typebuf(K_IGNORE, 0);

    --entered;
}
#endif


typedef enum {
    map_result_fail,    // failed, break loop
    map_result_get,     // get a character from typeahead
    map_result_retry,   // try to map again
    map_result_nomatch  // no matching mapping, get char
} map_result_T;

/*
 * Check if the bytes at the start of the typeahead buffer are a character used
 * in Insert mode completion.  This includes the form with a CTRL modifier.
 */
    static int
at_ins_compl_key(void)
{
    char_u  *p = typebuf.tb_buf + typebuf.tb_off;
    int	    c = *p;

    if (typebuf.tb_len > 3
	    && (c == K_SPECIAL || c == CSI)  // CSI is used by the GUI
	    && p[1] == KS_MODIFIER
	    && (p[2] & MOD_MASK_CTRL))
	c = p[3] & 0x1f;
    return (ctrl_x_mode_not_default() && vim_is_ctrl_x_key(c))
		|| (compl_status_local() && (c == Ctrl_N || c == Ctrl_P));
}

/*
 * Check if typebuf.tb_buf[] contains a modifier plus key that can be changed
 * into just a key, apply that.
 * Check from typebuf.tb_buf[typebuf.tb_off] to typebuf.tb_buf[typebuf.tb_off
 * + "max_offset"].
 * Return the length of the replaced bytes, 0 if nothing changed, -1 for error.
 */
    static int
check_simplify_modifier(int max_offset)
{
    int		offset;
    char_u	*tp;

    for (offset = 0; offset < max_offset; ++offset)
    {
	if (offset + 3 >= typebuf.tb_len)
	    break;
	tp = typebuf.tb_buf + typebuf.tb_off + offset;
	if ((tp[0] == K_SPECIAL || tp[0] == CSI) && tp[1] == KS_MODIFIER)
	{
	    // A modifier was not used for a mapping, apply it to ASCII keys.
	    // Shift would already have been applied.
	    int modifier = tp[2];
	    int	c = tp[3];
	    int new_c = merge_modifyOtherKeys(c, &modifier);

	    if (new_c != c)
	    {
		char_u	new_string[MB_MAXBYTES];
		int	len;

		if (offset == 0)
		{
		    // At the start: remember the character and mod_mask before
		    // merging, in some cases, e.g. at the hit-return prompt,
		    // they are put back in the typeahead buffer.
		    vgetc_char = c;
		    vgetc_mod_mask = tp[2];
		}
		if (IS_SPECIAL(new_c))
		{
		    new_string[0] = K_SPECIAL;
		    new_string[1] = K_SECOND(new_c);
		    new_string[2] = K_THIRD(new_c);
		    len = 3;
		}
		else
		    len = mb_char2bytes(new_c, new_string);
		if (modifier == 0)
		{
		    if (put_string_in_typebuf(offset, 4, new_string, len,
							NULL, 0, NULL) == FAIL)
			return -1;
		}
		else
		{
		    tp[2] = modifier;
		    if (put_string_in_typebuf(offset + 3, 1, new_string, len,
							NULL, 0, NULL) == FAIL)
			return -1;
		}
		return len;
	    }
	}
    }
    return 0;
}

/*
 * Return TRUE if the terminal sends modifiers with various keys.  This is when
 * modifyOtherKeys level 2 is enabled or the kitty keyboard protocol is
 * enabled.
 */
    int
key_protocol_enabled(void)
{
    // If xterm has responded to XTQMODKEYS it overrules seenModifyOtherKeys.
    int using_mok = modify_otherkeys_state != MOKS_INITIAL
			? modify_otherkeys_state == MOKS_ENABLED
			: seenModifyOtherKeys;
    return using_mok || kitty_protocol_state == KKPS_ENABLED;
}

/*
 * Handle mappings in the typeahead buffer.
 * - When something was mapped, return map_result_retry for recursive mappings.
 * - When nothing mapped and typeahead has a character: return map_result_get.
 * - When there is no match yet, return map_result_nomatch, need to get more
 *   typeahead.
 * - On failure (out of memory) return map_result_fail.
 */
    static int
handle_mapping(
	    int *keylenp,
	    int *timedout,
	    int *mapdepth)
{
    mapblock_T	*mp = NULL;
    mapblock_T	*mp2;
    mapblock_T	*mp_match;
    int		mp_match_len = 0;
    int		max_mlen = 0;
    int		want_termcode = 0;  // 1 if termcode expected after max_mlen
    int		tb_c1;
    int		mlen;
#ifdef FEAT_LANGMAP
    int		nolmaplen;
#endif
    int		keylen = *keylenp;
    int		i;
    int		local_State = get_real_state();
    int		is_plug_map = FALSE;

    // If typeahead starts with <Plug> then remap, even for a "noremap" mapping.
    if (typebuf.tb_len >= 3
	    && typebuf.tb_buf[typebuf.tb_off] == K_SPECIAL
	    && typebuf.tb_buf[typebuf.tb_off + 1] == KS_EXTRA
	    && typebuf.tb_buf[typebuf.tb_off + 2] == KE_PLUG)
	is_plug_map = TRUE;

    /*
     * Check for a mappable key sequence.
     * Walk through one maphash[] list until we find an entry that matches.
     *
     * Don't look for mappings if:
     * - no_mapping set: mapping disabled (e.g. for CTRL-V)
     * - maphash_valid not set: no mappings present.
     * - typebuf.tb_buf[typebuf.tb_off] should not be remapped
     * - in insert or cmdline mode and 'paste' option set
     * - waiting for "hit return to continue" and CR or SPACE typed
     * - waiting for a char with --more--
     * - in Ctrl-X mode, and we get a valid char for that mode
     */
    tb_c1 = typebuf.tb_buf[typebuf.tb_off];
    if (no_mapping == 0 && is_maphash_valid()
	    && (no_zero_mapping == 0 || tb_c1 != '0')
	    && (typebuf.tb_maplen == 0 || is_plug_map
		|| (p_remap
		    && (typebuf.tb_noremap[typebuf.tb_off]
				    & (RM_NONE|RM_ABBR)) == 0))
	    && !(p_paste && (State & (MODE_INSERT | MODE_CMDLINE)))
	    && !(State == MODE_HITRETURN && (tb_c1 == CAR || tb_c1 == ' '))
	    && State != MODE_ASKMORE
	    && State != MODE_CONFIRM
	    && !at_ins_compl_key())
    {
#ifdef FEAT_GUI
	if (gui.in_use && tb_c1 == CSI && typebuf.tb_len >= 2
		&& typebuf.tb_buf[typebuf.tb_off + 1] == KS_MODIFIER)
	{
	    // The GUI code sends CSI KS_MODIFIER {flags}, but mappings expect
	    // K_SPECIAL KS_MODIFIER {flags}.
	    tb_c1 = K_SPECIAL;
	}
#endif
#ifdef FEAT_LANGMAP
	if (tb_c1 == K_SPECIAL)
	    nolmaplen = 2;
	else
	{
	    LANGMAP_ADJUST(tb_c1, (State & (MODE_CMDLINE | MODE_INSERT)) == 0
					   && get_real_state() != MODE_SELECT);
	    nolmaplen = 0;
	}
#endif
	// First try buffer-local mappings.
	mp = get_buf_maphash_list(local_State, tb_c1);
	mp2 = get_maphash_list(local_State, tb_c1);
	if (mp == NULL)
	{
	    // There are no buffer-local mappings.
	    mp = mp2;
	    mp2 = NULL;
	}

	/*
	 * Loop until a partly matching mapping is found or all (local)
	 * mappings have been checked.
	 * The longest full match is remembered in "mp_match".
	 * A full match is only accepted if there is no partly match, so "aa"
	 * and "aaa" can both be mapped.
	 */
	mp_match = NULL;
	mp_match_len = 0;
	for ( ; mp != NULL;
	       mp->m_next == NULL ? (mp = mp2, mp2 = NULL) : (mp = mp->m_next))
	{
	    // Only consider an entry if the first character matches and it is
	    // for the current state.
	    // Skip ":lmap" mappings if keys were mapped.
	    if (mp->m_keys[0] == tb_c1
		    && (mp->m_mode & local_State)
		    && !(mp->m_simplified && key_protocol_enabled()
						     && typebuf.tb_maplen == 0)
		    && ((mp->m_mode & MODE_LANGMAP) == 0
						    || typebuf.tb_maplen == 0))
	    {
#ifdef FEAT_LANGMAP
		int	nomap = nolmaplen;
		int	modifiers = 0;
#endif
		// find the match length of this mapping
		for (mlen = 1; mlen < typebuf.tb_len; ++mlen)
		{
		    int	c2 = typebuf.tb_buf[typebuf.tb_off + mlen];
#ifdef FEAT_LANGMAP
		    if (nomap > 0)
		    {
			if (nomap == 2 && c2 == KS_MODIFIER)
			    modifiers = 1;
			else if (nomap == 1 && modifiers == 1)
			    modifiers = c2;
			--nomap;
		    }
		    else
		    {
			if (c2 == K_SPECIAL)
			    nomap = 2;
			else if (merge_modifyOtherKeys(c2, &modifiers) == c2)
			    // Only apply 'langmap' if merging modifiers into
			    // the key will not result in another character,
			    // so that 'langmap' behaves consistently in
			    // different terminals and GUIs.
			    LANGMAP_ADJUST(c2, TRUE);
			modifiers = 0;
		    }
#endif
		    if (mp->m_keys[mlen] != c2)
			break;
		}

		// Don't allow mapping the first byte(s) of a multi-byte char.
		// Happens when mapping <M-a> and then changing 'encoding'.
		// Beware that 0x80 is escaped.
		{
		    char_u *p1 = mp->m_keys;
		    char_u *p2 = mb_unescape(&p1);

		    if (has_mbyte && p2 != NULL
					&& MB_BYTE2LEN(tb_c1) > mb_ptr2len(p2))
			mlen = 0;
		}

		// Check an entry whether it matches.
		// - Full match: mlen == keylen
		// - Partly match: mlen == typebuf.tb_len
		keylen = mp->m_keylen;
		if (mlen == keylen || (mlen == typebuf.tb_len
						   && typebuf.tb_len < keylen))
		{
		    char_u  *s;
		    int	    n;

		    // If only script-local mappings are allowed, check if the
		    // mapping starts with K_SNR.
		    s = typebuf.tb_noremap + typebuf.tb_off;
		    if (*s == RM_SCRIPT
			    && (mp->m_keys[0] != K_SPECIAL
				|| mp->m_keys[1] != KS_EXTRA
				|| mp->m_keys[2] != KE_SNR))
			continue;

		    // If one of the typed keys cannot be remapped, skip the
		    // entry.
		    for (n = mlen; --n >= 0; )
			if (*s++ & (RM_NONE|RM_ABBR))
			    break;
		    if (!is_plug_map && n >= 0)
			continue;

		    if (keylen > typebuf.tb_len)
		    {
			if (!*timedout && !(mp_match != NULL
							&& mp_match->m_nowait))
			{
			    // break at a partly match
			    keylen = KEYLEN_PART_MAP;
			    break;
			}
		    }
		    else if (keylen > mp_match_len)
		    {
			// found a longer match
			mp_match = mp;
			mp_match_len = keylen;
		    }
		}
		else
		    // No match; may have to check for termcode at next
		    // character.  If the first character that didn't match is
		    // K_SPECIAL then check for a termcode.  This isn't perfect
		    // but should work in most cases.
		    if (max_mlen < mlen)
		    {
			max_mlen = mlen;
			want_termcode = mp->m_keys[mlen] == K_SPECIAL;
		    }
		    else if (max_mlen == mlen && mp->m_keys[mlen] == K_SPECIAL)
			want_termcode = 1;
	    }
	}

	// If no partly match found, use the longest full match.
	if (keylen != KEYLEN_PART_MAP && mp_match != NULL)
	{
	    mp = mp_match;
	    keylen = mp_match_len;
	}
    }

    /*
     * Check for match with 'pastetoggle'
     */
    if (*p_pt != NUL && mp == NULL && (State & (MODE_INSERT | MODE_NORMAL)))
    {
	for (mlen = 0; mlen < typebuf.tb_len && p_pt[mlen]; ++mlen)
	    if (p_pt[mlen] != typebuf.tb_buf[typebuf.tb_off + mlen])
		    break;
	if (p_pt[mlen] == NUL)	// match
	{
	    // write chars to script file(s)
	    if (mlen > typebuf.tb_maplen)
		gotchars(typebuf.tb_buf + typebuf.tb_off + typebuf.tb_maplen,
						     mlen - typebuf.tb_maplen);

	    del_typebuf(mlen, 0); // remove the chars
	    set_option_value_give_err((char_u *)"paste",
						      (long)!p_paste, NULL, 0);
	    if (!(State & MODE_INSERT))
	    {
		msg_col = 0;
		msg_row = Rows - 1;
		msg_clr_eos();		// clear ruler
	    }
	    status_redraw_all();
	    redraw_statuslines();
	    showmode();
	    setcursor();
	    *keylenp = keylen;
	    return map_result_retry;
	}
	// Need more chars for partly match.
	if (mlen == typebuf.tb_len)
	    keylen = KEYLEN_PART_KEY;
	else if (max_mlen < mlen)
	    // no match, may have to check for termcode at next character
	    max_mlen = mlen + 1;
    }

    // May check for a terminal code when there is no mapping or only a partial
    // mapping.  Also check if there is a full mapping with <Esc>, unless timed
    // out, since that is nearly always a partial match with a terminal code.
    if ((mp == NULL || max_mlen + want_termcode > mp_match_len
		    || (mp_match_len == 1 && *mp->m_keys == ESC && !*timedout))
	    && keylen != KEYLEN_PART_MAP)
    {
	int	save_keylen = keylen;

	/*
	 * When no matching mapping found or found a non-matching mapping that
	 * matches at least what the matching mapping matched:
	 * Check if we have a terminal code, when:
	 * - mapping is allowed,
	 * - keys have not been mapped,
	 * - and not an ESC sequence, not in insert mode or p_ek is on,
	 * - and when not timed out,
	 */
	if (no_mapping == 0 || allow_keys != 0)
	{
	    if ((typebuf.tb_maplen == 0
			    || (p_remap && typebuf.tb_noremap[
						    typebuf.tb_off] == RM_YES))
		    && !*timedout)
		keylen = check_termcode(max_mlen + 1, NULL, 0, NULL);
	    else
		keylen = 0;

	    // If no termcode matched but 'pastetoggle' matched partially
	    // it's like an incomplete key sequence.
	    if (keylen == 0 && save_keylen == KEYLEN_PART_KEY && !*timedout)
		keylen = KEYLEN_PART_KEY;

	    // If no termcode matched, try to include the modifier into the
	    // key.  This is for when modifyOtherKeys is working.
#ifdef FEAT_TERMINAL
	    check_no_reduce_keys();  // may update the no_reduce_keys flag
#endif
	    if (keylen == 0 && !no_reduce_keys)
	    {
		keylen = check_simplify_modifier(max_mlen + 1);
		if (keylen < 0)
		    // ins_typebuf() failed
		    return map_result_fail;
	    }

	    // When getting a partial match, but the last characters were not
	    // typed, don't wait for a typed character to complete the
	    // termcode.  This helps a lot when a ":normal" command ends in an
	    // ESC.
	    if (keylen < 0 && typebuf.tb_len == typebuf.tb_maplen)
		keylen = 0;
	}
	else
	    keylen = 0;
	if (keylen == 0)	// no matching terminal code
	{
#ifdef AMIGA
	    // check for window bounds report
	    if (typebuf.tb_maplen == 0 && (typebuf.tb_buf[
						typebuf.tb_off] & 0xff) == CSI)
	    {
		char_u *s;

		for (s = typebuf.tb_buf + typebuf.tb_off + 1;
			   s < typebuf.tb_buf + typebuf.tb_off + typebuf.tb_len
		   && (VIM_ISDIGIT(*s) || *s == ';' || *s == ' ');
			++s)
		    ;
		if (*s == 'r' || *s == '|') // found one
		{
		    del_typebuf(
			  (int)(s + 1 - (typebuf.tb_buf + typebuf.tb_off)), 0);
		    // get size and redraw screen
		    shell_resized();
		    *keylenp = keylen;
		    return map_result_retry;
		}
		if (*s == NUL)	    // need more characters
		    keylen = KEYLEN_PART_KEY;
	    }
	    if (keylen >= 0)
#endif
		// When there was a matching mapping and no termcode could be
		// replaced after another one, use that mapping (loop around).
		// If there was no mapping at all use the character from the
		// typeahead buffer right here.
		if (mp == NULL)
		{
		    *keylenp = keylen;
		    return map_result_get;    // get character from typeahead
		}
	}

	if (keylen > 0)	    // full matching terminal code
	{
#if defined(FEAT_GUI) && defined(FEAT_MENU)
	    if (typebuf.tb_len >= 2
		    && typebuf.tb_buf[typebuf.tb_off] == K_SPECIAL
			      && typebuf.tb_buf[typebuf.tb_off + 1] == KS_MENU)
	    {
		int	idx;

		// Using a menu may cause a break in undo!  It's like using
		// gotchars(), but without recording or writing to a script
		// file.
		may_sync_undo();
		del_typebuf(3, 0);
		idx = get_menu_index(current_menu, local_State);
		if (idx != MENU_INDEX_INVALID)
		{
		    // In Select mode and a Visual mode menu is used:  Switch
		    // to Visual mode temporarily.  Append K_SELECT to switch
		    // back to Select mode.
		    if (VIsual_active && VIsual_select
					&& (current_menu->modes & MODE_VISUAL))
		    {
			VIsual_select = FALSE;
			(void)ins_typebuf(K_SELECT_STRING,
						   REMAP_NONE, 0, TRUE, FALSE);
		    }
		    ins_typebuf(current_menu->strings[idx],
				current_menu->noremap[idx],
				0, TRUE, current_menu->silent[idx]);
		}
	    }
#endif // FEAT_GUI && FEAT_MENU
	    *keylenp = keylen;
	    return map_result_retry;	// try mapping again
	}

	// Partial match: get some more characters.  When a matching mapping
	// was found use that one.
	if (mp == NULL || keylen < 0)
	    keylen = KEYLEN_PART_KEY;
	else
	    keylen = mp_match_len;
    }

    /*
     * complete match
     */
    if (keylen >= 0 && keylen <= typebuf.tb_len)
    {
	char_u *map_str;

#ifdef FEAT_EVAL
	int	save_m_expr;
	int	save_m_noremap;
	int	save_m_silent;
	char_u	*save_m_keys;
#else
# define save_m_noremap mp->m_noremap
# define save_m_silent mp->m_silent
#endif

	// write chars to script file(s)
	if (keylen > typebuf.tb_maplen)
	    gotchars(typebuf.tb_buf + typebuf.tb_off + typebuf.tb_maplen,
						   keylen - typebuf.tb_maplen);

	cmd_silent = (typebuf.tb_silent > 0);
	del_typebuf(keylen, 0);	// remove the mapped keys

	/*
	 * Put the replacement string in front of mapstr.
	 * The depth check catches ":map x y" and ":map y x".
	 */
	if (++*mapdepth >= p_mmd)
	{
	    emsg(_(e_recursive_mapping));
	    if (State & MODE_CMDLINE)
		redrawcmdline();
	    else
		setcursor();
	    flush_buffers(FLUSH_MINIMAL);
	    *mapdepth = 0;	// for next one
	    *keylenp = keylen;
	    return map_result_fail;
	}

	/*
	 * In Select mode and a Visual mode mapping is used: Switch to Visual
	 * mode temporarily.  Append K_SELECT to switch back to Select mode.
	 */
	if (VIsual_active && VIsual_select && (mp->m_mode & MODE_VISUAL))
	{
	    VIsual_select = FALSE;
	    (void)ins_typebuf(K_SELECT_STRING, REMAP_NONE, 0, TRUE, FALSE);
	}

#ifdef FEAT_EVAL
	// Copy the values from *mp that are used, because evaluating the
	// expression may invoke a function that redefines the mapping, thereby
	// making *mp invalid.
	save_m_expr = mp->m_expr;
	save_m_noremap = mp->m_noremap;
	save_m_silent = mp->m_silent;
	save_m_keys = NULL;  // only saved when needed

	/*
	 * Handle ":map <expr>": evaluate the {rhs} as an expression.  Also
	 * save and restore the command line for "normal :".
	 */
	if (mp->m_expr)
	{
	    int save_vgetc_busy = vgetc_busy;
	    int save_may_garbage_collect = may_garbage_collect;
	    int was_screen_col = screen_cur_col;
	    int was_screen_row = screen_cur_row;
	    int prev_did_emsg = did_emsg;

	    vgetc_busy = 0;
	    may_garbage_collect = FALSE;

	    save_m_keys = vim_strsave(mp->m_keys);
	    map_str = eval_map_expr(mp, NUL);

	    // The mapping may do anything, but we expect it to take care of
	    // redrawing.  Do put the cursor back where it was.
	    windgoto(was_screen_row, was_screen_col);
	    out_flush();

	    // If an error was displayed and the expression returns an empty
	    // string, generate a <Nop> to allow for a redraw.
	    if (prev_did_emsg != did_emsg
				       && (map_str == NULL || *map_str == NUL))
	    {
		char_u	buf[4];

		vim_free(map_str);
		buf[0] = K_SPECIAL;
		buf[1] = KS_EXTRA;
		buf[2] = KE_IGNORE;
		buf[3] = NUL;
		map_str = vim_strsave(buf);
		if (State & MODE_CMDLINE)
		{
		    // redraw the command below the error
		    msg_didout = TRUE;
		    if (msg_row < cmdline_row)
			msg_row = cmdline_row;
		    redrawcmd();
		}
	    }

	    vgetc_busy = save_vgetc_busy;
	    may_garbage_collect = save_may_garbage_collect;
	}
	else
#endif
	    map_str = mp->m_str;

	/*
	 * Insert the 'to' part in the typebuf.tb_buf.
	 * If 'from' field is the same as the start of the 'to' field, don't
	 * remap the first character (but do allow abbreviations).
	 * If m_noremap is set, don't remap the whole 'to' part.
	 */
	if (map_str == NULL)
	    i = FAIL;
	else
	{
	    int noremap;

#ifdef FEAT_EVAL
	    last_used_map = mp;
	    last_used_sid = -1;
#endif
	    if (save_m_noremap != REMAP_YES)
		noremap = save_m_noremap;
	    else if (
#ifdef FEAT_EVAL
		STRNCMP(map_str, save_m_keys != NULL ? save_m_keys : mp->m_keys,
								(size_t)keylen)
#else
		STRNCMP(map_str, mp->m_keys, (size_t)keylen)
#endif
		   != 0)
		noremap = REMAP_YES;
	    else
		noremap = REMAP_SKIP;
	    i = ins_typebuf(map_str, noremap,
					 0, TRUE, cmd_silent || save_m_silent);
#ifdef FEAT_EVAL
	    if (save_m_expr)
		vim_free(map_str);
#endif
	}
#ifdef FEAT_EVAL
	vim_free(save_m_keys);
#endif
	*keylenp = keylen;
	if (i == FAIL)
	    return map_result_fail;
	return map_result_retry;
    }

    *keylenp = keylen;
    return map_result_nomatch;
}

/*
 * unget one character (can only be done once!)
 * If the character was stuffed, vgetc() will get it next time it is called.
 * Otherwise vgetc() will only get it when the stuff buffer is empty.
 */
    void
vungetc(int c)
{
    old_char = c;
    old_mod_mask = mod_mask;
    old_mouse_row = mouse_row;
    old_mouse_col = mouse_col;
    old_KeyStuffed = KeyStuffed;
}

/*
 * When peeking and not getting a character, reg_executing cannot be cleared
 * yet, so set a flag to clear it later.
 */
    static void
check_end_reg_executing(int advance)
{
    if (reg_executing != 0 && (typebuf.tb_maplen == 0
						|| pending_end_reg_executing))
    {
	if (advance)
	{
	    reg_executing = 0;
	    pending_end_reg_executing = FALSE;
	}
	else
	    pending_end_reg_executing = TRUE;
    }

}

/*
 * Get a byte:
 * 1. from the stuffbuffer
 *	This is used for abbreviated commands like "D" -> "d$".
 *	Also used to redo a command for ".".
 * 2. from the typeahead buffer
 *	Stores text obtained previously but not used yet.
 *	Also stores the result of mappings.
 *	Also used for the ":normal" command.
 * 3. from the user
 *	This may do a blocking wait if "advance" is TRUE.
 *
 * if "advance" is TRUE (vgetc()):
 *	Really get the character.
 *	KeyTyped is set to TRUE in the case the user typed the key.
 *	KeyStuffed is TRUE if the character comes from the stuff buffer.
 * if "advance" is FALSE (vpeekc()):
 *	Just look whether there is a character available.
 *	Return NUL if not.
 *
 * When "no_mapping" is zero, checks for mappings in the current mode.
 * Only returns one byte (of a multi-byte character).
 * K_SPECIAL and CSI may be escaped, need to get two more bytes then.
 */
    static int
vgetorpeek(int advance)
{
    int		c;
    int		timedout = FALSE;	// waited for more than 'timeoutlen'
					// for mapping to complete or
					// 'ttimeoutlen' for complete key code
    int		mapdepth = 0;		// check for recursive mapping
    int		mode_deleted = FALSE;   // set when mode has been deleted
    int		new_wcol, new_wrow;
#ifdef FEAT_GUI
    int		shape_changed = FALSE;  // adjusted cursor shape
#endif
    int		n;
    int		old_wcol, old_wrow;
    int		wait_tb_len;

    /*
     * This function doesn't work very well when called recursively.  This may
     * happen though, because of:
     * 1. The call to add_to_showcmd().	char_avail() is then used to check if
     * there is a character available, which calls this function.  In that
     * case we must return NUL, to indicate no character is available.
     * 2. A GUI callback function writes to the screen, causing a
     * wait_return().
     * Using ":normal" can also do this, but it saves the typeahead buffer,
     * thus it should be OK.  But don't get a key from the user then.
     */
    if (vgetc_busy > 0 && ex_normal_busy == 0)
	return NUL;

    ++vgetc_busy;

    if (advance)
    {
	KeyStuffed = FALSE;
	typebuf_was_empty = FALSE;
    }

    init_typebuf();
    start_stuff();
    check_end_reg_executing(advance);
    do
    {
/*
 * get a character: 1. from the stuffbuffer
 */
	if (typeahead_char != 0)
	{
	    c = typeahead_char;
	    if (advance)
		typeahead_char = 0;
	}
	else
	    c = read_readbuffers(advance);
	if (c != NUL && !got_int)
	{
	    if (advance)
	    {
		// KeyTyped = FALSE;  When the command that stuffed something
		// was typed, behave like the stuffed command was typed.
		// needed for CTRL-W CTRL-] to open a fold, for example.
		KeyStuffed = TRUE;
	    }
	    if (typebuf.tb_no_abbr_cnt == 0)
		typebuf.tb_no_abbr_cnt = 1;	// no abbreviations now
	}
	else
	{
	    /*
	     * Loop until we either find a matching mapped key, or we
	     * are sure that it is not a mapped key.
	     * If a mapped key sequence is found we go back to the start to
	     * try re-mapping.
	     */
	    for (;;)
	    {
		long	wait_time;
		int	keylen = 0;
		int	showcmd_idx;
		check_end_reg_executing(advance);
		/*
		 * ui_breakcheck() is slow, don't use it too often when
		 * inside a mapping.  But call it each time for typed
		 * characters.
		 */
		if (typebuf.tb_maplen)
		    line_breakcheck();
		else
		    ui_breakcheck();		// check for CTRL-C
		if (got_int)
		{
		    // flush all input
		    c = inchar(typebuf.tb_buf, typebuf.tb_buflen - 1, 0L);

		    /*
		     * If inchar() returns TRUE (script file was active) or we
		     * are inside a mapping, get out of Insert mode.
		     * Otherwise we behave like having gotten a CTRL-C.
		     * As a result typing CTRL-C in insert mode will
		     * really insert a CTRL-C.
		     */
		    if ((c || typebuf.tb_maplen)
				     && (State & (MODE_INSERT | MODE_CMDLINE)))
			c = ESC;
		    else
			c = Ctrl_C;
		    flush_buffers(FLUSH_INPUT);	// flush all typeahead

		    if (advance)
		    {
			// Also record this character, it might be needed to
			// get out of Insert mode.
			*typebuf.tb_buf = c;
			gotchars(typebuf.tb_buf, 1);
		    }
		    cmd_silent = FALSE;

		    break;
		}
		else if (typebuf.tb_len > 0)
		{
		    /*
		     * Check for a mapping in "typebuf".
		     */
		    map_result_T result = handle_mapping(
						&keylen, &timedout, &mapdepth);

		    if (result == map_result_retry)
			// try mapping again
			continue;
		    if (result == map_result_fail)
		    {
			// failed, use the outer loop
			c = -1;
			break;
		    }
		    if (result == map_result_get)
		    {
/*
 * get a character: 2. from the typeahead buffer
 */
			c = typebuf.tb_buf[typebuf.tb_off];
			if (advance)	// remove chars from typebuf
			{
			    cmd_silent = (typebuf.tb_silent > 0);
			    if (typebuf.tb_maplen > 0)
				KeyTyped = FALSE;
			    else
			    {
				KeyTyped = TRUE;
				// write char to script file(s)
				gotchars(typebuf.tb_buf
						 + typebuf.tb_off, 1);
			    }
			    KeyNoremap = typebuf.tb_noremap[typebuf.tb_off];
			    del_typebuf(1, 0);
			}
			break;  // got character, break the for loop
		    }

		    // not enough characters, get more
		}

/*
 * get a character: 3. from the user - handle <Esc> in Insert mode
 */
		/*
		 * Special case: if we get an <ESC> in Insert mode and there
		 * are no more characters at once, we pretend to go out of
		 * Insert mode.  This prevents the one second delay after
		 * typing an <ESC>.  If we get something after all, we may
		 * have to redisplay the mode. That the cursor is in the wrong
		 * place does not matter.
		 * Do not do this if the kitty keyboard protocol is used, every
		 * <ESC> is the start of an escape sequence then.
		 */
		c = 0;
		new_wcol = curwin->w_wcol;
		new_wrow = curwin->w_wrow;
		if (	   advance
			&& typebuf.tb_len == 1
			&& typebuf.tb_buf[typebuf.tb_off] == ESC
			&& !no_mapping
			&& kitty_protocol_state != KKPS_ENABLED
			&& ex_normal_busy == 0
			&& typebuf.tb_maplen == 0
			&& (State & MODE_INSERT)
			&& (p_timeout
			    || (keylen == KEYLEN_PART_KEY && p_ttimeout))
			&& (c = inchar(typebuf.tb_buf + typebuf.tb_off
					       + typebuf.tb_len, 3, 25L)) == 0)
		{
		    colnr_T	col = 0;
		    char_u	*ptr;

		    if (mode_displayed)
		    {
			unshowmode(TRUE);
			mode_deleted = TRUE;
		    }
#ifdef FEAT_GUI
		    // may show a different cursor shape
		    if (gui.in_use && State != MODE_NORMAL && !cmd_silent)
		    {
			int	    save_State;

			save_State = State;
			State = MODE_NORMAL;
			gui_update_cursor(TRUE, FALSE);
			State = save_State;
			shape_changed = TRUE;
		    }
#endif
		    validate_cursor();
		    old_wcol = curwin->w_wcol;
		    old_wrow = curwin->w_wrow;

		    // move cursor left, if possible
		    if (curwin->w_cursor.col != 0)
		    {
			if (curwin->w_wcol > 0)
			{
			    // After auto-indenting and no text is following,
			    // we are expecting to truncate the trailing
			    // white-space, so find the last non-white
			    // character -- webb
			    if (did_ai && *skipwhite(ml_get_curline()
						+ curwin->w_cursor.col) == NUL)
			    {
				chartabsize_T cts;

				curwin->w_wcol = 0;
				ptr = ml_get_curline();
				init_chartabsize_arg(&cts, curwin,
					  curwin->w_cursor.lnum, 0, ptr, ptr);
				while (cts.cts_ptr < ptr + curwin->w_cursor.col)
				{
				    if (!VIM_ISWHITE(*cts.cts_ptr))
					curwin->w_wcol = cts.cts_vcol;
				    cts.cts_vcol += lbr_chartabsize(&cts);
				    if (has_mbyte)
					cts.cts_ptr +=
						   (*mb_ptr2len)(cts.cts_ptr);
				    else
					++cts.cts_ptr;
				}
				clear_chartabsize_arg(&cts);

				curwin->w_wrow = curwin->w_cline_row
					   + curwin->w_wcol / curwin->w_width;
				curwin->w_wcol %= curwin->w_width;
				curwin->w_wcol += curwin_col_off();
				col = 0;	// no correction needed
			    }
			    else
			    {
				--curwin->w_wcol;
				col = curwin->w_cursor.col - 1;
			    }
			}
			else if (curwin->w_p_wrap && curwin->w_wrow)
			{
			    --curwin->w_wrow;
			    curwin->w_wcol = curwin->w_width - 1;
			    col = curwin->w_cursor.col - 1;
			}
			if (has_mbyte && col > 0 && curwin->w_wcol > 0)
			{
			    // Correct when the cursor is on the right halve
			    // of a double-wide character.
			    ptr = ml_get_curline();
			    col -= (*mb_head_off)(ptr, ptr + col);
			    if ((*mb_ptr2cells)(ptr + col) > 1)
				--curwin->w_wcol;
			}
		    }
		    setcursor();
		    out_flush();
		    new_wcol = curwin->w_wcol;
		    new_wrow = curwin->w_wrow;
		    curwin->w_wcol = old_wcol;
		    curwin->w_wrow = old_wrow;
		}
		if (c < 0)
		    continue;	// end of input script reached

		// Allow mapping for just typed characters. When we get here c
		// is the number of extra bytes and typebuf.tb_len is 1.
		for (n = 1; n <= c; ++n)
		    typebuf.tb_noremap[typebuf.tb_off + n] = RM_YES;
		typebuf.tb_len += c;

		// buffer full, don't map
		if (typebuf.tb_len >= typebuf.tb_maplen + MAXMAPLEN)
		{
		    timedout = TRUE;
		    continue;
		}

		if (ex_normal_busy > 0)
		{
		    static int tc = 0;

		    // No typeahead left and inside ":normal".  Must return
		    // something to avoid getting stuck.  When an incomplete
		    // mapping is present, behave like it timed out.
		    if (typebuf.tb_len > 0)
		    {
			timedout = TRUE;
			continue;
		    }

		    // When 'insertmode' is set, ESC just beeps in Insert
		    // mode.  Use CTRL-L to make edit() return.
		    // For the command line only CTRL-C always breaks it.
		    // For the cmdline window: Alternate between ESC and
		    // CTRL-C: ESC for most situations and CTRL-C to close the
		    // cmdline window.
		    if (p_im && (State & MODE_INSERT))
			c = Ctrl_L;
#ifdef FEAT_TERMINAL
		    else if (terminal_is_active())
			c = K_CANCEL;
#endif
		    else if ((State & MODE_CMDLINE)
					     || (cmdwin_type > 0 && tc == ESC))
			c = Ctrl_C;
		    else
			c = ESC;
		    tc = c;
		    // set a flag to indicate this wasn't a normal char
		    if (advance)
			typebuf_was_empty = TRUE;

		    // return from main_loop()
		    if (pending_exmode_active)
			exmode_active = EXMODE_NORMAL;

		    // no chars to block abbreviation for
		    typebuf.tb_no_abbr_cnt = 0;

		    break;
		}

/*
 * get a character: 3. from the user - update display
 */
		// In insert mode a screen update is skipped when characters
		// are still available.  But when those available characters
		// are part of a mapping, and we are going to do a blocking
		// wait here.  Need to update the screen to display the
		// changed text so far. Also for when 'lazyredraw' is set and
		// redrawing was postponed because there was something in the
		// input buffer (e.g., termresponse).
		if (((State & MODE_INSERT) != 0 || p_lz)
			&& (State & MODE_CMDLINE) == 0
			&& advance && must_redraw != 0 && !need_wait_return)
		{
		    update_screen(0);
		    setcursor(); // put cursor back where it belongs
		}

		/*
		 * If we have a partial match (and are going to wait for more
		 * input from the user), show the partially matched characters
		 * to the user with showcmd.
		 */
		showcmd_idx = 0;
		int showing_partial = FALSE;
		if (typebuf.tb_len > 0 && advance && !exmode_active)
		{
		    if (((State & (MODE_NORMAL | MODE_INSERT))
						      || State == MODE_LANGMAP)
			    && State != MODE_HITRETURN)
		    {
			// this looks nice when typing a dead character map
			if (State & MODE_INSERT
			    && ptr2cells(typebuf.tb_buf + typebuf.tb_off
						   + typebuf.tb_len - 1) == 1)
			{
			    edit_putchar(typebuf.tb_buf[typebuf.tb_off
						+ typebuf.tb_len - 1], FALSE);
			    setcursor(); // put cursor back where it belongs
			    showing_partial = TRUE;
			}
			// need to use the col and row from above here
			old_wcol = curwin->w_wcol;
			old_wrow = curwin->w_wrow;
			curwin->w_wcol = new_wcol;
			curwin->w_wrow = new_wrow;
			push_showcmd();
			if (typebuf.tb_len > SHOWCMD_COLS)
			    showcmd_idx = typebuf.tb_len - SHOWCMD_COLS;
			while (showcmd_idx < typebuf.tb_len)
			    (void)add_to_showcmd(
			       typebuf.tb_buf[typebuf.tb_off + showcmd_idx++]);
			curwin->w_wcol = old_wcol;
			curwin->w_wrow = old_wrow;
		    }

		    // This looks nice when typing a dead character map.
		    // There is no actual command line for get_number().
		    if ((State & MODE_CMDLINE)
			    && get_cmdline_info()->cmdbuff != NULL
#if defined(FEAT_CRYPT) || defined(FEAT_EVAL)
			    && cmdline_star == 0
#endif
			    && ptr2cells(typebuf.tb_buf + typebuf.tb_off
						   + typebuf.tb_len - 1) == 1)
		    {
			putcmdline(typebuf.tb_buf[typebuf.tb_off
						+ typebuf.tb_len - 1], FALSE);
			showing_partial = TRUE;
		    }
		}

/*
 * get a character: 3. from the user - get it
 */
		if (typebuf.tb_len == 0)
		    // timedout may have been set if a mapping with empty RHS
		    // fully matched while longer mappings timed out.
		    timedout = FALSE;

		if (advance)
		{
		    if (typebuf.tb_len == 0
			    || !(p_timeout
				 || (p_ttimeout && keylen == KEYLEN_PART_KEY)))
			// blocking wait
			wait_time = -1L;
		    else if (keylen == KEYLEN_PART_KEY && p_ttm >= 0)
			wait_time = p_ttm;
		    else
			wait_time = p_tm;
		}
		else
		    wait_time = 0;

		wait_tb_len = typebuf.tb_len;
		c = inchar(typebuf.tb_buf + typebuf.tb_off + typebuf.tb_len,
			typebuf.tb_buflen - typebuf.tb_off - typebuf.tb_len - 1,
			wait_time);

		if (showcmd_idx != 0)
		    pop_showcmd();
		if (showing_partial)
		{
		    if (State & MODE_INSERT)
			edit_unputchar();
		    if ((State & MODE_CMDLINE)
					&& get_cmdline_info()->cmdbuff != NULL)
			unputcmdline();
		    else
			setcursor();	// put cursor back where it belongs
		}

		if (c < 0)
		    continue;		// end of input script reached
		if (c == NUL)		// no character available
		{
		    if (!advance)
			break;
		    if (wait_tb_len > 0)	// timed out
		    {
			timedout = TRUE;
			continue;
		    }
		}
		else
		{	    // allow mapping for just typed characters
		    while (typebuf.tb_buf[typebuf.tb_off
						     + typebuf.tb_len] != NUL)
			typebuf.tb_noremap[typebuf.tb_off
						 + typebuf.tb_len++] = RM_YES;
#ifdef HAVE_INPUT_METHOD
		    // Get IM status right after getting keys, not after the
		    // timeout for a mapping (focus may be lost by then).
		    vgetc_im_active = im_get_status();
#endif
		}
	    }	    // for (;;)
	}	// if (!character from stuffbuf)

	// if advance is FALSE don't loop on NULs
    } while ((c < 0 && c != K_CANCEL) || (advance && c == NUL));

    /*
     * The "INSERT" message is taken care of here:
     *	 if we return an ESC to exit insert mode, the message is deleted
     *	 if we don't return an ESC but deleted the message before, redisplay it
     */
    if (advance && p_smd && msg_silent == 0 && (State & MODE_INSERT))
    {
	if (c == ESC && !mode_deleted && !no_mapping && mode_displayed)
	{
	    if (typebuf.tb_len && !KeyTyped)
		redraw_cmdline = TRUE;	    // delete mode later
	    else
		unshowmode(FALSE);
	}
	else if (c != ESC && mode_deleted)
	{
	    if (typebuf.tb_len && !KeyTyped)
		redraw_cmdline = TRUE;	    // show mode later
	    else
		showmode();
	}
    }
#ifdef FEAT_GUI
    // may unshow different cursor shape
    if (gui.in_use && shape_changed)
	gui_update_cursor(TRUE, FALSE);
#endif
    if (timedout && c == ESC)
    {
	// When recording there will be no timeout.  Add an <Ignore> after the
	// ESC to avoid that it forms a key code with following characters.
	gotchars_ignore();
    }

    --vgetc_busy;

    return c;
}

/*
 * inchar() - get one character from
 *	1. a scriptfile
 *	2. the keyboard
 *
 *  As many characters as we can get (up to 'maxlen') are put in "buf" and
 *  NUL terminated (buffer length must be 'maxlen' + 1).
 *  Minimum for "maxlen" is 3!!!!
 *
 *  "tb_change_cnt" is the value of typebuf.tb_change_cnt if "buf" points into
 *  it.  When typebuf.tb_change_cnt changes (e.g., when a message is received
 *  from a remote client) "buf" can no longer be used.  "tb_change_cnt" is 0
 *  otherwise.
 *
 *  If we got an interrupt all input is read until none is available.
 *
 *  If wait_time == 0  there is no waiting for the char.
 *  If wait_time == n  we wait for n msec for a character to arrive.
 *  If wait_time == -1 we wait forever for a character to arrive.
 *
 *  Return the number of obtained characters.
 *  Return -1 when end of input script reached.
 */
    static int
inchar(
    char_u	*buf,
    int		maxlen,
    long	wait_time)	    // milliseconds
{
    int		len = 0;	    // init for GCC
    int		retesc = FALSE;	    // return ESC with gotint
    int		script_char;
    int		tb_change_cnt = typebuf.tb_change_cnt;

    if (wait_time == -1L || wait_time > 100L)  // flush output before waiting
    {
	cursor_on();
	out_flush_cursor(FALSE, FALSE);
#if defined(FEAT_GUI) && defined(FEAT_MOUSESHAPE)
	if (gui.in_use && postponed_mouseshape)
	    update_mouseshape(-1);
#endif
    }

    /*
     * Don't reset these when at the hit-return prompt, otherwise a endless
     * recursive loop may result (write error in swapfile, hit-return, timeout
     * on char wait, flush swapfile, write error....).
     */
    if (State != MODE_HITRETURN)
    {
	did_outofmem_msg = FALSE;   // display out of memory message (again)
	did_swapwrite_msg = FALSE;  // display swap file write error again
    }
    undo_off = FALSE;		    // restart undo now

    /*
     * Get a character from a script file if there is one.
     * If interrupted: Stop reading script files, close them all.
     */
    script_char = -1;
    while (scriptin[curscript] != NULL && script_char < 0
#ifdef FEAT_EVAL
	    && !ignore_script
#endif
	    )
    {
#ifdef MESSAGE_QUEUE
	parse_queued_messages();
#endif

	if (got_int || (script_char = getc(scriptin[curscript])) < 0)
	{
	    // Reached EOF.
	    // Careful: closescript() frees typebuf.tb_buf[] and buf[] may
	    // point inside typebuf.tb_buf[].  Don't use buf[] after this!
	    closescript();
	    /*
	     * When reading script file is interrupted, return an ESC to get
	     * back to normal mode.
	     * Otherwise return -1, because typebuf.tb_buf[] has changed.
	     */
	    if (got_int)
		retesc = TRUE;
	    else
		return -1;
	}
	else
	{
	    buf[0] = script_char;
	    len = 1;
	}
    }

    if (script_char < 0)	// did not get a character from script
    {
	/*
	 * If we got an interrupt, skip all previously typed characters and
	 * return TRUE if quit reading script file.
	 * Stop reading typeahead when a single CTRL-C was read,
	 * fill_input_buf() returns this when not able to read from stdin.
	 * Don't use buf[] here, closescript() may have freed typebuf.tb_buf[]
	 * and buf may be pointing inside typebuf.tb_buf[].
	 */
	if (got_int)
	{
#define DUM_LEN (MAXMAPLEN * 3 + 3)
	    char_u	dum[DUM_LEN + 1];

	    for (;;)
	    {
		len = ui_inchar(dum, DUM_LEN, 0L, 0);
		if (len == 0 || (len == 1 && dum[0] == Ctrl_C))
		    break;
	    }
	    return retesc;
	}

	/*
	 * Always flush the output characters when getting input characters
	 * from the user and not just peeking.
	 */
	if (wait_time == -1L || wait_time > 10L)
	    out_flush();

	/*
	 * Fill up to a third of the buffer, because each character may be
	 * tripled below.
	 */
	len = ui_inchar(buf, maxlen / 3, wait_time, tb_change_cnt);
    }

    // If the typebuf was changed further down, it is like nothing was added by
    // this call.
    if (typebuf_changed(tb_change_cnt))
	return 0;

    // Note the change in the typeahead buffer, this matters for when
    // vgetorpeek() is called recursively, e.g. using getchar(1) in a timer
    // function.
    if (len > 0 && ++typebuf.tb_change_cnt == 0)
	typebuf.tb_change_cnt = 1;

    return fix_input_buffer(buf, len);
}

/*
 * Fix typed characters for use by vgetc() and check_termcode().
 * "buf[]" must have room to triple the number of bytes!
 * Returns the new length.
 */
    int
fix_input_buffer(char_u *buf, int len)
{
    int		i;
    char_u	*p = buf;

    /*
     * Two characters are special: NUL and K_SPECIAL.
     * When compiled With the GUI CSI is also special.
     * Replace	     NUL by K_SPECIAL KS_ZERO	 KE_FILLER
     * Replace K_SPECIAL by K_SPECIAL KS_SPECIAL KE_FILLER
     * Replace       CSI by K_SPECIAL KS_EXTRA   KE_CSI
     */
    for (i = len; --i >= 0; ++p)
    {
#ifdef FEAT_GUI
	// When the GUI is used any character can come after a CSI, don't
	// escape it.
	if (gui.in_use && p[0] == CSI && i >= 2)
	{
	    p += 2;
	    i -= 2;
	}
# ifndef MSWIN
	// When not on MS-Windows and the GUI is not used CSI needs to be
	// escaped.
	else if (!gui.in_use && p[0] == CSI)
	{
	    mch_memmove(p + 3, p + 1, (size_t)i);
	    *p++ = K_SPECIAL;
	    *p++ = KS_EXTRA;
	    *p = (int)KE_CSI;
	    len += 2;
	}
# endif
	else
#endif
	if (p[0] == NUL || (p[0] == K_SPECIAL
		    // timeout may generate K_CURSORHOLD
		    && (i < 2 || p[1] != KS_EXTRA || p[2] != (int)KE_CURSORHOLD)
#if defined(MSWIN) && (!defined(FEAT_GUI) || defined(VIMDLL))
		    // Win32 console passes modifiers
		    && (
# ifdef VIMDLL
			gui.in_use ||
# endif
			(i < 2 || p[1] != KS_MODIFIER))
#endif
		    ))
	{
	    mch_memmove(p + 3, p + 1, (size_t)i);
	    p[2] = K_THIRD(p[0]);
	    p[1] = K_SECOND(p[0]);
	    p[0] = K_SPECIAL;
	    p += 2;
	    len += 2;
	}
    }
    *p = NUL;		// add trailing NUL
    return len;
}

#if defined(USE_INPUT_BUF) || defined(PROTO)
/*
 * Return TRUE when bytes are in the input buffer or in the typeahead buffer.
 * Normally the input buffer would be sufficient, but the server_to_input_buf()
 * or feedkeys() may insert characters in the typeahead buffer while we are
 * waiting for input to arrive.
 */
    int
input_available(void)
{
    return (!vim_is_input_buf_empty()
# if defined(FEAT_CLIENTSERVER) || defined(FEAT_EVAL)
	    || typebuf_was_filled
# endif
	    );
}
#endif

/*
 * Function passed to do_cmdline() to get the command after a <Cmd> key from
 * typeahead.
 */
    static char_u *
getcmdkeycmd(
	int		promptc UNUSED,
	void		*cookie UNUSED,
	int		indent UNUSED,
	getline_opt_T	do_concat UNUSED)
{
    garray_T	line_ga;
    int		c1 = -1;
    int		c2;
    int		cmod = 0;
    int		aborted = FALSE;

    ga_init2(&line_ga, 1, 32);

    // no mapping for these characters
    no_mapping++;

    got_int = FALSE;
    while (c1 != NUL && !aborted)
    {
	if (ga_grow(&line_ga, 32) == FAIL)
	{
	    aborted = TRUE;
	    break;
	}

	if (vgetorpeek(FALSE) == NUL)
	{
	    // incomplete <Cmd> is an error, because there is not much the user
	    // could do in this state.
	    emsg(_(e_cmd_mapping_must_end_with_cr));
	    aborted = TRUE;
	    break;
	}

	// Get one character at a time.
	c1 = vgetorpeek(TRUE);

	// Get two extra bytes for special keys
	if (c1 == K_SPECIAL)
	{
	    c1 = vgetorpeek(TRUE);
	    c2 = vgetorpeek(TRUE);
	    if (c1 == KS_MODIFIER)
	    {
		cmod = c2;
		continue;
	    }
	    c1 = TO_SPECIAL(c1, c2);

	    // K_ESC is used to avoid ambiguity with the single Esc character
	    // that might be the start of an escape sequence.  Convert it back
	    // to a single Esc here.
	    if (c1 == K_ESC)
		c1 = ESC;
	}

	if (got_int)
	    aborted = TRUE;
	else if (c1 == '\r' || c1 == '\n')
	    c1 = NUL;  // end the line
	else if (c1 == ESC)
	    aborted = TRUE;
	else if (c1 == K_COMMAND || c1 == K_SCRIPT_COMMAND)
	{
	    // give a nicer error message for this special case
	    emsg(_(e_cmd_mapping_must_end_with_cr_before_second_cmd));
	    aborted = TRUE;
	}
	else if (c1 == K_SNR)
	{
	    ga_concat(&line_ga, (char_u *)"<SNR>");
	}
	else
	{
	    if (cmod != 0)
	    {
		ga_append(&line_ga, K_SPECIAL);
		ga_append(&line_ga, KS_MODIFIER);
		ga_append(&line_ga, cmod);
	    }
	    if (IS_SPECIAL(c1))
	    {
		ga_append(&line_ga, K_SPECIAL);
		ga_append(&line_ga, K_SECOND(c1));
		ga_append(&line_ga, K_THIRD(c1));
	    }
	    else
		ga_append(&line_ga, c1);
	}

	cmod = 0;
    }

    no_mapping--;

    if (aborted)
	ga_clear(&line_ga);

    return (char_u *)line_ga.ga_data;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * If there was a mapping we get its SID.  Otherwise, use "last_used_sid", it
 * is set when redo'ing.
 * Put this SID in the redo buffer, so that "." will use the same script
 * context.
 */
    void
may_add_last_used_map_to_redobuff(void)
{
    char_u  buf[3 + 20];
    int	    sid = -1;

    if (last_used_map != NULL)
	sid = last_used_map->m_script_ctx.sc_sid;
    if (sid < 0)
	sid = last_used_sid;

    if (sid < 0)
	return;

    // <K_SID>{nr};
    buf[0] = K_SPECIAL;
    buf[1] = KS_EXTRA;
    buf[2] = KE_SID;
    vim_snprintf((char *)buf + 3, 20, "%d;", sid);
    add_buff(&redobuff, buf, -1L);
}
#endif

    int
do_cmdkey_command(int key UNUSED, int flags)
{
    int	    res;
#ifdef FEAT_EVAL
    sctx_T  save_current_sctx = {-1, 0, 0, 0};

    if (key == K_SCRIPT_COMMAND
		  && (last_used_map != NULL || SCRIPT_ID_VALID(last_used_sid)))
    {
	save_current_sctx = current_sctx;
	if (last_used_map != NULL)
	    current_sctx = last_used_map->m_script_ctx;
	else
	{
	    current_sctx.sc_sid = last_used_sid;
	    current_sctx.sc_lnum = 0;
	    current_sctx.sc_version = SCRIPT_ITEM(last_used_sid)->sn_version;
	}
    }
#endif

    res = do_cmdline(NULL, getcmdkeycmd, NULL, flags);

#ifdef FEAT_EVAL
    if (save_current_sctx.sc_sid >= 0)
	current_sctx = save_current_sctx;
#endif

    return res;
}

#if defined(FEAT_EVAL) || defined(PROTO)
    void
reset_last_used_map(mapblock_T *mp)
{
    if (last_used_map != mp)
	return;

    last_used_map = NULL;
    last_used_sid = -1;
}
#endif
