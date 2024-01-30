/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * vim9execute.c: execute Vim9 script instructions
 */

#define USING_FLOAT_STUFF
#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

// When not generating protos this is included in proto.h
#ifdef PROTO
# include "vim9.h"
#endif


// Structure put on ec_trystack when ISN_TRY is encountered.
typedef struct {
    int	    tcd_frame_idx;	// ec_frame_idx at ISN_TRY
    int	    tcd_stack_len;	// size of ectx.ec_stack at ISN_TRY
    int	    tcd_in_catch;	// in catch or finally block
    int	    tcd_did_throw;	// set did_throw in :endtry
    int	    tcd_catch_idx;	// instruction of the first :catch or :finally
    int	    tcd_finally_idx;	// instruction of the :finally block or zero
    int	    tcd_endtry_idx;	// instruction of the :endtry
    int	    tcd_caught;		// catch block entered
    int	    tcd_cont;		// :continue encountered, jump here (minus one)
    int	    tcd_return;		// when TRUE return from end of :finally
} trycmd_T;

// Data local to a function.
// On a function call, if not empty, is saved on the stack and restored when
// returning.
typedef struct {
    int		floc_restore_cmdmod;
    cmdmod_T	floc_save_cmdmod;
    int		floc_restore_cmdmod_stacklen;
} funclocal_T;

// Structure to hold a reference to an outer_T, with information of whether it
// was allocated.
typedef struct {
    outer_T	*or_outer;
    partial_T	*or_partial;	// decrement "or_partial->pt_refcount" later
    int		or_outer_allocated;  // free "or_outer" later
} outer_ref_T;

// A stack is used to store:
// - arguments passed to a :def function
// - info about the calling function, to use when returning
// - local variables
// - temporary values
//
// In detail (FP == Frame Pointer):
//	  arg1		first argument from caller (if present)
//	  arg2		second argument from caller (if present)
//	  extra_arg1	any missing optional argument default value
// FP ->  cur_func	calling function
//	  current	previous instruction pointer
//	  frame_ptr	previous Frame Pointer
//	  var1		space for local variable
//	  var2		space for local variable
//	  ....		fixed space for max. number of local variables
//	  temp		temporary values
//	  ....		flexible space for temporary values (can grow big)

/*
 * Execution context.
 */
struct ectx_S {
    garray_T	ec_stack;	// stack of typval_T values
    int		ec_frame_idx;	// index in ec_stack: context of ec_dfunc_idx
    int		ec_initial_frame_idx;	// frame index when called

    outer_ref_T	*ec_outer_ref;	// outer scope used for closures, allocated
    funclocal_T ec_funclocal;

    garray_T	ec_trystack;	// stack of trycmd_T values

    isn_T	*ec_instr;	// array with instructions
    int		ec_dfunc_idx;	// current function index
    int		ec_iidx;	// index in ec_instr: instruction to execute

    garray_T	ec_funcrefs;	// partials that might be a closure

    int		ec_did_emsg_before;
    int		ec_trylevel_at_start;
    where_T	ec_where;
};

#ifdef FEAT_PROFILE
// stack of profinfo_T used when profiling.
static garray_T profile_info_ga = {0, 0, sizeof(profinfo_T), 20, NULL};
#endif

// Get pointer to item in the stack.
#define STACK_TV(idx) (((typval_T *)ectx->ec_stack.ga_data) + idx)

// Get pointer to item relative to the bottom of the stack, -1 is the last one.
#define STACK_TV_BOT(idx) (((typval_T *)ectx->ec_stack.ga_data) + ectx->ec_stack.ga_len + (idx))

// Get pointer to a local variable on the stack.  Negative for arguments.
#define STACK_TV_VAR(idx) (((typval_T *)ectx->ec_stack.ga_data) + ectx->ec_frame_idx + STACK_FRAME_SIZE + idx)

    void
to_string_error(vartype_T vartype)
{
    semsg(_(e_cannot_convert_str_to_string), vartype_name(vartype));
}

/*
 * Return the number of arguments, including optional arguments and any vararg.
 */
    static int
ufunc_argcount(ufunc_T *ufunc)
{
    return ufunc->uf_args.ga_len + (ufunc->uf_va_name != NULL ? 1 : 0);
}

/*
 * Create a new string from "count" items at the bottom of the stack.
 * A trailing NUL is appended.
 * When "count" is zero an empty string is added to the stack.
 */
    static int
exe_concat(int count, ectx_T *ectx)
{
    int		idx;
    int		len = 0;
    typval_T	*tv;
    garray_T	ga;

    ga_init2(&ga, sizeof(char), 1);
    // Preallocate enough space for the whole string to avoid having to grow
    // and copy.
    for (idx = 0; idx < count; ++idx)
    {
	tv = STACK_TV_BOT(idx - count);
	if (tv->vval.v_string != NULL)
	    len += (int)STRLEN(tv->vval.v_string);
    }

    if (ga_grow(&ga, len + 1) == FAIL)
	return FAIL;

    for (idx = 0; idx < count; ++idx)
    {
	tv = STACK_TV_BOT(idx - count);
	ga_concat(&ga, tv->vval.v_string);
	clear_tv(tv);
    }

    // add a terminating NUL
    ga_append(&ga, NUL);

    ectx->ec_stack.ga_len -= count - 1;
    STACK_TV_BOT(-1)->vval.v_string = ga.ga_data;

    return OK;
}

/*
 * Create a new list from "count" items at the bottom of the stack.
 * When "count" is zero an empty list is added to the stack.
 * When "count" is -1 a NULL list is added to the stack.
 */
    static int
exe_newlist(int count, ectx_T *ectx)
{
    list_T	*list = NULL;
    int		idx;
    typval_T	*tv;

    if (count >= 0)
    {
	list = list_alloc_with_items(count);
	if (list == NULL)
	    return FAIL;
	for (idx = 0; idx < count; ++idx)
	    list_set_item(list, idx, STACK_TV_BOT(idx - count));
    }

    if (count > 0)
	ectx->ec_stack.ga_len -= count - 1;
    else if (GA_GROW_FAILS(&ectx->ec_stack, 1))
    {
	list_unref(list);
	return FAIL;
    }
    else
	++ectx->ec_stack.ga_len;
    tv = STACK_TV_BOT(-1);
    tv->v_type = VAR_LIST;
    tv->vval.v_list = list;
    tv->v_lock = 0;
    if (list != NULL)
	++list->lv_refcount;
    return OK;
}

/*
 * Implementation of ISN_NEWDICT.
 * Returns FAIL on total failure, MAYBE on error.
 */
    static int
exe_newdict(int count, ectx_T *ectx)
{
    dict_T	*dict = NULL;
    dictitem_T	*item;
    char_u	*key;
    int		idx;
    typval_T	*tv;

    if (count >= 0)
    {
	dict = dict_alloc();
	if (unlikely(dict == NULL))
	    return FAIL;
	for (idx = 0; idx < count; ++idx)
	{
	    // have already checked key type is VAR_STRING
	    tv = STACK_TV_BOT(2 * (idx - count));
	    // check key is unique
	    key = tv->vval.v_string == NULL ? (char_u *)"" : tv->vval.v_string;
	    item = dict_find(dict, key, -1);
	    if (item != NULL)
	    {
		semsg(_(e_duplicate_key_in_dictionary_str), key);
		dict_unref(dict);
		return MAYBE;
	    }
	    item = dictitem_alloc(key);
	    clear_tv(tv);
	    if (unlikely(item == NULL))
	    {
		dict_unref(dict);
		return FAIL;
	    }
	    tv = STACK_TV_BOT(2 * (idx - count) + 1);
	    item->di_tv = *tv;
	    item->di_tv.v_lock = 0;
	    tv->v_type = VAR_UNKNOWN;
	    if (dict_add(dict, item) == FAIL)
	    {
		// can this ever happen?
		dict_unref(dict);
		return FAIL;
	    }
	}
    }

    if (count > 0)
	ectx->ec_stack.ga_len -= 2 * count - 1;
    else if (GA_GROW_FAILS(&ectx->ec_stack, 1))
	return FAIL;
    else
	++ectx->ec_stack.ga_len;
    tv = STACK_TV_BOT(-1);
    tv->v_type = VAR_DICT;
    tv->v_lock = 0;
    tv->vval.v_dict = dict;
    if (dict != NULL)
	++dict->dv_refcount;
    return OK;
}

/*
 * If debug_tick changed check if "ufunc" has a breakpoint and update
 * "uf_has_breakpoint".
 */
    void
update_has_breakpoint(ufunc_T *ufunc)
{
    if (ufunc->uf_debug_tick == debug_tick)
	return;

    linenr_T breakpoint;

    ufunc->uf_debug_tick = debug_tick;
    breakpoint = dbg_find_breakpoint(FALSE, ufunc->uf_name, 0);
    ufunc->uf_has_breakpoint = breakpoint > 0;
}

static garray_T dict_stack = GA_EMPTY;

/*
 * Put a value on the dict stack.  This consumes "tv".
 */
    static int
dict_stack_save(typval_T *tv)
{
    if (dict_stack.ga_growsize == 0)
	ga_init2(&dict_stack, sizeof(typval_T), 10);
    if (ga_grow(&dict_stack, 1) == FAIL)
	return FAIL;
    ((typval_T *)dict_stack.ga_data)[dict_stack.ga_len] = *tv;
    ++dict_stack.ga_len;
    return OK;
}

/*
 * Get the typval at top of the dict stack.
 */
    static typval_T *
dict_stack_get_tv(void)
{
    if (dict_stack.ga_len == 0)
	return NULL;
    return ((typval_T *)dict_stack.ga_data) + dict_stack.ga_len - 1;
}

/*
 * Get the dict at top of the dict stack.
 */
    static dict_T *
dict_stack_get_dict(void)
{
    typval_T *tv;

    if (dict_stack.ga_len == 0)
	return NULL;
    tv = ((typval_T *)dict_stack.ga_data) + dict_stack.ga_len - 1;
    if (tv->v_type == VAR_DICT)
	return tv->vval.v_dict;
    return NULL;
}

/*
 * Drop an item from the dict stack.
 */
    static void
dict_stack_drop(void)
{
    if (dict_stack.ga_len == 0)
    {
	iemsg("Dict stack underflow");
	return;
    }
    --dict_stack.ga_len;
    clear_tv(((typval_T *)dict_stack.ga_data) + dict_stack.ga_len);
}

/*
 * Drop items from the dict stack until the length is equal to "len".
 */
    static void
dict_stack_clear(int len)
{
    while (dict_stack.ga_len > len)
	dict_stack_drop();
}

/*
 * Get a pointer to useful "pt_outer" of "pt".
 */
    static outer_T *
get_pt_outer(partial_T *pt)
{
    partial_T *ptref = pt->pt_outer_partial;

    if (ptref == NULL)
	return &pt->pt_outer;

    // partial using partial (recursively)
    while (ptref->pt_outer_partial != NULL)
	ptref = ptref->pt_outer_partial;
    return &ptref->pt_outer;
}

/*
 * Check "argcount" arguments on the stack against what "ufunc" expects.
 * "off" is the offset of arguments on the stack.
 * Return OK or FAIL.
 */
    static int
check_ufunc_arg_types(ufunc_T *ufunc, int argcount, int off, ectx_T *ectx)
{
    if (ufunc->uf_arg_types == NULL && ufunc->uf_va_type == NULL)
	return OK;

    typval_T	*argv = STACK_TV_BOT(0) - argcount - off;

    // The function can change at runtime, check that the argument
    // types are correct.
    for (int i = 0; i < argcount; ++i)
    {
	type_T *type = NULL;

	// assume a v:none argument, using the default value, is always OK
	if (argv[i].v_type == VAR_SPECIAL
		&& argv[i].vval.v_number == VVAL_NONE)
	    continue;
	// only pass values to user functions, never types
	if (check_typval_is_value(&argv[i]) == FAIL)
	    return FAIL;

	if (i < ufunc->uf_args.ga_len && ufunc->uf_arg_types != NULL)
	    type = ufunc->uf_arg_types[i];
	else if (ufunc->uf_va_type != NULL)
	    type = ufunc->uf_va_type->tt_member;
	if (type != NULL && check_typval_arg_type(type,
		    &argv[i], NULL, i + 1) == FAIL)
	    return FAIL;
    }
    return OK;
}

/*
 * Call compiled function "cdf_idx" from compiled code.
 * This adds a stack frame and sets the instruction pointer to the start of the
 * called function.
 * If "pt_arg" is not NULL use "pt_arg->pt_outer" for ec_outer_ref->or_outer.
 *
 * Stack has:
 * - current arguments (already there)
 * - omitted optional argument (default values) added here
 * - stack frame:
 *	- pointer to calling function
 *	- Index of next instruction in calling function
 *	- previous frame pointer
 * - reserved space for local variables
 */
    static int
call_dfunc(
	int		cdf_idx,
	partial_T	*pt_arg,
	int		argcount_arg,
	ectx_T		*ectx)
{
    int		argcount = argcount_arg;
    dfunc_T	*dfunc = ((dfunc_T *)def_functions.ga_data) + cdf_idx;
    ufunc_T	*ufunc = dfunc->df_ufunc;
    int		did_emsg_before = did_emsg_cumul + did_emsg;
    int		arg_to_add;
    int		vararg_count = 0;
    int		varcount;
    int		idx;
    estack_T	*entry;
    funclocal_T	*floc = NULL;
    int		res = OK;
    compiletype_T compile_type;

    if (dfunc->df_deleted)
    {
	// don't use ufunc->uf_name, it may have been freed
	emsg_funcname(e_function_was_deleted_str,
		dfunc->df_name == NULL ? (char_u *)"unknown" : dfunc->df_name);
	return FAIL;
    }

#ifdef FEAT_PROFILE
    if (do_profiling == PROF_YES)
    {
	if (GA_GROW_OK(&profile_info_ga, 1))
	{
	    profinfo_T *info = ((profinfo_T *)profile_info_ga.ga_data)
						      + profile_info_ga.ga_len;
	    ++profile_info_ga.ga_len;
	    CLEAR_POINTER(info);
	    profile_may_start_func(info, ufunc,
			(((dfunc_T *)def_functions.ga_data)
					      + ectx->ec_dfunc_idx)->df_ufunc);
	}
    }
#endif

    // When debugging and using "cont" switches to the not-debugged
    // instructions, may need to still compile them.
    compile_type = get_compile_type(ufunc);
    if (func_needs_compiling(ufunc, compile_type))
    {
	res = compile_def_function(ufunc, FALSE, compile_type, NULL);

	// compile_def_function() may cause def_functions.ga_data to change
	dfunc = ((dfunc_T *)def_functions.ga_data) + cdf_idx;
    }
    if (res == FAIL || INSTRUCTIONS(dfunc) == NULL)
    {
	if (did_emsg_cumul + did_emsg == did_emsg_before)
	    semsg(_(e_function_is_not_compiled_str),
						   printable_func_name(ufunc));
	return FAIL;
    }

    if (ufunc->uf_va_name != NULL)
    {
	// Need to make a list out of the vararg arguments.
	// Stack at time of call with 2 varargs:
	//   normal_arg
	//   optional_arg
	//   vararg_1
	//   vararg_2
	// After creating the list:
	//   normal_arg
	//   optional_arg
	//   vararg-list
	// With missing optional arguments we get:
	//    normal_arg
	// After creating the list
	//    normal_arg
	//    (space for optional_arg)
	//    vararg-list
	vararg_count = argcount - ufunc->uf_args.ga_len;
	if (vararg_count < 0)
	    vararg_count = 0;
	else
	    argcount -= vararg_count;
	if (exe_newlist(vararg_count, ectx) == FAIL)
	    return FAIL;

	vararg_count = 1;
    }

    arg_to_add = ufunc->uf_args.ga_len - argcount;
    if (arg_to_add < 0)
    {
	semsg(NGETTEXT(e_one_argument_too_many, e_nr_arguments_too_many,
			-arg_to_add), -arg_to_add);
	return FAIL;
    }
    else if (arg_to_add > ufunc->uf_def_args.ga_len)
    {
	int missing = arg_to_add - ufunc->uf_def_args.ga_len;

	semsg(NGETTEXT(e_one_argument_too_few, e_nr_arguments_too_few,
			missing), missing);
	return FAIL;
    }

    // If this is an object method, the object is just before the arguments.
    typval_T	*obj = STACK_TV_BOT(0) - argcount - vararg_count - 1;

    if (IS_OBJECT_METHOD(ufunc) && !IS_CONSTRUCTOR_METHOD(ufunc)
	    && obj->v_type == VAR_OBJECT && obj->vval.v_object == NULL)
    {
	// If this is not a constructor method, then a valid object is
	// needed.
	emsg(_(e_using_null_object));
	return FAIL;
    }

    // Check the argument types.
    if (check_ufunc_arg_types(ufunc, argcount, vararg_count, ectx) == FAIL)
	return FAIL;

    // While check_ufunc_arg_types call, def function compilation process may
    // run.  If so many def functions are compiled, def_functions array may be
    // reallocated and dfunc may no longer have valid pointer.  Get the object
    // pointer from def_functions again here.
    dfunc = ((dfunc_T *)def_functions.ga_data) + cdf_idx;

    // Reserve space for:
    // - missing arguments
    // - stack frame
    // - local variables
    // - if needed: a counter for number of closures created in
    //   ectx->ec_funcrefs.
    varcount = dfunc->df_varcount + dfunc->df_has_closure;
    if (GA_GROW_FAILS(&ectx->ec_stack,
				     arg_to_add + STACK_FRAME_SIZE + varcount))
	return FAIL;

    // The object pointer is in the execution typval stack.  The GA_GROW call
    // above may have reallocated the execution typval stack.  So the object
    // pointer may not be valid anymore.  Get the object pointer again from the
    // execution stack.
    obj = STACK_TV_BOT(0) - argcount - vararg_count - 1;

    // If depth of calling is getting too high, don't execute the function.
    if (funcdepth_increment() == FAIL)
	return FAIL;
    ++ex_nesting_level;

    // Only make a copy of funclocal if it contains something to restore.
    if (ectx->ec_funclocal.floc_restore_cmdmod)
    {
	floc = ALLOC_ONE(funclocal_T);
	if (floc == NULL)
	    return FAIL;
	*floc = ectx->ec_funclocal;
	ectx->ec_funclocal.floc_restore_cmdmod = FALSE;
    }

    // Move the vararg-list to below the missing optional arguments.
    if (vararg_count > 0 && arg_to_add > 0)
	*STACK_TV_BOT(arg_to_add - 1) = *STACK_TV_BOT(-1);

    // Reserve space for omitted optional arguments, filled in soon.
    for (idx = 0; idx < arg_to_add; ++idx)
	STACK_TV_BOT(idx - vararg_count)->v_type = VAR_UNKNOWN;
    ectx->ec_stack.ga_len += arg_to_add;

    // Store current execution state in stack frame for ISN_RETURN.
    STACK_TV_BOT(STACK_FRAME_FUNC_OFF)->vval.v_number = ectx->ec_dfunc_idx;
    STACK_TV_BOT(STACK_FRAME_IIDX_OFF)->vval.v_number = ectx->ec_iidx;
    STACK_TV_BOT(STACK_FRAME_INSTR_OFF)->vval.v_string = (void *)ectx->ec_instr;
    STACK_TV_BOT(STACK_FRAME_OUTER_OFF)->vval.v_string =
						    (void *)ectx->ec_outer_ref;
    STACK_TV_BOT(STACK_FRAME_FUNCLOCAL_OFF)->vval.v_string = (void *)floc;
    STACK_TV_BOT(STACK_FRAME_IDX_OFF)->vval.v_number = ectx->ec_frame_idx;
    ectx->ec_frame_idx = ectx->ec_stack.ga_len;

    // Initialize all local variables to number zero.  Also initialize the
    // variable that counts how many closures were created.  This is used in
    // handle_closure_in_use().
    int initcount = dfunc->df_varcount + (dfunc->df_has_closure ? 1 : 0);
    for (idx = 0; idx < initcount; ++idx)
    {
	typval_T *tv = STACK_TV_BOT(STACK_FRAME_SIZE + idx);

	tv->v_type = VAR_NUMBER;
	tv->vval.v_number = 0;
    }
    ectx->ec_stack.ga_len += STACK_FRAME_SIZE + varcount;

    // For an object method move the object from just before the arguments to
    // the first local variable.
    if (IS_OBJECT_METHOD(ufunc))
    {
	if (obj->v_type != VAR_OBJECT)
	{
	    semsg(_(e_internal_error_str), "type in stack is not an object");
	    return FAIL;
	}

	*STACK_TV_VAR(0) = *obj;
	obj->v_type = VAR_UNKNOWN;
    }

    partial_T *pt = pt_arg != NULL ? pt_arg : ufunc->uf_partial;
    if (pt != NULL || (ufunc->uf_flags & FC_CLOSURE))
    {
	outer_ref_T *ref = ALLOC_CLEAR_ONE(outer_ref_T);

	if (ref == NULL)
	    return FAIL;
	if (pt != NULL)
	{
	    ref->or_outer = get_pt_outer(pt);
	    ++pt->pt_refcount;
	    ref->or_partial = pt;
	}
	else
	{
	    ref->or_outer = ALLOC_CLEAR_ONE(outer_T);
	    if (unlikely(ref->or_outer == NULL))
	    {
		vim_free(ref);
		return FAIL;
	    }
	    ref->or_outer_allocated = TRUE;
	    ref->or_outer->out_stack = &ectx->ec_stack;
	    ref->or_outer->out_frame_idx = ectx->ec_frame_idx;
	    if (ectx->ec_outer_ref != NULL)
		ref->or_outer->out_up = ectx->ec_outer_ref->or_outer;
	}
	ectx->ec_outer_ref = ref;
    }
    else
	ectx->ec_outer_ref = NULL;

    ++ufunc->uf_calls;

    // Set execution state to the start of the called function.
    ectx->ec_dfunc_idx = cdf_idx;
    ectx->ec_instr = INSTRUCTIONS(dfunc);
    entry = estack_push_ufunc(ufunc, 1);
    if (entry != NULL)
    {
	// Set the script context to the script where the function was defined.
	// Save the current context so it can be restored on return.
	entry->es_save_sctx = current_sctx;
	current_sctx = ufunc->uf_script_ctx;
    }

    // Start execution at the first instruction.
    ectx->ec_iidx = 0;

    return OK;
}

// Double linked list of funcstack_T in use.
static funcstack_T *first_funcstack = NULL;

    static void
add_funcstack_to_list(funcstack_T *funcstack)
{
	// Link in list of funcstacks.
    if (first_funcstack != NULL)
	first_funcstack->fs_prev = funcstack;
    funcstack->fs_next = first_funcstack;
    funcstack->fs_prev = NULL;
    first_funcstack = funcstack;
}

    static void
remove_funcstack_from_list(funcstack_T *funcstack)
{
    if (funcstack->fs_prev == NULL)
	first_funcstack = funcstack->fs_next;
    else
	funcstack->fs_prev->fs_next = funcstack->fs_next;
    if (funcstack->fs_next != NULL)
	funcstack->fs_next->fs_prev = funcstack->fs_prev;
}

/*
 * Used when returning from a function: Check if any closure is still
 * referenced.  If so then move the arguments and variables to a separate piece
 * of stack to be used when the closure is called.
 * When "free_arguments" is TRUE the arguments are to be freed.
 * Returns FAIL when out of memory.
 */
    static int
handle_closure_in_use(ectx_T *ectx, int free_arguments)
{
    dfunc_T	*dfunc = ((dfunc_T *)def_functions.ga_data)
							  + ectx->ec_dfunc_idx;
    int		argcount;
    int		top;
    int		idx;
    typval_T	*tv;
    int		closure_in_use = FALSE;
    garray_T	*gap = &ectx->ec_funcrefs;
    varnumber_T	closure_count;

    if (dfunc->df_ufunc == NULL)
	return OK;  // function was freed
    if (dfunc->df_has_closure == 0)
	return OK;  // no closures
    tv = STACK_TV(ectx->ec_frame_idx + STACK_FRAME_SIZE + dfunc->df_varcount);
    closure_count = tv->vval.v_number;
    if (closure_count == 0)
	return OK;  // no funcrefs created

    // Compute "top": the first entry in the stack used by the function.
    // This is the first argument (after that comes the stack frame and then
    // the local variables).
    argcount = ufunc_argcount(dfunc->df_ufunc);
    top = ectx->ec_frame_idx - argcount;

    // Check if any created closure is still in use.
    for (idx = 0; idx < closure_count; ++idx)
    {
	partial_T   *pt;
	int	    off = gap->ga_len - closure_count + idx;

	if (off < 0)
	    continue;  // count is off or already done
	pt = ((partial_T **)gap->ga_data)[off];
	if (pt->pt_refcount > 1)
	{
	    int refcount = pt->pt_refcount;
	    int i;

	    // A Reference in a local variable doesn't count, it gets
	    // unreferenced on return.
	    for (i = 0; i < dfunc->df_varcount; ++i)
	    {
		typval_T *stv = STACK_TV(ectx->ec_frame_idx
						       + STACK_FRAME_SIZE + i);
		if (stv->v_type == VAR_PARTIAL && pt == stv->vval.v_partial)
		    --refcount;
	    }
	    if (refcount > 1)
	    {
		closure_in_use = TRUE;
		break;
	    }
	}
    }

    if (closure_in_use)
    {
	funcstack_T *funcstack = ALLOC_CLEAR_ONE(funcstack_T);
	typval_T    *stack;

	// A closure is using the arguments and/or local variables.
	// Move them to the called function.
	if (funcstack == NULL)
	    return FAIL;

	funcstack->fs_var_offset = argcount + STACK_FRAME_SIZE;
	funcstack->fs_ga.ga_len = funcstack->fs_var_offset
							  + dfunc->df_varcount;
	stack = ALLOC_CLEAR_MULT(typval_T, funcstack->fs_ga.ga_len);
	funcstack->fs_ga.ga_data = stack;
	if (stack == NULL)
	{
	    vim_free(funcstack);
	    return FAIL;
	}
	add_funcstack_to_list(funcstack);

	// Move or copy the arguments.
	for (idx = 0; idx < argcount; ++idx)
	{
	    tv = STACK_TV(top + idx);
	    if (free_arguments)
	    {
		*(stack + idx) = *tv;
		tv->v_type = VAR_UNKNOWN;
	    }
	    else
		copy_tv(tv, stack + idx);
	}
	// Skip the stack frame.
	// Move the local variables.
	for (idx = 0; idx < dfunc->df_varcount; ++idx)
	{
	    tv = STACK_TV(ectx->ec_frame_idx + STACK_FRAME_SIZE + idx);

	    // A partial created for a local function, that is also used as a
	    // local variable, has a reference count for the variable, thus
	    // will never go down to zero.  When all these refcounts are one
	    // then the funcstack is unused.  We need to count how many we have
	    // so we know when to check.
	    if (tv->v_type == VAR_PARTIAL && tv->vval.v_partial != NULL)
	    {
		int	    i;

		for (i = 0; i < closure_count; ++i)
		    if (tv->vval.v_partial == ((partial_T **)gap->ga_data)[
					      gap->ga_len - closure_count + i])
			++funcstack->fs_min_refcount;
	    }

	    *(stack + funcstack->fs_var_offset + idx) = *tv;
	    tv->v_type = VAR_UNKNOWN;
	}

	for (idx = 0; idx < closure_count; ++idx)
	{
	    partial_T *pt = ((partial_T **)gap->ga_data)[gap->ga_len
							- closure_count + idx];
	    if (pt->pt_refcount > 1)
	    {
		++funcstack->fs_refcount;
		pt->pt_funcstack = funcstack;
		pt->pt_outer.out_stack = &funcstack->fs_ga;
		pt->pt_outer.out_frame_idx = ectx->ec_frame_idx - top;
	    }
	}
    }

    for (idx = 0; idx < closure_count; ++idx)
	partial_unref(((partial_T **)gap->ga_data)[gap->ga_len
						       - closure_count + idx]);
    gap->ga_len -= closure_count;
    if (gap->ga_len == 0)
	ga_clear(gap);

    return OK;
}

/*
 * Called when a partial is freed or its reference count goes down to one.  The
 * funcstack may be the only reference to the partials in the local variables.
 * Go over all of them, the funcref and can be freed if all partials
 * referencing the funcstack have a reference count of one.
 * Returns TRUE if the funcstack is freed, the partial referencing it will then
 * also have been freed.
 */
    int
funcstack_check_refcount(funcstack_T *funcstack)
{
    int		i;
    garray_T	*gap = &funcstack->fs_ga;
    int		done = 0;
    typval_T	*stack;

    if (funcstack->fs_refcount > funcstack->fs_min_refcount)
	return FALSE;
    for (i = funcstack->fs_var_offset; i < gap->ga_len; ++i)
    {
	typval_T *tv = ((typval_T *)gap->ga_data) + i;

	if (tv->v_type == VAR_PARTIAL && tv->vval.v_partial != NULL
		&& tv->vval.v_partial->pt_funcstack == funcstack
		&& tv->vval.v_partial->pt_refcount == 1)
	    ++done;
    }
    if (done != funcstack->fs_min_refcount)
	return FALSE;

    stack = gap->ga_data;

    // All partials referencing the funcstack have a reference count of
    // one, thus the funcstack is no longer of use.
    for (i = 0; i < gap->ga_len; ++i)
	clear_tv(stack + i);
    vim_free(stack);
    remove_funcstack_from_list(funcstack);
    vim_free(funcstack);

    return TRUE;
}

/*
 * For garbage collecting: set references in all variables referenced by
 * all funcstacks.
 */
    int
set_ref_in_funcstacks(int copyID)
{
    funcstack_T *funcstack;

    for (funcstack = first_funcstack; funcstack != NULL;
						funcstack = funcstack->fs_next)
    {
	typval_T    *stack = funcstack->fs_ga.ga_data;
	int	    i;

	for (i = 0; i < funcstack->fs_ga.ga_len; ++i)
	    if (set_ref_in_item(stack + i, copyID, NULL, NULL))
		return TRUE;  // abort
    }
    return FALSE;
}

// Ugly static to avoid passing the execution context around through many
// layers.
static ectx_T *current_ectx = NULL;

/*
 * Return TRUE if currently executing a :def function.
 * Can be used by builtin functions only.
 */
    int
in_def_function(void)
{
    return current_ectx != NULL;
}

/*
 * If executing a class/object method, then fill in the lval_T.
 * Set lr_tv to the executing item, and lr_exec_class to the executing class;
 * use free_tv and class_unref when finished with the lval_root.
 * For use by builtin functions.
 *
 * Return FAIL and do nothing if not executing in a class; otherwise OK.
 */
    int
