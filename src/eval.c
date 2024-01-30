/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * eval.c: Expression evaluation.
 */
#define USING_FLOAT_STUFF

#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

#ifdef VMS
# include <float.h>
#endif

#define NAMESPACE_CHAR	(char_u *)"abglstvw"

/*
 * When recursively copying lists and dicts we need to remember which ones we
 * have done to avoid endless recursiveness.  This unique ID is used for that.
 * The last bit is used for previous_funccal, ignored when comparing.
 */
static int current_copyID = 0;

static int eval2(char_u **arg, typval_T *rettv, evalarg_T *evalarg);
static int eval3(char_u **arg, typval_T *rettv, evalarg_T *evalarg);
static int eval4(char_u **arg, typval_T *rettv, evalarg_T *evalarg);
static int eval5(char_u **arg, typval_T *rettv, evalarg_T *evalarg);
static int eval6(char_u **arg, typval_T *rettv, evalarg_T *evalarg);
static int eval7(char_u **arg, typval_T *rettv, evalarg_T *evalarg, int want_string);
static int eval8(char_u **arg, typval_T *rettv, evalarg_T *evalarg, int want_string);
static int eval9(char_u **arg, typval_T *rettv, evalarg_T *evalarg, int want_string);
static int eval9_leader(typval_T *rettv, int numeric_only, char_u *start_leader, char_u **end_leaderp);

static int free_unref_items(int copyID);
static char_u *make_expanded_name(char_u *in_start, char_u *expr_start, char_u *expr_end, char_u *in_end);

/*
 * Return "n1" divided by "n2", taking care of dividing by zero.
 * If "failed" is not NULL set it to TRUE when dividing by zero fails.
 */
	varnumber_T
num_divide(varnumber_T n1, varnumber_T n2, int *failed)
{
    varnumber_T	result;

    if (n2 == 0)
    {
	if (in_vim9script())
	{
	    emsg(_(e_divide_by_zero));
	    if (failed != NULL)
		*failed = TRUE;
	}
	if (n1 == 0)
	    result = VARNUM_MIN; // similar to NaN
	else if (n1 < 0)
	    result = -VARNUM_MAX;
	else
	    result = VARNUM_MAX;
    }
    else if (n1 == VARNUM_MIN && n2 == -1)
    {
	// specific case: trying to do VARNUM_MIN / -1 results in a positive
	// number that doesn't fit in varnumber_T and causes an FPE
	result = VARNUM_MAX;
    }
    else
	result = n1 / n2;

    return result;
}

/*
 * Return "n1" modulus "n2", taking care of dividing by zero.
 * If "failed" is not NULL set it to TRUE when dividing by zero fails.
 */
	varnumber_T
num_modulus(varnumber_T n1, varnumber_T n2, int *failed)
{
    if (n2 == 0 && in_vim9script())
    {
	emsg(_(e_divide_by_zero));
	if (failed != NULL)
	    *failed = TRUE;
    }
    return (n2 == 0) ? 0 : (n1 % n2);
}

/*
 * Initialize the global and v: variables.
 */
    void
eval_init(void)
{
    evalvars_init();
    func_init();
}

#if defined(EXITFREE) || defined(PROTO)
    void
eval_clear(void)
{
    evalvars_clear();
    free_scriptnames();  // must come after evalvars_clear().
    free_locales();

    // autoloaded script names
    free_autoload_scriptnames();

    // unreferenced lists and dicts
    (void)garbage_collect(FALSE);

    // functions not garbage collected
    free_all_functions();
}
#endif

    void
fill_evalarg_from_eap(evalarg_T *evalarg, exarg_T *eap, int skip)
{
    init_evalarg(evalarg);
    evalarg->eval_flags = skip ? 0 : EVAL_EVALUATE;

    if (eap == NULL)
	return;

    evalarg->eval_cstack = eap->cstack;
    if (sourcing_a_script(eap) || eap->ea_getline == get_list_line)
    {
	evalarg->eval_getline = eap->ea_getline;
	evalarg->eval_cookie = eap->cookie;
    }
}

/*
 * Top level evaluation function, returning a boolean.
 * Sets "error" to TRUE if there was an error.
 * Return TRUE or FALSE.
 */
    int
eval_to_bool(
    char_u	*arg,
    int		*error,
    exarg_T	*eap,
    int		skip,	    // only parse, don't execute
    int		use_simple_function)
{
    typval_T	tv;
    varnumber_T	retval = FALSE;
    evalarg_T	evalarg;
    int		r;

    fill_evalarg_from_eap(&evalarg, eap, skip);

    if (skip)
	++emsg_skip;
    if (use_simple_function)
	r = eval0_simple_funccal(arg, &tv, eap, &evalarg);
    else
	r = eval0(arg, &tv, eap, &evalarg);
    if (r == FAIL)
	*error = TRUE;
    else
    {
	*error = FALSE;
	if (!skip)
	{
	    if (in_vim9script())
		retval = tv_get_bool_chk(&tv, error);
	    else
		retval = (tv_get_number_chk(&tv, error) != 0);
	    clear_tv(&tv);
	}
    }
    if (skip)
	--emsg_skip;
    clear_evalarg(&evalarg, eap);

    return (int)retval;
}

/*
 * Call eval1() and give an error message if not done at a lower level.
 */
    static int
eval1_emsg(char_u **arg, typval_T *rettv, exarg_T *eap)
{
    char_u	*start = *arg;
    int		ret;
    int		did_emsg_before = did_emsg;
    int		called_emsg_before = called_emsg;
    evalarg_T	evalarg;

    fill_evalarg_from_eap(&evalarg, eap, eap != NULL && eap->skip);

    ret = eval1(arg, rettv, &evalarg);
    if (ret == FAIL)
    {
	// Report the invalid expression unless the expression evaluation has
	// been cancelled due to an aborting error, an interrupt, or an
	// exception, or we already gave a more specific error.
	// Also check called_emsg for when using assert_fails().
	if (!aborting() && did_emsg == did_emsg_before
					  && called_emsg == called_emsg_before)
	    semsg(_(e_invalid_expression_str), start);
    }
    clear_evalarg(&evalarg, eap);
    return ret;
}

/*
 * Return whether a typval is a valid expression to pass to eval_expr_typval()
 * or eval_expr_to_bool().  An empty string returns FALSE;
 */
    int
eval_expr_valid_arg(typval_T *tv)
{
    return tv->v_type != VAR_UNKNOWN
	    && (tv->v_type != VAR_STRING
		  || (tv->vval.v_string != NULL && *tv->vval.v_string != NUL));
}

/*
 * When calling eval_expr_typval() many times we only need one funccall_T.
 * Returns NULL when no funccall_T is to be used.
 * When returning non-NULL remove_funccal() must be called later.
 */
    funccall_T *
eval_expr_get_funccal(typval_T *expr, typval_T *rettv)
{
    if (expr->v_type != VAR_PARTIAL)
	return NULL;

    partial_T *partial = expr->vval.v_partial;
    if (partial == NULL)
	return NULL;
    if (partial->pt_func == NULL
	    || partial->pt_func->uf_def_status == UF_NOT_COMPILED)
	return NULL;

    return create_funccal(partial->pt_func, rettv);
}

/*
 * Evaluate an expression, which can be a function, partial or string.
 * Pass arguments "argv[argc]".
 * If "want_func" is TRUE treat a string as a function name, not an expression.
 * "fc_arg" is from eval_expr_get_funccal() or NULL;
 * Return the result in "rettv" and OK or FAIL.
 */
    int
eval_expr_typval(
	typval_T    *expr,
	int	    want_func,
	typval_T    *argv,
	int	    argc,
	funccall_T  *fc_arg,
	typval_T    *rettv)
{
    char_u	*s;
    char_u	buf[NUMBUFLEN];
    funcexe_T	funcexe;

    if (expr->v_type == VAR_PARTIAL)
    {
	partial_T   *partial = expr->vval.v_partial;

	if (partial == NULL)
	    return FAIL;

	if (partial->pt_func != NULL
			  && partial->pt_func->uf_def_status != UF_NOT_COMPILED)
	{
	    funccall_T	*fc = fc_arg != NULL ? fc_arg
				     : create_funccal(partial->pt_func, rettv);
	    int		r;

	    if (fc == NULL)
		return FAIL;

	    // Shortcut to call a compiled function with minimal overhead.
	    r = call_def_function(partial->pt_func, argc, argv,
				    DEF_USE_PT_ARGV, partial, NULL, fc, rettv);
	    if (fc_arg == NULL)
		remove_funccal();
	    if (r == FAIL)
		return FAIL;
	}
	else
	{
	    s = partial_name(partial);
	    if (s == NULL || *s == NUL)
		return FAIL;
	    CLEAR_FIELD(funcexe);
	    funcexe.fe_evaluate = TRUE;
	    funcexe.fe_partial = partial;
	    if (call_func(s, -1, rettv, argc, argv, &funcexe) == FAIL)
		return FAIL;
	}
    }
    else if (expr->v_type == VAR_INSTR)
    {
	return exe_typval_instr(expr, rettv);
    }
    else if (expr->v_type == VAR_FUNC || want_func)
    {
	s = expr->v_type == VAR_FUNC
		? expr->vval.v_string
		: tv_get_string_buf_chk_strict(expr, buf, in_vim9script());
	if (s == NULL || *s == NUL)
	    return FAIL;
	CLEAR_FIELD(funcexe);
	funcexe.fe_evaluate = TRUE;
	if (call_func(s, -1, rettv, argc, argv, &funcexe) == FAIL)
	    return FAIL;
    }
    else
    {
	s = tv_get_string_buf_chk_strict(expr, buf, in_vim9script());
	if (s == NULL)
	    return FAIL;
	s = skipwhite(s);
	if (eval1_emsg(&s, rettv, NULL) == FAIL)
	    return FAIL;
	if (*skipwhite(s) != NUL)  // check for trailing chars after expr
	{
	    clear_tv(rettv);
	    semsg(_(e_invalid_expression_str), s);
	    return FAIL;
	}
    }
    return OK;
}

/*
 * Like eval_to_bool() but using a typval_T instead of a string.
 * Works for string, funcref and partial.
 */
    int
eval_expr_to_bool(typval_T *expr, int *error)
{
    typval_T	rettv;
    int		res;

    if (eval_expr_typval(expr, FALSE, NULL, 0, NULL, &rettv) == FAIL)
    {
	*error = TRUE;
	return FALSE;
    }
    res = (tv_get_bool_chk(&rettv, error) != 0);
    clear_tv(&rettv);
    return res;
}

/*
 * Top level evaluation function, returning a string.  If "skip" is TRUE,
 * only parsing to "nextcmd" is done, without reporting errors.  Return
 * pointer to allocated memory, or NULL for failure or when "skip" is TRUE.
 */
    char_u *
eval_to_string_skip(
    char_u	*arg,
    exarg_T	*eap,
    int		skip)	    // only parse, don't execute
{
    typval_T	tv;
    char_u	*retval;
    evalarg_T	evalarg;

    fill_evalarg_from_eap(&evalarg, eap, skip);
    if (skip)
	++emsg_skip;
    if (eval0(arg, &tv, eap, &evalarg) == FAIL || skip)
	retval = NULL;
    else
    {
	retval = vim_strsave(tv_get_string(&tv));
	clear_tv(&tv);
    }
    if (skip)
	--emsg_skip;
    clear_evalarg(&evalarg, eap);

    return retval;
}

/*
 * Initialize "evalarg" for use.
 */
    void
init_evalarg(evalarg_T *evalarg)
{
    CLEAR_POINTER(evalarg);
    ga_init2(&evalarg->eval_tofree_ga, sizeof(char_u *), 20);
}

/*
 * If "evalarg->eval_tofree" is not NULL free it later.
 * Caller is expected to overwrite "evalarg->eval_tofree" next.
 */
    static void
free_eval_tofree_later(evalarg_T *evalarg)
{
    if (evalarg->eval_tofree == NULL)
	return;

    if (ga_grow(&evalarg->eval_tofree_ga, 1) == OK)
	((char_u **)evalarg->eval_tofree_ga.ga_data)
	    [evalarg->eval_tofree_ga.ga_len++]
	    = evalarg->eval_tofree;
    else
	vim_free(evalarg->eval_tofree);
}

/*
 * After using "evalarg" filled from "eap": free the memory.
 */
    void
clear_evalarg(evalarg_T *evalarg, exarg_T *eap)
{
    if (evalarg == NULL)
	return;

    garray_T *etga = &evalarg->eval_tofree_ga;

    if (evalarg->eval_tofree != NULL || evalarg->eval_using_cmdline)
    {
	if (eap != NULL)
	{
	    // We may need to keep the original command line, e.g. for
	    // ":let" it has the variable names.  But we may also need
	    // the new one, "nextcmd" points into it.  Keep both.
	    vim_free(eap->cmdline_tofree);
	    eap->cmdline_tofree = *eap->cmdlinep;

	    if (evalarg->eval_using_cmdline && etga->ga_len > 0)
	    {
		// "nextcmd" points into the last line in eval_tofree_ga,
		// need to keep it around.
		--etga->ga_len;
		*eap->cmdlinep = ((char_u **)etga->ga_data)[etga->ga_len];
		vim_free(evalarg->eval_tofree);
	    }
	    else
		*eap->cmdlinep = evalarg->eval_tofree;
	}
	else
	    vim_free(evalarg->eval_tofree);
	evalarg->eval_tofree = NULL;
    }

    ga_clear_strings(etga);
    VIM_CLEAR(evalarg->eval_tofree_lambda);
}

/*
 * Skip over an expression at "*pp".
 * Return FAIL for an error, OK otherwise.
 */
    int
skip_expr(char_u **pp, evalarg_T *evalarg)
{
    typval_T	rettv;

    *pp = skipwhite(*pp);
    return eval1(pp, &rettv, evalarg);
}

/*
 * Skip over an expression at "*arg".
 * If in Vim9 script and line breaks are encountered, the lines are
 * concatenated.  "evalarg->eval_tofree" will be set accordingly.
 * "arg" is advanced to just after the expression.
 * "start" is set to the start of the expression, "end" to just after the end.
 * Also when the expression is copied to allocated memory.
 * Return FAIL for an error, OK otherwise.
 */
    int
skip_expr_concatenate(
	char_u	    **arg,
	char_u	    **start,
	char_u	    **end,
	evalarg_T   *evalarg)
{
    typval_T	rettv;
    int		res;
    int		vim9script = in_vim9script();
    garray_T    *gap = evalarg == NULL ? NULL : &evalarg->eval_ga;
    garray_T    *freegap = evalarg == NULL ? NULL : &evalarg->eval_freega;
    int		save_flags = evalarg == NULL ? 0 : evalarg->eval_flags;
    int		evaluate = evalarg == NULL
			       ? FALSE : (evalarg->eval_flags & EVAL_EVALUATE);

    if (vim9script && evaluate
	       && (evalarg->eval_cookie != NULL || evalarg->eval_cctx != NULL))
    {
	ga_init2(gap, sizeof(char_u *), 10);
	// leave room for "start"
	if (ga_grow(gap, 1) == OK)
	    ++gap->ga_len;
	ga_init2(freegap, sizeof(char_u *), 10);
    }
    *start = *arg;

    // Don't evaluate the expression.
    if (evalarg != NULL)
	evalarg->eval_flags &= ~EVAL_EVALUATE;
    *arg = skipwhite(*arg);
    res = eval1(arg, &rettv, evalarg);
    *end = *arg;
    if (evalarg != NULL)
	evalarg->eval_flags = save_flags;

    if (vim9script && evaluate
	    && (evalarg->eval_cookie != NULL || evalarg->eval_cctx != NULL))
    {
	if (evalarg->eval_ga.ga_len == 1)
	{
	    // just the one line, no need to concatenate
	    ga_clear(gap);
	    gap->ga_itemsize = 0;
	}
	else
	{
	    char_u	    *p;
	    size_t	    endoff = STRLEN(*arg);

	    // Line breaks encountered, concatenate all the lines.
	    *((char_u **)gap->ga_data) = *start;
	    p = ga_concat_strings(gap, " ");

	    // free the lines only when using getsourceline()
	    if (evalarg->eval_cookie != NULL)
	    {
		// Do not free the first line, the caller can still use it.
		*((char_u **)gap->ga_data) = NULL;
		// Do not free the last line, "arg" points into it, free it
		// later.  Also free "eval_tofree" later if needed.
		free_eval_tofree_later(evalarg);
		evalarg->eval_tofree =
				    ((char_u **)gap->ga_data)[gap->ga_len - 1];
		((char_u **)gap->ga_data)[gap->ga_len - 1] = NULL;
		ga_clear_strings(gap);
	    }
	    else
	    {
		ga_clear(gap);

		// free lines that were explicitly marked for freeing
		ga_clear_strings(freegap);
	    }

	    gap->ga_itemsize = 0;
	    if (p == NULL)
		return FAIL;
	    *start = p;
	    vim_free(evalarg->eval_tofree_lambda);
	    evalarg->eval_tofree_lambda = p;
	    // Compute "end" relative to the end.
	    *end = *start + STRLEN(*start) - endoff;
	}
    }

    return res;
}

/*
 * Convert "tv" to a string.
 * When "convert" is TRUE convert a List into a sequence of lines.
 * Returns an allocated string (NULL when out of memory).
 */
    char_u *
typval2string(typval_T *tv, int convert)
{
    garray_T	ga;
    char_u	*retval;

    if (convert && tv->v_type == VAR_LIST)
    {
	ga_init2(&ga, sizeof(char), 80);
	if (tv->vval.v_list != NULL)
	{
	    list_join(&ga, tv->vval.v_list, (char_u *)"\n", TRUE, FALSE, 0);
	    if (tv->vval.v_list->lv_len > 0)
		ga_append(&ga, NL);
	}
	ga_append(&ga, NUL);
	retval = (char_u *)ga.ga_data;
    }
    else
	retval = vim_strsave(tv_get_string(tv));
    return retval;
}

/*
 * Top level evaluation function, returning a string.  Does not handle line
 * breaks.
 * When "convert" is TRUE convert a List into a sequence of lines.
 * Return pointer to allocated memory, or NULL for failure.
 */
    char_u *
eval_to_string_eap(
    char_u	*arg,
    int		convert,
    exarg_T	*eap,
    int		use_simple_function)
{
    typval_T	tv;
    char_u	*retval;
    evalarg_T	evalarg;
    int		r;

    fill_evalarg_from_eap(&evalarg, eap, eap != NULL && eap->skip);
    if (use_simple_function)
	r = eval0_simple_funccal(arg, &tv, NULL, &evalarg);
    else
	r = eval0(arg, &tv, NULL, &evalarg);
    if (r == FAIL)
	retval = NULL;
    else
    {
	retval = typval2string(&tv, convert);
	clear_tv(&tv);
    }
    clear_evalarg(&evalarg, NULL);

    return retval;
}

    char_u *
eval_to_string(
    char_u	*arg,
    int		convert,
    int		use_simple_function)
{
    return eval_to_string_eap(arg, convert, NULL, use_simple_function);
}

/*
 * Call eval_to_string() without using current local variables and using
 * textlock.  When "use_sandbox" is TRUE use the sandbox.
 * Use legacy Vim script syntax.
 */
    char_u *
eval_to_string_safe(
    char_u	*arg,
    int		use_sandbox,
    int		keep_script_version,
    int		use_simple_function)
{
    char_u	*retval;
    funccal_entry_T funccal_entry;
    int		save_sc_version = current_sctx.sc_version;
    int		save_garbage = may_garbage_collect;

    if (!keep_script_version)
	current_sctx.sc_version = 1;
    save_funccal(&funccal_entry);
    if (use_sandbox)
	++sandbox;
    ++textlock;
    may_garbage_collect = FALSE;
    retval = eval_to_string(arg, FALSE, use_simple_function);
    if (use_sandbox)
	--sandbox;
    --textlock;
    may_garbage_collect = save_garbage;
    restore_funccal();
    current_sctx.sc_version = save_sc_version;
    return retval;
}

/*
 * Top level evaluation function, returning a number.
 * Evaluates "expr" silently.
 * Returns -1 for an error.
 */
    varnumber_T
eval_to_number(char_u *expr, int use_simple_function)
{
    typval_T	rettv;
    varnumber_T	retval;
    char_u	*p = skipwhite(expr);
    int		r = NOTDONE;

    ++emsg_off;

    if (use_simple_function)
	r = may_call_simple_func(expr, &rettv);
    if (r == NOTDONE)
	r = eval1(&p, &rettv, &EVALARG_EVALUATE);
    if (r == FAIL)
	retval = -1;
    else
    {
	retval = tv_get_number_chk(&rettv, NULL);
	clear_tv(&rettv);
    }
    --emsg_off;

    return retval;
}

/*
 * Top level evaluation function.
 * Returns an allocated typval_T with the result.
 * Returns NULL when there is an error.
 */
    typval_T *
eval_expr(char_u *arg, exarg_T *eap)
{
    return eval_expr_ext(arg, eap, FALSE);
}

    typval_T *
eval_expr_ext(char_u *arg, exarg_T *eap, int use_simple_function)
{
    typval_T	*tv;
    evalarg_T	evalarg;

    fill_evalarg_from_eap(&evalarg, eap, eap != NULL && eap->skip);

    tv = ALLOC_ONE(typval_T);
    if (tv != NULL)
    {
	int	r = NOTDONE;

	if (use_simple_function)
	    r = eval0_simple_funccal(arg, tv, eap, &evalarg);
	if (r == NOTDONE)
	    r = eval0(arg, tv, eap, &evalarg);

	if (r == FAIL)
	    VIM_CLEAR(tv);
    }

    clear_evalarg(&evalarg, eap);
    return tv;
}

/*
 * "*arg" points to what can be a function name in the form of "import.Name" or
 * "Funcref".  Return the name of the function.  Set "tofree" to something that
 * was allocated.
 * If "verbose" is FALSE no errors are given.
 * Return NULL for any failure.
 */
    static char_u *
deref_function_name(
	    char_u	**arg,
	    char_u	**tofree,
	    evalarg_T	*evalarg,
	    int		verbose)
{
    typval_T	ref;
    char_u	*name = *arg;
    int		save_flags = 0;

    ref.v_type = VAR_UNKNOWN;
    if (evalarg != NULL)
    {
	// need to evaluate this to get an import, like in "a.Func"
	save_flags = evalarg->eval_flags;
	evalarg->eval_flags |= EVAL_EVALUATE;
    }
    if (eval9(arg, &ref, evalarg, FALSE) == FAIL)
    {
	dictitem_T	*v;

	// If <SID>VarName was used it would not be found, try another way.
	v = find_var_also_in_script(name, NULL, FALSE);
	if (v == NULL)
	{
	    name = NULL;
	    goto theend;
	}
	copy_tv(&v->di_tv, &ref);
    }
    if (*skipwhite(*arg) != NUL)
    {
	if (verbose)
	    semsg(_(e_trailing_characters_str), *arg);
	name = NULL;
    }
    else if (ref.v_type == VAR_FUNC && ref.vval.v_string != NULL)
    {
	name = ref.vval.v_string;
	ref.vval.v_string = NULL;
	*tofree = name;
    }
    else if (ref.v_type == VAR_PARTIAL && ref.vval.v_partial != NULL)
    {
	if (ref.vval.v_partial->pt_argc > 0
		|| ref.vval.v_partial->pt_dict != NULL)
	{
	    if (verbose)
		emsg(_(e_cannot_use_partial_here));
	    name = NULL;
	}
	else
	{
	    name = vim_strsave(partial_name(ref.vval.v_partial));
	    *tofree = name;
	}
    }
    else
    {
	if (verbose)
	    semsg(_(e_not_callable_type_str), name);
	name = NULL;
    }

theend:
    clear_tv(&ref);
    if (evalarg != NULL)
	evalarg->eval_flags = save_flags;
    return name;
}

/*
 * Call some Vim script function and return the result in "*rettv".
 * Uses argv[0] to argv[argc - 1] for the function arguments.  argv[argc]
 * should have type VAR_UNKNOWN.
 * Returns OK or FAIL.
 */
    int
call_vim_function(
    char_u      *func,
    int		argc,
    typval_T	*argv,
    typval_T	*rettv)
{
    int		ret;
    funcexe_T	funcexe;
    char_u	*arg;
    char_u	*name;
    char_u	*tofree = NULL;
    int		ignore_errors;

    rettv->v_type = VAR_UNKNOWN;		// clear_tv() uses this
    CLEAR_FIELD(funcexe);
    funcexe.fe_firstline = curwin->w_cursor.lnum;
    funcexe.fe_lastline = curwin->w_cursor.lnum;
    funcexe.fe_evaluate = TRUE;

    // The name might be "import.Func" or "Funcref".  We don't know, we need to
    // ignore errors for an undefined name.  But we do want errors when an
    // autoload script has errors.  Guess that when there is a dot in the name
    // showing errors is the right choice.
    ignore_errors = vim_strchr(func, '.') == NULL;
    arg = func;
    if (ignore_errors)
	++emsg_off;
    name = deref_function_name(&arg, &tofree, &EVALARG_EVALUATE, FALSE);
    if (ignore_errors)
	--emsg_off;
    if (name == NULL)
	name = func;

    ret = call_func(name, -1, rettv, argc, argv, &funcexe);

    if (ret == FAIL)
	clear_tv(rettv);
    vim_free(tofree);

    return ret;
}

/*
 * Call Vim script function "func" and return the result as a string.
 * Uses "argv[0]" to "argv[argc - 1]" for the function arguments. "argv[argc]"
 * should have type VAR_UNKNOWN.
 * Returns NULL when calling the function fails.
 */
    void *
call_func_retstr(
    char_u      *func,
    int		argc,
    typval_T	*argv)
{
    typval_T	rettv;
    char_u	*retval;

    if (call_vim_function(func, argc, argv, &rettv) == FAIL)
	return NULL;

    retval = vim_strsave(tv_get_string(&rettv));
    clear_tv(&rettv);
    return retval;
}

/*
 * Call Vim script function "func" and return the result as a List.
 * Uses "argv" and "argc" as call_func_retstr().
 * Returns NULL when there is something wrong.
 * Gives an error when the returned value is not a list.
 */
    void *
call_func_retlist(
    char_u      *func,
    int		argc,
    typval_T	*argv)
{
    typval_T	rettv;

    if (call_vim_function(func, argc, argv, &rettv) == FAIL)
	return NULL;

    if (rettv.v_type != VAR_LIST)
    {
	semsg(_(e_custom_list_completion_function_does_not_return_list_but_str),
		vartype_name(rettv.v_type));
	clear_tv(&rettv);
	return NULL;
    }

    return rettv.vval.v_list;
}

#if defined(FEAT_FOLDING) || defined(PROTO)
/*
 * Evaluate "arg", which is 'foldexpr'.
 * Note: caller must set "curwin" to match "arg".
 * Returns the foldlevel, and any character preceding it in "*cp".  Doesn't
 * give error messages.
 */
    int
eval_foldexpr(win_T *wp, int *cp)
{
    char_u	*arg;
    typval_T	tv;
    varnumber_T	retval;
    char_u	*s;
    sctx_T	saved_sctx = current_sctx;
    int		use_sandbox = was_set_insecurely((char_u *)"foldexpr",
								    OPT_LOCAL);

    arg = skipwhite(wp->w_p_fde);
    current_sctx = wp->w_p_script_ctx[WV_FDE];

    ++emsg_off;
    if (use_sandbox)
	++sandbox;
    ++textlock;
    *cp = NUL;

    // Evaluate the expression.  If the expression is "FuncName()" call the
    // function directly.
    if (eval0_simple_funccal(arg, &tv, NULL, &EVALARG_EVALUATE) == FAIL)
	retval = 0;
    else
    {
	// If the result is a number, just return the number.
	if (tv.v_type == VAR_NUMBER)
	    retval = tv.vval.v_number;
	else if (tv.v_type != VAR_STRING || tv.vval.v_string == NULL)
	    retval = 0;
	else
	{
	    // If the result is a string, check if there is a non-digit before
	    // the number.
	    s = tv.vval.v_string;
	    if (*s != NUL && !VIM_ISDIGIT(*s) && *s != '-')
		*cp = *s++;
	    retval = atol((char *)s);
	}
	clear_tv(&tv);
    }
    --emsg_off;
    if (use_sandbox)
	--sandbox;
    --textlock;
    clear_evalarg(&EVALARG_EVALUATE, NULL);
    current_sctx = saved_sctx;

    return (int)retval;
}
#endif

