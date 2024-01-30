/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

/*
 * option.h: definition of global variables for settable options
 */

#ifndef _OPTION_H_
#define _OPTION_H_

//
// Option Flags
//
#define P_BOOL		0x01	// the option is boolean
#define P_NUM		0x02	// the option is numeric
#define P_STRING	0x04	// the option is a string
#define P_ALLOCED	0x08	// the string option is in allocated memory,
				// must use free_string_option() when
				// assigning new value. Not set if default is
				// the same.
#define P_EXPAND	0x10	// environment expansion.  NOTE: P_EXPAND can
				// never be used for local or hidden options!
#define P_NO_CMD_EXPAND	0x20	// don't perform cmdline completions
#define P_NODEFAULT	0x40	// don't set to default value
#define P_DEF_ALLOCED	0x80	// default value is in allocated memory, must
				//  use vim_free() when assigning new value
#define P_WAS_SET	0x100	// option has been set/reset
#define P_NO_MKRC	0x200	// don't include in :mkvimrc output
#define P_VI_DEF	0x400	// Use Vi default for Vim
#define P_VIM		0x800	// Vim option, reset when 'cp' set

				// when option changed, what to display:
#define P_RSTAT		0x1000	// redraw status lines
#define P_RWIN		0x2000	// redraw current window and recompute text
#define P_RBUF		0x4000	// redraw current buffer and recompute text
#define P_RALL		0x6000	// redraw all windows
#define P_RCLR		0x7000	// clear and redraw all

#define P_COMMA		 0x8000	 // comma separated list
#define P_ONECOMMA	0x18000L // P_COMMA and cannot have two consecutive
				 // commas
#define P_NODUP		0x20000L // don't allow duplicate strings
#define P_FLAGLIST	0x40000L // list of single-char flags

#define P_SECURE	0x80000L // cannot change in modeline or secure mode
#define P_GETTEXT      0x100000L // expand default value with _()
#define P_NOGLOB       0x200000L // do not use local value for global vimrc
#define P_NFNAME       0x400000L // only normal file name chars allowed
#define P_INSECURE     0x800000L // option was set from a modeline
#define P_PRI_MKRC    0x1000000L // priority for :mkvimrc (setting option has
				 // side effects)
#define P_NO_ML       0x2000000L // not allowed in modeline
#define P_CURSWANT    0x4000000L // update curswant required; not needed when
				 // there is a redraw flag
#define P_NDNAME      0x8000000L // only normal dir name chars allowed
#define P_RWINONLY   0x10000000L // only redraw current window
#define P_MLE	     0x20000000L // under control of 'modelineexpr'
#define P_FUNC	     0x40000000L // accept a function reference or a lambda
#define P_COLON	     0x80000000L // values use colons to create sublists
// Warning: Currently we have used all 32 bits for option flags. On some 32-bit
//          systems, the flags are stored as a 32-bit integer, and adding more
//          flags will overflow it. Adding another flag will need to change how
//          it's stored first.

// Returned by get_option_value().
typedef enum {
    gov_unknown,
    gov_bool,
    gov_number,
    gov_string,
    gov_hidden_bool,
    gov_hidden_number,
    gov_hidden_string
} getoption_T;

/*
 * Default values for 'errorformat'.
 * The "%f|%l| %m" one is used for when the contents of the quickfix window is
 * written to a file.
 */
#ifdef AMIGA
# define DFLT_EFM	"%f>%l:%c:%t:%n:%m,%f:%l: %t%*\\D%n: %m,%f %l %t%*\\D%n: %m,%*[^\"]\"%f\"%*\\D%l: %m,%f:%l:%m,%f|%l| %m"
#else
# if defined(MSWIN)
#  define DFLT_EFM	"%f(%l): %t%*\\D%n: %m,%f(%l\\,%c): %t%*\\D%n: %m,%f(%l) \\=: %t%*\\D%n: %m,%*[^\"]\"%f\"%*\\D%l: %m,%f(%l) \\=: %m,%*[^ ] %f %l: %m,%f:%l:%c:%m,%f(%l):%m,%f:%l:%m,%f|%l| %m"
# else
#  if defined(__QNX__)
#   define DFLT_EFM	"%f(%l):%*[^WE]%t%*\\D%n:%m,%f|%l| %m"
#  else
#   ifdef VMS
#    define DFLT_EFM	"%A%p^,%C%%CC-%t-%m,%Cat line number %l in file %f,%f|%l| %m"
#   else // Unix, probably
#define DFLT_EFM	"%*[^\"]\"%f\"%*\\D%l: %m,\"%f\"%*\\D%l: %m,%-Gg%\\?make[%*\\d]: *** [%f:%l:%m,%-Gg%\\?make: *** [%f:%l:%m,%-G%f:%l: (Each undeclared identifier is reported only once,%-G%f:%l: for each function it appears in.),%-GIn file included from %f:%l:%c:,%-GIn file included from %f:%l:%c\\,,%-GIn file included from %f:%l:%c,%-GIn file included from %f:%l,%-G%*[ ]from %f:%l:%c,%-G%*[ ]from %f:%l:,%-G%*[ ]from %f:%l\\,,%-G%*[ ]from %f:%l,%f:%l:%c:%m,%f(%l):%m,%f:%l:%m,\"%f\"\\, line %l%*\\D%c%*[^ ] %m,%D%*\\a[%*\\d]: Entering directory %*[`']%f',%X%*\\a[%*\\d]: Leaving directory %*[`']%f',%D%*\\a: Entering directory %*[`']%f',%X%*\\a: Leaving directory %*[`']%f',%DMaking %*\\a in %f,%f|%l| %m"
#   endif
#  endif
# endif
#endif

#define DFLT_GREPFORMAT	"%f:%l:%m,%f:%l%m,%f  %l%m"

// default values for b_p_ff 'fileformat' and p_ffs 'fileformats'
#define FF_DOS		"dos"
#define FF_MAC		"mac"
#define FF_UNIX		"unix"

#ifdef USE_CRNL
# define DFLT_FF	"dos"
# define DFLT_FFS_VIM	"dos,unix"
# define DFLT_FFS_VI	"dos,unix"	// also autodetect in compatible mode
# define DFLT_TEXTAUTO	TRUE
#else
# define DFLT_FF	"unix"
# define DFLT_FFS_VIM	"unix,dos"
# ifdef __CYGWIN__
#  define DFLT_FFS_VI	"unix,dos"	// Cygwin always needs file detection
#  define DFLT_TEXTAUTO TRUE
# else
#  define DFLT_FFS_VI	""
#  define DFLT_TEXTAUTO FALSE
# endif
#endif


// Possible values for 'encoding'
#define ENC_UCSBOM	"ucs-bom"	// check for BOM at start of file

// default value for 'encoding'
#if defined(MSWIN) || defined(__MVS__)
# define ENC_DFLT	"utf-8"
#else
# define ENC_DFLT	"latin1"
#endif

// end-of-line style
#define EOL_UNKNOWN	(-1)	// not defined yet
#define EOL_UNIX	0	// NL
#define EOL_DOS		1	// CR NL
#define EOL_MAC		2	// CR

