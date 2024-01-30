/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * list.c: List support and container (List, Dict, Blob) functions.
 */

#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

// List heads for garbage collection.
static list_T		*first_list = NULL;	// list of all lists

#define FOR_ALL_WATCHERS(l, lw) \
    for ((lw) = (l)->lv_watch; (lw) != NULL; (lw) = (lw)->lw_next)

static void list_free_item(list_T *l, listitem_T *item);

/*
 * Add a watcher to a list.
 */
    void
list_add_watch(list_T *l, listwatch_T *lw)
{
    lw->lw_next = l->lv_watch;
    l->lv_watch = lw;
}

/*
 * Remove a watcher from a list.
 * No warning when it isn't found...
 */
    void
list_rem_watch(list_T *l, listwatch_T *lwrem)
{
    listwatch_T	*lw, **lwp;

    lwp = &l->lv_watch;
    FOR_ALL_WATCHERS(l, lw)
    {
	if (lw == lwrem)
	{
	    *lwp = lw->lw_next;
	    break;
	}
	lwp = &lw->lw_next;
    }
}

/*
 * Just before removing an item from a list: advance watchers to the next
 * item.
 */
    static void
list_fix_watch(list_T *l, listitem_T *item)
{
    listwatch_T	*lw;

    FOR_ALL_WATCHERS(l, lw)
	if (lw->lw_item == item)
	    lw->lw_item = item->li_next;
}

    static void
list_init(list_T *l)
{
    // Prepend the list to the list of lists for garbage collection.
    if (first_list != NULL)
	first_list->lv_used_prev = l;
    l->lv_used_prev = NULL;
    l->lv_used_next = first_list;
    first_list = l;
}

/*
 * Allocate an empty header for a list.
 * Caller should take care of the reference count.
 */
    list_T *
list_alloc(void)
{
    list_T  *l;

    l = ALLOC_CLEAR_ONE(list_T);
    if (l != NULL)
	list_init(l);
    return l;
}

/*
 * list_alloc() with an ID for alloc_fail().
 */
    list_T *
list_alloc_id(alloc_id_T id UNUSED)
{
#ifdef FEAT_EVAL
    if (alloc_fail_id == id && alloc_does_fail(sizeof(list_T)))
	return NULL;
#endif
    return (list_alloc());
}

/*
 * Allocate space for a list, plus "count" items.
 * This uses one allocation for efficiency.
 * The reference count is not set.
 * Next list_set_item() must be called for each item.
 */
    list_T *
list_alloc_with_items(int count)
{
    list_T	*l;

    l = (list_T *)alloc_clear(sizeof(list_T) + count * sizeof(listitem_T));
    if (l == NULL)
	return NULL;

    list_init(l);

    if (count <= 0)
	return l;

    listitem_T	*li = (listitem_T *)(l + 1);
    int		i;

    l->lv_len = count;
    l->lv_with_items = count;
    l->lv_first = li;
    l->lv_u.mat.lv_last = li + count - 1;
    for (i = 0; i < count; ++i)
    {
	if (i == 0)
	    li->li_prev = NULL;
	else
	    li->li_prev = li - 1;
	if (i == count - 1)
	    li->li_next = NULL;
	else
	    li->li_next = li + 1;
	++li;
    }

    return l;
}

/*
 * Set item "idx" for a list previously allocated with list_alloc_with_items().
 * The contents of "tv" is moved into the list item.
 * Each item must be set exactly once.
 */
    void
list_set_item(list_T *l, int idx, typval_T *tv)
{
    listitem_T	*li = (listitem_T *)(l + 1) + idx;

    li->li_tv = *tv;
}

/*
 * Allocate an empty list for a return value, with reference count set.
 * Returns OK or FAIL.
 */
    int
rettv_list_alloc(typval_T *rettv)
{
    list_T	*l = list_alloc();

    if (l == NULL)
	return FAIL;

    rettv->v_lock = 0;
    rettv_list_set(rettv, l);
    return OK;
}

/*
 * Same as rettv_list_alloc() but uses an allocation id for testing.
 */
    int
rettv_list_alloc_id(typval_T *rettv, alloc_id_T id UNUSED)
{
#ifdef FEAT_EVAL
    if (alloc_fail_id == id && alloc_does_fail(sizeof(list_T)))
	return FAIL;
#endif
    return rettv_list_alloc(rettv);
}


/*
 * Set a list as the return value.  Increments the reference count.
 */
    void
rettv_list_set(typval_T *rettv, list_T *l)
{
    rettv->v_type = VAR_LIST;
    rettv->vval.v_list = l;
    if (l != NULL)
	++l->lv_refcount;
}

/*
 * Unreference a list: decrement the reference count and free it when it
 * becomes zero.
 */
    void
list_unref(list_T *l)
{
    if (l != NULL && --l->lv_refcount <= 0)
	list_free(l);
}

/*
 * Free a list, including all non-container items it points to.
 * Ignores the reference count.
 */
    static void
list_free_contents(list_T *l)
{
    listitem_T *item;

    if (l->lv_first != &range_list_item)
	for (item = l->lv_first; item != NULL; item = l->lv_first)
	{
	    // Remove the item before deleting it.
	    l->lv_first = item->li_next;
	    clear_tv(&item->li_tv);
	    list_free_item(l, item);
	}
}

/*
 * Go through the list of lists and free items without the copyID.
 * But don't free a list that has a watcher (used in a for loop), these
 * are not referenced anywhere.
 */
    int
list_free_nonref(int copyID)
{
    list_T	*ll;
    int		did_free = FALSE;

    for (ll = first_list; ll != NULL; ll = ll->lv_used_next)
	if ((ll->lv_copyID & COPYID_MASK) != (copyID & COPYID_MASK)
						      && ll->lv_watch == NULL)
	{
	    // Free the List and ordinary items it contains, but don't recurse
	    // into Lists and Dictionaries, they will be in the list of dicts
	    // or list of lists.
	    list_free_contents(ll);
	    did_free = TRUE;
	}
    return did_free;
}

    static void
list_free_list(list_T  *l)
{
    // Remove the list from the list of lists for garbage collection.
    if (l->lv_used_prev == NULL)
	first_list = l->lv_used_next;
    else
	l->lv_used_prev->lv_used_next = l->lv_used_next;
    if (l->lv_used_next != NULL)
	l->lv_used_next->lv_used_prev = l->lv_used_prev;

    free_type(l->lv_type);
    vim_free(l);
}

    void
list_free_items(int copyID)
{
    list_T	*ll, *ll_next;

    for (ll = first_list; ll != NULL; ll = ll_next)
    {
	ll_next = ll->lv_used_next;
	if ((ll->lv_copyID & COPYID_MASK) != (copyID & COPYID_MASK)
						      && ll->lv_watch == NULL)
	{
	    // Free the List and ordinary items it contains, but don't recurse
	    // into Lists and Dictionaries, they will be in the list of dicts
	    // or list of lists.
	    list_free_list(ll);
	}
    }
}

    void
list_free(list_T *l)
{
    if (in_free_unref_items)
	return;

    list_free_contents(l);
    list_free_list(l);
}

/*
 * Allocate a list item.
 * It is not initialized, don't forget to set v_lock.
 */
    listitem_T *
listitem_alloc(void)
{
    return ALLOC_ONE(listitem_T);
}

/*
 * Free a list item, unless it was allocated together with the list itself.
 * Does not clear the value.  Does not notify watchers.
 */
    static void
list_free_item(list_T *l, listitem_T *item)
{
    if (l->lv_with_items == 0 || item < (listitem_T *)l
			   || item >= (listitem_T *)(l + 1) + l->lv_with_items)
	vim_free(item);
}

/*
 * Free a list item, unless it was allocated together with the list itself.
 * Also clears the value.  Does not notify watchers.
 */
    void
listitem_free(list_T *l, listitem_T *item)
{
    clear_tv(&item->li_tv);
    list_free_item(l, item);
}

/*
 * Remove a list item from a List and free it.  Also clears the value.
 */
    void
listitem_remove(list_T *l, listitem_T *item)
{
    vimlist_remove(l, item, item);
    listitem_free(l, item);
}

/*
 * Get the number of items in a list.
 */
    long
list_len(list_T *l)
{
    if (l == NULL)
	return 0L;
    return l->lv_len;
}

/*
 * Return TRUE when two lists have exactly the same values.
 */
    int
list_equal(
    list_T	*l1,
    list_T	*l2,
    int		ic,	// ignore case for strings
    int		recursive)  // TRUE when used recursively
{
    listitem_T	*item1, *item2;

    if (l1 == l2)
	return TRUE;
    if (list_len(l1) != list_len(l2))
	return FALSE;
    if (list_len(l1) == 0)
	// empty and NULL list are considered equal
	return TRUE;
    if (l1 == NULL || l2 == NULL)
	return FALSE;

    CHECK_LIST_MATERIALIZE(l1);
    CHECK_LIST_MATERIALIZE(l2);

    for (item1 = l1->lv_first, item2 = l2->lv_first;
	    item1 != NULL && item2 != NULL;
			       item1 = item1->li_next, item2 = item2->li_next)
	if (!tv_equal(&item1->li_tv, &item2->li_tv, ic, recursive))
	    return FALSE;
    return item1 == NULL && item2 == NULL;
}

/*
 * Locate item with index "n" in list "l" and return it.
 * A negative index is counted from the end; -1 is the last item.
 * Returns NULL when "n" is out of range.
 */
    listitem_T *
list_find(list_T *l, long n)
{
    listitem_T	*item;
    long	idx;

    if (l == NULL)
	return NULL;

    // Negative index is relative to the end.
    if (n < 0)
	n = l->lv_len + n;

    // Check for index out of range.
    if (n < 0 || n >= l->lv_len)
	return NULL;

    CHECK_LIST_MATERIALIZE(l);

    // range_list_materialize may reset l->lv_len
    if (n >= l->lv_len)
	return NULL;

    // When there is a cached index may start search from there.
    if (l->lv_u.mat.lv_idx_item != NULL)
    {
	if (n < l->lv_u.mat.lv_idx / 2)
	{
	    // closest to the start of the list
	    item = l->lv_first;
	    idx = 0;
	}
	else if (n > (l->lv_u.mat.lv_idx + l->lv_len) / 2)
	{
	    // closest to the end of the list
	    item = l->lv_u.mat.lv_last;
	    idx = l->lv_len - 1;
	}
	else
	{
	    // closest to the cached index
	    item = l->lv_u.mat.lv_idx_item;
	    idx = l->lv_u.mat.lv_idx;
	}
    }
    else
    {
	if (n < l->lv_len / 2)
	{
	    // closest to the start of the list
	    item = l->lv_first;
	    idx = 0;
	}
	else
	{
	    // closest to the end of the list
	    item = l->lv_u.mat.lv_last;
	    idx = l->lv_len - 1;
	}
    }

    while (n > idx)
    {
	// search forward
	item = item->li_next;
	++idx;
    }
    while (n < idx)
    {
	// search backward
	item = item->li_prev;
	--idx;
    }

    // cache the used index
    l->lv_u.mat.lv_idx = idx;
    l->lv_u.mat.lv_idx_item = item;

    return item;
}

