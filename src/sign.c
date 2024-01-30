/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * sign.c: functions for managing signs
 */

#include "vim.h"

#if defined(FEAT_SIGNS) || defined(PROTO)

/*
 * Struct to hold the sign properties.
 */
typedef struct sign sign_T;

struct sign
{
    sign_T	*sn_next;	// next sign in list
    int		sn_typenr;	// type number of sign
    char_u	*sn_name;	// name of sign
    char_u	*sn_icon;	// name of pixmap
# ifdef FEAT_SIGN_ICONS
    void	*sn_image;	// icon image
# endif
    char_u	*sn_text;	// text used instead of pixmap
    int		sn_line_hl;	// highlight ID for line
    int		sn_text_hl;	// highlight ID for text
    int		sn_cul_hl;	// highlight ID for text on current line when 'cursorline' is set
    int		sn_num_hl;	// highlight ID for line number
};

static sign_T	*first_sign = NULL;
static int	next_sign_typenr = 1;

static void sign_list_defined(sign_T *sp);
static void sign_undefine(sign_T *sp, sign_T *sp_prev);

static char *cmds[] = {
			"define",
# define SIGNCMD_DEFINE	0
			"undefine",
# define SIGNCMD_UNDEFINE 1
			"list",
# define SIGNCMD_LIST	2
			"place",
# define SIGNCMD_PLACE	3
			"unplace",
# define SIGNCMD_UNPLACE 4
			"jump",
# define SIGNCMD_JUMP	5
			NULL
# define SIGNCMD_LAST	6
};

#define FOR_ALL_SIGNS(sp)	\
    for ((sp) = first_sign; (sp) != NULL; (sp) = (sp)->sn_next)

static hashtab_T	sg_table;	// sign group (signgroup_T) hashtable
static int		next_sign_id = 1; // next sign id in the global group

/*
 * Initialize data needed for managing signs
 */
    void
init_signs(void)
{
    hash_init(&sg_table);		// sign group hash table
}

/*
 * A new sign in group 'groupname' is added. If the group is not present,
 * create it. Otherwise reference the group.
 */
    static signgroup_T *
sign_group_ref(char_u *groupname)
{
    hash_T		hash;
    hashitem_T		*hi;
    signgroup_T		*group;

    hash = hash_hash(groupname);
    hi = hash_lookup(&sg_table, groupname, hash);
    if (HASHITEM_EMPTY(hi))
    {
	// new group
	group = alloc(offsetof(signgroup_T, sg_name) + STRLEN(groupname) + 1);
	if (group == NULL)
	    return NULL;
	STRCPY(group->sg_name, groupname);
	group->sg_refcount = 1;
	group->sg_next_sign_id = 1;
	hash_add_item(&sg_table, hi, group->sg_name, hash);
    }
    else
    {
	// existing group
	group = HI2SG(hi);
	group->sg_refcount++;
    }

    return group;
}

/*
 * A sign in group 'groupname' is removed. If all the signs in this group are
 * removed, then remove the group.
 */
    static void
sign_group_unref(char_u *groupname)
{
    hashitem_T		*hi;
    signgroup_T		*group;

    hi = hash_find(&sg_table, groupname);
    if (HASHITEM_EMPTY(hi))
	return;

    group = HI2SG(hi);
    group->sg_refcount--;
    if (group->sg_refcount == 0)
    {
	// All the signs in this group are removed
	hash_remove(&sg_table, hi, "sign remove");
	vim_free(group);
    }
}

/*
 * Returns TRUE if 'sign' is in 'group'.
 * A sign can either be in the global group (sign->se_group == NULL)
 * or in a named group. If 'group' is '*', then the sign is part of the group.
 */
    static int
sign_in_group(sign_entry_T *sign, char_u *group)
{
    return ((group != NULL && STRCMP(group, "*") == 0)
	    || (group == NULL && sign->se_group == NULL)
	    || (group != NULL && sign->se_group != NULL
			      && STRCMP(group, sign->se_group->sg_name) == 0));
}

/*
 * Return TRUE if "sign" is to be displayed in window "wp".
 * If the group name starts with "PopUp" it only shows in a popup window.
 */
    static int
sign_group_for_window(sign_entry_T *sign, win_T *wp)
{
    int for_popup = sign->se_group != NULL
			&& STRNCMP("PopUp", sign->se_group->sg_name, 5) == 0;

    return WIN_IS_POPUP(wp) ? for_popup : !for_popup;
}

/*
 * Get the next free sign identifier in the specified group
 */
    static int
sign_group_get_next_signid(buf_T *buf, char_u *groupname)
{
    int			id = 1;
    signgroup_T		*group = NULL;
    sign_entry_T	*sign;
    hashitem_T		*hi;
    int			found = FALSE;

    if (groupname != NULL)
    {
	hi = hash_find(&sg_table, groupname);
	if (HASHITEM_EMPTY(hi))
	    return id;
	group = HI2SG(hi);
    }

    // Search for the next usable sign identifier
    while (!found)
    {
	if (group == NULL)
	    id = next_sign_id++;		// global group
	else
	    id = group->sg_next_sign_id++;

	// Check whether this sign is already placed in the buffer
	found = TRUE;
	FOR_ALL_SIGNS_IN_BUF(buf, sign)
	{
	    if (id == sign->se_id && sign_in_group(sign, groupname))
	    {
		found = FALSE;		// sign identifier is in use
		break;
	    }
	}
    }

    return id;
}

/*
 * Insert a new sign into the signlist for buffer 'buf' between the 'prev' and
 * 'next' signs.
 */
    static void
insert_sign(
    buf_T	*buf,		// buffer to store sign in
    sign_entry_T *prev,		// previous sign entry
    sign_entry_T *next,		// next sign entry
    int		id,		// sign ID
    char_u	*group,		// sign group; NULL for global group
    int		prio,		// sign priority
    linenr_T	lnum,		// line number which gets the mark
    int		typenr)		// typenr of sign we are adding
{
    sign_entry_T *newsign;

    newsign = lalloc_id(sizeof(sign_entry_T), FALSE, aid_insert_sign);
    if (newsign == NULL)
	return;

    newsign->se_id = id;
    newsign->se_lnum = lnum;
    newsign->se_typenr = typenr;
    if (group != NULL)
    {
	newsign->se_group = sign_group_ref(group);
	if (newsign->se_group == NULL)
	{
	    vim_free(newsign);
	    return;
	}
    }
    else
	newsign->se_group = NULL;
    newsign->se_priority = prio;
    newsign->se_next = next;
    newsign->se_prev = prev;
    if (next != NULL)
	next->se_prev = newsign;

    if (prev == NULL)
    {
	// When adding first sign need to redraw the windows to create the
	// column for signs.
	if (buf->b_signlist == NULL)
	{
	    redraw_buf_later(buf, UPD_NOT_VALID);
	    changed_line_abv_curs();
	}

	// first sign in signlist
	buf->b_signlist = newsign;
#ifdef FEAT_NETBEANS_INTG
	if (netbeans_active())
	    buf->b_has_sign_column = TRUE;
#endif
    }
    else
	prev->se_next = newsign;
}

/*
 * Insert a new sign sorted by line number and sign priority.
 */
    static void
insert_sign_by_lnum_prio(
    buf_T	*buf,		// buffer to store sign in
    sign_entry_T *prev,		// previous sign entry
    int		id,		// sign ID
    char_u	*group,		// sign group; NULL for global group
    int		prio,		// sign priority
    linenr_T	lnum,		// line number which gets the mark
    int		typenr)		// typenr of sign we are adding
{
    sign_entry_T	*sign;

    // keep signs sorted by lnum and by priority: insert new sign at
    // the proper position in the list for this lnum.
    while (prev != NULL && prev->se_lnum == lnum && prev->se_priority <= prio)
	prev = prev->se_prev;
    if (prev == NULL)
	sign = buf->b_signlist;
    else
	sign = prev->se_next;

    insert_sign(buf, prev, sign, id, group, prio, lnum, typenr);
}

/*
 * Lookup a sign by typenr. Returns NULL if sign is not found.
 */
    static sign_T *
find_sign_by_typenr(int typenr)
{
    sign_T	*sp;

    FOR_ALL_SIGNS(sp)
	if (sp->sn_typenr == typenr)
	    return sp;
    return NULL;
}

/*
 * Get the name of a sign by its typenr.
 */
    static char_u *
sign_typenr2name(int typenr)
{
    sign_T	*sp;

    FOR_ALL_SIGNS(sp)
	if (sp->sn_typenr == typenr)
	    return sp->sn_name;
    return (char_u *)_("[Deleted]");
}

/*
 * Return information about a sign in a Dict
 */
    static dict_T *
sign_get_info(sign_entry_T *sign)
{
    dict_T	*d;

    if ((d = dict_alloc_id(aid_sign_getinfo)) == NULL)
	return NULL;
    dict_add_number(d, "id", sign->se_id);
    dict_add_string(d, "group", (sign->se_group == NULL) ?
				       (char_u *)"" : sign->se_group->sg_name);
    dict_add_number(d, "lnum", sign->se_lnum);
    dict_add_string(d, "name", sign_typenr2name(sign->se_typenr));
    dict_add_number(d, "priority", sign->se_priority);

    return d;
}

/*
 * Sort the signs placed on the same line as "sign" by priority.  Invoked after
 * changing the priority of an already placed sign.  Assumes the signs in the
 * buffer are sorted by line number and priority.
 */
    static void