// Formatting options for p_fo 'formatoptions'
#define FO_WRAP		't'
#define FO_WRAP_COMS	'c'
#define FO_RET_COMS	'r'
#define FO_OPEN_COMS	'o'
#define FO_NO_OPEN_COMS	'/'
#define FO_Q_COMS	'q'
#define FO_Q_NUMBER	'n'
#define FO_Q_SECOND	'2'
#define FO_INS_VI	'v'
#define FO_INS_LONG	'l'
#define FO_INS_BLANK	'b'
#define FO_MBYTE_BREAK	'm'	// break before/after multi-byte char
#define FO_MBYTE_JOIN	'M'	// no space before/after multi-byte char
#define FO_MBYTE_JOIN2	'B'	// no space between multi-byte chars
#define FO_ONE_LETTER	'1'
#define FO_WHITE_PAR	'w'	// trailing white space continues paragr.
#define FO_AUTO		'a'	// automatic formatting
#define FO_RIGOROUS_TW	']'     // respect textwidth rigorously
#define FO_REMOVE_COMS	'j'	// remove comment leaders when joining lines
#define FO_PERIOD_ABBR	'p'	// don't break a single space after a period

#define DFLT_FO_VI	"vt"
#define DFLT_FO_VIM	"tcq"
#define FO_ALL		"tcro/q2vlb1mMBn,aw]jp"	// for do_set()

// characters for the p_cpo option:
#define CPO_ALTREAD	'a'	// ":read" sets alternate file name
#define CPO_ALTWRITE	'A'	// ":write" sets alternate file name
#define CPO_BAR		'b'	// "\|" ends a mapping
#define CPO_BSLASH	'B'	// backslash in mapping is not special
#define CPO_SEARCH	'c'
#define CPO_CONCAT	'C'	// Don't concatenate sourced lines
#define CPO_DOTTAG	'd'	// "./tags" in 'tags' is in current dir
#define CPO_DIGRAPH	'D'	// No digraph after "r", "f", etc.
#define CPO_EXECBUF	'e'
#define CPO_EMPTYREGION	'E'	// operating on empty region is an error
#define CPO_FNAMER	'f'	// set file name for ":r file"
#define CPO_FNAMEW	'F'	// set file name for ":w file"
#define CPO_GOTO1	'g'	// goto line 1 for ":edit"
#define CPO_INSEND	'H'	// "I" inserts before last blank in line
#define CPO_INTMOD	'i'	// interrupt a read makes buffer modified
#define CPO_INDENT	'I'	// remove auto-indent more often
#define CPO_JOINSP	'j'	// only use two spaces for join after '.'
#define CPO_ENDOFSENT	'J'	// need two spaces to detect end of sentence
#define CPO_KEYCODE	'k'	// don't recognize raw key code in mappings
#define CPO_KOFFSET	'K'	// don't wait for key code in mappings
#define CPO_LITERAL	'l'	// take char after backslash in [] literal
#define CPO_LISTWM	'L'	// 'list' changes wrapmargin
#define CPO_SHOWMATCH	'm'
#define CPO_MATCHBSL	'M'	// "%" ignores use of backslashes
#define CPO_NUMCOL	'n'	// 'number' column also used for text
#define CPO_LINEOFF	'o'
#define CPO_OVERNEW	'O'	// silently overwrite new file
#define CPO_LISP	'p'	// 'lisp' indenting
#define CPO_FNAMEAPP	'P'	// set file name for ":w >>file"
#define CPO_JOINCOL	'q'	// with "3J" use column after first join
#define CPO_REDO	'r'
#define CPO_REMMARK	'R'	// remove marks when filtering
#define CPO_BUFOPT	's'
#define CPO_BUFOPTGLOB	'S'
#define CPO_TAGPAT	't'	// tag pattern is used for "n"
#define CPO_UNDO	'u'	// "u" undoes itself
#define CPO_BACKSPACE	'v'	// "v" keep deleted text
#define CPO_CW		'w'	// "cw" only changes one blank
#define CPO_FWRITE	'W'	// "w!" doesn't overwrite readonly files
#define CPO_ESC		'x'
#define CPO_REPLCNT	'X'	// "R" with a count only deletes chars once
#define CPO_YANK	'y'
#define CPO_KEEPRO	'Z'	// don't reset 'readonly' on ":w!"
#define CPO_DOLLAR	'$'
#define CPO_FILTER	'!'
#define CPO_MATCH	'%'
#define CPO_STAR	'*'	// ":*" means ":@"
#define CPO_PLUS	'+'	// ":write file" resets 'modified'
#define CPO_MINUS	'-'	// "9-" fails at and before line 9
#define CPO_SPECI	'<'	// don't recognize <> in mappings
#define CPO_REGAPPEND	'>'	// insert NL when appending to a register
// POSIX flags
#define CPO_HASH	'#'	// "D", "o" and "O" do not use a count
#define CPO_PARA	'{'	// "{" is also a paragraph boundary
#define CPO_TSIZE	'|'	// $LINES and $COLUMNS overrule term size
#define CPO_PRESERVE	'&'	// keep swap file after :preserve
#define CPO_SUBPERCENT	'/'	// % in :s string uses previous one
#define CPO_BACKSL	'\\'	// \ is not special in []
#define CPO_CHDIR	'.'	// don't chdir if buffer is modified
#define CPO_SCOLON	';'	// using "," and ";" will skip over char if
				// cursor would not move
// default values for Vim, Vi and POSIX
#define CPO_VIM		"aABceFs"
#define CPO_VI		"aAbBcCdDeEfFgHiIjJkKlLmMnoOpPqrRsStuvwWxXyZ$!%*-+<>;"
#define CPO_ALL		"aAbBcCdDeEfFgHiIjJkKlLmMnoOpPqrRsStuvwWxXyZ$!%*-+<>#{|&/\\.;"

// characters for p_ww option:
#define WW_ALL		"bshl<>[]~"

// characters for p_mouse option:
#define MOUSE_NORMAL	'n'		// use mouse in Normal mode
#define MOUSE_VISUAL	'v'		// use mouse in Visual/Select mode
#define MOUSE_INSERT	'i'		// use mouse in Insert mode
#define MOUSE_COMMAND	'c'		// use mouse in Command-line mode
#define MOUSE_HELP	'h'		// use mouse in help buffers
#define MOUSE_RETURN	'r'		// use mouse for hit-return message
#define MOUSE_A		"nvich"		// used for 'a' flag
#define MOUSE_ALL	"anvichr"	// all possible characters
#define MOUSE_NONE	' '		// don't use Visual selection
#define MOUSE_NONEF	'x'		// forced modeless selection

#define COCU_ALL	"nvic"		// flags for 'concealcursor'

// characters for p_shm option:
#define SHM_RO		'r'		// readonly
#define SHM_MOD		'm'		// modified
#define SHM_FILE	'f'		// (file 1 of 2)
#define SHM_LAST	'i'		// last line incomplete
#define SHM_TEXT	'x'		// tx instead of textmode
#define SHM_LINES	'l'		// "L" instead of "lines"
#define SHM_NEW		'n'		// "[New]" instead of "[New file]"
#define SHM_WRI		'w'		// "[w]" instead of "written"
#define SHM_A		"rmfixlnw"	// represented by 'a' flag
#define SHM_WRITE	'W'		// don't use "written" at all
#define SHM_TRUNC	't'		// truncate file messages
#define SHM_TRUNCALL	'T'		// truncate all messages
#define SHM_OVER	'o'		// overwrite file messages
#define SHM_OVERALL	'O'		// overwrite more messages
#define SHM_SEARCH	's'		// no search hit bottom messages
#define SHM_ATTENTION	'A'		// no ATTENTION messages
#define SHM_INTRO	'I'		// intro messages
#define SHM_COMPLETIONMENU  'c'		// completion menu messages
#define SHM_COMPLETIONSCAN  'C'		// completion scanning messages
#define SHM_RECORDING	'q'		// short recording message
#define SHM_FILEINFO	'F'		// no file info messages
#define SHM_SEARCHCOUNT  'S'		// no search stats: '[1/10]'
#define SHM_POSIX       "AS"		// POSIX value
#define SHM_ALL		"rmfixlnwaWtToOsAIcCqFS" // all possible flags for 'shm'
#define SHM_LEN		30		// max length of all flags together
					// plus a NUL character

