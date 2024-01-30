/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * register.c: functions for managing registers
 */

#include "vim.h"

/*
 * Registers:
 *	0 = unnamed register, for normal yanks and puts
 *   1..9 = registers '1' to '9', for deletes
 * 10..35 = registers 'a' to 'z' ('A' to 'Z' for appending)
 *     36 = delete register '-'
 *     37 = Selection register '*'. Only if FEAT_CLIPBOARD defined
 *     38 = Clipboard register '+'. Only if FEAT_CLIPBOARD and FEAT_X11 defined
 */
static yankreg_T	y_regs[NUM_REGISTERS];

static yankreg_T	*y_current;	    // ptr to current yankreg
static int		y_append;	    // TRUE when appending
static yankreg_T	*y_previous = NULL; // ptr to last written yankreg

static int	stuff_yank(int, char_u *);
static void	put_reedit_in_typebuf(int silent);
static int	put_in_typebuf(char_u *s, int esc, int colon,
								 int silent);
static int	yank_copy_line(struct block_def *bd, long y_idx, int exclude_trailing_space);
#ifdef FEAT_CLIPBOARD
static void	copy_yank_reg(yankreg_T *reg);
#endif
static void	dis_msg(char_u *p, int skip_esc);

#if defined(FEAT_VIMINFO) || defined(PROTO)
    yankreg_T *
get_y_regs(void)
{
    return y_regs;
}
#endif

#if defined(FEAT_CLIPBOARD) || defined(PROTO)
    yankreg_T *
get_y_register(int reg)
{
    return &y_regs[reg];
}
#endif

    yankreg_T *
get_y_current(void)
{
    return y_current;
}

    yankreg_T *
get_y_previous(void)
{
    return y_previous;
}

    void
set_y_current(yankreg_T *yreg)
{
    y_current = yreg;
}

    void
set_y_previous(yankreg_T *yreg)
{
    y_previous = yreg;
}

    void
reset_y_append(void)
{
    y_append = FALSE;
}


#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Keep the last expression line here, for repeating.
 */
static char_u	*expr_line = NULL;
static exarg_T	*expr_eap = NULL;

/*
 * Get an expression for the "\"=expr1" or "CTRL-R =expr1"
 * Returns '=' when OK, NUL otherwise.
 */
    int
get_expr_register(void)
{
    char_u	*new_line;

    new_line = getcmdline('=', 0L, 0, 0);
    if (new_line == NULL)
	return NUL;
    if (*new_line == NUL)	// use previous line
	vim_free(new_line);
    else
	set_expr_line(new_line, NULL);
    return '=';
}

/*
 * Set the expression for the '=' register.
 * Argument must be an allocated string.
 * "eap" may be used if the next line needs to be checked when evaluating the
 * expression.
 */
    void
set_expr_line(char_u *new_line, exarg_T *eap)
{
    vim_free(expr_line);
    expr_line = new_line;
    expr_eap = eap;
}

/*
 * Get the result of the '=' register expression.
 * Returns a pointer to allocated memory, or NULL for failure.
 */
    char_u *
get_expr_line(void)
{
    char_u	*expr_copy;
    char_u	*rv;
    static int	nested = 0;

    if (expr_line == NULL)
	return NULL;

    // Make a copy of the expression, because evaluating it may cause it to be
    // changed.
    expr_copy = vim_strsave(expr_line);
    if (expr_copy == NULL)
	return NULL;

    // When we are invoked recursively limit the evaluation to 10 levels.
    // Then return the string as-is.
    if (nested >= 10)
	return expr_copy;

    ++nested;
    rv = eval_to_string_eap(expr_copy, TRUE, expr_eap, FALSE);
    --nested;
    vim_free(expr_copy);
    return rv;
}

/*
 * Get the '=' register expression itself, without evaluating it.
 */
    static char_u *
get_expr_line_src(void)
{
    if (expr_line == NULL)
	return NULL;
    return vim_strsave(expr_line);
}
#endif // FEAT_EVAL

/*
 * Check if 'regname' is a valid name of a yank register.
 * Note: There is no check for 0 (default register), caller should do this
 */
    int
valid_yank_reg(
    int	    regname,
    int	    writing)	    // if TRUE check for writable registers
{
    if (       (regname > 0 && ASCII_ISALNUM(regname))
	    || (!writing && vim_strchr((char_u *)
#ifdef FEAT_EVAL
				    "/.%:="
#else
				    "/.%:"
#endif
					, regname) != NULL)
	    || regname == '#'
	    || regname == '"'
	    || regname == '-'
	    || regname == '_'
#ifdef FEAT_CLIPBOARD
	    || regname == '*'
	    || regname == '+'
#endif
#ifdef FEAT_DND
	    || (!writing && regname == '~')
#endif
							)
	return TRUE;
    return FALSE;
}

/*
 * Set y_current and y_append, according to the value of "regname".
 * Cannot handle the '_' register.
 * Must only be called with a valid register name!
 *
 * If regname is 0 and writing, use register 0
 * If regname is 0 and reading, use previous register
 *
 * Return TRUE when the register should be inserted literally (selection or
 * clipboard).
 */
    int
get_yank_register(int regname, int writing)
{
    int	    i;
    int	    ret = FALSE;

    y_append = FALSE;
    if ((regname == 0 || regname == '"') && !writing && y_previous != NULL)
    {
	y_current = y_previous;
	return ret;
    }
    i = regname;
    if (VIM_ISDIGIT(i))
	i -= '0';
    else if (ASCII_ISLOWER(i))
	i = CharOrdLow(i) + 10;
    else if (ASCII_ISUPPER(i))
    {
	i = CharOrdUp(i) + 10;
	y_append = TRUE;
    }
    else if (regname == '-')
	i = DELETION_REGISTER;
#ifdef FEAT_CLIPBOARD
    // When selection is not available, use register 0 instead of '*'
    else if (clip_star.available && regname == '*')
    {
	i = STAR_REGISTER;
	ret = TRUE;
    }
    // When clipboard is not available, use register 0 instead of '+'
    else if (clip_plus.available && regname == '+')
    {
	i = PLUS_REGISTER;
	ret = TRUE;
    }
#endif
#ifdef FEAT_DND
    else if (!writing && regname == '~')
	i = TILDE_REGISTER;
#endif
    else		// not 0-9, a-z, A-Z or '-': use register 0
	i = 0;
    y_current = &(y_regs[i]);
    if (writing)	// remember the register we write into for do_put()
	y_previous = y_current;
    return ret;
}

/*
 * Obtain the contents of a "normal" register. The register is made empty.
 * The returned pointer has allocated memory, use put_register() later.
 */
    void *
get_register(
    int		name,
    int		copy)	// make a copy, if FALSE make register empty.
{
    yankreg_T	*reg;
    int		i;

#ifdef FEAT_CLIPBOARD
    // When Visual area changed, may have to update selection.  Obtain the
    // selection too.
    if (name == '*' && clip_star.available)
    {
	if (clip_isautosel_star())
	    clip_update_selection(&clip_star);
	may_get_selection(name);
    }
    if (name == '+' && clip_plus.available)
    {
	if (clip_isautosel_plus())
	    clip_update_selection(&clip_plus);
	may_get_selection(name);
    }
#endif

    get_yank_register(name, 0);
    reg = ALLOC_ONE(yankreg_T);
    if (reg == NULL)
	return (void *)NULL;

    *reg = *y_current;
    if (copy)
    {
	// If we run out of memory some or all of the lines are empty.
	if (reg->y_size == 0 || y_current->y_array == NULL)
	    reg->y_array = NULL;
	else
	    reg->y_array = ALLOC_MULT(char_u *, reg->y_size);
	if (reg->y_array != NULL)
	{
	    for (i = 0; i < reg->y_size; ++i)
		reg->y_array[i] = vim_strsave(y_current->y_array[i]);
	}
    }
    else
	y_current->y_array = NULL;
    return (void *)reg;
}

/*
 * Put "reg" into register "name".  Free any previous contents and "reg".
 */
    void
put_register(int name, void *reg)
{
    get_yank_register(name, 0);
    free_yank_all();
    *y_current = *(yankreg_T *)reg;
    vim_free(reg);

#ifdef FEAT_CLIPBOARD
    // Send text written to clipboard register to the clipboard.
    may_set_selection();
#endif
}

#if defined(FEAT_CLIPBOARD) || defined(PROTO)
    void
free_register(void *reg)
{
    yankreg_T tmp;

    tmp = *y_current;
    *y_current = *(yankreg_T *)reg;
    free_yank_all();
    vim_free(reg);
    *y_current = tmp;
}
#endif

/*
 * return TRUE if the current yank register has type MLINE
 */
    int
yank_register_mline(int regname)
{
    if (regname != 0 && !valid_yank_reg(regname, FALSE))
	return FALSE;
    if (regname == '_')		// black hole is always empty
	return FALSE;
    get_yank_register(regname, FALSE);
    return (y_current->y_type == MLINE);
}

/*
 * Start or stop recording into a yank register.
 *
 * Return FAIL for failure, OK otherwise.
 */
    int
do_record(int c)
{
    char_u	    *p;
    static int	    regname;
    yankreg_T	    *old_y_previous, *old_y_current;
    int		    retval;

    if (reg_recording == 0)	    // start recording
    {
	// registers 0-9, a-z and " are allowed
	if (c < 0 || (!ASCII_ISALNUM(c) && c != '"'))
	    retval = FAIL;
	else
	{
	    reg_recording = c;
	    showmode();
	    regname = c;
	    retval = OK;
	}
    }
    else			    // stop recording
    {
	// Get the recorded key hits.  K_SPECIAL and CSI will be escaped, this
	// needs to be removed again to put it in a register.  exec_reg then
	// adds the escaping back later.
	reg_recording = 0;
	msg("");
	p = get_recorded();
	if (p == NULL)
	    retval = FAIL;
	else
	{
	    // Remove escaping for CSI and K_SPECIAL in multi-byte chars.
	    vim_unescape_csi(p);

	    // We don't want to change the default register here, so save and
	    // restore the current register name.
	    old_y_previous = y_previous;
	    old_y_current = y_current;

	    retval = stuff_yank(regname, p);

	    y_previous = old_y_previous;
	    y_current = old_y_current;
	}
    }
    return retval;
}

/*
 * Stuff string "p" into yank register "regname" as a single line (append if
 * uppercase).	"p" must have been alloced.
 *
 * return FAIL for failure, OK otherwise
 */
    static int
stuff_yank(int regname, char_u *p)
{
    char_u	*lp;
    char_u	**pp;

    // check for read-only register
    if (regname != 0 && !valid_yank_reg(regname, TRUE))
    {
	vim_free(p);
	return FAIL;
    }
    if (regname == '_')		    // black hole: don't do anything
    {
	vim_free(p);
	return OK;
    }
    get_yank_register(regname, TRUE);
    if (y_append && y_current->y_array != NULL)
    {
	pp = &(y_current->y_array[y_current->y_size - 1]);
	lp = alloc(STRLEN(*pp) + STRLEN(p) + 1);
	if (lp == NULL)
	{
	    vim_free(p);
	    return FAIL;
	}
	STRCPY(lp, *pp);
	STRCAT(lp, p);
	vim_free(p);
	vim_free(*pp);
	*pp = lp;
    }
    else
    {
	free_yank_all();
	if ((y_current->y_array = ALLOC_ONE(char_u *)) == NULL)
	{
	    vim_free(p);
	    return FAIL;
	}
	y_current->y_array[0] = p;
	y_current->y_size = 1;
	y_current->y_type = MCHAR;  // used to be MLINE, why?
#ifdef FEAT_VIMINFO
	y_current->y_time_set = vim_time();
#endif
    }
    return OK;
}

