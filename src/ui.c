/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * ui.c: functions that handle the user interface.
 * 1. Keyboard input stuff, and a bit of windowing stuff.  These are called
 *    before the machine specific stuff (mch_*) so that we can call the GUI
 *    stuff instead if the GUI is running.
 * 2. Input buffer stuff.
 */

#include "vim.h"

    void
ui_write(char_u *s, int len, int console UNUSED)
{
#ifdef FEAT_GUI
    if (gui.in_use && !gui.dying && !gui.starting
# ifndef NO_CONSOLE
	    && !console
# endif
	    )
    {
	gui_write(s, len);
	if (p_wd)
	    gui_wait_for_chars(p_wd, typebuf.tb_change_cnt);
	return;
    }
#endif
#ifndef NO_CONSOLE
    // Don't output anything in silent mode ("ex -s") unless 'verbose' set
    if (!(silent_mode && p_verbose == 0))
    {
# if !defined(MSWIN)
	char_u	*tofree = NULL;

	if (output_conv.vc_type != CONV_NONE)
	{
	    // Convert characters from 'encoding' to 'termencoding'.
	    tofree = string_convert(&output_conv, s, &len);
	    if (tofree != NULL)
		s = tofree;
	}
# endif

	mch_write(s, len);
# if defined(HAVE_FSYNC)
	if (console && s[len - 1] == '\n')
	    vim_fsync(1);
# endif

# if !defined(MSWIN)
	if (output_conv.vc_type != CONV_NONE)
	    vim_free(tofree);
# endif
    }
#endif
}

#if defined(UNIX) || defined(VMS) || defined(PROTO) || defined(MSWIN)
/*
 * When executing an external program, there may be some typed characters that
 * are not consumed by it.  Give them back to ui_inchar() and they are stored
 * here for the next call.
 */
static char_u *ta_str = NULL;
static int ta_off;	// offset for next char to use when ta_str != NULL
static int ta_len;	// length of ta_str when it's not NULL

    void
ui_inchar_undo(char_u *s, int len)
{
    char_u  *new;
    int	    newlen;

    newlen = len;
    if (ta_str != NULL)
	newlen += ta_len - ta_off;
    new = alloc(newlen);
    if (new == NULL)
	return;

    if (ta_str != NULL)
    {
	mch_memmove(new, ta_str + ta_off, (size_t)(ta_len - ta_off));
	mch_memmove(new + ta_len - ta_off, s, (size_t)len);
	vim_free(ta_str);
    }
    else
	mch_memmove(new, s, (size_t)len);
    ta_str = new;
    ta_len = newlen;
    ta_off = 0;
}
#endif

/*
 * ui_inchar(): low level input function.
 * Get characters from the keyboard.
 * Return the number of characters that are available.
 * If "wtime" == 0 do not wait for characters.
 * If "wtime" == -1 wait forever for characters.
 * If "wtime" > 0 wait "wtime" milliseconds for a character.
 *
 * "tb_change_cnt" is the value of typebuf.tb_change_cnt if "buf" points into
 * it.  When typebuf.tb_change_cnt changes (e.g., when a message is received
 * from a remote client) "buf" can no longer be used.  "tb_change_cnt" is NULL
 * otherwise.
 */
    int
