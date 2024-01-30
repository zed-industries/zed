/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * evalwindow.c: Window related builtin functions
 */

#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

    static int
win_getid(typval_T *argvars)
{
    int	    winnr;
    win_T   *wp;

    if (argvars[0].v_type == VAR_UNKNOWN)
	return curwin->w_id;
    winnr = tv_get_number(&argvars[0]);
    if (winnr <= 0)
	return 0;

    if (argvars[1].v_type == VAR_UNKNOWN)
	wp = firstwin;
    else
    {
	tabpage_T	*tp;
	int		tabnr = tv_get_number(&argvars[1]);

	FOR_ALL_TABPAGES(tp)
	    if (--tabnr == 0)
		break;
	if (tp == NULL)
	    return -1;
	if (tp == curtab)
	    wp = firstwin;
	else
	    wp = tp->tp_firstwin;
    }
    for ( ; wp != NULL; wp = wp->w_next)
	if (--winnr == 0)
	    return wp->w_id;
    return 0;
}

    static void
win_id2tabwin(typval_T *argvars, list_T *list)
{
    win_T	*wp;
    tabpage_T   *tp;
    int		winnr = 1;
    int		tabnr = 1;
    int		id = tv_get_number(&argvars[0]);

    FOR_ALL_TABPAGES(tp)
    {
	FOR_ALL_WINDOWS_IN_TAB(tp, wp)
	{
	    if (wp->w_id == id)
	    {
		list_append_number(list, tabnr);
		list_append_number(list, winnr);
		return;
	    }
	    ++winnr;
	}
	++tabnr;
	winnr = 1;
    }
    list_append_number(list, 0);
    list_append_number(list, 0);
}

/*
 * Return the window pointer of window "id".
 */
    win_T *
win_id2wp(int id)
{
    return win_id2wp_tp(id, NULL);
}

/*
 * Return the window and tab pointer of window "id".
 * Returns NULL when not found.
 */
    win_T *
win_id2wp_tp(int id, tabpage_T **tpp)
{
    win_T	*wp;
    tabpage_T   *tp;

    FOR_ALL_TAB_WINDOWS(tp, wp)
	if (wp->w_id == id)
	{
	    if (tpp != NULL)
		*tpp = tp;
	    return wp;
	}
#ifdef FEAT_PROP_POPUP
    // popup windows are in separate lists
     FOR_ALL_TABPAGES(tp)
	 FOR_ALL_POPUPWINS_IN_TAB(tp, wp)
	     if (wp->w_id == id)
	     {
		 if (tpp != NULL)
		     *tpp = tp;
		 return wp;
	     }
    FOR_ALL_POPUPWINS(wp)
	if (wp->w_id == id)
	{
	    if (tpp != NULL)
		*tpp = curtab;  // any tabpage would do
	    return wp;
	}
#endif

    return NULL;
}

    static int
win_id2win(typval_T *argvars)
{
    win_T   *wp;
    int	    nr = 1;
    int	    id = tv_get_number(&argvars[0]);

    FOR_ALL_WINDOWS(wp)
    {
	if (wp->w_id == id)
	    return nr;
	++nr;
    }
    return 0;
}

    void
win_findbuf(typval_T *argvars, list_T *list)
{
    win_T	*wp;
    tabpage_T   *tp;
    int		bufnr = tv_get_number(&argvars[0]);

    FOR_ALL_TAB_WINDOWS(tp, wp)
	    if (wp->w_buffer->b_fnum == bufnr)
		list_append_number(list, wp->w_id);
}

/*
 * Find window specified by "vp" in tabpage "tp".
 * Returns current window if "vp" is number zero.
 * Returns NULL if not found.
 */
    win_T *
find_win_by_nr(
    typval_T	*vp,
    tabpage_T	*tp)	// NULL for current tab page
{
    win_T	*wp;
    int		nr = (int)tv_get_number_chk(vp, NULL);

    if (nr < 0)
	return NULL;
    if (nr == 0)
	return curwin;

    FOR_ALL_WINDOWS_IN_TAB(tp, wp)
    {
	if (nr >= LOWEST_WIN_ID)
	{
	    if (wp->w_id == nr)
		return wp;
	}
	else if (--nr <= 0)
	    break;
    }
    if (nr >= LOWEST_WIN_ID)
    {
#ifdef FEAT_PROP_POPUP
	// check tab-local popup windows
	for (wp = (tp == NULL ? curtab : tp)->tp_first_popupwin;
						   wp != NULL; wp = wp->w_next)
	    if (wp->w_id == nr)
		return wp;
	// check global popup windows
	FOR_ALL_POPUPWINS(wp)
	    if (wp->w_id == nr)
		return wp;
#endif
	return NULL;
    }
    return wp;
}