/*
 * Get list item "l[idx]" as a number.
 */
    long
list_find_nr(
    list_T	*l,
    long	idx,
    int		*errorp)	// set to TRUE when something wrong
{
    listitem_T	*li;

    if (l != NULL && l->lv_first == &range_list_item)
    {
	long	    n = idx;

	// not materialized range() list: compute the value.
	// Negative index is relative to the end.
	if (n < 0)
	    n = l->lv_len + n;

	// Check for index out of range.
	if (n < 0 || n >= l->lv_len)
	{
	    if (errorp != NULL)
		*errorp = TRUE;
	    return -1L;
	}

	return l->lv_u.nonmat.lv_start + n * l->lv_u.nonmat.lv_stride;
    }

    li = list_find(l, idx);
    if (li == NULL)
    {
	if (errorp != NULL)
	    *errorp = TRUE;
	return -1L;
    }
    return (long)tv_get_number_chk(&li->li_tv, errorp);
}

/*
 * Get list item "l[idx - 1]" as a string.  Returns NULL for failure.
 */
    char_u *
list_find_str(list_T *l, long idx)
{
    listitem_T	*li;

    li = list_find(l, idx - 1);
    if (li == NULL)
    {
	semsg(_(e_list_index_out_of_range_nr), idx);
	return NULL;
    }
    return tv_get_string(&li->li_tv);
}

/*
 * Like list_find() but when a negative index is used that is not found use
 * zero and set "idx" to zero.  Used for first index of a range.
 */
    listitem_T *
list_find_index(list_T *l, long *idx)
{
    listitem_T *li = list_find(l, *idx);

    if (li != NULL)
	return li;

    if (*idx < 0)
    {
	*idx = 0;
	li = list_find(l, *idx);
    }
    return li;
}

/*
 * Locate "item" list "l" and return its index.
 * Returns -1 when "item" is not in the list.
 */
    long
list_idx_of_item(list_T *l, listitem_T *item)
{
    long	idx = 0;
    listitem_T	*li;

    if (l == NULL)
	return -1;
    CHECK_LIST_MATERIALIZE(l);
    idx = 0;
    for (li = l->lv_first; li != NULL && li != item; li = li->li_next)
	++idx;
    if (li == NULL)
	return -1;
    return idx;
}

/*
 * Append item "item" to the end of list "l".
 */
    void
list_append(list_T *l, listitem_T *item)
{
    CHECK_LIST_MATERIALIZE(l);
    if (l->lv_u.mat.lv_last == NULL)
    {
	// empty list
	l->lv_first = item;
	item->li_prev = NULL;
    }
    else
    {
	l->lv_u.mat.lv_last->li_next = item;
	item->li_prev = l->lv_u.mat.lv_last;
    }
    l->lv_u.mat.lv_last = item;
    ++l->lv_len;
    item->li_next = NULL;
}

/*
 * Append typval_T "tv" to the end of list "l".  "tv" is copied.
 * Return FAIL when out of memory or the type is wrong.
 */
    int
list_append_tv(list_T *l, typval_T *tv)
{
    listitem_T	*li;

    if (l->lv_type != NULL && l->lv_type->tt_member != NULL
		&& check_typval_arg_type(l->lv_type->tt_member, tv,
							      NULL, 0) == FAIL)
	return FAIL;
    li = listitem_alloc();
    if (li == NULL)
	return FAIL;
    copy_tv(tv, &li->li_tv);
    list_append(l, li);
    return OK;
}

/*
 * As list_append_tv() but move the value instead of copying it.
 * Return FAIL when out of memory.
 */
    static int
list_append_tv_move(list_T *l, typval_T *tv)
{
    listitem_T	*li = listitem_alloc();

    if (li == NULL)
	return FAIL;
    li->li_tv = *tv;
    list_append(l, li);
    return OK;
}

/*
 * Add a dictionary to a list.  Used by getqflist().
 * Return FAIL when out of memory.
 */
    int
list_append_dict(list_T *list, dict_T *dict)
{
    listitem_T	*li = listitem_alloc();

    if (li == NULL)
	return FAIL;
    li->li_tv.v_type = VAR_DICT;
    li->li_tv.v_lock = 0;
    li->li_tv.vval.v_dict = dict;
    list_append(list, li);
    ++dict->dv_refcount;
    return OK;
}

/*
 * Append list2 to list1.
 * Return FAIL when out of memory.
 */
    int
list_append_list(list_T *list1, list_T *list2)
{
    listitem_T	*li = listitem_alloc();

    if (li == NULL)
	return FAIL;
    li->li_tv.v_type = VAR_LIST;
    li->li_tv.v_lock = 0;
    li->li_tv.vval.v_list = list2;
    list_append(list1, li);
    ++list2->lv_refcount;
    return OK;
}

/*
 * Make a copy of "str" and append it as an item to list "l".
 * When "len" >= 0 use "str[len]".
 * Returns FAIL when out of memory.
 */
    int
list_append_string(list_T *l, char_u *str, int len)
{
    listitem_T *li = listitem_alloc();

    if (li == NULL)
	return FAIL;
    list_append(l, li);
    li->li_tv.v_type = VAR_STRING;
    li->li_tv.v_lock = 0;
    if (str == NULL)
	li->li_tv.vval.v_string = NULL;
    else if ((li->li_tv.vval.v_string = (len >= 0 ? vim_strnsave(str, len)
						 : vim_strsave(str))) == NULL)
	return FAIL;
    return OK;
}

/*
 * Append "n" to list "l".
 * Returns FAIL when out of memory.
 */
    int
list_append_number(list_T *l, varnumber_T n)
{
    listitem_T	*li;

    li = listitem_alloc();
    if (li == NULL)
	return FAIL;
    li->li_tv.v_type = VAR_NUMBER;
    li->li_tv.v_lock = 0;
    li->li_tv.vval.v_number = n;
    list_append(l, li);
    return OK;
}

/*
 * Insert typval_T "tv" in list "l" before "item".
 * If "item" is NULL append at the end.
 * Return FAIL when out of memory or the type is wrong.
 */
    int
list_insert_tv(list_T *l, typval_T *tv, listitem_T *item)
{
    listitem_T	*ni;

    if (l->lv_type != NULL && l->lv_type->tt_member != NULL
		&& check_typval_arg_type(l->lv_type->tt_member, tv,
							      NULL, 0) == FAIL)
	return FAIL;
    ni = listitem_alloc();
    if (ni == NULL)
	return FAIL;
    copy_tv(tv, &ni->li_tv);
    list_insert(l, ni, item);
    return OK;
}

    void
list_insert(list_T *l, listitem_T *ni, listitem_T *item)
{
    CHECK_LIST_MATERIALIZE(l);
    if (item == NULL)
	// Append new item at end of list.
	list_append(l, ni);
    else
    {
	// Insert new item before existing item.
	ni->li_prev = item->li_prev;
	ni->li_next = item;
	if (item->li_prev == NULL)
	{
	    l->lv_first = ni;
	    ++l->lv_u.mat.lv_idx;
	}
	else
	{
	    item->li_prev->li_next = ni;
	    l->lv_u.mat.lv_idx_item = NULL;
	}
	item->li_prev = ni;
	++l->lv_len;
    }
}

/*
 * Get the list item in "l" with index "n1".  "n1" is adjusted if needed.
 * In Vim9, it is at the end of the list, add an item if "can_append" is TRUE.
 * Return NULL if there is no such item.
 */
    listitem_T *
check_range_index_one(list_T *l, long *n1, int can_append, int quiet)
{
    long	orig_n1 = *n1;
    listitem_T	*li = list_find_index(l, n1);

    if (li != NULL)
	return li;

    // Vim9: Allow for adding an item at the end.
    if (can_append && in_vim9script()
	    && *n1 == l->lv_len && l->lv_lock == 0)
    {
	list_append_number(l, 0);
	li = list_find_index(l, n1);
    }
    if (li == NULL)
    {
	if (!quiet)
	    semsg(_(e_list_index_out_of_range_nr), orig_n1);
	return NULL;
    }
    return li;
}

/*
 * Check that "n2" can be used as the second index in a range of list "l".
 * If "n1" or "n2" is negative it is changed to the positive index.
 * "li1" is the item for item "n1".
 * Return OK or FAIL.
 */
    int
check_range_index_two(
	list_T	    *l,
	long	    *n1,
	listitem_T  *li1,
	long	    *n2,
	int	    quiet)
{
    if (*n2 < 0)
    {
	listitem_T	*ni = list_find(l, *n2);

	if (ni == NULL)
	{
	    if (!quiet)
		semsg(_(e_list_index_out_of_range_nr), *n2);
	    return FAIL;
	}
	*n2 = list_idx_of_item(l, ni);
    }

    // Check that n2 isn't before n1.
    if (*n1 < 0)
	*n1 = list_idx_of_item(l, li1);
    if (*n2 < *n1)
    {
	if (!quiet)
	    semsg(_(e_list_index_out_of_range_nr), *n2);
	return FAIL;
    }
    return OK;
}

/*
 * Assign values from list "src" into a range of "dest".
 * "idx1_arg" is the index of the first item in "dest" to be replaced.
 * "idx2" is the index of last item to be replaced, but when "empty_idx2" is
 * TRUE then replace all items after "idx1".
 * "op" is the operator, normally "=" but can be "+=" and the like.
 * "varname" is used for error messages.
 * Returns OK or FAIL.
 */
    int
list_assign_range(
	list_T	    *dest,
	list_T	    *src,
	long	    idx1_arg,
	long	    idx2,
	int	    empty_idx2,
	char_u	    *op,
	char_u	    *varname)
{
    listitem_T	*src_li;
    listitem_T	*dest_li;
    long	idx1 = idx1_arg;
    listitem_T	*first_li = list_find_index(dest, &idx1);
    long	idx;
    type_T	*member_type = NULL;

    // Check whether any of the list items is locked before making any changes.
    idx = idx1;
    dest_li = first_li;
    for (src_li = src->lv_first; src_li != NULL && dest_li != NULL; )
    {
	if (value_check_lock(dest_li->li_tv.v_lock, varname, FALSE))
	    return FAIL;
	src_li = src_li->li_next;
	if (src_li == NULL || (!empty_idx2 && idx2 == idx))
	    break;
	dest_li = dest_li->li_next;
	++idx;
    }

    if (in_vim9script() && dest->lv_type != NULL
					   && dest->lv_type->tt_member != NULL)
	member_type = dest->lv_type->tt_member;

    // Assign the List values to the list items.
    idx = idx1;
    dest_li = first_li;
    for (src_li = src->lv_first; src_li != NULL; )
    {
	if (op != NULL && *op != '=')
	    tv_op(&dest_li->li_tv, &src_li->li_tv, op);
	else
	{
	    if (member_type != NULL
		    && check_typval_arg_type(member_type, &src_li->li_tv,
							      NULL, 0) == FAIL)
		return FAIL;
	    clear_tv(&dest_li->li_tv);
	    copy_tv(&src_li->li_tv, &dest_li->li_tv);
	}
	src_li = src_li->li_next;
	if (src_li == NULL || (!empty_idx2 && idx2 == idx))
	    break;
	if (dest_li->li_next == NULL)
	{
	    // Need to add an empty item.
	    if (list_append_number(dest, 0) == FAIL)
	    {
		src_li = NULL;
		break;
	    }
	}
	dest_li = dest_li->li_next;
	++idx;
    }
    if (src_li != NULL)
    {
	emsg(_(e_list_value_has_more_items_than_targets));
	return FAIL;
    }
    if (empty_idx2
	    ? (dest_li != NULL && dest_li->li_next != NULL)
	    : idx != idx2)
    {
	emsg(_(e_list_value_does_not_have_enough_items));
	return FAIL;
    }
    return OK;
}

