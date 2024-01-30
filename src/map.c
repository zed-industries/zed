/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * map.c: Code for mappings and abbreviations.
 */

#include "vim.h"

/*
 * List used for abbreviations.
 */
static mapblock_T	*first_abbr = NULL; // first entry in abbrlist

/*
 * Each mapping is put in one of the 256 hash lists, to speed up finding it.
 */
static mapblock_T	*(maphash[256]);
static int		maphash_valid = FALSE;

// When non-zero then no mappings can be added or removed.  Prevents mappings
// to change while listing them.
static int		map_locked = 0;

/*
 * Make a hash value for a mapping.
 * "mode" is the lower 4 bits of the State for the mapping.
 * "c1" is the first character of the "lhs".
 * Returns a value between 0 and 255, index in maphash.
 * Put Normal/Visual mode mappings mostly separately from Insert/Cmdline mode.
 */
#define MAP_HASH(mode, c1) (((mode) & (MODE_NORMAL | MODE_VISUAL | MODE_SELECT | MODE_OP_PENDING | MODE_TERMINAL)) ? (c1) : ((c1) ^ 0x80))

/*
 * Get the start of the hashed map list for "state" and first character "c".
 */
    mapblock_T *
get_maphash_list(int state, int c)
{
    return maphash[MAP_HASH(state, c)];
}

/*
 * Get the buffer-local hashed map list for "state" and first character "c".
 */
    mapblock_T *
get_buf_maphash_list(int state, int c)
{
    return curbuf->b_maphash[MAP_HASH(state, c)];
}

    int
is_maphash_valid(void)
{
    return maphash_valid;
}

/*
 * Initialize maphash[] for first use.
 */
    static void
validate_maphash(void)
{
    if (maphash_valid)
	return;

    CLEAR_FIELD(maphash);
    maphash_valid = TRUE;
}

/*
 * Delete one entry from the abbrlist or maphash[].
 * "mpp" is a pointer to the m_next field of the PREVIOUS entry!
 */
    static void
map_free(mapblock_T **mpp)
{
    mapblock_T	*mp;

    mp = *mpp;
    vim_free(mp->m_keys);
    vim_free(mp->m_str);
    vim_free(mp->m_orig_str);
    *mpp = mp->m_next;
#ifdef FEAT_EVAL
    reset_last_used_map(mp);
#endif
    vim_free(mp);
}

/*
 * Return characters to represent the map mode in an allocated string.
 * Returns NULL when out of memory.
 */
    static char_u *
map_mode_to_chars(int mode)
{
    garray_T    mapmode;

    ga_init2(&mapmode, 1, 7);

    if ((mode & (MODE_INSERT | MODE_CMDLINE)) == (MODE_INSERT | MODE_CMDLINE))
	ga_append(&mapmode, '!');			// :map!
    else if (mode & MODE_INSERT)
	ga_append(&mapmode, 'i');			// :imap
    else if (mode & MODE_LANGMAP)
	ga_append(&mapmode, 'l');			// :lmap
    else if (mode & MODE_CMDLINE)
	ga_append(&mapmode, 'c');			// :cmap
    else if ((mode
		 & (MODE_NORMAL | MODE_VISUAL | MODE_SELECT | MODE_OP_PENDING))
		== (MODE_NORMAL | MODE_VISUAL | MODE_SELECT | MODE_OP_PENDING))
	ga_append(&mapmode, ' ');			// :map
    else
    {
	if (mode & MODE_NORMAL)
	    ga_append(&mapmode, 'n');			// :nmap
	if (mode & MODE_OP_PENDING)
	    ga_append(&mapmode, 'o');			// :omap
	if (mode & MODE_TERMINAL)
	    ga_append(&mapmode, 't');			// :tmap
	if ((mode & (MODE_VISUAL | MODE_SELECT)) == (MODE_VISUAL | MODE_SELECT))
	    ga_append(&mapmode, 'v');			// :vmap
	else
	{
	    if (mode & MODE_VISUAL)
		ga_append(&mapmode, 'x');		// :xmap
	    if (mode & MODE_SELECT)
		ga_append(&mapmode, 's');		// :smap
	}
    }

    ga_append(&mapmode, NUL);
    return (char_u *)mapmode.ga_data;
}

/*
 * Output a line for one mapping.
 */
    static void
showmap(
    mapblock_T	*mp,
    int		local)	    // TRUE for buffer-local map
{
    int		len = 1;
    char_u	*mapchars;

    if (message_filtered(mp->m_keys) && message_filtered(mp->m_str))
	return;

    // Prevent mappings to be cleared while at the more prompt.
    // Must jump to "theend" instead of returning.
    ++map_locked;

    if (msg_didout || msg_silent != 0)
    {
	msg_putchar('\n');
	if (got_int)	    // 'q' typed at MORE prompt
	    goto theend;
    }

    mapchars = map_mode_to_chars(mp->m_mode);
    if (mapchars != NULL)
    {
	msg_puts((char *)mapchars);
	len = (int)STRLEN(mapchars);
	vim_free(mapchars);
    }

    while (++len <= 3)
	msg_putchar(' ');

    // Display the LHS.  Get length of what we write.
    len = msg_outtrans_special(mp->m_keys, TRUE, 0);
    do
    {
	msg_putchar(' ');		// pad with blanks
	++len;
    } while (len < 12);

    if (mp->m_noremap == REMAP_NONE)
	msg_puts_attr("*", HL_ATTR(HLF_8));
    else if (mp->m_noremap == REMAP_SCRIPT)
	msg_puts_attr("&", HL_ATTR(HLF_8));
    else
	msg_putchar(' ');

    if (local)
	msg_putchar('@');
    else
	msg_putchar(' ');

    // Use FALSE below if we only want things like <Up> to show up as such on
    // the rhs, and not M-x etc, TRUE gets both -- webb
    if (*mp->m_str == NUL)
	msg_puts_attr("<Nop>", HL_ATTR(HLF_8));
    else
	msg_outtrans_special(mp->m_str, FALSE, 0);
#ifdef FEAT_EVAL
    if (p_verbose > 0)
	last_set_msg(mp->m_script_ctx);
#endif
    msg_clr_eos();
    out_flush();			// show one line at a time

theend:
    --map_locked;
}

    static int
map_add(
	mapblock_T  **map_table,
	mapblock_T  **abbr_table,
	char_u	    *keys,
	char_u	    *rhs,
	char_u	    *orig_rhs,
	int	    noremap,
	int	    nowait,
	int	    silent,
	int	    mode,
	int	    is_abbr,
#ifdef FEAT_EVAL
	int	    expr,
	scid_T	    sid,	    // 0 to use current_sctx
	int	    scriptversion,
	linenr_T    lnum,
#endif
	int	    simplified)
{
    mapblock_T	*mp = ALLOC_CLEAR_ONE(mapblock_T);

    if (mp == NULL)
	return FAIL;

    // If CTRL-C has been mapped, don't always use it for Interrupting.
    if (*keys == Ctrl_C)
    {
	if (map_table == curbuf->b_maphash)
	    curbuf->b_mapped_ctrl_c |= mode;
	else
	    mapped_ctrl_c |= mode;
    }

    mp->m_keys = vim_strsave(keys);
    mp->m_str = vim_strsave(rhs);
    mp->m_orig_str = vim_strsave(orig_rhs);
    if (mp->m_keys == NULL || mp->m_str == NULL)
    {
	vim_free(mp->m_keys);
	vim_free(mp->m_str);
	vim_free(mp->m_orig_str);
	vim_free(mp);
	return FAIL;
    }
    mp->m_keylen = (int)STRLEN(mp->m_keys);
    mp->m_noremap = noremap;
    mp->m_nowait = nowait;
    mp->m_silent = silent;
    mp->m_mode = mode;
    mp->m_simplified = simplified;
#ifdef FEAT_EVAL
    mp->m_expr = expr;
    if (sid != 0)
    {
	mp->m_script_ctx.sc_sid = sid;
	mp->m_script_ctx.sc_lnum = lnum;
	mp->m_script_ctx.sc_version = scriptversion;
    }
    else
    {
	mp->m_script_ctx = current_sctx;
	mp->m_script_ctx.sc_lnum += SOURCING_LNUM;
    }
#endif

    // add the new entry in front of the abbrlist or maphash[] list
    if (is_abbr)
    {
	mp->m_next = *abbr_table;
	*abbr_table = mp;
    }
    else
    {
	int n = MAP_HASH(mp->m_mode, mp->m_keys[0]);

	mp->m_next = map_table[n];
	map_table[n] = mp;
    }
    return OK;
}

/*
 * List mappings.  When "haskey" is FALSE all mappings, otherwise mappings that
 * match "keys[keys_len]".
 */
    static void
list_mappings(
	int	keyround,
	int	abbrev,
	int	haskey,
	char_u	*keys,
	int	keys_len,
	int	mode,
	int	*did_local)
{
    // Prevent mappings to be cleared while at the more prompt.
    ++map_locked;

    if (p_verbose > 0 && keyround == 1)
    {
	if (seenModifyOtherKeys)
	    msg_puts(_("Seen modifyOtherKeys: true\n"));

	if (modify_otherkeys_state != MOKS_INITIAL)
	{
	    char *name = _("Unknown");
	    switch (modify_otherkeys_state)
	    {
		case MOKS_INITIAL: break;
		case MOKS_OFF: name = _("Off"); break;
		case MOKS_ENABLED: name = _("On"); break;
		case MOKS_DISABLED: name = _("Disabled"); break;
		case MOKS_AFTER_T_TE: name = _("Cleared"); break;
	    }

	    char buf[200];
	    vim_snprintf(buf, sizeof(buf),
				    _("modifyOtherKeys detected: %s\n"), name);
	    msg_puts(buf);
	}

	if (kitty_protocol_state != KKPS_INITIAL)
	{
	    char *name = _("Unknown");
	    switch (kitty_protocol_state)
	    {
		case KKPS_INITIAL: break;
		case KKPS_OFF: name = _("Off"); break;
		case KKPS_ENABLED: name = _("On"); break;
		case KKPS_DISABLED: name = _("Disabled"); break;
		case KKPS_AFTER_T_TE: name = _("Cleared"); break;
	    }

	    char buf[200];
	    vim_snprintf(buf, sizeof(buf),
				     _("Kitty keyboard protocol: %s\n"), name);
	    msg_puts(buf);
	}
    }

    // need to loop over all global hash lists
    for (int hash = 0; hash < 256 && !got_int; ++hash)
    {
	mapblock_T	*mp;

	if (abbrev)
	{
	    if (hash != 0)	// there is only one abbreviation list
		break;
	    mp = curbuf->b_first_abbr;
	}
	else
	    mp = curbuf->b_maphash[hash];
	for ( ; mp != NULL && !got_int; mp = mp->m_next)
	{
	    // check entries with the same mode
	    if (!mp->m_simplified && (mp->m_mode & mode) != 0)
	    {
		if (!haskey)		    // show all entries
		{
		    showmap(mp, TRUE);
		    *did_local = TRUE;
		}
		else
		{
		    int n = mp->m_keylen;
		    if (STRNCMP(mp->m_keys, keys,
				   (size_t)(n < keys_len ? n : keys_len)) == 0)
		    {
			showmap(mp, TRUE);
			*did_local = TRUE;
		    }
		}
	    }
	}
    }

    --map_locked;
}

