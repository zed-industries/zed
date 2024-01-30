/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * Text properties implementation.  See ":help text-properties".
 */

#include "vim.h"

#if defined(FEAT_PROP_POPUP) || defined(PROTO)

/*
 * In a hashtable item "hi_key" points to "pt_name" in a proptype_T.
 * This avoids adding a pointer to the hashtable item.
 * PT2HIKEY() converts a proptype pointer to a hashitem key pointer.
 * HIKEY2PT() converts a hashitem key pointer to a proptype pointer.
 * HI2PT() converts a hashitem pointer to a proptype pointer.
 */
#define PT2HIKEY(p)  ((p)->pt_name)
#define HIKEY2PT(p)   ((proptype_T *)((p) - offsetof(proptype_T, pt_name)))
#define HI2PT(hi)      HIKEY2PT((hi)->hi_key)

// The global text property types.
static hashtab_T *global_proptypes = NULL;
static proptype_T **global_proparray = NULL;

// The last used text property type ID.
static int proptype_id = 0;

/*
 * Find a property type by name, return the hashitem.
 * Returns NULL if the item can't be found.
 */
    static hashitem_T *
find_prop_type_hi(char_u *name, buf_T *buf)
{
    hashtab_T	*ht;
    hashitem_T	*hi;

    if (*name == NUL)
	return NULL;
    if (buf == NULL)
	ht = global_proptypes;
    else
	ht = buf->b_proptypes;

    if (ht == NULL)
	return NULL;
    hi = hash_find(ht, name);
    if (HASHITEM_EMPTY(hi))
	return NULL;
    return hi;
}

/*
 * Like find_prop_type_hi() but return the property type.
 */
    static proptype_T *
find_prop_type(char_u *name, buf_T *buf)
{
    hashitem_T	*hi = find_prop_type_hi(name, buf);

    if (hi == NULL)
	return NULL;
    return HI2PT(hi);
}

/*
 * Get the prop type ID of "name".
 * When not found return zero.
 */
    int
find_prop_type_id(char_u *name, buf_T *buf)
{
    proptype_T *pt = find_prop_type(name, buf);

    if (pt == NULL)
	return 0;
    return pt->pt_id;
}

/*
 * Lookup a property type by name.  First in "buf" and when not found in the
 * global types.
 * When not found gives an error message and returns NULL.
 */
    static proptype_T *
lookup_prop_type(char_u *name, buf_T *buf)
{
    proptype_T *type = find_prop_type(name, buf);

    if (type == NULL)
	type = find_prop_type(name, NULL);
    if (type == NULL)
	semsg(_(e_property_type_str_does_not_exist), name);
    return type;
}

/*
 * Get an optional "bufnr" item from the dict in "arg".
 * When the argument is not used or "bufnr" is not present then "buf" is
 * unchanged.
 * If "bufnr" is valid or not present return OK.
 * When "arg" is not a dict or "bufnr" is invalid return FAIL.
 */
    static int
get_bufnr_from_arg(typval_T *arg, buf_T **buf)
{
    dictitem_T	*di;

    if (arg->v_type != VAR_DICT)
    {
	emsg(_(e_dictionary_required));
	return FAIL;
    }
    if (arg->vval.v_dict == NULL)
	return OK;  // NULL dict is like an empty dict
    di = dict_find(arg->vval.v_dict, (char_u *)"bufnr", -1);
    if (di != NULL && (di->di_tv.v_type != VAR_NUMBER
					      || di->di_tv.vval.v_number != 0))
    {
	*buf = get_buf_arg(&di->di_tv);
	if (*buf == NULL)
	    return FAIL;
    }
    return OK;
}

/*
 * prop_add({lnum}, {col}, {props})
 */
    void
f_prop_add(typval_T *argvars, typval_T *rettv)
{
    linenr_T	start_lnum;
    colnr_T	start_col;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL
		|| check_for_dict_arg(argvars, 2) == FAIL))
	return;

    start_lnum = tv_get_number(&argvars[0]);
    start_col = tv_get_number(&argvars[1]);
    if (check_for_dict_arg(argvars, 2) == FAIL)
	return;

    rettv->vval.v_number = prop_add_common(start_lnum, start_col,
				 argvars[2].vval.v_dict, curbuf, &argvars[2]);
}

/*
 * Attach a text property 'type_name' to the text starting
 * at [start_lnum, start_col] and ending at [end_lnum, end_col] in
 * the buffer "buf" and assign identifier "id".
 * When "text" is not NULL add it to buf->b_textprop_text[-id - 1].
 */
    static int
prop_add_one(
	buf_T		*buf,
	char_u		*type_name,
	int		id,
	char_u		*text_arg,
	int		text_padding_left,
	int		text_flags,
	linenr_T	start_lnum,
	linenr_T	end_lnum,
	colnr_T		start_col,
	colnr_T		end_col)
{
    proptype_T	*type;
    linenr_T	lnum;
    int		proplen;
    char_u	*props = NULL;
    char_u	*newprops;
    size_t	textlen;
    char_u	*newtext;
    int		i;
    textprop_T	tmp_prop;
    char_u	*text = text_arg;
    int		res = FAIL;

    type = lookup_prop_type(type_name, buf);
    if (type == NULL)
	goto theend;

    if (start_lnum < 1 || start_lnum > buf->b_ml.ml_line_count)
    {
	semsg(_(e_invalid_line_number_nr), (long)start_lnum);
	goto theend;
    }
    if (end_lnum < start_lnum || end_lnum > buf->b_ml.ml_line_count)
    {
	semsg(_(e_invalid_line_number_nr), (long)end_lnum);
	goto theend;
    }

    if (buf->b_ml.ml_mfp == NULL)
    {
	emsg(_(e_cannot_add_text_property_to_unloaded_buffer));
	goto theend;
    }

    if (text != NULL)
    {
	garray_T    *gap = &buf->b_textprop_text;
	char_u	    *p;

	// double check we got the right ID
	if (-id - 1 != gap->ga_len)
	    iemsg("text prop ID mismatch");
	if (gap->ga_growsize == 0)
	    ga_init2(gap, sizeof(char *), 50);
	if (ga_grow(gap, 1) == FAIL)
	    goto theend;
	((char_u **)gap->ga_data)[gap->ga_len++] = text;

	// change any control character (Tab, Newline, etc.) to a Space to make
	// it simpler to compute the size
	for (p = text; *p != NUL; MB_PTR_ADV(p))
	    if (*p < ' ')
		*p = ' ';
	text = NULL;
    }

    for (lnum = start_lnum; lnum <= end_lnum; ++lnum)
    {
	colnr_T col;	    // start column use in tp_col
	colnr_T sort_col;   // column where it appears
	long	length;	    // in bytes

	// Fetch the line to get the ml_line_len field updated.
	proplen = get_text_props(buf, lnum, &props, TRUE);
	textlen = buf->b_ml.ml_line_len - proplen * sizeof(textprop_T);

	if (lnum == start_lnum)
	    col = start_col;
	else
	    col = 1;
	if (col - 1 > (colnr_T)textlen && !(col == 0 && text_arg != NULL))
	{
	    semsg(_(e_invalid_column_number_nr), (long)start_col);
	    goto theend;
	}
	sort_col = col;

	if (lnum == end_lnum)
	    length = end_col - col;
	else
	    length = (int)textlen - col + 1;
	if (length > (long)textlen)
	    length = (int)textlen;	// can include the end-of-line
	if (length < 0)
	    length = 0;		// zero-width property

	if (text_arg != NULL)
	{
	    length = 1;		// text is placed on one character
	    if (col == 0)
	    {
		col = MAXCOL;	// before or after the line
		if ((text_flags & TP_FLAG_ALIGN_ABOVE) == 0)
		    sort_col = MAXCOL;
		length += text_padding_left;
	    }
	}

	// Allocate the new line with space for the new property.
	newtext = alloc(buf->b_ml.ml_line_len + sizeof(textprop_T));
	if (newtext == NULL)
	    goto theend;
	// Copy the text, including terminating NUL.
	mch_memmove(newtext, buf->b_ml.ml_line_ptr, textlen);

	// Find the index where to insert the new property.
	// Since the text properties are not aligned properly when stored with
	// the text, we need to copy them as bytes before using it as a struct.
	for (i = 0; i < proplen; ++i)
	{
	    colnr_T prop_col;

	    mch_memmove(&tmp_prop, props + i * sizeof(textprop_T),
							   sizeof(textprop_T));
	    // col is MAXCOL when the text goes above or after the line, when
	    // above we should use column zero for sorting
	    prop_col = (tmp_prop.tp_flags & TP_FLAG_ALIGN_ABOVE)
				? 0 : tmp_prop.tp_col;
	    if (prop_col >= sort_col)
		break;
	}
	newprops = newtext + textlen;
	if (i > 0)
	    mch_memmove(newprops, props, sizeof(textprop_T) * i);

	tmp_prop.tp_col = col;
	tmp_prop.tp_len = length;
	tmp_prop.tp_id = id;
	tmp_prop.tp_type = type->pt_id;
	tmp_prop.tp_flags = text_flags
			    | (lnum > start_lnum ? TP_FLAG_CONT_PREV : 0)
			    | (lnum < end_lnum ? TP_FLAG_CONT_NEXT : 0)
			    | ((type->pt_flags & PT_FLAG_INS_START_INCL)
						     ? TP_FLAG_START_INCL : 0);
	tmp_prop.tp_padleft = text_padding_left;
	mch_memmove(newprops + i * sizeof(textprop_T), &tmp_prop,
							   sizeof(textprop_T));

	if (i < proplen)
	    mch_memmove(newprops + (i + 1) * sizeof(textprop_T),
					    props + i * sizeof(textprop_T),
					    sizeof(textprop_T) * (proplen - i));

	if (buf->b_ml.ml_flags & (ML_LINE_DIRTY | ML_ALLOCATED))
	    vim_free(buf->b_ml.ml_line_ptr);
	buf->b_ml.ml_line_ptr = newtext;
	buf->b_ml.ml_line_len += sizeof(textprop_T);
	buf->b_ml.ml_flags |= ML_LINE_DIRTY;
    }

    changed_line_display_buf(buf);
    changed_lines_buf(buf, start_lnum, end_lnum + 1, 0);
    res = OK;

theend:
    vim_free(text);
    return res;
}

