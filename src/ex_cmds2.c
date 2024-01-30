/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * ex_cmds2.c: some more functions for command line commands
 */

#include "vim.h"
#include "version.h"

/*
 * If 'autowrite' option set, try to write the file.
 * Careful: autocommands may make "buf" invalid!
 *
 * return FAIL for failure, OK otherwise
 */
    int
autowrite(buf_T *buf, int forceit)
{
    int		r;
    bufref_T	bufref;

    if (!(p_aw || p_awa) || !p_write
	    // never autowrite a "nofile" or "nowrite" buffer
	    || bt_dontwrite(buf)
	    || (!forceit && buf->b_p_ro) || buf->b_ffname == NULL)
	return FAIL;
    set_bufref(&bufref, buf);
    r = buf_write_all(buf, forceit);

    // Writing may succeed but the buffer still changed, e.g., when there is a
    // conversion error.  We do want to return FAIL then.
    if (bufref_valid(&bufref) && bufIsChanged(buf))
	r = FAIL;
    return r;
}

/*
 * Flush all buffers, except the ones that are readonly or are never written.
 */
    void
autowrite_all(void)
{
    buf_T	*buf;

    if (!(p_aw || p_awa) || !p_write)
	return;
    FOR_ALL_BUFFERS(buf)
	if (bufIsChanged(buf) && !buf->b_p_ro && !bt_dontwrite(buf))
	{
	    bufref_T	bufref;

	    set_bufref(&bufref, buf);

	    (void)buf_write_all(buf, FALSE);

	    // an autocommand may have deleted the buffer
	    if (!bufref_valid(&bufref))
		buf = firstbuf;
	}
}

/*
 * Return TRUE if buffer was changed and cannot be abandoned.
 * For flags use the CCGD_ values.
 */
    int
check_changed(buf_T *buf, int flags)
{
    int		forceit = (flags & CCGD_FORCEIT);
    bufref_T	bufref;

    set_bufref(&bufref, buf);

    if (       !forceit
	    && bufIsChanged(buf)
	    && ((flags & CCGD_MULTWIN) || buf->b_nwindows <= 1)
	    && (!(flags & CCGD_AW) || autowrite(buf, forceit) == FAIL))
    {
#if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
	if ((p_confirm || (cmdmod.cmod_flags & CMOD_CONFIRM)) && p_write)
	{
# ifdef FEAT_TERMINAL
	    if (term_job_running(buf->b_term))
	    {
		return term_confirm_stop(buf) == FAIL;
	    }
# endif

	    buf_T	*buf2;
	    int		count = 0;

	    if (flags & CCGD_ALLBUF)
		FOR_ALL_BUFFERS(buf2)
		    if (bufIsChanged(buf2)
				     && (buf2->b_ffname != NULL
# ifdef FEAT_BROWSE
					 || (cmdmod.cmod_flags & CMOD_BROWSE)
# endif
					))
			++count;
	    if (!bufref_valid(&bufref))
		// Autocommand deleted buffer, oops!  It's not changed now.
		return FALSE;

	    dialog_changed(buf, count > 1);

	    if (!bufref_valid(&bufref))
		// Autocommand deleted buffer, oops!  It's not changed now.
		return FALSE;
	    return bufIsChanged(buf);
	}
#endif
	if (flags & CCGD_EXCMD)
	    no_write_message();
	else
	    no_write_message_nobang(curbuf);
	return TRUE;
    }
    return FALSE;
}

#if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG) || defined(PROTO)

#if defined(FEAT_BROWSE) || defined(PROTO)
/*
 * When wanting to write a file without a file name, ask the user for a name.
 */
    void
browse_save_fname(buf_T *buf)
{
    if (buf->b_fname != NULL)
	return;

    char_u *fname;

    fname = do_browse(BROWSE_SAVE, (char_u *)_("Save As"),
	    NULL, NULL, NULL, NULL, buf);
    if (fname == NULL)
	return;

    if (setfname(buf, fname, NULL, TRUE) == OK)
	buf->b_flags |= BF_NOTEDITED;
    vim_free(fname);
}
#endif

/*
 * Ask the user what to do when abandoning a changed buffer.
 * Must check 'write' option first!
 */
    void
