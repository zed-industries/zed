/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * blob.c: Blob support by Yasuhiro Matsumoto
 */

#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

/*
 * Allocate an empty blob.
 * Caller should take care of the reference count.
 */
    blob_T *
blob_alloc(void)
{
    blob_T *blob = ALLOC_CLEAR_ONE_ID(blob_T, aid_blob_alloc);

    if (blob != NULL)
	ga_init2(&blob->bv_ga, 1, 100);
    return blob;
}

/*
 * Allocate an empty blob for a return value, with reference count set.
 * Returns OK or FAIL.
 */
    int
rettv_blob_alloc(typval_T *rettv)
{
    blob_T	*b = blob_alloc();

    if (b == NULL)
	return FAIL;

    rettv_blob_set(rettv, b);
    return OK;
}

/*
 * Set a blob as the return value.
 */
    void
rettv_blob_set(typval_T *rettv, blob_T *b)
{
    rettv->v_type = VAR_BLOB;
    rettv->vval.v_blob = b;
    if (b != NULL)
	++b->bv_refcount;
}

    int
blob_copy(blob_T *from, typval_T *to)
{
    int		len;

    to->v_type = VAR_BLOB;
    to->v_lock = 0;
    if (from == NULL)
    {
	to->vval.v_blob = NULL;
	return OK;
    }

    if (rettv_blob_alloc(to) == FAIL)
	return FAIL;

    len = from->bv_ga.ga_len;
    if (len > 0)
    {
	to->vval.v_blob->bv_ga.ga_data =
	    vim_memsave(from->bv_ga.ga_data, len);
	if (to->vval.v_blob->bv_ga.ga_data == NULL)
	    len = 0;
    }
    to->vval.v_blob->bv_ga.ga_len = len;
    to->vval.v_blob->bv_ga.ga_maxlen = len;

    return OK;
}

    void
blob_free(blob_T *b)
{
    ga_clear(&b->bv_ga);
    vim_free(b);
}

/*
 * Unreference a blob: decrement the reference count and free it when it
 * becomes zero.
 */
    void
blob_unref(blob_T *b)
{
    if (b != NULL && --b->bv_refcount <= 0)
	blob_free(b);
}

/*
 * Get the length of data.
 */
    long
blob_len(blob_T *b)
{
    if (b == NULL)
	return 0L;
    return b->bv_ga.ga_len;
}

/*
 * Get byte "idx" in blob "b".
 * Caller must check that "idx" is valid.
 */
    int
blob_get(blob_T *b, int idx)
{
    return ((char_u*)b->bv_ga.ga_data)[idx];
}

/*
 * Store one byte "byte" in blob "blob" at "idx".
 * Caller must make sure that "idx" is valid.
 */
    void
blob_set(blob_T *blob, int idx, int byte)
{
    ((char_u*)blob->bv_ga.ga_data)[idx] = byte;
}

/*
 * Store one byte "byte" in blob "blob" at "idx".
 * Append one byte if needed.
 */
    void
blob_set_append(blob_T *blob, int idx, int byte)
{
    garray_T *gap = &blob->bv_ga;

    // Allow for appending a byte.  Setting a byte beyond
    // the end is an error otherwise.
    if (idx < gap->ga_len
	    || (idx == gap->ga_len && ga_grow(gap, 1) == OK))
    {
	blob_set(blob, idx, byte);
	if (idx == gap->ga_len)
	    ++gap->ga_len;
    }
}

/*
 * Return TRUE when two blobs have exactly the same values.
 */
    int
blob_equal(
    blob_T	*b1,
    blob_T	*b2)
{
    int	    i;
    int	    len1 = blob_len(b1);
    int	    len2 = blob_len(b2);

    // empty and NULL are considered the same
    if (len1 == 0 && len2 == 0)
	return TRUE;
    if (b1 == b2)
	return TRUE;
    if (len1 != len2)
	return FALSE;

    for (i = 0; i < b1->bv_ga.ga_len; i++)
	if (blob_get(b1, i) != blob_get(b2, i)) return FALSE;
    return TRUE;
}

/*
 * Read blob from file "fd".
 * Caller has allocated a blob in "rettv".
 * Return OK or FAIL.
 */
    int
