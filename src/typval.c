/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * typval.c: functions that deal with a typval
 */

#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

/*
 * Allocate memory for a variable type-value, and make it empty (0 or NULL
 * value).
 */
    typval_T *
alloc_tv(void)
{
    return ALLOC_CLEAR_ONE(typval_T);
}

/*
 * Allocate memory for a variable type-value, and assign a string to it.
 * The string "s" must have been allocated, it is consumed.
 * Return NULL for out of memory, the variable otherwise.
 */
    typval_T *
alloc_string_tv(char_u *s)
{
    typval_T	*rettv;

    rettv = alloc_tv();
    if (rettv != NULL)
    {
	rettv->v_type = VAR_STRING;
	rettv->vval.v_string = s;
    }
    else
	vim_free(s);
    return rettv;
}

/*
 * Free the memory for a variable type-value.
 */
    void
free_tv(typval_T *varp)
{
    if (varp == NULL)
	return;

    switch (varp->v_type)
    {
	case VAR_FUNC:
	    func_unref(varp->vval.v_string);
	    // FALLTHROUGH
	case VAR_STRING:
	    vim_free(varp->vval.v_string);
	    break;
	case VAR_PARTIAL:
	    partial_unref(varp->vval.v_partial);
	    break;
	case VAR_BLOB:
	    blob_unref(varp->vval.v_blob);
	    break;
	case VAR_LIST:
	    list_unref(varp->vval.v_list);
	    break;
	case VAR_DICT:
	    dict_unref(varp->vval.v_dict);
	    break;
	case VAR_JOB:
#ifdef FEAT_JOB_CHANNEL
	    job_unref(varp->vval.v_job);
	    break;
#endif
	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    channel_unref(varp->vval.v_channel);
	    break;
#endif
	case VAR_CLASS:
	    class_unref(varp->vval.v_class);
	    break;
	case VAR_OBJECT:
	    object_unref(varp->vval.v_object);
	    break;

	case VAR_TYPEALIAS:
	    typealias_unref(varp->vval.v_typealias);
	    break;

	case VAR_NUMBER:
	case VAR_FLOAT:
	case VAR_ANY:
	case VAR_UNKNOWN:
	case VAR_VOID:
	case VAR_BOOL:
	case VAR_SPECIAL:
	case VAR_INSTR:
	    break;
    }
    vim_free(varp);
}

/*
 * Free the memory for a variable value and set the value to NULL or 0.
 */
    void
clear_tv(typval_T *varp)
{
    if (varp == NULL)
	return;

    switch (varp->v_type)
    {
	case VAR_FUNC:
	    func_unref(varp->vval.v_string);
	    // FALLTHROUGH
	case VAR_STRING:
	    VIM_CLEAR(varp->vval.v_string);
	    break;
	case VAR_PARTIAL:
	    partial_unref(varp->vval.v_partial);
	    varp->vval.v_partial = NULL;
	    break;
	case VAR_BLOB:
	    blob_unref(varp->vval.v_blob);
	    varp->vval.v_blob = NULL;
	    break;
	case VAR_LIST:
	    list_unref(varp->vval.v_list);
	    varp->vval.v_list = NULL;
	    break;
	case VAR_DICT:
	    dict_unref(varp->vval.v_dict);
	    varp->vval.v_dict = NULL;
	    break;
	case VAR_NUMBER:
	case VAR_BOOL:
	case VAR_SPECIAL:
	    varp->vval.v_number = 0;
	    break;
	case VAR_FLOAT:
	    varp->vval.v_float = 0.0;
	    break;
	case VAR_JOB:
#ifdef FEAT_JOB_CHANNEL
	    job_unref(varp->vval.v_job);
	    varp->vval.v_job = NULL;
#endif
	    break;
	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    channel_unref(varp->vval.v_channel);
	    varp->vval.v_channel = NULL;
#endif
	    break;
	case VAR_INSTR:
	    VIM_CLEAR(varp->vval.v_instr);
	    break;
	case VAR_CLASS:
	    class_unref(varp->vval.v_class);
	    varp->vval.v_class = NULL;
	    break;
	case VAR_OBJECT:
	    object_unref(varp->vval.v_object);
	    varp->vval.v_object = NULL;
	    break;
	case VAR_TYPEALIAS:
	    typealias_unref(varp->vval.v_typealias);
	    varp->vval.v_typealias = NULL;
	    break;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	    break;
    }
    varp->v_lock = 0;
}

/*
 * Set the value of a variable to NULL without freeing items.
 */
    void
init_tv(typval_T *varp)
{
    if (varp != NULL)
	CLEAR_POINTER(varp);
}

    static varnumber_T
tv_get_bool_or_number_chk(
	typval_T    *varp,
	int	    *denote,
	int	    want_bool,
	int	    vim9_string_error) // in Vim9 using a string is an error
{
    varnumber_T	n = 0L;

    switch (varp->v_type)
    {
	case VAR_NUMBER:
	    if (in_vim9script() && want_bool && varp->vval.v_number != 0
						   && varp->vval.v_number != 1)
	    {
		semsg(_(e_using_number_as_bool_nr), varp->vval.v_number);
		break;
	    }
	    return varp->vval.v_number;
	case VAR_FLOAT:
	    emsg(_(e_using_float_as_number));
	    break;
	case VAR_FUNC:
	case VAR_PARTIAL:
	    emsg(_(e_using_funcref_as_number));
	    break;
	case VAR_STRING:
	    if (vim9_string_error && in_vim9script())
	    {
		emsg_using_string_as(varp, !want_bool);
		break;
	    }
	    if (varp->vval.v_string != NULL)
		vim_str2nr(varp->vval.v_string, NULL, NULL,
					 STR2NR_ALL, &n, NULL, 0, FALSE, NULL);
	    return n;
	case VAR_LIST:
	    emsg(_(e_using_list_as_number));
	    break;
	case VAR_DICT:
	    emsg(_(e_using_dictionary_as_number));
	    break;
	case VAR_BOOL:
	case VAR_SPECIAL:
	    if (!want_bool && in_vim9script())
	    {
		if (varp->v_type == VAR_BOOL)
		    emsg(_(e_using_bool_as_number));
		else
		    emsg(_(e_using_special_as_number));
		break;
	    }
	    return varp->vval.v_number == VVAL_TRUE ? 1 : 0;
	case VAR_JOB:
#ifdef FEAT_JOB_CHANNEL
	    emsg(_(e_using_job_as_number));
	    break;
#endif
	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    emsg(_(e_using_channel_as_number));
	    break;
#endif
	case VAR_BLOB:
	    emsg(_(e_using_blob_as_number));
	    break;
	case VAR_CLASS:
	case VAR_TYPEALIAS:
	    check_typval_is_value(varp);
	    break;
	case VAR_OBJECT:
	    emsg(_(e_using_object_as_number));
	    break;
	case VAR_VOID:
	    emsg(_(e_cannot_use_void_value));
	    break;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_INSTR:
	    internal_error_no_abort("tv_get_number(UNKNOWN)");
	    break;
    }
    if (denote == NULL)		// useful for values that must be unsigned
	n = -1;
    else
	*denote = TRUE;
    return n;
}

/*
 * Get the number value of a variable.
 * If it is a String variable, uses vim_str2nr().
 * For incompatible types, return 0.
 * tv_get_number_chk() is similar to tv_get_number(), but informs the
 * caller of incompatible types: it sets *denote to TRUE if "denote"
 * is not NULL or returns -1 otherwise.
 */
    varnumber_T
tv_get_number(typval_T *varp)
{
    int		error = FALSE;

    return tv_get_number_chk(varp, &error);	// return 0L on error
}

/*
 * Like tv_get_number() but in Vim9 script do convert a number in a string to a
 * number without giving an error.
 */
    varnumber_T
tv_to_number(typval_T *varp)
{
    int		error = FALSE;

    return tv_get_bool_or_number_chk(varp, &error, FALSE, FALSE);
}

    varnumber_T
tv_get_number_chk(typval_T *varp, int *denote)
{
    return tv_get_bool_or_number_chk(varp, denote, FALSE, TRUE);
}

/*
 * Get the boolean value of "varp".  This is like tv_get_number_chk(),
 * but in Vim9 script accepts Number (0 and 1) and Bool/Special.
 */
    varnumber_T
tv_get_bool(typval_T *varp)
{
    return tv_get_bool_or_number_chk(varp, NULL, TRUE, TRUE);
}

/*
 * Get the boolean value of "varp".  This is like tv_get_number_chk(),
 * but in Vim9 script accepts Number and Bool.
 */
    varnumber_T
tv_get_bool_chk(typval_T *varp, int *denote)
{
    return tv_get_bool_or_number_chk(varp, denote, TRUE, TRUE);
}

    static float_T