/*
 * Last executed register (@ command)
 */
static int execreg_lastc = NUL;

#if defined(FEAT_VIMINFO) || defined(PROTO)
    int
get_execreg_lastc(void)
{
    return execreg_lastc;
}

    void
set_execreg_lastc(int lastc)
{
    execreg_lastc = lastc;
}
#endif

/*
 * When executing a register as a series of ex-commands, if the
 * line-continuation character is used for a line, then join it with one or
 * more previous lines. Note that lines are processed backwards starting from
 * the last line in the register.
 *
 * Arguments:
 *   lines - list of lines in the register
 *   idx - index of the line starting with \ or "\. Join this line with all the
 *	   immediate predecessor lines that start with a \ and the first line
 *	   that doesn't start with a \. Lines that start with a comment "\
 *	   character are ignored.
 *
 * Returns the concatenated line. The index of the line that should be
 * processed next is returned in idx.
 */
    static char_u *
execreg_line_continuation(char_u **lines, long *idx)
{
    garray_T	ga;
    long	i = *idx;
    char_u	*p;
    int		cmd_start;
    int		cmd_end = i;
    int		j;
    char_u	*str;

    ga_init2(&ga, sizeof(char_u), 400);

    // search backwards to find the first line of this command.
    // Any line not starting with \ or "\ is the start of the
    // command.
    while (--i > 0)
    {
	p = skipwhite(lines[i]);
	if (*p != '\\' && (p[0] != '"' || p[1] != '\\' || p[2] != ' '))
	    break;
    }
    cmd_start = i;

    // join all the lines
    ga_concat(&ga, lines[cmd_start]);
    for (j = cmd_start + 1; j <= cmd_end; j++)
    {
	p = skipwhite(lines[j]);
	if (*p == '\\')
	{
	    // Adjust the growsize to the current length to
	    // speed up concatenating many lines.
	    if (ga.ga_len > 400)
	    {
		if (ga.ga_len > 8000)
		    ga.ga_growsize = 8000;
		else
		    ga.ga_growsize = ga.ga_len;
	    }
	    ga_concat(&ga, p + 1);
	}
    }
    ga_append(&ga, NUL);
    str = vim_strsave(ga.ga_data);
    ga_clear(&ga);

    *idx = i;
    return str;
}

/*
 * Execute a yank register: copy it into the stuff buffer.
 *
 * Return FAIL for failure, OK otherwise.
 */
    int
do_execreg(
    int	    regname,
    int	    colon,		// insert ':' before each line
    int	    addcr,		// always add '\n' to end of line
    int	    silent)		// set "silent" flag in typeahead buffer
{
    long	i;
    char_u	*p;
    int		retval = OK;
    int		remap;

    // repeat previous one
    if (regname == '@')
    {
	if (execreg_lastc == NUL)
	{
	    emsg(_(e_no_previously_used_register));
	    return FAIL;
	}
	regname = execreg_lastc;
    }
    // check for valid regname
    if (regname == '%' || regname == '#' || !valid_yank_reg(regname, FALSE))
    {
	emsg_invreg(regname);
	return FAIL;
    }
    execreg_lastc = regname;

#ifdef FEAT_CLIPBOARD
    regname = may_get_selection(regname);
#endif

    // black hole: don't stuff anything
    if (regname == '_')
	return OK;

    // use last command line
    if (regname == ':')
    {
	if (last_cmdline == NULL)
	{
	    emsg(_(e_no_previous_command_line));
	    return FAIL;
	}
	// don't keep the cmdline containing @:
	VIM_CLEAR(new_last_cmdline);
	// Escape all control characters with a CTRL-V
	p = vim_strsave_escaped_ext(last_cmdline,
		    (char_u *)"\001\002\003\004\005\006\007"
			  "\010\011\012\013\014\015\016\017"
			  "\020\021\022\023\024\025\026\027"
			  "\030\031\032\033\034\035\036\037",
		    Ctrl_V, FALSE);
	if (p != NULL)
	{
	    // When in Visual mode "'<,'>" will be prepended to the command.
	    // Remove it when it's already there.
	    if (VIsual_active && STRNCMP(p, "'<,'>", 5) == 0)
		retval = put_in_typebuf(p + 5, TRUE, TRUE, silent);
	    else
		retval = put_in_typebuf(p, TRUE, TRUE, silent);
	}
	vim_free(p);
    }
#ifdef FEAT_EVAL
    else if (regname == '=')
    {
	p = get_expr_line();
	if (p == NULL)
	    return FAIL;
	retval = put_in_typebuf(p, TRUE, colon, silent);
	vim_free(p);
    }
#endif
    else if (regname == '.')		// use last inserted text
    {
	p = get_last_insert_save();
	if (p == NULL)
	{
	    emsg(_(e_no_inserted_text_yet));
	    return FAIL;
	}
	retval = put_in_typebuf(p, FALSE, colon, silent);
	vim_free(p);
    }
    else
    {
	get_yank_register(regname, FALSE);
	if (y_current->y_array == NULL)
	    return FAIL;

	// Disallow remapping for ":@r".
	remap = colon ? REMAP_NONE : REMAP_YES;

	// Insert lines into typeahead buffer, from last one to first one.
	put_reedit_in_typebuf(silent);
	for (i = y_current->y_size; --i >= 0; )
	{
	    char_u *escaped;
	    char_u *str;
	    int	    free_str = FALSE;

	    // insert NL between lines and after last line if type is MLINE
	    if (y_current->y_type == MLINE || i < y_current->y_size - 1
								     || addcr)
	    {
		if (ins_typebuf((char_u *)"\n", remap, 0, TRUE, silent) == FAIL)
		    return FAIL;
	    }

	    // Handle line-continuation for :@<register>
	    str = y_current->y_array[i];
	    if (colon && i > 0)
	    {
		p = skipwhite(str);
		if (*p == '\\' || (p[0] == '"' && p[1] == '\\' && p[2] == ' '))
		{
		    str = execreg_line_continuation(y_current->y_array, &i);
		    if (str == NULL)
			return FAIL;
		    free_str = TRUE;
		}
	    }
	    escaped = vim_strsave_escape_csi(str);
	    if (free_str)
		vim_free(str);
	    if (escaped == NULL)
		return FAIL;
	    retval = ins_typebuf(escaped, remap, 0, TRUE, silent);
	    vim_free(escaped);
	    if (retval == FAIL)
		return FAIL;
	    if (colon && ins_typebuf((char_u *)":", remap, 0, TRUE, silent)
								      == FAIL)
		return FAIL;
	}
	reg_executing = regname == 0 ? '"' : regname; // disable "q" command
	pending_end_reg_executing = FALSE;
    }
    return retval;
}

/*
 * If "restart_edit" is not zero, put it in the typeahead buffer, so that it's
 * used only after other typeahead has been processed.
 */
    static void
put_reedit_in_typebuf(int silent)
{
    char_u	buf[3];

    if (restart_edit == NUL)
	return;

    if (restart_edit == 'V')
    {
	buf[0] = 'g';
	buf[1] = 'R';
	buf[2] = NUL;
    }
    else
    {
	buf[0] = restart_edit == 'I' ? 'i' : restart_edit;
	buf[1] = NUL;
    }
    if (ins_typebuf(buf, REMAP_NONE, 0, TRUE, silent) == OK)
	restart_edit = NUL;
}

/*
 * Insert register contents "s" into the typeahead buffer, so that it will be
 * executed again.
 * When "esc" is TRUE it is to be taken literally: Escape CSI characters and
 * no remapping.
 */
    static int
put_in_typebuf(
    char_u	*s,
    int		esc,
    int		colon,	    // add ':' before the line
    int		silent)
{
    int		retval = OK;

    put_reedit_in_typebuf(silent);
    if (colon)
	retval = ins_typebuf((char_u *)"\n", REMAP_NONE, 0, TRUE, silent);
    if (retval == OK)
    {
	char_u	*p;

	if (esc)
	    p = vim_strsave_escape_csi(s);
	else
	    p = s;
	if (p == NULL)
	    retval = FAIL;
	else
	    retval = ins_typebuf(p, esc ? REMAP_NONE : REMAP_YES,
							     0, TRUE, silent);
	if (esc)
	    vim_free(p);
    }
    if (colon && retval == OK)
	retval = ins_typebuf((char_u *)":", REMAP_NONE, 0, TRUE, silent);
    return retval;
}

/*
 * Insert a yank register: copy it into the Read buffer.
 * Used by CTRL-R command and middle mouse button in insert mode.
 *
 * return FAIL for failure, OK otherwise
 */
    int
insert_reg(
    int		regname,
    int		literally_arg)	// insert literally, not as if typed
{
    long	i;
    int		retval = OK;
    char_u	*arg;
    int		allocated;
    int		literally = literally_arg;

    // It is possible to get into an endless loop by having CTRL-R a in
    // register a and then, in insert mode, doing CTRL-R a.
    // If you hit CTRL-C, the loop will be broken here.
    ui_breakcheck();
    if (got_int)
	return FAIL;

    // check for valid regname
    if (regname != NUL && !valid_yank_reg(regname, FALSE))
	return FAIL;

#ifdef FEAT_CLIPBOARD
    regname = may_get_selection(regname);
#endif

    if (regname == '.')			// insert last inserted text
	retval = stuff_inserted(NUL, 1L, TRUE);
    else if (get_spec_reg(regname, &arg, &allocated, TRUE))
    {
	if (arg == NULL)
	    return FAIL;
	stuffescaped(arg, literally);
	if (allocated)
	    vim_free(arg);
    }
    else				// name or number register
    {
	if (get_yank_register(regname, FALSE))
	    literally = TRUE;
	if (y_current->y_array == NULL)
	    retval = FAIL;
	else
	{
	    for (i = 0; i < y_current->y_size; ++i)
	    {
		if (regname == '-')
		{
		    int dir = BACKWARD;
		    if ((State & REPLACE_FLAG) != 0)
		    {
			pos_T curpos;
			if (u_save_cursor() == FAIL)
			    return FAIL;
			del_chars((long)mb_charlen(y_current->y_array[0]), TRUE);
			curpos = curwin->w_cursor;
			if (oneright() == FAIL)
			    // hit end of line, need to put forward (after the current position)
			    dir = FORWARD;
			curwin->w_cursor = curpos;
		    }

		    AppendCharToRedobuff(Ctrl_R);
		    AppendCharToRedobuff(regname);
		    do_put(regname, NULL, dir, 1L, PUT_CURSEND);
		}
		else
		    stuffescaped(y_current->y_array[i], literally);
		// Insert a newline between lines and after last line if
		// y_type is MLINE.
		if (y_current->y_type == MLINE || i < y_current->y_size - 1)
		    stuffcharReadbuff('\n');
	    }
	}
    }

    return retval;
}