fill_exec_lval_root(lval_root_T *root)
{
    ectx_T *ectx = current_ectx;
    if (ectx != NULL)
    {
	dfunc_T	    *dfunc = ((dfunc_T *)def_functions.ga_data)
						  + current_ectx->ec_dfunc_idx;
	ufunc_T    *ufunc = dfunc->df_ufunc;
	if (ufunc->uf_class != NULL)	// executing a method?
	{
	    typval_T *tv = alloc_tv();
	    if (tv != NULL)
	    {
		CLEAR_POINTER(root);
		root->lr_tv = tv;
		copy_tv(STACK_TV_VAR(0), root->lr_tv);
		root->lr_cl_exec = ufunc->uf_class;
		++root->lr_cl_exec->class_refcount;
		return OK;
	    }
	}
    }

    return FAIL;
}

/*
 * Clear "current_ectx" and return the previous value.  To be used when calling
 * a user function.
 */
    ectx_T *
clear_current_ectx(void)
{
    ectx_T *r = current_ectx;

    current_ectx = NULL;
    return r;
}

    void
restore_current_ectx(ectx_T *ectx)
{
    if (current_ectx != NULL)
	iemsg("Restoring current_ectx while it is not NULL");
    current_ectx = ectx;
}

/*
 * Add an entry for a deferred function call to the currently executing
 * function.
 * Return the list or NULL when failed.
 */
    static list_T *
add_defer_item(int var_idx, int argcount, ectx_T *ectx)
{
    typval_T	*defer_tv = STACK_TV_VAR(var_idx);
    list_T	*defer_l;
    list_T	*l;
    typval_T	listval;

    if (defer_tv->v_type != VAR_LIST)
    {
	// first time, allocate the list
	if (rettv_list_alloc(defer_tv) == FAIL)
	    return NULL;
    }
    defer_l = defer_tv->vval.v_list;

    l = list_alloc_with_items(argcount + 1);
    if (l == NULL)
	return NULL;
    listval.v_type = VAR_LIST;
    listval.vval.v_list = l;
    listval.v_lock = 0;
    if (list_insert_tv(defer_l, &listval, defer_l->lv_first) == FAIL)
    {
	vim_free(l);
	return NULL;
    }

    return l;
}

/*
 * Handle ISN_DEFER.  Stack has a function reference and "argcount" arguments.
 * The local variable that lists deferred functions is "var_idx".
 * Returns OK or FAIL.
 */
    static int
defer_command(int var_idx, int argcount, ectx_T *ectx)
{
    list_T	*l = add_defer_item(var_idx, argcount, ectx);
    int		i;
    typval_T	*func_tv;

    if (l == NULL)
	return FAIL;

    func_tv = STACK_TV_BOT(-argcount - 1);
    if (func_tv->v_type != VAR_PARTIAL && func_tv->v_type != VAR_FUNC)
    {
	semsg(_(e_expected_str_but_got_str),
		"function or partial",
		vartype_name(func_tv->v_type));
	return FAIL;
    }
    list_set_item(l, 0, func_tv);

    for (i = 0; i < argcount; ++i)
	list_set_item(l, i + 1, STACK_TV_BOT(-argcount + i));
    ectx->ec_stack.ga_len -= argcount + 1;
    return OK;
}

/*
 * Add a deferred call for "name" with arguments "argvars[argcount]".
 * Consumes "name", also on failure.
 * Only to be called when in_def_function() returns TRUE.
 */
    int
add_defer_function(char_u *name, int argcount, typval_T *argvars)
{
    dfunc_T	*dfunc = ((dfunc_T *)def_functions.ga_data)
						  + current_ectx->ec_dfunc_idx;
    list_T	*l;
    typval_T	func_tv;
    int		i;

    if (dfunc->df_defer_var_idx == 0)
    {
	iemsg("df_defer_var_idx is zero");
	vim_free(name);
	return FAIL;
    }

    l = add_defer_item(dfunc->df_defer_var_idx - 1, argcount, current_ectx);
    if (l == NULL)
    {
	vim_free(name);
	return FAIL;
    }

    func_tv.v_type = VAR_FUNC;
    func_tv.v_lock = 0;
    func_tv.vval.v_string = name;
    list_set_item(l, 0, &func_tv);

    for (i = 0; i < argcount; ++i)
	list_set_item(l, i + 1, argvars + i);
    return OK;
}

/*
 * Invoked when returning from a function: Invoke any deferred calls.
 */
    static void
invoke_defer_funcs(ectx_T *ectx)
{
    dfunc_T	*dfunc = ((dfunc_T *)def_functions.ga_data)
							  + ectx->ec_dfunc_idx;
    typval_T	*defer_tv = STACK_TV_VAR(dfunc->df_defer_var_idx - 1);
    listitem_T	*li;

    if (defer_tv->v_type != VAR_LIST)
	return;	 // no function added
    FOR_ALL_LIST_ITEMS(defer_tv->vval.v_list, li)
    {
	list_T	    *l = li->li_tv.vval.v_list;
	typval_T    rettv;
	typval_T    argvars[MAX_FUNC_ARGS];
	int	    i;
	listitem_T  *arg_li = l->lv_first;
	typval_T    *functv = &l->lv_first->li_tv;
	int	    argcount = l->lv_len - 1;

	if (functv->vval.v_string == NULL)
	    // already being called, can happen if function does ":qa"
	    continue;

	for (i = 0; i < argcount; ++i)
	{
	    arg_li = arg_li->li_next;
	    argvars[i] = arg_li->li_tv;
	}

	funcexe_T   funcexe;
	CLEAR_FIELD(funcexe);
	funcexe.fe_evaluate = TRUE;
	rettv.v_type = VAR_UNKNOWN;
	if (functv->v_type == VAR_PARTIAL)
	{
	    funcexe.fe_partial = functv->vval.v_partial;
	    funcexe.fe_object = functv->vval.v_partial->pt_obj;
	    if (funcexe.fe_object != NULL)
		++funcexe.fe_object->obj_refcount;
	}

	char_u *name = functv->vval.v_string;
	functv->vval.v_string = NULL;

	// If the deferred function is called after an exception, then only the
	// first statement in the function will be executed (because of the
	// exception).  So save and restore the try/catch/throw exception
	// state.
	exception_state_T estate;
	exception_state_save(&estate);
	exception_state_clear();

	(void)call_func(name, -1, &rettv, argcount, argvars, &funcexe);

	exception_state_restore(&estate);

	clear_tv(&rettv);
	vim_free(name);
    }
}

/*
 * Return from the current function.
 */
    static int
func_return(ectx_T *ectx)
{
    int		idx;
    int		ret_idx;
    dfunc_T	*dfunc = ((dfunc_T *)def_functions.ga_data)
							  + ectx->ec_dfunc_idx;
    int		argcount = ufunc_argcount(dfunc->df_ufunc);
    estack_T	*entry;
    int		prev_dfunc_idx = STACK_TV(ectx->ec_frame_idx
					+ STACK_FRAME_FUNC_OFF)->vval.v_number;
    funclocal_T	*floc;
#ifdef FEAT_PROFILE
    dfunc_T	*prev_dfunc = ((dfunc_T *)def_functions.ga_data)
							      + prev_dfunc_idx;

    if (do_profiling == PROF_YES)
    {
	ufunc_T *caller = prev_dfunc->df_ufunc;

	if (dfunc->df_ufunc->uf_profiling
				   || (caller != NULL && caller->uf_profiling))
	{
	    profile_may_end_func(((profinfo_T *)profile_info_ga.ga_data)
			+ profile_info_ga.ga_len - 1, dfunc->df_ufunc, caller);
	    --profile_info_ga.ga_len;
	}
    }
#endif

    if (dfunc->df_defer_var_idx > 0)
	invoke_defer_funcs(ectx);

    // No check for uf_refcount being zero, cannot think of a way that would
    // happen.
    --dfunc->df_ufunc->uf_calls;

    // execution context goes one level up
    entry = estack_pop();
    if (entry != NULL)
	current_sctx = entry->es_save_sctx;

    if (handle_closure_in_use(ectx, TRUE) == FAIL)
	return FAIL;

    // Clear the arguments.  If this was an object method also clear the
    // object, it is just before the arguments.
    int top = ectx->ec_frame_idx - argcount;
    if (IS_OBJECT_METHOD(dfunc->df_ufunc))
	--top;
    for (idx = top; idx < ectx->ec_frame_idx; ++idx)
	clear_tv(STACK_TV(idx));

    // Clear local variables and temp values, but not the return value.
    for (idx = ectx->ec_frame_idx + STACK_FRAME_SIZE;
					idx < ectx->ec_stack.ga_len - 1; ++idx)
	clear_tv(STACK_TV(idx));

    // The return value should be on top of the stack.  However, when aborting
    // it may not be there and ec_frame_idx is the top of the stack.
    ret_idx = ectx->ec_stack.ga_len - 1;
    if (ret_idx == ectx->ec_frame_idx + STACK_FRAME_IDX_OFF)
	ret_idx = 0;

    if (ectx->ec_outer_ref != NULL)
    {
	if (ectx->ec_outer_ref->or_outer_allocated)
	    vim_free(ectx->ec_outer_ref->or_outer);
	partial_unref(ectx->ec_outer_ref->or_partial);
	vim_free(ectx->ec_outer_ref);
    }

    // Restore the previous frame.
    ectx->ec_dfunc_idx = prev_dfunc_idx;
    ectx->ec_iidx = STACK_TV(ectx->ec_frame_idx
					+ STACK_FRAME_IIDX_OFF)->vval.v_number;
    ectx->ec_instr = (void *)STACK_TV(ectx->ec_frame_idx
				       + STACK_FRAME_INSTR_OFF)->vval.v_string;
    ectx->ec_outer_ref = (void *)STACK_TV(ectx->ec_frame_idx
				       + STACK_FRAME_OUTER_OFF)->vval.v_string;
    floc = (void *)STACK_TV(ectx->ec_frame_idx
				   + STACK_FRAME_FUNCLOCAL_OFF)->vval.v_string;
    // restoring ec_frame_idx must be last
    ectx->ec_frame_idx = STACK_TV(ectx->ec_frame_idx
				       + STACK_FRAME_IDX_OFF)->vval.v_number;

    if (floc == NULL)
	ectx->ec_funclocal.floc_restore_cmdmod = FALSE;
    else
    {
	ectx->ec_funclocal = *floc;
	vim_free(floc);
    }

    if (ret_idx > 0)
    {
	// Reset the stack to the position before the call, with a spot for the
	// return value, moved there from above the frame.
	ectx->ec_stack.ga_len = top + 1;
	*STACK_TV_BOT(-1) = *STACK_TV(ret_idx);
    }
    else
	// Reset the stack to the position before the call.
	ectx->ec_stack.ga_len = top;

    funcdepth_decrement();
    --ex_nesting_level;
    return OK;
}

/*
 * Prepare arguments and rettv for calling a builtin or user function.
 */
    static int
call_prepare(int argcount, typval_T *argvars, ectx_T *ectx)
{
    int		idx;
    typval_T	*tv;

    // Move arguments from bottom of the stack to argvars[] and add terminator.
    for (idx = 0; idx < argcount; ++idx)
	argvars[idx] = *STACK_TV_BOT(idx - argcount);
    argvars[argcount].v_type = VAR_UNKNOWN;

    // Result replaces the arguments on the stack.
    if (argcount > 0)
	ectx->ec_stack.ga_len -= argcount - 1;
    else if (GA_GROW_FAILS(&ectx->ec_stack, 1))
	return FAIL;
    else
	++ectx->ec_stack.ga_len;

    // Default return value is zero.
    tv = STACK_TV_BOT(-1);
    tv->v_type = VAR_NUMBER;
    tv->vval.v_number = 0;
    tv->v_lock = 0;

    return OK;
}

/*
 * Call a builtin function by index.
 */
    static int
call_bfunc(int func_idx, int argcount, ectx_T *ectx)
{
    typval_T	argvars[MAX_FUNC_ARGS];
    int		idx;
    int		did_emsg_before = did_emsg;
    ectx_T	*prev_ectx = current_ectx;
    char	*save_func_name = ectx->ec_where.wt_func_name;

    if (call_prepare(argcount, argvars, ectx) == FAIL)
	return FAIL;
    ectx->ec_where.wt_func_name = internal_func_name(func_idx);

    // Call the builtin function.  Set "current_ectx" so that when it
    // recursively invokes call_def_function() a closure context can be set.
    current_ectx = ectx;
    call_internal_func_by_idx(func_idx, argvars, STACK_TV_BOT(-1));
    current_ectx = prev_ectx;
    ectx->ec_where.wt_func_name = save_func_name;

    // Clear the arguments.
    for (idx = 0; idx < argcount; ++idx)
	clear_tv(&argvars[idx]);

    if (did_emsg > did_emsg_before)
	return FAIL;
    return OK;
}

/*
 * Execute a user defined function.
 * If the function is compiled this will add a stack frame and set the
 * instruction pointer at the start of the function.
 * Otherwise the function is called here.
 * If "pt" is not null use "pt->pt_outer" for ec_outer_ref->or_outer.
 * "iptr" can be used to replace the instruction with a more efficient one.
 */
    static int
call_ufunc(
	ufunc_T	    *ufunc,
	partial_T   *pt,
	int	    argcount,
	ectx_T	    *ectx,
	isn_T	    *iptr,
	dict_T	    *selfdict)
{
    typval_T	argvars[MAX_FUNC_ARGS];
    funcexe_T   funcexe;
    funcerror_T	error;
    int		idx;
    int		did_emsg_before = did_emsg;
    compiletype_T compile_type = get_compile_type(ufunc);

    if (func_needs_compiling(ufunc, compile_type)
		&& compile_def_function(ufunc, FALSE, compile_type, NULL)
								       == FAIL)
	return FAIL;
    if (ufunc->uf_def_status == UF_COMPILED)
    {
	error = check_user_func_argcount(ufunc, argcount);
	if (error != FCERR_UNKNOWN)
	{
	    if (error == FCERR_TOOMANY)
		semsg(_(e_too_many_arguments_for_function_str),
						   printable_func_name(ufunc));
	    else
		semsg(_(e_not_enough_arguments_for_function_str),
						   printable_func_name(ufunc));
	    return FAIL;
	}

	// The function has been compiled, can call it quickly.  For a function
	// that was defined later: we can call it directly next time.
	if (iptr != NULL)
	{
	    delete_instr(iptr);
	    iptr->isn_type = ISN_DCALL;
	    iptr->isn_arg.dfunc.cdf_idx = ufunc->uf_dfunc_idx;
	    iptr->isn_arg.dfunc.cdf_argcount = argcount;
	}
	return call_dfunc(ufunc->uf_dfunc_idx, pt, argcount, ectx);
    }

    if (call_prepare(argcount, argvars, ectx) == FAIL)
	return FAIL;
    CLEAR_FIELD(funcexe);
    funcexe.fe_evaluate = TRUE;
    funcexe.fe_selfdict = selfdict != NULL ? selfdict : dict_stack_get_dict();

    // Call the user function.  Result goes in last position on the stack.
    error = call_user_func_check(ufunc, argcount, argvars,
			      STACK_TV_BOT(-1), &funcexe, funcexe.fe_selfdict);

    // Clear the arguments.
    for (idx = 0; idx < argcount; ++idx)
	clear_tv(&argvars[idx]);

    if (error != FCERR_NONE)
    {
	user_func_error(error, printable_func_name(ufunc),
							 funcexe.fe_found_var);
	return FAIL;
    }
    if (did_emsg > did_emsg_before)
	// Error other than from calling the function itself.
	return FAIL;
    return OK;
}

/*
 * If command modifiers were applied restore them.
 */
    static void
may_restore_cmdmod(funclocal_T *funclocal)
{
    if (funclocal->floc_restore_cmdmod)
    {
	cmdmod.cmod_filter_regmatch.regprog = NULL;
	undo_cmdmod(&cmdmod);
	cmdmod = funclocal->floc_save_cmdmod;
	funclocal->floc_restore_cmdmod = FALSE;
    }
}

/*
 * Return TRUE if an error was given (not caught in try/catch) or CTRL-C was
 * pressed.
 */
    static int
vim9_aborting(int prev_uncaught_emsg)
{
    return uncaught_emsg > prev_uncaught_emsg || got_int || did_throw;
}

/*
 * Execute a function by "name".
 * This can be a builtin function or a user function.
 * "iptr" can be used to replace the instruction with a more efficient one.
 * Returns FAIL if not found without an error message.
 */
    static int
call_by_name(
	char_u	    *name,
	int	    argcount,
	ectx_T	    *ectx,
	isn_T	    *iptr,
	dict_T	    *selfdict)
{
    ufunc_T *ufunc;

    if (builtin_function(name, -1))
    {
	int func_idx = find_internal_func(name);

	if (func_idx < 0)  // Impossible?
	    return FAIL;
	if (check_internal_func(func_idx, argcount) < 0)
	    return FAIL;
	return call_bfunc(func_idx, argcount, ectx);
    }

    ufunc = find_func(name, FALSE);

    if (ufunc == NULL)
    {
	int prev_uncaught_emsg = uncaught_emsg;

	if (script_autoload(name, TRUE))
	    // loaded a package, search for the function again
	    ufunc = find_func(name, FALSE);

	if (vim9_aborting(prev_uncaught_emsg))
	    return FAIL;  // bail out if loading the script caused an error
    }

    if (ufunc != NULL)
    {
	if (check_ufunc_arg_types(ufunc, argcount, 0, ectx) == FAIL)
	    return FAIL;

	return call_ufunc(ufunc, NULL, argcount, ectx, iptr, selfdict);
    }

    return FAIL;
}

    static int
call_partial(
	typval_T    *tv,
	int	    argcount_arg,
	ectx_T	    *ectx)
{
    int		argcount = argcount_arg;
    char_u	*name = NULL;
    int		called_emsg_before = called_emsg;
    int		res = FAIL;
    dict_T	*selfdict = NULL;

    if (tv->v_type == VAR_PARTIAL)
    {
	partial_T   *pt = tv->vval.v_partial;
	int	    i;

	if (pt->pt_obj != NULL)
	{
	    // partial with an object method.  Push the object before the
	    // function arguments.
	    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		return FAIL;
	    for (i = 1; i <= argcount; ++i)
		*STACK_TV_BOT(-i + 1) = *STACK_TV_BOT(-i);

	    typval_T *obj_tv = STACK_TV_BOT(-argcount);
	    obj_tv->v_type = VAR_OBJECT;
	    obj_tv->v_lock = 0;
	    obj_tv->vval.v_object = pt->pt_obj;
	    ++pt->pt_obj->obj_refcount;
	    ++ectx->ec_stack.ga_len;
	}

	if (pt->pt_argc > 0)
	{
	    // Make space for arguments from the partial, shift the "argcount"
	    // arguments up.
	    if (GA_GROW_FAILS(&ectx->ec_stack, pt->pt_argc))
		return FAIL;
	    for (i = 1; i <= argcount; ++i)
		*STACK_TV_BOT(-i + pt->pt_argc) = *STACK_TV_BOT(-i);
	    ectx->ec_stack.ga_len += pt->pt_argc;
	    argcount += pt->pt_argc;

	    // copy the arguments from the partial onto the stack
	    for (i = 0; i < pt->pt_argc; ++i)
		copy_tv(&pt->pt_argv[i], STACK_TV_BOT(-argcount + i));
	}
	selfdict = pt->pt_dict;

	if (pt->pt_func != NULL)
	    return call_ufunc(pt->pt_func, pt, argcount, ectx, NULL, selfdict);

	name = pt->pt_name;
    }
    else if (tv->v_type == VAR_FUNC)
	name = tv->vval.v_string;
    if (name != NULL)
    {
	char_u	    fname_buf[FLEN_FIXED + 1];
	char_u	    *tofree = NULL;
	funcerror_T error = FCERR_NONE;
	char_u	    *fname;

	// May need to translate <SNR>123_ to K_SNR.
	fname = fname_trans_sid(name, fname_buf, &tofree, &error);
	if (error != FCERR_NONE)
	    res = FAIL;
	else
	    res = call_by_name(fname, argcount, ectx, NULL, selfdict);
	vim_free(tofree);
    }

    if (res == FAIL)
    {
	if (called_emsg == called_emsg_before)
	    emsg_funcname(e_unknown_function_str,
				  name == NULL ? (char_u *)"[unknown]" : name);
	return FAIL;
    }
    return OK;
}

/*
 * Check if "lock" is VAR_LOCKED or VAR_FIXED.  If so give an error and return
 * TRUE.
 */
    static int
error_if_locked(int lock, char *error)
{
    if (lock & (VAR_LOCKED | VAR_FIXED))
    {
	emsg(_(error));
	return TRUE;
    }
    return FALSE;
}

/*
 * Give an error if "tv" is not a number and return FAIL.
 */
    static int
check_for_number(typval_T *tv)
{
    if (tv->v_type != VAR_NUMBER)
    {
	semsg(_(e_expected_str_but_got_str),
		vartype_name(VAR_NUMBER), vartype_name(tv->v_type));
	return FAIL;
    }
    return OK;
}

/*
 * Store "tv" in variable "name".
 * This is for s: and g: variables.
 */
    static void
store_var(char_u *name, typval_T *tv)
{
    funccal_entry_T entry;
    int		    flags = ASSIGN_DECL;

    if (tv->v_lock)
	flags |= ASSIGN_CONST;
    save_funccal(&entry);
    set_var_const(name, 0, NULL, tv, FALSE, flags, 0);
    restore_funccal();
}

/*
 * Convert "tv" to a string.
 * Return FAIL if not allowed.
 */
    static int
do_2string(typval_T *tv, int is_2string_any, int tolerant)
{
    if (tv->v_type == VAR_STRING)
	return OK;

    char_u *str;

    if (is_2string_any)
    {
	switch (tv->v_type)
	{
	    case VAR_SPECIAL:
	    case VAR_BOOL:
	    case VAR_NUMBER:
	    case VAR_FLOAT:
	    case VAR_BLOB:	break;

	    case VAR_LIST:
				if (tolerant)
				{
				    char_u	*s, *e, *p;
				    garray_T	ga;

				    ga_init2(&ga, sizeof(char_u *), 1);

				    // Convert to NL separated items, then
				    // escape the items and replace the NL with
				    // a space.
				    str = typval2string(tv, TRUE);
				    if (str == NULL)
					return FAIL;
				    s = str;
				    while ((e = vim_strchr(s, '\n')) != NULL)
				    {
					*e = NUL;
					p = vim_strsave_fnameescape(s,
						VSE_NONE);
					if (p != NULL)
					{
					    ga_concat(&ga, p);
					    ga_concat(&ga, (char_u *)" ");
					    vim_free(p);
					}
					s = e + 1;
				    }
				    vim_free(str);
				    clear_tv(tv);
				    tv->v_type = VAR_STRING;
				    tv->vval.v_string = ga.ga_data;
				    return OK;
				}
				// FALLTHROUGH
	    default:	to_string_error(tv->v_type);
			return FAIL;
	}
    }
    str = typval_tostring(tv, TRUE);
    clear_tv(tv);
    tv->v_type = VAR_STRING;
    tv->vval.v_string = str;
    return OK;
}

/*
 * When the value of "sv" is a null list of dict, allocate it.
 */
    static void
allocate_if_null(svar_T *sv)
{
    typval_T *tv = sv->sv_tv;

    switch (tv->v_type)
    {
	case VAR_LIST:
	    if (tv->vval.v_list == NULL && sv->sv_type != &t_list_empty)
		(void)rettv_list_alloc(tv);
	    break;
	case VAR_DICT:
	    if (tv->vval.v_dict == NULL && sv->sv_type != &t_dict_empty)
		(void)rettv_dict_alloc(tv);
	    break;
	case VAR_BLOB:
	    if (tv->vval.v_blob == NULL && sv->sv_type != &t_blob_null)
		(void)rettv_blob_alloc(tv);
	    break;
	default:
	    break;
    }
}

/*
 * Return the character "str[index]" where "index" is the character index,
 * including composing characters.
 * If "index" is out of range NULL is returned.
 */
    char_u *
char_from_string(char_u *str, varnumber_T index)
{
    size_t	    nbyte = 0;
    varnumber_T	    nchar = index;
    size_t	    slen;

    if (str == NULL)
	return NULL;
    slen = STRLEN(str);

    // Do the same as for a list: a negative index counts from the end.
    // Optimization to check the first byte to be below 0x80 (and no composing
    // character follows) makes this a lot faster.
    if (index < 0)
    {
	int	clen = 0;

	for (nbyte = 0; nbyte < slen; ++clen)
	{
	    if (str[nbyte] < 0x80 && str[nbyte + 1] < 0x80)
		++nbyte;
	    else if (enc_utf8)
		nbyte += utfc_ptr2len(str + nbyte);
	    else
		nbyte += mb_ptr2len(str + nbyte);
	}
	nchar = clen + index;
	if (nchar < 0)
	    // unlike list: index out of range results in empty string
	    return NULL;
    }

    for (nbyte = 0; nchar > 0 && nbyte < slen; --nchar)
    {
	if (str[nbyte] < 0x80 && str[nbyte + 1] < 0x80)
	    ++nbyte;
	else if (enc_utf8)
	    nbyte += utfc_ptr2len(str + nbyte);
	else
	    nbyte += mb_ptr2len(str + nbyte);
    }
    if (nbyte >= slen)
	return NULL;
    return vim_strnsave(str + nbyte, mb_ptr2len(str + nbyte));
}

/*
 * Get the byte index for character index "idx" in string "str" with length
 * "str_len".  Composing characters are included.
 * If going over the end return "str_len".
 * If "idx" is negative count from the end, -1 is the last character.
 * When going over the start return -1.
 */
    static long
char_idx2byte(char_u *str, size_t str_len, varnumber_T idx)
{
    varnumber_T nchar = idx;
    size_t	nbyte = 0;

    if (nchar >= 0)
    {
	while (nchar > 0 && nbyte < str_len)
	{
	    nbyte += mb_ptr2len(str + nbyte);
	    --nchar;
	}
    }
    else
    {
	nbyte = str_len;
	while (nchar < 0 && nbyte > 0)
	{
	    --nbyte;
	    nbyte -= mb_head_off(str, str + nbyte);
	    ++nchar;
	}
	if (nchar < 0)
	    return -1;
    }
    return (long)nbyte;
}

/*
 * Return the slice "str[first : last]" using character indexes.  Composing
 * characters are included.
 * "exclusive" is TRUE for slice().
 * Return NULL when the result is empty.
 */
    char_u *
string_slice(char_u *str, varnumber_T first, varnumber_T last, int exclusive)
{
    long	start_byte, end_byte;
    size_t	slen;

    if (str == NULL)
	return NULL;
    slen = STRLEN(str);
    start_byte = char_idx2byte(str, slen, first);
    if (start_byte < 0)
	start_byte = 0; // first index very negative: use zero
    if ((last == -1 && !exclusive) || last == VARNUM_MAX)
	end_byte = (long)slen;
    else
    {
	end_byte = char_idx2byte(str, slen, last);
	if (!exclusive && end_byte >= 0 && end_byte < (long)slen)
	    // end index is inclusive
	    end_byte += mb_ptr2len(str + end_byte);
    }

    if (start_byte >= (long)slen || end_byte <= start_byte)
	return NULL;
    return vim_strnsave(str + start_byte, end_byte - start_byte);
}

/*
 * Get a script variable for ISN_STORESCRIPT and ISN_LOADSCRIPT.
 * When "dfunc_idx" is negative don't give an error.
 * Returns NULL for an error.
 */
    static svar_T *
get_script_svar(scriptref_T *sref, int dfunc_idx)
{
    scriptitem_T    *si = SCRIPT_ITEM(sref->sref_sid);
    dfunc_T	    *dfunc = dfunc_idx < 0 ? NULL
			      : ((dfunc_T *)def_functions.ga_data) + dfunc_idx;
    svar_T	    *sv;

    if (sref->sref_seq != si->sn_script_seq)
    {
	// The script was reloaded after the function was compiled, the
	// script_idx may not be valid.
	if (dfunc != NULL)
	    semsg(_(e_script_variable_invalid_after_reload_in_function_str),
					 printable_func_name(dfunc->df_ufunc));
	return NULL;
    }
    sv = ((svar_T *)si->sn_var_vals.ga_data) + sref->sref_idx;
    if (sv->sv_name == NULL)
    {
	if (dfunc != NULL)
	    emsg(_(e_script_variable_was_deleted));
	return NULL;
    }
    if (!equal_type(sv->sv_type, sref->sref_type, 0))
    {
	if (dfunc != NULL)
	    emsg(_(e_script_variable_type_changed));
	return NULL;
    }

    if ((sv->sv_flags & SVFLAG_EXPORTED) == 0
				      && sref->sref_sid != current_sctx.sc_sid)
    {
	if (dfunc != NULL)
	    semsg(_(e_item_not_exported_in_script_str), sv->sv_name);
	return NULL;
    }
    return sv;
}

/*
 * Function passed to do_cmdline() for splitting a script joined by NL
 * characters.
 */
    static char_u *
get_split_sourceline(
	int c UNUSED,
	void *cookie,
	int indent UNUSED,
	getline_opt_T options UNUSED)
{
    source_cookie_T	*sp = (source_cookie_T *)cookie;
    char_u		*p;
    char_u		*line;

    p = vim_strchr(sp->nextline, '\n');
    if (p == NULL)
    {
	line = vim_strsave(sp->nextline);
	sp->nextline += STRLEN(sp->nextline);
    }
    else
    {
	line = vim_strnsave(sp->nextline, p - sp->nextline);
	sp->nextline = p + 1;
    }
    return line;
}

/*
 * Execute a function by "name".
 * This can be a builtin function, user function or a funcref.
 * "iptr" can be used to replace the instruction with a more efficient one.
 */
    static int
call_eval_func(
	char_u	    *name,
	int	    argcount,
	ectx_T	    *ectx,
	isn_T	    *iptr)
{
    int	    called_emsg_before = called_emsg;
    int	    res;

    res = call_by_name(name, argcount, ectx, iptr, NULL);
    if (res == FAIL && called_emsg == called_emsg_before)
    {
	dictitem_T	*v;

	v = find_var(name, NULL, FALSE);
	if (v == NULL || (v->di_tv.v_type != VAR_PARTIAL
					       && v->di_tv.v_type != VAR_FUNC))
	{
	    emsg_funcname(e_unknown_function_str, name);
	    return FAIL;
	}
	return call_partial(&v->di_tv, argcount, ectx);
    }
    return res;
}

/*
 * When a function reference is used, fill a partial with the information
 * needed, especially when it is used as a closure.
 */
    int
