/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * usercmd.c: User defined command support
 */

#include "vim.h"

typedef struct ucmd
{
    char_u	*uc_name;	// The command name
    long_u	uc_argt;	// The argument type
    char_u	*uc_rep;	// The command's replacement string
    long	uc_def;		// The default value for a range/count
    int		uc_compl;	// completion type
    cmd_addr_T	uc_addr_type;	// The command's address type
    sctx_T	uc_script_ctx;	// SCTX where the command was defined
    int		uc_flags;	// some UC_ flags
# ifdef FEAT_EVAL
    char_u	*uc_compl_arg;	// completion argument if any
# endif
} ucmd_T;

// List of all user commands.
static garray_T ucmds = {0, 0, sizeof(ucmd_T), 4, NULL};

// When non-zero it is not allowed to add or remove user commands
static int ucmd_locked = 0;

#define USER_CMD(i) (&((ucmd_T *)(ucmds.ga_data))[i])
#define USER_CMD_GA(gap, i) (&((ucmd_T *)((gap)->ga_data))[i])

/*
 * List of names for completion for ":command" with the EXPAND_ flag.
 * Must be alphabetical for completion.
 */
static struct
{
    int	    expand;
    char    *name;
} command_complete[] =
{
    {EXPAND_ARGLIST, "arglist"},
    {EXPAND_AUGROUP, "augroup"},
    {EXPAND_BEHAVE, "behave"},
    {EXPAND_BUFFERS, "buffer"},
    {EXPAND_COLORS, "color"},
    {EXPAND_COMMANDS, "command"},
    {EXPAND_COMPILER, "compiler"},
#if defined(FEAT_CSCOPE)
    {EXPAND_CSCOPE, "cscope"},
#endif
#if defined(FEAT_EVAL)
    {EXPAND_USER_DEFINED, "custom"},
    {EXPAND_USER_LIST, "customlist"},
#endif
    {EXPAND_DIFF_BUFFERS, "diff_buffer"},
    {EXPAND_DIRECTORIES, "dir"},
    {EXPAND_ENV_VARS, "environment"},
    {EXPAND_EVENTS, "event"},
    {EXPAND_EXPRESSION, "expression"},
    {EXPAND_FILES, "file"},
    {EXPAND_FILES_IN_PATH, "file_in_path"},
    {EXPAND_FILETYPE, "filetype"},
    {EXPAND_FUNCTIONS, "function"},
    {EXPAND_HELP, "help"},
    {EXPAND_HIGHLIGHT, "highlight"},
    {EXPAND_HISTORY, "history"},
#if defined(FEAT_KEYMAP)
    {EXPAND_KEYMAP, "keymap"},
#endif
#if defined(HAVE_LOCALE_H) || defined(X_LOCALE)
    {EXPAND_LOCALES, "locale"},
#endif
    {EXPAND_MAPCLEAR, "mapclear"},
    {EXPAND_MAPPINGS, "mapping"},
    {EXPAND_MENUS, "menu"},
    {EXPAND_MESSAGES, "messages"},
    {EXPAND_OWNSYNTAX, "syntax"},
#if defined(FEAT_PROFILE)
    {EXPAND_SYNTIME, "syntime"},
#endif
    {EXPAND_SETTINGS, "option"},
    {EXPAND_PACKADD, "packadd"},
    {EXPAND_RUNTIME, "runtime"},
    {EXPAND_SHELLCMD, "shellcmd"},
#if defined(FEAT_SIGNS)
    {EXPAND_SIGN, "sign"},
#endif
    {EXPAND_TAGS, "tag"},
    {EXPAND_TAGS_LISTFILES, "tag_listfiles"},
    {EXPAND_USER, "user"},
    {EXPAND_USER_VARS, "var"},
#if defined(FEAT_EVAL)
    {EXPAND_BREAKPOINT, "breakpoint"},
    {EXPAND_SCRIPTNAMES, "scriptnames"},
#endif
    {0, NULL}
};

/*
 * List of names of address types.  Must be alphabetical for completion.
 */
static struct
{
    cmd_addr_T	expand;
    char	*name;
    char	*shortname;
} addr_type_complete[] =
{
    {ADDR_ARGUMENTS, "arguments", "arg"},
    {ADDR_LINES, "lines", "line"},
    {ADDR_LOADED_BUFFERS, "loaded_buffers", "load"},
    {ADDR_TABS, "tabs", "tab"},
    {ADDR_BUFFERS, "buffers", "buf"},
    {ADDR_WINDOWS, "windows", "win"},
    {ADDR_QUICKFIX, "quickfix", "qf"},
    {ADDR_OTHER, "other", "?"},
    {ADDR_NONE, NULL, NULL}
};

/*
 * Search for a user command that matches "eap->cmd".
 * Return cmdidx in "eap->cmdidx", flags in "eap->argt", idx in "eap->useridx".
 * Return a pointer to just after the command.
 * Return NULL if there is no matching command.
 */
    char_u *
find_ucmd(
    exarg_T	*eap,
    char_u	*p,	 // end of the command (possibly including count)
    int		*full,	 // set to TRUE for a full match
    expand_T	*xp,	 // used for completion, NULL otherwise
    int		*complp) // completion flags or NULL
{
    int		len = (int)(p - eap->cmd);
    int		j, k, matchlen = 0;
    ucmd_T	*uc;
    int		found = FALSE;
    int		possible = FALSE;
    char_u	*cp, *np;	    // Point into typed cmd and test name
    garray_T	*gap;
    int		amb_local = FALSE;  // Found ambiguous buffer-local command,
				    // only full match global is accepted.

    /*
     * Look for buffer-local user commands first, then global ones.
     */
    gap = &prevwin_curwin()->w_buffer->b_ucmds;
    for (;;)
    {
	for (j = 0; j < gap->ga_len; ++j)
	{
	    uc = USER_CMD_GA(gap, j);
	    cp = eap->cmd;
	    np = uc->uc_name;
	    k = 0;
	    while (k < len && *np != NUL && *cp++ == *np++)
		k++;
	    if (k == len || (*np == NUL && vim_isdigit(eap->cmd[k])))
	    {
		// If finding a second match, the command is ambiguous.  But
		// not if a buffer-local command wasn't a full match and a
		// global command is a full match.
		if (k == len && found && *np != NUL)
		{
		    if (gap == &ucmds)
			return NULL;
		    amb_local = TRUE;
		}

		if (!found || (k == len && *np == NUL))
		{
		    // If we matched up to a digit, then there could
		    // be another command including the digit that we
		    // should use instead.
		    if (k == len)
			found = TRUE;
		    else
			possible = TRUE;

		    if (gap == &ucmds)
			eap->cmdidx = CMD_USER;
		    else
			eap->cmdidx = CMD_USER_BUF;
		    eap->argt = (long)uc->uc_argt;
		    eap->useridx = j;
		    eap->addr_type = uc->uc_addr_type;

		    if (complp != NULL)
			*complp = uc->uc_compl;
# ifdef FEAT_EVAL
		    if (xp != NULL)
		    {
			xp->xp_arg = uc->uc_compl_arg;
			xp->xp_script_ctx = uc->uc_script_ctx;
			xp->xp_script_ctx.sc_lnum += SOURCING_LNUM;
		    }
# endif
		    // Do not search for further abbreviations
		    // if this is an exact match.
		    matchlen = k;
		    if (k == len && *np == NUL)
		    {
			if (full != NULL)
			    *full = TRUE;
			amb_local = FALSE;
			break;
		    }
		}
	    }
	}

	// Stop if we found a full match or searched all.
	if (j < gap->ga_len || gap == &ucmds)
	    break;
	gap = &ucmds;
    }

    // Only found ambiguous matches.
    if (amb_local)
    {
	if (xp != NULL)
	    xp->xp_context = EXPAND_UNSUCCESSFUL;
	return NULL;
    }

    // The match we found may be followed immediately by a number.  Move "p"
    // back to point to it.
    if (found || possible)
	return p + (matchlen - len);
    return p;
}

