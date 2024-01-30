/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * autocmd.c: Autocommand related functions
 */

#include "vim.h"

/*
 * The autocommands are stored in a list for each event.
 * Autocommands for the same pattern, that are consecutive, are joined
 * together, to avoid having to match the pattern too often.
 * The result is an array of Autopat lists, which point to AutoCmd lists:
 *
 * last_autopat[0]  -----------------------------+
 *						 V
 * first_autopat[0] --> Autopat.next  -->  Autopat.next -->  NULL
 *			Autopat.cmds	   Autopat.cmds
 *			    |			 |
 *			    V			 V
 *			AutoCmd.next	   AutoCmd.next
 *			    |			 |
 *			    V			 V
 *			AutoCmd.next		NULL
 *			    |
 *			    V
 *			   NULL
 *
 * last_autopat[1]  --------+
 *			    V
 * first_autopat[1] --> Autopat.next  -->  NULL
 *			Autopat.cmds
 *			    |
 *			    V
 *			AutoCmd.next
 *			    |
 *			    V
 *			   NULL
 *   etc.
 *
 *   The order of AutoCmds is important, this is the order in which they were
 *   defined and will have to be executed.
 */
typedef struct AutoCmd
{
    char_u	    *cmd;		// The command to be executed (NULL
					// when command has been removed).
    char	    once;		// "One shot": removed after execution
    char	    nested;		// If autocommands nest here.
    char	    last;		// last command in list
    sctx_T	    script_ctx;		// script context where it is defined
    struct AutoCmd  *next;		// next AutoCmd in list
} AutoCmd;

typedef struct AutoPat
{
    struct AutoPat  *next;		// Next AutoPat in AutoPat list; MUST
					// be the first entry.
    char_u	    *pat;		// pattern as typed (NULL when pattern
					// has been removed)
    regprog_T	    *reg_prog;		// compiled regprog for pattern
    AutoCmd	    *cmds;		// list of commands to do
    int		    group;		// group ID
    int		    patlen;		// strlen() of pat
    int		    buflocal_nr;	// !=0 for buffer-local AutoPat
    char	    allow_dirs;		// Pattern may match whole path
    char	    last;		// last pattern for apply_autocmds()
} AutoPat;

static struct event_name
{
    char	*name;	// event name
    event_T	event;	// event number
} event_names[] =
{
    {"BufAdd",		EVENT_BUFADD},
    {"BufCreate",	EVENT_BUFADD},
    {"BufDelete",	EVENT_BUFDELETE},
    {"BufEnter",	EVENT_BUFENTER},
    {"BufFilePost",	EVENT_BUFFILEPOST},
    {"BufFilePre",	EVENT_BUFFILEPRE},
    {"BufHidden",	EVENT_BUFHIDDEN},
    {"BufLeave",	EVENT_BUFLEAVE},
    {"BufNew",		EVENT_BUFNEW},
    {"BufNewFile",	EVENT_BUFNEWFILE},
    {"BufRead",		EVENT_BUFREADPOST},
    {"BufReadCmd",	EVENT_BUFREADCMD},
    {"BufReadPost",	EVENT_BUFREADPOST},
    {"BufReadPre",	EVENT_BUFREADPRE},
    {"BufUnload",	EVENT_BUFUNLOAD},
    {"BufWinEnter",	EVENT_BUFWINENTER},
    {"BufWinLeave",	EVENT_BUFWINLEAVE},
    {"BufWipeout",	EVENT_BUFWIPEOUT},
    {"BufWrite",	EVENT_BUFWRITEPRE},
    {"BufWritePost",	EVENT_BUFWRITEPOST},
    {"BufWritePre",	EVENT_BUFWRITEPRE},
    {"BufWriteCmd",	EVENT_BUFWRITECMD},
    {"CmdlineChanged",	EVENT_CMDLINECHANGED},
    {"CmdlineEnter",	EVENT_CMDLINEENTER},
    {"CmdlineLeave",	EVENT_CMDLINELEAVE},
    {"CmdwinEnter",	EVENT_CMDWINENTER},
    {"CmdwinLeave",	EVENT_CMDWINLEAVE},
    {"CmdUndefined",	EVENT_CMDUNDEFINED},
    {"ColorScheme",	EVENT_COLORSCHEME},
    {"ColorSchemePre",	EVENT_COLORSCHEMEPRE},
    {"CompleteChanged",	EVENT_COMPLETECHANGED},
    {"CompleteDone",	EVENT_COMPLETEDONE},
    {"CompleteDonePre",	EVENT_COMPLETEDONEPRE},
    {"CursorHold",	EVENT_CURSORHOLD},
    {"CursorHoldI",	EVENT_CURSORHOLDI},
    {"CursorMoved",	EVENT_CURSORMOVED},
    {"CursorMovedI",	EVENT_CURSORMOVEDI},
    {"DiffUpdated",	EVENT_DIFFUPDATED},
    {"DirChanged",	EVENT_DIRCHANGED},
    {"DirChangedPre",	EVENT_DIRCHANGEDPRE},
    {"EncodingChanged",	EVENT_ENCODINGCHANGED},
    {"ExitPre",		EVENT_EXITPRE},
    {"FileEncoding",	EVENT_ENCODINGCHANGED},
    {"FileAppendPost",	EVENT_FILEAPPENDPOST},
    {"FileAppendPre",	EVENT_FILEAPPENDPRE},
    {"FileAppendCmd",	EVENT_FILEAPPENDCMD},
    {"FileChangedShell",EVENT_FILECHANGEDSHELL},
    {"FileChangedShellPost",EVENT_FILECHANGEDSHELLPOST},
    {"FileChangedRO",	EVENT_FILECHANGEDRO},
    {"FileReadPost",	EVENT_FILEREADPOST},
    {"FileReadPre",	EVENT_FILEREADPRE},
    {"FileReadCmd",	EVENT_FILEREADCMD},
    {"FileType",	EVENT_FILETYPE},
    {"FileWritePost",	EVENT_FILEWRITEPOST},
    {"FileWritePre",	EVENT_FILEWRITEPRE},
    {"FileWriteCmd",	EVENT_FILEWRITECMD},
    {"FilterReadPost",	EVENT_FILTERREADPOST},
    {"FilterReadPre",	EVENT_FILTERREADPRE},
    {"FilterWritePost",	EVENT_FILTERWRITEPOST},
    {"FilterWritePre",	EVENT_FILTERWRITEPRE},
    {"FocusGained",	EVENT_FOCUSGAINED},
    {"FocusLost",	EVENT_FOCUSLOST},
    {"FuncUndefined",	EVENT_FUNCUNDEFINED},
    {"GUIEnter",	EVENT_GUIENTER},
    {"GUIFailed",	EVENT_GUIFAILED},
    {"InsertChange",	EVENT_INSERTCHANGE},
    {"InsertEnter",	EVENT_INSERTENTER},
    {"InsertLeave",	EVENT_INSERTLEAVE},
    {"InsertLeavePre",	EVENT_INSERTLEAVEPRE},
    {"InsertCharPre",	EVENT_INSERTCHARPRE},
    {"MenuPopup",	EVENT_MENUPOPUP},
    {"ModeChanged",	EVENT_MODECHANGED},
    {"OptionSet",	EVENT_OPTIONSET},
    {"QuickFixCmdPost",	EVENT_QUICKFIXCMDPOST},
    {"QuickFixCmdPre",	EVENT_QUICKFIXCMDPRE},
    {"QuitPre",		EVENT_QUITPRE},
    {"RemoteReply",	EVENT_REMOTEREPLY},
    {"SafeState",	EVENT_SAFESTATE},
    {"SafeStateAgain",	EVENT_SAFESTATEAGAIN},
    {"SessionLoadPost",	EVENT_SESSIONLOADPOST},
    {"ShellCmdPost",	EVENT_SHELLCMDPOST},
    {"ShellFilterPost",	EVENT_SHELLFILTERPOST},
    {"SigUSR1",		EVENT_SIGUSR1},
    {"SourceCmd",	EVENT_SOURCECMD},
    {"SourcePre",	EVENT_SOURCEPRE},
    {"SourcePost",	EVENT_SOURCEPOST},
    {"SpellFileMissing",EVENT_SPELLFILEMISSING},
    {"StdinReadPost",	EVENT_STDINREADPOST},
    {"StdinReadPre",	EVENT_STDINREADPRE},
    {"SwapExists",	EVENT_SWAPEXISTS},
    {"Syntax",		EVENT_SYNTAX},
    {"TabNew",		EVENT_TABNEW},
    {"TabClosed",	EVENT_TABCLOSED},
    {"TabEnter",	EVENT_TABENTER},
    {"TabLeave",	EVENT_TABLEAVE},
    {"TermChanged",	EVENT_TERMCHANGED},
    {"TerminalOpen",	EVENT_TERMINALOPEN},
    {"TerminalWinOpen", EVENT_TERMINALWINOPEN},
    {"TermResponse",	EVENT_TERMRESPONSE},
    {"TermResponseAll",	EVENT_TERMRESPONSEALL},
    {"TextChanged",	EVENT_TEXTCHANGED},
    {"TextChangedI",	EVENT_TEXTCHANGEDI},
    {"TextChangedP",	EVENT_TEXTCHANGEDP},
    {"TextChangedT",	EVENT_TEXTCHANGEDT},
    {"User",		EVENT_USER},
    {"VimEnter",	EVENT_VIMENTER},
    {"VimLeave",	EVENT_VIMLEAVE},
    {"VimLeavePre",	EVENT_VIMLEAVEPRE},
    {"WinNewPre",	EVENT_WINNEWPRE},
    {"WinNew",		EVENT_WINNEW},
    {"WinClosed",	EVENT_WINCLOSED},
    {"WinEnter",	EVENT_WINENTER},
    {"WinLeave",	EVENT_WINLEAVE},
    {"WinResized",	EVENT_WINRESIZED},
    {"WinScrolled",	EVENT_WINSCROLLED},
    {"VimResized",	EVENT_VIMRESIZED},
    {"TextYankPost",	EVENT_TEXTYANKPOST},
    {"VimSuspend",	EVENT_VIMSUSPEND},
    {"VimResume",	EVENT_VIMRESUME},
    {NULL,		(event_T)0}
};

static AutoPat *first_autopat[NUM_EVENTS] =
{
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL
};

static AutoPat *last_autopat[NUM_EVENTS] =
{
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL,
    NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL
};

#define AUGROUP_DEFAULT    (-1)	    // default autocmd group
#define AUGROUP_ERROR	   (-2)	    // erroneous autocmd group
#define AUGROUP_ALL	   (-3)	    // all autocmd groups

/*
 * struct used to keep status while executing autocommands for an event.
 */
struct AutoPatCmd_S
{
    AutoPat	*curpat;	// next AutoPat to examine
    AutoCmd	*nextcmd;	// next AutoCmd to execute
    int		group;		// group being used
    char_u	*fname;		// fname to match with
    char_u	*sfname;	// sfname to match with
    char_u	*tail;		// tail of fname
    event_T	event;		// current event
    sctx_T	script_ctx;	// script context where it is defined
    int		arg_bufnr;	// Initially equal to <abuf>, set to zero when
				// buf is deleted.
    AutoPatCmd_T *next;		// chain of active apc-s for auto-invalidation
};

static AutoPatCmd_T *active_apc_list = NULL; // stack of active autocommands

// Macro to loop over all the patterns for an autocmd event
#define FOR_ALL_AUTOCMD_PATTERNS(event, ap) \
    for ((ap) = first_autopat[(int)(event)]; (ap) != NULL; (ap) = (ap)->next)

/*
 * augroups stores a list of autocmd group names.
 */
static garray_T augroups = {0, 0, sizeof(char_u *), 10, NULL};
#define AUGROUP_NAME(i) (((char_u **)augroups.ga_data)[i])
// use get_deleted_augroup() to get this
static char_u *deleted_augroup = NULL;

/*
 * The ID of the current group.  Group 0 is the default one.
 */
static int current_augroup = AUGROUP_DEFAULT;

static int au_need_clean = FALSE;   // need to delete marked patterns

static char_u *event_nr2name(event_T event);
static int au_get_grouparg(char_u **argp);
static int do_autocmd_event(event_T event, char_u *pat, int once, int nested, char_u *cmd, int forceit, int group, int flags);
static int apply_autocmds_group(event_T event, char_u *fname, char_u *fname_io, int force, int group, buf_T *buf, exarg_T *eap);
static void auto_next_pat(AutoPatCmd_T *apc, int stop_at_last);
static int au_find_group(char_u *name);

static event_T	last_event;
static int	last_group;
static int	autocmd_blocked = 0;	// block all autocmds

    static char_u *
get_deleted_augroup(void)
{
    if (deleted_augroup == NULL)
	deleted_augroup = (char_u *)_("--Deleted--");
    return deleted_augroup;
}

