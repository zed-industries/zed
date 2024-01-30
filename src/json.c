/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * json.c: Encoding and decoding JSON.
 *
 * Follows this standard: https://tools.ietf.org/html/rfc7159.html
 */
#define USING_FLOAT_STUFF

#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

static int json_encode_item(garray_T *gap, typval_T *val, int copyID, int options);

/*
 * Encode "val" into a JSON format string.
 * The result is added to "gap"
 * Returns FAIL on failure and makes gap->ga_data empty.
 */
    static int
json_encode_gap(garray_T *gap, typval_T *val, int options)
{
    if (json_encode_item(gap, val, get_copyID(), options) == FAIL)
    {
	ga_clear(gap);
	gap->ga_data = vim_strsave((char_u *)"");
	return FAIL;
    }
    return OK;
}

/*
 * Encode "val" into a JSON format string.
 * The result is in allocated memory.
 * The result is empty when encoding fails.
 * "options" can contain JSON_JS, JSON_NO_NONE and JSON_NL.
 */
    char_u *
json_encode(typval_T *val, int options)
{
    garray_T ga;

    // Store bytes in the growarray.
    ga_init2(&ga, 1, 4000);
    json_encode_gap(&ga, val, options);
    ga_append(&ga, NUL);
    return ga.ga_data;
}

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
/*
 * Encode ["nr", "val"] into a JSON format string in allocated memory.
 * "options" can contain JSON_JS, JSON_NO_NONE and JSON_NL.
 * Returns NULL when out of memory.
 */
    char_u *
json_encode_nr_expr(int nr, typval_T *val, int options)
{
    typval_T	listtv;
    typval_T	nrtv;
    garray_T	ga;

    nrtv.v_type = VAR_NUMBER;
    nrtv.vval.v_number = nr;
    if (rettv_list_alloc(&listtv) == FAIL)
	return NULL;
    if (list_append_tv(listtv.vval.v_list, &nrtv) == FAIL
	    || list_append_tv(listtv.vval.v_list, val) == FAIL)
    {
	list_unref(listtv.vval.v_list);
	return NULL;
    }

    ga_init2(&ga, 1, 4000);
    if (json_encode_gap(&ga, &listtv, options) == OK && (options & JSON_NL))
	ga_append(&ga, '\n');
    list_unref(listtv.vval.v_list);
    ga_append(&ga, NUL);
    return ga.ga_data;
}

/*
 * Encode "val" into a JSON format string prefixed by the LSP HTTP header.
 * Returns NULL when out of memory.
 */
    char_u *
json_encode_lsp_msg(typval_T *val)
{
    garray_T	ga;
    garray_T	lspga;

    ga_init2(&ga, 1, 4000);
    if (json_encode_gap(&ga, val, 0) == FAIL)
	return NULL;
    ga_append(&ga, NUL);

    ga_init2(&lspga, 1, 4000);
    // Header according to LSP specification.
    vim_snprintf((char *)IObuff, IOSIZE,
	    "Content-Length: %u\r\n\r\n",
	    ga.ga_len - 1);
    ga_concat(&lspga, IObuff);
    ga_concat_len(&lspga, ga.ga_data, ga.ga_len);
    ga_clear(&ga);
    return lspga.ga_data;
}
#endif

/*
 * Lookup table to quickly know if the given ASCII character must be escaped.
 */
static const char ascii_needs_escape[128] = {
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x0.
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x1.
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x2.
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x3.
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x4.
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, // 0x5.
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x6.
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x7.
};

/*
 * Encode the utf-8 encoded string "str" into "gap".
 */
    static void