// characters for p_go:
#define GO_TERMINAL	'!'		// use terminal for system commands
#define GO_ASEL		'a'		// autoselect
#define GO_ASELML	'A'		// autoselect modeless selection
#define GO_BOT		'b'		// use bottom scrollbar
#define GO_CONDIALOG	'c'		// use console dialog
#define GO_DARKTHEME	'd'		// use dark theme variant
#define GO_TABLINE	'e'		// may show tabline
#define GO_FORG		'f'		// start GUI in foreground
#define GO_GREY		'g'		// use grey menu items
#define GO_HORSCROLL	'h'		// flexible horizontal scrolling
#define GO_ICON		'i'		// use Vim icon
#define GO_LEFT		'l'		// use left scrollbar
#define GO_VLEFT	'L'		// left scrollbar with vert split
#define GO_MENUS	'm'		// use menu bar
#define GO_NOSYSMENU	'M'		// don't source system menu
#define GO_POINTER	'p'		// pointer enter/leave callbacks
#define GO_ASELPLUS	'P'		// autoselectPlus
#define GO_RIGHT	'r'		// use right scrollbar
#define GO_VRIGHT	'R'		// right scrollbar with vert split
#define GO_TEAROFF	't'		// add tear-off menu items
#define GO_TOOLBAR	'T'		// add toolbar
#define GO_FOOTER	'F'		// add footer
#define GO_VERTICAL	'v'		// arrange dialog buttons vertically
#define GO_KEEPWINSIZE	'k'		// keep GUI window size
// all possible flags for 'go'
#define GO_ALL		"!aAbcdefFghilLmMpPrRtTvk"

// flags for 'comments' option
#define COM_NEST	'n'		// comments strings nest
#define COM_BLANK	'b'		// needs blank after string
#define COM_START	's'		// start of comment
#define COM_MIDDLE	'm'		// middle of comment
#define COM_END		'e'		// end of comment
#define COM_AUTO_END	'x'		// last char of end closes comment
#define COM_FIRST	'f'		// first line comment only
#define COM_LEFT	'l'		// left adjusted
#define COM_RIGHT	'r'		// right adjusted
#define COM_NOBACK	'O'		// don't use for "O" command
#define COM_ALL		"nbsmexflrO"	// all flags for 'comments' option
#define COM_MAX_LEN	50		// maximum length of a part

// flags for 'statusline' option
#define STL_FILEPATH	'f'		// path of file in buffer
#define STL_FULLPATH	'F'		// full path of file in buffer
#define STL_FILENAME	't'		// last part (tail) of file path
#define STL_COLUMN	'c'		// column og cursor
#define STL_VIRTCOL	'v'		// virtual column
#define STL_VIRTCOL_ALT	'V'		// - with 'if different' display
#define STL_LINE	'l'		// line number of cursor
#define STL_NUMLINES	'L'		// number of lines in buffer
#define STL_BUFNO	'n'		// current buffer number
#define STL_KEYMAP	'k'		// 'keymap' when active
#define STL_OFFSET	'o'		// offset of character under cursor
#define STL_OFFSET_X	'O'		// - in hexadecimal
#define STL_BYTEVAL	'b'		// byte value of character
#define STL_BYTEVAL_X	'B'		// - in hexadecimal
#define STL_ROFLAG	'r'		// readonly flag
#define STL_ROFLAG_ALT	'R'		// - other display
#define STL_HELPFLAG	'h'		// window is showing a help file
#define STL_HELPFLAG_ALT 'H'		// - other display
#define STL_FILETYPE	'y'		// 'filetype'
#define STL_FILETYPE_ALT 'Y'		// - other display
#define STL_PREVIEWFLAG	'w'		// window is showing the preview buf
#define STL_PREVIEWFLAG_ALT 'W'		// - other display
#define STL_MODIFIED	'm'		// modified flag
#define STL_MODIFIED_ALT 'M'		// - other display
#define STL_QUICKFIX	'q'		// quickfix window description
#define STL_PERCENTAGE	'p'		// percentage through file
#define STL_ALTPERCENT	'P'		// percentage as TOP BOT ALL or NN%
#define STL_ARGLISTSTAT	'a'		// argument list status as (x of y)
#define STL_PAGENUM	'N'		// page number (when printing)
#define STL_SHOWCMD	'S'		// 'showcmd' buffer
#define STL_VIM_EXPR	'{'		// start of expression to substitute
#define STL_SEPARATE	'='		// separation between alignment
					// sections
#define STL_TRUNCMARK	'<'		// truncation mark if line is too long
#define STL_USER_HL	'*'		// highlight from (User)1..9 or 0
#define STL_HIGHLIGHT	'#'		// highlight name
#define STL_TABPAGENR	'T'		// tab page label nr
#define STL_TABCLOSENR	'X'		// tab page close nr
#define STL_ALL		((char_u *) "fFtcvVlLknoObBrRhHmYyWwMqpPaNS{#")

// flags used for parsed 'wildmode'
#define WIM_FULL	0x01
#define WIM_LONGEST	0x02
#define WIM_LIST	0x04
#define WIM_BUFLASTUSED	0x08

// flags for the 'wildoptions' option
// each defined char should be unique over all values.
#define WOP_FUZZY	'z'
#define WOP_TAGFILE	't'
#define WOP_PUM		'p'

// arguments for can_bs()
// each defined char should be unique over all values
// except for BS_START, that intentionally also matches BS_NOSTOP
// because BS_NOSTOP behaves exactly the same except it
// does not stop at the start of the insert point
#define BS_INDENT	'i'	// "Indent"
#define BS_EOL		'l'	// "eoL"
#define BS_START	's'	// "Start"
#define BS_NOSTOP	'p'	// "nostoP

// flags for the 'culopt' option
#define CULOPT_LINE	0x01	// Highlight complete line
#define CULOPT_SCRLINE	0x02	// Highlight screen line
#define CULOPT_NBR	0x04	// Highlight Number column

#define LISPWORD_VALUE	"defun,define,defmacro,set!,lambda,if,case,let,flet,let*,letrec,do,do*,define-syntax,let-syntax,letrec-syntax,destructuring-bind,defpackage,defparameter,defstruct,deftype,defvar,do-all-symbols,do-external-symbols,do-symbols,dolist,dotimes,ecase,etypecase,eval-when,labels,macrolet,multiple-value-bind,multiple-value-call,multiple-value-prog1,multiple-value-setq,prog1,progv,typecase,unless,unwind-protect,when,with-input-from-string,with-open-file,with-open-stream,with-output-to-string,with-package-iterator,define-condition,handler-bind,handler-case,restart-bind,restart-case,with-simple-restart,store-value,use-value,muffle-warning,abort,continue,with-slots,with-slots*,with-accessors,with-accessors*,defclass,defmethod,print-unreadable-object"

/*
 * The following are actual variables for the options
 */