sign_sort_by_prio_on_line(buf_T *buf, sign_entry_T *sign)
{
    sign_entry_T *p = NULL;

    // If there is only one sign in the buffer or only one sign on the line or
    // the sign is already sorted by priority, then return.
    if ((sign->se_prev == NULL
		|| sign->se_prev->se_lnum != sign->se_lnum
		|| sign->se_prev->se_priority > sign->se_priority)
	    && (sign->se_next == NULL
		|| sign->se_next->se_lnum != sign->se_lnum
		|| sign->se_next->se_priority < sign->se_priority))
	return;

    // One or more signs on the same line as 'sign'
    // Find a sign after which 'sign' should be inserted

    // First search backward for a sign with higher priority on the same line
    p = sign;
    while (p->se_prev != NULL && p->se_prev->se_lnum == sign->se_lnum
			       && p->se_prev->se_priority <= sign->se_priority)
	p = p->se_prev;

    if (p == sign)
    {
	// Sign not found. Search forward for a sign with priority just before
	// 'sign'.
	p = sign->se_next;
	while (p->se_next != NULL && p->se_next->se_lnum == sign->se_lnum
				&& p->se_next->se_priority > sign->se_priority)
	    p = p->se_next;
    }

    // Remove 'sign' from the list
    if (buf->b_signlist == sign)
	buf->b_signlist = sign->se_next;
    if (sign->se_prev != NULL)
	sign->se_prev->se_next = sign->se_next;
    if (sign->se_next != NULL)
	sign->se_next->se_prev = sign->se_prev;
    sign->se_prev = NULL;
    sign->se_next = NULL;

    // Re-insert 'sign' at the right place
    if (p->se_priority <= sign->se_priority)
    {
	// 'sign' has a higher priority and should be inserted before 'p'
	sign->se_prev = p->se_prev;
	sign->se_next = p;
	p->se_prev = sign;
	if (sign->se_prev != NULL)
	    sign->se_prev->se_next = sign;
	if (buf->b_signlist == p)
	    buf->b_signlist = sign;
    }
    else
    {
	// 'sign' has a lower priority and should be inserted after 'p'
	sign->se_prev = p;
	sign->se_next = p->se_next;
	p->se_next = sign;
	if (sign->se_next != NULL)
	    sign->se_next->se_prev = sign;
    }
}

/*
 * Add the sign into the signlist. Find the right spot to do it though.
 */
    static void
buf_addsign(
    buf_T	*buf,		// buffer to store sign in
    int		id,		// sign ID
    char_u	*groupname,	// sign group
    int		prio,		// sign priority
    linenr_T	lnum,		// line number which gets the mark
    int		typenr)		// typenr of sign we are adding
{
    sign_entry_T	*sign;		// a sign in the signlist
    sign_entry_T	*prev;		// the previous sign

    prev = NULL;
    FOR_ALL_SIGNS_IN_BUF(buf, sign)
    {
	if (lnum == sign->se_lnum && id == sign->se_id
		&& sign_in_group(sign, groupname))
	{
	    // Update an existing sign
	    sign->se_typenr = typenr;
	    sign->se_priority = prio;
	    sign_sort_by_prio_on_line(buf, sign);
	    return;
	}
	else if (lnum < sign->se_lnum)
	{
	    insert_sign_by_lnum_prio(buf, prev, id, groupname, prio,
								lnum, typenr);
	    return;
	}
	prev = sign;
    }

    insert_sign_by_lnum_prio(buf, prev, id, groupname, prio, lnum, typenr);
    return;
}

/*
 * For an existing, placed sign "markId" change the type to "typenr".
 * Returns the line number of the sign, or zero if the sign is not found.
 */
    static linenr_T
buf_change_sign_type(
    buf_T	*buf,		// buffer to store sign in
    int		markId,		// sign ID
    char_u	*group,		// sign group
    int		typenr,		// typenr of sign we are adding
    int		prio)		// sign priority
{
    sign_entry_T	*sign;		// a sign in the signlist

    FOR_ALL_SIGNS_IN_BUF(buf, sign)
    {
	if (sign->se_id == markId && sign_in_group(sign, group))
	{
	    sign->se_typenr = typenr;
	    sign->se_priority = prio;
	    sign_sort_by_prio_on_line(buf, sign);
	    return sign->se_lnum;
	}
    }

    return (linenr_T)0;
}

/*
 * Return the attributes of the first sign placed on line 'lnum' in buffer
 * 'buf'. Used when refreshing the screen. Returns TRUE if a sign is found on
 * 'lnum', FALSE otherwise.
 */
    int
buf_get_signattrs(win_T *wp, linenr_T lnum, sign_attrs_T *sattr)
{
    sign_entry_T	*sign;
    sign_T		*sp;
    buf_T		*buf = wp->w_buffer;

    CLEAR_POINTER(sattr);

    FOR_ALL_SIGNS_IN_BUF(buf, sign)
    {
	if (sign->se_lnum > lnum)
	    // Signs are sorted by line number in the buffer. No need to check
	    // for signs after the specified line number 'lnum'.
	    break;

	if (sign->se_lnum == lnum
# ifdef FEAT_PROP_POPUP
		&& sign_group_for_window(sign, wp)
# endif
		)
	{
	    sattr->sat_typenr = sign->se_typenr;
	    sp = find_sign_by_typenr(sign->se_typenr);
	    if (sp == NULL)
		return FALSE;

# ifdef FEAT_SIGN_ICONS
	    sattr->sat_icon = sp->sn_image;
# endif
	    sattr->sat_text = sp->sn_text;
	    if (sattr->sat_text != NULL && sp->sn_text_hl > 0)
		sattr->sat_texthl = syn_id2attr(sp->sn_text_hl);
	    if (sp->sn_line_hl > 0)
		sattr->sat_linehl = syn_id2attr(sp->sn_line_hl);
	    if (sp->sn_cul_hl > 0)
		sattr->sat_culhl = syn_id2attr(sp->sn_cul_hl);
	    if (sp->sn_num_hl > 0)
		sattr->sat_numhl = syn_id2attr(sp->sn_num_hl);
	    sattr->sat_priority = sign->se_priority;

	    // If there is another sign next with the same priority, may
	    // combine the text and the line highlighting.
	    if (sign->se_next != NULL
		    && sign->se_next->se_priority == sign->se_priority
		    && sign->se_next->se_lnum == sign->se_lnum)
	    {
		sign_T	*next_sp = find_sign_by_typenr(sign->se_next->se_typenr);

		if (next_sp != NULL)
		{
		    if (sattr->sat_icon == NULL && sattr->sat_text == NULL)
		    {
# ifdef FEAT_SIGN_ICONS
			sattr->sat_icon = next_sp->sn_image;
# endif
			sattr->sat_text = next_sp->sn_text;
		    }
		    if (sp->sn_text_hl <= 0 && next_sp->sn_text_hl > 0)
			sattr->sat_texthl = syn_id2attr(next_sp->sn_text_hl);
		    if (sp->sn_line_hl <= 0 && next_sp->sn_line_hl > 0)
			sattr->sat_linehl = syn_id2attr(next_sp->sn_line_hl);
		    if (sp->sn_cul_hl <= 0 && next_sp->sn_cul_hl > 0)
			sattr->sat_culhl = syn_id2attr(next_sp->sn_cul_hl);
		    if (sp->sn_num_hl <= 0 && next_sp->sn_num_hl > 0)
			sattr->sat_numhl = syn_id2attr(next_sp->sn_num_hl);
		}
	    }
	    return TRUE;
	}
    }
    return FALSE;
}

/*
 * Delete sign 'id' in group 'group' from buffer 'buf'.
 * If 'id' is zero, then delete all the signs in group 'group'. Otherwise
 * delete only the specified sign.
 * If 'group' is '*', then delete the sign in all the groups. If 'group' is
 * NULL, then delete the sign in the global group. Otherwise delete the sign in
 * the specified group.
 * Returns the line number of the deleted sign. If multiple signs are deleted,
 * then returns the line number of the last sign deleted.
 */
    linenr_T
buf_delsign(
    buf_T	*buf,		// buffer sign is stored in
    linenr_T	atlnum,		// sign at this line, 0 - at any line
    int		id,		// sign id
    char_u	*group)		// sign group
{
    sign_entry_T	**lastp;	// pointer to pointer to current sign
    sign_entry_T	*sign;		// a sign in a b_signlist
    sign_entry_T	*next;		// the next sign in a b_signlist
    linenr_T		lnum;		// line number whose sign was deleted

    lastp = &buf->b_signlist;
    lnum = 0;
    for (sign = buf->b_signlist; sign != NULL; sign = next)
    {
	next = sign->se_next;
	if ((id == 0 || sign->se_id == id)
		&& (atlnum == 0 || sign->se_lnum == atlnum)
		&& sign_in_group(sign, group))

	{
	    *lastp = next;
	    if (next != NULL)
		next->se_prev = sign->se_prev;
	    lnum = sign->se_lnum;
	    if (sign->se_group != NULL)
		sign_group_unref(sign->se_group->sg_name);
	    vim_free(sign);
	    redraw_buf_line_later(buf, lnum);

	    // Check whether only one sign needs to be deleted
	    // If deleting a sign with a specific identifier in a particular
	    // group or deleting any sign at a particular line number, delete
	    // only one sign.
	    if (group == NULL
		    || (*group != '*' && id != 0)
		    || (*group == '*' && atlnum != 0))
		break;
	}
	else
	    lastp = &sign->se_next;
    }

    // When deleting the last sign the cursor position may change, because the
    // sign columns no longer shows.  And the 'signcolumn' may be hidden.
    if (buf->b_signlist == NULL)
    {
	redraw_buf_later(buf, UPD_NOT_VALID);
	changed_line_abv_curs();
    }

    return lnum;
}


/*
 * Find the line number of the sign with the requested id in group 'group'. If
 * the sign does not exist, return 0 as the line number. This will still let
 * the correct file get loaded.
 */
    int
buf_findsign(
    buf_T	*buf,		// buffer to store sign in
    int		id,		// sign ID
    char_u	*group)		// sign group
{
    sign_entry_T	*sign;		// a sign in the signlist

    FOR_ALL_SIGNS_IN_BUF(buf, sign)
	if (sign->se_id == id && sign_in_group(sign, group))
	    return sign->se_lnum;

    return 0;
}

/*
 * Return the sign at line 'lnum' in buffer 'buf'. Returns NULL if a sign is
 * not found at the line. If 'groupname' is NULL, searches in the global group.
 */
    static sign_entry_T *