/*
 * prop_add_list()
 * First argument specifies the text property:
 *   {'type': <str>, 'id': <num>, 'bufnr': <num>}
 * Second argument is a List where each item is a List with the following
 * entries: [lnum, start_col, end_col]
 */
    void
f_prop_add_list(typval_T *argvars, typval_T *rettv UNUSED)
{
    dict_T	*dict;
    char_u	*type_name;
    buf_T	*buf = curbuf;
    int		id = 0;
    listitem_T	*li;
    list_T	*pos_list;
    linenr_T	start_lnum;
    colnr_T	start_col;
    linenr_T	end_lnum;
    colnr_T	end_col;
    int		error = FALSE;
    int		prev_did_emsg = did_emsg;

    if (check_for_dict_arg(argvars, 0) == FAIL
	    || check_for_list_arg(argvars, 1) == FAIL)
	return;

    if (check_for_nonnull_list_arg(argvars, 1) == FAIL)
	return;

    dict = argvars[0].vval.v_dict;
    if (dict == NULL || !dict_has_key(dict, "type"))
    {
	emsg(_(e_missing_property_type_name));
	return;
    }
    type_name = dict_get_string(dict, "type", FALSE);

    if (dict_has_key(dict, "id"))
	id = dict_get_number(dict, "id");

    if (get_bufnr_from_arg(&argvars[0], &buf) == FAIL)
	return;

    // This must be done _before_ we start adding properties because property
    // changes trigger buffer (memline) reorganisation, which needs this flag
    // to be correctly set.
    buf->b_has_textprop = TRUE;  // this is never reset
    FOR_ALL_LIST_ITEMS(argvars[1].vval.v_list, li)
    {
	if (li->li_tv.v_type != VAR_LIST || li->li_tv.vval.v_list == NULL)
	{
	    emsg(_(e_list_required));
	    return;
	}

	pos_list = li->li_tv.vval.v_list;
	start_lnum = list_find_nr(pos_list, 0L, &error);
	if (!error)
	    start_col = list_find_nr(pos_list, 1L, &error);
	if (!error)
	    end_lnum = list_find_nr(pos_list, 2L, &error);
	if (!error)
	    end_col = list_find_nr(pos_list, 3L, &error);
	int this_id = id;
	if (!error && pos_list->lv_len > 4)
	    this_id = list_find_nr(pos_list, 4L, &error);
	if (error || start_lnum <= 0 || start_col <= 0
		  || end_lnum <= 0 || end_col <= 0)
	{
	    if (prev_did_emsg == did_emsg)
		emsg(_(e_invalid_argument));
	    return;
	}
	if (prop_add_one(buf, type_name, this_id, NULL, 0, 0,
			     start_lnum, end_lnum, start_col, end_col) == FAIL)
	    return;
    }

    redraw_buf_later(buf, UPD_VALID);
}

/*
 * Get the next ID to use for a textprop with text in buffer "buf".
 */
    static int
get_textprop_id(buf_T *buf)
{
    // TODO: recycle deleted entries
    return -(buf->b_textprop_text.ga_len + 1);
}

// Flag that is set when a negative ID isused for a normal text property.
// It is then impossible to use virtual text properties.
static int did_use_negative_pop_id = FALSE;

/*
 * Shared between prop_add() and popup_create().
 * "dict_arg" is the function argument of a dict containing "bufnr".
 * it is NULL for popup_create().
 * Returns the "id" used for "text" or zero.
 */
    int
prop_add_common(
	linenr_T    start_lnum,
	colnr_T	    start_col,
	dict_T	    *dict,
	buf_T	    *default_buf,
	typval_T    *dict_arg)
{
    linenr_T	end_lnum;
    colnr_T	end_col;
    char_u	*type_name;
    buf_T	*buf = default_buf;
    int		id = 0;
    char_u	*text = NULL;
    int		text_padding_left = 0;
    int		flags = 0;

    if (dict == NULL || !dict_has_key(dict, "type"))
    {
	emsg(_(e_missing_property_type_name));
	goto theend;
    }
    type_name = dict_get_string(dict, "type", FALSE);

    if (dict_has_key(dict, "end_lnum"))
    {
	end_lnum = dict_get_number(dict, "end_lnum");
	if (end_lnum < start_lnum)
	{
	    semsg(_(e_invalid_value_for_argument_str), "end_lnum");
	    goto theend;
	}
    }
    else
	end_lnum = start_lnum;

    if (dict_has_key(dict, "length"))
    {
	long length = dict_get_number(dict, "length");

	if (length < 0 || end_lnum > start_lnum)
	{
	    semsg(_(e_invalid_value_for_argument_str), "length");
	    goto theend;
	}
	end_col = start_col + length;
    }
    else if (dict_has_key(dict, "end_col"))
    {
	end_col = dict_get_number(dict, "end_col");
	if (end_col <= 0)
	{
	    semsg(_(e_invalid_value_for_argument_str), "end_col");
	    goto theend;
	}
    }
    else if (start_lnum == end_lnum)
	end_col = start_col;
    else
	end_col = 1;

    if (dict_has_key(dict, "id"))
	id = dict_get_number(dict, "id");

    if (dict_has_key(dict, "text"))
    {
	if (dict_has_key(dict, "length")
		|| dict_has_key(dict, "end_col")
		|| dict_has_key(dict, "end_lnum"))
	{
	    emsg(_(e_cannot_use_length_endcol_and_endlnum_with_text));
	    goto theend;
	}

	text = dict_get_string(dict, "text", TRUE);
	if (text == NULL)
	    goto theend;
	// use a default length of 1 to make multiple props show up
	end_col = start_col + 1;

	if (dict_has_key(dict, "text_align"))
	{
	    char_u *p = dict_get_string(dict, "text_align", FALSE);

	    if (p == NULL)
		goto theend;
	    if (start_col != 0)
	    {
		emsg(_(e_can_only_use_text_align_when_column_is_zero));
		goto theend;
	    }
	    if (STRCMP(p, "right") == 0)
		flags |= TP_FLAG_ALIGN_RIGHT;
	    else if (STRCMP(p, "above") == 0)
		flags |= TP_FLAG_ALIGN_ABOVE;
	    else if (STRCMP(p, "below") == 0)
		flags |= TP_FLAG_ALIGN_BELOW;
	    else if (STRCMP(p, "after") != 0)
	    {
		semsg(_(e_invalid_value_for_argument_str_str), "text_align", p);
		goto theend;
	    }
	}

	if (dict_has_key(dict, "text_padding_left"))
	{
	    text_padding_left = dict_get_number(dict, "text_padding_left");
	    if (text_padding_left < 0)
	    {
		semsg(_(e_argument_must_be_positive_str), "text_padding_left");
		goto theend;
	    }
	}

	if (dict_has_key(dict, "text_wrap"))
	{
	    char_u *p = dict_get_string(dict, "text_wrap", FALSE);

	    if (p == NULL)
		goto theend;
	    if (STRCMP(p, "wrap") == 0)
		flags |= TP_FLAG_WRAP;
	    else if (STRCMP(p, "truncate") != 0)
	    {
		semsg(_(e_invalid_value_for_argument_str_str), "text_wrap", p);
		goto theend;
	    }
	}
    }

    // Column must be 1 or more for a normal text property; when "text" is
    // present zero means it goes after the line.
    if (start_col < (text == NULL ? 1 : 0))
    {
	semsg(_(e_invalid_column_number_nr), (long)start_col);
	goto theend;
    }
    if (start_col > 0 && text_padding_left > 0)
    {
	emsg(_(e_can_only_use_left_padding_when_column_is_zero));
	goto theend;
    }

    if (dict_arg != NULL && get_bufnr_from_arg(dict_arg, &buf) == FAIL)
	goto theend;

    if (id < 0)
    {
	if (buf->b_textprop_text.ga_len > 0)
	{
	    emsg(_(e_cannot_use_negative_id_after_adding_textprop_with_text));
	    goto theend;
	}
	did_use_negative_pop_id = TRUE;
    }

    if (text != NULL)
    {
	if (did_use_negative_pop_id)
	{
	    emsg(_(e_cannot_add_textprop_with_text_after_using_textprop_with_negative_id));
	    goto theend;
	}
	id = get_textprop_id(buf);
    }

    // This must be done _before_ we add the property because property changes
    // trigger buffer (memline) reorganisation, which needs this flag to be
    // correctly set.
    buf->b_has_textprop = TRUE;  // this is never reset

    prop_add_one(buf, type_name, id, text, text_padding_left, flags,
				    start_lnum, end_lnum, start_col, end_col);
    text = NULL;

    redraw_buf_later(buf, UPD_VALID);

theend:
    vim_free(text);
    return id;
}