/*
 * Find a window: When using a Window ID in any tab page, when using a number
 * in the current tab page.
 * Returns NULL when not found.
 */
    win_T *
find_win_by_nr_or_id(typval_T *vp)
{
    int	nr = (int)tv_get_number_chk(vp, NULL);

    if (nr >= LOWEST_WIN_ID)
	return win_id2wp(tv_get_number(vp));
    return find_win_by_nr(vp, NULL);
}

/*
 * Find window specified by "wvp" in tabpage "tvp".
 * Returns the tab page in 'ptp'
 */
    win_T *
find_tabwin(
    typval_T	*wvp,	// VAR_UNKNOWN for current window
    typval_T	*tvp,	// VAR_UNKNOWN for current tab page
    tabpage_T	**ptp)
{
    win_T	*wp = NULL;
    tabpage_T	*tp = NULL;
    long	n;

    if (wvp->v_type != VAR_UNKNOWN)
    {
	if (tvp->v_type != VAR_UNKNOWN)
	{
	    n = (long)tv_get_number(tvp);
	    if (n >= 0)
		tp = find_tabpage(n);
	}
	else
	    tp = curtab;

	if (tp != NULL)
	{
	    wp = find_win_by_nr(wvp, tp);
	    if (wp == NULL && wvp->v_type == VAR_NUMBER
						&& wvp->vval.v_number != -1)
		// A window with the specified number is not found
		tp = NULL;
	}
    }
    else
    {
	wp = curwin;
	tp = curtab;
    }

    if (ptp != NULL)
	*ptp = tp;

    return wp;
}

/*
 * Get the layout of the given tab page for winlayout() and add it to "l".
 */
    static void
get_framelayout(frame_T *fr, list_T *l, int outer)
{
    frame_T	*child;
    list_T	*fr_list;
    list_T	*win_list;

    if (fr == NULL)
	return;

    if (outer)
	// outermost call from f_winlayout()
	fr_list = l;
    else
    {
	fr_list = list_alloc();
	if (fr_list == NULL)
	    return;
	if (list_append_list(l, fr_list) == FAIL)
	{
	    vim_free(fr_list);
	    return;
	}
    }

    if (fr->fr_layout == FR_LEAF)
    {
	if (fr->fr_win != NULL)
	{
	    list_append_string(fr_list, (char_u *)"leaf", -1);
	    list_append_number(fr_list, fr->fr_win->w_id);
	}
    }
    else
    {
	list_append_string(fr_list,
	     fr->fr_layout == FR_ROW ?  (char_u *)"row" : (char_u *)"col", -1);

	win_list = list_alloc();
	if (win_list == NULL)
	    return;
	if (list_append_list(fr_list, win_list) == FAIL)
	{
	    vim_free(win_list);
	    return;
	}

	child = fr->fr_child;
	while (child != NULL)
	{
	    get_framelayout(child, win_list, FALSE);
	    child = child->fr_next;
	}
    }
}

/*
 * Common code for tabpagewinnr() and winnr().
 */
    static int
get_winnr(tabpage_T *tp, typval_T *argvar)
{
    win_T	*twin;
    int		nr = 1;
    win_T	*wp;
    char_u	*arg;

    twin = (tp == curtab) ? curwin : tp->tp_curwin;
    if (argvar->v_type != VAR_UNKNOWN)
    {
	int	invalid_arg = FALSE;

	arg = tv_get_string_chk(argvar);
	if (arg == NULL)
	    nr = 0;		// type error; errmsg already given
	else if (STRCMP(arg, "$") == 0)
	    twin = (tp == curtab) ? lastwin : tp->tp_lastwin;
	else if (STRCMP(arg, "#") == 0)
	{
	    twin = (tp == curtab) ? prevwin : tp->tp_prevwin;
	}
	else
	{
	    long	count;
	    char_u	*endp;

	    // Extract the window count (if specified). e.g. winnr('3j')
	    count = strtol((char *)arg, (char **)&endp, 10);
	    if (count <= 0)
		count = 1;	// if count is not specified, default to 1
	    if (endp != NULL && *endp != '\0')
	    {
		if (STRCMP(endp, "j") == 0)
		    twin = win_vert_neighbor(tp, twin, FALSE, count);
		else if (STRCMP(endp, "k") == 0)
		    twin = win_vert_neighbor(tp, twin, TRUE, count);
		else if (STRCMP(endp, "h") == 0)
		    twin = win_horz_neighbor(tp, twin, TRUE, count);
		else if (STRCMP(endp, "l") == 0)
		    twin = win_horz_neighbor(tp, twin, FALSE, count);
		else
		    invalid_arg = TRUE;
	    }
	    else
		invalid_arg = TRUE;
	}
	if (twin == NULL)
	    nr = 0;

	if (invalid_arg)
	{
	    semsg(_(e_invalid_expression_str), arg);
	    nr = 0;
	}
    }

    if (nr <= 0)
	return 0;

    for (wp = (tp == curtab) ? firstwin : tp->tp_firstwin;
	    wp != twin; wp = wp->w_next)
    {
	if (wp == NULL)
	{
	    // didn't find it in this tabpage
	    nr = 0;
	    break;
	}
	++nr;
    }
    return nr;
}