#ifdef LOG_LOCKVAR
typedef struct flag_string_S
{
    int	    flag;
    char    *str;
} flag_string_T;

    static char *
flags_tostring(int flags, flag_string_T *_fstring, char *buf, size_t n)
{
    char *p = buf;
    *p = NUL;
    for (flag_string_T *fstring = _fstring; fstring->flag; ++fstring)
    {
	if ((fstring->flag & flags) != 0)
	{
	    size_t len = STRLEN(fstring->str);
	    if (n > p - buf + len + 7)
	    {
		STRCAT(p, fstring->str);
		p += len;
		STRCAT(p, " ");
		++p;
	    }
	    else
	    {
		STRCAT(buf, "...");
		break;
	    }
	}
    }
    return buf;
}

flag_string_T glv_flag_strings[] = {
    { GLV_QUIET,		"QUIET" },
    { GLV_NO_AUTOLOAD,		"NO_AUTOLOAD" },
    { GLV_READ_ONLY,		"READ_ONLY" },
    { GLV_NO_DECL,		"NO_DECL" },
    { GLV_COMPILING,		"COMPILING" },
    { GLV_ASSIGN_WITH_OP,	"ASSIGN_WITH_OP" },
    { GLV_PREFER_FUNC,		"PREFER_FUNC" },
    { 0,			NULL }
};
#endif

/*
 * Fill in "lp" using "root". This is used in a special case when
 * "get_lval()" parses a bare word when "lval_root" is not NULL.
 *
 * This is typically called with "lval_root" as "root". For a class, find
 * the name from lp in the class from root, fill in lval_T if found. For a
 * complex type, list/dict use it as the result; just put the root into
 * ll_tv.
 *
 * "lval_root" is a hack used during run-time/instr-execution to provide the
 * starting point for "get_lval()" to traverse a chain of indexes. In some
 * cases get_lval sees a bare name and uses this function to populate the
 * lval_T.
 *
 * For setting up "lval_root" (currently only used with lockvar)
 *	compile_lock_unlock - pushes object on stack (which becomes lval_root)
 *	execute_instructions: ISN_LOCKUNLOCK - sets lval_root from stack.
 */
    static void
fill_lval_from_lval_root(lval_T *lp, lval_root_T *lr)
{
#ifdef LOG_LOCKVAR
    ch_log(NULL, "LKVAR: fill_lval_from_lval_root(): name %s, tv %p",
						lp->ll_name, (void*)lr->lr_tv);
#endif
    if (lr->lr_tv == NULL)
	return;
    if (!lr->lr_is_arg && lr->lr_tv->v_type == VAR_CLASS)
    {
	if (lr->lr_tv->vval.v_class != NULL)
	{
	    // Special special case. Look for a bare class variable reference.
	    class_T	*cl = lr->lr_tv->vval.v_class;
	    int		m_idx;
	    ocmember_T	*m = class_member_lookup(cl, lp->ll_name,
					lp->ll_name_end - lp->ll_name, &m_idx);
	    if (m != NULL)
	    {
		// Assuming "inside class" since bare reference.
		lp->ll_class = lr->lr_tv->vval.v_class;
		lp->ll_oi = m_idx;
		lp->ll_valtype = m->ocm_type;
		lp->ll_tv = &lp->ll_class->class_members_tv[m_idx];
#ifdef LOG_LOCKVAR
		ch_log(NULL, "LKVAR:    ... class member %s.%s",
					lp->ll_class->class_name, lp->ll_name);
#endif
		return;
	    }
	}
    }

#ifdef LOG_LOCKVAR
    ch_log(NULL, "LKVAR:    ... type: %s", vartype_name(lr->lr_tv->v_type));
#endif
    lp->ll_tv = lr->lr_tv;
    lp->ll_is_root = TRUE;
}

/*
 * Check if the class has permission to access the member.
 * Returns OK or FAIL.
 */
    static int
get_lval_check_access(
    class_T	*cl_exec,   // executing class, NULL if :def or script level
    class_T	*cl,	    // class which contains the member
    ocmember_T	*om,	    // member being accessed
    char_u	*p,	    // char after member name
    int		flags)	    // GLV flags to check if writing to lval
{
#ifdef LOG_LOCKVAR
    ch_log(NULL, "LKVAR: get_lval_check_access(), cl_exec %p, cl %p, %c",
						(void*)cl_exec, (void*)cl, *p);
#endif
    if (cl_exec == NULL || cl_exec != cl)
    {
	char *msg = NULL;
	switch (om->ocm_access)
	{
	    case VIM_ACCESS_PRIVATE:
		msg = e_cannot_access_protected_variable_str;
		break;
	    case VIM_ACCESS_READ:
		// If [idx] or .key following, read only OK.
		if (*p == '[' || *p == '.')
		    break;
		if ((flags & GLV_READ_ONLY) == 0)
		    msg = e_variable_is_not_writable_str;
		break;
	    case VIM_ACCESS_ALL:
		break;
	}
	if (msg != NULL)
	{
	    emsg_var_cl_define(msg, om->ocm_name, 0, cl);
	    return FAIL;
	}

    }
    return OK;
}

/*
 * Get an lval: variable, Dict item or List item that can be assigned a value
 * to: "name", "na{me}", "name[expr]", "name[expr:expr]", "name[expr][expr]",
 * "name.key", "name.key[expr]" etc.
 * Indexing only works if "name" is an existing List or Dictionary.
 * "name" points to the start of the name.
 * If "rettv" is not NULL it points to the value to be assigned.
 * "unlet" is TRUE for ":unlet": slightly different behavior when something is
 * wrong; must end in space or cmd separator.
 *
 * flags:
 *  GLV_QUIET:       do not give error messages
 *  GLV_READ_ONLY:   will not change the variable
 *  GLV_NO_AUTOLOAD: do not use script autoloading
 *
 * Returns a pointer to just after the name, including indexes.
 * When an evaluation error occurs "lp->ll_name" is NULL;
 * Returns NULL for a parsing error.  Still need to free items in "lp"!
 */
    char_u *
get_lval(
    char_u	*name,
    typval_T	*rettv,
    lval_T	*lp,
    int		unlet,
    int		skip,
    int		flags,	    // GLV_ values
    int		fne_flags)  // flags for find_name_end()
{
    char_u	*p;
    char_u	*expr_start, *expr_end;
    int		cc;
    dictitem_T	*v;
    typval_T	var1;
    typval_T	var2;
    int		empty1 = FALSE;
    char_u	*key = NULL;
    int		len;
    hashtab_T	*ht = NULL;
    int		quiet = flags & GLV_QUIET;
    int		writing = 0;
    int		vim9script = in_vim9script();
    class_T	*cl_exec = NULL;    // class that is executing, or NULL.

#ifdef LOG_LOCKVAR
    if (lval_root == NULL)
	ch_log(NULL, "LKVAR: get_lval(): name: %s, lval_root (nil)", name);
    else
	ch_log(NULL, "LKVAR: get_lval(): name: %s, lr_tv %p lr_is_arg %d",
			name, (void*)lval_root->lr_tv, lval_root->lr_is_arg);
    char buf[80];
    ch_log(NULL, "LKVAR:    ...: GLV flags: %s",
		    flags_tostring(flags, glv_flag_strings, buf, sizeof(buf)));
#endif

    // Clear everything in "lp".
    CLEAR_POINTER(lp);

    if (skip || (flags & GLV_COMPILING))
    {
	// When skipping or compiling just find the end of the name.
	lp->ll_name = name;
	lp->ll_name_end = find_name_end(name, NULL, NULL,
						      FNE_INCL_BR | fne_flags);
	return lp->ll_name_end;
    }

    // Cannot use "s:var" at the Vim9 script level.  "s: type" is OK.
    if (vim9script && at_script_level()
		  && name[0] == 's' && name[1] == ':' && !VIM_ISWHITE(name[2]))
    {
	semsg(_(e_cannot_use_s_colon_in_vim9_script_str), name);
	return NULL;
    }

    // Find the end of the name.
    p = find_name_end(name, &expr_start, &expr_end, fne_flags);
    lp->ll_name_end = p;
    if (expr_start != NULL)
    {
	// Don't expand the name when we already know there is an error.
	if (unlet && !VIM_ISWHITE(*p) && !ends_excmd(*p)
						    && *p != '[' && *p != '.')
	{
	    semsg(_(e_trailing_characters_str), p);
	    return NULL;
	}

	lp->ll_exp_name = make_expanded_name(name, expr_start, expr_end, p);
	if (lp->ll_exp_name == NULL)
	{
	    // Report an invalid expression in braces, unless the
	    // expression evaluation has been cancelled due to an
	    // aborting error, an interrupt, or an exception.
	    if (!aborting() && !quiet)
	    {
		emsg_severe = TRUE;
		semsg(_(e_invalid_argument_str), name);
		return NULL;
	    }
	}
	lp->ll_name = lp->ll_exp_name;
    }
    else
    {
	lp->ll_name = name;

	if (vim9script)
	{
	    // "a: type" is declaring variable "a" with a type, not "a:".
	    // However, "g:[key]" is indexing a dictionary.
	    if (p == name + 2 && p[-1] == ':' && *p != '[')
	    {
		--p;
		lp->ll_name_end = p;
	    }
	    if (*skipwhite(p) == ':')
	    {
		char_u	    *tp = skipwhite(p + 1);

		if (is_scoped_variable(name))
		{
		    semsg(_(e_cannot_use_type_with_this_variable_str), name);
		    return NULL;
		}
		if (VIM_ISWHITE(*p))
		{
		    semsg(_(e_no_white_space_allowed_before_colon_str), p);
		    return NULL;
		}
		if (tp == p + 1 && !quiet)
		{
		    semsg(_(e_white_space_required_after_str_str), ":", p);
		    return NULL;
		}
		if (!SCRIPT_ID_VALID(current_sctx.sc_sid))
		{
		    semsg(_(e_using_type_not_in_script_context_str), p);
		    return NULL;
		}
		if (vim9script && (flags & GLV_NO_DECL) &&
			!(flags & GLV_FOR_LOOP))
		{
		    // Using a type and not in a "var" declaration.
		    semsg(_(e_trailing_characters_str), p);
		    return NULL;
		}


		// parse the type after the name
		lp->ll_type = parse_type(&tp,
			       &SCRIPT_ITEM(current_sctx.sc_sid)->sn_type_list,
			       !quiet);
		if (lp->ll_type == NULL && !quiet)
		    return NULL;
		lp->ll_name_end = tp;
	    }
	    // TODO: check inside class?
	}
    }
    if (lp->ll_name == NULL)
	return p;

    if (*p == '.')
    {
	imported_T *import = find_imported(lp->ll_name, p - lp->ll_name, TRUE);

	if (import != NULL)
	{
	    ufunc_T *ufunc;
	    type_T *type;

	    import_check_sourced_sid(&import->imp_sid);
	    lp->ll_sid = import->imp_sid;
	    lp->ll_name = skipwhite(p + 1);
	    p = find_name_end(lp->ll_name, NULL, NULL, fne_flags);
	    lp->ll_name_end = p;

	    // check the item is exported
	    cc = *p;
	    *p = NUL;
	    if (find_exported(import->imp_sid, lp->ll_name, &ufunc, &type,
						       NULL, NULL, TRUE) == -1)
	    {
		*p = cc;
		return NULL;
	    }
	    *p = cc;
	}
    }

    // Without [idx] or .key we are done.
    if (*p != '[' && *p != '.')
    {
	if (lval_root != NULL)
	    fill_lval_from_lval_root(lp, lval_root);
	return p;
    }

    if (vim9script && lval_root != NULL)
	cl_exec = lval_root->lr_cl_exec;
    if (vim9script && lval_root != NULL && lval_root->lr_tv != NULL)
    {
	// using local variable
	lp->ll_tv = lval_root->lr_tv;
	v = NULL;
    }
    else
    {
	cc = *p;
	*p = NUL;
	// When we would write to the variable pass &ht and prevent autoload.
	writing = !(flags & GLV_READ_ONLY);
	v = find_var(lp->ll_name, writing ? &ht : NULL,
					 (flags & GLV_NO_AUTOLOAD) || writing);
	if (v == NULL && !quiet)
	    semsg(_(e_undefined_variable_str), lp->ll_name);
	*p = cc;
	if (v == NULL)
	    return NULL;
	lp->ll_tv = &v->di_tv;
    }

    if (vim9script && (flags & GLV_NO_DECL) == 0)
    {
	if (!quiet)
	    semsg(_(e_variable_already_declared_str), lp->ll_name);
	return NULL;
    }

    /*
     * Loop until no more [idx] or .key is following.
     */
    var1.v_type = VAR_UNKNOWN;
    var2.v_type = VAR_UNKNOWN;
    while (*p == '[' || (*p == '.' && p[1] != '=' && p[1] != '.'))
    {
	vartype_T v_type = lp->ll_tv->v_type;

	if (*p == '.' && v_type != VAR_DICT
		      && v_type != VAR_OBJECT
		      && v_type != VAR_CLASS)
	{
	    if (!quiet)
		semsg(_(e_dot_not_allowed_after_str_str),
						vartype_name(v_type), name);
	    return NULL;
	}
	if (v_type != VAR_LIST
		&& v_type != VAR_DICT
		&& v_type != VAR_BLOB
		&& v_type != VAR_OBJECT
		&& v_type != VAR_CLASS)
	{
	    if (!quiet)
		semsg(_(e_index_not_allowed_after_str_str),
						vartype_name(v_type), name);
	    return NULL;
	}

	// A NULL list/blob works like an empty list/blob, allocate one now.
	int r = OK;
	if (v_type == VAR_LIST && lp->ll_tv->vval.v_list == NULL)
	    r = rettv_list_alloc(lp->ll_tv);
	else if (v_type == VAR_BLOB && lp->ll_tv->vval.v_blob == NULL)
	    r = rettv_blob_alloc(lp->ll_tv);
	if (r == FAIL)
	    return NULL;

	if (lp->ll_range)
	{
	    if (!quiet)
		emsg(_(e_slice_must_come_last));
	    return NULL;
	}
#ifdef LOG_LOCKVAR
	ch_log(NULL, "LKVAR: get_lval() loop: p: %s, type: %s", p,
							vartype_name(v_type));
#endif

	if (vim9script && lp->ll_valtype == NULL
		&& v != NULL
		&& lp->ll_tv == &v->di_tv
		&& ht != NULL && ht == get_script_local_ht())
	{
	    svar_T  *sv = find_typval_in_script(lp->ll_tv, 0, TRUE);

	    // Vim9 script local variable: get the type
	    if (sv != NULL)
	    {
		lp->ll_valtype = sv->sv_type;
#ifdef LOG_LOCKVAR
		ch_log(NULL, "LKVAR:    ... loop: vim9 assign type: %s",
					vartype_name(lp->ll_valtype->tt_type));
#endif
	    }
	}

	len = -1;
	if (*p == '.')
	{
	    key = p + 1;
	    for (len = 0; ASCII_ISALNUM(key[len]) || key[len] == '_'; ++len)
		;
	    if (len == 0)
	    {
		if (!quiet)
		    emsg(_(e_cannot_use_empty_key_for_dictionary));
		return NULL;
	    }
	    p = key + len;
	}
	else
	{
	    // Get the index [expr] or the first index [expr: ].
	    p = skipwhite(p + 1);
	    if (*p == ':')
		empty1 = TRUE;
	    else
	    {
		empty1 = FALSE;
		if (eval1(&p, &var1, &EVALARG_EVALUATE) == FAIL)  // recursive!
		    return NULL;
		if (tv_get_string_chk(&var1) == NULL)
		{
		    // not a number or string
		    clear_tv(&var1);
		    return NULL;
		}
		p = skipwhite(p);
	    }

	    // Optionally get the second index [ :expr].
	    if (*p == ':')
	    {
		if (v_type == VAR_DICT)
		{
		    if (!quiet)
			emsg(_(e_cannot_slice_dictionary));
		    clear_tv(&var1);
		    return NULL;
		}
		if (rettv != NULL
			&& !(rettv->v_type == VAR_LIST
						 && rettv->vval.v_list != NULL)
			&& !(rettv->v_type == VAR_BLOB
						&& rettv->vval.v_blob != NULL))
		{
		    if (!quiet)
			emsg(_(e_slice_requires_list_or_blob_value));
		    clear_tv(&var1);
		    return NULL;
		}
		p = skipwhite(p + 1);
		if (*p == ']')
		    lp->ll_empty2 = TRUE;
		else
		{
		    lp->ll_empty2 = FALSE;
		    // recursive!
		    if (eval1(&p, &var2, &EVALARG_EVALUATE) == FAIL)
		    {
			clear_tv(&var1);
			return NULL;
		    }
		    if (tv_get_string_chk(&var2) == NULL)
		    {
			// not a number or string
			clear_tv(&var1);
			clear_tv(&var2);
			return NULL;
		    }
		}
		lp->ll_range = TRUE;
	    }
	    else
		lp->ll_range = FALSE;

	    if (*p != ']')
	    {
		if (!quiet)
		    emsg(_(e_missing_closing_square_brace));
		clear_tv(&var1);
		clear_tv(&var2);
		return NULL;
	    }

	    // Skip to past ']'.
	    ++p;
	}
#ifdef LOG_LOCKVAR
	if (len == -1)
	    ch_log(NULL, "LKVAR:    ... loop: p: %s, '[' key: %s", p,
				empty1 ? ":" : (char*)tv_get_string(&var1));
	else
	    ch_log(NULL, "LKVAR:    ... loop: p: %s, '.' key: %s", p, key);
#endif

	if (v_type == VAR_DICT)
	{
	    if (len == -1)
	    {
		// "[key]": get key from "var1"
		key = tv_get_string_chk(&var1);	// is number or string
		if (key == NULL)
		{
		    clear_tv(&var1);
		    return NULL;
		}
	    }
	    lp->ll_list = NULL;
	    lp->ll_object = NULL;
	    lp->ll_class = NULL;

	    // a NULL dict is equivalent with an empty dict
	    if (lp->ll_tv->vval.v_dict == NULL)
	    {
		lp->ll_tv->vval.v_dict = dict_alloc();
		if (lp->ll_tv->vval.v_dict == NULL)
		{
		    clear_tv(&var1);
		    return NULL;
		}
		++lp->ll_tv->vval.v_dict->dv_refcount;
	    }
	    lp->ll_dict = lp->ll_tv->vval.v_dict;

	    lp->ll_di = dict_find(lp->ll_dict, key, len);

	    // When assigning to a scope dictionary check that a function and
	    // variable name is valid (only variable name unless it is l: or
	    // g: dictionary). Disallow overwriting a builtin function.
	    if (rettv != NULL && lp->ll_dict->dv_scope != 0)
	    {
		int prevval;

		if (len != -1)
		{
		    prevval = key[len];
		    key[len] = NUL;
		}
		else
		    prevval = 0; // avoid compiler warning
		int wrong = (lp->ll_dict->dv_scope == VAR_DEF_SCOPE
			       && (rettv->v_type == VAR_FUNC
					    || rettv->v_type == VAR_PARTIAL)
			       && var_wrong_func_name(key, lp->ll_di == NULL))
			|| !valid_varname(key, -1, TRUE);
		if (len != -1)
		    key[len] = prevval;
		if (wrong)
		{
		    clear_tv(&var1);
		    return NULL;
		}
	    }

	    if (lp->ll_valtype != NULL)
		// use the type of the member
		lp->ll_valtype = lp->ll_valtype->tt_member;

	    if (lp->ll_di == NULL)
	    {
		// Can't add "v:" or "a:" variable.
		if (lp->ll_dict == get_vimvar_dict()
			 || &lp->ll_dict->dv_hashtab == get_funccal_args_ht())
		{
		    semsg(_(e_illegal_variable_name_str), name);
		    clear_tv(&var1);
		    return NULL;
		}

		// Key does not exist in dict: may need to add it.
		if (*p == '[' || *p == '.' || unlet)
		{
		    if (!quiet)
			semsg(_(e_key_not_present_in_dictionary_str), key);
		    clear_tv(&var1);
		    return NULL;
		}
		if (len == -1)
		    lp->ll_newkey = vim_strsave(key);
		else
		    lp->ll_newkey = vim_strnsave(key, len);
		clear_tv(&var1);
		if (lp->ll_newkey == NULL)
		    p = NULL;
		break;
	    }
	    // existing variable, need to check if it can be changed
	    else if ((flags & GLV_READ_ONLY) == 0
			&& (var_check_ro(lp->ll_di->di_flags, name, FALSE)
			  || var_check_lock(lp->ll_di->di_flags, name, FALSE)))
	    {
		clear_tv(&var1);
		return NULL;
	    }

	    clear_tv(&var1);
	    lp->ll_tv = &lp->ll_di->di_tv;
	}
	else if (v_type == VAR_BLOB)
	{
	    long bloblen = blob_len(lp->ll_tv->vval.v_blob);

	    /*
	     * Get the number and item for the only or first index of the List.
	     */
	    if (empty1)
		lp->ll_n1 = 0;
	    else
		// is number or string
		lp->ll_n1 = (long)tv_get_number(&var1);
	    clear_tv(&var1);

	    if (check_blob_index(bloblen, lp->ll_n1, quiet) == FAIL)
	    {
		clear_tv(&var2);
		return NULL;
	    }
	    if (lp->ll_range && !lp->ll_empty2)
	    {
		lp->ll_n2 = (long)tv_get_number(&var2);
		clear_tv(&var2);
		if (check_blob_range(bloblen, lp->ll_n1, lp->ll_n2, quiet)
								       == FAIL)
		    return NULL;
	    }
	    lp->ll_blob = lp->ll_tv->vval.v_blob;
	    lp->ll_tv = NULL;
	    break;
	}
	else if (v_type == VAR_LIST)
	{
	    /*
	     * Get the number and item for the only or first index of the List.
	     */
	    if (empty1)
		lp->ll_n1 = 0;
	    else
		// is number or string
		lp->ll_n1 = (long)tv_get_number(&var1);
	    clear_tv(&var1);

	    lp->ll_dict = NULL;
	    lp->ll_object = NULL;
	    lp->ll_class = NULL;
	    lp->ll_list = lp->ll_tv->vval.v_list;
	    lp->ll_li = check_range_index_one(lp->ll_list, &lp->ll_n1,
				     (flags & GLV_ASSIGN_WITH_OP) == 0, quiet);
	    if (lp->ll_li == NULL)
	    {
		clear_tv(&var2);
		return NULL;
	    }

	    if (lp->ll_valtype != NULL)
		// use the type of the member
		lp->ll_valtype = lp->ll_valtype->tt_member;

	    /*
	     * May need to find the item or absolute index for the second
	     * index of a range.
	     * When no index given: "lp->ll_empty2" is TRUE.
	     * Otherwise "lp->ll_n2" is set to the second index.
	     */
	    if (lp->ll_range && !lp->ll_empty2)
	    {
		lp->ll_n2 = (long)tv_get_number(&var2);
						    // is number or string
		clear_tv(&var2);
		if (check_range_index_two(lp->ll_list,
					    &lp->ll_n1, lp->ll_li,
					    &lp->ll_n2, quiet) == FAIL)
		    return NULL;
	    }

	    lp->ll_tv = &lp->ll_li->li_tv;
	}
	else  // v_type == VAR_CLASS || v_type == VAR_OBJECT
	{
	    lp->ll_dict = NULL;
	    lp->ll_list = NULL;

	    class_T *cl;
	    if (v_type == VAR_OBJECT)
	    {
		if (lp->ll_tv->vval.v_object == NULL)
		{
		    if (!quiet)
			emsg(_(e_using_null_object));
		    return NULL;
		}
	        cl = lp->ll_tv->vval.v_object->obj_class;
	        lp->ll_object = lp->ll_tv->vval.v_object;
	    }
	    else
	    {
	        cl = lp->ll_tv->vval.v_class;
	        lp->ll_object = NULL;
	    }
	    lp->ll_class = cl;

	    // TODO: what if class is NULL?
	    if (cl != NULL)
	    {
		lp->ll_valtype = NULL;

		if (flags & GLV_PREFER_FUNC)
		{
		    // First look for a function with this name.
		    // round 1: class functions (skipped for an object)
		    // round 2: object methods
		    for (int round = v_type == VAR_OBJECT ? 2 : 1;
							round <= 2; ++round)
		    {
			int	m_idx;
			ufunc_T	*fp;

			fp = method_lookup(cl,
				round == 1 ? VAR_CLASS : VAR_OBJECT,
				key, p - key, &m_idx);
			lp->ll_oi = m_idx;
			if (fp != NULL)
			{
			    lp->ll_ufunc = fp;
			    lp->ll_valtype = fp->uf_func_type;
			    break;
			}
		    }
		}

		if (lp->ll_valtype == NULL)
		{
		    int		m_idx;
		    ocmember_T	*om
			    = member_lookup(cl, v_type, key, p - key, &m_idx);
		    lp->ll_oi = m_idx;
		    if (om != NULL)
		    {
			if (get_lval_check_access(cl_exec, cl, om,
							  p, flags) == FAIL)
			    return NULL;

			// When lhs is used to modify the variable, check it is
			// not a read-only variable.
			if ((flags & GLV_READ_ONLY) == 0
				&& (*p != '.' && *p != '[')
				&& oc_var_check_ro(cl, om))
			    return NULL;

			lp->ll_valtype = om->ocm_type;

			if (v_type == VAR_OBJECT)
			    lp->ll_tv = ((typval_T *)(
					lp->ll_tv->vval.v_object + 1)) + m_idx;
			else
			    lp->ll_tv = &cl->class_members_tv[m_idx];
		    }
		}

		if (lp->ll_valtype == NULL)
		{
		    member_not_found_msg(cl, v_type, key, p - key);
		    return NULL;
		}
	    }
	}
    }

    clear_tv(&var1);
    lp->ll_name_end = p;
    return p;
}

/*
 * Clear lval "lp" that was filled by get_lval().
 */
    void
clear_lval(lval_T *lp)
{
    vim_free(lp->ll_exp_name);
    vim_free(lp->ll_newkey);
}

/*
 * Set a variable that was parsed by get_lval() to "rettv".
 * "endp" points to just after the parsed name.
 * "op" is NULL, "+" for "+=", "-" for "-=", "*" for "*=", "/" for "/=",
 * "%" for "%=", "." for ".=" or "=" for "=".
 */
    void