write_string(garray_T *gap, char_u *str)
{
    char_u	*res = str;
    char_u	numbuf[NUMBUFLEN];
    char_u	*from;
#if defined(USE_ICONV)
    vimconv_T   conv;
    char_u	*converted = NULL;
#endif
    int		c;

    if (res == NULL)
    {
	ga_concat(gap, (char_u *)"\"\"");
	return;
    }

#if defined(USE_ICONV)
    if (!enc_utf8)
    {
	// Convert the text from 'encoding' to utf-8, because a JSON string is
	// always utf-8.
	conv.vc_type = CONV_NONE;
	convert_setup(&conv, p_enc, (char_u*)"utf-8");
	if (conv.vc_type != CONV_NONE)
	    converted = res = string_convert(&conv, res, NULL);
	convert_setup(&conv, NULL, NULL);
    }
#endif
    ga_append(gap, '"');
    // `from` is the beginning of a sequence of bytes we can directly copy from
    // the input string, avoiding the overhead associated to decoding/encoding
    // them.
    from = res;
    while ((c = *res) != NUL)
    {
	// always use utf-8 encoding, ignore 'encoding'
	if (c < 0x80)
	{
	    if (!ascii_needs_escape[c])
	    {
		res += 1;
		continue;
	    }

	    if (res != from)
		ga_concat_len(gap, from, res - from);
	    from = res + 1;

	    switch (c)
	    {
		case 0x08:
		    ga_append(gap, '\\'); ga_append(gap, 'b'); break;
		case 0x09:
		    ga_append(gap, '\\'); ga_append(gap, 't'); break;
		case 0x0a:
		    ga_append(gap, '\\'); ga_append(gap, 'n'); break;
		case 0x0c:
		    ga_append(gap, '\\'); ga_append(gap, 'f'); break;
		case 0x0d:
		    ga_append(gap, '\\'); ga_append(gap, 'r'); break;
		case 0x22: // "
		case 0x5c: // backslash
		    ga_append(gap, '\\');
		    ga_append(gap, c);
		    break;
		default:
		    vim_snprintf((char *)numbuf, NUMBUFLEN, "\\u%04lx",
								      (long)c);
		    ga_concat(gap, numbuf);
	    }

	    res += 1;
	}
	else
	{
	    int l = utf_ptr2len(res);

	    if (l > 1)
	    {
		res += l;
		continue;
	    }

	    // Invalid utf-8 sequence, replace it with the Unicode replacement
	    // character U+FFFD.
	    if (res != from)
		ga_concat_len(gap, from, res - from);
	    from = res + 1;

	    numbuf[utf_char2bytes(0xFFFD, numbuf)] = NUL;
	    ga_concat(gap, numbuf);

	    res += l;
	}
    }

    if (res != from)
	ga_concat_len(gap, from, res - from);

    ga_append(gap, '"');
#if defined(USE_ICONV)
    vim_free(converted);
#endif
}

/*
 * Return TRUE if "key" can be used without quotes.
 * That is when it starts with a letter and only contains letters, digits and
 * underscore.
 */
    static int
is_simple_key(char_u *key)
{
    char_u *p;

    if (!ASCII_ISALPHA(*key))
	return FALSE;
    for (p = key + 1; *p != NUL; ++p)
	if (!ASCII_ISALPHA(*p) && *p != '_' && !vim_isdigit(*p))
	    return FALSE;
    return TRUE;
}

/*
 * Encode "val" into "gap".
 * Return FAIL or OK.
 */
    static int