/*
 * If "regname" is a special register, return TRUE and store a pointer to its
 * value in "argp".
 */
    int
get_spec_reg(
    int		regname,
    char_u	**argp,
    int		*allocated,	// return: TRUE when value was allocated
    int		errmsg)		// give error message when failing
{
    int		cnt;

    *argp = NULL;
    *allocated = FALSE;
    switch (regname)
    {
	case '%':		// file name
	    if (errmsg)
		check_fname();	// will give emsg if not set
	    *argp = curbuf->b_fname;
	    return TRUE;

	case '#':		// alternate file name
	    *argp = getaltfname(errmsg);	// may give emsg if not set
	    return TRUE;

#ifdef FEAT_EVAL
	case '=':		// result of expression
	    *argp = get_expr_line();
	    *allocated = TRUE;
	    return TRUE;
#endif

	case ':':		// last command line
	    if (last_cmdline == NULL && errmsg)
		emsg(_(e_no_previous_command_line));
	    *argp = last_cmdline;
	    return TRUE;

	case '/':		// last search-pattern
	    if (last_search_pat() == NULL && errmsg)
		emsg(_(e_no_previous_regular_expression));
	    *argp = last_search_pat();
	    return TRUE;

	case '.':		// last inserted text
	    *argp = get_last_insert_save();
	    *allocated = TRUE;
	    if (*argp == NULL && errmsg)
		emsg(_(e_no_inserted_text_yet));
	    return TRUE;

	case Ctrl_F:		// Filename under cursor
	case Ctrl_P:		// Path under cursor, expand via "path"
	    if (!errmsg)
		return FALSE;
	    *argp = file_name_at_cursor(FNAME_MESS | FNAME_HYP
			    | (regname == Ctrl_P ? FNAME_EXP : 0), 1L, NULL);
	    *allocated = TRUE;
	    return TRUE;

	case Ctrl_W:		// word under cursor
	case Ctrl_A:		// WORD (mnemonic All) under cursor
	    if (!errmsg)
		return FALSE;
	    cnt = find_ident_under_cursor(argp, regname == Ctrl_W
				   ?  (FIND_IDENT|FIND_STRING) : FIND_STRING);
	    *argp = cnt ? vim_strnsave(*argp, cnt) : NULL;
	    *allocated = TRUE;
	    return TRUE;

	case Ctrl_L:		// Line under cursor
	    if (!errmsg)
		return FALSE;

	    *argp = ml_get_buf(curwin->w_buffer,
			curwin->w_cursor.lnum, FALSE);
	    return TRUE;

	case '_':		// black hole: always empty
	    *argp = (char_u *)"";
	    return TRUE;
    }

    return FALSE;
}

/*
 * Paste a yank register into the command line.
 * Only for non-special registers.
 * Used by CTRL-R command in command-line mode
 * insert_reg() can't be used here, because special characters from the
 * register contents will be interpreted as commands.
 *
 * return FAIL for failure, OK otherwise
 */
    int
cmdline_paste_reg(
    int regname,
    int literally_arg,	// Insert text literally instead of "as typed"
    int remcr)		// don't add CR characters
{
    long	i;
    int		literally = literally_arg;

    if (get_yank_register(regname, FALSE))
	literally = TRUE;
    if (y_current->y_array == NULL)
	return FAIL;

    for (i = 0; i < y_current->y_size; ++i)
    {
	cmdline_paste_str(y_current->y_array[i], literally);

	// Insert ^M between lines and after last line if type is MLINE.
	// Don't do this when "remcr" is TRUE.
	if ((y_current->y_type == MLINE || i < y_current->y_size - 1) && !remcr)
	    cmdline_paste_str((char_u *)"\r", literally);

	// Check for CTRL-C, in case someone tries to paste a few thousand
	// lines and gets bored.
	ui_breakcheck();
	if (got_int)
	    return FAIL;
    }
    return OK;
}

/*
 * Shift the delete registers: "9 is cleared, "8 becomes "9, etc.
 */
    void
shift_delete_registers(void)
{
    int		n;

    y_current = &y_regs[9];
    free_yank_all();			// free register nine
    for (n = 9; n > 1; --n)
	y_regs[n] = y_regs[n - 1];
    y_current = &y_regs[1];
    if (!y_append)
	y_previous = y_current;
    y_regs[1].y_array = NULL;		// set register one to empty
}

#if defined(FEAT_EVAL)
    void
yank_do_autocmd(oparg_T *oap, yankreg_T *reg)
{
    static int	    recursive = FALSE;
    dict_T	    *v_event;
    list_T	    *list;
    int		    n;
    char_u	    buf[NUMBUFLEN + 2];
    long	    reglen = 0;
    save_v_event_T  save_v_event;

    if (recursive)
	return;

    v_event = get_v_event(&save_v_event);

    list = list_alloc();
    if (list == NULL)
	return;

    // yanked text contents
    for (n = 0; n < reg->y_size; n++)
	list_append_string(list, reg->y_array[n], -1);
    list->lv_lock = VAR_FIXED;
    (void)dict_add_list(v_event, "regcontents", list);

    // register name or empty string for unnamed operation
    buf[0] = (char_u)oap->regname;
    buf[1] = NUL;
    (void)dict_add_string(v_event, "regname", buf);

    // motion type: inclusive or exclusive
    (void)dict_add_bool(v_event, "inclusive", oap->inclusive);

    // kind of operation (yank, delete, change)
    buf[0] = get_op_char(oap->op_type);
    buf[1] = get_extra_op_char(oap->op_type);
    buf[2] = NUL;
    (void)dict_add_string(v_event, "operator", buf);

    // register type
    buf[0] = NUL;
    buf[1] = NUL;
    switch (get_reg_type(oap->regname, &reglen))
    {
	case MLINE: buf[0] = 'V'; break;
	case MCHAR: buf[0] = 'v'; break;
	case MBLOCK:
		vim_snprintf((char *)buf, sizeof(buf), "%c%ld", Ctrl_V,
			     reglen + 1);
		break;
    }
    (void)dict_add_string(v_event, "regtype", buf);

    // selection type - visual or not
    (void)dict_add_bool(v_event, "visual", oap->is_VIsual);

    // Lock the dictionary and its keys
    dict_set_items_ro(v_event);

    recursive = TRUE;
    textlock++;
    apply_autocmds(EVENT_TEXTYANKPOST, NULL, NULL, FALSE, curbuf);
    textlock--;
    recursive = FALSE;

    // Empty the dictionary, v:event is still valid
    restore_v_event(v_event, &save_v_event);
}
#endif

/*
 * set all the yank registers to empty (called from main())
 */
    void
init_yank(void)
{
    int		i;

    for (i = 0; i < NUM_REGISTERS; ++i)
	y_regs[i].y_array = NULL;
}

#if defined(EXITFREE) || defined(PROTO)
    void
clear_registers(void)
{
    int		i;

    for (i = 0; i < NUM_REGISTERS; ++i)
    {
	y_current = &y_regs[i];
	if (y_current->y_array != NULL)
	    free_yank_all();
    }
}
#endif

/*
 * Free "n" lines from the current yank register.
 * Called for normal freeing and in case of error.
 */
    static void
free_yank(long n)
{
    if (y_current->y_array == NULL)
	return;

    long	    i;

    for (i = n; --i >= 0; )
	vim_free(y_current->y_array[i]);
    VIM_CLEAR(y_current->y_array);
}

    void
free_yank_all(void)
{
    free_yank(y_current->y_size);
}

/*
 * Yank the text between "oap->start" and "oap->end" into a yank register.
 * If we are to append (uppercase register), we first yank into a new yank
 * register and then concatenate the old and the new one (so we keep the old
 * one in case of out-of-memory).
 *
 * Return FAIL for failure, OK otherwise.
 */
    int
