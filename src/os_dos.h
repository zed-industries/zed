/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

/*
 * Common MS-DOS and Win32 (Windows NT and Windows 95) defines.
 *
 * Names for the EXRC, HELP and temporary files.
 * Some of these may have been defined in the makefile or feature.h.
 */

#ifndef SYS_VIMRC_FILE
# define SYS_VIMRC_FILE		"$VIM\\vimrc"
#endif
#ifndef USR_VIMRC_FILE
# define USR_VIMRC_FILE		"$HOME\\_vimrc"
#endif
#ifndef USR_VIMRC_FILE2
# define USR_VIMRC_FILE2	"$HOME\\vimfiles\\vimrc"
#endif
#ifndef USR_VIMRC_FILE3
# define USR_VIMRC_FILE3	"$VIM\\_vimrc"
#endif
#ifndef VIM_DEFAULTS_FILE
# define VIM_DEFAULTS_FILE	"$VIMRUNTIME\\defaults.vim"
#endif
#ifndef EVIM_FILE
# define EVIM_FILE		"$VIMRUNTIME\\evim.vim"
#endif

#ifndef USR_EXRC_FILE
# define USR_EXRC_FILE		"$HOME\\_exrc"
#endif
#ifndef USR_EXRC_FILE2
# define USR_EXRC_FILE2		"$VIM\\_exrc"
#endif

#ifdef FEAT_GUI
# ifndef SYS_GVIMRC_FILE
#  define SYS_GVIMRC_FILE	"$VIM\\gvimrc"
# endif
# ifndef USR_GVIMRC_FILE
#  define USR_GVIMRC_FILE	"$HOME\\_gvimrc"
# endif
# ifndef USR_GVIMRC_FILE2
#  define USR_GVIMRC_FILE2	"$HOME\\vimfiles\\gvimrc"
# endif
# ifndef USR_GVIMRC_FILE3
#  define USR_GVIMRC_FILE3	"$VIM\\_gvimrc"
# endif
# ifndef SYS_MENU_FILE
#  define SYS_MENU_FILE		"$VIMRUNTIME\\menu.vim"
# endif
#endif

#ifndef SYS_OPTWIN_FILE
# define SYS_OPTWIN_FILE	"$VIMRUNTIME\\optwin.vim"
#endif

#ifdef FEAT_VIMINFO
# ifndef VIMINFO_FILE
#  define VIMINFO_FILE		"$HOME\\_viminfo"
# endif
# ifndef VIMINFO_FILE2
#  define VIMINFO_FILE2		"$VIM\\_viminfo"
# endif
#endif

#ifndef VIMRC_FILE
# define VIMRC_FILE	"_vimrc"
#endif

#ifndef EXRC_FILE
# define EXRC_FILE	"_exrc"
#endif

#ifdef FEAT_GUI
# ifndef GVIMRC_FILE
#  define GVIMRC_FILE	"_gvimrc"
# endif
#endif

#ifndef DFLT_HELPFILE
# define DFLT_HELPFILE	"$VIMRUNTIME\\doc\\help.txt"
#endif

#ifndef SYNTAX_FNAME
# define SYNTAX_FNAME	"$VIMRUNTIME\\syntax\\%s.vim"
#endif

#ifndef DFLT_BDIR
# define DFLT_BDIR	".,$TEMP,c:\\tmp,c:\\temp" // default for 'backupdir'
#endif

#ifndef DFLT_VDIR
# define DFLT_VDIR	"$HOME/vimfiles/view"	// default for 'viewdir'
#endif

#ifndef DFLT_DIR
# define DFLT_DIR	".,$TEMP,c:\\tmp,c:\\temp" // default for 'directory'
#endif

#define DFLT_ERRORFILE		"errors.err"
#define DFLT_RUNTIMEPATH	"$HOME/vimfiles,$VIM/vimfiles,$VIMRUNTIME,$HOME/vimfiles/after,$VIM/vimfiles/after"
#define CLEAN_RUNTIMEPATH	"$VIM/vimfiles,$VIMRUNTIME,$VIM/vimfiles/after"

#define CASE_INSENSITIVE_FILENAME   // ignore case when comparing file names
#define SPACE_IN_FILENAME
#define BACKSLASH_IN_FILENAME
#define USE_CRNL		// lines end in CR-NL instead of NL
#define HAVE_DUP		// have dup()
#define HAVE_ST_MODE		// have stat.st_mode