/*
 * Flatten up to "maxitems" in "list", starting at "first" to depth "maxdepth".
 * When "first" is NULL use the first item.
 * It does nothing if "maxdepth" is 0.
 * Returns FAIL when out of memory.
 */
    static void
list_flatten(list_T *list, listitem_T *first, long maxitems, long maxdepth)
{
    listitem_T	*item;
    int		done = 0;

    if (maxdepth == 0)
	return;
    CHECK_LIST_MATERIALIZE(list);
    if (first == NULL)
	item = list->lv_first;
    else
	item = first;

    while (item != NULL && done < maxitems)
    {
	listitem_T	*next = item->li_next;

	fast_breakcheck();
	if (got_int)
	    return;

	if (item->li_tv.v_type == VAR_LIST)
	{
	    list_T	*itemlist = item->li_tv.vval.v_list;

	    vimlist_remove(list, item, item);
	    if (list_extend(list, itemlist, next) == FAIL)
	    {
		list_free_item(list, item);
		return;
	    }

	    if (maxdepth > 0)
		list_flatten(list, item->li_prev == NULL
				     ? list->lv_first : item->li_prev->li_next,
				itemlist->lv_len, maxdepth - 1);
	    clear_tv(&item->li_tv);
	    list_free_item(list, item);
	}

	++done;
	item = next;
    }
}

/*
 * "flatten()" and "flattennew()" functions
 */
    static void
flatten_common(typval_T *argvars, typval_T *rettv, int make_copy)
{
    list_T  *l;
    long    maxdepth;
    int	    error = FALSE;

    if (in_vim9script()
	    && (check_for_list_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL))
	return;

    if (argvars[0].v_type != VAR_LIST)
    {
	semsg(_(e_argument_of_str_must_be_list), "flatten()");
	return;
    }

    if (argvars[1].v_type == VAR_UNKNOWN)
	maxdepth = 999999;
    else
    {
	maxdepth = (long)tv_get_number_chk(&argvars[1], &error);
	if (error)
	    return;
	if (maxdepth < 0)
	{
	    emsg(_(e_maxdepth_must_be_non_negative_number));
	    return;
	}
    }

    l = argvars[0].vval.v_list;
    rettv->v_type = VAR_LIST;
    rettv->vval.v_list = l;
    if (l == NULL)
	return;

    if (make_copy)
    {
	l = list_copy(l, FALSE, TRUE, get_copyID());
	rettv->vval.v_list = l;
	if (l == NULL)
	    return;
	// The type will change.
	free_type(l->lv_type);
	l->lv_type = NULL;
    }
    else
    {
	if (value_check_lock(l->lv_lock,
				     (char_u *)N_("flatten() argument"), TRUE))
	    return;
	++l->lv_refcount;
    }

    list_flatten(l, NULL, l->lv_len, maxdepth);
}

/*
 * "flatten(list[, {maxdepth}])" function
 */
    void
f_flatten(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script())
	emsg(_(e_cannot_use_flatten_in_vim9_script));
    else
	flatten_common(argvars, rettv, FALSE);
}

/*
 * "flattennew(list[, {maxdepth}])" function
 */
    void
f_flattennew(typval_T *argvars, typval_T *rettv)
{
    flatten_common(argvars, rettv, TRUE);
}

/*
 * "items(list)" function
 * Caller must have already checked that argvars[0] is a List.
 */
    void
list2items(typval_T *argvars, typval_T *rettv)
{
    list_T	*l = argvars[0].vval.v_list;
    listitem_T	*li;
    varnumber_T	idx;

    if (rettv_list_alloc(rettv) == FAIL)
	return;
    if (l == NULL)
	return;  // null list behaves like an empty list

    // TODO: would be more efficient to not materialize the argument
    CHECK_LIST_MATERIALIZE(l);
    for (idx = 0, li = l->lv_first; li != NULL; li = li->li_next, ++idx)
    {
	list_T	    *l2 = list_alloc();

	if (l2 == NULL)
	    break;
	if (list_append_list(rettv->vval.v_list, l2) == FAIL)
	{
	    vim_free(l2);
	    break;
	}
	if (list_append_number(l2, idx) == FAIL
		|| list_append_tv(l2, &li->li_tv) == FAIL)
	    break;
    }
}

/*
 * "items(string)" function
 * Caller must have already checked that argvars[0] is a String.
 */
    void
string2items(typval_T *argvars, typval_T *rettv)
{
    char_u	*p = argvars[0].vval.v_string;
    varnumber_T idx;

    if (rettv_list_alloc(rettv) == FAIL)
	return;
    if (p == NULL)
	return;  // null string behaves like an empty string

    for (idx = 0; *p != NUL; ++idx)
    {
	int	    len = mb_ptr2len(p);
	list_T	    *l2;

	if (len == 0)
	    break;
	l2 = list_alloc();
	if (l2 == NULL)
	    break;
	if (list_append_list(rettv->vval.v_list, l2) == FAIL)
	{
	    vim_free(l2);
	    break;
	}
	if (list_append_number(l2, idx) == FAIL
		|| list_append_string(l2, p, len) == FAIL)
	    break;
	p += len;
    }
}

/*
 * Extend "l1" with "l2".  "l1" must not be NULL.
 * If "bef" is NULL append at the end, otherwise insert before this item.
 * Returns FAIL when out of memory.
 */
    int
list_extend(list_T *l1, list_T *l2, listitem_T *bef)
{
    listitem_T	*item;
    int		todo;
    listitem_T	*bef_prev;

    // NULL list is equivalent to an empty list: nothing to do.
    if (l2 == NULL || l2->lv_len == 0)
	return OK;

    todo = l2->lv_len;
    CHECK_LIST_MATERIALIZE(l1);
    CHECK_LIST_MATERIALIZE(l2);

    // When exending a list with itself, at some point we run into the item
    // that was before "bef" and need to skip over the already inserted items
    // to "bef".
    bef_prev = bef == NULL ? NULL : bef->li_prev;

    // We also quit the loop when we have inserted the original item count of
    // the list, avoid a hang when we extend a list with itself.
    for (item = l2->lv_first; item != NULL && --todo >= 0;
				 item = item == bef_prev ? bef : item->li_next)
	if (list_insert_tv(l1, &item->li_tv, bef) == FAIL)
	    return FAIL;
    return OK;
}

/*
 * Concatenate lists "l1" and "l2" into a new list, stored in "tv".
 * Return FAIL when out of memory.
 */
    int
list_concat(list_T *l1, list_T *l2, typval_T *tv)
{
    list_T	*l;

    // make a copy of the first list.
    if (l1 == NULL)
	l = list_alloc();
    else
	l = list_copy(l1, FALSE, TRUE, 0);
    if (l == NULL)
	return FAIL;
    tv->v_type = VAR_LIST;
    tv->v_lock = 0;
    tv->vval.v_list = l;
    if (l1 == NULL)
	++l->lv_refcount;

    // append all items from the second list
    return list_extend(l, l2, NULL);
}

    list_T *
list_slice(list_T *ol, long n1, long n2)
{
    listitem_T	*item;
    list_T	*l = list_alloc();

    if (l == NULL)
	return NULL;
    for (item = list_find(ol, n1); n1 <= n2; ++n1)
    {
	if (list_append_tv(l, &item->li_tv) == FAIL)
	{
	    list_free(l);
	    return NULL;
	}
	item = item->li_next;
    }
    return l;
}

    int
list_slice_or_index(
	    list_T	*list,
	    int		range,
	    varnumber_T	n1_arg,
	    varnumber_T	n2_arg,
	    int		exclusive,
	    typval_T	*rettv,
	    int		verbose)
{
    long	len = list_len(list);
    varnumber_T	n1 = n1_arg;
    varnumber_T	n2 = n2_arg;
    typval_T	var1;

    if (n1 < 0)
	n1 = len + n1;
    if (n1 < 0 || n1 >= len)
    {
	// For a range we allow invalid values and for legacy script return an
	// empty list, for Vim9 script start at the first item.
	// A list index out of range is an error.
	if (!range)
	{
	    if (verbose)
		semsg(_(e_list_index_out_of_range_nr), (long)n1_arg);
	    return FAIL;
	}
	if (in_vim9script())
	    n1 = n1 < 0 ? 0 : len;
	else
	    n1 = len;
    }
    if (range)
    {
	list_T	*l;

	if (n2 < 0)
	    n2 = len + n2;
	else if (n2 >= len)
	    n2 = len - (exclusive ? 0 : 1);
	if (exclusive)
	    --n2;
	if (n2 < 0 || n2 + 1 < n1)
	    n2 = -1;
	l = list_slice(list, n1, n2);
	if (l == NULL)
	    return FAIL;
	clear_tv(rettv);
	rettv_list_set(rettv, l);
    }
    else
    {
	// copy the item to "var1" to avoid that freeing the list makes it
	// invalid.
	copy_tv(&list_find(list, n1)->li_tv, &var1);
	clear_tv(rettv);
	*rettv = var1;
    }
    return OK;
}

/*
 * Make a copy of list "orig".  Shallow if "deep" is FALSE.
 * The refcount of the new list is set to 1.
 * See item_copy() for "top" and "copyID".
 * Returns NULL when out of memory.
 */
    list_T *