/*
 * Fetch the text properties for line "lnum" in buffer "buf".
 * Returns the number of text properties and, when non-zero, a pointer to the
 * first one in "props" (note that it is not aligned, therefore the char_u
 * pointer).
 */
    int
get_text_props(buf_T *buf, linenr_T lnum, char_u **props, int will_change)
{
    char_u *text;
    size_t textlen;
    size_t proplen;

    // Be quick when no text property types have been defined for the buffer,
    // unless we are adding one.
    if ((!buf->b_has_textprop && !will_change) || buf->b_ml.ml_mfp == NULL)
	return 0;

    // Fetch the line to get the ml_line_len field updated.
    text = ml_get_buf(buf, lnum, will_change);
    textlen = STRLEN(text) + 1;
    proplen = buf->b_ml.ml_line_len - textlen;
    if (proplen == 0)
	return 0;
    if (proplen % sizeof(textprop_T) != 0)
    {
	iemsg(e_text_property_info_corrupted);
	return 0;
    }
    *props = text + textlen;
    return (int)(proplen / sizeof(textprop_T));
}

/*
 * Return the number of text properties with "above" or "below" alignment in
 * line "lnum".  A "right" aligned property also goes below after a "below" or
 * other "right" aligned property.
 */
    int
prop_count_above_below(buf_T *buf, linenr_T lnum)
{
    char_u	*props;
    int		count = get_text_props(buf, lnum, &props, FALSE);
    int		result = 0;
    textprop_T	prop;
    int		i;
    int		next_right_goes_below = FALSE;

    if (count == 0)
	return 0;
    for (i = 0; i < count; ++i)
    {
	mch_memmove(&prop, props + i * sizeof(prop), sizeof(prop));
	if (prop.tp_col == MAXCOL && text_prop_type_valid(buf, &prop))
	{
	    if ((prop.tp_flags & TP_FLAG_ALIGN_BELOW)
		    || (next_right_goes_below
				     && (prop.tp_flags & TP_FLAG_ALIGN_RIGHT)))
	    {
		next_right_goes_below = TRUE;
		++result;
	    }
	    else if (prop.tp_flags & TP_FLAG_ALIGN_ABOVE)
	    {
		next_right_goes_below = FALSE;
		++result;
	    }
	    else if (prop.tp_flags & TP_FLAG_ALIGN_RIGHT)
		next_right_goes_below = TRUE;
	}
    }
    return result;
}

/**
 * Return the number of text properties on line "lnum" in the current buffer.
 * When "only_starting" is true only text properties starting in this line will
 * be considered.
 * When "last_line" is FALSE then text properties after the line are not
 * counted.
 */
    int
count_props(linenr_T lnum, int only_starting, int last_line)
{
    char_u	*props;
    int		proplen = get_text_props(curbuf, lnum, &props, 0);
    int		result = proplen;
    int		i;
    textprop_T	prop;

    for (i = 0; i < proplen; ++i)
    {
	mch_memmove(&prop, props + i * sizeof(prop), sizeof(prop));
	// A prop is dropped when in the first line and it continues from the
	// previous line, or when not in the last line and it is virtual text
	// after the line.
	if ((only_starting && (prop.tp_flags & TP_FLAG_CONT_PREV))
		|| (!last_line && prop.tp_col == MAXCOL))
	    --result;
    }
    return result;
}

static textprop_T	*text_prop_compare_props;
static buf_T		*text_prop_compare_buf;

/*
 * Score for sorting on position of the text property: 0: above,
 * 1: after (default), 2: right, 3: below (comes last)
 */
    static int
text_prop_order(int flags)
{
    if (flags & TP_FLAG_ALIGN_ABOVE)
	return 0;
    if (flags & TP_FLAG_ALIGN_RIGHT)
	return 2;
    if (flags & TP_FLAG_ALIGN_BELOW)
	return 3;
    return 1;
}

/*
 * Function passed to qsort() to sort text properties.
 * Return 1 if "s1" has priority over "s2", -1 if the other way around, zero if
 * both have the same priority.
 */
    static int
text_prop_compare(const void *s1, const void *s2)
{
    int  idx1, idx2;
    textprop_T	*tp1, *tp2;
    proptype_T  *pt1, *pt2;
    colnr_T col1, col2;

    idx1 = *(int *)s1;
    idx2 = *(int *)s2;
    tp1 = &text_prop_compare_props[idx1];
    tp2 = &text_prop_compare_props[idx2];
    col1 = tp1->tp_col;
    col2 = tp2->tp_col;
    if (col1 == MAXCOL || col2 == MAXCOL)
    {
	int order1 = text_prop_order(tp1->tp_flags);
	int order2 = text_prop_order(tp2->tp_flags);

	// sort on order where it is added
	if (order1 != order2)
	    return order1 < order2 ? 1 : -1;
    }

    // property that inserts text has priority over one that doesn't
    if ((tp1->tp_id < 0) != (tp2->tp_id < 0))
	return tp1->tp_id < 0 ? 1 : -1;

    // check highest priority, defined by the type
    pt1 = text_prop_type_by_id(text_prop_compare_buf, tp1->tp_type);
    pt2 = text_prop_type_by_id(text_prop_compare_buf, tp2->tp_type);
    if (pt1 != pt2)
    {
	if (pt1 == NULL)
	    return -1;
	if (pt2 == NULL)
	    return 1;
	if (pt1->pt_priority != pt2->pt_priority)
	    return pt1->pt_priority > pt2->pt_priority ? 1 : -1;
    }

    // same priority, one that starts first wins
    if (col1 != col2)
	return col1 < col2 ? 1 : -1;

    // for a property with text the id can be used as tie breaker
    if (tp1->tp_id < 0)
	return tp1->tp_id > tp2->tp_id ? 1 : -1;

    return 0;
}

/*
 * Sort "count" text properties using an array if indexes "idxs" into the list
 * of text props "props" for buffer "buf".
 */
    void
sort_text_props(
	buf_T	    *buf,
	textprop_T  *props,
	int	    *idxs,
	int	    count)
{
    text_prop_compare_buf = buf;
    text_prop_compare_props = props;
    qsort((void *)idxs, (size_t)count, sizeof(int), text_prop_compare);
}

/*
 * Find text property "type_id" in the visible lines of window "wp".
 * Match "id" when it is > 0.
 * Returns FAIL when not found.
 */
    int
find_visible_prop(
	win_T	    *wp,
	int	    type_id,
	int	    id,
	textprop_T  *prop,
	linenr_T    *found_lnum)
{
    // return when "type_id" no longer exists
    if (text_prop_type_by_id(wp->w_buffer, type_id) == NULL)
	return FAIL;

    // w_botline may not have been updated yet.
    validate_botline_win(wp);
    for (linenr_T lnum = wp->w_topline; lnum < wp->w_botline; ++lnum)
    {
	char_u	*props;
	int	count = get_text_props(wp->w_buffer, lnum, &props, FALSE);
	for (int i = 0; i < count; ++i)
	{
	    mch_memmove(prop, props + i * sizeof(textprop_T),
							   sizeof(textprop_T));
	    if (prop->tp_type == type_id && (id <= 0 || prop->tp_id == id))
	    {
		*found_lnum = lnum;
		return OK;
	    }
	}
    }
    return FAIL;
}

/*
 * Set the text properties for line "lnum" to "props" with length "len".
 * If "len" is zero text properties are removed, "props" is not used.
 * Any existing text properties are dropped.
 * Only works for the current buffer.
 */
    static void
set_text_props(linenr_T lnum, char_u *props, int len)
{
    char_u  *text;
    char_u  *newtext;
    int	    textlen;

    text = ml_get(lnum);
    textlen = (int)STRLEN(text) + 1;
    newtext = alloc(textlen + len);
    if (newtext == NULL)
	return;
    mch_memmove(newtext, text, textlen);
    if (len > 0)
	mch_memmove(newtext + textlen, props, len);
    if (curbuf->b_ml.ml_flags & (ML_LINE_DIRTY | ML_ALLOCATED))
	vim_free(curbuf->b_ml.ml_line_ptr);
    curbuf->b_ml.ml_line_ptr = newtext;
    curbuf->b_ml.ml_line_len = textlen + len;
    curbuf->b_ml.ml_flags |= ML_LINE_DIRTY;
}

/*
 * Add "text_props" with "text_prop_count" text properties to line "lnum".
 */
    void
add_text_props(linenr_T lnum, textprop_T *text_props, int text_prop_count)
{
    char_u  *text;
    char_u  *newtext;
    int	    proplen = text_prop_count * (int)sizeof(textprop_T);

    text = ml_get(lnum);
    newtext = alloc(curbuf->b_ml.ml_line_len + proplen);
    if (newtext == NULL)
	return;
    mch_memmove(newtext, text, curbuf->b_ml.ml_line_len);
    mch_memmove(newtext + curbuf->b_ml.ml_line_len, text_props, proplen);
    if (curbuf->b_ml.ml_flags & (ML_LINE_DIRTY | ML_ALLOCATED))
	vim_free(curbuf->b_ml.ml_line_ptr);
    curbuf->b_ml.ml_line_ptr = newtext;
    curbuf->b_ml.ml_line_len += proplen;
    curbuf->b_ml.ml_flags |= ML_LINE_DIRTY;
}