op_yank(oparg_T *oap, int deleting, int mess)
{
    long		y_idx;		// index in y_array[]
    yankreg_T		*curr;		// copy of y_current
    yankreg_T		newreg;		// new yank register when appending
    char_u		**new_ptr;
    linenr_T		lnum;		// current line number
    long		j;
    int			yanktype = oap->motion_type;
    long		yanklines = oap->line_count;
    linenr_T		yankendlnum = oap->end.lnum;
    char_u		*p;
    char_u		*pnew;
    struct block_def	bd;
#if defined(FEAT_CLIPBOARD) && defined(FEAT_X11)
    int			did_star = FALSE;
#endif

				    // check for read-only register
    if (oap->regname != 0 && !valid_yank_reg(oap->regname, TRUE))
    {
	beep_flush();
	return FAIL;
    }
    if (oap->regname == '_')	    // black hole: nothing to do
	return OK;

#ifdef FEAT_CLIPBOARD
    if (!clip_star.available && oap->regname == '*')
	oap->regname = 0;
    else if (!clip_plus.available && oap->regname == '+')
	oap->regname = 0;
#endif

    if (!deleting)		    // op_delete() already set y_current
	get_yank_register(oap->regname, TRUE);

    curr = y_current;
				    // append to existing contents
    if (y_append && y_current->y_array != NULL)
	y_current = &newreg;
    else
	free_yank_all();	    // free previously yanked lines

    // If the cursor was in column 1 before and after the movement, and the
    // operator is not inclusive, the yank is always linewise.
    if (       oap->motion_type == MCHAR
	    && oap->start.col == 0
	    && !oap->inclusive
	    && (!oap->is_VIsual || *p_sel == 'o')
	    && !oap->block_mode
	    && oap->end.col == 0
	    && yanklines > 1)
    {
	yanktype = MLINE;
	--yankendlnum;
	--yanklines;
    }

    y_current->y_size = yanklines;
    y_current->y_type = yanktype;   // set the yank register type
    y_current->y_width = 0;
    y_current->y_array = lalloc_clear(sizeof(char_u *) * yanklines, TRUE);
    if (y_current->y_array == NULL)
    {
	y_current = curr;
	return FAIL;
    }
#ifdef FEAT_VIMINFO
    y_current->y_time_set = vim_time();
#endif

    y_idx = 0;
    lnum = oap->start.lnum;

    if (oap->block_mode)
    {
	// Visual block mode
	y_current->y_type = MBLOCK;	    // set the yank register type
	y_current->y_width = oap->end_vcol - oap->start_vcol;

	if (curwin->w_curswant == MAXCOL && y_current->y_width > 0)
	    y_current->y_width--;
    }

    for ( ; lnum <= yankendlnum; lnum++, y_idx++)
    {
	switch (y_current->y_type)
	{
	    case MBLOCK:
		block_prep(oap, &bd, lnum, FALSE);
		if (yank_copy_line(&bd, y_idx, oap->excl_tr_ws) == FAIL)
		    goto fail;
		break;

	    case MLINE:
		if ((y_current->y_array[y_idx] =
					    vim_strsave(ml_get(lnum))) == NULL)
		    goto fail;
		break;

	    case MCHAR:
		{
		    colnr_T startcol = 0, endcol = MAXCOL;
		    int	    is_oneChar = FALSE;
		    colnr_T cs, ce;

		    p = ml_get(lnum);
		    bd.startspaces = 0;
		    bd.endspaces = 0;

		    if (lnum == oap->start.lnum)
		    {
			startcol = oap->start.col;
			if (virtual_op)
			{
			    getvcol(curwin, &oap->start, &cs, NULL, &ce);
			    if (ce != cs && oap->start.coladd > 0)
			    {
				// Part of a tab selected -- but don't
				// double-count it.
				bd.startspaces = (ce - cs + 1)
							  - oap->start.coladd;
				if (bd.startspaces < 0)
				    bd.startspaces = 0;
				startcol++;
			    }
			}
		    }

		    if (lnum == oap->end.lnum)
		    {
			endcol = oap->end.col;
			if (virtual_op)
			{
			    getvcol(curwin, &oap->end, &cs, NULL, &ce);
			    if (p[endcol] == NUL || (cs + oap->end.coladd < ce
					// Don't add space for double-wide
					// char; endcol will be on last byte
					// of multi-byte char.
					&& (*mb_head_off)(p, p + endcol) == 0))
			    {
				if (oap->start.lnum == oap->end.lnum
					    && oap->start.col == oap->end.col)
				{
				    // Special case: inside a single char
				    is_oneChar = TRUE;
				    bd.startspaces = oap->end.coladd
					 - oap->start.coladd + oap->inclusive;
				    endcol = startcol;
				}
				else
				{
				    bd.endspaces = oap->end.coladd
							     + oap->inclusive;
				    endcol -= oap->inclusive;
				}
			    }
			}
		    }
		    if (endcol == MAXCOL)
			endcol = (colnr_T)STRLEN(p);
		    if (startcol > endcol || is_oneChar)
			bd.textlen = 0;
		    else
			bd.textlen = endcol - startcol + oap->inclusive;
		    bd.textstart = p + startcol;
		    if (yank_copy_line(&bd, y_idx, FALSE) == FAIL)
			goto fail;
		    break;
		}
		// NOTREACHED
	}
    }

    if (curr != y_current)	// append the new block to the old block
    {
	new_ptr = ALLOC_MULT(char_u *, curr->y_size + y_current->y_size);
	if (new_ptr == NULL)
	    goto fail;
	for (j = 0; j < curr->y_size; ++j)
	    new_ptr[j] = curr->y_array[j];
	vim_free(curr->y_array);
	curr->y_array = new_ptr;
#ifdef FEAT_VIMINFO
	curr->y_time_set = vim_time();
#endif

	if (yanktype == MLINE)	// MLINE overrides MCHAR and MBLOCK
	    curr->y_type = MLINE;

	// Concatenate the last line of the old block with the first line of
	// the new block, unless being Vi compatible.
	if (curr->y_type == MCHAR && vim_strchr(p_cpo, CPO_REGAPPEND) == NULL)
	{
	    pnew = alloc(STRLEN(curr->y_array[curr->y_size - 1])
					  + STRLEN(y_current->y_array[0]) + 1);
	    if (pnew == NULL)
	    {
		y_idx = y_current->y_size - 1;
		goto fail;
	    }
	    STRCPY(pnew, curr->y_array[--j]);
	    STRCAT(pnew, y_current->y_array[0]);
	    vim_free(curr->y_array[j]);
	    vim_free(y_current->y_array[0]);
	    curr->y_array[j++] = pnew;
	    y_idx = 1;
	}
	else
	    y_idx = 0;
	while (y_idx < y_current->y_size)
	    curr->y_array[j++] = y_current->y_array[y_idx++];
	curr->y_size = j;
	vim_free(y_current->y_array);
	y_current = curr;
    }

    if (mess)			// Display message about yank?
    {
	if (yanktype == MCHAR
		&& !oap->block_mode
		&& yanklines == 1)
	    yanklines = 0;
	// Some versions of Vi use ">=" here, some don't...
	if (yanklines > p_report)
	{
	    char namebuf[100];

	    if (oap->regname == NUL)
		*namebuf = NUL;
	    else
		vim_snprintf(namebuf, sizeof(namebuf),
						_(" into \"%c"), oap->regname);

	    // redisplay now, so message is not deleted
	    update_topline_redraw();
	    if (oap->block_mode)
	    {
		smsg(NGETTEXT("block of %ld line yanked%s",
				     "block of %ld lines yanked%s", yanklines),
			yanklines, namebuf);
	    }
	    else
	    {
		smsg(NGETTEXT("%ld line yanked%s",
					      "%ld lines yanked%s", yanklines),
			yanklines, namebuf);
	    }
	}
    }

    if ((cmdmod.cmod_flags & CMOD_LOCKMARKS) == 0)
    {
	// Set "'[" and "']" marks.
	curbuf->b_op_start = oap->start;
	curbuf->b_op_end = oap->end;
	if (yanktype == MLINE && !oap->block_mode)
	{
	    curbuf->b_op_start.col = 0;
	    curbuf->b_op_end.col = MAXCOL;
	}
    }

#ifdef FEAT_CLIPBOARD
    // If we were yanking to the '*' register, send result to clipboard.
    // If no register was specified, and "unnamed" in 'clipboard', make a copy
    // to the '*' register.
    if (clip_star.available
	    && (curr == &(y_regs[STAR_REGISTER])
		|| (!deleting && oap->regname == 0
		   && ((clip_unnamed | clip_unnamed_saved) & CLIP_UNNAMED))))
    {
	if (curr != &(y_regs[STAR_REGISTER]))
	    // Copy the text from register 0 to the clipboard register.
	    copy_yank_reg(&(y_regs[STAR_REGISTER]));

	clip_own_selection(&clip_star);
	clip_gen_set_selection(&clip_star);
# ifdef FEAT_X11
	did_star = TRUE;
# endif
    }

# ifdef FEAT_X11
    // If we were yanking to the '+' register, send result to selection.
    // Also copy to the '*' register, in case auto-select is off.  But not when
    // 'clipboard' has "unnamedplus" and not "unnamed"; and not when
    // deleting and both "unnamedplus" and "unnamed".
    if (clip_plus.available
	    && (curr == &(y_regs[PLUS_REGISTER])
		|| (!deleting && oap->regname == 0
		  && ((clip_unnamed | clip_unnamed_saved) &
							  CLIP_UNNAMED_PLUS))))
    {
	if (curr != &(y_regs[PLUS_REGISTER]))
	    // Copy the text from register 0 to the clipboard register.
	    copy_yank_reg(&(y_regs[PLUS_REGISTER]));

	clip_own_selection(&clip_plus);
	clip_gen_set_selection(&clip_plus);
	if (!clip_isautosel_star()
		&& !clip_isautosel_plus()
		&& !((clip_unnamed | clip_unnamed_saved) == CLIP_UNNAMED_PLUS)
		&& !(deleting && (clip_unnamed | clip_unnamed_saved)
					 == (CLIP_UNNAMED | CLIP_UNNAMED_PLUS))
		&& !did_star
		&& curr == &(y_regs[PLUS_REGISTER]))
	{
	    copy_yank_reg(&(y_regs[STAR_REGISTER]));
	    clip_own_selection(&clip_star);
	    clip_gen_set_selection(&clip_star);
	}
    }
# endif
#endif

#if defined(FEAT_EVAL)
    if (!deleting && has_textyankpost())
	yank_do_autocmd(oap, y_current);
#endif

    return OK;

fail:		// free the allocated lines
    free_yank(y_idx + 1);
    y_current = curr;
    return FAIL;
}

/*
 * Copy a block range into a register.
 * If "exclude_trailing_space" is set, do not copy trailing whitespaces.
 */
    static int
yank_copy_line(struct block_def *bd, long y_idx, int exclude_trailing_space)
{
    char_u	*pnew;

    if (exclude_trailing_space)
	bd->endspaces = 0;
    if ((pnew = alloc(bd->startspaces + bd->endspaces + bd->textlen + 1))
								      == NULL)
	return FAIL;
    y_current->y_array[y_idx] = pnew;
    vim_memset(pnew, ' ', (size_t)bd->startspaces);
    pnew += bd->startspaces;
    mch_memmove(pnew, bd->textstart, (size_t)bd->textlen);
    pnew += bd->textlen;
    vim_memset(pnew, ' ', (size_t)bd->endspaces);
    pnew += bd->endspaces;
    if (exclude_trailing_space)
    {
	int s = bd->textlen + bd->endspaces;

	while (s > 0 && VIM_ISWHITE(*(bd->textstart + s - 1)))
	{
	    s = s - (*mb_head_off)(bd->textstart, bd->textstart + s - 1) - 1;
	    pnew--;
	}
    }
    *pnew = NUL;
    return OK;
}

#ifdef FEAT_CLIPBOARD
/*
 * Make a copy of the y_current register to register "reg".
 */
    static void
copy_yank_reg(yankreg_T *reg)
{
    yankreg_T	*curr = y_current;
    long	j;

    y_current = reg;
    free_yank_all();
    *y_current = *curr;
    y_current->y_array = lalloc_clear(
				    sizeof(char_u *) * y_current->y_size, TRUE);
    if (y_current->y_array == NULL)
	y_current->y_size = 0;
    else
	for (j = 0; j < y_current->y_size; ++j)
	    if ((y_current->y_array[j] = vim_strsave(curr->y_array[j])) == NULL)
	    {
		free_yank(j);
		y_current->y_size = 0;
		break;
	    }
    y_current = curr;
}
#endif

/*
 * Put contents of register "regname" into the text.
 * Caller must check "regname" to be valid!
 * "flags": PUT_FIXINDENT	make indent look nice
 *	    PUT_CURSEND		leave cursor after end of new text
 *	    PUT_LINE		force linewise put (":put")
 *	    PUT_BLOCK_INNER     in block mode, do not add trailing spaces
 */
    void