tv_get_float_chk(typval_T *varp, int *error)
{
    switch (varp->v_type)
    {
	case VAR_NUMBER:
	    return (float_T)(varp->vval.v_number);
	case VAR_FLOAT:
	    return varp->vval.v_float;
	case VAR_FUNC:
	case VAR_PARTIAL:
	    emsg(_(e_using_funcref_as_float));
	    break;
	case VAR_STRING:
	    emsg(_(e_using_string_as_float));
	    break;
	case VAR_LIST:
	    emsg(_(e_using_list_as_float));
	    break;
	case VAR_DICT:
	    emsg(_(e_using_dictionary_as_float));
	    break;
	case VAR_BOOL:
	    emsg(_(e_using_boolean_value_as_float));
	    break;
	case VAR_SPECIAL:
	    emsg(_(e_using_special_value_as_float));
	    break;
	case VAR_JOB:
# ifdef FEAT_JOB_CHANNEL
	    emsg(_(e_using_job_as_float));
	    break;
# endif
	case VAR_CHANNEL:
# ifdef FEAT_JOB_CHANNEL
	    emsg(_(e_using_channel_as_float));
	    break;
# endif
	case VAR_BLOB:
	    emsg(_(e_using_blob_as_float));
	    break;
	case VAR_CLASS:
	case VAR_TYPEALIAS:
	    check_typval_is_value(varp);
	    break;
	case VAR_OBJECT:
	    emsg(_(e_using_object_as_float));
	    break;
	case VAR_VOID:
	    emsg(_(e_cannot_use_void_value));
	    break;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_INSTR:
	    internal_error_no_abort("tv_get_float(UNKNOWN)");
	    break;
    }
    if (error != NULL)
	*error = TRUE;
    return 0;
}

    float_T
tv_get_float(typval_T *varp)
{
    return tv_get_float_chk(varp, NULL);
}

/*
 * Give an error and return FAIL unless "args[idx]" is unknown
 */
    int