ui_inchar(
    char_u	*buf,
    int		maxlen,
    long	wtime,	    // don't use "time", MIPS cannot handle it
    int		tb_change_cnt)
{
    int		retval = 0;

#if defined(FEAT_GUI) && (defined(UNIX) || defined(VMS))
    /*
     * Use the typeahead if there is any.
     */
    if (ta_str != NULL)
    {
	if (maxlen >= ta_len - ta_off)
	{
	    mch_memmove(buf, ta_str + ta_off, (size_t)ta_len);
	    VIM_CLEAR(ta_str);
	    return ta_len;
	}
	mch_memmove(buf, ta_str + ta_off, (size_t)maxlen);
	ta_off += maxlen;
	return maxlen;
    }
#endif

#ifdef FEAT_PROFILE
    if (do_profiling == PROF_YES && wtime != 0)
	prof_inchar_enter();
#endif

#ifdef NO_CONSOLE_INPUT
    // Don't wait for character input when the window hasn't been opened yet.
    // Do try reading, this works when redirecting stdin from a file.
    // Must return something, otherwise we'll loop forever.  If we run into
    // this very often we probably got stuck, exit Vim.
    if (no_console_input())
    {
	static int count = 0;

# ifndef NO_CONSOLE
	retval = mch_inchar(buf, maxlen, wtime, tb_change_cnt);
	if (retval > 0 || typebuf_changed(tb_change_cnt) || wtime >= 0)
	    goto theend;
# endif
	if (wtime == -1 && ++count == 1000)
	    read_error_exit();
	buf[0] = CAR;
	retval = 1;
	goto theend;
    }
#endif

    // If we are going to wait for some time or block...
    if (wtime == -1 || wtime > 100L)
    {
	// ... allow signals to kill us.
	(void)vim_handle_signal(SIGNAL_UNBLOCK);

	// ... there is no need for CTRL-C to interrupt something, don't let
	// it set got_int when it was mapped.
	if ((mapped_ctrl_c | curbuf->b_mapped_ctrl_c) & get_real_state())
	    ctrl_c_interrupts = FALSE;
    }

    /*
     * Here we call gui_inchar() or mch_inchar(), the GUI or machine-dependent
     * input function.  The functionality they implement is like this:
     *
     * while (not timed out)
     * {
     *    handle-resize;
     *    parse-queued-messages;
     *    if (waited for 'updatetime')
     *       trigger-cursorhold;
     *    ui_wait_for_chars_or_timer()
     *    if (character available)
     *      break;
     * }
     *
     * ui_wait_for_chars_or_timer() does:
     *
     * while (not timed out)
     * {
     *     if (any-timer-triggered)
     *	      invoke-timer-callback;
     *     wait-for-character();
     *     if (character available)
     *	      break;
     * }
     *
     * wait-for-character() does:
     * while (not timed out)
     * {
     *     Wait for event;
     *     if (something on channel)
     *	      read/write channel;
     *	   else if (resized)
     *	      handle_resize();
     *	   else if (system event)
     *	      deal-with-system-event;
     *	   else if (character available)
     *	      break;
     * }
     *
     */

#ifdef FEAT_GUI
    if (gui.in_use)
	retval = gui_inchar(buf, maxlen, wtime, tb_change_cnt);
#endif
#ifndef NO_CONSOLE
# ifdef FEAT_GUI
    else
# endif
	retval = mch_inchar(buf, maxlen, wtime, tb_change_cnt);
#endif

    if (wtime == -1 || wtime > 100L)
	// block SIGHUP et al.
	(void)vim_handle_signal(SIGNAL_BLOCK);

    ctrl_c_interrupts = TRUE;

#ifdef NO_CONSOLE_INPUT
theend:
#endif
#ifdef FEAT_PROFILE
    if (do_profiling == PROF_YES && wtime != 0)
	prof_inchar_exit();
#endif
    return retval;
}

#if defined(UNIX) || defined(VMS) || defined(FEAT_GUI) || defined(PROTO)
/*
 * Common code for mch_inchar() and gui_inchar(): Wait for a while or
 * indefinitely until characters are available, dealing with timers and
 * messages on channels.
 *
 * "buf" may be NULL if the available characters are not to be returned, only
 * check if they are available.
 *
 * Return the number of characters that are available.
 * If "wtime" == 0 do not wait for characters.
 * If "wtime" == n wait a short time for characters.
 * If "wtime" == -1 wait forever for characters.
 */
    int