buf_getsign_at_line(
    buf_T	*buf,		// buffer whose sign we are searching for
    linenr_T	lnum,		// line number of sign
    char_u	*groupname)	// sign group name
{
    sign_entry_T	*sign;		// a sign in the signlist

    FOR_ALL_SIGNS_IN_BUF(buf, sign)
    {
	if (sign->se_lnum > lnum)
	    // Signs are sorted by line number in the buffer. No need to check
	    // for signs after the specified line number 'lnum'.
	    break;

	if (sign->se_lnum == lnum && sign_in_group(sign, groupname))
	    return sign;
    }

    return NULL;
}

/*
 * Return the identifier of the sign at line number 'lnum' in buffer 'buf'.
 */
    int
buf_findsign_id(
    buf_T	*buf,		// buffer whose sign we are searching for
    linenr_T	lnum,		// line number of sign
    char_u	*groupname)	// sign group name
{
    sign_entry_T	*sign;		// a sign in the signlist

    sign = buf_getsign_at_line(buf, lnum, groupname);
    if (sign != NULL)
	return sign->se_id;

    return 0;
}

# if defined(FEAT_NETBEANS_INTG) || defined(PROTO)
/*
 * See if a given type of sign exists on a specific line.
 */
    int
buf_findsigntype_id(
    buf_T	*buf,		// buffer whose sign we are searching for
    linenr_T	lnum,		// line number of sign
    int		typenr)		// sign type number
{
    sign_entry_T	*sign;		// a sign in the signlist

    FOR_ALL_SIGNS_IN_BUF(buf, sign)
    {
	if (sign->se_lnum > lnum)
	    // Signs are sorted by line number in the buffer. No need to check
	    // for signs after the specified line number 'lnum'.
	    break;

	if (sign->se_lnum == lnum && sign->se_typenr == typenr)
	    return sign->se_id;
    }

    return 0;
}


#  if defined(FEAT_SIGN_ICONS) || defined(PROTO)
/*
 * Return the number of icons on the given line.
 */
    int
buf_signcount(buf_T *buf, linenr_T lnum)
{
    sign_entry_T	*sign;		// a sign in the signlist
    int			count = 0;

    FOR_ALL_SIGNS_IN_BUF(buf, sign)
    {
	if (sign->se_lnum > lnum)
	    // Signs are sorted by line number in the buffer. No need to check
	    // for signs after the specified line number 'lnum'.
	    break;

	if (sign->se_lnum == lnum)
	    if (sign_get_image(sign->se_typenr) != NULL)
		count++;
    }

    return count;
}
#  endif // FEAT_SIGN_ICONS
# endif // FEAT_NETBEANS_INTG

/*
 * Delete signs in group 'group' in buffer "buf". If 'group' is '*', then
 * delete all the signs.
 */
    void
buf_delete_signs(buf_T *buf, char_u *group)
{
    sign_entry_T	*sign;
    sign_entry_T	**lastp;	// pointer to pointer to current sign
    sign_entry_T	*next;

    // When deleting the last sign need to redraw the windows to remove the
    // sign column. Not when curwin is NULL (this means we're exiting).
    if (buf->b_signlist != NULL && curwin != NULL)
    {
	redraw_buf_later(buf, UPD_NOT_VALID);
	changed_line_abv_curs();
    }

    lastp = &buf->b_signlist;
    for (sign = buf->b_signlist; sign != NULL; sign = next)
    {
	next = sign->se_next;
	if (sign_in_group(sign, group))
	{
	    *lastp = next;
	    if (next != NULL)
		next->se_prev = sign->se_prev;
	    if (sign->se_group != NULL)
		sign_group_unref(sign->se_group->sg_name);
	    vim_free(sign);
	}
	else
	    lastp = &sign->se_next;
    }
}

/*
 * List placed signs for "rbuf".  If "rbuf" is NULL do it for all buffers.
 */
    static void
sign_list_placed(buf_T *rbuf, char_u *sign_group)
{
    buf_T		*buf;
    sign_entry_T	*sign;
    char		lbuf[MSG_BUF_LEN];
    char		group[MSG_BUF_LEN];

    msg_puts_title(_("\n--- Signs ---"));
    msg_putchar('\n');
    if (rbuf == NULL)
	buf = firstbuf;
    else
	buf = rbuf;
    while (buf != NULL && !got_int)
    {
	if (buf->b_signlist != NULL)
	{
	    vim_snprintf(lbuf, MSG_BUF_LEN, _("Signs for %s:"), buf->b_fname);
	    msg_puts_attr(lbuf, HL_ATTR(HLF_D));
	    msg_putchar('\n');
	}
	FOR_ALL_SIGNS_IN_BUF(buf, sign)
	{
	    if (got_int)
		break;
	    if (!sign_in_group(sign, sign_group))
		continue;
	    if (sign->se_group != NULL)
		vim_snprintf(group, MSG_BUF_LEN, _("  group=%s"),
						      sign->se_group->sg_name);
	    else
		group[0] = '\0';
	    vim_snprintf(lbuf, MSG_BUF_LEN,
			   _("    line=%ld  id=%d%s  name=%s  priority=%d"),
			   (long)sign->se_lnum, sign->se_id, group,
			 sign_typenr2name(sign->se_typenr), sign->se_priority);
	    msg_puts(lbuf);
	    msg_putchar('\n');
	}
	if (rbuf != NULL)
	    break;
	buf = buf->b_next;
    }
}

/*
 * Adjust a placed sign for inserted/deleted lines.
 */
    void
sign_mark_adjust(
    linenr_T	line1,
    linenr_T	line2,
    long	amount,
    long	amount_after)
{
    sign_entry_T	*sign;		// a sign in a b_signlist
    linenr_T		new_lnum;

    FOR_ALL_SIGNS_IN_BUF(curbuf, sign)
    {
	// Ignore changes to lines after the sign
	if (sign->se_lnum < line1)
	    continue;
	new_lnum = sign->se_lnum;
	if (sign->se_lnum <= line2)
	{
	    if (amount != MAXLNUM)
		new_lnum += amount;
	}
	else if (sign->se_lnum > line2)
	    // Lines inserted or deleted before the sign
	    new_lnum += amount_after;

	// If the new sign line number is past the last line in the buffer,
	// then don't adjust the line number. Otherwise, it will always be past
	// the last line and will not be visible.
	if (new_lnum <= curbuf->b_ml.ml_line_count)
	    sign->se_lnum = new_lnum;
    }
}

/*
 * Find index of a ":sign" subcmd from its name.
 * "*end_cmd" must be writable.
 */
    static int
sign_cmd_idx(
    char_u	*begin_cmd,	// begin of sign subcmd
    char_u	*end_cmd)	// just after sign subcmd
{
    int		idx;
    char	save = *end_cmd;

    *end_cmd = NUL;
    for (idx = 0; ; ++idx)
	if (cmds[idx] == NULL || STRCMP(begin_cmd, cmds[idx]) == 0)
	    break;
    *end_cmd = save;
    return idx;
}

/*
 * Find a sign by name. Also returns pointer to the previous sign.
 */
    static sign_T *
sign_find(char_u *name, sign_T **sp_prev)
{
    sign_T *sp;

    if (sp_prev != NULL)
	*sp_prev = NULL;
    FOR_ALL_SIGNS(sp)
    {
	if (STRCMP(sp->sn_name, name) == 0)
	    break;
	if (sp_prev != NULL)
	    *sp_prev = sp;
    }

    return sp;
}

/*
 * Allocate a new sign
 */
    static sign_T *
alloc_new_sign(char_u *name)
{
    sign_T	*sp;
    sign_T	*lp;
    int	start = next_sign_typenr;

    // Allocate a new sign.
    sp = alloc_clear_id(sizeof(sign_T), aid_sign_define_by_name);
    if (sp == NULL)
	return NULL;

    // Check that next_sign_typenr is not already being used.
    // This only happens after wrapping around.  Hopefully
    // another one got deleted and we can use its number.
    for (lp = first_sign; lp != NULL; )
    {
	if (lp->sn_typenr == next_sign_typenr)
	{
	    ++next_sign_typenr;
	    if (next_sign_typenr == MAX_TYPENR)
		next_sign_typenr = 1;
	    if (next_sign_typenr == start)
	    {
		vim_free(sp);
		emsg(_(e_too_many_signs_defined));
		return NULL;
	    }
	    lp = first_sign;  // start all over
	    continue;
	}
	lp = lp->sn_next;
    }

    sp->sn_typenr = next_sign_typenr;
    if (++next_sign_typenr == MAX_TYPENR)
	next_sign_typenr = 1; // wrap around

    sp->sn_name = vim_strsave(name);
    if (sp->sn_name == NULL)  // out of memory
    {
	vim_free(sp);
	return NULL;
    }

    return sp;
}

/*
 * Initialize the icon information for a new sign
 */
    static void
sign_define_init_icon(sign_T *sp, char_u *icon)
{
    vim_free(sp->sn_icon);
    sp->sn_icon = vim_strsave(icon);
    backslash_halve(sp->sn_icon);
# ifdef FEAT_SIGN_ICONS
    if (gui.in_use)
    {
	out_flush();
	if (sp->sn_image != NULL)
	    gui_mch_destroy_sign(sp->sn_image);
	sp->sn_image = gui_mch_register_sign(sp->sn_icon);
    }
# endif
}

/*
 * Initialize the text for a new sign
 */
    static int
sign_define_init_text(sign_T *sp, char_u *text)
{
    char_u	*s;
    char_u	*endp;
    int		cells;
    int		len;

    endp = text + (int)STRLEN(text);

    // Remove backslashes so that it is possible to use a space.
    for (s = text; s + 1 < endp; ++s)
	if (*s == '\\')
	{
	    STRMOVE(s, s + 1);
	    --endp;
	}

    // Count cells and check for non-printable chars
    if (has_mbyte)
    {
	cells = 0;
	for (s = text; s < endp; s += (*mb_ptr2len)(s))
	{
	    if (!vim_isprintc((*mb_ptr2char)(s)))
		break;
	    cells += (*mb_ptr2cells)(s);
	}
    }
    else
    {
	for (s = text; s < endp; ++s)
	    if (!vim_isprintc(*s))
		break;
	cells = (int)(s - text);
    }

    // Currently sign text must be one or two display cells
    if (s != endp || cells < 1 || cells > 2)
    {
	semsg(_(e_invalid_sign_text_str), text);
	return FAIL;
    }

    vim_free(sp->sn_text);
    // Allocate one byte more if we need to pad up
    // with a space.
    len = (int)(endp - text + ((cells == 1) ? 1 : 0));
    sp->sn_text = vim_strnsave(text, len);

    // For single character sign text, pad with a space.
    if (sp->sn_text != NULL && cells == 1)
	STRCPY(sp->sn_text + len - 1, " ");

    return OK;
}