list_copy(list_T *orig, int deep, int top, int copyID)
{
    list_T	*copy;
    listitem_T	*item;
    listitem_T	*ni;

    if (orig == NULL)
	return NULL;

    copy = list_alloc();
    if (copy == NULL)
	return NULL;

    if (orig->lv_type == NULL || top || deep)
	copy->lv_type = NULL;
    else
	copy->lv_type = alloc_type(orig->lv_type);
    if (copyID != 0)
    {
	// Do this before adding the items, because one of the items may
	// refer back to this list.
	orig->lv_copyID = copyID;
	orig->lv_copylist = copy;
    }
    CHECK_LIST_MATERIALIZE(orig);
    for (item = orig->lv_first; item != NULL && !got_int;
	    item = item->li_next)
    {
	ni = listitem_alloc();
	if (ni == NULL)
	    break;
	if (deep)
	{
	    if (item_copy(&item->li_tv, &ni->li_tv,
			deep, FALSE, copyID) == FAIL)
	    {
		vim_free(ni);
		break;
	    }
	}
	else
	    copy_tv(&item->li_tv, &ni->li_tv);
	list_append(copy, ni);
    }
    ++copy->lv_refcount;
    if (item != NULL)
    {
	list_unref(copy);
	copy = NULL;
    }

    return copy;
}

/*
 * Remove items "item" to "item2" from list "l".
 * Does not free the listitem or the value!
 * This used to be called list_remove, but that conflicts with a Sun header
 * file.
 */
    void
vimlist_remove(list_T *l, listitem_T *item, listitem_T *item2)
{
    listitem_T	*ip;

    CHECK_LIST_MATERIALIZE(l);

    // notify watchers
    for (ip = item; ip != NULL; ip = ip->li_next)
    {
	--l->lv_len;
	list_fix_watch(l, ip);
	if (ip == item2)
	    break;
    }

    if (item2->li_next == NULL)
	l->lv_u.mat.lv_last = item->li_prev;
    else
	item2->li_next->li_prev = item->li_prev;
    if (item->li_prev == NULL)
	l->lv_first = item2->li_next;
    else
	item->li_prev->li_next = item2->li_next;
    l->lv_u.mat.lv_idx_item = NULL;
}

/*
 * Return an allocated string with the string representation of a list.
 * May return NULL.
 */
    char_u *
list2string(typval_T *tv, int copyID, int restore_copyID)
{
    garray_T	ga;

    if (tv->vval.v_list == NULL)
	return NULL;
    ga_init2(&ga, sizeof(char), 80);
    ga_append(&ga, '[');
    CHECK_LIST_MATERIALIZE(tv->vval.v_list);
    if (list_join(&ga, tv->vval.v_list, (char_u *)", ",
				       FALSE, restore_copyID, copyID) == FAIL)
    {
	vim_free(ga.ga_data);
	return NULL;
    }
    ga_append(&ga, ']');
    ga_append(&ga, NUL);
    return (char_u *)ga.ga_data;
}

typedef struct join_S {
    char_u	*s;
    char_u	*tofree;
} join_T;

    static int
list_join_inner(
    garray_T	*gap,		// to store the result in
    list_T	*l,
    char_u	*sep,
    int		echo_style,
    int		restore_copyID,
    int		copyID,
    garray_T	*join_gap)	// to keep each list item string
{
    int		i;
    join_T	*p;
    int		len;
    int		sumlen = 0;
    int		first = TRUE;
    char_u	*tofree;
    char_u	numbuf[NUMBUFLEN];
    listitem_T	*item;
    char_u	*s;

    // Stringify each item in the list.
    CHECK_LIST_MATERIALIZE(l);
    for (item = l->lv_first; item != NULL && !got_int; item = item->li_next)
    {
	s = echo_string_core(&item->li_tv, &tofree, numbuf, copyID,
				      echo_style, restore_copyID, !echo_style);
	if (s == NULL)
	    return FAIL;

	len = (int)STRLEN(s);
	sumlen += len;

	(void)ga_grow(join_gap, 1);
	p = ((join_T *)join_gap->ga_data) + (join_gap->ga_len++);
	if (tofree != NULL || s != numbuf)
	{
	    p->s = s;
	    p->tofree = tofree;
	}
	else
	{
	    p->s = vim_strnsave(s, len);
	    p->tofree = p->s;
	}

	line_breakcheck();
	if (did_echo_string_emsg)  // recursion error, bail out
	    break;
    }

    // Allocate result buffer with its total size, avoid re-allocation and
    // multiple copy operations.  Add 2 for a tailing ']' and NUL.
    if (join_gap->ga_len >= 2)
	sumlen += (int)STRLEN(sep) * (join_gap->ga_len - 1);
    if (ga_grow(gap, sumlen + 2) == FAIL)
	return FAIL;

    for (i = 0; i < join_gap->ga_len && !got_int; ++i)
    {
	if (first)
	    first = FALSE;
	else
	    ga_concat(gap, sep);
	p = ((join_T *)join_gap->ga_data) + i;

	if (p->s != NULL)
	    ga_concat(gap, p->s);
	line_breakcheck();
    }

    return OK;
}

/*
 * Join list "l" into a string in "*gap", using separator "sep".
 * When "echo_style" is TRUE use String as echoed, otherwise as inside a List.
 * Return FAIL or OK.
 */
    int
list_join(
    garray_T	*gap,
    list_T	*l,
    char_u	*sep,
    int		echo_style,
    int		restore_copyID,
    int		copyID)
{
    garray_T	join_ga;
    int		retval;
    join_T	*p;
    int		i;

    if (l->lv_len < 1)
	return OK; // nothing to do
    ga_init2(&join_ga, sizeof(join_T), l->lv_len);
    retval = list_join_inner(gap, l, sep, echo_style, restore_copyID,
							    copyID, &join_ga);

    if (join_ga.ga_data == NULL)
	return retval;

    // Dispose each item in join_ga.
    p = (join_T *)join_ga.ga_data;
    for (i = 0; i < join_ga.ga_len; ++i)
    {
	vim_free(p->tofree);
	++p;
    }
    ga_clear(&join_ga);

    return retval;
}

/*
 * "join()" function
 */
    void
f_join(typval_T *argvars, typval_T *rettv)
{
    garray_T	ga;
    char_u	*sep;

    rettv->v_type = VAR_STRING;

    if (in_vim9script()
	    && (check_for_list_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL))
	return;

    if (check_for_list_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].vval.v_list == NULL)
	return;

    if (argvars[1].v_type == VAR_UNKNOWN)
	sep = (char_u *)" ";
    else
	sep = tv_get_string_chk(&argvars[1]);

    if (sep != NULL)
    {
	ga_init2(&ga, sizeof(char), 80);
	list_join(&ga, argvars[0].vval.v_list, sep, TRUE, FALSE, 0);
	ga_append(&ga, NUL);
	rettv->vval.v_string = (char_u *)ga.ga_data;
    }
    else
	rettv->vval.v_string = NULL;
}

/*
 * Allocate a variable for a List and fill it from "*arg".
 * "*arg" points to the "[".
 * Return OK or FAIL.
 */
    int
eval_list(char_u **arg, typval_T *rettv, evalarg_T *evalarg, int do_error)
{
    int		evaluate = evalarg == NULL ? FALSE
					 : evalarg->eval_flags & EVAL_EVALUATE;
    list_T	*l = NULL;
    typval_T	tv;
    listitem_T	*item;
    int		vim9script = in_vim9script();
    int		had_comma;

    if (evaluate)
    {
	l = list_alloc();
	if (l == NULL)
	    return FAIL;
    }

    *arg = skipwhite_and_linebreak(*arg + 1, evalarg);
    while (**arg != ']' && **arg != NUL)
    {
	if (eval1(arg, &tv, evalarg) == FAIL)	// recursive!
	    goto failret;
	if (check_typval_is_value(&tv) == FAIL)
	{
	    if (evaluate)
		clear_tv(&tv);
	    goto failret;
	}
	if (evaluate)
	{
	    item = listitem_alloc();
	    if (item != NULL)
	    {
		item->li_tv = tv;
		item->li_tv.v_lock = 0;
		list_append(l, item);
	    }
	    else
		clear_tv(&tv);
	}
	// Legacy Vim script allowed a space before the comma.
	if (!vim9script)
	    *arg = skipwhite(*arg);

	// the comma must come after the value
	had_comma = **arg == ',';
	if (had_comma)
	{
	    if (vim9script && !IS_WHITE_NL_OR_NUL((*arg)[1]) && (*arg)[1] != ']')
	    {
		semsg(_(e_white_space_required_after_str_str), ",", *arg);
		goto failret;
	    }
	    *arg = skipwhite(*arg + 1);
	}

	// The "]" can be on the next line.  But a double quoted string may
	// follow, not a comment.
	*arg = skipwhite_and_linebreak(*arg, evalarg);
	if (**arg == ']')
	    break;

	if (!had_comma)
	{
	    if (do_error)
	    {
		if (**arg == ',')
		    semsg(_(e_no_white_space_allowed_before_str_str),
								    ",", *arg);
		else
		    semsg(_(e_missing_comma_in_list_str), *arg);
	    }
	    goto failret;
	}
    }

    if (**arg != ']')
    {
	if (do_error)
	    semsg(_(e_missing_end_of_list_rsb_str), *arg);
failret:
	if (evaluate)
	    list_free(l);
	return FAIL;
    }

    *arg += 1;
    if (evaluate)
	rettv_list_set(rettv, l);

    return OK;
}

/*
 * Write "list" of strings to file "fd".
 */
    int
write_list(FILE *fd, list_T *list, int binary)
{
    listitem_T	*li;
    int		c;
    int		ret = OK;
    char_u	*s;

    CHECK_LIST_MATERIALIZE(list);
    FOR_ALL_LIST_ITEMS(list, li)
    {
	for (s = tv_get_string(&li->li_tv); *s != NUL; ++s)
	{
	    if (*s == '\n')
		c = putc(NUL, fd);
	    else
		c = putc(*s, fd);
	    if (c == EOF)
	    {
		ret = FAIL;
		break;
	    }
	}
	if (!binary || li->li_next != NULL)
	    if (putc('\n', fd) == EOF)
	    {
		ret = FAIL;
		break;
	    }
	if (ret == FAIL)
	{
	    emsg(_(e_error_while_writing));
	    break;
	}
    }
    return ret;
}

/*
 * Initialize a static list with 10 items.
 */
    void
init_static_list(staticList10_T *sl)
{
    list_T  *l = &sl->sl_list;
    int	    i;

    CLEAR_POINTER(sl);
    l->lv_first = &sl->sl_items[0];
    l->lv_u.mat.lv_last = &sl->sl_items[9];
    l->lv_refcount = DO_NOT_FREE_CNT;
    l->lv_lock = VAR_FIXED;
    sl->sl_list.lv_len = 10;

    for (i = 0; i < 10; ++i)
    {
	listitem_T *li = &sl->sl_items[i];

	if (i == 0)
	    li->li_prev = NULL;
	else
	    li->li_prev = li - 1;
	if (i == 9)
	    li->li_next = NULL;
	else
	    li->li_next = li + 1;
    }
}

/*
 * "list2str()" function
 */
    void
