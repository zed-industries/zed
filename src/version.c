/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

#include "vim.h"

/*
 * Vim originated from Stevie version 3.6 (Fish disk 217) by GRWalter (Fred)
 * It has been changed beyond recognition since then.
 *
 * Differences between version 8.2 and 9.0 can be found with ":help version9".
 * Differences between version 7.4 and 8.x can be found with ":help version8".
 * Differences between version 6.4 and 7.x can be found with ":help version7".
 * Differences between version 5.8 and 6.x can be found with ":help version6".
 * Differences between version 4.x and 5.x can be found with ":help version5".
 * Differences between version 3.0 and 4.x can be found with ":help version4".
 * All the remarks about older versions have been removed, they are not very
 * interesting.
 */

#include "version.h"

char		*Version = VIM_VERSION_SHORT;
static char	*mediumVersion = VIM_VERSION_MEDIUM;

#if defined(HAVE_DATE_TIME) || defined(PROTO)
# if (defined(VMS) && defined(VAXC)) || defined(PROTO)
char	longVersion[sizeof(VIM_VERSION_LONG_DATE) + sizeof(__DATE__)
						      + sizeof(__TIME__) + 3];

    void
init_longVersion(void)
{
    /*
     * Construct the long version string.  Necessary because
     * VAX C can't concatenate strings in the preprocessor.
     */
    strcpy(longVersion, VIM_VERSION_LONG_DATE);
#ifdef BUILD_DATE
    strcat(longVersion, BUILD_DATE);
#else
    strcat(longVersion, __DATE__);
    strcat(longVersion, " ");
    strcat(longVersion, __TIME__);
#endif
    strcat(longVersion, ")");
}

# else
char	*longVersion = NULL;

    void
init_longVersion(void)
{
    if (longVersion != NULL)
	return;

#ifdef BUILD_DATE
    char *date_time = BUILD_DATE;
#else
    char *date_time = __DATE__ " " __TIME__;
#endif
    char *msg = _("%s (%s, compiled %s)");
    size_t len = strlen(msg)
	+ strlen(VIM_VERSION_LONG_ONLY)
	+ strlen(VIM_VERSION_DATE_ONLY)
	+ strlen(date_time);

    longVersion = alloc(len);
    if (longVersion == NULL)
	longVersion = VIM_VERSION_LONG;
    else
	vim_snprintf(longVersion, len, msg,
		VIM_VERSION_LONG_ONLY, VIM_VERSION_DATE_ONLY, date_time);
}
# endif
#else
char	*longVersion = VIM_VERSION_LONG;

    void
init_longVersion(void)
{
    // nothing to do
}
#endif