inchar_loop(
    char_u	*buf,
    int		maxlen,
    long	wtime,	    // don't use "time", MIPS cannot handle it
    int		tb_change_cnt,
    int		(*wait_func)(long wtime, int *interrupted, int ignore_input),
    int		(*resize_func)(int check_only))
{
    int		len;
    int		interrupted = FALSE;
    int		did_call_wait_func = FALSE;
    int		did_start_blocking = FALSE;
    long	wait_time;
    long	elapsed_time = 0;
#ifdef ELAPSED_FUNC
    elapsed_T	start_tv;

    ELAPSED_INIT(start_tv);
#endif

    // repeat until we got a character or waited long enough
    for (;;)
    {
	// Check if window changed size while we were busy, perhaps the ":set
	// columns=99" command was used.
	if (resize_func != NULL)
	    resize_func(FALSE);

#ifdef MESSAGE_QUEUE
	// Only process messages when waiting.
	if (wtime != 0)
	{
	    parse_queued_messages();
	    // If input was put directly in typeahead buffer bail out here.
	    if (typebuf_changed(tb_change_cnt))
		return 0;
	}
#endif
	if (wtime < 0 && did_start_blocking)
	    // blocking and already waited for p_ut
	    wait_time = -1;
	else
	{
	    if (wtime >= 0)
		wait_time = wtime;
	    else
		// going to block after p_ut
		wait_time = p_ut;
#ifdef ELAPSED_FUNC
	    elapsed_time = ELAPSED_FUNC(start_tv);
#endif
	    wait_time -= elapsed_time;

	    // If the waiting time is now zero or less, we timed out.  However,
	    // loop at least once to check for characters and events.  Matters
	    // when "wtime" is zero.
	    if (wait_time <= 0 && did_call_wait_func)
	    {
		if (wtime >= 0)
		    // no character available within "wtime"
		    return 0;

		// No character available within 'updatetime'.
		did_start_blocking = TRUE;
		if (trigger_cursorhold() && maxlen >= 3
					    && !typebuf_changed(tb_change_cnt))
		{
		    // Put K_CURSORHOLD in the input buffer or return it.
		    if (buf == NULL)
		    {
			char_u	ibuf[3];

			ibuf[0] = CSI;
			ibuf[1] = KS_EXTRA;
			ibuf[2] = (int)KE_CURSORHOLD;
			add_to_input_buf(ibuf, 3);
		    }
		    else
		    {
			buf[0] = K_SPECIAL;
			buf[1] = KS_EXTRA;
			buf[2] = (int)KE_CURSORHOLD;
		    }
		    return 3;
		}

		// There is no character available within 'updatetime' seconds:
		// flush all the swap files to disk.  Also done when
		// interrupted by SIGWINCH.
		before_blocking();
		continue;
	    }
	}

#ifdef FEAT_JOB_CHANNEL
	if (wait_time < 0 || wait_time > 100L)
	{
	    // Checking if a job ended requires polling.  Do this at least
	    // every 100 msec.
	    if (has_pending_job())
		wait_time = 100L;

	    // If there is readahead then parse_queued_messages() timed out and
	    // we should call it again soon.
	    if (channel_any_readahead())
		wait_time = 10L;
	}
#endif
#ifdef FEAT_BEVAL_GUI
	if (p_beval && wait_time > 100L)
	    // The 'balloonexpr' may indirectly invoke a callback while waiting
	    // for a character, need to check often.
	    wait_time = 100L;
#endif

	// Wait for a character to be typed or another event, such as the winch
	// signal or an event on the monitored file descriptors.
	did_call_wait_func = TRUE;
	if (wait_func(wait_time, &interrupted, FALSE))
	{
	    // If input was put directly in typeahead buffer bail out here.
	    if (typebuf_changed(tb_change_cnt))
		return 0;

	    // We might have something to return now.
	    if (buf == NULL)
		// "buf" is NULL, we were just waiting, not actually getting
		// input.
		return input_available();

	    len = read_from_input_buf(buf, (long)maxlen);
	    if (len > 0)
		return len;
	    continue;
	}
	// Timed out or interrupted with no character available.

#ifndef ELAPSED_FUNC
	// estimate the elapsed time
	elapsed_time += wait_time;
#endif

	if ((resize_func != NULL && resize_func(TRUE))
#if defined(FEAT_CLIENTSERVER) && defined(UNIX)
		|| server_waiting()
#endif
#ifdef MESSAGE_QUEUE
		|| interrupted
#endif
		|| wait_time > 0
		|| (wtime < 0 && !did_start_blocking))
	    // no character available, but something to be done, keep going
	    continue;

	// no character available or interrupted, return zero
	break;
    }
    return 0;
}
#endif

#if defined(FEAT_TIMERS) || defined(PROTO)
/*
 * Wait for a timer to fire or "wait_func" to return non-zero.
 * Returns OK when something was read.
 * Returns FAIL when it timed out or was interrupted.
 */
    int
