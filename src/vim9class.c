/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * vim9class.c: Vim9 script class support
 */

#define USING_FLOAT_STUFF
#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

// When not generating protos this is included in proto.h
#ifdef PROTO
# include "vim9.h"
#endif

static class_T *first_class = NULL;
static class_T *next_nonref_class = NULL;

/*
 * Call this function when a class has been created.  It will be added to the
 * list headed by "first_class".
 */
    static void
class_created(class_T *cl)
{
    if (first_class != NULL)
    {
	cl->class_next_used = first_class;
	first_class->class_prev_used = cl;
    }
    first_class = cl;
}

/*
 * Call this function when a class has been cleared and is about to be freed.
 * It is removed from the list headed by "first_class".
 */
    static void
class_cleared(class_T *cl)
{
    if (cl->class_next_used != NULL)
	cl->class_next_used->class_prev_used = cl->class_prev_used;
    if (cl->class_prev_used != NULL)
	cl->class_prev_used->class_next_used = cl->class_next_used;
    else if (first_class == cl)
	first_class = cl->class_next_used;

    // update the next class to check if needed
    if (cl == next_nonref_class)
	next_nonref_class = cl->class_next_used;
}

/*
 * Parse a member declaration, both object and class member.
 * Returns OK or FAIL.  When OK then "varname_end" is set to just after the
 * variable name and "type_ret" is set to the declared or detected type.
 * "init_expr" is set to the initialisation expression (allocated), if there is
 * one.  For an interface "init_expr" is NULL.
 */
    static int
parse_member(
    exarg_T	*eap,
    char_u	*line,
    char_u	*varname,
    int		has_public,	    // TRUE if "public" seen before "varname"
    char_u	**varname_end,
    int		*has_type,
    garray_T	*type_list,
    type_T	**type_ret,
    char_u	**init_expr)
{
    *varname_end = to_name_end(varname, FALSE);
    if (*varname == '_' && has_public)
    {
	semsg(_(e_public_variable_name_cannot_start_with_underscore_str), line);
	return FAIL;
    }

    char_u *colon = skipwhite(*varname_end);
    char_u *type_arg = colon;
    type_T *type = NULL;
    *has_type = FALSE;
    if (*colon == ':')
    {
	if (VIM_ISWHITE(**varname_end))
	{
	    semsg(_(e_no_white_space_allowed_before_colon_str), varname);
	    return FAIL;
	}
	if (!VIM_ISWHITE(colon[1]))
	{
	    semsg(_(e_white_space_required_after_str_str), ":", varname);
	    return FAIL;
	}
	type_arg = skipwhite(colon + 1);
	type = parse_type(&type_arg, type_list, TRUE);
	if (type == NULL)
	    return FAIL;
	*has_type = TRUE;
    }

    char_u *init_arg = skipwhite(type_arg);
    if (type == NULL && *init_arg != '=')
    {
	emsg(_(e_type_or_initialization_required));
	return FAIL;
    }

    if (init_expr == NULL && *init_arg == '=')
    {
	emsg(_(e_cannot_initialize_variable_in_interface));
	return FAIL;
    }

    if (*init_arg == '=')
    {
	evalarg_T evalarg;
	char_u *expr_start, *expr_end;

	if (!VIM_ISWHITE(init_arg[-1]) || !VIM_ISWHITE(init_arg[1]))
	{
	    semsg(_(e_white_space_required_before_and_after_str_at_str),
							"=", type_arg);
	    return FAIL;
	}
	init_arg = skipwhite(init_arg + 1);

	fill_evalarg_from_eap(&evalarg, eap, FALSE);
	(void)skip_expr_concatenate(&init_arg, &expr_start, &expr_end, &evalarg);

	// No type specified for the member.  Set it to "any" and the correct
	// type will be set when the object is instantiated.
	if (type == NULL)
	    type = &t_any;

	*init_expr = vim_strnsave(expr_start, expr_end - expr_start);
	// Free the memory pointed by expr_start.
	clear_evalarg(&evalarg, NULL);
    }
    else if (!valid_declaration_type(type))
	return FAIL;

    *type_ret = type;
    return OK;
}

typedef struct oc_newmember_S oc_newmember_T;
struct oc_newmember_S
{
    garray_T	*gap;
    char_u	*varname;
    char_u	*varname_end;
    int		has_public;
    int		has_final;
    int		has_type;
    type_T	*type;
    char_u	*init_expr;
};

/*
 * Add a member to an object or a class.
 * Returns OK when successful, "init_expr" will be consumed then.
 * Returns FAIL otherwise, caller might need to free "init_expr".
 */
    static int
add_member(
    garray_T	*gap,
    char_u	*varname,
    char_u	*varname_end,
    int		has_public,
    int		has_final,
    int		has_const,
    int		has_type,
    type_T	*type,
    char_u	*init_expr)
{
    if (ga_grow(gap, 1) == FAIL)
	return FAIL;
    ocmember_T *m = ((ocmember_T *)gap->ga_data) + gap->ga_len;
    m->ocm_name = vim_strnsave(varname, varname_end - varname);
    m->ocm_access = has_public ? VIM_ACCESS_ALL
		      : *varname == '_' ? VIM_ACCESS_PRIVATE : VIM_ACCESS_READ;
    if (has_final)
	m->ocm_flags |= OCMFLAG_FINAL;
    if (has_const)
	m->ocm_flags |= OCMFLAG_CONST;
    if (has_type)
	m->ocm_flags |= OCMFLAG_HAS_TYPE;
    m->ocm_type = type;
    if (init_expr != NULL)
	m->ocm_init = init_expr;
    ++gap->ga_len;
    return OK;
}

/*
 * Move the class or object members found while parsing a class into the class.
 * "gap" contains the found members.
 * "parent_members" points to the members in the parent class (if any)
 * "parent_count" is the number of members in the parent class
 * "members" will be set to the newly allocated array of members and
 * "member_count" set to the number of members.
 * Returns OK or FAIL.
 */
    static int
add_members_to_class(
    garray_T	*gap,
    ocmember_T	*parent_members,
    int		parent_count,
    ocmember_T	**members,
    int		*member_count)
{
    *member_count = parent_count + gap->ga_len;
    *members = *member_count == 0 ? NULL
				       : ALLOC_MULT(ocmember_T, *member_count);
    if (*member_count > 0 && *members == NULL)
	return FAIL;
    for (int i = 0; i < parent_count; ++i)
    {
	// parent members need to be copied
	ocmember_T	*m = *members + i;
	*m = parent_members[i];
	m->ocm_name = vim_strsave(m->ocm_name);
	if (m->ocm_init != NULL)
	    m->ocm_init = vim_strsave(m->ocm_init);
    }
    if (gap->ga_len > 0)
	// new members are moved
	mch_memmove(*members + parent_count,
			       gap->ga_data, sizeof(ocmember_T) * gap->ga_len);
    VIM_CLEAR(gap->ga_data);
    return OK;
}

/*
 * Convert a member index "idx" of interface "itf" to the member index of class
 * "cl" implementing that interface.
 */
    int
object_index_from_itf_index(class_T *itf, int is_method, int idx, class_T *cl)
{
    if (idx >= (is_method ? itf->class_obj_method_count
				   : itf->class_obj_member_count))
    {
	siemsg("index %d out of range for interface %s", idx, itf->class_name);
	return 0;
    }

    // If "cl" is the interface or the class that is extended, then the method
    // index can be used directly and there is no need to search for the method
    // index in one of the child classes.
    if (cl == itf)
	return idx;

    itf2class_T		*i2c = NULL;
    int			searching = TRUE;
    int			method_offset = 0;

    for (class_T *super = cl; super != NULL && searching;
						super = super->class_extends)
    {
	for (i2c = itf->class_itf2class; i2c != NULL; i2c = i2c->i2c_next)
	{
	    if (i2c->i2c_class == super && i2c->i2c_is_method == is_method)
	    {
		searching = FALSE;
		break;
	    }
	}
	if (searching && is_method)
	    // The parent class methods are stored after the current class
	    // methods.
	    method_offset += super->class_obj_method_count_child;
    }
    if (i2c == NULL)
    {
	siemsg("class %s not found on interface %s",
					      cl->class_name, itf->class_name);
	return 0;
    }

    // A table follows the i2c for the class
    int *table = (int *)(i2c + 1);
    // "method_offset" is 0, if method is in the current class.  If method
    // is in a parent class, then it is non-zero.
    return table[idx] + method_offset;
}

/*
 * Check whether a class named "extends_name" is present.  If the class is
 * valid, then "extends_clp" is set with the class pointer.
 * Returns TRUE if the class name "extends_names" is a valid class.
 */
    static int
validate_extends_class(
    char_u  *extends_name,
    class_T **extends_clp,
    int	    is_class)
{
    typval_T	tv;
    int		success = FALSE;

    tv.v_type = VAR_UNKNOWN;
    if (eval_variable_import(extends_name, &tv) == FAIL)
    {
	semsg(_(e_class_name_not_found_str), extends_name);
	return success;
    }

    if (tv.v_type != VAR_CLASS || tv.vval.v_class == NULL
	    || (is_class
		&& (tv.vval.v_class->class_flags & CLASS_INTERFACE) != 0)
	    || (!is_class
		&& (tv.vval.v_class->class_flags & CLASS_INTERFACE) == 0))
	// a interface cannot extend a class and a class cannot extend an
	// interface.
	semsg(_(e_cannot_extend_str), extends_name);
    else
    {
	class_T *extends_cl = tv.vval.v_class;
	++extends_cl->class_refcount;
	*extends_clp = extends_cl;
	success = TRUE;
    }
    clear_tv(&tv);

    return success;
}

/*
 * Check method names in the parent class lineage to make sure the access is
 * the same for overridden methods.
 */
    static int
validate_extends_methods(
    garray_T	*objmethods_gap,
    class_T	*extends_cl)
{
    class_T	*super = extends_cl;
    int		method_count = objmethods_gap->ga_len;
    ufunc_T	**cl_fp = (ufunc_T **)(objmethods_gap->ga_data);

    while (super != NULL)
    {
	int extends_method_count = super->class_obj_method_count_child;
	if (extends_method_count == 0)
	{
	    super = super->class_extends;
	    continue;
	}

	ufunc_T **extends_methods = super->class_obj_methods;

	for (int i = 0; i < extends_method_count; i++)
	{
	    char_u  *pstr = extends_methods[i]->uf_name;
	    int	    extends_private = (*pstr == '_');
	    if (extends_private)
		pstr++;

	    // When comparing the method names, ignore the access type (public
	    // and private methods are considered the same).
	    for (int j = 0; j < method_count; j++)
	    {
		char_u  *qstr = cl_fp[j]->uf_name;
		int	priv_method = (*qstr == '_');
		if (priv_method)
		    qstr++;
		if (STRCMP(pstr, qstr) == 0 && priv_method != extends_private)
		{
		    // Method access is different between the super class and
		    // the subclass
		    semsg(_(e_method_str_of_class_str_has_different_access),
			    cl_fp[j]->uf_name, super->class_name);
		    return FALSE;
		}
	    }
	}
	super = super->class_extends;
    }

    return TRUE;
}

/*
 * Check whether a object member variable in "objmembers_gap" is a duplicate of
 * a member in any of the extended parent class lineage.  Returns TRUE if there
 * are no duplicates.
 */
    static int
extends_check_dup_members(
    garray_T	*objmembers_gap,
    class_T	*extends_cl)
{
    int member_count = objmembers_gap->ga_len;
    if (member_count == 0)
	return TRUE;

    ocmember_T *members = (ocmember_T *)(objmembers_gap->ga_data);

    // Validate each member variable
    for (int c_i = 0; c_i < member_count; c_i++)
    {
	class_T	    *p_cl = extends_cl;
	ocmember_T  *c_m = members + c_i;
	char_u	    *pstr = (*c_m->ocm_name == '_')
					? c_m->ocm_name + 1 : c_m->ocm_name;

	// Check in all the parent classes in the lineage
	while (p_cl != NULL)
	{
	    int p_member_count = p_cl->class_obj_member_count;
	    if (p_member_count == 0)
	    {
		p_cl = p_cl->class_extends;
		continue;
	    }
	    ocmember_T *p_members = p_cl->class_obj_members;

	    // Compare against all the members in the parent class
	    for (int p_i = 0; p_i < p_member_count; p_i++)
	    {
		ocmember_T	*p_m = p_members + p_i;
		char_u	*qstr = (*p_m->ocm_name == '_')
		    ? p_m->ocm_name + 1 : p_m->ocm_name;
		if (STRCMP(pstr, qstr) == 0)
		{
		    semsg(_(e_duplicate_variable_str), c_m->ocm_name);
		    return FALSE;
		}
	    }

	    p_cl = p_cl->class_extends;
	}
    }

    return TRUE;
}