f_list2str(typval_T *argvars, typval_T *rettv)
{
    list_T	*l;
    listitem_T	*li;
    garray_T	ga;
    int		utf8 = FALSE;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script()
	    && (check_for_list_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL))
	return;

    if (check_for_list_arg(argvars, 0) == FAIL)
	return;

    l = argvars[0].vval.v_list;
    if (l == NULL)
	return;  // empty list results in empty string

    if (argvars[1].v_type != VAR_UNKNOWN)
	utf8 = (int)tv_get_bool_chk(&argvars[1], NULL);

    CHECK_LIST_MATERIALIZE(l);
    ga_init2(&ga, 1, 80);
    if (has_mbyte || utf8)
    {
	char_u	buf[MB_MAXBYTES + 1];
	int	(*char2bytes)(int, char_u *);

	if (utf8 || enc_utf8)
	    char2bytes = utf_char2bytes;
	else
	    char2bytes = mb_char2bytes;

	FOR_ALL_LIST_ITEMS(l, li)
	{
	    buf[(*char2bytes)(tv_get_number(&li->li_tv), buf)] = NUL;
	    ga_concat(&ga, buf);
	}
	ga_append(&ga, NUL);
    }
    else if (ga_grow(&ga, list_len(l) + 1) == OK)
    {
	FOR_ALL_LIST_ITEMS(l, li)
	    ga_append(&ga, tv_get_number(&li->li_tv));
	ga_append(&ga, NUL);
    }

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = ga.ga_data;
}

/*
 * Remove item argvars[1] from List argvars[0]. If argvars[2] is supplied, then
 * remove the range of items from argvars[1] to argvars[2] (inclusive).
 */
    static void
list_remove(typval_T *argvars, typval_T *rettv, char_u *arg_errmsg)
{
    list_T	*l;
    listitem_T	*item, *item2;
    listitem_T	*li;
    int		error = FALSE;
    long	idx;
    long	end;
    int		cnt = 0;
    list_T	*rl;

    if ((l = argvars[0].vval.v_list) == NULL
			     || value_check_lock(l->lv_lock, arg_errmsg, TRUE))
	return;

    idx = (long)tv_get_number_chk(&argvars[1], &error);
    if (error)
	return;		// type error: do nothing, errmsg already given

    if ((item = list_find(l, idx)) == NULL)
    {
	semsg(_(e_list_index_out_of_range_nr), idx);
	return;
    }

    if (argvars[2].v_type == VAR_UNKNOWN)
    {
	// Remove one item, return its value.
	vimlist_remove(l, item, item);
	*rettv = item->li_tv;
	list_free_item(l, item);
	return;
    }

    // Remove range of items, return list with values.
    end = (long)tv_get_number_chk(&argvars[2], &error);
    if (error)
	return;		// type error: do nothing

    if ((item2 = list_find(l, end)) == NULL)
    {
	semsg(_(e_list_index_out_of_range_nr), end);
	return;
    }

    for (li = item; li != NULL; li = li->li_next)
    {
	++cnt;
	if (li == item2)
	    break;
    }
    if (li == NULL)  // didn't find "item2" after "item"
    {
	emsg(_(e_invalid_range));
	return;
    }

    vimlist_remove(l, item, item2);
    if (rettv_list_alloc(rettv) == FAIL)
	return;

    rl = rettv->vval.v_list;

    if (l->lv_with_items > 0)
    {
	// need to copy the list items and move the value
	while (item != NULL)
	{
	    li = listitem_alloc();
	    if (li == NULL)
		return;
	    li->li_tv = item->li_tv;
	    init_tv(&item->li_tv);
	    list_append(rl, li);
	    if (item == item2)
		break;
	    item = item->li_next;
	}
    }
    else
    {
	rl->lv_first = item;
	rl->lv_u.mat.lv_last = item2;
	item->li_prev = NULL;
	item2->li_next = NULL;
	rl->lv_len = cnt;
    }
}

static int item_compare(const void *s1, const void *s2);
static int item_compare2(const void *s1, const void *s2);

// struct used in the array that's given to qsort()
typedef struct
{
    listitem_T	*item;
    int		idx;
} sortItem_T;

// struct storing information about current sort
typedef struct
{
    int		item_compare_ic;
    int		item_compare_lc;
    int		item_compare_numeric;
    int		item_compare_numbers;
    int		item_compare_float;
    char_u	*item_compare_func;
    partial_T	*item_compare_partial;
    dict_T	*item_compare_selfdict;
    int		item_compare_func_err;
    int		item_compare_keep_zero;
} sortinfo_T;
static sortinfo_T	*sortinfo = NULL;
#define ITEM_COMPARE_FAIL 999

/*
 * Compare functions for f_sort() and f_uniq() below.
 */
    static int
item_compare(const void *s1, const void *s2)
{
    sortItem_T  *si1, *si2;
    typval_T	*tv1, *tv2;
    char_u	*p1, *p2;
    char_u	*tofree1 = NULL, *tofree2 = NULL;
    int		res;
    char_u	numbuf1[NUMBUFLEN];
    char_u	numbuf2[NUMBUFLEN];

    si1 = (sortItem_T *)s1;
    si2 = (sortItem_T *)s2;
    tv1 = &si1->item->li_tv;
    tv2 = &si2->item->li_tv;

    if (sortinfo->item_compare_numbers)
    {
	varnumber_T	v1 = tv_to_number(tv1);
	varnumber_T	v2 = tv_to_number(tv2);

	return v1 == v2 ? 0 : v1 > v2 ? 1 : -1;
    }

    if (sortinfo->item_compare_float)
    {
	float_T	v1 = tv_get_float(tv1);
	float_T	v2 = tv_get_float(tv2);

	return v1 == v2 ? 0 : v1 > v2 ? 1 : -1;
    }

    // tv2string() puts quotes around a string and allocates memory.  Don't do
    // that for string variables. Use a single quote when comparing with a
    // non-string to do what the docs promise.
    if (tv1->v_type == VAR_STRING)
    {
	if (tv2->v_type != VAR_STRING || sortinfo->item_compare_numeric)
	    p1 = (char_u *)"'";
	else
	    p1 = tv1->vval.v_string;
    }
    else
	p1 = tv2string(tv1, &tofree1, numbuf1, 0);
    if (tv2->v_type == VAR_STRING)
    {
	if (tv1->v_type != VAR_STRING || sortinfo->item_compare_numeric)
	    p2 = (char_u *)"'";
	else
	    p2 = tv2->vval.v_string;
    }
    else
	p2 = tv2string(tv2, &tofree2, numbuf2, 0);
    if (p1 == NULL)
	p1 = (char_u *)"";
    if (p2 == NULL)
	p2 = (char_u *)"";
    if (!sortinfo->item_compare_numeric)
    {
	if (sortinfo->item_compare_lc)
	    res = strcoll((char *)p1, (char *)p2);
	else
	    res = sortinfo->item_compare_ic ? STRICMP(p1, p2): STRCMP(p1, p2);
    }
    else
    {
	double n1, n2;
	n1 = strtod((char *)p1, (char **)&p1);
	n2 = strtod((char *)p2, (char **)&p2);
	res = n1 == n2 ? 0 : n1 > n2 ? 1 : -1;
    }

    // When the result would be zero, compare the item indexes.  Makes the
    // sort stable.
    if (res == 0 && !sortinfo->item_compare_keep_zero)
	res = si1->idx > si2->idx ? 1 : -1;

    vim_free(tofree1);
    vim_free(tofree2);
    return res;
}

    static int
item_compare2(const void *s1, const void *s2)
{
    sortItem_T  *si1, *si2;
    int		res;
    typval_T	rettv;
    typval_T	argv[3];
    char_u	*func_name;
    partial_T	*partial = sortinfo->item_compare_partial;
    funcexe_T	funcexe;
    int		did_emsg_before = did_emsg;

    // shortcut after failure in previous call; compare all items equal
    if (sortinfo->item_compare_func_err)
	return 0;

    si1 = (sortItem_T *)s1;
    si2 = (sortItem_T *)s2;

    if (partial == NULL)
	func_name = sortinfo->item_compare_func;
    else
	func_name = partial_name(partial);

    // Copy the values.  This is needed to be able to set v_lock to VAR_FIXED
    // in the copy without changing the original list items.
    copy_tv(&si1->item->li_tv, &argv[0]);
    copy_tv(&si2->item->li_tv, &argv[1]);

    rettv.v_type = VAR_UNKNOWN;		// clear_tv() uses this
    CLEAR_FIELD(funcexe);
    funcexe.fe_evaluate = TRUE;
    funcexe.fe_partial = partial;
    funcexe.fe_selfdict = sortinfo->item_compare_selfdict;
    res = call_func(func_name, -1, &rettv, 2, argv, &funcexe);
    clear_tv(&argv[0]);
    clear_tv(&argv[1]);

    if (res == FAIL || did_emsg > did_emsg_before)
	res = ITEM_COMPARE_FAIL;
    else
    {
	res = (int)tv_get_number_chk(&rettv, &sortinfo->item_compare_func_err);
	if (res > 0)
	    res = 1;
	else if (res < 0)
	    res = -1;
    }
    if (sortinfo->item_compare_func_err)
	res = ITEM_COMPARE_FAIL;  // return value has wrong type
    clear_tv(&rettv);

    // When the result would be zero, compare the pointers themselves.  Makes
    // the sort stable.
    if (res == 0 && !sortinfo->item_compare_keep_zero)
	res = si1->idx > si2->idx ? 1 : -1;

    return res;
}

/*
 * sort() List "l"
 */
    static void
do_sort(list_T *l, sortinfo_T *info)
{
    long	len;
    sortItem_T	*ptrs;
    long	i = 0;
    listitem_T	*li;

    len = list_len(l);

    // Make an array with each entry pointing to an item in the List.
    ptrs = ALLOC_MULT(sortItem_T, len);
    if (ptrs == NULL)
	return;

    // sort(): ptrs will be the list to sort
    FOR_ALL_LIST_ITEMS(l, li)
    {
	ptrs[i].item = li;
	ptrs[i].idx = i;
	++i;
    }

    info->item_compare_func_err = FALSE;
    info->item_compare_keep_zero = FALSE;
    // test the compare function
    if ((info->item_compare_func != NULL
		|| info->item_compare_partial != NULL)
	    && item_compare2((void *)&ptrs[0], (void *)&ptrs[1])
	    == ITEM_COMPARE_FAIL)
	emsg(_(e_sort_compare_function_failed));
    else
    {
	// Sort the array with item pointers.
	qsort((void *)ptrs, (size_t)len, sizeof(sortItem_T),
		info->item_compare_func == NULL
		&& info->item_compare_partial == NULL
		? item_compare : item_compare2);

	if (!info->item_compare_func_err)
	{
	    // Clear the List and append the items in sorted order.
	    l->lv_first = l->lv_u.mat.lv_last
		= l->lv_u.mat.lv_idx_item = NULL;
	    l->lv_len = 0;
	    for (i = 0; i < len; ++i)
		list_append(l, ptrs[i].item);
	}
    }

    vim_free(ptrs);
}

/*
 * uniq() List "l"
 */
    static void