ui_wait_for_chars_or_timer(
    long    wtime,
    int	    (*wait_func)(long wtime, int *interrupted, int ignore_input),
    int	    *interrupted,
    int	    ignore_input)
{
    int	    due_time;
    long    remaining = wtime;
    int	    tb_change_cnt = typebuf.tb_change_cnt;
# ifdef FEAT_JOB_CHANNEL
    int	    brief_wait = FALSE;
# endif

    // When waiting very briefly don't trigger timers.
    if (wtime >= 0 && wtime < 10L)
	return wait_func(wtime, NULL, ignore_input);

    while (wtime < 0 || remaining > 0)
    {
	// Trigger timers and then get the time in wtime until the next one is
	// due.  Wait up to that time.
	due_time = check_due_timer();
	if (typebuf.tb_change_cnt != tb_change_cnt)
	{
	    // timer may have used feedkeys()
	    return FAIL;
	}
	if (due_time <= 0 || (wtime > 0 && due_time > remaining))
	    due_time = remaining;
# if defined(FEAT_JOB_CHANNEL) || defined(FEAT_SOUND_CANBERRA) || defined(FEAT_SOUND_MACOSX)
	if ((due_time < 0 || due_time > 10L) && (
#  if defined(FEAT_JOB_CHANNEL)
		(
#   if defined(FEAT_GUI)
		!gui.in_use &&
#   endif
		(has_pending_job() || channel_any_readahead()))
#   if defined(FEAT_SOUND_CANBERRA) || defined(FEAT_SOUND_MACOSX)
		||
#   endif
#  endif
#  if defined(FEAT_SOUND_CANBERRA) ||  defined(FEAT_SOUND_MACOSX)
		    has_any_sound_callback()
#  endif
		    ))
	{
	    // There is a pending job or channel, should return soon in order
	    // to handle them ASAP.  Do check for input briefly.
	    due_time = 10L;
#  ifdef FEAT_JOB_CHANNEL
	    brief_wait = TRUE;
#  endif
	}
# endif
	if (wait_func(due_time, interrupted, ignore_input))
	    return OK;
	if ((interrupted != NULL && *interrupted)
# ifdef FEAT_JOB_CHANNEL
		|| brief_wait
# endif
		)
	    // Nothing available, but need to return so that side effects get
	    // handled, such as handling a message on a channel.
	    return FAIL;
	if (wtime > 0)
	    remaining -= due_time;
    }
    return FAIL;
}
#endif

/*
 * Return non-zero if a character is available.
 */
    int
ui_char_avail(void)
{
#ifdef FEAT_GUI
    if (gui.in_use)
    {
	gui_mch_update();
	return input_available();
    }
#endif
#ifndef NO_CONSOLE
# ifdef NO_CONSOLE_INPUT
    if (no_console_input())
	return 0;
# endif
    return mch_char_avail();
#else
    return 0;
#endif
}

/*
 * Delay for the given number of milliseconds.	If ignoreinput is FALSE then we
 * cancel the delay if a key is hit.
 */
    void
ui_delay(long msec_arg, int ignoreinput)
{
    long msec = msec_arg;

#ifdef FEAT_EVAL
    if (ui_delay_for_testing > 0)
	msec = ui_delay_for_testing;
    ch_log(NULL, "ui_delay(%ld)", msec);
#endif
#ifdef FEAT_GUI
    if (gui.in_use && !ignoreinput)
	gui_wait_for_chars(msec, typebuf.tb_change_cnt);
    else
#endif
	mch_delay(msec, ignoreinput ? MCH_DELAY_IGNOREINPUT : 0);
}

/*
 * If the machine has job control, use it to suspend the program,
 * otherwise fake it by starting a new shell.
 * When running the GUI iconify the window.
 */
    void
ui_suspend(void)
{
#ifdef FEAT_GUI
    if (gui.in_use)
    {
	gui_mch_iconify();
	return;
    }
#endif
    mch_suspend();
}

#if !defined(UNIX) || !defined(SIGTSTP) || defined(PROTO)
/*
 * When the OS can't really suspend, call this function to start a shell.
 * This is never called in the GUI.
 */
    void
suspend_shell(void)
{
    if (*p_sh == NUL)
	emsg(_(e_shell_option_is_empty));
    else
    {
	msg_puts(_("new shell started\n"));
	do_shell(NULL, 0);
    }
}
#endif

/*
 * Try to get the current Vim shell size.  Put the result in Rows and Columns.
 * Use the new sizes as defaults for 'columns' and 'lines'.
 * Return OK when size could be determined, FAIL otherwise.
 */
    int
ui_get_shellsize(void)
{
    int	    retval;

#ifdef FEAT_GUI
    if (gui.in_use)
	retval = gui_get_shellsize();
    else
#endif
	retval = mch_get_shellsize();

    check_shellsize();

    // adjust the default for 'lines' and 'columns'
    if (retval == OK)
    {
	set_number_default("lines", Rows);
	set_number_default("columns", Columns);
    }
    return retval;
}

/*
 * Set the size of the Vim shell according to Rows and Columns, if possible.
 * The gui_set_shellsize() or mch_set_shellsize() function will try to set the
 * new size.  If this is not possible, it will adjust Rows and Columns.
 */
    void
ui_set_shellsize(
    int		mustset UNUSED)	// set by the user
{
#ifdef FEAT_GUI
    if (gui.in_use)
	gui_set_shellsize(mustset, TRUE, RESIZE_BOTH);
    else
#endif
	mch_set_shellsize();
}

/*
 * Called when Rows and/or Columns changed.  Adjust scroll region and mouse
 * region.
 */
    void