read_blob(FILE *fd, typval_T *rettv, off_T offset, off_T size_arg)
{
    blob_T	*blob = rettv->vval.v_blob;
    struct stat	st;
    int		whence;
    off_T	size = size_arg;

    if (fstat(fileno(fd), &st) < 0)
	return FAIL;  // can't read the file, error

    if (offset >= 0)
    {
	// The size defaults to the whole file.  If a size is given it is
	// limited to not go past the end of the file.
	if (size == -1 || (size > st.st_size - offset
#ifdef S_ISCHR
		    && !S_ISCHR(st.st_mode)
#endif
		    ))
	    // size may become negative, checked below
	    size = st.st_size - offset;
	whence = SEEK_SET;
    }
    else
    {
	// limit the offset to not go before the start of the file
	if (-offset > st.st_size
#ifdef S_ISCHR
		    && !S_ISCHR(st.st_mode)
#endif
		    )
	    offset = -st.st_size;
	// Size defaults to reading until the end of the file.
	if (size == -1 || size > -offset)
	    size = -offset;
	whence = SEEK_END;
    }
    if (size <= 0)
	return OK;
    if (offset != 0 && vim_fseek(fd, offset, whence) != 0)
	return OK;

    if (ga_grow(&blob->bv_ga, (int)size) == FAIL)
	return FAIL;
    blob->bv_ga.ga_len = (int)size;
    if (fread(blob->bv_ga.ga_data, 1, blob->bv_ga.ga_len, fd)
						  < (size_t)blob->bv_ga.ga_len)
    {
	// An empty blob is returned on error.
	blob_free(rettv->vval.v_blob);
	rettv->vval.v_blob = NULL;
	return FAIL;
    }
    return OK;
}

/*
 * Write "blob" to file "fd".
 * Return OK or FAIL.
 */
    int
write_blob(FILE *fd, blob_T *blob)
{
    if (fwrite(blob->bv_ga.ga_data, 1, blob->bv_ga.ga_len, fd)
						  < (size_t)blob->bv_ga.ga_len)
    {
	emsg(_(e_error_while_writing));
	return FAIL;
    }
    return OK;
}

/*
 * Convert a blob to a readable form: "0z00112233.44556677.8899"
 */
    char_u *
blob2string(blob_T *blob, char_u **tofree, char_u *numbuf)
{
    int		i;
    garray_T    ga;

    if (blob == NULL)
    {
	*tofree = NULL;
	return (char_u *)"0z";
    }

    // Store bytes in the growarray.
    ga_init2(&ga, 1, 4000);
    ga_concat(&ga, (char_u *)"0z");
    for (i = 0; i < blob_len(blob); i++)
    {
	if (i > 0 && (i & 3) == 0)
	    ga_concat(&ga, (char_u *)".");
	vim_snprintf((char *)numbuf, NUMBUFLEN, "%02X", blob_get(blob, i));
	ga_concat(&ga, numbuf);
    }
    ga_append(&ga, NUL);		// append a NUL at the end
    *tofree = ga.ga_data;
    return *tofree;
}

/*
 * Convert a string variable, in the format of blob2string(), to a blob.
 * Return NULL when conversion failed.
 */
    blob_T *
string2blob(char_u *str)
{
    blob_T  *blob = blob_alloc();
    char_u  *s = str;

    if (blob == NULL)
	return NULL;
    if (s[0] != '0' || (s[1] != 'z' && s[1] != 'Z'))
	goto failed;
    s += 2;
    while (vim_isxdigit(*s))
    {
	if (!vim_isxdigit(s[1]))
	    goto failed;
	ga_append(&blob->bv_ga, (hex2nr(s[0]) << 4) + hex2nr(s[1]));
	s += 2;
	if (*s == '.' && vim_isxdigit(s[1]))
	    ++s;
    }
    if (*skipwhite(s) != NUL)
	goto failed;  // text after final digit

    ++blob->bv_refcount;
    return blob;

failed:
    blob_free(blob);
    return NULL;
}

/*
 * Returns a slice of 'blob' from index 'n1' to 'n2' in 'rettv'.  The length of
 * the blob is 'len'.  Returns an empty blob if the indexes are out of range.
 */
    static int
blob_slice(
	blob_T		*blob,
	long		len,
	varnumber_T	n1,
	varnumber_T	n2,
	int		exclusive,
	typval_T	*rettv)
{
    if (n1 < 0)
    {
	n1 = len + n1;
	if (n1 < 0)
	    n1 = 0;
    }
    if (n2 < 0)
	n2 = len + n2;
    else if (n2 >= len)
	n2 = len - (exclusive ? 0 : 1);
    if (exclusive)
	--n2;
    if (n1 >= len || n2 < 0 || n1 > n2)
    {
	clear_tv(rettv);
	rettv->v_type = VAR_BLOB;
	rettv->vval.v_blob = NULL;
    }
    else
    {
	blob_T  *new_blob = blob_alloc();
	long    i;

	if (new_blob != NULL)
	{
	    if (ga_grow(&new_blob->bv_ga, n2 - n1 + 1) == FAIL)
	    {
		blob_free(new_blob);
		return FAIL;
	    }
	    new_blob->bv_ga.ga_len = n2 - n1 + 1;
	    for (i = n1; i <= n2; i++)
		blob_set(new_blob, i - n1, blob_get(blob, i));

	    clear_tv(rettv);
	    rettv_blob_set(rettv, new_blob);
	}
    }

    return OK;
}