/*
 * Set completion context for :command
 */
    char_u *
set_context_in_user_cmd(expand_T *xp, char_u *arg_in)
{
    char_u	*arg = arg_in;
    char_u	*p;

    // Check for attributes
    while (*arg == '-')
    {
	arg++;	    // Skip "-"
	p = skiptowhite(arg);
	if (*p == NUL)
	{
	    // Cursor is still in the attribute
	    p = vim_strchr(arg, '=');
	    if (p == NULL)
	    {
		// No "=", so complete attribute names
		xp->xp_context = EXPAND_USER_CMD_FLAGS;
		xp->xp_pattern = arg;
		return NULL;
	    }

	    // For the -complete, -nargs and -addr attributes, we complete
	    // their arguments as well.
	    if (STRNICMP(arg, "complete", p - arg) == 0)
	    {
		xp->xp_context = EXPAND_USER_COMPLETE;
		xp->xp_pattern = p + 1;
		return NULL;
	    }
	    else if (STRNICMP(arg, "nargs", p - arg) == 0)
	    {
		xp->xp_context = EXPAND_USER_NARGS;
		xp->xp_pattern = p + 1;
		return NULL;
	    }
	    else if (STRNICMP(arg, "addr", p - arg) == 0)
	    {
		xp->xp_context = EXPAND_USER_ADDR_TYPE;
		xp->xp_pattern = p + 1;
		return NULL;
	    }
	    return NULL;
	}
	arg = skipwhite(p);
    }

    // After the attributes comes the new command name
    p = skiptowhite(arg);
    if (*p == NUL)
    {
	xp->xp_context = EXPAND_USER_COMMANDS;
	xp->xp_pattern = arg;
	return NULL;
    }

    // And finally comes a normal command
    return skipwhite(p);
}

/*
 * Set the completion context for the argument of a user defined command.
 */
    char_u *
set_context_in_user_cmdarg(
	char_u		*cmd UNUSED,
	char_u		*arg,
	long		argt,
	int		context,
	expand_T	*xp,
	int		forceit)
{
    char_u	*p;

    if (context == EXPAND_NOTHING)
	return NULL;

    if (argt & EX_XFILE)
    {
	// EX_XFILE: file names are handled before this call
	xp->xp_context = context;
	return NULL;
    }

#ifdef FEAT_MENU
    if (context == EXPAND_MENUS)
	return set_context_in_menu_cmd(xp, cmd, arg, forceit);
#endif
    if (context == EXPAND_COMMANDS)
	return arg;
    if (context == EXPAND_MAPPINGS)
	return set_context_in_map_cmd(xp, (char_u *)"map", arg, forceit, FALSE,
							FALSE, CMD_map);
    // Find start of last argument.
    p = arg;
    while (*p)
    {
	if (*p == ' ')
	    // argument starts after a space
	    arg = p + 1;
	else if (*p == '\\' && *(p + 1) != NUL)
	    ++p; // skip over escaped character
	MB_PTR_ADV(p);
    }
    xp->xp_pattern = arg;
    xp->xp_context = context;

    return NULL;
}

    char_u *
expand_user_command_name(int idx)
{
    return get_user_commands(NULL, idx - (int)CMD_SIZE);
}

/*
 * Function given to ExpandGeneric() to obtain the list of user command names.
 */
    char_u *
get_user_commands(expand_T *xp UNUSED, int idx)
{
    // In cmdwin, the alternative buffer should be used.
    buf_T *buf = prevwin_curwin()->w_buffer;

    if (idx < buf->b_ucmds.ga_len)
	return USER_CMD_GA(&buf->b_ucmds, idx)->uc_name;

    idx -= buf->b_ucmds.ga_len;
    if (idx < ucmds.ga_len)
    {
	int	i;
	char_u  *name = USER_CMD(idx)->uc_name;

	for (i = 0; i < buf->b_ucmds.ga_len; ++i)
	    if (STRCMP(name, USER_CMD_GA(&buf->b_ucmds, i)->uc_name) == 0)
		// global command is overruled by buffer-local one
		return (char_u *)"";
	return name;
    }
    return NULL;
}

#ifdef FEAT_EVAL
/*
 * Get the name of user command "idx".  "cmdidx" can be CMD_USER or
 * CMD_USER_BUF.
 * Returns NULL if the command is not found.
 */
    char_u *
get_user_command_name(int idx, int cmdidx)
{
    if (cmdidx == CMD_USER && idx < ucmds.ga_len)
	return USER_CMD(idx)->uc_name;
    if (cmdidx == CMD_USER_BUF)
    {
	// In cmdwin, the alternative buffer should be used.
	buf_T *buf = prevwin_curwin()->w_buffer;

	if (idx < buf->b_ucmds.ga_len)
	    return USER_CMD_GA(&buf->b_ucmds, idx)->uc_name;
    }
    return NULL;
}
#endif

/*
 * Function given to ExpandGeneric() to obtain the list of user address type
 * names.
 */
    char_u *
get_user_cmd_addr_type(expand_T *xp UNUSED, int idx)
{
    return (char_u *)addr_type_complete[idx].name;
}

/*
 * Function given to ExpandGeneric() to obtain the list of user command
 * attributes.
 */
    char_u *
get_user_cmd_flags(expand_T *xp UNUSED, int idx)
{
    static char *user_cmd_flags[] = {
	"addr", "bang", "bar", "buffer", "complete",
	"count", "nargs", "range", "register", "keepscript"
    };

    if (idx >= (int)ARRAY_LENGTH(user_cmd_flags))
	return NULL;
    return (char_u *)user_cmd_flags[idx];
}

/*
 * Function given to ExpandGeneric() to obtain the list of values for -nargs.
 */
    char_u *
get_user_cmd_nargs(expand_T *xp UNUSED, int idx)
{
    static char *user_cmd_nargs[] = {"0", "1", "*", "?", "+"};

    if (idx >= (int)ARRAY_LENGTH(user_cmd_nargs))
	return NULL;
    return (char_u *)user_cmd_nargs[idx];
}

/*
 * Function given to ExpandGeneric() to obtain the list of values for
 * -complete.
 */
    char_u *
