/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar et al.
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 * This file defines the Normal mode commands.
 */

/*
 * When adding a Normal/Visual mode command:
 * 1. Add an entry in the table `nv_cmds[]` below.
 * 2. Run "make nvcmdidxs" to re-generate nv_cmdidxs.h.
 * 3. Add an entry in the index for Normal/Visual commands at
 *    ":help normal-index" and ":help visual-index" .
 * 4. Add documentation in ../doc/xxx.txt.  Add a tag for both the short and
 *    long name of the command.
 */

#ifdef DO_DECLARE_NVCMD

/*
 * Used when building Vim.
 */
# define NVCMD(a, b, c, d) \
	{a, b, c, d}

#ifdef FEAT_GUI
#define NV_VER_SCROLLBAR	nv_ver_scrollbar
#define NV_HOR_SCROLLBAR	nv_hor_scrollbar
#else
#define NV_VER_SCROLLBAR nv_error
#define NV_HOR_SCROLLBAR nv_error
#endif

#ifdef FEAT_GUI_TABLINE
#define NV_TABLINE	nv_tabline
#define NV_TABMENU	nv_tabmenu
#else
#define NV_TABLINE	nv_error
#define NV_TABMENU	nv_error
#endif

#ifdef FEAT_NETBEANS_INTG
#define NV_NBCMD	nv_nbcmd
#else
#define NV_NBCMD	nv_error
#endif

#ifdef FEAT_DND
#define NV_DROP		nv_drop
#else
#define NV_DROP		nv_error
#endif

/*
 * Function to be called for a Normal or Visual mode command.
 * The argument is a cmdarg_T.
 */
typedef void (*nv_func_T)(cmdarg_T *cap);

// Values for cmd_flags.
#define NV_NCH	    0x01	  // may need to get a second char
#define NV_NCH_NOP  (0x02|NV_NCH) // get second char when no operator pending
#define NV_NCH_ALW  (0x04|NV_NCH) // always get a second char
#define NV_LANG	    0x08	// second char needs language adjustment

#define NV_SS	    0x10	// may start selection
#define NV_SSS	    0x20	// may start selection with shift modifier
#define NV_STS	    0x40	// may stop selection without shift modif.
#define NV_RL	    0x80	// 'rightleft' modifies command
#define NV_KEEPREG  0x100	// don't clear regname
#define NV_NCW	    0x200	// not allowed in command-line window

/*
 * Generally speaking, every Normal mode command should either clear any
 * pending operator (with *clearop*()), or set the motion type variable
 * oap->motion_type.
 *
 * When a cursor motion command is made, it is marked as being a character or
 * line oriented motion.  Then, if an operator is in effect, the operation
 * becomes character or line oriented accordingly.
 */

/*
 * This table contains one entry for every Normal or Visual mode command.
 * The order doesn't matter, this will be sorted by the create_nvcmdidx.vim
 * script to generate the nv_cmd_idx[] lookup table.
 * It is faster when all keys from zero to '~' are present.
 */
static const struct nv_cmd
{
    int		cmd_char;	// (first) command character
    nv_func_T   cmd_func;	// function for this command
    short_u	cmd_flags;	// NV_ flags
    short	cmd_arg;	// value for ca.arg
} nv_cmds[] =

#else  // DO_DECLARE_NVCMD

/*
 * Used when creating nv_cmdidxs.h.
 */
# define NVCMD(a, b, c, d)  a
static const int nv_cmds[] =