fill_partial_and_closure(
	partial_T	*pt,
	ufunc_T		*ufunc,
	loopvarinfo_T	*lvi,
	ectx_T		*ectx)
{
    pt->pt_func = ufunc;
    pt->pt_refcount = 1;

    if (ufunc->uf_flags & FC_CLOSURE)
    {
	dfunc_T	*dfunc = ((dfunc_T *)def_functions.ga_data)
							  + ectx->ec_dfunc_idx;

	// The closure may need to find arguments and local variables of the
	// current function in the stack.
	pt->pt_outer.out_stack = &ectx->ec_stack;
	pt->pt_outer.out_frame_idx = ectx->ec_frame_idx;
	if (ectx->ec_outer_ref != NULL)
	{
	    // The current context already has a context, link to that one.
	    pt->pt_outer.out_up = ectx->ec_outer_ref->or_outer;
	    if (ectx->ec_outer_ref->or_partial != NULL)
	    {
		pt->pt_outer.out_up_partial = ectx->ec_outer_ref->or_partial;
		++pt->pt_outer.out_up_partial->pt_refcount;
	    }
	}

	if (lvi != NULL)
	{
	    int	depth;

	    // The closure may need to find variables defined inside a loop,
	    // for every nested loop.  A new reference is made every time,
	    // ISN_ENDLOOP will check if they are actually used.
	    for (depth = 0; depth < lvi->lvi_depth; ++depth)
	    {
		pt->pt_outer.out_loop[depth].stack = &ectx->ec_stack;
		pt->pt_outer.out_loop[depth].var_idx = ectx->ec_frame_idx
			 + STACK_FRAME_SIZE + lvi->lvi_loop[depth].var_idx;
		pt->pt_outer.out_loop[depth].var_count =
					    lvi->lvi_loop[depth].var_count;
	    }
	    pt->pt_outer.out_loop_size = lvi->lvi_depth;
	}
	else
	    pt->pt_outer.out_loop_size = 0;

	// If the function currently executing returns and the closure is still
	// being referenced, we need to make a copy of the context (arguments
	// and local variables) so that the closure can use it later.
	// Store a reference to the partial so we can handle that.
	if (GA_GROW_FAILS(&ectx->ec_funcrefs, 1))
	{
	    vim_free(pt);
	    return FAIL;
	}
	// Extra variable keeps the count of closures created in the current
	// function call.
	++(((typval_T *)ectx->ec_stack.ga_data) + ectx->ec_frame_idx
		       + STACK_FRAME_SIZE + dfunc->df_varcount)->vval.v_number;

	((partial_T **)ectx->ec_funcrefs.ga_data)[ectx->ec_funcrefs.ga_len]
									  = pt;
	++pt->pt_refcount;
	++ectx->ec_funcrefs.ga_len;
    }
    ++ufunc->uf_refcount;
    return OK;
}

/*
 * Execute iptr->isn_arg.string as an Ex command.
 */
    static int
exec_command(isn_T *iptr, char_u *cmd_string)
{
    source_cookie_T cookie;

    SOURCING_LNUM = iptr->isn_lnum;
    // Pass getsourceline to get an error for a missing ":end" command.
    CLEAR_FIELD(cookie);
    cookie.sourcing_lnum = iptr->isn_lnum - 1;
    if (do_cmdline(cmd_string, getsourceline, &cookie,
			     DOCMD_VERBOSE|DOCMD_NOWAIT|DOCMD_KEYTYPED) == FAIL
		|| did_emsg)
	return FAIL;
    return OK;
}

/*
 * If script "sid" is not loaded yet then load it now.
 * Caller must make sure "sid" is a valid script ID.
 * "loaded" is set to TRUE if the script had to be loaded.
 * Returns FAIL if loading fails, OK if already loaded or loaded now.
 */
    int
may_load_script(int sid, int *loaded)
{
    scriptitem_T *si = SCRIPT_ITEM(sid);

    if (si->sn_state == SN_STATE_NOT_LOADED)
    {
	*loaded = TRUE;
	if (do_source(si->sn_name, FALSE, DOSO_NONE, NULL) == FAIL)
	{
	    semsg(_(e_cant_open_file_str), si->sn_name);
	    return FAIL;
	}
    }
    return OK;
}

// used for v_instr of typval of VAR_INSTR
struct instr_S {
    ectx_T	*instr_ectx;
    isn_T	*instr_instr;
};

// used for substitute_instr
typedef struct subs_expr_S {
    ectx_T	*subs_ectx;
    isn_T	*subs_instr;
    int		subs_status;
} subs_expr_T;

// Set when calling do_debug().
static ectx_T	*debug_context = NULL;
static int	debug_var_count;

/*
 * When debugging lookup "name" and return the typeval.
 * When not found return NULL.
 */
    typval_T *
lookup_debug_var(char_u *name)
{
    int		    idx;
    dfunc_T	    *dfunc;
    ufunc_T	    *ufunc;
    ectx_T	    *ectx = debug_context;
    int		    varargs_off;

    if (ectx == NULL)
	return NULL;
    dfunc = ((dfunc_T *)def_functions.ga_data) + ectx->ec_dfunc_idx;

    // Go through the local variable names, from last to first.
    for (idx = debug_var_count - 1; idx >= 0; --idx)
    {
	char_u *varname = ((char_u **)dfunc->df_var_names.ga_data)[idx];

	// the variable name may be NULL when not available in this block
	if (varname != NULL && STRCMP(varname, name) == 0)
	    return STACK_TV_VAR(idx);
    }

    // Go through argument names.
    ufunc = dfunc->df_ufunc;
    varargs_off = ufunc->uf_va_name == NULL ? 0 : 1;
    for (idx = 0; idx < ufunc->uf_args.ga_len; ++idx)
	if (STRCMP(((char_u **)(ufunc->uf_args.ga_data))[idx], name) == 0)
	    return STACK_TV(ectx->ec_frame_idx - ufunc->uf_args.ga_len
							  - varargs_off + idx);
    if (ufunc->uf_va_name != NULL && STRCMP(ufunc->uf_va_name, name) == 0)
	return STACK_TV(ectx->ec_frame_idx - 1);

    return NULL;
}

/*
 * Return TRUE if there might be a breakpoint in "ufunc", which is when a
 * breakpoint was set in that function or when there is any expression.
 */
    int
may_break_in_function(ufunc_T *ufunc)
{
    return ufunc->uf_has_breakpoint || debug_has_expr_breakpoint();
}

    static void
handle_debug(isn_T *iptr, ectx_T *ectx)
{
    char_u	*line;
    ufunc_T	*ufunc = (((dfunc_T *)def_functions.ga_data)
					       + ectx->ec_dfunc_idx)->df_ufunc;
    isn_T	*ni;
    int		end_lnum = iptr->isn_lnum;
    garray_T	ga;
    int		lnum;

    if (ex_nesting_level > debug_break_level)
    {
	linenr_T breakpoint;

	if (!may_break_in_function(ufunc))
	    return;

	// check for the next breakpoint if needed
	breakpoint = dbg_find_breakpoint(FALSE, ufunc->uf_name,
					   iptr->isn_arg.debug.dbg_break_lnum);
	if (breakpoint <= 0 || breakpoint > iptr->isn_lnum)
	    return;
    }

    SOURCING_LNUM = iptr->isn_lnum;
    debug_context = ectx;
    debug_var_count = iptr->isn_arg.debug.dbg_var_names_len;

    for (ni = iptr + 1; ni->isn_type != ISN_FINISH; ++ni)
	if (ni->isn_type == ISN_DEBUG
		  || ni->isn_type == ISN_RETURN
		  || ni->isn_type == ISN_RETURN_OBJECT
		  || ni->isn_type == ISN_RETURN_VOID)
	{
	    end_lnum = ni->isn_lnum + (ni->isn_type == ISN_DEBUG ? 0 : 1);
	    break;
	}

    if (end_lnum > iptr->isn_lnum)
    {
	ga_init2(&ga, sizeof(char_u *), 10);
	for (lnum = iptr->isn_lnum; lnum < end_lnum
				     && lnum <= ufunc->uf_lines.ga_len; ++lnum)
	{
	    char_u *p = ((char_u **)ufunc->uf_lines.ga_data)[lnum - 1];

	    if (p == NULL)
		continue;  // left over from continuation line
	    p = skipwhite(p);
	    if (*p == '#')
		break;
	    if (GA_GROW_OK(&ga, 1))
		((char_u **)(ga.ga_data))[ga.ga_len++] = p;
	    if (STRNCMP(p, "def ", 4) == 0)
		break;
	}
	line = ga_concat_strings(&ga, "  ");
	vim_free(ga.ga_data);
    }
    else
	line = ((char_u **)ufunc->uf_lines.ga_data)[iptr->isn_lnum - 1];

    do_debug(line == NULL ? (char_u *)"[empty]" : line);
    debug_context = NULL;

    if (end_lnum > iptr->isn_lnum)
	vim_free(line);
}

/*
 * Store a value in a list, dict, blob or object variable.
 * Returns OK, FAIL or NOTDONE (uncatchable error).
 */
    static int
execute_storeindex(isn_T *iptr, ectx_T *ectx)
{
    vartype_T	dest_type = iptr->isn_arg.storeindex.si_vartype;
    typval_T	*tv;
    typval_T	*tv_idx = STACK_TV_BOT(-2);
    long	lidx = 0;
    typval_T	*tv_dest = STACK_TV_BOT(-1);
    int		status = OK;

    if (tv_idx->v_type == VAR_NUMBER)
	lidx = (long)tv_idx->vval.v_number;

    // Stack contains:
    // -3 value to be stored
    // -2 index
    // -1 dict, list, blob, object or class
    tv = STACK_TV_BOT(-3);
    SOURCING_LNUM = iptr->isn_lnum;

    // Make sure an object has been initialized
    if (dest_type == VAR_OBJECT && tv_dest->vval.v_object == NULL)
    {
	emsg(_(e_using_null_object));
	status = FAIL;
    }
    else if (dest_type == VAR_ANY)
    {
	dest_type = tv_dest->v_type;
	if (dest_type == VAR_DICT)
	    status = do_2string(tv_idx, TRUE, FALSE);
	else if (dest_type == VAR_OBJECT && tv_idx->v_type == VAR_STRING)
	{
	    // Need to get the member index now that the class is known.
	    object_T *obj = tv_dest->vval.v_object;
	    class_T *cl = obj->obj_class;
	    char_u  *member = tv_idx->vval.v_string;

	    int		m_idx;
	    ocmember_T *m = object_member_lookup(cl, member, 0, &m_idx);
	    if (m != NULL)
	    {
		if (*member == '_')
		{
		    emsg_var_cl_define(e_cannot_access_protected_variable_str,
							m->ocm_name, 0, cl);
		    status = FAIL;
		}

		lidx = m_idx;
	    }
	    else
	    {
		member_not_found_msg(cl, VAR_OBJECT, member, 0);
		status = FAIL;
	    }
	}
	else if ((dest_type == VAR_LIST || dest_type == VAR_OBJECT)
		&& tv_idx->v_type != VAR_NUMBER)
	{
	    emsg(_(e_number_expected));
	    status = FAIL;
	}
    }

    if (status == OK)
    {
	if (dest_type == VAR_LIST)
	{
	    list_T	    *list = tv_dest->vval.v_list;

	    if (list == NULL)
	    {
		emsg(_(e_list_not_set));
		return FAIL;
	    }
	    if (lidx < 0 && list->lv_len + lidx >= 0)
		// negative index is relative to the end
		lidx = list->lv_len + lidx;
	    if (lidx < 0 || lidx > list->lv_len)
	    {
		semsg(_(e_list_index_out_of_range_nr), lidx);
		return FAIL;
	    }
	    if (lidx < list->lv_len)
	    {
		listitem_T *li = list_find(list, lidx);

		if (error_if_locked(li->li_tv.v_lock,
					     e_cannot_change_locked_list_item))
		    return FAIL;
		// overwrite existing list item
		clear_tv(&li->li_tv);
		li->li_tv = *tv;
	    }
	    else
	    {
		if (error_if_locked(list->lv_lock, e_cannot_change_locked_list))
		    return FAIL;
		// append to list, only fails when out of memory
		if (list_append_tv(list, tv) == FAIL)
		    return NOTDONE;
		clear_tv(tv);
	    }
	}
	else if (dest_type == VAR_DICT)
	{
	    char_u		*key = tv_idx->vval.v_string;
	    dict_T		*dict = tv_dest->vval.v_dict;
	    dictitem_T	*di;

	    SOURCING_LNUM = iptr->isn_lnum;
	    if (dict == NULL)
	    {
		emsg(_(e_dictionary_not_set));
		return FAIL;
	    }
	    if (key == NULL)
		key = (char_u *)"";
	    di = dict_find(dict, key, -1);
	    if (di != NULL)
	    {
		if (error_if_locked(di->di_tv.v_lock,
						    e_cannot_change_dict_item))
		    return FAIL;
		// overwrite existing value
		clear_tv(&di->di_tv);
		di->di_tv = *tv;
	    }
	    else
	    {
		if (error_if_locked(dict->dv_lock, e_cannot_change_dict))
		    return FAIL;
		// add to dict, only fails when out of memory
		if (dict_add_tv(dict, (char *)key, tv) == FAIL)
		    return NOTDONE;
		clear_tv(tv);
	    }
	}
	else if (dest_type == VAR_BLOB)
	{
	    blob_T	    *blob = tv_dest->vval.v_blob;
	    varnumber_T	    nr;
	    int		    error = FALSE;
	    int		    len;

	    if (blob == NULL)
	    {
		emsg(_(e_blob_not_set));
		return FAIL;
	    }
	    len = blob_len(blob);
	    if (lidx < 0 && len + lidx >= 0)
		// negative index is relative to the end
		lidx = len + lidx;

	    // Can add one byte at the end.
	    if (lidx < 0 || lidx > len)
	    {
		semsg(_(e_blob_index_out_of_range_nr), lidx);
		return FAIL;
	    }
	    if (value_check_lock(blob->bv_lock, (char_u *)"blob", FALSE))
		return FAIL;
	    nr = tv_get_number_chk(tv, &error);
	    if (error)
		return FAIL;
	    blob_set_append(blob, lidx, nr);
	}
	else if (dest_type == VAR_CLASS || dest_type == VAR_OBJECT)
	{
	    typval_T	    *otv;

	    if (dest_type == VAR_OBJECT)
	    {
		object_T	*obj = tv_dest->vval.v_object;

		otv = (typval_T *)(obj + 1);
		class_T	    *itf = iptr->isn_arg.storeindex.si_class;
		if (itf != NULL)
		    // convert interface member index to class member index
		    lidx = object_index_from_itf_index(itf, FALSE, lidx,
						       obj->obj_class);
	    }
	    else
	    {
		// VAR_CLASS
		class_T	*class = tv_dest->vval.v_class;
		otv = class->class_members_tv;
	    }

	    clear_tv(&otv[lidx]);
	    otv[lidx] = *tv;
	}
	else
	{
	    status = FAIL;
	    semsg(_(e_cannot_index_str), vartype_name(dest_type));
	}
    }

    clear_tv(tv_idx);
    clear_tv(tv_dest);
    ectx->ec_stack.ga_len -= 3;
    if (status == FAIL)
    {
	clear_tv(tv);
	return FAIL;
    }
    return OK;
}

/*
 * Store a value in a list or blob range.
 */
    static int
execute_storerange(isn_T *iptr, ectx_T *ectx)
{
    typval_T	*tv;
    typval_T	*tv_idx1 = STACK_TV_BOT(-3);
    typval_T	*tv_idx2 = STACK_TV_BOT(-2);
    typval_T	*tv_dest = STACK_TV_BOT(-1);
    int		status = OK;

    // Stack contains:
    // -4 value to be stored
    // -3 first index or "none"
    // -2 second index or "none"
    // -1 destination list or blob
    tv = STACK_TV_BOT(-4);
    SOURCING_LNUM = iptr->isn_lnum;
    if (tv_dest->v_type == VAR_LIST)
    {
	long	    n1;
	long	    n2;
	listitem_T  *li1;

	n1 = (long)tv_get_number_chk(tv_idx1, NULL);
	if (tv_idx2->v_type == VAR_SPECIAL
		    && tv_idx2->vval.v_number == VVAL_NONE)
	    n2 = list_len(tv_dest->vval.v_list) - 1;
	else
	    n2 = (long)tv_get_number_chk(tv_idx2, NULL);

	li1 = check_range_index_one(tv_dest->vval.v_list, &n1, TRUE, FALSE);
	if (li1 == NULL)
	    status = FAIL;
	else
	{
	    status = check_range_index_two(tv_dest->vval.v_list,
							 &n1, li1, &n2, FALSE);
	    if (status != FAIL)
		status = list_assign_range(
			tv_dest->vval.v_list,
			tv->vval.v_list,
			n1,
			n2,
			tv_idx2->v_type == VAR_SPECIAL,
			(char_u *)"=",
			(char_u *)"[unknown]");
	}
    }
    else if (tv_dest->v_type == VAR_BLOB)
    {
	varnumber_T n1;
	varnumber_T n2;
	long	    bloblen;

	n1 = tv_get_number_chk(tv_idx1, NULL);
	if (tv_idx2->v_type == VAR_SPECIAL
					&& tv_idx2->vval.v_number == VVAL_NONE)
	    n2 = blob_len(tv_dest->vval.v_blob) - 1;
	else
	    n2 = tv_get_number_chk(tv_idx2, NULL);
	bloblen = blob_len(tv_dest->vval.v_blob);

	if (check_blob_index(bloblen, n1, FALSE) == FAIL
		|| check_blob_range(bloblen, n1, n2, FALSE) == FAIL)
	    status = FAIL;
	else
	    status = blob_set_range(tv_dest->vval.v_blob, n1, n2, tv);
    }
    else
    {
	status = FAIL;
	emsg(_(e_list_or_blob_required));
    }

    clear_tv(tv_idx1);
    clear_tv(tv_idx2);
    clear_tv(tv_dest);
    ectx->ec_stack.ga_len -= 4;
    clear_tv(tv);

    return status;
}

/*
 * Unlet item in list or dict variable.
 */
    static int
execute_unletindex(isn_T *iptr, ectx_T *ectx)
{
    typval_T	*tv_idx = STACK_TV_BOT(-2);
    typval_T	*tv_dest = STACK_TV_BOT(-1);
    int		status = OK;

    // Stack contains:
    // -2 index
    // -1 dict or list
    SOURCING_LNUM = iptr->isn_lnum;
    if (tv_dest->v_type == VAR_DICT)
    {
	// unlet a dict item, index must be a string
	if (tv_idx->v_type != VAR_STRING && tv_idx->v_type != VAR_NUMBER)
	{
	    semsg(_(e_expected_str_but_got_str),
			vartype_name(VAR_STRING),
			vartype_name(tv_idx->v_type));
	    status = FAIL;
	}
	else
	{
	    dict_T	*d = tv_dest->vval.v_dict;
	    char_u	*key;
	    dictitem_T  *di = NULL;

	    if (d != NULL && value_check_lock(
				      d->dv_lock, NULL, FALSE))
		status = FAIL;
	    else
	    {
		if (tv_idx->v_type == VAR_STRING)
		{
		    key = tv_idx->vval.v_string;
		    if (key == NULL)
			key = (char_u *)"";
		}
		else
		{
		    key = tv_get_string(tv_idx);
		}
		if (d != NULL)
		    di = dict_find(d, key, (int)STRLEN(key));
		if (di == NULL)
		{
		    // NULL dict is equivalent to empty dict
		    semsg(_(e_key_not_present_in_dictionary_str), key);
		    status = FAIL;
		}
		else if (var_check_fixed(di->di_flags,
						   NULL, FALSE)
			|| var_check_ro(di->di_flags,
						  NULL, FALSE))
		    status = FAIL;
		else
		    dictitem_remove(d, di, "unlet");
	    }
	}
    }
    else if (tv_dest->v_type == VAR_LIST)
    {
	// unlet a List item, index must be a number
	if (check_for_number(tv_idx) == FAIL)
	{
	    status = FAIL;
	}
	else
	{
	    list_T	*l = tv_dest->vval.v_list;
	    long	n = (long)tv_idx->vval.v_number;

	    if (l != NULL && value_check_lock(
				      l->lv_lock, NULL, FALSE))
		status = FAIL;
	    else
	    {
		listitem_T	*li = list_find(l, n);

		if (li == NULL)
		{
		    semsg(_(e_list_index_out_of_range_nr), n);
		    status = FAIL;
		}
		else
		    listitem_remove(l, li);
	    }
	}
    }
    else
    {
	status = FAIL;
	semsg(_(e_cannot_index_str),
				vartype_name(tv_dest->v_type));
    }

    clear_tv(tv_idx);
    clear_tv(tv_dest);
    ectx->ec_stack.ga_len -= 2;

    return status;
}

/*
 * Unlet a range of items in a list variable.
 */
    static int
execute_unletrange(isn_T *iptr, ectx_T *ectx)
{
    // Stack contains:
    // -3 index1
    // -2 index2
    // -1 dict or list
    typval_T	*tv_idx1 = STACK_TV_BOT(-3);
    typval_T	*tv_idx2 = STACK_TV_BOT(-2);
    typval_T	*tv_dest = STACK_TV_BOT(-1);
    int		status = OK;

    if (tv_dest->v_type == VAR_LIST)
    {
	// indexes must be a number
	SOURCING_LNUM = iptr->isn_lnum;
	if (check_for_number(tv_idx1) == FAIL
		|| (tv_idx2->v_type != VAR_SPECIAL
			 && check_for_number(tv_idx2) == FAIL))
	{
	    status = FAIL;
	}
	else
	{
	    list_T	*l = tv_dest->vval.v_list;
	    long	n1 = (long)tv_idx1->vval.v_number;
	    long	n2 = tv_idx2->v_type == VAR_SPECIAL
			    ? 0 : (long)tv_idx2->vval.v_number;
	    listitem_T	*li;

	    li = list_find_index(l, &n1);
	    if (li == NULL)
	    {
		semsg(_(e_list_index_out_of_range_nr),
						 (long)tv_idx1->vval.v_number);
		status = FAIL;
	    }
	    else
	    {
		if (n1 < 0)
		    n1 = list_idx_of_item(l, li);
		if (n2 < 0)
		{
		    listitem_T *li2 = list_find(l, n2);

		    if (li2 == NULL)
		    {
			semsg(_(e_list_index_out_of_range_nr), n2);
			status = FAIL;
		    }
		    else
			n2 = list_idx_of_item(l, li2);
		}
		if (status != FAIL
			&& tv_idx2->v_type != VAR_SPECIAL
			&& n2 < n1)
		{
		    semsg(_(e_list_index_out_of_range_nr), n2);
		    status = FAIL;
		}
		if (status != FAIL)
		    list_unlet_range(l, li, n1,
					   tv_idx2->v_type != VAR_SPECIAL, n2);
	    }
	}
    }
    else
    {
	status = FAIL;
	SOURCING_LNUM = iptr->isn_lnum;
	semsg(_(e_cannot_index_str),
				vartype_name(tv_dest->v_type));
    }

    clear_tv(tv_idx1);
    clear_tv(tv_idx2);
    clear_tv(tv_dest);
    ectx->ec_stack.ga_len -= 3;

    return status;
}

/*
 * Top of a for loop.
 */
    static int
execute_for(isn_T *iptr, ectx_T *ectx)
{
    typval_T	*tv;
    int		jump = FALSE;
    typval_T	*ltv = STACK_TV_BOT(-1);
    typval_T	*idxtv =
		   STACK_TV_VAR(iptr->isn_arg.forloop.for_loop_idx);

    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
	return FAIL;
    if (ltv->v_type == VAR_LIST)
    {
	list_T *list = ltv->vval.v_list;

	// push the next item from the list
	++idxtv->vval.v_number;
	if (list == NULL
		       || idxtv->vval.v_number >= list->lv_len)
	{
	    jump = TRUE;
	}
	else if (list->lv_first == &range_list_item)
	{
	    // non-materialized range() list
	    tv = STACK_TV_BOT(0);
	    tv->v_type = VAR_NUMBER;
	    tv->v_lock = 0;
	    tv->vval.v_number = list_find_nr(
			     list, idxtv->vval.v_number, NULL);
	    ++ectx->ec_stack.ga_len;
	}
	else
	{
	    listitem_T *li = list_find(list,
					 idxtv->vval.v_number);

	    copy_tv(&li->li_tv, STACK_TV_BOT(0));
	    ++ectx->ec_stack.ga_len;
	}
    }
    else if (ltv->v_type == VAR_STRING)
    {
	char_u	*str = ltv->vval.v_string;

	// The index is for the last byte of the previous
	// character.
	++idxtv->vval.v_number;
	if (str == NULL || str[idxtv->vval.v_number] == NUL)
	{
	    jump = TRUE;
	}
	else
	{
	    int	clen = mb_ptr2len(str + idxtv->vval.v_number);

	    // Push the next character from the string.
	    tv = STACK_TV_BOT(0);
	    tv->v_type = VAR_STRING;
	    tv->vval.v_string = vim_strnsave(
			     str + idxtv->vval.v_number, clen);
	    ++ectx->ec_stack.ga_len;
	    idxtv->vval.v_number += clen - 1;
	}
    }
    else if (ltv->v_type == VAR_BLOB)
    {
	blob_T	*blob = ltv->vval.v_blob;

	// When we get here the first time make a copy of the
	// blob, so that the iteration still works when it is
	// changed.
	if (idxtv->vval.v_number == -1 && blob != NULL)
	{
	    blob_copy(blob, ltv);
	    blob_unref(blob);
	    blob = ltv->vval.v_blob;
	}

	// The index is for the previous byte.
	++idxtv->vval.v_number;
	if (blob == NULL || idxtv->vval.v_number >= blob_len(blob))
	{
	    jump = TRUE;
	}
	else
	{
	    // Push the next byte from the blob.
	    tv = STACK_TV_BOT(0);
	    tv->v_type = VAR_NUMBER;
	    tv->vval.v_number = blob_get(blob,
					 idxtv->vval.v_number);
	    ++ectx->ec_stack.ga_len;
	}
    }
    else
    {
	semsg(_(e_for_loop_on_str_not_supported),
				    vartype_name(ltv->v_type));
	return FAIL;
    }

    if (jump)
    {
	// past the end of the list/string/blob, jump to "endfor"
	ectx->ec_iidx = iptr->isn_arg.forloop.for_end;
	may_restore_cmdmod(&ectx->ec_funclocal);
    }
    else
    {
	// Store the current number of funcrefs, this may be used in
	// ISN_LOOPEND.  The variable index is always one more than the loop
	// variable index.
	tv = STACK_TV_VAR(iptr->isn_arg.forloop.for_loop_idx + 1);
	tv->vval.v_number = ectx->ec_funcrefs.ga_len;
    }

    return OK;
}

/*
 * Code for handling variables declared inside a loop and used in a closure.
 * This is very similar to what is done with funcstack_T.  The difference is
 * that the funcstack_T has the scope of a function, while a loopvars_T has the
 * scope of the block inside a loop and each loop may have its own.
 */

// Double linked list of loopvars_T in use.
static loopvars_T *first_loopvars = NULL;

    static void
add_loopvars_to_list(loopvars_T *loopvars)
{
	// Link in list of loopvarss.
    if (first_loopvars != NULL)
	first_loopvars->lvs_prev = loopvars;
    loopvars->lvs_next = first_loopvars;
    loopvars->lvs_prev = NULL;
    first_loopvars = loopvars;
}

    static void
remove_loopvars_from_list(loopvars_T *loopvars)
{
    if (loopvars->lvs_prev == NULL)
	first_loopvars = loopvars->lvs_next;
    else
	loopvars->lvs_prev->lvs_next = loopvars->lvs_next;
    if (loopvars->lvs_next != NULL)
	loopvars->lvs_next->lvs_prev = loopvars->lvs_prev;
}

/*
 * End of a for or while loop: Handle any variables used by a closure.
 */
    static int
execute_endloop(isn_T *iptr, ectx_T *ectx)
{
    endloop_T	*endloop = &iptr->isn_arg.endloop;
    typval_T	*tv_refcount = STACK_TV_VAR(endloop->end_funcref_idx);
    int		prev_closure_count = tv_refcount->vval.v_number;
    int		depth = endloop->end_depth;
    garray_T	*gap = &ectx->ec_funcrefs;
    int		closure_in_use = FALSE;
    loopvars_T  *loopvars;
    typval_T    *stack;
    int		idx;

    // Check if any created closure is still being referenced and loopvars have
    // not been saved yet for the current depth.
    for (idx = prev_closure_count; idx < gap->ga_len; ++idx)
    {
	partial_T   *pt = ((partial_T **)gap->ga_data)[idx];

	if (pt->pt_refcount > 1 && pt->pt_loopvars[depth] == NULL)
	{
	    int refcount = pt->pt_refcount;
	    int i;

	    // A Reference in a variable inside the loop doesn't count, it gets
	    // unreferenced at the end of the loop.
	    for (i = 0; i < endloop->end_var_count; ++i)
	    {
		typval_T *stv = STACK_TV_VAR(endloop->end_var_idx + i);

		if (stv->v_type == VAR_PARTIAL && pt == stv->vval.v_partial)
		    --refcount;
	    }
	    if (refcount > 1)
	    {
		closure_in_use = TRUE;
		break;
	    }
	}
    }

    // If no function reference were created since the start of the loop block
    // or it is no longer referenced there is nothing to do.
    if (!closure_in_use)
	return OK;

    // A closure is using variables declared inside the loop.
    // Move them to the called function.
    loopvars = ALLOC_CLEAR_ONE(loopvars_T);
    if (loopvars == NULL)
	return FAIL;

    loopvars->lvs_ga.ga_len = endloop->end_var_count;
    stack = ALLOC_CLEAR_MULT(typval_T, loopvars->lvs_ga.ga_len);
    loopvars->lvs_ga.ga_data = stack;
    if (stack == NULL)
    {
	vim_free(loopvars);
	return FAIL;
    }
    add_loopvars_to_list(loopvars);

    // Move the variable values.
    for (idx = 0; idx < endloop->end_var_count; ++idx)
    {
	typval_T *tv = STACK_TV_VAR(endloop->end_var_idx + idx);

	*(stack + idx) = *tv;
	tv->v_type = VAR_UNKNOWN;
    }

    for (idx = prev_closure_count; idx < gap->ga_len; ++idx)
    {
	partial_T   *pt = ((partial_T **)gap->ga_data)[idx];

	if (pt->pt_refcount > 1 && pt->pt_loopvars[depth] == NULL)
	{
	    ++loopvars->lvs_refcount;
	    pt->pt_loopvars[depth] = loopvars;

	    pt->pt_outer.out_loop[depth].stack = &loopvars->lvs_ga;
	    pt->pt_outer.out_loop[depth].var_idx -=
		  ectx->ec_frame_idx + STACK_FRAME_SIZE + endloop->end_var_idx;
	}
    }

    return OK;
}