/*
 * Compare the variable type of interface variables in "objmembers_gap" against
 * the variable in any of the extended super interface lineage.  Used to
 * compare the variable types when extending interfaces.  Returns TRUE if the
 * variable types are the same.
 */
    static int
extends_check_intf_var_type(
    garray_T	*objmembers_gap,
    class_T	*extends_cl)
{
    int member_count = objmembers_gap->ga_len;
    if (member_count == 0)
	return TRUE;

    ocmember_T *members = (ocmember_T *)(objmembers_gap->ga_data);

    // Validate each member variable
    for (int c_i = 0; c_i < member_count; c_i++)
    {
	class_T	    *p_cl = extends_cl;
	ocmember_T  *c_m = members + c_i;
	int	    var_found = FALSE;

	// Check in all the parent classes in the lineage
	while (p_cl != NULL && !var_found)
	{
	    int p_member_count = p_cl->class_obj_member_count;
	    if (p_member_count == 0)
	    {
		p_cl = p_cl->class_extends;
		continue;
	    }
	    ocmember_T *p_members = p_cl->class_obj_members;

	    // Compare against all the members in the parent class
	    for (int p_i = 0; p_i < p_member_count; p_i++)
	    {
		where_T		where = WHERE_INIT;
		ocmember_T	*p_m = p_members + p_i;

		if (STRCMP(p_m->ocm_name, c_m->ocm_name) != 0)
		    continue;

		// Ensure the type is matching.
		where.wt_func_name = (char *)c_m->ocm_name;
		where.wt_kind = WT_MEMBER;

		if (check_type(p_m->ocm_type, c_m->ocm_type, TRUE,
								where) == FAIL)
		    return FALSE;

		var_found = TRUE;
	    }

	    p_cl = p_cl->class_extends;
	}
    }

    return TRUE;
}

/*
 * When extending an abstract class, check whether all the abstract methods in
 * the parent class are implemented.  Returns TRUE if all the methods are
 * implemented.
 */
    static int
validate_abstract_class_methods(
    garray_T	*classmethods_gap,
    garray_T	*objmethods_gap,
    class_T	*extends_cl)
{
    for (int loop = 1; loop <= 2; ++loop)
    {
	// loop == 1: check class methods
	// loop == 2: check object methods
	int extends_method_count = loop == 1
				? extends_cl->class_class_function_count
				: extends_cl->class_obj_method_count;
	if (extends_method_count == 0)
	    continue;

	ufunc_T **extends_methods = loop == 1
				? extends_cl->class_class_functions
				: extends_cl->class_obj_methods;

	int method_count = loop == 1 ? classmethods_gap->ga_len
						: objmethods_gap->ga_len;
	ufunc_T **cl_fp = (ufunc_T **)(loop == 1
						? classmethods_gap->ga_data
						: objmethods_gap->ga_data);

	for (int i = 0; i < extends_method_count; i++)
	{
	    ufunc_T *uf = extends_methods[i];
	    if (!IS_ABSTRACT_METHOD(uf))
		continue;

	    int method_found = FALSE;

	    for (int j = 0; j < method_count; j++)
	    {
		if (STRCMP(uf->uf_name, cl_fp[j]->uf_name) == 0)
		{
		    method_found = TRUE;
		    break;
		}
	    }

	    if (!method_found)
	    {
		semsg(_(e_abstract_method_str_not_found), uf->uf_name);
		return FALSE;
	    }
	}
    }

    return TRUE;
}

/*
 * Returns TRUE if the interface variable "if_var" is present in the list of
 * variables in "cl_mt" or in the parent lineage of one of the extended classes
 * in "extends_cl".  For a class variable, 'is_class_var' is TRUE.
 */
    static int
intf_variable_present(
    char_u	*intf_class_name,
    ocmember_T *if_var,
    ocmember_T *cl_mt,
    int		cl_member_count,
    class_T	*extends_cl)
{
    int		variable_present  = FALSE;

    for (int cl_i = 0; cl_i < cl_member_count; ++cl_i)
    {
	ocmember_T	*m = &cl_mt[cl_i];
	where_T		where = WHERE_INIT;

	if (STRCMP(if_var->ocm_name, m->ocm_name) != 0)
	    continue;

	// Ensure the access type is same
	if (if_var->ocm_access != m->ocm_access)
	{
	    semsg(_(e_variable_str_of_interface_str_has_different_access),
		    if_var->ocm_name, intf_class_name);
	    return FALSE;
	}

	// Ensure the type is matching.
	if (m->ocm_type == &t_any)
	{
	    // variable type is not specified.  Use the variable type in the
	    // interface.
	    m->ocm_type = if_var->ocm_type;
	}
	else
	{
	    where.wt_func_name = (char *)m->ocm_name;
	    where.wt_kind = WT_MEMBER;
	    if (check_type(if_var->ocm_type, m->ocm_type, TRUE,
							    where) == FAIL)
		return FALSE;
	}

	variable_present = TRUE;
	break;
    }

    if (!variable_present && extends_cl != NULL)
    {
	int ext_cl_count = extends_cl->class_obj_member_count;
	ocmember_T *ext_cl_mt = extends_cl->class_obj_members;
	return intf_variable_present(intf_class_name, if_var,
					ext_cl_mt, ext_cl_count,
					extends_cl->class_extends);
    }

    return variable_present;
}

/*
 * Check the variables of the interface class "ifcl" match object variables
 * ("objmembers_gap") of a class.
 * Returns TRUE if the object variables names are valid.
 */
    static int
validate_interface_variables(
    char_u	*intf_class_name,
    class_T	*ifcl,
    garray_T	*objmembers_gap,
    class_T	*extends_cl)
{
    int if_count = ifcl->class_obj_member_count;
    if (if_count == 0)
	return TRUE;

    ocmember_T *if_ms = ifcl->class_obj_members;
    ocmember_T *cl_ms = (ocmember_T *)(objmembers_gap->ga_data);
    int cl_count = objmembers_gap->ga_len;
    for (int if_i = 0; if_i < if_count; ++if_i)
    {
	if (!intf_variable_present(intf_class_name, &if_ms[if_i], cl_ms,
							cl_count, extends_cl))
	{
	    semsg(_(e_variable_str_of_interface_str_not_implemented),
		    if_ms[if_i].ocm_name, intf_class_name);
	    return FALSE;
	}
    }

    return TRUE;
}

/*
 * Returns TRUE if the method signature of "if_method" and "cl_method" matches.
 */
    static int
intf_method_type_matches(ufunc_T *if_method, ufunc_T *cl_method)
{
    where_T where = WHERE_INIT;

    // Ensure the type is matching.
    where.wt_func_name = (char *)if_method->uf_name;
    where.wt_kind = WT_METHOD;
    if (check_type(if_method->uf_func_type, cl_method->uf_func_type, TRUE,
								where) == FAIL)
	return FALSE;

    return TRUE;
}

/*
 * Returns TRUE if the interface method "if_ufunc" is present in the list of
 * methods in "cl_fp" or in the parent lineage of one of the extended classes
 * in "extends_cl".  For a class method, 'is_class_method' is TRUE.
 */
    static int
intf_method_present(
    ufunc_T *if_ufunc,
    ufunc_T **cl_fp,
    int	    cl_count,
    class_T *extends_cl)
{
    int		method_present  = FALSE;

    for (int cl_i = 0; cl_i < cl_count; ++cl_i)
    {
	char_u *cl_name = cl_fp[cl_i]->uf_name;
	if (STRCMP(if_ufunc->uf_name, cl_name) == 0)
	{
	    // Ensure the type is matching.
	    if (!intf_method_type_matches(if_ufunc, cl_fp[cl_i]))
		return FALSE;
	    method_present = TRUE;
	    break;
	}
    }

    if (!method_present && extends_cl != NULL)
    {
	ufunc_T **ext_cl_fp = (ufunc_T **)(extends_cl->class_obj_methods);
	int	ext_cl_count = extends_cl->class_obj_method_count;
	return intf_method_present(if_ufunc, ext_cl_fp, ext_cl_count,
						extends_cl->class_extends);
    }

    return method_present;
}

/*
 * Validate that a new class implements all the class/instance methods in the
 * interface "ifcl".  The new class methods are in "classfunctions_gap" and the
 * new object methods are in "objmemthods_gap".  Also validates the method
 * types.
 * Returns TRUE if all the interface class/object methods are implemented in
 * the new class.
 */
    static int
validate_interface_methods(
    char_u	*intf_class_name,
    class_T	*ifcl,
    garray_T	*objmethods_gap,
    class_T	*extends_cl)
{
    int if_count = ifcl->class_obj_method_count;
    if (if_count == 0)
	return TRUE;

    ufunc_T **if_fp = ifcl->class_obj_methods;
    ufunc_T **cl_fp = (ufunc_T **)(objmethods_gap->ga_data);
    int cl_count = objmethods_gap->ga_len;
    for (int if_i = 0; if_i < if_count; ++if_i)
    {
	char_u	*if_name = if_fp[if_i]->uf_name;

	if (!intf_method_present(if_fp[if_i], cl_fp, cl_count, extends_cl))
	{
	    semsg(_(e_method_str_of_interface_str_not_implemented),
		    if_name, intf_class_name);
	    return FALSE;
	}
    }

    return TRUE;
}

/*
 * Validate all the "implements" classes when creating a new class.  The
 * classes are returned in "intf_classes".  The class functions, class members,
 * object methods and object members in the new class are in
 * "classfunctions_gap", "classmembers_gap", "objmethods_gap", and
 * "objmembers_gap" respectively.
 */
    static int
validate_implements_classes(
    garray_T	*impl_gap,
    class_T	**intf_classes,
    garray_T	*objmethods_gap,
    garray_T	*objmembers_gap,
    class_T	*extends_cl)
{
    int		success = TRUE;

    for (int i = 0; i < impl_gap->ga_len && success; ++i)
    {
	char_u *impl = ((char_u **)impl_gap->ga_data)[i];
	typval_T tv;
	tv.v_type = VAR_UNKNOWN;
	if (eval_variable_import(impl, &tv) == FAIL)
	{
	    semsg(_(e_interface_name_not_found_str), impl);
	    success = FALSE;
	    break;
	}

	if (tv.v_type != VAR_CLASS
		|| tv.vval.v_class == NULL
		|| (tv.vval.v_class->class_flags & CLASS_INTERFACE) == 0)
	{
	    semsg(_(e_not_valid_interface_str), impl);
	    success = FALSE;
	    clear_tv(&tv);
	    break;
	}

	class_T *ifcl = tv.vval.v_class;
	intf_classes[i] = ifcl;
	++ifcl->class_refcount;

	// check the variables of the interface match the members of the class
	success = validate_interface_variables(impl, ifcl, objmembers_gap,
								extends_cl);

	// check the functions/methods of the interface match the
	// functions/methods of the class
	if (success)
	    success = validate_interface_methods(impl, ifcl, objmethods_gap,
								extends_cl);
	clear_tv(&tv);
    }

    return success;
}

/*
 * Check no function argument name is used as a class member.
 * (Object members are always accessed with "this." prefix, so no need
 * to check them.)
 */
    static int