/*
 * map[!]		    : show all key mappings
 * map[!] {lhs}		    : show key mapping for {lhs}
 * map[!] {lhs} {rhs}	    : set key mapping for {lhs} to {rhs}
 * noremap[!] {lhs} {rhs}   : same, but no remapping for {rhs}
 * unmap[!] {lhs}	    : remove key mapping for {lhs}
 * abbr			    : show all abbreviations
 * abbr {lhs}		    : show abbreviations for {lhs}
 * abbr {lhs} {rhs}	    : set abbreviation for {lhs} to {rhs}
 * noreabbr {lhs} {rhs}	    : same, but no remapping for {rhs}
 * unabbr {lhs}		    : remove abbreviation for {lhs}
 *
 * maptype: MAPTYPE_MAP for :map
 *	    MAPTYPE_UNMAP for :unmap
 *	    MAPTYPE_NOREMAP for noremap
 *
 * arg is pointer to any arguments. Note: arg cannot be a read-only string,
 * it will be modified.
 *
 * for :map   mode is MODE_NORMAL | MODE_VISUAL | MODE_SELECT | MODE_OP_PENDING
 * for :map!  mode is MODE_INSERT | MODE_CMDLINE
 * for :cmap  mode is MODE_CMDLINE
 * for :imap  mode is MODE_INSERT
 * for :lmap  mode is MODE_LANGMAP
 * for :nmap  mode is MODE_NORMAL
 * for :vmap  mode is MODE_VISUAL | MODE_SELECT
 * for :xmap  mode is MODE_VISUAL
 * for :smap  mode is MODE_SELECT
 * for :omap  mode is MODE_OP_PENDING
 * for :tmap  mode is MODE_TERMINAL
 *
 * for :abbr  mode is MODE_INSERT | MODE_CMDLINE
 * for :iabbr mode is MODE_INSERT
 * for :cabbr mode is MODE_CMDLINE
 *
 * Return 0 for success
 *	  1 for invalid arguments
 *	  2 for no match
 *	  4 for out of mem
 *	  5 for entry not unique
 */
    int
do_map(
    int		maptype,
    char_u	*arg,
    int		mode,
    int		abbrev)		// not a mapping but an abbreviation
{
    char_u	*keys;
    mapblock_T	*mp, **mpp;
    char_u	*rhs;
    char_u	*p;
    int		n;
    int		len = 0;	// init for GCC
    int		hasarg;
    int		haskey;
    int		do_print;
    int		keyround;
    char_u	*keys_buf = NULL;
    char_u	*alt_keys_buf = NULL;
    char_u	*arg_buf = NULL;
    int		retval = 0;
    int		do_backslash;
    mapblock_T	**abbr_table;
    mapblock_T	**map_table;
    int		unique = FALSE;
    int		nowait = FALSE;
    int		silent = FALSE;
    int		special = FALSE;
#ifdef FEAT_EVAL
    int		expr = FALSE;
#endif
    int		did_simplify = FALSE;
    int		noremap;
    char_u      *orig_rhs;

    keys = arg;
    map_table = maphash;
    abbr_table = &first_abbr;

    // For ":noremap" don't remap, otherwise do remap.
    if (maptype == MAPTYPE_NOREMAP)
	noremap = REMAP_NONE;
    else
	noremap = REMAP_YES;

    // Accept <buffer>, <nowait>, <silent>, <expr> <script> and <unique> in
    // any order.
    for (;;)
    {
	// Check for "<buffer>": mapping local to buffer.
	if (STRNCMP(keys, "<buffer>", 8) == 0)
	{
	    keys = skipwhite(keys + 8);
	    map_table = curbuf->b_maphash;
	    abbr_table = &curbuf->b_first_abbr;
	    continue;
	}

	// Check for "<nowait>": don't wait for more characters.
	if (STRNCMP(keys, "<nowait>", 8) == 0)
	{
	    keys = skipwhite(keys + 8);
	    nowait = TRUE;
	    continue;
	}

	// Check for "<silent>": don't echo commands.
	if (STRNCMP(keys, "<silent>", 8) == 0)
	{
	    keys = skipwhite(keys + 8);
	    silent = TRUE;
	    continue;
	}

	// Check for "<special>": accept special keys in <>
	if (STRNCMP(keys, "<special>", 9) == 0)
	{
	    keys = skipwhite(keys + 9);
	    special = TRUE;
	    continue;
	}

#ifdef FEAT_EVAL
	// Check for "<script>": remap script-local mappings only
	if (STRNCMP(keys, "<script>", 8) == 0)
	{
	    keys = skipwhite(keys + 8);
	    noremap = REMAP_SCRIPT;
	    continue;
	}

	// Check for "<expr>": {rhs} is an expression.
	if (STRNCMP(keys, "<expr>", 6) == 0)
	{
	    keys = skipwhite(keys + 6);
	    expr = TRUE;
	    continue;
	}
#endif
	// Check for "<unique>": don't overwrite an existing mapping.
	if (STRNCMP(keys, "<unique>", 8) == 0)
	{
	    keys = skipwhite(keys + 8);
	    unique = TRUE;
	    continue;
	}
	break;
    }

    validate_maphash();

    // Find end of keys and skip CTRL-Vs (and backslashes) in it.
    // Accept backslash like CTRL-V when 'cpoptions' does not contain 'B'.
    // with :unmap white space is included in the keys, no argument possible.
    p = keys;
    do_backslash = (vim_strchr(p_cpo, CPO_BSLASH) == NULL);
    while (*p && (maptype == MAPTYPE_UNMAP || !VIM_ISWHITE(*p)))
    {
	if ((p[0] == Ctrl_V || (do_backslash && p[0] == '\\')) &&
								  p[1] != NUL)
	    ++p;		// skip CTRL-V or backslash
	++p;
    }
    if (*p != NUL)
	*p++ = NUL;

    p = skipwhite(p);
    rhs = p;
    hasarg = (*rhs != NUL);
    haskey = (*keys != NUL);
    do_print = !haskey || (maptype != MAPTYPE_UNMAP && !hasarg);

    // check for :unmap without argument
    if (maptype == MAPTYPE_UNMAP && !haskey)
    {
	retval = 1;
	goto theend;
    }

    // If mapping has been given as ^V<C_UP> say, then replace the term codes
    // with the appropriate two bytes. If it is a shifted special key, unshift
    // it too, giving another two bytes.
    // replace_termcodes() may move the result to allocated memory, which
    // needs to be freed later (*keys_buf and *arg_buf).
    // replace_termcodes() also removes CTRL-Vs and sometimes backslashes.
    // If something like <C-H> is simplified to 0x08 then mark it as simplified
    // and also add an entry with a modifier, which will work when using a key
    // protocol.
    if (haskey)
    {
	char_u	*new_keys;
	int	flags = REPTERM_FROM_PART | REPTERM_DO_LT;

	if (special)
	    flags |= REPTERM_SPECIAL;
	new_keys = replace_termcodes(keys, &keys_buf, 0, flags, &did_simplify);
	if (did_simplify)
	    (void)replace_termcodes(keys, &alt_keys_buf, 0,
					    flags | REPTERM_NO_SIMPLIFY, NULL);
	keys = new_keys;
    }
    orig_rhs = rhs;
    if (hasarg)
    {
	if (STRICMP(rhs, "<nop>") == 0)	    // "<Nop>" means nothing
	    rhs = (char_u *)"";
	else
	    rhs = replace_termcodes(rhs, &arg_buf, 0,
			REPTERM_DO_LT | (special ? REPTERM_SPECIAL : 0), NULL);
    }

    /*
     * The following is done twice if we have two versions of keys:
     * "alt_keys_buf" is not NULL.
     */
    for (keyround = 1; keyround <= 2; ++keyround)
    {
	int	did_it = FALSE;
	int	did_local = FALSE;
	int	keyround1_simplified = keyround == 1 && did_simplify;
	int	round;

	if (keyround == 2)
	{
	    if (alt_keys_buf == NULL)
		break;
	    keys = alt_keys_buf;
	}
	else if (alt_keys_buf != NULL && do_print)
	    // when printing always use the not-simplified map
	    keys = alt_keys_buf;

	// check arguments and translate function keys
	if (haskey)
	{
	    len = (int)STRLEN(keys);
	    if (len > MAXMAPLEN)	// maximum length of MAXMAPLEN chars
	    {
		retval = 1;
		goto theend;
	    }

	    if (abbrev && maptype != MAPTYPE_UNMAP)
	    {
		// If an abbreviation ends in a keyword character, the
		// rest must be all keyword-char or all non-keyword-char.
		// Otherwise we won't be able to find the start of it in a
		// vi-compatible way.
		if (has_mbyte)
		{
		    int	first, last;
		    int	same = -1;

		    first = vim_iswordp(keys);
		    last = first;
		    p = keys + (*mb_ptr2len)(keys);
		    n = 1;
		    while (p < keys + len)
		    {
			++n;			// nr of (multi-byte) chars
			last = vim_iswordp(p);	// type of last char
			if (same == -1 && last != first)
			    same = n - 1;	// count of same char type
			p += (*mb_ptr2len)(p);
		    }
		    if (last && n > 2 && same >= 0 && same < n - 1)
		    {
			retval = 1;
			goto theend;
		    }
		}
		else if (vim_iswordc(keys[len - 1]))
		    // ends in keyword char
		    for (n = 0; n < len - 2; ++n)
			if (vim_iswordc(keys[n]) != vim_iswordc(keys[len - 2]))
			{
			    retval = 1;
			    goto theend;
			}
		// An abbreviation cannot contain white space.
		for (n = 0; n < len; ++n)
		    if (VIM_ISWHITE(keys[n]))
		    {
			retval = 1;
			goto theend;
		    }
	    }
	}

	if (haskey && hasarg && abbrev)	// if we will add an abbreviation
	    no_abbr = FALSE;		// reset flag that indicates there are
					// no abbreviations

	if (do_print)
	    msg_start();

	// Check if a new local mapping wasn't already defined globally.
	if (unique && map_table == curbuf->b_maphash
			       && haskey && hasarg && maptype != MAPTYPE_UNMAP)
	{
	    // need to loop over all global hash lists
	    for (int hash = 0; hash < 256 && !got_int; ++hash)
	    {
		if (abbrev)
		{
		    if (hash != 0)	// there is only one abbreviation list
			break;
		    mp = first_abbr;
		}
		else
		    mp = maphash[hash];
		for ( ; mp != NULL && !got_int; mp = mp->m_next)
		{
		    // check entries with the same mode
		    if ((mp->m_mode & mode) != 0
			    && mp->m_keylen == len
			    && STRNCMP(mp->m_keys, keys, (size_t)len) == 0)
		    {
			if (abbrev)
			    semsg(
			       _(e_global_abbreviation_already_exists_for_str),
				    mp->m_keys);
			else
			    semsg(_(e_global_mapping_already_exists_for_str),
				    mp->m_keys);
			retval = 5;
			goto theend;
		    }
		}
	    }
	}

	// When listing global mappings, also list buffer-local ones here.
	if (map_table != curbuf->b_maphash && !hasarg
						   && maptype != MAPTYPE_UNMAP)
	    list_mappings(keyround, abbrev, haskey, keys, len,
							     mode, &did_local);

	// Find an entry in the maphash[] list that matches.
	// For :unmap we may loop two times: once to try to unmap an entry with
	// a matching 'from' part, a second time, if the first fails, to unmap
	// an entry with a matching 'to' part. This was done to allow
	// ":ab foo bar" to be unmapped by typing ":unab foo", where "foo" will
	// be replaced by "bar" because of the abbreviation.
	for (round = 0; (round == 0 || maptype == MAPTYPE_UNMAP) && round <= 1
					       && !did_it && !got_int; ++round)
	{
	    // need to loop over all hash lists
	    for (int hash = 0; hash < 256 && !got_int; ++hash)
	    {
		if (abbrev)
		{
		    if (hash > 0)	// there is only one abbreviation list
			break;
		    mpp = abbr_table;
		}
		else
		    mpp = &(map_table[hash]);
		for (mp = *mpp; mp != NULL && !got_int; mp = *mpp)
		{

		    if ((mp->m_mode & mode) == 0)
		    {
			// skip entries with wrong mode
			mpp = &(mp->m_next);
			continue;
		    }
		    if (!haskey)	// show all entries
		    {
			if (!mp->m_simplified)
			{
			    showmap(mp, map_table != maphash);
			    did_it = TRUE;
			}
		    }
		    else	// do we have a match?
		    {
			if (round)	// second round: Try unmap "rhs" string
			{
			    n = (int)STRLEN(mp->m_str);
			    p = mp->m_str;
			}
			else
			{
			    n = mp->m_keylen;
			    p = mp->m_keys;
			}
			if (STRNCMP(p, keys, (size_t)(n < len ? n : len)) == 0)
			{
			    if (maptype == MAPTYPE_UNMAP)
			    {
				// Delete entry.
				// Only accept a full match.  For abbreviations
				// we ignore trailing space when matching with
				// the "lhs", since an abbreviation can't have
				// trailing space.
				if (n != len && (!abbrev || round || n > len
					       || *skipwhite(keys + n) != NUL))
				{
				    mpp = &(mp->m_next);
				    continue;
				}
				// In keyround for simplified keys, don't unmap
				// a mapping without m_simplified flag.
				if (keyround1_simplified && !mp->m_simplified)
				    break;
				// We reset the indicated mode bits. If nothing
				// is left the entry is deleted below.
				mp->m_mode &= ~mode;
				did_it = TRUE;	// remember we did something
			    }
			    else if (!hasarg)	// show matching entry
			    {
				if (!mp->m_simplified)
				{
				    showmap(mp, map_table != maphash);
				    did_it = TRUE;
				}
			    }
			    else if (n != len)	// new entry is ambiguous
			    {
				mpp = &(mp->m_next);
				continue;
			    }
			    else if (unique)
			    {
				if (abbrev)
				    semsg(
				      _(e_abbreviation_already_exists_for_str),
					    p);
				else
				    semsg(_(e_mapping_already_exists_for_str),
					    p);
				retval = 5;
				goto theend;
			    }
			    else
			    {
				// new rhs for existing entry
				mp->m_mode &= ~mode;	// remove mode bits
				if (mp->m_mode == 0 && !did_it) // reuse entry
				{
				    char_u *newstr = vim_strsave(rhs);

				    if (newstr == NULL)
				    {
					retval = 4;		// no mem
					goto theend;
				    }
				    vim_free(mp->m_str);
				    mp->m_str = newstr;
				    vim_free(mp->m_orig_str);
				    mp->m_orig_str = vim_strsave(orig_rhs);
				    mp->m_noremap = noremap;
				    mp->m_nowait = nowait;
				    mp->m_silent = silent;
				    mp->m_mode = mode;
				    mp->m_simplified = keyround1_simplified;
#ifdef FEAT_EVAL
				    mp->m_expr = expr;
				    mp->m_script_ctx = current_sctx;
				    mp->m_script_ctx.sc_lnum += SOURCING_LNUM;
#endif
				    did_it = TRUE;
				}
			    }
			    if (mp->m_mode == 0)  // entry can be deleted
			    {
				map_free(mpp);
				continue;	// continue with *mpp
			    }

			    // May need to put this entry into another hash
			    // list.
			    int new_hash = MAP_HASH(mp->m_mode, mp->m_keys[0]);
			    if (!abbrev && new_hash != hash)
			    {
				*mpp = mp->m_next;
				mp->m_next = map_table[new_hash];
				map_table[new_hash] = mp;

				continue;	// continue with *mpp
			    }
			}
		    }
		    mpp = &(mp->m_next);
		}
	    }
	}

	if (maptype == MAPTYPE_UNMAP)
	{
	    // delete entry
	    if (!did_it)
	    {
		if (!keyround1_simplified)
		    retval = 2;		// no match
	    }
	    else if (*keys == Ctrl_C)
	    {
		// If CTRL-C has been unmapped, reuse it for Interrupting.
		if (map_table == curbuf->b_maphash)
		    curbuf->b_mapped_ctrl_c &= ~mode;
		else
		    mapped_ctrl_c &= ~mode;
	    }
	    continue;
	}

	if (!haskey || !hasarg)
	{
	    // print entries
	    if (!did_it && !did_local)
	    {
		if (abbrev)
		    msg(_("No abbreviation found"));
		else
		    msg(_("No mapping found"));
	    }
	    goto theend;    // listing finished
	}

	if (did_it)
	    continue;	// have added the new entry already

	// Get here when adding a new entry to the maphash[] list or abbrlist.
	if (map_add(map_table, abbr_table, keys, rhs, orig_rhs,
		    noremap, nowait, silent, mode, abbrev,
#ifdef FEAT_EVAL
		    expr, /* sid */ 0, /* scriptversion */ 0, /* lnum */ 0,
#endif
		    keyround1_simplified) == FAIL)
	{
	    retval = 4;	    // no mem
	    goto theend;
	}
    }

theend:
    vim_free(keys_buf);
    vim_free(alt_keys_buf);
    vim_free(arg_buf);
    return retval;
}