static char *(features[]) =
{
#ifdef HAVE_ACL
	"+acl",
#else
	"-acl",
#endif
#ifdef AMIGA		// only for Amiga systems
# ifdef FEAT_ARP
	"+ARP",
# else
	"-ARP",
# endif
#endif
#ifdef FEAT_ARABIC
	"+arabic",
#else
	"-arabic",
#endif
	"+autocmd",
#ifdef FEAT_AUTOCHDIR
       "+autochdir",
#else
       "-autochdir",
#endif
#ifdef FEAT_AUTOSERVERNAME
	"+autoservername",
#else
	"-autoservername",
#endif
#ifdef FEAT_BEVAL_GUI
	"+balloon_eval",
#else
	"-balloon_eval",
#endif
#ifdef FEAT_BEVAL_TERM
	"+balloon_eval_term",
#else
	"-balloon_eval_term",
#endif
#ifdef FEAT_BROWSE
	"+browse",
#else
	"-browse",
#endif
	"++builtin_terms",
#ifdef FEAT_BYTEOFF
	"+byte_offset",
#else
	"-byte_offset",
#endif
#ifdef FEAT_JOB_CHANNEL
	"+channel",
#else
	"-channel",
#endif
	"+cindent",
#ifdef FEAT_CLIENTSERVER
	"+clientserver",
#else
	"-clientserver",
#endif
#ifdef FEAT_CLIPBOARD
	"+clipboard",
#else
	"-clipboard",
#endif
	"+cmdline_compl",
	"+cmdline_hist",
	"+cmdline_info",
	"+comments",
#ifdef FEAT_CONCEAL
	"+conceal",
#else
	"-conceal",
#endif
#ifdef FEAT_CRYPT
	"+cryptv",
#else
	"-cryptv",
#endif
#ifdef FEAT_CSCOPE
	"+cscope",
#else
	"-cscope",
#endif
	"+cursorbind",
#ifdef CURSOR_SHAPE
	"+cursorshape",
#else
	"-cursorshape",
#endif
#if defined(FEAT_CON_DIALOG) && defined(FEAT_GUI_DIALOG)
	"+dialog_con_gui",
#else
# if defined(FEAT_CON_DIALOG)
	"+dialog_con",
# else
#  if defined(FEAT_GUI_DIALOG)
	"+dialog_gui",
#  else
	"-dialog",
#  endif
# endif
#endif
#ifdef FEAT_DIFF
	"+diff",
#else
	"-diff",
#endif
#ifdef FEAT_DIGRAPHS
	"+digraphs",
#else
	"-digraphs",
#endif
#ifdef FEAT_GUI_MSWIN
# ifdef FEAT_DIRECTX
	"+directx",
# else
	"-directx",
# endif
#endif
#ifdef FEAT_DND
	"+dnd",
#else
	"-dnd",
#endif
	"-ebcdic",
#ifdef FEAT_EMACS_TAGS
	"+emacs_tags",
#else
	"-emacs_tags",
#endif
#ifdef FEAT_EVAL
	"+eval",
#else
	"-eval",
#endif
	"+ex_extra",
#ifdef FEAT_SEARCH_EXTRA
	"+extra_search",
#else
	"-extra_search",
#endif
	"-farsi",
	"+file_in_path",
#ifdef FEAT_FIND_ID
	"+find_in_path",
#else
	"-find_in_path",
#endif
	"+float",
#ifdef FEAT_FOLDING
	"+folding",
#else
	"-folding",
#endif
	"-footer",
	// only interesting on Unix systems
#if !defined(USE_SYSTEM) && defined(UNIX)
	"+fork()",
#endif
#ifdef FEAT_GETTEXT
# ifdef DYNAMIC_GETTEXT
	"+gettext/dyn",
# else
	"+gettext",
# endif
#else
	"-gettext",
#endif
	"-hangul_input",
#if (defined(HAVE_ICONV_H) && defined(USE_ICONV)) || defined(DYNAMIC_ICONV)
# ifdef DYNAMIC_ICONV
	"+iconv/dyn",
# else
	"+iconv",
# endif
#else
	"-iconv",
#endif
	"+insert_expand",
#ifdef FEAT_IPV6
	"+ipv6",
#else
	"-ipv6",
#endif
#ifdef FEAT_JOB_CHANNEL
	"+job",
#else
	"-job",
#endif
	"+jumplist",
#ifdef FEAT_KEYMAP
	"+keymap",
#else
	"-keymap",
#endif
#ifdef FEAT_EVAL
	"+lambda",
#else
	"-lambda",
#endif
#ifdef FEAT_LANGMAP
	"+langmap",
#else
	"-langmap",
#endif
#ifdef FEAT_LIBCALL
	"+libcall",
#else
	"-libcall",
#endif
#ifdef FEAT_LINEBREAK
	"+linebreak",
#else
	"-linebreak",
#endif
	"+lispindent",
	"+listcmds",
	"+localmap",
#ifdef FEAT_LUA
# ifdef DYNAMIC_LUA
	"+lua/dyn",
# else
	"+lua",
# endif
#else
	"-lua",
#endif
#ifdef FEAT_MENU
	"+menu",
#else
	"-menu",
#endif
#ifdef FEAT_SESSION
	"+mksession",
#else
	"-mksession",
#endif
	"+modify_fname",
	"+mouse",
#ifdef FEAT_MOUSESHAPE
	"+mouseshape",
#else
	"-mouseshape",
#endif

#if defined(UNIX) || defined(VMS)
# ifdef FEAT_MOUSE_DEC
	"+mouse_dec",
# else
	"-mouse_dec",
# endif
# ifdef FEAT_MOUSE_GPM
#  ifdef DYNAMIC_GPM
	"+mouse_gpm/dyn",
#  else
	"+mouse_gpm",
#  endif
# else
	"-mouse_gpm",
# endif
# ifdef FEAT_MOUSE_JSB
	"+mouse_jsbterm",
# else
	"-mouse_jsbterm",
# endif
# ifdef FEAT_MOUSE_NET
	"+mouse_netterm",
# else
	"-mouse_netterm",
# endif
#endif

#ifdef __QNX__
# ifdef FEAT_MOUSE_PTERM
	"+mouse_pterm",
# else
	"-mouse_pterm",
# endif
#endif

#if defined(UNIX) || defined(VMS)
	"+mouse_sgr",
# ifdef FEAT_SYSMOUSE
	"+mouse_sysmouse",
# else
	"-mouse_sysmouse",
# endif
# ifdef FEAT_MOUSE_URXVT
	"+mouse_urxvt",
# else
	"-mouse_urxvt",
# endif
	"+mouse_xterm",
#endif

#ifdef FEAT_MBYTE_IME
# ifdef DYNAMIC_IME
	"+multi_byte_ime/dyn",
# else
	"+multi_byte_ime",
# endif
#else
	"+multi_byte",
#endif
#ifdef FEAT_MULTI_LANG
	"+multi_lang",
#else
	"-multi_lang",
#endif
#ifdef FEAT_MZSCHEME
# ifdef DYNAMIC_MZSCHEME
	"+mzscheme/dyn",
# else
	"+mzscheme",
# endif
#else
	"-mzscheme",
#endif
#ifdef FEAT_NETBEANS_INTG
	"+netbeans_intg",
#else
	"-netbeans_intg",
#endif
	"+num64",
#ifdef FEAT_GUI_MSWIN
# ifdef FEAT_OLE
	"+ole",
# else
	"-ole",
# endif
#endif
#ifdef FEAT_EVAL
	"+packages",
#else
	"-packages",
#endif
	"+path_extra",
#ifdef FEAT_PERL
# ifdef DYNAMIC_PERL
	"+perl/dyn",
# else
	"+perl",
# endif
#else
	"-perl",
#endif
#ifdef FEAT_PERSISTENT_UNDO
	"+persistent_undo",
#else
	"-persistent_undo",
#endif
#ifdef FEAT_PROP_POPUP
	"+popupwin",
#else
	"-popupwin",
#endif
#ifdef FEAT_PRINTER
# ifdef FEAT_POSTSCRIPT
	"+postscript",
# else
	"-postscript",
# endif
	"+printer",
#else
	"-printer",
#endif
#ifdef FEAT_PROFILE
	"+profile",
#else
	"-profile",
#endif
#ifdef FEAT_PYTHON
# ifdef DYNAMIC_PYTHON
	"+python/dyn",
# else
	"+python",
# endif
#else
	"-python",
#endif
#ifdef FEAT_PYTHON3
# ifdef DYNAMIC_PYTHON3
#  ifdef DYNAMIC_PYTHON3_STABLE_ABI
	"+python3/dyn-stable",
#  else
	"+python3/dyn",
#  endif
# else
	"+python3",
# endif
#else
	"-python3",
#endif
#ifdef FEAT_QUICKFIX
	"+quickfix",
#else
	"-quickfix",
#endif
#ifdef FEAT_RELTIME
	"+reltime",
#else
	"-reltime",
#endif
#ifdef FEAT_RIGHTLEFT
	"+rightleft",
#else
	"-rightleft",
#endif
#ifdef FEAT_RUBY
# ifdef DYNAMIC_RUBY
	"+ruby/dyn",
# else
	"+ruby",
# endif
#else
	"-ruby",
#endif
	"+scrollbind",
#ifdef FEAT_SIGNS
	"+signs",
#else
	"-signs",
#endif
	"+smartindent",
#ifdef FEAT_SODIUM
# ifdef DYNAMIC_SODIUM
	"+sodium/dyn",
# else
	"+sodium",
# endif
#else
	"-sodium",
#endif
#ifdef FEAT_SOUND
	"+sound",
#else
	"-sound",
#endif
#ifdef FEAT_SPELL
	"+spell",
#else
	"-spell",
#endif
#ifdef STARTUPTIME
	"+startuptime",
#else
	"-startuptime",
#endif
#ifdef FEAT_STL_OPT
	"+statusline",
#else
	"-statusline",
#endif
	"-sun_workshop",
#ifdef FEAT_SYN_HL
	"+syntax",
#else
	"-syntax",
#endif
	    // only interesting on Unix systems
#if defined(USE_SYSTEM) && defined(UNIX)
	"+system()",
#endif
	"+tag_binary",
	"-tag_old_static",
	"-tag_any_white",
#ifdef FEAT_TCL
# ifdef DYNAMIC_TCL
	"+tcl/dyn",
# else
	"+tcl",
# endif
#else
	"-tcl",
#endif
#ifdef FEAT_TERMGUICOLORS
	"+termguicolors",
#else
	"-termguicolors",
#endif
#ifdef FEAT_TERMINAL
	"+terminal",
#else
	"-terminal",
#endif
#if defined(UNIX)
// only Unix can have terminfo instead of termcap
# ifdef TERMINFO
	"+terminfo",
# else
	"-terminfo",
# endif
#endif
#ifdef FEAT_TERMRESPONSE
	"+termresponse",
#else
	"-termresponse",
#endif
	"+textobjects",
#ifdef FEAT_PROP_POPUP
	"+textprop",
#else
	"-textprop",
#endif
#if !defined(UNIX)
// unix always includes termcap support
# ifdef HAVE_TGETENT
	"+tgetent",
# else
	"-tgetent",
# endif
#endif
#ifdef FEAT_TIMERS
	"+timers",
#else
	"-timers",
#endif
	"+title",
#ifdef FEAT_TOOLBAR
	"+toolbar",
#else
	"-toolbar",
#endif
	"+user_commands",
#ifdef FEAT_VARTABS
	"+vartabs",
#else
	"-vartabs",
#endif
	"+vertsplit",
	"+vim9script",
#ifdef FEAT_VIMINFO
	"+viminfo",
#else
	"-viminfo",
#endif
	"+virtualedit",
	"+visual",
	"+visualextra",
	"+vreplace",
#ifdef MSWIN
# ifdef FEAT_VTP
	"+vtp",
# else
	"-vtp",
# endif
#endif
	"+wildignore",
	"+wildmenu",
	"+windows",
#ifdef FEAT_WRITEBACKUP
	"+writebackup",
#else
	"-writebackup",
#endif
#if defined(UNIX) || defined(VMS)
# ifdef FEAT_X11
	"+X11",
# else
	"-X11",
# endif
#endif
# ifdef FEAT_XATTR
	"+xattr",
# else
	"-xattr",
# endif
#ifdef FEAT_XFONTSET
	"+xfontset",
#else
	"-xfontset",
#endif
#ifdef FEAT_XIM
	"+xim",
#else
	"-xim",
#endif
#if defined(MSWIN)
# ifdef FEAT_XPM_W32
	"+xpm_w32",
# else
	"-xpm_w32",
# endif
#elif defined(HAVE_XPM)
	"+xpm",
#else
	"-xpm",
#endif
#if defined(UNIX) || defined(VMS)
# if defined(USE_XSMP_INTERACT)
	"+xsmp_interact",
# elif defined(USE_XSMP)
	"+xsmp",
# else
	"-xsmp",
# endif
# ifdef FEAT_XCLIPBOARD
	"+xterm_clipboard",
# else
	"-xterm_clipboard",
# endif
#endif
#ifdef FEAT_XTERM_SAVE
	"+xterm_save",
#else
	"-xterm_save",
#endif
	NULL
};