check_func_arg_names(
    garray_T	*classfunctions_gap,
    garray_T	*objmethods_gap,
    garray_T	*classmembers_gap)
{
    // loop 1: class functions, loop 2: object methods
    for (int loop = 1; loop <= 2; ++loop)
    {
	garray_T *gap = loop == 1 ? classfunctions_gap : objmethods_gap;

	for (int fi = 0; fi < gap->ga_len; ++fi)
	{
	    ufunc_T *uf = ((ufunc_T **)gap->ga_data)[fi];

	    for (int i = 0; i < uf->uf_args.ga_len; ++i)
	    {
		char_u *aname = ((char_u **)uf->uf_args.ga_data)[i];
		garray_T *mgap = classmembers_gap;

		// Check all the class member names
		for (int mi = 0; mi < mgap->ga_len; ++mi)
		{
		    char_u *mname =
				((ocmember_T *)mgap->ga_data + mi)->ocm_name;
		    if (STRCMP(aname, mname) == 0)
		    {
			if (uf->uf_script_ctx.sc_sid > 0)
			    SOURCING_LNUM = uf->uf_script_ctx.sc_lnum;

			semsg(_(e_argument_already_declared_in_class_str),
				aname);

			return FALSE;
		    }
		}
	    }
	}
    }

    return TRUE;
}

/*
 * Returns TRUE if 'varname' is a reserved keyword name
 */
    static int
is_reserved_varname(char_u *varname, char_u *varname_end)
{
    int reserved = FALSE;
    char_u save_varname_end = *varname_end;
    *varname_end = NUL;

    reserved = check_reserved_name(varname, FALSE) == FAIL;

    *varname_end = save_varname_end;

    return reserved;
}

/*
 * Returns TRUE if the variable "varname" is already defined either as a class
 * variable or as an object variable.
 */
    static int
is_duplicate_variable(
    garray_T	*class_members,
    garray_T	*obj_members,
    char_u	*varname,
    char_u	*varname_end)
{
    char_u	*name = vim_strnsave(varname, varname_end - varname);
    char_u	*pstr = (*name == '_') ? name + 1 : name;
    int		dup = FALSE;

    for (int loop = 1; loop <= 2; loop++)
    {
	// loop == 1: class variables, loop == 2: object variables
	garray_T    *vgap = (loop == 1) ? class_members : obj_members;
	for (int i = 0; i < vgap->ga_len; ++i)
	{
	    ocmember_T *m = ((ocmember_T *)vgap->ga_data) + i;
	    char_u	*qstr = *m->ocm_name == '_' ? m->ocm_name + 1
						    : m->ocm_name;
	    if (STRCMP(pstr, qstr) == 0)
	    {
		semsg(_(e_duplicate_variable_str), name);
		dup = TRUE;
		break;
	    }
	}
    }

    vim_free(name);
    return dup;
}

/*
 * Returns TRUE if the method "name" is already defined.
 */
    static int
is_duplicate_method(
    garray_T	*classmethods_gap,
    garray_T	*objmethods_gap,
    char_u	*name)
{
    char_u *pstr = (*name == '_') ? name + 1 : name;

    // loop 1: class methods, loop 2: object methods
    for (int loop = 1; loop <= 2; loop++)
    {
	garray_T *fgap = (loop == 1) ? classmethods_gap : objmethods_gap;
	for (int i = 0; i < fgap->ga_len; ++i)
	{
	    char_u	*n = ((ufunc_T **)fgap->ga_data)[i]->uf_name;
	    char_u	*qstr = *n == '_' ? n + 1 : n;
	    if (STRCMP(pstr, qstr) == 0)
	    {
		semsg(_(e_duplicate_function_str), name);
		return TRUE;
	    }
	}
    }

    return FALSE;
}

/*
 * Returns TRUE if the constructor is valid.
 */
    static int
is_valid_constructor(ufunc_T *uf, int is_abstract, int has_static)
{
    // Constructors are not allowed in abstract classes.
    if (is_abstract)
    {
	emsg(_(e_cannot_define_new_method_in_abstract_class));
	return FALSE;
    }
    // A constructor is always static, no need to define it so.
    if (has_static)
    {
	emsg(_(e_cannot_define_new_method_as_static));
	return FALSE;
    }
    // A return type should not be specified for the new()
    // constructor method.
    if (uf->uf_ret_type->tt_type != VAR_VOID)
    {
	emsg(_(e_cannot_use_a_return_type_with_new_method));
	return FALSE;
    }
    return TRUE;
}

/*
 * Update the interface class lookup table for the member index on the
 * interface to the member index in the class implementing the interface.
 * And a lookup table for the object method index on the interface
 * to the object method index in the class implementing the interface.
 * This is also used for updating the lookup table for the extended class
 * hierarchy.
 */
    static int
update_member_method_lookup_table(
    class_T	*ifcl,
    class_T	*cl,
    garray_T	*objmethods,
    int		pobj_method_offset)
{
    if (ifcl == NULL)
	return OK;

    // Table for members.
    itf2class_T *if2cl = alloc_clear(sizeof(itf2class_T)
				+ ifcl->class_obj_member_count * sizeof(int));
    if (if2cl == NULL)
	return FAIL;
    if2cl->i2c_next = ifcl->class_itf2class;
    ifcl->class_itf2class = if2cl;
    if2cl->i2c_class = cl;
    if2cl->i2c_is_method = FALSE;

    for (int if_i = 0; if_i < ifcl->class_obj_member_count; ++if_i)
	for (int cl_i = 0; cl_i < cl->class_obj_member_count; ++cl_i)
	{
	    if (STRCMP(ifcl->class_obj_members[if_i].ocm_name,
				cl->class_obj_members[cl_i].ocm_name) == 0)
	    {
		int *table = (int *)(if2cl + 1);
		table[if_i] = cl_i;
		break;
	    }
	}

    // Table for methods.
    if2cl = alloc_clear(sizeof(itf2class_T)
				+ ifcl->class_obj_method_count * sizeof(int));
    if (if2cl == NULL)
	return FAIL;
    if2cl->i2c_next = ifcl->class_itf2class;
    ifcl->class_itf2class = if2cl;
    if2cl->i2c_class = cl;
    if2cl->i2c_is_method = TRUE;

    for (int if_i = 0; if_i < ifcl->class_obj_method_count; ++if_i)
    {
	int done = FALSE;
	for (int cl_i = 0; cl_i < objmethods->ga_len; ++cl_i)
	{
	    if (STRCMP(ifcl->class_obj_methods[if_i]->uf_name,
			((ufunc_T **)objmethods->ga_data)[cl_i]->uf_name) == 0)
	    {
		int *table = (int *)(if2cl + 1);
		table[if_i] = cl_i;
		done = TRUE;
		break;
	    }
	}

	// extended class object method is not overridden by the child class.
	// Keep the method declared in one of the parent classes in the
	// lineage.
	if (!done)
	{
	    // If "ifcl" is not the immediate parent of "cl", then search in
	    // the intermediate parent classes.
	    if (cl->class_extends != ifcl)
	    {
		class_T		*parent = cl->class_extends;
		int		method_offset = objmethods->ga_len;

		while (!done && parent != NULL && parent != ifcl)
		{

		    for (int cl_i = 0;
			    cl_i < parent->class_obj_method_count_child; ++cl_i)
		    {
			if (STRCMP(ifcl->class_obj_methods[if_i]->uf_name,
				    parent->class_obj_methods[cl_i]->uf_name)
				== 0)
			{
			    int *table = (int *)(if2cl + 1);
			    table[if_i] = method_offset + cl_i;
			    done = TRUE;
			    break;
			}
		    }
		    method_offset += parent->class_obj_method_count_child;
		    parent = parent->class_extends;
		}
	    }

	    if (!done)
	    {
		int *table = (int *)(if2cl + 1);
		table[if_i] = pobj_method_offset + if_i;
	    }
	}
    }

    return OK;
}

/*
 * Update the member and object method lookup tables for a new class in the
 * interface class.
 * For each interface add a lookup table for the member index on the interface
 * to the member index in the new class.  And a lookup table for the object
 * method index on the interface to the object method index in the new class.
 */
    static int
add_lookup_tables(class_T *cl, class_T *extends_cl, garray_T *objmethods_gap)
{
    // update the lookup table for all the implemented interfaces
    for (int i = 0; i < cl->class_interface_count; ++i)
    {
	class_T *ifcl = cl->class_interfaces_cl[i];

	// update the lookup table for this interface and all its super
	// interfaces.
	while (ifcl != NULL)
	{
	    if (update_member_method_lookup_table(ifcl, cl, objmethods_gap,
								0) == FAIL)
		return FAIL;
	    ifcl = ifcl->class_extends;
	}
    }

    // Update the lookup table for the extended class, if any
    if (extends_cl != NULL)
    {
	class_T		*pclass = extends_cl;
	int		pobj_method_offset = objmethods_gap->ga_len;

	// Update the entire lineage of extended classes.
	while (pclass != NULL)
	{
	    if (update_member_method_lookup_table(pclass, cl,
			objmethods_gap, pobj_method_offset) == FAIL)
		return FAIL;

	    pobj_method_offset += pclass->class_obj_method_count_child;
	    pclass = pclass->class_extends;
	}
    }

    return OK;
}

/*
 * Add class members to a new class.  Allocate a typval for each class member
 * and initialize it.
 */
    static void
add_class_members(class_T *cl, exarg_T *eap, garray_T *type_list_gap)
{
    // Allocate a typval for each class member and initialize it.
    cl->class_members_tv = ALLOC_CLEAR_MULT(typval_T,
					    cl->class_class_member_count);
    if (cl->class_members_tv == NULL)
	return;

    for (int i = 0; i < cl->class_class_member_count; ++i)
    {
	ocmember_T	*m = &cl->class_class_members[i];
	typval_T	*tv = &cl->class_members_tv[i];
	if (m->ocm_init != NULL)
	{
	    typval_T *etv = eval_expr(m->ocm_init, eap);
	    if (etv != NULL)
	    {
		if (m->ocm_type->tt_type == VAR_ANY
			&& !(m->ocm_flags & OCMFLAG_HAS_TYPE)
			&& etv->v_type != VAR_SPECIAL)
		    // If the member variable type is not yet set, then use
		    // the initialization expression type.
		    m->ocm_type = typval2type(etv, get_copyID(),
					type_list_gap,
					TVTT_DO_MEMBER|TVTT_MORE_SPECIFIC);
		*tv = *etv;
		vim_free(etv);
	    }
	}
	else
	{
	    // TODO: proper default value
	    tv->v_type = m->ocm_type->tt_type;
	    tv->vval.v_string = NULL;
	}
	if (m->ocm_flags & OCMFLAG_CONST)
	    item_lock(tv, DICT_MAXNEST, TRUE, TRUE);
    }
}

/*
 * Add a default constructor method (new()) to the class "cl".
 */
    static void
add_default_constructor(
    class_T	*cl,
    garray_T	*classfunctions_gap,
    garray_T	*type_list_gap)
{
    garray_T fga;

    ga_init2(&fga, 1, 1000);
    ga_concat(&fga, (char_u *)"new(");
    for (int i = 0; i < cl->class_obj_member_count; ++i)
    {
	if (i > 0)
	    ga_concat(&fga, (char_u *)", ");
	ga_concat(&fga, (char_u *)"this.");
	ocmember_T *m = cl->class_obj_members + i;
	ga_concat(&fga, (char_u *)m->ocm_name);
	ga_concat(&fga, (char_u *)" = v:none");
    }
    ga_concat(&fga, (char_u *)")\nenddef\n");
    ga_append(&fga, NUL);

    exarg_T fea;
    CLEAR_FIELD(fea);
    fea.cmdidx = CMD_def;
    fea.cmd = fea.arg = fga.ga_data;

    garray_T lines_to_free;
    ga_init2(&lines_to_free, sizeof(char_u *), 50);

    ufunc_T *nf = define_function(&fea, NULL, &lines_to_free, CF_CLASS,
			    cl->class_obj_members, cl->class_obj_member_count);

    ga_clear_strings(&lines_to_free);
    vim_free(fga.ga_data);

    if (nf != NULL && ga_grow(classfunctions_gap, 1) == OK)
    {
	((ufunc_T **)classfunctions_gap->ga_data)[classfunctions_gap->ga_len]
									= nf;
	++classfunctions_gap->ga_len;

	nf->uf_flags |= FC_NEW;
	nf->uf_ret_type = get_type_ptr(type_list_gap);
	if (nf->uf_ret_type != NULL)
	{
	    nf->uf_ret_type->tt_type = VAR_OBJECT;
	    nf->uf_ret_type->tt_class = cl;
	    nf->uf_ret_type->tt_argcount = 0;
	    nf->uf_ret_type->tt_args = NULL;
	}
    }
}