do_uniq(list_T *l, sortinfo_T *info)
{
    long	len;
    sortItem_T	*ptrs;
    long	i = 0;
    listitem_T	*li;
    int	(*item_compare_func_ptr)(const void *, const void *);

    len = list_len(l);

    // Make an array with each entry pointing to an item in the List.
    ptrs = ALLOC_MULT(sortItem_T, len);
    if (ptrs == NULL)
	return;

    // f_uniq(): ptrs will be a stack of items to remove
    info->item_compare_func_err = FALSE;
    info->item_compare_keep_zero = TRUE;
    item_compare_func_ptr = info->item_compare_func != NULL
	|| info->item_compare_partial != NULL
	? item_compare2 : item_compare;

    for (li = l->lv_first; li != NULL && li->li_next != NULL;
	    li = li->li_next)
    {
	if (item_compare_func_ptr((void *)&li, (void *)&li->li_next)
		== 0)
	    ptrs[i++].item = li;
	if (info->item_compare_func_err)
	{
	    emsg(_(e_uniq_compare_function_failed));
	    break;
	}
    }

    if (!info->item_compare_func_err)
    {
	while (--i >= 0)
	{
	    li = ptrs[i].item->li_next;
	    ptrs[i].item->li_next = li->li_next;
	    if (li->li_next != NULL)
		li->li_next->li_prev = ptrs[i].item;
	    else
		l->lv_u.mat.lv_last = ptrs[i].item;
	    list_fix_watch(l, li);
	    listitem_free(l, li);
	    l->lv_len--;
	}
    }

    vim_free(ptrs);
}

/*
 * Parse the optional arguments supplied to the sort() or uniq() function and
 * return the values in "info".
 */
    static int
parse_sort_uniq_args(typval_T *argvars, sortinfo_T *info)
{
    info->item_compare_ic = FALSE;
    info->item_compare_lc = FALSE;
    info->item_compare_numeric = FALSE;
    info->item_compare_numbers = FALSE;
    info->item_compare_float = FALSE;
    info->item_compare_func = NULL;
    info->item_compare_partial = NULL;
    info->item_compare_selfdict = NULL;

    if (argvars[1].v_type == VAR_UNKNOWN)
	return OK;

    // optional second argument: {func}
    if (argvars[1].v_type == VAR_FUNC)
	info->item_compare_func = argvars[1].vval.v_string;
    else if (argvars[1].v_type == VAR_PARTIAL)
	info->item_compare_partial = argvars[1].vval.v_partial;
    else
    {
	int	    error = FALSE;
	int	    nr = 0;

	if (argvars[1].v_type == VAR_NUMBER)
	{
	    nr = tv_get_number_chk(&argvars[1], &error);
	    if (error)
		return FAIL;
	    if (nr == 1)
		info->item_compare_ic = TRUE;
	}
	if (nr != 1)
	{
	    if (argvars[1].v_type != VAR_NUMBER)
		info->item_compare_func = tv_get_string(&argvars[1]);
	    else if (nr != 0)
	    {
		emsg(_(e_invalid_argument));
		return FAIL;
	    }
	}
	if (info->item_compare_func != NULL)
	{
	    if (*info->item_compare_func == NUL)
	    {
		// empty string means default sort
		info->item_compare_func = NULL;
	    }
	    else if (STRCMP(info->item_compare_func, "n") == 0)
	    {
		info->item_compare_func = NULL;
		info->item_compare_numeric = TRUE;
	    }
	    else if (STRCMP(info->item_compare_func, "N") == 0)
	    {
		info->item_compare_func = NULL;
		info->item_compare_numbers = TRUE;
	    }
	    else if (STRCMP(info->item_compare_func, "f") == 0)
	    {
		info->item_compare_func = NULL;
		info->item_compare_float = TRUE;
	    }
	    else if (STRCMP(info->item_compare_func, "i") == 0)
	    {
		info->item_compare_func = NULL;
		info->item_compare_ic = TRUE;
	    }
	    else if (STRCMP(info->item_compare_func, "l") == 0)
	    {
		info->item_compare_func = NULL;
		info->item_compare_lc = TRUE;
	    }
	}
    }

    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	// optional third argument: {dict}
	if (check_for_dict_arg(argvars, 2) == FAIL)
	    return FAIL;
	info->item_compare_selfdict = argvars[2].vval.v_dict;
    }

    return OK;
}

/*
 * "sort()" or "uniq()" function
 */
    static void
do_sort_uniq(typval_T *argvars, typval_T *rettv, int sort)
{
    list_T	*l;
    sortinfo_T	*old_sortinfo;
    sortinfo_T	info;
    long	len;

    if (in_vim9script()
	    && (check_for_list_arg(argvars, 0) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && (check_for_string_or_func_arg(argvars, 1) == FAIL
			      || check_for_opt_dict_arg(argvars, 2) == FAIL))))
	return;

    if (argvars[0].v_type != VAR_LIST)
    {
	semsg(_(e_argument_of_str_must_be_list), sort ? "sort()" : "uniq()");
	return;
    }

    // Pointer to current info struct used in compare function. Save and
    // restore the current one for nested calls.
    old_sortinfo = sortinfo;
    sortinfo = &info;

    l = argvars[0].vval.v_list;
    if (l != NULL && value_check_lock(l->lv_lock,
		(char_u *)(sort ? N_("sort() argument") : N_("uniq() argument")),
		TRUE))
	goto theend;
    rettv_list_set(rettv, l);
    if (l == NULL)
	goto theend;
    CHECK_LIST_MATERIALIZE(l);

    len = list_len(l);
    if (len <= 1)
	goto theend;	// short list sorts pretty quickly

    if (parse_sort_uniq_args(argvars, &info) == FAIL)
	goto theend;

    if (sort)
	do_sort(l, &info);
    else
	do_uniq(l, &info);

theend:
    sortinfo = old_sortinfo;
}

/*
 * "sort({list})" function
 */
    void
f_sort(typval_T *argvars, typval_T *rettv)
{
    do_sort_uniq(argvars, rettv, TRUE);
}

/*
 * "uniq({list})" function
 */
    void
f_uniq(typval_T *argvars, typval_T *rettv)
{
    do_sort_uniq(argvars, rettv, FALSE);
}

/*
 * Handle one item for map(), filter(), foreach().
 * Sets v:val to "tv".  Caller must set v:key.
 */
    int
filter_map_one(
	typval_T	*tv,	    // original value
	typval_T	*expr,	    // callback
	filtermap_T	filtermap,
	funccall_T	*fc,	    // from eval_expr_get_funccal()
	typval_T	*newtv,	    // for map() and mapnew(): new value
	int		*remp)	    // for filter(): remove flag
{
    typval_T	argv[3];
    int		retval = FAIL;

    copy_tv(tv, get_vim_var_tv(VV_VAL));

    newtv->v_type = VAR_UNKNOWN;
    if (filtermap == FILTERMAP_FOREACH && expr->v_type == VAR_STRING)
    {
	// foreach() is not limited to an expression
	do_cmdline_cmd(expr->vval.v_string);
	if (!did_emsg)
	    retval = OK;
	goto theend;
    }

    argv[0] = *get_vim_var_tv(VV_KEY);
    argv[1] = *get_vim_var_tv(VV_VAL);
    if (eval_expr_typval(expr, FALSE, argv, 2, fc, newtv) == FAIL)
	goto theend;
    if (filtermap == FILTERMAP_FILTER)
    {
	int	    error = FALSE;

	// filter(): when expr is zero remove the item
	if (in_vim9script())
	    *remp = !tv_get_bool_chk(newtv, &error);
	else
	    *remp = (tv_get_number_chk(newtv, &error) == 0);
	clear_tv(newtv);
	// On type error, nothing has been removed; return FAIL to stop the
	// loop.  The error message was given by tv_get_number_chk().
	if (error)
	    goto theend;
    }
    else if (filtermap == FILTERMAP_FOREACH)
	clear_tv(newtv);
    retval = OK;
theend:
    clear_tv(get_vim_var_tv(VV_VAL));
    return retval;
}

/*
 * Implementation of map(), filter(), foreach() for a List.  Apply "expr" to
 * every item in List "l" and return the result in "rettv".
 */
    static void
list_filter_map(
	list_T		*l,
	filtermap_T	filtermap,
	type_T		*argtype,
	char		*func_name,
	char_u		*arg_errmsg,
	typval_T	*expr,
	typval_T	*rettv)
{
    int		prev_lock;
    list_T	*l_ret = NULL;
    int		idx = 0;
    int		rem;
    listitem_T	*li, *nli;
    typval_T	newtv;
    funccall_T	*fc;

    if (filtermap == FILTERMAP_MAPNEW)
    {
	rettv->v_type = VAR_LIST;
	rettv->vval.v_list = NULL;
    }
    if (l == NULL || (filtermap == FILTERMAP_FILTER
			    && value_check_lock(l->lv_lock, arg_errmsg, TRUE)))
	return;

    prev_lock = l->lv_lock;

    if (filtermap == FILTERMAP_MAPNEW)
    {
	if (rettv_list_alloc(rettv) == FAIL)
	    return;
	l_ret = rettv->vval.v_list;
    }
    // set_vim_var_nr() doesn't set the type
    set_vim_var_type(VV_KEY, VAR_NUMBER);

    if (l->lv_lock == 0)
	l->lv_lock = VAR_LOCKED;

    // Create one funccall_T for all eval_expr_typval() calls.
    fc = eval_expr_get_funccal(expr, &newtv);

    if (l->lv_first == &range_list_item)
    {
	varnumber_T	val = l->lv_u.nonmat.lv_start;
	int		len = l->lv_len;
	int		stride = l->lv_u.nonmat.lv_stride;

	// List from range(): loop over the numbers
	// NOTE: foreach() returns the range_list_item
	if (filtermap != FILTERMAP_MAPNEW && filtermap != FILTERMAP_FOREACH)
	{
	    l->lv_first = NULL;
	    l->lv_u.mat.lv_last = NULL;
	    l->lv_len = 0;
	    l->lv_u.mat.lv_idx_item = NULL;
	}

	for (idx = 0; idx < len; ++idx)
	{
	    typval_T tv;

	    tv.v_type = VAR_NUMBER;
	    tv.v_lock = 0;
	    tv.vval.v_number = val;
	    set_vim_var_nr(VV_KEY, idx);
	    if (filter_map_one(&tv, expr, filtermap, fc, &newtv, &rem) == FAIL)
		break;
	    if (did_emsg)
	    {
		clear_tv(&newtv);
		break;
	    }
	    if (filtermap != FILTERMAP_FOREACH)
	    {
		if (filtermap != FILTERMAP_FILTER)
		{
		    if (filtermap == FILTERMAP_MAP && argtype != NULL
			&& check_typval_arg_type(
						 argtype->tt_member, &newtv,
						 func_name, 0) == FAIL)
		    {
			clear_tv(&newtv);
			break;
		    }
		    // map(), mapnew(): always append the new value to the
		    // list
		    if (list_append_tv_move(filtermap == FILTERMAP_MAP
					    ? l : l_ret, &newtv) == FAIL)
			break;
		}
		else if (!rem)
		{
		    // filter(): append the list item value when not rem
		    if (list_append_tv_move(l, &tv) == FAIL)
			break;
		}
	    }

	    val += stride;
	}
    }
    else
    {
	// Materialized list: loop over the items
	for (li = l->lv_first; li != NULL; li = nli)
	{
	    if (filtermap == FILTERMAP_MAP && value_check_lock(
			li->li_tv.v_lock, arg_errmsg, TRUE))
		break;
	    nli = li->li_next;
	    set_vim_var_nr(VV_KEY, idx);
	    if (filter_map_one(&li->li_tv, expr, filtermap, fc,
							 &newtv, &rem) == FAIL)
		break;
	    if (did_emsg)
	    {
		clear_tv(&newtv);
		break;
	    }
	    if (filtermap == FILTERMAP_MAP)
	    {
		if (argtype != NULL && check_typval_arg_type(
			    argtype->tt_member, &newtv, func_name, 0) == FAIL)
		{
		    clear_tv(&newtv);
		    break;
		}
		// map(): replace the list item value
		clear_tv(&li->li_tv);
		newtv.v_lock = 0;
		li->li_tv = newtv;
	    }
	    else if (filtermap == FILTERMAP_MAPNEW)
	    {
		// mapnew(): append the list item value
		if (list_append_tv_move(l_ret, &newtv) == FAIL)
		    break;
	    }
	    else if (filtermap == FILTERMAP_FILTER && rem)
		    listitem_remove(l, li);
	    ++idx;
	}
    }

    l->lv_lock = prev_lock;
    if (fc != NULL)
	remove_funccal();
}