/*
 * Show the autocommands for one AutoPat.
 */
    static void
show_autocmd(AutoPat *ap, event_T event)
{
    AutoCmd *ac;

    // Check for "got_int" (here and at various places below), which is set
    // when "q" has been hit for the "--more--" prompt
    if (got_int)
	return;
    if (ap->pat == NULL)		// pattern has been removed
	return;

    // Make sure no info referenced by "ap" is cleared, e.g. when a timer
    // clears an augroup.  Jump to "theend" after this!
    // "ap->pat" may be cleared anyway.
    ++autocmd_busy;

    msg_putchar('\n');
    if (got_int)
	goto theend;
    if (event != last_event || ap->group != last_group)
    {
	if (ap->group != AUGROUP_DEFAULT)
	{
	    if (AUGROUP_NAME(ap->group) == NULL)
		msg_puts_attr((char *)get_deleted_augroup(), HL_ATTR(HLF_E));
	    else
		msg_puts_attr((char *)AUGROUP_NAME(ap->group), HL_ATTR(HLF_T));
	    msg_puts("  ");
	}
	msg_puts_attr((char *)event_nr2name(event), HL_ATTR(HLF_T));
	last_event = event;
	last_group = ap->group;
	msg_putchar('\n');
	if (got_int)
	    goto theend;
    }

    if (ap->pat == NULL)
	goto theend;  // timer might have cleared the pattern or group

    msg_col = 4;
    msg_outtrans(ap->pat);

    for (ac = ap->cmds; ac != NULL; ac = ac->next)
    {
	if (ac->cmd == NULL)		// skip removed commands
	    continue;

	if (msg_col >= 14)
	    msg_putchar('\n');
	msg_col = 14;
	if (got_int)
	    goto theend;
	msg_outtrans(ac->cmd);
#ifdef FEAT_EVAL
	if (p_verbose > 0)
	    last_set_msg(ac->script_ctx);
#endif
	if (got_int)
	    goto theend;
	if (ac->next != NULL)
	{
	    msg_putchar('\n');
	    if (got_int)
		goto theend;
	}
    }

theend:
    --autocmd_busy;
}

/*
 * Mark an autocommand pattern for deletion.
 */
    static void
au_remove_pat(AutoPat *ap)
{
    VIM_CLEAR(ap->pat);
    ap->buflocal_nr = -1;
    au_need_clean = TRUE;
}

/*
 * Mark all commands for a pattern for deletion.
 */
    static void
au_remove_cmds(AutoPat *ap)
{
    AutoCmd *ac;

    for (ac = ap->cmds; ac != NULL; ac = ac->next)
	VIM_CLEAR(ac->cmd);
    au_need_clean = TRUE;
}

// Delete one command from an autocmd pattern.
static void au_del_cmd(AutoCmd *ac)
{
    VIM_CLEAR(ac->cmd);
    au_need_clean = TRUE;
}

/*
 * Cleanup autocommands and patterns that have been deleted.
 * This is only done when not executing autocommands.
 */
    static void
au_cleanup(void)
{
    AutoPat	*ap, **prev_ap;
    AutoCmd	*ac, **prev_ac;
    event_T	event;

    if (autocmd_busy || !au_need_clean)
	return;

    // loop over all events
    for (event = (event_T)0; (int)event < NUM_EVENTS;
					    event = (event_T)((int)event + 1))
    {
	// loop over all autocommand patterns
	prev_ap = &(first_autopat[(int)event]);
	for (ap = *prev_ap; ap != NULL; ap = *prev_ap)
	{
	    int has_cmd = FALSE;

	    // loop over all commands for this pattern
	    prev_ac = &(ap->cmds);
	    for (ac = *prev_ac; ac != NULL; ac = *prev_ac)
	    {
		// remove the command if the pattern is to be deleted or when
		// the command has been marked for deletion
		if (ap->pat == NULL || ac->cmd == NULL)
		{
		    *prev_ac = ac->next;
		    vim_free(ac->cmd);
		    vim_free(ac);
		}
		else
		{
		    has_cmd = TRUE;
		    prev_ac = &(ac->next);
		}
	    }

	    if (ap->pat != NULL && !has_cmd)
		// Pattern was not marked for deletion, but all of its
		// commands were.  So mark the pattern for deletion.
		au_remove_pat(ap);

	    // remove the pattern if it has been marked for deletion
	    if (ap->pat == NULL)
	    {
		if (ap->next == NULL)
		{
		    if (prev_ap == &(first_autopat[(int)event]))
			last_autopat[(int)event] = NULL;
		    else
			// this depends on the "next" field being the first in
			// the struct
			last_autopat[(int)event] = (AutoPat *)prev_ap;
		}
		*prev_ap = ap->next;
		vim_regfree(ap->reg_prog);
		vim_free(ap);
	    }
	    else
		prev_ap = &(ap->next);
	}
    }

    au_need_clean = FALSE;
}

/*
 * Called when buffer is freed, to remove/invalidate related buffer-local
 * autocmds.
 */
    void
aubuflocal_remove(buf_T *buf)
{
    AutoPat	    *ap;
    event_T	    event;
    AutoPatCmd_T    *apc;

    // invalidate currently executing autocommands
    for (apc = active_apc_list; apc; apc = apc->next)
	if (buf->b_fnum == apc->arg_bufnr)
	    apc->arg_bufnr = 0;

    // invalidate buflocals looping through events
    for (event = (event_T)0; (int)event < NUM_EVENTS;
					    event = (event_T)((int)event + 1))
	// loop over all autocommand patterns
	FOR_ALL_AUTOCMD_PATTERNS(event, ap)
	    if (ap->buflocal_nr == buf->b_fnum)
	    {
		au_remove_pat(ap);
		if (p_verbose >= 6)
		{
		    verbose_enter();
		    smsg(_("auto-removing autocommand: %s <buffer=%d>"),
					   event_nr2name(event), buf->b_fnum);
		    verbose_leave();
		}
	    }
    au_cleanup();
}

/*
 * Add an autocmd group name.
 * Return its ID.  Returns AUGROUP_ERROR (< 0) for error.
 */
    static int
au_new_group(char_u *name)
{
    int		i;

    i = au_find_group(name);
    if (i != AUGROUP_ERROR)
	return i;

    // the group doesn't exist yet, add it.  First try using a free entry.
    for (i = 0; i < augroups.ga_len; ++i)
	if (AUGROUP_NAME(i) == NULL)
	    break;
    if (i == augroups.ga_len && ga_grow(&augroups, 1) == FAIL)
	return AUGROUP_ERROR;

    AUGROUP_NAME(i) = vim_strsave(name);
    if (AUGROUP_NAME(i) == NULL)
	return AUGROUP_ERROR;
    if (i == augroups.ga_len)
	++augroups.ga_len;

    return i;
}

    static void
au_del_group(char_u *name)
{
    int		i;
    event_T	event;
    AutoPat	*ap;
    int		in_use = FALSE;


    i = au_find_group(name);
    if (i == AUGROUP_ERROR)	// the group doesn't exist
    {
	semsg(_(e_no_such_group_str), name);
	return;
    }
    if (i == current_augroup)
    {
	emsg(_(e_cannot_delete_current_group));
	return;
    }

    for (event = (event_T)0; (int)event < NUM_EVENTS;
	    event = (event_T)((int)event + 1))
    {
	FOR_ALL_AUTOCMD_PATTERNS(event, ap)
	    if (ap->group == i && ap->pat != NULL)
	    {
		give_warning((char_u *)_("W19: Deleting augroup that is still in use"), TRUE);
		in_use = TRUE;
		event = NUM_EVENTS;
		break;
	    }
    }
    vim_free(AUGROUP_NAME(i));
    if (in_use)
	AUGROUP_NAME(i) = get_deleted_augroup();
    else
	AUGROUP_NAME(i) = NULL;
}

/*
 * Find the ID of an autocmd group name.
 * Return its ID.  Returns AUGROUP_ERROR (< 0) for error.
 */
    static int
au_find_group(char_u *name)
{
    int	    i;

    for (i = 0; i < augroups.ga_len; ++i)
	if (AUGROUP_NAME(i) != NULL && AUGROUP_NAME(i) != get_deleted_augroup()
		&& STRCMP(AUGROUP_NAME(i), name) == 0)
	    return i;
    return AUGROUP_ERROR;
}

/*
 * Return TRUE if augroup "name" exists.
 */
    int
au_has_group(char_u *name)
{
    return au_find_group(name) != AUGROUP_ERROR;
}

/*
 * ":augroup {name}".
 */
    void
do_augroup(char_u *arg, int del_group)
{
    int	    i;

    if (del_group)
    {
	if (*arg == NUL)
	    emsg(_(e_argument_required));
	else
	    au_del_group(arg);
    }
    else if (STRICMP(arg, "end") == 0)   // ":aug end": back to group 0
	current_augroup = AUGROUP_DEFAULT;
    else if (*arg)		    // ":aug xxx": switch to group xxx
    {
	i = au_new_group(arg);
	if (i != AUGROUP_ERROR)
	    current_augroup = i;
    }
    else			    // ":aug": list the group names
    {
	msg_start();
	for (i = 0; i < augroups.ga_len; ++i)
	{
	    if (AUGROUP_NAME(i) != NULL)
	    {
		msg_puts((char *)AUGROUP_NAME(i));
		msg_puts("  ");
	    }
	}
	msg_clr_eos();
	msg_end();
    }
}

    void
autocmd_init(void)
{
    CLEAR_FIELD(aucmd_win);
}

#if defined(EXITFREE) || defined(PROTO)
    void
free_all_autocmds(void)
{
    char_u	*s;

    for (current_augroup = -1; current_augroup < augroups.ga_len;
							    ++current_augroup)
	do_autocmd(NULL, (char_u *)"", TRUE);

    for (int i = 0; i < augroups.ga_len; ++i)
    {
	s = ((char_u **)(augroups.ga_data))[i];
	if (s != get_deleted_augroup())
	    vim_free(s);
    }
    ga_clear(&augroups);

    // aucmd_win[] is freed in win_free_all()
}
#endif

/*
 * Return TRUE if "win" is an active entry in aucmd_win[].
 */
    int
is_aucmd_win(win_T *win)
{
    for (int i = 0; i < AUCMD_WIN_COUNT; ++i)
	if (aucmd_win[i].auc_win_used && aucmd_win[i].auc_win == win)
	    return TRUE;
    return FALSE;
}

/*
 * Return the event number for event name "start".
 * Return NUM_EVENTS if the event name was not found.
 * Return a pointer to the next event name in "end".
 */
    static event_T
event_name2nr(char_u *start, char_u **end)
{
    char_u	*p;
    int		i;
    int		len;

    // the event name ends with end of line, '|', a blank or a comma
    for (p = start; *p && !VIM_ISWHITE(*p) && *p != ',' && *p != '|'; ++p)
	;
    for (i = 0; event_names[i].name != NULL; ++i)
    {
	len = (int)STRLEN(event_names[i].name);
	if (len == p - start && STRNICMP(event_names[i].name, start, len) == 0)
	    break;
    }
    if (*p == ',')
	++p;
    *end = p;
    if (event_names[i].name == NULL)
	return NUM_EVENTS;
    return event_names[i].event;
}

/*
 * Return the name for event "event".
 */
    static char_u *
event_nr2name(event_T event)
{
    int	    i;

    for (i = 0; event_names[i].name != NULL; ++i)
	if (event_names[i].event == event)
	    return (char_u *)event_names[i].name;
    return (char_u *)"Unknown";
}

/*
 * Scan over the events.  "*" stands for all events.
 */
    static char_u *
find_end_event(
    char_u  *arg,
    int	    have_group)	    // TRUE when group name was found
{
    char_u  *pat;
    char_u  *p;

    if (*arg == '*')
    {
	if (arg[1] && !VIM_ISWHITE(arg[1]))
	{
	    semsg(_(e_illegal_character_after_star_str), arg);
	    return NULL;
	}
	pat = arg + 1;
    }
    else
    {
	for (pat = arg; *pat && *pat != '|' && !VIM_ISWHITE(*pat); pat = p)
	{
	    if ((int)event_name2nr(pat, &p) >= NUM_EVENTS)
	    {
		if (have_group)
		    semsg(_(e_no_such_event_str), pat);
		else
		    semsg(_(e_no_such_group_or_event_str), pat);
		return NULL;
	    }
	}
    }
    return pat;
}

/*
 * Return TRUE if "event" is included in 'eventignore'.
 */
    static int
event_ignored(event_T event)
{
    char_u	*p = p_ei;

    while (*p != NUL)
    {
	if (STRNICMP(p, "all", 3) == 0 && (p[3] == NUL || p[3] == ','))
	    return TRUE;
	if (event_name2nr(p, &p) == event)
	    return TRUE;
    }

    return FALSE;
}