dialog_changed(
    buf_T	*buf,
    int		checkall)	// may abandon all changed buffers
{
    char_u	buff[DIALOG_MSG_SIZE];
    int		ret;
    buf_T	*buf2;
    exarg_T     ea;

    dialog_msg(buff, _("Save changes to \"%s\"?"), buf->b_fname);
    if (checkall)
	ret = vim_dialog_yesnoallcancel(VIM_QUESTION, NULL, buff, 1);
    else
	ret = vim_dialog_yesnocancel(VIM_QUESTION, NULL, buff, 1);

    // Init ea pseudo-structure, this is needed for the check_overwrite()
    // function.
    CLEAR_FIELD(ea);

    if (ret == VIM_YES)
    {
#ifdef FEAT_BROWSE
	// May get file name, when there is none
	browse_save_fname(buf);
#endif
	if (buf->b_fname != NULL && check_overwrite(&ea, buf,
				    buf->b_fname, buf->b_ffname, FALSE) == OK)
	    // didn't hit Cancel
	    (void)buf_write_all(buf, FALSE);
    }
    else if (ret == VIM_NO)
    {
	unchanged(buf, TRUE, FALSE);
    }
    else if (ret == VIM_ALL)
    {
	/*
	 * Write all modified files that can be written.
	 * Skip readonly buffers, these need to be confirmed
	 * individually.
	 */
	FOR_ALL_BUFFERS(buf2)
	{
	    if (bufIsChanged(buf2)
		    && (buf2->b_ffname != NULL
#ifdef FEAT_BROWSE
			|| (cmdmod.cmod_flags & CMOD_BROWSE)
#endif
			)
		    && !bt_dontwrite(buf2)
		    && !buf2->b_p_ro)
	    {
		bufref_T bufref;

		set_bufref(&bufref, buf2);
#ifdef FEAT_BROWSE
		// May get file name, when there is none
		browse_save_fname(buf2);
#endif
		if (buf2->b_fname != NULL && check_overwrite(&ea, buf2,
				  buf2->b_fname, buf2->b_ffname, FALSE) == OK)
		    // didn't hit Cancel
		    (void)buf_write_all(buf2, FALSE);

		// an autocommand may have deleted the buffer
		if (!bufref_valid(&bufref))
		    buf2 = firstbuf;
	    }
	}
    }
    else if (ret == VIM_DISCARDALL)
    {
	/*
	 * mark all buffers as unchanged
	 */
	FOR_ALL_BUFFERS(buf2)
	    unchanged(buf2, TRUE, FALSE);
    }
}
#endif

/*
 * Return TRUE if the buffer "buf" can be abandoned, either by making it
 * hidden, autowriting it or unloading it.
 */
    int
can_abandon(buf_T *buf, int forceit)
{
    return (	   buf_hide(buf)
		|| !bufIsChanged(buf)
		|| buf->b_nwindows > 1
		|| autowrite(buf, forceit) == OK
		|| forceit);
}

/*
 * Add a buffer number to "bufnrs", unless it's already there.
 */
    static void
add_bufnum(int *bufnrs, int *bufnump, int nr)
{
    int i;

    for (i = 0; i < *bufnump; ++i)
	if (bufnrs[i] == nr)
	    return;
    bufnrs[*bufnump] = nr;
    *bufnump = *bufnump + 1;
}

/*
 * Return TRUE if any buffer was changed and cannot be abandoned.
 * That changed buffer becomes the current buffer.
 * When "unload" is TRUE the current buffer is unloaded instead of making it
 * hidden.  This is used for ":q!".
 */
    int