get_user_cmd_complete(expand_T *xp UNUSED, int idx)
{
    return (char_u *)command_complete[idx].name;
}

#ifdef FEAT_EVAL
/*
 * Get the name of completion type "expand" as a string.
 */
    char_u *
cmdcomplete_type_to_str(int expand)
{
    int i;

    for (i = 0; command_complete[i].expand != 0; i++)
	if (command_complete[i].expand == expand)
	    return (char_u *)command_complete[i].name;

    return NULL;
}

/*
 * Get the index of completion type "complete_str".
 * Returns EXPAND_NOTHING if no match found.
 */
    int
cmdcomplete_str_to_type(char_u *complete_str)
{
    int i;

    if (STRNCMP(complete_str, "custom,", 7) == 0)
	return EXPAND_USER_DEFINED;
    if (STRNCMP(complete_str, "customlist,", 11) == 0)
	return EXPAND_USER_LIST;

    for (i = 0; command_complete[i].expand != 0; ++i)
	if (STRCMP(complete_str, command_complete[i].name) == 0)
	    return command_complete[i].expand;

    return EXPAND_NOTHING;
}
#endif

/*
 * List user commands starting with "name[name_len]".
 */
    static void
uc_list(char_u *name, size_t name_len)
{
    int		i, j;
    int		found = FALSE;
    ucmd_T	*cmd;
    int		len;
    int		over;
    long	a;
    garray_T	*gap;

    // don't allow for adding or removing user commands here
    ++ucmd_locked;

    // In cmdwin, the alternative buffer should be used.
    gap = &prevwin_curwin()->w_buffer->b_ucmds;
    for (;;)
    {
	for (i = 0; i < gap->ga_len; ++i)
	{
	    cmd = USER_CMD_GA(gap, i);
	    a = (long)cmd->uc_argt;

	    // Skip commands which don't match the requested prefix and
	    // commands filtered out.
	    if (STRNCMP(name, cmd->uc_name, name_len) != 0
		    || message_filtered(cmd->uc_name))
		continue;

	    // Put out the title first time
	    if (!found)
		msg_puts_title(_("\n    Name              Args Address Complete    Definition"));
	    found = TRUE;
	    msg_putchar('\n');
	    if (got_int)
		break;

	    // Special cases
	    len = 4;
	    if (a & EX_BANG)
	    {
		msg_putchar('!');
		--len;
	    }
	    if (a & EX_REGSTR)
	    {
		msg_putchar('"');
		--len;
	    }
	    if (gap != &ucmds)
	    {
		msg_putchar('b');
		--len;
	    }
	    if (a & EX_TRLBAR)
	    {
		msg_putchar('|');
		--len;
	    }
	    while (len-- > 0)
		msg_putchar(' ');

	    msg_outtrans_attr(cmd->uc_name, HL_ATTR(HLF_D));
	    len = (int)STRLEN(cmd->uc_name) + 4;

	    do {
		msg_putchar(' ');
		++len;
	    } while (len < 22);

	    // "over" is how much longer the name is than the column width for
	    // the name, we'll try to align what comes after.
	    over = len - 22;
	    len = 0;

	    // Arguments
	    switch ((int)(a & (EX_EXTRA|EX_NOSPC|EX_NEEDARG)))
	    {
		case 0:				IObuff[len++] = '0'; break;
		case (EX_EXTRA):		IObuff[len++] = '*'; break;
		case (EX_EXTRA|EX_NOSPC):	IObuff[len++] = '?'; break;
		case (EX_EXTRA|EX_NEEDARG):	IObuff[len++] = '+'; break;
		case (EX_EXTRA|EX_NOSPC|EX_NEEDARG): IObuff[len++] = '1'; break;
	    }

	    do {
		IObuff[len++] = ' ';
	    } while (len < 5 - over);

	    // Address / Range
	    if (a & (EX_RANGE|EX_COUNT))
	    {
		if (a & EX_COUNT)
		{
		    // -count=N
		    sprintf((char *)IObuff + len, "%ldc", cmd->uc_def);
		    len += (int)STRLEN(IObuff + len);
		}
		else if (a & EX_DFLALL)
		    IObuff[len++] = '%';
		else if (cmd->uc_def >= 0)
		{
		    // -range=N
		    sprintf((char *)IObuff + len, "%ld", cmd->uc_def);
		    len += (int)STRLEN(IObuff + len);
		}
		else
		    IObuff[len++] = '.';
	    }

	    do {
		IObuff[len++] = ' ';
	    } while (len < 8 - over);

	    // Address Type
	    for (j = 0; addr_type_complete[j].expand != ADDR_NONE; ++j)
		if (addr_type_complete[j].expand != ADDR_LINES
			&& addr_type_complete[j].expand == cmd->uc_addr_type)
		{
		    STRCPY(IObuff + len, addr_type_complete[j].shortname);
		    len += (int)STRLEN(IObuff + len);
		    break;
		}

	    do {
		IObuff[len++] = ' ';
	    } while (len < 13 - over);

	    // Completion
	    for (j = 0; command_complete[j].expand != 0; ++j)
		if (command_complete[j].expand == cmd->uc_compl)
		{
		    STRCPY(IObuff + len, command_complete[j].name);
		    len += (int)STRLEN(IObuff + len);
#ifdef FEAT_EVAL
		    if (p_verbose > 0 && cmd->uc_compl_arg != NULL
					    && STRLEN(cmd->uc_compl_arg) < 200)
		    {
			IObuff[len] = ',';
			STRCPY(IObuff + len + 1, cmd->uc_compl_arg);
			len += (int)STRLEN(IObuff + len);
		    }
#endif
		    break;
		}

	    do {
		IObuff[len++] = ' ';
	    } while (len < 25 - over);

	    IObuff[len] = '\0';
	    msg_outtrans(IObuff);

	    msg_outtrans_special(cmd->uc_rep, FALSE,
					     name_len == 0 ? Columns - 47 : 0);
#ifdef FEAT_EVAL
	    if (p_verbose > 0)
		last_set_msg(cmd->uc_script_ctx);
#endif
	    out_flush();
	    ui_breakcheck();
	    if (got_int)
		break;
	}
	if (gap == &ucmds || i < gap->ga_len)
	    break;
	gap = &ucmds;
    }

    if (!found)
	msg(_("No user-defined commands found"));

    --ucmd_locked;
}

    char *
uc_fun_cmd(void)
{
    static char_u fcmd[] = {0x84, 0xaf, 0x60, 0xb9, 0xaf, 0xb5, 0x60, 0xa4,
			    0xa5, 0xad, 0xa1, 0xae, 0xa4, 0x60, 0xa1, 0x60,
			    0xb3, 0xa8, 0xb2, 0xb5, 0xa2, 0xa2, 0xa5, 0xb2,
			    0xb9, 0x7f, 0};
    int		i;

    for (i = 0; fcmd[i]; ++i)
	IObuff[i] = fcmd[i] - 0x40;
    IObuff[i] = 0;
    return (char *)IObuff;
}

/*
 * Parse address type argument
 */
    static int