#ifdef FEAT_RIGHTLEFT
EXTERN long	p_aleph;	// 'aleph'
#endif
EXTERN char_u	*p_ambw;	// 'ambiwidth'
#ifdef FEAT_AUTOCHDIR
EXTERN int	p_acd;		// 'autochdir'
#endif
#ifdef FEAT_AUTOSHELLDIR
EXTERN int	p_asd;		// 'autoshelldir'
#endif
EXTERN int	p_ai;		// 'autoindent'
EXTERN int	p_bin;		// 'binary'
EXTERN int	p_bomb;		// 'bomb'
EXTERN int	p_bl;		// 'buflisted'
EXTERN int	p_cin;		// 'cindent'
EXTERN char_u	*p_cink;	// 'cinkeys'
EXTERN char_u	*p_cinsd;	// 'cinscopedecls'
EXTERN char_u	*p_cinw;	// 'cinwords'
#ifdef FEAT_COMPL_FUNC
EXTERN char_u	*p_cfu;		// 'completefunc'
EXTERN char_u	*p_ofu;		// 'omnifunc'
EXTERN char_u	*p_tsrfu;	// 'thesaurusfunc'
#endif
EXTERN int	p_ci;		// 'copyindent'
#if defined(FEAT_GUI) && defined(MACOS_X)
EXTERN int	*p_antialias;	// 'antialias'
#endif
EXTERN int	p_ar;		// 'autoread'
EXTERN int	p_aw;		// 'autowrite'
EXTERN int	p_awa;		// 'autowriteall'
EXTERN char_u	*p_bs;		// 'backspace'
EXTERN char_u	*p_bg;		// 'background'
EXTERN int	p_bk;		// 'backup'
EXTERN char_u	*p_bkc;		// 'backupcopy'
EXTERN unsigned	bkc_flags;	// flags from 'backupcopy'
# define BKC_YES		0x001
# define BKC_AUTO		0x002
# define BKC_NO			0x004
# define BKC_BREAKSYMLINK	0x008
# define BKC_BREAKHARDLINK	0x010
EXTERN char_u	*p_bdir;	// 'backupdir'
EXTERN char_u	*p_bex;		// 'backupext'
EXTERN char_u	*p_bo;		// 'belloff'
EXTERN unsigned	bo_flags;

// values for the 'belloff' option
#define BO_ALL		0x0001
#define BO_BS		0x0002
#define BO_CRSR		0x0004
#define BO_COMPL	0x0008
#define BO_COPY		0x0010
#define BO_CTRLG	0x0020
#define BO_ERROR	0x0040
#define BO_ESC		0x0080
#define BO_EX		0x0100
#define BO_HANGUL	0x0200
#define BO_IM		0x0400
#define BO_LANG		0x0800
#define BO_MESS		0x1000
#define BO_MATCH	0x2000
#define BO_OPER		0x4000
#define BO_REG		0x8000
#define BO_SH		0x10000
#define BO_SPELL	0x20000
#define BO_TERM		0x40000
#define BO_WILD		0x80000

