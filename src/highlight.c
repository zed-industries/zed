/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * Highlighting stuff.
 */

#include "vim.h"

#define SG_TERM		1	// term has been set
#define SG_CTERM	2	// cterm has been set
#define SG_GUI		4	// gui has been set
#define SG_LINK		8	// link has been set

#define MAX_SYN_NAME	200

/*
 * The "term", "cterm" and "gui" arguments can be any combination of the
 * following names, separated by commas (but no spaces!).
 */
static char *(hl_name_table[]) =
    {"bold", "standout", "underline",
	"undercurl", "underdouble", "underdotted", "underdashed",
	"italic", "reverse", "inverse", "nocombine", "strikethrough", "NONE"};
static int hl_attr_table[] =
    {HL_BOLD, HL_STANDOUT, HL_UNDERLINE,
	HL_UNDERCURL, HL_UNDERDOUBLE, HL_UNDERDOTTED, HL_UNDERDASHED,
	HL_ITALIC, HL_INVERSE, HL_INVERSE, HL_NOCOMBINE, HL_STRIKETHROUGH, 0};
// length of all attribute names, plus commas, together (and a bit more)
#define MAX_ATTR_LEN 120

#define ATTR_COMBINE(attr_a, attr_b) ((((attr_b) & HL_NOCOMBINE) ? (attr_b) : (attr_a)) | (attr_b))

/*
 * Structure that stores information about a highlight group.
 * The ID of a highlight group is also called group ID.  It is the index in
 * the highlight_ga array PLUS ONE.
 */
typedef struct
{
    char_u	*sg_name;	// highlight group name
    char_u	*sg_name_u;	// uppercase of sg_name
    int		sg_cleared;	// "hi clear" was used
// for normal terminals
    int		sg_term;	// "term=" highlighting attributes
    char_u	*sg_start;	// terminal string for start highl
    char_u	*sg_stop;	// terminal string for stop highl
    int		sg_term_attr;	// Screen attr for term mode
// for color terminals
    int		sg_cterm;	// "cterm=" highlighting attr
    int		sg_cterm_bold;	// bold attr was set for light color
    int		sg_cterm_fg;	// terminal fg color number + 1
    int		sg_cterm_bg;	// terminal bg color number + 1
    int		sg_cterm_ul;	// terminal ul color number + 1
    int		sg_cterm_attr;	// Screen attr for color term mode
    int		sg_cterm_font;	// terminal alternative font (0 for normal)
// for when using the GUI
#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    guicolor_T	sg_gui_fg;	// GUI foreground color handle
    guicolor_T	sg_gui_bg;	// GUI background color handle
    guicolor_T	sg_gui_sp;	// GUI special color handle
#endif
#ifdef FEAT_GUI
    GuiFont	sg_font;	// GUI font handle
#ifdef FEAT_XFONTSET
    GuiFontset	sg_fontset;	// GUI fontset handle
#endif
    char_u	*sg_font_name;  // GUI font or fontset name
    int		sg_gui_attr;    // Screen attr for GUI mode
#endif
#if defined(FEAT_GUI) || defined(FEAT_EVAL)
// Store the sp color name for the GUI or synIDattr()
    int		sg_gui;		// "gui=" highlighting attributes
    char_u	*sg_gui_fg_name;// GUI foreground color name
    char_u	*sg_gui_bg_name;// GUI background color name
    char_u	*sg_gui_sp_name;// GUI special color name
#endif
    int		sg_link;	// link to this highlight group ID
    int		sg_deflink;	// default link; restored in highlight_clear()
    int		sg_set;		// combination of SG_* flags
#ifdef FEAT_EVAL
    sctx_T	sg_deflink_sctx;  // script where the default link was set
    sctx_T	sg_script_ctx;	// script in which the group was last set
#endif
} hl_group_T;

// highlight groups for 'highlight' option
static garray_T highlight_ga;
#define HL_TABLE()	((hl_group_T *)((highlight_ga.ga_data)))

/*
 * An attribute number is the index in attr_table plus ATTR_OFF.
 */
#define ATTR_OFF (HL_ALL + 1)

static void syn_unadd_group(void);
static void set_hl_attr(int idx);
static void highlight_list_one(int id);
static int highlight_list_arg(int id, int didh, int type, int iarg, char_u *sarg, char *name);
static int syn_add_group(char_u *name);
static int hl_has_settings(int idx, int check_link);
static void highlight_clear(int idx);

#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
static void gui_do_one_color(int idx, int do_menu, int do_tooltip);
#endif
#ifdef FEAT_GUI
static int  set_group_colors(char_u *name, guicolor_T *fgp, guicolor_T *bgp, int do_menu, int use_norm, int do_tooltip);
static void hl_do_font(int idx, char_u *arg, int do_normal, int do_menu, int do_tooltip, int free_font);
#endif

/*
 * The default highlight groups.  These are compiled-in for fast startup and
 * they still work when the runtime files can't be found.
 * When making changes here, also change runtime/colors/default.vim!
 * The #ifdefs are needed to reduce the amount of static data.  Helps to make
 * the 16 bit DOS (museum) version compile.
 */
#if defined(FEAT_GUI) || defined(FEAT_EVAL)
# define CENT(a, b) b
#else
# define CENT(a, b) a
#endif
static char *(highlight_init_both[]) = {
    CENT("ErrorMsg term=standout ctermbg=DarkRed ctermfg=White",
	 "ErrorMsg term=standout ctermbg=DarkRed ctermfg=White guibg=Red guifg=White"),
    CENT("IncSearch term=reverse cterm=reverse",
	 "IncSearch term=reverse cterm=reverse gui=reverse"),
    CENT("ModeMsg term=bold cterm=bold",
	 "ModeMsg term=bold cterm=bold gui=bold"),
    CENT("NonText term=bold ctermfg=Blue",
	 "NonText term=bold ctermfg=Blue gui=bold guifg=Blue"),
    CENT("StatusLine term=reverse,bold cterm=reverse,bold",
	 "StatusLine term=reverse,bold cterm=reverse,bold gui=reverse,bold"),
    CENT("StatusLineNC term=reverse cterm=reverse",
	 "StatusLineNC term=reverse cterm=reverse gui=reverse"),
    "default link EndOfBuffer NonText",
    CENT("VertSplit term=reverse cterm=reverse",
	 "VertSplit term=reverse cterm=reverse gui=reverse"),
#ifdef FEAT_CLIPBOARD
    CENT("VisualNOS term=underline,bold cterm=underline,bold",
	 "VisualNOS term=underline,bold cterm=underline,bold gui=underline,bold"),
#endif
#ifdef FEAT_DIFF
    CENT("DiffText term=reverse cterm=bold ctermbg=Red",
	 "DiffText term=reverse cterm=bold ctermbg=Red gui=bold guibg=Red"),
#endif
    CENT("PmenuSbar ctermbg=Grey",
	 "PmenuSbar ctermbg=Grey guibg=Grey"),
    CENT("TabLineSel term=bold cterm=bold",
	 "TabLineSel term=bold cterm=bold gui=bold"),
    CENT("TabLineFill term=reverse cterm=reverse",
	 "TabLineFill term=reverse cterm=reverse gui=reverse"),
#ifdef FEAT_GUI
    "Cursor guibg=fg guifg=bg",
    "lCursor guibg=fg guifg=bg", // should be different, but what?
#endif
    "default link QuickFixLine Search",
    "default link CursorLineSign SignColumn",
    "default link CursorLineFold FoldColumn",
    "default link CurSearch Search",
    "default link PmenuKind Pmenu",
    "default link PmenuKindSel PmenuSel",
    "default link PmenuExtra Pmenu",
    "default link PmenuExtraSel PmenuSel",
    CENT("Normal cterm=NONE", "Normal gui=NONE"),
    NULL
};

// Default colors only used with a light background.
static char *(highlight_init_light[]) = {
    CENT("Directory term=bold ctermfg=DarkBlue",
	 "Directory term=bold ctermfg=DarkBlue guifg=Blue"),
    CENT("LineNr term=underline ctermfg=Brown",
	 "LineNr term=underline ctermfg=Brown guifg=Brown"),
    CENT("CursorLineNr term=bold cterm=underline ctermfg=Brown",
	 "CursorLineNr term=bold cterm=underline ctermfg=Brown gui=bold guifg=Brown"),
    CENT("MoreMsg term=bold ctermfg=DarkGreen",
	 "MoreMsg term=bold ctermfg=DarkGreen gui=bold guifg=SeaGreen"),
    CENT("Question term=standout ctermfg=DarkGreen",
	 "Question term=standout ctermfg=DarkGreen gui=bold guifg=SeaGreen"),
    CENT("Search term=reverse ctermbg=Yellow ctermfg=NONE",
	 "Search term=reverse ctermbg=Yellow ctermfg=NONE guibg=Yellow guifg=NONE"),
#ifdef FEAT_SPELL
    CENT("SpellBad term=reverse ctermbg=LightRed",
	 "SpellBad term=reverse ctermbg=LightRed guisp=Red gui=undercurl"),
    CENT("SpellCap term=reverse ctermbg=LightBlue",
	 "SpellCap term=reverse ctermbg=LightBlue guisp=Blue gui=undercurl"),
    CENT("SpellRare term=reverse ctermbg=LightMagenta",
	 "SpellRare term=reverse ctermbg=LightMagenta guisp=Magenta gui=undercurl"),
    CENT("SpellLocal term=underline ctermbg=Cyan",
	 "SpellLocal term=underline ctermbg=Cyan guisp=DarkCyan gui=undercurl"),
#endif
    CENT("PmenuThumb ctermbg=Black",
	 "PmenuThumb ctermbg=Black guibg=Black"),
    CENT("Pmenu ctermbg=LightMagenta ctermfg=Black",
	 "Pmenu ctermbg=LightMagenta ctermfg=Black guibg=LightMagenta"),
    CENT("PmenuSel ctermbg=LightGrey ctermfg=Black",
	 "PmenuSel ctermbg=LightGrey ctermfg=Black guibg=Grey"),
    CENT("SpecialKey term=bold ctermfg=DarkBlue",
	 "SpecialKey term=bold ctermfg=DarkBlue guifg=Blue"),
    CENT("Title term=bold ctermfg=DarkMagenta",
	 "Title term=bold ctermfg=DarkMagenta gui=bold guifg=Magenta"),
    CENT("WarningMsg term=standout ctermfg=DarkRed",
	 "WarningMsg term=standout ctermfg=DarkRed guifg=Red"),
    CENT("WildMenu term=standout ctermbg=Yellow ctermfg=Black",
	 "WildMenu term=standout ctermbg=Yellow ctermfg=Black guibg=Yellow guifg=Black"),
#ifdef FEAT_FOLDING
    CENT("Folded term=standout ctermbg=Grey ctermfg=DarkBlue",
	 "Folded term=standout ctermbg=Grey ctermfg=DarkBlue guibg=LightGrey guifg=DarkBlue"),
    CENT("FoldColumn term=standout ctermbg=Grey ctermfg=DarkBlue",
	 "FoldColumn term=standout ctermbg=Grey ctermfg=DarkBlue guibg=Grey guifg=DarkBlue"),
#endif
#ifdef FEAT_SIGNS
    CENT("SignColumn term=standout ctermbg=Grey ctermfg=DarkBlue",
	 "SignColumn term=standout ctermbg=Grey ctermfg=DarkBlue guibg=Grey guifg=DarkBlue"),
#endif
    CENT("Visual ctermbg=DarkGrey ctermfg=White",
	 "Visual ctermbg=DarkGrey ctermfg=White guibg=LightGrey"),
#ifdef FEAT_DIFF
    CENT("DiffAdd term=bold ctermbg=LightBlue",
	 "DiffAdd term=bold ctermbg=LightBlue guibg=LightBlue"),
    CENT("DiffChange term=bold ctermbg=LightMagenta",
	 "DiffChange term=bold ctermbg=LightMagenta guibg=LightMagenta"),
    CENT("DiffDelete term=bold ctermfg=Blue ctermbg=LightCyan",
	 "DiffDelete term=bold ctermfg=Blue ctermbg=LightCyan gui=bold guifg=Blue guibg=LightCyan"),
#endif
    CENT("TabLine term=underline cterm=underline ctermfg=black ctermbg=LightGrey",
	 "TabLine term=underline cterm=underline ctermfg=black ctermbg=LightGrey gui=underline guibg=LightGrey"),
#ifdef FEAT_SYN_HL
    CENT("CursorColumn term=reverse ctermbg=LightGrey",
	 "CursorColumn term=reverse ctermbg=LightGrey guibg=Grey90"),
    CENT("CursorLine term=underline cterm=underline",
	 "CursorLine term=underline cterm=underline guibg=Grey90"),
    CENT("ColorColumn term=reverse ctermbg=LightRed",
	 "ColorColumn term=reverse ctermbg=LightRed guibg=LightRed"),
#endif
#ifdef FEAT_CONCEAL
    CENT("Conceal ctermbg=DarkGrey ctermfg=LightGrey",
	 "Conceal ctermbg=DarkGrey ctermfg=LightGrey guibg=DarkGrey guifg=LightGrey"),
#endif
    CENT("MatchParen term=reverse ctermbg=Cyan",
	 "MatchParen term=reverse ctermbg=Cyan guibg=Cyan"),
#ifdef FEAT_TERMINAL
    CENT("StatusLineTerm term=reverse,bold cterm=bold ctermfg=White ctermbg=DarkGreen",
	 "StatusLineTerm term=reverse,bold cterm=bold ctermfg=White ctermbg=DarkGreen gui=bold guifg=bg guibg=DarkGreen"),
    CENT("StatusLineTermNC term=reverse ctermfg=White ctermbg=DarkGreen",
	 "StatusLineTermNC term=reverse ctermfg=White ctermbg=DarkGreen guifg=bg guibg=DarkGreen"),
#endif
#ifdef FEAT_MENU
    CENT("ToolbarLine term=underline ctermbg=LightGrey",
	 "ToolbarLine term=underline ctermbg=LightGrey guibg=LightGrey"),
    CENT("ToolbarButton cterm=bold ctermfg=White ctermbg=DarkGrey",
	 "ToolbarButton cterm=bold ctermfg=White ctermbg=DarkGrey gui=bold guifg=White guibg=Grey40"),
#endif
    NULL
};

// Default colors only used with a dark background.
static char *(highlight_init_dark[]) = {
    CENT("Directory term=bold ctermfg=LightCyan",
	 "Directory term=bold ctermfg=LightCyan guifg=Cyan"),
    CENT("LineNr term=underline ctermfg=Yellow",
	 "LineNr term=underline ctermfg=Yellow guifg=Yellow"),
    CENT("CursorLineNr term=bold cterm=underline ctermfg=Yellow",
	 "CursorLineNr term=bold cterm=underline ctermfg=Yellow gui=bold guifg=Yellow"),
    CENT("MoreMsg term=bold ctermfg=LightGreen",
	 "MoreMsg term=bold ctermfg=LightGreen gui=bold guifg=SeaGreen"),
    CENT("Question term=standout ctermfg=LightGreen",
	 "Question term=standout ctermfg=LightGreen gui=bold guifg=Green"),
    CENT("Search term=reverse ctermbg=Yellow ctermfg=Black",
	 "Search term=reverse ctermbg=Yellow ctermfg=Black guibg=Yellow guifg=Black"),
    CENT("SpecialKey term=bold ctermfg=LightBlue",
	 "SpecialKey term=bold ctermfg=LightBlue guifg=Cyan"),
#ifdef FEAT_SPELL
    CENT("SpellBad term=reverse ctermbg=Red",
	 "SpellBad term=reverse ctermbg=Red guisp=Red gui=undercurl"),
    CENT("SpellCap term=reverse ctermbg=Blue",
	 "SpellCap term=reverse ctermbg=Blue guisp=Blue gui=undercurl"),
    CENT("SpellRare term=reverse ctermbg=Magenta",
	 "SpellRare term=reverse ctermbg=Magenta guisp=Magenta gui=undercurl"),
    CENT("SpellLocal term=underline ctermbg=Cyan",
	 "SpellLocal term=underline ctermbg=Cyan guisp=Cyan gui=undercurl"),
#endif
    CENT("PmenuThumb ctermbg=White",
	 "PmenuThumb ctermbg=White guibg=White"),
    CENT("Pmenu ctermbg=Magenta ctermfg=Black",
	 "Pmenu ctermbg=Magenta ctermfg=Black guibg=Magenta"),
    CENT("PmenuSel ctermbg=Black ctermfg=DarkGrey",
	 "PmenuSel ctermbg=Black ctermfg=DarkGrey guibg=DarkGrey"),
    CENT("Title term=bold ctermfg=LightMagenta",
	 "Title term=bold ctermfg=LightMagenta gui=bold guifg=Magenta"),
    CENT("WarningMsg term=standout ctermfg=LightRed",
	 "WarningMsg term=standout ctermfg=LightRed guifg=Red"),
    CENT("WildMenu term=standout ctermbg=Yellow ctermfg=Black",
	 "WildMenu term=standout ctermbg=Yellow ctermfg=Black guibg=Yellow guifg=Black"),
#ifdef FEAT_FOLDING
    CENT("Folded term=standout ctermbg=DarkGrey ctermfg=Cyan",
	 "Folded term=standout ctermbg=DarkGrey ctermfg=Cyan guibg=DarkGrey guifg=Cyan"),
    CENT("FoldColumn term=standout ctermbg=DarkGrey ctermfg=Cyan",
	 "FoldColumn term=standout ctermbg=DarkGrey ctermfg=Cyan guibg=Grey guifg=Cyan"),
#endif
#ifdef FEAT_SIGNS
    CENT("SignColumn term=standout ctermbg=DarkGrey ctermfg=Cyan",
	 "SignColumn term=standout ctermbg=DarkGrey ctermfg=Cyan guibg=Grey guifg=Cyan"),
#endif
    CENT("Visual ctermbg=Grey ctermfg=Black",
	 "Visual ctermbg=Grey ctermfg=Black guibg=#575757"),
#ifdef FEAT_DIFF
    CENT("DiffAdd term=bold ctermbg=DarkBlue",
	 "DiffAdd term=bold ctermbg=DarkBlue guibg=DarkBlue"),
    CENT("DiffChange term=bold ctermbg=DarkMagenta",
	 "DiffChange term=bold ctermbg=DarkMagenta guibg=DarkMagenta"),
    CENT("DiffDelete term=bold ctermfg=Blue ctermbg=DarkCyan",
	 "DiffDelete term=bold ctermfg=Blue ctermbg=DarkCyan gui=bold guifg=Blue guibg=DarkCyan"),
#endif
    CENT("TabLine term=underline cterm=underline ctermfg=white ctermbg=DarkGrey",
	 "TabLine term=underline cterm=underline ctermfg=white ctermbg=DarkGrey gui=underline guibg=DarkGrey"),
#ifdef FEAT_SYN_HL
    CENT("CursorColumn term=reverse ctermbg=DarkGrey",
	 "CursorColumn term=reverse ctermbg=DarkGrey guibg=Grey40"),
    CENT("CursorLine term=underline cterm=underline",
	 "CursorLine term=underline cterm=underline guibg=Grey40"),
    CENT("ColorColumn term=reverse ctermbg=DarkRed",
	 "ColorColumn term=reverse ctermbg=DarkRed guibg=DarkRed"),
#endif
    CENT("MatchParen term=reverse ctermbg=DarkCyan",
	 "MatchParen term=reverse ctermbg=DarkCyan guibg=DarkCyan"),
#ifdef FEAT_CONCEAL
    CENT("Conceal ctermbg=DarkGrey ctermfg=LightGrey",
	 "Conceal ctermbg=DarkGrey ctermfg=LightGrey guibg=DarkGrey guifg=LightGrey"),
#endif
#ifdef FEAT_TERMINAL
    CENT("StatusLineTerm term=reverse,bold cterm=bold ctermfg=Black ctermbg=LightGreen",
	 "StatusLineTerm term=reverse,bold cterm=bold ctermfg=Black ctermbg=LightGreen gui=bold guifg=bg guibg=LightGreen"),
    CENT("StatusLineTermNC term=reverse ctermfg=Black ctermbg=LightGreen",
	 "StatusLineTermNC term=reverse ctermfg=Black ctermbg=LightGreen guifg=bg guibg=LightGreen"),
#endif
#ifdef FEAT_MENU
    CENT("ToolbarLine term=underline ctermbg=DarkGrey",
	 "ToolbarLine term=underline ctermbg=DarkGrey guibg=Grey50"),
    CENT("ToolbarButton cterm=bold ctermfg=Black ctermbg=LightGrey",
	 "ToolbarButton cterm=bold ctermfg=Black ctermbg=LightGrey gui=bold guifg=Black guibg=LightGrey"),
#endif
    NULL
};

