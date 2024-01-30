/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

/*
 * proto.h: include the (automatically generated) function prototypes
 */

/*
 * Don't include these while generating prototypes.  Prevents problems when
 * files are missing.
 */
#if !defined(PROTO) && !defined(NOPROTO)

/*
 * Machine-dependent routines.
 */
// avoid errors in function prototypes
# if !defined(FEAT_X11) && !defined(FEAT_GUI_GTK)
#  define Display int
#  define Widget int
# endif
# ifndef FEAT_GUI_GTK
#  define GdkEvent int
#  define GdkEventKey int
# endif
# ifndef FEAT_X11
#  define XImage int
# endif

# ifdef AMIGA
#  include "os_amiga.pro"
# endif
# if defined(UNIX) || defined(VMS)
#  include "os_unix.pro"
# endif
# ifdef MSWIN
#  include "os_win32.pro"
#  include "os_mswin.pro"
#  include "winclip.pro"
#  if (defined(__GNUC__) && !defined(__MINGW32__))
extern int _stricoll(char *a, char *b);
#  endif
# endif
# ifdef VMS
#  include "os_vms.pro"
# endif
# ifdef __QNX__
#  include "os_qnx.pro"
# endif

# ifdef FEAT_CRYPT
#  include "blowfish.pro"
#  include "crypt.pro"
#  include "crypt_zip.pro"
# endif
# include "alloc.pro"
# include "arglist.pro"
# include "autocmd.pro"
# include "buffer.pro"
# include "bufwrite.pro"
# include "change.pro"
# include "charset.pro"
# include "cindent.pro"
# include "clientserver.pro"
# include "clipboard.pro"
# include "cmdexpand.pro"
# include "cmdhist.pro"
# include "if_cscope.pro"
# include "debugger.pro"
# include "dict.pro"
# include "diff.pro"
# include "digraph.pro"
# include "drawline.pro"
# include "drawscreen.pro"
# include "edit.pro"
# include "eval.pro"
# include "evalbuffer.pro"
# include "evalfunc.pro"
# include "evalvars.pro"
# include "evalwindow.pro"
# include "ex_cmds.pro"
# include "ex_cmds2.pro"
# include "ex_docmd.pro"
# include "ex_eval.pro"
# include "ex_getln.pro"
# include "fileio.pro"
# include "filepath.pro"
# include "findfile.pro"
# include "float.pro"
# include "fold.pro"
# include "getchar.pro"
# include "gui_xim.pro"
# include "hardcopy.pro"
# include "hashtab.pro"
# include "help.pro"
# include "highlight.pro"
# include "indent.pro"
# include "insexpand.pro"
# include "json.pro"
# include "list.pro"
# include "locale.pro"
# include "logfile.pro"
# include "blob.pro"
# include "main.pro"
# include "map.pro"
# include "mark.pro"
# include "match.pro"
# include "memfile.pro"
# include "memline.pro"
# ifdef FEAT_MENU
#  include "menu.pro"
# endif
# ifdef FEAT_ARABIC
#  include "arabic.pro"
# endif
# ifdef FEAT_VIMINFO
#  include "viminfo.pro"
# endif

// These prototypes cannot be produced automatically.
int smsg(const char *, ...) ATTRIBUTE_COLD ATTRIBUTE_FORMAT_PRINTF(1, 2);

int smsg_attr(int, const char *, ...) ATTRIBUTE_FORMAT_PRINTF(2, 3);

int smsg_attr_keep(int, const char *, ...) ATTRIBUTE_FORMAT_PRINTF(2, 3);

// These prototypes cannot be produced automatically.
int semsg(const char *, ...) ATTRIBUTE_COLD ATTRIBUTE_FORMAT_PRINTF(1, 2);

// These prototypes cannot be produced automatically.
void siemsg(const char *, ...) ATTRIBUTE_COLD ATTRIBUTE_FORMAT_PRINTF(1, 2);

int vim_snprintf_add(char *, size_t, const char *, ...) ATTRIBUTE_FORMAT_PRINTF(3, 4);

int vim_snprintf(char *, size_t, const char *, ...) ATTRIBUTE_FORMAT_PRINTF(3, 4);

int vim_vsnprintf(char *str, size_t str_m, const char *fmt, va_list ap)
	ATTRIBUTE_FORMAT_PRINTF(3, 0);
int vim_vsnprintf_typval(char *str, size_t str_m, const char *fmt, va_list ap, typval_T *tvs)
	ATTRIBUTE_FORMAT_PRINTF(3, 0);