/*
 * Called when a partial is freed or its reference count goes down to one.  The
 * loopvars may be the only reference to the partials in the local variables.
 * Go over all of them, the funcref and can be freed if all partials
 * referencing the loopvars have a reference count of one.
 * Return TRUE if it was freed.
 */
    int
loopvars_check_refcount(loopvars_T *loopvars)
{
    int		    i;
    garray_T	    *gap = &loopvars->lvs_ga;
    int		    done = 0;
	typval_T	*stack = gap->ga_data;

    if (loopvars->lvs_refcount > loopvars->lvs_min_refcount)
	return FALSE;
    for (i = 0; i < gap->ga_len; ++i)
    {
	typval_T    *tv = ((typval_T *)gap->ga_data) + i;

	if (tv->v_type == VAR_PARTIAL && tv->vval.v_partial != NULL
		&& tv->vval.v_partial->pt_refcount == 1)
	{
	    int	    depth;

	    for (depth = 0; depth < MAX_LOOP_DEPTH; ++depth)
		if (tv->vval.v_partial->pt_loopvars[depth] == loopvars)
		    ++done;
	}
    }
    if (done != loopvars->lvs_min_refcount)
	return FALSE;

    // All partials referencing the loopvars have a reference count of
    // one, thus the loopvars is no longer of use.
    stack = gap->ga_data;
    for (i = 0; i < gap->ga_len; ++i)
	clear_tv(stack + i);
    vim_free(stack);
    remove_loopvars_from_list(loopvars);
    vim_free(loopvars);
    return TRUE;
}

/*
 * For garbage collecting: set references in all variables referenced by
 * all loopvars.
 */
    int
set_ref_in_loopvars(int copyID)
{
    loopvars_T *loopvars;

    for (loopvars = first_loopvars; loopvars != NULL;
						 loopvars = loopvars->lvs_next)
    {
	typval_T    *stack = loopvars->lvs_ga.ga_data;
	int	    i;

	for (i = 0; i < loopvars->lvs_ga.ga_len; ++i)
	    if (set_ref_in_item(stack + i, copyID, NULL, NULL))
		return TRUE;  // abort
    }
    return FALSE;
}

/*
 * Load instruction for w:/b:/g:/t: variable.
 * "isn_type" is used instead of "iptr->isn_type".
 */
    static int
load_namespace_var(ectx_T *ectx, isntype_T isn_type, isn_T *iptr)
{
    dictitem_T	*di = NULL;
    hashtab_T	*ht = NULL;
    char	namespace;

    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
	return NOTDONE;
    switch (isn_type)
    {
	case ISN_LOADG:
	    ht = get_globvar_ht();
	    namespace = 'g';
	    break;
	case ISN_LOADB:
	    ht = &curbuf->b_vars->dv_hashtab;
	    namespace = 'b';
	    break;
	case ISN_LOADW:
	    ht = &curwin->w_vars->dv_hashtab;
	    namespace = 'w';
	    break;
	case ISN_LOADT:
	    ht = &curtab->tp_vars->dv_hashtab;
	    namespace = 't';
	    break;
	default:  // Cannot reach here
	    return NOTDONE;
    }
    di = find_var_in_ht(ht, 0, iptr->isn_arg.string, TRUE);

    if (di == NULL)
    {
	if (isn_type == ISN_LOADG)
	{
	    ufunc_T *ufunc = find_func(iptr->isn_arg.string, TRUE);

	    // g:Something could be a function
	    if (ufunc != NULL)
	    {
		typval_T    *tv = STACK_TV_BOT(0);

		++ectx->ec_stack.ga_len;
		tv->v_type = VAR_FUNC;
		tv->vval.v_string = alloc(STRLEN(iptr->isn_arg.string) + 3);
		if (tv->vval.v_string == NULL)
		    return FAIL;
		STRCPY(tv->vval.v_string, "g:");
		STRCPY(tv->vval.v_string + 2, iptr->isn_arg.string);
		return OK;
	    }
	}
	SOURCING_LNUM = iptr->isn_lnum;
	if (vim_strchr(iptr->isn_arg.string, AUTOLOAD_CHAR) != NULL)
	    // no check if the item exists in the script but
	    // isn't exported, it is too complicated
	    semsg(_(e_item_not_found_in_script_str), iptr->isn_arg.string);
	else
	    semsg(_(e_undefined_variable_char_str),
					      namespace, iptr->isn_arg.string);
	return FAIL;
    }
    else
    {
	copy_tv(&di->di_tv, STACK_TV_BOT(0));
	++ectx->ec_stack.ga_len;
    }
    return OK;
}


    static void
object_required_error(typval_T *tv)
{
    garray_T type_list;
    ga_init2(&type_list, sizeof(type_T *), 10);
    type_T *type = typval2type(tv, get_copyID(), &type_list, TVTT_DO_MEMBER);
    char *tofree = NULL;
    char *typename = type_name(type, &tofree);
    semsg(_(e_object_required_found_str), typename);
    vim_free(tofree);
    clear_type_list(&type_list);
}

/*
 * Execute instructions in execution context "ectx".
 * Return OK or FAIL;
 */
    static int