set_var_lval(
    lval_T	*lp,
    char_u	*endp,
    typval_T	*rettv,
    int		copy,
    int		flags,	    // ASSIGN_CONST, ASSIGN_NO_DECL
    char_u	*op,
    int		var_idx)    // index for "let [a, b] = list"
{
    int		cc;
    dictitem_T	*di;

    if (lp->ll_tv == NULL)
    {
	cc = *endp;
	*endp = NUL;
	if (in_vim9script() && check_reserved_name(lp->ll_name, FALSE) == FAIL)
	    return;

	if (lp->ll_blob != NULL)
	{
	    int	    error = FALSE, val;

	    if (op != NULL && *op != '=')
	    {
		semsg(_(e_wrong_variable_type_for_str_equal), op);
		return;
	    }
	    if (value_check_lock(lp->ll_blob->bv_lock, lp->ll_name, FALSE))
		return;

	    if (lp->ll_range && rettv->v_type == VAR_BLOB)
	    {
		if (lp->ll_empty2)
		    lp->ll_n2 = blob_len(lp->ll_blob) - 1;

		if (blob_set_range(lp->ll_blob, lp->ll_n1, lp->ll_n2,
								rettv) == FAIL)
		    return;
	    }
	    else
	    {
		val = (int)tv_get_number_chk(rettv, &error);
		if (!error)
		    blob_set_append(lp->ll_blob, lp->ll_n1, val);
	    }
	}
	else if (op != NULL && *op != '=')
	{
	    typval_T tv;

	    if ((flags & (ASSIGN_CONST | ASSIGN_FINAL))
					     && (flags & ASSIGN_FOR_LOOP) == 0)
	    {
		emsg(_(e_cannot_modify_existing_variable));
		*endp = cc;
		return;
	    }

	    // handle +=, -=, *=, /=, %= and .=
	    di = NULL;
	    if (eval_variable(lp->ll_name, (int)STRLEN(lp->ll_name),
				 lp->ll_sid, &tv, &di, EVAL_VAR_VERBOSE) == OK)
	    {
		if (di != NULL && check_typval_is_value(&di->di_tv) == FAIL)
		{
		    clear_tv(&tv);
		    return;
		}

		if ((di == NULL
			 || (!var_check_ro(di->di_flags, lp->ll_name, FALSE)
			   && !tv_check_lock(&di->di_tv, lp->ll_name, FALSE)))
			&& tv_op(&tv, rettv, op) == OK)
		    set_var_const(lp->ll_name, lp->ll_sid, NULL, &tv, FALSE,
							    ASSIGN_NO_DECL, 0);
		clear_tv(&tv);
	    }
	}
	else
	{
	    if (lp->ll_type != NULL && check_typval_arg_type(lp->ll_type, rettv,
							      NULL, 0) == FAIL)
		return;
	    set_var_const(lp->ll_name, lp->ll_sid, lp->ll_type, rettv, copy,
							       flags, var_idx);
	}
	*endp = cc;
    }
    else if (value_check_lock(lp->ll_newkey == NULL
		? lp->ll_tv->v_lock
		: lp->ll_tv->vval.v_dict->dv_lock, lp->ll_name, FALSE))
	;
    else if (lp->ll_range)
    {
	if ((flags & (ASSIGN_CONST | ASSIGN_FINAL))
					     && (flags & ASSIGN_FOR_LOOP) == 0)
	{
	    emsg(_(e_cannot_lock_range));
	    return;
	}

	(void)list_assign_range(lp->ll_list, rettv->vval.v_list,
			 lp->ll_n1, lp->ll_n2, lp->ll_empty2, op, lp->ll_name);
    }
    else
    {
	/*
	 * Assign to a List, Dictionary or Object item.
	 */
	if ((flags & (ASSIGN_CONST | ASSIGN_FINAL))
					     && (flags & ASSIGN_FOR_LOOP) == 0)
	{
	    emsg(_(e_cannot_lock_list_or_dict));
	    return;
	}

	if (lp->ll_valtype != NULL
		    && check_typval_arg_type(lp->ll_valtype, rettv,
							      NULL, 0) == FAIL)
	    return;

	if (lp->ll_newkey != NULL)
	{
	    if (op != NULL && *op != '=')
	    {
		semsg(_(e_key_not_present_in_dictionary_str), lp->ll_newkey);
		return;
	    }
	    if (dict_wrong_func_name(lp->ll_tv->vval.v_dict, rettv,
								lp->ll_newkey))
		return;

	    // Need to add an item to the Dictionary.
	    di = dictitem_alloc(lp->ll_newkey);
	    if (di == NULL)
		return;
	    if (dict_add(lp->ll_tv->vval.v_dict, di) == FAIL)
	    {
		vim_free(di);
		return;
	    }
	    lp->ll_tv = &di->di_tv;
	}
	else if (op != NULL && *op != '=')
	{
	    tv_op(lp->ll_tv, rettv, op);
	    return;
	}
	else
	    clear_tv(lp->ll_tv);

	/*
	 * Assign the value to the variable or list item.
	 */
	if (copy)
	    copy_tv(rettv, lp->ll_tv);
	else
	{
	    *lp->ll_tv = *rettv;
	    lp->ll_tv->v_lock = 0;
	    init_tv(rettv);
	}
    }
}

/*
 * Handle "tv1 += tv2", "tv1 -= tv2", "tv1 *= tv2", "tv1 /= tv2", "tv1 %= tv2"
 * and "tv1 .= tv2"
 * Returns OK or FAIL.
 */
    int
tv_op(typval_T *tv1, typval_T *tv2, char_u *op)
{
    varnumber_T	n;
    char_u	numbuf[NUMBUFLEN];
    char_u	*s;
    int		failed = FALSE;

    // Can't do anything with a Funcref or Dict or Type on the right.
    // v:true and friends only work with "..=".
    if (tv2->v_type != VAR_FUNC && tv2->v_type != VAR_DICT
		    && tv2->v_type != VAR_CLASS && tv2->v_type != VAR_TYPEALIAS
		    && ((tv2->v_type != VAR_BOOL && tv2->v_type != VAR_SPECIAL)
								|| *op == '.'))
    {
	switch (tv1->v_type)
	{
	    case VAR_UNKNOWN:
	    case VAR_ANY:
	    case VAR_VOID:
	    case VAR_DICT:
	    case VAR_FUNC:
	    case VAR_PARTIAL:
	    case VAR_BOOL:
	    case VAR_SPECIAL:
	    case VAR_JOB:
	    case VAR_CHANNEL:
	    case VAR_INSTR:
	    case VAR_OBJECT:
		break;
	    case VAR_CLASS:
	    case VAR_TYPEALIAS:
		check_typval_is_value(tv1);
		return FAIL;

	    case VAR_BLOB:
		if (*op != '+' || tv2->v_type != VAR_BLOB)
		    break;
		// BLOB += BLOB
		if (tv1->vval.v_blob != NULL && tv2->vval.v_blob != NULL)
		{
		    blob_T  *b1 = tv1->vval.v_blob;
		    blob_T  *b2 = tv2->vval.v_blob;
		    int	i, len = blob_len(b2);
		    for (i = 0; i < len; i++)
			ga_append(&b1->bv_ga, blob_get(b2, i));
		}
		return OK;

	    case VAR_LIST:
		if (*op != '+' || tv2->v_type != VAR_LIST)
		    break;
		// List += List
		if (tv2->vval.v_list != NULL)
		{
		    if (tv1->vval.v_list == NULL)
		    {
			tv1->vval.v_list = tv2->vval.v_list;
			++tv1->vval.v_list->lv_refcount;
		    }
		    else
			list_extend(tv1->vval.v_list, tv2->vval.v_list, NULL);
		}
		return OK;

	    case VAR_NUMBER:
	    case VAR_STRING:
		if (tv2->v_type == VAR_LIST)
		    break;
		if (vim_strchr((char_u *)"+-*/%", *op) != NULL)
		{
		    // nr += nr , nr -= nr , nr *=nr , nr /= nr , nr %= nr
		    n = tv_get_number(tv1);
		    if (tv2->v_type == VAR_FLOAT)
		    {
			float_T f = n;

			if (*op == '%')
			    break;
			switch (*op)
			{
			    case '+': f += tv2->vval.v_float; break;
			    case '-': f -= tv2->vval.v_float; break;
			    case '*': f *= tv2->vval.v_float; break;
			    case '/': f /= tv2->vval.v_float; break;
			}
			clear_tv(tv1);
			tv1->v_type = VAR_FLOAT;
			tv1->vval.v_float = f;
		    }
		    else
		    {
			switch (*op)
			{
			    case '+': n += tv_get_number(tv2); break;
			    case '-': n -= tv_get_number(tv2); break;
			    case '*': n *= tv_get_number(tv2); break;
			    case '/': n = num_divide(n, tv_get_number(tv2),
							       &failed); break;
			    case '%': n = num_modulus(n, tv_get_number(tv2),
							       &failed); break;
			}
			clear_tv(tv1);
			tv1->v_type = VAR_NUMBER;
			tv1->vval.v_number = n;
		    }
		}
		else
		{
		    if (tv2->v_type == VAR_FLOAT)
			break;

		    // str .= str
		    s = tv_get_string(tv1);
		    s = concat_str(s, tv_get_string_buf(tv2, numbuf));
		    clear_tv(tv1);
		    tv1->v_type = VAR_STRING;
		    tv1->vval.v_string = s;
		}
		return failed ? FAIL : OK;

	    case VAR_FLOAT:
		{
		    float_T f;

		    if (*op == '%' || *op == '.'
				   || (tv2->v_type != VAR_FLOAT
				    && tv2->v_type != VAR_NUMBER
				    && tv2->v_type != VAR_STRING))
			break;
		    if (tv2->v_type == VAR_FLOAT)
			f = tv2->vval.v_float;
		    else
			f = tv_get_number(tv2);
		    switch (*op)
		    {
			case '+': tv1->vval.v_float += f; break;
			case '-': tv1->vval.v_float -= f; break;
			case '*': tv1->vval.v_float *= f; break;
			case '/': tv1->vval.v_float /= f; break;
		    }
		}
		return OK;
	}
    }

    if (check_typval_is_value(tv2) == OK)
	semsg(_(e_wrong_variable_type_for_str_equal), op);
    return FAIL;
}

/*
 * Evaluate the expression used in a ":for var in expr" command.
 * "arg" points to "var".
 * Set "*errp" to TRUE for an error, FALSE otherwise;
 * Return a pointer that holds the info.  Null when there is an error.
 */
    void *
eval_for_line(
    char_u	*arg,
    int		*errp,
    exarg_T	*eap,
    evalarg_T	*evalarg)
{
    forinfo_T	*fi;
    char_u	*var_list_end;
    char_u	*expr;
    typval_T	tv;
    list_T	*l;
    int		skip = !(evalarg->eval_flags & EVAL_EVALUATE);

    *errp = TRUE;	// default: there is an error

    fi = ALLOC_CLEAR_ONE(forinfo_T);
    if (fi == NULL)
	return NULL;

    var_list_end = skip_var_list(arg, TRUE, &fi->fi_varcount,
						     &fi->fi_semicolon, FALSE);
    if (var_list_end == NULL)
	return fi;

    expr = skipwhite_and_linebreak(var_list_end, evalarg);
    if (expr[0] != 'i' || expr[1] != 'n'
				  || !(expr[2] == NUL || VIM_ISWHITE(expr[2])))
    {
	if (in_vim9script() && *expr == ':' && expr != var_list_end)
	    semsg(_(e_no_white_space_allowed_before_colon_str), expr);
	else
	    emsg(_(e_missing_in_after_for));
	return fi;
    }

    if (skip)
	++emsg_skip;
    expr = skipwhite_and_linebreak(expr + 2, evalarg);
    if (eval0(expr, &tv, eap, evalarg) == OK)
    {
	*errp = FALSE;
	if (!skip)
	{
	    if (tv.v_type == VAR_LIST)
	    {
		l = tv.vval.v_list;
		if (l == NULL)
		{
		    // a null list is like an empty list: do nothing
		    clear_tv(&tv);
		}
		else
		{
		    // Need a real list here.
		    CHECK_LIST_MATERIALIZE(l);

		    // No need to increment the refcount, it's already set for
		    // the list being used in "tv".
		    fi->fi_list = l;
		    list_add_watch(l, &fi->fi_lw);
		    fi->fi_lw.lw_item = l->lv_first;
		}
	    }
	    else if (tv.v_type == VAR_BLOB)
	    {
		fi->fi_bi = 0;
		if (tv.vval.v_blob != NULL)
		{
		    typval_T btv;

		    // Make a copy, so that the iteration still works when the
		    // blob is changed.
		    blob_copy(tv.vval.v_blob, &btv);
		    fi->fi_blob = btv.vval.v_blob;
		}
		clear_tv(&tv);
	    }
	    else if (tv.v_type == VAR_STRING)
	    {
		fi->fi_byte_idx = 0;
		fi->fi_string = tv.vval.v_string;
		tv.vval.v_string = NULL;
		if (fi->fi_string == NULL)
		    fi->fi_string = vim_strsave((char_u *)"");
	    }
	    else
	    {
		emsg(_(e_string_list_or_blob_required));
		clear_tv(&tv);
	    }
	}
    }
    if (skip)
	--emsg_skip;
    fi->fi_break_count = evalarg->eval_break_count;

    return fi;
}

/*
 * Used when looping over a :for line, skip the "in expr" part.
 */
    void
skip_for_lines(void *fi_void, evalarg_T *evalarg)
{
    forinfo_T	*fi = (forinfo_T *)fi_void;
    int		i;

    for (i = 0; i < fi->fi_break_count; ++i)
	eval_next_line(NULL, evalarg);
}

/*
 * Use the first item in a ":for" list.  Advance to the next.
 * Assign the values to the variable (list).  "arg" points to the first one.
 * Return TRUE when a valid item was found, FALSE when at end of list or
 * something wrong.
 */
    int
next_for_item(void *fi_void, char_u *arg)
{
    forinfo_T	*fi = (forinfo_T *)fi_void;
    int		result;
    int		flag = ASSIGN_FOR_LOOP | (in_vim9script()
			 ? (ASSIGN_FINAL
			     // first round: error if variable exists
			     | (fi->fi_bi == 0 ? 0 : ASSIGN_DECL)
			     | ASSIGN_NO_MEMBER_TYPE
			     | ASSIGN_UPDATE_BLOCK_ID)
			 : 0);
    listitem_T	*item;
    int		skip_assign = in_vim9script() && arg[0] == '_'
						      && !eval_isnamec(arg[1]);

    if (fi->fi_blob != NULL)
    {
	typval_T	tv;

	if (fi->fi_bi >= blob_len(fi->fi_blob))
	    return FALSE;
	tv.v_type = VAR_NUMBER;
	tv.v_lock = VAR_FIXED;
	tv.vval.v_number = blob_get(fi->fi_blob, fi->fi_bi);
	++fi->fi_bi;
	if (skip_assign)
	    return TRUE;
	return ex_let_vars(arg, &tv, TRUE, fi->fi_semicolon,
					    fi->fi_varcount, flag, NULL) == OK;
    }

    if (fi->fi_string != NULL)
    {
	typval_T	tv;
	int		len;

	len = mb_ptr2len(fi->fi_string + fi->fi_byte_idx);
	if (len == 0)
	    return FALSE;
	tv.v_type = VAR_STRING;
	tv.v_lock = VAR_FIXED;
	tv.vval.v_string = vim_strnsave(fi->fi_string + fi->fi_byte_idx, len);
	fi->fi_byte_idx += len;
	++fi->fi_bi;
	if (skip_assign)
	    result = TRUE;
	else
	    result = ex_let_vars(arg, &tv, TRUE, fi->fi_semicolon,
					    fi->fi_varcount, flag, NULL) == OK;
	vim_free(tv.vval.v_string);
	return result;
    }

    item = fi->fi_lw.lw_item;
    if (item == NULL)
	result = FALSE;
    else
    {
	fi->fi_lw.lw_item = item->li_next;
	++fi->fi_bi;
	if (skip_assign)
	    result = TRUE;
	else
	    result = (ex_let_vars(arg, &item->li_tv, TRUE, fi->fi_semicolon,
					   fi->fi_varcount, flag, NULL) == OK);
    }
    return result;
}

/*
 * Free the structure used to store info used by ":for".
 */
    void
free_for_info(void *fi_void)
{
    forinfo_T    *fi = (forinfo_T *)fi_void;

    if (fi == NULL)
	return;
    if (fi->fi_list != NULL)
    {
	list_rem_watch(fi->fi_list, &fi->fi_lw);
	list_unref(fi->fi_list);
    }
    else if (fi->fi_blob != NULL)
	blob_unref(fi->fi_blob);
    else
	vim_free(fi->fi_string);
    vim_free(fi);
}

    void
set_context_for_expression(
    expand_T	*xp,
    char_u	*arg,
    cmdidx_T	cmdidx)
{
    int		has_expr = cmdidx != CMD_let && cmdidx != CMD_var;
    int		c;
    char_u	*p;

    if (cmdidx == CMD_let || cmdidx == CMD_var
				 || cmdidx == CMD_const || cmdidx == CMD_final)
    {
	xp->xp_context = EXPAND_USER_VARS;
	if (vim_strpbrk(arg, (char_u *)"\"'+-*/%.=!?~|&$([<>,#") == NULL)
	{
	    // ":let var1 var2 ...": find last space.
	    for (p = arg + STRLEN(arg); p >= arg; )
	    {
		xp->xp_pattern = p;
		MB_PTR_BACK(arg, p);
		if (VIM_ISWHITE(*p))
		    break;
	    }
	    return;
	}
    }
    else
	xp->xp_context = cmdidx == CMD_call ? EXPAND_FUNCTIONS
							  : EXPAND_EXPRESSION;
    while ((xp->xp_pattern = vim_strpbrk(arg,
				  (char_u *)"\"'+-*/%.=!?~|&$([<>,#")) != NULL)
    {
	c = *xp->xp_pattern;
	if (c == '&')
	{
	    c = xp->xp_pattern[1];
	    if (c == '&')
	    {
		++xp->xp_pattern;
		xp->xp_context = has_expr ? EXPAND_EXPRESSION : EXPAND_NOTHING;
	    }
	    else if (c != ' ')
	    {
		xp->xp_context = EXPAND_SETTINGS;
		if ((c == 'l' || c == 'g') && xp->xp_pattern[2] == ':')
		    xp->xp_pattern += 2;

	    }
	}
	else if (c == '$')
	{
	    // environment variable
	    xp->xp_context = EXPAND_ENV_VARS;
	}
	else if (c == '=')
	{
	    has_expr = TRUE;
	    xp->xp_context = EXPAND_EXPRESSION;
	}
	else if (c == '#'
		&& xp->xp_context == EXPAND_EXPRESSION)
	{
	    // Autoload function/variable contains '#'.
	    break;
	}
	else if ((c == '<' || c == '#')
		&& xp->xp_context == EXPAND_FUNCTIONS
		&& vim_strchr(xp->xp_pattern, '(') == NULL)
	{
	    // Function name can start with "<SNR>" and contain '#'.
	    break;
	}
	else if (has_expr)
	{
	    if (c == '"')	    // string
	    {
		while ((c = *++xp->xp_pattern) != NUL && c != '"')
		    if (c == '\\' && xp->xp_pattern[1] != NUL)
			++xp->xp_pattern;
		xp->xp_context = EXPAND_NOTHING;
	    }
	    else if (c == '\'')	    // literal string
	    {
		// Trick: '' is like stopping and starting a literal string.
		while ((c = *++xp->xp_pattern) != NUL && c != '\'')
		    /* skip */ ;
		xp->xp_context = EXPAND_NOTHING;
	    }
	    else if (c == '|')
	    {
		if (xp->xp_pattern[1] == '|')
		{
		    ++xp->xp_pattern;
		    xp->xp_context = EXPAND_EXPRESSION;
		}
		else
		    xp->xp_context = EXPAND_COMMANDS;
	    }
	    else
		xp->xp_context = EXPAND_EXPRESSION;
	}
	else
	    // Doesn't look like something valid, expand as an expression
	    // anyway.
	    xp->xp_context = EXPAND_EXPRESSION;
	arg = xp->xp_pattern;
	if (*arg != NUL)
	    while ((c = *++arg) != NUL && (c == ' ' || c == '\t'))
		/* skip */ ;
    }

    // ":exe one two" completes "two"
    if ((cmdidx == CMD_execute
		|| cmdidx == CMD_echo
		|| cmdidx == CMD_echon
		|| cmdidx == CMD_echomsg
		|| cmdidx == CMD_echowindow)
	    && xp->xp_context == EXPAND_EXPRESSION)
    {
	for (;;)
	{
	    char_u *n = skiptowhite(arg);

	    if (n == arg || IS_WHITE_OR_NUL(*skipwhite(n)))
		break;
	    arg = skipwhite(n);
	}
    }

    xp->xp_pattern = arg;
}

/*
 * Return TRUE if "pat" matches "text".
 * Does not use 'cpo' and always uses 'magic'.
 */
    int
pattern_match(char_u *pat, char_u *text, int ic)
{
    int		matches = FALSE;
    char_u	*save_cpo;
    regmatch_T	regmatch;

    // avoid 'l' flag in 'cpoptions'
    save_cpo = p_cpo;
    p_cpo = empty_option;
    regmatch.regprog = vim_regcomp(pat, RE_MAGIC + RE_STRING);
    if (regmatch.regprog != NULL)
    {
	regmatch.rm_ic = ic;
	matches = vim_regexec_nl(&regmatch, text, (colnr_T)0);
	vim_regfree(regmatch.regprog);
    }
    p_cpo = save_cpo;
    return matches;
}

/*
 * Handle a name followed by "(".  Both for just "name(arg)" and for
 * "expr->name(arg)".
 * Returns OK or FAIL.
 */
    static int
eval_func(
	char_u	    **arg,	// points to "(", will be advanced
	evalarg_T   *evalarg,
	char_u	    *name,
	int	    name_len,
	typval_T    *rettv,
	int	    flags,
	typval_T    *basetv)	// "expr" for "expr->name(arg)"
{
    int		evaluate = flags & EVAL_EVALUATE;
    char_u	*s = name;
    int		len = name_len;
    partial_T	*partial;
    int		ret = OK;
    type_T	*type = NULL;
    int		found_var = FALSE;

    if (!evaluate)
	check_vars(s, len);

    // If "s" is the name of a variable of type VAR_FUNC
    // use its contents.
    s = deref_func_name(s, &len, &partial,
		 in_vim9script() ? &type : NULL, !evaluate, FALSE, &found_var);

    // Need to make a copy, in case evaluating the arguments makes
    // the name invalid.
    s = vim_strsave(s);
    if (s == NULL || (evaluate && *s == NUL))
	ret = FAIL;
    else
    {
	funcexe_T funcexe;

	// Invoke the function.
	CLEAR_FIELD(funcexe);
	funcexe.fe_firstline = curwin->w_cursor.lnum;
	funcexe.fe_lastline = curwin->w_cursor.lnum;
	funcexe.fe_evaluate = evaluate;
	funcexe.fe_partial = partial;
	if (partial != NULL)
	{
	    funcexe.fe_object = partial->pt_obj;
	    if (funcexe.fe_object != NULL)
		++funcexe.fe_object->obj_refcount;
	}
	funcexe.fe_basetv = basetv;
	funcexe.fe_check_type = type;
	funcexe.fe_found_var = found_var;
	ret = get_func_tv(s, len, rettv, arg, evalarg, &funcexe);
    }
    vim_free(s);

    // If evaluate is FALSE rettv->v_type was not set in
    // get_func_tv, but it's needed in handle_subscript() to parse
    // what follows. So set it here.
    if (rettv->v_type == VAR_UNKNOWN && !evaluate && **arg == '(')
    {
	rettv->vval.v_string = NULL;
	rettv->v_type = VAR_FUNC;
    }

    // Stop the expression evaluation when immediately
    // aborting on error, or when an interrupt occurred or
    // an exception was thrown but not caught.
    if (evaluate && aborting())
    {
	if (ret == OK)
	    clear_tv(rettv);
	ret = FAIL;
    }
    return ret;
}

/*
 * After a NL, skip over empty lines and comment-only lines.
 */
    static char_u *
newline_skip_comments(char_u *arg)
{
    char_u *p = arg + 1;

    for (;;)
    {
	p = skipwhite(p);

	if (*p == NUL)
	    break;
	if (vim9_comment_start(p))
	{
	    char_u *nl = vim_strchr(p, NL);

	    if (nl == NULL)
		    break;
	    p = nl;
	}
	if (*p != NL)
	    break;
	++p;  // skip another NL
    }
    return p;
}

/*
 * Get the next line source line without advancing.  But do skip over comment
 * lines.
 * Only called for Vim9 script.
 */
    static char_u *
getline_peek_skip_comments(evalarg_T *evalarg)
{
    for (;;)
    {
	char_u *next = getline_peek(evalarg->eval_getline,
							 evalarg->eval_cookie);
	char_u *p;

	if (next == NULL)
	    break;
	p = skipwhite(next);
	if (*p != NUL && !vim9_comment_start(p))
	    return next;
	if (eval_next_line(NULL, evalarg) == NULL)
	    break;
    }
    return NULL;
}

/*
 * If inside Vim9 script, "arg" points to the end of a line (ignoring a #
 * comment) and there is a next line, return the next line (skipping blanks)
 * and set "getnext".
 * Otherwise return the next non-white at or after "arg" and set "getnext" to
 * FALSE.
 * "arg" must point somewhere inside a line, not at the start.
 */
    char_u *
eval_next_non_blank(char_u *arg, evalarg_T *evalarg, int *getnext)
{
    char_u *p = skipwhite(arg);

    *getnext = FALSE;
    if (in_vim9script()
	    && evalarg != NULL
	    && (evalarg->eval_cookie != NULL || evalarg->eval_cctx != NULL
								   || *p == NL)
	    && (*p == NUL || *p == NL
			     || (vim9_comment_start(p) && VIM_ISWHITE(p[-1]))))
    {
	char_u *next;

	if (*p == NL)
	    next = newline_skip_comments(p);
	else if (evalarg->eval_cookie != NULL)
	    next = getline_peek_skip_comments(evalarg);
	else
	    next = peek_next_line_from_context(evalarg->eval_cctx);

	if (next != NULL)
	{
	    *getnext = *p != NL;
	    return skipwhite(next);
	}
    }
    return p;
}

/*
 * To be called after eval_next_non_blank() sets "getnext" to TRUE.
 * Only called for Vim9 script.
 *
 * If "arg" is not NULL, then the caller should assign the return value to
 * "arg".
 */
    char_u *
eval_next_line(char_u *arg, evalarg_T *evalarg)
{
    garray_T	*gap = &evalarg->eval_ga;
    char_u	*line;

    if (arg != NULL)
    {
	if (*arg == NL)
	    return newline_skip_comments(arg);
	// Truncate before a trailing comment, so that concatenating the lines
	// won't turn the rest into a comment.
	if (*skipwhite(arg) == '#')
	    *arg = NUL;
    }

    if (evalarg->eval_cookie != NULL)
	line = evalarg->eval_getline(0, evalarg->eval_cookie, 0,
							   GETLINE_CONCAT_ALL);
    else
	line = next_line_from_context(evalarg->eval_cctx, TRUE);
    if (line == NULL)
	return NULL;

    ++evalarg->eval_break_count;
    if (gap->ga_itemsize > 0 && ga_grow(gap, 1) == OK)
    {
	char_u *p = skipwhite(line);

	// Going to concatenate the lines after parsing.  For an empty or
	// comment line use an empty string.
	if (*p == NUL || vim9_comment_start(p))
	{
	    vim_free(line);
	    line = vim_strsave((char_u *)"");
	}

	((char_u **)gap->ga_data)[gap->ga_len] = line;
	++gap->ga_len;
    }
    else if (evalarg->eval_cookie != NULL)
    {
	free_eval_tofree_later(evalarg);
	evalarg->eval_tofree = line;
    }

    // Advanced to the next line, "arg" no longer points into the previous
    // line.  The caller assigns the return value to "arg".
    // If "arg" is NULL, then the return value is discarded.  In that case,
    // "arg" still points to the previous line.  So don't reset
    // "eval_using_cmdline".
    if (arg != NULL)
	evalarg->eval_using_cmdline = FALSE;
    return skipwhite(line);
}

/*
 * Call eval_next_non_blank() and get the next line if needed.
 */
    char_u *
skipwhite_and_linebreak(char_u *arg, evalarg_T *evalarg)
{
    int	    getnext;
    char_u  *p = skipwhite_and_nl(arg);

    if (evalarg == NULL)
	return skipwhite(arg);
    eval_next_non_blank(p, evalarg, &getnext);
    if (getnext)
	return eval_next_line(arg, evalarg);
    return p;
}

/*
 * The "eval" functions have an "evalarg" argument: When NULL or
 * "evalarg->eval_flags" does not have EVAL_EVALUATE, then the argument is only
 * parsed but not executed.  The functions may return OK, but the rettv will be
 * of type VAR_UNKNOWN.  The functions still returns FAIL for a syntax error.
 */