EXTERN char_u	*p_bsk;		// 'backupskip'
#ifdef FEAT_CRYPT
EXTERN char_u	*p_cm;		// 'cryptmethod'
#endif
#ifdef FEAT_BEVAL
# ifdef FEAT_BEVAL_GUI
EXTERN int	p_beval;	// 'ballooneval'
# endif
EXTERN long	p_bdlay;	// 'balloondelay'
# ifdef FEAT_EVAL
EXTERN char_u	*p_bexpr;
# endif
# ifdef FEAT_BEVAL_TERM
EXTERN int	p_bevalterm;	// 'balloonevalterm'
# endif
#endif
#ifdef FEAT_BROWSE
EXTERN char_u	*p_bsdir;	// 'browsedir'
#endif
#ifdef FEAT_LINEBREAK
EXTERN char_u	*p_breakat;	// 'breakat'
#endif
EXTERN char_u	*p_bh;		// 'bufhidden'
EXTERN char_u	*p_bt;		// 'buftype'
EXTERN char_u	*p_cmp;		// 'casemap'
EXTERN unsigned	cmp_flags;
#define CMP_INTERNAL		0x001
#define CMP_KEEPASCII		0x002
EXTERN char_u	*p_enc;		// 'encoding'
EXTERN int	p_deco;		// 'delcombine'
#ifdef FEAT_EVAL
EXTERN char_u	*p_ccv;		// 'charconvert'
#endif
EXTERN int	p_cdh;		// 'cdhome'
EXTERN char_u	*p_cino;	// 'cinoptions'
EXTERN char_u	*p_cedit;	// 'cedit'
EXTERN long	p_cwh;		// 'cmdwinheight'
#ifdef FEAT_CLIPBOARD
EXTERN char_u	*p_cb;		// 'clipboard'
#endif
EXTERN long	p_ch;		// 'cmdheight'
#ifdef FEAT_FOLDING
EXTERN char_u	*p_cms;		// 'commentstring'
#endif
EXTERN char_u	*p_cpt;		// 'complete'
#if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
EXTERN int	p_confirm;	// 'confirm'
#endif
EXTERN int	p_cp;		// 'compatible'
EXTERN char_u	*p_cot;		// 'completeopt'
#ifdef BACKSLASH_IN_FILENAME
EXTERN char_u	*p_csl;		// 'completeslash'
#endif
EXTERN long	p_ph;		// 'pumheight'
EXTERN long	p_pw;		// 'pumwidth'
EXTERN char_u	*p_com;		// 'comments'
EXTERN char_u	*p_cpo;		// 'cpoptions'
#ifdef FEAT_CSCOPE
EXTERN char_u	*p_csprg;	// 'cscopeprg'
EXTERN int	p_csre;		// 'cscoperelative'
# ifdef FEAT_QUICKFIX
EXTERN char_u	*p_csqf;	// 'cscopequickfix'
#  define	CSQF_CMDS   "sgdctefia"
#  define	CSQF_FLAGS  "+-0"
# endif
EXTERN int	p_cst;		// 'cscopetag'
EXTERN long	p_csto;		// 'cscopetagorder'
EXTERN long	p_cspc;		// 'cscopepathcomp'
EXTERN int	p_csverbose;	// 'cscopeverbose'
#endif
EXTERN char_u	*p_debug;	// 'debug'
#ifdef FEAT_FIND_ID
EXTERN char_u	*p_def;		// 'define'
EXTERN char_u	*p_inc;
#endif
#ifdef FEAT_DIFF
EXTERN char_u	*p_dip;		// 'diffopt'
# ifdef FEAT_EVAL
EXTERN char_u	*p_dex;		// 'diffexpr'
# endif
#endif
EXTERN char_u	*p_dict;	// 'dictionary'
#ifdef FEAT_DIGRAPHS
EXTERN int	p_dg;		// 'digraph'
#endif
EXTERN char_u	*p_dir;		// 'directory'
EXTERN char_u	*p_dy;		// 'display'
EXTERN unsigned	dy_flags;
#define DY_LASTLINE		0x001
#define DY_TRUNCATE		0x002
#define DY_UHEX			0x004
EXTERN int	p_ed;		// 'edcompatible'
EXTERN char_u	*p_ead;		// 'eadirection'
EXTERN char_u	*p_emoji;	// 'emoji'
EXTERN int	p_ea;		// 'equalalways'
EXTERN char_u	*p_ep;		// 'equalprg'
EXTERN int	p_eb;		// 'errorbells'
#ifdef FEAT_QUICKFIX
EXTERN char_u	*p_ef;		// 'errorfile'
EXTERN char_u	*p_efm;		// 'errorformat'
EXTERN char_u	*p_gefm;	// 'grepformat'
EXTERN char_u	*p_gp;		// 'grepprg'
#endif
EXTERN int	p_eof;		// 'endoffile'
EXTERN int	p_eol;		// 'endofline'
EXTERN int	p_ek;		// 'esckeys'
EXTERN char_u	*p_ei;		// 'eventignore'
EXTERN int	p_et;		// 'expandtab'
EXTERN int	p_exrc;		// 'exrc'
EXTERN char_u	*p_fenc;	// 'fileencoding'
EXTERN char_u	*p_fencs;	// 'fileencodings'
EXTERN char_u	*p_ff;		// 'fileformat'
EXTERN char_u	*p_ffs;		// 'fileformats'
EXTERN int	p_fic;		// 'fileignorecase'
EXTERN char_u	*p_ft;		// 'filetype'
EXTERN char_u	*p_fcs;		// 'fillchar'
EXTERN int	p_fixeol;	// 'fixendofline'
#ifdef FEAT_FOLDING
EXTERN char_u	*p_fcl;		// 'foldclose'
EXTERN long	p_fdls;		// 'foldlevelstart'
EXTERN char_u	*p_fdo;		// 'foldopen'
EXTERN unsigned	fdo_flags;
# define FDO_ALL		0x001
# define FDO_BLOCK		0x002
# define FDO_HOR		0x004
# define FDO_MARK		0x008
# define FDO_PERCENT		0x010
# define FDO_QUICKFIX		0x020
# define FDO_SEARCH		0x040
# define FDO_TAG		0x080
# define FDO_INSERT		0x100
# define FDO_UNDO		0x200
# define FDO_JUMP		0x400
#endif
#if defined(FEAT_EVAL)
EXTERN char_u	*p_fex;		// 'formatexpr'
#endif
EXTERN char_u	*p_flp;		// 'formatlistpat'
EXTERN char_u	*p_fo;		// 'formatoptions'
EXTERN char_u	*p_fp;		// 'formatprg'
#ifdef HAVE_FSYNC
EXTERN int	p_fs;		// 'fsync'
#endif
EXTERN int	p_gd;		// 'gdefault'
EXTERN char_u	*p_jop;		// 'jumpoptions'
EXTERN unsigned	jop_flags;	//
#define JOP_STACK		0x001
#ifdef FEAT_PROP_POPUP
# ifdef FEAT_QUICKFIX
EXTERN char_u	*p_cpp;		// 'completepopup'
# endif
EXTERN char_u	*p_pvp;		// 'previewpopup'
#endif
#ifdef FEAT_PRINTER
EXTERN char_u	*p_pdev;	// 'printdevice'
# ifdef FEAT_POSTSCRIPT
EXTERN char_u	*p_penc;	// 'printencoding'
EXTERN char_u	*p_pexpr;	// 'printexpr'
EXTERN char_u	*p_pmfn;	// 'printmbfont'
EXTERN char_u	*p_pmcs;	// 'printmbcharset'
# endif
EXTERN char_u	*p_pfn;		// 'printfont'
EXTERN char_u	*p_popt;	// 'printoptions'
EXTERN char_u	*p_header;	// 'printheader'
#endif
EXTERN int	p_prompt;	// 'prompt'
#ifdef FEAT_GUI
EXTERN char_u	*p_guifont;	// 'guifont'
# ifdef FEAT_XFONTSET
EXTERN char_u	*p_guifontset;	// 'guifontset'
# endif
EXTERN char_u	*p_guifontwide;	// 'guifontwide'
EXTERN int	p_guipty;	// 'guipty'
#endif
#ifdef FEAT_GUI_GTK
EXTERN char_u	*p_guiligatures;  // 'guiligatures'
# endif
#if defined(FEAT_GUI_GTK) || defined(FEAT_GUI_X11)
EXTERN long	p_ghr;		// 'guiheadroom'
#endif
#ifdef CURSOR_SHAPE
EXTERN char_u	*p_guicursor;	// 'guicursor'
#endif
#ifdef FEAT_MOUSESHAPE
EXTERN char_u	*p_mouseshape;	// 'mouseshape'
#endif
#if defined(FEAT_GUI)
EXTERN char_u	*p_go;		// 'guioptions'
#endif
#if defined(FEAT_GUI_TABLINE)
EXTERN char_u	*p_gtl;		// 'guitablabel'
EXTERN char_u	*p_gtt;		// 'guitabtooltip'
#endif
EXTERN char_u	*p_hf;		// 'helpfile'
EXTERN long	p_hh;		// 'helpheight'
#ifdef FEAT_MULTI_LANG
EXTERN char_u	*p_hlg;		// 'helplang'
#endif
EXTERN int	p_hid;		// 'hidden'
EXTERN char_u	*p_hl;		// 'highlight'
EXTERN int	p_hls;		// 'hlsearch'
EXTERN long	p_hi;		// 'history'
#ifdef FEAT_RIGHTLEFT
EXTERN int	p_hkmap;	// 'hkmap'
EXTERN int	p_hkmapp;	// 'hkmapp'
# ifdef FEAT_ARABIC
EXTERN int	p_arshape;	// 'arabicshape'
# endif
#endif
EXTERN int	p_icon;		// 'icon'
EXTERN char_u	*p_iconstring;	// 'iconstring'
EXTERN int	p_ic;		// 'ignorecase'
#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
EXTERN char_u	*p_imak;	// 'imactivatekey'
#define IM_ON_THE_SPOT		0L
#define IM_OVER_THE_SPOT	1L
EXTERN long	p_imst;		// 'imstyle'
#endif
#if defined(FEAT_EVAL)
EXTERN char_u	*p_imaf;	// 'imactivatefunc'
EXTERN char_u	*p_imsf;	// 'imstatusfunc'
#endif
EXTERN int	p_imcmdline;	// 'imcmdline'
EXTERN int	p_imdisable;	// 'imdisable'
EXTERN long	p_iminsert;	// 'iminsert'
EXTERN long	p_imsearch;	// 'imsearch'
EXTERN int	p_inf;		// 'infercase'
#if defined(FEAT_FIND_ID) && defined(FEAT_EVAL)
EXTERN char_u	*p_inex;	// 'includeexpr'
#endif
EXTERN int	p_is;		// 'incsearch'
#if defined(FEAT_EVAL)
EXTERN char_u	*p_inde;	// 'indentexpr'
EXTERN char_u	*p_indk;	// 'indentkeys'
#endif
EXTERN int	p_im;		// 'insertmode'
EXTERN char_u	*p_isf;		// 'isfname'
EXTERN char_u	*p_isi;		// 'isident'
EXTERN char_u	*p_isk;		// 'iskeyword'
EXTERN char_u	*p_isp;		// 'isprint'
EXTERN int	p_js;		// 'joinspaces'
#ifdef FEAT_CRYPT
EXTERN char_u	*p_key;		// 'key'
#endif
#ifdef FEAT_KEYMAP
EXTERN char_u	*p_keymap;	// 'keymap'
#endif
EXTERN char_u	*p_kp;		// 'keywordprg'
EXTERN char_u	*p_km;		// 'keymodel'
EXTERN char_u	*p_kpc;		// 'keyprotocol'
#ifdef FEAT_LANGMAP
EXTERN char_u	*p_langmap;	// 'langmap'
EXTERN int	p_lnr;		// 'langnoremap'
EXTERN int	p_lrm;		// 'langremap'
#endif
#if defined(FEAT_MENU) && defined(FEAT_MULTI_LANG)
EXTERN char_u	*p_lm;		// 'langmenu'
#endif
#ifdef FEAT_GUI
EXTERN long	p_linespace;	// 'linespace'
#endif
EXTERN int	p_lisp;		// 'lisp'
EXTERN char_u	*p_lop;		// 'lispoptions'
EXTERN char_u	*p_lispwords;	// 'lispwords'
EXTERN long	p_ls;		// 'laststatus'
EXTERN long	p_stal;		// 'showtabline'
EXTERN char_u	*p_lcs;		// 'listchars'

