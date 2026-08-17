/*    invlist_inline.h
 *
 *    Copyright (C) 2012 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 */

#ifndef PERL_INVLIST_INLINE_H_
#define PERL_INVLIST_INLINE_H_

#if defined(PERL_IN_UTF8_C)             \
 || defined(PERL_IN_REGCOMP_C)          \
 || defined(PERL_IN_REGEXEC_C)          \
 || defined(PERL_IN_TOKE_C)             \
 || defined(PERL_IN_PP_C)               \
 || defined(PERL_IN_OP_C)               \
 || defined(PERL_IN_DOOP_C)

/* An element is in an inversion list iff its index is even numbered: 0, 2, 4,
 * etc */
#define ELEMENT_RANGE_MATCHES_INVLIST(i) (! ((i) & 1))
#define PREV_RANGE_MATCHES_INVLIST(i) (! ELEMENT_RANGE_MATCHES_INVLIST(i))

/* This converts to/from our UVs to what the SV code is expecting: bytes. */
#define TO_INTERNAL_SIZE(x) ((x) * sizeof(UV))
#define FROM_INTERNAL_SIZE(x) ((x)/ sizeof(UV))

PERL_STATIC_INLINE bool
S_is_invlist(SV* const invlist)
{
    return invlist != NULL && SvTYPE(invlist) == SVt_INVLIST;
}

PERL_STATIC_INLINE bool*
S_get_invlist_offset_addr(SV* invlist)
{
    /* Return the address of the field that says whether the inversion list is
     * offset (it contains 1) or not (contains 0) */
    PERL_ARGS_ASSERT_GET_INVLIST_OFFSET_ADDR;

    assert(is_invlist(invlist));

    return &(((XINVLIST*) SvANY(invlist))->is_offset);
}

PERL_STATIC_INLINE UV
S__invlist_len(SV* const invlist)
{
    /* Returns the current number of elements stored in the inversion list's
     * array */

    PERL_ARGS_ASSERT__INVLIST_LEN;

    assert(is_invlist(invlist));

    return (SvCUR(invlist) == 0)
           ? 0
           : FROM_INTERNAL_SIZE(SvCUR(invlist)) - *get_invlist_offset_addr(invlist);
}

PERL_STATIC_INLINE bool
S__invlist_contains_cp(SV* const invlist, const UV cp)
{
    /* Does <invlist> contain code point <cp> as part of the set? */

    IV index = _invlist_search(invlist, cp);

    PERL_ARGS_ASSERT__INVLIST_CONTAINS_CP;

    return index >= 0 && ELEMENT_RANGE_MATCHES_INVLIST(index);
}

PERL_STATIC_INLINE UV*
S_invlist_array(SV* const invlist)
{
    /* Returns the pointer to the inversion list's array.  Every time the
     * length changes, this needs to be called in case malloc or realloc moved
     * it */

    PERL_ARGS_ASSERT_INVLIST_ARRAY;

    /* Must not be empty.  If these fail, you probably didn't check for <len>
     * being non-zero before trying to get the array */
    assert(_invlist_len(invlist));

    /* The very first element always contains zero, The array begins either
     * there, or if the inversion list is offset, at the element after it.
     * The offset header field determines which; it contains 0 or 1 to indicate
     * how much additionally to add */
    assert(0 == *(SvPVX(invlist)));
    return ((UV *) SvPVX(invlist) + *get_invlist_offset_addr(invlist));
}

#endif
#if defined(PERL_IN_REGCOMP_C) || defined(PERL_IN_OP_C) || defined(PERL_IN_DOOP_C)

PERL_STATIC_INLINE void
S_invlist_extend(pTHX_ SV* const invlist, const UV new_max)
{
    /* Grow the maximum size of an inversion list */

    PERL_ARGS_ASSERT_INVLIST_EXTEND;

    assert(SvTYPE(invlist) == SVt_INVLIST);

    /* Add one to account for the zero element at the beginning which may not
     * be counted by the calling parameters */
    SvGROW((SV *)invlist, TO_INTERNAL_SIZE(new_max + 1));
}