#if defined(FEAT_SYN_HL) || defined(PROTO)
/*
 * Returns the number of highlight groups.
 */
    int
highlight_num_groups(void)
{
    return highlight_ga.ga_len;
}

/*
 * Returns the name of a highlight group.
 */
    char_u *
highlight_group_name(int id)
{
    return HL_TABLE()[id].sg_name;
}

/*
 * Returns the ID of the link to a highlight group.
 */
    int
highlight_link_id(int id)
{
    return HL_TABLE()[id].sg_link;
}
#endif

    void
init_highlight(
    int		both,	    // include groups where 'bg' doesn't matter
    int		reset)	    // clear group first
{
    int		i;
    char	**pp;
    static int	had_both = FALSE;
#ifdef FEAT_EVAL
    char_u	*p;

    // Try finding the color scheme file.  Used when a color file was loaded
    // and 'background' or 't_Co' is changed.
    p = get_var_value((char_u *)"g:colors_name");
    if (p != NULL)
    {
	// The value of g:colors_name could be freed when sourcing the script,
	// making "p" invalid, so copy it.
	char_u *copy_p = vim_strsave(p);
	int    r;

	if (copy_p != NULL)
	{
	    r = load_colors(copy_p);
	    vim_free(copy_p);
	    if (r == OK)
		return;
	}
    }

#endif

    // Didn't use a color file, use the compiled-in colors.
    if (both)
    {
	had_both = TRUE;
	pp = highlight_init_both;
	for (i = 0; pp[i] != NULL; ++i)
	    do_highlight((char_u *)pp[i], reset, TRUE);
    }
    else if (!had_both)
	// Don't do anything before the call with both == TRUE from main().
	// Not everything has been setup then, and that call will overrule
	// everything anyway.
	return;

    if (*p_bg == 'l')
	pp = highlight_init_light;
    else
	pp = highlight_init_dark;
    for (i = 0; pp[i] != NULL; ++i)
	do_highlight((char_u *)pp[i], reset, TRUE);

    // Reverse looks ugly, but grey may not work for 8 colors.  Thus let it
    // depend on the number of colors available.
    // With 8 colors brown is equal to yellow, need to use black for Search fg
    // to avoid Statement highlighted text disappears.
    // Clear the attributes, needed when changing the t_Co value.
    if (t_colors <= 8)
    {
	if (*p_bg == 'l')
	    do_highlight((char_u *)"Search ctermfg=black", FALSE, TRUE);
    }

#ifdef FEAT_SYN_HL
    // If syntax highlighting is enabled load the highlighting for it.
    if (get_var_value((char_u *)"g:syntax_on") != NULL)
    {
	static int	recursive = 0;

	if (recursive >= 5)
	    emsg(_(e_recursive_loop_loading_syncolor_vim));
	else
	{
	    ++recursive;
	    (void)source_runtime((char_u *)"syntax/syncolor.vim", DIP_ALL);
	    --recursive;
	}
    }
#endif
}

#if defined(FEAT_EVAL) && (defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS))
/*
 * Load a default color list. Intended to support legacy color names but allows
 * the user to override the color values. Only loaded once.
 */
    static void
load_default_colors_lists(void)
{
    // Lacking a default color list isn't the end of the world but it is likely
    // an inconvenience so users should know when it is missing.
    if (source_runtime((char_u *)"colors/lists/default.vim", DIP_ALL) != OK)
	msg("failed to load colors/lists/default.vim");
}
#endif

/*
 * Load color file "name".
 * Return OK for success, FAIL for failure.
 */
    int
load_colors(char_u *name)
{
    char_u	*buf;
    int		retval = FAIL;
    static int	recursive = FALSE;

    // When being called recursively, this is probably because setting
    // 'background' caused the highlighting to be reloaded.  This means it is
    // working, thus we should return OK.
    if (recursive)
	return OK;

    recursive = TRUE;
    buf = alloc(STRLEN(name) + 12);
    if (buf != NULL)
    {
#if defined(FEAT_EVAL) && (defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS))
	load_default_colors_lists();
#endif
	apply_autocmds(EVENT_COLORSCHEMEPRE, name,
					       curbuf->b_fname, FALSE, curbuf);
	sprintf((char *)buf, "colors/%s.vim", name);
	retval = source_runtime(buf, DIP_START + DIP_OPT);
	vim_free(buf);
	if (retval == OK)
	    apply_autocmds(EVENT_COLORSCHEME, name, curbuf->b_fname,
								FALSE, curbuf);
    }
    recursive = FALSE;

    return retval;
}

static char *(color_names[28]) = {
	    "Black", "DarkBlue", "DarkGreen", "DarkCyan",
	    "DarkRed", "DarkMagenta", "Brown", "DarkYellow",
	    "Gray", "Grey", "LightGray", "LightGrey",
	    "DarkGray", "DarkGrey",
	    "Blue", "LightBlue", "Green", "LightGreen",
	    "Cyan", "LightCyan", "Red", "LightRed", "Magenta",
	    "LightMagenta", "Yellow", "LightYellow", "White", "NONE"};
	    // indices:
	    // 0, 1, 2, 3,
	    // 4, 5, 6, 7,
	    // 8, 9, 10, 11,
	    // 12, 13,
	    // 14, 15, 16, 17,
	    // 18, 19, 20, 21, 22,
	    // 23, 24, 25, 26, 27
static int color_numbers_16[28] = {0, 1, 2, 3,
				 4, 5, 6, 6,
				 7, 7, 7, 7,
				 8, 8,
				 9, 9, 10, 10,
				 11, 11, 12, 12, 13,
				 13, 14, 14, 15, -1};
// for xterm with 88 colors...
static int color_numbers_88[28] = {0, 4, 2, 6,
				 1, 5, 32, 72,
				 84, 84, 7, 7,
				 82, 82,
				 12, 43, 10, 61,
				 14, 63, 9, 74, 13,
				 75, 11, 78, 15, -1};
// for xterm with 256 colors...
static int color_numbers_256[28] = {0, 4, 2, 6,
				 1, 5, 130, 3,
				 248, 248, 7, 7,
				 242, 242,
				 12, 81, 10, 121,
				 14, 159, 9, 224, 13,
				 225, 11, 229, 15, -1};
// for terminals with less than 16 colors...
static int color_numbers_8[28] = {0, 4, 2, 6,
				 1, 5, 3, 3,
				 7, 7, 7, 7,
				 0+8, 0+8,
				 4+8, 4+8, 2+8, 2+8,
				 6+8, 6+8, 1+8, 1+8, 5+8,
				 5+8, 3+8, 3+8, 7+8, -1};

/*
 * Lookup the "cterm" value to be used for color with index "idx" in
 * color_names[].
 * "boldp" will be set to TRUE or FALSE for a foreground color when using 8
 * colors, otherwise it will be unchanged.
 */
    static int
lookup_color(int idx, int foreground, int *boldp)
{
    int		color = color_numbers_16[idx];
    char_u	*p;

    // Use the _16 table to check if it's a valid color name.
    if (color < 0)
	return -1;

    if (t_colors == 8)
    {
	// t_Co is 8: use the 8 colors table
#if defined(__QNXNTO__)
	// On qnx, the 8 & 16 color arrays are the same
	if (STRNCMP(T_NAME, "qansi", 5) == 0)
	    color = color_numbers_16[idx];
	else
#endif
	    color = color_numbers_8[idx];
	if (foreground)
	{
	    // set/reset bold attribute to get light foreground
	    // colors (on some terminals, e.g. "linux")
	    if (color & 8)
		*boldp = TRUE;
	    else
		*boldp = FALSE;
	}
	color &= 7;	// truncate to 8 colors
    }
    else if (t_colors == 16 || t_colors == 88
					   || t_colors >= 256)
    {
	// Guess: if the termcap entry ends in 'm', it is
	// probably an xterm-like terminal.  Use the changed
	// order for colors.
	if (*T_CAF != NUL)
	    p = T_CAF;
	else
	    p = T_CSF;
	if (*p != NUL && (t_colors > 256
			      || *(p + STRLEN(p) - 1) == 'm'))
	{
	    if (t_colors == 88)
		color = color_numbers_88[idx];
	    else if (t_colors >= 256)
		color = color_numbers_256[idx];
	    else
		color = color_numbers_8[idx];
	}
#ifdef FEAT_TERMRESPONSE
	if (t_colors >= 256 && color == 15 && is_mac_terminal)
	    // Terminal.app has a bug: 15 is light grey. Use white
	    // from the color cube instead.
	    color = 231;
#endif
    }
    return color;
}

/*
 * Link highlight group 'from_hg' to 'to_hg'.
 * 'dodefault' is set to TRUE for ":highlight default link".
 * 'forceit' is set to TRUE for ":highlight! link"
 * 'init' is set to TRUE when initializing all the highlight groups.
 */
    static void
highlight_group_link(
	char_u	*from_hg,
	int	from_len,
	char_u	*to_hg,
	int	to_len,
	int	dodefault,
	int	forceit,
	int	init)
{
    int		from_id;
    int		to_id;
    hl_group_T	*hlgroup = NULL;

    from_id = syn_check_group(from_hg, from_len);
    if (STRNCMP(to_hg, "NONE", 4) == 0)
	to_id = 0;
    else
	to_id = syn_check_group(to_hg, to_len);

    if (from_id > 0)
    {
	hlgroup = &HL_TABLE()[from_id - 1];
	if (dodefault && (forceit || hlgroup->sg_deflink == 0))
	{
	    hlgroup->sg_deflink = to_id;
#ifdef FEAT_EVAL
	    hlgroup->sg_deflink_sctx = current_sctx;
	    hlgroup->sg_deflink_sctx.sc_lnum += SOURCING_LNUM;
#endif
	}
    }

    if (from_id > 0 && (!init || hlgroup->sg_set == 0))
    {
	// Don't allow a link when there already is some highlighting
	// for the group, unless '!' is used
	if (to_id > 0 && !forceit && !init
		&& hl_has_settings(from_id - 1, dodefault))
	{
	    if (SOURCING_NAME == NULL && !dodefault)
		emsg(_(e_group_has_settings_highlight_link_ignored));
	}
	else if (hlgroup->sg_link != to_id
#ifdef FEAT_EVAL
		|| hlgroup->sg_script_ctx.sc_sid != current_sctx.sc_sid
#endif
		|| hlgroup->sg_cleared)
	{
	    if (!init)
		hlgroup->sg_set |= SG_LINK;
	    hlgroup->sg_link = to_id;
#ifdef FEAT_EVAL
	    hlgroup->sg_script_ctx = current_sctx;
	    hlgroup->sg_script_ctx.sc_lnum += SOURCING_LNUM;
#endif
	    hlgroup->sg_cleared = FALSE;
	    redraw_all_later(UPD_SOME_VALID);

	    // Only call highlight_changed() once after multiple changes.
	    need_highlight_changed = TRUE;
	}
    }

}

/*
 * Reset all highlighting to the defaults. Removes all highlighting for the
 * groups added by the user.
 */
    static void
highlight_reset_all(void)
{
    int		idx;

#ifdef FEAT_GUI
    // First, we do not destroy the old values, but allocate the new
    // ones and update the display. THEN we destroy the old values.
    // If we destroy the old values first, then the old values
    // (such as GuiFont's or GuiFontset's) will still be displayed but
    // invalid because they were free'd.
    if (gui.in_use)
    {
# ifdef FEAT_BEVAL_TIP
	gui_init_tooltip_font();
# endif
# if defined(FEAT_MENU) && defined(FEAT_GUI_MOTIF)
	gui_init_menu_font();
# endif
    }
# if defined(FEAT_GUI_MSWIN) || defined(FEAT_GUI_X11)
    gui_mch_def_colors();
# endif
# ifdef FEAT_GUI_X11
#  ifdef FEAT_MENU

    // This only needs to be done when there is no Menu highlight
    // group defined by default, which IS currently the case.
    gui_mch_new_menu_colors();
#  endif
    if (gui.in_use)
    {
	gui_new_scrollbar_colors();
#  ifdef FEAT_BEVAL_GUI
	gui_mch_new_tooltip_colors();
#  endif
#  ifdef FEAT_MENU
	gui_mch_new_menu_font();
#  endif
    }
# endif

    // Ok, we're done allocating the new default graphics items.
    // The screen should already be refreshed at this point.
    // It is now Ok to clear out the old data.
#endif
#ifdef FEAT_EVAL
    do_unlet((char_u *)"g:colors_name", TRUE);
#endif
    restore_cterm_colors();

    // Clear all default highlight groups and load the defaults.
    for (idx = 0; idx < highlight_ga.ga_len; ++idx)
	highlight_clear(idx);
    init_highlight(TRUE, TRUE);
#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    if (USE_24BIT)
	highlight_gui_started();
    else
#endif
	highlight_changed();
    redraw_later_clear();
}

/*
 * Set the 'term' or 'cterm' or 'gui' attributes for the highlight group at
 * index 'idx'.
 * 'key' is one of 'TERM' or 'CTERM' or 'GUI'
 * 'arg' is the list of attribute names separated by comma.
 * 'init' is set to TRUE when initializing all the highlight groups.
 * Returns TRUE if the attributes are set.
 */
    static int
highlight_set_termgui_attr(int idx, char_u *key, char_u *arg, int init)
{
    int		attr;
    int		off;
    long	i;
    int		len;

    attr = 0;
    off = 0;
    while (arg[off] != NUL)
    {
	for (i = ARRAY_LENGTH(hl_attr_table); --i >= 0; )
	{
	    len = (int)STRLEN(hl_name_table[i]);
	    if (STRNICMP(arg + off, hl_name_table[i], len) == 0)
	    {
		attr |= hl_attr_table[i];
		off += len;
		break;
	    }
	}
	if (i < 0)
	{
	    semsg(_(e_illegal_value_str), arg);
	    return FALSE;
	}
	if (arg[off] == ',')		// another one follows
	    ++off;
    }
    if (*key == 'T')
    {
	if (!init || !(HL_TABLE()[idx].sg_set & SG_TERM))
	{
	    if (!init)
		HL_TABLE()[idx].sg_set |= SG_TERM;
	    HL_TABLE()[idx].sg_term = attr;
	}
    }
    else if (*key == 'C')
    {
	if (!init || !(HL_TABLE()[idx].sg_set & SG_CTERM))
	{
	    if (!init)
		HL_TABLE()[idx].sg_set |= SG_CTERM;
	    HL_TABLE()[idx].sg_cterm = attr;
	    HL_TABLE()[idx].sg_cterm_bold = FALSE;
	}
    }
#if defined(FEAT_GUI) || defined(FEAT_EVAL)
    else
    {
	if (!init || !(HL_TABLE()[idx].sg_set & SG_GUI))
	{
	    if (!init)
		HL_TABLE()[idx].sg_set |= SG_GUI;
	    HL_TABLE()[idx].sg_gui = attr;
	}
    }
#endif

    return TRUE;
}

#ifdef FEAT_GUI
/*
 * Set the font for the highlight group at 'idx'.
 * 'arg' is the font name.
 * Returns TRUE if the font is changed.
 */
    static int
highlight_set_font(
	int	idx,
	char_u	*arg,
	int	is_normal_group,
	int	is_menu_group,
	int	is_tooltip_group)
{
    int		did_change = FALSE;

    // in non-GUI fonts are simply ignored
    if (HL_TABLE()[idx].sg_font_name != NULL
	    && STRCMP(HL_TABLE()[idx].sg_font_name, arg) == 0)
    {
	// Font name didn't change, ignore.
    }
    else if (!gui.shell_created)
    {
	// GUI not started yet, always accept the name.
	vim_free(HL_TABLE()[idx].sg_font_name);
	HL_TABLE()[idx].sg_font_name = vim_strsave(arg);
	did_change = TRUE;
    }
    else
    {
	GuiFont temp_sg_font = HL_TABLE()[idx].sg_font;
# ifdef FEAT_XFONTSET
	GuiFontset temp_sg_fontset = HL_TABLE()[idx].sg_fontset;
# endif
	// First, save the current font/fontset.
	// Then try to allocate the font/fontset.
	// If the allocation fails, HL_TABLE()[idx].sg_font OR
	// sg_fontset will be set to NOFONT or NOFONTSET respectively.

	HL_TABLE()[idx].sg_font = NOFONT;
# ifdef FEAT_XFONTSET
	HL_TABLE()[idx].sg_fontset = NOFONTSET;
# endif
	hl_do_font(idx, arg, is_normal_group, is_menu_group,
		is_tooltip_group, FALSE);

# ifdef FEAT_XFONTSET
	if (HL_TABLE()[idx].sg_fontset != NOFONTSET)
	{
	    // New fontset was accepted. Free the old one, if there
	    // was one.
	    gui_mch_free_fontset(temp_sg_fontset);
	    vim_free(HL_TABLE()[idx].sg_font_name);
	    HL_TABLE()[idx].sg_font_name = vim_strsave(arg);
	    did_change = TRUE;
	}
	else
	    HL_TABLE()[idx].sg_fontset = temp_sg_fontset;
# endif
	if (HL_TABLE()[idx].sg_font != NOFONT)
	{
	    // New font was accepted. Free the old one, if there was
	    // one.
	    gui_mch_free_font(temp_sg_font);
	    vim_free(HL_TABLE()[idx].sg_font_name);
	    HL_TABLE()[idx].sg_font_name = vim_strsave(arg);
	    did_change = TRUE;
	}
	else
	    HL_TABLE()[idx].sg_font = temp_sg_font;
    }

    return did_change;
}
#endif

/*
 * Set the cterm foreground color for the Normal highlight group to "color" and
 * the bold attribute to "bold".
 */
    static void
hl_set_ctermfg_normal_group(int color, int bold)
{
    cterm_normal_fg_color = color + 1;
    cterm_normal_fg_bold = bold;
#ifdef FEAT_GUI
    // Don't do this if the GUI is used.
    if (!gui.in_use && !gui.starting)
#endif
    {
	set_must_redraw(UPD_CLEAR);
	if (termcap_active && color >= 0)
	    term_fg_color(color);
    }
}

/*
 * Set the cterm foreground color for the highlight group at 'idx' to 'color'.
 */
    static void
highlight_set_ctermfg(int idx, int color, int is_normal_group)
{
    HL_TABLE()[idx].sg_cterm_fg = color + 1;
    if (is_normal_group)
	hl_set_ctermfg_normal_group(color,
					(HL_TABLE()[idx].sg_cterm & HL_BOLD));
}

/*
 * Set the cterm background color for the Normal highlight group to "color".
 */
    static void
hl_set_ctermbg_normal_group(int color)
{
    cterm_normal_bg_color = color + 1;
#ifdef FEAT_GUI
    // Don't mess with 'background' if the GUI is used.
    if (!gui.in_use && !gui.starting)
#endif
    {
	set_must_redraw(UPD_CLEAR);
	if (color >= 0)
	{
	    int dark = -1;

	    if (termcap_active)
		term_bg_color(color);
	    if (t_colors < 16)
		dark = (color == 0 || color == 4);
	    // Limit the heuristic to the standard 16 colors
	    else if (color < 16)
		dark = (color < 7 || color == 8);
	    // Set the 'background' option if the value is
	    // wrong.
	    if (dark != -1
		    && dark != (*p_bg == 'd')
		    && !option_was_set((char_u *)"bg"))
	    {
		set_option_value_give_err((char_u *)"bg",
			0L, (char_u *)(dark ? "dark" : "light"), 0);
		reset_option_was_set((char_u *)"bg");
	    }
	}
    }
}