EXTERN int	p_lz;		// 'lazyredraw'
EXTERN int	p_lpl;		// 'loadplugins'
#if defined(DYNAMIC_LUA)
EXTERN char_u	*p_luadll;	// 'luadll'
#endif
EXTERN int	p_magic;	// 'magic'
EXTERN char_u	*p_menc;	// 'makeencoding'
#ifdef FEAT_QUICKFIX
EXTERN char_u	*p_mef;		// 'makeef'
EXTERN char_u	*p_mp;		// 'makeprg'
#endif
EXTERN char_u	*p_mps;		// 'matchpairs'
EXTERN long	p_mat;		// 'matchtime'
EXTERN long	p_mco;		// 'maxcombine'
#ifdef FEAT_EVAL
EXTERN long	p_mfd;		// 'maxfuncdepth'
#endif
EXTERN long	p_mmd;		// 'maxmapdepth'
EXTERN long	p_mm;		// 'maxmem'
EXTERN long	p_mmp;		// 'maxmempattern'
EXTERN long	p_mmt;		// 'maxmemtot'
#ifdef FEAT_MENU
EXTERN long	p_mis;		// 'menuitems'
#endif
#ifdef FEAT_SPELL
EXTERN char_u	*p_msm;		// 'mkspellmem'
#endif
EXTERN int	p_ml;		// 'modeline'
EXTERN int	p_mle;		// 'modelineexpr'
EXTERN long	p_mls;		// 'modelines'
EXTERN int	p_ma;		// 'modifiable'
EXTERN int	p_mod;		// 'modified'
EXTERN char_u	*p_mouse;	// 'mouse'
#ifdef FEAT_GUI
EXTERN int	p_mousef;	// 'mousefocus'
EXTERN int	p_mh;		// 'mousehide'
#endif
EXTERN char_u	*p_mousem;	// 'mousemodel'
#ifdef FEAT_GUI
EXTERN int	p_mousemev;	// 'mousemoveevent'
#endif
EXTERN long	p_mouset;	// 'mousetime'
EXTERN int	p_more;		// 'more'
#ifdef FEAT_MZSCHEME
EXTERN long	p_mzq;		// 'mzquantum
# if defined(DYNAMIC_MZSCHEME)
EXTERN char_u	*p_mzschemedll;	// 'mzschemedll'
EXTERN char_u	*p_mzschemegcdll; // 'mzschemegcdll'
# endif
#endif
EXTERN char_u	*p_nf;		// 'nrformats'
#if defined(MSWIN)
EXTERN int	p_odev;		// 'opendevice'
#endif
EXTERN char_u	*p_opfunc;	// 'operatorfunc'
EXTERN char_u	*p_para;	// 'paragraphs'
EXTERN int	p_paste;	// 'paste'
EXTERN char_u	*p_pt;		// 'pastetoggle'
#if defined(FEAT_EVAL) && defined(FEAT_DIFF)
EXTERN char_u	*p_pex;		// 'patchexpr'
#endif
EXTERN char_u	*p_pm;		// 'patchmode'
EXTERN char_u	*p_path;	// 'path'
EXTERN char_u	*p_cdpath;	// 'cdpath'
#if defined(DYNAMIC_PERL)
EXTERN char_u	*p_perldll;	// 'perldll'
#endif
EXTERN int	p_pi;		// 'preserveindent'
#if defined(DYNAMIC_PYTHON3)
EXTERN char_u	*p_py3dll;	// 'pythonthreedll'
#endif
#ifdef FEAT_PYTHON3
EXTERN char_u	*p_py3home;	// 'pythonthreehome'
#endif
#if defined(DYNAMIC_PYTHON)
EXTERN char_u	*p_pydll;	// 'pythondll'
#endif
#ifdef FEAT_PYTHON
EXTERN char_u	*p_pyhome;	// 'pythonhome'
#endif
#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3)
EXTERN long	p_pyx;		// 'pyxversion'
#endif
EXTERN char_u	*p_qe;		// 'quoteescape'
EXTERN int	p_ro;		// 'readonly'
#ifdef FEAT_RELTIME
EXTERN long	p_rdt;		// 'redrawtime'
#endif
EXTERN int	p_remap;	// 'remap'
EXTERN long	p_re;		// 'regexpengine'
#ifdef FEAT_RENDER_OPTIONS
EXTERN char_u	*p_rop;		// 'renderoptions'
#endif
EXTERN long	p_report;	// 'report'
#if defined(FEAT_QUICKFIX)
EXTERN long	p_pvh;		// 'previewheight'
#endif
#ifdef MSWIN
EXTERN int	p_rs;		// 'restorescreen'
#endif
#ifdef FEAT_RIGHTLEFT
EXTERN int	p_ari;		// 'allowrevins'
EXTERN int	p_ri;		// 'revins'
#endif
#if defined(DYNAMIC_RUBY)
EXTERN char_u	*p_rubydll;	// 'rubydll'
#endif
EXTERN int	p_ru;		// 'ruler'
#ifdef FEAT_STL_OPT
EXTERN char_u	*p_ruf;		// 'rulerformat'
#endif
EXTERN char_u	*p_pp;		// 'packpath'
#ifdef FEAT_QUICKFIX
EXTERN char_u	*p_qftf;	// 'quickfixtextfunc'
#endif
EXTERN char_u	*p_rtp;		// 'runtimepath'
EXTERN long	p_sj;		// 'scrolljump'
#if defined(MSWIN) && defined(FEAT_GUI)
EXTERN int	p_scf;		// 'scrollfocus'
#endif
EXTERN long	p_so;		// 'scrolloff'
EXTERN char_u	*p_sbo;		// 'scrollopt'
EXTERN char_u	*p_sections;	// 'sections'
EXTERN int	p_secure;	// 'secure'
EXTERN char_u	*p_sel;		// 'selection'
EXTERN char_u	*p_slm;		// 'selectmode'
#ifdef FEAT_SESSION
EXTERN char_u	*p_ssop;	// 'sessionoptions'
EXTERN unsigned	ssop_flags;
# define SSOP_BUFFERS		0x001
# define SSOP_WINPOS		0x002
# define SSOP_RESIZE		0x004
# define SSOP_WINSIZE		0x008
# define SSOP_LOCALOPTIONS	0x010
# define SSOP_OPTIONS		0x020
# define SSOP_HELP		0x040
# define SSOP_BLANK		0x080
# define SSOP_GLOBALS		0x100
# define SSOP_SLASH		0x200
# define SSOP_UNIX		0x400
# define SSOP_SESDIR		0x800
# define SSOP_CURDIR		0x1000
# define SSOP_FOLDS		0x2000
# define SSOP_CURSOR		0x4000
# define SSOP_TABPAGES		0x8000
# define SSOP_TERMINAL		0x10000
# define SSOP_SKIP_RTP		0x20000
#endif
EXTERN char_u	*p_sh;		// 'shell'
EXTERN char_u	*p_shcf;	// 'shellcmdflag'
#ifdef FEAT_QUICKFIX
EXTERN char_u	*p_sp;		// 'shellpipe'
#endif
EXTERN char_u	*p_shq;		// 'shellquote'
EXTERN char_u	*p_sxq;		// 'shellxquote'
EXTERN char_u	*p_sxe;		// 'shellxescape'
EXTERN char_u	*p_srr;		// 'shellredir'
#ifdef AMIGA
EXTERN long	p_st;		// 'shelltype'
#endif
EXTERN int	p_stmp;		// 'shelltemp'
#ifdef BACKSLASH_IN_FILENAME
EXTERN int	p_ssl;		// 'shellslash'
#endif
#ifdef FEAT_STL_OPT
EXTERN char_u	*p_stl;		// 'statusline'
#endif
EXTERN int	p_sr;		// 'shiftround'
EXTERN long	p_sw;		// 'shiftwidth'
EXTERN char_u	*p_shm;		// 'shortmess'
EXTERN int	p_sn;		// 'shortname'
#ifdef FEAT_LINEBREAK
EXTERN char_u	*p_sbr;		// 'showbreak'
#endif
EXTERN int	p_sc;		// 'showcmd'
EXTERN char_u	*p_sloc;	// 'showcmdloc'
EXTERN int	p_sft;		// 'showfulltag'
EXTERN int	p_sm;		// 'showmatch'
EXTERN int	p_smd;		// 'showmode'
EXTERN long	p_ss;		// 'sidescroll'
EXTERN long	p_siso;		// 'sidescrolloff'
EXTERN int	p_scs;		// 'smartcase'
EXTERN int	p_si;		// 'smartindent'
EXTERN int	p_sta;		// 'smarttab'
EXTERN long	p_sts;		// 'softtabstop'
EXTERN int	p_sb;		// 'splitbelow'
EXTERN char_u	*p_sua;		// 'suffixesadd'
EXTERN int	p_swf;		// 'swapfile'
#ifdef FEAT_SYN_HL
EXTERN long	p_smc;		// 'synmaxcol'
#endif
EXTERN long	p_tpm;		// 'tabpagemax'
#ifdef FEAT_STL_OPT
EXTERN char_u	*p_tal;		// 'tabline'
#endif
#ifdef FEAT_EVAL
EXTERN char_u	*p_tfu;		// 'tagfunc'
#endif
#ifdef FEAT_SPELL
EXTERN char_u	*p_spc;		// 'spellcapcheck'
EXTERN char_u	*p_spf;		// 'spellfile'
EXTERN char_u	*p_spl;		// 'spelllang'
EXTERN char_u	*p_spo;		// 'spelloptions'
EXTERN char_u	*p_sps;		// 'spellsuggest'
#endif
EXTERN int	p_spr;		// 'splitright'
EXTERN int	p_sol;		// 'startofline'
EXTERN char_u	*p_su;		// 'suffixes'
EXTERN char_u	*p_sws;		// 'swapsync'
EXTERN char_u	*p_swb;		// 'switchbuf'
EXTERN char_u	*p_spk;		// 'splitkeep'
EXTERN unsigned	swb_flags;
// Keep in sync with p_swb_values in optionstr.c
#define SWB_USEOPEN		0x001
#define SWB_USETAB		0x002
#define SWB_SPLIT		0x004
#define SWB_NEWTAB		0x008
#define SWB_VSPLIT		0x010
#define SWB_USELAST		0x020
#ifdef FEAT_SYN_HL
EXTERN char_u	*p_syn;		// 'syntax'
#endif
EXTERN long	p_ts;		// 'tabstop'
EXTERN int	p_tbs;		// 'tagbsearch'
EXTERN char_u	*p_tc;		// 'tagcase'
EXTERN unsigned tc_flags;       // flags from 'tagcase'
#define TC_FOLLOWIC		0x01
#define TC_IGNORE		0x02
#define TC_MATCH		0x04
#define TC_FOLLOWSCS		0x08
#define TC_SMART		0x10
EXTERN long	p_tl;		// 'taglength'
EXTERN int	p_tr;		// 'tagrelative'
EXTERN char_u	*p_tags;	// 'tags'
EXTERN int	p_tgst;		// 'tagstack'
#if defined(DYNAMIC_TCL)
EXTERN char_u	*p_tcldll;	// 'tcldll'
#endif
#ifdef FEAT_ARABIC
EXTERN int	p_tbidi;	// 'termbidi'
#endif
EXTERN char_u	*p_tenc;	// 'termencoding'
#ifdef FEAT_TERMGUICOLORS
EXTERN int	p_tgc;		// 'termguicolors'
#endif
#ifdef FEAT_TERMINAL
EXTERN long	p_twsl;		// 'termwinscroll'
#endif
#if defined(MSWIN) && defined(FEAT_TERMINAL)
EXTERN char_u	*p_twt;		// 'termwintype'
#endif
EXTERN int	p_terse;	// 'terse'
EXTERN int	p_ta;		// 'textauto'
EXTERN int	p_tx;		// 'textmode'
EXTERN long	p_tw;		// 'textwidth'
EXTERN int	p_to;		// 'tildeop'
EXTERN int	p_timeout;	// 'timeout'
EXTERN long	p_tm;		// 'timeoutlen'
EXTERN int	p_title;	// 'title'
EXTERN long	p_titlelen;	// 'titlelen'
EXTERN char_u	*p_titleold;	// 'titleold'
EXTERN char_u	*p_titlestring;	// 'titlestring'
EXTERN char_u	*p_tsr;		// 'thesaurus'
EXTERN int	p_ttimeout;	// 'ttimeout'
EXTERN long	p_ttm;		// 'ttimeoutlen'
EXTERN int	p_tbi;		// 'ttybuiltin'
EXTERN int	p_tf;		// 'ttyfast'
#if defined(FEAT_TOOLBAR) && !defined(FEAT_GUI_MSWIN)
EXTERN char_u	*p_toolbar;	// 'toolbar'
EXTERN unsigned toolbar_flags;
# define TOOLBAR_TEXT		0x01
# define TOOLBAR_ICONS		0x02
# define TOOLBAR_TOOLTIPS	0x04
# define TOOLBAR_HORIZ		0x08
#endif
#if defined(FEAT_TOOLBAR) && defined(FEAT_GUI_GTK)
EXTERN char_u	*p_tbis;	// 'toolbariconsize'
EXTERN unsigned tbis_flags;
# define TBIS_TINY		0x01
# define TBIS_SMALL		0x02
# define TBIS_MEDIUM		0x04
# define TBIS_LARGE		0x08
# define TBIS_HUGE		0x10
# define TBIS_GIANT		0x20
#endif
EXTERN long	p_ttyscroll;	// 'ttyscroll'
#if defined(UNIX) || defined(VMS)
EXTERN char_u	*p_ttym;	// 'ttymouse'
EXTERN unsigned ttym_flags;
# define TTYM_XTERM		0x01
# define TTYM_XTERM2		0x02
# define TTYM_DEC		0x04
# define TTYM_NETTERM		0x08
# define TTYM_JSBTERM		0x10
# define TTYM_PTERM		0x20
# define TTYM_URXVT		0x40
# define TTYM_SGR		0x80
#endif
#ifdef FEAT_PERSISTENT_UNDO
EXTERN char_u	*p_udir;	// 'undodir'
EXTERN int	p_udf;		// 'undofile'
#endif
EXTERN long	p_ul;		// 'undolevels'
EXTERN long	p_ur;		// 'undoreload'
EXTERN long	p_uc;		// 'updatecount'
EXTERN long	p_ut;		// 'updatetime'
#ifdef FEAT_VARTABS
EXTERN char_u	*p_vsts;	// 'varsofttabstop'
EXTERN char_u	*p_vts;		// 'vartabstop'
#endif
#ifdef FEAT_VIMINFO
EXTERN char_u	*p_viminfo;	// 'viminfo'
EXTERN char_u	*p_viminfofile;	// 'viminfofile'
#endif
#ifdef FEAT_SESSION
EXTERN char_u	*p_vdir;	// 'viewdir'
EXTERN char_u	*p_vop;		// 'viewoptions'
EXTERN unsigned	vop_flags;	// uses SSOP_ flags
#endif
EXTERN int	p_vb;		// 'visualbell'
EXTERN char_u	*p_ve;		// 'virtualedit'
EXTERN unsigned ve_flags;
#define VE_BLOCK	5	// includes "all"
#define VE_INSERT	6	// includes "all"
#define VE_ALL		4
#define VE_ONEMORE	8
#define VE_NONE		16	// "none"
#define VE_NONEU	32      // "NONE"
EXTERN long	p_verbose;	// 'verbose'
#ifdef IN_OPTION_C
char_u	*p_vfile = (char_u *)""; // used before options are initialized
#else
extern char_u	*p_vfile;	// 'verbosefile'
#endif
EXTERN int	p_warn;		// 'warn'
EXTERN char_u	*p_wop;		// 'wildoptions'
EXTERN long	p_window;	// 'window'
#if defined(FEAT_GUI_MSWIN) || defined(FEAT_GUI_MOTIF) || defined(LINT) \
	|| defined (FEAT_GUI_GTK) || defined(FEAT_GUI_PHOTON)