/*
 * Handle zero level expression.
 * This calls eval1() and handles error message and nextcmd.
 * Put the result in "rettv" when returning OK and "evaluate" is TRUE.
 * Note: "rettv.v_lock" is not set.
 * "evalarg" can be NULL, EVALARG_EVALUATE or a pointer.
 * Return OK or FAIL.
 */
    int
eval0(
    char_u	*arg,
    typval_T	*rettv,
    exarg_T	*eap,
    evalarg_T	*evalarg)
{
    return eval0_retarg(arg, rettv, eap, evalarg, NULL);
}

/*
 * If "arg" is a simple function call without arguments then call it and return
 * the result.  Otherwise return NOTDONE.
 */
    int
may_call_simple_func(
    char_u	*arg,
    typval_T	*rettv)
{
    char_u  *parens = (char_u *)strstr((char *)arg, "()");
    int	    r = NOTDONE;

    // If the expression is "FuncName()" then we can skip a lot of overhead.
    if (parens != NULL && *skipwhite(parens + 2) == NUL)
    {
	char_u *p = STRNCMP(arg, "<SNR>", 5) == 0 ? skipdigits(arg + 5) : arg;

	if (to_name_end(p, TRUE) == parens)
	    r = call_simple_func(arg, (int)(parens - arg), rettv);
    }
    return r;
}

/*
 * Handle zero level expression with optimization for a simple function call.
 * Same arguments and return value as eval0().
 */
    int
eval0_simple_funccal(
    char_u	*arg,
    typval_T	*rettv,
    exarg_T	*eap,
    evalarg_T	*evalarg)
{
    int	    r = may_call_simple_func(arg, rettv);

    if (r == NOTDONE)
	r = eval0_retarg(arg, rettv, eap, evalarg, NULL);
    return r;
}

/*
 * Like eval0() but when "retarg" is not NULL store the pointer to after the
 * expression and don't check what comes after the expression.
 */
    int
eval0_retarg(
    char_u	*arg,
    typval_T	*rettv,
    exarg_T	*eap,
    evalarg_T	*evalarg,
    char_u	**retarg)
{
    int		ret;
    char_u	*p;
    char_u	*expr_end;
    int		did_emsg_before = did_emsg;
    int		called_emsg_before = called_emsg;
    int		check_for_end = retarg == NULL;
    int		end_error = FALSE;

    p = skipwhite(arg);
    ret = eval1(&p, rettv, evalarg);

    if (ret != FAIL)
    {
	expr_end = p;
	p = skipwhite(p);

	// In Vim9 script a command block is not split at NL characters for
	// commands using an expression argument.  Skip over a '#' comment to
	// check for a following NL.  Require white space before the '#'.
	if (in_vim9script() && p > expr_end && retarg == NULL)
	    while (*p == '#')
	    {
		char_u *nl = vim_strchr(p, NL);

		if (nl == NULL)
		    break;
		p = skipwhite(nl + 1);
		if (eap != NULL && *p != NUL)
		    eap->nextcmd = p;
		check_for_end = FALSE;
	    }

	if (check_for_end)
	    end_error = !ends_excmd2(arg, p);
    }

    if (ret == FAIL || end_error)
    {
	if (ret != FAIL)
	    clear_tv(rettv);
	/*
	 * Report the invalid expression unless the expression evaluation has
	 * been cancelled due to an aborting error, an interrupt, or an
	 * exception, or we already gave a more specific error.
	 * Also check called_emsg for when using assert_fails().
	 */
	if (!aborting()
		&& did_emsg == did_emsg_before
		&& called_emsg == called_emsg_before
		&& (!in_vim9script() || !vim9_bad_comment(p)))
	{
	    if (end_error)
		semsg(_(e_trailing_characters_str), p);
	    else
		semsg(_(e_invalid_expression_str), arg);
	}

	if (eap != NULL && p != NULL)
	{
	    // Some of the expression may not have been consumed.
	    // Only execute a next command if it cannot be a "||" operator.
	    // The next command may be "catch".
	    char_u *nextcmd = check_nextcmd(p);
	    if (nextcmd != NULL && *nextcmd != '|')
		eap->nextcmd = nextcmd;
	}
	return FAIL;
    }

    if (retarg != NULL)
	*retarg = p;
    else if (check_for_end && eap != NULL)
	set_nextcmd(eap, p);

    return ret;
}

/*
 * Handle top level expression:
 *	expr2 ? expr1 : expr1
 *	expr2 ?? expr1
 *
 * "arg" must point to the first non-white of the expression.
 * "arg" is advanced to just after the recognized expression.
 *
 * Note: "rettv.v_lock" is not set.
 *
 * Return OK or FAIL.
 */
    int
eval1(char_u **arg, typval_T *rettv, evalarg_T *evalarg)
{
    char_u  *p;
    int	    getnext;

    CLEAR_POINTER(rettv);

    /*
     * Get the first variable.
     */
    if (eval2(arg, rettv, evalarg) == FAIL)
	return FAIL;

    p = eval_next_non_blank(*arg, evalarg, &getnext);
    if (*p == '?')
    {
	int		op_falsy = p[1] == '?';
	int		result;
	typval_T	var2;
	evalarg_T	*evalarg_used = evalarg;
	evalarg_T	local_evalarg;
	int		orig_flags;
	int		evaluate;
	int		vim9script = in_vim9script();

	if (evalarg == NULL)
	{
	    init_evalarg(&local_evalarg);
	    evalarg_used = &local_evalarg;
	}
	orig_flags = evalarg_used->eval_flags;
	evaluate = evalarg_used->eval_flags & EVAL_EVALUATE;

	if (getnext)
	    *arg = eval_next_line(*arg, evalarg_used);
	else
	{
	    if (evaluate && vim9script && !VIM_ISWHITE(p[-1]))
	    {
		error_white_both(p, op_falsy ? 2 : 1);
		clear_tv(rettv);
		return FAIL;
	    }
	    *arg = p;
	}

	result = FALSE;
	if (evaluate)
	{
	    int		error = FALSE;

	    if (op_falsy)
		result = tv2bool(rettv);
	    else if (vim9script)
		result = tv_get_bool_chk(rettv, &error);
	    else if (tv_get_number_chk(rettv, &error) != 0)
		result = TRUE;
	    if (error || !op_falsy || !result)
		clear_tv(rettv);
	    if (error)
		return FAIL;
	}

	/*
	 * Get the second variable.  Recursive!
	 */
	if (op_falsy)
	    ++*arg;
	if (evaluate && vim9script && !IS_WHITE_OR_NUL((*arg)[1]))
	{
	    error_white_both(*arg - (op_falsy ? 1 : 0), op_falsy ? 2 : 1);
	    clear_tv(rettv);
	    return FAIL;
	}
	*arg = skipwhite_and_linebreak(*arg + 1, evalarg_used);
	evalarg_used->eval_flags = (op_falsy ? !result : result)
				  ? orig_flags : (orig_flags & ~EVAL_EVALUATE);
	if (eval1(arg, &var2, evalarg_used) == FAIL)
	{
	    evalarg_used->eval_flags = orig_flags;
	    return FAIL;
	}
	if (!op_falsy || !result)
	    *rettv = var2;

	if (!op_falsy)
	{
	    /*
	     * Check for the ":".
	     */
	    p = eval_next_non_blank(*arg, evalarg_used, &getnext);
	    if (*p != ':')
	    {
		emsg(_(e_missing_colon_after_questionmark));
		if (evaluate && result)
		    clear_tv(rettv);
		evalarg_used->eval_flags = orig_flags;
		return FAIL;
	    }
	    if (getnext)
		*arg = eval_next_line(*arg, evalarg_used);
	    else
	    {
		if (evaluate && vim9script && !VIM_ISWHITE(p[-1]))
		{
		    error_white_both(p, 1);
		    clear_tv(rettv);
		    evalarg_used->eval_flags = orig_flags;
		    return FAIL;
		}
		*arg = p;
	    }

	    /*
	     * Get the third variable.  Recursive!
	     */
	    if (evaluate && vim9script && !IS_WHITE_OR_NUL((*arg)[1]))
	    {
		error_white_both(*arg, 1);
		clear_tv(rettv);
		evalarg_used->eval_flags = orig_flags;
		return FAIL;
	    }
	    *arg = skipwhite_and_linebreak(*arg + 1, evalarg_used);
	    evalarg_used->eval_flags = !result ? orig_flags
					       : (orig_flags & ~EVAL_EVALUATE);
	    if (eval1(arg, &var2, evalarg_used) == FAIL)
	    {
		if (evaluate && result)
		    clear_tv(rettv);
		evalarg_used->eval_flags = orig_flags;
		return FAIL;
	    }
	    if (evaluate && !result)
		*rettv = var2;
	}

	if (evalarg == NULL)
	    clear_evalarg(&local_evalarg, NULL);
	else
	    evalarg->eval_flags = orig_flags;
    }

    return OK;
}

/*
 * Handle first level expression:
 *	expr2 || expr2 || expr2	    logical OR
 *
 * "arg" must point to the first non-white of the expression.
 * "arg" is advanced to just after the recognized expression.
 *
 * Return OK or FAIL.
 */
    static int
eval2(char_u **arg, typval_T *rettv, evalarg_T *evalarg)
{
    char_u	*p;
    int		getnext;

    /*
     * Get the first expression.
     */
    if (eval3(arg, rettv, evalarg) == FAIL)
	return FAIL;

    /*
     * Handle the  "||" operator.
     */
    p = eval_next_non_blank(*arg, evalarg, &getnext);
    if (p[0] == '|' && p[1] == '|')
    {
	evalarg_T   *evalarg_used = evalarg;
	evalarg_T   local_evalarg;
	int	    evaluate;
	int	    orig_flags;
	long	    result = FALSE;
	typval_T    var2;
	int	    error = FALSE;
	int	    vim9script = in_vim9script();

	if (evalarg == NULL)
	{
	    init_evalarg(&local_evalarg);
	    evalarg_used = &local_evalarg;
	}
	orig_flags = evalarg_used->eval_flags;
	evaluate = orig_flags & EVAL_EVALUATE;
	if (evaluate)
	{
	    if (vim9script)
		result = tv_get_bool_chk(rettv, &error);
	    else if (tv_get_number_chk(rettv, &error) != 0)
		result = TRUE;
	    clear_tv(rettv);
	    if (error)
		return FAIL;
	}

	/*
	 * Repeat until there is no following "||".
	 */
	while (p[0] == '|' && p[1] == '|')
	{
	    if (getnext)
		*arg = eval_next_line(*arg, evalarg_used);
	    else
	    {
		if (evaluate && vim9script && !VIM_ISWHITE(p[-1]))
		{
		    error_white_both(p, 2);
		    clear_tv(rettv);
		    return FAIL;
		}
		*arg = p;
	    }

	    /*
	     * Get the second variable.
	     */
	    if (evaluate && vim9script && !IS_WHITE_OR_NUL((*arg)[2]))
	    {
		error_white_both(*arg, 2);
		clear_tv(rettv);
		return FAIL;
	    }
	    *arg = skipwhite_and_linebreak(*arg + 2, evalarg_used);
	    evalarg_used->eval_flags = !result ? orig_flags
					       : (orig_flags & ~EVAL_EVALUATE);
	    if (eval3(arg, &var2, evalarg_used) == FAIL)
		return FAIL;

	    /*
	     * Compute the result.
	     */
	    if (evaluate && !result)
	    {
		if (vim9script)
		    result = tv_get_bool_chk(&var2, &error);
		else if (tv_get_number_chk(&var2, &error) != 0)
		    result = TRUE;
		clear_tv(&var2);
		if (error)
		    return FAIL;
	    }
	    if (evaluate)
	    {
		if (vim9script)
		{
		    rettv->v_type = VAR_BOOL;
		    rettv->vval.v_number = result ? VVAL_TRUE : VVAL_FALSE;
		}
		else
		{
		    rettv->v_type = VAR_NUMBER;
		    rettv->vval.v_number = result;
		}
	    }

	    p = eval_next_non_blank(*arg, evalarg_used, &getnext);
	}

	if (evalarg == NULL)
	    clear_evalarg(&local_evalarg, NULL);
	else
	    evalarg->eval_flags = orig_flags;
    }

    return OK;
}

/*
 * Handle second level expression:
 *	expr3 && expr3 && expr3	    logical AND
 *
 * "arg" must point to the first non-white of the expression.
 * "arg" is advanced to just after the recognized expression.
 *
 * Return OK or FAIL.
 */
    static int
eval3(char_u **arg, typval_T *rettv, evalarg_T *evalarg)
{
    char_u	*p;
    int		getnext;

    /*
     * Get the first expression.
     */
    if (eval4(arg, rettv, evalarg) == FAIL)
	return FAIL;

    /*
     * Handle the "&&" operator.
     */
    p = eval_next_non_blank(*arg, evalarg, &getnext);
    if (p[0] == '&' && p[1] == '&')
    {
	evalarg_T   *evalarg_used = evalarg;
	evalarg_T   local_evalarg;
	int	    orig_flags;
	int	    evaluate;
	long	    result = TRUE;
	typval_T    var2;
	int	    error = FALSE;
	int	    vim9script = in_vim9script();

	if (evalarg == NULL)
	{
	    init_evalarg(&local_evalarg);
	    evalarg_used = &local_evalarg;
	}
	orig_flags = evalarg_used->eval_flags;
	evaluate = orig_flags & EVAL_EVALUATE;
	if (evaluate)
	{
	    if (vim9script)
		result = tv_get_bool_chk(rettv, &error);
	    else if (tv_get_number_chk(rettv, &error) == 0)
		result = FALSE;
	    clear_tv(rettv);
	    if (error)
		return FAIL;
	}

	/*
	 * Repeat until there is no following "&&".
	 */
	while (p[0] == '&' && p[1] == '&')
	{
	    if (getnext)
		*arg = eval_next_line(*arg, evalarg_used);
	    else
	    {
		if (evaluate && vim9script && !VIM_ISWHITE(p[-1]))
		{
		    error_white_both(p, 2);
		    clear_tv(rettv);
		    return FAIL;
		}
		*arg = p;
	    }

	    /*
	     * Get the second variable.
	     */
	    if (evaluate && vim9script && !IS_WHITE_OR_NUL((*arg)[2]))
	    {
		error_white_both(*arg, 2);
		clear_tv(rettv);
		return FAIL;
	    }
	    *arg = skipwhite_and_linebreak(*arg + 2, evalarg_used);
	    evalarg_used->eval_flags = result ? orig_flags
					      : (orig_flags & ~EVAL_EVALUATE);
	    CLEAR_FIELD(var2);
	    if (eval4(arg, &var2, evalarg_used) == FAIL)
		return FAIL;

	    /*
	     * Compute the result.
	     */
	    if (evaluate && result)
	    {
		if (vim9script)
		    result = tv_get_bool_chk(&var2, &error);
		else if (tv_get_number_chk(&var2, &error) == 0)
		    result = FALSE;
		clear_tv(&var2);
		if (error)
		    return FAIL;
	    }
	    if (evaluate)
	    {
		if (vim9script)
		{
		    rettv->v_type = VAR_BOOL;
		    rettv->vval.v_number = result ? VVAL_TRUE : VVAL_FALSE;
		}
		else
		{
		    rettv->v_type = VAR_NUMBER;
		    rettv->vval.v_number = result;
		}
	    }

	    p = eval_next_non_blank(*arg, evalarg_used, &getnext);
	}

	if (evalarg == NULL)
	    clear_evalarg(&local_evalarg, NULL);
	else
	    evalarg->eval_flags = orig_flags;
    }

    return OK;
}

/*
 * Handle third level expression:
 *	var1 == var2
 *	var1 =~ var2
 *	var1 != var2
 *	var1 !~ var2
 *	var1 > var2
 *	var1 >= var2
 *	var1 < var2
 *	var1 <= var2
 *	var1 is var2
 *	var1 isnot var2
 *
 * "arg" must point to the first non-white of the expression.
 * "arg" is advanced to just after the recognized expression.
 *
 * Return OK or FAIL.
 */
    static int
eval4(char_u **arg, typval_T *rettv, evalarg_T *evalarg)
{
    char_u	*p;
    int		getnext;
    exprtype_T	type = EXPR_UNKNOWN;
    int		len = 2;
    int		type_is = FALSE;

    /*
     * Get the first expression.
     */
    if (eval5(arg, rettv, evalarg) == FAIL)
	return FAIL;

    p = eval_next_non_blank(*arg, evalarg, &getnext);

    type = get_compare_type(p, &len, &type_is);

    /*
     * If there is a comparative operator, use it.
     */
    if (type != EXPR_UNKNOWN)
    {
	typval_T    var2;
	int	    ic;
	int	    vim9script = in_vim9script();
	int	    evaluate = evalarg == NULL
				   ? 0 : (evalarg->eval_flags & EVAL_EVALUATE);
	long	    comp_lnum = SOURCING_LNUM;

	if (getnext)
	{
	    *arg = eval_next_line(*arg, evalarg);
	    p = *arg;
	}
	else if (evaluate && vim9script && !VIM_ISWHITE(**arg))
	{
	    error_white_both(*arg, len);
	    clear_tv(rettv);
	    return FAIL;
	}

	if (vim9script && type_is && (p[len] == '?' || p[len] == '#'))
	{
	    semsg(_(e_invalid_expression_str), p);
	    clear_tv(rettv);
	    return FAIL;
	}

	// extra question mark appended: ignore case
	if (p[len] == '?')
	{
	    ic = TRUE;
	    ++len;
	}
	// extra '#' appended: match case
	else if (p[len] == '#')
	{
	    ic = FALSE;
	    ++len;
	}
	// nothing appended: use 'ignorecase' if not in Vim script
	else
	    ic = vim9script ? FALSE : p_ic;

	/*
	 * Get the second variable.
	 */
	if (evaluate && vim9script && !IS_WHITE_OR_NUL(p[len]))
	{
	    error_white_both(p, len);
	    clear_tv(rettv);
	    return FAIL;
	}
	*arg = skipwhite_and_linebreak(p + len, evalarg);
	if (eval5(arg, &var2, evalarg) == FAIL)
	{
	    clear_tv(rettv);
	    return FAIL;
	}
	if (evaluate)
	{
	    int ret;

	    // use the line of the comparison for messages
	    SOURCING_LNUM = comp_lnum;
	    if (vim9script && check_compare_types(type, rettv, &var2) == FAIL)
	    {
		ret = FAIL;
		clear_tv(rettv);
	    }
	    else
		ret = typval_compare(rettv, &var2, type, ic);
	    clear_tv(&var2);
	    return ret;
	}
    }

    return OK;
}

/*
 * Make a copy of blob "tv1" and append blob "tv2".
 */
    void
eval_addblob(typval_T *tv1, typval_T *tv2)
{
    blob_T  *b1 = tv1->vval.v_blob;
    blob_T  *b2 = tv2->vval.v_blob;
    blob_T  *b = blob_alloc();
    int	    i;

    if (b == NULL)
	return;

    for (i = 0; i < blob_len(b1); i++)
	ga_append(&b->bv_ga, blob_get(b1, i));
    for (i = 0; i < blob_len(b2); i++)
	ga_append(&b->bv_ga, blob_get(b2, i));

    clear_tv(tv1);
    rettv_blob_set(tv1, b);
}

/*
 * Make a copy of list "tv1" and append list "tv2".
 */
    int
eval_addlist(typval_T *tv1, typval_T *tv2)
{
    typval_T var3;

    // concatenate Lists
    if (list_concat(tv1->vval.v_list, tv2->vval.v_list, &var3) == FAIL)
    {
	clear_tv(tv1);
	clear_tv(tv2);
	return FAIL;
    }
    clear_tv(tv1);
    *tv1 = var3;
    return OK;
}

/*
 * Handle the bitwise left/right shift operator expression:
 *	var1 << var2
 *	var1 >> var2
 *
 * "arg" must point to the first non-white of the expression.
 * "arg" is advanced to just after the recognized expression.
 *
 * Return OK or FAIL.
 */
    static int
eval5(char_u **arg, typval_T *rettv, evalarg_T *evalarg)
{
    /*
     * Get the first expression.
     */
    if (eval6(arg, rettv, evalarg) == FAIL)
	return FAIL;

    /*
     * Repeat computing, until no '<<' or '>>' is following.
     */
    for (;;)
    {
	char_u		*p;
	int		getnext;
	exprtype_T	type;
	int		evaluate;
	typval_T	var2;
	int		vim9script;

	p = eval_next_non_blank(*arg, evalarg, &getnext);
	if (p[0] == '<' && p[1] == '<')
	    type = EXPR_LSHIFT;
	else if (p[0] == '>' && p[1] == '>')
	    type = EXPR_RSHIFT;
	else
	    return OK;

	// Handle a bitwise left or right shift operator
	evaluate = evalarg == NULL ? 0 : (evalarg->eval_flags & EVAL_EVALUATE);
	if (evaluate && rettv->v_type != VAR_NUMBER)
	{
	    // left operand should be a number
	    emsg(_(e_bitshift_ops_must_be_number));
	    clear_tv(rettv);
	    return FAIL;
	}

	vim9script = in_vim9script();
	if (getnext)
	{
	    *arg = eval_next_line(*arg, evalarg);
	    p = *arg;
	}
	else if (evaluate && vim9script && !VIM_ISWHITE(**arg))
	{
	    error_white_both(*arg, 2);
	    clear_tv(rettv);
	    return FAIL;
	}

	/*
	 * Get the second variable.
	 */
	if (evaluate && vim9script && !IS_WHITE_OR_NUL(p[2]))
	{
	    error_white_both(p, 2);
	    clear_tv(rettv);
	    return FAIL;
	}
	*arg = skipwhite_and_linebreak(p + 2, evalarg);
	if (eval6(arg, &var2, evalarg) == FAIL)
	{
	    clear_tv(rettv);
	    return FAIL;
	}

	if (evaluate)
	{
	    if (var2.v_type != VAR_NUMBER || var2.vval.v_number < 0)
	    {
		// right operand should be a positive number
		if (var2.v_type != VAR_NUMBER)
		    emsg(_(e_bitshift_ops_must_be_number));
		else
		    emsg(_(e_bitshift_ops_must_be_positive));
		clear_tv(rettv);
		clear_tv(&var2);
		return FAIL;
	    }

	    if (var2.vval.v_number > MAX_LSHIFT_BITS)
		// shifting more bits than we have always results in zero
		rettv->vval.v_number = 0;
	    else if (type == EXPR_LSHIFT)
		rettv->vval.v_number =
		      (uvarnumber_T)rettv->vval.v_number << var2.vval.v_number;
	    else
		rettv->vval.v_number =
		      (uvarnumber_T)rettv->vval.v_number >> var2.vval.v_number;
	}

	clear_tv(&var2);
    }

    return OK;
}

/*
 * Handle fifth level expression:
 *	+	number addition, concatenation of list or blob
 *	-	number subtraction
 *	.	string concatenation (if script version is 1)
 *	..	string concatenation
 *
 * "arg" must point to the first non-white of the expression.
 * "arg" is advanced to just after the recognized expression.
 *
 * Return OK or FAIL.
 */
    static int
eval6(char_u **arg, typval_T *rettv, evalarg_T *evalarg)
{
    /*
     * Get the first expression.
     */
    if (eval7(arg, rettv, evalarg, FALSE) == FAIL)
	return FAIL;

    /*
     * Repeat computing, until no '+', '-' or '.' is following.
     */
    for (;;)
    {
	int	    evaluate;
	int	    getnext;
	char_u	    *p;
	int	    op;
	int	    oplen;
	int	    concat;
	typval_T    var2;
	int	    vim9script = in_vim9script();

	// "." is only string concatenation when scriptversion is 1
	// "+=", "-=" and "..=" are assignments
	// "++" and "--" on the next line are a separate command.
	p = eval_next_non_blank(*arg, evalarg, &getnext);
	op = *p;
	concat = op == '.' && (*(p + 1) == '.' || in_old_script(2));
	if ((op != '+' && op != '-' && !concat) || p[1] == '='
					       || (p[1] == '.' && p[2] == '='))
	    break;
	if (getnext && (op == '+' || op == '-') && p[0] == p[1])
	    break;

	evaluate = evalarg == NULL ? 0 : (evalarg->eval_flags & EVAL_EVALUATE);
	oplen = (concat && p[1] == '.') ? 2 : 1;
	if (getnext)
	    *arg = eval_next_line(*arg, evalarg);
	else
	{
	    if (evaluate && vim9script && !VIM_ISWHITE(**arg))
	    {
		error_white_both(*arg, oplen);
		clear_tv(rettv);
		return FAIL;
	    }
	    *arg = p;
	}
	if ((op != '+' || (rettv->v_type != VAR_LIST
						 && rettv->v_type != VAR_BLOB))
		&& (op == '.' || rettv->v_type != VAR_FLOAT)
		&& evaluate)
	{
	    int		error = FALSE;

	    // For "list + ...", an illegal use of the first operand as
	    // a number cannot be determined before evaluating the 2nd
	    // operand: if this is also a list, all is ok.
	    // For "something . ...", "something - ..." or "non-list + ...",
	    // we know that the first operand needs to be a string or number
	    // without evaluating the 2nd operand.  So check before to avoid
	    // side effects after an error.
	    if (op != '.')
		tv_get_number_chk(rettv, &error);
	    if ((op == '.' && tv_get_string_chk(rettv) == NULL) || error)
	    {
		clear_tv(rettv);
		return FAIL;
	    }
	}

	/*
	 * Get the second variable.
	 */
	if (evaluate && vim9script && !IS_WHITE_OR_NUL((*arg)[oplen]))
	{
	    error_white_both(*arg, oplen);
	    clear_tv(rettv);
	    return FAIL;
	}
	*arg = skipwhite_and_linebreak(*arg + oplen, evalarg);
	if (eval7(arg, &var2, evalarg, !vim9script && op == '.') == FAIL)
	{
	    clear_tv(rettv);
	    return FAIL;
	}

	if (evaluate)
	{
	    /*
	     * Compute the result.
	     */
	    if (op == '.')
	    {
		char_u	buf1[NUMBUFLEN], buf2[NUMBUFLEN];
		char_u	*s1 = tv_get_string_buf(rettv, buf1);
		char_u	*s2 = NULL;

		if (vim9script && (var2.v_type == VAR_VOID
			|| var2.v_type == VAR_CHANNEL
			|| var2.v_type == VAR_JOB))
		    semsg(_(e_using_invalid_value_as_string_str),
						   vartype_name(var2.v_type));
		else if (vim9script && var2.v_type == VAR_FLOAT)
		{
		    vim_snprintf((char *)buf2, NUMBUFLEN, "%g",
							    var2.vval.v_float);
		    s2 = buf2;
		}
		else
		    s2 = tv_get_string_buf_chk(&var2, buf2);
		if (s2 == NULL)		// type error ?
		{
		    clear_tv(rettv);
		    clear_tv(&var2);
		    return FAIL;
		}
		p = concat_str(s1, s2);
		clear_tv(rettv);
		rettv->v_type = VAR_STRING;
		rettv->vval.v_string = p;
	    }
	    else if (op == '+' && rettv->v_type == VAR_BLOB
						   && var2.v_type == VAR_BLOB)
		eval_addblob(rettv, &var2);
	    else if (op == '+' && rettv->v_type == VAR_LIST
						   && var2.v_type == VAR_LIST)
	    {
		if (eval_addlist(rettv, &var2) == FAIL)
		    return FAIL;
	    }
	    else
	    {
		int		error = FALSE;
		varnumber_T	n1, n2;
		float_T		f1 = 0, f2 = 0;

		if (rettv->v_type == VAR_FLOAT)
		{
		    f1 = rettv->vval.v_float;
		    n1 = 0;
		}
		else
		{
		    n1 = tv_get_number_chk(rettv, &error);
		    if (error)
		    {
			// This can only happen for "list + non-list" or
			// "blob + non-blob".  For "non-list + ..." or
			// "something - ...", we returned before evaluating the
			// 2nd operand.
			clear_tv(rettv);
			clear_tv(&var2);
			return FAIL;
		    }
		    if (var2.v_type == VAR_FLOAT)
			f1 = n1;
		}
		if (var2.v_type == VAR_FLOAT)
		{
		    f2 = var2.vval.v_float;
		    n2 = 0;
		}
		else
		{
		    n2 = tv_get_number_chk(&var2, &error);
		    if (error)
		    {
			clear_tv(rettv);
			clear_tv(&var2);
			return FAIL;
		    }
		    if (rettv->v_type == VAR_FLOAT)
			f2 = n2;
		}
		clear_tv(rettv);

		// If there is a float on either side the result is a float.
		if (rettv->v_type == VAR_FLOAT || var2.v_type == VAR_FLOAT)
		{
		    if (op == '+')
			f1 = f1 + f2;
		    else
			f1 = f1 - f2;
		    rettv->v_type = VAR_FLOAT;
		    rettv->vval.v_float = f1;
		}
		else
		{
		    if (op == '+')
			n1 = n1 + n2;
		    else
			n1 = n1 - n2;
		    rettv->v_type = VAR_NUMBER;
		    rettv->vval.v_number = n1;
		}
	    }
	    clear_tv(&var2);
	}
    }
    return OK;
}

