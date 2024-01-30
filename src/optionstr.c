/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * optionstr.c: Functions related to string options
 */

#include "vim.h"

static char_u shm_buf[SHM_LEN];
static int set_shm_recursive = 0;

static char *(p_ambw_values[]) = {"single", "double", NULL};
static char *(p_bg_values[]) = {"light", "dark", NULL};
static char *(p_bkc_values[]) = {"yes", "auto", "no", "breaksymlink", "breakhardlink", NULL};
static char *(p_bo_values[]) = {"all", "backspace", "cursor", "complete",
				 "copy", "ctrlg", "error", "esc", "ex",
				 "hangul", "insertmode", "lang", "mess",
				 "showmatch", "operator", "register", "shell",
				 "spell", "term", "wildmode", NULL};
#if defined(FEAT_LINEBREAK)
// Note: Keep this in sync with briopt_check()
static char *(p_briopt_values[]) = {"shift:", "min:", "sbr", "list:", "column:", NULL};
#endif
#if defined(FEAT_DIFF)
// Note: Keep this in sync with diffopt_changed()
static char *(p_dip_values[]) = {"filler", "context:", "iblank", "icase", "iwhite", "iwhiteall", "iwhiteeol", "horizontal", "vertical", "closeoff", "hiddenoff", "foldcolumn:", "followwrap", "internal", "indent-heuristic", "algorithm:", NULL};
static char *(p_dip_algorithm_values[]) = {"myers", "minimal", "patience", "histogram", NULL};
#endif
static char *(p_nf_values[]) = {"bin", "octal", "hex", "alpha", "unsigned", NULL};
static char *(p_ff_values[]) = {FF_UNIX, FF_DOS, FF_MAC, NULL};
#ifdef FEAT_CLIPBOARD
// Note: Keep this in sync with did_set_clipboard()
static char *(p_cb_values[]) = {"unnamed", "unnamedplus", "autoselect", "autoselectplus", "autoselectml", "html", "exclude:", NULL};
#endif
#ifdef FEAT_CRYPT
static char *(p_cm_values[]) = {"zip", "blowfish", "blowfish2",
 # ifdef FEAT_SODIUM
    "xchacha20", "xchacha20v2",
 # endif
    NULL};
#endif
static char *(p_cmp_values[]) = {"internal", "keepascii", NULL};
#ifdef FEAT_SYN_HL
// Note: Keep this in sync with fill_culopt_flags()
static char *(p_culopt_values[]) = {"line", "screenline", "number", "both", NULL};
#endif
static char *(p_dy_values[]) = {"lastline", "truncate", "uhex", NULL};
static char *(p_jop_values[]) = {"stack", NULL};
#ifdef FEAT_FOLDING
static char *(p_fdo_values[]) = {"all", "block", "hor", "mark", "percent",
				 "quickfix", "search", "tag", "insert",
				 "undo", "jump", NULL};
#endif
// Note: Keep this in sync with match_keyprotocol()
static char *(p_kpc_protocol_values[]) = {"none", "mok2", "kitty", NULL};
#ifdef FEAT_PROP_POPUP
// Note: Keep this in sync with parse_popup_option()
static char *(p_popup_option_values[]) = {"height:", "width:", "highlight:", "border:", "align:", NULL};
static char *(p_popup_option_border_values[]) = {"on", "off", NULL};
static char *(p_popup_option_align_values[]) = {"item", "menu", NULL};
#endif
#if defined(FEAT_SPELL)
// Note: Keep this in sync with spell_check_sps()
static char *(p_sps_values[]) = {"best", "fast", "double", "expr:", "file:", "timeout:", NULL};
#endif
#ifdef FEAT_SESSION
// Also used for 'viewoptions'!  Keep in sync with SSOP_ flags.
static char *(p_ssop_values[]) = {"buffers", "winpos", "resize", "winsize",
    "localoptions", "options", "help", "blank", "globals", "slash", "unix",
    "sesdir", "curdir", "folds", "cursor", "tabpages", "terminal", "skiprtp",
    NULL};
#endif
// Keep in sync with SWB_ flags in option.h
static char *(p_swb_values[]) = {"useopen", "usetab", "split", "newtab", "vsplit", "uselast", NULL};
static char *(p_spk_values[]) = {"cursor", "screen", "topline", NULL};
static char *(p_tc_values[]) = {"followic", "ignore", "match", "followscs", "smart", NULL};
#if defined(FEAT_TOOLBAR) && !defined(FEAT_GUI_MSWIN)
static char *(p_toolbar_values[]) = {"text", "icons", "tooltips", "horiz", NULL};
#endif
#if defined(FEAT_TOOLBAR) && defined(FEAT_GUI_GTK)
static char *(p_tbis_values[]) = {"tiny", "small", "medium", "large", "huge", "giant", NULL};
#endif
#if defined(UNIX) || defined(VMS)
static char *(p_ttym_values[]) = {"xterm", "xterm2", "dec", "netterm", "jsbterm", "pterm", "urxvt", "sgr", NULL};
#endif
static char *(p_ve_values[]) = {"block", "insert", "all", "onemore", "none", "NONE", NULL};
// Note: Keep this in sync with check_opt_wim()
static char *(p_wim_values[]) = {"full", "longest", "list", "lastused", NULL};
static char *(p_wop_values[]) = {"fuzzy", "tagfile", "pum", NULL};
#ifdef FEAT_WAK
static char *(p_wak_values[]) = {"yes", "menu", "no", NULL};
#endif
static char *(p_mousem_values[]) = {"extend", "popup", "popup_setpos", "mac", NULL};
static char *(p_sel_values[]) = {"inclusive", "exclusive", "old", NULL};
static char *(p_slm_values[]) = {"mouse", "key", "cmd", NULL};
static char *(p_km_values[]) = {"startsel", "stopsel", NULL};
#ifdef FEAT_BROWSE
static char *(p_bsdir_values[]) = {"current", "last", "buffer", NULL};
#endif
static char *(p_scbopt_values[]) = {"ver", "hor", "jump", NULL};
static char *(p_debug_values[]) = {"msg", "throw", "beep", NULL};
static char *(p_ead_values[]) = {"both", "ver", "hor", NULL};
static char *(p_buftype_values[]) = {"nofile", "nowrite", "quickfix", "help", "terminal", "acwrite", "prompt", "popup", NULL};
static char *(p_bufhidden_values[]) = {"hide", "unload", "delete", "wipe", NULL};
static char *(p_bs_values[]) = {"indent", "eol", "start", "nostop", NULL};
#ifdef FEAT_FOLDING
static char *(p_fdm_values[]) = {"manual", "expr", "marker", "indent", "syntax",
# ifdef FEAT_DIFF
				"diff",
# endif
				NULL};
static char *(p_fcl_values[]) = {"all", NULL};
#endif
static char *(p_cot_values[]) = {"menu", "menuone", "longest", "preview", "popup", "popuphidden", "noinsert", "noselect", NULL};
#ifdef BACKSLASH_IN_FILENAME
static char *(p_csl_values[]) = {"slash", "backslash", NULL};
#endif
#ifdef FEAT_SIGNS
static char *(p_scl_values[]) = {"yes", "no", "auto", "number", NULL};
#endif
#if defined(MSWIN) && defined(FEAT_TERMINAL)
static char *(p_twt_values[]) = {"winpty", "conpty", "", NULL};
#endif
static char *(p_sloc_values[]) = {"last", "statusline", "tabline", NULL};
static char *(p_sws_values[]) = {"fsync", "sync", NULL};

static int check_opt_strings(char_u *val, char **values, int list);
static int opt_strings_flags(char_u *val, char **values, unsigned *flagp, int list);

/*
 * After setting various option values: recompute variables that depend on
 * option values.
 */
    void