/*
 * Returns information about a window as a dictionary.
 */
    static dict_T *
get_win_info(win_T *wp, short tpnr, short winnr)
{
    dict_T	*dict;

    dict = dict_alloc();
    if (dict == NULL)
	return NULL;

    // make sure w_botline is valid
    validate_botline_win(wp);

    dict_add_number(dict, "tabnr", tpnr);
    dict_add_number(dict, "winnr", winnr);
    dict_add_number(dict, "winid", wp->w_id);
    dict_add_number(dict, "height", wp->w_height);
    dict_add_number(dict, "winrow", wp->w_winrow + 1);
    dict_add_number(dict, "topline", wp->w_topline);
    dict_add_number(dict, "botline", wp->w_botline - 1);
#ifdef FEAT_MENU
    dict_add_number(dict, "winbar", wp->w_winbar_height);
#endif
    dict_add_number(dict, "width", wp->w_width);
    dict_add_number(dict, "wincol", wp->w_wincol + 1);
    dict_add_number(dict, "textoff", win_col_off(wp));
    dict_add_number(dict, "bufnr", wp->w_buffer->b_fnum);

#ifdef FEAT_TERMINAL
    dict_add_number(dict, "terminal", bt_terminal(wp->w_buffer));
#endif
#ifdef FEAT_QUICKFIX
    dict_add_number(dict, "quickfix", bt_quickfix(wp->w_buffer));
    dict_add_number(dict, "loclist",
		      (bt_quickfix(wp->w_buffer) && wp->w_llist_ref != NULL));
#endif

    // Add a reference to window variables
    dict_add_dict(dict, "variables", wp->w_vars);

    return dict;
}

/*
 * Returns information (variables, options, etc.) about a tab page
 * as a dictionary.
 */
    static dict_T *
get_tabpage_info(tabpage_T *tp, int tp_idx)
{
    win_T	*wp;
    dict_T	*dict;
    list_T	*l;

    dict = dict_alloc();
    if (dict == NULL)
	return NULL;

    dict_add_number(dict, "tabnr", tp_idx);

    l = list_alloc();
    if (l != NULL)
    {
	FOR_ALL_WINDOWS_IN_TAB(tp, wp)
	    list_append_number(l, (varnumber_T)wp->w_id);
	dict_add_list(dict, "windows", l);
    }

    // Make a reference to tabpage variables
    dict_add_dict(dict, "variables", tp->tp_vars);

    return dict;
}

/*
 * "gettabinfo()" function
 */
    void
f_gettabinfo(typval_T *argvars, typval_T *rettv)
{
    tabpage_T	*tp, *tparg = NULL;
    dict_T	*d;
    int		tpnr = 0;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	// Information about one tab page
	tparg = find_tabpage((int)tv_get_number_chk(&argvars[0], NULL));
	if (tparg == NULL)
	    return;
    }

    // Get information about a specific tab page or all tab pages
    FOR_ALL_TABPAGES(tp)
    {
	tpnr++;
	if (tparg != NULL && tp != tparg)
	    continue;
	d = get_tabpage_info(tp, tpnr);
	if (d != NULL)
	    list_append_dict(rettv->vval.v_list, d);
	if (tparg != NULL)
	    return;
    }
}

/*
 * "getwininfo()" function
 */
    void
f_getwininfo(typval_T *argvars, typval_T *rettv)
{
    tabpage_T	*tp;
    win_T	*wp = NULL, *wparg = NULL;
    dict_T	*d;
    short	tabnr = 0, winnr;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	wparg = win_id2wp(tv_get_number(&argvars[0]));
	if (wparg == NULL)
	    return;
    }

    // Collect information about either all the windows across all the tab
    // pages or one particular window.
    FOR_ALL_TABPAGES(tp)
    {
	tabnr++;
	winnr = 0;
	FOR_ALL_WINDOWS_IN_TAB(tp, wp)
	{
	    winnr++;
	    if (wparg != NULL && wp != wparg)
		continue;
	    d = get_win_info(wp, tabnr, winnr);
	    if (d != NULL)
		list_append_dict(rettv->vval.v_list, d);
	    if (wparg != NULL)
		// found information about a specific window
		return;
	}
    }