/*
 * Handle sixth level expression:
 *	*	number multiplication
 *	/	number division
 *	%	number modulo
 *
 * "arg" must point to the first non-white of the expression.
 * "arg" is advanced to just after the recognized expression.
 *
 * Return OK or FAIL.
 */
    static int
eval7(
    char_u	**arg,
    typval_T	*rettv,
    evalarg_T	*evalarg,
    int		want_string)  // after "." operator
{
    int	    use_float = FALSE;

    /*
     * Get the first expression.
     */
    if (eval8(arg, rettv, evalarg, want_string) == FAIL)
	return FAIL;

    /*
     * Repeat computing, until no '*', '/' or '%' is following.
     */
    for (;;)
    {
	int	    evaluate;
	int	    getnext;
	typval_T    var2;
	char_u	    *p;
	int	    op;
	varnumber_T n1, n2;
	float_T	    f1, f2;
	int	    error;

	// "*=", "/=" and "%=" are assignments
	p = eval_next_non_blank(*arg, evalarg, &getnext);
	op = *p;
	if ((op != '*' && op != '/' && op != '%') || p[1] == '=')
	    break;

	evaluate = evalarg == NULL ? 0 : (evalarg->eval_flags & EVAL_EVALUATE);
	if (getnext)
	    *arg = eval_next_line(*arg, evalarg);
	else
	{
	    if (evaluate && in_vim9script() && !VIM_ISWHITE(**arg))
	    {
		error_white_both(*arg, 1);
		clear_tv(rettv);
		return FAIL;
	    }
	    *arg = p;
	}

	f1 = 0;
	f2 = 0;
	error = FALSE;
	if (evaluate)
	{
	    if (rettv->v_type == VAR_FLOAT)
	    {
		f1 = rettv->vval.v_float;
		use_float = TRUE;
		n1 = 0;
	    }
	    else
		n1 = tv_get_number_chk(rettv, &error);
	    clear_tv(rettv);
	    if (error)
		return FAIL;
	}
	else
	    n1 = 0;

	/*
	 * Get the second variable.
	 */
	if (evaluate && in_vim9script() && !IS_WHITE_OR_NUL((*arg)[1]))
	{
	    error_white_both(*arg, 1);
	    clear_tv(rettv);
	    return FAIL;
	}
	*arg = skipwhite_and_linebreak(*arg + 1, evalarg);
	if (eval8(arg, &var2, evalarg, FALSE) == FAIL)
	    return FAIL;

	if (evaluate)
	{
	    if (var2.v_type == VAR_FLOAT)
	    {
		if (!use_float)
		{
		    f1 = n1;
		    use_float = TRUE;
		}
		f2 = var2.vval.v_float;
		n2 = 0;
	    }
	    else
	    {
		n2 = tv_get_number_chk(&var2, &error);
		clear_tv(&var2);
		if (error)
		    return FAIL;
		if (use_float)
		    f2 = n2;
	    }

	    /*
	     * Compute the result.
	     * When either side is a float the result is a float.
	     */
	    if (use_float)
	    {
		if (op == '*')
		    f1 = f1 * f2;
		else if (op == '/')
		{
#ifdef VMS
		    // VMS crashes on divide by zero, work around it
		    if (f2 == 0.0)
		    {
			if (f1 == 0)
			    f1 = -1 * __F_FLT_MAX - 1L;   // similar to NaN
			else if (f1 < 0)
			    f1 = -1 * __F_FLT_MAX;
			else
			    f1 = __F_FLT_MAX;
		    }
		    else
			f1 = f1 / f2;
#else
		    // We rely on the floating point library to handle divide
		    // by zero to result in "inf" and not a crash.
		    f1 = f1 / f2;
#endif
		}
		else
		{
		    emsg(_(e_cannot_use_percent_with_float));
		    return FAIL;
		}
		rettv->v_type = VAR_FLOAT;
		rettv->vval.v_float = f1;
	    }
	    else
	    {
		int	    failed = FALSE;

		if (op == '*')
		    n1 = n1 * n2;
		else if (op == '/')
		    n1 = num_divide(n1, n2, &failed);
		else
		    n1 = num_modulus(n1, n2, &failed);
		if (failed)
		    return FAIL;

		rettv->v_type = VAR_NUMBER;
		rettv->vval.v_number = n1;
	    }
	}
    }

    return OK;
}

/*
 * Handle a type cast before a base level expression.
 * "arg" must point to the first non-white of the expression.
 * "arg" is advanced to just after the recognized expression.
 * Return OK or FAIL.
 */
    static int
eval8(
    char_u	**arg,
    typval_T	*rettv,
    evalarg_T	*evalarg,
    int		want_string)	// after "." operator
{
    type_T	*want_type = NULL;
    garray_T	type_list;	    // list of pointers to allocated types
    int		res;
    int		evaluate = evalarg == NULL ? 0
				       : (evalarg->eval_flags & EVAL_EVALUATE);

    // Recognize <type> in Vim9 script only.
    if (in_vim9script() && **arg == '<' && eval_isnamec1((*arg)[1])
					     && STRNCMP(*arg, "<SNR>", 5) != 0)
    {
	++*arg;
	ga_init2(&type_list, sizeof(type_T *), 10);
	want_type = parse_type(arg, &type_list, TRUE);
	if (want_type == NULL && (evaluate || **arg != '>'))
	{
	    clear_type_list(&type_list);
	    return FAIL;
	}

	if (**arg != '>')
	{
	    if (*skipwhite(*arg) == '>')
		semsg(_(e_no_white_space_allowed_before_str_str), ">", *arg);
	    else
		emsg(_(e_missing_gt));
	    clear_type_list(&type_list);
	    return FAIL;
	}
	++*arg;
	*arg = skipwhite_and_linebreak(*arg, evalarg);
    }

    res = eval9(arg, rettv, evalarg, want_string);

    if (want_type != NULL && evaluate)
    {
	if (res == OK)
	{
	    type_T *actual = typval2type(rettv, get_copyID(), &type_list,
							       TVTT_DO_MEMBER);

	    if (!equal_type(want_type, actual, 0))
	    {
		if (want_type->tt_type == VAR_BOOL
					&& actual->tt_type != VAR_BOOL
					&& (actual->tt_flags & TTFLAG_BOOL_OK))
		{
		    int n = tv2bool(rettv);

		    // can use "0" and "1" for boolean in some places
		    clear_tv(rettv);
		    rettv->v_type = VAR_BOOL;
		    rettv->vval.v_number = n ? VVAL_TRUE : VVAL_FALSE;
		}
		else
		{
		    where_T where = WHERE_INIT;

		    res = check_type(want_type, actual, TRUE, where);
		}
	    }
	}
	clear_type_list(&type_list);
    }

    return res;
}

    int
eval_leader(char_u **arg, int vim9)
{
    char_u	*s = *arg;
    char_u	*p = *arg;

    while (*p == '!' || *p == '-' || *p == '+')
    {
	char_u *n = skipwhite(p + 1);

	// ++, --, -+ and +- are not accepted in Vim9 script
	if (vim9 && (*p == '-' || *p == '+') && (*n == '-' || *n == '+'))
	{
	    semsg(_(e_invalid_expression_str), s);
	    return FAIL;
	}
	p = n;
    }
    *arg = p;
    return OK;
}

/*
 * Check for a predefined value "true", "false" and "null.*".
 * Return OK when recognized.
 */
    int
handle_predefined(char_u *s, int len, typval_T *rettv)
{
    switch (len)
    {
	case 4: if (STRNCMP(s, "true", 4) == 0)
		{
		    rettv->v_type = VAR_BOOL;
		    rettv->vval.v_number = VVAL_TRUE;
		    return OK;
		}
		if (STRNCMP(s, "null", 4) == 0)
		{
		    rettv->v_type = VAR_SPECIAL;
		    rettv->vval.v_number = VVAL_NULL;
		    return OK;
		}
		break;
	case 5: if (STRNCMP(s, "false", 5) == 0)
		{
		    rettv->v_type = VAR_BOOL;
		    rettv->vval.v_number = VVAL_FALSE;
		    return OK;
		}
		break;
	case 8: if (STRNCMP(s, "null_job", 8) == 0)
		{
#ifdef FEAT_JOB_CHANNEL
		    rettv->v_type = VAR_JOB;
		    rettv->vval.v_job = NULL;
#else
		    rettv->v_type = VAR_SPECIAL;
		    rettv->vval.v_number = VVAL_NULL;
#endif
		    return OK;
		}
		break;
	case 9:
		if (STRNCMP(s, "null_", 5) != 0)
		    break;
		if (STRNCMP(s + 5, "list", 4) == 0)
		{
		    rettv->v_type = VAR_LIST;
		    rettv->vval.v_list = NULL;
		    return OK;
		}
		if (STRNCMP(s + 5, "dict", 4) == 0)
		{
		    rettv->v_type = VAR_DICT;
		    rettv->vval.v_dict = NULL;
		    return OK;
		}
		if (STRNCMP(s + 5, "blob", 4) == 0)
		{
		    rettv->v_type = VAR_BLOB;
		    rettv->vval.v_blob = NULL;
		    return OK;
		}
		break;
	case 10: if (STRNCMP(s, "null_class", 10) == 0)
		{
		    rettv->v_type = VAR_CLASS;
		    rettv->vval.v_class = NULL;
		    return OK;
		}
		 break;
	case 11: if (STRNCMP(s, "null_string", 11) == 0)
		{
		    rettv->v_type = VAR_STRING;
		    rettv->vval.v_string = NULL;
		    return OK;
		}
		if (STRNCMP(s, "null_object", 11) == 0)
		{
		    rettv->v_type = VAR_OBJECT;
		    rettv->vval.v_object = NULL;
		    return OK;
		}
		break;
	case 12:
		if (STRNCMP(s, "null_channel", 12) == 0)
		{
#ifdef FEAT_JOB_CHANNEL
		    rettv->v_type = VAR_CHANNEL;
		    rettv->vval.v_channel = NULL;
#else
		    rettv->v_type = VAR_SPECIAL;
		    rettv->vval.v_number = VVAL_NULL;
#endif
		    return OK;
		}
		if (STRNCMP(s, "null_partial", 12) == 0)
		{
		    rettv->v_type = VAR_PARTIAL;
		    rettv->vval.v_partial = NULL;
		    return OK;
		}
		break;
	case 13: if (STRNCMP(s, "null_function", 13) == 0)
		{
		    rettv->v_type = VAR_FUNC;
		    rettv->vval.v_string = NULL;
		    return OK;
		}
		break;
    }
    return FAIL;
}

/*
 * Handle sixth level expression:
 *  number		number constant
 *  0zFFFFFFFF		Blob constant
 *  "string"		string constant
 *  'string'		literal string constant
 *  &option-name	option value
 *  @r			register contents
 *  identifier		variable value
 *  function()		function call
 *  $VAR		environment variable
 *  (expression)	nested expression
 *  [expr, expr]	List
 *  {arg, arg -> expr}	Lambda
 *  {key: val, key: val}   Dictionary
 *  #{key: val, key: val}  Dictionary with literal keys
 *
 *  Also handle:
 *  ! in front		logical NOT
 *  - in front		unary minus
 *  + in front		unary plus (ignored)
 *  trailing []		subscript in String or List
 *  trailing .name	entry in Dictionary
 *  trailing ->name()	method call
 *
 * "arg" must point to the first non-white of the expression.
 * "arg" is advanced to just after the recognized expression.
 *
 * Return OK or FAIL.
 */
    static int
eval9(
    char_u	**arg,
    typval_T	*rettv,
    evalarg_T	*evalarg,
    int		want_string)	// after "." operator
{
    int		evaluate = evalarg != NULL
				      && (evalarg->eval_flags & EVAL_EVALUATE);
    int		len;
    char_u	*s;
    char_u	*name_start = NULL;
    char_u	*start_leader, *end_leader;
    int		ret = OK;
    char_u	*alias;
    static int	recurse = 0;
    int		vim9script = in_vim9script();

    /*
     * Initialise variable so that clear_tv() can't mistake this for a
     * string and free a string that isn't there.
     */
    rettv->v_type = VAR_UNKNOWN;

    /*
     * Skip '!', '-' and '+' characters.  They are handled later.
     */
    start_leader = *arg;
    if (eval_leader(arg, vim9script) == FAIL)
	return FAIL;
    end_leader = *arg;

    if (**arg == '.' && (!SAFE_isdigit(*(*arg + 1)) || in_old_script(2)))
    {
	semsg(_(e_invalid_expression_str), *arg);
	++*arg;
	return FAIL;
    }

    // Limit recursion to 1000 levels.  At least at 10000 we run out of stack
    // and crash.  With MSVC the stack is smaller.
    if (recurse ==
#ifdef _MSC_VER
		    300
#else
		    1000
#endif
		    )
    {
	semsg(_(e_expression_too_recursive_str), *arg);
	return FAIL;
    }
    ++recurse;

    switch (**arg)
    {
    /*
     * Number constant.
     */
    case '0':
    case '1':
    case '2':
    case '3':
    case '4':
    case '5':
    case '6':
    case '7':
    case '8':
    case '9':
    case '.':	ret = eval_number(arg, rettv, evaluate, want_string);

		// Apply prefixed "-" and "+" now.  Matters especially when
		// "->" follows.
		if (ret == OK && evaluate && end_leader > start_leader
						  && rettv->v_type != VAR_BLOB)
		    ret = eval9_leader(rettv, TRUE, start_leader, &end_leader);
		break;

    /*
     * String constant: "string".
     */
    case '"':	ret = eval_string(arg, rettv, evaluate, FALSE);
		break;

    /*
     * Literal string constant: 'str''ing'.
     */
    case '\'':	ret = eval_lit_string(arg, rettv, evaluate, FALSE);
		break;

    /*
     * List: [expr, expr]
     */
    case '[':	ret = eval_list(arg, rettv, evalarg, TRUE);
		break;

    /*
     * Dictionary: #{key: val, key: val}
     */
    case '#':	if (vim9script)
		{
		    ret = vim9_bad_comment(*arg) ? FAIL : NOTDONE;
		}
		else if ((*arg)[1] == '{')
		{
		    ++*arg;
		    ret = eval_dict(arg, rettv, evalarg, TRUE);
		}
		else
		    ret = NOTDONE;
		break;

    /*
     * Lambda: {arg, arg -> expr}
     * Dictionary: {'key': val, 'key': val}
     */
    case '{':	if (vim9script)
		    ret = NOTDONE;
		else
		    ret = get_lambda_tv(arg, rettv, vim9script, evalarg);
		if (ret == NOTDONE)
		    ret = eval_dict(arg, rettv, evalarg, FALSE);
		break;

    /*
     * Option value: &name
     */
    case '&':	ret = eval_option(arg, rettv, evaluate);
		break;

    /*
     * Environment variable: $VAR.
     * Interpolated string: $"string" or $'string'.
     */
    case '$':	if ((*arg)[1] == '"' || (*arg)[1] == '\'')
		    ret = eval_interp_string(arg, rettv, evaluate);
		else
		    ret = eval_env_var(arg, rettv, evaluate);
		break;

    /*
     * Register contents: @r.
     */
    case '@':	++*arg;
		if (evaluate)
		{
		    if (vim9script && IS_WHITE_OR_NUL(**arg))
			semsg(_(e_syntax_error_at_str), *arg);
		    else if (vim9script && !valid_yank_reg(**arg, FALSE))
			emsg_invreg(**arg);
		    else
		    {
			rettv->v_type = VAR_STRING;
			rettv->vval.v_string = get_reg_contents(**arg,
								GREG_EXPR_SRC);
		    }
		}
		if (**arg != NUL)
		    ++*arg;
		break;

    /*
     * nested expression: (expression).
     * or lambda: (arg) => expr
     */
    case '(':	ret = NOTDONE;
		if (vim9script)
		{
		    ret = get_lambda_tv(arg, rettv, TRUE, evalarg);
		    if (ret == OK && evaluate)
		    {
			ufunc_T *ufunc = rettv->vval.v_partial->pt_func;

			// Compile it here to get the return type.  The return
			// type is optional, when it's missing use t_unknown.
			// This is recognized in compile_return().
			if (ufunc->uf_ret_type->tt_type == VAR_VOID)
			    ufunc->uf_ret_type = &t_unknown;
			if (compile_def_function(ufunc, FALSE,
					get_compile_type(ufunc), NULL) == FAIL)
			{
			    clear_tv(rettv);
			    ret = FAIL;
			}
		    }
		}
		if (ret == NOTDONE)
		{
		    *arg = skipwhite_and_linebreak(*arg + 1, evalarg);
		    ret = eval1(arg, rettv, evalarg);	// recursive!

		    *arg = skipwhite_and_linebreak(*arg, evalarg);
		    if (**arg == ')')
			++*arg;
		    else if (ret == OK)
		    {
			emsg(_(e_missing_closing_paren));
			clear_tv(rettv);
			ret = FAIL;
		    }
		}
		break;

    default:	ret = NOTDONE;
		break;
    }

    if (ret == NOTDONE)
    {
	/*
	 * Must be a variable or function name.
	 * Can also be a curly-braces kind of name: {expr}.
	 */
	s = *arg;
	len = get_name_len(arg, &alias, evaluate, TRUE);
	if (alias != NULL)
	    s = alias;

	if (len <= 0)
	    ret = FAIL;
	else
	{
	    int	    flags = evalarg == NULL ? 0 : evalarg->eval_flags;

	    if (evaluate && vim9script && len == 1 && *s == '_')
	    {
		emsg(_(e_cannot_use_underscore_here));
		ret = FAIL;
	    }
	    else if (evaluate && vim9script && len > 2
						 && s[0] == 's' && s[1] == ':')
	    {
		semsg(_(e_cannot_use_s_colon_in_vim9_script_str), s);
		ret = FAIL;
	    }
	    else if ((vim9script ? **arg : *skipwhite(*arg)) == '(')
	    {
		// "name(..."  recursive!
		*arg = skipwhite(*arg);
		ret = eval_func(arg, evalarg, s, len, rettv, flags, NULL);
	    }
	    else if (evaluate)
	    {
		// get the value of "true", "false", etc. or a variable
		ret = FAIL;
		if (vim9script)
		    ret = handle_predefined(s, len, rettv);
		if (ret == FAIL)
		{
		    name_start = s;
		    ret = eval_variable(s, len, 0, rettv, NULL,
					   EVAL_VAR_VERBOSE + EVAL_VAR_IMPORT);
		}
	    }
	    else
	    {
		// skip the name
		check_vars(s, len);
		ret = OK;
	    }
	}
	vim_free(alias);
    }

    // Handle following '[', '(' and '.' for expr[expr], expr.name,
    // expr(expr), expr->name(expr)
    if (ret == OK)
	ret = handle_subscript(arg, name_start, rettv, evalarg, evaluate);

    /*
     * Apply logical NOT and unary '-', from right to left, ignore '+'.
     */
    if (ret == OK && evaluate && end_leader > start_leader)
	ret = eval9_leader(rettv, FALSE, start_leader, &end_leader);

    --recurse;
    return ret;
}

/*
 * Apply the leading "!" and "-" before an eval9 expression to "rettv".
 * When "numeric_only" is TRUE only handle "+" and "-".
 * Adjusts "end_leaderp" until it is at "start_leader".
 */
    static int
eval9_leader(
	typval_T    *rettv,
	int	    numeric_only,
	char_u	    *start_leader,
	char_u	    **end_leaderp)
{
    char_u	*end_leader = *end_leaderp;
    int		ret = OK;
    int		error = FALSE;
    varnumber_T val = 0;
    vartype_T	type = rettv->v_type;
    int		vim9script = in_vim9script();
    float_T	    f = 0.0;

    if (rettv->v_type == VAR_FLOAT)
	f = rettv->vval.v_float;
    else
    {
	while (VIM_ISWHITE(end_leader[-1]))
	    --end_leader;
	if (vim9script && end_leader[-1] == '!')
	    val = tv2bool(rettv);
	else
	    val = tv_get_number_chk(rettv, &error);
    }
    if (error)
    {
	clear_tv(rettv);
	ret = FAIL;
    }
    else
    {
	while (end_leader > start_leader)
	{
	    --end_leader;
	    if (*end_leader == '!')
	    {
		if (numeric_only)
		{
		    ++end_leader;
		    break;
		}
		if (rettv->v_type == VAR_FLOAT)
		{
		    if (vim9script)
		    {
			rettv->v_type = VAR_BOOL;
			val = f == 0.0 ? VVAL_TRUE : VVAL_FALSE;
		    }
		    else
			f = !f;
		}
		else
		{
		    val = !val;
		    type = VAR_BOOL;
		}
	    }
	    else if (*end_leader == '-')
	    {
		if (rettv->v_type == VAR_FLOAT)
		    f = -f;
		else
		{
		    val = -val;
		    type = VAR_NUMBER;
		}
	    }
	}
	if (rettv->v_type == VAR_FLOAT)
	{
	    clear_tv(rettv);
	    rettv->vval.v_float = f;
	}
	else
	{
	    clear_tv(rettv);
	    if (vim9script)
		rettv->v_type = type;
	    else
		rettv->v_type = VAR_NUMBER;
	    rettv->vval.v_number = val;
	}
    }
    *end_leaderp = end_leader;
    return ret;
}

/*
 * Call the function referred to in "rettv".
 */
    static int
call_func_rettv(
	char_u	    **arg,
	evalarg_T   *evalarg,
	typval_T    *rettv,
	int	    evaluate,
	dict_T	    *selfdict,
	typval_T    *basetv)
{
    partial_T	*pt = NULL;
    funcexe_T	funcexe;
    typval_T	functv;
    char_u	*s;
    int		ret;

    // need to copy the funcref so that we can clear rettv
    if (evaluate)
    {
	functv = *rettv;
	rettv->v_type = VAR_UNKNOWN;

	// Invoke the function.  Recursive!
	if (functv.v_type == VAR_PARTIAL)
	{
	    pt = functv.vval.v_partial;
	    s = partial_name(pt);
	}
	else
	{
	    s = functv.vval.v_string;
	    if (s == NULL || *s == NUL)
	    {
		emsg(_(e_empty_function_name));
		ret = FAIL;
		goto theend;
	    }
	}
    }
    else
	s = (char_u *)"";

    CLEAR_FIELD(funcexe);
    funcexe.fe_firstline = curwin->w_cursor.lnum;
    funcexe.fe_lastline = curwin->w_cursor.lnum;
    funcexe.fe_evaluate = evaluate;
    funcexe.fe_partial = pt;
    funcexe.fe_selfdict = selfdict;
    funcexe.fe_basetv = basetv;
    ret = get_func_tv(s, -1, rettv, arg, evalarg, &funcexe);

theend:
    // Clear the funcref afterwards, so that deleting it while
    // evaluating the arguments is possible (see test55).
    if (evaluate)
	clear_tv(&functv);

    return ret;
}

/*
 * Evaluate "->method()".
 * "*arg" points to "method".
 * Returns FAIL or OK. "*arg" is advanced to after the ')'.
 */
    static int
eval_lambda(
    char_u	**arg,
    typval_T	*rettv,
    evalarg_T	*evalarg,
    int		verbose)	// give error messages
{
    int		evaluate = evalarg != NULL
				      && (evalarg->eval_flags & EVAL_EVALUATE);
    typval_T	base = *rettv;
    int		ret;

    rettv->v_type = VAR_UNKNOWN;

    if (**arg == '{')
    {
	// ->{lambda}()
	ret = get_lambda_tv(arg, rettv, FALSE, evalarg);
    }
    else
    {
	// ->(lambda)()
	++*arg;
	ret = eval1(arg, rettv, evalarg);
	*arg = skipwhite_and_linebreak(*arg, evalarg);
	if (**arg != ')')
	{
	    emsg(_(e_missing_closing_paren));
	    return FAIL;
	}
	if (rettv->v_type != VAR_STRING && rettv->v_type != VAR_FUNC
					       && rettv->v_type != VAR_PARTIAL)
	{
	    emsg(_(e_string_or_function_required_for_arrow_parens_expr));
	    return FAIL;
	}
	++*arg;
    }
    if (ret != OK)
	return FAIL;

    if (**arg != '(')
    {
	if (verbose)
	{
	    if (*skipwhite(*arg) == '(')
		emsg(_(e_no_white_space_allowed_before_parenthesis));
	    else
		semsg(_(e_missing_parenthesis_str), "lambda");
	}
	clear_tv(rettv);
	ret = FAIL;
    }
    else
	ret = call_func_rettv(arg, evalarg, rettv, evaluate, NULL, &base);

    // Clear the funcref afterwards, so that deleting it while
    // evaluating the arguments is possible (see test55).
    if (evaluate)
	clear_tv(&base);

    return ret;
}

/*
 * Evaluate "->method()".
 * "*arg" points to "method".
 * Returns FAIL or OK. "*arg" is advanced to after the ')'.
 */
    static int
eval_method(
    char_u	**arg,
    typval_T	*rettv,
    evalarg_T	*evalarg,
    int		verbose)	// give error messages
{
    char_u	*name;
    long	len;
    char_u	*alias;
    char_u	*tofree = NULL;
    typval_T	base = *rettv;
    int		ret = OK;
    int		evaluate = evalarg != NULL
				      && (evalarg->eval_flags & EVAL_EVALUATE);

    rettv->v_type = VAR_UNKNOWN;

    name = *arg;
    len = get_name_len(arg, &alias, evaluate, evaluate);
    if (alias != NULL)
	name = alias;

    if (len <= 0)
    {
	if (verbose)
	    emsg(_(e_missing_name_after_method));
	ret = FAIL;
    }
    else
    {
	char_u *paren;

	// If there is no "(" immediately following, but there is further on,
	// it can be "import.Func()", "dict.Func()", "list[nr]", etc.
	// Does not handle anything where "(" is part of the expression.
	*arg = skipwhite(*arg);

	if (**arg != '(' && alias == NULL
				    && (paren = vim_strchr(*arg, '(')) != NULL)
	{
	    *arg = name;

	    // Truncate the name a the "(".  Avoid trying to get another line
	    // by making "getline" NULL.
	    *paren = NUL;
	    char_u	*(*getline)(int, void *, int, getline_opt_T) = NULL;
	    if (evalarg != NULL)
	    {
		getline = evalarg->eval_getline;
		evalarg->eval_getline = NULL;
	    }

	    char_u *deref = deref_function_name(arg, &tofree, evalarg, verbose);
	    if (deref == NULL)
	    {
		*arg = name + len;
		ret = FAIL;
	    }
	    else
	    {
		name = deref;
		len = (long)STRLEN(name);
	    }

	    *paren = '(';
	    if (getline != NULL)
		evalarg->eval_getline = getline;
	}

	if (ret == OK)
	{
	    *arg = skipwhite(*arg);

	    if (**arg != '(')
	    {
		if (verbose)
		    semsg(_(e_missing_parenthesis_str), name);
		ret = FAIL;
	    }
	    else if (VIM_ISWHITE((*arg)[-1]))
	    {
		if (verbose)
		    emsg(_(e_no_white_space_allowed_before_parenthesis));
		ret = FAIL;
	    }
	    else
		ret = eval_func(arg, evalarg, name, len, rettv,
					  evaluate ? EVAL_EVALUATE : 0, &base);
	}
    }

    // Clear the funcref afterwards, so that deleting it while
    // evaluating the arguments is possible (see test55).
    if (evaluate)
	clear_tv(&base);
    vim_free(tofree);

    return ret;
}