didset_string_options(void)
{
    (void)opt_strings_flags(p_cmp, p_cmp_values, &cmp_flags, TRUE);
    (void)opt_strings_flags(p_bkc, p_bkc_values, &bkc_flags, TRUE);
    (void)opt_strings_flags(p_bo, p_bo_values, &bo_flags, TRUE);
#ifdef FEAT_SESSION
    (void)opt_strings_flags(p_ssop, p_ssop_values, &ssop_flags, TRUE);
    (void)opt_strings_flags(p_vop, p_ssop_values, &vop_flags, TRUE);
#endif
#ifdef FEAT_FOLDING
    (void)opt_strings_flags(p_fdo, p_fdo_values, &fdo_flags, TRUE);
#endif
    (void)opt_strings_flags(p_dy, p_dy_values, &dy_flags, TRUE);
    (void)opt_strings_flags(p_jop, p_jop_values, &jop_flags, TRUE);
    (void)opt_strings_flags(p_tc, p_tc_values, &tc_flags, FALSE);
    (void)opt_strings_flags(p_ve, p_ve_values, &ve_flags, TRUE);
#if defined(UNIX) || defined(VMS)
    (void)opt_strings_flags(p_ttym, p_ttym_values, &ttym_flags, FALSE);
#endif
#if defined(FEAT_TOOLBAR) && !defined(FEAT_GUI_MSWIN)
    (void)opt_strings_flags(p_toolbar, p_toolbar_values, &toolbar_flags, TRUE);
#endif
#if defined(FEAT_TOOLBAR) && defined(FEAT_GUI_GTK)
    (void)opt_strings_flags(p_tbis, p_tbis_values, &tbis_flags, FALSE);
#endif
    (void)opt_strings_flags(p_swb, p_swb_values, &swb_flags, TRUE);
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Trigger the OptionSet autocommand.
 * "opt_idx"	is the index of the option being set.
 * "opt_flags"	can be OPT_LOCAL etc.
 * "oldval"	the old value
 *  "oldval_l"  the old local value (only non-NULL if global and local value
 *		are set)
 * "oldval_g"   the old global value (only non-NULL if global and local value
 *		are set)
 * "newval"	the new value
 */
    void
trigger_optionset_string(
	int	opt_idx,
	int	opt_flags,
	char_u  *oldval,
	char_u  *oldval_l,
	char_u  *oldval_g,
	char_u  *newval)
{
    // Don't do this recursively.
    if (oldval == NULL || newval == NULL
				    || *get_vim_var_str(VV_OPTION_TYPE) != NUL)
	return;

    char_u buf_type[7];

    sprintf((char *)buf_type, "%s",
	    (opt_flags & OPT_LOCAL) ? "local" : "global");
    set_vim_var_string(VV_OPTION_OLD, oldval, -1);
    set_vim_var_string(VV_OPTION_NEW, newval, -1);
    set_vim_var_string(VV_OPTION_TYPE, buf_type, -1);
    if (opt_flags & OPT_LOCAL)
    {
	set_vim_var_string(VV_OPTION_COMMAND, (char_u *)"setlocal", -1);
	set_vim_var_string(VV_OPTION_OLDLOCAL, oldval, -1);
    }
    if (opt_flags & OPT_GLOBAL)
    {
	set_vim_var_string(VV_OPTION_COMMAND, (char_u *)"setglobal", -1);
	set_vim_var_string(VV_OPTION_OLDGLOBAL, oldval, -1);
    }
    if ((opt_flags & (OPT_LOCAL | OPT_GLOBAL)) == 0)
    {
	set_vim_var_string(VV_OPTION_COMMAND, (char_u *)"set", -1);
	set_vim_var_string(VV_OPTION_OLDLOCAL, oldval_l, -1);
	set_vim_var_string(VV_OPTION_OLDGLOBAL, oldval_g, -1);
    }
    if (opt_flags & OPT_MODELINE)
    {
	set_vim_var_string(VV_OPTION_COMMAND, (char_u *)"modeline", -1);
	set_vim_var_string(VV_OPTION_OLDLOCAL, oldval, -1);
    }
    apply_autocmds(EVENT_OPTIONSET,
	    get_option_fullname(opt_idx), NULL, FALSE,
	    NULL);
    reset_v_option_vars();
}
#endif

    static char *
illegal_char(char *errbuf, size_t errbuflen, int c)
{
    if (errbuf == NULL)
	return "";
    vim_snprintf(errbuf, errbuflen, _(e_illegal_character_str),
		    (char *)transchar(c));
    return errbuf;
}

/*
 * Check string options in a buffer for NULL value.
 */
    void
check_buf_options(buf_T *buf)
{
    check_string_option(&buf->b_p_bh);
    check_string_option(&buf->b_p_bt);
    check_string_option(&buf->b_p_fenc);
    check_string_option(&buf->b_p_ff);
#ifdef FEAT_FIND_ID
    check_string_option(&buf->b_p_def);
    check_string_option(&buf->b_p_inc);
# ifdef FEAT_EVAL
    check_string_option(&buf->b_p_inex);
# endif
#endif
#if defined(FEAT_EVAL)
    check_string_option(&buf->b_p_inde);
    check_string_option(&buf->b_p_indk);
#endif
#if defined(FEAT_BEVAL) && defined(FEAT_EVAL)
    check_string_option(&buf->b_p_bexpr);
#endif
#if defined(FEAT_CRYPT)
    check_string_option(&buf->b_p_cm);
#endif
    check_string_option(&buf->b_p_fp);
#if defined(FEAT_EVAL)
    check_string_option(&buf->b_p_fex);
#endif
#ifdef FEAT_CRYPT
    check_string_option(&buf->b_p_key);
#endif
    check_string_option(&buf->b_p_kp);
    check_string_option(&buf->b_p_mps);
    check_string_option(&buf->b_p_fo);
    check_string_option(&buf->b_p_flp);
    check_string_option(&buf->b_p_isk);
    check_string_option(&buf->b_p_com);
#ifdef FEAT_FOLDING
    check_string_option(&buf->b_p_cms);
#endif
    check_string_option(&buf->b_p_nf);
    check_string_option(&buf->b_p_qe);
#ifdef FEAT_SYN_HL
    check_string_option(&buf->b_p_syn);
    check_string_option(&buf->b_s.b_syn_isk);
#endif
#ifdef FEAT_SPELL
    check_string_option(&buf->b_s.b_p_spc);
    check_string_option(&buf->b_s.b_p_spf);
    check_string_option(&buf->b_s.b_p_spl);
    check_string_option(&buf->b_s.b_p_spo);
#endif
    check_string_option(&buf->b_p_sua);
    check_string_option(&buf->b_p_cink);
    check_string_option(&buf->b_p_cino);
    check_string_option(&buf->b_p_cinsd);
    parse_cino(buf);
    check_string_option(&buf->b_p_lop);
    check_string_option(&buf->b_p_ft);
    check_string_option(&buf->b_p_cinw);
    check_string_option(&buf->b_p_cpt);
#ifdef FEAT_COMPL_FUNC
    check_string_option(&buf->b_p_cfu);
    check_string_option(&buf->b_p_ofu);
    check_string_option(&buf->b_p_tsrfu);
#endif
#ifdef FEAT_EVAL
    check_string_option(&buf->b_p_tfu);
#endif
#ifdef FEAT_KEYMAP
    check_string_option(&buf->b_p_keymap);
#endif
#ifdef FEAT_QUICKFIX
    check_string_option(&buf->b_p_gp);
    check_string_option(&buf->b_p_mp);
    check_string_option(&buf->b_p_efm);
#endif
    check_string_option(&buf->b_p_ep);
    check_string_option(&buf->b_p_path);
    check_string_option(&buf->b_p_tags);
    check_string_option(&buf->b_p_tc);
    check_string_option(&buf->b_p_dict);
    check_string_option(&buf->b_p_tsr);
    check_string_option(&buf->b_p_lw);
    check_string_option(&buf->b_p_bkc);
    check_string_option(&buf->b_p_menc);
#ifdef FEAT_VARTABS
    check_string_option(&buf->b_p_vsts);
    check_string_option(&buf->b_p_vts);
#endif
}

/*
 * Free the string allocated for an option.
 * Checks for the string being empty_option. This may happen if we're out of
 * memory, vim_strsave() returned NULL, which was replaced by empty_option by
 * check_options().
 * Does NOT check for P_ALLOCED flag!
 */
    void
free_string_option(char_u *p)
{
    if (p != empty_option)
	vim_free(p);
}

    void
clear_string_option(char_u **pp)
{
    if (*pp != empty_option)
	vim_free(*pp);
    *pp = empty_option;
}

    void
check_string_option(char_u **pp)
{
    if (*pp == NULL)
	*pp = empty_option;
}

/*
 * Set global value for string option when it's a local option.
 */
    static void
set_string_option_global(
    int		opt_idx,	// option index
    char_u	**varp)		// pointer to option variable
{
    char_u	**p, *s;

    // the global value is always allocated
    if (is_window_local_option(opt_idx))
	p = (char_u **)GLOBAL_WO(varp);
    else
	p = (char_u **)get_option_var(opt_idx);
    if (!is_global_option(opt_idx)
	    && p != varp
	    && (s = vim_strsave(*varp)) != NULL)
    {
	free_string_option(*p);
	*p = s;
    }
}

/*
 * Set a string option to a new value (without checking the effect).
 * The string is copied into allocated memory.
 * if ("opt_idx" == -1) "name" is used, otherwise "opt_idx" is used.
 * When "set_sid" is zero set the scriptID to current_sctx.sc_sid.  When
 * "set_sid" is SID_NONE don't set the scriptID.  Otherwise set the scriptID to
 * "set_sid".
 */
    void
set_string_option_direct(
    char_u	*name,
    int		opt_idx,
    char_u	*val,
    int		opt_flags,	// OPT_FREE, OPT_LOCAL and/or OPT_GLOBAL
    int		set_sid UNUSED)
{
    char_u	*s;
    char_u	**varp;
    int		both = (opt_flags & (OPT_LOCAL | OPT_GLOBAL)) == 0;
    int		idx = opt_idx;

    if (idx == -1)		// use name
    {
	idx = findoption(name);
	if (idx < 0)	// not found (should not happen)
	{
	    semsg(_(e_internal_error_str), "set_string_option_direct()");
	    siemsg("For option %s", name);
	    return;
	}
    }

    if (is_hidden_option(idx))		// can't set hidden option
	return;

    s = vim_strsave(val);
    if (s == NULL)
	return;

    varp = (char_u **)get_option_varp_scope(idx,
	    both ? OPT_LOCAL : opt_flags);
    if ((opt_flags & OPT_FREE) && (get_option_flags(idx) & P_ALLOCED))
	free_string_option(*varp);
    *varp = s;

    // For buffer/window local option may also set the global value.
    if (both)
	set_string_option_global(idx, varp);

    set_option_flag(idx, P_ALLOCED);

    // When setting both values of a global option with a local value,
    // make the local value empty, so that the global value is used.
    if (is_global_local_option(idx) && both)
    {
	free_string_option(*varp);
	*varp = empty_option;
    }
# ifdef FEAT_EVAL
    if (set_sid != SID_NONE)
    {
	sctx_T script_ctx;

	if (set_sid == 0)
	    script_ctx = current_sctx;
	else
	{
	    script_ctx.sc_sid = set_sid;
	    script_ctx.sc_seq = 0;
	    script_ctx.sc_lnum = 0;
	    script_ctx.sc_version = 1;
	}
	set_option_sctx_idx(idx, opt_flags, script_ctx);
    }
# endif
}

#if defined(FEAT_PROP_POPUP) || \
    (defined(FEAT_DIFF) && defined(FEAT_FOLDING)) || defined(PROTO)
/*
 * Like set_string_option_direct(), but for a window-local option in "wp".
 * Blocks autocommands to avoid the old curwin becoming invalid.
 */
    void
set_string_option_direct_in_win(
	win_T		*wp,
	char_u		*name,
	int		opt_idx,
	char_u		*val,
	int		opt_flags,
	int		set_sid)
{
    win_T	*save_curwin = curwin;

    block_autocmds();
    curwin = wp;
    curbuf = curwin->w_buffer;
    set_string_option_direct(name, opt_idx, val, opt_flags, set_sid);
    curwin = save_curwin;
    curbuf = curwin->w_buffer;
    unblock_autocmds();
}
#endif

#if defined(FEAT_PROP_POPUP) || defined(PROTO)
/*
 * Like set_string_option_direct(), but for a buffer-local option in "buf".
 * Blocks autocommands to avoid the old curbuf becoming invalid.
 */
    void
set_string_option_direct_in_buf(
	buf_T		*buf,
	char_u		*name,
	int		opt_idx,
	char_u		*val,
	int		opt_flags,
	int		set_sid)
{
    buf_T	*save_curbuf = curbuf;

    block_autocmds();
    curbuf = buf;
    curwin->w_buffer = curbuf;
    set_string_option_direct(name, opt_idx, val, opt_flags, set_sid);
    curbuf = save_curbuf;
    curwin->w_buffer = curbuf;
    unblock_autocmds();
}
#endif

/*
 * Set a string option to a new value, and handle the effects.
 *
 * Returns NULL on success or an untranslated error message on error.
 */
    char *
set_string_option(
    int		opt_idx,
    char_u	*value,
    int		opt_flags,	// OPT_LOCAL and/or OPT_GLOBAL
    char	*errbuf,
    size_t	errbuflen)
{
    char_u	*s;
    char_u	**varp;
    char_u	*oldval;
#if defined(FEAT_EVAL)
    char_u	*oldval_l = NULL;
    char_u	*oldval_g = NULL;
    char_u	*saved_oldval = NULL;
    char_u	*saved_oldval_l = NULL;
    char_u	*saved_oldval_g = NULL;
    char_u	*saved_newval = NULL;
#endif
    char	*errmsg = NULL;
    int		value_checked = FALSE;

    if (is_hidden_option(opt_idx))	// don't set hidden option
	return NULL;

    s = vim_strsave(value == NULL ? (char_u *)"" : value);
    if (s == NULL)
	return NULL;

    varp = (char_u **)get_option_varp_scope(opt_idx,
	    (opt_flags & (OPT_LOCAL | OPT_GLOBAL)) == 0
	    ? (is_global_local_option(opt_idx)
		? OPT_GLOBAL : OPT_LOCAL)
	    : opt_flags);
    oldval = *varp;
#if defined(FEAT_EVAL)
    if ((opt_flags & (OPT_LOCAL | OPT_GLOBAL)) == 0)
    {
	oldval_l = *(char_u **)get_option_varp_scope(opt_idx, OPT_LOCAL);
	oldval_g = *(char_u **)get_option_varp_scope(opt_idx, OPT_GLOBAL);
    }
#endif
    *varp = s;

#if defined(FEAT_EVAL)
    if (!starting
# ifdef FEAT_CRYPT
	    && !is_crypt_key_option(opt_idx)
# endif
       )
    {
	if (oldval_l != NULL)
	    saved_oldval_l = vim_strsave(oldval_l);
	if (oldval_g != NULL)
	    saved_oldval_g = vim_strsave(oldval_g);
	saved_oldval = vim_strsave(oldval);
	saved_newval = vim_strsave(s);
    }
#endif
    if ((errmsg = did_set_string_option(opt_idx, varp, oldval, value, errbuf,
		    errbuflen, opt_flags, OP_NONE, &value_checked)) == NULL)
	did_set_option(opt_idx, opt_flags, TRUE, value_checked);

#if defined(FEAT_EVAL)
    // call autocommand after handling side effects
    if (errmsg == NULL)
	trigger_optionset_string(opt_idx, opt_flags,
		saved_oldval, saved_oldval_l,
		saved_oldval_g, saved_newval);
    vim_free(saved_oldval);
    vim_free(saved_oldval_l);
    vim_free(saved_oldval_g);
    vim_free(saved_newval);
#endif
    return errmsg;
}

/*
 * Return TRUE if "val" is a valid 'filetype' name.
 * Also used for 'syntax' and 'keymap'.
 */
    static int
valid_filetype(char_u *val)
{
    return valid_name(val, ".-_");
}

#ifdef FEAT_STL_OPT
/*
 * Check validity of options with the 'statusline' format.
 * Return an untranslated error message or NULL.
 */
    static char *
check_stl_option(char_u *s)
{
    int		groupdepth = 0;
    static char errbuf[ERR_BUFLEN];
    int		errbuflen = ERR_BUFLEN;

    while (*s)
    {
	// Check for valid keys after % sequences
	while (*s && *s != '%')
	    s++;
	if (!*s)
	    break;
	s++;
	if (*s == '%' || *s == STL_TRUNCMARK || *s == STL_SEPARATE)
	{
	    s++;
	    continue;
	}
	if (*s == ')')
	{
	    s++;
	    if (--groupdepth < 0)
		break;
	    continue;
	}
	if (*s == '-')
	    s++;
	while (VIM_ISDIGIT(*s))
	    s++;
	if (*s == STL_USER_HL)
	    continue;
	if (*s == '.')
	{
	    s++;
	    while (*s && VIM_ISDIGIT(*s))
		s++;
	}
	if (*s == '(')
	{
	    groupdepth++;
	    continue;
	}
	if (vim_strchr(STL_ALL, *s) == NULL)
	{
	    return illegal_char(errbuf, errbuflen, *s);
	}
	if (*s == '{')
	{
	    int reevaluate = (*++s == '%');

	    if (reevaluate && *++s == '}')
		// "}" is not allowed immediately after "%{%"
		return illegal_char(errbuf, errbuflen, '}');
	    while ((*s != '}' || (reevaluate && s[-1] != '%')) && *s)
		s++;
	    if (*s != '}')
		return e_unclosed_expression_sequence;
	}
    }
    if (groupdepth != 0)
	return e_unbalanced_groups;
    return NULL;
}
#endif

/*
 * Check for a "normal" directory or file name in some options.  Disallow a
 * path separator (slash and/or backslash), wildcards and characters that are
 * often illegal in a file name. Be more permissive if "secure" is off.
 */
    static int
check_illegal_path_names(int opt_idx, char_u **varp)
{
    return (((get_option_flags(opt_idx) & P_NFNAME)
		&& vim_strpbrk(*varp, (char_u *)(secure
			? "/\\*?[|;&<>\r\n" : "/\\*?[<>\r\n")) != NULL)
	    || ((get_option_flags(opt_idx) & P_NDNAME)
		&& vim_strpbrk(*varp, (char_u *)"*?[|;&<>\r\n") != NULL));
}

/*
 * An option that accepts a list of flags is changed.
 * e.g. 'viewoptions', 'switchbuf', 'casemap', etc.
 */
    static char *
did_set_opt_flags(char_u *val, char **values, unsigned *flagp, int list)
{
    if (opt_strings_flags(val, values, flagp, list) == FAIL)
	return e_invalid_argument;

    return NULL;
}

/*
 * An option that accepts a list of string values is changed.
 * e.g. 'nrformats', 'scrollopt', 'wildoptions', etc.
 */
    static char *
did_set_opt_strings(char_u *val, char **values, int list)
{
    return did_set_opt_flags(val, values, NULL, list);
}

/*
 * An option which is a list of flags is set.  Valid values are in 'flags'.
 */
    static char *
did_set_option_listflag(
	char_u *val,
	char_u *flags,
	char *errbuf,
	size_t errbuflen)
{
    char_u	*s;

    for (s = val; *s; ++s)
	if (vim_strchr(flags, *s) == NULL)
	    return illegal_char(errbuf, errbuflen, *s);

    return NULL;
}

/*
 * Expand an option that accepts a list of fixed string values with known
 * number of items.
 */
    static int
expand_set_opt_string(
	optexpand_T *args,
	char **values,
	size_t numValues,
	int *numMatches,
	char_u ***matches)
{
    char_u	*p;
    regmatch_T	*regmatch = args->oe_regmatch;
    int		include_orig_val = args->oe_include_orig_val;
    char_u	*option_val = args->oe_opt_value;

    // Assume numValues is small since they are fixed enums, so just allocate
    // upfront instead of needing two passes to calculate output size.
    *matches = ALLOC_MULT(char_u *, numValues + 1);
    if (*matches == NULL)
	return FAIL;

    int count = 0;

    if (include_orig_val && *option_val != NUL)
    {
	p = vim_strsave(option_val);
	if (p == NULL)
	{
	    VIM_CLEAR(*matches);
	    return FAIL;
	}
	(*matches)[count++] = p;
    }

    for (char **val = values; *val != NULL; val++)
    {
	if (include_orig_val && *option_val != NUL)
	{
	    if (STRCMP((char_u*)*val, option_val) == 0)
		continue;
	}
	if (vim_regexec(regmatch, (char_u*)(*val), (colnr_T)0))
	{
	    p = vim_strsave((char_u*)*val);
	    if (p == NULL)
	    {
		if (count == 0)
		{
		    VIM_CLEAR(*matches);
		    return FAIL;
		}
		else
		    break;
	    }
	    (*matches)[count++] = p;
	}
    }
    if (count == 0)
    {
	VIM_CLEAR(*matches);
	return FAIL;
    }
    *numMatches = count;
    return OK;
}

static char_u *set_opt_callback_orig_option = NULL;
static char_u *((*set_opt_callback_func)(expand_T *, int));

/*
 * Callback used by expand_set_opt_generic to also include the original value
 * as the first item.
 */
    static char_u *
expand_set_opt_generic_cb(expand_T *xp, int idx)
{
    if (idx == 0)
    {
	if (set_opt_callback_orig_option != NULL)
	    return set_opt_callback_orig_option;
	else
	    return (char_u *)""; // empty strings are ignored
    }
    return set_opt_callback_func(xp, idx - 1);
}

/*
 * Expand an option with a callback that iterates through a list of possible
 * names using an index.
 */
    static int
expand_set_opt_generic(
	optexpand_T *args,
	char_u *((*func)(expand_T *, int)),
	int *numMatches,
	char_u ***matches)
{
    int ret;

    set_opt_callback_orig_option = args->oe_include_orig_val ?
	args->oe_opt_value : NULL;
    set_opt_callback_func = func;

    ret = ExpandGeneric(
	    (char_u*)"", // not using fuzzy as currently EXPAND_STRING_SETTING doesn't use it
	    args->oe_xp,
	    args->oe_regmatch,
	    matches,
	    numMatches,
	    expand_set_opt_generic_cb,
	    FALSE);

    set_opt_callback_orig_option = NULL;
    set_opt_callback_func = NULL;
    return ret;
}

# if defined(FEAT_GUI_MSWIN) || defined(FEAT_GUI_GTK)
static garray_T *expand_cb_ga;
static optexpand_T *expand_cb_args;

/*
 * Callback provided to a function in expand_set_opt_callback. Will perform
 * regex matching against the value and add to the list.
 *
 * Returns OK usually. Returns FAIL if it failed to allocate memory, and the
 * caller should terminate the enumeration.
 */
    static int
expand_set_opt_callback_cb(char_u *val)
{
    regmatch_T	*regmatch = expand_cb_args->oe_regmatch;
    expand_T	*xp = expand_cb_args->oe_xp;
    garray_T	*ga = expand_cb_ga;
    char_u	*str;

    if (val == NULL || *val == NUL)
	return OK;

    if (xp->xp_pattern[0] != NUL &&
	    !vim_regexec(regmatch, val, (colnr_T)0))
	return OK;

    str = vim_strsave_escaped(val, (char_u *)" \t\\");

    if (str == NULL)
	return FAIL;

    if (ga_grow(ga, 1) == FAIL)
    {
	vim_free(str);
	return FAIL;
    }

    ((char_u **)ga->ga_data)[ga->ga_len] = str;
    ++ga->ga_len;
    return OK;
}

/*
 * Expand an option with a provided function that takes a callback. The
 * function will enumerate through all options and call the callback to add it
 * to the list.
 *
 * "func" is the enumerator function that will generate the list of options.
 * "func_params" is a single parameter that will be passed to func.
 */
    static int
expand_set_opt_callback(
	optexpand_T *args,
	void (*func)(optexpand_T *, void* params, int (*cb)(char_u *val)),
	void *func_params,
	int *numMatches,
	char_u ***matches)
{
    garray_T	ga;
    int		include_orig_val = args->oe_include_orig_val;
    char_u	*option_val = args->oe_opt_value;

    ga_init2(&ga, sizeof(char *), 30);

    if (include_orig_val && *option_val != NUL)
    {
	char_u *p = vim_strsave(option_val);
	if (p == NULL)
	    return FAIL;
	if (ga_grow(&ga, 1) == FAIL)
	{
	    vim_free(p);
	    return FAIL;
	}
	((char_u **)ga.ga_data)[ga.ga_len] = p;
	++ga.ga_len;
    }

    expand_cb_ga = &ga;
    expand_cb_args = args;

    func(args, func_params, expand_set_opt_callback_cb);

    expand_cb_ga = NULL;
    expand_cb_args = NULL;

    *matches = ga.ga_data;
    *numMatches = ga.ga_len;
    return OK;
}
#endif

/*
 * Expand an option which is a list of flags.
 */
    static int
expand_set_opt_listflag(
	optexpand_T *args,
	char_u *flags,
	int *numMatches,
	char_u ***matches)
{
    char_u  *p;
    char_u  *option_val = args->oe_opt_value;
    char_u  *cmdline_val = args->oe_set_arg;
    int	    append = args->oe_append;
    int	    include_orig_val = args->oe_include_orig_val && (*option_val != NUL);

    size_t num_flags = STRLEN(flags);

    // Assume we only have small number of flags, so just allocate max size.
    *matches = ALLOC_MULT(char_u *, num_flags + 1);
    if (*matches == NULL)
	return FAIL;

    int count = 0;

    if (include_orig_val)
    {
	p = vim_strsave(option_val);
	if (p == NULL)
	{
	    VIM_CLEAR(*matches);
	    return FAIL;
	}
	(*matches)[count++] = p;
    }

    for (char_u *flag = flags; *flag != NUL; flag++)
    {
	if (append && vim_strchr(option_val, *flag) != NULL)
	    continue;

	if (vim_strchr(cmdline_val, *flag) == NULL)
	{
	    if (include_orig_val
		    && option_val[1] == NUL
		    && *flag == option_val[0])
	    {
		// This value is already used as the first choice as it's the
		// existing flag. Just skip it to avoid duplicate.
		continue;
	    }
	    p = vim_strnsave(flag, 1);
	    if (p == NULL)
	    {
		if (count == 0)
		{
		    VIM_CLEAR(*matches);
		    return FAIL;
		}
		else
		    break;
	    }
	    (*matches)[count++] = p;
	}
    }

    if (count == 0)
    {
	VIM_CLEAR(*matches);
	return FAIL;
    }
    *numMatches = count;
    return OK;
}

/*
 * The 'ambiwidth' option is changed.
 */
    char *
did_set_ambiwidth(optset_T *args UNUSED)
{
    if (check_opt_strings(p_ambw, p_ambw_values, FALSE) != OK)
	return e_invalid_argument;

    return check_chars_options();
}

    int
expand_set_ambiwidth(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_ambw_values,
	    ARRAY_LENGTH(p_ambw_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'background' option is changed.
 */
    char *
did_set_background(optset_T *args)
{
    if (check_opt_strings(p_bg, p_bg_values, FALSE) == FAIL)
	return e_invalid_argument;

    if (args->os_oldval.string != NULL && args->os_oldval.string[0] == *p_bg)
	// Value was not changed
	return NULL;

#ifdef FEAT_EVAL
    int dark = (*p_bg == 'd');
#endif

    init_highlight(FALSE, FALSE);

#ifdef FEAT_EVAL
    if (dark != (*p_bg == 'd')
	    && get_var_value((char_u *)"g:colors_name") != NULL)
    {
	// The color scheme must have set 'background' back to another
	// value, that's not what we want here.  Disable the color
	// scheme and set the colors again.
	do_unlet((char_u *)"g:colors_name", TRUE);
	free_string_option(p_bg);
	p_bg = vim_strsave((char_u *)(dark ? "dark" : "light"));
	check_string_option(&p_bg);
	init_highlight(FALSE, FALSE);
    }
#endif
#ifdef FEAT_TERMINAL
    term_update_colors_all();
#endif

    return NULL;
}

    int
expand_set_background(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_bg_values,
	    ARRAY_LENGTH(p_bg_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'backspace' option is changed.
 */
    char *
did_set_backspace(optset_T *args UNUSED)
{
    if (VIM_ISDIGIT(*p_bs))
    {
	if (*p_bs > '3' || p_bs[1] != NUL)
	    return e_invalid_argument;
    }
    else if (check_opt_strings(p_bs, p_bs_values, TRUE) != OK)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_backspace(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_bs_values,
	    ARRAY_LENGTH(p_bs_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'backupcopy' option is changed.
 */
    char *
did_set_backupcopy(optset_T *args)
{
    char_u		*bkc = p_bkc;
    unsigned int	*flags = &bkc_flags;
    char		*errmsg = NULL;

    if (args->os_flags & OPT_LOCAL)
    {
	bkc = curbuf->b_p_bkc;
	flags = &curbuf->b_bkc_flags;
    }

    if ((args->os_flags & OPT_LOCAL) && *bkc == NUL)
	// make the local value empty: use the global value
	*flags = 0;
    else
    {
	if (opt_strings_flags(bkc, p_bkc_values, flags, TRUE) != OK)
	    errmsg = e_invalid_argument;
	if ((((int)*flags & BKC_AUTO) != 0)
		+ (((int)*flags & BKC_YES) != 0)
		+ (((int)*flags & BKC_NO) != 0) != 1)
	{
	    // Must have exactly one of "auto", "yes"  and "no".
	    (void)opt_strings_flags(args->os_oldval.string, p_bkc_values,
								  flags, TRUE);
	    errmsg = e_invalid_argument;
	}
    }

    return errmsg;
}

    int
expand_set_backupcopy(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_bkc_values,
	    ARRAY_LENGTH(p_bkc_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'backupext' or the 'patchmode' option is changed.
 */
    char *
did_set_backupext_or_patchmode(optset_T *args UNUSED)
{
    if (STRCMP(*p_bex == '.' ? p_bex + 1 : p_bex,
		*p_pm == '.' ? p_pm + 1 : p_pm) == 0)
	return e_backupext_and_patchmode_are_equal;

    return NULL;
}

/*
 * The 'belloff' option is changed.
 */
    char *
did_set_belloff(optset_T *args UNUSED)
{
    return did_set_opt_flags(p_bo, p_bo_values, &bo_flags, TRUE);
}

    int
expand_set_belloff(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_bo_values,
	    ARRAY_LENGTH(p_bo_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_LINEBREAK) || defined(PROTO)
/*
 * The 'breakindentopt' option is changed.
 */
    char *
did_set_breakindentopt(optset_T *args UNUSED)
{
    char *errmsg = NULL;

    if (briopt_check(curwin) == FAIL)
	errmsg = e_invalid_argument;
    // list setting requires a redraw
    if (curwin->w_briopt_list)
	redraw_all_later(UPD_NOT_VALID);

    return errmsg;
}

    int
expand_set_breakindentopt(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_briopt_values,
	    ARRAY_LENGTH(p_briopt_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if defined(FEAT_BROWSE) || defined(PROTO)
/*
 * The 'browsedir' option is changed.
 */
    char *
did_set_browsedir(optset_T *args UNUSED)
{
    if (check_opt_strings(p_bsdir, p_bsdir_values, FALSE) != OK
	    && !mch_isdir(p_bsdir))
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_browsedir(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_bsdir_values,
	    ARRAY_LENGTH(p_bsdir_values) - 1,
	    numMatches,
	    matches);
}
#endif

/*
 * The 'bufhidden' option is changed.
 */
    char *
did_set_bufhidden(optset_T *args UNUSED)
{
    return did_set_opt_strings(curbuf->b_p_bh, p_bufhidden_values, FALSE);
}

    int
expand_set_bufhidden(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_bufhidden_values,
	    ARRAY_LENGTH(p_bufhidden_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'buftype' option is changed.
 */
    char *
did_set_buftype(optset_T *args UNUSED)
{
    if (check_opt_strings(curbuf->b_p_bt, p_buftype_values, FALSE) != OK)
	return e_invalid_argument;

    if (curwin->w_status_height)
    {
	curwin->w_redr_status = TRUE;
	redraw_later(UPD_VALID);
    }
    curbuf->b_help = (curbuf->b_p_bt[0] == 'h');
    redraw_titles();

    return NULL;
}

    int
expand_set_buftype(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_buftype_values,
	    ARRAY_LENGTH(p_buftype_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'casemap' option is changed.
 */
    char *
did_set_casemap(optset_T *args UNUSED)
{
    return did_set_opt_flags(p_cmp, p_cmp_values, &cmp_flags, TRUE);
}

    int
expand_set_casemap(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_cmp_values,
	    ARRAY_LENGTH(p_cmp_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_CLIPBOARD) || defined(PROTO)
    int
expand_set_clipboard(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_cb_values,
	    ARRAY_LENGTH(p_cb_values) - 1,
	    numMatches,
	    matches);
}
#endif

/*
 * The global 'listchars' or 'fillchars' option is changed.
 */
    static char *
did_set_global_listfillchars(char_u *val, int opt_lcs, int opt_flags,
						char *errbuf, size_t errbuflen)
{
    char	*errmsg = NULL;
    char_u	**local_ptr = opt_lcs ? &curwin->w_p_lcs : &curwin->w_p_fcs;

    // only apply the global value to "curwin" when it does not have a
    // local value
    if (opt_lcs)
	errmsg = set_listchars_option(curwin, val,
		**local_ptr == NUL || !(opt_flags & OPT_GLOBAL),
							    errbuf, errbuflen);
    else
	errmsg = set_fillchars_option(curwin, val,
		**local_ptr == NUL || !(opt_flags & OPT_GLOBAL),
							    errbuf, errbuflen);
    if (errmsg != NULL)
	return errmsg;

    tabpage_T	*tp;
    win_T	*wp;

    // If the current window is set to use the global
    // 'listchars'/'fillchars' value, clear the window-local value.
    if (!(opt_flags & OPT_GLOBAL))
	clear_string_option(local_ptr);
    FOR_ALL_TAB_WINDOWS(tp, wp)
    {
	// If the current window has a local value need to apply it
	// again, it was changed when setting the global value.
	// If no error was returned above, we don't expect an error
	// here, so ignore the return value.
	if (opt_lcs)
	{
	    if (*wp->w_p_lcs == NUL)
		(void)set_listchars_option(wp, wp->w_p_lcs, TRUE, NULL, 0);
	}
	else
	{
	    if (*wp->w_p_fcs == NUL)
		(void)set_fillchars_option(wp, wp->w_p_fcs, TRUE, NULL, 0);
	}
    }

    redraw_all_later(UPD_NOT_VALID);

    return NULL;
}

/*
 * The 'fillchars' option or the 'listchars' option is changed.
 */
    char *
did_set_chars_option(optset_T *args)
{
    char_u **varp = (char_u **)args->os_varp;
    char *errmsg = NULL;

    if (   varp == &p_lcs		// global 'listchars'
	|| varp == &p_fcs)		// global 'fillchars'
	errmsg = did_set_global_listfillchars(*varp, varp == &p_lcs,
			  args->os_flags, args->os_errbuf, args->os_errbuflen);
    else if (varp == &curwin->w_p_lcs)	// local 'listchars'
	errmsg = set_listchars_option(curwin, *varp, TRUE,
					  args->os_errbuf, args->os_errbuflen);
    else if (varp == &curwin->w_p_fcs)	// local 'fillchars'
	errmsg = set_fillchars_option(curwin, *varp, TRUE,
					  args->os_errbuf, args->os_errbuflen);

    return errmsg;
}

/*
 * Expand 'fillchars' or 'listchars' option value.
 */
    int
expand_set_chars_option(optexpand_T *args, int *numMatches, char_u ***matches)
{
    char_u **varp = (char_u **)args->oe_varp;
    int is_lcs = (varp == &p_lcs || varp == &curwin->w_p_lcs);
    return expand_set_opt_generic(
	    args,
	    is_lcs ? get_listchars_name : get_fillchars_name,
	    numMatches,
	    matches);
}

/*
 * The 'cinoptions' option is changed.
 */
    char *
did_set_cinoptions(optset_T *args UNUSED)
{
    // TODO: recognize errors
    parse_cino(curbuf);

    return NULL;
}

#if defined(FEAT_SYN_HL) || defined(PROTO)
/*
 * The 'colorcolumn' option is changed.
 */
    char *
did_set_colorcolumn(optset_T *args UNUSED)
{
    return check_colorcolumn(curwin);
}
#endif

/*
 * The 'comments' option is changed.
 */
    char *
did_set_comments(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char_u	*s;
    char	*errmsg = NULL;

    for (s = *varp; *s; )
    {
	while (*s && *s != ':')
	{
	    if (vim_strchr((char_u *)COM_ALL, *s) == NULL
		    && !VIM_ISDIGIT(*s) && *s != '-')
	    {
		errmsg = illegal_char(args->os_errbuf, args->os_errbuflen, *s);
		break;
	    }
	    ++s;
	}
	if (*s++ == NUL)
	    errmsg = e_missing_colon;
	else if (*s == ',' || *s == NUL)
	    errmsg = e_zero_length_string;
	if (errmsg != NULL)
	    break;
	while (*s && *s != ',')
	{
	    if (*s == '\\' && s[1] != NUL)
		++s;
	    ++s;
	}
	s = skip_to_option_part(s);
    }

    return errmsg;
}

#if defined(FEAT_FOLDING) || defined(PROTO)
/*
 * The 'commentstring' option is changed.
 */
    char *
did_set_commentstring(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (**varp != NUL && strstr((char *)*varp, "%s") == NULL)
	return e_commentstring_must_be_empty_or_contain_str;

    return NULL;
}
#endif

/*
 * The 'complete' option is changed.
 */
    char *
did_set_complete(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char_u	*s;

    // check if it is a valid value for 'complete' -- Acevedo
    for (s = *varp; *s;)
    {
	while (*s == ',' || *s == ' ')
	    s++;
	if (!*s)
	    break;
	if (vim_strchr((char_u *)".wbuksid]tU", *s) == NULL)
	    return illegal_char(args->os_errbuf, args->os_errbuflen, *s);
	if (*++s != NUL && *s != ',' && *s != ' ')
	{
	    if (s[-1] == 'k' || s[-1] == 's')
	    {
		// skip optional filename after 'k' and 's'
		while (*s && *s != ',' && *s != ' ')
		{
		    if (*s == '\\' && s[1] != NUL)
			++s;
		    ++s;
		}
	    }
	    else
	    {
		if (args->os_errbuf != NULL)
		{
		    vim_snprintf((char *)args->os_errbuf, args->os_errbuflen,
			    _(e_illegal_character_after_chr), *--s);
		    return args->os_errbuf;
		}
		return "";
	    }
	}
    }

    return NULL;
}

    int
expand_set_complete(optexpand_T *args, int *numMatches, char_u ***matches)
{
    static char *(p_cpt_values[]) = {
	".", "w", "b", "u", "k", "kspell", "s", "i", "d", "]", "t", "U",
	NULL};
    return expand_set_opt_string(
	    args,
	    p_cpt_values,
	    ARRAY_LENGTH(p_cpt_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'completeopt' option is changed.
 */
    char *
did_set_completeopt(optset_T *args UNUSED)
{
    if (check_opt_strings(p_cot, p_cot_values, TRUE) != OK)
	return e_invalid_argument;

    completeopt_was_set();
    return NULL;
}

    int
expand_set_completeopt(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_cot_values,
	    ARRAY_LENGTH(p_cot_values) - 1,
	    numMatches,
	    matches);
}

#if (defined(FEAT_PROP_POPUP) && defined(FEAT_QUICKFIX)) || defined(PROTO)
/*
 * The 'completepopup' option is changed.
 */
    char *
did_set_completepopup(optset_T *args UNUSED)
{
    if (parse_completepopup(NULL) == FAIL)
	return e_invalid_argument;

    popup_close_info();
    return NULL;
}
#endif

#if defined(BACKSLASH_IN_FILENAME) || defined(PROTO)
/*
 * The 'completeslash' option is changed.
 */
    char *
did_set_completeslash(optset_T *args UNUSED)
{
    if (check_opt_strings(p_csl, p_csl_values, FALSE) != OK
	    || check_opt_strings(curbuf->b_p_csl, p_csl_values, FALSE) != OK)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_completeslash(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_csl_values,
	    ARRAY_LENGTH(p_csl_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if defined(FEAT_CONCEAL) || defined(PROTO)
/*
 * The 'concealcursor' option is changed.
 */
    char *
did_set_concealcursor(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    return did_set_option_listflag(*varp, (char_u *)COCU_ALL, args->os_errbuf,
		    args->os_errbuflen);
}

    int
expand_set_concealcursor(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_listflag(args, (char_u*)COCU_ALL, numMatches, matches);
}
#endif

/*
 * The 'cpoptions' option is changed.
 */
    char *
did_set_cpoptions(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    return did_set_option_listflag(*varp, (char_u *)CPO_ALL, args->os_errbuf,
		    args->os_errbuflen);
}

    int
expand_set_cpoptions(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_listflag(args, (char_u*)CPO_ALL, numMatches, matches);
}

#if defined(FEAT_CRYPT) || defined(PROTO)
/*
 * The 'cryptkey' option is changed.
 */
    char *
did_set_cryptkey(optset_T *args)
{
    // Make sure the ":set" command doesn't show the new value in the
    // history.
    remove_key_from_history();

    if (args->os_op != OP_NONE)
	// Don't allow set+=/-=/^= as they can allow for substring guessing
	return e_invalid_argument;

    if (STRCMP(curbuf->b_p_key, args->os_oldval.string) != 0)
    {
	// Need to update the swapfile.
	ml_set_crypt_key(curbuf, args->os_oldval.string,
		*curbuf->b_p_cm == NUL ? p_cm : curbuf->b_p_cm);
	changed_internal();
    }
# ifdef FEAT_SODIUM
    if (crypt_method_is_sodium(crypt_get_method_nr(curbuf)))
       crypt_sodium_lock_key(args->os_newval.string);
# endif

    return NULL;
}

/*
 * The 'cryptmethod' option is changed.
 */
    char *
did_set_cryptmethod(optset_T *args)
{
    char_u  *p;
    char_u  *s;

    if (args->os_flags & OPT_LOCAL)
	p = curbuf->b_p_cm;
    else
	p = p_cm;
    if (check_opt_strings(p, p_cm_values, TRUE) != OK)
	return e_invalid_argument;
    else if (crypt_self_test() == FAIL)
	return e_invalid_argument;

    // When setting the global value to empty, make it "zip".
    if (*p_cm == NUL)
    {
	free_string_option(p_cm);
	p_cm = vim_strsave((char_u *)"zip");
    }
    // When using ":set cm=name" the local value is going to be empty.
    // Do that here, otherwise the crypt functions will still use the
    // local value.
    if ((args->os_flags & (OPT_LOCAL | OPT_GLOBAL)) == 0)
    {
	free_string_option(curbuf->b_p_cm);
	curbuf->b_p_cm = empty_option;
    }

    // Need to update the swapfile when the effective method changed.
    // Set "s" to the effective old value, "p" to the effective new
    // method and compare.
    if ((args->os_flags & OPT_LOCAL) && *args->os_oldval.string == NUL)
	s = p_cm;  // was previously using the global value
    else
	s = args->os_oldval.string;
    if (*curbuf->b_p_cm == NUL)
	p = p_cm;  // is now using the global value
    else
	p = curbuf->b_p_cm;
    if (STRCMP(s, p) != 0)
	ml_set_crypt_key(curbuf, curbuf->b_p_key, s);

    // If the global value changes need to update the swapfile for all
    // buffers using that value.
    if ((args->os_flags & OPT_GLOBAL)
	    && STRCMP(p_cm, args->os_oldval.string) != 0)
    {
	buf_T	*buf;

	FOR_ALL_BUFFERS(buf)
	    if (buf != curbuf && *buf->b_p_cm == NUL)
		ml_set_crypt_key(buf, buf->b_p_key, args->os_oldval.string);
    }
    return NULL;
}

    int
expand_set_cryptmethod(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_cm_values,
	    ARRAY_LENGTH(p_cm_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if (defined(FEAT_CSCOPE) && defined(FEAT_QUICKFIX)) || defined(PROTO)
/*
 * The 'cscopequickfix' option is changed.
 */
    char *
did_set_cscopequickfix(optset_T *args UNUSED)
{
    char_u	*p;

    if (p_csqf == NULL)
	return NULL;

    p = p_csqf;
    while (*p != NUL)
    {
	if (vim_strchr((char_u *)CSQF_CMDS, *p) == NULL
		|| p[1] == NUL
		|| vim_strchr((char_u *)CSQF_FLAGS, p[1]) == NULL
		|| (p[2] != NUL && p[2] != ','))
	    return e_invalid_argument;
	else if (p[2] == NUL)
	    break;
	else
	    p += 3;
    }

    return NULL;
}
#endif

#if defined(FEAT_SYN_HL) || defined(PROTO)
/*
 * The 'cursorlineopt' option is changed.
 */
    char *
did_set_cursorlineopt(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    // This could be changed to use opt_strings_flags() instead.
    if (**varp == NUL || fill_culopt_flags(*varp, curwin) != OK)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_cursorlineopt(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_culopt_values,
	    ARRAY_LENGTH(p_culopt_values) - 1,
	    numMatches,
	    matches);
}
#endif

/*
 * The 'debug' option is changed.
 */
    char *
did_set_debug(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_debug, p_debug_values, TRUE);
}

    int
expand_set_debug(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_debug_values,
	    ARRAY_LENGTH(p_debug_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_DIFF) || defined(PROTO)
/*
 * The 'diffopt' option is changed.
 */
    char *
did_set_diffopt(optset_T *args UNUSED)
{
    if (diffopt_changed() == FAIL)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_diffopt(optexpand_T *args, int *numMatches, char_u ***matches)
{
    expand_T *xp = args->oe_xp;

    if (xp->xp_pattern > args->oe_set_arg && *(xp->xp_pattern-1) == ':')
    {
	// Within "algorithm:", we have a subgroup of possible options.
	int algo_len = (int)STRLEN("algorithm:");
	if (xp->xp_pattern - args->oe_set_arg >= algo_len &&
		STRNCMP(xp->xp_pattern - algo_len, "algorithm:", algo_len) == 0)
	{
	    return expand_set_opt_string(
		    args,
		    p_dip_algorithm_values,
		    ARRAY_LENGTH(p_dip_algorithm_values) - 1,
		    numMatches,
		    matches);
	}
	return FAIL;
    }

    return expand_set_opt_string(
	    args,
	    p_dip_values,
	    ARRAY_LENGTH(p_dip_values) - 1,
	    numMatches,
	    matches);
}
#endif

/*
 * The 'display' option is changed.
 */
    char *
did_set_display(optset_T *args UNUSED)
{
    if (opt_strings_flags(p_dy, p_dy_values, &dy_flags, TRUE) != OK)
	return e_invalid_argument;

    (void)init_chartab();
    return NULL;
}

    int
expand_set_display(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_dy_values,
	    ARRAY_LENGTH(p_dy_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'eadirection' option is changed.
 */
    char *
did_set_eadirection(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_ead, p_ead_values, FALSE);
}

    int
expand_set_eadirection(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_ead_values,
	    ARRAY_LENGTH(p_ead_values) - 1,
	    numMatches,
	    matches);
}

/*
 * One of the 'encoding', 'fileencoding', 'termencoding' or 'makeencoding'
 * options is changed.
 */
    char *
did_set_encoding(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char_u	**gvarp;
    char	*errmsg = NULL;
    char_u	*p;

    // Get the global option to compare with, otherwise we would have to check
    // two values for all local options.
    gvarp = (char_u **)get_option_varp_scope(args->os_idx, OPT_GLOBAL);

    if (gvarp == &p_fenc)
    {
	if (!curbuf->b_p_ma && args->os_flags != OPT_GLOBAL)
	    errmsg = e_cannot_make_changes_modifiable_is_off;
	else if (vim_strchr(*varp, ',') != NULL)
	    // No comma allowed in 'fileencoding'; catches confusing it
	    // with 'fileencodings'.
	    errmsg = e_invalid_argument;
	else
	{
	    // May show a "+" in the title now.
	    redraw_titles();
	    // Add 'fileencoding' to the swap file.
	    ml_setflags(curbuf);
	}
    }
    if (errmsg == NULL)
    {
	// canonize the value, so that STRCMP() can be used on it
	p = enc_canonize(*varp);
	if (p != NULL)
	{
	    vim_free(*varp);
	    *varp = p;
	}
	if (varp == &p_enc)
	{
	    errmsg = mb_init();
	    redraw_titles();
	}
    }

#if defined(FEAT_GUI_GTK)
    if (errmsg == NULL && varp == &p_tenc && gui.in_use)
    {
	// GTK uses only a single encoding, and that is UTF-8.
	if (STRCMP(p_tenc, "utf-8") != 0)
	    errmsg = e_cannot_be_changed_in_gtk_GUI;
    }
#endif

    if (errmsg == NULL)
    {
#ifdef FEAT_KEYMAP
	// When 'keymap' is used and 'encoding' changes, reload the keymap
	// (with another encoding).
	if (varp == &p_enc && *curbuf->b_p_keymap != NUL)
	    (void)keymap_init();
#endif

	// When 'termencoding' is not empty and 'encoding' changes or when
	// 'termencoding' changes, need to setup for keyboard input and
	// display output conversion.
	if (((varp == &p_enc && *p_tenc != NUL) || varp == &p_tenc))
	{
	    if (convert_setup(&input_conv, p_tenc, p_enc) == FAIL
		    || convert_setup(&output_conv, p_enc, p_tenc) == FAIL)
	    {
		semsg(_(e_cannot_convert_between_str_and_str),
			p_tenc, p_enc);
		errmsg = e_invalid_argument;
	    }
	}

#if defined(MSWIN)
	// $HOME, $VIM and $VIMRUNTIME may have characters in active code page.
	if (varp == &p_enc)
	{
	    init_homedir();
	    init_vimdir();
	}
#endif
    }

    return errmsg;
}

    int
expand_set_encoding(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_generic(
	    args,
	    get_encoding_name,
	    numMatches,
	    matches);
}

/*
 * The 'eventignore' option is changed.
 */
    char *
did_set_eventignore(optset_T *args UNUSED)
{
    if (check_ei() == FAIL)
	return e_invalid_argument;
    return NULL;
}

    static char_u *
get_eventignore_name(expand_T *xp, int idx)
{
    // 'eventignore' allows special keyword "all" in addition to
    // all event names.
    if (idx == 0)
	return (char_u *)"all";
    return get_event_name_no_group(xp, idx - 1);
}

    int
expand_set_eventignore(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_generic(
	    args,
	    get_eventignore_name,
	    numMatches,
	    matches);
}

/*
 * The 'fileformat' option is changed.
 */
    char *
did_set_fileformat(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (!curbuf->b_p_ma && !(args->os_flags & OPT_GLOBAL))
	return e_cannot_make_changes_modifiable_is_off;
    else if (check_opt_strings(*varp, p_ff_values, FALSE) != OK)
	return e_invalid_argument;

    // may also change 'textmode'
    if (get_fileformat(curbuf) == EOL_DOS)
	curbuf->b_p_tx = TRUE;
    else
	curbuf->b_p_tx = FALSE;
    redraw_titles();
    // update flag in swap file
    ml_setflags(curbuf);
    // Redraw needed when switching to/from "mac": a CR in the text
    // will be displayed differently.
    if (get_fileformat(curbuf) == EOL_MAC || *args->os_oldval.string == 'm')
	redraw_curbuf_later(UPD_NOT_VALID);

    return NULL;
}

    int
expand_set_fileformat(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_ff_values,
	    ARRAY_LENGTH(p_ff_values) - 1,
	    numMatches,
	    matches);
}

/*
 * Function given to ExpandGeneric() to obtain the possible arguments of the
 * fileformat options.
 */
    char_u *
get_fileformat_name(expand_T *xp UNUSED, int idx)
{
    if (idx >= (int)ARRAY_LENGTH(p_ff_values))
	return NULL;

    return (char_u*)p_ff_values[idx];
}

/*
 * The 'fileformats' option is changed.
 */
    char *
did_set_fileformats(optset_T *args UNUSED)
{
    if (check_opt_strings(p_ffs, p_ff_values, TRUE) != OK)
	return e_invalid_argument;

    // also change 'textauto'
    if (*p_ffs == NUL)
	p_ta = FALSE;
    else
	p_ta = TRUE;

    return NULL;
}

/*
 * The 'filetype' or the 'syntax' option is changed.
 */
    char *
did_set_filetype_or_syntax(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (!valid_filetype(*varp))
	return e_invalid_argument;

    args->os_value_changed = STRCMP(args->os_oldval.string, *varp) != 0;

    // Since we check the value, there is no need to set P_INSECURE,
    // even when the value comes from a modeline.
    args->os_value_checked = TRUE;

    return NULL;
}

#if defined(FEAT_FOLDING) || defined(PROTO)
/*
 * The 'foldclose' option is changed.
 */
    char *
did_set_foldclose(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_fcl, p_fcl_values, TRUE);
}

    int
expand_set_foldclose(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_fcl_values,
	    ARRAY_LENGTH(p_fcl_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if (defined(FEAT_EVAL) && defined(FEAT_FOLDING)) || defined(PROTO)
/*
 * The 'foldexpr' option is changed.
 */
    char *
did_set_foldexpr(optset_T *args)
{
    (void)did_set_optexpr(args);
    if (foldmethodIsExpr(curwin))
	foldUpdateAll(curwin);
    return NULL;
}
#endif

#if defined(FEAT_FOLDING) || defined(PROTO)
/*
 * The 'foldignore' option is changed.
 */
    char *
did_set_foldignore(optset_T *args UNUSED)
{
    if (foldmethodIsIndent(curwin))
	foldUpdateAll(curwin);
    return NULL;
}

/*
 * The 'foldmarker' option is changed.
 */
    char *
did_set_foldmarker(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char_u	*p;

    p = vim_strchr(*varp, ',');
    if (p == NULL)
	return e_comma_required;
    else if (p == *varp || p[1] == NUL)
	return e_invalid_argument;
    else if (foldmethodIsMarker(curwin))
	foldUpdateAll(curwin);

    return NULL;
}

/*
 * The 'foldmethod' option is changed.
 */
    char *
did_set_foldmethod(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (check_opt_strings(*varp, p_fdm_values, FALSE) != OK
	    || *curwin->w_p_fdm == NUL)
	return e_invalid_argument;

    foldUpdateAll(curwin);
    if (foldmethodIsDiff(curwin))
	newFoldLevel();
    return NULL;
}

    int
expand_set_foldmethod(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_fdm_values,
	    ARRAY_LENGTH(p_fdm_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'foldopen' option is changed.
 */
    char *
did_set_foldopen(optset_T *args UNUSED)
{
    return did_set_opt_flags(p_fdo, p_fdo_values, &fdo_flags, TRUE);
}

    int
expand_set_foldopen(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_fdo_values,
	    ARRAY_LENGTH(p_fdo_values) - 1,
	    numMatches,
	    matches);
}
#endif

/*
 * The 'formatoptions' option is changed.
 */
    char *
did_set_formatoptions(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    return did_set_option_listflag(*varp, (char_u *)FO_ALL, args->os_errbuf,
		    args->os_errbuflen);
}

    int
expand_set_formatoptions(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_listflag(args, (char_u*)FO_ALL, numMatches, matches);
}

#if defined(CURSOR_SHAPE) || defined(PROTO)
/*
 * The 'guicursor' option is changed.
 */
    char *
did_set_guicursor(optset_T *args UNUSED)
{
    return parse_shape_opt(SHAPE_CURSOR);
}
#endif

#if defined(FEAT_GUI) || defined(PROTO)
/*
 * The 'guifont' option is changed.
 */
    char *
did_set_guifont(optset_T *args UNUSED)
{
    char_u	*p;
    char	*errmsg = NULL;

    if (gui.in_use)
    {
	p = p_guifont;
# if defined(FEAT_GUI_GTK)
	// Put up a font dialog and let the user select a new value.
	// If this is cancelled go back to the old value but don't
	// give an error message.
	if (STRCMP(p, "*") == 0)
	{
	    p = gui_mch_font_dialog(args->os_oldval.string);
	    free_string_option(p_guifont);
	    p_guifont = (p != NULL) ? p : vim_strsave(args->os_oldval.string);
	}
# endif
	if (p != NULL && gui_init_font(p_guifont, FALSE) != OK)
	{
# if defined(FEAT_GUI_MSWIN) || defined(FEAT_GUI_PHOTON)
	    if (STRCMP(p_guifont, "*") == 0)
	    {
		// Dialog was cancelled: Keep the old value without giving
		// an error message.
		free_string_option(p_guifont);
		p_guifont = vim_strsave(args->os_oldval.string);
	    }
	    else
# endif
		errmsg = e_invalid_fonts;
	}
    }

    return errmsg;
}

/*
 * Expand the 'guifont' option. Only when GUI is being used. Each platform has
 * specific behaviors.
 */
    int
expand_set_guifont(optexpand_T *args, int *numMatches, char_u ***matches)
{
    if (!gui.in_use)
	return FAIL;

# if defined(FEAT_GUI_MSWIN) || defined(FEAT_GUI_GTK)
    char_u **varp = (char_u **)args->oe_varp;
    int wide = (varp == &p_guifontwide);

    return expand_set_opt_callback(
	    args, gui_mch_expand_font, &wide, numMatches, matches);
# else
    return FAIL;
# endif
}

# if defined(FEAT_XFONTSET) || defined(PROTO)
/*
 * The 'guifontset' option is changed.
 */
    char *
did_set_guifontset(optset_T *args UNUSED)
{
    char *errmsg = NULL;

    if (STRCMP(p_guifontset, "*") == 0)
	errmsg = e_cant_select_fontset;
    else if (gui.in_use && gui_init_font(p_guifontset, TRUE) != OK)
	errmsg = e_invalid_fontset;

    return errmsg;
}
# endif

/*
 * The 'guifontwide' option is changed.
 */
    char *
did_set_guifontwide(optset_T *args UNUSED)
{
    char *errmsg = NULL;

    if (STRCMP(p_guifontwide, "*") == 0)
	errmsg = e_cant_select_wide_font;
    else if (gui_get_wide_font() == FAIL)
	errmsg = e_invalid_wide_font;

    return errmsg;
}
#endif

#if defined(FEAT_GUI_GTK) || defined(PROTO)
/*
 * The 'guiligatures' option is changed.
 */
    char *
did_set_guiligatures(optset_T *args UNUSED)
{
    gui_set_ligatures();
    return NULL;
}
#endif

#if defined(FEAT_GUI) || defined(PROTO)
/*
 * The 'guioptions' option is changed.
 */
    char *
did_set_guioptions(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char *errmsg;

    errmsg = did_set_option_listflag(*varp, (char_u *)GO_ALL, args->os_errbuf,
		    args->os_errbuflen);
    if (errmsg != NULL)
	return errmsg;

    gui_init_which_components(args->os_oldval.string);
    return NULL;
}

    int
expand_set_guioptions(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_listflag(args, (char_u*)GO_ALL, numMatches, matches);
}
#endif

#if defined(FEAT_GUI_TABLINE) || defined(PROTO)
/*
 * The 'guitablabel' option is changed.
 */
    char *
did_set_guitablabel(optset_T *args UNUSED)
{
    redraw_tabline = TRUE;
    return NULL;
}
#endif

/*
 * The 'helpfile' option is changed.
 */
    char *
did_set_helpfile(optset_T *args UNUSED)
{
    // May compute new values for $VIM and $VIMRUNTIME
    if (didset_vim)
	vim_unsetenv_ext((char_u *)"VIM");
    if (didset_vimruntime)
	vim_unsetenv_ext((char_u *)"VIMRUNTIME");
    return NULL;
}

#if defined(FEAT_MULTI_LANG) || defined(PROTO)
/*
 * The 'helplang' option is changed.
 */
    char *
did_set_helplang(optset_T *args UNUSED)
{
    char *errmsg = NULL;

    // Check for "", "ab", "ab,cd", etc.
    for (char_u *s = p_hlg; *s != NUL; s += 3)
    {
	if (s[1] == NUL || ((s[2] != ',' || s[3] == NUL) && s[2] != NUL))
	{
	    errmsg = e_invalid_argument;
	    break;
	}
	if (s[2] == NUL)
	    break;
    }

    return errmsg;
}
#endif

/*
 * The 'highlight' option is changed.
 */
    char *
did_set_highlight(optset_T *args UNUSED)
{
    if (highlight_changed() == FAIL)
	return e_invalid_argument;	// invalid flags

    return NULL;
}

/*
 * Expand 'highlight' option.
 */
    int
expand_set_highlight(optexpand_T *args, int *numMatches, char_u ***matches)
{
    char_u	    *p;
    expand_T	    *xp = args->oe_xp;
    static char_u   hl_flags[HLF_COUNT] = HL_FLAGS;
    size_t	    i;
    int		    count = 0;

    if (xp->xp_pattern > args->oe_set_arg && *(xp->xp_pattern-1) == ':')
    {
	// Right after a ':', meaning we just return all highlight names.
	return expand_set_opt_generic(
		args,
		get_highlight_name,
		numMatches,
		matches);
    }

    if (*xp->xp_pattern == NUL)
    {
	// At beginning of a comma-separated list. Return the specific list of
	// supported occasions.
	*matches = ALLOC_MULT(char_u *, HLF_COUNT + 1);
	if (*matches == NULL)
	    return FAIL;

	// We still want to return the full option if it's requested.
	if (args->oe_include_orig_val)
	{
	    p = vim_strsave(args->oe_opt_value);
	    if (p == NULL)
	    {
		VIM_CLEAR(*matches);
		return FAIL;
	    }
	    (*matches)[count++] = p;
	}

	for (i = 0; i < HLF_COUNT; i++)
	{
	    p = vim_strnsave(&hl_flags[i], 1);
	    if (p == NULL)
	    {
		if (count == 0)
		{
		    VIM_CLEAR(*matches);
		    return FAIL;
		}
		else
		    break;
	    }
	    (*matches)[count++] = p;
	}

	if (count == 0)
	{
	    VIM_CLEAR(*matches);
	    return FAIL;
	}
	*numMatches = count;
	return OK;
    }

    // We are after the initial character (which indicates the occasion). We
    // already made sure we are not matching after a ':' above, so now we want
    // to match against display mode modifiers.
    // Since the xp_pattern starts from the beginning, we need to include it in
    // the returned match.

    // Note: Keep this in sync with highlight_changed()
    static char_u p_hl_mode_values[] =
	{':', 'b', 'i', '-', 'n', 'r', 's', 'u', 'c', '2', 'd', '=', 't'};
    size_t num_hl_modes = ARRAY_LENGTH(p_hl_mode_values);

    *matches = ALLOC_MULT(char_u *, num_hl_modes);
    if (*matches == NULL)
	return FAIL;

    int pattern_len = xp->xp_pattern_len;

    for (i = 0; i < num_hl_modes; i++)
    {
	// Don't allow duplicates as these are unique flags
	char_u *dup = vim_strchr(xp->xp_pattern + 1, p_hl_mode_values[i]);
	if (dup != NULL && (int)(dup - xp->xp_pattern) < pattern_len)
	    continue;

	// ':' only works by itself, not with other flags.
	if (pattern_len > 1 && p_hl_mode_values[i] == ':')
	    continue;

	p = vim_strnsave(xp->xp_pattern, pattern_len + 1);
	if (p == NULL)
	{
	    if (i == 0)
	    {
		VIM_CLEAR(*matches);
		return FAIL;
	    }
	    else
		break;
	}
	p[pattern_len] = p_hl_mode_values[i];
	p[pattern_len + 1] = NUL;
	(*matches)[count++] = p;
    }
    if (count == 0)
    {
	VIM_CLEAR(*matches);
	return FAIL;
    }
    *numMatches = count;

    return OK;
}

/*
 * The 'titlestring' or the 'iconstring' option is changed.
 */
    static char *
parse_titleiconstring(optset_T *args UNUSED, int flagval UNUSED)
{
#ifdef FEAT_STL_OPT
    char_u	**varp = (char_u **)args->os_varp;

    // NULL => statusline syntax
    if (vim_strchr(*varp, '%') && check_stl_option(*varp) == NULL)
	stl_syntax |= flagval;
    else
	stl_syntax &= ~flagval;
#endif
    did_set_title();

    return NULL;
}

/*
 * The 'iconstring' option is changed.
 */
    char *
did_set_iconstring(optset_T *args)
{
    int flagval = 0;

#ifdef FEAT_STL_OPT
    flagval = STL_IN_ICON;
#endif

    return parse_titleiconstring(args, flagval);
}

#if (defined(FEAT_XIM) && defined(FEAT_GUI_GTK)) || defined(PROTO)
/*
 * The 'imactivatekey' option is changed.
 */
    char *
did_set_imactivatekey(optset_T *args UNUSED)
{
    if (!im_xim_isvalid_imactivate())
	return e_invalid_argument;
    return NULL;
}
#endif

/*
 * The 'isident' or the 'iskeyword' or the 'isprint' or the 'isfname' option is
 * changed.
 */
    char *
did_set_isopt(optset_T *args)
{
    // 'isident', 'iskeyword', 'isprint or 'isfname' option: refill g_chartab[]
    // If the new option is invalid, use old value.
    // 'lisp' option: refill g_chartab[] for '-' char.
    if (init_chartab() == FAIL)
    {
	args->os_restore_chartab = TRUE;// need to restore the chartab.
	return e_invalid_argument;	// error in value
    }

    return NULL;
}

/*
 * The 'jumpoptions' option is changed.
 */
    char *
did_set_jumpoptions(optset_T *args UNUSED)
{
    if (opt_strings_flags(p_jop, p_jop_values, &jop_flags, TRUE) != OK)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_jumpoptions(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_jop_values,
	    ARRAY_LENGTH(p_jop_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_KEYMAP) || defined(PROTO)
/*
 * The 'keymap' option is changed.
 */
    char *
did_set_keymap(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char *errmsg = NULL;

    if (!valid_filetype(*varp))
	errmsg = e_invalid_argument;
    else
    {
	int	    secure_save = secure;

	// Reset the secure flag, since the value of 'keymap' has
	// been checked to be safe.
	secure = 0;

	// load or unload key mapping tables
	errmsg = keymap_init();

	secure = secure_save;

	// Since we check the value, there is no need to set P_INSECURE,
	// even when the value comes from a modeline.
	args->os_value_checked = TRUE;
    }

    if (errmsg == NULL)
    {
	if (*curbuf->b_p_keymap != NUL)
	{
	    // Installed a new keymap, switch on using it.
	    curbuf->b_p_iminsert = B_IMODE_LMAP;
	    if (curbuf->b_p_imsearch != B_IMODE_USE_INSERT)
		curbuf->b_p_imsearch = B_IMODE_LMAP;
	}
	else
	{
	    // Cleared the keymap, may reset 'iminsert' and 'imsearch'.
	    if (curbuf->b_p_iminsert == B_IMODE_LMAP)
		curbuf->b_p_iminsert = B_IMODE_NONE;
	    if (curbuf->b_p_imsearch == B_IMODE_LMAP)
		curbuf->b_p_imsearch = B_IMODE_USE_INSERT;
	}
	if ((args->os_flags & OPT_LOCAL) == 0)
	{
	    set_iminsert_global();
	    set_imsearch_global();
	}
	status_redraw_curbuf();
    }

    return errmsg;
}
#endif

/*
 * The 'keymodel' option is changed.
 */
    char *
did_set_keymodel(optset_T *args UNUSED)
{
    if (check_opt_strings(p_km, p_km_values, TRUE) != OK)
	return e_invalid_argument;

    km_stopsel = (vim_strchr(p_km, 'o') != NULL);
    km_startsel = (vim_strchr(p_km, 'a') != NULL);
    return NULL;
}

    int
expand_set_keymodel(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_km_values,
	    ARRAY_LENGTH(p_km_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'keyprotocol' option is changed.
 */
    char *
did_set_keyprotocol(optset_T *args UNUSED)
{
    char_u *term = T_NAME;
    keyprot_T kpc = match_keyprotocol(term);
    if (kpc == KEYPROTOCOL_FAIL)
	return e_invalid_argument;

    apply_keyprotocol(term, kpc);

    return NULL;
}

    int
expand_set_keyprotocol(optexpand_T *args, int *numMatches, char_u ***matches)
{
    expand_T *xp = args->oe_xp;
    if (xp->xp_pattern > args->oe_set_arg && *(xp->xp_pattern-1) == ':')
    {
	// 'keyprotocol' only has well-defined terms for completion for the
	// protocol part after the colon.
	return expand_set_opt_string(
		args,
		p_kpc_protocol_values,
		ARRAY_LENGTH(p_kpc_protocol_values) - 1,
		numMatches,
		matches);
    }
    // Use expand_set_opt_string instead of returning FAIL so that we can
    // include the original value if args->oe_include_orig_val is set.
    static char *(empty[]) = {NULL};
    return expand_set_opt_string(args, empty, 0, numMatches, matches);
}

/*
 * The 'lispoptions' option is changed.
 */
    char *
did_set_lispoptions(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (**varp != NUL
	    && STRCMP(*varp, "expr:0") != 0
	    && STRCMP(*varp, "expr:1") != 0)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_lispoptions(optexpand_T *args, int *numMatches, char_u ***matches)
{
    static char *(p_lop_values[]) = {"expr:0", "expr:1", NULL};
    return expand_set_opt_string(
	    args,
	    p_lop_values,
	    ARRAY_LENGTH(p_lop_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'matchpairs' option is changed.
 */
    char *
did_set_matchpairs(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char_u	*p;

    if (has_mbyte)
    {
	for (p = *varp; *p != NUL; ++p)
	{
	    int x2 = -1;
	    int x3 = -1;

	    p += mb_ptr2len(p);
	    if (*p != NUL)
		x2 = *p++;
	    if (*p != NUL)
	    {
		x3 = mb_ptr2char(p);
		p += mb_ptr2len(p);
	    }
	    if (x2 != ':' || x3 == -1 || (*p != NUL && *p != ','))
		return e_invalid_argument;
	    if (*p == NUL)
		break;
	}
    }
    else
    {
	// Check for "x:y,x:y"
	for (p = *varp; *p != NUL; p += 4)
	{
	    if (p[1] != ':' || p[2] == NUL || (p[3] != NUL && p[3] != ','))
		return e_invalid_argument;
	    if (p[3] == NUL)
		break;
	}
    }

    return NULL;
}

#if defined(FEAT_SPELL) || defined(PROTO)
/*
 * The 'mkspellmem' option is changed.
 */
    char *
did_set_mkspellmem(optset_T *args UNUSED)
{
    if (spell_check_msm() != OK)
	return e_invalid_argument;

    return NULL;
}
#endif

/*
 * The 'mouse' option is changed.
 */
    char *
did_set_mouse(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    return did_set_option_listflag(*varp, (char_u *)MOUSE_ALL, args->os_errbuf,
		    args->os_errbuflen);
}

    int
expand_set_mouse(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_listflag(args, (char_u*)MOUSE_ALL, numMatches, matches);
}

/*
 * The 'mousemodel' option is changed.
 */
    char *
did_set_mousemodel(optset_T *args UNUSED)
{
    if (check_opt_strings(p_mousem, p_mousem_values, FALSE) != OK)
	return e_invalid_argument;
#if defined(FEAT_GUI_MOTIF) && defined(FEAT_MENU) && (XmVersion <= 1002)
    else if (*p_mousem != *args->os_oldval.string)
	// Changed from "extend" to "popup" or "popup_setpos" or vv: need
	// to create or delete the popup menus.
	gui_motif_update_mousemodel(root_menu);
#endif

    return NULL;
}

    int
expand_set_mousemodel(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_mousem_values,
	    ARRAY_LENGTH(p_mousem_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_MOUSESHAPE) || defined(PROTO)
    char *
did_set_mouseshape(optset_T *args UNUSED)
{
    char *errmsg = NULL;

    errmsg = parse_shape_opt(SHAPE_MOUSE);
    update_mouseshape(-1);

    return errmsg;
}
#endif

/*
 * The 'nrformats' option is changed.
 */
    char *
did_set_nrformats(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    return did_set_opt_strings(*varp, p_nf_values, TRUE);
}

    int
expand_set_nrformats(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_nf_values,
	    ARRAY_LENGTH(p_nf_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * One of the '*expr' options is changed: 'balloonexpr', 'diffexpr',
 * 'foldexpr', 'foldtext', 'formatexpr', 'includeexpr', 'indentexpr',
 * 'patchexpr', 'printexpr' and 'charconvert'.
 *
 */
    char *
did_set_optexpr(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    // If the option value starts with <SID> or s:, then replace that with
    // the script identifier.
    char_u *name = get_scriptlocal_funcname(*varp);
    if (name != NULL)
    {
	free_string_option(*varp);
	*varp = name;
    }

    return NULL;
}
#endif

/*
 * The 'pastetoggle' option is changed.
 */
    char *
did_set_pastetoggle(optset_T *args UNUSED)
{
    char_u	*p;

    // translate key codes like in a mapping
    if (*p_pt)
    {
	(void)replace_termcodes(p_pt, &p, 0,
		REPTERM_FROM_PART | REPTERM_DO_LT, NULL);
	if (p != NULL)
	{
	    free_string_option(p_pt);
	    p_pt = p;
	}
    }

    return NULL;
}

#if defined(FEAT_PROP_POPUP) || defined(PROTO)
/*
 * The 'previewpopup' option is changed.
 */
    char *
did_set_previewpopup(optset_T *args UNUSED)
{
    if (parse_previewpopup(NULL) == FAIL)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_popupoption(optexpand_T *args, int *numMatches, char_u ***matches)
{
    expand_T *xp = args->oe_xp;

    if (xp->xp_pattern > args->oe_set_arg && *(xp->xp_pattern-1) == ':')
    {
	// Within "highlight:"/"border:"/"align:", we have a subgroup of possible options.
	int border_len = (int)STRLEN("border:");
	if (xp->xp_pattern - args->oe_set_arg >= border_len &&
		STRNCMP(xp->xp_pattern - border_len, "border:", border_len) == 0)
	{
	    return expand_set_opt_string(
		    args,
		    p_popup_option_border_values,
		    ARRAY_LENGTH(p_popup_option_border_values) - 1,
		    numMatches,
		    matches);
	}
	int align_len = (int)STRLEN("align:");
	if (xp->xp_pattern - args->oe_set_arg >= align_len &&
		STRNCMP(xp->xp_pattern - align_len, "align:", align_len) == 0)
	{
	    return expand_set_opt_string(
		    args,
		    p_popup_option_align_values,
		    ARRAY_LENGTH(p_popup_option_align_values) - 1,
		    numMatches,
		    matches);
	}
	int highlight_len = (int)STRLEN("highlight:");
	if (xp->xp_pattern - args->oe_set_arg >= highlight_len &&
		STRNCMP(xp->xp_pattern - highlight_len, "highlight:", highlight_len) == 0)
	{
	    // Return the list of all highlight names
	    return expand_set_opt_generic(
		    args,
		    get_highlight_name,
		    numMatches,
		    matches);
	}
	return FAIL;
    }

    return expand_set_opt_string(
	    args,
	    p_popup_option_values,
	    ARRAY_LENGTH(p_popup_option_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if defined(FEAT_POSTSCRIPT) || defined(PROTO)
/*
 * The 'printencoding' option is changed.
 */
    char *
did_set_printencoding(optset_T *args UNUSED)
{
    char_u	*s, *p;

    // Canonize 'printencoding' if VIM standard one
    p = enc_canonize(p_penc);
    if (p != NULL)
    {
	vim_free(p_penc);
	p_penc = p;
    }
    else
    {
	// Ensure lower case and '-' for '_'
	for (s = p_penc; *s != NUL; s++)
	{
	    if (*s == '_')
		*s = '-';
	    else
		*s = TOLOWER_ASC(*s);
	}
    }

    return NULL;
}
#endif

#if defined(FEAT_PRINTER) || defined(PROTO)

    static char_u *
get_printoptions_names(expand_T *xp UNUSED, int idx)
{
    if (idx >= (int)ARRAY_LENGTH(printer_opts))
	return NULL;
    return (char_u*)printer_opts[idx].name;
}

/*
 * Expand 'printoptions' option
 */
    int
expand_set_printoptions(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_generic(
	    args,
	    get_printoptions_names,
	    numMatches,
	    matches);
}
#endif

#if defined(FEAT_STL_OPT) || defined(PROTO)
/*
 * The 'statusline' or the 'tabline' or the 'rulerformat' option is changed.
 * "rulerformat" is TRUE if the 'rulerformat' option is changed.
 */
    static char *
parse_statustabline_rulerformat(optset_T *args, int rulerformat)
{
    char_u	**varp = (char_u **)args->os_varp;
    char_u	*s;
    char	*errmsg = NULL;
    int		wid;

    if (rulerformat)	// reset ru_wid first
	ru_wid = 0;
    s = *varp;
    if (rulerformat && *s == '%')
    {
	// set ru_wid if 'ruf' starts with "%99("
	if (*++s == '-')	// ignore a '-'
	    s++;
	wid = getdigits(&s);
	if (wid && *s == '(' && (errmsg = check_stl_option(p_ruf)) == NULL)
	    ru_wid = wid;
	else
	    errmsg = check_stl_option(p_ruf);
    }
    // check 'statusline' or 'tabline' only if it doesn't start with "%!"
    else if (rulerformat || s[0] != '%' || s[1] != '!')
	errmsg = check_stl_option(s);
    if (rulerformat && errmsg == NULL)
	comp_col();

    return errmsg;
}
#endif

#if defined(FEAT_RENDER_OPTIONS) || defined(PROTO)
/*
 * The 'renderoptions' option is changed.
 */
    char *
did_set_renderoptions(optset_T *args UNUSED)
{
    if (!gui_mch_set_rendering_options(p_rop))
	return e_invalid_argument;

    return NULL;
}
#endif

#if defined(FEAT_RIGHTLEFT) || defined(PROTO)
/*
 * The 'rightleftcmd' option is changed.
 */
    char *
did_set_rightleftcmd(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    // Currently only "search" is a supported value.
    if (**varp != NUL && STRCMP(*varp, "search") != 0)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_rightleftcmd(optexpand_T *args, int *numMatches, char_u ***matches)
{
    static char *(p_rlc_values[]) = {"search", NULL};
    return expand_set_opt_string(
	    args,
	    p_rlc_values,
	    ARRAY_LENGTH(p_rlc_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if defined(FEAT_STL_OPT) || defined(PROTO)
/*
 * The 'rulerformat' option is changed.
 */
    char *
did_set_rulerformat(optset_T *args)
{
    return parse_statustabline_rulerformat(args, TRUE);
}
#endif

/*
 * The 'scrollopt' option is changed.
 */
    char *
did_set_scrollopt(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_sbo, p_scbopt_values, TRUE);
}

    int
expand_set_scrollopt(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_scbopt_values,
	    ARRAY_LENGTH(p_scbopt_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'selection' option is changed.
 */
    char *
did_set_selection(optset_T *args UNUSED)
{
    if (*p_sel == NUL || check_opt_strings(p_sel, p_sel_values, FALSE) != OK)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_selection(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_sel_values,
	    ARRAY_LENGTH(p_sel_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'selectmode' option is changed.
 */
    char *
did_set_selectmode(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_slm, p_slm_values, TRUE);
}

    int
expand_set_selectmode(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_slm_values,
	    ARRAY_LENGTH(p_slm_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_SESSION) || defined(PROTO)
/*
 * The 'sessionoptions' option is changed.
 */
    char *
did_set_sessionoptions(optset_T *args)
{
    if (opt_strings_flags(p_ssop, p_ssop_values, &ssop_flags, TRUE) != OK)
	return e_invalid_argument;
    if ((ssop_flags & SSOP_CURDIR) && (ssop_flags & SSOP_SESDIR))
    {
	// Don't allow both "sesdir" and "curdir".
	(void)opt_strings_flags(args->os_oldval.string, p_ssop_values,
							&ssop_flags, TRUE);
	return e_invalid_argument;
    }

    return NULL;
}

    int
expand_set_sessionoptions(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_ssop_values,
	    ARRAY_LENGTH(p_ssop_values) - 1,
	    numMatches,
	    matches);
}
#endif

/*
 * The 'shortmess' option is changed.
 */
    char *
did_set_shortmess(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    return did_set_option_listflag(*varp, (char_u *)SHM_ALL, args->os_errbuf,
		    args->os_errbuflen);
}

    int
expand_set_shortmess(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_listflag(args, (char_u*)SHM_ALL, numMatches, matches);
}

#if defined(FEAT_LINEBREAK) || defined(PROTO)
/*
 * The 'showbreak' option is changed.
 */
    char *
did_set_showbreak(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char_u	*s;

    for (s = *varp; *s; )
    {
	if (ptr2cells(s) != 1)
	    return e_showbreak_contains_unprintable_or_wide_character;
	MB_PTR_ADV(s);
    }

    return NULL;
}
#endif

/*
 * The 'showcmdloc' option is changed.
 */
    char *
did_set_showcmdloc(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_sloc, p_sloc_values, FALSE);
}

    int
expand_set_showcmdloc(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_sloc_values,
	    ARRAY_LENGTH(p_sloc_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_SIGNS) || defined(PROTO)
/*
 * The 'signcolumn' option is changed.
 */
    char *
did_set_signcolumn(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (check_opt_strings(*varp, p_scl_values, FALSE) != OK)
	return e_invalid_argument;
    // When changing the 'signcolumn' to or from 'number', recompute the
    // width of the number column if 'number' or 'relativenumber' is set.
    if (((*args->os_oldval.string == 'n' && args->os_oldval.string[1] == 'u')
		|| (*curwin->w_p_scl == 'n' && *(curwin->w_p_scl + 1) =='u'))
	    && (curwin->w_p_nu || curwin->w_p_rnu))
	curwin->w_nrwidth_line_count = 0;

    return NULL;
}

    int
expand_set_signcolumn(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_scl_values,
	    ARRAY_LENGTH(p_scl_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if defined(FEAT_SPELL) || defined(PROTO)
/*
 * The 'spellcapcheck' option is changed.
 */
    char *
did_set_spellcapcheck(optset_T *args UNUSED)
{
    // compile the regexp program.
    return compile_cap_prog(curwin->w_s);
}

/*
 * The 'spellfile' option is changed.
 */
    char *
did_set_spellfile(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (!valid_spellfile(*varp))
	return e_invalid_argument;

    // If there is a window for this buffer in which 'spell' is set load the
    // wordlists.
    return did_set_spell_option(TRUE);
}

/*
 * The 'spell' option is changed.
 */
    char *
did_set_spelllang(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (!valid_spelllang(*varp))
	return e_invalid_argument;

    // If there is a window for this buffer in which 'spell' is set load the
    // wordlists.
    return did_set_spell_option(FALSE);
}

/*
 * The 'spelloptions' option is changed.
 */
    char *
did_set_spelloptions(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (**varp != NUL && STRCMP(*varp, "camel") != 0)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_spelloptions(optexpand_T *args, int *numMatches, char_u ***matches)
{
    static char *(p_spo_values[]) = {"camel", NULL};
    return expand_set_opt_string(
	    args,
	    p_spo_values,
	    ARRAY_LENGTH(p_spo_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'spellsuggest' option is changed.
 */
    char *
did_set_spellsuggest(optset_T *args UNUSED)
{
    if (spell_check_sps() != OK)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_spellsuggest(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_sps_values,
	    ARRAY_LENGTH(p_sps_values) - 1,
	    numMatches,
	    matches);
}
#endif

/*
 * The 'splitkeep' option is changed.
 */
    char *
did_set_splitkeep(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_spk, p_spk_values, FALSE);
}

    int
expand_set_splitkeep(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_spk_values,
	    ARRAY_LENGTH(p_spk_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_STL_OPT) || defined(PROTO)
/*
 * The 'statusline' option is changed.
 */
    char *
did_set_statusline(optset_T *args)
{
    return parse_statustabline_rulerformat(args, FALSE);
}
#endif

/*
 * The 'swapsync' option is changed.
 */
    char *
did_set_swapsync(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_sws, p_sws_values, FALSE);
}

    int
expand_set_swapsync(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_sws_values,
	    ARRAY_LENGTH(p_sws_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'switchbuf' option is changed.
 */
    char *
did_set_switchbuf(optset_T *args UNUSED)
{
    return did_set_opt_flags(p_swb, p_swb_values, &swb_flags, TRUE);
}

    int
expand_set_switchbuf(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_swb_values,
	    ARRAY_LENGTH(p_swb_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_STL_OPT) || defined(PROTO)
/*
 * The 'tabline' option is changed.
 */
    char *
did_set_tabline(optset_T *args)
{
    return parse_statustabline_rulerformat(args, FALSE);
}
#endif

/*
 * The 'tagcase' option is changed.
 */
    char *
did_set_tagcase(optset_T *args)
{
    unsigned int	*flags;
    char_u		*p;

    if (args->os_flags & OPT_LOCAL)
    {
	p = curbuf->b_p_tc;
	flags = &curbuf->b_tc_flags;
    }
    else
    {
	p = p_tc;
	flags = &tc_flags;
    }

    if ((args->os_flags & OPT_LOCAL) && *p == NUL)
	// make the local value empty: use the global value
	*flags = 0;
    else if (*p == NUL
	    || opt_strings_flags(p, p_tc_values, flags, FALSE) != OK)
	return e_invalid_argument;

    return NULL;
}

    int
expand_set_tagcase(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_tc_values,
	    ARRAY_LENGTH(p_tc_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'term' option is changed.
 */
    char *
did_set_term(optset_T *args UNUSED)
{
    if (T_NAME[0] == NUL)
	return e_cannot_set_term_to_empty_string;
#ifdef FEAT_GUI
    if (gui.in_use)
	return e_cannot_change_term_in_GUI;
    if (term_is_gui(T_NAME))
	return e_use_gui_to_start_GUI;
#endif
    if (set_termname(T_NAME) == FAIL)
	return e_not_found_in_termcap;

    // Screen colors may have changed.
    redraw_later_clear();

    return NULL;
}

/*
 * Some terminal option (t_xxx) is changed
 */
    char *
did_set_term_option(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    if (!full_screen)
	return NULL;

    // ":set t_Co=0" and ":set t_Co=1" do ":set t_Co="
    if (varp == &T_CCO)
    {
	int colors = atoi((char *)T_CCO);

	// Only reinitialize colors if t_Co value has really changed to
	// avoid expensive reload of colorscheme if t_Co is set to the
	// same value multiple times.
	if (colors != t_colors)
	{
	    t_colors = colors;
	    if (t_colors <= 1)
	    {
		vim_free(T_CCO);
		T_CCO = empty_option;
	    }
#if defined(FEAT_VTP) && defined(FEAT_TERMGUICOLORS)
	    if (is_term_win32())
	    {
		swap_tcap();
		args->os_did_swaptcap = TRUE;
	    }
#endif
	    // We now have a different color setup, initialize it again.
	    init_highlight(TRUE, FALSE);
	}
    }
    ttest(FALSE);
    if (varp == &T_ME)
    {
	out_str(T_ME);
	redraw_later(UPD_CLEAR);
#if defined(MSWIN) && (!defined(FEAT_GUI_MSWIN) || defined(VIMDLL))
	// Since t_me has been set, this probably means that the user
	// wants to use this as default colors.  Need to reset default
	// background/foreground colors.
# ifdef VIMDLL
	if (!gui.in_use && !gui.starting)
# endif
	    mch_set_normal_colors();
#endif
    }
    if (varp == &T_BE && termcap_active)
    {
	MAY_WANT_TO_LOG_THIS;

	if (*T_BE == NUL)
	    // When clearing t_BE we assume the user no longer wants
	    // bracketed paste, thus disable it by writing t_BD.
	    out_str(T_BD);
	else
	    out_str(T_BE);
    }

    return NULL;
}

#if defined(FEAT_TERMINAL) || defined(PROTO)
/*
 * The 'termwinkey' option is changed.
 */
    char *
did_set_termwinkey(optset_T *args UNUSED)
{
    if (*curwin->w_p_twk != NUL && string_to_key(curwin->w_p_twk, TRUE) == 0)
	return e_invalid_argument;

    return NULL;
}

/*
 * The 'termwinsize' option is changed.
 */
    char *
did_set_termwinsize(optset_T *args UNUSED)
{
    char_u	*p;

    if (*curwin->w_p_tws == NUL)
	return NULL;

    p = skipdigits(curwin->w_p_tws);
    if (p == curwin->w_p_tws
	    || (*p != 'x' && *p != '*')
	    || *skipdigits(p + 1) != NUL)
	return e_invalid_argument;

    return NULL;
}

# if defined(MSWIN) || defined(PROTO)
/*
 * The 'termwintype' option is changed.
 */
    char *
did_set_termwintype(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_twt, p_twt_values, FALSE);
}

    int
expand_set_termwintype(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_twt_values,
	    ARRAY_LENGTH(p_twt_values) - 1,
	    numMatches,
	    matches);
}
# endif
#endif

/*
 * The 'titlestring' option is changed.
 */
    char *
did_set_titlestring(optset_T *args)
{
    int flagval = 0;

#ifdef FEAT_STL_OPT
    flagval = STL_IN_TITLE;
#endif
    return parse_titleiconstring(args, flagval);
}

#if (defined(FEAT_TOOLBAR) && !defined(FEAT_GUI_MSWIN)) || defined(PROTO)
/*
 * The 'toolbar' option is changed.
 */
    char *
did_set_toolbar(optset_T *args UNUSED)
{
    if (opt_strings_flags(p_toolbar, p_toolbar_values, &toolbar_flags,
								TRUE) != OK)
	return e_invalid_argument;

    out_flush();
    gui_mch_show_toolbar((toolbar_flags &
		(TOOLBAR_TEXT | TOOLBAR_ICONS)) != 0);
    return NULL;
}

    int
expand_set_toolbar(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_toolbar_values,
	    ARRAY_LENGTH(p_toolbar_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if (defined(FEAT_TOOLBAR) && defined(FEAT_GUI_GTK)) || defined(PROTO)
/*
 * The 'toolbariconsize' option is changed.  GTK+ 2 only.
 */
    char *
did_set_toolbariconsize(optset_T *args UNUSED)
{
    if (opt_strings_flags(p_tbis, p_tbis_values, &tbis_flags, FALSE) != OK)
	return e_invalid_argument;

    out_flush();
    gui_mch_show_toolbar((toolbar_flags &
		(TOOLBAR_TEXT | TOOLBAR_ICONS)) != 0);
    return NULL;
}

    int
expand_set_toolbariconsize(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_tbis_values,
	    ARRAY_LENGTH(p_tbis_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if defined(UNIX) || defined(VMS) || defined(PROTO)
/*
 * The 'ttymouse' option is changed.
 */
    char *
did_set_ttymouse(optset_T *args UNUSED)
{
    char *errmsg = NULL;

    // Switch the mouse off before changing the escape sequences used for
    // that.
    mch_setmouse(FALSE);
    if (opt_strings_flags(p_ttym, p_ttym_values, &ttym_flags, FALSE) != OK)
	errmsg = e_invalid_argument;
    else
	check_mouse_termcode();
    if (termcap_active)
	setmouse();		// may switch it on again

    return errmsg;
}

    int
expand_set_ttymouse(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_ttym_values,
	    ARRAY_LENGTH(p_ttym_values) - 1,
	    numMatches,
	    matches);
}
#endif

#if defined(FEAT_VARTABS) || defined(PROTO)
/*
 * The 'varsofttabstop' option is changed.
 */
    char *
did_set_varsofttabstop(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char_u *cp;

    if (!((*varp)[0]) || ((*varp)[0] == '0' && !((*varp)[1])))
	VIM_CLEAR(curbuf->b_p_vsts_array);
    else
    {
	for (cp = *varp; *cp; ++cp)
	{
	    if (vim_isdigit(*cp))
		continue;
	    if (*cp == ',' && cp > *varp && *(cp-1) != ',')
		continue;
	    return e_invalid_argument;
	}

	int *oldarray = curbuf->b_p_vsts_array;
	if (tabstop_set(*varp, &(curbuf->b_p_vsts_array)) == OK)
	{
	    if (oldarray)
		vim_free(oldarray);
	}
	else
	    return e_invalid_argument;
    }

    return NULL;
}

/*
 * The 'vartabstop' option is changed.
 */
    char *
did_set_vartabstop(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;
    char_u *cp;

    if (!((*varp)[0]) || ((*varp)[0] == '0' && !((*varp)[1])))
	VIM_CLEAR(curbuf->b_p_vts_array);
    else
    {
	for (cp = *varp; *cp; ++cp)
	{
	    if (vim_isdigit(*cp))
		continue;
	    if (*cp == ',' && cp > *varp && *(cp-1) != ',')
		continue;
	    return e_invalid_argument;
	}

	int *oldarray = curbuf->b_p_vts_array;

	if (tabstop_set(*varp, &(curbuf->b_p_vts_array)) == OK)
	{
	    vim_free(oldarray);
# ifdef FEAT_FOLDING
	    if (foldmethodIsIndent(curwin))
		foldUpdateAll(curwin);
# endif
	}
	else
	    return e_invalid_argument;
    }

    return NULL;
}
#endif

/*
 * The 'verbosefile' option is changed.
 */
    char *
did_set_verbosefile(optset_T *args UNUSED)
{
    verbose_stop();
    if (*p_vfile != NUL && verbose_open() == FAIL)
	return e_invalid_argument;

    return NULL;
}

#if defined(FEAT_SESSION) || defined(PROTO)
/*
 * The 'viewoptions' option is changed.
 */
    char *
did_set_viewoptions(optset_T *args UNUSED)
{
    return did_set_opt_flags(p_vop, p_ssop_values, &vop_flags, TRUE);
}
#endif

#if defined(FEAT_VIMINFO) || defined(PROTO)
/*
 * The 'viminfo' option is changed.
 */
    char *
did_set_viminfo(optset_T *args)
{
    char_u	*s;
    char	*errmsg = NULL;

    for (s = p_viminfo; *s;)
    {
	// Check it's a valid character
	if (vim_strchr((char_u *)"!\"%'/:<@cfhnrs", *s) == NULL)
	{
	    errmsg = illegal_char(args->os_errbuf, args->os_errbuflen, *s);
	    break;
	}
	if (*s == 'n')	// name is always last one
	    break;
	else if (*s == 'r') // skip until next ','
	{
	    while (*++s && *s != ',')
		;
	}
	else if (*s == '%')
	{
	    // optional number
	    while (vim_isdigit(*++s))
		;
	}
	else if (*s == '!' || *s == 'h' || *s == 'c')
	    ++s;		// no extra chars
	else		// must have a number
	{
	    while (vim_isdigit(*++s))
		;

	    if (!VIM_ISDIGIT(*(s - 1)))
	    {
		if (args->os_errbuf != NULL)
		{
		    vim_snprintf(args->os_errbuf, args->os_errbuflen,
			    _(e_missing_number_after_angle_str_angle),
			    transchar_byte(*(s - 1)));
		    errmsg = args->os_errbuf;
		}
		else
		    errmsg = "";
		break;
	    }
	}
	if (*s == ',')
	    ++s;
	else if (*s)
	{
	    if (args->os_errbuf != NULL)
		errmsg = e_missing_comma;
	    else
		errmsg = "";
	    break;
	}
    }
    if (*p_viminfo && errmsg == NULL && get_viminfo_parameter('\'') < 0)
	errmsg = e_must_specify_a_value;

    return errmsg;
}
#endif

/*
 * The 'virtualedit' option is changed.
 */
    char *
did_set_virtualedit(optset_T *args)
{
    char_u		*ve = p_ve;
    unsigned int	*flags = &ve_flags;

    if (args->os_flags & OPT_LOCAL)
    {
	ve = curwin->w_p_ve;
	flags = &curwin->w_ve_flags;
    }

    if ((args->os_flags & OPT_LOCAL) && *ve == NUL)
	// make the local value empty: use the global value
	*flags = 0;
    else
    {
	if (opt_strings_flags(ve, p_ve_values, flags, TRUE) != OK)
	    return e_invalid_argument;
	else if (STRCMP(ve, args->os_oldval.string) != 0)
	{
	    // Recompute cursor position in case the new 've' setting
	    // changes something.
	    validate_virtcol();
	    coladvance(curwin->w_virtcol);
	}
    }

    return NULL;
}

    int
expand_set_virtualedit(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_ve_values,
	    ARRAY_LENGTH(p_ve_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'whichwrap' option is changed.
 */
    char *
did_set_whichwrap(optset_T *args)
{
    char_u	**varp = (char_u **)args->os_varp;

    // Add ',' to the list flags because 'whichwrap' is a flag
    // list that is comma-separated.
    return did_set_option_listflag(*varp, (char_u *)(WW_ALL ","),
		    args->os_errbuf, args->os_errbuflen);
}

    int
expand_set_whichwrap(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_listflag(args, (char_u*)WW_ALL, numMatches, matches);
}

/*
 * The 'wildmode' option is changed.
 */
    char *
did_set_wildmode(optset_T *args UNUSED)
{
    if (check_opt_wim() == FAIL)
	return e_invalid_argument;
    return NULL;
}

    int
expand_set_wildmode(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_wim_values,
	    ARRAY_LENGTH(p_wim_values) - 1,
	    numMatches,
	    matches);
}

/*
 * The 'wildoptions' option is changed.
 */
    char *
did_set_wildoptions(optset_T *args UNUSED)
{
    return did_set_opt_strings(p_wop, p_wop_values, TRUE);
}

    int
expand_set_wildoptions(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_wop_values,
	    ARRAY_LENGTH(p_wop_values) - 1,
	    numMatches,
	    matches);
}

#if defined(FEAT_WAK) || defined(PROTO)
/*
 * The 'winaltkeys' option is changed.
 */
    char *
did_set_winaltkeys(optset_T *args UNUSED)
{
    char *errmsg = NULL;

    if (*p_wak == NUL || check_opt_strings(p_wak, p_wak_values, FALSE) != OK)
	errmsg = e_invalid_argument;
# ifdef FEAT_MENU
#  if defined(FEAT_GUI_MOTIF)
    else if (gui.in_use)
	gui_motif_set_mnemonics(p_wak[0] == 'y' || p_wak[0] == 'm');
#  elif defined(FEAT_GUI_GTK)
    else if (gui.in_use)
	gui_gtk_set_mnemonics(p_wak[0] == 'y' || p_wak[0] == 'm');
#  endif
# endif
    return errmsg;
}

    int
expand_set_winaltkeys(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_string(
	    args,
	    p_wak_values,
	    ARRAY_LENGTH(p_wak_values) - 1,
	    numMatches,
	    matches);
}
#endif

/*
 * The 'wincolor' option is changed.
 */
    char *
did_set_wincolor(optset_T *args UNUSED)
{
#ifdef FEAT_TERMINAL
    term_update_wincolor(curwin);
#endif
    return NULL;
}

    int
expand_set_wincolor(optexpand_T *args, int *numMatches, char_u ***matches)
{
    return expand_set_opt_generic(
	    args,
	    get_highlight_name,
	    numMatches,
	    matches);
}

#ifdef FEAT_SYN_HL
/*
 * When the 'syntax' option is set, load the syntax of that name.
 */
    static void
do_syntax_autocmd(int value_changed)
{
    static int syn_recursive = 0;

    ++syn_recursive;
    // Only pass TRUE for "force" when the value changed or not used
    // recursively, to avoid endless recurrence.
    apply_autocmds(EVENT_SYNTAX, curbuf->b_p_syn, curbuf->b_fname,
	    value_changed || syn_recursive == 1, curbuf);
    curbuf->b_flags |= BF_SYN_SET;
    --syn_recursive;
}
#endif

/*
 * When the 'filetype' option is set, trigger the FileType autocommand.
 */
    static void
do_filetype_autocmd(char_u **varp, int opt_flags, int value_changed)
{
    // Skip this when called from a modeline and the filetype was already set
    // to this value.
    if ((opt_flags & OPT_MODELINE) && !value_changed)
	return;

    static int  ft_recursive = 0;
    int	    secure_save = secure;

    // Reset the secure flag, since the value of 'filetype' has
    // been checked to be safe.
    secure = 0;

    ++ft_recursive;
    did_filetype = TRUE;
    // Only pass TRUE for "force" when the value changed or not
    // used recursively, to avoid endless recurrence.
    apply_autocmds(EVENT_FILETYPE, curbuf->b_p_ft, curbuf->b_fname,
	    value_changed || ft_recursive == 1, curbuf);
    --ft_recursive;
    // Just in case the old "curbuf" is now invalid.
    if (varp != &(curbuf->b_p_ft))
	varp = NULL;

    secure = secure_save;
}

#ifdef FEAT_SPELL
/*
 * When the 'spelllang' option is set, source the spell/LANG.vim file in
 * 'runtimepath'.
 */
    static void
do_spelllang_source(void)
{
    char_u	fname[200];
    char_u	*p;
    char_u	*q = curwin->w_s->b_p_spl;

    // Skip the first name if it is "cjk".
    if (STRNCMP(q, "cjk,", 4) == 0)
	q += 4;

    // They could set 'spellcapcheck' depending on the language.  Use the first
    // name in 'spelllang' up to '_region' or '.encoding'.
    for (p = q; *p != NUL; ++p)
	if (!ASCII_ISALNUM(*p) && *p != '-')
	    break;
    if (p > q)
    {
	vim_snprintf((char *)fname, 200, "spell/%.*s.vim",
		(int)(p - q), q);
	source_runtime(fname, DIP_ALL);
    }
}
#endif

/*
 * Handle string options that need some action to perform when changed.
 * The new value must be allocated.
 * Returns NULL for success, or an untranslated error message for an error.
 */
    char *
did_set_string_option(
    int		opt_idx,		// index in options[] table
    char_u	**varp,			// pointer to the option variable
    char_u	*oldval,		// previous value of the option
    char_u	*value,			// new value of the option
    char	*errbuf,		// buffer for errors, or NULL
    size_t	errbuflen,		// length of error buffer
    int		opt_flags,		// OPT_LOCAL and/or OPT_GLOBAL
    set_op_T    op,			// OP_ADDING/OP_PREPENDING/OP_REMOVING
    int		*value_checked)		// value was checked to be safe, no
					// need to set P_INSECURE
{
    char	*errmsg = NULL;
    long_u	free_oldval = (get_option_flags(opt_idx) & P_ALLOCED);
    opt_did_set_cb_T did_set_cb = get_option_did_set_cb(opt_idx);
    optset_T	args;

    // 'ttytype' is an alias for 'term'.  Both 'term' and 'ttytype' point to
    // T_NAME.  If 'term' or 'ttytype' is modified, then use the index for the
    // 'term' option.  Only set the P_ALLOCED flag on 'term'.
    if (varp == &T_NAME)
    {
	opt_idx = findoption((char_u *)"term");
	if (opt_idx >= 0)
	{
	    free_oldval = (get_option_flags(opt_idx) & P_ALLOCED);
	    did_set_cb = get_option_did_set_cb(opt_idx);
	}
    }

    CLEAR_FIELD(args);

    // Disallow changing some options from secure mode
    if ((secure
#ifdef HAVE_SANDBOX
		|| sandbox != 0
#endif
		) && (get_option_flags(opt_idx) & P_SECURE))
	errmsg = e_not_allowed_here;
    // Check for a "normal" directory or file name in some options.
    else if (check_illegal_path_names(opt_idx, varp))
	errmsg = e_invalid_argument;
    else if (did_set_cb != NULL)
    {
	args.os_varp = (char_u *)varp;
	args.os_idx = opt_idx;
	args.os_flags = opt_flags;
	args.os_op = op;
	args.os_oldval.string = oldval;
	args.os_newval.string = value;
	args.os_errbuf = errbuf;
	args.os_errbuflen = errbuflen;
	// Invoke the option specific callback function to validate and apply
	// the new option value.
	errmsg = did_set_cb(&args);

	// The 'keymap', 'filetype' and 'syntax' option callback functions
	// may change the os_value_checked field.
	*value_checked = args.os_value_checked;
    }

    // If an error is detected, restore the previous value.
    if (errmsg != NULL)
    {
	free_string_option(*varp);
	*varp = oldval;
	// When resetting some values, need to act on it.
	if (args.os_restore_chartab)
	    (void)init_chartab();
	if (varp == &p_hl)
	    (void)highlight_changed();
    }
    else
    {
#ifdef FEAT_EVAL
	// Remember where the option was set.
	set_option_sctx_idx(opt_idx, opt_flags, current_sctx);
#endif
	// Free string options that are in allocated memory.
	// Use "free_oldval", because recursiveness may change the flags under
	// our fingers (esp. init_highlight()).
	if (free_oldval)
	    free_string_option(oldval);
	set_option_flag(opt_idx, P_ALLOCED);

	if ((opt_flags & (OPT_LOCAL | OPT_GLOBAL)) == 0
		&& is_global_local_option(opt_idx))
	{
	    // global option with local value set to use global value; free
	    // the local value and make it empty
	    char_u *p = get_option_varp_scope(opt_idx, OPT_LOCAL);
	    free_string_option(*(char_u **)p);
	    *(char_u **)p = empty_option;
	}

	// May set global value for local option.
	else if (!(opt_flags & OPT_LOCAL) && opt_flags != OPT_GLOBAL)
	    set_string_option_global(opt_idx, varp);

	// Trigger the autocommand only after setting the flags.
#ifdef FEAT_SYN_HL
	if (varp == &(curbuf->b_p_syn))
	    do_syntax_autocmd(args.os_value_changed);
#endif
	else if (varp == &(curbuf->b_p_ft))
	    do_filetype_autocmd(varp, opt_flags, args.os_value_changed);
#ifdef FEAT_SPELL
	if (varp == &(curwin->w_s->b_p_spl))
	    do_spelllang_source();
#endif
    }

    if (varp == &p_mouse)
    {
	if (*p_mouse == NUL)
	    mch_setmouse(FALSE);    // switch mouse off
	else
	    setmouse();		    // in case 'mouse' changed
    }

#if defined(FEAT_LUA) || defined(PROTO)
    if (varp == &p_rtp)
	update_package_paths_in_lua();
#endif

#if defined(FEAT_LINEBREAK)
    // Changing Formatlistpattern when briopt includes the list setting:
    // redraw
    if ((varp == &p_flp || varp == &(curbuf->b_p_flp))
	    && curwin->w_briopt_list)
	redraw_all_later(UPD_NOT_VALID);
#endif

    if (curwin->w_curswant != MAXCOL
		   && (get_option_flags(opt_idx) & (P_CURSWANT | P_RALL)) != 0)
	curwin->w_set_curswant = TRUE;

    if ((opt_flags & OPT_NO_REDRAW) == 0)
    {
#ifdef FEAT_GUI
	// set when changing an option that only requires a redraw in the GUI
	int	redraw_gui_only = FALSE;

	if (varp == &p_go			// 'guioptions'
		|| varp == &p_guifont		// 'guifont'
# ifdef FEAT_GUI_TABLINE
		|| varp == &p_gtl		// 'guitablabel'
		|| varp == &p_gtt		// 'guitabtooltip'
# endif
# ifdef FEAT_XFONTSET
		|| varp == &p_guifontset	// 'guifontset'
# endif
		|| varp == &p_guifontwide	// 'guifontwide'
# ifdef FEAT_GUI_GTK
		|| varp == &p_guiligatures	// 'guiligatures'
# endif
	    )
	    redraw_gui_only = TRUE;

	// check redraw when it's not a GUI option or the GUI is active.
	if (!redraw_gui_only || gui.in_use)
#endif
	    check_redraw(get_option_flags(opt_idx));
    }

#if defined(FEAT_VTP) && defined(FEAT_TERMGUICOLORS)
    if (args.os_did_swaptcap)
    {
	set_termname((char_u *)"win32");
	init_highlight(TRUE, FALSE);
    }
#endif

    return errmsg;
}

/*
 * Check an option that can be a range of string values.
 *
 * Return OK for correct value, FAIL otherwise.
 * Empty is always OK.
 */
    static int
check_opt_strings(
    char_u	*val,
    char	**values,
    int		list)	    // when TRUE: accept a list of values
{
    return opt_strings_flags(val, values, NULL, list);
}

/*
 * Handle an option that can be a range of string values.
 * Set a flag in "*flagp" for each string present.
 *
 * Return OK for correct value, FAIL otherwise.
 * Empty is always OK.
 */
    static int
opt_strings_flags(
    char_u	*val,		// new value
    char	**values,	// array of valid string values
    unsigned	*flagp,
    int		list)		// when TRUE: accept a list of values
{
    int		i;
    int		len;
    unsigned	new_flags = 0;

    while (*val)
    {
	for (i = 0; ; ++i)
	{
	    if (values[i] == NULL)	// val not found in values[]
		return FAIL;

	    len = (int)STRLEN(values[i]);
	    if (STRNCMP(values[i], val, len) == 0
		    && ((list && val[len] == ',') || val[len] == NUL))
	    {
		val += len + (val[len] == ',');
		new_flags |= (1 << i);
		break;		// check next item in val list
	    }
	}
    }
    if (flagp != NULL)
	*flagp = new_flags;

    return OK;
}

/*
 * return OK if "p" is a valid fileformat name, FAIL otherwise.
 */
    int
check_ff_value(char_u *p)
{
    return check_opt_strings(p, p_ff_values, FALSE);
}

/*
 * Save the actual shortmess Flags and clear them temporarily to avoid that
 * file messages overwrites any output from the following commands.
 *
 * Caller must make sure to first call save_clear_shm_value() and then
 * restore_shm_value() exactly the same number of times.
 */
    void
save_clear_shm_value(void)
{
    if (STRLEN(p_shm) >= SHM_LEN)
    {
	iemsg(e_internal_error_shortmess_too_long);
	return;
    }

    if (++set_shm_recursive == 1)
    {
	STRCPY(shm_buf, p_shm);
	set_option_value_give_err((char_u *)"shm", 0L, (char_u *)"", 0);
    }
}

/*
 * Restore the shortmess Flags set from the save_clear_shm_value() function.
 */
    void
restore_shm_value(void)
{
    if (--set_shm_recursive == 0)
    {
	set_option_value_give_err((char_u *)"shm", 0L, shm_buf, 0);
	vim_memset(shm_buf, 0, SHM_LEN);
    }
}
