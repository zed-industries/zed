/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * Porting to GTK+ was done by:
 *
 * (C) 1998,1999,2000 by Marcin Dalecki <martin@dalecki.de>
 *
 * With GREAT support and continuous encouragements by Andy Kahn and of
 * course Bram Moolenaar!
 *
 * Support for GTK+ 2 was added by:
 *
 * (C) 2002,2003  Jason Hildebrand  <jason@peaceworks.ca>
 *		  Daniel Elstner  <daniel.elstner@gmx.net>
 *
 * Support for GTK+ 3 was added by:
 *
 * 2016  Kazunobu Kuriyama  <kazunobu.kuriyama@gmail.com>
 */

#include "vim.h"
#ifdef USE_GRESOURCE
#include "auto/gui_gtk_gresources.h"
#endif

#ifdef FEAT_GUI_GNOME
// Gnome redefines _() and N_().  Grrr...
# ifdef _
#  undef _
# endif
# ifdef N_
#  undef N_
# endif
# ifdef textdomain
#  undef textdomain
# endif
# ifdef bindtextdomain
#  undef bindtextdomain
# endif
# ifdef bind_textdomain_codeset
#  undef bind_textdomain_codeset
# endif
# if defined(FEAT_GETTEXT) && !defined(ENABLE_NLS)
#  define ENABLE_NLS	// so the texts in the dialog boxes are translated
# endif
# include <gnome.h>
# include "version.h"
// missing prototype in bonobo-dock-item.h
extern void bonobo_dock_item_set_behavior(BonoboDockItem *dock_item, BonoboDockItemBehavior beh);
#endif

#if !defined(FEAT_GUI_GTK) && defined(PROTO)
// When generating prototypes we don't want syntax errors.
# define GdkAtom int
# define GdkEventExpose int
# define GdkEventFocus int
# define GdkEventVisibility int
# define GdkEventProperty int
# define GtkContainer int
# define GtkTargetEntry int
# define GtkType int
# define GtkWidget int
# define gint int
# define gpointer int
# define guint int
# define GdkEventKey int
# define GdkEventSelection int
# define GtkSelectionData int
# define GdkEventMotion int
# define GdkEventButton int
# define GdkDragContext int
# define GdkEventConfigure int
# define GdkEventClient int
#else
# if GTK_CHECK_VERSION(3,0,0)
#  include <gdk/gdkkeysyms-compat.h>
#  include <gtk/gtkx.h>
# else
#  include <gdk/gdkkeysyms.h>
# endif
# include <gdk/gdk.h>
# ifdef MSWIN
#  include <gdk/gdkwin32.h>
# else
#  include <gdk/gdkx.h>
# endif
# include <gtk/gtk.h>
# include "gui_gtk_f.h"
#endif

#ifdef HAVE_X11_SUNKEYSYM_H
# include <X11/Sunkeysym.h>
#endif

/*
 * Easy-to-use macro for multihead support.
 */
#define GET_X_ATOM(atom)	gdk_x11_atom_to_xatom_for_display( \
				    gtk_widget_get_display(gui.mainwin), atom)

// Selection type distinguishers
enum
{
    TARGET_TYPE_NONE,
    TARGET_UTF8_STRING,
    TARGET_STRING,
    TARGET_COMPOUND_TEXT,
    TARGET_HTML,
    TARGET_TEXT,
    TARGET_TEXT_URI_LIST,
    TARGET_TEXT_PLAIN,
    TARGET_TEXT_PLAIN_UTF8,
    TARGET_VIM,
    TARGET_VIMENC
};

/*
 * Table of selection targets supported by Vim.
 * Note: Order matters, preferred types should come first.
 */
static const GtkTargetEntry selection_targets[] =
{
    {VIMENC_ATOM_NAME,	0, TARGET_VIMENC},
    {VIM_ATOM_NAME,	0, TARGET_VIM},
    {"text/html",	0, TARGET_HTML},
    {"UTF8_STRING",	0, TARGET_UTF8_STRING},
    {"COMPOUND_TEXT",	0, TARGET_COMPOUND_TEXT},
    {"TEXT",		0, TARGET_TEXT},
    {"STRING",		0, TARGET_STRING},
    {"text/plain;charset=utf-8", 0, TARGET_TEXT_PLAIN_UTF8},
    {"text/plain",	0, TARGET_TEXT_PLAIN}
};
#define N_SELECTION_TARGETS ARRAY_LENGTH(selection_targets)

#ifdef FEAT_DND
/*
 * Table of DnD targets supported by Vim.
 * Note: Order matters, preferred types should come first.
 */
static const GtkTargetEntry dnd_targets[] =
{
    {"text/uri-list",	0, TARGET_TEXT_URI_LIST},
    {"text/html",	0, TARGET_HTML},
    {"UTF8_STRING",	0, TARGET_UTF8_STRING},
    {"STRING",		0, TARGET_STRING},
    {"text/plain",	0, TARGET_TEXT_PLAIN}
};
# define N_DND_TARGETS ARRAY_LENGTH(dnd_targets)
#endif


/*
 * "Monospace" is a standard font alias that should be present
 * on all proper Pango/fontconfig installations.
 */
# define DEFAULT_FONT	"Monospace 10"

#if defined(FEAT_GUI_GNOME) && defined(FEAT_SESSION)
# define USE_GNOME_SESSION
#endif

#if !defined(FEAT_GUI_GNOME)
/*
 * Atoms used to communicate save-yourself from the X11 session manager. There
 * is no need to move them into the GUI struct, since they should be constant.
 */
static GdkAtom wm_protocols_atom = GDK_NONE;
static GdkAtom save_yourself_atom = GDK_NONE;
#endif

/*
 * Atoms used to control/reference X11 selections.
 */
static GdkAtom html_atom = GDK_NONE;
static GdkAtom utf8_string_atom = GDK_NONE;
static GdkAtom vim_atom = GDK_NONE;	// Vim's own special selection format
static GdkAtom vimenc_atom = GDK_NONE;	// Vim's extended selection format

/*
 * Keycodes recognized by vim.
 * NOTE: when changing this, the table in gui_x11.c probably needs the same
 * change!
 */
static struct special_key
{
    guint key_sym;
    char_u code0;
    char_u code1;
}
const special_keys[] =
{
    {GDK_Up,		'k', 'u'},
    {GDK_Down,		'k', 'd'},
    {GDK_Left,		'k', 'l'},
    {GDK_Right,		'k', 'r'},
    {GDK_F1,		'k', '1'},
    {GDK_F2,		'k', '2'},
    {GDK_F3,		'k', '3'},
    {GDK_F4,		'k', '4'},
    {GDK_F5,		'k', '5'},
    {GDK_F6,		'k', '6'},
    {GDK_F7,		'k', '7'},
    {GDK_F8,		'k', '8'},
    {GDK_F9,		'k', '9'},
    {GDK_F10,		'k', ';'},
    {GDK_F11,		'F', '1'},
    {GDK_F12,		'F', '2'},
    {GDK_F13,		'F', '3'},
    {GDK_F14,		'F', '4'},
    {GDK_F15,		'F', '5'},
    {GDK_F16,		'F', '6'},
    {GDK_F17,		'F', '7'},
    {GDK_F18,		'F', '8'},
    {GDK_F19,		'F', '9'},
    {GDK_F20,		'F', 'A'},
    {GDK_F21,		'F', 'B'},
    {GDK_Pause,		'F', 'B'}, // Pause == F21 according to netbeans.txt
    {GDK_F22,		'F', 'C'},
    {GDK_F23,		'F', 'D'},
    {GDK_F24,		'F', 'E'},
    {GDK_F25,		'F', 'F'},
    {GDK_F26,		'F', 'G'},
    {GDK_F27,		'F', 'H'},
    {GDK_F28,		'F', 'I'},
    {GDK_F29,		'F', 'J'},
    {GDK_F30,		'F', 'K'},
    {GDK_F31,		'F', 'L'},
    {GDK_F32,		'F', 'M'},
    {GDK_F33,		'F', 'N'},
    {GDK_F34,		'F', 'O'},
    {GDK_F35,		'F', 'P'},
#ifdef SunXK_F36
    {SunXK_F36,		'F', 'Q'},
    {SunXK_F37,		'F', 'R'},
#endif
    {GDK_Help,		'%', '1'},
    {GDK_Undo,		'&', '8'},
    {GDK_BackSpace,	'k', 'b'},
    {GDK_Insert,	'k', 'I'},
    {GDK_Delete,	'k', 'D'},
    {GDK_3270_BackTab,	'k', 'B'},
    {GDK_Clear,		'k', 'C'},
    {GDK_Home,		'k', 'h'},
    {GDK_End,		'@', '7'},
    {GDK_Prior,		'k', 'P'},
    {GDK_Next,		'k', 'N'},
    {GDK_Print,		'%', '9'},
    // Keypad keys:
    {GDK_KP_Left,	'k', 'l'},
    {GDK_KP_Right,	'k', 'r'},
    {GDK_KP_Up,		'k', 'u'},
    {GDK_KP_Down,	'k', 'd'},
    {GDK_KP_Insert,	KS_EXTRA, (char_u)KE_KINS},
    {GDK_KP_Delete,	KS_EXTRA, (char_u)KE_KDEL},
    {GDK_KP_Home,	'K', '1'},
    {GDK_KP_End,	'K', '4'},
    {GDK_KP_Prior,	'K', '3'},  // page up
    {GDK_KP_Next,	'K', '5'},  // page down

    {GDK_KP_Add,	'K', '6'},
    {GDK_KP_Subtract,	'K', '7'},
    {GDK_KP_Divide,	'K', '8'},
    {GDK_KP_Multiply,	'K', '9'},
    {GDK_KP_Enter,	'K', 'A'},
    {GDK_KP_Decimal,	'K', 'B'},

    {GDK_KP_0,		'K', 'C'},
    {GDK_KP_1,		'K', 'D'},
    {GDK_KP_2,		'K', 'E'},
    {GDK_KP_3,		'K', 'F'},
    {GDK_KP_4,		'K', 'G'},
    {GDK_KP_5,		'K', 'H'},
    {GDK_KP_6,		'K', 'I'},
    {GDK_KP_7,		'K', 'J'},
    {GDK_KP_8,		'K', 'K'},
    {GDK_KP_9,		'K', 'L'},

    // End of list marker:
    {0, 0, 0}
};

/*
 * Flags for command line options table below.
 */
#define ARG_FONT	1
#define ARG_GEOMETRY	2
#define ARG_REVERSE	3
#define ARG_NOREVERSE	4
#define ARG_BACKGROUND	5
#define ARG_FOREGROUND	6
#define ARG_ICONIC	7
#define ARG_ROLE	8
#define ARG_NETBEANS	9
#define ARG_XRM		10	// ignored
#define ARG_MENUFONT	11	// ignored
#define ARG_INDEX_MASK	0x00ff
#define ARG_HAS_VALUE	0x0100	// a value is expected after the argument
#define ARG_NEEDS_GUI	0x0200	// need to initialize the GUI for this
#define ARG_FOR_GTK	0x0400	// argument is handled by GTK+ or GNOME
#define ARG_COMPAT_LONG	0x0800	// accept -foo but substitute with --foo
#define ARG_KEEP	0x1000	// don't remove argument from argv[]

/*
 * This table holds all the X GUI command line options allowed.  This includes
 * the standard ones so that we can skip them when Vim is started without the
 * GUI (but the GUI might start up later).
 *
 * When changing this, also update doc/gui_x11.txt and the usage message!!!
 */
typedef struct
{
    const char	    *name;
    unsigned int    flags;
}
cmdline_option_T;