/*
 * Return OK when the contents of p_ei is valid, FAIL otherwise.
 */
    int
check_ei(void)
{
    char_u	*p = p_ei;

    while (*p)
    {
	if (STRNICMP(p, "all", 3) == 0 && (p[3] == NUL || p[3] == ','))
	{
	    p += 3;
	    if (*p == ',')
		++p;
	}
	else if (event_name2nr(p, &p) == NUM_EVENTS)
	    return FAIL;
    }

    return OK;
}

# if defined(FEAT_SYN_HL) || defined(PROTO)

/*
 * Add "what" to 'eventignore' to skip loading syntax highlighting for every
 * buffer loaded into the window.  "what" must start with a comma.
 * Returns the old value of 'eventignore' in allocated memory.
 */
    char_u *
au_event_disable(char *what)
{
    char_u	*new_ei;
    char_u	*save_ei;

    save_ei = vim_strsave(p_ei);
    if (save_ei == NULL)
	return NULL;

    new_ei = vim_strnsave(p_ei, STRLEN(p_ei) + STRLEN(what));
    if (new_ei == NULL)
    {
	vim_free(save_ei);
	return NULL;
    }

    if (*what == ',' && *p_ei == NUL)
	STRCPY(new_ei, what + 1);
    else
	STRCAT(new_ei, what);
    set_string_option_direct((char_u *)"ei", -1, new_ei,
	    OPT_FREE, SID_NONE);
    vim_free(new_ei);
    return save_ei;
}

    void
au_event_restore(char_u *old_ei)
{
    if (old_ei != NULL)
    {
	set_string_option_direct((char_u *)"ei", -1, old_ei,
							  OPT_FREE, SID_NONE);
	vim_free(old_ei);
    }
}
# endif  // FEAT_SYN_HL

/*
 * do_autocmd() -- implements the :autocmd command.  Can be used in the
 *  following ways:
 *
 * :autocmd <event> <pat> <cmd>	    Add <cmd> to the list of commands that
 *				    will be automatically executed for <event>
 *				    when editing a file matching <pat>, in
 *				    the current group.
 * :autocmd <event> <pat>	    Show the autocommands associated with
 *				    <event> and <pat>.
 * :autocmd <event>		    Show the autocommands associated with
 *				    <event>.
 * :autocmd			    Show all autocommands.
 * :autocmd! <event> <pat> <cmd>    Remove all autocommands associated with
 *				    <event> and <pat>, and add the command
 *				    <cmd>, for the current group.
 * :autocmd! <event> <pat>	    Remove all autocommands associated with
 *				    <event> and <pat> for the current group.
 * :autocmd! <event>		    Remove all autocommands associated with
 *				    <event> for the current group.
 * :autocmd!			    Remove ALL autocommands for the current
 *				    group.
 *
 *  Multiple events and patterns may be given separated by commas.  Here are
 *  some examples:
 * :autocmd bufread,bufenter *.c,*.h	set tw=0 smartindent noic
 * :autocmd bufleave	     *		set tw=79 nosmartindent ic infercase
 *
 * :autocmd * *.c		show all autocommands for *.c files.
 *
 * Mostly a {group} argument can optionally appear before <event>.
 * "eap" can be NULL.
 */
    void
do_autocmd(exarg_T *eap, char_u *arg_in, int forceit)
{
    char_u	*arg = arg_in;
    char_u	*pat;
    char_u	*envpat = NULL;
    char_u	*cmd;
    int		cmd_need_free = FALSE;
    event_T	event;
    char_u	*tofree = NULL;
    int		nested = FALSE;
    int		once = FALSE;
    int		group;
    int		i;
    int		flags = 0;

    if (*arg == '|')
    {
	eap->nextcmd = arg + 1;
	arg = (char_u *)"";
	group = AUGROUP_ALL;	// no argument, use all groups
    }
    else
    {
	/*
	 * Check for a legal group name.  If not, use AUGROUP_ALL.
	 */
	group = au_get_grouparg(&arg);
	if (arg == NULL)	    // out of memory
	    return;
    }

    /*
     * Scan over the events.
     * If we find an illegal name, return here, don't do anything.
     */
    pat = find_end_event(arg, group != AUGROUP_ALL);
    if (pat == NULL)
	return;

    pat = skipwhite(pat);
    if (*pat == '|')
    {
	eap->nextcmd = pat + 1;
	pat = (char_u *)"";
	cmd = (char_u *)"";
    }
    else
    {
	/*
	 * Scan over the pattern.  Put a NUL at the end.
	 */
	cmd = pat;
	while (*cmd && (!VIM_ISWHITE(*cmd) || cmd[-1] == '\\'))
	    cmd++;
	if (*cmd)
	    *cmd++ = NUL;

	// Expand environment variables in the pattern.  Set 'shellslash', we
	// want forward slashes here.
	if (vim_strchr(pat, '$') != NULL || vim_strchr(pat, '~') != NULL)
	{
#ifdef BACKSLASH_IN_FILENAME
	    int	p_ssl_save = p_ssl;

	    p_ssl = TRUE;
#endif
	    envpat = expand_env_save(pat);
#ifdef BACKSLASH_IN_FILENAME
	    p_ssl = p_ssl_save;
#endif
	    if (envpat != NULL)
		pat = envpat;
	}

	cmd = skipwhite(cmd);
	for (i = 0; i < 2; i++)
	{
	    if (*cmd == NUL)
		continue;

	    // Check for "++once" flag.
	    if (STRNCMP(cmd, "++once", 6) == 0 && VIM_ISWHITE(cmd[6]))
	    {
		if (once)
		    semsg(_(e_duplicate_argument_str), "++once");
		once = TRUE;
		cmd = skipwhite(cmd + 6);
	    }

	    // Check for "++nested" flag.
	    if ((STRNCMP(cmd, "++nested", 8) == 0 && VIM_ISWHITE(cmd[8])))
	    {
		if (nested)
		{
		    semsg(_(e_duplicate_argument_str), "++nested");
		    return;
		}
		nested = TRUE;
		cmd = skipwhite(cmd + 8);
	    }

	    // Check for the old "nested" flag in legacy script.
	    if (STRNCMP(cmd, "nested", 6) == 0 && VIM_ISWHITE(cmd[6]))
	    {
		if (in_vim9script())
		{
		    // If there ever is a :nested command this error should
		    // be removed and "nested" accepted as the start of the
		    // command.
		    emsg(_(e_invalid_command_nested_did_you_mean_plusplus_nested));
		    return;
		}
		if (nested)
		{
		    semsg(_(e_duplicate_argument_str), "nested");
		    return;
		}
		nested = TRUE;
		cmd = skipwhite(cmd + 6);
	    }
	}

	/*
	 * Find the start of the commands.
	 * Expand <sfile> in it.
	 */
	if (*cmd != NUL)
	{
	    if (eap != NULL)
		// Read a {} block if it follows.
		cmd = may_get_cmd_block(eap, cmd, &tofree, &flags);

	    cmd = expand_sfile(cmd);
	    if (cmd == NULL)	    // some error
		return;
	    cmd_need_free = TRUE;
	}
    }

    /*
     * Print header when showing autocommands.
     */
    if (!forceit && *cmd == NUL)
	// Highlight title
	msg_puts_title(_("\n--- Autocommands ---"));

    /*
     * Loop over the events.
     */
    last_event = (event_T)-1;		// for listing the event name
    last_group = AUGROUP_ERROR;		// for listing the group name
    if (*arg == '*' || *arg == NUL || *arg == '|')
    {
	if (*cmd != NUL)
	    emsg(_(e_cannot_define_autocommands_for_all_events));
	else
	    for (event = (event_T)0; (int)event < NUM_EVENTS;
					     event = (event_T)((int)event + 1))
		if (do_autocmd_event(event, pat,
			     once, nested, cmd, forceit, group, flags) == FAIL)
		    break;
    }
    else
    {
	while (*arg && *arg != '|' && !VIM_ISWHITE(*arg))
	    if (do_autocmd_event(event_name2nr(arg, &arg), pat,
			  once, nested,	cmd, forceit, group, flags) == FAIL)
		break;
    }

    if (cmd_need_free)
	vim_free(cmd);
    vim_free(tofree);
    vim_free(envpat);
}

/*
 * Find the group ID in a ":autocmd" or ":doautocmd" argument.
 * The "argp" argument is advanced to the following argument.
 *
 * Returns the group ID, AUGROUP_ERROR for error (out of memory).
 */
    static int
au_get_grouparg(char_u **argp)
{
    char_u	*group_name;
    char_u	*p;
    char_u	*arg = *argp;
    int		group = AUGROUP_ALL;

    for (p = arg; *p && !VIM_ISWHITE(*p) && *p != '|'; ++p)
	;
    if (p <= arg)
	return AUGROUP_ALL;

    group_name = vim_strnsave(arg, p - arg);
    if (group_name == NULL)		// out of memory
	return AUGROUP_ERROR;
    group = au_find_group(group_name);
    if (group == AUGROUP_ERROR)
	group = AUGROUP_ALL;	// no match, use all groups
    else
	*argp = skipwhite(p);	// match, skip over group name
    vim_free(group_name);
    return group;
}

/*
 * do_autocmd() for one event.
 * If *pat == NUL do for all patterns.
 * If *cmd == NUL show entries.
 * If forceit == TRUE delete entries.
 * If group is not AUGROUP_ALL, only use this group.
 */
    static int