/*
 * Define a new sign or update an existing sign
 */
    int
sign_define_by_name(
	char_u	*name,
	char_u	*icon,
	char_u	*linehl,
	char_u	*text,
	char_u	*texthl,
	char_u	*culhl,
	char_u	*numhl)
{
    sign_T	*sp_prev;
    sign_T	*sp;

    sp = sign_find(name, &sp_prev);
    if (sp == NULL)
    {
	sp = alloc_new_sign(name);
	if (sp == NULL)
	    return FAIL;

	// add the new sign to the list of signs
	if (sp_prev == NULL)
	    first_sign = sp;
	else
	    sp_prev->sn_next = sp;
    }
    else
    {
	win_T *wp;

	// Signs may already exist, a redraw is needed in windows with a
	// non-empty sign list.
	FOR_ALL_WINDOWS(wp)
	    if (wp->w_buffer->b_signlist != NULL)
		redraw_buf_later(wp->w_buffer, UPD_NOT_VALID);
    }

    // set values for a defined sign.
    if (icon != NULL)
	sign_define_init_icon(sp, icon);

    if (text != NULL && (sign_define_init_text(sp, text) == FAIL))
	return FAIL;

    if (linehl != NULL)
    {
	if (*linehl == NUL)
	    sp->sn_line_hl = 0;
	else
	    sp->sn_line_hl = syn_check_group(linehl, (int)STRLEN(linehl));
    }

    if (texthl != NULL)
    {
	if (*texthl == NUL)
	    sp->sn_text_hl = 0;
	else
	    sp->sn_text_hl = syn_check_group(texthl, (int)STRLEN(texthl));
    }

    if (culhl != NULL)
    {
	if (*culhl == NUL)
	    sp->sn_cul_hl = 0;
	else
	    sp->sn_cul_hl = syn_check_group(culhl, (int)STRLEN(culhl));
    }

    if (numhl != NULL)
    {
	if (*numhl == NUL)
	    sp->sn_num_hl = 0;
	else
	    sp->sn_num_hl = syn_check_group(numhl, (int)STRLEN(numhl));
    }

    return OK;
}

/*
 * Return TRUE if sign "name" exists.
 */
    int
sign_exists_by_name(char_u *name)
{
    return sign_find(name, NULL) != NULL;
}

/*
 * Free the sign specified by 'name'.
 */
    int
sign_undefine_by_name(char_u *name, int give_error)
{
    sign_T	*sp_prev;
    sign_T	*sp;

    sp = sign_find(name, &sp_prev);
    if (sp == NULL)
    {
	if (give_error)
	    semsg(_(e_unknown_sign_str), name);
	return FAIL;
    }
    sign_undefine(sp, sp_prev);

    return OK;
}

/*
 * List the signs matching 'name'
 */
    static void
sign_list_by_name(char_u *name)
{
    sign_T	*sp;

    sp = sign_find(name, NULL);
    if (sp != NULL)
	sign_list_defined(sp);
    else
	semsg(_(e_unknown_sign_str), name);
}

    static void
may_force_numberwidth_recompute(buf_T *buf, int unplace)
{
    tabpage_T	*tp;
    win_T		*wp;

    FOR_ALL_TAB_WINDOWS(tp, wp)
	if (wp->w_buffer == buf
		&& (wp->w_p_nu || wp->w_p_rnu)
		&& (unplace || wp->w_nrwidth_width < 2)
		&& (*wp->w_p_scl == 'n' && *(wp->w_p_scl + 1) == 'u'))
	    wp->w_nrwidth_line_count = 0;
}

/*
 * Place a sign at the specified file location or update a sign.
 */
    int
sign_place(
	int		*sign_id,
	char_u		*sign_group,
	char_u		*sign_name,
	buf_T		*buf,
	linenr_T	lnum,
	int		prio)
{
    sign_T	*sp;

    // Check for reserved character '*' in group name
    if (sign_group != NULL && (*sign_group == '*' || *sign_group == '\0'))
	return FAIL;

    FOR_ALL_SIGNS(sp)
	if (STRCMP(sp->sn_name, sign_name) == 0)
	    break;
    if (sp == NULL)
    {
	semsg(_(e_unknown_sign_str), sign_name);
	return FAIL;
    }
    if (*sign_id == 0)
	*sign_id = sign_group_get_next_signid(buf, sign_group);

    if (lnum > 0)
	// ":sign place {id} line={lnum} name={name} file={fname}":
	// place a sign
	buf_addsign(buf, *sign_id, sign_group, prio, lnum, sp->sn_typenr);
    else
	// ":sign place {id} file={fname}": change sign type and/or priority
	lnum = buf_change_sign_type(buf, *sign_id, sign_group, sp->sn_typenr,
								prio);
    if (lnum > 0)
    {
	redraw_buf_line_later(buf, lnum);

	// When displaying signs in the 'number' column, if the width of the
	// number column is less than 2, then force recomputing the width.
	may_force_numberwidth_recompute(buf, FALSE);
    }
    else
    {
	semsg(_(e_not_possible_to_change_sign_str), sign_name);
	return FAIL;
    }

    return OK;
}

/*
 * Unplace the specified sign
 */
    static int
sign_unplace(int sign_id, char_u *sign_group, buf_T *buf, linenr_T atlnum)
{
    if (buf->b_signlist == NULL)	// No signs in the buffer
	return OK;

    if (sign_id == 0)
    {
	// Delete all the signs in the specified buffer
	redraw_buf_later(buf, UPD_NOT_VALID);
	buf_delete_signs(buf, sign_group);
    }
    else
    {
	linenr_T	lnum;

	// Delete only the specified signs
	lnum = buf_delsign(buf, atlnum, sign_id, sign_group);
	if (lnum == 0)
	    return FAIL;
    }

    // When all the signs in a buffer are removed, force recomputing the
    // number column width (if enabled) in all the windows displaying the
    // buffer if 'signcolumn' is set to 'number' in that window.
    if (buf->b_signlist == NULL)
	may_force_numberwidth_recompute(buf, TRUE);

    return OK;
}

/*
 * Unplace the sign at the current cursor line.
 */
    static void
sign_unplace_at_cursor(char_u *groupname)
{
    int		id = -1;

    id = buf_findsign_id(curwin->w_buffer, curwin->w_cursor.lnum, groupname);
    if (id > 0)
	sign_unplace(id, groupname, curwin->w_buffer, curwin->w_cursor.lnum);
    else
	emsg(_(e_missing_sign_number));
}

/*
 * Jump to a sign.
 */
    static linenr_T
sign_jump(int sign_id, char_u *sign_group, buf_T *buf)
{
    linenr_T	lnum;

    if ((lnum = buf_findsign(buf, sign_id, sign_group)) <= 0)
    {
	semsg(_(e_invalid_sign_id_nr), sign_id);
	return -1;
    }

    // goto a sign ...
    if (buf_jump_open_win(buf) != NULL)
    {			// ... in a current window
	curwin->w_cursor.lnum = lnum;
	check_cursor_lnum();
	beginline(BL_WHITE);
    }
    else
    {			// ... not currently in a window
	char_u	*cmd;

	if (buf->b_fname == NULL)
	{
	    emsg(_(e_cannot_jump_to_buffer_that_does_not_have_name));
	    return -1;
	}
	cmd = alloc(STRLEN(buf->b_fname) + 25);
	if (cmd == NULL)
	    return -1;
	sprintf((char *)cmd, "e +%ld %s", (long)lnum, buf->b_fname);
	do_cmdline_cmd(cmd);
	vim_free(cmd);
    }
# ifdef FEAT_FOLDING
    foldOpenCursor();
# endif

    return lnum;
}

/*
 * ":sign define {name} ..." command
 */
    static void
sign_define_cmd(char_u *sign_name, char_u *cmdline)
{
    char_u	*arg;
    char_u	*p = cmdline;
    char_u	*icon = NULL;
    char_u	*text = NULL;
    char_u	*linehl = NULL;
    char_u	*texthl = NULL;
    char_u	*culhl = NULL;
    char_u	*numhl = NULL;
    int		failed = FALSE;

    // set values for a defined sign.
    for (;;)
    {
	arg = skipwhite(p);
	if (*arg == NUL)
	    break;
	p = skiptowhite_esc(arg);
	if (STRNCMP(arg, "icon=", 5) == 0)
	{
	    arg += 5;
	    icon = vim_strnsave(arg, p - arg);
	}
	else if (STRNCMP(arg, "text=", 5) == 0)
	{
	    arg += 5;
	    text = vim_strnsave(arg, p - arg);
	}
	else if (STRNCMP(arg, "linehl=", 7) == 0)
	{
	    arg += 7;
	    linehl = vim_strnsave(arg, p - arg);
	}
	else if (STRNCMP(arg, "texthl=", 7) == 0)
	{
	    arg += 7;
	    texthl = vim_strnsave(arg, p - arg);
	}
	else if (STRNCMP(arg, "culhl=", 6) == 0)
	{
	    arg += 6;
	    culhl = vim_strnsave(arg, p - arg);
	}
	else if (STRNCMP(arg, "numhl=", 6) == 0)
	{
	    arg += 6;
	    numhl = vim_strnsave(arg, p - arg);
	}
	else
	{
	    semsg(_(e_invalid_argument_str), arg);
	    failed = TRUE;
	    break;
	}
    }

    if (!failed)
	sign_define_by_name(sign_name, icon, linehl, text, texthl, culhl, numhl);

    vim_free(icon);
    vim_free(text);
    vim_free(linehl);
    vim_free(texthl);
    vim_free(culhl);
    vim_free(numhl);
}

/*
 * ":sign place" command
 */
    static void