check_changed_any(
    int		hidden,		// Only check hidden buffers
    int		unload)
{
    int		ret = FALSE;
    buf_T	*buf;
    int		save;
    int		i;
    int		bufnum = 0;
    int		bufcount = 0;
    int		*bufnrs;
    tabpage_T   *tp;
    win_T	*wp;

    // Make a list of all buffers, with the most important ones first.
    FOR_ALL_BUFFERS(buf)
	++bufcount;

    if (bufcount == 0)
	return FALSE;

    bufnrs = ALLOC_MULT(int, bufcount);
    if (bufnrs == NULL)
	return FALSE;

    // curbuf
    bufnrs[bufnum++] = curbuf->b_fnum;

    // buffers in current tab
    FOR_ALL_WINDOWS(wp)
	if (wp->w_buffer != curbuf)
	    add_bufnum(bufnrs, &bufnum, wp->w_buffer->b_fnum);

    // buffers in other tabs
    FOR_ALL_TABPAGES(tp)
	if (tp != curtab)
	    FOR_ALL_WINDOWS_IN_TAB(tp, wp)
		add_bufnum(bufnrs, &bufnum, wp->w_buffer->b_fnum);

    // any other buffer
    FOR_ALL_BUFFERS(buf)
	add_bufnum(bufnrs, &bufnum, buf->b_fnum);

    for (i = 0; i < bufnum; ++i)
    {
	buf = buflist_findnr(bufnrs[i]);
	if (buf == NULL)
	    continue;
	if ((!hidden || buf->b_nwindows == 0) && bufIsChanged(buf))
	{
	    bufref_T bufref;

	    set_bufref(&bufref, buf);
#ifdef FEAT_TERMINAL
	    if (term_job_running(buf->b_term))
	    {
		if (term_try_stop_job(buf) == FAIL)
		    break;
	    }
	    else
#endif
	    // Try auto-writing the buffer.  If this fails but the buffer no
	    // longer exists it's not changed, that's OK.
	    if (check_changed(buf, (p_awa ? CCGD_AW : 0)
				 | CCGD_MULTWIN
				 | CCGD_ALLBUF) && bufref_valid(&bufref))
		break;	    // didn't save - still changes
	}
    }

    if (i >= bufnum)
	goto theend;

    // Get here if "buf" cannot be abandoned.
    ret = TRUE;
    exiting = FALSE;
#if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
    /*
     * When ":confirm" used, don't give an error message.
     */
    if (!(p_confirm || (cmdmod.cmod_flags & CMOD_CONFIRM)))
#endif
    {
	// There must be a wait_return() for this message, do_buffer()
	// may cause a redraw.  But wait_return() is a no-op when vgetc()
	// is busy (Quit used from window menu), then make sure we don't
	// cause a scroll up.
	if (vgetc_busy > 0)
	{
	    msg_row = cmdline_row;
	    msg_col = 0;
	    msg_didout = FALSE;
	}
	if (
#ifdef FEAT_TERMINAL
		term_job_running(buf->b_term)
		    ? semsg(_(e_job_still_running_in_buffer_str), buf->b_fname)
		    :
#endif
		semsg(_(e_no_write_since_last_change_for_buffer_str),
		    buf_spname(buf) != NULL ? buf_spname(buf) : buf->b_fname))
	{
	    save = no_wait_return;
	    no_wait_return = FALSE;
	    wait_return(FALSE);
	    no_wait_return = save;
	}
    }

    // Try to find a window that contains the buffer.
    if (buf != curbuf)
	FOR_ALL_TAB_WINDOWS(tp, wp)
	    if (wp->w_buffer == buf)
	    {
		bufref_T bufref;

		set_bufref(&bufref, buf);

		goto_tabpage_win(tp, wp);

		// Paranoia: did autocmd wipe out the buffer with changes?
		if (!bufref_valid(&bufref))
		    goto theend;
		goto buf_found;
	    }
buf_found:

    // Open the changed buffer in the current window.
    if (buf != curbuf)
	set_curbuf(buf, unload ? DOBUF_UNLOAD : DOBUF_GOTO);

theend:
    vim_free(bufnrs);
    return ret;
}

/*
 * return FAIL if there is no file name, OK if there is one
 * give error message for FAIL
 */
    int
check_fname(void)
{
    if (curbuf->b_ffname == NULL)
    {
	emsg(_(e_no_file_name));
	return FAIL;
    }
    return OK;
}

/*
 * flush the contents of a buffer, unless it has no file name
 *
 * return FAIL for failure, OK otherwise
 */
    int