/*
 * Add the class methods and object methods to the new class "cl".
 * When extending a class "extends_cl", add the instance methods from the
 * parent class also.
 */
    static int
add_classfuncs_objmethods(
    class_T	*cl,
    class_T	*extends_cl,
    garray_T	*classfunctions_gap,
    garray_T	*objmethods_gap)
{
    // loop 1: class functions, loop 2: object methods
    for (int loop = 1; loop <= 2; ++loop)
    {
	garray_T	*gap = loop == 1 ? classfunctions_gap : objmethods_gap;
	int		*fcount = loop == 1 ? &cl->class_class_function_count
						: &cl->class_obj_method_count;
	ufunc_T		***fup = loop == 1 ? &cl->class_class_functions
						: &cl->class_obj_methods;

	int parent_count = 0;
	if (extends_cl != NULL)
	    // Include object methods from the parent.
	    // Don't include the parent class methods.
	    parent_count = loop == 1
				? 0
				: extends_cl->class_obj_method_count;

	*fcount = parent_count + gap->ga_len;
	if (*fcount == 0)
	{
	    *fup = NULL;
	    continue;
	}
	*fup = ALLOC_MULT(ufunc_T *, *fcount);
	if (*fup == NULL)
	    return FAIL;

	if (gap->ga_len != 0)
	    mch_memmove(*fup, gap->ga_data, sizeof(ufunc_T *) * gap->ga_len);
	vim_free(gap->ga_data);
	if (loop == 1)
	    cl->class_class_function_count_child = gap->ga_len;
	else
	    cl->class_obj_method_count_child = gap->ga_len;

	if (loop == 2)
	{
	    // Copy instance methods from the parent.

	    for (int i = 0; i < parent_count; ++i)
	    {
		// Can't use the same parent function, because "uf_class" is
		// different and compilation will have a different result.
		// Put them after the functions in the current class, object
		// methods may be overruled, then "super.Method()" is used to
		// find a method from the parent.
		ufunc_T *pf = (extends_cl->class_obj_methods)[i];
		(*fup)[gap->ga_len + i] = copy_function(pf);

		// If the child class overrides a function from the parent
		// the signature must be equal.
		char_u *pname = pf->uf_name;
		for (int ci = 0; ci < gap->ga_len; ++ci)
		{
		    ufunc_T *cf = (*fup)[ci];
		    char_u *cname = cf->uf_name;
		    if (STRCMP(pname, cname) == 0)
		    {
			where_T where = WHERE_INIT;
			where.wt_func_name = (char *)pname;
			where.wt_kind = WT_METHOD;
			(void)check_type(pf->uf_func_type, cf->uf_func_type,
								TRUE, where);
		    }
		}
	    }
	}

	// Set the class pointer on all the functions and object methods.
	for (int i = 0; i < *fcount; ++i)
	{
	    ufunc_T *fp = (*fup)[i];
	    fp->uf_class = cl;
	    if (i < gap->ga_len)
		fp->uf_defclass = cl;
	    if (loop == 2)
		fp->uf_flags |= FC_OBJECT;
	}
    }

    return OK;
}

/*
 * Return the end of the class name starting at "arg".  Valid characters in a
 * class name are alphanumeric characters and "_".  Also handles imported class
 * names.
 */
    static char_u *
find_class_name_end(char_u *arg)
{
    char_u *end = arg;

    while (ASCII_ISALNUM(*end) || *end == '_'
	    || (*end == '.' && (ASCII_ISALNUM(end[1]) || end[1] == '_')))
	++end;

    return end;
}


/*
 * Handle ":class" and ":abstract class" up to ":endclass".
 * Handle ":interface" up to ":endinterface".
 */
    void
