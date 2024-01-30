/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 *
 * term.c: functions for controlling the terminal
 *
 * primitive termcap support for Amiga and Win32 included
 *
 * NOTE: padding and variable substitution is not performed,
 * when compiling without HAVE_TGETENT, we use tputs() and tgoto() dummies.
 */

/*
 * Some systems have a prototype for tgetstr() with (char *) instead of
 * (char **). This define removes that prototype. We include our own prototype
 * below.
 */
#define tgetstr tgetstr_defined_wrong

#include "vim.h"

#ifdef HAVE_TGETENT
# ifdef HAVE_TERMIOS_H
#  include <termios.h>	    // seems to be required for some Linux
# endif
# ifdef HAVE_TERMCAP_H
#  include <termcap.h>
# endif

/*
 * A few linux systems define outfuntype in termcap.h to be used as the third
 * argument for tputs().
 */
# if defined(VMS) || defined(AMIGA)
#  define TPUTSFUNCAST (void (*)(unsigned int))
# else
#  ifdef HAVE_OUTFUNTYPE
#   define TPUTSFUNCAST (outfuntype)
#  else
#   define TPUTSFUNCAST (int (*)(int))
#  endif
# endif
#endif

#undef tgetstr

// start of keys that are not directly used by Vim but can be mapped
#define BT_EXTRA_KEYS	0x101

static void parse_builtin_tcap(char_u *s);
static void gather_termleader(void);
#ifdef FEAT_TERMRESPONSE
static void req_codes_from_term(void);
static void req_more_codes_from_term(void);
static void got_code_from_term(char_u *code, int len);
static void check_for_codes_from_term(void);
#endif
static void del_termcode_idx(int idx);
static int find_term_bykeys(char_u *src);
static int term_is_builtin(char_u *name);
static int term_7to8bit(char_u *p);
static void accept_modifiers_for_function_keys(void);

    // Change this to "if 1" to debug what happens with termresponse.
#  if 0
#   define DEBUG_TERMRESPONSE
static void log_tr(const char *fmt, ...) ATTRIBUTE_FORMAT_PRINTF(1, 2);
#   define LOG_TR(msg) log_tr msg
#  else
#   define LOG_TR(msg) do { /**/ } while (0)
#  endif

#ifdef HAVE_TGETENT
static char *invoke_tgetent(char_u *, char_u *);

/*
 * Here is our own prototype for tgetstr(), any prototypes from the include
 * files have been disabled by the define at the start of this file.
 */
char		*tgetstr(char *, char **);
#endif

typedef enum {
    STATUS_GET,		// send request when switching to RAW mode
    STATUS_SENT,	// did send request, checking for response
    STATUS_GOT,		// received response
    STATUS_FAIL		// timed out
} request_progress_T;

typedef struct {
    request_progress_T	    tr_progress;
    time_t		    tr_start;	// when request was sent, -1 for never
} termrequest_T;

# define TERMREQUEST_INIT {STATUS_GET, -1}

// Request Terminal Version status:
static termrequest_T crv_status = TERMREQUEST_INIT;

// Request Cursor position report:
static termrequest_T u7_status = TERMREQUEST_INIT;

// Request xterm compatibility check:
static termrequest_T xcc_status = TERMREQUEST_INIT;

#ifdef FEAT_TERMRESPONSE
# ifdef FEAT_TERMINAL
// Request foreground color report:
static termrequest_T rfg_status = TERMREQUEST_INIT;
static int fg_r = 0;
static int fg_g = 0;
static int fg_b = 0;
static int bg_r = 255;
static int bg_g = 255;
static int bg_b = 255;
# endif

// Request background color report:
static termrequest_T rbg_status = TERMREQUEST_INIT;

// Request cursor blinking mode report:
static termrequest_T rbm_status = TERMREQUEST_INIT;

// Request cursor style report:
static termrequest_T rcs_status = TERMREQUEST_INIT;

// Request window's position report:
static termrequest_T winpos_status = TERMREQUEST_INIT;

static termrequest_T *all_termrequests[] = {
    &crv_status,
    &u7_status,
    &xcc_status,
#  ifdef FEAT_TERMINAL
    &rfg_status,
#  endif
    &rbg_status,
    &rbm_status,
    &rcs_status,
    &winpos_status,
    NULL
};

// The t_8u code may default to a value but get reset when the term response is
// received.  To avoid redrawing too often, only redraw when t_8u is not reset
// and it was supposed to be written.  Unless t_8u was set explicitly.
// FALSE -> don't output t_8u yet
// MAYBE -> tried outputting t_8u while FALSE
// OK    -> can write t_8u
int write_t_8u_state = FALSE;
#endif

#ifdef HAVE_TGETENT
/*
 * Don't declare these variables if termcap.h contains them.
 * Autoconf checks if these variables should be declared extern (not all
 * systems have them).
 * Some versions define ospeed to be speed_t, but that is incompatible with
 * BSD, where ospeed is short and speed_t is long.
 */
# ifndef HAVE_OSPEED
#  ifdef OSPEED_EXTERN
extern short ospeed;
#   else
short ospeed;
#   endif
# endif
# ifndef HAVE_UP_BC_PC
#  ifdef UP_BC_PC_EXTERN
extern char *UP, *BC, PC;
#  else
char *UP, *BC, PC;
#  endif
# endif

# define TGETSTR(s, p)	vim_tgetstr((s), (p))
# define TGETENT(b, t)	tgetent((char *)(b), (char *)(t))
static char_u *vim_tgetstr(char *s, char_u **pp);
#endif // HAVE_TGETENT

static int  detected_8bit = FALSE;	// detected 8-bit terminal

#if (defined(UNIX) || defined(VMS))
static int focus_state = MAYBE; // TRUE if the Vim window has focus
#endif

#ifdef FEAT_TERMRESPONSE
// When the cursor shape was detected these values are used:
// 1: block, 2: underline, 3: vertical bar
static int initial_cursor_shape = 0;

// The blink flag from the style response may be inverted from the actual
// blinking state, xterm XORs the flags.
static int initial_cursor_shape_blink = FALSE;

// The blink flag from the blinking-cursor mode response
static int initial_cursor_blink = FALSE;
#endif

/*
 * The builtin termcap entries.
 *
 * The entries are also included when HAVE_TGETENT is defined, the system
 * termcap may be incomplete and a few Vim-specific entries are added.
 *
 * When HAVE_TGETENT is defined, the builtin entries can be accessed with
 * "builtin_amiga", "builtin_ansi", "builtin_debug", etc.
 *
 * Each termcap is a list of tcap_entry_T.  See parse_builtin_tcap() for all
 * details.
 *
 * Entries marked with "guessed" may be wrong.
 */
typedef struct
{
    int		bt_entry;   // either a KS_xxx code (>= 0), or a K_xxx code.
    char	*bt_string; // value
} tcap_entry_T;

/*
 * Standard ANSI terminal, default for Unix.
 */