#ifdef FEAT_PROP_POPUP
    if (wparg != NULL)
    {
	tabnr = 0;
	FOR_ALL_TABPAGES(tp)
	{
	    tabnr++;
	    FOR_ALL_POPUPWINS_IN_TAB(tp, wp)
	    if (wp == wparg)
		break;
	}
	d = get_win_info(wparg, tp == NULL ? 0 : tabnr, 0);
	if (d != NULL)
	    list_append_dict(rettv->vval.v_list, d);
    }
#endif
}

/*
 * "getwinpos({timeout})" function
 */
    void
f_getwinpos(typval_T *argvars UNUSED, typval_T *rettv)
{
    int x = -1;
    int y = -1;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

#if defined(FEAT_GUI) \
	|| (defined(HAVE_TGETENT) && defined(FEAT_TERMRESPONSE)) \
	|| defined(MSWIN)
    {
	varnumber_T timeout = 100;

	if (argvars[0].v_type != VAR_UNKNOWN)
	    timeout = tv_get_number(&argvars[0]);

	(void)ui_get_winpos(&x, &y, timeout);
    }
#endif
    list_append_number(rettv->vval.v_list, (varnumber_T)x);
    list_append_number(rettv->vval.v_list, (varnumber_T)y);
}


/*
 * "getwinposx()" function
 */
    void
f_getwinposx(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->vval.v_number = -1;
#if defined(FEAT_GUI) \
	|| (defined(HAVE_TGETENT) && defined(FEAT_TERMRESPONSE)) \
	|| defined(MSWIN)

    {
	int	    x, y;

	if (ui_get_winpos(&x, &y, 100) == OK)
	    rettv->vval.v_number = x;
    }
#endif
}

/*
 * "getwinposy()" function
 */
    void
f_getwinposy(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->vval.v_number = -1;
#if defined(FEAT_GUI) \
	|| (defined(HAVE_TGETENT) && defined(FEAT_TERMRESPONSE)) \
	|| defined(MSWIN)
    {
	int	    x, y;

	if (ui_get_winpos(&x, &y, 100) == OK)
	    rettv->vval.v_number = y;
    }
#endif
}

/*
 * "tabpagenr()" function
 */
    void
f_tabpagenr(typval_T *argvars UNUSED, typval_T *rettv)
{
    int		nr = 1;
    char_u	*arg;

    if (in_vim9script() && check_for_opt_string_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	arg = tv_get_string_chk(&argvars[0]);
	nr = 0;
	if (arg != NULL)
	{
	    if (STRCMP(arg, "$") == 0)
		nr = tabpage_index(NULL) - 1;
	    else if (STRCMP(arg, "#") == 0)
		nr = valid_tabpage(lastused_tabpage) ?
					tabpage_index(lastused_tabpage) : 0;
	    else
		semsg(_(e_invalid_expression_str), arg);
	}
    }
    else
	nr = tabpage_index(curtab);
    rettv->vval.v_number = nr;
}

/*
 * "tabpagewinnr()" function
 */
    void
f_tabpagewinnr(typval_T *argvars UNUSED, typval_T *rettv)
{
    int		nr = 1;
    tabpage_T	*tp;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL))
	return;

    tp = find_tabpage((int)tv_get_number(&argvars[0]));
    if (tp == NULL)
	nr = 0;
    else
	nr = get_winnr(tp, &argvars[1]);
    rettv->vval.v_number = nr;
}

/*
 * "win_execute()" function
 */
    void
f_win_execute(typval_T *argvars, typval_T *rettv)
{
    int		id;
    tabpage_T	*tp;
    win_T	*wp;
    switchwin_T	switchwin;

    // Return an empty string if something fails.
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_string_or_list_arg(argvars, 1) == FAIL
		|| check_for_opt_string_arg(argvars, 2) == FAIL))
	return;

    id = (int)tv_get_number(argvars);
    wp = win_id2wp_tp(id, &tp);
    if (wp == NULL || tp == NULL)
	return;

    pos_T	curpos = wp->w_cursor;
    char_u	cwd[MAXPATHL];
    int	cwd_status = FAIL;
#ifdef FEAT_AUTOCHDIR
    char_u	autocwd[MAXPATHL];
    int	apply_acd = FALSE;
#endif

    // Getting and setting directory can be slow on some systems, only do
    // this when the current or target window/tab have a local directory or
    // 'acd' is set.
    if (curwin != wp
	    && (curwin->w_localdir != NULL
		|| wp->w_localdir != NULL
		|| (curtab != tp
		    && (curtab->tp_localdir != NULL
			|| tp->tp_localdir != NULL))
#ifdef FEAT_AUTOCHDIR
		|| p_acd
#endif
	       ))
	cwd_status = mch_dirname(cwd, MAXPATHL);