/*
 * Get the mapping mode from the command name.
 */
    static int
get_map_mode(char_u **cmdp, int forceit)
{
    char_u	*p;
    int		modec;
    int		mode;

    p = *cmdp;
    modec = *p++;
    if (modec == 'i')
	mode = MODE_INSERT;				// :imap
    else if (modec == 'l')
	mode = MODE_LANGMAP;				// :lmap
    else if (modec == 'c')
	mode = MODE_CMDLINE;				// :cmap
    else if (modec == 'n' && *p != 'o')		    // avoid :noremap
	mode = MODE_NORMAL;				// :nmap
    else if (modec == 'v')
	mode = MODE_VISUAL | MODE_SELECT;		// :vmap
    else if (modec == 'x')
	mode = MODE_VISUAL;				// :xmap
    else if (modec == 's')
	mode = MODE_SELECT;			// :smap
    else if (modec == 'o')
	mode = MODE_OP_PENDING;			// :omap
    else if (modec == 't')
	mode = MODE_TERMINAL;			// :tmap
    else
    {
	--p;
	if (forceit)
	    mode = MODE_INSERT | MODE_CMDLINE;		// :map !
	else
	    mode = MODE_VISUAL | MODE_SELECT | MODE_NORMAL | MODE_OP_PENDING;
							// :map
    }

    *cmdp = p;
    return mode;
}

/*
 * Clear all mappings (":mapclear") or abbreviations (":abclear").
 * "abbr" should be FALSE for mappings, TRUE for abbreviations.
 */
    static void
map_clear(
    char_u	*cmdp,
    char_u	*arg,
    int		forceit,
    int		abbr)
{
    int		mode;
    int		local;

    local = (STRCMP(arg, "<buffer>") == 0);
    if (!local && *arg != NUL)
    {
	emsg(_(e_invalid_argument));
	return;
    }

    mode = get_map_mode(&cmdp, forceit);
    map_clear_mode(curbuf, mode, local, abbr);
}

/*
 * If "map_locked" is set then give an error and return TRUE.
 * Otherwise return FALSE.
 */
    static int
is_map_locked(void)
{
    if (map_locked > 0)
    {
	emsg(_(e_cannot_change_mappings_while_listing));
	return TRUE;
    }
    return FALSE;
}

/*
 * Clear all mappings in "mode".
 */
    void
map_clear_mode(
    buf_T	*buf,		// buffer for local mappings
    int		mode,		// mode in which to delete
    int		local,		// TRUE for buffer-local mappings
    int		abbr)		// TRUE for abbreviations
{
    mapblock_T	*mp, **mpp;
    int		hash;
    int		new_hash;

    if (is_map_locked())
	return;

    validate_maphash();

    for (hash = 0; hash < 256; ++hash)
    {
	if (abbr)
	{
	    if (hash > 0)	// there is only one abbrlist
		break;
	    if (local)
		mpp = &buf->b_first_abbr;
	    else
		mpp = &first_abbr;
	}
	else
	{
	    if (local)
		mpp = &buf->b_maphash[hash];
	    else
		mpp = &maphash[hash];
	}
	while (*mpp != NULL)
	{
	    mp = *mpp;
	    if (mp->m_mode & mode)
	    {
		mp->m_mode &= ~mode;
		if (mp->m_mode == 0) // entry can be deleted
		{
		    map_free(mpp);
		    continue;
		}
		// May need to put this entry into another hash list.
		new_hash = MAP_HASH(mp->m_mode, mp->m_keys[0]);
		if (!abbr && new_hash != hash)
		{
		    *mpp = mp->m_next;
		    if (local)
		    {
			mp->m_next = buf->b_maphash[new_hash];
			buf->b_maphash[new_hash] = mp;
		    }
		    else
		    {
			mp->m_next = maphash[new_hash];
			maphash[new_hash] = mp;
		    }
		    continue;		// continue with *mpp
		}
	    }
	    mpp = &(mp->m_next);
	}
    }
}

#if defined(FEAT_EVAL) || defined(PROTO)
    int
mode_str2flags(char_u *modechars)
{
    int		mode = 0;

    if (vim_strchr(modechars, 'n') != NULL)
	mode |= MODE_NORMAL;
    if (vim_strchr(modechars, 'v') != NULL)
	mode |= MODE_VISUAL | MODE_SELECT;
    if (vim_strchr(modechars, 'x') != NULL)
	mode |= MODE_VISUAL;
    if (vim_strchr(modechars, 's') != NULL)
	mode |= MODE_SELECT;
    if (vim_strchr(modechars, 'o') != NULL)
	mode |= MODE_OP_PENDING;
    if (vim_strchr(modechars, 'i') != NULL)
	mode |= MODE_INSERT;
    if (vim_strchr(modechars, 'l') != NULL)
	mode |= MODE_LANGMAP;
    if (vim_strchr(modechars, 'c') != NULL)
	mode |= MODE_CMDLINE;

    return mode;
}

/*
 * Return TRUE if a map exists that has "str" in the rhs for mode "modechars".
 * Recognize termcap codes in "str".
 * Also checks mappings local to the current buffer.
 */
    int
map_to_exists(char_u *str, char_u *modechars, int abbr)
{
    char_u	*rhs;
    char_u	*buf;
    int		retval;

    rhs = replace_termcodes(str, &buf, 0, REPTERM_DO_LT, NULL);

    retval = map_to_exists_mode(rhs, mode_str2flags(modechars), abbr);
    vim_free(buf);

    return retval;
}
#endif

/*
 * Return TRUE if a map exists that has "str" in the rhs for mode "mode".
 * Also checks mappings local to the current buffer.
 */
    int
map_to_exists_mode(char_u *rhs, int mode, int abbr)
{
    mapblock_T	*mp;
    int		hash;
    int		exp_buffer = FALSE;

    validate_maphash();

    // Do it twice: once for global maps and once for local maps.
    for (;;)
    {
	for (hash = 0; hash < 256; ++hash)
	{
	    if (abbr)
	    {
		if (hash > 0)		// there is only one abbr list
		    break;
		if (exp_buffer)
		    mp = curbuf->b_first_abbr;
		else
		    mp = first_abbr;
	    }
	    else if (exp_buffer)
		mp = curbuf->b_maphash[hash];
	    else
		mp = maphash[hash];
	    for (; mp; mp = mp->m_next)
	    {
		if ((mp->m_mode & mode)
			&& strstr((char *)mp->m_str, (char *)rhs) != NULL)
		    return TRUE;
	    }
	}
	if (exp_buffer)
	    break;
	exp_buffer = TRUE;
    }

    return FALSE;
}

/*
 * Used below when expanding mapping/abbreviation names.
 */
static int	expand_mapmodes = 0;
static int	expand_isabbrev = 0;
static int	expand_buffer = FALSE;

/*
 * Translate an internal mapping/abbreviation representation into the
 * corresponding external one recognized by :map/:abbrev commands.
 * Respects the current B/k/< settings of 'cpoption'.
 *
 * This function is called when expanding mappings/abbreviations on the
 * command-line.
 *
 * It uses a growarray to build the translation string since the latter can be
 * wider than the original description. The caller has to free the string
 * afterwards.
 *
 * Returns NULL when there is a problem.
 */
    static char_u *