#endif // DO_DECLARE_NVCMD
{
    NVCMD(NUL,		nv_error,	0,			0),
    NVCMD(Ctrl_A,	nv_addsub,	0,			0),
    NVCMD(Ctrl_B,	nv_page,	NV_STS,			BACKWARD),
    NVCMD(Ctrl_C,	nv_esc,		0,			TRUE),
    NVCMD(Ctrl_D,	nv_halfpage,	0,			0),
    NVCMD(Ctrl_E,	nv_scroll_line,	0,			TRUE),
    NVCMD(Ctrl_F,	nv_page,	NV_STS,			FORWARD),
    NVCMD(Ctrl_G,	nv_ctrlg,	0,			0),
    NVCMD(Ctrl_H,	nv_ctrlh,	0,			0),
    NVCMD(Ctrl_I,	nv_pcmark,	0,			0),
    NVCMD(NL,		nv_down,	0,			FALSE),
    NVCMD(Ctrl_K,	nv_error,	0,			0),
    NVCMD(Ctrl_L,	nv_clear,	0,			0),
    NVCMD(CAR,		nv_down,	0,			TRUE),
    NVCMD(Ctrl_N,	nv_down,	NV_STS,			FALSE),
    NVCMD(Ctrl_O,	nv_ctrlo,	0,			0),
    NVCMD(Ctrl_P,	nv_up,		NV_STS,			FALSE),
    NVCMD(Ctrl_Q,	nv_visual,	0,			FALSE),
    NVCMD(Ctrl_R,	nv_redo_or_register, 0,			0),
    NVCMD(Ctrl_S,	nv_ignore,	0,			0),
    NVCMD(Ctrl_T,	nv_tagpop,	NV_NCW,			0),
    NVCMD(Ctrl_U,	nv_halfpage,	0,			0),
    NVCMD(Ctrl_V,	nv_visual,	0,			FALSE),
    NVCMD(Ctrl_W,	nv_window,	0,			0),
    NVCMD(Ctrl_X,	nv_addsub,	0,			0),
    NVCMD(Ctrl_Y,	nv_scroll_line,	0,			FALSE),
    NVCMD(Ctrl_Z,	nv_suspend,	0,			0),
    NVCMD(ESC,		nv_esc,		0,			FALSE),
    NVCMD(Ctrl_BSL,	nv_normal,	NV_NCH_ALW,		0),
    NVCMD(Ctrl_RSB,	nv_ident,	NV_NCW,			0),
    NVCMD(Ctrl_HAT,	nv_hat,		NV_NCW,			0),
    NVCMD(Ctrl__,	nv_error,	0,			0),
    NVCMD(' ',		nv_right,	0,			0),
    NVCMD('!',		nv_operator,	0,			0),
    NVCMD('"',		nv_regname,	NV_NCH_NOP|NV_KEEPREG,	0),
    NVCMD('#',		nv_ident,	0,			0),
    NVCMD('$',		nv_dollar,	0,			0),
    NVCMD('%',		nv_percent,	0,			0),
    NVCMD('&',		nv_optrans,	0,			0),
    NVCMD('\'',		nv_gomark,	NV_NCH_ALW,		TRUE),
    NVCMD('(',		nv_brace,	0,			BACKWARD),
    NVCMD(')',		nv_brace,	0,			FORWARD),
    NVCMD('*',		nv_ident,	0,			0),
    NVCMD('+',		nv_down,	0,			TRUE),
    NVCMD(',',		nv_csearch,	0,			TRUE),
    NVCMD('-',		nv_up,		0,			TRUE),
    NVCMD('.',		nv_dot,		NV_KEEPREG,		0),
    NVCMD('/',		nv_search,	0,			FALSE),
    NVCMD('0',		nv_beginline,	0,			0),
    NVCMD('1',		nv_ignore,	0,			0),
    NVCMD('2',		nv_ignore,	0,			0),
    NVCMD('3',		nv_ignore,	0,			0),
    NVCMD('4',		nv_ignore,	0,			0),
    NVCMD('5',		nv_ignore,	0,			0),
    NVCMD('6',		nv_ignore,	0,			0),
    NVCMD('7',		nv_ignore,	0,			0),
    NVCMD('8',		nv_ignore,	0,			0),
    NVCMD('9',		nv_ignore,	0,			0),
    NVCMD(':',		nv_colon,	0,			0),
    NVCMD(';',		nv_csearch,	0,			FALSE),
    NVCMD('<',		nv_operator,	NV_RL,			0),
    NVCMD('=',		nv_operator,	0,			0),
    NVCMD('>',		nv_operator,	NV_RL,			0),
    NVCMD('?',		nv_search,	0,			FALSE),
    NVCMD('@',		nv_at,		NV_NCH_NOP,		FALSE),
    NVCMD('A',		nv_edit,	0,			0),
    NVCMD('B',		nv_bck_word,	0,			1),
    NVCMD('C',		nv_abbrev,	NV_KEEPREG,		0),
    NVCMD('D',		nv_abbrev,	NV_KEEPREG,		0),
    NVCMD('E',		nv_wordcmd,	0,			TRUE),
    NVCMD('F',		nv_csearch,	NV_NCH_ALW|NV_LANG,	BACKWARD),
    NVCMD('G',		nv_goto,	0,			TRUE),
    NVCMD('H',		nv_scroll,	0,			0),
    NVCMD('I',		nv_edit,	0,			0),
    NVCMD('J',		nv_join,	0,			0),
    NVCMD('K',		nv_ident,	0,			0),
    NVCMD('L',		nv_scroll,	0,			0),
    NVCMD('M',		nv_scroll,	0,			0),
    NVCMD('N',		nv_next,	0,			SEARCH_REV),
    NVCMD('O',		nv_open,	0,			0),
    NVCMD('P',		nv_put,		0,			0),
    NVCMD('Q',		nv_exmode,	NV_NCW,			0),
    NVCMD('R',		nv_Replace,	0,			FALSE),
    NVCMD('S',		nv_subst,	NV_KEEPREG,		0),
    NVCMD('T',		nv_csearch,	NV_NCH_ALW|NV_LANG,	BACKWARD),
    NVCMD('U',		nv_Undo,	0,			0),
    NVCMD('V',		nv_visual,	0,			FALSE),
    NVCMD('W',		nv_wordcmd,	0,			TRUE),
    NVCMD('X',		nv_abbrev,	NV_KEEPREG,		0),
    NVCMD('Y',		nv_abbrev,	NV_KEEPREG,		0),
    NVCMD('Z',		nv_Zet,		NV_NCH_NOP|NV_NCW,	0),
    NVCMD('[',		nv_brackets,	NV_NCH_ALW,		BACKWARD),
    NVCMD('\\',		nv_error,	0,			0),
    NVCMD(']',		nv_brackets,	NV_NCH_ALW,		FORWARD),
    NVCMD('^',		nv_beginline,	0,		    BL_WHITE | BL_FIX),
    NVCMD('_',		nv_lineop,	0,			0),
    NVCMD('`',		nv_gomark,	NV_NCH_ALW,		FALSE),
    NVCMD('a',		nv_edit,	NV_NCH,			0),
    NVCMD('b',		nv_bck_word,	0,			0),
    NVCMD('c',		nv_operator,	0,			0),
    NVCMD('d',		nv_operator,	0,			0),
    NVCMD('e',		nv_wordcmd,	0,			FALSE),
    NVCMD('f',		nv_csearch,	NV_NCH_ALW|NV_LANG,	FORWARD),
    NVCMD('g',		nv_g_cmd,	NV_NCH_ALW,		FALSE),
    NVCMD('h',		nv_left,	NV_RL,			0),
    NVCMD('i',		nv_edit,	NV_NCH,			0),
    NVCMD('j',		nv_down,	0,			FALSE),
    NVCMD('k',		nv_up,		0,			FALSE),
    NVCMD('l',		nv_right,	NV_RL,			0),
    NVCMD('m',		nv_mark,	NV_NCH_NOP,		0),
    NVCMD('n',		nv_next,	0,			0),
    NVCMD('o',		nv_open,	0,			0),
    NVCMD('p',		nv_put,		0,			0),
    NVCMD('q',		nv_record,	NV_NCH,			0),
    NVCMD('r',		nv_replace,	NV_NCH_NOP|NV_LANG,	0),
    NVCMD('s',		nv_subst,	NV_KEEPREG,		0),
    NVCMD('t',		nv_csearch,	NV_NCH_ALW|NV_LANG,	FORWARD),
    NVCMD('u',		nv_undo,	0,			0),
    NVCMD('v',		nv_visual,	0,			FALSE),
    NVCMD('w',		nv_wordcmd,	0,			FALSE),
    NVCMD('x',		nv_abbrev,	NV_KEEPREG,		0),
    NVCMD('y',		nv_operator,	0,			0),
    NVCMD('z',		nv_zet,		NV_NCH_ALW,		0),
    NVCMD('{',		nv_findpar,	0,			BACKWARD),
    NVCMD('|',		nv_pipe,	0,			0),
    NVCMD('}',		nv_findpar,	0,			FORWARD),
    NVCMD('~',		nv_tilde,	0,			0),

    // pound sign
    NVCMD(POUND,	nv_ident,	0,			0),
    NVCMD(K_MOUSEUP,	nv_mousescroll,	0,			MSCR_UP),
    NVCMD(K_MOUSEDOWN,	nv_mousescroll, 0,			MSCR_DOWN),
    NVCMD(K_MOUSELEFT,	nv_mousescroll, 0,			MSCR_LEFT),
    NVCMD(K_MOUSERIGHT, nv_mousescroll, 0,			MSCR_RIGHT),
    NVCMD(K_LEFTMOUSE,	nv_mouse,	0,			0),
    NVCMD(K_LEFTMOUSE_NM, nv_mouse,	0,			0),
    NVCMD(K_LEFTDRAG,	nv_mouse,	0,			0),
    NVCMD(K_LEFTRELEASE, nv_mouse,	0,			0),
    NVCMD(K_LEFTRELEASE_NM, nv_mouse,	0,			0),
    NVCMD(K_MOUSEMOVE,	nv_mouse,	0,			0),
    NVCMD(K_MIDDLEMOUSE, nv_mouse,	0,			0),
    NVCMD(K_MIDDLEDRAG, nv_mouse,	0,			0),
    NVCMD(K_MIDDLERELEASE, nv_mouse,	0,			0),
    NVCMD(K_RIGHTMOUSE, nv_mouse,	0,			0),
    NVCMD(K_RIGHTDRAG,	nv_mouse,	0,			0),
    NVCMD(K_RIGHTRELEASE, nv_mouse,	0,			0),
    NVCMD(K_X1MOUSE,	nv_mouse,	0,			0),
    NVCMD(K_X1DRAG,	nv_mouse,	0,			0),
    NVCMD(K_X1RELEASE,	nv_mouse,	0,			0),
    NVCMD(K_X2MOUSE,	nv_mouse,	0,			0),
    NVCMD(K_X2DRAG,	nv_mouse,	0,			0),
    NVCMD(K_X2RELEASE,	nv_mouse,	0,			0),
    NVCMD(K_IGNORE,	nv_ignore,	NV_KEEPREG,		0),
    NVCMD(K_NOP,	nv_nop,		0,			0),
    NVCMD(K_INS,	nv_edit,	0,			0),
    NVCMD(K_KINS,	nv_edit,	0,			0),
    NVCMD(K_BS,		nv_ctrlh,	0,			0),
    NVCMD(K_UP,		nv_up,		NV_SSS|NV_STS,		FALSE),
    NVCMD(K_S_UP,	nv_page,	NV_SS,			BACKWARD),
    NVCMD(K_DOWN,	nv_down,	NV_SSS|NV_STS,		FALSE),
    NVCMD(K_S_DOWN,	nv_page,	NV_SS,			FORWARD),
    NVCMD(K_LEFT,	nv_left,	NV_SSS|NV_STS|NV_RL,	0),
    NVCMD(K_S_LEFT,	nv_bck_word,	NV_SS|NV_RL,		0),
    NVCMD(K_C_LEFT,	nv_bck_word,	NV_SSS|NV_RL|NV_STS,	1),
    NVCMD(K_RIGHT,	nv_right,	NV_SSS|NV_STS|NV_RL,	0),
    NVCMD(K_S_RIGHT,	nv_wordcmd,	NV_SS|NV_RL,		FALSE),
    NVCMD(K_C_RIGHT,	nv_wordcmd,	NV_SSS|NV_RL|NV_STS,	TRUE),
    NVCMD(K_PAGEUP,	nv_page,	NV_SSS|NV_STS,		BACKWARD),
    NVCMD(K_KPAGEUP,	nv_page,	NV_SSS|NV_STS,		BACKWARD),
    NVCMD(K_PAGEDOWN,	nv_page,	NV_SSS|NV_STS,		FORWARD),
    NVCMD(K_KPAGEDOWN,	nv_page,	NV_SSS|NV_STS,		FORWARD),
    NVCMD(K_END,	nv_end,		NV_SSS|NV_STS,		FALSE),
    NVCMD(K_KEND,	nv_end,		NV_SSS|NV_STS,		FALSE),
    NVCMD(K_S_END,	nv_end,		NV_SS,			FALSE),
    NVCMD(K_C_END,	nv_end,		NV_SSS|NV_STS,		TRUE),
    NVCMD(K_HOME,	nv_home,	NV_SSS|NV_STS,		0),
    NVCMD(K_KHOME,	nv_home,	NV_SSS|NV_STS,		0),
    NVCMD(K_S_HOME,	nv_home,	NV_SS,			0),
    NVCMD(K_C_HOME,	nv_goto,	NV_SSS|NV_STS,		FALSE),
    NVCMD(K_DEL,	nv_abbrev,	0,			0),
    NVCMD(K_KDEL,	nv_abbrev,	0,			0),
    NVCMD(K_UNDO,	nv_kundo,	0,			0),
    NVCMD(K_HELP,	nv_help,	NV_NCW,			0),
    NVCMD(K_F1,		nv_help,	NV_NCW,			0),
    NVCMD(K_XF1,	nv_help,	NV_NCW,			0),
    NVCMD(K_SELECT,	nv_select,	0,			0),
    NVCMD(K_VER_SCROLLBAR, NV_VER_SCROLLBAR, 0,			0),
    NVCMD(K_HOR_SCROLLBAR, NV_HOR_SCROLLBAR, 0,			0),
    NVCMD(K_TABLINE,	NV_TABLINE,	0,			0),
    NVCMD(K_TABMENU,	NV_TABMENU,	0,			0),
    NVCMD(K_F21,	NV_NBCMD,	NV_NCH_ALW,		0),
    NVCMD(K_DROP,	NV_DROP,	NV_STS,			0),
    NVCMD(K_CURSORHOLD, nv_cursorhold,	NV_KEEPREG,		0),
    NVCMD(K_PS,		nv_edit,	0,			0),
    NVCMD(K_COMMAND,	nv_colon,	0,			0),
    NVCMD(K_SCRIPT_COMMAND, nv_colon,	0,			0),
};

// Number of commands in nv_cmds[].
#define NV_CMDS_SIZE ARRAY_LENGTH(nv_cmds)