do_put(
    int		regname,
    char_u	*expr_result,	// result for regname "=" when compiled
    int		dir,		// BACKWARD for 'P', FORWARD for 'p'
    long	count,
    int		flags)
{
    char_u	*ptr;
    char_u	*newp, *oldp;
    int		yanklen;
    int		totlen = 0;		// init for gcc
    linenr_T	lnum;
    colnr_T	col;
    long	i;			// index in y_array[]
    int		y_type;
    long	y_size;
    int		oldlen;
    long	y_width = 0;
    colnr_T	vcol;
    int		delcount;
    int		incr = 0;
    long	j;
    struct block_def bd;
    char_u	**y_array = NULL;
    yankreg_T	*y_current_used = NULL;
    long	nr_lines = 0;
    pos_T	new_cursor;
    int		indent;
    int		orig_indent = 0;	// init for gcc
    int		indent_diff = 0;	// init for gcc
    int		first_indent = TRUE;
    int		lendiff = 0;
    pos_T	old_pos;
    char_u	*insert_string = NULL;
    int		allocated = FALSE;
    long	cnt;
    pos_T	orig_start = curbuf->b_op_start;
    pos_T	orig_end = curbuf->b_op_end;
    unsigned int cur_ve_flags = get_ve_flags();

#ifdef FEAT_CLIPBOARD
    // Adjust register name for "unnamed" in 'clipboard'.
    adjust_clip_reg(&regname);
    (void)may_get_selection(regname);
#endif

    if (flags & PUT_FIXINDENT)
	orig_indent = get_indent();

    curbuf->b_op_start = curwin->w_cursor;	// default for '[ mark
    curbuf->b_op_end = curwin->w_cursor;	// default for '] mark

    // Using inserted text works differently, because the register includes
    // special characters (newlines, etc.).
    if (regname == '.')
    {
	if (VIsual_active)
	    stuffcharReadbuff(VIsual_mode);
	(void)stuff_inserted((dir == FORWARD ? (count == -1 ? 'o' : 'a') :
				    (count == -1 ? 'O' : 'i')), count, FALSE);
	// Putting the text is done later, so can't really move the cursor to
	// the next character.  Use "l" to simulate it.
	if ((flags & PUT_CURSEND) && gchar_cursor() != NUL)
	    stuffcharReadbuff('l');
	return;
    }

    // For special registers '%' (file name), '#' (alternate file name) and
    // ':' (last command line), etc. we have to create a fake yank register.
    // For compiled code "expr_result" holds the expression result.
    if (regname == '=' && expr_result != NULL)
	insert_string = expr_result;
    else if (get_spec_reg(regname, &insert_string, &allocated, TRUE)
		&& insert_string == NULL)
	return;

    // Autocommands may be executed when saving lines for undo.  This might
    // make "y_array" invalid, so we start undo now to avoid that.
    if (u_save(curwin->w_cursor.lnum, curwin->w_cursor.lnum + 1) == FAIL)
	goto end;

    if (insert_string != NULL)
    {
	y_type = MCHAR;
#ifdef FEAT_EVAL
	if (regname == '=')
	{
	    // For the = register we need to split the string at NL
	    // characters.
	    // Loop twice: count the number of lines and save them.
	    for (;;)
	    {
		y_size = 0;
		ptr = insert_string;
		while (ptr != NULL)
		{
		    if (y_array != NULL)
			y_array[y_size] = ptr;
		    ++y_size;
		    ptr = vim_strchr(ptr, '\n');
		    if (ptr != NULL)
		    {
			if (y_array != NULL)
			    *ptr = NUL;
			++ptr;
			// A trailing '\n' makes the register linewise.
			if (*ptr == NUL)
			{
			    y_type = MLINE;
			    break;
			}
		    }
		}
		if (y_array != NULL)
		    break;
		y_array = ALLOC_MULT(char_u *, y_size);
		if (y_array == NULL)
		    goto end;
	    }
	}
	else
#endif
	{
	    y_size = 1;		// use fake one-line yank register
	    y_array = &insert_string;
	}
    }
    else
    {
	get_yank_register(regname, FALSE);

	y_type = y_current->y_type;
	y_width = y_current->y_width;
	y_size = y_current->y_size;
	y_array = y_current->y_array;
	y_current_used = y_current;
    }

    if (y_type == MLINE)
    {
	if (flags & PUT_LINE_SPLIT)
	{
	    char_u *p;

	    // "p" or "P" in Visual mode: split the lines to put the text in
	    // between.
	    if (u_save_cursor() == FAIL)
		goto end;
	    p = ml_get_cursor();
	    if (dir == FORWARD && *p != NUL)
		MB_PTR_ADV(p);
	    ptr = vim_strsave(p);
	    if (ptr == NULL)
		goto end;
	    ml_append(curwin->w_cursor.lnum, ptr, (colnr_T)0, FALSE);
	    vim_free(ptr);

	    oldp = ml_get_curline();
	    p = oldp + curwin->w_cursor.col;
	    if (dir == FORWARD && *p != NUL)
		MB_PTR_ADV(p);
	    ptr = vim_strnsave(oldp, p - oldp);
	    if (ptr == NULL)
		goto end;
	    ml_replace(curwin->w_cursor.lnum, ptr, FALSE);
	    ++nr_lines;
	    dir = FORWARD;
	}
	if (flags & PUT_LINE_FORWARD)
	{
	    // Must be "p" for a Visual block, put lines below the block.
	    curwin->w_cursor = curbuf->b_visual.vi_end;
	    dir = FORWARD;
	}
	curbuf->b_op_start = curwin->w_cursor;	// default for '[ mark
	curbuf->b_op_end = curwin->w_cursor;	// default for '] mark
    }

    if (flags & PUT_LINE)	// :put command or "p" in Visual line mode.
	y_type = MLINE;

    if (y_size == 0 || y_array == NULL)
    {
	semsg(_(e_nothing_in_register_str),
		  regname == 0 ? (char_u *)"\"" : transchar(regname));
	goto end;
    }

    if (y_type == MBLOCK)
    {
	lnum = curwin->w_cursor.lnum + y_size + 1;
	if (lnum > curbuf->b_ml.ml_line_count)
	    lnum = curbuf->b_ml.ml_line_count + 1;
	if (u_save(curwin->w_cursor.lnum - 1, lnum) == FAIL)
	    goto end;
    }
    else if (y_type == MLINE)
    {
	lnum = curwin->w_cursor.lnum;
#ifdef FEAT_FOLDING
	// Correct line number for closed fold.  Don't move the cursor yet,
	// u_save() uses it.
	if (dir == BACKWARD)
	    (void)hasFolding(lnum, &lnum, NULL);
	else
	    (void)hasFolding(lnum, NULL, &lnum);
#endif
	if (dir == FORWARD)
	    ++lnum;
	// In an empty buffer the empty line is going to be replaced, include
	// it in the saved lines.
	if ((BUFEMPTY() ? u_save(0, 2) : u_save(lnum - 1, lnum)) == FAIL)
	    goto end;
#ifdef FEAT_FOLDING
	if (dir == FORWARD)
	    curwin->w_cursor.lnum = lnum - 1;
	else
	    curwin->w_cursor.lnum = lnum;
	curbuf->b_op_start = curwin->w_cursor;	// for mark_adjust()
#endif
    }
    else if (u_save_cursor() == FAIL)
	goto end;

    yanklen = (int)STRLEN(y_array[0]);

    if (cur_ve_flags == VE_ALL && y_type == MCHAR)
    {
	if (gchar_cursor() == TAB)
	{
	    int viscol = getviscol();
	    int ts = curbuf->b_p_ts;

	    // Don't need to insert spaces when "p" on the last position of a
	    // tab or "P" on the first position.
	    if (dir == FORWARD ?
#ifdef FEAT_VARTABS
		    tabstop_padding(viscol, ts, curbuf->b_p_vts_array) != 1
#else
		    ts - (viscol % ts) != 1
#endif
		    : curwin->w_cursor.coladd > 0)
		coladvance_force(viscol);
	    else
		curwin->w_cursor.coladd = 0;
	}
	else if (curwin->w_cursor.coladd > 0 || gchar_cursor() == NUL)
	    coladvance_force(getviscol() + (dir == FORWARD));
    }

    lnum = curwin->w_cursor.lnum;
    col = curwin->w_cursor.col;

    // Block mode
    if (y_type == MBLOCK)
    {
	int	c = gchar_cursor();
	colnr_T	endcol2 = 0;

	if (dir == FORWARD && c != NUL)
	{
	    if (cur_ve_flags == VE_ALL)
		getvcol(curwin, &curwin->w_cursor, &col, NULL, &endcol2);
	    else
		getvcol(curwin, &curwin->w_cursor, NULL, NULL, &col);

	    if (has_mbyte)
		// move to start of next multi-byte character
		curwin->w_cursor.col += (*mb_ptr2len)(ml_get_cursor());
	    else
	    if (c != TAB || cur_ve_flags != VE_ALL)
		++curwin->w_cursor.col;
	    ++col;
	}
	else
	    getvcol(curwin, &curwin->w_cursor, &col, NULL, &endcol2);

	col += curwin->w_cursor.coladd;
	if (cur_ve_flags == VE_ALL
		&& (curwin->w_cursor.coladd > 0
		    || endcol2 == curwin->w_cursor.col))
	{
	    if (dir == FORWARD && c == NUL)
		++col;
	    if (dir != FORWARD && c != NUL && curwin->w_cursor.coladd > 0)
		++curwin->w_cursor.col;
	    if (c == TAB)
	    {
		if (dir == BACKWARD && curwin->w_cursor.col)
		    curwin->w_cursor.col--;
		if (dir == FORWARD && col - 1 == endcol2)
		    curwin->w_cursor.col++;
	    }
	}
	curwin->w_cursor.coladd = 0;
	bd.textcol = 0;
	for (i = 0; i < y_size; ++i)
	{
	    int		    spaces = 0;
	    char	    shortline;
	    chartabsize_T   cts;

	    bd.startspaces = 0;
	    bd.endspaces = 0;
	    vcol = 0;
	    delcount = 0;

	    // add a new line
	    if (curwin->w_cursor.lnum > curbuf->b_ml.ml_line_count)
	    {
		if (ml_append(curbuf->b_ml.ml_line_count, (char_u *)"",
						   (colnr_T)1, FALSE) == FAIL)
		    break;
		++nr_lines;
	    }
	    // get the old line and advance to the position to insert at
	    oldp = ml_get_curline();
	    oldlen = (int)STRLEN(oldp);
	    init_chartabsize_arg(&cts, curwin, curwin->w_cursor.lnum, 0,
								  oldp, oldp);

	    while (cts.cts_vcol < col && *cts.cts_ptr != NUL)
	    {
		// Count a tab for what it's worth (if list mode not on)
		incr = lbr_chartabsize_adv(&cts);
		cts.cts_vcol += incr;
	    }
	    vcol = cts.cts_vcol;
	    ptr = cts.cts_ptr;
	    bd.textcol = (colnr_T)(ptr - oldp);
	    clear_chartabsize_arg(&cts);

	    shortline = (vcol < col) || (vcol == col && !*ptr) ;

	    if (vcol < col) // line too short, pad with spaces
		bd.startspaces = col - vcol;
	    else if (vcol > col)
	    {
		bd.endspaces = vcol - col;
		bd.startspaces = incr - bd.endspaces;
		--bd.textcol;
		delcount = 1;
		if (has_mbyte)
		    bd.textcol -= (*mb_head_off)(oldp, oldp + bd.textcol);
		if (oldp[bd.textcol] != TAB)
		{
		    // Only a Tab can be split into spaces.  Other
		    // characters will have to be moved to after the
		    // block, causing misalignment.
		    delcount = 0;
		    bd.endspaces = 0;
		}
	    }

	    yanklen = (int)STRLEN(y_array[i]);

	    if ((flags & PUT_BLOCK_INNER) == 0)
	    {
		// calculate number of spaces required to fill right side of
		// block
		spaces = y_width + 1;
		init_chartabsize_arg(&cts, curwin, 0, 0,
						      y_array[i], y_array[i]);

		while (*cts.cts_ptr != NUL)
		{
		    spaces -= lbr_chartabsize_adv(&cts);
		    cts.cts_vcol = 0;
		}
		clear_chartabsize_arg(&cts);
		if (spaces < 0)
		    spaces = 0;
	    }

	    // Insert the new text.
	    // First check for multiplication overflow.
	    if (yanklen + spaces != 0
		     && count > ((INT_MAX - (bd.startspaces + bd.endspaces))
							/ (yanklen + spaces)))
	    {
		emsg(_(e_resulting_text_too_long));
		break;
	    }

	    totlen = count * (yanklen + spaces) + bd.startspaces + bd.endspaces;
	    newp = alloc(totlen + oldlen + 1);
	    if (newp == NULL)
		break;

	    // copy part up to cursor to new line
	    ptr = newp;
	    mch_memmove(ptr, oldp, (size_t)bd.textcol);
	    ptr += bd.textcol;

	    // may insert some spaces before the new text
	    vim_memset(ptr, ' ', (size_t)bd.startspaces);
	    ptr += bd.startspaces;

	    // insert the new text
	    for (j = 0; j < count; ++j)
	    {
		mch_memmove(ptr, y_array[i], (size_t)yanklen);
		ptr += yanklen;

		// insert block's trailing spaces only if there's text behind
		if ((j < count - 1 || !shortline) && spaces > 0)
		{
		    vim_memset(ptr, ' ', (size_t)spaces);
		    ptr += spaces;
		}
		else
		    totlen -= spaces;  // didn't use these spaces
	    }

	    // may insert some spaces after the new text
	    vim_memset(ptr, ' ', (size_t)bd.endspaces);
	    ptr += bd.endspaces;

	    // move the text after the cursor to the end of the line.
	    mch_memmove(ptr, oldp + bd.textcol + delcount,
				(size_t)(oldlen - bd.textcol - delcount + 1));
	    ml_replace(curwin->w_cursor.lnum, newp, FALSE);

	    ++curwin->w_cursor.lnum;
	    if (i == 0)
		curwin->w_cursor.col += bd.startspaces;
	}

	changed_lines(lnum, 0, curwin->w_cursor.lnum, nr_lines);

	// Set '[ mark.
	curbuf->b_op_start = curwin->w_cursor;
	curbuf->b_op_start.lnum = lnum;

	// adjust '] mark
	curbuf->b_op_end.lnum = curwin->w_cursor.lnum - 1;
	curbuf->b_op_end.col = bd.textcol + totlen - 1;
	if (curbuf->b_op_end.col < 0)
	    curbuf->b_op_end.col = 0;
	curbuf->b_op_end.coladd = 0;
	if (flags & PUT_CURSEND)
	{
	    colnr_T len;

	    curwin->w_cursor = curbuf->b_op_end;
	    curwin->w_cursor.col++;

	    // in Insert mode we might be after the NUL, correct for that
	    len = (colnr_T)STRLEN(ml_get_curline());
	    if (curwin->w_cursor.col > len)
		curwin->w_cursor.col = len;
	}
	else
	    curwin->w_cursor.lnum = lnum;
    }
    else
    {
	// Character or Line mode
	if (y_type == MCHAR)
	{
	    // if type is MCHAR, FORWARD is the same as BACKWARD on the next
	    // char
	    if (dir == FORWARD && gchar_cursor() != NUL)
	    {
		if (has_mbyte)
		{
		    int bytelen = (*mb_ptr2len)(ml_get_cursor());

		    // put it on the next of the multi-byte character.
		    col += bytelen;
		    if (yanklen)
		    {
			curwin->w_cursor.col += bytelen;
			curbuf->b_op_end.col += bytelen;
		    }
		}
		else
		{
		    ++col;
		    if (yanklen)
		    {
			++curwin->w_cursor.col;
			++curbuf->b_op_end.col;
		    }
		}
	    }
	    curbuf->b_op_start = curwin->w_cursor;
	}
	// Line mode: BACKWARD is the same as FORWARD on the previous line
	else if (dir == BACKWARD)
	    --lnum;
	new_cursor = curwin->w_cursor;

	// simple case: insert into one line at a time
	if (y_type == MCHAR && y_size == 1)
	{
	    linenr_T	end_lnum = 0; // init for gcc
	    linenr_T	start_lnum = lnum;
	    int		first_byte_off = 0;

	    if (VIsual_active)
	    {
		end_lnum = curbuf->b_visual.vi_end.lnum;
		if (end_lnum < curbuf->b_visual.vi_start.lnum)
		    end_lnum = curbuf->b_visual.vi_start.lnum;
		if (end_lnum > start_lnum)
		{
		    pos_T   pos;

		    // "col" is valid for the first line, in following lines
		    // the virtual column needs to be used.  Matters for
		    // multi-byte characters.
		    pos.lnum = lnum;
		    pos.col = col;
		    pos.coladd = 0;
		    getvcol(curwin, &pos, NULL, &vcol, NULL);
		}
	    }

	    if (count == 0 || yanklen == 0)
	    {
		if (VIsual_active)
		    lnum = end_lnum;
	    }
	    else if (count > INT_MAX / yanklen)
		// multiplication overflow
		emsg(_(e_resulting_text_too_long));
	    else
	    {
		totlen = count * yanklen;
		do {
		    oldp = ml_get(lnum);
		    oldlen = (int)STRLEN(oldp);
		    if (lnum > start_lnum)
		    {
			pos_T   pos;

			pos.lnum = lnum;
			if (getvpos(&pos, vcol) == OK)
			    col = pos.col;
			else
			    col = MAXCOL;
		    }
		    if (VIsual_active && col > oldlen)
		    {
			lnum++;
			continue;
		    }
		    newp = alloc(totlen + oldlen + 1);
		    if (newp == NULL)
			goto end;	// alloc() gave an error message
		    mch_memmove(newp, oldp, (size_t)col);
		    ptr = newp + col;
		    for (i = 0; i < count; ++i)
		    {
			mch_memmove(ptr, y_array[0], (size_t)yanklen);
			ptr += yanklen;
		    }
		    STRMOVE(ptr, oldp + col);

		    // compute the byte offset for the last character
		    first_byte_off = mb_head_off(newp, ptr - 1);

		    // Note: this may free "newp"
		    ml_replace(lnum, newp, FALSE);

		    inserted_bytes(lnum, col, totlen);

		    // Place cursor on last putted char.
		    if (lnum == curwin->w_cursor.lnum)
		    {
			// make sure curwin->w_virtcol is updated
			changed_cline_bef_curs();
			invalidate_botline();
			curwin->w_cursor.col += (colnr_T)(totlen - 1);
		    }
		    if (VIsual_active)
			lnum++;
		} while (VIsual_active && lnum <= end_lnum);

		if (VIsual_active) // reset lnum to the last visual line
		    lnum--;
	    }

	    // put '] at the first byte of the last character
	    curbuf->b_op_end = curwin->w_cursor;
	    curbuf->b_op_end.col -= first_byte_off;

	    // For "CTRL-O p" in Insert mode, put cursor after last char
	    if (totlen && (restart_edit != 0 || (flags & PUT_CURSEND)))
		++curwin->w_cursor.col;
	    else
		curwin->w_cursor.col -= first_byte_off;
	}
	else
	{
	    linenr_T	new_lnum = new_cursor.lnum;
	    size_t	len;

	    // Insert at least one line.  When y_type is MCHAR, break the first
	    // line in two.
	    for (cnt = 1; cnt <= count; ++cnt)
	    {
		i = 0;
		if (y_type == MCHAR)
		{
		    // Split the current line in two at the insert position.
		    // First insert y_array[size - 1] in front of second line.
		    // Then append y_array[0] to first line.
		    lnum = new_cursor.lnum;
		    ptr = ml_get(lnum) + col;
		    totlen = (int)STRLEN(y_array[y_size - 1]);
		    newp = alloc(STRLEN(ptr) + totlen + 1);
		    if (newp == NULL)
			goto error;
		    STRCPY(newp, y_array[y_size - 1]);
		    STRCAT(newp, ptr);
		    // insert second line
		    ml_append(lnum, newp, (colnr_T)0, FALSE);
		    ++new_lnum;
		    vim_free(newp);

		    oldp = ml_get(lnum);
		    newp = alloc(col + yanklen + 1);
		    if (newp == NULL)
			goto error;
					    // copy first part of line
		    mch_memmove(newp, oldp, (size_t)col);
					    // append to first line
		    mch_memmove(newp + col, y_array[0], (size_t)(yanklen + 1));
		    ml_replace(lnum, newp, FALSE);

		    curwin->w_cursor.lnum = lnum;
		    i = 1;
		}

		for (; i < y_size; ++i)
		{
		    if (y_type != MCHAR || i < y_size - 1)
		    {
			if (ml_append(lnum, y_array[i], (colnr_T)0, FALSE)
								      == FAIL)
			    goto error;
			new_lnum++;
		    }
		    lnum++;
		    ++nr_lines;
		    if (flags & PUT_FIXINDENT)
		    {
			old_pos = curwin->w_cursor;
			curwin->w_cursor.lnum = lnum;
			ptr = ml_get(lnum);
			if (cnt == count && i == y_size - 1)
			    lendiff = (int)STRLEN(ptr);
			if (*ptr == '#' && preprocs_left())
			    indent = 0;     // Leave # lines at start
			else
			     if (*ptr == NUL)
			    indent = 0;     // Ignore empty lines
			else if (first_indent)
			{
			    indent_diff = orig_indent - get_indent();
			    indent = orig_indent;
			    first_indent = FALSE;
			}
			else if ((indent = get_indent() + indent_diff) < 0)
			    indent = 0;
			(void)set_indent(indent, 0);
			curwin->w_cursor = old_pos;
			// remember how many chars were removed
			if (cnt == count && i == y_size - 1)
			    lendiff -= (int)STRLEN(ml_get(lnum));
		    }
		}
		if (cnt == 1)
		    new_lnum = lnum;
	    }

error:
	    // Adjust marks.
	    if (y_type == MLINE)
	    {
		curbuf->b_op_start.col = 0;
		if (dir == FORWARD)
		    curbuf->b_op_start.lnum++;
	    }
	    mark_adjust(curbuf->b_op_start.lnum + (y_type == MCHAR),
					     (linenr_T)MAXLNUM, nr_lines, 0L);

	    // note changed text for displaying and folding
	    if (y_type == MCHAR)
		changed_lines(curwin->w_cursor.lnum, col,
					 curwin->w_cursor.lnum + 1, nr_lines);
	    else
		changed_lines(curbuf->b_op_start.lnum, 0,
					   curbuf->b_op_start.lnum, nr_lines);
	    if (y_current_used != NULL && (y_current_used != y_current
					     || y_current->y_array != y_array))
	    {
		// Something invoked through changed_lines() has changed the
		// yank buffer, e.g. a GUI clipboard callback.
		emsg(_(e_yank_register_changed_while_using_it));
		goto end;
	    }

	    // Put the '] mark on the first byte of the last inserted character.
	    // Correct the length for change in indent.
	    curbuf->b_op_end.lnum = new_lnum;
	    len = STRLEN(y_array[y_size - 1]);
	    col = (colnr_T)len - lendiff;
	    if (col > 1)
	    {
		curbuf->b_op_end.col = col - 1;
		if (len > 0)
		    curbuf->b_op_end.col -= mb_head_off(y_array[y_size - 1],
						y_array[y_size - 1] + len - 1);
	    }
	    else
		curbuf->b_op_end.col = 0;

	    if (flags & PUT_CURSLINE)
	    {
		// ":put": put cursor on last inserted line
		curwin->w_cursor.lnum = lnum;
		beginline(BL_WHITE | BL_FIX);
	    }
	    else if (flags & PUT_CURSEND)
	    {
		// put cursor after inserted text
		if (y_type == MLINE)
		{
		    if (lnum >= curbuf->b_ml.ml_line_count)
			curwin->w_cursor.lnum = curbuf->b_ml.ml_line_count;
		    else
			curwin->w_cursor.lnum = lnum + 1;
		    curwin->w_cursor.col = 0;
		}
		else
		{
		    curwin->w_cursor.lnum = new_lnum;
		    curwin->w_cursor.col = col;
		    curbuf->b_op_end = curwin->w_cursor;
		    if (col > 1)
			curbuf->b_op_end.col = col - 1;
		}
	    }
	    else if (y_type == MLINE)
	    {
		// put cursor on first non-blank in first inserted line
		curwin->w_cursor.col = 0;
		if (dir == FORWARD)
		    ++curwin->w_cursor.lnum;
		beginline(BL_WHITE | BL_FIX);
	    }
	    else	// put cursor on first inserted character
		curwin->w_cursor = new_cursor;
	}
    }

    msgmore(nr_lines);
    curwin->w_set_curswant = TRUE;

    // Make sure the cursor is not after the NUL.
    int len = (int)STRLEN(ml_get_curline());
    if (curwin->w_cursor.col > len)
    {
	if (cur_ve_flags == VE_ALL)
	    curwin->w_cursor.coladd = curwin->w_cursor.col - len;
	curwin->w_cursor.col = len;
    }

end:
    if (cmdmod.cmod_flags & CMOD_LOCKMARKS)
    {
	curbuf->b_op_start = orig_start;
	curbuf->b_op_end = orig_end;
    }
    if (allocated)
	vim_free(insert_string);
    if (regname == '=')
	vim_free(y_array);

    VIsual_active = FALSE;

    // If the cursor is past the end of the line put it at the end.
    adjust_cursor_eol();
}

