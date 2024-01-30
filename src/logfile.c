/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

/*
 * Implements logging.  Originally intended for the channel feature, which is
 * why the "ch_" prefix is used.  Also useful for any kind of low-level and
 * async debugging.
 */

#include "vim.h"

#if defined(FEAT_EVAL) || defined(PROTO)

// Log file opened with ch_logfile().
static FILE *log_fd = NULL;
static char_u *log_name = NULL;
#ifdef FEAT_RELTIME
static proftime_T log_start;
#endif

    void
ch_logfile(char_u *fname, char_u *opt)
{
    FILE	*file = NULL;
    char	*mode = "a";

    if (log_fd != NULL)
    {
	if (*fname != NUL)
	    ch_log(NULL, "closing this logfile, opening %s", fname);
	else
	    ch_log(NULL, "closing logfile %s", log_name);
	fclose(log_fd);
    }

    // The "a" flag overrules the "w" flag.
    if (vim_strchr(opt, 'a') == NULL && vim_strchr(opt, 'w') != NULL)
	mode = "w";
    ch_log_output = vim_strchr(opt, 'o') != NULL ? LOG_ALWAYS : FALSE;

    if (*fname != NUL)
    {
	file = fopen((char *)fname, mode);
	if (file == NULL)
	{
	    semsg(_(e_cant_open_file_str), fname);
	    return;
	}
	vim_free(log_name);
	log_name = vim_strsave(fname);
    }
    log_fd = file;

    if (log_fd != NULL)
    {
	fprintf(log_fd, "==== start log session %s ====\n",
						 get_ctime(time(NULL), FALSE));
	// flush now, if fork/exec follows it could be written twice
	fflush(log_fd);
#ifdef FEAT_RELTIME
	profile_start(&log_start);
#endif
    }
}

    int
ch_log_active(void)
{
    return log_fd != NULL;
}

    static void
ch_log_lead(const char *what, channel_T *ch UNUSED, ch_part_T part UNUSED)
{
    if (log_fd == NULL)
	return;

#ifdef FEAT_RELTIME
    proftime_T log_now;

    profile_start(&log_now);
    profile_sub(&log_now, &log_start);
    fprintf(log_fd, "%s ", profile_msg(&log_now));
#endif
#ifdef FEAT_JOB_CHANNEL
    if (ch != NULL)
    {
	if (part < PART_COUNT)
	    fprintf(log_fd, "%son %d(%s): ", what, ch->ch_id, ch_part_names[part]);
	else
	    fprintf(log_fd, "%son %d: ", what, ch->ch_id);
    }
    else
#endif
	fprintf(log_fd, "%s: ", what);
}

#ifndef PROTO  // prototype is in proto.h
    void
ch_log(channel_T *ch, const char *fmt, ...)
{
    if (log_fd == NULL)
	return;

    va_list ap;

    ch_log_lead("", ch, PART_COUNT);
    va_start(ap, fmt);
    vfprintf(log_fd, fmt, ap);
    va_end(ap);
    fputc('\n', log_fd);
    fflush(log_fd);
    did_repeated_msg = 0;
}

    void
ch_error(channel_T *ch, const char *fmt, ...)
{
    if (log_fd == NULL)
	return;

    va_list ap;

    ch_log_lead("ERR ", ch, PART_COUNT);
    va_start(ap, fmt);
    vfprintf(log_fd, fmt, ap);
    va_end(ap);
    fputc('\n', log_fd);
    fflush(log_fd);
    did_repeated_msg = 0;
}
#endif

#if defined(FEAT_JOB_CHANNEL) || defined(PROTO)
/*
 * Log a message "buf[len]" for channel "ch" part "part".
 * Only to be called when ch_log_active() returns TRUE.
 */
    void
ch_log_literal(
	char	    *lead,
	channel_T   *ch,
	ch_part_T   part,
	char_u	    *buf,
	int	    len)
{
    ch_log_lead(lead, ch, part);
    fprintf(log_fd, "'");
    vim_ignored = (int)fwrite(buf, len, 1, log_fd);
    fprintf(log_fd, "'\n");
    fflush(log_fd);
}
#endif

/*
 * "ch_log()" function
 */
    void
f_ch_log(typval_T *argvars, typval_T *rettv UNUSED)
{
    char_u	*msg;
    channel_T	*channel = NULL;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_chan_or_job_arg(argvars, 1) == FAIL))
	return;

    msg = tv_get_string(&argvars[0]);
#if defined(FEAT_JOB_CHANNEL)
    if (argvars[1].v_type != VAR_UNKNOWN)
	channel = get_channel_arg(&argvars[1], FALSE, FALSE, 0);
#endif

    // Prepend "ch_log()" to make it easier to find these entries in the
    // logfile.
    ch_log(channel, "ch_log(): %s", msg);
}

/*
 * "ch_logfile()" function
 */
    void
f_ch_logfile(typval_T *argvars, typval_T *rettv UNUSED)
{
    char_u *fname;
    char_u *opt = (char_u *)"";
    char_u buf[NUMBUFLEN];

    // Don't open a file in restricted mode.
    if (check_restricted() || check_secure())
	return;

    if (in_vim9script()
	    && (check_for_string_arg(argvars, 0) == FAIL
		|| check_for_opt_string_arg(argvars, 1) == FAIL))
	return;

    fname = tv_get_string(&argvars[0]);
    if (argvars[1].v_type == VAR_STRING)
	opt = tv_get_string_buf(&argvars[1], buf);
    ch_logfile(fname, opt);
}

#endif // FEAT_EVAL