# include "message.pro"
# include "misc1.pro"
# include "misc2.pro"
# ifndef HAVE_STRPBRK	    // not generated automatically from misc2.c
char_u *vim_strpbrk(char_u *s, char_u *charset);
# endif
# ifndef HAVE_QSORT
// Use our own qsort(), don't define the prototype when not used.
void qsort(void *base, size_t elm_count, size_t elm_size, int (*cmp)(const void *, const void *));
# endif
# include "mouse.pro"
# include "move.pro"
# include "mbyte.pro"
# ifdef VIMDLL
// Function name differs when VIMDLL is defined
int mbyte_im_get_status(void);
void mbyte_im_set_active(int active_arg);
# endif
# include "normal.pro"
# include "ops.pro"
# include "option.pro"
# include "optionstr.pro"
# include "popupmenu.pro"
# if defined(FEAT_PROFILE) || defined(FEAT_RELTIME)
#  include "profiler.pro"
# endif
# include "quickfix.pro"
# include "regexp.pro"
# include "register.pro"
# include "scriptfile.pro"
# include "screen.pro"
# include "session.pro"
# if defined(FEAT_CRYPT) || defined(FEAT_PERSISTENT_UNDO)
#  include "sha256.pro"
# endif
# include "search.pro"
# ifdef FEAT_SIGNS
#  include "sign.pro"
# endif
# include "sound.pro"
# include "spell.pro"
# include "spellfile.pro"
# include "spellsuggest.pro"
# include "strings.pro"
# include "syntax.pro"
# include "tag.pro"
# include "term.pro"
# ifdef FEAT_TERMINAL
#  include "terminal.pro"
# endif
# if defined(HAVE_TGETENT) && (defined(AMIGA) || defined(VMS))
#  include "termlib.pro"
# endif
# ifdef FEAT_PROP_POPUP
#  include "popupwin.pro"
#  include "textprop.pro"
# endif
# include "testing.pro"
# include "textobject.pro"
# include "textformat.pro"
# include "time.pro"
# include "typval.pro"
# include "ui.pro"
# include "undo.pro"
# include "usercmd.pro"
# include "userfunc.pro"
# include "version.pro"
# include "vim9script.pro"
# ifdef FEAT_EVAL
// include vim9.h here, the types defined there are used by function arguments.
#  include "vim9.h"
#  include "vim9class.pro"
#  include "vim9cmds.pro"
#  include "vim9compile.pro"
#  include "vim9execute.pro"
#  include "vim9expr.pro"
#  include "vim9instr.pro"
#  include "vim9type.pro"
# endif
# include "window.pro"

# ifdef FEAT_LUA
#  include "if_lua.pro"
# endif

# ifdef FEAT_MZSCHEME
#  include "if_mzsch.pro"
# endif

# ifdef FEAT_PYTHON
#  include "if_python.pro"
# endif

# ifdef FEAT_PYTHON3
#  include "if_python3.pro"
# endif

# ifdef FEAT_TCL
#  include "if_tcl.pro"
# endif

# ifdef FEAT_RUBY
#  include "if_ruby.pro"
# endif

// Ugly solution for "BalloonEval" not being defined while it's used in some
// .pro files.
# ifndef FEAT_BEVAL
#  define BalloonEval int
# endif
# if defined(FEAT_BEVAL) || defined(FEAT_PROP_POPUP)
#  include "beval.pro"
# endif

# ifdef FEAT_NETBEANS_INTG
#  include "netbeans.pro"
# endif
# ifdef FEAT_JOB_CHANNEL
#  include "job.pro"
#  include "channel.pro"
# endif

# ifdef FEAT_EVAL
// Not generated automatically so that we can add an extra attribute.
void ch_log(channel_T *ch, const char *fmt, ...) ATTRIBUTE_FORMAT_PRINTF(2, 3);
void ch_error(channel_T *ch, const char *fmt, ...) ATTRIBUTE_FORMAT_PRINTF(2, 3);
# endif

# if defined(FEAT_GUI) || defined(FEAT_JOB_CHANNEL)
#  if defined(UNIX) || defined(MACOS_X) || defined(VMS)
#   include "pty.pro"
#  endif
# endif

# ifdef FEAT_GUI
#  include "gui.pro"
#  if !defined(HAVE_SETENV) && !defined(HAVE_PUTENV) && !defined(VMS)
extern int putenv(const char *string);			// in misc2.c
#   ifdef USE_VIMPTY_GETENV
extern char_u *vimpty_getenv(const char_u *string);	// in misc2.c
#   endif
#  endif
#  ifdef FEAT_GUI_MSWIN
#   include "gui_w32.pro"
#  endif
#  ifdef FEAT_GUI_GTK
#   include "gui_gtk.pro"
#   include "gui_gtk_x11.pro"
#  endif
#  ifdef FEAT_GUI_MOTIF
#   include "gui_motif.pro"
#   include "gui_xmdlg.pro"
#  endif
#  ifdef FEAT_GUI_HAIKU
#   include "gui_haiku.pro"
#  endif
#  ifdef FEAT_GUI_X11
#   include "gui_x11.pro"
#  endif
#  ifdef FEAT_GUI_PHOTON
#   include "gui_photon.pro"
#  endif
# endif	// FEAT_GUI

# ifdef FEAT_OLE
#  include "if_ole.pro"
# endif
# if defined(FEAT_CLIENTSERVER) && defined(FEAT_X11)
#  include "if_xcmdsrv.pro"
# endif

/*
 * The perl include files pollute the namespace, therefore proto.h must be
 * included before the perl include files.  But then CV is not defined, which
 * is used in if_perl.pro.  To get around this, the perl prototype files are
 * not included here for the perl files.  Use a dummy define for CV for the
 * other files.
 */
# if defined(FEAT_PERL) && !defined(IN_PERL_FILE)
#  define CV void
#  include "if_perl.pro"
#  include "if_perlsfio.pro"
# endif

# ifdef MACOS_CONVERT
#  include "os_mac_conv.pro"
# endif
# ifdef MACOS_X
#  include "os_macosx.pro"
# endif
# if defined(MACOS_X_DARWIN) && defined(FEAT_CLIPBOARD) && !defined(FEAT_GUI)
// functions in os_macosx.m
void clip_mch_lose_selection(Clipboard_T *cbd);
int clip_mch_own_selection(Clipboard_T *cbd);
void clip_mch_request_selection(Clipboard_T *cbd);
void clip_mch_set_selection(Clipboard_T *cbd);
# endif
#endif // !PROTO && !NOPROTO