buf_write_all(buf_T *buf, int forceit)
{
    int	    retval;
    buf_T	*old_curbuf = curbuf;

    retval = (buf_write(buf, buf->b_ffname, buf->b_fname,
				   (linenr_T)1, buf->b_ml.ml_line_count, NULL,
						  FALSE, forceit, TRUE, FALSE));
    if (curbuf != old_curbuf)
    {
	msg_source(HL_ATTR(HLF_W));
	msg(_("Warning: Entered other buffer unexpectedly (check autocommands)"));
    }
    return retval;
}

/*
 * ":argdo", ":windo", ":bufdo", ":tabdo", ":cdo", ":ldo", ":cfdo" and ":lfdo"
 */
    void
ex_listdo(exarg_T *eap)
{
    int		i;
    win_T	*wp;
    tabpage_T	*tp;
    buf_T	*buf = curbuf;
    int		next_fnum = 0;
#if defined(FEAT_SYN_HL)
    char_u	*save_ei = NULL;
#endif
#ifdef FEAT_QUICKFIX
    int		qf_size = 0;
    int		qf_idx;
#endif

#ifndef FEAT_QUICKFIX
    if (eap->cmdidx == CMD_cdo || eap->cmdidx == CMD_ldo ||
	    eap->cmdidx == CMD_cfdo || eap->cmdidx == CMD_lfdo)
    {
	ex_ni(eap);
	return;
    }
#endif

#if defined(FEAT_SYN_HL)
    if (eap->cmdidx != CMD_windo && eap->cmdidx != CMD_tabdo)
    {
	// Don't do syntax HL autocommands.  Skipping the syntax file is a
	// great speed improvement.
	save_ei = au_event_disable(",Syntax");

	FOR_ALL_BUFFERS(buf)
	    buf->b_flags &= ~BF_SYN_SET;
	buf = curbuf;
    }
#endif
#ifdef FEAT_CLIPBOARD
    start_global_changes();
#endif

    if (eap->cmdidx == CMD_windo
	    || eap->cmdidx == CMD_tabdo
	    || buf_hide(curbuf)
	    || !check_changed(curbuf, CCGD_AW
				    | (eap->forceit ? CCGD_FORCEIT : 0)
				    | CCGD_EXCMD))
    {
	i = 0;
	// start at the eap->line1 argument/window/buffer
	wp = firstwin;
	tp = first_tabpage;
	switch (eap->cmdidx)
	{
	    case CMD_windo:
		for ( ; wp != NULL && i + 1 < eap->line1; wp = wp->w_next)
		    i++;
		break;
	    case CMD_tabdo:
		for ( ; tp != NULL && i + 1 < eap->line1; tp = tp->tp_next)
		    i++;
		break;
	    case CMD_argdo:
		i = eap->line1 - 1;
		break;
	    default:
		break;
	}
	// set pcmark now
	if (eap->cmdidx == CMD_bufdo)
	{
	    // Advance to the first listed buffer after "eap->line1".
	    for (buf = firstbuf; buf != NULL && (buf->b_fnum < eap->line1
					  || !buf->b_p_bl); buf = buf->b_next)
		if (buf->b_fnum > eap->line2)
		{
		    buf = NULL;
		    break;
		}
	    if (buf != NULL)
		goto_buffer(eap, DOBUF_FIRST, FORWARD, buf->b_fnum);
	}
#ifdef FEAT_QUICKFIX
	else if (eap->cmdidx == CMD_cdo || eap->cmdidx == CMD_ldo
		|| eap->cmdidx == CMD_cfdo || eap->cmdidx == CMD_lfdo)
	{
	    qf_size = qf_get_valid_size(eap);
	    if (qf_size <= 0 || eap->line1 > qf_size)
		buf = NULL;
	    else
	    {
		save_clear_shm_value();
		ex_cc(eap);
		restore_shm_value();

		buf = curbuf;
		i = eap->line1 - 1;
		if (eap->addr_count <= 0)
		    // default is all the quickfix/location list entries
		    eap->line2 = qf_size;
	    }
	}
#endif
	else
	    setpcmark();
	listcmd_busy = TRUE;	    // avoids setting pcmark below

	while (!got_int && buf != NULL)
	{
	    if (eap->cmdidx == CMD_argdo)
	    {
		// go to argument "i"
		if (i == ARGCOUNT)
		    break;
		// Don't call do_argfile() when already there, it will try
		// reloading the file.
		if (curwin->w_arg_idx != i || !editing_arg_idx(curwin))
		{
		    // Clear 'shm' to avoid that the file message overwrites
		    // any output from the command.
		    save_clear_shm_value();
		    do_argfile(eap, i);
		    restore_shm_value();
		}
		if (curwin->w_arg_idx != i)
		    break;
	    }
	    else if (eap->cmdidx == CMD_windo)
	    {
		// go to window "wp"
		if (!win_valid(wp))
		    break;
		win_goto(wp);
		if (curwin != wp)
		    break;  // something must be wrong
		wp = curwin->w_next;
	    }
	    else if (eap->cmdidx == CMD_tabdo)
	    {
		// go to window "tp"
		if (!valid_tabpage(tp))
		    break;
		goto_tabpage_tp(tp, TRUE, TRUE);
		tp = tp->tp_next;
	    }
	    else if (eap->cmdidx == CMD_bufdo)
	    {
		// Remember the number of the next listed buffer, in case
		// ":bwipe" is used or autocommands do something strange.
		next_fnum = -1;
		for (buf = curbuf->b_next; buf != NULL; buf = buf->b_next)
		    if (buf->b_p_bl)
		    {
			next_fnum = buf->b_fnum;
			break;
		    }
	    }

	    ++i;

	    // execute the command
	    do_cmdline(eap->arg, eap->ea_getline, eap->cookie,
						DOCMD_VERBOSE + DOCMD_NOWAIT);

	    if (eap->cmdidx == CMD_bufdo)
	    {
		// Done?
		if (next_fnum < 0 || next_fnum > eap->line2)
		    break;
		// Check if the buffer still exists.
		FOR_ALL_BUFFERS(buf)
		    if (buf->b_fnum == next_fnum)
			break;
		if (buf == NULL)
		    break;

		// Go to the next buffer.  Clear 'shm' to avoid that the file
		// message overwrites any output from the command.
		save_clear_shm_value();
		goto_buffer(eap, DOBUF_FIRST, FORWARD, next_fnum);
		restore_shm_value();

		// If autocommands took us elsewhere, quit here.
		if (curbuf->b_fnum != next_fnum)
		    break;
	    }

#ifdef FEAT_QUICKFIX
	    if (eap->cmdidx == CMD_cdo || eap->cmdidx == CMD_ldo
		    || eap->cmdidx == CMD_cfdo || eap->cmdidx == CMD_lfdo)
	    {
		if (i >= qf_size || i >= eap->line2)
		    break;

		qf_idx = qf_get_cur_idx(eap);

		save_clear_shm_value();
		ex_cnext(eap);
		restore_shm_value();

		// If jumping to the next quickfix entry fails, quit here
		if (qf_get_cur_idx(eap) == qf_idx)
		    break;
	    }
#endif

	    if (eap->cmdidx == CMD_windo)
	    {
		validate_cursor();	// cursor may have moved

		// required when 'scrollbind' has been set
		if (curwin->w_p_scb)
		    do_check_scrollbind(TRUE);
	    }

	    if (eap->cmdidx == CMD_windo || eap->cmdidx == CMD_tabdo)
		if (i+1 > eap->line2)
		    break;
	    if (eap->cmdidx == CMD_argdo && i >= eap->line2)
		break;
	}
	listcmd_busy = FALSE;
    }

#if defined(FEAT_SYN_HL)
    if (save_ei != NULL)
    {
	buf_T		*bnext;
	aco_save_T	aco;

	au_event_restore(save_ei);

	for (buf = firstbuf; buf != NULL; buf = bnext)
	{
	    bnext = buf->b_next;
	    if (buf->b_nwindows > 0 && (buf->b_flags & BF_SYN_SET))
	    {
		buf->b_flags &= ~BF_SYN_SET;

		// buffer was opened while Syntax autocommands were disabled,
		// need to trigger them now.
		if (buf == curbuf)
		    apply_autocmds(EVENT_SYNTAX, curbuf->b_p_syn,
					       curbuf->b_fname, TRUE, curbuf);
		else
		{
		    aucmd_prepbuf(&aco, buf);
		    if (curbuf == buf)
		    {
			apply_autocmds(EVENT_SYNTAX, buf->b_p_syn,
						      buf->b_fname, TRUE, buf);
			aucmd_restbuf(&aco);
		    }
		}

		// start over, in case autocommands messed things up.
		bnext = firstbuf;
	    }
	}
    }
#endif
#ifdef FEAT_CLIPBOARD
    end_global_changes();
#endif
}