/*
 * Evaluate an "[expr]" or "[expr:expr]" index.  Also "dict.key".
 * "*arg" points to the '[' or '.'.
 * Returns FAIL or OK. "*arg" is advanced to after the ']'.
 */
    static int
eval_index(
    char_u	**arg,
    typval_T	*rettv,
    evalarg_T	*evalarg,
    int		verbose)	// give error messages
{
    int		evaluate = evalarg != NULL
				      && (evalarg->eval_flags & EVAL_EVALUATE);
    int		empty1 = FALSE, empty2 = FALSE;
    typval_T	var1, var2;
    int		range = FALSE;
    char_u	*key = NULL;
    int		keylen = -1;
    int		vim9script = in_vim9script();

    if (check_can_index(rettv, evaluate, verbose) == FAIL)
	return FAIL;

    init_tv(&var1);
    init_tv(&var2);
    if (**arg == '.')
    {
	/*
	 * dict.name
	 */
	key = *arg + 1;
	for (keylen = 0; eval_isdictc(key[keylen]); ++keylen)
	    ;
	if (keylen == 0)
	    return FAIL;
	*arg = key + keylen;
    }
    else
    {
	/*
	 * something[idx]
	 *
	 * Get the (first) variable from inside the [].
	 */
	*arg = skipwhite_and_linebreak(*arg + 1, evalarg);
	if (**arg == ':')
	    empty1 = TRUE;
	else if (eval1(arg, &var1, evalarg) == FAIL)	// recursive!
	    return FAIL;
	else if (vim9script && **arg == ':')
	{
	    semsg(_(e_white_space_required_before_and_after_str_at_str),
								    ":", *arg);
	    clear_tv(&var1);
	    return FAIL;
	}
	else if (evaluate)
	{
	    int error = FALSE;

	    // allow for indexing with float
	    if (vim9script && rettv->v_type == VAR_DICT
						   && var1.v_type == VAR_FLOAT)
	    {
		var1.vval.v_string = typval_tostring(&var1, TRUE);
		var1.v_type = VAR_STRING;
	    }

	    if (vim9script && rettv->v_type == VAR_LIST)
		tv_get_number_chk(&var1, &error);
	    else
		error = tv_get_string_chk(&var1) == NULL;
	    if (error)
	    {
		// not a number or string
		clear_tv(&var1);
		return FAIL;
	    }
	}

	/*
	 * Get the second variable from inside the [:].
	 */
	*arg = skipwhite_and_linebreak(*arg, evalarg);
	if (**arg == ':')
	{
	    range = TRUE;
	    ++*arg;
	    if (vim9script && !IS_WHITE_OR_NUL(**arg) && **arg != ']')
	    {
		semsg(_(e_white_space_required_before_and_after_str_at_str),
								":", *arg - 1);
		if (!empty1)
		    clear_tv(&var1);
		return FAIL;
	    }
	    *arg = skipwhite_and_linebreak(*arg, evalarg);
	    if (**arg == ']')
		empty2 = TRUE;
	    else if (eval1(arg, &var2, evalarg) == FAIL)	// recursive!
	    {
		if (!empty1)
		    clear_tv(&var1);
		return FAIL;
	    }
	    else if (evaluate && tv_get_string_chk(&var2) == NULL)
	    {
		// not a number or string
		if (!empty1)
		    clear_tv(&var1);
		clear_tv(&var2);
		return FAIL;
	    }
	}

	// Check for the ']'.
	*arg = skipwhite_and_linebreak(*arg, evalarg);
	if (**arg != ']')
	{
	    if (verbose)
		emsg(_(e_missing_closing_square_brace));
	    clear_tv(&var1);
	    if (range)
		clear_tv(&var2);
	    return FAIL;
	}
	*arg = *arg + 1;	// skip over the ']'
    }

    if (evaluate)
    {
	int res = eval_index_inner(rettv, range,
		empty1 ? NULL : &var1, empty2 ? NULL : &var2, FALSE,
		key, keylen, verbose);

	if (!empty1)
	    clear_tv(&var1);
	if (range)
	    clear_tv(&var2);
	return res;
    }
    return OK;
}

/*
 * Check if "rettv" can have an [index] or [sli:ce]
 */
    int
check_can_index(typval_T *rettv, int evaluate, int verbose)
{
    switch (rettv->v_type)
    {
	case VAR_FUNC:
	case VAR_PARTIAL:
	    if (verbose)
		emsg(_(e_cannot_index_a_funcref));
	    return FAIL;
	case VAR_FLOAT:
	    if (verbose)
		emsg(_(e_using_float_as_string));
	    return FAIL;
	case VAR_BOOL:
	case VAR_SPECIAL:
	case VAR_JOB:
	case VAR_CHANNEL:
	case VAR_INSTR:
	case VAR_OBJECT:
	    if (verbose)
		emsg(_(e_cannot_index_special_variable));
	    return FAIL;
	case VAR_CLASS:
	case VAR_TYPEALIAS:
	    if (verbose)
		check_typval_is_value(rettv);
	    return FAIL;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	    if (evaluate)
	    {
		emsg(_(e_cannot_index_special_variable));
		return FAIL;
	    }
	    // FALLTHROUGH

	case VAR_STRING:
	case VAR_LIST:
	case VAR_DICT:
	case VAR_BLOB:
	    break;
	case VAR_NUMBER:
	    if (in_vim9script())
		emsg(_(e_cannot_index_number));
	    break;
    }
    return OK;
}

/*
 * slice() function
 */
    void
f_slice(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && ((argvars[0].v_type != VAR_STRING
		    && argvars[0].v_type != VAR_LIST
		    && argvars[0].v_type != VAR_BLOB
		    && check_for_list_arg(argvars, 0) == FAIL)
		|| check_for_number_arg(argvars, 1) == FAIL
		|| check_for_opt_number_arg(argvars, 2) == FAIL))
	return;

    if (check_can_index(argvars, TRUE, FALSE) != OK)
	return;

    copy_tv(argvars, rettv);
    eval_index_inner(rettv, TRUE, argvars + 1,
	    argvars[2].v_type == VAR_UNKNOWN ? NULL : argvars + 2,
	    TRUE, NULL, 0, FALSE);
}

/*
 * Apply index or range to "rettv".
 * "var1" is the first index, NULL for [:expr].
 * "var2" is the second index, NULL for [expr] and [expr: ]
 * "exclusive" is TRUE for slice(): second index is exclusive, use character
 * index for string.
 * Alternatively, "key" is not NULL, then key[keylen] is the dict index.
 */
    int
eval_index_inner(
	typval_T    *rettv,
	int	    is_range,
	typval_T    *var1,
	typval_T    *var2,
	int	    exclusive,
	char_u	    *key,
	int	    keylen,
	int	    verbose)
{
    varnumber_T	    n1, n2 = 0;
    long	    len;

    n1 = 0;
    if (var1 != NULL && rettv->v_type != VAR_DICT)
	n1 = tv_get_number(var1);

    if (is_range)
    {
	if (rettv->v_type == VAR_DICT)
	{
	    if (verbose)
		emsg(_(e_cannot_slice_dictionary));
	    return FAIL;
	}
	if (var2 != NULL)
	    n2 = tv_get_number(var2);
	else
	    n2 = VARNUM_MAX;
    }

    switch (rettv->v_type)
    {
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	case VAR_FUNC:
	case VAR_PARTIAL:
	case VAR_FLOAT:
	case VAR_BOOL:
	case VAR_SPECIAL:
	case VAR_JOB:
	case VAR_CHANNEL:
	case VAR_INSTR:
	case VAR_CLASS:
	case VAR_OBJECT:
	case VAR_TYPEALIAS:
	    break; // not evaluating, skipping over subscript

	case VAR_NUMBER:
	case VAR_STRING:
	    {
		char_u	*s = tv_get_string(rettv);

		len = (long)STRLEN(s);
		if (in_vim9script() || exclusive)
		{
		    if (is_range)
			s = string_slice(s, n1, n2, exclusive);
		    else
			s = char_from_string(s, n1);
		}
		else if (is_range)
		{
		    // The resulting variable is a substring.  If the indexes
		    // are out of range the result is empty.
		    if (n1 < 0)
		    {
			n1 = len + n1;
			if (n1 < 0)
			    n1 = 0;
		    }
		    if (n2 < 0)
			n2 = len + n2;
		    else if (n2 >= len)
			n2 = len;
		    if (n1 >= len || n2 < 0 || n1 > n2)
			s = NULL;
		    else
			s = vim_strnsave(s + n1, n2 - n1 + 1);
		}
		else
		{
		    // The resulting variable is a string of a single
		    // character.  If the index is too big or negative the
		    // result is empty.
		    if (n1 >= len || n1 < 0)
			s = NULL;
		    else
			s = vim_strnsave(s + n1, 1);
		}
		clear_tv(rettv);
		rettv->v_type = VAR_STRING;
		rettv->vval.v_string = s;
	    }
	    break;

	case VAR_BLOB:
	    blob_slice_or_index(rettv->vval.v_blob, is_range, n1, n2,
							     exclusive, rettv);
	    break;

	case VAR_LIST:
	    if (var1 == NULL)
		n1 = 0;
	    if (var2 == NULL)
		n2 = VARNUM_MAX;
	    if (list_slice_or_index(rettv->vval.v_list,
			  is_range, n1, n2, exclusive, rettv, verbose) == FAIL)
		return FAIL;
	    break;

	case VAR_DICT:
	    {
		dictitem_T	*item;
		typval_T	tmp;

		if (key == NULL)
		{
		    key = tv_get_string_chk(var1);
		    if (key == NULL)
			return FAIL;
		}

		item = dict_find(rettv->vval.v_dict, key, keylen);

		if (item == NULL)
		{
		    if (verbose)
		    {
			if (keylen > 0)
			    key[keylen] = NUL;
			semsg(_(e_key_not_present_in_dictionary_str), key);
		    }
		    return FAIL;
		}

		copy_tv(&item->di_tv, &tmp);
		clear_tv(rettv);
		*rettv = tmp;
	    }
	    break;
    }
    return OK;
}

/*
 * Return the function name of partial "pt".
 */
    char_u *
partial_name(partial_T *pt)
{
    if (pt != NULL)
    {
	if (pt->pt_name != NULL)
	    return pt->pt_name;
	if (pt->pt_func != NULL)
	    return pt->pt_func->uf_name;
    }
    return (char_u *)"";
}

    static void
partial_free(partial_T *pt)
{
    int i;

    for (i = 0; i < pt->pt_argc; ++i)
	clear_tv(&pt->pt_argv[i]);
    vim_free(pt->pt_argv);
    dict_unref(pt->pt_dict);
    if (pt->pt_name != NULL)
    {
	func_unref(pt->pt_name);
	vim_free(pt->pt_name);
    }
    else
	func_ptr_unref(pt->pt_func);
    object_unref(pt->pt_obj);

    // "out_up" is no longer used, decrement refcount on partial that owns it.
    partial_unref(pt->pt_outer.out_up_partial);

    // Using pt_outer from another partial.
    partial_unref(pt->pt_outer_partial);

    // Decrease the reference count for the context of a closure.  If down
    // to the minimum it may be time to free it.
    if (pt->pt_funcstack != NULL)
    {
	--pt->pt_funcstack->fs_refcount;
	funcstack_check_refcount(pt->pt_funcstack);
    }
    // Similarly for loop variables.
    for (i = 0; i < MAX_LOOP_DEPTH; ++i)
	if (pt->pt_loopvars[i] != NULL)
	{
	    --pt->pt_loopvars[i]->lvs_refcount;
	    loopvars_check_refcount(pt->pt_loopvars[i]);
	}

    vim_free(pt);
}

/*
 * Unreference a closure: decrement the reference count and free it when it
 * becomes zero.
 */
    void
partial_unref(partial_T *pt)
{
    if (pt == NULL)
	return;

    int	done = FALSE;

    if (--pt->pt_refcount <= 0)
	partial_free(pt);

    // If the reference count goes down to one, the funcstack may be the
    // only reference and can be freed if no other partials reference it.
    else if (pt->pt_refcount == 1)
    {
	// careful: if the funcstack is freed it may contain this partial
	// and it gets freed as well
	if (pt->pt_funcstack != NULL)
	    done = funcstack_check_refcount(pt->pt_funcstack);

	if (!done)
	{
	    int	depth;

	    for (depth = 0; depth < MAX_LOOP_DEPTH; ++depth)
		if (pt->pt_loopvars[depth] != NULL
			&& loopvars_check_refcount(pt->pt_loopvars[depth]))
		    break;
	}
    }
}

/*
 * Return the next (unique) copy ID.
 * Used for serializing nested structures.
 */
    int
get_copyID(void)
{
    current_copyID += COPYID_INC;
    return current_copyID;
}

/*
 * Garbage collection for lists and dictionaries.
 *
 * We use reference counts to be able to free most items right away when they
 * are no longer used.  But for composite items it's possible that it becomes
 * unused while the reference count is > 0: When there is a recursive
 * reference.  Example:
 *	:let l = [1, 2, 3]
 *	:let d = {9: l}
 *	:let l[1] = d
 *
 * Since this is quite unusual we handle this with garbage collection: every
 * once in a while find out which lists and dicts are not referenced from any
 * variable.
 *
 * Here is a good reference text about garbage collection (refers to Python
 * but it applies to all reference-counting mechanisms):
 *	http://python.ca/nas/python/gc/
 */

/*
 * Do garbage collection for lists and dicts.
 * When "testing" is TRUE this is called from test_garbagecollect_now().
 * Return TRUE if some memory was freed.
 */
    int
garbage_collect(int testing)
{
    int		copyID;
    int		abort = FALSE;
    buf_T	*buf;
    win_T	*wp;
    int		did_free = FALSE;
    tabpage_T	*tp;

    if (!testing)
    {
	// Only do this once.
	want_garbage_collect = FALSE;
	may_garbage_collect = FALSE;
	garbage_collect_at_exit = FALSE;
    }

    // The execution stack can grow big, limit the size.
    if (exestack.ga_maxlen - exestack.ga_len > 500)
    {
	size_t	new_len;
	char_u	*pp;
	int	n;

	// Keep 150% of the current size, with a minimum of the growth size.
	n = exestack.ga_len / 2;
	if (n < exestack.ga_growsize)
	    n = exestack.ga_growsize;

	// Don't make it bigger though.
	if (exestack.ga_len + n < exestack.ga_maxlen)
	{
	    new_len = (size_t)exestack.ga_itemsize * (exestack.ga_len + n);
	    pp = vim_realloc(exestack.ga_data, new_len);
	    if (pp == NULL)
		return FAIL;
	    exestack.ga_maxlen = exestack.ga_len + n;
	    exestack.ga_data = pp;
	}
    }

    // We advance by two because we add one for items referenced through
    // previous_funccal.
    copyID = get_copyID();

    /*
     * 1. Go through all accessible variables and mark all lists and dicts
     *    with copyID.
     */

    // Don't free variables in the previous_funccal list unless they are only
    // referenced through previous_funccal.  This must be first, because if
    // the item is referenced elsewhere the funccal must not be freed.
    abort = abort || set_ref_in_previous_funccal(copyID);

    // script-local variables
    abort = abort || garbage_collect_scriptvars(copyID);

    // buffer-local variables
    FOR_ALL_BUFFERS(buf)
	abort = abort || set_ref_in_item(&buf->b_bufvar.di_tv, copyID,
								  NULL, NULL);

    // window-local variables
    FOR_ALL_TAB_WINDOWS(tp, wp)
	abort = abort || set_ref_in_item(&wp->w_winvar.di_tv, copyID,
								  NULL, NULL);
    // window-local variables in autocmd windows
    for (int i = 0; i < AUCMD_WIN_COUNT; ++i)
	if (aucmd_win[i].auc_win != NULL)
	    abort = abort || set_ref_in_item(
		    &aucmd_win[i].auc_win->w_winvar.di_tv, copyID, NULL, NULL);
#ifdef FEAT_PROP_POPUP
    FOR_ALL_POPUPWINS(wp)
	abort = abort || set_ref_in_item(&wp->w_winvar.di_tv, copyID,
								  NULL, NULL);
    FOR_ALL_TABPAGES(tp)
	FOR_ALL_POPUPWINS_IN_TAB(tp, wp)
		abort = abort || set_ref_in_item(&wp->w_winvar.di_tv, copyID,
								  NULL, NULL);
#endif

    // tabpage-local variables
    FOR_ALL_TABPAGES(tp)
	abort = abort || set_ref_in_item(&tp->tp_winvar.di_tv, copyID,
								  NULL, NULL);
    // global variables
    abort = abort || garbage_collect_globvars(copyID);

    // function-local variables
    abort = abort || set_ref_in_call_stack(copyID);

    // named functions (matters for closures)
    abort = abort || set_ref_in_functions(copyID);

    // function call arguments, if v:testing is set.
    abort = abort || set_ref_in_func_args(copyID);

    // funcstacks keep variables for closures
    abort = abort || set_ref_in_funcstacks(copyID);

    // loopvars keep variables for loop blocks
    abort = abort || set_ref_in_loopvars(copyID);

    // v: vars
    abort = abort || garbage_collect_vimvars(copyID);

    // callbacks in buffers
    abort = abort || set_ref_in_buffers(copyID);

    // 'completefunc', 'omnifunc' and 'thesaurusfunc' callbacks
    abort = abort || set_ref_in_insexpand_funcs(copyID);

    // 'operatorfunc' callback
    abort = abort || set_ref_in_opfunc(copyID);

    // 'tagfunc' callback
    abort = abort || set_ref_in_tagfunc(copyID);

    // 'imactivatefunc' and 'imstatusfunc' callbacks
    abort = abort || set_ref_in_im_funcs(copyID);

#ifdef FEAT_LUA
    abort = abort || set_ref_in_lua(copyID);
#endif

#ifdef FEAT_PYTHON
    abort = abort || set_ref_in_python(copyID);
#endif

#ifdef FEAT_PYTHON3
    abort = abort || set_ref_in_python3(copyID);
#endif

#ifdef FEAT_JOB_CHANNEL
    abort = abort || set_ref_in_channel(copyID);
    abort = abort || set_ref_in_job(copyID);
#endif
#ifdef FEAT_NETBEANS_INTG
    abort = abort || set_ref_in_nb_channel(copyID);
#endif

#ifdef FEAT_TIMERS
    abort = abort || set_ref_in_timer(copyID);
#endif

#ifdef FEAT_QUICKFIX
    abort = abort || set_ref_in_quickfix(copyID);
#endif

#ifdef FEAT_TERMINAL
    abort = abort || set_ref_in_term(copyID);
#endif

#ifdef FEAT_PROP_POPUP
    abort = abort || set_ref_in_popups(copyID);
#endif

    abort = abort || set_ref_in_classes(copyID);

    if (!abort)
    {
	/*
	 * 2. Free lists and dictionaries that are not referenced.
	 */
	did_free = free_unref_items(copyID);

	/*
	 * 3. Check if any funccal can be freed now.
	 *    This may call us back recursively.
	 */
	free_unref_funccal(copyID, testing);
    }
    else if (p_verbose > 0)
    {
	verb_msg(_("Not enough memory to set references, garbage collection aborted!"));
    }

    return did_free;
}

/*
 * Free lists, dictionaries, channels and jobs that are no longer referenced.
 */
    static int
free_unref_items(int copyID)
{
    int		did_free = FALSE;

    // Let all "free" functions know that we are here.  This means no
    // dictionaries, lists, channels or jobs are to be freed, because we will
    // do that here.
    in_free_unref_items = TRUE;

    /*
     * PASS 1: free the contents of the items.  We don't free the items
     * themselves yet, so that it is possible to decrement refcount counters
     */

    // Go through the list of dicts and free items without this copyID.
    did_free |= dict_free_nonref(copyID);

    // Go through the list of lists and free items without this copyID.
    did_free |= list_free_nonref(copyID);

    // Go through the list of objects and free items without this copyID.
    did_free |= object_free_nonref(copyID);

    // Go through the list of classes and free items without this copyID.
    did_free |= class_free_nonref(copyID);

#ifdef FEAT_JOB_CHANNEL
    // Go through the list of jobs and free items without the copyID. This
    // must happen before doing channels, because jobs refer to channels, but
    // the reference from the channel to the job isn't tracked.
    did_free |= free_unused_jobs_contents(copyID, COPYID_MASK);

    // Go through the list of channels and free items without the copyID.
    did_free |= free_unused_channels_contents(copyID, COPYID_MASK);
#endif

    /*
     * PASS 2: free the items themselves.
     */
    object_free_items(copyID);
    dict_free_items(copyID);
    list_free_items(copyID);

#ifdef FEAT_JOB_CHANNEL
    // Go through the list of jobs and free items without the copyID. This
    // must happen before doing channels, because jobs refer to channels, but
    // the reference from the channel to the job isn't tracked.
    free_unused_jobs(copyID, COPYID_MASK);

    // Go through the list of channels and free items without the copyID.
    free_unused_channels(copyID, COPYID_MASK);
#endif

    in_free_unref_items = FALSE;

    return did_free;
}

/*
 * Mark all lists and dicts referenced through hashtab "ht" with "copyID".
 * "list_stack" is used to add lists to be marked.  Can be NULL.
 *
 * Returns TRUE if setting references failed somehow.
 */
    int
set_ref_in_ht(hashtab_T *ht, int copyID, list_stack_T **list_stack)
{
    int		todo;
    int		abort = FALSE;
    hashitem_T	*hi;
    hashtab_T	*cur_ht;
    ht_stack_T	*ht_stack = NULL;
    ht_stack_T	*tempitem;

    cur_ht = ht;
    for (;;)
    {
	if (!abort)
	{
	    // Mark each item in the hashtab.  If the item contains a hashtab
	    // it is added to ht_stack, if it contains a list it is added to
	    // list_stack.
	    todo = (int)cur_ht->ht_used;
	    FOR_ALL_HASHTAB_ITEMS(cur_ht, hi, todo)
		if (!HASHITEM_EMPTY(hi))
		{
		    --todo;
		    abort = abort || set_ref_in_item(&HI2DI(hi)->di_tv, copyID,
						       &ht_stack, list_stack);
		}
	}

	if (ht_stack == NULL)
	    break;

	// take an item from the stack
	cur_ht = ht_stack->ht;
	tempitem = ht_stack;
	ht_stack = ht_stack->prev;
	free(tempitem);
    }

    return abort;
}

#if defined(FEAT_LUA) || defined(FEAT_PYTHON) || defined(FEAT_PYTHON3) \
							|| defined(PROTO)
/*
 * Mark a dict and its items with "copyID".
 * Returns TRUE if setting references failed somehow.
 */
    int
set_ref_in_dict(dict_T *d, int copyID)
{
    if (d != NULL && d->dv_copyID != copyID)
    {
	d->dv_copyID = copyID;
	return set_ref_in_ht(&d->dv_hashtab, copyID, NULL);
    }
    return FALSE;
}
#endif

/*
 * Mark a list and its items with "copyID".
 * Returns TRUE if setting references failed somehow.
 */
    int
set_ref_in_list(list_T *ll, int copyID)
{
    if (ll != NULL && ll->lv_copyID != copyID)
    {
	ll->lv_copyID = copyID;
	return set_ref_in_list_items(ll, copyID, NULL);
    }
    return FALSE;
}

/*
 * Mark all lists and dicts referenced through list "l" with "copyID".
 * "ht_stack" is used to add hashtabs to be marked.  Can be NULL.
 *
 * Returns TRUE if setting references failed somehow.
 */
    int
set_ref_in_list_items(list_T *l, int copyID, ht_stack_T **ht_stack)
{
    listitem_T	 *li;
    int		 abort = FALSE;
    list_T	 *cur_l;
    list_stack_T *list_stack = NULL;
    list_stack_T *tempitem;

    cur_l = l;
    for (;;)
    {
	if (!abort && cur_l->lv_first != &range_list_item)
	    // Mark each item in the list.  If the item contains a hashtab
	    // it is added to ht_stack, if it contains a list it is added to
	    // list_stack.
	    for (li = cur_l->lv_first; !abort && li != NULL; li = li->li_next)
		abort = abort || set_ref_in_item(&li->li_tv, copyID,
						       ht_stack, &list_stack);
	if (list_stack == NULL)
	    break;

	// take an item from the stack
	cur_l = list_stack->list;
	tempitem = list_stack;
	list_stack = list_stack->prev;
	free(tempitem);
    }

    return abort;
}

/*
 * Mark the partial in callback 'cb' with "copyID".
 */
    int
set_ref_in_callback(callback_T *cb, int copyID)
{
    typval_T tv;

    if (cb->cb_name == NULL || *cb->cb_name == NUL || cb->cb_partial == NULL)
	return FALSE;

    tv.v_type = VAR_PARTIAL;
    tv.vval.v_partial = cb->cb_partial;
    return set_ref_in_item(&tv, copyID, NULL, NULL);
}

/*
 * Mark the dict "dd" with "copyID".
 * Also see set_ref_in_item().
 */
    static int
set_ref_in_item_dict(
    dict_T		*dd,
    int			copyID,
    ht_stack_T		**ht_stack,
    list_stack_T	**list_stack)
{
    if (dd == NULL || dd->dv_copyID == copyID)
	return FALSE;

    // Didn't see this dict yet.
    dd->dv_copyID = copyID;
    if (ht_stack == NULL)
	return set_ref_in_ht(&dd->dv_hashtab, copyID, list_stack);

    ht_stack_T *newitem = ALLOC_ONE(ht_stack_T);
    if (newitem == NULL)
	return TRUE;

    newitem->ht = &dd->dv_hashtab;
    newitem->prev = *ht_stack;
    *ht_stack = newitem;

    return FALSE;
}

/*
 * Mark the list "ll" with "copyID".
 * Also see set_ref_in_item().
 */
    static int
set_ref_in_item_list(
    list_T		*ll,
    int			copyID,
    ht_stack_T		**ht_stack,
    list_stack_T	**list_stack)
{
    if (ll == NULL || ll->lv_copyID == copyID)
	return FALSE;

    // Didn't see this list yet.
    ll->lv_copyID = copyID;
    if (list_stack == NULL)
	return set_ref_in_list_items(ll, copyID, ht_stack);

    list_stack_T *newitem = ALLOC_ONE(list_stack_T);
    if (newitem == NULL)
	return TRUE;

    newitem->list = ll;
    newitem->prev = *list_stack;
    *list_stack = newitem;

    return FALSE;
}

/*
 * Mark the partial "pt" with "copyID".
 * Also see set_ref_in_item().
 */
    static int