json_encode_item(garray_T *gap, typval_T *val, int copyID, int options)
{
    char_u	numbuf[NUMBUFLEN];
    char_u	*res;
    blob_T	*b;
    list_T	*l;
    dict_T	*d;
    int		i;

    switch (val->v_type)
    {
	case VAR_BOOL:
	    switch ((long)val->vval.v_number)
	    {
		case VVAL_FALSE: ga_concat(gap, (char_u *)"false"); break;
		case VVAL_TRUE: ga_concat(gap, (char_u *)"true"); break;
	    }
	    break;

	case VAR_SPECIAL:
	    switch ((long)val->vval.v_number)
	    {
		case VVAL_NONE: if ((options & JSON_JS) != 0
					     && (options & JSON_NO_NONE) == 0)
				    // empty item
				    break;
				// FALLTHROUGH
		case VVAL_NULL: ga_concat(gap, (char_u *)"null"); break;
	    }
	    break;

	case VAR_NUMBER:
	    vim_snprintf((char *)numbuf, NUMBUFLEN, "%lld",
					      (varnumber_T)val->vval.v_number);
	    ga_concat(gap, numbuf);
	    break;

	case VAR_STRING:
	    res = val->vval.v_string;
	    write_string(gap, res);
	    break;

	case VAR_FUNC:
	case VAR_PARTIAL:
	case VAR_JOB:
	case VAR_CHANNEL:
	case VAR_INSTR:
	case VAR_CLASS:
	case VAR_OBJECT:
	case VAR_TYPEALIAS:
	    semsg(_(e_cannot_json_encode_str), vartype_name(val->v_type));
	    return FAIL;

	case VAR_BLOB:
	    b = val->vval.v_blob;
	    if (b == NULL || b->bv_ga.ga_len == 0)
		ga_concat(gap, (char_u *)"[]");
	    else
	    {
		ga_append(gap, '[');
		for (i = 0; i < b->bv_ga.ga_len; i++)
		{
		    if (i > 0)
			ga_concat(gap, (char_u *)",");
		    vim_snprintf((char *)numbuf, NUMBUFLEN, "%d",
			    blob_get(b, i));
		    ga_concat(gap, numbuf);
		}
		ga_append(gap, ']');
	    }
	    break;

	case VAR_LIST:
	    l = val->vval.v_list;
	    if (l == NULL)
		ga_concat(gap, (char_u *)"[]");
	    else
	    {
		if (l->lv_copyID == copyID)
		    ga_concat(gap, (char_u *)"[]");
		else
		{
		    listitem_T	*li;

		    l->lv_copyID = copyID;
		    ga_append(gap, '[');
		    CHECK_LIST_MATERIALIZE(l);
		    for (li = l->lv_first; li != NULL && !got_int; )
		    {
			if (json_encode_item(gap, &li->li_tv, copyID,
						   options & JSON_JS) == FAIL)
			    return FAIL;
			if ((options & JSON_JS)
				&& li->li_next == NULL
				&& li->li_tv.v_type == VAR_SPECIAL
				&& li->li_tv.vval.v_number == VVAL_NONE)
			    // add an extra comma if the last item is v:none
			    ga_append(gap, ',');
			li = li->li_next;
			if (li != NULL)
			    ga_append(gap, ',');
		    }
		    ga_append(gap, ']');
		    l->lv_copyID = 0;
		}
	    }
	    break;

	case VAR_DICT:
	    d = val->vval.v_dict;
	    if (d == NULL)
		ga_concat(gap, (char_u *)"{}");
	    else
	    {
		if (d->dv_copyID == copyID)
		    ga_concat(gap, (char_u *)"{}");
		else
		{
		    int		first = TRUE;
		    int		todo = (int)d->dv_hashtab.ht_used;
		    hashitem_T	*hi;

		    d->dv_copyID = copyID;
		    ga_append(gap, '{');

		    for (hi = d->dv_hashtab.ht_array; todo > 0 && !got_int;
									 ++hi)
			if (!HASHITEM_EMPTY(hi))
			{
			    --todo;
			    if (first)
				first = FALSE;
			    else
				ga_append(gap, ',');
			    if ((options & JSON_JS)
						 && is_simple_key(hi->hi_key))
				ga_concat(gap, hi->hi_key);
			    else
				write_string(gap, hi->hi_key);
			    ga_append(gap, ':');
			    if (json_encode_item(gap, &dict_lookup(hi)->di_tv,
				      copyID, options | JSON_NO_NONE) == FAIL)
				return FAIL;
			}
		    ga_append(gap, '}');
		    d->dv_copyID = 0;
		}
	    }
	    break;

	case VAR_FLOAT:
#if defined(HAVE_MATH_H)
	    if (isnan(val->vval.v_float))
		ga_concat(gap, (char_u *)"NaN");
	    else if (isinf(val->vval.v_float))
	    {
		if (val->vval.v_float < 0.0)
		    ga_concat(gap, (char_u *)"-Infinity");
		else
		    ga_concat(gap, (char_u *)"Infinity");
	    }
	    else
#endif
	    {
		vim_snprintf((char *)numbuf, NUMBUFLEN, "%g",
							   val->vval.v_float);
		ga_concat(gap, numbuf);
	    }
	    break;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	    internal_error_no_abort("json_encode_item()");
	    return FAIL;
    }
    return OK;
}

/*
 * When "reader" has less than NUMBUFLEN bytes available, call the fill
 * callback to get more.
 */
    static void
fill_numbuflen(js_read_T *reader)
{
    if (reader->js_fill != NULL && (int)(reader->js_end - reader->js_buf)
						- reader->js_used < NUMBUFLEN)
    {
	if (reader->js_fill(reader))
	    reader->js_end = reader->js_buf + STRLEN(reader->js_buf);
    }
}

/*
 * Skip white space in "reader".  All characters <= space are considered white
 * space.
 * Also tops up readahead when needed.
 */
    static void
json_skip_white(js_read_T *reader)
{
    int c;

    for (;;)
    {
	c = reader->js_buf[reader->js_used];
	if (reader->js_fill != NULL && c == NUL)
	{
	    if (reader->js_fill(reader))
	    {
		reader->js_end = reader->js_buf + STRLEN(reader->js_buf);
		continue;
	    }
	}
	if (c == NUL || c > ' ')
	    break;
	++reader->js_used;
    }
    fill_numbuflen(reader);
}

    static int