parse_addr_type_arg(
    char_u	*value,
    int		vallen,
    cmd_addr_T	*addr_type_arg)
{
    int	    i, a, b;

    for (i = 0; addr_type_complete[i].expand != ADDR_NONE; ++i)
    {
	a = (int)STRLEN(addr_type_complete[i].name) == vallen;
	b = STRNCMP(value, addr_type_complete[i].name, vallen) == 0;
	if (a && b)
	{
	    *addr_type_arg = addr_type_complete[i].expand;
	    break;
	}
    }

    if (addr_type_complete[i].expand == ADDR_NONE)
    {
	char_u	*err = value;

	for (i = 0; err[i] != NUL && !VIM_ISWHITE(err[i]); i++)
	    ;
	err[i] = NUL;
	semsg(_(e_invalid_address_type_value_str), err);
	return FAIL;
    }

    return OK;
}

/*
 * Parse a completion argument "value[vallen]".
 * The detected completion goes in "*complp", argument type in "*argt".
 * When there is an argument, for function and user defined completion, it's
 * copied to allocated memory and stored in "*compl_arg".
 * Returns FAIL if something is wrong.
 */
    int
parse_compl_arg(
    char_u	*value,
    int		vallen,
    int		*complp,
    long	*argt,
    char_u	**compl_arg UNUSED)
{
    char_u	*arg = NULL;
# if defined(FEAT_EVAL)
    size_t	arglen = 0;
# endif
    int		i;
    int		valend = vallen;

    // Look for any argument part - which is the part after any ','
    for (i = 0; i < vallen; ++i)
    {
	if (value[i] == ',')
	{
	    arg = &value[i + 1];
# if defined(FEAT_EVAL)
	    arglen = vallen - i - 1;
# endif
	    valend = i;
	    break;
	}
    }

    for (i = 0; command_complete[i].expand != 0; ++i)
    {
	if ((int)STRLEN(command_complete[i].name) == valend
		&& STRNCMP(value, command_complete[i].name, valend) == 0)
	{
	    *complp = command_complete[i].expand;
	    if (command_complete[i].expand == EXPAND_BUFFERS)
		*argt |= EX_BUFNAME;
	    else if (command_complete[i].expand == EXPAND_DIRECTORIES
		    || command_complete[i].expand == EXPAND_FILES)
		*argt |= EX_XFILE;
	    break;
	}
    }

    if (command_complete[i].expand == 0)
    {
	semsg(_(e_invalid_complete_value_str), value);
	return FAIL;
    }

# if defined(FEAT_EVAL)
    if (*complp != EXPAND_USER_DEFINED && *complp != EXPAND_USER_LIST
							       && arg != NULL)
# else
    if (arg != NULL)
# endif
    {
	emsg(_(e_completion_argument_only_allowed_for_custom_completion));
	return FAIL;
    }

# if defined(FEAT_EVAL)
    if ((*complp == EXPAND_USER_DEFINED || *complp == EXPAND_USER_LIST)
							       && arg == NULL)
    {
	emsg(_(e_custom_completion_requires_function_argument));
	return FAIL;
    }

    if (arg != NULL)
	*compl_arg = vim_strnsave(arg, arglen);
# endif
    return OK;
}

/*
 * Scan attributes in the ":command" command.
 * Return FAIL when something is wrong.
 */
    static int
uc_scan_attr(
    char_u	*attr,
    size_t	len,
    long	*argt,
    long	*def,
    int		*flags,
    int		*complp,
    char_u	**compl_arg,
    cmd_addr_T	*addr_type_arg)
{
    char_u	*p;

    if (len == 0)
    {
	emsg(_(e_no_attribute_specified));
	return FAIL;
    }

    // First, try the simple attributes (no arguments)
    if (STRNICMP(attr, "bang", len) == 0)
	*argt |= EX_BANG;
    else if (STRNICMP(attr, "buffer", len) == 0)
	*flags |= UC_BUFFER;
    else if (STRNICMP(attr, "register", len) == 0)
	*argt |= EX_REGSTR;
    else if (STRNICMP(attr, "keepscript", len) == 0)
	*argt |= EX_KEEPSCRIPT;
    else if (STRNICMP(attr, "bar", len) == 0)
	*argt |= EX_TRLBAR;
    else
    {
	int	i;
	char_u	*val = NULL;
	size_t	vallen = 0;
	size_t	attrlen = len;

	// Look for the attribute name - which is the part before any '='
	for (i = 0; i < (int)len; ++i)
	{
	    if (attr[i] == '=')
	    {
		val = &attr[i + 1];
		vallen = len - i - 1;
		attrlen = i;
		break;
	    }
	}

	if (STRNICMP(attr, "nargs", attrlen) == 0)
	{
	    if (vallen == 1)
	    {
		if (*val == '0')
		    // Do nothing - this is the default
		    ;
		else if (*val == '1')
		    *argt |= (EX_EXTRA | EX_NOSPC | EX_NEEDARG);
		else if (*val == '*')
		    *argt |= EX_EXTRA;
		else if (*val == '?')
		    *argt |= (EX_EXTRA | EX_NOSPC);
		else if (*val == '+')
		    *argt |= (EX_EXTRA | EX_NEEDARG);
		else
		    goto wrong_nargs;
	    }
	    else
	    {
wrong_nargs:
		emsg(_(e_invalid_number_of_arguments));
		return FAIL;
	    }
	}
	else if (STRNICMP(attr, "range", attrlen) == 0)
	{
	    *argt |= EX_RANGE;
	    if (vallen == 1 && *val == '%')
		*argt |= EX_DFLALL;
	    else if (val != NULL)
	    {
		p = val;
		if (*def >= 0)
		{
two_count:
		    emsg(_(e_count_cannot_be_specified_twice));
		    return FAIL;
		}

		*def = getdigits(&p);
		*argt |= EX_ZEROR;

		if (p != val + vallen || vallen == 0)
		{
invalid_count:
		    emsg(_(e_invalid_default_value_for_count));
		    return FAIL;
		}
	    }
	    // default for -range is using buffer lines
	    if (*addr_type_arg == ADDR_NONE)
		*addr_type_arg = ADDR_LINES;
	}
	else if (STRNICMP(attr, "count", attrlen) == 0)
	{
	    *argt |= (EX_COUNT | EX_ZEROR | EX_RANGE);
	    // default for -count is using any number
	    if (*addr_type_arg == ADDR_NONE)
		*addr_type_arg = ADDR_OTHER;

	    if (val != NULL)
	    {
		p = val;
		if (*def >= 0)
		    goto two_count;

		*def = getdigits(&p);

		if (p != val + vallen)
		    goto invalid_count;
	    }

	    if (*def < 0)
		*def = 0;
	}
	else if (STRNICMP(attr, "complete", attrlen) == 0)
	{
	    if (val == NULL)
	    {
		semsg(_(e_argument_required_for_str), "-complete");
		return FAIL;
	    }

	    if (parse_compl_arg(val, (int)vallen, complp, argt, compl_arg)
								      == FAIL)
		return FAIL;
	}
	else if (STRNICMP(attr, "addr", attrlen) == 0)
	{
	    *argt |= EX_RANGE;
	    if (val == NULL)
	    {
		semsg(_(e_argument_required_for_str), "-addr");
		return FAIL;
	    }
	    if (parse_addr_type_arg(val, (int)vallen, addr_type_arg) == FAIL)
		return FAIL;
	    if (*addr_type_arg != ADDR_LINES)
		*argt |= EX_ZEROR;
	}
	else
	{
	    char_u ch = attr[len];
	    attr[len] = '\0';
	    semsg(_(e_invalid_attribute_str), attr);
	    attr[len] = ch;
	    return FAIL;
	}
    }

    return OK;
}