set_ref_in_item_partial(
    partial_T		*pt,
    int			copyID,
    ht_stack_T		**ht_stack,
    list_stack_T	**list_stack)
{
    if (pt == NULL || pt->pt_copyID == copyID)
	return FALSE;

    // Didn't see this partial yet.
    pt->pt_copyID = copyID;

    int abort = set_ref_in_func(pt->pt_name, pt->pt_func, copyID);

    if (pt->pt_dict != NULL)
    {
	typval_T dtv;

	dtv.v_type = VAR_DICT;
	dtv.vval.v_dict = pt->pt_dict;
	set_ref_in_item(&dtv, copyID, ht_stack, list_stack);
    }

    if (pt->pt_obj != NULL)
    {
	typval_T objtv;

	objtv.v_type = VAR_OBJECT;
	objtv.vval.v_object = pt->pt_obj;
	set_ref_in_item(&objtv, copyID, ht_stack, list_stack);
    }

    for (int i = 0; i < pt->pt_argc; ++i)
	abort = abort || set_ref_in_item(&pt->pt_argv[i], copyID,
		ht_stack, list_stack);
    // pt_funcstack is handled in set_ref_in_funcstacks()
    // pt_loopvars is handled in set_ref_in_loopvars()

    return abort;
}

#ifdef FEAT_JOB_CHANNEL
/*
 * Mark the job "pt" with "copyID".
 * Also see set_ref_in_item().
 */
    static int
set_ref_in_item_job(
    job_T		*job,
    int			copyID,
    ht_stack_T		**ht_stack,
    list_stack_T	**list_stack)
{
    typval_T    dtv;

    if (job == NULL || job->jv_copyID == copyID)
	return FALSE;

    job->jv_copyID = copyID;
    if (job->jv_channel != NULL)
    {
	dtv.v_type = VAR_CHANNEL;
	dtv.vval.v_channel = job->jv_channel;
	set_ref_in_item(&dtv, copyID, ht_stack, list_stack);
    }
    if (job->jv_exit_cb.cb_partial != NULL)
    {
	dtv.v_type = VAR_PARTIAL;
	dtv.vval.v_partial = job->jv_exit_cb.cb_partial;
	set_ref_in_item(&dtv, copyID, ht_stack, list_stack);
    }

    return FALSE;
}

/*
 * Mark the channel "ch" with "copyID".
 * Also see set_ref_in_item().
 */
    static int
set_ref_in_item_channel(
    channel_T		*ch,
    int			copyID,
    ht_stack_T		**ht_stack,
    list_stack_T	**list_stack)
{
    typval_T    dtv;

    if (ch == NULL || ch->ch_copyID == copyID)
	return FALSE;

    ch->ch_copyID = copyID;
    for (ch_part_T part = PART_SOCK; part < PART_COUNT; ++part)
    {
	for (jsonq_T *jq = ch->ch_part[part].ch_json_head.jq_next;
		jq != NULL; jq = jq->jq_next)
	    set_ref_in_item(jq->jq_value, copyID, ht_stack, list_stack);
	for (cbq_T *cq = ch->ch_part[part].ch_cb_head.cq_next; cq != NULL;
		cq = cq->cq_next)
	    if (cq->cq_callback.cb_partial != NULL)
	    {
		dtv.v_type = VAR_PARTIAL;
		dtv.vval.v_partial = cq->cq_callback.cb_partial;
		set_ref_in_item(&dtv, copyID, ht_stack, list_stack);
	    }
	if (ch->ch_part[part].ch_callback.cb_partial != NULL)
	{
	    dtv.v_type = VAR_PARTIAL;
	    dtv.vval.v_partial = ch->ch_part[part].ch_callback.cb_partial;
	    set_ref_in_item(&dtv, copyID, ht_stack, list_stack);
	}
    }
    if (ch->ch_callback.cb_partial != NULL)
    {
	dtv.v_type = VAR_PARTIAL;
	dtv.vval.v_partial = ch->ch_callback.cb_partial;
	set_ref_in_item(&dtv, copyID, ht_stack, list_stack);
    }
    if (ch->ch_close_cb.cb_partial != NULL)
    {
	dtv.v_type = VAR_PARTIAL;
	dtv.vval.v_partial = ch->ch_close_cb.cb_partial;
	set_ref_in_item(&dtv, copyID, ht_stack, list_stack);
    }

    return FALSE;
}
#endif

/*
 * Mark the class "cl" with "copyID".
 * Also see set_ref_in_item().
 */
    int
set_ref_in_item_class(
    class_T		*cl,
    int			copyID,
    ht_stack_T		**ht_stack,
    list_stack_T	**list_stack)
{
    int abort = FALSE;

    if (cl == NULL || cl->class_copyID == copyID)
	return FALSE;

    cl->class_copyID = copyID;
    if (cl->class_members_tv != NULL)
    {
	// The "class_members_tv" table is allocated only for regular classes
	// and not for interfaces.
	for (int i = 0; !abort && i < cl->class_class_member_count; ++i)
	    abort = abort || set_ref_in_item(
		    &cl->class_members_tv[i],
		    copyID, ht_stack, list_stack);
    }

    for (int i = 0; !abort && i < cl->class_class_function_count; ++i)
	abort = abort || set_ref_in_func(NULL,
		cl->class_class_functions[i], copyID);

    for (int i = 0; !abort && i < cl->class_obj_method_count; ++i)
	abort = abort || set_ref_in_func(NULL,
		cl->class_obj_methods[i], copyID);

    return abort;
}

/*
 * Mark the object "cl" with "copyID".
 * Also see set_ref_in_item().
 */
    static int
set_ref_in_item_object(
    object_T		*obj,
    int			copyID,
    ht_stack_T		**ht_stack,
    list_stack_T	**list_stack)
{
    int abort = FALSE;

    if (obj == NULL || obj->obj_copyID == copyID)
	return FALSE;

    obj->obj_copyID = copyID;

    // The typval_T array is right after the object_T.
    typval_T *mtv = (typval_T *)(obj + 1);
    for (int i = 0; !abort
	    && i < obj->obj_class->class_obj_member_count; ++i)
	abort = abort || set_ref_in_item(mtv + i, copyID,
		ht_stack, list_stack);

    return abort;
}

/*
 * Mark all lists, dicts and other container types referenced through typval
 * "tv" with "copyID".
 * "list_stack" is used to add lists to be marked.  Can be NULL.
 * "ht_stack" is used to add hashtabs to be marked.  Can be NULL.
 *
 * Returns TRUE if setting references failed somehow.
 */
    int
set_ref_in_item(
    typval_T	    *tv,
    int		    copyID,
    ht_stack_T	    **ht_stack,
    list_stack_T    **list_stack)
{
    int		abort = FALSE;

    switch (tv->v_type)
    {
	case VAR_DICT:
	    return set_ref_in_item_dict(tv->vval.v_dict, copyID,
							 ht_stack, list_stack);

	case VAR_LIST:
	    return set_ref_in_item_list(tv->vval.v_list, copyID,
							 ht_stack, list_stack);

	case VAR_FUNC:
	{
	    abort = set_ref_in_func(tv->vval.v_string, NULL, copyID);
	    break;
	}

	case VAR_PARTIAL:
	    return set_ref_in_item_partial(tv->vval.v_partial, copyID,
							ht_stack, list_stack);

	case VAR_JOB:
#ifdef FEAT_JOB_CHANNEL
	    return set_ref_in_item_job(tv->vval.v_job, copyID,
							 ht_stack, list_stack);
#else
	    break;
#endif

	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    return set_ref_in_item_channel(tv->vval.v_channel, copyID,
							 ht_stack, list_stack);
#else
	    break;
#endif

	case VAR_CLASS:
	    return set_ref_in_item_class(tv->vval.v_class, copyID,
							 ht_stack, list_stack);

	case VAR_OBJECT:
	    return set_ref_in_item_object(tv->vval.v_object, copyID,
							 ht_stack, list_stack);

	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	case VAR_BOOL:
	case VAR_SPECIAL:
	case VAR_NUMBER:
	case VAR_FLOAT:
	case VAR_STRING:
	case VAR_BLOB:
	case VAR_TYPEALIAS:
	case VAR_INSTR:
	    // Types that do not contain any other item
	    break;
    }

    return abort;
}

/*
 * Return a string with the string representation of a variable.
 * If the memory is allocated "tofree" is set to it, otherwise NULL.
 * "numbuf" is used for a number.
 * When "copyID" is not NULL replace recursive lists and dicts with "...".
 * When both "echo_style" and "composite_val" are FALSE, put quotes around
 * strings as "string()", otherwise does not put quotes around strings, as
 * ":echo" displays values.
 * When "restore_copyID" is FALSE, repeated items in dictionaries and lists
 * are replaced with "...".
 * May return NULL.
 */
    char_u *
echo_string_core(
    typval_T	*tv,
    char_u	**tofree,
    char_u	*numbuf,
    int		copyID,
    int		echo_style,
    int		restore_copyID,
    int		composite_val)
{
    static int	recurse = 0;
    char_u	*r = NULL;

    if (recurse >= DICT_MAXNEST)
    {
	if (!did_echo_string_emsg)
	{
	    // Only give this message once for a recursive call to avoid
	    // flooding the user with errors.  And stop iterating over lists
	    // and dicts.
	    did_echo_string_emsg = TRUE;
	    emsg(_(e_variable_nested_too_deep_for_displaying));
	}
	*tofree = NULL;
	return (char_u *)"{E724}";
    }
    ++recurse;

    switch (tv->v_type)
    {
	case VAR_STRING:
	    if (echo_style && !composite_val)
	    {
		*tofree = NULL;
		r = tv->vval.v_string;
		if (r == NULL)
		    r = (char_u *)"";
	    }
	    else
	    {
		*tofree = string_quote(tv->vval.v_string, FALSE);
		r = *tofree;
	    }
	    break;

	case VAR_FUNC:
	    {
		char_u buf[MAX_FUNC_NAME_LEN];

		if (echo_style)
		{
		    r = tv->vval.v_string == NULL ? (char_u *)"function()"
				  : make_ufunc_name_readable(tv->vval.v_string,
						       buf, MAX_FUNC_NAME_LEN);
		    if (r == buf)
		    {
			r = vim_strsave(buf);
			*tofree = r;
		    }
		    else
			*tofree = NULL;
		}
		else
		{
		    *tofree = string_quote(tv->vval.v_string == NULL ? NULL
			    : make_ufunc_name_readable(
				tv->vval.v_string, buf, MAX_FUNC_NAME_LEN),
									 TRUE);
		    r = *tofree;
		}
	    }
	    break;

	case VAR_PARTIAL:
	    {
		partial_T   *pt = tv->vval.v_partial;
		char_u	    *fname = string_quote(pt == NULL ? NULL
						    : partial_name(pt), FALSE);
		garray_T    ga;
		int	    i;
		char_u	    *tf;

		ga_init2(&ga, 1, 100);
		ga_concat(&ga, (char_u *)"function(");
		if (fname != NULL)
		{
		    // When using uf_name prepend "g:" for a global function.
		    if (pt != NULL && pt->pt_name == NULL && fname[0] == '\''
						      && vim_isupper(fname[1]))
		    {
			ga_concat(&ga, (char_u *)"'g:");
			ga_concat(&ga, fname + 1);
		    }
		    else
			ga_concat(&ga, fname);
		    vim_free(fname);
		}
		if (pt != NULL && pt->pt_argc > 0)
		{
		    ga_concat(&ga, (char_u *)", [");
		    for (i = 0; i < pt->pt_argc; ++i)
		    {
			if (i > 0)
			    ga_concat(&ga, (char_u *)", ");
			ga_concat(&ga,
			     tv2string(&pt->pt_argv[i], &tf, numbuf, copyID));
			vim_free(tf);
		    }
		    ga_concat(&ga, (char_u *)"]");
		}
		if (pt != NULL && pt->pt_dict != NULL)
		{
		    typval_T dtv;

		    ga_concat(&ga, (char_u *)", ");
		    dtv.v_type = VAR_DICT;
		    dtv.vval.v_dict = pt->pt_dict;
		    ga_concat(&ga, tv2string(&dtv, &tf, numbuf, copyID));
		    vim_free(tf);
		}
		// terminate with ')' and a NUL
		ga_concat_len(&ga, (char_u *)")", 2);

		*tofree = ga.ga_data;
		r = *tofree;
		break;
	    }

	case VAR_BLOB:
	    r = blob2string(tv->vval.v_blob, tofree, numbuf);
	    break;

	case VAR_LIST:
	    if (tv->vval.v_list == NULL)
	    {
		// NULL list is equivalent to empty list.
		*tofree = NULL;
		r = (char_u *)"[]";
	    }
	    else if (copyID != 0 && tv->vval.v_list->lv_copyID == copyID
		    && tv->vval.v_list->lv_len > 0)
	    {
		*tofree = NULL;
		r = (char_u *)"[...]";
	    }
	    else
	    {
		int old_copyID = tv->vval.v_list->lv_copyID;

		tv->vval.v_list->lv_copyID = copyID;
		*tofree = list2string(tv, copyID, restore_copyID);
		if (restore_copyID)
		    tv->vval.v_list->lv_copyID = old_copyID;
		r = *tofree;
	    }
	    break;

	case VAR_DICT:
	    if (tv->vval.v_dict == NULL)
	    {
		// NULL dict is equivalent to empty dict.
		*tofree = NULL;
		r = (char_u *)"{}";
	    }
	    else if (copyID != 0 && tv->vval.v_dict->dv_copyID == copyID
		    && tv->vval.v_dict->dv_hashtab.ht_used != 0)
	    {
		*tofree = NULL;
		r = (char_u *)"{...}";
	    }
	    else
	    {
		int old_copyID = tv->vval.v_dict->dv_copyID;

		tv->vval.v_dict->dv_copyID = copyID;
		*tofree = dict2string(tv, copyID, restore_copyID);
		if (restore_copyID)
		    tv->vval.v_dict->dv_copyID = old_copyID;
		r = *tofree;
	    }
	    break;

	case VAR_NUMBER:
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	    *tofree = NULL;
	    r = tv_get_string_buf(tv, numbuf);
	    break;

	case VAR_JOB:
	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    *tofree = NULL;
	    r = tv->v_type == VAR_JOB ? job_to_string_buf(tv, numbuf)
					   : channel_to_string_buf(tv, numbuf);
	    if (composite_val)
	    {
		*tofree = string_quote(r, FALSE);
		r = *tofree;
	    }
#endif
	    break;

	case VAR_INSTR:
	    *tofree = NULL;
	    r = (char_u *)"instructions";
	    break;

	case VAR_CLASS:
	    {
		class_T *cl = tv->vval.v_class;
		size_t len = 6 + (cl == NULL ? 9 : STRLEN(cl->class_name)) + 1;
		r = *tofree = alloc(len);
		vim_snprintf((char *)r, len, "class %s",
			    cl == NULL ? "[unknown]" : (char *)cl->class_name);
	    }
	    break;

	case VAR_OBJECT:
	    {
		garray_T ga;
		ga_init2(&ga, 1, 50);
		ga_concat(&ga, (char_u *)"object of ");
		object_T *obj = tv->vval.v_object;
		class_T *cl = obj == NULL ? NULL : obj->obj_class;
		ga_concat(&ga, cl == NULL ? (char_u *)"[unknown]"
							     : cl->class_name);
		if (cl != NULL)
		{
		    ga_concat(&ga, (char_u *)" {");
		    for (int i = 0; i < cl->class_obj_member_count; ++i)
		    {
			if (i > 0)
			    ga_concat(&ga, (char_u *)", ");
			ocmember_T *m = &cl->class_obj_members[i];
			ga_concat(&ga, m->ocm_name);
			ga_concat(&ga, (char_u *)": ");
			char_u *tf = NULL;
			ga_concat(&ga, echo_string_core(
					       (typval_T *)(obj + 1) + i,
					       &tf, numbuf, copyID, echo_style,
					       restore_copyID, composite_val));
			vim_free(tf);
		    }
		    ga_concat(&ga, (char_u *)"}");
		}

		*tofree = r = ga.ga_data;
	    }
	    break;

	case VAR_FLOAT:
	    *tofree = NULL;
	    vim_snprintf((char *)numbuf, NUMBUFLEN, "%g", tv->vval.v_float);
	    r = numbuf;
	    break;

	case VAR_BOOL:
	case VAR_SPECIAL:
	    *tofree = NULL;
	    r = (char_u *)get_var_special_name(tv->vval.v_number);
	    break;

	case VAR_TYPEALIAS:
	    *tofree = vim_strsave(tv->vval.v_typealias->ta_name);
	    r = *tofree;
	    if (r == NULL)
		r = (char_u *)"";
	    break;
    }

    if (--recurse == 0)
	did_echo_string_emsg = FALSE;
    return r;
}

/*
 * Return a string with the string representation of a variable.
 * If the memory is allocated "tofree" is set to it, otherwise NULL.
 * "numbuf" is used for a number.
 * Does not put quotes around strings, as ":echo" displays values.
 * When "copyID" is not NULL replace recursive lists and dicts with "...".
 * May return NULL.
 */
    char_u *
echo_string(
    typval_T	*tv,
    char_u	**tofree,
    char_u	*numbuf,
    int		copyID)
{
    return echo_string_core(tv, tofree, numbuf, copyID, TRUE, FALSE, FALSE);
}

/*
 * Convert the specified byte index of line 'lnum' in buffer 'buf' to a
 * character index.  Works only for loaded buffers. Returns -1 on failure.
 * The index of the first byte and the first character is zero.
 */
    int
buf_byteidx_to_charidx(buf_T *buf, int lnum, int byteidx)
{
    char_u	*str;
    char_u	*t;
    int		count;

    if (buf == NULL || buf->b_ml.ml_mfp == NULL)
	return -1;

    if (lnum > buf->b_ml.ml_line_count)
	lnum = buf->b_ml.ml_line_count;

    str = ml_get_buf(buf, lnum, FALSE);
    if (str == NULL)
	return -1;

    if (*str == NUL)
	return 0;

    // count the number of characters
    t = str;
    for (count = 0; *t != NUL && t <= str + byteidx; count++)
	t += mb_ptr2len(t);

    // In insert mode, when the cursor is at the end of a non-empty line,
    // byteidx points to the NUL character immediately past the end of the
    // string. In this case, add one to the character count.
    if (*t == NUL && byteidx != 0 && t == str + byteidx)
	count++;

    return count - 1;
}

/*
 * Convert the specified character index of line 'lnum' in buffer 'buf' to a
 * byte index.  Works only for loaded buffers. Returns -1 on failure.
 * The index of the first byte and the first character is zero.
 */
    int
buf_charidx_to_byteidx(buf_T *buf, int lnum, int charidx)
{
    char_u	*str;
    char_u	*t;

    if (buf == NULL || buf->b_ml.ml_mfp == NULL)
	return -1;

    if (lnum > buf->b_ml.ml_line_count)
	lnum = buf->b_ml.ml_line_count;

    str = ml_get_buf(buf, lnum, FALSE);
    if (str == NULL)
	return -1;

    // Convert the character offset to a byte offset
    t = str;
    while (*t != NUL && --charidx > 0)
	t += mb_ptr2len(t);

    return t - str;
}

/*
 * Translate a String variable into a position.
 * Returns NULL when there is an error.
 */
    pos_T *
var2fpos(
    typval_T	*varp,
    int		dollar_lnum,	// TRUE when $ is last line
    int		*fnum,		// set to fnum for '0, 'A, etc.
    int		charcol)	// return character column
{
    char_u		*name;
    static pos_T	pos;
    pos_T		*pp;

    // Argument can be [lnum, col, coladd].
    if (varp->v_type == VAR_LIST)
    {
	list_T		*l;
	int		len;
	int		error = FALSE;
	listitem_T	*li;

	l = varp->vval.v_list;
	if (l == NULL)
	    return NULL;

	// Get the line number
	pos.lnum = list_find_nr(l, 0L, &error);
	if (error || pos.lnum <= 0 || pos.lnum > curbuf->b_ml.ml_line_count)
	    return NULL;	// invalid line number
	if (charcol)
	    len = (long)mb_charlen(ml_get(pos.lnum));
	else
	    len = (long)STRLEN(ml_get(pos.lnum));

	// Get the column number
	// We accept "$" for the column number: last column.
	li = list_find(l, 1L);
	if (li != NULL && li->li_tv.v_type == VAR_STRING
		&& li->li_tv.vval.v_string != NULL
		&& STRCMP(li->li_tv.vval.v_string, "$") == 0)
	{
	    pos.col = len + 1;
	}
	else
	{
	    pos.col = list_find_nr(l, 1L, &error);
	    if (error)
		return NULL;
	}

	// Accept a position up to the NUL after the line.
	if (pos.col == 0 || (int)pos.col > len + 1)
	    return NULL;	// invalid column number
	--pos.col;

	// Get the virtual offset.  Defaults to zero.
	pos.coladd = list_find_nr(l, 2L, &error);
	if (error)
	    pos.coladd = 0;

	return &pos;
    }

    if (in_vim9script() && check_for_string_arg(varp, 0) == FAIL)
	return NULL;

    name = tv_get_string_chk(varp);
    if (name == NULL)
	return NULL;

    pos.lnum = 0;
    if (name[0] == '.' && (!in_vim9script() || name[1] == NUL))
    {
	// cursor
	pos = curwin->w_cursor;
    }
    else if (name[0] == 'v' && name[1] == NUL)
    {
	// Visual start
	if (VIsual_active)
	    pos = VIsual;
	else
	    pos = curwin->w_cursor;
    }
    else if (name[0] == '\'' && (!in_vim9script()
					|| (name[1] != NUL && name[2] == NUL)))
    {
	// mark
	pp = getmark_buf_fnum(curbuf, name[1], FALSE, fnum);
	if (pp == NULL || pp == (pos_T *)-1 || pp->lnum <= 0)
	    return NULL;
	pos = *pp;
    }
    if (pos.lnum != 0)
    {
	if (charcol)
	    pos.col = buf_byteidx_to_charidx(curbuf, pos.lnum, pos.col);
	return &pos;
    }

    pos.coladd = 0;

    if (name[0] == 'w' && dollar_lnum)
    {
	// the "w_valid" flags are not reset when moving the cursor, but they
	// do matter for update_topline() and validate_botline().
	check_cursor_moved(curwin);

	pos.col = 0;
	if (name[1] == '0')		// "w0": first visible line
	{
	    update_topline();
	    // In silent Ex mode topline is zero, but that's not a valid line
	    // number; use one instead.
	    pos.lnum = curwin->w_topline > 0 ? curwin->w_topline : 1;
	    return &pos;
	}
	else if (name[1] == '$')	// "w$": last visible line
	{
	    validate_botline();
	    // In silent Ex mode botline is zero, return zero then.
	    pos.lnum = curwin->w_botline > 0 ? curwin->w_botline - 1 : 0;
	    return &pos;
	}
    }
    else if (name[0] == '$')		// last column or line
    {
	if (dollar_lnum)
	{
	    pos.lnum = curbuf->b_ml.ml_line_count;
	    pos.col = 0;
	}
	else
	{
	    pos.lnum = curwin->w_cursor.lnum;
	    if (charcol)
		pos.col = (colnr_T)mb_charlen(ml_get_curline());
	    else
		pos.col = (colnr_T)STRLEN(ml_get_curline());
	}
	return &pos;
    }
    if (in_vim9script())
	semsg(_(e_invalid_value_for_line_number_str), name);
    return NULL;
}

/*
 * Convert list in "arg" into position "posp" and optional file number "fnump".
 * When "fnump" is NULL there is no file number, only 3 items: [lnum, col, off]
 * Note that the column is passed on as-is, the caller may want to decrement
 * it to use 1 for the first column.
 * If "charcol" is TRUE use the column as the character index instead of the
 * byte index.
 * Return FAIL when conversion is not possible, doesn't check the position for
 * validity.
 */
    int
list2fpos(
    typval_T	*arg,
    pos_T	*posp,
    int		*fnump,
    colnr_T	*curswantp,
    int		charcol)
{
    list_T	*l = arg->vval.v_list;
    long	i = 0;
    long	n;

    // List must be: [fnum, lnum, col, coladd, curswant], where "fnum" is only
    // there when "fnump" isn't NULL; "coladd" and "curswant" are optional.
    if (arg->v_type != VAR_LIST
	    || l == NULL
	    || l->lv_len < (fnump == NULL ? 2 : 3)
	    || l->lv_len > (fnump == NULL ? 4 : 5))
	return FAIL;

    if (fnump != NULL)
    {
	n = list_find_nr(l, i++, NULL);	// fnum
	if (n < 0)
	    return FAIL;
	if (n == 0)
	    n = curbuf->b_fnum;		// current buffer
	*fnump = n;
    }

    n = list_find_nr(l, i++, NULL);	// lnum
    if (n < 0)
	return FAIL;
    posp->lnum = n;

    n = list_find_nr(l, i++, NULL);	// col
    if (n < 0)
	return FAIL;
    // If character position is specified, then convert to byte position
    // If the line number is zero use the cursor line.
    if (charcol)
    {
	buf_T	*buf;

	// Get the text for the specified line in a loaded buffer
	buf = buflist_findnr(fnump == NULL ? curbuf->b_fnum : *fnump);
	if (buf == NULL || buf->b_ml.ml_mfp == NULL)
	    return FAIL;

	n = buf_charidx_to_byteidx(buf,
		  posp->lnum == 0 ? curwin->w_cursor.lnum : posp->lnum, n) + 1;
    }
    posp->col = n;

    n = list_find_nr(l, i, NULL);	// off
    if (n < 0)
	posp->coladd = 0;
    else
	posp->coladd = n;

    if (curswantp != NULL)
	*curswantp = list_find_nr(l, i + 1, NULL);  // curswant

    return OK;
}

/*
 * Get the length of an environment variable name.
 * Advance "arg" to the first character after the name.
 * Return 0 for error.
 */
    int
get_env_len(char_u **arg)
{
    char_u	*p;
    int		len;

    for (p = *arg; vim_isIDc(*p); ++p)
	;
    if (p == *arg)	    // no name found
	return 0;

    len = (int)(p - *arg);
    *arg = p;
    return len;
}

/*
 * Get the length of the name of a function or internal variable.
 * "arg" is advanced to after the name.
 * Return 0 if something is wrong.
 */
    int
get_id_len(char_u **arg)
{
    char_u	*p;
    int		len;

    // Find the end of the name.
    for (p = *arg; eval_isnamec(*p); ++p)
    {
	if (*p == ':')
	{
	    // "s:" is start of "s:var", but "n:" is not and can be used in
	    // slice "[n:]".  Also "xx:" is not a namespace.
	    len = (int)(p - *arg);
	    if ((len == 1 && vim_strchr(NAMESPACE_CHAR, **arg) == NULL)
		    || len > 1)
		break;
	}
    }
    if (p == *arg)	    // no name found
	return 0;

    len = (int)(p - *arg);
    *arg = p;

    return len;
}

/*
 * Get the length of the name of a variable or function.
 * Only the name is recognized, does not handle ".key" or "[idx]".
 * "arg" is advanced to the first non-white character after the name.
 * Return -1 if curly braces expansion failed.
 * Return 0 if something else is wrong.
 * If the name contains 'magic' {}'s, expand them and return the
 * expanded name in an allocated string via 'alias' - caller must free.
 */
    int
get_name_len(
    char_u	**arg,
    char_u	**alias,
    int		evaluate,
    int		verbose)
{
    int		len;
    char_u	*p;
    char_u	*expr_start;
    char_u	*expr_end;

    *alias = NULL;  // default to no alias

    if ((*arg)[0] == K_SPECIAL && (*arg)[1] == KS_EXTRA
						  && (*arg)[2] == (int)KE_SNR)
    {
	// hard coded <SNR>, already translated
	*arg += 3;
	return get_id_len(arg) + 3;
    }
    len = eval_fname_script(*arg);
    if (len > 0)
    {
	// literal "<SID>", "s:" or "<SNR>"
	*arg += len;
    }

    /*
     * Find the end of the name; check for {} construction.
     */
    p = find_name_end(*arg, &expr_start, &expr_end,
					       len > 0 ? 0 : FNE_CHECK_START);
    if (expr_start != NULL)
    {
	char_u	*temp_string;

	if (!evaluate)
	{
	    len += (int)(p - *arg);
	    *arg = skipwhite(p);
	    return len;
	}

	/*
	 * Include any <SID> etc in the expanded string:
	 * Thus the -len here.
	 */
	temp_string = make_expanded_name(*arg - len, expr_start, expr_end, p);
	if (temp_string == NULL)
	    return -1;
	*alias = temp_string;
	*arg = skipwhite(p);
	return (int)STRLEN(temp_string);
    }

    len += get_id_len(arg);
    // Only give an error when there is something, otherwise it will be
    // reported at a higher level.
    if (len == 0 && verbose && **arg != NUL)
	semsg(_(e_invalid_expression_str), *arg);

    return len;
}