static tcap_entry_T builtin_ansi[] = {
    {(int)KS_CE,	"\033[K"},
    {(int)KS_AL,	"\033[L"},
#  ifdef TERMINFO
    {(int)KS_CAL,	"\033[%p1%dL"},
#  else
    {(int)KS_CAL,	"\033[%dL"},
#  endif
    {(int)KS_DL,	"\033[M"},
#  ifdef TERMINFO
    {(int)KS_CDL,	"\033[%p1%dM"},
#  else
    {(int)KS_CDL,	"\033[%dM"},
#  endif
    {(int)KS_CL,	"\033[H\033[2J"},
    {(int)KS_ME,	"\033[0m"},
    {(int)KS_MR,	"\033[7m"},
    {(int)KS_MS,	"y"},
    {(int)KS_UT,	"y"},		// guessed
    {(int)KS_LE,	"\b"},
#  ifdef TERMINFO
    {(int)KS_CM,	"\033[%i%p1%d;%p2%dH"},
#  else
    {(int)KS_CM,	"\033[%i%d;%dH"},
#  endif
#  ifdef TERMINFO
    {(int)KS_CRI,	"\033[%p1%dC"},
#  else
    {(int)KS_CRI,	"\033[%dC"},
#  endif

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * VT320 is working as an ANSI terminal compatible DEC terminal.
 * (it covers VT1x0, VT2x0 and VT3x0 up to VT320 on VMS as well)
 * TODO:- rewrite ESC[ codes to CSI
 *      - keyboard languages (CSI ? 26 n)
 */
static tcap_entry_T builtin_vt320[] = {
    {(int)KS_CE,	"\033[K"},
    {(int)KS_AL,	"\033[L"},
#  ifdef TERMINFO
    {(int)KS_CAL,	"\033[%p1%dL"},
#  else
    {(int)KS_CAL,	"\033[%dL"},
#  endif
    {(int)KS_DL,	"\033[M"},
#  ifdef TERMINFO
    {(int)KS_CDL,	"\033[%p1%dM"},
#  else
    {(int)KS_CDL,	"\033[%dM"},
#  endif
    {(int)KS_CL,	"\033[H\033[2J"},
    {(int)KS_CD,	"\033[J"},
    {(int)KS_CCO,	"8"},			// allow 8 colors
    {(int)KS_ME,	"\033[0m"},
    {(int)KS_MR,	"\033[7m"},
    {(int)KS_MD,	"\033[1m"},  // bold mode
    {(int)KS_SE,	"\033[22m"},// normal mode
    {(int)KS_UE,	"\033[24m"},// exit underscore mode
    {(int)KS_US,	"\033[4m"},  // underscore mode
    {(int)KS_CZH,	"\033[34;43m"},  // italic mode: blue text on yellow
    {(int)KS_CZR,	"\033[0m"},	    // italic mode end
    {(int)KS_CAB,	"\033[4%dm"},	    // set background color (ANSI)
    {(int)KS_CAF,	"\033[3%dm"},	    // set foreground color (ANSI)
    {(int)KS_CSB,	"\033[102;%dm"},	// set screen background color
    {(int)KS_CSF,	"\033[101;%dm"},	// set screen foreground color
    {(int)KS_MS,	"y"},
    {(int)KS_UT,	"y"},
    {(int)KS_XN,	"y"},
    {(int)KS_LE,	"\b"},
#  ifdef TERMINFO
    {(int)KS_CM,	"\033[%i%p1%d;%p2%dH"},
#  else
    {(int)KS_CM,	"\033[%i%d;%dH"},
#  endif
#  ifdef TERMINFO
    {(int)KS_CRI,	"\033[%p1%dC"},
#  else
    {(int)KS_CRI,	"\033[%dC"},
#  endif
    {K_UP,		"\033[A"},
    {K_DOWN,		"\033[B"},
    {K_RIGHT,		"\033[C"},
    {K_LEFT,		"\033[D"},
    // Note: cursor key sequences for application cursor mode are omitted,
    // because they interfere with typed commands: <Esc>OA.
    {K_F1,		"\033[11~"},
    {K_F2,		"\033[12~"},
    {K_F3,		"\033[13~"},
    {K_F4,		"\033[14~"},
    {K_F5,		"\033[15~"},
    {K_F6,		"\033[17~"},
    {K_F7,		"\033[18~"},
    {K_F8,		"\033[19~"},
    {K_F9,		"\033[20~"},
    {K_F10,		"\033[21~"},
    {K_F11,		"\033[23~"},
    {K_F12,		"\033[24~"},
    {K_F13,		"\033[25~"},
    {K_F14,		"\033[26~"},
    {K_F15,		"\033[28~"},	// Help
    {K_F16,		"\033[29~"},	// Select
    {K_F17,		"\033[31~"},
    {K_F18,		"\033[32~"},
    {K_F19,		"\033[33~"},
    {K_F20,		"\033[34~"},
    {K_INS,		"\033[2~"},
    {K_DEL,		"\033[3~"},
    {K_HOME,		"\033[1~"},
    {K_END,		"\033[4~"},
    {K_PAGEUP,		"\033[5~"},
    {K_PAGEDOWN,	"\033[6~"},
    // These sequences starting with <Esc> O may interfere with what the user
    // is typing.  Remove these if that bothers you.
    {K_KPLUS,		"\033Ok"},	// keypad plus
    {K_KMINUS,		"\033Om"},	// keypad minus
    {K_KDIVIDE,		"\033Oo"},	// keypad /
    {K_KMULTIPLY,	"\033Oj"},	// keypad *
    {K_KENTER,		"\033OM"},	// keypad Enter
    {K_K0,		"\033Op"},	// keypad 0
    {K_K1,		"\033Oq"},	// keypad 1
    {K_K2,		"\033Or"},	// keypad 2
    {K_K3,		"\033Os"},	// keypad 3
    {K_K4,		"\033Ot"},	// keypad 4
    {K_K5,		"\033Ou"},	// keypad 5
    {K_K6,		"\033Ov"},	// keypad 6
    {K_K7,		"\033Ow"},	// keypad 7
    {K_K8,		"\033Ox"},	// keypad 8
    {K_K9,		"\033Oy"},	// keypad 9
    {K_BS,		"\x7f"},	// for some reason 0177 doesn't work

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * Ordinary vt52
 */
static tcap_entry_T builtin_vt52[] = {
    {(int)KS_CE,	"\033K"},
    {(int)KS_CD,	"\033J"},
#  ifdef TERMINFO
    {(int)KS_CM,	"\033Y%p1%' '%+%c%p2%' '%+%c"},
#  else
    {(int)KS_CM,	"\033Y%+ %+ "},
#  endif
    {(int)KS_LE,	"\b"},
    {(int)KS_SR,	"\033I"},
    {(int)KS_AL,	"\033L"},
    {(int)KS_DL,	"\033M"},
    {K_UP,		"\033A"},
    {K_DOWN,		"\033B"},
    {K_LEFT,		"\033D"},
    {K_RIGHT,		"\033C"},
    {K_F1,		"\033P"},
    {K_F2,		"\033Q"},
    {K_F3,		"\033R"},
    {(int)KS_CL,	"\033H\033J"},
    {(int)KS_MS,	"y"},

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * Builtin xterm with Vim-specific entries.
 */
static tcap_entry_T builtin_xterm[] = {
    {(int)KS_CE,	"\033[K"},
    {(int)KS_AL,	"\033[L"},
#  ifdef TERMINFO
    {(int)KS_CAL,	"\033[%p1%dL"},
#  else
    {(int)KS_CAL,	"\033[%dL"},
#  endif
    {(int)KS_DL,	"\033[M"},
#  ifdef TERMINFO
    {(int)KS_CDL,	"\033[%p1%dM"},
#  else
    {(int)KS_CDL,	"\033[%dM"},
#  endif
#  ifdef TERMINFO
    {(int)KS_CS,	"\033[%i%p1%d;%p2%dr"},
#  else
    {(int)KS_CS,	"\033[%i%d;%dr"},
#  endif
    {(int)KS_CL,	"\033[H\033[2J"},
    {(int)KS_CD,	"\033[J"},
    {(int)KS_ME,	"\033[m"},
    {(int)KS_MR,	"\033[7m"},
    {(int)KS_MD,	"\033[1m"},
    {(int)KS_UE,	"\033[m"},
    {(int)KS_US,	"\033[4m"},
    {(int)KS_STE,	"\033[29m"},
    {(int)KS_STS,	"\033[9m"},
    {(int)KS_MS,	"y"},
    {(int)KS_UT,	"y"},
    {(int)KS_LE,	"\b"},
    {(int)KS_VI,	"\033[?25l"},
    {(int)KS_VE,	"\033[?25h"},
    {(int)KS_VS,	"\033[?12h"},
    {(int)KS_CVS,	"\033[?12l"},
#  ifdef TERMINFO
    {(int)KS_CSH,	"\033[%p1%d q"},
#  else
    {(int)KS_CSH,	"\033[%d q"},
#  endif
    {(int)KS_CRC,	"\033[?12$p"},
    {(int)KS_CRS,	"\033P$q q\033\\"},
#  ifdef TERMINFO
    {(int)KS_CM,	"\033[%i%p1%d;%p2%dH"},
#  else
    {(int)KS_CM,	"\033[%i%d;%dH"},
#  endif
    {(int)KS_SR,	"\033M"},
#  ifdef TERMINFO
    {(int)KS_CRI,	"\033[%p1%dC"},
#  else
    {(int)KS_CRI,	"\033[%dC"},
#  endif
    {(int)KS_KS,	"\033[?1h\033="},
    {(int)KS_KE,	"\033[?1l\033>"},
#  ifdef FEAT_XTERM_SAVE
    {(int)KS_TI,	"\0337\033[?47h"},
    {(int)KS_TE,	"\033[?47l\0338"},
#  endif
    // These are now under control of the 'keyprotocol' option, see
    // "builtin_mok2".
    // {(int)KS_CTI,	"\033[>4;2m"},
    // {(int)KS_CRK,	"\033[?4m"},
    // {(int)KS_CTE,	"\033[>4;m"},
    {(int)KS_CIS,	"\033]1;"},
    {(int)KS_CIE,	"\007"},
    {(int)KS_TS,	"\033]2;"},
    {(int)KS_FS,	"\007"},
    {(int)KS_CSC,	"\033]12;"},
    {(int)KS_CEC,	"\007"},
#  ifdef TERMINFO
    {(int)KS_CWS,	"\033[8;%p1%d;%p2%dt"},
    {(int)KS_CWP,	"\033[3;%p1%d;%p2%dt"},
    {(int)KS_CGP,	"\033[13t"},
#  else
    {(int)KS_CWS,	"\033[8;%d;%dt"},
    {(int)KS_CWP,	"\033[3;%d;%dt"},
    {(int)KS_CGP,	"\033[13t"},
#  endif
    {(int)KS_CRV,	"\033[>c"},
    {(int)KS_CXM,	"\033[?1006;1000%?%p1%{1}%=%th%el%;"},
    {(int)KS_RFG,	"\033]10;?\007"},
    {(int)KS_RBG,	"\033]11;?\007"},
    {(int)KS_U7,	"\033[6n"},
    {(int)KS_CAU,	"\033[58;5;%dm"},
    {(int)KS_CBE,	"\033[?2004h"},
    {(int)KS_CBD,	"\033[?2004l"},
    {(int)KS_CST,	"\033[22;2t"},
    {(int)KS_CRT,	"\033[23;2t"},
    {(int)KS_SSI,	"\033[22;1t"},
    {(int)KS_SRI,	"\033[23;1t"},
#  if (defined(UNIX) || defined(VMS))
    {(int)KS_FD,	"\033[?1004l"},
    {(int)KS_FE,	"\033[?1004h"},
#  endif

    {K_UP,		"\033O*A"},
    {K_DOWN,		"\033O*B"},
    {K_RIGHT,		"\033O*C"},
    {K_LEFT,		"\033O*D"},
    // An extra set of cursor keys for vt100 mode
    {K_XUP,		"\033[@;*A"},	// Esc [ A or Esc [ 1 ; A
    {K_XDOWN,		"\033[@;*B"},	// Esc [ B or Esc [ 1 ; B
    {K_XRIGHT,		"\033[@;*C"},	// Esc [ C or Esc [ 1 ; C
    {K_XLEFT,		"\033[@;*D"},	// Esc [ D or Esc [ 1 ; D
    // An extra set of function keys for vt100 mode
    {K_XF1,		"\033O*P"},
    {K_XF2,		"\033O*Q"},
    {K_XF3,		"\033O*R"},
    {K_XF4,		"\033O*S"},
    {K_F1,		"\033[11;*~"},
    {K_F2,		"\033[12;*~"},
    {K_F3,		"\033[13;*~"},
    {K_F4,		"\033[14;*~"},
    {K_F5,		"\033[15;*~"},
    {K_F6,		"\033[17;*~"},
    {K_F7,		"\033[18;*~"},
    {K_F8,		"\033[19;*~"},
    {K_F9,		"\033[20;*~"},
    {K_F10,		"\033[21;*~"},
    {K_F11,		"\033[23;*~"},
    {K_F12,		"\033[24;*~"},
    {K_S_TAB,		"\033[Z"},
    {K_HELP,		"\033[28;*~"},
    {K_UNDO,		"\033[26;*~"},
    {K_INS,		"\033[2;*~"},
    {K_HOME,		"\033[@;*H"},    // Esc [ H  or Esc 1 ; H
    // {K_S_HOME,		"\033O2H"},
    // {K_C_HOME,		"\033O5H"},
    {K_KHOME,		"\033[1;*~"},
    {K_XHOME,		"\033O*H"},	// other Home
    {K_ZHOME,		"\033[7;*~"},	// other Home
    {K_END,		"\033[@;*F"},	// Esc [ F or Esc 1 ; F
    // {K_S_END,		"\033O2F"},
    // {K_C_END,		"\033O5F"},
    {K_KEND,		"\033[4;*~"},
    {K_XEND,		"\033O*F"},	// other End
    {K_ZEND,		"\033[8;*~"},
    {K_PAGEUP,		"\033[5;*~"},
    {K_PAGEDOWN,	"\033[6;*~"},
    {K_KPLUS,		"\033O*k"},     // keypad plus
    {K_KMINUS,		"\033O*m"},     // keypad minus
    {K_KDIVIDE,		"\033O*o"},     // keypad /
    {K_KMULTIPLY,	"\033O*j"},     // keypad *
    {K_KENTER,		"\033O*M"},     // keypad Enter
    {K_KPOINT,		"\033O*n"},     // keypad .
    {K_K0,		"\033O*p"},     // keypad 0
    {K_K1,		"\033O*q"},     // keypad 1
    {K_K2,		"\033O*r"},     // keypad 2
    {K_K3,		"\033O*s"},     // keypad 3
    {K_K4,		"\033O*t"},     // keypad 4
    {K_K5,		"\033O*u"},     // keypad 5
    {K_K6,		"\033O*v"},     // keypad 6
    {K_K7,		"\033O*w"},     // keypad 7
    {K_K8,		"\033O*x"},     // keypad 8
    {K_K9,		"\033O*y"},     // keypad 9
    {K_KDEL,		"\033[3;*~"}, // keypad Del
    {K_PS,		"\033[200~"}, // paste start
    {K_PE,		"\033[201~"}, // paste end

    {BT_EXTRA_KEYS,   ""},
    {TERMCAP2KEY('k', '0'), "\033[10;*~"}, // F0
    {TERMCAP2KEY('F', '3'), "\033[25;*~"}, // F13
    // F14 and F15 are missing, because they send the same codes as the undo
    // and help key, although they don't work on all keyboards.
    {TERMCAP2KEY('F', '6'), "\033[29;*~"}, // F16
    {TERMCAP2KEY('F', '7'), "\033[31;*~"}, // F17
    {TERMCAP2KEY('F', '8'), "\033[32;*~"}, // F18
    {TERMCAP2KEY('F', '9'), "\033[33;*~"}, // F19
    {TERMCAP2KEY('F', 'A'), "\033[34;*~"}, // F20

    {TERMCAP2KEY('F', 'B'), "\033[42;*~"}, // F21
    {TERMCAP2KEY('F', 'C'), "\033[43;*~"}, // F22
    {TERMCAP2KEY('F', 'D'), "\033[44;*~"}, // F23
    {TERMCAP2KEY('F', 'E'), "\033[45;*~"}, // F24
    {TERMCAP2KEY('F', 'F'), "\033[46;*~"}, // F25
    {TERMCAP2KEY('F', 'G'), "\033[47;*~"}, // F26
    {TERMCAP2KEY('F', 'H'), "\033[48;*~"}, // F27
    {TERMCAP2KEY('F', 'I'), "\033[49;*~"}, // F28
    {TERMCAP2KEY('F', 'J'), "\033[50;*~"}, // F29
    {TERMCAP2KEY('F', 'K'), "\033[51;*~"}, // F30

    {TERMCAP2KEY('F', 'L'), "\033[52;*~"}, // F31
    {TERMCAP2KEY('F', 'M'), "\033[53;*~"}, // F32
    {TERMCAP2KEY('F', 'N'), "\033[54;*~"}, // F33
    {TERMCAP2KEY('F', 'O'), "\033[55;*~"}, // F34
    {TERMCAP2KEY('F', 'P'), "\033[56;*~"}, // F35
    {TERMCAP2KEY('F', 'Q'), "\033[57;*~"}, // F36
    {TERMCAP2KEY('F', 'R'), "\033[58;*~"}, // F37

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * Additions for using modifyOtherKeys level 2.  Same as what is used for
 * xterm.
 */
static tcap_entry_T builtin_mok2[] = {
    // t_TI enables modifyOtherKeys level 2
    {(int)KS_CTI,	"\033[>4;2m"},

    // XTQMODKEYS was added in xterm version 377: "CSI ? 4 m" which should
    // return "{lead} > 4 ; Pv m".  Before version 377 we expect it to have no
    // effect.
    {(int)KS_CRK,	"\033[?4m"},

    // t_TE disables modifyOtherKeys
    {(int)KS_CTE,	"\033[>4;m"},

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * Additions for using the Kitty keyboard protocol.
 */
static tcap_entry_T builtin_kitty[] = {
    // t_TI enables the kitty keyboard protocol.
    {(int)KS_CTI,	"\033[=1;1u"},

    // t_RK requests the kitty keyboard protocol state
    {(int)KS_CRK,	"\033[?u"},

    // t_TE also disables modifyOtherKeys, because t_TI from xterm may already
    // have been used.
    {(int)KS_CTE,	"\033[>4;m\033[=0;1u"},

    {(int)KS_NAME,	NULL}  // end marker
};

#ifdef FEAT_TERMGUICOLORS
/*
 * Additions for using the RGB colors and terminal font
 */
static tcap_entry_T builtin_rgb[] = {
    // These are printf strings, not terminal codes.
    {(int)KS_8F,	"\033[38;2;%lu;%lu;%lum"},
    {(int)KS_8B,	"\033[48;2;%lu;%lu;%lum"},
    {(int)KS_8U,	"\033[58;2;%lu;%lu;%lum"},

    {(int)KS_NAME,	NULL}  // end marker
};
#endif

#ifdef HAVE_TGETENT
static tcap_entry_T special_term[] = {
    // These are printf strings, not terminal codes.
    {(int)KS_CF,	"\033[%dm"},
    {(int)KS_NAME,	NULL}  // end marker
};
#endif

/*
 * iris-ansi for Silicon Graphics machines.
 */
static tcap_entry_T builtin_iris_ansi[] = {
    {(int)KS_CE,	"\033[K"},
    {(int)KS_CD,	"\033[J"},
    {(int)KS_AL,	"\033[L"},
#  ifdef TERMINFO
    {(int)KS_CAL,	"\033[%p1%dL"},
#  else
    {(int)KS_CAL,	"\033[%dL"},
#  endif
    {(int)KS_DL,	"\033[M"},
#  ifdef TERMINFO
    {(int)KS_CDL,	"\033[%p1%dM"},
#  else
    {(int)KS_CDL,	"\033[%dM"},
#  endif
#if 0	// The scroll region is not working as Vim expects.
#  ifdef TERMINFO
    {(int)KS_CS,	"\033[%i%p1%d;%p2%dr"},
#  else
    {(int)KS_CS,	"\033[%i%d;%dr"},
#  endif
#endif
    {(int)KS_CL,	"\033[H\033[2J"},
    {(int)KS_VE,	"\033[9/y\033[12/y"},	// These aren't documented
    {(int)KS_VS,	"\033[10/y\033[=1h\033[=2l"}, // These aren't documented
    {(int)KS_TI,	"\033[=6h"},
    {(int)KS_TE,	"\033[=6l"},
    {(int)KS_SE,	"\033[21;27m"},
    {(int)KS_SO,	"\033[1;7m"},
    {(int)KS_ME,	"\033[m"},
    {(int)KS_MR,	"\033[7m"},
    {(int)KS_MD,	"\033[1m"},
    {(int)KS_CCO,	"8"},			// allow 8 colors
    {(int)KS_CZH,	"\033[3m"},		// italic mode on
    {(int)KS_CZR,	"\033[23m"},		// italic mode off
    {(int)KS_US,	"\033[4m"},		// underline on
    {(int)KS_UE,	"\033[24m"},		// underline off
#  ifdef TERMINFO
    {(int)KS_CAB,	"\033[4%p1%dm"},    // set background color (ANSI)
    {(int)KS_CAF,	"\033[3%p1%dm"},    // set foreground color (ANSI)
    {(int)KS_CSB,	"\033[102;%p1%dm"}, // set screen background color
    {(int)KS_CSF,	"\033[101;%p1%dm"}, // set screen foreground color
#  else
    {(int)KS_CAB,	"\033[4%dm"},	    // set background color (ANSI)
    {(int)KS_CAF,	"\033[3%dm"},	    // set foreground color (ANSI)
    {(int)KS_CSB,	"\033[102;%dm"},    // set screen background color
    {(int)KS_CSF,	"\033[101;%dm"},    // set screen foreground color
#  endif
    {(int)KS_MS,	"y"},		// guessed
    {(int)KS_UT,	"y"},		// guessed
    {(int)KS_LE,	"\b"},
#  ifdef TERMINFO
    {(int)KS_CM,	"\033[%i%p1%d;%p2%dH"},
#  else
    {(int)KS_CM,	"\033[%i%d;%dH"},
#  endif
    {(int)KS_SR,	"\033M"},
#  ifdef TERMINFO
    {(int)KS_CRI,	"\033[%p1%dC"},
#  else
    {(int)KS_CRI,	"\033[%dC"},
#  endif
    {(int)KS_CIS,	"\033P3.y"},
    {(int)KS_CIE,	"\234"},    // ST "String Terminator"
    {(int)KS_TS,	"\033P1.y"},
    {(int)KS_FS,	"\234"},    // ST "String Terminator"
#  ifdef TERMINFO
    {(int)KS_CWS,	"\033[203;%p1%d;%p2%d/y"},
    {(int)KS_CWP,	"\033[205;%p1%d;%p2%d/y"},
#  else
    {(int)KS_CWS,	"\033[203;%d;%d/y"},
    {(int)KS_CWP,	"\033[205;%d;%d/y"},
#  endif
    {K_UP,		"\033[A"},
    {K_DOWN,		"\033[B"},
    {K_LEFT,		"\033[D"},
    {K_RIGHT,		"\033[C"},
    {K_S_UP,		"\033[161q"},
    {K_S_DOWN,		"\033[164q"},
    {K_S_LEFT,		"\033[158q"},
    {K_S_RIGHT,		"\033[167q"},
    {K_F1,		"\033[001q"},
    {K_F2,		"\033[002q"},
    {K_F3,		"\033[003q"},
    {K_F4,		"\033[004q"},
    {K_F5,		"\033[005q"},
    {K_F6,		"\033[006q"},
    {K_F7,		"\033[007q"},
    {K_F8,		"\033[008q"},
    {K_F9,		"\033[009q"},
    {K_F10,		"\033[010q"},
    {K_F11,		"\033[011q"},
    {K_F12,		"\033[012q"},
    {K_S_F1,		"\033[013q"},
    {K_S_F2,		"\033[014q"},
    {K_S_F3,		"\033[015q"},
    {K_S_F4,		"\033[016q"},
    {K_S_F5,		"\033[017q"},
    {K_S_F6,		"\033[018q"},
    {K_S_F7,		"\033[019q"},
    {K_S_F8,		"\033[020q"},
    {K_S_F9,		"\033[021q"},
    {K_S_F10,		"\033[022q"},
    {K_S_F11,		"\033[023q"},
    {K_S_F12,		"\033[024q"},
    {K_INS,		"\033[139q"},
    {K_HOME,		"\033[H"},
    {K_END,		"\033[146q"},
    {K_PAGEUP,		"\033[150q"},
    {K_PAGEDOWN,	"\033[154q"},

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * These codes are valid when nansi.sys or equivalent has been installed.
 * Function keys on a PC are preceded with a NUL. These are converted into
 * K_NUL '\316' in mch_inchar(), because we cannot handle NULs in key codes.
 * CTRL-arrow is used instead of SHIFT-arrow.
 */
static tcap_entry_T builtin_pcansi[] = {
    {(int)KS_DL,	"\033[M"},
    {(int)KS_AL,	"\033[L"},
    {(int)KS_CE,	"\033[K"},
    {(int)KS_CL,	"\033[2J"},
    {(int)KS_ME,	"\033[0m"},
    {(int)KS_MR,	"\033[5m"},	// reverse: black on lightgrey
    {(int)KS_MD,	"\033[1m"},	// bold: white text
    {(int)KS_SE,	"\033[0m"},	// standout end
    {(int)KS_SO,	"\033[31m"},	// standout: white on blue
    {(int)KS_CZH,	"\033[34;43m"},	// italic mode: blue text on yellow
    {(int)KS_CZR,	"\033[0m"},	// italic mode end
    {(int)KS_US,	"\033[36;41m"},	// underscore mode: cyan text on red
    {(int)KS_UE,	"\033[0m"},	// underscore mode end
    {(int)KS_CCO,	"8"},		// allow 8 colors
#  ifdef TERMINFO
    {(int)KS_CAB,	"\033[4%p1%dm"},// set background color
    {(int)KS_CAF,	"\033[3%p1%dm"},// set foreground color
#  else
    {(int)KS_CAB,	"\033[4%dm"},	// set background color
    {(int)KS_CAF,	"\033[3%dm"},	// set foreground color
#  endif
    {(int)KS_OP,	"\033[0m"},	// reset colors
    {(int)KS_MS,	"y"},
    {(int)KS_UT,	"y"},		// guessed
    {(int)KS_LE,	"\b"},
#  ifdef TERMINFO
    {(int)KS_CM,	"\033[%i%p1%d;%p2%dH"},
#  else
    {(int)KS_CM,	"\033[%i%d;%dH"},
#  endif
#  ifdef TERMINFO
    {(int)KS_CRI,	"\033[%p1%dC"},
#  else
    {(int)KS_CRI,	"\033[%dC"},
#  endif
    {K_UP,		"\316H"},
    {K_DOWN,		"\316P"},
    {K_LEFT,		"\316K"},
    {K_RIGHT,		"\316M"},
    {K_S_LEFT,		"\316s"},
    {K_S_RIGHT,		"\316t"},
    {K_F1,		"\316;"},
    {K_F2,		"\316<"},
    {K_F3,		"\316="},
    {K_F4,		"\316>"},
    {K_F5,		"\316?"},
    {K_F6,		"\316@"},
    {K_F7,		"\316A"},
    {K_F8,		"\316B"},
    {K_F9,		"\316C"},
    {K_F10,		"\316D"},
    {K_F11,		"\316\205"},	// guessed
    {K_F12,		"\316\206"},	// guessed
    {K_S_F1,		"\316T"},
    {K_S_F2,		"\316U"},
    {K_S_F3,		"\316V"},
    {K_S_F4,		"\316W"},
    {K_S_F5,		"\316X"},
    {K_S_F6,		"\316Y"},
    {K_S_F7,		"\316Z"},
    {K_S_F8,		"\316["},
    {K_S_F9,		"\316\\"},
    {K_S_F10,		"\316]"},
    {K_S_F11,		"\316\207"},	// guessed
    {K_S_F12,		"\316\210"},	// guessed
    {K_INS,		"\316R"},
    {K_DEL,		"\316S"},
    {K_HOME,		"\316G"},
    {K_END,		"\316O"},
    {K_PAGEDOWN,	"\316Q"},
    {K_PAGEUP,		"\316I"},

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * These codes are valid for the Win32 Console.  The entries that start with
 * ESC | are translated into console calls in os_win32.c.  The function keys
 * are also translated in os_win32.c.
 */
static tcap_entry_T builtin_win32[] = {
    {(int)KS_CE,	"\033|K"},	// clear to end of line
    {(int)KS_AL,	"\033|L"},	// add new blank line
#  ifdef TERMINFO
    {(int)KS_CAL,	"\033|%p1%dL"},	// add number of new blank lines
#  else
    {(int)KS_CAL,	"\033|%dL"},	// add number of new blank lines
#  endif
    {(int)KS_DL,	"\033|M"},	// delete line
#  ifdef TERMINFO
    {(int)KS_CDL,	"\033|%p1%dM"},	// delete number of lines
    {(int)KS_CSV,	"\033|%p1%d;%p2%dV"},
#  else
    {(int)KS_CDL,	"\033|%dM"},	// delete number of lines
    {(int)KS_CSV,	"\033|%d;%dV"},
#  endif
    {(int)KS_CL,	"\033|J"},	// clear screen
    {(int)KS_CD,	"\033|j"},	// clear to end of display
    {(int)KS_VI,	"\033|v"},	// cursor invisible
    {(int)KS_VE,	"\033|V"},	// cursor visible

    {(int)KS_ME,	"\033|0m"},	// normal
    {(int)KS_MR,	"\033|112m"},	// reverse: black on lightgray
    {(int)KS_MD,	"\033|15m"},	// bold: white on black
#if 1
    {(int)KS_SO,	"\033|31m"},	// standout: white on blue
    {(int)KS_SE,	"\033|0m"},	// standout end
#else
    {(int)KS_SO,	"\033|F"},	// standout: high intensity
    {(int)KS_SE,	"\033|f"},	// standout end
#endif
    {(int)KS_CZH,	"\033|225m"},	// italic: blue text on yellow
    {(int)KS_CZR,	"\033|0m"},	// italic end
    {(int)KS_US,	"\033|67m"},	// underscore: cyan text on red
    {(int)KS_UE,	"\033|0m"},	// underscore end
    {(int)KS_CCO,	"16"},		// allow 16 colors
#  ifdef TERMINFO
    {(int)KS_CAB,	"\033|%p1%db"},	// set background color
    {(int)KS_CAF,	"\033|%p1%df"},	// set foreground color
#  else
    {(int)KS_CAB,	"\033|%db"},	// set background color
    {(int)KS_CAF,	"\033|%df"},	// set foreground color
#  endif

    {(int)KS_MS,	"y"},		// save to move cur in reverse mode
    {(int)KS_UT,	"y"},
    {(int)KS_XN,	"y"},
    {(int)KS_LE,	"\b"},
#  ifdef TERMINFO
    {(int)KS_CM,	"\033|%i%p1%d;%p2%dH"}, // cursor motion
#  else
    {(int)KS_CM,	"\033|%i%d;%dH"}, // cursor motion
#  endif
    {(int)KS_VB,	"\033|B"},	// visual bell
    {(int)KS_TI,	"\033|S"},	// put terminal in termcap mode
    {(int)KS_TE,	"\033|E"},	// out of termcap mode
#  ifdef TERMINFO
    {(int)KS_CS,	"\033|%i%p1%d;%p2%dr"}, // scroll region
#  else
    {(int)KS_CS,	"\033|%i%d;%dr"}, // scroll region
#  endif

    {K_UP,		"\316H"},
    {K_DOWN,		"\316P"},
    {K_LEFT,		"\316K"},
    {K_RIGHT,		"\316M"},
    {K_S_UP,		"\316\304"},
    {K_S_DOWN,		"\316\317"},
    {K_S_LEFT,		"\316\311"},
    {K_C_LEFT,		"\316s"},
    {K_S_RIGHT,		"\316\313"},
    {K_C_RIGHT,		"\316t"},
    {K_S_TAB,		"\316\017"},
    {K_F1,		"\316;"},
    {K_F2,		"\316<"},
    {K_F3,		"\316="},
    {K_F4,		"\316>"},
    {K_F5,		"\316?"},
    {K_F6,		"\316@"},
    {K_F7,		"\316A"},
    {K_F8,		"\316B"},
    {K_F9,		"\316C"},
    {K_F10,		"\316D"},
    {K_F11,		"\316\205"},
    {K_F12,		"\316\206"},
    {K_S_F1,		"\316T"},
    {K_S_F2,		"\316U"},
    {K_S_F3,		"\316V"},
    {K_S_F4,		"\316W"},
    {K_S_F5,		"\316X"},
    {K_S_F6,		"\316Y"},
    {K_S_F7,		"\316Z"},
    {K_S_F8,		"\316["},
    {K_S_F9,		"\316\\"},
    {K_S_F10,		"\316]"},
    {K_S_F11,		"\316\207"},
    {K_S_F12,		"\316\210"},
    {K_INS,		"\316R"},
    {K_DEL,		"\316S"},
    {K_HOME,		"\316G"},
    {K_S_HOME,		"\316\302"},
    {K_C_HOME,		"\316w"},
    {K_END,		"\316O"},
    {K_S_END,		"\316\315"},
    {K_C_END,		"\316u"},
    {K_PAGEDOWN,	"\316Q"},
    {K_PAGEUP,		"\316I"},
    {K_KPLUS,		"\316N"},
    {K_KMINUS,		"\316J"},
    {K_KMULTIPLY,	"\316\067"},
    {K_K0,		"\316\332"},
    {K_K1,		"\316\336"},
    {K_K2,		"\316\342"},
    {K_K3,		"\316\346"},
    {K_K4,		"\316\352"},
    {K_K5,		"\316\356"},
    {K_K6,		"\316\362"},
    {K_K7,		"\316\366"},
    {K_K8,		"\316\372"},
    {K_K9,		"\316\376"},
    {K_BS,		"\316x"},
    {K_S_BS,		"\316y"},

    {(int)KS_NAME,	NULL}  // end marker
};

#if defined(FEAT_GUI)
/*
 * GUI uses made-up codes, only used inside Vim.
 */
static tcap_entry_T builtin_gui[] = {
    {(int)KS_CE,	"\033|$"},
    {(int)KS_AL,	"\033|i"},
# ifdef TERMINFO
    {(int)KS_CAL,	"\033|%p1%dI"},
# else
    {(int)KS_CAL,	"\033|%dI"},
# endif
    {(int)KS_DL,	"\033|d"},
# ifdef TERMINFO
    {(int)KS_CDL,	"\033|%p1%dD"},
    {(int)KS_CS,	"\033|%p1%d;%p2%dR"},
    {(int)KS_CSV,	"\033|%p1%d;%p2%dV"},
# else
    {(int)KS_CDL,	"\033|%dD"},
    {(int)KS_CS,	"\033|%d;%dR"},
    {(int)KS_CSV,	"\033|%d;%dV"},
# endif
    {(int)KS_CL,	"\033|C"},
			// attributes switched on with 'h', off with * 'H'
    {(int)KS_ME,	"\033|31H"}, // HL_ALL
    {(int)KS_MR,	"\033|1h"},   // HL_INVERSE
    {(int)KS_MD,	"\033|2h"},   // HL_BOLD
    {(int)KS_SE,	"\033|16H"}, // HL_STANDOUT
    {(int)KS_SO,	"\033|16h"}, // HL_STANDOUT
    {(int)KS_UE,	"\033|8H"},   // HL_UNDERLINE
    {(int)KS_US,	"\033|8h"},   // HL_UNDERLINE
    {(int)KS_UCE,	"\033|8C"},   // HL_UNDERCURL
    {(int)KS_UCS,	"\033|8c"},   // HL_UNDERCURL
    {(int)KS_STE,	"\033|4C"},   // HL_STRIKETHROUGH
    {(int)KS_STS,	"\033|4c"},   // HL_STRIKETHROUGH
    {(int)KS_CZR,	"\033|4H"},   // HL_ITALIC
    {(int)KS_CZH,	"\033|4h"},   // HL_ITALIC
    {(int)KS_VB,	"\033|f"},
    {(int)KS_MS,	"y"},
    {(int)KS_UT,	"y"},
    {(int)KS_XN,	"y"},
    {(int)KS_LE,	"\b"},		// cursor-left = BS
    {(int)KS_ND,	"\014"},	// cursor-right = CTRL-L
# ifdef TERMINFO
    {(int)KS_CM,	"\033|%p1%d;%p2%dM"},
# else
    {(int)KS_CM,	"\033|%d;%dM"},
# endif
	// there are no key sequences here, the GUI sequences are recognized
	// in check_termcode()

    {(int)KS_NAME,	NULL}  // end marker
};
#endif

/*
 * Amiga console window, default for Amiga.
 */
static tcap_entry_T builtin_amiga[] = {
    {(int)KS_CE,	"\033[K"},
    {(int)KS_CD,	"\033[J"},
    {(int)KS_AL,	"\033[L"},
#  ifdef TERMINFO
    {(int)KS_CAL,	"\033[%p1%dL"},
#  else
    {(int)KS_CAL,	"\033[%dL"},
#  endif
    {(int)KS_DL,	"\033[M"},
#  ifdef TERMINFO
    {(int)KS_CDL,	"\033[%p1%dM"},
#  else
    {(int)KS_CDL,	"\033[%dM"},
#  endif
    {(int)KS_CL,	"\014"},
    {(int)KS_VI,	"\033[0 p"},
    {(int)KS_VE,	"\033[1 p"},
    {(int)KS_ME,	"\033[0m"},
    {(int)KS_MR,	"\033[7m"},
    {(int)KS_MD,	"\033[1m"},
    {(int)KS_SE,	"\033[0m"},
    {(int)KS_SO,	"\033[33m"},
    {(int)KS_US,	"\033[4m"},
    {(int)KS_UE,	"\033[0m"},
    {(int)KS_CZH,	"\033[3m"},
    {(int)KS_CZR,	"\033[0m"},
#if defined(__amigaos4__) || defined(__MORPHOS__) || defined(__AROS__)
    {(int)KS_CCO,	"8"},		// allow 8 colors
#  ifdef TERMINFO
    {(int)KS_CAB,	"\033[4%p1%dm"},// set background color
    {(int)KS_CAF,	"\033[3%p1%dm"},// set foreground color
#  else
    {(int)KS_CAB,	"\033[4%dm"},	// set background color
    {(int)KS_CAF,	"\033[3%dm"},	// set foreground color
#  endif
    {(int)KS_OP,	"\033[m"},	// reset colors
#endif
    {(int)KS_MS,	"y"},
    {(int)KS_UT,	"y"},		// guessed
    {(int)KS_LE,	"\b"},
#  ifdef TERMINFO
    {(int)KS_CM,	"\033[%i%p1%d;%p2%dH"},
#  else
    {(int)KS_CM,	"\033[%i%d;%dH"},
#  endif
#if defined(__MORPHOS__)
    {(int)KS_SR,	"\033M"},
#endif
#  ifdef TERMINFO
    {(int)KS_CRI,	"\033[%p1%dC"},
#  else
    {(int)KS_CRI,	"\033[%dC"},
#  endif
    {K_UP,		"\233A"},
    {K_DOWN,		"\233B"},
    {K_LEFT,		"\233D"},
    {K_RIGHT,		"\233C"},
    {K_S_UP,		"\233T"},
    {K_S_DOWN,		"\233S"},
    {K_S_LEFT,		"\233 A"},
    {K_S_RIGHT,		"\233 @"},
    {K_S_TAB,		"\233Z"},
    {K_F1,		"\233\060~"},// some compilers don't dig "\2330"
    {K_F2,		"\233\061~"},
    {K_F3,		"\233\062~"},
    {K_F4,		"\233\063~"},
    {K_F5,		"\233\064~"},
    {K_F6,		"\233\065~"},
    {K_F7,		"\233\066~"},
    {K_F8,		"\233\067~"},
    {K_F9,		"\233\070~"},
    {K_F10,		"\233\071~"},
    {K_S_F1,		"\233\061\060~"},
    {K_S_F2,		"\233\061\061~"},
    {K_S_F3,		"\233\061\062~"},
    {K_S_F4,		"\233\061\063~"},
    {K_S_F5,		"\233\061\064~"},
    {K_S_F6,		"\233\061\065~"},
    {K_S_F7,		"\233\061\066~"},
    {K_S_F8,		"\233\061\067~"},
    {K_S_F9,		"\233\061\070~"},
    {K_S_F10,		"\233\061\071~"},
    {K_HELP,		"\233?~"},
    {K_INS,		"\233\064\060~"},	// 101 key keyboard
    {K_PAGEUP,		"\233\064\061~"},	// 101 key keyboard
    {K_PAGEDOWN,	"\233\064\062~"},	// 101 key keyboard
    {K_HOME,		"\233\064\064~"},	// 101 key keyboard
    {K_END,		"\233\064\065~"},	// 101 key keyboard

    {BT_EXTRA_KEYS,	""},
    {TERMCAP2KEY('#', '2'), "\233\065\064~"},	// shifted home key
    {TERMCAP2KEY('#', '3'), "\233\065\060~"},	// shifted insert key
    {TERMCAP2KEY('*', '7'), "\233\065\065~"},	// shifted end key

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * The most minimal terminal: only clear screen and cursor positioning.
 */
static tcap_entry_T builtin_dumb[] = {
    {(int)KS_CL,	"\014"},
#ifdef TERMINFO
    {(int)KS_CM,	"\033[%i%p1%d;%p2%dH"},
#else
    {(int)KS_CM,	"\033[%i%d;%dH"},
#endif

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * Terminal used for debugging.
 */
static tcap_entry_T builtin_debug[] = {
    {(int)KS_CE,	"[CE]"},
    {(int)KS_CD,	"[CD]"},
    {(int)KS_AL,	"[AL]"},
#  ifdef TERMINFO
    {(int)KS_CAL,	"[CAL%p1%d]"},
#  else
    {(int)KS_CAL,	"[CAL%d]"},
#  endif
    {(int)KS_DL,	"[DL]"},
#  ifdef TERMINFO
    {(int)KS_CDL,	"[CDL%p1%d]"},
#  else
    {(int)KS_CDL,	"[CDL%d]"},
#  endif
#  ifdef TERMINFO
    {(int)KS_CS,	"[%p1%dCS%p2%d]"},
#  else
    {(int)KS_CS,	"[%dCS%d]"},
#  endif
#  ifdef TERMINFO
    {(int)KS_CSV,	"[%p1%dCSV%p2%d]"},
#  else
    {(int)KS_CSV,	"[%dCSV%d]"},
#  endif
#  ifdef TERMINFO
    {(int)KS_CAB,	"[CAB%p1%d]"},
    {(int)KS_CAF,	"[CAF%p1%d]"},
    {(int)KS_CSB,	"[CSB%p1%d]"},
    {(int)KS_CSF,	"[CSF%p1%d]"},
#  else
    {(int)KS_CAB,	"[CAB%d]"},
    {(int)KS_CAF,	"[CAF%d]"},
    {(int)KS_CSB,	"[CSB%d]"},
    {(int)KS_CSF,	"[CSF%d]"},
#  endif
    {(int)KS_CAU,	"[CAU%d]"},
    {(int)KS_OP,	"[OP]"},
    {(int)KS_LE,	"[LE]"},
    {(int)KS_CL,	"[CL]"},
    {(int)KS_VI,	"[VI]"},
    {(int)KS_VE,	"[VE]"},
    {(int)KS_VS,	"[VS]"},
    {(int)KS_ME,	"[ME]"},
    {(int)KS_MR,	"[MR]"},
    {(int)KS_MB,	"[MB]"},
    {(int)KS_MD,	"[MD]"},
    {(int)KS_SE,	"[SE]"},
    {(int)KS_SO,	"[SO]"},
    {(int)KS_UE,	"[UE]"},
    {(int)KS_US,	"[US]"},
    {(int)KS_UCE,	"[UCE]"},
    {(int)KS_UCS,	"[UCS]"},
    {(int)KS_USS,	"[USS]"},
    {(int)KS_DS,	"[DS]"},
    {(int)KS_CDS,	"[CDS]"},
    {(int)KS_STE,	"[STE]"},
    {(int)KS_STS,	"[STS]"},
    {(int)KS_MS,	"[MS]"},
    {(int)KS_UT,	"[UT]"},
    {(int)KS_XN,	"[XN]"},
#  ifdef TERMINFO
    {(int)KS_CM,	"[%p1%dCM%p2%d]"},
#  else
    {(int)KS_CM,	"[%dCM%d]"},
#  endif
    {(int)KS_SR,	"[SR]"},
#  ifdef TERMINFO
    {(int)KS_CRI,	"[CRI%p1%d]"},
#  else
    {(int)KS_CRI,	"[CRI%d]"},
#  endif
    {(int)KS_VB,	"[VB]"},
    {(int)KS_KS,	"[KS]"},
    {(int)KS_KE,	"[KE]"},
    {(int)KS_TI,	"[TI]"},
    {(int)KS_TE,	"[TE]"},
    {(int)KS_CIS,	"[CIS]"},
    {(int)KS_CIE,	"[CIE]"},
    {(int)KS_CSC,	"[CSC]"},
    {(int)KS_CEC,	"[CEC]"},
    {(int)KS_TS,	"[TS]"},
    {(int)KS_FS,	"[FS]"},
#  ifdef TERMINFO
    {(int)KS_CWS,	"[%p1%dCWS%p2%d]"},
    {(int)KS_CWP,	"[%p1%dCWP%p2%d]"},
#  else
    {(int)KS_CWS,	"[%dCWS%d]"},
    {(int)KS_CWP,	"[%dCWP%d]"},
#  endif
    {(int)KS_CRV,	"[CRV]"},
    {(int)KS_CXM,	"[CXM]"},
    {(int)KS_U7,	"[U7]"},
    {(int)KS_RFG,	"[RFG]"},
    {(int)KS_RBG,	"[RBG]"},
    {(int)KS_CF,	"[CF%d]"},
    {K_UP,		"[KU]"},
    {K_DOWN,		"[KD]"},
    {K_LEFT,		"[KL]"},
    {K_RIGHT,		"[KR]"},
    {K_XUP,		"[xKU]"},
    {K_XDOWN,		"[xKD]"},
    {K_XLEFT,		"[xKL]"},
    {K_XRIGHT,		"[xKR]"},
    {K_S_UP,		"[S-KU]"},
    {K_S_DOWN,		"[S-KD]"},
    {K_S_LEFT,		"[S-KL]"},
    {K_C_LEFT,		"[C-KL]"},
    {K_S_RIGHT,		"[S-KR]"},
    {K_C_RIGHT,		"[C-KR]"},
    {K_F1,		"[F1]"},
    {K_XF1,		"[xF1]"},
    {K_F2,		"[F2]"},
    {K_XF2,		"[xF2]"},
    {K_F3,		"[F3]"},
    {K_XF3,		"[xF3]"},
    {K_F4,		"[F4]"},
    {K_XF4,		"[xF4]"},
    {K_F5,		"[F5]"},
    {K_F6,		"[F6]"},
    {K_F7,		"[F7]"},
    {K_F8,		"[F8]"},
    {K_F9,		"[F9]"},
    {K_F10,		"[F10]"},
    {K_F11,		"[F11]"},
    {K_F12,		"[F12]"},
    {K_S_F1,		"[S-F1]"},
    {K_S_XF1,		"[S-xF1]"},
    {K_S_F2,		"[S-F2]"},
    {K_S_XF2,		"[S-xF2]"},
    {K_S_F3,		"[S-F3]"},
    {K_S_XF3,		"[S-xF3]"},
    {K_S_F4,		"[S-F4]"},
    {K_S_XF4,		"[S-xF4]"},
    {K_S_F5,		"[S-F5]"},
    {K_S_F6,		"[S-F6]"},
    {K_S_F7,		"[S-F7]"},
    {K_S_F8,		"[S-F8]"},
    {K_S_F9,		"[S-F9]"},
    {K_S_F10,		"[S-F10]"},
    {K_S_F11,		"[S-F11]"},
    {K_S_F12,		"[S-F12]"},
    {K_HELP,		"[HELP]"},
    {K_UNDO,		"[UNDO]"},
    {K_BS,		"[BS]"},
    {K_INS,		"[INS]"},
    {K_KINS,		"[KINS]"},
    {K_DEL,		"[DEL]"},
    {K_KDEL,		"[KDEL]"},
    {K_HOME,		"[HOME]"},
    {K_S_HOME,		"[C-HOME]"},
    {K_C_HOME,		"[C-HOME]"},
    {K_KHOME,		"[KHOME]"},
    {K_XHOME,		"[XHOME]"},
    {K_ZHOME,		"[ZHOME]"},
    {K_END,		"[END]"},
    {K_S_END,		"[C-END]"},
    {K_C_END,		"[C-END]"},
    {K_KEND,		"[KEND]"},
    {K_XEND,		"[XEND]"},
    {K_ZEND,		"[ZEND]"},
    {K_PAGEUP,		"[PAGEUP]"},
    {K_PAGEDOWN,	"[PAGEDOWN]"},
    {K_KPAGEUP,		"[KPAGEUP]"},
    {K_KPAGEDOWN,	"[KPAGEDOWN]"},
    {K_MOUSE,		"[MOUSE]"},
    {K_KPLUS,		"[KPLUS]"},
    {K_KMINUS,		"[KMINUS]"},
    {K_KDIVIDE,		"[KDIVIDE]"},
    {K_KMULTIPLY,	"[KMULTIPLY]"},
    {K_KENTER,		"[KENTER]"},
    {K_KPOINT,		"[KPOINT]"},
    {K_PS,		"[PASTE-START]"},
    {K_PE,		"[PASTE-END]"},
    {K_K0,		"[K0]"},
    {K_K1,		"[K1]"},
    {K_K2,		"[K2]"},
    {K_K3,		"[K3]"},
    {K_K4,		"[K4]"},
    {K_K5,		"[K5]"},
    {K_K6,		"[K6]"},
    {K_K7,		"[K7]"},
    {K_K8,		"[K8]"},
    {K_K9,		"[K9]"},

    {(int)KS_NAME,	NULL}  // end marker
};

/*
 * List of builtin terminals.
 */
typedef struct {
    char	    *bitc_name;	    // name, such as "xterm"
    tcap_entry_T    *bitc_table;    // table with entries for bitc_name
} builtin_tcap_T;

builtin_tcap_T builtin_terminals[] = {
    // Unix and Generic
    {"ansi",	    builtin_ansi},
    {"vt320",	    builtin_vt320},
    {"vt52",	    builtin_vt52},
    {"xterm",	    builtin_xterm},
    {"iris-ansi",   builtin_iris_ansi},

    // MS-Windows
    {"pcansi",	    builtin_pcansi},
    {"win32",	    builtin_win32},

    // Other systems
#if defined(FEAT_GUI)
    {"gui",	    builtin_gui},
#endif
    {"amiga",	    builtin_amiga},
    {"dumb",	    builtin_dumb},
    {"debug",	    builtin_debug},

    {NULL,	    NULL},  // end marker
};

#if defined(FEAT_TERMGUICOLORS) || defined(PROTO)
    static guicolor_T
termgui_mch_get_color(char_u *name)
{
    return gui_get_color_cmn(name);
}

    guicolor_T
termgui_get_color(char_u *name)
{
    guicolor_T	t;

    if (*name == NUL)
	return INVALCOLOR;
    t = termgui_mch_get_color(name);

    if (t == INVALCOLOR)
	semsg(_(e_cannot_allocate_color_str), name);
    return t;
}

    guicolor_T
termgui_mch_get_rgb(guicolor_T color)
{
    return color;
}
#endif

/*
 * DEFAULT_TERM is used, when no terminal is specified with -T option or $TERM.
 */
#ifdef AMIGA
# define DEFAULT_TERM	(char_u *)"amiga"
#endif

#ifdef MSWIN
# define DEFAULT_TERM	(char_u *)"win32"
#endif

#if defined(UNIX)
# define DEFAULT_TERM	(char_u *)"ansi"
#endif

#ifdef VMS
# define DEFAULT_TERM	(char_u *)"vt320"
#endif

#ifdef __HAIKU__
# undef DEFAULT_TERM
# define DEFAULT_TERM	(char_u *)"xterm"
#endif

#ifndef DEFAULT_TERM
# define DEFAULT_TERM	(char_u *)"dumb"
#endif

/*
 * Term_strings contains currently used terminal output strings.
 * It is initialized with the default values by parse_builtin_tcap().
 * The values can be changed by setting the option with the same name.
 */
char_u *(term_strings[(int)KS_LAST + 1]);

static int	need_gather = FALSE;	    // need to fill termleader[]
static char_u	termleader[256 + 1];	    // for check_termcode()
#ifdef FEAT_TERMRESPONSE
static int	check_for_codes = FALSE;    // check for key code response
#endif

/*
 * Structure and table to store terminal features that can be detected by
 * querying the terminal.  Either by inspecting the termresponse or a more
 * specific request.  Besides this there are:
 * t_colors - number of colors supported
 */
typedef struct {
    char    *tpr_name;
    int	    tpr_set_by_termresponse;
    int	    tpr_status;
} termprop_T;

// Values for tpr_status.
#define TPR_UNKNOWN	    'u'
#define TPR_YES		    'y'
#define TPR_NO		    'n'
#define TPR_MOUSE_XTERM     'x'	// use "xterm" for 'ttymouse'
#define TPR_MOUSE_XTERM2    '2'	// use "xterm2" for 'ttymouse'
#define TPR_MOUSE_SGR	    's'	// use "sgr" for 'ttymouse'

// can request the cursor style without messing up the display
#define TPR_CURSOR_STYLE	    0
// can request the cursor blink mode without messing up the display
#define TPR_CURSOR_BLINK	    1
// can set the underline color with t_8u without resetting other colors
#define TPR_UNDERLINE_RGB	    2
// mouse support - TPR_MOUSE_XTERM, TPR_MOUSE_XTERM2 or TPR_MOUSE_SGR
#define TPR_MOUSE		    3
// term response indicates kitty
#define TPR_KITTY		    4
// table size
#define TPR_COUNT		    5

static termprop_T term_props[TPR_COUNT];

/*
 * Initialize the term_props table.
 * When "all" is FALSE only set those that are detected from the version
 * response.
 */
    void
init_term_props(int all)
{
    int i;

    term_props[TPR_CURSOR_STYLE].tpr_name = "cursor_style";
    term_props[TPR_CURSOR_STYLE].tpr_set_by_termresponse = FALSE;
    term_props[TPR_CURSOR_BLINK].tpr_name = "cursor_blink_mode";
    term_props[TPR_CURSOR_BLINK].tpr_set_by_termresponse = FALSE;
    term_props[TPR_UNDERLINE_RGB].tpr_name = "underline_rgb";
    term_props[TPR_UNDERLINE_RGB].tpr_set_by_termresponse = TRUE;
    term_props[TPR_MOUSE].tpr_name = "mouse";
    term_props[TPR_MOUSE].tpr_set_by_termresponse = TRUE;
    term_props[TPR_KITTY].tpr_name = "kitty";
    term_props[TPR_KITTY].tpr_set_by_termresponse = FALSE;

    for (i = 0; i < TPR_COUNT; ++i)
	if (all || term_props[i].tpr_set_by_termresponse)
	    term_props[i].tpr_status = TPR_UNKNOWN;
}

#if defined(FEAT_EVAL) || defined(PROTO)
    void
f_terminalprops(typval_T *argvars UNUSED, typval_T *rettv)
{
# ifdef FEAT_TERMRESPONSE
    int i;
# endif

    if (rettv_dict_alloc(rettv) == FAIL)
	return;
# ifdef FEAT_TERMRESPONSE
    for (i = 0; i < TPR_COUNT; ++i)
    {
	char_u	value[2];

	value[0] = term_props[i].tpr_status;
	value[1] = NUL;
	dict_add_string(rettv->vval.v_dict, term_props[i].tpr_name, value);
    }
# endif
}
#endif

/*
 * Find the builtin termcap entries for "term".
 * This also recognizes similar names.  E.g. "xterm-256color" finds the "xterm"
 * entry.
 * Returns NULL when "term" is not found.
 */
    static tcap_entry_T *
find_builtin_term(char_u *term)
{
    for (int i = 0; ; ++i)
    {
	char_u *name = (char_u *)builtin_terminals[i].bitc_name;
	if (name == NULL)  // end marker
	    break;
#ifdef UNIX
	if (STRCMP(name, "iris-ansi") == 0 && vim_is_iris(term))
	    return builtin_terminals[i].bitc_table;
	if (STRCMP(name, "xterm") == 0 && vim_is_xterm(term))
	    return builtin_terminals[i].bitc_table;
#endif
#ifdef VMS
	if (STRCMP(name, "vt320") == 0 && vim_is_vt300(term))
	    return builtin_terminals[i].bitc_table;
#endif
	if (STRCMP(term, name) == 0)
	    return builtin_terminals[i].bitc_table;
    }
    return NULL;
}

/*
 * Apply entries from a builtin termcap.
 */
    static void
apply_builtin_tcap(char_u *term, tcap_entry_T *entries, int overwrite)
{
    int term_8bit = term_is_8bit(term);

    for (tcap_entry_T *p = entries;
	      p->bt_entry != (int)KS_NAME && p->bt_entry != BT_EXTRA_KEYS; ++p)
    {
	if ((int)p->bt_entry >= 0)	// KS_xx entry
	{
	    // Only set the value if it wasn't set yet or "overwrite" is TRUE.
	    if (term_strings[p->bt_entry] == NULL
		    || term_strings[p->bt_entry] == empty_option
		    || overwrite)
	    {
#ifdef FEAT_EVAL
		int opt_idx = -1;
#endif
		// 8bit terminal: use CSI instead of <Esc>[
		if (term_8bit && term_7to8bit((char_u *)p->bt_string) != 0)
		{
		    char_u  *s, *t;

		    s = vim_strsave((char_u *)p->bt_string);
		    if (s != NULL)
		    {
			for (t = s; *t; ++t)
			    if (term_7to8bit(t))
			    {
				*t = term_7to8bit(t);
				STRMOVE(t + 1, t + 2);
			    }
			term_strings[p->bt_entry] = s;
#ifdef FEAT_EVAL
			opt_idx =
#endif
				  set_term_option_alloced(
						   &term_strings[p->bt_entry]);
		    }
		}
		else
		{
		    term_strings[p->bt_entry] = (char_u *)p->bt_string;
#ifdef FEAT_EVAL
		    opt_idx = get_term_opt_idx(&term_strings[p->bt_entry]);
#endif
		}
#ifdef FEAT_EVAL
		set_term_option_sctx_idx(NULL, opt_idx);
#endif
	    }
	}
	else
	{
	    char_u  name[2];
	    name[0] = KEY2TERMCAP0((int)p->bt_entry);
	    name[1] = KEY2TERMCAP1((int)p->bt_entry);
	    if (find_termcode(name) == NULL || overwrite)
		add_termcode(name, (char_u *)p->bt_string, term_8bit);
	}
    }
}

/*
 * Apply builtin termcap entries for a given keyprotocol.
 */
    void
apply_keyprotocol(char_u *term, keyprot_T prot)
{
    if (prot == KEYPROTOCOL_KITTY)
	apply_builtin_tcap(term, builtin_kitty, TRUE);
    if (prot == KEYPROTOCOL_MOK2)
	apply_builtin_tcap(term, builtin_mok2, TRUE);

    if (prot != KEYPROTOCOL_NONE)
	// Some function keys may accept modifiers even though the
	// terminfo/termcap entry does not indicate this.
	accept_modifiers_for_function_keys();
}

/*
 * Parsing of the builtin termcap entries.
 * Caller should check if "term" is a valid builtin terminal name.
 * The terminal's name is not set, as this is already done in termcapinit().
 */
    static void
parse_builtin_tcap(char_u *term)
{
    tcap_entry_T *entries = find_builtin_term(term);
    if (entries != NULL)
	apply_builtin_tcap(term, entries, FALSE);
}

/*
 * Set number of colors.
 * Store it as a number in t_colors.
 * Store it as a string in T_CCO (using nr_colors[]).
 */
    void
set_color_count(int nr)
{
    char_u	nr_colors[20];		// string for number of colors

    t_colors = nr;
    if (t_colors > 1)
	sprintf((char *)nr_colors, "%d", t_colors);
    else
	*nr_colors = NUL;
    set_string_option_direct((char_u *)"t_Co", -1, nr_colors, OPT_FREE, 0);
}

/*
 * Set the color count to "val" and redraw if it changed.
 */
    static void
may_adjust_color_count(int val)
{
    if (val == t_colors)
	return;

    // Nr of colors changed, initialize highlighting and redraw everything.
    // This causes a redraw, which usually clears the message.  Try keeping
    // the message if it might work.
    set_keep_msg_from_hist();
    set_color_count(val);
    init_highlight(TRUE, FALSE);
#ifdef DEBUG_TERMRESPONSE
    {
	int r = redraw_asap(UPD_CLEAR);

	log_tr("Received t_Co, redraw_asap(): %d", r);
    }
#else
    redraw_asap(UPD_CLEAR);
#endif
}

#ifdef HAVE_TGETENT
static char *(key_names[]) =
{
# ifdef FEAT_TERMRESPONSE
    // Do this one first, it may cause a screen redraw.
    "Co",
# endif
    "ku", "kd", "kr", "kl",
    "#2", "#4", "%i", "*7",
    "k1", "k2", "k3", "k4", "k5", "k6",
    "k7", "k8", "k9", "k;", "F1", "F2",
    "%1", "&8", "kb", "kI", "kD", "kh",
    "@7", "kP", "kN", "K1", "K3", "K4", "K5", "kB",
    "PS", "PE",
    NULL
};
#endif

#if defined(HAVE_TGETENT) || defined(FEAT_TERMGUICOLORS)
/*
 * Return TRUE if "term_strings[idx]" was not set.
 */
    static int
term_strings_not_set(enum SpecialKey idx)
{
    return TERM_STR(idx) == NULL || TERM_STR(idx) == empty_option;
}
#endif

#ifdef HAVE_TGETENT
/*
 * Get the termcap entries we need with tgetstr(), tgetflag() and tgetnum().
 * "invoke_tgetent()" must have been called before.
 * If "*height" or "*width" are not zero then use the "li" and "col" entries to
 * get their value.
 */
    static void
get_term_entries(int *height, int *width)
{
    static struct {
		    enum SpecialKey dest; // index in term_strings[]
		    char *name;		  // termcap name for string
		  } string_names[] =
		    {	{KS_CE, "ce"}, {KS_AL, "al"}, {KS_CAL,"AL"},
			{KS_DL, "dl"}, {KS_CDL,"DL"}, {KS_CS, "cs"},
			{KS_CL, "cl"}, {KS_CD, "cd"},
			{KS_VI, "vi"}, {KS_VE, "ve"}, {KS_MB, "mb"},
			{KS_ME, "me"}, {KS_MR, "mr"},
			{KS_MD, "md"}, {KS_SE, "se"}, {KS_SO, "so"},
			{KS_CZH,"ZH"}, {KS_CZR,"ZR"}, {KS_UE, "ue"},
			{KS_US, "us"}, {KS_UCE, "Ce"}, {KS_UCS, "Cs"},
			{KS_USS, "Us"}, {KS_DS, "ds"}, {KS_CDS, "Ds"},
			{KS_STE,"Te"}, {KS_STS,"Ts"},
			{KS_CM, "cm"}, {KS_SR, "sr"},
			{KS_CRI,"RI"}, {KS_VB, "vb"}, {KS_KS, "ks"},
			{KS_KE, "ke"}, {KS_TI, "ti"}, {KS_TE, "te"},
			{KS_CTI, "TI"}, {KS_CRK, "RK"}, {KS_CTE, "TE"},
			{KS_BC, "bc"}, {KS_CSB,"Sb"}, {KS_CSF,"Sf"},
			{KS_CAB,"AB"}, {KS_CAF,"AF"}, {KS_CAU,"AU"},
			{KS_LE, "le"},
			{KS_ND, "nd"}, {KS_OP, "op"},
			{KS_CRV, "RV"}, {KS_CXM, "XM"},
			{KS_VS, "vs"}, {KS_CVS, "VS"},
			{KS_CIS, "IS"}, {KS_CIE, "IE"},
			{KS_CSC, "SC"}, {KS_CEC, "EC"},
			{KS_TS, "ts"}, {KS_FS, "fs"},
			{KS_CWP, "WP"}, {KS_CWS, "WS"},
			{KS_CSI, "SI"}, {KS_CEI, "EI"},
			{KS_U7, "u7"}, {KS_RFG, "RF"}, {KS_RBG, "RB"},
			{KS_8F, "8f"}, {KS_8B, "8b"}, {KS_8U, "8u"},
			{KS_CBE, "BE"}, {KS_CBD, "BD"},
			{KS_CST, "ST"}, {KS_CRT, "RT"},
			{KS_SSI, "Si"}, {KS_SRI, "Ri"},
			{KS_CF, "CF"},
			{(enum SpecialKey)0, NULL}
		    };
    int		    i;
    static char_u   tstrbuf[TBUFSZ];
    char_u	    *tp = tstrbuf;

    /*
     * get output strings
     */
    for (i = 0; string_names[i].name != NULL; ++i)
    {
	if (term_strings_not_set(string_names[i].dest))
	{
	    TERM_STR(string_names[i].dest) = TGETSTR(string_names[i].name, &tp);
#ifdef FEAT_EVAL
	    set_term_option_sctx_idx(string_names[i].name, -1);
#endif
	}
    }

    // tgetflag() returns 1 if the flag is present, 0 if not and
    // possibly -1 if the flag doesn't exist.
    if ((T_MS == NULL || T_MS == empty_option) && tgetflag("ms") > 0)
	T_MS = (char_u *)"y";
    if ((T_XS == NULL || T_XS == empty_option) && tgetflag("xs") > 0)
	T_XS = (char_u *)"y";
    if ((T_XN == NULL || T_XN == empty_option) && tgetflag("xn") > 0)
	T_XN = (char_u *)"y";
    if ((T_DB == NULL || T_DB == empty_option) && tgetflag("db") > 0)
	T_DB = (char_u *)"y";
    if ((T_DA == NULL || T_DA == empty_option) && tgetflag("da") > 0)
	T_DA = (char_u *)"y";
    if ((T_UT == NULL || T_UT == empty_option) && tgetflag("ut") > 0)
	T_UT = (char_u *)"y";

    /*
     * get key codes
     */
    for (i = 0; key_names[i] != NULL; ++i)
	if (find_termcode((char_u *)key_names[i]) == NULL)
	{
	    char_u *p = TGETSTR(key_names[i], &tp);

	    // if cursor-left == backspace, ignore it (televideo 925)
	    if (p != NULL
		    && (*p != Ctrl_H
			|| key_names[i][0] != 'k'
			|| key_names[i][1] != 'l'))
		add_termcode((char_u *)key_names[i], p, FALSE);
	}

    if (*height == 0)
	*height = tgetnum("li");
    if (*width == 0)
	*width = tgetnum("co");

    /*
     * Get number of colors (if not done already).
     */
    if (term_strings_not_set(KS_CCO))
    {
	set_color_count(tgetnum("Co"));
#ifdef FEAT_EVAL
	set_term_option_sctx_idx("Co", -1);
#endif
    }

# ifndef hpux
    BC = (char *)TGETSTR("bc", &tp);
    UP = (char *)TGETSTR("up", &tp);
    char_u *p = TGETSTR("pc", &tp);
    if (p != NULL)
	PC = *p;
# endif
}
#endif

/*
 * Report "term" is not found and list the ones we do know about.
 */
    static void
report_term_error(char *error_msg, char_u *term)
{
    mch_errmsg("\r\n");
    if (error_msg != NULL)
    {
	mch_errmsg(error_msg);
	mch_errmsg("\r\n");
    }
    mch_errmsg("'");
    mch_errmsg((char *)term);
    mch_errmsg(_("' not known. Available builtin terminals are:"));
    mch_errmsg("\r\n");

    for (int i = 0; ; ++i)
    {
	char *name = builtin_terminals[i].bitc_name;
	if (name == NULL)  // end marker
	    break;
	// Do not mention the "gui" entry, the user won't need to type it.
	if (STRCMP(name, "gui") != 0)
	{
#ifdef HAVE_TGETENT
	    mch_errmsg("    builtin_");
#else
	    mch_errmsg("    ");
#endif
	    mch_errmsg(name);
	    mch_errmsg("\r\n");
	}
    }
    // Output extra 'cmdheight' line breaks to avoid that the following error
    // message overwrites the last terminal name.
    for (int i = 1; i < p_ch; ++i)
	mch_errmsg("\r\n");
}

    static void
report_default_term(char_u *term)
{
    mch_errmsg(_("defaulting to '"));
    mch_errmsg((char *)term);
    mch_errmsg("'\r\n");
    if (emsg_silent == 0 && !in_assert_fails)
    {
	screen_start();	// don't know where cursor is now
	out_flush();
	if (!is_not_a_term())
	    ui_delay(2007L, TRUE);
    }
}

/*
 * Parse the 'keyprotocol' option, match against "term" and return the protocol
 * for the first matching entry.
 * When "term" is NULL then compile all patterns to check for any errors.
 * Returns KEYPROTOCOL_FAIL if a pattern cannot be compiled.
 * Returns KEYPROTOCOL_NONE if there is no match.
 */
    keyprot_T
match_keyprotocol(char_u *term)
{
    int		len = (int)STRLEN(p_kpc) + 1;
    char_u	*buf = alloc(len);
    if (buf == NULL)
	return KEYPROTOCOL_FAIL;

    keyprot_T	ret = KEYPROTOCOL_FAIL;
    char_u	*p = p_kpc;
    while (*p != NUL)
    {
	// Isolate one comma separated item.
	(void)copy_option_part(&p, buf, len, ",");
	char_u *colon = vim_strchr(buf, ':');
	if (colon == NULL || colon == buf || colon[1] == NUL)
	    goto theend;
	*colon = NUL;

	keyprot_T prot;
	// Note: Keep this in sync with p_kpc_protocol_values.
	if (STRCMP(colon + 1, "none") == 0)
	    prot = KEYPROTOCOL_NONE;
	else if (STRCMP(colon + 1, "mok2") == 0)
	    prot = KEYPROTOCOL_MOK2;
	else if (STRCMP(colon + 1, "kitty") == 0)
	    prot = KEYPROTOCOL_KITTY;
	else
	    goto theend;

	regmatch_T regmatch;
	CLEAR_FIELD(regmatch);
	regmatch.rm_ic = TRUE;
	regmatch.regprog = vim_regcomp(buf, RE_MAGIC);
	if (regmatch.regprog == NULL)
	    goto theend;

	int match = term != NULL && vim_regexec(&regmatch, term, (colnr_T)0);
	vim_regfree(regmatch.regprog);
	if (match)
	{
	    ret = prot;
	    goto theend;
	}

    }

    // No match found, use "none".
    ret = KEYPROTOCOL_NONE;

theend:
    vim_free(buf);
    return ret;
}

/*
 * Set terminal options for terminal "term".
 * Return OK if terminal 'term' was found in a termcap, FAIL otherwise.
 *
 * While doing this, until ttest(), some options may be NULL, be careful.
 */
    int
set_termname(char_u *term)
{
#ifdef HAVE_TGETENT
    int		builtin_first = p_tbi;
    int		try;
    int		termcap_cleared = FALSE;
#endif
    int		width = 0, height = 0;
    char	*error_msg = NULL;
    char_u	*bs_p, *del_p;

    // In silect mode (ex -s) we don't use the 'term' option.
    if (silent_mode)
	return OK;

    detected_8bit = FALSE;		// reset 8-bit detection

    if (term_is_builtin(term))
    {
	term += 8;
#ifdef HAVE_TGETENT
	builtin_first = 1;
#endif
    }

/*
 * If HAVE_TGETENT is not defined, only the builtin termcap is used, otherwise:
 *   If builtin_first is TRUE:
 *     0. try builtin termcap
 *     1. try external termcap
 *     2. if both fail default to a builtin terminal
 *   If builtin_first is FALSE:
 *     1. try external termcap
 *     2. try builtin termcap, if both fail default to a builtin terminal
 */
#ifdef HAVE_TGETENT
    for (try = builtin_first ? 0 : 1; try < 3; ++try)
    {
	/*
	 * Use external termcap
	 */
	if (try == 1)
	{
	    char_u	    tbuf[TBUFSZ];

	    /*
	     * If the external termcap does not have a matching entry, try the
	     * builtin ones.
	     */
	    if ((error_msg = invoke_tgetent(tbuf, term)) == NULL)
	    {
		if (!termcap_cleared)
		{
		    clear_termoptions();	// clear old options
		    termcap_cleared = TRUE;
		}

		get_term_entries(&height, &width);
	    }
	}
	else	    // try == 0 || try == 2
#endif // HAVE_TGETENT
	/*
	 * Use builtin termcap
	 */
	{
#ifdef HAVE_TGETENT
	    /*
	     * If builtin termcap was already used, there is no need to search
	     * for the builtin termcap again, quit now.
	     */
	    if (try == 2 && builtin_first && termcap_cleared)
		break;
#endif
	    /*
	     * Search for 'term' in builtin_terminals[].
	     */
	    tcap_entry_T *termp = find_builtin_term(term);
	    if (termp == NULL)	// did not find it
	    {
#ifdef HAVE_TGETENT
		/*
		 * If try == 0, first try the external termcap. If that is not
		 * found we'll get back here with try == 2.
		 * If termcap_cleared is set we used the external termcap,
		 * don't complain about not finding the term in the builtin
		 * termcap.
		 */
		if (try == 0)			// try external one
		    continue;
		if (termcap_cleared)		// found in external termcap
		    break;
#endif
		report_term_error(error_msg, term);

		// when user typed :set term=xxx, quit here
		if (starting != NO_SCREEN)
		{
		    screen_start();	// don't know where cursor is now
		    wait_return(TRUE);
		    return FAIL;
		}
		term = DEFAULT_TERM;
		report_default_term(term);
		set_string_option_direct((char_u *)"term", -1, term,
								 OPT_FREE, 0);
		display_errors();
	    }
	    out_flush();
#ifdef HAVE_TGETENT
	    if (!termcap_cleared)
	    {
#endif
		clear_termoptions();	    // clear old options
#ifdef HAVE_TGETENT
		termcap_cleared = TRUE;
	    }
#endif
	    parse_builtin_tcap(term);

#ifdef FEAT_GUI
	    if (term_is_gui(term))
	    {
		out_flush();
		gui_init();
		// If starting the GUI failed, don't do any of the other
		// things for this terminal
		if (!gui.in_use)
		    return FAIL;
# ifdef HAVE_TGETENT
		break;		// don't try using external termcap
# endif
	    }
#endif // FEAT_GUI
	}
#ifdef HAVE_TGETENT
    }
#endif

#ifdef FEAT_GUI
    if (!gui.in_use)
#endif
    {
	// Use the 'keyprotocol' option to adjust the t_TE and t_TI
	// termcap entries if there is an entry matching "term".
	keyprot_T kpc = match_keyprotocol(term);
	apply_keyprotocol(term, kpc);

#ifdef FEAT_TERMGUICOLORS
	// There is no good way to detect that the terminal supports RGB
	// colors.  Since these termcap entries are non-standard anyway and
	// only used when the user sets 'termguicolors' we might as well add
	// them.  But not when one of them was already set.
	if (term_strings_not_set(KS_8F)
		&& term_strings_not_set(KS_8B)
		&& term_strings_not_set(KS_8U))
	    apply_builtin_tcap(term, builtin_rgb, TRUE);
#endif
#ifdef HAVE_TGETENT
	if (term_strings_not_set(KS_CF))
	    apply_builtin_tcap(term, special_term, TRUE);
#endif
    }

/*
 * special: There is no info in the termcap about whether the cursor
 * positioning is relative to the start of the screen or to the start of the
 * scrolling region.  We just guess here. Only msdos pcterm is known to do it
 * relative.
 */
    if (STRCMP(term, "pcterm") == 0)
	T_CCS = (char_u *)"yes";
    else
	T_CCS = empty_option;

    // Special case: "kitty" may not have a "RV" entry in terminfo, but we need
    // to request the version for several other things to work.
    if (strstr((char *)term, "kitty") != NULL
					   && (T_CRV == NULL || *T_CRV == NUL))
	T_CRV = (char_u *)"\033[>c";

#ifdef UNIX
/*
 * Any "stty" settings override the default for t_kb from the termcap.
 * This is in os_unix.c, because it depends a lot on the version of unix that
 * is being used.
 * Don't do this when the GUI is active, it uses "t_kb" and "t_kD" directly.
 */
# ifdef FEAT_GUI
    if (!gui.in_use)
# endif
	get_stty();
#endif

/*
 * If the termcap has no entry for 'bs' and/or 'del' and the ioctl() also
 * didn't work, use the default CTRL-H
 * The default for t_kD is DEL, unless t_kb is DEL.
 * The vim_strsave'd strings are probably lost forever, well it's only two
 * bytes.  Don't do this when the GUI is active, it uses "t_kb" and "t_kD"
 * directly.
 */
#ifdef FEAT_GUI
    if (!gui.in_use)
#endif
    {
	bs_p = find_termcode((char_u *)"kb");
	del_p = find_termcode((char_u *)"kD");
	if (bs_p == NULL || *bs_p == NUL)
	    add_termcode((char_u *)"kb", (bs_p = (char_u *)CTRL_H_STR), FALSE);
	if ((del_p == NULL || *del_p == NUL) &&
					    (bs_p == NULL || *bs_p != DEL))
	    add_termcode((char_u *)"kD", (char_u *)DEL_STR, FALSE);
    }

#if defined(UNIX) || defined(VMS)
    term_is_xterm = vim_is_xterm(term);
#endif
#ifdef FEAT_TERMRESPONSE
    // Reset terminal properties that are set based on the termresponse, which
    // will be sent out soon.
    init_term_props(FALSE);
#endif

#if defined(UNIX) || defined(VMS)
    // If the first number in t_XM is 1006 then the terminal will support SGR
    // mouse reporting.
    int did_set_ttym = FALSE;
    if (T_CXM != NULL && *T_CXM != NUL && !option_was_set((char_u *)"ttym"))
    {
	char_u *p = T_CXM;

	while (*p != NUL && !VIM_ISDIGIT(*p))
	    ++p;
	if (getdigits(&p) == 1006)
	{
	    did_set_ttym = TRUE;
	    set_option_value_give_err((char_u *)"ttym", 0L, (char_u *)"sgr", 0);
	}
    }

    /*
     * For Unix, set the 'ttymouse' option to the type of mouse to be used.
     * The termcode for the mouse is added as a side effect in option.c.
     */
    {
	char_u	*p = (char_u *)"";

# ifdef FEAT_MOUSE_XTERM
	if (use_xterm_like_mouse(term))
	{
	    if (use_xterm_mouse())
		p = NULL;	// keep existing value, might be "xterm2"
	    else
		p = (char_u *)"xterm";
	}
# endif
	if (p != NULL && !did_set_ttym)
	{
	    set_option_value_give_err((char_u *)"ttym", 0L, p, 0);
	    // Reset the WAS_SET flag, 'ttymouse' can be set to "sgr" or
	    // "xterm2" in check_termcode().
	    reset_option_was_set((char_u *)"ttym");
	}
	if (p == NULL
#  ifdef FEAT_GUI
		|| gui.in_use
#  endif
		)
	    check_mouse_termcode();	// set mouse termcode anyway
    }
#else
    set_mouse_termcode(KS_MOUSE, (char_u *)"\233M");
#endif

#ifdef FEAT_MOUSE_XTERM
    // Focus reporting is supported by xterm compatible terminals and tmux.
    // We hard-code the received escape sequences here.  There are the terminfo
    // entries kxIN and kxOUT, but they are rarely used and do hot have a
    // two-letter termcap name.
    // This used to be done only for xterm-like terminals, but some others also
    // may produce these codes.  Always recognize them, as the chance of them
    // being used for something else is very small.
    {
	char_u name[3];

	// handle focus in event
	name[0] = KS_EXTRA;
	name[1] = KE_FOCUSGAINED;
	name[2] = NUL;
	add_termcode(name, (char_u *)"\033[I", FALSE);

	// handle focus out event
	name[1] = KE_FOCUSLOST;
	add_termcode(name, (char_u *)"\033[O", FALSE);

	need_gather = TRUE;
    }
#endif
#if (defined(UNIX) || defined(VMS))
    // First time after setting 'term' a focus event is always reported.
    focus_state = MAYBE;
#endif

#ifdef USE_TERM_CONSOLE
    // DEFAULT_TERM indicates that it is the machine console.
    if (STRCMP(term, DEFAULT_TERM) != 0)
	term_console = FALSE;
    else
    {
	term_console = TRUE;
# ifdef AMIGA
	win_resize_on();	// enable window resizing reports
# endif
    }
#endif

    ttest(TRUE);	// make sure we have a valid set of terminal codes

    full_screen = TRUE;		// we can use termcap codes from now on
    set_term_defaults();	// use current values as defaults
#ifdef FEAT_TERMRESPONSE
    LOG_TR(("setting crv_status to STATUS_GET"));
    crv_status.tr_progress = STATUS_GET;	// Get terminal version later
    write_t_8u_state = FALSE;
#endif

    /*
     * Initialize the terminal with the appropriate termcap codes.
     * Set the mouse and window title if possible.
     * Don't do this when starting, need to parse the .vimrc first, because it
     * may redefine t_TI etc.
     */
    if (starting != NO_SCREEN)
    {
	starttermcap();		// may change terminal mode
	setmouse();		// may start using the mouse
	maketitle();		// may display window title
    }

	// display initial screen after ttest() checking. jw.
    if (width <= 0 || height <= 0)
    {
	// termcap failed to report size
	// set defaults, in case ui_get_shellsize() also fails
	width = 80;
#if defined(MSWIN)
	height = 25;	    // console is often 25 lines
#else
	height = 24;	    // most terminals are 24 lines
#endif
    }
    set_shellsize(width, height, FALSE);	// may change Rows
    if (starting != NO_SCREEN)
    {
	if (scroll_region)
	    scroll_region_reset();		// In case Rows changed
	check_map_keycodes();	// check mappings for terminal codes used

	{
	    buf_T	*buf;
	    aco_save_T	aco;

	    /*
	     * Execute the TermChanged autocommands for each buffer that is
	     * loaded.
	     */
	    FOR_ALL_BUFFERS(buf)
	    {
		if (curbuf->b_ml.ml_mfp != NULL)
		{
		    aucmd_prepbuf(&aco, buf);
		    if (curbuf == buf)
		    {
			apply_autocmds(EVENT_TERMCHANGED, NULL, NULL, FALSE,
								      curbuf);
			// restore curwin/curbuf and a few other things
			aucmd_restbuf(&aco);
		    }
		}
	    }
	}
    }

#ifdef FEAT_TERMRESPONSE
    may_req_termresponse();
#endif

    return OK;
}

#if defined(EXITFREE) || defined(PROTO)

# ifdef HAVE_DEL_CURTERM
#  include <term.h>	    // declares cur_term
# endif

/*
 * If supported, delete "cur_term", which caches terminal related entries.
 * Avoids that valgrind reports possibly lost memory.
 */
    void
free_cur_term(void)
{
# ifdef HAVE_DEL_CURTERM
    if (cur_term)
	del_curterm(cur_term);
# endif
}

#endif

#ifdef HAVE_TGETENT
/*
 * Call tgetent()
 * Return error message if it fails, NULL if it's OK.
 */
    static char *
invoke_tgetent(char_u *tbuf, char_u *term)
{
    int	    i;

    // Note: Valgrind may report a leak here, because the library keeps one
    // buffer around that we can't ever free.
    i = TGETENT(tbuf, term);
    if (i < 0		    // -1 is always an error
# ifdef TGETENT_ZERO_ERR
	    || i == 0	    // sometimes zero is also an error
# endif
       )
    {
	// On FreeBSD tputs() gets a SEGV after a tgetent() which fails.  Call
	// tgetent() with the always existing "dumb" entry to avoid a crash or
	// hang.
	(void)TGETENT(tbuf, "dumb");

	if (i < 0)
# ifdef TGETENT_ZERO_ERR
	    return _(e_cannot_open_termcap_file);
	if (i == 0)
# endif
#ifdef TERMINFO
	    return _(e_terminal_entry_not_found_in_terminfo);
#else
	    return _(e_terminal_entry_not_found_in_termcap);
#endif
    }
    return NULL;
}

/*
 * Some versions of tgetstr() have been reported to return -1 instead of NULL.
 * Fix that here.
 */
    static char_u *
vim_tgetstr(char *s, char_u **pp)
{
    char	*p;

    p = tgetstr(s, (char **)pp);
    if (p == (char *)-1)
	p = NULL;
    return (char_u *)p;
}
#endif // HAVE_TGETENT

#if defined(HAVE_TGETENT) && (defined(UNIX) || defined(VMS) || defined(MACOS_X))
/*
 * Get Columns and Rows from the termcap. Used after a window signal if the
 * ioctl() fails. It doesn't make sense to call tgetent each time if the "co"
 * and "li" entries never change. But on some systems this works.
 * Errors while getting the entries are ignored.
 */
    void
getlinecol(
    long	*cp,	// pointer to columns
    long	*rp)	// pointer to rows
{
    char_u	tbuf[TBUFSZ];

    if (T_NAME == NULL || *T_NAME == NUL
	    || invoke_tgetent(tbuf, T_NAME) != NULL)
	return;

    if (*cp == 0)
	*cp = tgetnum("co");
    if (*rp == 0)
	*rp = tgetnum("li");
}
#endif // defined(HAVE_TGETENT) && defined(UNIX)

/*
 * Get a string entry from the termcap and add it to the list of termcodes.
 * Used for <t_xx> special keys.
 * Give an error message for failure when not sourcing.
 * If force given, replace an existing entry.
 * Return FAIL if the entry was not found, OK if the entry was added.
 */
    int
add_termcap_entry(char_u *name, int force)
{
    char_u  *term;
    int	    key;
#ifdef HAVE_TGETENT
    char_u  *string;
    int	    i;
    int	    builtin_first;
    char_u  tbuf[TBUFSZ];
    char_u  tstrbuf[TBUFSZ];
    char_u  *tp = tstrbuf;
    char    *error_msg = NULL;
#endif

/*
 * If the GUI is running or will start in a moment, we only support the keys
 * that the GUI can produce.
 */
#ifdef FEAT_GUI
    if (gui.in_use || gui.starting)
	return gui_mch_haskey(name);
#endif

    if (!force && find_termcode(name) != NULL)	    // it's already there
	return OK;

    term = T_NAME;
    if (term == NULL || *term == NUL)	    // 'term' not defined yet
	return FAIL;

    if (term_is_builtin(term))		    // name starts with "builtin_"
    {
	term += 8;
#ifdef HAVE_TGETENT
	builtin_first = TRUE;
#endif
    }
#ifdef HAVE_TGETENT
    else
	builtin_first = p_tbi;
#endif

#ifdef HAVE_TGETENT
/*
 * We can get the entry from the builtin termcap and from the external one.
 * If 'ttybuiltin' is on or the terminal name starts with "builtin_", try
 * builtin termcap first.
 * If 'ttybuiltin' is off, try external termcap first.
 */
    for (i = 0; i < 2; ++i)
    {
	if ((!builtin_first) == i)
#endif
	/*
	 * Search in builtin termcaps
	 */
	{
	    tcap_entry_T *termp = find_builtin_term(term);
	    if (termp != NULL)	// found it
	    {
		key = TERMCAP2KEY(name[0], name[1]);
		++termp;
		while (termp->bt_entry != (int)KS_NAME)
		{
		    if ((int)termp->bt_entry == key)
		    {
			add_termcode(name, (char_u *)termp->bt_string,
							  term_is_8bit(term));
			return OK;
		    }
		    ++termp;
		}
	    }
	}
#ifdef HAVE_TGETENT
	else
	/*
	 * Search in external termcap
	 */
	{
	    error_msg = invoke_tgetent(tbuf, term);
	    if (error_msg == NULL)
	    {
		string = TGETSTR((char *)name, &tp);
		if (string != NULL && *string != NUL)
		{
		    add_termcode(name, string, FALSE);
		    return OK;
		}
	    }
	}
    }
#endif

    if (SOURCING_NAME == NULL)
    {
#ifdef HAVE_TGETENT
	if (error_msg != NULL)
	    emsg(error_msg);
	else
#endif
	    semsg(_(e_no_str_entry_in_termcap), name);
    }
    return FAIL;
}

    static int
term_is_builtin(char_u *name)
{
    return (STRNCMP(name, "builtin_", (size_t)8) == 0);
}

/*
 * Return TRUE if terminal "name" uses CSI instead of <Esc>[.
 * Assume that the terminal is using 8-bit controls when the name contains
 * "8bit", like in "xterm-8bit".
 */
    int
term_is_8bit(char_u *name)
{
    return (detected_8bit || strstr((char *)name, "8bit") != NULL);
}

/*
 * Translate terminal control chars from 7-bit to 8-bit:
 * <Esc>[ -> CSI  <M_C_[>
 * <Esc>] -> OSC  <M-C-]>
 * <Esc>O -> <M-C-O>
 */
    static int
term_7to8bit(char_u *p)
{
    if (*p != ESC)
	return 0;

    if (p[1] == '[')
	return CSI;
    else if (p[1] == ']')
	return OSC;
    else if (p[1] == 'O')
	return 0x8f;
    return 0;
}

#if defined(FEAT_GUI) || defined(PROTO)
    int
term_is_gui(char_u *name)
{
    return (STRCMP(name, "builtin_gui") == 0 || STRCMP(name, "gui") == 0);
}
#endif

#if !defined(HAVE_TGETENT) || defined(AMIGA) || defined(PROTO)

    char_u *
tltoa(unsigned long i)
{
    static char_u buf[16];
    char_u	*p;

    p = buf + 15;
    *p = '\0';
    do
    {
	--p;
	*p = (char_u) (i % 10 + '0');
	i /= 10;
    }
    while (i > 0 && p > buf);
    return p;
}
#endif

#ifndef HAVE_TGETENT

/*
 * minimal tgoto() implementation.
 * no padding and we only parse for %i %d and %+char
 */
    static char *
tgoto(char *cm, int x, int y)
{
    static char buf[30];
    char *p, *s, *e;

    if (!cm)
	return "OOPS";
    e = buf + 29;
    for (s = buf; s < e && *cm; cm++)
    {
	if (*cm != '%')
	{
	    *s++ = *cm;
	    continue;
	}
	switch (*++cm)
	{
	case 'd':
	    p = (char *)tltoa((unsigned long)y);
	    y = x;
	    while (*p)
		*s++ = *p++;
	    break;
	case 'i':
	    x++;
	    y++;
	    break;
	case '+':
	    *s++ = (char)(*++cm + y);
	    y = x;
	    break;
	case '%':
	    *s++ = *cm;
	    break;
	default:
	    return "OOPS";
	}
    }
    *s = '\0';
    return buf;
}

#endif // HAVE_TGETENT

/*
 * Set the terminal name and initialize the terminal options.
 * If "name" is NULL or empty, get the terminal name from the environment.
 * If that fails, use the default terminal name.
 */
    void
termcapinit(char_u *name)
{
    char_u	*term = name;

    if (term != NULL && *term == NUL)
	term = NULL;	    // empty name is equal to no name

#ifndef MSWIN
    if (term == NULL)
	term = mch_getenv((char_u *)"TERM");
#endif
    if (term == NULL || *term == NUL)
	term = DEFAULT_TERM;
    set_string_option_direct((char_u *)"term", -1, term, OPT_FREE, 0);

    // Set the default terminal name.
    set_string_default("term", term);
    set_string_default("ttytype", term);

    // Avoid using "term" here, because the next mch_getenv() may overwrite it.
    set_termname(T_NAME != NULL ? T_NAME : term);
}

/*
 * The number of calls to ui_write is reduced by using "out_buf".
 */
#define OUT_SIZE	2047

// add one to allow mch_write() in os_win32.c to append a NUL
static char_u		out_buf[OUT_SIZE + 1];

static int		out_pos = 0;	// number of chars in out_buf

// Since the maximum number of SGR parameters shown as a normal value range is
// 16, the escape sequence length can be 4 * 16 + lead + tail.
#define MAX_ESC_SEQ_LEN	80

/*
 * out_flush(): flush the output buffer
 */
    void
out_flush(void)
{
    int	    len;

    if (out_pos == 0)
	return;

    // set out_pos to 0 before ui_write, to avoid recursiveness
    len = out_pos;
    out_pos = 0;
    ui_write(out_buf, len, FALSE);
#ifdef FEAT_EVAL
    if (ch_log_output != FALSE)
    {
	out_buf[len] = NUL;
	ch_log(NULL, "raw %s output: \"%s\"",
# ifdef FEAT_GUI
		(gui.in_use && !gui.dying && !gui.starting) ? "GUI" :
# endif
		"terminal",
		out_buf);
	if (ch_log_output == TRUE)
	    ch_log_output = FALSE;  // only log once
    }
#endif
}

/*
 * out_flush_cursor(): flush the output buffer and redraw the cursor.
 * Does not flush recursively in the GUI to avoid slow drawing.
 */
    void
out_flush_cursor(
    int	    force UNUSED,   // when TRUE, update cursor even when not moved
    int	    clear_selection UNUSED) // clear selection under cursor
{
    mch_disable_flush();
    out_flush();
    mch_enable_flush();
#ifdef FEAT_GUI
    if (gui.in_use)
    {
	gui_update_cursor(force, clear_selection);
	gui_may_flush();
    }
#endif
}


/*
 * Sometimes a byte out of a multi-byte character is written with out_char().
 * To avoid flushing half of the character, call this function first.
 */
    void
out_flush_check(void)
{
    if (enc_dbcs != 0 && out_pos >= OUT_SIZE - MB_MAXBYTES)
	out_flush();
}

#ifdef FEAT_GUI
/*
 * out_trash(): Throw away the contents of the output buffer
 */
    void
out_trash(void)
{
    out_pos = 0;
}
#endif

/*
 * out_char(c): put a byte into the output buffer.
 *		Flush it if it becomes full.
 * This should not be used for outputting text on the screen (use functions
 * like msg_puts() and screen_putchar() for that).
 */
    void
out_char(unsigned c)
{
#if defined(UNIX) || defined(VMS) || defined(AMIGA) || defined(MACOS_X)
    if (c == '\n')	// turn LF into CR-LF (CRMOD doesn't seem to do this)
	out_char('\r');
#endif

    out_buf[out_pos++] = c;

    // For testing we flush each time.
    if (out_pos >= OUT_SIZE || p_wd)
	out_flush();
}

/*
 * Output "c" like out_char(), but don't flush when p_wd is set.
 */
    static int
out_char_nf(int c)
{
    out_buf[out_pos++] = (unsigned)c;

    if (out_pos >= OUT_SIZE)
	out_flush();
    return (unsigned)c;
}

/*
 * A never-padding out_str().
 * Use this whenever you don't want to run the string through tputs().
 * tputs() above is harmless, but tputs() from the termcap library
 * is likely to strip off leading digits, that it mistakes for padding
 * information, and "%i", "%d", etc.
 * This should only be used for writing terminal codes, not for outputting
 * normal text (use functions like msg_puts() and screen_putchar() for that).
 */
    void
out_str_nf(char_u *s)
{
    // avoid terminal strings being split up
    if (out_pos > OUT_SIZE - MAX_ESC_SEQ_LEN)
	out_flush();

    for (char_u *p = s; *p != NUL; ++p)
	out_char_nf(*p);

    // For testing we write one string at a time.
    if (p_wd)
	out_flush();
}

/*
 * A conditional-flushing out_str, mainly for visualbell.
 * Handles a delay internally, because termlib may not respect the delay or do
 * it at the wrong time.
 * Note: Only for terminal strings.
 */
    void
out_str_cf(char_u *s)
{
    if (s == NULL || *s == NUL)
	return;

#ifdef HAVE_TGETENT
    char_u *p;
#endif

#ifdef FEAT_GUI
    // Don't use tputs() when GUI is used, ncurses crashes.
    if (gui.in_use)
    {
	out_str_nf(s);
	return;
    }
#endif
    if (out_pos > OUT_SIZE - MAX_ESC_SEQ_LEN)
	out_flush();
#ifdef HAVE_TGETENT
    for (p = s; *s; ++s)
    {
	// flush just before delay command
	if (*s == '$' && *(s + 1) == '<')
	{
	    char_u save_c = *s;
	    int duration = atoi((char *)s + 2);

	    *s = NUL;
	    tputs((char *)p, 1, TPUTSFUNCAST out_char_nf);
	    *s = save_c;
	    out_flush();
# ifdef ELAPSED_FUNC
	    // Only sleep here if we can limit this happening in
	    // vim_beep().
	    p = vim_strchr(s, '>');
	    if (p == NULL || duration <= 0)
	    {
		// can't parse the time, don't sleep here
		p = s;
	    }
	    else
	    {
		++p;
		do_sleep(duration, FALSE);
	    }
# else
	    // Rely on the terminal library to sleep.
	    p = s;
# endif
	    break;
	}
    }
    tputs((char *)p, 1, TPUTSFUNCAST out_char_nf);
#else
    while (*s)
	out_char_nf(*s++);
#endif

    // For testing we write one string at a time.
    if (p_wd)
	out_flush();
}

/*
 * out_str(s): Put a character string a byte at a time into the output buffer.
 * If HAVE_TGETENT is defined use tputs(), the termcap parser. (jw)
 * This should only be used for writing terminal codes, not for outputting
 * normal text (use functions like msg_puts() and screen_putchar() for that).
 */
    void
out_str(char_u *s)
{
    if (s == NULL || *s == NUL)
	return;

#ifdef FEAT_GUI
    // Don't use tputs() when GUI is used, ncurses crashes.
    if (gui.in_use)
    {
	out_str_nf(s);
	return;
    }
#endif
    // avoid terminal strings being split up
    if (out_pos > OUT_SIZE - MAX_ESC_SEQ_LEN)
	out_flush();
#ifdef HAVE_TGETENT
    tputs((char *)s, 1, TPUTSFUNCAST out_char_nf);
#else
    while (*s)
	out_char_nf(*s++);
#endif

    // For testing we write one string at a time.
    if (p_wd)
	out_flush();
}

/*
 * cursor positioning using termcap parser. (jw)
 */
    void
term_windgoto(int row, int col)
{
    OUT_STR(tgoto((char *)T_CM, col, row));
}

    void
term_cursor_right(int i)
{
    OUT_STR(tgoto((char *)T_CRI, 0, i));
}

    void
term_append_lines(int line_count)
{
    OUT_STR(tgoto((char *)T_CAL, 0, line_count));
}

    void
term_delete_lines(int line_count)
{
    OUT_STR(tgoto((char *)T_CDL, 0, line_count));
}

#if defined(UNIX) || defined(VMS) || defined(PROTO)
    void
term_enable_mouse(int enable)
{
    int on = enable ? 1 : 0;
    OUT_STR(tgoto((char *)T_CXM, 0, on));
}
#endif

#if defined(HAVE_TGETENT) || defined(PROTO)
    void
term_set_winpos(int x, int y)
{
    // Can't handle a negative value here
    if (x < 0)
	x = 0;
    if (y < 0)
	y = 0;
    OUT_STR(tgoto((char *)T_CWP, y, x));
}

# if defined(FEAT_TERMRESPONSE) || defined(PROTO)
/*
 * Return TRUE if we can request the terminal for a response.
 */
    static int
can_get_termresponse(void)
{
    return cur_tmode == TMODE_RAW
	    && termcap_active
#  ifdef UNIX
	    && (is_not_a_term() || (isatty(1) && isatty(read_cmd_fd)))
#  endif
	    && p_ek;
}

/*
 * Set "status" to STATUS_SENT.
 */
    static void
termrequest_sent(termrequest_T *status)
{
    status->tr_progress = STATUS_SENT;
    status->tr_start = time(NULL);
}

/*
 * Return TRUE if any of the requests are in STATUS_SENT.
 */
    static int
termrequest_any_pending(void)
{
    int	    i;
    time_t  now = time(NULL);

    for (i = 0; all_termrequests[i] != NULL; ++i)
    {
	if (all_termrequests[i]->tr_progress == STATUS_SENT)
	{
	    if (all_termrequests[i]->tr_start > 0 && now > 0
				    && all_termrequests[i]->tr_start + 2 < now)
		// Sent the request more than 2 seconds ago and didn't get a
		// response, assume it failed.
		all_termrequests[i]->tr_progress = STATUS_FAIL;
	    else
		return TRUE;
	}
    }
    return FALSE;
}

static int winpos_x = -1;
static int winpos_y = -1;
static int did_request_winpos = 0;

# if defined(FEAT_EVAL) || defined(FEAT_TERMINAL) || defined(PROTO)
/*
 * Try getting the Vim window position from the terminal.
 * Returns OK or FAIL.
 */
    int
term_get_winpos(int *x, int *y, varnumber_T timeout)
{
    int count = 0;
    int prev_winpos_x = winpos_x;
    int prev_winpos_y = winpos_y;

    if (*T_CGP == NUL || !can_get_termresponse())
	return FAIL;
    winpos_x = -1;
    winpos_y = -1;
    ++did_request_winpos;
    termrequest_sent(&winpos_status);
    OUT_STR(T_CGP);
    out_flush();

    // Try reading the result for "timeout" msec.
    while (count++ <= timeout / 10 && !got_int)
    {
	(void)vpeekc_nomap();
	if (winpos_x >= 0 && winpos_y >= 0)
	{
	    *x = winpos_x;
	    *y = winpos_y;
	    return OK;
	}
	ui_delay(10L, FALSE);
    }
    // Do not reset "did_request_winpos", if we timed out the response might
    // still come later and we must consume it.

    winpos_x = prev_winpos_x;
    winpos_y = prev_winpos_y;
    if (timeout < 10 && prev_winpos_y >= 0 && prev_winpos_x >= 0)
    {
	// Polling: return previous values if we have them.
	*x = winpos_x;
	*y = winpos_y;
	return OK;
    }

    return FALSE;
}
#  endif
# endif

    void
term_set_winsize(int height, int width)
{
    OUT_STR(tgoto((char *)T_CWS, width, height));
}
#endif

    void
term_font(int n)
{
    if (*T_CFO)
    {
	char buf[20];
	sprintf(buf, (char *)T_CFO, 9 + n);
	OUT_STR(buf);
    }
}

    static void
term_color(char_u *s, int n)
{
    char	buf[20];
    int		i = *s == CSI ? 1 : 2;
		// index in s[] just after <Esc>[ or CSI

    // Special handling of 16 colors, because termcap can't handle it
    // Also accept "\e[3%dm" for TERMINFO, it is sometimes used
    // Also accept CSI instead of <Esc>[
    if (n >= 8 && t_colors >= 16
	      && ((s[0] == ESC && s[1] == '[')
#if defined(FEAT_VTP) && defined(FEAT_TERMGUICOLORS)
		  || (s[0] == ESC && s[1] == '|')
#endif
		  || (s[0] == CSI && (i = 1) == 1))
	      && s[i] != NUL
	      && (STRCMP(s + i + 1, "%p1%dm") == 0
		  || STRCMP(s + i + 1, "%dm") == 0)
	      && (s[i] == '3' || s[i] == '4'))
    {
#ifdef TERMINFO
	char *format = "%s%s%%p1%%dm";
#else
	char *format = "%s%s%%dm";
#endif
	char *lead = i == 2 ? (
#if defined(FEAT_VTP) && defined(FEAT_TERMGUICOLORS)
		    s[1] == '|' ? "\033|" :
#endif
		    "\033[") : "\233";
	char *tail = s[i] == '3' ? (n >= 16 ? "38;5;" : "9")
				 : (n >= 16 ? "48;5;" : "10");

	sprintf(buf, format, lead, tail);
	OUT_STR(tgoto(buf, 0, n >= 16 ? n : n - 8));
    }
    else
	OUT_STR(tgoto((char *)s, 0, n));
}

    void
term_fg_color(int n)
{
    // Use "AF" termcap entry if present, "Sf" entry otherwise
    if (*T_CAF)
	term_color(T_CAF, n);
    else if (*T_CSF)
	term_color(T_CSF, n);
}

    void
term_bg_color(int n)
{
    // Use "AB" termcap entry if present, "Sb" entry otherwise
    if (*T_CAB)
	term_color(T_CAB, n);
    else if (*T_CSB)
	term_color(T_CSB, n);
}

    void
term_ul_color(int n)
{
    if (*T_CAU)
	term_color(T_CAU, n);
}

/*
 * Return "dark" or "light" depending on the kind of terminal.
 * This is just guessing!  Recognized are:
 * "linux"	    Linux console
 * "screen.linux"   Linux console with screen
 * "cygwin.*"	    Cygwin shell
 * "putty.*"	    Putty program
 * We also check the COLORFGBG environment variable, which is set by
 * rxvt and derivatives. This variable contains either two or three
 * values separated by semicolons; we want the last value in either
 * case. If this value is 0-6 or 8, our background is dark.
 */
    char_u *
term_bg_default(void)
{
#if defined(MSWIN)
    // DOS console is nearly always black
    return (char_u *)"dark";
#else
    char_u	*p;

    if (STRCMP(T_NAME, "linux") == 0
	    || STRCMP(T_NAME, "screen.linux") == 0
	    || STRNCMP(T_NAME, "cygwin", 6) == 0
	    || STRNCMP(T_NAME, "putty", 5) == 0
	    || ((p = mch_getenv((char_u *)"COLORFGBG")) != NULL
		&& (p = vim_strrchr(p, ';')) != NULL
		&& ((p[1] >= '0' && p[1] <= '6') || p[1] == '8')
		&& p[2] == NUL))
	return (char_u *)"dark";
    return (char_u *)"light";
#endif
}

#if defined(FEAT_TERMGUICOLORS) || defined(PROTO)

#define RED(rgb)   (((long_u)(rgb) >> 16) & 0xFF)
#define GREEN(rgb) (((long_u)(rgb) >>  8) & 0xFF)
#define BLUE(rgb)  (((long_u)(rgb)      ) & 0xFF)

    static void
term_rgb_color(char_u *s, guicolor_T rgb)
{
#define MAX_COLOR_STR_LEN 100
    char	buf[MAX_COLOR_STR_LEN];

    if (*s == NUL)
	return;
    vim_snprintf(buf, MAX_COLOR_STR_LEN,
				  (char *)s, RED(rgb), GREEN(rgb), BLUE(rgb));
#ifdef FEAT_VTP
    if (use_vtp() && (p_tgc || t_colors >= 256))
    {
	out_flush();
	buf[1] = '[';
	vtp_printf(buf);
    }
    else
#endif
	OUT_STR(buf);
}

    void
term_fg_rgb_color(guicolor_T rgb)
{
    if (rgb != INVALCOLOR)
	term_rgb_color(T_8F, rgb);
}

    void
term_bg_rgb_color(guicolor_T rgb)
{
    if (rgb != INVALCOLOR)
	term_rgb_color(T_8B, rgb);
}

    void
term_ul_rgb_color(guicolor_T rgb)
{
# ifdef FEAT_TERMRESPONSE
    // If the user explicitly sets t_8u then use it.  Otherwise wait for
    // termresponse to be received, which is when t_8u would be set and a
    // redraw is needed if it was used.
    if (!option_was_set((char_u *)"t_8u") && write_t_8u_state != OK)
	write_t_8u_state = MAYBE;
    else
# endif
	term_rgb_color(T_8U, rgb);
}
#endif

#if (defined(UNIX) || defined(VMS) || defined(MACOS_X)) || defined(PROTO)
/*
 * Generic function to set window title, using t_ts and t_fs.
 */
    void
term_settitle(char_u *title)
{
    MAY_WANT_TO_LOG_THIS;

    // t_ts takes one argument: column in status line
    OUT_STR(tgoto((char *)T_TS, 0, 0));	// set title start
    out_str_nf(title);
    out_str(T_FS);			// set title end
    out_flush();
}

/*
 * Tell the terminal to push (save) the title and/or icon, so that it can be
 * popped (restored) later.
 */
    void
term_push_title(int which)
{
    if ((which & SAVE_RESTORE_TITLE) && T_CST != NULL && *T_CST != NUL)
    {
	OUT_STR(T_CST);
	out_flush();
    }

    if ((which & SAVE_RESTORE_ICON) && T_SSI != NULL && *T_SSI != NUL)
    {
	OUT_STR(T_SSI);
	out_flush();
    }
}

/*
 * Tell the terminal to pop the title and/or icon.
 */
    void
term_pop_title(int which)
{
    if ((which & SAVE_RESTORE_TITLE) && T_CRT != NULL && *T_CRT != NUL)
    {
	OUT_STR(T_CRT);
	out_flush();
    }

    if ((which & SAVE_RESTORE_ICON) && T_SRI != NULL && *T_SRI != NUL)
    {
	OUT_STR(T_SRI);
	out_flush();
    }
}
#endif

/*
 * Make sure we have a valid set or terminal options.
 * Replace all entries that are NULL by empty_option
 */
    void
ttest(int pairs)
{
    char_u *env_colors;

    check_options();		    // make sure no options are NULL

    /*
     * MUST have "cm": cursor motion.
     */
    if (*T_CM == NUL)
	emsg(_(e_terminal_capability_cm_required));

    /*
     * if "cs" defined, use a scroll region, it's faster.
     */
    if (*T_CS != NUL)
	scroll_region = TRUE;
    else
	scroll_region = FALSE;

    if (pairs)
    {
	/*
	 * optional pairs
	 */
	// TP goes to normal mode for TI (invert) and TB (bold)
	if (*T_ME == NUL)
	    T_ME = T_MR = T_MD = T_MB = empty_option;
	if (*T_SO == NUL || *T_SE == NUL)
	    T_SO = T_SE = empty_option;
	if (*T_US == NUL || *T_UE == NUL)
	    T_US = T_UE = empty_option;
	if (*T_CZH == NUL || *T_CZR == NUL)
	    T_CZH = T_CZR = empty_option;

	// T_VE is needed even though T_VI is not defined
	if (*T_VE == NUL)
	    T_VI = empty_option;

	// if 'mr' or 'me' is not defined use 'so' and 'se'
	if (*T_ME == NUL)
	{
	    T_ME = T_SE;
	    T_MR = T_SO;
	    T_MD = T_SO;
	}

	// if 'so' or 'se' is not defined use 'mr' and 'me'
	if (*T_SO == NUL)
	{
	    T_SE = T_ME;
	    if (*T_MR == NUL)
		T_SO = T_MD;
	    else
		T_SO = T_MR;
	}

	// if 'ZH' or 'ZR' is not defined use 'mr' and 'me'
	if (*T_CZH == NUL)
	{
	    T_CZR = T_ME;
	    if (*T_MR == NUL)
		T_CZH = T_MD;
	    else
		T_CZH = T_MR;
	}

	// "Sb" and "Sf" come in pairs
	if (*T_CSB == NUL || *T_CSF == NUL)
	{
	    T_CSB = empty_option;
	    T_CSF = empty_option;
	}

	// "AB" and "AF" come in pairs
	if (*T_CAB == NUL || *T_CAF == NUL)
	{
	    T_CAB = empty_option;
	    T_CAF = empty_option;
	}

	// if 'Sb' and 'AB' are not defined, reset "Co"
	if (*T_CSB == NUL && *T_CAB == NUL)
	    free_one_termoption(T_CCO);

	// Set 'weirdinvert' according to value of 't_xs'
	p_wiv = (*T_XS != NUL);
    }
    need_gather = TRUE;

    // Set t_colors to the value of $COLORS or t_Co.  Ignore $COLORS in the
    // GUI.
    t_colors = atoi((char *)T_CCO);
#ifdef FEAT_GUI
    if (!gui.in_use)
#endif
    {
	env_colors = mch_getenv((char_u *)"COLORS");
	if (env_colors != NULL && SAFE_isdigit(*env_colors))
	{
	    int colors = atoi((char *)env_colors);

	    if (colors != t_colors)
		set_color_count(colors);
	}
    }
}

#if (defined(FEAT_GUI) && (defined(FEAT_MENU) || !defined(USE_ON_FLY_SCROLL))) \
	|| defined(PROTO)
/*
 * Represent the given long_u as individual bytes, with the most significant
 * byte first, and store them in dst.
 */
    void
add_long_to_buf(long_u val, char_u *dst)
{
    int	    i;
    int	    shift;

    for (i = 1; i <= (int)sizeof(long_u); i++)
    {
	shift = 8 * (sizeof(long_u) - i);
	dst[i - 1] = (char_u) ((val >> shift) & 0xff);
    }
}

/*
 * Interpret the next string of bytes in buf as a long integer, with the most
 * significant byte first.  Note that it is assumed that buf has been through
 * inchar(), so that NUL and K_SPECIAL will be represented as three bytes each.
 * Puts result in val, and returns the number of bytes read from buf
 * (between sizeof(long_u) and 2 * sizeof(long_u)), or -1 if not enough bytes
 * were present.
 */
    static int
get_long_from_buf(char_u *buf, long_u *val)
{
    int	    len;
    char_u  bytes[sizeof(long_u)];
    int	    i;
    int	    shift;

    *val = 0;
    len = get_bytes_from_buf(buf, bytes, (int)sizeof(long_u));
    if (len == -1)
	return -1;
    for (i = 0; i < (int)sizeof(long_u); i++)
    {
	shift = 8 * (sizeof(long_u) - 1 - i);
	*val += (long_u)bytes[i] << shift;
    }
    return len;
}
#endif

/*
 * Read the next num_bytes bytes from buf, and store them in bytes.  Assume
 * that buf has been through inchar().	Returns the actual number of bytes used
 * from buf (between num_bytes and num_bytes*2), or -1 if not enough bytes were
 * available.
 */
    int
get_bytes_from_buf(char_u *buf, char_u *bytes, int num_bytes)
{
    int	    len = 0;
    int	    i;
    char_u  c;

    for (i = 0; i < num_bytes; i++)
    {
	if ((c = buf[len++]) == NUL)
	    return -1;
	if (c == K_SPECIAL)
	{
	    if (buf[len] == NUL || buf[len + 1] == NUL)	    // cannot happen?
		return -1;
	    if (buf[len++] == (int)KS_ZERO)
		c = NUL;
	    // else it should be KS_SPECIAL; when followed by KE_FILLER c is
	    // K_SPECIAL, or followed by KE_CSI and c must be CSI.
	    if (buf[len++] == (int)KE_CSI)
		c = CSI;
	}
	else if (c == CSI && buf[len] == KS_EXTRA
					       && buf[len + 1] == (int)KE_CSI)
	    // CSI is stored as CSI KS_SPECIAL KE_CSI to avoid confusion with
	    // the start of a special key, see add_to_input_buf_csi().
	    len += 2;
	bytes[i] = c;
    }
    return len;
}

/*
 * Check if the new shell size is valid, correct it if it's too small or way
 * too big.
 */
    void
check_shellsize(void)
{
    if (Rows < min_rows())	// need room for one window and command line
	Rows = min_rows();
    limit_screen_size();

    // make sure these values are not invalid
    if (cmdline_row >= Rows)
	cmdline_row = Rows - 1;
    if (msg_row >= Rows)
	msg_row = Rows - 1;
}

/*
 * Limit Rows and Columns to avoid an overflow in Rows * Columns.
 */
    void
limit_screen_size(void)
{
    if (Columns < MIN_COLUMNS)
	Columns = MIN_COLUMNS;
    else if (Columns > 10000)
	Columns = 10000;
    if (Rows > 1000)
	Rows = 1000;
}

/*
 * Invoked just before the screen structures are going to be (re)allocated.
 */
    void
win_new_shellsize(void)
{
    static int	old_Rows = 0;
    static int	old_Columns = 0;

    if (old_Rows != Rows || old_Columns != Columns)
	ui_new_shellsize();
    if (old_Rows != Rows)
    {
	// If 'window' uses the whole screen, keep it using that.
	// Don't change it when set with "-w size" on the command line.
	if (p_window == old_Rows - 1
		    || (old_Rows == 0 && !option_was_set((char_u *)"window")))
	    p_window = Rows - 1;
	old_Rows = Rows;
	shell_new_rows();	// update window sizes
    }
    if (old_Columns != Columns)
    {
	old_Columns = Columns;
	shell_new_columns();	// update window sizes
    }
}

/*
 * Call this function when the Vim shell has been resized in any way.
 * Will obtain the current size and redraw (also when size didn't change).
 */
    void
shell_resized(void)
{
    set_shellsize(0, 0, FALSE);
}

/*
 * Check if the shell size changed.  Handle a resize.
 * When the size didn't change, nothing happens.
 */
    void
shell_resized_check(void)
{
    int		old_Rows = Rows;
    int		old_Columns = Columns;

    if (exiting
#ifdef FEAT_GUI
	    // Do not get the size when executing a shell command during
	    // startup.
	    || gui.starting
#endif
	    )
	return;

    (void)ui_get_shellsize();
    check_shellsize();
    if (old_Rows != Rows || old_Columns != Columns)
	shell_resized();
}

/*
 * Set size of the Vim shell.
 * If 'mustset' is TRUE, we must set Rows and Columns, do not get the real
 * window size (this is used for the :win command).
 * If 'mustset' is FALSE, we may try to get the real window size and if
 * it fails use 'width' and 'height'.
 */
    static void
set_shellsize_inner(int width, int height, int mustset)
{
    if (updating_screen)
	// resizing while in update_screen() may cause a crash
	return;

    // curwin->w_buffer can be NULL when we are closing a window and the
    // buffer (or window) has already been closed and removing a scrollbar
    // causes a resize event. Don't resize then, it will happen after entering
    // another buffer.
    if (curwin->w_buffer == NULL || curwin->w_lines == NULL)
	return;

#ifdef AMIGA
    out_flush();	    // must do this before mch_get_shellsize() for
			    // some obscure reason
#endif

    if (mustset || (ui_get_shellsize() == FAIL && height != 0))
    {
	Rows = height;
	Columns = width;
	check_shellsize();
	ui_set_shellsize(mustset);
    }
    else
	check_shellsize();

    // The window layout used to be adjusted here, but it now happens in
    // screenalloc() (also invoked from screenclear()).  That is because the
    // "busy" check above may skip this, but not screenalloc().

    if (State != MODE_ASKMORE && State != MODE_EXTERNCMD
						      && State != MODE_CONFIRM)
	screenclear();
    else
	screen_start();	    // don't know where cursor is now

    if (starting != NO_SCREEN)
    {
	maketitle();

	changed_line_abv_curs();
	invalidate_botline();

	/*
	 * We only redraw when it's needed:
	 * - While at the more prompt or executing an external command, don't
	 *   redraw, but position the cursor.
	 * - While editing the command line, only redraw that.
	 * - in Ex mode, don't redraw anything.
	 * - Otherwise, redraw right now, and position the cursor.
	 * Always need to call update_screen() or screenalloc(), to make
	 * sure Rows/Columns and the size of ScreenLines[] is correct!
	 */
	if (State == MODE_ASKMORE || State == MODE_EXTERNCMD
				     || State == MODE_CONFIRM || exmode_active)
	{
	    screenalloc(FALSE);
	    repeat_message();
	}
	else
	{
	    if (curwin->w_p_scb)
		do_check_scrollbind(TRUE);
	    if (State & MODE_CMDLINE)
	    {
		update_screen(UPD_NOT_VALID);
		redrawcmdline();
	    }
	    else
	    {
		update_topline();
		if (pum_visible())
		{
		    redraw_later(UPD_NOT_VALID);
		    ins_compl_show_pum();
		}
		update_screen(UPD_NOT_VALID);
		if (redrawing())
		    setcursor();
	    }
	}
	cursor_on();	    // redrawing may have switched it off
    }
    out_flush();
}

    void
set_shellsize(int width, int height, int mustset)
{
    static int	busy = FALSE;
    static int	do_run = FALSE;

    if (width < 0 || height < 0)    // just checking...
	return;

    if (State == MODE_HITRETURN || State == MODE_SETWSIZE)
    {
	// postpone the resizing
	State = MODE_SETWSIZE;
	return;
    }

    // Avoid recursiveness.  This can happen when setting the window size
    // causes another window-changed signal or when two SIGWINCH signals come
    // very close together.  There needs to be another run then after the
    // current one is done to pick up the latest size.
    do_run = TRUE;
    if (busy)
	return;

    while (do_run)
    {
	do_run = FALSE;
	busy = TRUE;
	set_shellsize_inner(width, height, mustset);
	busy = FALSE;
    }
}

/*
 * Output T_CTE, the t_TE termcap entry, and handle expected effects.
 * The code possibly disables modifyOtherKeys and the Kitty keyboard protocol.
 */
    void
out_str_t_TE(void)
{
    out_str(T_CTE);

    // The seenModifyOtherKeys flag is not reset here.  We do expect t_TE to
    // disable modifyOtherKeys, but until Xterm version 377 there is no way to
    // detect it's enabled again after the following t_TI.  We assume that when
    // seenModifyOtherKeys was set before it will still be valid.

    // When the modifyOtherKeys level is detected to be 2 we expect t_TE to
    // disable it.  Remembering that it was detected to be enabled is useful in
    // some situations.
    // The following t_TI is expected to request the state and then
    // modify_otherkeys_state will be set again.
    if (modify_otherkeys_state == MOKS_ENABLED
	    || modify_otherkeys_state == MOKS_DISABLED)
	modify_otherkeys_state = MOKS_DISABLED;
    else if (modify_otherkeys_state != MOKS_INITIAL)
	modify_otherkeys_state = MOKS_AFTER_T_TE;

    // When the kitty keyboard protocol is enabled we expect t_TE to disable
    // it.  Remembering that it was detected to be enabled is useful in some
    // situations.
    // The following t_TI is expected to request the state and then
    // kitty_protocol_state will be set again.
    if (kitty_protocol_state == KKPS_ENABLED
	    || kitty_protocol_state == KKPS_DISABLED)
	kitty_protocol_state = KKPS_DISABLED;
    else
	kitty_protocol_state = KKPS_AFTER_T_TE;
}

static int send_t_RK = FALSE;

/*
 * Output T_TI and setup for what follows.
 */
    void
out_str_t_TI(void)
{
    out_str(T_CTI);

    // Send t_RK when there is no more work to do.
    send_t_RK = TRUE;
}

/*
 * Output T_BE, but only when t_PS and t_PE are set.
 */
    void
out_str_t_BE(void)
{
    char_u *p;

    if (T_BE == NULL || *T_BE == NUL
	    || (p = find_termcode((char_u *)"PS")) == NULL || *p == NUL
	    || (p = find_termcode((char_u *)"PE")) == NULL || *p == NUL)
	return;
    out_str(T_BE);
}

/*
 * If t_TI was recently sent and there is no typeahead or work to do, now send
 * t_RK.  This is postponed to avoid the response arriving in a shell command
 * or after Vim exits.
 */
    void
may_send_t_RK(void)
{
    if (send_t_RK
	    && !work_pending()
	    && !ex_normal_busy
#ifdef FEAT_EVAL
	    && !in_feedkeys
#endif
	    && !exiting)
    {
	send_t_RK = FALSE;
	out_str(T_CRK);
	out_flush();
    }
}

/*
 * Set the terminal to TMODE_RAW (for Normal mode) or TMODE_COOK (for external
 * commands and Ex mode).
 */
    void
settmode(tmode_T tmode)
{
#ifdef FEAT_GUI
    // don't set the term where gvim was started to any mode
    if (gui.in_use)
	return;
#endif

    if (!full_screen)
	return;

    /*
     * When returning after calling a shell cur_tmode is TMODE_UNKNOWN,
     * set the terminal to raw mode, even though we think it already is,
     * because the shell program may have reset the terminal mode.
     * When we think the terminal is normal, don't try to set it to
     * normal again, because that causes problems (logout!) on some
     * machines.
     */
    if (tmode != cur_tmode)
    {
#ifdef FEAT_TERMRESPONSE
# ifdef FEAT_GUI
	if (!gui.in_use && !gui.starting)
# endif
	{
	    // May need to check for T_CRV response and termcodes, it
	    // doesn't work in Cooked mode, an external program may get
	    // them.
	    if (tmode != TMODE_RAW && termrequest_any_pending())
		(void)vpeekc_nomap();
	    check_for_codes_from_term();
	}
#endif
	if (tmode != TMODE_RAW)
	    mch_setmouse(FALSE);	// switch mouse off

	// Disable bracketed paste and modifyOtherKeys in cooked mode.
	// Avoid doing this too often, on some terminals the codes are not
	// handled properly.
	if (termcap_active && tmode != TMODE_SLEEP
		&& cur_tmode != TMODE_SLEEP)
	{
	    MAY_WANT_TO_LOG_THIS;

	    if (tmode != TMODE_RAW)
	    {
		out_str(T_BD);	// disable bracketed paste mode
		out_str_t_TE();	// possibly disables modifyOtherKeys
	    }
	    else
	    {
		out_str_t_BE();	// enable bracketed paste mode (should
				// be before mch_settmode().
		out_str_t_TI();	// possibly enables modifyOtherKeys
	    }
	}
	out_flush();
	mch_settmode(tmode);	// machine specific function
	cur_tmode = tmode;
	if (tmode == TMODE_RAW)
	    setmouse();		// may switch mouse on
	out_flush();
    }
#ifdef FEAT_TERMRESPONSE
    may_req_termresponse();
#endif
}

    void
starttermcap(void)
{
    if (!full_screen || termcap_active)
	return;

    MAY_WANT_TO_LOG_THIS;

    out_str(T_TI);			// start termcap mode
    out_str_t_TI();			// start "raw" mode
    out_str(T_KS);			// start "keypad transmit" mode
    out_str_t_BE();			// enable bracketed paste mode

#if defined(UNIX) || defined(VMS)
    // Enable xterm's focus reporting mode when 'esckeys' is set.
    if (p_ek && *T_FE != NUL)
	out_str(T_FE);
#endif

    out_flush();
    termcap_active = TRUE;
    screen_start();			// don't know where cursor is now
#ifdef FEAT_TERMRESPONSE
# ifdef FEAT_GUI
    if (!gui.in_use && !gui.starting)
# endif
    {
	may_req_termresponse();
	// Immediately check for a response.  If t_Co changes, we don't
	// want to redraw with wrong colors first.
	if (crv_status.tr_progress == STATUS_SENT)
	    check_for_codes_from_term();
    }
#endif
}

    void
stoptermcap(void)
{
    screen_stop_highlight();
    reset_cterm_colors();

    if (!termcap_active)
	return;

#ifdef FEAT_TERMRESPONSE
# ifdef FEAT_GUI
    if (!gui.in_use && !gui.starting)
# endif
    {
	// May need to discard T_CRV, T_U7 or T_RBG response.
	if (termrequest_any_pending())
	{
# ifdef UNIX
	    // Give the terminal a chance to respond.
	    mch_delay(100L, 0);
# endif
# ifdef TCIFLUSH
	    // Discard data received but not read.
	    if (exiting)
		tcflush(fileno(stdin), TCIFLUSH);
# endif
	}
	// Check for termcodes first, otherwise an external program may
	// get them.
	check_for_codes_from_term();
    }
#endif
    MAY_WANT_TO_LOG_THIS;

#if defined(UNIX) || defined(VMS)
    // Disable xterm's focus reporting mode if 'esckeys' is set.
    if (p_ek && *T_FD != NUL)
	out_str(T_FD);
#endif

    out_str(T_BD);			// disable bracketed paste mode
    out_str(T_KE);			// stop "keypad transmit" mode
    out_flush();
    termcap_active = FALSE;

    // Output t_te before t_TE, t_te may switch between main and alternate
    // screen and following codes may work on the active screen only.
    //
    // When using the Kitty keyboard protocol the main and alternate screen
    // use a separate state.  If we are (or were) using the Kitty keyboard
    // protocol and t_te is not empty (possibly switching screens) then
    // output t_TE both before and after outputting t_te.
    if (*T_TE != NUL && (kitty_protocol_state == KKPS_ENABLED
		|| kitty_protocol_state == KKPS_DISABLED))
	out_str_t_TE();		// probably disables the kitty keyboard
				// protocol

    out_str(T_TE);			// stop termcap mode
    cursor_on();			// just in case it is still off
    out_str_t_TE();			// stop "raw" mode, modifyOtherKeys and
					// Kitty keyboard protocol
    screen_start();			// don't know where cursor is now
    out_flush();
}

#if defined(FEAT_TERMRESPONSE) || defined(PROTO)
/*
 * Request version string (for xterm) when needed.
 * Only do this after switching to raw mode, otherwise the result will be
 * echoed.
 * Only do this after startup has finished, to avoid that the response comes
 * while executing "-c !cmd" or even after "-c quit".
 * Only do this after termcap mode has been started, otherwise the codes for
 * the cursor keys may be wrong.
 * Only do this when 'esckeys' is on, otherwise the response causes trouble in
 * Insert mode.
 * On Unix only do it when both output and input are a tty (avoid writing
 * request to terminal while reading from a file).
 * The result is caught in check_termcode().
 */
    void
may_req_termresponse(void)
{
    if (crv_status.tr_progress == STATUS_GET
	    && can_get_termresponse()
	    && starting == 0
	    && *T_CRV != NUL)
    {
	MAY_WANT_TO_LOG_THIS;
	LOG_TR(("Sending CRV request"));
	out_str(T_CRV);
	termrequest_sent(&crv_status);
	// check for the characters now, otherwise they might be eaten by
	// get_keystroke()
	out_flush();
	(void)vpeekc_nomap();
    }
}

/*
 * Send sequences to the terminal and check with t_u7 how the cursor moves, to
 * find out properties of the terminal.
 * Note that this goes out before T_CRV, so that the result can be used when
 * the termresponse arrives.
 */
    void
check_terminal_behavior(void)
{
    int	    did_send = FALSE;

    if (!can_get_termresponse() || starting != 0 || *T_U7 == NUL)
	return;

    if (u7_status.tr_progress == STATUS_GET
	    && !option_was_set((char_u *)"ambiwidth"))
    {
	char_u	buf[16];

	// Ambiguous width check.
	// Check how the terminal treats ambiguous character width (UAX #11).
	// First, we move the cursor to (1, 0) and print a test ambiguous
	// character \u25bd (WHITE DOWN-POINTING TRIANGLE) and then query
	// the current cursor position.  If the terminal treats \u25bd as
	// single width, the position is (1, 1), or if it is treated as double
	// width, that will be (1, 2).  This function has the side effect that
	// changes cursor position, so it must be called immediately after
	// entering termcap mode.
	MAY_WANT_TO_LOG_THIS;
	LOG_TR(("Sending request for ambiwidth check"));
	// Do this in the second row.  In the first row the returned sequence
	// may be CSI 1;2R, which is the same as <S-F3>.
	term_windgoto(1, 0);
	buf[mb_char2bytes(0x25bd, buf)] = NUL;
	out_str(buf);
	out_str(T_U7);
	termrequest_sent(&u7_status);
	out_flush();
	did_send = TRUE;

	// This overwrites a few characters on the screen, a redraw is needed
	// after this. Clear them out for now.
	screen_stop_highlight();
	term_windgoto(1, 0);
	out_str((char_u *)"  ");
	line_was_clobbered(1);
    }

    if (xcc_status.tr_progress == STATUS_GET && Rows > 2)
    {
	// 2. Check compatibility with xterm.
	// We move the cursor to (2, 0), print a test sequence and then query
	// the current cursor position.  If the terminal properly handles
	// unknown DCS string and CSI sequence with intermediate byte, the test
	// sequence is ignored and the cursor does not move.  If the terminal
	// handles test sequence incorrectly, a garbage string is displayed and
	// the cursor does move.
	MAY_WANT_TO_LOG_THIS;
	LOG_TR(("Sending xterm compatibility test sequence."));
	// Do this in the third row.  Second row is used by ambiguous
	// character width check.
	term_windgoto(2, 0);
	// send the test DCS string.
	out_str((char_u *)"\033Pzz\033\\");
	// send the test CSI sequence with intermediate byte.
	out_str((char_u *)"\033[0%m");
	out_str(T_U7);
	termrequest_sent(&xcc_status);
	out_flush();
	did_send = TRUE;

	// If the terminal handles test sequence incorrectly, garbage text is
	// displayed. Clear them out for now.
	screen_stop_highlight();
	term_windgoto(2, 0);
	out_str((char_u *)"           ");
	line_was_clobbered(2);
    }

    if (did_send)
    {
	term_windgoto(0, 0);

	// Need to reset the known cursor position.
	screen_start();

	// check for the characters now, otherwise they might be eaten by
	// get_keystroke()
	out_flush();
	(void)vpeekc_nomap();
    }
}

/*
 * Similar to requesting the version string: Request the terminal background
 * color when it is the right moment.
 */
    void
may_req_bg_color(void)
{
    if (can_get_termresponse() && starting == 0)
    {
	int didit = FALSE;

# ifdef FEAT_TERMINAL
	// Only request foreground if t_RF is set.
	if (rfg_status.tr_progress == STATUS_GET && *T_RFG != NUL)
	{
	    MAY_WANT_TO_LOG_THIS;
	    LOG_TR(("Sending FG request"));
	    out_str(T_RFG);
	    termrequest_sent(&rfg_status);
	    didit = TRUE;
	}
# endif

	// Only request background if t_RB is set.
	if (rbg_status.tr_progress == STATUS_GET && *T_RBG != NUL)
	{
	    MAY_WANT_TO_LOG_THIS;
	    LOG_TR(("Sending BG request"));
	    out_str(T_RBG);
	    termrequest_sent(&rbg_status);
	    didit = TRUE;
	}

	if (didit)
	{
	    // check for the characters now, otherwise they might be eaten by
	    // get_keystroke()
	    out_flush();
	    (void)vpeekc_nomap();
	}
    }
}

# ifdef DEBUG_TERMRESPONSE
    static void
log_tr(const char *fmt, ...)
{
    static FILE *fd_tr = NULL;
    static proftime_T start;
    proftime_T now;
    va_list ap;

    if (fd_tr == NULL)
    {
	fd_tr = fopen("termresponse.log", "w");
	profile_start(&start);
    }
    now = start;
    profile_end(&now);
    fprintf(fd_tr, "%s: %s ", profile_msg(&now),
				must_redraw == UPD_NOT_VALID ? "NV"
				     : must_redraw == UPD_CLEAR ? "CL" : "  ");
    va_start(ap, fmt);
    vfprintf(fd_tr, fmt, ap);
    va_end(ap);
    fputc('\n', fd_tr);
    fflush(fd_tr);
}
# endif
#endif

/*
 * Return TRUE when saving and restoring the screen.
 */
    int
swapping_screen(void)
{
    return (full_screen && *T_TI != NUL);
}

/*
 * By outputting the 'cursor very visible' termcap code, for some windowed
 * terminals this makes the screen scrolled to the correct position.
 * Used when starting Vim or returning from a shell.
 */
    void
scroll_start(void)
{
    if (*T_VS == NUL || *T_CVS == NUL)
	return;

    MAY_WANT_TO_LOG_THIS;
    out_str(T_VS);
    out_str(T_CVS);
    screen_start();		// don't know where cursor is now
}

// True if cursor is not visible
static int cursor_is_off = FALSE;

// True if cursor is not visible due to an ongoing cursor-less sleep
static int cursor_is_asleep = FALSE;

/*
 * Enable the cursor without checking if it's already enabled.
 */
    void
cursor_on_force(void)
{
    out_str(T_VE);
    cursor_is_off = FALSE;
    cursor_is_asleep = FALSE;
}

/*
 * Enable the cursor if it's currently off.
 */
    void
cursor_on(void)
{
    if (cursor_is_off && !cursor_is_asleep)
	cursor_on_force();
}

/*
 * Disable the cursor.
 */
    void
cursor_off(void)
{
    if (full_screen && !cursor_is_off)
    {
	out_str(T_VI);	    // disable cursor
	cursor_is_off = TRUE;
    }
}

#ifdef FEAT_GUI
/*
 * Check whether the cursor is invisible due to an ongoing cursor-less sleep
 */
    int
cursor_is_sleeping(void)
{
    return cursor_is_asleep;
}
#endif

/*
 * Disable the cursor and mark it disabled by cursor-less sleep
 */
    void
cursor_sleep(void)
{
    cursor_is_asleep = TRUE;
    cursor_off();
}

/*
 * Enable the cursor and mark it not disabled by cursor-less sleep
 */
    void
cursor_unsleep(void)
{
    cursor_is_asleep = FALSE;
    cursor_on();
}

#if defined(CURSOR_SHAPE) || defined(PROTO)
/*
 * Set cursor shape to match Insert or Replace mode.
 */
    void
term_cursor_mode(int forced)
{
    static int showing_mode = -1;
    char_u *p;

    // Only do something when redrawing the screen and we can restore the
    // mode.
    if (!full_screen || *T_CEI == NUL)
    {
# ifdef FEAT_TERMRESPONSE
	if (forced && initial_cursor_shape > 0)
	    // Restore to initial values.
	    term_cursor_shape(initial_cursor_shape, initial_cursor_blink);
# endif
	return;
    }

    if ((State & MODE_REPLACE) == MODE_REPLACE)
    {
	if (forced || showing_mode != MODE_REPLACE)
	{
	    if (*T_CSR != NUL)
		p = T_CSR;	// Replace mode cursor
	    else
		p = T_CSI;	// fall back to Insert mode cursor
	    if (*p != NUL)
	    {
		out_str(p);
		showing_mode = MODE_REPLACE;
	    }
	}
    }
    else if (State & MODE_INSERT)
    {
	if ((forced || showing_mode != MODE_INSERT) && *T_CSI != NUL)
	{
	    out_str(T_CSI);	    // Insert mode cursor
	    showing_mode = MODE_INSERT;
	}
    }
    else if (forced || showing_mode != MODE_NORMAL)
    {
	out_str(T_CEI);		    // non-Insert mode cursor
	showing_mode = MODE_NORMAL;
    }
}

# if defined(FEAT_TERMINAL) || defined(PROTO)
    void
term_cursor_color(char_u *color)
{
    if (*T_CSC == NUL)
	return;

    out_str(T_CSC);		// set cursor color start
    out_str_nf(color);
    out_str(T_CEC);		// set cursor color end
    out_flush();
}
# endif

    int
blink_state_is_inverted(void)
{
#ifdef FEAT_TERMRESPONSE
    return rbm_status.tr_progress == STATUS_GOT
	&& rcs_status.tr_progress == STATUS_GOT
		&& initial_cursor_blink != initial_cursor_shape_blink;
#else
    return FALSE;
#endif
}

/*
 * "shape": 1 = block, 2 = underline, 3 = vertical bar
 */
    void
term_cursor_shape(int shape, int blink)
{
    if (*T_CSH != NUL)
    {
	OUT_STR(tgoto((char *)T_CSH, 0, shape * 2 - blink));
	out_flush();
    }
    else
    {
	int do_blink = blink;

	// t_SH is empty: try setting just the blink state.
	// The blink flags are XORed together, if the initial blinking from
	// style and shape differs, we need to invert the flag here.
	if (blink_state_is_inverted())
	    do_blink = !blink;

	if (do_blink && *T_VS != NUL)
	{
	    out_str(T_VS);
	    out_flush();
	}
	else if (!do_blink && *T_CVS != NUL)
	{
	    out_str(T_CVS);
	    out_flush();
	}
    }
}
#endif

/*
 * Set scrolling region for window 'wp'.
 * The region starts 'off' lines from the start of the window.
 * Also set the vertical scroll region for a vertically split window.  Always
 * the full width of the window, excluding the vertical separator.
 */
    void
scroll_region_set(win_T *wp, int off)
{
    OUT_STR(tgoto((char *)T_CS, W_WINROW(wp) + wp->w_height - 1,
							 W_WINROW(wp) + off));
    if (*T_CSV != NUL && wp->w_width != Columns)
	OUT_STR(tgoto((char *)T_CSV, wp->w_wincol + wp->w_width - 1,
							       wp->w_wincol));
    screen_start();		    // don't know where cursor is now
}

/*
 * Reset scrolling region to the whole screen.
 */
    void
scroll_region_reset(void)
{
    OUT_STR(tgoto((char *)T_CS, (int)Rows - 1, 0));
    if (*T_CSV != NUL)
	OUT_STR(tgoto((char *)T_CSV, (int)Columns - 1, 0));
    screen_start();		    // don't know where cursor is now
}


/*
 * List of terminal codes that are currently recognized.
 */

static struct termcode
{
    char_u  name[2];	    // termcap name of entry
    char_u  *code;	    // terminal code (in allocated memory)
    int	    len;	    // STRLEN(code)
    int	    modlen;	    // length of part before ";*~".
} *termcodes = NULL;

static int  tc_max_len = 0; // number of entries that termcodes[] can hold
static int  tc_len = 0;	    // current number of entries in termcodes[]

static int termcode_star(char_u *code, int len);

    void
clear_termcodes(void)
{
    while (tc_len > 0)
	vim_free(termcodes[--tc_len].code);
    VIM_CLEAR(termcodes);
    tc_max_len = 0;

#ifdef HAVE_TGETENT
    BC = (char *)empty_option;
    UP = (char *)empty_option;
    PC = NUL;			// set pad character to NUL
    ospeed = 0;
#endif

    need_gather = TRUE;		// need to fill termleader[]
}

#define ATC_FROM_TERM 55

/*
 * For xterm we recognize special codes like "ESC[42;*X" and "ESC O*X" that
 * accept modifiers.
 * Set "termcodes[idx].modlen".
 */
    static void
adjust_modlen(int idx)
{
    termcodes[idx].modlen = 0;
    int j = termcode_star(termcodes[idx].code, termcodes[idx].len);
    if (j <= 0)
	return;

    termcodes[idx].modlen = termcodes[idx].len - 1 - j;
    // For "CSI[@;X" the "@" is not included in "modlen".
    if (termcodes[idx].code[termcodes[idx].modlen - 1] == '@')
	--termcodes[idx].modlen;
}

/*
 * Add a new entry for "name[2]" to the list of terminal codes.
 * Note that "name" may not have a terminating NUL.
 * The list is kept alphabetical for ":set termcap"
 * "flags" is TRUE when replacing 7-bit by 8-bit controls is desired.
 * "flags" can also be ATC_FROM_TERM for got_code_from_term().
 */
    void
add_termcode(char_u *name, char_u *string, int flags)
{
    struct termcode *new_tc;
    int		    i, j;
    char_u	    *s;
    int		    len;
#ifdef FEAT_EVAL
    char	    *action = "Setting";
#endif

    if (string == NULL || *string == NUL)
    {
	del_termcode(name);
	return;
    }

#if defined(MSWIN) && !defined(FEAT_GUI)
    s = vim_strnsave(string, STRLEN(string) + 1);
#else
# ifdef VIMDLL
    if (!gui.in_use)
	s = vim_strnsave(string, STRLEN(string) + 1);
    else
# endif
	s = vim_strsave(string);
#endif
    if (s == NULL)
	return;

    // Change leading <Esc>[ to CSI, change <Esc>O to <M-O>.
    if (flags != 0 && flags != ATC_FROM_TERM && term_7to8bit(string) != 0)
    {
	STRMOVE(s, s + 1);
	s[0] = term_7to8bit(string);
    }

#if defined(MSWIN) && (!defined(FEAT_GUI) || defined(VIMDLL))
# ifdef VIMDLL
    if (!gui.in_use)
# endif
    {
	if (s[0] == K_NUL)
	{
	    STRMOVE(s + 1, s);
	    s[1] = 3;
	}
    }
#endif

    len = (int)STRLEN(s);

    need_gather = TRUE;		// need to fill termleader[]

    /*
     * need to make space for more entries
     */
    if (tc_len == tc_max_len)
    {
	tc_max_len += 20;
	new_tc = ALLOC_MULT(struct termcode, tc_max_len);
	if (new_tc == NULL)
	{
	    tc_max_len -= 20;
	    vim_free(s);
	    return;
	}
	for (i = 0; i < tc_len; ++i)
	    new_tc[i] = termcodes[i];
	vim_free(termcodes);
	termcodes = new_tc;
    }

    /*
     * Look for existing entry with the same name, it is replaced.
     * Look for an existing entry that is alphabetical higher, the new entry
     * is inserted in front of it.
     */
    for (i = 0; i < tc_len; ++i)
    {
	if (termcodes[i].name[0] < name[0])
	    continue;
	if (termcodes[i].name[0] == name[0])
	{
	    if (termcodes[i].name[1] < name[1])
		continue;
	    /*
	     * Exact match: May replace old code.
	     */
	    if (termcodes[i].name[1] == name[1])
	    {
		if (flags == ATC_FROM_TERM && (j = termcode_star(
				    termcodes[i].code, termcodes[i].len)) > 0)
		{
		    // Don't replace ESC[123;*X or ESC O*X with another when
		    // invoked from got_code_from_term().
		    if (len == termcodes[i].len - j
			    && STRNCMP(s, termcodes[i].code, len - 1) == 0
			    && s[len - 1]
				   == termcodes[i].code[termcodes[i].len - 1])
		    {
			// They are equal but for the ";*": don't add it.
#ifdef FEAT_EVAL
			ch_log(NULL, "Termcap entry %c%c did not change",
							     name[0], name[1]);
#endif
			vim_free(s);
			return;
		    }
		}
		else
		{
		    // Replace old code.
#ifdef FEAT_EVAL
		    ch_log(NULL, "Termcap entry %c%c was: %s",
					  name[0], name[1], termcodes[i].code);
#endif
		    vim_free(termcodes[i].code);
		    --tc_len;
		    break;
		}
	    }
	}
	/*
	 * Found alphabetical larger entry, move rest to insert new entry
	 */
#ifdef FEAT_EVAL
	action = "Adding";
#endif
	for (j = tc_len; j > i; --j)
	    termcodes[j] = termcodes[j - 1];
	break;
    }

#ifdef FEAT_EVAL
    ch_log(NULL, "%s termcap entry %c%c to %s", action, name[0], name[1], s);
#endif
    termcodes[i].name[0] = name[0];
    termcodes[i].name[1] = name[1];
    termcodes[i].code = s;
    termcodes[i].len = len;
    adjust_modlen(i);

    ++tc_len;
}

/*
 * Some function keys may include modifiers, but the terminfo/termcap entries
 * do not indicate that.  Insert ";*" where we expect modifiers might appear.
 */
    static void
accept_modifiers_for_function_keys(void)
{
    regmatch_T regmatch;
    CLEAR_FIELD(regmatch);
    regmatch.rm_ic = TRUE;
    regmatch.regprog = vim_regcomp((char_u *)"^\033[\\d\\+\\~$", RE_MAGIC);

    for (int i = 0; i < tc_len; ++i)
    {
	if (regmatch.regprog == NULL)
	    return;

	// skip PasteStart and PasteEnd
	if (termcodes[i].name[0] == 'P'
		&& (termcodes[i].name[1] == 'S' || termcodes[i].name[1] == 'E'))
	    continue;

	char_u *s = termcodes[i].code;
	if (s != NULL && vim_regexec(&regmatch, s, (colnr_T)0))
	{
	    size_t len = STRLEN(s);
	    char_u *ns = alloc(len + 3);
	    if (ns != NULL)
	    {
		mch_memmove(ns, s, len - 1);
		mch_memmove(ns + len - 1, ";*~", 4);
		vim_free(s);
		termcodes[i].code = ns;
		termcodes[i].len += 2;
		adjust_modlen(i);
	    }
	}
    }

    vim_regfree(regmatch.regprog);
}

/*
 * Check termcode "code[len]" for ending in ;*X or *X.
 * The "X" can be any character.
 * Return 0 if not found, 2 for ;*X and 1 for *X.
 */
    static int
termcode_star(char_u *code, int len)
{
    // Shortest is <M-O>*X.  With ; shortest is <CSI>@;*X
    if (len >= 3 && code[len - 2] == '*')
    {
	if (len >= 5 && code[len - 3] == ';')
	    return 2;
	else
	    return 1;
    }
    return 0;
}

    char_u  *
find_termcode(char_u *name)
{
    int	    i;

    for (i = 0; i < tc_len; ++i)
	if (termcodes[i].name[0] == name[0] && termcodes[i].name[1] == name[1])
	    return termcodes[i].code;
    return NULL;
}

    char_u *
get_termcode(int i)
{
    if (i >= tc_len)
	return NULL;
    return &termcodes[i].name[0];
}

/*
 * Returns the length of the terminal code at index 'idx'.
 */
    int
get_termcode_len(int idx)
{
    return termcodes[idx].len;
}

    void
del_termcode(char_u *name)
{
    int	    i;

    if (termcodes == NULL)	// nothing there yet
	return;

    need_gather = TRUE;		// need to fill termleader[]

    for (i = 0; i < tc_len; ++i)
	if (termcodes[i].name[0] == name[0] && termcodes[i].name[1] == name[1])
	{
	    del_termcode_idx(i);
	    return;
	}
    // not found. Give error message?
}

    static void
del_termcode_idx(int idx)
{
    int		i;

    vim_free(termcodes[idx].code);
    --tc_len;
    for (i = idx; i < tc_len; ++i)
	termcodes[i] = termcodes[i + 1];
}

/*
 * Called when detected that the terminal sends 8-bit codes.
 * Convert all 7-bit codes to their 8-bit equivalent.
 */
    static void
switch_to_8bit(void)
{
    int		i;
    int		c;

    // Only need to do something when not already using 8-bit codes.
    if (!term_is_8bit(T_NAME))
    {
	for (i = 0; i < tc_len; ++i)
	{
	    c = term_7to8bit(termcodes[i].code);
	    if (c != 0)
	    {
		STRMOVE(termcodes[i].code + 1, termcodes[i].code + 2);
		termcodes[i].code[0] = c;
	    }
	}
	need_gather = TRUE;		// need to fill termleader[]
    }
    detected_8bit = TRUE;
    LOG_TR(("Switching to 8 bit"));
}

#ifdef CHECK_DOUBLE_CLICK
static linenr_T orig_topline = 0;
# ifdef FEAT_DIFF
static int orig_topfill = 0;
# endif
#endif
#if defined(CHECK_DOUBLE_CLICK) || defined(PROTO)
/*
 * Checking for double-clicks ourselves.
 * "orig_topline" is used to avoid detecting a double-click when the window
 * contents scrolled (e.g., when 'scrolloff' is non-zero).
 */
/*
 * Set orig_topline.  Used when jumping to another window, so that a double
 * click still works.
 */
    void
set_mouse_topline(win_T *wp)
{
    orig_topline = wp->w_topline;
# ifdef FEAT_DIFF
    orig_topfill = wp->w_topfill;
# endif
}

/*
 * Returns TRUE if the top line and top fill of window 'wp' matches the saved
 * topline and topfill.
 */
    int
is_mouse_topline(win_T *wp)
{
    return orig_topline == wp->w_topline
#ifdef FEAT_DIFF
	&& orig_topfill == wp->w_topfill
#endif
	;
}
#endif

/*
 * If "buf" is NULL put "string[new_slen]" in typebuf; "buflen" is not used.
 * If "buf" is not NULL put "string[new_slen]" in "buf[bufsize]" and adjust
 * "buflen".
 * Remove "slen" bytes.
 * Returns FAIL for error.
 */
    int
put_string_in_typebuf(
	int	offset,
	int	slen,
	char_u	*string,
	int	new_slen,
	char_u	*buf,
	int	bufsize,
	int	*buflen)
{
    int		extra = new_slen - slen;

    string[new_slen] = NUL;
    if (buf == NULL)
    {
	if (extra < 0)
	    // remove matched chars, taking care of noremap
	    del_typebuf(-extra, offset);
	else if (extra > 0)
	    // insert the extra space we need
	    if (ins_typebuf(string + slen, REMAP_YES, offset, FALSE, FALSE)
								       == FAIL)
		return FAIL;

	// Careful: del_typebuf() and ins_typebuf() may have reallocated
	// typebuf.tb_buf[]!
	mch_memmove(typebuf.tb_buf + typebuf.tb_off + offset, string,
							     (size_t)new_slen);
    }
    else
    {
	if (extra < 0)
	    // remove matched characters
	    mch_memmove(buf + offset, buf + offset - extra,
					   (size_t)(*buflen + offset + extra));
	else if (extra > 0)
	{
	    // Insert the extra space we need.  If there is insufficient
	    // space return -1.
	    if (*buflen + extra + new_slen >= bufsize)
		return FAIL;
	    mch_memmove(buf + offset + extra, buf + offset,
						   (size_t)(*buflen - offset));
	}
	mch_memmove(buf + offset, string, (size_t)new_slen);
	*buflen = *buflen + extra + new_slen;
    }
    return OK;
}

/*
 * Decode a modifier number as xterm provides it into MOD_MASK bits.
 */
    int
decode_modifiers(int n)
{
    int	    code = n - 1;
    int	    modifiers = 0;

    if (code & 1)
	modifiers |= MOD_MASK_SHIFT;
    if (code & 2)
	modifiers |= MOD_MASK_ALT;
    if (code & 4)
	modifiers |= MOD_MASK_CTRL;
    if (code & 8)
	modifiers |= MOD_MASK_META;
    // Any further modifiers are silently dropped.

    return modifiers;
}

    static int
modifiers2keycode(int modifiers, int *key, char_u *string)
{
    int new_slen = 0;

    if (modifiers == 0)
	return 0;

    // Some keys have the modifier included.  Need to handle that here to
    // make mappings work.  This may result in a special key, such as
    // K_S_TAB.
    *key = simplify_key(*key, &modifiers);
    if (modifiers != 0)
    {
	string[new_slen++] = K_SPECIAL;
	string[new_slen++] = (int)KS_MODIFIER;
	string[new_slen++] = modifiers;
    }
    return new_slen;
}

/*
 * Handle a cursor position report.
 */
    static void
handle_u7_response(int *arg, char_u *tp UNUSED, int csi_len UNUSED)
{
    if (arg[0] == 2 && arg[1] >= 2)
    {
	char *aw = NULL;

	LOG_TR(("Received U7 status: %s", tp));
	u7_status.tr_progress = STATUS_GOT;
	did_cursorhold = TRUE;
	if (arg[1] == 2)
	    aw = "single";
	else if (arg[1] == 3)
	    aw = "double";
	if (aw != NULL && STRCMP(aw, p_ambw) != 0)
	{
	    // Setting the option causes a screen redraw. Do
	    // that right away if possible, keeping any
	    // messages.
	    set_option_value_give_err((char_u *)"ambw", 0L, (char_u *)aw, 0);
#ifdef DEBUG_TERMRESPONSE
	    {
		int r = redraw_asap(UPD_CLEAR);

		log_tr("set 'ambiwidth', redraw_asap(): %d", r);
	    }
#else
	    redraw_asap(UPD_CLEAR);
#endif
#ifdef FEAT_EVAL
	    set_vim_var_string(VV_TERMU7RESP, tp, csi_len);
#endif
	    apply_autocmds(EVENT_TERMRESPONSEALL,
					(char_u *)"ambiguouswidth", NULL, FALSE, curbuf);
	}
    }
    else if (arg[0] == 3)
    {
	int value;

	LOG_TR(("Received compatibility test result: %s", tp));
	xcc_status.tr_progress = STATUS_GOT;

	// Third row: xterm compatibility test.
	// If the cursor is on the first column then the terminal can handle
	// the request for cursor style and blinking.
	value = arg[1] == 1 ? TPR_YES : TPR_NO;
	term_props[TPR_CURSOR_STYLE].tpr_status = value;
	term_props[TPR_CURSOR_BLINK].tpr_status = value;
    }
}

/*
 * Handle a response to T_CRV: {lead}{first}{x};{vers};{y}c
 * Xterm and alike use '>' for {first}.
 * Rxvt sends "{lead}?1;2c".
 */
    static void
handle_version_response(int first, int *arg, int argc, char_u *tp)
{
    // The xterm version.  It is set to zero when it can't be an actual xterm
    // version.
    int version = arg[1];

    LOG_TR(("Received CRV response: %s", tp));
    crv_status.tr_progress = STATUS_GOT;
    did_cursorhold = TRUE;

    // Reset terminal properties that are set based on the termresponse.
    // Mainly useful for tests that send the termresponse multiple times.
    // For testing all props can be reset.
    init_term_props(
#ifdef FEAT_EVAL
	    reset_term_props_on_termresponse
#else
	    FALSE
#endif
	    );

    // If this code starts with CSI, you can bet that the
    // terminal uses 8-bit codes.
    if (tp[0] == CSI)
	switch_to_8bit();

    // Screen sends 40500.
    // rxvt sends its version number: "20703" is 2.7.3.
    // Ignore it for when the user has set 'term' to xterm,
    // even though it's an rxvt.
    if (version > 20000)
	version = 0;

    // Figure out more if the response is CSI > 99 ; 99 ; 99 c
    if (first == '>' && argc == 3)
    {
	// mintty 2.9.5 sends 77;20905;0c.
	// (77 is ASCII 'M' for mintty.)
	if (arg[0] == 77)
	{
	    // mintty can do SGR mouse reporting
	    term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_SGR;
	}

#ifdef FEAT_TERMRESPONSE
	// If xterm version >= 141 try to get termcap codes.  For other
	// terminals the request should be ignored.
	if (version >= 141 && p_xtermcodes)
	{
	    LOG_TR(("Enable checking for XT codes"));
	    check_for_codes = TRUE;
	    need_gather = TRUE;
	    req_codes_from_term();
	}
#endif

	// libvterm sends 0;100;0
	// Konsole sends 0;115;0 and works the same way
	if ((version == 100 || version == 115) && arg[0] == 0 && arg[2] == 0)
	{
	    // If run from Vim $COLORS is set to the number of
	    // colors the terminal supports.  Otherwise assume
	    // 256, libvterm supports even more.
	    if (mch_getenv((char_u *)"COLORS") == NULL)
		may_adjust_color_count(256);
	    // Libvterm can handle SGR mouse reporting.
	    term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_SGR;
	}

	if (version == 95)
	{
	    // Mac Terminal.app sends 1;95;0
	    if (arg[0] == 1 && arg[2] == 0)
	    {
		term_props[TPR_UNDERLINE_RGB].tpr_status = TPR_YES;
		term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_SGR;
	    }
	    // iTerm2 sends 0;95;0
	    else if (arg[0] == 0 && arg[2] == 0)
	    {
		// iTerm2 can do SGR mouse reporting
		term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_SGR;
	    }
	    // old iTerm2 sends 0;95;
	    else if (arg[0] == 0 && arg[2] == -1)
		term_props[TPR_UNDERLINE_RGB].tpr_status = TPR_YES;
	}

	// screen sends 83;40500;0 83 is 'S' in ASCII.
	if (arg[0] == 83)
	{
	    // screen supports SGR mouse codes since 4.7.0
	    if (arg[1] >= 40700)
		term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_SGR;
	    else
		term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_XTERM;
	}

	// If no recognized terminal has set mouse behavior, assume xterm.
	if (term_props[TPR_MOUSE].tpr_status == TPR_UNKNOWN)
	{
	    // Xterm version 277 supports SGR.
	    // Xterm version >= 95 supports mouse dragging.
	    if (version >= 277)
		term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_SGR;
	    else if (version >= 95)
		term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_XTERM2;
	}

	// Detect terminals that set $TERM to something like
	// "xterm-256color" but are not fully xterm compatible.
	//
	// Gnome terminal sends 1;3801;0, 1;4402;0 or 1;2501;0.
	// Newer Gnome-terminal sends 65;6001;1.
	// xfce4-terminal sends 1;2802;0.
	// screen sends 83;40500;0
	// Assuming any version number over 2500 is not an
	// xterm (without the limit for rxvt and screen).
	if (arg[1] >= 2500)
	    term_props[TPR_UNDERLINE_RGB].tpr_status = TPR_YES;

	else if (version == 136 && arg[2] == 0)
	{
	    term_props[TPR_UNDERLINE_RGB].tpr_status = TPR_YES;

	    // PuTTY sends 0;136;0
	    if (arg[0] == 0)
	    {
		// supports sgr-like mouse reporting.
		term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_SGR;
	    }
	    // vandyke SecureCRT sends 1;136;0
	}

	// Konsole sends 0;115;0 - but t_u8 does not actually work, therefore
	// commented out.
	// else if (version == 115 && arg[0] == 0 && arg[2] == 0)
	//     term_props[TPR_UNDERLINE_RGB].tpr_status = TPR_YES;

	// Kitty up to 9.x sends 1;400{version};{secondary-version}
	if (arg[0] == 1 && arg[1] >= 4000 && arg[1] <= 4009)
	{
	    term_props[TPR_KITTY].tpr_status = TPR_YES;
	    term_props[TPR_KITTY].tpr_set_by_termresponse = TRUE;

	    // Kitty can handle SGR mouse reporting.
	    term_props[TPR_MOUSE].tpr_status = TPR_MOUSE_SGR;
	}

	// GNU screen sends 83;30600;0, 83;40500;0, etc.
	// 30600/40500 is a version number of GNU screen. DA2 support is added
	// on 3.6.  DCS string has a special meaning to GNU screen, but xterm
	// compatibility checking does not detect GNU screen.
	if (arg[0] == 83 && arg[1] >= 30600)
	{
	    term_props[TPR_CURSOR_STYLE].tpr_status = TPR_NO;
	    term_props[TPR_CURSOR_BLINK].tpr_status = TPR_NO;
	}

	// Xterm first responded to this request at patch level
	// 95, so assume anything below 95 is not xterm and hopefully supports
	// the underline RGB color sequence.
	if (version < 95)
	    term_props[TPR_UNDERLINE_RGB].tpr_status = TPR_YES;

	// Getting the cursor style is only supported properly by xterm since
	// version 279 (otherwise it returns 0x18).
	if (version < 279)
	    term_props[TPR_CURSOR_STYLE].tpr_status = TPR_NO;

	/*
	 * Take action on the detected properties.
	 */

	// Unless the underline RGB color is expected to work, disable "t_8u".
	// It does not work for the real Xterm, it resets the background color.
	// This may cause some flicker.  Alternative would be to set "t_8u"
	// here if the terminal is expected to support it, but that might
	// conflict with what was set in the .vimrc.
	if (term_props[TPR_UNDERLINE_RGB].tpr_status != TPR_YES
			&& *T_8U != NUL
			&& !option_was_set((char_u *)"t_8u"))
	{
	    set_string_option_direct((char_u *)"t_8u", -1, (char_u *)"",
								  OPT_FREE, 0);
	}
#ifdef FEAT_TERMRESPONSE
	if (*T_8U != NUL && write_t_8u_state == MAYBE)
	    // Did skip writing t_8u, a complete redraw is needed.
	    redraw_later_clear();
	write_t_8u_state = OK;  // can output t_8u now
#endif

	// Only set 'ttymouse' automatically if it was not set
	// by the user already.
	if (!option_was_set((char_u *)"ttym")
		&& (term_props[TPR_MOUSE].tpr_status == TPR_MOUSE_XTERM2
		    || term_props[TPR_MOUSE].tpr_status == TPR_MOUSE_SGR))
	{
	    set_option_value_give_err((char_u *)"ttym", 0L,
		    term_props[TPR_MOUSE].tpr_status == TPR_MOUSE_SGR
				    ? (char_u *)"sgr" : (char_u *)"xterm2", 0);
	}

#ifdef FEAT_TERMRESPONSE
	int need_flush = FALSE;

	// Only request the cursor style if t_SH and t_RS are
	// set. Only supported properly by xterm since version
	// 279 (otherwise it returns 0x18).
	// Only when getting the cursor style was detected to work.
	// Not for Terminal.app, it can't handle t_RS, it
	// echoes the characters to the screen.
	if (rcs_status.tr_progress == STATUS_GET
		&& term_props[TPR_CURSOR_STYLE].tpr_status == TPR_YES
		&& *T_CSH != NUL
		&& *T_CRS != NUL)
	{
	    MAY_WANT_TO_LOG_THIS;
	    LOG_TR(("Sending cursor style request"));
	    out_str(T_CRS);
	    termrequest_sent(&rcs_status);
	    need_flush = TRUE;
	}

	// Only request the cursor blink mode if t_RC set. Not
	// for Gnome terminal, it can't handle t_RC, it
	// echoes the characters to the screen.
	// Only when getting the cursor style was detected to work.
	if (rbm_status.tr_progress == STATUS_GET
		&& term_props[TPR_CURSOR_BLINK].tpr_status == TPR_YES
		&& *T_CRC != NUL)
	{
	    MAY_WANT_TO_LOG_THIS;
	    LOG_TR(("Sending cursor blink mode request"));
	    out_str(T_CRC);
	    termrequest_sent(&rbm_status);
	    need_flush = TRUE;
	}

	if (need_flush)
	    out_flush();
#endif
    }
}

/*
 * Add "key" to "buf" and return the number of bytes used.
 * Handles special keys and multi-byte characters.
 */
    static int
add_key_to_buf(int key, char_u *buf)
{
    int idx = 0;

    if (IS_SPECIAL(key))
    {
	buf[idx++] = K_SPECIAL;
	buf[idx++] = KEY2TERMCAP0(key);
	buf[idx++] = KEY2TERMCAP1(key);
    }
    else if (has_mbyte)
	idx += (*mb_char2bytes)(key, buf + idx);
    else
	buf[idx++] = key;
    return idx;
}

/*
 * Shared between handle_key_with_modifier() and handle_csi_function_key().
 */
    static int
put_key_modifiers_in_typebuf(
	int	key_arg,
	int	modifiers_arg,
	int	csi_len,
	int	offset,
	char_u	*buf,
	int	bufsize,
	int	*buflen)
{
    int key = key_arg;
    int modifiers = modifiers_arg;

    // Some keys need adjustment when the Ctrl modifier is used.
    key = may_adjust_key_for_ctrl(modifiers, key);

    // May remove the shift modifier if it's already included in the key.
    modifiers = may_remove_shift_modifier(modifiers, key);

    // Produce modifiers with K_SPECIAL KS_MODIFIER {mod}
    char_u string[MAX_KEY_CODE_LEN + 1];
    int new_slen = modifiers2keycode(modifiers, &key, string);

    // Add the bytes for the key.
    new_slen += add_key_to_buf(key, string + new_slen);

    string[new_slen] = NUL;
    if (put_string_in_typebuf(offset, csi_len, string, new_slen,
						 buf, bufsize, buflen) == FAIL)
	return -1;
    return new_slen - csi_len + offset;
}

/*
 * Handle a sequence with key and modifier, one of:
 *	{lead}27;{modifier};{key}~
 *	{lead}{key};{modifier}u
 * Returns the difference in length.
 */
    static int
handle_key_with_modifier(
	int	*arg,
	int	trail,
	int	csi_len,
	int	offset,
	char_u	*buf,
	int	bufsize,
	int	*buflen)
{
    // Only set seenModifyOtherKeys for the "{lead}27;" code to avoid setting
    // it for terminals using the kitty keyboard protocol.  Xterm sends
    // the form ending in "u" when the formatOtherKeys resource is set.  We do
    // not support this.
    //
    // Do not set seenModifyOtherKeys if there was a positive response at any
    // time from requesting the kitty keyboard protocol state, these are not
    // expected to support modifyOtherKeys level 2.
    //
    // Do not set seenModifyOtherKeys for kitty, it does send some sequences
    // like this but does not have the modifyOtherKeys feature.
    if (trail != 'u'
	    && (kitty_protocol_state == KKPS_INITIAL
		|| kitty_protocol_state == KKPS_OFF
		|| kitty_protocol_state == KKPS_AFTER_T_TE)
	    && term_props[TPR_KITTY].tpr_status != TPR_YES)
    {
#ifdef FEAT_EVAL
	ch_log(NULL, "setting seenModifyOtherKeys to TRUE");
#endif
	seenModifyOtherKeys = TRUE;
    }

    int key = trail == 'u' ? arg[0] : arg[2];
    int modifiers = decode_modifiers(arg[1]);

    // Some terminals do not apply the Shift modifier to the key.  To make
    // mappings consistent we do it here.  TODO: support more keys.
    if ((modifiers & MOD_MASK_SHIFT) && key >= 'a' && key <= 'z')
	key += 'A' - 'a';

    // Putting Esc in the buffer creates ambiguity, it can be the start of an
    // escape sequence.  Use K_ESC to avoid that.
    if (key == ESC)
	key = K_ESC;

    return put_key_modifiers_in_typebuf(key, modifiers,
					csi_len, offset, buf, bufsize, buflen);
}

/*
 * Handle a sequence with key without a modifier:
 *	{lead}{key}u
 * Returns the difference in length.
 */
    static int
handle_key_without_modifier(
	int	*arg,
	int	csi_len,
	int	offset,
	char_u	*buf,
	int	bufsize,
	int	*buflen)
{
    char_u  string[MAX_KEY_CODE_LEN + 1];
    int	    new_slen;

    if (arg[0] == ESC)
    {
	// Putting Esc in the buffer creates ambiguity, it can be the start of
	// an escape sequence.  Use K_ESC to avoid that.
	string[0] = K_SPECIAL;
	string[1] = KS_EXTRA;
	string[2] = KE_ESC;
	new_slen = 3;
    }
    else
	new_slen = add_key_to_buf(arg[0], string);

    if (put_string_in_typebuf(offset, csi_len, string, new_slen,
						 buf, bufsize, buflen) == FAIL)
	return -1;
    return new_slen - csi_len + offset;
}

/*
 * CSI function key without or with modifiers:
 *	{lead}[ABCDEFHPQRS]
 *	{lead}1;{modifier}[ABCDEFHPQRS]
 * Returns zero when nog recognized, a positive number when recognized.
 */
    static int
handle_csi_function_key(
	int	argc,
	int	*arg,
	int	trail,
	int	csi_len,
	char_u	*key_name,
	int	offset,
	char_u  *buf,
	int	bufsize,
	int	*buflen)
{
    key_name[0] = 'k';
    switch (trail)
    {
	case 'A': key_name[1] = 'u'; break;  // K_UP
	case 'B': key_name[1] = 'd'; break;  // K_DOWN
	case 'C': key_name[1] = 'r'; break;  // K_RIGHT
	case 'D': key_name[1] = 'l'; break;  // K_LEFT

	// case 'E': keypad BEGIN - not supported
	case 'F': key_name[0] = '@'; key_name[1] = '7'; break;  // K_END
	case 'H': key_name[1] = 'h'; break;  // K_HOME

	case 'P': key_name[1] = '1'; break;  // K_F1
	case 'Q': key_name[1] = '2'; break;  // K_F2
	case 'R': key_name[1] = '3'; break;  // K_F3
	case 'S': key_name[1] = '4'; break;  // K_F4

	default: return 0;  // not recognized
    }

    int key = TERMCAP2KEY(key_name[0], key_name[1]);
    int modifiers = argc == 2 ? decode_modifiers(arg[1]) : 0;
    put_key_modifiers_in_typebuf(key, modifiers,
					csi_len, offset, buf, bufsize, buflen);
    return csi_len;
}

/*
 * Handle a CSI escape sequence.
 * - Xterm version string.
 *
 * - Response to XTQMODKEYS: "{lead} > 4 ; Pv m".
 *
 * - Cursor position report: {lead}{row};{col}R
 *   The final byte must be 'R'. It is used for checking the
 *   ambiguous-width character state.
 *
 * - window position reply: {lead}3;{x};{y}t
 *
 * - key with modifiers when modifyOtherKeys is enabled or the Kitty keyboard
 *   protocol is used:
 *	    {lead}27;{modifier};{key}~
 *	    {lead}{key};{modifier}u
 *
 * - function key with or without modifiers:
 *	{lead}[ABCDEFHPQRS]
 *	{lead}1;{modifier}[ABCDEFHPQRS]
 *
 * Return 0 for no match, -1 for partial match, > 0 for full match.
 */
    static int
handle_csi(
	char_u	*tp,
	int	len,
	char_u	*argp,
	int	offset,
	char_u  *buf,
	int	bufsize,
	int	*buflen,
	char_u	*key_name,
	int	*slen)
{
    int		first = -1;  // optional char right after {lead}
    int		trail;	     // char that ends CSI sequence
    int		arg[3] = {-1, -1, -1};	// argument numbers
    int		argc = 0;		// number of arguments
    char_u	*ap = argp;
    int		csi_len;

    // Check for non-digit after CSI.
    if (!VIM_ISDIGIT(*ap))
	first = *ap++;

    if (first >= 'A' && first <= 'Z')
    {
	// If "first" is in [ABCDEFHPQRS] then it is actually the "trail" and
	// no argument follows.
	trail = first;
	first = -1;
	--ap;
    }
    else
    {
	// Find up to three argument numbers.
	for (argc = 0; argc < 3; )
	{
	    if (ap >= tp + len)
		return -1;
	    if (*ap == ';')
		arg[argc++] = -1;  // omitted number
	    else if (VIM_ISDIGIT(*ap))
	    {
		arg[argc] = 0;
		for (;;)
		{
		    if (ap >= tp + len)
			return -1;
		    if (!VIM_ISDIGIT(*ap))
			break;
		    arg[argc] = arg[argc] * 10 + (*ap - '0');
		    ++ap;
		}
		++argc;
	    }
	    if (*ap == ';')
		++ap;
	    else
		break;
	}

	// mrxvt has been reported to have "+" in the version. Assume
	// the escape sequence ends with a letter or one of "{|}~".
	while (ap < tp + len
		&& !(*ap >= '{' && *ap <= '~')
		&& !ASCII_ISALPHA(*ap))
	    ++ap;
	if (ap >= tp + len)
	    return -1;
	trail = *ap;
    }

    csi_len = (int)(ap - tp) + 1;

    // Response to XTQMODKEYS: "CSI > 4 ; Pv m" where Pv indicates the
    // modifyOtherKeys level.  Drop similar responses.
    if (first == '>' && (argc == 1 || argc == 2) && trail == 'm')
    {
	if (arg[0] == 4 && argc == 2)
	    modify_otherkeys_state = arg[1] == 2 ? MOKS_ENABLED : MOKS_OFF;

	key_name[0] = (int)KS_EXTRA;
	key_name[1] = (int)KE_IGNORE;
	*slen = csi_len;
    }

    // Function key starting with CSI:
    //	{lead}[ABCDEFHPQRS]
    //	{lead}1;{modifier}[ABCDEFHPQRS]
    else if (first == -1 && ASCII_ISUPPER(trail)
	    && (argc == 0 || (argc == 2 && arg[0] == 1)))
    {
	int res = handle_csi_function_key(argc, arg, trail,
			      csi_len, key_name, offset, buf, bufsize, buflen);
	return res <= 0 ? res : len + res;
    }

    // Cursor position report: {lead}{row};{col}R
    // Eat it when there are 2 arguments and it ends in 'R'.
    // Also when u7_status is not "sent", it may be from a previous Vim that
    // just exited.  But not for <S-F3>, it sends something similar, check for
    // row and column to make sense.
    else if (first == -1 && argc == 2 && trail == 'R')
    {
	handle_u7_response(arg, tp, csi_len);

	key_name[0] = (int)KS_EXTRA;
	key_name[1] = (int)KE_IGNORE;
	*slen = csi_len;
    }

    // Version string: Eat it when there is at least one digit and
    // it ends in 'c'
    else if (*T_CRV != NUL && ap > argp + 1 && trail == 'c')
    {
	handle_version_response(first, arg, argc, tp);

	*slen = csi_len;
#ifdef FEAT_EVAL
	set_vim_var_string(VV_TERMRESPONSE, tp, *slen);
#endif
	apply_autocmds(EVENT_TERMRESPONSE,
					NULL, NULL, FALSE, curbuf);
	apply_autocmds(EVENT_TERMRESPONSEALL,
					(char_u *)"version", NULL, FALSE, curbuf);
	key_name[0] = (int)KS_EXTRA;
	key_name[1] = (int)KE_IGNORE;
    }

#ifdef FEAT_TERMRESPONSE
    // Check blinking cursor from xterm:
    // {lead}?12;1$y       set
    // {lead}?12;2$y       not set
    //
    // {lead} can be <Esc>[ or CSI
    else if (rbm_status.tr_progress == STATUS_SENT
	    && first == '?'
	    && ap == argp + 6
	    && arg[0] == 12
	    && ap[-1] == '$'
	    && trail == 'y')
    {
	initial_cursor_blink = (arg[1] == '1');
	rbm_status.tr_progress = STATUS_GOT;
	LOG_TR(("Received cursor blinking mode response: %s", tp));
	key_name[0] = (int)KS_EXTRA;
	key_name[1] = (int)KE_IGNORE;
	*slen = csi_len;
# ifdef FEAT_EVAL
	set_vim_var_string(VV_TERMBLINKRESP, tp, *slen);
# endif
	apply_autocmds(EVENT_TERMRESPONSEALL,
					(char_u *)"cursorblink", NULL, FALSE, curbuf);
    }
#endif

    // Kitty keyboard protocol status response: CSI ? flags u
    else if (first == '?' && argc == 1 && trail == 'u')
    {
	// The protocol has various "progressive enhancement flags" values, but
	// we only check for zero and non-zero here.
	if (arg[0] == '0')
	{
	    kitty_protocol_state = KKPS_OFF;
	}
	else
	{
	    kitty_protocol_state = KKPS_ENABLED;

	    // Reset seenModifyOtherKeys just in case some key combination has
	    // been seen that set it before we get the status response.
#ifdef FEAT_EVAL
	    ch_log(NULL, "setting seenModifyOtherKeys to FALSE");
#endif
	    seenModifyOtherKeys = FALSE;
	}

	key_name[0] = (int)KS_EXTRA;
	key_name[1] = (int)KE_IGNORE;
	*slen = csi_len;
    }

#ifdef FEAT_TERMRESPONSE
    // Check for a window position response from the terminal:
    //       {lead}3;{x};{y}t
    else if (did_request_winpos && argc == 3 && arg[0] == 3
						   && trail == 't')
    {
	winpos_x = arg[1];
	winpos_y = arg[2];
	// got finished code: consume it
	key_name[0] = (int)KS_EXTRA;
	key_name[1] = (int)KE_IGNORE;
	*slen = csi_len;

	if (--did_request_winpos <= 0)
	    winpos_status.tr_progress = STATUS_GOT;
    }
#endif

    // Key with modifier:
    //	{lead}27;{modifier};{key}~
    //	{lead}{key};{modifier}u
    // Even though we only handle four modifiers and the {modifier} value
    // should be 16 or lower, we accept all modifier values to avoid the raw
    // sequence to be passed through.
    else if ((arg[0] == 27 && argc == 3 && trail == '~')
		|| (argc == 2 && trail == 'u'))
    {
	return len + handle_key_with_modifier(arg, trail,
					csi_len, offset, buf, bufsize, buflen);
    }

    // Key without modifier (Kitty sends this for Esc):
    //	{lead}{key}u
    else if (argc == 1 && trail == 'u')
    {
	return len + handle_key_without_modifier(arg,
			    csi_len, offset, buf, bufsize, buflen);
    }

    // else: Unknown CSI sequence.  We could drop it, but then the
    // user can't create a map for it.
    return 0;
}

/*
 * Handle an OSC sequence, fore/background color response from the terminal:
 *
 *       {lead}{code};rgb:{rrrr}/{gggg}/{bbbb}{tail}
 * or    {lead}{code};rgb:{rr}/{gg}/{bb}{tail}
 *
 * {code} is 10 for foreground, 11 for background
 * {lead} can be <Esc>] or OSC
 * {tail} can be '\007', <Esc>\ or STERM.
 *
 * Consume any code that starts with "{lead}11;", it's also
 * possible that "rgba" is following.
 */
    static int
handle_osc(char_u *tp, char_u *argp, int len, char_u *key_name, int *slen)
{
    int		i, j;

    j = 1 + (tp[0] == ESC);
    if (len >= j + 3 && (argp[0] != '1'
			     || (argp[1] != '1' && argp[1] != '0')
			     || argp[2] != ';'))
	i = 0; // no match
    else
	for (i = j; i < len; ++i)
	    if (tp[i] == '\007' || (tp[0] == OSC ? tp[i] == STERM
			: (tp[i] == ESC && i + 1 < len && tp[i + 1] == '\\')))
	    {
		int is_bg = argp[1] == '1';
		int is_4digit = i - j >= 21 && tp[j + 11] == '/'
						  && tp[j + 16] == '/';

		if (i - j >= 15 && STRNCMP(tp + j + 3, "rgb:", 4) == 0
			    && (is_4digit
				   || (tp[j + 9] == '/' && tp[j + 12] == '/')))
		{
		    char_u *tp_r = tp + j + 7;
		    char_u *tp_g = tp + j + (is_4digit ? 12 : 10);
		    char_u *tp_b = tp + j + (is_4digit ? 17 : 13);
#if defined(FEAT_TERMRESPONSE) && defined(FEAT_TERMINAL)
		    int rval, gval, bval;

		    rval = hexhex2nr(tp_r);
		    gval = hexhex2nr(tp_b);
		    bval = hexhex2nr(tp_g);
#endif
		    if (is_bg)
		    {
			char *new_bg_val = (3 * '6' < *tp_r + *tp_g +
					     *tp_b) ? "light" : "dark";

			LOG_TR(("Received RBG response: %s", tp));
#ifdef FEAT_TERMRESPONSE
			rbg_status.tr_progress = STATUS_GOT;
# ifdef FEAT_TERMINAL
			bg_r = rval;
			bg_g = gval;
			bg_b = bval;
# endif
#endif
			if (!option_was_set((char_u *)"bg")
				      && STRCMP(p_bg, new_bg_val) != 0)
			{
			    // value differs, apply it
			    set_option_value_give_err((char_u *)"bg",
						  0L, (char_u *)new_bg_val, 0);
			    reset_option_was_set((char_u *)"bg");
			    redraw_asap(UPD_CLEAR);
			}
		    }
#if defined(FEAT_TERMRESPONSE) && defined(FEAT_TERMINAL)
		    else
		    {
			LOG_TR(("Received RFG response: %s", tp));
			rfg_status.tr_progress = STATUS_GOT;
			fg_r = rval;
			fg_g = gval;
			fg_b = bval;
		    }
#endif
		}

		// got finished code: consume it
		key_name[0] = (int)KS_EXTRA;
		key_name[1] = (int)KE_IGNORE;
		*slen = i + 1 + (tp[i] == ESC);
#ifdef FEAT_EVAL
		set_vim_var_string(is_bg ? VV_TERMRBGRESP
						  : VV_TERMRFGRESP, tp, *slen);
#endif
		apply_autocmds(EVENT_TERMRESPONSEALL,
			    is_bg ? (char_u *)"background" : (char_u *)"foreground", NULL, FALSE, curbuf);
		break;
	    }
    if (i == len)
    {
	LOG_TR(("not enough characters for RB"));
	return FAIL;
    }
    return OK;
}

/*
 * Check for key code response from xterm:
 * {lead}{flag}+r<hex bytes><{tail}
 *
 * {lead} can be <Esc>P or DCS
 * {flag} can be '0' or '1'
 * {tail} can be Esc>\ or STERM
 *
 * Check for resource response from xterm (and drop it):
 * {lead}{flag}+R<hex bytes>=<value>{tail}
 *
 * Check for cursor shape response from xterm:
 * {lead}1$r<digit> q{tail}
 *
 * {lead} can be <Esc>P or DCS
 * {tail} can be <Esc>\ or STERM
 *
 * Consume any code that starts with "{lead}.+r" or "{lead}.$r".
 */
    static int
handle_dcs(char_u *tp, char_u *argp, int len, char_u *key_name, int *slen)
{
    int i, j;

    j = 1 + (tp[0] == ESC);
    if (len < j + 3)
	i = len; // need more chars
    else if ((argp[1] != '+' && argp[1] != '$')
	    || (argp[2] != 'r' && argp[2] != 'R'))
	i = 0; // no match
    else if (argp[1] == '+')
	// key code response
	for (i = j; i < len; ++i)
	{
	    if ((tp[i] == ESC && i + 1 < len && tp[i + 1] == '\\')
		    || tp[i] == STERM)
	    {
#ifdef FEAT_TERMRESPONSE
		// handle a key code response, drop a resource response
		if (i - j >= 3 && argp[2] == 'r')
		    got_code_from_term(tp + j, i);
#endif
		key_name[0] = (int)KS_EXTRA;
		key_name[1] = (int)KE_IGNORE;
		*slen = i + 1 + (tp[i] == ESC);
		break;
	    }
	}
    else
    {
	// Probably the cursor shape response.  Make sure that "i"
	// is equal to "len" when there are not sufficient
	// characters.
	for (i = j + 3; i < len; ++i)
	{
	    if (i - j == 3 && !SAFE_isdigit(tp[i]))
		break;
	    if (i - j == 4 && tp[i] != ' ')
		break;
	    if (i - j == 5 && tp[i] != 'q')
		break;
	    if (i - j == 6 && tp[i] != ESC && tp[i] != STERM)
		break;
	    if ((i - j == 6 && tp[i] == STERM)
		    || (i - j == 7 && tp[i] == '\\'))
	    {
#ifdef FEAT_TERMRESPONSE
		int number = argp[3] - '0';

		// 0, 1 = block blink, 2 = block
		// 3 = underline blink, 4 = underline
		// 5 = vertical bar blink, 6 = vertical bar
		number = number == 0 ? 1 : number;
		initial_cursor_shape = (number + 1) / 2;
		// The blink flag is actually inverted, compared to
		// the value set with T_SH.
		initial_cursor_shape_blink =
				       (number & 1) ? FALSE : TRUE;
		rcs_status.tr_progress = STATUS_GOT;
#endif
		LOG_TR(("Received cursor shape response: %s", tp));

		key_name[0] = (int)KS_EXTRA;
		key_name[1] = (int)KE_IGNORE;
		*slen = i + 1;
#ifdef FEAT_EVAL
		set_vim_var_string(VV_TERMSTYLERESP, tp, *slen);
#endif
		apply_autocmds(EVENT_TERMRESPONSEALL,
					(char_u *)"cursorshape", NULL, FALSE, curbuf);
		break;
	    }
	}
    }

    if (i == len)
    {
	// These codes arrive many together, each code can be
	// truncated at any point.
	LOG_TR(("not enough characters for XT"));
	return FAIL;
    }
    return OK;
}

/*
 * Check if typebuf.tb_buf[] contains a terminal key code.
 * Check from typebuf.tb_buf[typebuf.tb_off] to typebuf.tb_buf[typebuf.tb_off
 * + "max_offset"].
 * Return 0 for no match, -1 for partial match, > 0 for full match.
 * Return KEYLEN_REMOVED when a key code was deleted.
 * With a match, the match is removed, the replacement code is inserted in
 * typebuf.tb_buf[] and the number of characters in typebuf.tb_buf[] is
 * returned.
 * When "buf" is not NULL, buf[bufsize] is used instead of typebuf.tb_buf[].
 * "buflen" is then the length of the string in buf[] and is updated for
 * inserts and deletes.
 */
    int
check_termcode(
    int		max_offset,
    char_u	*buf,
    int		bufsize,
    int		*buflen)
{
    char_u	*tp;
    char_u	*p;
    int		slen = 0;	// init for GCC
    int		modslen;
    int		len;
    int		retval = 0;
    int		offset;
    char_u	key_name[2];
    int		modifiers;
    char_u	*modifiers_start = NULL;
    int		key;
    int		new_slen;   // Length of what will replace the termcode
    char_u	string[MAX_KEY_CODE_LEN + 1];
    int		i, j;
    int		idx = 0;
    int		cpo_koffset;

    cpo_koffset = (vim_strchr(p_cpo, CPO_KOFFSET) != NULL);

    /*
     * Speed up the checks for terminal codes by gathering all first bytes
     * used in termleader[].  Often this is just a single <Esc>.
     */
    if (need_gather)
	gather_termleader();

    /*
     * Check at several positions in typebuf.tb_buf[], to catch something like
     * "x<Up>" that can be mapped. Stop at max_offset, because characters
     * after that cannot be used for mapping, and with @r commands
     * typebuf.tb_buf[] can become very long.
     * This is used often, KEEP IT FAST!
     */
    for (offset = 0; offset < max_offset; ++offset)
    {
	if (buf == NULL)
	{
	    if (offset >= typebuf.tb_len)
		break;
	    tp = typebuf.tb_buf + typebuf.tb_off + offset;
	    len = typebuf.tb_len - offset;	// length of the input
	}
	else
	{
	    if (offset >= *buflen)
		break;
	    tp = buf + offset;
	    len = *buflen - offset;
	}

	/*
	 * Don't check characters after K_SPECIAL, those are already
	 * translated terminal chars (avoid translating ~@^Hx).
	 */
	if (*tp == K_SPECIAL)
	{
	    offset += 2;	// there are always 2 extra characters
	    continue;
	}

	/*
	 * Skip this position if the character does not appear as the first
	 * character in term_strings. This speeds up a lot, since most
	 * termcodes start with the same character (ESC or CSI).
	 */
	i = *tp;
	for (p = termleader; *p && *p != i; ++p)
	    ;
	if (*p == NUL)
	    continue;

	/*
	 * Skip this position if p_ek is not set and tp[0] is an ESC and we
	 * are in Insert mode.
	 */
	if (*tp == ESC && !p_ek && (State & MODE_INSERT))
	    continue;

	tp[len] = NUL;
	key_name[0] = NUL;	// no key name found yet
	key_name[1] = NUL;	// no key name found yet
	modifiers = 0;		// no modifiers yet

#ifdef FEAT_GUI
	if (gui.in_use)
	{
	    /*
	     * GUI special key codes are all of the form [CSI xx].
	     */
	    if (*tp == CSI)	    // Special key from GUI
	    {
		if (len < 3)
		    return -1;	    // Shouldn't happen
		slen = 3;
		key_name[0] = tp[1];
		key_name[1] = tp[2];
	    }
	}
	else
#endif // FEAT_GUI
#ifdef MSWIN
	    if (len >= 3 && tp[0] == CSI && tp[1] == KS_EXTRA
		    && (tp[2] == KE_MOUSEUP
			|| tp[2] == KE_MOUSEDOWN
			|| tp[2] == KE_MOUSELEFT
			|| tp[2] == KE_MOUSERIGHT))
	{
	    // MS-Windows console sends mouse scroll events encoded:
	    // - CSI
	    // - KS_EXTRA
	    // - {KE_MOUSE[UP|DOWN|LEFT|RIGHT]}
	    slen = 3;
	    key_name[0] = tp[1];
	    key_name[1] = tp[2];
	}
	else
#endif
	{
	    int  mouse_index_found = -1;

	    for (idx = 0; idx < tc_len; ++idx)
	    {
		/*
		 * Ignore the entry if we are not at the start of
		 * typebuf.tb_buf[]
		 * and there are not enough characters to make a match.
		 * But only when the 'K' flag is in 'cpoptions'.
		 */
		slen = termcodes[idx].len;
		modifiers_start = NULL;
		if (cpo_koffset && offset && len < slen)
		    continue;
		if (STRNCMP(termcodes[idx].code, tp,
				     (size_t)(slen > len ? len : slen)) == 0)
		{
		    int	    looks_like_mouse_start = FALSE;

		    if (len < slen)		// got a partial sequence
			return -1;		// need to get more chars

		    /*
		     * When found a keypad key, check if there is another key
		     * that matches and use that one.  This makes <Home> to be
		     * found instead of <kHome> when they produce the same
		     * key code.
		     */
		    if (termcodes[idx].name[0] == 'K'
				       && VIM_ISDIGIT(termcodes[idx].name[1]))
		    {
			for (j = idx + 1; j < tc_len; ++j)
			    if (termcodes[j].len == slen &&
				    STRNCMP(termcodes[idx].code,
					    termcodes[j].code, slen) == 0)
			    {
				idx = j;
				break;
			    }
		    }

		    if (slen == 2 && len > 2
			    && termcodes[idx].code[0] == ESC
			    && termcodes[idx].code[1] == '[')
		    {
			// The mouse termcode "ESC [" is also the prefix of
			// "ESC [ I" (focus gained) and other keys.  Check some
			// more bytes to find out.
			if (!SAFE_isdigit(tp[2]))
			{
			    // ESC [ without number following: Only use it when
			    // there is no other match.
			    looks_like_mouse_start = TRUE;
			}
			else if (termcodes[idx].name[0] == KS_DEC_MOUSE)
			{
			    char_u  *nr = tp + 2;
			    int	    count = 0;

			    // If a digit is following it could be a key with
			    // modifier, e.g., ESC [ 1;2P.  Can be confused
			    // with DEC_MOUSE, which requires four numbers
			    // following.  If not then it can't be a DEC_MOUSE
			    // code.
			    for (;;)
			    {
				++count;
				(void)getdigits(&nr);
				if (nr >= tp + len)
				    return -1;	// partial sequence
				if (*nr != ';')
				    break;
				++nr;
				if (nr >= tp + len)
				    return -1;	// partial sequence
			    }
			    if (count < 4)
				continue;	// no match
			}
		    }
		    if (looks_like_mouse_start)
		    {
			// Only use it when there is no other match.
			if (mouse_index_found < 0)
			    mouse_index_found = idx;
		    }
		    else
		    {
			key_name[0] = termcodes[idx].name[0];
			key_name[1] = termcodes[idx].name[1];
			break;
		    }
		}

		/*
		 * Check for code with modifier, like xterm uses:
		 * <Esc>[123;*X  (modslen == slen - 3)
		 * <Esc>[@;*X    (matches <Esc>[X and <Esc>[1;9X )
		 * Also <Esc>O*X and <M-O>*X (modslen == slen - 2).
		 * When there is a modifier the * matches a number.
		 * When there is no modifier the ;* or * is omitted.
		 */
		if (termcodes[idx].modlen > 0 && mouse_index_found < 0)
		{
		    modslen = termcodes[idx].modlen;
		    if (cpo_koffset && offset && len < modslen)
			continue;
		    if (STRNCMP(termcodes[idx].code, tp,
				(size_t)(modslen > len ? len : modslen)) == 0)
		    {
			int	    n;

			if (len <= modslen)	// got a partial sequence
			    return -1;		// need to get more chars

			if (tp[modslen] == termcodes[idx].code[slen - 1])
			    // no modifiers
			    slen = modslen + 1;
			else if (tp[modslen] != ';' && modslen == slen - 3)
			    // no match for "code;*X" with "code;"
			    continue;
			else if (termcodes[idx].code[modslen] == '@'
					 && (tp[modslen] != '1'
						    || tp[modslen + 1] != ';'))
			    // no match for "<Esc>[@" with "<Esc>[1;"
			    continue;
			else
			{
			    // Skip over the digits, the final char must
			    // follow. URXVT can use a negative value, thus
			    // also accept '-'.
			    for (j = slen - 2; j < len && (SAFE_isdigit(tp[j])
				       || tp[j] == '-' || tp[j] == ';'); ++j)
				;
			    ++j;
			    if (len < j)	// got a partial sequence
				return -1;	// need to get more chars
			    if (tp[j - 1] != termcodes[idx].code[slen - 1])
				continue;	// no match

			    modifiers_start = tp + slen - 2;

			    // Match!  Convert modifier bits.
			    n = atoi((char *)modifiers_start);
			    modifiers |= decode_modifiers(n);

			    slen = j;
			}
			key_name[0] = termcodes[idx].name[0];
			key_name[1] = termcodes[idx].name[1];
			break;
		    }
		}
	    }
	    if (idx == tc_len && mouse_index_found >= 0)
	    {
		key_name[0] = termcodes[mouse_index_found].name[0];
		key_name[1] = termcodes[mouse_index_found].name[1];
	    }
	}

	if (key_name[0] == NUL
	    // Mouse codes of DEC and pterm start with <ESC>[.  When
	    // detecting the start of these mouse codes they might as well be
	    // another key code or terminal response.
#ifdef FEAT_MOUSE_DEC
	    || key_name[0] == KS_DEC_MOUSE
#endif
#ifdef FEAT_MOUSE_PTERM
	    || key_name[0] == KS_PTERM_MOUSE
#endif
	   )
	{
	    char_u *argp = tp[0] == ESC ? tp + 2 : tp + 1;

	    /*
	     * Check for responses from the terminal starting with {lead}:
	     * "<Esc>[" or CSI followed by [0-9>?].
	     * Also for function keys without a modifier:
	     * "<Esc>[" or CSI followed by [ABCDEFHPQRS].
	     *
	     * - Xterm version string: {lead}>{x};{vers};{y}c
	     *   Also eat other possible responses to t_RV, rxvt returns
	     *   "{lead}?1;2c".
	     *
	     * - Response to XTQMODKEYS: "{lead} > 4 ; Pv m".
	     *
	     * - Cursor position report: {lead}{row};{col}R
	     *   The final byte must be 'R'. It is used for checking the
	     *   ambiguous-width character state.
	     *
	     * - window position reply: {lead}3;{x};{y}t
	     *
	     * - key with modifiers when modifyOtherKeys is enabled:
	     *	    {lead}27;{modifier};{key}~
	     *	    {lead}{key};{modifier}u
	     */
	    if (((tp[0] == ESC && len >= 3 && tp[1] == '[')
			|| (tp[0] == CSI && len >= 2))
		    && vim_strchr((char_u *)"0123456789>?ABCDEFHPQRS",
								*argp) != NULL)
	    {
		int resp = handle_csi(tp, len, argp, offset, buf,
					     bufsize, buflen, key_name, &slen);
		if (resp != 0)
		{
#ifdef DEBUG_TERMRESPONSE
		    if (resp == -1)
			LOG_TR(("Not enough characters for CSI sequence"));
#endif
		    return resp;
		}
	    }

	    // Check for fore/background color response from the terminal,
	    // starting} with <Esc>] or OSC
	    else if ((*T_RBG != NUL || *T_RFG != NUL)
			&& ((tp[0] == ESC && len >= 2 && tp[1] == ']')
			    || tp[0] == OSC))
	    {
		if (handle_osc(tp, argp, len, key_name, &slen) == FAIL)
		    return -1;
	    }

	    // Check for key code response from xterm,
	    // starting with <Esc>P or DCS
	    // It would only be needed with this condition:
	    //	    (check_for_codes || rcs_status.tr_progress == STATUS_SENT)
	    // Now this is always done so that DCS codes don't mess up things.
	    else if ((tp[0] == ESC && len >= 2 && tp[1] == 'P') || tp[0] == DCS)
	    {
		if (handle_dcs(tp, argp, len, key_name, &slen) == FAIL)
		    return -1;
	    }
	}

	if (key_name[0] == NUL)
	    continue;	    // No match at this position, try next one

	// We only get here when we have a complete termcode match

#if defined(FEAT_GUI) || defined(MSWIN)
	/*
	 * For scroll events from the GUI or MS-Windows console, fetch the
	 * pointer coordinates so that we know which window to scroll later.
	 */
	if (TRUE
# if defined(FEAT_GUI) && !defined(MSWIN)
		&& gui.in_use
# endif
		&& key_name[0] == (int)KS_EXTRA
		&& (key_name[1] == (int)KE_X1MOUSE
		    || key_name[1] == (int)KE_X2MOUSE
		    || key_name[1] == (int)KE_MOUSEMOVE_XY
		    || key_name[1] == (int)KE_MOUSELEFT
		    || key_name[1] == (int)KE_MOUSERIGHT
		    || key_name[1] == (int)KE_MOUSEDOWN
		    || key_name[1] == (int)KE_MOUSEUP))
	{
	    char_u	bytes[6];
	    int		num_bytes = get_bytes_from_buf(tp + slen, bytes, 4);

	    if (num_bytes == -1)	// not enough coordinates
		return -1;
	    mouse_col = 128 * (bytes[0] - ' ' - 1) + bytes[1] - ' ' - 1;
	    mouse_row = 128 * (bytes[2] - ' ' - 1) + bytes[3] - ' ' - 1;
	    slen += num_bytes;
	    // equal to K_MOUSEMOVE
	    if (key_name[1] == (int)KE_MOUSEMOVE_XY)
		key_name[1] = (int)KE_MOUSEMOVE;
	}
	else
#endif
	/*
	 * If it is a mouse click, get the coordinates.
	 */
	if (key_name[0] == KS_MOUSE
#ifdef FEAT_MOUSE_GPM
		|| key_name[0] == KS_GPM_MOUSE
#endif
#ifdef FEAT_MOUSE_JSB
		|| key_name[0] == KS_JSBTERM_MOUSE
#endif
#ifdef FEAT_MOUSE_NET
		|| key_name[0] == KS_NETTERM_MOUSE
#endif
#ifdef FEAT_MOUSE_DEC
		|| key_name[0] == KS_DEC_MOUSE
#endif
#ifdef FEAT_MOUSE_PTERM
		|| key_name[0] == KS_PTERM_MOUSE
#endif
#ifdef FEAT_MOUSE_URXVT
		|| key_name[0] == KS_URXVT_MOUSE
#endif
		|| key_name[0] == KS_SGR_MOUSE
		|| key_name[0] == KS_SGR_MOUSE_RELEASE)
	{
	    if (check_termcode_mouse(tp, &slen, key_name, modifiers_start, idx,
							     &modifiers) == -1)
		return -1;
	}

#ifdef FEAT_GUI
	/*
	 * If using the GUI, then we get menu and scrollbar events.
	 *
	 * A menu event is encoded as K_SPECIAL, KS_MENU, KE_FILLER followed by
	 * four bytes which are to be taken as a pointer to the vimmenu_T
	 * structure.
	 *
	 * A tab line event is encoded as K_SPECIAL KS_TABLINE nr, where "nr"
	 * is one byte with the tab index.
	 *
	 * A scrollbar event is K_SPECIAL, KS_VER_SCROLLBAR, KE_FILLER followed
	 * by one byte representing the scrollbar number, and then four bytes
	 * representing a long_u which is the new value of the scrollbar.
	 *
	 * A horizontal scrollbar event is K_SPECIAL, KS_HOR_SCROLLBAR,
	 * KE_FILLER followed by four bytes representing a long_u which is the
	 * new value of the scrollbar.
	 */
# ifdef FEAT_MENU
	else if (key_name[0] == (int)KS_MENU)
	{
	    long_u	val;
	    int		num_bytes = get_long_from_buf(tp + slen, &val);

	    if (num_bytes == -1)
		return -1;
	    current_menu = (vimmenu_T *)val;
	    slen += num_bytes;

	    // The menu may have been deleted right after it was used, check
	    // for that.
	    if (check_menu_pointer(root_menu, current_menu) == FAIL)
	    {
		key_name[0] = KS_EXTRA;
		key_name[1] = (int)KE_IGNORE;
	    }
	}
# endif
# ifdef FEAT_GUI_TABLINE
	else if (key_name[0] == (int)KS_TABLINE)
	{
	    // Selecting tabline tab or using its menu.
	    char_u	bytes[6];
	    int		num_bytes = get_bytes_from_buf(tp + slen, bytes, 1);

	    if (num_bytes == -1)
		return -1;
	    current_tab = (int)bytes[0];
	    if (current_tab == 255)	// -1 in a byte gives 255
		current_tab = -1;
	    slen += num_bytes;
	}
	else if (key_name[0] == (int)KS_TABMENU)
	{
	    // Selecting tabline tab or using its menu.
	    char_u	bytes[6];
	    int		num_bytes = get_bytes_from_buf(tp + slen, bytes, 2);

	    if (num_bytes == -1)
		return -1;
	    current_tab = (int)bytes[0];
	    current_tabmenu = (int)bytes[1];
	    slen += num_bytes;
	}
# endif
# ifndef USE_ON_FLY_SCROLL
	else if (key_name[0] == (int)KS_VER_SCROLLBAR)
	{
	    long_u	val;
	    char_u	bytes[6];
	    int		num_bytes;

	    // Get the last scrollbar event in the queue of the same type
	    j = 0;
	    for (i = 0; tp[j] == CSI && tp[j + 1] == KS_VER_SCROLLBAR
						     && tp[j + 2] != NUL; ++i)
	    {
		j += 3;
		num_bytes = get_bytes_from_buf(tp + j, bytes, 1);
		if (num_bytes == -1)
		    break;
		if (i == 0)
		    current_scrollbar = (int)bytes[0];
		else if (current_scrollbar != (int)bytes[0])
		    break;
		j += num_bytes;
		num_bytes = get_long_from_buf(tp + j, &val);
		if (num_bytes == -1)
		    break;
		scrollbar_value = val;
		j += num_bytes;
		slen = j;
	    }
	    if (i == 0)		// not enough characters to make one
		return -1;
	}
	else if (key_name[0] == (int)KS_HOR_SCROLLBAR)
	{
	    long_u	val;
	    int		num_bytes;

	    // Get the last horiz. scrollbar event in the queue
	    j = 0;
	    for (i = 0; tp[j] == CSI && tp[j + 1] == KS_HOR_SCROLLBAR
						     && tp[j + 2] != NUL; ++i)
	    {
		j += 3;
		num_bytes = get_long_from_buf(tp + j, &val);
		if (num_bytes == -1)
		    break;
		scrollbar_value = val;
		j += num_bytes;
		slen = j;
	    }
	    if (i == 0)		// not enough characters to make one
		return -1;
	}
# endif // !USE_ON_FLY_SCROLL
#endif // FEAT_GUI

#if defined(UNIX) || defined(VMS)
	/*
	 * Handle FocusIn/FocusOut event sequences reported by XTerm.
	 * (CSI I/CSI O)
	 */
	if (key_name[0] == KS_EXTRA
# ifdef FEAT_GUI
		&& !gui.in_use
# endif
	    )
	{
	    if (key_name[1] == KE_FOCUSGAINED)
	    {
		if (!focus_state)
		{
		    ui_focus_change(TRUE);
		    did_cursorhold = TRUE;
		    focus_state = TRUE;
		}
		key_name[1] = (int)KE_IGNORE;
	    }
	    else if (key_name[1] == KE_FOCUSLOST)
	    {
		if (focus_state)
		{
		    ui_focus_change(FALSE);
		    did_cursorhold = TRUE;
		    focus_state = FALSE;
		}
		key_name[1] = (int)KE_IGNORE;
	    }
	}
#endif

	/*
	 * Change <xHome> to <Home>, <xUp> to <Up>, etc.
	 */
	key = handle_x_keys(TERMCAP2KEY(key_name[0], key_name[1]));

	/*
	 * Add any modifier codes to our string.
	 */
	new_slen = modifiers2keycode(modifiers, &key, string);

	// Finally, add the special key code to our string
	key_name[0] = KEY2TERMCAP0(key);
	key_name[1] = KEY2TERMCAP1(key);
	if (key_name[0] == KS_KEY)
	{
	    // from ":set <M-b>=xx"
	    if (has_mbyte)
		new_slen += (*mb_char2bytes)(key_name[1], string + new_slen);
	    else
		string[new_slen++] = key_name[1];
	}
	else if (new_slen == 0 && key_name[0] == KS_EXTRA
						  && key_name[1] == KE_IGNORE)
	{
	    // Do not put K_IGNORE into the buffer, do return KEYLEN_REMOVED
	    // to indicate what happened.
	    retval = KEYLEN_REMOVED;
	}
	else
	{
	    string[new_slen++] = K_SPECIAL;
	    string[new_slen++] = key_name[0];
	    string[new_slen++] = key_name[1];
	}
	if (put_string_in_typebuf(offset, slen, string, new_slen,
						 buf, bufsize, buflen) == FAIL)
	    return -1;
	return retval == 0 ? (len + new_slen - slen + offset) : retval;
    }

#ifdef FEAT_TERMRESPONSE
    LOG_TR(("normal character"));
#endif

    return 0;			    // no match found
}

#if (defined(FEAT_TERMINAL) && defined(FEAT_TERMRESPONSE)) || defined(PROTO)
/*
 * Get the text foreground color, if known.
 */
    void
term_get_fg_color(char_u *r, char_u *g, char_u *b)
{
    if (rfg_status.tr_progress != STATUS_GOT)
	return;

    *r = fg_r;
    *g = fg_g;
    *b = fg_b;
}

/*
 * Get the text background color, if known.
 */
    void
term_get_bg_color(char_u *r, char_u *g, char_u *b)
{
    if (rbg_status.tr_progress != STATUS_GOT)
	return;

    *r = bg_r;
    *g = bg_g;
    *b = bg_b;
}
#endif

/*
 * Replace any terminal code strings in from[] with the equivalent internal
 * vim representation.	This is used for the "from" and "to" part of a
 * mapping, and the "to" part of a menu command.
 * Any strings like "<C-UP>" are also replaced, unless 'cpoptions' contains
 * '<'.
 * K_SPECIAL by itself is replaced by K_SPECIAL KS_SPECIAL KE_FILLER.
 *
 * The replacement is done in result[] and finally copied into allocated
 * memory. If this all works well *bufp is set to the allocated memory and a
 * pointer to it is returned. If something fails *bufp is set to NULL and from
 * is returned.
 *
 * CTRL-V characters are removed.  When "flags" has REPTERM_FROM_PART, a
 * trailing CTRL-V is included, otherwise it is removed (for ":map xx ^V", maps
 * xx to nothing).  When 'cpoptions' does not contain 'B', a backslash can be
 * used instead of a CTRL-V.
 *
 * Flags:
 *  REPTERM_FROM_PART	see above
 *  REPTERM_DO_LT	also translate <lt>
 *  REPTERM_SPECIAL	always accept <key> notation
 *  REPTERM_NO_SIMPLIFY	do not simplify <C-H> to 0x08 and set 8th bit for <A-x>
 *
 * "did_simplify" is set when some <C-H> or <A-x> code was simplified, unless
 * it is NULL.
 */
    char_u *
replace_termcodes(
    char_u	*from,
    char_u	**bufp,
    scid_T	sid_arg UNUSED,	// script ID to use for <SID>,
				// or 0 to use current_sctx
    int		flags,
    int		*did_simplify)
{
    int		i;
    int		slen;
    int		key;
    size_t	dlen = 0;
    char_u	*src;
    int		do_backslash;	// backslash is a special character
    int		do_special;	// recognize <> key codes
    int		do_key_code;	// recognize raw key codes
    char_u	*result;	// buffer for resulting string
    garray_T	ga;

    do_backslash = (vim_strchr(p_cpo, CPO_BSLASH) == NULL);
    do_special = (vim_strchr(p_cpo, CPO_SPECI) == NULL)
						  || (flags & REPTERM_SPECIAL);
    do_key_code = (vim_strchr(p_cpo, CPO_KEYCODE) == NULL);
    src = from;

    /*
     * Allocate space for the translation.  Worst case a single character is
     * replaced by 6 bytes (shifted special key), plus a NUL at the end.
     * In the rare case more might be needed ga_grow() must be called again.
     */
    ga_init2(&ga, 1L, 100);
    if (ga_grow(&ga, (int)(STRLEN(src) * 6 + 1)) == FAIL) // out of memory
    {
	*bufp = NULL;
	return from;
    }
    result = ga.ga_data;

    /*
     * Check for #n at start only: function key n
     */
    if ((flags & REPTERM_FROM_PART) && src[0] == '#' && VIM_ISDIGIT(src[1]))
    {
	result[dlen++] = K_SPECIAL;
	result[dlen++] = 'k';
	if (src[1] == '0')
	    result[dlen++] = ';';	// #0 is F10 is "k;"
	else
	    result[dlen++] = src[1];	// #3 is F3 is "k3"
	src += 2;
    }

    /*
     * Copy each byte from *from to result[dlen]
     */
    while (*src != NUL)
    {
	/*
	 * If 'cpoptions' does not contain '<', check for special key codes,
	 * like "<C-S-LeftMouse>"
	 */
	if (do_special && ((flags & REPTERM_DO_LT)
					      || STRNCMP(src, "<lt>", 4) != 0))
	{
#ifdef FEAT_EVAL
	    /*
	     * Change <SID>Func to K_SNR <script-nr> _Func.  This name is used
	     * for script-local user functions.
	     * (room: 5 * 6 = 30 bytes; needed: 3 + <nr> + 1 <= 14)
	     * Also change <SID>name.Func to K_SNR <import-script-nr> _Func.
	     * Only if "name" is recognized as an import.
	     */
	    if (STRNICMP(src, "<SID>", 5) == 0)
	    {
		if (sid_arg < 0 || (sid_arg == 0 && current_sctx.sc_sid <= 0))
		    emsg(_(e_using_sid_not_in_script_context));
		else
		{
		    char_u  *dot;
		    long    sid = sid_arg != 0 ? sid_arg : current_sctx.sc_sid;

		    src += 5;
		    if (in_vim9script()
				       && (dot = vim_strchr(src, '.')) != NULL)
		    {
			imported_T *imp = find_imported(src, dot - src, FALSE);

			if (imp != NULL)
			{
			    scriptitem_T    *si = SCRIPT_ITEM(imp->imp_sid);
			    size_t	    len;

			    src = dot + 1;
			    if (si->sn_autoload_prefix != NULL)
			    {
				// Turn "<SID>name.Func"
				// into "scriptname#Func".
				len = STRLEN(si->sn_autoload_prefix);
				if (ga_grow(&ga,
				     (int)(STRLEN(src) * 6 + len + 1)) == FAIL)
				{
				    ga_clear(&ga);
				    *bufp = NULL;
				    return from;
				}
				result = ga.ga_data;
				STRCPY(result + dlen, si->sn_autoload_prefix);
				dlen += len;
				continue;
			    }
			    sid = imp->imp_sid;
			}
		    }

		    result[dlen++] = K_SPECIAL;
		    result[dlen++] = (int)KS_EXTRA;
		    result[dlen++] = (int)KE_SNR;
		    sprintf((char *)result + dlen, "%ld", sid);
		    dlen += STRLEN(result + dlen);
		    result[dlen++] = '_';
		    continue;
		}
	    }
#endif
	    int fsk_flags = FSK_KEYCODE
			| ((flags & REPTERM_NO_SIMPLIFY) ? 0 : FSK_SIMPLIFY)
			| ((flags & REPTERM_FROM_PART) ? FSK_FROM_PART : 0);
	    slen = trans_special(&src, result + dlen, fsk_flags,
							   TRUE, did_simplify);
	    if (slen > 0)
	    {
		dlen += slen;
		continue;
	    }
	}

	/*
	 * If 'cpoptions' does not contain 'k', see if it's an actual key-code.
	 * Note that this is also checked after replacing the <> form.
	 * Single character codes are NOT replaced (e.g. ^H or DEL), because
	 * it could be a character in the file.
	 */
	if (do_key_code)
	{
	    i = find_term_bykeys(src);
	    if (i >= 0)
	    {
		result[dlen++] = K_SPECIAL;
		result[dlen++] = termcodes[i].name[0];
		result[dlen++] = termcodes[i].name[1];
		src += termcodes[i].len;
		// If terminal code matched, continue after it.
		continue;
	    }
	}

#ifdef FEAT_EVAL
	if (do_special)
	{
	    char_u	*p, *s, len;

	    /*
	     * Replace <Leader> by the value of "mapleader".
	     * Replace <LocalLeader> by the value of "maplocalleader".
	     * If "mapleader" or "maplocalleader" isn't set use a backslash.
	     */
	    if (STRNICMP(src, "<Leader>", 8) == 0)
	    {
		len = 8;
		p = get_var_value((char_u *)"g:mapleader");
	    }
	    else if (STRNICMP(src, "<LocalLeader>", 13) == 0)
	    {
		len = 13;
		p = get_var_value((char_u *)"g:maplocalleader");
	    }
	    else
	    {
		len = 0;
		p = NULL;
	    }
	    if (len != 0)
	    {
		// Allow up to 8 * 6 characters for "mapleader".
		if (p == NULL || *p == NUL || STRLEN(p) > 8 * 6)
		    s = (char_u *)"\\";
		else
		    s = p;
		while (*s != NUL)
		    result[dlen++] = *s++;
		src += len;
		continue;
	    }
	}
#endif

	/*
	 * Remove CTRL-V and ignore the next character.
	 * For "from" side the CTRL-V at the end is included, for the "to"
	 * part it is removed.
	 * If 'cpoptions' does not contain 'B', also accept a backslash.
	 */
	key = *src;
	if (key == Ctrl_V || (do_backslash && key == '\\'))
	{
	    ++src;				// skip CTRL-V or backslash
	    if (*src == NUL)
	    {
		if (flags & REPTERM_FROM_PART)
		    result[dlen++] = key;
		break;
	    }
	}

	// skip multibyte char correctly
	for (i = (*mb_ptr2len)(src); i > 0; --i)
	{
	    /*
	     * If the character is K_SPECIAL, replace it with K_SPECIAL
	     * KS_SPECIAL KE_FILLER.
	     * If compiled with the GUI replace CSI with K_CSI.
	     */
	    if (*src == K_SPECIAL)
	    {
		result[dlen++] = K_SPECIAL;
		result[dlen++] = KS_SPECIAL;
		result[dlen++] = KE_FILLER;
	    }
# ifdef FEAT_GUI
	    else if (*src == CSI)
	    {
		result[dlen++] = K_SPECIAL;
		result[dlen++] = KS_EXTRA;
		result[dlen++] = (int)KE_CSI;
	    }
# endif
	    else
		result[dlen++] = *src;
	    ++src;
	}
    }
    result[dlen] = NUL;

    /*
     * Copy the new string to allocated memory.
     * If this fails, just return from.
     */
    if ((*bufp = vim_strsave(result)) != NULL)
	from = *bufp;
    vim_free(result);
    return from;
}

/*
 * Find a termcode with keys 'src' (must be NUL terminated).
 * Return the index in termcodes[], or -1 if not found.
 */
    static int
find_term_bykeys(char_u *src)
{
    int		i;
    int		slen = (int)STRLEN(src);

    for (i = 0; i < tc_len; ++i)
    {
	if (slen == termcodes[i].len
			&& STRNCMP(termcodes[i].code, src, (size_t)slen) == 0)
	    return i;
    }
    return -1;
}

/*
 * Gather the first characters in the terminal key codes into a string.
 * Used to speed up check_termcode().
 */
    static void
gather_termleader(void)
{
    int	    i;
    int	    len = 0;

#ifdef FEAT_GUI
    if (gui.in_use)
	termleader[len++] = CSI;    // the GUI codes are not in termcodes[]
#endif
#ifdef FEAT_TERMRESPONSE
    if (check_for_codes || *T_CRS != NUL)
	termleader[len++] = DCS;    // the termcode response starts with DCS
				    // in 8-bit mode
#endif
    termleader[len] = NUL;

    for (i = 0; i < tc_len; ++i)
	if (vim_strchr(termleader, termcodes[i].code[0]) == NULL)
	{
	    termleader[len++] = termcodes[i].code[0];
	    termleader[len] = NUL;
	}

    need_gather = FALSE;
}

/*
 * Show all termcodes (for ":set termcap")
 * This code looks a lot like showoptions(), but is different.
 * "flags" can have OPT_ONECOLUMN.
 */
    void
show_termcodes(int flags)
{
    int		col;
    int		*items;
    int		item_count;
    int		run;
    int		row, rows;
    int		cols;
    int		i;
    int		len;

#define INC3 27	    // try to make three columns
#define INC2 40	    // try to make two columns
#define GAP 2	    // spaces between columns

    if (tc_len == 0)	    // no terminal codes (must be GUI)
	return;
    items = ALLOC_MULT(int, tc_len);
    if (items == NULL)
	return;

    // Highlight title
    msg_puts_title(_("\n--- Terminal keys ---"));

    /*
     * Do the loop three times:
     * 1. display the short items (non-strings and short strings)
     * 2. display the medium items (medium length strings)
     * 3. display the long items (remaining strings)
     * When "flags" has OPT_ONECOLUMN do everything in 3.
     */
    for (run = (flags & OPT_ONECOLUMN) ? 3 : 1; run <= 3 && !got_int; ++run)
    {
	/*
	 * collect the items in items[]
	 */
	item_count = 0;
	for (i = 0; i < tc_len; i++)
	{
	    len = show_one_termcode(termcodes[i].name,
						    termcodes[i].code, FALSE);
	    if ((flags & OPT_ONECOLUMN) ||
		    (len <= INC3 - GAP ? run == 1
			: len <= INC2 - GAP ? run == 2
			: run == 3))
		items[item_count++] = i;
	}

	/*
	 * display the items
	 */
	if (run <= 2)
	{
	    cols = (Columns + GAP) / (run == 1 ? INC3 : INC2);
	    if (cols == 0)
		cols = 1;
	    rows = (item_count + cols - 1) / cols;
	}
	else	// run == 3
	    rows = item_count;
	for (row = 0; row < rows && !got_int; ++row)
	{
	    msg_putchar('\n');			// go to next line
	    if (got_int)			// 'q' typed in more
		break;
	    col = 0;
	    for (i = row; i < item_count; i += rows)
	    {
		msg_col = col;			// make columns
		show_one_termcode(termcodes[items[i]].name,
					      termcodes[items[i]].code, TRUE);
		if (run == 2)
		    col += INC2;
		else
		    col += INC3;
	    }
	    out_flush();
	    ui_breakcheck();
	}
    }
    vim_free(items);
}

/*
 * Show one termcode entry.
 * Output goes into IObuff[]
 */
    int
show_one_termcode(char_u *name, char_u *code, int printit)
{
    char_u	*p;
    int		len;

    if (name[0] > '~')
    {
	IObuff[0] = ' ';
	IObuff[1] = ' ';
	IObuff[2] = ' ';
	IObuff[3] = ' ';
    }
    else
    {
	IObuff[0] = 't';
	IObuff[1] = '_';
	IObuff[2] = name[0];
	IObuff[3] = name[1];
    }
    IObuff[4] = ' ';

    p = get_special_key_name(TERMCAP2KEY(name[0], name[1]), 0);
    if (p[1] != 't')
	STRCPY(IObuff + 5, p);
    else
	IObuff[5] = NUL;
    len = (int)STRLEN(IObuff);
    do
	IObuff[len++] = ' ';
    while (len < 17);
    IObuff[len] = NUL;
    if (code == NULL)
	len += 4;
    else
	len += vim_strsize(code);

    if (printit)
    {
	msg_puts((char *)IObuff);
	if (code == NULL)
	    msg_puts("NULL");
	else
	    msg_outtrans(code);
    }
    return len;
}

#if defined(FEAT_TERMRESPONSE) || defined(PROTO)
/*
 * For Xterm >= 140 compiled with OPT_TCAP_QUERY: Obtain the actually used
 * termcap codes from the terminal itself.
 * We get them one by one to avoid a very long response string.
 */
static int xt_index_in = 0;
static int xt_index_out = 0;

    static void
req_codes_from_term(void)
{
    xt_index_out = 0;
    xt_index_in = 0;
    req_more_codes_from_term();
}

    static void
req_more_codes_from_term(void)
{
    char	buf[23];  // extra size to shut up LGTM
    int		old_idx = xt_index_out;

    // Don't do anything when going to exit.
    if (exiting)
	return;

    // Send up to 10 more requests out than we received.  Avoid sending too
    // many, there can be a buffer overflow somewhere.
    while (xt_index_out < xt_index_in + 10 && key_names[xt_index_out] != NULL)
    {
	char *key_name = key_names[xt_index_out];

	MAY_WANT_TO_LOG_THIS;
	LOG_TR(("Requesting XT %d: %s", xt_index_out, key_name));
	sprintf(buf, "\033P+q%02x%02x\033\\", key_name[0], key_name[1]);
	out_str_nf((char_u *)buf);
	++xt_index_out;
    }

    // Send the codes out right away.
    if (xt_index_out != old_idx)
	out_flush();
}

/*
 * Decode key code response from xterm: '<Esc>P1+r<name>=<string><Esc>\'.
 * A "0" instead of the "1" indicates a code that isn't supported.
 * Both <name> and <string> are encoded in hex.
 * "code" points to the "0" or "1".
 */
    static void
got_code_from_term(char_u *code, int len)
{
#define XT_LEN 100
    char_u	name[3];
    char_u	str[XT_LEN];
    int		i;
    int		j = 0;
    int		c;

    // A '1' means the code is supported, a '0' means it isn't.
    // When half the length is > XT_LEN we can't use it.
    // Our names are currently all 2 characters.
    if (code[0] == '1' && code[7] == '=' && len / 2 < XT_LEN)
    {
	// Get the name from the response and find it in the table.
	name[0] = hexhex2nr(code + 3);
	name[1] = hexhex2nr(code + 5);
	name[2] = NUL;
	for (i = 0; key_names[i] != NULL; ++i)
	{
	    if (STRCMP(key_names[i], name) == 0)
	    {
		xt_index_in = i;
		break;
	    }
	}

	LOG_TR(("Received XT %d: %s", xt_index_in, (char *)name));

	if (key_names[i] != NULL)
	{
	    for (i = 8; (c = hexhex2nr(code + i)) >= 0; i += 2)
		str[j++] = c;
	    str[j] = NUL;
	    if (name[0] == 'C' && name[1] == 'o')
	    {
		// Color count is not a key code.
		int val = atoi((char *)str);
#if defined(FEAT_EVAL)
		if (val == t_colors)
		    ch_log(NULL, "got_code_from_term(Co): no change (%d)", val);
		else
		    ch_log(NULL,
			       "got_code_from_term(Co): changed from %d to %d",
								t_colors, val);
#endif
		may_adjust_color_count(val);
	    }
	    else
	    {
		i = find_term_bykeys(str);
		if (i >= 0 && name[0] == termcodes[i].name[0]
					    && name[1] == termcodes[i].name[1])
		{
		    // Existing entry with the same name and code - skip.
#ifdef FEAT_EVAL
		    ch_log(NULL, "got_code_from_term(): Entry %c%c did not change",
							     name[0], name[1]);
#endif
		}
		else
		{
		    if (i >= 0)
		    {
			// Delete an existing entry using the same code.
#ifdef FEAT_EVAL
			ch_log(NULL, "got_code_from_term(): Deleting entry %c%c with matching keys %s",
			      termcodes[i].name[0], termcodes[i].name[1], str);
#endif
			del_termcode_idx(i);
		    }
#ifdef FEAT_EVAL
		    else
			ch_log(NULL, "got_code_from_term(): Adding entry %c%c with keys %s",
							name[0], name[1], str);
#endif
		    add_termcode(name, str, ATC_FROM_TERM);
		}
	    }
	}
    }

    // May request more codes now that we received one.
    ++xt_index_in;
    req_more_codes_from_term();
}

/*
 * Check if there are any unanswered requests and deal with them.
 * This is called before starting an external program or getting direct
 * keyboard input.  We don't want responses to be send to that program or
 * handled as typed text.
 */
    static void
check_for_codes_from_term(void)
{
    int		c;

    // If no codes requested or all are answered, no need to wait.
    if (xt_index_out == 0 || xt_index_out == xt_index_in)
	return;

    // Vgetc() will check for and handle any response.
    // Keep calling vpeekc() until we don't get any responses.
    ++no_mapping;
    ++allow_keys;
    for (;;)
    {
	c = vpeekc();
	if (c == NUL)	    // nothing available
	    break;

	// If a response is recognized it's replaced with K_IGNORE, must read
	// it from the input stream.  If there is no K_IGNORE we can't do
	// anything, break here (there might be some responses further on, but
	// we don't want to throw away any typed chars).
	if (c != K_SPECIAL && c != K_IGNORE)
	    break;
	c = vgetc();
	if (c != K_IGNORE)
	{
	    vungetc(c);
	    break;
	}
    }
    --no_mapping;
    --allow_keys;
}
#endif

#if (defined(MSWIN) && (!defined(FEAT_GUI) || defined(VIMDLL))) || defined(PROTO)
static char ksme_str[20];
static char ksmr_str[20];
static char ksmd_str[20];

/*
 * For Win32 console: update termcap codes for existing console attributes.
 */
    void
update_tcap(int attr)
{
    sprintf(ksme_str, "\033|%dm", attr);
    sprintf(ksmd_str, "\033|%dm", attr | 0x08);  // FOREGROUND_INTENSITY
    sprintf(ksmr_str, "\033|%dm", ((attr & 0x0F) << 4) | ((attr & 0xF0) >> 4));

    tcap_entry_T *p = find_builtin_term(DEFAULT_TERM);
    if (p == NULL)  // did not find it
	return;
    while (p->bt_string != NULL)
    {
      if (p->bt_entry == (int)KS_ME)
	  p->bt_string = &ksme_str[0];
      else if (p->bt_entry == (int)KS_MR)
	  p->bt_string = &ksmr_str[0];
      else if (p->bt_entry == (int)KS_MD)
	  p->bt_string = &ksmd_str[0];
      ++p;
    }
}

# ifdef FEAT_TERMGUICOLORS
#  define KSSIZE 20

typedef enum
{
    CMODE_INDEXED = 0,	// Use cmd.exe 4bit palette.
    CMODE_RGB,		// Use 24bit RGB colors using VTP.
    CMODE_256COL,	// Emulate xterm's 256-color palette using VTP.
    CMODE_LAST,
} cmode_T;

struct ks_tbl_S
{
    int  code;				// value of KS_
    char *vtp;				// code in RGB mode
    char *vtp2;				// code in 256color mode
    char buf[CMODE_LAST][KSSIZE];	// real buffer
};

static struct ks_tbl_S ks_tbl[] =
{
    {(int)KS_ME,  "\033|0m",  "\033|0m", {""}},   // normal
    {(int)KS_MR,  "\033|7m",  "\033|7m", {""}},   // reverse
    {(int)KS_MD,  "\033|1m",  "\033|1m", {""}},   // bold
    {(int)KS_SO,  "\033|91m", "\033|91m", {""}},  // standout: bright red text
    {(int)KS_SE,  "\033|39m", "\033|39m", {""}},  // standout end: default color
    {(int)KS_CZH, "\033|3m",  "\033|3m", {""}},   // italic
    {(int)KS_CZR, "\033|0m",  "\033|0m", {""}},   // italic end
    {(int)KS_US,  "\033|4m",  "\033|4m", {""}},   // underscore
    {(int)KS_UE,  "\033|24m", "\033|24m", {""}},  // underscore end
#  ifdef TERMINFO
    {(int)KS_CAB, "\033|%p1%db", "\033|%p14%dm", {""}}, // set background color
    {(int)KS_CAF, "\033|%p1%df", "\033|%p13%dm", {""}}, // set foreground color
    {(int)KS_CS,  "\033|%p1%d;%p2%dR", "\033|%p1%d;%p2%dR", {""}},
    {(int)KS_CSV, "\033|%p1%d;%p2%dV", "\033|%p1%d;%p2%dV", {""}},
#  else
    {(int)KS_CAB, "\033|%db", "\033|4%dm", {""}}, // set background color
    {(int)KS_CAF, "\033|%df", "\033|3%dm", {""}}, // set foreground color
    {(int)KS_CS,  "\033|%d;%dR", "\033|%d;%dR", {""}},
    {(int)KS_CSV, "\033|%d;%dV", "\033|%d;%dV", {""}},
#  endif
    {(int)KS_CCO, "256", "256", {""}},	    // colors
    {(int)KS_NAME, NULL, NULL, {""}}			    // terminator
};

/*
 * Find the first entry for "code" in the builtin termcap for "name".
 * Returns NULL when not found.
 */
    static tcap_entry_T *
find_first_tcap(
    char_u *name,
    int	    code)
{
    tcap_entry_T *p = find_builtin_term(name);
    if (p == NULL)
	return NULL;
    while (p->bt_string != NULL)
    {
	if (p->bt_entry == code)
	    return p;
	++p;
    }
    return NULL;
}
# endif

/*
 * For Win32 console: replace the sequence immediately after termguicolors.
 */
    void
swap_tcap(void)
{
# ifdef FEAT_TERMGUICOLORS
    static int		init_done = FALSE;
    static cmode_T	curr_mode;
    struct ks_tbl_S	*ks;
    cmode_T		mode;

    if (!init_done)
    {
	for (ks = ks_tbl; ks->code != (int)KS_NAME; ks++)
	{
	    tcap_entry_T *bt = find_first_tcap(DEFAULT_TERM, ks->code);
	    if (bt != NULL)
	    {
		// Preserve the original value.
		STRNCPY(ks->buf[CMODE_INDEXED], bt->bt_string, KSSIZE);
		STRNCPY(ks->buf[CMODE_RGB], ks->vtp, KSSIZE);
		STRNCPY(ks->buf[CMODE_256COL], ks->vtp2, KSSIZE);

		bt->bt_string = ks->buf[CMODE_INDEXED];
	    }
	}
	init_done = TRUE;
	curr_mode = CMODE_INDEXED;
    }

    if (p_tgc)
	mode = CMODE_RGB;
    else if (t_colors >= 256)
	mode = CMODE_256COL;
    else
	mode = CMODE_INDEXED;

    if (mode == curr_mode)
	return;

    for (ks = ks_tbl; ks->code != (int)KS_NAME; ks++)
    {
	tcap_entry_T *bt = find_first_tcap(DEFAULT_TERM, ks->code);
	if (bt != NULL)
	    bt->bt_string = ks->buf[mode];
    }

    curr_mode = mode;
# endif
}

#endif


#if (defined(MSWIN) && (!defined(FEAT_GUI_MSWIN) || defined(VIMDLL))) || defined(FEAT_TERMINAL) \
	|| defined(PROTO)
static int cube_value[] = {
    0x00, 0x5F, 0x87, 0xAF, 0xD7, 0xFF
};

static int grey_ramp[] = {
    0x08, 0x12, 0x1C, 0x26, 0x30, 0x3A, 0x44, 0x4E, 0x58, 0x62, 0x6C, 0x76,
    0x80, 0x8A, 0x94, 0x9E, 0xA8, 0xB2, 0xBC, 0xC6, 0xD0, 0xDA, 0xE4, 0xEE
};

static const char_u ansi_table[16][3] = {
//   R    G    B
  {  0,   0,   0}, // black
  {224,   0,   0}, // dark red
  {  0, 224,   0}, // dark green
  {224, 224,   0}, // dark yellow / brown
  {  0,   0, 224}, // dark blue
  {224,   0, 224}, // dark magenta
  {  0, 224, 224}, // dark cyan
  {224, 224, 224}, // light grey

  {128, 128, 128}, // dark grey
  {255,  64,  64}, // light red
  { 64, 255,  64}, // light green
  {255, 255,  64}, // yellow
  { 64,  64, 255}, // light blue
  {255,  64, 255}, // light magenta
  { 64, 255, 255}, // light cyan
  {255, 255, 255}, // white
};

#if defined(MSWIN)
// Mapping between cterm indices < 16 and their counterpart in the ANSI palette.
static const char_u cterm_ansi_idx[] = {
    0, 4, 2, 6, 1, 5, 3, 7, 8, 12, 10, 14, 9, 13, 11, 15
};
#endif

#define ANSI_INDEX_NONE 0

    void
ansi_color2rgb(int nr, char_u *r, char_u *g, char_u *b, char_u *ansi_idx)
{
    if (nr < 16)
    {
	*r = ansi_table[nr][0];
	*g = ansi_table[nr][1];
	*b = ansi_table[nr][2];
	*ansi_idx = nr;
    }
    else
    {
	*r = 0;
	*g = 0;
	*b = 0;
	*ansi_idx = ANSI_INDEX_NONE;
    }
}

    void
cterm_color2rgb(int nr, char_u *r, char_u *g, char_u *b, char_u *ansi_idx)
{
    int idx;

    if (nr < 16)
    {
#if defined(MSWIN)
	idx = cterm_ansi_idx[nr];
#else
	idx = nr;
#endif
	*r = ansi_table[idx][0];
	*g = ansi_table[idx][1];
	*b = ansi_table[idx][2];
	*ansi_idx = idx + 1;
    }
    else if (nr < 232)
    {
	// 216 color cube
	idx = nr - 16;
	*r = cube_value[idx / 36 % 6];
	*g = cube_value[idx / 6  % 6];
	*b = cube_value[idx      % 6];
	*ansi_idx = ANSI_INDEX_NONE;
    }
    else if (nr < 256)
    {
	// 24 grey scale ramp
	idx = nr - 232;
	*r = grey_ramp[idx];
	*g = grey_ramp[idx];
	*b = grey_ramp[idx];
	*ansi_idx = ANSI_INDEX_NONE;
    }
    else
    {
	*r = 0;
	*g = 0;
	*b = 0;
	*ansi_idx = ANSI_INDEX_NONE;
    }
}
#endif

/*
 * Replace K_BS by <BS> and K_DEL by <DEL>.
 * Include any modifiers into the key and drop them.
 * Returns "len" adjusted for replaced codes.
 */
    int
term_replace_keycodes(char_u *ta_buf, int ta_len, int len_arg)
{
    int		len = len_arg;
    int		i;
    int		c;

    for (i = ta_len; i < ta_len + len; ++i)
    {
	if (ta_buf[i] == CSI && len - i > 3 && ta_buf[i + 1] == KS_MODIFIER)
	{
	    int modifiers = ta_buf[i + 2];
	    int key = ta_buf[i + 3];

	    // Try to use the modifier to modify the key.  In any case drop the
	    // modifier.
	    mch_memmove(ta_buf + i + 1, ta_buf + i + 4, (size_t)(len - i - 3));
	    len -= 3;
	    if (key < 0x80)
		key = merge_modifyOtherKeys(key, &modifiers);
	    ta_buf[i] = key;
	}
	else if (ta_buf[i] == CSI && len - i > 2)
	{
	    c = TERMCAP2KEY(ta_buf[i + 1], ta_buf[i + 2]);
	    if (c == K_DEL || c == K_KDEL || c == K_BS)
	    {
		mch_memmove(ta_buf + i + 1, ta_buf + i + 3,
							(size_t)(len - i - 2));
		if (c == K_DEL || c == K_KDEL)
		    ta_buf[i] = DEL;
		else
		    ta_buf[i] = Ctrl_H;
		len -= 2;
	    }
	}
	else if (ta_buf[i] == '\r')
	    ta_buf[i] = '\n';
	if (has_mbyte)
	    i += (*mb_ptr2len_len)(ta_buf + i, ta_len + len - i) - 1;
    }
    return len;
}