translate_mapping(char_u *str)
{
    garray_T	ga;
    int		c;
    int		modifiers;
    int		cpo_bslash;
    int		cpo_special;

    ga_init(&ga);
    ga.ga_itemsize = 1;
    ga.ga_growsize = 40;

    cpo_bslash = (vim_strchr(p_cpo, CPO_BSLASH) != NULL);
    cpo_special = (vim_strchr(p_cpo, CPO_SPECI) != NULL);

    for (; *str; ++str)
    {
	c = *str;
	if (c == K_SPECIAL && str[1] != NUL && str[2] != NUL)
	{
	    modifiers = 0;
	    if (str[1] == KS_MODIFIER)
	    {
		str++;
		modifiers = *++str;
		c = *++str;
	    }
	    if (c == K_SPECIAL && str[1] != NUL && str[2] != NUL)
	    {
		if (cpo_special)
		{
		    ga_clear(&ga);
		    return NULL;
		}
		c = TO_SPECIAL(str[1], str[2]);
		if (c == K_ZERO)	// display <Nul> as ^@
		    c = NUL;
		str += 2;
	    }
	    if (IS_SPECIAL(c) || modifiers)	// special key
	    {
		if (cpo_special)
		{
		    ga_clear(&ga);
		    return NULL;
		}
		ga_concat(&ga, get_special_key_name(c, modifiers));
		continue; // for (str)
	    }
	}
	if (c == ' ' || c == '\t' || c == Ctrl_J || c == Ctrl_V
	    || (c == '<' && !cpo_special) || (c == '\\' && !cpo_bslash))
	    ga_append(&ga, cpo_bslash ? Ctrl_V : '\\');
	if (c)
	    ga_append(&ga, c);
    }
    ga_append(&ga, NUL);
    return (char_u *)(ga.ga_data);
}

/*
 * Work out what to complete when doing command line completion of mapping
 * or abbreviation names.
 */
    char_u *
set_context_in_map_cmd(
    expand_T	*xp,
    char_u	*cmd,
    char_u	*arg,
    int		forceit,	// TRUE if '!' given
    int		isabbrev,	// TRUE if abbreviation
    int		isunmap,	// TRUE if unmap/unabbrev command
    cmdidx_T	cmdidx)
{
    if (forceit && cmdidx != CMD_map && cmdidx != CMD_unmap)
	xp->xp_context = EXPAND_NOTHING;
    else
    {
	if (isunmap)
	    expand_mapmodes = get_map_mode(&cmd, forceit || isabbrev);
	else
	{
	    expand_mapmodes = MODE_INSERT | MODE_CMDLINE;
	    if (!isabbrev)
		expand_mapmodes += MODE_VISUAL | MODE_SELECT | MODE_NORMAL
							     | MODE_OP_PENDING;
	}
	expand_isabbrev = isabbrev;
	xp->xp_context = EXPAND_MAPPINGS;
	expand_buffer = FALSE;
	for (;;)
	{
	    if (STRNCMP(arg, "<buffer>", 8) == 0)
	    {
		expand_buffer = TRUE;
		arg = skipwhite(arg + 8);
		continue;
	    }
	    if (STRNCMP(arg, "<unique>", 8) == 0)
	    {
		arg = skipwhite(arg + 8);
		continue;
	    }
	    if (STRNCMP(arg, "<nowait>", 8) == 0)
	    {
		arg = skipwhite(arg + 8);
		continue;
	    }
	    if (STRNCMP(arg, "<silent>", 8) == 0)
	    {
		arg = skipwhite(arg + 8);
		continue;
	    }
	    if (STRNCMP(arg, "<special>", 9) == 0)
	    {
		arg = skipwhite(arg + 9);
		continue;
	    }
#ifdef FEAT_EVAL
	    if (STRNCMP(arg, "<script>", 8) == 0)
	    {
		arg = skipwhite(arg + 8);
		continue;
	    }
	    if (STRNCMP(arg, "<expr>", 6) == 0)
	    {
		arg = skipwhite(arg + 6);
		continue;
	    }
#endif
	    break;
	}
	xp->xp_pattern = arg;
    }

    return NULL;
}

/*
 * Find all mapping/abbreviation names that match regexp "regmatch"'.
 * For command line expansion of ":[un]map" and ":[un]abbrev" in all modes.
 * Return OK if matches found, FAIL otherwise.
 */
    int
ExpandMappings(
    char_u	*pat,
    regmatch_T	*regmatch,
    int		*numMatches,
    char_u	***matches)
{
    mapblock_T	*mp;
    garray_T	ga;
    int		hash;
    int		count;
    char_u	*p;
    int		i;
    int		fuzzy;
    int		match;
    int		score = 0;
    fuzmatch_str_T  *fuzmatch;

    fuzzy = cmdline_fuzzy_complete(pat);

    validate_maphash();

    *numMatches = 0;		    // return values in case of FAIL
    *matches = NULL;

    if (!fuzzy)
	ga_init2(&ga, sizeof(char *), 3);
    else
	ga_init2(&ga, sizeof(fuzmatch_str_T), 3);

    // First search in map modifier arguments
    for (i = 0; i < 7; ++i)
    {
	if (i == 0)
	    p = (char_u *)"<silent>";
	else if (i == 1)
	    p = (char_u *)"<unique>";
#ifdef FEAT_EVAL
	else if (i == 2)
	    p = (char_u *)"<script>";
	else if (i == 3)
	    p = (char_u *)"<expr>";
#endif
	else if (i == 4 && !expand_buffer)
	    p = (char_u *)"<buffer>";
	else if (i == 5)
	    p = (char_u *)"<nowait>";
	else if (i == 6)
	    p = (char_u *)"<special>";
	else
	    continue;

	if (!fuzzy)
	    match = vim_regexec(regmatch, p, (colnr_T)0);
	else
	{
	    score = fuzzy_match_str(p, pat);
	    match = (score != 0);
	}

	if (!match)
	    continue;

	if (ga_grow(&ga, 1) == FAIL)
	    break;

	if (fuzzy)
	{
	    fuzmatch = &((fuzmatch_str_T *)ga.ga_data)[ga.ga_len];
	    fuzmatch->idx = ga.ga_len;
	    fuzmatch->str = vim_strsave(p);
	    fuzmatch->score = score;
	}
	else
	    ((char_u **)ga.ga_data)[ga.ga_len] = vim_strsave(p);
	++ga.ga_len;
    }

    for (hash = 0; hash < 256; ++hash)
    {
	if (expand_isabbrev)
	{
	    if (hash > 0)	// only one abbrev list
		break; // for (hash)
	    mp = first_abbr;
	}
	else if (expand_buffer)
	    mp = curbuf->b_maphash[hash];
	else
	    mp = maphash[hash];
	for (; mp; mp = mp->m_next)
	{
	    if (mp->m_simplified || !(mp->m_mode & expand_mapmodes))
		continue;

	    p = translate_mapping(mp->m_keys);
	    if (p == NULL)
		continue;

	    if (!fuzzy)
		match = vim_regexec(regmatch, p, (colnr_T)0);
	    else
	    {
		score = fuzzy_match_str(p, pat);
		match = (score != 0);
	    }

	    if (!match)
	    {
		vim_free(p);
		continue;
	    }

	    if (ga_grow(&ga, 1) == FAIL)
	    {
		vim_free(p);
		break;
	    }

	    if (fuzzy)
	    {
		fuzmatch = &((fuzmatch_str_T *)ga.ga_data)[ga.ga_len];
		fuzmatch->idx = ga.ga_len;
		fuzmatch->str = p;
		fuzmatch->score = score;
	    }
	    else
		((char_u **)ga.ga_data)[ga.ga_len] = p;

	    ++ga.ga_len;
	} // for (mp)
    } // for (hash)

    if (ga.ga_len == 0)
	return FAIL;

    if (!fuzzy)
    {
	*matches = ga.ga_data;
	*numMatches = ga.ga_len;
    }
    else
    {
	if (fuzzymatches_to_strmatches(ga.ga_data, matches, ga.ga_len,
							FALSE) == FAIL)
	    return FAIL;
	*numMatches = ga.ga_len;
    }

    count = *numMatches;
    if (count > 1)
    {
	char_u	**ptr1;
	char_u	**ptr2;
	char_u	**ptr3;

	// Sort the matches
	// Fuzzy matching already sorts the matches
	if (!fuzzy)
	    sort_strings(*matches, count);

	// Remove multiple entries
	ptr1 = *matches;
	ptr2 = ptr1 + 1;
	ptr3 = ptr1 + count;

	while (ptr2 < ptr3)
	{
	    if (STRCMP(*ptr1, *ptr2))
		*++ptr1 = *ptr2++;
	    else
	    {
		vim_free(*ptr2++);
		count--;
	    }
	}
    }

    *numMatches = count;
    return (count == 0 ? FAIL : OK);
}

/*
 * Check for an abbreviation.
 * Cursor is at ptr[col].
 * When inserting, mincol is where insert started.
 * For the command line, mincol is what is to be skipped over.
 * "c" is the character typed before check_abbr was called.  It may have
 * ABBR_OFF added to avoid prepending a CTRL-V to it.
 *
 * Historic vi practice: The last character of an abbreviation must be an id
 * character ([a-zA-Z0-9_]). The characters in front of it must be all id
 * characters or all non-id characters. This allows for abbr. "#i" to
 * "#include".
 *
 * Vim addition: Allow for abbreviations that end in a non-keyword character.
 * Then there must be white space before the abbr.
 *
 * return TRUE if there is an abbreviation, FALSE if not
 */
    int