do_autocmd_event(
    event_T	event,
    char_u	*pat,
    int		once,
    int		nested,
    char_u	*cmd,
    int		forceit,
    int		group,
    int		flags)
{
    AutoPat	*ap;
    AutoPat	**prev_ap;
    AutoCmd	*ac;
    AutoCmd	**prev_ac;
    int		brace_level;
    char_u	*endpat;
    int		findgroup;
    int		allgroups;
    int		patlen;
    int		is_buflocal;
    int		buflocal_nr;
    char_u	buflocal_pat[25];	// for "<buffer=X>"

    if (group == AUGROUP_ALL)
	findgroup = current_augroup;
    else
	findgroup = group;
    allgroups = (group == AUGROUP_ALL && !forceit && *cmd == NUL);

    /*
     * Show or delete all patterns for an event.
     */
    if (*pat == NUL)
    {
	FOR_ALL_AUTOCMD_PATTERNS(event, ap)
	{
	    if (forceit)  // delete the AutoPat, if it's in the current group
	    {
		if (ap->group == findgroup)
		    au_remove_pat(ap);
	    }
	    else if (group == AUGROUP_ALL || ap->group == group)
		show_autocmd(ap, event);
	}
    }

    /*
     * Loop through all the specified patterns.
     */
    for ( ; *pat; pat = (*endpat == ',' ? endpat + 1 : endpat))
    {
	/*
	 * Find end of the pattern.
	 * Watch out for a comma in braces, like "*.\{obj,o\}".
	 */
	brace_level = 0;
	for (endpat = pat; *endpat && (*endpat != ',' || brace_level
			   || (endpat > pat && endpat[-1] == '\\')); ++endpat)
	{
	    if (*endpat == '{')
		brace_level++;
	    else if (*endpat == '}')
		brace_level--;
	}
	if (pat == endpat)		// ignore single comma
	    continue;
	patlen = (int)(endpat - pat);

	/*
	 * detect special <buflocal[=X]> buffer-local patterns
	 */
	is_buflocal = FALSE;
	buflocal_nr = 0;

	if (patlen >= 8 && STRNCMP(pat, "<buffer", 7) == 0
						    && pat[patlen - 1] == '>')
	{
	    // "<buffer...>": Error will be printed only for addition.
	    // printing and removing will proceed silently.
	    is_buflocal = TRUE;
	    if (patlen == 8)
		// "<buffer>"
		buflocal_nr = curbuf->b_fnum;
	    else if (patlen > 9 && pat[7] == '=')
	    {
		if (patlen == 13 && STRNICMP(pat, "<buffer=abuf>", 13) == 0)
		    // "<buffer=abuf>"
		    buflocal_nr = autocmd_bufnr;
		else if (skipdigits(pat + 8) == pat + patlen - 1)
		    // "<buffer=123>"
		    buflocal_nr = atoi((char *)pat + 8);
	    }
	}

	if (is_buflocal)
	{
	    // normalize pat into standard "<buffer>#N" form
	    sprintf((char *)buflocal_pat, "<buffer=%d>", buflocal_nr);
	    pat = buflocal_pat;			// can modify pat and patlen
	    patlen = (int)STRLEN(buflocal_pat);	//   but not endpat
	}

	/*
	 * Find AutoPat entries with this pattern.  When adding a command it
	 * always goes at or after the last one, so start at the end.
	 */
	if (!forceit && *cmd != NUL && last_autopat[(int)event] != NULL)
	    prev_ap = &last_autopat[(int)event];
	else
	    prev_ap = &first_autopat[(int)event];
	while ((ap = *prev_ap) != NULL)
	{
	    if (ap->pat != NULL)
	    {
		/*
		 * Accept a pattern when:
		 * - a group was specified and it's that group, or a group was
		 *   not specified and it's the current group, or a group was
		 *   not specified and we are listing
		 * - the length of the pattern matches
		 * - the pattern matches.
		 * For <buffer[=X]>, this condition works because we normalize
		 * all buffer-local patterns.
		 */
		if ((allgroups || ap->group == findgroup)
			&& ap->patlen == patlen
			&& STRNCMP(pat, ap->pat, patlen) == 0)
		{
		    /*
		     * Remove existing autocommands.
		     * If adding any new autocmd's for this AutoPat, don't
		     * delete the pattern from the autopat list, append to
		     * this list.
		     */
		    if (forceit)
		    {
			if (*cmd != NUL && ap->next == NULL)
			{
			    au_remove_cmds(ap);
			    break;
			}
			au_remove_pat(ap);
		    }

		    /*
		     * Show autocmd's for this autopat, or buflocals <buffer=X>
		     */
		    else if (*cmd == NUL)
			show_autocmd(ap, event);

		    /*
		     * Add autocmd to this autopat, if it's the last one.
		     */
		    else if (ap->next == NULL)
			break;
		}
	    }
	    prev_ap = &ap->next;
	}

	/*
	 * Add a new command.
	 */
	if (*cmd != NUL)
	{
	    /*
	     * If the pattern we want to add a command to does appear at the
	     * end of the list (or not is not in the list at all), add the
	     * pattern at the end of the list.
	     */
	    if (ap == NULL)
	    {
		// refuse to add buffer-local ap if buffer number is invalid
		if (is_buflocal && (buflocal_nr == 0
				      || buflist_findnr(buflocal_nr) == NULL))
		{
		    semsg(_(e_buffer_nr_invalid_buffer_number), buflocal_nr);
		    return FAIL;
		}

		ap = ALLOC_ONE(AutoPat);
		if (ap == NULL)
		    return FAIL;
		ap->pat = vim_strnsave(pat, patlen);
		ap->patlen = patlen;
		if (ap->pat == NULL)
		{
		    vim_free(ap);
		    return FAIL;
		}

#ifdef FEAT_EVAL
		// need to initialize last_mode for the first ModeChanged
		// autocmd
		if (event == EVENT_MODECHANGED && !has_modechanged())
		    get_mode(last_mode);
#endif
		// Initialize the fields checked by the WinScrolled and
		// WinResized trigger to prevent them from firing right after
		// the first autocmd is defined.
		if ((event == EVENT_WINSCROLLED || event == EVENT_WINRESIZED)
			&& !(has_winscrolled() || has_winresized()))
		{
		    tabpage_T *save_curtab = curtab;
		    tabpage_T *tp;
		    FOR_ALL_TABPAGES(tp)
		    {
			unuse_tabpage(curtab);
			use_tabpage(tp);
			snapshot_windows_scroll_size();
		    }
		    unuse_tabpage(curtab);
		    use_tabpage(save_curtab);
		}

		if (is_buflocal)
		{
		    ap->buflocal_nr = buflocal_nr;
		    ap->reg_prog = NULL;
		}
		else
		{
		    char_u	*reg_pat;

		    ap->buflocal_nr = 0;
		    reg_pat = file_pat_to_reg_pat(pat, endpat,
							 &ap->allow_dirs, TRUE);
		    if (reg_pat != NULL)
			ap->reg_prog = vim_regcomp(reg_pat, RE_MAGIC);
		    vim_free(reg_pat);
		    if (reg_pat == NULL || ap->reg_prog == NULL)
		    {
			vim_free(ap->pat);
			vim_free(ap);
			return FAIL;
		    }
		}
		ap->cmds = NULL;
		*prev_ap = ap;
		last_autopat[(int)event] = ap;
		ap->next = NULL;
		if (group == AUGROUP_ALL)
		    ap->group = current_augroup;
		else
		    ap->group = group;
	    }

	    /*
	     * Add the autocmd at the end of the AutoCmd list.
	     */
	    prev_ac = &(ap->cmds);
	    while ((ac = *prev_ac) != NULL)
		prev_ac = &ac->next;
	    ac = ALLOC_ONE(AutoCmd);
	    if (ac == NULL)
		return FAIL;
	    ac->cmd = vim_strsave(cmd);
	    ac->script_ctx = current_sctx;
	    if (flags & UC_VIM9)
		ac->script_ctx.sc_version = SCRIPT_VERSION_VIM9;
#ifdef FEAT_EVAL
	    ac->script_ctx.sc_lnum += SOURCING_LNUM;
#endif
	    if (ac->cmd == NULL)
	    {
		vim_free(ac);
		return FAIL;
	    }
	    ac->next = NULL;
	    *prev_ac = ac;
	    ac->once = once;
	    ac->nested = nested;
	}
    }

    au_cleanup();	// may really delete removed patterns/commands now
    return OK;
}

/*
 * Implementation of ":doautocmd [group] event [fname]".
 * Return OK for success, FAIL for failure;
 */
    int
do_doautocmd(
    char_u	*arg_start,
    int		do_msg,	    // give message for no matching autocmds?
    int		*did_something)
{
    char_u	*arg = arg_start;
    char_u	*fname;
    int		nothing_done = TRUE;
    int		group;

    if (did_something != NULL)
	*did_something = FALSE;

    /*
     * Check for a legal group name.  If not, use AUGROUP_ALL.
     */
    group = au_get_grouparg(&arg);
    if (arg == NULL)	    // out of memory
	return FAIL;

    if (*arg == '*')
    {
	emsg(_(e_cant_execute_autocommands_for_all_events));
	return FAIL;
    }

    /*
     * Scan over the events.
     * If we find an illegal name, return here, don't do anything.
     */
    fname = find_end_event(arg, group != AUGROUP_ALL);
    if (fname == NULL)
	return FAIL;

    fname = skipwhite(fname);

    /*
     * Loop over the events.
     */
    while (*arg && !ends_excmd(*arg) && !VIM_ISWHITE(*arg))
	if (apply_autocmds_group(event_name2nr(arg, &arg),
				      fname, NULL, TRUE, group, curbuf, NULL))
	    nothing_done = FALSE;

    if (nothing_done && do_msg
#ifdef FEAT_EVAL
		&& !aborting()
#endif
	       )
	smsg(_("No matching autocommands: %s"), arg_start);
    if (did_something != NULL)
	*did_something = !nothing_done;

#ifdef FEAT_EVAL
    return aborting() ? FAIL : OK;
#else
    return OK;
#endif
}

/*
 * ":doautoall": execute autocommands for each loaded buffer.
 */
    void
ex_doautoall(exarg_T *eap)
{
    int		retval = OK;
    aco_save_T	aco;
    buf_T	*buf;
    bufref_T	bufref;
    char_u	*arg = eap->arg;
    int		call_do_modelines = check_nomodeline(&arg);
    int		did_aucmd;

    /*
     * This is a bit tricky: For some commands curwin->w_buffer needs to be
     * equal to curbuf, but for some buffers there may not be a window.
     * So we change the buffer for the current window for a moment.  This
     * gives problems when the autocommands make changes to the list of
     * buffers or windows...
     */
    FOR_ALL_BUFFERS(buf)
    {
	// Only do loaded buffers and skip the current buffer, it's done last.
	if (buf->b_ml.ml_mfp == NULL || buf == curbuf)
	    continue;

	// Find a window for this buffer and save some values.
	aucmd_prepbuf(&aco, buf);
	if (curbuf != buf)
	{
	    // Failed to find a window for this buffer.  Better not execute
	    // autocommands then.
	    retval = FAIL;
	    break;
	}

	set_bufref(&bufref, buf);

	// execute the autocommands for this buffer
	retval = do_doautocmd(arg, FALSE, &did_aucmd);

	if (call_do_modelines && did_aucmd)
	    // Execute the modeline settings, but don't set window-local
	    // options if we are using the current window for another
	    // buffer.
	    do_modelines(is_aucmd_win(curwin) ? OPT_NOWIN : 0);

	// restore the current window
	aucmd_restbuf(&aco);

	// stop if there is some error or buffer was deleted
	if (retval == FAIL || !bufref_valid(&bufref))
	{
	    retval = FAIL;
	    break;
	}
    }

    // Execute autocommands for the current buffer last.
    if (retval == OK)
    {
	do_doautocmd(arg, FALSE, &did_aucmd);
	if (call_do_modelines && did_aucmd)
	    do_modelines(0);
    }
}

/*
 * Check *argp for <nomodeline>.  When it is present return FALSE, otherwise
 * return TRUE and advance *argp to after it.
 * Thus return TRUE when do_modelines() should be called.
 */
    int
check_nomodeline(char_u **argp)
{
    if (STRNCMP(*argp, "<nomodeline>", 12) == 0)
    {
	*argp = skipwhite(*argp + 12);
	return FALSE;
    }
    return TRUE;
}

/*
 * Prepare for executing autocommands for (hidden) buffer "buf".
 * Search for a visible window containing the current buffer.  If there isn't
 * one then use an entry in "aucmd_win[]".
 * Set "curbuf" and "curwin" to match "buf".
 * When this fails "curbuf" is not equal "buf".
 */
    void
aucmd_prepbuf(
    aco_save_T	*aco,		// structure to save values in
    buf_T	*buf)		// new curbuf
{
    win_T	*win;
    int		save_ea;
#ifdef FEAT_AUTOCHDIR
    int		save_acd;
#endif

    // Find a window that is for the new buffer
    if (buf == curbuf)		// be quick when buf is curbuf
	win = curwin;
    else
	FOR_ALL_WINDOWS(win)
	    if (win->w_buffer == buf)
		break;

    // Allocate a window when needed.
    win_T *auc_win = NULL;
    int auc_idx = AUCMD_WIN_COUNT;
    if (win == NULL)
    {
	for (auc_idx = 0; auc_idx < AUCMD_WIN_COUNT; ++auc_idx)
	    if (!aucmd_win[auc_idx].auc_win_used)
	    {
		if (aucmd_win[auc_idx].auc_win == NULL)
		    aucmd_win[auc_idx].auc_win = win_alloc_popup_win();
		auc_win = aucmd_win[auc_idx].auc_win;
		if (auc_win != NULL)
		    aucmd_win[auc_idx].auc_win_used = TRUE;
		break;
	    }

	// If this fails (out of memory or using all AUCMD_WIN_COUNT
	// entries) then we can't reliable execute the autocmd, return with
	// "curbuf" unequal "buf".
	if (auc_win == NULL)
	    return;
    }

    aco->save_curwin_id = curwin->w_id;
    aco->save_prevwin_id = prevwin == NULL ? 0 : prevwin->w_id;
    aco->save_State = State;
#ifdef FEAT_JOB_CHANNEL
    if (bt_prompt(curbuf))
	aco->save_prompt_insert = curbuf->b_prompt_insert;
#endif

    if (win != NULL)
    {
	// There is a window for "buf" in the current tab page, make it the
	// curwin.  This is preferred, it has the least side effects (esp. if
	// "buf" is curbuf).
	aco->use_aucmd_win_idx = -1;
	curwin = win;
    }
    else
    {
	// There is no window for "buf", use "auc_win".  To minimize the side
	// effects, insert it in the current tab page.
	// Anything related to a window (e.g., setting folds) may have
	// unexpected results.
	aco->use_aucmd_win_idx = auc_idx;

	win_init_popup_win(auc_win, buf);

	aco->globaldir = globaldir;
	globaldir = NULL;

	// Split the current window, put the auc_win in the upper half.
	// We don't want the BufEnter or WinEnter autocommands.
	block_autocmds();
	make_snapshot(SNAP_AUCMD_IDX);
	save_ea = p_ea;
	p_ea = FALSE;

#ifdef FEAT_AUTOCHDIR
	// Prevent chdir() call in win_enter_ext(), through do_autochdir().
	save_acd = p_acd;
	p_acd = FALSE;
#endif

	(void)win_split_ins(0, WSP_TOP, auc_win, 0);
	(void)win_comp_pos();   // recompute window positions
	p_ea = save_ea;
#ifdef FEAT_AUTOCHDIR
	p_acd = save_acd;
#endif
	unblock_autocmds();
	curwin = auc_win;
    }
    curbuf = buf;
    aco->new_curwin_id = curwin->w_id;
    set_bufref(&aco->new_curbuf, curbuf);

    // disable the Visual area, the position may be invalid in another buffer
    aco->save_VIsual_active = VIsual_active;
    VIsual_active = FALSE;
}