ui_new_shellsize(void)
{
    if (full_screen && !exiting)
    {
#ifdef FEAT_GUI
	if (gui.in_use)
	    gui_new_shellsize();
	else
#endif
	    mch_new_shellsize();
    }
}

#if ((defined(FEAT_EVAL) || defined(FEAT_TERMINAL)) \
	    && (defined(FEAT_GUI) \
		|| defined(MSWIN) \
		|| (defined(HAVE_TGETENT) && defined(FEAT_TERMRESPONSE)))) \
	|| defined(PROTO)
/*
 * Get the window position in pixels, if possible.
 * Return FAIL when not possible.
 */
    int
ui_get_winpos(int *x, int *y, varnumber_T timeout UNUSED)
{
# ifdef FEAT_GUI
    if (gui.in_use)
	return gui_mch_get_winpos(x, y);
# endif
# if defined(MSWIN) && (!defined(FEAT_GUI) || defined(VIMDLL))
    return mch_get_winpos(x, y);
# else
#  if defined(HAVE_TGETENT) && defined(FEAT_TERMRESPONSE)
    return term_get_winpos(x, y, timeout);
#  else
    return FAIL;
#  endif
# endif
}
#endif

    void
ui_breakcheck(void)
{
    ui_breakcheck_force(FALSE);
}

/*
 * When "force" is true also check when the terminal is not in raw mode.
 * This is useful to read input on channels.
 */
    void
ui_breakcheck_force(int force)
{
    static int	recursive = FALSE;
    int		save_updating_screen = updating_screen;

    // We could be called recursively if stderr is redirected, calling
    // fill_input_buf() calls settmode() when stdin isn't a tty.  settmode()
    // calls vgetorpeek() which calls ui_breakcheck() again.
    if (recursive)
	return;
    recursive = TRUE;

    // We do not want gui_resize_shell() to redraw the screen here.
    ++updating_screen;

#ifdef FEAT_GUI
    if (gui.in_use)
	gui_mch_update();
    else
#endif
	mch_breakcheck(force);

    if (save_updating_screen)
	updating_screen = TRUE;
    else
	after_updating_screen(FALSE);

    recursive = FALSE;
}

//////////////////////////////////////////////////////////////////////////////
// Functions that handle the input buffer.
// This is used for any GUI version, and the unix terminal version.
//
// For Unix, the input characters are buffered to be able to check for a
// CTRL-C.  This should be done with signals, but I don't know how to do that
// in a portable way for a tty in RAW mode.
//
// For the client-server code in the console the received keys are put in the
// input buffer.

#if defined(USE_INPUT_BUF) || defined(PROTO)

/*
 * Internal typeahead buffer.  Includes extra space for long key code
 * descriptions which would otherwise overflow.  The buffer is considered full
 * when only this extra space (or part of it) remains.
 */
#if defined(FEAT_JOB_CHANNEL) || defined(FEAT_CLIENTSERVER)
   /*
    * NetBeans stuffs debugger commands into the input buffer.
    * This requires a larger buffer...
    * (Madsen) Go with this for remote input as well ...
    */
# define INBUFLEN 4096
#else
# define INBUFLEN 250
#endif

static char_u	inbuf[INBUFLEN + MAX_KEY_CODE_LEN];
static int	inbufcount = 0;	    // number of chars in inbuf[]

/*
 * vim_is_input_buf_full(), vim_is_input_buf_empty(), add_to_input_buf(), and
 * trash_input_buf() are functions for manipulating the input buffer.  These
 * are used by the gui_* calls when a GUI is used to handle keyboard input.
 */

    int
vim_is_input_buf_full(void)
{
    return (inbufcount >= INBUFLEN);
}

    int
vim_is_input_buf_empty(void)
{
    return (inbufcount == 0);
}

#if defined(FEAT_OLE) || defined(PROTO)
    int
vim_free_in_input_buf(void)
{
    return (INBUFLEN - inbufcount);
}
#endif

#if defined(FEAT_GUI_GTK) || defined(PROTO)
    int
vim_used_in_input_buf(void)
{
    return inbufcount;
}
#endif

/*
 * Return the current contents of the input buffer and make it empty.
 * The returned pointer must be passed to set_input_buf() later.
 */
    char_u *
get_input_buf(void)
{
    garray_T	*gap;

    // We use a growarray to store the data pointer and the length.
    gap = ALLOC_ONE(garray_T);
    if (gap != NULL)
    {
	// Add one to avoid a zero size.
	gap->ga_data = alloc(inbufcount + 1);
	if (gap->ga_data != NULL)
	    mch_memmove(gap->ga_data, inbuf, (size_t)inbufcount);
	gap->ga_len = inbufcount;
    }
    trash_input_buf();
    return (char_u *)gap;
}