/*
 * Implementation of map(), filter() and foreach().
 */
    static void
filter_map(typval_T *argvars, typval_T *rettv, filtermap_T filtermap)
{
    typval_T	*expr;
    char	*func_name = filtermap == FILTERMAP_MAP ? "map()"
				  : filtermap == FILTERMAP_MAPNEW ? "mapnew()"
				  : filtermap == FILTERMAP_FILTER ? "filter()"
				  : "foreach()";
    char_u	*arg_errmsg = (char_u *)(filtermap == FILTERMAP_MAP
							 ? N_("map() argument")
				       : filtermap == FILTERMAP_MAPNEW
						      ? N_("mapnew() argument")
				       : filtermap == FILTERMAP_FILTER
						      ? N_("filter() argument")
						   : N_("foreach() argument"));
    int		save_did_emsg;
    type_T	*type = NULL;

    // map(), filter(), foreach() return the first argument, also on failure.
    if (filtermap != FILTERMAP_MAPNEW && argvars[0].v_type != VAR_STRING)
	copy_tv(&argvars[0], rettv);

    if (in_vim9script()
	    && (check_for_list_or_dict_or_blob_or_string_arg(argvars, 0)
								== FAIL))
	return;

    if (filtermap == FILTERMAP_MAP && in_vim9script())
    {
	// Check that map() does not change the declared type of the list or
	// dict.
	if (argvars[0].v_type == VAR_DICT && argvars[0].vval.v_dict != NULL)
	    type = argvars[0].vval.v_dict->dv_type;
	else if (argvars[0].v_type == VAR_LIST
					     && argvars[0].vval.v_list != NULL)
	    type = argvars[0].vval.v_list->lv_type;
    }

    if (argvars[0].v_type != VAR_BLOB
	    && argvars[0].v_type != VAR_LIST
	    && argvars[0].v_type != VAR_DICT
	    && argvars[0].v_type != VAR_STRING)
    {
	semsg(_(e_argument_of_str_must_be_list_string_dictionary_or_blob),
								    func_name);
	return;
    }

    // On type errors, the preceding call has already displayed an error
    // message.  Avoid a misleading error message for an empty string that
    // was not passed as argument.
    expr = &argvars[1];
    if (expr->v_type == VAR_UNKNOWN)
	return;

    typval_T	save_val;
    typval_T	save_key;

    prepare_vimvar(VV_VAL, &save_val);
    prepare_vimvar(VV_KEY, &save_key);

    // We reset "did_emsg" to be able to detect whether an error
    // occurred during evaluation of the expression.
    save_did_emsg = did_emsg;
    did_emsg = FALSE;

    if (argvars[0].v_type == VAR_DICT)
	dict_filter_map(argvars[0].vval.v_dict, filtermap, type, func_name,
						      arg_errmsg, expr, rettv);
    else if (argvars[0].v_type == VAR_BLOB)
	blob_filter_map(argvars[0].vval.v_blob, filtermap, expr,
							    arg_errmsg, rettv);
    else if (argvars[0].v_type == VAR_STRING)
	string_filter_map(tv_get_string(&argvars[0]), filtermap, expr, rettv);
    else // argvars[0].v_type == VAR_LIST
	list_filter_map(argvars[0].vval.v_list, filtermap, type, func_name,
						      arg_errmsg, expr, rettv);

    restore_vimvar(VV_KEY, &save_key);
    restore_vimvar(VV_VAL, &save_val);

    did_emsg |= save_did_emsg;
}

/*
 * "filter()" function
 */
    void
f_filter(typval_T *argvars, typval_T *rettv)
{
    filter_map(argvars, rettv, FILTERMAP_FILTER);
}

/*
 * "map()" function
 */
    void
f_map(typval_T *argvars, typval_T *rettv)
{
    filter_map(argvars, rettv, FILTERMAP_MAP);
}

/*
 * "mapnew()" function
 */
    void
f_mapnew(typval_T *argvars, typval_T *rettv)
{
    filter_map(argvars, rettv, FILTERMAP_MAPNEW);
}

/*
 * "foreach()" function
 */
    void
f_foreach(typval_T *argvars, typval_T *rettv)
{
    filter_map(argvars, rettv, FILTERMAP_FOREACH);
}

/*
 * "add(list, item)" function
 */
    static void
list_add(typval_T *argvars, typval_T *rettv)
{
    list_T	*l = argvars[0].vval.v_list;

    if (l == NULL)
    {
	if (in_vim9script())
	    emsg(_(e_cannot_add_to_null_list));
    }
    else if (!value_check_lock(l->lv_lock,
		(char_u *)N_("add() argument"), TRUE)
	    && list_append_tv(l, &argvars[1]) == OK)
    {
	copy_tv(&argvars[0], rettv);
    }
}

/*
 * "add(object, item)" function
 */
    void
f_add(typval_T *argvars, typval_T *rettv)
{
    rettv->vval.v_number = 1; // Default: Failed

    if (in_vim9script()
	    && (check_for_list_or_blob_arg(argvars, 0) == FAIL
		|| (argvars[0].v_type == VAR_BLOB
		    && check_for_number_arg(argvars, 1) == FAIL)))
	return;

    if (argvars[0].v_type == VAR_LIST)
	list_add(argvars, rettv);
    else if (argvars[0].v_type == VAR_BLOB)
	blob_add(argvars, rettv);
    else
	emsg(_(e_list_or_blob_required));
}

/*
 * Count the number of times item "needle" occurs in List "l" starting at index
 * "idx". Case is ignored if "ic" is TRUE.
 */
    static long
list_count(list_T *l, typval_T *needle, long idx, int ic)
{
    long	n = 0;
    listitem_T	*li;

    if (l == NULL)
	return 0;

    CHECK_LIST_MATERIALIZE(l);

    if (list_len(l) == 0)
	return 0;

    li = list_find(l, idx);
    if (li == NULL)
    {
	semsg(_(e_list_index_out_of_range_nr), idx);
	return 0;
    }

    for ( ; li != NULL; li = li->li_next)
	if (tv_equal(&li->li_tv, needle, ic, FALSE))
	    ++n;

    return n;
}

/*
 * "count()" function
 */
    void
f_count(typval_T *argvars, typval_T *rettv)
{
    long	n = 0;
    int		ic = FALSE;
    int		error = FALSE;

    if (in_vim9script()
	    && (check_for_string_or_list_or_dict_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 2) == FAIL
		|| (argvars[2].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 3) == FAIL)))
	return;

    if (argvars[2].v_type != VAR_UNKNOWN)
	ic = (int)tv_get_bool_chk(&argvars[2], &error);

    if (!error && argvars[0].v_type == VAR_STRING)
	n = string_count(argvars[0].vval.v_string,
					tv_get_string_chk(&argvars[1]), ic);
    else if (!error && argvars[0].v_type == VAR_LIST)
    {
	long idx = 0;

	if (argvars[2].v_type != VAR_UNKNOWN
		&& argvars[3].v_type != VAR_UNKNOWN)
	    idx = (long)tv_get_number_chk(&argvars[3], &error);
	if (!error)
	    n = list_count(argvars[0].vval.v_list, &argvars[1], idx, ic);
    }
    else if (!error && argvars[0].v_type == VAR_DICT)
    {
	if (argvars[2].v_type != VAR_UNKNOWN
		&& argvars[3].v_type != VAR_UNKNOWN)
	    emsg(_(e_invalid_argument));
	else
	    n = dict_count(argvars[0].vval.v_dict, &argvars[1], ic);
    }
    else if (!error)
	semsg(_(e_argument_of_str_must_be_list_string_or_dictionary),
								    "count()");
    rettv->vval.v_number = n;
}

/*
 * extend() a List. Append List argvars[1] to List argvars[0] before index
 * argvars[3] and return the resulting list in "rettv".  "is_new" is TRUE for
 * extendnew().
 */
    static void
list_extend_func(
	typval_T	*argvars,
	type_T		*type,
	char		*func_name,
	char_u		*arg_errmsg,
	int		is_new,
	typval_T	*rettv)
{
    list_T	*l1, *l2;
    listitem_T	*item;
    long	before;
    int		error = FALSE;

    l1 = argvars[0].vval.v_list;
    if (l1 == NULL)
    {
	emsg(_(e_cannot_extend_null_list));
	return;
    }
    l2 = argvars[1].vval.v_list;
    if ((is_new || !value_check_lock(l1->lv_lock, arg_errmsg, TRUE))
								 && l2 != NULL)
    {
	if (is_new)
	{
	    l1 = list_copy(l1, FALSE, TRUE, get_copyID());
	    if (l1 == NULL)
		return;
	}

	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    before = (long)tv_get_number_chk(&argvars[2], &error);
	    if (error)
		return;		// type error; errmsg already given

	    if (before == l1->lv_len)
		item = NULL;
	    else
	    {
		item = list_find(l1, before);
		if (item == NULL)
		{
		    semsg(_(e_list_index_out_of_range_nr), before);
		    return;
		}
	    }
	}
	else
	    item = NULL;
	if (type != NULL && check_typval_arg_type(
		    type, &argvars[1], func_name, 2) == FAIL)
	    return;
	list_extend(l1, l2, item);

	if (is_new)
	{
	    rettv->v_type = VAR_LIST;
	    rettv->vval.v_list = l1;
	    rettv->v_lock = FALSE;
	}
	else
	    copy_tv(&argvars[0], rettv);
    }
}