ex_class(exarg_T *eap)
{
    int		is_class = eap->cmdidx == CMD_class;  // FALSE for :interface
    long	start_lnum = SOURCING_LNUM;
    char_u	*arg = eap->arg;
    int		is_abstract = eap->cmdidx == CMD_abstract;

    if (is_abstract)
    {
	if (STRNCMP(arg, "class", 5) != 0 || !VIM_ISWHITE(arg[5]))
	{
	    semsg(_(e_invalid_argument_str), arg);
	    return;
	}
	arg = skipwhite(arg + 5);
	is_class = TRUE;
    }

    if (!current_script_is_vim9()
		|| (cmdmod.cmod_flags & CMOD_LEGACY)
		|| !getline_equal(eap->ea_getline, eap->cookie, getsourceline))
    {
	if (is_class)
	    emsg(_(e_class_can_only_be_defined_in_vim9_script));
	else
	    emsg(_(e_interface_can_only_be_defined_in_vim9_script));
	return;
    }

    if (!ASCII_ISUPPER(*arg))
    {
	if (is_class)
	    semsg(_(e_class_name_must_start_with_uppercase_letter_str), arg);
	else
	    semsg(_(e_interface_name_must_start_with_uppercase_letter_str),
									  arg);
	return;
    }
    char_u *name_end = find_name_end(arg, NULL, NULL, FNE_CHECK_START);
    if (!IS_WHITE_OR_NUL(*name_end))
    {
	semsg(_(e_white_space_required_after_name_str), arg);
	return;
    }
    char_u *name_start = arg;

    // "export class" gets used when creating the class, don't use "is_export"
    // for the items inside the class.
    int class_export = is_export;
    is_export = FALSE;

    // TODO:
    //    generics: <Tkey, Tentry>

    // Name for "extends BaseClass"
    char_u *extends = NULL;

    // Names for "implements SomeInterface"
    garray_T	ga_impl;
    ga_init2(&ga_impl, sizeof(char_u *), 5);

    arg = skipwhite(name_end);
    while (*arg != NUL && *arg != '#' && *arg != '\n')
    {
	// TODO:
	//    specifies SomeInterface
	if (STRNCMP(arg, "extends", 7) == 0 && IS_WHITE_OR_NUL(arg[7]))
	{
	    if (extends != NULL)
	    {
		emsg(_(e_duplicate_extends));
		goto early_ret;
	    }
	    arg = skipwhite(arg + 7);

	    char_u *end = find_class_name_end(arg);
	    if (!IS_WHITE_OR_NUL(*end))
	    {
		semsg(_(e_white_space_required_after_name_str), arg);
		goto early_ret;
	    }
	    extends = vim_strnsave(arg, end - arg);
	    if (extends == NULL)
		goto early_ret;

	    arg = skipwhite(end + 1);
	}
	else if (STRNCMP(arg, "implements", 10) == 0
						   && IS_WHITE_OR_NUL(arg[10]))
	{
	    if (!is_class)
	    {
		emsg(_(e_interface_cannot_use_implements));
		goto early_ret;
	    }

	    if (ga_impl.ga_len > 0)
	    {
		emsg(_(e_duplicate_implements));
		goto early_ret;
	    }
	    arg = skipwhite(arg + 10);

	    for (;;)
	    {
		char_u *impl_end = find_class_name_end(arg);
		if ((!IS_WHITE_OR_NUL(*impl_end) && *impl_end != ',')
			|| (*impl_end == ','
			    && !IS_WHITE_OR_NUL(*(impl_end + 1))))
		{
		    semsg(_(e_white_space_required_after_name_str), arg);
		    goto early_ret;
		}
		if (impl_end - arg == 0)
		{
		    emsg(_(e_missing_name_after_implements));
		    goto early_ret;
		}

		char_u *iname = vim_strnsave(arg, impl_end - arg);
		if (iname == NULL)
		    goto early_ret;
		for (int i = 0; i < ga_impl.ga_len; ++i)
		    if (STRCMP(((char_u **)ga_impl.ga_data)[i], iname) == 0)
		    {
			semsg(_(e_duplicate_interface_after_implements_str),
									iname);
			vim_free(iname);
			goto early_ret;
		    }
		if (ga_add_string(&ga_impl, iname) == FAIL)
		{
		    vim_free(iname);
		    goto early_ret;
		}
		if (*impl_end != ',')
		{
		    arg = skipwhite(impl_end);
		    break;
		}
		arg = skipwhite(impl_end + 1);
	    }
	}
	else
	{
	    semsg(_(e_trailing_characters_str), arg);
early_ret:
	    vim_free(extends);
	    ga_clear_strings(&ga_impl);
	    return;
	}
    }

    garray_T	type_list;	    // list of pointers to allocated types
    ga_init2(&type_list, sizeof(type_T *), 10);

    // Growarray with class members declared in the class.
    garray_T classmembers;
    ga_init2(&classmembers, sizeof(ocmember_T), 10);

    // Growarray with functions declared in the class.
    garray_T classfunctions;
    ga_init2(&classfunctions, sizeof(ufunc_T *), 10);

    // Growarray with object members declared in the class.
    garray_T objmembers;
    ga_init2(&objmembers, sizeof(ocmember_T), 10);

    // Growarray with object methods declared in the class.
    garray_T objmethods;
    ga_init2(&objmethods, sizeof(ufunc_T *), 10);

    /*
     * Go over the body of the class/interface until "endclass" or
     * "endinterface" is found.
     */
    char_u *theline = NULL;
    int success = FALSE;
    for (;;)
    {
	vim_free(theline);
	theline = eap->ea_getline(':', eap->cookie, 0, GETLINE_CONCAT_ALL);
	if (theline == NULL)
	    break;
	char_u *line = skipwhite(theline);

	// Skip empty and comment lines.
	if (*line == NUL)
	    continue;
	if (*line == '#')
	{
	    if (vim9_bad_comment(line))
		break;
	    continue;
	}

	char_u *p = line;
	char *end_name = is_class ? "endclass" : "endinterface";
	if (checkforcmd(&p, end_name, is_class ? 4 : 5))
	{
	    if (STRNCMP(line, end_name, is_class ? 8 : 12) != 0)
		semsg(_(e_command_cannot_be_shortened_str), line);
	    else if (*p == '|' || !ends_excmd2(line, p))
		semsg(_(e_trailing_characters_str), p);
	    else
		success = TRUE;
	    break;
	}
	char *wrong_name = is_class ? "endinterface" : "endclass";
	if (checkforcmd(&p, wrong_name, is_class ? 5 : 4))
	{
	    semsg(_(e_invalid_command_str_expected_str), line, end_name);
	    break;
	}

	int has_public = FALSE;
	if (checkforcmd(&p, "public", 3))
	{
	    if (STRNCMP(line, "public", 6) != 0)
	    {
		semsg(_(e_command_cannot_be_shortened_str), line);
		break;
	    }
	    if (!is_class)
	    {
		emsg(_(e_public_variable_not_supported_in_interface));
		break;
	    }
	    has_public = TRUE;
	    p = skipwhite(line + 6);

	    if (STRNCMP(p, "var", 3) != 0 && STRNCMP(p, "static", 6) != 0
		&& STRNCMP(p, "final", 5) != 0 && STRNCMP(p, "const", 5) != 0)
	    {
		emsg(_(e_public_must_be_followed_by_var_static_final_or_const));
		break;
	    }
	}

	int abstract_method = FALSE;
	char_u *pa = p;
	if (checkforcmd(&p, "abstract", 3))
	{
	    if (STRNCMP(pa, "abstract", 8) != 0)
	    {
		semsg(_(e_command_cannot_be_shortened_str), pa);
		break;
	    }

	    if (!is_class)
	    {
		// "abstract" not supported in an interface
		emsg(_(e_abstract_cannot_be_used_in_interface));
		break;
	    }

	    if (!is_abstract)
	    {
		semsg(_(e_abstract_method_in_concrete_class), pa);
		break;
	    }

	    p = skipwhite(pa + 8);
	    if (STRNCMP(p, "def", 3) != 0)
	    {
		emsg(_(e_abstract_must_be_followed_by_def));
		break;
	    }

	    abstract_method = TRUE;
	}

	int has_static = FALSE;
	char_u *ps = p;
	if (checkforcmd(&p, "static", 4))
	{
	    if (STRNCMP(ps, "static", 6) != 0)
	    {
		semsg(_(e_command_cannot_be_shortened_str), ps);
		break;
	    }

	    if (!is_class)
	    {
		emsg(_(e_static_member_not_supported_in_interface));
		break;
	    }
	    has_static = TRUE;
	    p = skipwhite(ps + 6);

	    if (STRNCMP(p, "var", 3) != 0 && STRNCMP(p, "def", 3) != 0
		&& STRNCMP(p, "final", 5) != 0 && STRNCMP(p, "const", 5) != 0)
	    {
		emsg(_(e_static_must_be_followed_by_var_def_final_or_const));
		break;
	    }
	}

	int has_final = FALSE;
	int has_var = FALSE;
	int has_const = FALSE;
	if (checkforcmd(&p, "var", 3))
	    has_var = TRUE;
	else if (checkforcmd(&p, "final", 5))
	{
	    if (!is_class)
	    {
		emsg(_(e_final_variable_not_supported_in_interface));
		break;
	    }
	    has_final = TRUE;
	}
	else if (checkforcmd(&p, "const", 5))
	{
	    if (!is_class)
	    {
		emsg(_(e_const_variable_not_supported_in_interface));
		break;
	    }
	    has_const = TRUE;
	}
	p = skipwhite(p);

	// object members (public, read access, private):
	//	"var _varname"
	//	"var varname"
	//	"public var varname"
	//	"final _varname"
	//	"final varname"
	//	"public final varname"
	//	"const _varname"
	//	"const varname"
	//	"public const varname"
	// class members (public, read access, private):
	//	"static var _varname"
	//	"static var varname"
	//	"public static var varname"
	//	"static final _varname"
	//	"static final varname"
	//	"public static final varname"
	//	"static const _varname"
	//	"static const varname"
	//	"public static const varname"
	if (has_var || has_final || has_const)
	{
	    char_u *varname = p;
	    char_u *varname_end = NULL;
	    type_T *type = NULL;
	    char_u *init_expr = NULL;
	    int	    has_type = FALSE;

	    if (!eval_isnamec1(*p))
	    {
		if (has_static)
		    semsg(_(e_invalid_class_variable_declaration_str), line);
		else
		    semsg(_(e_invalid_object_variable_declaration_str), line);
		break;
	    }

	    if (!is_class && *varname == '_')
	    {
		// private variables are not supported in an interface
		semsg(_(e_protected_variable_not_supported_in_interface),
			varname);
		break;
	    }

	    if (parse_member(eap, line, varname, has_public,
			  &varname_end, &has_type, &type_list, &type,
			  is_class ? &init_expr: NULL) == FAIL)
		break;
	    if (is_reserved_varname(varname, varname_end))
	    {
		vim_free(init_expr);
		break;
	    }
	    if (is_duplicate_variable(&classmembers, &objmembers, varname,
								varname_end))
	    {
		vim_free(init_expr);
		break;
	    }
	    if (add_member(has_static ? &classmembers : &objmembers, varname,
				varname_end, has_public, has_final, has_const,
				has_type, type, init_expr) == FAIL)
	    {
		vim_free(init_expr);
		break;
	    }
	}

	// constructors:
	//	  def new()
	//	  enddef
	//	  def newOther()
	//	  enddef
	// object methods and class functions:
	//	  def SomeMethod()
	//	  enddef
	//	  static def ClassFunction()
	//	  enddef
	// TODO:
	//	  def <Tval> someMethod()
	//	  enddef
	else if (checkforcmd(&p, "def", 3))
	{
	    exarg_T	ea;
	    garray_T	lines_to_free;

	    if (has_public)
	    {
		// "public" keyword is not supported when defining an object or
		// class method
		emsg(_(e_public_keyword_not_supported_for_method));
		break;
	    }

	    if (*p == NUL)
	    {
		// No method name following def
		semsg(_(e_not_valid_command_in_class_str), line);
		break;
	    }

	    if (*p == '_' && *(p + 1) == '_')
	    {
		// double underscore prefix for a method name is currently
		// reserved.  This could be used in the future to support
		// object methods called by Vim builtin functions.
		semsg(_(e_cannot_use_reserved_name_str), p);
		break;
	    }

	    CLEAR_FIELD(ea);
	    ea.cmd = line;
	    ea.arg = p;
	    ea.cmdidx = CMD_def;
	    ea.ea_getline = eap->ea_getline;
	    ea.cookie = eap->cookie;

	    ga_init2(&lines_to_free, sizeof(char_u *), 50);
	    int class_flags;
	    if (is_class)
		class_flags = abstract_method ? CF_ABSTRACT_METHOD : CF_CLASS;
	    else
		class_flags = CF_INTERFACE;
	    ufunc_T *uf = define_function(&ea, NULL, &lines_to_free,
			class_flags, objmembers.ga_data, objmembers.ga_len);
	    ga_clear_strings(&lines_to_free);

	    if (uf != NULL)
	    {
		char_u	*name = uf->uf_name;
		int	is_new = STRNCMP(name, "new", 3) == 0;

		if (!is_class && *name == '_')
		{
		    // private variables are not supported in an interface
		    semsg(_(e_protected_method_not_supported_in_interface),
			    name);
		    func_clear_free(uf, FALSE);
		    break;
		}
		if (is_new && !is_valid_constructor(uf, is_abstract,
								has_static))
		{
		    func_clear_free(uf, FALSE);
		    break;
		}

		// Check the name isn't used already.
		if (is_duplicate_method(&classfunctions, &objmethods, name))
		{
		    success = FALSE;
		    func_clear_free(uf, FALSE);
		    break;
		}

		garray_T *fgap = has_static || is_new
					       ? &classfunctions : &objmethods;
		if (ga_grow(fgap, 1) == OK)
		{
		    if (is_new)
			uf->uf_flags |= FC_NEW;

		    if (abstract_method)
			uf->uf_flags |= FC_ABSTRACT;

		    ((ufunc_T **)fgap->ga_data)[fgap->ga_len] = uf;
		    ++fgap->ga_len;
		}
	    }
	}

	else
	{
	    if (is_class)
		semsg(_(e_not_valid_command_in_class_str), line);
	    else
		semsg(_(e_not_valid_command_in_interface_str), line);
	    break;
	}
    }
    vim_free(theline);

    class_T *extends_cl = NULL;  // class from "extends" argument

    /*
     * Check a few things before defining the class.
     */

    // Check the "extends" class is valid.
    if (success && extends != NULL)
	success = validate_extends_class(extends, &extends_cl, is_class);
    VIM_CLEAR(extends);

    // Check the new object methods to make sure their access (public or
    // private) is the same as that in the extended class lineage.
    if (success && extends_cl != NULL)
	success = validate_extends_methods(&objmethods, extends_cl);

    // Check the new class and object variables are not duplicates of the
    // variables in the extended class lineage.  If an interface is extending
    // another interface, then it can duplicate the member variables.
    if (success && extends_cl != NULL)
    {
	if (is_class)
	    success = extends_check_dup_members(&objmembers, extends_cl);
	else
	    success = extends_check_intf_var_type(&objmembers, extends_cl);
    }

    // When extending an abstract class, make sure all the abstract methods in
    // the parent class are implemented.  If the current class is an abstract
    // class, then there is no need for this check.
    if (success && !is_abstract && extends_cl != NULL
				&& (extends_cl->class_flags & CLASS_ABSTRACT))
	success = validate_abstract_class_methods(&classfunctions,
						&objmethods, extends_cl);

    class_T **intf_classes = NULL;

    // Check all "implements" entries are valid.
    if (success && ga_impl.ga_len > 0)
    {
	intf_classes = ALLOC_CLEAR_MULT(class_T *, ga_impl.ga_len);

	success = validate_implements_classes(&ga_impl, intf_classes,
					&objmethods, &objmembers, extends_cl);
    }

    // Check no function argument name is used as a class member.
    if (success)
	success = check_func_arg_names(&classfunctions, &objmethods,
							&classmembers);

    class_T *cl = NULL;
    if (success)
    {
	// "endclass" encountered without failures: Create the class.

	cl = ALLOC_CLEAR_ONE(class_T);
	if (cl == NULL)
	    goto cleanup;
	if (!is_class)
	    cl->class_flags = CLASS_INTERFACE;
	else if (is_abstract)
	    cl->class_flags = CLASS_ABSTRACT;

	cl->class_refcount = 1;
	cl->class_name = vim_strnsave(name_start, name_end - name_start);
	if (cl->class_name == NULL)
	    goto cleanup;

	if (extends_cl != NULL)
	{
	    cl->class_extends = extends_cl;
	    extends_cl->class_flags |= CLASS_EXTENDED;
	}

	// Add class and object variables to "cl".
	if (add_members_to_class(&classmembers,
				 NULL,
				 0,
				 &cl->class_class_members,
				 &cl->class_class_member_count) == FAIL
		|| add_members_to_class(&objmembers,
				 extends_cl == NULL ? NULL
					       : extends_cl->class_obj_members,
				 extends_cl == NULL ? 0
					  : extends_cl->class_obj_member_count,
				 &cl->class_obj_members,
				 &cl->class_obj_member_count) == FAIL)
	    goto cleanup;

	if (ga_impl.ga_len > 0)
	{
	    // Move the "implements" names into the class.
	    cl->class_interface_count = ga_impl.ga_len;
	    cl->class_interfaces = ALLOC_MULT(char_u *, ga_impl.ga_len);
	    if (cl->class_interfaces == NULL)
		goto cleanup;
	    for (int i = 0; i < ga_impl.ga_len; ++i)
		cl->class_interfaces[i] = ((char_u **)ga_impl.ga_data)[i];
	    VIM_CLEAR(ga_impl.ga_data);
	    ga_impl.ga_len = 0;

	    cl->class_interfaces_cl = intf_classes;
	    intf_classes = NULL;
	}

	if (cl->class_interface_count > 0 || extends_cl != NULL)
	{
	    // Add a method and member lookup table to each of the interface
	    // classes.
	    if (add_lookup_tables(cl, extends_cl, &objmethods) == FAIL)
		goto cleanup;
	}

	// Allocate a typval for each class member and initialize it.
	if (is_class && cl->class_class_member_count > 0)
	    add_class_members(cl, eap, &type_list);

	int	have_new = FALSE;
	ufunc_T	*class_func = NULL;
	for (int i = 0; i < classfunctions.ga_len; ++i)
	{
	    class_func = ((ufunc_T **)classfunctions.ga_data)[i];
	    if (STRCMP(class_func->uf_name, "new") == 0)
	    {
		have_new = TRUE;
		break;
	    }
	}

	if (have_new)
	    // The return type of new() is an object of class "cl"
	    class_func->uf_ret_type->tt_class = cl;
	else if (is_class && !is_abstract && !have_new)
	    // No new() method was defined, add the default constructor.
	    add_default_constructor(cl, &classfunctions, &type_list);

	// Move all the functions into the created class.
	if (add_classfuncs_objmethods(cl, extends_cl, &classfunctions,
							&objmethods) == FAIL)
	    goto cleanup;

	cl->class_type.tt_type = VAR_CLASS;
	cl->class_type.tt_class = cl;
	cl->class_object_type.tt_type = VAR_OBJECT;
	cl->class_object_type.tt_class = cl;
	cl->class_type_list = type_list;

	class_created(cl);

	// TODO:
	// - Fill hashtab with object members and methods ?

	// Add the class to the script-local variables.
	// TODO: handle other context, e.g. in a function
	// TODO: does uf_hash need to be cleared?
	typval_T tv;
	tv.v_type = VAR_CLASS;
	tv.vval.v_class = cl;
	is_export = class_export;
	SOURCING_LNUM = start_lnum;
	set_var_const(cl->class_name, current_sctx.sc_sid,
						       NULL, &tv, FALSE, 0, 0);
	return;
    }

cleanup:
    if (cl != NULL)
    {
	vim_free(cl->class_name);
	vim_free(cl->class_class_functions);
	if (cl->class_interfaces != NULL)
	{
	    for (int i = 0; i < cl->class_interface_count; ++i)
		vim_free(cl->class_interfaces[i]);
	    vim_free(cl->class_interfaces);
	}
	if (cl->class_interfaces_cl != NULL)
	{
	    for (int i = 0; i < cl->class_interface_count; ++i)
		class_unref(cl->class_interfaces_cl[i]);
	    vim_free(cl->class_interfaces_cl);
	}
	vim_free(cl->class_obj_members);
	vim_free(cl->class_obj_methods);
	vim_free(cl);
    }

    vim_free(extends);
    class_unref(extends_cl);

    if (intf_classes != NULL)
    {
	for (int i = 0; i < ga_impl.ga_len; ++i)
	    class_unref(intf_classes[i]);
	vim_free(intf_classes);
    }
    ga_clear_strings(&ga_impl);

    for (int round = 1; round <= 2; ++round)
    {
	garray_T *gap = round == 1 ? &classmembers : &objmembers;
	if (gap->ga_len == 0 || gap->ga_data == NULL)
	    continue;

	for (int i = 0; i < gap->ga_len; ++i)
	{
	    ocmember_T *m = ((ocmember_T *)gap->ga_data) + i;
	    vim_free(m->ocm_name);
	    vim_free(m->ocm_init);
	}
	ga_clear(gap);
    }

    for (int i = 0; i < objmethods.ga_len; ++i)
    {
	ufunc_T *uf = ((ufunc_T **)objmethods.ga_data)[i];
	func_clear_free(uf, FALSE);
    }
    ga_clear(&objmethods);

    for (int i = 0; i < classfunctions.ga_len; ++i)
    {
	ufunc_T *uf = ((ufunc_T **)classfunctions.ga_data)[i];
	func_clear_free(uf, FALSE);
    }
    ga_clear(&classfunctions);

    clear_type_list(&type_list);
}