exec_instructions(ectx_T *ectx)
{
    int		ret = FAIL;
    int		save_trylevel_at_start = ectx->ec_trylevel_at_start;
    int		dict_stack_len_at_start = dict_stack.ga_len;

    // Start execution at the first instruction.
    ectx->ec_iidx = 0;

    // Only catch exceptions in this instruction list.
    ectx->ec_trylevel_at_start = trylevel;

    for (;;)
    {
	static int  breakcheck_count = 0;  // using "static" makes it faster
	isn_T	    *iptr;
	typval_T    *tv;

	if (unlikely(++breakcheck_count >= 100))
	{
	    line_breakcheck();
	    breakcheck_count = 0;
	}
	if (unlikely(got_int))
	{
	    // Turn CTRL-C into an exception.
	    got_int = FALSE;
	    if (throw_exception("Vim:Interrupt", ET_INTERRUPT, NULL) == FAIL)
		goto theend;
	    did_throw = TRUE;
	}

	if (unlikely(did_emsg && msg_list != NULL && *msg_list != NULL))
	{
	    // Turn an error message into an exception.
	    did_emsg = FALSE;
	    if (throw_exception(*msg_list, ET_ERROR, NULL) == FAIL)
		goto theend;
	    did_throw = TRUE;
	    *msg_list = NULL;

	    // This exception was not caught (yet).
	    garray_T	*trystack = &ectx->ec_trystack;
	    if (trystack->ga_len > 0)
	    {
		trycmd_T *trycmd = ((trycmd_T *)trystack->ga_data)
							+ trystack->ga_len - 1;
		if (trycmd->tcd_frame_idx == ectx->ec_frame_idx)
		    trycmd->tcd_caught = FALSE;
	    }
	}

	if (unlikely(did_throw))
	{
	    garray_T	*trystack = &ectx->ec_trystack;
	    trycmd_T    *trycmd = NULL;
	    int		index = trystack->ga_len;

	    // An exception jumps to the first catch, finally, or returns from
	    // the current function.
	    while (index > 0)
	    {
		trycmd = ((trycmd_T *)trystack->ga_data) + index - 1;
		// 1. after :try and before :catch - jump to first :catch
		// 2. in :catch block - jump to :finally
		// 3. in :catch block and no finally - jump to :endtry
		if (!trycmd->tcd_in_catch || trycmd->tcd_finally_idx != 0
				|| trycmd->tcd_frame_idx == ectx->ec_frame_idx)
		    break;
		// In the catch and finally block of this try we have to go up
		// one level.
		--index;
		trycmd = NULL;
	    }
	    if (trycmd != NULL && trycmd->tcd_frame_idx == ectx->ec_frame_idx)
	    {
		if (trycmd->tcd_in_catch)
		{
		    if (trycmd->tcd_finally_idx > 0)
		    {
			// exception inside ":catch", jump to ":finally" once
			ectx->ec_iidx = trycmd->tcd_finally_idx;
			trycmd->tcd_finally_idx = 0;
		    }
		    else
		    {
			// exception inside ":catch" or ":finally", jump to
			// ":endtry"
			ectx->ec_iidx = trycmd->tcd_endtry_idx;
		    }
		}
		else
		{
		    // jump to first ":catch"
		    ectx->ec_iidx = trycmd->tcd_catch_idx;
		    trycmd->tcd_in_catch = TRUE;
		}
		did_throw = FALSE;  // don't come back here until :endtry
		trycmd->tcd_did_throw = TRUE;
	    }
	    else
	    {
		// Not inside try or need to return from current function.
		// Push a dummy return value.
		if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    goto theend;
		tv = STACK_TV_BOT(0);
		tv->v_type = VAR_NUMBER;
		tv->vval.v_number = 0;
		++ectx->ec_stack.ga_len;
		if (ectx->ec_frame_idx == ectx->ec_initial_frame_idx)
		{
		    // At the toplevel we are done.
		    need_rethrow = TRUE;
		    if (handle_closure_in_use(ectx, FALSE) == FAIL)
			goto theend;
		    goto done;
		}

		if (func_return(ectx) == FAIL)
		    goto theend;
	    }
	    continue;
	}

	/*
	 * Big switch on the instruction.  Most compilers will be turning this
	 * into an efficient lookup table, since the "case" values are an enum
	 * with sequential numbers.  It may look ugly, but it should be the
	 * most efficient way.
	 */
	iptr = &ectx->ec_instr[ectx->ec_iidx++];
	switch (iptr->isn_type)
	{
	    // Constructor, first instruction in a new() method.
	    case ISN_CONSTRUCT:
		// "this" is always the local variable at index zero
		tv = STACK_TV_VAR(0);
		tv->v_type = VAR_OBJECT;
		tv->vval.v_object = alloc_clear(
				       iptr->isn_arg.construct.construct_size);
		tv->vval.v_object->obj_class =
				       iptr->isn_arg.construct.construct_class;
		++tv->vval.v_object->obj_class->class_refcount;
		tv->vval.v_object->obj_refcount = 1;
		object_created(tv->vval.v_object);
		break;

	    // execute Ex command line
	    case ISN_EXEC:
		if (exec_command(iptr, iptr->isn_arg.string) == FAIL)
		    goto on_error;
		break;

	    // execute Ex command line split at NL characters.
	    case ISN_EXEC_SPLIT:
		{
		    source_cookie_T cookie;
		    char_u	    *line;

		    SOURCING_LNUM = iptr->isn_lnum;
		    CLEAR_FIELD(cookie);
		    cookie.sourcing_lnum = iptr->isn_lnum - 1;
		    cookie.nextline = iptr->isn_arg.string;
		    line = get_split_sourceline(0, &cookie, 0, 0);
		    if (do_cmdline(line,
				get_split_sourceline, &cookie,
				   DOCMD_VERBOSE|DOCMD_NOWAIT|DOCMD_KEYTYPED)
									== FAIL
				|| did_emsg)
		    {
			vim_free(line);
			goto on_error;
		    }
		    vim_free(line);
		}
		break;

	    // execute Ex command line that is only a range
	    case ISN_EXECRANGE:
		{
		    exarg_T	ea;
		    char	*error = NULL;

		    CLEAR_FIELD(ea);
		    ea.cmdidx = CMD_SIZE;
		    ea.addr_type = ADDR_LINES;
		    ea.cmd = iptr->isn_arg.string;
		    SOURCING_LNUM = iptr->isn_lnum;
		    parse_cmd_address(&ea, &error, FALSE);
		    if (ea.cmd == NULL)
			goto on_error;
		    // error is always NULL when using ADDR_LINES
		    error = ex_range_without_command(&ea);
		    if (error != NULL)
		    {
			emsg(error);
			goto on_error;
		    }
		}
		break;

	    // Evaluate an expression with legacy syntax, push it onto the
	    // stack.
	    case ISN_LEGACY_EVAL:
		{
		    char_u  *arg = iptr->isn_arg.string;
		    int	    res;
		    int	    save_flags = cmdmod.cmod_flags;

		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    tv = STACK_TV_BOT(0);
		    init_tv(tv);
		    cmdmod.cmod_flags |= CMOD_LEGACY;
		    res = eval0(arg, tv, NULL, &EVALARG_EVALUATE);
		    cmdmod.cmod_flags = save_flags;
		    if (res == FAIL)
			goto on_error;
		    ++ectx->ec_stack.ga_len;
		}
		break;

	    // push typeval VAR_INSTR with instructions to be executed
	    case ISN_INSTR:
		{
		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    tv = STACK_TV_BOT(0);
		    tv->vval.v_instr = ALLOC_ONE(instr_T);
		    if (tv->vval.v_instr == NULL)
			goto on_error;
		    ++ectx->ec_stack.ga_len;

		    tv->v_type = VAR_INSTR;
		    tv->vval.v_instr->instr_ectx = ectx;
		    tv->vval.v_instr->instr_instr = iptr->isn_arg.instr;
		}
		break;

	    case ISN_SOURCE:
		{
		    int notused;

		    SOURCING_LNUM = iptr->isn_lnum;
		    if (may_load_script((int)iptr->isn_arg.number, &notused)
								       == FAIL)
			goto on_error;
		}
		break;

	    // execute :substitute with an expression
	    case ISN_SUBSTITUTE:
		{
		    subs_T		*subs = &iptr->isn_arg.subs;
		    source_cookie_T	cookie;
		    struct subs_expr_S	*save_instr = substitute_instr;
		    struct subs_expr_S	subs_instr;
		    int			res;

		    subs_instr.subs_ectx = ectx;
		    subs_instr.subs_instr = subs->subs_instr;
		    subs_instr.subs_status = OK;
		    substitute_instr = &subs_instr;

		    SOURCING_LNUM = iptr->isn_lnum;
		    // This is very much like ISN_EXEC
		    CLEAR_FIELD(cookie);
		    cookie.sourcing_lnum = iptr->isn_lnum - 1;
		    res = do_cmdline(subs->subs_cmd,
				getsourceline, &cookie,
				   DOCMD_VERBOSE|DOCMD_NOWAIT|DOCMD_KEYTYPED);
		    substitute_instr = save_instr;

		    if (res == FAIL || did_emsg
					     || subs_instr.subs_status == FAIL)
			goto on_error;
		}
		break;

	    case ISN_FINISH:
		goto done;

	    case ISN_REDIRSTART:
		// create a dummy entry for var_redir_str()
		if (alloc_redir_lval() == FAIL)
		    goto on_error;

		// The output is stored in growarray "redir_ga" until
		// redirection ends.
		init_redir_ga();
		redir_vname = 1;
		break;

	    case ISN_REDIREND:
		{
		    char_u *res = get_clear_redir_ga();

		    // End redirection, put redirected text on the stack.
		    clear_redir_lval();
		    redir_vname = 0;

		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    {
			vim_free(res);
			goto theend;
		    }
		    tv = STACK_TV_BOT(0);
		    tv->v_type = VAR_STRING;
		    tv->vval.v_string = res;
		    ++ectx->ec_stack.ga_len;
		}
		break;

	    case ISN_CEXPR_AUCMD:
#ifdef FEAT_QUICKFIX
		force_abort = TRUE;
		if (trigger_cexpr_autocmd(iptr->isn_arg.number) == FAIL)
		    goto on_error;
		force_abort = FALSE;
#endif
		break;

	    case ISN_CEXPR_CORE:
#ifdef FEAT_QUICKFIX
		{
		    exarg_T ea;
		    int	    res;

		    CLEAR_FIELD(ea);
		    ea.cmdidx = iptr->isn_arg.cexpr.cexpr_ref->cer_cmdidx;
		    ea.forceit = iptr->isn_arg.cexpr.cexpr_ref->cer_forceit;
		    ea.cmdlinep = &iptr->isn_arg.cexpr.cexpr_ref->cer_cmdline;
		    --ectx->ec_stack.ga_len;
		    tv = STACK_TV_BOT(0);
		    SOURCING_LNUM = iptr->isn_lnum;
		    res = cexpr_core(&ea, tv);
		    clear_tv(tv);
		    if (res == FAIL)
			goto on_error;
		}
#endif
		break;

	    // execute Ex command from pieces on the stack
	    case ISN_EXECCONCAT:
		{
		    int	    count = iptr->isn_arg.number;
		    size_t  len = 0;
		    int	    pass;
		    int	    i;
		    char_u  *cmd = NULL;
		    char_u  *str;

		    for (pass = 1; pass <= 2; ++pass)
		    {
			for (i = 0; i < count; ++i)
			{
			    tv = STACK_TV_BOT(i - count);
			    str = tv->vval.v_string;
			    if (str != NULL && *str != NUL)
			    {
				if (pass == 2)
				    STRCPY(cmd + len, str);
				len += STRLEN(str);
			    }
			    if (pass == 2)
				clear_tv(tv);
			}
			if (pass == 1)
			{
			    cmd = alloc(len + 1);
			    if (unlikely(cmd == NULL))
				goto theend;
			    len = 0;
			}
		    }

		    SOURCING_LNUM = iptr->isn_lnum;
		    do_cmdline_cmd(cmd);
		    vim_free(cmd);
		}
		break;

	    // execute :echo {string} ...
	    case ISN_ECHO:
		{
		    int count = iptr->isn_arg.echo.echo_count;
		    int	atstart = TRUE;
		    int needclr = TRUE;
		    int	idx;

		    for (idx = 0; idx < count; ++idx)
		    {
			tv = STACK_TV_BOT(idx - count);
			echo_one(tv, iptr->isn_arg.echo.echo_with_white,
							   &atstart, &needclr);
			clear_tv(tv);
		    }
		    if (needclr)
			msg_clr_eos();
		    ectx->ec_stack.ga_len -= count;
		}
		break;

	    // :execute {string} ...
	    // :echomsg {string} ...
	    // :echowindow {string} ...
	    // :echoconsole {string} ...
	    // :echoerr {string} ...
	    case ISN_EXECUTE:
	    case ISN_ECHOMSG:
	    case ISN_ECHOWINDOW:
	    case ISN_ECHOCONSOLE:
	    case ISN_ECHOERR:
		{
		    int		count;
		    garray_T	ga;
		    char_u	buf[NUMBUFLEN];
		    char_u	*p;
		    int		len;
		    int		failed = FALSE;
		    int		idx;

		    if (iptr->isn_type == ISN_ECHOWINDOW)
			count = iptr->isn_arg.echowin.ewin_count;
		    else
			count = iptr->isn_arg.number;
		    ga_init2(&ga, 1, 80);
		    for (idx = 0; idx < count; ++idx)
		    {
			tv = STACK_TV_BOT(idx - count);
			if (iptr->isn_type == ISN_EXECUTE)
			{
			    if (tv->v_type == VAR_CHANNEL
						      || tv->v_type == VAR_JOB)
			    {
				SOURCING_LNUM = iptr->isn_lnum;
				semsg(_(e_using_invalid_value_as_string_str),
						    vartype_name(tv->v_type));
				break;
			    }
			    else
				p = tv_get_string_buf(tv, buf);
			}
			else
			    p = tv_stringify(tv, buf);

			len = (int)STRLEN(p);
			if (GA_GROW_FAILS(&ga, len + 2))
			    failed = TRUE;
			else
			{
			    if (ga.ga_len > 0)
				((char_u *)(ga.ga_data))[ga.ga_len++] = ' ';
			    STRCPY((char_u *)(ga.ga_data) + ga.ga_len, p);
			    ga.ga_len += len;
			}
			clear_tv(tv);
		    }
		    ectx->ec_stack.ga_len -= count;
		    if (failed)
		    {
			ga_clear(&ga);
			goto on_error;
		    }

		    if (ga.ga_data != NULL)
		    {
			if (iptr->isn_type == ISN_EXECUTE)
			{
			    SOURCING_LNUM = iptr->isn_lnum;
			    do_cmdline_cmd((char_u *)ga.ga_data);
			    if (did_emsg)
			    {
				ga_clear(&ga);
				goto on_error;
			    }
			}
			else
			{
			    msg_sb_eol();
			    if (iptr->isn_type == ISN_ECHOMSG)
			    {
				msg_attr(ga.ga_data, echo_attr);
				out_flush();
			    }
#ifdef HAS_MESSAGE_WINDOW
			    else if (iptr->isn_type == ISN_ECHOWINDOW)
			    {
				start_echowindow(
					      iptr->isn_arg.echowin.ewin_time);
				msg_attr(ga.ga_data, echo_attr);
				end_echowindow();
			    }
#endif
			    else if (iptr->isn_type == ISN_ECHOCONSOLE)
			    {
				ui_write(ga.ga_data, (int)STRLEN(ga.ga_data),
									 TRUE);
				ui_write((char_u *)"\r\n", 2, TRUE);
			    }
			    else
			    {
				SOURCING_LNUM = iptr->isn_lnum;
				emsg(ga.ga_data);
			    }
			}
		    }
		    ga_clear(&ga);
		}
		break;

	    // load local variable or argument
	    case ISN_LOAD:
		if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    goto theend;
		tv = STACK_TV_VAR(iptr->isn_arg.number);
		if (tv->v_type == VAR_UNKNOWN)
		{
		    // missing argument or default value v:none
		    STACK_TV_BOT(0)->v_type = VAR_SPECIAL;
		    STACK_TV_BOT(0)->vval.v_number = VVAL_NONE;
		}
		else
		    copy_tv(tv, STACK_TV_BOT(0));
		++ectx->ec_stack.ga_len;
		break;

	    // load v: variable
	    case ISN_LOADV:
		if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    goto theend;
		copy_tv(get_vim_var_tv(iptr->isn_arg.number), STACK_TV_BOT(0));
		++ectx->ec_stack.ga_len;
		break;

	    // load s: variable in Vim9 script
	    case ISN_LOADSCRIPT:
		{
		    scriptref_T	*sref = iptr->isn_arg.script.scriptref;
		    svar_T	 *sv;

		    sv = get_script_svar(sref, ectx->ec_dfunc_idx);
		    if (sv == NULL)
			goto theend;
		    allocate_if_null(sv);
		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    copy_tv(sv->sv_tv, STACK_TV_BOT(0));
		    ++ectx->ec_stack.ga_len;
		}
		break;

	    // load s: variable in old script or autoload import
	    case ISN_LOADS:
	    case ISN_LOADEXPORT:
		{
		    int		sid = iptr->isn_arg.loadstore.ls_sid;
		    hashtab_T	*ht = &SCRIPT_VARS(sid);
		    char_u	*name = iptr->isn_arg.loadstore.ls_name;
		    dictitem_T	*di = find_var_in_ht(ht, 0, name, TRUE);

		    if (di == NULL)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			semsg(_(e_undefined_variable_str), name);
			goto on_error;
		    }
		    else
		    {
			if (iptr->isn_type == ISN_LOADEXPORT)
			{
			    int idx = get_script_item_idx(sid, name, 0,
								   NULL, NULL);
			    svar_T	*sv;

			    if (idx >= 0)
			    {
				sv = ((svar_T *)SCRIPT_ITEM(sid)
						  ->sn_var_vals.ga_data) + idx;
				if ((sv->sv_flags & SVFLAG_EXPORTED) == 0)
				{
				    SOURCING_LNUM = iptr->isn_lnum;
				    semsg(_(e_item_not_exported_in_script_str),
									 name);
				    goto on_error;
				}
			    }
			}
			if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			    goto theend;
			copy_tv(&di->di_tv, STACK_TV_BOT(0));
			++ectx->ec_stack.ga_len;
		    }
		}
		break;

	    // load g:/b:/w:/t: variable
	    case ISN_LOADG:
	    case ISN_LOADB:
	    case ISN_LOADW:
	    case ISN_LOADT:
		{
		    int res = load_namespace_var(ectx, iptr->isn_type, iptr);

		    if (res == NOTDONE)
			goto theend;
		    if (res == FAIL)
			goto on_error;
		}

		break;

	    // load autoload variable
	    case ISN_LOADAUTO:
		{
		    char_u *name = iptr->isn_arg.string;

		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    SOURCING_LNUM = iptr->isn_lnum;
		    if (eval_variable(name, (int)STRLEN(name), 0,
			      STACK_TV_BOT(0), NULL, EVAL_VAR_VERBOSE) == FAIL)
			goto on_error;
		    ++ectx->ec_stack.ga_len;
		}
		break;

	    // load g:/b:/w:/t: namespace
	    case ISN_LOADGDICT:
	    case ISN_LOADBDICT:
	    case ISN_LOADWDICT:
	    case ISN_LOADTDICT:
		{
		    dict_T *d = NULL;

		    switch (iptr->isn_type)
		    {
			case ISN_LOADGDICT: d = get_globvar_dict(); break;
			case ISN_LOADBDICT: d = curbuf->b_vars; break;
			case ISN_LOADWDICT: d = curwin->w_vars; break;
			case ISN_LOADTDICT: d = curtab->tp_vars; break;
			default:  // Cannot reach here
			    goto theend;
		    }
		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    tv = STACK_TV_BOT(0);
		    tv->v_type = VAR_DICT;
		    tv->v_lock = 0;
		    tv->vval.v_dict = d;
		    ++d->dv_refcount;
		    ++ectx->ec_stack.ga_len;
		}
		break;

	    // load &option
	    case ISN_LOADOPT:
		{
		    typval_T	optval;
		    char_u	*name = iptr->isn_arg.string;

		    // This is not expected to fail, name is checked during
		    // compilation: don't set SOURCING_LNUM.
		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    if (eval_option(&name, &optval, TRUE) == FAIL)
			goto theend;
		    *STACK_TV_BOT(0) = optval;
		    ++ectx->ec_stack.ga_len;
		}
		break;

	    // load $ENV
	    case ISN_LOADENV:
		{
		    typval_T	optval;
		    char_u	*name = iptr->isn_arg.string;

		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    // name is always valid, checked when compiling
		    (void)eval_env_var(&name, &optval, TRUE);
		    *STACK_TV_BOT(0) = optval;
		    ++ectx->ec_stack.ga_len;
		}
		break;

	    // load @register
	    case ISN_LOADREG:
		if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    goto theend;
		tv = STACK_TV_BOT(0);
		tv->v_type = VAR_STRING;
		tv->v_lock = 0;
		// This may result in NULL, which should be equivalent to an
		// empty string.
		tv->vval.v_string = get_reg_contents(
					  iptr->isn_arg.number, GREG_EXPR_SRC);
		++ectx->ec_stack.ga_len;
		break;

	    // store local variable
	    case ISN_STORE:
		--ectx->ec_stack.ga_len;
		tv = STACK_TV_VAR(iptr->isn_arg.number);
		if (check_typval_is_value(STACK_TV_BOT(0)) == FAIL)
		{
		    clear_tv(STACK_TV_BOT(0));
		    goto on_error;
		}
		clear_tv(tv);
		*tv = *STACK_TV_BOT(0);
		break;

	    // store s: variable in old script or autoload import
	    case ISN_STORES:
	    case ISN_STOREEXPORT:
		{
		    int		sid = iptr->isn_arg.loadstore.ls_sid;
		    hashtab_T	*ht = &SCRIPT_VARS(sid);
		    char_u	*name = iptr->isn_arg.loadstore.ls_name;
		    dictitem_T	*di = find_var_in_ht(ht, 0,
					    iptr->isn_type == ISN_STORES
						     ? name + 2 : name, TRUE);

		    --ectx->ec_stack.ga_len;
		    SOURCING_LNUM = iptr->isn_lnum;
		    if (di == NULL)
		    {
			if (iptr->isn_type == ISN_STOREEXPORT)
			{
			    semsg(_(e_undefined_variable_str), name);
			    clear_tv(STACK_TV_BOT(0));
			    goto on_error;
			}
			store_var(name, STACK_TV_BOT(0));
		    }
		    else
		    {
			if (iptr->isn_type == ISN_STOREEXPORT)
			{
			    int idx = get_script_item_idx(sid, name, 0,
								   NULL, NULL);

			    // can this ever fail?
			    if (idx >= 0)
			    {
				svar_T	*sv = ((svar_T *)SCRIPT_ITEM(sid)
						  ->sn_var_vals.ga_data) + idx;

				if ((sv->sv_flags & SVFLAG_EXPORTED) == 0)
				{
				    semsg(_(e_item_not_exported_in_script_str),
									 name);
				    clear_tv(STACK_TV_BOT(0));
				    goto on_error;
				}
			    }
			}
			if (var_check_permission(di, name) == FAIL)
			{
			    clear_tv(STACK_TV_BOT(0));
			    goto on_error;
			}
			clear_tv(&di->di_tv);
			di->di_tv = *STACK_TV_BOT(0);
		    }
		}
		break;

	    // store script-local variable in Vim9 script
	    case ISN_STORESCRIPT:
		{
		    scriptref_T	    *sref = iptr->isn_arg.script.scriptref;
		    svar_T	    *sv;

		    sv = get_script_svar(sref, ectx->ec_dfunc_idx);
		    if (sv == NULL)
			goto theend;
		    --ectx->ec_stack.ga_len;

		    // "const" and "final" are checked at compile time, locking
		    // the value needs to be checked here.
		    SOURCING_LNUM = iptr->isn_lnum;
		    if (value_check_lock(sv->sv_tv->v_lock, sv->sv_name, FALSE))
		    {
			clear_tv(STACK_TV_BOT(0));
			goto on_error;
		    }

		    clear_tv(sv->sv_tv);
		    *sv->sv_tv = *STACK_TV_BOT(0);
		}
		break;

	    // store option
	    case ISN_STOREOPT:
	    case ISN_STOREFUNCOPT:
		{
		    char_u	*opt_name = iptr->isn_arg.storeopt.so_name;
		    int		opt_flags = iptr->isn_arg.storeopt.so_flags;
		    long	n = 0;
		    char_u	*s = NULL;
		    char	*msg;
		    char_u	numbuf[NUMBUFLEN];
		    char_u	*tofree = NULL;

		    --ectx->ec_stack.ga_len;
		    SOURCING_LNUM = iptr->isn_lnum;
		    tv = STACK_TV_BOT(0);
		    if (tv->v_type == VAR_STRING)
		    {
			s = tv->vval.v_string;
			if (s == NULL)
			    s = (char_u *)"";
		    }
		    else if (iptr->isn_type == ISN_STOREFUNCOPT)
		    {
			// If the option can be set to a function reference or
			// a lambda and the passed value is a function
			// reference, then convert it to the name (string) of
			// the function reference.
			s = tv2string(tv, &tofree, numbuf, 0);
			if (s == NULL || *s == NUL)
			{
			    // cannot happen?
			    clear_tv(tv);
			    vim_free(tofree);
			    goto on_error;
			}
		    }
		    else
			// must be VAR_NUMBER, CHECKTYPE makes sure
			n = tv->vval.v_number;
		    msg = set_option_value(opt_name, n, s, opt_flags);
		    clear_tv(tv);
		    vim_free(tofree);
		    if (msg != NULL)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			emsg(_(msg));
			goto on_error;
		    }
		}
		break;

	    // store $ENV
	    case ISN_STOREENV:
		--ectx->ec_stack.ga_len;
		tv = STACK_TV_BOT(0);
		vim_setenv_ext(iptr->isn_arg.string, tv_get_string(tv));
		clear_tv(tv);
		break;

	    // store @r
	    case ISN_STOREREG:
		{
		    int	reg = iptr->isn_arg.number;

		    --ectx->ec_stack.ga_len;
		    tv = STACK_TV_BOT(0);
		    write_reg_contents(reg, tv_get_string(tv), -1, FALSE);
		    clear_tv(tv);
		}
		break;

	    // store v: variable
	    case ISN_STOREV:
		--ectx->ec_stack.ga_len;
		if (set_vim_var_tv(iptr->isn_arg.number, STACK_TV_BOT(0))
								       == FAIL)
		    // should not happen, type is checked when compiling
		    goto on_error;
		break;

	    // store g:/b:/w:/t: variable
	    case ISN_STOREG:
	    case ISN_STOREB:
	    case ISN_STOREW:
	    case ISN_STORET:
		{
		    dictitem_T	*di;
		    hashtab_T	*ht;
		    char_u	*name = iptr->isn_arg.string + 2;

		    switch (iptr->isn_type)
		    {
			case ISN_STOREG:
			    ht = get_globvar_ht();
			    break;
			case ISN_STOREB:
			    ht = &curbuf->b_vars->dv_hashtab;
			    break;
			case ISN_STOREW:
			    ht = &curwin->w_vars->dv_hashtab;
			    break;
			case ISN_STORET:
			    ht = &curtab->tp_vars->dv_hashtab;
			    break;
			default:  // Cannot reach here
			    goto theend;
		    }

		    --ectx->ec_stack.ga_len;
		    di = find_var_in_ht(ht, 0, name, TRUE);
		    if (di == NULL)
			store_var(iptr->isn_arg.string, STACK_TV_BOT(0));
		    else
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			if (var_check_permission(di, name) == FAIL)
			    goto on_error;
			clear_tv(&di->di_tv);
			di->di_tv = *STACK_TV_BOT(0);
		    }
		}
		break;

	    // store an autoload variable
	    case ISN_STOREAUTO:
		SOURCING_LNUM = iptr->isn_lnum;
		set_var(iptr->isn_arg.string, STACK_TV_BOT(-1), TRUE);
		clear_tv(STACK_TV_BOT(-1));
		--ectx->ec_stack.ga_len;
		break;

	    // store number in local variable
	    case ISN_STORENR:
		tv = STACK_TV_VAR(iptr->isn_arg.storenr.stnr_idx);
		clear_tv(tv);
		tv->v_type = VAR_NUMBER;
		tv->vval.v_number = iptr->isn_arg.storenr.stnr_val;
		break;

	    // Store a value in a list, dict, blob or object variable.
	    case ISN_STOREINDEX:
		{
		    int res = execute_storeindex(iptr, ectx);

		    if (res == FAIL)
			goto on_error;
		    if (res == NOTDONE)
			goto theend;
		}
		break;

	    // store value in list or blob range
	    case ISN_STORERANGE:
		if (execute_storerange(iptr, ectx) == FAIL)
		    goto on_error;
		break;

	    case ISN_LOAD_CLASSMEMBER:
		{
		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    classmember_T *cm = &iptr->isn_arg.classmember;
		    copy_tv(cm->cm_class->class_members_tv + cm->cm_idx,
							      STACK_TV_BOT(0));
		    ++ectx->ec_stack.ga_len;
		}
		break;

	    case ISN_STORE_CLASSMEMBER:
		{
		    classmember_T *cm = &iptr->isn_arg.classmember;
		    tv = &cm->cm_class->class_members_tv[cm->cm_idx];
		    clear_tv(tv);
		    *tv = *STACK_TV_BOT(-1);
		    --ectx->ec_stack.ga_len;
		}
		break;

	    // Load or store variable or argument from outer scope.
	    case ISN_LOADOUTER:
	    case ISN_STOREOUTER:
		{
		    int		depth = iptr->isn_arg.outer.outer_depth;
		    outer_T	*outer = ectx->ec_outer_ref == NULL ? NULL
						: ectx->ec_outer_ref->or_outer;

		    while (depth > 1 && outer != NULL)
		    {
			outer = outer->out_up;
			--depth;
		    }
		    if (outer == NULL)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			if (ectx->ec_frame_idx == ectx->ec_initial_frame_idx
						 || ectx->ec_outer_ref == NULL)
			    // Possibly :def function called from legacy
			    // context.
			    emsg(_(e_closure_called_from_invalid_context));
			else
			    iemsg("LOADOUTER depth more than scope levels");
			goto theend;
		    }
		    if (depth < 0)
			// Variable declared in loop.  May be copied if the
			// loop block has already ended.
			tv = ((typval_T *)outer->out_loop[-depth - 1]
							       .stack->ga_data)
					  + outer->out_loop[-depth - 1].var_idx
					  + iptr->isn_arg.outer.outer_idx;
		    else
			// Variable declared in a function.  May be copied if
			// the function has already returned.
			tv = ((typval_T *)outer->out_stack->ga_data)
				      + outer->out_frame_idx + STACK_FRAME_SIZE
				      + iptr->isn_arg.outer.outer_idx;
		    if (iptr->isn_type == ISN_LOADOUTER)
		    {
			typval_T *copy;
			if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			    goto theend;
			// careful: ga_grow_inner may re-alloc the stack
			if (depth < 0)
			    copy = ((typval_T *)outer->out_loop[-depth - 1]
								   .stack->ga_data)
					      + outer->out_loop[-depth - 1].var_idx
					      + iptr->isn_arg.outer.outer_idx;
			else
			    copy = ((typval_T *)outer->out_stack->ga_data)
					  + outer->out_frame_idx + STACK_FRAME_SIZE
					  + iptr->isn_arg.outer.outer_idx;
			// memory was freed, get tv again
			if (copy != tv)
			    tv = copy;
			copy_tv(tv, STACK_TV_BOT(0));
			++ectx->ec_stack.ga_len;
		    }
		    else
		    {
			--ectx->ec_stack.ga_len;
			clear_tv(tv);
			*tv = *STACK_TV_BOT(0);
		    }
		}
		break;

	    // unlet item in list or dict variable
	    case ISN_UNLETINDEX:
		if (execute_unletindex(iptr, ectx) == FAIL)
		    goto on_error;
		break;

	    // unlet range of items in list variable
	    case ISN_UNLETRANGE:
		if (execute_unletrange(iptr, ectx) == FAIL)
		    goto on_error;
		break;

	    // push constant
	    case ISN_PUSHNR:
	    case ISN_PUSHBOOL:
	    case ISN_PUSHSPEC:
	    case ISN_PUSHF:
	    case ISN_PUSHS:
	    case ISN_PUSHBLOB:
	    case ISN_PUSHFUNC:
	    case ISN_PUSHCHANNEL:
	    case ISN_PUSHJOB:
	    case ISN_PUSHOBJ:
	    case ISN_PUSHCLASS:
		if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    goto theend;
		tv = STACK_TV_BOT(0);
		tv->v_lock = 0;
		++ectx->ec_stack.ga_len;
		switch (iptr->isn_type)
		{
		    case ISN_PUSHNR:
			tv->v_type = VAR_NUMBER;
			tv->vval.v_number = iptr->isn_arg.number;
			break;
		    case ISN_PUSHBOOL:
			tv->v_type = VAR_BOOL;
			tv->vval.v_number = iptr->isn_arg.number;
			break;
		    case ISN_PUSHSPEC:
			tv->v_type = VAR_SPECIAL;
			tv->vval.v_number = iptr->isn_arg.number;
			break;
		    case ISN_PUSHF:
			tv->v_type = VAR_FLOAT;
			tv->vval.v_float = iptr->isn_arg.fnumber;
			break;
		    case ISN_PUSHBLOB:
			blob_copy(iptr->isn_arg.blob, tv);
			break;
		    case ISN_PUSHFUNC:
			tv->v_type = VAR_FUNC;
			if (iptr->isn_arg.string == NULL)
			    tv->vval.v_string = NULL;
			else
			    tv->vval.v_string =
					     vim_strsave(iptr->isn_arg.string);
			break;
		    case ISN_PUSHCHANNEL:
#ifdef FEAT_JOB_CHANNEL
			tv->v_type = VAR_CHANNEL;
			tv->vval.v_channel = NULL;
#endif
			break;
		    case ISN_PUSHJOB:
#ifdef FEAT_JOB_CHANNEL
			tv->v_type = VAR_JOB;
			tv->vval.v_job = NULL;
#endif
			break;
		    case ISN_PUSHOBJ:
			tv->v_type = VAR_OBJECT;
			tv->vval.v_object = NULL;
			break;
		    case ISN_PUSHCLASS:
			tv->v_type = VAR_CLASS;
			tv->vval.v_class = iptr->isn_arg.classarg;
			break;
		    default:
			tv->v_type = VAR_STRING;
			tv->vval.v_string = iptr->isn_arg.string == NULL
				    ? NULL : vim_strsave(iptr->isn_arg.string);
		}
		break;

	    case ISN_AUTOLOAD:
		{
		    char_u  *name = iptr->isn_arg.string;

		    (void)script_autoload(name, FALSE);
		    if (find_func(name, TRUE))
		    {
			if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			    goto theend;
			tv = STACK_TV_BOT(0);
			tv->v_lock = 0;
			++ectx->ec_stack.ga_len;
			tv->v_type = VAR_FUNC;
			tv->vval.v_string = vim_strsave(name);
		    }
		    else
		    {
			int res = load_namespace_var(ectx, ISN_LOADG, iptr);

			if (res == NOTDONE)
			    goto theend;
			if (res == FAIL)
			    goto on_error;
		    }
		}
		break;

	    case ISN_UNLET:
		if (do_unlet(iptr->isn_arg.unlet.ul_name,
				       iptr->isn_arg.unlet.ul_forceit) == FAIL)
		    goto on_error;
		break;
	    case ISN_UNLETENV:
		vim_unsetenv_ext(iptr->isn_arg.unlet.ul_name);
		break;

	    case ISN_LOCKUNLOCK:
		{
#ifdef LOG_LOCKVAR
		    ch_log(NULL, "LKVAR: execute INS_LOCKUNLOCK isn_arg %s",
							iptr->isn_arg.string);
#endif
		    lval_root_T	*lval_root_save = lval_root;

		    // Stack has the local variable, argument the whole :lock
		    // or :unlock command, like ISN_EXEC.
		    --ectx->ec_stack.ga_len;
		    lval_root_T root = { .lr_tv = STACK_TV_BOT(0),
			    .lr_cl_exec = iptr->isn_arg.lockunlock.lu_cl_exec,
			    .lr_is_arg  = iptr->isn_arg.lockunlock.lu_is_arg };
		    lval_root = &root;
		    int res = exec_command(iptr,
					iptr->isn_arg.lockunlock.lu_string);
		    clear_tv(root.lr_tv);
		    lval_root = lval_root_save;
		    if (res == FAIL)
			goto on_error;
		}
		break;

	    case ISN_LOCKCONST:
		item_lock(STACK_TV_BOT(-1), 100, TRUE, TRUE);
		break;

	    // create a list from items on the stack; uses a single allocation
	    // for the list header and the items
	    case ISN_NEWLIST:
		if (exe_newlist(iptr->isn_arg.number, ectx) == FAIL)
		    goto theend;
		break;

	    // create a dict from items on the stack
	    case ISN_NEWDICT:
		{
		    int res;

		    SOURCING_LNUM = iptr->isn_lnum;
		    res = exe_newdict(iptr->isn_arg.number, ectx);
		    if (res == FAIL)
			goto theend;
		    if (res == MAYBE)
			goto on_error;
		}
		break;

	    case ISN_CONCAT:
		if (exe_concat(iptr->isn_arg.number, ectx) == FAIL)
		    goto theend;
		break;

	    // create a partial with NULL value
	    case ISN_NEWPARTIAL:
		if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    goto theend;
		++ectx->ec_stack.ga_len;
		tv = STACK_TV_BOT(-1);
		tv->v_type = VAR_PARTIAL;
		tv->v_lock = 0;
		tv->vval.v_partial = NULL;
		break;

	    // call a :def function
	    case ISN_DCALL:
		SOURCING_LNUM = iptr->isn_lnum;
		if (call_dfunc(iptr->isn_arg.dfunc.cdf_idx,
				NULL,
				iptr->isn_arg.dfunc.cdf_argcount,
				ectx) == FAIL)
		    goto on_error;
		break;

	    // call a method on an interface
	    case ISN_METHODCALL:
		{
		    cmfunc_T *mfunc = iptr->isn_arg.mfunc;

		    SOURCING_LNUM = iptr->isn_lnum;
		    tv = STACK_TV_BOT(-1 - mfunc->cmf_argcount);
		    if (tv->v_type != VAR_OBJECT)
		    {
			object_required_error(tv);
			goto on_error;
		    }
		    object_T *obj = tv->vval.v_object;
		    class_T *cl = obj->obj_class;

		    // convert the interface index to the object index
		    int idx = object_index_from_itf_index(mfunc->cmf_itf,
						    TRUE, mfunc->cmf_idx, cl);

		    if (call_ufunc(cl->class_obj_methods[idx], NULL,
				mfunc->cmf_argcount, ectx, NULL, NULL) == FAIL)
			goto on_error;
		}
		break;

	    // call a builtin function
	    case ISN_BCALL:
		SOURCING_LNUM = iptr->isn_lnum;
		if (call_bfunc(iptr->isn_arg.bfunc.cbf_idx,
			      iptr->isn_arg.bfunc.cbf_argcount,
			      ectx) == FAIL)
		    goto on_error;
		break;

	    // call a funcref or partial
	    case ISN_PCALL:
		{
		    cpfunc_T	*pfunc = &iptr->isn_arg.pfunc;
		    int		r;
		    typval_T	partial_tv;

		    SOURCING_LNUM = iptr->isn_lnum;
		    if (pfunc->cpf_top)
		    {
			// funcref is above the arguments
			tv = STACK_TV_BOT(-pfunc->cpf_argcount - 1);
		    }
		    else
		    {
			// Get the funcref from the stack.
			--ectx->ec_stack.ga_len;
			partial_tv = *STACK_TV_BOT(0);
			tv = &partial_tv;
		    }
		    r = call_partial(tv, pfunc->cpf_argcount, ectx);
		    if (tv == &partial_tv)
			clear_tv(&partial_tv);
		    if (r == FAIL)
			goto on_error;
		}
		break;

	    case ISN_PCALL_END:
		// PCALL finished, arguments have been consumed and replaced by
		// the return value.  Now clear the funcref from the stack,
		// and move the return value in its place.
		--ectx->ec_stack.ga_len;
		clear_tv(STACK_TV_BOT(-1));
		*STACK_TV_BOT(-1) = *STACK_TV_BOT(0);
		break;

	    // call a user defined function or funcref/partial
	    case ISN_UCALL:
		{
		    cufunc_T	*cufunc = &iptr->isn_arg.ufunc;

		    SOURCING_LNUM = iptr->isn_lnum;
		    if (call_eval_func(cufunc->cuf_name, cufunc->cuf_argcount,
							   ectx, iptr) == FAIL)
			goto on_error;
		}
		break;

	    // :defer func(arg)
	    case ISN_DEFER:
		if (defer_command(iptr->isn_arg.defer.defer_var_idx,
			     iptr->isn_arg.defer.defer_argcount, ectx) == FAIL)
		    goto on_error;
		break;

	    // Return from a :def function call without a value.
	    // Return from a constructor.
	    case ISN_RETURN_VOID:
	    case ISN_RETURN_OBJECT:
		if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    goto theend;
		tv = STACK_TV_BOT(0);
		++ectx->ec_stack.ga_len;
		if (iptr->isn_type == ISN_RETURN_VOID)
		{
		    tv->v_type = VAR_VOID;
		    tv->vval.v_number = 0;
		    tv->v_lock = 0;
		}
		else
		{
		    *tv = *STACK_TV_VAR(0);
		    object_T *obj = tv->vval.v_object;
		    ++obj->obj_refcount;

		    // Lock all the constant object variables
		    obj_lock_const_vars(obj);
		}
		// FALLTHROUGH

	    // return from a :def function call with what is on the stack
	    case ISN_RETURN:
		{
		    garray_T	*trystack = &ectx->ec_trystack;
		    trycmd_T    *trycmd = NULL;

		    ///////////////////////////////////////////////////
		    // TODO: If FAIL, line number in output not correct
		    ///////////////////////////////////////////////////
		    if (check_typval_is_value(STACK_TV_BOT(-1)) == FAIL)
			goto theend;

		    if (trystack->ga_len > 0)
			trycmd = ((trycmd_T *)trystack->ga_data)
							+ trystack->ga_len - 1;
		    if (trycmd != NULL
				 && trycmd->tcd_frame_idx == ectx->ec_frame_idx)
		    {
			// jump to ":finally" or ":endtry"
			if (trycmd->tcd_finally_idx != 0)
			    ectx->ec_iidx = trycmd->tcd_finally_idx;
			else
			    ectx->ec_iidx = trycmd->tcd_endtry_idx;
			trycmd->tcd_return = TRUE;
		    }
		    else
			goto func_return;
		}
		break;

	    // push a partial, a reference to a compiled function
	    case ISN_FUNCREF:
		{
		    partial_T	    *pt = ALLOC_CLEAR_ONE(partial_T);
		    ufunc_T	    *ufunc;
		    funcref_T	    *funcref = &iptr->isn_arg.funcref;
		    funcref_extra_T *extra = funcref->fr_extra;

		    if (pt == NULL)
			goto theend;
		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    {
			vim_free(pt);
			goto theend;
		    }
		    if (extra != NULL && extra->fre_class != NULL)
		    {
			class_T	*cl;
			if (extra->fre_object_method)
			{
			    tv = STACK_TV_BOT(-1);
			    if (tv->v_type != VAR_OBJECT)
			    {
				object_required_error(tv);
				vim_free(pt);
				goto on_error;
			    }

			    object_T *obj = tv->vval.v_object;
			    cl = obj->obj_class;
			    // drop the value from the stack
			    clear_tv(tv);
			    --ectx->ec_stack.ga_len;

			    pt->pt_obj = obj;
			    ++obj->obj_refcount;
			}
			else
			    cl = extra->fre_class;

			if (extra->fre_object_method)
			{
			    // object method
			    // convert the interface index to the object index
			    int idx =
				object_index_from_itf_index(extra->fre_class,
					TRUE, extra->fre_method_idx, cl);
			    ufunc = cl->class_obj_methods[idx];
			}
			else
			{
			    // class method
			    ufunc =
				cl->class_class_functions[extra->fre_method_idx];
			}
		    }
		    else if (extra == NULL || extra->fre_func_name == NULL)
		    {
			dfunc_T	*pt_dfunc = ((dfunc_T *)def_functions.ga_data)
						       + funcref->fr_dfunc_idx;

			ufunc = pt_dfunc->df_ufunc;
		    }
		    else
		    {
			ufunc = find_func(extra->fre_func_name, FALSE);
		    }
		    if (ufunc == NULL)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			iemsg("ufunc unexpectedly NULL for FUNCREF");
			goto theend;
		    }
		    if (fill_partial_and_closure(pt, ufunc,
			       extra == NULL ? NULL : &extra->fre_loopvar_info,
								 ectx) == FAIL)
			goto theend;
		    tv = STACK_TV_BOT(0);
		    ++ectx->ec_stack.ga_len;
		    tv->vval.v_partial = pt;
		    tv->v_type = VAR_PARTIAL;
		    tv->v_lock = 0;
		}
		break;

	    // Create a global function from a lambda.
	    case ISN_NEWFUNC:
		{
		    newfuncarg_T    *arg = iptr->isn_arg.newfunc.nf_arg;

		    if (copy_lambda_to_global_func(arg->nfa_lambda,
				       arg->nfa_global, &arg->nfa_loopvar_info,
				       ectx) == FAIL)
			goto theend;
		}
		break;

	    // List functions
	    case ISN_DEF:
		if (iptr->isn_arg.string == NULL)
		    list_functions(NULL);
		else
		{
		    exarg_T	ea;
		    garray_T	lines_to_free;

		    CLEAR_FIELD(ea);
		    ea.cmd = ea.arg = iptr->isn_arg.string;
		    ga_init2(&lines_to_free, sizeof(char_u *), 50);
		    SOURCING_LNUM = iptr->isn_lnum;
		    define_function(&ea, NULL, &lines_to_free, 0, NULL, 0);
		    ga_clear_strings(&lines_to_free);
		}
		break;

	    // jump if a condition is met
	    case ISN_JUMP:
		{
		    jumpwhen_T	when = iptr->isn_arg.jump.jump_when;
		    int		error = FALSE;
		    int		jump = TRUE;

		    if (when != JUMP_ALWAYS)
		    {
			tv = STACK_TV_BOT(-1);
			if (when == JUMP_IF_COND_FALSE
				|| when == JUMP_IF_FALSE
				|| when == JUMP_IF_COND_TRUE)
			{
			    SOURCING_LNUM = iptr->isn_lnum;
			    jump = tv_get_bool_chk(tv, &error);
			    if (error)
				goto on_error;
			}
			else
			    jump = tv2bool(tv);
			if (when == JUMP_IF_FALSE || when == JUMP_IF_COND_FALSE)
			    jump = !jump;
			if (when == JUMP_IF_FALSE || !jump)
			{
			    // drop the value from the stack
			    clear_tv(tv);
			    --ectx->ec_stack.ga_len;
			}
		    }
		    if (jump)
			ectx->ec_iidx = iptr->isn_arg.jump.jump_where;
		}
		break;

	    // "while": jump to end if a condition is false
	    case ISN_WHILE:
		{
		    int		error = FALSE;
		    int		jump = TRUE;

		    tv = STACK_TV_BOT(-1);
		    SOURCING_LNUM = iptr->isn_lnum;
		    jump = !tv_get_bool_chk(tv, &error);
		    if (error)
			goto on_error;
		    // drop the value from the stack
		    clear_tv(tv);
		    --ectx->ec_stack.ga_len;
		    if (jump)
			ectx->ec_iidx = iptr->isn_arg.whileloop.while_end;

		    // Store the current funcref count, may be used by
		    // ISN_ENDLOOP later
		    tv = STACK_TV_VAR(
				    iptr->isn_arg.whileloop.while_funcref_idx);
		    tv->vval.v_number = ectx->ec_funcrefs.ga_len;
		}
		break;

	    // Jump if an argument with a default value was already set and not
	    // v:none.
	    case ISN_JUMP_IF_ARG_SET:
	    case ISN_JUMP_IF_ARG_NOT_SET:
		tv = STACK_TV_VAR(iptr->isn_arg.jumparg.jump_arg_off);
		int arg_set = tv->v_type != VAR_UNKNOWN
				&& !(tv->v_type == VAR_SPECIAL
					    && tv->vval.v_number == VVAL_NONE);
		if (iptr->isn_type == ISN_JUMP_IF_ARG_SET ? arg_set : !arg_set)
		    ectx->ec_iidx = iptr->isn_arg.jumparg.jump_where;
		break;

	    // top of a for loop
	    case ISN_FOR:
		if (execute_for(iptr, ectx) == FAIL)
		    goto theend;
		break;

	    // end of a for or while loop
	    case ISN_ENDLOOP:
		if (execute_endloop(iptr, ectx) == FAIL)
		    goto theend;
		break;

	    // start of ":try" block
	    case ISN_TRY:
		{
		    trycmd_T    *trycmd = NULL;

		    if (GA_GROW_FAILS(&ectx->ec_trystack, 1))
			goto theend;
		    trycmd = ((trycmd_T *)ectx->ec_trystack.ga_data)
						     + ectx->ec_trystack.ga_len;
		    ++ectx->ec_trystack.ga_len;
		    ++trylevel;
		    CLEAR_POINTER(trycmd);
		    trycmd->tcd_frame_idx = ectx->ec_frame_idx;
		    trycmd->tcd_stack_len = ectx->ec_stack.ga_len;
		    trycmd->tcd_catch_idx =
				       iptr->isn_arg.tryref.try_ref->try_catch;
		    trycmd->tcd_finally_idx =
				     iptr->isn_arg.tryref.try_ref->try_finally;
		    trycmd->tcd_endtry_idx =
				      iptr->isn_arg.tryref.try_ref->try_endtry;
		}
		break;

	    case ISN_PUSHEXC:
		if (current_exception == NULL)
		{
		    SOURCING_LNUM = iptr->isn_lnum;
		    iemsg("Evaluating catch while current_exception is NULL");
		    goto theend;
		}
		if (GA_GROW_FAILS(&ectx->ec_stack, 1))
		    goto theend;
		tv = STACK_TV_BOT(0);
		++ectx->ec_stack.ga_len;
		tv->v_type = VAR_STRING;
		tv->v_lock = 0;
		tv->vval.v_string = vim_strsave(
					   (char_u *)current_exception->value);
		break;

	    case ISN_CATCH:
		{
		    garray_T	*trystack = &ectx->ec_trystack;
		    trycmd_T    *trycmd;

		    may_restore_cmdmod(&ectx->ec_funclocal);
		    trycmd = ((trycmd_T *)trystack->ga_data)
							+ trystack->ga_len - 1;
		    trycmd->tcd_caught = TRUE;
		    trycmd->tcd_did_throw = FALSE;

		    did_emsg = got_int = did_throw = FALSE;
		    force_abort = need_rethrow = FALSE;
		    catch_exception(current_exception);
		}
		break;

	    case ISN_TRYCONT:
		{
		    garray_T	*trystack = &ectx->ec_trystack;
		    trycont_T	*trycont = &iptr->isn_arg.trycont;
		    int		i;
		    trycmd_T    *trycmd;
		    int		iidx = trycont->tct_where;

		    if (trystack->ga_len < trycont->tct_levels)
		    {
			siemsg("TRYCONT: expected %d levels, found %d",
					trycont->tct_levels, trystack->ga_len);
			goto theend;
		    }
		    // Make :endtry jump to any outer try block and the last
		    // :endtry inside the loop to the loop start.
		    for (i = trycont->tct_levels; i > 0; --i)
		    {
			trycmd = ((trycmd_T *)trystack->ga_data)
							+ trystack->ga_len - i;
			// Add one to tcd_cont to be able to jump to
			// instruction with index zero.
			trycmd->tcd_cont = iidx + 1;
			iidx = trycmd->tcd_finally_idx == 0
			    ? trycmd->tcd_endtry_idx : trycmd->tcd_finally_idx;
		    }
		    // jump to :finally or :endtry of current try statement
		    ectx->ec_iidx = iidx;
		}
		break;

	    case ISN_FINALLY:
		{
		    garray_T	*trystack = &ectx->ec_trystack;
		    trycmd_T    *trycmd = ((trycmd_T *)trystack->ga_data)
							+ trystack->ga_len - 1;

		    // Reset the index to avoid a return statement jumps here
		    // again.
		    trycmd->tcd_finally_idx = 0;
		    break;
		}

	    // end of ":try" block
	    case ISN_ENDTRY:
		{
		    garray_T	*trystack = &ectx->ec_trystack;
		    trycmd_T    *trycmd;

		    --trystack->ga_len;
		    --trylevel;
		    trycmd = ((trycmd_T *)trystack->ga_data) + trystack->ga_len;
		    if (trycmd->tcd_did_throw)
			did_throw = TRUE;
		    if (trycmd->tcd_caught && current_exception != NULL)
		    {
			// discard the exception
			if (caught_stack == current_exception)
			    caught_stack = caught_stack->caught;
			discard_current_exception();
		    }

		    if (trycmd->tcd_return)
			goto func_return;

		    while (ectx->ec_stack.ga_len > trycmd->tcd_stack_len)
		    {
			--ectx->ec_stack.ga_len;
			clear_tv(STACK_TV_BOT(0));
		    }
		    if (trycmd->tcd_cont != 0)
			// handling :continue: jump to outer try block or
			// start of the loop
			ectx->ec_iidx = trycmd->tcd_cont - 1;
		}
		break;

	    case ISN_THROW:
		{
		    garray_T	*trystack = &ectx->ec_trystack;

		    if (trystack->ga_len == 0 && trylevel == 0 && emsg_silent)
		    {
			// Throwing an exception while using "silent!" causes
			// the function to abort but not display an error.
			tv = STACK_TV_BOT(-1);
			clear_tv(tv);
			tv->v_type = VAR_NUMBER;
			tv->vval.v_number = 0;
			goto done;
		    }
		    --ectx->ec_stack.ga_len;
		    tv = STACK_TV_BOT(0);
		    if (tv->vval.v_string == NULL
				       || *skipwhite(tv->vval.v_string) == NUL)
		    {
			vim_free(tv->vval.v_string);
			SOURCING_LNUM = iptr->isn_lnum;
			emsg(_(e_throw_with_empty_string));
			goto theend;
		    }

		    // Inside a "catch" we need to first discard the caught
		    // exception.
		    if (trystack->ga_len > 0)
		    {
			trycmd_T    *trycmd = ((trycmd_T *)trystack->ga_data)
							+ trystack->ga_len - 1;
			if (trycmd->tcd_caught && current_exception != NULL)
			{
			    // discard the exception
			    if (caught_stack == current_exception)
				caught_stack = caught_stack->caught;
			    discard_current_exception();
			    trycmd->tcd_caught = FALSE;
			}
		    }

		    SOURCING_LNUM = iptr->isn_lnum;
		    if (throw_exception(tv->vval.v_string, ET_USER, NULL)
								       == FAIL)
		    {
			vim_free(tv->vval.v_string);
			goto theend;
		    }
		    did_throw = TRUE;
		}
		break;

	    // compare with special values
	    case ISN_COMPAREBOOL:
	    case ISN_COMPARESPECIAL:
		{
		    typval_T	*tv1 = STACK_TV_BOT(-2);
		    typval_T	*tv2 = STACK_TV_BOT(-1);
		    varnumber_T arg1 = tv1->vval.v_number;
		    varnumber_T arg2 = tv2->vval.v_number;
		    int		res;

		    if (iptr->isn_arg.op.op_type == EXPR_EQUAL)
			res = arg1 == arg2;
		    else
			res = arg1 != arg2;

		    --ectx->ec_stack.ga_len;
		    tv1->v_type = VAR_BOOL;
		    tv1->vval.v_number = res ? VVAL_TRUE : VVAL_FALSE;
		}
		break;

	    case ISN_COMPARENULL:
		{
		    typval_T	*tv1 = STACK_TV_BOT(-2);
		    typval_T	*tv2 = STACK_TV_BOT(-1);
		    int		res;

		    res = typval_compare_null(tv1, tv2);
		    if (res == MAYBE)
			goto on_error;
		    if (iptr->isn_arg.op.op_type == EXPR_NEQUAL)
			res = !res;
		    clear_tv(tv1);
		    clear_tv(tv2);
		    --ectx->ec_stack.ga_len;
		    tv1->v_type = VAR_BOOL;
		    tv1->vval.v_number = res ? VVAL_TRUE : VVAL_FALSE;
		}
		break;

	    // Operation with two number arguments
	    case ISN_OPNR:
	    case ISN_COMPARENR:
		{
		    typval_T	*tv1 = STACK_TV_BOT(-2);
		    typval_T	*tv2 = STACK_TV_BOT(-1);
		    varnumber_T arg1 = tv1->vval.v_number;
		    varnumber_T arg2 = tv2->vval.v_number;
		    varnumber_T res = 0;
		    int		div_zero = FALSE;

		    if (iptr->isn_arg.op.op_type == EXPR_LSHIFT
			    || iptr->isn_arg.op.op_type == EXPR_RSHIFT)
		    {
			if (arg2 < 0)
			{
			    SOURCING_LNUM = iptr->isn_lnum;
			    emsg(_(e_bitshift_ops_must_be_positive));
			    goto on_error;
			}
		    }

		    switch (iptr->isn_arg.op.op_type)
		    {
			case EXPR_MULT: res = arg1 * arg2; break;
			case EXPR_DIV:  if (arg2 == 0)
					    div_zero = TRUE;
					else
					    res = arg1 / arg2;
					break;
			case EXPR_REM:  if (arg2 == 0)
					    div_zero = TRUE;
					else
					    res = arg1 % arg2;
					break;
			case EXPR_SUB: res = arg1 - arg2; break;
			case EXPR_ADD: res = arg1 + arg2; break;

			case EXPR_EQUAL: res = arg1 == arg2; break;
			case EXPR_NEQUAL: res = arg1 != arg2; break;
			case EXPR_GREATER: res = arg1 > arg2; break;
			case EXPR_GEQUAL: res = arg1 >= arg2; break;
			case EXPR_SMALLER: res = arg1 < arg2; break;
			case EXPR_SEQUAL: res = arg1 <= arg2; break;
			case EXPR_LSHIFT: if (arg2 > MAX_LSHIFT_BITS)
					      res = 0;
					  else
					      res = (uvarnumber_T)arg1 << arg2;
					  break;
			case EXPR_RSHIFT: if (arg2 > MAX_LSHIFT_BITS)
					      res = 0;
					  else
					      res = (uvarnumber_T)arg1 >> arg2;
					  break;
			default: break;
		    }

		    --ectx->ec_stack.ga_len;
		    if (iptr->isn_type == ISN_COMPARENR)
		    {
			tv1->v_type = VAR_BOOL;
			tv1->vval.v_number = res ? VVAL_TRUE : VVAL_FALSE;
		    }
		    else
			tv1->vval.v_number = res;
		    if (div_zero)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			emsg(_(e_divide_by_zero));
			goto on_error;
		    }
		}
		break;

	    // Computation with two float arguments
	    case ISN_OPFLOAT:
	    case ISN_COMPAREFLOAT:
		{
		    typval_T	*tv1 = STACK_TV_BOT(-2);
		    typval_T	*tv2 = STACK_TV_BOT(-1);
		    float_T	arg1 = tv1->vval.v_float;
		    float_T	arg2 = tv2->vval.v_float;
		    float_T	res = 0;
		    int		cmp = FALSE;

		    switch (iptr->isn_arg.op.op_type)
		    {
			case EXPR_MULT: res = arg1 * arg2; break;
			case EXPR_DIV: res = arg1 / arg2; break;
			case EXPR_SUB: res = arg1 - arg2; break;
			case EXPR_ADD: res = arg1 + arg2; break;

			case EXPR_EQUAL: cmp = arg1 == arg2; break;
			case EXPR_NEQUAL: cmp = arg1 != arg2; break;
			case EXPR_GREATER: cmp = arg1 > arg2; break;
			case EXPR_GEQUAL: cmp = arg1 >= arg2; break;
			case EXPR_SMALLER: cmp = arg1 < arg2; break;
			case EXPR_SEQUAL: cmp = arg1 <= arg2; break;
			default: cmp = 0; break;
		    }
		    --ectx->ec_stack.ga_len;
		    if (iptr->isn_type == ISN_COMPAREFLOAT)
		    {
			tv1->v_type = VAR_BOOL;
			tv1->vval.v_number = cmp ? VVAL_TRUE : VVAL_FALSE;
		    }
		    else
			tv1->vval.v_float = res;
		}
		break;

	    case ISN_COMPARELIST:
	    case ISN_COMPAREDICT:
	    case ISN_COMPAREFUNC:
	    case ISN_COMPARESTRING:
	    case ISN_COMPAREBLOB:
	    case ISN_COMPAREOBJECT:
		{
		    typval_T	*tv1 = STACK_TV_BOT(-2);
		    typval_T	*tv2 = STACK_TV_BOT(-1);
		    exprtype_T	exprtype = iptr->isn_arg.op.op_type;
		    int		ic = iptr->isn_arg.op.op_ic;
		    int		res = FALSE;
		    int		status = OK;

		    SOURCING_LNUM = iptr->isn_lnum;
		    if (iptr->isn_type == ISN_COMPARELIST)
		    {
			status = typval_compare_list(tv1, tv2,
							   exprtype, ic, &res);
		    }
		    else if (iptr->isn_type == ISN_COMPAREDICT)
		    {
			status = typval_compare_dict(tv1, tv2,
							   exprtype, ic, &res);
		    }
		    else if (iptr->isn_type == ISN_COMPAREFUNC)
		    {
			status = typval_compare_func(tv1, tv2,
							   exprtype, ic, &res);
		    }
		    else if (iptr->isn_type == ISN_COMPARESTRING)
		    {
			status = typval_compare_string(tv1, tv2,
							   exprtype, ic, &res);
		    }
		    else if (iptr->isn_type == ISN_COMPAREBLOB)
		    {
			status = typval_compare_blob(tv1, tv2, exprtype, &res);
		    }
		    else // ISN_COMPAREOBJECT
		    {
			status = typval_compare_object(tv1, tv2,
							exprtype, FALSE, &res);
		    }
		    --ectx->ec_stack.ga_len;
		    clear_tv(tv1);
		    clear_tv(tv2);
		    tv1->v_type = VAR_BOOL;
		    tv1->vval.v_number = res ? VVAL_TRUE : VVAL_FALSE;
		    if (status == FAIL)
			goto theend;
		}
		break;

	    case ISN_COMPAREANY:
		{
		    typval_T	*tv1 = STACK_TV_BOT(-2);
		    typval_T	*tv2 = STACK_TV_BOT(-1);
		    exprtype_T	exprtype = iptr->isn_arg.op.op_type;
		    int		ic = iptr->isn_arg.op.op_ic;
		    int		status;

		    SOURCING_LNUM = iptr->isn_lnum;
		    status = typval_compare(tv1, tv2, exprtype, ic);
		    clear_tv(tv2);
		    --ectx->ec_stack.ga_len;
		    if (status == FAIL)
			goto theend;
		}
		break;

	    case ISN_ADDLIST:
	    case ISN_ADDBLOB:
		{
		    typval_T *tv1 = STACK_TV_BOT(-2);
		    typval_T *tv2 = STACK_TV_BOT(-1);

		    // add two lists or blobs
		    if (iptr->isn_type == ISN_ADDLIST)
		    {
			if (iptr->isn_arg.op.op_type == EXPR_APPEND
						   && tv1->vval.v_list != NULL)
			    list_extend(tv1->vval.v_list, tv2->vval.v_list,
									 NULL);
			else
			    eval_addlist(tv1, tv2);
		    }
		    else
			eval_addblob(tv1, tv2);
		    clear_tv(tv2);
		    --ectx->ec_stack.ga_len;
		}
		break;

	    case ISN_LISTAPPEND:
		{
		    typval_T	*tv1 = STACK_TV_BOT(-2);
		    typval_T	*tv2 = STACK_TV_BOT(-1);
		    list_T	*l = tv1->vval.v_list;

		    // add an item to a list
		    SOURCING_LNUM = iptr->isn_lnum;
		    if (l == NULL)
		    {
			emsg(_(e_cannot_add_to_null_list));
			goto on_error;
		    }
		    if (value_check_lock(l->lv_lock, NULL, FALSE))
			goto on_error;
		    if (list_append_tv(l, tv2) == FAIL)
			goto theend;
		    clear_tv(tv2);
		    --ectx->ec_stack.ga_len;
		}
		break;

	    case ISN_BLOBAPPEND:
		{
		    typval_T	*tv1 = STACK_TV_BOT(-2);
		    typval_T	*tv2 = STACK_TV_BOT(-1);
		    blob_T	*b = tv1->vval.v_blob;
		    int		error = FALSE;
		    varnumber_T n;

		    // add a number to a blob
		    if (b == NULL)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			emsg(_(e_cannot_add_to_null_blob));
			goto on_error;
		    }
		    n = tv_get_number_chk(tv2, &error);
		    if (error)
			goto on_error;
		    ga_append(&b->bv_ga, (int)n);
		    --ectx->ec_stack.ga_len;
		}
		break;

	    // Computation with two arguments of unknown type
	    case ISN_OPANY:
		{
		    typval_T	*tv1 = STACK_TV_BOT(-2);
		    typval_T	*tv2 = STACK_TV_BOT(-1);
		    varnumber_T	n1, n2;
		    float_T	f1 = 0, f2 = 0;
		    int		error = FALSE;

		    if (iptr->isn_arg.op.op_type == EXPR_ADD)
		    {
			if (tv1->v_type == VAR_LIST && tv2->v_type == VAR_LIST)
			{
			    eval_addlist(tv1, tv2);
			    clear_tv(tv2);
			    --ectx->ec_stack.ga_len;
			    break;
			}
			else if (tv1->v_type == VAR_BLOB
						    && tv2->v_type == VAR_BLOB)
			{
			    eval_addblob(tv1, tv2);
			    clear_tv(tv2);
			    --ectx->ec_stack.ga_len;
			    break;
			}
		    }
		    if (tv1->v_type == VAR_FLOAT)
		    {
			f1 = tv1->vval.v_float;
			n1 = 0;
		    }
		    else
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			n1 = tv_get_number_chk(tv1, &error);
			if (error)
			    goto on_error;
			if (tv2->v_type == VAR_FLOAT)
			    f1 = n1;
		    }
		    if (tv2->v_type == VAR_FLOAT)
		    {
			f2 = tv2->vval.v_float;
			n2 = 0;
		    }
		    else
		    {
			n2 = tv_get_number_chk(tv2, &error);
			if (error)
			    goto on_error;
			if (tv1->v_type == VAR_FLOAT)
			    f2 = n2;
		    }
		    // if there is a float on either side the result is a float
		    if (tv1->v_type == VAR_FLOAT || tv2->v_type == VAR_FLOAT)
		    {
			switch (iptr->isn_arg.op.op_type)
			{
			    case EXPR_MULT: f1 = f1 * f2; break;
			    case EXPR_DIV:  f1 = f1 / f2; break;
			    case EXPR_SUB:  f1 = f1 - f2; break;
			    case EXPR_ADD:  f1 = f1 + f2; break;
			    default: SOURCING_LNUM = iptr->isn_lnum;
				     emsg(_(e_cannot_use_percent_with_float));
				     goto on_error;
			}
			clear_tv(tv1);
			clear_tv(tv2);
			tv1->v_type = VAR_FLOAT;
			tv1->vval.v_float = f1;
			--ectx->ec_stack.ga_len;
		    }
		    else
		    {
			int failed = FALSE;

			switch (iptr->isn_arg.op.op_type)
			{
			    case EXPR_MULT: n1 = n1 * n2; break;
			    case EXPR_DIV:  n1 = num_divide(n1, n2, &failed);
					    if (failed)
						goto on_error;
					    break;
			    case EXPR_SUB:  n1 = n1 - n2; break;
			    case EXPR_ADD:  n1 = n1 + n2; break;
			    default:	    n1 = num_modulus(n1, n2, &failed);
					    if (failed)
						goto on_error;
					    break;
			}
			clear_tv(tv1);
			clear_tv(tv2);
			tv1->v_type = VAR_NUMBER;
			tv1->vval.v_number = n1;
			--ectx->ec_stack.ga_len;
		    }
		}
		break;

	    case ISN_STRINDEX:
	    case ISN_STRSLICE:
		{
		    int		is_slice = iptr->isn_type == ISN_STRSLICE;
		    varnumber_T	n1 = 0, n2;
		    char_u	*res;

		    // string index: string is at stack-2, index at stack-1
		    // string slice: string is at stack-3, first index at
		    // stack-2, second index at stack-1
		    if (is_slice)
		    {
			tv = STACK_TV_BOT(-2);
			n1 = tv->vval.v_number;
		    }

		    tv = STACK_TV_BOT(-1);
		    n2 = tv->vval.v_number;

		    ectx->ec_stack.ga_len -= is_slice ? 2 : 1;
		    tv = STACK_TV_BOT(-1);
		    if (is_slice)
			// Slice: Select the characters from the string
			res = string_slice(tv->vval.v_string, n1, n2, FALSE);
		    else
			// Index: The resulting variable is a string of a
			// single character (including composing characters).
			// If the index is too big or negative the result is
			// empty.
			res = char_from_string(tv->vval.v_string, n2);
		    vim_free(tv->vval.v_string);
		    tv->vval.v_string = res;
		}
		break;

	    case ISN_LISTINDEX:
	    case ISN_LISTSLICE:
	    case ISN_BLOBINDEX:
	    case ISN_BLOBSLICE:
		{
		    int		is_slice = iptr->isn_type == ISN_LISTSLICE
					    || iptr->isn_type == ISN_BLOBSLICE;
		    int		is_blob = iptr->isn_type == ISN_BLOBINDEX
					    || iptr->isn_type == ISN_BLOBSLICE;
		    varnumber_T	n1, n2;
		    typval_T	*val_tv;

		    // list index: list is at stack-2, index at stack-1
		    // list slice: list is at stack-3, indexes at stack-2 and
		    // stack-1
		    // Same for blob.
		    val_tv = is_slice ? STACK_TV_BOT(-3) : STACK_TV_BOT(-2);

		    tv = STACK_TV_BOT(-1);
		    n1 = n2 = tv->vval.v_number;
		    clear_tv(tv);

		    if (is_slice)
		    {
			tv = STACK_TV_BOT(-2);
			n1 = tv->vval.v_number;
			clear_tv(tv);
		    }

		    ectx->ec_stack.ga_len -= is_slice ? 2 : 1;
		    tv = STACK_TV_BOT(-1);
		    SOURCING_LNUM = iptr->isn_lnum;
		    if (is_blob)
		    {
			if (blob_slice_or_index(val_tv->vval.v_blob, is_slice,
						    n1, n2, FALSE, tv) == FAIL)
			    goto on_error;
		    }
		    else
		    {
			if (list_slice_or_index(val_tv->vval.v_list, is_slice,
					      n1, n2, FALSE, tv, TRUE) == FAIL)
			    goto on_error;
		    }
		}
		break;

	    case ISN_ANYINDEX:
	    case ISN_ANYSLICE:
		{
		    int		is_slice = iptr->isn_type == ISN_ANYSLICE;
		    typval_T	*var1, *var2;
		    int		res;

		    // index: composite is at stack-2, index at stack-1
		    // slice: composite is at stack-3, indexes at stack-2 and
		    // stack-1
		    tv = is_slice ? STACK_TV_BOT(-3) : STACK_TV_BOT(-2);
		    SOURCING_LNUM = iptr->isn_lnum;
		    if (check_can_index(tv, TRUE, TRUE) == FAIL)
			goto on_error;
		    var1 = is_slice ? STACK_TV_BOT(-2) : STACK_TV_BOT(-1);
		    var2 = is_slice ? STACK_TV_BOT(-1) : NULL;
		    res = eval_index_inner(tv, is_slice, var1, var2,
							FALSE, NULL, -1, TRUE);
		    clear_tv(var1);
		    if (is_slice)
			clear_tv(var2);
		    ectx->ec_stack.ga_len -= is_slice ? 2 : 1;
		    if (res == FAIL)
			goto on_error;
		}
		break;

	    case ISN_SLICE:
		{
		    list_T	*list;
		    int		count = iptr->isn_arg.number;

		    // type will have been checked to be a list
		    tv = STACK_TV_BOT(-1);
		    list = tv->vval.v_list;

		    // no error for short list, expect it to be checked earlier
		    if (list != NULL && list->lv_len >= count)
		    {
			list_T	*newlist = list_slice(list,
						      count, list->lv_len - 1);

			if (newlist != NULL)
			{
			    list_unref(list);
			    tv->vval.v_list = newlist;
			    ++newlist->lv_refcount;
			}
		    }
		}
		break;

	    case ISN_GETITEM:
		{
		    listitem_T	*li;
		    getitem_T	*gi = &iptr->isn_arg.getitem;

		    // Get list item: list is at stack-1, push item.
		    // List type and length is checked for when compiling.
		    tv = STACK_TV_BOT(-1 - gi->gi_with_op);
		    li = list_find(tv->vval.v_list, gi->gi_index);

		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    ++ectx->ec_stack.ga_len;
		    copy_tv(&li->li_tv, STACK_TV_BOT(-1));

		    // Useful when used in unpack assignment.  Reset at
		    // ISN_DROP.
		    ectx->ec_where.wt_index = gi->gi_index + 1;
		    ectx->ec_where.wt_kind = WT_VARIABLE;
		}
		break;

	    case ISN_MEMBER:
		{
		    dict_T	*dict;
		    char_u	*key;
		    dictitem_T	*di;

		    // dict member: dict is at stack-2, key at stack-1
		    tv = STACK_TV_BOT(-2);
		    // no need to check for VAR_DICT, CHECKTYPE will check.
		    dict = tv->vval.v_dict;

		    tv = STACK_TV_BOT(-1);
		    // no need to check for VAR_STRING, 2STRING will check.
		    key = tv->vval.v_string;
		    if (key == NULL)
			key = (char_u *)"";

		    if ((di = dict_find(dict, key, -1)) == NULL)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			semsg(_(e_key_not_present_in_dictionary_str), key);

			// If :silent! is used we will continue, make sure the
			// stack contents makes sense and the dict stack is
			// updated.
			clear_tv(tv);
			--ectx->ec_stack.ga_len;
			tv = STACK_TV_BOT(-1);
			(void) dict_stack_save(tv);
			tv->v_type = VAR_NUMBER;
			tv->vval.v_number = 0;
			goto on_fatal_error;
		    }
		    clear_tv(tv);
		    --ectx->ec_stack.ga_len;
		    // Put the dict used on the dict stack, it might be used by
		    // a dict function later.
		    tv = STACK_TV_BOT(-1);
		    if (dict_stack_save(tv) == FAIL)
			goto on_fatal_error;
		    copy_tv(&di->di_tv, tv);
		}
		break;

	    // dict member with string key
	    case ISN_STRINGMEMBER:
		{
		    dict_T	*dict;
		    dictitem_T	*di;

		    tv = STACK_TV_BOT(-1);
		    if (tv->v_type != VAR_DICT || tv->vval.v_dict == NULL)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			emsg(_(e_dictionary_required));
			goto on_error;
		    }
		    dict = tv->vval.v_dict;

		    if ((di = dict_find(dict, iptr->isn_arg.string, -1))
								       == NULL)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			semsg(_(e_key_not_present_in_dictionary_str),
							 iptr->isn_arg.string);
			goto on_error;
		    }
		    // Put the dict used on the dict stack, it might be used by
		    // a dict function later.
		    if (dict_stack_save(tv) == FAIL)
			goto on_fatal_error;

		    copy_tv(&di->di_tv, tv);
		}
		break;

	    case ISN_GET_OBJ_MEMBER:
	    case ISN_GET_ITF_MEMBER:
		{
		    tv = STACK_TV_BOT(-1);
		    if (tv->v_type != VAR_OBJECT)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			object_required_error(tv);
			goto on_error;
		    }

		    object_T *obj = tv->vval.v_object;
		    if (obj == NULL)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			emsg(_(e_using_null_object));
			goto on_error;
		    }

		    int idx;
		    if (iptr->isn_type == ISN_GET_OBJ_MEMBER)
			idx = iptr->isn_arg.classmember.cm_idx;
		    else
		    {
			idx = iptr->isn_arg.classmember.cm_idx;
			// convert the interface index to the object index
			idx = object_index_from_itf_index(
					iptr->isn_arg.classmember.cm_class,
					FALSE, idx, obj->obj_class);
		    }

		    // The members are located right after the object struct.
		    typval_T *mtv = ((typval_T *)(obj + 1)) + idx;
		    copy_tv(mtv, tv);

		    // Unreference the object after getting the member, it may
		    // be freed.
		    object_unref(obj);
		}
		break;

	    case ISN_STORE_THIS:
		{
		    int idx = iptr->isn_arg.number;
		    object_T *obj = STACK_TV_VAR(0)->vval.v_object;
		    // the members are located right after the object struct
		    typval_T *mtv = ((typval_T *)(obj + 1)) + idx;
		    clear_tv(mtv);
		    *mtv = *STACK_TV_BOT(-1);
		    --ectx->ec_stack.ga_len;
		}
		break;

	    case ISN_CLEARDICT:
		dict_stack_drop();
		break;

	    case ISN_USEDICT:
		{
		    typval_T *dict_tv = dict_stack_get_tv();

		    // Turn "dict.Func" into a partial for "Func" bound to
		    // "dict".  Don't do this when "Func" is already a partial
		    // that was bound explicitly (pt_auto is FALSE).
		    tv = STACK_TV_BOT(-1);
		    if (dict_tv != NULL
			    && dict_tv->v_type == VAR_DICT
			    && dict_tv->vval.v_dict != NULL
			    && (tv->v_type == VAR_FUNC
				|| (tv->v_type == VAR_PARTIAL
				    && (tv->vval.v_partial->pt_auto
				     || tv->vval.v_partial->pt_dict == NULL))))
		    dict_tv->vval.v_dict =
					make_partial(dict_tv->vval.v_dict, tv);
		    dict_stack_drop();
		}
		break;

	    case ISN_NEGATENR:
		tv = STACK_TV_BOT(-1);
		// CHECKTYPE should have checked the variable type
		if (tv->v_type == VAR_FLOAT)
		    tv->vval.v_float = -tv->vval.v_float;
		else
		    tv->vval.v_number = -tv->vval.v_number;
		break;

	    case ISN_CHECKTYPE:
		{
		    checktype_T *ct = &iptr->isn_arg.type;
		    int		r;
		    where_T	where = WHERE_INIT;

		    tv = STACK_TV_BOT((int)ct->ct_off);
		    SOURCING_LNUM = iptr->isn_lnum;
		    if (ct->ct_arg_idx > 0)
		    {
			where.wt_index = ct->ct_arg_idx;
			where.wt_kind = ct->ct_is_var ? WT_VARIABLE : WT_ARGUMENT;
		    }
		    where.wt_func_name = ectx->ec_where.wt_func_name;
		    r = check_typval_type(ct->ct_type, tv, where);
		    if (r == FAIL)
			goto on_error;

		    // number 0 is FALSE, number 1 is TRUE
		    if (tv->v_type == VAR_NUMBER
			    && ct->ct_type->tt_type == VAR_BOOL
			    && (tv->vval.v_number == 0
						|| tv->vval.v_number == 1))
		    {
			tv->v_type = VAR_BOOL;
			tv->vval.v_number = tv->vval.v_number
						      ? VVAL_TRUE : VVAL_FALSE;
		    }
		}
		break;

	    case ISN_CHECKLEN:
		{
		    int	    min_len = iptr->isn_arg.checklen.cl_min_len;
		    list_T  *list = NULL;

		    tv = STACK_TV_BOT(-1);
		    if (tv->v_type == VAR_LIST)
			    list = tv->vval.v_list;
		    if (list == NULL || list->lv_len < min_len
			    || (list->lv_len > min_len
					&& !iptr->isn_arg.checklen.cl_more_OK))
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			semsg(_(e_expected_nr_items_but_got_nr),
				     min_len, list == NULL ? 0 : list->lv_len);
			goto on_error;
		    }
		}
		break;

	    case ISN_SETTYPE:
		set_tv_type(STACK_TV_BOT(-1), iptr->isn_arg.type.ct_type);
		break;

	    case ISN_2BOOL:
	    case ISN_COND2BOOL:
		{
		    int n;
		    int error = FALSE;

		    if (iptr->isn_type == ISN_2BOOL)
		    {
			tv = STACK_TV_BOT(iptr->isn_arg.tobool.offset);
			n = tv2bool(tv);
			if (iptr->isn_arg.tobool.invert)
			    n = !n;
		    }
		    else
		    {
			tv = STACK_TV_BOT(-1);
			SOURCING_LNUM = iptr->isn_lnum;
			n = tv_get_bool_chk(tv, &error);
			if (error)
			    goto on_error;
		    }
		    clear_tv(tv);
		    tv->v_type = VAR_BOOL;
		    tv->vval.v_number = n ? VVAL_TRUE : VVAL_FALSE;
		}
		break;

	    case ISN_2STRING:
	    case ISN_2STRING_ANY:
		SOURCING_LNUM = iptr->isn_lnum;
		if (do_2string(STACK_TV_BOT(iptr->isn_arg.tostring.offset),
				iptr->isn_type == ISN_2STRING_ANY,
				      iptr->isn_arg.tostring.tolerant) == FAIL)
			    goto on_error;
		break;

	    case ISN_RANGE:
		{
		    exarg_T	ea;
		    char	*errormsg;

		    ea.line2 = 0;
		    ea.addr_count = 0;
		    ea.addr_type = ADDR_LINES;
		    ea.cmd = iptr->isn_arg.string;
		    ea.skip = FALSE;
		    if (parse_cmd_address(&ea, &errormsg, FALSE) == FAIL)
			goto on_error;

		    if (GA_GROW_FAILS(&ectx->ec_stack, 1))
			goto theend;
		    ++ectx->ec_stack.ga_len;
		    tv = STACK_TV_BOT(-1);
		    tv->v_type = VAR_NUMBER;
		    tv->v_lock = 0;
		    tv->vval.v_number = ea.line2;
		}
		break;

	    case ISN_PUT:
		{
		    int		regname = iptr->isn_arg.put.put_regname;
		    linenr_T	lnum = iptr->isn_arg.put.put_lnum;
		    char_u	*expr = NULL;
		    int		dir = FORWARD;

		    if (lnum < -2)
		    {
			// line number was put on the stack by ISN_RANGE
			tv = STACK_TV_BOT(-1);
			curwin->w_cursor.lnum = tv->vval.v_number;
			if (lnum == LNUM_VARIABLE_RANGE_ABOVE)
			    dir = BACKWARD;
			--ectx->ec_stack.ga_len;
		    }
		    else if (lnum == -2)
			// :put! above cursor
			dir = BACKWARD;
		    else if (lnum >= 0)
		    {
			curwin->w_cursor.lnum = lnum;
			if (lnum == 0)
			    // check_cursor() below will move to line 1
			    dir = BACKWARD;
		    }

		    if (regname == '=')
		    {
			tv = STACK_TV_BOT(-1);
			if (tv->v_type == VAR_STRING)
			    expr = tv->vval.v_string;
			else
			{
			    expr = typval2string(tv, TRUE); // allocates value
			    clear_tv(tv);
			}
			--ectx->ec_stack.ga_len;
		    }
		    check_cursor();
		    do_put(regname, expr, dir, 1L, PUT_LINE|PUT_CURSLINE);
		    vim_free(expr);
		}
		break;

	    case ISN_CMDMOD:
		ectx->ec_funclocal.floc_save_cmdmod = cmdmod;
		ectx->ec_funclocal.floc_restore_cmdmod = TRUE;
		ectx->ec_funclocal.floc_restore_cmdmod_stacklen =
							 ectx->ec_stack.ga_len;
		cmdmod = *iptr->isn_arg.cmdmod.cf_cmdmod;
		apply_cmdmod(&cmdmod);
		break;

	    case ISN_CMDMOD_REV:
		// filter regprog is owned by the instruction, don't free it
		cmdmod.cmod_filter_regmatch.regprog = NULL;
		undo_cmdmod(&cmdmod);
		cmdmod = ectx->ec_funclocal.floc_save_cmdmod;
		ectx->ec_funclocal.floc_restore_cmdmod = FALSE;
		break;

	    case ISN_UNPACK:
		{
		    int		count = iptr->isn_arg.unpack.unp_count;
		    int		semicolon = iptr->isn_arg.unpack.unp_semicolon;
		    list_T	*l;
		    listitem_T	*li;
		    int		i;

		    // Check there is a valid list to unpack.
		    tv = STACK_TV_BOT(-1);
		    if (tv->v_type != VAR_LIST)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			emsg(_(e_for_argument_must_be_sequence_of_lists));
			goto on_error;
		    }
		    l = tv->vval.v_list;
		    if (l == NULL
				|| l->lv_len < (semicolon ? count - 1 : count))
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			emsg(_(e_list_value_does_not_have_enough_items));
			goto on_error;
		    }
		    else if (!semicolon && l->lv_len > count)
		    {
			SOURCING_LNUM = iptr->isn_lnum;
			emsg(_(e_list_value_has_more_items_than_targets));
			goto on_error;
		    }

		    CHECK_LIST_MATERIALIZE(l);
		    if (GA_GROW_FAILS(&ectx->ec_stack, count - 1))
			goto theend;
		    ectx->ec_stack.ga_len += count - 1;

		    // Variable after semicolon gets a list with the remaining
		    // items.
		    if (semicolon)
		    {
			list_T	*rem_list =
				  list_alloc_with_items(l->lv_len - count + 1);

			if (rem_list == NULL)
			    goto theend;
			tv = STACK_TV_BOT(-count);
			tv->vval.v_list = rem_list;
			++rem_list->lv_refcount;
			tv->v_lock = 0;
			li = l->lv_first;
			for (i = 0; i < count - 1; ++i)
			    li = li->li_next;
			for (i = 0; li != NULL; ++i)
			{
			    typval_T tvcopy;

			    copy_tv(&li->li_tv, &tvcopy);
			    list_set_item(rem_list, i, &tvcopy);
			    li = li->li_next;
			}
			--count;
		    }

		    // Produce the values in reverse order, first item last.
		    li = l->lv_first;
		    for (i = 0; i < count; ++i)
		    {
			tv = STACK_TV_BOT(-i - 1);
			copy_tv(&li->li_tv, tv);
			li = li->li_next;
		    }

		    list_unref(l);
		}
		break;

	    case ISN_PROF_START:
	    case ISN_PROF_END:
		{
#ifdef FEAT_PROFILE
		    funccall_T	cookie;
		    ufunc_T	*cur_ufunc =
				    (((dfunc_T *)def_functions.ga_data)
					       + ectx->ec_dfunc_idx)->df_ufunc;

		    cookie.fc_func = cur_ufunc;
		    if (iptr->isn_type == ISN_PROF_START)
		    {
			func_line_start(&cookie, iptr->isn_lnum);
			// if we get here the instruction is executed
			func_line_exec(&cookie);
		    }
		    else
			func_line_end(&cookie);
#endif
		}
		break;

	    case ISN_DEBUG:
		handle_debug(iptr, ectx);
		break;

	    case ISN_SHUFFLE:
		{
		    typval_T	tmp_tv;
		    int		item = iptr->isn_arg.shuffle.shfl_item;
		    int		up = iptr->isn_arg.shuffle.shfl_up;

		    tmp_tv = *STACK_TV_BOT(-item);
		    for ( ; up > 0 && item > 1; --up)
		    {
			*STACK_TV_BOT(-item) = *STACK_TV_BOT(-item + 1);
			--item;
		    }
		    *STACK_TV_BOT(-item) = tmp_tv;
		}
		break;

	    case ISN_DROP:
		--ectx->ec_stack.ga_len;
		clear_tv(STACK_TV_BOT(0));
		ectx->ec_where = (where_T)WHERE_INIT;
		break;
	}
	continue;