/*
 * Restore the input buffer with a pointer returned from get_input_buf().
 * The allocated memory is freed, this only works once!
 * When "overwrite" is FALSE input typed later is kept.
 */
    void
set_input_buf(char_u *p, int overwrite)
{
    garray_T	*gap = (garray_T *)p;

    if (gap == NULL)
	return;

    if (gap->ga_data != NULL)
    {
	if (overwrite || inbufcount + gap->ga_len >= INBUFLEN)
	{
	    mch_memmove(inbuf, gap->ga_data, gap->ga_len);
	    inbufcount = gap->ga_len;
	}
	else
	{
	    mch_memmove(inbuf + gap->ga_len, inbuf, inbufcount);
	    mch_memmove(inbuf, gap->ga_data, gap->ga_len);
	    inbufcount += gap->ga_len;
	}
	vim_free(gap->ga_data);
    }
    vim_free(gap);
}

/*
 * Add the given bytes to the input buffer
 * Special keys start with CSI.  A real CSI must have been translated to
 * CSI KS_EXTRA KE_CSI.  K_SPECIAL doesn't require translation.
 */
    void
add_to_input_buf(char_u *s, int len)
{
    if (inbufcount + len > INBUFLEN + MAX_KEY_CODE_LEN)
	return;	    // Shouldn't ever happen!

    while (len--)
	inbuf[inbufcount++] = *s++;
}

/*
 * Add "str[len]" to the input buffer while escaping CSI bytes.
 */
    void
add_to_input_buf_csi(char_u *str, int len)
{
    int		i;
    char_u	buf[2];

    for (i = 0; i < len; ++i)
    {
	add_to_input_buf(str + i, 1);
	if (str[i] == CSI)
	{
	    // Turn CSI into K_CSI.
	    buf[0] = KS_EXTRA;
	    buf[1] = (int)KE_CSI;
	    add_to_input_buf(buf, 2);
	}
    }
}

/*
 * Remove everything from the input buffer.  Called when ^C is found.
 */
    void
trash_input_buf(void)
{
    inbufcount = 0;
}

/*
 * Read as much data from the input buffer as possible up to maxlen, and store
 * it in buf.
 */
    int
read_from_input_buf(char_u *buf, long maxlen)
{
    if (inbufcount == 0)	// if the buffer is empty, fill it
	fill_input_buf(TRUE);
    if (maxlen > inbufcount)
	maxlen = inbufcount;
    mch_memmove(buf, inbuf, (size_t)maxlen);
    inbufcount -= maxlen;
    // check "maxlen" to avoid clang warning
    if (inbufcount > 0 && maxlen > 0)
	mch_memmove(inbuf, inbuf + maxlen, (size_t)inbufcount);
    return (int)maxlen;
}

    void