#ifdef FEAT_EVAL
/*
 * ":compiler[!] {name}"
 */
    void
ex_compiler(exarg_T *eap)
{
    char_u	*buf;
    char_u	*old_cur_comp = NULL;
    char_u	*p;

    if (*eap->arg == NUL)
    {
	// List all compiler scripts.
	do_cmdline_cmd((char_u *)"echo globpath(&rtp, 'compiler/*.vim')");
					// ) keep the indenter happy...
	return;
    }

    buf = alloc(STRLEN(eap->arg) + 14);
    if (buf == NULL)
	return;

    if (eap->forceit)
    {
	// ":compiler! {name}" sets global options
	do_cmdline_cmd((char_u *)
		"command -nargs=* CompilerSet set <args>");
    }
    else
    {
	// ":compiler! {name}" sets local options.
	// To remain backwards compatible "current_compiler" is always
	// used.  A user's compiler plugin may set it, the distributed
	// plugin will then skip the settings.  Afterwards set
	// "b:current_compiler" and restore "current_compiler".
	// Explicitly prepend "g:" to make it work in a function.
	old_cur_comp = get_var_value((char_u *)"g:current_compiler");
	if (old_cur_comp != NULL)
	    old_cur_comp = vim_strsave(old_cur_comp);
	do_cmdline_cmd((char_u *)
		"command -nargs=* -keepscript CompilerSet setlocal <args>");
    }
    do_unlet((char_u *)"g:current_compiler", TRUE);
    do_unlet((char_u *)"b:current_compiler", TRUE);

    sprintf((char *)buf, "compiler/%s.vim", eap->arg);
    if (source_runtime(buf, DIP_ALL) == FAIL)
	semsg(_(e_compiler_not_supported_str), eap->arg);
    vim_free(buf);

    do_cmdline_cmd((char_u *)":delcommand CompilerSet");

    // Set "b:current_compiler" from "current_compiler".
    p = get_var_value((char_u *)"g:current_compiler");
    if (p != NULL)
	set_internal_string_var((char_u *)"b:current_compiler", p);

    // Restore "current_compiler" for ":compiler {name}".
    if (!eap->forceit)
    {
	if (old_cur_comp != NULL)
	{
	    set_internal_string_var((char_u *)"g:current_compiler",
		    old_cur_comp);
	    vim_free(old_cur_comp);
	}
	else
	    do_unlet((char_u *)"g:current_compiler", TRUE);
    }
}
#endif