sign_place_cmd(
	buf_T		*buf,
	linenr_T	lnum,
	char_u		*sign_name,
	int		id,
	char_u		*group,
	int		prio)
{
    if (id <= 0)
    {
	// List signs placed in a file/buffer
	//   :sign place file={fname}
	//   :sign place group={group} file={fname}
	//   :sign place group=* file={fname}
	//   :sign place buffer={nr}
	//   :sign place group={group} buffer={nr}
	//   :sign place group=* buffer={nr}
	//   :sign place
	//   :sign place group={group}
	//   :sign place group=*
	if (lnum >= 0 || sign_name != NULL
		|| (group != NULL && *group == '\0'))
	    emsg(_(e_invalid_argument));
	else
	    sign_list_placed(buf, group);
    }
    else
    {
	// Place a new sign
	if (sign_name == NULL || buf == NULL
		|| (group != NULL && *group == '\0'))
	{
	    emsg(_(e_invalid_argument));
	    return;
	}

	sign_place(&id, group, sign_name, buf, lnum, prio);
    }
}

/*
 * ":sign unplace" command
 */
    static void
sign_unplace_cmd(
	buf_T		*buf,
	linenr_T	lnum,
	char_u		*sign_name,
	int		id,
	char_u		*group)
{
    if (lnum >= 0 || sign_name != NULL || (group != NULL && *group == '\0'))
    {
	emsg(_(e_invalid_argument));
	return;
    }

    if (id == -2)
    {
	if (buf != NULL)
	    // :sign unplace * file={fname}
	    // :sign unplace * group={group} file={fname}
	    // :sign unplace * group=* file={fname}
	    // :sign unplace * buffer={nr}
	    // :sign unplace * group={group} buffer={nr}
	    // :sign unplace * group=* buffer={nr}
	    sign_unplace(0, group, buf, 0);
	else
	    // :sign unplace *
	    // :sign unplace * group={group}
	    // :sign unplace * group=*
	    FOR_ALL_BUFFERS(buf)
		if (buf->b_signlist != NULL)
		    buf_delete_signs(buf, group);
    }
    else
    {
	if (buf != NULL)
	    // :sign unplace {id} file={fname}
	    // :sign unplace {id} group={group} file={fname}
	    // :sign unplace {id} group=* file={fname}
	    // :sign unplace {id} buffer={nr}
	    // :sign unplace {id} group={group} buffer={nr}
	    // :sign unplace {id} group=* buffer={nr}
	    sign_unplace(id, group, buf, 0);
	else
	{
	    if (id == -1)
	    {
		// :sign unplace group={group}
		// :sign unplace group=*
		sign_unplace_at_cursor(group);
	    }
	    else
	    {
		// :sign unplace {id}
		// :sign unplace {id} group={group}
		// :sign unplace {id} group=*
		FOR_ALL_BUFFERS(buf)
		    sign_unplace(id, group, buf, 0);
	    }
	}
    }
}

/*
 * Jump to a placed sign commands:
 *   :sign jump {id} file={fname}
 *   :sign jump {id} buffer={nr}
 *   :sign jump {id} group={group} file={fname}
 *   :sign jump {id} group={group} buffer={nr}
 */
    static void
sign_jump_cmd(
	buf_T		*buf,
	linenr_T	lnum,
	char_u		*sign_name,
	int		id,
	char_u		*group)
{
    if (sign_name == NULL && group == NULL && id == -1)
    {
	emsg(_(e_argument_required));
	return;
    }

    if (buf == NULL || (group != NULL && *group == '\0')
					     || lnum >= 0 || sign_name != NULL)
    {
	// File or buffer is not specified or an empty group is used
	// or a line number or a sign name is specified.
	emsg(_(e_invalid_argument));
	return;
    }
    (void)sign_jump(id, group, buf);
}

/*
 * Parse the command line arguments for the ":sign place", ":sign unplace" and
 * ":sign jump" commands.
 * The supported arguments are: line={lnum} name={name} group={group}
 * priority={prio} and file={fname} or buffer={nr}.
 */
    static int
parse_sign_cmd_args(
	int	    cmd,
	char_u	    *arg,
	char_u	    **sign_name,
	int	    *signid,
	char_u	    **group,
	int	    *prio,
	buf_T	    **buf,
	linenr_T    *lnum)
{
    char_u	*arg1;
    char_u	*name;
    char_u	*filename = NULL;
    int		lnum_arg = FALSE;

    // first arg could be placed sign id
    arg1 = arg;
    if (VIM_ISDIGIT(*arg))
    {
	*signid = getdigits(&arg);
	if (!VIM_ISWHITE(*arg) && *arg != NUL)
	{
	    *signid = -1;
	    arg = arg1;
	}
	else
	    arg = skipwhite(arg);
    }

    while (*arg != NUL)
    {
	if (STRNCMP(arg, "line=", 5) == 0)
	{
	    arg += 5;
	    *lnum = atoi((char *)arg);
	    arg = skiptowhite(arg);
	    lnum_arg = TRUE;
	}
	else if (STRNCMP(arg, "*", 1) == 0 && cmd == SIGNCMD_UNPLACE)
	{
	    if (*signid != -1)
	    {
		emsg(_(e_invalid_argument));
		return FAIL;
	    }
	    *signid = -2;
	    arg = skiptowhite(arg + 1);
	}
	else if (STRNCMP(arg, "name=", 5) == 0)
	{
	    arg += 5;
	    name = arg;
	    arg = skiptowhite(arg);
	    if (*arg != NUL)
		*arg++ = NUL;
	    while (name[0] == '0' && name[1] != NUL)
		++name;
	    *sign_name = name;
	}
	else if (STRNCMP(arg, "group=", 6) == 0)
	{
	    arg += 6;
	    *group = arg;
	    arg = skiptowhite(arg);
	    if (*arg != NUL)
		*arg++ = NUL;
	}
	else if (STRNCMP(arg, "priority=", 9) == 0)
	{
	    arg += 9;
	    *prio = atoi((char *)arg);
	    arg = skiptowhite(arg);
	}
	else if (STRNCMP(arg, "file=", 5) == 0)
	{
	    arg += 5;
	    filename = arg;
	    *buf = buflist_findname_exp(arg);
	    break;
	}
	else if (STRNCMP(arg, "buffer=", 7) == 0)
	{
	    arg += 7;
	    filename = arg;
	    *buf = buflist_findnr((int)getdigits(&arg));
	    if (*skipwhite(arg) != NUL)
		semsg(_(e_trailing_characters_str), arg);
	    break;
	}
	else
	{
	    emsg(_(e_invalid_argument));
	    return FAIL;
	}
	arg = skipwhite(arg);
    }

    if (filename != NULL && *buf == NULL)
    {
	semsg(_(e_invalid_buffer_name_str), filename);
	return FAIL;
    }

    // If the filename is not supplied for the sign place or the sign jump
    // command, then use the current buffer.
    if (filename == NULL && ((cmd == SIGNCMD_PLACE && lnum_arg)
						       || cmd == SIGNCMD_JUMP))
	*buf = curwin->w_buffer;

    return OK;
}

/*
 * ":sign" command
 */
    void
ex_sign(exarg_T *eap)
{
    char_u	*arg = eap->arg;
    char_u	*p;
    int		idx;
    sign_T	*sp;
    buf_T	*buf = NULL;

    // Parse the subcommand.
    p = skiptowhite(arg);
    idx = sign_cmd_idx(arg, p);
    if (idx == SIGNCMD_LAST)
    {
	semsg(_(e_unknown_sign_command_str), arg);
	return;
    }
    arg = skipwhite(p);

    if (idx <= SIGNCMD_LIST)
    {
	// Define, undefine or list signs.
	if (idx == SIGNCMD_LIST && *arg == NUL)
	{
	    // ":sign list": list all defined signs
	    for (sp = first_sign; sp != NULL && !got_int; sp = sp->sn_next)
		sign_list_defined(sp);
	}
	else if (*arg == NUL)
	    emsg(_(e_missing_sign_name));
	else
	{
	    char_u	*name;

	    // Isolate the sign name.  If it's a number skip leading zeroes,
	    // so that "099" and "99" are the same sign.  But keep "0".
	    p = skiptowhite(arg);
	    if (*p != NUL)
		*p++ = NUL;
	    while (arg[0] == '0' && arg[1] != NUL)
		++arg;
	    name = vim_strsave(arg);

	    if (idx == SIGNCMD_DEFINE)
		sign_define_cmd(name, p);
	    else if (idx == SIGNCMD_LIST)
		// ":sign list {name}"
		sign_list_by_name(name);
	    else
		// ":sign undefine {name}"
		sign_undefine_by_name(name, TRUE);

	    vim_free(name);
	    return;
	}
    }
    else
    {
	int		id = -1;
	linenr_T	lnum = -1;
	char_u		*sign_name = NULL;
	char_u		*group = NULL;
	int		prio = SIGN_DEF_PRIO;

	// Parse command line arguments
	if (parse_sign_cmd_args(idx, arg, &sign_name, &id, &group, &prio,
							  &buf, &lnum) == FAIL)
	    return;

	if (idx == SIGNCMD_PLACE)
	    sign_place_cmd(buf, lnum, sign_name, id, group, prio);
	else if (idx == SIGNCMD_UNPLACE)
	    sign_unplace_cmd(buf, lnum, sign_name, id, group);
	else if (idx == SIGNCMD_JUMP)
	    sign_jump_cmd(buf, lnum, sign_name, id, group);
    }
}

/*
 * Return information about a specified sign
 */
    static void