/*
 * Function passed to qsort() for sorting proptype_T on pt_id.
 */
    static int
compare_pt(const void *s1, const void *s2)
{
    proptype_T	*tp1 = *(proptype_T **)s1;
    proptype_T	*tp2 = *(proptype_T **)s2;

    return tp1->pt_id == tp2->pt_id ? 0 : tp1->pt_id < tp2->pt_id ? -1 : 1;
}

    static proptype_T *
find_type_by_id(hashtab_T *ht, proptype_T ***array, int id)
{
    int low = 0;
    int high;

    if (ht == NULL || ht->ht_used == 0)
	return NULL;

    // Make the lookup faster by creating an array with pointers to
    // hashtable entries, sorted on pt_id.
    if (*array == NULL)
    {
	long	    todo;
	hashitem_T  *hi;
	int	    i = 0;

	*array = ALLOC_MULT(proptype_T *, ht->ht_used);
	if (*array == NULL)
	    return NULL;
	todo = (long)ht->ht_used;
	FOR_ALL_HASHTAB_ITEMS(ht, hi, todo)
	{
	    if (!HASHITEM_EMPTY(hi))
	    {
		(*array)[i++] = HI2PT(hi);
		--todo;
	    }
	}
	qsort((void *)*array, ht->ht_used, sizeof(proptype_T *), compare_pt);
    }

    // binary search in the sorted array
    high = ht->ht_used;
    while (high > low)
    {
	int m = (high + low) / 2;

	if ((*array)[m]->pt_id == id)
	    return (*array)[m];
	if ((*array)[m]->pt_id > id)
	    high = m;
	else
	    low = m + 1;
    }
    return NULL;
}

/*
 * Fill 'dict' with text properties in 'prop'.
 */
    static void
prop_fill_dict(dict_T *dict, textprop_T *prop, buf_T *buf)
{
    proptype_T *pt;
    int buflocal = TRUE;
    int virtualtext_prop = prop->tp_id < 0;

    dict_add_number(dict, "col", (prop->tp_col == MAXCOL) ? 0 : prop->tp_col);
    if (!virtualtext_prop)
    {
	dict_add_number(dict, "length", prop->tp_len);
	dict_add_number(dict, "id", prop->tp_id);
    }
    dict_add_number(dict, "start", !(prop->tp_flags & TP_FLAG_CONT_PREV));
    dict_add_number(dict, "end", !(prop->tp_flags & TP_FLAG_CONT_NEXT));

    pt = find_type_by_id(buf->b_proptypes, &buf->b_proparray, prop->tp_type);
    if (pt == NULL)
    {
	pt = find_type_by_id(global_proptypes, &global_proparray,
								prop->tp_type);
	buflocal = FALSE;
    }
    if (pt != NULL)
	dict_add_string(dict, "type", pt->pt_name);

    if (buflocal)
	dict_add_number(dict, "type_bufnr", buf->b_fnum);
    else
	dict_add_number(dict, "type_bufnr", 0);
    if (virtualtext_prop)
    {
	// virtual text property
	garray_T    *gap = &buf->b_textprop_text;
	char_u	    *text;

	// negate the property id to get the string index
	text = ((char_u **)gap->ga_data)[-prop->tp_id - 1];
	dict_add_string(dict, "text", text);

	// text_align
	char_u	    *text_align = NULL;
	if (prop->tp_flags & TP_FLAG_ALIGN_RIGHT)
	    text_align = (char_u *)"right";
	else if (prop->tp_flags & TP_FLAG_ALIGN_ABOVE)
	    text_align = (char_u *)"above";
	else if (prop->tp_flags & TP_FLAG_ALIGN_BELOW)
	    text_align = (char_u *)"below";
	if (text_align != NULL)
	    dict_add_string(dict, "text_align", text_align);

	// text_wrap
	if (prop->tp_flags & TP_FLAG_WRAP)
	    dict_add_string(dict, "text_wrap", (char_u *)"wrap");
	if (prop->tp_padleft != 0)
	    dict_add_number(dict, "text_padding_left", prop->tp_padleft);
    }
}

/*
 * Find a property type by ID in "buf" or globally.
 * Returns NULL if not found.
 */
    proptype_T *
text_prop_type_by_id(buf_T *buf, int id)
{
    proptype_T *type;

    type = find_type_by_id(buf->b_proptypes, &buf->b_proparray, id);
    if (type == NULL)
	type = find_type_by_id(global_proptypes, &global_proparray, id);
    return type;
}

/*
 * Return TRUE if "prop" is a valid text property type.
 */
    int
text_prop_type_valid(buf_T *buf, textprop_T *prop)
{
    return text_prop_type_by_id(buf, prop->tp_type) != NULL;
}

/*
 * prop_clear({lnum} [, {lnum_end} [, {bufnr}]])
 */
    void
f_prop_clear(typval_T *argvars, typval_T *rettv UNUSED)
{
    linenr_T start;
    linenr_T end;
    linenr_T lnum;
    buf_T    *buf = curbuf;
    int	    did_clear = FALSE;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_dict_arg(argvars, 2) == FAIL)))
	return;

    start = tv_get_number(&argvars[0]);
    end = start;
    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	end = tv_get_number(&argvars[1]);
	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    if (get_bufnr_from_arg(&argvars[2], &buf) == FAIL)
		return;
	}
    }
    if (start < 1 || end < 1)
    {
	emsg(_(e_invalid_range));
	return;
    }

    for (lnum = start; lnum <= end; ++lnum)
    {
	char_u *text;
	size_t len;

	if (lnum > buf->b_ml.ml_line_count)
	    break;
	text = ml_get_buf(buf, lnum, FALSE);
	len = STRLEN(text) + 1;
	if ((size_t)buf->b_ml.ml_line_len > len)
	{
	    did_clear = TRUE;
	    if (!(buf->b_ml.ml_flags & ML_LINE_DIRTY))
	    {
		char_u *newtext = vim_strsave(text);

		// need to allocate the line now
		if (newtext == NULL)
		    return;
		if (buf->b_ml.ml_flags & ML_ALLOCATED)
		    vim_free(buf->b_ml.ml_line_ptr);
		buf->b_ml.ml_line_ptr = newtext;
		buf->b_ml.ml_flags |= ML_LINE_DIRTY;
	    }
	    buf->b_ml.ml_line_len = (int)len;
	}
    }
    if (did_clear)
	redraw_buf_later(buf, UPD_NOT_VALID);
}

/*
 * prop_find({props} [, {direction}])
 */
    void
