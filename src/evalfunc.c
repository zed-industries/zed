/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * evalfunc.c: Builtin functions
 */
#define USING_FLOAT_STUFF

#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

#ifdef VMS
# include <float.h>
#endif

static void f_and(typval_T *argvars, typval_T *rettv);
#ifdef FEAT_BEVAL
static void f_balloon_gettext(typval_T *argvars, typval_T *rettv);
static void f_balloon_show(typval_T *argvars, typval_T *rettv);
# if defined(FEAT_BEVAL_TERM)
static void f_balloon_split(typval_T *argvars, typval_T *rettv);
# endif
#endif
static void f_byte2line(typval_T *argvars, typval_T *rettv);
static void f_call(typval_T *argvars, typval_T *rettv);
static void f_changenr(typval_T *argvars, typval_T *rettv);
static void f_char2nr(typval_T *argvars, typval_T *rettv);
static void f_charcol(typval_T *argvars, typval_T *rettv);
static void f_col(typval_T *argvars, typval_T *rettv);
static void f_confirm(typval_T *argvars, typval_T *rettv);
static void f_copy(typval_T *argvars, typval_T *rettv);
static void f_cursor(typval_T *argsvars, typval_T *rettv);
#ifdef MSWIN
static void f_debugbreak(typval_T *argvars, typval_T *rettv);
#endif
static void f_deepcopy(typval_T *argvars, typval_T *rettv);
static void f_did_filetype(typval_T *argvars, typval_T *rettv);
static void f_echoraw(typval_T *argvars, typval_T *rettv);
static void f_empty(typval_T *argvars, typval_T *rettv);
static void f_environ(typval_T *argvars, typval_T *rettv);
static void f_err_teapot(typval_T *argvars, typval_T *rettv);
static void f_escape(typval_T *argvars, typval_T *rettv);
static void f_eval(typval_T *argvars, typval_T *rettv);
static void f_eventhandler(typval_T *argvars, typval_T *rettv);
static void f_execute(typval_T *argvars, typval_T *rettv);
static void f_exists_compiled(typval_T *argvars, typval_T *rettv);
static void f_expand(typval_T *argvars, typval_T *rettv);
static void f_expandcmd(typval_T *argvars, typval_T *rettv);
static void f_feedkeys(typval_T *argvars, typval_T *rettv);
static void f_fnameescape(typval_T *argvars, typval_T *rettv);
static void f_foreground(typval_T *argvars, typval_T *rettv);
static void f_funcref(typval_T *argvars, typval_T *rettv);
static void f_function(typval_T *argvars, typval_T *rettv);
static void f_garbagecollect(typval_T *argvars, typval_T *rettv);
static void f_get(typval_T *argvars, typval_T *rettv);
static void f_getchangelist(typval_T *argvars, typval_T *rettv);
static void f_getcharpos(typval_T *argvars, typval_T *rettv);
static void f_getcharsearch(typval_T *argvars, typval_T *rettv);
static void f_getenv(typval_T *argvars, typval_T *rettv);
static void f_getfontname(typval_T *argvars, typval_T *rettv);
static void f_getjumplist(typval_T *argvars, typval_T *rettv);
static void f_getpid(typval_T *argvars, typval_T *rettv);
static void f_getcurpos(typval_T *argvars, typval_T *rettv);
static void f_getcursorcharpos(typval_T *argvars, typval_T *rettv);
static void f_getpos(typval_T *argvars, typval_T *rettv);
static void f_getreg(typval_T *argvars, typval_T *rettv);
static void f_getreginfo(typval_T *argvars, typval_T *rettv);
static void f_getregtype(typval_T *argvars, typval_T *rettv);
static void f_gettagstack(typval_T *argvars, typval_T *rettv);
static void f_gettext(typval_T *argvars, typval_T *rettv);
static void f_haslocaldir(typval_T *argvars, typval_T *rettv);
static void f_hlID(typval_T *argvars, typval_T *rettv);
static void f_hlexists(typval_T *argvars, typval_T *rettv);
static void f_hostname(typval_T *argvars, typval_T *rettv);
static void f_index(typval_T *argvars, typval_T *rettv);
static void f_indexof(typval_T *argvars, typval_T *rettv);
static void f_input(typval_T *argvars, typval_T *rettv);
static void f_inputdialog(typval_T *argvars, typval_T *rettv);
static void f_inputlist(typval_T *argvars, typval_T *rettv);
static void f_inputrestore(typval_T *argvars, typval_T *rettv);
static void f_inputsave(typval_T *argvars, typval_T *rettv);
static void f_inputsecret(typval_T *argvars, typval_T *rettv);
static void f_interrupt(typval_T *argvars, typval_T *rettv);
static void f_invert(typval_T *argvars, typval_T *rettv);
static void f_islocked(typval_T *argvars, typval_T *rettv);
static void f_keytrans(typval_T *argvars, typval_T *rettv);
static void f_last_buffer_nr(typval_T *argvars, typval_T *rettv);
static void f_libcall(typval_T *argvars, typval_T *rettv);
static void f_libcallnr(typval_T *argvars, typval_T *rettv);
static void f_line(typval_T *argvars, typval_T *rettv);
static void f_line2byte(typval_T *argvars, typval_T *rettv);
#ifdef FEAT_LUA
static void f_luaeval(typval_T *argvars, typval_T *rettv);
#endif
static void f_match(typval_T *argvars, typval_T *rettv);
static void f_matchbufline(typval_T *argvars, typval_T *rettv);
static void f_matchend(typval_T *argvars, typval_T *rettv);
static void f_matchlist(typval_T *argvars, typval_T *rettv);
static void f_matchstr(typval_T *argvars, typval_T *rettv);
static void f_matchstrlist(typval_T *argvars, typval_T *rettv);
static void f_matchstrpos(typval_T *argvars, typval_T *rettv);
static void f_max(typval_T *argvars, typval_T *rettv);
static void f_min(typval_T *argvars, typval_T *rettv);
#ifdef FEAT_MZSCHEME
static void f_mzeval(typval_T *argvars, typval_T *rettv);
#endif
static void f_nextnonblank(typval_T *argvars, typval_T *rettv);
static void f_nr2char(typval_T *argvars, typval_T *rettv);
static void f_or(typval_T *argvars, typval_T *rettv);
#ifdef FEAT_PERL
static void f_perleval(typval_T *argvars, typval_T *rettv);
#endif
static void f_prevnonblank(typval_T *argvars, typval_T *rettv);
static void f_printf(typval_T *argvars, typval_T *rettv);
static void f_pum_getpos(typval_T *argvars, typval_T *rettv);
static void f_pumvisible(typval_T *argvars, typval_T *rettv);
#ifdef FEAT_PYTHON3
static void f_py3eval(typval_T *argvars, typval_T *rettv);
#endif
#ifdef FEAT_PYTHON
static void f_pyeval(typval_T *argvars, typval_T *rettv);
#endif
#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3)
static void f_pyxeval(typval_T *argvars, typval_T *rettv);
#endif
static void f_test_srand_seed(typval_T *argvars, typval_T *rettv);
static void f_rand(typval_T *argvars, typval_T *rettv);
static void f_range(typval_T *argvars, typval_T *rettv);
static void f_reg_executing(typval_T *argvars, typval_T *rettv);
static void f_reg_recording(typval_T *argvars, typval_T *rettv);
static void f_rename(typval_T *argvars, typval_T *rettv);
static void f_repeat(typval_T *argvars, typval_T *rettv);
#ifdef FEAT_RUBY
static void f_rubyeval(typval_T *argvars, typval_T *rettv);
#endif
static void f_screenattr(typval_T *argvars, typval_T *rettv);
static void f_screenchar(typval_T *argvars, typval_T *rettv);
static void f_screenchars(typval_T *argvars, typval_T *rettv);
static void f_screencol(typval_T *argvars, typval_T *rettv);
static void f_screenrow(typval_T *argvars, typval_T *rettv);
static void f_screenstring(typval_T *argvars, typval_T *rettv);
static void f_search(typval_T *argvars, typval_T *rettv);
static void f_searchdecl(typval_T *argvars, typval_T *rettv);
static void f_searchpair(typval_T *argvars, typval_T *rettv);
static void f_searchpairpos(typval_T *argvars, typval_T *rettv);
static void f_searchpos(typval_T *argvars, typval_T *rettv);
static void f_setcharpos(typval_T *argvars, typval_T *rettv);
static void f_setcharsearch(typval_T *argvars, typval_T *rettv);
static void f_setcursorcharpos(typval_T *argvars, typval_T *rettv);
static void f_setenv(typval_T *argvars, typval_T *rettv);
static void f_setfperm(typval_T *argvars, typval_T *rettv);
static void f_setpos(typval_T *argvars, typval_T *rettv);
static void f_setreg(typval_T *argvars, typval_T *rettv);
static void f_settagstack(typval_T *argvars, typval_T *rettv);
#ifdef FEAT_CRYPT
static void f_sha256(typval_T *argvars, typval_T *rettv);
#endif
static void f_shellescape(typval_T *argvars, typval_T *rettv);
static void f_shiftwidth(typval_T *argvars, typval_T *rettv);
static void f_soundfold(typval_T *argvars, typval_T *rettv);
static void f_spellbadword(typval_T *argvars, typval_T *rettv);
static void f_spellsuggest(typval_T *argvars, typval_T *rettv);
static void f_split(typval_T *argvars, typval_T *rettv);
static void f_srand(typval_T *argvars, typval_T *rettv);
static void f_submatch(typval_T *argvars, typval_T *rettv);
static void f_substitute(typval_T *argvars, typval_T *rettv);
static void f_swapfilelist(typval_T *argvars, typval_T *rettv);
static void f_swapinfo(typval_T *argvars, typval_T *rettv);
static void f_swapname(typval_T *argvars, typval_T *rettv);
static void f_synID(typval_T *argvars, typval_T *rettv);
static void f_synIDattr(typval_T *argvars, typval_T *rettv);
static void f_synIDtrans(typval_T *argvars, typval_T *rettv);
static void f_synstack(typval_T *argvars, typval_T *rettv);
static void f_synconcealed(typval_T *argvars, typval_T *rettv);
static void f_tabpagebuflist(typval_T *argvars, typval_T *rettv);
static void f_taglist(typval_T *argvars, typval_T *rettv);
static void f_tagfiles(typval_T *argvars, typval_T *rettv);
static void f_type(typval_T *argvars, typval_T *rettv);
static void f_virtcol(typval_T *argvars, typval_T *rettv);
static void f_visualmode(typval_T *argvars, typval_T *rettv);
static void f_wildmenumode(typval_T *argvars, typval_T *rettv);
static void f_windowsversion(typval_T *argvars, typval_T *rettv);
static void f_wordcount(typval_T *argvars, typval_T *rettv);
static void f_xor(typval_T *argvars, typval_T *rettv);


/*
 * Functions that check the argument type of a builtin function.
 * Each function returns FAIL and gives an error message if the type is wrong.
 */

// Context passed to an arg_ function.
typedef struct {
    int		arg_count;	// actual argument count
    type2_T	*arg_types;	// list of argument types
    int		arg_idx;	// current argument index (first arg is zero)
    cctx_T	*arg_cctx;
} argcontext_T;

// A function to check one argument type.  The first argument is the type to
// check.  If needed, other argument types can be obtained with the context.
// E.g. if "arg_idx" is 1, then (type - 1) is the first argument type.
// NOTE:    Use "arg_any", not NULL, in funcentry_T.f_argcheck array
//	    to accept an argument of any type.
typedef int (*argcheck_T)(type_T *, type_T *, argcontext_T *);

/*
 * Call need_type() to check an argument type.
 */
    static int
check_arg_type(
	type_T		*expected,
	type_T		*actual,
	argcontext_T	*context)
{
    return need_type(actual, expected, FALSE,
	    context->arg_idx - context->arg_count, context->arg_idx + 1,
	    context->arg_cctx, FALSE, FALSE);
}

/*
 * Call need_type() to check an argument type and that it is modifiable
 */
    static int
check_arg_type_mod(
	type_T		*expected,
	type_T		*actual,
	argcontext_T	*context)
{
    if (need_type(actual, expected, FALSE,
	    context->arg_idx - context->arg_count, context->arg_idx + 1,
	    context->arg_cctx, FALSE, FALSE) == FAIL)
	return FAIL;
    return arg_type_modifiable(actual, context->arg_idx + 1);
}

/*
 * Give an error if "type" is a constant.
 */
    int
arg_type_modifiable(type_T *type, int arg_idx)
{
    char *tofree;

    if ((type->tt_flags & TTFLAG_CONST) == 0)
	return OK;
    semsg(_(e_argument_nr_trying_to_modify_const_str),
	    arg_idx, type_name(type, &tofree));
    vim_free(tofree);
    return FAIL;
}

/*
 *  Return OK for any type unconditionally.
 */
    static int
arg_any(type_T *type UNUSED,
	type_T *decl_type UNUSED,
	argcontext_T *context UNUSED)
{
    return OK;
}

/*
 * Check "type" is a float or a number.
 */
    static int
arg_float_or_nr(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_FLOAT
	    || type->tt_type == VAR_NUMBER
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_number, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a number.
 */
    static int
arg_number(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    return check_arg_type(&t_number, type, context);
}

/*
 * Check "type" is an object.
 */
    static int
arg_object(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_OBJECT
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_object, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a dict of 'any'.
 */
    static int
arg_dict_any(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    return check_arg_type(&t_dict_any, type, context);
}

/*
 * Check "type" is a list of 'any'.
 */
    static int
arg_list_any(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    return check_arg_type(&t_list_any, type, context);
}

/*
 * Check "type" is a list of 'any' and modifiable
 */
    static int
arg_list_any_mod(
	type_T	     *type,
	type_T	     *decl_type UNUSED,
	argcontext_T *context)
{
    return check_arg_type_mod(&t_list_any, type, context);
}

/*
 * Check "type" is a list of numbers.
 */
    static int
arg_list_number(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    return check_arg_type(&t_list_number, type, context);
}

/*
 * Check "type" is a list of strings.
 */
    static int
arg_list_string(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    return check_arg_type(&t_list_string, type, context);
}

/*
 * Check "type" is a string.
 */
    static int
arg_string(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    return check_arg_type(&t_string, type, context);
}

/*
 * Check "type" is a blob
 */
    static int
arg_blob(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    return check_arg_type(&t_blob, type, context);
}

/*
 * Check "type" is a bool or number 0 or 1.
 */
    static int
arg_bool(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    return check_arg_type(&t_bool, type, context);
}

/*
 * Check "type" is a list of 'any' or a blob.
 */
    static int
arg_list_or_blob(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_LIST
	    || type->tt_type == VAR_BLOB
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_list_any, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a modifiable list of 'any' or a blob.
 */
    static int
arg_list_or_blob_mod(
	type_T	     *type,
	type_T	     *decl_type,
	argcontext_T *context)
{
    if (arg_list_or_blob(type, decl_type, context) == FAIL)
	return FAIL;
    return arg_type_modifiable(type, context->arg_idx + 1);
}

/*
 * Check "type" is a string or a number
 */
    static int
arg_string_or_nr(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_NUMBER
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a buffer (string or a number)
 */
    static int
arg_buffer(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_NUMBER
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a buffer or a dict of any
 */
    static int
arg_buffer_or_dict_any(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_NUMBER
	    || type->tt_type == VAR_DICT
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a line (string or a number)
 */
    static int
arg_lnum(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_NUMBER
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a string or a list of strings.
 */
    static int
arg_string_or_list_string(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type_any_or_unknown(type))
	return OK;
    if (type->tt_type != VAR_LIST)
    {
	arg_type_mismatch(&t_string, type, context->arg_idx + 1);
	return FAIL;
    }
    if (type->tt_member->tt_type == VAR_ANY
		    || type->tt_member->tt_type == VAR_STRING)
	return OK;

    arg_type_mismatch(&t_list_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a string or a list of 'any'
 */
    static int
arg_string_or_list_any(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_LIST
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a string or a dict of 'any'
 */
    static int
arg_string_or_dict_any(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_DICT
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a string or a blob
 */
    static int
arg_string_or_blob(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_BLOB
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a list of 'any' or a dict of 'any'.
 */
    static int
arg_list_or_dict(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_LIST
	    || type->tt_type == VAR_DICT
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_list_any, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a list of 'any' or a dict of 'any'.  And modifiable.
 */
    static int
arg_list_or_dict_mod(
	type_T	     *type,
	type_T	     *decl_type,
	argcontext_T *context)
{
    if (arg_list_or_dict(type, decl_type, context) == FAIL)
	return FAIL;
    return arg_type_modifiable(type, context->arg_idx + 1);
}

/*
 * Check "type" is a list of 'any' or a dict of 'any' or a blob.
 * Also check if "type" is modifiable.
 */
    static int
arg_list_or_dict_or_blob_mod(
	type_T	     *type,
	type_T	     *decl_type UNUSED,
	argcontext_T *context)
{
    if (type->tt_type == VAR_LIST
	    || type->tt_type == VAR_DICT
	    || type->tt_type == VAR_BLOB
	    || type_any_or_unknown(type))
	return arg_type_modifiable(type, context->arg_idx + 1);
    arg_type_mismatch(&t_list_any, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a list of 'any' or a dict of 'any' or a blob or a string.
 */
    static int
arg_list_or_dict_or_blob_or_string(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_LIST
	    || type->tt_type == VAR_DICT
	    || type->tt_type == VAR_BLOB
	    || type->tt_type == VAR_STRING
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_list_any, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a list of 'any' or a dict of 'any' or a blob or a string.
 * Also check the value is modifiable.
 */
    static int
arg_list_or_dict_or_blob_or_string_mod(
	type_T	     *type,
	type_T	     *decl_type,
	argcontext_T *context)
{
    if (arg_list_or_dict_or_blob_or_string(type, decl_type, context) == FAIL)
	return FAIL;
    return arg_type_modifiable(type, context->arg_idx + 1);
}

/*
 * Check second argument of map(), filter(), foreach().
 */
    static int
check_map_filter_arg2(type_T *type, argcontext_T *context,
							filtermap_T filtermap)
{
    type_T *expected_member = NULL;
    type_T *(args[2]);
    type_T t_func_exp = {VAR_FUNC, 2, 0, 0, NULL, NULL, args};

    if (context->arg_types[0].type_curr->tt_type == VAR_LIST
	    || context->arg_types[0].type_curr->tt_type == VAR_DICT)
    {
	// Use the declared type if possible, so that an error is given if
	// a declared list changes type, but not if a constant list changes
	// type.
	if (context->arg_types[0].type_decl->tt_type == VAR_LIST
		|| context->arg_types[0].type_decl->tt_type == VAR_DICT)
	    expected_member = context->arg_types[0].type_decl->tt_member;
	else
	    expected_member = context->arg_types[0].type_curr->tt_member;
    }
    else if (context->arg_types[0].type_curr->tt_type == VAR_STRING)
	expected_member = &t_string;
    else if (context->arg_types[0].type_curr->tt_type == VAR_BLOB)
	expected_member = &t_number;

    args[0] = NULL;
    args[1] = &t_unknown;
    if (type->tt_argcount != -1)
    {
	if (!(type->tt_argcount == 2 || (type->tt_argcount == 1
				    && (type->tt_flags & TTFLAG_VARARGS))))
	{
	    emsg(_(e_invalid_number_of_arguments));
	    return FAIL;
	}
	if (type->tt_flags & TTFLAG_VARARGS)
	    // check the argument types at runtime
	    t_func_exp.tt_argcount = -1;
	else
	{
	    if (context->arg_types[0].type_curr->tt_type == VAR_STRING
		    || context->arg_types[0].type_curr->tt_type == VAR_BLOB
		    || context->arg_types[0].type_curr->tt_type == VAR_LIST)
		args[0] = &t_number;
	    else if (context->arg_types[0].type_decl->tt_type == VAR_DICT)
		args[0] = &t_string;
	    if (args[0] != NULL)
		args[1] = expected_member;
	}
    }

    if (!type_any_or_unknown(type->tt_member) || args[0] != NULL)
    {
	where_T where = WHERE_INIT;

	if (filtermap == FILTERMAP_MAP)
	    t_func_exp.tt_member = expected_member == NULL
					|| type_any_or_unknown(type->tt_member)
				? &t_any : expected_member;
	else if (filtermap == FILTERMAP_FILTER)
	    t_func_exp.tt_member = &t_bool;
	else // filtermap == FILTERMAP_FOREACH
	    t_func_exp.tt_member = &t_unknown;
	if (args[0] == NULL)
	    args[0] = &t_unknown;
	if (type->tt_argcount == -1)
	    t_func_exp.tt_argcount = -1;

	where.wt_index = 2;
	where.wt_kind = WT_ARGUMENT;
	return check_type(&t_func_exp, type, TRUE, where);
    }
    return OK;
}

/*
 * Check second argument of filter(): func must return a bool.
 */
    static int
arg_filter_func(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_PARTIAL
	    || type_any_or_unknown(type))
	return OK;

    if (type->tt_type == VAR_FUNC)
	return check_map_filter_arg2(type, context, FILTERMAP_FILTER);
    semsg(_(e_string_or_function_required_for_argument_nr), 2);
    return FAIL;
}

/*
 * Check second argument of map(), the function.
 */
    static int
arg_map_func(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_PARTIAL
	    || type_any_or_unknown(type))
	return OK;

    if (type->tt_type == VAR_FUNC)
	return check_map_filter_arg2(type, context, FILTERMAP_MAP);
    semsg(_(e_string_or_function_required_for_argument_nr), 2);
    return FAIL;
}

/*
 * Check second argument of foreach(), the function.
 */
    static int
arg_foreach_func(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_PARTIAL
	    || type_any_or_unknown(type))
	return OK;

    if (type->tt_type == VAR_FUNC)
	return check_map_filter_arg2(type, context, FILTERMAP_FOREACH);
    semsg(_(e_string_or_function_required_for_argument_nr), 2);
    return FAIL;
}

/*
 * Check second argument of sort() and uniq(), the "how" argument.
 */
    static int
arg_sort_how(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_PARTIAL
	    || type_any_or_unknown(type))
	return OK;

    if (type->tt_type == VAR_FUNC)
    {
	type_T *(args[2]);
	type_T t_func_exp = {VAR_FUNC, 2, 0, 0, &t_number, NULL, args};

	if (context->arg_types[0].type_curr->tt_type == VAR_LIST)
	    args[0] = context->arg_types[0].type_curr->tt_member;
	else
	    args[0] = &t_unknown;
	if (!type_any_or_unknown(type->tt_member) || args[0] != &t_unknown)
	{
	    where_T where = WHERE_INIT;

	    args[1] = args[0];
	    if (type->tt_argcount == -1)
		t_func_exp.tt_argcount = -1;
	    where.wt_index = 2;
	    where.wt_kind = WT_ARGUMENT;
	    return check_type(&t_func_exp, type, TRUE, where);
	}

	return OK;
    }
    semsg(_(e_string_or_function_required_for_argument_nr), 2);
    return FAIL;
}

/*
 * Check an expression argument, can be a string, funcref or partial.
 * Also accept a bool, a constant resulting from compiling a string argument.
 * Also accept a number, one and zero are accepted.
 */
    static int
arg_string_or_func(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_PARTIAL
	    || type->tt_type == VAR_FUNC
	    || type->tt_type == VAR_BOOL
	    || type->tt_type == VAR_NUMBER
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_func_any, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check varargs' "type" are class.
 */
    static int
varargs_class(type_T *type UNUSED,
	      type_T *decl_type UNUSED,
	      argcontext_T *context)
{
    for (int i = context->arg_idx; i < context->arg_count; ++i)
    {
	type2_T *types = &context->arg_types[i];
	if (types->type_curr->tt_type != VAR_CLASS)
	{
	    semsg(_(e_class_or_typealias_required_for_argument_nr), i + 1);
	    return FAIL;
	}
    }
    return OK;
}

/*
 * Check "type" is a list of 'any' or a blob or a string.
 */
    static int
arg_string_list_or_blob(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_LIST
	    || type->tt_type == VAR_BLOB
	    || type->tt_type == VAR_STRING
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_list_any, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a modifiable list of 'any' or a blob or a string.
 */
    static int
arg_string_list_or_blob_mod(type_T *type, type_T *decl_type, argcontext_T *context)
{
    if (arg_string_list_or_blob(type, decl_type, context) == FAIL)
	return FAIL;
    return arg_type_modifiable(type, context->arg_idx + 1);
}

/*
 * Check "type" is a job.
 */
    static int
arg_job(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    return check_arg_type(&t_job, type, context);
}

/*
 * Check "type" is a channel or a job.
 */
    static int
arg_chan_or_job(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_CHANNEL
	    || type->tt_type == VAR_JOB
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_channel, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" can be used as the type_decl of the previous argument.
 * Must not be used for the first argcheck_T entry.
 */
    static int
arg_same_as_prev(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    type_T *prev_type = context->arg_types[context->arg_idx - 1].type_decl;

    return check_arg_type(prev_type, type, context);
}

/*
 * Check "type" is the same basic type as the previous argument, checks list or
 * dict vs other type, but not member type.
 * Must not be used for the first argcheck_T entry.
 */
    static int
arg_same_struct_as_prev(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    type_T *prev_type = context->arg_types[context->arg_idx - 1].type_curr;

    if (prev_type->tt_type != context->arg_types[context->arg_idx].type_curr->tt_type)
	return check_arg_type(prev_type, type, context);
    return OK;
}

/*
 * Check "type" is an item of the list or blob of the previous arg.
 * Must not be used for the first argcheck_T entry.
 */
    static int
arg_item_of_prev(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    type_T *prev_type = context->arg_types[context->arg_idx - 1].type_curr;
    type_T *expected;

    if (prev_type->tt_type == VAR_LIST)
	expected = prev_type->tt_member;
    else if (prev_type->tt_type == VAR_BLOB)
	expected = &t_number;
    else
	// probably VAR_ANY, can't check
	return OK;

    return check_arg_type(expected, type, context);
}

/*
 * Check "type" is a string or a number or a list
 */
    static int
arg_str_or_nr_or_list(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_NUMBER
	    || type->tt_type == VAR_LIST
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" is a dict of 'any' or a string
 */
    static int
arg_dict_any_or_string(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_DICT
	    || type->tt_type == VAR_STRING
	    || type_any_or_unknown(type))
	return OK;
    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" which is the third argument of extend() (number or string or
 * any)
 */
    static int
arg_extend3(type_T *type, type_T *decl_type, argcontext_T *context)
{
    type_T *first_type = context->arg_types[context->arg_idx - 2].type_curr;

    if (first_type->tt_type == VAR_LIST)
	return arg_number(type, decl_type, context);
    if (first_type->tt_type == VAR_DICT)
	return arg_string(type, decl_type, context);
    return OK;
}

/*
 * Check "type" which is the first argument of get() (blob or list or dict or
 * funcref)
 */
    static int
arg_get1(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_BLOB
	    || type->tt_type == VAR_LIST
	    || type->tt_type == VAR_DICT
	    || type->tt_type == VAR_FUNC
	    || type->tt_type == VAR_PARTIAL
	    || type_any_or_unknown(type))
	return OK;

    arg_type_mismatch(&t_list_any, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" which is the first argument of len() (number or string or
 * blob or list or dict)
 */
    static int
arg_len1(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_NUMBER
	    || type->tt_type == VAR_BLOB
	    || type->tt_type == VAR_LIST
	    || type->tt_type == VAR_DICT
	    || type_any_or_unknown(type))
	return OK;

    arg_type_mismatch(&t_list_any, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" which is the second argument of remove() (number or string or
 * any)
 */
    static int
arg_remove2(type_T *type, type_T *decl_type, argcontext_T *context)
{
    type_T *first_type = context->arg_types[context->arg_idx - 1].type_curr;

    if (first_type->tt_type == VAR_LIST || first_type->tt_type == VAR_BLOB)
	return arg_number(type, decl_type, context);
    if (first_type->tt_type == VAR_DICT)
	return arg_string_or_nr(type, decl_type, context);
    return OK;
}

/*
 * Check "type" which is the first argument of repeat() (string or number or
 * list or any)
 */
    static int
arg_repeat1(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_NUMBER
	    || type->tt_type == VAR_BLOB
	    || type->tt_type == VAR_LIST
	    || type_any_or_unknown(type))
	return OK;

    arg_type_mismatch(&t_string, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" which is the first argument of slice() (list or blob or string
 * or any)
 */
    static int
arg_slice1(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_LIST
	    || type->tt_type == VAR_BLOB
	    || type->tt_type == VAR_STRING
	    || type_any_or_unknown(type))
	return OK;

    arg_type_mismatch(&t_list_any, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" which is the first argument of count() (string or list or dict
 * or any)
 */
    static int
arg_string_or_list_or_dict(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_STRING
	    || type->tt_type == VAR_LIST
	    || type->tt_type == VAR_DICT
	    || type_any_or_unknown(type))
	return OK;

    semsg(_(e_string_list_or_dict_required_for_argument_nr),
							 context->arg_idx + 1);
    return FAIL;
}

/*
 * Check "type" which is the first argument of cursor() (number or string or
 * list or any)
 */
    static int
arg_cursor1(type_T *type, type_T *decl_type UNUSED, argcontext_T *context)
{
    if (type->tt_type == VAR_NUMBER
	    || type->tt_type == VAR_STRING
	    || type->tt_type == VAR_LIST
	    || type_any_or_unknown(type))
	return OK;

    arg_type_mismatch(&t_number, type, context->arg_idx + 1);
    return FAIL;
}

/*
 * Lists of functions that check the argument types of a builtin function.
 */
static argcheck_T arg1_blob[] = {arg_blob};
static argcheck_T arg1_bool[] = {arg_bool};
static argcheck_T arg1_buffer[] = {arg_buffer};
static argcheck_T arg1_buffer_or_dict_any[] = {arg_buffer_or_dict_any};
static argcheck_T arg1_chan_or_job[] = {arg_chan_or_job};
static argcheck_T arg1_dict_any[] = {arg_dict_any};
static argcheck_T arg1_dict_or_string[] = {arg_dict_any_or_string};
static argcheck_T arg1_float_or_nr[] = {arg_float_or_nr};
static argcheck_T arg1_job[] = {arg_job};
static argcheck_T arg1_list_any[] = {arg_list_any};
static argcheck_T arg1_list_number[] = {arg_list_number};
static argcheck_T arg1_string_or_list_or_blob_mod[] = {arg_string_list_or_blob_mod};
static argcheck_T arg1_list_or_dict[] = {arg_list_or_dict};
static argcheck_T arg1_list_string[] = {arg_list_string};
static argcheck_T arg1_string_or_list_or_dict[] = {arg_string_or_list_or_dict};
static argcheck_T arg1_lnum[] = {arg_lnum};
static argcheck_T arg1_number[] = {arg_number};
static argcheck_T arg1_string[] = {arg_string};
static argcheck_T arg1_string_or_list_any[] = {arg_string_or_list_any};
static argcheck_T arg1_string_or_list_string[] = {arg_string_or_list_string};
static argcheck_T arg1_string_or_nr[] = {arg_string_or_nr};
static argcheck_T arg2_any_buffer[] = {arg_any, arg_buffer};
static argcheck_T arg2_buffer_any[] = {arg_buffer, arg_any};
static argcheck_T arg2_buffer_bool[] = {arg_buffer, arg_bool};
static argcheck_T arg2_buffer_list_any[] = {arg_buffer, arg_list_any};
static argcheck_T arg2_buffer_lnum[] = {arg_buffer, arg_lnum};
static argcheck_T arg2_buffer_number[] = {arg_buffer, arg_number};
static argcheck_T arg2_buffer_string[] = {arg_buffer, arg_string};
static argcheck_T arg2_chan_or_job_dict[] = {arg_chan_or_job, arg_dict_any};
static argcheck_T arg2_chan_or_job_string[] = {arg_chan_or_job, arg_string};
static argcheck_T arg2_dict_any_list_any[] = {arg_dict_any, arg_list_any};
static argcheck_T arg2_dict_any_string_or_nr[] = {arg_dict_any, arg_string_or_nr};
static argcheck_T arg2_dict_string[] = {arg_dict_any, arg_string};
static argcheck_T arg2_float_or_nr[] = {arg_float_or_nr, arg_float_or_nr};
static argcheck_T arg2_job_dict[] = {arg_job, arg_dict_any};
static argcheck_T arg2_job_string_or_number[] = {arg_job, arg_string_or_nr};
static argcheck_T arg2_list_any_number[] = {arg_list_any, arg_number};
static argcheck_T arg2_list_any_string[] = {arg_list_any, arg_string};
static argcheck_T arg2_list_number[] = {arg_list_number, arg_list_number};
static argcheck_T arg2_list_number_bool[] = {arg_list_number, arg_bool};
static argcheck_T arg2_listblobmod_item[] = {arg_list_or_blob_mod, arg_item_of_prev};
static argcheck_T arg2_lnum[] = {arg_lnum, arg_lnum};
static argcheck_T arg2_lnum_number[] = {arg_lnum, arg_number};
static argcheck_T arg2_number[] = {arg_number, arg_number};
static argcheck_T arg2_number_any[] = {arg_number, arg_any};
static argcheck_T arg2_number_bool[] = {arg_number, arg_bool};
static argcheck_T arg2_number_dict_any[] = {arg_number, arg_dict_any};
static argcheck_T arg2_number_list[] = {arg_number, arg_list_any};
static argcheck_T arg2_number_string[] = {arg_number, arg_string};
static argcheck_T arg2_number_string_or_list[] = {arg_number, arg_string_or_list_any};
static argcheck_T arg2_str_or_nr_or_list_dict[] = {arg_str_or_nr_or_list, arg_dict_any};
static argcheck_T arg2_string[] = {arg_string, arg_string};
static argcheck_T arg2_string_any[] = {arg_string, arg_any};
static argcheck_T arg2_string_bool[] = {arg_string, arg_bool};
static argcheck_T arg2_string_chan_or_job[] = {arg_string, arg_chan_or_job};
static argcheck_T arg2_string_dict[] = {arg_string, arg_dict_any};
static argcheck_T arg2_string_list_number[] = {arg_string, arg_list_number};
static argcheck_T arg2_string_number[] = {arg_string, arg_number};
static argcheck_T arg2_string_or_list_dict[] = {arg_string_or_list_any, arg_dict_any};
static argcheck_T arg2_string_or_list_number[] = {arg_string_or_list_any, arg_number};
static argcheck_T arg2_string_string_or_number[] = {arg_string, arg_string_or_nr};
static argcheck_T arg3_any_list_dict[] = {arg_any, arg_list_any, arg_dict_any};
static argcheck_T arg3_buffer_lnum_lnum[] = {arg_buffer, arg_lnum, arg_lnum};
static argcheck_T arg3_buffer_number_number[] = {arg_buffer, arg_number, arg_number};
static argcheck_T arg3_buffer_string_any[] = {arg_buffer, arg_string, arg_any};
static argcheck_T arg3_buffer_string_dict[] = {arg_buffer, arg_string, arg_dict_any};
static argcheck_T arg3_dict_number_number[] = {arg_dict_any, arg_number, arg_number};
static argcheck_T arg3_list_string_dict[] = {arg_list_any, arg_string, arg_dict_any};
static argcheck_T arg3_lnum_number_bool[] = {arg_lnum, arg_number, arg_bool};
static argcheck_T arg3_number[] = {arg_number, arg_number, arg_number};
static argcheck_T arg3_number_any_dict[] = {arg_number, arg_any, arg_dict_any};
static argcheck_T arg3_number_number_dict[] = {arg_number, arg_number, arg_dict_any};
static argcheck_T arg3_number_string_any[] = {arg_number, arg_string, arg_any};
static argcheck_T arg3_number_string_buffer[] = {arg_number, arg_string, arg_buffer};
static argcheck_T arg3_number_string_string[] = {arg_number, arg_string, arg_string};
static argcheck_T arg3_string[] = {arg_string, arg_string, arg_string};
static argcheck_T arg3_string_any_dict[] = {arg_string, arg_any, arg_dict_any};
static argcheck_T arg3_string_any_string[] = {arg_string, arg_any, arg_string};
static argcheck_T arg3_string_bool_bool[] = {arg_string, arg_bool, arg_bool};
static argcheck_T arg3_string_number_bool[] = {arg_string, arg_number, arg_bool};
static argcheck_T arg3_string_number_number[] = {arg_string, arg_number, arg_number};
static argcheck_T arg3_string_or_dict_bool_dict[] = {arg_string_or_dict_any, arg_bool, arg_dict_any};
static argcheck_T arg3_string_or_list_bool_number[] = {arg_string_or_list_any, arg_bool, arg_number};
static argcheck_T arg3_string_string_bool[] = {arg_string, arg_string, arg_bool};
static argcheck_T arg3_string_string_dict[] = {arg_string, arg_string, arg_dict_any};
static argcheck_T arg3_string_string_number[] = {arg_string, arg_string, arg_number};
static argcheck_T arg4_number_number_string_any[] = {arg_number, arg_number, arg_string, arg_any};
static argcheck_T arg4_string_string_any_string[] = {arg_string, arg_string, arg_any, arg_string};
static argcheck_T arg4_string_string_number_string[] = {arg_string, arg_string, arg_number, arg_string};
static argcheck_T arg4_string_number_bool_bool[] = {arg_string, arg_number, arg_bool, arg_bool};
/* Function specific argument types (not covered by the above) */
static argcheck_T arg15_assert_fails[] = {arg_string_or_nr, arg_string_or_list_any, arg_any, arg_number, arg_string};
static argcheck_T arg34_assert_inrange[] = {arg_float_or_nr, arg_float_or_nr, arg_float_or_nr, arg_string};
static argcheck_T arg4_browse[] = {arg_bool, arg_string, arg_string, arg_string};
static argcheck_T arg23_chanexpr[] = {arg_chan_or_job, arg_any, arg_dict_any};
static argcheck_T arg23_chanraw[] = {arg_chan_or_job, arg_string_or_blob, arg_dict_any};
static argcheck_T arg24_count[] = {arg_string_or_list_or_dict, arg_any, arg_bool, arg_number};
static argcheck_T arg13_cursor[] = {arg_cursor1, arg_number, arg_number};
static argcheck_T arg12_deepcopy[] = {arg_any, arg_bool};
static argcheck_T arg12_execute[] = {arg_string_or_list_string, arg_string};
static argcheck_T arg23_extend[] = {arg_list_or_dict_mod, arg_same_as_prev, arg_extend3};
static argcheck_T arg23_extendnew[] = {arg_list_or_dict, arg_same_struct_as_prev, arg_extend3};
static argcheck_T arg23_get[] = {arg_get1, arg_string_or_nr, arg_any};
static argcheck_T arg14_glob[] = {arg_string, arg_bool, arg_bool, arg_bool};
static argcheck_T arg25_globpath[] = {arg_string, arg_string, arg_bool, arg_bool, arg_bool};
static argcheck_T arg24_index[] = {arg_list_or_blob, arg_item_of_prev, arg_number, arg_bool};
static argcheck_T arg23_index[] = {arg_list_or_blob, arg_filter_func, arg_dict_any};
static argcheck_T arg23_insert[] = {arg_list_or_blob, arg_item_of_prev, arg_number};
static argcheck_T arg1_len[] = {arg_len1};
static argcheck_T arg3_libcall[] = {arg_string, arg_string, arg_string_or_nr};
static argcheck_T arg14_maparg[] = {arg_string, arg_string, arg_bool, arg_bool};
static argcheck_T arg2_filter[] = {arg_list_or_dict_or_blob_or_string_mod, arg_filter_func};
static argcheck_T arg2_foreach[] = {arg_list_or_dict_or_blob_or_string, arg_foreach_func};
static argcheck_T arg2_instanceof[] = {arg_object, varargs_class, NULL };
static argcheck_T arg2_map[] = {arg_list_or_dict_or_blob_or_string_mod, arg_map_func};
static argcheck_T arg2_mapnew[] = {arg_list_or_dict_or_blob_or_string, arg_any};
static argcheck_T arg25_matchadd[] = {arg_string, arg_string, arg_number, arg_number, arg_dict_any};
static argcheck_T arg25_matchaddpos[] = {arg_string, arg_list_any, arg_number, arg_number, arg_dict_any};
static argcheck_T arg23_matchstrlist[] = {arg_list_string, arg_string, arg_dict_any};
static argcheck_T arg45_matchbufline[] = {arg_buffer, arg_string, arg_lnum, arg_lnum, arg_dict_any};
static argcheck_T arg119_printf[] = {arg_string_or_nr, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any, arg_any};
static argcheck_T arg23_reduce[] = {arg_string_list_or_blob, arg_any, arg_any};
static argcheck_T arg24_remote_expr[] = {arg_string, arg_string, arg_string, arg_number};
static argcheck_T arg23_remove[] = {arg_list_or_dict_or_blob_mod, arg_remove2, arg_number};
static argcheck_T arg2_repeat[] = {arg_repeat1, arg_number};
static argcheck_T arg15_search[] = {arg_string, arg_string, arg_number, arg_number, arg_string_or_func};
static argcheck_T arg37_searchpair[] = {arg_string, arg_string, arg_string, arg_string, arg_string_or_func, arg_number, arg_number};
static argcheck_T arg3_setbufline[] = {arg_buffer, arg_lnum, arg_str_or_nr_or_list};
static argcheck_T arg2_setline[] = {arg_lnum, arg_any};
static argcheck_T arg24_setloclist[] = {arg_number, arg_list_any, arg_string, arg_dict_any};
static argcheck_T arg13_setqflist[] = {arg_list_any, arg_string, arg_dict_any};
static argcheck_T arg23_settagstack[] = {arg_number, arg_dict_any, arg_string};
static argcheck_T arg02_sign_getplaced[] = {arg_buffer, arg_dict_any};
static argcheck_T arg45_sign_place[] = {arg_number, arg_string, arg_string, arg_buffer, arg_dict_any};
static argcheck_T arg23_slice[] = {arg_slice1, arg_number, arg_number};
static argcheck_T arg13_sortuniq[] = {arg_list_any_mod, arg_sort_how, arg_dict_any};
static argcheck_T arg24_strpart[] = {arg_string, arg_number, arg_number, arg_bool};
static argcheck_T arg12_system[] = {arg_string, arg_str_or_nr_or_list};
static argcheck_T arg23_win_execute[] = {arg_number, arg_string_or_list_string, arg_string};
static argcheck_T arg23_writefile[] = {arg_list_or_blob, arg_string, arg_string};
static argcheck_T arg24_match_func[] = {arg_string_or_list_any, arg_string, arg_number, arg_number};

// Can be used by functions called through "f_retfunc" to create new types.
static garray_T *current_type_gap = NULL;

/*
 * Functions that return the return type of a builtin function.
 * Note that "argtypes" is NULL if "argcount" is zero.
 */
    static type_T *
ret_void(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_void;
}
    static type_T *
ret_any(int	argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_any;
}
    static type_T *
ret_bool(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_bool;
}
    static type_T *
ret_number_bool(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_number_bool;
}
    static type_T *
ret_number(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_number;
}
    static type_T *
ret_float(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_float;
}
    static type_T *
ret_string(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_string;
}
    static type_T *
ret_list_any(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_list_any;
}
    static type_T *
ret_list_number(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    *decl_type = &t_list_any;
    return &t_list_number;
}
    static type_T *
ret_list_string(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    *decl_type = &t_list_any;
    return &t_list_string;
}
    static type_T *
ret_list_dict_any(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    *decl_type = &t_list_any;
    return &t_list_dict_any;
}
    static type_T *
ret_list_items(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    *decl_type = &t_list_any;
    return &t_list_list_any;
}

    static type_T *
ret_list_string_items(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    *decl_type = &t_list_any;
    return &t_list_list_string;
}
    static type_T *
ret_dict_any(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_dict_any;
}
    static type_T *
ret_job_info(int argcount,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    if (argcount == 0)
    {
	*decl_type = &t_list_any;
	return &t_list_job;
    }
    return &t_dict_any;
}
    static type_T *
ret_dict_number(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_dict_number;
}
    static type_T *
ret_dict_string(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_dict_string;
}
    static type_T *
ret_blob(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_blob;
}
    static type_T *
ret_func_any(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_func_any;
}
    static type_T *
ret_func_unknown(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_func_unknown;
}
    static type_T *
ret_channel(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_channel;
}
    static type_T *
ret_job(int argcount UNUSED,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    return &t_job;
}
    static type_T *
ret_first_arg(int argcount,
	type2_T *argtypes,
	type_T	**decl_type)
{
    if (argcount > 0)
    {
	*decl_type = argtypes[0].type_decl;
	return argtypes[0].type_curr;
    }
    return &t_void;
}
    static type_T *
ret_slice(int argcount,
	type2_T *argtypes,
	type_T	**decl_type)
{
    if (argcount > 0)
    {
	if (argtypes[0].type_decl != NULL)
	{
	    switch (argtypes[0].type_decl->tt_type)
	    {
		case VAR_STRING: *decl_type = &t_string; break;
		case VAR_BLOB: *decl_type = &t_blob; break;
		case VAR_LIST: *decl_type = &t_list_any; break;
		default: break;
	    }
	}
	return argtypes[0].type_curr;
    }
    return &t_void;
}
    static type_T *
ret_copy(int argcount,
	type2_T *argtypes,
	type_T	**decl_type)
{
    if (argcount > 0)
    {
	if (argtypes[0].type_decl != NULL)
	{
	    if (argtypes[0].type_decl->tt_type == VAR_LIST)
		*decl_type = &t_list_any;
	    else if (argtypes[0].type_decl->tt_type == VAR_DICT)
		*decl_type = &t_dict_any;
	    else
		*decl_type = argtypes[0].type_decl;
	}
	if (argtypes[0].type_curr != NULL)
	{
	    if (argtypes[0].type_curr->tt_type == VAR_LIST)
		return &t_list_any;
	    else if (argtypes[0].type_curr->tt_type == VAR_DICT)
		return &t_dict_any;
	}
	return argtypes[0].type_curr;
    }
    return &t_void;
}
    static type_T *
ret_extend(int argcount,
	type2_T *argtypes,
	type_T	**decl_type)
{
    if (argcount > 0)
    {
	*decl_type = argtypes[0].type_decl;
	// if the second argument has a different current type then the current
	// type is "any"
	if (argcount > 1 && !equal_type(argtypes[0].type_curr,
						     argtypes[1].type_curr, 0))
	{
	    if (argtypes[0].type_curr->tt_type == VAR_LIST)
		return &t_list_any;
	    if (argtypes[0].type_curr->tt_type == VAR_DICT)
		return &t_dict_any;
	}
	return argtypes[0].type_curr;
    }
    return &t_void;
}
    static type_T *
ret_repeat(int argcount,
	type2_T *argtypes,
	type_T	**decl_type UNUSED)
{
    if (argcount == 0)
	return &t_any;
    if (argtypes[0].type_curr == &t_number)
	return &t_string;
    return argtypes[0].type_curr;
}
// for map(): returns first argument but item type may differ
    static type_T *
ret_first_cont(int argcount,
	type2_T *argtypes,
	type_T	**decl_type UNUSED)
{
    if (argcount > 0)
    {
	if (argtypes[0].type_curr->tt_type == VAR_LIST)
	    return &t_list_any;
	if (argtypes[0].type_curr->tt_type == VAR_DICT)
	    return &t_dict_any;
	if (argtypes[0].type_curr->tt_type == VAR_BLOB)
	    return argtypes[0].type_curr;
    }
    return &t_any;
}
// for getline()
    static type_T *
ret_getline(int argcount,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    if (argcount == 1)
	return &t_string;
    *decl_type = &t_list_any;
    return &t_list_string;
}
// for finddir()
    static type_T *
ret_finddir(int argcount,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    if (argcount < 3)
	return &t_string;
    // Depending on the count would be a string or a list of strings.
    return &t_any;
}
// for values(): list of member of first argument
    static type_T *
ret_list_member(int argcount,
	type2_T *argtypes,
	type_T	**decl_type)
{
    if (argcount > 0)
    {
	type_T *t = argtypes[0].type_decl;
	if (current_type_gap != NULL
		&& (t->tt_type == VAR_DICT || t->tt_type == VAR_LIST))
	    t = get_list_type(t->tt_member, current_type_gap);
	else
	    t = &t_list_any;
	*decl_type = t;

	t = argtypes[0].type_curr;
	if (current_type_gap != NULL
		&& (t->tt_type == VAR_DICT || t->tt_type == VAR_LIST))
	    return get_list_type(t->tt_member, current_type_gap);
    }
    return &t_list_any;
}

/*
 * Used for getqflist(): returns list if there is no argument, dict if there is
 * one.
 */
    static type_T *
ret_list_or_dict_0(int argcount,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    if (argcount > 0)
	return &t_dict_any;
    *decl_type = &t_list_any;
    return &t_list_dict_any;
}

/*
 * Used for getloclist(): returns list if there is one argument, dict if there
 * are two.
 */
    static type_T *
ret_list_or_dict_1(int argcount,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    if (argcount > 1)
	return &t_dict_any;
    *decl_type = &t_list_any;
    return &t_list_dict_any;
}

    static type_T *
ret_argv(int argcount,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    // argv() returns list of strings
    if (argcount == 0)
    {
	*decl_type = &t_list_any;
	return &t_list_string;
    }

    // argv(0) returns a string, but argv(-1] returns a list
    return &t_any;
}

    static type_T *
ret_remove(int argcount,
	type2_T *argtypes,
	type_T	**decl_type)
{
    if (argcount > 0)
    {
	if (argtypes[0].type_curr->tt_type == VAR_LIST
		|| argtypes[0].type_curr->tt_type == VAR_DICT)
	{
	    if (argcount == 3)
	    {
		*decl_type = argtypes[0].type_decl;
		return argtypes[0].type_curr;
	    }
	    if (argtypes[0].type_curr->tt_type
					     == argtypes[0].type_decl->tt_type)
		*decl_type = argtypes[0].type_decl->tt_member;
	    return argtypes[0].type_curr->tt_member;
	}
	if (argtypes[0].type_curr->tt_type == VAR_BLOB)
	    return &t_number;
    }
    return &t_any;
}

    static type_T *
ret_getreg(int argcount,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    // Assume that if the third argument is passed it's non-zero
    if (argcount == 3)
    {
	*decl_type = &t_list_any;
	return &t_list_string;
    }
    return &t_string;
}

    static type_T *
ret_virtcol(int argcount,
	type2_T *argtypes UNUSED,
	type_T	**decl_type)
{
    // Assume that if the second argument is passed it's non-zero
    if (argcount > 1)
    {
	*decl_type = &t_list_any;
	return &t_list_number;
    }
    return &t_number;
}

    static type_T *
ret_maparg(int argcount,
	type2_T *argtypes UNUSED,
	type_T	**decl_type UNUSED)
{
    // Assume that if the fourth argument is passed it's non-zero
    if (argcount == 4)
	return &t_dict_any;
    return &t_string;
}

/*
 * Array with names and number of arguments of all internal functions
 * MUST BE KEPT SORTED IN strcmp() ORDER FOR BINARY SEARCH!
 *
 * The builtin function may be varargs. In that case
 *	- f_max_argc == VARGS
 *	- For varargs, f_argcheck must be NULL terminated. The last non-null
 *	  entry in f_argcheck should validate all the remaining args.
 */
typedef struct
{
    char	*f_name;	// function name
    char	f_min_argc;	// minimal number of arguments
    char	f_max_argc;	// maximal number of arguments
    char	f_argtype;	// for method: FEARG_ values; bits FE_
    argcheck_T	*f_argcheck;	// list of functions to check argument types;
				// use "arg_any" (not NULL) to accept an
				// argument of any type
    type_T	*(*f_retfunc)(int argcount, type2_T *argtypes,
							   type_T **decl_type);
				// return type function
    void	(*f_func)(typval_T *args, typval_T *rvar);
				// implementation of function
} funcentry_T;

// Set f_max_argc to VARGS for varargs.
#define VARGS    CHAR_MAX

// values for f_argtype; zero means it cannot be used as a method
#define FEARG_1	    0x01    // base is the first argument
#define FEARG_2     0x02    // base is the second argument
#define FEARG_3     0x03    // base is the third argument
#define FEARG_4     0x04    // base is the fourth argument
#define FEARG_MASK  0x0F    // bits in f_argtype used as argument index
#define FE_X	    0x10    // builtin accepts a non-value (class, typealias)

#if defined(HAVE_MATH_H)
# define MATH_FUNC(name) name
#else
# define MATH_FUNC(name) NULL
#endif
#ifdef FEAT_TIMERS
# define TIMER_FUNC(name) name
#else
# define TIMER_FUNC(name) NULL
#endif
#ifdef FEAT_JOB_CHANNEL
# define JOB_FUNC(name) name
#else
# define JOB_FUNC(name) NULL
#endif
#ifdef FEAT_PROP_POPUP
# define PROP_FUNC(name) name
#else
# define PROP_FUNC(name) NULL
#endif
#ifdef FEAT_SIGNS
# define SIGN_FUNC(name) name
#else
# define SIGN_FUNC(name) NULL
#endif
#ifdef FEAT_SOUND
# define SOUND_FUNC(name) name
#else
# define SOUND_FUNC(name) NULL
#endif
#ifdef FEAT_TERMINAL
# define TERM_FUNC(name) name
#else
# define TERM_FUNC(name) NULL
#endif

static funcentry_T global_functions[] =
{
    {"abs",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_any,	    f_abs},
    {"acos",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_acos},
    {"add",		2, 2, FEARG_1,	    arg2_listblobmod_item,
			ret_first_arg,	    f_add},
    {"and",		2, 2, FEARG_1,	    arg2_number,
			ret_number,	    f_and},
    {"append",		2, 2, FEARG_2,	    arg2_setline,
			ret_number_bool,    f_append},
    {"appendbufline",	3, 3, FEARG_3,	    arg3_setbufline,
			ret_number_bool,    f_appendbufline},
    {"argc",		0, 1, 0,	    arg1_number,
			ret_number,	    f_argc},
    {"argidx",		0, 0, 0,	    NULL,
			ret_number,	    f_argidx},
    {"arglistid",	0, 2, 0,	    arg2_number,
			ret_number,	    f_arglistid},
    {"argv",		0, 2, 0,	    arg2_number,
			ret_argv,	    f_argv},
    {"asin",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_asin},
    {"assert_beeps",	1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_assert_beeps},
    {"assert_equal",	2, 3, FEARG_2,	    NULL,
			ret_number_bool,    f_assert_equal},
    {"assert_equalfile", 2, 3, FEARG_1,	    arg3_string,
			ret_number_bool,    f_assert_equalfile},
    {"assert_exception", 1, 2, 0,	    arg2_string,
			ret_number_bool,    f_assert_exception},
    {"assert_fails",	1, 5, FEARG_1,	    arg15_assert_fails,
			ret_number_bool,    f_assert_fails},
    {"assert_false",	1, 2, FEARG_1,	    NULL,
			ret_number_bool,    f_assert_false},
    {"assert_inrange",	3, 4, FEARG_3,	    arg34_assert_inrange,
			ret_number_bool,    f_assert_inrange},
    {"assert_match",	2, 3, FEARG_2,	    arg3_string,
			ret_number_bool,    f_assert_match},
    {"assert_nobeep",	1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_assert_nobeep},
    {"assert_notequal",	2, 3, FEARG_2,	    NULL,
			ret_number_bool,    f_assert_notequal},
    {"assert_notmatch",	2, 3, FEARG_2,	    arg3_string,
			ret_number_bool,    f_assert_notmatch},
    {"assert_report",	1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_assert_report},
    {"assert_true",	1, 2, FEARG_1,	    NULL,
			ret_number_bool,    f_assert_true},
    {"atan",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_atan},
    {"atan2",		2, 2, FEARG_1,	    arg2_float_or_nr,
			ret_float,	    f_atan2},
    {"autocmd_add",	1, 1, FEARG_1,	    arg1_list_any,
			ret_number_bool,    f_autocmd_add},
    {"autocmd_delete",	1, 1, FEARG_1,	    arg1_list_any,
			ret_number_bool,    f_autocmd_delete},
    {"autocmd_get",	0, 1, FEARG_1,	    arg1_dict_any,
			ret_list_dict_any,  f_autocmd_get},
    {"balloon_gettext",	0, 0, 0,	    NULL,
			ret_string,
#ifdef FEAT_BEVAL
	    f_balloon_gettext
#else
	    NULL
#endif
			},
    {"balloon_show",	1, 1, FEARG_1,	    arg1_string_or_list_any,
			ret_void,
#ifdef FEAT_BEVAL
	    f_balloon_show
#else
	    NULL
#endif
			},
    {"balloon_split",	1, 1, FEARG_1,	    arg1_string,
			ret_list_string,
#if defined(FEAT_BEVAL_TERM)
	    f_balloon_split
#else
	    NULL
#endif
			},
    {"blob2list",	1, 1, FEARG_1,	    arg1_blob,
			ret_list_number,    f_blob2list},
    {"browse",		4, 4, 0,	    arg4_browse,
			ret_string,	    f_browse},
    {"browsedir",	2, 2, 0,	    arg2_string,
			ret_string,	    f_browsedir},
    {"bufadd",		1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_bufadd},
    {"bufexists",	1, 1, FEARG_1,	    arg1_buffer,
			ret_number_bool,    f_bufexists},
    {"buffer_exists",	1, 1, FEARG_1,	    arg1_buffer,	// obsolete
			ret_number_bool,    f_bufexists},
    {"buffer_name",	0, 1, FEARG_1,	    arg1_buffer,	// obsolete
			ret_string,	    f_bufname},
    {"buffer_number",	0, 1, FEARG_1,	    arg1_buffer,	// obsolete
			ret_number,	    f_bufnr},
    {"buflisted",	1, 1, FEARG_1,	    arg1_buffer,
			ret_number_bool,    f_buflisted},
    {"bufload",		1, 1, FEARG_1,	    arg1_buffer,
			ret_void,	    f_bufload},
    {"bufloaded",	1, 1, FEARG_1,	    arg1_buffer,
			ret_number_bool,    f_bufloaded},
    {"bufname",		0, 1, FEARG_1,	    arg1_buffer,
			ret_string,	    f_bufname},
    {"bufnr",		0, 2, FEARG_1,	    arg2_buffer_bool,
			ret_number,	    f_bufnr},
    {"bufwinid",	1, 1, FEARG_1,	    arg1_buffer,
			ret_number,	    f_bufwinid},
    {"bufwinnr",	1, 1, FEARG_1,	    arg1_buffer,
			ret_number,	    f_bufwinnr},
    {"byte2line",	1, 1, FEARG_1,	    arg1_number,
			ret_number,	    f_byte2line},
    {"byteidx",		2, 3, FEARG_1,	    arg3_string_number_bool,
			ret_number,	    f_byteidx},
    {"byteidxcomp",	2, 3, FEARG_1,	    arg3_string_number_bool,
			ret_number,	    f_byteidxcomp},
    {"call",		2, 3, FEARG_1,	    arg3_any_list_dict,
			ret_any,	    f_call},
    {"ceil",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_ceil},
    {"ch_canread",	1, 1, FEARG_1,	    arg1_chan_or_job,
			ret_number_bool,    JOB_FUNC(f_ch_canread)},
    {"ch_close",	1, 1, FEARG_1,	    arg1_chan_or_job,
			ret_void,	    JOB_FUNC(f_ch_close)},
    {"ch_close_in",	1, 1, FEARG_1,	    arg1_chan_or_job,
			ret_void,	    JOB_FUNC(f_ch_close_in)},
    {"ch_evalexpr",	2, 3, FEARG_1,	    arg23_chanexpr,
			ret_any,	    JOB_FUNC(f_ch_evalexpr)},
    {"ch_evalraw",	2, 3, FEARG_1,	    arg23_chanraw,
			ret_any,	    JOB_FUNC(f_ch_evalraw)},
    {"ch_getbufnr",	2, 2, FEARG_1,	    arg2_chan_or_job_string,
			ret_number,	    JOB_FUNC(f_ch_getbufnr)},
    {"ch_getjob",	1, 1, FEARG_1,	    arg1_chan_or_job,
			ret_job,	    JOB_FUNC(f_ch_getjob)},
    {"ch_info",		1, 1, FEARG_1,	    arg1_chan_or_job,
			ret_dict_any,	    JOB_FUNC(f_ch_info)},
    {"ch_log",		1, 2, FEARG_1,	    arg2_string_chan_or_job,
			ret_void,	    f_ch_log},
    {"ch_logfile",	1, 2, FEARG_1,	    arg2_string,
			ret_void,	    f_ch_logfile},
    {"ch_open",		1, 2, FEARG_1,	    arg2_string_dict,
			ret_channel,	    JOB_FUNC(f_ch_open)},
    {"ch_read",		1, 2, FEARG_1,	    arg2_chan_or_job_dict,
			ret_string,	    JOB_FUNC(f_ch_read)},
    {"ch_readblob",	1, 2, FEARG_1,	    arg2_chan_or_job_dict,
			ret_blob,	    JOB_FUNC(f_ch_readblob)},
    {"ch_readraw",	1, 2, FEARG_1,	    arg2_chan_or_job_dict,
			ret_string,	    JOB_FUNC(f_ch_readraw)},
    {"ch_sendexpr",	2, 3, FEARG_1,	    arg23_chanexpr,
			ret_any,	    JOB_FUNC(f_ch_sendexpr)},
    {"ch_sendraw",	2, 3, FEARG_1,	    arg23_chanraw,
			ret_void,	    JOB_FUNC(f_ch_sendraw)},
    {"ch_setoptions",	2, 2, FEARG_1,	    arg2_chan_or_job_dict,
			ret_void,	    JOB_FUNC(f_ch_setoptions)},
    {"ch_status",	1, 2, FEARG_1,	    arg2_chan_or_job_dict,
			ret_string,	    JOB_FUNC(f_ch_status)},
    {"changenr",	0, 0, 0,	    NULL,
			ret_number,	    f_changenr},
    {"char2nr",		1, 2, FEARG_1,	    arg2_string_bool,
			ret_number,	    f_char2nr},
    {"charclass",	1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_charclass},
    {"charcol",		1, 2, FEARG_1,	    arg2_string_or_list_number,
			ret_number,	    f_charcol},
    {"charidx",		2, 4, FEARG_1,	    arg4_string_number_bool_bool,
			ret_number,	    f_charidx},
    {"chdir",		1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_chdir},
    {"cindent",		1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_cindent},
    {"clearmatches",	0, 1, FEARG_1,	    arg1_number,
			ret_void,	    f_clearmatches},
    {"col",		1, 2, FEARG_1,	    arg2_string_or_list_number,
			ret_number,	    f_col},
    {"complete",	2, 2, FEARG_2,	    arg2_number_list,
			ret_void,	    f_complete},
    {"complete_add",	1, 1, FEARG_1,	    arg1_dict_or_string,
			ret_number,	    f_complete_add},
    {"complete_check",	0, 0, 0,	    NULL,
			ret_number_bool,    f_complete_check},
    {"complete_info",	0, 1, FEARG_1,	    arg1_list_string,
			ret_dict_any,	    f_complete_info},
    {"confirm",		1, 4, FEARG_1,	    arg4_string_string_number_string,
			ret_number,	    f_confirm},
    {"copy",		1, 1, FEARG_1,	    NULL,
			ret_copy,	    f_copy},
    {"cos",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_cos},
    {"cosh",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_cosh},
    {"count",		2, 4, FEARG_1,	    arg24_count,
			ret_number,	    f_count},
    {"cscope_connection",0,3, 0,	    arg3_number_string_string,
			ret_number,	    f_cscope_connection},
    {"cursor",		1, 3, FEARG_1,	    arg13_cursor,
			ret_number,	    f_cursor},
    {"debugbreak",	1, 1, FEARG_1,	    arg1_number,
			ret_number,
#ifdef MSWIN
	    f_debugbreak
#else
	    NULL
#endif
			},
    {"deepcopy",	1, 2, FEARG_1,	    arg12_deepcopy,
			ret_copy,	    f_deepcopy},
    {"delete",		1, 2, FEARG_1,	    arg2_string,
			ret_number_bool,    f_delete},
    {"deletebufline",	2, 3, FEARG_1,	    arg3_buffer_lnum_lnum,
			ret_number_bool,    f_deletebufline},
    {"did_filetype",	0, 0, 0,	    NULL,
			ret_number_bool,    f_did_filetype},
    {"diff_filler",	1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_diff_filler},
    {"diff_hlID",	2, 2, FEARG_1,	    arg2_lnum_number,
			ret_number,	    f_diff_hlID},
    {"digraph_get",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_digraph_get},
    {"digraph_getlist",0, 1, FEARG_1,	    arg1_bool,
			ret_list_string_items, f_digraph_getlist},
    {"digraph_set",	2, 2, FEARG_1,	    arg2_string,
			ret_bool,	f_digraph_set},
    {"digraph_setlist",1, 1, FEARG_1,	    arg1_list_string,
			ret_bool,	    f_digraph_setlist},
    {"echoraw",		1, 1, FEARG_1,	    arg1_string,
			ret_void,	    f_echoraw},
    {"empty",		1, 1, FEARG_1,	    NULL,
			ret_number_bool,    f_empty},
    {"environ",		0, 0, 0,	    NULL,
			ret_dict_string,    f_environ},
    {"err_teapot",	0, 1, 0,	    NULL,
			ret_number_bool,    f_err_teapot},
    {"escape",		2, 2, FEARG_1,	    arg2_string,
			ret_string,	    f_escape},
    {"eval",		1, 1, FEARG_1,	    arg1_string,
			ret_any,	    f_eval},
    {"eventhandler",	0, 0, 0,	    NULL,
			ret_number_bool,    f_eventhandler},
    {"executable",	1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_executable},
    {"execute",		1, 2, FEARG_1,	    arg12_execute,
			ret_string,	    f_execute},
    {"exepath",		1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_exepath},
    {"exists",		1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_exists},
    {"exists_compiled",	1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_exists_compiled},
    {"exp",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_exp},
    {"expand",		1, 3, FEARG_1,	    arg3_string_bool_bool,
			ret_any,	    f_expand},
    {"expandcmd",	1, 2, FEARG_1,	    arg2_string_dict,
			ret_string,	    f_expandcmd},
    {"extend",		2, 3, FEARG_1,	    arg23_extend,
			ret_extend,	    f_extend},
    {"extendnew",	2, 3, FEARG_1,	    arg23_extendnew,
			ret_first_cont,	    f_extendnew},
    {"feedkeys",	1, 2, FEARG_1,	    arg2_string,
			ret_void,	    f_feedkeys},
    {"file_readable",	1, 1, FEARG_1,	    arg1_string,	// obsolete
			ret_number_bool,    f_filereadable},
    {"filereadable",	1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_filereadable},
    {"filewritable",	1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_filewritable},
    {"filter",		2, 2, FEARG_1,	    arg2_filter,
			ret_first_arg,	    f_filter},
    {"finddir",		1, 3, FEARG_1,	    arg3_string_string_number,
			ret_finddir,	    f_finddir},
    {"findfile",	1, 3, FEARG_1,	    arg3_string_string_number,
			ret_any,	    f_findfile},
    {"flatten",		1, 2, FEARG_1,	    arg2_list_any_number,
			ret_list_any,	    f_flatten},
    {"flattennew",	1, 2, FEARG_1,	    arg2_list_any_number,
			ret_list_any,	    f_flattennew},
    {"float2nr",	1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_number,	    f_float2nr},
    {"floor",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_floor},
    {"fmod",		2, 2, FEARG_1,	    arg2_float_or_nr,
			ret_float,	    f_fmod},
    {"fnameescape",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_fnameescape},
    {"fnamemodify",	2, 2, FEARG_1,	    arg2_string,
			ret_string,	    f_fnamemodify},
    {"foldclosed",	1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_foldclosed},
    {"foldclosedend",	1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_foldclosedend},
    {"foldlevel",	1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_foldlevel},
    {"foldtext",	0, 0, 0,	    NULL,
			ret_string,	    f_foldtext},
    {"foldtextresult",	1, 1, FEARG_1,	    arg1_lnum,
			ret_string,	    f_foldtextresult},
    {"foreach",		2, 2, FEARG_1,	    arg2_foreach,
			ret_first_arg,	    f_foreach},
    {"foreground",	0, 0, 0,	    NULL,
			ret_void,	    f_foreground},
    {"fullcommand",	1, 2, FEARG_1,	    arg2_string_bool,
			ret_string,	    f_fullcommand},
    {"funcref",		1, 3, FEARG_1,	    arg3_any_list_dict,
			ret_func_unknown,   f_funcref},
    {"function",	1, 3, FEARG_1,	    arg3_any_list_dict,
			ret_func_unknown,   f_function},
    {"garbagecollect",	0, 1, 0,	    arg1_bool,
			ret_void,	    f_garbagecollect},
    {"get",		2, 3, FEARG_1,	    arg23_get,
			ret_any,	    f_get},
    {"getbufinfo",	0, 1, FEARG_1,	    arg1_buffer_or_dict_any,
			ret_list_dict_any,  f_getbufinfo},
    {"getbufline",	2, 3, FEARG_1,	    arg3_buffer_lnum_lnum,
			ret_list_string,    f_getbufline},
    {"getbufoneline",	2, 2, FEARG_1,	    arg2_buffer_lnum,
			ret_string,	    f_getbufoneline},
    {"getbufvar",	2, 3, FEARG_1,	    arg3_buffer_string_any,
			ret_any,	    f_getbufvar},
    {"getcellwidths",	0, 0, 0,	    NULL,
			ret_list_any,	    f_getcellwidths},
    {"getchangelist",	0, 1, FEARG_1,	    arg1_buffer,
			ret_list_any,	    f_getchangelist},
    {"getchar",		0, 1, 0,	    arg1_bool,
			ret_any,	    f_getchar},
    {"getcharmod",	0, 0, 0,	    NULL,
			ret_number,	    f_getcharmod},
    {"getcharpos",	1, 1, FEARG_1,	    arg1_string,
			ret_list_number,    f_getcharpos},
    {"getcharsearch",	0, 0, 0,	    NULL,
			ret_dict_any,	    f_getcharsearch},
    {"getcharstr",	0, 1, 0,	    arg1_bool,
			ret_string,	    f_getcharstr},
    {"getcmdcompltype",	0, 0, 0,	    NULL,
			ret_string,	    f_getcmdcompltype},
    {"getcmdline",	0, 0, 0,	    NULL,
			ret_string,	    f_getcmdline},
    {"getcmdpos",	0, 0, 0,	    NULL,
			ret_number,	    f_getcmdpos},
    {"getcmdscreenpos",	0, 0, 0,	    NULL,
			ret_number,	    f_getcmdscreenpos},
    {"getcmdtype",	0, 0, 0,	    NULL,
			ret_string,	    f_getcmdtype},
    {"getcmdwintype",	0, 0, 0,	    NULL,
			ret_string,	    f_getcmdwintype},
    {"getcompletion",	2, 3, FEARG_1,	    arg3_string_string_bool,
			ret_list_string,    f_getcompletion},
    {"getcurpos",	0, 1, FEARG_1,	    arg1_number,
			ret_list_number,    f_getcurpos},
    {"getcursorcharpos", 0, 1, FEARG_1,	    arg1_number,
			ret_list_number,    f_getcursorcharpos},
    {"getcwd",		0, 2, FEARG_1,	    arg2_number,
			ret_string,	    f_getcwd},
    {"getenv",		1, 1, FEARG_1,	    arg1_string,
			ret_any,	    f_getenv},
    {"getfontname",	0, 1, 0,	    arg1_string,
			ret_string,	    f_getfontname},
    {"getfperm",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_getfperm},
    {"getfsize",	1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_getfsize},
    {"getftime",	1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_getftime},
    {"getftype",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_getftype},
    {"getimstatus",	0, 0, 0,	    NULL,
			ret_number_bool,    f_getimstatus},
    {"getjumplist",	0, 2, FEARG_1,	    arg2_number,
			ret_list_any,	    f_getjumplist},
    {"getline",		1, 2, FEARG_1,	    arg2_lnum,
			ret_getline,	    f_getline},
    {"getloclist",	1, 2, 0,	    arg2_number_dict_any,
			ret_list_or_dict_1, f_getloclist},
    {"getmarklist",	0, 1, FEARG_1,	    arg1_buffer,
			ret_list_dict_any,  f_getmarklist},
    {"getmatches",	0, 1, 0,	    arg1_number,
			ret_list_dict_any,  f_getmatches},
    {"getmousepos",	0, 0, 0,	    NULL,
			ret_dict_number,    f_getmousepos},
    {"getmouseshape",	0, 0, 0,	    NULL,
			ret_string,	    f_getmouseshape},
    {"getpid",		0, 0, 0,	    NULL,
			ret_number,	    f_getpid},
    {"getpos",		1, 1, FEARG_1,	    arg1_string,
			ret_list_number,    f_getpos},
    {"getqflist",	0, 1, 0,	    arg1_dict_any,
			ret_list_or_dict_0, f_getqflist},
    {"getreg",		0, 3, FEARG_1,	    arg3_string_bool_bool,
			ret_getreg,	    f_getreg},
    {"getreginfo",	0, 1, FEARG_1,	    arg1_string,
			ret_dict_any,	    f_getreginfo},
    {"getregtype",	0, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_getregtype},
    {"getscriptinfo",	0, 1, 0,	    arg1_dict_any,
			ret_list_dict_any,  f_getscriptinfo},
    {"gettabinfo",	0, 1, FEARG_1,	    arg1_number,
			ret_list_dict_any,  f_gettabinfo},
    {"gettabvar",	2, 3, FEARG_1,	    arg3_number_string_any,
			ret_any,	    f_gettabvar},
    {"gettabwinvar",	3, 4, FEARG_1,	    arg4_number_number_string_any,
			ret_any,	    f_gettabwinvar},
    {"gettagstack",	0, 1, FEARG_1,	    arg1_number,
			ret_dict_any,	    f_gettagstack},
    {"gettext",		1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_gettext},
    {"getwininfo",	0, 1, FEARG_1,	    arg1_number,
			ret_list_dict_any,  f_getwininfo},
    {"getwinpos",	0, 1, FEARG_1,	    arg1_number,
			ret_list_number,    f_getwinpos},
    {"getwinposx",	0, 0, 0,	    NULL,
			ret_number,	    f_getwinposx},
    {"getwinposy",	0, 0, 0,	    NULL,
			ret_number,	    f_getwinposy},
    {"getwinvar",	2, 3, FEARG_1,	    arg3_number_string_any,
			ret_any,	    f_getwinvar},
    {"glob",		1, 4, FEARG_1,	    arg14_glob,
			ret_any,	    f_glob},
    {"glob2regpat",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_glob2regpat},
    {"globpath",	2, 5, FEARG_2,	    arg25_globpath,
			ret_any,	    f_globpath},
    {"has",		1, 2, 0,	    arg2_string_bool,
			ret_number_bool,    f_has},
    {"has_key",		2, 2, FEARG_1,	    arg2_dict_any_string_or_nr,
			ret_number_bool,    f_has_key},
    {"haslocaldir",	0, 2, FEARG_1,	    arg2_number,
			ret_number,	    f_haslocaldir},
    {"hasmapto",	1, 3, FEARG_1,	    arg3_string_string_bool,
			ret_number_bool,    f_hasmapto},
    {"highlightID",	1, 1, FEARG_1,	    arg1_string,	// obsolete
			ret_number,	    f_hlID},
    {"highlight_exists",1, 1, FEARG_1,	    arg1_string,	// obsolete
			ret_number_bool,    f_hlexists},
    {"histadd",		2, 2, FEARG_2,	    arg2_string,
			ret_number_bool,    f_histadd},
    {"histdel",		1, 2, FEARG_1,	    arg2_string_string_or_number,
			ret_number_bool,    f_histdel},
    {"histget",		1, 2, FEARG_1,	    arg2_string_number,
			ret_string,	    f_histget},
    {"histnr",		1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_histnr},
    {"hlID",		1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_hlID},
    {"hlexists",	1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_hlexists},
    {"hlget",		0, 2, FEARG_1,	    arg2_string_bool,
			ret_list_dict_any,  f_hlget},
    {"hlset",		1, 1, FEARG_1,	    arg1_list_any,
			ret_number_bool,    f_hlset},
    {"hostname",	0, 0, 0,	    NULL,
			ret_string,	    f_hostname},
    {"iconv",		3, 3, FEARG_1,	    arg3_string,
			ret_string,	    f_iconv},
    {"indent",		1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_indent},
    {"index",		2, 4, FEARG_1,	    arg24_index,
			ret_number,	    f_index},
    {"indexof",		2, 3, FEARG_1,	    arg23_index,
			ret_number,	    f_indexof},
    {"input",		1, 3, FEARG_1,	    arg3_string,
			ret_string,	    f_input},
    {"inputdialog",	1, 3, FEARG_1,	    arg3_string,
			ret_string,	    f_inputdialog},
    {"inputlist",	1, 1, FEARG_1,	    arg1_list_string,
			ret_number,	    f_inputlist},
    {"inputrestore",	0, 0, 0,	    NULL,
			ret_number_bool,    f_inputrestore},
    {"inputsave",	0, 0, 0,	    NULL,
			ret_number_bool,    f_inputsave},
    {"inputsecret",	1, 2, FEARG_1,	    arg2_string,
			ret_string,	    f_inputsecret},
    {"insert",		2, 3, FEARG_1,	    arg23_insert,
			ret_first_arg,	    f_insert},
    {"instanceof",	2, VARGS, FEARG_1|FE_X,	arg2_instanceof,
			ret_bool,	    f_instanceof},
    {"interrupt",	0, 0, 0,	    NULL,
			ret_void,	    f_interrupt},
    {"invert",		1, 1, FEARG_1,	    arg1_number,
			ret_number,	    f_invert},
    {"isabsolutepath",	1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_isabsolutepath},
    {"isdirectory",	1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_isdirectory},
    {"isinf",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_number,	    MATH_FUNC(f_isinf)},
    {"islocked",	1, 1, FEARG_1,	    arg1_string,
			ret_number_bool,    f_islocked},
    {"isnan",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_number_bool,    MATH_FUNC(f_isnan)},
    {"items",		1, 1, FEARG_1,	    arg1_string_or_list_or_dict,
			ret_list_items,	    f_items},
    {"job_getchannel",	1, 1, FEARG_1,	    arg1_job,
			ret_channel,	    JOB_FUNC(f_job_getchannel)},
    {"job_info",	0, 1, FEARG_1,	    arg1_job,
			ret_job_info,	    JOB_FUNC(f_job_info)},
    {"job_setoptions",	2, 2, FEARG_1,	    arg2_job_dict,
			ret_void,	    JOB_FUNC(f_job_setoptions)},
    {"job_start",	1, 2, FEARG_1,	    arg2_string_or_list_dict,
			ret_job,	    JOB_FUNC(f_job_start)},
    {"job_status",	1, 1, FEARG_1,	    arg1_job,
			ret_string,	    JOB_FUNC(f_job_status)},
    {"job_stop",	1, 2, FEARG_1,	    arg2_job_string_or_number,
			ret_number_bool,    JOB_FUNC(f_job_stop)},
    {"join",		1, 2, FEARG_1,	    arg2_list_any_string,
			ret_string,	    f_join},
    {"js_decode",	1, 1, FEARG_1,	    arg1_string,
			ret_any,	    f_js_decode},
    {"js_encode",	1, 1, FEARG_1,	    NULL,
			ret_string,	    f_js_encode},
    {"json_decode",	1, 1, FEARG_1,	    arg1_string,
			ret_any,	    f_json_decode},
    {"json_encode",	1, 1, FEARG_1,	    NULL,
			ret_string,	    f_json_encode},
    {"keys",		1, 1, FEARG_1,	    arg1_dict_any,
			ret_list_string,    f_keys},
    {"keytrans",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_keytrans},
    {"last_buffer_nr",	0, 0, 0,	    NULL,	// obsolete
			ret_number,	    f_last_buffer_nr},
    {"len",		1, 1, FEARG_1,	    arg1_len,
			ret_number,	    f_len},
    {"libcall",		3, 3, FEARG_3,	    arg3_libcall,
			ret_string,	    f_libcall},
    {"libcallnr",	3, 3, FEARG_3,	    arg3_libcall,
			ret_number,	    f_libcallnr},
    {"line",		1, 2, FEARG_1,	    arg2_string_number,
			ret_number,	    f_line},
    {"line2byte",	1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_line2byte},
    {"lispindent",	1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_lispindent},
    {"list2blob",	1, 1, FEARG_1,	    arg1_list_number,
			ret_blob,	    f_list2blob},
    {"list2str",	1, 2, FEARG_1,	    arg2_list_number_bool,
			ret_string,	    f_list2str},
    {"listener_add",	1, 2, FEARG_2,	    arg2_any_buffer,
			ret_number,	    f_listener_add},
    {"listener_flush",	0, 1, FEARG_1,	    arg1_buffer,
			ret_void,	    f_listener_flush},
    {"listener_remove",	1, 1, FEARG_1,	    arg1_number,
			ret_number_bool,    f_listener_remove},
    {"localtime",	0, 0, 0,	    NULL,
			ret_number,	    f_localtime},
    {"log",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_log},
    {"log10",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_log10},
    {"luaeval",		1, 2, FEARG_1,	    arg2_string_any,
			ret_any,
#ifdef FEAT_LUA
		f_luaeval
#else
		NULL
#endif
			},
    {"map",		2, 2, FEARG_1,	    arg2_map,
			ret_first_cont,	    f_map},
    {"maparg",		1, 4, FEARG_1,	    arg14_maparg,
			ret_maparg,	    f_maparg},
    {"mapcheck",	1, 3, FEARG_1,	    arg3_string_string_bool,
			ret_string,	    f_mapcheck},
    {"maplist",		0, 1, 0,	    arg1_bool,
			ret_list_dict_any,  f_maplist},
    {"mapnew",		2, 2, FEARG_1,	    arg2_mapnew,
			ret_first_cont,	    f_mapnew},
    {"mapset",		1, 3, FEARG_1,	    arg3_string_or_dict_bool_dict,
			ret_void,	    f_mapset},
    {"match",		2, 4, FEARG_1,	    arg24_match_func,
			ret_any,	    f_match},
    {"matchadd",	2, 5, FEARG_1,	    arg25_matchadd,
			ret_number,	    f_matchadd},
    {"matchaddpos",	2, 5, FEARG_1,	    arg25_matchaddpos,
			ret_number,	    f_matchaddpos},
    {"matcharg",	1, 1, FEARG_1,	    arg1_number,
			ret_list_string,    f_matcharg},
    {"matchbufline",	4, 5, FEARG_1,	    arg45_matchbufline,
			ret_list_any,	    f_matchbufline},
    {"matchdelete",	1, 2, FEARG_1,	    arg2_number,
			ret_number_bool,    f_matchdelete},
    {"matchend",	2, 4, FEARG_1,	    arg24_match_func,
			ret_number,	    f_matchend},
    {"matchfuzzy",	2, 3, FEARG_1,	    arg3_list_string_dict,
			ret_list_any,	    f_matchfuzzy},
    {"matchfuzzypos",	2, 3, FEARG_1,	    arg3_list_string_dict,
			ret_list_any,	    f_matchfuzzypos},
    {"matchlist",	2, 4, FEARG_1,	    arg24_match_func,
			ret_list_string,    f_matchlist},
    {"matchstr",	2, 4, FEARG_1,	    arg24_match_func,
			ret_string,	    f_matchstr},
    {"matchstrlist",	2, 3, FEARG_1,	    arg23_matchstrlist,
			ret_list_any,	    f_matchstrlist},
    {"matchstrpos",	2, 4, FEARG_1,	    arg24_match_func,
			ret_list_any,	    f_matchstrpos},
    {"max",		1, 1, FEARG_1,	    arg1_list_or_dict,
			ret_number,	    f_max},
    {"menu_info",	1, 2, FEARG_1,	    arg2_string,
			ret_dict_any,
#ifdef FEAT_MENU
	    f_menu_info
#else
	    NULL
#endif
			},
    {"min",		1, 1, FEARG_1,	    arg1_list_or_dict,
			ret_number,	    f_min},
    {"mkdir",		1, 3, FEARG_1,	    arg3_string_string_number,
			ret_number_bool,    f_mkdir},
    {"mode",		0, 1, FEARG_1,	    arg1_bool,
			ret_string,	    f_mode},
    {"mzeval",		1, 1, FEARG_1,	    arg1_string,
			ret_any,
#ifdef FEAT_MZSCHEME
	    f_mzeval
#else
	    NULL
#endif
			},
    {"nextnonblank",	1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_nextnonblank},
    {"nr2char",		1, 2, FEARG_1,	    arg2_number_bool,
			ret_string,	    f_nr2char},
    {"or",		2, 2, FEARG_1,	    arg2_number,
			ret_number,	    f_or},
    {"pathshorten",	1, 2, FEARG_1,	    arg2_string_number,
			ret_string,	    f_pathshorten},
    {"perleval",	1, 1, FEARG_1,	    arg1_string,
			ret_any,
#ifdef FEAT_PERL
	    f_perleval
#else
	    NULL
#endif
			},
    {"popup_atcursor",	2, 2, FEARG_1,	    arg2_str_or_nr_or_list_dict,
			ret_number,	    PROP_FUNC(f_popup_atcursor)},
    {"popup_beval",	2, 2, FEARG_1,	    arg2_str_or_nr_or_list_dict,
			ret_number,	    PROP_FUNC(f_popup_beval)},
    {"popup_clear",	0, 1, 0,	    arg1_bool,
			ret_void,	    PROP_FUNC(f_popup_clear)},
    {"popup_close",	1, 2, FEARG_1,	    arg2_number_any,
			ret_void,	    PROP_FUNC(f_popup_close)},
    {"popup_create",	2, 2, FEARG_1,	    arg2_str_or_nr_or_list_dict,
			ret_number,	    PROP_FUNC(f_popup_create)},
    {"popup_dialog",	2, 2, FEARG_1,	    arg2_str_or_nr_or_list_dict,
			ret_number,	    PROP_FUNC(f_popup_dialog)},
    {"popup_filter_menu", 2, 2, 0,	    arg2_number_string,
			ret_bool,	    PROP_FUNC(f_popup_filter_menu)},
    {"popup_filter_yesno", 2, 2, 0,	    arg2_number_string,
			ret_bool,	    PROP_FUNC(f_popup_filter_yesno)},
    {"popup_findecho",	0, 0, 0,	    NULL,
			ret_number,	    PROP_FUNC(f_popup_findecho)},
    {"popup_findinfo",	0, 0, 0,	    NULL,
			ret_number,	    PROP_FUNC(f_popup_findinfo)},
    {"popup_findpreview", 0, 0, 0,	    NULL,
			ret_number,	    PROP_FUNC(f_popup_findpreview)},
    {"popup_getoptions", 1, 1, FEARG_1,	    arg1_number,
			ret_dict_any,	    PROP_FUNC(f_popup_getoptions)},
    {"popup_getpos",	1, 1, FEARG_1,	    arg1_number,
			ret_dict_any,	    PROP_FUNC(f_popup_getpos)},
    {"popup_hide",	1, 1, FEARG_1,	    arg1_number,
			ret_void,	    PROP_FUNC(f_popup_hide)},
    {"popup_list",	0, 0, 0,	    NULL,
			ret_list_number,    PROP_FUNC(f_popup_list)},
    {"popup_locate",	2, 2, 0,	    arg2_number,
			ret_number,	    PROP_FUNC(f_popup_locate)},
    {"popup_menu",	2, 2, FEARG_1,	    arg2_str_or_nr_or_list_dict,
			ret_number,	    PROP_FUNC(f_popup_menu)},
    {"popup_move",	2, 2, FEARG_1,	    arg2_number_dict_any,
			ret_void,	    PROP_FUNC(f_popup_move)},
    {"popup_notification", 2, 2, FEARG_1,   arg2_str_or_nr_or_list_dict,
			ret_number,	    PROP_FUNC(f_popup_notification)},
    {"popup_setoptions", 2, 2, FEARG_1,	    arg2_number_dict_any,
			ret_void,	    PROP_FUNC(f_popup_setoptions)},
    {"popup_settext",	2, 2, FEARG_1,	    arg2_number_string_or_list,
			ret_void,	    PROP_FUNC(f_popup_settext)},
    {"popup_show",	1, 1, FEARG_1,	    arg1_number,
			ret_void,	    PROP_FUNC(f_popup_show)},
    {"pow",		2, 2, FEARG_1,	    arg2_float_or_nr,
			ret_float,	    f_pow},
    {"prevnonblank",	1, 1, FEARG_1,	    arg1_lnum,
			ret_number,	    f_prevnonblank},
    {"printf",		1, 19, FEARG_2,	    arg119_printf,
			ret_string,	    f_printf},
    {"prompt_getprompt", 1, 1, FEARG_1,	    arg1_buffer,
			ret_string,	    JOB_FUNC(f_prompt_getprompt)},
    {"prompt_setcallback", 2, 2, FEARG_1,   arg2_buffer_any,
			ret_void,	    JOB_FUNC(f_prompt_setcallback)},
    {"prompt_setinterrupt", 2, 2, FEARG_1,  arg2_buffer_any,
			ret_void,	    JOB_FUNC(f_prompt_setinterrupt)},
    {"prompt_setprompt", 2, 2, FEARG_1,	    arg2_buffer_string,
			ret_void,	    JOB_FUNC(f_prompt_setprompt)},
    {"prop_add",	3, 3, FEARG_1,	    arg3_number_number_dict,
			ret_number,	    PROP_FUNC(f_prop_add)},
    {"prop_add_list",	2, 2, FEARG_1,	    arg2_dict_any_list_any,
			ret_void,	    PROP_FUNC(f_prop_add_list)},
    {"prop_clear",	1, 3, FEARG_1,	    arg3_number_number_dict,
			ret_void,	    PROP_FUNC(f_prop_clear)},
    {"prop_find",	1, 2, FEARG_1,	    arg2_dict_string,
			ret_dict_any,	    PROP_FUNC(f_prop_find)},
    {"prop_list",	1, 2, FEARG_1,	    arg2_number_dict_any,
			ret_list_dict_any,  PROP_FUNC(f_prop_list)},
    {"prop_remove",	1, 3, FEARG_1,	    arg3_dict_number_number,
			ret_number,	    PROP_FUNC(f_prop_remove)},
    {"prop_type_add",	2, 2, FEARG_1,	    arg2_string_dict,
			ret_void,	    PROP_FUNC(f_prop_type_add)},
    {"prop_type_change", 2, 2, FEARG_1,	    arg2_string_dict,
			ret_void,	    PROP_FUNC(f_prop_type_change)},
    {"prop_type_delete", 1, 2, FEARG_1,	    arg2_string_dict,
			ret_void,	    PROP_FUNC(f_prop_type_delete)},
    {"prop_type_get",	1, 2, FEARG_1,	    arg2_string_dict,
			ret_dict_any,	    PROP_FUNC(f_prop_type_get)},
    {"prop_type_list",	0, 1, FEARG_1,	    arg1_dict_any,
			ret_list_string,    PROP_FUNC(f_prop_type_list)},
    {"pum_getpos",	0, 0, 0,	    NULL,
			ret_dict_number,    f_pum_getpos},
    {"pumvisible",	0, 0, 0,	    NULL,
			ret_number_bool,    f_pumvisible},
    {"py3eval",		1, 1, FEARG_1,	    arg1_string,
			ret_any,
#ifdef FEAT_PYTHON3
	    f_py3eval
#else
	    NULL
#endif
	    },
    {"pyeval",		1, 1, FEARG_1,	    arg1_string,
			ret_any,
#ifdef FEAT_PYTHON
	    f_pyeval
#else
	    NULL
#endif
			},
    {"pyxeval",		1, 1, FEARG_1,	    arg1_string,
			ret_any,
#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3)
	    f_pyxeval
#else
	    NULL
#endif
			},
    {"rand",		0, 1, FEARG_1,	    arg1_list_number,
			ret_number,	    f_rand},
    {"range",		1, 3, FEARG_1,	    arg3_number,
			ret_list_number,    f_range},
    {"readblob",	1, 3, FEARG_1,	    arg3_string_number_number,
			ret_blob,	    f_readblob},
    {"readdir",		1, 3, FEARG_1,	    arg3_string_any_dict,
			ret_list_string,    f_readdir},
    {"readdirex",	1, 3, FEARG_1,	    arg3_string_any_dict,
			ret_list_dict_any,  f_readdirex},
    {"readfile",	1, 3, FEARG_1,	    arg3_string_string_number,
			ret_list_string,    f_readfile},
    {"reduce",		2, 3, FEARG_1,	    arg23_reduce,
			ret_any,	    f_reduce},
    {"reg_executing",	0, 0, 0,	    NULL,
			ret_string,	    f_reg_executing},
    {"reg_recording",	0, 0, 0,	    NULL,
			ret_string,	    f_reg_recording},
    {"reltime",		0, 2, FEARG_1,	    arg2_list_number,
			ret_list_any,	    f_reltime},
    {"reltimefloat",	1, 1, FEARG_1,	    arg1_list_number,
			ret_float,	    f_reltimefloat},
    {"reltimestr",	1, 1, FEARG_1,	    arg1_list_number,
			ret_string,	    f_reltimestr},
    {"remote_expr",	2, 4, FEARG_1,	    arg24_remote_expr,
			ret_string,	    f_remote_expr},
    {"remote_foreground", 1, 1, FEARG_1,    arg1_string,
			ret_string,	    f_remote_foreground},
    {"remote_peek",	1, 2, FEARG_1,	    arg2_string,
			ret_number,	    f_remote_peek},
    {"remote_read",	1, 2, FEARG_1,	    arg2_string_number,
			ret_string,	    f_remote_read},
    {"remote_send",	2, 3, FEARG_1,	    arg3_string,
			ret_string,	    f_remote_send},
    {"remote_startserver", 1, 1, FEARG_1,   arg1_string,
			ret_void,	    f_remote_startserver},
    {"remove",		2, 3, FEARG_1,	    arg23_remove,
			ret_remove,	    f_remove},
    {"rename",		2, 2, FEARG_1,	    arg2_string,
			ret_number_bool,    f_rename},
    {"repeat",		2, 2, FEARG_1,	    arg2_repeat,
			ret_repeat,	    f_repeat},
    {"resolve",		1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_resolve},
    {"reverse",		1, 1, FEARG_1,	    arg1_string_or_list_or_blob_mod,
			ret_first_arg,	    f_reverse},
    {"round",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_round},
    {"rubyeval",	1, 1, FEARG_1,	    arg1_string,
			ret_any,
#ifdef FEAT_RUBY
	    f_rubyeval
#else
	    NULL
#endif
			},
    {"screenattr",	2, 2, FEARG_1,	    arg2_number,
			ret_number,	    f_screenattr},
    {"screenchar",	2, 2, FEARG_1,	    arg2_number,
			ret_number,	    f_screenchar},
    {"screenchars",	2, 2, FEARG_1,	    arg2_number,
			ret_list_number,    f_screenchars},
    {"screencol",	0, 0, 0,	    NULL,
			ret_number,	    f_screencol},
    {"screenpos",	3, 3, FEARG_1,	    arg3_number,
			ret_dict_number,    f_screenpos},
    {"screenrow",	0, 0, 0,	    NULL,
			ret_number,	    f_screenrow},
    {"screenstring",	2, 2, FEARG_1,	    arg2_number,
			ret_string,	    f_screenstring},
    {"search",		1, 5, FEARG_1,	    arg15_search,
			ret_number,	    f_search},
    {"searchcount",	0, 1, FEARG_1,	    arg1_dict_any,
			ret_dict_any,	    f_searchcount},
    {"searchdecl",	1, 3, FEARG_1,	    arg3_string_bool_bool,
			ret_number_bool,    f_searchdecl},
    {"searchpair",	3, 7, 0,	    arg37_searchpair,
			ret_number,	    f_searchpair},
    {"searchpairpos",	3, 7, 0,	    arg37_searchpair,
			ret_list_number,    f_searchpairpos},
    {"searchpos",	1, 5, FEARG_1,	    arg15_search,
			ret_list_number,    f_searchpos},
    {"server2client",	2, 2, FEARG_1,	    arg2_string,
			ret_number_bool,    f_server2client},
    {"serverlist",	0, 0, 0,	    NULL,
			ret_string,	    f_serverlist},
    {"setbufline",	3, 3, FEARG_3,	    arg3_setbufline,
			ret_number_bool,    f_setbufline},
    {"setbufvar",	3, 3, FEARG_3,	    arg3_buffer_string_any,
			ret_void,	    f_setbufvar},
    {"setcellwidths",	1, 1, FEARG_1,	    arg1_list_any,
			ret_void,	    f_setcellwidths},
    {"setcharpos",	2, 2, FEARG_2,	    arg2_string_list_number,
			ret_number_bool,    f_setcharpos},
    {"setcharsearch",	1, 1, FEARG_1,	    arg1_dict_any,
			ret_void,	    f_setcharsearch},
    {"setcmdline",	1, 2, FEARG_1,	    arg2_string_number,
			ret_number_bool,    f_setcmdline},
    {"setcmdpos",	1, 1, FEARG_1,	    arg1_number,
			ret_number_bool,    f_setcmdpos},
    {"setcursorcharpos", 1, 3, FEARG_1,	    arg13_cursor,
			ret_number_bool,    f_setcursorcharpos},
    {"setenv",		2, 2, FEARG_2,	    arg2_string_any,
			ret_void,	    f_setenv},
    {"setfperm",	2, 2, FEARG_1,	    arg2_string,
			ret_number_bool,    f_setfperm},
    {"setline",		2, 2, FEARG_2,	    arg2_setline,
			ret_number_bool,    f_setline},
    {"setloclist",	2, 4, FEARG_2,	    arg24_setloclist,
			ret_number_bool,    f_setloclist},
    {"setmatches",	1, 2, FEARG_1,	    arg2_list_any_number,
			ret_number_bool,    f_setmatches},
    {"setpos",		2, 2, FEARG_2,	    arg2_string_list_number,
			ret_number_bool,    f_setpos},
    {"setqflist",	1, 3, FEARG_1,	    arg13_setqflist,
			ret_number_bool,    f_setqflist},
    {"setreg",		2, 3, FEARG_2,	    arg3_string_any_string,
			ret_number_bool,    f_setreg},
    {"settabvar",	3, 3, FEARG_3,	    arg3_number_string_any,
			ret_void,	    f_settabvar},
    {"settabwinvar",	4, 4, FEARG_4,	    arg4_number_number_string_any,
			ret_void,	    f_settabwinvar},
    {"settagstack",	2, 3, FEARG_2,	    arg23_settagstack,
			ret_number_bool,    f_settagstack},
    {"setwinvar",	3, 3, FEARG_3,	    arg3_number_string_any,
			ret_void,	    f_setwinvar},
    {"sha256",		1, 1, FEARG_1,	    arg1_string,
			ret_string,
#ifdef FEAT_CRYPT
	    f_sha256
#else
	    NULL
#endif
			},
    {"shellescape",	1, 2, FEARG_1,	    arg2_string_bool,
			ret_string,	    f_shellescape},
    {"shiftwidth",	0, 1, FEARG_1,	    arg1_number,
			ret_number,	    f_shiftwidth},
    {"sign_define",	1, 2, FEARG_1,	    arg2_string_or_list_dict,
			ret_any,	    SIGN_FUNC(f_sign_define)},
    {"sign_getdefined",	0, 1, FEARG_1,	    arg1_string,
			ret_list_dict_any,  SIGN_FUNC(f_sign_getdefined)},
    {"sign_getplaced",	0, 2, FEARG_1,	    arg02_sign_getplaced,
			ret_list_dict_any,  SIGN_FUNC(f_sign_getplaced)},
    {"sign_jump",	3, 3, FEARG_1,	    arg3_number_string_buffer,
			ret_number,	    SIGN_FUNC(f_sign_jump)},
    {"sign_place",	4, 5, FEARG_1,	    arg45_sign_place,
			ret_number,	    SIGN_FUNC(f_sign_place)},
    {"sign_placelist",	1, 1, FEARG_1,	    arg1_list_any,
			ret_list_number,    SIGN_FUNC(f_sign_placelist)},
    {"sign_undefine",	0, 1, FEARG_1,	    arg1_string_or_list_string,
			ret_number_bool,    SIGN_FUNC(f_sign_undefine)},
    {"sign_unplace",	1, 2, FEARG_1,	    arg2_string_dict,
			ret_number_bool,    SIGN_FUNC(f_sign_unplace)},
    {"sign_unplacelist", 1, 1, FEARG_1,	    arg1_list_any,
			ret_list_number,    SIGN_FUNC(f_sign_unplacelist)},
    {"simplify",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_simplify},
    {"sin",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_sin},
    {"sinh",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_sinh},
    {"slice",		2, 3, FEARG_1,	    arg23_slice,
			ret_slice,	    f_slice},
    {"sort",		1, 3, FEARG_1,	    arg13_sortuniq,
			ret_first_arg,	    f_sort},
    {"sound_clear",	0, 0, 0,	    NULL,
			ret_void,	    SOUND_FUNC(f_sound_clear)},
    {"sound_playevent",	1, 2, FEARG_1,	    arg2_string_any,
			ret_number,	    SOUND_FUNC(f_sound_playevent)},
    {"sound_playfile",	1, 2, FEARG_1,	    arg2_string_any,
			ret_number,	    SOUND_FUNC(f_sound_playfile)},
    {"sound_stop",	1, 1, FEARG_1,	    arg1_number,
			ret_void,	    SOUND_FUNC(f_sound_stop)},
    {"soundfold",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_soundfold},
    {"spellbadword",	0, 1, FEARG_1,	    arg1_string,
			ret_list_string,    f_spellbadword},
    {"spellsuggest",	1, 3, FEARG_1,	    arg3_string_number_bool,
			ret_list_string,    f_spellsuggest},
    {"split",		1, 3, FEARG_1,	    arg3_string_string_bool,
			ret_list_string,    f_split},
    {"sqrt",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_sqrt},
    {"srand",		0, 1, FEARG_1,	    arg1_number,
			ret_list_number,    f_srand},
    {"state",		0, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_state},
    {"str2float",	1, 2, FEARG_1,	    arg2_string_bool,
			ret_float,	    f_str2float},
    {"str2list",	1, 2, FEARG_1,	    arg2_string_bool,
			ret_list_number,    f_str2list},
    {"str2nr",		1, 3, FEARG_1,	    arg3_string_number_bool,
			ret_number,	    f_str2nr},
    {"strcharlen",	1, 1, FEARG_1,	    arg1_string_or_nr,
			ret_number,	    f_strcharlen},
    {"strcharpart",	2, 4, FEARG_1,	    arg24_strpart,
			ret_string,	    f_strcharpart},
    {"strchars",	1, 2, FEARG_1,	    arg2_string_bool,
			ret_number,	    f_strchars},
    {"strdisplaywidth",	1, 2, FEARG_1,	    arg2_string_number,
			ret_number,	    f_strdisplaywidth},
    {"strftime",	1, 2, FEARG_1,	    arg2_string_number,
			ret_string,
#ifdef HAVE_STRFTIME
	    f_strftime
#else
	    NULL
#endif
			},
    {"strgetchar",	2, 2, FEARG_1,	    arg2_string_number,
			ret_number,	    f_strgetchar},
    {"stridx",		2, 3, FEARG_1,	    arg3_string_string_number,
			ret_number,	    f_stridx},
    {"string",		1, 1, FEARG_1|FE_X, NULL,
			ret_string,	    f_string},
    {"strlen",		1, 1, FEARG_1,	    arg1_string_or_nr,
			ret_number,	    f_strlen},
    {"strpart",		2, 4, FEARG_1,	    arg24_strpart,
			ret_string,	    f_strpart},
    {"strptime",	2, 2, FEARG_1,	    arg2_string,
			ret_number,
#ifdef HAVE_STRPTIME
	    f_strptime
#else
	    NULL
#endif
			},
    {"strridx",		2, 3, FEARG_1,	    arg3_string_string_number,
			ret_number,	    f_strridx},
    {"strtrans",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_strtrans},
    {"strutf16len",	1, 2, FEARG_1,	    arg2_string_bool,
			ret_number,	    f_strutf16len},
    {"strwidth",	1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_strwidth},
    {"submatch",	1, 2, FEARG_1,	    arg2_number_bool,
			ret_string,	    f_submatch},
    {"substitute",	4, 4, FEARG_1,	    arg4_string_string_any_string,
			ret_string,	    f_substitute},
    {"swapfilelist",	0, 0, 0,	    NULL,
			ret_list_string,    f_swapfilelist},
    {"swapinfo",	1, 1, FEARG_1,	    arg1_string,
			ret_dict_any,	    f_swapinfo},
    {"swapname",	1, 1, FEARG_1,	    arg1_buffer,
			ret_string,	    f_swapname},
    {"synID",		3, 3, 0,	    arg3_lnum_number_bool,
			ret_number,	    f_synID},
    {"synIDattr",	2, 3, FEARG_1,	    arg3_number_string_string,
			ret_string,	    f_synIDattr},
    {"synIDtrans",	1, 1, FEARG_1,	    arg1_number,
			ret_number,	    f_synIDtrans},
    {"synconcealed",	2, 2, 0,	    arg2_lnum_number,
			ret_list_any,	    f_synconcealed},
    {"synstack",	2, 2, 0,	    arg2_lnum_number,
			ret_list_number,    f_synstack},
    {"system",		1, 2, FEARG_1,	    arg12_system,
			ret_string,	    f_system},
    {"systemlist",	1, 2, FEARG_1,	    arg12_system,
			ret_list_string,    f_systemlist},
    {"tabpagebuflist",	0, 1, FEARG_1,	    arg1_number,
			ret_list_number,    f_tabpagebuflist},
    {"tabpagenr",	0, 1, 0,	    arg1_string,
			ret_number,	    f_tabpagenr},
    {"tabpagewinnr",	1, 2, FEARG_1,	    arg2_number_string,
			ret_number,	    f_tabpagewinnr},
    {"tagfiles",	0, 0, 0,	    NULL,
			ret_list_string,    f_tagfiles},
    {"taglist",		1, 2, FEARG_1,	    arg2_string,
			ret_list_dict_any,  f_taglist},
    {"tan",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_tan},
    {"tanh",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_tanh},
    {"tempname",	0, 0, 0,	    NULL,
			ret_string,	    f_tempname},
    {"term_dumpdiff",	2, 3, FEARG_1,	    arg3_string_string_dict,
			ret_number,	    TERM_FUNC(f_term_dumpdiff)},
    {"term_dumpload",	1, 2, FEARG_1,	    arg2_string_dict,
			ret_number,	    TERM_FUNC(f_term_dumpload)},
    {"term_dumpwrite",	2, 3, FEARG_2,	    arg3_buffer_string_dict,
			ret_void,	    TERM_FUNC(f_term_dumpwrite)},
    {"term_getaltscreen", 1, 1, FEARG_1,    arg1_buffer,
			ret_number,	    TERM_FUNC(f_term_getaltscreen)},
    {"term_getansicolors", 1, 1, FEARG_1,   arg1_buffer,
			ret_list_string,
#if defined(FEAT_TERMINAL) && (defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS))
	    f_term_getansicolors
#else
	    NULL
#endif
			},
    {"term_getattr",	2, 2, FEARG_1,	    arg2_number_string,
			ret_number,	    TERM_FUNC(f_term_getattr)},
    {"term_getcursor",	1, 1, FEARG_1,	    arg1_buffer,
			ret_list_any,	    TERM_FUNC(f_term_getcursor)},
    {"term_getjob",	1, 1, FEARG_1,	    arg1_buffer,
			ret_job,	    TERM_FUNC(f_term_getjob)},
    {"term_getline",	2, 2, FEARG_1,	    arg2_buffer_lnum,
			ret_string,	    TERM_FUNC(f_term_getline)},
    {"term_getscrolled", 1, 1, FEARG_1,	    arg1_buffer,
			ret_number,	    TERM_FUNC(f_term_getscrolled)},
    {"term_getsize",	1, 1, FEARG_1,	    arg1_buffer,
			ret_list_number,    TERM_FUNC(f_term_getsize)},
    {"term_getstatus",	1, 1, FEARG_1,	    arg1_buffer,
			ret_string,	    TERM_FUNC(f_term_getstatus)},
    {"term_gettitle",	1, 1, FEARG_1,	    arg1_buffer,
			ret_string,	    TERM_FUNC(f_term_gettitle)},
    {"term_gettty",	1, 2, FEARG_1,	    arg2_buffer_bool,
			ret_string,	    TERM_FUNC(f_term_gettty)},
    {"term_list",	0, 0, 0,	    NULL,
			ret_list_number,    TERM_FUNC(f_term_list)},
    {"term_scrape",	2, 2, FEARG_1,	    arg2_buffer_lnum,
			ret_list_dict_any,  TERM_FUNC(f_term_scrape)},
    {"term_sendkeys",	2, 2, FEARG_1,	    arg2_buffer_string,
			ret_void,	    TERM_FUNC(f_term_sendkeys)},
    {"term_setansicolors", 2, 2, FEARG_1,   arg2_buffer_list_any,
			ret_void,
#if defined(FEAT_TERMINAL) && (defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS))
	    f_term_setansicolors
#else
	    NULL
#endif
			},
    {"term_setapi",	2, 2, FEARG_1,	    arg2_buffer_string,
			ret_void,	    TERM_FUNC(f_term_setapi)},
    {"term_setkill",	2, 2, FEARG_1,	    arg2_buffer_string,
			ret_void,	    TERM_FUNC(f_term_setkill)},
    {"term_setrestore",	2, 2, FEARG_1,	    arg2_buffer_string,
			ret_void,	    TERM_FUNC(f_term_setrestore)},
    {"term_setsize",	3, 3, FEARG_1,	    arg3_buffer_number_number,
			ret_void,	    TERM_FUNC(f_term_setsize)},
    {"term_start",	1, 2, FEARG_1,	    arg2_string_or_list_dict,
			ret_number,	    TERM_FUNC(f_term_start)},
    {"term_wait",	1, 2, FEARG_1,	    arg2_buffer_number,
			ret_void,	    TERM_FUNC(f_term_wait)},
    {"terminalprops",	0, 0, 0,	    NULL,
			ret_dict_string,    f_terminalprops},
    {"test_alloc_fail",	3, 3, FEARG_1,	    arg3_number,
			ret_void,	    f_test_alloc_fail},
    {"test_autochdir",	0, 0, 0,	    NULL,
			ret_void,	    f_test_autochdir},
    {"test_feedinput",	1, 1, FEARG_1,	    arg1_string,
			ret_void,	    f_test_feedinput},
    {"test_garbagecollect_now",	0, 0, 0,    NULL,
			ret_void,	    f_test_garbagecollect_now},
    {"test_garbagecollect_soon", 0, 0, 0,   NULL,
			ret_void,	    f_test_garbagecollect_soon},
    {"test_getvalue",	1, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_test_getvalue},
    {"test_gui_event",	2, 2, FEARG_1,	    arg2_string_dict,
			ret_bool,	    f_test_gui_event},
    {"test_ignore_error", 1, 1, FEARG_1,    arg1_string,
			ret_void,	    f_test_ignore_error},
    {"test_mswin_event", 2, 2, FEARG_1,     arg2_string_dict,
			ret_number,	    f_test_mswin_event},
    {"test_null_blob",	0, 0, 0,	    NULL,
			ret_blob,	    f_test_null_blob},
    {"test_null_channel", 0, 0, 0,	    NULL,
			ret_channel,	    JOB_FUNC(f_test_null_channel)},
    {"test_null_dict",	0, 0, 0,	    NULL,
			ret_dict_any,	    f_test_null_dict},
    {"test_null_function", 0, 0, 0,	    NULL,
			ret_func_any,	    f_test_null_function},
    {"test_null_job",	0, 0, 0,	    NULL,
			ret_job,	    JOB_FUNC(f_test_null_job)},
    {"test_null_list",	0, 0, 0,	    NULL,
			ret_list_any,	    f_test_null_list},
    {"test_null_partial", 0, 0, 0,	    NULL,
			ret_func_any,	    f_test_null_partial},
    {"test_null_string", 0, 0, 0,	    NULL,
			ret_string,	    f_test_null_string},
    {"test_option_not_set", 1, 1, FEARG_1,  arg1_string,
			ret_void,	    f_test_option_not_set},
    {"test_override",	2, 2, FEARG_2,	    arg2_string_number,
			ret_void,	    f_test_override},
    {"test_refcount",	1, 1, FEARG_1|FE_X, NULL,
			ret_number,	    f_test_refcount},
    {"test_setmouse",	2, 2, 0,	    arg2_number,
			ret_void,	    f_test_setmouse},
    {"test_settime",	1, 1, FEARG_1,	    arg1_number,
			ret_void,	    f_test_settime},
    {"test_srand_seed",	0, 1, FEARG_1,	    arg1_number,
			ret_void,	    f_test_srand_seed},
    {"test_unknown",	0, 0, 0,	    NULL,
			ret_any,	    f_test_unknown},
    {"test_void",	0, 0, 0,	    NULL,
			ret_void,	    f_test_void},
    {"timer_info",	0, 1, FEARG_1,	    arg1_number,
			ret_list_dict_any,  TIMER_FUNC(f_timer_info)},
    {"timer_pause",	2, 2, FEARG_1,	    arg2_number_bool,
			ret_void,	    TIMER_FUNC(f_timer_pause)},
    {"timer_start",	2, 3, FEARG_1,	    arg3_number_any_dict,
			ret_number,	    TIMER_FUNC(f_timer_start)},
    {"timer_stop",	1, 1, FEARG_1,	    arg1_number,
			ret_void,	    TIMER_FUNC(f_timer_stop)},
    {"timer_stopall",	0, 0, 0,	    NULL,
			ret_void,	    TIMER_FUNC(f_timer_stopall)},
    {"tolower",		1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_tolower},
    {"toupper",		1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_toupper},
    {"tr",		3, 3, FEARG_1,	    arg3_string,
			ret_string,	    f_tr},
    {"trim",		1, 3, FEARG_1,	    arg3_string_string_number,
			ret_string,	    f_trim},
    {"trunc",		1, 1, FEARG_1,	    arg1_float_or_nr,
			ret_float,	    f_trunc},
    {"type",		1, 1, FEARG_1|FE_X, NULL,
			ret_number,	    f_type},
    {"typename",	1, 1, FEARG_1|FE_X, NULL,
			ret_string,	    f_typename},
    {"undofile",	1, 1, FEARG_1,	    arg1_string,
			ret_string,	    f_undofile},
    {"undotree",	0, 1, FEARG_1,	    arg1_buffer,
			ret_dict_any,	    f_undotree},
    {"uniq",		1, 3, FEARG_1,	    arg13_sortuniq,
			ret_first_arg,	    f_uniq},
    {"utf16idx",	2, 4, FEARG_1,	    arg4_string_number_bool_bool,
			ret_number,	    f_utf16idx},
    {"values",		1, 1, FEARG_1,	    arg1_dict_any,
			ret_list_member,    f_values},
    {"virtcol",		1, 3, FEARG_1,	    arg3_string_or_list_bool_number,
			ret_virtcol,	    f_virtcol},
    {"virtcol2col",	3, 3, FEARG_1,	    arg3_number,
			ret_number,	    f_virtcol2col},
    {"visualmode",	0, 1, 0,	    arg1_bool,
			ret_string,	    f_visualmode},
    {"wildmenumode",	0, 0, 0,	    NULL,
			ret_number,	    f_wildmenumode},
    {"win_execute",	2, 3, FEARG_2,	    arg23_win_execute,
			ret_string,	    f_win_execute},
    {"win_findbuf",	1, 1, FEARG_1,	    arg1_number,
			ret_list_number,    f_win_findbuf},
    {"win_getid",	0, 2, FEARG_1,	    arg2_number,
			ret_number,	    f_win_getid},
    {"win_gettype",	0, 1, FEARG_1,	    arg1_number,
			ret_string,	    f_win_gettype},
    {"win_gotoid",	1, 1, FEARG_1,	    arg1_number,
			ret_number_bool,    f_win_gotoid},
    {"win_id2tabwin",	1, 1, FEARG_1,	    arg1_number,
			ret_list_number,    f_win_id2tabwin},
    {"win_id2win",	1, 1, FEARG_1,	    arg1_number,
			ret_number,	    f_win_id2win},
    {"win_move_separator", 2, 2, FEARG_1,   arg2_number,
			ret_number_bool,    f_win_move_separator},
    {"win_move_statusline", 2, 2, FEARG_1,  arg2_number,
			ret_number_bool,    f_win_move_statusline},
    {"win_screenpos",	1, 1, FEARG_1,	    arg1_number,
			ret_list_number,    f_win_screenpos},
    {"win_splitmove",   2, 3, FEARG_1,	    arg3_number_number_dict,
			ret_number_bool,    f_win_splitmove},
    {"winbufnr",	1, 1, FEARG_1,	    arg1_number,
			ret_number,	    f_winbufnr},
    {"wincol",		0, 0, 0,	    NULL,
			ret_number,	    f_wincol},
    {"windowsversion",	0, 0, 0,	    NULL,
			ret_string,	    f_windowsversion},
    {"winheight",	1, 1, FEARG_1,	    arg1_number,
			ret_number,	    f_winheight},
    {"winlayout",	0, 1, FEARG_1,	    arg1_number,
			ret_list_any,	    f_winlayout},
    {"winline",		0, 0, 0,	    NULL,
			ret_number,	    f_winline},
    {"winnr",		0, 1, FEARG_1,	    arg1_string,
			ret_number,	    f_winnr},
    {"winrestcmd",	0, 0, 0,	    NULL,
			ret_string,	    f_winrestcmd},
    {"winrestview",	1, 1, FEARG_1,	    arg1_dict_any,
			ret_void,	    f_winrestview},
    {"winsaveview",	0, 0, 0,	    NULL,
			ret_dict_number,    f_winsaveview},
    {"winwidth",	1, 1, FEARG_1,	    arg1_number,
			ret_number,	    f_winwidth},
    {"wordcount",	0, 0, 0,	    NULL,
			ret_dict_number,    f_wordcount},
    {"writefile",	2, 3, FEARG_1,	    arg23_writefile,
			ret_number_bool,    f_writefile},
    {"xor",		2, 2, FEARG_1,	    arg2_number,
			ret_number,	    f_xor},
};

/*
 * Return true if specified function allows a type as an argument.
 */
    static int
func_allows_type(int idx)
{
    return (global_functions[idx].f_argtype & FE_X) != 0;
}

/*
 * Function given to ExpandGeneric() to obtain the list of internal
 * or user defined function names.
 */
    char_u *
get_function_name(expand_T *xp, int idx)
{
    static int	intidx = -1;
    char_u	*name;

    if (idx == 0)
	intidx = -1;
    if (intidx < 0)
    {
	name = get_user_func_name(xp, idx);
	if (name != NULL)
	{
	    if (*name != NUL && *name != '<'
				      && STRNCMP("g:", xp->xp_pattern, 2) == 0)
		return cat_prefix_varname('g', name);
	    return name;
	}
    }
    if (++intidx < (int)ARRAY_LENGTH(global_functions))
    {
	// Skip if the function doesn't have an implementation (feature not
	// implemented).
	if (global_functions[intidx].f_func == NULL)
	    return (char_u *)"";
	STRCPY(IObuff, global_functions[intidx].f_name);
	STRCAT(IObuff, "(");
	if (global_functions[intidx].f_max_argc == 0)
	    STRCAT(IObuff, ")");
	return IObuff;
    }

    return NULL;
}

/*
 * Function given to ExpandGeneric() to obtain the list of internal or
 * user defined variable or function names.
 */
    char_u *
get_expr_name(expand_T *xp, int idx)
{
    static int	intidx = -1;
    char_u	*name;

    if (idx == 0)
	intidx = -1;
    if (intidx < 0)
    {
	name = get_function_name(xp, idx);
	if (name != NULL)
	    return name;
    }
    return get_user_var_name(xp, ++intidx);
}

/*
 * Find internal function "name" in table "global_functions".
 * Return index, or -1 if not found or "implemented" is TRUE and the function
 * is not implemented.
 */
    static int
find_internal_func_opt(char_u *name, int implemented)
{
    int		first = 0;
    int		last;
    int		cmp;
    int		x;

    last = (int)ARRAY_LENGTH(global_functions) - 1;

    // Find the function name in the table. Binary search.
    while (first <= last)
    {
	x = first + ((unsigned)(last - first) >> 1);
	cmp = STRCMP(name, global_functions[x].f_name);
	if (cmp < 0)
	    last = x - 1;
	else if (cmp > 0)
	    first = x + 1;
	else if (implemented && global_functions[x].f_func == NULL)
	    break;
	else
	    return x;
    }
    return -1;
}

/*
 * Find internal function "name" in table "global_functions".
 * Return index, or -1 if not found or the function is not implemented.
 */
    int
find_internal_func(char_u *name)
{
    return find_internal_func_opt(name, TRUE);
}

    int
has_internal_func(char_u *name)
{
    return find_internal_func_opt(name, TRUE) >= 0;
}

    static int
has_internal_func_name(char_u *name)
{
    return find_internal_func_opt(name, FALSE) >= 0;
}

    char *
internal_func_name(int idx)
{
    return global_functions[idx].f_name;
}

/*
 * Check the argument types for builtin function "idx".
 * Uses the list of types on the type stack: "types".
 * Return FAIL and gives an error message when a type is wrong.
 */
    int
internal_func_check_arg_types(
	type2_T	*types,
	int	idx,
	int	argcount,
	cctx_T	*cctx)
{
    // Some internal functions accept types like Class as arguments. For other
    // functions, check the arguments are not types.
    if (!(func_allows_type(idx)))
    {
        for (int i = 0; i < argcount; ++i)
            if (check_type_is_value(types[i].type_curr) == FAIL)
		return FAIL;
    }

    argcheck_T	*argchecks = global_functions[idx].f_argcheck;

    if (argchecks == NULL)
	return OK;

    argcontext_T context;

    context.arg_count = argcount;
    context.arg_types = types;
    context.arg_cctx = cctx;
    for (int i = 0; i < argcount && argchecks[i] != NULL; ++i)
    {
	context.arg_idx = i;
	if (argchecks[i](types[i].type_curr, types[i].type_decl,
							    &context) == FAIL)
	    return FAIL;
    }
    return OK;
}

/*
 * Get the argument count for function "idx".
 * "argcount" is the total argument count, "min_argcount" the non-optional
 * argument count.
 */
    void
internal_func_get_argcount(int idx, int *argcount, int *min_argcount)
{
    *argcount = global_functions[idx].f_max_argc;
    *min_argcount = global_functions[idx].f_min_argc;
}

/*
 * Call the "f_retfunc" function to obtain the return type of function "idx".
 * "decl_type" is set to the declared type.
 * "argtypes" is the list of argument types or NULL when there are no
 * arguments.
 * "argcount" may be less than the actual count when only getting the type.
 */
    type_T *
internal_func_ret_type(
	int	    idx,
	int	    argcount,
	type2_T	    *argtypes,
	type_T	    **decl_type,
	garray_T    *type_gap)
{
    type_T *ret;

    current_type_gap = type_gap;
    *decl_type = NULL;
    ret = global_functions[idx].f_retfunc(argcount, argtypes, decl_type);
    if (*decl_type == NULL)
	*decl_type = ret;
    current_type_gap = NULL;
    return ret;
}

/*
 * Return TRUE if "idx" is for the map() function.
 */
    int
internal_func_is_map(int idx)
{
    return global_functions[idx].f_func == f_map;
}

/*
 * Check the argument count to use for internal function "idx".
 * Returns -1 for failure, 0 if no method base accepted, 1 if method base is
 * first argument, 2 if method base is second argument, etc.  9 if method base
 * is last argument.
 */
    int
check_internal_func(int idx, int argcount)
{
    funcerror_T	    res;
    char	    *name;

    if (argcount < global_functions[idx].f_min_argc)
	res = FCERR_TOOFEW;
    else if (argcount > global_functions[idx].f_max_argc)
	res = FCERR_TOOMANY;
    else
	return global_functions[idx].f_argtype & FEARG_MASK;

    name = internal_func_name(idx);
    if (res == FCERR_TOOMANY)
	semsg(_(e_too_many_arguments_for_function_str), name);
    else
	semsg(_(e_not_enough_arguments_for_function_str), name);
    return -1;
}

/*
 * Some internal functions accept types like Class as arguments. For other
 * functions, check the arguments are not types.
 *
 * Return OK/FAIL.
 */
    static int
check_args_for_type(int idx, int argcount, typval_T *argvars)
{
    if (!func_allows_type(idx))
    {
	for (int i = 0; i < argcount; ++i)
	    if (check_typval_is_value(&argvars[i]) == FAIL)
		return FAIL;
    }
    return OK;
}

    funcerror_T
call_internal_func(
	char_u	    *name,
	int	    argcount,
	typval_T    *argvars,
	typval_T    *rettv)
{
    int i;

    i = find_internal_func(name);
    if (i < 0)
	return FCERR_UNKNOWN;
    if (argcount < global_functions[i].f_min_argc)
	return FCERR_TOOFEW;
    if (argcount > global_functions[i].f_max_argc)
	return FCERR_TOOMANY;
    if (check_args_for_type(i, argcount, argvars) == FAIL)
	return FCERR_OTHER;
    argvars[argcount].v_type = VAR_UNKNOWN;
    global_functions[i].f_func(argvars, rettv);
    return FCERR_NONE;
}

    void
call_internal_func_by_idx(
	int	    idx,
	typval_T    *argvars,
	typval_T    *rettv)
{
    global_functions[idx].f_func(argvars, rettv);
}

/*
 * Invoke a method for base->method().
 */
    funcerror_T
call_internal_method(
	char_u	    *name,
	int	    argcount,
	typval_T    *argvars,
	typval_T    *rettv,
	typval_T    *basetv)
{
    int		fi;
    typval_T	argv[MAX_FUNC_ARGS + 1];

    fi = find_internal_func(name);
    if (fi < 0)
	return FCERR_UNKNOWN;
    if ((global_functions[fi].f_argtype & FEARG_MASK) == 0)
	return FCERR_NOTMETHOD;
    if (argcount + 1 < global_functions[fi].f_min_argc)
	return FCERR_TOOFEW;
    if (argcount + 1 > global_functions[fi].f_max_argc)
	return FCERR_TOOMANY;
    if (check_args_for_type(fi, argcount, argvars) == FAIL)
	return FCERR_OTHER;

    if ((global_functions[fi].f_argtype & FEARG_MASK) == FEARG_2)
    {
	if (argcount < 1)
	    return FCERR_TOOFEW;

	// base value goes second
	argv[0] = argvars[0];
	argv[1] = *basetv;
	for (int i = 1; i < argcount; ++i)
	    argv[i + 1] = argvars[i];
    }
    else if ((global_functions[fi].f_argtype & FEARG_MASK) == FEARG_3)
    {
	if (argcount < 2)
	    return FCERR_TOOFEW;

	// base value goes third
	argv[0] = argvars[0];
	argv[1] = argvars[1];
	argv[2] = *basetv;
	for (int i = 2; i < argcount; ++i)
	    argv[i + 1] = argvars[i];
    }
    else if ((global_functions[fi].f_argtype & FEARG_MASK) == FEARG_4)
    {
	if (argcount < 3)
	    return FCERR_TOOFEW;

	// base value goes fourth
	argv[0] = argvars[0];
	argv[1] = argvars[1];
	argv[2] = argvars[2];
	argv[3] = *basetv;
	for (int i = 3; i < argcount; ++i)
	    argv[i + 1] = argvars[i];
    }
    else
    {
	// FEARG_1: base value goes first
	argv[0] = *basetv;
	for (int i = 0; i < argcount; ++i)
	    argv[i + 1] = argvars[i];
    }
    argv[argcount + 1].v_type = VAR_UNKNOWN;

    if (check_args_for_type(fi, argcount + 1, argv) == FAIL)
	return FCERR_OTHER;

    global_functions[fi].f_func(argv, rettv);
    return FCERR_NONE;
}

/*
 * Return TRUE for a non-zero Number and a non-empty String.
 */
    int
non_zero_arg(typval_T *argvars)
{
    return ((argvars[0].v_type == VAR_NUMBER
		&& argvars[0].vval.v_number != 0)
	    || (argvars[0].v_type == VAR_BOOL
		&& argvars[0].vval.v_number == VVAL_TRUE)
	    || (argvars[0].v_type == VAR_STRING
		&& argvars[0].vval.v_string != NULL
		&& *argvars[0].vval.v_string != NUL));
}

/*
 * "and(expr, expr)" function
 */
    static void
f_and(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    rettv->vval.v_number = tv_get_number_chk(&argvars[0], NULL)
					& tv_get_number_chk(&argvars[1], NULL);
}

/*
 * "balloon_show()" function
 */
#ifdef FEAT_BEVAL
    static void
f_balloon_gettext(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->v_type = VAR_STRING;
    if (balloonEval == NULL)
	return;

    if (balloonEval->msg == NULL)
	rettv->vval.v_string = NULL;
    else
	rettv->vval.v_string = vim_strsave(balloonEval->msg);
}

    static void
f_balloon_show(typval_T *argvars, typval_T *rettv UNUSED)
{
    if (balloonEval == NULL)
	return;

    if (in_vim9script()
	    && check_for_string_or_list_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_LIST
# ifdef FEAT_GUI
	    && !gui.in_use
# endif
       )
    {
	list_T *l = argvars[0].vval.v_list;

	// empty list removes the balloon
	post_balloon(balloonEval, NULL,
		l == NULL || l->lv_len == 0 ? NULL : l);
    }
    else
    {
	char_u *mesg;

	if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	    return;

	mesg = tv_get_string_chk(&argvars[0]);
	if (mesg != NULL)
	    // empty string removes the balloon
	    post_balloon(balloonEval, *mesg == NUL ? NULL : mesg, NULL);
    }
}

# if defined(FEAT_BEVAL_TERM)
    static void
f_balloon_split(typval_T *argvars, typval_T *rettv UNUSED)
{
    if (rettv_list_alloc(rettv) != OK)
	return;

    char_u *msg;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;
    msg = tv_get_string_chk(&argvars[0]);
    if (msg != NULL)
    {
	pumitem_T	*array;
	int		size = split_message(msg, &array);

	// Skip the first and last item, they are always empty.
	for (int i = 1; i < size - 1; ++i)
	    list_append_string(rettv->vval.v_list, array[i].pum_text, -1);
	while (size > 0)
	    vim_free(array[--size].pum_text);
	vim_free(array);
    }
}
# endif
#endif

/*
 * Get the buffer from "arg" and give an error and return NULL if it is not
 * valid.
 */
    buf_T *
get_buf_arg(typval_T *arg)
{
    buf_T *buf;

    ++emsg_off;
    buf = tv_get_buf(arg, FALSE);
    --emsg_off;
    if (buf == NULL)
	semsg(_(e_invalid_buffer_name_str), tv_get_string(arg));
    return buf;
}

/*
 * "byte2line(byte)" function
 */
    static void
f_byte2line(typval_T *argvars UNUSED, typval_T *rettv)
{
#ifndef FEAT_BYTEOFF
    rettv->vval.v_number = -1;
#else
    long	boff = 0;

    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    boff = tv_get_number(&argvars[0]) - 1;  // boff gets -1 on type error
    if (boff < 0)
	rettv->vval.v_number = -1;
    else
	rettv->vval.v_number = ml_find_line_or_offset(curbuf,
							  (linenr_T)0, &boff);
#endif
}

/*
 * "call(func, arglist [, dict])" function
 */
    static void
f_call(typval_T *argvars, typval_T *rettv)
{
    char_u	*func;
    partial_T   *partial = NULL;
    dict_T	*selfdict = NULL;
    char_u	*dot;
    char_u	*tofree = NULL;

    if (in_vim9script()
	    && (check_for_string_or_func_arg(argvars, 0) == FAIL
		|| check_for_list_arg(argvars, 1) == FAIL
		|| check_for_opt_dict_arg(argvars, 2) == FAIL))
	return;

    if (check_for_list_arg(argvars, 1) == FAIL)
	return;
    if (argvars[1].vval.v_list == NULL)
	return;

    if (argvars[0].v_type == VAR_FUNC)
	func = argvars[0].vval.v_string;
    else if (argvars[0].v_type == VAR_PARTIAL)
    {
	partial = argvars[0].vval.v_partial;
	func = partial_name(partial);
    }
    else
	func = tv_get_string(&argvars[0]);
    if (func == NULL || *func == NUL)
	return;		// type error, empty name or null function

    dot = vim_strchr(func, '.');
    if (dot != NULL)
    {
	imported_T *import = find_imported(func, dot - func, TRUE);

	if (import != NULL && SCRIPT_ID_VALID(import->imp_sid))
	{
	    scriptitem_T *si = SCRIPT_ITEM(import->imp_sid);

	    if (si->sn_autoload_prefix != NULL)
	    {
		// Turn "import.Func" into "scriptname#Func".
		tofree = concat_str(si->sn_autoload_prefix, dot + 1);
		if (tofree == NULL)
		    return;
		func = tofree;
	    }
	}
    }

    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	if (check_for_dict_arg(argvars, 2) == FAIL)
	    return;

	selfdict = argvars[2].vval.v_dict;
    }

    (void)func_call(func, &argvars[1], partial, selfdict, rettv);

    vim_free(tofree);
}

/*
 * "changenr()" function
 */
    static void
f_changenr(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->vval.v_number = curbuf->b_u_seq_cur;
}

/*
 * "char2nr(string)" function
 */
    static void
f_char2nr(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL))
	return;

    if (has_mbyte)
    {
	int	utf8 = 0;

	if (argvars[1].v_type != VAR_UNKNOWN)
	    utf8 = (int)tv_get_bool_chk(&argvars[1], NULL);

	if (utf8)
	    rettv->vval.v_number = utf_ptr2char(tv_get_string(&argvars[0]));
	else
	    rettv->vval.v_number = (*mb_ptr2char)(tv_get_string(&argvars[0]));
    }
    else
	rettv->vval.v_number = tv_get_string(&argvars[0])[0];
}

/*
 * Get the current cursor column and store it in 'rettv'. If 'charcol' is TRUE,
 * returns the character index of the column. Otherwise, returns the byte index
 * of the column.
 */
    static void
get_col(typval_T *argvars, typval_T *rettv, int charcol)
{
    colnr_T	col = 0;
    pos_T	*fp;
    switchwin_T	switchwin;
    int		winchanged = FALSE;

    if (check_for_string_or_list_arg(argvars, 0) == FAIL
	    || check_for_opt_number_arg(argvars, 1) == FAIL)
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	tabpage_T	*tp;
	win_T		*wp;

	// use the window specified in the second argument
	wp = win_id2wp_tp((int)tv_get_number(&argvars[1]), &tp);
	if (wp == NULL || tp == NULL)
	    return;

	if (switch_win_noblock(&switchwin, wp, tp, TRUE) != OK)
	    return;

	check_cursor();
	winchanged = TRUE;
    }

    int fnum = curbuf->b_fnum;
    fp = var2fpos(&argvars[0], FALSE, &fnum, charcol);
    if (fp != NULL && fnum == curbuf->b_fnum)
    {
	if (fp->col == MAXCOL)
	{
	    // '> can be MAXCOL, get the length of the line then
	    if (fp->lnum <= curbuf->b_ml.ml_line_count)
		col = (colnr_T)STRLEN(ml_get(fp->lnum)) + 1;
	    else
		col = MAXCOL;
	}
	else
	{
	    col = fp->col + 1;
	    // col(".") when the cursor is on the NUL at the end of the line
	    // because of "coladd" can be seen as an extra column.
	    if (virtual_active() && fp == &curwin->w_cursor)
	    {
		char_u	*p = ml_get_cursor();

		if (curwin->w_cursor.coladd >= (colnr_T)chartabsize(p,
				 curwin->w_virtcol - curwin->w_cursor.coladd))
		{
		    int		l;

		    if (*p != NUL && p[(l = (*mb_ptr2len)(p))] == NUL)
			col += l;
		}
	    }
	}
    }
    rettv->vval.v_number = col;

    if (winchanged)
	restore_win_noblock(&switchwin, TRUE);
}

/*
 * "charcol()" function
 */
    static void
f_charcol(typval_T *argvars, typval_T *rettv)
{
    get_col(argvars, rettv, TRUE);
}

    win_T *
get_optional_window(typval_T *argvars, int idx)
{
    win_T   *win = curwin;

    if (argvars[idx].v_type == VAR_UNKNOWN)
	return curwin;

    win = find_win_by_nr_or_id(&argvars[idx]);
    if (win == NULL)
    {
	emsg(_(e_invalid_window_number));
	return NULL;
    }
    return win;
}

/*
 * "col(string)" function
 */
    static void
f_col(typval_T *argvars, typval_T *rettv)
{
    get_col(argvars, rettv, FALSE);
}

/*
 * "confirm(message, buttons[, default [, type]])" function
 */
    static void
f_confirm(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
#if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
    char_u	*message;
    char_u	*buttons = NULL;
    char_u	buf[NUMBUFLEN];
    char_u	buf2[NUMBUFLEN];
    int		def = 1;
    int		type = VIM_GENERIC;
    char_u	*typestr;
    int		error = FALSE;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| (check_for_opt_string_arg(argvars, 1) == FAIL
		    || (argvars[1].v_type != VAR_UNKNOWN
			&& (check_for_opt_number_arg(argvars, 2) == FAIL
			    || (argvars[2].v_type != VAR_UNKNOWN
				&& check_for_opt_string_arg(argvars, 3) == FAIL))))))
	return;

    message = tv_get_string_chk(&argvars[0]);
    if (message == NULL)
	error = TRUE;
    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	buttons = tv_get_string_buf_chk(&argvars[1], buf);
	if (buttons == NULL)
	    error = TRUE;
	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    def = (int)tv_get_number_chk(&argvars[2], &error);
	    if (argvars[3].v_type != VAR_UNKNOWN)
	    {
		typestr = tv_get_string_buf_chk(&argvars[3], buf2);
		if (typestr == NULL)
		    error = TRUE;
		else
		{
		    switch (TOUPPER_ASC(*typestr))
		    {
			case 'E': type = VIM_ERROR; break;
			case 'Q': type = VIM_QUESTION; break;
			case 'I': type = VIM_INFO; break;
			case 'W': type = VIM_WARNING; break;
			case 'G': type = VIM_GENERIC; break;
		    }
		}
	    }
	}
    }

    if (buttons == NULL || *buttons == NUL)
	buttons = (char_u *)_("&Ok");

    if (!error)
	rettv->vval.v_number = do_dialog(type, NULL, message, buttons,
							    def, NULL, FALSE);
#endif
}

/*
 * "copy()" function
 */
    static void
f_copy(typval_T *argvars, typval_T *rettv)
{
    item_copy(&argvars[0], rettv, FALSE, TRUE, 0);
}

/*
 * Set the cursor position.
 * If "charcol" is TRUE, then use the column number as a character offset.
 * Otherwise use the column number as a byte offset.
 */
    static void
set_cursorpos(typval_T *argvars, typval_T *rettv, int charcol)
{
    long	lnum, col;
    long	coladd = 0;
    int		set_curswant = TRUE;

    if (in_vim9script()
	    && (check_for_string_or_number_or_list_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 2) == FAIL)))
	return;

    rettv->vval.v_number = -1;
    if (argvars[0].v_type == VAR_LIST)
    {
	pos_T	    pos;
	colnr_T	    curswant = -1;

	if (list2fpos(argvars, &pos, NULL, &curswant, charcol) == FAIL)
	{
	    emsg(_(e_invalid_argument));
	    return;
	}
	lnum = pos.lnum;
	col = pos.col;
	coladd = pos.coladd;
	if (curswant >= 0)
	{
	    curwin->w_curswant = curswant - 1;
	    set_curswant = FALSE;
	}
    }
    else if ((argvars[0].v_type == VAR_NUMBER
					    || argvars[0].v_type == VAR_STRING)
	    && (argvars[1].v_type == VAR_NUMBER
					   || argvars[1].v_type == VAR_STRING))
    {
	lnum = tv_get_lnum(argvars);
	if (lnum < 0)
	    semsg(_(e_invalid_argument_str), tv_get_string(&argvars[0]));
	else if (lnum == 0)
	    lnum = curwin->w_cursor.lnum;
	col = (long)tv_get_number_chk(&argvars[1], NULL);
	if (charcol)
	    col = buf_charidx_to_byteidx(curbuf, lnum, col) + 1;
	if (argvars[2].v_type != VAR_UNKNOWN)
	    coladd = (long)tv_get_number_chk(&argvars[2], NULL);
    }
    else
    {
	emsg(_(e_invalid_argument));
	return;
    }
    if (lnum < 0 || col < 0 || coladd < 0)
	return;		// type error; errmsg already given
    if (lnum > 0)
	curwin->w_cursor.lnum = lnum;
    if (col > 0)
	curwin->w_cursor.col = col - 1;
    curwin->w_cursor.coladd = coladd;

    // Make sure the cursor is in a valid position.
    check_cursor();
    // Correct cursor for multi-byte character.
    if (has_mbyte)
	mb_adjust_cursor();

    curwin->w_set_curswant = set_curswant;
    rettv->vval.v_number = 0;
}

/*
 * "cursor(lnum, col)" function, or
 * "cursor(list)"
 *
 * Moves the cursor to the specified line and column.
 * Returns 0 when the position could be set, -1 otherwise.
 */
    static void
f_cursor(typval_T *argvars, typval_T *rettv)
{
    set_cursorpos(argvars, rettv, FALSE);
}

#ifdef MSWIN
/*
 * "debugbreak()" function
 */
    static void
f_debugbreak(typval_T *argvars, typval_T *rettv)
{
    int		pid;

    rettv->vval.v_number = FAIL;
    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    pid = (int)tv_get_number(&argvars[0]);
    if (pid == 0)
    {
	emsg(_(e_invalid_argument));
	return;
    }

    HANDLE hProcess = OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
    if (hProcess == NULL)
	return;

    DebugBreakProcess(hProcess);
    CloseHandle(hProcess);
    rettv->vval.v_number = OK;
}
#endif

/*
 * "deepcopy()" function
 */
    static void
f_deepcopy(typval_T *argvars, typval_T *rettv)
{
    varnumber_T	noref = 0;

    if (check_for_opt_bool_arg(argvars, 1) == FAIL)
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
	noref = tv_get_bool_chk(&argvars[1], NULL);

    item_copy(&argvars[0], rettv, TRUE, TRUE, noref == 0 ? get_copyID() : 0);
}

/*
 * "did_filetype()" function
 */
    static void
f_did_filetype(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
    rettv->vval.v_number = did_filetype;
}

/*
 * "echoraw({expr})" function
 */
    static void
f_echoraw(typval_T *argvars, typval_T *rettv UNUSED)
{
    char_u *str;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    str = tv_get_string_chk(&argvars[0]);
    if (str != NULL && *str != NUL)
    {
	out_str(str);
	out_flush();
    }
}

/*
 * "empty({expr})" function
 */
    static void
f_empty(typval_T *argvars, typval_T *rettv)
{
    int		n = FALSE;

    switch (argvars[0].v_type)
    {
	case VAR_STRING:
	case VAR_FUNC:
	    n = argvars[0].vval.v_string == NULL
					  || *argvars[0].vval.v_string == NUL;
	    break;
	case VAR_PARTIAL:
	    n = FALSE;
	    break;
	case VAR_NUMBER:
	    n = argvars[0].vval.v_number == 0;
	    break;
	case VAR_FLOAT:
	    n = argvars[0].vval.v_float == 0.0;
	    break;
	case VAR_LIST:
	    n = argvars[0].vval.v_list == NULL
					|| argvars[0].vval.v_list->lv_len == 0;
	    break;
	case VAR_DICT:
	    n = argvars[0].vval.v_dict == NULL
			|| argvars[0].vval.v_dict->dv_hashtab.ht_used == 0;
	    break;
	case VAR_BOOL:
	case VAR_SPECIAL:
	    n = argvars[0].vval.v_number != VVAL_TRUE;
	    break;
	case VAR_CLASS:
	    n = argvars[0].vval.v_class != NULL;
	    break;
	case VAR_OBJECT:
	    n = argvars[0].vval.v_object != NULL;
	    break;

	case VAR_BLOB:
	    n = argvars[0].vval.v_blob == NULL
		|| argvars[0].vval.v_blob->bv_ga.ga_len == 0;
	    break;

	case VAR_JOB:
#ifdef FEAT_JOB_CHANNEL
	    n = argvars[0].vval.v_job == NULL
			   || argvars[0].vval.v_job->jv_status != JOB_STARTED;
	    break;
#endif
	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    n = argvars[0].vval.v_channel == NULL
			       || !channel_is_open(argvars[0].vval.v_channel);
	    break;
#endif
	case VAR_TYPEALIAS:
	    n = argvars[0].vval.v_typealias == NULL
		|| argvars[0].vval.v_typealias->ta_name == NULL
		|| *argvars[0].vval.v_typealias->ta_name == NUL;
	    break;

	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	case VAR_INSTR:
	    internal_error_no_abort("f_empty(UNKNOWN)");
	    n = TRUE;
	    break;
    }

    rettv->vval.v_number = n;
}

/*
 * "environ()" function
 */
    static void
f_environ(typval_T *argvars UNUSED, typval_T *rettv)
{
#if !defined(AMIGA)
    int			i = 0;
    char_u		*entry, *value;
# if defined (MSWIN)
#  if !defined(_UCRT)
    extern wchar_t	**_wenviron;
#  endif
# else
    extern char		**environ;
# endif

    if (rettv_dict_alloc(rettv) == FAIL)
	return;

# ifdef MSWIN
    if (*_wenviron == NULL)
	return;
# else
    if (*environ == NULL)
	return;
# endif

    for (i = 0; ; ++i)
    {
# ifdef MSWIN
	short_u		*p;

	if ((p = (short_u *)_wenviron[i]) == NULL)
	    return;
	entry = utf16_to_enc(p, NULL);
# else
	if ((entry = (char_u *)environ[i]) == NULL)
	    return;
	entry = vim_strsave(entry);
# endif
	if (entry == NULL) // out of memory
	    return;
	if ((value = vim_strchr(entry, '=')) == NULL)
	{
	    vim_free(entry);
	    continue;
	}
	*value++ = NUL;
	dict_add_string(rettv->vval.v_dict, (char *)entry, value);
	vim_free(entry);
    }
#endif
}

/*
 * "err_teapot()" function
 */
    static void
f_err_teapot(typval_T *argvars, typval_T *rettv UNUSED)
{
    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	if (argvars[0].v_type == VAR_STRING)
	{
	    char_u *s = tv_get_string_strict(&argvars[0]);
	    if (*skipwhite(s) == NUL)
		return;
	}

	int err = FALSE;
	int do_503 = eval_expr_to_bool(&argvars[0], &err);
	if (!err && do_503)
	{
	    emsg(_(e_coffee_currently_not_available));
	    return;
	}
    }

    emsg(_(e_im_a_teapot));
}

/*
 * "escape({string}, {chars})" function
 */
    static void
f_escape(typval_T *argvars, typval_T *rettv)
{
    char_u	buf[NUMBUFLEN];

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL))
	return;

    rettv->vval.v_string = vim_strsave_escaped(tv_get_string(&argvars[0]),
					 tv_get_string_buf(&argvars[1], buf));
    rettv->v_type = VAR_STRING;
}

/*
 * "eval()" function
 */
    static void
f_eval(typval_T *argvars, typval_T *rettv)
{
    char_u	*s, *p;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    s = tv_get_string_chk(&argvars[0]);
    if (s != NULL)
	s = skipwhite(s);

    p = s;
    if (s == NULL || eval1(&s, rettv, &EVALARG_EVALUATE) == FAIL)
    {
	if (p != NULL && !aborting())
	    semsg(_(e_invalid_expression_str), p);
	need_clr_eos = FALSE;
	rettv->v_type = VAR_NUMBER;
	rettv->vval.v_number = 0;
    }
    else if (*s != NUL)
	semsg(_(e_trailing_characters_str), s);
}

/*
 * "eventhandler()" function
 */
    static void
f_eventhandler(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->vval.v_number = vgetc_busy || input_busy;
}

static garray_T	redir_execute_ga;

/*
 * Append "value[value_len]" to the execute() output.
 */
    void
execute_redir_str(char_u *value, int value_len)
{
    int		len;

    if (value_len == -1)
	len = (int)STRLEN(value);	// Append the entire string
    else
	len = value_len;		// Append only "value_len" characters
    if (ga_grow(&redir_execute_ga, len) == FAIL)
	return;

    mch_memmove((char *)redir_execute_ga.ga_data
	    + redir_execute_ga.ga_len, value, len);
    redir_execute_ga.ga_len += len;
}

#if defined(FEAT_LUA) || defined(PROTO)
/*
 * Get next line from a string containing NL separated lines.
 * Called by do_cmdline() to get the next line.
 * Returns an allocated string, or NULL when at the end of the string.
 */
    static char_u *
get_str_line(
    int	    c UNUSED,
    void    *cookie,
    int	    indent UNUSED,
    getline_opt_T options UNUSED)
{
    char_u	*start = *(char_u **)cookie;
    char_u	*line;
    char_u	*p;

    p = start;
    if (p == NULL || *p == NUL)
	return NULL;
    p = vim_strchr(p, '\n');
    if (p == NULL)
	line = vim_strsave(start);
    else
    {
	line = vim_strnsave(start, p - start);
	p++;
    }

    *(char_u **)cookie = p;
    return line;
}

/*
 * Execute a series of Ex commands in 'str'
 */
    void
execute_cmds_from_string(char_u *str)
{
    do_cmdline(NULL, get_str_line, (void *)&str,
	    DOCMD_NOWAIT|DOCMD_VERBOSE|DOCMD_REPEAT|DOCMD_KEYTYPED);
}
#endif

/*
 * Get next line from a list.
 * Called by do_cmdline() to get the next line.
 * Returns allocated string, or NULL for end of function.
 */
    char_u *
get_list_line(
    int	    c UNUSED,
    void    *cookie,
    int	    indent UNUSED,
    getline_opt_T options UNUSED)
{
    listitem_T **p = (listitem_T **)cookie;
    listitem_T *item = *p;
    char_u	buf[NUMBUFLEN];
    char_u	*s;

    if (item == NULL)
	return NULL;
    s = tv_get_string_buf_chk(&item->li_tv, buf);
    *p = item->li_next;
    return s == NULL ? NULL : vim_strsave(s);
}

/*
 * "execute()" function
 */
    void
execute_common(typval_T *argvars, typval_T *rettv, int arg_off)
{
    char_u	*cmd = NULL;
    list_T	*list = NULL;
    int		save_msg_silent = msg_silent;
    int		save_emsg_silent = emsg_silent;
    int		save_emsg_noredir = emsg_noredir;
    int		save_redir_execute = redir_execute;
    int		save_redir_off = redir_off;
    garray_T	save_ga;
    int		save_msg_col = msg_col;
    int		save_sticky_cmdmod_flags = sticky_cmdmod_flags;
    int		echo_output = FALSE;

    rettv->vval.v_string = NULL;
    rettv->v_type = VAR_STRING;

    if (argvars[arg_off].v_type == VAR_LIST)
    {
	list = argvars[arg_off].vval.v_list;
	if (list == NULL || list->lv_len == 0)
	    // empty list, no commands, empty output
	    return;
	++list->lv_refcount;
    }
    else if (argvars[arg_off].v_type == VAR_JOB
	    || argvars[arg_off].v_type == VAR_CHANNEL)
    {
	semsg(_(e_using_invalid_value_as_string_str),
				       vartype_name(argvars[arg_off].v_type));
	return;
    }
    else
    {
	cmd = tv_get_string_chk(&argvars[arg_off]);
	if (cmd == NULL)
	    return;
    }

    if (argvars[arg_off + 1].v_type != VAR_UNKNOWN)
    {
	char_u	buf[NUMBUFLEN];
	char_u  *s = tv_get_string_buf_chk_strict(&argvars[arg_off + 1], buf,
							      in_vim9script());

	if (s == NULL)
	    return;
	if (*s == NUL)
	    echo_output = TRUE;
	if (STRNCMP(s, "silent", 6) == 0)
	    ++msg_silent;
	if (STRCMP(s, "silent!") == 0)
	{
	    emsg_silent = TRUE;
	    emsg_noredir = TRUE;
	}
    }
    else
	++msg_silent;

    if (redir_execute)
	save_ga = redir_execute_ga;
    ga_init2(&redir_execute_ga, sizeof(char), 500);
    redir_execute = TRUE;
    redir_off = FALSE;
    if (!echo_output)
	msg_col = 0;  // prevent leading spaces

    // For "legacy call execute('cmd')" and "vim9cmd execute('cmd')" apply the
    // command modifiers to "cmd".
    sticky_cmdmod_flags = cmdmod.cmod_flags & (CMOD_LEGACY | CMOD_VIM9CMD);
    if (cmd != NULL)
	do_cmdline_cmd(cmd);
    else
    {
	listitem_T	*item;

	CHECK_LIST_MATERIALIZE(list);
	item = list->lv_first;
	do_cmdline(NULL, get_list_line, (void *)&item,
		      DOCMD_NOWAIT|DOCMD_VERBOSE|DOCMD_REPEAT|DOCMD_KEYTYPED);
	--list->lv_refcount;
    }
    sticky_cmdmod_flags = save_sticky_cmdmod_flags;

    // Need to append a NUL to the result.
    if (ga_grow(&redir_execute_ga, 1) == OK)
    {
	((char *)redir_execute_ga.ga_data)[redir_execute_ga.ga_len] = NUL;
	rettv->vval.v_string = redir_execute_ga.ga_data;
    }
    else
    {
	ga_clear(&redir_execute_ga);
	rettv->vval.v_string = NULL;
    }
    msg_silent = save_msg_silent;
    emsg_silent = save_emsg_silent;
    emsg_noredir = save_emsg_noredir;

    redir_execute = save_redir_execute;
    if (redir_execute)
	redir_execute_ga = save_ga;
    redir_off = save_redir_off;

    // "silent reg" or "silent echo x" leaves msg_col somewhere in the line.
    if (echo_output)
	// When not working silently: put it in column zero.  A following
	// "echon" will overwrite the message, unavoidably.
	msg_col = 0;
    else
	// When working silently: Put it back where it was, since nothing
	// should have been written.
	msg_col = save_msg_col;
}

/*
 * "execute()" function
 */
    static void
f_execute(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_string_or_list_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL))
	return;

    execute_common(argvars, rettv, 0);
}

/*
 * "exists()" function
 */
    void
f_exists(typval_T *argvars, typval_T *rettv)
{
    char_u	*p;
    int		n = FALSE;

    if (in_vim9script() && check_for_nonempty_string_arg(argvars, 0) == FAIL)
	return;

    p = tv_get_string(&argvars[0]);
    if (*p == '$')			// environment variable
    {
	// first try "normal" environment variables (fast)
	if (mch_getenv(p + 1) != NULL)
	    n = TRUE;
	else
	{
	    // try expanding things like $VIM and ${HOME}
	    p = expand_env_save(p);
	    if (p != NULL && *p != '$')
		n = TRUE;
	    vim_free(p);
	}
    }
    else if (*p == '&' || *p == '+')			// option
    {
	n = (eval_option(&p, NULL, TRUE) == OK);
	if (*skipwhite(p) != NUL)
	    n = FALSE;			// trailing garbage
    }
    else if (*p == '*')			// internal or user defined function
    {
	n = function_exists(p + 1, FALSE);
    }
    else if (*p == '?')			// internal function only
    {
	n = has_internal_func_name(p + 1);
    }
    else if (*p == ':')
    {
	n = cmd_exists(p + 1);
    }
    else if (*p == '#')
    {
	if (p[1] == '#')
	    n = autocmd_supported(p + 2);
	else
	    n = au_exists(p + 1);
    }
    else				// internal variable
    {
	n = var_exists(p);
    }

    rettv->vval.v_number = n;
}

    static void
f_exists_compiled(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
    emsg(_(e_exists_compiled_can_only_be_used_in_def_function));
}

/*
 * "expand()" function
 */
    static void
f_expand(typval_T *argvars, typval_T *rettv)
{
    char_u	*s;
    int		len;
    int		options = WILD_SILENT|WILD_USE_NL|WILD_LIST_NOTFOUND;
    expand_T	xpc;
    int		error = FALSE;
    char_u	*result;
#ifdef BACKSLASH_IN_FILENAME
    char_u	*p_csl_save = p_csl;
#endif

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 2) == FAIL)))
	return;

#ifdef BACKSLASH_IN_FILENAME
    // avoid using 'completeslash' here
    p_csl = empty_option;
#endif

    rettv->v_type = VAR_STRING;
    if (argvars[1].v_type != VAR_UNKNOWN
	    && argvars[2].v_type != VAR_UNKNOWN
	    && tv_get_bool_chk(&argvars[2], &error)
	    && !error)
	rettv_list_set(rettv, NULL);

    s = tv_get_string(&argvars[0]);
    if (*s == '%' || *s == '#' || *s == '<')
    {
	char	*errormsg = NULL;

	if (p_verbose == 0)
	    ++emsg_off;
	result = eval_vars(s, s, &len, NULL, &errormsg, NULL, FALSE);
	if (p_verbose == 0)
	    --emsg_off;
	else if (errormsg != NULL)
	    emsg(errormsg);
	if (rettv->v_type == VAR_LIST)
	{
	    if (rettv_list_alloc(rettv) == OK && result != NULL)
		list_append_string(rettv->vval.v_list, result, -1);
	    vim_free(result);
	}
	else
	    rettv->vval.v_string = result;
    }
    else
    {
	// When the optional second argument is non-zero, don't remove matches
	// for 'wildignore' and don't put matches for 'suffixes' at the end.
	if (argvars[1].v_type != VAR_UNKNOWN
				    && tv_get_bool_chk(&argvars[1], &error))
	    options |= WILD_KEEP_ALL;
	if (!error)
	{
	    ExpandInit(&xpc);
	    xpc.xp_context = EXPAND_FILES;
	    if (p_wic)
		options += WILD_ICASE;
	    if (rettv->v_type == VAR_STRING)
		rettv->vval.v_string = ExpandOne(&xpc, s, NULL,
							   options, WILD_ALL);
	    else if (rettv_list_alloc(rettv) == OK)
	    {
		ExpandOne(&xpc, s, NULL, options, WILD_ALL_KEEP);
		for (int i = 0; i < xpc.xp_numfiles; i++)
		    list_append_string(rettv->vval.v_list, xpc.xp_files[i], -1);
		ExpandCleanup(&xpc);
	    }
	}
	else
	    rettv->vval.v_string = NULL;
    }
#ifdef BACKSLASH_IN_FILENAME
    p_csl = p_csl_save;
#endif
}

/*
 * "expandcmd()" function
 * Expand all the special characters in a command string.
 */
    static void
f_expandcmd(typval_T *argvars, typval_T *rettv)
{
    exarg_T	eap;
    char_u	*cmdstr;
    char	*errormsg = NULL;
    int		emsgoff = TRUE;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_dict_arg(argvars, 1) == FAIL))
	return;

    if (argvars[1].v_type == VAR_DICT
		&& dict_get_bool(argvars[1].vval.v_dict, "errmsg", VVAL_FALSE))
	emsgoff = FALSE;

    rettv->v_type = VAR_STRING;
    cmdstr = vim_strsave(tv_get_string(&argvars[0]));

    CLEAR_FIELD(eap);
    eap.cmd = cmdstr;
    eap.arg = cmdstr;
    eap.argt |= EX_NOSPC;
    eap.usefilter = FALSE;
    eap.nextcmd = NULL;
    eap.cmdidx = CMD_USER;

    if (emsgoff)
	++emsg_off;
    if (expand_filename(&eap, &cmdstr, &errormsg) == FAIL)
	if (!emsgoff && errormsg != NULL && *errormsg != NUL)
	    emsg(errormsg);
    if (emsgoff)
	--emsg_off;

    rettv->vval.v_string = cmdstr;
}

/*
 * "feedkeys()" function
 */
    static void
f_feedkeys(typval_T *argvars, typval_T *rettv UNUSED)
{
    int		remap = TRUE;
    int		insert = FALSE;
    char_u	*keys, *flags;
    char_u	nbuf[NUMBUFLEN];
    int		typed = FALSE;
    int		execute = FALSE;
    int		context = FALSE;
    int		dangerous = FALSE;
    int		lowlevel = FALSE;

    // This is not allowed in the sandbox.  If the commands would still be
    // executed in the sandbox it would be OK, but it probably happens later,
    // when "sandbox" is no longer set.
    if (check_secure())
	return;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL))
	return;

    keys = tv_get_string(&argvars[0]);

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	flags = tv_get_string_buf(&argvars[1], nbuf);
	for ( ; *flags != NUL; ++flags)
	{
	    switch (*flags)
	    {
		case 'n': remap = FALSE; break;
		case 'm': remap = TRUE; break;
		case 't': typed = TRUE; break;
		case 'i': insert = TRUE; break;
		case 'x': execute = TRUE; break;
		case 'c': context = TRUE; break;
		case '!': dangerous = TRUE; break;
		case 'L': lowlevel = TRUE; break;
	    }
	}
    }

    if (*keys != NUL || execute)
    {
	if (lowlevel
#ifdef FEAT_VTP
		&& (!is_term_win32()
		    || (keys[0] == 3 && ctrl_c_interrupts && typed))
#endif
	   )
	{
#ifdef USE_INPUT_BUF
	    ch_log(NULL, "feedkeys() lowlevel: %s", keys);

	    int len = (int)STRLEN(keys);
	    for (int idx = 0; idx < len; ++idx)
	    {
		// if a CTRL-C was typed, set got_int, similar to what
		// happens in fill_input_buf()
		if (keys[idx] == 3 && ctrl_c_interrupts && typed)
		    got_int = TRUE;
		add_to_input_buf(keys + idx, 1);
	    }
#else
	    emsg(_(e_lowlevel_input_not_supported));
#endif
	}
	else
	{
	    // Need to escape K_SPECIAL and CSI before putting the string in
	    // the typeahead buffer.
	    char_u *keys_esc = vim_strsave_escape_csi(keys);
	    if (keys_esc == NULL)
		return;

	    ch_log(NULL, "feedkeys(%s): %s", typed ? "typed" : "", keys);

	    ins_typebuf(keys_esc, (remap ? REMAP_YES : REMAP_NONE),
				   insert ? 0 : typebuf.tb_len, !typed, FALSE);
	    if (vgetc_busy
#ifdef FEAT_TIMERS
		    || timer_busy
#endif
		    || input_busy)
		typebuf_was_filled = TRUE;

	    vim_free(keys_esc);
	}

	if (execute)
	{
	    int		save_msg_scroll = msg_scroll;
	    sctx_T	save_sctx;

	    // Avoid a 1 second delay when the keys start Insert mode.
	    msg_scroll = FALSE;

	    ch_log(NULL, "feedkeys() executing");

	    if (context)
	    {
		save_sctx = current_sctx;
		current_sctx.sc_sid = 0;
		current_sctx.sc_version = 0;
	    }

	    if (!dangerous)
	    {
		++ex_normal_busy;
		++in_feedkeys;
	    }
	    exec_normal(TRUE, lowlevel, TRUE);
	    if (!dangerous)
	    {
		--ex_normal_busy;
		--in_feedkeys;
	    }

	    msg_scroll |= save_msg_scroll;

	    if (context)
		current_sctx = save_sctx;
	}
    }
}

/*
 * "fnameescape({string})" function
 */
    static void
f_fnameescape(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_string = vim_strsave_fnameescape(
					 tv_get_string(&argvars[0]), VSE_NONE);
    rettv->v_type = VAR_STRING;
}

/*
 * "foreground()" function
 */
    static void
f_foreground(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
#ifdef FEAT_GUI
    if (gui.in_use)
    {
	gui_mch_set_foreground();
	return;
    }
#endif
#if defined(MSWIN) && (!defined(FEAT_GUI) || defined(VIMDLL))
    win32_set_foreground();
#endif
}

/*
 * "function()" function
 * "funcref()" function
 */
    static void
common_function(typval_T *argvars, typval_T *rettv, int is_funcref)
{
    char_u	*s;
    char_u	*name;
    int		use_string = FALSE;
    partial_T   *arg_pt = NULL;
    char_u	*trans_name = NULL;
    int		is_global = FALSE;

    if (in_vim9script()
	    && (check_for_string_or_func_arg(argvars, 0) == FAIL
		|| check_for_opt_list_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_dict_arg(argvars, 2) == FAIL)))
	return;

    if (argvars[0].v_type == VAR_FUNC)
    {
	// function(MyFunc, [arg], dict)
	s = argvars[0].vval.v_string;
    }
    else if (argvars[0].v_type == VAR_PARTIAL
					 && argvars[0].vval.v_partial != NULL)
    {
	// function(dict.MyFunc, [arg])
	arg_pt = argvars[0].vval.v_partial;
	s = partial_name(arg_pt);
    }
    else
    {
	// function('MyFunc', [arg], dict)
	s = tv_get_string(&argvars[0]);
	use_string = TRUE;
    }
    if (s == NULL)
    {
	semsg(_(e_invalid_argument_str), "NULL");
	return;
    }

    if ((use_string && vim_strchr(s, AUTOLOAD_CHAR) == NULL) || is_funcref)
    {
	name = s;
	trans_name = save_function_name(&name, &is_global, FALSE,
		   TFN_INT | TFN_QUIET | TFN_NO_AUTOLOAD | TFN_NO_DEREF, NULL);
	if (*name != NUL)
	    s = NULL;
    }

    if (s == NULL || *s == NUL || (use_string && VIM_ISDIGIT(*s))
					 || (is_funcref && trans_name == NULL))
	semsg(_(e_invalid_argument_str),
				  use_string ? tv_get_string(&argvars[0]) : s);
    // Don't check an autoload name for existence here.
    else if (trans_name != NULL && (is_funcref
			 ? find_func(trans_name, is_global) == NULL
			 : !translated_function_exists(trans_name, is_global)))
	semsg(_(e_unknown_function_str_2), s);
    else
    {
	int	dict_idx = 0;
	int	arg_idx = 0;
	list_T	*list = NULL;

	if (STRNCMP(s, "s:", 2) == 0 || STRNCMP(s, "<SID>", 5) == 0)
	    // Expand s: and <SID> into <SNR>nr_, so that the function can
	    // also be called from another script. Using trans_function_name()
	    // would also work, but some plugins depend on the name being
	    // printable text.
	    name = get_scriptlocal_funcname(s);
	else if (trans_name != NULL && *trans_name == K_SPECIAL)
	    name = alloc_printable_func_name(trans_name);
	else
	    name = vim_strsave(s);

	if (argvars[1].v_type != VAR_UNKNOWN)
	{
	    if (argvars[2].v_type != VAR_UNKNOWN)
	    {
		// function(name, [args], dict)
		arg_idx = 1;
		dict_idx = 2;
	    }
	    else if (argvars[1].v_type == VAR_DICT)
		// function(name, dict)
		dict_idx = 1;
	    else
		// function(name, [args])
		arg_idx = 1;
	    if (dict_idx > 0)
	    {
		if (check_for_dict_arg(argvars, dict_idx) == FAIL)
		{
		    vim_free(name);
		    goto theend;
		}
		if (argvars[dict_idx].vval.v_dict == NULL)
		    dict_idx = 0;
	    }
	    if (arg_idx > 0)
	    {
		if (argvars[arg_idx].v_type != VAR_LIST)
		{
		    emsg(_(e_second_argument_of_function_must_be_list_or_dict));
		    vim_free(name);
		    goto theend;
		}
		list = argvars[arg_idx].vval.v_list;
		if (list == NULL || list->lv_len == 0)
		    arg_idx = 0;
		else if (list->lv_len > MAX_FUNC_ARGS)
		{
		    emsg_funcname(e_too_many_arguments_for_function_str, s);
		    vim_free(name);
		    goto theend;
		}
	    }
	}
	if (dict_idx > 0 || arg_idx > 0 || arg_pt != NULL || is_funcref)
	{
	    partial_T	*pt = ALLOC_CLEAR_ONE(partial_T);

	    // result is a VAR_PARTIAL
	    if (pt == NULL)
		vim_free(name);
	    else
	    {
		if (arg_idx > 0 || (arg_pt != NULL && arg_pt->pt_argc > 0))
		{
		    listitem_T	*li;
		    int		i = 0;
		    int		arg_len = 0;
		    int		lv_len = 0;

		    if (arg_pt != NULL)
			arg_len = arg_pt->pt_argc;
		    if (list != NULL)
			lv_len = list->lv_len;
		    pt->pt_argc = arg_len + lv_len;
		    pt->pt_argv = ALLOC_MULT(typval_T, pt->pt_argc);
		    if (pt->pt_argv == NULL)
		    {
			vim_free(pt);
			vim_free(name);
			goto theend;
		    }
		    for (i = 0; i < arg_len; i++)
			copy_tv(&arg_pt->pt_argv[i], &pt->pt_argv[i]);
		    if (lv_len > 0)
		    {
			CHECK_LIST_MATERIALIZE(list);
			FOR_ALL_LIST_ITEMS(list, li)
			    copy_tv(&li->li_tv, &pt->pt_argv[i++]);
		    }
		}

		// For "function(dict.func, [], dict)" and "func" is a partial
		// use "dict".  That is backwards compatible.
		if (dict_idx > 0)
		{
		    // The dict is bound explicitly, pt_auto is FALSE.
		    pt->pt_dict = argvars[dict_idx].vval.v_dict;
		    ++pt->pt_dict->dv_refcount;
		}
		else if (arg_pt != NULL)
		{
		    // If the dict was bound automatically the result is also
		    // bound automatically.
		    pt->pt_dict = arg_pt->pt_dict;
		    pt->pt_auto = arg_pt->pt_auto;
		    if (pt->pt_dict != NULL)
			++pt->pt_dict->dv_refcount;
		    pt->pt_obj = arg_pt->pt_obj;
		    if (pt->pt_obj != NULL)
			++pt->pt_obj->obj_refcount;
		}

		pt->pt_refcount = 1;
		if (arg_pt != NULL && arg_pt->pt_func != NULL)
		{
		    pt->pt_func = arg_pt->pt_func;
		    func_ptr_ref(pt->pt_func);
		    vim_free(name);
		}
		else if (is_funcref)
		{
		    pt->pt_func = find_func(trans_name, is_global);
		    func_ptr_ref(pt->pt_func);
		    vim_free(name);
		}
		else
		{
		    pt->pt_name = name;
		    func_ref(name);
		}

		if (arg_pt != NULL)
		{
		    pt->pt_outer_partial = arg_pt;
		    ++arg_pt->pt_refcount;
		}
	    }
	    rettv->v_type = VAR_PARTIAL;
	    rettv->vval.v_partial = pt;
	}
	else
	{
	    // result is a VAR_FUNC
	    rettv->v_type = VAR_FUNC;
	    rettv->vval.v_string = name;
	    func_ref(name);
	}
    }
theend:
    vim_free(trans_name);
}

/*
 * "funcref()" function
 */
    static void
f_funcref(typval_T *argvars, typval_T *rettv)
{
    common_function(argvars, rettv, TRUE);
}

/*
 * "function()" function
 */
    static void
f_function(typval_T *argvars, typval_T *rettv)
{
    common_function(argvars, rettv, FALSE);
}

/*
 * "garbagecollect()" function
 */
    static void
f_garbagecollect(typval_T *argvars, typval_T *rettv UNUSED)
{
    if (in_vim9script() && check_for_opt_bool_arg(argvars, 0) == FAIL)
	return;

    // This is postponed until we are back at the toplevel, because we may be
    // using Lists and Dicts internally.  E.g.: ":echo [garbagecollect()]".
    want_garbage_collect = TRUE;

    if (argvars[0].v_type != VAR_UNKNOWN && tv_get_bool(&argvars[0]) == 1)
	garbage_collect_at_exit = TRUE;
}

/*
 * "get()" function
 */
    static void
f_get(typval_T *argvars, typval_T *rettv)
{
    listitem_T	*li;
    list_T	*l;
    dictitem_T	*di;
    dict_T	*d;
    typval_T	*tv = NULL;
    int		what_is_dict = FALSE;

    if (argvars[0].v_type == VAR_BLOB)
    {
	int error = FALSE;
	int idx = tv_get_number_chk(&argvars[1], &error);

	if (!error)
	{
	    rettv->v_type = VAR_NUMBER;
	    if (idx < 0)
		idx = blob_len(argvars[0].vval.v_blob) + idx;
	    if (idx < 0 || idx >= blob_len(argvars[0].vval.v_blob))
		rettv->vval.v_number = -1;
	    else
	    {
		rettv->vval.v_number = blob_get(argvars[0].vval.v_blob, idx);
		tv = rettv;
	    }
	}
    }
    else if (argvars[0].v_type == VAR_LIST)
    {
	if ((l = argvars[0].vval.v_list) != NULL)
	{
	    int		error = FALSE;

	    li = list_find(l, (long)tv_get_number_chk(&argvars[1], &error));
	    if (!error && li != NULL)
		tv = &li->li_tv;
	}
    }
    else if (argvars[0].v_type == VAR_DICT)
    {
	if ((d = argvars[0].vval.v_dict) != NULL)
	{
	    di = dict_find(d, tv_get_string(&argvars[1]), -1);
	    if (di != NULL)
		tv = &di->di_tv;
	}
    }
    else if (argvars[0].v_type == VAR_PARTIAL || argvars[0].v_type == VAR_FUNC)
    {
	partial_T	*pt;
	partial_T	fref_pt;

	if (argvars[0].v_type == VAR_PARTIAL)
	    pt = argvars[0].vval.v_partial;
	else
	{
	    CLEAR_FIELD(fref_pt);
	    fref_pt.pt_name = argvars[0].vval.v_string;
	    pt = &fref_pt;
	}

	if (pt != NULL)
	{
	    char_u *what = tv_get_string(&argvars[1]);

	    if (STRCMP(what, "func") == 0 || STRCMP(what, "name") == 0)
	    {
		char_u *name = partial_name(pt);

		rettv->v_type = (*what == 'f' ? VAR_FUNC : VAR_STRING);
		if (name == NULL)
		    rettv->vval.v_string = NULL;
		else
		{
		    if (rettv->v_type == VAR_FUNC)
			func_ref(name);
		    if (*what == 'n' && pt->pt_name == NULL
							&& pt->pt_func != NULL)
			// use <SNR> instead of the byte code
			name = printable_func_name(pt->pt_func);
		    rettv->vval.v_string = vim_strsave(name);
		}
	    }
	    else if (STRCMP(what, "dict") == 0)
	    {
		what_is_dict = TRUE;
		if (pt->pt_dict != NULL)
		    rettv_dict_set(rettv, pt->pt_dict);
	    }
	    else if (STRCMP(what, "args") == 0)
	    {
		rettv->v_type = VAR_LIST;
		if (rettv_list_alloc(rettv) == OK)
		{
		    int i;

		    for (i = 0; i < pt->pt_argc; ++i)
			list_append_tv(rettv->vval.v_list, &pt->pt_argv[i]);
		}
	    }
	    else
		semsg(_(e_invalid_argument_str), what);

	    // When {what} == "dict" and pt->pt_dict == NULL, evaluate the
	    // third argument
	    if (!what_is_dict)
		return;
	}
    }
    else
	semsg(_(e_argument_of_str_must_be_list_dictionary_or_blob), "get()");

    if (tv == NULL)
    {
	if (argvars[2].v_type != VAR_UNKNOWN)
	    copy_tv(&argvars[2], rettv);
    }
    else
	copy_tv(tv, rettv);
}

/*
 * "getchangelist()" function
 */
    static void
f_getchangelist(typval_T *argvars, typval_T *rettv)
{
    buf_T	*buf;
    int		i;
    list_T	*l;
    dict_T	*d;
    int		changelistindex;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_buffer_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_UNKNOWN)
	buf = curbuf;
    else
	buf = tv_get_buf_from_arg(&argvars[0]);
    if (buf == NULL)
	return;

    l = list_alloc();
    if (l == NULL)
	return;
    if (list_append_list(rettv->vval.v_list, l) == FAIL)
    {
	vim_free(l);
	return;
    }

    /*
     * The current window change list index tracks only the position for the
     * current buffer. For other buffers use the stored index for the current
     * window, or, if that's not available, the change list length.
     */
    if (buf == curwin->w_buffer)
    {
	changelistindex = curwin->w_changelistidx;
    }
    else
    {
	wininfo_T	*wip;

	FOR_ALL_BUF_WININFO(buf, wip)
	    if (wip->wi_win == curwin)
		break;
	changelistindex = wip != NULL ? wip->wi_changelistidx
							: buf->b_changelistlen;
    }
    list_append_number(rettv->vval.v_list, (varnumber_T)changelistindex);

    for (i = 0; i < buf->b_changelistlen; ++i)
    {
	if (buf->b_changelist[i].lnum == 0)
	    continue;
	if ((d = dict_alloc()) == NULL)
	    return;
	if (list_append_dict(l, d) == FAIL)
	    return;
	dict_add_number(d, "lnum", (long)buf->b_changelist[i].lnum);
	dict_add_number(d, "col", (long)buf->b_changelist[i].col);
	dict_add_number(d, "coladd", (long)buf->b_changelist[i].coladd);
    }
}

    static void
getpos_both(
    typval_T	*argvars,
    typval_T	*rettv,
    int		getcurpos,
    int		charcol)
{
    pos_T	*fp = NULL;
    pos_T	pos;
    win_T	*wp = curwin;
    list_T	*l;
    int		fnum = -1;

    if (rettv_list_alloc(rettv) == OK)
    {
	l = rettv->vval.v_list;
	if (getcurpos)
	{
	    if (argvars[0].v_type != VAR_UNKNOWN)
	    {
		wp = find_win_by_nr_or_id(&argvars[0]);
		if (wp != NULL)
		    fp = &wp->w_cursor;
	    }
	    else
		fp = &curwin->w_cursor;
	    if (fp != NULL && charcol)
	    {
		pos = *fp;
		pos.col =
		    buf_byteidx_to_charidx(wp->w_buffer, pos.lnum, pos.col);
		fp = &pos;
	    }
	}
	else
	    fp = var2fpos(&argvars[0], TRUE, &fnum, charcol);
	if (fnum != -1)
	    list_append_number(l, (varnumber_T)fnum);
	else
	    list_append_number(l, (varnumber_T)0);
	list_append_number(l, (fp != NULL) ? (varnumber_T)fp->lnum
							    : (varnumber_T)0);
	list_append_number(l, (fp != NULL)
		     ? (varnumber_T)(fp->col == MAXCOL ? MAXCOL : fp->col + 1)
							    : (varnumber_T)0);
	list_append_number(l, (fp != NULL) ? (varnumber_T)fp->coladd :
							      (varnumber_T)0);
	if (getcurpos)
	{
	    int	    save_set_curswant = curwin->w_set_curswant;
	    colnr_T save_curswant = curwin->w_curswant;
	    colnr_T save_virtcol = curwin->w_virtcol;

	    if (wp == curwin)
		update_curswant();
	    list_append_number(l, wp == NULL ? 0 : wp->w_curswant == MAXCOL
		    ?  (varnumber_T)MAXCOL : (varnumber_T)wp->w_curswant + 1);

	    // Do not change "curswant", as it is unexpected that a get
	    // function has a side effect.
	    if (wp == curwin && save_set_curswant)
	    {
		curwin->w_set_curswant = save_set_curswant;
		curwin->w_curswant = save_curswant;
		curwin->w_virtcol = save_virtcol;
		curwin->w_valid &= ~VALID_VIRTCOL;
	    }
	}
    }
    else
	rettv->vval.v_number = FALSE;
}

/*
 * "getcharpos()" function
 */
    static void
f_getcharpos(typval_T *argvars UNUSED, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    getpos_both(argvars, rettv, FALSE, TRUE);
}

/*
 * "getcharsearch()" function
 */
    static void
f_getcharsearch(typval_T *argvars UNUSED, typval_T *rettv)
{
    if (rettv_dict_alloc(rettv) != OK)
	return;

    dict_T *dict = rettv->vval.v_dict;

    dict_add_string(dict, "char", last_csearch());
    dict_add_number(dict, "forward", last_csearch_forward());
    dict_add_number(dict, "until", last_csearch_until());
}

/*
 * "getenv()" function
 */
    static void
f_getenv(typval_T *argvars, typval_T *rettv)
{
    int	    mustfree = FALSE;
    char_u  *p;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    p = vim_getenv(tv_get_string(&argvars[0]), &mustfree);
    if (p == NULL)
    {
	rettv->v_type = VAR_SPECIAL;
	rettv->vval.v_number = VVAL_NULL;
	return;
    }
    if (!mustfree)
	p = vim_strsave(p);
    rettv->vval.v_string = p;
    rettv->v_type = VAR_STRING;
}

/*
 * "getfontname()" function
 */
    static void
f_getfontname(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script() && check_for_opt_string_arg(argvars, 0) == FAIL)
	return;

#ifdef FEAT_GUI
    if (gui.in_use)
    {
	GuiFont font;
	char_u	*name = NULL;

	if (argvars[0].v_type == VAR_UNKNOWN)
	{
	    // Get the "Normal" font.  Either the name saved by
	    // hl_set_font_name() or from the font ID.
	    font = gui.norm_font;
	    name = hl_get_font_name();
	}
	else
	{
	    name = tv_get_string(&argvars[0]);
	    if (STRCMP(name, "*") == 0)	    // don't use font dialog
		return;
	    font = gui_mch_get_font(name, FALSE);
	    if (font == NOFONT)
		return;	    // Invalid font name, return empty string.
	}
	rettv->vval.v_string = gui_mch_get_fontname(font, name);
	if (argvars[0].v_type != VAR_UNKNOWN)
	    gui_mch_free_font(font);
    }
#endif
}

/*
 * "getjumplist()" function
 */
    static void
f_getjumplist(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp;
    int		i;
    list_T	*l;
    dict_T	*d;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script()
	    && (check_for_opt_number_arg(argvars, 0) == FAIL
		|| (argvars[0].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 1) == FAIL)))
	return;

    wp = find_tabwin(&argvars[0], &argvars[1], NULL);
    if (wp == NULL)
	return;

    cleanup_jumplist(wp, TRUE);

    l = list_alloc();
    if (l == NULL)
	return;
    if (list_append_list(rettv->vval.v_list, l) == FAIL)
    {
	vim_free(l);
	return;
    }

    list_append_number(rettv->vval.v_list, (varnumber_T)wp->w_jumplistidx);

    for (i = 0; i < wp->w_jumplistlen; ++i)
    {
	if (wp->w_jumplist[i].fmark.mark.lnum == 0)
	    continue;
	if ((d = dict_alloc()) == NULL)
	    return;
	if (list_append_dict(l, d) == FAIL)
	    return;
	dict_add_number(d, "lnum", (long)wp->w_jumplist[i].fmark.mark.lnum);
	dict_add_number(d, "col", (long)wp->w_jumplist[i].fmark.mark.col);
	dict_add_number(d, "coladd", (long)wp->w_jumplist[i].fmark.mark.coladd);
	dict_add_number(d, "bufnr", (long)wp->w_jumplist[i].fmark.fnum);
	if (wp->w_jumplist[i].fname != NULL)
	    dict_add_string(d, "filename", wp->w_jumplist[i].fname);
    }
}

/*
 * "getpid()" function
 */
    static void
f_getpid(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->vval.v_number = mch_get_pid();
}

/*
 * "getcurpos()" function
 */
    static void
f_getcurpos(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    getpos_both(argvars, rettv, TRUE, FALSE);
}

    static void
f_getcursorcharpos(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    getpos_both(argvars, rettv, TRUE, TRUE);
}

/*
 * "getpos(string)" function
 */
    static void
f_getpos(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    getpos_both(argvars, rettv, FALSE, FALSE);
}

/*
 * Common between getreg(), getreginfo() and getregtype(): get the register
 * name from the first argument.
 * Returns zero on error.
 */
    static int
getreg_get_regname(typval_T *argvars)
{
    char_u  *strregname;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	strregname = tv_get_string_chk(&argvars[0]);
	if (strregname != NULL && in_vim9script() && STRLEN(strregname) > 1)
	{
	    semsg(_(e_register_name_must_be_one_char_str), strregname);
	    strregname = NULL;
	}
	if (strregname == NULL)	    // type error; errmsg already given
	    return 0;
    }
    else
	// Default to v:register
	strregname = get_vim_var_str(VV_REG);

    return *strregname == 0 ? '"' : *strregname;
}

/*
 * "getreg()" function
 */
    static void
f_getreg(typval_T *argvars, typval_T *rettv)
{
    int		regname;
    int		arg2 = FALSE;
    int		return_list = FALSE;

    if (in_vim9script()
	    && (check_for_opt_string_arg(argvars, 0) == FAIL
		|| (argvars[0].v_type != VAR_UNKNOWN
		    && (check_for_opt_bool_arg(argvars, 1) == FAIL
			|| (argvars[1].v_type != VAR_UNKNOWN
			    && check_for_opt_bool_arg(argvars, 2) == FAIL)))))
	return;

    regname = getreg_get_regname(argvars);
    if (regname == 0)
	return;

    if (argvars[0].v_type != VAR_UNKNOWN && argvars[1].v_type != VAR_UNKNOWN)
    {
	int		error = FALSE;

	arg2 = (int)tv_get_bool_chk(&argvars[1], &error);

	if (!error && argvars[2].v_type != VAR_UNKNOWN)
	    return_list = (int)tv_get_bool_chk(&argvars[2], &error);
	if (error)
	    return;
    }

    if (return_list)
    {
	rettv->v_type = VAR_LIST;
	rettv->vval.v_list = (list_T *)get_reg_contents(regname,
				      (arg2 ? GREG_EXPR_SRC : 0) | GREG_LIST);
	if (rettv->vval.v_list == NULL)
	    (void)rettv_list_alloc(rettv);
	else
	    ++rettv->vval.v_list->lv_refcount;
    }
    else
    {
	rettv->v_type = VAR_STRING;
	rettv->vval.v_string = get_reg_contents(regname,
						    arg2 ? GREG_EXPR_SRC : 0);
    }
}

/*
 * "getregtype()" function
 */
    static void
f_getregtype(typval_T *argvars, typval_T *rettv)
{
    int		regname;
    char_u	buf[NUMBUFLEN + 2];
    long	reglen = 0;

    // on error return an empty string
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script() && check_for_opt_string_arg(argvars, 0) == FAIL)
	return;

    regname = getreg_get_regname(argvars);
    if (regname == 0)
	return;

    buf[0] = NUL;
    buf[1] = NUL;
    switch (get_reg_type(regname, &reglen))
    {
	case MLINE: buf[0] = 'V'; break;
	case MCHAR: buf[0] = 'v'; break;
	case MBLOCK:
		buf[0] = Ctrl_V;
		sprintf((char *)buf + 1, "%ld", reglen + 1);
		break;
    }
    rettv->vval.v_string = vim_strsave(buf);
}

/*
 * "gettagstack()" function
 */
    static void
f_gettagstack(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp = curwin;			// default is current window

    if (rettv_dict_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	wp = find_win_by_nr_or_id(&argvars[0]);
	if (wp == NULL)
	    return;
    }

    get_tagstack(wp, rettv->vval.v_dict);
}

/*
 * "gettext()" function
 */
    static void
f_gettext(typval_T *argvars, typval_T *rettv)
{
    if (check_for_nonempty_string_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = vim_strsave((char_u *)_(argvars[0].vval.v_string));
}

// for VIM_VERSION_ defines
#include "version.h"

/*
 * "has()" function
 */
    void
f_has(typval_T *argvars, typval_T *rettv)
{
    int		i;
    char_u	*name;
    int		x = FALSE;
    int		n = FALSE;
    typedef struct {
	char *name;
	short present;
    } has_item_T;
    static has_item_T has_list[] =
    {
	{"amiga",
#ifdef AMIGA
		1
#else
		0
#endif
		},
	{"arp",
#if defined(AMIGA) && defined(FEAT_ARP)
		1
#else
		0
#endif
		},
	{"haiku",
#ifdef __HAIKU__
		1
#else
		0
#endif
		},
	{"bsd",
#if defined(BSD) && !defined(MACOS_X)
		1
#else
		0
#endif
		},
	{"hpux",
#ifdef hpux
		1
#else
		0
#endif
		},
	{"linux",
#ifdef __linux__
		1
#else
		0
#endif
		},
	{"mac",		// Mac OS X (and, once, Mac OS Classic)
#ifdef MACOS_X
		1
#else
		0
#endif
		},
	{"osx",		// Mac OS X
#ifdef MACOS_X
		1
#else
		0
#endif
		},
	{"macunix",	// Mac OS X, with the darwin feature
#if defined(MACOS_X) && defined(MACOS_X_DARWIN)
		1
#else
		0
#endif
		},
	{"osxdarwin",	// synonym for macunix
#if defined(MACOS_X) && defined(MACOS_X_DARWIN)
		1
#else
		0
#endif
		},
	{"qnx",
#ifdef __QNX__
		1
#else
		0
#endif
		},
	{"sun",
#ifdef SUN_SYSTEM
		1
#else
		0
#endif
		},
	{"unix",
#ifdef UNIX
		1
#else
		0
#endif
		},
	{"vms",
#ifdef VMS
		1
#else
		0
#endif
		},
	{"win32",
#ifdef MSWIN
		1
#else
		0
#endif
		},
	{"win32unix",
#ifdef WIN32UNIX
		1
#else
		0
#endif
		},
	{"win64",
#ifdef _WIN64
		1
#else
		0
#endif
		},
	{"ebcdic", 0 },
	{"fname_case",
#ifndef CASE_INSENSITIVE_FILENAME
		1
#else
		0
#endif
		},
	{"acl",
#ifdef HAVE_ACL
		1
#else
		0
#endif
		},
	{"arabic",
#ifdef FEAT_ARABIC
		1
#else
		0
#endif
		},
	{"autocmd", 1},
	{"autochdir",
#ifdef FEAT_AUTOCHDIR
		1
#else
		0
#endif
		},
	{"autoservername",
#ifdef FEAT_AUTOSERVERNAME
		1
#else
		0
#endif
		},
	{"balloon_eval",
#ifdef FEAT_BEVAL_GUI
		1
#else
		0
#endif
		},
	{"balloon_multiline",
#ifdef FEAT_BEVAL_GUI
		1
#else
		0
#endif
		},
	{"balloon_eval_term",
#ifdef FEAT_BEVAL_TERM
		1
#else
		0
#endif
		},
	{"builtin_terms", 1},
	{"all_builtin_terms", 1},
	{"browsefilter",
#if defined(FEAT_BROWSE) && (defined(USE_FILE_CHOOSER) \
	|| defined(FEAT_GUI_MSWIN) \
	|| defined(FEAT_GUI_MOTIF))
		1
#else
		0
#endif
		},
	{"byte_offset",
#ifdef FEAT_BYTEOFF
		1
#else
		0
#endif
		},
	{"channel",
#ifdef FEAT_JOB_CHANNEL
		1
#else
		0
#endif
		},
	{"cindent", 1},
	{"clientserver",
#ifdef FEAT_CLIENTSERVER
		1
#else
		0
#endif
		},
	{"clipboard",
#ifdef FEAT_CLIPBOARD
		1
#else
		0
#endif
		},
	{"cmdline_compl", 1},
	{"cmdline_hist", 1},
	{"cmdwin", 1},
	{"comments", 1},
	{"conceal",
#ifdef FEAT_CONCEAL
		1
#else
		0
#endif
		},
	{"cryptv",
#ifdef FEAT_CRYPT
		1
#else
		0
#endif
		},
	{"crypt-blowfish",
#ifdef FEAT_CRYPT
		1
#else
		0
#endif
		},
	{"crypt-blowfish2",
#ifdef FEAT_CRYPT
		1
#else
		0
#endif
		},
	{"cscope",
#ifdef FEAT_CSCOPE
		1
#else
		0
#endif
		},
	{"cursorbind", 1},
	{"cursorshape",
#ifdef CURSOR_SHAPE
		1
#else
		0
#endif
		},
	{"debug",
#ifdef DEBUG
		1
#else
		0
#endif
		},
	{"dialog_con",
#ifdef FEAT_CON_DIALOG
		1
#else
		0
#endif
		},
	{"dialog_gui",
#ifdef FEAT_GUI_DIALOG
		1
#else
		0
#endif
		},
	{"diff",
#ifdef FEAT_DIFF
		1
#else
		0
#endif
		},
	{"digraphs",
#ifdef FEAT_DIGRAPHS
		1
#else
		0
#endif
		},
	{"directx",
#ifdef FEAT_DIRECTX
		1
#else
		0
#endif
		},
	{"dnd",
#ifdef FEAT_DND
		1
#else
		0
#endif
		},
	{"drop_file",
#ifdef HAVE_DROP_FILE
		1
#else
		0
#endif
		},
	{"emacs_tags",
#ifdef FEAT_EMACS_TAGS
		1
#else
		0
#endif
		},
	{"eval", 1},		// always present, of course!
	{"ex_extra", 1},	// graduated feature
	{"extra_search",
#ifdef FEAT_SEARCH_EXTRA
		1
#else
		0
#endif
		},
	{"file_in_path", 1},
	{"filterpipe",
#if defined(FEAT_FILTERPIPE) && !defined(VIMDLL)
		1
#else
		0
#endif
		},
	{"find_in_path",
#ifdef FEAT_FIND_ID
		1
#else
		0
#endif
		},
	{"float", 1},
	{"folding",
#ifdef FEAT_FOLDING
		1
#else
		0
#endif
		},
	{"footer", 0},
	{"fork",
#if !defined(USE_SYSTEM) && defined(UNIX)
		1
#else
		0
#endif
		},
	{"gettext",
#ifdef FEAT_GETTEXT
		1
#else
		0
#endif
		},
	{"gui",
#ifdef FEAT_GUI
		1
#else
		0
#endif
		},
	{"gui_neXtaw", 0 },
	{"gui_athena", 0 },
	{"gui_gtk",
#ifdef FEAT_GUI_GTK
		1
#else
		0
#endif
		},
	{"gui_gtk2",
#if defined(FEAT_GUI_GTK) && !defined(USE_GTK3)
		1
#else
		0
#endif
		},
	{"gui_gtk3",
#if defined(FEAT_GUI_GTK) && defined(USE_GTK3)
		1
#else
		0
#endif
		},
	{"gui_gnome",
#ifdef FEAT_GUI_GNOME
		1
#else
		0
#endif
		},
	{"gui_haiku",
#ifdef FEAT_GUI_HAIKU
		1
#else
		0
#endif
		},
	{"gui_mac", 0},
	{"gui_motif",
#ifdef FEAT_GUI_MOTIF
		1
#else
		0
#endif
		},
	{"gui_photon",
#ifdef FEAT_GUI_PHOTON
		1
#else
		0
#endif
		},
	{"gui_win32",
#ifdef FEAT_GUI_MSWIN
		1
#else
		0
#endif
		},
	{"iconv",
#if defined(HAVE_ICONV_H) && defined(USE_ICONV)
		1
#else
		0
#endif
		},
	{"insert_expand", 1},
	{"ipv6",
#ifdef FEAT_IPV6
		1
#else
		0
#endif
	},
	{"job",
#ifdef FEAT_JOB_CHANNEL
		1
#else
		0
#endif
		},
	{"jumplist", 1},
	{"keymap",
#ifdef FEAT_KEYMAP
		1
#else
		0
#endif
		},
	{"lambda", 1}, // always with FEAT_EVAL, since 7.4.2120 with closure
	{"langmap",
#ifdef FEAT_LANGMAP
		1
#else
		0
#endif
		},
	{"libcall",
#ifdef FEAT_LIBCALL
		1
#else
		0
#endif
		},
	{"linebreak",
#ifdef FEAT_LINEBREAK
		1
#else
		0
#endif
		},
	{"lispindent", 1},
	{"listcmds", 1},
	{"localmap", 1},
	{"lua",
#if defined(FEAT_LUA) && !defined(DYNAMIC_LUA)
		1
#else
		0
#endif
		},
	{"menu",
#ifdef FEAT_MENU
		1
#else
		0
#endif
		},
	{"mksession",
#ifdef FEAT_SESSION
		1
#else
		0
#endif
		},
	{"modify_fname", 1},
	{"mouse", 1},
	{"mouseshape",
#ifdef FEAT_MOUSESHAPE
		1
#else
		0
#endif
		},
	{"mouse_dec",
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_MOUSE_DEC)
		1
#else
		0
#endif
		},
	{"mouse_gpm",
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_MOUSE_GPM) && !defined(DYNAMIC_GPM)
		1
#else
		0
#endif
		},
	{"mouse_jsbterm",
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_MOUSE_JSB)
		1
#else
		0
#endif
		},
	{"mouse_netterm",
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_MOUSE_NET)
		1
#else
		0
#endif
		},
	{"mouse_pterm",
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_MOUSE_PTERM)
		1
#else
		0
#endif
		},
	{"mouse_sgr",
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_MOUSE_XTERM)
		1
#else
		0
#endif
		},
	{"mouse_sysmouse",
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_SYSMOUSE)
		1
#else
		0
#endif
		},
	{"mouse_urxvt",
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_MOUSE_URXVT)
		1
#else
		0
#endif
		},
	{"mouse_xterm",
#if (defined(UNIX) || defined(VMS)) && defined(FEAT_MOUSE_XTERM)
		1
#else
		0
#endif
		},
	{"multi_byte", 1},
	{"multi_byte_ime",
#ifdef FEAT_MBYTE_IME
		1
#else
		0
#endif
		},
	{"multi_lang",
#ifdef FEAT_MULTI_LANG
		1
#else
		0
#endif
		},
	{"mzscheme",
#if defined(FEAT_MZSCHEME) && !defined(DYNAMIC_MZSCHEME)
		1
#else
		0
#endif
		},
	{"nanotime",
#ifdef ST_MTIM_NSEC
		1
#else
		0
#endif
	},
	{"num64", 1},
	{"ole",
#ifdef FEAT_OLE
		1
#else
		0
#endif
		},
	{"packages",
#ifdef FEAT_EVAL
		1
#else
		0
#endif
		},
	{"path_extra", 1},
	{"perl",
#if defined(FEAT_PERL) && !defined(DYNAMIC_PERL)
		1
#else
		0
#endif
		},
	{"persistent_undo",
#ifdef FEAT_PERSISTENT_UNDO
		1
#else
		0
#endif
		},
	{"python_compiled",
#if defined(FEAT_PYTHON)
		1
#else
		0
#endif
		},
	{"python_dynamic",
#if defined(FEAT_PYTHON) && defined(DYNAMIC_PYTHON)
		1
#else
		0
#endif
		},
	{"python",
#if defined(FEAT_PYTHON) && !defined(DYNAMIC_PYTHON)
		1
#else
		0
#endif
		},
	{"pythonx",
#if (defined(FEAT_PYTHON) && !defined(DYNAMIC_PYTHON)) \
	|| (defined(FEAT_PYTHON3) && !defined(DYNAMIC_PYTHON3))
		1
#else
		0
#endif
		},
	{"python3_compiled",
#if defined(FEAT_PYTHON3)
		1
#else
		0
#endif
		},
	{"python3_dynamic",
#if defined(FEAT_PYTHON3) && defined(DYNAMIC_PYTHON3)
		1
#else
		0
#endif
		},
	{"python3_stable",
#if defined(FEAT_PYTHON3) && defined(DYNAMIC_PYTHON3_STABLE_ABI)
		1
#else
		0
#endif
		},
	{"python3",
#if defined(FEAT_PYTHON3) && !defined(DYNAMIC_PYTHON3)
		1
#else
		0
#endif
		},
	{"popupwin",
#ifdef FEAT_PROP_POPUP
		1
#else
		0
#endif
		},
	{"postscript",
#ifdef FEAT_POSTSCRIPT
		1
#else
		0
#endif
		},
	{"printer",
#ifdef FEAT_PRINTER
		1
#else
		0
#endif
		},
	{"profile",
#ifdef FEAT_PROFILE
		1
#else
		0
#endif
		},
	{"prof_nsec",
#ifdef PROF_NSEC
		1
#else
		0
#endif
		},
	{"reltime",
#ifdef FEAT_RELTIME
		1
#else
		0
#endif
		},
	{"quickfix",
#ifdef FEAT_QUICKFIX
		1
#else
		0
#endif
		},
	{"rightleft",
#ifdef FEAT_RIGHTLEFT
		1
#else
		0
#endif
		},
	{"ruby",
#if defined(FEAT_RUBY) && !defined(DYNAMIC_RUBY)
		1
#else
		0
#endif
		},
	{"scrollbind", 1},
	{"showcmd", 1},
	{"cmdline_info", 1},
	{"signs",
#ifdef FEAT_SIGNS
		1
#else
		0
#endif
		},
	{"smartindent", 1},
	{"startuptime",
#ifdef STARTUPTIME
		1
#else
		0
#endif
		},
	{"statusline",
#ifdef FEAT_STL_OPT
		1
#else
		0
#endif
		},
	{"netbeans_intg",
#ifdef FEAT_NETBEANS_INTG
		1
#else
		0
#endif
		},
	{"sodium",
#if defined(FEAT_SODIUM) && !defined(DYNAMIC_SODIUM)
		1
#else
		0
#endif
		},
	{"sound",
#ifdef FEAT_SOUND
		1
#else
		0
#endif
		},
	{"spell",
#ifdef FEAT_SPELL
		1
#else
		0
#endif
		},
	{"syntax",
#ifdef FEAT_SYN_HL
		1
#else
		0
#endif
		},
	{"system",
#if defined(USE_SYSTEM) || !defined(UNIX)
		1
#else
		0
#endif
		},
	{"tag_binary", 1},	// graduated feature
	{"tcl",
#if defined(FEAT_TCL) && !defined(DYNAMIC_TCL)
		1
#else
		0
#endif
		},
	{"termguicolors",
#ifdef FEAT_TERMGUICOLORS
		1
#else
		0
#endif
		},
	{"terminal",
#if defined(FEAT_TERMINAL) && !defined(MSWIN)
		1
#else
		0
#endif
		},
	{"terminfo",
#ifdef TERMINFO
		1
#else
		0
#endif
		},
	{"termresponse",
#ifdef FEAT_TERMRESPONSE
		1
#else
		0
#endif
		},
	{"textobjects", 1},
	{"textprop",
#ifdef FEAT_PROP_POPUP
		1
#else
		0
#endif
		},
	{"tgetent",
#ifdef HAVE_TGETENT
		1
#else
		0
#endif
		},
	{"timers",
#ifdef FEAT_TIMERS
		1
#else
		0
#endif
		},
	{"title", 1},
	{"toolbar",
#ifdef FEAT_TOOLBAR
		1
#else
		0
#endif
		},
	{"unnamedplus",
#if defined(FEAT_CLIPBOARD) && defined(FEAT_X11)
		1
#else
		0
#endif
		},
	{"user-commands", 1},    // was accidentally included in 5.4
	{"user_commands", 1},
	{"vartabs",
#ifdef FEAT_VARTABS
		1
#else
		0
#endif
		},
	{"vertsplit", 1},
	{"viminfo",
#ifdef FEAT_VIMINFO
		1
#else
		0
#endif
		},
	{"vim9script", 1},
	{"vimscript-1", 1},
	{"vimscript-2", 1},
	{"vimscript-3", 1},
	{"vimscript-4", 1},
	{"virtualedit", 1},
	{"visual", 1},
	{"visualextra", 1},
	{"vreplace", 1},
	{"vtp",
#ifdef FEAT_VTP
		1
#else
		0
#endif
		},
	{"wildignore", 1},
	{"wildmenu", 1},
	{"windows", 1},
	{"winaltkeys",
#ifdef FEAT_WAK
		1
#else
		0
#endif
		},
	{"writebackup",
#ifdef FEAT_WRITEBACKUP
		1
#else
		0
#endif
		},
	{"xattr",
#ifdef FEAT_XATTR
		1
#else
		0
#endif
		},
	{"xim",
#ifdef FEAT_XIM
		1
#else
		0
#endif
		},
	{"xfontset",
#ifdef FEAT_XFONTSET
		1
#else
		0
#endif
		},
	{"xpm",
#if defined(FEAT_XPM_W32) || defined(HAVE_XPM)
		1
#else
		0
#endif
		},
	{"xpm_w32",	// for backward compatibility
#ifdef FEAT_XPM_W32
		1
#else
		0
#endif
		},
	{"xsmp",
#ifdef USE_XSMP
		1
#else
		0
#endif
		},
	{"xsmp_interact",
#ifdef USE_XSMP_INTERACT
		1
#else
		0
#endif
		},
	{"xterm_clipboard",
#ifdef FEAT_XCLIPBOARD
		1
#else
		0
#endif
		},
	{"xterm_save",
#ifdef FEAT_XTERM_SAVE
		1
#else
		0
#endif
		},
	{"X11",
#if defined(UNIX) && defined(FEAT_X11)
		1
#else
		0
#endif
		},
	{":tearoff",
// same #ifdef as used for ex_tearoff().
#if defined(FEAT_GUI_MSWIN) && defined(FEAT_MENU) && defined(FEAT_TEAROFF)
		1
#else
		0
#endif
		},
	{NULL, 0}
    };

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL))
	return;

    name = tv_get_string(&argvars[0]);
    for (i = 0; has_list[i].name != NULL; ++i)
	if (STRICMP(name, has_list[i].name) == 0)
	{
	    x = TRUE;
	    n = has_list[i].present;
	    break;
	}

    // features also in has_list[] but sometimes enabled at runtime
    if (x == TRUE && n == FALSE)
    {
	if (0)
	{
	    // intentionally empty
	}
#ifdef VIMDLL
	else if (STRICMP(name, "filterpipe") == 0)
	    n = gui.in_use || gui.starting;
#endif
#if defined(USE_ICONV) && defined(DYNAMIC_ICONV)
	else if (STRICMP(name, "iconv") == 0)
	    n = iconv_enabled(FALSE);
#endif
#ifdef DYNAMIC_LUA
	else if (STRICMP(name, "lua") == 0)
	    n = lua_enabled(FALSE);
#endif
#ifdef DYNAMIC_MZSCHEME
	else if (STRICMP(name, "mzscheme") == 0)
	    n = mzscheme_enabled(FALSE);
#endif
#ifdef DYNAMIC_PERL
	else if (STRICMP(name, "perl") == 0)
	    n = perl_enabled(FALSE);
#endif
#ifdef DYNAMIC_PYTHON
	else if (STRICMP(name, "python") == 0)
	    n = python_enabled(FALSE);
#endif
#ifdef DYNAMIC_PYTHON3
	else if (STRICMP(name, "python3") == 0)
	    n = python3_enabled(FALSE);
#endif
#if defined(DYNAMIC_PYTHON) || defined(DYNAMIC_PYTHON3)
	else if (STRICMP(name, "pythonx") == 0)
	{
# if defined(DYNAMIC_PYTHON) && defined(DYNAMIC_PYTHON3)
	    if (p_pyx == 0)
		n = python3_enabled(FALSE) || python_enabled(FALSE);
	    else if (p_pyx == 3)
		n = python3_enabled(FALSE);
	    else if (p_pyx == 2)
		n = python_enabled(FALSE);
# elif defined(DYNAMIC_PYTHON)
	    n = python_enabled(FALSE);
# elif defined(DYNAMIC_PYTHON3)
	    n = python3_enabled(FALSE);
# endif
	}
#endif
#ifdef DYNAMIC_RUBY
	else if (STRICMP(name, "ruby") == 0)
	    n = ruby_enabled(FALSE);
#endif
#ifdef DYNAMIC_TCL
	else if (STRICMP(name, "tcl") == 0)
	    n = tcl_enabled(FALSE);
#endif
#ifdef DYNAMIC_SODIUM
	else if (STRICMP(name, "sodium") == 0)
	    n = sodium_enabled(FALSE);
#endif
#if defined(FEAT_TERMINAL) && defined(MSWIN)
	else if (STRICMP(name, "terminal") == 0)
	    n = terminal_enabled();
#endif
#ifdef DYNAMIC_GPM
	else if (STRICMP(name, "mouse_gpm") == 0)
	    n = gpm_available();
#endif
    }

    // features not in has_list[]
    if (x == FALSE)
    {
	if (STRNICMP(name, "patch", 5) == 0)
	{
	    x = TRUE;
	    if (name[5] == '-'
		    && STRLEN(name) >= 11
		    && (name[6] >= '1' && name[6] <= '9'))
	    {
		char	*end;
		int	major, minor;

		// This works for patch-8.1.2, patch-9.0.3, patch-10.0.4, etc.
		// Not for patch-9.10.5.
		major = (int)strtoul((char *)name + 6, &end, 10);
		if (*end == '.' && vim_isdigit(end[1])
			&& end[2] == '.' && vim_isdigit(end[3]))
		{
		    minor = atoi(end + 1);

		    // Expect "patch-9.9.01234".
		    n = (major < VIM_VERSION_MAJOR
			 || (major == VIM_VERSION_MAJOR
			     && (minor < VIM_VERSION_MINOR
				 || (minor == VIM_VERSION_MINOR
				     && has_patch(atoi(end + 3))))));
		}
	    }
	    else if (SAFE_isdigit(name[5]))
		n = has_patch(atoi((char *)name + 5));
	}
	else if (STRICMP(name, "vim_starting") == 0)
	{
	    x = TRUE;
	    n = (starting != 0);
	}
	else if (STRICMP(name, "ttyin") == 0)
	{
	    x = TRUE;
	    n = mch_input_isatty();
	}
	else if (STRICMP(name, "ttyout") == 0)
	{
	    x = TRUE;
	    n = stdout_isatty;
	}
	else if (STRICMP(name, "multi_byte_encoding") == 0)
	{
	    x = TRUE;
	    n = has_mbyte;
	}
	else if (STRICMP(name, "gui_running") == 0)
	{
	    x = TRUE;
#ifdef FEAT_GUI
	    n = (gui.in_use || gui.starting);
#endif
	}
	else if (STRICMP(name, "browse") == 0)
	{
	    x = TRUE;
#if defined(FEAT_GUI) && defined(FEAT_BROWSE)
	    n = gui.in_use;	// gui_mch_browse() works when GUI is running
#endif
	}
	else if (STRICMP(name, "syntax_items") == 0)
	{
	    x = TRUE;
#ifdef FEAT_SYN_HL
	    n = syntax_present(curwin);
#endif
	}
	else if (STRICMP(name, "vcon") == 0)
	{
	    x = TRUE;
#ifdef FEAT_VTP
	    n = is_term_win32() && has_vtp_working();
#endif
	}
	else if (STRICMP(name, "netbeans_enabled") == 0)
	{
	    x = TRUE;
#ifdef FEAT_NETBEANS_INTG
	    n = netbeans_active();
#endif
	}
	else if (STRICMP(name, "mouse_gpm_enabled") == 0)
	{
	    x = TRUE;
#ifdef FEAT_MOUSE_GPM
	    n = gpm_enabled();
#endif
	}
	else if (STRICMP(name, "conpty") == 0)
	{
	    x = TRUE;
#if defined(FEAT_TERMINAL) && defined(MSWIN)
	    n = use_conpty();
#endif
	}
	else if (STRICMP(name, "clipboard_working") == 0)
	{
	    x = TRUE;
#ifdef FEAT_CLIPBOARD
	    n = clip_star.available;
#endif
	}
    }

    if (argvars[1].v_type != VAR_UNKNOWN && tv_get_bool(&argvars[1]))
	// return whether feature could ever be enabled
	rettv->vval.v_number = x;
    else
	// return whether feature is enabled
	rettv->vval.v_number = n;
}

/*
 * Return TRUE if "feature" can change later.
 * Also when checking for the feature has side effects, such as loading a DLL.
 */
    int
dynamic_feature(char_u *feature)
{
    return (feature == NULL
#if defined(FEAT_GUI) && defined(FEAT_BROWSE)
	    || (STRICMP(feature, "browse") == 0 && !gui.in_use)
#endif
#ifdef VIMDLL
	    || STRICMP(feature, "filterpipe") == 0
#endif
#if defined(FEAT_GUI) && !defined(ALWAYS_USE_GUI) && !defined(VIMDLL)
	    // this can only change on Unix where the ":gui" command could be
	    // used.
	    || (STRICMP(feature, "gui_running") == 0 && !gui.in_use)
#endif
#if defined(USE_ICONV) && defined(DYNAMIC_ICONV)
	    || STRICMP(feature, "iconv") == 0
#endif
#ifdef DYNAMIC_LUA
	    || STRICMP(feature, "lua") == 0
#endif
#ifdef FEAT_MOUSE_GPM
	    || (STRICMP(feature, "mouse_gpm_enabled") == 0 && !gpm_enabled())
#endif
#ifdef DYNAMIC_MZSCHEME
	    || STRICMP(feature, "mzscheme") == 0
#endif
#ifdef FEAT_NETBEANS_INTG
	    || STRICMP(feature, "netbeans_enabled") == 0
#endif
#ifdef DYNAMIC_PERL
	    || STRICMP(feature, "perl") == 0
#endif
#ifdef DYNAMIC_PYTHON
	    || STRICMP(feature, "python") == 0
#endif
#ifdef DYNAMIC_PYTHON3
	    || STRICMP(feature, "python3") == 0
#endif
#if defined(DYNAMIC_PYTHON) || defined(DYNAMIC_PYTHON3)
	    || STRICMP(feature, "pythonx") == 0
#endif
#ifdef DYNAMIC_RUBY
	    || STRICMP(feature, "ruby") == 0
#endif
#ifdef FEAT_SYN_HL
	    || STRICMP(feature, "syntax_items") == 0
#endif
#ifdef DYNAMIC_TCL
	    || STRICMP(feature, "tcl") == 0
#endif
	    // once "starting" is zero it will stay that way
	    || (STRICMP(feature, "vim_starting") == 0 && starting != 0)
	    || STRICMP(feature, "multi_byte_encoding") == 0
#if defined(FEAT_TERMINAL) && defined(MSWIN)
	    || STRICMP(feature, "conpty") == 0
#endif
	    );
}

/*
 * "haslocaldir()" function
 */
    static void
f_haslocaldir(typval_T *argvars, typval_T *rettv)
{
    tabpage_T	*tp = NULL;
    win_T	*wp = NULL;

    if (in_vim9script()
	    && (check_for_opt_number_arg(argvars, 0) == FAIL
		|| (argvars[0].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 1) == FAIL)))
	return;

    wp = find_tabwin(&argvars[0], &argvars[1], &tp);

    // Check for window-local and tab-local directories
    if (wp != NULL && wp->w_localdir != NULL)
	rettv->vval.v_number = 1;
    else if (tp != NULL && tp->tp_localdir != NULL)
	rettv->vval.v_number = 2;
    else
	rettv->vval.v_number = 0;
}

/*
 * "highlightID(name)" function
 */
    static void
f_hlID(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_number = syn_name2id(tv_get_string(&argvars[0]));
}

/*
 * "highlight_exists()" function
 */
    static void
f_hlexists(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_number = highlight_exists(tv_get_string(&argvars[0]));
}

/*
 * "hostname()" function
 */
    static void
f_hostname(typval_T *argvars UNUSED, typval_T *rettv)
{
    char_u hostname[256];

    mch_get_host_name(hostname, 256);
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = vim_strsave(hostname);
}

/*
 * "index()" function
 */
    static void
f_index(typval_T *argvars, typval_T *rettv)
{
    list_T	*l;
    listitem_T	*item;
    blob_T	*b;
    long	idx = 0;
    int		ic = FALSE;
    int		error = FALSE;

    rettv->vval.v_number = -1;

    if (in_vim9script()
	    && (check_for_list_or_blob_arg(argvars, 0) == FAIL
		|| (argvars[0].v_type == VAR_BLOB
		    && check_for_number_arg(argvars, 1) == FAIL)
		|| check_for_opt_number_arg(argvars, 2) == FAIL
		|| (argvars[2].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 3) == FAIL)))
	return;

    if (argvars[0].v_type == VAR_BLOB)
    {
	typval_T	tv;
	int		start = 0;

	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    start = tv_get_number_chk(&argvars[2], &error);
	    if (error)
		return;
	}
	b = argvars[0].vval.v_blob;
	if (b == NULL)
	    return;
	if (start < 0)
	{
	    start = blob_len(b) + start;
	    if (start < 0)
		start = 0;
	}

	for (idx = start; idx < blob_len(b); ++idx)
	{
	    tv.v_type = VAR_NUMBER;
	    tv.vval.v_number = blob_get(b, idx);
	    if (tv_equal(&tv, &argvars[1], ic, FALSE))
	    {
		rettv->vval.v_number = idx;
		return;
	    }
	}
	return;
    }
    else if (argvars[0].v_type != VAR_LIST)
    {
	emsg(_(e_list_or_blob_required));
	return;
    }

    l = argvars[0].vval.v_list;
    if (l == NULL)
	return;

    CHECK_LIST_MATERIALIZE(l);
    item = l->lv_first;
    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	// Start at specified item.  Use the cached index that list_find()
	// sets, so that a negative number also works.
	item = list_find(l, (long)tv_get_number_chk(&argvars[2], &error));
	idx = l->lv_u.mat.lv_idx;
	if (argvars[3].v_type != VAR_UNKNOWN)
	    ic = (int)tv_get_bool_chk(&argvars[3], &error);
	if (error)
	    item = NULL;
    }

    for ( ; item != NULL; item = item->li_next, ++idx)
	if (tv_equal(&item->li_tv, &argvars[1], ic, FALSE))
	{
	    rettv->vval.v_number = idx;
	    break;
	}
}

/*
 * Evaluate 'expr' with the v:key and v:val arguments and return the result.
 * The expression is expected to return a boolean value.  The caller should set
 * the VV_KEY and VV_VAL vim variables before calling this function.
 */
    static int
indexof_eval_expr(typval_T *expr)
{
    typval_T	argv[3];
    typval_T	newtv;
    varnumber_T	found;
    int		error = FALSE;

    argv[0] = *get_vim_var_tv(VV_KEY);
    argv[1] = *get_vim_var_tv(VV_VAL);
    newtv.v_type = VAR_UNKNOWN;

    if (eval_expr_typval(expr, FALSE, argv, 2, NULL, &newtv) == FAIL)
	return FALSE;

    found = tv_get_bool_chk(&newtv, &error);
    clear_tv(&newtv);

    return error ? FALSE : found;
}

/*
 * Evaluate 'expr' for each byte in the Blob 'b' starting with the byte at
 * 'startidx' and return the index of the byte where 'expr' is TRUE.  Returns
 * -1 if 'expr' doesn't evaluate to TRUE for any of the bytes.
 */
    static int
indexof_blob(blob_T *b, long startidx, typval_T *expr)
{
    long	idx = 0;

    if (b == NULL)
	return -1;

    if (startidx < 0)
    {
	// negative index: index from the last byte
	startidx = blob_len(b) + startidx;
	if (startidx < 0)
	    startidx = 0;
    }

    set_vim_var_type(VV_KEY, VAR_NUMBER);
    set_vim_var_type(VV_VAL, VAR_NUMBER);

    for (idx = startidx; idx < blob_len(b); ++idx)
    {
	set_vim_var_nr(VV_KEY, idx);
	set_vim_var_nr(VV_VAL, blob_get(b, idx));

	if (indexof_eval_expr(expr))
	    return idx;
    }

    return -1;
}

/*
 * Evaluate 'expr' for each item in the List 'l' starting with the item at
 * 'startidx' and return the index of the item where 'expr' is TRUE.  Returns
 * -1 if 'expr' doesn't evaluate to TRUE for any of the items.
 */
    static int
indexof_list(list_T *l, long startidx, typval_T *expr)
{
    listitem_T	*item;
    long	idx = 0;
    int		found;

    if (l == NULL)
	return -1;

    CHECK_LIST_MATERIALIZE(l);

    if (startidx == 0)
	item = l->lv_first;
    else
    {
	// Start at specified item.  Use the cached index that list_find()
	// sets, so that a negative number also works.
	item = list_find(l, startidx);
	if (item != NULL)
	    idx = l->lv_u.mat.lv_idx;
    }

    set_vim_var_type(VV_KEY, VAR_NUMBER);

    for ( ; item != NULL; item = item->li_next, ++idx)
    {
	set_vim_var_nr(VV_KEY, idx);
	copy_tv(&item->li_tv, get_vim_var_tv(VV_VAL));

	found = indexof_eval_expr(expr);
	clear_tv(get_vim_var_tv(VV_VAL));

	if (found)
	    return idx;
    }

    return -1;
}

/*
 * "indexof()" function
 */
    static void
f_indexof(typval_T *argvars, typval_T *rettv)
{
    long	startidx = 0;
    typval_T	save_val;
    typval_T	save_key;
    int		save_did_emsg;

    rettv->vval.v_number = -1;

    if (check_for_list_or_blob_arg(argvars, 0) == FAIL
	    || check_for_string_or_func_arg(argvars, 1) == FAIL
	    || check_for_opt_dict_arg(argvars, 2) == FAIL)
	return;

    if ((argvars[1].v_type == VAR_STRING && argvars[1].vval.v_string == NULL)
	    || (argvars[1].v_type == VAR_FUNC
		&& argvars[1].vval.v_partial == NULL))
	return;

    if (argvars[2].v_type == VAR_DICT)
	startidx = dict_get_number_def(argvars[2].vval.v_dict, "startidx", 0);

    prepare_vimvar(VV_VAL, &save_val);
    prepare_vimvar(VV_KEY, &save_key);

    // We reset "did_emsg" to be able to detect whether an error occurred
    // during evaluation of the expression.
    save_did_emsg = did_emsg;
    did_emsg = FALSE;

    if (argvars[0].v_type == VAR_BLOB)
	rettv->vval.v_number = indexof_blob(argvars[0].vval.v_blob, startidx,
								&argvars[1]);
    else
	rettv->vval.v_number = indexof_list(argvars[0].vval.v_list, startidx,
								&argvars[1]);

    restore_vimvar(VV_KEY, &save_key);
    restore_vimvar(VV_VAL, &save_val);
    did_emsg |= save_did_emsg;
}

static int inputsecret_flag = 0;

/*
 * "input()" function
 *     Also handles inputsecret() when inputsecret is set.
 */
    static void
f_input(typval_T *argvars, typval_T *rettv)
{
    get_user_input(argvars, rettv, FALSE, inputsecret_flag);
}

/*
 * "inputdialog()" function
 */
    static void
f_inputdialog(typval_T *argvars, typval_T *rettv)
{
#if defined(FEAT_GUI_TEXTDIALOG)
    // Use a GUI dialog if the GUI is running and 'c' is not in 'guioptions'
    if (gui.in_use && vim_strchr(p_go, GO_CONDIALOG) == NULL)
    {
	char_u	*message;
	char_u	buf[NUMBUFLEN];
	char_u	*defstr = (char_u *)"";

	if (in_vim9script()
		&& (check_for_string_arg(argvars, 0) == FAIL
		    || check_for_opt_string_arg(argvars, 1) == FAIL
		    || (argvars[1].v_type != VAR_UNKNOWN
			&& check_for_opt_string_arg(argvars, 2) == FAIL)))
	    return;

	message = tv_get_string_chk(&argvars[0]);
	if (argvars[1].v_type != VAR_UNKNOWN
		&& (defstr = tv_get_string_buf_chk(&argvars[1], buf)) != NULL)
	    vim_strncpy(IObuff, defstr, IOSIZE - 1);
	else
	    IObuff[0] = NUL;
	if (message != NULL && defstr != NULL
		&& do_dialog(VIM_QUESTION, NULL, message,
			  (char_u *)_("&OK\n&Cancel"), 1, IObuff, FALSE) == 1)
	    rettv->vval.v_string = vim_strsave(IObuff);
	else
	{
	    if (message != NULL && defstr != NULL
					&& argvars[1].v_type != VAR_UNKNOWN
					&& argvars[2].v_type != VAR_UNKNOWN)
		rettv->vval.v_string = vim_strsave(
				      tv_get_string_buf(&argvars[2], buf));
	    else
		rettv->vval.v_string = NULL;
	}
	rettv->v_type = VAR_STRING;
    }
    else
#endif
	get_user_input(argvars, rettv, TRUE, inputsecret_flag);
}

/*
 * "inputlist()" function
 */
    static void
f_inputlist(typval_T *argvars, typval_T *rettv)
{
    list_T	*l;
    listitem_T	*li;
    int		selected;
    int		mouse_used;

#ifdef NO_CONSOLE_INPUT
    // While starting up, there is no place to enter text. When running tests
    // with --not-a-term we assume feedkeys() will be used.
    if (no_console_input() && !is_not_a_term())
	return;
#endif
    if (in_vim9script() && check_for_list_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_LIST || argvars[0].vval.v_list == NULL)
    {
	semsg(_(e_argument_of_str_must_be_list), "inputlist()");
	return;
    }

    msg_start();
    msg_row = Rows - 1;	// for when 'cmdheight' > 1
    lines_left = Rows;	// avoid more prompt
    msg_scroll = TRUE;
    msg_clr_eos();

    l = argvars[0].vval.v_list;
    CHECK_LIST_MATERIALIZE(l);
    FOR_ALL_LIST_ITEMS(l, li)
    {
	msg_puts((char *)tv_get_string(&li->li_tv));
	msg_putchar('\n');
    }

    // Ask for choice.
    selected = prompt_for_number(&mouse_used);
    if (mouse_used)
	selected -= lines_left;

    rettv->vval.v_number = selected;
}

static garray_T	    ga_userinput = {0, 0, sizeof(tasave_T), 4, NULL};

/*
 * "inputrestore()" function
 */
    static void
f_inputrestore(typval_T *argvars UNUSED, typval_T *rettv)
{
    if (ga_userinput.ga_len > 0)
    {
	--ga_userinput.ga_len;
	restore_typeahead((tasave_T *)(ga_userinput.ga_data)
						  + ga_userinput.ga_len, TRUE);
	// default return is zero == OK
    }
    else if (p_verbose > 1)
    {
	verb_msg(_("called inputrestore() more often than inputsave()"));
	rettv->vval.v_number = 1; // Failed
    }
}

/*
 * "inputsave()" function
 */
    static void
f_inputsave(typval_T *argvars UNUSED, typval_T *rettv)
{
    // Add an entry to the stack of typeahead storage.
    if (ga_grow(&ga_userinput, 1) == OK)
    {
	save_typeahead((tasave_T *)(ga_userinput.ga_data)
						       + ga_userinput.ga_len);
	++ga_userinput.ga_len;
	// default return is zero == OK
    }
    else
	rettv->vval.v_number = 1; // Failed
}

/*
 * "inputsecret()" function
 */
    static void
f_inputsecret(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL))
	return;

    ++cmdline_star;
    ++inputsecret_flag;
    f_input(argvars, rettv);
    --cmdline_star;
    --inputsecret_flag;
}

/*
 * "interrupt()" function
 */
    static void
f_interrupt(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
    got_int = TRUE;
}

/*
 * "invert(expr)" function
 */
    static void
f_invert(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_number = ~tv_get_number_chk(&argvars[0], NULL);
}

/*
 * Free resources in lval_root allocated by fill_exec_lval_root().
 */
    static void
free_lval_root(lval_root_T *root)
{
    if (root->lr_tv != NULL)
	free_tv(root->lr_tv);
    class_unref(root->lr_cl_exec);
    root->lr_tv = NULL;
    root->lr_cl_exec = NULL;
}

/*
 * This is used if executing in a method, the argument string is a
 * variable/item expr/reference. It may start with a potential class/object
 * variable.
 *
 * Adjust "root" as needed; lr_tv may be changed or freed.
 *
 * Always returns OK.
 * Free resources and return FAIL if the root should not be used. Otherwise OK.
 */

    static int
fix_variable_reference_lval_root(lval_root_T *root, char_u *name)
{

    // Check if lr_tv is the name of an object/class reference: name start with
    // "this" or name is class variable. Clear lr_tv if neither.
    int found_member = FALSE;
    if (root->lr_tv->v_type == VAR_OBJECT)
    {
	if (STRNCMP("this.", name, 5) == 0 ||STRCMP("this", name) == 0)
	    found_member = TRUE;
    }
    if (!found_member)	// not object member, try class member
    {
	// Explicitly check if the name is a class member.
	// If it's not then do nothing.
	char_u	*end;
	for (end = name; ASCII_ISALNUM(*end) || *end == '_'; ++end)
	    ;
	int idx = class_member_idx(root->lr_cl_exec, name, end - name);
	if (idx >= 0)
	{
	    // A class variable, replace lr_tv with it
	    clear_tv(root->lr_tv);
	    copy_tv(&root->lr_cl_exec->class_members_tv[idx], root->lr_tv);
	    found_member = TRUE;
	}
    }
    if (!found_member)
    {
	free_tv(root->lr_tv);
	root->lr_tv = NULL;	    // Not a member variable
    }
    // If FAIL, then must free_lval_root(root);
    return OK;
}

/*
 * "islocked()" function
 */
    static void
f_islocked(typval_T *argvars, typval_T *rettv)
{
    lval_T	lv;
    char_u	*end;
    dictitem_T	*di;

    rettv->vval.v_number = -1;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    char_u *name = tv_get_string(&argvars[0]);
#ifdef LOG_LOCKVAR
    ch_log(NULL, "LKVAR: f_islocked(): name: %s", name);
#endif

    lval_root_T	aroot;	// fully initialized in fill_exec_lval_root
    lval_root_T *root = NULL;

    // Set up lval_root if executing in a method.
    if (fill_exec_lval_root(&aroot) == OK)
    {
	// Almost always produces a valid lval_root since lr_cl_exec is used
	// for access verification, lr_tv may be set to NULL.
	if (fix_variable_reference_lval_root(&aroot, name) == OK)
	    root = &aroot;
    }

    lval_root_T	*lval_root_save = lval_root;
    lval_root = root;
    end = get_lval(name, NULL, &lv, FALSE, FALSE,
			     GLV_NO_AUTOLOAD | GLV_READ_ONLY | GLV_NO_DECL,
			     FNE_CHECK_START);
    lval_root = lval_root_save;

    if (end != NULL && lv.ll_name != NULL)
    {
	if (*end != NUL)
	{
	    semsg(_(lv.ll_name == lv.ll_name_end
		   ? e_invalid_argument_str : e_trailing_characters_str), end);
	}
	else
	{
	    if (lv.ll_tv == NULL)
	    {
		di = find_var(lv.ll_name, NULL, TRUE);
		if (di != NULL)
		{
		    // Consider a variable locked when:
		    // 1. the variable itself is locked
		    // 2. the value of the variable is locked.
		    // 3. the List or Dict value is locked.
		    rettv->vval.v_number = ((di->di_flags & DI_FLAGS_LOCK)
						   || tv_islocked(&di->di_tv));
		}
	    }
	    else if (lv.ll_is_root)
	    {
		rettv->vval.v_number = tv_islocked(lv.ll_tv);
	    }
	    else if (lv.ll_object != NULL)
	    {
		typval_T *tv = ((typval_T *)(lv.ll_object + 1)) + lv.ll_oi;
		rettv->vval.v_number = tv_islocked(tv);
#ifdef LOG_LOCKVAR
		ch_log(NULL, "LKVAR: f_islocked(): name %s (obj)", lv.ll_name);
#endif
	    }
	    else if (lv.ll_class != NULL)
	    {
		typval_T *tv = &lv.ll_class->class_members_tv[lv.ll_oi];
		rettv->vval.v_number = tv_islocked(tv);
#ifdef LOG_LOCKVAR
		ch_log(NULL, "LKVAR: f_islocked(): name %s (cl)", lv.ll_name);
#endif
	    }
	    else if (lv.ll_range)
		emsg(_(e_range_not_allowed));
	    else if (lv.ll_newkey != NULL)
		semsg(_(e_key_not_present_in_dictionary_str), lv.ll_newkey);
	    else if (lv.ll_list != NULL)
		// List item.
		rettv->vval.v_number = tv_islocked(&lv.ll_li->li_tv);
	    else
		// Dictionary item.
		rettv->vval.v_number = tv_islocked(&lv.ll_di->di_tv);
	}
    }

    if (root != NULL)
	free_lval_root(root);
    clear_lval(&lv);
}

/*
 * "keytrans()" function
 */
    static void
f_keytrans(typval_T *argvars, typval_T *rettv)
{
    char_u *escaped;

    rettv->v_type = VAR_STRING;
    if (check_for_string_arg(argvars, 0) == FAIL
	    || argvars[0].vval.v_string == NULL)
	return;
    // Need to escape K_SPECIAL and CSI for mb_unescape().
    escaped = vim_strsave_escape_csi(argvars[0].vval.v_string);
    rettv->vval.v_string = str2special_save(escaped, TRUE, TRUE);
    vim_free(escaped);
}

/*
 * "last_buffer_nr()" function.
 */
    static void
f_last_buffer_nr(typval_T *argvars UNUSED, typval_T *rettv)
{
    int		n = 0;
    buf_T	*buf;

    FOR_ALL_BUFFERS(buf)
	if (n < buf->b_fnum)
	    n = buf->b_fnum;

    rettv->vval.v_number = n;
}

/*
 * "len()" function
 */
    void
f_len(typval_T *argvars, typval_T *rettv)
{
    switch (argvars[0].v_type)
    {
	case VAR_STRING:
	case VAR_NUMBER:
	    rettv->vval.v_number = (varnumber_T)STRLEN(
					       tv_get_string(&argvars[0]));
	    break;
	case VAR_BLOB:
	    rettv->vval.v_number = blob_len(argvars[0].vval.v_blob);
	    break;
	case VAR_LIST:
	    rettv->vval.v_number = list_len(argvars[0].vval.v_list);
	    break;
	case VAR_DICT:
	    rettv->vval.v_number = dict_len(argvars[0].vval.v_dict);
	    break;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	case VAR_BOOL:
	case VAR_SPECIAL:
	case VAR_FLOAT:
	case VAR_FUNC:
	case VAR_PARTIAL:
	case VAR_JOB:
	case VAR_CHANNEL:
	case VAR_INSTR:
	case VAR_CLASS:
	case VAR_OBJECT:
	case VAR_TYPEALIAS:
	    emsg(_(e_invalid_type_for_len));
	    break;
    }
}

    static void
libcall_common(typval_T *argvars UNUSED, typval_T *rettv, int type)
{
#ifdef FEAT_LIBCALL
    char_u		*string_in;
    char_u		**string_result;
    int			nr_result;
#endif

    rettv->v_type = type;
    if (type != VAR_NUMBER)
	rettv->vval.v_string = NULL;

    if (check_restricted() || check_secure())
	return;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_string_or_number_arg(argvars, 2) == FAIL))
	return;

#ifdef FEAT_LIBCALL
    // The first two args must be strings, otherwise it's meaningless
    if (argvars[0].v_type == VAR_STRING && argvars[1].v_type == VAR_STRING)
    {
	string_in = NULL;
	if (argvars[2].v_type == VAR_STRING)
	    string_in = argvars[2].vval.v_string;
	if (type == VAR_NUMBER)
	{
	    string_result = NULL;
	}
	else
	{
	    rettv->vval.v_string = NULL;
	    string_result = &rettv->vval.v_string;
	}
	if (mch_libcall(argvars[0].vval.v_string,
			     argvars[1].vval.v_string,
			     string_in,
			     argvars[2].vval.v_number,
			     string_result,
			     &nr_result) == OK
		&& type == VAR_NUMBER)
	    rettv->vval.v_number = nr_result;
    }
#endif
}

/*
 * "libcall()" function
 */
    static void
f_libcall(typval_T *argvars, typval_T *rettv)
{
    libcall_common(argvars, rettv, VAR_STRING);
}

/*
 * "libcallnr()" function
 */
    static void
f_libcallnr(typval_T *argvars, typval_T *rettv)
{
    libcall_common(argvars, rettv, VAR_NUMBER);
}

/*
 * "line(string, [winid])" function
 */
    static void
f_line(typval_T *argvars, typval_T *rettv)
{
    linenr_T	lnum = 0;
    pos_T	*fp = NULL;
    int		fnum;
    int		id;
    tabpage_T	*tp;
    win_T	*wp;
    switchwin_T	switchwin;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL))
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	// use window specified in the second argument
	id = (int)tv_get_number(&argvars[1]);
	wp = win_id2wp_tp(id, &tp);
	if (wp != NULL && tp != NULL)
	{
	    if (switch_win_noblock(&switchwin, wp, tp, TRUE) == OK)
	    {
		check_cursor();
		fp = var2fpos(&argvars[0], TRUE, &fnum, FALSE);
	    }
	    restore_win_noblock(&switchwin, TRUE);
	}
    }
    else
	// use current window
	fp = var2fpos(&argvars[0], TRUE, &fnum, FALSE);

    if (fp != NULL)
	lnum = fp->lnum;
    rettv->vval.v_number = lnum;
}

/*
 * "line2byte(lnum)" function
 */
    static void
f_line2byte(typval_T *argvars UNUSED, typval_T *rettv)
{
#ifndef FEAT_BYTEOFF
    rettv->vval.v_number = -1;
#else
    linenr_T	lnum;

    if (in_vim9script() && check_for_lnum_arg(argvars, 0) == FAIL)
	return;

    lnum = tv_get_lnum(argvars);
    if (lnum < 1 || lnum > curbuf->b_ml.ml_line_count + 1)
	rettv->vval.v_number = -1;
    else
	rettv->vval.v_number = ml_find_line_or_offset(curbuf, lnum, NULL);
    if (rettv->vval.v_number >= 0)
	++rettv->vval.v_number;
#endif
}

#ifdef FEAT_LUA
/*
 * "luaeval()" function
 */
    static void
f_luaeval(typval_T *argvars, typval_T *rettv)
{
    char_u	*str;
    char_u	buf[NUMBUFLEN];

    if (check_restricted() || check_secure())
	return;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    str = tv_get_string_buf(&argvars[0], buf);
    do_luaeval(str, argvars + 1, rettv);
}
#endif

typedef enum
{
    MATCH_END,	    // matchend()
    MATCH_MATCH,    // match()
    MATCH_STR,	    // matchstr()
    MATCH_LIST,	    // matchlist()
    MATCH_POS	    // matchstrpos()
} matchtype_T;

    static void
find_some_match(typval_T *argvars, typval_T *rettv, matchtype_T type)
{
    char_u	*str = NULL;
    long	len = 0;
    char_u	*expr = NULL;
    char_u	*pat;
    regmatch_T	regmatch;
    char_u	patbuf[NUMBUFLEN];
    char_u	strbuf[NUMBUFLEN];
    char_u	*save_cpo;
    long	start = 0;
    long	nth = 1;
    colnr_T	startcol = 0;
    int		match = 0;
    list_T	*l = NULL;
    listitem_T	*li = NULL;
    long	idx = 0;
    char_u	*tofree = NULL;

    // Make 'cpoptions' empty, the 'l' flag should not be used here.
    save_cpo = p_cpo;
    p_cpo = empty_option;

    rettv->vval.v_number = -1;
    if (type == MATCH_LIST || type == MATCH_POS)
    {
	// type MATCH_LIST: return empty list when there are no matches.
	// type MATCH_POS: return ["", -1, -1, -1]
	if (rettv_list_alloc(rettv) == FAIL)
	    goto theend;
	if (type == MATCH_POS
		&& (list_append_string(rettv->vval.v_list,
					    (char_u *)"", 0) == FAIL
		    || list_append_number(rettv->vval.v_list,
					    (varnumber_T)-1) == FAIL
		    || list_append_number(rettv->vval.v_list,
					    (varnumber_T)-1) == FAIL
		    || list_append_number(rettv->vval.v_list,
					    (varnumber_T)-1) == FAIL))
	{
		list_free(rettv->vval.v_list);
		rettv->vval.v_list = NULL;
		goto theend;
	}
    }
    else if (type == MATCH_STR)
    {
	rettv->v_type = VAR_STRING;
	rettv->vval.v_string = NULL;
    }

    if (in_vim9script()
	    && (check_for_string_or_list_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_opt_number_arg(argvars, 2) == FAIL
		|| (argvars[2].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 3) == FAIL)))
	goto theend;

    if (argvars[0].v_type == VAR_LIST)
    {
	if ((l = argvars[0].vval.v_list) == NULL)
	    goto theend;
	CHECK_LIST_MATERIALIZE(l);
	li = l->lv_first;
    }
    else
    {
	expr = str = tv_get_string(&argvars[0]);
	len = (long)STRLEN(str);
    }

    pat = tv_get_string_buf_chk(&argvars[1], patbuf);
    if (pat == NULL)
	goto theend;

    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	int	    error = FALSE;

	start = (long)tv_get_number_chk(&argvars[2], &error);
	if (error)
	    goto theend;
	if (l != NULL)
	{
	    li = list_find(l, start);
	    if (li == NULL)
		goto theend;
	    idx = l->lv_u.mat.lv_idx;	// use the cached index
	}
	else
	{
	    if (start < 0)
		start = 0;
	    if (start > len)
		goto theend;
	    // When "count" argument is there ignore matches before "start",
	    // otherwise skip part of the string.  Differs when pattern is "^"
	    // or "\<".
	    if (argvars[3].v_type != VAR_UNKNOWN)
		startcol = start;
	    else
	    {
		str += start;
		len -= start;
	    }
	}

	if (argvars[3].v_type != VAR_UNKNOWN)
	    nth = (long)tv_get_number_chk(&argvars[3], &error);
	if (error)
	    goto theend;
    }

    regmatch.regprog = vim_regcomp(pat, RE_MAGIC + RE_STRING);
    if (regmatch.regprog != NULL)
    {
	regmatch.rm_ic = p_ic;

	for (;;)
	{
	    if (l != NULL)
	    {
		if (li == NULL)
		{
		    match = FALSE;
		    break;
		}
		vim_free(tofree);
		expr = str = echo_string(&li->li_tv, &tofree, strbuf, 0);
		if (str == NULL)
		    break;
	    }

	    match = vim_regexec_nl(&regmatch, str, startcol);

	    if (match && --nth <= 0)
		break;
	    if (l == NULL && !match)
		break;

	    // Advance to just after the match.
	    if (l != NULL)
	    {
		li = li->li_next;
		++idx;
	    }
	    else
	    {
		startcol = (colnr_T)(regmatch.startp[0]
				    + (*mb_ptr2len)(regmatch.startp[0]) - str);
		if (startcol > (colnr_T)len
				      || str + startcol <= regmatch.startp[0])
		{
		    match = FALSE;
		    break;
		}
	    }
	}

	if (match)
	{
	    if (type == MATCH_POS)
	    {
		listitem_T *li1 = rettv->vval.v_list->lv_first;
		listitem_T *li2 = li1->li_next;
		listitem_T *li3 = li2->li_next;
		listitem_T *li4 = li3->li_next;

		vim_free(li1->li_tv.vval.v_string);
		li1->li_tv.vval.v_string = vim_strnsave(regmatch.startp[0],
					regmatch.endp[0] - regmatch.startp[0]);
		li3->li_tv.vval.v_number =
				      (varnumber_T)(regmatch.startp[0] - expr);
		li4->li_tv.vval.v_number =
					(varnumber_T)(regmatch.endp[0] - expr);
		if (l != NULL)
		    li2->li_tv.vval.v_number = (varnumber_T)idx;
	    }
	    else if (type == MATCH_LIST)
	    {
		int i;

		// return list with matched string and submatches
		for (i = 0; i < NSUBEXP; ++i)
		{
		    if (regmatch.endp[i] == NULL)
		    {
			if (list_append_string(rettv->vval.v_list,
						     (char_u *)"", 0) == FAIL)
			    break;
		    }
		    else if (list_append_string(rettv->vval.v_list,
				regmatch.startp[i],
				(int)(regmatch.endp[i] - regmatch.startp[i]))
			    == FAIL)
			break;
		}
	    }
	    else if (type == MATCH_STR)
	    {
		// return matched string
		if (l != NULL)
		    copy_tv(&li->li_tv, rettv);
		else
		    rettv->vval.v_string = vim_strnsave(regmatch.startp[0],
					regmatch.endp[0] - regmatch.startp[0]);
	    }
	    else if (l != NULL)
		rettv->vval.v_number = idx;
	    else
	    {
		if (type != MATCH_END)
		    rettv->vval.v_number =
				      (varnumber_T)(regmatch.startp[0] - str);
		else
		    rettv->vval.v_number =
					(varnumber_T)(regmatch.endp[0] - str);
		rettv->vval.v_number += (varnumber_T)(str - expr);
	    }
	}
	vim_regfree(regmatch.regprog);
    }

theend:
    if (type == MATCH_POS && l == NULL && rettv->vval.v_list != NULL)
	// matchstrpos() without a list: drop the second item.
	listitem_remove(rettv->vval.v_list,
				       rettv->vval.v_list->lv_first->li_next);
    vim_free(tofree);
    p_cpo = save_cpo;
}

/*
 * Return all the matches in string "str" for pattern "rmp".
 * The matches are returned in the List "mlist".
 * If "submatches" is TRUE, then submatch information is also returned.
 * "matchbuf" is TRUE when called for matchbufline().
 */
    static int
get_matches_in_str(
    char_u	*str,
    regmatch_T	*rmp,
    list_T	*mlist,
    int		idx,
    int		submatches,
    int		matchbuf)
{
    long	len = (long)STRLEN(str);
    int		match = 0;
    colnr_T	startidx = 0;

    for (;;)
    {
	match = vim_regexec_nl(rmp, str, startidx);
	if (!match)
	    break;

	dict_T *d = dict_alloc();
	if (d == NULL)
	    return FAIL;
	if (list_append_dict(mlist, d) == FAIL)
	    return FAIL;;

	if (dict_add_number(d, matchbuf ? "lnum" : "idx", idx) == FAIL)
	    return FAIL;

	if (dict_add_number(d, "byteidx",
		    (colnr_T)(rmp->startp[0] - str)) == FAIL)
	    return FAIL;

	if (dict_add_string_len(d, "text", rmp->startp[0],
		    (int)(rmp->endp[0] - rmp->startp[0])) == FAIL)
	    return FAIL;

	if (submatches)
	{
	    list_T *sml = list_alloc();
	    if (sml == NULL)
		return FAIL;

	    if (dict_add_list(d, "submatches", sml) == FAIL)
		return FAIL;

	    // return a list with the submatches
	    for (int i = 1; i < NSUBEXP; ++i)
	    {
		if (rmp->endp[i] == NULL)
		{
		    if (list_append_string(sml, (char_u *)"", 0) == FAIL)
			return FAIL;
		}
		else if (list_append_string(sml, rmp->startp[i],
			    (int)(rmp->endp[i] - rmp->startp[i])) == FAIL)
		    return FAIL;
	    }
	}
	startidx = (colnr_T)(rmp->endp[0] - str);
	if (startidx >= (colnr_T)len || str + startidx <= rmp->startp[0])
	    break;
    }

    return OK;
}

/*
 * "matchbufline()" function
 */
    static void
f_matchbufline(typval_T *argvars, typval_T *rettv)
{
    list_T	*retlist = NULL;
    char_u	*save_cpo;
    char_u	patbuf[NUMBUFLEN];
    regmatch_T	regmatch;

    rettv->vval.v_number = -1;
    if (rettv_list_alloc(rettv) != OK)
	return;
    retlist = rettv->vval.v_list;

    if (check_for_buffer_arg(argvars, 0) == FAIL
	    || check_for_string_arg(argvars, 1) == FAIL
	    || check_for_lnum_arg(argvars, 2) == FAIL
	    || check_for_lnum_arg(argvars, 3) == FAIL
	    || check_for_opt_dict_arg(argvars, 4) == FAIL)
	return;

    int prev_did_emsg = did_emsg;
    buf_T *buf = tv_get_buf(&argvars[0], FALSE);
    if (buf == NULL)
    {
	if (did_emsg == prev_did_emsg)
	    semsg(_(e_invalid_buffer_name_str), tv_get_string(&argvars[0]));
	return;
    }
    if (buf->b_ml.ml_mfp == NULL)
    {
	emsg(_(e_buffer_is_not_loaded));
	return;
    }

    char_u *pat = tv_get_string_buf(&argvars[1], patbuf);

    int		did_emsg_before = did_emsg;
    linenr_T slnum = tv_get_lnum_buf(&argvars[2], buf);
    if (did_emsg > did_emsg_before)
	return;
    if (slnum < 1)
    {
	semsg(_(e_invalid_value_for_argument_str), "lnum");
	return;
    }

    linenr_T elnum = tv_get_lnum_buf(&argvars[3], buf);
    if (did_emsg > did_emsg_before)
	return;
    if (elnum < 1 || elnum < slnum)
    {
	semsg(_(e_invalid_value_for_argument_str), "end_lnum");
	return;
    }

    if (elnum > buf->b_ml.ml_line_count)
	elnum = buf->b_ml.ml_line_count;

    int		submatches = FALSE;
    if (argvars[4].v_type != VAR_UNKNOWN)
    {
	dict_T *d = argvars[4].vval.v_dict;
	if (d != NULL)
	{
	    dictitem_T *di = dict_find(d, (char_u *)"submatches", -1);
	    if (di != NULL)
	    {
		if (di->di_tv.v_type != VAR_BOOL)
		{
		    semsg(_(e_invalid_value_for_argument_str), "submatches");
		    return;
		}
		submatches = tv_get_bool(&di->di_tv);
	    }
	}
    }

    // Make 'cpoptions' empty, the 'l' flag should not be used here.
    save_cpo = p_cpo;
    p_cpo = empty_option;

    regmatch.regprog = vim_regcomp(pat, RE_MAGIC + RE_STRING);
    if (regmatch.regprog == NULL)
	goto theend;
    regmatch.rm_ic = p_ic;

    while (slnum <= elnum)
    {
	char_u *str = ml_get_buf(buf, slnum, FALSE);
	if (get_matches_in_str(str, &regmatch, retlist, slnum, submatches,
								TRUE) == FAIL)
	    goto cleanup;
	slnum++;
    }

cleanup:
    vim_regfree(regmatch.regprog);

theend:
    p_cpo = save_cpo;
}

/*
 * "match()" function
 */
    static void
f_match(typval_T *argvars, typval_T *rettv)
{
    find_some_match(argvars, rettv, MATCH_MATCH);
}

/*
 * "matchend()" function
 */
    static void
f_matchend(typval_T *argvars, typval_T *rettv)
{
    find_some_match(argvars, rettv, MATCH_END);
}

/*
 * "matchlist()" function
 */
    static void
f_matchlist(typval_T *argvars, typval_T *rettv)
{
    find_some_match(argvars, rettv, MATCH_LIST);
}

/*
 * "matchstr()" function
 */
    static void
f_matchstr(typval_T *argvars, typval_T *rettv)
{
    find_some_match(argvars, rettv, MATCH_STR);
}

/*
 * "matchstrlist()" function
 */
    static void
f_matchstrlist(typval_T *argvars, typval_T *rettv)
{
    list_T	*retlist = NULL;
    char_u	*save_cpo;
    list_T	*l = NULL;
    listitem_T	*li = NULL;
    char_u	patbuf[NUMBUFLEN];
    regmatch_T	regmatch;

    rettv->vval.v_number = -1;
    if (rettv_list_alloc(rettv) != OK)
	return;
    retlist = rettv->vval.v_list;

    if (check_for_list_arg(argvars, 0) == FAIL
	    || check_for_string_arg(argvars, 1) == FAIL
	    || check_for_opt_dict_arg(argvars, 2) == FAIL)
	return;

    if ((l = argvars[0].vval.v_list) == NULL)
	return;

    char_u *pat = tv_get_string_buf_chk(&argvars[1], patbuf);
    if (pat == NULL)
	return;

    // Make 'cpoptions' empty, the 'l' flag should not be used here.
    save_cpo = p_cpo;
    p_cpo = empty_option;

    regmatch.regprog = vim_regcomp(pat, RE_MAGIC + RE_STRING);
    if (regmatch.regprog == NULL)
	goto theend;
    regmatch.rm_ic = p_ic;

    int		submatches = FALSE;
    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	dict_T *d = argvars[2].vval.v_dict;
	if (d != NULL)
	{
	    dictitem_T *di = dict_find(d, (char_u *)"submatches", -1);
	    if (di != NULL)
	    {
		if (di->di_tv.v_type != VAR_BOOL)
		{
		    semsg(_(e_invalid_value_for_argument_str), "submatches");
		    goto cleanup;
		}
		submatches = tv_get_bool(&di->di_tv);
	    }
	}
    }

    int idx = 0;
    CHECK_LIST_MATERIALIZE(l);
    FOR_ALL_LIST_ITEMS(l, li)
    {
	if (li->li_tv.v_type == VAR_STRING && li->li_tv.vval.v_string != NULL)
	{
	    char_u *str = li->li_tv.vval.v_string;
	    if (get_matches_in_str(str, &regmatch, retlist, idx, submatches,
								FALSE) == FAIL)
		goto cleanup;
	}
	idx++;
    }

cleanup:
    vim_regfree(regmatch.regprog);

theend:
    p_cpo = save_cpo;
}

/*
 * "matchstrpos()" function
 */
    static void
f_matchstrpos(typval_T *argvars, typval_T *rettv)
{
    find_some_match(argvars, rettv, MATCH_POS);
}

    static void
max_min(typval_T *argvars, typval_T *rettv, int domax)
{
    varnumber_T	n = 0;
    varnumber_T	i;
    int		error = FALSE;

    if (in_vim9script() && check_for_list_or_dict_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_LIST)
    {
	list_T		*l;
	listitem_T	*li;

	l = argvars[0].vval.v_list;
	if (l != NULL && l->lv_len > 0)
	{
	    if (l->lv_first == &range_list_item)
	    {
		if ((l->lv_u.nonmat.lv_stride > 0) ^ domax)
		    n = l->lv_u.nonmat.lv_start;
		else
		    n = l->lv_u.nonmat.lv_start + ((varnumber_T)l->lv_len - 1)
						    * l->lv_u.nonmat.lv_stride;
	    }
	    else
	    {
		li = l->lv_first;
		if (li != NULL)
		{
		    n = tv_get_number_chk(&li->li_tv, &error);
		    if (error)
			return; // type error; errmsg already given
		    for (;;)
		    {
			li = li->li_next;
			if (li == NULL)
			    break;
			i = tv_get_number_chk(&li->li_tv, &error);
			if (error)
			    return; // type error; errmsg already given
			if (domax ? i > n : i < n)
			    n = i;
		    }
		}
	    }
	}
    }
    else if (argvars[0].v_type == VAR_DICT)
    {
	dict_T		*d;
	int		first = TRUE;
	hashitem_T	*hi;
	int		todo;

	d = argvars[0].vval.v_dict;
	if (d != NULL)
	{
	    todo = (int)d->dv_hashtab.ht_used;
	    FOR_ALL_HASHTAB_ITEMS(&d->dv_hashtab, hi, todo)
	    {
		if (!HASHITEM_EMPTY(hi))
		{
		    --todo;
		    i = tv_get_number_chk(&HI2DI(hi)->di_tv, &error);
		    if (error)
			return; // type error; errmsg already given
		    if (first)
		    {
			n = i;
			first = FALSE;
		    }
		    else if (domax ? i > n : i < n)
			n = i;
		}
	    }
	}
    }
    else
	semsg(_(e_argument_of_str_must_be_list_or_dictionary), domax ? "max()" : "min()");

    rettv->vval.v_number = n;
}

/*
 * "max()" function
 */
    static void
f_max(typval_T *argvars, typval_T *rettv)
{
    max_min(argvars, rettv, TRUE);
}

/*
 * "min()" function
 */
    static void
f_min(typval_T *argvars, typval_T *rettv)
{
    max_min(argvars, rettv, FALSE);
}

#if defined(FEAT_MZSCHEME) || defined(PROTO)
/*
 * "mzeval()" function
 */
    static void
f_mzeval(typval_T *argvars, typval_T *rettv)
{
    char_u	*str;
    char_u	buf[NUMBUFLEN];

    if (check_restricted() || check_secure())
	return;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    str = tv_get_string_buf(&argvars[0], buf);
    do_mzeval(str, rettv);
}

    void
mzscheme_call_vim(char_u *name, typval_T *args, typval_T *rettv)
{
    typval_T argvars[3];

    argvars[0].v_type = VAR_STRING;
    argvars[0].vval.v_string = name;
    copy_tv(args, &argvars[1]);
    argvars[2].v_type = VAR_UNKNOWN;
    f_call(argvars, rettv);
    clear_tv(&argvars[1]);
}
#endif

/*
 * "nextnonblank()" function
 */
    static void
f_nextnonblank(typval_T *argvars, typval_T *rettv)
{
    linenr_T	lnum;

    if (in_vim9script() && check_for_lnum_arg(argvars, 0) == FAIL)
	return;

    for (lnum = tv_get_lnum(argvars); ; ++lnum)
    {
	if (lnum < 0 || lnum > curbuf->b_ml.ml_line_count)
	{
	    lnum = 0;
	    break;
	}
	if (*skipwhite(ml_get(lnum)) != NUL)
	    break;
    }
    rettv->vval.v_number = lnum;
}

/*
 * "nr2char()" function
 */
    static void
f_nr2char(typval_T *argvars, typval_T *rettv)
{
    char_u	buf[NUMBUFLEN];

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL))
	return;

    if (has_mbyte)
    {
	int	utf8 = 0;

	if (argvars[1].v_type != VAR_UNKNOWN)
	    utf8 = (int)tv_get_bool_chk(&argvars[1], NULL);
	if (utf8)
	    buf[utf_char2bytes((int)tv_get_number(&argvars[0]), buf)] = NUL;
	else
	    buf[(*mb_char2bytes)((int)tv_get_number(&argvars[0]), buf)] = NUL;
    }
    else
    {
	buf[0] = (char_u)tv_get_number(&argvars[0]);
	buf[1] = NUL;
    }
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = vim_strsave(buf);
}

/*
 * "or(expr, expr)" function
 */
    static void
f_or(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    rettv->vval.v_number = tv_get_number_chk(&argvars[0], NULL)
					| tv_get_number_chk(&argvars[1], NULL);
}

#ifdef FEAT_PERL
/*
 * "perleval()" function
 */
    static void
f_perleval(typval_T *argvars, typval_T *rettv)
{
    char_u	*str;
    char_u	buf[NUMBUFLEN];

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    str = tv_get_string_buf(&argvars[0], buf);
    do_perleval(str, rettv);
}
#endif

/*
 * "prevnonblank()" function
 */
    static void
f_prevnonblank(typval_T *argvars, typval_T *rettv)
{
    linenr_T	lnum;

    if (in_vim9script() && check_for_lnum_arg(argvars, 0) == FAIL)
	return;

    lnum = tv_get_lnum(argvars);
    if (lnum < 1 || lnum > curbuf->b_ml.ml_line_count)
	lnum = 0;
    else
	while (lnum >= 1 && *skipwhite(ml_get(lnum)) == NUL)
	    --lnum;
    rettv->vval.v_number = lnum;
}

// This dummy va_list is here because:
// - passing a NULL pointer doesn't work when va_list isn't a pointer
// - locally in the function results in a "used before set" warning
// - using va_start() to initialize it gives "function with fixed args" error
static va_list	ap;

/*
 * "printf()" function
 */
    static void
f_printf(typval_T *argvars, typval_T *rettv)
{
    char_u	buf[NUMBUFLEN];
    int		len;
    char_u	*s;
    int		saved_did_emsg = did_emsg;
    char	*fmt;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script() && check_for_string_or_number_arg(argvars, 0) == FAIL)
	return;

    // Get the required length, allocate the buffer and do it for real.
    did_emsg = FALSE;
    fmt = (char *)tv_get_string_buf(&argvars[0], buf);
    len = vim_vsnprintf_typval(NULL, 0, fmt, ap, argvars + 1);
    if (!did_emsg)
    {
	s = alloc(len + 1);
	if (s != NULL)
	{
	    rettv->vval.v_string = s;
	    (void)vim_vsnprintf_typval((char *)s, len + 1, fmt,
							      ap, argvars + 1);
	}
    }
    did_emsg |= saved_did_emsg;
}

/*
 * "pum_getpos()" function
 */
    static void
f_pum_getpos(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
    if (rettv_dict_alloc(rettv) == FAIL)
	return;
    pum_set_event_info(rettv->vval.v_dict);
}

/*
 * "pumvisible()" function
 */
    static void
f_pumvisible(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
    if (pum_visible())
	rettv->vval.v_number = 1;
}

#ifdef FEAT_PYTHON3
/*
 * "py3eval()" function
 */
    static void
f_py3eval(typval_T *argvars, typval_T *rettv)
{
    char_u	*str;
    char_u	buf[NUMBUFLEN];

    if (check_restricted() || check_secure())
	return;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    if (p_pyx == 0)
	p_pyx = 3;

    str = tv_get_string_buf(&argvars[0], buf);
    do_py3eval(str, rettv);
}
#endif

#ifdef FEAT_PYTHON
/*
 * "pyeval()" function
 */
    static void
f_pyeval(typval_T *argvars, typval_T *rettv)
{
    char_u	*str;
    char_u	buf[NUMBUFLEN];

    if (check_restricted() || check_secure())
	return;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    if (p_pyx == 0)
	p_pyx = 2;

    str = tv_get_string_buf(&argvars[0], buf);
    do_pyeval(str, rettv);
}
#endif

#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3)
/*
 * "pyxeval()" function
 */
    static void
f_pyxeval(typval_T *argvars, typval_T *rettv)
{
    if (check_restricted() || check_secure())
	return;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

# if defined(FEAT_PYTHON) && defined(FEAT_PYTHON3)
    init_pyxversion();
    if (p_pyx == 2)
	f_pyeval(argvars, rettv);
    else
	f_py3eval(argvars, rettv);
# elif defined(FEAT_PYTHON)
    f_pyeval(argvars, rettv);
# elif defined(FEAT_PYTHON3)
    f_py3eval(argvars, rettv);
# endif
}
#endif

static UINT32_T srand_seed_for_testing = 0;
static int	srand_seed_for_testing_is_used = FALSE;

    static void
f_test_srand_seed(typval_T *argvars, typval_T *rettv UNUSED)
{
    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_UNKNOWN)
	srand_seed_for_testing_is_used = FALSE;
    else
    {
	srand_seed_for_testing = (UINT32_T)tv_get_number(&argvars[0]);
	srand_seed_for_testing_is_used = TRUE;
    }
}

    static void
init_srand(UINT32_T *x)
{
#ifndef MSWIN
    static int dev_urandom_state = NOTDONE;  // FAIL or OK once tried
#endif

    if (srand_seed_for_testing_is_used)
    {
	*x = srand_seed_for_testing;
	return;
    }
#ifndef MSWIN
    if (dev_urandom_state != FAIL)
    {
	int  fd = open("/dev/urandom", O_RDONLY);
	struct {
	    union {
		UINT32_T number;
		char     bytes[sizeof(UINT32_T)];
	    } contents;
	} buf;

	// Attempt reading /dev/urandom.
	if (fd == -1)
	    dev_urandom_state = FAIL;
	else
	{
	    buf.contents.number = 0;
	    if (read(fd, buf.contents.bytes, sizeof(UINT32_T))
							   != sizeof(UINT32_T))
		dev_urandom_state = FAIL;
	    else
	    {
		dev_urandom_state = OK;
		*x = buf.contents.number;
	    }
	    close(fd);
	}
    }
    if (dev_urandom_state != OK)
#endif
    {
	// Reading /dev/urandom doesn't work, fall back to:
	// - randombytes_random()
	// - reltime() or time()
	// - XOR with process ID
#if defined(FEAT_SODIUM)
	if (crypt_sodium_init() >= 0)
	    *x = crypt_sodium_randombytes_random();
	else
#endif
	{
#if defined(FEAT_RELTIME)
	    proftime_T res;
	    profile_start(&res);
#  if defined(MSWIN)
	    *x = (UINT32_T)res.LowPart;
#  else
	    *x = (UINT32_T)res.tv_fsec;
#  endif
#else
	    *x = vim_time();
#endif
	    *x ^= mch_get_pid();
	}
    }
}

#define ROTL(x, k) (((x) << (k)) | ((x) >> (32 - (k))))
#define SPLITMIX32(x, z) ( \
    (z) = ((x) += 0x9e3779b9), \
    (z) = ((z) ^ ((z) >> 16)) * 0x85ebca6b, \
    (z) = ((z) ^ ((z) >> 13)) * 0xc2b2ae35, \
    (z) ^ ((z) >> 16) \
    )
#define SHUFFLE_XOSHIRO128STARSTAR(x, y, z, w) \
    result = ROTL((y) * 5, 7) * 9; \
    t = (y) << 9; \
    (z) ^= (x); \
    (w) ^= (y); \
    (y) ^= (z), (x) ^= (w); \
    (z) ^= t; \
    (w) = ROTL(w, 11);

/*
 * "rand()" function
 */
    static void
f_rand(typval_T *argvars, typval_T *rettv)
{
    list_T	*l = NULL;
    static UINT32_T	gx, gy, gz, gw;
    static int	initialized = FALSE;
    listitem_T	*lx, *ly, *lz, *lw;
    UINT32_T	x = 0, y, z, w, t, result;

    if (in_vim9script() && check_for_opt_list_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_UNKNOWN)
    {
	// When no argument is given use the global seed list.
	if (initialized == FALSE)
	{
	    // Initialize the global seed list.
	    init_srand(&x);

	    gx = SPLITMIX32(x, z);
	    gy = SPLITMIX32(x, z);
	    gz = SPLITMIX32(x, z);
	    gw = SPLITMIX32(x, z);
	    initialized = TRUE;
	}

	SHUFFLE_XOSHIRO128STARSTAR(gx, gy, gz, gw);
    }
    else if (argvars[0].v_type == VAR_LIST)
    {
	l = argvars[0].vval.v_list;
	if (l == NULL || list_len(l) != 4)
	    goto theend;

	lx = list_find(l, 0L);
	ly = list_find(l, 1L);
	lz = list_find(l, 2L);
	lw = list_find(l, 3L);
	if (lx->li_tv.v_type != VAR_NUMBER) goto theend;
	if (ly->li_tv.v_type != VAR_NUMBER) goto theend;
	if (lz->li_tv.v_type != VAR_NUMBER) goto theend;
	if (lw->li_tv.v_type != VAR_NUMBER) goto theend;
	x = (UINT32_T)lx->li_tv.vval.v_number;
	y = (UINT32_T)ly->li_tv.vval.v_number;
	z = (UINT32_T)lz->li_tv.vval.v_number;
	w = (UINT32_T)lw->li_tv.vval.v_number;

	SHUFFLE_XOSHIRO128STARSTAR(x, y, z, w);

	lx->li_tv.vval.v_number = (varnumber_T)x;
	ly->li_tv.vval.v_number = (varnumber_T)y;
	lz->li_tv.vval.v_number = (varnumber_T)z;
	lw->li_tv.vval.v_number = (varnumber_T)w;
    }
    else
	goto theend;

    rettv->v_type = VAR_NUMBER;
    rettv->vval.v_number = (varnumber_T)result;
    return;

theend:
    semsg(_(e_invalid_argument_str), tv_get_string(&argvars[0]));
    rettv->v_type = VAR_NUMBER;
    rettv->vval.v_number = -1;
}

/*
 * "srand()" function
 */
    static void
f_srand(typval_T *argvars, typval_T *rettv)
{
    UINT32_T x = 0, z;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_UNKNOWN)
    {
	init_srand(&x);
    }
    else
    {
	int	    error = FALSE;

	x = (UINT32_T)tv_get_number_chk(&argvars[0], &error);
	if (error)
	    return;
    }

    list_append_number(rettv->vval.v_list, (varnumber_T)SPLITMIX32(x, z));
    list_append_number(rettv->vval.v_list, (varnumber_T)SPLITMIX32(x, z));
    list_append_number(rettv->vval.v_list, (varnumber_T)SPLITMIX32(x, z));
    list_append_number(rettv->vval.v_list, (varnumber_T)SPLITMIX32(x, z));
}

#undef ROTL
#undef SPLITMIX32
#undef SHUFFLE_XOSHIRO128STARSTAR

/*
 * "range()" function
 */
    static void
f_range(typval_T *argvars, typval_T *rettv)
{
    varnumber_T	start;
    varnumber_T	end;
    varnumber_T	stride = 1;
    int		error = FALSE;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 2) == FAIL)))
	return;

    start = tv_get_number_chk(&argvars[0], &error);
    if (argvars[1].v_type == VAR_UNKNOWN)
    {
	end = start - 1;
	start = 0;
    }
    else
    {
	end = tv_get_number_chk(&argvars[1], &error);
	if (argvars[2].v_type != VAR_UNKNOWN)
	    stride = tv_get_number_chk(&argvars[2], &error);
    }

    if (error)
	return;		// type error; errmsg already given
    if (stride == 0)
    {
	emsg(_(e_stride_is_zero));
	return;
    }
    if (stride > 0 ? end + 1 < start : end - 1 > start)
    {
	emsg(_(e_start_past_end));
	return;
    }

    list_T *list = rettv->vval.v_list;

    // Create a non-materialized list.  This is much more efficient and
    // works with ":for".  If used otherwise CHECK_LIST_MATERIALIZE() must
    // be called.
    list->lv_first = &range_list_item;
    list->lv_u.nonmat.lv_start = start;
    list->lv_u.nonmat.lv_end = end;
    list->lv_u.nonmat.lv_stride = stride;
    if (stride > 0 ? end < start : end > start)
	list->lv_len = 0;
    else
	list->lv_len = (end - start) / stride + 1;
}

/*
 * Materialize "list".
 * Do not call directly, use CHECK_LIST_MATERIALIZE()
 */
    void
range_list_materialize(list_T *list)
{
    varnumber_T start = list->lv_u.nonmat.lv_start;
    varnumber_T end = list->lv_u.nonmat.lv_end;
    int		stride = list->lv_u.nonmat.lv_stride;
    varnumber_T i;

    list->lv_first = NULL;
    list->lv_u.mat.lv_last = NULL;
    list->lv_len = 0;
    list->lv_u.mat.lv_idx_item = NULL;
    for (i = start; stride > 0 ? i <= end : i >= end; i += stride)
    {
	if (list_append_number(list, i) == FAIL)
	    break;
	if (list->lv_lock & VAR_ITEMS_LOCKED)
	    list->lv_u.mat.lv_last->li_tv.v_lock = VAR_LOCKED;
    }
    list->lv_lock &= ~VAR_ITEMS_LOCKED;
}

/*
 * "getreginfo()" function
 */
    static void
f_getreginfo(typval_T *argvars, typval_T *rettv)
{
    int		regname;
    char_u	buf[NUMBUFLEN + 2];
    long	reglen = 0;
    dict_T	*dict;
    list_T	*list;

    if (in_vim9script() && check_for_opt_string_arg(argvars, 0) == FAIL)
	return;

    regname = getreg_get_regname(argvars);
    if (regname == 0)
	return;

    if (regname == '@')
	regname = '"';

    if (rettv_dict_alloc(rettv) == FAIL)
	return;
    dict = rettv->vval.v_dict;

    list = (list_T *)get_reg_contents(regname, GREG_EXPR_SRC | GREG_LIST);
    if (list == NULL)
	return;
    (void)dict_add_list(dict, "regcontents", list);

    buf[0] = NUL;
    buf[1] = NUL;
    switch (get_reg_type(regname, &reglen))
    {
	case MLINE: buf[0] = 'V'; break;
	case MCHAR: buf[0] = 'v'; break;
	case MBLOCK:
		    vim_snprintf((char *)buf, sizeof(buf), "%c%ld", Ctrl_V,
			    reglen + 1);
		    break;
    }
    (void)dict_add_string(dict, (char *)"regtype", buf);

    buf[0] = get_register_name(get_unname_register());
    buf[1] = NUL;
    if (regname == '"')
	(void)dict_add_string(dict, (char *)"points_to", buf);
    else
    {
	dictitem_T	*item = dictitem_alloc((char_u *)"isunnamed");

	if (item != NULL)
	{
	    item->di_tv.v_type = VAR_BOOL;
	    item->di_tv.vval.v_number = regname == buf[0]
						      ? VVAL_TRUE : VVAL_FALSE;
	    (void)dict_add(dict, item);
	}
    }
}

    static void
return_register(int regname, typval_T *rettv)
{
    char_u buf[2] = {0, 0};

    buf[0] = (char_u)regname;
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = vim_strsave(buf);
}

/*
 * "reg_executing()" function
 */
    static void
f_reg_executing(typval_T *argvars UNUSED, typval_T *rettv)
{
    return_register(reg_executing, rettv);
}

/*
 * "reg_recording()" function
 */
    static void
f_reg_recording(typval_T *argvars UNUSED, typval_T *rettv)
{
    return_register(reg_recording, rettv);
}

/*
 * "rename({from}, {to})" function
 */
    static void
f_rename(typval_T *argvars, typval_T *rettv)
{
    char_u	buf[NUMBUFLEN];

    rettv->vval.v_number = -1;
    if (check_restricted() || check_secure())
	return;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL))
	return;

    rettv->vval.v_number = vim_rename(tv_get_string(&argvars[0]),
				      tv_get_string_buf(&argvars[1], buf));
}

/*
 * "repeat()" function
 */
    static void
f_repeat(typval_T *argvars, typval_T *rettv)
{
    char_u	*p;
    varnumber_T	n;
    int		slen;
    int		len;
    char_u	*r;
    int		i;

    if (in_vim9script()
	    && (check_for_string_or_number_or_list_or_blob_arg(argvars, 0)
		    == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    n = tv_get_number(&argvars[1]);
    if (argvars[0].v_type == VAR_LIST)
    {
	if (rettv_list_alloc(rettv) == OK && argvars[0].vval.v_list != NULL)
	    while (n-- > 0)
		if (list_extend(rettv->vval.v_list,
					argvars[0].vval.v_list, NULL) == FAIL)
		    break;
    }
    else if (argvars[0].v_type == VAR_BLOB)
    {
	if (rettv_blob_alloc(rettv) == FAIL
		|| argvars[0].vval.v_blob == NULL
		|| n <= 0)
	    return;

	slen = argvars[0].vval.v_blob->bv_ga.ga_len;
	len = (int)slen * n;
	if (len <= 0)
	    return;

	if (ga_grow(&rettv->vval.v_blob->bv_ga, len) == FAIL)
	    return;

	rettv->vval.v_blob->bv_ga.ga_len = len;

	for (i = 0; i < slen; ++i)
	    if (blob_get(argvars[0].vval.v_blob, i) != 0)
		break;

	if (i == slen)
	    // No need to copy since all bytes are already zero
	    return;

	for (i = 0; i < n; ++i)
	    blob_set_range(rettv->vval.v_blob,
		    (long)i * slen, ((long)i + 1) * slen - 1, argvars);
    }
    else
    {
	p = tv_get_string(&argvars[0]);
	rettv->v_type = VAR_STRING;
	rettv->vval.v_string = NULL;

	slen = (int)STRLEN(p);
	len = slen * n;
	if (len <= 0)
	    return;

	r = alloc(len + 1);
	if (r != NULL)
	{
	    for (i = 0; i < n; i++)
		mch_memmove(r + i * slen, p, (size_t)slen);
	    r[len] = NUL;
	}

	rettv->vval.v_string = r;
    }
}

#define SP_NOMOVE	0x01	    // don't move cursor
#define SP_REPEAT	0x02	    // repeat to find outer pair
#define SP_RETCOUNT	0x04	    // return matchcount
#define SP_SETPCMARK	0x08	    // set previous context mark
#define SP_START	0x10	    // accept match at start position
#define SP_SUBPAT	0x20	    // return nr of matching sub-pattern
#define SP_END		0x40	    // leave cursor at end of match
#define SP_COLUMN	0x80	    // start at cursor column

/*
 * Get flags for a search function.
 * Possibly sets "p_ws".
 * Returns BACKWARD, FORWARD or zero (for an error).
 */
    static int
get_search_arg(typval_T *varp, int *flagsp)
{
    int		dir = FORWARD;
    char_u	*flags;
    char_u	nbuf[NUMBUFLEN];
    int		mask;

    if (varp->v_type == VAR_UNKNOWN)
	return FORWARD;

    flags = tv_get_string_buf_chk(varp, nbuf);
    if (flags == NULL)
	return 0;		// type error; errmsg already given
    while (*flags != NUL)
    {
	switch (*flags)
	{
	    case 'b': dir = BACKWARD; break;
	    case 'w': p_ws = TRUE; break;
	    case 'W': p_ws = FALSE; break;
	    default:  mask = 0;
		      if (flagsp != NULL)
			  switch (*flags)
			  {
			      case 'c': mask = SP_START; break;
			      case 'e': mask = SP_END; break;
			      case 'm': mask = SP_RETCOUNT; break;
			      case 'n': mask = SP_NOMOVE; break;
			      case 'p': mask = SP_SUBPAT; break;
			      case 'r': mask = SP_REPEAT; break;
			      case 's': mask = SP_SETPCMARK; break;
			      case 'z': mask = SP_COLUMN; break;
			  }
		      if (mask == 0)
		      {
			  semsg(_(e_invalid_argument_str), flags);
			  dir = 0;
		      }
		      else
			  *flagsp |= mask;
	}
	if (dir == 0)
	    break;
	++flags;
    }
    return dir;
}

/*
 * Shared by search() and searchpos() functions.
 */
    static int
search_cmn(typval_T *argvars, pos_T *match_pos, int *flagsp)
{
    int		flags;
    char_u	*pat;
    pos_T	pos;
    pos_T	save_cursor;
    int		save_p_ws = p_ws;
    int		dir;
    int		retval = 0;	// default: FAIL
    long	lnum_stop = 0;
#ifdef FEAT_RELTIME
    long	time_limit = 0;
#endif
    int		options = SEARCH_KEEP;
    int		subpatnum;
    searchit_arg_T sia;
    int		use_skip = FALSE;
    pos_T	firstpos;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && (check_for_opt_number_arg(argvars, 2) == FAIL
			|| (argvars[2].v_type != VAR_UNKNOWN
			    && check_for_opt_number_arg(argvars, 3) == FAIL)))))
	goto theend;

    pat = tv_get_string(&argvars[0]);
    dir = get_search_arg(&argvars[1], flagsp);	// may set p_ws
    if (dir == 0)
	goto theend;
    flags = *flagsp;
    if (flags & SP_START)
	options |= SEARCH_START;
    if (flags & SP_END)
	options |= SEARCH_END;
    if (flags & SP_COLUMN)
	options |= SEARCH_COL;

    // Optional arguments: line number to stop searching, timeout and skip.
    if (argvars[1].v_type != VAR_UNKNOWN && argvars[2].v_type != VAR_UNKNOWN)
    {
	lnum_stop = (long)tv_get_number_chk(&argvars[2], NULL);
	if (lnum_stop < 0)
	    goto theend;
	if (argvars[3].v_type != VAR_UNKNOWN)
	{
#ifdef FEAT_RELTIME
	    time_limit = (long)tv_get_number_chk(&argvars[3], NULL);
	    if (time_limit < 0)
		goto theend;
#endif
	    use_skip = eval_expr_valid_arg(&argvars[4]);
	}
    }

    /*
     * This function does not accept SP_REPEAT and SP_RETCOUNT flags.
     * Check to make sure only those flags are set.
     * Also, Only the SP_NOMOVE or the SP_SETPCMARK flag can be set. Both
     * flags cannot be set. Check for that condition also.
     */
    if (((flags & (SP_REPEAT | SP_RETCOUNT)) != 0)
	    || ((flags & SP_NOMOVE) && (flags & SP_SETPCMARK)))
    {
	semsg(_(e_invalid_argument_str), tv_get_string(&argvars[1]));
	goto theend;
    }

    pos = save_cursor = curwin->w_cursor;
    CLEAR_FIELD(firstpos);
    CLEAR_FIELD(sia);
    sia.sa_stop_lnum = (linenr_T)lnum_stop;
#ifdef FEAT_RELTIME
    sia.sa_tm = time_limit;
#endif

    // Repeat until {skip} returns FALSE.
    for (;;)
    {
	subpatnum = searchit(curwin, curbuf, &pos, NULL, dir, pat, 1L,
						     options, RE_SEARCH, &sia);
	// finding the first match again means there is no match where {skip}
	// evaluates to zero.
	if (firstpos.lnum != 0 && EQUAL_POS(pos, firstpos))
	    subpatnum = FAIL;

	if (subpatnum == FAIL || !use_skip)
	    // didn't find it or no skip argument
	    break;
	if (firstpos.lnum == 0)
	    firstpos = pos;

	// If the skip expression matches, ignore this match.
	{
	    int	    do_skip;
	    int	    err;
	    pos_T   save_pos = curwin->w_cursor;

	    curwin->w_cursor = pos;
	    err = FALSE;
	    do_skip = eval_expr_to_bool(&argvars[4], &err);
	    curwin->w_cursor = save_pos;
	    if (err)
	    {
		// Evaluating {skip} caused an error, break here.
		subpatnum = FAIL;
		break;
	    }
	    if (!do_skip)
		break;
	}

	// clear the start flag to avoid getting stuck here
	options &= ~SEARCH_START;
    }

    if (subpatnum != FAIL)
    {
	if (flags & SP_SUBPAT)
	    retval = subpatnum;
	else
	    retval = pos.lnum;
	if (flags & SP_SETPCMARK)
	    setpcmark();
	curwin->w_cursor = pos;
	if (match_pos != NULL)
	{
	    // Store the match cursor position
	    match_pos->lnum = pos.lnum;
	    match_pos->col = pos.col + 1;
	}
	// "/$" will put the cursor after the end of the line, may need to
	// correct that here
	check_cursor();
    }

    // If 'n' flag is used: restore cursor position.
    if (flags & SP_NOMOVE)
	curwin->w_cursor = save_cursor;
    else
	curwin->w_set_curswant = TRUE;
theend:
    p_ws = save_p_ws;

    return retval;
}

#ifdef FEAT_RUBY
/*
 * "rubyeval()" function
 */
    static void
f_rubyeval(typval_T *argvars, typval_T *rettv)
{
    char_u	*str;
    char_u	buf[NUMBUFLEN];

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    str = tv_get_string_buf(&argvars[0], buf);
    do_rubyeval(str, rettv);
}
#endif

/*
 * "screenattr()" function
 */
    static void
f_screenattr(typval_T *argvars, typval_T *rettv)
{
    int		row;
    int		col;
    int		c;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    row = (int)tv_get_number_chk(&argvars[0], NULL) - 1;
    col = (int)tv_get_number_chk(&argvars[1], NULL) - 1;
    if (row < 0 || row >= screen_Rows
	    || col < 0 || col >= screen_Columns)
	c = -1;
    else
	c = ScreenAttrs[LineOffset[row] + col];
    rettv->vval.v_number = c;
}

/*
 * "screenchar()" function
 */
    static void
f_screenchar(typval_T *argvars, typval_T *rettv)
{
    int		row;
    int		col;
    int		c;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    row = (int)tv_get_number_chk(&argvars[0], NULL) - 1;
    col = (int)tv_get_number_chk(&argvars[1], NULL) - 1;
    if (row < 0 || row >= screen_Rows || col < 0 || col >= screen_Columns)
	c = -1;
    else
    {
	char_u buf[MB_MAXBYTES + 1];
	screen_getbytes(row, col, buf, NULL);
	c = (*mb_ptr2char)(buf);
    }
    rettv->vval.v_number = c;
}

/*
 * "screenchars()" function
 */
    static void
f_screenchars(typval_T *argvars, typval_T *rettv)
{
    int		row;
    int		col;
    int		c;
    int		i;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    row = (int)tv_get_number_chk(&argvars[0], NULL) - 1;
    col = (int)tv_get_number_chk(&argvars[1], NULL) - 1;
    if (row < 0 || row >= screen_Rows || col < 0 || col >= screen_Columns)
	return;

    char_u buf[MB_MAXBYTES + 1];
    screen_getbytes(row, col, buf, NULL);
    int pcc[MAX_MCO];
    if (enc_utf8)
	c = utfc_ptr2char(buf, pcc);
    else
	c = (*mb_ptr2char)(buf);
    list_append_number(rettv->vval.v_list, (varnumber_T)c);

    if (enc_utf8)
	for (i = 0; i < Screen_mco && pcc[i] != 0; ++i)
	    list_append_number(rettv->vval.v_list, (varnumber_T)pcc[i]);
}

/*
 * "screencol()" function
 *
 * First column is 1 to be consistent with virtcol().
 */
    static void
f_screencol(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->vval.v_number = screen_screencol() + 1;
}

/*
 * "screenrow()" function
 */
    static void
f_screenrow(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->vval.v_number = screen_screenrow() + 1;
}

/*
 * "screenstring()" function
 */
    static void
f_screenstring(typval_T *argvars, typval_T *rettv)
{
    int		row;
    int		col;
    char_u	buf[MB_MAXBYTES + 1];

    rettv->vval.v_string = NULL;
    rettv->v_type = VAR_STRING;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    row = (int)tv_get_number_chk(&argvars[0], NULL) - 1;
    col = (int)tv_get_number_chk(&argvars[1], NULL) - 1;
    if (row < 0 || row >= screen_Rows || col < 0 || col >= screen_Columns)
	return;

    screen_getbytes(row, col, buf, NULL);
    rettv->vval.v_string = vim_strsave(buf);
}

/*
 * "search()" function
 */
    static void
f_search(typval_T *argvars, typval_T *rettv)
{
    int		flags = 0;

    rettv->vval.v_number = search_cmn(argvars, NULL, &flags);
}

/*
 * "searchdecl()" function
 */
    static void
f_searchdecl(typval_T *argvars, typval_T *rettv)
{
    int		locally = TRUE;
    int		thisblock = FALSE;
    int		error = FALSE;
    char_u	*name;

    rettv->vval.v_number = 1;	// default: FAIL

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 2) == FAIL)))
	return;

    name = tv_get_string_chk(&argvars[0]);
    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	locally = !(int)tv_get_bool_chk(&argvars[1], &error);
	if (!error && argvars[2].v_type != VAR_UNKNOWN)
	    thisblock = (int)tv_get_bool_chk(&argvars[2], &error);
    }
    if (!error && name != NULL)
	rettv->vval.v_number = find_decl(name, (int)STRLEN(name),
				     locally, thisblock, SEARCH_KEEP) == FAIL;
}

/*
 * Used by searchpair() and searchpairpos()
 */
    static int
searchpair_cmn(typval_T *argvars, pos_T *match_pos)
{
    char_u	*spat, *mpat, *epat;
    typval_T	*skip;
    int		save_p_ws = p_ws;
    int		dir;
    int		flags = 0;
    char_u	nbuf1[NUMBUFLEN];
    char_u	nbuf2[NUMBUFLEN];
    int		retval = 0;		// default: FAIL
    long	lnum_stop = 0;
    long	time_limit = 0;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_string_arg(argvars, 2) == FAIL
		|| check_for_opt_string_arg(argvars, 3) == FAIL
		|| (argvars[3].v_type != VAR_UNKNOWN
		    && argvars[4].v_type != VAR_UNKNOWN
		    && (check_for_opt_number_arg(argvars, 5) == FAIL
			|| (argvars[5].v_type != VAR_UNKNOWN
			    && check_for_opt_number_arg(argvars, 6) == FAIL)))))
	goto theend;

    // Get the three pattern arguments: start, middle, end. Will result in an
    // error if not a valid argument.
    spat = tv_get_string_chk(&argvars[0]);
    mpat = tv_get_string_buf_chk(&argvars[1], nbuf1);
    epat = tv_get_string_buf_chk(&argvars[2], nbuf2);
    if (spat == NULL || mpat == NULL || epat == NULL)
	goto theend;	    // type error

    // Handle the optional fourth argument: flags
    dir = get_search_arg(&argvars[3], &flags); // may set p_ws
    if (dir == 0)
	goto theend;

    // Don't accept SP_END or SP_SUBPAT.
    // Only one of the SP_NOMOVE or SP_SETPCMARK flags can be set.
    if ((flags & (SP_END | SP_SUBPAT)) != 0
	    || ((flags & SP_NOMOVE) && (flags & SP_SETPCMARK)))
    {
	semsg(_(e_invalid_argument_str), tv_get_string(&argvars[3]));
	goto theend;
    }

    // Using 'r' implies 'W', otherwise it doesn't work.
    if (flags & SP_REPEAT)
	p_ws = FALSE;

    // Optional fifth argument: skip expression
    if (argvars[3].v_type == VAR_UNKNOWN
	    || argvars[4].v_type == VAR_UNKNOWN)
	skip = NULL;
    else
    {
	// Type is checked later.
	skip = &argvars[4];

	if (argvars[5].v_type != VAR_UNKNOWN)
	{
	    lnum_stop = (long)tv_get_number_chk(&argvars[5], NULL);
	    if (lnum_stop < 0)
	    {
		semsg(_(e_invalid_argument_str), tv_get_string(&argvars[5]));
		goto theend;
	    }
#ifdef FEAT_RELTIME
	    if (argvars[6].v_type != VAR_UNKNOWN)
	    {
		time_limit = (long)tv_get_number_chk(&argvars[6], NULL);
		if (time_limit < 0)
		{
		    semsg(_(e_invalid_argument_str), tv_get_string(&argvars[6]));
		    goto theend;
		}
	    }
#endif
	}
    }

    retval = do_searchpair(spat, mpat, epat, dir, skip, flags,
					    match_pos, lnum_stop, time_limit);

theend:
    p_ws = save_p_ws;

    return retval;
}

/*
 * "searchpair()" function
 */
    static void
f_searchpair(typval_T *argvars, typval_T *rettv)
{
    rettv->vval.v_number = searchpair_cmn(argvars, NULL);
}

/*
 * "searchpairpos()" function
 */
    static void
f_searchpairpos(typval_T *argvars, typval_T *rettv)
{
    pos_T	match_pos;
    int		lnum = 0;
    int		col = 0;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (searchpair_cmn(argvars, &match_pos) > 0)
    {
	lnum = match_pos.lnum;
	col = match_pos.col;
    }

    list_append_number(rettv->vval.v_list, (varnumber_T)lnum);
    list_append_number(rettv->vval.v_list, (varnumber_T)col);
}

/*
 * Search for a start/middle/end thing.
 * Used by searchpair(), see its documentation for the details.
 * Returns 0 or -1 for no match,
 */
    long
do_searchpair(
    char_u	*spat,	    // start pattern
    char_u	*mpat,	    // middle pattern
    char_u	*epat,	    // end pattern
    int		dir,	    // BACKWARD or FORWARD
    typval_T	*skip,	    // skip expression
    int		flags,	    // SP_SETPCMARK and other SP_ values
    pos_T	*match_pos,
    linenr_T	lnum_stop,  // stop at this line if not zero
    long	time_limit UNUSED) // stop after this many msec
{
    char_u	*save_cpo;
    char_u	*pat, *pat2 = NULL, *pat3 = NULL;
    long	retval = 0;
    pos_T	pos;
    pos_T	firstpos;
    pos_T	foundpos;
    pos_T	save_cursor;
    pos_T	save_pos;
    int		n;
    int		r;
    int		nest = 1;
    int		use_skip = FALSE;
    int		err;
    int		options = SEARCH_KEEP;

    // Make 'cpoptions' empty, the 'l' flag should not be used here.
    save_cpo = p_cpo;
    p_cpo = empty_option;

    // Make two search patterns: start/end (pat2, for in nested pairs) and
    // start/middle/end (pat3, for the top pair).
    pat2 = alloc(STRLEN(spat) + STRLEN(epat) + 17);
    pat3 = alloc(STRLEN(spat) + STRLEN(mpat) + STRLEN(epat) + 25);
    if (pat2 == NULL || pat3 == NULL)
	goto theend;
    sprintf((char *)pat2, "\\m\\(%s\\m\\)\\|\\(%s\\m\\)", spat, epat);
    if (*mpat == NUL)
	STRCPY(pat3, pat2);
    else
	sprintf((char *)pat3, "\\m\\(%s\\m\\)\\|\\(%s\\m\\)\\|\\(%s\\m\\)",
							    spat, epat, mpat);
    if (flags & SP_START)
	options |= SEARCH_START;

    if (skip != NULL)
	use_skip = eval_expr_valid_arg(skip);

#ifdef FEAT_RELTIME
    if (time_limit > 0)
	init_regexp_timeout(time_limit);
#endif
    save_cursor = curwin->w_cursor;
    pos = curwin->w_cursor;
    CLEAR_POS(&firstpos);
    CLEAR_POS(&foundpos);
    pat = pat3;
    for (;;)
    {
	searchit_arg_T sia;

	CLEAR_FIELD(sia);
	sia.sa_stop_lnum = lnum_stop;
	n = searchit(curwin, curbuf, &pos, NULL, dir, pat, 1L,
						     options, RE_SEARCH, &sia);
	if (n == FAIL || (firstpos.lnum != 0 && EQUAL_POS(pos, firstpos)))
	    // didn't find it or found the first match again: FAIL
	    break;

	if (firstpos.lnum == 0)
	    firstpos = pos;
	if (EQUAL_POS(pos, foundpos))
	{
	    // Found the same position again.  Can happen with a pattern that
	    // has "\zs" at the end and searching backwards.  Advance one
	    // character and try again.
	    if (dir == BACKWARD)
		decl(&pos);
	    else
		incl(&pos);
	}
	foundpos = pos;

	// clear the start flag to avoid getting stuck here
	options &= ~SEARCH_START;

	// If the skip pattern matches, ignore this match.
	if (use_skip)
	{
	    save_pos = curwin->w_cursor;
	    curwin->w_cursor = pos;
	    err = FALSE;
	    r = eval_expr_to_bool(skip, &err);
	    curwin->w_cursor = save_pos;
	    if (err)
	    {
		// Evaluating {skip} caused an error, break here.
		curwin->w_cursor = save_cursor;
		retval = -1;
		break;
	    }
	    if (r)
		continue;
	}

	if ((dir == BACKWARD && n == 3) || (dir == FORWARD && n == 2))
	{
	    // Found end when searching backwards or start when searching
	    // forward: nested pair.
	    ++nest;
	    pat = pat2;		// nested, don't search for middle
	}
	else
	{
	    // Found end when searching forward or start when searching
	    // backward: end of (nested) pair; or found middle in outer pair.
	    if (--nest == 1)
		pat = pat3;	// outer level, search for middle
	}

	if (nest == 0)
	{
	    // Found the match: return matchcount or line number.
	    if (flags & SP_RETCOUNT)
		++retval;
	    else
		retval = pos.lnum;
	    if (flags & SP_SETPCMARK)
		setpcmark();
	    curwin->w_cursor = pos;
	    if (!(flags & SP_REPEAT))
		break;
	    nest = 1;	    // search for next unmatched
	}
    }

    if (match_pos != NULL)
    {
	// Store the match cursor position
	match_pos->lnum = curwin->w_cursor.lnum;
	match_pos->col = curwin->w_cursor.col + 1;
    }

    // If 'n' flag is used or search failed: restore cursor position.
    if ((flags & SP_NOMOVE) || retval == 0)
	curwin->w_cursor = save_cursor;

theend:
#ifdef FEAT_RELTIME
    if (time_limit > 0)
	disable_regexp_timeout();
#endif
    vim_free(pat2);
    vim_free(pat3);
    if (p_cpo == empty_option)
	p_cpo = save_cpo;
    else
    {
	// Darn, evaluating the {skip} expression changed the value.
	// If it's still empty it was changed and restored, need to restore in
	// the complicated way.
	if (*p_cpo == NUL)
	    set_option_value_give_err((char_u *)"cpo", 0L, save_cpo, 0);
	free_string_option(save_cpo);
    }

    return retval;
}

/*
 * "searchpos()" function
 */
    static void
f_searchpos(typval_T *argvars, typval_T *rettv)
{
    pos_T	match_pos;
    int		lnum = 0;
    int		col = 0;
    int		n;
    int		flags = 0;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    n = search_cmn(argvars, &match_pos, &flags);
    if (n > 0)
    {
	lnum = match_pos.lnum;
	col = match_pos.col;
    }

    list_append_number(rettv->vval.v_list, (varnumber_T)lnum);
    list_append_number(rettv->vval.v_list, (varnumber_T)col);
    if (flags & SP_SUBPAT)
	list_append_number(rettv->vval.v_list, (varnumber_T)n);
}

/*
 * Set the cursor or mark position.
 * If "charpos" is TRUE, then use the column number as a character offset.
 * Otherwise use the column number as a byte offset.
 */
    static void
set_position(typval_T *argvars, typval_T *rettv, int charpos)
{
    pos_T	pos;
    int		fnum;
    char_u	*name;
    colnr_T	curswant = -1;

    rettv->vval.v_number = -1;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_list_arg(argvars, 1) == FAIL))
	return;

    name = tv_get_string_chk(argvars);
    if (name == NULL)
	return;

    if (list2fpos(&argvars[1], &pos, &fnum, &curswant, charpos) != OK)
	return;

    if (pos.col != MAXCOL && --pos.col < 0)
	pos.col = 0;
    if ((name[0] == '.' && name[1] == NUL))
    {
	// set cursor; "fnum" is ignored
	curwin->w_cursor = pos;
	if (curswant >= 0)
	{
	    curwin->w_curswant = curswant - 1;
	    curwin->w_set_curswant = FALSE;
	}
	check_cursor();
	rettv->vval.v_number = 0;
    }
    else if (name[0] == '\'' && name[1] != NUL && name[2] == NUL)
    {
	// set mark
	if (setmark_pos(name[1], &pos, fnum) == OK)
	    rettv->vval.v_number = 0;
    }
    else
	emsg(_(e_invalid_argument));
}
/*
 * "setcharpos()" function
 */
    static void
f_setcharpos(typval_T *argvars, typval_T *rettv)
{
    set_position(argvars, rettv, TRUE);
}

    static void
f_setcharsearch(typval_T *argvars, typval_T *rettv UNUSED)
{
    dict_T	*d;
    dictitem_T	*di;
    char_u	*csearch;

    if (check_for_dict_arg(argvars, 0) == FAIL)
	return;

    if ((d = argvars[0].vval.v_dict) == NULL)
	return;

    csearch = dict_get_string(d, "char", FALSE);
    if (csearch != NULL)
    {
	if (enc_utf8)
	{
	    int pcc[MAX_MCO];
	    int c = utfc_ptr2char(csearch, pcc);

	    set_last_csearch(c, csearch, utfc_ptr2len(csearch));
	}
	else
	    set_last_csearch(PTR2CHAR(csearch),
		    csearch, mb_ptr2len(csearch));
    }

    di = dict_find(d, (char_u *)"forward", -1);
    if (di != NULL)
	set_csearch_direction((int)tv_get_number(&di->di_tv)
		? FORWARD : BACKWARD);

    di = dict_find(d, (char_u *)"until", -1);
    if (di != NULL)
	set_csearch_until(!!tv_get_number(&di->di_tv));
}

/*
 * "setcursorcharpos" function
 */
    static void
f_setcursorcharpos(typval_T *argvars, typval_T *rettv)
{
    set_cursorpos(argvars, rettv, TRUE);
}

/*
 * "setenv()" function
 */
    static void
f_setenv(typval_T *argvars, typval_T *rettv UNUSED)
{
    char_u   namebuf[NUMBUFLEN];
    char_u   valbuf[NUMBUFLEN];
    char_u  *name;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    // setting an environment variable may be dangerous, e.g. you could
    // setenv GCONV_PATH=/tmp and then have iconv() unexpectedly call
    // a shell command using some shared library:
    if (check_restricted() || check_secure())
	return;

    name = tv_get_string_buf(&argvars[0], namebuf);
    if (argvars[1].v_type == VAR_SPECIAL
				      && argvars[1].vval.v_number == VVAL_NULL)
	vim_unsetenv_ext(name);
    else
	vim_setenv_ext(name, tv_get_string_buf(&argvars[1], valbuf));
}

/*
 * "setfperm({fname}, {mode})" function
 */
    static void
f_setfperm(typval_T *argvars, typval_T *rettv)
{
    char_u	*fname;
    char_u	modebuf[NUMBUFLEN];
    char_u	*mode_str;
    int		i;
    int		mask;
    int		mode = 0;

    rettv->vval.v_number = 0;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL))
	return;

    fname = tv_get_string_chk(&argvars[0]);
    if (fname == NULL)
	return;
    mode_str = tv_get_string_buf_chk(&argvars[1], modebuf);
    if (mode_str == NULL)
	return;
    if (STRLEN(mode_str) != 9)
    {
	semsg(_(e_invalid_argument_str), mode_str);
	return;
    }

    mask = 1;
    for (i = 8; i >= 0; --i)
    {
	if (mode_str[i] != '-')
	    mode |= mask;
	mask = mask << 1;
    }
    rettv->vval.v_number = mch_setperm(fname, mode) == OK;
}

/*
 * "setpos()" function
 */
    static void
f_setpos(typval_T *argvars, typval_T *rettv)
{
    set_position(argvars, rettv, FALSE);
}

/*
 * Translate a register type string to the yank type and block length
 */
    static int
get_yank_type(char_u **pp, char_u *yank_type, long *block_len)
{
    char_u *stropt = *pp;
    switch (*stropt)
    {
	case 'v': case 'c':	// character-wise selection
	    *yank_type = MCHAR;
	    break;
	case 'V': case 'l':	// line-wise selection
	    *yank_type = MLINE;
	    break;
	case 'b': case Ctrl_V:	// block-wise selection
	    *yank_type = MBLOCK;
	    if (VIM_ISDIGIT(stropt[1]))
	    {
		++stropt;
		*block_len = getdigits(&stropt) - 1;
		--stropt;
	    }
	    break;
	default:
	    return FAIL;
    }
    *pp = stropt;
    return OK;
}

/*
 * "setreg()" function
 */
    static void
f_setreg(typval_T *argvars, typval_T *rettv)
{
    int		regname;
    char_u	*strregname;
    char_u	*stropt;
    char_u	*strval;
    int		append;
    char_u	yank_type;
    long	block_len;
    typval_T	*regcontents;
    int		pointreg;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 2) == FAIL))
	return;

    pointreg = 0;
    regcontents = NULL;
    block_len = -1;
    yank_type = MAUTO;
    append = FALSE;

    strregname = tv_get_string_chk(argvars);
    rettv->vval.v_number = 1;		// FAIL is default

    if (strregname == NULL)
	return;		// type error; errmsg already given
    if (in_vim9script() && STRLEN(strregname) > 1)
    {
	semsg(_(e_register_name_must_be_one_char_str), strregname);
	return;
    }
    regname = *strregname;
    if (regname == 0 || regname == '@')
	regname = '"';

    if (argvars[1].v_type == VAR_DICT)
    {
	dict_T	    *d = argvars[1].vval.v_dict;
	dictitem_T  *di;

	if (d == NULL || d->dv_hashtab.ht_used == 0)
	{
	    // Empty dict, clear the register (like setreg(0, []))
	    char_u *lstval[2] = {NULL, NULL};
	    write_reg_contents_lst(regname, lstval, 0, FALSE, MAUTO, -1);
	    return;
	}

	di = dict_find(d, (char_u *)"regcontents", -1);
	if (di != NULL)
	    regcontents = &di->di_tv;

	stropt = dict_get_string(d, "regtype", FALSE);
	if (stropt != NULL)
	{
	    int ret = get_yank_type(&stropt, &yank_type, &block_len);

	    if (ret == FAIL || *++stropt != NUL)
	    {
		semsg(_(e_invalid_value_for_argument_str), "value");
		return;
	    }
	}

	if (regname == '"')
	{
	    stropt = dict_get_string(d, "points_to", FALSE);
	    if (stropt != NULL)
	    {
		pointreg = *stropt;
		regname = pointreg;
	    }
	}
	else if (dict_get_bool(d, "isunnamed", -1) > 0)
	    pointreg = regname;
    }
    else
	regcontents = &argvars[1];

    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	if (yank_type != MAUTO)
	{
	    semsg(_(e_too_many_arguments_for_function_str), "setreg");
	    return;
	}

	stropt = tv_get_string_chk(&argvars[2]);
	if (stropt == NULL)
	    return;		// type error
	for (; *stropt != NUL; ++stropt)
	    switch (*stropt)
	    {
		case 'a': case 'A':	// append
		    append = TRUE;
		    break;
		default:
		    get_yank_type(&stropt, &yank_type, &block_len);
	    }
    }

    if (regcontents && regcontents->v_type == VAR_LIST)
    {
	char_u		**lstval;
	char_u		**allocval;
	char_u		buf[NUMBUFLEN];
	char_u		**curval;
	char_u		**curallocval;
	list_T		*ll = regcontents->vval.v_list;
	listitem_T	*li;
	int		len;

	// If the list is NULL handle like an empty list.
	len = ll == NULL ? 0 : ll->lv_len;

	// First half: use for pointers to result lines; second half: use for
	// pointers to allocated copies.
	lstval = ALLOC_MULT(char_u *, (len + 1) * 2);
	if (lstval == NULL)
	    return;
	curval = lstval;
	allocval = lstval + len + 2;
	curallocval = allocval;

	if (ll != NULL)
	{
	    CHECK_LIST_MATERIALIZE(ll);
	    FOR_ALL_LIST_ITEMS(ll, li)
	    {
		strval = tv_get_string_buf_chk(&li->li_tv, buf);
		if (strval == NULL)
		    goto free_lstval;
		if (strval == buf)
		{
		    // Need to make a copy, next tv_get_string_buf_chk() will
		    // overwrite the string.
		    strval = vim_strsave(buf);
		    if (strval == NULL)
			goto free_lstval;
		    *curallocval++ = strval;
		}
		*curval++ = strval;
	    }
	}
	*curval++ = NULL;

	write_reg_contents_lst(regname, lstval, -1,
						append, yank_type, block_len);
free_lstval:
	while (curallocval > allocval)
	    vim_free(*--curallocval);
	vim_free(lstval);
    }
    else if (regcontents)
    {
	strval = tv_get_string_chk(regcontents);
	if (strval == NULL)
	    return;
	write_reg_contents_ex(regname, strval, -1,
						append, yank_type, block_len);
    }
    if (pointreg != 0)
	get_yank_register(pointreg, TRUE);

    rettv->vval.v_number = 0;
}

/*
 * "settagstack()" function
 */
    static void
f_settagstack(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp;
    dict_T	*d;
    int		action = 'r';

    rettv->vval.v_number = -1;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_dict_arg(argvars, 1) == FAIL
		|| check_for_opt_string_arg(argvars, 2) == FAIL))
	return;

    // first argument: window number or id
    wp = find_win_by_nr_or_id(&argvars[0]);
    if (wp == NULL)
	return;

    // second argument: dict with items to set in the tag stack
    if (check_for_dict_arg(argvars, 1) == FAIL)
	return;
    d = argvars[1].vval.v_dict;
    if (d == NULL)
	return;

    // third argument: action - 'a' for append and 'r' for replace.
    // default is to replace the stack.
    if (argvars[2].v_type == VAR_UNKNOWN)
	action = 'r';
    else if (check_for_string_arg(argvars, 2) == FAIL)
	return;
    else
    {
	char_u	*actstr;
	actstr = tv_get_string_chk(&argvars[2]);
	if (actstr == NULL)
	    return;
	if ((*actstr == 'r' || *actstr == 'a' || *actstr == 't')
		&& actstr[1] == NUL)
	    action = *actstr;
	else
	{
	    semsg(_(e_invalid_action_str_2), actstr);
	    return;
	}
    }

    if (set_tagstack(wp, d, action) == OK)
	rettv->vval.v_number = 0;
}

#ifdef FEAT_CRYPT
/*
 * "sha256({string})" function
 */
    static void
f_sha256(typval_T *argvars, typval_T *rettv)
{
    char_u	*p;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    p = tv_get_string(&argvars[0]);
    rettv->vval.v_string = vim_strsave(
				    sha256_bytes(p, (int)STRLEN(p), NULL, 0));
    rettv->v_type = VAR_STRING;
}
#endif // FEAT_CRYPT

/*
 * "shellescape({string})" function
 */
    static void
f_shellescape(typval_T *argvars, typval_T *rettv)
{
    int do_special;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL))
	return;

    do_special = non_zero_arg(&argvars[1]);
    rettv->vval.v_string = vim_strsave_shellescape(
			   tv_get_string(&argvars[0]), do_special, do_special);
    rettv->v_type = VAR_STRING;
}

/*
 * shiftwidth() function
 */
    static void
f_shiftwidth(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->vval.v_number = 0;

    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	long	col;

	col = (long)tv_get_number_chk(argvars, NULL);
	if (col < 0)
	    return;	// type error; errmsg already given
#ifdef FEAT_VARTABS
	rettv->vval.v_number = get_sw_value_col(curbuf, col);
	return;
#endif
    }

    rettv->vval.v_number = get_sw_value(curbuf);
}

/*
 * "soundfold({word})" function
 */
    static void
f_soundfold(typval_T *argvars, typval_T *rettv)
{
    char_u	*s;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_STRING;
    s = tv_get_string(&argvars[0]);
#ifdef FEAT_SPELL
    rettv->vval.v_string = eval_soundfold(s);
#else
    rettv->vval.v_string = vim_strsave(s);
#endif
}

/*
 * "spellbadword()" function
 */
    static void
f_spellbadword(typval_T *argvars UNUSED, typval_T *rettv)
{
    char_u	*word = (char_u *)"";
    hlf_T	attr = HLF_COUNT;
    int		len = 0;
#ifdef FEAT_SPELL
    int		wo_spell_save = curwin->w_p_spell;

    if (in_vim9script() && check_for_opt_string_arg(argvars, 0) == FAIL)
	return;

    if (!curwin->w_p_spell)
    {
	parse_spelllang(curwin);
	curwin->w_p_spell = TRUE;
    }

    if (*curwin->w_s->b_p_spl == NUL)
    {
	emsg(_(e_spell_checking_is_not_possible));
	curwin->w_p_spell = wo_spell_save;
	return;
    }
#endif

    if (rettv_list_alloc(rettv) == FAIL)
    {
#ifdef FEAT_SPELL
	curwin->w_p_spell = wo_spell_save;
#endif
	return;
    }

#ifdef FEAT_SPELL
    if (argvars[0].v_type == VAR_UNKNOWN)
    {
	// Find the start and length of the badly spelled word.
	len = spell_move_to(curwin, FORWARD, TRUE, TRUE, &attr);
	if (len != 0)
	{
	    word = ml_get_cursor();
	    curwin->w_set_curswant = TRUE;
	}
    }
    else if (*curbuf->b_s.b_p_spl != NUL)
    {
	char_u	*str = tv_get_string_chk(&argvars[0]);
	int	capcol = -1;

	if (str != NULL)
	{
	    // Check the argument for spelling.
	    while (*str != NUL)
	    {
		len = spell_check(curwin, str, &attr, &capcol, FALSE);
		if (attr != HLF_COUNT)
		{
		    word = str;
		    break;
		}
		str += len;
		capcol -= len;
		len = 0;
	    }
	}
    }
    curwin->w_p_spell = wo_spell_save;
#endif

    list_append_string(rettv->vval.v_list, word, len);
    list_append_string(rettv->vval.v_list, (char_u *)(
			attr == HLF_SPB ? "bad" :
			attr == HLF_SPR ? "rare" :
			attr == HLF_SPL ? "local" :
			attr == HLF_SPC ? "caps" :
			""), -1);
}

/*
 * "spellsuggest()" function
 */
    static void
f_spellsuggest(typval_T *argvars UNUSED, typval_T *rettv)
{
#ifdef FEAT_SPELL
    char_u	*str;
    int		typeerr = FALSE;
    int		maxcount;
    garray_T	ga;
    int		i;
    listitem_T	*li;
    int		need_capital = FALSE;
    int		wo_spell_save = curwin->w_p_spell;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_number_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 2) == FAIL)))
	return;

    if (!curwin->w_p_spell)
    {
	parse_spelllang(curwin);
	curwin->w_p_spell = TRUE;
    }

    if (*curwin->w_s->b_p_spl == NUL)
    {
	emsg(_(e_spell_checking_is_not_possible));
	curwin->w_p_spell = wo_spell_save;
	return;
    }
#endif

    if (rettv_list_alloc(rettv) == FAIL)
    {
#ifdef FEAT_SPELL
	curwin->w_p_spell = wo_spell_save;
#endif
	return;
    }

#ifdef FEAT_SPELL
    str = tv_get_string(&argvars[0]);
    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	maxcount = (int)tv_get_number_chk(&argvars[1], &typeerr);
	if (maxcount <= 0)
	    return;
	if (argvars[2].v_type != VAR_UNKNOWN)
	{
	    need_capital = (int)tv_get_bool_chk(&argvars[2], &typeerr);
	    if (typeerr)
		return;
	}
    }
    else
	maxcount = 25;

    spell_suggest_list(&ga, str, maxcount, need_capital, FALSE);

    for (i = 0; i < ga.ga_len; ++i)
    {
	str = ((char_u **)ga.ga_data)[i];

	li = listitem_alloc();
	if (li == NULL)
	    vim_free(str);
	else
	{
	    li->li_tv.v_type = VAR_STRING;
	    li->li_tv.v_lock = 0;
	    li->li_tv.vval.v_string = str;
	    list_append(rettv->vval.v_list, li);
	}
    }
    ga_clear(&ga);
    curwin->w_p_spell = wo_spell_save;
#endif
}

    static void
f_split(typval_T *argvars, typval_T *rettv)
{
    char_u	*str;
    char_u	*end;
    char_u	*pat = NULL;
    regmatch_T	regmatch;
    char_u	patbuf[NUMBUFLEN];
    char_u	*save_cpo;
    int		match;
    colnr_T	col = 0;
    int		keepempty = FALSE;
    int		typeerr = FALSE;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && check_for_opt_bool_arg(argvars, 2) == FAIL)))
	return;

    // Make 'cpoptions' empty, the 'l' flag should not be used here.
    save_cpo = p_cpo;
    p_cpo = empty_option;

    str = tv_get_string(&argvars[0]);
    if (argvars[1].v_type != VAR_UNKNOWN)
    {
	pat = tv_get_string_buf_chk(&argvars[1], patbuf);
	if (pat == NULL)
	    typeerr = TRUE;
	if (argvars[2].v_type != VAR_UNKNOWN)
	    keepempty = (int)tv_get_bool_chk(&argvars[2], &typeerr);
    }
    if (pat == NULL || *pat == NUL)
	pat = (char_u *)"[\\x01- ]\\+";

    if (rettv_list_alloc(rettv) == FAIL)
	goto theend;
    if (typeerr)
	goto theend;

    regmatch.regprog = vim_regcomp(pat, RE_MAGIC + RE_STRING);
    if (regmatch.regprog != NULL)
    {
	regmatch.rm_ic = FALSE;
	while (*str != NUL || keepempty)
	{
	    if (*str == NUL)
		match = FALSE;	// empty item at the end
	    else
		match = vim_regexec_nl(&regmatch, str, col);
	    if (match)
		end = regmatch.startp[0];
	    else
		end = str + STRLEN(str);
	    if (keepempty || end > str || (rettv->vval.v_list->lv_len > 0
			   && *str != NUL && match && end < regmatch.endp[0]))
	    {
		if (list_append_string(rettv->vval.v_list, str,
						    (int)(end - str)) == FAIL)
		    break;
	    }
	    if (!match)
		break;
	    // Advance to just after the match.
	    if (regmatch.endp[0] > str)
		col = 0;
	    else
		// Don't get stuck at the same match.
		col = (*mb_ptr2len)(regmatch.endp[0]);
	    str = regmatch.endp[0];
	}

	vim_regfree(regmatch.regprog);
    }

theend:
    p_cpo = save_cpo;
}

/*
 * "submatch()" function
 */
    static void
f_submatch(typval_T *argvars, typval_T *rettv)
{
    int		error = FALSE;
    int		no;
    int		retList = 0;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_opt_bool_arg(argvars, 1) == FAIL))
	return;

    no = (int)tv_get_number_chk(&argvars[0], &error);
    if (error)
	return;
    if (no < 0 || no >= NSUBEXP)
    {
	semsg(_(e_invalid_submatch_number_nr), no);
	return;
    }
    if (argvars[1].v_type != VAR_UNKNOWN)
	retList = (int)tv_get_bool_chk(&argvars[1], &error);
    if (error)
	return;

    if (retList == 0)
    {
	rettv->v_type = VAR_STRING;
	rettv->vval.v_string = reg_submatch(no);
    }
    else
    {
	rettv->v_type = VAR_LIST;
	rettv->vval.v_list = reg_submatch_list(no);
    }
}

/*
 * "substitute()" function
 */
    static void
f_substitute(typval_T *argvars, typval_T *rettv)
{
    char_u	patbuf[NUMBUFLEN];
    char_u	subbuf[NUMBUFLEN];
    char_u	flagsbuf[NUMBUFLEN];
    char_u	*str;
    char_u	*pat;
    char_u	*sub = NULL;
    typval_T	*expr = NULL;
    char_u	*flg;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_string_arg(argvars, 3) == FAIL))
	return;

    str = tv_get_string_chk(&argvars[0]);
    pat = tv_get_string_buf_chk(&argvars[1], patbuf);
    flg = tv_get_string_buf_chk(&argvars[3], flagsbuf);

    if (argvars[2].v_type == VAR_FUNC
	    || argvars[2].v_type == VAR_PARTIAL
	    || argvars[2].v_type == VAR_INSTR
	    || argvars[2].v_type == VAR_CLASS
	    || argvars[2].v_type == VAR_OBJECT)
	expr = &argvars[2];
    else
	sub = tv_get_string_buf_chk(&argvars[2], subbuf);

    rettv->v_type = VAR_STRING;
    if (str == NULL || pat == NULL || (sub == NULL && expr == NULL)
								|| flg == NULL)
	rettv->vval.v_string = NULL;
    else
	rettv->vval.v_string = do_string_sub(str, pat, sub, expr, flg);
}

/*
 * "swapfilelist()" function
 */
    static void
f_swapfilelist(typval_T *argvars UNUSED, typval_T *rettv)
{
    if (rettv_list_alloc(rettv) == FAIL)
	return;
    recover_names(NULL, FALSE, rettv->vval.v_list, 0, NULL);
}

/*
 * "swapinfo(swap_filename)" function
 */
    static void
f_swapinfo(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    if (rettv_dict_alloc(rettv) == OK)
	get_b0_dict(tv_get_string(argvars), rettv->vval.v_dict);
}

/*
 * "swapname(expr)" function
 */
    static void
f_swapname(typval_T *argvars, typval_T *rettv)
{
    buf_T	*buf;

    rettv->v_type = VAR_STRING;

    if (in_vim9script() && check_for_buffer_arg(argvars, 0) == FAIL)
	return;

    buf = tv_get_buf(&argvars[0], FALSE);
    if (buf == NULL || buf->b_ml.ml_mfp == NULL
					|| buf->b_ml.ml_mfp->mf_fname == NULL)
	rettv->vval.v_string = NULL;
    else
	rettv->vval.v_string = vim_strsave(buf->b_ml.ml_mfp->mf_fname);
}

/*
 * "synID(lnum, col, trans)" function
 */
    static void
f_synID(typval_T *argvars UNUSED, typval_T *rettv)
{
    int		id = 0;
#ifdef FEAT_SYN_HL
    linenr_T	lnum;
    colnr_T	col;
    int		trans;
    int		transerr = FALSE;

    if (in_vim9script()
	    && (check_for_lnum_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL
		|| check_for_bool_arg(argvars, 2) == FAIL))
	return;

    lnum = tv_get_lnum(argvars);		// -1 on type error
    col = (linenr_T)tv_get_number(&argvars[1]) - 1;	// -1 on type error
    trans = (int)tv_get_bool_chk(&argvars[2], &transerr);

    if (!transerr && lnum >= 1 && lnum <= curbuf->b_ml.ml_line_count
	    && col >= 0 && col < (long)STRLEN(ml_get(lnum)))
	id = syn_get_id(curwin, lnum, col, trans, NULL, FALSE);
#endif

    rettv->vval.v_number = id;
}

/*
 * "synIDattr(id, what [, mode])" function
 */
    static void
f_synIDattr(typval_T *argvars UNUSED, typval_T *rettv)
{
    char_u	*p = NULL;
#ifdef FEAT_SYN_HL
    int		id;
    char_u	*what;
    char_u	*mode;
    char_u	modebuf[NUMBUFLEN];
    int		modec;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_string_arg(argvars, 1) == FAIL
		|| check_for_opt_string_arg(argvars, 2) == FAIL))
	return;

    id = (int)tv_get_number(&argvars[0]);
    what = tv_get_string(&argvars[1]);
    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	mode = tv_get_string_buf(&argvars[2], modebuf);
	modec = TOLOWER_ASC(mode[0]);
	if (modec != 't' && modec != 'c' && modec != 'g')
	    modec = 0;	// replace invalid with current
    }
    else
    {
#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
	if (USE_24BIT)
	    modec = 'g';
	else
#endif
	    if (t_colors > 1)
		modec = 'c';
	    else
		modec = 't';
    }

    switch (TOLOWER_ASC(what[0]))
    {
	case 'b':
		if (TOLOWER_ASC(what[1]) == 'g')	// bg[#]
		    p = highlight_color(id, what, modec);
		else					// bold
		    p = highlight_has_attr(id, HL_BOLD, modec);
		break;

	case 'f':					// fg[#] or font
		p = highlight_color(id, what, modec);
		break;

	case 'i':
		if (TOLOWER_ASC(what[1]) == 'n')	// inverse
		    p = highlight_has_attr(id, HL_INVERSE, modec);
		else					// italic
		    p = highlight_has_attr(id, HL_ITALIC, modec);
		break;

	case 'n':
		if (TOLOWER_ASC(what[1]) == 'o')	// nocombine
		    p = highlight_has_attr(id, HL_NOCOMBINE, modec);
		else					// name
		    p = get_highlight_name_ext(NULL, id - 1, FALSE);
		break;

	case 'r':					// reverse
		p = highlight_has_attr(id, HL_INVERSE, modec);
		break;

	case 's':
		if (TOLOWER_ASC(what[1]) == 'p')	// sp[#]
		    p = highlight_color(id, what, modec);
							// strikeout
		else if (TOLOWER_ASC(what[1]) == 't' &&
			TOLOWER_ASC(what[2]) == 'r')
		    p = highlight_has_attr(id, HL_STRIKETHROUGH, modec);
		else					// standout
		    p = highlight_has_attr(id, HL_STANDOUT, modec);
		break;

	case 'u':
		if (STRLEN(what) >= 9)
		{
		    if (TOLOWER_ASC(what[5]) == 'l')
							// underline
			p = highlight_has_attr(id, HL_UNDERLINE, modec);
		    else if (TOLOWER_ASC(what[5]) != 'd')
							// undercurl
			p = highlight_has_attr(id, HL_UNDERCURL, modec);
		    else if (TOLOWER_ASC(what[6]) != 'o')
							// underdashed
			p = highlight_has_attr(id, HL_UNDERDASHED, modec);
		    else if (TOLOWER_ASC(what[7]) == 'u')
							// underdouble
			p = highlight_has_attr(id, HL_UNDERDOUBLE, modec);
		    else
							// underdotted
			p = highlight_has_attr(id, HL_UNDERDOTTED, modec);
		}
		else
							// ul
		    p = highlight_color(id, what, modec);
		break;
    }

    if (p != NULL)
	p = vim_strsave(p);
#endif
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = p;
}

/*
 * "synIDtrans(id)" function
 */
    static void
f_synIDtrans(typval_T *argvars UNUSED, typval_T *rettv)
{
    int		id;

#ifdef FEAT_SYN_HL
    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    id = (int)tv_get_number(&argvars[0]);

    if (id > 0)
	id = syn_get_final_id(id);
    else
#endif
	id = 0;

    rettv->vval.v_number = id;
}

/*
 * "synconcealed(lnum, col)" function
 */
    static void
f_synconcealed(typval_T *argvars UNUSED, typval_T *rettv)
{
#if defined(FEAT_SYN_HL) && defined(FEAT_CONCEAL)
    linenr_T	lnum;
    colnr_T	col;
    int		syntax_flags = 0;
    int		cchar;
    int		matchid = 0;
    char_u	str[NUMBUFLEN];
#endif

    rettv_list_set(rettv, NULL);

    if (in_vim9script()
	    && (check_for_lnum_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

#if defined(FEAT_SYN_HL) && defined(FEAT_CONCEAL)
    lnum = tv_get_lnum(argvars);		// -1 on type error
    col = (colnr_T)tv_get_number(&argvars[1]) - 1;	// -1 on type error

    CLEAR_FIELD(str);

    if (rettv_list_alloc(rettv) == OK)
    {
	if (lnum >= 1 && lnum <= curbuf->b_ml.ml_line_count
	    && col >= 0 && col <= (long)STRLEN(ml_get(lnum))
	    && curwin->w_p_cole > 0)
	{
	    (void)syn_get_id(curwin, lnum, col, FALSE, NULL, FALSE);
	    syntax_flags = get_syntax_info(&matchid);

	    // get the conceal character
	    if ((syntax_flags & HL_CONCEAL) && curwin->w_p_cole < 3)
	    {
		cchar = syn_get_sub_char();
		if (cchar == NUL && curwin->w_p_cole == 1)
		    cchar = (curwin->w_lcs_chars.conceal == NUL) ? ' '
					: curwin->w_lcs_chars.conceal;
		if (cchar != NUL)
		{
		    if (has_mbyte)
			(*mb_char2bytes)(cchar, str);
		    else
			str[0] = cchar;
		}
	    }
	}

	list_append_number(rettv->vval.v_list,
					    (syntax_flags & HL_CONCEAL) != 0);
	// -1 to auto-determine strlen
	list_append_string(rettv->vval.v_list, str, -1);
	list_append_number(rettv->vval.v_list, matchid);
    }
#endif
}

/*
 * "synstack(lnum, col)" function
 */
    static void
f_synstack(typval_T *argvars UNUSED, typval_T *rettv)
{
#ifdef FEAT_SYN_HL
    linenr_T	lnum;
    colnr_T	col;
    int		i;
    int		id;
#endif

    rettv_list_set(rettv, NULL);

    if (in_vim9script()
	    && (check_for_lnum_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

#ifdef FEAT_SYN_HL
    lnum = tv_get_lnum(argvars);		// -1 on type error
    col = (colnr_T)tv_get_number(&argvars[1]) - 1;	// -1 on type error

    if (lnum >= 1 && lnum <= curbuf->b_ml.ml_line_count
	    && col >= 0 && col <= (long)STRLEN(ml_get(lnum))
	    && rettv_list_alloc(rettv) == OK)
    {
	(void)syn_get_id(curwin, lnum, col, FALSE, NULL, TRUE);
	for (i = 0; ; ++i)
	{
	    id = syn_get_stack_item(i);
	    if (id < 0)
		break;
	    if (list_append_number(rettv->vval.v_list, id) == FAIL)
		break;
	}
    }
#endif
}

/*
 * "tabpagebuflist()" function
 */
    static void
f_tabpagebuflist(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
    tabpage_T	*tp;
    win_T	*wp = NULL;

    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_UNKNOWN)
	wp = firstwin;
    else
    {
	tp = find_tabpage((int)tv_get_number(&argvars[0]));
	if (tp != NULL)
	    wp = (tp == curtab) ? firstwin : tp->tp_firstwin;
    }
    if (wp != NULL && rettv_list_alloc(rettv) == OK)
    {
	for (; wp != NULL; wp = wp->w_next)
	    if (list_append_number(rettv->vval.v_list,
						wp->w_buffer->b_fnum) == FAIL)
		break;
    }
}

/*
 * "tagfiles()" function
 */
    static void
f_tagfiles(typval_T *argvars UNUSED, typval_T *rettv)
{
    char_u	*fname;
    tagname_T	tn;
    int		first;

    if (rettv_list_alloc(rettv) == FAIL)
	return;
    fname = alloc(MAXPATHL);
    if (fname == NULL)
	return;

    for (first = TRUE; ; first = FALSE)
	if (get_tagfname(&tn, first, fname) == FAIL
		|| list_append_string(rettv->vval.v_list, fname, -1) == FAIL)
	    break;
    tagname_free(&tn);
    vim_free(fname);
}

/*
 * "taglist()" function
 */
    static void
f_taglist(typval_T *argvars, typval_T *rettv)
{
    char_u  *fname = NULL;
    char_u  *tag_pattern;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL))
	return;

    tag_pattern = tv_get_string(&argvars[0]);

    rettv->vval.v_number = FALSE;
    if (*tag_pattern == NUL)
	return;

    if (argvars[1].v_type != VAR_UNKNOWN)
	fname = tv_get_string(&argvars[1]);
    if (rettv_list_alloc(rettv) == OK)
	(void)get_tags(rettv->vval.v_list, tag_pattern, fname);
}

/*
 * "type(expr)" function
 */
    static void
f_type(typval_T *argvars, typval_T *rettv)
{
    int n = -1;

    switch (argvars[0].v_type)
    {
	case VAR_NUMBER:  n = VAR_TYPE_NUMBER; break;
	case VAR_STRING:  n = VAR_TYPE_STRING; break;
	case VAR_PARTIAL:
	case VAR_FUNC:    n = VAR_TYPE_FUNC; break;
	case VAR_LIST:    n = VAR_TYPE_LIST; break;
	case VAR_DICT:    n = VAR_TYPE_DICT; break;
	case VAR_FLOAT:   n = VAR_TYPE_FLOAT; break;
	case VAR_BOOL:	  n = VAR_TYPE_BOOL; break;
	case VAR_SPECIAL: n = VAR_TYPE_NONE; break;
	case VAR_JOB:     n = VAR_TYPE_JOB; break;
	case VAR_CHANNEL: n = VAR_TYPE_CHANNEL; break;
	case VAR_BLOB:    n = VAR_TYPE_BLOB; break;
	case VAR_INSTR:   n = VAR_TYPE_INSTR; break;
	case VAR_CLASS:   n = VAR_TYPE_CLASS; break;
	case VAR_OBJECT:  n = VAR_TYPE_OBJECT; break;
	case VAR_TYPEALIAS: n = VAR_TYPE_TYPEALIAS; break;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	     internal_error_no_abort("f_type(UNKNOWN)");
	     n = -1;
	     break;
    }
    rettv->vval.v_number = n;
}

/*
 * "virtcol({expr}, [, {list} [, {winid}]])" function
 */
    static void
f_virtcol(typval_T *argvars, typval_T *rettv)
{
    colnr_T	vcol_start = 0;
    colnr_T	vcol_end = 0;
    pos_T	*fp;
    switchwin_T	switchwin;
    int		winchanged = FALSE;
    int		len;

    if (in_vim9script()
	    && (check_for_string_or_list_arg(argvars, 0) == FAIL
		|| (argvars[1].v_type != VAR_UNKNOWN
		    && (check_for_bool_arg(argvars, 1) == FAIL
			|| check_for_opt_number_arg(argvars, 2) == FAIL))))
	return;

    if (argvars[1].v_type != VAR_UNKNOWN && argvars[2].v_type != VAR_UNKNOWN)
    {
	tabpage_T	*tp;
	win_T		*wp;

	// use the window specified in the third argument
	wp = win_id2wp_tp((int)tv_get_number(&argvars[2]), &tp);
	if (wp == NULL || tp == NULL)
	    goto theend;

	if (switch_win_noblock(&switchwin, wp, tp, TRUE) != OK)
	    goto theend;

	check_cursor();
	winchanged = TRUE;
    }

    int fnum = curbuf->b_fnum;
    fp = var2fpos(&argvars[0], FALSE, &fnum, FALSE);
    if (fp != NULL && fp->lnum <= curbuf->b_ml.ml_line_count
	    && fnum == curbuf->b_fnum)
    {
	// Limit the column to a valid value, getvvcol() doesn't check.
	if (fp->col < 0)
	    fp->col = 0;
	else
	{
	    len = (int)STRLEN(ml_get(fp->lnum));
	    if (fp->col > len)
		fp->col = len;
	}
	getvvcol(curwin, fp, &vcol_start, NULL, &vcol_end);
	++vcol_start;
	++vcol_end;
    }

theend:
    if (argvars[1].v_type != VAR_UNKNOWN && tv_get_bool(&argvars[1]))
    {
	if (rettv_list_alloc(rettv) == OK)
	{
	    list_append_number(rettv->vval.v_list, vcol_start);
	    list_append_number(rettv->vval.v_list, vcol_end);
	}
	else
	    rettv->vval.v_number = 0;
    }
    else
	rettv->vval.v_number = vcol_end;

    if (winchanged)
	restore_win_noblock(&switchwin, TRUE);
}

/*
 * "visualmode()" function
 */
    static void
f_visualmode(typval_T *argvars, typval_T *rettv)
{
    char_u	str[2];

    if (in_vim9script() && check_for_opt_bool_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_STRING;
    str[0] = curbuf->b_visual_mode_eval;
    str[1] = NUL;
    rettv->vval.v_string = vim_strsave(str);

    // A non-zero number or non-empty string argument: reset mode.
    if (non_zero_arg(&argvars[0]))
	curbuf->b_visual_mode_eval = NUL;
}

/*
 * "wildmenumode()" function
 */
    static void
f_wildmenumode(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
    if (wild_menu_showing || ((State & MODE_CMDLINE) && cmdline_pum_active()))
	rettv->vval.v_number = 1;
}

/*
 * "windowsversion()" function
 */
    static void
f_windowsversion(typval_T *argvars UNUSED, typval_T *rettv UNUSED)
{
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = vim_strsave((char_u *)windowsVersion);
}

/*
 * "wordcount()" function
 */
    static void
f_wordcount(typval_T *argvars UNUSED, typval_T *rettv)
{
    if (rettv_dict_alloc(rettv) == FAIL)
	return;
    cursor_pos_info(rettv->vval.v_dict);
}

/*
 * "xor(expr, expr)" function
 */
    static void
f_xor(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    rettv->vval.v_number = tv_get_number_chk(&argvars[0], NULL)
					^ tv_get_number_chk(&argvars[1], NULL);
}

#endif // FEAT_EVAL