static int included_patches[] =
{   /* Add new patch number below this line */
/**/
    65,
/**/
    64,
/**/
    63,
/**/
    62,
/**/
    61,
/**/
    60,
/**/
    59,
/**/
    58,
/**/
    57,
/**/
    56,
/**/
    55,
/**/
    54,
/**/
    53,
/**/
    52,
/**/
    51,
/**/
    50,
/**/
    49,
/**/
    48,
/**/
    47,
/**/
    46,
/**/
    45,
/**/
    44,
/**/
    43,
/**/
    42,
/**/
    41,
/**/
    40,
/**/
    39,
/**/
    38,
/**/
    37,
/**/
    36,
/**/
    35,
/**/
    34,
/**/
    33,
/**/
    32,
/**/
    31,
/**/
    30,
/**/
    29,
/**/
    28,
/**/
    27,
/**/
    26,
/**/
    25,
/**/
    24,
/**/
    23,
/**/
    22,
/**/
    21,
/**/
    20,
/**/
    19,
/**/
    18,
/**/
    17,
/**/
    16,
/**/
    15,
/**/
    14,
/**/
    13,
/**/
    12,
/**/
    11,
/**/
    10,
/**/
    9,
/**/
    8,
/**/
    7,
/**/
    6,
/**/
    5,
/**/
    4,
/**/
    3,
/**/
    2,
/**/
    1,
/**/
    0
};