#ifdef FEAT_AUTOCHDIR
    // If 'acd' is set, check we are using that directory.  If yes, then
    // apply 'acd' afterwards, otherwise restore the current directory.
    if (cwd_status == OK && p_acd)
    {
	do_autochdir();
	apply_acd = mch_dirname(autocwd, MAXPATHL) == OK
	    && STRCMP(cwd, autocwd) == 0;
    }
#endif

    if (switch_win_noblock(&switchwin, wp, tp, TRUE) == OK)
    {
	check_cursor();
	execute_common(argvars, rettv, 1);
    }
    restore_win_noblock(&switchwin, TRUE);
#ifdef FEAT_AUTOCHDIR
    if (apply_acd)
	do_autochdir();
    else
#endif
	if (cwd_status == OK)
	    mch_chdir((char *)cwd);

    // Update the status line if the cursor moved.
    if (win_valid(wp) && !EQUAL_POS(curpos, wp->w_cursor))
	wp->w_redr_status = TRUE;

    // In case the command moved the cursor or changed the Visual area,
    // check it is valid.
    check_cursor();
    if (VIsual_active)
	check_pos(curbuf, &VIsual);
}

/*
 * "win_findbuf()" function
 */
    void
f_win_findbuf(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    if (rettv_list_alloc(rettv) == OK)
	win_findbuf(argvars, rettv->vval.v_list);
}

/*
 * "win_getid()" function
 */
    void
f_win_getid(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script()
	    && (check_for_opt_number_arg(argvars, 0) == FAIL
		|| (argvars[0].v_type != VAR_UNKNOWN
		    && check_for_opt_number_arg(argvars, 1) == FAIL)))
	return;

    rettv->vval.v_number = win_getid(argvars);
}

/*
 * "win_gotoid()" function
 */
    void
f_win_gotoid(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp;
    tabpage_T   *tp;
    int		id;

    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    id = tv_get_number(&argvars[0]);
    if (cmdwin_type != 0)
    {
	emsg(_(e_invalid_in_cmdline_window));
	return;
    }
#if defined(FEAT_PROP_POPUP) && defined(FEAT_TERMINAL)
    if (popup_is_popup(curwin) && curbuf->b_term != NULL)
    {
	emsg(_(e_not_allowed_for_terminal_in_popup_window));
	return;
    }
#endif
    FOR_ALL_TAB_WINDOWS(tp, wp)
	if (wp->w_id == id)
	{
	    // When jumping to another buffer stop Visual mode.
	    if (VIsual_active && wp->w_buffer != curbuf)
		end_visual_mode();
	    goto_tabpage_win(tp, wp);
	    rettv->vval.v_number = 1;
	    return;
	}
}

/*
 * "win_id2tabwin()" function
 */
    void
f_win_id2tabwin(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    if (rettv_list_alloc(rettv) == OK)
	win_id2tabwin(argvars, rettv->vval.v_list);
}

/*
 * "win_id2win()" function
 */
    void
f_win_id2win(typval_T *argvars, typval_T *rettv)
{
    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    rettv->vval.v_number = win_id2win(argvars);
}

/*
 * "win_move_separator()" function
 */
    void
f_win_move_separator(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp;
    int		offset;

    rettv->vval.v_number = FALSE;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    wp = find_win_by_nr_or_id(&argvars[0]);
    if (wp == NULL || win_valid_popup(wp))
	return;
    if (!win_valid(wp))
    {
	emsg(_(e_cannot_resize_window_in_another_tab_page));
	return;
    }

    offset = (int)tv_get_number(&argvars[1]);
    win_drag_vsep_line(wp, offset);
    rettv->vval.v_number = TRUE;
}

/*
 * "win_move_statusline()" function
 */
    void
f_win_move_statusline(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp;
    int		offset;

    rettv->vval.v_number = FALSE;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL))
	return;

    wp = find_win_by_nr_or_id(&argvars[0]);
    if (wp == NULL || win_valid_popup(wp))
	return;
    if (!win_valid(wp))
    {
	emsg(_(e_cannot_resize_window_in_another_tab_page));
	return;
    }

    offset = (int)tv_get_number(&argvars[1]);
    win_drag_status_line(wp, offset);
    rettv->vval.v_number = TRUE;
}

/*
 * "win_screenpos()" function
 */
    void
f_win_screenpos(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    wp = find_win_by_nr_or_id(&argvars[0]);
    list_append_number(rettv->vval.v_list, wp == NULL ? 0 : wp->w_winrow + 1);
    list_append_number(rettv->vval.v_list, wp == NULL ? 0 : wp->w_wincol + 1);
}

/*
 * Move the window wp into a new split of targetwin in a given direction
 */
    static void
