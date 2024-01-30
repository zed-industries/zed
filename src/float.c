/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * float.c: Floating point functions
 */
#define USING_FLOAT_STUFF

#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

#ifdef VMS
# include <float.h>
#endif

/*
 * Convert the string "text" to a floating point number.
 * This uses strtod().  setlocale(LC_NUMERIC, "C") has been used to make sure
 * this always uses a decimal point.
 * Returns the length of the text that was consumed.
 */
    int
string2float(
    char_u	*text,
    float_T	*value,	    // result stored here
    int		skip_quotes)
{
    char	*s = (char *)text;
    float_T	f;

    // MS-Windows does not deal with "inf" and "nan" properly.
    if (STRNICMP(text, "inf", 3) == 0)
    {
	*value = INFINITY;
	return 3;
    }
    if (STRNICMP(text, "-inf", 3) == 0)
    {
	*value = -INFINITY;
	return 4;
    }
    if (STRNICMP(text, "nan", 3) == 0)
    {
	*value = NAN;
	return 3;
    }
    if (skip_quotes && vim_strchr((char_u *)s, '\'') != NULL)
    {
	char_u	    buf[100];
	char_u	    *p;
	int	    quotes = 0;

	vim_strncpy(buf, (char_u *)s, 99);
	for (p = buf; ; p = skipdigits(p))
	{
	    // remove single quotes between digits, not in the exponent
	    if (*p == '\'')
	    {
		++quotes;
		mch_memmove(p, p + 1, STRLEN(p));
	    }
	    if (!vim_isdigit(*p))
		break;
	}
	s = (char *)buf;
	f = strtod(s, &s);
	*value = f;
	return (int)((char_u *)s - buf) + quotes;
    }

    f = strtod(s, &s);
    *value = f;
    return (int)((char_u *)s - text);
}

/*
 * Get the float value of "argvars[0]" into "f".
 * Returns FAIL when the argument is not a Number or Float.
 */
    static int
get_float_arg(typval_T *argvars, float_T *f)
{
    if (argvars[0].v_type == VAR_FLOAT)
    {
	*f = argvars[0].vval.v_float;
	return OK;
    }
    if (argvars[0].v_type == VAR_NUMBER)
    {
	*f = (float_T)argvars[0].vval.v_number;
	return OK;
    }
    emsg(_(e_number_or_float_required));
    return FAIL;
}

/*
 * "abs(expr)" function
 */
    void
f_abs(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_FLOAT)
    {
	rettv->v_type = VAR_FLOAT;
	rettv->vval.v_float = fabs(argvars[0].vval.v_float);
    }
    else
    {
	varnumber_T	n;
	int		error = FALSE;

	n = tv_get_number_chk(&argvars[0], &error);
	if (error)
	    rettv->vval.v_number = -1;
	else if (n > 0)
	    rettv->vval.v_number = n;
	else
	    rettv->vval.v_number = -n;
    }
}

/*
 * "acos()" function
 */
    void