#if defined(FEAT_PYTHON3) || defined(FEAT_PYTHON) || defined(PROTO)

# if (defined(FEAT_PYTHON) && defined(FEAT_PYTHON3)) || defined(PROTO)
/*
 * Detect Python 3 or 2, and initialize 'pyxversion'.
 */
    void
init_pyxversion(void)
{
    if (p_pyx == 0)
    {
	if (python3_enabled(FALSE))
	    p_pyx = 3;
	else if (python_enabled(FALSE))
	    p_pyx = 2;
    }
}
# endif

/*
 * Does a file contain one of the following strings at the beginning of any
 * line?
 * "#!(any string)python2"  => returns 2
 * "#!(any string)python3"  => returns 3
 * "# requires python 2.x"  => returns 2
 * "# requires python 3.x"  => returns 3
 * otherwise return 0.
 */
    static int
requires_py_version(char_u *filename)
{
    FILE    *file;
    int	    requires_py_version = 0;
    int	    i, lines;

    lines = (int)p_mls;
    if (lines < 0)
	lines = 5;

    file = mch_fopen((char *)filename, "r");
    if (file == NULL)
	return 0;

    for (i = 0; i < lines; i++)
    {
	if (vim_fgets(IObuff, IOSIZE, file))
	    break;
	if (i == 0 && IObuff[0] == '#' && IObuff[1] == '!')
	{
	    // Check shebang.
	    if (strstr((char *)IObuff + 2, "python2") != NULL)
	    {
		requires_py_version = 2;
		break;
	    }
	    if (strstr((char *)IObuff + 2, "python3") != NULL)
	    {
		requires_py_version = 3;
		break;
	    }
	}
	IObuff[21] = '\0';
	if (STRCMP("# requires python 2.x", IObuff) == 0)
	{
	    requires_py_version = 2;
	    break;
	}
	if (STRCMP("# requires python 3.x", IObuff) == 0)
	{
	    requires_py_version = 3;
	    break;
	}
    }
    fclose(file);
    return requires_py_version;
}