fill_input_buf(int exit_on_error UNUSED)
{
#if defined(UNIX) || defined(VMS) || defined(MACOS_X)
    int		len;
    int		try;
    static int	did_read_something = FALSE;
    static char_u *rest = NULL;	    // unconverted rest of previous read
    static int	restlen = 0;
    int		unconverted;
#endif

#ifdef FEAT_GUI
    if (gui.in_use
# ifdef NO_CONSOLE_INPUT
    // Don't use the GUI input when the window hasn't been opened yet.
    // We get here from ui_inchar() when we should try reading from stdin.
	    && !no_console_input()
# endif
       )
    {
	gui_mch_update();
	return;
    }
#endif
#if defined(UNIX) || defined(VMS) || defined(MACOS_X)
    if (vim_is_input_buf_full())
	return;
    /*
     * Fill_input_buf() is only called when we really need a character.
     * If we can't get any, but there is some in the buffer, just return.
     * If we can't get any, and there isn't any in the buffer, we give up and
     * exit Vim.
     */
    if (rest != NULL)
    {
	// Use remainder of previous call, starts with an invalid character
	// that may become valid when reading more.
	if (restlen > INBUFLEN - inbufcount)
	    unconverted = INBUFLEN - inbufcount;
	else
	    unconverted = restlen;
	mch_memmove(inbuf + inbufcount, rest, unconverted);
	if (unconverted == restlen)
	    VIM_CLEAR(rest);
	else
	{
	    restlen -= unconverted;
	    mch_memmove(rest, rest + unconverted, restlen);
	}
	inbufcount += unconverted;
    }
    else
	unconverted = 0;

    len = 0;	// to avoid gcc warning
    for (try = 0; try < 100; ++try)
    {
	size_t readlen = (size_t)((INBUFLEN - inbufcount)
						       / input_conv.vc_factor);
#  ifdef VMS
	len = vms_read((char *)inbuf + inbufcount, readlen);
#  else
	len = read(read_cmd_fd, (char *)inbuf + inbufcount, readlen);
#  endif
#  ifdef FEAT_EVAL
	if (len > 0)
	{
	    inbuf[inbufcount + len] = NUL;
	    ch_log(NULL, "raw key input: \"%s\"", inbuf + inbufcount);
	}
#  endif

	if (len > 0 || got_int)
	    break;
	/*
	 * If reading stdin results in an error, continue reading stderr.
	 * This helps when using "foo | xargs vim".
	 */
	if (!did_read_something && !isatty(read_cmd_fd) && read_cmd_fd == 0)
	{
	    int m = cur_tmode;

	    // We probably set the wrong file descriptor to raw mode.  Switch
	    // back to cooked mode, use another descriptor and set the mode to
	    // what it was.
	    settmode(TMODE_COOK);
#ifdef HAVE_DUP
	    // Use stderr for stdin, also works for shell commands.
	    close(0);
	    vim_ignored = dup(2);
#else
	    read_cmd_fd = 2;	// read from stderr instead of stdin
#endif
	    settmode(m);
	}
	if (!exit_on_error)
	    return;
    }
    if (len <= 0 && !got_int)
	read_error_exit();
    if (len > 0)
	did_read_something = TRUE;
    if (got_int)
    {
	// Interrupted, pretend a CTRL-C was typed.
	inbuf[0] = 3;
	inbufcount = 1;
    }
    else
    {
	/*
	 * May perform conversion on the input characters.
	 * Include the unconverted rest of the previous call.
	 * If there is an incomplete char at the end it is kept for the next
	 * time, reading more bytes should make conversion possible.
	 * Don't do this in the unlikely event that the input buffer is too
	 * small ("rest" still contains more bytes).
	 */
	if (input_conv.vc_type != CONV_NONE)
	{
	    inbufcount -= unconverted;
	    len = convert_input_safe(inbuf + inbufcount,
				     len + unconverted, INBUFLEN - inbufcount,
				       rest == NULL ? &rest : NULL, &restlen);
	}
	while (len > 0)
	{
	    // If a CTRL-C was typed, remove it from the buffer and set
	    // got_int.  Also recognize CTRL-C with modifyOtherKeys set, lower
	    // and upper case, in two forms.
	    // If terminal key protocols are in use, we expect to receive
	    // Ctrl_C as an escape sequence, ignore a raw Ctrl_C as this could
	    // be paste data.
	    if (ctrl_c_interrupts
			&& ((inbuf[inbufcount] == Ctrl_C && !key_protocol_enabled())
			|| (len >= 10 && STRNCMP(inbuf + inbufcount,
						   "\033[27;5;99~", 10) == 0)
			|| (len >= 10 && STRNCMP(inbuf + inbufcount,
						   "\033[27;5;67~", 10) == 0)
			|| (len >= 7 && STRNCMP(inbuf + inbufcount,
						       "\033[99;5u", 7) == 0)
			|| (len >= 7 && STRNCMP(inbuf + inbufcount,
						       "\033[67;5u", 7) == 0)))
	    {
		// remove everything typed before the CTRL-C
		mch_memmove(inbuf, inbuf + inbufcount, (size_t)(len));
		inbufcount = 0;
		got_int = TRUE;
	    }
	    --len;
	    ++inbufcount;
	}
    }
#endif // UNIX || VMS || MACOS_X
}
#endif // USE_INPUT_BUF

/*
 * Exit because of an input read error.
 */
    void
read_error_exit(void)
{
    if (silent_mode)	// Normal way to exit for "ex -s"
	getout(0);
    STRCPY(IObuff, _("Vim: Error reading input, exiting...\n"));
    preserve_exit();
}

#if defined(CURSOR_SHAPE) || defined(PROTO)
/*
 * May update the shape of the cursor.
 */
    void
ui_cursor_shape_forced(int forced)
{
# ifdef FEAT_GUI
    if (gui.in_use)
	gui_update_cursor_later();
    else
# endif
	term_cursor_mode(forced);

# ifdef MCH_CURSOR_SHAPE
    mch_update_cursor();
# endif

# ifdef FEAT_CONCEAL
    conceal_check_cursor_line(FALSE);
# endif
}

    void
ui_cursor_shape(void)
{
    ui_cursor_shape_forced(FALSE);
}
#endif

/*
 * Check bounds for column number
 */
    int