json_decode_string(js_read_T *reader, typval_T *res, int quote)
{
    garray_T    ga;
    int		len;
    char_u	*p;
    int		c;
    varnumber_T	nr;

    if (res != NULL)
	ga_init2(&ga, 1, 200);

    p = reader->js_buf + reader->js_used + 1; // skip over " or '
    while (*p != quote)
    {
	// The JSON is always expected to be utf-8, thus use utf functions
	// here. The string is converted below if needed.
	if (*p == NUL || p[1] == NUL || utf_ptr2len(p) < utf_byte2len(*p))
	{
	    // Not enough bytes to make a character or end of the string. Get
	    // more if possible.
	    if (reader->js_fill == NULL)
		break;
	    len = (int)(reader->js_end - p);
	    reader->js_used = (int)(p - reader->js_buf);
	    if (!reader->js_fill(reader))
		break; // didn't get more
	    p = reader->js_buf + reader->js_used;
	    reader->js_end = reader->js_buf + STRLEN(reader->js_buf);
	    continue;
	}

	if (*p == '\\')
	{
	    c = -1;
	    switch (p[1])
	    {
		case '\\': c = '\\'; break;
		case '"': c = '"'; break;
		case 'b': c = BS; break;
		case 't': c = TAB; break;
		case 'n': c = NL; break;
		case 'f': c = FF; break;
		case 'r': c = CAR; break;
		case 'u':
		    if (reader->js_fill != NULL
				     && (int)(reader->js_end - p) < NUMBUFLEN)
		    {
			reader->js_used = (int)(p - reader->js_buf);
			if (reader->js_fill(reader))
			{
			    p = reader->js_buf + reader->js_used;
			    reader->js_end = reader->js_buf
						     + STRLEN(reader->js_buf);
			}
		    }
		    nr = 0;
		    len = 0;
		    vim_str2nr(p + 2, NULL, &len,
			     STR2NR_HEX + STR2NR_FORCE, &nr, NULL, 4, TRUE, NULL);
		    if (len == 0)
		    {
			if (res != NULL)
			    ga_clear(&ga);
			return FAIL;
		    }
		    p += len + 2;
		    if (0xd800 <= nr && nr <= 0xdfff
			    && (int)(reader->js_end - p) >= 6
			    && *p == '\\' && *(p+1) == 'u')
		    {
			varnumber_T	nr2 = 0;

			// decode surrogate pair: \ud812\u3456
			len = 0;
			vim_str2nr(p + 2, NULL, &len, STR2NR_HEX + STR2NR_FORCE,
						    &nr2, NULL, 4, TRUE, NULL);
			if (len == 0)
			{
			    if (res != NULL)
				ga_clear(&ga);
			    return FAIL;
			}
			if (0xdc00 <= nr2 && nr2 <= 0xdfff)
			{
			    p += len + 2;
			    nr = (((nr - 0xd800) << 10) |
				((nr2 - 0xdc00) & 0x3ff)) + 0x10000;
			}
		    }
		    if (res != NULL)
		    {
			char_u	buf[NUMBUFLEN];

			buf[utf_char2bytes((int)nr, buf)] = NUL;
			ga_concat(&ga, buf);
		    }
		    break;
		default:
		    // not a special char, skip over backslash
		    ++p;
		    continue;
	    }
	    if (c > 0)
	    {
		p += 2;
		if (res != NULL)
		    ga_append(&ga, c);
	    }
	}
	else
	{
	    len = utf_ptr2len(p);
	    if (res != NULL)
	    {
		if (ga_grow(&ga, len) == FAIL)
		{
		    ga_clear(&ga);
		    return FAIL;
		}
		mch_memmove((char *)ga.ga_data + ga.ga_len, p, (size_t)len);
		ga.ga_len += len;
	    }
	    p += len;
	}
    }

    reader->js_used = (int)(p - reader->js_buf);
    if (*p == quote)
    {
	++reader->js_used;
	if (res != NULL)
	{
	    ga_append(&ga, NUL);
	    res->v_type = VAR_STRING;
#if defined(USE_ICONV)
	    if (!enc_utf8)
	    {
		vimconv_T   conv;

		// Convert the utf-8 string to 'encoding'.
		conv.vc_type = CONV_NONE;
		convert_setup(&conv, (char_u*)"utf-8", p_enc);
		if (conv.vc_type != CONV_NONE)
		{
		    res->vval.v_string =
				      string_convert(&conv, ga.ga_data, NULL);
		    vim_free(ga.ga_data);
		}
		convert_setup(&conv, NULL, NULL);
	    }
	    else
#endif
		res->vval.v_string = ga.ga_data;
	}
	return OK;
    }
    if (res != NULL)
    {
	res->v_type = VAR_SPECIAL;
	res->vval.v_number = VVAL_NONE;
	ga_clear(&ga);
    }
    return MAYBE;
}