check_abbr(
    int		c,
    char_u	*ptr,
    int		col,
    int		mincol)
{
    int		len;
    int		scol;		// starting column of the abbr.
    int		j;
    char_u	*s;
    char_u	tb[MB_MAXBYTES + 4];
    mapblock_T	*mp;
    mapblock_T	*mp2;
    int		clen = 0;	// length in characters
    int		is_id = TRUE;
    int		vim_abbr;

    if (typebuf.tb_no_abbr_cnt)	// abbrev. are not recursive
	return FALSE;

    // no remapping implies no abbreviation, except for CTRL-]
    if (noremap_keys() && c != Ctrl_RSB)
	return FALSE;

    // Check for word before the cursor: If it ends in a keyword char all
    // chars before it must be keyword chars or non-keyword chars, but not
    // white space. If it ends in a non-keyword char we accept any characters
    // before it except white space.
    if (col == 0)				// cannot be an abbr.
	return FALSE;

    if (has_mbyte)
    {
	char_u *p;

	p = mb_prevptr(ptr, ptr + col);
	if (!vim_iswordp(p))
	    vim_abbr = TRUE;			// Vim added abbr.
	else
	{
	    vim_abbr = FALSE;			// vi compatible abbr.
	    if (p > ptr)
		is_id = vim_iswordp(mb_prevptr(ptr, p));
	}
	clen = 1;
	while (p > ptr + mincol)
	{
	    p = mb_prevptr(ptr, p);
	    if (vim_isspace(*p) || (!vim_abbr && is_id != vim_iswordp(p)))
	    {
		p += (*mb_ptr2len)(p);
		break;
	    }
	    ++clen;
	}
	scol = (int)(p - ptr);
    }
    else
    {
	if (!vim_iswordc(ptr[col - 1]))
	    vim_abbr = TRUE;			// Vim added abbr.
	else
	{
	    vim_abbr = FALSE;			// vi compatible abbr.
	    if (col > 1)
		is_id = vim_iswordc(ptr[col - 2]);
	}
	for (scol = col - 1; scol > 0 && !vim_isspace(ptr[scol - 1])
		&& (vim_abbr || is_id == vim_iswordc(ptr[scol - 1])); --scol)
	    ;
    }

    if (scol < mincol)
	scol = mincol;
    if (scol < col)		// there is a word in front of the cursor
    {
	ptr += scol;
	len = col - scol;
	mp = curbuf->b_first_abbr;
	mp2 = first_abbr;
	if (mp == NULL)
	{
	    mp = mp2;
	    mp2 = NULL;
	}
	for ( ; mp; mp->m_next == NULL
				  ? (mp = mp2, mp2 = NULL) : (mp = mp->m_next))
	{
	    int		qlen = mp->m_keylen;
	    char_u	*q = mp->m_keys;
	    int		match;

	    if (vim_strbyte(mp->m_keys, K_SPECIAL) != NULL)
	    {
		char_u *qe = vim_strsave(mp->m_keys);

		// might have CSI escaped mp->m_keys
		if (qe != NULL)
		{
		    q = qe;
		    vim_unescape_csi(q);
		    qlen = (int)STRLEN(q);
		}
	    }

	    // find entries with right mode and keys
	    match =    (mp->m_mode & State)
		    && qlen == len
		    && !STRNCMP(q, ptr, (size_t)len);
	    if (q != mp->m_keys)
		vim_free(q);
	    if (match)
		break;
	}
	if (mp != NULL)
	{
	    int	noremap;
	    int silent;
#ifdef FEAT_EVAL
	    int expr;
#endif

	    // Found a match:
	    // Insert the rest of the abbreviation in typebuf.tb_buf[].
	    // This goes from end to start.
	    //
	    // Characters 0x000 - 0x100: normal chars, may need CTRL-V,
	    // except K_SPECIAL: Becomes K_SPECIAL KS_SPECIAL KE_FILLER
	    // Characters where IS_SPECIAL() == TRUE: key codes, need
	    // K_SPECIAL. Other characters (with ABBR_OFF): don't use CTRL-V.
	    //
	    // Character CTRL-] is treated specially - it completes the
	    // abbreviation, but is not inserted into the input stream.
	    j = 0;
	    if (c != Ctrl_RSB)
	    {
					// special key code, split up
		if (IS_SPECIAL(c) || c == K_SPECIAL)
		{
		    tb[j++] = K_SPECIAL;
		    tb[j++] = K_SECOND(c);
		    tb[j++] = K_THIRD(c);
		}
		else
		{
		    if (c < ABBR_OFF && (c < ' ' || c > '~'))
			tb[j++] = Ctrl_V;	// special char needs CTRL-V
		    if (has_mbyte)
		    {
			int	newlen;
			char_u	*escaped;

			// if ABBR_OFF has been added, remove it here
			if (c >= ABBR_OFF)
			    c -= ABBR_OFF;
			newlen = (*mb_char2bytes)(c, tb + j);
			tb[j + newlen] = NUL;
			// Need to escape K_SPECIAL.
			escaped = vim_strsave_escape_csi(tb + j);
			if (escaped != NULL)
			{
			    newlen = (int)STRLEN(escaped);
			    mch_memmove(tb + j, escaped, newlen);
			    j += newlen;
			    vim_free(escaped);
			}
		    }
		    else
			tb[j++] = c;
		}
		tb[j] = NUL;
					// insert the last typed char
		(void)ins_typebuf(tb, 1, 0, TRUE, mp->m_silent);
	    }

	    // copy values here, calling eval_map_expr() may make "mp" invalid!
	    noremap = mp->m_noremap;
	    silent = mp->m_silent;
#ifdef FEAT_EVAL
	    expr = mp->m_expr;

	    if (expr)
		s = eval_map_expr(mp, c);
	    else
#endif
		s = mp->m_str;
	    if (s != NULL)
	    {
					// insert the to string
		(void)ins_typebuf(s, noremap, 0, TRUE, silent);
					// no abbrev. for these chars
		typebuf.tb_no_abbr_cnt += (int)STRLEN(s) + j + 1;
#ifdef FEAT_EVAL
		if (expr)
		    vim_free(s);
#endif
	    }

	    tb[0] = Ctrl_H;
	    tb[1] = NUL;
	    if (has_mbyte)
		len = clen;	// Delete characters instead of bytes
	    while (len-- > 0)		// delete the from string
		(void)ins_typebuf(tb, 1, 0, TRUE, silent);
	    return TRUE;
	}
    }
    return FALSE;
}

#ifdef FEAT_EVAL
/*
 * Evaluate the RHS of a mapping or abbreviations and take care of escaping
 * special characters.
 * Careful: after this "mp" will be invalid if the mapping was deleted.
 */
    char_u *
eval_map_expr(
    mapblock_T	*mp,
    int		c)	    // NUL or typed character for abbreviation
{
    char_u	*res;
    char_u	*p;
    char_u	*expr;
    pos_T	save_cursor;
    int		save_msg_col;
    int		save_msg_row;
    scid_T	save_sctx_sid = current_sctx.sc_sid;
    int		save_sctx_version = current_sctx.sc_version;

    // Remove escaping of CSI, because "str" is in a format to be used as
    // typeahead.
    expr = vim_strsave(mp->m_str);
    if (expr == NULL)
	return NULL;
    vim_unescape_csi(expr);

    // Forbid changing text or using ":normal" to avoid most of the bad side
    // effects.  Also restore the cursor position.
    ++textlock;
    ++ex_normal_lock;
    set_vim_var_char(c);  // set v:char to the typed character
    save_cursor = curwin->w_cursor;
    save_msg_col = msg_col;
    save_msg_row = msg_row;
    if (mp->m_script_ctx.sc_version == SCRIPT_VERSION_VIM9)
    {
	current_sctx.sc_sid = mp->m_script_ctx.sc_sid;
	current_sctx.sc_version = SCRIPT_VERSION_VIM9;
    }

    // Note: the evaluation may make "mp" invalid.
    p = eval_to_string(expr, FALSE, FALSE);

    --textlock;
    --ex_normal_lock;
    curwin->w_cursor = save_cursor;
    msg_col = save_msg_col;
    msg_row = save_msg_row;
    current_sctx.sc_sid = save_sctx_sid;
    current_sctx.sc_version = save_sctx_version;

    vim_free(expr);

    if (p == NULL)
	return NULL;
    // Escape CSI in the result to be able to use the string as typeahead.
    res = vim_strsave_escape_csi(p);
    vim_free(p);

    return res;
}
#endif

/*
 * Copy "p" to allocated memory, escaping K_SPECIAL and CSI so that the result
 * can be put in the typeahead buffer.
 * Returns NULL when out of memory.
 */
    char_u *
vim_strsave_escape_csi(char_u *p)
{
    char_u	*res;
    char_u	*s, *d;

    // Need a buffer to hold up to three times as much.  Four in case of an
    // illegal utf-8 byte:
    // 0xc0 -> 0xc3 0x80 -> 0xc3 K_SPECIAL KS_SPECIAL KE_FILLER
    res = alloc(STRLEN(p) * 4 + 1);
    if (res == NULL)
	return NULL;

    d = res;
    for (s = p; *s != NUL; )
    {
	if ((s[0] == K_SPECIAL
#ifdef FEAT_GUI
		    || (gui.in_use && s[0] == CSI)
#endif
	    ) && s[1] != NUL && s[2] != NUL)
	{
	    // Copy special key unmodified.
	    *d++ = *s++;
	    *d++ = *s++;
	    *d++ = *s++;
	}
	else
	{
	    // Add character, possibly multi-byte to destination, escaping
	    // CSI and K_SPECIAL. Be careful, it can be an illegal byte!
	    d = add_char2buf(PTR2CHAR(s), d);
	    s += MB_CPTR2LEN(s);
	}
    }
    *d = NUL;

    return res;
}

/*
 * Remove escaping from CSI and K_SPECIAL characters.  Reverse of
 * vim_strsave_escape_csi().  Works in-place.
 */
    void
vim_unescape_csi(char_u *p)
{
    char_u	*s = p, *d = p;

    while (*s != NUL)
    {
	if (s[0] == K_SPECIAL && s[1] == KS_SPECIAL && s[2] == KE_FILLER)
	{
	    *d++ = K_SPECIAL;
	    s += 3;
	}
	else if ((s[0] == K_SPECIAL || s[0] == CSI)
				   && s[1] == KS_EXTRA && s[2] == (int)KE_CSI)
	{
	    *d++ = CSI;
	    s += 3;
	}
	else
	    *d++ = *s++;
    }
    *d = NUL;
}

/*
 * Write map commands for the current mappings to an .exrc file.
 * Return FAIL on error, OK otherwise.
 */
    int
makemap(
    FILE	*fd,
    buf_T	*buf)	    // buffer for local mappings or NULL
{
    mapblock_T	*mp;
    char_u	c1, c2, c3;
    char_u	*p;
    char	*cmd;
    int		abbr;
    int		hash;
    int		did_cpo = FALSE;
    int		i;

    validate_maphash();

    // Do the loop twice: Once for mappings, once for abbreviations.
    // Then loop over all map hash lists.
    for (abbr = 0; abbr < 2; ++abbr)
	for (hash = 0; hash < 256; ++hash)
	{
	    if (abbr)
	    {
		if (hash > 0)		// there is only one abbr list
		    break;
		if (buf != NULL)
		    mp = buf->b_first_abbr;
		else
		    mp = first_abbr;
	    }
	    else
	    {
		if (buf != NULL)
		    mp = buf->b_maphash[hash];
		else
		    mp = maphash[hash];
	    }

	    for ( ; mp; mp = mp->m_next)
	    {
		// skip script-local mappings
		if (mp->m_noremap == REMAP_SCRIPT)
		    continue;

		// skip mappings that contain a <SNR> (script-local thing),
		// they probably don't work when loaded again
		for (p = mp->m_str; *p != NUL; ++p)
		    if (p[0] == K_SPECIAL && p[1] == KS_EXTRA
						       && p[2] == (int)KE_SNR)
			break;
		if (*p != NUL)
		    continue;

		// It's possible to create a mapping and then ":unmap" certain
		// modes.  We recreate this here by mapping the individual
		// modes, which requires up to three of them.
		c1 = NUL;
		c2 = NUL;
		c3 = NUL;
		if (abbr)
		    cmd = "abbr";
		else
		    cmd = "map";
		switch (mp->m_mode)
		{
		    case MODE_NORMAL | MODE_VISUAL | MODE_SELECT
							     | MODE_OP_PENDING:
			break;
		    case MODE_NORMAL:
			c1 = 'n';
			break;
		    case MODE_VISUAL:
			c1 = 'x';
			break;
		    case MODE_SELECT:
			c1 = 's';
			break;
		    case MODE_OP_PENDING:
			c1 = 'o';
			break;
		    case MODE_NORMAL | MODE_VISUAL:
			c1 = 'n';
			c2 = 'x';
			break;
		    case MODE_NORMAL | MODE_SELECT:
			c1 = 'n';
			c2 = 's';
			break;
		    case MODE_NORMAL | MODE_OP_PENDING:
			c1 = 'n';
			c2 = 'o';
			break;
		    case MODE_VISUAL | MODE_SELECT:
			c1 = 'v';
			break;
		    case MODE_VISUAL | MODE_OP_PENDING:
			c1 = 'x';
			c2 = 'o';
			break;
		    case MODE_SELECT | MODE_OP_PENDING:
			c1 = 's';
			c2 = 'o';
			break;
		    case MODE_NORMAL | MODE_VISUAL | MODE_SELECT:
			c1 = 'n';
			c2 = 'v';
			break;
		    case MODE_NORMAL | MODE_VISUAL | MODE_OP_PENDING:
			c1 = 'n';
			c2 = 'x';
			c3 = 'o';
			break;
		    case MODE_NORMAL | MODE_SELECT | MODE_OP_PENDING:
			c1 = 'n';
			c2 = 's';
			c3 = 'o';
			break;
		    case MODE_VISUAL | MODE_SELECT | MODE_OP_PENDING:
			c1 = 'v';
			c2 = 'o';
			break;
		    case MODE_CMDLINE | MODE_INSERT:
			if (!abbr)
			    cmd = "map!";
			break;
		    case MODE_CMDLINE:
			c1 = 'c';
			break;
		    case MODE_INSERT:
			c1 = 'i';
			break;
		    case MODE_LANGMAP:
			c1 = 'l';
			break;
		    case MODE_TERMINAL:
			c1 = 't';
			break;
		    default:
			iemsg(e_makemap_illegal_mode);
			return FAIL;
		}
		do	// do this twice if c2 is set, 3 times with c3
		{
		    // When outputting <> form, need to make sure that 'cpo'
		    // is set to the Vim default.
		    if (!did_cpo)
		    {
			if (*mp->m_str == NUL)		// will use <Nop>
			    did_cpo = TRUE;
			else
			    for (i = 0; i < 2; ++i)
				for (p = (i ? mp->m_str : mp->m_keys); *p; ++p)
				    if (*p == K_SPECIAL || *p == NL)
					did_cpo = TRUE;
			if (did_cpo)
			{
			    if (fprintf(fd, "let s:cpo_save=&cpo") < 0
				    || put_eol(fd) < 0
				    || fprintf(fd, "set cpo&vim") < 0
				    || put_eol(fd) < 0)
				return FAIL;
			}
		    }
		    if (c1 && putc(c1, fd) < 0)
			return FAIL;
		    if (mp->m_noremap != REMAP_YES && fprintf(fd, "nore") < 0)
			return FAIL;
		    if (fputs(cmd, fd) < 0)
			return FAIL;
		    if (buf != NULL && fputs(" <buffer>", fd) < 0)
			return FAIL;
		    if (mp->m_nowait && fputs(" <nowait>", fd) < 0)
			return FAIL;
		    if (mp->m_silent && fputs(" <silent>", fd) < 0)
			return FAIL;
#ifdef FEAT_EVAL
		    if (mp->m_noremap == REMAP_SCRIPT
						 && fputs("<script>", fd) < 0)
			return FAIL;
		    if (mp->m_expr && fputs(" <expr>", fd) < 0)
			return FAIL;
#endif

		    if (       putc(' ', fd) < 0
			    || put_escstr(fd, mp->m_keys, 0) == FAIL
			    || putc(' ', fd) < 0
			    || put_escstr(fd, mp->m_str, 1) == FAIL
			    || put_eol(fd) < 0)
			return FAIL;
		    c1 = c2;
		    c2 = c3;
		    c3 = NUL;
		} while (c1 != NUL);
	    }
	}

    if (did_cpo)
	if (fprintf(fd, "let &cpo=s:cpo_save") < 0
		|| put_eol(fd) < 0
		|| fprintf(fd, "unlet s:cpo_save") < 0
		|| put_eol(fd) < 0)
	    return FAIL;
    return OK;
}