#define FEAT_WAK
EXTERN char_u	*p_wak;		// 'winaltkeys'
#endif
EXTERN char_u	*p_wig;		// 'wildignore'
EXTERN int	p_wiv;		// 'weirdinvert'
EXTERN char_u	*p_ww;		// 'whichwrap'
EXTERN long	p_wc;		// 'wildchar'
EXTERN long	p_wcm;		// 'wildcharm'
EXTERN int	p_wic;		// 'wildignorecase'
EXTERN char_u	*p_wim;		// 'wildmode'
EXTERN int	p_wmnu;		// 'wildmenu'
EXTERN long	p_wh;		// 'winheight'
EXTERN long	p_wmh;		// 'winminheight'
EXTERN long	p_wmw;		// 'winminwidth'
EXTERN long	p_wiw;		// 'winwidth'
#if defined(MSWIN) && defined(FEAT_TERMINAL)
EXTERN char_u	*p_winptydll;	// 'winptydll'
#endif
EXTERN long	p_wm;		// 'wrapmargin'
EXTERN int	p_ws;		// 'wrapscan'
EXTERN int	p_write;	// 'write'
EXTERN int	p_wa;		// 'writeany'
EXTERN int	p_wb;		// 'writebackup'
EXTERN long	p_wd;		// 'writedelay'
EXTERN int	p_xtermcodes;	// 'xtermcodes'