func_return:
	// Restore previous function. If the frame pointer is where we started
	// then there is none and we are done.
	if (ectx->ec_frame_idx == ectx->ec_initial_frame_idx)
	    goto done;

	if (func_return(ectx) == FAIL)
	    // only fails when out of memory
	    goto theend;
	continue;

on_error:
	// Jump here for an error that does not require aborting execution.
	// If "emsg_silent" is set then ignore the error, unless it was set
	// when calling the function.
	if (did_emsg_cumul + did_emsg == ectx->ec_did_emsg_before
					   && emsg_silent && did_emsg_def == 0)
	{
	    // If a sequence of instructions causes an error while ":silent!"
	    // was used, restore the stack length and jump ahead to restoring
	    // the cmdmod.
	    if (ectx->ec_funclocal.floc_restore_cmdmod)
	    {
		while (ectx->ec_stack.ga_len
			     > ectx->ec_funclocal.floc_restore_cmdmod_stacklen)
		{
		    --ectx->ec_stack.ga_len;
		    clear_tv(STACK_TV_BOT(0));
		}
		while (ectx->ec_instr[ectx->ec_iidx].isn_type != ISN_CMDMOD_REV)
		    ++ectx->ec_iidx;
	    }
	    continue;
	}
on_fatal_error:
	// Jump here for an error that messes up the stack.
	// If we are not inside a try-catch started here, abort execution.
	if (trylevel <= ectx->ec_trylevel_at_start)
	    goto theend;
    }