f_prop_find(typval_T *argvars, typval_T *rettv)
{
    pos_T       *cursor = &curwin->w_cursor;
    dict_T      *dict;
    buf_T       *buf = curbuf;
    dictitem_T  *di;
    int		lnum_start;
    int		start_pos_has_prop = 0;
    int		seen_end = FALSE;
    int		id = 0;
    int		id_found = FALSE;
    int		type_id = -1;
    int		skipstart = FALSE;
    int		lnum = -1;
    int		col = -1;
    int		dir = FORWARD;    // FORWARD == 1, BACKWARD == -1
    int		both;

    if (in_vim9script()
	    && (check_for_dict_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL))
	return;

    if (check_for_nonnull_dict_arg(argvars, 0) == FAIL)
	return;
    dict = argvars[0].vval.v_dict;

    if (get_bufnr_from_arg(&argvars[0], &buf) == FAIL)
	return;
    if (buf->b_ml.ml_mfp == NULL)
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	char_u      *dir_s = tv_get_string(&argvars[1]);

	if (*dir_s == 'b')
	    dir = BACKWARD;
	else if (*dir_s != 'f')
	{
	    emsg(_(e_invalid_argument));
	    return;
	}
    }

    di = dict_find(dict, (char_u *)"lnum", -1);
    if (di != NULL)
	lnum = tv_get_number(&di->di_tv);

    di = dict_find(dict, (char_u *)"col", -1);
    if (di != NULL)
	col = tv_get_number(&di->di_tv);

    if (lnum == -1)
    {
	lnum = cursor->lnum;
	col = cursor->col + 1;
    }
    else if (col == -1)
	col = 1;

    if (lnum < 1 || lnum > buf->b_ml.ml_line_count)
    {
	emsg(_(e_invalid_range));
	return;
    }

    skipstart = dict_get_bool(dict, "skipstart", 0);

    if (dict_has_key(dict, "id"))
    {
	id = dict_get_number(dict, "id");
	id_found = TRUE;
    }
    if (dict_has_key(dict, "type"))
    {
	char_u	    *name = dict_get_string(dict, "type", FALSE);
	proptype_T  *type = lookup_prop_type(name, buf);

	if (type == NULL)
	    return;
	type_id = type->pt_id;
    }
    both = dict_get_bool(dict, "both", FALSE);
    if (!id_found && type_id == -1)
    {
	emsg(_(e_need_at_least_one_of_id_or_type));
	return;
    }
    if (both && (!id_found || type_id == -1))
    {
	emsg(_(e_need_id_and_type_or_types_with_both));
	return;
    }

    lnum_start = lnum;

    if (rettv_dict_alloc(rettv) == FAIL)
	return;

    while (1)
    {
	char_u	*text = ml_get_buf(buf, lnum, FALSE);
	size_t	textlen = STRLEN(text) + 1;
	int	count = (int)((buf->b_ml.ml_line_len - textlen)
							 / sizeof(textprop_T));
	int	    i;
	textprop_T  prop;
	int	    prop_start;
	int	    prop_end;

	for (i = dir == BACKWARD ? count - 1 : 0; i >= 0 && i < count; i += dir)
	{
	    mch_memmove(&prop, text + textlen + i * sizeof(textprop_T),
							   sizeof(textprop_T));

	    // For the very first line try to find the first property before or
	    // after `col`, depending on the search direction.
	    if (lnum == lnum_start)
	    {
		if (dir == BACKWARD)
		{
		    if (prop.tp_col > col)
			continue;
		}
		else if (prop.tp_col + prop.tp_len - (prop.tp_len != 0) < col)
		    continue;
	    }
	    if (both ? prop.tp_id == id && prop.tp_type == type_id
		     : (id_found && prop.tp_id == id)
						    || prop.tp_type == type_id)
	    {
		// Check if the starting position has text props.
		if (lnum_start == lnum
			&& col >= prop.tp_col
			&& (col <= prop.tp_col + prop.tp_len
							 - (prop.tp_len != 0)))
		    start_pos_has_prop = 1;

		// The property was not continued from last line, it starts on
		// this line.
		prop_start = !(prop.tp_flags & TP_FLAG_CONT_PREV);
		// The property does not continue on the next line, it ends on
		// this line.
		prop_end = !(prop.tp_flags & TP_FLAG_CONT_NEXT);
		if (!prop_start && prop_end && dir == FORWARD)
		    seen_end = 1;

		// Skip lines without the start flag.
		if (!prop_start)
		{
		    // Always search backwards for start when search started
		    // on a prop and we're not skipping.
		    if (start_pos_has_prop && !skipstart)
			dir = BACKWARD;
		    continue;
		}

		// If skipstart is true, skip the prop at start pos (even if
		// continued from another line).
		if (start_pos_has_prop && skipstart && !seen_end)
		{
		    start_pos_has_prop = 0;
		    continue;
		}

		prop_fill_dict(rettv->vval.v_dict, &prop, buf);
		dict_add_number(rettv->vval.v_dict, "lnum", lnum);

		return;
	    }
	}

	if (dir > 0)
	{
	    if (lnum >= buf->b_ml.ml_line_count)
		break;
	    lnum++;
	}
	else
	{
	    if (lnum <= 1)
		break;
	    lnum--;
	}
    }
}

/*
 * Returns TRUE if 'type_or_id' is in the 'types_or_ids' list.
 */
    static int
prop_type_or_id_in_list(int *types_or_ids, int len, int type_or_id)
{
    int i;

    for (i = 0; i < len; i++)
	if (types_or_ids[i] == type_or_id)
	    return TRUE;

    return FALSE;
}

/*
 * Return all the text properties in line 'lnum' in buffer 'buf' in 'retlist'.
 * If 'prop_types' is not NULL, then return only the text properties with
 * matching property type in the 'prop_types' array.
 * If 'prop_ids' is not NULL, then return only the text properties with
 * an identifier in the 'props_ids' array.
 * If 'add_lnum' is TRUE, then add the line number also to the text property
 * dictionary.
 */
    static void
get_props_in_line(
	buf_T		*buf,
	linenr_T	lnum,
	int		*prop_types,
	int		prop_types_len,
	int		*prop_ids,
	int		prop_ids_len,
	list_T		*retlist,
	int		add_lnum)
{
    char_u	*text = ml_get_buf(buf, lnum, FALSE);
    size_t	textlen = STRLEN(text) + 1;
    int		count;
    int		i;
    textprop_T	prop;

    count = (int)((buf->b_ml.ml_line_len - textlen) / sizeof(textprop_T));
    for (i = 0; i < count; ++i)
    {
	mch_memmove(&prop, text + textlen + i * sizeof(textprop_T),
		sizeof(textprop_T));
	if ((prop_types == NULL
		    || prop_type_or_id_in_list(prop_types, prop_types_len,
			prop.tp_type))
		&& (prop_ids == NULL
		    || prop_type_or_id_in_list(prop_ids, prop_ids_len,
								 prop.tp_id)))
	{
	    dict_T *d = dict_alloc();

	    if (d == NULL)
		break;
	    prop_fill_dict(d, &prop, buf);
	    if (add_lnum)
		dict_add_number(d, "lnum", lnum);
	    list_append_dict(retlist, d);
	}
    }
}

/*
 * Convert a List of property type names into an array of property type
 * identifiers. Returns a pointer to the allocated array. Returns NULL on
 * error. 'num_types' is set to the number of returned property types.
 */
    static int *
get_prop_types_from_names(list_T *l, buf_T *buf, int *num_types)
{
    int		*prop_types;
    listitem_T	*li;
    int		i;
    char_u	*name;
    proptype_T	*type;

    *num_types = 0;

    prop_types = ALLOC_MULT(int, list_len(l));
    if (prop_types == NULL)
	return NULL;

    i = 0;
    FOR_ALL_LIST_ITEMS(l, li)
    {
	if (li->li_tv.v_type != VAR_STRING)
	{
	    emsg(_(e_string_required));
	    goto errret;
	}
	name = li->li_tv.vval.v_string;
	if (name == NULL)
	    goto errret;

	type = lookup_prop_type(name, buf);
	if (type == NULL)
	    goto errret;
	prop_types[i++] = type->pt_id;
    }

    *num_types = i;
    return prop_types;

errret:
    VIM_CLEAR(prop_types);
    return NULL;
}

/*
 * Convert a List of property identifiers into an array of property
 * identifiers.  Returns a pointer to the allocated array. Returns NULL on
 * error. 'num_ids' is set to the number of returned property identifiers.
 */
    static int *
get_prop_ids_from_list(list_T *l, int *num_ids)
{
    int		*prop_ids;
    listitem_T	*li;
    int		i;
    int		id;
    int		error;

    *num_ids = 0;

    prop_ids = ALLOC_MULT(int, list_len(l));
    if (prop_ids == NULL)
	return NULL;

    i = 0;
    FOR_ALL_LIST_ITEMS(l, li)
    {
	error = FALSE;
	id = tv_get_number_chk(&li->li_tv, &error);
	if (error)
	    goto errret;

	prop_ids[i++] = id;
    }

    *num_ids = i;
    return prop_ids;

errret:
    VIM_CLEAR(prop_ids);
    return NULL;
}

/*
 * prop_list({lnum} [, {bufnr}])
 */
    void
f_prop_list(typval_T *argvars, typval_T *rettv)
{
    linenr_T	lnum;
    linenr_T	start_lnum;
    linenr_T	end_lnum;
    buf_T	*buf = curbuf;
    int		add_lnum = FALSE;
    int		*prop_types = NULL;
    int		prop_types_len = 0;
    int		*prop_ids = NULL;
    int		prop_ids_len = 0;
    list_T	*l;
    dictitem_T	*di;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 1) == FAIL))
	return;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    // default: get text properties on current line
    start_lnum = tv_get_number(&argvars[0]);
    end_lnum = start_lnum;
    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	dict_T *d;

	if (check_for_dict_arg(argvars, 1) == FAIL)
	    return;
	d = argvars[1].vval.v_dict;

	if (get_bufnr_from_arg(&argvars[1], &buf) == FAIL)
	    return;

	if (d != NULL && (di = dict_find(d, (char_u *)"end_lnum", -1)) != NULL)
	{
	    if (di->di_tv.v_type != VAR_NUMBER)
	    {
		emsg(_(e_number_required));
		return;
	    }
	    end_lnum = tv_get_number(&di->di_tv);
	    if (end_lnum < 0)
		// negative end_lnum is used as an offset from the last buffer
		// line
		end_lnum = buf->b_ml.ml_line_count + end_lnum + 1;
	    else if (end_lnum > buf->b_ml.ml_line_count)
		end_lnum = buf->b_ml.ml_line_count;
	    add_lnum = TRUE;
	}
	if (d != NULL && (di = dict_find(d, (char_u *)"types", -1)) != NULL)
	{
	    if (di->di_tv.v_type != VAR_LIST)
	    {
		emsg(_(e_list_required));
		return;
	    }

	    l = di->di_tv.vval.v_list;
	    if (l != NULL && list_len(l) > 0)
	    {
		prop_types = get_prop_types_from_names(l, buf, &prop_types_len);
		if (prop_types == NULL)
		    return;
	    }
	}
	if (d != NULL && (di = dict_find(d, (char_u *)"ids", -1)) != NULL)
	{
	    if (di->di_tv.v_type != VAR_LIST)
	    {
		emsg(_(e_list_required));
		goto errret;
	    }

	    l = di->di_tv.vval.v_list;
	    if (l != NULL && list_len(l) > 0)
	    {
		prop_ids = get_prop_ids_from_list(l, &prop_ids_len);
		if (prop_ids == NULL)
		    goto errret;
	    }
	}
    }
    if (start_lnum < 1 || start_lnum > buf->b_ml.ml_line_count
		|| end_lnum < 1 || end_lnum < start_lnum)
	emsg(_(e_invalid_range));
    else
	for (lnum = start_lnum; lnum <= end_lnum; lnum++)
	    get_props_in_line(buf, lnum, prop_types, prop_types_len,
		    prop_ids, prop_ids_len,
		    rettv->vval.v_list, add_lnum);