/*
 * Return the byte value in 'blob' at index 'idx' in 'rettv'.  If the index is
 * too big or negative that is an error.  The length of the blob is 'len'.
 */
    static int
blob_index(
	blob_T		*blob,
	int		len,
	varnumber_T	idx,
	typval_T	*rettv)
{
    // The resulting variable is a byte value.
    // If the index is too big or negative that is an error.
    if (idx < 0)
	idx = len + idx;
    if (idx < len && idx >= 0)
    {
	int v = blob_get(blob, idx);

	clear_tv(rettv);
	rettv->v_type = VAR_NUMBER;
	rettv->vval.v_number = v;
    }
    else
    {
	semsg(_(e_blob_index_out_of_range_nr), idx);
	return FAIL;
    }

    return OK;
}

    int
blob_slice_or_index(
	blob_T		*blob,
	int		is_range,
	varnumber_T	n1,
	varnumber_T	n2,
	int		exclusive,
	typval_T	*rettv)
{
    long	len = blob_len(blob);

    if (is_range)
	return blob_slice(blob, len, n1, n2, exclusive, rettv);
    else
	return blob_index(blob, len, n1, rettv);
    return OK;
}

/*
 * Check if "n1"- is a valid index for a blobl with length "bloblen".
 */
    int
check_blob_index(long bloblen, varnumber_T n1, int quiet)
{
    if (n1 < 0 || n1 > bloblen)
    {
	if (!quiet)
	    semsg(_(e_blob_index_out_of_range_nr), n1);
	return FAIL;
    }
    return OK;
}

/*
 * Check if "n1"-"n2" is a valid range for a blob with length "bloblen".
 */
    int
check_blob_range(long bloblen, varnumber_T n1, varnumber_T n2, int quiet)
{
    if (n2 < 0 || n2 >= bloblen || n2 < n1)
    {
	if (!quiet)
	    semsg(_(e_blob_index_out_of_range_nr), n2);
	return FAIL;
    }
    return OK;
}

/*
 * Set bytes "n1" to "n2" (inclusive) in "dest" to the value of "src".
 * Caller must make sure "src" is a blob.
 * Returns FAIL if the number of bytes does not match.
 */
    int
blob_set_range(blob_T *dest, long n1, long n2, typval_T *src)
{
    int	il, ir;

    if (n2 - n1 + 1 != blob_len(src->vval.v_blob))
    {
	emsg(_(e_blob_value_does_not_have_right_number_of_bytes));
	return FAIL;
    }

    ir = 0;
    for (il = n1; il <= n2; il++)
	blob_set(dest, il, blob_get(src->vval.v_blob, ir++));
    return OK;
}

/*
 * "add(blob, item)" function
 */
    void
blob_add(typval_T *argvars, typval_T *rettv)
{
    blob_T	*b = argvars[0].vval.v_blob;
    int		error = FALSE;
    varnumber_T n;

    if (b == NULL)
    {
	if (in_vim9script())
	    emsg(_(e_cannot_add_to_null_blob));
	return;
    }

    if (value_check_lock(b->bv_lock, (char_u *)N_("add() argument"), TRUE))
	return;

    n = tv_get_number_chk(&argvars[1], &error);
    if (error)
	return;

    ga_append(&b->bv_ga, (int)n);
    copy_tv(&argvars[0], rettv);
}

/*
 * "remove({blob}, {idx} [, {end}])" function
 */
    void