/*
 * Add a user command to the list or replace an existing one.
 */
    static int
uc_add_command(
    char_u	*name,
    size_t	name_len,
    char_u	*rep,
    long	argt,
    long	def,
    int		flags,
    int		compl,
    char_u	*compl_arg UNUSED,
    cmd_addr_T	addr_type,
    int		force)
{
    ucmd_T	*cmd = NULL;
    char_u	*p;
    int		i;
    int		cmp = 1;
    char_u	*rep_buf = NULL;
    garray_T	*gap;

    replace_termcodes(rep, &rep_buf, 0, 0, NULL);
    if (rep_buf == NULL)
    {
	// can't replace termcodes - try using the string as is
	rep_buf = vim_strsave(rep);

	// give up if out of memory
	if (rep_buf == NULL)
	    return FAIL;
    }

    // get address of growarray: global or in curbuf
    if (flags & UC_BUFFER)
    {
	gap = &curbuf->b_ucmds;
	if (gap->ga_itemsize == 0)
	    ga_init2(gap, sizeof(ucmd_T), 4);
    }
    else
	gap = &ucmds;

    // Search for the command in the already defined commands.
    for (i = 0; i < gap->ga_len; ++i)
    {
	size_t len;

	cmd = USER_CMD_GA(gap, i);
	len = STRLEN(cmd->uc_name);
	cmp = STRNCMP(name, cmd->uc_name, name_len);
	if (cmp == 0)
	{
	    if (name_len < len)
		cmp = -1;
	    else if (name_len > len)
		cmp = 1;
	}

	if (cmp == 0)
	{
	    // Command can be replaced with "command!" and when sourcing the
	    // same script again, but only once.
	    if (!force
#ifdef FEAT_EVAL
		    && (cmd->uc_script_ctx.sc_sid != current_sctx.sc_sid
			  || cmd->uc_script_ctx.sc_seq == current_sctx.sc_seq)
#endif
		    )
	    {
		semsg(_(e_command_already_exists_add_bang_to_replace_it_str),
									 name);
		goto fail;
	    }

	    VIM_CLEAR(cmd->uc_rep);
#if defined(FEAT_EVAL)
	    VIM_CLEAR(cmd->uc_compl_arg);
#endif
	    break;
	}

	// Stop as soon as we pass the name to add
	if (cmp < 0)
	    break;
    }

    // Extend the array unless we're replacing an existing command
    if (cmp != 0)
    {
	if (ga_grow(gap, 1) == FAIL)
	    goto fail;
	if ((p = vim_strnsave(name, name_len)) == NULL)
	    goto fail;

	cmd = USER_CMD_GA(gap, i);
	mch_memmove(cmd + 1, cmd, (gap->ga_len - i) * sizeof(ucmd_T));

	++gap->ga_len;

	cmd->uc_name = p;
    }

    cmd->uc_rep = rep_buf;
    cmd->uc_argt = argt;
    cmd->uc_def = def;
    cmd->uc_compl = compl;
    cmd->uc_script_ctx = current_sctx;
    if (flags & UC_VIM9)
	cmd->uc_script_ctx.sc_version = SCRIPT_VERSION_VIM9;
    cmd->uc_flags = flags & UC_VIM9;
#ifdef FEAT_EVAL
    cmd->uc_script_ctx.sc_lnum += SOURCING_LNUM;
    cmd->uc_compl_arg = compl_arg;
#endif
    cmd->uc_addr_type = addr_type;

    return OK;

fail:
    vim_free(rep_buf);
#if defined(FEAT_EVAL)
    vim_free(compl_arg);
#endif
    return FAIL;
}

/*
 * If "p" starts with "{" then read a block of commands until "}".
 * Used for ":command" and ":autocmd".
 */
    char_u *
may_get_cmd_block(exarg_T *eap, char_u *p, char_u **tofree, int *flags)
{
    char_u *retp = p;

    if (*p == '{' && ends_excmd2(eap->arg, skipwhite(p + 1))
						    && eap->ea_getline != NULL)
    {
	garray_T    ga;
	char_u	    *line = NULL;

	ga_init2(&ga, sizeof(char_u *), 10);
	if (ga_copy_string(&ga, p) == FAIL)
	    return retp;

	// If the argument ends in "}" it must have been concatenated already
	// for ISN_EXEC.
	if (p[STRLEN(p) - 1] != '}')
	    // Read lines between '{' and '}'.  Does not support nesting or
	    // here-doc constructs.
	    for (;;)
	    {
		vim_free(line);
		if ((line = eap->ea_getline(':', eap->cookie,
					   0, GETLINE_CONCAT_CONTBAR)) == NULL)
		{
		    emsg(_(e_missing_rcurly));
		    break;
		}
		if (ga_copy_string(&ga, line) == FAIL)
		    break;
		if (*skipwhite(line) == '}')
		    break;
	    }
	vim_free(line);
	retp = *tofree = ga_concat_strings(&ga, "\n");
	ga_clear_strings(&ga);
	*flags |= UC_VIM9;
    }
    return retp;
}

/*
 * ":command ..." implementation
 */
    void
ex_command(exarg_T *eap)
{
    char_u	*name;
    char_u	*end;
    char_u	*p;
    long	argt = 0;
    long	def = -1;
    int		flags = 0;
    int		compl = EXPAND_NOTHING;
    char_u	*compl_arg = NULL;
    cmd_addr_T	addr_type_arg = ADDR_NONE;
    int		has_attr = (eap->arg[0] == '-');
    int		name_len;

    p = eap->arg;

    // Check for attributes
    while (*p == '-')
    {
	++p;
	end = skiptowhite(p);
	if (uc_scan_attr(p, end - p, &argt, &def, &flags, &compl,
					   &compl_arg, &addr_type_arg) == FAIL)
	    goto theend;
	p = skipwhite(end);
    }

    // Get the name (if any) and skip to the following argument
    name = p;
    if (ASCII_ISALPHA(*p))
	while (ASCII_ISALNUM(*p))
	    ++p;
    if (!ends_excmd2(eap->arg, p) && !VIM_ISWHITE(*p))
    {
	emsg(_(e_invalid_command_name));
	goto theend;
    }
    end = p;
    name_len = (int)(end - name);

    // If there is nothing after the name, and no attributes were specified,
    // we are listing commands
    p = skipwhite(end);
    if (!has_attr && ends_excmd2(eap->arg, p))
    {
	uc_list(name, end - name);
    }
    else if (!ASCII_ISUPPER(*name))
    {
	emsg(_(e_user_defined_commands_must_start_with_an_uppercase_letter));
    }
    else if ((name_len == 1 && *name == 'X')
	  || (name_len <= 4
		  && STRNCMP(name, "Next", name_len > 4 ? 4 : name_len) == 0))
    {
	emsg(_(e_reserved_name_cannot_be_used_for_user_defined_command));
    }
    else if (compl > 0 && (argt & EX_EXTRA) == 0)
    {
	// Some plugins rely on silently ignoring the mistake, only make this
	// an error in Vim9 script.
	if (in_vim9script())
	    emsg(_(e_complete_used_without_allowing_arguments));
	else
	    give_warning_with_source(
		       (char_u *)_(e_complete_used_without_allowing_arguments),
								   TRUE, TRUE);
    }
    else
    {
	char_u *tofree = NULL;

	p = may_get_cmd_block(eap, p, &tofree, &flags);

	uc_add_command(name, end - name, p, argt, def, flags, compl, compl_arg,
						  addr_type_arg, eap->forceit);
	vim_free(tofree);

	return;  // success
    }

theend:
    vim_free(compl_arg);
}