PERL_STATIC_INLINE void
S_invlist_set_len(pTHX_ SV* const invlist, const UV len, const bool offset)
{
    /* Sets the current number of elements stored in the inversion list.
     * Updates SvCUR correspondingly */
    PERL_UNUSED_CONTEXT;
    PERL_ARGS_ASSERT_INVLIST_SET_LEN;

    assert(SvTYPE(invlist) == SVt_INVLIST);

    SvCUR_set(invlist,
              (len == 0)
               ? 0
               : TO_INTERNAL_SIZE(len + offset));
    assert(SvLEN(invlist) == 0 || SvCUR(invlist) <= SvLEN(invlist));
}

PERL_STATIC_INLINE SV*
S_add_cp_to_invlist(pTHX_ SV* invlist, const UV cp) {
    return _add_range_to_invlist(invlist, cp, cp);
}

PERL_STATIC_INLINE UV
S_invlist_highest(SV* const invlist)
{
    /* Returns the highest code point that matches an inversion list.  This API
     * has an ambiguity, as it returns 0 under either the highest is actually
     * 0, or if the list is empty.  If this distinction matters to you, check
     * for emptiness before calling this function */

    UV len = _invlist_len(invlist);
    UV *array;

    PERL_ARGS_ASSERT_INVLIST_HIGHEST;

    if (len == 0) {
	return 0;
    }

    array = invlist_array(invlist);

    /* The last element in the array in the inversion list always starts a
     * range that goes to infinity.  That range may be for code points that are
     * matched in the inversion list, or it may be for ones that aren't
     * matched.  In the latter case, the highest code point in the set is one
     * less than the beginning of this range; otherwise it is the final element
     * of this range: infinity */
    return (ELEMENT_RANGE_MATCHES_INVLIST(len - 1))
           ? UV_MAX
           : array[len - 1] - 1;
}

#endif
#if defined(PERL_IN_REGCOMP_C) || defined(PERL_IN_OP_C)

PERL_STATIC_INLINE STRLEN*
S_get_invlist_iter_addr(SV* invlist)
{
    /* Return the address of the UV that contains the current iteration
     * position */

    PERL_ARGS_ASSERT_GET_INVLIST_ITER_ADDR;

    assert(is_invlist(invlist));

    return &(((XINVLIST*) SvANY(invlist))->iterator);
}

PERL_STATIC_INLINE void
S_invlist_iterinit(SV* invlist)	/* Initialize iterator for invlist */
{
    PERL_ARGS_ASSERT_INVLIST_ITERINIT;

    *get_invlist_iter_addr(invlist) = 0;
}

PERL_STATIC_INLINE void
S_invlist_iterfinish(SV* invlist)
{
    /* Terminate iterator for invlist.  This is to catch development errors.
     * Any iteration that is interrupted before completed should call this
     * function.  Functions that add code points anywhere else but to the end
     * of an inversion list assert that they are not in the middle of an
     * iteration.  If they were, the addition would make the iteration
     * problematical: if the iteration hadn't reached the place where things
     * were being added, it would be ok */

    PERL_ARGS_ASSERT_INVLIST_ITERFINISH;

    *get_invlist_iter_addr(invlist) = (STRLEN) UV_MAX;
}

STATIC bool
S_invlist_iternext(SV* invlist, UV* start, UV* end)
{
    /* An C<invlist_iterinit> call on <invlist> must be used to set this up.
     * This call sets in <*start> and <*end>, the next range in <invlist>.
     * Returns <TRUE> if successful and the next call will return the next
     * range; <FALSE> if was already at the end of the list.  If the latter,
     * <*start> and <*end> are unchanged, and the next call to this function
     * will start over at the beginning of the list */

    STRLEN* pos = get_invlist_iter_addr(invlist);
    UV len = _invlist_len(invlist);
    UV *array;

    PERL_ARGS_ASSERT_INVLIST_ITERNEXT;

    if (*pos >= len) {
	*pos = (STRLEN) UV_MAX;	/* Force iterinit() to be required next time */
	return FALSE;
    }

    array = invlist_array(invlist);

    *start = array[(*pos)++];

    if (*pos >= len) {
	*end = UV_MAX;
    }
    else {
	*end = array[(*pos)++] - 1;
    }

    return TRUE;
}

#endif

#ifndef PERL_IN_REGCOMP_C

/* These symbols are only needed later in regcomp.c */
#       undef TO_INTERNAL_SIZE
#       undef FROM_INTERNAL_SIZE
#endif

#endif /* PERL_INVLIST_INLINE_H_ */