/*
 * Cleanup after executing autocommands for a (hidden) buffer.
 * Restore the window as it was (if possible).
 */
    void
aucmd_restbuf(
    aco_save_T	*aco)		// structure holding saved values
{
    int	    dummy;
    win_T   *save_curwin;

    if (aco->use_aucmd_win_idx >= 0)
    {
	win_T *awp = aucmd_win[aco->use_aucmd_win_idx].auc_win;

	// Find "awp", it can't be closed, but it may be in another tab
	// page. Do not trigger autocommands here.
	block_autocmds();
	if (curwin != awp)
	{
	    tabpage_T	*tp;
	    win_T	*wp;

	    FOR_ALL_TAB_WINDOWS(tp, wp)
	    {
		if (wp == awp)
		{
		    if (tp != curtab)
			goto_tabpage_tp(tp, TRUE, TRUE);
		    win_goto(awp);
		    goto win_found;
		}
	    }
	}
win_found:
	--curbuf->b_nwindows;
#ifdef FEAT_JOB_CHANNEL
	int save_stop_insert_mode = stop_insert_mode;
	// May need to stop Insert mode if we were in a prompt buffer.
	leaving_window(curwin);
	// Do not stop Insert mode when already in Insert mode before.
	if (aco->save_State & MODE_INSERT)
	    stop_insert_mode = save_stop_insert_mode;
#endif
	// Remove the window and frame from the tree of frames.
	(void)winframe_remove(curwin, &dummy, NULL);
	win_remove(curwin, NULL);

	// The window is marked as not used, but it is not freed, it can be
	// used again.
	aucmd_win[aco->use_aucmd_win_idx].auc_win_used = FALSE;
	last_status(FALSE);	    // may need to remove last status line

	if (!valid_tabpage_win(curtab))
	    // no valid window in current tabpage
	    close_tabpage(curtab);

	restore_snapshot(SNAP_AUCMD_IDX, FALSE);
	(void)win_comp_pos();   // recompute window positions
	unblock_autocmds();

	save_curwin = win_find_by_id(aco->save_curwin_id);
	if (save_curwin != NULL)
	    curwin = save_curwin;
	else
	    // Hmm, original window disappeared.  Just use the first one.
	    curwin = firstwin;
	curbuf = curwin->w_buffer;
#ifdef FEAT_JOB_CHANNEL
	// May need to restore insert mode for a prompt buffer.
	entering_window(curwin);
	if (bt_prompt(curbuf))
	    curbuf->b_prompt_insert = aco->save_prompt_insert;
#endif
	prevwin = win_find_by_id(aco->save_prevwin_id);
#ifdef FEAT_EVAL
	vars_clear(&awp->w_vars->dv_hashtab);  // free all w: variables
	hash_init(&awp->w_vars->dv_hashtab);   // re-use the hashtab
#endif
	vim_free(globaldir);
	globaldir = aco->globaldir;

	// the buffer contents may have changed
	VIsual_active = aco->save_VIsual_active;
	check_cursor();
	if (curwin->w_topline > curbuf->b_ml.ml_line_count)
	{
	    curwin->w_topline = curbuf->b_ml.ml_line_count;
#ifdef FEAT_DIFF
	    curwin->w_topfill = 0;
#endif
	}
#if defined(FEAT_GUI)
	if (gui.in_use)
	{
	    // Hide the scrollbars from the "awp" and update.
	    gui_mch_enable_scrollbar(&awp->w_scrollbars[SBAR_LEFT], FALSE);
	    gui_mch_enable_scrollbar(&awp->w_scrollbars[SBAR_RIGHT], FALSE);
	    gui_may_update_scrollbars();
	}
#endif
    }
    else
    {
	// Restore curwin.  Use the window ID, a window may have been closed
	// and the memory re-used for another one.
	save_curwin = win_find_by_id(aco->save_curwin_id);
	if (save_curwin != NULL)
	{
	    // Restore the buffer which was previously edited by curwin, if
	    // it was changed, we are still the same window and the buffer is
	    // valid.
	    if (curwin->w_id == aco->new_curwin_id
		    && curbuf != aco->new_curbuf.br_buf
		    && bufref_valid(&aco->new_curbuf)
		    && aco->new_curbuf.br_buf->b_ml.ml_mfp != NULL)
	    {
# if defined(FEAT_SYN_HL) || defined(FEAT_SPELL)
		if (curwin->w_s == &curbuf->b_s)
		    curwin->w_s = &aco->new_curbuf.br_buf->b_s;
# endif
		--curbuf->b_nwindows;
		curbuf = aco->new_curbuf.br_buf;
		curwin->w_buffer = curbuf;
		++curbuf->b_nwindows;
	    }

	    curwin = save_curwin;
	    curbuf = curwin->w_buffer;
	    prevwin = win_find_by_id(aco->save_prevwin_id);

	    // In case the autocommand moves the cursor to a position that
	    // does not exist in curbuf.
	    VIsual_active = aco->save_VIsual_active;
	    check_cursor();
	}
    }

    VIsual_active = aco->save_VIsual_active;
    check_cursor();	    // just in case lines got deleted
    if (VIsual_active)
	check_pos(curbuf, &VIsual);
}

static int	autocmd_nested = FALSE;

/*
 * Execute autocommands for "event" and file name "fname".
 * Return TRUE if some commands were executed.
 */
    int
apply_autocmds(
    event_T	event,
    char_u	*fname,	    // NULL or empty means use actual file name
    char_u	*fname_io,  // fname to use for <afile> on cmdline
    int		force,	    // when TRUE, ignore autocmd_busy
    buf_T	*buf)	    // buffer for <abuf>
{
    return apply_autocmds_group(event, fname, fname_io, force,
						      AUGROUP_ALL, buf, NULL);
}

/*
 * Like apply_autocmds(), but with extra "eap" argument.  This takes care of
 * setting v:filearg.
 */
    int
apply_autocmds_exarg(
    event_T	event,
    char_u	*fname,
    char_u	*fname_io,
    int		force,
    buf_T	*buf,
    exarg_T	*eap)
{
    return apply_autocmds_group(event, fname, fname_io, force,
						       AUGROUP_ALL, buf, eap);
}

/*
 * Like apply_autocmds(), but handles the caller's retval.  If the script
 * processing is being aborted or if retval is FAIL when inside a try
 * conditional, no autocommands are executed.  If otherwise the autocommands
 * cause the script to be aborted, retval is set to FAIL.
 */
    int
apply_autocmds_retval(
    event_T	event,
    char_u	*fname,	    // NULL or empty means use actual file name
    char_u	*fname_io,  // fname to use for <afile> on cmdline
    int		force,	    // when TRUE, ignore autocmd_busy
    buf_T	*buf,	    // buffer for <abuf>
    int		*retval)    // pointer to caller's retval
{
    int		did_cmd;

#ifdef FEAT_EVAL
    if (should_abort(*retval))
	return FALSE;
#endif

    did_cmd = apply_autocmds_group(event, fname, fname_io, force,
						      AUGROUP_ALL, buf, NULL);
    if (did_cmd
#ifdef FEAT_EVAL
	    && aborting()
#endif
	    )
	*retval = FAIL;
    return did_cmd;
}

/*
 * Return TRUE when there is a CursorHold autocommand defined.
 */
    static int
has_cursorhold(void)
{
    return (first_autopat[(int)(get_real_state() == MODE_NORMAL_BUSY
			    ? EVENT_CURSORHOLD : EVENT_CURSORHOLDI)] != NULL);
}

/*
 * Return TRUE if the CursorHold event can be triggered.
 */
    int
trigger_cursorhold(void)
{
    int		state;

    if (!did_cursorhold
	    && has_cursorhold()
	    && reg_recording == 0
	    && typebuf.tb_len == 0
	    && !ins_compl_active())
    {
	state = get_real_state();
	if (state == MODE_NORMAL_BUSY || (state & MODE_INSERT) != 0)
	    return TRUE;
    }
    return FALSE;
}

/*
 * Return TRUE when there is a WinResized autocommand defined.
 */
    int
has_winresized(void)
{
    return (first_autopat[(int)EVENT_WINRESIZED] != NULL);
}

/*
 * Return TRUE when there is a WinScrolled autocommand defined.
 */
    int
has_winscrolled(void)
{
    return (first_autopat[(int)EVENT_WINSCROLLED] != NULL);
}

/*
 * Return TRUE when there is a CursorMoved autocommand defined.
 */
    int
has_cursormoved(void)
{
    return (first_autopat[(int)EVENT_CURSORMOVED] != NULL);
}

/*
 * Return TRUE when there is a CursorMovedI autocommand defined.
 */
    int
has_cursormovedI(void)
{
    return (first_autopat[(int)EVENT_CURSORMOVEDI] != NULL);
}

/*
 * Return TRUE when there is a TextChanged autocommand defined.
 */
    int
has_textchanged(void)
{
    return (first_autopat[(int)EVENT_TEXTCHANGED] != NULL);
}

/*
 * Return TRUE when there is a TextChangedI autocommand defined.
 */
    int
has_textchangedI(void)
{
    return (first_autopat[(int)EVENT_TEXTCHANGEDI] != NULL);
}

/*
 * Return TRUE when there is a TextChangedP autocommand defined.
 */
    int
has_textchangedP(void)
{
    return (first_autopat[(int)EVENT_TEXTCHANGEDP] != NULL);
}

/*
 * Return TRUE when there is an InsertCharPre autocommand defined.
 */
    int
has_insertcharpre(void)
{
    return (first_autopat[(int)EVENT_INSERTCHARPRE] != NULL);
}

/*
 * Return TRUE when there is an CmdUndefined autocommand defined.
 */
    int