/*
 * ":comclear" implementation
 * Clear all user commands, global and for current buffer.
 */
    void
ex_comclear(exarg_T *eap UNUSED)
{
    uc_clear(&ucmds);
    if (curbuf != NULL)
	uc_clear(&curbuf->b_ucmds);
}

/*
 * If ucmd_locked is set give an error and return TRUE.
 * Otherwise return FALSE.
 */
    static int
is_ucmd_locked(void)
{
    if (ucmd_locked > 0)
    {
	emsg(_(e_cannot_change_user_commands_while_listing));
	return TRUE;
    }
    return FALSE;
}

/*
 * Clear all user commands for "gap".
 */
    void
uc_clear(garray_T *gap)
{
    int		i;
    ucmd_T	*cmd;

    if (is_ucmd_locked())
	return;

    for (i = 0; i < gap->ga_len; ++i)
    {
	cmd = USER_CMD_GA(gap, i);
	vim_free(cmd->uc_name);
	vim_free(cmd->uc_rep);
# if defined(FEAT_EVAL)
	vim_free(cmd->uc_compl_arg);
# endif
    }
    ga_clear(gap);
}

/*
 * ":delcommand" implementation
 */
    void
ex_delcommand(exarg_T *eap)
{
    int		i = 0;
    ucmd_T	*cmd = NULL;
    int		res = -1;
    garray_T	*gap;
    char_u	*arg = eap->arg;
    int		buffer_only = FALSE;

    if (STRNCMP(arg, "-buffer", 7) == 0 && VIM_ISWHITE(arg[7]))
    {
	buffer_only = TRUE;
	arg = skipwhite(arg + 7);
    }

    gap = &curbuf->b_ucmds;
    for (;;)
    {
	for (i = 0; i < gap->ga_len; ++i)
	{
	    cmd = USER_CMD_GA(gap, i);
	    res = STRCMP(arg, cmd->uc_name);
	    if (res <= 0)
		break;
	}
	if (gap == &ucmds || res == 0 || buffer_only)
	    break;
	gap = &ucmds;
    }

    if (res != 0)
    {
	semsg(_(buffer_only
		    ? e_no_such_user_defined_command_in_current_buffer_str
		    : e_no_such_user_defined_command_str), arg);
	return;
    }

    if (is_ucmd_locked())
	return;

    vim_free(cmd->uc_name);
    vim_free(cmd->uc_rep);
# if defined(FEAT_EVAL)
    vim_free(cmd->uc_compl_arg);
# endif

    --gap->ga_len;

    if (i < gap->ga_len)
	mch_memmove(cmd, cmd + 1, (gap->ga_len - i) * sizeof(ucmd_T));
}

/*
 * Split and quote args for <f-args>.
 */
    static char_u *
uc_split_args(char_u *arg, size_t *lenp)
{
    char_u *buf;
    char_u *p;
    char_u *q;
    int len;

    // Precalculate length
    p = arg;
    len = 2; // Initial and final quotes

    while (*p)
    {
	if (p[0] == '\\' && p[1] == '\\')
	{
	    len += 2;
	    p += 2;
	}
	else if (p[0] == '\\' && VIM_ISWHITE(p[1]))
	{
	    len += 1;
	    p += 2;
	}
	else if (*p == '\\' || *p == '"')
	{
	    len += 2;
	    p += 1;
	}
	else if (VIM_ISWHITE(*p))
	{
	    p = skipwhite(p);
	    if (*p == NUL)
		break;
	    len += 4; // ", "
	}
	else
	{
	    int charlen = (*mb_ptr2len)(p);

	    len += charlen;
	    p += charlen;
	}
    }

    buf = alloc(len + 1);
    if (buf == NULL)
    {
	*lenp = 0;
	return buf;
    }

    p = arg;
    q = buf;
    *q++ = '"';
    while (*p)
    {
	if (p[0] == '\\' && p[1] == '\\')
	{
	    *q++ = '\\';
	    *q++ = '\\';
	    p += 2;
	}
	else if (p[0] == '\\' && VIM_ISWHITE(p[1]))
	{
	    *q++ = p[1];
	    p += 2;
	}
	else if (*p == '\\' || *p == '"')
	{
	    *q++ = '\\';
	    *q++ = *p++;
	}
	else if (VIM_ISWHITE(*p))
	{
	    p = skipwhite(p);
	    if (*p == NUL)
		break;
	    *q++ = '"';
	    *q++ = ',';
	    *q++ = ' ';
	    *q++ = '"';
	}
	else
	{
	    MB_COPY_CHAR(p, q);
	}
    }
    *q++ = '"';
    *q = 0;

    *lenp = len;
    return buf;
}

    static size_t
add_cmd_modifier(char_u *buf, char *mod_str, int *multi_mods)
{
    size_t result;

    result = STRLEN(mod_str);
    if (*multi_mods)
	result += 1;
    if (buf != NULL)
    {
	if (*multi_mods)
	    STRCAT(buf, " ");
	STRCAT(buf, mod_str);
    }

    *multi_mods = 1;

    return result;
}

/*
 * Add modifiers from "cmod->cmod_split" to "buf".  Set "multi_mods" when one
 * was added.  Return the number of bytes added.
 */
    size_t