/*
 * Return the character name of the register with the given number.
 */
    int
get_register_name(int num)
{
    if (num == -1)
	return '"';
    else if (num < 10)
	return num + '0';
    else if (num == DELETION_REGISTER)
	return '-';
#ifdef FEAT_CLIPBOARD
    else if (num == STAR_REGISTER)
	return '*';
    else if (num == PLUS_REGISTER)
	return '+';
#endif
    else
	return num + 'a' - 10;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return the index of the register "" points to.
 */
    int
get_unname_register(void)
{
    return y_previous == NULL ? -1 : y_previous - &y_regs[0];
}
#endif

/*
 * ":dis" and ":registers": Display the contents of the yank registers.
 */
    void
ex_display(exarg_T *eap)
{
    int		i, n;
    long	j;
    char_u	*p;
    yankreg_T	*yb;
    int		name;
    int		attr;
    char_u	*arg = eap->arg;
    int		clen;
    int		type;

    if (arg != NULL && *arg == NUL)
	arg = NULL;
    attr = HL_ATTR(HLF_8);

    // Highlight title
    msg_puts_title(_("\nType Name Content"));
    for (i = -1; i < NUM_REGISTERS && !got_int; ++i)
    {
	name = get_register_name(i);
	switch (get_reg_type(name, NULL))
	{
	    case MLINE: type = 'l'; break;
	    case MCHAR: type = 'c'; break;
	    default:	type = 'b'; break;
	}
	if (arg != NULL && vim_strchr(arg, name) == NULL
#ifdef ONE_CLIPBOARD
	    // Star register and plus register contain the same thing.
		&& (name != '*' || vim_strchr(arg, '+') == NULL)
#endif
		)
	    continue;	    // did not ask for this register

#ifdef FEAT_CLIPBOARD
	// Adjust register name for "unnamed" in 'clipboard'.
	// When it's a clipboard register, fill it with the current contents
	// of the clipboard.
	adjust_clip_reg(&name);
	(void)may_get_selection(name);
#endif

	if (i == -1)
	{
	    if (y_previous != NULL)
		yb = y_previous;
	    else
		yb = &(y_regs[0]);
	}
	else
	    yb = &(y_regs[i]);

#ifdef FEAT_EVAL
	if (name == MB_TOLOWER(redir_reg)
		|| (redir_reg == '"' && yb == y_previous))
	    continue;	    // do not list register being written to, the
			    // pointer can be freed
#endif

	if (yb->y_array != NULL)
	{
	    int do_show = FALSE;

	    for (j = 0; !do_show && j < yb->y_size; ++j)
		do_show = !message_filtered(yb->y_array[j]);

	    if (do_show || yb->y_size == 0)
	    {
		msg_putchar('\n');
		msg_puts("  ");
		msg_putchar(type);
		msg_puts("  ");
		msg_putchar('"');
		msg_putchar(name);
		msg_puts("   ");

		n = (int)Columns - 11;
		for (j = 0; j < yb->y_size && n > 1; ++j)
		{
		    if (j)
		    {
			msg_puts_attr("^J", attr);
			n -= 2;
		    }
		    for (p = yb->y_array[j];
				    *p != NUL && (n -= ptr2cells(p)) >= 0; ++p)
		    {
			clen = (*mb_ptr2len)(p);
			msg_outtrans_len(p, clen);
			p += clen - 1;
		    }
		}
		if (n > 1 && yb->y_type == MLINE)
		    msg_puts_attr("^J", attr);
		out_flush();		    // show one line at a time
	    }
	    ui_breakcheck();
	}
    }

    // display last inserted text
    if ((p = get_last_insert()) != NULL
		  && (arg == NULL || vim_strchr(arg, '.') != NULL) && !got_int
						      && !message_filtered(p))
    {
	msg_puts("\n  c  \".   ");
	dis_msg(p, TRUE);
    }

    // display last command line
    if (last_cmdline != NULL && (arg == NULL || vim_strchr(arg, ':') != NULL)
			       && !got_int && !message_filtered(last_cmdline))
    {
	msg_puts("\n  c  \":   ");
	dis_msg(last_cmdline, FALSE);
    }

    // display current file name
    if (curbuf->b_fname != NULL
	    && (arg == NULL || vim_strchr(arg, '%') != NULL) && !got_int
					&& !message_filtered(curbuf->b_fname))
    {
	msg_puts("\n  c  \"%   ");
	dis_msg(curbuf->b_fname, FALSE);
    }

    // display alternate file name
    if ((arg == NULL || vim_strchr(arg, '%') != NULL) && !got_int)
    {
	char_u	    *fname;
	linenr_T    dummy;

	if (buflist_name_nr(0, &fname, &dummy) != FAIL
						  && !message_filtered(fname))
	{
	    msg_puts("\n  c  \"#   ");
	    dis_msg(fname, FALSE);
	}
    }

    // display last search pattern
    if (last_search_pat() != NULL
		 && (arg == NULL || vim_strchr(arg, '/') != NULL) && !got_int
				      && !message_filtered(last_search_pat()))
    {
	msg_puts("\n  c  \"/   ");
	dis_msg(last_search_pat(), FALSE);
    }

#ifdef FEAT_EVAL
    // display last used expression
    if (expr_line != NULL && (arg == NULL || vim_strchr(arg, '=') != NULL)
				  && !got_int && !message_filtered(expr_line))
    {
	msg_puts("\n  c  \"=   ");
	dis_msg(expr_line, FALSE);
    }
#endif
}

/*
 * display a string for do_dis()
 * truncate at end of screen line
 */
    static void
dis_msg(
    char_u	*p,
    int		skip_esc)	    // if TRUE, ignore trailing ESC
{
    int		n;
    int		l;

    n = (int)Columns - 6;
    while (*p != NUL
	    && !(*p == ESC && skip_esc && *(p + 1) == NUL)
	    && (n -= ptr2cells(p)) >= 0)
    {
	if (has_mbyte && (l = (*mb_ptr2len)(p)) > 1)
	{
	    msg_outtrans_len(p, l);
	    p += l;
	}
	else
	    msg_outtrans_len(p++, 1);
    }
    ui_breakcheck();
}

#if defined(FEAT_DND) || defined(PROTO)
/*
 * Replace the contents of the '~' register with str.
 */
    void
dnd_yank_drag_data(char_u *str, long len)
{
    yankreg_T *curr;

    curr = y_current;
    y_current = &y_regs[TILDE_REGISTER];
    free_yank_all();
    str_to_reg(y_current, MCHAR, str, len, 0L, FALSE);
    y_current = curr;
}
#endif


/*
 * Return the type of a register.
 * Used for getregtype()
 * Returns MAUTO for error.
 */
    char_u
get_reg_type(int regname, long *reglen)
{
    switch (regname)
    {
	case '%':		// file name
	case '#':		// alternate file name
	case '=':		// expression
	case ':':		// last command line
	case '/':		// last search-pattern
	case '.':		// last inserted text
	case Ctrl_F:		// Filename under cursor
	case Ctrl_P:		// Path under cursor, expand via "path"
	case Ctrl_W:		// word under cursor
	case Ctrl_A:		// WORD (mnemonic All) under cursor
	case '_':		// black hole: always empty
	    return MCHAR;
    }

# ifdef FEAT_CLIPBOARD
    regname = may_get_selection(regname);
# endif

    if (regname != NUL && !valid_yank_reg(regname, FALSE))
	return MAUTO;

    get_yank_register(regname, FALSE);

    if (y_current->y_array != NULL)
    {
	if (reglen != NULL && y_current->y_type == MBLOCK)
	    *reglen = y_current->y_width;
	return y_current->y_type;
    }
    return MAUTO;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * When "flags" has GREG_LIST return a list with text "s".
 * Otherwise just return "s".
 */
    static char_u *
getreg_wrap_one_line(char_u *s, int flags)
{
    if (flags & GREG_LIST)
    {
	list_T *list = list_alloc();

	if (list != NULL)
	{
	    if (list_append_string(list, NULL, -1) == FAIL)
	    {
		list_free(list);
		return NULL;
	    }
	    list->lv_first->li_tv.vval.v_string = s;
	}
	return (char_u *)list;
    }
    return s;
}

/*
 * Return the contents of a register as a single allocated string or as a list.
 * Used for "@r" in expressions and for getreg().
 * Returns NULL for error.
 * Flags:
 *	GREG_NO_EXPR	Do not allow expression register
 *	GREG_EXPR_SRC	For the expression register: return expression itself,
 *			not the result of its evaluation.
 *	GREG_LIST	Return a list of lines instead of a single string.
 */
    char_u *
get_reg_contents(int regname, int flags)
{
    long	i;
    char_u	*retval;
    int		allocated;
    long	len;

    // Don't allow using an expression register inside an expression
    if (regname == '=')
    {
	if (flags & GREG_NO_EXPR)
	    return NULL;
	if (flags & GREG_EXPR_SRC)
	    return getreg_wrap_one_line(get_expr_line_src(), flags);
	return getreg_wrap_one_line(get_expr_line(), flags);
    }

    if (regname == '@')	    // "@@" is used for unnamed register
	regname = '"';

    // check for valid regname
    if (regname != NUL && !valid_yank_reg(regname, FALSE))
	return NULL;

# ifdef FEAT_CLIPBOARD
    regname = may_get_selection(regname);
# endif

    if (get_spec_reg(regname, &retval, &allocated, FALSE))
    {
	if (retval == NULL)
	    return NULL;
	if (allocated)
	    return getreg_wrap_one_line(retval, flags);
	return getreg_wrap_one_line(vim_strsave(retval), flags);
    }

    get_yank_register(regname, FALSE);
    if (y_current->y_array == NULL)
	return NULL;

    if (flags & GREG_LIST)
    {
	list_T	*list = list_alloc();
	int	error = FALSE;

	if (list == NULL)
	    return NULL;
	for (i = 0; i < y_current->y_size; ++i)
	    if (list_append_string(list, y_current->y_array[i], -1) == FAIL)
		error = TRUE;
	if (error)
	{
	    list_free(list);
	    return NULL;
	}
	return (char_u *)list;
    }

    // Compute length of resulting string.
    len = 0;
    for (i = 0; i < y_current->y_size; ++i)
    {
	len += (long)STRLEN(y_current->y_array[i]);
	// Insert a newline between lines and after last line if
	// y_type is MLINE.
	if (y_current->y_type == MLINE || i < y_current->y_size - 1)
	    ++len;
    }

    retval = alloc(len + 1);
    if (retval == NULL)
	return NULL;

    // Copy the lines of the yank register into the string.
    len = 0;
    for (i = 0; i < y_current->y_size; ++i)
    {
	STRCPY(retval + len, y_current->y_array[i]);
	len += (long)STRLEN(retval + len);

	// Insert a NL between lines and after the last line if y_type is
	// MLINE.
	if (y_current->y_type == MLINE || i < y_current->y_size - 1)
	    retval[len++] = '\n';
    }
    retval[len] = NUL;

    return retval;
}

    static int
init_write_reg(
    int		name,
    yankreg_T	**old_y_previous,
    yankreg_T	**old_y_current,
    int		must_append,
    int		*yank_type UNUSED)
{
    if (!valid_yank_reg(name, TRUE))	    // check for valid reg name
    {
	emsg_invreg(name);
	return FAIL;
    }

    // Don't want to change the current (unnamed) register
    *old_y_previous = y_previous;
    *old_y_current = y_current;

    get_yank_register(name, TRUE);
    if (!y_append && !must_append)
	free_yank_all();
    return OK;
}

    static void
finish_write_reg(
    int		name,
    yankreg_T	*old_y_previous,
    yankreg_T	*old_y_current)
{
# ifdef FEAT_CLIPBOARD
    // Send text of clipboard register to the clipboard.
    may_set_selection();
# endif

    // ':let @" = "val"' should change the meaning of the "" register
    if (name != '"')
	y_previous = old_y_previous;
    y_current = old_y_current;
}

/*
 * Store string "str" in register "name".
 * "maxlen" is the maximum number of bytes to use, -1 for all bytes.
 * If "must_append" is TRUE, always append to the register.  Otherwise append
 * if "name" is an uppercase letter.
 * Note: "maxlen" and "must_append" don't work for the "/" register.
 * Careful: 'str' is modified, you may have to use a copy!
 * If "str" ends in '\n' or '\r', use linewise, otherwise use characterwise.
 */
    void
write_reg_contents(
    int		name,
    char_u	*str,
    int		maxlen,
    int		must_append)
{
    write_reg_contents_ex(name, str, maxlen, must_append, MAUTO, 0L);
}

    void
write_reg_contents_lst(
    int		name,
    char_u	**strings,
    int		maxlen UNUSED,
    int		must_append,
    int		yank_type,
    long	block_len)
{
    yankreg_T  *old_y_previous, *old_y_current;

    if (name == '/' || name == '=')
    {
	char_u	*s;

	if (strings[0] == NULL)
	    s = (char_u *)"";
	else if (strings[1] != NULL)
	{
	    emsg(_(e_search_pattern_and_expression_register_may_not_contain_two_or_more_lines));
	    return;
	}
	else
	    s = strings[0];
	write_reg_contents_ex(name, s, -1, must_append, yank_type, block_len);
	return;
    }

    if (name == '_')	    // black hole: nothing to do
	return;

    if (init_write_reg(name, &old_y_previous, &old_y_current, must_append,
		&yank_type) == FAIL)
	return;

    str_to_reg(y_current, yank_type, (char_u *)strings, -1, block_len, TRUE);

    finish_write_reg(name, old_y_previous, old_y_current);
}

    void
write_reg_contents_ex(
    int		name,
    char_u	*str,
    int		maxlen,
    int		must_append,
    int		yank_type,
    long	block_len)
{
    yankreg_T	*old_y_previous, *old_y_current;
    long	len;

    if (maxlen >= 0)
	len = maxlen;
    else
	len = (long)STRLEN(str);

    // Special case: '/' search pattern
    if (name == '/')
    {
	set_last_search_pat(str, RE_SEARCH, TRUE, TRUE);
	return;
    }

    if (name == '#')
    {
	buf_T	*buf;

	if (VIM_ISDIGIT(*str))
	{
	    int	num = atoi((char *)str);

	    buf = buflist_findnr(num);
	    if (buf == NULL)
		semsg(_(e_buffer_nr_does_not_exist), (long)num);
	}
	else
	    buf = buflist_findnr(buflist_findpat(str, str + STRLEN(str),
							 TRUE, FALSE, FALSE));
	if (buf == NULL)
	    return;
	curwin->w_alt_fnum = buf->b_fnum;
	return;
    }

    if (name == '=')
    {
	char_u	    *p, *s;

	p = vim_strnsave(str, len);
	if (p == NULL)
	    return;
	if (must_append && expr_line != NULL)
	{
	    s = concat_str(expr_line, p);
	    vim_free(p);
	    p = s;
	}
	set_expr_line(p, NULL);
	return;
    }

    if (name == '_')	    // black hole: nothing to do
	return;

    if (init_write_reg(name, &old_y_previous, &old_y_current, must_append,
		&yank_type) == FAIL)
	return;

    str_to_reg(y_current, yank_type, str, len, block_len, FALSE);

    finish_write_reg(name, old_y_previous, old_y_current);
}
#endif	// FEAT_EVAL

#if defined(FEAT_CLIPBOARD) || defined(FEAT_EVAL)
/*
 * Put a string into a register.  When the register is not empty, the string
 * is appended.
 */
    void
str_to_reg(
    yankreg_T	*y_ptr,		// pointer to yank register
    int		yank_type,	// MCHAR, MLINE, MBLOCK, MAUTO
    char_u	*str,		// string to put in register
    long	len,		// length of string
    long	blocklen,	// width of Visual block
    int		str_list)	// TRUE if str is char_u **
{
    int		type;			// MCHAR, MLINE or MBLOCK
    int		lnum;
    long	start;
    long	i;
    int		extra;
    int		newlines;		// number of lines added
    int		extraline = 0;		// extra line at the end
    int		append = FALSE;		// append to last line in register
    char_u	*s;
    char_u	**ss;
    char_u	**pp;
    long	maxlen;

    if (y_ptr->y_array == NULL)		// NULL means empty register
	y_ptr->y_size = 0;

    if (yank_type == MAUTO)
	type = ((str_list || (len > 0 && (str[len - 1] == NL
					    || str[len - 1] == CAR)))
							     ? MLINE : MCHAR);
    else
	type = yank_type;

    // Count the number of lines within the string
    newlines = 0;
    if (str_list)
    {
	for (ss = (char_u **) str; *ss != NULL; ++ss)
	    ++newlines;
    }
    else
    {
	for (i = 0; i < len; i++)
	    if (str[i] == '\n')
		++newlines;
	if (type == MCHAR || len == 0 || str[len - 1] != '\n')
	{
	    extraline = 1;
	    ++newlines;	// count extra newline at the end
	}
	if (y_ptr->y_size > 0 && y_ptr->y_type == MCHAR)
	{
	    append = TRUE;
	    --newlines;	// uncount newline when appending first line
	}
    }

    // Without any lines make the register empty.
    if (y_ptr->y_size + newlines == 0)
    {
	VIM_CLEAR(y_ptr->y_array);
	return;
    }

    // Allocate an array to hold the pointers to the new register lines.
    // If the register was not empty, move the existing lines to the new array.
    pp = lalloc_clear((y_ptr->y_size + newlines) * sizeof(char_u *), TRUE);
    if (pp == NULL)	// out of memory
	return;
    for (lnum = 0; lnum < y_ptr->y_size; ++lnum)
	pp[lnum] = y_ptr->y_array[lnum];
    vim_free(y_ptr->y_array);
    y_ptr->y_array = pp;
    maxlen = 0;

    // Find the end of each line and save it into the array.
    if (str_list)
    {
	for (ss = (char_u **) str; *ss != NULL; ++ss, ++lnum)
	{
	    pp[lnum] = vim_strsave(*ss);
	    if (type == MBLOCK)
	    {
		int charlen = mb_string2cells(*ss, -1);

		if (charlen > maxlen)
		    maxlen = charlen;
	    }
	}
    }
    else
    {
	for (start = 0; start < len + extraline; start += i + 1)
	{
	    int charlen = 0;

	    for (i = start; i < len; ++i)	// find the end of the line
	    {
		if (str[i] == '\n')
		    break;
		if (type == MBLOCK)
		    charlen += mb_ptr2cells_len(str + i, len - i);
	    }
	    i -= start;			// i is now length of line
	    if (charlen > maxlen)
		maxlen = charlen;
	    if (append)
	    {
		--lnum;
		extra = (int)STRLEN(y_ptr->y_array[lnum]);
	    }
	    else
		extra = 0;
	    s = alloc(i + extra + 1);
	    if (s == NULL)
		break;
	    if (extra)
		mch_memmove(s, y_ptr->y_array[lnum], (size_t)extra);
	    if (append)
		vim_free(y_ptr->y_array[lnum]);
	    if (i > 0)
		mch_memmove(s + extra, str + start, (size_t)i);
	    extra += i;
	    s[extra] = NUL;
	    y_ptr->y_array[lnum++] = s;
	    while (--extra >= 0)
	    {
		if (*s == NUL)
		    *s = '\n';	    // replace NUL with newline
		++s;
	    }
	    append = FALSE;		    // only first line is appended
	}
    }
    y_ptr->y_type = type;
    y_ptr->y_size = lnum;
    if (type == MBLOCK)
	y_ptr->y_width = (blocklen < 0 ? maxlen - 1 : blocklen);
    else
	y_ptr->y_width = 0;
# ifdef FEAT_VIMINFO
    y_ptr->y_time_set = vim_time();
# endif
}
#endif // FEAT_CLIPBOARD || FEAT_EVAL || PROTO