done:
    ret = OK;
theend:
    may_invoke_defer_funcs(ectx);

    dict_stack_clear(dict_stack_len_at_start);
    ectx->ec_trylevel_at_start = save_trylevel_at_start;
    return ret;
}

/*
 * Execute the instructions from a VAR_INSTR typval and put the result in
 * "rettv".
 * Return OK or FAIL.
 */
    int
exe_typval_instr(typval_T *tv, typval_T *rettv)
{
    ectx_T	*ectx = tv->vval.v_instr->instr_ectx;
    isn_T	*save_instr = ectx->ec_instr;
    int		save_iidx = ectx->ec_iidx;
    int		res;

    // Initialize rettv so that it is safe for caller to invoke clear_tv(rettv)
    // even when the compilation fails.
    rettv->v_type = VAR_UNKNOWN;

    ectx->ec_instr = tv->vval.v_instr->instr_instr;
    res = exec_instructions(ectx);
    if (res == OK)
    {
	*rettv = *STACK_TV_BOT(-1);
	--ectx->ec_stack.ga_len;
    }

    ectx->ec_instr = save_instr;
    ectx->ec_iidx = save_iidx;

    return res;
}

/*
 * Execute the instructions from an ISN_SUBSTITUTE command, which are in
 * "substitute_instr".
 */
    char_u *
exe_substitute_instr(void)
{
    ectx_T	*ectx = substitute_instr->subs_ectx;
    isn_T	*save_instr = ectx->ec_instr;
    int		save_iidx = ectx->ec_iidx;
    char_u	*res;

    ectx->ec_instr = substitute_instr->subs_instr;
    if (exec_instructions(ectx) == OK)
    {
	typval_T *tv = STACK_TV_BOT(-1);

	res = typval2string(tv, TRUE);
	--ectx->ec_stack.ga_len;
	clear_tv(tv);
    }
    else
    {
	substitute_instr->subs_status = FAIL;
	res = vim_strsave((char_u *)"");
    }

    ectx->ec_instr = save_instr;
    ectx->ec_iidx = save_iidx;

    return res;
}

/*
 * Call a "def" function from old Vim script.
 * Return OK or FAIL.
 */
    int
call_def_function(
    ufunc_T	*ufunc,
    int		argc_arg,	// nr of arguments
    typval_T	*argv,		// arguments
    int		flags,		// DEF_ flags
    partial_T	*partial,	// optional partial for context
    object_T	*object,	// object, e.g. for this.Func()
    funccall_T	*funccal,
    typval_T	*rettv)		// return value
{
    ectx_T	ectx;		// execution context
    int		argc = argc_arg;
    int		partial_argc = partial == NULL
				  || (flags & DEF_USE_PT_ARGV) == 0
							? 0 : partial->pt_argc;
    int		total_argc = argc + partial_argc;
    typval_T	*tv;
    int		idx;
    int		ret = FAIL;
    int		defcount = ufunc->uf_args.ga_len - total_argc;
    sctx_T	save_current_sctx = current_sctx;
    int		did_emsg_before = did_emsg_cumul + did_emsg;
    int		save_suppress_errthrow = suppress_errthrow;
    msglist_T	**saved_msg_list = NULL;
    msglist_T	*private_msg_list = NULL;
    int		save_emsg_silent_def = emsg_silent_def;
    int		save_did_emsg_def = did_emsg_def;
    int		orig_funcdepth;
    int		orig_nesting_level = ex_nesting_level;

// Get pointer to item in the stack.
#undef STACK_TV
#define STACK_TV(idx) (((typval_T *)ectx.ec_stack.ga_data) + idx)

// Get pointer to item at the bottom of the stack, -1 is the bottom.
#undef STACK_TV_BOT
#define STACK_TV_BOT(idx) (((typval_T *)ectx.ec_stack.ga_data) + ectx.ec_stack.ga_len + idx)

// Get pointer to a local variable on the stack.  Negative for arguments.
#undef STACK_TV_VAR
#define STACK_TV_VAR(idx) (((typval_T *)ectx.ec_stack.ga_data) + ectx.ec_frame_idx + STACK_FRAME_SIZE + idx)

    if (ufunc->uf_def_status == UF_NOT_COMPILED
	    || ufunc->uf_def_status == UF_COMPILE_ERROR
	    || (func_needs_compiling(ufunc, get_compile_type(ufunc))
		&& compile_def_function(ufunc, FALSE,
				       get_compile_type(ufunc), NULL) == FAIL))
    {
	if (did_emsg_cumul + did_emsg == did_emsg_before)
	    semsg(_(e_function_is_not_compiled_str),
						   printable_func_name(ufunc));
	return FAIL;
    }

    {
	// Check the function was really compiled.
	dfunc_T	*dfunc = ((dfunc_T *)def_functions.ga_data)
							 + ufunc->uf_dfunc_idx;
	if (dfunc->df_ufunc == NULL)
	{
	    semsg(_(e_function_was_deleted_str), printable_func_name(ufunc));
	    return FAIL;
	}
	if (INSTRUCTIONS(dfunc) == NULL)
	{
	    iemsg("using call_def_function() on not compiled function");
	    return FAIL;
	}
    }

    // If depth of calling is getting too high, don't execute the function.
    orig_funcdepth = funcdepth_get();
    if (funcdepth_increment() == FAIL)
	return FAIL;

    CLEAR_FIELD(ectx);
    ectx.ec_dfunc_idx = ufunc->uf_dfunc_idx;
    ga_init2(&ectx.ec_stack, sizeof(typval_T), 500);
    if (GA_GROW_FAILS(&ectx.ec_stack, 20))
    {
	funcdepth_decrement();
	return FAIL;
    }
    ga_init2(&ectx.ec_trystack, sizeof(trycmd_T), 10);
    ga_init2(&ectx.ec_funcrefs, sizeof(partial_T *), 10);
    ectx.ec_did_emsg_before = did_emsg_before;
    ++ex_nesting_level;

    idx = total_argc - ufunc->uf_args.ga_len;
    if (idx > 0 && ufunc->uf_va_name == NULL)
    {
	semsg(NGETTEXT(e_one_argument_too_many, e_nr_arguments_too_many,
								    idx), idx);
	goto failed_early;
    }
    idx = total_argc - ufunc->uf_args.ga_len + ufunc->uf_def_args.ga_len;
    if (idx < 0)
    {
	semsg(NGETTEXT(e_one_argument_too_few, e_nr_arguments_too_few,
								  -idx), -idx);
	goto failed_early;
    }

    // Put values from the partial and arguments on the stack, but no more than
    // what the function expects.  A lambda can be called with more arguments
    // than it uses.
    for (idx = 0; idx < total_argc
	    && (ufunc->uf_va_name != NULL || idx < ufunc->uf_args.ga_len);
									 ++idx)
    {
	int argv_idx = idx - partial_argc;

	tv = idx < partial_argc ? partial->pt_argv + idx : argv + argv_idx;
	if (idx >= ufunc->uf_args.ga_len - ufunc->uf_def_args.ga_len
		&& tv->v_type == VAR_SPECIAL
		&& tv->vval.v_number == VVAL_NONE)
	{
	    // Use the default value.
	    STACK_TV_BOT(0)->v_type = VAR_UNKNOWN;
	}
	else
	{
	    int done = FALSE;
	    if (ufunc->uf_arg_types != NULL && idx < ufunc->uf_args.ga_len)
	    {
		type_T *expected = ufunc->uf_arg_types[idx];
		if (expected->tt_type == VAR_FLOAT && tv->v_type == VAR_NUMBER)
		{
		    // When a float is expected and a number was given, convert
		    // the value.
		    STACK_TV_BOT(0)->v_type = VAR_FLOAT;
		    STACK_TV_BOT(0)->v_lock = 0;
		    STACK_TV_BOT(0)->vval.v_float = tv->vval.v_number;
		    done = TRUE;
		}
		else if (check_typval_arg_type(expected, tv,
						   NULL, argv_idx + 1) == FAIL)
		    goto failed_early;
	}
	    if (!done)
		copy_tv(tv, STACK_TV_BOT(0));
	}
	++ectx.ec_stack.ga_len;
    }

    // Turn varargs into a list.  Empty list if no args.
    if (ufunc->uf_va_name != NULL)
    {
	int vararg_count = argc - ufunc->uf_args.ga_len;

	if (vararg_count < 0)
	    vararg_count = 0;
	else
	    argc -= vararg_count;
	if (exe_newlist(vararg_count, &ectx) == FAIL)
	    goto failed_early;

	// Check the type of the list items.
	tv = STACK_TV_BOT(-1);
	if (ufunc->uf_va_type != NULL
		&& ufunc->uf_va_type != &t_list_any
		&& ufunc->uf_va_type->tt_member != &t_any
		&& tv->vval.v_list != NULL)
	{
	    type_T	*expected = ufunc->uf_va_type->tt_member;
	    listitem_T	*li = tv->vval.v_list->lv_first;

	    for (idx = 0; idx < vararg_count; ++idx)
	    {
		if (check_typval_arg_type(expected, &li->li_tv,
						 NULL, argc + idx + 1) == FAIL)
		    goto failed_early;
		li = li->li_next;
	    }
	}

	if (defcount > 0)
	    // Move varargs list to below missing default arguments.
	    *STACK_TV_BOT(defcount - 1) = *STACK_TV_BOT(-1);
	--ectx.ec_stack.ga_len;
    }

    // Make space for omitted arguments, will store default value below.
    // Any varargs list goes after them.
    if (defcount > 0)
	for (idx = 0; idx < defcount; ++idx)
	{
	    STACK_TV_BOT(0)->v_type = VAR_UNKNOWN;
	    ++ectx.ec_stack.ga_len;
	}
    if (ufunc->uf_va_name != NULL)
	++ectx.ec_stack.ga_len;

    // Frame pointer points to just after arguments.
    ectx.ec_frame_idx = ectx.ec_stack.ga_len;
    ectx.ec_initial_frame_idx = ectx.ec_frame_idx;

    {
	dfunc_T	*dfunc = ((dfunc_T *)def_functions.ga_data)
							 + ufunc->uf_dfunc_idx;
	ufunc_T *base_ufunc = dfunc->df_ufunc;

	// "uf_partial" is on the ufunc that "df_ufunc" points to, as is done
	// by copy_lambda_to_global_func().
	if (partial != NULL || base_ufunc->uf_partial != NULL)
	{
	    ectx.ec_outer_ref = ALLOC_CLEAR_ONE(outer_ref_T);
	    if (ectx.ec_outer_ref == NULL)
		goto failed_early;
	    if (partial != NULL)
	    {
		outer_T *outer = get_pt_outer(partial);

		if (outer->out_stack == NULL && outer->out_loop_size == 0)
		{
		    // no stack was set
		    if (current_ectx != NULL)
		    {
			if (current_ectx->ec_outer_ref != NULL
			       && current_ectx->ec_outer_ref->or_outer != NULL)
			    ectx.ec_outer_ref->or_outer =
					  current_ectx->ec_outer_ref->or_outer;
		    }
		    // else: should there be an error here?
		}
		else
		{
		    ectx.ec_outer_ref->or_outer = outer;
		    ++partial->pt_refcount;
		    ectx.ec_outer_ref->or_partial = partial;
		}
	    }
	    else
	    {
		ectx.ec_outer_ref->or_outer = &base_ufunc->uf_partial->pt_outer;
		++base_ufunc->uf_partial->pt_refcount;
		ectx.ec_outer_ref->or_partial = base_ufunc->uf_partial;
	    }
	}
    }

    // dummy frame entries
    for (idx = 0; idx < STACK_FRAME_SIZE; ++idx)
    {
	STACK_TV(ectx.ec_stack.ga_len)->v_type = VAR_UNKNOWN;
	++ectx.ec_stack.ga_len;
    }

    {
	// Reserve space for local variables and any closure reference count.
	dfunc_T	*dfunc = ((dfunc_T *)def_functions.ga_data)
							 + ufunc->uf_dfunc_idx;

	// Initialize variables to zero.  That avoids having to generate
	// initializing instructions for "var nr: number", "var x: any", etc.
	for (idx = 0; idx < dfunc->df_varcount; ++idx)
	{
	    STACK_TV_VAR(idx)->v_type = VAR_NUMBER;
	    STACK_TV_VAR(idx)->vval.v_number = 0;
	}
	ectx.ec_stack.ga_len += dfunc->df_varcount;

	if (object != NULL)
	{
	    // the object is always the variable at index zero
	    tv = STACK_TV_VAR(0);
	    tv->v_type = VAR_OBJECT;
	    tv->vval.v_object = object;
	}

	if (dfunc->df_has_closure)
	{
	    // Initialize the variable that counts how many closures were
	    // created.  This is used in handle_closure_in_use().
	    STACK_TV_VAR(idx)->v_type = VAR_NUMBER;
	    STACK_TV_VAR(idx)->vval.v_number = 0;
	    ++ectx.ec_stack.ga_len;
	}

	ectx.ec_instr = INSTRUCTIONS(dfunc);
    }

    // Store the execution context in funccal, used by invoke_all_defer().
    if (funccal != NULL)
	funccal->fc_ectx = &ectx;

    // Following errors are in the function, not the caller.
    // Commands behave like vim9script.
    estack_push_ufunc(ufunc, 1);
    current_sctx = ufunc->uf_script_ctx;
    current_sctx.sc_version = SCRIPT_VERSION_VIM9;

    // Use a specific location for storing error messages to be converted to an
    // exception.
    saved_msg_list = msg_list;
    msg_list = &private_msg_list;

    // Do turn errors into exceptions.
    suppress_errthrow = FALSE;

    // Do not delete the function while executing it.
    ++ufunc->uf_calls;

    // When ":silent!" was used before calling then we still abort the
    // function.  If ":silent!" is used in the function then we don't.
    emsg_silent_def = emsg_silent;
    did_emsg_def = 0;

    ectx.ec_where = (where_T)WHERE_INIT;

    /*
     * Execute the instructions until done.
     */
    ret = exec_instructions(&ectx);
    if (ret == OK)
    {
	// function finished, get result from the stack.
	if (ufunc->uf_ret_type == &t_void)
	{
	    rettv->v_type = VAR_VOID;
	}
	else
	{
	    tv = STACK_TV_BOT(-1);
	    *rettv = *tv;
	    tv->v_type = VAR_UNKNOWN;
	}
    }

    // When failed need to unwind the call stack.
    unwind_def_callstack(&ectx);

    // Deal with any remaining closures, they may be in use somewhere.
    if (ectx.ec_funcrefs.ga_len > 0)
    {
	handle_closure_in_use(&ectx, FALSE);
	ga_clear(&ectx.ec_funcrefs);
    }

    estack_pop();
    current_sctx = save_current_sctx;

    if (--ufunc->uf_calls <= 0 && ufunc->uf_refcount <= 0)
	// Function was unreferenced while being used, free it now.
	func_clear_free(ufunc, FALSE);

    if (*msg_list != NULL && saved_msg_list != NULL)
    {
	msglist_T **plist = saved_msg_list;

	// Append entries from the current msg_list (uncaught exceptions) to
	// the saved msg_list.
	while (*plist != NULL)
	    plist = &(*plist)->next;

	*plist = *msg_list;
    }
    msg_list = saved_msg_list;

    if (ectx.ec_funclocal.floc_restore_cmdmod)
    {
	cmdmod.cmod_filter_regmatch.regprog = NULL;
	undo_cmdmod(&cmdmod);
	cmdmod = ectx.ec_funclocal.floc_save_cmdmod;
    }
    emsg_silent_def = save_emsg_silent_def;
    did_emsg_def += save_did_emsg_def;

failed_early:
    // Free all arguments and local variables.
    for (idx = 0; idx < ectx.ec_stack.ga_len; ++idx)
    {
	tv = STACK_TV(idx);
	if (tv->v_type != VAR_NUMBER && tv->v_type != VAR_UNKNOWN)
	    clear_tv(tv);
    }
    ex_nesting_level = orig_nesting_level;

    vim_free(ectx.ec_stack.ga_data);
    vim_free(ectx.ec_trystack.ga_data);
    if (ectx.ec_outer_ref != NULL)
    {
	if (ectx.ec_outer_ref->or_outer_allocated)
	    vim_free(ectx.ec_outer_ref->or_outer);
	partial_unref(ectx.ec_outer_ref->or_partial);
	vim_free(ectx.ec_outer_ref);
    }

    // Not sure if this is necessary.
    suppress_errthrow = save_suppress_errthrow;

    if (ret != OK && did_emsg_cumul + did_emsg == did_emsg_before
							      && !need_rethrow)
	semsg(_(e_unknown_error_while_executing_str),
						   printable_func_name(ufunc));
    funcdepth_restore(orig_funcdepth);
    return ret;
}

/*
 * Called when a def function has finished (possibly failed).
 * Invoke all the function returns to clean up and invoke deferred functions,
 * except the toplevel one.
 */
    void
unwind_def_callstack(ectx_T *ectx)
{
    while (ectx->ec_frame_idx != ectx->ec_initial_frame_idx)
	func_return(ectx);
}

/*
 * Invoke any deferred functions for the top function in "ectx".
 */
    void
may_invoke_defer_funcs(ectx_T *ectx)
{
    dfunc_T *dfunc = ((dfunc_T *)def_functions.ga_data) + ectx->ec_dfunc_idx;

    if (dfunc->df_defer_var_idx > 0)
	invoke_defer_funcs(ectx);
}

/*
 * Return loopvarinfo in a printable form in allocated memory.
 */
    static char_u *
printable_loopvarinfo(loopvarinfo_T *lvi)
{
    garray_T	ga;
    int		depth;

    ga_init2(&ga, 1, 100);
    for (depth = 0; depth < lvi->lvi_depth; ++depth)
    {
	if (ga_grow(&ga, 50) == FAIL)
	    break;
	if (lvi->lvi_loop[depth].var_idx == 0)
	    STRCPY((char *)ga.ga_data + ga.ga_len, " -");
	else
	    vim_snprintf((char *)ga.ga_data + ga.ga_len, 50, " $%d-$%d",
			    lvi->lvi_loop[depth].var_idx,
			    lvi->lvi_loop[depth].var_idx
					 + lvi->lvi_loop[depth].var_count - 1);
	ga.ga_len = (int)STRLEN(ga.ga_data);
    }
    return ga.ga_data;
}

/*
 * List instructions "instr" up to "instr_count" or until ISN_FINISH.
 * "ufunc" has the source lines, NULL for the instructions of ISN_SUBSTITUTE.
 * "pfx" is prefixed to every line.
 */
    static void