/*
 * Set the cterm background color for the highlight group at 'idx' to 'color'.
 */
    static void
highlight_set_ctermbg(int idx, int color, int is_normal_group)
{
    HL_TABLE()[idx].sg_cterm_bg = color + 1;
    if (is_normal_group)
	hl_set_ctermbg_normal_group(color);
}

/*
 * Set the cterm underline color for the Normal highlight group to "color".
 */
    static void
hl_set_ctermul_normal_group(int color)
{
    cterm_normal_ul_color = color + 1;
#ifdef FEAT_GUI
    // Don't do this if the GUI is used.
    if (!gui.in_use && !gui.starting)
#endif
    {
	set_must_redraw(UPD_CLEAR);
	if (termcap_active && color >= 0)
	    term_ul_color(color);
    }
}

/*
 * Set the cterm underline color for the highlight group at 'idx' to 'color'.
 */
    static void
highlight_set_ctermul(int idx, int color, int is_normal_group)
{
    HL_TABLE()[idx].sg_cterm_ul = color + 1;
    if (is_normal_group)
	hl_set_ctermul_normal_group(color);
}

/*
 * Set the cterm font for the highlight group at 'idx'.
 * 'arg' is the color name or the numeric value as a string.
 * 'init' is set to TRUE when initializing highlighting.
 * Called for the ":highlight" command and the "hlset()" function.
 *
 * Returns TRUE if the font is set.
 */
    static int
highlight_set_cterm_font(
	int	idx,
	char_u	*arg,
	int	init)
{
    int		font;

    if (init && (HL_TABLE()[idx].sg_set & SG_CTERM))
	return FALSE;

    if (!init)
	HL_TABLE()[idx].sg_set |= SG_CTERM;

    if (VIM_ISDIGIT(*arg))
	font = atoi((char *)arg);
    else if (STRICMP(arg, "NONE") == 0)
	font = -1;
    else
	return FALSE;

    HL_TABLE()[idx].sg_cterm_font = font + 1;
    return TRUE;
}

/*
 * Set the cterm fg/bg/ul color for the highlight group at 'idx'.
 * 'key' is one of 'CTERMFG' or 'CTERMBG' or 'CTERMUL'.
 * 'keystart' is the color name/value.
 * 'arg' is the color name or the numeric value as a string.
 * 'is_normal_group' is set if the highlight group is 'NORMAL'
 * 'init' is set to TRUE when initializing highlighting.
 * Called for the ":highlight" command and the "hlset()" function.
 *
 * Returns TRUE if the color is set.
 */
    static int
highlight_set_cterm_color(
	int	idx,
	char_u	*key,
	char_u	*key_start,
	char_u	*arg,
	int	is_normal_group,
	int	init)
{
    int		color;
    long	i;
    int		off;

    if (init && (HL_TABLE()[idx].sg_set & SG_CTERM))
	return FALSE;

    if (!init)
	HL_TABLE()[idx].sg_set |= SG_CTERM;

    // When setting the foreground color, and previously the "bold"
    // flag was set for a light color, reset it now
    if (key[5] == 'F' && HL_TABLE()[idx].sg_cterm_bold)
    {
	HL_TABLE()[idx].sg_cterm &= ~HL_BOLD;
	HL_TABLE()[idx].sg_cterm_bold = FALSE;
    }

    if (VIM_ISDIGIT(*arg))
	color = atoi((char *)arg);
    else if (STRICMP(arg, "fg") == 0)
    {
	if (cterm_normal_fg_color)
	    color = cterm_normal_fg_color - 1;
	else
	{
	    emsg(_(e_fg_color_unknown));
	    return FALSE;
	}
    }
    else if (STRICMP(arg, "bg") == 0)
    {
	if (cterm_normal_bg_color > 0)
	    color = cterm_normal_bg_color - 1;
	else
	{
	    emsg(_(e_bg_color_unknown));
	    return FALSE;
	}
    }
    else if (STRICMP(arg, "ul") == 0)
    {
	if (cterm_normal_ul_color > 0)
	    color = cterm_normal_ul_color - 1;
	else
	{
	    emsg(_(e_ul_color_unknown));
	    return FALSE;
	}
    }
    else
    {
	int bold = MAYBE;

	// reduce calls to STRICMP a bit, it can be slow
	off = TOUPPER_ASC(*arg);
	for (i = ARRAY_LENGTH(color_names); --i >= 0; )
	    if (off == color_names[i][0]
		    && STRICMP(arg + 1, color_names[i] + 1) == 0)
		break;
	if (i < 0)
	{
	    semsg(_(e_color_name_or_number_not_recognized_str), key_start);
	    return FALSE;
	}

	color = lookup_color(i, key[5] == 'F', &bold);

	// set/reset bold attribute to get light foreground
	// colors (on some terminals, e.g. "linux")
	if (bold == TRUE)
	{
	    HL_TABLE()[idx].sg_cterm |= HL_BOLD;
	    HL_TABLE()[idx].sg_cterm_bold = TRUE;
	}
	else if (bold == FALSE)
	    HL_TABLE()[idx].sg_cterm &= ~HL_BOLD;
    }

    // Add one to the argument, to avoid zero.  Zero is used for
    // "NONE", then "color" is -1.
    if (key[5] == 'F')
	highlight_set_ctermfg(idx, color, is_normal_group);
    else if (key[5] == 'B')
	highlight_set_ctermbg(idx, color, is_normal_group);
    else // ctermul
	highlight_set_ctermul(idx, color, is_normal_group);

    return TRUE;
}

#if defined(FEAT_GUI) || defined(FEAT_EVAL)
/*
 * Set the GUI foreground color for the highlight group at 'idx'.
 * Returns TRUE if the color is set.
 */
    static int
highlight_set_guifg(
	int	idx,
	char_u	*arg,
	int	is_menu_group UNUSED,
	int	is_scrollbar_group UNUSED,
	int	is_tooltip_group UNUSED,
	int	*do_colors UNUSED,
	int	init)
{
# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    long	i;
# endif
    char_u	**namep;
    int		did_change = FALSE;

    if (init && (HL_TABLE()[idx].sg_set & SG_GUI))
	return FALSE;

    namep = &HL_TABLE()[idx].sg_gui_fg_name;
    if (!init)
	HL_TABLE()[idx].sg_set |= SG_GUI;

# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    // In GUI guifg colors are only used when recognized
    i = color_name2handle(arg);
    if (i != INVALCOLOR || STRCMP(arg, "NONE") == 0 || !USE_24BIT)
    {
	HL_TABLE()[idx].sg_gui_fg = i;
# endif
	if (*namep == NULL || STRCMP(*namep, arg) != 0)
	{
	    vim_free(*namep);
	    if (STRCMP(arg, "NONE") != 0)
		*namep = vim_strsave(arg);
	    else
		*namep = NULL;
	    did_change = TRUE;
	}
# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
#  ifdef FEAT_GUI_X11
	if (is_menu_group && gui.menu_fg_pixel != i)
	{
	    gui.menu_fg_pixel = i;
	    *do_colors = TRUE;
	}
	if (is_scrollbar_group && gui.scroll_fg_pixel != i)
	{
	    gui.scroll_fg_pixel = i;
	    *do_colors = TRUE;
	}
#   ifdef FEAT_BEVAL_GUI
	if (is_tooltip_group && gui.tooltip_fg_pixel != i)
	{
	    gui.tooltip_fg_pixel = i;
	    *do_colors = TRUE;
	}
#   endif
#  endif
    }
# endif

    return did_change;
}

/*
 * Set the GUI background color for the highlight group at 'idx'.
 * Returns TRUE if the color is set.
 */
    static int
highlight_set_guibg(
	int	idx,
	char_u	*arg,
	int	is_menu_group UNUSED,
	int	is_scrollbar_group UNUSED,
	int	is_tooltip_group UNUSED,
	int	*do_colors UNUSED,
	int	init)
{
# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    int		i;
# endif
    char_u	**namep;
    int		did_change = FALSE;

    if (init && (HL_TABLE()[idx].sg_set & SG_GUI))
	return FALSE;

    namep = &HL_TABLE()[idx].sg_gui_bg_name;
    if (!init)
	HL_TABLE()[idx].sg_set |= SG_GUI;

# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    // In GUI guibg colors are only used when recognized
    i = color_name2handle(arg);
    if (i != INVALCOLOR || STRCMP(arg, "NONE") == 0 || !USE_24BIT)
    {
	HL_TABLE()[idx].sg_gui_bg = i;
# endif
	if (*namep == NULL || STRCMP(*namep, arg) != 0)
	{
	    vim_free(*namep);
	    if (STRCMP(arg, "NONE") != 0)
		*namep = vim_strsave(arg);
	    else
		*namep = NULL;
	    did_change = TRUE;
	}
# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
#  ifdef FEAT_GUI_X11
	if (is_menu_group && gui.menu_bg_pixel != i)
	{
	    gui.menu_bg_pixel = i;
	    *do_colors = TRUE;
	}
	if (is_scrollbar_group && gui.scroll_bg_pixel != i)
	{
	    gui.scroll_bg_pixel = i;
	    *do_colors = TRUE;
	}
#   ifdef FEAT_BEVAL_GUI
	if (is_tooltip_group && gui.tooltip_bg_pixel != i)
	{
	    gui.tooltip_bg_pixel = i;
	    *do_colors = TRUE;
	}
#   endif
#  endif
    }
# endif

    return did_change;
}

/*
 * Set the GUI undercurl/strikethrough color for the highlight group at 'idx'.
 * Returns TRUE if the color is set.
 */
    static int