check_col(int col)
{
    if (col < 0)
	return 0;
    if (col >= screen_Columns)
	return screen_Columns - 1;
    return col;
}

/*
 * Check bounds for row number
 */
    int
check_row(int row)
{
    if (row < 0)
	return 0;
    if (row >= screen_Rows)
	return screen_Rows - 1;
    return row;
}

/*
 * Return length of line "lnum" in screen cells for horizontal scrolling.
 */
    long
scroll_line_len(linenr_T lnum)
{
    char_u	*p = ml_get(lnum);
    colnr_T	col = 0;

    if (*p != NUL)
	for (;;)
	{
	    int	    w = chartabsize(p, col);
	    MB_PTR_ADV(p);
	    if (*p == NUL)		// don't count the last character
		break;
	    col += w;
	}
    return col;
}

/*
 * Find the longest visible line number.  This is used for horizontal
 * scrolling.  If this is not possible (or not desired, by setting 'h' in
 * "guioptions") then the current line number is returned.
 */
    linenr_T
ui_find_longest_lnum(void)
{
    linenr_T ret = 0;

    // Calculate maximum for horizontal scrollbar.  Check for reasonable
    // line numbers, topline and botline can be invalid when displaying is
    // postponed.
    if (
# ifdef FEAT_GUI
	    (!gui.in_use || vim_strchr(p_go, GO_HORSCROLL) == NULL) &&
# endif
	    curwin->w_topline <= curwin->w_cursor.lnum
	    && curwin->w_botline > curwin->w_cursor.lnum
	    && curwin->w_botline <= curbuf->b_ml.ml_line_count + 1)
    {
	linenr_T    lnum;
	long	    n;
	long	    max = 0;

	// Use maximum of all visible lines.  Remember the lnum of the
	// longest line, closest to the cursor line.  Used when scrolling
	// below.
	for (lnum = curwin->w_topline; lnum < curwin->w_botline; ++lnum)
	{
	    n = scroll_line_len(lnum);
	    if (n > max)
	    {
		max = n;
		ret = lnum;
	    }
	    else if (n == max && abs((int)(lnum - curwin->w_cursor.lnum))
				     < abs((int)(ret - curwin->w_cursor.lnum)))
		ret = lnum;
	}
    }
    else
	// Use cursor line only.
	ret = curwin->w_cursor.lnum;

    return ret;
}

/*
 * Called when focus changed.  Used for the GUI or for systems where this can
 * be done in the console (Win32).
 */
    void
ui_focus_change(
    int		in_focus)	// TRUE if focus gained.
{
    static time_t	last_time = (time_t)0;
    int			need_redraw = FALSE;

    // When activated: Check if any file was modified outside of Vim.
    // Only do this when not done within the last two seconds (could get
    // several events in a row).
    if (in_focus && last_time + 2 < time(NULL))
    {
	need_redraw = check_timestamps(
# ifdef FEAT_GUI
		gui.in_use
# else
		FALSE
# endif
		);
	last_time = time(NULL);
    }

#ifdef FEAT_TERMINAL
    term_focus_change(in_focus);
#endif

    /*
     * Fire the focus gained/lost autocommand.
     */
    need_redraw |= apply_autocmds(in_focus ? EVENT_FOCUSGAINED
				: EVENT_FOCUSLOST, NULL, NULL, FALSE, curbuf);

    if (need_redraw)
	redraw_after_callback(TRUE, TRUE);

    // File may have been changed from 'readonly' to 'noreadonly'
    if (need_maketitle)
	maketitle();
}

#if defined(HAVE_INPUT_METHOD) || defined(PROTO)
/*
 * Save current Input Method status to specified place.
 */
    void
im_save_status(long *psave)
{
    // Don't save when 'imdisable' is set or "xic" is NULL, IM is always
    // disabled then (but might start later).
    // Also don't save when inside a mapping, vgetc_im_active has not been set
    // then.
    // And don't save when the keys were stuffed (e.g., for a "." command).
    // And don't save when the GUI is running but our window doesn't have
    // input focus (e.g., when a find dialog is open).
    if (!p_imdisable && KeyTyped && !KeyStuffed
# ifdef FEAT_XIM
	    && xic != NULL
# endif
# ifdef FEAT_GUI
	    && (!gui.in_use || gui.in_focus)
# endif
	)
    {
	// Do save when IM is on, or IM is off and saved status is on.
	if (vgetc_im_active)
	    *psave = B_IMODE_IM;
	else if (*psave == B_IMODE_IM)
	    *psave = B_IMODE_NONE;
    }
}
#endif