f_acos(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = acos(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "asin()" function
 */
    void
f_asin(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = asin(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "atan()" function
 */
    void
f_atan(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = atan(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "atan2()" function
 */
    void
f_atan2(typval_T *argvars, typval_T *rettv)
{
    float_T	fx = 0.0, fy = 0.0;

    if (in_vim9script()
	    && (check_for_float_or_nr_arg(argvars, 0) == FAIL
		|| check_for_float_or_nr_arg(argvars, 1) == FAIL))
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &fx) == OK
				     && get_float_arg(&argvars[1], &fy) == OK)
	rettv->vval.v_float = atan2(fx, fy);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "ceil({float})" function
 */
    void
f_ceil(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = ceil(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "cos()" function
 */
    void
f_cos(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = cos(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "cosh()" function
 */
    void
f_cosh(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = cosh(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "exp()" function
 */
    void
f_exp(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = exp(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "float2nr({float})" function
 */
    void
f_float2nr(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    if (get_float_arg(argvars, &f) != OK)
	return;

    if (f <= (float_T)-VARNUM_MAX + DBL_EPSILON)
	rettv->vval.v_number = -VARNUM_MAX;
    else if (f >= (float_T)VARNUM_MAX - DBL_EPSILON)
	rettv->vval.v_number = VARNUM_MAX;
    else
	rettv->vval.v_number = (varnumber_T)f;
}

/*
 * "floor({float})" function
 */
    void
f_floor(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = floor(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "fmod()" function
 */
    void
f_fmod(typval_T *argvars, typval_T *rettv)
{
    float_T	fx = 0.0, fy = 0.0;

    if (in_vim9script()
	    && (check_for_float_or_nr_arg(argvars, 0) == FAIL
		|| check_for_float_or_nr_arg(argvars, 1) == FAIL))
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &fx) == OK
				     && get_float_arg(&argvars[1], &fy) == OK)
	rettv->vval.v_float = fmod(fx, fy);
    else
	rettv->vval.v_float = 0.0;
}

# if defined(HAVE_MATH_H) || defined(PROTO)
/*
 * "isinf()" function
 */
    void
f_isinf(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_FLOAT && isinf(argvars[0].vval.v_float))
	rettv->vval.v_number = argvars[0].vval.v_float > 0.0 ? 1 : -1;
}

/*
 * "isnan()" function
 */
    void
f_isnan(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_number = argvars[0].v_type == VAR_FLOAT
					    && isnan(argvars[0].vval.v_float);
}
# endif

/*
 * "log()" function
 */
    void
f_log(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = log(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "log10()" function
 */
    void
f_log10(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = log10(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "pow()" function
 */
    void
f_pow(typval_T *argvars, typval_T *rettv)
{
    float_T	fx = 0.0, fy = 0.0;

    if (in_vim9script()
	    && (check_for_float_or_nr_arg(argvars, 0) == FAIL
		|| check_for_float_or_nr_arg(argvars, 1) == FAIL))
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &fx) == OK
				     && get_float_arg(&argvars[1], &fy) == OK)
	rettv->vval.v_float = pow(fx, fy);
    else
	rettv->vval.v_float = 0.0;
}


/*
 * round() is not in C90, use ceil() or floor() instead.
 */
    float_T
vim_round(float_T f)
{
    return f > 0 ? floor(f + 0.5) : ceil(f - 0.5);
}

/*
 * "round({float})" function
 */
    void
f_round(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = vim_round(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "sin()" function
 */
    void
f_sin(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = sin(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "sinh()" function
 */
    void
f_sinh(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = sinh(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "sqrt()" function
 */
    void
f_sqrt(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = sqrt(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "str2float()" function
 */
    void
f_str2float(typval_T *argvars, typval_T *rettv)
{
    char_u *p;
    int     isneg;
    int	    skip_quotes;

    if (in_vim9script() && check_for_string_arg(argvars, 0) == FAIL)
	return;

    skip_quotes = argvars[1].v_type != VAR_UNKNOWN && tv_get_bool(&argvars[1]);

    p = skipwhite(tv_get_string_strict(&argvars[0]));
    isneg = (*p == '-');

    if (*p == '+' || *p == '-')
	p = skipwhite(p + 1);
    (void)string2float(p, &rettv->vval.v_float, skip_quotes);
    if (isneg)
	rettv->vval.v_float *= -1;
    rettv->v_type = VAR_FLOAT;
}

/*
 * "tan()" function
 */
    void
f_tan(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = tan(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "tanh()" function
 */
    void
f_tanh(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	rettv->vval.v_float = tanh(f);
    else
	rettv->vval.v_float = 0.0;
}

/*
 * "trunc({float})" function
 */
    void
f_trunc(typval_T *argvars, typval_T *rettv)
{
    float_T	f = 0.0;

    if (in_vim9script() && check_for_float_or_nr_arg(argvars, 0) == FAIL)
	return;

    rettv->v_type = VAR_FLOAT;
    if (get_float_arg(argvars, &f) == OK)
	// trunc() is not in C90, use floor() or ceil() instead.
	rettv->vval.v_float = f > 0 ? floor(f) : ceil(f);
    else
	rettv->vval.v_float = 0.0;
}

#endif