/*
 * "indir" values for buffer-local options.
 * These need to be defined globally, so that the BV_COUNT can be used with
 * b_p_scriptID[].
 */
enum
{
    BV_AI = 0
    , BV_AR
    , BV_BH
    , BV_BKC
    , BV_BT
#ifdef FEAT_QUICKFIX
    , BV_EFM
    , BV_GP
    , BV_MP
#endif
    , BV_BIN
    , BV_BL
    , BV_BOMB
    , BV_CI
    , BV_CIN
    , BV_CINK
    , BV_CINO
    , BV_CINSD
    , BV_CINW
    , BV_CM
#ifdef FEAT_FOLDING
    , BV_CMS
#endif
    , BV_COM
    , BV_CPT
    , BV_DICT
    , BV_TSR
#ifdef BACKSLASH_IN_FILENAME
    , BV_CSL
#endif
#ifdef FEAT_COMPL_FUNC
    , BV_CFU
#endif
#ifdef FEAT_FIND_ID
    , BV_DEF
    , BV_INC
#endif
    , BV_EOF
    , BV_EOL
    , BV_FIXEOL
    , BV_EP
    , BV_ET
    , BV_FENC
    , BV_FP
#ifdef FEAT_EVAL
    , BV_BEXPR
    , BV_FEX
#endif
    , BV_FF
    , BV_FLP
    , BV_FO
    , BV_FT
    , BV_IMI
    , BV_IMS
#if defined(FEAT_EVAL)
    , BV_INDE
    , BV_INDK
#endif
#if defined(FEAT_FIND_ID) && defined(FEAT_EVAL)
    , BV_INEX
#endif
    , BV_INF
    , BV_ISK
#ifdef FEAT_CRYPT
    , BV_KEY
#endif
#ifdef FEAT_KEYMAP
    , BV_KMAP
#endif
    , BV_KP
    , BV_LISP
    , BV_LOP
    , BV_LW
    , BV_MENC
    , BV_MA
    , BV_ML
    , BV_MOD
    , BV_MPS
    , BV_NF
#ifdef FEAT_COMPL_FUNC
    , BV_OFU
#endif
    , BV_PATH
    , BV_PI
    , BV_QE
    , BV_RO
    , BV_SI
    , BV_SN
#ifdef FEAT_SYN_HL
    , BV_SMC
    , BV_SYN
#endif
#ifdef FEAT_SPELL
    , BV_SPC
    , BV_SPF
    , BV_SPL
    , BV_SPO
#endif
    , BV_STS
    , BV_SUA
    , BV_SW
    , BV_SWF
#ifdef FEAT_EVAL
    , BV_TFU
#endif
    , BV_TAGS
    , BV_TC
#ifdef FEAT_COMPL_FUNC
    , BV_TSRFU
#endif
    , BV_TS
    , BV_TW
    , BV_TX
    , BV_UDF
    , BV_UL
    , BV_WM
#ifdef FEAT_TERMINAL
    , BV_TWSL
#endif
#ifdef FEAT_VARTABS
    , BV_VSTS
    , BV_VTS
#endif
    , BV_COUNT	    // must be the last one
};

/*
 * "indir" values for window-local options.
 * These need to be defined globally, so that the WV_COUNT can be used in the
 * window structure.
 */
enum
{
    WV_LIST = 0
    , WV_LCS
    , WV_FCS
#ifdef FEAT_ARABIC
    , WV_ARAB
#endif
#ifdef FEAT_CONCEAL
    , WV_COCU
    , WV_COLE
#endif
#ifdef FEAT_TERMINAL
    , WV_TWK
    , WV_TWS
#endif
    , WV_CRBIND
#ifdef FEAT_LINEBREAK
    , WV_BRI
    , WV_BRIOPT
#endif
    , WV_WCR
#ifdef FEAT_DIFF
    , WV_DIFF
#endif
#ifdef FEAT_FOLDING
    , WV_FDC
    , WV_FEN
    , WV_FDI
    , WV_FDL
    , WV_FDM
    , WV_FML
    , WV_FDN
# ifdef FEAT_EVAL
    , WV_FDE
    , WV_FDT
# endif
    , WV_FMR
#endif
#ifdef FEAT_LINEBREAK
    , WV_LBR
#endif
    , WV_NU
    , WV_RNU
    , WV_VE
#ifdef FEAT_LINEBREAK
    , WV_NUW
#endif
#if defined(FEAT_QUICKFIX)
    , WV_PVW
#endif
#ifdef FEAT_RIGHTLEFT
    , WV_RL
    , WV_RLC
#endif
    , WV_SCBIND
    , WV_SCROLL
    , WV_SMS
    , WV_SISO
    , WV_SO
#ifdef FEAT_SPELL
    , WV_SPELL
#endif
#ifdef FEAT_SYN_HL
    , WV_CUC
    , WV_CUL
    , WV_CULOPT
    , WV_CC
#endif
#ifdef FEAT_LINEBREAK
    , WV_SBR
#endif
#ifdef FEAT_STL_OPT
    , WV_STL
#endif
    , WV_WFH
    , WV_WFW
    , WV_WRAP
#ifdef FEAT_SIGNS
    , WV_SCL
#endif
    , WV_COUNT	    // must be the last one
};

// Value for b_p_ul indicating the global value must be used.
#define NO_LOCAL_UNDOLEVEL (-123456)

#define ERR_BUFLEN 80

#endif // _OPTION_H_