/*
 * Find member "name" in class "cl", set "member_idx" to the member index and
 * return its type.
 * When "is_object" is TRUE, then look for object members.  Otherwise look for
 * class members.
 * When not found "member_idx" is set to -1 and t_any is returned.
 * Set *p_m ocmmember_T if not NULL
 */
    type_T *
oc_member_type(
    class_T	*cl,
    int		is_object,
    char_u	*name,
    char_u	*name_end,
    int		*member_idx)
{
    size_t	len = name_end - name;
    ocmember_T	*m;

    *member_idx = -1;  // not found (yet)

    m = member_lookup(cl, is_object ? VAR_OBJECT : VAR_CLASS, name, len,
								member_idx);
    if (m == NULL)
    {
	member_not_found_msg(cl, is_object ? VAR_OBJECT : VAR_CLASS, name,
									len);
	return &t_any;
    }

    return m->ocm_type;
}

/*
 * Given a class or object variable index, return the variable type
 */
    type_T *
oc_member_type_by_idx(
    class_T	*cl,
    int		is_object,
    int		member_idx)
{
    ocmember_T	*m;
    int		member_count;

    if (is_object)
    {
	m = cl->class_obj_members;
	member_count = cl->class_obj_member_count;
    }
    else
    {
	m = cl->class_class_members;
	member_count = cl->class_class_member_count;
    }

    if (member_idx >= member_count)
	return NULL;

    return m[member_idx].ocm_type;
}

/*
 * Handle ":enum" up to ":endenum".
 */
    void
ex_enum(exarg_T *eap UNUSED)
{
    // TODO
}

/*
 * Type aliases (:type)
 */

    void
typealias_free(typealias_T *ta)
{
    // ta->ta_type is freed in clear_type_list()
    vim_free(ta->ta_name);
    vim_free(ta);
}

    void
typealias_unref(typealias_T *ta)
{
    if (ta != NULL && --ta->ta_refcount <= 0)
	typealias_free(ta);
}

/*
 * Handle ":type".  Create an alias for a type specification.
 */
    void
ex_type(exarg_T *eap UNUSED)
{
    char_u	*arg = eap->arg;

    if (!current_script_is_vim9()
		|| (cmdmod.cmod_flags & CMOD_LEGACY)
		|| !getline_equal(eap->ea_getline, eap->cookie, getsourceline))
    {
	emsg(_(e_type_can_only_be_defined_in_vim9_script));
	return;
    }

    if (*arg == NUL)
    {
	emsg(_(e_missing_typealias_name));
	return;
    }

    if (!ASCII_ISUPPER(*arg))
    {
	semsg(_(e_type_name_must_start_with_uppercase_letter_str), arg);
	return;
    }

    char_u *name_end = find_name_end(arg, NULL, NULL, FNE_CHECK_START);
    if (!IS_WHITE_OR_NUL(*name_end))
    {
	semsg(_(e_white_space_required_after_name_str), arg);
	return;
    }
    char_u *name_start = arg;

    arg = skipwhite(name_end);
    if (*arg != '=')
    {
	semsg(_(e_missing_equal_str), arg);
	return;
    }
    if (!IS_WHITE_OR_NUL(*(arg + 1)))
    {
	semsg(_(e_white_space_required_after_str_str), "=", arg);
	return;
    }
    arg++;
    arg = skipwhite(arg);

    if (*arg == NUL)
    {
	emsg(_(e_missing_typealias_type));
	return;
    }

    scriptitem_T    *si = SCRIPT_ITEM(current_sctx.sc_sid);
    type_T *type = parse_type(&arg, &si->sn_type_list, TRUE);
    if (type == NULL)
	return;

    if (*arg != NUL)
    {
	// some text after the type
	semsg(_(e_trailing_characters_str), arg);
	return;
    }

    int cc = *name_end;
    *name_end = NUL;

    typval_T tv;
    tv.v_type = VAR_UNKNOWN;
    if (eval_variable_import(name_start, &tv) == OK)
    {
	if (tv.v_type == VAR_TYPEALIAS)
	    semsg(_(e_typealias_already_exists_for_str), name_start);
	else
	    semsg(_(e_redefining_script_item_str), name_start);
	clear_tv(&tv);
	goto done;
    }

    // Create a script-local variable for the type alias.
    if (type->tt_type != VAR_OBJECT)
    {
	tv.v_type = VAR_TYPEALIAS;
	tv.v_lock = 0;
	tv.vval.v_typealias = ALLOC_CLEAR_ONE(typealias_T);
	++tv.vval.v_typealias->ta_refcount;
	tv.vval.v_typealias->ta_name = vim_strsave(name_start);
	tv.vval.v_typealias->ta_type = type;
    }
    else
    {
	// When creating a type alias for a class, use the class type itself to
	// create the type alias variable.  This is needed to use the type
	// alias to invoke class methods (e.g. new()) and use class variables.
	tv.v_type = VAR_CLASS;
	tv.v_lock = 0;
	tv.vval.v_class = type->tt_class;
	++tv.vval.v_class->class_refcount;
    }
    set_var_const(name_start, current_sctx.sc_sid, NULL, &tv, FALSE,
						ASSIGN_CONST | ASSIGN_FINAL, 0);

done:
    *name_end = cc;
}

/*
 * Returns OK if a member variable named "name" is present in the class "cl".
 * Otherwise returns FAIL.  If found, the member variable typval is set in
 * "rettv".  If "is_object" is TRUE, then the object member variable table is
 * searched.  Otherwise the class member variable table is searched.
 */
    static int
get_member_tv(
    class_T	*cl,
    int		is_object,
    char_u	*name,
    size_t	namelen,
    typval_T	*rettv)
{
    ocmember_T *m;
    int		m_idx;

    m = member_lookup(cl, is_object ? VAR_OBJECT : VAR_CLASS, name, namelen,
								&m_idx);
    if (m == NULL)
	return FAIL;

    if (*name == '_')
    {
	emsg_var_cl_define(e_cannot_access_protected_variable_str,
							m->ocm_name, 0, cl);
	return FAIL;
    }

    if (is_object)
    {
	// The object only contains a pointer to the class, the member values
	// array follows right after that.
	object_T *obj = rettv->vval.v_object;
	typval_T *tv = (typval_T *)(obj + 1) + m_idx;
	copy_tv(tv, rettv);
	object_unref(obj);
    }
    else
    {
	copy_tv(&cl->class_members_tv[m_idx], rettv);
	class_unref(cl);
    }

    return OK;
}

/*
 * Call an object or class method "name" in class "cl".  The method return
 * value is returned in "rettv".
 */
    static int
call_oc_method(
    class_T	*cl,
    char_u	*name,
    size_t	len,
    char_u	*name_end,
    evalarg_T	*evalarg,
    char_u	**arg,
    typval_T	*rettv)
{
    ufunc_T	*fp;
    typval_T	argvars[MAX_FUNC_ARGS + 1];
    int		argcount = 0;
    ocmember_T	*ocm = NULL;
    int		m_idx;

    fp = method_lookup(cl, rettv->v_type, name, len, NULL);
    if (fp == NULL)
    {
	// could be an object or class funcref variable
	ocm = member_lookup(cl, rettv->v_type, name, len, &m_idx);
	if (ocm == NULL || ocm->ocm_type->tt_type != VAR_FUNC)
	{
	    method_not_found_msg(cl, rettv->v_type, name, len);
	    return FAIL;
	}

	if (rettv->v_type == VAR_OBJECT)
	{
	    // funcref object variable
	    object_T	*obj = rettv->vval.v_object;
	    typval_T	*tv = (typval_T *)(obj + 1) + m_idx;
	    copy_tv(tv, rettv);
	}
	else
	    // funcref class variable
	    copy_tv(&cl->class_members_tv[m_idx], rettv);
	*arg = name_end;
	return OK;
    }

    if (ocm == NULL && *fp->uf_name == '_')
    {
	// Cannot access a private method outside of a class
	semsg(_(e_cannot_access_protected_method_str), fp->uf_name);
	return FAIL;
    }

    char_u *argp = name_end;
    int ret = get_func_arguments(&argp, evalarg, 0, argvars, &argcount, FALSE);
    if (ret == FAIL)
	return FAIL;

    funcexe_T funcexe;
    CLEAR_FIELD(funcexe);
    funcexe.fe_evaluate = TRUE;
    if (rettv->v_type == VAR_OBJECT)
    {
	funcexe.fe_object = rettv->vval.v_object;
	++funcexe.fe_object->obj_refcount;
    }

    // Clear the class or object after calling the function, in
    // case the refcount is one.
    typval_T tv_tofree = *rettv;
    rettv->v_type = VAR_UNKNOWN;

    // Call the user function.  Result goes into rettv;
    int error = call_user_func_check(fp, argcount, argvars, rettv, &funcexe,
								NULL);

    // Clear the previous rettv and the arguments.
    clear_tv(&tv_tofree);
    for (int idx = 0; idx < argcount; ++idx)
	clear_tv(&argvars[idx]);

    if (error != FCERR_NONE)
    {
	user_func_error(error, printable_func_name(fp), funcexe.fe_found_var);
	return FAIL;
    }
    *arg = argp;

    return OK;
}

/*
 * Evaluate what comes after a class:
 * - class member: SomeClass.varname
 * - class function: SomeClass.SomeMethod()
 * - class constructor: SomeClass.new()
 * - object member: someObject.varname
 * - object method: someObject.SomeMethod()
 *
 * "*arg" points to the '.'.
 * "*arg" is advanced to after the member name or method call.
 *
 * Returns FAIL or OK.
 */
    int