errret:
    VIM_CLEAR(prop_types);
    VIM_CLEAR(prop_ids);
}

/*
 * prop_remove({props} [, {lnum} [, {lnum_end}]])
 */
    void
f_prop_remove(typval_T *argvars, typval_T *rettv)
{
    linenr_T	start = 1;
    linenr_T	end = 0;
    linenr_T	lnum;
    linenr_T	first_changed = 0;
    linenr_T	last_changed = 0;
    dict_T	*dict;
    buf_T	*buf = curbuf;
    int		do_all;
    int		id = -MAXCOL;
    int		type_id = -1;	    // for a single "type"
    int		*type_ids = NULL;   // array, for a list of "types", allocated
    int		num_type_ids = 0;   // number of elements in "type_ids"
    int		both;
    int		did_remove_text = FALSE;

    rettv->vval.v_number = 0;

    if (in_vim9script()
	    && (check_for_dict_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 2) == FAIL)))
	return;

    if (check_for_nonnull_dict_arg(argvars, 0) == FAIL)
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	start = tv_get_number(&argvars[1]);
	end = start;
	if (argvars[2].v_type != VAR_UNKNOWN)
	    end = tv_get_number(&argvars[2]);
	if (start < 1 || end < 1)
	{
	    emsg(_(e_invalid_range));
	    return;
	}
    }

    dict = argvars[0].vval.v_dict;
    if (get_bufnr_from_arg(&argvars[0], &buf) == FAIL)
	return;
    if (buf->b_ml.ml_mfp == NULL)
	return;

    do_all = dict_get_bool(dict, "all", FALSE);

    if (dict_has_key(dict, "id"))
	id = dict_get_number(dict, "id");

    // if a specific type was supplied "type": check that (and ignore "types".
    // Otherwise check against the list of "types".
    if (dict_has_key(dict, "type"))
    {
	char_u	    *name = dict_get_string(dict, "type", FALSE);
	proptype_T  *type = lookup_prop_type(name, buf);

	if (type == NULL)
	    return;
	type_id = type->pt_id;
    }
    if (dict_has_key(dict, "types"))
    {
	typval_T types;
	listitem_T *li = NULL;

	dict_get_tv(dict, "types", &types);
	if (types.v_type == VAR_LIST && types.vval.v_list->lv_len > 0)
	{
	    type_ids = alloc( sizeof(int) * types.vval.v_list->lv_len );

	    FOR_ALL_LIST_ITEMS(types.vval.v_list, li)
	    {
		proptype_T *prop_type;

		if (li->li_tv.v_type != VAR_STRING)
		    continue;

		prop_type = lookup_prop_type(li->li_tv.vval.v_string, buf);

		if (!prop_type)
		    goto cleanup_prop_remove;

		type_ids[num_type_ids++] = prop_type->pt_id;
	    }
	}
    }
    both = dict_get_bool(dict, "both", FALSE);

    if (id == -MAXCOL && (type_id == -1 && num_type_ids == 0))
    {
	emsg(_(e_need_at_least_one_of_id_or_type));
	goto cleanup_prop_remove;
    }
    if (both && (id == -MAXCOL || (type_id == -1 && num_type_ids == 0)))
    {
	emsg(_(e_need_id_and_type_or_types_with_both));
	goto cleanup_prop_remove;
    }
    if (type_id != -1 && num_type_ids > 0)
    {
	emsg(_(e_cannot_specify_both_type_and_types));
	goto cleanup_prop_remove;
    }

    if (end == 0)
	end = buf->b_ml.ml_line_count;
    for (lnum = start; lnum <= end; ++lnum)
    {
	char_u *text;
	size_t len;

	if (lnum > buf->b_ml.ml_line_count)
	    break;
	text = ml_get_buf(buf, lnum, FALSE);
	len = STRLEN(text) + 1;
	if ((size_t)buf->b_ml.ml_line_len > len)
	{
	    static textprop_T	textprop;  // static because of alignment
	    unsigned		idx;

	    for (idx = 0; idx < (buf->b_ml.ml_line_len - len)
						   / sizeof(textprop_T); ++idx)
	    {
		char_u *cur_prop = buf->b_ml.ml_line_ptr + len
						    + idx * sizeof(textprop_T);
		size_t	taillen;
		int matches_id = 0;
		int matches_type = 0;

		mch_memmove(&textprop, cur_prop, sizeof(textprop_T));

		matches_id = textprop.tp_id == id;
		if (num_type_ids > 0)
		{
		    int idx2;

		    for (idx2 = 0; !matches_type && idx2 < num_type_ids; ++idx2)
			matches_type = textprop.tp_type == type_ids[idx2];
		}
		else
		{
		    matches_type = textprop.tp_type == type_id;
		}

		if (both ? matches_id && matches_type
			 : matches_id || matches_type)
		{
		    if (!(buf->b_ml.ml_flags & ML_LINE_DIRTY))
		    {
			char_u *newptr = alloc(buf->b_ml.ml_line_len);

			// need to allocate the line to be able to change it
			if (newptr == NULL)
			    goto cleanup_prop_remove;
			mch_memmove(newptr, buf->b_ml.ml_line_ptr,
							buf->b_ml.ml_line_len);
			if (buf->b_ml.ml_flags & ML_ALLOCATED)
			    vim_free(buf->b_ml.ml_line_ptr);
			buf->b_ml.ml_line_ptr = newptr;
			buf->b_ml.ml_flags |= ML_LINE_DIRTY;

			cur_prop = buf->b_ml.ml_line_ptr + len
						    + idx * sizeof(textprop_T);
		    }

		    taillen = buf->b_ml.ml_line_len - len
					      - (idx + 1) * sizeof(textprop_T);
		    if (taillen > 0)
			mch_memmove(cur_prop, cur_prop + sizeof(textprop_T),
								      taillen);
		    buf->b_ml.ml_line_len -= sizeof(textprop_T);
		    --idx;

		    if (textprop.tp_id < 0)
		    {
			garray_T    *gap = &buf->b_textprop_text;
			int	    ii = -textprop.tp_id - 1;

			// negative ID: property with text - free the text
			if (ii < gap->ga_len)
			{
			    char_u **p = ((char_u **)gap->ga_data) + ii;
			    VIM_CLEAR(*p);
			    did_remove_text = TRUE;
			}
		    }

		    if (first_changed == 0)
			first_changed = lnum;
		    last_changed = lnum;
		    ++rettv->vval.v_number;
		    if (!do_all)
			break;
		}
	    }
	}
    }

    if (first_changed > 0)
    {
	changed_line_display_buf(buf);
	changed_lines_buf(buf, first_changed, last_changed + 1, 0);
	redraw_buf_later(buf, UPD_VALID);
    }

    if (did_remove_text)
    {
	garray_T    *gap = &buf->b_textprop_text;

	// Reduce the growarray size for NULL pointers at the end.
	while (gap->ga_len > 0
			 && ((char_u **)gap->ga_data)[gap->ga_len - 1] == NULL)
	    --gap->ga_len;
    }

cleanup_prop_remove:
    vim_free(type_ids);
}

/*
 * Common for f_prop_type_add() and f_prop_type_change().
 */
    static void