typedef enum {
    JSON_ARRAY,		// parsing items in an array
    JSON_OBJECT_KEY,	// parsing key of an object
    JSON_OBJECT		// parsing item in an object, after the key
} json_decode_T;

typedef struct {
    json_decode_T jd_type;
    typval_T	  jd_tv;	// the list or dict
    typval_T	  jd_key_tv;
    char_u	  *jd_key;
} json_dec_item_T;

/*
 * Decode one item and put it in "res".  If "res" is NULL only advance.
 * Must already have skipped white space.
 *
 * Return FAIL for a decoding error (and give an error).
 * Return MAYBE for an incomplete message.
 */
    static int
json_decode_item(js_read_T *reader, typval_T *res, int options)
{
    char_u	*p;
    int		i;
    int		len;
    int		retval;
    garray_T	stack;
    typval_T	item;
    typval_T	*cur_item;
    json_dec_item_T *top_item;
    char_u	key_buf[NUMBUFLEN];

    ga_init2(&stack, sizeof(json_dec_item_T), 100);
    cur_item = res;
    init_tv(&item);
    if (res != NULL)
	init_tv(res);

    fill_numbuflen(reader);
    p = reader->js_buf + reader->js_used;
    for (;;)
    {
	top_item = NULL;
	if (stack.ga_len > 0)
	{
	    top_item = ((json_dec_item_T *)stack.ga_data) + stack.ga_len - 1;
	    json_skip_white(reader);
	    p = reader->js_buf + reader->js_used;
	    if (*p == NUL)
	    {
		retval = MAYBE;
		goto theend;
	    }
	    if (top_item->jd_type == JSON_OBJECT_KEY
					    || top_item->jd_type == JSON_ARRAY)
	    {
		// Check for end of object or array.
		if (*p == (top_item->jd_type == JSON_ARRAY ? ']' : '}'))
		{
		    ++reader->js_used; // consume the ']' or '}'
		    --stack.ga_len;
		    if (stack.ga_len == 0)
		    {
			retval = OK;
			goto theend;
		    }
		    if (cur_item != NULL)
			cur_item = &top_item->jd_tv;
		    goto item_end;
		}
	    }
	}

	if (top_item != NULL && top_item->jd_type == JSON_OBJECT_KEY
		&& (options & JSON_JS)
		&& reader->js_buf[reader->js_used] != '"'
		&& reader->js_buf[reader->js_used] != '\''
		&& reader->js_buf[reader->js_used] != '['
		&& reader->js_buf[reader->js_used] != '{')
	{
	    char_u *key;

	    // accept an object key that is not in quotes
	    key = p = reader->js_buf + reader->js_used;
	    while (*p != NUL && *p != ':' && *p > ' ')
		++p;
	    if (cur_item != NULL)
	    {
		cur_item->v_type = VAR_STRING;
		cur_item->vval.v_string = vim_strnsave(key, p - key);
		top_item->jd_key = cur_item->vval.v_string;
	    }
	    reader->js_used += (int)(p - key);
	}
	else
	{
	    switch (*p)
	    {
		case '[': // start of array
		    if (top_item && top_item->jd_type == JSON_OBJECT_KEY)
		    {
			retval = FAIL;
			break;
		    }
		    if (ga_grow(&stack, 1) == FAIL)
		    {
			retval = FAIL;
			break;
		    }
		    if (cur_item != NULL && rettv_list_alloc(cur_item) == FAIL)
		    {
			cur_item->v_type = VAR_SPECIAL;
			cur_item->vval.v_number = VVAL_NONE;
			retval = FAIL;
			break;
		    }

		    ++reader->js_used; // consume the '['
		    top_item = ((json_dec_item_T *)stack.ga_data)
								+ stack.ga_len;
		    top_item->jd_type = JSON_ARRAY;
		    ++stack.ga_len;
		    if (cur_item != NULL)
		    {
			top_item->jd_tv = *cur_item;
			cur_item = &item;
		    }
		    continue;

		case '{': // start of object
		    if (top_item && top_item->jd_type == JSON_OBJECT_KEY)
		    {
			retval = FAIL;
			break;
		    }
		    if (ga_grow(&stack, 1) == FAIL)
		    {
			retval = FAIL;
			break;
		    }
		    if (cur_item != NULL && rettv_dict_alloc(cur_item) == FAIL)
		    {
			cur_item->v_type = VAR_SPECIAL;
			cur_item->vval.v_number = VVAL_NONE;
			retval = FAIL;
			break;
		    }

		    ++reader->js_used; // consume the '{'
		    top_item = ((json_dec_item_T *)stack.ga_data)
								+ stack.ga_len;
		    top_item->jd_type = JSON_OBJECT_KEY;
		    ++stack.ga_len;
		    if (cur_item != NULL)
		    {
			top_item->jd_tv = *cur_item;
			cur_item = &top_item->jd_key_tv;
		    }
		    continue;

		case '"': // string
		    retval = json_decode_string(reader, cur_item, *p);
		    break;

		case '\'':
		    if (options & JSON_JS)
			retval = json_decode_string(reader, cur_item, *p);
		    else
		    {
			semsg(_(e_json_decode_error_at_str), p);
			retval = FAIL;
		    }
		    break;

		case ',': // comma: empty item
		    if ((options & JSON_JS) == 0)
		    {
			semsg(_(e_json_decode_error_at_str), p);
			retval = FAIL;
			break;
		    }
		    // FALLTHROUGH
		case NUL: // empty
		    if (cur_item != NULL)
		    {
			cur_item->v_type = VAR_SPECIAL;
			cur_item->vval.v_number = VVAL_NONE;
		    }
		    retval = OK;
		    break;

		default:
		    if (VIM_ISDIGIT(*p) || (*p == '-'
					&& (VIM_ISDIGIT(p[1]) || p[1] == NUL)))
		    {
			char_u  *sp = p;

			if (*sp == '-')
			{
			    ++sp;
			    if (*sp == NUL)
			    {
				retval = MAYBE;
				break;
			    }
			    if (!VIM_ISDIGIT(*sp))
			    {
				semsg(_(e_json_decode_error_at_str), p);
				retval = FAIL;
				break;
			    }
			}
			sp = skipdigits(sp);
			if (*sp == '.' || *sp == 'e' || *sp == 'E')
			{
			    if (cur_item == NULL)
			    {
				float_T f;

				len = string2float(p, &f, FALSE);
			    }
			    else
			    {
				cur_item->v_type = VAR_FLOAT;
				len = string2float(p, &cur_item->vval.v_float,
									FALSE);
			    }
			}
			else
			{
			    varnumber_T nr;

			    vim_str2nr(reader->js_buf + reader->js_used,
				    NULL, &len, 0, // what
				    &nr, NULL, 0, TRUE, NULL);
			    if (len == 0)
			    {
				semsg(_(e_json_decode_error_at_str), p);
				retval = FAIL;
				goto theend;
			    }
			    if (cur_item != NULL)
			    {
				cur_item->v_type = VAR_NUMBER;
				cur_item->vval.v_number = nr;
			    }
			}
			reader->js_used += len;
			retval = OK;
			break;
		    }
		    if (STRNICMP((char *)p, "false", 5) == 0)
		    {
			reader->js_used += 5;
			if (cur_item != NULL)
			{
			    cur_item->v_type = VAR_BOOL;
			    cur_item->vval.v_number = VVAL_FALSE;
			}
			retval = OK;
			break;
		    }
		    if (STRNICMP((char *)p, "true", 4) == 0)
		    {
			reader->js_used += 4;
			if (cur_item != NULL)
			{
			    cur_item->v_type = VAR_BOOL;
			    cur_item->vval.v_number = VVAL_TRUE;
			}
			retval = OK;
			break;
		    }
		    if (STRNICMP((char *)p, "null", 4) == 0)
		    {
			reader->js_used += 4;
			if (cur_item != NULL)
			{
			    cur_item->v_type = VAR_SPECIAL;
			    cur_item->vval.v_number = VVAL_NULL;
			}
			retval = OK;
			break;
		    }
		    if (STRNICMP((char *)p, "NaN", 3) == 0)
		    {
			reader->js_used += 3;
			if (cur_item != NULL)
			{
			    cur_item->v_type = VAR_FLOAT;
			    cur_item->vval.v_float = NAN;
			}
			retval = OK;
			break;
		    }
		    if (STRNICMP((char *)p, "-Infinity", 9) == 0)
		    {
			reader->js_used += 9;
			if (cur_item != NULL)
			{
			    cur_item->v_type = VAR_FLOAT;
			    cur_item->vval.v_float = -INFINITY;
			}
			retval = OK;
			break;
		    }
		    if (STRNICMP((char *)p, "Infinity", 8) == 0)
		    {
			reader->js_used += 8;
			if (cur_item != NULL)
			{
			    cur_item->v_type = VAR_FLOAT;
			    cur_item->vval.v_float = INFINITY;
			}
			retval = OK;
			break;
		    }
		    // check for truncated name
		    len = (int)(reader->js_end
					 - (reader->js_buf + reader->js_used));
		    if (
			    (len < 5 && STRNICMP((char *)p, "false", len) == 0)
			    || (len < 9
				 && STRNICMP((char *)p, "-Infinity", len) == 0)
			    || (len < 8
				  && STRNICMP((char *)p, "Infinity", len) == 0)
			    || (len < 3 && STRNICMP((char *)p, "NaN", len) == 0)
			    || (len < 4
				  && (STRNICMP((char *)p, "true", len) == 0
				    || STRNICMP((char *)p, "null", len) == 0)))

			retval = MAYBE;
		    else
			retval = FAIL;
		    break;
	    }

	    // We are finished when retval is FAIL or MAYBE and when at the
	    // toplevel.
	    if (retval == FAIL)
		break;
	    if (retval == MAYBE || stack.ga_len == 0)
		goto theend;

	    if (top_item != NULL && top_item->jd_type == JSON_OBJECT_KEY
		    && cur_item != NULL)
	    {
		if (cur_item->v_type == VAR_FLOAT)
		{
		    // cannot use a float as a key
		    emsg(_(e_using_float_as_string));
		    retval = FAIL;
		    goto theend;
		}
		top_item->jd_key = tv_get_string_buf_chk(cur_item, key_buf);
		if (top_item->jd_key == NULL)
		{
		    emsg(_(e_invalid_argument));
		    retval = FAIL;
		    goto theend;
		}
	    }
	}

item_end:
	top_item = ((json_dec_item_T *)stack.ga_data) + stack.ga_len - 1;
	switch (top_item->jd_type)
	{
	    case JSON_ARRAY:
		if (res != NULL)
		{
		    listitem_T	*li = listitem_alloc();

		    if (li == NULL)
		    {
			clear_tv(cur_item);
			retval = FAIL;
			goto theend;
		    }
		    li->li_tv = *cur_item;
		    list_append(top_item->jd_tv.vval.v_list, li);
		}
		if (cur_item != NULL)
		    cur_item = &item;

		json_skip_white(reader);
		p = reader->js_buf + reader->js_used;
		if (*p == ',')
		    ++reader->js_used;
		else if (*p != ']')
		{
		    if (*p == NUL)
			retval = MAYBE;
		    else
		    {
			semsg(_(e_json_decode_error_at_str), p);
			retval = FAIL;
		    }
		    goto theend;
		}
		break;

	    case JSON_OBJECT_KEY:
		json_skip_white(reader);
		p = reader->js_buf + reader->js_used;
		if (*p != ':')
		{
		    if (cur_item != NULL)
			clear_tv(cur_item);
		    if (*p == NUL)
			retval = MAYBE;
		    else
		    {
			semsg(_(e_json_decode_error_at_str), p);
			retval = FAIL;
		    }
		    goto theend;
		}
		++reader->js_used;
		json_skip_white(reader);
		top_item->jd_type = JSON_OBJECT;
		if (cur_item != NULL)
		    cur_item = &item;
		break;

	    case JSON_OBJECT:
		if (cur_item != NULL
			&& dict_has_key(top_item->jd_tv.vval.v_dict,
						(char *)top_item->jd_key))
		{
		    semsg(_(e_duplicate_key_in_json_str), top_item->jd_key);
		    clear_tv(cur_item);
		    retval = FAIL;
		    goto theend;
		}

		if (cur_item != NULL)
		{
		    dictitem_T *di = dictitem_alloc(top_item->jd_key);

		    clear_tv(&top_item->jd_key_tv);
		    if (di == NULL)
		    {
			clear_tv(cur_item);
			retval = FAIL;
			goto theend;
		    }
		    di->di_tv = *cur_item;
		    di->di_tv.v_lock = 0;
		    if (dict_add(top_item->jd_tv.vval.v_dict, di) == FAIL)
		    {
			dictitem_free(di);
			retval = FAIL;
			goto theend;
		    }
		}

		json_skip_white(reader);
		p = reader->js_buf + reader->js_used;
		if (*p == ',')
		    ++reader->js_used;
		else if (*p != '}')
		{
		    if (*p == NUL)
			retval = MAYBE;
		    else
		    {
			semsg(_(e_json_decode_error_at_str), p);
			retval = FAIL;
		    }
		    goto theend;
		}
		top_item->jd_type = JSON_OBJECT_KEY;
		if (cur_item != NULL)
		    cur_item = &top_item->jd_key_tv;
		break;
	}
    }

    // Get here when parsing failed.
    if (res != NULL)
    {
	clear_tv(res);
	res->v_type = VAR_SPECIAL;
	res->vval.v_number = VVAL_NONE;
    }
    semsg(_(e_json_decode_error_at_str), p);

theend:
    for (i = 0; i < stack.ga_len; i++)
	clear_tv(&(((json_dec_item_T *)stack.ga_data) + i)->jd_key_tv);
    ga_clear(&stack);

    return retval;
}