/*
 * Place to put a short description when adding a feature with a patch.
 * Keep it short, e.g.,: "relative numbers", "persistent undo".
 * Also add a comment marker to separate the lines.
 * See the official Vim patches for the diff format: It must use a context of
 * one line only.  Create it by hand or use "diff -C2" and edit the patch.
 */
static char *(extra_patches[]) =
{   /* Add your patch description below this line */
/**/
    NULL
};

    int
highest_patch(void)
{
    // this relies on the highest patch number to be the first entry
    return included_patches[0];
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return TRUE if patch "n" has been included.
 */
    int
has_patch(int n)
{
    int		h, m, l;

    // Perform a binary search.
    l = 0;
    h = (int)ARRAY_LENGTH(included_patches) - 1;
    for (;;)
    {
	m = (l + h) / 2;
	if (included_patches[m] == n)
	    return TRUE;
	if (l == h)
	    break;
	if (included_patches[m] < n)
	    h = m;
	else
	    l = m + 1;
    }
    return FALSE;
}
#endif

    void
ex_version(exarg_T *eap)
{
    /*
     * Ignore a ":version 9.99" command.
     */
    if (*eap->arg == NUL)
    {
	msg_putchar('\n');
	list_version();
    }
}

/*
 * Output a string for the version message.  If it's going to wrap, output a
 * newline, unless the message is too long to fit on the screen anyway.
 * When "wrap" is TRUE wrap the string in [].
 */
    static void
version_msg_wrap(char_u *s, int wrap)
{
    int		len = vim_strsize(s) + (wrap ? 2 : 0);

    if (!got_int && len < (int)Columns && msg_col + len >= (int)Columns
								&& *s != '\n')
	msg_putchar('\n');
    if (!got_int)
    {
	if (wrap)
	    msg_puts("[");
	msg_puts((char *)s);
	if (wrap)
	    msg_puts("]");
    }
}

    static void
version_msg(char *s)
{
    version_msg_wrap((char_u *)s, FALSE);
}

/*
 * List all features aligned in columns, dictionary style.
 */
    static void
list_features(void)
{
    list_in_columns((char_u **)features, -1, -1);
}

/*
 * List string items nicely aligned in columns.
 * When "size" is < 0 then the last entry is marked with NULL.
 * The entry with index "current" is inclosed in [].
 */
    void
list_in_columns(char_u **items, int size, int current)
{
    int		i;
    int		ncol;
    int		nrow;
    int		cur_row = 1;
    int		item_count = 0;
    int		width = 0;
#ifdef FEAT_SYN_HL
    int		use_highlight = (items == (char_u **)features);
#endif

    // Find the length of the longest item, use that + 1 as the column
    // width.
    for (i = 0; size < 0 ? items[i] != NULL : i < size; ++i)
    {
	int l = vim_strsize(items[i]) + (i == current ? 2 : 0);

	if (l > width)
	    width = l;
	++item_count;
    }
    width += 1;

    if (Columns < width)
    {
	// Not enough screen columns - show one per line
	for (i = 0; i < item_count; ++i)
	{
	    version_msg_wrap(items[i], i == current);
	    if (msg_col > 0 && i < item_count - 1)
		msg_putchar('\n');
	}
	return;
    }

    // The rightmost column doesn't need a separator.
    // Sacrifice it to fit in one more column if possible.
    ncol = (int) (Columns + 1) / width;
    nrow = item_count / ncol + ((item_count % ncol) ? 1 : 0);

    // "i" counts columns then rows.  "idx" counts rows then columns.
    for (i = 0; !got_int && i < nrow * ncol; ++i)
    {
	int idx = (i / ncol) + (i % ncol) * nrow;

	if (idx < item_count)
	{
	    int last_col = (i + 1) % ncol == 0;

	    if (idx == current)
		msg_putchar('[');
#ifdef FEAT_SYN_HL
	    if (use_highlight && items[idx][0] == '-')
		msg_puts_attr((char *)items[idx], HL_ATTR(HLF_W));
	    else
#endif
		msg_puts((char *)items[idx]);
	    if (idx == current)
		msg_putchar(']');
	    if (last_col)
	    {
		if (msg_col > 0 && cur_row < nrow)
		    msg_putchar('\n');
		++cur_row;
	    }
	    else
	    {
		while (msg_col % width)
		    msg_putchar(' ');
	    }
	}
	else
	{
	    // this row is out of items, thus at the end of the row
	    if (msg_col > 0)
	    {
		if (cur_row < nrow)
		    msg_putchar('\n');
		++cur_row;
	    }
	}
    }
}

    void
list_version(void)
{
    int		i;
    int		first;
    char	*s = "";

    /*
     * When adding features here, don't forget to update the list of
     * internal variables in eval.c!
     */
    init_longVersion();
    msg(longVersion);
#ifdef MSWIN
# ifdef FEAT_GUI_MSWIN
#  ifdef VIMDLL
#   ifdef _WIN64
    msg_puts(_("\nMS-Windows 64-bit GUI/console version"));
#   else
    msg_puts(_("\nMS-Windows 32-bit GUI/console version"));
#   endif
#  else
#   ifdef _WIN64
    msg_puts(_("\nMS-Windows 64-bit GUI version"));
#   else
    msg_puts(_("\nMS-Windows 32-bit GUI version"));
#   endif
#  endif
#  ifdef FEAT_OLE
    msg_puts(_(" with OLE support"));
#  endif
# else
#  ifdef _WIN64
    msg_puts(_("\nMS-Windows 64-bit console version"));
#  else
    msg_puts(_("\nMS-Windows 32-bit console version"));
#  endif
# endif
#endif
#if defined(MACOS_X)
# if defined(MACOS_X_DARWIN)
    msg_puts(_("\nmacOS version"));
# else
    msg_puts(_("\nmacOS version w/o darwin feat."));
# endif
# if defined(__arm64__)
    msg_puts(" - arm64");
# elif defined(__x86_64__)
    msg_puts(" - x86_64");
# endif
#endif

#ifdef VMS
    msg_puts(_("\nOpenVMS version"));
# ifdef HAVE_PATHDEF
    if (*compiled_arch != NUL)
    {
	msg_puts(" - ");
	msg_puts((char *)compiled_arch);
    }
# endif

#endif

    // Print the list of patch numbers if there is at least one.
    // Print a range when patches are consecutive: "1-10, 12, 15-40, 42-45"
    if (included_patches[0] != 0)
    {
	msg_puts(_("\nIncluded patches: "));
	first = -1;
	i = (int)ARRAY_LENGTH(included_patches) - 1;
	while (--i >= 0)
	{
	    if (first < 0)
		first = included_patches[i];
	    if (i == 0 || included_patches[i - 1] != included_patches[i] + 1)
	    {
		msg_puts(s);
		s = ", ";
		msg_outnum((long)first);
		if (first != included_patches[i])
		{
		    msg_puts("-");
		    msg_outnum((long)included_patches[i]);
		}
		first = -1;
	    }
	}
    }

    // Print the list of extra patch descriptions if there is at least one.
    if (extra_patches[0] != NULL)
    {
	msg_puts(_("\nExtra patches: "));
	s = "";
	for (i = 0; extra_patches[i] != NULL; ++i)
	{
	    msg_puts(s);
	    s = ", ";
	    msg_puts(extra_patches[i]);
	}
    }

#ifdef MODIFIED_BY
    msg_puts("\n");
    msg_puts(_("Modified by "));
    msg_puts(MODIFIED_BY);
#endif

#ifdef HAVE_PATHDEF
    if (*compiled_user != NUL || *compiled_sys != NUL)
    {
	msg_puts(_("\nCompiled "));
	if (*compiled_user != NUL)
	{
	    msg_puts(_("by "));
	    msg_puts((char *)compiled_user);
	}
	if (*compiled_sys != NUL)
	{
	    msg_puts("@");
	    msg_puts((char *)compiled_sys);
	}
    }
#endif

#if defined(FEAT_HUGE)
    msg_puts(_("\nHuge version "));
#elif defined(FEAT_NORMAL)
    msg_puts(_("\nNormal version "));
#else
    msg_puts(_("\nTiny version "));
#endif
#if !defined(FEAT_GUI)
    msg_puts(_("without GUI."));
#elif defined(FEAT_GUI_GTK)
# if defined(USE_GTK3)
    msg_puts(_("with GTK3 GUI."));
# elif defined(FEAT_GUI_GNOME)
     msg_puts(_("with GTK2-GNOME GUI."));
# else
     msg_puts(_("with GTK2 GUI."));
# endif
#elif defined(FEAT_GUI_MOTIF)
    msg_puts(_("with X11-Motif GUI."));
#elif defined(FEAT_GUI_HAIKU)
    msg_puts(_("with Haiku GUI."));
#elif defined(FEAT_GUI_PHOTON)
    msg_puts(_("with Photon GUI."));
#elif defined(MSWIN)
    msg_puts(_("with GUI."));
#endif
    version_msg(_("  Features included (+) or not (-):\n"));

    list_features();
    if (msg_col > 0)
	msg_putchar('\n');

#ifdef SYS_VIMRC_FILE
    version_msg(_("   system vimrc file: \""));
    version_msg(SYS_VIMRC_FILE);
    version_msg("\"\n");
#endif
#ifdef USR_VIMRC_FILE
    version_msg(_("     user vimrc file: \""));
    version_msg(USR_VIMRC_FILE);
    version_msg("\"\n");
#endif
#ifdef USR_VIMRC_FILE2
    version_msg(_(" 2nd user vimrc file: \""));
    version_msg(USR_VIMRC_FILE2);
    version_msg("\"\n");
#endif
#ifdef USR_VIMRC_FILE3
    version_msg(_(" 3rd user vimrc file: \""));
    version_msg(USR_VIMRC_FILE3);
    version_msg("\"\n");
#endif
#ifdef USR_EXRC_FILE
    version_msg(_("      user exrc file: \""));
    version_msg(USR_EXRC_FILE);
    version_msg("\"\n");
#endif
#ifdef USR_EXRC_FILE2
    version_msg(_("  2nd user exrc file: \""));
    version_msg(USR_EXRC_FILE2);
    version_msg("\"\n");
#endif
#ifdef FEAT_GUI
# ifdef SYS_GVIMRC_FILE
    version_msg(_("  system gvimrc file: \""));
    version_msg(SYS_GVIMRC_FILE);
    version_msg("\"\n");
# endif
    version_msg(_("    user gvimrc file: \""));
    version_msg(USR_GVIMRC_FILE);
    version_msg("\"\n");
# ifdef USR_GVIMRC_FILE2
    version_msg(_("2nd user gvimrc file: \""));
    version_msg(USR_GVIMRC_FILE2);
    version_msg("\"\n");
# endif
# ifdef USR_GVIMRC_FILE3
    version_msg(_("3rd user gvimrc file: \""));
    version_msg(USR_GVIMRC_FILE3);
    version_msg("\"\n");
# endif
#endif
    version_msg(_("       defaults file: \""));
    version_msg(VIM_DEFAULTS_FILE);
    version_msg("\"\n");
#ifdef FEAT_GUI
# ifdef SYS_MENU_FILE
    version_msg(_("    system menu file: \""));
    version_msg(SYS_MENU_FILE);
    version_msg("\"\n");
# endif
#endif
#ifdef HAVE_PATHDEF
    if (*default_vim_dir != NUL)
    {
	version_msg(_("  fall-back for $VIM: \""));
	version_msg((char *)default_vim_dir);
	version_msg("\"\n");
    }
    if (*default_vimruntime_dir != NUL)
    {
	version_msg(_(" f-b for $VIMRUNTIME: \""));
	version_msg((char *)default_vimruntime_dir);
	version_msg("\"\n");
    }
    version_msg(_("Compilation: "));
    version_msg((char *)all_cflags);
    version_msg("\n");
#ifdef VMS
    if (*compiler_version != NUL)
    {
	version_msg(_("Compiler: "));
	version_msg((char *)compiler_version);
	version_msg("\n");
    }
#endif
    version_msg(_("Linking: "));
    version_msg((char *)all_lflags);
#endif
#ifdef DEBUG
    version_msg("\n");
    version_msg(_("  DEBUG BUILD"));
#endif
}

static void do_intro_line(int row, char_u *mesg, int add_version, int attr);
static void intro_message(int colon);

/*
 * Show the intro message when not editing a file.
 */
    void
maybe_intro_message(void)
{
    if (BUFEMPTY()
	    && curbuf->b_fname == NULL
	    && firstwin->w_next == NULL
	    && vim_strchr(p_shm, SHM_INTRO) == NULL)
	intro_message(FALSE);
}

/*
 * Give an introductory message about Vim.
 * Only used when starting Vim on an empty file, without a file name.
 * Or with the ":intro" command (for Sven :-).
 */
    static void
intro_message(
    int		colon)		// TRUE for ":intro"
{
    int		i;
    int		row;
    int		blanklines;
    int		sponsor;
    char	*p;
    static char	*(lines[]) =
    {
	N_("VIM - Vi IMproved"),
	"",
	N_("version "),
	N_("by Bram Moolenaar et al."),
#ifdef MODIFIED_BY
	" ",
#endif
	N_("Vim is open source and freely distributable"),
	"",
	N_("Help poor children in Uganda!"),
	N_("type  :help iccf<Enter>       for information "),
	"",
	N_("type  :q<Enter>               to exit         "),
	N_("type  :help<Enter>  or  <F1>  for on-line help"),
	N_("type  :help version9<Enter>   for version info"),
	NULL,
	"",
	N_("Running in Vi compatible mode"),
	N_("type  :set nocp<Enter>        for Vim defaults"),
	N_("type  :help cp-default<Enter> for info on this"),
    };
#ifdef FEAT_GUI
    static char	*(gui_lines[]) =
    {
	NULL,
	NULL,
	NULL,
	NULL,
#ifdef MODIFIED_BY
	NULL,
#endif
	NULL,
	NULL,
	NULL,
	N_("menu  Help->Orphans           for information    "),
	NULL,
	N_("Running modeless, typed text is inserted"),
	N_("menu  Edit->Global Settings->Toggle Insert Mode  "),
	N_("                              for two modes      "),
	NULL,
	NULL,
	NULL,
	N_("menu  Edit->Global Settings->Toggle Vi Compatible"),
	N_("                              for Vim defaults   "),
    };
#endif

    // blanklines = screen height - # message lines
    blanklines = (int)Rows - (ARRAY_LENGTH(lines) - 1);
    if (!p_cp)
	blanklines += 4;  // add 4 for not showing "Vi compatible" message

    // Don't overwrite a statusline.  Depends on 'cmdheight'.
    if (p_ls > 1)
	blanklines -= Rows - topframe->fr_height;
    if (blanklines < 0)
	blanklines = 0;

    // Show the sponsor and register message one out of four times, the Uganda
    // message two out of four times.
    sponsor = (int)time(NULL);
    sponsor = ((sponsor & 2) == 0) - ((sponsor & 4) == 0);

    // start displaying the message lines after half of the blank lines
    row = blanklines / 2;
    if ((row >= 2 && Columns >= 50) || colon)
    {
	for (i = 0; i < (int)ARRAY_LENGTH(lines); ++i)
	{
	    p = lines[i];
#ifdef FEAT_GUI
	    if (p_im && gui.in_use && gui_lines[i] != NULL)
		p = gui_lines[i];
#endif
	    if (p == NULL)
	    {
		if (!p_cp)
		    break;
		continue;
	    }
	    if (sponsor != 0)
	    {
		if (strstr(p, "children") != NULL)
		    p = sponsor < 0
			? N_("Sponsor Vim development!")
			: N_("Become a registered Vim user!");
		else if (strstr(p, "iccf") != NULL)
		    p = sponsor < 0
			? N_("type  :help sponsor<Enter>    for information ")
			: N_("type  :help register<Enter>   for information ");
		else if (strstr(p, "Orphans") != NULL)
		    p = N_("menu  Help->Sponsor/Register  for information    ");
	    }
	    if (*p != NUL)
		do_intro_line(row, (char_u *)_(p), i == 2, 0);
	    ++row;
	}
    }

    // Make the wait-return message appear just below the text.
    if (colon)
	msg_row = row;
}

    static void
do_intro_line(
    int		row,
    char_u	*mesg,
    int		add_version,
    int		attr)
{
    char_u	vers[20];
    int		col;
    char_u	*p;
    int		l;
    int		clen;
#ifdef MODIFIED_BY
# define MODBY_LEN 150
    char_u	modby[MODBY_LEN];

    if (*mesg == ' ')
    {
	vim_strncpy(modby, (char_u *)_("Modified by "), MODBY_LEN - 1);
	l = (int)STRLEN(modby);
	vim_strncpy(modby + l, (char_u *)MODIFIED_BY, MODBY_LEN - l - 1);
	mesg = modby;
    }
#endif

    // Center the message horizontally.
    col = vim_strsize(mesg);
    if (add_version)
    {
	STRCPY(vers, mediumVersion);
	if (highest_patch())
	{
	    // Check for 9.9x or 9.9xx, alpha/beta version
	    if (SAFE_isalpha((int)vers[3]))
	    {
		int len = (SAFE_isalpha((int)vers[4])) ? 5 : 4;
		sprintf((char *)vers + len, ".%d%s", highest_patch(),
							 mediumVersion + len);
	    }
	    else
		sprintf((char *)vers + 3, ".%d", highest_patch());
	}
	col += (int)STRLEN(vers);
    }
    col = (Columns - col) / 2;
    if (col < 0)
	col = 0;

    // Split up in parts to highlight <> items differently.
    for (p = mesg; *p != NUL; p += l)
    {
	clen = 0;
	for (l = 0; p[l] != NUL
			 && (l == 0 || (p[l] != '<' && p[l - 1] != '>')); ++l)
	{
	    if (has_mbyte)
	    {
		clen += ptr2cells(p + l);
		l += (*mb_ptr2len)(p + l) - 1;
	    }
	    else
		clen += byte2cells(p[l]);
	}
	screen_puts_len(p, l, row, col, *p == '<' ? HL_ATTR(HLF_8) : attr);
	col += clen;
    }

    // Add the version number to the version line.
    if (add_version)
	screen_puts(vers, row, col, 0);
}

/*
 * ":intro": clear screen, display intro screen and wait for return.
 */
    void
ex_intro(exarg_T *eap UNUSED)
{
    screenclear();
    intro_message(TRUE);
    wait_return(TRUE);
}