prop_type_set(typval_T *argvars, int add)
{
    char_u	*name;
    buf_T	*buf = NULL;
    dict_T	*dict;
    dictitem_T  *di;
    proptype_T	*prop;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_dict_arg(argvars, 1) == FAIL))
	return;

    name = tv_get_string(&argvars[0]);
    if (*name == NUL)
    {
	semsg(_(e_invalid_argument_str), "\"\"");
	return;
    }

    if (get_bufnr_from_arg(&argvars[1], &buf) == FAIL)
	return;
    dict = argvars[1].vval.v_dict;

    prop = find_prop_type(name, buf);
    if (add)
    {
	hashtab_T **htp;

	if (prop != NULL)
	{
	    semsg(_(e_property_type_str_already_defined), name);
	    return;
	}
	prop = alloc_clear(offsetof(proptype_T, pt_name) + STRLEN(name) + 1);
	if (prop == NULL)
	    return;
	STRCPY(prop->pt_name, name);
	prop->pt_id = ++proptype_id;
	prop->pt_flags = PT_FLAG_COMBINE;
	if (buf == NULL)
	{
	    htp = &global_proptypes;
	    VIM_CLEAR(global_proparray);
	}
	else
	{
	    htp = &buf->b_proptypes;
	    VIM_CLEAR(buf->b_proparray);
	}
	if (*htp == NULL)
	{
	    *htp = ALLOC_ONE(hashtab_T);
	    if (*htp == NULL)
	    {
		vim_free(prop);
		return;
	    }
	    hash_init(*htp);
	}
	hash_add(*htp, PT2HIKEY(prop), "prop type");
    }
    else
    {
	if (prop == NULL)
	{
	    semsg(_(e_property_type_str_does_not_exist), name);
	    return;
	}
    }

    if (dict != NULL)
    {
	di = dict_find(dict, (char_u *)"highlight", -1);
	if (di != NULL)
	{
	    char_u	*highlight;
	    int		hl_id = 0;

	    highlight = dict_get_string(dict, "highlight", FALSE);
	    if (highlight != NULL && *highlight != NUL)
		hl_id = syn_name2id(highlight);
	    if (hl_id <= 0)
	    {
		semsg(_(e_unknown_highlight_group_name_str),
			highlight == NULL ? (char_u *)"" : highlight);
		return;
	    }
	    prop->pt_hl_id = hl_id;
	}

	di = dict_find(dict, (char_u *)"combine", -1);
	if (di != NULL)
	{
	    if (tv_get_bool(&di->di_tv))
		prop->pt_flags |= PT_FLAG_COMBINE;
	    else
		prop->pt_flags &= ~PT_FLAG_COMBINE;
	}

	di = dict_find(dict, (char_u *)"override", -1);
	if (di != NULL)
	{
	    if (tv_get_bool(&di->di_tv))
		prop->pt_flags |= PT_FLAG_OVERRIDE;
	    else
		prop->pt_flags &= ~PT_FLAG_OVERRIDE;
	}

	di = dict_find(dict, (char_u *)"priority", -1);
	if (di != NULL)
	    prop->pt_priority = tv_get_number(&di->di_tv);

	di = dict_find(dict, (char_u *)"start_incl", -1);
	if (di != NULL)
	{
	    if (tv_get_bool(&di->di_tv))
		prop->pt_flags |= PT_FLAG_INS_START_INCL;
	    else
		prop->pt_flags &= ~PT_FLAG_INS_START_INCL;
	}

	di = dict_find(dict, (char_u *)"end_incl", -1);
	if (di != NULL)
	{
	    if (tv_get_bool(&di->di_tv))
		prop->pt_flags |= PT_FLAG_INS_END_INCL;
	    else
		prop->pt_flags &= ~PT_FLAG_INS_END_INCL;
	}
    }
}

/*
 * prop_type_add({name}, {props})
 */
    void
f_prop_type_add(typval_T *argvars, typval_T *rettv UNUSED)
{
    prop_type_set(argvars, TRUE);
}

/*
 * prop_type_change({name}, {props})
 */
    void
f_prop_type_change(typval_T *argvars, typval_T *rettv UNUSED)
{
    prop_type_set(argvars, FALSE);
}

/*
 * prop_type_delete({name} [, {bufnr}])
 */
    void
f_prop_type_delete(typval_T *argvars, typval_T *rettv UNUSED)
{
    char_u	*name;
    buf_T	*buf = NULL;
    hashitem_T	*hi;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 1) == FAIL))
	return;

    name = tv_get_string(&argvars[0]);
    if (*name == NUL)
    {
	semsg(_(e_invalid_argument_str), "\"\"");
	return;
    }

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	if (get_bufnr_from_arg(&argvars[1], &buf) == FAIL)
	    return;
    }

    hi = find_prop_type_hi(name, buf);
    if (hi == NULL)
	return;

    hashtab_T	*ht;
    proptype_T	*prop = HI2PT(hi);

    if (buf == NULL)
    {
	ht = global_proptypes;
	VIM_CLEAR(global_proparray);
    }
    else
    {
	ht = buf->b_proptypes;
	VIM_CLEAR(buf->b_proparray);
    }
    hash_remove(ht, hi, "prop type delete");
    vim_free(prop);

    // currently visible text properties will disappear
    redraw_all_later(UPD_CLEAR);
    changed_window_setting_buf(buf == NULL ? curbuf : buf);
}

/*
 * prop_type_get({name} [, {props}])
 */
    void
f_prop_type_get(typval_T *argvars, typval_T *rettv)
{
    char_u *name;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 1) == FAIL))
	return;

    name = tv_get_string(&argvars[0]);
    if (*name == NUL)
    {
	semsg(_(e_invalid_argument_str), "\"\"");
	return;
    }

    if (rettv_dict_alloc(rettv) == FAIL)
	return;

    proptype_T  *prop = NULL;
    buf_T	    *buf = NULL;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	if (get_bufnr_from_arg(&argvars[1], &buf) == FAIL)
	    return;
    }

    prop = find_prop_type(name, buf);
    if (prop == NULL)
	return;

    dict_T *d = rettv->vval.v_dict;

    if (prop->pt_hl_id > 0)
	dict_add_string(d, "highlight", syn_id2name(prop->pt_hl_id));
    dict_add_number(d, "priority", prop->pt_priority);
    dict_add_number(d, "combine",
	    (prop->pt_flags & PT_FLAG_COMBINE) ? 1 : 0);
    dict_add_number(d, "start_incl",
	    (prop->pt_flags & PT_FLAG_INS_START_INCL) ? 1 : 0);
    dict_add_number(d, "end_incl",
	    (prop->pt_flags & PT_FLAG_INS_END_INCL) ? 1 : 0);
    if (buf != NULL)
	dict_add_number(d, "bufnr", buf->b_fnum);
}

    static void
list_types(hashtab_T *ht, list_T *l)
{
    long	todo;
    hashitem_T	*hi;

    todo = (long)ht->ht_used;
    FOR_ALL_HASHTAB_ITEMS(ht, hi, todo)
    {
	if (!HASHITEM_EMPTY(hi))
	{
	    proptype_T *prop = HI2PT(hi);

	    list_append_string(l, prop->pt_name, -1);
	    --todo;
	}
    }
}

/*
 * prop_type_list([{bufnr}])
 */
    void
f_prop_type_list(typval_T *argvars, typval_T *rettv UNUSED)
{
    buf_T *buf = NULL;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_dict_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	if (get_bufnr_from_arg(&argvars[0], &buf) == FAIL)
	    return;
    }
    if (buf == NULL)
    {
	if (global_proptypes != NULL)
	    list_types(global_proptypes, rettv->vval.v_list);
    }
    else if (buf->b_proptypes != NULL)
	list_types(buf->b_proptypes, rettv->vval.v_list);
}

/*
 * Free all property types in "ht".
 */
    static void
clear_ht_prop_types(hashtab_T *ht)
{
    long	todo;
    hashitem_T	*hi;

    if (ht == NULL)
	return;

    todo = (long)ht->ht_used;
    FOR_ALL_HASHTAB_ITEMS(ht, hi, todo)
    {
	if (!HASHITEM_EMPTY(hi))
	{
	    proptype_T *prop = HI2PT(hi);

	    vim_free(prop);
	    --todo;
	}
    }

    hash_clear(ht);
    vim_free(ht);
}

#if defined(EXITFREE) || defined(PROTO)
/*
 * Free all global property types.
 */
    void
clear_global_prop_types(void)
{
    clear_ht_prop_types(global_proptypes);
    global_proptypes = NULL;
    VIM_CLEAR(global_proparray);
}
#endif

/*
 * Free all property types for "buf".
 */
    void
clear_buf_prop_types(buf_T *buf)
{
    clear_ht_prop_types(buf->b_proptypes);
    buf->b_proptypes = NULL;
    VIM_CLEAR(buf->b_proparray);
}

// Struct used to return two values from adjust_prop().
typedef struct
{
    int dirty;	    // if the property was changed
    int can_drop;   // whether after this change, the prop may be removed
} adjustres_T;

/*
 * Adjust the property for "added" bytes (can be negative) inserted at "col".
 *
 * Note that "col" is zero-based, while tp_col is one-based.
 * Only for the current buffer.
 * "flags" can have:
 * APC_SUBSTITUTE:	Text is replaced, not inserted.
 * APC_INDENT:		Text is inserted before virtual text prop
 */
    static adjustres_T
adjust_prop(
	textprop_T  *prop,
	colnr_T	    col,
	int	    added,
	int	    flags)
{
    proptype_T	*pt;
    int		start_incl;
    int		end_incl;
    int		droppable;
    adjustres_T res = {TRUE, FALSE};

    // prop after end of the line doesn't move
    if (prop->tp_col == MAXCOL)
    {
	res.dirty = FALSE;
	return res;
    }

    pt = text_prop_type_by_id(curbuf, prop->tp_type);
    start_incl = (pt != NULL && (pt->pt_flags & PT_FLAG_INS_START_INCL))
				|| (flags & APC_SUBSTITUTE)
				|| (prop->tp_flags & TP_FLAG_CONT_PREV);
    if (prop->tp_id < 0 && (flags & APC_INDENT))
	// when inserting indent just before a character with virtual text
	// shift the text property
	start_incl = FALSE;
    end_incl = (pt != NULL && (pt->pt_flags & PT_FLAG_INS_END_INCL))
				|| (prop->tp_flags & TP_FLAG_CONT_NEXT);
    // do not drop zero-width props if they later can increase in size
    droppable = !(start_incl || end_incl);

    if (added > 0)
    {
	if (col + 1 <= prop->tp_col
		- (start_incl || (prop->tp_len == 0 && end_incl)))
	    // Change is entirely before the text property: Only shift
	    prop->tp_col += added;
	else if (col + 1 < prop->tp_col + prop->tp_len + end_incl)
	    // Insertion was inside text property
	    prop->tp_len += added;
    }
    else if (prop->tp_col > col + 1)
    {
	if (prop->tp_col + added < col + 1)
	{
	    prop->tp_len += (prop->tp_col - 1 - col) + added;
	    prop->tp_col = col + 1;
	    if (prop->tp_len <= 0)
	    {
		prop->tp_len = 0;
		res.can_drop = droppable;
	    }
	}
	else
	    prop->tp_col += added;
    }
    else if (prop->tp_len > 0 && prop->tp_col + prop->tp_len > col
	    && prop->tp_id >= 0)  // don't change length for virtual text
    {
	int after = col - added - (prop->tp_col - 1 + prop->tp_len);

	prop->tp_len += after > 0 ? added + after : added;
	res.can_drop = prop->tp_len <= 0 && droppable;
    }
    else
	res.dirty = FALSE;

    return res;
}