static const cmdline_option_T cmdline_options[] =
{
    // We handle these options ourselves
    {"-fn",		ARG_FONT|ARG_HAS_VALUE},
    {"-font",		ARG_FONT|ARG_HAS_VALUE},
    {"-geom",		ARG_GEOMETRY|ARG_HAS_VALUE},
    {"-geometry",	ARG_GEOMETRY|ARG_HAS_VALUE},
    {"-rv",		ARG_REVERSE},
    {"-reverse",	ARG_REVERSE},
    {"+rv",		ARG_NOREVERSE},
    {"+reverse",	ARG_NOREVERSE},
    {"-bg",		ARG_BACKGROUND|ARG_HAS_VALUE},
    {"-background",	ARG_BACKGROUND|ARG_HAS_VALUE},
    {"-fg",		ARG_FOREGROUND|ARG_HAS_VALUE},
    {"-foreground",	ARG_FOREGROUND|ARG_HAS_VALUE},
    {"-iconic",		ARG_ICONIC},
    {"--role",		ARG_ROLE|ARG_HAS_VALUE},
#ifdef FEAT_NETBEANS_INTG
    {"-nb",		ARG_NETBEANS},	      // non-standard value format
    {"-xrm",		ARG_XRM|ARG_HAS_VALUE},		// not implemented
    {"-mf",		ARG_MENUFONT|ARG_HAS_VALUE},	// not implemented
    {"-menufont",	ARG_MENUFONT|ARG_HAS_VALUE},	// not implemented
#endif
    // Arguments handled by GTK (and GNOME) internally.
    {"--g-fatal-warnings",	ARG_FOR_GTK},
    {"--gdk-debug",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--gdk-no-debug",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--gtk-debug",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--gtk-no-debug",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--gtk-module",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--sync",			ARG_FOR_GTK},
    {"--display",		ARG_FOR_GTK|ARG_HAS_VALUE|ARG_COMPAT_LONG},
    {"--name",			ARG_FOR_GTK|ARG_HAS_VALUE|ARG_COMPAT_LONG},
    {"--class",			ARG_FOR_GTK|ARG_HAS_VALUE|ARG_COMPAT_LONG},
    {"--screen",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--gxid-host",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--gxid-port",		ARG_FOR_GTK|ARG_HAS_VALUE},
#ifdef FEAT_GUI_GNOME
    {"--load-modules",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--sm-client-id",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--sm-config-prefix",	ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--sm-disable",		ARG_FOR_GTK},
    {"--oaf-ior-fd",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--oaf-activate-iid",	ARG_FOR_GTK|ARG_HAS_VALUE},
    {"--oaf-private",		ARG_FOR_GTK},
    {"--enable-sound",		ARG_FOR_GTK},
    {"--disable-sound",		ARG_FOR_GTK},
    {"--espeaker",		ARG_FOR_GTK|ARG_HAS_VALUE},
    {"-?",			ARG_FOR_GTK|ARG_NEEDS_GUI},
    {"--help",			ARG_FOR_GTK|ARG_NEEDS_GUI|ARG_KEEP},
    {"--usage",			ARG_FOR_GTK|ARG_NEEDS_GUI},
# if 0 // conflicts with Vim's own --version argument
    {"--version",		ARG_FOR_GTK|ARG_NEEDS_GUI},
# endif
    {"--disable-crash-dialog",	ARG_FOR_GTK},
#endif
    {NULL, 0}
};

static int    gui_argc = 0;
static char **gui_argv = NULL;

static const char *role_argument = NULL;
#if defined(USE_GNOME_SESSION)
static const char *restart_command = NULL;
static       char *abs_restart_command = NULL;
#endif
static int found_iconic_arg = FALSE;

#ifdef FEAT_GUI_GNOME
/*
 * Can't use Gnome if --socketid given
 */
static int using_gnome = 0;
#else
# define using_gnome 0
#endif

// Comment out the following line to ignore code for resize history tracking.
#define TRACK_RESIZE_HISTORY
#ifdef TRACK_RESIZE_HISTORY
/*
 * Keep a short term resize history so that stale gtk responses can be
 * discarded.
 * When a gtk_window_resize() request is sent to gtk, the width/height of
 * the request is saved. Recent stale requests are kept around in a list.
 * See https://github.com/vim/vim/issues/10123
 */
# if 0  // Change to 1 to enable ch_log() calls for debugging.
#  ifdef FEAT_EVAL
#   define ENABLE_RESIZE_HISTORY_LOG
#  endif
# endif

/*
 * History item of a resize request.
 * Width and height are of gui.mainwin.
 */
typedef struct resize_history {
    int used;	    // If true, can't match for discard. Only matches once.
    int width;
    int height;
# ifdef ENABLE_RESIZE_HISTORY_LOG
    int seq;	    // for ch_log messages
# endif
    struct resize_history *next;
} resize_hist_T;

// never NULL during execution
static resize_hist_T *latest_resize_hist;
// list of stale resize requests
static resize_hist_T *old_resize_hists;

/*
 * Used when calling gtk_window_resize().
 * Create a resize request history item, put previous request on stale list.
 * Width/height are the size of the request for the gui.mainwin.
 */
    static void
alloc_resize_hist(int width, int height)
{
    // alloc a new resize hist, save current in list of old history
    resize_hist_T *prev_hist = latest_resize_hist;
    resize_hist_T *new_hist = ALLOC_CLEAR_ONE(resize_hist_T);

    new_hist->width = width;
    new_hist->height = height;
    latest_resize_hist = new_hist;

    // previous hist item becomes head of list
    prev_hist->next = old_resize_hists;
    old_resize_hists = prev_hist;

# ifdef ENABLE_RESIZE_HISTORY_LOG
    new_hist->seq = prev_hist->seq + 1;
    ch_log(NULL, "gui_gtk: New resize seq %d (%d, %d) [%d, %d]",
	   new_hist->seq, width, height, (int)Columns, (int)Rows);
# endif
}

/*
 * Free everything on the stale resize history list.
 * This list is empty when there are no outstanding resize requests.
 */
    static void
clear_resize_hists(void)
{
# ifdef ENABLE_RESIZE_HISTORY_LOG
    int		    i = 0;
# endif

    if (latest_resize_hist)
	latest_resize_hist->used = TRUE;
    while (old_resize_hists != NULL)
    {
	resize_hist_T *next_hist = old_resize_hists->next;

	vim_free(old_resize_hists);
	old_resize_hists = next_hist;
# ifdef ENABLE_RESIZE_HISTORY_LOG
	i++;
# endif
    }
# ifdef ENABLE_RESIZE_HISTORY_LOG
    ch_log(NULL, "gui_gtk: free %d hists", i);
# endif
}

// true if hist item is unused and matches w,h
# define MATCH_WIDTH_HEIGHT(hist, w, h) \
	  (!hist->used && hist->width == w && hist->height == h)

/*
 * Search the resize hist list.
 * Return true if the specified width,height match an item in the list that
 * has never matched before. Mark the matching item as used so it will
 * not match again.
 */
    static int
match_stale_width_height(int width, int height)
{
    resize_hist_T *hist = old_resize_hists;

    for (hist = old_resize_hists; hist != NULL; hist = hist->next)
	if (MATCH_WIDTH_HEIGHT(hist, width, height))
	{
# ifdef ENABLE_RESIZE_HISTORY_LOG
	    ch_log(NULL, "gui_gtk: discard seq %d, cur seq %d",
		   hist->seq, latest_resize_hist->seq);
# endif
	    hist->used = TRUE;
	    return TRUE;
	}
    return FALSE;
}

# if defined(EXITFREE)
    static void
free_all_resize_hist(void)
{
    clear_resize_hists();
    vim_free(latest_resize_hist);
}
# endif
#endif

/*
 * GTK doesn't set the GDK_BUTTON1_MASK state when dragging a touch. Add this
 * state when dragging.
 */
static guint dragging_button_state = 0;

/*
 * Parse the GUI related command-line arguments.  Any arguments used are
 * deleted from argv, and *argc is decremented accordingly.  This is called
 * when vim is started, whether or not the GUI has been started.
 */
    void
gui_mch_prepare(int *argc, char **argv)
{
    const cmdline_option_T  *option;
    int			    i	= 0;
    int			    len = 0;

#if defined(USE_GNOME_SESSION)
    /*
     * Determine the command used to invoke Vim, to be passed as restart
     * command to the session manager.	If argv[0] contains any directory
     * components try building an absolute path, otherwise leave it as is.
     */
    restart_command = argv[0];

    if (strchr(argv[0], G_DIR_SEPARATOR) != NULL)
    {
	char_u buf[MAXPATHL];

	if (mch_FullName((char_u *)argv[0], buf, (int)sizeof(buf), TRUE) == OK)
	{
	    abs_restart_command = (char *)vim_strsave(buf);
	    restart_command = abs_restart_command;
	}
    }
#endif

    /*
     * Move all the entries in argv which are relevant to GTK+ and GNOME
     * into gui_argv.  Freed later in gui_mch_init().
     */
    gui_argc = 0;
    gui_argv = ALLOC_MULT(char *, *argc + 1);

    g_return_if_fail(gui_argv != NULL);

    gui_argv[gui_argc++] = argv[i++];

    while (i < *argc)
    {
	// Don't waste CPU cycles on non-option arguments.
	if (argv[i][0] != '-' && argv[i][0] != '+')
	{
	    ++i;
	    continue;
	}

	// Look for argv[i] in cmdline_options[] table.
	for (option = &cmdline_options[0]; option->name != NULL; ++option)
	{
	    len = strlen(option->name);

	    if (strncmp(argv[i], option->name, len) == 0)
	    {
		if (argv[i][len] == '\0')
		    break;
		// allow --foo=bar style
		if (argv[i][len] == '=' && (option->flags & ARG_HAS_VALUE))
		    break;
#ifdef FEAT_NETBEANS_INTG
		// darn, -nb has non-standard syntax
		if (vim_strchr((char_u *)":=", argv[i][len]) != NULL
			&& (option->flags & ARG_INDEX_MASK) == ARG_NETBEANS)
		    break;
#endif
	    }
	    else if ((option->flags & ARG_COMPAT_LONG)
			&& strcmp(argv[i], option->name + 1) == 0)
	    {
		// Replace the standard X arguments "-name" and "-display"
		// with their GNU-style long option counterparts.
		argv[i] = (char *)option->name;
		break;
	    }
	}
	if (option->name == NULL) // no match
	{
	    ++i;
	    continue;
	}

	if (option->flags & ARG_FOR_GTK)
	{
	    // Move the argument into gui_argv, which
	    // will later be passed to gtk_init_check()
	    gui_argv[gui_argc++] = argv[i];
	}
	else
	{
	    char *value = NULL;

	    // Extract the option's value if there is one.
	    // Accept both "--foo bar" and "--foo=bar" style.
	    if (option->flags & ARG_HAS_VALUE)
	    {
		if (argv[i][len] == '=')
		    value = &argv[i][len + 1];
		else if (i + 1 < *argc && strcmp(argv[i + 1], "--") != 0)
		    value = argv[i + 1];
	    }

	    // Check for options handled by Vim itself
	    switch (option->flags & ARG_INDEX_MASK)
	    {
		case ARG_REVERSE:
		    found_reverse_arg = TRUE;
		    break;
		case ARG_NOREVERSE:
		    found_reverse_arg = FALSE;
		    break;
		case ARG_FONT:
		    font_argument = value;
		    break;
		case ARG_GEOMETRY:
		    if (value != NULL)
			gui.geom = vim_strsave((char_u *)value);
		    break;
		case ARG_BACKGROUND:
		    background_argument = value;
		    break;
		case ARG_FOREGROUND:
		    foreground_argument = value;
		    break;
		case ARG_ICONIC:
		    found_iconic_arg = TRUE;
		    break;
		case ARG_ROLE:
		    role_argument = value; // used later in gui_mch_open()
		    break;
#ifdef FEAT_NETBEANS_INTG
		case ARG_NETBEANS:
		    gui.dofork = FALSE; // don't fork() when starting GUI
		    netbeansArg = argv[i];
		    break;
#endif
		default:
		    break;
	    }
	}

	// These arguments make gnome_program_init() print a message and exit.
	// Must start the GUI for this, otherwise ":gui" will exit later!
	// Only when the GUI can start.
	if ((option->flags & ARG_NEEDS_GUI)
				      && gui_mch_early_init_check(FALSE) == OK)
	    gui.starting = TRUE;

	if (option->flags & ARG_KEEP)
	    ++i;
	else
	{
	    // Remove the flag from the argument vector.
	    if (--*argc > i)
	    {
		int n_strip = 1;

		// Move the argument's value as well, if there is one.
		if ((option->flags & ARG_HAS_VALUE)
			&& argv[i][len] != '='
			&& strcmp(argv[i + 1], "--") != 0)
		{
		    ++n_strip;
		    --*argc;
		    if (option->flags & ARG_FOR_GTK)
			gui_argv[gui_argc++] = argv[i + 1];
		}

		if (*argc > i)
		    mch_memmove(&argv[i], &argv[i + n_strip],
						(*argc - i) * sizeof(char *));
	    }
	    argv[*argc] = NULL;
	}
    }

    gui_argv[gui_argc] = NULL;
}

#if defined(EXITFREE) || defined(PROTO)
    void
gui_mch_free_all(void)
{
    vim_free(gui_argv);
#if defined(USE_GNOME_SESSION)
    vim_free(abs_restart_command);
#endif
#ifdef TRACK_RESIZE_HISTORY
    free_all_resize_hist();
#endif
}
#endif

#if !GTK_CHECK_VERSION(3,0,0)
/*
 * This should be maybe completely removed.
 * Doesn't seem possible, since check_copy_area() relies on
 * this information.  --danielk
 */
    static gint
visibility_event(GtkWidget *widget UNUSED,
		 GdkEventVisibility *event,
		 gpointer data UNUSED)
{
    gui.visibility = event->state;
    /*
     * When we do an gdk_window_copy_area(), and the window is partially
     * obscured, we want to receive an event to tell us whether it worked
     * or not.
     */
    if (gui.text_gc != NULL)
	gdk_gc_set_exposures(gui.text_gc,
			     gui.visibility != GDK_VISIBILITY_UNOBSCURED);
    return FALSE;
}
#endif // !GTK_CHECK_VERSION(3,0,0)

/*
 * Redraw the corresponding portions of the screen.
 */
#if GTK_CHECK_VERSION(3,0,0)
    static gboolean
draw_event(GtkWidget *widget UNUSED,
	   cairo_t   *cr,
	   gpointer   user_data UNUSED)
{
    // Skip this when the GUI isn't set up yet, will redraw later.
    if (gui.starting)
	return FALSE;

    out_flush();		// make sure all output has been processed
				// for GTK+ 3, may induce other draw events.

    cairo_set_source_surface(cr, gui.surface, 0, 0);

    {
	cairo_rectangle_list_t *list = NULL;

	list = cairo_copy_clip_rectangle_list(cr);
	if (list->status != CAIRO_STATUS_CLIP_NOT_REPRESENTABLE)
	{
	    int i;

	    for (i = 0; i < list->num_rectangles; i++)
	    {
		const cairo_rectangle_t *rect = &list->rectangles[i];
		cairo_rectangle(cr, rect->x, rect->y,
						    rect->width, rect->height);
		cairo_fill(cr);
	    }
	}
	cairo_rectangle_list_destroy(list);
    }

    return FALSE;
}

# if GTK_CHECK_VERSION(3,10,0)
    static gboolean
scale_factor_event(GtkWidget *widget,
	           GParamSpec* pspec UNUSED,
	           gpointer   user_data UNUSED)
{
    if (gui.surface != NULL)
	cairo_surface_destroy(gui.surface);

    int	    w, h;
    gtk_window_get_size(GTK_WINDOW(gui.mainwin), &w, &h);
    gui.surface = gdk_window_create_similar_surface(
	    gtk_widget_get_window(widget),
	    CAIRO_CONTENT_COLOR_ALPHA,
	    w, h);

    int	    usable_height = h;
    if (gtk_socket_id != 0)
	usable_height -= (gui.char_height - (gui.char_height/2)); // sic.

    gui_gtk_form_freeze(GTK_FORM(gui.formwin));
    gui.force_redraw = 1;
    gui_resize_shell(w, usable_height);
    gui_gtk_form_thaw(GTK_FORM(gui.formwin));

    return TRUE;
}
# endif // GTK_CHECK_VERSION(3,10,0)

#else // !GTK_CHECK_VERSION(3,0,0)
    static gint
expose_event(GtkWidget *widget UNUSED,
	     GdkEventExpose *event,
	     gpointer data UNUSED)
{
    // Skip this when the GUI isn't set up yet, will redraw later.
    if (gui.starting)
	return FALSE;

    out_flush();		// make sure all output has been processed
    gui_redraw(event->area.x, event->area.y,
	       event->area.width, event->area.height);

    // Clear the border areas if needed
    if (event->area.x < FILL_X(0))
	gdk_window_clear_area(gui.drawarea->window, 0, 0, FILL_X(0), 0);
    if (event->area.y < FILL_Y(0))
	gdk_window_clear_area(gui.drawarea->window, 0, 0, 0, FILL_Y(0));
    if (event->area.x > FILL_X(Columns))
	gdk_window_clear_area(gui.drawarea->window,
			      FILL_X((int)Columns), 0, 0, 0);
    if (event->area.y > FILL_Y(Rows))
	gdk_window_clear_area(gui.drawarea->window, 0, FILL_Y((int)Rows), 0, 0);

    return FALSE;
}
#endif // !GTK_CHECK_VERSION(3,0,0)

#ifdef FEAT_CLIENTSERVER
/*
 * Handle changes to the "Comm" property
 */
    static gint
property_event(GtkWidget *widget,
	       GdkEventProperty *event,
	       gpointer data UNUSED)
{
    if (event->type == GDK_PROPERTY_NOTIFY
	    && event->state == (int)GDK_PROPERTY_NEW_VALUE
	    && GDK_WINDOW_XID(event->window) == commWindow
	    && GET_X_ATOM(event->atom) == commProperty)
    {
	XEvent xev;

	// Translate to XLib
	xev.xproperty.type = PropertyNotify;
	xev.xproperty.atom = commProperty;
	xev.xproperty.window = commWindow;
	xev.xproperty.state = PropertyNewValue;
	serverEventProc(GDK_WINDOW_XDISPLAY(gtk_widget_get_window(widget)),
								      &xev, 0);
    }
    return FALSE;
}
#endif // defined(FEAT_CLIENTSERVER)

/*
 * Handle changes to the "Xft/DPI" setting
 */
    static void
gtk_settings_xft_dpi_changed_cb(GtkSettings *gtk_settings UNUSED,
				    GParamSpec *pspec UNUSED,
				    gpointer data UNUSED)
{
    // Create a new PangoContext for this screen, and initialize it
    // with the current font if necessary.
    if (gui.text_context != NULL)
	g_object_unref(gui.text_context);

    gui.text_context = gtk_widget_create_pango_context(gui.mainwin);
    pango_context_set_base_dir(gui.text_context, PANGO_DIRECTION_LTR);

    if (gui.norm_font != NULL)
    {
	// force default font
	gui_mch_init_font(*p_guifont == NUL ? NULL : p_guifont, FALSE);
	gui_set_shellsize(TRUE, FALSE, RESIZE_BOTH);
    }
}

typedef gboolean timeout_cb_type;

/*
 * Start a timer that will invoke the specified callback.
 * Returns the ID of the timer.
 */
    static guint
timeout_add(int time, timeout_cb_type (*callback)(gpointer), int *flagp)
{
    return g_timeout_add((guint)time, (GSourceFunc)callback, flagp);
}

    static void
timeout_remove(guint timer)
{
    g_source_remove(timer);
}


/////////////////////////////////////////////////////////////////////////////
// Focus handlers:


/*
 * This is a simple state machine:
 * BLINK_NONE	not blinking at all
 * BLINK_OFF	blinking, cursor is not shown
 * BLINK_ON	blinking, cursor is shown
 */

#define BLINK_NONE  0
#define BLINK_OFF   1
#define BLINK_ON    2

static int blink_state = BLINK_NONE;
static long_u blink_waittime = 700;
static long_u blink_ontime = 400;
static long_u blink_offtime = 250;
static guint blink_timer = 0;

    int
gui_mch_is_blinking(void)
{
    return blink_state != BLINK_NONE;
}

    int
gui_mch_is_blink_off(void)
{
    return blink_state == BLINK_OFF;
}

    void
gui_mch_set_blinking(long waittime, long on, long off)
{
    blink_waittime = waittime;
    blink_ontime = on;
    blink_offtime = off;
}

/*
 * Stop the cursor blinking.  Show the cursor if it wasn't shown.
 */
    void
gui_mch_stop_blink(int may_call_gui_update_cursor)
{
    if (blink_timer)
    {
	timeout_remove(blink_timer);
	blink_timer = 0;
    }
    if (blink_state == BLINK_OFF && may_call_gui_update_cursor)
    {
	gui_update_cursor(TRUE, FALSE);
#if !GTK_CHECK_VERSION(3,0,0)
	gui_mch_flush();
#endif
    }
    blink_state = BLINK_NONE;
}

    static timeout_cb_type
blink_cb(gpointer data UNUSED)
{
    if (blink_state == BLINK_ON)
    {
	gui_undraw_cursor();
	blink_state = BLINK_OFF;
	blink_timer = timeout_add(blink_offtime, blink_cb, NULL);
    }
    else
    {
	gui_update_cursor(TRUE, FALSE);
	blink_state = BLINK_ON;
	blink_timer = timeout_add(blink_ontime, blink_cb, NULL);
    }
#if !GTK_CHECK_VERSION(3,0,0)
    gui_mch_flush();
#endif

    return FALSE;		// don't happen again
}

/*
 * Start the cursor blinking.  If it was already blinking, this restarts the
 * waiting time and shows the cursor.
 */
    void
gui_mch_start_blink(void)
{
    if (blink_timer)
    {
	timeout_remove(blink_timer);
	blink_timer = 0;
    }
    // Only switch blinking on if none of the times is zero
    if (blink_waittime && blink_ontime && blink_offtime && gui.in_focus)
    {
	blink_timer = timeout_add(blink_waittime, blink_cb, NULL);
	blink_state = BLINK_ON;
	gui_update_cursor(TRUE, FALSE);
#if !GTK_CHECK_VERSION(3,0,0)
	gui_mch_flush();
#endif
    }
}

    static gint
enter_notify_event(GtkWidget *widget UNUSED,
		   GdkEventCrossing *event UNUSED,
		   gpointer data UNUSED)
{
    if (blink_state == BLINK_NONE)
	gui_mch_start_blink();

    // make sure keyboard input goes there
    if (gtk_socket_id == 0 || !gtk_widget_has_focus(gui.drawarea))
	gtk_widget_grab_focus(gui.drawarea);

    return FALSE;
}

    static gint
leave_notify_event(GtkWidget *widget UNUSED,
		   GdkEventCrossing *event UNUSED,
		   gpointer data UNUSED)
{
    if (blink_state != BLINK_NONE)
	gui_mch_stop_blink(TRUE);

    return FALSE;
}

    static gint
focus_in_event(GtkWidget *widget,
	       GdkEventFocus *event UNUSED,
	       gpointer data UNUSED)
{
    gui_focus_change(TRUE);

    if (blink_state == BLINK_NONE)
	gui_mch_start_blink();

    // make sure keyboard input goes to the draw area (if this is focus for a
    // window)
    if (widget != gui.drawarea)
	gtk_widget_grab_focus(gui.drawarea);

    return TRUE;
}

    static gint
focus_out_event(GtkWidget *widget UNUSED,
		GdkEventFocus *event UNUSED,
		gpointer data UNUSED)
{
    gui_focus_change(FALSE);

    if (blink_state != BLINK_NONE)
	gui_mch_stop_blink(TRUE);

    return TRUE;
}


/*
 * Translate a GDK key value to UTF-8 independently of the current locale.
 * The output is written to string, which must have room for at least 6 bytes
 * plus the NUL terminator.  Returns the length in bytes.
 *
 * event->string is evil; see here why:
 * http://developer.gnome.org/doc/API/2.0/gdk/gdk-Event-Structures.html#GdkEventKey
 */
    static int
keyval_to_string(unsigned int keyval, char_u *string)
{
    int	    len;
    guint32 uc;

    uc = gdk_keyval_to_unicode(keyval);
    if (uc != 0)
    {
	// Translate a normal key to UTF-8.  This doesn't work for dead
	// keys of course, you _have_ to use an input method for that.
	len = utf_char2bytes((int)uc, string);
    }
    else
    {
	// Translate keys which are represented by ASCII control codes in Vim.
	// There are only a few of those; most control keys are translated to
	// special terminal-like control sequences.
	len = 1;
	switch (keyval)
	{
	    case GDK_Tab: case GDK_KP_Tab: case GDK_ISO_Left_Tab:
		string[0] = TAB;
		break;
	    case GDK_Linefeed:
		string[0] = NL;
		break;
	    case GDK_Return: case GDK_ISO_Enter: case GDK_3270_Enter:
		string[0] = CAR;
		break;
	    case GDK_Escape:
		string[0] = ESC;
		break;
	    default:
		len = 0;
		break;
	}
    }
    string[len] = NUL;

    return len;
}

    static int
modifiers_gdk2vim(guint state)
{
    int modifiers = 0;

    if (state & GDK_SHIFT_MASK)
	modifiers |= MOD_MASK_SHIFT;
    if (state & GDK_CONTROL_MASK)
	modifiers |= MOD_MASK_CTRL;
    if (state & GDK_MOD1_MASK)
	modifiers |= MOD_MASK_ALT;
#if GTK_CHECK_VERSION(2,10,0)
    if (state & GDK_META_MASK)
	modifiers |= MOD_MASK_META;
    if (state & GDK_SUPER_MASK)
	modifiers |= MOD_MASK_CMD;
#else
    if (state & GDK_MOD4_MASK)
	modifiers |= MOD_MASK_CMD;
#endif

    return modifiers;
}

    static int
modifiers_gdk2mouse(guint state)
{
    int modifiers = 0;

    if (state & GDK_SHIFT_MASK)
	modifiers |= MOUSE_SHIFT;
    if (state & GDK_CONTROL_MASK)
	modifiers |= MOUSE_CTRL;
    if (state & GDK_MOD1_MASK)
	modifiers |= MOUSE_ALT;

    return modifiers;
}

/*
 * Main keyboard handler:
 */
    static gint
key_press_event(GtkWidget *widget UNUSED,
		GdkEventKey *event,
		gpointer data UNUSED)
{
    // For GTK+ 2 we know for sure how large the string might get.
    // (That is, up to 6 bytes + NUL + CSI escapes + safety measure.)
    char_u	string[32], string2[32];
    guint	key_sym;
    int		len;
    int		i;
    int		modifiers;
    int		key;
    guint	state;
    char_u	*s, *d;
    int		ctrl_prefix_added = 0;

    gui.event_time = event->time;
    key_sym = event->keyval;
    state = event->state;

#ifdef FEAT_XIM
    if (xim_queue_key_press_event(event, TRUE))
	return TRUE;
#endif

#ifdef SunXK_F36
    /*
     * These keys have bogus lookup strings, and trapping them here is
     * easier than trying to XRebindKeysym() on them with every possible
     * combination of modifiers.
     */
    if (key_sym == SunXK_F36 || key_sym == SunXK_F37)
	len = 0;
    else
#endif
    {
	len = keyval_to_string(key_sym, string2);

	// Careful: convert_input() doesn't handle the NUL character.
	// No need to convert pure ASCII anyway, thus the len > 1 check.
	if (len > 1 && input_conv.vc_type != CONV_NONE)
	    len = convert_input(string2, len, sizeof(string2));

	s = string2;
	d = string;
	for (i = 0; i < len; ++i)
	{
	    *d++ = s[i];
	    if (d[-1] == CSI && d + 2 < string + sizeof(string))
	    {
		// Turn CSI into K_CSI.
		*d++ = KS_EXTRA;
		*d++ = (int)KE_CSI;
	    }
	}
	len = d - string;
    }

    // Shift-Tab results in Left_Tab, but we want <S-Tab>
    if (key_sym == GDK_ISO_Left_Tab)
    {
	key_sym = GDK_Tab;
	state |= GDK_SHIFT_MASK;
    }

#ifdef FEAT_MENU
    // If there is a menu and 'wak' is "yes", or 'wak' is "menu" and the key
    // is a menu shortcut, we ignore everything with the ALT modifier.
    if ((state & GDK_MOD1_MASK)
	    && gui.menu_is_active
	    && (*p_wak == 'y'
		|| (*p_wak == 'm'
		    && len == 1
		    && gui_is_menu_shortcut(string[0]))))
	// For GTK2 we return false to signify that we haven't handled the
	// keypress, so that gtk will handle the mnemonic or accelerator.
	return FALSE;
#endif

    // We used to apply Alt/Meta to the key here (Mod1Mask), but that is now
    // done later, the same as it happens for the terminal.  Hopefully that
    // works for everybody...

    // Check for special keys.	Also do this when len == 1 (key has an ASCII
    // value) to detect backspace, delete and keypad keys.
    if (len == 0 || len == 1)
    {
	for (i = 0; special_keys[i].key_sym != 0; i++)
	{
	    if (special_keys[i].key_sym == key_sym)
	    {
		string[0] = CSI;
		string[1] = special_keys[i].code0;
		string[2] = special_keys[i].code1;
		len = -3;
		break;
	    }
	}
    }

#ifdef GDK_KEY_dead_circumflex
    // Belgian Ctrl+[ workaround
    if (len == 0 && key_sym == GDK_KEY_dead_circumflex)
    {
	string[0] = CSI;
	string[1] = KS_MODIFIER;
	string[2] = MOD_MASK_CTRL;
	string[3] = '[';
	len = 4;
	add_to_input_buf(string, len);
	// workaround has to return here, otherwise our fake string[] entries
	// are confusing code downstream
	return TRUE;
    }
#endif

    if (len == 0)   // Unrecognized key
	return TRUE;

    // For some keys a shift modifier is translated into another key code.
    if (len == -3)
	key = TO_SPECIAL(string[1], string[2]);
    else
    {
	string[len] = NUL;
	key = mb_ptr2char(string);
    }

    // Handle modifiers.
    modifiers = modifiers_gdk2vim(state);

    // Recognize special keys.
    key = simplify_key(key, &modifiers);
    if (key == CSI)
	key = K_CSI;
    if (IS_SPECIAL(key))
    {
	string[0] = CSI;
	string[1] = K_SECOND(key);
	string[2] = K_THIRD(key);
	len = 3;
    }
    else
    {
	// Some keys need adjustment when the Ctrl modifier is used.
	key = may_adjust_key_for_ctrl(modifiers, key);

	// May remove the Shift modifier if it's included in the key.
	modifiers = may_remove_shift_modifier(modifiers, key);

	len = mb_char2bytes(key, string);
    }

    if (modifiers != 0)
    {
	string2[0] = CSI;
	string2[1] = KS_MODIFIER;
	string2[2] = modifiers;
	add_to_input_buf(string2, 3);
	if (modifiers == 0x4)
	    ctrl_prefix_added = 1;
    }

    // Check if the key interrupts.
    {
	int int_ch = check_for_interrupt(key, modifiers);

	if (int_ch != NUL)
	{
	    trash_input_buf();
	    string[0] = int_ch;
	    len = 1;
	}
    }

    // workaround for German keyboard, where instead of '[' char we have code
    // sequence of bytes 195, 188 (UTF-8 for "u-umlaut")
    if (ctrl_prefix_added && len == 2
		    && ((int)string[0]) == 195
		    && ((int)string[1]) == 188)
    {
	string[0] = 91; // ASCII('[')
	len = 1;
    }
    add_to_input_buf(string, len);

    // blank out the pointer if necessary
    if (p_mh)
	gui_mch_mousehide(TRUE);

    return TRUE;
}

#if defined(FEAT_XIM) || GTK_CHECK_VERSION(3,0,0)
    static gboolean
key_release_event(GtkWidget *widget UNUSED,
		  GdkEventKey *event UNUSED,
		  gpointer data UNUSED)
{
# if defined(FEAT_XIM)
    gui.event_time = event->time;
    /*
     * GTK+ 2 input methods may do fancy stuff on key release events too.
     * With the default IM for instance, you can enter any UCS code point
     * by holding down CTRL-SHIFT and typing hexadecimal digits.
     */
    return xim_queue_key_press_event(event, FALSE);
# else
    return TRUE;
# endif
}
#endif


/////////////////////////////////////////////////////////////////////////////
// Selection handlers:

// Remember when clip_lose_selection was called from here, we must not call
// gtk_selection_owner_set() then.
static int in_selection_clear_event = FALSE;

    static gint
selection_clear_event(GtkWidget		*widget UNUSED,
		      GdkEventSelection	*event,
		      gpointer		user_data UNUSED)
{
    in_selection_clear_event = TRUE;
    if (event->selection == clip_plus.gtk_sel_atom)
	clip_lose_selection(&clip_plus);
    else
	clip_lose_selection(&clip_star);
    in_selection_clear_event = FALSE;

    return TRUE;
}

#define RS_NONE	0	// selection_received_cb() not called yet
#define RS_OK	1	// selection_received_cb() called and OK
#define RS_FAIL	2	// selection_received_cb() called and failed
static int received_selection = RS_NONE;

    static void
selection_received_cb(GtkWidget		*widget UNUSED,
		      GtkSelectionData	*data,
		      guint		time_ UNUSED,
		      gpointer		user_data UNUSED)
{
    Clipboard_T	    *cbd;
    char_u	    *text;
    char_u	    *tmpbuf = NULL;
    guchar	    *tmpbuf_utf8 = NULL;
    int		    len;
    int		    motion_type = MAUTO;

    if (gtk_selection_data_get_selection(data) == clip_plus.gtk_sel_atom)
	cbd = &clip_plus;
    else
	cbd = &clip_star;

    text = (char_u *)gtk_selection_data_get_data(data);
    len = gtk_selection_data_get_length(data);

    if (text == NULL || len <= 0)
    {
	received_selection = RS_FAIL;
	// clip_free_selection(cbd); ???

	return;
    }

    if (gtk_selection_data_get_data_type(data) == vim_atom)
    {
	motion_type = *text++;
	--len;
    }
    else if (gtk_selection_data_get_data_type(data) == vimenc_atom)
    {
	char_u		*enc;
	vimconv_T	conv;

	motion_type = *text++;
	--len;

	enc = text;
	text += STRLEN(text) + 1;
	len -= text - enc;

	// If the encoding of the text is different from 'encoding', attempt
	// converting it.
	conv.vc_type = CONV_NONE;
	convert_setup(&conv, enc, p_enc);
	if (conv.vc_type != CONV_NONE)
	{
	    tmpbuf = string_convert(&conv, text, &len);
	    if (tmpbuf != NULL)
		text = tmpbuf;
	    convert_setup(&conv, NULL, NULL);
	}
    }

    // gtk_selection_data_get_text() handles all the nasty details
    // and targets and encodings etc.  This rocks so hard.
    else
    {
	tmpbuf_utf8 = gtk_selection_data_get_text(data);
	if (tmpbuf_utf8 != NULL)
	{
	    len = STRLEN(tmpbuf_utf8);
	    if (input_conv.vc_type != CONV_NONE)
	    {
		tmpbuf = string_convert(&input_conv, tmpbuf_utf8, &len);
		if (tmpbuf != NULL)
		    text = tmpbuf;
	    }
	    else
		text = tmpbuf_utf8;
	}
	else if (len >= 2 && text[0] == 0xff && text[1] == 0xfe)
	{
	    vimconv_T conv;

	    // UTF-16, we get this for HTML
	    conv.vc_type = CONV_NONE;
	    convert_setup_ext(&conv, (char_u *)"utf-16le", FALSE, p_enc, TRUE);

	    if (conv.vc_type != CONV_NONE)
	    {
		text += 2;
		len -= 2;
		tmpbuf = string_convert(&conv, text, &len);
		convert_setup(&conv, NULL, NULL);
	    }
	    if (tmpbuf != NULL)
		text = tmpbuf;
	}
    }

    // Chop off any trailing NUL bytes.  OpenOffice sends these.
    while (len > 0 && text[len - 1] == NUL)
	--len;

    clip_yank_selection(motion_type, text, (long)len, cbd);
    received_selection = RS_OK;
    vim_free(tmpbuf);
    g_free(tmpbuf_utf8);
}

/*
 * Prepare our selection data for passing it to the external selection
 * client.
 */
    static void
selection_get_cb(GtkWidget	    *widget UNUSED,
		 GtkSelectionData   *selection_data,
		 guint		    info,
		 guint		    time_ UNUSED,
		 gpointer	    user_data UNUSED)
{
    char_u	    *string;
    char_u	    *tmpbuf;
    long_u	    tmplen;
    int		    length;
    int		    motion_type;
    GdkAtom	    type;
    Clipboard_T    *cbd;

    if (gtk_selection_data_get_selection(selection_data)
	    == clip_plus.gtk_sel_atom)
	cbd = &clip_plus;
    else
	cbd = &clip_star;

    if (!cbd->owned)
	return;			// Shouldn't ever happen

    if (info != (guint)TARGET_STRING
	    && (!clip_html || info != (guint)TARGET_HTML)
	    && info != (guint)TARGET_UTF8_STRING
	    && info != (guint)TARGET_VIMENC
	    && info != (guint)TARGET_VIM
	    && info != (guint)TARGET_COMPOUND_TEXT
	    && info != (guint)TARGET_TEXT_PLAIN
	    && info != (guint)TARGET_TEXT_PLAIN_UTF8
	    && info != (guint)TARGET_TEXT)
	return;

    // get the selection from the '*'/'+' register
    clip_get_selection(cbd);

    motion_type = clip_convert_selection(&string, &tmplen, cbd);
    if (motion_type < 0 || string == NULL)
	return;
    // Due to int arguments we can't handle more than G_MAXINT.  Also
    // reserve one extra byte for NUL or the motion type; just in case.
    // (Not that pasting 2G of text is ever going to work, but... ;-)
    length = MIN(tmplen, (long_u)(G_MAXINT - 1));

    if (info == (guint)TARGET_VIM)
    {
	tmpbuf = alloc(length + 1);
	if (tmpbuf != NULL)
	{
	    tmpbuf[0] = motion_type;
	    mch_memmove(tmpbuf + 1, string, (size_t)length);
	}
	// For our own format, the first byte contains the motion type
	++length;
	vim_free(string);
	string = tmpbuf;
	type = vim_atom;
    }

    else if (info == (guint)TARGET_HTML)
    {
	vimconv_T conv;

	// Since we get utf-16, we probably should set it as well.
	conv.vc_type = CONV_NONE;
	convert_setup_ext(&conv, p_enc, TRUE, (char_u *)"utf-16le", FALSE);
	if (conv.vc_type != CONV_NONE)
	{
	    tmpbuf = string_convert(&conv, string, &length);
	    convert_setup(&conv, NULL, NULL);
	    vim_free(string);
	    string = tmpbuf;
	}

	// Prepend the BOM: "fffe"
	if (string != NULL)
	{
	    tmpbuf = alloc(length + 2);
	    if (tmpbuf != NULL)
	    {
		tmpbuf[0] = 0xff;
		tmpbuf[1] = 0xfe;
		mch_memmove(tmpbuf + 2, string, (size_t)length);
		vim_free(string);
		string = tmpbuf;
		length += 2;
	    }

#if !GTK_CHECK_VERSION(3,0,0)
	    // Looks redundant even for GTK2 because these values are
	    // overwritten by gtk_selection_data_set() that follows.
	    selection_data->type = selection_data->target;
	    selection_data->format = 16;	// 16 bits per char
#endif
	    gtk_selection_data_set(selection_data, html_atom, 16,
							      string, length);
	    vim_free(string);
	}
	return;
    }
    else if (info == (guint)TARGET_VIMENC)
    {
	int l = STRLEN(p_enc);

	// contents: motion_type 'encoding' NUL text
	tmpbuf = alloc(length + l + 2);
	if (tmpbuf != NULL)
	{
	    tmpbuf[0] = motion_type;
	    STRCPY(tmpbuf + 1, p_enc);
	    mch_memmove(tmpbuf + l + 2, string, (size_t)length);
	    length += l + 2;
	    vim_free(string);
	    string = tmpbuf;
	}
	type = vimenc_atom;
    }

    // gtk_selection_data_set_text() handles everything for us.  This is
    // so easy and simple and cool, it'd be insane not to use it.
    else
    {
	if (output_conv.vc_type != CONV_NONE)
	{
	    tmpbuf = string_convert(&output_conv, string, &length);
	    vim_free(string);
	    if (tmpbuf == NULL)
		return;
	    string = tmpbuf;
	}
	// Validate the string to avoid runtime warnings
	if (g_utf8_validate((const char *)string, (gssize)length, NULL))
	{
	    gtk_selection_data_set_text(selection_data,
					(const char *)string, length);
	}
	vim_free(string);
	return;
    }

    if (string != NULL)
    {
#if !GTK_CHECK_VERSION(3,0,0)
	// Looks redundant even for GTK2 because these values are
	// overwritten by gtk_selection_data_set() that follows.
	selection_data->type = selection_data->target;
	selection_data->format = 8;	// 8 bits per char
#endif
	gtk_selection_data_set(selection_data, type, 8, string, length);
	vim_free(string);
    }
}

/*
 * Check if the GUI can be started.  Called before gvimrc is sourced and
 * before fork().
 * Return OK or FAIL.
 */
    int
gui_mch_early_init_check(int give_message)
{
    char_u *p, *q;

    // Guess that when $DISPLAY isn't set the GUI can't start.
    p = mch_getenv((char_u *)"DISPLAY");
    q = mch_getenv((char_u *)"WAYLAND_DISPLAY");
    if ((p == NULL || *p == NUL) && (q == NULL || *q == NUL))
    {
	gui.dying = TRUE;
	if (give_message)
	    emsg(_((char *)e_cannot_open_display));
	return FAIL;
    }
    return OK;
}

/*
 * Check if the GUI can be started.  Called before gvimrc is sourced but after
 * fork().
 * Return OK or FAIL.
 */
    int
gui_mch_init_check(void)
{
#ifdef USE_GRESOURCE
    static int res_registered = FALSE;

    if (!res_registered)
    {
	// Call this function in the GUI process; otherwise, the resources
	// won't be available.  Don't call it twice.
	res_registered = TRUE;
	gui_gtk_register_resource();
    }
#endif

#if GTK_CHECK_VERSION(3,10,0)
    // Vim currently assumes that Gtk means X11, so it cannot use native Gtk
    // support for other backends such as Wayland.
    //
    // Use an environment variable to enable unfinished Wayland support.
    if (getenv("GVIM_ENABLE_WAYLAND") == NULL)
	gdk_set_allowed_backends ("x11");
#endif

#ifdef FEAT_GUI_GNOME
    if (gtk_socket_id == 0)
	using_gnome = 1;
#endif

    // This defaults to argv[0], but we want it to match the name of the
    // shipped gvim.desktop so that Vim's windows can be associated with this
    // file.
    g_set_prgname("gvim");

    // Don't use gtk_init() or gnome_init(), it exits on failure.
    if (!gtk_init_check(&gui_argc, &gui_argv))
    {
	gui.dying = TRUE;
	emsg(_((char *)e_cannot_open_display));
	return FAIL;
    }

    return OK;
}

/////////////////////////////////////////////////////////////////////////////
// Mouse handling callbacks


static guint mouse_click_timer = 0;
static int mouse_timed_out = TRUE;

/*
 * Timer used to recognize multiple clicks of the mouse button
 */
    static timeout_cb_type
mouse_click_timer_cb(gpointer data)
{
    // we don't use this information currently
    int *timed_out = (int *) data;

    *timed_out = TRUE;
    return FALSE;		// don't happen again
}

static guint		motion_repeat_timer  = 0;
static int		motion_repeat_offset = FALSE;
static timeout_cb_type	motion_repeat_timer_cb(gpointer);

    static void
process_motion_notify(int x, int y, GdkModifierType state)
{
    int	    button;
    int_u   vim_modifiers;
    GtkAllocation allocation;

    // Need to add GDK_BUTTON1_MASK state when dragging a touch.
    state |= dragging_button_state;

    button = (state & (GDK_BUTTON1_MASK | GDK_BUTTON2_MASK |
		       GDK_BUTTON3_MASK | GDK_BUTTON4_MASK |
		       GDK_BUTTON5_MASK))
	      ? MOUSE_DRAG : ' ';

    // If our pointer is currently hidden, then we should show it.
    gui_mch_mousehide(FALSE);

    // Just moving the rodent above the drawing area without any button
    // being pressed.
    if (button != MOUSE_DRAG)
    {
	gui_mouse_moved(x, y);
	return;
    }

    // translate modifier coding between the main engine and GTK
    vim_modifiers = modifiers_gdk2mouse(state);

    // inform the editor engine about the occurrence of this event
    gui_send_mouse_event(button, x, y, FALSE, vim_modifiers);

    /*
     * Auto repeat timer handling.
     */
    gtk_widget_get_allocation(gui.drawarea, &allocation);

    if (x < 0 || y < 0
	    || x >= allocation.width
	    || y >= allocation.height)
    {

	int dx;
	int dy;
	int offshoot;
	int delay = 10;

	// Calculate the maximal distance of the cursor from the drawing area.
	// (offshoot can't become negative here!).
	dx = x < 0 ? -x : x - allocation.width;
	dy = y < 0 ? -y : y - allocation.height;

	offshoot = dx > dy ? dx : dy;

	// Make a linearly decaying timer delay with a threshold of 5 at a
	// distance of 127 pixels from the main window.
	//
	// One could think endlessly about the most ergonomic variant here.
	// For example it could make sense to calculate the distance from the
	// drags start instead...
	//
	// Maybe a parabolic interpolation would suite us better here too...
	if (offshoot > 127)
	{
	    // 5 appears to be somehow near to my perceptual limits :-).
	    delay = 5;
	}
	else
	{
	    delay = (130 * (127 - offshoot)) / 127 + 5;
	}

	// shoot again
	if (!motion_repeat_timer)
	    motion_repeat_timer = timeout_add(delay, motion_repeat_timer_cb,
									 NULL);
    }
}

#if GTK_CHECK_VERSION(3,0,0)
    static GdkDevice *
gui_gtk_get_pointer_device(GtkWidget *widget)
{
    GdkWindow * const win = gtk_widget_get_window(widget);
    GdkDisplay * const dpy = gdk_window_get_display(win);
# if GTK_CHECK_VERSION(3,20,0)
    GdkSeat * const seat = gdk_display_get_default_seat(dpy);
    return gdk_seat_get_pointer(seat);
# else
    GdkDeviceManager * const mngr = gdk_display_get_device_manager(dpy);
    return gdk_device_manager_get_client_pointer(mngr);
# endif
}

    static GdkWindow *
gui_gtk_get_pointer(GtkWidget       *widget,
		    gint	    *x,
		    gint	    *y,
		    GdkModifierType *state)
{
    GdkWindow * const win = gtk_widget_get_window(widget);
    GdkDevice * const dev = gui_gtk_get_pointer_device(widget);
    return gdk_window_get_device_position(win, dev , x, y, state);
}

# if defined(FEAT_GUI_TABLINE) || defined(PROTO)
    static GdkWindow *
gui_gtk_window_at_position(GtkWidget *widget,
			   gint      *x,
			   gint      *y)
{
    GdkDevice * const dev = gui_gtk_get_pointer_device(widget);
    return gdk_device_get_window_at_position(dev, x, y);
}
# endif
#else // !GTK_CHECK_VERSION(3,0,0)
# define gui_gtk_get_pointer(wid, x, y, s) \
    gdk_window_get_pointer((wid)->window, x, y, s)
# define gui_gtk_window_at_position(wid, x, y)	gdk_window_at_pointer(x, y)
#endif

/*
 * Timer used to recognize multiple clicks of the mouse button.
 */
    static timeout_cb_type
motion_repeat_timer_cb(gpointer data UNUSED)
{
    int		    x;
    int		    y;
    GdkModifierType state;

    gui_gtk_get_pointer(gui.drawarea, &x, &y, &state);

    if (!(state & (GDK_BUTTON1_MASK | GDK_BUTTON2_MASK |
		   GDK_BUTTON3_MASK | GDK_BUTTON4_MASK |
		   GDK_BUTTON5_MASK)))
    {
	motion_repeat_timer = 0;
	return FALSE;
    }

    // If there already is a mouse click in the input buffer, wait another
    // time (otherwise we would create a backlog of clicks)
    if (vim_used_in_input_buf() > 10)
	return TRUE;

    motion_repeat_timer = 0;

    /*
     * Fake a motion event.
     * Trick: Pretend the mouse moved to the next character on every other
     * event, otherwise drag events will be discarded, because they are still
     * in the same character.
     */
    if (motion_repeat_offset)
	x += gui.char_width;

    motion_repeat_offset = !motion_repeat_offset;
    process_motion_notify(x, y, state);

    // Don't happen again.  We will get reinstalled in the synthetic event
    // if needed -- thus repeating should still work.
    return FALSE;
}

    static gint
motion_notify_event(GtkWidget *widget,
		    GdkEventMotion *event,
		    gpointer data UNUSED)
{
    if (event->is_hint)
    {
	int		x;
	int		y;
	GdkModifierType	state;

	gui_gtk_get_pointer(widget, &x, &y, &state);
	process_motion_notify(x, y, state);
    }
    else
    {
	process_motion_notify((int)event->x, (int)event->y,
			      (GdkModifierType)event->state);
    }

    return TRUE; // handled
}


/*
 * Mouse button handling.  Note please that we are capturing multiple click's
 * by our own timeout mechanism instead of the one provided by GTK+ itself.
 * This is due to the way the generic VIM code is recognizing multiple clicks.
 */
    static gint
button_press_event(GtkWidget *widget,
		   GdkEventButton *event,
		   gpointer data UNUSED)
{
    int button;
    int repeated_click = FALSE;
    int x, y;
    int_u vim_modifiers;

    gui.event_time = event->time;

    // Make sure we have focus now we've been selected
    if (gtk_socket_id != 0 && !gtk_widget_has_focus(widget))
	gtk_widget_grab_focus(widget);

    /*
     * Don't let additional events about multiple clicks send by GTK to us
     * after the initial button press event confuse us.
     */
    if (event->type != GDK_BUTTON_PRESS)
	return FALSE;

    x = event->x;
    y = event->y;

    // Handle multiple clicks
    if (!mouse_timed_out && mouse_click_timer)
    {
	timeout_remove(mouse_click_timer);
	mouse_click_timer = 0;
	repeated_click = TRUE;
    }

    mouse_timed_out = FALSE;
    mouse_click_timer = timeout_add(p_mouset, mouse_click_timer_cb,
							     &mouse_timed_out);

    switch (event->button)
    {
	// Keep in sync with gui_x11.c.
	// Buttons 4-7 are handled in scroll_event()
	case 1:
		button = MOUSE_LEFT;
		// needed for touch-drag
		dragging_button_state |= GDK_BUTTON1_MASK;
		break;
	case 2: button = MOUSE_MIDDLE; break;
	case 3: button = MOUSE_RIGHT; break;
	case 8: button = MOUSE_X1; break;
	case 9: button = MOUSE_X2; break;
	default:
	    return FALSE;		// Unknown button
    }

#ifdef FEAT_XIM
    // cancel any preediting
    if (im_is_preediting())
	xim_reset();
#endif

    vim_modifiers = modifiers_gdk2mouse(event->state);

    gui_send_mouse_event(button, x, y, repeated_click, vim_modifiers);

    return TRUE;
}

/*
 * GTK+ 2 abstracts scrolling via the GdkEventScroll.
 */
    static gboolean
scroll_event(GtkWidget *widget,
	     GdkEventScroll *event,
	     gpointer data UNUSED)
{
    int	    button;
    int_u   vim_modifiers;
#if GTK_CHECK_VERSION(3,4,0)
    static double  acc_x, acc_y;
    static guint32 last_smooth_event_time;
#endif

    if (gtk_socket_id != 0 && !gtk_widget_has_focus(widget))
	gtk_widget_grab_focus(widget);

    switch (event->direction)
    {
	case GDK_SCROLL_UP:
	    button = MOUSE_4;
	    break;
	case GDK_SCROLL_DOWN:
	    button = MOUSE_5;
	    break;
	case GDK_SCROLL_LEFT:
	    button = MOUSE_7;
	    break;
	case GDK_SCROLL_RIGHT:
	    button = MOUSE_6;
	    break;
#if GTK_CHECK_VERSION(3,4,0)
	case GDK_SCROLL_SMOOTH:
	    if (event->time - last_smooth_event_time > 50)
		// reset our accumulations after 50ms of silence
		acc_x = acc_y = 0;
	    acc_x += event->delta_x;
	    acc_y += event->delta_y;
	    last_smooth_event_time = event->time;
	    break;
#endif
	default: // This shouldn't happen
	    return FALSE;
    }

# ifdef FEAT_XIM
    // cancel any preediting
    if (im_is_preediting())
	xim_reset();
# endif

    vim_modifiers = modifiers_gdk2mouse(event->state);

#if GTK_CHECK_VERSION(3,4,0)
    if (event->direction == GDK_SCROLL_SMOOTH)
    {
	while (acc_x > 1.0)
	{ // right
	    acc_x = MAX(0.0, acc_x - 1.0);
	    gui_send_mouse_event(MOUSE_6, (int)event->x, (int)event->y,
		    FALSE, vim_modifiers);
	}
	while (acc_x < -1.0)
	{ // left
	    acc_x = MIN(0.0, acc_x + 1.0);
	    gui_send_mouse_event(MOUSE_7, (int)event->x, (int)event->y,
		    FALSE, vim_modifiers);
	}
	while (acc_y > 1.0)
	{ // down
	    acc_y = MAX(0.0, acc_y - 1.0);
	    gui_send_mouse_event(MOUSE_5, (int)event->x, (int)event->y,
		    FALSE, vim_modifiers);
	}
	while (acc_y < -1.0)
	{ // up
	    acc_y = MIN(0.0, acc_y + 1.0);
	    gui_send_mouse_event(MOUSE_4, (int)event->x, (int)event->y,
		    FALSE, vim_modifiers);
	}
    }
    else
#endif
	gui_send_mouse_event(button, (int)event->x, (int)event->y,
		FALSE, vim_modifiers);

    return TRUE;
}


    static gint
button_release_event(GtkWidget *widget UNUSED,
		     GdkEventButton *event,
		     gpointer data UNUSED)
{
    int x, y;
    int_u vim_modifiers;

    gui.event_time = event->time;

    // Remove any motion "machine gun" timers used for automatic further
    // extension of allocation areas if outside of the applications window
    // area .
    if (motion_repeat_timer)
    {
	timeout_remove(motion_repeat_timer);
	motion_repeat_timer = 0;
    }

    x = event->x;
    y = event->y;

    vim_modifiers = modifiers_gdk2mouse(event->state);

    gui_send_mouse_event(MOUSE_RELEASE, x, y, FALSE, vim_modifiers);

    switch (event->button)
    {
	case 1:  // MOUSE_LEFT
	    dragging_button_state = 0;
	    break;
    }

    return TRUE;
}


#ifdef FEAT_DND
/////////////////////////////////////////////////////////////////////////////
// Drag aNd Drop support handlers.

/*
 * Count how many items there may be and separate them with a NUL.
 * Apparently the items are separated with \r\n.  This is not documented,
 * thus be careful not to go past the end.	Also allow separation with
 * NUL characters.
 */
    static int
count_and_decode_uri_list(char_u *out, char_u *raw, int len)
{
    int		i;
    char_u	*p = out;
    int		count = 0;

    for (i = 0; i < len; ++i)
    {
	if (raw[i] == NUL || raw[i] == '\n' || raw[i] == '\r')
	{
	    if (p > out && p[-1] != NUL)
	    {
		++count;
		*p++ = NUL;
	    }
	}
	else if (raw[i] == '%' && i + 2 < len && hexhex2nr(raw + i + 1) > 0)
	{
	    *p++ = hexhex2nr(raw + i + 1);
	    i += 2;
	}
	else
	    *p++ = raw[i];
    }
    if (p > out && p[-1] != NUL)
    {
	*p = NUL;	// last item didn't have \r or \n
	++count;
    }
    return count;
}

/*
 * Parse NUL separated "src" strings.  Make it an array "outlist" form.  On
 * this process, URI which protocol is not "file:" are removed.  Return
 * length of array (less than "max").
 */
    static int
filter_uri_list(char_u **outlist, int max, char_u *src)
{
    int	i, j;

    for (i = j = 0; i < max; ++i)
    {
	outlist[i] = NULL;
	if (STRNCMP(src, "file:", 5) == 0)
	{
	    src += 5;
	    if (STRNCMP(src, "//localhost", 11) == 0)
		src += 11;
	    while (src[0] == '/' && src[1] == '/')
		++src;
	    outlist[j++] = vim_strsave(src);
	}
	src += STRLEN(src) + 1;
    }
    return j;
}

    static char_u **
parse_uri_list(int *count, char_u *data, int len)
{
    int	    n	    = 0;
    char_u  *tmp    = NULL;
    char_u  **array = NULL;

    if (data != NULL && len > 0 && (tmp = alloc(len + 1)) != NULL)
    {
	n = count_and_decode_uri_list(tmp, data, len);
	if (n > 0 && (array = ALLOC_MULT(char_u *, n)) != NULL)
	    n = filter_uri_list(array, n, tmp);
    }
    vim_free(tmp);
    *count = n;
    return array;
}

    static void
drag_handle_uri_list(GdkDragContext	*context,
		     GtkSelectionData	*data,
		     guint		time_,
		     GdkModifierType	state,
		     gint		x,
		     gint		y)
{
    char_u  **fnames;
    int	    nfiles = 0;

    fnames = parse_uri_list(&nfiles,
	    (char_u *)gtk_selection_data_get_data(data),
	    gtk_selection_data_get_length(data));

    if (fnames != NULL && nfiles > 0)
    {
	int_u   modifiers;

	gtk_drag_finish(context, TRUE, FALSE, time_); // accept

	modifiers = modifiers_gdk2mouse(state);

	gui_handle_drop(x, y, modifiers, fnames, nfiles);
    }
    else
	vim_free(fnames);
}

    static void
drag_handle_text(GdkDragContext	    *context,
		 GtkSelectionData   *data,
		 guint		    time_,
		 GdkModifierType    state)
{
    char_u  dropkey[6] = {CSI, KS_MODIFIER, 0, CSI, KS_EXTRA, (char_u)KE_DROP};
    char_u  *text;
    int	    len;
    char_u  *tmpbuf = NULL;

    text = (char_u *)gtk_selection_data_get_data(data);
    len = gtk_selection_data_get_length(data);

    if (gtk_selection_data_get_data_type(data) == utf8_string_atom)
    {
	if (input_conv.vc_type != CONV_NONE)
	    tmpbuf = string_convert(&input_conv, text, &len);
	if (tmpbuf != NULL)
	    text = tmpbuf;
    }

    dnd_yank_drag_data(text, (long)len);
    gtk_drag_finish(context, TRUE, FALSE, time_); // accept
    vim_free(tmpbuf);

    dropkey[2] = modifiers_gdk2vim(state);

    if (dropkey[2] != 0)
	add_to_input_buf(dropkey, (int)sizeof(dropkey));
    else
	add_to_input_buf(dropkey + 3, (int)(sizeof(dropkey) - 3));
}

/*
 * DND receiver.
 */
    static void
drag_data_received_cb(GtkWidget		*widget,
		      GdkDragContext	*context,
		      gint		x,
		      gint		y,
		      GtkSelectionData	*data,
		      guint		info,
		      guint		time_,
		      gpointer		user_data UNUSED)
{
    GdkModifierType state;

    // Guard against trash
    const guchar * const data_data = gtk_selection_data_get_data(data);
    const gint data_length = gtk_selection_data_get_length(data);
    const gint data_format = gtk_selection_data_get_format(data);

    if (data_data == NULL
	    || data_length <= 0
	    || data_format != 8
	    || data_data[data_length] != '\0')
    {
	gtk_drag_finish(context, FALSE, FALSE, time_);
	return;
    }

    // Get the current modifier state for proper distinguishment between
    // different operations later.
    gui_gtk_get_pointer(widget, NULL, NULL, &state);

    // Not sure about the role of "text/plain" here...
    if (info == (guint)TARGET_TEXT_URI_LIST)
	drag_handle_uri_list(context, data, time_, state, x, y);
    else
	drag_handle_text(context, data, time_, state);

}
#endif // FEAT_DND


#if defined(USE_GNOME_SESSION)
/*
 * GnomeClient interact callback.  Check for unsaved buffers that cannot
 * be abandoned and pop up a dialog asking the user for confirmation if
 * necessary.
 */
    static void
sm_client_check_changed_any(GnomeClient	    *client UNUSED,
			    gint	    key,
			    GnomeDialogType type UNUSED,
			    gpointer	    data UNUSED)
{
    cmdmod_T	save_cmdmod;
    gboolean	shutdown_cancelled;

    save_cmdmod = cmdmod;

# ifdef FEAT_BROWSE
    cmdmod.cmod_flags |= CMOD_BROWSE;
# endif
# if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
    cmdmod.cmod_flags |= CMOD_CONFIRM;
# endif
    /*
     * If there are changed buffers, present the user with
     * a dialog if possible, otherwise give an error message.
     */
    shutdown_cancelled = check_changed_any(FALSE, FALSE);

    exiting = FALSE;
    cmdmod = save_cmdmod;
    setcursor(); // position the cursor
    out_flush();
    /*
     * If the user hit the [Cancel] button the whole shutdown
     * will be cancelled.  Wow, quite powerful feature (:
     */
    gnome_interaction_key_return(key, shutdown_cancelled);
}

/*
 * "save_yourself" signal handler.  Initiate an interaction to ask the user
 * for confirmation if necessary.  Save the current editing session and tell
 * the session manager how to restart Vim.
 */
    static gboolean
sm_client_save_yourself(GnomeClient	    *client,
			gint		    phase UNUSED,
			GnomeSaveStyle	    save_style UNUSED,
			gboolean	    shutdown UNUSED,
			GnomeInteractStyle  interact_style,
			gboolean	    fast UNUSED,
			gpointer	    data UNUSED)
{
    static const char	suffix[] = "-session.vim";
    char		*session_file;
    unsigned int	len;
    gboolean		success;

    // Always request an interaction if possible.  check_changed_any()
    // won't actually show a dialog unless any buffers have been modified.
    // There doesn't seem to be an obvious way to check that without
    // automatically firing the dialog.  Anyway, it works just fine.
    if (interact_style == GNOME_INTERACT_ANY)
	gnome_client_request_interaction(client, GNOME_DIALOG_NORMAL,
					 &sm_client_check_changed_any,
					 NULL);
    out_flush();
    ml_sync_all(FALSE, FALSE); // preserve all swap files

    // The path is unique for each session save.  We do neither know nor care
    // which session script will actually be used later.  This decision is in
    // the domain of the session manager.
    session_file = gnome_config_get_real_path(
			gnome_client_get_config_prefix(client));
    len = strlen(session_file);

    if (len > 0 && session_file[len-1] == G_DIR_SEPARATOR)
	--len; // get rid of the superfluous trailing '/'

    session_file = g_renew(char, session_file, len + sizeof(suffix));
    memcpy(session_file + len, suffix, sizeof(suffix));

    success = write_session_file((char_u *)session_file);

    if (success)
    {
	const char  *argv[8];
	int	    i;

	// Tell the session manager how to wipe out the stored session data.
	// This isn't as dangerous as it looks, don't worry :)	session_file
	// is a unique absolute filename.  Usually it'll be something like
	// `/home/user/.gnome2/vim-XXXXXX-session.vim'.
	i = 0;
	argv[i++] = "rm";
	argv[i++] = session_file;
	argv[i] = NULL;

	gnome_client_set_discard_command(client, i, (char **)argv);

	// Tell the session manager how to restore the just saved session.
	// This is easily done thanks to Vim's -S option.  Pass the -f flag
	// since there's no need to fork -- it might even cause confusion.
	// Also pass the window role to give the WM something to match on.
	// The role is set in gui_mch_open(), thus should _never_ be NULL.
	i = 0;
	argv[i++] = restart_command;
	argv[i++] = "-f";
	argv[i++] = "-g";
	argv[i++] = "--role";
	argv[i++] = gtk_window_get_role(GTK_WINDOW(gui.mainwin));
	argv[i++] = "-S";
	argv[i++] = session_file;
	argv[i] = NULL;

	gnome_client_set_restart_command(client, i, (char **)argv);
	gnome_client_set_clone_command(client, 0, NULL);
    }

    g_free(session_file);

    return success;
}

/*
 * Called when the session manager wants us to die.  There isn't much to save
 * here since "save_yourself" has been emitted before (unless serious trouble
 * is happening).
 */
    static void
sm_client_die(GnomeClient *client UNUSED, gpointer data UNUSED)
{
    // Don't write messages to the GUI anymore
    full_screen = FALSE;

    vim_strncpy(IObuff, (char_u *)
		    _("Vim: Received \"die\" request from session manager\n"),
		    IOSIZE - 1);
    preserve_exit();
}

/*
 * Connect our signal handlers to be notified on session save and shutdown.
 */
    static void
setup_save_yourself(void)
{
    GnomeClient *client;

    client = gnome_master_client();
    if (client == NULL)
	return;

    // Must use the deprecated gtk_signal_connect() for compatibility
    // with GNOME 1.  Arrgh, zombies!
    gtk_signal_connect(GTK_OBJECT(client), "save_yourself",
	    GTK_SIGNAL_FUNC(&sm_client_save_yourself), NULL);
    gtk_signal_connect(GTK_OBJECT(client), "die",
	    GTK_SIGNAL_FUNC(&sm_client_die), NULL);
}

#else // !USE_GNOME_SESSION

# ifdef USE_XSMP
/*
 * GTK tells us that XSMP needs attention
 */
    static gboolean
local_xsmp_handle_requests(
    GIOChannel		*source UNUSED,
    GIOCondition	condition,
    gpointer		data)
{
    if (condition == G_IO_IN)
    {
	// Do stuff; maybe close connection
	if (xsmp_handle_requests() == FAIL)
	    g_io_channel_unref((GIOChannel *)data);
	return TRUE;
    }
    // Error
    g_io_channel_unref((GIOChannel *)data);
    xsmp_close();
    return TRUE;
}
# endif // USE_XSMP

/*
 * Setup the WM_PROTOCOLS to indicate we want the WM_SAVE_YOURSELF event.
 * This is an ugly use of X functions.	GTK doesn't offer an alternative.
 */
    static void
setup_save_yourself(void)
{
    Atom    *existing_atoms = NULL;
    int	    count = 0;

# ifdef USE_XSMP
    if (xsmp_icefd != -1)
    {
	/*
	 * Use XSMP is preference to legacy WM_SAVE_YOURSELF;
	 * set up GTK IO monitor
	 */
	GIOChannel *g_io = g_io_channel_unix_new(xsmp_icefd);

	g_io_add_watch(g_io, G_IO_IN | G_IO_ERR | G_IO_HUP,
				  local_xsmp_handle_requests, (gpointer)g_io);
	g_io_channel_unref(g_io);
    }
    else
# endif
    {
	// Fall back to old method

	// first get the existing value
	Display * dpy = gui_mch_get_display();
	if (!dpy)
	    return;

	GdkWindow * const mainwin_win = gtk_widget_get_window(gui.mainwin);
	if (XGetWMProtocols(dpy, GDK_WINDOW_XID(mainwin_win),
		    &existing_atoms, &count))
	{
	    Atom	*new_atoms;
	    Atom	save_yourself_xatom;
	    int	i;

	    save_yourself_xatom = GET_X_ATOM(save_yourself_atom);

	    // check if WM_SAVE_YOURSELF isn't there yet
	    for (i = 0; i < count; ++i)
		if (existing_atoms[i] == save_yourself_xatom)
		    break;

	    if (i == count)
	    {
		// allocate an Atoms array which is one item longer
		new_atoms = ALLOC_MULT(Atom, count + 1);
		if (new_atoms != NULL)
		{
		    memcpy(new_atoms, existing_atoms, count * sizeof(Atom));
		    new_atoms[count] = save_yourself_xatom;
		    XSetWMProtocols(GDK_WINDOW_XDISPLAY(mainwin_win),
			    GDK_WINDOW_XID(mainwin_win),
			    new_atoms, count + 1);
		    vim_free(new_atoms);
		}
	    }
	    XFree(existing_atoms);
	}
    }
}

/*
 * Installing a global event filter seems to be the only way to catch
 * client messages of type WM_PROTOCOLS without overriding GDK's own
 * client message event filter.  Well, that's still better than trying
 * to guess what the GDK filter had done if it had been invoked instead
 *
 * GTK2_FIXME:	This doesn't seem to work.  For some reason we never
 * receive WM_SAVE_YOURSELF even though everything is set up correctly.
 * I have the nasty feeling modern session managers just don't send this
 * deprecated message anymore.	Addition: confirmed by several people.
 *
 * The GNOME session support is much cooler anyway.  Unlike this ugly
 * WM_SAVE_YOURSELF hack it actually stores the session...  And yes,
 * it should work with KDE as well.
 */
    static GdkFilterReturn
global_event_filter(GdkXEvent *xev,
		    GdkEvent *event UNUSED,
		    gpointer data UNUSED)
{
    XEvent *xevent = (XEvent *)xev;

    if (xevent != NULL
	    && xevent->type == ClientMessage
	    && xevent->xclient.message_type == GET_X_ATOM(wm_protocols_atom)
	    && (long_u)xevent->xclient.data.l[0]
					    == GET_X_ATOM(save_yourself_atom))
    {
	out_flush();
	ml_sync_all(FALSE, FALSE); // preserve all swap files
	/*
	 * Set the window's WM_COMMAND property, to let the window manager
	 * know we are done saving ourselves.  We don't want to be
	 * restarted, thus set argv to NULL.
	 */
	XSetCommand(GDK_WINDOW_XDISPLAY(gtk_widget_get_window(gui.mainwin)),
		    GDK_WINDOW_XID(gtk_widget_get_window(gui.mainwin)),
		    NULL, 0);
	return GDK_FILTER_REMOVE;
    }

    return GDK_FILTER_CONTINUE;
}
#endif // !USE_GNOME_SESSION


/*
 * Setup the window icon & xcmdsrv comm after the main window has been realized.
 */
    static void
mainwin_realize(GtkWidget *widget UNUSED, gpointer data UNUSED)
{
// If you get an error message here, you still need to unpack the runtime
// archive!
#ifdef magick
# undef magick
#endif
  // A bit hackish, but avoids casting later and allows optimization
# define static static const
#define magick vim32x32
#include "../runtime/vim32x32.xpm"
#undef magick
#define magick vim16x16
#include "../runtime/vim16x16.xpm"
#undef magick
#define magick vim48x48
#include "../runtime/vim48x48.xpm"
#undef magick
# undef static

    GdkWindow * const mainwin_win = gtk_widget_get_window(gui.mainwin);

    // When started with "--echo-wid" argument, write window ID on stdout.
    if (echo_wid_arg)
    {
	if (gui_mch_get_display())
	    printf("WID: %ld\n", (long)GDK_WINDOW_XID(mainwin_win));
	else
	    printf("WID: 0\n");
	fflush(stdout);
    }

    if (vim_strchr(p_go, GO_ICON) != NULL)
    {
	/*
	 * Add an icon to the main window. For fun and convenience of the user.
	 */
	GList *icons = NULL;

	icons = g_list_prepend(icons, gdk_pixbuf_new_from_xpm_data(vim16x16));
	icons = g_list_prepend(icons, gdk_pixbuf_new_from_xpm_data(vim32x32));
	icons = g_list_prepend(icons, gdk_pixbuf_new_from_xpm_data(vim48x48));

	gtk_window_set_icon_list(GTK_WINDOW(gui.mainwin), icons);

	// TODO: is this type cast OK?
	g_list_foreach(icons, (GFunc)(void *)&g_object_unref, NULL);
	g_list_free(icons);
    }

#if !defined(USE_GNOME_SESSION)
    // Register a handler for WM_SAVE_YOURSELF with GDK's low-level X I/F
    gdk_window_add_filter(NULL, &global_event_filter, NULL);
#endif
    // Setup to indicate to the window manager that we want to catch the
    // WM_SAVE_YOURSELF event.	For GNOME, this connects to the session
    // manager instead.
#if defined(USE_GNOME_SESSION)
    if (using_gnome)
#endif
	setup_save_yourself();

#ifdef FEAT_CLIENTSERVER
    if (gui_mch_get_display())
    {
	if (serverName == NULL && serverDelayedStartName != NULL)
	{
	    // This is a :gui command in a plain vim with no previous server
	    commWindow = GDK_WINDOW_XID(mainwin_win);

	    (void)serverRegisterName(GDK_WINDOW_XDISPLAY(mainwin_win),
				    serverDelayedStartName);
	}
	else
	{
	    /*
	    * Cannot handle "XLib-only" windows with gtk event routines, we'll
	    * have to change the "server" registration to that of the main window
	    * If we have not registered a name yet, remember the window.
	    */
	    serverChangeRegisteredWindow(GDK_WINDOW_XDISPLAY(mainwin_win),
					GDK_WINDOW_XID(mainwin_win));
	}
	gtk_widget_add_events(gui.mainwin, GDK_PROPERTY_CHANGE_MASK);
	g_signal_connect(G_OBJECT(gui.mainwin), "property-notify-event",
			G_CALLBACK(property_event), NULL);
    }
#endif
}

    static GdkCursor *
create_blank_pointer(void)
{
    GdkWindow	*root_window = NULL;
#if GTK_CHECK_VERSION(3,0,0)
    GdkPixbuf   *blank_mask;
#else
    GdkPixmap	*blank_mask;
#endif
    GdkCursor	*cursor;
#if GTK_CHECK_VERSION(3,0,0)
    GdkRGBA	color = { 0.0, 0.0, 0.0, 0.0 };
#else
    GdkColor	color = { 0, 0, 0, 0 };
    char	blank_data[] = { 0x0 };
#endif

#if GTK_CHECK_VERSION(3,12,0)
    {
	GdkWindow * const win = gtk_widget_get_window(gui.mainwin);
	GdkScreen * const scrn = gdk_window_get_screen(win);
	root_window = gdk_screen_get_root_window(scrn);
    }
#else
    root_window = gtk_widget_get_root_window(gui.mainwin);
#endif

    // Create a pseudo blank pointer, which is in fact one pixel by one pixel
    // in size.
#if GTK_CHECK_VERSION(3,0,0)
    {
	cairo_surface_t *surf;
	cairo_t		*cr;

	surf = cairo_image_surface_create(CAIRO_FORMAT_A1, 1, 1);
	cr = cairo_create(surf);

	cairo_set_source_rgba(cr,
			     color.red,
			     color.green,
			     color.blue,
			     color.alpha);
	cairo_rectangle(cr, 0, 0, 1, 1);
	cairo_fill(cr);
	cairo_destroy(cr);

	blank_mask = gdk_pixbuf_get_from_surface(surf, 0, 0, 1, 1);
	cairo_surface_destroy(surf);

	cursor = gdk_cursor_new_from_pixbuf(gdk_window_get_display(root_window),
					    blank_mask, 0, 0);
	g_object_unref(blank_mask);
    }
#else
    blank_mask = gdk_bitmap_create_from_data(root_window, blank_data, 1, 1);
    cursor = gdk_cursor_new_from_pixmap(blank_mask, blank_mask,
					&color, &color, 0, 0);
    gdk_bitmap_unref(blank_mask);
#endif

    return cursor;
}

    static void
mainwin_screen_changed_cb(GtkWidget  *widget,
			  GdkScreen  *previous_screen UNUSED,
			  gpointer   data UNUSED)
{
    if (!gtk_widget_has_screen(widget))
	return;

    /*
     * Recreate the invisible mouse cursor.
     */
    if (gui.blank_pointer != NULL)
#if GTK_CHECK_VERSION(3,0,0)
	g_object_unref(G_OBJECT(gui.blank_pointer));
#else
	gdk_cursor_unref(gui.blank_pointer);
#endif

    gui.blank_pointer = create_blank_pointer();

    if (gui.pointer_hidden && gtk_widget_get_window(gui.drawarea) != NULL)
	gdk_window_set_cursor(gtk_widget_get_window(gui.drawarea),
		gui.blank_pointer);

    /*
     * Create a new PangoContext for this screen, and initialize it
     * with the current font if necessary.
     */
    if (gui.text_context != NULL)
	g_object_unref(gui.text_context);

    gui.text_context = gtk_widget_create_pango_context(widget);
    pango_context_set_base_dir(gui.text_context, PANGO_DIRECTION_LTR);

    if (gui.norm_font != NULL)
    {
	gui_mch_init_font(p_guifont, FALSE);
	gui_set_shellsize(TRUE, FALSE, RESIZE_BOTH);
    }
}

/*
 * After the drawing area comes up, we calculate all colors and create the
 * dummy blank cursor.
 *
 * Don't try to set any VIM scrollbar sizes anywhere here. I'm relying on the
 * fact that the main VIM engine doesn't take them into account anywhere.
 */
    static void
drawarea_realize_cb(GtkWidget *widget, gpointer data UNUSED)
{
    GtkWidget *sbar;
    GtkAllocation allocation;

#ifdef FEAT_XIM
    xim_init();
#endif
    gui_mch_new_colors();
#if GTK_CHECK_VERSION(3,0,0)
    gui.surface = gdk_window_create_similar_surface(
	    gtk_widget_get_window(widget),
	    CAIRO_CONTENT_COLOR_ALPHA,
	    gtk_widget_get_allocated_width(widget),
	    gtk_widget_get_allocated_height(widget));
#else
    gui.text_gc = gdk_gc_new(gui.drawarea->window);
#endif

    gui.blank_pointer = create_blank_pointer();
    if (gui.pointer_hidden)
	gdk_window_set_cursor(gtk_widget_get_window(widget), gui.blank_pointer);

    // get the actual size of the scrollbars, if they are realized
    sbar = firstwin->w_scrollbars[SBAR_LEFT].id;
    if (!sbar || (!gui.which_scrollbars[SBAR_LEFT]
				    && firstwin->w_scrollbars[SBAR_RIGHT].id))
	sbar = firstwin->w_scrollbars[SBAR_RIGHT].id;
    gtk_widget_get_allocation(sbar, &allocation);
    if (sbar && gtk_widget_get_realized(sbar) && allocation.width)
	gui.scrollbar_width = allocation.width;

    sbar = gui.bottom_sbar.id;
    if (sbar && gtk_widget_get_realized(sbar) && allocation.height)
	gui.scrollbar_height = allocation.height;
}

/*
 * Properly clean up on shutdown.
 */
    static void
drawarea_unrealize_cb(GtkWidget *widget UNUSED, gpointer data UNUSED)
{
    // Don't write messages to the GUI anymore
    full_screen = FALSE;

#ifdef FEAT_XIM
    im_shutdown();
#endif
    if (gui.ascii_glyphs != NULL)
    {
	pango_glyph_string_free(gui.ascii_glyphs);
	gui.ascii_glyphs = NULL;
    }
    if (gui.ascii_font != NULL)
    {
	g_object_unref(gui.ascii_font);
	gui.ascii_font = NULL;
    }
    g_object_unref(gui.text_context);
    gui.text_context = NULL;

#if GTK_CHECK_VERSION(3,0,0)
    if (gui.surface != NULL)
    {
	cairo_surface_destroy(gui.surface);
	gui.surface = NULL;
    }
#else
    g_object_unref(gui.text_gc);
    gui.text_gc = NULL;
#endif

#if GTK_CHECK_VERSION(3,0,0)
    g_object_unref(G_OBJECT(gui.blank_pointer));
#else
    gdk_cursor_unref(gui.blank_pointer);
#endif
    gui.blank_pointer = NULL;
}

#if GTK_CHECK_VERSION(3,22,2)
    static void
drawarea_style_updated_cb(GtkWidget *widget UNUSED,
			 gpointer data UNUSED)
#else
    static void
drawarea_style_set_cb(GtkWidget	*widget UNUSED,
		      GtkStyle	*previous_style UNUSED,
		      gpointer	data UNUSED)
#endif
{
    gui_mch_new_colors();
}

#if GTK_CHECK_VERSION(3,0,0)
    static gboolean
drawarea_configure_event_cb(GtkWidget	      *widget,
			    GdkEventConfigure *event,
			    gpointer	       data UNUSED)
{
    static int cur_width = 0;
    static int cur_height = 0;

    g_return_val_if_fail(event
	    && event->width >= 1 && event->height >= 1, TRUE);

# if GTK_CHECK_VERSION(3,22,2) && !GTK_CHECK_VERSION(3,22,4)
    // As of 3.22.2, GdkWindows have started distributing configure events to
    // their "native" children (https://git.gnome.org/browse/gtk+/commit/?h=gtk-3-22&id=12579fe71b3b8f79eb9c1b80e429443bcc437dd0).
    //
    // As can be seen from the implementation of move_native_children() and
    // configure_native_child() in gdkwindow.c, those functions actually
    // propagate configure events to every child, failing to distinguish
    // "native" one from non-native one.
    //
    // Naturally, configure events propagated to here like that are fallacious
    // and, as a matter of fact, they trigger a geometric collapse of
    // gui.drawarea in fullscreen and maximized modes.
    //
    // To filter out such nuisance events, we are making use of the fact that
    // the field send_event of such GdkEventConfigures is set to FALSE in
    // configure_native_child().
    //
    // Obviously, this is a terrible hack making GVim depend on GTK's
    // implementation details.  Therefore, watch out any relevant internal
    // changes happening in GTK in the feature (sigh).
    //
    // Follow-up
    // After a few weeks later, the GdkWindow change mentioned above was
    // reverted (https://git.gnome.org/browse/gtk+/commit/?h=gtk-3-22&id=f70039cb9603a02d2369fec4038abf40a1711155).
    // The corresponding official release is 3.22.4.
    if (event->send_event == FALSE)
	return TRUE;
# endif

    if (event->width == cur_width && event->height == cur_height)
	return TRUE;

    cur_width = event->width;
    cur_height = event->height;

    if (gui.surface != NULL)
	cairo_surface_destroy(gui.surface);

    gui.surface = gdk_window_create_similar_surface(
	    gtk_widget_get_window(widget),
	    CAIRO_CONTENT_COLOR_ALPHA,
	    event->width, event->height);

    gtk_widget_queue_draw(widget);

    return TRUE;
}
#endif

/*
 * Callback routine for the "delete_event" signal on the toplevel window.
 * Tries to vim gracefully, or refuses to exit with changed buffers.
 */
    static gint
delete_event_cb(GtkWidget *widget UNUSED,
		GdkEventAny *event UNUSED,
		gpointer data UNUSED)
{
    gui_shell_closed();
    return TRUE;
}

#if defined(FEAT_MENU) || defined(FEAT_TOOLBAR) || defined(FEAT_GUI_TABLINE)
    static int
get_item_dimensions(GtkWidget *widget, GtkOrientation orientation)
{
# ifdef FEAT_GUI_GNOME
    GtkOrientation item_orientation = GTK_ORIENTATION_HORIZONTAL;

    if (using_gnome && widget != NULL)
    {
	GtkWidget *parent;
	BonoboDockItem *dockitem;

	parent	 = gtk_widget_get_parent(widget);
	if (G_TYPE_FROM_INSTANCE(parent) == BONOBO_TYPE_DOCK_ITEM)
	{
	    // Only menu & toolbar are dock items.  Could tabline be?
	    // Seem to be only the 2 defined in GNOME
	    widget = parent;
	    dockitem = BONOBO_DOCK_ITEM(widget);

	    if (dockitem == NULL || dockitem->is_floating)
		return 0;
	    item_orientation = bonobo_dock_item_get_orientation(dockitem);
	}
    }
# else
#  define item_orientation GTK_ORIENTATION_HORIZONTAL
# endif

    if (widget != NULL
	    && item_orientation == orientation
	    && gtk_widget_get_realized(widget)
	    && gtk_widget_get_visible(widget))
    {
# if GTK_CHECK_VERSION(3,0,0) || !defined(FEAT_GUI_GNOME)
	GtkAllocation allocation;

	gtk_widget_get_allocation(widget, &allocation);
	return allocation.height;
# else
	if (orientation == GTK_ORIENTATION_HORIZONTAL)
	    return widget->allocation.height;
	else
	    return widget->allocation.width;
# endif
    }
    return 0;
}
#endif

    static int
get_menu_tool_width(void)
{
    int width = 0;

#ifdef FEAT_GUI_GNOME // these are never vertical without GNOME
# ifdef FEAT_MENU
    width += get_item_dimensions(gui.menubar, GTK_ORIENTATION_VERTICAL);
# endif
# ifdef FEAT_TOOLBAR
    width += get_item_dimensions(gui.toolbar, GTK_ORIENTATION_VERTICAL);
# endif
# ifdef FEAT_GUI_TABLINE
    if (gui.tabline != NULL)
	width += get_item_dimensions(gui.tabline, GTK_ORIENTATION_VERTICAL);
# endif
#endif

    return width;
}

    static int
get_menu_tool_height(void)
{
    int height = 0;

#ifdef FEAT_MENU
    height += get_item_dimensions(gui.menubar, GTK_ORIENTATION_HORIZONTAL);
#endif
#ifdef FEAT_TOOLBAR
    height += get_item_dimensions(gui.toolbar, GTK_ORIENTATION_HORIZONTAL);
#endif
#ifdef FEAT_GUI_TABLINE
    if (gui.tabline != NULL)
	height += get_item_dimensions(gui.tabline, GTK_ORIENTATION_HORIZONTAL);
#endif

    return height;
}

// This controls whether we can set the real window hints at
// start-up when in a GtkPlug.
// 0 = normal processing (default)
// 1 = init. hints set, no-one's tried to reset since last check
// 2 = init. hints set, attempt made to change hints
static int init_window_hints_state = 0;

    static void
update_window_manager_hints(int force_width, int force_height)
{
    static int old_width  = 0;
    static int old_height = 0;
    static int old_min_width  = 0;
    static int old_min_height = 0;
    static int old_char_width  = 0;
    static int old_char_height = 0;

    int width;
    int height;
    int min_width;
    int min_height;

    // At start-up, don't try to set the hints until the initial
    // values have been used (those that dictate our initial size)
    // Let forced (i.e., correct) values through always.
    if (!(force_width && force_height)  &&  init_window_hints_state > 0)
    {
	// Don't do it!
	init_window_hints_state = 2;
	return;
    }

    // This also needs to be done when the main window isn't there yet,
    // otherwise the hints don't work.
    width  = gui_get_base_width();
    height = gui_get_base_height();
# ifdef FEAT_MENU
    height += tabline_height() * gui.char_height;
# endif
    width  += get_menu_tool_width();
    height += get_menu_tool_height();

    // GtkSockets use GtkPlug's [gui,mainwin] min-size hints to determine
    // their actual widget size.  When we set our size ourselves (e.g.,
    // 'set columns=' or init. -geom) we briefly set the min. to the size
    // we wish to be instead of the legitimate minimum so that we actually
    // resize correctly.
    if (force_width && force_height)
    {
	min_width  = force_width;
	min_height = force_height;
    }
    else
    {
	min_width  = width  + MIN_COLUMNS * gui.char_width;
	min_height = height + MIN_LINES   * gui.char_height;
    }

    // Avoid an expose event when the size didn't change.
    if (width != old_width
	    || height != old_height
	    || min_width != old_min_width
	    || min_height != old_min_height
	    || gui.char_width != old_char_width
	    || gui.char_height != old_char_height)
    {
	GdkGeometry	geometry;
	GdkWindowHints	geometry_mask;

	geometry.width_inc   = gui.char_width;
	geometry.height_inc  = gui.char_height;
	geometry.base_width  = width;
	geometry.base_height = height;
	geometry.min_width   = min_width;
	geometry.min_height  = min_height;
	geometry_mask	     = GDK_HINT_BASE_SIZE|GDK_HINT_RESIZE_INC
			       |GDK_HINT_MIN_SIZE;
	// Using gui.formwin as geometry widget doesn't work as expected
	// with GTK+ 2 -- dunno why.  Presumably all the resizing hacks
	// in Vim confuse GTK+.  For GTK 3 the second argument should be NULL
	// to make the width/height inc works, despite the docs saying
	// something else.
	gtk_window_set_geometry_hints(GTK_WINDOW(gui.mainwin), NULL,
				      &geometry, geometry_mask);
	old_width       = width;
	old_height      = height;
	old_min_width   = min_width;
	old_min_height  = min_height;
	old_char_width  = gui.char_width;
	old_char_height = gui.char_height;
    }
}

#if defined(FEAT_GUI_DARKTHEME) || defined(PROTO)
    void
gui_mch_set_dark_theme(int dark)
{
# if GTK_CHECK_VERSION(3,0,0)
    GtkSettings *gtk_settings;

    gtk_settings = gtk_settings_get_for_screen(gdk_screen_get_default());
    g_object_set(gtk_settings, "gtk-application-prefer-dark-theme", (gboolean)dark, NULL);
# endif
}
#endif // FEAT_GUI_DARKTHEME

#ifdef FEAT_TOOLBAR

/*
 * This extra effort wouldn't be necessary if we only used stock icons in the
 * toolbar, as we do for all builtin icons.  But user-defined toolbar icons
 * shouldn't be treated differently, thus we do need this.
 */
    static void
icon_size_changed_foreach(GtkWidget *widget, gpointer user_data)
{
    if (GTK_IS_IMAGE(widget))
    {
	GtkImage *image = (GtkImage *)widget;

# if GTK_CHECK_VERSION(3,10,0)
	if (gtk_image_get_storage_type(image) == GTK_IMAGE_ICON_NAME)
	{
	    const GtkIconSize icon_size = GPOINTER_TO_INT(user_data);
	    const gchar *icon_name;

	    gtk_image_get_icon_name(image, &icon_name, NULL);
	    image = (GtkImage *)gtk_image_new_from_icon_name(
							 icon_name, icon_size);
	}
# else
	// User-defined icons are stored in a GtkIconSet
	if (gtk_image_get_storage_type(image) == GTK_IMAGE_ICON_SET)
	{
	    GtkIconSet	*icon_set;
	    GtkIconSize	icon_size;

	    gtk_image_get_icon_set(image, &icon_set, &icon_size);
	    icon_size = (GtkIconSize)(long)user_data;

	    gtk_icon_set_ref(icon_set);
	    gtk_image_set_from_icon_set(image, icon_set, icon_size);
	    gtk_icon_set_unref(icon_set);
	}
# endif
    }
    else if (GTK_IS_CONTAINER(widget))
    {
	gtk_container_foreach((GtkContainer *)widget,
			      &icon_size_changed_foreach,
			      user_data);
    }
}

    static void
set_toolbar_style(GtkToolbar *toolbar)
{
    GtkToolbarStyle style;
    GtkIconSize	    size;
    GtkIconSize	    oldsize;

    if ((toolbar_flags & (TOOLBAR_TEXT | TOOLBAR_ICONS | TOOLBAR_HORIZ))
		      == (TOOLBAR_TEXT | TOOLBAR_ICONS | TOOLBAR_HORIZ))
	style = GTK_TOOLBAR_BOTH_HORIZ;
    else if ((toolbar_flags & (TOOLBAR_TEXT | TOOLBAR_ICONS))
		      == (TOOLBAR_TEXT | TOOLBAR_ICONS))
	style = GTK_TOOLBAR_BOTH;
    else if (toolbar_flags & TOOLBAR_TEXT)
	style = GTK_TOOLBAR_TEXT;
    else
	style = GTK_TOOLBAR_ICONS;

    gtk_toolbar_set_style(toolbar, style);
# if !GTK_CHECK_VERSION(3,0,0)
    gtk_toolbar_set_tooltips(toolbar, (toolbar_flags & TOOLBAR_TOOLTIPS) != 0);
# endif

    switch (tbis_flags)
    {
	case TBIS_TINY:	    size = GTK_ICON_SIZE_MENU;		break;
	case TBIS_SMALL:    size = GTK_ICON_SIZE_SMALL_TOOLBAR;	break;
	case TBIS_MEDIUM:   size = GTK_ICON_SIZE_BUTTON;	break;
	case TBIS_LARGE:    size = GTK_ICON_SIZE_LARGE_TOOLBAR;	break;
	case TBIS_HUGE:     size = GTK_ICON_SIZE_DND;		break;
	case TBIS_GIANT:    size = GTK_ICON_SIZE_DIALOG;	break;
	default:	    size = GTK_ICON_SIZE_INVALID;	break;
    }
    oldsize = gtk_toolbar_get_icon_size(toolbar);

    if (size == GTK_ICON_SIZE_INVALID)
    {
	// Let global user preferences decide the icon size.
	gtk_toolbar_unset_icon_size(toolbar);
	size = gtk_toolbar_get_icon_size(toolbar);
    }
    if (size != oldsize)
    {
	gtk_container_foreach(GTK_CONTAINER(toolbar),
			      &icon_size_changed_foreach,
			      GINT_TO_POINTER((int)size));
    }
    gtk_toolbar_set_icon_size(toolbar, size);
}

#endif // FEAT_TOOLBAR

#if defined(FEAT_GUI_TABLINE) || defined(PROTO)
static int ignore_tabline_evt = FALSE;
static GtkWidget *tabline_menu;
# if !GTK_CHECK_VERSION(3,0,0)
static GtkTooltips *tabline_tooltip;
# endif
static int clicked_page;	    // page clicked in tab line

/*
 * Handle selecting an item in the tab line popup menu.
 */
    static void
tabline_menu_handler(GtkMenuItem *item UNUSED, gpointer user_data)
{
    // Add the string cmd into input buffer
    send_tabline_menu_event(clicked_page, (int)(long)user_data);
}

    static void
add_tabline_menu_item(GtkWidget *menu, char_u *text, int resp)
{
    GtkWidget	*item;
    char_u	*utf_text;

    utf_text = CONVERT_TO_UTF8(text);
    item = gtk_menu_item_new_with_label((const char *)utf_text);
    gtk_widget_show(item);
    CONVERT_TO_UTF8_FREE(utf_text);

    gtk_container_add(GTK_CONTAINER(menu), item);
    g_signal_connect(G_OBJECT(item), "activate",
	    G_CALLBACK(tabline_menu_handler),
	    GINT_TO_POINTER(resp));
}

/*
 * Create a menu for the tab line.
 */
    static GtkWidget *
create_tabline_menu(void)
{
    GtkWidget *menu;

    menu = gtk_menu_new();
    add_tabline_menu_item(menu, (char_u *)_("Close tab"), TABLINE_MENU_CLOSE);
    add_tabline_menu_item(menu, (char_u *)_("New tab"), TABLINE_MENU_NEW);
    add_tabline_menu_item(menu, (char_u *)_("Open Tab..."), TABLINE_MENU_OPEN);

    return menu;
}

    static gboolean
on_tabline_menu(GtkWidget *widget, GdkEvent *event)
{
    // Was this button press event ?
    if (event->type == GDK_BUTTON_PRESS)
    {
	GdkEventButton *bevent = (GdkEventButton *)event;
	int		x = bevent->x;
	int		y = bevent->y;
	GtkWidget	*tabwidget;
	GdkWindow	*tabwin;

	// When ignoring events return TRUE so that the selected page doesn't
	// change.
	if (hold_gui_events || cmdwin_type != 0)
	    return TRUE;

	tabwin = gui_gtk_window_at_position(gui.mainwin, &x, &y);

	gdk_window_get_user_data(tabwin, (gpointer)&tabwidget);
	clicked_page = GPOINTER_TO_INT(g_object_get_data(G_OBJECT(tabwidget),
							 "tab_num"));

	// If the event was generated for 3rd button popup the menu.
	if (bevent->button == 3)
	{
# if GTK_CHECK_VERSION(3,22,2)
	    gtk_menu_popup_at_pointer(GTK_MENU(widget), event);
# else
	    gtk_menu_popup(GTK_MENU(widget), NULL, NULL, NULL, NULL,
						bevent->button, bevent->time);
# endif
	    // We handled the event.
	    return TRUE;
	}
	else if (bevent->button == 1)
	{
	    if (clicked_page == 0)
	    {
		// Click after all tabs moves to next tab page.  When "x" is
		// small guess it's the left button.
		send_tabline_event(x < 50 ? -1 : 0);
	    }
	}
	else if (bevent->button == 2)
	{
	    if (clicked_page != 0)
		// Middle mouse click on tabpage label closes that tab.
		send_tabline_menu_event(clicked_page, TABLINE_MENU_CLOSE);
	}
    }

    // We didn't handle the event.
    return FALSE;
}

/*
 * Handle selecting one of the tabs.
 */
    static void
on_select_tab(
	GtkNotebook	*notebook UNUSED,
	gpointer	*page UNUSED,
	gint		idx,
	gpointer	data UNUSED)
{
    if (!ignore_tabline_evt)
	send_tabline_event(idx + 1);
}

# if GTK_CHECK_VERSION(2,10,0)
/*
 * Handle reordering the tabs (using D&D).
 */
    static void
on_tab_reordered(
	GtkNotebook	*notebook UNUSED,
	gpointer	*page UNUSED,
	gint		idx,
	gpointer	data UNUSED)
{
    if (ignore_tabline_evt)
	return;

    if ((tabpage_index(curtab) - 1) < idx)
	tabpage_move(idx + 1);
    else
	tabpage_move(idx);
}
# endif

/*
 * Show or hide the tabline.
 */
    void
gui_mch_show_tabline(int showit)
{
    if (gui.tabline == NULL)
	return;

    if (!showit != !gtk_notebook_get_show_tabs(GTK_NOTEBOOK(gui.tabline)))
    {
	// Note: this may cause a resize event
	gtk_notebook_set_show_tabs(GTK_NOTEBOOK(gui.tabline), showit);
	update_window_manager_hints(0, 0);
	if (showit)
	    gtk_widget_set_can_focus(GTK_WIDGET(gui.tabline), FALSE);
    }

    gui_mch_update();
}

/*
 * Return TRUE when tabline is displayed.
 */
    int
gui_mch_showing_tabline(void)
{
    return gui.tabline != NULL
		     && gtk_notebook_get_show_tabs(GTK_NOTEBOOK(gui.tabline));
}

/*
 * Update the labels of the tabline.
 */
    void
gui_mch_update_tabline(void)
{
    GtkWidget	    *page;
    GtkWidget	    *event_box;
    GtkWidget	    *label;
    tabpage_T	    *tp;
    int		    nr = 0;
    int		    tab_num;
    int		    curtabidx = 0;
    char_u	    *labeltext;

    if (gui.tabline == NULL)
	return;

    ignore_tabline_evt = TRUE;

    // Add a label for each tab page.  They all contain the same text area.
    for (tp = first_tabpage; tp != NULL; tp = tp->tp_next, ++nr)
    {
	if (tp == curtab)
	    curtabidx = nr;

	tab_num = nr + 1;

	page = gtk_notebook_get_nth_page(GTK_NOTEBOOK(gui.tabline), nr);
	if (page == NULL)
	{
	    // Add notebook page
# if GTK_CHECK_VERSION(3,2,0)
	    page = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
	    gtk_box_set_homogeneous(GTK_BOX(page), FALSE);
# else
	    page = gtk_vbox_new(FALSE, 0);
# endif
	    gtk_widget_show(page);
	    event_box = gtk_event_box_new();
	    gtk_widget_show(event_box);
	    label = gtk_label_new("-Empty-");
# if !GTK_CHECK_VERSION(3,14,0)
	    gtk_misc_set_padding(GTK_MISC(label), 2, 2);
# endif
	    gtk_container_add(GTK_CONTAINER(event_box), label);
	    gtk_widget_show(label);
	    gtk_notebook_insert_page(GTK_NOTEBOOK(gui.tabline),
		    page,
		    event_box,
		    nr++);
# if GTK_CHECK_VERSION(2,10,0)
	    gtk_notebook_set_tab_reorderable(GTK_NOTEBOOK(gui.tabline),
		    page,
		    TRUE);
# endif
	}

	event_box = gtk_notebook_get_tab_label(GTK_NOTEBOOK(gui.tabline), page);
	g_object_set_data(G_OBJECT(event_box), "tab_num",
						     GINT_TO_POINTER(tab_num));
	label = gtk_bin_get_child(GTK_BIN(event_box));
	get_tabline_label(tp, FALSE);
	labeltext = CONVERT_TO_UTF8(NameBuff);
	gtk_label_set_text(GTK_LABEL(label), (const char *)labeltext);
	CONVERT_TO_UTF8_FREE(labeltext);

	get_tabline_label(tp, TRUE);
	labeltext = CONVERT_TO_UTF8(NameBuff);
# if GTK_CHECK_VERSION(3,0,0)
	gtk_widget_set_tooltip_text(event_box, (const gchar *)labeltext);
# else
	gtk_tooltips_set_tip(GTK_TOOLTIPS(tabline_tooltip), event_box,
			     (const char *)labeltext, NULL);
# endif
	CONVERT_TO_UTF8_FREE(labeltext);
    }

    // Remove any old labels.
    while (gtk_notebook_get_nth_page(GTK_NOTEBOOK(gui.tabline), nr) != NULL)
	gtk_notebook_remove_page(GTK_NOTEBOOK(gui.tabline), nr);

    if (gtk_notebook_get_current_page(GTK_NOTEBOOK(gui.tabline)) != curtabidx)
	gtk_notebook_set_current_page(GTK_NOTEBOOK(gui.tabline), curtabidx);

    // Make sure everything is in place before drawing text.
    gui_mch_update();

    ignore_tabline_evt = FALSE;
}

/*
 * Set the current tab to "nr".  First tab is 1.
 */
    void
gui_mch_set_curtab(int nr)
{
    if (gui.tabline == NULL)
	return;

    ignore_tabline_evt = TRUE;
    if (gtk_notebook_get_current_page(GTK_NOTEBOOK(gui.tabline)) != nr - 1)
	gtk_notebook_set_current_page(GTK_NOTEBOOK(gui.tabline), nr - 1);
    ignore_tabline_evt = FALSE;
}

#endif // FEAT_GUI_TABLINE

/*
 * Add selection targets for PRIMARY and CLIPBOARD selections.
 */
    void
gui_gtk_set_selection_targets(GdkAtom selection)
{
    int		    i, j = 0;
    static int	    n_targets = N_SELECTION_TARGETS;
    static GtkTargetEntry  targets[N_SELECTION_TARGETS];

    if (targets[0].target == NULL)
    {
	for (i = 0; i < (int)N_SELECTION_TARGETS; ++i)
	{
	    // OpenOffice tries to use TARGET_HTML and fails when we don't
	    // return something, instead of trying another target. Therefore only
	    // offer TARGET_HTML when it works.
	    if (!clip_html && selection_targets[i].info == TARGET_HTML)
		n_targets--;
	    else
		targets[j++] = selection_targets[i];
	}
    }

    gtk_selection_clear_targets(gui.drawarea, selection);
    gtk_selection_add_targets(gui.drawarea, selection,
			      targets, n_targets);
}

/*
 * Set up for receiving DND items.
 */
    void
gui_gtk_set_dnd_targets(void)
{
    int		    i, j = 0;
    int		    n_targets = N_DND_TARGETS;
    GtkTargetEntry  targets[N_DND_TARGETS];

    for (i = 0; i < (int)N_DND_TARGETS; ++i)
    {
	if (!clip_html && dnd_targets[i].info == TARGET_HTML)
	    n_targets--;
	else
	    targets[j++] = dnd_targets[i];
    }

    gtk_drag_dest_unset(gui.drawarea);
    gtk_drag_dest_set(gui.drawarea,
		      GTK_DEST_DEFAULT_ALL,
		      targets, n_targets,
		      GDK_ACTION_COPY | GDK_ACTION_MOVE);
}

/*
 * Initialize the GUI.	Create all the windows, set up all the callbacks etc.
 * Returns OK for success, FAIL when the GUI can't be started.
 */
    int
gui_mch_init(void)
{
    GtkWidget *vbox;

#ifdef FEAT_GUI_GNOME
    // Initialize the GNOME libraries.	gnome_program_init()/gnome_init()
    // exits on failure, but that's a non-issue because we already called
    // gtk_init_check() in gui_mch_init_check().
    if (using_gnome)
    {
	gnome_program_init(VIMPACKAGE, VIM_VERSION_SHORT,
			   LIBGNOMEUI_MODULE, gui_argc, gui_argv, NULL);
# if defined(LC_NUMERIC)
	{
	    char *p = setlocale(LC_NUMERIC, NULL);

	    // Make sure strtod() uses a decimal point, not a comma. Gnome
	    // init may change it.
	    if (p == NULL || strcmp(p, "C") != 0)
	       setlocale(LC_NUMERIC, "C");
	}
# endif
    }
#endif
    VIM_CLEAR(gui_argv);

#if GLIB_CHECK_VERSION(2,1,3)
    // Set the human-readable application name
    g_set_application_name("Vim");
#endif
    /*
     * Force UTF-8 output no matter what the value of 'encoding' is.
     * did_set_string_option() in option.c prohibits changing 'termencoding'
     * to something else than UTF-8 if the GUI is in use.
     */
    set_option_value_give_err((char_u *)"termencoding",
						     0L, (char_u *)"utf-8", 0);

#ifdef FEAT_TOOLBAR
    gui_gtk_register_stock_icons();
#endif
    // FIXME: Need to install the classic icons and a gtkrc.classic file.
    // The hard part is deciding install locations and the Makefile magic.
#if !GTK_CHECK_VERSION(3,0,0)
# if 0
    gtk_rc_parse("gtkrc");
# endif
#endif

    // Initialize values
    gui.border_width = 2;
    gui.scrollbar_width = SB_DEFAULT_WIDTH;
    gui.scrollbar_height = SB_DEFAULT_WIDTH;
#if GTK_CHECK_VERSION(3,0,0)
    gui.fgcolor = g_new(GdkRGBA, 1);
    gui.bgcolor = g_new(GdkRGBA, 1);
    gui.spcolor = g_new(GdkRGBA, 1);
#else
    // LINTED: avoid warning: conversion to 'unsigned long'
    gui.fgcolor = g_new0(GdkColor, 1);
    // LINTED: avoid warning: conversion to 'unsigned long'
    gui.bgcolor = g_new0(GdkColor, 1);
    // LINTED: avoid warning: conversion to 'unsigned long'
    gui.spcolor = g_new0(GdkColor, 1);
#endif

    // Initialise atoms
    html_atom = gdk_atom_intern("text/html", FALSE);
    utf8_string_atom = gdk_atom_intern("UTF8_STRING", FALSE);

    // Set default foreground and background colors.
    gui.norm_pixel = gui.def_norm_pixel;
    gui.back_pixel = gui.def_back_pixel;

    if (gtk_socket_id != 0)
    {
	GtkWidget *plug;

	// Use GtkSocket from another app.
	plug = gtk_plug_new_for_display(gdk_display_get_default(),
					gtk_socket_id);
	if (plug != NULL && gtk_plug_get_socket_window(GTK_PLUG(plug)) != NULL)
	{
	    gui.mainwin = plug;
	}
	else
	{
	    g_warning("Connection to GTK+ socket (ID %u) failed",
		      (unsigned int)gtk_socket_id);
	    // Pretend we never wanted it if it failed (get own window)
	    gtk_socket_id = 0;
	}
    }

    if (gtk_socket_id == 0)
    {
#ifdef FEAT_GUI_GNOME
	if (using_gnome)
	{
	    gui.mainwin = gnome_app_new("Vim", NULL);
# ifdef USE_XSMP
	    // Use the GNOME save-yourself functionality now.
	    xsmp_close();
# endif
	}
	else
#endif
	    gui.mainwin = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    }

    gtk_widget_set_name(gui.mainwin, "vim-main-window");

    // Create the PangoContext used for drawing all text.
    gui.text_context = gtk_widget_create_pango_context(gui.mainwin);
    pango_context_set_base_dir(gui.text_context, PANGO_DIRECTION_LTR);

    gtk_container_set_border_width(GTK_CONTAINER(gui.mainwin), 0);
    gtk_widget_add_events(gui.mainwin, GDK_VISIBILITY_NOTIFY_MASK);

    g_signal_connect(G_OBJECT(gui.mainwin), "delete-event",
		     G_CALLBACK(&delete_event_cb), NULL);

    g_signal_connect(G_OBJECT(gui.mainwin), "realize",
		     G_CALLBACK(&mainwin_realize), NULL);

    g_signal_connect(G_OBJECT(gui.mainwin), "screen-changed",
		     G_CALLBACK(&mainwin_screen_changed_cb), NULL);

    gui.accel_group = gtk_accel_group_new();
    gtk_window_add_accel_group(GTK_WINDOW(gui.mainwin), gui.accel_group);

    // A vertical box holds the menubar, toolbar and main text window.
#if GTK_CHECK_VERSION(3,2,0)
    vbox = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
    gtk_box_set_homogeneous(GTK_BOX(vbox), FALSE);
#else
    vbox = gtk_vbox_new(FALSE, 0);
#endif

#ifdef FEAT_GUI_GNOME
    if (using_gnome)
    {
# if defined(FEAT_MENU)
	// automagically restore menubar/toolbar placement
	gnome_app_enable_layout_config(GNOME_APP(gui.mainwin), TRUE);
# endif
	gnome_app_set_contents(GNOME_APP(gui.mainwin), vbox);
    }
    else
#endif
    {
	gtk_container_add(GTK_CONTAINER(gui.mainwin), vbox);
	gtk_widget_show(vbox);
    }

#ifdef FEAT_MENU
    /*
     * Create the menubar and handle
     */
    gui.menubar = gtk_menu_bar_new();
    gtk_widget_set_name(gui.menubar, "vim-menubar");

    // Avoid that GTK takes <F10> away from us.
    {
	GtkSettings *gtk_settings;

	gtk_settings = gtk_settings_get_for_screen(gdk_screen_get_default());
	g_object_set(gtk_settings, "gtk-menu-bar-accel", NULL, NULL);
    }


# ifdef FEAT_GUI_GNOME
    if (using_gnome)
    {
	BonoboDockItem *dockitem;

	gnome_app_set_menus(GNOME_APP(gui.mainwin), GTK_MENU_BAR(gui.menubar));
	dockitem = gnome_app_get_dock_item_by_name(GNOME_APP(gui.mainwin),
						   GNOME_APP_MENUBAR_NAME);
	// We don't want the menu to float.
	bonobo_dock_item_set_behavior(dockitem,
		bonobo_dock_item_get_behavior(dockitem)
				       | BONOBO_DOCK_ITEM_BEH_NEVER_FLOATING);
	gui.menubar_h = GTK_WIDGET(dockitem);
    }
    else
# endif	// FEAT_GUI_GNOME
    {
	// Always show the menubar, otherwise <F10> doesn't work.  It may be
	// disabled in gui_init() later.
	gtk_widget_show(gui.menubar);
	gtk_box_pack_start(GTK_BOX(vbox), gui.menubar, FALSE, FALSE, 0);
    }
#endif	// FEAT_MENU

#ifdef FEAT_TOOLBAR
    /*
     * Create the toolbar and handle
     */
    // some aesthetics on the toolbar
# ifdef USE_GTK3
    // TODO: Add GTK+ 3 code here using GtkCssProvider if necessary.
    // N.B.  Since the default value of GtkToolbar::button-relief is
    // GTK_RELIEF_NONE, there's no need to specify that, probably.
# else
    gtk_rc_parse_string(
	    "style \"vim-toolbar-style\" {\n"
	    "  GtkToolbar::button_relief = GTK_RELIEF_NONE\n"
	    "}\n"
	    "widget \"*.vim-toolbar\" style \"vim-toolbar-style\"\n");
# endif
    gui.toolbar = gtk_toolbar_new();
    gtk_widget_set_name(gui.toolbar, "vim-toolbar");
    set_toolbar_style(GTK_TOOLBAR(gui.toolbar));

# ifdef FEAT_GUI_GNOME
    if (using_gnome)
    {
	BonoboDockItem *dockitem;

	gnome_app_set_toolbar(GNOME_APP(gui.mainwin), GTK_TOOLBAR(gui.toolbar));
	dockitem = gnome_app_get_dock_item_by_name(GNOME_APP(gui.mainwin),
						   GNOME_APP_TOOLBAR_NAME);
	gui.toolbar_h = GTK_WIDGET(dockitem);
	// When the toolbar is floating it gets stuck.  So long as that isn't
	// fixed let's disallow floating.
	bonobo_dock_item_set_behavior(dockitem,
		bonobo_dock_item_get_behavior(dockitem)
				       | BONOBO_DOCK_ITEM_BEH_NEVER_FLOATING);
	gtk_container_set_border_width(GTK_CONTAINER(gui.toolbar), 0);
    }
    else
# endif	// FEAT_GUI_GNOME
    {
	if (vim_strchr(p_go, GO_TOOLBAR) != NULL
		&& (toolbar_flags & (TOOLBAR_TEXT | TOOLBAR_ICONS)))
	    gtk_widget_show(gui.toolbar);
	gtk_box_pack_start(GTK_BOX(vbox), gui.toolbar, FALSE, FALSE, 0);
    }
#endif // FEAT_TOOLBAR

#ifdef FEAT_GUI_TABLINE
    /*
     * Use a Notebook for the tab pages labels.  The labels are hidden by
     * default.
     */
    gui.tabline = gtk_notebook_new();
    gtk_widget_show(gui.tabline);
    gtk_box_pack_start(GTK_BOX(vbox), gui.tabline, FALSE, FALSE, 0);
    gtk_notebook_set_show_border(GTK_NOTEBOOK(gui.tabline), FALSE);
    gtk_notebook_set_show_tabs(GTK_NOTEBOOK(gui.tabline), FALSE);
    gtk_notebook_set_scrollable(GTK_NOTEBOOK(gui.tabline), TRUE);
# if !GTK_CHECK_VERSION(3,0,0)
    gtk_notebook_set_tab_border(GTK_NOTEBOOK(gui.tabline), FALSE);
# endif

# if !GTK_CHECK_VERSION(3,0,0)
    tabline_tooltip = gtk_tooltips_new();
    gtk_tooltips_enable(GTK_TOOLTIPS(tabline_tooltip));
# endif

    {
	GtkWidget *page, *label, *event_box;

	// Add the first tab.
# if GTK_CHECK_VERSION(3,2,0)
	page = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
	gtk_box_set_homogeneous(GTK_BOX(page), FALSE);
# else
	page = gtk_vbox_new(FALSE, 0);
# endif
	gtk_widget_show(page);
	gtk_container_add(GTK_CONTAINER(gui.tabline), page);
	label = gtk_label_new("-Empty-");
	gtk_widget_show(label);
	event_box = gtk_event_box_new();
	gtk_widget_show(event_box);
	g_object_set_data(G_OBJECT(event_box), "tab_num", GINT_TO_POINTER(1L));
# if !GTK_CHECK_VERSION(3,14,0)
	gtk_misc_set_padding(GTK_MISC(label), 2, 2);
# endif
	gtk_container_add(GTK_CONTAINER(event_box), label);
	gtk_notebook_set_tab_label(GTK_NOTEBOOK(gui.tabline), page, event_box);
# if GTK_CHECK_VERSION(2,10,0)
	gtk_notebook_set_tab_reorderable(GTK_NOTEBOOK(gui.tabline), page, TRUE);
# endif
    }

    g_signal_connect(G_OBJECT(gui.tabline), "switch-page",
		     G_CALLBACK(on_select_tab), NULL);
# if GTK_CHECK_VERSION(2,10,0)
    g_signal_connect(G_OBJECT(gui.tabline), "page-reordered",
		     G_CALLBACK(on_tab_reordered), NULL);
# endif

    // Create a popup menu for the tab line and connect it.
    tabline_menu = create_tabline_menu();
    g_signal_connect_swapped(G_OBJECT(gui.tabline), "button-press-event",
	    G_CALLBACK(on_tabline_menu), G_OBJECT(tabline_menu));
#endif // FEAT_GUI_TABLINE

    gui.formwin = gui_gtk_form_new();
    gtk_container_set_border_width(GTK_CONTAINER(gui.formwin), 0);
#if !GTK_CHECK_VERSION(3,0,0)
    gtk_widget_set_events(gui.formwin, GDK_EXPOSURE_MASK);
#endif
#if GTK_CHECK_VERSION(3,22,2)
    gtk_widget_set_name(gui.formwin, "vim-gtk-form");
#endif

    gui.drawarea = gtk_drawing_area_new();
#if GTK_CHECK_VERSION(3,0,0)
    gui.surface = NULL;
#endif

    // Determine which events we will filter.
    gtk_widget_set_events(gui.drawarea,
			  GDK_EXPOSURE_MASK |
			  GDK_ENTER_NOTIFY_MASK |
			  GDK_LEAVE_NOTIFY_MASK |
			  GDK_BUTTON_PRESS_MASK |
			  GDK_BUTTON_RELEASE_MASK |
			  GDK_SCROLL_MASK |
#if GTK_CHECK_VERSION(3,4,0)
			  GDK_SMOOTH_SCROLL_MASK |
#endif
			  GDK_KEY_PRESS_MASK |
			  GDK_KEY_RELEASE_MASK |
			  GDK_POINTER_MOTION_MASK |
			  GDK_POINTER_MOTION_HINT_MASK);

    gtk_widget_show(gui.drawarea);
    gui_gtk_form_put(GTK_FORM(gui.formwin), gui.drawarea, 0, 0);
    gtk_widget_show(gui.formwin);
    gtk_box_pack_start(GTK_BOX(vbox), gui.formwin, TRUE, TRUE, 0);

    // For GtkSockets, key-presses must go to the focus widget (drawarea)
    // and not the window.
    g_signal_connect((gtk_socket_id == 0) ? G_OBJECT(gui.mainwin)
					  : G_OBJECT(gui.drawarea),
		       "key-press-event",
		       G_CALLBACK(key_press_event), NULL);
#if defined(FEAT_XIM) || GTK_CHECK_VERSION(3,0,0)
    // Also forward key release events for the benefit of GTK+ 2 input
    // modules.  Try CTRL-SHIFT-xdigits to enter a Unicode code point.
    g_signal_connect((gtk_socket_id == 0) ? G_OBJECT(gui.mainwin)
					  : G_OBJECT(gui.drawarea),
		     "key-release-event",
		     G_CALLBACK(&key_release_event), NULL);
#endif
    g_signal_connect(G_OBJECT(gui.drawarea), "realize",
		     G_CALLBACK(drawarea_realize_cb), NULL);
    g_signal_connect(G_OBJECT(gui.drawarea), "unrealize",
		     G_CALLBACK(drawarea_unrealize_cb), NULL);
#if GTK_CHECK_VERSION(3,0,0)
    g_signal_connect(G_OBJECT(gui.drawarea), "configure-event",
	    G_CALLBACK(drawarea_configure_event_cb), NULL);
#endif
#if GTK_CHECK_VERSION(3,22,2)
    g_signal_connect_after(G_OBJECT(gui.drawarea), "style-updated",
			   G_CALLBACK(&drawarea_style_updated_cb), NULL);
#else
    g_signal_connect_after(G_OBJECT(gui.drawarea), "style-set",
			   G_CALLBACK(&drawarea_style_set_cb), NULL);
#endif

#if !GTK_CHECK_VERSION(3,0,0)
    gui.visibility = GDK_VISIBILITY_UNOBSCURED;
#endif

#if !defined(USE_GNOME_SESSION)
    wm_protocols_atom = gdk_atom_intern("WM_PROTOCOLS", FALSE);
    save_yourself_atom = gdk_atom_intern("WM_SAVE_YOURSELF", FALSE);
#endif

    if (gtk_socket_id != 0)
	// make sure keyboard input can go to the drawarea
	gtk_widget_set_can_focus(gui.drawarea, TRUE);

    /*
     * Set clipboard specific atoms
     */
    vim_atom = gdk_atom_intern(VIM_ATOM_NAME, FALSE);
    vimenc_atom = gdk_atom_intern(VIMENC_ATOM_NAME, FALSE);
    clip_star.gtk_sel_atom = GDK_SELECTION_PRIMARY;
    clip_plus.gtk_sel_atom = gdk_atom_intern("CLIPBOARD", FALSE);

    /*
     * Start out by adding the configured border width into the border offset.
     */
    gui.border_offset = gui.border_width;

#if GTK_CHECK_VERSION(3,0,0)
    g_signal_connect(G_OBJECT(gui.drawarea), "draw",
		     G_CALLBACK(draw_event), NULL);
#else
    gtk_signal_connect(GTK_OBJECT(gui.mainwin), "visibility_notify_event",
		       GTK_SIGNAL_FUNC(visibility_event), NULL);
    gtk_signal_connect(GTK_OBJECT(gui.drawarea), "expose_event",
		       GTK_SIGNAL_FUNC(expose_event), NULL);
#endif

    /*
     * Only install these enter/leave callbacks when 'p' in 'guioptions'.
     * Only needed for some window managers.
     */
    if (vim_strchr(p_go, GO_POINTER) != NULL)
    {
	g_signal_connect(G_OBJECT(gui.drawarea), "leave-notify-event",
			 G_CALLBACK(leave_notify_event), NULL);
	g_signal_connect(G_OBJECT(gui.drawarea), "enter-notify-event",
			 G_CALLBACK(enter_notify_event), NULL);
    }

    // Real windows can get focus ... GtkPlug, being a mere container can't,
    // only its widgets.  Arguably, this could be common code and we do not
    // use the window focus at all, but let's be safe.
    if (gtk_socket_id == 0)
    {
	g_signal_connect(G_OBJECT(gui.mainwin), "focus-out-event",
			 G_CALLBACK(focus_out_event), NULL);
	g_signal_connect(G_OBJECT(gui.mainwin), "focus-in-event",
			 G_CALLBACK(focus_in_event), NULL);
    }
    else
    {
	g_signal_connect(G_OBJECT(gui.drawarea), "focus-out-event",
			 G_CALLBACK(focus_out_event), NULL);
	g_signal_connect(G_OBJECT(gui.drawarea), "focus-in-event",
			 G_CALLBACK(focus_in_event), NULL);
#ifdef FEAT_GUI_TABLINE
	g_signal_connect(G_OBJECT(gui.tabline), "focus-out-event",
			 G_CALLBACK(focus_out_event), NULL);
	g_signal_connect(G_OBJECT(gui.tabline), "focus-in-event",
			 G_CALLBACK(focus_in_event), NULL);
#endif // FEAT_GUI_TABLINE
    }

    g_signal_connect(G_OBJECT(gui.drawarea), "motion-notify-event",
		     G_CALLBACK(motion_notify_event), NULL);
    g_signal_connect(G_OBJECT(gui.drawarea), "button-press-event",
		     G_CALLBACK(button_press_event), NULL);
    g_signal_connect(G_OBJECT(gui.drawarea), "button-release-event",
		     G_CALLBACK(button_release_event), NULL);
    g_signal_connect(G_OBJECT(gui.drawarea), "scroll-event",
		     G_CALLBACK(scroll_event), NULL);

    // Pretend we don't have input focus, we will get an event if we do.
    gui.in_focus = FALSE;

    // Handle changes to the "Xft/DPI" setting.
    {
	GtkSettings *gtk_settings =
			 gtk_settings_get_for_screen(gdk_screen_get_default());

	g_signal_connect(gtk_settings, "notify::gtk-xft-dpi",
			   G_CALLBACK(gtk_settings_xft_dpi_changed_cb), NULL);
    }

    return OK;
}

#if defined(USE_GNOME_SESSION) || defined(PROTO)
/*
 * This is called from gui_start() after a fork() has been done.
 * We have to tell the session manager our new PID.
 */
    void
gui_mch_forked(void)
{
    if (!using_gnome)
	return;

    GnomeClient *client;

    client = gnome_master_client();

    if (client != NULL)
	gnome_client_set_process_id(client, getpid());
}
#endif // USE_GNOME_SESSION

#if GTK_CHECK_VERSION(3,0,0)
    static GdkRGBA
color_to_rgba(guicolor_T color)
{
    GdkRGBA rgba;
    rgba.red   = ((color & 0xff0000) >> 16) / 255.0;
    rgba.green = ((color & 0xff00) >> 8) / 255.0;
    rgba.blue  = ((color & 0xff)) / 255.0;
    rgba.alpha = 1.0;
    return rgba;
}

    static void
set_cairo_source_rgba_from_color(cairo_t *cr, guicolor_T color)
{
    const GdkRGBA rgba = color_to_rgba(color);
    cairo_set_source_rgba(cr, rgba.red, rgba.green, rgba.blue, rgba.alpha);
}
#endif // GTK_CHECK_VERSION(3,0,0)

/*
 * Called when the foreground or background color has been changed.
 * This used to change the graphics contexts directly but we are
 * currently manipulating them where desired.
 */
    void
gui_mch_new_colors(void)
{
    if (gui.drawarea != NULL
#if GTK_CHECK_VERSION(3,22,2)
	    && gui.formwin != NULL
#endif
	    && gtk_widget_get_window(gui.drawarea) != NULL)
    {
#if !GTK_CHECK_VERSION(3,22,2)
	GdkWindow * const da_win = gtk_widget_get_window(gui.drawarea);
#endif
#if GTK_CHECK_VERSION(3,22,2)
	GtkStyleContext * const context =
				     gtk_widget_get_style_context(gui.formwin);
	GtkCssProvider * const provider = gtk_css_provider_new();
	gchar * const css = g_strdup_printf(
		"widget#vim-gtk-form {\n"
		"  background-color: #%.2lx%.2lx%.2lx;\n"
		"}\n",
		 (gui.back_pixel >> 16) & 0xff,
		 (gui.back_pixel >> 8) & 0xff,
		 gui.back_pixel & 0xff);

	gtk_css_provider_load_from_data(provider, css, -1, NULL);
	gtk_style_context_add_provider(context,
		GTK_STYLE_PROVIDER(provider), G_MAXUINT);

	g_free(css);
	g_object_unref(provider);
#elif GTK_CHECK_VERSION(3,4,0) // !GTK_CHECK_VERSION(3,22,2)
	GdkRGBA rgba;

	rgba = color_to_rgba(gui.back_pixel);
	{
	    cairo_pattern_t * const pat = cairo_pattern_create_rgba(
		    rgba.red, rgba.green, rgba.blue, rgba.alpha);
	    if (pat != NULL)
	    {
		gdk_window_set_background_pattern(da_win, pat);
		cairo_pattern_destroy(pat);
	    }
	    else
		gdk_window_set_background_rgba(da_win, &rgba);
	}
#else // !GTK_CHECK_VERSION(3,4,0)
	GdkColor color = { 0, 0, 0, 0 };

	color.pixel = gui.back_pixel;
	gdk_window_set_background(da_win, &color);
#endif // !GTK_CHECK_VERSION(3,22,2)
    }
}

/*
 * This signal informs us about the need to rearrange our sub-widgets.
 */
    static gint
form_configure_event(GtkWidget *widget UNUSED,
		     GdkEventConfigure *event,
		     gpointer data UNUSED)
{
    int	    usable_height = event->height;
#ifdef TRACK_RESIZE_HISTORY
    // Resize requests are made for gui.mainwin;
    // get its dimensions for searching if this event
    // is a response to a vim request.
    int	    w, h;
    gtk_window_get_size(GTK_WINDOW(gui.mainwin), &w, &h);

# ifdef ENABLE_RESIZE_HISTORY_LOG
    ch_log(NULL, "gui_gtk: form_configure_event: (%d, %d) [%d, %d]",
	   w, h, (int)Columns, (int)Rows);
# endif

    // Look through history of recent vim resize requests.
    // If this event matches:
    //	    - "latest resize hist" We're caught up;
    //		clear the history and process this event.
    //		If history is, old to new, 100, 99, 100, 99. If this event is
    //		99 for the stale, it is matched against the current. History
    //		is cleared, we may bounce, but no worse than before.
    //	    - "older/stale hist" If match an unused event in history,
    //		then discard this event, and mark the matching event as used.
    //	    - "no match" Figure it's a user resize event, clear history.
    // NOTE: clear history is default, then all incoming events are processed

    if (!MATCH_WIDTH_HEIGHT(latest_resize_hist, w, h)
					     && match_stale_width_height(w, h))
	// discard stale event
	return TRUE;
    clear_resize_hists();
#endif

#if GTK_CHECK_VERSION(3,22,2) && !GTK_CHECK_VERSION(3,22,4)
    // As of 3.22.2, GdkWindows have started distributing configure events to
    // their "native" children (https://git.gnome.org/browse/gtk+/commit/?h=gtk-3-22&id=12579fe71b3b8f79eb9c1b80e429443bcc437dd0).
    //
    // As can be seen from the implementation of move_native_children() and
    // configure_native_child() in gdkwindow.c, those functions actually
    // propagate configure events to every child, failing to distinguish
    // "native" one from non-native one.
    //
    // Naturally, configure events propagated to here like that are fallacious
    // and, as a matter of fact, they trigger a geometric collapse of
    // gui.formwin.
    //
    // To filter out such fallacious events, check if the given event is the
    // one that was sent out to the right place. Ignore it if not.
    //
    // Follow-up
    // After a few weeks later, the GdkWindow change mentioned above was
    // reverted (https://git.gnome.org/browse/gtk+/commit/?h=gtk-3-22&id=f70039cb9603a02d2369fec4038abf40a1711155).
    // The corresponding official release is 3.22.4.
    if (event->window != gtk_widget_get_window(gui.formwin))
	return TRUE;
#endif

    // When in a GtkPlug, we can't guarantee valid heights (as a round
    // no. of char-heights), so we have to manually sanitise them.
    // Widths seem to sort themselves out, don't ask me why.
    if (gtk_socket_id != 0)
	usable_height -= (gui.char_height - (gui.char_height/2)); // sic.

    gui_gtk_form_freeze(GTK_FORM(gui.formwin));
    gui_resize_shell(event->width, usable_height);
    gui_gtk_form_thaw(GTK_FORM(gui.formwin));

    return TRUE;
}

/*
 * Function called when window already closed.
 * We can't do much more here than to trying to preserve what had been done,
 * since the window is already inevitably going away.
 */
    static void
mainwin_destroy_cb(GObject *object UNUSED, gpointer data UNUSED)
{
    // Don't write messages to the GUI anymore
    full_screen = FALSE;

    gui.mainwin  = NULL;
    gui.drawarea = NULL;

    if (!exiting) // only do anything if the destroy was unexpected
    {
	vim_strncpy(IObuff,
		(char_u *)_("Vim: Main window unexpectedly destroyed\n"),
		IOSIZE - 1);
	preserve_exit();
    }
#ifdef USE_GRESOURCE
    gui_gtk_unregister_resource();
#endif
}

    void
gui_gtk_get_screen_geom_of_win(
	GtkWidget *wid,
	int point_x,	    // x position of window if not initialized
	int point_y,	    // y position of window if not initialized
	int *screen_x,
	int *screen_y,
	int *width,
	int *height)
{
    GdkRectangle geometry;
    GdkWindow *win = gtk_widget_get_window(wid);
#if GTK_CHECK_VERSION(3,22,0)
    GdkDisplay *dpy;
    GdkMonitor *monitor;

    if (wid != NULL && gtk_widget_get_realized(wid))
	dpy = gtk_widget_get_display(wid);
    else
	dpy = gdk_display_get_default();
    if (win != NULL)
	monitor = gdk_display_get_monitor_at_window(dpy, win);
    else
	monitor = gdk_display_get_monitor_at_point(dpy, point_x, point_y);
    gdk_monitor_get_geometry(monitor, &geometry);
#else
    GdkScreen* screen;
    int monitor;

    if (wid != NULL && gtk_widget_has_screen(wid))
	screen = gtk_widget_get_screen(wid);
    else
	screen = gdk_screen_get_default();
    if (win != NULL)
	monitor = gdk_screen_get_monitor_at_window(screen, win);
    else
	monitor = gdk_screen_get_monitor_at_point(screen, point_x, point_y);
    gdk_screen_get_monitor_geometry(screen, monitor, &geometry);
#endif
    *screen_x = geometry.x;
    *screen_y = geometry.y;
    *width = geometry.width;
    *height = geometry.height;
}

/*
 * The screen size is used to make sure the initial window doesn't get bigger
 * than the screen.  This subtracts some room for menubar, toolbar and window
 * decorations.
 */
    static void
gui_gtk_get_screen_dimensions(
	int point_x,
	int point_y,
	int *screen_w,
	int *screen_h)
{
    int	    x, y;

    gui_gtk_get_screen_geom_of_win(gui.mainwin, point_x, point_y,
						   &x, &y, screen_w, screen_h);

    // Subtract 'guiheadroom' from the height to allow some room for the
    // window manager (task list and window title bar).
    *screen_h -= p_ghr;

    /*
     * FIXME: dirty trick: Because the gui_get_base_height() doesn't include
     * the toolbar and menubar for GTK, we subtract them from the screen
     * height, so that the window size can be made to fit on the screen.
     * This should be completely changed later.
     */
    *screen_w -= get_menu_tool_width();
    *screen_h -= get_menu_tool_height();
}

    void
gui_mch_get_screen_dimensions(int *screen_w, int *screen_h)
{
    gui_gtk_get_screen_dimensions(0, 0, screen_w, screen_h);
}


/*
 * Bit of a hack to ensure we start GtkPlug windows with the correct window
 * hints (and thus the required size from -geom), but that after that we
 * put the hints back to normal (the actual minimum size) so we may
 * subsequently be resized smaller.  GtkSocket (the parent end) uses the
 * plug's window 'min hints to set *its* minimum size, but that's also the
 * only way we have of making ourselves bigger (by set lines/columns).
 * Thus set hints at start-up to ensure correct init. size, then a
 * second after the final attempt to reset the real minimum hints (done by
 * scrollbar init.), actually do the standard hints and stop the timer.
 * We'll not let the default hints be set while this timer's active.
 */
    static timeout_cb_type
check_startup_plug_hints(gpointer data UNUSED)
{
    if (init_window_hints_state == 1)
    {
	// Safe to use normal hints now
	init_window_hints_state = 0;
	update_window_manager_hints(0, 0);
	return FALSE;   // stop timer
    }

    // Keep on trying
    init_window_hints_state = 1;
    return TRUE;
}

/*
 * Open the GUI window which was created by a call to gui_mch_init().
 */
    int
gui_mch_open(void)
{
    guicolor_T fg_pixel = INVALCOLOR;
    guicolor_T bg_pixel = INVALCOLOR;
    guint		pixel_width;
    guint		pixel_height;

    /*
     * Allow setting a window role on the command line, or invent one
     * if none was specified.  This is mainly useful for GNOME session
     * support; allowing the WM to restore window placement.
     */
    if (role_argument != NULL)
    {
	gtk_window_set_role(GTK_WINDOW(gui.mainwin), role_argument);
    }
    else
    {
	char *role;

	// Invent a unique-enough ID string for the role
	role = g_strdup_printf("vim-%u-%u-%u",
			       (unsigned)mch_get_pid(),
			       (unsigned)g_random_int(),
			       (unsigned)time(NULL));

	gtk_window_set_role(GTK_WINDOW(gui.mainwin), role);
	g_free(role);
    }

    if (gui_win_x != -1 && gui_win_y != -1)
	gtk_window_move(GTK_WINDOW(gui.mainwin), gui_win_x, gui_win_y);

    // Determine user specified geometry, if present.
    if (gui.geom != NULL)
    {
	int		mask;
	unsigned int	w, h;
	int		x = 0;
	int		y = 0;

	mask = XParseGeometry((char *)gui.geom, &x, &y, &w, &h);

	if (mask & WidthValue)
	    Columns = w;
	if (mask & HeightValue)
	{
	    if (p_window > (long)h - 1 || !option_was_set((char_u *)"window"))
		p_window = h - 1;
	    Rows = h;
	}
	limit_screen_size();

	pixel_width = (guint)(gui_get_base_width() + Columns * gui.char_width);
	pixel_height = (guint)(gui_get_base_height() + Rows * gui.char_height);

	pixel_width  += get_menu_tool_width();
	pixel_height += get_menu_tool_height();

	if (mask & (XValue | YValue))
	{
	    int ww, hh;

#ifdef FEAT_GUI_GTK
	    gui_gtk_get_screen_dimensions(x, y, &ww, &hh);
#else
	    gui_mch_get_screen_dimensions(&ww, &hh);
#endif
	    hh += p_ghr + get_menu_tool_height();
	    ww += get_menu_tool_width();
	    if (mask & XNegative)
		x += ww - pixel_width;
	    if (mask & YNegative)
		y += hh - pixel_height;
	    gtk_window_move(GTK_WINDOW(gui.mainwin), x, y);
	}
	VIM_CLEAR(gui.geom);

	// From now until everyone's stopped trying to set the window hints
	// to their correct minimum values, stop them being set as we need
	// them to remain at our required size for the parent GtkSocket to
	// give us the right initial size.
	if (gtk_socket_id != 0  &&  (mask & WidthValue || mask & HeightValue))
	{
	    update_window_manager_hints(pixel_width, pixel_height);
	    init_window_hints_state = 1;
	    timeout_add(1000, check_startup_plug_hints, NULL);
	}
    }

    pixel_width = (guint)(gui_get_base_width() + Columns * gui.char_width);
    pixel_height = (guint)(gui_get_base_height() + Rows * gui.char_height);
    // For GTK2 changing the size of the form widget doesn't cause window
    // resizing.
    if (gtk_socket_id == 0)
	gtk_window_resize(GTK_WINDOW(gui.mainwin), pixel_width, pixel_height);
    update_window_manager_hints(0, 0);

    if (foreground_argument != NULL)
	fg_pixel = gui_get_color((char_u *)foreground_argument);
    if (fg_pixel == INVALCOLOR)
	fg_pixel = gui_get_color((char_u *)"Black");

    if (background_argument != NULL)
	bg_pixel = gui_get_color((char_u *)background_argument);
    if (bg_pixel == INVALCOLOR)
	bg_pixel = gui_get_color((char_u *)"White");

    if (found_reverse_arg)
    {
	gui.def_norm_pixel = bg_pixel;
	gui.def_back_pixel = fg_pixel;
    }
    else
    {
	gui.def_norm_pixel = fg_pixel;
	gui.def_back_pixel = bg_pixel;
    }

    // Get the colors from the "Normal" and "Menu" group (set in syntax.c or
    // in a vimrc file)
    set_normal_colors();

    // Check that none of the colors are the same as the background color
    gui_check_colors();

    // Get the colors for the highlight groups (gui_check_colors() might have
    // changed them).
    highlight_gui_started();	// re-init colors and fonts

    g_signal_connect(G_OBJECT(gui.mainwin), "destroy",
		     G_CALLBACK(mainwin_destroy_cb), NULL);

    /*
     * Notify the fixed area about the need to resize the contents of the
     * gui.formwin, which we use for random positioning of the included
     * components.
     *
     * We connect this signal deferred finally after anything is in place,
     * since this is intended to handle resizements coming from the window
     * manager upon us and should not interfere with what VIM is requesting
     * upon startup.
     */
#ifdef TRACK_RESIZE_HISTORY
    latest_resize_hist = ALLOC_CLEAR_ONE(resize_hist_T);
#endif
    g_signal_connect(G_OBJECT(gui.formwin), "configure-event",
				       G_CALLBACK(form_configure_event), NULL);
#if GTK_CHECK_VERSION(3,10,0)
    g_signal_connect(G_OBJECT(gui.formwin), "notify::scale-factor",
		     G_CALLBACK(scale_factor_event), NULL);
#endif

#ifdef FEAT_DND
    // Set up for receiving DND items.
    gui_gtk_set_dnd_targets();

    g_signal_connect(G_OBJECT(gui.drawarea), "drag-data-received",
				      G_CALLBACK(drag_data_received_cb), NULL);
#endif

	// With GTK+ 2, we need to iconify the window before calling show()
	// to avoid mapping the window for a short time.
	if (found_iconic_arg && gtk_socket_id == 0)
	    gui_mch_iconify();

    {
#if defined(FEAT_GUI_GNOME) && defined(FEAT_MENU)
	unsigned long menu_handler = 0;
# ifdef FEAT_TOOLBAR
	unsigned long tool_handler = 0;
# endif
	/*
	 * Urgh hackish :/  For some reason BonoboDockLayout always forces a
	 * show when restoring the saved layout configuration.	We can't just
	 * hide the widgets again after gtk_widget_show(gui.mainwin) since it's
	 * a toplevel window and thus will be realized immediately.  Instead,
	 * connect signal handlers to hide the widgets just after they've been
	 * marked visible, but before the main window is realized.
	 */
	if (using_gnome && vim_strchr(p_go, GO_MENUS) == NULL)
	    menu_handler = g_signal_connect_after(gui.menubar_h, "show",
						  G_CALLBACK(&gtk_widget_hide),
						  NULL);
# ifdef FEAT_TOOLBAR
	if (using_gnome && vim_strchr(p_go, GO_TOOLBAR) == NULL
		&& (toolbar_flags & (TOOLBAR_TEXT | TOOLBAR_ICONS)))
	    tool_handler = g_signal_connect_after(gui.toolbar_h, "show",
						  G_CALLBACK(&gtk_widget_hide),
						  NULL);
# endif
#endif
	gtk_widget_show(gui.mainwin);

#if defined(FEAT_GUI_GNOME) && defined(FEAT_MENU)
	if (menu_handler != 0)
	    g_signal_handler_disconnect(gui.menubar_h, menu_handler);
# ifdef FEAT_TOOLBAR
	if (tool_handler != 0)
	    g_signal_handler_disconnect(gui.toolbar_h, tool_handler);
# endif
#endif
    }

    /*
     * Add selection handler functions.
     */
    g_signal_connect(G_OBJECT(gui.drawarea), "selection-clear-event",
		     G_CALLBACK(selection_clear_event), NULL);
    g_signal_connect(G_OBJECT(gui.drawarea), "selection-received",
		     G_CALLBACK(selection_received_cb), NULL);

    g_signal_connect(G_OBJECT(gui.drawarea), "selection-get",
		     G_CALLBACK(selection_get_cb), NULL);

    return OK;
}

/*
 * Clean up for when exiting Vim.
 */
    void
gui_mch_exit(int rc UNUSED)
{
    // Clean up, unless we don't want to invoke free().
    if (gui.mainwin != NULL && !really_exiting)
	gtk_widget_destroy(gui.mainwin);
}

/*
 * Get the position of the top left corner of the window.
 */
    int
gui_mch_get_winpos(int *x, int *y)
{
    if (gui_mch_get_display())
    {
	gtk_window_get_position(GTK_WINDOW(gui.mainwin), x, y);
	return OK;
    }
    return FAIL;
}

/*
 * Set the position of the top left corner of the window to the given
 * coordinates.
 */
    void
gui_mch_set_winpos(int x, int y)
{
    gtk_window_move(GTK_WINDOW(gui.mainwin), x, y);
}

#if !GTK_CHECK_VERSION(3,0,0)
# if 0
static int resize_idle_installed = FALSE;
/*
 * Idle handler to force resize.  Used by gui_mch_set_shellsize() to ensure
 * the shell size doesn't exceed the window size, i.e. if the window manager
 * ignored our size request.  Usually this happens if the window is maximized.
 *
 * FIXME: It'd be nice if we could find a little more orthodox solution.
 * See also the remark below in gui_mch_set_shellsize().
 *
 * DISABLED: When doing ":set lines+=1" this function would first invoke
 * gui_resize_shell() with the old size, then the normal callback would
 * report the new size through form_configure_event().  That caused the window
 * layout to be messed up.
 */
    static gboolean
force_shell_resize_idle(gpointer data)
{
    if (gui.mainwin != NULL
	    && GTK_WIDGET_REALIZED(gui.mainwin)
	    && GTK_WIDGET_VISIBLE(gui.mainwin))
    {
	int width;
	int height;

	gtk_window_get_size(GTK_WINDOW(gui.mainwin), &width, &height);

	width  -= get_menu_tool_width();
	height -= get_menu_tool_height();

	gui_resize_shell(width, height);
    }

    resize_idle_installed = FALSE;
    return FALSE; // don't call me again
}
# endif
#endif // !GTK_CHECK_VERSION(3,0,0)

/*
 * Return TRUE if the main window is maximized.
 */
    int
gui_mch_maximized(void)
{
    return (gui.mainwin != NULL && gtk_widget_get_window(gui.mainwin) != NULL
	    && (gdk_window_get_state(gtk_widget_get_window(gui.mainwin))
					       & GDK_WINDOW_STATE_MAXIMIZED));
}

/*
 * Unmaximize the main window
 */
    void
gui_mch_unmaximize(void)
{
    if (gui.mainwin != NULL)
	gtk_window_unmaximize(GTK_WINDOW(gui.mainwin));
}

/*
 * Called when the font changed while the window is maximized or GO_KEEPWINSIZE
 * is set.  Compute the new Rows and Columns.  This is like resizing the
 * window.
 */
    void
gui_mch_newfont(void)
{
    int w, h;

    gtk_window_get_size(GTK_WINDOW(gui.mainwin), &w, &h);
    w -= get_menu_tool_width();
    h -= get_menu_tool_height();
    gui_resize_shell(w, h);
}

/*
 * Set the windows size.
 */
    void
gui_mch_set_shellsize(int width, int height,
		      int min_width UNUSED,  int min_height UNUSED,
		      int base_width UNUSED, int base_height UNUSED,
		      int direction UNUSED)
{
    // give GTK+ a chance to put all widget's into place
    gui_mch_update();

    // this will cause the proper resizement to happen too
    if (gtk_socket_id == 0)
	update_window_manager_hints(0, 0);

    // With GTK+ 2, changing the size of the form widget doesn't resize
    // the window.  So let's do it the other way around and resize the
    // main window instead.
    width  += get_menu_tool_width();
    height += get_menu_tool_height();

#ifdef TRACK_RESIZE_HISTORY
    alloc_resize_hist(width, height); // track the resize request
#endif
    if (gtk_socket_id == 0)
	gtk_window_resize(GTK_WINDOW(gui.mainwin), width, height);
    else
	update_window_manager_hints(width, height);

# if !GTK_CHECK_VERSION(3,0,0)
#  if 0
    if (!resize_idle_installed)
    {
	g_idle_add_full(GDK_PRIORITY_EVENTS + 10,
			&force_shell_resize_idle, NULL, NULL);
	resize_idle_installed = TRUE;
    }
#  endif
# endif // !GTK_CHECK_VERSION(3,0,0)
    /*
     * Wait until all events are processed to prevent a crash because the
     * real size of the drawing area doesn't reflect Vim's internal ideas.
     *
     * This is a bit of a hack, since Vim is a terminal application with a GUI
     * on top, while the GUI expects to be the boss.
     */
    gui_mch_update();
}

    void
gui_mch_settitle(char_u *title, char_u *icon UNUSED)
{
    if (title != NULL && output_conv.vc_type != CONV_NONE)
	title = string_convert(&output_conv, title, NULL);

    gtk_window_set_title(GTK_WINDOW(gui.mainwin), (const char *)title);

    if (output_conv.vc_type != CONV_NONE)
	vim_free(title);
}

#if defined(FEAT_MENU) || defined(PROTO)
    void
gui_mch_enable_menu(int showit)
{
    GtkWidget *widget;

# ifdef FEAT_GUI_GNOME
    if (using_gnome)
	widget = gui.menubar_h;
    else
# endif
	widget = gui.menubar;

    // Do not disable the menu while starting up, otherwise F10 doesn't work.
    if (!showit != !gtk_widget_get_visible(widget) && !gui.starting)
    {
	if (showit)
	    gtk_widget_show(widget);
	else
	    gtk_widget_hide(widget);

	update_window_manager_hints(0, 0);
    }
}
#endif // FEAT_MENU

#if defined(FEAT_TOOLBAR) || defined(PROTO)
    void
gui_mch_show_toolbar(int showit)
{
    GtkWidget *widget;

    if (gui.toolbar == NULL)
	return;

# ifdef FEAT_GUI_GNOME
    if (using_gnome)
	widget = gui.toolbar_h;
    else
# endif
	widget = gui.toolbar;

    if (showit)
	set_toolbar_style(GTK_TOOLBAR(gui.toolbar));

    if (!showit != !gtk_widget_get_visible(widget))
    {
	if (showit)
	    gtk_widget_show(widget);
	else
	    gtk_widget_hide(widget);

	update_window_manager_hints(0, 0);
    }
}
#endif // FEAT_TOOLBAR

/*
 * Check if a given font is a CJK font. This is done in a very crude manner. It
 * just see if U+04E00 for zh and ja and U+AC00 for ko are covered in a given
 * font. Consequently, this function cannot  be used as a general purpose check
 * for CJK-ness for which fontconfig APIs should be used.  This is only used by
 * gui_mch_init_font() to deal with 'CJK fixed width fonts'.
 */
    static int
is_cjk_font(PangoFontDescription *font_desc)
{
    static const char * const cjk_langs[] =
	{"zh_CN", "zh_TW", "zh_HK", "ja", "ko"};

    PangoFont	*font;
    unsigned	i;
    int		is_cjk = FALSE;

    font = pango_context_load_font(gui.text_context, font_desc);

    if (font == NULL)
	return FALSE;

    for (i = 0; !is_cjk && i < G_N_ELEMENTS(cjk_langs); ++i)
    {
	PangoCoverage	*coverage;
	gunichar	uc;

	// Valgrind reports a leak for pango_language_from_string(), but the
	// documentation says "This is owned by Pango and should not be freed".
	coverage = pango_font_get_coverage(
		font, pango_language_from_string(cjk_langs[i]));

	if (coverage != NULL)
	{
	    uc = (cjk_langs[i][0] == 'k') ? 0xAC00 : 0x4E00;
	    is_cjk = (pango_coverage_get(coverage, uc) == PANGO_COVERAGE_EXACT);
#if PANGO_VERSION_CHECK(1, 52, 0)
	    g_object_unref(coverage);
#else
	    pango_coverage_unref(coverage);
#endif
	}
    }

    g_object_unref(font);

    return is_cjk;
}

/*
 * Adjust gui.char_height (after 'linespace' was changed).
 */
    int
gui_mch_adjust_charheight(void)
{
    PangoFontMetrics	*metrics;
    int			ascent;
    int			descent;

    metrics = pango_context_get_metrics(gui.text_context, gui.norm_font,
				pango_context_get_language(gui.text_context));
    ascent  = pango_font_metrics_get_ascent(metrics);
    descent = pango_font_metrics_get_descent(metrics);

    pango_font_metrics_unref(metrics);

    // Round up when the value is more than about 1/16 of a pixel above a whole
    // pixel (12.0624 becomes 12, 12.07 becomes 13).  Then add 'linespace'.
    gui.char_height = (ascent + descent + (PANGO_SCALE * 15) / 16)
						   / PANGO_SCALE + p_linespace;
    // LINTED: avoid warning: bitwise operation on signed value
    gui.char_ascent = PANGO_PIXELS(ascent + p_linespace * PANGO_SCALE / 2);

    // A not-positive value of char_height may crash Vim.  Only happens
    // if 'linespace' is negative (which does make sense sometimes).
    gui.char_ascent = MAX(gui.char_ascent, 0);
    gui.char_height = MAX(gui.char_height, gui.char_ascent + 1);

    return OK;
}

#if GTK_CHECK_VERSION(3,0,0)
// Callback function used in gui_mch_font_dialog()
    static gboolean
font_filter(const PangoFontFamily *family,
	    const PangoFontFace   *face UNUSED,
	    gpointer		   data UNUSED)
{
    return pango_font_family_is_monospace((PangoFontFamily *)family);
}
#endif

/*
 * Put up a font dialog and return the selected font name in allocated memory.
 * "oldval" is the previous value.  Return NULL when cancelled.
 * This should probably go into gui_gtk.c.  Hmm.
 * FIXME:
 * The GTK2 font selection dialog has no filtering API.  So we could either
 * a) implement our own (possibly copying the code from somewhere else) or
 * b) just live with it.
 */
    char_u *
gui_mch_font_dialog(char_u *oldval)
{
    GtkWidget	*dialog;
    int		response;
    char_u	*fontname = NULL;
    char_u	*oldname;

#if GTK_CHECK_VERSION(3,2,0)
    dialog = gtk_font_chooser_dialog_new(NULL, NULL);
    gtk_font_chooser_set_filter_func(GTK_FONT_CHOOSER(dialog), font_filter,
	    NULL, NULL);
#else
    dialog = gtk_font_selection_dialog_new(NULL);
#endif

    gtk_window_set_transient_for(GTK_WINDOW(dialog), GTK_WINDOW(gui.mainwin));
    gtk_window_set_destroy_with_parent(GTK_WINDOW(dialog), TRUE);

    if (oldval != NULL && oldval[0] != NUL)
    {
	if (output_conv.vc_type != CONV_NONE)
	    oldname = string_convert(&output_conv, oldval, NULL);
	else
	    oldname = oldval;

	// Annoying bug in GTK (or Pango): if the font name does not include a
	// size, zero is used.  Use default point size ten.
	if (!vim_isdigit(oldname[STRLEN(oldname) - 1]))
	{
	    char_u	*p = vim_strnsave(oldname, STRLEN(oldname) + 3);

	    if (p != NULL)
	    {
		STRCPY(p + STRLEN(p), " 10");
		if (oldname != oldval)
		    vim_free(oldname);
		oldname = p;
	    }
	}

#if GTK_CHECK_VERSION(3,2,0)
	gtk_font_chooser_set_font(
		GTK_FONT_CHOOSER(dialog), (const gchar *)oldname);
#else
	gtk_font_selection_dialog_set_font_name(
		GTK_FONT_SELECTION_DIALOG(dialog), (const char *)oldname);
#endif

	if (oldname != oldval)
	    vim_free(oldname);
    }
    else
#if GTK_CHECK_VERSION(3,2,0)
	gtk_font_chooser_set_font(
		GTK_FONT_CHOOSER(dialog), DEFAULT_FONT);
#else
	gtk_font_selection_dialog_set_font_name(
		GTK_FONT_SELECTION_DIALOG(dialog), DEFAULT_FONT);
#endif

    response = gtk_dialog_run(GTK_DIALOG(dialog));

    if (response == GTK_RESPONSE_OK)
    {
	char *name;

#if GTK_CHECK_VERSION(3,2,0)
	name = gtk_font_chooser_get_font(GTK_FONT_CHOOSER(dialog));
#else
	name = gtk_font_selection_dialog_get_font_name(
			    GTK_FONT_SELECTION_DIALOG(dialog));
#endif
	if (name != NULL)
	{
	    char_u  *p;

	    // Apparently some font names include a comma, need to escape
	    // that, because in 'guifont' it separates names.
	    p = vim_strsave_escaped((char_u *)name, (char_u *)",");
	    g_free(name);
	    if (p != NULL && input_conv.vc_type != CONV_NONE)
	    {
		fontname = string_convert(&input_conv, p, NULL);
		vim_free(p);
	    }
	    else
		fontname = p;
	}
    }

    if (response != GTK_RESPONSE_NONE)
	gtk_widget_destroy(dialog);

    return fontname;
}

/*
 * Some monospace fonts don't support a bold weight, and fall back
 * silently to the regular weight.  But this is no good since our text
 * drawing function can emulate bold by overstriking.  So let's try
 * to detect whether bold weight is actually available and emulate it
 * otherwise.
 *
 * Note that we don't need to check for italic style since Xft can
 * emulate italic on its own, provided you have a proper fontconfig
 * setup.  We wouldn't be able to emulate it in Vim anyway.
 */
    static void
get_styled_font_variants(void)
{
    PangoFontDescription    *bold_font_desc;
    PangoFont		    *plain_font;
    PangoFont		    *bold_font;

    gui.font_can_bold = FALSE;

    plain_font = pango_context_load_font(gui.text_context, gui.norm_font);

    if (plain_font == NULL)
	return;

    bold_font_desc = pango_font_description_copy_static(gui.norm_font);
    pango_font_description_set_weight(bold_font_desc, PANGO_WEIGHT_BOLD);

    bold_font = pango_context_load_font(gui.text_context, bold_font_desc);
    /*
     * The comparison relies on the unique handle nature of a PangoFont*,
     * i.e. it's assumed that a different PangoFont* won't refer to the
     * same font.  Seems to work, and failing here isn't critical anyway.
     */
    if (bold_font != NULL)
    {
	gui.font_can_bold = (bold_font != plain_font);
	g_object_unref(bold_font);
    }

    pango_font_description_free(bold_font_desc);
    if (bold_font != NULL && gui.font_can_bold)
	g_object_unref(plain_font);
}

static PangoEngineShape *default_shape_engine = NULL;

/*
 * Create a map from ASCII characters in the range [32,126] to glyphs
 * of the current font.  This is used by gui_gtk2_draw_string() to skip
 * the itemize and shaping process for the most common case.
 */
    static void
ascii_glyph_table_init(void)
{
    char_u	    ascii_chars[2 * 128];
    PangoAttrList   *attr_list;
    GList	    *item_list;
    int		    i;

    if (gui.ascii_glyphs != NULL)
	pango_glyph_string_free(gui.ascii_glyphs);
    if (gui.ascii_font != NULL)
	g_object_unref(gui.ascii_font);

    gui.ascii_glyphs = NULL;
    gui.ascii_font   = NULL;

    // For safety, fill in question marks for the control characters.
    // Put a space between characters to avoid shaping.
    for (i = 0; i < 128; ++i)
    {
	if (i >= 32 && i < 127)
	    ascii_chars[2 * i] = i;
	else
	    ascii_chars[2 * i] = '?';
	ascii_chars[2 * i + 1] = ' ';
    }

    attr_list = pango_attr_list_new();
    item_list = pango_itemize(gui.text_context, (const char *)ascii_chars,
			      0, sizeof(ascii_chars), attr_list, NULL);

    if (item_list != NULL && item_list->next == NULL) // play safe
    {
	PangoItem   *item;
	int	    width;

	item  = (PangoItem *)item_list->data;
	width = gui.char_width * PANGO_SCALE;

	// Remember the shape engine used for ASCII.
	default_shape_engine = item->analysis.shape_engine;

	gui.ascii_font = item->analysis.font;
	g_object_ref(gui.ascii_font);

	gui.ascii_glyphs = pango_glyph_string_new();

	pango_shape((const char *)ascii_chars, sizeof(ascii_chars),
		    &item->analysis, gui.ascii_glyphs);

	g_return_if_fail(gui.ascii_glyphs->num_glyphs == sizeof(ascii_chars));

	for (i = 0; i < gui.ascii_glyphs->num_glyphs; ++i)
	{
	    PangoGlyphGeometry *geom;

	    geom = &gui.ascii_glyphs->glyphs[i].geometry;
	    geom->x_offset += MAX(0, width - geom->width) / 2;
	    geom->width = width;
	}
    }

    // TODO: is this type cast OK?
    g_list_foreach(item_list, (GFunc)(void *)&pango_item_free, NULL);
    g_list_free(item_list);
    pango_attr_list_unref(attr_list);
}

/*
 * Initialize Vim to use the font or fontset with the given name.
 * Return FAIL if the font could not be loaded, OK otherwise.
 */
    int
gui_mch_init_font(char_u *font_name, int fontset UNUSED)
{
    PangoFontDescription    *font_desc;
    PangoLayout		    *layout;
    int			    width;

    // If font_name is NULL, this means to use the default, which should
    // be present on all proper Pango/fontconfig installations.
    if (font_name == NULL)
	font_name = (char_u *)DEFAULT_FONT;

    font_desc = gui_mch_get_font(font_name, FALSE);

    if (font_desc == NULL)
	return FAIL;

    gui_mch_free_font(gui.norm_font);
    gui.norm_font = font_desc;

    pango_context_set_font_description(gui.text_context, font_desc);

    layout = pango_layout_new(gui.text_context);
    pango_layout_set_text(layout, "MW", 2);
    pango_layout_get_size(layout, &width, NULL);
    /*
     * Set char_width to half the width obtained from pango_layout_get_size()
     * for CJK fixed_width/bi-width fonts.  An unpatched version of Xft leads
     * Pango to use the same width for both non-CJK characters (e.g. Latin
     * letters and numbers) and CJK characters.  This results in 's p a c e d
     * o u t' rendering when a CJK 'fixed width' font is used. To work around
     * that, divide the width returned by Pango by 2 if cjk_width is equal to
     * width for CJK fonts.
     *
     * For related bugs, see:
     * http://bugzilla.gnome.org/show_bug.cgi?id=106618
     * http://bugzilla.gnome.org/show_bug.cgi?id=106624
     *
     * With this, for all four of the following cases, Vim works fine:
     *	   guifont=CJK_fixed_width_font
     *	   guifont=Non_CJK_fixed_font
     *	   guifont=Non_CJK_fixed_font,CJK_Fixed_font
     *	   guifont=Non_CJK_fixed_font guifontwide=CJK_fixed_font
     */
    if (is_cjk_font(gui.norm_font))
    {
	int cjk_width;

	// Measure the text extent of U+4E00 and U+4E8C
	pango_layout_set_text(layout, "\344\270\200\344\272\214", -1);
	pango_layout_get_size(layout, &cjk_width, NULL);

	if (width == cjk_width)  // Xft not patched
	    width /= 2;
    }
    g_object_unref(layout);

    gui.char_width = (width / 2 + PANGO_SCALE - 1) / PANGO_SCALE;

    // A zero width may cause a crash.	Happens for semi-invalid fontsets.
    if (gui.char_width <= 0)
	gui.char_width = 8;

    gui_mch_adjust_charheight();

    // Set the fontname, which will be used for information purposes
    hl_set_font_name(font_name);

    get_styled_font_variants();
    ascii_glyph_table_init();

    // Avoid unnecessary overhead if 'guifontwide' is equal to 'guifont'.
    if (gui.wide_font != NULL
	&& pango_font_description_equal(gui.norm_font, gui.wide_font))
    {
	pango_font_description_free(gui.wide_font);
	gui.wide_font = NULL;
    }

    if (gui_mch_maximized())
    {
	// Update lines and columns in accordance with the new font, keep the
	// window maximized.
	gui_mch_newfont();
    }
    else
    {
	// Preserve the logical dimensions of the screen.
	update_window_manager_hints(0, 0);
    }

    return OK;
}

/*
 * Get a reference to the font "name".
 * Return zero for failure.
 */
    GuiFont
gui_mch_get_font(char_u *name, int report_error)
{
    PangoFontDescription    *font;

    // can't do this when GUI is not running
    if (!gui.in_use || name == NULL)
	return NULL;

    if (output_conv.vc_type != CONV_NONE)
    {
	char_u *buf;

	buf = string_convert(&output_conv, name, NULL);
	if (buf != NULL)
	{
	    font = pango_font_description_from_string((const char *)buf);
	    vim_free(buf);
	}
	else
	    font = NULL;
    }
    else
	font = pango_font_description_from_string((const char *)name);

    if (font != NULL)
    {
	PangoFont *real_font;

	// pango_context_load_font() bails out if no font size is set
	if (pango_font_description_get_size(font) <= 0)
	    pango_font_description_set_size(font, 10 * PANGO_SCALE);

	real_font = pango_context_load_font(gui.text_context, font);

	if (real_font == NULL)
	{
	    pango_font_description_free(font);
	    font = NULL;
	}
	else
	    g_object_unref(real_font);
    }

    if (font == NULL)
    {
	if (report_error)
	    semsg(_((char *)e_unknown_font_str), name);
	return NULL;
    }

    return font;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return the name of font "font" in allocated memory.
 */
    char_u *
gui_mch_get_fontname(GuiFont font, char_u *name UNUSED)
{
    if (font != NOFONT)
    {
	char	*pangoname = pango_font_description_to_string(font);

	if (pangoname != NULL)
	{
	    char_u	*s = vim_strsave((char_u *)pangoname);

	    g_free(pangoname);
	    return s;
	}
    }
    return NULL;
}
#endif

/*
 * If a font is not going to be used, free its structure.
 */
    void
gui_mch_free_font(GuiFont font)
{
    if (font != NOFONT)
	pango_font_description_free(font);
}

/*
 * Cmdline expansion for setting 'guifont' / 'guifontwide'. Will enumerate
 * through all fonts for completion. When setting 'guifont' it will only show
 * monospace fonts as it's unlikely other fonts would be useful.
 */
    void
gui_mch_expand_font(optexpand_T *args, void *param, int (*add_match)(char_u *val))
{
    PangoFontFamily	**font_families = NULL;
    int			n_families = 0;
    int			wide = *(int *)param;

    if (args->oe_include_orig_val && *args->oe_opt_value == NUL && !wide)
    {
	// If guifont is empty, and we want to fill in the orig value, suggest
	// the default so the user can modify it.
	if (add_match((char_u *)DEFAULT_FONT) != OK)
	    return;
    }

    pango_context_list_families(
	    gui.text_context,
	    &font_families,
	    &n_families);

    for (int i = 0; i < n_families; i++)
    {
	if (!wide && !pango_font_family_is_monospace(font_families[i]))
	    continue;

	const char* fam_name = pango_font_family_get_name(font_families[i]);
	if (input_conv.vc_type != CONV_NONE)
	{
	    char_u *buf = string_convert(&input_conv, (char_u*)fam_name, NULL);
	    if (buf != NULL)
	    {
		if (add_match(buf) != OK)
		{
		    vim_free(buf);
		    break;
		}
		vim_free(buf);
	    }
	    else
		break;
	}
	else
	{
	    if (add_match((char_u *)fam_name) != OK)
		break;
	}
    }

    g_free(font_families);
}

/*
 * Return the Pixel value (color) for the given color name.
 *
 * Return INVALCOLOR for error.
 */
    guicolor_T
gui_mch_get_color(char_u *name)
{
    guicolor_T color = INVALCOLOR;

    if (!gui.in_use)		// can't do this when GUI not running
	return color;

    if (name != NULL)
	color = gui_get_color_cmn(name);

#if GTK_CHECK_VERSION(3,0,0)
    return color;
#else
    if (color == INVALCOLOR)
	return INVALCOLOR;

    return gui_mch_get_rgb_color(
	    (color & 0xff0000) >> 16,
	    (color & 0xff00) >> 8,
	    color & 0xff);
#endif
}

/*
 * Return the Pixel value (color) for the given RGB values.
 * Return INVALCOLOR for error.
 */
    guicolor_T
gui_mch_get_rgb_color(int r, int g, int b)
{
#if GTK_CHECK_VERSION(3,0,0)
    return gui_get_rgb_color_cmn(r, g, b);
#else
    GdkColor gcolor;
    int ret;

    gcolor.red = (guint16)(r / 255.0 * 65535 + 0.5);
    gcolor.green = (guint16)(g / 255.0 * 65535 + 0.5);
    gcolor.blue = (guint16)(b / 255.0 * 65535 + 0.5);

    ret = gdk_colormap_alloc_color(gtk_widget_get_colormap(gui.drawarea),
	    &gcolor, FALSE, TRUE);

    return ret != 0 ? (guicolor_T)gcolor.pixel : INVALCOLOR;
#endif
}

/*
 * Set the current text foreground color.
 */
    void
gui_mch_set_fg_color(guicolor_T color)
{
#if GTK_CHECK_VERSION(3,0,0)
    *gui.fgcolor = color_to_rgba(color);
#else
    gui.fgcolor->pixel = (unsigned long)color;
#endif
}

/*
 * Set the current text background color.
 */
    void
gui_mch_set_bg_color(guicolor_T color)
{
#if GTK_CHECK_VERSION(3,0,0)
    *gui.bgcolor = color_to_rgba(color);
#else
    gui.bgcolor->pixel = (unsigned long)color;
#endif
}

/*
 * Set the current text special color.
 */
    void
gui_mch_set_sp_color(guicolor_T color)
{
#if GTK_CHECK_VERSION(3,0,0)
    *gui.spcolor = color_to_rgba(color);
#else
    gui.spcolor->pixel = (unsigned long)color;
#endif
}

/*
 * Function-like convenience macro for the sake of efficiency.
 */
#define INSERT_PANGO_ATTR(Attribute, AttrList, Start, End)  \
    G_STMT_START{					    \
	PangoAttribute *tmp_attr_;			    \
	tmp_attr_ = (Attribute);			    \
	tmp_attr_->start_index = (Start);		    \
	tmp_attr_->end_index = (End);			    \
	pango_attr_list_insert((AttrList), tmp_attr_);	    \
    }G_STMT_END

    static void
apply_wide_font_attr(char_u *s, int len, PangoAttrList *attr_list)
{
    char_u  *start = NULL;
    char_u  *p;
    int	    uc;

    for (p = s; p < s + len; p += utf_byte2len(*p))
    {
	uc = utf_ptr2char(p);

	if (start == NULL)
	{
	    if (uc >= 0x80 && utf_char2cells(uc) == 2)
		start = p;
	}
	else if (uc < 0x80 // optimization shortcut
		 || (utf_char2cells(uc) != 2 && !utf_iscomposing(uc)))
	{
	    INSERT_PANGO_ATTR(pango_attr_font_desc_new(gui.wide_font),
			      attr_list, start - s, p - s);
	    start = NULL;
	}
    }

    if (start != NULL)
	INSERT_PANGO_ATTR(pango_attr_font_desc_new(gui.wide_font),
			  attr_list, start - s, len);
}

    static int
count_cluster_cells(char_u *s, PangoItem *item,
		    PangoGlyphString* glyphs, int i,
		    int *cluster_width,
		    int *last_glyph_rbearing)
{
    char_u  *p;
    int	    next;	// glyph start index of next cluster
    int	    start, end; // string segment of current cluster
    int	    width;	// real cluster width in Pango units
    int	    uc;
    int	    cellcount = 0;

    width = glyphs->glyphs[i].geometry.width;

    for (next = i + 1; next < glyphs->num_glyphs; ++next)
    {
	if (glyphs->glyphs[next].attr.is_cluster_start)
	    break;
	else if (glyphs->glyphs[next].geometry.width > width)
	    width = glyphs->glyphs[next].geometry.width;
    }

    start = item->offset + glyphs->log_clusters[i];
    end   = item->offset + ((next < glyphs->num_glyphs) ?
			    glyphs->log_clusters[next] : item->length);

    for (p = s + start; p < s + end; p += utf_byte2len(*p))
    {
	uc = utf_ptr2char(p);
	if (uc < 0x80)
	    ++cellcount;
	else if (!utf_iscomposing(uc))
	    cellcount += utf_char2cells(uc);
    }

    if (last_glyph_rbearing != NULL
	    && cellcount > 0 && next == glyphs->num_glyphs)
    {
	PangoRectangle ink_rect;
	/*
	 * If a certain combining mark had to be taken from a non-monospace
	 * font, we have to compensate manually by adapting x_offset according
	 * to the ink extents of the previous glyph.
	 */
	pango_font_get_glyph_extents(item->analysis.font,
				     glyphs->glyphs[i].glyph,
				     &ink_rect, NULL);

	if (PANGO_RBEARING(ink_rect) > 0)
	    *last_glyph_rbearing = PANGO_RBEARING(ink_rect);
    }

    if (cellcount > 0)
	*cluster_width = width;

    return cellcount;
}

/*
 * If there are only combining characters in the cluster, we cannot just
 * change the width of the previous glyph since there is none.	Therefore
 * some guesswork is needed.
 *
 * If ink_rect.x is negative Pango apparently has taken care of the composing
 * by itself.  Actually setting x_offset = 0 should be sufficient then, but due
 * to problems with composing from different fonts we still need to fine-tune
 * x_offset to avoid ugliness.
 *
 * If ink_rect.x is not negative, force overstriking by pointing x_offset to
 * the position of the previous glyph.	Apparently this happens only with old
 * X fonts which don't provide the special combining information needed by
 * Pango.
 */
    static void
setup_zero_width_cluster(PangoItem *item, PangoGlyphInfo *glyph,
			 int last_cellcount, int last_cluster_width,
			 int last_glyph_rbearing)
{
    PangoRectangle  ink_rect;
    PangoRectangle  logical_rect;
    int		    width;

    width = last_cellcount * gui.char_width * PANGO_SCALE;
    glyph->geometry.x_offset = -width + MAX(0, width - last_cluster_width) / 2;
    glyph->geometry.width = 0;

    pango_font_get_glyph_extents(item->analysis.font,
				 glyph->glyph,
				 &ink_rect, &logical_rect);
    if (ink_rect.x < 0)
    {
	glyph->geometry.x_offset += last_glyph_rbearing;
	glyph->geometry.y_offset  = logical_rect.height
		- (gui.char_height - p_linespace) * PANGO_SCALE;
    }
    else
	// If the accent width is smaller than the cluster width, position it
	// in the middle.
	glyph->geometry.x_offset = -width + MAX(0, width - ink_rect.width) / 2;
}

#if GTK_CHECK_VERSION(3,0,0)
    static void
draw_glyph_string(int row, int col, int num_cells, int flags,
		  PangoFont *font, PangoGlyphString *glyphs,
		  cairo_t *cr)
#else
    static void
draw_glyph_string(int row, int col, int num_cells, int flags,
		  PangoFont *font, PangoGlyphString *glyphs)
#endif
{
    if (!(flags & DRAW_TRANSP))
    {
#if GTK_CHECK_VERSION(3,0,0)
	cairo_set_source_rgba(cr,
		gui.bgcolor->red, gui.bgcolor->green, gui.bgcolor->blue,
		gui.bgcolor->alpha);
	cairo_rectangle(cr,
			FILL_X(col), FILL_Y(row),
			num_cells * gui.char_width, gui.char_height);
	cairo_fill(cr);
#else
	gdk_gc_set_foreground(gui.text_gc, gui.bgcolor);

	gdk_draw_rectangle(gui.drawarea->window,
			   gui.text_gc,
			   TRUE,
			   FILL_X(col),
			   FILL_Y(row),
			   num_cells * gui.char_width,
			   gui.char_height);
#endif
    }

#if GTK_CHECK_VERSION(3,0,0)
    cairo_set_source_rgba(cr,
	    gui.fgcolor->red, gui.fgcolor->green, gui.fgcolor->blue,
	    gui.fgcolor->alpha);
    cairo_move_to(cr, TEXT_X(col), TEXT_Y(row));
    pango_cairo_show_glyph_string(cr, font, glyphs);
#else
    gdk_gc_set_foreground(gui.text_gc, gui.fgcolor);

    gdk_draw_glyphs(gui.drawarea->window,
		    gui.text_gc,
		    font,
		    TEXT_X(col),
		    TEXT_Y(row),
		    glyphs);
#endif

    // redraw the contents with an offset of 1 to emulate bold
    if ((flags & DRAW_BOLD) && !gui.font_can_bold)
#if GTK_CHECK_VERSION(3,0,0)
    {
	cairo_set_source_rgba(cr,
		gui.fgcolor->red, gui.fgcolor->green, gui.fgcolor->blue,
		gui.fgcolor->alpha);
	cairo_move_to(cr, TEXT_X(col) + 1, TEXT_Y(row));
	pango_cairo_show_glyph_string(cr, font, glyphs);
    }
#else
	gdk_draw_glyphs(gui.drawarea->window,
			gui.text_gc,
			font,
			TEXT_X(col) + 1,
			TEXT_Y(row),
			glyphs);
#endif
}

/*
 * Draw underline and undercurl at the bottom of the character cell.
 */
#if GTK_CHECK_VERSION(3,0,0)
    static void
draw_under(int flags, int row, int col, int cells, cairo_t *cr)
#else
    static void
draw_under(int flags, int row, int col, int cells)
#endif
{
    int			i;
    int			offset;
    static const int	val[8] = {1, 0, 0, 0, 1, 2, 2, 2 };
    int			y = FILL_Y(row + 1) - 1;

    // Undercurl: draw curl at the bottom of the character cell.
    if (flags & DRAW_UNDERC)
    {
#if GTK_CHECK_VERSION(3,0,0)
	cairo_set_line_width(cr, 1.0);
	cairo_set_line_cap(cr, CAIRO_LINE_CAP_BUTT);
	cairo_set_source_rgba(cr,
		gui.spcolor->red, gui.spcolor->green, gui.spcolor->blue,
		gui.spcolor->alpha);
	cairo_move_to(cr, FILL_X(col) + 1, y - 2 + 0.5);
	for (i = FILL_X(col) + 1; i < FILL_X(col + cells); ++i)
	{
	    offset = val[i % 8];
	    cairo_line_to(cr, i, y - offset + 0.5);
	}
	cairo_stroke(cr);
#else
	gdk_gc_set_foreground(gui.text_gc, gui.spcolor);
	for (i = FILL_X(col); i < FILL_X(col + cells); ++i)
	{
	    offset = val[i % 8];
	    gdk_draw_point(gui.drawarea->window, gui.text_gc, i, y - offset);
	}
	gdk_gc_set_foreground(gui.text_gc, gui.fgcolor);
#endif
    }

    // Draw a strikethrough line
    if (flags & DRAW_STRIKE)
    {
#if GTK_CHECK_VERSION(3,0,0)
	cairo_set_line_width(cr, 1.0);
	cairo_set_line_cap(cr, CAIRO_LINE_CAP_BUTT);
	cairo_set_source_rgba(cr,
		gui.spcolor->red, gui.spcolor->green, gui.spcolor->blue,
		gui.spcolor->alpha);
	cairo_move_to(cr, FILL_X(col), y + 1 - gui.char_height/2 + 0.5);
	cairo_line_to(cr, FILL_X(col + cells), y + 1 - gui.char_height/2 + 0.5);
	cairo_stroke(cr);
#else
	gdk_gc_set_foreground(gui.text_gc, gui.spcolor);
	gdk_draw_line(gui.drawarea->window, gui.text_gc,
		      FILL_X(col), y + 1 - gui.char_height/2,
		      FILL_X(col + cells), y + 1 - gui.char_height/2);
	gdk_gc_set_foreground(gui.text_gc, gui.fgcolor);
#endif
    }

    // Underline: draw a line at the bottom of the character cell.
    if (flags & DRAW_UNDERL)
    {
	// When p_linespace is 0, overwrite the bottom row of pixels.
	// Otherwise put the line just below the character.
	if (p_linespace > 1)
	    y -= p_linespace - 1;
#if GTK_CHECK_VERSION(3,0,0)
	cairo_set_line_width(cr, 1.0);
	cairo_set_line_cap(cr, CAIRO_LINE_CAP_BUTT);
	cairo_set_source_rgba(cr,
		gui.fgcolor->red, gui.fgcolor->green, gui.fgcolor->blue,
		gui.fgcolor->alpha);
	cairo_move_to(cr, FILL_X(col), y + 0.5);
	cairo_line_to(cr, FILL_X(col + cells), y + 0.5);
	cairo_stroke(cr);
#else
	gdk_draw_line(gui.drawarea->window, gui.text_gc,
		      FILL_X(col), y,
		      FILL_X(col + cells) - 1, y);
#endif
    }
}

    int
gui_gtk2_draw_string(int row, int col, char_u *s, int len, int flags)
{
    char_u	*conv_buf = NULL;   // result of UTF-8 conversion
    char_u	*new_conv_buf;
    int		convlen;
    char_u	*sp, *bp;
    int		plen;
    int		len_sum;	// return value needs to add up since we are
				// printing substrings
    int		byte_sum;	// byte position in string
    char_u	*cs;		// current *s pointer
    int		needs_pango;	// look ahead, 0=ascii 1=unicode/ligatures
    int		should_need_pango = FALSE;
    int		slen;
    int		is_ligature;
    int		is_utf8;
    char_u	backup_ch;

    if (gui.text_context == NULL || gtk_widget_get_window(gui.drawarea) == NULL)
	return len;

    if (output_conv.vc_type != CONV_NONE)
    {
	/*
	 * Convert characters from 'encoding' to 'termencoding', which is set
	 * to UTF-8 by gui_mch_init().	did_set_string_option() in option.c
	 * prohibits changing this to something else than UTF-8 if the GUI is
	 * in use.
	 */
	convlen = len;
	conv_buf = string_convert(&output_conv, s, &convlen);
	g_return_val_if_fail(conv_buf != NULL, len);

	// Correct for differences in char width: some chars are
	// double-wide in 'encoding' but single-wide in utf-8.  Add a space to
	// compensate for that.
	for (sp = s, bp = conv_buf; sp < s + len && bp < conv_buf + convlen; )
	{
	    plen = utf_ptr2len(bp);
	    if ((*mb_ptr2cells)(sp) == 2 && utf_ptr2cells(bp) == 1)
	    {
		new_conv_buf = alloc(convlen + 2);
		if (new_conv_buf == NULL)
		    return len;
		plen += bp - conv_buf;
		mch_memmove(new_conv_buf, conv_buf, plen);
		new_conv_buf[plen] = ' ';
		mch_memmove(new_conv_buf + plen + 1, conv_buf + plen,
							  convlen - plen + 1);
		vim_free(conv_buf);
		conv_buf = new_conv_buf;
		++convlen;
		bp = conv_buf + plen;
		plen = 1;
	    }
	    sp += (*mb_ptr2len)(sp);
	    bp += plen;
	}
	s = conv_buf;
	len = convlen;
    }

    /*
     * Ligature support and complex utf-8 char optimization:
     * String received to output to screen can print using pre-cached glyphs
     * (fast) or Pango (slow). Ligatures and multibype utf-8 must use Pango.
     * Since we receive mixed content string, split it into logical segments
     * that are guaranteed to go through glyphs as much as possible. Since
     * single ligature char prints as ascii, print it that way.
     */
    len_sum = 0;    // return value needs to add up since we are printing
		    // substrings
    byte_sum = 0;
    cs = s;
    // First char decides starting needs_pango mode, 0=ascii 1=utf8/ligatures.
    // Even if it is ligature char, two chars or more make ligature.
    // Ascii followed by utf8 is also going through pango.
    is_utf8 = (*cs & 0x80);
    is_ligature = gui.ligatures_map[*cs] && (len > 1);
    if (is_ligature)
	is_ligature = gui.ligatures_map[*(cs + 1)];
    if (!is_utf8 && len > 1)
	is_utf8 = (*(cs + 1) & 0x80) != 0;
    needs_pango = is_utf8 || is_ligature;

    // split string into ascii and non-ascii (ligatures + utf-8) substrings,
    // print glyphs or use Pango
    while (cs < s + len)
    {
	slen = 0;
	while (slen < (len - byte_sum))
	{
	    is_ligature = gui.ligatures_map[*(cs + slen)];
	    // look ahead, single ligature char between ascii is ascii
	    if (is_ligature && !needs_pango)
	    {
		if ((slen + 1) < (len - byte_sum))
		    is_ligature = gui.ligatures_map[*(cs + slen + 1)];
		else
		    is_ligature = 0;
	    }
	    is_utf8 = *(cs + slen) & 0x80;
	    // ascii followed by utf8 could be combining
	    // if so send it through pango
	    if ((!is_utf8) && ((slen + 1) < (len - byte_sum)))
		is_utf8 = (*(cs + slen + 1) & 0x80);
	    should_need_pango = (is_ligature || is_utf8);
	    if (needs_pango != should_need_pango) // mode switch
		break;
	    if (needs_pango)
	    {
		if (is_ligature)
		{
		    slen++; // ligature char by char
		}
		else // is_utf8
		{
		    if ((*(cs + slen) & 0xC0) == 0x80)
		    {
			// a continuation, find next 0xC0 != 0x80 but don't
			// include it
			while ((slen < (len - byte_sum))
					    && ((*(cs + slen) & 0xC0) == 0x80))
			{
			    slen++;
			}
		    }
		    else if ((*(cs + slen) & 0xE0) == 0xC0)
		    {
			// + one byte utf8
			slen++;
		    }
		    else if ((*(cs + slen) & 0xF0) == 0xE0)
		    {
			// + two bytes utf8
			slen += 2;
		    }
		    else if ((*(cs + slen) & 0xF8) == 0xF0)
		    {
			// + three bytes utf8
			slen += 3;
		    }
		    else
		    {
			// this should not happen, try moving forward, Pango
			// will catch it
			slen++;
		    }
		}
	    }
	    else
	    {
		slen++; // ascii
	    }
	}

	if (slen < len)
	{
	    // temporarily zero terminate substring, print, restore char, wrap
	    backup_ch = *(cs + slen);
	    *(cs + slen) = NUL;
	}
	len_sum += gui_gtk2_draw_string_ext(row, col + len_sum,
						 cs, slen, flags, needs_pango);
	if (slen < len)
	    *(cs + slen) = backup_ch;
	cs += slen;
	byte_sum += slen;
	needs_pango = should_need_pango;
    }
    vim_free(conv_buf);
    return len_sum;
}

    int
gui_gtk2_draw_string_ext(
	int	row,
	int	col,
	char_u	*s,
	int	len,
	int	flags,
	int	force_pango)
{
    GdkRectangle	area;		    // area for clip mask
    PangoGlyphString	*glyphs;	    // glyphs of current item
    int			column_offset = 0;  // column offset in cells
    int			i;
#if GTK_CHECK_VERSION(3,0,0)
    cairo_t		*cr;
#endif

    /*
     * Restrict all drawing to the current screen line in order to prevent
     * fuzzy font lookups from messing up the screen.
     */
    area.x = gui.border_offset;
    area.y = FILL_Y(row);
    area.width	= gui.num_cols * gui.char_width;
    area.height = gui.char_height;

#if GTK_CHECK_VERSION(3,0,0)
    cr = cairo_create(gui.surface);
    cairo_rectangle(cr, area.x, area.y, area.width, area.height);
    cairo_clip(cr);
#else
    gdk_gc_set_clip_origin(gui.text_gc, 0, 0);
    gdk_gc_set_clip_rectangle(gui.text_gc, &area);
#endif

    glyphs = pango_glyph_string_new();

    /*
     * Optimization hack:  If possible, skip the itemize and shaping process
     * for pure ASCII strings.	This optimization is particularly effective
     * because Vim draws space characters to clear parts of the screen.
     */
    if (!(flags & DRAW_ITALIC)
	    && !((flags & DRAW_BOLD) && gui.font_can_bold)
	    && gui.ascii_glyphs != NULL
	    && !force_pango)
    {
	char_u *p;

	for (p = s; p < s + len; ++p)
	    if (*p & 0x80)
		goto not_ascii;

	pango_glyph_string_set_size(glyphs, len);

	for (i = 0; i < len; ++i)
	{
	    glyphs->glyphs[i] = gui.ascii_glyphs->glyphs[2 * s[i]];
	    glyphs->log_clusters[i] = i;
	}

#if GTK_CHECK_VERSION(3,0,0)
	draw_glyph_string(row, col, len, flags, gui.ascii_font, glyphs, cr);
#else
	draw_glyph_string(row, col, len, flags, gui.ascii_font, glyphs);
#endif

	column_offset = len;
    }
    else
not_ascii:
    {
	PangoAttrList	*attr_list;
	GList		*item_list;
	int		cluster_width;
	int		last_glyph_rbearing;
	int		cells = 0;  // cells occupied by current cluster

	// Safety check: pango crashes when invoked with invalid utf-8
	// characters.
	if (!utf_valid_string(s, s + len))
	{
	    column_offset = len;
	    goto skipitall;
	}

	// original width of the current cluster
	cluster_width = PANGO_SCALE * gui.char_width;

	// right bearing of the last non-composing glyph
	last_glyph_rbearing = PANGO_SCALE * gui.char_width;

	attr_list = pango_attr_list_new();

	// If 'guifontwide' is set then use that for double-width characters.
	// Otherwise just go with 'guifont' and let Pango do its thing.
	if (gui.wide_font != NULL)
	    apply_wide_font_attr(s, len, attr_list);

	if ((flags & DRAW_BOLD) && gui.font_can_bold)
	    INSERT_PANGO_ATTR(pango_attr_weight_new(PANGO_WEIGHT_BOLD),
			      attr_list, 0, len);
	if (flags & DRAW_ITALIC)
	    INSERT_PANGO_ATTR(pango_attr_style_new(PANGO_STYLE_ITALIC),
			      attr_list, 0, len);
	/*
	 * Break the text into segments with consistent directional level
	 * and shaping engine.	Pure Latin text needs only a single segment,
	 * so there's no need to worry about the loop's efficiency.  Better
	 * try to optimize elsewhere, e.g. reducing exposes and stuff :)
	 */
	item_list = pango_itemize(gui.text_context,
				  (const char *)s, 0, len, attr_list, NULL);

	while (item_list != NULL)
	{
	    PangoItem	*item;
	    int		item_cells = 0; // item length in cells

	    item = (PangoItem *)item_list->data;
	    item_list = g_list_delete_link(item_list, item_list);
	    /*
	     * Increment the bidirectional embedding level by 1 if it is not
	     * even.  An odd number means the output will be RTL, but we don't
	     * want that since Vim handles right-to-left text on its own.  It
	     * would probably be sufficient to just set level = 0, but you can
	     * never know :)
	     *
	     * Unfortunately we can't take advantage of Pango's ability to
	     * render both LTR and RTL at the same time.  In order to support
	     * that, Vim's main screen engine would have to make use of Pango
	     * functionality.
	     */
	    item->analysis.level = (item->analysis.level + 1) & (~1U);

	    // HACK: Overrule the shape engine, we don't want shaping to be
	    // done, because drawing the cursor would change the display.
	    item->analysis.shape_engine = default_shape_engine;

#ifdef HAVE_PANGO_SHAPE_FULL
	    pango_shape_full((const char *)s + item->offset, item->length,
		    (const char *)s, len, &item->analysis, glyphs);
#else
	    pango_shape((const char *)s + item->offset, item->length,
			&item->analysis, glyphs);
#endif
	    /*
	     * Fixed-width hack: iterate over the array and assign a fixed
	     * width to each glyph, thus overriding the choice made by the
	     * shaping engine.	We use utf_char2cells() to determine the
	     * number of cells needed.
	     *
	     * Also perform all kind of dark magic to get composing
	     * characters right (and pretty too of course).
	     */
	    for (i = 0; i < glyphs->num_glyphs; ++i)
	    {
		PangoGlyphInfo *glyph;

		glyph = &glyphs->glyphs[i];

		if (glyph->attr.is_cluster_start)
		{
		    int cellcount;

		    cellcount = count_cluster_cells(
			s, item, glyphs, i, &cluster_width,
			(item_list != NULL) ? &last_glyph_rbearing : NULL);

		    if (cellcount > 0)
		    {
			int width;

			width = cellcount * gui.char_width * PANGO_SCALE;
			glyph->geometry.x_offset +=
					    MAX(0, width - cluster_width) / 2;
			glyph->geometry.width = width;
		    }
		    else
		    {
			// If there are only combining characters in the
			// cluster, we cannot just change the width of the
			// previous glyph since there is none.	Therefore
			// some guesswork is needed.
			setup_zero_width_cluster(item, glyph, cells,
						 cluster_width,
						 last_glyph_rbearing);
		    }

		    item_cells += cellcount;
		    cells = cellcount;
		}
		else if (i > 0)
		{
		    int width;

		    // There is a previous glyph, so we deal with combining
		    // characters the canonical way.
		    // In some circumstances Pango uses a positive x_offset,
		    // then use the width of the previous glyph for this one
		    // and set the previous width to zero.
		    // Otherwise we get a negative x_offset, Pango has already
		    // positioned the combining char, keep the widths as they
		    // are.
		    // For both adjust the x_offset to position the glyph in
		    // the middle.
		    if (glyph->geometry.x_offset >= 0)
		    {
			glyphs->glyphs[i].geometry.width =
					 glyphs->glyphs[i - 1].geometry.width;
			glyphs->glyphs[i - 1].geometry.width = 0;
		    }
		    width = cells * gui.char_width * PANGO_SCALE;
		    glyph->geometry.x_offset +=
					    MAX(0, width - cluster_width) / 2;
		}
		else // i == 0 "cannot happen"
		{
		    glyph->geometry.width = 0;
		}
	    }

	    //// Aaaaand action! **
#if GTK_CHECK_VERSION(3,0,0)
	    draw_glyph_string(row, col + column_offset, item_cells,
			      flags, item->analysis.font, glyphs,
			      cr);
#else
	    draw_glyph_string(row, col + column_offset, item_cells,
			      flags, item->analysis.font, glyphs);
#endif

	    pango_item_free(item);

	    column_offset += item_cells;
	}

	pango_attr_list_unref(attr_list);
    }

skipitall:
    // Draw underline and undercurl.
#if GTK_CHECK_VERSION(3,0,0)
    draw_under(flags, row, col, column_offset, cr);
#else
    draw_under(flags, row, col, column_offset);
#endif

    pango_glyph_string_free(glyphs);

#if GTK_CHECK_VERSION(3,0,0)
    cairo_destroy(cr);
    gtk_widget_queue_draw_area(gui.drawarea, area.x, area.y,
	    area.width, area.height);
#else
    gdk_gc_set_clip_rectangle(gui.text_gc, NULL);
#endif

    return column_offset;
}

/*
 * Return OK if the key with the termcap name "name" is supported.
 */
    int
gui_mch_haskey(char_u *name)
{
    int i;

    for (i = 0; special_keys[i].key_sym != 0; i++)
	if (name[0] == special_keys[i].code0
		&& name[1] == special_keys[i].code1)
	    return OK;
    return FAIL;
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Return the text window-id and display.  Only required for X-based GUI's
 */
    int
gui_get_x11_windis(Window *win, Display **dis)
{
    Display * dpy = gui_mch_get_display();
    if (dpy)
    {
	*dis = dpy;
	*win = GDK_WINDOW_XID(gtk_widget_get_window(gui.mainwin));
	return OK;
    }

    *dis = NULL;
    *win = 0;
    return FAIL;
}
#endif

    Display *
gui_mch_get_display(void)
{
    if (gui.mainwin != NULL && gtk_widget_get_window(gui.mainwin) != NULL
#if GTK_CHECK_VERSION(3,0,0)
	    && GDK_IS_X11_DISPLAY(gtk_widget_get_display(gui.mainwin))
#endif
	)
	return GDK_WINDOW_XDISPLAY(gtk_widget_get_window(gui.mainwin));
    else
	return NULL;
}

    void
gui_mch_beep(void)
{
    GdkDisplay *display;

    if (gui.mainwin != NULL && gtk_widget_get_realized(gui.mainwin))
	display = gtk_widget_get_display(gui.mainwin);
    else
	display = gdk_display_get_default();

    if (display != NULL)
	gdk_display_beep(display);
}

    void
gui_mch_flash(int msec)
{
#if GTK_CHECK_VERSION(3,0,0)
    // TODO Replace GdkGC with Cairo
    (void)msec;
#else
    GdkGCValues	values;
    GdkGC	*invert_gc;

    if (gui.drawarea->window == NULL)
	return;

    values.foreground.pixel = gui.norm_pixel ^ gui.back_pixel;
    values.background.pixel = gui.norm_pixel ^ gui.back_pixel;
    values.function = GDK_XOR;
    invert_gc = gdk_gc_new_with_values(gui.drawarea->window,
				       &values,
				       GDK_GC_FOREGROUND |
				       GDK_GC_BACKGROUND |
				       GDK_GC_FUNCTION);
    gdk_gc_set_exposures(invert_gc,
			 gui.visibility != GDK_VISIBILITY_UNOBSCURED);
    /*
     * Do a visual beep by changing back and forth in some undetermined way,
     * the foreground and background colors.  This is due to the fact that
     * there can't be really any prediction about the effects of XOR on
     * arbitrary X11 servers. However this seems to be enough for what we
     * intend it to do.
     */
    gdk_draw_rectangle(gui.drawarea->window, invert_gc,
		       TRUE,
		       0, 0,
		       FILL_X((int)Columns) + gui.border_offset,
		       FILL_Y((int)Rows) + gui.border_offset);

    gui_mch_flush();
    ui_delay((long)msec, TRUE);	// wait so many msec

    gdk_draw_rectangle(gui.drawarea->window, invert_gc,
		       TRUE,
		       0, 0,
		       FILL_X((int)Columns) + gui.border_offset,
		       FILL_Y((int)Rows) + gui.border_offset);

    gdk_gc_destroy(invert_gc);
#endif
}

/*
 * Invert a rectangle from row r, column c, for nr rows and nc columns.
 */
    void
gui_mch_invert_rectangle(int r, int c, int nr, int nc)
{
#if GTK_CHECK_VERSION(3,0,0)
    const GdkRectangle rect = {
	FILL_X(c), FILL_Y(r), nc * gui.char_width, nr * gui.char_height
    };
    cairo_t * const cr = cairo_create(gui.surface);

    cairo_set_source_rgba(cr, 1.0, 1.0, 1.0, 1.0);
# if CAIRO_VERSION >= CAIRO_VERSION_ENCODE(1,9,2)
    cairo_set_operator(cr, CAIRO_OPERATOR_DIFFERENCE);
# else
    // Give an implementation for older cairo versions if necessary.
# endif
    cairo_rectangle(cr, rect.x, rect.y, rect.width, rect.height);
    cairo_fill(cr);

    cairo_destroy(cr);

    gtk_widget_queue_draw_area(gui.drawarea, rect.x, rect.y,
	    rect.width, rect.height);
#else
    GdkGCValues values;
    GdkGC *invert_gc;

    if (gui.drawarea->window == NULL)
	return;

    values.function = GDK_INVERT;
    invert_gc = gdk_gc_new_with_values(gui.drawarea->window,
				       &values,
				       GDK_GC_FUNCTION);
    gdk_gc_set_exposures(invert_gc, gui.visibility !=
						   GDK_VISIBILITY_UNOBSCURED);
    gdk_draw_rectangle(gui.drawarea->window, invert_gc,
		       TRUE,
		       FILL_X(c), FILL_Y(r),
		       (nc) * gui.char_width, (nr) * gui.char_height);
    gdk_gc_destroy(invert_gc);
#endif
}

/*
 * Iconify the GUI window.
 */
    void
gui_mch_iconify(void)
{
    gtk_window_iconify(GTK_WINDOW(gui.mainwin));
}

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Bring the Vim window to the foreground.
 */
    void
gui_mch_set_foreground(void)
{
    // Just calling gtk_window_present() used to work in the past, but now this
    // sequence appears to be needed:
    // - Show the window on top of others.
    // - Present the window (also shows it above others).
    // - Do not the window on top of others (otherwise it would be stuck there).
    gtk_window_set_keep_above(GTK_WINDOW(gui.mainwin), TRUE);
    gui_may_flush();
    gtk_window_present(GTK_WINDOW(gui.mainwin));
    gui_may_flush();
    gtk_window_set_keep_above(GTK_WINDOW(gui.mainwin), FALSE);
    gui_may_flush();
}
#endif

/*
 * Draw a cursor without focus.
 */
    void
gui_mch_draw_hollow_cursor(guicolor_T color)
{
    int		i = 1;
#if GTK_CHECK_VERSION(3,0,0)
    cairo_t    *cr;
#endif

    if (gtk_widget_get_window(gui.drawarea) == NULL)
	return;

#if GTK_CHECK_VERSION(3,0,0)
    cr = cairo_create(gui.surface);
#endif

    gui_mch_set_fg_color(color);

#if GTK_CHECK_VERSION(3,0,0)
    cairo_set_source_rgba(cr,
	    gui.fgcolor->red, gui.fgcolor->green, gui.fgcolor->blue,
	    gui.fgcolor->alpha);
#else
    gdk_gc_set_foreground(gui.text_gc, gui.fgcolor);
#endif
    if (mb_lefthalve(gui.row, gui.col))
	i = 2;
#if GTK_CHECK_VERSION(3,0,0)
    cairo_set_line_width(cr, 1.0);
    cairo_set_line_cap(cr, CAIRO_LINE_CAP_BUTT);
    cairo_rectangle(cr,
	    FILL_X(gui.col) + 0.5, FILL_Y(gui.row) + 0.5,
	    i * gui.char_width - 1, gui.char_height - 1);
    cairo_stroke(cr);
    cairo_destroy(cr);
#else
    gdk_draw_rectangle(gui.drawarea->window, gui.text_gc,
	    FALSE,
	    FILL_X(gui.col), FILL_Y(gui.row),
	    i * gui.char_width - 1, gui.char_height - 1);
#endif
}

/*
 * Draw part of a cursor, "w" pixels wide, and "h" pixels high, using
 * color "color".
 */
    void
gui_mch_draw_part_cursor(int w, int h, guicolor_T color)
{
    if (gtk_widget_get_window(gui.drawarea) == NULL)
	return;

    gui_mch_set_fg_color(color);

#if GTK_CHECK_VERSION(3,0,0)
    {
	cairo_t *cr;

	cr = cairo_create(gui.surface);
	cairo_set_source_rgba(cr,
		gui.fgcolor->red, gui.fgcolor->green, gui.fgcolor->blue,
		gui.fgcolor->alpha);
	cairo_rectangle(cr,
# ifdef FEAT_RIGHTLEFT
	    // vertical line should be on the right of current point
	    CURSOR_BAR_RIGHT ? FILL_X(gui.col + 1) - w :
# endif
	    FILL_X(gui.col), FILL_Y(gui.row) + gui.char_height - h,
	    w, h);
	cairo_fill(cr);
	cairo_destroy(cr);
    }
#else // !GTK_CHECK_VERSION(3,0,0)
    gdk_gc_set_foreground(gui.text_gc, gui.fgcolor);
    gdk_draw_rectangle(gui.drawarea->window, gui.text_gc,
	    TRUE,
# ifdef FEAT_RIGHTLEFT
	    // vertical line should be on the right of current point
	    CURSOR_BAR_RIGHT ? FILL_X(gui.col + 1) - w :
# endif
	    FILL_X(gui.col),
	    FILL_Y(gui.row) + gui.char_height - h,
	    w, h);
#endif // !GTK_CHECK_VERSION(3,0,0)
}


/*
 * Catch up with any queued X11 events.  This may put keyboard input into the
 * input buffer, call resize call-backs, trigger timers etc.  If there is
 * nothing in the X11 event queue (& no timers pending), then we return
 * immediately.
 */
    void
gui_mch_update(void)
{
    while (g_main_context_pending(NULL) && !vim_is_input_buf_full())
	g_main_context_iteration(NULL, TRUE);
}

    static timeout_cb_type
input_timer_cb(gpointer data)
{
    int *timed_out = (int *) data;

    // Just inform the caller about the occurrence of it
    *timed_out = TRUE;

    return FALSE;		// don't happen again
}

#ifdef FEAT_JOB_CHANNEL
    static timeout_cb_type
channel_poll_cb(gpointer data UNUSED)
{
    // Using an event handler for a channel that may be disconnected does
    // not work, it hangs.  Instead poll for messages.
    channel_handle_events(TRUE);
    parse_queued_messages();

    return TRUE;		// repeat
}
#endif

/*
 * GUI input routine called by gui_wait_for_chars().  Waits for a character
 * from the keyboard.
 *  wtime == -1     Wait forever.
 *  wtime == 0	    This should never happen.
 *  wtime > 0	    Wait wtime milliseconds for a character.
 * Returns OK if a character was found to be available within the given time,
 * or FAIL otherwise.
 */
    int
gui_mch_wait_for_chars(long wtime)
{
    int		focus;
    guint	timer;
    static int	timed_out;
    int		retval = FAIL;
#ifdef FEAT_JOB_CHANNEL
    guint	channel_timer = 0;
#endif

    timed_out = FALSE;

    // This timeout makes sure that we will return if no characters arrived in
    // time. If "wtime" is zero just use one.
    if (wtime >= 0)
	timer = timeout_add(wtime == 0 ? 1L : wtime,
						   input_timer_cb, &timed_out);
    else
	timer = 0;

#ifdef FEAT_JOB_CHANNEL
    // If there is a channel with the keep_open flag we need to poll for input
    // on them.
    if (channel_any_keep_open())
	channel_timer = timeout_add(20, channel_poll_cb, NULL);
#endif

    focus = gui.in_focus;

    do
    {
	// Stop or start blinking when focus changes
	if (gui.in_focus != focus)
	{
	    if (gui.in_focus)
		gui_mch_start_blink();
	    else
		gui_mch_stop_blink(TRUE);
	    focus = gui.in_focus;
	}

#ifdef MESSAGE_QUEUE
# ifdef FEAT_TIMERS
	did_add_timer = FALSE;
# endif
	parse_queued_messages();
# ifdef FEAT_TIMERS
	if (did_add_timer)
	    // Need to recompute the waiting time.
	    goto theend;
# endif
#endif

	/*
	 * Loop in GTK+ processing  until a timeout or input occurs.
	 * Skip this if input is available anyway (can happen in rare
	 * situations, sort of race condition).
	 */
	if (!input_available())
	    g_main_context_iteration(NULL, TRUE);

	// Got char, return immediately
	if (input_available())
	{
	    retval = OK;
	    goto theend;
	}
    } while (wtime < 0 || !timed_out);

    /*
     * Flush all eventually pending (drawing) events.
     */
    gui_mch_update();

theend:
    if (timer != 0 && !timed_out)
	timeout_remove(timer);
#ifdef FEAT_JOB_CHANNEL
    if (channel_timer != 0)
	timeout_remove(channel_timer);
#endif

    return retval;
}


/////////////////////////////////////////////////////////////////////////////
// Output drawing routines.
//


// Flush any output to the screen
    void
gui_mch_flush(void)
{
    if (gui.mainwin != NULL && gtk_widget_get_realized(gui.mainwin))
#if GTK_CHECK_VERSION(2,4,0)
	gdk_display_flush(gtk_widget_get_display(gui.mainwin));
#else
	gdk_display_sync(gtk_widget_get_display(gui.mainwin));
#endif
}

/*
 * Clear a rectangular region of the screen from text pos (row1, col1) to
 * (row2, col2) inclusive.
 */
    void
gui_mch_clear_block(int row1arg, int col1arg, int row2arg, int col2arg)
{

    int col1 = check_col(col1arg);
    int col2 = check_col(col2arg);
    int row1 = check_row(row1arg);
    int row2 = check_row(row2arg);

#if GTK_CHECK_VERSION(3,0,0)
    if (gtk_widget_get_window(gui.drawarea) == NULL)
	return;
#else
    GdkColor color;

    if (gui.drawarea->window == NULL)
	return;

    color.pixel = gui.back_pixel;
#endif

#if GTK_CHECK_VERSION(3,0,0)
    {
	// Add one pixel to the far right column in case a double-stroked
	// bold glyph may sit there.
	const GdkRectangle rect = {
	    FILL_X(col1), FILL_Y(row1),
	    (col2 - col1 + 1) * gui.char_width + (col2 == Columns - 1),
	    (row2 - row1 + 1) * gui.char_height
	};
	cairo_t * const cr = cairo_create(gui.surface);
# if GTK_CHECK_VERSION(3,22,2)
	set_cairo_source_rgba_from_color(cr, gui.back_pixel);
# else
	GdkWindow * const win = gtk_widget_get_window(gui.drawarea);
	cairo_pattern_t * const pat = gdk_window_get_background_pattern(win);
	if (pat != NULL)
	    cairo_set_source(cr, pat);
	else
	    set_cairo_source_rgba_from_color(cr, gui.back_pixel);
# endif
	cairo_rectangle(cr, rect.x, rect.y, rect.width, rect.height);
	cairo_fill(cr);
	cairo_destroy(cr);

	gtk_widget_queue_draw_area(gui.drawarea,
		rect.x, rect.y, rect.width, rect.height);
    }
#else // !GTK_CHECK_VERSION(3,0,0)
    gdk_gc_set_foreground(gui.text_gc, &color);

    // Clear one extra pixel at the far right, for when bold characters have
    // spilled over to the window border.
    gdk_draw_rectangle(gui.drawarea->window, gui.text_gc, TRUE,
		       FILL_X(col1), FILL_Y(row1),
		       (col2 - col1 + 1) * gui.char_width
						      + (col2 == Columns - 1),
		       (row2 - row1 + 1) * gui.char_height);
#endif // !GTK_CHECK_VERSION(3,0,0)
}

#if GTK_CHECK_VERSION(3,0,0)
    static void
gui_gtk_window_clear(GdkWindow *win)
{
    const GdkRectangle rect = {
	0, 0, gdk_window_get_width(win), gdk_window_get_height(win)
    };
    cairo_t * const cr = cairo_create(gui.surface);
# if GTK_CHECK_VERSION(3,22,2)
    set_cairo_source_rgba_from_color(cr, gui.back_pixel);
# else
    cairo_pattern_t * const pat = gdk_window_get_background_pattern(win);
    if (pat != NULL)
	cairo_set_source(cr, pat);
    else
	set_cairo_source_rgba_from_color(cr, gui.back_pixel);
# endif
    cairo_rectangle(cr, rect.x, rect.y, rect.width, rect.height);
    cairo_fill(cr);
    cairo_destroy(cr);

    gtk_widget_queue_draw_area(gui.drawarea,
	    rect.x, rect.y, rect.width, rect.height);
}
#else
# define gui_gtk_window_clear(win)  gdk_window_clear(win)
#endif

    void
gui_mch_clear_all(void)
{
    if (gtk_widget_get_window(gui.drawarea) != NULL)
	gui_gtk_window_clear(gtk_widget_get_window(gui.drawarea));
}

#if !GTK_CHECK_VERSION(3,0,0)
/*
 * Redraw any text revealed by scrolling up/down.
 */
    static void
check_copy_area(void)
{
    GdkEvent	*event;
    int		expose_count;

    if (gui.visibility != GDK_VISIBILITY_PARTIAL)
	return;

    // Avoid redrawing the cursor while scrolling or it'll end up where
    // we don't want it to be.	I'm not sure if it's correct to call
    // gui_dont_update_cursor() at this point but it works as a quick
    // fix for now.
    gui_dont_update_cursor(TRUE);

    do
    {
	// Wait to check whether the scroll worked or not.
	event = gdk_event_get_graphics_expose(gui.drawarea->window);

	if (event == NULL)
	    break; // received NoExpose event

	gui_redraw(event->expose.area.x, event->expose.area.y,
		   event->expose.area.width, event->expose.area.height);

	expose_count = event->expose.count;
	gdk_event_free(event);
    }
    while (expose_count > 0); // more events follow

    gui_can_update_cursor();
}
#endif // !GTK_CHECK_VERSION(3,0,0)

#if GTK_CHECK_VERSION(3,0,0)
    static void
gui_gtk_surface_copy_rect(int dest_x, int dest_y,
			  int src_x,  int src_y,
			  int width,  int height)
{
    cairo_t * const cr = cairo_create(gui.surface);

    cairo_rectangle(cr, dest_x, dest_y, width, height);
    cairo_clip(cr);
    cairo_push_group(cr);
    cairo_set_source_surface(cr, gui.surface, dest_x - src_x, dest_y - src_y);
    cairo_paint(cr);
    cairo_pop_group_to_source(cr);
    cairo_paint(cr);

    cairo_destroy(cr);
}
#endif

/*
 * Delete the given number of lines from the given row, scrolling up any
 * text further down within the scroll region.
 */
    void
gui_mch_delete_lines(int row, int num_lines)
{
#if GTK_CHECK_VERSION(3,0,0)
    const int ncols = gui.scroll_region_right - gui.scroll_region_left + 1;
    const int nrows = gui.scroll_region_bot - row + 1;
    const int src_nrows = nrows - num_lines;

    gui_gtk_surface_copy_rect(
	    FILL_X(gui.scroll_region_left), FILL_Y(row),
	    FILL_X(gui.scroll_region_left), FILL_Y(row + num_lines),
	    gui.char_width * ncols + 1,     gui.char_height * src_nrows);
    gui_clear_block(
	    gui.scroll_region_bot - num_lines + 1, gui.scroll_region_left,
	    gui.scroll_region_bot,		   gui.scroll_region_right);
    gtk_widget_queue_draw_area(gui.drawarea,
	    FILL_X(gui.scroll_region_left), FILL_Y(row),
	    gui.char_width * ncols + 1,	gui.char_height * nrows);
#else
    if (gui.visibility == GDK_VISIBILITY_FULLY_OBSCURED)
	return;			// Can't see the window

    gdk_gc_set_foreground(gui.text_gc, gui.fgcolor);
    gdk_gc_set_background(gui.text_gc, gui.bgcolor);

    // copy one extra pixel, for when bold has spilled over
    gdk_window_copy_area(gui.drawarea->window, gui.text_gc,
	    FILL_X(gui.scroll_region_left), FILL_Y(row),
	    gui.drawarea->window,
	    FILL_X(gui.scroll_region_left),
	    FILL_Y(row + num_lines),
	    gui.char_width * (gui.scroll_region_right
					    - gui.scroll_region_left + 1) + 1,
	    gui.char_height * (gui.scroll_region_bot - row - num_lines + 1));

    gui_clear_block(gui.scroll_region_bot - num_lines + 1,
						       gui.scroll_region_left,
		    gui.scroll_region_bot, gui.scroll_region_right);
    check_copy_area();
#endif // !GTK_CHECK_VERSION(3,0,0)
}

/*
 * Insert the given number of lines before the given row, scrolling down any
 * following text within the scroll region.
 */
    void
gui_mch_insert_lines(int row, int num_lines)
{
#if GTK_CHECK_VERSION(3,0,0)
    const int ncols = gui.scroll_region_right - gui.scroll_region_left + 1;
    const int nrows = gui.scroll_region_bot - row + 1;
    const int src_nrows = nrows - num_lines;

    gui_gtk_surface_copy_rect(
	    FILL_X(gui.scroll_region_left), FILL_Y(row + num_lines),
	    FILL_X(gui.scroll_region_left), FILL_Y(row),
	    gui.char_width * ncols + 1,     gui.char_height * src_nrows);
    gui_clear_block(
	    row,		 gui.scroll_region_left,
	    row + num_lines - 1, gui.scroll_region_right);
    gtk_widget_queue_draw_area(gui.drawarea,
	    FILL_X(gui.scroll_region_left), FILL_Y(row),
	    gui.char_width * ncols + 1,	gui.char_height * nrows);
#else
    if (gui.visibility == GDK_VISIBILITY_FULLY_OBSCURED)
	return;			// Can't see the window

    gdk_gc_set_foreground(gui.text_gc, gui.fgcolor);
    gdk_gc_set_background(gui.text_gc, gui.bgcolor);

    // copy one extra pixel, for when bold has spilled over
    gdk_window_copy_area(gui.drawarea->window, gui.text_gc,
	    FILL_X(gui.scroll_region_left), FILL_Y(row + num_lines),
	    gui.drawarea->window,
	    FILL_X(gui.scroll_region_left), FILL_Y(row),
	    gui.char_width * (gui.scroll_region_right
					    - gui.scroll_region_left + 1) + 1,
	    gui.char_height * (gui.scroll_region_bot - row - num_lines + 1));

    gui_clear_block(row, gui.scroll_region_left,
				row + num_lines - 1, gui.scroll_region_right);
    check_copy_area();
#endif // !GTK_CHECK_VERSION(3,0,0)
}

/*
 * X Selection stuff, for cutting and pasting text to other windows.
 */
    void
clip_mch_request_selection(Clipboard_T *cbd)
{
    GdkAtom	target;
    unsigned	i;
    time_t	start;

    for (i = 0; i < N_SELECTION_TARGETS; ++i)
    {
	if (!clip_html && selection_targets[i].info == TARGET_HTML)
	    continue;
	received_selection = RS_NONE;
	target = gdk_atom_intern(selection_targets[i].target, FALSE);

	gtk_selection_convert(gui.drawarea,
			      cbd->gtk_sel_atom, target,
			      (guint32)GDK_CURRENT_TIME);

	// Hack: Wait up to three seconds for the selection.  A hang was
	// noticed here when using the netrw plugin combined with ":gui"
	// during the FocusGained event.
	start = time(NULL);
	while (received_selection == RS_NONE && time(NULL) < start + 3)
	    g_main_context_iteration(NULL, TRUE);	// wait for selection_received_cb

	if (received_selection != RS_FAIL)
	    return;
    }

    if (gui_mch_get_display())
	// Final fallback position - use the X CUT_BUFFER0 store
	yank_cut_buffer0(GDK_WINDOW_XDISPLAY(gtk_widget_get_window(gui.mainwin)),
		cbd);
}

/*
 * Disown the selection.
 */
    void
clip_mch_lose_selection(Clipboard_T *cbd UNUSED)
{
    if (in_selection_clear_event)
	return;

    gtk_selection_owner_set(NULL, cbd->gtk_sel_atom, gui.event_time);
    gui_mch_update();
}

/*
 * Own the selection and return OK if it worked.
 */
    int
clip_mch_own_selection(Clipboard_T *cbd)
{
    // If we're blocking autocmds, we are filling the register to offer the
    // selection (inside selection-get)
    if (is_autocmd_blocked())
	return OK;

    int success;

    success = gtk_selection_owner_set(gui.drawarea, cbd->gtk_sel_atom,
				      gui.event_time);
    // don't update on every visual selection change
    if (!(cbd->owned && VIsual_active))
	gui_gtk_set_selection_targets(cbd->gtk_sel_atom);
    gui_mch_update();
    return (success) ? OK : FAIL;
}

/*
 * Send the current selection to the clipboard.  Do nothing for X because we
 * will fill in the selection only when requested by another app.
 */
    void
clip_mch_set_selection(Clipboard_T *cbd UNUSED)
{
}

#if (defined(FEAT_XCLIPBOARD) && defined(USE_SYSTEM)) || defined(PROTO)
    int
clip_gtk_owner_exists(Clipboard_T *cbd)
{
    return gdk_selection_owner_get(cbd->gtk_sel_atom) != NULL;
}
#endif


#if defined(FEAT_MENU) || defined(PROTO)
/*
 * Make a menu item appear either active or not active (grey or not grey).
 */
    void
gui_mch_menu_grey(vimmenu_T *menu, int grey)
{
    if (menu->id == NULL)
	return;

    if (menu_is_separator(menu->name))
	grey = TRUE;

    gui_mch_menu_hidden(menu, FALSE);
    // Be clever about bitfields versus true booleans here!
    if (!gtk_widget_get_sensitive(menu->id) == !grey)
    {
	gtk_widget_set_sensitive(menu->id, !grey);
	gui_mch_update();
    }
}

/*
 * Make menu item hidden or not hidden.
 */
    void
gui_mch_menu_hidden(vimmenu_T *menu, int hidden)
{
    if (menu->id == 0)
	return;

    if (hidden)
    {
	if (gtk_widget_get_visible(menu->id))
	{
	    gtk_widget_hide(menu->id);
	    gui_mch_update();
	}
    }
    else
    {
	if (!gtk_widget_get_visible(menu->id))
	{
	    gtk_widget_show(menu->id);
	    gui_mch_update();
	}
    }
}

/*
 * This is called after setting all the menus to grey/hidden or not.
 */
    void
gui_mch_draw_menubar(void)
{
    // just make sure that the visual changes get effect immediately
    gui_mch_update();
}
#endif // FEAT_MENU

/*
 * Scrollbar stuff.
 */
    void
gui_mch_enable_scrollbar(scrollbar_T *sb, int flag)
{
    if (sb->id == NULL)
	return;

    gtk_widget_set_visible(sb->id, flag);
    update_window_manager_hints(0, 0);
}


/*
 * Return the RGB value of a pixel as long.
 */
    guicolor_T
gui_mch_get_rgb(guicolor_T pixel)
{
#if GTK_CHECK_VERSION(3,0,0)
    return (long_u)pixel;
#else
    GdkColor color;

    gdk_colormap_query_color(gtk_widget_get_colormap(gui.drawarea),
			     (unsigned long)pixel, &color);

    return (guicolor_T)(
	    (((unsigned)color.red   & 0xff00) << 8)
	 |  ((unsigned)color.green & 0xff00)
	 | (((unsigned)color.blue  & 0xff00) >> 8));
#endif
}

/*
 * Get current mouse coordinates in text window.
 */
    void
gui_mch_getmouse(int *x, int *y)
{
    gui_gtk_get_pointer(gui.drawarea, x, y, NULL);
}

    void
gui_mch_setmouse(int x, int y)
{
    // Sorry for the Xlib call, but we can't avoid it, since there is no
    // internal GDK mechanism present to accomplish this.  (and for good
    // reason...)
    Display * dpy = gui_mch_get_display();
    if (dpy)
	XWarpPointer(dpy, (Window)0,
		GDK_WINDOW_XID(gtk_widget_get_window(gui.drawarea)),
		0, 0, 0U, 0U, x, y);
}


#ifdef FEAT_MOUSESHAPE
// The last set mouse pointer shape is remembered, to be used when it goes
// from hidden to not hidden.
static int last_shape = 0;
#endif

/*
 * Use the blank mouse pointer or not.
 *
 * hide: TRUE = use blank ptr, FALSE = use parent ptr
 */
    void
gui_mch_mousehide(int hide)
{
    if (gui.pointer_hidden == hide)
	return;

    gui.pointer_hidden = hide;
    if (gtk_widget_get_window(gui.drawarea) && gui.blank_pointer != NULL)
    {
	if (hide)
	    gdk_window_set_cursor(gtk_widget_get_window(gui.drawarea),
		    gui.blank_pointer);
	else
#ifdef FEAT_MOUSESHAPE
	    mch_set_mouse_shape(last_shape);
#else
	gdk_window_set_cursor(gtk_widget_get_window(gui.drawarea), NULL);
#endif
    }
}

#if defined(FEAT_MOUSESHAPE) || defined(PROTO)

// Table for shape IDs.  Keep in sync with the mshape_names[] table in
// misc2.c!
static const int mshape_ids[] =
{
    GDK_LEFT_PTR,		// arrow
    GDK_CURSOR_IS_PIXMAP,	// blank
    GDK_XTERM,			// beam
    GDK_SB_V_DOUBLE_ARROW,	// updown
    GDK_SIZING,			// udsizing
    GDK_SB_H_DOUBLE_ARROW,	// leftright
    GDK_SIZING,			// lrsizing
    GDK_WATCH,			// busy
    GDK_X_CURSOR,		// no
    GDK_CROSSHAIR,		// crosshair
    GDK_HAND1,			// hand1
    GDK_HAND2,			// hand2
    GDK_PENCIL,			// pencil
    GDK_QUESTION_ARROW,		// question
    GDK_RIGHT_PTR,		// right-arrow
    GDK_CENTER_PTR,		// up-arrow
    GDK_LEFT_PTR		// last one
};

    void
mch_set_mouse_shape(int shape)
{
    int		   id;
    GdkCursor	   *c;

    if (gtk_widget_get_window(gui.drawarea) == NULL)
	return;

    if (shape == MSHAPE_HIDE || gui.pointer_hidden)
	gdk_window_set_cursor(gtk_widget_get_window(gui.drawarea),
		gui.blank_pointer);
    else
    {
	if (shape >= MSHAPE_NUMBERED)
	{
	    id = shape - MSHAPE_NUMBERED;
	    if (id >= GDK_LAST_CURSOR)
		id = GDK_LEFT_PTR;
	    else
		id &= ~1;	// they are always even (why?)
	}
	else if (shape < (int)ARRAY_LENGTH(mshape_ids))
	    id = mshape_ids[shape];
	else
	    return;
	c = gdk_cursor_new_for_display(
		gtk_widget_get_display(gui.drawarea), (GdkCursorType)id);
	gdk_window_set_cursor(gtk_widget_get_window(gui.drawarea), c);
# if GTK_CHECK_VERSION(3,0,0)
	g_object_unref(G_OBJECT(c));
# else
	gdk_cursor_destroy(c); // Unref, actually.  Bloody GTK+ 1.
# endif
    }
    if (shape != MSHAPE_HIDE)
	last_shape = shape;
}
#endif // FEAT_MOUSESHAPE


#if defined(FEAT_SIGN_ICONS) || defined(PROTO)
/*
 * Signs are currently always 2 chars wide.  With GTK+ 2, the image will be
 * scaled down if the current font is not big enough, or scaled up if the image
 * size is less than 3/4 of the maximum sign size.  With GTK+ 1, the pixmap
 * will be cut off if the current font is not big enough, or centered if it's
 * too small.
 */
# define SIGN_WIDTH  (2 * gui.char_width)
# define SIGN_HEIGHT (gui.char_height)
# define SIGN_ASPECT ((double)SIGN_HEIGHT / (double)SIGN_WIDTH)

    void
gui_mch_drawsign(int row, int col, int typenr)
{
    GdkPixbuf *sign;

    sign = (GdkPixbuf *)sign_get_image(typenr);

    if (sign == NULL || gui.drawarea == NULL
	    || gtk_widget_get_window(gui.drawarea) == NULL)
	return;

    int width;
    int height;
    int xoffset;
    int yoffset;
    int need_scale;

    width  = gdk_pixbuf_get_width(sign);
    height = gdk_pixbuf_get_height(sign);
    /*
     * Decide whether we need to scale.  Allow one pixel of border
     * width to be cut off, in order to avoid excessive scaling for
     * tiny differences in font size.
     * Do scale to fit the height to avoid gaps because of linespacing.
     */
    need_scale = (width > SIGN_WIDTH + 2
	    || height != SIGN_HEIGHT
	    || (width < 3 * SIGN_WIDTH / 4
		&& height < 3 * SIGN_HEIGHT / 4));
    if (need_scale)
    {
	double  aspect;
	int	    w = width;
	int	    h = height;

	// Keep the original aspect ratio
	aspect = (double)height / (double)width;
	width  = (double)SIGN_WIDTH * SIGN_ASPECT / aspect;
	width  = MIN(width, SIGN_WIDTH);
	if (((double)(MAX(height, SIGN_HEIGHT)) /
		    (double)(MIN(height, SIGN_HEIGHT))) < 1.15)
	{
	    // Change the aspect ratio by at most 15% to fill the
	    // available space completely.
	    height = (double)SIGN_HEIGHT * SIGN_ASPECT / aspect;
	    height = MIN(height, SIGN_HEIGHT);
	}
	else
	    height = (double)width * aspect;

	if (w == width && h == height)
	{
	    // no change in dimensions; don't decrease reference counter
	    // (below)
	    need_scale = FALSE;
	}
	else
	{
	    // This doesn't seem to be worth caching, and doing so would
	    // complicate the code quite a bit.
	    sign = gdk_pixbuf_scale_simple(sign, width, height,
		    GDK_INTERP_BILINEAR);
	    if (sign == NULL)
		return; // out of memory
	}
    }

    // The origin is the upper-left corner of the pixmap.  Therefore
    // these offset may become negative if the pixmap is smaller than
    // the 2x1 cells reserved for the sign icon.
    xoffset = (width  - SIGN_WIDTH)  / 2;
    yoffset = (height - SIGN_HEIGHT) / 2;

# if GTK_CHECK_VERSION(3,0,0)
    {
	cairo_t	    *cr;
	cairo_surface_t *bg_surf;
	cairo_t	    *bg_cr;
	cairo_surface_t *sign_surf;
	cairo_t	    *sign_cr;

	cr = cairo_create(gui.surface);

	bg_surf = cairo_surface_create_similar(gui.surface,
		cairo_surface_get_content(gui.surface),
		SIGN_WIDTH, SIGN_HEIGHT);
	bg_cr = cairo_create(bg_surf);
	cairo_set_source_rgba(bg_cr,
		gui.bgcolor->red, gui.bgcolor->green, gui.bgcolor->blue,
		gui.bgcolor->alpha);
	cairo_paint(bg_cr);

	sign_surf = cairo_surface_create_similar(gui.surface,
		cairo_surface_get_content(gui.surface),
		SIGN_WIDTH, SIGN_HEIGHT);
	sign_cr = cairo_create(sign_surf);
	gdk_cairo_set_source_pixbuf(sign_cr, sign, -xoffset, -yoffset);
	cairo_paint(sign_cr);

	cairo_set_operator(sign_cr, CAIRO_OPERATOR_DEST_OVER);
	cairo_set_source_surface(sign_cr, bg_surf, 0, 0);
	cairo_paint(sign_cr);

	cairo_set_source_surface(cr, sign_surf, FILL_X(col), FILL_Y(row));
	cairo_paint(cr);

	cairo_destroy(sign_cr);
	cairo_surface_destroy(sign_surf);
	cairo_destroy(bg_cr);
	cairo_surface_destroy(bg_surf);
	cairo_destroy(cr);

	gtk_widget_queue_draw_area(gui.drawarea,
		FILL_X(col), FILL_Y(col), width, height);

    }
# else // !GTK_CHECK_VERSION(3,0,0)
    gdk_gc_set_foreground(gui.text_gc, gui.bgcolor);

    gdk_draw_rectangle(gui.drawarea->window,
	    gui.text_gc,
	    TRUE,
	    FILL_X(col),
	    FILL_Y(row),
	    SIGN_WIDTH,
	    SIGN_HEIGHT);

    gdk_pixbuf_render_to_drawable_alpha(sign,
	    gui.drawarea->window,
	    MAX(0, xoffset),
	    MAX(0, yoffset),
	    FILL_X(col) - MIN(0, xoffset),
	    FILL_Y(row) - MIN(0, yoffset),
	    MIN(width,	SIGN_WIDTH),
	    MIN(height, SIGN_HEIGHT),
	    GDK_PIXBUF_ALPHA_BILEVEL,
	    127,
	    GDK_RGB_DITHER_NORMAL,
	    0, 0);
# endif // !GTK_CHECK_VERSION(3,0,0)
    if (need_scale)
	g_object_unref(sign);
}

    void *
gui_mch_register_sign(char_u *signfile)
{
    if (signfile[0] != NUL && signfile[0] != '-' && gui.in_use)
    {
	GdkPixbuf   *sign;
	GError	    *error = NULL;
	char_u	    *message;

	sign = gdk_pixbuf_new_from_file((const char *)signfile, &error);

	if (error == NULL)
	    return sign;

	message = (char_u *)error->message;

	if (message != NULL && input_conv.vc_type != CONV_NONE)
	    message = string_convert(&input_conv, message, NULL);

	if (message != NULL)
	{
	    // The error message is already translated and will be more
	    // descriptive than anything we could possibly do ourselves.
	    semsg("E255: %s", message);

	    if (input_conv.vc_type != CONV_NONE)
		vim_free(message);
	}
	g_error_free(error);
    }

    return NULL;
}

    void
gui_mch_destroy_sign(void *sign)
{
    if (sign != NULL)
	g_object_unref(sign);
}

#endif // FEAT_SIGN_ICONS