highlight_set_guisp(int idx, char_u *arg, int init)
{
# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    int		i;
# endif
    int		did_change = FALSE;
    char_u	**namep;

    if (init && (HL_TABLE()[idx].sg_set & SG_GUI))
	return FALSE;

    namep = &HL_TABLE()[idx].sg_gui_sp_name;
    if (!init)
	HL_TABLE()[idx].sg_set |= SG_GUI;

# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    // In GUI guisp colors are only used when recognized
    i = color_name2handle(arg);
    if (i != INVALCOLOR || STRCMP(arg, "NONE") == 0 || !USE_24BIT)
    {
	HL_TABLE()[idx].sg_gui_sp = i;
# endif
	if (*namep == NULL || STRCMP(*namep, arg) != 0)
	{
	    vim_free(*namep);
	    if (STRCMP(arg, "NONE") != 0)
		*namep = vim_strsave(arg);
	    else
		*namep = NULL;
	    did_change = TRUE;
	}
# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    }
# endif

    return did_change;
}
#endif

/*
 * Set the start/stop terminal codes for a highlight group.
 * Returns TRUE if the terminal code is set.
 */
    static int
highlight_set_startstop_termcode(int idx, char_u *key, char_u *arg, int init)
{
    int		off;
    char_u	buf[100];
    int		len;
    char_u	*tname;
    char_u	*p;

    if (!init)
	HL_TABLE()[idx].sg_set |= SG_TERM;

    // The "start" and "stop"  arguments can be a literal escape
    // sequence, or a comma separated list of terminal codes.
    if (STRNCMP(arg, "t_", 2) == 0)
    {
	off = 0;
	buf[0] = 0;
	while (arg[off] != NUL)
	{
	    // Isolate one termcap name
	    for (len = 0; arg[off + len] &&
		    arg[off + len] != ','; ++len)
		;
	    tname = vim_strnsave(arg + off, len);
	    if (tname == NULL)		// out of memory
		return FALSE;
	    // lookup the escape sequence for the item
	    p = get_term_code(tname);
	    vim_free(tname);
	    if (p == NULL)	    // ignore non-existing things
		p = (char_u *)"";

	    // Append it to the already found stuff
	    if ((int)(STRLEN(buf) + STRLEN(p)) >= 99)
	    {
		semsg(_(e_terminal_code_too_long_str), arg);
		return FALSE;
	    }
	    STRCAT(buf, p);

	    // Advance to the next item
	    off += len;
	    if (arg[off] == ',')	    // another one follows
		++off;
	}
    }
    else
    {
	// Copy characters from arg[] to buf[], translating <> codes.
	for (p = arg, off = 0; off < 100 - 6 && *p; )
	{
	    len = trans_special(&p, buf + off, FSK_SIMPLIFY, FALSE, NULL);
	    if (len > 0)	    // recognized special char
		off += len;
	    else		    // copy as normal char
		buf[off++] = *p++;
	}
	buf[off] = NUL;
    }

    if (STRCMP(buf, "NONE") == 0)	// resetting the value
	p = NULL;
    else
	p = vim_strsave(buf);
    if (key[2] == 'A')
    {
	vim_free(HL_TABLE()[idx].sg_start);
	HL_TABLE()[idx].sg_start = p;
    }
    else
    {
	vim_free(HL_TABLE()[idx].sg_stop);
	HL_TABLE()[idx].sg_stop = p;
    }
    return TRUE;
}

/*
 * Handle the ":highlight .." command.
 * When using ":hi clear" this is called recursively for each group with
 * "forceit" and "init" both TRUE.
 */
    void
do_highlight(
    char_u	*line,
    int		forceit,
    int		init)	    // TRUE when called for initializing
{
    char_u	*name_end;
    char_u	*linep;
    char_u	*key_start;
    char_u	*arg_start;
    char_u	*key = NULL, *arg = NULL;
    long	i;
    int		id;
    int		idx;
    hl_group_T	item_before;
    int		did_change = FALSE;
    int		dodefault = FALSE;
    int		doclear = FALSE;
    int		dolink = FALSE;
    int		error = FALSE;
    int		is_normal_group = FALSE;	// "Normal" group
#ifdef FEAT_GUI_X11
    int		is_menu_group = FALSE;		// "Menu" group
    int		is_scrollbar_group = FALSE;	// "Scrollbar" group
    int		is_tooltip_group = FALSE;	// "Tooltip" group
#else
# define is_menu_group 0
# define is_tooltip_group 0
# define is_scrollbar_group 0
#endif
#if defined(FEAT_GUI) || defined(FEAT_EVAL)
    int		do_colors = FALSE;		// need to update colors?
#endif
#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    int		did_highlight_changed = FALSE;
#endif

    // If no argument, list current highlighting.
    if (!init && ends_excmd2(line - 1, line))
    {
	for (i = 1; i <= highlight_ga.ga_len && !got_int; ++i)
	    // TODO: only call when the group has attributes set
	    highlight_list_one((int)i);
	return;
    }

    // Isolate the name.
    name_end = skiptowhite(line);
    linep = skipwhite(name_end);

    // Check for "default" argument.
    if (STRNCMP(line, "default", name_end - line) == 0)
    {
	dodefault = TRUE;
	line = linep;
	name_end = skiptowhite(line);
	linep = skipwhite(name_end);
    }

    // Check for "clear" or "link" argument.
    if (STRNCMP(line, "clear", name_end - line) == 0)
	doclear = TRUE;
    if (STRNCMP(line, "link", name_end - line) == 0)
	dolink = TRUE;

    // ":highlight {group-name}": list highlighting for one group.
    if (!doclear && !dolink && ends_excmd2(line, linep))
    {
	id = syn_namen2id(line, (int)(name_end - line));
	if (id == 0)
	    semsg(_(e_highlight_group_name_not_found_str), line);
	else
	    highlight_list_one(id);
	return;
    }

    // Handle ":highlight link {from} {to}" command.
    if (dolink)
    {
	char_u	    *from_start = linep;
	char_u	    *from_end;
	int	    from_len;
	char_u	    *to_start;
	char_u	    *to_end;
	int	    to_len;

	from_end = skiptowhite(from_start);
	to_start = skipwhite(from_end);
	to_end	 = skiptowhite(to_start);

	if (ends_excmd2(line, from_start) || ends_excmd2(line, to_start))
	{
	    semsg(_(e_not_enough_arguments_highlight_link_str), from_start);
	    return;
	}

	if (!ends_excmd2(line, skipwhite(to_end)))
	{
	    semsg(_(e_too_many_arguments_highlight_link_str), from_start);
	    return;
	}

	from_len = (int)(from_end - from_start);
	to_len = (int)(to_end - to_start);
	highlight_group_link(from_start, from_len, to_start, to_len,
						dodefault, forceit, init);
	return;
    }

    if (doclear)
    {
	// ":highlight clear [group]" command.
	if (ends_excmd2(line, linep))
	{
	    // ":highlight clear" without group name
	    highlight_reset_all();
	    return;
	}
	line = linep;
	name_end = skiptowhite(line);
	linep = skipwhite(name_end);
    }

    // Find the group name in the table.  If it does not exist yet, add it.
    id = syn_check_group(line, (int)(name_end - line));
    if (id == 0)	// failed (out of memory)
	return;
    idx = id - 1;	// index is ID minus one

    // Return if "default" was used and the group already has settings.
    if (dodefault && hl_has_settings(idx, TRUE))
	return;

    // Make a copy so we can check if any attribute actually changed.
    item_before = HL_TABLE()[idx];

    if (STRCMP(HL_TABLE()[idx].sg_name_u, "NORMAL") == 0)
	is_normal_group = TRUE;
#ifdef FEAT_GUI_X11
    else if (STRCMP(HL_TABLE()[idx].sg_name_u, "MENU") == 0)
	is_menu_group = TRUE;
    else if (STRCMP(HL_TABLE()[idx].sg_name_u, "SCROLLBAR") == 0)
	is_scrollbar_group = TRUE;
    else if (STRCMP(HL_TABLE()[idx].sg_name_u, "TOOLTIP") == 0)
	is_tooltip_group = TRUE;
#endif

    // Clear the highlighting for ":hi clear {group}" and ":hi clear".
    if (doclear || (forceit && init))
    {
	highlight_clear(idx);
	if (!doclear)
	    HL_TABLE()[idx].sg_set = 0;
    }

    if (!doclear)
	while (!ends_excmd2(line, linep))
	{
	    key_start = linep;
	    if (*linep == '=')
	    {
		semsg(_(e_unexpected_equal_sign_str), key_start);
		error = TRUE;
		break;
	    }

	    // Isolate the key ("term", "ctermfg", "ctermbg", "font", "guifg"
	    // or "guibg").
	    while (*linep && !VIM_ISWHITE(*linep) && *linep != '=')
		++linep;
	    vim_free(key);
	    key = vim_strnsave_up(key_start, linep - key_start);
	    if (key == NULL)
	    {
		error = TRUE;
		break;
	    }
	    linep = skipwhite(linep);

	    if (STRCMP(key, "NONE") == 0)
	    {
		if (!init || HL_TABLE()[idx].sg_set == 0)
		{
		    if (!init)
			HL_TABLE()[idx].sg_set |= SG_TERM+SG_CTERM+SG_GUI;
		    highlight_clear(idx);
		}
		continue;
	    }

	    // Check for the equal sign.
	    if (*linep != '=')
	    {
		semsg(_(e_missing_equal_sign_str_2), key_start);
		error = TRUE;
		break;
	    }
	    ++linep;

	    // Isolate the argument.
	    linep = skipwhite(linep);
	    if (*linep == '\'')		// guifg='color name'
	    {
		arg_start = ++linep;
		linep = vim_strchr(linep, '\'');
		if (linep == NULL)
		{
		    semsg(_(e_invalid_argument_str), key_start);
		    error = TRUE;
		    break;
		}
	    }
	    else
	    {
		arg_start = linep;
		linep = skiptowhite(linep);
	    }
	    if (linep == arg_start)
	    {
		semsg(_(e_missing_argument_str), key_start);
		error = TRUE;
		break;
	    }
	    vim_free(arg);
	    arg = vim_strnsave(arg_start, linep - arg_start);
	    if (arg == NULL)
	    {
		error = TRUE;
		break;
	    }
	    if (*linep == '\'')
		++linep;

	    // Store the argument.
	    if (STRCMP(key, "TERM") == 0
		    || STRCMP(key, "CTERM") == 0
		    || STRCMP(key, "GUI") == 0)
	    {
		if (!highlight_set_termgui_attr(idx, key, arg, init))
		{
		    error = TRUE;
		    break;
		}
	    }
	    else if (STRCMP(key, "FONT") == 0)
	    {
		// in non-GUI fonts are simply ignored
#ifdef FEAT_GUI
		if (highlight_set_font(idx, arg, is_normal_group,
					      is_menu_group, is_tooltip_group))
		    did_change = TRUE;
#endif
	    }
	    else if (STRCMP(key, "CTERMFG") == 0
		    || STRCMP(key, "CTERMBG") == 0
		    || STRCMP(key, "CTERMUL") == 0)
	    {
		if (!highlight_set_cterm_color(idx, key, key_start, arg,
							is_normal_group, init))
		{
		    error = TRUE;
		    break;
		}
	    }
	    else if (STRCMP(key, "CTERMFONT") == 0)
	    {
		if (!highlight_set_cterm_font(idx, arg, init))
		{
		    error = TRUE;
		    break;
		}
	    }
	    else if (STRCMP(key, "GUIFG") == 0)
	    {
#if defined(FEAT_GUI) || defined(FEAT_EVAL)
		if (highlight_set_guifg(idx, arg, is_menu_group,
					  is_scrollbar_group, is_tooltip_group,
							     &do_colors, init))
		    did_change = TRUE;
#endif
	    }
	    else if (STRCMP(key, "GUIBG") == 0)
	    {
#if defined(FEAT_GUI) || defined(FEAT_EVAL)
		if (highlight_set_guibg(idx, arg, is_menu_group,
			    is_scrollbar_group, is_tooltip_group,
			    &do_colors, init))
		    did_change = TRUE;
#endif
	    }
	    else if (STRCMP(key, "GUISP") == 0)
	    {
#if defined(FEAT_GUI) || defined(FEAT_EVAL)
		if (highlight_set_guisp(idx, arg, init))
		    did_change = TRUE;
#endif
	    }
	    else if (STRCMP(key, "START") == 0 || STRCMP(key, "STOP") == 0)
	    {
		if (!highlight_set_startstop_termcode(idx, key, arg, init))
		{
		    error = TRUE;
		    break;
		}
	    }
	    else
	    {
		semsg(_(e_illegal_argument_str_3), key_start);
		error = TRUE;
		break;
	    }
	    HL_TABLE()[idx].sg_cleared = FALSE;

	    // When highlighting has been given for a group, don't link it.
	    if (!init || !(HL_TABLE()[idx].sg_set & SG_LINK))
		HL_TABLE()[idx].sg_link = 0;

	    // Continue with next argument.
	    linep = skipwhite(linep);
	}

    // If there is an error, and it's a new entry, remove it from the table.
    if (error && idx == highlight_ga.ga_len)
	syn_unadd_group();
    else
    {
	if (is_normal_group)
	{
	    HL_TABLE()[idx].sg_term_attr = 0;
	    HL_TABLE()[idx].sg_cterm_attr = 0;
#ifdef FEAT_GUI
	    HL_TABLE()[idx].sg_gui_attr = 0;
	    // Need to update all groups, because they might be using "bg"
	    // and/or "fg", which have been changed now.
#endif
#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
	    if (USE_24BIT)
	    {
		highlight_gui_started();
		did_highlight_changed = TRUE;
		redraw_all_later(UPD_NOT_VALID);
	    }
#endif
#ifdef FEAT_VTP
	    control_console_color_rgb();
#endif
	}
#ifdef FEAT_GUI_X11
# ifdef FEAT_MENU
	else if (is_menu_group)
	{
	    if (gui.in_use && do_colors)
		gui_mch_new_menu_colors();
	}
# endif
	else if (is_scrollbar_group)
	{
	    if (gui.in_use && do_colors)
		gui_new_scrollbar_colors();
	    else
		set_hl_attr(idx);
	}
# ifdef FEAT_BEVAL_GUI
	else if (is_tooltip_group)
	{
	    if (gui.in_use && do_colors)
		gui_mch_new_tooltip_colors();
	}
# endif
#endif
	else
	    set_hl_attr(idx);
#ifdef FEAT_EVAL
	HL_TABLE()[idx].sg_script_ctx = current_sctx;
	HL_TABLE()[idx].sg_script_ctx.sc_lnum += SOURCING_LNUM;
#endif
    }

    vim_free(key);
    vim_free(arg);

    // Only call highlight_changed() once, after a sequence of highlight
    // commands, and only if an attribute actually changed.
    if ((did_change
	   || memcmp(&HL_TABLE()[idx], &item_before, sizeof(item_before)) != 0)
#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
	    && !did_highlight_changed
#endif
       )
    {
	// Do not trigger a redraw when highlighting is changed while
	// redrawing.  This may happen when evaluating 'statusline' changes the
	// StatusLine group.
	if (!updating_screen)
	    redraw_all_later(UPD_NOT_VALID);
	need_highlight_changed = TRUE;
    }
}

#if defined(EXITFREE) || defined(PROTO)
    void
free_highlight(void)
{
    int	    i;

    for (i = 0; i < highlight_ga.ga_len; ++i)
    {
	highlight_clear(i);
	vim_free(HL_TABLE()[i].sg_name);
	vim_free(HL_TABLE()[i].sg_name_u);
    }
    ga_clear(&highlight_ga);
}
#endif

/*
 * Reset the cterm colors to what they were before Vim was started, if
 * possible.  Otherwise reset them to zero.
 */
    void
restore_cterm_colors(void)
{
#if defined(MSWIN) && !defined(FEAT_GUI_MSWIN)
    // Since t_me has been set, this probably means that the user
    // wants to use this as default colors.  Need to reset default
    // background/foreground colors.
    mch_set_normal_colors();
#else
# ifdef VIMDLL
    if (!gui.in_use)
    {
	mch_set_normal_colors();
	return;
    }
# endif
    cterm_normal_fg_color = 0;
    cterm_normal_fg_bold = 0;
    cterm_normal_bg_color = 0;
# ifdef FEAT_TERMGUICOLORS
    cterm_normal_fg_gui_color = INVALCOLOR;
    cterm_normal_bg_gui_color = INVALCOLOR;
    cterm_normal_ul_gui_color = INVALCOLOR;
# endif
#endif
}

/*
 * Return TRUE if highlight group "idx" has any settings.
 * When "check_link" is TRUE also check for an existing link.
 */
    static int
hl_has_settings(int idx, int check_link)
{
    return HL_TABLE()[idx].sg_cleared == 0
	 && (  HL_TABLE()[idx].sg_term_attr != 0
	    || HL_TABLE()[idx].sg_cterm_attr != 0
	    || HL_TABLE()[idx].sg_cterm_fg != 0
	    || HL_TABLE()[idx].sg_cterm_bg != 0
	    || HL_TABLE()[idx].sg_cterm_font != 0
#ifdef FEAT_GUI
	    || HL_TABLE()[idx].sg_gui_attr != 0
	    || HL_TABLE()[idx].sg_gui_fg_name != NULL
	    || HL_TABLE()[idx].sg_gui_bg_name != NULL
	    || HL_TABLE()[idx].sg_gui_sp_name != NULL
	    || HL_TABLE()[idx].sg_font_name != NULL
#endif
	    || (check_link && (HL_TABLE()[idx].sg_set & SG_LINK)));
}

/*
 * Clear highlighting for one group.
 */
    static void
highlight_clear(int idx)
{
    HL_TABLE()[idx].sg_cleared = TRUE;

    HL_TABLE()[idx].sg_term = 0;
    VIM_CLEAR(HL_TABLE()[idx].sg_start);
    VIM_CLEAR(HL_TABLE()[idx].sg_stop);
    HL_TABLE()[idx].sg_term_attr = 0;
    HL_TABLE()[idx].sg_cterm = 0;
    HL_TABLE()[idx].sg_cterm_bold = FALSE;
    HL_TABLE()[idx].sg_cterm_fg = 0;
    HL_TABLE()[idx].sg_cterm_bg = 0;
    HL_TABLE()[idx].sg_cterm_attr = 0;
    HL_TABLE()[idx].sg_cterm_font = 0;
#if defined(FEAT_GUI) || defined(FEAT_EVAL)
    HL_TABLE()[idx].sg_gui = 0;
    VIM_CLEAR(HL_TABLE()[idx].sg_gui_fg_name);
    VIM_CLEAR(HL_TABLE()[idx].sg_gui_bg_name);
    VIM_CLEAR(HL_TABLE()[idx].sg_gui_sp_name);
#endif
#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    HL_TABLE()[idx].sg_gui_fg = INVALCOLOR;
    HL_TABLE()[idx].sg_gui_bg = INVALCOLOR;
    HL_TABLE()[idx].sg_gui_sp = INVALCOLOR;
#endif
#ifdef FEAT_GUI
    gui_mch_free_font(HL_TABLE()[idx].sg_font);
    HL_TABLE()[idx].sg_font = NOFONT;
# ifdef FEAT_XFONTSET
    gui_mch_free_fontset(HL_TABLE()[idx].sg_fontset);
    HL_TABLE()[idx].sg_fontset = NOFONTSET;
# endif
    VIM_CLEAR(HL_TABLE()[idx].sg_font_name);
    HL_TABLE()[idx].sg_gui_attr = 0;
#endif
    // Restore default link and context if they exist. Otherwise clears.
    HL_TABLE()[idx].sg_link = HL_TABLE()[idx].sg_deflink;
#ifdef FEAT_EVAL
    // Since we set the default link, set the location to where the default
    // link was set.
    HL_TABLE()[idx].sg_script_ctx = HL_TABLE()[idx].sg_deflink_sctx;
#endif
}

#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS) || defined(PROTO)
/*
 * Set the normal foreground and background colors according to the "Normal"
 * highlighting group.  For X11 also set "Menu", "Scrollbar", and
 * "Tooltip" colors.
 */
    void
set_normal_colors(void)
{
# ifdef FEAT_GUI
#  ifdef FEAT_TERMGUICOLORS
    if (gui.in_use)
#  endif
    {
	if (set_group_colors((char_u *)"Normal",
				 &gui.norm_pixel, &gui.back_pixel,
				 FALSE, TRUE, FALSE))
	{
	    gui_mch_new_colors();
	    set_must_redraw(UPD_CLEAR);
	}
#  ifdef FEAT_GUI_X11
	if (set_group_colors((char_u *)"Menu",
			     &gui.menu_fg_pixel, &gui.menu_bg_pixel,
			     TRUE, FALSE, FALSE))
	{
#   ifdef FEAT_MENU
	    gui_mch_new_menu_colors();
#   endif
	    set_must_redraw(UPD_CLEAR);
	}
#   ifdef FEAT_BEVAL_GUI
	if (set_group_colors((char_u *)"Tooltip",
			     &gui.tooltip_fg_pixel, &gui.tooltip_bg_pixel,
			     FALSE, FALSE, TRUE))
	{
#    ifdef FEAT_TOOLBAR
	    gui_mch_new_tooltip_colors();
#    endif
	    set_must_redraw(UPD_CLEAR);
	}
#   endif
	if (set_group_colors((char_u *)"Scrollbar",
			&gui.scroll_fg_pixel, &gui.scroll_bg_pixel,
			FALSE, FALSE, FALSE))
	{
	    gui_new_scrollbar_colors();
	    set_must_redraw(UPD_CLEAR);
	}
#  endif
    }
# endif
# ifdef FEAT_TERMGUICOLORS
#  ifdef FEAT_GUI
    else
#  endif
    {
	int		idx;

	idx = syn_name2id((char_u *)"Normal") - 1;
	if (idx >= 0)
	{
	    gui_do_one_color(idx, FALSE, FALSE);

	    // If the normal fg or bg color changed a complete redraw is
	    // required.
	    if (cterm_normal_fg_gui_color != HL_TABLE()[idx].sg_gui_fg
		    || cterm_normal_bg_gui_color != HL_TABLE()[idx].sg_gui_bg)
	    {
		// if the GUI color is INVALCOLOR then we use the default cterm
		// color
		cterm_normal_fg_gui_color = HL_TABLE()[idx].sg_gui_fg;
		cterm_normal_bg_gui_color = HL_TABLE()[idx].sg_gui_bg;
		set_must_redraw(UPD_CLEAR);
	    }
	}
    }
# endif
}
#endif

#if defined(FEAT_GUI) || defined(PROTO)
/*
 * Set the colors for "Normal", "Menu", "Tooltip" or "Scrollbar".
 */
    static int
set_group_colors(
    char_u	*name,
    guicolor_T	*fgp,
    guicolor_T	*bgp,
    int		do_menu,
    int		use_norm,
    int		do_tooltip)
{
    int		idx;

    idx = syn_name2id(name) - 1;
    if (idx < 0)
	return FALSE;

    gui_do_one_color(idx, do_menu, do_tooltip);

    if (HL_TABLE()[idx].sg_gui_fg != INVALCOLOR)
	*fgp = HL_TABLE()[idx].sg_gui_fg;
    else if (use_norm)
	*fgp = gui.def_norm_pixel;
    if (HL_TABLE()[idx].sg_gui_bg != INVALCOLOR)
	*bgp = HL_TABLE()[idx].sg_gui_bg;
    else if (use_norm)
	*bgp = gui.def_back_pixel;
    return TRUE;
}

/*
 * Get the font of the "Normal" group.
 * Returns "" when it's not found or not set.
 */
    char_u *
hl_get_font_name(void)
{
    int		id;
    char_u	*s;

    id = syn_name2id((char_u *)"Normal");
    if (id > 0)
    {
	s = HL_TABLE()[id - 1].sg_font_name;
	if (s != NULL)
	    return s;
    }
    return (char_u *)"";
}

/*
 * Set font for "Normal" group.  Called by gui_mch_init_font() when a font has
 * actually chosen to be used.
 */
    void
hl_set_font_name(char_u *font_name)
{
    int	    id;

    id = syn_name2id((char_u *)"Normal");
    if (id <= 0)
	return;

    vim_free(HL_TABLE()[id - 1].sg_font_name);
    HL_TABLE()[id - 1].sg_font_name = vim_strsave(font_name);
}

/*
 * Set background color for "Normal" group.  Called by gui_set_bg_color()
 * when the color is known.
 */
    void
hl_set_bg_color_name(
    char_u  *name)	    // must have been allocated
{
    int	    id;

    if (name == NULL)
	return;

    id = syn_name2id((char_u *)"Normal");
    if (id <= 0)
	return;

    vim_free(HL_TABLE()[id - 1].sg_gui_bg_name);
    HL_TABLE()[id - 1].sg_gui_bg_name = name;
}

/*
 * Set foreground color for "Normal" group.  Called by gui_set_fg_color()
 * when the color is known.
 */
    void
hl_set_fg_color_name(
    char_u  *name)	    // must have been allocated
{
    int	    id;

    if (name == NULL)
	return;

    id = syn_name2id((char_u *)"Normal");
    if (id <= 0)
	return;

    vim_free(HL_TABLE()[id - 1].sg_gui_fg_name);
    HL_TABLE()[id - 1].sg_gui_fg_name = name;
}

/*
 * Return the handle for a font name.
 * Returns NOFONT when failed.
 */
    static GuiFont
font_name2handle(char_u *name)
{
    if (STRCMP(name, "NONE") == 0)
	return NOFONT;

    return gui_mch_get_font(name, TRUE);
}

# ifdef FEAT_XFONTSET
/*
 * Return the handle for a fontset name.
 * Returns NOFONTSET when failed.
 */
    static GuiFontset
fontset_name2handle(char_u *name, int fixed_width)
{
    if (STRCMP(name, "NONE") == 0)
	return NOFONTSET;

    return gui_mch_get_fontset(name, TRUE, fixed_width);
}
# endif

/*
 * Get the font or fontset for one highlight group.
 */
    static void
hl_do_font(
    int		idx,
    char_u	*arg,
    int		do_normal,		// set normal font
    int		do_menu UNUSED,		// set menu font
    int		do_tooltip UNUSED,	// set tooltip font
    int		free_font)		// free current font/fontset
{
# ifdef FEAT_XFONTSET
    // If 'guifontset' is not empty, first try using the name as a
    // fontset.  If that doesn't work, use it as a font name.
    if (*p_guifontset != NUL
#  ifdef FONTSET_ALWAYS
	|| do_menu
#  endif
#  ifdef FEAT_BEVAL_TIP
	// In Motif, the Tooltip highlight group is always a fontset
	|| do_tooltip
#  endif
	    )
    {
	if (free_font)
	    gui_mch_free_fontset(HL_TABLE()[idx].sg_fontset);
	HL_TABLE()[idx].sg_fontset = fontset_name2handle(arg, 0
#  ifdef FONTSET_ALWAYS
		|| do_menu
#  endif
#  ifdef FEAT_BEVAL_TIP
		|| do_tooltip
#  endif
		);
    }
    if (HL_TABLE()[idx].sg_fontset != NOFONTSET)
    {
	// If it worked and it's the Normal group, use it as the normal
	// fontset.  Same for the Menu group.
	if (do_normal)
	    gui_init_font(arg, TRUE);
#   if defined(FEAT_GUI_MOTIF) && defined(FEAT_MENU)
	if (do_menu)
	{
#    ifdef FONTSET_ALWAYS
	    gui.menu_fontset = HL_TABLE()[idx].sg_fontset;
#    else
	    // YIKES!  This is a bug waiting to crash the program
	    gui.menu_font = HL_TABLE()[idx].sg_fontset;
#    endif
	    gui_mch_new_menu_font();
	}
#    ifdef FEAT_BEVAL_GUI
	if (do_tooltip)
	{
	    // The Athena widget set could not handle switching between
	    // displaying a single font and a fontset.
	    // If the XtNinternational resource is set to True at widget
	    // creation, then a fontset is always used, otherwise an
	    // XFontStruct is used.
	    gui.tooltip_fontset = (XFontSet)HL_TABLE()[idx].sg_fontset;
	    gui_mch_new_tooltip_font();
	}
#    endif
#   endif
    }
    else
# endif
    {
	if (free_font)
	    gui_mch_free_font(HL_TABLE()[idx].sg_font);
	HL_TABLE()[idx].sg_font = font_name2handle(arg);
	// If it worked and it's the Normal group, use it as the
	// normal font.  Same for the Menu group.
	if (HL_TABLE()[idx].sg_font != NOFONT)
	{
	    if (do_normal)
		gui_init_font(arg, FALSE);
#ifndef FONTSET_ALWAYS
# if defined(FEAT_GUI_MOTIF) && defined(FEAT_MENU)
	    if (do_menu)
	    {
		gui.menu_font = HL_TABLE()[idx].sg_font;
		gui_mch_new_menu_font();
	    }
# endif
#endif
	}
    }
}

#endif // FEAT_GUI

#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS) || defined(PROTO)
/*
 * Return the handle for a color name.
 * Returns INVALCOLOR when failed.
 */
    guicolor_T
color_name2handle(char_u *name)
{
    if (STRCMP(name, "NONE") == 0)
	return INVALCOLOR;

    if (STRICMP(name, "fg") == 0 || STRICMP(name, "foreground") == 0)
    {
#if defined(FEAT_TERMGUICOLORS) && defined(FEAT_GUI)
	if (gui.in_use)
#endif
#ifdef FEAT_GUI
	    return gui.norm_pixel;
#endif
#ifdef FEAT_TERMGUICOLORS
	if (cterm_normal_fg_gui_color != INVALCOLOR)
	    return cterm_normal_fg_gui_color;
	// Guess that the foreground is black or white.
	return GUI_GET_COLOR((char_u *)(*p_bg == 'l' ? "black" : "white"));
#endif
    }
    if (STRICMP(name, "bg") == 0 || STRICMP(name, "background") == 0)
    {
#if defined(FEAT_TERMGUICOLORS) && defined(FEAT_GUI)
	if (gui.in_use)
#endif
#ifdef FEAT_GUI
	    return gui.back_pixel;
#endif
#ifdef FEAT_TERMGUICOLORS
	if (cterm_normal_bg_gui_color != INVALCOLOR)
	    return cterm_normal_bg_gui_color;
	// Guess that the background is white or black.
	return GUI_GET_COLOR((char_u *)(*p_bg == 'l' ? "white" : "black"));
#endif
    }

    return GUI_GET_COLOR(name);
}

// On MS-Windows an RGB macro is available and it produces 0x00bbggrr color
// values as used by the MS-Windows GDI api.  It should be used only for
// MS-Windows GDI builds.
# if defined(RGB) && defined(MSWIN) && !defined(FEAT_GUI)
#  undef RGB
# endif
# ifndef RGB
#  define RGB(r, g, b)	(((r)<<16) | ((g)<<8) | (b))
# endif

# ifdef VIMDLL
    static guicolor_T
gui_adjust_rgb(guicolor_T c)
{
    if (gui.in_use)
	return c;
    else
	return ((c & 0xff) << 16) | (c & 0x00ff00) | ((c >> 16) & 0xff);
}
# else
#  define gui_adjust_rgb(c) (c)
# endif

    static int
hex_digit(int c)
{
    if (SAFE_isdigit(c))
	return c - '0';
    c = TOLOWER_ASC(c);
    if (c >= 'a' && c <= 'f')
	return c - 'a' + 10;
    return 0x1ffffff;
}

    static guicolor_T
decode_hex_color(char_u *hex)
{
    guicolor_T color;

    if (hex[0] != '#' || STRLEN(hex) != 7)
	return INVALCOLOR;

    // Name is in "#rrggbb" format
    color = RGB(((hex_digit(hex[1]) << 4) + hex_digit(hex[2])),
		((hex_digit(hex[3]) << 4) + hex_digit(hex[4])),
		((hex_digit(hex[5]) << 4) + hex_digit(hex[6])));
    if (color > 0xffffff)
	return INVALCOLOR;
    return gui_adjust_rgb(color);
}

#ifdef FEAT_EVAL
// Returns the color currently mapped to the given name or INVALCOLOR if no
// such name exists in the color table. The convention is to use lowercase for
// all keys in the v:colornames dictionary. The value can be either a string in
// the form #rrggbb or a number, either of which is converted to a guicolor_T.
    static guicolor_T
colorname2rgb(char_u *name)
{
    dict_T      *colornames_table = get_vim_var_dict(VV_COLORNAMES);
    char_u      *lc_name;
    dictitem_T  *colentry;
    char_u      *colstr;
    varnumber_T colnum;

    lc_name = strlow_save(name);
    if (lc_name == NULL)
	return INVALCOLOR;

    colentry = dict_find(colornames_table, lc_name, -1);
    vim_free(lc_name);
    if (colentry == NULL)
	return INVALCOLOR;

    if (colentry->di_tv.v_type == VAR_STRING)
    {
	colstr = tv_get_string_strict(&colentry->di_tv);
	if ((STRLEN(colstr) == 7) && (*colstr == '#'))
	{
	    return decode_hex_color(colstr);
	}
	else
	{
	    semsg(_(e_bad_color_string_str), colstr);
	    return INVALCOLOR;
	}
    }

    if (colentry->di_tv.v_type == VAR_NUMBER)
    {
	colnum = tv_get_number(&colentry->di_tv);
	return (guicolor_T)colnum;
    }

    return INVALCOLOR;
}

#endif

    guicolor_T
gui_get_color_cmn(char_u *name)
{
    int		i;
    guicolor_T  color;

    struct rgbcolor_table_S {
	char_u	    *color_name;
	guicolor_T  color;
    };

    // Only non X11 colors (not present in rgb.txt) and colors in
    // color_names[], useful when $VIMRUNTIME is not found,.
    static struct rgbcolor_table_S rgb_table[] = {
	    {(char_u *)"black",		RGB(0x00, 0x00, 0x00)},
	    {(char_u *)"blue",		RGB(0x00, 0x00, 0xFF)},
	    {(char_u *)"brown",		RGB(0xA5, 0x2A, 0x2A)},
	    {(char_u *)"cyan",		RGB(0x00, 0xFF, 0xFF)},
	    {(char_u *)"darkblue",	RGB(0x00, 0x00, 0x8B)},
	    {(char_u *)"darkcyan",	RGB(0x00, 0x8B, 0x8B)},
	    {(char_u *)"darkgray",	RGB(0xA9, 0xA9, 0xA9)},
	    {(char_u *)"darkgreen",	RGB(0x00, 0x64, 0x00)},
	    {(char_u *)"darkgrey",	RGB(0xA9, 0xA9, 0xA9)},
	    {(char_u *)"darkmagenta",	RGB(0x8B, 0x00, 0x8B)},
	    {(char_u *)"darkred",	RGB(0x8B, 0x00, 0x00)},
	    {(char_u *)"darkyellow",	RGB(0x8B, 0x8B, 0x00)}, // No X11
	    {(char_u *)"gray",		RGB(0xBE, 0xBE, 0xBE)},
	    {(char_u *)"green",		RGB(0x00, 0xFF, 0x00)},
	    {(char_u *)"grey",		RGB(0xBE, 0xBE, 0xBE)},
	    {(char_u *)"grey40",	RGB(0x66, 0x66, 0x66)},
	    {(char_u *)"grey50",	RGB(0x7F, 0x7F, 0x7F)},
	    {(char_u *)"grey90",	RGB(0xE5, 0xE5, 0xE5)},
	    {(char_u *)"lightblue",	RGB(0xAD, 0xD8, 0xE6)},
	    {(char_u *)"lightcyan",	RGB(0xE0, 0xFF, 0xFF)},
	    {(char_u *)"lightgray",	RGB(0xD3, 0xD3, 0xD3)},
	    {(char_u *)"lightgreen",	RGB(0x90, 0xEE, 0x90)},
	    {(char_u *)"lightgrey",	RGB(0xD3, 0xD3, 0xD3)},
	    {(char_u *)"lightmagenta",	RGB(0xFF, 0x8B, 0xFF)}, // No X11
	    {(char_u *)"lightred",	RGB(0xFF, 0x8B, 0x8B)}, // No X11
	    {(char_u *)"lightyellow",	RGB(0xFF, 0xFF, 0xE0)},
	    {(char_u *)"magenta",	RGB(0xFF, 0x00, 0xFF)},
	    {(char_u *)"red",		RGB(0xFF, 0x00, 0x00)},
	    {(char_u *)"seagreen",	RGB(0x2E, 0x8B, 0x57)},
	    {(char_u *)"white",		RGB(0xFF, 0xFF, 0xFF)},
	    {(char_u *)"yellow",	RGB(0xFF, 0xFF, 0x00)},
    };

    color = decode_hex_color(name);
    if (color != INVALCOLOR)
	return color;

    // Check if the name is one of the colors we know
    for (i = 0; i < (int)ARRAY_LENGTH(rgb_table); i++)
	if (STRICMP(name, rgb_table[i].color_name) == 0)
	    return gui_adjust_rgb(rgb_table[i].color);

#if defined(FEAT_EVAL)
    /*
     * Not a traditional color. Load additional color aliases and then consult the alias table.
     */

    color = colorname2rgb(name);
    if (color == INVALCOLOR)
    {
	load_default_colors_lists();
	color = colorname2rgb(name);
    }

    return color;
#else
    return INVALCOLOR;
#endif
}

    guicolor_T
gui_get_rgb_color_cmn(int r, int g, int b)
{
    guicolor_T  color = RGB(r, g, b);

    if (color > 0xffffff)
	return INVALCOLOR;
    return gui_adjust_rgb(color);
}
#endif

/*
 * Table with the specifications for an attribute number.
 * Note that this table is used by ALL buffers.  This is required because the
 * GUI can redraw at any time for any buffer.
 */
static garray_T	term_attr_table = {0, 0, 0, 0, NULL};

#define TERM_ATTR_ENTRY(idx) ((attrentry_T *)term_attr_table.ga_data)[idx]

static garray_T	cterm_attr_table = {0, 0, 0, 0, NULL};

#define CTERM_ATTR_ENTRY(idx) ((attrentry_T *)cterm_attr_table.ga_data)[idx]

#ifdef FEAT_GUI
static garray_T	gui_attr_table = {0, 0, 0, 0, NULL};

#define GUI_ATTR_ENTRY(idx) ((attrentry_T *)gui_attr_table.ga_data)[idx]
#endif

/*
 * Return the attr number for a set of colors and font.
 * Add a new entry to the term_attr_table, cterm_attr_table or gui_attr_table
 * if the combination is new.
 * Return 0 for error (no more room).
 */
    static int
get_attr_entry(garray_T *table, attrentry_T *aep)
{
    int		i;
    attrentry_T	*taep;
    static int	recursive = FALSE;

    // Init the table, in case it wasn't done yet.
    table->ga_itemsize = sizeof(attrentry_T);
    table->ga_growsize = 7;

    // Try to find an entry with the same specifications.
    for (i = 0; i < table->ga_len; ++i)
    {
	taep = &(((attrentry_T *)table->ga_data)[i]);
	if (	   aep->ae_attr == taep->ae_attr
		&& (
#ifdef FEAT_GUI
		       (table == &gui_attr_table
			&& (aep->ae_u.gui.fg_color == taep->ae_u.gui.fg_color
			    && aep->ae_u.gui.bg_color
						    == taep->ae_u.gui.bg_color
			    && aep->ae_u.gui.sp_color
						    == taep->ae_u.gui.sp_color
			    && aep->ae_u.gui.font == taep->ae_u.gui.font
#  ifdef FEAT_XFONTSET
			    && aep->ae_u.gui.fontset == taep->ae_u.gui.fontset
#  endif
			    ))
		    ||
#endif
		       (table == &term_attr_table
			&& (aep->ae_u.term.start == NULL)
					    == (taep->ae_u.term.start == NULL)
			&& (aep->ae_u.term.start == NULL
			    || STRCMP(aep->ae_u.term.start,
						  taep->ae_u.term.start) == 0)
			&& (aep->ae_u.term.stop == NULL)
					     == (taep->ae_u.term.stop == NULL)
			&& (aep->ae_u.term.stop == NULL
			    || STRCMP(aep->ae_u.term.stop,
						  taep->ae_u.term.stop) == 0))
		    || (table == &cterm_attr_table
			    && aep->ae_u.cterm.fg_color
						  == taep->ae_u.cterm.fg_color
			    && aep->ae_u.cterm.bg_color
						  == taep->ae_u.cterm.bg_color
			    && aep->ae_u.cterm.ul_color
						  == taep->ae_u.cterm.ul_color
			    && aep->ae_u.cterm.font
						  == taep->ae_u.cterm.font
#ifdef FEAT_TERMGUICOLORS
			    && aep->ae_u.cterm.fg_rgb
						    == taep->ae_u.cterm.fg_rgb
			    && aep->ae_u.cterm.bg_rgb
						    == taep->ae_u.cterm.bg_rgb
			    && aep->ae_u.cterm.ul_rgb
						    == taep->ae_u.cterm.ul_rgb
#endif
		       )))

	return i + ATTR_OFF;
    }

    if (table->ga_len + ATTR_OFF > MAX_TYPENR)
    {
	// Running out of attribute entries!  remove all attributes, and
	// compute new ones for all groups.
	// When called recursively, we are really out of numbers.
	if (recursive)
	{
	    emsg(_(e_too_many_different_highlighting_attributes_in_use));
	    return 0;
	}
	recursive = TRUE;

	clear_hl_tables();

	set_must_redraw(UPD_CLEAR);

	for (i = 0; i < highlight_ga.ga_len; ++i)
	    set_hl_attr(i);

	recursive = FALSE;
    }

    // This is a new combination of colors and font, add an entry.
    if (ga_grow(table, 1) == FAIL)
	return 0;

    taep = &(((attrentry_T *)table->ga_data)[table->ga_len]);
    CLEAR_POINTER(taep);
    taep->ae_attr = aep->ae_attr;
#ifdef FEAT_GUI
    if (table == &gui_attr_table)
    {
	taep->ae_u.gui.fg_color = aep->ae_u.gui.fg_color;
	taep->ae_u.gui.bg_color = aep->ae_u.gui.bg_color;
	taep->ae_u.gui.sp_color = aep->ae_u.gui.sp_color;
	taep->ae_u.gui.font = aep->ae_u.gui.font;
# ifdef FEAT_XFONTSET
	taep->ae_u.gui.fontset = aep->ae_u.gui.fontset;
# endif
    }
#endif
    if (table == &term_attr_table)
    {
	if (aep->ae_u.term.start == NULL)
	    taep->ae_u.term.start = NULL;
	else
	    taep->ae_u.term.start = vim_strsave(aep->ae_u.term.start);
	if (aep->ae_u.term.stop == NULL)
	    taep->ae_u.term.stop = NULL;
	else
	    taep->ae_u.term.stop = vim_strsave(aep->ae_u.term.stop);
    }
    else if (table == &cterm_attr_table)
    {
	taep->ae_u.cterm.fg_color = aep->ae_u.cterm.fg_color;
	taep->ae_u.cterm.bg_color = aep->ae_u.cterm.bg_color;
	taep->ae_u.cterm.ul_color = aep->ae_u.cterm.ul_color;
	taep->ae_u.cterm.font = aep->ae_u.cterm.font;
#ifdef FEAT_TERMGUICOLORS
	taep->ae_u.cterm.fg_rgb = aep->ae_u.cterm.fg_rgb;
	taep->ae_u.cterm.bg_rgb = aep->ae_u.cterm.bg_rgb;
	taep->ae_u.cterm.ul_rgb = aep->ae_u.cterm.ul_rgb;
#endif
    }
    ++table->ga_len;
    return (table->ga_len - 1 + ATTR_OFF);
}

#if defined(FEAT_TERMINAL) || defined(PROTO)
/*
 * Get an attribute index for a cterm entry.
 * Uses an existing entry when possible or adds one when needed.
 */
    int
get_cterm_attr_idx(int attr, int fg, int bg)
{
    attrentry_T		at_en;

    CLEAR_FIELD(at_en);
#ifdef FEAT_TERMGUICOLORS
    at_en.ae_u.cterm.fg_rgb = INVALCOLOR;
    at_en.ae_u.cterm.bg_rgb = INVALCOLOR;
    at_en.ae_u.cterm.ul_rgb = INVALCOLOR;
#endif
    at_en.ae_attr = attr;
    at_en.ae_u.cterm.fg_color = fg;
    at_en.ae_u.cterm.bg_color = bg;
    at_en.ae_u.cterm.ul_color = 0;
    at_en.ae_u.cterm.font = 0;
    return get_attr_entry(&cterm_attr_table, &at_en);
}
#endif

#if (defined(FEAT_TERMINAL) && defined(FEAT_TERMGUICOLORS)) || defined(PROTO)
/*
 * Get an attribute index for a 'termguicolors' entry.
 * Uses an existing entry when possible or adds one when needed.
 */
    int
get_tgc_attr_idx(int attr, guicolor_T fg, guicolor_T bg)
{
    attrentry_T		at_en;

    CLEAR_FIELD(at_en);
    at_en.ae_attr = attr;
    if (fg == INVALCOLOR && bg == INVALCOLOR)
    {
	// If both GUI colors are not set fall back to the cterm colors.  Helps
	// if the GUI only has an attribute, such as undercurl.
	at_en.ae_u.cterm.fg_rgb = CTERMCOLOR;
	at_en.ae_u.cterm.bg_rgb = CTERMCOLOR;
    }
    else
    {
	at_en.ae_u.cterm.fg_rgb = fg;
	at_en.ae_u.cterm.bg_rgb = bg;
    }
    at_en.ae_u.cterm.ul_rgb = INVALCOLOR;
    return get_attr_entry(&cterm_attr_table, &at_en);
}
#endif

#if (defined(FEAT_TERMINAL) && defined(FEAT_GUI)) || defined(PROTO)
/*
 * Get an attribute index for a cterm entry.
 * Uses an existing entry when possible or adds one when needed.
 */
    int
get_gui_attr_idx(int attr, guicolor_T fg, guicolor_T bg)
{
    attrentry_T		at_en;

    CLEAR_FIELD(at_en);
    at_en.ae_attr = attr;
    at_en.ae_u.gui.fg_color = fg;
    at_en.ae_u.gui.bg_color = bg;
    return get_attr_entry(&gui_attr_table, &at_en);
}
#endif

/*
 * Clear all highlight tables.
 */
    void
clear_hl_tables(void)
{
    int		i;
    attrentry_T	*taep;

#ifdef FEAT_GUI
    ga_clear(&gui_attr_table);
#endif
    for (i = 0; i < term_attr_table.ga_len; ++i)
    {
	taep = &(((attrentry_T *)term_attr_table.ga_data)[i]);
	vim_free(taep->ae_u.term.start);
	vim_free(taep->ae_u.term.stop);
    }
    ga_clear(&term_attr_table);
    ga_clear(&cterm_attr_table);
}

/*
 * Combine special attributes (e.g., for spelling) with other attributes
 * (e.g., for syntax highlighting).
 * "prim_attr" overrules "char_attr".
 * This creates a new group when required.
 * Since we expect there to be few spelling mistakes we don't cache the
 * result.
 * Return the resulting attributes.
 */
    int
hl_combine_attr(int char_attr, int prim_attr)
{
    attrentry_T *char_aep = NULL;
    attrentry_T *prim_aep;
    attrentry_T new_en;

    if (char_attr == 0)
	return prim_attr;
    if (char_attr <= HL_ALL && prim_attr <= HL_ALL)
	return ATTR_COMBINE(char_attr, prim_attr);
#ifdef FEAT_GUI
    if (gui.in_use)
    {
	if (char_attr > HL_ALL)
	    char_aep = syn_gui_attr2entry(char_attr);
	if (char_aep != NULL)
	    new_en = *char_aep;
	else
	{
	    CLEAR_FIELD(new_en);
	    new_en.ae_u.gui.fg_color = INVALCOLOR;
	    new_en.ae_u.gui.bg_color = INVALCOLOR;
	    new_en.ae_u.gui.sp_color = INVALCOLOR;
	    if (char_attr <= HL_ALL)
		new_en.ae_attr = char_attr;
	}

	if (prim_attr <= HL_ALL)
	    new_en.ae_attr = ATTR_COMBINE(new_en.ae_attr, prim_attr);
	else
	{
	    prim_aep = syn_gui_attr2entry(prim_attr);
	    if (prim_aep != NULL)
	    {
		new_en.ae_attr = ATTR_COMBINE(new_en.ae_attr,
							   prim_aep->ae_attr);
		if (prim_aep->ae_u.gui.fg_color != INVALCOLOR)
		    new_en.ae_u.gui.fg_color = prim_aep->ae_u.gui.fg_color;
		if (prim_aep->ae_u.gui.bg_color != INVALCOLOR)
		    new_en.ae_u.gui.bg_color = prim_aep->ae_u.gui.bg_color;
		if (prim_aep->ae_u.gui.sp_color != INVALCOLOR)
		    new_en.ae_u.gui.sp_color = prim_aep->ae_u.gui.sp_color;
		if (prim_aep->ae_u.gui.font != NOFONT)
		    new_en.ae_u.gui.font = prim_aep->ae_u.gui.font;
# ifdef FEAT_XFONTSET
		if (prim_aep->ae_u.gui.fontset != NOFONTSET)
		    new_en.ae_u.gui.fontset = prim_aep->ae_u.gui.fontset;
# endif
	    }
	}
	return get_attr_entry(&gui_attr_table, &new_en);
    }
#endif

    if (IS_CTERM)
    {
	if (char_attr > HL_ALL)
	    char_aep = syn_cterm_attr2entry(char_attr);
	if (char_aep != NULL)
	    new_en = *char_aep;
	else
	{
	    CLEAR_FIELD(new_en);
#ifdef FEAT_TERMGUICOLORS
	    new_en.ae_u.cterm.bg_rgb = INVALCOLOR;
	    new_en.ae_u.cterm.fg_rgb = INVALCOLOR;
	    new_en.ae_u.cterm.ul_rgb = INVALCOLOR;
#endif
	    if (char_attr <= HL_ALL)
		new_en.ae_attr = char_attr;
	}

	if (prim_attr <= HL_ALL)
		new_en.ae_attr = ATTR_COMBINE(new_en.ae_attr, prim_attr);
	else
	{
	    prim_aep = syn_cterm_attr2entry(prim_attr);
	    if (prim_aep != NULL)
	    {
		new_en.ae_attr = ATTR_COMBINE(new_en.ae_attr,
							   prim_aep->ae_attr);
		if (prim_aep->ae_u.cterm.fg_color > 0)
		    new_en.ae_u.cterm.fg_color = prim_aep->ae_u.cterm.fg_color;
		if (prim_aep->ae_u.cterm.bg_color > 0)
		    new_en.ae_u.cterm.bg_color = prim_aep->ae_u.cterm.bg_color;
		if (prim_aep->ae_u.cterm.ul_color > 0)
		    new_en.ae_u.cterm.ul_color = prim_aep->ae_u.cterm.ul_color;
		if (prim_aep->ae_u.cterm.font > 0)
		    new_en.ae_u.cterm.font = prim_aep->ae_u.cterm.font;
#ifdef FEAT_TERMGUICOLORS
		// If both fg and bg are not set fall back to cterm colors.
		// Helps for SpellBad which uses undercurl in the GUI.
		if (COLOR_INVALID(prim_aep->ae_u.cterm.fg_rgb)
			&& COLOR_INVALID(prim_aep->ae_u.cterm.bg_rgb))
		{
		    if (prim_aep->ae_u.cterm.fg_color > 0)
			new_en.ae_u.cterm.fg_rgb = CTERMCOLOR;
		    if (prim_aep->ae_u.cterm.bg_color > 0)
			new_en.ae_u.cterm.bg_rgb = CTERMCOLOR;
		}
		else
		{
		    if (prim_aep->ae_u.cterm.fg_rgb != INVALCOLOR)
			new_en.ae_u.cterm.fg_rgb = prim_aep->ae_u.cterm.fg_rgb;
		    if (prim_aep->ae_u.cterm.bg_rgb != INVALCOLOR)
			new_en.ae_u.cterm.bg_rgb = prim_aep->ae_u.cterm.bg_rgb;
		}
		if (prim_aep->ae_u.cterm.ul_rgb != INVALCOLOR)
		    new_en.ae_u.cterm.ul_rgb = prim_aep->ae_u.cterm.ul_rgb;
#endif
	    }
	}
	return get_attr_entry(&cterm_attr_table, &new_en);
    }

    if (char_attr > HL_ALL)
	char_aep = syn_term_attr2entry(char_attr);
    if (char_aep != NULL)
	new_en = *char_aep;
    else
    {
	CLEAR_FIELD(new_en);
	if (char_attr <= HL_ALL)
	    new_en.ae_attr = char_attr;
    }

    if (prim_attr <= HL_ALL)
	new_en.ae_attr = ATTR_COMBINE(new_en.ae_attr, prim_attr);
    else
    {
	prim_aep = syn_term_attr2entry(prim_attr);
	if (prim_aep != NULL)
	{
	    new_en.ae_attr = ATTR_COMBINE(new_en.ae_attr, prim_aep->ae_attr);
	    if (prim_aep->ae_u.term.start != NULL)
	    {
		new_en.ae_u.term.start = prim_aep->ae_u.term.start;
		new_en.ae_u.term.stop = prim_aep->ae_u.term.stop;
	    }
	}
    }
    return get_attr_entry(&term_attr_table, &new_en);
}

#ifdef FEAT_GUI
    attrentry_T *
syn_gui_attr2entry(int attr)
{
    attr -= ATTR_OFF;
    if (attr >= gui_attr_table.ga_len)	    // did ":syntax clear"
	return NULL;
    return &(GUI_ATTR_ENTRY(attr));
}
#endif

/*
 * Get the highlight attributes (HL_BOLD etc.) from an attribute nr.
 * Only to be used when "attr" > HL_ALL.
 */
    int
syn_attr2attr(int attr)
{
    attrentry_T	*aep;

#ifdef FEAT_GUI
    if (gui.in_use)
	aep = syn_gui_attr2entry(attr);
    else
#endif
	if (IS_CTERM)
	    aep = syn_cterm_attr2entry(attr);
	else
	    aep = syn_term_attr2entry(attr);

    if (aep == NULL)	    // highlighting not set
	return 0;
    return aep->ae_attr;
}


    attrentry_T *
syn_term_attr2entry(int attr)
{
    attr -= ATTR_OFF;
    if (attr >= term_attr_table.ga_len)	    // did ":syntax clear"
	return NULL;
    return &(TERM_ATTR_ENTRY(attr));
}

    attrentry_T *
syn_cterm_attr2entry(int attr)
{
    attr -= ATTR_OFF;
    if (attr >= cterm_attr_table.ga_len)	// did ":syntax clear"
	return NULL;
    return &(CTERM_ATTR_ENTRY(attr));
}

#define LIST_ATTR   1
#define LIST_STRING 2
#define LIST_INT    3

    static void
highlight_list_one(int id)
{
    hl_group_T	    *sgp;
    int		    didh = FALSE;

    sgp = &HL_TABLE()[id - 1];	    // index is ID minus one

    if (message_filtered(sgp->sg_name))
	return;

    didh = highlight_list_arg(id, didh, LIST_ATTR,
				    sgp->sg_term, NULL, "term");
    didh = highlight_list_arg(id, didh, LIST_STRING,
				    0, sgp->sg_start, "start");
    didh = highlight_list_arg(id, didh, LIST_STRING,
				    0, sgp->sg_stop, "stop");

    didh = highlight_list_arg(id, didh, LIST_ATTR,
				    sgp->sg_cterm, NULL, "cterm");
    didh = highlight_list_arg(id, didh, LIST_INT,
				    sgp->sg_cterm_fg, NULL, "ctermfg");
    didh = highlight_list_arg(id, didh, LIST_INT,
				    sgp->sg_cterm_bg, NULL, "ctermbg");
    didh = highlight_list_arg(id, didh, LIST_INT,
				    sgp->sg_cterm_ul, NULL, "ctermul");
    didh = highlight_list_arg(id, didh, LIST_INT,
				    sgp->sg_cterm_font, NULL, "ctermfont");

#if defined(FEAT_GUI) || defined(FEAT_EVAL)
    didh = highlight_list_arg(id, didh, LIST_ATTR,
				    sgp->sg_gui, NULL, "gui");
    didh = highlight_list_arg(id, didh, LIST_STRING,
				    0, sgp->sg_gui_fg_name, "guifg");
    didh = highlight_list_arg(id, didh, LIST_STRING,
				    0, sgp->sg_gui_bg_name, "guibg");
    didh = highlight_list_arg(id, didh, LIST_STRING,
				    0, sgp->sg_gui_sp_name, "guisp");
#endif
#ifdef FEAT_GUI
    didh = highlight_list_arg(id, didh, LIST_STRING,
				    0, sgp->sg_font_name, "font");
#endif

    if (sgp->sg_link && !got_int)
    {
	(void)syn_list_header(didh, 9999, id);
	didh = TRUE;
	msg_puts_attr("links to", HL_ATTR(HLF_D));
	msg_putchar(' ');
	msg_outtrans(HL_TABLE()[HL_TABLE()[id - 1].sg_link - 1].sg_name);
    }

    if (!didh)
	highlight_list_arg(id, didh, LIST_STRING, 0, (char_u *)"cleared", "");
#ifdef FEAT_EVAL
    if (p_verbose > 0)
	last_set_msg(sgp->sg_script_ctx);
#endif
}

    static int
highlight_list_arg(
    int		id,
    int		didh,
    int		type,
    int		iarg,
    char_u	*sarg,
    char	*name)
{
    char_u	buf[MAX_ATTR_LEN];
    char_u	*ts;
    int		i;

    if (got_int)
	return FALSE;

    if (type == LIST_STRING ? (sarg == NULL) : (iarg == 0))
	return didh;

    ts = buf;
    if (type == LIST_INT)
	sprintf((char *)buf, "%d", iarg - 1);
    else if (type == LIST_STRING)
	ts = sarg;
    else // type == LIST_ATTR
    {
	buf[0] = NUL;
	for (i = 0; hl_attr_table[i] != 0; ++i)
	{
	    if (iarg & hl_attr_table[i])
	    {
		if (buf[0] != NUL)
		    vim_strcat(buf, (char_u *)",", MAX_ATTR_LEN);
		vim_strcat(buf, (char_u *)hl_name_table[i], MAX_ATTR_LEN);
		iarg &= ~hl_attr_table[i];	    // don't want "inverse"
	    }
	}
    }

    (void)syn_list_header(didh,
	    (int)(vim_strsize(ts) + STRLEN(name) + 1), id);
    didh = TRUE;
    if (!got_int)
    {
	if (*name != NUL)
	{
	    msg_puts_attr(name, HL_ATTR(HLF_D));
	    msg_puts_attr("=", HL_ATTR(HLF_D));
	}
	msg_outtrans(ts);
    }
    return didh;
}

#if (((defined(FEAT_EVAL) || defined(FEAT_PRINTER))) && defined(FEAT_SYN_HL)) || defined(PROTO)
/*
 * Return "1" if highlight group "id" has attribute "flag".
 * Return NULL otherwise.
 */
    char_u *
highlight_has_attr(
    int		id,
    int		flag,
    int		modec)	// 'g' for GUI, 'c' for cterm, 't' for term
{
    int		attr;

    if (id <= 0 || id > highlight_ga.ga_len)
	return NULL;

#if defined(FEAT_GUI) || defined(FEAT_EVAL)
    if (modec == 'g')
	attr = HL_TABLE()[id - 1].sg_gui;
    else
#endif
    {
	if (modec == 'c')
	    attr = HL_TABLE()[id - 1].sg_cterm;
	else
	    attr = HL_TABLE()[id - 1].sg_term;
    }

    if (attr & flag)
	return (char_u *)"1";
    return NULL;
}
#endif

#if (defined(FEAT_SYN_HL) && defined(FEAT_EVAL)) || defined(PROTO)
/*
 * Return color name of highlight group "id".
 */
    char_u *
highlight_color(
    int		id,
    char_u	*what,	// "font", "fg", "bg", "sp", "ul", "fg#", "bg#" or "sp#"
    int		modec)	// 'g' for GUI, 'c' for cterm, 't' for term
{
    static char_u	name[20];
    int			n;
    int			fg = FALSE;
    int			sp = FALSE;
    int			ul = FALSE;
    int			font = FALSE;

    if (id <= 0 || id > highlight_ga.ga_len)
	return NULL;

    if (TOLOWER_ASC(what[0]) == 'f' && TOLOWER_ASC(what[1]) == 'g')
	fg = TRUE;
    else if (TOLOWER_ASC(what[0]) == 'f' && TOLOWER_ASC(what[1]) == 'o'
	     && TOLOWER_ASC(what[2]) == 'n' && TOLOWER_ASC(what[3]) == 't')
	font = TRUE;
    else if (TOLOWER_ASC(what[0]) == 's' && TOLOWER_ASC(what[1]) == 'p')
	sp = TRUE;
    else if (TOLOWER_ASC(what[0]) == 'u' && TOLOWER_ASC(what[1]) == 'l')
	ul = TRUE;
    else if (!(TOLOWER_ASC(what[0]) == 'b' && TOLOWER_ASC(what[1]) == 'g'))
	return NULL;
    if (modec == 'g')
    {
# if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
#  ifdef FEAT_GUI
	// return font name
	if (font)
	    return HL_TABLE()[id - 1].sg_font_name;
#  endif

	// return #RRGGBB form (only possible when GUI is running)
	if ((USE_24BIT) && what[2] == '#')
	{
	    guicolor_T		color;
	    long_u		rgb;
	    static char_u	buf[10];

	    if (fg)
		color = HL_TABLE()[id - 1].sg_gui_fg;
	    else if (sp)
		color = HL_TABLE()[id - 1].sg_gui_sp;
	    else
		color = HL_TABLE()[id - 1].sg_gui_bg;
	    if (color == INVALCOLOR)
		return NULL;
	    rgb = (long_u)GUI_MCH_GET_RGB(color);
	    sprintf((char *)buf, "#%02x%02x%02x",
				      (unsigned)(rgb >> 16),
				      (unsigned)(rgb >> 8) & 255,
				      (unsigned)rgb & 255);
	    return buf;
	}
# endif
	if (fg)
	    return (HL_TABLE()[id - 1].sg_gui_fg_name);
	if (sp)
	    return (HL_TABLE()[id - 1].sg_gui_sp_name);
	return (HL_TABLE()[id - 1].sg_gui_bg_name);
    }
    if (sp)
	return NULL;
    if (modec == 'c')
    {
	if (fg)
	    n = HL_TABLE()[id - 1].sg_cterm_fg - 1;
	else if (ul)
	    n = HL_TABLE()[id - 1].sg_cterm_ul - 1;
	else if (font)
	    n = HL_TABLE()[id - 1].sg_cterm_font - 1;
	else
	    n = HL_TABLE()[id - 1].sg_cterm_bg - 1;
	if (n < 0)
	    return NULL;
	sprintf((char *)name, "%d", n);
	return name;
    }
    // term doesn't have color
    return NULL;
}
#endif

#if (defined(FEAT_SYN_HL) \
	    && (defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)) \
	&& defined(FEAT_PRINTER)) || defined(PROTO)