blob_remove(typval_T *argvars, typval_T *rettv, char_u *arg_errmsg)
{
    blob_T	*b = argvars[0].vval.v_blob;
    blob_T	*newblob;
    int		error = FALSE;
    long	idx;
    long	end;
    int		len;
    char_u	*p;

    if (b != NULL && value_check_lock(b->bv_lock, arg_errmsg, TRUE))
	return;

    idx = (long)tv_get_number_chk(&argvars[1], &error);
    if (error)
	return;

    len = blob_len(b);

    if (idx < 0)
	// count from the end
	idx = len + idx;
    if (idx < 0 || idx >= len)
    {
	semsg(_(e_blob_index_out_of_range_nr), idx);
	return;
    }
    if (argvars[2].v_type == VAR_UNKNOWN)
    {
	// Remove one item, return its value.
	p = (char_u *)b->bv_ga.ga_data;
	rettv->vval.v_number = (varnumber_T) *(p + idx);
	mch_memmove(p + idx, p + idx + 1, (size_t)len - idx - 1);
	--b->bv_ga.ga_len;
	return;
    }

    // Remove range of items, return blob with values.
    end = (long)tv_get_number_chk(&argvars[2], &error);
    if (error)
	return;
    if (end < 0)
	// count from the end
	end = len + end;
    if (end >= len || idx > end)
    {
	semsg(_(e_blob_index_out_of_range_nr), end);
	return;
    }
    newblob = blob_alloc();
    if (newblob == NULL)
	return;
    newblob->bv_ga.ga_len = end - idx + 1;
    if (ga_grow(&newblob->bv_ga, end - idx + 1) == FAIL)
    {
	vim_free(newblob);
	return;
    }
    p = (char_u *)b->bv_ga.ga_data;
    mch_memmove((char_u *)newblob->bv_ga.ga_data, p + idx,
	    (size_t)(end - idx + 1));
    ++newblob->bv_refcount;
    rettv->v_type = VAR_BLOB;
    rettv->vval.v_blob = newblob;

    if (len - end - 1 > 0)
	mch_memmove(p + idx, p + end + 1, (size_t)(len - end - 1));
    b->bv_ga.ga_len -= end - idx + 1;
}

/*
 * Implementation of map() and filter() for a Blob.  Apply "expr" to every
 * number in Blob "blob_arg" and return the result in "rettv".
 */
    void
blob_filter_map(
	blob_T		*blob_arg,
	filtermap_T	filtermap,
	typval_T	*expr,
	char_u		*arg_errmsg,
	typval_T	*rettv)
{
    blob_T	*b = blob_arg;
    int		i;
    typval_T	tv;
    varnumber_T	val;
    blob_T	*b_ret;
    int		idx = 0;
    int		rem;
    typval_T	newtv;
    funccall_T	*fc;

    if (filtermap == FILTERMAP_MAPNEW)
    {
	rettv->v_type = VAR_BLOB;
	rettv->vval.v_blob = NULL;
    }
    if (b == NULL || (filtermap == FILTERMAP_FILTER
			    && value_check_lock(b->bv_lock, arg_errmsg, TRUE)))
	return;

    b_ret = b;
    if (filtermap == FILTERMAP_MAPNEW)
    {
	if (blob_copy(b, rettv) == FAIL)
	    return;
	b_ret = rettv->vval.v_blob;
    }

    // set_vim_var_nr() doesn't set the type
    set_vim_var_type(VV_KEY, VAR_NUMBER);

    int prev_lock = b->bv_lock;
    if (b->bv_lock == 0)
	b->bv_lock = VAR_LOCKED;

    // Create one funccall_T for all eval_expr_typval() calls.
    fc = eval_expr_get_funccal(expr, &newtv);

    for (i = 0; i < b->bv_ga.ga_len; i++)
    {
	tv.v_type = VAR_NUMBER;
	val = blob_get(b, i);
	tv.vval.v_number = val;
	set_vim_var_nr(VV_KEY, idx);
	if (filter_map_one(&tv, expr, filtermap, fc, &newtv, &rem) == FAIL
		|| did_emsg)
	    break;
	if (filtermap != FILTERMAP_FOREACH)
	{
	    if (newtv.v_type != VAR_NUMBER && newtv.v_type != VAR_BOOL)
	    {
		clear_tv(&newtv);
		emsg(_(e_invalid_operation_for_blob));
		break;
	    }
	    if (filtermap != FILTERMAP_FILTER)
	    {
		if (newtv.vval.v_number != val)
		    blob_set(b_ret, i, newtv.vval.v_number);
	    }
	    else if (rem)
	    {
		char_u *p = (char_u *)blob_arg->bv_ga.ga_data;

		mch_memmove(p + i, p + i + 1,
			    (size_t)b->bv_ga.ga_len - i - 1);
		--b->bv_ga.ga_len;
		--i;
	    }
	}
	++idx;
    }

    b->bv_lock = prev_lock;
    if (fc != NULL)
	remove_funccal();
}

/*
 * "insert(blob, {item} [, {idx}])" function
 */
    void