win_move_into_split(win_T *wp, win_T *targetwin, int size, int flags)
{
    int	    dir;
    int	    height = wp->w_height;
    win_T   *oldwin = curwin;

    if (wp == targetwin)
	return;

    // Jump to the target window
    if (curwin != targetwin)
	win_goto(targetwin);

    // Remove the old window and frame from the tree of frames
    (void)winframe_remove(wp, &dir, NULL);
    win_remove(wp, NULL);
    last_status(FALSE);	    // may need to remove last status line
    (void)win_comp_pos();   // recompute window positions

    // Split a window on the desired side and put the old window there
    (void)win_split_ins(size, flags, wp, dir);

    // If splitting horizontally, try to preserve height
    if (size == 0 && !(flags & WSP_VERT))
    {
	win_setheight_win(height, wp);
	if (p_ea)
	    win_equal(wp, TRUE, 'v');
    }

#if defined(FEAT_GUI)
    // When 'guioptions' includes 'L' or 'R' may have to remove or add
    // scrollbars.  Have to update them anyway.
    gui_may_update_scrollbars();
#endif

    if (oldwin != curwin)
	win_goto(oldwin);
}

/*
 * "win_splitmove()" function
 */
    void
f_win_splitmove(typval_T *argvars, typval_T *rettv)
{
    win_T   *wp;
    win_T   *targetwin;
    int     flags = 0, size = 0;

    if (in_vim9script()
	    && (check_for_number_arg(argvars, 0) == FAIL
		|| check_for_number_arg(argvars, 1) == FAIL
		|| check_for_opt_dict_arg(argvars, 2) == FAIL))
	return;

    wp = find_win_by_nr_or_id(&argvars[0]);
    targetwin = find_win_by_nr_or_id(&argvars[1]);

    if (wp == NULL || targetwin == NULL || wp == targetwin
	    || !win_valid(wp) || !win_valid(targetwin)
	    || win_valid_popup(wp) || win_valid_popup(targetwin))
    {
	emsg(_(e_invalid_window_number));
	rettv->vval.v_number = -1;
	return;
    }

    if (argvars[2].v_type != VAR_UNKNOWN)
    {
	dict_T      *d;
	dictitem_T  *di;

	if (check_for_nonnull_dict_arg(argvars, 2) == FAIL)
	    return;

	d = argvars[2].vval.v_dict;
	if (dict_get_bool(d, "vertical", FALSE))
	    flags |= WSP_VERT;
	if ((di = dict_find(d, (char_u *)"rightbelow", -1)) != NULL)
	    flags |= tv_get_bool(&di->di_tv) ? WSP_BELOW : WSP_ABOVE;
	size = (int)dict_get_number(d, "size");
    }

    win_move_into_split(wp, targetwin, size, flags);
}

/*
 * "win_gettype(nr)" function
 */
    void
f_win_gettype(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp = curwin;

    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;

    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	wp = find_win_by_nr_or_id(&argvars[0]);
	if (wp == NULL)
	{
	    rettv->vval.v_string = vim_strsave((char_u *)"unknown");
	    return;
	}
    }
    if (is_aucmd_win(wp))
	rettv->vval.v_string = vim_strsave((char_u *)"autocmd");
#if defined(FEAT_QUICKFIX)
    else if (wp->w_p_pvw)
	rettv->vval.v_string = vim_strsave((char_u *)"preview");
#endif
#ifdef FEAT_PROP_POPUP
    else if (WIN_IS_POPUP(wp))
	rettv->vval.v_string = vim_strsave((char_u *)"popup");
#endif
    else if (wp == cmdwin_win)
	rettv->vval.v_string = vim_strsave((char_u *)"command");
#ifdef FEAT_QUICKFIX
    else if (bt_quickfix(wp->w_buffer))
	rettv->vval.v_string = vim_strsave((char_u *)
		(wp->w_llist_ref != NULL ? "loclist" : "quickfix"));
#endif

}

/*
 * "getcmdwintype()" function
 */
    void
f_getcmdwintype(typval_T *argvars UNUSED, typval_T *rettv)
{
    rettv->v_type = VAR_STRING;
    rettv->vval.v_string = NULL;
    rettv->vval.v_string = alloc(2);
    if (rettv->vval.v_string == NULL)
	return;

    rettv->vval.v_string[0] = cmdwin_type;
    rettv->vval.v_string[1] = NUL;
}

/*
 * "winbufnr(nr)" function
 */
    void
f_winbufnr(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp;

    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    wp = find_win_by_nr_or_id(&argvars[0]);
    if (wp == NULL)
	rettv->vval.v_number = -1;
    else
	rettv->vval.v_number = wp->w_buffer->b_fnum;
}