/*
 * Return color name of highlight group "id" as RGB value.
 */
    long_u
highlight_gui_color_rgb(
    int		id,
    int		fg)	// TRUE = fg, FALSE = bg
{
    guicolor_T	color;

    if (id <= 0 || id > highlight_ga.ga_len)
	return 0L;

    if (fg)
	color = HL_TABLE()[id - 1].sg_gui_fg;
    else
	color = HL_TABLE()[id - 1].sg_gui_bg;

    if (color == INVALCOLOR)
	return 0L;

    return GUI_MCH_GET_RGB(color);
}
#endif

/*
 * Output the syntax list header.
 * Return TRUE when started a new line.
 */
    int
syn_list_header(
    int	    did_header,		// did header already
    int	    outlen,		// length of string that comes
    int	    id)			// highlight group id
{
    int	    endcol = 19;
    int	    newline = TRUE;
    int	    name_col = 0;

    if (!did_header)
    {
	msg_putchar('\n');
	if (got_int)
	    return TRUE;
	msg_outtrans(HL_TABLE()[id - 1].sg_name);
	name_col = msg_col;
	endcol = 15;
    }
    else if (msg_col + outlen + 1 >= Columns)
    {
	msg_putchar('\n');
	if (got_int)
	    return TRUE;
    }
    else
    {
	if (msg_col >= endcol)	// wrap around is like starting a new line
	    newline = FALSE;
    }

    if (msg_col >= endcol)	// output at least one space
	endcol = msg_col + 1;
    if (Columns <= endcol)	// avoid hang for tiny window
	endcol = Columns - 1;

    msg_advance(endcol);

    // Show "xxx" with the attributes.
    if (!did_header)
    {
	if (endcol == Columns - 1 && endcol <= name_col)
	    msg_putchar(' ');
	msg_puts_attr("xxx", syn_id2attr(id));
	msg_putchar(' ');
    }

    return newline;
}