blob_insert_func(typval_T *argvars, typval_T *rettv)
{
    blob_T	*b = argvars[0].vval.v_blob;
    long	before = 0;
    int		error = FALSE;
    int		val, len;
    char_u	*p;

    if (b == NULL)
    {
	if (in_vim9script())
	    emsg(_(e_cannot_add_to_null_blob));
	return;
    }

    if (value_check_lock(b->bv_lock, (char_u *)N_("insert() argument"), TRUE))
	return;

    len = blob_len(b);
    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	before = (long)tv_get_number_chk(&argvars[2], &error);
	if (error)
	    return;		// type error; errmsg already given
	if (before < 0 || before > len)
	{
	    semsg(_(e_invalid_argument_str), tv_get_string(&argvars[2]));
	    return;
	}
    }
    val = tv_get_number_chk(&argvars[1], &error);
    if (error)
	return;
    if (val < 0 || val > 255)
    {
	semsg(_(e_invalid_argument_str), tv_get_string(&argvars[1]));
	return;
    }

    if (ga_grow(&b->bv_ga, 1) == FAIL)
	return;
    p = (char_u *)b->bv_ga.ga_data;
    mch_memmove(p + before + 1, p + before, (size_t)len - before);
    *(p + before) = val;
    ++b->bv_ga.ga_len;

    copy_tv(&argvars[0], rettv);
}

/*
 * Implementation of reduce() for Blob "argvars[0]" using the function "expr"
 * starting with the optional initial value "argvars[2]" and return the result
 * in "rettv".
 */
    void
blob_reduce(
	typval_T	*argvars,
	typval_T	*expr,
	typval_T	*rettv)
{
    blob_T	*b = argvars[0].vval.v_blob;
    int		called_emsg_start = called_emsg;
    int		r;
    typval_T	initial;
    typval_T	argv[3];
    int	i;

    if (argvars[2].v_type == VAR_UNKNOWN)
    {
	if (b == NULL || b->bv_ga.ga_len == 0)
	{
	    semsg(_(e_reduce_of_an_empty_str_with_no_initial_value), "Blob");
	    return;
	}
	initial.v_type = VAR_NUMBER;
	initial.vval.v_number = blob_get(b, 0);
	i = 1;
    }
    else if (check_for_number_arg(argvars, 2) == FAIL)
	return;
    else
    {
	initial = argvars[2];
	i = 0;
    }

    copy_tv(&initial, rettv);
    if (b == NULL)
	return;

    for ( ; i < b->bv_ga.ga_len; i++)
    {
	argv[0] = *rettv;
	argv[1].v_type = VAR_NUMBER;
	argv[1].vval.v_number = blob_get(b, i);

	r = eval_expr_typval(expr, TRUE, argv, 2, NULL, rettv);

	clear_tv(&argv[0]);
	if (r == FAIL || called_emsg != called_emsg_start)
	    return;
    }
}

/*
 * "reverse({blob})" function
 */
    void
blob_reverse(blob_T *b, typval_T *rettv)
{
    int	i, len = blob_len(b);

    for (i = 0; i < len / 2; i++)
    {
	int tmp = blob_get(b, i);

	blob_set(b, i, blob_get(b, len - i - 1));
	blob_set(b, len - i - 1, tmp);
    }
    rettv_blob_set(rettv, b);
}

/*
 * blob2list() function
 */
    void
f_blob2list(typval_T *argvars, typval_T *rettv)
{
    blob_T	*blob;
    list_T	*l;
    int		i;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (check_for_blob_arg(argvars, 0) == FAIL)
	return;

    blob = argvars->vval.v_blob;
    l = rettv->vval.v_list;
    for (i = 0; i < blob_len(blob); i++)
	list_append_number(l, blob_get(blob, i));
}

/*
 * list2blob() function
 */
    void
f_list2blob(typval_T *argvars, typval_T *rettv)
{
    list_T	*l;
    listitem_T	*li;
    blob_T	*blob;

    if (rettv_blob_alloc(rettv) == FAIL)
	return;
    blob = rettv->vval.v_blob;

    if (check_for_list_arg(argvars, 0) == FAIL)
	return;

    l = argvars->vval.v_list;
    if (l == NULL)
	return;

    CHECK_LIST_MATERIALIZE(l);
    FOR_ALL_LIST_ITEMS(l, li)
    {
	int		error;
	varnumber_T	n;

	error = FALSE;
	n = tv_get_number_chk(&li->li_tv, &error);
	if (error == TRUE || n < 0 || n > 255)
	{
	    if (!error)
		semsg(_(e_invalid_value_for_blob_nr), n);
	    ga_clear(&blob->bv_ga);
	    return;
	}
	ga_append(&blob->bv_ga, n);
    }
}

#endif // defined(FEAT_EVAL)