check_for_unknown_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_UNKNOWN)
    {
	semsg(_(e_too_many_arguments), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a string.
 */
    int
check_for_string_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_STRING)
    {
	semsg(_(e_string_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a non-empty string.
 */
    int
check_for_nonempty_string_arg(typval_T *args, int idx)
{
    if (check_for_string_arg(args, idx) == FAIL)
	return FAIL;
    if (args[idx].vval.v_string == NULL || *args[idx].vval.v_string == NUL)
    {
	semsg(_(e_non_empty_string_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Check for an optional string argument at 'idx'
 */
    int
check_for_opt_string_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_string_arg(args, idx) != FAIL) ? OK : FAIL;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a number.
 */
    int
check_for_number_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_NUMBER)
    {
	semsg(_(e_number_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Check for an optional number argument at 'idx'
 */
    int
check_for_opt_number_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_number_arg(args, idx) != FAIL) ? OK : FAIL;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a float or a number.
 */
    int
check_for_float_or_nr_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_FLOAT && args[idx].v_type != VAR_NUMBER)
    {
	semsg(_(e_float_or_number_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a bool.
 */
    int
check_for_bool_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_BOOL
	    && !(args[idx].v_type == VAR_NUMBER
		&& (args[idx].vval.v_number == 0
		    || args[idx].vval.v_number == 1)))
    {
	semsg(_(e_bool_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Check for an optional bool argument at 'idx'.
 * Return FAIL if the type is wrong.
 */
    int
check_for_opt_bool_arg(typval_T *args, int idx)
{
    if (args[idx].v_type == VAR_UNKNOWN)
	return OK;
    return check_for_bool_arg(args, idx);
}

/*
 * Give an error and return FAIL unless "args[idx]" is a blob.
 */
    int
check_for_blob_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_BLOB)
    {
	semsg(_(e_blob_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a list.
 */
    int
check_for_list_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_LIST)
    {
	semsg(_(e_list_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a non-NULL list.
 */
    int
check_for_nonnull_list_arg(typval_T *args, int idx)
{
    if (check_for_list_arg(args, idx) == FAIL)
	return FAIL;

    if (args[idx].vval.v_list == NULL)
    {
	semsg(_(e_non_null_list_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Check for an optional list argument at 'idx'
 */
    int
check_for_opt_list_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_list_arg(args, idx) != FAIL) ? OK : FAIL;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a dict.
 */
    int
check_for_dict_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_DICT)
    {
	semsg(_(e_dict_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a non-NULL dict.
 */
    int
check_for_nonnull_dict_arg(typval_T *args, int idx)
{
    if (check_for_dict_arg(args, idx) == FAIL)
	return FAIL;

    if (args[idx].vval.v_dict == NULL)
    {
	semsg(_(e_non_null_dict_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Check for an optional dict argument at 'idx'
 */
    int
check_for_opt_dict_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_dict_arg(args, idx) != FAIL) ? OK : FAIL;
}

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
/*
 * Give an error and return FAIL unless "args[idx]" is a channel or a job.
 */
    int
check_for_chan_or_job_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_CHANNEL && args[idx].v_type != VAR_JOB)
    {
	semsg(_(e_chan_or_job_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is an optional channel or a
 * job.
 */
    int
check_for_opt_chan_or_job_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_chan_or_job_arg(args, idx) != FAIL) ? OK : FAIL;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a job.
 */
    int
check_for_job_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_JOB)
    {
	semsg(_(e_job_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Check for an optional job argument at 'idx'.
 */
    int
check_for_opt_job_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_job_arg(args, idx) != FAIL) ? OK : FAIL;
}
#else
/*
 * Give an error and return FAIL unless "args[idx]" is an optional channel or a
 * job.  Used without the +channel feature, thus only VAR_UNKNOWN is accepted.
 */
    int
check_for_opt_chan_or_job_arg(typval_T *args, int idx)
{
    return args[idx].v_type == VAR_UNKNOWN ? OK : FAIL;
}
#endif

/*
 * Give an error and return FAIL unless "args[idx]" is a string or
 * a number.
 */
    int
check_for_string_or_number_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_STRING && args[idx].v_type != VAR_NUMBER)
    {
	semsg(_(e_string_or_number_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Check for an optional string or number argument at 'idx'.
 */
    int
check_for_opt_string_or_number_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_string_or_number_arg(args, idx) != FAIL) ? OK : FAIL;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a buffer number.
 * Buffer number can be a number or a string.
 */
    int
check_for_buffer_arg(typval_T *args, int idx)
{
    return check_for_string_or_number_arg(args, idx);
}

/*
 * Check for an optional buffer argument at 'idx'
 */
    int
check_for_opt_buffer_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_buffer_arg(args, idx) != FAIL) ? OK : FAIL;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a line number.
 * Line number can be a number or a string.
 */
    int
check_for_lnum_arg(typval_T *args, int idx)
{
    return check_for_string_or_number_arg(args, idx);
}

/*
 * Check for an optional line number argument at 'idx'
 */
    int
check_for_opt_lnum_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_lnum_arg(args, idx) != FAIL) ? OK : FAIL;
}

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
/*
 * Give an error and return FAIL unless "args[idx]" is a string or a blob.
 */
    int
check_for_string_or_blob_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_STRING && args[idx].v_type != VAR_BLOB)
    {
	semsg(_(e_string_or_blob_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}
#endif

/*
 * Give an error and return FAIL unless "args[idx]" is a string or a list.
 */
    int
check_for_string_or_list_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_STRING && args[idx].v_type != VAR_LIST)
    {
	semsg(_(e_string_or_list_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a string, a list or a
 * blob.
 */
    int
check_for_string_or_list_or_blob_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_STRING
	    && args[idx].v_type != VAR_LIST
	    && args[idx].v_type != VAR_BLOB)
    {
	semsg(_(e_string_list_or_blob_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Check for an optional string or list argument at 'idx'
 */
    int
check_for_opt_string_or_list_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_string_or_list_arg(args, idx) != FAIL) ? OK : FAIL;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a string or a dict.
 */
    int
check_for_string_or_dict_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_STRING && args[idx].v_type != VAR_DICT)
    {
	semsg(_(e_string_or_dict_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a string or a number
 * or a list.
 */
    int
check_for_string_or_number_or_list_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_STRING
	    && args[idx].v_type != VAR_NUMBER
	    && args[idx].v_type != VAR_LIST)
    {
	semsg(_(e_string_number_or_list_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is an optional string
 * or number or a list
 */
    int
check_for_opt_string_or_number_or_list_arg(typval_T *args, int idx)
{
    return (args[idx].v_type == VAR_UNKNOWN
	    || check_for_string_or_number_or_list_arg(args, idx)
							!= FAIL) ? OK : FAIL;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a string or a number
 * or a list or a blob.
 */
    int
check_for_string_or_number_or_list_or_blob_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_STRING
	    && args[idx].v_type != VAR_NUMBER
	    && args[idx].v_type != VAR_LIST
	    && args[idx].v_type != VAR_BLOB)
    {
	semsg(_(e_string_number_list_or_blob_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a string or a list
 * or a dict.
 */
    int
check_for_string_or_list_or_dict_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_STRING
	    && args[idx].v_type != VAR_LIST
	    && args[idx].v_type != VAR_DICT)
    {
	semsg(_(e_string_list_or_dict_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a string
 * or a function reference.
 */
    int
check_for_string_or_func_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_PARTIAL
	    && args[idx].v_type != VAR_FUNC
	    && args[idx].v_type != VAR_STRING)
    {
	semsg(_(e_string_or_function_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a list or a blob.
 */
    int
check_for_list_or_blob_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_LIST && args[idx].v_type != VAR_BLOB)
    {
	semsg(_(e_list_or_blob_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a list or dict
 */
    int
check_for_list_or_dict_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_LIST
	    && args[idx].v_type != VAR_DICT)
    {
	semsg(_(e_list_or_dict_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a list or dict or a
 * blob.
 */
    int
check_for_list_or_dict_or_blob_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_LIST
	    && args[idx].v_type != VAR_DICT
	    && args[idx].v_type != VAR_BLOB)
    {
	semsg(_(e_list_dict_or_blob_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a list or dict or a
 * blob or a string.
 */
    int
check_for_list_or_dict_or_blob_or_string_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_LIST
	    && args[idx].v_type != VAR_DICT
	    && args[idx].v_type != VAR_BLOB
	    && args[idx].v_type != VAR_STRING)
    {
	semsg(_(e_list_dict_blob_or_string_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is an optional buffer
 * number or a dict.
 */
    int
check_for_opt_buffer_or_dict_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_UNKNOWN
	    && args[idx].v_type != VAR_STRING
	    && args[idx].v_type != VAR_NUMBER
	    && args[idx].v_type != VAR_DICT)
    {
	semsg(_(e_string_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Give an error and return FAIL unless "args[idx]" is an object.
 */
    int
check_for_object_arg(typval_T *args, int idx)
{
    if (args[idx].v_type != VAR_OBJECT)
    {
	semsg(_(e_object_required_for_argument_nr), idx + 1);
	return FAIL;
    }
    return OK;
}

/*
 * Returns TRUE if "tv" is a type alias for a class
 */
    int
tv_class_alias(typval_T *tv)
{
    return tv->v_type == VAR_TYPEALIAS &&
			tv->vval.v_typealias->ta_type->tt_type == VAR_OBJECT;
}

/*
 * Give an error and return FAIL unless "args[idx]" is a class
 * or class typealias.
 */
    int
check_for_class_or_typealias_args(typval_T *args, int idx)
{
    for (int i = idx; args[i].v_type != VAR_UNKNOWN; ++i)
    {
	if (args[i].v_type != VAR_CLASS && !tv_class_alias(&args[idx]))
	{
	    semsg(_(e_class_or_typealias_required_for_argument_nr), i + 1);
	    return FAIL;
	}
    }
    return OK;
}

/*
 * Get the string value of a variable.
 * If it is a Number variable, the number is converted into a string.
 * tv_get_string() uses a single, static buffer.  YOU CAN ONLY USE IT ONCE!
 * tv_get_string_buf() uses a given buffer.
 * If the String variable has never been set, return an empty string.
 * Never returns NULL;
 * tv_get_string_chk() and tv_get_string_buf_chk() are similar, but return
 * NULL on error.
 */
    char_u *
tv_get_string(typval_T *varp)
{
    static char_u   mybuf[NUMBUFLEN];

    return tv_get_string_buf(varp, mybuf);
}

/*
 * Like tv_get_string() but don't allow number to string conversion for Vim9.
 */
    char_u *
tv_get_string_strict(typval_T *varp)
{
    static char_u   mybuf[NUMBUFLEN];
    char_u	    *res =  tv_get_string_buf_chk_strict(
						 varp, mybuf, in_vim9script());

    return res != NULL ? res : (char_u *)"";
}

    char_u *
tv_get_string_buf(typval_T *varp, char_u *buf)
{
    char_u	*res = tv_get_string_buf_chk(varp, buf);

    return res != NULL ? res : (char_u *)"";
}

/*
 * Careful: This uses a single, static buffer.  YOU CAN ONLY USE IT ONCE!
 */
    char_u *
tv_get_string_chk(typval_T *varp)
{
    static char_u   mybuf[NUMBUFLEN];

    return tv_get_string_buf_chk(varp, mybuf);
}

    char_u *
tv_get_string_buf_chk(typval_T *varp, char_u *buf)
{
    return tv_get_string_buf_chk_strict(varp, buf, FALSE);
}

    char_u *
tv_get_string_buf_chk_strict(typval_T *varp, char_u *buf, int strict)
{
    switch (varp->v_type)
    {
	case VAR_NUMBER:
	    if (strict)
	    {
		emsg(_(e_using_number_as_string));
		break;
	    }
	    vim_snprintf((char *)buf, NUMBUFLEN, "%lld",
					    (varnumber_T)varp->vval.v_number);
	    return buf;
	case VAR_FUNC:
	case VAR_PARTIAL:
	    emsg(_(e_using_funcref_as_string));
	    break;
	case VAR_LIST:
	    emsg(_(e_using_list_as_string));
	    break;
	case VAR_DICT:
	    emsg(_(e_using_dictionary_as_string));
	    break;
	case VAR_FLOAT:
	    if (strict)
	    {
		emsg(_(e_using_float_as_string));
		break;
	    }
	    vim_snprintf((char *)buf, NUMBUFLEN, "%g", varp->vval.v_float);
	    return buf;
	case VAR_STRING:
	    if (varp->vval.v_string != NULL)
		return varp->vval.v_string;
	    return (char_u *)"";
	case VAR_BOOL:
	case VAR_SPECIAL:
	    STRCPY(buf, get_var_special_name(varp->vval.v_number));
	    return buf;
	case VAR_BLOB:
	    emsg(_(e_using_blob_as_string));
	    break;
	case VAR_CLASS:
	case VAR_TYPEALIAS:
	    check_typval_is_value(varp);
	    break;
	case VAR_OBJECT:
	    emsg(_(e_using_object_as_string));
	    break;
	case VAR_JOB:
#ifdef FEAT_JOB_CHANNEL
	    if (in_vim9script())
	    {
		semsg(_(e_using_invalid_value_as_string_str), "job");
		break;
	    }
	    return job_to_string_buf(varp, buf);
#endif
	    break;
	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    if (in_vim9script())
	    {
		semsg(_(e_using_invalid_value_as_string_str), "channel");
		break;
	    }
	    return channel_to_string_buf(varp, buf);
#endif
	    break;
	case VAR_VOID:
	    emsg(_(e_cannot_use_void_value));
	    break;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_INSTR:
	    semsg(_(e_using_invalid_value_as_string_str),
						  vartype_name(varp->v_type));
	    break;
    }
    return NULL;
}

/*
 * Turn a typeval into a string.  Similar to tv_get_string_buf() but uses
 * string() on Dict, List, etc.
 */
    char_u *
tv_stringify(typval_T *varp, char_u *buf)
{
    if (varp->v_type == VAR_LIST
	    || varp->v_type == VAR_DICT
	    || varp->v_type == VAR_BLOB
	    || varp->v_type == VAR_FUNC
	    || varp->v_type == VAR_PARTIAL
	    || varp->v_type == VAR_FLOAT)
    {
	typval_T tmp;

	f_string(varp, &tmp);
	tv_get_string_buf(&tmp, buf);
	clear_tv(varp);
	*varp = tmp;
	return tmp.vval.v_string;
    }
    return tv_get_string_buf(varp, buf);
}

/*
 * Return TRUE if typeval "tv" and its value are set to be locked (immutable).
 * Also give an error message, using "name" or _("name") when use_gettext is
 * TRUE.
 */
    int
tv_check_lock(typval_T *tv, char_u *name, int use_gettext)
{
    int	lock = 0;

    switch (tv->v_type)
    {
	case VAR_BLOB:
	    if (tv->vval.v_blob != NULL)
		lock = tv->vval.v_blob->bv_lock;
	    break;
	case VAR_LIST:
	    if (tv->vval.v_list != NULL)
		lock = tv->vval.v_list->lv_lock;
	    break;
	case VAR_DICT:
	    if (tv->vval.v_dict != NULL)
		lock = tv->vval.v_dict->dv_lock;
	    break;
	default:
	    break;
    }
    return value_check_lock(tv->v_lock, name, use_gettext)
		   || (lock != 0 && value_check_lock(lock, name, use_gettext));
}

/*
 * Copy the values from typval_T "from" to typval_T "to".
 * When needed allocates string or increases reference count.
 * Does not make a copy of a list, blob or dict but copies the reference!
 * It is OK for "from" and "to" to point to the same item.  This is used to
 * make a copy later.
 */
    void
copy_tv(typval_T *from, typval_T *to)
{
    to->v_type = from->v_type;
    to->v_lock = 0;
    switch (from->v_type)
    {
	case VAR_NUMBER:
	case VAR_BOOL:
	case VAR_SPECIAL:
	    to->vval.v_number = from->vval.v_number;
	    break;
	case VAR_FLOAT:
	    to->vval.v_float = from->vval.v_float;
	    break;
	case VAR_JOB:
#ifdef FEAT_JOB_CHANNEL
	    to->vval.v_job = from->vval.v_job;
	    if (to->vval.v_job != NULL)
		++to->vval.v_job->jv_refcount;
	    break;
#endif
	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    to->vval.v_channel = from->vval.v_channel;
	    if (to->vval.v_channel != NULL)
		++to->vval.v_channel->ch_refcount;
	    break;
#endif
	case VAR_INSTR:
	    to->vval.v_instr = from->vval.v_instr;
	    break;

	case VAR_CLASS:
	    copy_class(from, to);
	    break;

	case VAR_OBJECT:
	    copy_object(from, to);
	    break;

	case VAR_STRING:
	case VAR_FUNC:
	    if (from->vval.v_string == NULL)
		to->vval.v_string = NULL;
	    else
	    {
		to->vval.v_string = vim_strsave(from->vval.v_string);
		if (from->v_type == VAR_FUNC)
		    func_ref(to->vval.v_string);
	    }
	    break;
	case VAR_PARTIAL:
	    if (from->vval.v_partial == NULL)
		to->vval.v_partial = NULL;
	    else
	    {
		to->vval.v_partial = from->vval.v_partial;
		++to->vval.v_partial->pt_refcount;
	    }
	    break;
	case VAR_BLOB:
	    if (from->vval.v_blob == NULL)
		to->vval.v_blob = NULL;
	    else
	    {
		to->vval.v_blob = from->vval.v_blob;
		++to->vval.v_blob->bv_refcount;
	    }
	    break;
	case VAR_LIST:
	    if (from->vval.v_list == NULL)
		to->vval.v_list = NULL;
	    else
	    {
		to->vval.v_list = from->vval.v_list;
		++to->vval.v_list->lv_refcount;
	    }
	    break;
	case VAR_DICT:
	    if (from->vval.v_dict == NULL)
		to->vval.v_dict = NULL;
	    else
	    {
		to->vval.v_dict = from->vval.v_dict;
		++to->vval.v_dict->dv_refcount;
	    }
	    break;
	case VAR_TYPEALIAS:
	    if (from->vval.v_typealias == NULL)
		to->vval.v_typealias = NULL;
	    else
	    {
		to->vval.v_typealias = from->vval.v_typealias;
		++to->vval.v_typealias->ta_refcount;
	    }
	    break;
	case VAR_VOID:
	    emsg(_(e_cannot_use_void_value));
	    break;
	case VAR_UNKNOWN:
	case VAR_ANY:
	    internal_error_no_abort("copy_tv(UNKNOWN)");
	    break;
    }
}

/*
 * Compare "tv1" and "tv2".
 * Put the result in "tv1".  Caller should clear "tv2".
 */
    int
typval_compare(
    typval_T	*tv1,	// first operand
    typval_T	*tv2,	// second operand
    exprtype_T	type,   // operator
    int		ic)     // ignore case
{
    varnumber_T	n1, n2;
    int		res = 0;
    int		type_is = type == EXPR_IS || type == EXPR_ISNOT;

    if (check_typval_is_value(tv1) == FAIL
	|| check_typval_is_value(tv2) == FAIL)
    {
	clear_tv(tv1);
	return FAIL;
    }
    else if (type_is && tv1->v_type != tv2->v_type)
    {
	// For "is" a different type always means FALSE, for "isnot"
	// it means TRUE.
	n1 = (type == EXPR_ISNOT);
    }
    else if (((tv1->v_type == VAR_SPECIAL && tv1->vval.v_number == VVAL_NULL)
		|| (tv2->v_type == VAR_SPECIAL
					   && tv2->vval.v_number == VVAL_NULL))
	    && tv1->v_type != tv2->v_type
	    && (type == EXPR_EQUAL || type == EXPR_NEQUAL))
    {
	n1 = typval_compare_null(tv1, tv2);
	if (n1 == MAYBE)
	{
	    clear_tv(tv1);
	    return FAIL;
	}
	if (type == EXPR_NEQUAL)
	    n1 = !n1;
    }
    else if (tv1->v_type == VAR_BLOB || tv2->v_type == VAR_BLOB)
    {
	if (typval_compare_blob(tv1, tv2, type, &res) == FAIL)
	{
	    clear_tv(tv1);
	    return FAIL;
	}
	n1 = res;
    }
    else if (tv1->v_type == VAR_LIST || tv2->v_type == VAR_LIST)
    {
	if (typval_compare_list(tv1, tv2, type, ic, &res) == FAIL)
	{
	    clear_tv(tv1);
	    return FAIL;
	}
	n1 = res;
    }
    else if (tv1->v_type == VAR_OBJECT || tv2->v_type == VAR_OBJECT)
    {
	if (typval_compare_object(tv1, tv2, type, ic, &res) == FAIL)
	{
	    clear_tv(tv1);
	    return FAIL;
	}
	n1 = res;
    }
    else if (tv1->v_type == VAR_DICT || tv2->v_type == VAR_DICT)
    {
	if (typval_compare_dict(tv1, tv2, type, ic, &res) == FAIL)
	{
	    clear_tv(tv1);
	    return FAIL;
	}
	n1 = res;
    }
    else if (tv1->v_type == VAR_FUNC || tv2->v_type == VAR_FUNC
	|| tv1->v_type == VAR_PARTIAL || tv2->v_type == VAR_PARTIAL)
    {
	if (typval_compare_func(tv1, tv2, type, ic, &res) == FAIL)
	{
	    clear_tv(tv1);
	    return FAIL;
	}
	n1 = res;
    }

    // If one of the two variables is a float, compare as a float.
    // When using "=~" or "!~", always compare as string.
    else if ((tv1->v_type == VAR_FLOAT || tv2->v_type == VAR_FLOAT)
	    && type != EXPR_MATCH && type != EXPR_NOMATCH)
    {
	float_T f1, f2;
	int	error = FALSE;

	f1 = tv_get_float_chk(tv1, &error);
	if (!error)
	    f2 = tv_get_float_chk(tv2, &error);
	if (error)
	{
	    clear_tv(tv1);
	    return FAIL;
	}
	n1 = FALSE;
	switch (type)
	{
	    case EXPR_IS:
	    case EXPR_EQUAL:    n1 = (f1 == f2); break;
	    case EXPR_ISNOT:
	    case EXPR_NEQUAL:   n1 = (f1 != f2); break;
	    case EXPR_GREATER:  n1 = (f1 > f2); break;
	    case EXPR_GEQUAL:   n1 = (f1 >= f2); break;
	    case EXPR_SMALLER:  n1 = (f1 < f2); break;
	    case EXPR_SEQUAL:   n1 = (f1 <= f2); break;
	    case EXPR_UNKNOWN:
	    case EXPR_MATCH:
	    default:  break;  // avoid gcc warning
	}
    }

    // If one of the two variables is a number, compare as a number.
    // When using "=~" or "!~", always compare as string.
    else if ((tv1->v_type == VAR_NUMBER || tv2->v_type == VAR_NUMBER)
	    && type != EXPR_MATCH && type != EXPR_NOMATCH)
    {
	int error = FALSE;

	n1 = tv_get_number_chk(tv1, &error);
	if (!error)
	    n2 = tv_get_number_chk(tv2, &error);
	if (error)
	{
	    clear_tv(tv1);
	    return FAIL;
	}
	switch (type)
	{
	    case EXPR_IS:
	    case EXPR_EQUAL:    n1 = (n1 == n2); break;
	    case EXPR_ISNOT:
	    case EXPR_NEQUAL:   n1 = (n1 != n2); break;
	    case EXPR_GREATER:  n1 = (n1 > n2); break;
	    case EXPR_GEQUAL:   n1 = (n1 >= n2); break;
	    case EXPR_SMALLER:  n1 = (n1 < n2); break;
	    case EXPR_SEQUAL:   n1 = (n1 <= n2); break;
	    case EXPR_UNKNOWN:
	    case EXPR_MATCH:
	    default:  break;  // avoid gcc warning
	}
    }
    else if (in_vim9script() && (tv1->v_type == VAR_BOOL
				    || tv2->v_type == VAR_BOOL
				    || (tv1->v_type == VAR_SPECIAL
					      && tv2->v_type == VAR_SPECIAL)))
    {
	if (tv1->v_type != tv2->v_type)
	{
	    semsg(_(e_cannot_compare_str_with_str),
		       vartype_name(tv1->v_type), vartype_name(tv2->v_type));
	    clear_tv(tv1);
	    return FAIL;
	}
	n1 = tv1->vval.v_number;
	n2 = tv2->vval.v_number;
	switch (type)
	{
	    case EXPR_IS:
	    case EXPR_EQUAL:    n1 = (n1 == n2); break;
	    case EXPR_ISNOT:
	    case EXPR_NEQUAL:   n1 = (n1 != n2); break;
	    default:
		semsg(_(e_invalid_operation_for_str),
						   vartype_name(tv1->v_type));
		clear_tv(tv1);
		return FAIL;
	}
    }
#ifdef FEAT_JOB_CHANNEL
    else if (tv1->v_type == tv2->v_type
	    && (tv1->v_type == VAR_CHANNEL || tv1->v_type == VAR_JOB)
	    && (type == EXPR_NEQUAL || type == EXPR_EQUAL))
    {
	if (tv1->v_type == VAR_CHANNEL)
	    n1 = tv1->vval.v_channel == tv2->vval.v_channel;
	else
	    n1 = tv1->vval.v_job == tv2->vval.v_job;
	if (type == EXPR_NEQUAL)
	    n1 = !n1;
    }
#endif
    else
    {
	if (typval_compare_string(tv1, tv2, type, ic, &res) == FAIL)
	{
	    clear_tv(tv1);
	    return FAIL;
	}
	n1 = res;
    }
    clear_tv(tv1);
    if (in_vim9script())
    {
	tv1->v_type = VAR_BOOL;
	tv1->vval.v_number = n1 ? VVAL_TRUE : VVAL_FALSE;
    }
    else
    {
	tv1->v_type = VAR_NUMBER;
	tv1->vval.v_number = n1;
    }

    return OK;
}

/*
 * Compare "tv1" to "tv2" as lists according to "type" and "ic".
 * Put the result, false or true, in "res".
 * Return FAIL and give an error message when the comparison can't be done.
 */
    int
typval_compare_list(
	typval_T    *tv1,
	typval_T    *tv2,
	exprtype_T  type,
	int	    ic,
	int	    *res)
{
    int	    val = 0;

    if (type == EXPR_IS || type == EXPR_ISNOT)
    {
	val = (tv1->v_type == tv2->v_type
				      && tv1->vval.v_list == tv2->vval.v_list);
	if (type == EXPR_ISNOT)
	    val = !val;
    }
    else if (tv1->v_type != tv2->v_type
	    || (type != EXPR_EQUAL && type != EXPR_NEQUAL))
    {
	if (tv1->v_type != tv2->v_type)
	    emsg(_(e_can_only_compare_list_with_list));
	else
	    emsg(_(e_invalid_operation_for_list));
	return FAIL;
    }
    else
    {
	val = list_equal(tv1->vval.v_list, tv2->vval.v_list,
							ic, FALSE);
	if (type == EXPR_NEQUAL)
	    val = !val;
    }
    *res = val;
    return OK;
}

/*
 * Compare v:null with another type.  Return TRUE if the value is NULL.
 */
    int
typval_compare_null(typval_T *tv1, typval_T *tv2)
{
    if ((tv1->v_type == VAR_SPECIAL && tv1->vval.v_number == VVAL_NULL)
	    || (tv2->v_type == VAR_SPECIAL && tv2->vval.v_number == VVAL_NULL))
    {
	typval_T	*tv = tv1->v_type == VAR_SPECIAL ? tv2 : tv1;

	switch (tv->v_type)
	{
	    case VAR_BLOB: return tv->vval.v_blob == NULL;
#ifdef FEAT_JOB_CHANNEL
	    case VAR_CHANNEL: return tv->vval.v_channel == NULL;
#endif
	    // TODO: null_class handling
	    // case VAR_CLASS: return tv->vval.v_class == NULL;
	    case VAR_DICT: return tv->vval.v_dict == NULL;
	    case VAR_FUNC: return tv->vval.v_string == NULL;
#ifdef FEAT_JOB_CHANNEL
	    case VAR_JOB: return tv->vval.v_job == NULL;
#endif
	    case VAR_LIST: return tv->vval.v_list == NULL;
	    case VAR_OBJECT: return tv->vval.v_object == NULL;
	    case VAR_PARTIAL: return tv->vval.v_partial == NULL;
	    case VAR_STRING: return tv->vval.v_string == NULL;

	    case VAR_NUMBER: if (!in_vim9script())
				 return tv->vval.v_number == 0;
			     break;
	    case VAR_FLOAT: if (!in_vim9script())
				 return tv->vval.v_float == 0.0;
			     break;
	    case VAR_TYPEALIAS: return tv->vval.v_typealias == NULL;
	    default: break;
	}
    }
    // although comparing null with number, float or bool is not very useful
    // we won't give an error
    return FALSE;
}

/*
 * Compare "tv1" to "tv2" as blobs according to "type".
 * Put the result, false or true, in "res".
 * Return FAIL and give an error message when the comparison can't be done.
 */
    int
typval_compare_blob(
	typval_T    *tv1,
	typval_T    *tv2,
	exprtype_T  type,
	int	    *res)
{
    int	    val = 0;

    if (type == EXPR_IS || type == EXPR_ISNOT)
    {
	val = (tv1->v_type == tv2->v_type
			&& tv1->vval.v_blob == tv2->vval.v_blob);
	if (type == EXPR_ISNOT)
	    val = !val;
    }
    else if (tv1->v_type != tv2->v_type
	    || (type != EXPR_EQUAL && type != EXPR_NEQUAL))
    {
	if (tv1->v_type != tv2->v_type)
	    emsg(_(e_can_only_compare_blob_with_blob));
	else
	    emsg(_(e_invalid_operation_for_blob));
	return FAIL;
    }
    else
    {
	val = blob_equal(tv1->vval.v_blob, tv2->vval.v_blob);
	if (type == EXPR_NEQUAL)
	    val = !val;
    }
    *res = val;
    return OK;
}

/*
 * Compare "tv1" to "tv2" as classes according to "type".
 * Put the result, false or true, in "res".
 * Return FAIL and give an error message when the comparison can't be done.
 */
    int
typval_compare_class(
	typval_T    *tv1,
	typval_T    *tv2,
	exprtype_T  type UNUSED,
	int	    ic UNUSED,
	int	    *res)
{
    // TODO: use "type"
    *res = tv1->vval.v_class == tv2->vval.v_class;
    return OK;
}

/*
 * Compare "tv1" to "tv2" as objects according to "type".
 * Put the result, false or true, in "res".
 * Return FAIL and give an error message when the comparison can't be done.
 */
    int
typval_compare_object(
	typval_T    *tv1,
	typval_T    *tv2,
	exprtype_T  type,
	int	    ic,
	int	    *res)
{
    int res_match = type == EXPR_EQUAL || type == EXPR_IS ? TRUE : FALSE;

    if (tv1->vval.v_object == NULL && tv2->vval.v_object == NULL)
    {
	*res = res_match;
	return OK;
    }
    if (tv1->vval.v_object == NULL || tv2->vval.v_object == NULL)
    {
	*res = !res_match;
	return OK;
    }

    class_T *cl1 = tv1->vval.v_object->obj_class;
    class_T *cl2 = tv2->vval.v_object->obj_class;
    if (cl1 != cl2 || cl1 == NULL || cl2 == NULL)
    {
	*res = !res_match;
	return OK;
    }

    object_T *obj1 = tv1->vval.v_object;
    object_T *obj2 = tv2->vval.v_object;
    if (type == EXPR_IS || type == EXPR_ISNOT)
    {
	*res = obj1 == obj2 ? res_match : !res_match;
	return OK;
    }

    for (int i = 0; i < cl1->class_obj_member_count; ++i)
	if (!tv_equal((typval_T *)(obj1 + 1) + i,
				 (typval_T *)(obj2 + 1) + i, ic, TRUE))
	{
	    *res = !res_match;
	    return OK;
	}
    *res = res_match;
    return OK;
}

/*
 * Compare "tv1" to "tv2" as dictionaries according to "type" and "ic".
 * Put the result, false or true, in "res".
 * Return FAIL and give an error message when the comparison can't be done.
 */
    int
typval_compare_dict(
	typval_T    *tv1,
	typval_T    *tv2,
	exprtype_T  type,
	int	    ic,
	int	    *res)
{
    int	    val;

    if (type == EXPR_IS || type == EXPR_ISNOT)
    {
	val = (tv1->v_type == tv2->v_type
			&& tv1->vval.v_dict == tv2->vval.v_dict);
	if (type == EXPR_ISNOT)
	    val = !val;
    }
    else if (tv1->v_type != tv2->v_type
		|| (type != EXPR_EQUAL && type != EXPR_NEQUAL))
    {
	if (tv1->v_type != tv2->v_type)
	    emsg(_(e_can_only_compare_dictionary_with_dictionary));
	else
	    emsg(_(e_invalid_operation_for_dictionary));
	return FAIL;
    }
    else
    {
	val = dict_equal(tv1->vval.v_dict, tv2->vval.v_dict, ic, FALSE);
	if (type == EXPR_NEQUAL)
	    val = !val;
    }
    *res = val;
    return OK;
}

/*
 * Compare "tv1" to "tv2" as funcrefs according to "type" and "ic".
 * Put the result, false or true, in "res".
 * Return FAIL and give an error message when the comparison can't be done.
 */
    int
typval_compare_func(
	typval_T    *tv1,
	typval_T    *tv2,
	exprtype_T  type,
	int	    ic,
	int	    *res)
{
    int	    val = 0;

    if (type != EXPR_EQUAL && type != EXPR_NEQUAL
	    && type != EXPR_IS && type != EXPR_ISNOT)
    {
	emsg(_(e_invalid_operation_for_funcrefs));
	return FAIL;
    }
    if ((tv1->v_type == VAR_PARTIAL && tv1->vval.v_partial == NULL)
	    || (tv2->v_type == VAR_PARTIAL && tv2->vval.v_partial == NULL))
	// When both partials are NULL, then they are equal.
	// Otherwise they are not equal.
	val = (tv1->vval.v_partial == tv2->vval.v_partial);
    else if (type == EXPR_IS || type == EXPR_ISNOT)
    {
	if (tv1->v_type == VAR_FUNC && tv2->v_type == VAR_FUNC)
	    // strings are considered the same if their value is
	    // the same
	    val = tv_equal(tv1, tv2, ic, FALSE);
	else if (tv1->v_type == VAR_PARTIAL && tv2->v_type == VAR_PARTIAL)
	    val = (tv1->vval.v_partial == tv2->vval.v_partial);
	else
	    val = FALSE;
    }
    else
	val = tv_equal(tv1, tv2, ic, FALSE);
    if (type == EXPR_NEQUAL || type == EXPR_ISNOT)
	val = !val;
    *res = val;
    return OK;
}

/*
 * Compare "tv1" to "tv2" as strings according to "type" and "ic".
 * Put the result, false or true, in "res".
 * Return FAIL and give an error message when the comparison can't be done.
 */
    int
typval_compare_string(
	typval_T    *tv1,
	typval_T    *tv2,
	exprtype_T  type,
	int	    ic,
	int	    *res)
{
    int		i = 0;
    int		val = FALSE;
    char_u	*s1, *s2;
    char_u	buf1[NUMBUFLEN], buf2[NUMBUFLEN];

    if (in_vim9script()
	  && ((tv1->v_type != VAR_STRING && tv1->v_type != VAR_SPECIAL)
	   || (tv2->v_type != VAR_STRING && tv2->v_type != VAR_SPECIAL)))
    {
	semsg(_(e_cannot_compare_str_with_str),
		   vartype_name(tv1->v_type), vartype_name(tv2->v_type));
	return FAIL;
    }
    s1 = tv_get_string_buf(tv1, buf1);
    s2 = tv_get_string_buf(tv2, buf2);
    if (type != EXPR_MATCH && type != EXPR_NOMATCH)
	i = ic ? MB_STRICMP(s1, s2) : STRCMP(s1, s2);
    switch (type)
    {
	case EXPR_IS:	    if (in_vim9script())
			    {
				// Really check it is the same string, not just
				// the same value.
				val = tv1->vval.v_string == tv2->vval.v_string;
				break;
			    }
			    // FALLTHROUGH
	case EXPR_EQUAL:    val = (i == 0); break;
	case EXPR_ISNOT:    if (in_vim9script())
			    {
				// Really check it is not the same string, not
				// just a different value.
				val = tv1->vval.v_string != tv2->vval.v_string;
				break;
			    }
			    // FALLTHROUGH
	case EXPR_NEQUAL:   val = (i != 0); break;
	case EXPR_GREATER:  val = (i > 0); break;
	case EXPR_GEQUAL:   val = (i >= 0); break;
	case EXPR_SMALLER:  val = (i < 0); break;
	case EXPR_SEQUAL:   val = (i <= 0); break;

	case EXPR_MATCH:
	case EXPR_NOMATCH:
		val = pattern_match(s2, s1, ic);
		if (type == EXPR_NOMATCH)
		    val = !val;
		break;

	default:  break;  // avoid gcc warning
    }
    *res = val;
    return OK;
}
/*
 * Convert any type to a string, never give an error.
 * When "quotes" is TRUE add quotes to a string.
 * Returns an allocated string.
 */
    char_u *
typval_tostring(typval_T *arg, int quotes)
{
    char_u	*tofree;
    char_u	numbuf[NUMBUFLEN];
    char_u	*ret = NULL;

    if (arg == NULL)
	return vim_strsave((char_u *)"(does not exist)");
    if (!quotes && arg->v_type == VAR_STRING)
    {
	ret = vim_strsave(arg->vval.v_string == NULL ? (char_u *)""
							 : arg->vval.v_string);
    }
    else
    {
	ret = tv2string(arg, &tofree, numbuf, 0);
	// Make a copy if we have a value but it's not in allocated memory.
	if (ret != NULL && tofree == NULL)
	    ret = vim_strsave(ret);
    }
    return ret;
}

/*
 * Return TRUE if typeval "tv" is locked: Either that value is locked itself
 * or it refers to a List or Dictionary that is locked.
 */
    int
tv_islocked(typval_T *tv)
{
    return (tv->v_lock & VAR_LOCKED)
	|| (tv->v_type == VAR_LIST
		&& tv->vval.v_list != NULL
		&& (tv->vval.v_list->lv_lock & VAR_LOCKED))
	|| (tv->v_type == VAR_DICT
		&& tv->vval.v_dict != NULL
		&& (tv->vval.v_dict->dv_lock & VAR_LOCKED));
}

    static int
func_equal(
    typval_T *tv1,
    typval_T *tv2,
    int	     ic)	    // ignore case
{
    char_u	*s1, *s2;
    dict_T	*d1, *d2;
    int		a1, a2;
    int		i;

    // empty and NULL function name considered the same
    s1 = tv1->v_type == VAR_FUNC ? tv1->vval.v_string
					   : partial_name(tv1->vval.v_partial);
    if (s1 != NULL && *s1 == NUL)
	s1 = NULL;
    s2 = tv2->v_type == VAR_FUNC ? tv2->vval.v_string
					   : partial_name(tv2->vval.v_partial);
    if (s2 != NULL && *s2 == NUL)
	s2 = NULL;
    if (s1 == NULL || s2 == NULL)
    {
	if (s1 != s2)
	    return FALSE;
    }
    else if (STRCMP(s1, s2) != 0)
	return FALSE;

    // empty dict and NULL dict is different
    d1 = tv1->v_type == VAR_FUNC ? NULL : tv1->vval.v_partial->pt_dict;
    d2 = tv2->v_type == VAR_FUNC ? NULL : tv2->vval.v_partial->pt_dict;
    if (d1 == NULL || d2 == NULL)
    {
	if (d1 != d2)
	    return FALSE;
    }
    else if (!dict_equal(d1, d2, ic, TRUE))
	return FALSE;

    // empty list and no list considered the same
    a1 = tv1->v_type == VAR_FUNC ? 0 : tv1->vval.v_partial->pt_argc;
    a2 = tv2->v_type == VAR_FUNC ? 0 : tv2->vval.v_partial->pt_argc;
    if (a1 != a2)
	return FALSE;
    for (i = 0; i < a1; ++i)
	if (!tv_equal(tv1->vval.v_partial->pt_argv + i,
		      tv2->vval.v_partial->pt_argv + i, ic, TRUE))
	    return FALSE;

    return TRUE;
}

/*
 * Return TRUE if "tv1" and "tv2" have the same value.
 * Compares the items just like "==" would compare them, but strings and
 * numbers are different.  Floats and numbers are also different.
 */
    int
tv_equal(
    typval_T *tv1,
    typval_T *tv2,
    int	     ic,	    // ignore case
    int	     recursive)	    // TRUE when used recursively
{
    char_u	buf1[NUMBUFLEN], buf2[NUMBUFLEN];
    char_u	*s1, *s2;
    static int  recursive_cnt = 0;	    // catch recursive loops
    int		r;
    static int	tv_equal_recurse_limit;

    // Catch lists and dicts that have an endless loop by limiting
    // recursiveness to a limit.  We guess they are equal then.
    // A fixed limit has the problem of still taking an awful long time.
    // Reduce the limit every time running into it. That should work fine for
    // deeply linked structures that are not recursively linked and catch
    // recursiveness quickly.
    if (!recursive)
	tv_equal_recurse_limit = 1000;
    if (recursive_cnt >= tv_equal_recurse_limit)
    {
	--tv_equal_recurse_limit;
	return TRUE;
    }

    // For VAR_FUNC and VAR_PARTIAL compare the function name, bound dict and
    // arguments.
    if ((tv1->v_type == VAR_FUNC
		|| (tv1->v_type == VAR_PARTIAL && tv1->vval.v_partial != NULL))
	    && (tv2->v_type == VAR_FUNC
		|| (tv2->v_type == VAR_PARTIAL && tv2->vval.v_partial != NULL)))
    {
	++recursive_cnt;
	r = func_equal(tv1, tv2, ic);
	--recursive_cnt;
	return r;
    }

    if (tv1->v_type != tv2->v_type
	    && ((tv1->v_type != VAR_BOOL && tv1->v_type != VAR_SPECIAL)
		|| (tv2->v_type != VAR_BOOL && tv2->v_type != VAR_SPECIAL)))
	return FALSE;

    switch (tv1->v_type)
    {
	case VAR_LIST:
	    ++recursive_cnt;
	    r = list_equal(tv1->vval.v_list, tv2->vval.v_list, ic, TRUE);
	    --recursive_cnt;
	    return r;

	case VAR_DICT:
	    ++recursive_cnt;
	    r = dict_equal(tv1->vval.v_dict, tv2->vval.v_dict, ic, TRUE);
	    --recursive_cnt;
	    return r;

	case VAR_BLOB:
	    return blob_equal(tv1->vval.v_blob, tv2->vval.v_blob);

	case VAR_NUMBER:
	case VAR_BOOL:
	case VAR_SPECIAL:
	    return tv1->vval.v_number == tv2->vval.v_number;

	case VAR_STRING:
	    s1 = tv_get_string_buf(tv1, buf1);
	    s2 = tv_get_string_buf(tv2, buf2);
	    return ((ic ? MB_STRICMP(s1, s2) : STRCMP(s1, s2)) == 0);

	case VAR_FLOAT:
	    return tv1->vval.v_float == tv2->vval.v_float;
	case VAR_JOB:
#ifdef FEAT_JOB_CHANNEL
	    return tv1->vval.v_job == tv2->vval.v_job;
#endif
	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    return tv1->vval.v_channel == tv2->vval.v_channel;
#endif
	case VAR_INSTR:
	    return tv1->vval.v_instr == tv2->vval.v_instr;

	case VAR_CLASS:
	    // A class only exists once, equality is identity.
	    return tv1->vval.v_class == tv2->vval.v_class;

	case VAR_OBJECT:
	    (void)typval_compare_object(tv1, tv2, EXPR_EQUAL, ic, &r);
	    return r;

	case VAR_PARTIAL:
	    return tv1->vval.v_partial == tv2->vval.v_partial;

	case VAR_FUNC:
	    return tv1->vval.v_string == tv2->vval.v_string;

	case VAR_TYPEALIAS:
	    return tv1->vval.v_typealias == tv2->vval.v_typealias;

	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	    break;
    }

    // VAR_UNKNOWN can be the result of a invalid expression, let's say it
    // does not equal anything, not even itself.
    return FALSE;
}

/*
 * Get an option value.
 * "arg" points to the '&' or '+' before the option name.
 * "arg" is advanced to character after the option name.
 * Return OK or FAIL.
 */
    int
eval_option(
    char_u	**arg,
    typval_T	*rettv,	// when NULL, only check if option exists
    int		evaluate)
{
    char_u	*option_end;
    long	numval;
    char_u	*stringval;
    getoption_T	opt_type;
    int		c;
    int		working = (**arg == '+');    // has("+option")
    int		ret = OK;
    int		scope;

    // Isolate the option name and find its value.
    option_end = find_option_end(arg, &scope);
    if (option_end == NULL)
    {
	if (rettv != NULL)
	    semsg(_(e_option_name_missing_str), *arg);
	return FAIL;
    }

    if (!evaluate)
    {
	*arg = option_end;
	return OK;
    }

    c = *option_end;
    *option_end = NUL;
    opt_type = get_option_value(*arg, &numval,
			       rettv == NULL ? NULL : &stringval, NULL, scope);

    if (opt_type == gov_unknown)
    {
	if (rettv != NULL)
	    semsg(_(e_unknown_option_str), *arg);
	ret = FAIL;
    }
    else if (rettv != NULL)
    {
	rettv->v_lock = 0;
	if (opt_type == gov_hidden_string)
	{
	    rettv->v_type = VAR_STRING;
	    rettv->vval.v_string = NULL;
	}
	else if (opt_type == gov_hidden_bool || opt_type == gov_hidden_number)
	{
	    rettv->v_type = in_vim9script() && opt_type == gov_hidden_bool
						       ? VAR_BOOL : VAR_NUMBER;
	    rettv->vval.v_number = 0;
	}
	else if (opt_type == gov_bool || opt_type == gov_number)
	{
	    if (in_vim9script() && opt_type == gov_bool)
	    {
		rettv->v_type = VAR_BOOL;
		rettv->vval.v_number = numval ? VVAL_TRUE : VVAL_FALSE;
	    }
	    else
	    {
		rettv->v_type = VAR_NUMBER;
		rettv->vval.v_number = numval;
	    }
	}
	else				// string option
	{
	    rettv->v_type = VAR_STRING;
	    rettv->vval.v_string = stringval;
	}
    }
    else if (working && (opt_type == gov_hidden_bool
			|| opt_type == gov_hidden_number
			|| opt_type == gov_hidden_string))
	ret = FAIL;

    *option_end = c;		    // put back for error messages
    *arg = option_end;

    return ret;
}

/*
 * Allocate a variable for a number constant.  Also deals with "0z" for blob.
 * Return OK or FAIL.
 */
    int
eval_number(
	char_u	    **arg,
	typval_T    *rettv,
	int	    evaluate,
	int	    want_string UNUSED)
{
    int		len;
    int		skip_quotes = !in_old_script(4);
    char_u	*p;
    int		get_float = FALSE;

    // We accept a float when the format matches
    // "[0-9]\+\.[0-9]\+\([eE][+-]\?[0-9]\+\)\?".  This is very
    // strict to avoid backwards compatibility problems.
    // With script version 2 and later the leading digit can be
    // omitted.
    // Don't look for a float after the "." operator, so that
    // ":let vers = 1.2.3" doesn't fail.
    if (**arg == '.')
	p = *arg;
    else
    {
	p = *arg + 1;
	if (skip_quotes)
	    for (;;)
	    {
		if (*p == '\'')
		    ++p;
		if (!vim_isdigit(*p))
		    break;
		p = skipdigits(p);
	    }
	else
	    p = skipdigits(p);
    }
    if (!want_string && p[0] == '.' && vim_isdigit(p[1]))
    {
	get_float = TRUE;
	p = skipdigits(p + 2);
	if (*p == 'e' || *p == 'E')
	{
	    ++p;
	    if (*p == '-' || *p == '+')
		++p;
	    if (!vim_isdigit(*p))
		get_float = FALSE;
	    else
		p = skipdigits(p + 1);
	}
	if (ASCII_ISALPHA(*p) || *p == '.')
	    get_float = FALSE;
    }
    if (get_float)
    {
	float_T	f;

	*arg += string2float(*arg, &f, skip_quotes);
	if (evaluate)
	{
	    rettv->v_type = VAR_FLOAT;
	    rettv->vval.v_float = f;
	}
    }
    else
    if (**arg == '0' && ((*arg)[1] == 'z' || (*arg)[1] == 'Z'))
    {
	char_u  *bp;
	blob_T  *blob = NULL;  // init for gcc

	// Blob constant: 0z0123456789abcdef
	if (evaluate)
	    blob = blob_alloc();
	for (bp = *arg + 2; vim_isxdigit(bp[0]); bp += 2)
	{
	    if (!vim_isxdigit(bp[1]))
	    {
		if (blob != NULL)
		{
		    emsg(_(e_blob_literal_should_have_an_even_number_of_hex_characters));
		    ga_clear(&blob->bv_ga);
		    VIM_CLEAR(blob);
		}
		return FAIL;
	    }
	    if (blob != NULL)
		ga_append(&blob->bv_ga,
			     (hex2nr(*bp) << 4) + hex2nr(*(bp+1)));
	    if (bp[2] == '.' && vim_isxdigit(bp[3]))
		++bp;
	}
	if (blob != NULL)
	    rettv_blob_set(rettv, blob);
	*arg = bp;
    }
    else
    {
	varnumber_T	n;

	// decimal, hex or octal number
	vim_str2nr(*arg, NULL, &len, skip_quotes
		      ? STR2NR_NO_OCT + STR2NR_QUOTE
		      : STR2NR_ALL, &n, NULL, 0, TRUE, NULL);
	if (len == 0)
	{
	    if (evaluate)
		semsg(_(e_invalid_expression_str), *arg);
	    return FAIL;
	}
	*arg += len;
	if (evaluate)
	{
	    rettv->v_type = VAR_NUMBER;
	    rettv->vval.v_number = n;
	}
    }
    return OK;
}

/*
 * Evaluate a string constant and put the result in "rettv".
 * "*arg" points to the double quote or to after it when "interpolate" is TRUE.
 * When "interpolate" is TRUE reduce "{{" to "{", reduce "}}" to "}" and stop
 * at a single "{".
 * Return OK or FAIL.
 */
    int
eval_string(char_u **arg, typval_T *rettv, int evaluate, int interpolate)
{
    char_u	*p;
    char_u	*end;
    int		extra = interpolate ? 1 : 0;
    int		off = interpolate ? 0 : 1;
    int		len;

    // Find the end of the string, skipping backslashed characters.
    for (p = *arg + off; *p != NUL && *p != '"'; MB_PTR_ADV(p))
    {
	if (*p == '\\' && p[1] != NUL)
	{
	    ++p;
	    // A "\<x>" form occupies at least 4 characters, and produces up
	    // to 9 characters (6 for the char and 3 for a modifier):
	    // reserve space for 5 extra.
	    if (*p == '<')
	    {
		int		modifiers = 0;
		int		flags = FSK_KEYCODE | FSK_IN_STRING;

		extra += 5;

		// Skip to the '>' to avoid using '{' inside for string
		// interpolation.
		if (p[1] != '*')
		    flags |= FSK_SIMPLIFY;
		if (find_special_key(&p, &modifiers, flags, NULL) != 0)
		    --p;  // leave "p" on the ">"
	    }
	}
	else if (interpolate && (*p == '{' || *p == '}'))
	{
	    if (*p == '{' && p[1] != '{') // start of expression
		break;
	    ++p;
	    if (p[-1] == '}' && *p != '}') // single '}' is an error
	    {
		semsg(_(e_stray_closing_curly_str), *arg);
		return FAIL;
	    }
	    --extra;  // "{{" becomes "{", "}}" becomes "}"
	}
    }

    if (*p != '"' && !(interpolate && *p == '{'))
    {
	semsg(_(e_missing_double_quote_str), *arg);
	return FAIL;
    }

    // If only parsing, set *arg and return here
    if (!evaluate)
    {
	*arg = p + off;
	return OK;
    }

    // Copy the string into allocated memory, handling backslashed
    // characters.
    rettv->v_type = VAR_STRING;
    len = (int)(p - *arg + extra);
    rettv->vval.v_string = alloc(len);
    if (rettv->vval.v_string == NULL)
	return FAIL;
    end = rettv->vval.v_string;

    for (p = *arg + off; *p != NUL && *p != '"'; )
    {
	if (*p == '\\')
	{
	    switch (*++p)
	    {
		case 'b': *end++ = BS; ++p; break;
		case 'e': *end++ = ESC; ++p; break;
		case 'f': *end++ = FF; ++p; break;
		case 'n': *end++ = NL; ++p; break;
		case 'r': *end++ = CAR; ++p; break;
		case 't': *end++ = TAB; ++p; break;

		case 'X': // hex: "\x1", "\x12"
		case 'x':
		case 'u': // Unicode: "\u0023"
		case 'U':
			  if (vim_isxdigit(p[1]))
			  {
			      int	n, nr;
			      int	c = SAFE_toupper(*p);

			      if (c == 'X')
				  n = 2;
			      else if (*p == 'u')
				  n = 4;
			      else
				  n = 8;
			      nr = 0;
			      while (--n >= 0 && vim_isxdigit(p[1]))
			      {
				  ++p;
				  nr = (nr << 4) + hex2nr(*p);
			      }
			      ++p;
			      // For "\u" store the number according to
			      // 'encoding'.
			      if (c != 'X')
				  end += (*mb_char2bytes)(nr, end);
			      else
				  *end++ = nr;
			  }
			  break;

			  // octal: "\1", "\12", "\123"
		case '0':
		case '1':
		case '2':
		case '3':
		case '4':
		case '5':
		case '6':
		case '7': *end = *p++ - '0';
			  if (*p >= '0' && *p <= '7')
			  {
			      *end = (*end << 3) + *p++ - '0';
			      if (*p >= '0' && *p <= '7')
				  *end = (*end << 3) + *p++ - '0';
			  }
			  ++end;
			  break;

			  // Special key, e.g.: "\<C-W>"
		case '<':
			  {
			      int flags = FSK_KEYCODE | FSK_IN_STRING;

			      if (p[1] != '*')
				  flags |= FSK_SIMPLIFY;
			      extra = trans_special(&p, end, flags, FALSE, NULL);
			      if (extra != 0)
			      {
				  end += extra;
				  if (end >= rettv->vval.v_string + len)
				      iemsg("eval_string() used more space than allocated");
				  break;
			      }
			  }
			  // FALLTHROUGH

		default: MB_COPY_CHAR(p, end);
			  break;
	    }
	}
	else
	{
	    if (interpolate && (*p == '{' || *p == '}'))
	    {
		if (*p == '{' && p[1] != '{') // start of expression
		    break;
		++p;  // reduce "{{" to "{" and "}}" to "}"
	    }
	    MB_COPY_CHAR(p, end);
	}
    }
    *end = NUL;
    if (*p == '"' && !interpolate)
	++p;
    *arg = p;

    return OK;
}

/*
 * Allocate a variable for a 'str''ing' constant.
 * When "interpolate" is TRUE reduce "{{" to "{" and stop at a single "{".
 * Return OK when a "rettv" was set to the string.
 * Return FAIL on error, "rettv" is not set.
 */
    int
eval_lit_string(char_u **arg, typval_T *rettv, int evaluate, int interpolate)
{
    char_u	*p;
    char_u	*str;
    int		reduce = interpolate ? -1 : 0;
    int		off = interpolate ? 0 : 1;

    // Find the end of the string, skipping ''.
    for (p = *arg + off; *p != NUL; MB_PTR_ADV(p))
    {
	if (*p == '\'')
	{
	    if (p[1] != '\'')
		break;
	    ++reduce;
	    ++p;
	}
	else if (interpolate)
	{
	    if (*p == '{')
	    {
		if (p[1] != '{')
		    break;
		++p;
		++reduce;
	    }
	    else if (*p == '}')
	    {
		++p;
		if (*p != '}')
		{
		    semsg(_(e_stray_closing_curly_str), *arg);
		    return FAIL;
		}
		++reduce;
	    }
	}
    }

    if (*p != '\'' && !(interpolate && *p == '{'))
    {
	semsg(_(e_missing_single_quote_str), *arg);
	return FAIL;
    }

    // If only parsing return after setting "*arg"
    if (!evaluate)
    {
	*arg = p + off;
	return OK;
    }

    // Copy the string into allocated memory, handling '' to ' reduction and
    // any expressions.
    str = alloc((p - *arg) - reduce);
    if (str == NULL)
	return FAIL;
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = str;

    for (p = *arg + off; *p != NUL; )
    {
	if (*p == '\'')
	{
	    if (p[1] != '\'')
		break;
	    ++p;
	}
	else if (interpolate && (*p == '{' || *p == '}'))
	{
	    if (*p == '{' && p[1] != '{')
		break;
	    ++p;
	}
	MB_COPY_CHAR(p, str);
    }
    *str = NUL;
    *arg = p + off;

    return OK;
}

/*
 * Evaluate a single or double quoted string possibly containing expressions.
 * "arg" points to the '$'.  The result is put in "rettv".
 * Returns OK or FAIL.
 */
    int
eval_interp_string(char_u **arg, typval_T *rettv, int evaluate)
{
    typval_T	tv;
    int		ret = OK;
    int		quote;
    garray_T	ga;
    char_u	*p;

    ga_init2(&ga, 1, 80);

    // *arg is on the '$' character, move it to the first string character.
    ++*arg;
    quote = **arg;
    ++*arg;

    for (;;)
    {
	// Get the string up to the matching quote or to a single '{'.
	// "arg" is advanced to either the quote or the '{'.
	if (quote == '"')
	    ret = eval_string(arg, &tv, evaluate, TRUE);
	else
	    ret = eval_lit_string(arg, &tv, evaluate, TRUE);
	if (ret == FAIL)
	    break;
	if (evaluate)
	{
	    ga_concat(&ga, tv.vval.v_string);
	    clear_tv(&tv);
	}

	if (**arg != '{')
	{
	    // found terminating quote
	    ++*arg;
	    break;
	}
	p = eval_one_expr_in_str(*arg, &ga, evaluate);
	if (p == NULL)
	{
	    ret = FAIL;
	    break;
	}
	*arg = p;
    }

    rettv->v_type = VAR_STRING;
    if (ret == FAIL || !evaluate || ga_append(&ga, NUL) == FAIL)
    {
	ga_clear(&ga);
	rettv->vval.v_string = NULL;
	return ret;
    }

    rettv->vval.v_string = ga.ga_data;
    return OK;
}

/*
 * Return a string with the string representation of a variable.
 * If the memory is allocated "tofree" is set to it, otherwise NULL.
 * "numbuf" is used for a number.
 * Puts quotes around strings, so that they can be parsed back by eval().
 * May return NULL.
 */
    char_u *
tv2string(
    typval_T	*tv,
    char_u	**tofree,
    char_u	*numbuf,
    int		copyID)
{
    return echo_string_core(tv, tofree, numbuf, copyID, FALSE, TRUE, FALSE);
}

/*
 * Get the value of an environment variable.
 * "arg" is pointing to the '$'.  It is advanced to after the name.
 * If the environment variable was not set, silently assume it is empty.
 * Return FAIL if the name is invalid.
 */
    int
eval_env_var(char_u **arg, typval_T *rettv, int evaluate)
{
    char_u	*string = NULL;
    int		len;
    int		cc;
    char_u	*name;
    int		mustfree = FALSE;

    ++*arg;
    name = *arg;
    len = get_env_len(arg);
    if (evaluate)
    {
	if (len == 0)
	    return FAIL; // invalid empty name

	cc = name[len];
	name[len] = NUL;
	// first try vim_getenv(), fast for normal environment vars
	string = vim_getenv(name, &mustfree);
	if (string != NULL && *string != NUL)
	{
	    if (!mustfree)
		string = vim_strsave(string);
	}
	else
	{
	    if (mustfree)
		vim_free(string);

	    // next try expanding things like $VIM and ${HOME}
	    string = expand_env_save(name - 1);
	    if (string != NULL && *string == '$')
		VIM_CLEAR(string);
	}
	name[len] = cc;

	rettv->v_type = VAR_STRING;
	rettv->vval.v_string = string;
	rettv->v_lock = 0;
    }

    return OK;
}

/*
 * Get the lnum from the first argument.
 * Also accepts ".", "$", etc., but that only works for the current buffer.
 * Returns -1 on error.
 */
    linenr_T
tv_get_lnum(typval_T *argvars)
{
    linenr_T	lnum = -1;
    int		did_emsg_before = did_emsg;

    if (argvars[0].v_type != VAR_STRING || !in_vim9script())
	lnum = (linenr_T)tv_get_number_chk(&argvars[0], NULL);
    if (lnum <= 0 && did_emsg_before == did_emsg
					    && argvars[0].v_type != VAR_NUMBER)
    {
	int	fnum;
	pos_T	*fp;

	// no valid number, try using arg like line()
	fp = var2fpos(&argvars[0], TRUE, &fnum, FALSE);
	if (fp != NULL)
	    lnum = fp->lnum;
    }
    return lnum;
}

/*
 * Get the lnum from the first argument.
 * Also accepts "$", then "buf" is used.
 * Returns 0 on error.
 */
    linenr_T
tv_get_lnum_buf(typval_T *argvars, buf_T *buf)
{
    if (argvars[0].v_type == VAR_STRING
	    && argvars[0].vval.v_string != NULL
	    && argvars[0].vval.v_string[0] == '$'
	    && argvars[0].vval.v_string[1] == NUL
	    && buf != NULL)
	return buf->b_ml.ml_line_count;
    return (linenr_T)tv_get_number_chk(&argvars[0], NULL);
}

/*
 * Get buffer by number or pattern.
 */
    buf_T *
tv_get_buf(typval_T *tv, int curtab_only)
{
    char_u	*name = tv->vval.v_string;
    buf_T	*buf;

    if (tv->v_type == VAR_NUMBER)
	return buflist_findnr((int)tv->vval.v_number);
    if (tv->v_type != VAR_STRING)
	return NULL;
    if (name == NULL || *name == NUL)
	return curbuf;
    if (name[0] == '$' && name[1] == NUL)
	return lastbuf;

    buf = buflist_find_by_name(name, curtab_only);

    // If not found, try expanding the name, like done for bufexists().
    if (buf == NULL)
	buf = find_buffer(tv);

    return buf;
}

/*
 * Like tv_get_buf() but give an error message is the type is wrong.
 */
    buf_T *
tv_get_buf_from_arg(typval_T *tv)
{
    buf_T *buf;

    ++emsg_off;
    buf = tv_get_buf(tv, FALSE);
    --emsg_off;
    if (buf == NULL
	    && tv->v_type != VAR_NUMBER
	    && tv->v_type != VAR_STRING)
	// issue errmsg for type error
	(void)tv_get_number(tv);
    return buf;
}

#endif // FEAT_EVAL