sign_getinfo(sign_T *sp, dict_T *retdict)
{
    char_u	*p;

    dict_add_string(retdict, "name", sp->sn_name);
    if (sp->sn_icon != NULL)
	dict_add_string(retdict, "icon", sp->sn_icon);
    if (sp->sn_text != NULL)
	dict_add_string(retdict, "text", sp->sn_text);
    if (sp->sn_line_hl > 0)
    {
	p = get_highlight_name_ext(NULL, sp->sn_line_hl - 1, FALSE);
	if (p == NULL)
	    p = (char_u *)"NONE";
	dict_add_string(retdict, "linehl", p);
    }
    if (sp->sn_text_hl > 0)
    {
	p = get_highlight_name_ext(NULL, sp->sn_text_hl - 1, FALSE);
	if (p == NULL)
	    p = (char_u *)"NONE";
	dict_add_string(retdict, "texthl", p);
    }
    if (sp->sn_cul_hl > 0)
    {
	p = get_highlight_name_ext(NULL, sp->sn_cul_hl - 1, FALSE);
	if (p == NULL)
	    p = (char_u *)"NONE";
	dict_add_string(retdict, "culhl", p);
    }
    if (sp->sn_num_hl > 0)
    {
	p = get_highlight_name_ext(NULL, sp->sn_num_hl - 1, FALSE);
	if (p == NULL)
	    p = (char_u *)"NONE";
	dict_add_string(retdict, "numhl", p);
    }
}

/*
 * If 'name' is NULL, return a list of all the defined signs.
 * Otherwise, return information about the specified sign.
 */
    static void
sign_getlist(char_u *name, list_T *retlist)
{
    sign_T	*sp = first_sign;
    dict_T	*dict;

    if (name != NULL)
    {
	sp = sign_find(name, NULL);
	if (sp == NULL)
	    return;
    }

    for (; sp != NULL && !got_int; sp = sp->sn_next)
    {
	if ((dict = dict_alloc_id(aid_sign_getlist)) == NULL)
	    return;
	if (list_append_dict(retlist, dict) == FAIL)
	    return;
	sign_getinfo(sp, dict);

	if (name != NULL)	    // handle only the specified sign
	    break;
    }
}

/*
 * Returns information about signs placed in a buffer as list of dicts.
 */
    void
get_buffer_signs(buf_T *buf, list_T *l)
{
    sign_entry_T	*sign;
    dict_T		*d;

    FOR_ALL_SIGNS_IN_BUF(buf, sign)
    {
	if ((d = sign_get_info(sign)) != NULL)
	    list_append_dict(l, d);
    }
}

/*
 * Return information about all the signs placed in a buffer
 */
    static void
sign_get_placed_in_buf(
	buf_T		*buf,
	linenr_T	lnum,
	int		sign_id,
	char_u		*sign_group,
	list_T		*retlist)
{
    dict_T		*d;
    list_T		*l;
    sign_entry_T	*sign;
    dict_T		*sdict;

    if ((d = dict_alloc_id(aid_sign_getplaced_dict)) == NULL)
	return;
    list_append_dict(retlist, d);

    dict_add_number(d, "bufnr", (long)buf->b_fnum);

    if ((l = list_alloc_id(aid_sign_getplaced_list)) == NULL)
	return;
    dict_add_list(d, "signs", l);

    FOR_ALL_SIGNS_IN_BUF(buf, sign)
    {
	if (!sign_in_group(sign, sign_group))
	    continue;
	if ((lnum == 0 && sign_id == 0)
		|| (sign_id == 0 && lnum == sign->se_lnum)
		|| (lnum == 0 && sign_id == sign->se_id)
		|| (lnum == sign->se_lnum && sign_id == sign->se_id))
	{
	    if ((sdict = sign_get_info(sign)) != NULL)
		list_append_dict(l, sdict);
	}
    }
}

/*
 * Get a list of signs placed in buffer 'buf'. If 'num' is non-zero, return the
 * sign placed at the line number. If 'lnum' is zero, return all the signs
 * placed in 'buf'. If 'buf' is NULL, return signs placed in all the buffers.
 */
    static void
sign_get_placed(
	buf_T		*buf,
	linenr_T	lnum,
	int		sign_id,
	char_u		*sign_group,
	list_T		*retlist)
{
    if (buf != NULL)
	sign_get_placed_in_buf(buf, lnum, sign_id, sign_group, retlist);
    else
    {
	FOR_ALL_BUFFERS(buf)
	    if (buf->b_signlist != NULL)
		sign_get_placed_in_buf(buf, 0, sign_id, sign_group, retlist);
    }
}

# if defined(FEAT_SIGN_ICONS) || defined(PROTO)
/*
 * Allocate the icons.  Called when the GUI has started.  Allows defining
 * signs before it starts.
 */
    void
sign_gui_started(void)
{
    sign_T	*sp;

    FOR_ALL_SIGNS(sp)
	if (sp->sn_icon != NULL)
	    sp->sn_image = gui_mch_register_sign(sp->sn_icon);
}
# endif

/*
 * List one sign.
 */
    static void
sign_list_defined(sign_T *sp)
{
    char_u	*p;

    smsg("sign %s", sp->sn_name);
    if (sp->sn_icon != NULL)
    {
	msg_puts(" icon=");
	msg_outtrans(sp->sn_icon);
# ifdef FEAT_SIGN_ICONS
	if (sp->sn_image == NULL)
	    msg_puts(_(" (NOT FOUND)"));
# else
	msg_puts(_(" (not supported)"));
# endif
    }
    if (sp->sn_text != NULL)
    {
	msg_puts(" text=");
	msg_outtrans(sp->sn_text);
    }
    if (sp->sn_line_hl > 0)
    {
	msg_puts(" linehl=");
	p = get_highlight_name_ext(NULL, sp->sn_line_hl - 1, FALSE);
	if (p == NULL)
	    msg_puts("NONE");
	else
	    msg_puts((char *)p);
    }
    if (sp->sn_text_hl > 0)
    {
	msg_puts(" texthl=");
	p = get_highlight_name_ext(NULL, sp->sn_text_hl - 1, FALSE);
	if (p == NULL)
	    msg_puts("NONE");
	else
	    msg_puts((char *)p);
    }
    if (sp->sn_cul_hl > 0)
    {
	msg_puts(" culhl=");
	p = get_highlight_name_ext(NULL, sp->sn_cul_hl - 1, FALSE);
	if (p == NULL)
	    msg_puts("NONE");
	else
	    msg_puts((char *)p);
    }
    if (sp->sn_num_hl > 0)
    {
	msg_puts(" numhl=");
	p = get_highlight_name_ext(NULL, sp->sn_num_hl - 1, FALSE);
	if (p == NULL)
	    msg_puts("NONE");
	else
	    msg_puts((char *)p);
    }
}

/*
 * Undefine a sign and free its memory.
 */
    static void
sign_undefine(sign_T *sp, sign_T *sp_prev)
{
    vim_free(sp->sn_name);
    vim_free(sp->sn_icon);
# ifdef FEAT_SIGN_ICONS
    if (sp->sn_image != NULL)
    {
	out_flush();
	gui_mch_destroy_sign(sp->sn_image);
    }
# endif
    vim_free(sp->sn_text);
    if (sp_prev == NULL)
	first_sign = sp->sn_next;
    else
	sp_prev->sn_next = sp->sn_next;
    vim_free(sp);
}

# if defined(FEAT_SIGN_ICONS) || defined(PROTO)
    void *
sign_get_image(
    int		typenr)		// the attribute which may have a sign
{
    sign_T	*sp;

    FOR_ALL_SIGNS(sp)
	if (sp->sn_typenr == typenr)
	    return sp->sn_image;
    return NULL;
}
# endif

/*
 * Undefine/free all signs.
 */
    void
free_signs(void)
{
    while (first_sign != NULL)
	sign_undefine(first_sign, NULL);
}

static enum
{
    EXP_SUBCMD,		// expand :sign sub-commands
    EXP_DEFINE,		// expand :sign define {name} args
    EXP_PLACE,		// expand :sign place {id} args
    EXP_LIST,		// expand :sign place args
    EXP_UNPLACE,	// expand :sign unplace"
    EXP_SIGN_NAMES,	// expand with name of placed signs
    EXP_SIGN_GROUPS	// expand with name of placed sign groups
} expand_what;

/*
 * Return the n'th sign name (used for command line completion)
 */
    static char_u *
get_nth_sign_name(int idx)
{
    int		current_idx;
    sign_T	*sp;

    // Complete with name of signs already defined
    current_idx = 0;
    FOR_ALL_SIGNS(sp)
	if (current_idx++ == idx)
	    return sp->sn_name;
    return NULL;
}

/*
 * Return the n'th sign group name (used for command line completion)
 */
    static char_u *
get_nth_sign_group_name(int idx)
{
    int		current_idx;
    int		todo;
    hashitem_T	*hi;
    signgroup_T	*group;

    // Complete with name of sign groups already defined
    current_idx = 0;
    todo = (int)sg_table.ht_used;
    FOR_ALL_HASHTAB_ITEMS(&sg_table, hi, todo)
    {
	if (!HASHITEM_EMPTY(hi))
	{
	    --todo;
	    if (current_idx++ == idx)
	    {
		group = HI2SG(hi);
		return group->sg_name;
	    }
	}
    }
    return NULL;
}

/*
 * Function given to ExpandGeneric() to obtain the sign command
 * expansion.
 */
    char_u *
get_sign_name(expand_T *xp UNUSED, int idx)
{
    switch (expand_what)
    {
    case EXP_SUBCMD:
	return (char_u *)cmds[idx];
    case EXP_DEFINE:
	{
	    char *define_arg[] =
	    {
		"culhl=", "icon=", "linehl=", "numhl=", "text=", "texthl=", NULL
	    };
	    return (char_u *)define_arg[idx];
	}
    case EXP_PLACE:
	{
	    char *place_arg[] =
	    {
		"line=", "name=", "group=", "priority=", "file=",
		"buffer=", NULL
	    };
	    return (char_u *)place_arg[idx];
	}
    case EXP_LIST:
	{
	    char *list_arg[] =
	    {
		"group=", "file=", "buffer=", NULL
	    };
	    return (char_u *)list_arg[idx];
	}
    case EXP_UNPLACE:
	{
	    char *unplace_arg[] = { "group=", "file=", "buffer=", NULL };
	    return (char_u *)unplace_arg[idx];
	}
    case EXP_SIGN_NAMES:
	return get_nth_sign_name(idx);
    case EXP_SIGN_GROUPS:
	return get_nth_sign_group_name(idx);
    default:
	return NULL;
    }
}

/*
 * Handle command line completion for :sign command.
 */
    void