class_object_index(
    char_u	**arg,
    typval_T	*rettv,
    evalarg_T	*evalarg,
    int		verbose UNUSED)	// give error messages
{
    if (VIM_ISWHITE((*arg)[1]))
    {
	semsg(_(e_no_white_space_allowed_after_str_str), ".", *arg);
	return FAIL;
    }

    ++*arg;
    char_u *name = *arg;
    char_u *name_end = find_name_end(name, NULL, NULL, FNE_CHECK_START);
    if (name_end == name)
	return FAIL;
    size_t len = name_end - name;

    int did_emsg_save = did_emsg;
    class_T *cl;
    if (rettv->v_type == VAR_CLASS)
	cl = rettv->vval.v_class;
    else // VAR_OBJECT
    {
	if (rettv->vval.v_object == NULL)
	{
	    emsg(_(e_using_null_object));
	    return FAIL;
	}
	cl = rettv->vval.v_object->obj_class;
    }

    if (cl == NULL)
    {
	emsg(_(e_incomplete_type));
	return FAIL;
    }

    if (*name_end == '(')
	// Invoke the class or object method
	return call_oc_method(cl, name, len, name_end, evalarg, arg, rettv);

    else if (rettv->v_type == VAR_OBJECT || rettv->v_type == VAR_CLASS)
    {
	// Search in the object member variable table and the class member
	// variable table.
	int is_object = rettv->v_type == VAR_OBJECT;
	if (get_member_tv(cl, is_object, name, len, rettv) == OK)
	{
	    *arg = name_end;
	    return OK;
	}

	// could be a class method or an object method
	int	fidx;
	ufunc_T	*fp = method_lookup(cl, rettv->v_type, name, len, &fidx);
	if (fp != NULL)
	{
	    // Private methods are not accessible outside the class
	    if (*name == '_')
	    {
		semsg(_(e_cannot_access_protected_method_str), fp->uf_name);
		return FAIL;
	    }

	    partial_T	*pt = ALLOC_CLEAR_ONE(partial_T);
	    if (pt == NULL)
		return FAIL;

	    pt->pt_refcount = 1;
	    if (is_object)
	    {
		pt->pt_obj = rettv->vval.v_object;
		++pt->pt_obj->obj_refcount;
	    }
	    pt->pt_auto = TRUE;
	    pt->pt_func = fp;
	    func_ptr_ref(pt->pt_func);
	    rettv->v_type = VAR_PARTIAL;
	    rettv->vval.v_partial = pt;
	    *arg = name_end;
	    return OK;
	}

	if (did_emsg == did_emsg_save)
	    member_not_found_msg(cl, rettv->v_type, name, len);
    }

    return FAIL;
}

/*
 * If "arg" points to a class or object method, return it.
 * Otherwise return NULL.
 */
    ufunc_T *
find_class_func(char_u **arg)
{
    char_u *name = *arg;
    char_u *name_end = find_name_end(name, NULL, NULL, FNE_CHECK_START);
    if (name_end == name || *name_end != '.')
	return NULL;

    ufunc_T	*fp = NULL;
    size_t	len = name_end - name;
    typval_T	tv;
    tv.v_type = VAR_UNKNOWN;
    if (eval_variable(name, (int)len,
				    0, &tv, NULL, EVAL_VAR_NOAUTOLOAD) == FAIL)
	return NULL;
    if (tv.v_type != VAR_CLASS && tv.v_type != VAR_OBJECT)
	goto fail_after_eval;

    class_T *cl = tv.v_type == VAR_CLASS ? tv.vval.v_class
						 : tv.vval.v_object->obj_class;
    if (cl == NULL)
	goto fail_after_eval;
    char_u *fname = name_end + 1;
    char_u *fname_end = find_name_end(fname, NULL, NULL, FNE_CHECK_START);
    if (fname_end == fname)
	goto fail_after_eval;
    len = fname_end - fname;

    fp = method_lookup(cl, tv.v_type, fname, len, NULL);

fail_after_eval:
    clear_tv(&tv);
    return fp;
}

/*
 * Returns the index of class variable "name" in the class "cl".
 * Returns -1, if the variable is not found.
 * If "namelen" is zero, then it is assumed that "name" is NUL terminated.
 */
    int
class_member_idx(class_T *cl, char_u *name, size_t namelen)
{
    int idx;
    class_member_lookup(cl, name, namelen, &idx);
    return idx;
}

/*
 * Returns a pointer to the class member variable "name" in the class "cl".
 * Returns NULL if the variable is not found.
 * The member variable index is set in "idx".
 */
    ocmember_T *
class_member_lookup(class_T *cl, char_u *name, size_t namelen, int *idx)
{
    ocmember_T	*ret_m = NULL;
    int		ret_idx = -1;
    for (int i = 0; i < cl->class_class_member_count; ++i)
    {
	ocmember_T *m = &cl->class_class_members[i];
	if (namelen)
	{
	    if (STRNCMP(name, m->ocm_name, namelen) == 0
		    && m->ocm_name[namelen] == NUL)
	    {
		ret_m = m;
		ret_idx = i;
		break;
	    }
	}
	else if (STRCMP(name, m->ocm_name) == 0)
	{
	    ret_m = m;
	    ret_idx = i;
            break;
	}
    }
    if (idx != NULL)
	*idx = ret_idx;
    return ret_m;
}

/*
 * Returns a pointer to the class method "name" in class "cl".
 * Returns NULL if the method is not found.
 * The method index is set in "idx".
 */
    static ufunc_T *
class_method_lookup(class_T *cl, char_u *name, size_t namelen, int *idx)
{
    ufunc_T	*ret_fp = NULL;
    int		ret_idx = -1;
    for (int i = 0; i < cl->class_class_function_count; ++i)
    {
	ufunc_T *fp = cl->class_class_functions[i];
	char_u *ufname = (char_u *)fp->uf_name;
	if (STRNCMP(name, ufname, namelen) == 0 && ufname[namelen] == NUL)
	{
	    ret_fp = fp;
	    ret_idx = i;
	    break;
	}
    }
    if (idx != NULL)
	*idx = ret_idx;
    return ret_fp;
}

/*
 * Returns the index of class method "name" in the class "cl".
 * Returns -1, if the method is not found.
 */
    int
class_method_idx(class_T *cl, char_u *name, size_t namelen)
{
    int idx;
    class_method_lookup(cl, name, namelen, &idx);
    return idx;
}

/*
 * Returns the index of object member variable "name" in the class "cl".
 * Returns -1, if the variable is not found.
 * If "namelen" is zero, then it is assumed that "name" is NUL terminated.
 */
    static int
object_member_idx(class_T *cl, char_u *name, size_t namelen)
{
    int idx;
    object_member_lookup(cl, name, namelen, &idx);
    return idx;
}

/*
 * Returns a pointer to the object member variable "name" in the class "cl".
 * Returns NULL if the variable is not found.
 * The object member variable index is set in "idx".
 */
    ocmember_T *
object_member_lookup(class_T *cl, char_u *name, size_t namelen, int *idx)
{
    ocmember_T	*ret_m = NULL;
    int		ret_idx = -1;
    for (int i = 0; i < cl->class_obj_member_count; ++i)
    {
	ocmember_T  *m = &cl->class_obj_members[i];
	if (namelen)
	{
	    if (STRNCMP(name, m->ocm_name, namelen) == 0
		    && m->ocm_name[namelen] == NUL)
	    {
		ret_m = m;
		ret_idx = i;
		break;
	    }
	}
	else if (STRCMP(name, m->ocm_name) == 0)
        {
	    ret_m = m;
	    ret_idx = i;
            break;
        }
    }
    if (idx != NULL)
	*idx = ret_idx;
    return ret_m;
}

/*
 * Returns a pointer to the object method "name" in class "cl".
 * Returns NULL if the method is not found.
 * The object method index is set in "idx".
 */
    static ufunc_T *
object_method_lookup(class_T *cl, char_u *name, size_t namelen, int *idx)
{
    ufunc_T	*ret_fp = NULL;
    int		ret_idx = -1;
    for (int i = 0; i < cl->class_obj_method_count; ++i)
    {
	ufunc_T *fp = cl->class_obj_methods[i];
	// Use a separate pointer to avoid that ASAN complains about
	// uf_name[] only being 4 characters.
	char_u *ufname = (char_u *)fp->uf_name;
	if (STRNCMP(name, ufname, namelen) == 0 && ufname[namelen] == NUL)
	{
	    ret_fp = fp;
	    ret_idx = i;
	    break;
	}
    }
    if (idx != NULL)
	*idx = ret_idx;
    return ret_fp;
}

/*
 * Returns the index of object method "name" in the class "cl".
 * Returns -1, if the method is not found.
 */
    int
object_method_idx(class_T *cl, char_u *name, size_t namelen)
{
    int idx;
    object_method_lookup(cl, name, namelen, &idx);
    return idx;
}

/*
 * Lookup a class or object member variable by name.  If v_type is VAR_CLASS,
 * then lookup a class member variable and if it is VAR_OBJECT, then lookup a
 * object member variable.
 *
 * Returns a pointer to the member variable structure if variable is found.
 * Otherwise returns NULL.  The member variable index is set in "*idx".
 */
    ocmember_T *
member_lookup(
    class_T	*cl,
    vartype_T	v_type,
    char_u	*name,
    size_t	namelen,
    int		*idx)
{
    if (v_type == VAR_CLASS)
	return class_member_lookup(cl, name, namelen, idx);
    else
	return object_member_lookup(cl, name, namelen, idx);
}

/*
 * Find the class that defines the named member. Look up the hierarchy
 * starting at "cl".
 *
 * Return the class that defines the member "name", else NULL.
 * Fill in "p_m", if specified, for ocmember_T in found class.
 */
// NOTE: if useful for something could also indirectly return vartype and idx.
    static class_T *
class_defining_member(class_T *cl, char_u *name, size_t len, ocmember_T **p_m)
{
    class_T	*cl_found = NULL;
    vartype_T	vartype = VAR_UNKNOWN;
    ocmember_T	*m_found = NULL;

    len = len != 0 ? len : STRLEN(name);

    // Loop assumes if member is not defined in "cl", then it is not
    // defined in any super class; the last class where it's found is the
    // class where it is defined. Once the vartype is found, the other
    // type is no longer checked.
    for (class_T *super = cl; super != NULL; super = super->class_extends)
    {
	class_T		*cl_tmp = NULL;
	ocmember_T	*m = NULL;
	if (vartype == VAR_UNKNOWN || vartype == VAR_OBJECT)
	{
	    if ((m = object_member_lookup(super, name, len, NULL)) != NULL)
	    {
		cl_tmp = super;
		vartype = VAR_OBJECT;
	    }
	}
	if (vartype == VAR_UNKNOWN || vartype == VAR_CLASS)
	{
	    if (( m = class_member_lookup(super, name, len, NULL)) != NULL)
	    {
		cl_tmp = super;
		vartype = VAR_OBJECT;
	    }
	}
	if (cl_tmp == NULL)
	    break;  // member is not in this or any super class.
	cl_found = cl_tmp;
	m_found = m;
    }
    if (p_m != NULL)
	*p_m = m_found;
    return cl_found;
}

/*
 * Lookup a class or object method by name.  If v_type is VAR_CLASS, then
 * lookup a class method and if it is VAR_OBJECT, then lookup a object method.
 *
 * Returns a pointer to the method structure if variable is found.
 * Otherwise returns NULL.  The method variable index is set in "*idx".
 */
    ufunc_T *
method_lookup(
    class_T	*cl,
    vartype_T	v_type,
    char_u	*name,
    size_t	namelen,
    int		*idx)
{
    if (v_type == VAR_CLASS)
	return class_method_lookup(cl, name, namelen, idx);
    else
	return object_method_lookup(cl, name, namelen, idx);
}

/*
 * Return TRUE if current context "cctx_arg" is inside class "cl".
 * Return FALSE if not.
 */
    int
inside_class(cctx_T *cctx_arg, class_T *cl)
{
    for (cctx_T *cctx = cctx_arg; cctx != NULL; cctx = cctx->ctx_outer)
	if (cctx->ctx_ufunc != NULL
			&& class_instance_of(cctx->ctx_ufunc->uf_class, cl))
	    return TRUE;
    return FALSE;
}

/*
 * Return TRUE if object/class variable "m" is read-only.
 * Also give an error message.
 */
    int
oc_var_check_ro(class_T *cl, ocmember_T *m)
{
    if (m->ocm_flags & (OCMFLAG_FINAL | OCMFLAG_CONST))
    {
	semsg(_(e_cannot_change_readonly_variable_str_in_class_str),
		m->ocm_name, cl->class_name);
	return TRUE;
    }
    return FALSE;
}

/*
 * Lock all the constant object variables.  Called after creating and
 * initializing a new object.
 */
    void
obj_lock_const_vars(object_T *obj)
{
    for (int i = 0; i < obj->obj_class->class_obj_member_count; i++)
    {
	ocmember_T *ocm = &obj->obj_class->class_obj_members[i];
	if (ocm->ocm_flags & OCMFLAG_CONST)
	{
	    typval_T *mtv = ((typval_T *)(obj + 1)) + i;
	    item_lock(mtv, DICT_MAXNEST, TRUE, TRUE);
	}
    }
}