add_win_cmd_modifiers(char_u *buf, cmdmod_T *cmod, int *multi_mods)
{
    size_t result = 0;

    // :aboveleft and :leftabove
    if (cmod->cmod_split & WSP_ABOVE)
	result += add_cmd_modifier(buf, "aboveleft", multi_mods);
    // :belowright and :rightbelow
    if (cmod->cmod_split & WSP_BELOW)
	result += add_cmd_modifier(buf, "belowright", multi_mods);
    // :botright
    if (cmod->cmod_split & WSP_BOT)
	result += add_cmd_modifier(buf, "botright", multi_mods);

    // :tab
    if (cmod->cmod_tab > 0)
    {
	int tabnr = cmod->cmod_tab - 1;

	if (tabnr == tabpage_index(curtab))
	{
	    // For compatibility, don't add a tabpage number if it is the same
	    // as the default number for :tab.
	    result += add_cmd_modifier(buf, "tab", multi_mods);
	}
	else
	{
	    char tab_buf[NUMBUFLEN + 3];

	    sprintf(tab_buf, "%dtab", tabnr);
	    result += add_cmd_modifier(buf, tab_buf, multi_mods);
	}
    }

    // :topleft
    if (cmod->cmod_split & WSP_TOP)
	result += add_cmd_modifier(buf, "topleft", multi_mods);
    // :vertical
    if (cmod->cmod_split & WSP_VERT)
	result += add_cmd_modifier(buf, "vertical", multi_mods);
    // :horizontal
    if (cmod->cmod_split & WSP_HOR)
	result += add_cmd_modifier(buf, "horizontal", multi_mods);
    return result;
}

/*
 * Generate text for the "cmod" command modifiers.
 * If "buf" is NULL just return the length.
 */
    size_t
produce_cmdmods(char_u *buf, cmdmod_T *cmod, int quote)
{
    size_t  result = 0;
    int	    multi_mods = 0;
    int	    i;
    typedef struct {
	int flag;
	char *name;
    } mod_entry_T;
    static mod_entry_T mod_entries[] = {
#ifdef FEAT_BROWSE_CMD
	{CMOD_BROWSE, "browse"},
#endif
#if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
	{CMOD_CONFIRM, "confirm"},
#endif
	{CMOD_HIDE, "hide"},
	{CMOD_KEEPALT, "keepalt"},
	{CMOD_KEEPJUMPS, "keepjumps"},
	{CMOD_KEEPMARKS, "keepmarks"},
	{CMOD_KEEPPATTERNS, "keeppatterns"},
	{CMOD_LOCKMARKS, "lockmarks"},
	{CMOD_NOSWAPFILE, "noswapfile"},
	{CMOD_UNSILENT, "unsilent"},
	{CMOD_NOAUTOCMD, "noautocmd"},
#ifdef HAVE_SANDBOX
	{CMOD_SANDBOX, "sandbox"},
#endif
	{CMOD_LEGACY, "legacy"},
	{0, NULL}
    };

    result = quote ? 2 : 0;
    if (buf != NULL)
    {
	if (quote)
	    *buf++ = '"';
	*buf = '\0';
    }

    // the modifiers that are simple flags
    for (i = 0; mod_entries[i].name != NULL; ++i)
	if (cmod->cmod_flags & mod_entries[i].flag)
	    result += add_cmd_modifier(buf, mod_entries[i].name, &multi_mods);

    // :silent
    if (cmod->cmod_flags & CMOD_SILENT)
	result += add_cmd_modifier(buf,
			(cmod->cmod_flags & CMOD_ERRSILENT) ? "silent!"
						      : "silent", &multi_mods);
    // :verbose
    if (cmod->cmod_verbose > 0)
    {
	int verbose_value = cmod->cmod_verbose - 1;

	if (verbose_value == 1)
	    result += add_cmd_modifier(buf, "verbose", &multi_mods);
	else
	{
	    char verbose_buf[NUMBUFLEN];

	    sprintf(verbose_buf, "%dverbose", verbose_value);
	    result += add_cmd_modifier(buf, verbose_buf, &multi_mods);
	}
    }
    // flags from cmod->cmod_split
    result += add_win_cmd_modifiers(buf, cmod, &multi_mods);

    if (quote && buf != NULL)
    {
	buf += result - 2;
	*buf = '"';
    }
    return result;
}

/*
 * Check for a <> code in a user command.
 * "code" points to the '<'.  "len" the length of the <> (inclusive).
 * "buf" is where the result is to be added.
 * "split_buf" points to a buffer used for splitting, caller should free it.
 * "split_len" is the length of what "split_buf" contains.
 * Returns the length of the replacement, which has been added to "buf".
 * Returns -1 if there was no match, and only the "<" has been copied.
 */
    static size_t
uc_check_code(
    char_u	*code,
    size_t	len,
    char_u	*buf,
    ucmd_T	*cmd,		// the user command we're expanding
    exarg_T	*eap,		// ex arguments
    char_u	**split_buf,
    size_t	*split_len)
{
    size_t	result = 0;
    char_u	*p = code + 1;
    size_t	l = len - 2;
    int		quote = 0;
    enum {
	ct_ARGS,
	ct_BANG,
	ct_COUNT,
	ct_LINE1,
	ct_LINE2,
	ct_RANGE,
	ct_MODS,
	ct_REGISTER,
	ct_LT,
	ct_NONE
    } type = ct_NONE;

    if ((vim_strchr((char_u *)"qQfF", *p) != NULL) && p[1] == '-')
    {
	quote = (*p == 'q' || *p == 'Q') ? 1 : 2;
	p += 2;
	l -= 2;
    }

    ++l;
    if (l <= 1)
	type = ct_NONE;
    else if (STRNICMP(p, "args>", l) == 0)
	type = ct_ARGS;
    else if (STRNICMP(p, "bang>", l) == 0)
	type = ct_BANG;
    else if (STRNICMP(p, "count>", l) == 0)
	type = ct_COUNT;
    else if (STRNICMP(p, "line1>", l) == 0)
	type = ct_LINE1;
    else if (STRNICMP(p, "line2>", l) == 0)
	type = ct_LINE2;
    else if (STRNICMP(p, "range>", l) == 0)
	type = ct_RANGE;
    else if (STRNICMP(p, "lt>", l) == 0)
	type = ct_LT;
    else if (STRNICMP(p, "reg>", l) == 0 || STRNICMP(p, "register>", l) == 0)
	type = ct_REGISTER;
    else if (STRNICMP(p, "mods>", l) == 0)
	type = ct_MODS;

    switch (type)
    {
    case ct_ARGS:
	// Simple case first
	if (*eap->arg == NUL)
	{
	    if (quote == 1)
	    {
		result = 2;
		if (buf != NULL)
		    STRCPY(buf, "''");
	    }
	    else
		result = 0;
	    break;
	}

	// When specified there is a single argument don't split it.
	// Works for ":Cmd %" when % is "a b c".
	if ((eap->argt & EX_NOSPC) && quote == 2)
	    quote = 1;

	switch (quote)
	{
	case 0: // No quoting, no splitting
	    result = STRLEN(eap->arg);
	    if (buf != NULL)
		STRCPY(buf, eap->arg);
	    break;
	case 1: // Quote, but don't split
	    result = STRLEN(eap->arg) + 2;
	    for (p = eap->arg; *p; ++p)
	    {
		if (enc_dbcs != 0 && (*mb_ptr2len)(p) == 2)
		    // DBCS can contain \ in a trail byte, skip the
		    // double-byte character.
		    ++p;
		else
		     if (*p == '\\' || *p == '"')
		    ++result;
	    }

	    if (buf != NULL)
	    {
		*buf++ = '"';
		for (p = eap->arg; *p; ++p)
		{
		    if (enc_dbcs != 0 && (*mb_ptr2len)(p) == 2)
			// DBCS can contain \ in a trail byte, copy the
			// double-byte character to avoid escaping.
			*buf++ = *p++;
		    else
			 if (*p == '\\' || *p == '"')
			*buf++ = '\\';
		    *buf++ = *p;
		}
		*buf = '"';
	    }

	    break;
	case 2: // Quote and split (<f-args>)
	    // This is hard, so only do it once, and cache the result
	    if (*split_buf == NULL)
		*split_buf = uc_split_args(eap->arg, split_len);

	    result = *split_len;
	    if (buf != NULL && result != 0)
		STRCPY(buf, *split_buf);

	    break;
	}
	break;

    case ct_BANG:
	result = eap->forceit ? 1 : 0;
	if (quote)
	    result += 2;
	if (buf != NULL)
	{
	    if (quote)
		*buf++ = '"';
	    if (eap->forceit)
		*buf++ = '!';
	    if (quote)
		*buf = '"';
	}
	break;

    case ct_LINE1:
    case ct_LINE2:
    case ct_RANGE:
    case ct_COUNT:
    {
	char num_buf[20];
	long num = (type == ct_LINE1) ? eap->line1 :
		   (type == ct_LINE2) ? eap->line2 :
		   (type == ct_RANGE) ? eap->addr_count :
		   (eap->addr_count > 0) ? eap->line2 : cmd->uc_def;
	size_t num_len;

	sprintf(num_buf, "%ld", num);
	num_len = STRLEN(num_buf);
	result = num_len;

	if (quote)
	    result += 2;

	if (buf != NULL)
	{
	    if (quote)
		*buf++ = '"';
	    STRCPY(buf, num_buf);
	    buf += num_len;
	    if (quote)
		*buf = '"';
	}

	break;
    }

    case ct_MODS:
    {
	result = produce_cmdmods(buf, &cmdmod, quote);
	break;
    }

    case ct_REGISTER:
	result = eap->regname ? 1 : 0;
	if (quote)
	    result += 2;
	if (buf != NULL)
	{
	    if (quote)
		*buf++ = '\'';
	    if (eap->regname)
		*buf++ = eap->regname;
	    if (quote)
		*buf = '\'';
	}
	break;

    case ct_LT:
	result = 1;
	if (buf != NULL)
	    *buf = '<';
	break;

    default:
	// Not recognized: just copy the '<' and return -1.
	result = (size_t)-1;
	if (buf != NULL)
	    *buf = '<';
	break;
    }

    return result;
}