/*
 * write escape string to file
 * "what": 0 for :map lhs, 1 for :map rhs, 2 for :set
 *
 * return FAIL for failure, OK otherwise
 */
    int
put_escstr(FILE *fd, char_u *strstart, int what)
{
    char_u	*str = strstart;
    int		c;
    int		modifiers;

    // :map xx <Nop>
    if (*str == NUL && what == 1)
    {
	if (fprintf(fd, "<Nop>") < 0)
	    return FAIL;
	return OK;
    }

    for ( ; *str != NUL; ++str)
    {
	char_u	*p;

	// Check for a multi-byte character, which may contain escaped
	// K_SPECIAL and CSI bytes
	p = mb_unescape(&str);
	if (p != NULL)
	{
	    while (*p != NUL)
		if (fputc(*p++, fd) < 0)
		    return FAIL;
	    --str;
	    continue;
	}

	c = *str;
	// Special key codes have to be translated to be able to make sense
	// when they are read back.
	if (c == K_SPECIAL && what != 2)
	{
	    modifiers = 0;
	    if (str[1] == KS_MODIFIER)
	    {
		modifiers = str[2];
		str += 3;
		c = *str;
	    }
	    if (c == K_SPECIAL)
	    {
		c = TO_SPECIAL(str[1], str[2]);
		str += 2;
	    }
	    if (IS_SPECIAL(c) || modifiers)	// special key
	    {
		if (fputs((char *)get_special_key_name(c, modifiers), fd) < 0)
		    return FAIL;
		continue;
	    }
	}

	// A '\n' in a map command should be written as <NL>.
	// A '\n' in a set command should be written as \^V^J.
	if (c == NL)
	{
	    if (what == 2)
	    {
		if (fprintf(fd, "\\\026\n") < 0)
		    return FAIL;
	    }
	    else
	    {
		if (fprintf(fd, "<NL>") < 0)
		    return FAIL;
	    }
	    continue;
	}

	// Some characters have to be escaped with CTRL-V to
	// prevent them from misinterpreted in DoOneCmd().
	// A space, Tab and '"' has to be escaped with a backslash to
	// prevent it to be misinterpreted in do_set().
	// A space has to be escaped with a CTRL-V when it's at the start of a
	// ":map" rhs.
	// A '<' has to be escaped with a CTRL-V to prevent it being
	// interpreted as the start of a special key name.
	// A space in the lhs of a :map needs a CTRL-V.
	if (what == 2 && (VIM_ISWHITE(c) || c == '"' || c == '\\'))
	{
	    if (putc('\\', fd) < 0)
		return FAIL;
	}
	else if (c < ' ' || c > '~' || c == '|'
		|| (what == 0 && c == ' ')
		|| (what == 1 && str == strstart && c == ' ')
		|| (what != 2 && c == '<'))
	{
	    if (putc(Ctrl_V, fd) < 0)
		return FAIL;
	}
	if (putc(c, fd) < 0)
	    return FAIL;
    }
    return OK;
}

/*
 * Check all mappings for the presence of special key codes.
 * Used after ":set term=xxx".
 */
    void
check_map_keycodes(void)
{
    mapblock_T	*mp;
    char_u	*p;
    int		i;
    char_u	buf[3];
    int		abbr;
    int		hash;
    buf_T	*bp;
    ESTACK_CHECK_DECLARATION;

    validate_maphash();
    // avoids giving error messages
    estack_push(ETYPE_INTERNAL, (char_u *)"mappings", 0);
    ESTACK_CHECK_SETUP;

    // Do this once for each buffer, and then once for global
    // mappings/abbreviations with bp == NULL
    for (bp = firstbuf; ; bp = bp->b_next)
    {
	// Do the loop twice: Once for mappings, once for abbreviations.
	// Then loop over all map hash lists.
	for (abbr = 0; abbr <= 1; ++abbr)
	    for (hash = 0; hash < 256; ++hash)
	    {
		if (abbr)
		{
		    if (hash)	    // there is only one abbr list
			break;
		    if (bp != NULL)
			mp = bp->b_first_abbr;
		    else
			mp = first_abbr;
		}
		else
		{
		    if (bp != NULL)
			mp = bp->b_maphash[hash];
		    else
			mp = maphash[hash];
		}
		for ( ; mp != NULL; mp = mp->m_next)
		{
		    for (i = 0; i <= 1; ++i)	// do this twice
		    {
			if (i == 0)
			    p = mp->m_keys;	// once for the "from" part
			else
			    p = mp->m_str;	// and once for the "to" part
			while (*p)
			{
			    if (*p == K_SPECIAL)
			    {
				++p;
				if (*p < 128)   // for "normal" tcap entries
				{
				    buf[0] = p[0];
				    buf[1] = p[1];
				    buf[2] = NUL;
				    (void)add_termcap_entry(buf, FALSE);
				}
				++p;
			    }
			    ++p;
			}
		    }
		}
	    }
	if (bp == NULL)
	    break;
    }
    ESTACK_CHECK_NOW;
    estack_pop();
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Check the string "keys" against the lhs of all mappings.
 * Return pointer to rhs of mapping (mapblock->m_str).
 * NULL when no mapping found.
 */
    char_u *
check_map(
    char_u	*keys,
    int		mode,
    int		exact,		// require exact match
    int		ign_mod,	// ignore preceding modifier
    int		abbr,		// do abbreviations
    mapblock_T	**mp_ptr,	// return: pointer to mapblock or NULL
    int		*local_ptr)	// return: buffer-local mapping or NULL
{
    int		hash;
    int		len, minlen;
    mapblock_T	*mp;
    char_u	*s;
    int		local;

    validate_maphash();

    len = (int)STRLEN(keys);
    for (local = 1; local >= 0; --local)
	// loop over all hash lists
	for (hash = 0; hash < 256; ++hash)
	{
	    if (abbr)
	    {
		if (hash > 0)		// there is only one list.
		    break;
		if (local)
		    mp = curbuf->b_first_abbr;
		else
		    mp = first_abbr;
	    }
	    else if (local)
		mp = curbuf->b_maphash[hash];
	    else
		mp = maphash[hash];
	    for ( ; mp != NULL; mp = mp->m_next)
	    {
		// skip entries with wrong mode, wrong length and not matching
		// ones
		if ((mp->m_mode & mode) && (!exact || mp->m_keylen == len))
		{
		    if (len > mp->m_keylen)
			minlen = mp->m_keylen;
		    else
			minlen = len;
		    s = mp->m_keys;
		    if (ign_mod && s[0] == K_SPECIAL && s[1] == KS_MODIFIER
							       && s[2] != NUL)
		    {
			s += 3;
			if (len > mp->m_keylen - 3)
			    minlen = mp->m_keylen - 3;
		    }
		    if (STRNCMP(s, keys, minlen) == 0)
		    {
			if (mp_ptr != NULL)
			    *mp_ptr = mp;
			if (local_ptr != NULL)
			    *local_ptr = local;
			return mp->m_str;
		    }
		}
	    }
	}

    return NULL;
}

/*
 * "hasmapto()" function
 */
    void
f_hasmapto(typval_T *argvars, typval_T *rettv)
{
    char_u	*name;
    char_u	*mode;
    char_u	buf[NUMBUFLEN];
    int		abbr = FALSE;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 2) == FAIL)))
	return;

    name = tv_get_string(&argvars[0]);
    if (argvars[1].v_type == VAR_UNKNOWN)
	mode = (char_u *)"nvo";
    else
    {
	mode = tv_get_string_buf(&argvars[1], buf);
	if (argvars[2].v_type != VAR_UNKNOWN)
	    abbr = (int)tv_get_bool(&argvars[2]);
    }

    if (map_to_exists(name, mode, abbr))
	rettv->vval.v_number = TRUE;
    else
	rettv->vval.v_number = FALSE;
}

/*
 * Fill in the empty dictionary with items as defined by maparg builtin.
 */
    static void
mapblock2dict(
	mapblock_T  *mp,
	dict_T	    *dict,
	char_u	    *lhsrawalt,	    // may be NULL
	int	    buffer_local,   // false if not buffer local mapping
	int	    abbr)	    // true if abbreviation
{
    char_u	    *lhs = str2special_save(mp->m_keys, TRUE, FALSE);
    char_u	    *mapmode = map_mode_to_chars(mp->m_mode);

    dict_add_string(dict, "lhs", lhs);
    vim_free(lhs);
    dict_add_string(dict, "lhsraw", mp->m_keys);
    if (lhsrawalt)
	// Also add the value for the simplified entry.
	dict_add_string(dict, "lhsrawalt", lhsrawalt);
    dict_add_string(dict, "rhs", mp->m_orig_str);
    dict_add_number(dict, "noremap", mp->m_noremap ? 1L : 0L);
    dict_add_number(dict, "script", mp->m_noremap == REMAP_SCRIPT
		    ? 1L : 0L);
    dict_add_number(dict, "expr", mp->m_expr ? 1L : 0L);
    dict_add_number(dict, "silent", mp->m_silent ? 1L : 0L);
    dict_add_number(dict, "sid", (long)mp->m_script_ctx.sc_sid);
    dict_add_number(dict, "scriptversion",
		    (long)mp->m_script_ctx.sc_version);
    dict_add_number(dict, "lnum", (long)mp->m_script_ctx.sc_lnum);
    dict_add_number(dict, "buffer", (long)buffer_local);
    dict_add_number(dict, "nowait", mp->m_nowait ? 1L : 0L);
    dict_add_string(dict, "mode", mapmode);
    dict_add_number(dict, "abbr", abbr ? 1L : 0L);
    dict_add_number(dict, "mode_bits", mp->m_mode);

    vim_free(mapmode);
}

    static void