/*
 * Decode the JSON from "reader" and store the result in "res".
 * "options" can be JSON_JS or zero;
 * Return FAIL if not the whole message was consumed.
 */
    static int
json_decode_all(js_read_T *reader, typval_T *res, int options)
{
    int ret;

    // We find the end once, to avoid calling strlen() many times.
    reader->js_end = reader->js_buf + STRLEN(reader->js_buf);
    json_skip_white(reader);
    ret = json_decode_item(reader, res, options);
    if (ret != OK)
    {
	if (ret == MAYBE)
	    semsg(_(e_json_decode_error_at_str), reader->js_buf);
	return FAIL;
    }
    json_skip_white(reader);
    if (reader->js_buf[reader->js_used] != NUL)
    {
	semsg(_(e_trailing_characters_str), reader->js_buf + reader->js_used);
	return FAIL;
    }
    return OK;
}

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
/*
 * Decode the JSON from "reader" and store the result in "res".
 * "options" can be JSON_JS or zero;
 * Return FAIL for a decoding error.
 * Return MAYBE for an incomplete message.
 * Consumes the message anyway.
 */
    int
json_decode(js_read_T *reader, typval_T *res, int options)
{
    int ret;

    // We find the end once, to avoid calling strlen() many times.
    reader->js_end = reader->js_buf + STRLEN(reader->js_buf);
    json_skip_white(reader);
    ret = json_decode_item(reader, res, options);
    json_skip_white(reader);

    return ret;
}
#endif