set_context_in_sign_cmd(expand_T *xp, char_u *arg)
{
    char_u	*p;
    char_u	*end_subcmd;
    char_u	*last;
    int		cmd_idx;
    char_u	*begin_subcmd_args;

    // Default: expand subcommands.
    xp->xp_context = EXPAND_SIGN;
    expand_what = EXP_SUBCMD;
    xp->xp_pattern = arg;

    end_subcmd = skiptowhite(arg);
    if (*end_subcmd == NUL)
	// expand subcmd name
	// :sign {subcmd}<CTRL-D>
	return;

    cmd_idx = sign_cmd_idx(arg, end_subcmd);

    // :sign {subcmd} {subcmd_args}
    //		      |
    //		      begin_subcmd_args
    begin_subcmd_args = skipwhite(end_subcmd);

    // expand last argument of subcmd

    // :sign define {name} {args}...
    //		    |
    //		    p

    // Loop until reaching last argument.
    p = begin_subcmd_args;
    do
    {
	p = skipwhite(p);
	last = p;
	p = skiptowhite(p);
    } while (*p != NUL);

    p = vim_strchr(last, '=');

    // :sign define {name} {args}... {last}=
    //				     |	   |
    //				  last	   p
    if (p == NULL)
    {
	// Expand last argument name (before equal sign).
	xp->xp_pattern = last;
	switch (cmd_idx)
	{
	    case SIGNCMD_DEFINE:
		expand_what = EXP_DEFINE;
		break;
	    case SIGNCMD_PLACE:
		// List placed signs
		if (VIM_ISDIGIT(*begin_subcmd_args))
		    //   :sign place {id} {args}...
		    expand_what = EXP_PLACE;
		else
		    //   :sign place {args}...
		    expand_what = EXP_LIST;
		break;
	    case SIGNCMD_LIST:
	    case SIGNCMD_UNDEFINE:
		// :sign list <CTRL-D>
		// :sign undefine <CTRL-D>
		expand_what = EXP_SIGN_NAMES;
		break;
	    case SIGNCMD_JUMP:
	    case SIGNCMD_UNPLACE:
		expand_what = EXP_UNPLACE;
		break;
	    default:
		xp->xp_context = EXPAND_NOTHING;
	}
    }
    else
    {
	// Expand last argument value (after equal sign).
	xp->xp_pattern = p + 1;
	switch (cmd_idx)
	{
	    case SIGNCMD_DEFINE:
		if (STRNCMP(last, "texthl", 6) == 0
			|| STRNCMP(last, "linehl", 6) == 0
			|| STRNCMP(last, "culhl", 5) == 0
			|| STRNCMP(last, "numhl", 5) == 0)
		    xp->xp_context = EXPAND_HIGHLIGHT;
		else if (STRNCMP(last, "icon", 4) == 0)
		    xp->xp_context = EXPAND_FILES;
		else
		    xp->xp_context = EXPAND_NOTHING;
		break;
	    case SIGNCMD_PLACE:
		if (STRNCMP(last, "name", 4) == 0)
		    expand_what = EXP_SIGN_NAMES;
		else if (STRNCMP(last, "group", 5) == 0)
		    expand_what = EXP_SIGN_GROUPS;
		else if (STRNCMP(last, "file", 4) == 0)
		    xp->xp_context = EXPAND_BUFFERS;
		else
		    xp->xp_context = EXPAND_NOTHING;
		break;
	    case SIGNCMD_UNPLACE:
	    case SIGNCMD_JUMP:
		if (STRNCMP(last, "group", 5) == 0)
		    expand_what = EXP_SIGN_GROUPS;
		else if (STRNCMP(last, "file", 4) == 0)
		    xp->xp_context = EXPAND_BUFFERS;
		else
		    xp->xp_context = EXPAND_NOTHING;
		break;
	    default:
		xp->xp_context = EXPAND_NOTHING;
	}
    }
}

/*
 * Define a sign using the attributes in 'dict'. Returns 0 on success and -1 on
 * failure.
 */
    static int
sign_define_from_dict(char_u *name_arg, dict_T *dict)
{
    char_u	*name = NULL;
    char_u	*icon = NULL;
    char_u	*linehl = NULL;
    char_u	*text = NULL;
    char_u	*texthl = NULL;
    char_u	*culhl = NULL;
    char_u	*numhl = NULL;
    int		retval = -1;

    if (name_arg == NULL)
    {
	if (dict == NULL)
	    return -1;
	name = dict_get_string(dict, "name", TRUE);
    }
    else
	name = vim_strsave(name_arg);
    if (name == NULL || name[0] == NUL)
	goto cleanup;
    if (dict != NULL)
    {
	icon = dict_get_string(dict, "icon", TRUE);
	linehl = dict_get_string(dict, "linehl", TRUE);
	text = dict_get_string(dict, "text", TRUE);
	texthl = dict_get_string(dict, "texthl", TRUE);
	culhl = dict_get_string(dict, "culhl", TRUE);
	numhl = dict_get_string(dict, "numhl", TRUE);
    }

    if (sign_define_by_name(name, icon, linehl, text, texthl, culhl, numhl) == OK)
	retval = 0;

cleanup:
    vim_free(name);
    vim_free(icon);
    vim_free(linehl);
    vim_free(text);
    vim_free(texthl);
    vim_free(culhl);
    vim_free(numhl);

    return retval;
}

/*
 * Define multiple signs using attributes from list 'l' and store the return
 * values in 'retlist'.
 */
    static void
sign_define_multiple(list_T *l, list_T *retlist)
{
    listitem_T	*li;
    int		retval;

    FOR_ALL_LIST_ITEMS(l, li)
    {
	retval = -1;
	if (li->li_tv.v_type == VAR_DICT)
	    retval = sign_define_from_dict(NULL, li->li_tv.vval.v_dict);
	else
	    emsg(_(e_dictionary_required));
	list_append_number(retlist, retval);
    }
}

/*
 * "sign_define()" function
 */
    void
f_sign_define(typval_T *argvars, typval_T *rettv)
{
    char_u	*name;

    if (in_vim9script()
	    && (check_for_string_or_list_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 1) == FAIL))
	return;

    if (argvars[0].v_type == VAR_LIST && argvars[1].v_type == VAR_UNKNOWN)
    {
	// Define multiple signs
	if (rettv_list_alloc(rettv) == FAIL)
	    return;

	sign_define_multiple(argvars[0].vval.v_list, rettv->vval.v_list);
	return;
    }

    // Define a single sign
    rettv->vval.v_number = -1;

    name = tv_get_string_chk(&argvars[0]);
    if (name == NULL)
	return;

    if (check_for_opt_dict_arg(argvars, 1) == FAIL)
	return;

    rettv->vval.v_number = sign_define_from_dict(name,
	    argvars[1].v_type == VAR_DICT ? argvars[1].vval.v_dict : NULL);
}

/*
 * "sign_getdefined()" function
 */
    void
f_sign_getdefined(typval_T *argvars, typval_T *rettv)
{
    char_u	*name = NULL;

    if (rettv_list_alloc_id(rettv, aid_sign_getdefined) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_string_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
	name = tv_get_string(&argvars[0]);

    sign_getlist(name, rettv->vval.v_list);
}

/*
 * "sign_getplaced()" function
 */
    void
f_sign_getplaced(typval_T *argvars, typval_T *rettv)
{
    buf_T	*buf = NULL;
    dict_T	*dict;
    dictitem_T	*di;
    linenr_T	lnum = 0;
    int		sign_id = 0;
    char_u	*group = NULL;
    int		notanum = FALSE;

    if (rettv_list_alloc_id(rettv, aid_sign_getplaced) == FAIL)
	return;

    if (in_vim9script()
	    && (check_for_opt_buffer_arg(argvars, 0) == FAIL
		|| (argvars[0].v_type != VAR_UNKNOWN
		    && check_for_opt_dict_arg(argvars, 1) == FAIL)))
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	// get signs placed in the specified buffer
	buf = get_buf_arg(&argvars[0]);
	if (buf == NULL)
	    return;

	if (argvars[1].v_type != VAR_UNKNOWN)
	{
	    if (check_for_nonnull_dict_arg(argvars, 1) == FAIL)
		return;
	    dict = argvars[1].vval.v_dict;
	    if ((di = dict_find(dict, (char_u *)"lnum", -1)) != NULL)
	    {
		// get signs placed at this line
		(void)tv_get_number_chk(&di->di_tv, &notanum);
		if (notanum)
		    return;
		lnum = tv_get_lnum(&di->di_tv);
	    }
	    if ((di = dict_find(dict, (char_u *)"id", -1)) != NULL)
	    {
		// get sign placed with this identifier
		sign_id = (int)tv_get_number_chk(&di->di_tv, &notanum);
		if (notanum)
		    return;
	    }
	    if ((di = dict_find(dict, (char_u *)"group", -1)) != NULL)
	    {
		group = tv_get_string_chk(&di->di_tv);
		if (group == NULL)
		    return;
		if (*group == '\0')	// empty string means global group
		    group = NULL;
	    }
	}
    }

    sign_get_placed(buf, lnum, sign_id, group, rettv->vval.v_list);
}

/*
 * "sign_jump()" function
 */
    void
f_sign_jump(typval_T *argvars, typval_T *rettv)
{
    int		sign_id;
    char_u	*sign_group = NULL;
    buf_T	*buf;
    int		notanum = FALSE;

    rettv->vval.v_number = -1;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_buffer_arg(argvars, 2) == FAIL))
	return;

    // Sign identifier
    sign_id = (int)tv_get_number_chk(&argvars[0], &notanum);
    if (notanum)
	return;
    if (sign_id <= 0)
    {
	emsg(_(e_invalid_argument));
	return;
    }

    // Sign group
    sign_group = tv_get_string_chk(&argvars[1]);
    if (sign_group == NULL)
	return;
    if (sign_group[0] == '\0')
	sign_group = NULL;			// global sign group
    else
    {
	sign_group = vim_strsave(sign_group);
	if (sign_group == NULL)
	    return;
    }

    // Buffer to place the sign
    buf = get_buf_arg(&argvars[2]);
    if (buf == NULL)
	goto cleanup;

    rettv->vval.v_number = sign_jump(sign_id, sign_group, buf);