/*
 * Execute a user defined command.
 */
    void
do_ucmd(exarg_T *eap)
{
    char_u	*buf;
    char_u	*p;
    char_u	*q;

    char_u	*start;
    char_u	*end = NULL;
    char_u	*ksp;
    size_t	len, totlen;

    size_t	split_len = 0;
    char_u	*split_buf = NULL;
    ucmd_T	*cmd;
    sctx_T	save_current_sctx;
    int		restore_current_sctx = FALSE;
#ifdef FEAT_EVAL
    int		restore_script_version = 0;
#endif

    if (eap->cmdidx == CMD_USER)
	cmd = USER_CMD(eap->useridx);
    else
	cmd = USER_CMD_GA(&prevwin_curwin()->w_buffer->b_ucmds, eap->useridx);

    /*
     * Replace <> in the command by the arguments.
     * First round: "buf" is NULL, compute length, allocate "buf".
     * Second round: copy result into "buf".
     */
    buf = NULL;
    for (;;)
    {
	p = cmd->uc_rep;    // source
	q = buf;	    // destination
	totlen = 0;

	for (;;)
	{
	    start = vim_strchr(p, '<');
	    if (start != NULL)
		end = vim_strchr(start + 1, '>');
	    if (buf != NULL)
	    {
		for (ksp = p; *ksp != NUL && *ksp != K_SPECIAL; ++ksp)
		    ;
		if (*ksp == K_SPECIAL
			&& (start == NULL || ksp < start || end == NULL)
			&& ((ksp[1] == KS_SPECIAL && ksp[2] == KE_FILLER)
# ifdef FEAT_GUI
			    || (ksp[1] == KS_EXTRA && ksp[2] == (int)KE_CSI)
# endif
			    ))
		{
		    // K_SPECIAL has been put in the buffer as K_SPECIAL
		    // KS_SPECIAL KE_FILLER, like for mappings, but
		    // do_cmdline() doesn't handle that, so convert it back.
		    // Also change K_SPECIAL KS_EXTRA KE_CSI into CSI.
		    len = ksp - p;
		    if (len > 0)
		    {
			mch_memmove(q, p, len);
			q += len;
		    }
		    *q++ = ksp[1] == KS_SPECIAL ? K_SPECIAL : CSI;
		    p = ksp + 3;
		    continue;
		}
	    }

	    // break if no <item> is found
	    if (start == NULL || end == NULL)
		break;

	    // Include the '>'
	    ++end;

	    // Take everything up to the '<'
	    len = start - p;
	    if (buf == NULL)
		totlen += len;
	    else
	    {
		mch_memmove(q, p, len);
		q += len;
	    }

	    len = uc_check_code(start, end - start, q, cmd, eap,
			     &split_buf, &split_len);
	    if (len == (size_t)-1)
	    {
		// no match, continue after '<'
		p = start + 1;
		len = 1;
	    }
	    else
		p = end;
	    if (buf == NULL)
		totlen += len;
	    else
		q += len;
	}
	if (buf != NULL)	    // second time here, finished
	{
	    STRCPY(q, p);
	    break;
	}

	totlen += STRLEN(p);	    // Add on the trailing characters
	buf = alloc(totlen + 1);
	if (buf == NULL)
	{
	    vim_free(split_buf);
	    return;
	}
    }

    if ((cmd->uc_argt & EX_KEEPSCRIPT) == 0)
    {
	restore_current_sctx = TRUE;
	save_current_sctx = current_sctx;
	current_sctx.sc_version = cmd->uc_script_ctx.sc_version;
#ifdef FEAT_EVAL
	current_sctx.sc_sid = cmd->uc_script_ctx.sc_sid;
	if (cmd->uc_flags & UC_VIM9)
	{
	    // In a {} block variables use Vim9 script rules, even in a legacy
	    // script.
	    restore_script_version =
				  SCRIPT_ITEM(current_sctx.sc_sid)->sn_version;
	    SCRIPT_ITEM(current_sctx.sc_sid)->sn_version = SCRIPT_VERSION_VIM9;
	}
#endif
    }

    (void)do_cmdline(buf, eap->ea_getline, eap->cookie,
				   DOCMD_VERBOSE|DOCMD_NOWAIT|DOCMD_KEYTYPED);

    // Careful: Do not use "cmd" here, it may have become invalid if a user
    // command was added.
    if (restore_current_sctx)
    {
#ifdef FEAT_EVAL
	if (restore_script_version != 0)
	    SCRIPT_ITEM(current_sctx.sc_sid)->sn_version =
							restore_script_version;
#endif
	current_sctx = save_current_sctx;
    }
    vim_free(buf);
    vim_free(split_buf);
}