/*
 * "extend()" or "extendnew()" function.  "is_new" is TRUE for extendnew().
 */
    static void
extend(typval_T *argvars, typval_T *rettv, char_u *arg_errmsg, int is_new)
{
    type_T	*type = NULL;
    char	*func_name = is_new ? "extendnew()" : "extend()";

    if (argvars[0].v_type == VAR_LIST && argvars[1].v_type == VAR_LIST)
    {
	// Check that extend() does not change the type of the list if it was
	// declared.
	if (!is_new && in_vim9script() && argvars[0].vval.v_list != NULL)
	    type = argvars[0].vval.v_list->lv_type;
	list_extend_func(argvars, type, func_name, arg_errmsg, is_new, rettv);
    }
    else if (argvars[0].v_type == VAR_DICT && argvars[1].v_type == VAR_DICT)
    {
	// Check that extend() does not change the type of the dict if it was
	// declared.
	if (!is_new && in_vim9script() && argvars[0].vval.v_dict != NULL)
	    type = argvars[0].vval.v_dict->dv_type;
	dict_extend_func(argvars, type, func_name, arg_errmsg, is_new, rettv);
    }
    else
	semsg(_(e_argument_of_str_must_be_list_or_dictionary), func_name);
}

/*
 * "extend(list, list [, idx])" function
 * "extend(dict, dict [, action])" function
 */
    void
f_extend(typval_T *argvars, typval_T *rettv)
{
    char_u      *errmsg = (char_u *)N_("extend() argument");

    extend(argvars, rettv, errmsg, FALSE);
}

/*
 * "extendnew(list, list [, idx])" function
 * "extendnew(dict, dict [, action])" function
 */
    void
f_extendnew(typval_T *argvars, typval_T *rettv)
{
    char_u      *errmsg = (char_u *)N_("extendnew() argument");

    extend(argvars, rettv, errmsg, TRUE);
}

    static void
list_insert_func(typval_T *argvars, typval_T *rettv)
{
    list_T	*l = argvars[0].vval.v_list;
    long	before = 0;
    listitem_T	*item;
    int		error = FALSE;

    if (l == NULL)
    {
	if (in_vim9script())
	    emsg(_(e_cannot_add_to_null_list));
	return;
    }

    if (value_check_lock(l->lv_lock, (char_u *)N_("insert() argument"), TRUE))
	return;

    if (argvars[2].v_type != VAR_UNKNOWN)
	before = (long)tv_get_number_chk(&argvars[2], &error);
    if (error)
	return;		// type error; errmsg already given

    if (before == l->lv_len)
	item = NULL;
    else
    {
	item = list_find(l, before);
	if (item == NULL)
	{
	    semsg(_(e_list_index_out_of_range_nr), before);
	    l = NULL;
	}
    }
    if (l != NULL)
    {
	(void)list_insert_tv(l, &argvars[1], item);
	copy_tv(&argvars[0], rettv);
    }
}

/*
 * "insert()" function
 */
    void
f_insert(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_list_or_blob_arg(argvars, 0) == FAIL
		|| (argvars[0].v_type == VAR_BLOB
		    && check_for_number_arg(argvars, 1) == FAIL)
		|| check_for_opt_number_arg(argvars, 2) == FAIL))
	return;

    if (argvars[0].v_type == VAR_BLOB)
	blob_insert_func(argvars, rettv);
    else if (argvars[0].v_type != VAR_LIST)
	semsg(_(e_argument_of_str_must_be_list_or_blob), "insert()");
    else
	list_insert_func(argvars, rettv);
}

/*
 * "remove()" function
 */
    void
f_remove(typval_T *argvars, typval_T *rettv)
{
    char_u	*arg_errmsg = (char_u *)N_("remove() argument");

    if (in_vim9script()
	    && (check_for_list_or_dict_or_blob_arg(argvars, 0) == FAIL
		|| ((argvars[0].v_type == VAR_LIST
			|| argvars[0].v_type == VAR_BLOB)
		    && (check_for_number_arg(argvars, 1) == FAIL
			|| check_for_opt_number_arg(argvars, 2) == FAIL))
		|| (argvars[0].v_type == VAR_DICT
		    && check_for_string_or_number_arg(argvars, 1) == FAIL)))
	return;

    if (argvars[0].v_type == VAR_DICT)
	dict_remove(argvars, rettv, arg_errmsg);
    else if (argvars[0].v_type == VAR_BLOB)
	blob_remove(argvars, rettv, arg_errmsg);
    else if (argvars[0].v_type == VAR_LIST)
	list_remove(argvars, rettv, arg_errmsg);
    else
	semsg(_(e_argument_of_str_must_be_list_dictionary_or_blob), "remove()");
}

    static void
list_reverse(list_T *l, typval_T *rettv)
{
    listitem_T	*li, *ni;

    rettv_list_set(rettv, l);
    if (l != NULL
	    && !value_check_lock(l->lv_lock,
		(char_u *)N_("reverse() argument"), TRUE))
    {
	if (l->lv_first == &range_list_item)
	{
	    varnumber_T new_start = l->lv_u.nonmat.lv_start
		+ ((varnumber_T)l->lv_len - 1) * l->lv_u.nonmat.lv_stride;
	    l->lv_u.nonmat.lv_end = new_start
		- (l->lv_u.nonmat.lv_end - l->lv_u.nonmat.lv_start);
	    l->lv_u.nonmat.lv_start = new_start;
	    l->lv_u.nonmat.lv_stride = -l->lv_u.nonmat.lv_stride;
	    return;
	}
	li = l->lv_u.mat.lv_last;
	l->lv_first = l->lv_u.mat.lv_last = NULL;
	l->lv_len = 0;
	while (li != NULL)
	{
	    ni = li->li_prev;
	    list_append(l, li);
	    li = ni;
	}
	l->lv_u.mat.lv_idx = l->lv_len - l->lv_u.mat.lv_idx - 1;
    }
}

/*
 * "reverse({list})" function
 */
    void
f_reverse(typval_T *argvars, typval_T *rettv)
{
    if (check_for_string_or_list_or_blob_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_BLOB)
	blob_reverse(argvars[0].vval.v_blob, rettv);
    else if (argvars[0].v_type == VAR_STRING)
    {
	rettv->v_type = VAR_STRING;
	if (argvars[0].vval.v_string != NULL)
	    rettv->vval.v_string = reverse_text(argvars[0].vval.v_string);
	else
	    rettv->vval.v_string = NULL;
    }
    else if (argvars[0].v_type == VAR_LIST)
	list_reverse(argvars[0].vval.v_list, rettv);
}

/*
 * Implementation of reduce() for list "argvars[0]", using the function "expr"
 * starting with the optional initial value argvars[2] and return the result in
 * "rettv".
 */
    static void
list_reduce(
	typval_T	*argvars,
	typval_T	*expr,
	typval_T	*rettv)
{
    list_T	*l = argvars[0].vval.v_list;
    listitem_T  *li = NULL;
    int		range_list;
    int		range_idx = 0;
    varnumber_T	range_val = 0;
    typval_T	initial;
    typval_T	argv[3];
    int		r;
    int		called_emsg_start = called_emsg;
    int		prev_locked;
    funccall_T	*fc;

    // Using reduce on a range() uses "range_idx" and "range_val".
    range_list = l != NULL && l->lv_first == &range_list_item;
    if (range_list)
	range_val = l->lv_u.nonmat.lv_start;

    if (argvars[2].v_type == VAR_UNKNOWN)
    {
	if (l == NULL || l->lv_len == 0)
	{
	    semsg(_(e_reduce_of_an_empty_str_with_no_initial_value), "List");
	    return;
	}
	if (range_list)
	{
	    initial.v_type = VAR_NUMBER;
	    initial.vval.v_number = range_val;
	    range_val += l->lv_u.nonmat.lv_stride;
	    range_idx = 1;
	}
	else
	{
	    initial = l->lv_first->li_tv;
	    li = l->lv_first->li_next;
	}
    }
    else
    {
	initial = argvars[2];
	if (l != NULL && !range_list)
	    li = l->lv_first;
    }
    copy_tv(&initial, rettv);

    if (l == NULL)
	return;

    // Create one funccall_T for all eval_expr_typval() calls.
    fc = eval_expr_get_funccal(expr, rettv);

    prev_locked = l->lv_lock;
    l->lv_lock = VAR_FIXED;  // disallow the list changing here

    while (range_list ? range_idx < l->lv_len : li != NULL)
    {
	argv[0] = *rettv;
	rettv->v_type = VAR_UNKNOWN;

	if (range_list)
	{
	    argv[1].v_type = VAR_NUMBER;
	    argv[1].vval.v_number = range_val;
	}
	else
	    argv[1] = li->li_tv;

	r = eval_expr_typval(expr, TRUE, argv, 2, fc, rettv);

	if (argv[0].v_type != VAR_NUMBER && argv[0].v_type != VAR_UNKNOWN)
	    clear_tv(&argv[0]);
	if (r == FAIL || called_emsg != called_emsg_start)
	    break;

	// advance to the next item
	if (range_list)
	{
	    range_val += l->lv_u.nonmat.lv_stride;
	    ++range_idx;
	}
	else
	    li = li->li_next;
    }

    if (fc != NULL)
	remove_funccal();

    l->lv_lock = prev_locked;
}

/*
 * "reduce(list, { accumulator, element -> value } [, initial])" function
 * "reduce(blob, { accumulator, element -> value } [, initial])"
 * "reduce(string, { accumulator, element -> value } [, initial])"
 */
    void
f_reduce(typval_T *argvars, typval_T *rettv)
{
    char_u	*func_name;

    if (in_vim9script()
		   && check_for_string_or_list_or_blob_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_STRING
	    && argvars[0].v_type != VAR_LIST
	    && argvars[0].v_type != VAR_BLOB)
    {
	emsg(_(e_string_list_or_blob_required));
	return;
    }

    if (argvars[1].v_type == VAR_FUNC)
	func_name = argvars[1].vval.v_string;
    else if (argvars[1].v_type == VAR_PARTIAL)
	func_name = partial_name(argvars[1].vval.v_partial);
    else
	func_name = tv_get_string(&argvars[1]);
    if (func_name == NULL || *func_name == NUL)
    {
	emsg(_(e_missing_function_argument));
	return;
    }

    if (argvars[0].v_type == VAR_LIST)
	list_reduce(argvars, &argvars[1], rettv);
    else if (argvars[0].v_type == VAR_STRING)
	string_reduce(argvars, &argvars[1], rettv);
    else
	blob_reduce(argvars, &argvars[1], rettv);
}

#endif // defined(FEAT_EVAL)