has_cmdundefined(void)
{
    return (first_autopat[(int)EVENT_CMDUNDEFINED] != NULL);
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return TRUE when there is a TextYankPost autocommand defined.
 */
    int
has_textyankpost(void)
{
    return (first_autopat[(int)EVENT_TEXTYANKPOST] != NULL);
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return TRUE when there is a CompleteChanged autocommand defined.
 */
    int
has_completechanged(void)
{
    return (first_autopat[(int)EVENT_COMPLETECHANGED] != NULL);
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return TRUE when there is a ModeChanged autocommand defined.
 */
    int
has_modechanged(void)
{
    return (first_autopat[(int)EVENT_MODECHANGED] != NULL);
}
#endif

/*
 * Execute autocommands for "event" and file name "fname".
 * Return TRUE if some commands were executed.
 */
    static int
apply_autocmds_group(
    event_T	event,
    char_u	*fname,	     // NULL or empty means use actual file name
    char_u	*fname_io,   // fname to use for <afile> on cmdline, NULL means
			     // use fname
    int		force,	     // when TRUE, ignore autocmd_busy
    int		group,	     // group ID, or AUGROUP_ALL
    buf_T	*buf,	     // buffer for <abuf>
    exarg_T	*eap UNUSED) // command arguments
{
    char_u	*sfname = NULL;	// short file name
    char_u	*tail;
    int		save_changed;
    buf_T	*old_curbuf;
    int		retval = FALSE;
    char_u	*save_autocmd_fname;
    int		save_autocmd_fname_full;
    int		save_autocmd_bufnr;
    char_u	*save_autocmd_match;
    int		save_autocmd_busy;
    int		save_autocmd_nested;
    static int	nesting = 0;
    AutoPatCmd_T patcmd;
    AutoPat	*ap;
    sctx_T	save_current_sctx;
#ifdef FEAT_EVAL
    funccal_entry_T funccal_entry;
    char_u	*save_cmdarg;
    long	save_cmdbang;
#endif
    static int	filechangeshell_busy = FALSE;
#ifdef FEAT_PROFILE
    proftime_T	wait_time;
#endif
    int		did_save_redobuff = FALSE;
    save_redo_T	save_redo;
    int		save_KeyTyped = KeyTyped;
    ESTACK_CHECK_DECLARATION;

    /*
     * Quickly return if there are no autocommands for this event or
     * autocommands are blocked.
     */
    if (event == NUM_EVENTS || first_autopat[(int)event] == NULL
	    || autocmd_blocked > 0)
	goto BYPASS_AU;

    /*
     * When autocommands are busy, new autocommands are only executed when
     * explicitly enabled with the "nested" flag.
     */
    if (autocmd_busy && !(force || autocmd_nested))
	goto BYPASS_AU;

#ifdef FEAT_EVAL
    /*
     * Quickly return when immediately aborting on error, or when an interrupt
     * occurred or an exception was thrown but not caught.
     */
    if (aborting())
	goto BYPASS_AU;
#endif

    /*
     * FileChangedShell never nests, because it can create an endless loop.
     */
    if (filechangeshell_busy && (event == EVENT_FILECHANGEDSHELL
				      || event == EVENT_FILECHANGEDSHELLPOST))
	goto BYPASS_AU;

    /*
     * Ignore events in 'eventignore'.
     */
    if (event_ignored(event))
	goto BYPASS_AU;

    /*
     * Allow nesting of autocommands, but restrict the depth, because it's
     * possible to create an endless loop.
     */
    if (nesting == 10)
    {
	emsg(_(e_autocommand_nesting_too_deep));
	goto BYPASS_AU;
    }

    /*
     * Check if these autocommands are disabled.  Used when doing ":all" or
     * ":ball".
     */
    if (       (autocmd_no_enter
		&& (event == EVENT_WINENTER || event == EVENT_BUFENTER))
	    || (autocmd_no_leave
		&& (event == EVENT_WINLEAVE || event == EVENT_BUFLEAVE)))
	goto BYPASS_AU;

    if (event == EVENT_CMDLINECHANGED)
	++aucmd_cmdline_changed_count;

    /*
     * Save the autocmd_* variables and info about the current buffer.
     */
    save_autocmd_fname = autocmd_fname;
    save_autocmd_fname_full = autocmd_fname_full;
    save_autocmd_bufnr = autocmd_bufnr;
    save_autocmd_match = autocmd_match;
    save_autocmd_busy = autocmd_busy;
    save_autocmd_nested = autocmd_nested;
    save_changed = curbuf->b_changed;
    old_curbuf = curbuf;

    /*
     * Set the file name to be used for <afile>.
     * Make a copy to avoid that changing a buffer name or directory makes it
     * invalid.
     */
    if (fname_io == NULL)
    {
	if (event == EVENT_COLORSCHEME || event == EVENT_COLORSCHEMEPRE
						 || event == EVENT_OPTIONSET
						 || event == EVENT_MODECHANGED
						 || event == EVENT_TERMRESPONSEALL)
	    autocmd_fname = NULL;
	else if (fname != NULL && !ends_excmd(*fname))
	    autocmd_fname = fname;
	else if (buf != NULL)
	    autocmd_fname = buf->b_ffname;
	else
	    autocmd_fname = NULL;
    }
    else
	autocmd_fname = fname_io;
    if (autocmd_fname != NULL)
	autocmd_fname = vim_strsave(autocmd_fname);
    autocmd_fname_full = FALSE; // call FullName_save() later

    /*
     * Set the buffer number to be used for <abuf>.
     */
    if (buf == NULL)
	autocmd_bufnr = 0;
    else
	autocmd_bufnr = buf->b_fnum;

    /*
     * When the file name is NULL or empty, use the file name of buffer "buf".
     * Always use the full path of the file name to match with, in case
     * "allow_dirs" is set.
     */
    if (fname == NULL || *fname == NUL)
    {
	if (buf == NULL)
	    fname = NULL;
	else
	{
#ifdef FEAT_SYN_HL
	    if (event == EVENT_SYNTAX)
		fname = buf->b_p_syn;
	    else
#endif
		if (event == EVENT_FILETYPE)
		    fname = buf->b_p_ft;
		else
		{
		    if (buf->b_sfname != NULL)
			sfname = vim_strsave(buf->b_sfname);
		    fname = buf->b_ffname;
		}
	}
	if (fname == NULL)
	    fname = (char_u *)"";
	fname = vim_strsave(fname);	// make a copy, so we can change it
    }
    else
    {
	sfname = vim_strsave(fname);
	// Don't try expanding FileType, Syntax, FuncUndefined, WindowID,
	// ColorScheme, QuickFixCmd*, DirChanged and similar.
	if (event == EVENT_FILETYPE
		|| event == EVENT_SYNTAX
		|| event == EVENT_CMDLINECHANGED
		|| event == EVENT_CMDLINEENTER
		|| event == EVENT_CMDLINELEAVE
		|| event == EVENT_CMDWINENTER
		|| event == EVENT_CMDWINLEAVE
		|| event == EVENT_CMDUNDEFINED
		|| event == EVENT_FUNCUNDEFINED
		|| event == EVENT_REMOTEREPLY
		|| event == EVENT_SPELLFILEMISSING
		|| event == EVENT_QUICKFIXCMDPRE
		|| event == EVENT_COLORSCHEME
		|| event == EVENT_COLORSCHEMEPRE
		|| event == EVENT_OPTIONSET
		|| event == EVENT_QUICKFIXCMDPOST
		|| event == EVENT_DIRCHANGED
		|| event == EVENT_DIRCHANGEDPRE
		|| event == EVENT_MODECHANGED
		|| event == EVENT_MENUPOPUP
		|| event == EVENT_USER
		|| event == EVENT_WINCLOSED
		|| event == EVENT_WINRESIZED
		|| event == EVENT_WINSCROLLED
		|| event == EVENT_TERMRESPONSEALL)
	{
	    fname = vim_strsave(fname);
	    autocmd_fname_full = TRUE; // don't expand it later
	}
	else
	    fname = FullName_save(fname, FALSE);
    }
    if (fname == NULL)	    // out of memory
    {
	vim_free(sfname);
	retval = FALSE;
	goto BYPASS_AU;
    }

#ifdef BACKSLASH_IN_FILENAME
    /*
     * Replace all backslashes with forward slashes.  This makes the
     * autocommand patterns portable between Unix and MS-DOS.
     */
    if (sfname != NULL)
	forward_slash(sfname);
    forward_slash(fname);
#endif

#ifdef VMS
    // remove version for correct match
    if (sfname != NULL)
	vms_remove_version(sfname);
    vms_remove_version(fname);
#endif

    /*
     * Set the name to be used for <amatch>.
     */
    autocmd_match = fname;


    // Don't redraw while doing autocommands.
    ++RedrawingDisabled;

    // name and lnum are filled in later
    estack_push(ETYPE_AUCMD, NULL, 0);
    ESTACK_CHECK_SETUP;

    save_current_sctx = current_sctx;

#ifdef FEAT_EVAL
# ifdef FEAT_PROFILE
    if (do_profiling == PROF_YES)
	prof_child_enter(&wait_time); // doesn't count for the caller itself
# endif

    // Don't use local function variables, if called from a function.
    save_funccal(&funccal_entry);
#endif

    /*
     * When starting to execute autocommands, save the search patterns.
     */
    if (!autocmd_busy)
    {
	save_search_patterns();
	if (!ins_compl_active())
	{
	    saveRedobuff(&save_redo);
	    did_save_redobuff = TRUE;
	}
	did_filetype = keep_filetype;
    }

    /*
     * Note that we are applying autocmds.  Some commands need to know.
     */
    autocmd_busy = TRUE;
    filechangeshell_busy = (event == EVENT_FILECHANGEDSHELL);
    ++nesting;		// see matching decrement below

    // Remember that FileType was triggered.  Used for did_filetype().
    if (event == EVENT_FILETYPE)
	did_filetype = TRUE;

    tail = gettail(fname);

    // Find first autocommand that matches
    CLEAR_FIELD(patcmd);
    patcmd.curpat = first_autopat[(int)event];
    patcmd.group = group;
    patcmd.fname = fname;
    patcmd.sfname = sfname;
    patcmd.tail = tail;
    patcmd.event = event;
    patcmd.arg_bufnr = autocmd_bufnr;
    auto_next_pat(&patcmd, FALSE);

    // found one, start executing the autocommands
    if (patcmd.curpat != NULL)
    {
	// add to active_apc_list
	patcmd.next = active_apc_list;
	active_apc_list = &patcmd;

#ifdef FEAT_EVAL
	// set v:cmdarg (only when there is a matching pattern)
	save_cmdbang = (long)get_vim_var_nr(VV_CMDBANG);
	if (eap != NULL)
	{
	    save_cmdarg = set_cmdarg(eap, NULL);
	    set_vim_var_nr(VV_CMDBANG, (long)eap->forceit);
	}
	else
	    save_cmdarg = NULL;	// avoid gcc warning
#endif
	retval = TRUE;
	// mark the last pattern, to avoid an endless loop when more patterns
	// are added when executing autocommands
	for (ap = patcmd.curpat; ap->next != NULL; ap = ap->next)
	    ap->last = FALSE;
	ap->last = TRUE;

	// Make sure cursor and topline are valid.  The first time the current
	// values are saved, restored by reset_lnums().  When nested only the
	// values are corrected when needed.
	if (nesting == 1)
	    check_lnums(TRUE);
	else
	    check_lnums_nested(TRUE);

	int save_did_emsg = did_emsg;
	int save_ex_pressedreturn = get_pressedreturn();

	do_cmdline(NULL, getnextac, (void *)&patcmd,
				     DOCMD_NOWAIT|DOCMD_VERBOSE|DOCMD_REPEAT);

	did_emsg += save_did_emsg;
	set_pressedreturn(save_ex_pressedreturn);

	if (nesting == 1)
	    // restore cursor and topline, unless they were changed
	    reset_lnums();

#ifdef FEAT_EVAL
	if (eap != NULL)
	{
	    (void)set_cmdarg(NULL, save_cmdarg);
	    set_vim_var_nr(VV_CMDBANG, save_cmdbang);
	}
#endif
	// delete from active_apc_list
	if (active_apc_list == &patcmd)	    // just in case
	    active_apc_list = patcmd.next;
    }

    if (RedrawingDisabled > 0)
	--RedrawingDisabled;
    autocmd_busy = save_autocmd_busy;
    filechangeshell_busy = FALSE;
    autocmd_nested = save_autocmd_nested;
    vim_free(SOURCING_NAME);
    ESTACK_CHECK_NOW;
    estack_pop();
    vim_free(autocmd_fname);
    autocmd_fname = save_autocmd_fname;
    autocmd_fname_full = save_autocmd_fname_full;
    autocmd_bufnr = save_autocmd_bufnr;
    autocmd_match = save_autocmd_match;
    current_sctx = save_current_sctx;
#ifdef FEAT_EVAL
    restore_funccal();
# ifdef FEAT_PROFILE
    if (do_profiling == PROF_YES)
	prof_child_exit(&wait_time);
# endif
#endif
    KeyTyped = save_KeyTyped;
    vim_free(fname);
    vim_free(sfname);
    --nesting;		// see matching increment above

    /*
     * When stopping to execute autocommands, restore the search patterns and
     * the redo buffer.  Free any buffers in the au_pending_free_buf list and
     * free any windows in the au_pending_free_win list.
     */
    if (!autocmd_busy)
    {
	restore_search_patterns();
	if (did_save_redobuff)
	    restoreRedobuff(&save_redo);
	did_filetype = FALSE;
	while (au_pending_free_buf != NULL)
	{
	    buf_T *b = au_pending_free_buf->b_next;

	    vim_free(au_pending_free_buf);
	    au_pending_free_buf = b;
	}
	while (au_pending_free_win != NULL)
	{
	    win_T *w = au_pending_free_win->w_next;

	    vim_free(au_pending_free_win);
	    au_pending_free_win = w;
	}
    }

    /*
     * Some events don't set or reset the Changed flag.
     * Check if still in the same buffer!
     */
    if (curbuf == old_curbuf
	    && (event == EVENT_BUFREADPOST
		|| event == EVENT_BUFWRITEPOST
		|| event == EVENT_FILEAPPENDPOST
		|| event == EVENT_VIMLEAVE
		|| event == EVENT_VIMLEAVEPRE))
    {
	if (curbuf->b_changed != save_changed)
	    need_maketitle = TRUE;
	curbuf->b_changed = save_changed;
    }

    au_cleanup();	// may really delete removed patterns/commands now

BYPASS_AU:
    // When wiping out a buffer make sure all its buffer-local autocommands
    // are deleted.
    if (event == EVENT_BUFWIPEOUT && buf != NULL)
	aubuflocal_remove(buf);

    if (retval == OK && event == EVENT_FILETYPE)
	au_did_filetype = TRUE;

    return retval;
}

# ifdef FEAT_EVAL
static char_u	*old_termresponse = NULL;
static char_u	*old_termu7resp = NULL;
static char_u	*old_termblinkresp = NULL;
static char_u	*old_termrbgresp = NULL;
static char_u	*old_termrfgresp = NULL;
static char_u	*old_termstyleresp = NULL;
# endif

/*
 * Block triggering autocommands until unblock_autocmd() is called.
 * Can be used recursively, so long as it's symmetric.
 */
    void
block_autocmds(void)
{
# ifdef FEAT_EVAL
    // Remember the value of v:termresponse.
    if (autocmd_blocked == 0)
    {
	old_termresponse = get_vim_var_str(VV_TERMRESPONSE);
	old_termu7resp = get_vim_var_str(VV_TERMU7RESP);
	old_termblinkresp = get_vim_var_str(VV_TERMBLINKRESP);
	old_termrbgresp = get_vim_var_str(VV_TERMRBGRESP);
	old_termrfgresp = get_vim_var_str(VV_TERMRFGRESP);
	old_termstyleresp = get_vim_var_str(VV_TERMSTYLERESP);
    }
# endif
    ++autocmd_blocked;
}

    void
unblock_autocmds(void)
{
    --autocmd_blocked;

# ifdef FEAT_EVAL
    // When v:termresponse, etc, were set while autocommands were blocked,
    // trigger the autocommands now.  Esp. useful when executing a shell
    // command during startup (vimdiff).
    if (autocmd_blocked == 0)
    {
	if (get_vim_var_str(VV_TERMRESPONSE) != old_termresponse)
	{
	    apply_autocmds(EVENT_TERMRESPONSE, NULL, NULL, FALSE, curbuf);
	    apply_autocmds(EVENT_TERMRESPONSEALL, (char_u *)"version", NULL, FALSE, curbuf);
	}
	if (get_vim_var_str(VV_TERMU7RESP) != old_termu7resp)
	{
	    apply_autocmds(EVENT_TERMRESPONSEALL, (char_u *)"ambiguouswidth", NULL, FALSE, curbuf);
	}
	if (get_vim_var_str(VV_TERMBLINKRESP) != old_termblinkresp)
	{
	    apply_autocmds(EVENT_TERMRESPONSEALL, (char_u *)"cursorblink", NULL, FALSE, curbuf);
	}
	if (get_vim_var_str(VV_TERMRBGRESP) != old_termrbgresp)
	{
	    apply_autocmds(EVENT_TERMRESPONSEALL, (char_u *)"background", NULL, FALSE, curbuf);
	}
	if (get_vim_var_str(VV_TERMRFGRESP) != old_termrfgresp)
	{
	    apply_autocmds(EVENT_TERMRESPONSEALL, (char_u *)"foreground", NULL, FALSE, curbuf);
	}
	if (get_vim_var_str(VV_TERMSTYLERESP) != old_termstyleresp)
	{
	    apply_autocmds(EVENT_TERMRESPONSEALL, (char_u *)"cursorshape", NULL, FALSE, curbuf);
	}
    }
# endif
}

    int
is_autocmd_blocked(void)
{
    return autocmd_blocked != 0;
}

/*
 * Find next autocommand pattern that matches.
 */
    static void
auto_next_pat(
    AutoPatCmd_T *apc,
    int		stop_at_last)	    // stop when 'last' flag is set
{
    AutoPat	*ap;
    AutoCmd	*cp;
    char_u	*name;
    char	*s;
    estack_T	*entry;
    char_u	*namep;

    entry = ((estack_T *)exestack.ga_data) + exestack.ga_len - 1;

    // Clear the exestack entry for this ETYPE_AUCMD entry.
    VIM_CLEAR(entry->es_name);
    entry->es_info.aucmd = NULL;

    for (ap = apc->curpat; ap != NULL && !got_int; ap = ap->next)
    {
	apc->curpat = NULL;

	// Only use a pattern when it has not been removed, has commands and
	// the group matches. For buffer-local autocommands only check the
	// buffer number.
	if (ap->pat != NULL && ap->cmds != NULL
		&& (apc->group == AUGROUP_ALL || apc->group == ap->group))
	{
	    // execution-condition
	    if (ap->buflocal_nr == 0
		    ? (match_file_pat(NULL, &ap->reg_prog, apc->fname,
				      apc->sfname, apc->tail, ap->allow_dirs))
		    : ap->buflocal_nr == apc->arg_bufnr)
	    {
		name = event_nr2name(apc->event);
		s = _("%s Autocommands for \"%s\"");
		namep = alloc(STRLEN(s) + STRLEN(name) + ap->patlen + 1);
		if (namep != NULL)
		{
		    sprintf((char *)namep, s, (char *)name, (char *)ap->pat);
		    if (p_verbose >= 8)
		    {
			verbose_enter();
			smsg(_("Executing %s"), namep);
			verbose_leave();
		    }
		}

		// Update the exestack entry for this autocmd.
		entry->es_name = namep;
		entry->es_info.aucmd = apc;

		apc->curpat = ap;
		apc->nextcmd = ap->cmds;
		// mark last command
		for (cp = ap->cmds; cp->next != NULL; cp = cp->next)
		    cp->last = FALSE;
		cp->last = TRUE;
	    }
	    line_breakcheck();
	    if (apc->curpat != NULL)	    // found a match
		break;
	}
	if (stop_at_last && ap->last)
	    break;
    }
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Get the script context where autocommand "acp" is defined.
 */
    sctx_T *
acp_script_ctx(AutoPatCmd_T *acp)
{
    return &acp->script_ctx;
}
#endif

/*
 * Get next autocommand command.
 * Called by do_cmdline() to get the next line for ":if".
 * Returns allocated string, or NULL for end of autocommands.
 */
    char_u *
getnextac(
	int c UNUSED,
	void *cookie,
	int indent UNUSED,
	getline_opt_T options UNUSED)
{
    AutoPatCmd_T    *acp = (AutoPatCmd_T *)cookie;
    char_u	    *retval;
    AutoCmd	    *ac;

    // Can be called again after returning the last line.
    if (acp->curpat == NULL)
	return NULL;

    // repeat until we find an autocommand to execute
    for (;;)
    {
	// skip removed commands
	while (acp->nextcmd != NULL && acp->nextcmd->cmd == NULL)
	    if (acp->nextcmd->last)
		acp->nextcmd = NULL;
	    else
		acp->nextcmd = acp->nextcmd->next;

	if (acp->nextcmd != NULL)
	    break;

	// at end of commands, find next pattern that matches
	if (acp->curpat->last)
	    acp->curpat = NULL;
	else
	    acp->curpat = acp->curpat->next;
	if (acp->curpat != NULL)
	    auto_next_pat(acp, TRUE);
	if (acp->curpat == NULL)
	    return NULL;
    }

    ac = acp->nextcmd;

    if (p_verbose >= 9)
    {
	verbose_enter_scroll();
	smsg(_("autocommand %s"), ac->cmd);
	msg_puts("\n");   // don't overwrite this either
	verbose_leave_scroll();
    }
    retval = vim_strsave(ac->cmd);
    // Remove one-shot ("once") autocmd in anticipation of its execution.
    if (ac->once)
	au_del_cmd(ac);
    autocmd_nested = ac->nested;
    current_sctx = ac->script_ctx;
    acp->script_ctx = current_sctx;
    if (ac->last)
	acp->nextcmd = NULL;
    else
	acp->nextcmd = ac->next;
    return retval;
}

/*
 * Return TRUE if there is a matching autocommand for "fname".
 * To account for buffer-local autocommands, function needs to know
 * in which buffer the file will be opened.
 */
    int
has_autocmd(event_T event, char_u *sfname, buf_T *buf)
{
    AutoPat	*ap;
    char_u	*fname;
    char_u	*tail = gettail(sfname);
    int		retval = FALSE;

    fname = FullName_save(sfname, FALSE);
    if (fname == NULL)
	return FALSE;

#ifdef BACKSLASH_IN_FILENAME
    /*
     * Replace all backslashes with forward slashes.  This makes the
     * autocommand patterns portable between Unix and MS-DOS.
     */
    sfname = vim_strsave(sfname);
    if (sfname != NULL)
	forward_slash(sfname);
    forward_slash(fname);
#endif

    FOR_ALL_AUTOCMD_PATTERNS(event, ap)
	if (ap->pat != NULL && ap->cmds != NULL
	      && (ap->buflocal_nr == 0
		? match_file_pat(NULL, &ap->reg_prog,
					  fname, sfname, tail, ap->allow_dirs)
		: buf != NULL && ap->buflocal_nr == buf->b_fnum
	   ))
	{
	    retval = TRUE;
	    break;
	}

    vim_free(fname);
#ifdef BACKSLASH_IN_FILENAME
    vim_free(sfname);
#endif

    return retval;
}

/*
 * Function given to ExpandGeneric() to obtain the list of autocommand group
 * names.
 */
    char_u *
get_augroup_name(expand_T *xp UNUSED, int idx)
{
    if (idx == augroups.ga_len)		// add "END" add the end
	return (char_u *)"END";
    if (idx < 0 || idx >= augroups.ga_len)	// end of list
	return NULL;
    if (AUGROUP_NAME(idx) == NULL || AUGROUP_NAME(idx) == get_deleted_augroup())
	// skip deleted entries
	return (char_u *)"";
    return AUGROUP_NAME(idx);		// return a name
}

static int include_groups = FALSE;

    char_u  *
set_context_in_autocmd(
    expand_T	*xp,
    char_u	*arg,
    int		doautocmd)	// TRUE for :doauto*, FALSE for :autocmd
{
    char_u	*p;
    int		group;

    // check for a group name, skip it if present
    include_groups = FALSE;
    p = arg;
    group = au_get_grouparg(&arg);
    if (group == AUGROUP_ERROR)
	return NULL;
    // If there only is a group name that's what we expand.
    if (*arg == NUL && group != AUGROUP_ALL && !VIM_ISWHITE(arg[-1]))
    {
	arg = p;
	group = AUGROUP_ALL;
    }

    // skip over event name
    for (p = arg; *p != NUL && !VIM_ISWHITE(*p); ++p)
	if (*p == ',')
	    arg = p + 1;
    if (*p == NUL)
    {
	if (group == AUGROUP_ALL)
	    include_groups = TRUE;
	xp->xp_context = EXPAND_EVENTS;	    // expand event name
	xp->xp_pattern = arg;
	return NULL;
    }

    // skip over pattern
    arg = skipwhite(p);
    while (*arg && (!VIM_ISWHITE(*arg) || arg[-1] == '\\'))
	arg++;
    if (*arg)
	return arg;			    // expand (next) command

    if (doautocmd)
	xp->xp_context = EXPAND_FILES;	    // expand file names
    else
	xp->xp_context = EXPAND_NOTHING;    // pattern is not expanded
    return NULL;
}

/*
 * Function given to ExpandGeneric() to obtain the list of event names.
 */
    char_u *
get_event_name(expand_T *xp UNUSED, int idx)
{
    if (idx < augroups.ga_len)		// First list group names, if wanted
    {
	if (!include_groups || AUGROUP_NAME(idx) == NULL
				 || AUGROUP_NAME(idx) == get_deleted_augroup())
	    return (char_u *)"";	// skip deleted entries
	return AUGROUP_NAME(idx);	// return a name
    }
    return (char_u *)event_names[idx - augroups.ga_len].name;
}

/*
 * Function given to ExpandGeneric() to obtain the list of event names. Don't
 * include groups.
 */
    char_u *
get_event_name_no_group(expand_T *xp UNUSED, int idx)
{
    return (char_u *)event_names[idx].name;
}


#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return TRUE if autocmd is supported.
 */
    int
autocmd_supported(char_u *name)
{
    char_u *p;

    return (event_name2nr(name, &p) != NUM_EVENTS);
}

/*
 * Return TRUE if an autocommand is defined for a group, event and
 * pattern:  The group can be omitted to accept any group. "event" and "pattern"
 * can be NULL to accept any event and pattern. "pattern" can be NULL to accept
 * any pattern. Buffer-local patterns <buffer> or <buffer=N> are accepted.
 * Used for:
 *	exists("#Group") or
 *	exists("#Group#Event") or
 *	exists("#Group#Event#pat") or
 *	exists("#Event") or
 *	exists("#Event#pat")
 */
    int
au_exists(char_u *arg)
{
    char_u	*arg_save;
    char_u	*pattern = NULL;
    char_u	*event_name;
    char_u	*p;
    event_T	event;
    AutoPat	*ap;
    buf_T	*buflocal_buf = NULL;
    int		group;
    int		retval = FALSE;

    // Make a copy so that we can change the '#' chars to a NUL.
    arg_save = vim_strsave(arg);
    if (arg_save == NULL)
	return FALSE;
    p = vim_strchr(arg_save, '#');
    if (p != NULL)
	*p++ = NUL;

    // First, look for an autocmd group name
    group = au_find_group(arg_save);
    if (group == AUGROUP_ERROR)
    {
	// Didn't match a group name, assume the first argument is an event.
	group = AUGROUP_ALL;
	event_name = arg_save;
    }
    else
    {
	if (p == NULL)
	{
	    // "Group": group name is present and it's recognized
	    retval = TRUE;
	    goto theend;
	}

	// Must be "Group#Event" or "Group#Event#pat".
	event_name = p;
	p = vim_strchr(event_name, '#');
	if (p != NULL)
	    *p++ = NUL;	    // "Group#Event#pat"
    }

    pattern = p;	    // "pattern" is NULL when there is no pattern

    // find the index (enum) for the event name
    event = event_name2nr(event_name, &p);

    // return FALSE if the event name is not recognized
    if (event == NUM_EVENTS)
	goto theend;

    // Find the first autocommand for this event.
    // If there isn't any, return FALSE;
    // If there is one and no pattern given, return TRUE;
    ap = first_autopat[(int)event];
    if (ap == NULL)
	goto theend;

    // if pattern is "<buffer>", special handling is needed which uses curbuf
    // for pattern "<buffer=N>, fnamecmp() will work fine
    if (pattern != NULL && STRICMP(pattern, "<buffer>") == 0)
	buflocal_buf = curbuf;

    // Check if there is an autocommand with the given pattern.
    for ( ; ap != NULL; ap = ap->next)
	// only use a pattern when it has not been removed and has commands.
	// For buffer-local autocommands, fnamecmp() works fine.
	if (ap->pat != NULL && ap->cmds != NULL
	    && (group == AUGROUP_ALL || ap->group == group)
	    && (pattern == NULL
		|| (buflocal_buf == NULL
		    ? fnamecmp(ap->pat, pattern) == 0
		    : ap->buflocal_nr == buflocal_buf->b_fnum)))
	{
	    retval = TRUE;
	    break;
	}

theend:
    vim_free(arg_save);
    return retval;
}

/*
 * autocmd_add() and autocmd_delete() functions
 */
    static void
autocmd_add_or_delete(typval_T *argvars, typval_T *rettv, int delete)
{
    list_T	*aucmd_list;
    listitem_T	*li;
    dict_T	*event_dict;
    dictitem_T	*di;
    char_u	*event_name = NULL;
    list_T	*event_list;
    listitem_T	*eli;
    event_T	event;
    char_u	*group_name = NULL;
    int		group;
    char_u	*pat = NULL;
    list_T	*pat_list;
    listitem_T	*pli;
    char_u	*cmd = NULL;
    char_u	*end;
    int		once;
    int		nested;
    int		replace;		// replace the cmd for a group/event
    int		retval = VVAL_TRUE;
    int		save_augroup = current_augroup;

    rettv->v_type = VAR_BOOL;
    rettv->vval.v_number = VVAL_FALSE;

    if (check_for_list_arg(argvars, 0) == FAIL)
	return;

    aucmd_list = argvars[0].vval.v_list;
    if (aucmd_list == NULL)
	return;

    FOR_ALL_LIST_ITEMS(aucmd_list, li)
    {
	VIM_CLEAR(group_name);
	VIM_CLEAR(cmd);
	event_name = NULL;
	event_list = NULL;
	pat = NULL;
	pat_list = NULL;

	if (li->li_tv.v_type != VAR_DICT)
	    continue;

	event_dict = li->li_tv.vval.v_dict;
	if (event_dict == NULL)
	    continue;

	di = dict_find(event_dict, (char_u *)"event", -1);
	if (di != NULL)
	{
	    if (di->di_tv.v_type == VAR_STRING)
	    {
		event_name = di->di_tv.vval.v_string;
		if (event_name == NULL)
		{
		    emsg(_(e_string_required));
		    continue;
		}
	    }
	    else if (di->di_tv.v_type == VAR_LIST)
	    {
		event_list = di->di_tv.vval.v_list;
		if (event_list == NULL)
		{
		    emsg(_(e_list_required));
		    continue;
		}
	    }
	    else
	    {
		emsg(_(e_string_or_list_expected));
		continue;
	    }
	}

	group_name = dict_get_string(event_dict, "group", TRUE);
	if (group_name == NULL || *group_name == NUL)
	    // if the autocmd group name is not specified, then use the current
	    // autocmd group
	    group = current_augroup;
	else
	{
	    group = au_find_group(group_name);
	    if (group == AUGROUP_ERROR)
	    {
		if (delete)
		{
		    semsg(_(e_no_such_group_str), group_name);
		    retval = VVAL_FALSE;
		    break;
		}
		// group is not found, create it now
		group = au_new_group(group_name);
		if (group == AUGROUP_ERROR)
		{
		    semsg(_(e_no_such_group_str), group_name);
		    retval = VVAL_FALSE;
		    break;
		}

		current_augroup = group;
	    }
	}

	// if a buffer number is specified, then generate a pattern of the form
	// "<buffer=n>. Otherwise, use the pattern supplied by the user.
	if (dict_has_key(event_dict, "bufnr"))
	{
	    varnumber_T	bnum;

	    bnum = dict_get_number_def(event_dict, "bufnr", -1);
	    if (bnum == -1)
		continue;

	    vim_snprintf((char *)IObuff, IOSIZE, "<buffer=%d>", (int)bnum);
	    pat = IObuff;
	}
	else
	{
	    di = dict_find(event_dict, (char_u *)"pattern", -1);
	    if (di != NULL)
	    {
		if (di->di_tv.v_type == VAR_STRING)
		{
		    pat = di->di_tv.vval.v_string;
		    if (pat == NULL)
		    {
			emsg(_(e_string_required));
			continue;
		    }
		}
		else if (di->di_tv.v_type == VAR_LIST)
		{
		    pat_list = di->di_tv.vval.v_list;
		    if (pat_list == NULL)
		    {
			emsg(_(e_list_required));
			continue;
		    }
		}
		else
		{
		    emsg(_(e_string_or_list_expected));
		    continue;
		}
	    }
	    else if (delete)
		pat = (char_u *)"";
	}

	once = dict_get_bool(event_dict, "once", FALSE);
	nested = dict_get_bool(event_dict, "nested", FALSE);
	// if 'replace' is true, then remove all the commands associated with
	// this autocmd event/group and add the new command.
	replace = dict_get_bool(event_dict, "replace", FALSE);

	cmd = dict_get_string(event_dict, "cmd", TRUE);
	if (cmd == NULL)
	{
	    if (delete)
		cmd = vim_strsave((char_u *)"");
	    else
		continue;
	}

	if (delete && (event_name == NULL
		    || (event_name[0] == '*' && event_name[1] == NUL)))
	{
	    // if the event name is not specified or '*', delete all the events
	    for (event = (event_T)0; (int)event < NUM_EVENTS;
		    event = (event_T)((int)event + 1))
	    {
		if (do_autocmd_event(event, pat, once, nested, cmd, delete,
							group, 0) == FAIL)
		{
		    retval = VVAL_FALSE;
		    break;
		}
	    }
	}
	else
	{
	    char_u *p = NULL;

	    eli = NULL;
	    end = NULL;
	    while (TRUE)
	    {
		if (event_list != NULL)
		{
		    if (eli == NULL)
			eli = event_list->lv_first;
		    else
			eli = eli->li_next;
		    if (eli == NULL)
			break;
		    if (eli->li_tv.v_type != VAR_STRING
			    || (p = eli->li_tv.vval.v_string) == NULL)
		    {
			emsg(_(e_string_required));
			break;
		    }
		}
		else
		{
		    if (p == NULL)
			p = event_name;
		    if (p == NULL || *p == NUL)
			break;
		}

		event = event_name2nr(p, &end);
		if (event == NUM_EVENTS || *end != NUL)
		{
		    // this also catches something following a valid event name
		    semsg(_(e_no_such_event_str), p);
		    retval = VVAL_FALSE;
		    break;
		}
		if (pat != NULL)
		{
		    if (do_autocmd_event(event, pat, once, nested, cmd,
				delete | replace, group, 0) == FAIL)
		    {
			retval = VVAL_FALSE;
			break;
		    }
		}
		else if (pat_list != NULL)
		{
		    FOR_ALL_LIST_ITEMS(pat_list, pli)
		    {
			if (pli->li_tv.v_type != VAR_STRING
				|| pli->li_tv.vval.v_string == NULL)
			{
			    emsg(_(e_string_required));
			    continue;
			}
			if (do_autocmd_event(event,
				    pli->li_tv.vval.v_string, once, nested,
				    cmd, delete | replace, group, 0) ==
				FAIL)
			{
			    retval = VVAL_FALSE;
			    break;
			}
		    }
		    if (retval == VVAL_FALSE)
			break;
		}
		if (event_name != NULL)
		    p = end;
	    }
	}

	// if only the autocmd group name is specified for delete and the
	// autocmd event, pattern and cmd are not specified, then delete the
	// autocmd group.
	if (delete && group_name != NULL &&
		(event_name == NULL || event_name[0] == NUL)
		&& (pat == NULL || pat[0] == NUL)
		&& (cmd == NULL || cmd[0] == NUL))
	    au_del_group(group_name);
    }

    VIM_CLEAR(group_name);
    VIM_CLEAR(cmd);

    current_augroup = save_augroup;
    rettv->vval.v_number = retval;
}

/*
 * autocmd_add() function
 */
    void
f_autocmd_add(typval_T *argvars, typval_T *rettv)
{
    autocmd_add_or_delete(argvars, rettv, FALSE);
}

/*
 * autocmd_delete() function
 */
    void
f_autocmd_delete(typval_T *argvars, typval_T *rettv)
{
    autocmd_add_or_delete(argvars, rettv, TRUE);
}

/*
 * autocmd_get() function
 * Returns a List of autocmds.
 */
    void
f_autocmd_get(typval_T *argvars, typval_T *rettv)
{
    event_T	event_arg = NUM_EVENTS;
    event_T	event;
    AutoPat	*ap;
    AutoCmd	*ac;
    list_T	*event_list;
    dict_T	*event_dict;
    char_u	*event_name = NULL;
    char_u	*pat = NULL;
    char_u	*name = NULL;
    int		group = AUGROUP_ALL;

    if (rettv_list_alloc(rettv) == FAIL)
	return;
    if (check_for_opt_dict_arg(argvars, 0) == FAIL)
	return;

    if (argvars[0].v_type == VAR_DICT)
    {
	// return only the autocmds in the specified group
	if (dict_has_key(argvars[0].vval.v_dict, "group"))
	{
	    name = dict_get_string(argvars[0].vval.v_dict, "group", TRUE);
	    if (name == NULL)
		return;

	    if (*name == NUL)
		group = AUGROUP_DEFAULT;
	    else
	    {
		group = au_find_group(name);
		if (group == AUGROUP_ERROR)
		{
		    semsg(_(e_no_such_group_str), name);
		    vim_free(name);
		    return;
		}
	    }
	    vim_free(name);
	}

	// return only the autocmds for the specified event
	if (dict_has_key(argvars[0].vval.v_dict, "event"))
	{
	    int		i;

	    name = dict_get_string(argvars[0].vval.v_dict, "event", TRUE);
	    if (name == NULL)
		return;

	    if (name[0] == '*' && name[1] == NUL)
		event_arg = NUM_EVENTS;
	    else
	    {
		for (i = 0; event_names[i].name != NULL; i++)
		    if (STRICMP(event_names[i].name, name) == 0)
			break;
		if (event_names[i].name == NULL)
		{
		    semsg(_(e_no_such_event_str), name);
		    vim_free(name);
		    return;
		}
		event_arg = event_names[i].event;
	    }
	    vim_free(name);
	}

	// return only the autocmds for the specified pattern
	if (dict_has_key(argvars[0].vval.v_dict, "pattern"))
	{
	    pat = dict_get_string(argvars[0].vval.v_dict, "pattern", TRUE);
	    if (pat == NULL)
		return;
	}
    }

    event_list = rettv->vval.v_list;

    // iterate through all the autocmd events
    for (event = (event_T)0; (int)event < NUM_EVENTS;
	    event = (event_T)((int)event + 1))
    {
	if (event_arg != NUM_EVENTS && event != event_arg)
	    continue;

	event_name = event_nr2name(event);

	// iterate through all the patterns for this autocmd event
	FOR_ALL_AUTOCMD_PATTERNS(event, ap)
	{
	    char_u	*group_name;

	    if (group != AUGROUP_ALL && group != ap->group)
		continue;

	    if (pat != NULL && STRCMP(pat, ap->pat) != 0)
		continue;

	    group_name = get_augroup_name(NULL, ap->group);

	    // iterate through all the commands for this pattern and add one
	    // item for each cmd.
	    for (ac = ap->cmds; ac != NULL; ac = ac->next)
	    {
		event_dict = dict_alloc();
		if (event_dict == NULL
			|| list_append_dict(event_list, event_dict) == FAIL)
		    return;

		if (dict_add_string(event_dict, "event", event_name) == FAIL
			|| dict_add_string(event_dict, "group",
					group_name == NULL ? (char_u *)""
							  : group_name) == FAIL
			|| (ap->buflocal_nr != 0
				&& (dict_add_number(event_dict, "bufnr",
						    ap->buflocal_nr) == FAIL))
			|| dict_add_string(event_dict, "pattern",
							      ap->pat) == FAIL
			|| dict_add_string(event_dict, "cmd", ac->cmd) == FAIL
			|| dict_add_bool(event_dict, "once", ac->once) == FAIL
			|| dict_add_bool(event_dict, "nested",
							   ac->nested) == FAIL)
		    return;
	    }
	}
    }

    vim_free(pat);
}

#endif