get_maparg(typval_T *argvars, typval_T *rettv, int exact)
{
    char_u	*keys;
    char_u	*keys_simplified;
    char_u	*which;
    char_u	buf[NUMBUFLEN];
    char_u	*keys_buf = NULL;
    char_u	*alt_keys_buf = NULL;
    int		did_simplify = FALSE;
    char_u	*rhs;
    int		mode;
    int		abbr = FALSE;
    int		get_dict = FALSE;
    mapblock_T	*mp = NULL;
    int		buffer_local;
    int		flags = REPTERM_FROM_PART | REPTERM_DO_LT;

    // return empty string for failure
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    keys = tv_get_string(&argvars[0]);
    if (*keys == NUL)
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	which = tv_get_string_buf_chk(&argvars[1], buf);
	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    abbr = (int)tv_get_bool(&argvars[2]);
	    if (argvars[3].v_type != VAR_UNKNOWN)
		get_dict = (int)tv_get_bool(&argvars[3]);
	}
    }
    else
	which = (char_u *)"";
    if (which == NULL)
	return;

    mode = get_map_mode(&which, 0);

    keys_simplified = replace_termcodes(keys, &keys_buf, 0, flags,
								&did_simplify);
    rhs = check_map(keys_simplified, mode, exact, FALSE, abbr,
							   &mp, &buffer_local);
    if (did_simplify)
    {
	// When the lhs is being simplified the not-simplified keys are
	// preferred for printing, like in do_map().
	(void)replace_termcodes(keys, &alt_keys_buf, 0,
					flags | REPTERM_NO_SIMPLIFY, NULL);
	rhs = check_map(alt_keys_buf, mode, exact, FALSE, abbr, &mp,
								&buffer_local);
    }

    if (!get_dict)
    {
	// Return a string.
	if (rhs != NULL)
	{
	    if (*rhs == NUL)
		rettv->vval.v_string = vim_strsave((char_u *)"<Nop>");
	    else
		rettv->vval.v_string = str2special_save(rhs, FALSE, FALSE);
	}

    }
    else if (rettv_dict_alloc(rettv) == OK && rhs != NULL)
	mapblock2dict(mp, rettv->vval.v_dict,
			  did_simplify ? keys_simplified : NULL,
			  buffer_local, abbr);

    vim_free(keys_buf);
    vim_free(alt_keys_buf);
}

/*
 * "maplist()" function
 */
    void
f_maplist(typval_T *argvars UNUSED, typval_T *rettv)
{
    dict_T	*d;
    mapblock_T	*mp;
    int		buffer_local;
    char_u	*keys_buf;
    int		did_simplify;
    int		hash;
    char_u	*lhs;
    const int	flags = REPTERM_FROM_PART | REPTERM_DO_LT;
    int		abbr = FALSE;

    if (in_vim9script() && check_for_opt_bool_arg(argvars, 0) == FAIL)
	return;
    if (argvars[0].v_type != VAR_UNKNOWN)
	abbr = tv_get_bool(&argvars[0]);

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    validate_maphash();

    // Do it twice: once for global maps and once for local maps.
    for (buffer_local = 0; buffer_local <= 1; ++buffer_local)
    {
	for (hash = 0; hash < 256; ++hash)
	{
	    if (abbr)
	    {
		if (hash > 0)		// there is only one abbr list
		    break;
		if (buffer_local)
		    mp = curbuf->b_first_abbr;
		else
		    mp = first_abbr;
	    }
	    else if (buffer_local)
		mp = curbuf->b_maphash[hash];
	    else
		mp = maphash[hash];
	    for (; mp; mp = mp->m_next)
	    {
		if (mp->m_simplified)
		    continue;
		if ((d = dict_alloc()) == NULL)
		    return;
		if (list_append_dict(rettv->vval.v_list, d) == FAIL)
		    return;

		keys_buf = NULL;
		did_simplify = FALSE;

		lhs = str2special_save(mp->m_keys, TRUE, FALSE);
		(void)replace_termcodes(lhs, &keys_buf, 0, flags,
								&did_simplify);
		vim_free(lhs);

		mapblock2dict(mp, d,
				 did_simplify ? keys_buf : NULL,
				 buffer_local, abbr);
		vim_free(keys_buf);
	    }
	}
    }
}

/*
 * "maparg()" function
 */
    void
f_maparg(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && (check_for_opt_bool_arg(argvars, 2) == FAIL
			|| (argvars[2].v_type != VAR_UNKNOWN
			    && check_for_opt_bool_arg(argvars, 3) == FAIL)))))
		return;

    get_maparg(argvars, rettv, TRUE);
}

/*
 * "mapcheck()" function
 */
    void
f_mapcheck(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 2) == FAIL)))
	return;

    get_maparg(argvars, rettv, FALSE);
}

/*
 * Get the mapping mode from the mode string.
 * It may contain multiple characters, eg "nox", or "!", or ' '
 * Return 0 if there is an error.
 */
    static int
get_map_mode_string(char_u *mode_string, int abbr)
{
    char_u	*p = mode_string;
    int		mode = 0;
    int		tmode;
    int		modec;
    const int	MASK_V = MODE_VISUAL | MODE_SELECT;
    const int	MASK_MAP = MODE_VISUAL | MODE_SELECT | MODE_NORMAL
							     | MODE_OP_PENDING;
    const int	MASK_BANG = MODE_INSERT | MODE_CMDLINE;

    if (*p == NUL)
	p = (char_u *)" ";	// compatibility
    while ((modec = *p++))
    {
	switch (modec)
	{
	    case 'i': tmode = MODE_INSERT;	break;
	    case 'l': tmode = MODE_LANGMAP;	break;
	    case 'c': tmode = MODE_CMDLINE;	break;
	    case 'n': tmode = MODE_NORMAL;	break;
	    case 'x': tmode = MODE_VISUAL;	break;
	    case 's': tmode = MODE_SELECT;	break;
	    case 'o': tmode = MODE_OP_PENDING;	break;
	    case 't': tmode = MODE_TERMINAL;	break;
	    case 'v': tmode = MASK_V;		break;
	    case '!': tmode = MASK_BANG;	break;
	    case ' ': tmode = MASK_MAP;		break;
	    default:
		      return 0; // error, unknown mode character
	}
	mode |= tmode;
    }
    if ((abbr && (mode & ~MASK_BANG) != 0)
	|| (!abbr && (mode & (mode-1)) != 0 // more than one bit set
	    && (
		// false if multiple bits set in mode and mode is fully
		// contained in one mask
		!(((mode & MASK_BANG) != 0 && (mode & ~MASK_BANG) == 0)
		    || ((mode & MASK_MAP) != 0 && (mode & ~MASK_MAP) == 0)))))
	return 0;

    return mode;
}

/*
 * "mapset()" function
 */
    void
f_mapset(typval_T *argvars, typval_T *rettv UNUSED)
{
    char_u	*which;
    int		mode;
    char_u	buf[NUMBUFLEN];
    int		is_abbr;
    dict_T	*d;
    char_u	*lhs;
    char_u	*lhsraw;
    char_u	*lhsrawalt;
    char_u	*rhs;
    char_u	*orig_rhs;
    char_u	*arg_buf = NULL;
    int		noremap;
    int		expr;
    int		silent;
    int		buffer;
    scid_T	sid;
    int		scriptversion;
    linenr_T	lnum;
    mapblock_T	**map_table = maphash;
    mapblock_T  **abbr_table = &first_abbr;
    int		nowait;
    char_u	*arg;
    int		dict_only;

    // If first arg is a dict, then that's the only arg permitted.
    dict_only = argvars[0].v_type == VAR_DICT;
    if (in_vim9script()
	    && (check_for_string_or_dict_arg(argvars, 0) == FAIL
		|| (dict_only && check_for_unknown_arg(argvars, 1) == FAIL)
		|| (!dict_only
		    && (check_for_string_arg(argvars, 0) == FAIL
			|| check_for_bool_arg(argvars, 1) == FAIL
			|| check_for_dict_arg(argvars, 2) == FAIL))))
	return;

    if (dict_only)
    {
	d = argvars[0].vval.v_dict;
	which = dict_get_string(d, "mode", FALSE);
	is_abbr = dict_get_bool(d, "abbr", -1);
	if (which == NULL || is_abbr < 0)
	{
	    emsg(_(e_entries_missing_in_mapset_dict_argument));
	    return;
	}
    }
    else
    {
	which = tv_get_string_buf_chk(&argvars[0], buf);
	if (which == NULL)
	    return;
	is_abbr = (int)tv_get_bool(&argvars[1]);

	if (check_for_dict_arg(argvars, 2) == FAIL)
	    return;
	d = argvars[2].vval.v_dict;
    }
    mode = get_map_mode_string(which, is_abbr);
    if (mode == 0)
    {
	semsg(_(e_illegal_map_mode_string_str), which);
	return;
    }


    // Get the values in the same order as above in get_maparg().
    lhs = dict_get_string(d, "lhs", FALSE);
    lhsraw = dict_get_string(d, "lhsraw", FALSE);
    lhsrawalt = dict_get_string(d, "lhsrawalt", FALSE);
    rhs = dict_get_string(d, "rhs", FALSE);
    if (lhs == NULL || lhsraw == NULL || rhs == NULL)
    {
	emsg(_(e_entries_missing_in_mapset_dict_argument));
	return;
    }
    orig_rhs = rhs;

    noremap = dict_get_number(d, "noremap") ? REMAP_NONE: 0;
    if (dict_get_number(d, "script") != 0)
	noremap = REMAP_SCRIPT;
    expr = dict_get_number(d, "expr") != 0;
    silent = dict_get_number(d, "silent") != 0;
    sid = dict_get_number(d, "sid");
    scriptversion = dict_get_number(d, "scriptversion");
    lnum = dict_get_number(d, "lnum");
    buffer = dict_get_number(d, "buffer");
    nowait = dict_get_number(d, "nowait") != 0;
    // mode from the dict is not used

    if (STRICMP(rhs, "<nop>") == 0)	// "<Nop>" means nothing
	rhs = (char_u *)"";
    else
	rhs = replace_termcodes(rhs, &arg_buf, sid,
					REPTERM_DO_LT | REPTERM_SPECIAL, NULL);

    if (buffer)
    {
	map_table = curbuf->b_maphash;
	abbr_table = &curbuf->b_first_abbr;
    }

    // Delete any existing mapping for this lhs and mode.
    if (buffer)
    {
	arg = alloc(STRLEN(lhs) + STRLEN("<buffer>") + 1);
	if (arg == NULL)
	    return;
	STRCPY(arg, "<buffer>");
	STRCPY(arg + 8, lhs);
    }
    else
    {
	arg = vim_strsave(lhs);
	if (arg == NULL)
	    return;
    }
    do_map(MAPTYPE_UNMAP, arg, mode, is_abbr);
    vim_free(arg);

    (void)map_add(map_table, abbr_table, lhsraw, rhs, orig_rhs, noremap,
	    nowait, silent, mode, is_abbr, expr, sid, scriptversion, lnum, 0);
    if (lhsrawalt != NULL)
	(void)map_add(map_table, abbr_table, lhsrawalt, rhs, orig_rhs, noremap,
		nowait, silent, mode, is_abbr, expr, sid, scriptversion,
								      lnum, 1);
    vim_free(arg_buf);
}
#endif


#if defined(MSWIN) || defined(MACOS_X)

# define VIS_SEL	(MODE_VISUAL | MODE_SELECT)	// abbreviation