/*
 * Make a copy of an object.
 */
    void
copy_object(typval_T *from, typval_T *to)
{
    if (from->vval.v_object == NULL)
	to->vval.v_object = NULL;
    else
    {
	to->vval.v_object = from->vval.v_object;
	++to->vval.v_object->obj_refcount;
    }
}

/*
 * Make a copy of a class.
 */
    void
copy_class(typval_T *from, typval_T *to)
{
    if (from->vval.v_class == NULL)
	to->vval.v_class = NULL;
    else
    {
	to->vval.v_class = from->vval.v_class;
	++to->vval.v_class->class_refcount;
    }
}

/*
 * Free the class "cl" and its contents.
 */
    static void
class_free(class_T *cl)
{
    // Freeing what the class contains may recursively come back here.
    // Clear "class_name" first, if it is NULL the class does not need to
    // be freed.
    VIM_CLEAR(cl->class_name);

    class_unref(cl->class_extends);

    for (int i = 0; i < cl->class_interface_count; ++i)
    {
	vim_free(((char_u **)cl->class_interfaces)[i]);
	if (cl->class_interfaces_cl[i] != NULL)
	    class_unref(cl->class_interfaces_cl[i]);
    }
    vim_free(cl->class_interfaces);
    vim_free(cl->class_interfaces_cl);

    itf2class_T *next;
    for (itf2class_T *i2c = cl->class_itf2class; i2c != NULL; i2c = next)
    {
	next = i2c->i2c_next;
	vim_free(i2c);
    }

    for (int i = 0; i < cl->class_class_member_count; ++i)
    {
	ocmember_T *m = &cl->class_class_members[i];
	vim_free(m->ocm_name);
	vim_free(m->ocm_init);
	if (cl->class_members_tv != NULL)
	    clear_tv(&cl->class_members_tv[i]);
    }
    vim_free(cl->class_class_members);
    vim_free(cl->class_members_tv);

    for (int i = 0; i < cl->class_obj_member_count; ++i)
    {
	ocmember_T *m = &cl->class_obj_members[i];
	vim_free(m->ocm_name);
	vim_free(m->ocm_init);
    }
    vim_free(cl->class_obj_members);

    for (int i = 0; i < cl->class_class_function_count; ++i)
    {
	ufunc_T *uf = cl->class_class_functions[i];
	func_clear_free(uf, FALSE);
    }
    vim_free(cl->class_class_functions);

    for (int i = 0; i < cl->class_obj_method_count; ++i)
    {
	ufunc_T *uf = cl->class_obj_methods[i];
	func_clear_free(uf, FALSE);
    }
    vim_free(cl->class_obj_methods);

    clear_type_list(&cl->class_type_list);

    class_cleared(cl);

    vim_free(cl);
}

/*
 * Unreference a class.  Free it when the reference count goes down to zero.
 */
    void
class_unref(class_T *cl)
{
    if (cl != NULL && --cl->class_refcount <= 0 && cl->class_name != NULL)
	class_free(cl);
}

/*
 * Go through the list of all classes and free items without "copyID".
 */
    int
class_free_nonref(int copyID)
{
    int		did_free = FALSE;

    for (class_T *cl = first_class; cl != NULL; cl = next_nonref_class)
    {
	next_nonref_class = cl->class_next_used;
	if ((cl->class_copyID & COPYID_MASK) != (copyID & COPYID_MASK))
	{
	    // Free the class and items it contains.
	    class_free(cl);
	    did_free = TRUE;
	}
    }

    next_nonref_class = NULL;
    return did_free;
}

    int
set_ref_in_classes(int copyID)
{
    for (class_T *cl = first_class; cl != NULL; cl = cl->class_next_used)
	set_ref_in_item_class(cl, copyID, NULL, NULL);

    return FALSE;
}

static object_T *first_object = NULL;

/*
 * Call this function when an object has been created.  It will be added to the
 * list headed by "first_object".
 */
    void
object_created(object_T *obj)
{
    if (first_object != NULL)
    {
	obj->obj_next_used = first_object;
	first_object->obj_prev_used = obj;
    }
    first_object = obj;
}

/*
 * Call this function when an object has been cleared and is about to be freed.
 * It is removed from the list headed by "first_object".
 */
    static void
object_cleared(object_T *obj)
{
    if (obj->obj_next_used != NULL)
	obj->obj_next_used->obj_prev_used = obj->obj_prev_used;
    if (obj->obj_prev_used != NULL)
	obj->obj_prev_used->obj_next_used = obj->obj_next_used;
    else if (first_object == obj)
	first_object = obj->obj_next_used;
}

/*
 * Free the contents of an object ignoring the reference count.
 */
    static void
object_free_contents(object_T *obj)
{
    class_T *cl = obj->obj_class;

    if (!cl)
	return;

    // Avoid a recursive call, it can happen if "obj" has a circular reference.
    obj->obj_refcount = INT_MAX;

    // the member values are just after the object structure
    typval_T *tv = (typval_T *)(obj + 1);
    for (int i = 0; i < cl->class_obj_member_count; ++i)
	clear_tv(tv + i);
}

    static void
object_free_object(object_T *obj)
{
    class_T *cl = obj->obj_class;

    if (!cl)
	return;

    // Remove from the list headed by "first_object".
    object_cleared(obj);

    vim_free(obj);
    class_unref(cl);
}

    static void
object_free(object_T *obj)
{
    if (in_free_unref_items)
	return;

    object_free_contents(obj);
    object_free_object(obj);
}

/*
 * Unreference an object.
 */
    void
object_unref(object_T *obj)
{
    if (obj != NULL && --obj->obj_refcount <= 0)
	object_free(obj);
}

/*
 * Go through the list of all objects and free items without "copyID".
 */
    int
object_free_nonref(int copyID)
{
    int		did_free = FALSE;

    for (object_T *obj = first_object; obj != NULL; obj = obj->obj_next_used)
    {
	if ((obj->obj_copyID & COPYID_MASK) != (copyID & COPYID_MASK))
	{
	    // Free the object contents.  Object itself will be freed later.
	    object_free_contents(obj);
	    did_free = TRUE;
	}
    }

    return did_free;
}

    void
object_free_items(int copyID)
{
    object_T	*obj_next;

    for (object_T *obj = first_object; obj != NULL; obj = obj_next)
    {
	obj_next = obj->obj_next_used;
	if ((obj->obj_copyID & COPYID_MASK) != (copyID & COPYID_MASK))
	    object_free_object(obj);
    }
}

/*
 * Output message which takes a variable name and the class that defines it.
 * "cl" is that class where the name was found. Search "cl"'s hierarchy to
 * find the defining class.
 */
    void
emsg_var_cl_define(char *msg, char_u *name, size_t len, class_T *cl)
{
    ocmember_T	*m;
    class_T	*cl_def = class_defining_member(cl, name, len, &m);
    if (cl_def != NULL)
	semsg(_(msg), m->ocm_name, cl_def->class_name);
    else
	emsg(_(e_internal_error_please_report_a_bug));
}

/*
 * Echo a class or object method not found message.
 */
    void
method_not_found_msg(class_T *cl, vartype_T v_type, char_u *name, size_t len)
{
    char_u *method_name = vim_strnsave(name, len);
    if ((v_type == VAR_OBJECT)
	    && (class_method_idx(cl, name, len) >= 0))
    {
	// If this is a class method, then give a different error
	if (*name == '_')
	    semsg(_(e_cannot_access_protected_method_str), method_name);
	else
	    semsg(_(e_class_method_str_accessible_only_using_class_str),
		    method_name, cl->class_name);
    }
    else if ((v_type == VAR_CLASS)
	    && (object_method_idx(cl, name, len) >= 0))
    {
	// If this is an object method, then give a different error
	if (*name == '_')
	    semsg(_(e_cannot_access_protected_method_str), method_name);
	else
	    semsg(_(e_object_method_str_accessible_only_using_object_str),
		    method_name, cl->class_name);
    }
    else
	semsg(_(e_method_not_found_on_class_str_str), method_name,
	        cl->class_name);
    vim_free(method_name);
}

/*
 * Echo a class or object member not found message.
 */
    void
member_not_found_msg(class_T *cl, vartype_T v_type, char_u *name, size_t len)
{
    char_u *varname = len ? vim_strnsave(name, len) : vim_strsave(name);

    if (v_type == VAR_OBJECT)
    {
	if (class_member_idx(cl, name, len) >= 0)
	    semsg(_(e_class_variable_str_accessible_only_using_class_str),
		    varname, cl->class_name);
	else
	    semsg(_(e_variable_not_found_on_object_str_str), varname,
		    cl->class_name);
    }
    else
    {
	if (object_member_idx(cl, name, len) >= 0)
	    semsg(_(e_object_variable_str_accessible_only_using_object_str),
		    varname, cl->class_name);
	else
	    semsg(_(e_class_variable_str_not_found_in_class_str),
		    varname, cl->class_name);
    }
    vim_free(varname);
}

/*
 * Compile all the class and object methods in "cl".
 */
    void
defcompile_class(class_T *cl)
{
    for (int loop = 1; loop <= 2; ++loop)
    {
	int func_count = loop == 1 ? cl->class_class_function_count
						: cl->class_obj_method_count;
	for (int i = 0; i < func_count; i++)
	{
	    ufunc_T *ufunc = loop == 1 ? cl->class_class_functions[i]
						: cl->class_obj_methods[i];
	    defcompile_function(ufunc, cl);
	}
    }
}

/*
 * Compile all the classes defined in the current script
 */
    void
defcompile_classes_in_script(void)
{
    for (class_T *cl = first_class; cl != NULL; cl = cl->class_next_used)
    {
	if (eval_variable(cl->class_name, 0, 0, NULL, NULL,
			EVAL_VAR_NOAUTOLOAD | EVAL_VAR_NO_FUNC) != FAIL)
	    defcompile_class(cl);
    }
}

/*
 * Returns TRUE if "name" is the name of a class.  The typval for the class is
 * returned in "rettv".
 */
    int
is_class_name(char_u *name, typval_T *rettv)
{
    rettv->v_type = VAR_UNKNOWN;

    if (eval_variable(name, 0, 0, rettv, NULL, EVAL_VAR_NOAUTOLOAD |
						EVAL_VAR_NO_FUNC) != FAIL)
	return rettv->v_type == VAR_CLASS;
    return FALSE;
}

/*
 * Return TRUE when the class "cl", its base class or one of the implemented
 * interfaces matches the class "other_cl".
 */
    int
class_instance_of(class_T *cl, class_T *other_cl)
{
    if (cl == other_cl)
	return TRUE;

    // Recursively check the base classes.
    for (; cl != NULL; cl = cl->class_extends)
    {
	if (cl == other_cl)
	    return TRUE;
	// Check the implemented interfaces and the super interfaces
	for (int i = cl->class_interface_count - 1; i >= 0; --i)
	{
	    class_T	*intf = cl->class_interfaces_cl[i];
	    while (intf != NULL)
	    {
		if (intf == other_cl)
		    return TRUE;
		// check the super interfaces
		intf = intf->class_extends;
	    }
	}
    }

    return FALSE;
}

/*
 * "instanceof(object, classinfo, ...)" function
 */
    void
f_instanceof(typval_T *argvars, typval_T *rettv)
{
    typval_T	*object_tv = &argvars[0];
    typval_T	*classinfo_tv = &argvars[1];
    class_T	*c;

    rettv->vval.v_number = VVAL_FALSE;

    if (check_for_object_arg(argvars, 0) == FAIL
	    || check_for_class_or_typealias_args(argvars, 1) == FAIL)
	return;

    if (object_tv->vval.v_object == NULL)
	return;

    for (; classinfo_tv->v_type != VAR_UNKNOWN; ++classinfo_tv)
    {
	if (classinfo_tv->v_type == VAR_TYPEALIAS)
	    c = classinfo_tv->vval.v_typealias->ta_type->tt_class;
	else
	    c = classinfo_tv->vval.v_class;

	if (class_instance_of(object_tv->vval.v_object->obj_class, c))
	{
	    rettv->vval.v_number = VVAL_TRUE;
	    return;
	}
    }
}

#endif // FEAT_EVAL