/*
 * Adjust the columns of text properties in line "lnum" after position "col" to
 * shift by "bytes_added" (can be negative).
 * Note that "col" is zero-based, while tp_col is one-based.
 * Only for the current buffer.
 * "flags" can have:
 * APC_SAVE_FOR_UNDO:	Call u_savesub() before making changes to the line.
 * APC_SUBSTITUTE:	Text is replaced, not inserted.
 * APC_INDENT:		Text is inserted before virtual text prop
 * Caller is expected to check b_has_textprop and "bytes_added" being non-zero.
 * Returns TRUE when props were changed.
 */
    int
adjust_prop_columns(
	linenr_T    lnum,
	colnr_T	    col,
	int	    bytes_added,
	int	    flags)
{
    int		proplen;
    char_u	*props;
    int		dirty = FALSE;
    int		ri, wi;
    size_t	textlen;

    if (text_prop_frozen > 0)
	return FALSE;

    proplen = get_text_props(curbuf, lnum, &props, TRUE);
    if (proplen == 0)
	return FALSE;
    textlen = curbuf->b_ml.ml_line_len - proplen * sizeof(textprop_T);

    wi = 0; // write index
    for (ri = 0; ri < proplen; ++ri)
    {
	textprop_T	prop;
	adjustres_T	res;

	mch_memmove(&prop, props + ri * sizeof(prop), sizeof(prop));
	res = adjust_prop(&prop, col, bytes_added, flags);
	if (res.dirty)
	{
	    // Save for undo if requested and not done yet.
	    if ((flags & APC_SAVE_FOR_UNDO) && !dirty
						    && u_savesub(lnum) == FAIL)
		return FALSE;
	    dirty = TRUE;

	    // u_savesub() may have updated curbuf->b_ml, fetch it again
	    if (curbuf->b_ml.ml_line_lnum != lnum)
		proplen = get_text_props(curbuf, lnum, &props, TRUE);
	}
	if (res.can_drop)
	    continue; // Drop this text property
	mch_memmove(props + wi * sizeof(textprop_T), &prop, sizeof(textprop_T));
	++wi;
    }
    if (dirty)
    {
	colnr_T newlen = (int)textlen + wi * (colnr_T)sizeof(textprop_T);

	if ((curbuf->b_ml.ml_flags & ML_LINE_DIRTY) == 0)
	{
	    char_u *p = vim_memsave(curbuf->b_ml.ml_line_ptr, newlen);

	    if (curbuf->b_ml.ml_flags & ML_ALLOCATED)
		vim_free(curbuf->b_ml.ml_line_ptr);
	    curbuf->b_ml.ml_line_ptr = p;
	}
	curbuf->b_ml.ml_flags |= ML_LINE_DIRTY;
	curbuf->b_ml.ml_line_len = newlen;
    }
    return dirty;
}

/*
 * Adjust text properties for a line that was split in two.
 * "lnum_props" is the line that has the properties from before the split.
 * "lnum_top" is the top line.
 * "kept" is the number of bytes kept in the first line, while
 * "deleted" is the number of bytes deleted.
 * "at_eol" is true if the split is after the end of the line.
 */
    void
adjust_props_for_split(
	linenr_T    lnum_props,
	linenr_T    lnum_top,
	int	    kept,
	int	    deleted,
	int	    at_eol)
{
    char_u	*props;
    int		count;
    garray_T    prevprop;
    garray_T    nextprop;
    int		i;
    int		skipped = kept + deleted;

    if (!curbuf->b_has_textprop)
	return;

    // Get the text properties from "lnum_props".
    count = get_text_props(curbuf, lnum_props, &props, FALSE);
    ga_init2(&prevprop, sizeof(textprop_T), 10);
    ga_init2(&nextprop, sizeof(textprop_T), 10);

    // Keep the relevant ones in the first line, reducing the length if needed.
    // Copy the ones that include the split to the second line.
    // Move the ones after the split to the second line.
    for (i = 0; i < count; ++i)
    {
	textprop_T  prop;
	proptype_T *pt;
	int	    start_incl, end_incl;
	int	    cont_prev, cont_next;
	int	    prop_col;

	// copy the prop to an aligned structure
	mch_memmove(&prop, props + i * sizeof(textprop_T), sizeof(textprop_T));

	pt = text_prop_type_by_id(curbuf, prop.tp_type);
	start_incl = (pt != NULL && (pt->pt_flags & PT_FLAG_INS_START_INCL));
	end_incl = (pt != NULL && (pt->pt_flags & PT_FLAG_INS_END_INCL));

	// a text prop "above" behaves like it is on the first text column
	prop_col = (prop.tp_flags & TP_FLAG_ALIGN_ABOVE) ? 1 : prop.tp_col;

	if (prop_col == MAXCOL)
	{
	    cont_prev = at_eol;
	    cont_next = !at_eol;
	}
	else
	{
	    cont_prev = prop_col + !start_incl <= kept;
	    cont_next = skipped <= prop_col + prop.tp_len - !end_incl;
	}
	// when a prop has text it is never copied
	if (prop.tp_id < 0 && cont_next)
	    cont_prev = FALSE;

	if (cont_prev && ga_grow(&prevprop, 1) == OK)
	{
	    textprop_T *p = ((textprop_T *)prevprop.ga_data) + prevprop.ga_len;

	    *p = prop;
	    ++prevprop.ga_len;
	    if (p->tp_col != MAXCOL && p->tp_col + p->tp_len >= kept)
		p->tp_len = kept - p->tp_col;
	    if (cont_next)
		p->tp_flags |= TP_FLAG_CONT_NEXT;
	}

	// Only add the property to the next line if the length is bigger than
	// zero.
	if (cont_next && ga_grow(&nextprop, 1) == OK)
	{
	    textprop_T *p = ((textprop_T *)nextprop.ga_data) + nextprop.ga_len;

	    *p = prop;
	    ++nextprop.ga_len;
	    if (p->tp_col != MAXCOL)
	    {
		if (p->tp_col > skipped)
		    p->tp_col -= skipped - 1;
		else
		{
		    p->tp_len -= skipped - p->tp_col;
		    p->tp_col = 1;
		}
	    }
	    if (cont_prev)
		p->tp_flags |= TP_FLAG_CONT_PREV;
	}
    }

    set_text_props(lnum_top, prevprop.ga_data,
					 prevprop.ga_len * sizeof(textprop_T));
    ga_clear(&prevprop);
    set_text_props(lnum_top + 1, nextprop.ga_data,
					 nextprop.ga_len * sizeof(textprop_T));
    ga_clear(&nextprop);
}

/*
 * Prepend properties of joined line "lnum" to "new_props".
 */
    void
prepend_joined_props(
	char_u	    *new_props,
	int	    propcount,
	int	    *props_remaining,
	linenr_T    lnum,
	int	    last_line,
	long	    col,
	int	    removed)
{
    char_u *props;
    int	    proplen = get_text_props(curbuf, lnum, &props, FALSE);
    int	    i;

    for (i = proplen; i-- > 0; )
    {
	textprop_T  prop;
	int	    end;

	mch_memmove(&prop, props + i * sizeof(prop), sizeof(prop));
	if (prop.tp_col == MAXCOL && !last_line)
	    continue;  // drop property with text after the line
	end = !(prop.tp_flags & TP_FLAG_CONT_NEXT);

	adjust_prop(&prop, 0, -removed, 0); // Remove leading spaces
	adjust_prop(&prop, -1, col, 0); // Make line start at its final column

	if (last_line || end)
	    mch_memmove(new_props + --(*props_remaining) * sizeof(prop),
							  &prop, sizeof(prop));
	else
	{
	    int j;
	    int	found = FALSE;

	    // Search for continuing prop.
	    for (j = *props_remaining; j < propcount; ++j)
	    {
		textprop_T op;

		mch_memmove(&op, new_props + j * sizeof(op), sizeof(op));
		if ((op.tp_flags & TP_FLAG_CONT_PREV)
			&& op.tp_id == prop.tp_id && op.tp_type == prop.tp_type)
		{
		    found = TRUE;
		    op.tp_len += op.tp_col - prop.tp_col;
		    op.tp_col = prop.tp_col;
		    // Start/end is taken care of when deleting joined lines
		    op.tp_flags = prop.tp_flags;
		    mch_memmove(new_props + j * sizeof(op), &op, sizeof(op));
		    break;
		}
	    }
	    if (!found)
		internal_error("text property above joined line not found");
	}
    }
}

#endif // FEAT_PROP_POPUP