/*
 * Set the attribute numbers for a highlight group.
 * Called after one of the attributes has changed.
 */
    static void
set_hl_attr(
    int		idx)	    // index in array
{
    attrentry_T	    at_en;
    hl_group_T	    *sgp = HL_TABLE() + idx;

    // The "Normal" group doesn't need an attribute number
    if (sgp->sg_name_u != NULL && STRCMP(sgp->sg_name_u, "NORMAL") == 0)
	return;

#ifdef FEAT_GUI
    // For the GUI mode: If there are other than "normal" highlighting
    // attributes, need to allocate an attr number.
    if (sgp->sg_gui_fg == INVALCOLOR
	    && sgp->sg_gui_bg == INVALCOLOR
	    && sgp->sg_gui_sp == INVALCOLOR
	    && sgp->sg_font == NOFONT
# ifdef FEAT_XFONTSET
	    && sgp->sg_fontset == NOFONTSET
# endif
	    )
    {
	sgp->sg_gui_attr = sgp->sg_gui;
    }
    else
    {
	at_en.ae_attr = sgp->sg_gui;
	at_en.ae_u.gui.fg_color = sgp->sg_gui_fg;
	at_en.ae_u.gui.bg_color = sgp->sg_gui_bg;
	at_en.ae_u.gui.sp_color = sgp->sg_gui_sp;
	at_en.ae_u.gui.font = sgp->sg_font;
# ifdef FEAT_XFONTSET
	at_en.ae_u.gui.fontset = sgp->sg_fontset;
# endif
	sgp->sg_gui_attr = get_attr_entry(&gui_attr_table, &at_en);
    }
#endif
    // For the term mode: If there are other than "normal" highlighting
    // attributes, need to allocate an attr number.
    if (sgp->sg_start == NULL && sgp->sg_stop == NULL)
	sgp->sg_term_attr = sgp->sg_term;
    else
    {
	at_en.ae_attr = sgp->sg_term;
	at_en.ae_u.term.start = sgp->sg_start;
	at_en.ae_u.term.stop = sgp->sg_stop;
	sgp->sg_term_attr = get_attr_entry(&term_attr_table, &at_en);
    }

    // For the color term mode: If there are other than "normal"
    // highlighting attributes, need to allocate an attr number.
    if (sgp->sg_cterm_fg == 0 && sgp->sg_cterm_bg == 0 &&
	sgp->sg_cterm_ul == 0 && sgp->sg_cterm_font == 0
# ifdef FEAT_TERMGUICOLORS
	    && sgp->sg_gui_fg == INVALCOLOR
	    && sgp->sg_gui_bg == INVALCOLOR
	    && sgp->sg_gui_sp == INVALCOLOR
# endif
	    )
	sgp->sg_cterm_attr = sgp->sg_cterm;
    else
    {
	at_en.ae_attr = sgp->sg_cterm;
	at_en.ae_u.cterm.fg_color = sgp->sg_cterm_fg;
	at_en.ae_u.cterm.bg_color = sgp->sg_cterm_bg;
	at_en.ae_u.cterm.ul_color = sgp->sg_cterm_ul;
	at_en.ae_u.cterm.font = sgp->sg_cterm_font;
# ifdef FEAT_TERMGUICOLORS
	at_en.ae_u.cterm.fg_rgb = GUI_MCH_GET_RGB2(sgp->sg_gui_fg);
	at_en.ae_u.cterm.bg_rgb = GUI_MCH_GET_RGB2(sgp->sg_gui_bg);
	// Only use the underline/undercurl color when used, it may clear the
	// background color if not supported.
	if (sgp->sg_cterm & (HL_UNDERLINE | HL_UNDERCURL
			   | HL_UNDERDOUBLE | HL_UNDERDOTTED | HL_UNDERDASHED))
	    at_en.ae_u.cterm.ul_rgb = GUI_MCH_GET_RGB2(sgp->sg_gui_sp);
	else
	    at_en.ae_u.cterm.ul_rgb = INVALCOLOR;
	if (at_en.ae_u.cterm.fg_rgb == INVALCOLOR
		&& at_en.ae_u.cterm.bg_rgb == INVALCOLOR)
	{
	    // If both fg and bg are invalid fall back to the cterm colors.
	    // Helps when the GUI only uses an attribute, e.g. undercurl.
	    at_en.ae_u.cterm.fg_rgb = CTERMCOLOR;
	    at_en.ae_u.cterm.bg_rgb = CTERMCOLOR;
	}
# endif
	sgp->sg_cterm_attr = get_attr_entry(&cterm_attr_table, &at_en);
    }
}