/*
 * "wincol()" function
 */
    void
f_wincol(typval_T *argvars UNUSED, typval_T *rettv)
{
    validate_cursor();
    rettv->vval.v_number = curwin->w_wcol + 1;
}

/*
 * "winheight(nr)" function
 */
    void
f_winheight(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp;

    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    wp = find_win_by_nr_or_id(&argvars[0]);
    if (wp == NULL)
	rettv->vval.v_number = -1;
    else
	rettv->vval.v_number = wp->w_height;
}

/*
 * "winlayout()" function
 */
    void
f_winlayout(typval_T *argvars, typval_T *rettv)
{
    tabpage_T	*tp;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (in_vim9script() && check_for_opt_number_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_UNKNOWN)
	tp = curtab;
    else
    {
	tp = find_tabpage((int)tv_get_number(&argvars[0]));
	if (tp == NULL)
	    return;
    }

    get_framelayout(tp->tp_topframe, rettv->vval.v_list, TRUE);
}

/*
 * "winline()" function
 */
    void
f_winline(typval_T *argvars UNUSED, typval_T *rettv)
{
    validate_cursor();
    rettv->vval.v_number = curwin->w_wrow + 1;
}

/*
 * "winnr()" function
 */
    void
f_winnr(typval_T *argvars UNUSED, typval_T *rettv)
{
    int		nr = 1;

    if (in_vim9script() && check_for_opt_string_arg(argvars, 0) == FAIL)
	return;

    nr = get_winnr(curtab, &argvars[0]);
    rettv->vval.v_number = nr;
}

/*
 * "winrestcmd()" function
 */
    void
f_winrestcmd(typval_T *argvars UNUSED, typval_T *rettv)
{
    win_T	*wp;
    int		i;
    int		winnr;
    garray_T	ga;
    char_u	buf[50];

    ga_init2(&ga, sizeof(char), 70);

    // Do this twice to handle some window layouts properly.
    for (i = 0; i < 2; ++i)
    {
	winnr = 1;
	FOR_ALL_WINDOWS(wp)
	{
	    sprintf((char *)buf, ":%dresize %d|", winnr, wp->w_height);
	    ga_concat(&ga, buf);
	    sprintf((char *)buf, "vert :%dresize %d|", winnr, wp->w_width);
	    ga_concat(&ga, buf);
	    ++winnr;
	}
    }
    ga_append(&ga, NUL);

    rettv->vval.v_string = ga.ga_data;
    rettv->v_type = VAR_STRING;
}

/*
 * "winrestview()" function
 */
    void
f_winrestview(typval_T *argvars, typval_T *rettv UNUSED)
{
    dict_T	*dict;

    if (check_for_nonnull_dict_arg(argvars, 0) == FAIL)
	return;

    dict = argvars[0].vval.v_dict;
    if (dict_has_key(dict, "lnum"))
	curwin->w_cursor.lnum = (linenr_T)dict_get_number(dict, "lnum");
    if (dict_has_key(dict, "col"))
	curwin->w_cursor.col = (colnr_T)dict_get_number(dict, "col");
    if (dict_has_key(dict, "coladd"))
	curwin->w_cursor.coladd = (colnr_T)dict_get_number(dict, "coladd");
    if (dict_has_key(dict, "curswant"))
    {
	curwin->w_curswant = (colnr_T)dict_get_number(dict, "curswant");
	curwin->w_set_curswant = FALSE;
    }

    if (dict_has_key(dict, "topline"))
	set_topline(curwin, (linenr_T)dict_get_number(dict, "topline"));
#ifdef FEAT_DIFF
    if (dict_has_key(dict, "topfill"))
	curwin->w_topfill = (int)dict_get_number(dict, "topfill");
#endif
    if (dict_has_key(dict, "leftcol"))
	curwin->w_leftcol = (colnr_T)dict_get_number(dict, "leftcol");
    if (dict_has_key(dict, "skipcol"))
	curwin->w_skipcol = (colnr_T)dict_get_number(dict, "skipcol");

    check_cursor();
    win_new_height(curwin, curwin->w_height);
    win_new_width(curwin, curwin->w_width);
    changed_window_setting();

    if (curwin->w_topline <= 0)
	curwin->w_topline = 1;
    if (curwin->w_topline > curbuf->b_ml.ml_line_count)
	curwin->w_topline = curbuf->b_ml.ml_line_count;
#ifdef FEAT_DIFF
    check_topfill(curwin, TRUE);
#endif
}

/*
 * "winsaveview()" function
 */
    void