/*
 * Decode the JSON from "reader" to find the end of the message.
 * "options" can be JSON_JS or zero.
 * This is only used for testing.
 * Return FAIL if the message has a decoding error.
 * Return MAYBE if the message is truncated, need to read more.
 * This only works reliable if the message contains an object, array or
 * string.  A number might be truncated without knowing.
 * Does not advance the reader.
 */
    int
json_find_end(js_read_T *reader, int options)
{
    int used_save = reader->js_used;
    int ret;

    // We find the end once, to avoid calling strlen() many times.
    reader->js_end = reader->js_buf + STRLEN(reader->js_buf);
    json_skip_white(reader);
    ret = json_decode_item(reader, NULL, options);
    reader->js_used = used_save;
    return ret;
}

/*
 * "js_decode()" function
 */
    void
f_js_decode(typval_T *argvars, typval_T *rettv)
{
    js_read_T	reader;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    reader.js_buf = tv_get_string(&argvars[0]);
    reader.js_fill = NULL;
    reader.js_used = 0;
    if (json_decode_all(&reader, rettv, JSON_JS) != OK)
	emsg(_(e_invalid_argument));
}

/*
 * "js_encode()" function
 */
    void
f_js_encode(typval_T *argvars, typval_T *rettv)
{
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = json_encode(&argvars[0], JSON_JS);
}

/*
 * "json_decode()" function
 */
    void
f_json_decode(typval_T *argvars, typval_T *rettv)
{
    js_read_T	reader;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    reader.js_buf = tv_get_string(&argvars[0]);
    reader.js_fill = NULL;
    reader.js_used = 0;
    json_decode_all(&reader, rettv, 0);
}

/*
 * "json_encode()" function
 */
    void
f_json_encode(typval_T *argvars, typval_T *rettv)
{
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = json_encode(&argvars[0], 0);
}
#endif