/*
 * Lookup a highlight group name and return its ID.
 * If it is not found, 0 is returned.
 */
    int
syn_name2id(char_u *name)
{
    int		i;
    char_u	name_u[MAX_SYN_NAME + 1];

    // Avoid using stricmp() too much, it's slow on some systems
    // Avoid alloc()/free(), these are slow too.  ID names over 200 chars
    // don't deserve to be found!
    vim_strncpy(name_u, name, MAX_SYN_NAME);
    vim_strup(name_u);
    for (i = highlight_ga.ga_len; --i >= 0; )
	if (HL_TABLE()[i].sg_name_u != NULL
		&& STRCMP(name_u, HL_TABLE()[i].sg_name_u) == 0)
	    break;
    return i + 1;
}

/*
 * Lookup a highlight group name and return its attributes.
 * Return zero if not found.
 */
    int
syn_name2attr(char_u *name)
{
    int id = syn_name2id(name);

    if (id != 0)
	return syn_id2attr(id);
    return 0;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return TRUE if highlight group "name" exists.
 */
    int
highlight_exists(char_u *name)
{
    return (syn_name2id(name) > 0);
}

# if defined(FEAT_SEARCH_EXTRA) || defined(PROTO)
/*
 * Return the name of highlight group "id".
 * When not a valid ID return an empty string.
 */
    char_u *
syn_id2name(int id)
{
    if (id <= 0 || id > highlight_ga.ga_len)
	return (char_u *)"";
    return HL_TABLE()[id - 1].sg_name;
}
# endif
#endif

/*
 * Like syn_name2id(), but take a pointer + length argument.
 */
    int
syn_namen2id(char_u *linep, int len)
{
    char_u  *name;
    int	    id = 0;

    name = vim_strnsave(linep, len);
    if (name == NULL)
	return 0;

    id = syn_name2id(name);
    vim_free(name);
    return id;
}

/*
 * Find highlight group name in the table and return its ID.
 * The argument is a pointer to the name and the length of the name.
 * If it doesn't exist yet, a new entry is created.
 * Return 0 for failure.
 */
    int
syn_check_group(char_u *pp, int len)
{
    int	    id;
    char_u  *name;

    if (len > MAX_SYN_NAME)
    {
	emsg(_(e_highlight_group_name_too_long));
	return 0;
    }
    name = vim_strnsave(pp, len);
    if (name == NULL)
	return 0;

    id = syn_name2id(name);
    if (id == 0)			// doesn't exist yet
	id = syn_add_group(name);
    else
	vim_free(name);
    return id;
}

/*
 * Add new highlight group and return its ID.
 * "name" must be an allocated string, it will be consumed.
 * Return 0 for failure.
 */
    static int
syn_add_group(char_u *name)
{
    char_u	*p;
    char_u	*name_up;

    // Check that the name is valid (ASCII letters, digits, underscores, dots, or hyphens).
    for (p = name; *p != NUL; ++p)
    {
	if (!vim_isprintc(*p))
	{
	    emsg(_(e_unprintable_character_in_group_name));
	    vim_free(name);
	    return 0;
	}
	else if (!ASCII_ISALNUM(*p) && *p != '_' && *p != '.' && *p != '-')
	{
	    // This is an error, but since there previously was no check only
	    // give a warning.
	    msg_source(HL_ATTR(HLF_W));
	    msg(_("W18: Invalid character in group name"));
	    break;
	}
    }

    // First call for this growarray: init growing array.
    if (highlight_ga.ga_data == NULL)
    {
	highlight_ga.ga_itemsize = sizeof(hl_group_T);
	highlight_ga.ga_growsize = 10;
    }

    if (highlight_ga.ga_len >= MAX_HL_ID)
    {
	emsg(_(e_too_many_highlight_and_syntax_groups));
	vim_free(name);
	return 0;
    }

    // Make room for at least one other syntax_highlight entry.
    if (ga_grow(&highlight_ga, 1) == FAIL)
    {
	vim_free(name);
	return 0;
    }

    name_up = vim_strsave_up(name);
    if (name_up == NULL)
    {
	vim_free(name);
	return 0;
    }

    CLEAR_POINTER(&(HL_TABLE()[highlight_ga.ga_len]));
    HL_TABLE()[highlight_ga.ga_len].sg_name = name;
    HL_TABLE()[highlight_ga.ga_len].sg_name_u = name_up;
#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS)
    HL_TABLE()[highlight_ga.ga_len].sg_gui_bg = INVALCOLOR;
    HL_TABLE()[highlight_ga.ga_len].sg_gui_fg = INVALCOLOR;
    HL_TABLE()[highlight_ga.ga_len].sg_gui_sp = INVALCOLOR;
#endif
    ++highlight_ga.ga_len;

    return highlight_ga.ga_len;		    // ID is index plus one
}

/*
 * When, just after calling syn_add_group(), an error is discovered, this
 * function deletes the new name.
 */
    static void
syn_unadd_group(void)
{
    --highlight_ga.ga_len;
    vim_free(HL_TABLE()[highlight_ga.ga_len].sg_name);
    vim_free(HL_TABLE()[highlight_ga.ga_len].sg_name_u);
}

/*
 * Translate a group ID to highlight attributes.
 * "hl_id" must be valid: > 0, caller must check.
 */
    int
syn_id2attr(int hl_id)
{
    int		attr;
    hl_group_T	*sgp;

    hl_id = syn_get_final_id(hl_id);
    sgp = &HL_TABLE()[hl_id - 1];	    // index is ID minus one

#ifdef FEAT_GUI
    // Only use GUI attr when the GUI is being used.
    if (gui.in_use)
	attr = sgp->sg_gui_attr;
    else
#endif
	if (IS_CTERM)
	    attr = sgp->sg_cterm_attr;
	else
	    attr = sgp->sg_term_attr;

    return attr;
}

#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS) || defined(PROTO)
/*
 * Get the GUI colors and attributes for a group ID.
 * NOTE: the colors will be INVALCOLOR when not set, the color otherwise.
 */
    int
syn_id2colors(int hl_id, guicolor_T *fgp, guicolor_T *bgp)
{
    hl_group_T	*sgp;

    hl_id = syn_get_final_id(hl_id);
    sgp = &HL_TABLE()[hl_id - 1];	    // index is ID minus one

    *fgp = sgp->sg_gui_fg;
    *bgp = sgp->sg_gui_bg;
    return sgp->sg_gui;
}
#endif

#if (defined(MSWIN) \
	    && (!defined(FEAT_GUI_MSWIN) || defined(VIMDLL)) \
	    && defined(FEAT_TERMGUICOLORS)) \
	|| defined(FEAT_TERMINAL) || defined(PROTO)
    void
syn_id2cterm_bg(int hl_id, int *fgp, int *bgp)
{
    hl_group_T	*sgp;

    hl_id = syn_get_final_id(hl_id);
    sgp = &HL_TABLE()[hl_id - 1];	    // index is ID minus one
    *fgp = sgp->sg_cterm_fg - 1;
    *bgp = sgp->sg_cterm_bg - 1;
}
#endif

/*
 * Translate a group ID to the final group ID (following links).
 */
    int
syn_get_final_id(int hl_id)
{
    int		count;
    hl_group_T	*sgp;

    if (hl_id > highlight_ga.ga_len || hl_id < 1)
	return 0;			// Can be called from eval!!

    // Follow links until there is no more.
    // Look out for loops!  Break after 100 links.
    for (count = 100; --count >= 0; )
    {
	sgp = &HL_TABLE()[hl_id - 1];	    // index is ID minus one
	if (sgp->sg_link == 0 || sgp->sg_link > highlight_ga.ga_len)
	    break;
	hl_id = sgp->sg_link;
    }

    return hl_id;
}

#if defined(FEAT_GUI) || defined(FEAT_TERMGUICOLORS) || defined(PROTO)
/*
 * Call this function just after the GUI has started.
 * Also called when 'termguicolors' was set, gui.in_use will be FALSE then.
 * It finds the font and color handles for the highlighting groups.
 */
    void
highlight_gui_started(void)
{
    int	    idx;

    // First get the colors from the "Normal" and "Menu" group, if set
    if (USE_24BIT)
	set_normal_colors();

    for (idx = 0; idx < highlight_ga.ga_len; ++idx)
	gui_do_one_color(idx, FALSE, FALSE);

    highlight_changed();
}

    static void
gui_do_one_color(
    int		idx,
    int		do_menu UNUSED,	   // TRUE: might set the menu font
    int		do_tooltip UNUSED) // TRUE: might set the tooltip font
{
    int		didit = FALSE;

# ifdef FEAT_GUI
#  ifdef FEAT_TERMGUICOLORS
    if (gui.in_use)
#  endif
	if (HL_TABLE()[idx].sg_font_name != NULL)
	{
	    hl_do_font(idx, HL_TABLE()[idx].sg_font_name, FALSE, do_menu,
							    do_tooltip, TRUE);
	    didit = TRUE;
	}
# endif
    if (HL_TABLE()[idx].sg_gui_fg_name != NULL)
    {
	HL_TABLE()[idx].sg_gui_fg =
			    color_name2handle(HL_TABLE()[idx].sg_gui_fg_name);
	didit = TRUE;
    }
    if (HL_TABLE()[idx].sg_gui_bg_name != NULL)
    {
	HL_TABLE()[idx].sg_gui_bg =
			    color_name2handle(HL_TABLE()[idx].sg_gui_bg_name);
	didit = TRUE;
    }
    if (HL_TABLE()[idx].sg_gui_sp_name != NULL)
    {
	HL_TABLE()[idx].sg_gui_sp =
			    color_name2handle(HL_TABLE()[idx].sg_gui_sp_name);
	didit = TRUE;
    }
    if (didit)	// need to get a new attr number
	set_hl_attr(idx);
}
#endif

#if defined(USER_HIGHLIGHT) && defined(FEAT_STL_OPT)
/*
 * Apply difference between User[1-9] and HLF_S to HLF_SNC, HLF_ST or HLF_STNC.
 */
    static void
combine_stl_hlt(
	int id,
	int id_S,
	int id_alt,
	int hlcnt,
	int i,
	int hlf,
	int *table)
{
    hl_group_T *hlt = HL_TABLE();

    if (id_alt == 0)
    {
	CLEAR_POINTER(&hlt[hlcnt + i]);
	hlt[hlcnt + i].sg_term = highlight_attr[hlf];
	hlt[hlcnt + i].sg_cterm = highlight_attr[hlf];
#  if defined(FEAT_GUI) || defined(FEAT_EVAL)
	hlt[hlcnt + i].sg_gui = highlight_attr[hlf];
#  endif
    }
    else
	mch_memmove(&hlt[hlcnt + i],
		    &hlt[id_alt - 1],
		    sizeof(hl_group_T));
    hlt[hlcnt + i].sg_link = 0;

    hlt[hlcnt + i].sg_term ^=
	hlt[id - 1].sg_term ^ hlt[id_S - 1].sg_term;
    if (hlt[id - 1].sg_start != hlt[id_S - 1].sg_start)
	hlt[hlcnt + i].sg_start = hlt[id - 1].sg_start;
    if (hlt[id - 1].sg_stop != hlt[id_S - 1].sg_stop)
	hlt[hlcnt + i].sg_stop = hlt[id - 1].sg_stop;
    hlt[hlcnt + i].sg_cterm ^=
	hlt[id - 1].sg_cterm ^ hlt[id_S - 1].sg_cterm;
    if (hlt[id - 1].sg_cterm_fg != hlt[id_S - 1].sg_cterm_fg)
	hlt[hlcnt + i].sg_cterm_fg = hlt[id - 1].sg_cterm_fg;
    if (hlt[id - 1].sg_cterm_bg != hlt[id_S - 1].sg_cterm_bg)
	hlt[hlcnt + i].sg_cterm_bg = hlt[id - 1].sg_cterm_bg;
    if (hlt[id - 1].sg_cterm_font != hlt[id_S - 1].sg_cterm_font)
	hlt[hlcnt + i].sg_cterm_font = hlt[id - 1].sg_cterm_font;
#  if defined(FEAT_GUI) || defined(FEAT_EVAL)
    hlt[hlcnt + i].sg_gui ^=
	hlt[id - 1].sg_gui ^ hlt[id_S - 1].sg_gui;
#  endif
#  ifdef FEAT_GUI
    if (hlt[id - 1].sg_gui_fg != hlt[id_S - 1].sg_gui_fg)
	hlt[hlcnt + i].sg_gui_fg = hlt[id - 1].sg_gui_fg;
    if (hlt[id - 1].sg_gui_bg != hlt[id_S - 1].sg_gui_bg)
	hlt[hlcnt + i].sg_gui_bg = hlt[id - 1].sg_gui_bg;
    if (hlt[id - 1].sg_gui_sp != hlt[id_S - 1].sg_gui_sp)
	hlt[hlcnt + i].sg_gui_sp = hlt[id - 1].sg_gui_sp;
    if (hlt[id - 1].sg_font != hlt[id_S - 1].sg_font)
	hlt[hlcnt + i].sg_font = hlt[id - 1].sg_font;
#   ifdef FEAT_XFONTSET
    if (hlt[id - 1].sg_fontset != hlt[id_S - 1].sg_fontset)
	hlt[hlcnt + i].sg_fontset = hlt[id - 1].sg_fontset;
#   endif
#  endif
    highlight_ga.ga_len = hlcnt + i + 1;
    set_hl_attr(hlcnt + i);	// At long last we can apply
    table[i] = syn_id2attr(hlcnt + i + 1);
}
#endif

/*
 * Translate the 'highlight' option into attributes in highlight_attr[] and
 * set up the user highlights User1..9.  If FEAT_STL_OPT is in use, a set of
 * corresponding highlights to use on top of HLF_SNC is computed.
 * Called only when the 'highlight' option has been changed and upon first
 * screen redraw after any :highlight command.
 * Return FAIL when an invalid flag is found in 'highlight'.  OK otherwise.
 */
    int
highlight_changed(void)
{
    int		hlf;
    int		i;
    char_u	*p;
    int		attr;
    char_u	*end;
    int		id;
#ifdef USER_HIGHLIGHT
    char_u      userhl[30];  // use 30 to avoid compiler warning
# ifdef FEAT_STL_OPT
    int		id_S = -1;
    int		id_SNC = 0;
#  ifdef FEAT_TERMINAL
    int		id_ST = 0;
    int		id_STNC = 0;
#  endif
    int		hlcnt;
# endif
#endif
    static int	hl_flags[HLF_COUNT] = HL_FLAGS;

    need_highlight_changed = FALSE;

#ifdef FEAT_TERMINAL
    term_update_colors_all();
    term_update_wincolor_all();
#endif

    // Clear all attributes.
    for (hlf = 0; hlf < (int)HLF_COUNT; ++hlf)
	highlight_attr[hlf] = 0;

    // First set all attributes to their default value.
    // Then use the attributes from the 'highlight' option.
    for (i = 0; i < 2; ++i)
    {
	if (i)
	    p = p_hl;
	else
	    p = get_highlight_default();
	if (p == NULL)	    // just in case
	    continue;

	while (*p)
	{
	    for (hlf = 0; hlf < (int)HLF_COUNT; ++hlf)
		if (hl_flags[hlf] == *p)
		    break;
	    ++p;
	    if (hlf == (int)HLF_COUNT || *p == NUL)
		return FAIL;

	    // Allow several hl_flags to be combined, like "bu" for
	    // bold-underlined.
	    attr = 0;
	    for ( ; *p && *p != ','; ++p)	    // parse up to comma
	    {
		if (VIM_ISWHITE(*p))		    // ignore white space
		    continue;

		if (attr > HL_ALL)  // Combination with ':' is not allowed.
		    return FAIL;

		// Note: Keep this in sync with expand_set_highlight().
		switch (*p)
		{
		    case 'b':	attr |= HL_BOLD;
				break;
		    case 'i':	attr |= HL_ITALIC;
				break;
		    case '-':
		    case 'n':			    // no highlighting
				break;
		    case 'r':	attr |= HL_INVERSE;
				break;
		    case 's':	attr |= HL_STANDOUT;
				break;
		    case 'u':	attr |= HL_UNDERLINE;
				break;
		    case 'c':	attr |= HL_UNDERCURL;
				break;
		    case '2':	attr |= HL_UNDERDOUBLE;
				break;
		    case 'd':	attr |= HL_UNDERDOTTED;
				break;
		    case '=':	attr |= HL_UNDERDASHED;
				break;
		    case 't':	attr |= HL_STRIKETHROUGH;
				break;
		    case ':':	++p;		    // highlight group name
				if (attr || *p == NUL)	 // no combinations
				    return FAIL;
				end = vim_strchr(p, ',');
				if (end == NULL)
				    end = p + STRLEN(p);
				id = syn_check_group(p, (int)(end - p));
				if (id == 0)
				    return FAIL;
				attr = syn_id2attr(id);
				p = end - 1;
#if defined(FEAT_STL_OPT) && defined(USER_HIGHLIGHT)
				if (hlf == (int)HLF_SNC)
				    id_SNC = syn_get_final_id(id);
# ifdef FEAT_TERMINAL
				else if (hlf == (int)HLF_ST)
				    id_ST = syn_get_final_id(id);
				else if (hlf == (int)HLF_STNC)
				    id_STNC = syn_get_final_id(id);
# endif
				else if (hlf == (int)HLF_S)
				    id_S = syn_get_final_id(id);
#endif
				break;
		    default:	return FAIL;
		}
	    }
	    highlight_attr[hlf] = attr;

	    p = skip_to_option_part(p);	    // skip comma and spaces
	}
    }

#ifdef USER_HIGHLIGHT
    // Setup the user highlights
    //
    // Temporarily utilize 28 more hl entries:
    // 9 for User1-User9 combined with StatusLineNC
    // 9 for User1-User9 combined with StatusLineTerm
    // 9 for User1-User9 combined with StatusLineTermNC
    // 1 for StatusLine default
    // Have to be in there simultaneously in case of table overflows in
    // get_attr_entry()
# ifdef FEAT_STL_OPT
    if (ga_grow(&highlight_ga, 28) == FAIL)
	return FAIL;
    hlcnt = highlight_ga.ga_len;
    if (id_S == -1)
    {
	// Make sure id_S is always valid to simplify code below. Use the last
	// entry.
	CLEAR_POINTER(&HL_TABLE()[hlcnt + 27]);
	HL_TABLE()[hlcnt + 18].sg_term = highlight_attr[HLF_S];
	id_S = hlcnt + 19;
    }
# endif
    for (i = 0; i < 9; i++)
    {
	sprintf((char *)userhl, "User%d", i + 1);
	id = syn_name2id(userhl);
	if (id == 0)
	{
	    highlight_user[i] = 0;
# ifdef FEAT_STL_OPT
	    highlight_stlnc[i] = 0;
#  ifdef FEAT_TERMINAL
	    highlight_stlterm[i] = 0;
	    highlight_stltermnc[i] = 0;
#  endif
# endif
	}
	else
	{
	    highlight_user[i] = syn_id2attr(id);
# ifdef FEAT_STL_OPT
	    combine_stl_hlt(id, id_S, id_SNC, hlcnt, i,
						     HLF_SNC, highlight_stlnc);
#  ifdef FEAT_TERMINAL
	    combine_stl_hlt(id, id_S, id_ST, hlcnt + 9, i,
						    HLF_ST, highlight_stlterm);
	    combine_stl_hlt(id, id_S, id_STNC, hlcnt + 18, i,
						HLF_STNC, highlight_stltermnc);
#  endif
# endif
	}
    }
# ifdef FEAT_STL_OPT
    highlight_ga.ga_len = hlcnt;
# endif

#endif // USER_HIGHLIGHT

    return OK;
}

static void highlight_list(void);
static void highlight_list_two(int cnt, int attr);

/*
 * Handle command line completion for :highlight command.
 */
    void