/*
 * Find the end of a variable or function name, taking care of magic braces.
 * If "expr_start" is not NULL then "expr_start" and "expr_end" are set to the
 * start and end of the first magic braces item.
 * "flags" can have FNE_INCL_BR and FNE_CHECK_START.
 * Return a pointer to just after the name.  Equal to "arg" if there is no
 * valid name.
 */
    char_u *
find_name_end(
    char_u	*arg,
    char_u	**expr_start,
    char_u	**expr_end,
    int		flags)
{
    int		mb_nest = 0;
    int		br_nest = 0;
    char_u	*p;
    int		len;
    int		allow_curly = (flags & FNE_ALLOW_CURLY) || !in_vim9script();

    if (expr_start != NULL)
    {
	*expr_start = NULL;
	*expr_end = NULL;
    }

    // Quick check for valid starting character.
    if ((flags & FNE_CHECK_START) && !eval_isnamec1(*arg)
					      && (*arg != '{' || !allow_curly))
	return arg;

    for (p = arg; *p != NUL
		    && (eval_isnamec(*p)
			|| (*p == '{' && allow_curly)
			|| ((flags & FNE_INCL_BR) && (*p == '['
					 || (*p == '.' && eval_isdictc(p[1]))))
			|| mb_nest != 0
			|| br_nest != 0); MB_PTR_ADV(p))
    {
	if (*p == '\'')
	{
	    // skip over 'string' to avoid counting [ and ] inside it.
	    for (p = p + 1; *p != NUL && *p != '\''; MB_PTR_ADV(p))
		;
	    if (*p == NUL)
		break;
	}
	else if (*p == '"')
	{
	    // skip over "str\"ing" to avoid counting [ and ] inside it.
	    for (p = p + 1; *p != NUL && *p != '"'; MB_PTR_ADV(p))
		if (*p == '\\' && p[1] != NUL)
		    ++p;
	    if (*p == NUL)
		break;
	}
	else if (br_nest == 0 && mb_nest == 0 && *p == ':')
	{
	    // "s:" is start of "s:var", but "n:" is not and can be used in
	    // slice "[n:]".  Also "xx:" is not a namespace. But {ns}: is.
	    len = (int)(p - arg);
	    if ((len == 1 && vim_strchr(NAMESPACE_CHAR, *arg) == NULL)
		    || (len > 1 && p[-1] != '}'))
		break;
	}

	if (mb_nest == 0)
	{
	    if (*p == '[')
		++br_nest;
	    else if (*p == ']')
		--br_nest;
	}

	if (br_nest == 0 && allow_curly)
	{
	    if (*p == '{')
	    {
		mb_nest++;
		if (expr_start != NULL && *expr_start == NULL)
		    *expr_start = p;
	    }
	    else if (*p == '}')
	    {
		mb_nest--;
		if (expr_start != NULL && mb_nest == 0 && *expr_end == NULL)
		    *expr_end = p;
	    }
	}
    }

    return p;
}

/*
 * Expands out the 'magic' {}'s in a variable/function name.
 * Note that this can call itself recursively, to deal with
 * constructs like foo{bar}{baz}{bam}
 * The four pointer arguments point to "foo{expre}ss{ion}bar"
 *			"in_start"      ^
 *			"expr_start"	   ^
 *			"expr_end"		 ^
 *			"in_end"			    ^
 *
 * Returns a new allocated string, which the caller must free.
 * Returns NULL for failure.
 */
    static char_u *
make_expanded_name(
    char_u	*in_start,
    char_u	*expr_start,
    char_u	*expr_end,
    char_u	*in_end)
{
    char_u	c1;
    char_u	*retval = NULL;
    char_u	*temp_result;

    if (expr_end == NULL || in_end == NULL)
	return NULL;
    *expr_start	= NUL;
    *expr_end = NUL;
    c1 = *in_end;
    *in_end = NUL;

    temp_result = eval_to_string(expr_start + 1, FALSE, FALSE);
    if (temp_result != NULL)
    {
	retval = alloc(STRLEN(temp_result) + (expr_start - in_start)
						   + (in_end - expr_end) + 1);
	if (retval != NULL)
	{
	    STRCPY(retval, in_start);
	    STRCAT(retval, temp_result);
	    STRCAT(retval, expr_end + 1);
	}
    }
    vim_free(temp_result);

    *in_end = c1;		// put char back for error messages
    *expr_start = '{';
    *expr_end = '}';

    if (retval != NULL)
    {
	temp_result = find_name_end(retval, &expr_start, &expr_end, 0);
	if (expr_start != NULL)
	{
	    // Further expansion!
	    temp_result = make_expanded_name(retval, expr_start,
						       expr_end, temp_result);
	    vim_free(retval);
	    retval = temp_result;
	}
    }

    return retval;
}

/*
 * Return TRUE if character "c" can be used in a variable or function name.
 * Does not include '{' or '}' for magic braces.
 */
    int
eval_isnamec(int c)
{
    return ASCII_ISALNUM(c) || c == '_' || c == ':' || c == AUTOLOAD_CHAR;
}

/*
 * Return TRUE if character "c" can be used as the first character in a
 * variable or function name (excluding '{' and '}').
 */
    int
eval_isnamec1(int c)
{
    return ASCII_ISALPHA(c) || c == '_';
}

/*
 * Return TRUE if character "c" can be used as the first character of a
 * dictionary key.
 */
    int
eval_isdictc(int c)
{
    return ASCII_ISALNUM(c) || c == '_';
}

/*
 * Handle:
 * - expr[expr], expr[expr:expr] subscript
 * - ".name" lookup
 * - function call with Funcref variable: func(expr)
 * - method call: var->method()
 *
 * Can all be combined in any order: dict.func(expr)[idx]['func'](expr)->len()
 * "name_start" points to a variable before the subscript or is NULL.
 */
    int
handle_subscript(
    char_u	**arg,
    char_u	*name_start,
    typval_T	*rettv,
    evalarg_T	*evalarg,
    int		verbose)	// give error messages
{
    int		evaluate = evalarg != NULL
				      && (evalarg->eval_flags & EVAL_EVALUATE);
    int		ret = OK;
    dict_T	*selfdict = NULL;
    int		check_white = TRUE;
    int		getnext;
    char_u	*p;

    while (ret == OK)
    {
	// When at the end of the line and ".name" or "->{" or "->X" follows in
	// the next line then consume the line break.
	p = eval_next_non_blank(*arg, evalarg, &getnext);
	if (getnext
	    && ((*p == '.'
		    && ((rettv->v_type == VAR_DICT && eval_isdictc(p[1]))
			|| rettv->v_type == VAR_CLASS
			|| rettv->v_type == VAR_OBJECT))
		|| (p[0] == '-' && p[1] == '>' && (p[2] == '{'
			|| ASCII_ISALPHA(in_vim9script() ? *skipwhite(p + 2)
								    : p[2])))))
	{
	    *arg = eval_next_line(*arg, evalarg);
	    p = *arg;
	    check_white = FALSE;
	}

	if (rettv->v_type == VAR_ANY)
	{
	    char_u	*exp_name;
	    int		cc;
	    int		idx;
	    ufunc_T	*ufunc;
	    type_T	*type;

	    // Found script from "import {name} as name", script item name must
	    // follow.  "rettv->vval.v_number" has the script ID.
	    if (**arg != '.')
	    {
		if (verbose)
		    semsg(_(e_expected_dot_after_name_str),
					name_start != NULL ? name_start: *arg);
		ret = FAIL;
		break;
	    }
	    ++*arg;
	    if (IS_WHITE_OR_NUL(**arg))
	    {
		if (verbose)
		    emsg(_(e_no_white_space_allowed_after_dot));
		ret = FAIL;
		break;
	    }

	    // isolate the name
	    exp_name = *arg;
	    while (eval_isnamec(**arg))
		++*arg;
	    cc = **arg;
	    **arg = NUL;

	    idx = find_exported(rettv->vval.v_number, exp_name, &ufunc, &type,
		       evalarg == NULL ? NULL : evalarg->eval_cctx,
		       evalarg == NULL ? NULL : evalarg->eval_cstack, verbose);
	    **arg = cc;

	    if (idx < 0 && ufunc == NULL)
	    {
		ret = FAIL;
		break;
	    }
	    if (idx >= 0)
	    {
		scriptitem_T    *si = SCRIPT_ITEM(rettv->vval.v_number);
		svar_T		*sv = ((svar_T *)si->sn_var_vals.ga_data) + idx;

		copy_tv(sv->sv_tv, rettv);
	    }
	    else
	    {
		rettv->v_type = VAR_FUNC;
		rettv->vval.v_string = vim_strsave(ufunc->uf_name);
	    }
	    continue;
	}

	if ((**arg == '(' && (!evaluate || rettv->v_type == VAR_FUNC
			    || rettv->v_type == VAR_PARTIAL))
		    && (!check_white || !VIM_ISWHITE(*(*arg - 1))))
	{
	    ret = call_func_rettv(arg, evalarg, rettv, evaluate,
							       selfdict, NULL);

	    // Stop the expression evaluation when immediately aborting on
	    // error, or when an interrupt occurred or an exception was thrown
	    // but not caught.
	    if (aborting())
	    {
		if (ret == OK)
		    clear_tv(rettv);
		ret = FAIL;
	    }
	    dict_unref(selfdict);
	    selfdict = NULL;
	}
	else if (p[0] == '-' && p[1] == '>')
	{
	    if (in_vim9script())
		*arg = skipwhite(p + 2);
	    else
		*arg = p + 2;
	    if (VIM_ISWHITE(**arg))
	    {
		emsg(_(e_no_white_space_allowed_before_parenthesis));
		ret = FAIL;
	    }
	    else if ((**arg == '{' && !in_vim9script()) || **arg == '(')
		// expr->{lambda}() or expr->(lambda)()
		ret = eval_lambda(arg, rettv, evalarg, verbose);
	    else
		// expr->name()
		ret = eval_method(arg, rettv, evalarg, verbose);
	}
	// "." is ".name" lookup when we found a dict or when evaluating and
	// scriptversion is at least 2, where string concatenation is "..".
	else if (**arg == '['
		|| (**arg == '.' && (rettv->v_type == VAR_DICT
			|| (!evaluate
			    && (*arg)[1] != '.'
			    && !in_old_script(2)))))
	{
	    dict_unref(selfdict);
	    if (rettv->v_type == VAR_DICT)
	    {
		selfdict = rettv->vval.v_dict;
		if (selfdict != NULL)
		    ++selfdict->dv_refcount;
	    }
	    else
		selfdict = NULL;
	    if (eval_index(arg, rettv, evalarg, verbose) == FAIL)
	    {
		clear_tv(rettv);
		ret = FAIL;
	    }
	}
	else if (**arg == '.' && (rettv->v_type == VAR_CLASS
					       || rettv->v_type == VAR_OBJECT))
	{
	    // class member: SomeClass.varname
	    // class method: SomeClass.SomeMethod()
	    // class constructor: SomeClass.new()
	    // object member: someObject.varname
	    // object method: someObject.SomeMethod()
	    if (class_object_index(arg, rettv, evalarg, verbose) == FAIL)
	    {
		clear_tv(rettv);
		ret = FAIL;
	    }
	}
	else
	    break;
    }

    // Turn "dict.Func" into a partial for "Func" bound to "dict".
    // Don't do this when "Func" is already a partial that was bound
    // explicitly (pt_auto is FALSE).
    if (selfdict != NULL
	    && (rettv->v_type == VAR_FUNC
		|| (rettv->v_type == VAR_PARTIAL
		    && (rettv->vval.v_partial->pt_auto
			|| rettv->vval.v_partial->pt_dict == NULL))))
	selfdict = make_partial(selfdict, rettv);

    dict_unref(selfdict);
    return ret;
}

/*
 * Make a copy of an item.
 * Lists and Dictionaries are also copied.  A deep copy if "deep" is set.
 * "top" is TRUE for the toplevel of copy().
 * For deepcopy() "copyID" is zero for a full copy or the ID for when a
 * reference to an already copied list/dict can be used.
 * Returns FAIL or OK.
 */
    int
item_copy(
    typval_T	*from,
    typval_T	*to,
    int		deep,
    int		top,
    int		copyID)
{
    static int	recurse = 0;
    int		ret = OK;

    if (recurse >= DICT_MAXNEST)
    {
	emsg(_(e_variable_nested_too_deep_for_making_copy));
	return FAIL;
    }
    ++recurse;

    switch (from->v_type)
    {
	case VAR_NUMBER:
	case VAR_FLOAT:
	case VAR_STRING:
	case VAR_FUNC:
	case VAR_PARTIAL:
	case VAR_BOOL:
	case VAR_SPECIAL:
	case VAR_JOB:
	case VAR_CHANNEL:
	case VAR_INSTR:
	case VAR_CLASS:
	case VAR_OBJECT:
	case VAR_TYPEALIAS:
	    copy_tv(from, to);
	    break;
	case VAR_LIST:
	    to->v_type = VAR_LIST;
	    to->v_lock = 0;
	    if (from->vval.v_list == NULL)
		to->vval.v_list = NULL;
	    else if (copyID != 0 && from->vval.v_list->lv_copyID == copyID)
	    {
		// use the copy made earlier
		to->vval.v_list = from->vval.v_list->lv_copylist;
		++to->vval.v_list->lv_refcount;
	    }
	    else
		to->vval.v_list = list_copy(from->vval.v_list,
							    deep, top, copyID);
	    if (to->vval.v_list == NULL)
		ret = FAIL;
	    break;
	case VAR_BLOB:
	    ret = blob_copy(from->vval.v_blob, to);
	    break;
	case VAR_DICT:
	    to->v_type = VAR_DICT;
	    to->v_lock = 0;
	    if (from->vval.v_dict == NULL)
		to->vval.v_dict = NULL;
	    else if (copyID != 0 && from->vval.v_dict->dv_copyID == copyID)
	    {
		// use the copy made earlier
		to->vval.v_dict = from->vval.v_dict->dv_copydict;
		++to->vval.v_dict->dv_refcount;
	    }
	    else
		to->vval.v_dict = dict_copy(from->vval.v_dict,
							    deep, top, copyID);
	    if (to->vval.v_dict == NULL)
		ret = FAIL;
	    break;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	    internal_error_no_abort("item_copy(UNKNOWN)");
	    ret = FAIL;
    }
    --recurse;
    return ret;
}

    void
echo_one(typval_T *rettv, int with_space, int *atstart, int *needclr)
{
    char_u	*tofree;
    char_u	numbuf[NUMBUFLEN];
    char_u	*p = echo_string(rettv, &tofree, numbuf, get_copyID());

    if (*atstart)
    {
	*atstart = FALSE;
	// Call msg_start() after eval1(), evaluating the expression
	// may cause a message to appear.
	if (with_space)
	{
	    // Mark the saved text as finishing the line, so that what
	    // follows is displayed on a new line when scrolling back
	    // at the more prompt.
	    msg_sb_eol();
	    msg_start();
	}
    }
    else if (with_space)
	msg_puts_attr(" ", echo_attr);

    if (p != NULL)
	for ( ; *p != NUL && !got_int; ++p)
	{
	    if (*p == '\n' || *p == '\r' || *p == TAB)
	    {
		if (*p != TAB && *needclr)
		{
		    // remove any text still there from the command
		    msg_clr_eos();
		    *needclr = FALSE;
		}
		msg_putchar_attr(*p, echo_attr);
	    }
	    else
	    {
		if (has_mbyte)
		{
		    int i = (*mb_ptr2len)(p);

		    (void)msg_outtrans_len_attr(p, i, echo_attr);
		    p += i - 1;
		}
		else
		    (void)msg_outtrans_len_attr(p, 1, echo_attr);
	    }
	}
    vim_free(tofree);
}

/*
 * ":echo expr1 ..."	print each argument separated with a space, add a
 *			newline at the end.
 * ":echon expr1 ..."	print each argument plain.
 */
    void
ex_echo(exarg_T *eap)
{
    char_u	*arg = eap->arg;
    typval_T	rettv;
    char_u	*arg_start;
    int		needclr = TRUE;
    int		atstart = TRUE;
    int		did_emsg_before = did_emsg;
    int		called_emsg_before = called_emsg;
    evalarg_T	evalarg;

    fill_evalarg_from_eap(&evalarg, eap, eap->skip);

    if (eap->skip)
	++emsg_skip;
    while ((!ends_excmd2(eap->cmd, arg) || *arg == '"') && !got_int)
    {
	// If eval1() causes an error message the text from the command may
	// still need to be cleared. E.g., "echo 22,44".
	need_clr_eos = needclr;

	arg_start = arg;
	if (eval1(&arg, &rettv, &evalarg) == FAIL)
	{
	    /*
	     * Report the invalid expression unless the expression evaluation
	     * has been cancelled due to an aborting error, an interrupt, or an
	     * exception.
	     */
	    if (!aborting() && did_emsg == did_emsg_before
					  && called_emsg == called_emsg_before)
		semsg(_(e_invalid_expression_str), arg_start);
	    need_clr_eos = FALSE;
	    break;
	}
	need_clr_eos = FALSE;

	if (!eap->skip)
	{
	    if (rettv.v_type == VAR_VOID)
	    {
		semsg(_(e_expression_does_not_result_in_value_str), arg_start);
		break;
	    }
	    echo_one(&rettv, eap->cmdidx == CMD_echo, &atstart, &needclr);
	}

	clear_tv(&rettv);
	arg = skipwhite(arg);
    }
    set_nextcmd(eap, arg);
    clear_evalarg(&evalarg, eap);

    if (eap->skip)
	--emsg_skip;
    else
    {
	// remove text that may still be there from the command
	if (needclr)
	    msg_clr_eos();
	if (eap->cmdidx == CMD_echo)
	    msg_end();
    }
}

/*
 * ":echohl {name}".
 */
    void
ex_echohl(exarg_T *eap)
{
    echo_attr = syn_name2attr(eap->arg);
}

/*
 * Returns the :echo attribute
 */
    int
get_echo_attr(void)
{
    return echo_attr;
}

/*
 * ":execute expr1 ..."	execute the result of an expression.
 * ":echomsg expr1 ..."	Print a message
 * ":echowindow expr1 ..." Print a message in the messages window
 * ":echoerr expr1 ..."	Print an error
 * ":echoconsole expr1 ..." Print a message on stdout
 * Each gets spaces around each argument and a newline at the end for
 * echo commands
 */
    void
ex_execute(exarg_T *eap)
{
    char_u	*arg = eap->arg;
    typval_T	rettv;
    int		ret = OK;
    char_u	*p;
    garray_T	ga;
    int		len;
    long	start_lnum = SOURCING_LNUM;

    ga_init2(&ga, 1, 80);

    if (eap->skip)
	++emsg_skip;
    while (!ends_excmd2(eap->cmd, arg) || *arg == '"')
    {
	ret = eval1_emsg(&arg, &rettv, eap);
	if (ret == FAIL)
	    break;

	if (!eap->skip)
	{
	    char_u   buf[NUMBUFLEN];

	    if (eap->cmdidx == CMD_execute)
	    {
		if (rettv.v_type == VAR_CHANNEL || rettv.v_type == VAR_JOB)
		{
		    semsg(_(e_using_invalid_value_as_string_str),
						  vartype_name(rettv.v_type));
		    p = NULL;
		}
		else
		    p = tv_get_string_buf(&rettv, buf);
	    }
	    else
		p = tv_stringify(&rettv, buf);
	    if (p == NULL)
	    {
		clear_tv(&rettv);
		ret = FAIL;
		break;
	    }
	    len = (int)STRLEN(p);
	    if (ga_grow(&ga, len + 2) == FAIL)
	    {
		clear_tv(&rettv);
		ret = FAIL;
		break;
	    }
	    if (ga.ga_len)
		((char_u *)(ga.ga_data))[ga.ga_len++] = ' ';
	    STRCPY((char_u *)(ga.ga_data) + ga.ga_len, p);
	    ga.ga_len += len;
	}

	clear_tv(&rettv);
	arg = skipwhite(arg);
    }

    if (ret != FAIL && ga.ga_data != NULL)
    {
	// use the first line of continuation lines for messages
	SOURCING_LNUM = start_lnum;

	if (eap->cmdidx == CMD_echomsg
		|| eap->cmdidx == CMD_echowindow
		|| eap->cmdidx == CMD_echoerr)
	{
	    // Mark the already saved text as finishing the line, so that what
	    // follows is displayed on a new line when scrolling back at the
	    // more prompt.
	    msg_sb_eol();
	}

	if (eap->cmdidx == CMD_echomsg)
	{
	    msg_attr(ga.ga_data, echo_attr);
	    out_flush();
	}
	else if (eap->cmdidx == CMD_echowindow)
	{
#ifdef HAS_MESSAGE_WINDOW
	    start_echowindow(eap->addr_count > 0 ? eap->line2 : 0);
#endif
	    msg_attr(ga.ga_data, echo_attr);
#ifdef HAS_MESSAGE_WINDOW
	    end_echowindow();
#endif
	}
	else if (eap->cmdidx == CMD_echoconsole)
	{
	    ui_write(ga.ga_data, (int)STRLEN(ga.ga_data), TRUE);
	    ui_write((char_u *)"\r\n", 2, TRUE);
	}
	else if (eap->cmdidx == CMD_echoerr)
	{
	    int		save_did_emsg = did_emsg;

	    // We don't want to abort following commands, restore did_emsg.
	    emsg(ga.ga_data);
	    if (!force_abort)
		did_emsg = save_did_emsg;
	}
	else if (eap->cmdidx == CMD_execute)
	{
	    int save_sticky_cmdmod_flags = sticky_cmdmod_flags;

	    // "legacy exe cmd" and "vim9cmd exe cmd" applies to "cmd".
	    sticky_cmdmod_flags = cmdmod.cmod_flags
						& (CMOD_LEGACY | CMOD_VIM9CMD);
	    do_cmdline((char_u *)ga.ga_data,
		       eap->ea_getline, eap->cookie, DOCMD_NOWAIT|DOCMD_VERBOSE);
	    sticky_cmdmod_flags = save_sticky_cmdmod_flags;
	}
    }

    ga_clear(&ga);

    if (eap->skip)
	--emsg_skip;
    set_nextcmd(eap, arg);
}

/*
 * Skip over the name of an option: "&option", "&g:option" or "&l:option".
 * "arg" points to the "&" or '+' when called, to "option" when returning.
 * Returns NULL when no option name found.  Otherwise pointer to the char
 * after the option name.
 */
    char_u *
find_option_end(char_u **arg, int *scope)
{
    char_u	*p = *arg;

    ++p;
    if (*p == 'g' && p[1] == ':')
    {
	*scope = OPT_GLOBAL;
	p += 2;
    }
    else if (*p == 'l' && p[1] == ':')
    {
	*scope = OPT_LOCAL;
	p += 2;
    }
    else
	*scope = 0;

    if (!ASCII_ISALPHA(*p))
	return NULL;
    *arg = p;

    if (p[0] == 't' && p[1] == '_' && p[2] != NUL && p[3] != NUL)
	p += 4;	    // termcap option
    else
	while (ASCII_ISALPHA(*p))
	    ++p;
    return p;
}

/*
 * Display script name where an item was last set.
 * Should only be invoked when 'verbose' is non-zero.
 */
    void
last_set_msg(sctx_T script_ctx)
{
    char_u *p;

    if (script_ctx.sc_sid == 0)
	return;

    p = home_replace_save(NULL, get_scriptname(script_ctx.sc_sid));
    if (p == NULL)
	return;

    verbose_enter();
    msg_puts(_("\n\tLast set from "));
    msg_puts((char *)p);
    if (script_ctx.sc_lnum > 0)
    {
	msg_puts(_(line_msg));
	msg_outnum((long)script_ctx.sc_lnum);
    }
    verbose_leave();
    vim_free(p);
}

#endif // FEAT_EVAL

/*
 * Perform a substitution on "str" with pattern "pat" and substitute "sub".
 * When "sub" is NULL "expr" is used, must be a VAR_FUNC or VAR_PARTIAL.
 * "flags" can be "g" to do a global substitute.
 * Returns an allocated string, NULL for error.
 */
    char_u *
do_string_sub(
    char_u	*str,
    char_u	*pat,
    char_u	*sub,
    typval_T	*expr,
    char_u	*flags)
{
    int		sublen;
    regmatch_T	regmatch;
    int		i;
    int		do_all;
    char_u	*tail;
    char_u	*end;
    garray_T	ga;
    char_u	*ret;
    char_u	*save_cpo;
    char_u	*zero_width = NULL;

    // Make 'cpoptions' empty, so that the 'l' flag doesn't work here
    save_cpo = p_cpo;
    p_cpo = empty_option;

    ga_init2(&ga, 1, 200);

    do_all = (flags[0] == 'g');

    regmatch.rm_ic = p_ic;
    regmatch.regprog = vim_regcomp(pat, RE_MAGIC + RE_STRING);
    if (regmatch.regprog != NULL)
    {
	tail = str;
	end = str + STRLEN(str);
	while (vim_regexec_nl(&regmatch, str, (colnr_T)(tail - str)))
	{
	    // Skip empty match except for first match.
	    if (regmatch.startp[0] == regmatch.endp[0])
	    {
		if (zero_width == regmatch.startp[0])
		{
		    // avoid getting stuck on a match with an empty string
		    i = mb_ptr2len(tail);
		    mch_memmove((char_u *)ga.ga_data + ga.ga_len, tail,
								   (size_t)i);
		    ga.ga_len += i;
		    tail += i;
		    continue;
		}
		zero_width = regmatch.startp[0];
	    }

	    /*
	     * Get some space for a temporary buffer to do the substitution
	     * into.  It will contain:
	     * - The text up to where the match is.
	     * - The substituted text.
	     * - The text after the match.
	     */
	    sublen = vim_regsub(&regmatch, sub, expr, tail, 0, REGSUB_MAGIC);
	    if (sublen <= 0)
	    {
		ga_clear(&ga);
		break;
	    }
	    if (ga_grow(&ga, (int)((end - tail) + sublen -
			    (regmatch.endp[0] - regmatch.startp[0]))) == FAIL)
	    {
		ga_clear(&ga);
		break;
	    }

	    // copy the text up to where the match is
	    i = (int)(regmatch.startp[0] - tail);
	    mch_memmove((char_u *)ga.ga_data + ga.ga_len, tail, (size_t)i);
	    // add the substituted text
	    (void)vim_regsub(&regmatch, sub, expr,
				  (char_u *)ga.ga_data + ga.ga_len + i, sublen,
				  REGSUB_COPY | REGSUB_MAGIC);
	    ga.ga_len += i + sublen - 1;
	    tail = regmatch.endp[0];
	    if (*tail == NUL)
		break;
	    if (!do_all)
		break;
	}

	if (ga.ga_data != NULL)
	    STRCPY((char *)ga.ga_data + ga.ga_len, tail);

	vim_regfree(regmatch.regprog);
    }

    ret = vim_strsave(ga.ga_data == NULL ? str : (char_u *)ga.ga_data);
    ga_clear(&ga);
    if (p_cpo == empty_option)
	p_cpo = save_cpo;
    else
    {
	// Darn, evaluating {sub} expression or {expr} changed the value.
	// If it's still empty it was changed and restored, need to restore in
	// the complicated way.
	if (*p_cpo == NUL)
	    set_option_value_give_err((char_u *)"cpo", 0L, save_cpo, 0);
	free_string_option(save_cpo);
    }

    return ret;
}