cleanup:
    vim_free(sign_group);
}

/*
 * Place a new sign using the values specified in dict 'dict'. Returns the sign
 * identifier if successfully placed, otherwise returns 0.
 */
    static int
sign_place_from_dict(
	typval_T	*id_tv,
	typval_T	*group_tv,
	typval_T	*name_tv,
	typval_T	*buf_tv,
	dict_T		*dict)
{
    int		sign_id = 0;
    char_u	*group = NULL;
    char_u	*sign_name = NULL;
    buf_T	*buf = NULL;
    dictitem_T	*di;
    linenr_T	lnum = 0;
    int		prio = SIGN_DEF_PRIO;
    int		notanum = FALSE;
    int		ret_sign_id = -1;

    // sign identifier
    if (id_tv == NULL)
    {
	di = dict_find(dict, (char_u *)"id", -1);
	if (di != NULL)
	    id_tv = &di->di_tv;
    }
    if (id_tv == NULL)
	sign_id = 0;
    else
    {
	sign_id = tv_get_number_chk(id_tv, &notanum);
	if (notanum)
	    return -1;
	if (sign_id < 0)
	{
	    emsg(_(e_invalid_argument));
	    return -1;
	}
    }

    // sign group
    if (group_tv == NULL)
    {
	di = dict_find(dict, (char_u *)"group", -1);
	if (di != NULL)
	    group_tv = &di->di_tv;
    }
    if (group_tv == NULL)
	group = NULL;				// global group
    else
    {
	group = tv_get_string_chk(group_tv);
	if (group == NULL)
	    goto cleanup;
	if (group[0] == '\0')			// global sign group
	    group = NULL;
	else
	{
	    group = vim_strsave(group);
	    if (group == NULL)
		return -1;
	}
    }

    // sign name
    if (name_tv == NULL)
    {
	di = dict_find(dict, (char_u *)"name", -1);
	if (di != NULL)
	    name_tv = &di->di_tv;
    }
    if (name_tv == NULL)
	goto cleanup;
    sign_name = tv_get_string_chk(name_tv);
    if (sign_name == NULL)
	goto cleanup;

    // buffer to place the sign
    if (buf_tv == NULL)
    {
	di = dict_find(dict, (char_u *)"buffer", -1);
	if (di != NULL)
	    buf_tv = &di->di_tv;
    }
    if (buf_tv == NULL)
	goto cleanup;
    buf = get_buf_arg(buf_tv);
    if (buf == NULL)
	goto cleanup;

    // line number of the sign
    di = dict_find(dict, (char_u *)"lnum", -1);
    if (di != NULL)
    {
	lnum = tv_get_lnum(&di->di_tv);
	if (lnum <= 0)
	{
	    emsg(_(e_invalid_argument));
	    goto cleanup;
	}
    }

    // sign priority
    di = dict_find(dict, (char_u *)"priority", -1);
    if (di != NULL)
    {
	prio = (int)tv_get_number_chk(&di->di_tv, &notanum);
	if (notanum)
	    goto cleanup;
    }

    if (sign_place(&sign_id, group, sign_name, buf, lnum, prio) == OK)
	ret_sign_id = sign_id;

cleanup:
    vim_free(group);

    return ret_sign_id;
}

/*
 * "sign_place()" function
 */
    void
f_sign_place(typval_T *argvars, typval_T *rettv)
{
    dict_T	*dict = NULL;

    rettv->vval.v_number = -1;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_string_arg(argvars, 2) == FAIL
		|| check_for_buffer_arg(argvars, 3) == FAIL
		|| check_for_opt_dict_arg(argvars, 4) == FAIL))
	return;

    if (argvars[4].v_type != VAR_UNKNOWN)
    {
	if (check_for_nonnull_dict_arg(argvars, 4) == FAIL)
	    return;
	dict = argvars[4].vval.v_dict;
    }

    rettv->vval.v_number = sign_place_from_dict(&argvars[0], &argvars[1],
					&argvars[2], &argvars[3], dict);
}

/*
 * "sign_placelist()" function.  Place multiple signs.
 */
    void
f_sign_placelist(typval_T *argvars, typval_T *rettv)
{
    listitem_T	*li;
    int		sign_id;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_list_arg(argvars, 0) == FAIL)
	return;

    if (check_for_list_arg(argvars, 0) == FAIL)
	return;

    // Process the List of sign attributes
    FOR_ALL_LIST_ITEMS(argvars[0].vval.v_list, li)
    {
	sign_id = -1;
	if (li->li_tv.v_type == VAR_DICT)
	    sign_id = sign_place_from_dict(NULL, NULL, NULL, NULL,
						li->li_tv.vval.v_dict);
	else
	    emsg(_(e_dictionary_required));
	list_append_number(rettv->vval.v_list, sign_id);
    }
}

/*
 * Undefine multiple signs
 */
    static void
sign_undefine_multiple(list_T *l, list_T *retlist)
{
    char_u	*name;
    listitem_T	*li;
    int		retval;

    FOR_ALL_LIST_ITEMS(l, li)
    {
	retval = -1;
	name = tv_get_string_chk(&li->li_tv);
	if (name != NULL && (sign_undefine_by_name(name, TRUE) == OK))
	    retval = 0;
	list_append_number(retlist, retval);
    }
}

/*
 * "sign_undefine()" function
 */
    void
f_sign_undefine(typval_T *argvars, typval_T *rettv)
{
    char_u *name;

    if (in_vim9script()
	    && check_for_opt_string_or_list_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_LIST && argvars[1].v_type == VAR_UNKNOWN)
    {
	// Undefine multiple signs
	if (rettv_list_alloc(rettv) == FAIL)
	    return;

	sign_undefine_multiple(argvars[0].vval.v_list, rettv->vval.v_list);
	return;
    }

    rettv->vval.v_number = -1;

    if (argvars[0].v_type == VAR_UNKNOWN)
    {
	// Free all the signs
	free_signs();
	rettv->vval.v_number = 0;
    }
    else
    {
	// Free only the specified sign
	name = tv_get_string_chk(&argvars[0]);
	if (name == NULL)
	    return;

	if (sign_undefine_by_name(name, TRUE) == OK)
	    rettv->vval.v_number = 0;
    }
}

/*
 * Unplace the sign with attributes specified in 'dict'. Returns 0 on success
 * and -1 on failure.
 */
    static int
sign_unplace_from_dict(typval_T *group_tv, dict_T *dict)
{
    dictitem_T	*di;
    int		sign_id = 0;
    buf_T	*buf = NULL;
    char_u	*group = NULL;
    int		retval = -1;

    // sign group
    if (group_tv != NULL)
	group = tv_get_string(group_tv);
    else
	group = dict_get_string(dict, "group", FALSE);
    if (group != NULL)
    {
	if (group[0] == '\0')			// global sign group
	    group = NULL;
	else
	{
	    group = vim_strsave(group);
	    if (group == NULL)
		return -1;
	}
    }

    if (dict != NULL)
    {
	if ((di = dict_find(dict, (char_u *)"buffer", -1)) != NULL)
	{
	    buf = get_buf_arg(&di->di_tv);
	    if (buf == NULL)
		goto cleanup;
	}
	if (dict_has_key(dict, "id"))
	{
	    sign_id = dict_get_number(dict, "id");
	    if (sign_id <= 0)
	    {
		emsg(_(e_invalid_argument));
		goto cleanup;
	    }
	}
    }

    if (buf == NULL)
    {
	// Delete the sign in all the buffers
	retval = 0;
	FOR_ALL_BUFFERS(buf)
	    if (sign_unplace(sign_id, group, buf, 0) != OK)
		retval = -1;
    }
    else if (sign_unplace(sign_id, group, buf, 0) == OK)
	retval = 0;

cleanup:
    vim_free(group);

    return retval;
}

    sign_entry_T *
get_first_valid_sign(win_T *wp)
{
    sign_entry_T *sign = wp->w_buffer->b_signlist;

# ifdef FEAT_PROP_POPUP
    while (sign != NULL && !sign_group_for_window(sign, wp))
	sign = sign->se_next;
# endif
    return sign;
}

/*
 * Return TRUE when window "wp" has a column to draw signs in.
 */
     int
signcolumn_on(win_T *wp)
{
    // If 'signcolumn' is set to 'number', signs are displayed in the 'number'
    // column (if present). Otherwise signs are to be displayed in the sign
    // column.
    if (*wp->w_p_scl == 'n' && *(wp->w_p_scl + 1) == 'u')
	return get_first_valid_sign(wp) != NULL && !wp->w_p_nu && !wp->w_p_rnu;

    if (*wp->w_p_scl == 'n')
	return FALSE;
    if (*wp->w_p_scl == 'y')
	return TRUE;
    return (get_first_valid_sign(wp) != NULL
# ifdef FEAT_NETBEANS_INTG
			|| wp->w_buffer->b_has_sign_column
# endif
		    );
}

/*
 * "sign_unplace()" function
 */
    void
f_sign_unplace(typval_T *argvars, typval_T *rettv)
{
    dict_T	*dict = NULL;

    rettv->vval.v_number = -1;

    if ((check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 1) == FAIL))
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
	dict = argvars[1].vval.v_dict;

    rettv->vval.v_number = sign_unplace_from_dict(&argvars[0], dict);
}

/*
 * "sign_unplacelist()" function
 */
    void
f_sign_unplacelist(typval_T *argvars, typval_T *rettv)
{
    listitem_T	*li;
    int		retval;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_list_arg(argvars, 0) == FAIL)
	return;

    if (check_for_list_arg(argvars, 0) == FAIL)
	return;

    FOR_ALL_LIST_ITEMS(argvars[0].vval.v_list, li)
    {
	retval = -1;
	if (li->li_tv.v_type == VAR_DICT)
	    retval = sign_unplace_from_dict(NULL, li->li_tv.vval.v_dict);
	else
	    emsg(_(e_dictionary_required));
	list_append_number(rettv->vval.v_list, retval);
    }
}

#endif // FEAT_SIGNS