set_context_in_highlight_cmd(expand_T *xp, char_u *arg)
{
    char_u	*p;

    // Default: expand group names
    xp->xp_context = EXPAND_HIGHLIGHT;
    xp->xp_pattern = arg;
    include_link = 2;
    include_default = 1;

    if (*arg == NUL)
	return;

    // (part of) subcommand already typed
    p = skiptowhite(arg);
    if (*p == NUL)
	return;

    // past "default" or group name
    include_default = 0;
    if (STRNCMP("default", arg, p - arg) == 0)
    {
	arg = skipwhite(p);
	xp->xp_pattern = arg;
	p = skiptowhite(arg);
    }
    if (*p == NUL)
	return;

    // past group name
    include_link = 0;
    if (arg[1] == 'i' && arg[0] == 'N')
	highlight_list();
    if (STRNCMP("link", arg, p - arg) == 0
	    || STRNCMP("clear", arg, p - arg) == 0)
    {
	xp->xp_pattern = skipwhite(p);
	p = skiptowhite(xp->xp_pattern);
	if (*p != NUL)		// past first group name
	{
	    xp->xp_pattern = skipwhite(p);
	    p = skiptowhite(xp->xp_pattern);
	}
    }
    if (*p != NUL)			// past group name(s)
	xp->xp_context = EXPAND_NOTHING;
}

/*
 * List highlighting matches in a nice way.
 */
    static void
highlight_list(void)
{
    int		i;

    for (i = 10; --i >= 0; )
	highlight_list_two(i, HL_ATTR(HLF_D));
    for (i = 40; --i >= 0; )
	highlight_list_two(99, 0);
}

    static void
highlight_list_two(int cnt, int attr)
{
    msg_puts_attr(&("N \bI \b!  \b"[cnt / 11]), attr);
    msg_clr_eos();
    out_flush();
    ui_delay(cnt == 99 ? 40L : (long)cnt * 50L, FALSE);
}

/*
 * Function given to ExpandGeneric() to obtain the list of group names.
 */
    char_u *
get_highlight_name(expand_T *xp UNUSED, int idx)
{
    return get_highlight_name_ext(xp, idx, TRUE);
}

/*
 * Obtain a highlight group name.
 * When "skip_cleared" is TRUE don't return a cleared entry.
 */
    char_u *
get_highlight_name_ext(expand_T *xp UNUSED, int idx, int skip_cleared)
{
    if (idx < 0)
	return NULL;

    // Items are never removed from the table, skip the ones that were
    // cleared.
    if (skip_cleared && idx < highlight_ga.ga_len && HL_TABLE()[idx].sg_cleared)
	return (char_u *)"";

    if (idx == highlight_ga.ga_len && include_none != 0)
	return (char_u *)"none";
    if (idx == highlight_ga.ga_len + include_none && include_default != 0)
	return (char_u *)"default";
    if (idx == highlight_ga.ga_len + include_none + include_default
							 && include_link != 0)
	return (char_u *)"link";
    if (idx == highlight_ga.ga_len + include_none + include_default + 1
							 && include_link != 0)
	return (char_u *)"clear";
    if (idx >= highlight_ga.ga_len)
	return NULL;
    return HL_TABLE()[idx].sg_name;
}

#if defined(FEAT_GUI) || defined(PROTO)
/*
 * Free all the highlight group fonts.
 * Used when quitting for systems which need it.
 */
    void
free_highlight_fonts(void)
{
    int	    idx;

    for (idx = 0; idx < highlight_ga.ga_len; ++idx)
    {
	gui_mch_free_font(HL_TABLE()[idx].sg_font);
	HL_TABLE()[idx].sg_font = NOFONT;
# ifdef FEAT_XFONTSET
	gui_mch_free_fontset(HL_TABLE()[idx].sg_fontset);
	HL_TABLE()[idx].sg_fontset = NOFONTSET;
# endif
    }

    gui_mch_free_font(gui.norm_font);
# ifdef FEAT_XFONTSET
    gui_mch_free_fontset(gui.fontset);
# endif
# ifndef FEAT_GUI_GTK
    gui_mch_free_font(gui.bold_font);
    gui_mch_free_font(gui.ital_font);
    gui_mch_free_font(gui.boldital_font);
# endif
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Convert each of the highlight attribute bits (bold, standout, underline,
 * etc.) set in 'hlattr' into a separate boolean item in a Dictionary with
 * the attribute name as the key.
 */
    static dict_T *
highlight_get_attr_dict(int hlattr)
{
    dict_T	*dict;
    int		i;

    dict = dict_alloc();
    if (dict == NULL)
	return NULL;

    for (i = 0; hl_attr_table[i] != 0; ++i)
    {
	if (hlattr & hl_attr_table[i])
	{
	    dict_add_bool(dict, hl_name_table[i], VVAL_TRUE);
	    hlattr &= ~hl_attr_table[i];	// don't want "inverse"
	}
    }

    return dict;
}

/*
 * Return the attributes of the highlight group at index 'hl_idx' as a
 * Dictionary. If 'resolve_link' is TRUE, then resolves the highlight group
 * links recursively.
 */
    static dict_T *
highlight_get_info(int hl_idx, int resolve_link)
{
    dict_T	*dict;
    hl_group_T	*sgp;
    dict_T	*attr_dict;
    int		hlgid;

    dict = dict_alloc();
    if (dict == NULL)
	return dict;

    sgp = &HL_TABLE()[hl_idx];
    // highlight group id is 1-based
    hlgid = hl_idx + 1;

    if (dict_add_string(dict, "name", sgp->sg_name) == FAIL)
	goto error;
    if (dict_add_number(dict, "id", hlgid) == FAIL)
	goto error;

    if (sgp->sg_link && resolve_link)
    {
	// resolve the highlight group link recursively
	while (sgp->sg_link)
	{
	    hlgid = sgp->sg_link;
	    sgp = &HL_TABLE()[sgp->sg_link - 1];
	}
    }

    if (sgp->sg_term != 0)
    {
	attr_dict = highlight_get_attr_dict(sgp->sg_term);
	if (attr_dict != NULL)
	    if (dict_add_dict(dict, "term", attr_dict) == FAIL)
		goto error;
    }
    if (sgp->sg_start != NULL)
	if (dict_add_string(dict, "start", sgp->sg_start) == FAIL)
	    goto error;
    if (sgp->sg_stop != NULL)
	if (dict_add_string(dict, "stop", sgp->sg_stop) == FAIL)
	    goto error;
    if (sgp->sg_cterm != 0)
    {
	attr_dict = highlight_get_attr_dict(sgp->sg_cterm);
	if (attr_dict != NULL)
	    if (dict_add_dict(dict, "cterm", attr_dict) == FAIL)
		goto error;
    }
    if (sgp->sg_cterm_fg != 0)
	if (dict_add_string(dict, "ctermfg",
		    highlight_color(hlgid, (char_u *)"fg", 'c')) == FAIL)
	    goto error;
    if (sgp->sg_cterm_bg != 0)
	if (dict_add_string(dict, "ctermbg",
			highlight_color(hlgid, (char_u *)"bg", 'c')) == FAIL)
	    goto error;
    if (sgp->sg_cterm_ul != 0)
	if (dict_add_string(dict, "ctermul",
			highlight_color(hlgid, (char_u *)"ul", 'c')) == FAIL)
	    goto error;
    if (sgp->sg_cterm_font != 0)
	if (dict_add_string(dict, "ctermfont",
			highlight_color(hlgid, (char_u *)"font", 'c')) == FAIL)
	    goto error;
    if (sgp->sg_gui != 0)
    {
	attr_dict = highlight_get_attr_dict(sgp->sg_gui);
	if (attr_dict != NULL)
	    if (dict_add_dict(dict, "gui", attr_dict) == FAIL)
		goto error;
    }
    if (sgp->sg_gui_fg_name != NULL)
	if (dict_add_string(dict, "guifg",
			highlight_color(hlgid, (char_u *)"fg", 'g')) == FAIL)
	    goto error;
    if (sgp->sg_gui_bg_name != NULL)
	if (dict_add_string(dict, "guibg",
			highlight_color(hlgid, (char_u *)"bg", 'g')) == FAIL)
	    goto error;
    if (sgp->sg_gui_sp_name != NULL)
	if (dict_add_string(dict, "guisp",
			highlight_color(hlgid, (char_u *)"sp", 'g')) == FAIL)
	    goto error;
# ifdef FEAT_GUI
    if (sgp->sg_font_name != NULL)
	if (dict_add_string(dict, "font", sgp->sg_font_name) == FAIL)
	    goto error;
# endif
    if (sgp->sg_link)
    {
	char_u	*link;

	link = HL_TABLE()[sgp->sg_link - 1].sg_name;
	if (link != NULL && dict_add_string(dict, "linksto", link) == FAIL)
	    goto error;

	if (sgp->sg_deflink)
	    dict_add_bool(dict, "default", VVAL_TRUE);
    }
    if (dict_len(dict) == 2)
	// If only 'name' is present, then the highlight group is cleared.
	dict_add_bool(dict, "cleared", VVAL_TRUE);

    return dict;

error:
    vim_free(dict);
    return NULL;
}

/*
 * "hlget([name])" function
 * Return the attributes of a specific highlight group (if specified) or all
 * the highlight groups.
 */
    void
f_hlget(typval_T *argvars, typval_T *rettv)
{
    list_T	*list;
    dict_T	*dict;
    int		i;
    char_u	*hlarg = NULL;
    int		resolve_link = FALSE;

    if (rettv_list_alloc(rettv) == FAIL)
	return;

    if (check_for_opt_string_arg(argvars, 0) == FAIL
	    || (argvars[0].v_type != VAR_UNKNOWN
		&& check_for_opt_bool_arg(argvars, 1) == FAIL))
	return;

    if (argvars[0].v_type != VAR_UNKNOWN)
    {
	// highlight group name supplied
	hlarg = tv_get_string_chk(&argvars[0]);
	if (hlarg == NULL)
	    return;

	if (argvars[1].v_type != VAR_UNKNOWN)
	{
	    int error = FALSE;

	    resolve_link = tv_get_bool_chk(&argvars[1], &error);
	    if (error)
		return;
	}
    }

    list = rettv->vval.v_list;
    for (i = 0; i < highlight_ga.ga_len && !got_int; ++i)
    {
	if (hlarg == NULL || STRICMP(hlarg, HL_TABLE()[i].sg_name) == 0)
	{
	    dict = highlight_get_info(i, resolve_link);
	    if (dict != NULL)
		list_append_dict(list, dict);
	}
    }
}

/*
 * Returns the string value at 'dict[key]'. Returns NULL, if 'key' is not in
 * 'dict' or the value is not a string type. If the value is not a string type
 * or is NULL, then 'error' is set to TRUE.
 */
    static char_u *
hldict_get_string(dict_T *dict, char_u *key, int *error)
{
    dictitem_T	*di;

    *error = FALSE;
    di = dict_find(dict, key, -1);
    if (di == NULL)
	return NULL;

    if (di->di_tv.v_type != VAR_STRING || di->di_tv.vval.v_string == NULL)
    {
	emsg(_(e_string_required));
	*error = TRUE;
	return NULL;
    }

    return di->di_tv.vval.v_string;
}

/*
 * Convert the highlight attribute Dictionary at 'dict[key]' into a string
 * value in 'attr_str' of length 'len'. Returns FALSE if 'dict[key]' is not a
 * Dictionary or is NULL.
 */
    static int
hldict_attr_to_str(
	dict_T	*dict,
	char_u	*key,
	char_u	*attr_str,
	size_t	len)
{
    dictitem_T	*di;
    dict_T	*attrdict;
    int		i;
    char_u	*p;
    size_t	sz;

    attr_str[0] = NUL;
    di = dict_find(dict, key, -1);
    if (di == NULL)
	return TRUE;

    if (di->di_tv.v_type != VAR_DICT || di->di_tv.vval.v_dict == NULL)
    {
	emsg(_(e_dictionary_required));
	return FALSE;
    }

    attrdict = di->di_tv.vval.v_dict;

    // If the attribute dict is empty, then return NONE to clear the attributes
    if (dict_len(attrdict) == 0)
    {
	vim_strcat(attr_str, (char_u *)"NONE", len);
	return TRUE;
    }

    p = attr_str;
    for (i = 0; i < (int)ARRAY_LENGTH(hl_name_table); i++)
    {
	if (dict_get_bool(attrdict, hl_name_table[i], VVAL_FALSE) == VVAL_TRUE)
	{
	    if (p != attr_str && (size_t)(p - attr_str + 2) < len)
		STRCPY(p, (char_u *)",");
	    sz = STRLEN(hl_name_table[i]);
	    if (p - attr_str + sz + 1 < len)
	    {
		STRCPY(p, (char_u *)hl_name_table[i]);
		p += sz;
	    }
	}
    }

    return TRUE;
}

// Temporary buffer used to store the command string produced by hlset().
// IObuff cannot be used for this as the error messages produced by hlset()
// internally use IObuff.
#define	HLSETBUFSZ  512
static char_u hlsetBuf[HLSETBUFSZ + 1];

/*
 * Add the highlight attribute "attr" of length "attrlen" and "value" at
 * "dptr", which points into "hlsetBuf".
 * Returns the updated pointer.
 */
    static char_u *
add_attr_and_value(char_u *dptr, char_u *attr, int attrlen, char_u *value)
{
    size_t	vallen;

    // Do nothing if the value is not specified or is empty
    if (value == NULL || *value == NUL)
	return dptr;

    vallen = STRLEN(value);
    if (dptr + attrlen + vallen + 1 < hlsetBuf + HLSETBUFSZ)
    {
	STRCPY(dptr, attr);
	dptr += attrlen;
	STRCPY(dptr, value);
	dptr += vallen;
    }

    return dptr;
}

/*
 * Add or update a highlight group using 'dict' items. Returns TRUE if
 * successfully updated the highlight group.
 */
    static int
hlg_add_or_update(dict_T *dict)
{
    char_u	*name;
    int		error;
    char_u	term_attr[MAX_ATTR_LEN];
    char_u	cterm_attr[MAX_ATTR_LEN];
    char_u	gui_attr[MAX_ATTR_LEN];
    char_u	*start;
    char_u	*stop;
    char_u	*ctermfg;
    char_u	*ctermbg;
    char_u	*ctermul;
    char_u	*ctermfont;
    char_u	*guifg;
    char_u	*guibg;
    char_u	*guisp;
# ifdef FEAT_GUI
    char_u	*font;
# endif
    int		forceit = FALSE;
    int		dodefault = FALSE;
    int		done = FALSE;
    char_u	*p;

    name = hldict_get_string(dict, (char_u *)"name", &error);
    if (name == NULL || *name == NUL || error)
	return FALSE;

    if (dict_get_bool(dict, "force", VVAL_FALSE) == VVAL_TRUE)
	forceit = TRUE;

    if (dict_get_bool(dict, "default", VVAL_FALSE) == VVAL_TRUE)
	dodefault = TRUE;

    if (dict_has_key(dict, "cleared"))
    {
	varnumber_T	cleared;

	// clear a highlight group
	cleared = dict_get_bool(dict, "cleared", FALSE);
	if (cleared == TRUE)
	{
	    vim_snprintf((char *)hlsetBuf, HLSETBUFSZ, "clear %s", name);
	    do_highlight(hlsetBuf, forceit, FALSE);
	    done = TRUE;
	}
    }

    if (dict_has_key(dict, "linksto"))
    {
	char_u	*linksto;

	// link highlight groups
	linksto = hldict_get_string(dict, (char_u *)"linksto", &error);
	if (linksto == NULL || *linksto == NUL || error)
	    return FALSE;

	vim_snprintf((char *)hlsetBuf, HLSETBUFSZ, "%slink %s %s",
				dodefault ? "default " : "", name, linksto);
	do_highlight(hlsetBuf, forceit, FALSE);

	done = TRUE;
    }

    // If 'cleared' or 'linksto' are specified, then don't process the other
    // attributes.
    if (done)
	return TRUE;

    start = hldict_get_string(dict, (char_u *)"start", &error);
    if (error)
	return FALSE;

    stop = hldict_get_string(dict, (char_u *)"stop", &error);
    if (error)
	return FALSE;

    if (!hldict_attr_to_str(dict, (char_u *)"term", term_attr,
							    sizeof(term_attr)))
	return FALSE;

    if (!hldict_attr_to_str(dict, (char_u *)"cterm", cterm_attr,
							   sizeof(cterm_attr)))
	return FALSE;

    ctermfg = hldict_get_string(dict, (char_u *)"ctermfg", &error);
    if (error)
	return FALSE;

    ctermbg = hldict_get_string(dict, (char_u *)"ctermbg", &error);
    if (error)
	return FALSE;

    ctermul = hldict_get_string(dict, (char_u *)"ctermul", &error);
    if (error)
	return FALSE;

    ctermfont = hldict_get_string(dict, (char_u *)"ctermfont", &error);
    if (error)
	return FALSE;

    if (!hldict_attr_to_str(dict, (char_u *)"gui", gui_attr, sizeof(gui_attr)))
	return FALSE;

    guifg = hldict_get_string(dict, (char_u *)"guifg", &error);
    if (error)
	return FALSE;

    guibg = hldict_get_string(dict, (char_u *)"guibg", &error);
    if (error)
	return FALSE;

    guisp = hldict_get_string(dict, (char_u *)"guisp", &error);
    if (error)
	return FALSE;

# ifdef FEAT_GUI
    font = hldict_get_string(dict, (char_u *)"font", &error);
    if (error)
	return FALSE;
# endif

    // If none of the attributes are specified, then do nothing.
    if (term_attr[0] == NUL && start == NULL && stop == NULL
	    && cterm_attr[0] == NUL && ctermfg == NULL && ctermbg == NULL
	    && ctermul == NULL && ctermfont == NULL && gui_attr[0] == NUL
# ifdef FEAT_GUI
	    && font == NULL
# endif
	    && guifg == NULL && guibg == NULL && guisp == NULL
	    )
	return TRUE;

    hlsetBuf[0] = NUL;
    p = hlsetBuf;
    if (dodefault)
	p = add_attr_and_value(p, (char_u *)"default", 7, (char_u *)" ");
    p = add_attr_and_value(p, (char_u *)"", 0, name);
    p = add_attr_and_value(p, (char_u *)" term=", 6, term_attr);
    p = add_attr_and_value(p, (char_u *)" start=", 7, start);
    p = add_attr_and_value(p, (char_u *)" stop=", 6, stop);
    p = add_attr_and_value(p, (char_u *)" cterm=", 7, cterm_attr);
    p = add_attr_and_value(p, (char_u *)" ctermfg=", 9, ctermfg);
    p = add_attr_and_value(p, (char_u *)" ctermbg=", 9, ctermbg);
    p = add_attr_and_value(p, (char_u *)" ctermul=", 9, ctermul);
    p = add_attr_and_value(p, (char_u *)" ctermfont=", 9, ctermfont);
    p = add_attr_and_value(p, (char_u *)" gui=", 5, gui_attr);
# ifdef FEAT_GUI
    p = add_attr_and_value(p, (char_u *)" font=", 6, font);
# endif
    p = add_attr_and_value(p, (char_u *)" guifg=", 7, guifg);
    p = add_attr_and_value(p, (char_u *)" guibg=", 7, guibg);
    (void)add_attr_and_value(p, (char_u *)" guisp=", 7, guisp);

    do_highlight(hlsetBuf, forceit, FALSE);

    return TRUE;
}

/*
 * "hlset([{highlight_attr}])" function
 * Add or modify highlight groups
 */
    void
f_hlset(typval_T *argvars, typval_T *rettv)
{
    listitem_T	*li;
    dict_T	*dict;

    rettv->vval.v_number = -1;

    if (check_for_list_arg(argvars, 0) == FAIL)
	return;

    FOR_ALL_LIST_ITEMS(argvars->vval.v_list, li)
    {
	if (li->li_tv.v_type != VAR_DICT)
	{
	    emsg(_(e_dictionary_required));
	    return;
	}

	dict = li->li_tv.vval.v_dict;
	if (!hlg_add_or_update(dict))
	    return;
    }

    rettv->vval.v_number = 0;
}
#endif