list_instructions(char *pfx, isn_T *instr, int instr_count, ufunc_T *ufunc)
{
    int		line_idx = 0;
    int		prev_current = 0;
    int		current;
    int		def_arg_idx = 0;

    for (current = 0; current < instr_count; ++current)
    {
	isn_T	    *iptr = &instr[current];
	char	    *line;

	if (ufunc != NULL)
	{
	    while (line_idx < iptr->isn_lnum
					  && line_idx < ufunc->uf_lines.ga_len)
	    {
		if (current > prev_current)
		{
		    msg_puts("\n\n");
		    prev_current = current;
		}
		line = ((char **)ufunc->uf_lines.ga_data)[line_idx++];
		if (line != NULL)
		    msg(line);
	    }
	    if (iptr->isn_type == ISN_JUMP_IF_ARG_SET)
	    {
		int	first_def_arg = ufunc->uf_args.ga_len
						   - ufunc->uf_def_args.ga_len;

		if (def_arg_idx > 0)
		    msg_puts("\n\n");
		msg_start();
		msg_puts("  ");
		msg_puts(((char **)(ufunc->uf_args.ga_data))[
						 first_def_arg + def_arg_idx]);
		msg_puts(" = ");
		msg_puts(((char **)(ufunc->uf_def_args.ga_data))[def_arg_idx++]);
		msg_clr_eos();
		msg_end();
	    }
	}

	switch (iptr->isn_type)
	{
	    case ISN_CONSTRUCT:
		smsg("%s%4d NEW %s size %d", pfx, current,
			iptr->isn_arg.construct.construct_class->class_name,
				  (int)iptr->isn_arg.construct.construct_size);
		break;
	    case ISN_EXEC:
		smsg("%s%4d EXEC %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_EXEC_SPLIT:
		smsg("%s%4d EXEC_SPLIT %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_EXECRANGE:
		smsg("%s%4d EXECRANGE %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_LEGACY_EVAL:
		smsg("%s%4d EVAL legacy %s", pfx, current,
							 iptr->isn_arg.string);
		break;
	    case ISN_REDIRSTART:
		smsg("%s%4d REDIR", pfx, current);
		break;
	    case ISN_REDIREND:
		smsg("%s%4d REDIR END%s", pfx, current,
					iptr->isn_arg.number ? " append" : "");
		break;
	    case ISN_CEXPR_AUCMD:
#ifdef FEAT_QUICKFIX
		smsg("%s%4d CEXPR pre %s", pfx, current,
				       cexpr_get_auname(iptr->isn_arg.number));
#endif
		break;
	    case ISN_CEXPR_CORE:
#ifdef FEAT_QUICKFIX
		{
		    cexprref_T	    *cer = iptr->isn_arg.cexpr.cexpr_ref;

		    smsg("%s%4d CEXPR core %s%s \"%s\"", pfx, current,
				       cexpr_get_auname(cer->cer_cmdidx),
				       cer->cer_forceit ? "!" : "",
				       cer->cer_cmdline);
		}
#endif
		break;
	    case ISN_INSTR:
		smsg("%s%4d INSTR", pfx, current);
		list_instructions("    ", iptr->isn_arg.instr, INT_MAX, NULL);
		msg("     -------------");
		break;
	    case ISN_SOURCE:
		{
		    scriptitem_T    *si = SCRIPT_ITEM(iptr->isn_arg.number);

		    smsg("%s%4d SOURCE %s", pfx, current, si->sn_name);
		}
		break;
	    case ISN_SUBSTITUTE:
		{
		    subs_T *subs = &iptr->isn_arg.subs;

		    smsg("%s%4d SUBSTITUTE %s", pfx, current, subs->subs_cmd);
		    list_instructions("    ", subs->subs_instr, INT_MAX, NULL);
		    msg("     -------------");
		}
		break;
	    case ISN_EXECCONCAT:
		smsg("%s%4d EXECCONCAT %lld", pfx, current,
					      (varnumber_T)iptr->isn_arg.number);
		break;
	    case ISN_ECHO:
		{
		    echo_T *echo = &iptr->isn_arg.echo;

		    smsg("%s%4d %s %d", pfx, current,
			    echo->echo_with_white ? "ECHO" : "ECHON",
			    echo->echo_count);
		}
		break;
	    case ISN_EXECUTE:
		smsg("%s%4d EXECUTE %lld", pfx, current,
					  (varnumber_T)(iptr->isn_arg.number));
		break;
	    case ISN_ECHOMSG:
		smsg("%s%4d ECHOMSG %lld", pfx, current,
					  (varnumber_T)(iptr->isn_arg.number));
		break;
	    case ISN_ECHOWINDOW:
		if (iptr->isn_arg.echowin.ewin_time > 0)
		    smsg("%s%4d ECHOWINDOW %d (%ld sec)", pfx, current,
				      iptr->isn_arg.echowin.ewin_count,
				      iptr->isn_arg.echowin.ewin_time);
		else
		    smsg("%s%4d ECHOWINDOW %d", pfx, current,
					     iptr->isn_arg.echowin.ewin_count);
		break;
	    case ISN_ECHOCONSOLE:
		smsg("%s%4d ECHOCONSOLE %lld", pfx, current,
					  (varnumber_T)(iptr->isn_arg.number));
		break;
	    case ISN_ECHOERR:
		smsg("%s%4d ECHOERR %lld", pfx, current,
					  (varnumber_T)(iptr->isn_arg.number));
		break;
	    case ISN_LOAD:
		{
		    if (iptr->isn_arg.number < 0)
			smsg("%s%4d LOAD arg[%lld]", pfx, current,
				(varnumber_T)(iptr->isn_arg.number
							  + STACK_FRAME_SIZE));
		    else
			smsg("%s%4d LOAD $%lld", pfx, current,
					  (varnumber_T)(iptr->isn_arg.number));
		}
		break;
	    case ISN_LOADOUTER:
		{
		    isn_outer_T *outer = &iptr->isn_arg.outer;

		    if (outer->outer_idx < 0)
			smsg("%s%4d LOADOUTER level %d arg[%d]", pfx, current,
					outer->outer_depth,
					outer->outer_idx + STACK_FRAME_SIZE);
		    else if (outer->outer_depth < 0)
			smsg("%s%4d LOADOUTER $%d in loop level %d",
					       pfx, current,
					       outer->outer_idx,
					       -outer->outer_depth);
		    else
			smsg("%s%4d LOADOUTER level %d $%d", pfx, current,
					      outer->outer_depth,
					      outer->outer_idx);
		}
		break;
	    case ISN_LOADV:
		smsg("%s%4d LOADV v:%s", pfx, current,
				       get_vim_var_name(iptr->isn_arg.number));
		break;
	    case ISN_LOADSCRIPT:
		{
		    scriptref_T	    *sref = iptr->isn_arg.script.scriptref;
		    scriptitem_T    *si = SCRIPT_ITEM(sref->sref_sid);
		    svar_T	    *sv;

		    sv = get_script_svar(sref, -1);
		    if (sv == NULL)
			smsg("%s%4d LOADSCRIPT [deleted] from %s",
						    pfx, current, si->sn_name);
		    else
			smsg("%s%4d LOADSCRIPT %s-%d from %s", pfx, current,
					    sv->sv_name,
					    sref->sref_idx,
					    si->sn_name);
		}
		break;
	    case ISN_LOADS:
	    case ISN_LOADEXPORT:
		{
		    scriptitem_T *si = SCRIPT_ITEM(
					       iptr->isn_arg.loadstore.ls_sid);

		    smsg("%s%4d %s s:%s from %s", pfx, current,
			    iptr->isn_type == ISN_LOADS ? "LOADS"
								: "LOADEXPORT",
			    iptr->isn_arg.loadstore.ls_name, si->sn_name);
		}
		break;
	    case ISN_LOADAUTO:
		smsg("%s%4d LOADAUTO %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_LOADG:
		smsg("%s%4d LOADG g:%s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_LOADB:
		smsg("%s%4d LOADB b:%s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_LOADW:
		smsg("%s%4d LOADW w:%s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_LOADT:
		smsg("%s%4d LOADT t:%s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_LOADGDICT:
		smsg("%s%4d LOAD g:", pfx, current);
		break;
	    case ISN_LOADBDICT:
		smsg("%s%4d LOAD b:", pfx, current);
		break;
	    case ISN_LOADWDICT:
		smsg("%s%4d LOAD w:", pfx, current);
		break;
	    case ISN_LOADTDICT:
		smsg("%s%4d LOAD t:", pfx, current);
		break;
	    case ISN_LOADOPT:
		smsg("%s%4d LOADOPT %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_LOADENV:
		smsg("%s%4d LOADENV %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_LOADREG:
		smsg("%s%4d LOADREG @%c", pfx, current,
						  (int)(iptr->isn_arg.number));
		break;

	    case ISN_STORE:
		if (iptr->isn_arg.number < 0)
		    smsg("%s%4d STORE arg[%lld]", pfx, current,
				      iptr->isn_arg.number + STACK_FRAME_SIZE);
		else
		    smsg("%s%4d STORE $%lld", pfx, current,
							 iptr->isn_arg.number);
		break;
	    case ISN_STOREOUTER:
		{
		    isn_outer_T *outer = &iptr->isn_arg.outer;

		    if (outer->outer_depth == OUTER_LOOP_DEPTH)
			smsg("%s%4d STOREOUTER level 1 $%d in loop",
				pfx, current, outer->outer_idx);
		    else
			smsg("%s%4d STOREOUTER level %d $%d", pfx, current,
				outer->outer_depth, outer->outer_idx);
		}
		break;
	    case ISN_STOREV:
		smsg("%s%4d STOREV v:%s", pfx, current,
				       get_vim_var_name(iptr->isn_arg.number));
		break;
	    case ISN_STOREAUTO:
		smsg("%s%4d STOREAUTO %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_STOREG:
		smsg("%s%4d STOREG %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_STOREB:
		smsg("%s%4d STOREB %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_STOREW:
		smsg("%s%4d STOREW %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_STORET:
		smsg("%s%4d STORET %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_STORES:
	    case ISN_STOREEXPORT:
		{
		    scriptitem_T *si = SCRIPT_ITEM(
					       iptr->isn_arg.loadstore.ls_sid);

		    smsg("%s%4d %s %s in %s", pfx, current,
				iptr->isn_type == ISN_STORES
						    ? "STORES" : "STOREEXPORT",
				 iptr->isn_arg.loadstore.ls_name, si->sn_name);
		}
		break;
	    case ISN_STORESCRIPT:
		{
		    scriptref_T	    *sref = iptr->isn_arg.script.scriptref;
		    scriptitem_T    *si = SCRIPT_ITEM(sref->sref_sid);
		    svar_T	    *sv;

		    sv = get_script_svar(sref, -1);
		    if (sv == NULL)
			smsg("%s%4d STORESCRIPT [deleted] in %s",
						    pfx, current, si->sn_name);
		    else
			smsg("%s%4d STORESCRIPT %s-%d in %s", pfx, current,
					     sv->sv_name,
					     sref->sref_idx,
					     si->sn_name);
		}
		break;
	    case ISN_STOREOPT:
	    case ISN_STOREFUNCOPT:
		smsg("%s%4d %s &%s", pfx, current,
		  iptr->isn_type == ISN_STOREOPT ? "STOREOPT" : "STOREFUNCOPT",
					       iptr->isn_arg.storeopt.so_name);
		break;
	    case ISN_STOREENV:
		smsg("%s%4d STOREENV $%s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_STOREREG:
		smsg("%s%4d STOREREG @%c", pfx, current,
						    (int)iptr->isn_arg.number);
		break;
	    case ISN_STORENR:
		smsg("%s%4d STORE %lld in $%d", pfx, current,
				iptr->isn_arg.storenr.stnr_val,
				iptr->isn_arg.storenr.stnr_idx);
		break;

	    case ISN_STOREINDEX:
		smsg("%s%4d STOREINDEX %s", pfx, current,
			    vartype_name(iptr->isn_arg.storeindex.si_vartype));
		break;

	    case ISN_STORERANGE:
		smsg("%s%4d STORERANGE", pfx, current);
		break;

	    case ISN_LOAD_CLASSMEMBER:
	    case ISN_STORE_CLASSMEMBER:
		{
		    class_T *cl = iptr->isn_arg.classmember.cm_class;
		    int	    idx = iptr->isn_arg.classmember.cm_idx;
		    ocmember_T *ocm = &cl->class_class_members[idx];
		    smsg("%s%4d %s CLASSMEMBER %s.%s", pfx, current,
			    iptr->isn_type == ISN_LOAD_CLASSMEMBER
							    ? "LOAD" : "STORE",
			    cl->class_name, ocm->ocm_name);
		}
		break;

	    // constants
	    case ISN_PUSHNR:
		smsg("%s%4d PUSHNR %lld", pfx, current,
					    (varnumber_T)(iptr->isn_arg.number));
		break;
	    case ISN_PUSHBOOL:
	    case ISN_PUSHSPEC:
		smsg("%s%4d PUSH %s", pfx, current,
				   get_var_special_name(iptr->isn_arg.number));
		break;
	    case ISN_PUSHF:
		smsg("%s%4d PUSHF %g", pfx, current, iptr->isn_arg.fnumber);
		break;
	    case ISN_PUSHS:
		smsg("%s%4d PUSHS \"%s\"", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_PUSHBLOB:
		{
		    char_u	*r;
		    char_u	numbuf[NUMBUFLEN];
		    char_u	*tofree;

		    r = blob2string(iptr->isn_arg.blob, &tofree, numbuf);
		    smsg("%s%4d PUSHBLOB %s", pfx, current, r);
		    vim_free(tofree);
		}
		break;
	    case ISN_PUSHFUNC:
		{
		    char *name = (char *)iptr->isn_arg.string;

		    smsg("%s%4d PUSHFUNC \"%s\"", pfx, current,
					       name == NULL ? "[none]" : name);
		}
		break;
	    case ISN_PUSHCHANNEL:
#ifdef FEAT_JOB_CHANNEL
		smsg("%s%4d PUSHCHANNEL 0", pfx, current);
#endif
		break;
	    case ISN_PUSHJOB:
#ifdef FEAT_JOB_CHANNEL
		smsg("%s%4d PUSHJOB \"no process\"", pfx, current);
#endif
		break;
	    case ISN_PUSHOBJ:
		smsg("%s%4d PUSHOBJ null", pfx, current);
		break;
	    case ISN_PUSHCLASS:
		smsg("%s%4d PUSHCLASS %s", pfx, current,
			iptr->isn_arg.classarg == NULL ? "null"
				 : (char *)iptr->isn_arg.classarg->class_name);
		break;
	    case ISN_PUSHEXC:
		smsg("%s%4d PUSH v:exception", pfx, current);
		break;
	    case ISN_AUTOLOAD:
		smsg("%s%4d AUTOLOAD %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_UNLET:
		smsg("%s%4d UNLET%s %s", pfx, current,
			iptr->isn_arg.unlet.ul_forceit ? "!" : "",
			iptr->isn_arg.unlet.ul_name);
		break;
	    case ISN_UNLETENV:
		smsg("%s%4d UNLETENV%s $%s", pfx, current,
			iptr->isn_arg.unlet.ul_forceit ? "!" : "",
			iptr->isn_arg.unlet.ul_name);
		break;
	    case ISN_UNLETINDEX:
		smsg("%s%4d UNLETINDEX", pfx, current);
		break;
	    case ISN_UNLETRANGE:
		smsg("%s%4d UNLETRANGE", pfx, current);
		break;
	    case ISN_LOCKUNLOCK:
		smsg("%s%4d LOCKUNLOCK %s", pfx, current, iptr->isn_arg.string);
		break;
	    case ISN_LOCKCONST:
		smsg("%s%4d LOCKCONST", pfx, current);
		break;
	    case ISN_NEWLIST:
		smsg("%s%4d NEWLIST size %lld", pfx, current,
					  (varnumber_T)(iptr->isn_arg.number));
		break;
	    case ISN_NEWDICT:
		smsg("%s%4d NEWDICT size %lld", pfx, current,
					  (varnumber_T)(iptr->isn_arg.number));
		break;
	    case ISN_NEWPARTIAL:
		smsg("%s%4d NEWPARTIAL", pfx, current);
		break;

	    // function call
	    case ISN_BCALL:
		{
		    cbfunc_T	*cbfunc = &iptr->isn_arg.bfunc;

		    smsg("%s%4d BCALL %s(argc %d)", pfx, current,
			    internal_func_name(cbfunc->cbf_idx),
			    cbfunc->cbf_argcount);
		}
		break;
	    case ISN_DCALL:
		{
		    cdfunc_T	*cdfunc = &iptr->isn_arg.dfunc;
		    dfunc_T	*df = ((dfunc_T *)def_functions.ga_data)
							     + cdfunc->cdf_idx;

		    smsg("%s%4d DCALL %s(argc %d)", pfx, current,
					    printable_func_name(df->df_ufunc),
							 cdfunc->cdf_argcount);
		}
		break;
	    case ISN_METHODCALL:
		{
		    cmfunc_T	*mfunc = iptr->isn_arg.mfunc;

		    smsg("%s%4d METHODCALL %s.%s(argc %d)", pfx, current,
			    mfunc->cmf_itf->class_name,
			    mfunc->cmf_itf->class_obj_methods[
						      mfunc->cmf_idx]->uf_name,
			    mfunc->cmf_argcount);
		}
		break;
	    case ISN_UCALL:
		{
		    cufunc_T	*cufunc = &iptr->isn_arg.ufunc;

		    smsg("%s%4d UCALL %s(argc %d)", pfx, current,
				       cufunc->cuf_name, cufunc->cuf_argcount);
		}
		break;
	    case ISN_PCALL:
		{
		    cpfunc_T	*cpfunc = &iptr->isn_arg.pfunc;

		    smsg("%s%4d PCALL%s (argc %d)", pfx, current,
			   cpfunc->cpf_top ? " top" : "", cpfunc->cpf_argcount);
		}
		break;
	    case ISN_PCALL_END:
		smsg("%s%4d PCALL end", pfx, current);
		break;
	    case ISN_DEFER:
		smsg("%s%4d DEFER %d args", pfx, current,
				      (int)iptr->isn_arg.defer.defer_argcount);
		break;
	    case ISN_RETURN:
		smsg("%s%4d RETURN", pfx, current);
		break;
	    case ISN_RETURN_VOID:
		smsg("%s%4d RETURN void", pfx, current);
		break;
	    case ISN_RETURN_OBJECT:
		smsg("%s%4d RETURN object", pfx, current);
		break;
	    case ISN_FUNCREF:
		{
		    funcref_T		*funcref = &iptr->isn_arg.funcref;
		    funcref_extra_T	*extra = funcref->fr_extra;
		    char_u		*name;

		    if (extra == NULL || extra->fre_func_name == NULL)
		    {
			dfunc_T	*df = ((dfunc_T *)def_functions.ga_data)
						       + funcref->fr_dfunc_idx;
			name = df->df_ufunc->uf_name;
		    }
		    else
			name = extra->fre_func_name;
		    if (extra != NULL && extra->fre_class != NULL)
		    {
			smsg("%s%4d FUNCREF %s.%s", pfx, current,
					   extra->fre_class->class_name, name);
		    }
		    else if (extra == NULL
				     || extra->fre_loopvar_info.lvi_depth == 0)
		    {
			smsg("%s%4d FUNCREF %s", pfx, current, name);
		    }
		    else
		    {
			char_u	*info = printable_loopvarinfo(
						     &extra->fre_loopvar_info);

			smsg("%s%4d FUNCREF %s vars %s", pfx, current,
								   name, info);
			vim_free(info);
		    }
		}
		break;

	    case ISN_NEWFUNC:
		{
		    newfuncarg_T    *arg = iptr->isn_arg.newfunc.nf_arg;

		    if (arg->nfa_loopvar_info.lvi_depth == 0)
			smsg("%s%4d NEWFUNC %s %s", pfx, current,
					     arg->nfa_lambda, arg->nfa_global);
		    else
		    {
			char_u	*info = printable_loopvarinfo(
						       &arg->nfa_loopvar_info);

			smsg("%s%4d NEWFUNC %s %s vars %s", pfx, current,
				       arg->nfa_lambda, arg->nfa_global, info);
			vim_free(info);
		    }
		}
		break;

	    case ISN_DEF:
		{
		    char_u *name = iptr->isn_arg.string;

		    smsg("%s%4d DEF %s", pfx, current,
					   name == NULL ? (char_u *)"" : name);
		}
		break;

	    case ISN_JUMP:
		{
		    char *when = "?";

		    switch (iptr->isn_arg.jump.jump_when)
		    {
			case JUMP_ALWAYS:
			    when = "JUMP";
			    break;
			case JUMP_NEVER:
			    iemsg("JUMP_NEVER should not be used");
			    break;
			case JUMP_AND_KEEP_IF_TRUE:
			    when = "JUMP_AND_KEEP_IF_TRUE";
			    break;
			case JUMP_IF_FALSE:
			    when = "JUMP_IF_FALSE";
			    break;
			case JUMP_WHILE_FALSE:
			    when = "JUMP_WHILE_FALSE";  // unused
			    break;
			case JUMP_IF_COND_FALSE:
			    when = "JUMP_IF_COND_FALSE";
			    break;
			case JUMP_IF_COND_TRUE:
			    when = "JUMP_IF_COND_TRUE";
			    break;
		    }
		    smsg("%s%4d %s -> %d", pfx, current, when,
						iptr->isn_arg.jump.jump_where);
		}
		break;

	    case ISN_JUMP_IF_ARG_SET:
		smsg("%s%4d JUMP_IF_ARG_SET arg[%d] -> %d", pfx, current,
			 iptr->isn_arg.jumparg.jump_arg_off + STACK_FRAME_SIZE,
						iptr->isn_arg.jump.jump_where);
		break;

	    case ISN_JUMP_IF_ARG_NOT_SET:
		smsg("%s%4d JUMP_IF_ARG_NOT_SET arg[%d] -> %d", pfx, current,
			 iptr->isn_arg.jumparg.jump_arg_off + STACK_FRAME_SIZE,
						iptr->isn_arg.jump.jump_where);
		break;

	    case ISN_FOR:
		{
		    forloop_T *forloop = &iptr->isn_arg.forloop;

		    smsg("%s%4d FOR $%d -> %d", pfx, current,
				      forloop->for_loop_idx, forloop->for_end);
		}
		break;

	    case ISN_ENDLOOP:
		{
		    endloop_T *endloop = &iptr->isn_arg.endloop;

		    smsg("%s%4d ENDLOOP ref $%d save $%d-$%d depth %d",
								  pfx, current,
			    endloop->end_funcref_idx,
			    endloop->end_var_idx,
			    endloop->end_var_idx + endloop->end_var_count - 1,
			    endloop->end_depth);
		}
		break;

	    case ISN_WHILE:
		{
		    whileloop_T *whileloop = &iptr->isn_arg.whileloop;

		    smsg("%s%4d WHILE $%d -> %d", pfx, current,
					       whileloop->while_funcref_idx,
					       whileloop->while_end);
		}
		break;

	    case ISN_TRY:
		{
		    try_T *try = &iptr->isn_arg.tryref;

		    if (try->try_ref->try_finally == 0)
			smsg("%s%4d TRY catch -> %d, endtry -> %d",
				pfx, current,
				try->try_ref->try_catch,
				try->try_ref->try_endtry);
		    else
			smsg("%s%4d TRY catch -> %d, finally -> %d, endtry -> %d",
				pfx, current,
				try->try_ref->try_catch,
				try->try_ref->try_finally,
				try->try_ref->try_endtry);
		}
		break;
	    case ISN_CATCH:
		smsg("%s%4d CATCH", pfx, current);
		break;
	    case ISN_TRYCONT:
		{
		    trycont_T *trycont = &iptr->isn_arg.trycont;

		    smsg("%s%4d TRY-CONTINUE %d level%s -> %d", pfx, current,
				      trycont->tct_levels,
				      trycont->tct_levels == 1 ? "" : "s",
				      trycont->tct_where);
		}
		break;
	    case ISN_FINALLY:
		smsg("%s%4d FINALLY", pfx, current);
		break;
	    case ISN_ENDTRY:
		smsg("%s%4d ENDTRY", pfx, current);
		break;
	    case ISN_THROW:
		smsg("%s%4d THROW", pfx, current);
		break;

	    // expression operations on number
	    case ISN_OPNR:
	    case ISN_OPFLOAT:
	    case ISN_OPANY:
		{
		    char *what;
		    char *ins;

		    switch (iptr->isn_arg.op.op_type)
		    {
			case EXPR_MULT: what = "*"; break;
			case EXPR_DIV: what = "/"; break;
			case EXPR_REM: what = "%"; break;
			case EXPR_SUB: what = "-"; break;
			case EXPR_ADD: what = "+"; break;
			case EXPR_LSHIFT: what = "<<"; break;
			case EXPR_RSHIFT: what = ">>"; break;
			default:       what = "???"; break;
		    }
		    switch (iptr->isn_type)
		    {
			case ISN_OPNR: ins = "OPNR"; break;
			case ISN_OPFLOAT: ins = "OPFLOAT"; break;
			case ISN_OPANY: ins = "OPANY"; break;
			default: ins = "???"; break;
		    }
		    smsg("%s%4d %s %s", pfx, current, ins, what);
		}
		break;

	    case ISN_COMPAREBOOL:
	    case ISN_COMPARESPECIAL:
	    case ISN_COMPARENULL:
	    case ISN_COMPARENR:
	    case ISN_COMPAREFLOAT:
	    case ISN_COMPARESTRING:
	    case ISN_COMPAREBLOB:
	    case ISN_COMPARELIST:
	    case ISN_COMPAREDICT:
	    case ISN_COMPAREFUNC:
	    case ISN_COMPAREOBJECT:
	    case ISN_COMPAREANY:
		   {
		       char *p;
		       char buf[10];
		       char *type;

		       switch (iptr->isn_arg.op.op_type)
		       {
			   case EXPR_EQUAL:	 p = "=="; break;
			   case EXPR_NEQUAL:    p = "!="; break;
			   case EXPR_GREATER:   p = ">"; break;
			   case EXPR_GEQUAL:    p = ">="; break;
			   case EXPR_SMALLER:   p = "<"; break;
			   case EXPR_SEQUAL:    p = "<="; break;
			   case EXPR_MATCH:	 p = "=~"; break;
			   case EXPR_IS:	 p = "is"; break;
			   case EXPR_ISNOT:	 p = "isnot"; break;
			   case EXPR_NOMATCH:	 p = "!~"; break;
			   default:  p = "???"; break;
		       }
		       STRCPY(buf, p);
		       if (iptr->isn_arg.op.op_ic == TRUE)
			   strcat(buf, "?");
		       switch(iptr->isn_type)
		       {
			   case ISN_COMPAREBOOL: type = "COMPAREBOOL"; break;
			   case ISN_COMPARESPECIAL:
						 type = "COMPARESPECIAL"; break;
			   case ISN_COMPARENULL: type = "COMPARENULL"; break;
			   case ISN_COMPARENR: type = "COMPARENR"; break;
			   case ISN_COMPAREFLOAT: type = "COMPAREFLOAT"; break;
			   case ISN_COMPARESTRING:
						  type = "COMPARESTRING"; break;
			   case ISN_COMPAREBLOB: type = "COMPAREBLOB"; break;
			   case ISN_COMPARELIST: type = "COMPARELIST"; break;
			   case ISN_COMPAREDICT: type = "COMPAREDICT"; break;
			   case ISN_COMPAREFUNC: type = "COMPAREFUNC"; break;
			   case ISN_COMPAREOBJECT:
						 type = "COMPAREOBJECT"; break;
			   case ISN_COMPAREANY: type = "COMPAREANY"; break;
			   default: type = "???"; break;
		       }

		       smsg("%s%4d %s %s", pfx, current, type, buf);
		   }
		   break;

	    case ISN_ADDLIST: smsg("%s%4d ADDLIST", pfx, current); break;
	    case ISN_ADDBLOB: smsg("%s%4d ADDBLOB", pfx, current); break;

	    // expression operations
	    case ISN_CONCAT:
		smsg("%s%4d CONCAT size %lld", pfx, current,
					    (varnumber_T)(iptr->isn_arg.number));
		break;
	    case ISN_STRINDEX: smsg("%s%4d STRINDEX", pfx, current); break;
	    case ISN_STRSLICE: smsg("%s%4d STRSLICE", pfx, current); break;
	    case ISN_BLOBINDEX: smsg("%s%4d BLOBINDEX", pfx, current); break;
	    case ISN_BLOBSLICE: smsg("%s%4d BLOBSLICE", pfx, current); break;
	    case ISN_LISTAPPEND: smsg("%s%4d LISTAPPEND", pfx, current); break;
	    case ISN_BLOBAPPEND: smsg("%s%4d BLOBAPPEND", pfx, current); break;
	    case ISN_LISTINDEX: smsg("%s%4d LISTINDEX", pfx, current); break;
	    case ISN_LISTSLICE: smsg("%s%4d LISTSLICE", pfx, current); break;
	    case ISN_ANYINDEX: smsg("%s%4d ANYINDEX", pfx, current); break;
	    case ISN_ANYSLICE: smsg("%s%4d ANYSLICE", pfx, current); break;
	    case ISN_SLICE: smsg("%s%4d SLICE %lld",
				    pfx, current, iptr->isn_arg.number); break;
	    case ISN_GETITEM: smsg("%s%4d ITEM %lld%s", pfx, current,
					 iptr->isn_arg.getitem.gi_index,
					 iptr->isn_arg.getitem.gi_with_op ?
						       " with op" : ""); break;
	    case ISN_MEMBER: smsg("%s%4d MEMBER", pfx, current); break;
	    case ISN_STRINGMEMBER: smsg("%s%4d MEMBER %s", pfx, current,
						  iptr->isn_arg.string); break;
	    case ISN_GET_OBJ_MEMBER: smsg("%s%4d OBJ_MEMBER %d", pfx, current,
			     (int)iptr->isn_arg.classmember.cm_idx);
				     break;
	    case ISN_GET_ITF_MEMBER: smsg("%s%4d ITF_MEMBER %d on %s",
			     pfx, current,
			     (int)iptr->isn_arg.classmember.cm_idx,
			     iptr->isn_arg.classmember.cm_class->class_name);
				     break;
	    case ISN_STORE_THIS: smsg("%s%4d STORE_THIS %d", pfx, current,
					     (int)iptr->isn_arg.number); break;
	    case ISN_CLEARDICT: smsg("%s%4d CLEARDICT", pfx, current); break;
	    case ISN_USEDICT: smsg("%s%4d USEDICT", pfx, current); break;

	    case ISN_NEGATENR: smsg("%s%4d NEGATENR", pfx, current); break;

	    case ISN_CHECKTYPE:
		  {
		      checktype_T   *ct = &iptr->isn_arg.type;
		      char	    *tofree = NULL;
		      char	    *typename;

		      if (ct->ct_type->tt_type == VAR_FLOAT
			      && (ct->ct_type->tt_flags & TTFLAG_NUMBER_OK))
			  typename = "float|number";
		      else
			  typename = type_name(ct->ct_type, &tofree);

		      if (ct->ct_arg_idx == 0)
			  smsg("%s%4d CHECKTYPE %s stack[%d]", pfx, current,
					  typename,
					  (int)ct->ct_off);
		      else
			  smsg("%s%4d CHECKTYPE %s stack[%d] %s %d",
					  pfx, current,
					  typename,
					  (int)ct->ct_off,
					  ct->ct_is_var ? "var": "arg",
					  (int)ct->ct_arg_idx);
		      vim_free(tofree);
		      break;
		  }
	    case ISN_CHECKLEN: smsg("%s%4d CHECKLEN %s%d", pfx, current,
				iptr->isn_arg.checklen.cl_more_OK ? ">= " : "",
				iptr->isn_arg.checklen.cl_min_len);
			       break;
	    case ISN_SETTYPE:
		  {
		      char *tofree;

		      smsg("%s%4d SETTYPE %s", pfx, current,
			      type_name(iptr->isn_arg.type.ct_type, &tofree));
		      vim_free(tofree);
		      break;
		  }
	    case ISN_COND2BOOL: smsg("%s%4d COND2BOOL", pfx, current); break;
	    case ISN_2BOOL: if (iptr->isn_arg.tobool.invert)
				smsg("%s%4d INVERT %d (!val)", pfx, current,
					 iptr->isn_arg.tobool.offset);
			    else
				smsg("%s%4d 2BOOL %d (!!val)", pfx, current,
					 iptr->isn_arg.tobool.offset);
			    break;
	    case ISN_2STRING: smsg("%s%4d 2STRING stack[%lld]", pfx, current,
				 (varnumber_T)(iptr->isn_arg.tostring.offset));
			      break;
	    case ISN_2STRING_ANY: smsg("%s%4d 2STRING_ANY stack[%lld]",
								  pfx, current,
				 (varnumber_T)(iptr->isn_arg.tostring.offset));
			      break;
	    case ISN_RANGE: smsg("%s%4d RANGE %s", pfx, current,
							 iptr->isn_arg.string);
			    break;
	    case ISN_PUT:
		if (iptr->isn_arg.put.put_lnum == LNUM_VARIABLE_RANGE_ABOVE)
		    smsg("%s%4d PUT %c above range",
				  pfx, current, iptr->isn_arg.put.put_regname);
		else if (iptr->isn_arg.put.put_lnum == LNUM_VARIABLE_RANGE)
		    smsg("%s%4d PUT %c range",
				  pfx, current, iptr->isn_arg.put.put_regname);
		else
		    smsg("%s%4d PUT %c %ld", pfx, current,
						 iptr->isn_arg.put.put_regname,
					     (long)iptr->isn_arg.put.put_lnum);
		break;

	    case ISN_CMDMOD:
		{
		    char_u  *buf;
		    size_t  len = produce_cmdmods(
				  NULL, iptr->isn_arg.cmdmod.cf_cmdmod, FALSE);

		    buf = alloc(len + 1);
		    if (likely(buf != NULL))
		    {
			(void)produce_cmdmods(
				   buf, iptr->isn_arg.cmdmod.cf_cmdmod, FALSE);
			smsg("%s%4d CMDMOD %s", pfx, current, buf);
			vim_free(buf);
		    }
		    break;
		}
	    case ISN_CMDMOD_REV: smsg("%s%4d CMDMOD_REV", pfx, current); break;

	    case ISN_PROF_START:
		 smsg("%s%4d PROFILE START line %d", pfx, current,
							       iptr->isn_lnum);
		 break;

	    case ISN_PROF_END:
		smsg("%s%4d PROFILE END", pfx, current);
		break;

	    case ISN_DEBUG:
		smsg("%s%4d DEBUG line %d-%d varcount %lld", pfx, current,
			iptr->isn_arg.debug.dbg_break_lnum + 1,
			iptr->isn_lnum,
			iptr->isn_arg.debug.dbg_var_names_len);
		break;

	    case ISN_UNPACK: smsg("%s%4d UNPACK %d%s", pfx, current,
			iptr->isn_arg.unpack.unp_count,
			iptr->isn_arg.unpack.unp_semicolon ? " semicolon" : "");
			      break;
	    case ISN_SHUFFLE: smsg("%s%4d SHUFFLE %d up %d", pfx, current,
					 iptr->isn_arg.shuffle.shfl_item,
					 iptr->isn_arg.shuffle.shfl_up);
			      break;
	    case ISN_DROP: smsg("%s%4d DROP", pfx, current); break;

	    case ISN_FINISH: // End of list of instructions for ISN_SUBSTITUTE.
			   return;
	}

	out_flush();	    // output one line at a time
	ui_breakcheck();
	if (got_int)
	    break;
    }
}

/*
 * Handle command line completion for the :disassemble command.
 */
    void
set_context_in_disassemble_cmd(expand_T *xp, char_u *arg)
{
    char_u	*p;

    // Default: expand user functions, "debug" and "profile"
    xp->xp_context = EXPAND_DISASSEMBLE;
    xp->xp_pattern = arg;

    // first argument already typed: only user function names
    if (*arg != NUL && *(p = skiptowhite(arg)) != NUL)
    {
	xp->xp_context = EXPAND_USER_FUNC;
	xp->xp_pattern = skipwhite(p);
    }
}

/*
 * Function given to ExpandGeneric() to obtain the list of :disassemble
 * arguments.
 */
    char_u *
get_disassemble_argument(expand_T *xp, int idx)
{
    if (idx == 0)
	return (char_u *)"debug";
    if (idx == 1)
	return (char_u *)"profile";
    return get_user_func_name(xp, idx - 2);
}

/*
 * ":disassemble".
 * We don't really need this at runtime, but we do have tests that require it,
 * so always include this.
 */
    void
ex_disassemble(exarg_T *eap)
{
    char_u	*arg = eap->arg;
    ufunc_T	*ufunc;
    dfunc_T	*dfunc;
    isn_T	*instr = NULL;  // init to shut up gcc warning
    int		instr_count = 0;  // init to shut up gcc warning
    compiletype_T compile_type = CT_NONE;

    ufunc = find_func_by_name(arg, &compile_type);
    if (ufunc == NULL)
	return;
    if (func_needs_compiling(ufunc, compile_type)
	    && compile_def_function(ufunc, FALSE, compile_type, NULL) == FAIL)
	return;
    if (ufunc->uf_def_status != UF_COMPILED)
    {
	semsg(_(e_function_is_not_compiled_str), eap->arg);
	return;
    }
    msg((char *)printable_func_name(ufunc));

    dfunc = ((dfunc_T *)def_functions.ga_data) + ufunc->uf_dfunc_idx;
    switch (compile_type)
    {
	case CT_PROFILE:
#ifdef FEAT_PROFILE
	    instr = dfunc->df_instr_prof;
	    instr_count = dfunc->df_instr_prof_count;
	    break;
#endif
	    // FALLTHROUGH
	case CT_NONE:
	    instr = dfunc->df_instr;
	    instr_count = dfunc->df_instr_count;
	    break;
	case CT_DEBUG:
	    instr = dfunc->df_instr_debug;
	    instr_count = dfunc->df_instr_debug_count;
	    break;
    }

    list_instructions("", instr, instr_count, ufunc);
}

/*
 * Return TRUE when "tv" is not falsy: non-zero, non-empty string, non-empty
 * list, etc.  Mostly like what JavaScript does, except that empty list and
 * empty dictionary are FALSE.
 */
    int
tv2bool(typval_T *tv)
{
    switch (tv->v_type)
    {
	case VAR_NUMBER:
	    return tv->vval.v_number != 0;
	case VAR_FLOAT:
	    return tv->vval.v_float != 0.0;
	case VAR_PARTIAL:
	    return tv->vval.v_partial != NULL;
	case VAR_FUNC:
	case VAR_STRING:
	    return tv->vval.v_string != NULL && *tv->vval.v_string != NUL;
	case VAR_LIST:
	    return tv->vval.v_list != NULL && tv->vval.v_list->lv_len > 0;
	case VAR_DICT:
	    return tv->vval.v_dict != NULL
				    && tv->vval.v_dict->dv_hashtab.ht_used > 0;
	case VAR_BOOL:
	case VAR_SPECIAL:
	    return tv->vval.v_number == VVAL_TRUE ? TRUE : FALSE;
	case VAR_JOB:
#ifdef FEAT_JOB_CHANNEL
	    return tv->vval.v_job != NULL;
#else
	    break;
#endif
	case VAR_CHANNEL:
#ifdef FEAT_JOB_CHANNEL
	    return tv->vval.v_channel != NULL;
#else
	    break;
#endif
	case VAR_BLOB:
	    return tv->vval.v_blob != NULL && tv->vval.v_blob->bv_ga.ga_len > 0;
	case VAR_UNKNOWN:
	case VAR_ANY:
	case VAR_VOID:
	case VAR_INSTR:
	case VAR_CLASS:
	case VAR_OBJECT:
	case VAR_TYPEALIAS:
	    break;
    }
    return FALSE;
}

    void
emsg_using_string_as(typval_T *tv, int as_number)
{
    semsg(_(as_number ? e_using_string_as_number_str
						 : e_using_string_as_bool_str),
		       tv->vval.v_string == NULL
					   ? (char_u *)"" : tv->vval.v_string);
}

/*
 * If "tv" is a string give an error and return FAIL.
 */
    int
check_not_string(typval_T *tv)
{
    if (tv->v_type == VAR_STRING)
    {
	emsg_using_string_as(tv, TRUE);
	clear_tv(tv);
	return FAIL;
    }
    return OK;
}


#endif // FEAT_EVAL