f_winsaveview(typval_T *argvars UNUSED, typval_T *rettv)
{
    dict_T	*dict;

    if (rettv_dict_alloc(rettv) == FAIL)
	return;
    dict = rettv->vval.v_dict;

    dict_add_number(dict, "lnum", (long)curwin->w_cursor.lnum);
    dict_add_number(dict, "col", (long)curwin->w_cursor.col);
    dict_add_number(dict, "coladd", (long)curwin->w_cursor.coladd);
    update_curswant();
    dict_add_number(dict, "curswant", (long)curwin->w_curswant);

    dict_add_number(dict, "topline", (long)curwin->w_topline);
#ifdef FEAT_DIFF
    dict_add_number(dict, "topfill", (long)curwin->w_topfill);
#endif
    dict_add_number(dict, "leftcol", (long)curwin->w_leftcol);
    dict_add_number(dict, "skipcol", (long)curwin->w_skipcol);
}

/*
 * "winwidth(nr)" function
 */
    void
f_winwidth(typval_T *argvars, typval_T *rettv)
{
    win_T	*wp;

    if (in_vim9script() && check_for_number_arg(argvars, 0) == FAIL)
	return;

    wp = find_win_by_nr_or_id(&argvars[0]);
    if (wp == NULL)
	rettv->vval.v_number = -1;
    else
	rettv->vval.v_number = wp->w_width;
}
#endif // FEAT_EVAL

#if defined(FEAT_EVAL) || defined(FEAT_PYTHON) || defined(FEAT_PYTHON3) \
	|| defined(PROTO)
/*
 * Set "win" to be the curwin and "tp" to be the current tab page.
 * restore_win() MUST be called to undo, also when FAIL is returned.
 * No autocommands will be executed until restore_win() is called.
 * When "no_display" is TRUE the display won't be affected, no redraw is
 * triggered, another tabpage access is limited.
 * Returns FAIL if switching to "win" failed.
 */
    int
switch_win(
	switchwin_T *switchwin,
	win_T	    *win,
	tabpage_T   *tp,
	int	    no_display)
{
    block_autocmds();
    return switch_win_noblock(switchwin, win, tp, no_display);
}

/*
 * As switch_win() but without blocking autocommands.
 */
    int
switch_win_noblock(
	switchwin_T *switchwin,
	win_T	    *win,
	tabpage_T   *tp,
	int	    no_display)
{
    CLEAR_POINTER(switchwin);
    switchwin->sw_curwin = curwin;
    if (win == curwin)
	switchwin->sw_same_win = TRUE;
    else
    {
	// Disable Visual selection, because redrawing may fail.
	switchwin->sw_visual_active = VIsual_active;
	VIsual_active = FALSE;
    }

    if (tp != NULL)
    {
	switchwin->sw_curtab = curtab;
	if (no_display)
	{
	    curtab->tp_firstwin = firstwin;
	    curtab->tp_lastwin = lastwin;
	    curtab->tp_topframe = topframe;
	    curtab = tp;
	    firstwin = curtab->tp_firstwin;
	    lastwin = curtab->tp_lastwin;
	    topframe = curtab->tp_topframe;
	}
	else
	    goto_tabpage_tp(tp, FALSE, FALSE);
    }
    if (!win_valid(win))
	return FAIL;
    curwin = win;
    curbuf = curwin->w_buffer;
    return OK;
}

/*
 * Restore current tabpage and window saved by switch_win(), if still valid.
 * When "no_display" is TRUE the display won't be affected, no redraw is
 * triggered.
 */
    void
restore_win(
	switchwin_T *switchwin,
	int	    no_display)
{
    restore_win_noblock(switchwin, no_display);
    unblock_autocmds();
}

/*
 * As restore_win() but without unblocking autocommands.
 */
    void
restore_win_noblock(
	switchwin_T *switchwin,
	int	    no_display)
{
    if (switchwin->sw_curtab != NULL && valid_tabpage(switchwin->sw_curtab))
    {
	if (no_display)
	{
	    curtab->tp_firstwin = firstwin;
	    curtab->tp_lastwin = lastwin;
	    curtab->tp_topframe = topframe;
	    curtab = switchwin->sw_curtab;
	    firstwin = curtab->tp_firstwin;
	    lastwin = curtab->tp_lastwin;
	    topframe = curtab->tp_topframe;
	}
	else
	    goto_tabpage_tp(switchwin->sw_curtab, FALSE, FALSE);
    }

    if (!switchwin->sw_same_win)
	VIsual_active = switchwin->sw_visual_active;

    if (win_valid(switchwin->sw_curwin))
    {
	curwin = switchwin->sw_curwin;
	curbuf = curwin->w_buffer;
    }
# ifdef FEAT_PROP_POPUP
    else if (WIN_IS_POPUP(curwin))
	// original window was closed and now we're in a popup window: Go
	// to the first valid window.
	win_goto(firstwin);
# endif
}
#endif