/*
 * Source a python file using the requested python version.
 */
    static void
source_pyx_file(exarg_T *eap, char_u *fname)
{
    exarg_T ex;
    int	    v = requires_py_version(fname);

# if defined(FEAT_PYTHON) && defined(FEAT_PYTHON3)
    init_pyxversion();
# endif
    if (v == 0)
    {
# if defined(FEAT_PYTHON) && defined(FEAT_PYTHON3)
	// user didn't choose a preference, 'pyx' is used
	v = p_pyx;
# elif defined(FEAT_PYTHON)
	v = 2;
# elif defined(FEAT_PYTHON3)
	v = 3;
# endif
    }

    /*
     * now source, if required python version is not supported show
     * unobtrusive message.
     */
    if (eap == NULL)
	CLEAR_FIELD(ex);
    else
	ex = *eap;
    ex.arg = fname;
    ex.cmd = (char_u *)(v == 2 ? "pyfile" : "pyfile3");

    if (v == 2)
    {
# ifdef FEAT_PYTHON
	ex_pyfile(&ex);
# else
	vim_snprintf((char *)IObuff, IOSIZE,
		_("W20: Required python version 2.x not supported, ignoring file: %s"),
		fname);
	msg((char *)IObuff);
# endif
	return;
    }
    else
    {
# ifdef FEAT_PYTHON3
	ex_py3file(&ex);
# else
	vim_snprintf((char *)IObuff, IOSIZE,
		_("W21: Required python version 3.x not supported, ignoring file: %s"),
		fname);
	msg((char *)IObuff);
# endif
	return;
    }
}

/*
 * ":pyxfile {fname}"
 */
    void
ex_pyxfile(exarg_T *eap)
{
    source_pyx_file(eap, eap->arg);
}

/*
 * ":pyx"
 */
    void
ex_pyx(exarg_T *eap)
{
# if defined(FEAT_PYTHON) && defined(FEAT_PYTHON3)
    init_pyxversion();
    if (p_pyx == 2)
	ex_python(eap);
    else
	ex_py3(eap);
# elif defined(FEAT_PYTHON)
    ex_python(eap);
# elif defined(FEAT_PYTHON3)
    ex_py3(eap);
# endif
}

/*
 * ":pyxdo"
 */
    void
ex_pyxdo(exarg_T *eap)
{
# if defined(FEAT_PYTHON) && defined(FEAT_PYTHON3)
    init_pyxversion();
    if (p_pyx == 2)
	ex_pydo(eap);
    else
	ex_py3do(eap);
# elif defined(FEAT_PYTHON)
    ex_pydo(eap);
# elif defined(FEAT_PYTHON3)
    ex_py3do(eap);
# endif
}

#endif

/*
 * ":checktime [buffer]"
 */
    void
ex_checktime(exarg_T *eap)
{
    buf_T	*buf;
    int		save_no_check_timestamps = no_check_timestamps;

    no_check_timestamps = 0;
    if (eap->addr_count == 0)	// default is all buffers
	check_timestamps(FALSE);
    else
    {
	buf = buflist_findnr((int)eap->line2);
	if (buf != NULL)	// cannot happen?
	    (void)buf_check_timestamp(buf, FALSE);
    }
    no_check_timestamps = save_no_check_timestamps;
}