/*
 * Default mappings for some often used keys.
 */
struct initmap
{
    char_u	*arg;
    int		mode;
};

# ifdef FEAT_GUI_MSWIN
// Use the Windows (CUA) keybindings. (GUI)
static struct initmap initmappings[] =
{
	// paste, copy and cut
	{(char_u *)"<S-Insert> \"*P", MODE_NORMAL},
	{(char_u *)"<S-Insert> \"-d\"*P", VIS_SEL},
	{(char_u *)"<S-Insert> <C-R><C-O>*", MODE_INSERT | MODE_CMDLINE},
	{(char_u *)"<C-Insert> \"*y", VIS_SEL},
	{(char_u *)"<S-Del> \"*d", VIS_SEL},
	{(char_u *)"<C-Del> \"*d", VIS_SEL},
	{(char_u *)"<C-X> \"*d", VIS_SEL},
	// Missing: CTRL-C (cancel) and CTRL-V (block selection)
};
# endif

# if defined(MSWIN) && (!defined(FEAT_GUI) || defined(VIMDLL))
// Use the Windows (CUA) keybindings. (Console)
static struct initmap cinitmappings[] =
{
	{(char_u *)"\316w <C-Home>", MODE_NORMAL | VIS_SEL},
	{(char_u *)"\316w <C-Home>", MODE_INSERT | MODE_CMDLINE},
	{(char_u *)"\316u <C-End>", MODE_NORMAL | VIS_SEL},
	{(char_u *)"\316u <C-End>", MODE_INSERT | MODE_CMDLINE},

	// paste, copy and cut
#  ifdef FEAT_CLIPBOARD
	{(char_u *)"\316\324 \"*P", MODE_NORMAL},   // SHIFT-Insert is "*P
	{(char_u *)"\316\324 \"-d\"*P", VIS_SEL},   // SHIFT-Insert is "-d"*P
	{(char_u *)"\316\324 \022\017*", MODE_INSERT},  // SHIFT-Insert is ^R^O*
	{(char_u *)"\316\325 \"*y", VIS_SEL},	    // CTRL-Insert is "*y
	{(char_u *)"\316\327 \"*d", VIS_SEL},	    // SHIFT-Del is "*d
	{(char_u *)"\316\330 \"*d", VIS_SEL},	    // CTRL-Del is "*d
	{(char_u *)"\030 \"*d", VIS_SEL},	    // CTRL-X is "*d
#  else
	{(char_u *)"\316\324 P", MODE_NORMAL},	    // SHIFT-Insert is P
	{(char_u *)"\316\324 \"-dP", VIS_SEL},	    // SHIFT-Insert is "-dP
	{(char_u *)"\316\324 \022\017\"", MODE_INSERT}, // SHIFT-Insert is ^R^O"
	{(char_u *)"\316\325 y", VIS_SEL},	    // CTRL-Insert is y
	{(char_u *)"\316\327 d", VIS_SEL},	    // SHIFT-Del is d
	{(char_u *)"\316\330 d", VIS_SEL},	    // CTRL-Del is d
#  endif
};
# endif

# if defined(MACOS_X)
static struct initmap initmappings[] =
{
	// Use the Standard MacOS binding.
	// paste, copy and cut
	{(char_u *)"<D-v> \"*P", MODE_NORMAL},
	{(char_u *)"<D-v> \"-d\"*P", VIS_SEL},
	{(char_u *)"<D-v> <C-R>*", MODE_INSERT | MODE_CMDLINE},
	{(char_u *)"<D-c> \"*y", VIS_SEL},
	{(char_u *)"<D-x> \"*d", VIS_SEL},
	{(char_u *)"<Backspace> \"-d", VIS_SEL},
};
# endif

# undef VIS_SEL
#endif

/*
 * Set up default mappings.
 */
    void
init_mappings(void)
{
#if defined(MSWIN) || defined(MACOS_X)
    int		i;

# if defined(MSWIN) && (!defined(FEAT_GUI_MSWIN) || defined(VIMDLL))
#  ifdef VIMDLL
    if (!gui.starting)
#  endif
    {
	for (i = 0; i < (int)ARRAY_LENGTH(cinitmappings); ++i)
	    add_map(cinitmappings[i].arg, cinitmappings[i].mode, FALSE);
    }
# endif
# if defined(FEAT_GUI_MSWIN) || defined(MACOS_X)
    for (i = 0; i < (int)ARRAY_LENGTH(initmappings); ++i)
	add_map(initmappings[i].arg, initmappings[i].mode, FALSE);
# endif
#endif
}

/*
 * Add a mapping "map" for mode "mode".
 * When "nore" is TRUE use MAPTYPE_NOREMAP.
 * Need to put string in allocated memory, because do_map() will modify it.
 */
    void
add_map(char_u *map, int mode, int nore)
{
    char_u	*s;
    char_u	*cpo_save = p_cpo;

    p_cpo = empty_option;	// Allow <> notation
    s = vim_strsave(map);
    if (s != NULL)
    {
	(void)do_map(nore ? MAPTYPE_NOREMAP : MAPTYPE_MAP, s, mode, FALSE);
	vim_free(s);
    }
    p_cpo = cpo_save;
}

#if defined(FEAT_LANGMAP) || defined(PROTO)
/*
 * Any character has an equivalent 'langmap' character.  This is used for
 * keyboards that have a special language mode that sends characters above
 * 128 (although other characters can be translated too).  The "to" field is a
 * Vim command character.  This avoids having to switch the keyboard back to
 * ASCII mode when leaving Insert mode.
 *
 * langmap_mapchar[] maps any of 256 chars to an ASCII char used for Vim
 * commands.
 * langmap_mapga.ga_data is a sorted table of langmap_entry_T.  This does the
 * same as langmap_mapchar[] for characters >= 256.
 *
 * Use growarray for 'langmap' chars >= 256
 */
typedef struct
{
    int	    from;
    int     to;
} langmap_entry_T;

static garray_T langmap_mapga;

/*
 * Search for an entry in "langmap_mapga" for "from".  If found set the "to"
 * field.  If not found insert a new entry at the appropriate location.
 */
    static void
langmap_set_entry(int from, int to)
{
    langmap_entry_T *entries = (langmap_entry_T *)(langmap_mapga.ga_data);
    int		    a = 0;
    int		    b = langmap_mapga.ga_len;

    // Do a binary search for an existing entry.
    while (a != b)
    {
	int i = (a + b) / 2;
	int d = entries[i].from - from;

	if (d == 0)
	{
	    entries[i].to = to;
	    return;
	}
	if (d < 0)
	    a = i + 1;
	else
	    b = i;
    }

    if (ga_grow(&langmap_mapga, 1) == FAIL)
	return;  // out of memory

    // insert new entry at position "a"
    entries = (langmap_entry_T *)(langmap_mapga.ga_data) + a;
    mch_memmove(entries + 1, entries,
			(langmap_mapga.ga_len - a) * sizeof(langmap_entry_T));
    ++langmap_mapga.ga_len;
    entries[0].from = from;
    entries[0].to = to;
}

/*
 * Apply 'langmap' to multi-byte character "c" and return the result.
 */
    int
langmap_adjust_mb(int c)
{
    langmap_entry_T *entries = (langmap_entry_T *)(langmap_mapga.ga_data);
    int a = 0;
    int b = langmap_mapga.ga_len;

    while (a != b)
    {
	int i = (a + b) / 2;
	int d = entries[i].from - c;

	if (d == 0)
	    return entries[i].to;  // found matching entry
	if (d < 0)
	    a = i + 1;
	else
	    b = i;
    }
    return c;  // no entry found, return "c" unmodified
}

    void
langmap_init(void)
{
    int i;

    for (i = 0; i < 256; i++)
	langmap_mapchar[i] = i;	 // we init with a one-to-one map
    ga_init2(&langmap_mapga, sizeof(langmap_entry_T), 8);
}

/*
 * Called when langmap option is set; the language map can be
 * changed at any time!
 */
    char *
did_set_langmap(optset_T *args UNUSED)
{
    char_u  *p;
    char_u  *p2;
    int	    from, to;

    ga_clear(&langmap_mapga);		    // clear the previous map first
    langmap_init();			    // back to one-to-one map

    for (p = p_langmap; p[0] != NUL; )
    {
	for (p2 = p; p2[0] != NUL && p2[0] != ',' && p2[0] != ';';
							       MB_PTR_ADV(p2))
	{
	    if (p2[0] == '\\' && p2[1] != NUL)
		++p2;
	}
	if (p2[0] == ';')
	    ++p2;	    // abcd;ABCD form, p2 points to A
	else
	    p2 = NULL;	    // aAbBcCdD form, p2 is NULL
	while (p[0])
	{
	    if (p[0] == ',')
	    {
		++p;
		break;
	    }
	    if (p[0] == '\\' && p[1] != NUL)
		++p;
	    from = (*mb_ptr2char)(p);
	    to = NUL;
	    if (p2 == NULL)
	    {
		MB_PTR_ADV(p);
		if (p[0] != ',')
		{
		    if (p[0] == '\\')
			++p;
		    to = (*mb_ptr2char)(p);
		}
	    }
	    else
	    {
		if (p2[0] != ',')
		{
		    if (p2[0] == '\\')
			++p2;
		    to = (*mb_ptr2char)(p2);
		}
	    }
	    if (to == NUL)
	    {
		sprintf(args->os_errbuf,
			_(e_langmap_matching_character_missing_for_str),
			transchar(from));
		return args->os_errbuf;
	    }

	    if (from >= 256)
		langmap_set_entry(from, to);
	    else
		langmap_mapchar[from & 255] = to;

	    // Advance to next pair
	    MB_PTR_ADV(p);
	    if (p2 != NULL)
	    {
		MB_PTR_ADV(p2);
		if (*p == ';')
		{
		    p = p2;
		    if (p[0] != NUL)
		    {
			if (p[0] != ',')
			{
			    vim_snprintf(args->os_errbuf, args->os_errbuflen,
				    _(e_langmap_extra_characters_after_semicolon_str),
				    p);
			    return args->os_errbuf;
			}
			++p;
		    }
		    break;
		}
	    }
	}
    }

    return NULL;
}
#endif

    static void
do_exmap(exarg_T *eap, int isabbrev)
{
    int	    mode;
    char_u  *cmdp;

    cmdp = eap->cmd;
    mode = get_map_mode(&cmdp, eap->forceit || isabbrev);

    switch (do_map(*cmdp == 'n' ? MAPTYPE_NOREMAP
				: *cmdp == 'u' ? MAPTYPE_UNMAP : MAPTYPE_MAP,
						    eap->arg, mode, isabbrev))
    {
	case 1: emsg(_(e_invalid_argument));
		break;
	case 2: emsg((isabbrev ? _(e_no_such_abbreviation)
						      : _(e_no_such_mapping)));
		break;
    }
}

/*
 * ":abbreviate" and friends.
 */
    void
ex_abbreviate(exarg_T *eap)
{
    do_exmap(eap, TRUE);	// almost the same as mapping
}

/*
 * ":map" and friends.
 */
    void
ex_map(exarg_T *eap)
{
    // If we are sourcing .exrc or .vimrc in current directory we
    // print the mappings for security reasons.
    if (secure)
    {
	secure = 2;
	msg_outtrans(eap->cmd);
	msg_putchar('\n');
    }
    do_exmap(eap, FALSE);
}

/*
 * ":unmap" and friends.
 */
    void
ex_unmap(exarg_T *eap)
{
    do_exmap(eap, FALSE);
}

/*
 * ":mapclear" and friends.
 */
    void
ex_mapclear(exarg_T *eap)
{
    map_clear(eap->cmd, eap->arg, eap->forceit, FALSE);
}

/*
 * ":abclear" and friends.
 */
    void
ex_abclear(exarg_T *eap)
{
    map_clear(eap->cmd, eap->arg, TRUE, TRUE);
}
