/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 */

/*
 * optiondefs.h: option definitions
 */

// The options that are local to a window or buffer have "indir" set to one of
// these values.  Special values:
// PV_NONE: global option.
// PV_WIN is added: window-local option
// PV_BUF is added: buffer-local option
// PV_BOTH is added: global option which also has a local value.
#define PV_BOTH 0x1000
#define PV_WIN  0x2000
#define PV_BUF  0x4000
#define PV_MASK 0x0fff
#define OPT_WIN(x)  (idopt_T)(PV_WIN + (int)(x))
#define OPT_BUF(x)  (idopt_T)(PV_BUF + (int)(x))
#define OPT_BOTH(x) (idopt_T)(PV_BOTH + (int)(x))

// Definition of the PV_ values for buffer-local options.
// The BV_ values are defined in option.h.
#define PV_AI		OPT_BUF(BV_AI)
#define PV_AR		OPT_BOTH(OPT_BUF(BV_AR))
#define PV_BKC		OPT_BOTH(OPT_BUF(BV_BKC))
#define PV_BH		OPT_BUF(BV_BH)
#define PV_BT		OPT_BUF(BV_BT)
#ifdef FEAT_QUICKFIX
# define PV_EFM		OPT_BOTH(OPT_BUF(BV_EFM))
# define PV_GP		OPT_BOTH(OPT_BUF(BV_GP))
# define PV_MP		OPT_BOTH(OPT_BUF(BV_MP))
#endif
#define PV_BIN		OPT_BUF(BV_BIN)
#define PV_BL		OPT_BUF(BV_BL)
#define PV_BOMB		OPT_BUF(BV_BOMB)
#define PV_CI		OPT_BUF(BV_CI)
#define PV_CIN		OPT_BUF(BV_CIN)
#define PV_CINK		OPT_BUF(BV_CINK)
#define PV_CINO		OPT_BUF(BV_CINO)
#define PV_CINSD	OPT_BUF(BV_CINSD)
#define PV_CINW		OPT_BUF(BV_CINW)
#define PV_CM		OPT_BOTH(OPT_BUF(BV_CM))
#ifdef FEAT_FOLDING
# define PV_CMS		OPT_BUF(BV_CMS)
#endif
#define PV_COM		OPT_BUF(BV_COM)
#define PV_CPT		OPT_BUF(BV_CPT)
#define PV_DICT		OPT_BOTH(OPT_BUF(BV_DICT))
#define PV_TSR		OPT_BOTH(OPT_BUF(BV_TSR))
#define PV_CSL		OPT_BUF(BV_CSL)
#ifdef FEAT_COMPL_FUNC
# define PV_CFU		OPT_BUF(BV_CFU)
#endif
#ifdef FEAT_FIND_ID
# define PV_DEF		OPT_BOTH(OPT_BUF(BV_DEF))
# define PV_INC		OPT_BOTH(OPT_BUF(BV_INC))
#endif
#define PV_EOF		OPT_BUF(BV_EOF)
#define PV_EOL		OPT_BUF(BV_EOL)
#define PV_FIXEOL	OPT_BUF(BV_FIXEOL)
#define PV_EP		OPT_BOTH(OPT_BUF(BV_EP))
#define PV_ET		OPT_BUF(BV_ET)
#define PV_FENC		OPT_BUF(BV_FENC)
#if defined(FEAT_BEVAL) && defined(FEAT_EVAL)
# define PV_BEXPR	OPT_BOTH(OPT_BUF(BV_BEXPR))
#endif
#define PV_FP		OPT_BOTH(OPT_BUF(BV_FP))
#ifdef FEAT_EVAL
# define PV_FEX		OPT_BUF(BV_FEX)
#endif
#define PV_FF		OPT_BUF(BV_FF)
#define PV_FLP		OPT_BUF(BV_FLP)
#define PV_FO		OPT_BUF(BV_FO)
#define PV_FT		OPT_BUF(BV_FT)
#define PV_IMI		OPT_BUF(BV_IMI)
#define PV_IMS		OPT_BUF(BV_IMS)
#if defined(FEAT_EVAL)
# define PV_INDE	OPT_BUF(BV_INDE)
# define PV_INDK	OPT_BUF(BV_INDK)
#endif
#if defined(FEAT_FIND_ID) && defined(FEAT_EVAL)
# define PV_INEX	OPT_BUF(BV_INEX)
#endif
#define PV_INF		OPT_BUF(BV_INF)
#define PV_ISK		OPT_BUF(BV_ISK)
#ifdef FEAT_CRYPT
# define PV_KEY		OPT_BUF(BV_KEY)
#endif
#ifdef FEAT_KEYMAP
# define PV_KMAP	OPT_BUF(BV_KMAP)
#endif
#define PV_KP		OPT_BOTH(OPT_BUF(BV_KP))
#define PV_LISP		OPT_BUF(BV_LISP)
#define PV_LOP		OPT_BUF(BV_LOP)
#define PV_LW		OPT_BOTH(OPT_BUF(BV_LW))
#define PV_MENC		OPT_BOTH(OPT_BUF(BV_MENC))
#define PV_MA		OPT_BUF(BV_MA)
#define PV_ML		OPT_BUF(BV_ML)
#define PV_MOD		OPT_BUF(BV_MOD)
#define PV_MPS		OPT_BUF(BV_MPS)
#define PV_NF		OPT_BUF(BV_NF)
#ifdef FEAT_COMPL_FUNC
# define PV_OFU		OPT_BUF(BV_OFU)
#endif
#define PV_PATH		OPT_BOTH(OPT_BUF(BV_PATH))
#define PV_PI		OPT_BUF(BV_PI)
#define PV_QE		OPT_BUF(BV_QE)
#define PV_RO		OPT_BUF(BV_RO)
#define PV_SI		OPT_BUF(BV_SI)
#define PV_SN		OPT_BUF(BV_SN)
#ifdef FEAT_SYN_HL
# define PV_SMC		OPT_BUF(BV_SMC)
# define PV_SYN		OPT_BUF(BV_SYN)
#endif
#ifdef FEAT_SPELL
# define PV_SPC		OPT_BUF(BV_SPC)
# define PV_SPF		OPT_BUF(BV_SPF)
# define PV_SPL		OPT_BUF(BV_SPL)
# define PV_SPO		OPT_BUF(BV_SPO)
#endif
#define PV_STS		OPT_BUF(BV_STS)
#define PV_SUA		OPT_BUF(BV_SUA)
#define PV_SW		OPT_BUF(BV_SW)
#define PV_SWF		OPT_BUF(BV_SWF)
#ifdef FEAT_EVAL
# define PV_TFU		OPT_BUF(BV_TFU)
#endif
#ifdef FEAT_COMPL_FUNC
# define PV_TSRFU	OPT_BOTH(OPT_BUF(BV_TSRFU))
#endif
#define PV_TAGS		OPT_BOTH(OPT_BUF(BV_TAGS))
#define PV_TC		OPT_BOTH(OPT_BUF(BV_TC))
#define PV_TS		OPT_BUF(BV_TS)
#define PV_TW		OPT_BUF(BV_TW)
#define PV_TX		OPT_BUF(BV_TX)
#ifdef FEAT_PERSISTENT_UNDO
# define PV_UDF		OPT_BUF(BV_UDF)
#endif
#define PV_WM		OPT_BUF(BV_WM)
#ifdef FEAT_VARTABS
# define PV_VSTS	OPT_BUF(BV_VSTS)
# define PV_VTS		OPT_BUF(BV_VTS)
#endif

// Definition of the PV_ values for window-local options.
// The WV_ values are defined in option.h.
#define PV_LIST		OPT_WIN(WV_LIST)
#ifdef FEAT_ARABIC
# define PV_ARAB	OPT_WIN(WV_ARAB)
#endif
#ifdef FEAT_LINEBREAK
# define PV_BRI		OPT_WIN(WV_BRI)
# define PV_BRIOPT	OPT_WIN(WV_BRIOPT)
#endif
# define PV_WCR		OPT_WIN(WV_WCR)
#ifdef FEAT_DIFF
# define PV_DIFF	OPT_WIN(WV_DIFF)
#endif
#ifdef FEAT_FOLDING
# define PV_FDC		OPT_WIN(WV_FDC)
# define PV_FEN		OPT_WIN(WV_FEN)
# define PV_FDI		OPT_WIN(WV_FDI)
# define PV_FDL		OPT_WIN(WV_FDL)
# define PV_FDM		OPT_WIN(WV_FDM)
# define PV_FML		OPT_WIN(WV_FML)
# define PV_FDN		OPT_WIN(WV_FDN)
# ifdef FEAT_EVAL
#  define PV_FDE	OPT_WIN(WV_FDE)
#  define PV_FDT	OPT_WIN(WV_FDT)
# endif
# define PV_FMR		OPT_WIN(WV_FMR)
#endif
#ifdef FEAT_LINEBREAK
# define PV_LBR		OPT_WIN(WV_LBR)
#endif
#define PV_LCS		OPT_BOTH(OPT_WIN(WV_LCS))
#define PV_FCS		OPT_BOTH(OPT_WIN(WV_FCS))
#define PV_NU		OPT_WIN(WV_NU)
#define PV_RNU		OPT_WIN(WV_RNU)
#define PV_VE		OPT_BOTH(OPT_WIN(WV_VE))
#ifdef FEAT_LINEBREAK
# define PV_NUW		OPT_WIN(WV_NUW)
#endif
#if defined(FEAT_QUICKFIX)
# define PV_PVW		OPT_WIN(WV_PVW)
#endif
#ifdef FEAT_RIGHTLEFT
# define PV_RL		OPT_WIN(WV_RL)
# define PV_RLC		OPT_WIN(WV_RLC)
#endif
#define PV_SCBIND	OPT_WIN(WV_SCBIND)
#define PV_SCROLL	OPT_WIN(WV_SCROLL)
#define PV_SMS		OPT_WIN(WV_SMS)
#define PV_SISO		OPT_BOTH(OPT_WIN(WV_SISO))
#define PV_SO		OPT_BOTH(OPT_WIN(WV_SO))
#ifdef FEAT_SPELL
# define PV_SPELL	OPT_WIN(WV_SPELL)
#endif
#ifdef FEAT_SYN_HL
# define PV_CUC		OPT_WIN(WV_CUC)
# define PV_CUL		OPT_WIN(WV_CUL)
# define PV_CULOPT	OPT_WIN(WV_CULOPT)
# define PV_CC		OPT_WIN(WV_CC)
#endif
#ifdef FEAT_LINEBREAK
# define PV_SBR		OPT_BOTH(OPT_WIN(WV_SBR))
#endif
#ifdef FEAT_STL_OPT
# define PV_STL		OPT_BOTH(OPT_WIN(WV_STL))
#endif
#define PV_UL		OPT_BOTH(OPT_BUF(BV_UL))
# define PV_WFH		OPT_WIN(WV_WFH)
# define PV_WFW		OPT_WIN(WV_WFW)
#define PV_WRAP		OPT_WIN(WV_WRAP)
#define PV_CRBIND	OPT_WIN(WV_CRBIND)
#ifdef FEAT_CONCEAL
# define PV_COCU	OPT_WIN(WV_COCU)
# define PV_COLE	OPT_WIN(WV_COLE)
#endif
#ifdef FEAT_TERMINAL
# define PV_TWK		OPT_WIN(WV_TWK)
# define PV_TWS		OPT_WIN(WV_TWS)
# define PV_TWSL	OPT_BUF(BV_TWSL)
#endif
#ifdef FEAT_SIGNS
# define PV_SCL		OPT_WIN(WV_SCL)
#endif

// WV_ and BV_ values get typecasted to this for the "indir" field
typedef enum
{
    PV_NONE = 0,
    PV_MAXVAL = 0xffff    // to avoid warnings for value out of range
} idopt_T;

// Options local to a window have a value local to a buffer and global to all
// buffers.  Indicate this by setting "var" to VAR_WIN.
#define VAR_WIN ((char_u *)-1)

// Saved values for when 'bin' is set.
static int	p_et_nobin;
static int	p_ml_nobin;
static long	p_tw_nobin;
static long	p_wm_nobin;

// Saved values for when 'paste' is set
static int	p_ai_nopaste;
static int	p_et_nopaste;
static long	p_sts_nopaste;
static long	p_tw_nopaste;
static long	p_wm_nopaste;
#ifdef FEAT_VARTABS
static char_u	*p_vsts_nopaste;
#endif

struct vimoption
{
    char	*fullname;	// full option name
    char	*shortname;	// permissible abbreviation
    long_u	flags;		// see in option.h
    char_u	*var;		// global option: pointer to variable;
				// window-local option: VAR_WIN;
				// buffer-local option: global value
    idopt_T	indir;		// global option: PV_NONE;
				// local option: indirect option index
    // callback function to invoke after an option is modified to validate and
    // apply the new value.
    opt_did_set_cb_T	opt_did_set_cb;

    // callback function to invoke when expanding possible values on the
    // cmdline. Only useful for string options.
    opt_expand_cb_T	opt_expand_cb;

    char_u	*def_val[2];	// default values for variable (vi and vim)
#ifdef FEAT_EVAL
    sctx_T	script_ctx;	// script context where the option was last set
# define SCTX_INIT , {0, 0, 0, 1}
#else
# define SCTX_INIT
#endif
};

#define VI_DEFAULT  0	    // def_val[VI_DEFAULT] is Vi default value
#define VIM_DEFAULT 1	    // def_val[VIM_DEFAULT] is Vim default value

#define ISK_LATIN1  (char_u *)"@,48-57,_,192-255"

// 'isprint' for latin1 is also used for MS-Windows cp1252, where 0x80 is used
// for the currency sign.
#if defined(MSWIN)
# define ISP_LATIN1 (char_u *)"@,~-255"
#else
# define ISP_LATIN1 (char_u *)"@,161-255"
#endif

# define HIGHLIGHT_INIT "8:SpecialKey,~:EndOfBuffer,@:NonText,d:Directory,e:ErrorMsg,i:IncSearch,l:Search,y:CurSearch,m:MoreMsg,M:ModeMsg,n:LineNr,a:LineNrAbove,b:LineNrBelow,N:CursorLineNr,G:CursorLineSign,O:CursorLineFold,r:Question,s:StatusLine,S:StatusLineNC,c:VertSplit,t:Title,v:Visual,V:VisualNOS,w:WarningMsg,W:WildMenu,f:Folded,F:FoldColumn,A:DiffAdd,C:DiffChange,D:DiffDelete,T:DiffText,>:SignColumn,-:Conceal,B:SpellBad,P:SpellCap,R:SpellRare,L:SpellLocal,+:Pmenu,=:PmenuSel,[:PmenuKind,]:PmenuKindSel,{:PmenuExtra,}:PmenuExtraSel,x:PmenuSbar,X:PmenuThumb,*:TabLine,#:TabLineSel,_:TabLineFill,!:CursorColumn,.:CursorLine,o:ColorColumn,q:QuickFixLine,z:StatusLineTerm,Z:StatusLineTermNC"

// Default python version for pyx* commands
#if defined(FEAT_PYTHON) && defined(FEAT_PYTHON3)
# define DEFAULT_PYTHON_VER	0
#elif defined(FEAT_PYTHON3)
# define DEFAULT_PYTHON_VER	3
#elif defined(FEAT_PYTHON)
# define DEFAULT_PYTHON_VER	2
#else
# define DEFAULT_PYTHON_VER	0
#endif

// used for 'cinkeys' and 'indentkeys'
#define INDENTKEYS_DEFAULT (char_u *)"0{,0},0),0],:,0#,!^F,o,O,e"

// options[] is initialized here.
// The order of the options MUST be alphabetic for ":set all" and findoption().
// All option names MUST start with a lowercase letter (for findoption()).
// Exception: "t_" options are at the end.
// The options with a NULL variable are 'hidden': a set command for them is
// ignored and they are not printed.
static struct vimoption options[] =
{
    {"aleph",	    "al",   P_NUM|P_VI_DEF|P_CURSWANT,
#ifdef FEAT_RIGHTLEFT
			    (char_u *)&p_aleph, PV_NONE,
#else
			    (char_u *)NULL, PV_NONE,
#endif
			    NULL, NULL,
			    {
#if defined(MSWIN) && !defined(FEAT_GUI_MSWIN)
			    (char_u *)128L,
#else
			    (char_u *)224L,
#endif
					    (char_u *)0L} SCTX_INIT},
    {"allowrevins", "ari",  P_BOOL|P_VI_DEF|P_VIM,
#ifdef FEAT_RIGHTLEFT
			    (char_u *)&p_ari, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"altkeymap",   "akm",  P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"ambiwidth",  "ambw",  P_STRING|P_VI_DEF|P_RCLR,
			    (char_u *)&p_ambw, PV_NONE, did_set_ambiwidth, expand_set_ambiwidth,
			    {(char_u *)"single", (char_u *)0L}
			    SCTX_INIT},
    {"antialias",   "anti", P_BOOL|P_VI_DEF|P_VIM|P_RCLR,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)FALSE}
			    SCTX_INIT},
    {"arabic",	    "arab", P_BOOL|P_VI_DEF|P_VIM|P_CURSWANT,
#ifdef FEAT_ARABIC
			    (char_u *)VAR_WIN, PV_ARAB, did_set_arabic, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"arabicshape", "arshape", P_BOOL|P_VI_DEF|P_VIM|P_RCLR,
#ifdef FEAT_ARABIC
			    (char_u *)&p_arshape, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"autochdir",  "acd",   P_BOOL|P_VI_DEF,
#ifdef FEAT_AUTOCHDIR
			    (char_u *)&p_acd, PV_NONE, did_set_autochdir, NULL,
			    {(char_u *)FALSE, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"autoindent",  "ai",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_ai, PV_AI, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"autoprint",   "ap",   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"autoread",    "ar",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_ar, PV_AR, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"autoshelldir",  "asd",   P_BOOL|P_VI_DEF,
#ifdef FEAT_AUTOSHELLDIR
			    (char_u *)&p_asd, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"autowrite",   "aw",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_aw, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"autowriteall","awa",  P_BOOL|P_VI_DEF,
			    (char_u *)&p_awa, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"background",  "bg",   P_STRING|P_VI_DEF|P_RCLR,
			    (char_u *)&p_bg, PV_NONE, did_set_background, expand_set_background,
			    {
#if (defined(MSWIN)) && !defined(FEAT_GUI)
			    (char_u *)"dark",
#else
			    (char_u *)"light",
#endif
					    (char_u *)0L} SCTX_INIT},
    {"backspace",   "bs",   P_STRING|P_VI_DEF|P_VIM|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_bs, PV_NONE, did_set_backspace, expand_set_backspace,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"backup",	    "bk",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_bk, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"backupcopy",  "bkc",  P_STRING|P_VIM|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_bkc, PV_BKC, did_set_backupcopy, expand_set_backupcopy,
#ifdef UNIX
			    {(char_u *)"yes", (char_u *)"auto"}
#else
			    {(char_u *)"auto", (char_u *)"auto"}
#endif
			    SCTX_INIT},
    {"backupdir",   "bdir", P_STRING|P_EXPAND|P_VI_DEF|P_ONECOMMA
							    |P_NODUP|P_SECURE,
			    (char_u *)&p_bdir, PV_NONE, NULL, NULL,
			    {(char_u *)DFLT_BDIR, (char_u *)0L} SCTX_INIT},
    {"backupext",   "bex",  P_STRING|P_VI_DEF|P_NFNAME,
			    (char_u *)&p_bex, PV_NONE,
			    did_set_backupext_or_patchmode, NULL,
			    {
#ifdef VMS
			    (char_u *)"_",
#else
			    (char_u *)"~",
#endif
					    (char_u *)0L} SCTX_INIT},
    {"backupskip",  "bsk",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_bsk, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"balloondelay","bdlay",P_NUM|P_VI_DEF,
#ifdef FEAT_BEVAL
			    (char_u *)&p_bdlay, PV_NONE, NULL, NULL,
			    {(char_u *)600L, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"ballooneval", "beval",P_BOOL|P_VI_DEF|P_NO_MKRC,
#ifdef FEAT_BEVAL_GUI
			    (char_u *)&p_beval, PV_NONE, did_set_ballooneval, NULL,
			    {(char_u *)FALSE, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"balloonevalterm", "bevalterm",P_BOOL|P_VI_DEF|P_NO_MKRC,
#ifdef FEAT_BEVAL_TERM
			    (char_u *)&p_bevalterm, PV_NONE,
			    did_set_balloonevalterm, NULL,
			    {(char_u *)FALSE, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"balloonexpr", "bexpr", P_STRING|P_ALLOCED|P_VI_DEF|P_VIM|P_MLE,
#if defined(FEAT_BEVAL) && defined(FEAT_EVAL)
			    (char_u *)&p_bexpr, PV_BEXPR, did_set_optexpr, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"beautify",    "bf",   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"belloff",      "bo",  P_STRING|P_VI_DEF|P_COMMA|P_NODUP,
			    (char_u *)&p_bo, PV_NONE, did_set_belloff, expand_set_belloff,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"binary",	    "bin",  P_BOOL|P_VI_DEF|P_RSTAT,
			    (char_u *)&p_bin, PV_BIN, did_set_binary, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"bioskey",	    "biosk",P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"bomb",	    NULL,   P_BOOL|P_NO_MKRC|P_VI_DEF|P_RSTAT,
			    (char_u *)&p_bomb, PV_BOMB,
			    did_set_eof_eol_fixeol_bomb, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"breakat",	    "brk",  P_STRING|P_VI_DEF|P_RALL|P_FLAGLIST,
#ifdef FEAT_LINEBREAK
			    (char_u *)&p_breakat, PV_NONE, did_set_breakat, NULL,
			    {(char_u *)" \t!@*-+;:,./?", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"breakindent",   "bri",  P_BOOL|P_VI_DEF|P_VIM|P_RWIN,
#ifdef FEAT_LINEBREAK
			    (char_u *)VAR_WIN, PV_BRI, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"breakindentopt", "briopt", P_STRING|P_ALLOCED|P_VI_DEF|P_RBUF
						  |P_ONECOMMA|P_NODUP,
#ifdef FEAT_LINEBREAK
			    (char_u *)VAR_WIN, PV_BRIOPT,
			    did_set_breakindentopt, expand_set_breakindentopt,
			    {(char_u *)"", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)NULL}
#endif
			    SCTX_INIT},
    {"browsedir",   "bsdir",P_STRING|P_VI_DEF,
#ifdef FEAT_BROWSE
			    (char_u *)&p_bsdir, PV_NONE, did_set_browsedir, expand_set_browsedir,
			    {(char_u *)"last", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"bufhidden",   "bh",   P_STRING|P_ALLOCED|P_VI_DEF|P_NOGLOB,
			    (char_u *)&p_bh, PV_BH, did_set_bufhidden, expand_set_bufhidden,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"buflisted",   "bl",   P_BOOL|P_VI_DEF|P_NOGLOB,
			    (char_u *)&p_bl, PV_BL, did_set_buflisted, NULL,
			    {(char_u *)1L, (char_u *)0L}
			    SCTX_INIT},
    {"buftype",	    "bt",   P_STRING|P_ALLOCED|P_VI_DEF|P_NOGLOB,
			    (char_u *)&p_bt, PV_BT, did_set_buftype, expand_set_buftype,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"casemap",	    "cmp",   P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_cmp, PV_NONE, did_set_casemap, expand_set_casemap,
			    {(char_u *)"internal,keepascii", (char_u *)0L}
			    SCTX_INIT},
    {"cdhome",	    "cdh",  P_BOOL|P_VI_DEF|P_VIM|P_SECURE,
			    (char_u *)&p_cdh, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L}
			    SCTX_INIT},
    {"cdpath",	    "cd",   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE|P_COMMA|P_NODUP,
			    (char_u *)&p_cdpath, PV_NONE, NULL, NULL,
			    {(char_u *)",,", (char_u *)0L}
			    SCTX_INIT},
    {"cedit",	    NULL,   P_STRING,
			    (char_u *)&p_cedit, PV_NONE, did_set_cedit, NULL,
			    {(char_u *)"", (char_u *)CTRL_F_STR}
			    SCTX_INIT},
    {"charconvert",  "ccv", P_STRING|P_VI_DEF|P_SECURE,
#if defined(FEAT_EVAL)
			    (char_u *)&p_ccv, PV_NONE, did_set_optexpr, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"cindent",	    "cin",  P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_cin, PV_CIN, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"cinkeys",	    "cink", P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_cink, PV_CINK, NULL, NULL,
			    {INDENTKEYS_DEFAULT, (char_u *)0L} SCTX_INIT},
    {"cinoptions",  "cino", P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_cino, PV_CINO, did_set_cinoptions, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"cinscopedecls", "cinsd", P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_cinsd, PV_CINSD, NULL, NULL,
			    {(char_u *)"public,protected,private", (char_u *)0L}
			    SCTX_INIT},
    {"cinwords",    "cinw", P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_cinw, PV_CINW, NULL, NULL,
			    {(char_u *)"if,else,while,do,for,switch",
				(char_u *)0L}
			    SCTX_INIT},
    {"clipboard",   "cb",   P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
#ifdef FEAT_CLIPBOARD
			    (char_u *)&p_cb, PV_NONE, did_set_clipboard, expand_set_clipboard,
# ifdef FEAT_XCLIPBOARD
			    {(char_u *)"autoselect,exclude:cons\\|linux",
							       (char_u *)0L}
# else
			    {(char_u *)"", (char_u *)0L}
# endif
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#endif
			    SCTX_INIT},
    {"cmdheight",   "ch",   P_NUM|P_VI_DEF|P_RALL,
			    (char_u *)&p_ch, PV_NONE, did_set_cmdheight, NULL,
			    {(char_u *)1L, (char_u *)0L} SCTX_INIT},
    {"cmdwinheight", "cwh", P_NUM|P_VI_DEF,
			    (char_u *)&p_cwh, PV_NONE, NULL, NULL,
			    {(char_u *)7L, (char_u *)0L} SCTX_INIT},
    {"colorcolumn", "cc",   P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP|P_RWIN,
#ifdef FEAT_SYN_HL
			    (char_u *)VAR_WIN, PV_CC, did_set_colorcolumn, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"columns",	    "co",   P_NUM|P_NODEFAULT|P_NO_MKRC|P_VI_DEF|P_RCLR,
			    (char_u *)&Columns, PV_NONE, NULL, NULL,
			    {(char_u *)80L, (char_u *)0L} SCTX_INIT},
    {"comments",    "com",  P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA
							  |P_NODUP|P_CURSWANT,
			    (char_u *)&p_com, PV_COM, did_set_comments, NULL,
			    {(char_u *)"s1:/*,mb:*,ex:*/,://,b:#,:%,:XCOMM,n:>,fb:-",
				(char_u *)0L}
			    SCTX_INIT},
    {"commentstring", "cms", P_STRING|P_ALLOCED|P_VI_DEF|P_CURSWANT,
#ifdef FEAT_FOLDING
			    (char_u *)&p_cms, PV_CMS, did_set_commentstring, NULL,
			    {(char_u *)"/*%s*/", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
			    // P_PRI_MKRC isn't needed here, optval_default()
			    // always returns TRUE for 'compatible'
    {"compatible",  "cp",   P_BOOL|P_RALL,
			    (char_u *)&p_cp, PV_NONE, did_set_compatible, NULL,
			    {(char_u *)TRUE, (char_u *)FALSE} SCTX_INIT},
    {"complete",    "cpt",  P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_cpt, PV_CPT, did_set_complete, expand_set_complete,
			    {(char_u *)".,w,b,u,t,i", (char_u *)0L}
			    SCTX_INIT},
    {"completefunc", "cfu", P_STRING|P_ALLOCED|P_VI_DEF|P_SECURE|P_FUNC,
#ifdef FEAT_COMPL_FUNC
			    (char_u *)&p_cfu, PV_CFU, did_set_completefunc, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"completeopt",   "cot",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_cot, PV_NONE, did_set_completeopt, expand_set_completeopt,
			    {(char_u *)"menu,preview", (char_u *)0L}
			    SCTX_INIT},
    {"completepopup", "cpp", P_STRING|P_VI_DEF|P_COMMA|P_NODUP|P_COLON,
#if defined(FEAT_PROP_POPUP) && defined(FEAT_QUICKFIX)
			    (char_u *)&p_cpp, PV_NONE, did_set_completepopup, expand_set_popupoption,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"completeslash",   "csl",  P_STRING|P_VI_DEF|P_VIM,
#if defined(BACKSLASH_IN_FILENAME)
			    (char_u *)&p_csl, PV_CSL, did_set_completeslash, expand_set_completeslash,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"concealcursor","cocu", P_STRING|P_ALLOCED|P_RWIN|P_VI_DEF|P_FLAGLIST,
#ifdef FEAT_CONCEAL
			    (char_u *)VAR_WIN, PV_COCU, did_set_concealcursor, expand_set_concealcursor,
			    {(char_u *)"", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"conceallevel","cole", P_NUM|P_RWIN|P_VI_DEF,
#ifdef FEAT_CONCEAL
			    (char_u *)VAR_WIN, PV_COLE, did_set_conceallevel, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)0L, (char_u *)0L}
			    SCTX_INIT},
    {"confirm",     "cf",   P_BOOL|P_VI_DEF,
#if defined(FEAT_GUI_DIALOG) || defined(FEAT_CON_DIALOG)
			    (char_u *)&p_confirm, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"conskey",	    "consk",P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"copyindent",  "ci",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_ci, PV_CI, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"cpoptions",   "cpo",  P_STRING|P_VIM|P_RALL|P_FLAGLIST,
			    (char_u *)&p_cpo, PV_NONE, did_set_cpoptions, expand_set_cpoptions,
			    {(char_u *)CPO_VI, (char_u *)CPO_VIM}
			    SCTX_INIT},
    {"cryptmethod", "cm",   P_STRING|P_ALLOCED|P_VI_DEF,
#ifdef FEAT_CRYPT
			    (char_u *)&p_cm, PV_CM, did_set_cryptmethod, expand_set_cryptmethod,
			    {(char_u *)"blowfish2", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"cscopepathcomp", "cspc", P_NUM|P_VI_DEF|P_VIM,
#ifdef FEAT_CSCOPE
			    (char_u *)&p_cspc, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"cscopeprg",   "csprg", P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#ifdef FEAT_CSCOPE
			    (char_u *)&p_csprg, PV_NONE, NULL, NULL,
			    {(char_u *)"cscope", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"cscopequickfix", "csqf", P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
#if defined(FEAT_CSCOPE) && defined(FEAT_QUICKFIX)
			    (char_u *)&p_csqf, PV_NONE,
			    did_set_cscopequickfix, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"cscoperelative", "csre", P_BOOL|P_VI_DEF|P_VIM,
#ifdef FEAT_CSCOPE
			    (char_u *)&p_csre, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"cscopetag",   "cst",  P_BOOL|P_VI_DEF|P_VIM,
#ifdef FEAT_CSCOPE
			    (char_u *)&p_cst, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"cscopetagorder", "csto", P_NUM|P_VI_DEF|P_VIM,
#ifdef FEAT_CSCOPE
			    (char_u *)&p_csto, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"cscopeverbose", "csverb", P_BOOL|P_VI_DEF|P_VIM,
#ifdef FEAT_CSCOPE
			    (char_u *)&p_csverbose, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"cursorbind",  "crb",  P_BOOL|P_VI_DEF,
			    (char_u *)VAR_WIN, PV_CRBIND, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"cursorcolumn", "cuc", P_BOOL|P_VI_DEF|P_RWINONLY,
#ifdef FEAT_SYN_HL
			    (char_u *)VAR_WIN, PV_CUC, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"cursorline",   "cul", P_BOOL|P_VI_DEF|P_RWINONLY,
#ifdef FEAT_SYN_HL
			    (char_u *)VAR_WIN, PV_CUL, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"cursorlineopt", "culopt", P_STRING|P_VI_DEF|P_RWIN|P_ONECOMMA|P_NODUP,
#ifdef FEAT_SYN_HL
			    (char_u *)VAR_WIN, PV_CULOPT,
			    did_set_cursorlineopt, expand_set_cursorlineopt,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)"both", (char_u *)0L} SCTX_INIT},
    {"debug",	    NULL,   P_STRING|P_VI_DEF,
			    (char_u *)&p_debug, PV_NONE, did_set_debug, expand_set_debug,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"define",	    "def",  P_STRING|P_ALLOCED|P_VI_DEF|P_CURSWANT,
#ifdef FEAT_FIND_ID
			    (char_u *)&p_def, PV_DEF, NULL, NULL,
			    {(char_u *)"^\\s*#\\s*define", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"delcombine", "deco",  P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_deco, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"dictionary",  "dict", P_STRING|P_EXPAND|P_VI_DEF|P_ONECOMMA|P_NODUP|P_NDNAME,
			    (char_u *)&p_dict, PV_DICT, NULL, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"diff",	    NULL,   P_BOOL|P_VI_DEF|P_RWIN|P_NOGLOB,
#ifdef FEAT_DIFF
			    (char_u *)VAR_WIN, PV_DIFF, did_set_diff, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"diffexpr",    "dex",  P_STRING|P_VI_DEF|P_SECURE|P_CURSWANT,
#if defined(FEAT_DIFF) && defined(FEAT_EVAL)
			    (char_u *)&p_dex, PV_NONE, did_set_optexpr, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"diffopt",	    "dip",  P_STRING|P_ALLOCED|P_VI_DEF|P_RWIN|P_ONECOMMA|P_COLON
								     |P_NODUP,
#ifdef FEAT_DIFF
			    (char_u *)&p_dip, PV_NONE, did_set_diffopt, expand_set_diffopt,
			    {(char_u *)"internal,filler,closeoff",
								(char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)NULL}
#endif
			    SCTX_INIT},
    {"digraph",	    "dg",   P_BOOL|P_VI_DEF|P_VIM,
#ifdef FEAT_DIGRAPHS
			    (char_u *)&p_dg, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"directory",   "dir",  P_STRING|P_EXPAND|P_VI_DEF|P_ONECOMMA
							    |P_NODUP|P_SECURE,
			    (char_u *)&p_dir, PV_NONE, NULL, NULL,
			    {(char_u *)DFLT_DIR, (char_u *)0L} SCTX_INIT},
    {"display",	    "dy",   P_STRING|P_VI_DEF|P_ONECOMMA|P_RALL|P_NODUP,
			    (char_u *)&p_dy, PV_NONE, did_set_display, expand_set_display,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"eadirection", "ead",  P_STRING|P_VI_DEF,
			    (char_u *)&p_ead, PV_NONE, did_set_eadirection, expand_set_eadirection,
			    {(char_u *)"both", (char_u *)0L}
			    SCTX_INIT},
    {"edcompatible","ed",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_ed, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"emoji",      "emo",   P_BOOL|P_VI_DEF|P_RCLR,
			    (char_u *)&p_emoji, PV_NONE, did_set_ambiwidth, NULL,
			    {(char_u *)TRUE, (char_u *)0L}
			    SCTX_INIT},
    {"encoding",    "enc",  P_STRING|P_VI_DEF|P_RCLR|P_NO_ML,
			    (char_u *)&p_enc, PV_NONE, did_set_encoding, expand_set_encoding,
			    {(char_u *)ENC_DFLT, (char_u *)0L}
			    SCTX_INIT},
    {"endoffile",   "eof",  P_BOOL|P_NO_MKRC|P_VI_DEF|P_RSTAT,
			    (char_u *)&p_eof, PV_EOF,
			    did_set_eof_eol_fixeol_bomb, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"endofline",   "eol",  P_BOOL|P_NO_MKRC|P_VI_DEF|P_RSTAT,
			    (char_u *)&p_eol, PV_EOL,
			    did_set_eof_eol_fixeol_bomb, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"equalalways", "ea",   P_BOOL|P_VI_DEF|P_RALL,
			    (char_u *)&p_ea, PV_NONE, did_set_equalalways, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"equalprg",    "ep",   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
			    (char_u *)&p_ep, PV_EP, NULL, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"errorbells",  "eb",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_eb, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"errorfile",   "ef",   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#ifdef FEAT_QUICKFIX
			    (char_u *)&p_ef, PV_NONE, NULL, NULL,
			    {(char_u *)DFLT_ERRORFILE, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"errorformat", "efm",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
#ifdef FEAT_QUICKFIX
			    (char_u *)&p_efm, PV_EFM, NULL, NULL,
			    {(char_u *)DFLT_EFM, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"esckeys",	    "ek",   P_BOOL|P_VIM,
			    (char_u *)&p_ek, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)TRUE} SCTX_INIT},
    {"eventignore", "ei",   P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_ei, PV_NONE, did_set_eventignore, expand_set_eventignore,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"expandtab",   "et",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_et, PV_ET, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"exrc",	    "ex",   P_BOOL|P_VI_DEF|P_SECURE,
			    (char_u *)&p_exrc, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"fileencoding","fenc", P_STRING|P_ALLOCED|P_VI_DEF|P_RSTAT|P_RBUF
								   |P_NO_MKRC,
			    (char_u *)&p_fenc, PV_FENC, did_set_encoding, expand_set_encoding,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"fileencodings","fencs", P_STRING|P_VI_DEF|P_ONECOMMA,
			    (char_u *)&p_fencs, PV_NONE, NULL, expand_set_encoding,
			    {(char_u *)"ucs-bom", (char_u *)0L}
			    SCTX_INIT},
    {"fileformat",  "ff",   P_STRING|P_ALLOCED|P_VI_DEF|P_RSTAT|P_NO_MKRC
								  |P_CURSWANT,
			    (char_u *)&p_ff, PV_FF, did_set_fileformat, expand_set_fileformat,
			    {(char_u *)DFLT_FF, (char_u *)0L} SCTX_INIT},
    {"fileformats", "ffs",  P_STRING|P_VIM|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_ffs, PV_NONE, did_set_fileformats, expand_set_fileformat,
			    {(char_u *)DFLT_FFS_VI, (char_u *)DFLT_FFS_VIM}
			    SCTX_INIT},
    {"fileignorecase", "fic", P_BOOL|P_VI_DEF,
			    (char_u *)&p_fic, PV_NONE, NULL, NULL,
			    {
#ifdef CASE_INSENSITIVE_FILENAME
				    (char_u *)TRUE,
#else
				    (char_u *)FALSE,
#endif
					(char_u *)0L} SCTX_INIT},
    {"filetype",    "ft",   P_STRING|P_ALLOCED|P_VI_DEF|P_NOGLOB|P_NFNAME,
			    (char_u *)&p_ft, PV_FT,
			    did_set_filetype_or_syntax, NULL,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"fillchars",   "fcs",  P_STRING|P_VI_DEF|P_RALL|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_fcs, PV_FCS, did_set_chars_option, expand_set_chars_option,
			    {(char_u *)"vert:|,fold:-,eob:~,lastline:@",
								  (char_u *)0L}
			    SCTX_INIT},
    {"fixendofline",  "fixeol", P_BOOL|P_VI_DEF|P_RSTAT,
			    (char_u *)&p_fixeol, PV_FIXEOL,
			    did_set_eof_eol_fixeol_bomb, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"fkmap",	    "fk",   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"flash",	    "fl",   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"foldclose",   "fcl",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP|P_RWIN,
#ifdef FEAT_FOLDING
			    (char_u *)&p_fcl, PV_NONE, did_set_foldclose, expand_set_foldclose,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldcolumn",  "fdc",  P_NUM|P_VI_DEF|P_RWIN,
#ifdef FEAT_FOLDING
			    (char_u *)VAR_WIN, PV_FDC, did_set_foldcolumn, NULL,
			    {(char_u *)FALSE, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldenable",  "fen",  P_BOOL|P_VI_DEF|P_RWIN,
#ifdef FEAT_FOLDING
			    (char_u *)VAR_WIN, PV_FEN, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldexpr",    "fde",  P_STRING|P_ALLOCED|P_VIM|P_VI_DEF|P_RWIN|P_MLE,
#if defined(FEAT_FOLDING) && defined(FEAT_EVAL)
			    (char_u *)VAR_WIN, PV_FDE, did_set_foldexpr, NULL,
			    {(char_u *)"0", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldignore",  "fdi",  P_STRING|P_ALLOCED|P_VIM|P_VI_DEF|P_RWIN,
#ifdef FEAT_FOLDING
			    (char_u *)VAR_WIN, PV_FDI, did_set_foldignore, NULL,
			    {(char_u *)"#", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldlevel",   "fdl",  P_NUM|P_VI_DEF|P_RWIN,
#ifdef FEAT_FOLDING
			    (char_u *)VAR_WIN, PV_FDL, did_set_foldlevel, NULL,
			    {(char_u *)0L, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldlevelstart","fdls", P_NUM|P_VI_DEF|P_CURSWANT,
#ifdef FEAT_FOLDING
			    (char_u *)&p_fdls, PV_NONE, NULL, NULL,
			    {(char_u *)-1L, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldmarker",  "fmr",  P_STRING|P_ALLOCED|P_VIM|P_VI_DEF|
						    P_RWIN|P_ONECOMMA|P_NODUP,
#ifdef FEAT_FOLDING
			    (char_u *)VAR_WIN, PV_FMR, did_set_foldmarker, NULL,
			    {(char_u *)"{{{,}}}", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldmethod",  "fdm",  P_STRING|P_ALLOCED|P_VIM|P_VI_DEF|P_RWIN,
#ifdef FEAT_FOLDING
			    (char_u *)VAR_WIN, PV_FDM, did_set_foldmethod, expand_set_foldmethod,
			    {(char_u *)"manual", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldminlines","fml",  P_NUM|P_VI_DEF|P_RWIN,
#ifdef FEAT_FOLDING
			    (char_u *)VAR_WIN, PV_FML, did_set_foldminlines, NULL,
			    {(char_u *)1L, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldnestmax", "fdn",  P_NUM|P_VI_DEF|P_RWIN,
#ifdef FEAT_FOLDING
			    (char_u *)VAR_WIN, PV_FDN, did_set_foldnestmax, NULL,
			    {(char_u *)20L, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldopen",    "fdo",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP|P_CURSWANT,
#ifdef FEAT_FOLDING
			    (char_u *)&p_fdo, PV_NONE, did_set_foldopen, expand_set_foldopen,
		 {(char_u *)"block,hor,mark,percent,quickfix,search,tag,undo",
						 (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"foldtext",    "fdt",  P_STRING|P_ALLOCED|P_VIM|P_VI_DEF|P_RWIN|P_MLE,
#if defined(FEAT_FOLDING) && defined(FEAT_EVAL)
			    (char_u *)VAR_WIN, PV_FDT, did_set_optexpr, NULL,
			    {(char_u *)"foldtext()", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"formatexpr", "fex",   P_STRING|P_ALLOCED|P_VI_DEF|P_VIM|P_MLE,
#ifdef FEAT_EVAL
			    (char_u *)&p_fex, PV_FEX, did_set_optexpr, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"formatlistpat","flp", P_STRING|P_ALLOCED|P_VI_DEF,
			    (char_u *)&p_flp, PV_FLP, NULL, NULL,
			    {(char_u *)"^\\s*\\d\\+[\\]:.)}\\t ]\\s*",
						 (char_u *)0L} SCTX_INIT},
    {"formatoptions","fo",  P_STRING|P_ALLOCED|P_VIM|P_FLAGLIST,
			    (char_u *)&p_fo, PV_FO, did_set_formatoptions, expand_set_formatoptions,
			    {(char_u *)DFLT_FO_VI, (char_u *)DFLT_FO_VIM}
			    SCTX_INIT},
    {"formatprg",   "fp",   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
			    (char_u *)&p_fp, PV_FP, NULL, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"fsync",       "fs",   P_BOOL|P_SECURE|P_VI_DEF,
#ifdef HAVE_FSYNC
			    (char_u *)&p_fs, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"gdefault",    "gd",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_gd, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"graphic",	    "gr",   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"grepformat",  "gfm",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
#ifdef FEAT_QUICKFIX
			    (char_u *)&p_gefm, PV_NONE, NULL, NULL,
			    {(char_u *)DFLT_GREPFORMAT, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"grepprg",	    "gp",   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#ifdef FEAT_QUICKFIX
			    (char_u *)&p_gp, PV_GP, NULL, NULL,
			    {
# if defined(MSWIN)
			    // may be changed to "grep -n" in os_win32.c
			    (char_u *)"findstr /n",
# elif defined(UNIX)
			    // Add an extra file name so that grep will always
			    // insert a file name in the match line.
			    (char_u *)"grep -n $* /dev/null",
# elif defined(VMS)
			    (char_u *)"SEARCH/NUMBERS ",
# else
			    (char_u *)"grep -n ",
# endif
			    (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"guicursor",    "gcr", P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
#ifdef CURSOR_SHAPE
			    (char_u *)&p_guicursor, PV_NONE,
			    did_set_guicursor, NULL,
			    {
# ifdef FEAT_GUI
				(char_u *)"n-v-c:block-Cursor/lCursor,ve:ver35-Cursor,o:hor50-Cursor,i-ci:ver25-Cursor/lCursor,r-cr:hor20-Cursor/lCursor,sm:block-Cursor-blinkwait175-blinkoff150-blinkon175",
# else	// Win32 console
				(char_u *)"n-v-c:block,o:hor50,i-ci:hor15,r-cr:hor30,sm:block",
# endif
				    (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"guifont",	    "gfn",  P_STRING|P_VI_DEF|P_RCLR|P_ONECOMMA|P_NODUP
#if !defined(FEAT_GUI_GTK)
				|P_COLON
#endif
				,
#ifdef FEAT_GUI
			    (char_u *)&p_guifont, PV_NONE, did_set_guifont, expand_set_guifont,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"guifontset",  "gfs",  P_STRING|P_VI_DEF|P_RCLR|P_ONECOMMA,
#if defined(FEAT_GUI) && defined(FEAT_XFONTSET)
			    (char_u *)&p_guifontset, PV_NONE,
			    did_set_guifontset, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"guifontwide", "gfw",  P_STRING|P_VI_DEF|P_RCLR|P_ONECOMMA|P_NODUP
#if !defined(FEAT_GUI_GTK)
				|P_COLON
#endif
				,
#if defined(FEAT_GUI)
			    (char_u *)&p_guifontwide, PV_NONE,
			    did_set_guifontwide, expand_set_guifont,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},


    {"guiheadroom", "ghr",  P_NUM|P_VI_DEF,
#if defined(FEAT_GUI_GTK) || defined(FEAT_GUI_X11)
			    (char_u *)&p_ghr, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)50L, (char_u *)0L} SCTX_INIT},

    {"guiligatures", "gli", P_STRING|P_VI_DEF|P_RCLR|P_ONECOMMA|P_NODUP,
#if defined(FEAT_GUI_GTK)
			    (char_u *)&p_guiligatures, PV_NONE,
			    did_set_guiligatures, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},

    {"guioptions",  "go",   P_STRING|P_VI_DEF|P_RALL|P_FLAGLIST,
#if defined(FEAT_GUI)
			    (char_u *)&p_go, PV_NONE, did_set_guioptions, expand_set_guioptions,
# if defined(UNIX)
			    {(char_u *)"aegimrLtT", (char_u *)0L}
# else
			    {(char_u *)"egmrLtT", (char_u *)0L}
# endif
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"guipty",	    NULL,   P_BOOL|P_VI_DEF,
#if defined(FEAT_GUI)
			    (char_u *)&p_guipty, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"guitablabel",  "gtl", P_STRING|P_VI_DEF|P_RWIN|P_MLE,
#if defined(FEAT_GUI_TABLINE)
			    (char_u *)&p_gtl, PV_NONE, did_set_guitablabel, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"guitabtooltip",  "gtt", P_STRING|P_VI_DEF|P_RWIN,
#if defined(FEAT_GUI_TABLINE)
			    (char_u *)&p_gtt, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"hardtabs",    "ht",   P_NUM|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"helpfile",    "hf",   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
			    (char_u *)&p_hf, PV_NONE, did_set_helpfile, NULL,
			    {(char_u *)DFLT_HELPFILE, (char_u *)0L}
			    SCTX_INIT},
    {"helpheight",  "hh",   P_NUM|P_VI_DEF,
			    (char_u *)&p_hh, PV_NONE,
			    did_set_winheight_helpheight, NULL,
			    {(char_u *)20L, (char_u *)0L} SCTX_INIT},
    {"helplang",    "hlg",  P_STRING|P_VI_DEF|P_ONECOMMA,
#ifdef FEAT_MULTI_LANG
			    (char_u *)&p_hlg, PV_NONE, did_set_helplang, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"hidden",	    "hid",  P_BOOL|P_VI_DEF,
			    (char_u *)&p_hid, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"highlight",   "hl",   P_STRING|P_VI_DEF|P_RCLR|P_ONECOMMA|P_NODUP|P_COLON,
			    (char_u *)&p_hl, PV_NONE, did_set_highlight, expand_set_highlight,
			    {(char_u *)HIGHLIGHT_INIT, (char_u *)0L}
			    SCTX_INIT},
    {"history",	    "hi",   P_NUM|P_VIM,
			    (char_u *)&p_hi, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)50L} SCTX_INIT},
    {"hkmap",	    "hk",   P_BOOL|P_VI_DEF|P_VIM,
#ifdef FEAT_RIGHTLEFT
			    (char_u *)&p_hkmap, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"hkmapp",	    "hkp",  P_BOOL|P_VI_DEF|P_VIM,
#ifdef FEAT_RIGHTLEFT
			    (char_u *)&p_hkmapp, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"hlsearch",    "hls",  P_BOOL|P_VI_DEF|P_VIM|P_RALL,
#if defined(FEAT_SEARCH_EXTRA)
			    (char_u *)&p_hls, PV_NONE, did_set_hlsearch, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"icon",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_icon, PV_NONE, did_set_title_icon, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"iconstring",  NULL,   P_STRING|P_VI_DEF|P_MLE,
			    (char_u *)&p_iconstring, PV_NONE,
			    did_set_iconstring, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"ignorecase",  "ic",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_ic, PV_NONE, did_set_ignorecase, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"imactivatefunc","imaf",P_STRING|P_VI_DEF|P_SECURE|P_FUNC,
#if defined(FEAT_EVAL) && \
	(defined(FEAT_XIM) || defined(IME_WITHOUT_XIM) || defined(VIMDLL))
			    (char_u *)&p_imaf, PV_NONE,
			    did_set_imactivatefunc, NULL,
			    {(char_u *)"", (char_u *)NULL}
# else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
# endif
			    SCTX_INIT},
    {"imactivatekey","imak",P_STRING|P_VI_DEF,
#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
			    (char_u *)&p_imak, PV_NONE,
			    did_set_imactivatekey, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"imcmdline",   "imc",  P_BOOL|P_VI_DEF,
			    (char_u *)&p_imcmdline, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"imdisable",   "imd",  P_BOOL|P_VI_DEF,
#if defined(HAVE_INPUT_METHOD)
			    (char_u *)&p_imdisable, PV_NONE,
			    did_set_imdisable, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L}
			    SCTX_INIT},
    {"iminsert",    "imi",  P_NUM|P_VI_DEF,
			    (char_u *)&p_iminsert, PV_IMI, did_set_iminsert, NULL,
			    {(char_u *)B_IMODE_NONE, (char_u *)0L}
			    SCTX_INIT},
    {"imsearch",    "ims",  P_NUM|P_VI_DEF,
			    (char_u *)&p_imsearch, PV_IMS, did_set_imsearch, NULL,
			    {(char_u *)B_IMODE_USE_INSERT, (char_u *)0L}
			    SCTX_INIT},
    {"imstatusfunc","imsf",P_STRING|P_VI_DEF|P_SECURE|P_FUNC,
#if defined(FEAT_EVAL) && \
	(defined(FEAT_XIM) || defined(IME_WITHOUT_XIM) || defined(VIMDLL))
			    (char_u *)&p_imsf, PV_NONE, did_set_imstatusfunc, NULL,
			    {(char_u *)"", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"imstyle",	    "imst", P_NUM|P_VI_DEF|P_SECURE,
#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
			    (char_u *)&p_imst, PV_NONE, did_set_imstyle, NULL,
			    {(char_u *)IM_OVER_THE_SPOT, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"include",	    "inc",  P_STRING|P_ALLOCED|P_VI_DEF,
#ifdef FEAT_FIND_ID
			    (char_u *)&p_inc, PV_INC, NULL, NULL,
			    {(char_u *)"^\\s*#\\s*include", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"includeexpr", "inex", P_STRING|P_ALLOCED|P_VI_DEF|P_MLE,
#if defined(FEAT_FIND_ID) && defined(FEAT_EVAL)
			    (char_u *)&p_inex, PV_INEX, did_set_optexpr, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"incsearch",   "is",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_is, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"indentexpr", "inde",  P_STRING|P_ALLOCED|P_VI_DEF|P_VIM|P_MLE,
#if defined(FEAT_EVAL)
			    (char_u *)&p_inde, PV_INDE, did_set_optexpr, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"indentkeys", "indk",  P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA|P_NODUP,
#if defined(FEAT_EVAL)
			    (char_u *)&p_indk, PV_INDK, NULL, NULL,
			    {INDENTKEYS_DEFAULT, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"infercase",   "inf",  P_BOOL|P_VI_DEF,
			    (char_u *)&p_inf, PV_INF, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"insertmode",  "im",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_im, PV_NONE, did_set_insertmode, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"isfname",	    "isf",  P_STRING|P_VI_DEF|P_COMMA|P_NODUP,
			    (char_u *)&p_isf, PV_NONE, did_set_isopt, NULL,
			    {
#ifdef BACKSLASH_IN_FILENAME
				// Excluded are: & and ^ are special in cmd.exe
				// ( and ) are used in text separating fnames
			    (char_u *)"@,48-57,/,\\,.,-,_,+,,,#,$,%,{,},[,],:,@-@,!,~,=",
#else
# if defined(AMIGA)
			    (char_u *)"@,48-57,/,.,-,_,+,,,$,:",
# elif defined(VMS)
			    (char_u *)"@,48-57,/,.,-,_,+,,,#,$,%,<,>,[,],:,;,~",
# else // UNIX et al.
			    (char_u *)"@,48-57,/,.,-,_,+,,,#,$,%,~,=",
# endif
#endif
				(char_u *)0L} SCTX_INIT},
    {"isident",	    "isi",  P_STRING|P_VI_DEF|P_COMMA|P_NODUP,
			    (char_u *)&p_isi, PV_NONE, did_set_isopt, NULL,
			    {
#if defined(MSWIN)
			    (char_u *)"@,48-57,_,128-167,224-235",
#else
			    (char_u *)"@,48-57,_,192-255",
#endif
				(char_u *)0L} SCTX_INIT},
    {"iskeyword",   "isk",  P_STRING|P_ALLOCED|P_VIM|P_COMMA|P_NODUP,
			    (char_u *)&p_isk, PV_ISK, did_set_isopt, NULL,
			    {
				(char_u *)"@,48-57,_",
#if defined(MSWIN)
				(char_u *)"@,48-57,_,128-167,224-235"
#else
				ISK_LATIN1
#endif
			    } SCTX_INIT},
    {"isprint",	    "isp",  P_STRING|P_VI_DEF|P_RALL|P_COMMA|P_NODUP,
			    (char_u *)&p_isp, PV_NONE, did_set_isopt, NULL,
			    {
#if defined(MSWIN) || defined(VMS)
			    (char_u *)"@,~-255",
#else
			    ISP_LATIN1,
#endif
				(char_u *)0L} SCTX_INIT},
    {"joinspaces",  "js",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_js, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"jumpoptions", "jop",  P_STRING|P_VI_DEF|P_VIM|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_jop, PV_NONE, did_set_jumpoptions, expand_set_jumpoptions,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"key",	    NULL,   P_STRING|P_ALLOCED|P_VI_DEF|P_NO_MKRC|P_NO_CMD_EXPAND,
#ifdef FEAT_CRYPT
			    (char_u *)&p_key, PV_KEY, did_set_cryptkey, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"keymap",	    "kmp",  P_STRING|P_ALLOCED|P_VI_DEF|P_RBUF|P_RSTAT|P_NFNAME|P_PRI_MKRC,
#ifdef FEAT_KEYMAP
			    (char_u *)&p_keymap, PV_KMAP, did_set_keymap, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#endif
			    SCTX_INIT},
    {"keymodel",    "km",   P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_km, PV_NONE, did_set_keymodel, expand_set_keymodel,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"keyprotocol", "kpc",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP|P_COLON,
			    (char_u *)&p_kpc, PV_NONE, did_set_keyprotocol, expand_set_keyprotocol,
			    {(char_u *)"kitty:kitty,foot:kitty,wezterm:kitty,xterm:mok2", (char_u *)0L}
			    SCTX_INIT},
    {"keywordprg",  "kp",   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
			    (char_u *)&p_kp, PV_KP, NULL, NULL,
			    {
#if defined(MSWIN)
			    (char_u *)":help",
#elif defined(VMS)
			    (char_u *)"help",
#elif defined(USEMAN_S)
			    (char_u *)"man -s",
#else
			    (char_u *)"man",
#endif
				(char_u *)0L} SCTX_INIT},
    {"langmap",     "lmap", P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP|P_SECURE,
#ifdef FEAT_LANGMAP
			    (char_u *)&p_langmap, PV_NONE, did_set_langmap, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"langmenu",    "lm",   P_STRING|P_VI_DEF|P_NFNAME,
#if defined(FEAT_MENU) && defined(FEAT_MULTI_LANG)
			    (char_u *)&p_lm, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"langnoremap",  "lnr",   P_BOOL|P_VI_DEF,
#ifdef FEAT_LANGMAP
			    (char_u *)&p_lnr, PV_NONE, did_set_langnoremap, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"langremap",  "lrm",   P_BOOL|P_VI_DEF,
#ifdef FEAT_LANGMAP
			    (char_u *)&p_lrm, PV_NONE, did_set_langremap, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"laststatus",  "ls",   P_NUM|P_VI_DEF|P_RALL,
			    (char_u *)&p_ls, PV_NONE, did_set_laststatus, NULL,
			    {(char_u *)1L, (char_u *)0L} SCTX_INIT},
    {"lazyredraw",  "lz",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_lz, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"linebreak",   "lbr",  P_BOOL|P_VI_DEF|P_RWIN,
#ifdef FEAT_LINEBREAK
			    (char_u *)VAR_WIN, PV_LBR, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"lines",	    NULL,   P_NUM|P_NODEFAULT|P_NO_MKRC|P_VI_DEF|P_RCLR,
			    (char_u *)&Rows, PV_NONE, NULL, NULL,
			    {
#if defined(MSWIN)
			    (char_u *)25L,
#else
			    (char_u *)24L,
#endif
					    (char_u *)0L} SCTX_INIT},
    {"linespace",   "lsp",  P_NUM|P_VI_DEF|P_RCLR,
#ifdef FEAT_GUI
			    (char_u *)&p_linespace, PV_NONE,
			    did_set_linespace, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
#ifdef FEAT_GUI_MSWIN
			    {(char_u *)1L, (char_u *)0L}
#else
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"lisp",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_lisp, PV_LISP, did_set_lisp, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"lispoptions", "lop",  P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_lop, PV_LOP, did_set_lispoptions, expand_set_lispoptions,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"lispwords",   "lw",   P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_lispwords, PV_LW, NULL, NULL,
			    {(char_u *)LISPWORD_VALUE, (char_u *)0L} SCTX_INIT},
    {"list",	    NULL,   P_BOOL|P_VI_DEF|P_RWIN,
			    (char_u *)VAR_WIN, PV_LIST, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"listchars",   "lcs",  P_STRING|P_VI_DEF|P_RALL|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_lcs, PV_LCS, did_set_chars_option, expand_set_chars_option,
			    {(char_u *)"eol:$", (char_u *)0L} SCTX_INIT},
    {"loadplugins", "lpl",  P_BOOL|P_VI_DEF,
			    (char_u *)&p_lpl, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"luadll",      NULL,   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(DYNAMIC_LUA)
			    (char_u *)&p_luadll, PV_NONE, NULL, NULL,
			    {(char_u *)DYNAMIC_LUA_DLL, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#endif
			    SCTX_INIT},
    {"macatsui",    NULL,   P_BOOL|P_VI_DEF|P_RCLR,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"magic",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_magic, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"makeef",	    "mef",  P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#ifdef FEAT_QUICKFIX
			    (char_u *)&p_mef, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"makeencoding","menc", P_STRING|P_VI_DEF,
			    (char_u *)&p_menc, PV_MENC, did_set_encoding, expand_set_encoding,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"makeprg",	    "mp",   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#ifdef FEAT_QUICKFIX
			    (char_u *)&p_mp, PV_MP, NULL, NULL,
# ifdef VMS
			    {(char_u *)"MMS", (char_u *)0L}
# else
			    {(char_u *)"make", (char_u *)0L}
# endif
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"matchpairs",  "mps",  P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_mps, PV_MPS, did_set_matchpairs, NULL,
			    {(char_u *)"(:),{:},[:]", (char_u *)0L}
			    SCTX_INIT},
    {"matchtime",   "mat",  P_NUM|P_VI_DEF,
			    (char_u *)&p_mat, PV_NONE, NULL, NULL,
			    {(char_u *)5L, (char_u *)0L} SCTX_INIT},
    {"maxcombine",  "mco",  P_NUM|P_VI_DEF|P_CURSWANT,
			    (char_u *)&p_mco, PV_NONE, did_set_maxcombine, NULL,
			    {(char_u *)2, (char_u *)0L} SCTX_INIT},
    {"maxfuncdepth", "mfd", P_NUM|P_VI_DEF,
#ifdef FEAT_EVAL
			    (char_u *)&p_mfd, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)100L, (char_u *)0L} SCTX_INIT},
    {"maxmapdepth", "mmd",  P_NUM|P_VI_DEF,
			    (char_u *)&p_mmd, PV_NONE, NULL, NULL,
			    {(char_u *)1000L, (char_u *)0L} SCTX_INIT},
    {"maxmem",	    "mm",   P_NUM|P_VI_DEF,
			    (char_u *)&p_mm, PV_NONE, NULL, NULL,
			    {(char_u *)DFLT_MAXMEM, (char_u *)0L}
			    SCTX_INIT},
    {"maxmempattern","mmp", P_NUM|P_VI_DEF,
			    (char_u *)&p_mmp, PV_NONE, NULL, NULL,
			    {(char_u *)1000L, (char_u *)0L} SCTX_INIT},
    {"maxmemtot",   "mmt",  P_NUM|P_VI_DEF,
			    (char_u *)&p_mmt, PV_NONE, NULL, NULL,
			    {(char_u *)DFLT_MAXMEMTOT, (char_u *)0L}
			    SCTX_INIT},
    {"menuitems",   "mis",  P_NUM|P_VI_DEF,
#ifdef FEAT_MENU
			    (char_u *)&p_mis, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)25L, (char_u *)0L} SCTX_INIT},
    {"mesg",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"mkspellmem",  "msm",  P_STRING|P_VI_DEF|P_EXPAND|P_SECURE,
#ifdef FEAT_SPELL
			    (char_u *)&p_msm, PV_NONE, did_set_mkspellmem, NULL,
			    {(char_u *)"460000,2000,500", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"modeline",    "ml",   P_BOOL|P_VIM,
			    (char_u *)&p_ml, PV_ML, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)TRUE} SCTX_INIT},
    {"modelineexpr", "mle",  P_BOOL|P_VI_DEF|P_SECURE,
			    (char_u *)&p_mle, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"modelines",   "mls",  P_NUM|P_VI_DEF,
			    (char_u *)&p_mls, PV_NONE, NULL, NULL,
			    {(char_u *)5L, (char_u *)0L} SCTX_INIT},
    {"modifiable",  "ma",   P_BOOL|P_VI_DEF|P_NOGLOB,
			    (char_u *)&p_ma, PV_MA, did_set_modifiable, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"modified",    "mod",  P_BOOL|P_NO_MKRC|P_VI_DEF|P_RSTAT,
			    (char_u *)&p_mod, PV_MOD, did_set_modified, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"more",	    NULL,   P_BOOL|P_VIM,
			    (char_u *)&p_more, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)TRUE} SCTX_INIT},
    {"mouse",	    NULL,   P_STRING|P_VI_DEF|P_FLAGLIST,
			    (char_u *)&p_mouse, PV_NONE, did_set_mouse, expand_set_mouse,
			    {
#if defined(MSWIN)
				(char_u *)"a",
#else
				(char_u *)"",
#endif
				(char_u *)0L} SCTX_INIT},
    {"mousefocus",   "mousef", P_BOOL|P_VI_DEF,
#ifdef FEAT_GUI
			    (char_u *)&p_mousef, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"mousehide",   "mh",   P_BOOL|P_VI_DEF,
#ifdef FEAT_GUI
			    (char_u *)&p_mh, PV_NONE, did_set_mousehide, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"mousemodel",  "mousem", P_STRING|P_VI_DEF,
			    (char_u *)&p_mousem, PV_NONE, did_set_mousemodel, expand_set_mousemodel,
			    {
#if defined(MSWIN)
				(char_u *)"popup",
#else
# if defined(MACOS_X)
				(char_u *)"popup_setpos",
# else
				(char_u *)"extend",
# endif
#endif
				(char_u *)0L} SCTX_INIT},
    {"mousemoveevent",   "mousemev",   P_BOOL|P_VI_DEF,
#ifdef FEAT_GUI
			    (char_u *)&p_mousemev, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"mouseshape",  "mouses",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
#ifdef FEAT_MOUSESHAPE
			    (char_u *)&p_mouseshape, PV_NONE,
			    did_set_mouseshape, NULL,
			    {(char_u *)"i-r:beam,s:updown,sd:udsizing,vs:leftright,vd:lrsizing,m:no,ml:up-arrow,v:rightup-arrow", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"mousetime",   "mouset",	P_NUM|P_VI_DEF,
			    (char_u *)&p_mouset, PV_NONE, NULL, NULL,
			    {(char_u *)500L, (char_u *)0L} SCTX_INIT},
    {"mzquantum",  "mzq",   P_NUM,
#ifdef FEAT_MZSCHEME
			    (char_u *)&p_mzq, PV_NONE, did_set_mzquantum, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)100L, (char_u *)100L} SCTX_INIT},
    {"mzschemedll", NULL,   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(DYNAMIC_MZSCHEME)
			    (char_u *)&p_mzschemedll, PV_NONE, NULL, NULL,
			    {(char_u *)DYNAMIC_MZSCH_DLL, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#endif
			    SCTX_INIT},
    {"mzschemegcdll", NULL, P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(DYNAMIC_MZSCHEME)
			    (char_u *)&p_mzschemegcdll, PV_NONE, NULL, NULL,
			    {(char_u *)DYNAMIC_MZGC_DLL, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#endif
			    SCTX_INIT},
    {"novice",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"nrformats",   "nf",   P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_nf, PV_NF, did_set_nrformats, expand_set_nrformats,
			    {(char_u *)"bin,octal,hex", (char_u *)0L}
			    SCTX_INIT},
    {"number",	    "nu",   P_BOOL|P_VI_DEF|P_RWIN,
			    (char_u *)VAR_WIN, PV_NU,
			    did_set_number_relativenumber, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"numberwidth", "nuw",  P_NUM|P_RWIN|P_VIM,
#ifdef FEAT_LINEBREAK
			    (char_u *)VAR_WIN, PV_NUW, did_set_numberwidth, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)8L, (char_u *)4L} SCTX_INIT},
    {"omnifunc",    "ofu",  P_STRING|P_ALLOCED|P_VI_DEF|P_SECURE|P_FUNC,
#ifdef FEAT_COMPL_FUNC
			    (char_u *)&p_ofu, PV_OFU, did_set_omnifunc, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"open",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"opendevice",  "odev", P_BOOL|P_VI_DEF,
#if defined(MSWIN)
			    (char_u *)&p_odev, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)FALSE}
			    SCTX_INIT},
    {"operatorfunc", "opfunc", P_STRING|P_VI_DEF|P_SECURE|P_FUNC,
			    (char_u *)&p_opfunc, PV_NONE,
			    did_set_operatorfunc, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"optimize",    "opt",  P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"osfiletype",  "oft",  P_STRING|P_ALLOCED|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"packpath",    "pp",   P_STRING|P_VI_DEF|P_EXPAND|P_ONECOMMA|P_NODUP
								    |P_SECURE,
			    (char_u *)&p_pp, PV_NONE, NULL, NULL,
			    {(char_u *)DFLT_RUNTIMEPATH, (char_u *)0L}
			    SCTX_INIT},
    {"paragraphs",  "para", P_STRING|P_VI_DEF,
			    (char_u *)&p_para, PV_NONE, NULL, NULL,
			    {(char_u *)"IPLPPPQPP TPHPLIPpLpItpplpipbp",
				(char_u *)0L} SCTX_INIT},
    {"paste",	    NULL,   P_BOOL|P_VI_DEF|P_PRI_MKRC,
			    (char_u *)&p_paste, PV_NONE, did_set_paste, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"pastetoggle", "pt",   P_STRING|P_VI_DEF,
			    (char_u *)&p_pt, PV_NONE, did_set_pastetoggle, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"patchexpr",   "pex",  P_STRING|P_VI_DEF|P_SECURE,
#if defined(FEAT_DIFF) && defined(FEAT_EVAL)
			    (char_u *)&p_pex, PV_NONE, did_set_optexpr, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"patchmode",   "pm",   P_STRING|P_VI_DEF|P_NFNAME,
			    (char_u *)&p_pm, PV_NONE,
			    did_set_backupext_or_patchmode, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"path",	    "pa",   P_STRING|P_EXPAND|P_VI_DEF|P_COMMA|P_NODUP,
			    (char_u *)&p_path, PV_PATH, NULL, NULL,
			    {
#if defined(AMIGA) || defined(MSWIN)
			    (char_u *)".,,",
#else
			    (char_u *)".,/usr/include,,",
#endif
				(char_u *)0L} SCTX_INIT},
    {"perldll",     NULL,   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(DYNAMIC_PERL)
			    (char_u *)&p_perldll, PV_NONE, NULL, NULL,
			    {(char_u *)DYNAMIC_PERL_DLL, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"preserveindent", "pi", P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_pi, PV_PI, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"previewheight", "pvh", P_NUM|P_VI_DEF,
#if defined(FEAT_QUICKFIX)
			    (char_u *)&p_pvh, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)12L, (char_u *)0L} SCTX_INIT},
    {"previewpopup", "pvp", P_STRING|P_VI_DEF|P_COMMA|P_NODUP|P_COLON,
#ifdef FEAT_PROP_POPUP
			    (char_u *)&p_pvp, PV_NONE, did_set_previewpopup, expand_set_popupoption,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"previewwindow", "pvw", P_BOOL|P_VI_DEF|P_RSTAT|P_NOGLOB,
#if defined(FEAT_QUICKFIX)
			    (char_u *)VAR_WIN, PV_PVW, did_set_previewwindow, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"printdevice", "pdev", P_STRING|P_VI_DEF|P_SECURE,
#ifdef FEAT_PRINTER
			    (char_u *)&p_pdev, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"printencoding", "penc", P_STRING|P_VI_DEF,
#ifdef FEAT_POSTSCRIPT
			    (char_u *)&p_penc, PV_NONE,
			    did_set_printencoding, expand_set_encoding,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"printexpr", "pexpr",  P_STRING|P_VI_DEF|P_SECURE,
#ifdef FEAT_POSTSCRIPT
			    (char_u *)&p_pexpr, PV_NONE, did_set_optexpr, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"printfont", "pfn",    P_STRING|P_VI_DEF,
#ifdef FEAT_PRINTER
			    (char_u *)&p_pfn, PV_NONE, NULL, NULL,
			    {
# ifdef MSWIN
				(char_u *)"Courier_New:h10",
# else
				(char_u *)"courier",
# endif
				(char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"printheader", "pheader",  P_STRING|P_VI_DEF|P_GETTEXT,
#ifdef FEAT_PRINTER
			    (char_u *)&p_header, PV_NONE, NULL, NULL,
			    // untranslated to avoid problems when 'encoding'
			    // is changed
			    {(char_u *)"%<%f%h%m%=Page %N", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"printmbcharset", "pmbcs",  P_STRING|P_VI_DEF,
#if defined(FEAT_POSTSCRIPT)
			    (char_u *)&p_pmcs, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"printmbfont", "pmbfn",  P_STRING|P_VI_DEF,
#if defined(FEAT_POSTSCRIPT)
			    (char_u *)&p_pmfn, PV_NONE, parse_printmbfont, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"printoptions", "popt", P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
#ifdef FEAT_PRINTER
			    (char_u *)&p_popt, PV_NONE, parse_printoptions, expand_set_printoptions,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"prompt",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_prompt, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"pumheight",   "ph",   P_NUM|P_VI_DEF,
			    (char_u *)&p_ph, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"pumwidth",    "pw",   P_NUM|P_VI_DEF,
			    (char_u *)&p_pw, PV_NONE, NULL, NULL,
			    {(char_u *)15L, (char_u *)15L} SCTX_INIT},
    {"pythondll",   NULL,   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(DYNAMIC_PYTHON)
			    (char_u *)&p_pydll, PV_NONE, NULL, NULL,
			    {(char_u *)DYNAMIC_PYTHON_DLL, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"pythonhome",  NULL,   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(FEAT_PYTHON)
			    (char_u *)&p_pyhome, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"pythonthreedll",  NULL,   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(DYNAMIC_PYTHON3)
			    (char_u *)&p_py3dll, PV_NONE, NULL, NULL,
			    {(char_u *)DYNAMIC_PYTHON3_DLL, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"pythonthreehome", NULL,   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(FEAT_PYTHON3)
			    (char_u *)&p_py3home, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"pyxversion", "pyx",   P_NUM|P_VI_DEF|P_SECURE,
#if defined(FEAT_PYTHON) || defined(FEAT_PYTHON3)
			    (char_u *)&p_pyx, PV_NONE, did_set_pyxversion, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)DEFAULT_PYTHON_VER, (char_u *)0L}
			    SCTX_INIT},
    {"quickfixtextfunc", "qftf", P_STRING|P_ALLOCED|P_VI_DEF|P_VIM|P_SECURE|P_FUNC,
#if defined(FEAT_QUICKFIX) && defined(FEAT_EVAL)
			    (char_u *)&p_qftf, PV_NONE,
			    did_set_quickfixtextfunc, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)NULL}
#endif
			    SCTX_INIT},
    {"quoteescape", "qe",   P_STRING|P_ALLOCED|P_VI_DEF,
			    (char_u *)&p_qe, PV_QE, NULL, NULL,
			    {(char_u *)"\\", (char_u *)0L} SCTX_INIT},
    {"readonly",    "ro",   P_BOOL|P_VI_DEF|P_RSTAT|P_NOGLOB,
			    (char_u *)&p_ro, PV_RO, did_set_readonly, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"redraw",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"redrawtime",  "rdt",  P_NUM|P_VI_DEF,
#ifdef FEAT_RELTIME
			    (char_u *)&p_rdt, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)2000L, (char_u *)0L} SCTX_INIT},
    {"regexpengine", "re",  P_NUM|P_VI_DEF,
			    (char_u *)&p_re, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"relativenumber", "rnu", P_BOOL|P_VI_DEF|P_RWIN,
			    (char_u *)VAR_WIN, PV_RNU,
			    did_set_number_relativenumber, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"remap",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_remap, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"renderoptions", "rop", P_STRING|P_ONECOMMA|P_RCLR|P_VI_DEF,
#ifdef FEAT_RENDER_OPTIONS
			    (char_u *)&p_rop, PV_NONE, did_set_renderoptions, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"report",	    NULL,   P_NUM|P_VI_DEF,
			    (char_u *)&p_report, PV_NONE, NULL, NULL,
			    {(char_u *)2L, (char_u *)0L} SCTX_INIT},
    {"restorescreen", "rs", P_BOOL|P_VI_DEF,
#ifdef MSWIN
			    (char_u *)&p_rs, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"revins",	    "ri",   P_BOOL|P_VI_DEF|P_VIM,
#ifdef FEAT_RIGHTLEFT
			    (char_u *)&p_ri, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"rightleft",   "rl",   P_BOOL|P_VI_DEF|P_RWIN,
#ifdef FEAT_RIGHTLEFT
			    (char_u *)VAR_WIN, PV_RL, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"rightleftcmd", "rlc", P_STRING|P_ALLOCED|P_VI_DEF|P_RWIN,
#ifdef FEAT_RIGHTLEFT
			    (char_u *)VAR_WIN, PV_RLC, did_set_rightleftcmd, expand_set_rightleftcmd,
			    {(char_u *)"search", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"rubydll",     NULL,   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(DYNAMIC_RUBY)
			    (char_u *)&p_rubydll, PV_NONE, NULL, NULL,
			    {(char_u *)DYNAMIC_RUBY_DLL, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"ruler",	    "ru",   P_BOOL|P_VI_DEF|P_VIM|P_RSTAT,
			    (char_u *)&p_ru, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"rulerformat", "ruf",  P_STRING|P_VI_DEF|P_ALLOCED|P_RSTAT|P_MLE,
#ifdef FEAT_STL_OPT
			    (char_u *)&p_ruf, PV_NONE, did_set_rulerformat, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"runtimepath", "rtp",  P_STRING|P_VI_DEF|P_EXPAND|P_ONECOMMA|P_NODUP
								    |P_SECURE,
			    (char_u *)&p_rtp, PV_NONE, NULL, NULL,
			    {(char_u *)DFLT_RUNTIMEPATH, (char_u *)0L}
			    SCTX_INIT},
    {"scroll",	    "scr",  P_NUM|P_NO_MKRC|P_VI_DEF,
			    (char_u *)VAR_WIN, PV_SCROLL, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"scrollbind",  "scb",  P_BOOL|P_VI_DEF,
			    (char_u *)VAR_WIN, PV_SCBIND, did_set_scrollbind, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"scrollfocus", "scf",  P_BOOL|P_VI_DEF,
#if defined(MSWIN) && defined(FEAT_GUI)
			    (char_u *)&p_scf, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"scrolljump",  "sj",   P_NUM|P_VI_DEF|P_VIM,
			    (char_u *)&p_sj, PV_NONE, NULL, NULL,
			    {(char_u *)1L, (char_u *)0L} SCTX_INIT},
    {"scrolloff",   "so",   P_NUM|P_VI_DEF|P_VIM|P_RALL,
			    (char_u *)&p_so, PV_SO, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"scrollopt",   "sbo",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_sbo, PV_NONE, did_set_scrollopt, expand_set_scrollopt,
			    {(char_u *)"ver,jump", (char_u *)0L}
			    SCTX_INIT},
    {"sections",    "sect", P_STRING|P_VI_DEF,
			    (char_u *)&p_sections, PV_NONE, NULL, NULL,
			    {(char_u *)"SHNHH HUnhsh", (char_u *)0L}
			    SCTX_INIT},
    {"secure",	    NULL,   P_BOOL|P_VI_DEF|P_SECURE,
			    (char_u *)&p_secure, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"selection",   "sel",  P_STRING|P_VI_DEF,
			    (char_u *)&p_sel, PV_NONE, did_set_selection, expand_set_selection,
			    {(char_u *)"inclusive", (char_u *)0L}
			    SCTX_INIT},
    {"selectmode",  "slm",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_slm, PV_NONE, did_set_selectmode, expand_set_selectmode,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"sessionoptions", "ssop", P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
#ifdef FEAT_SESSION
			    (char_u *)&p_ssop, PV_NONE,
			    did_set_sessionoptions, expand_set_sessionoptions,
	 {(char_u *)"blank,buffers,curdir,folds,help,options,tabpages,winsize,terminal",
							       (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"shell",	    "sh",   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
			    (char_u *)&p_sh, PV_NONE, NULL, NULL,
			    {
#ifdef VMS
			    (char_u *)"-",
#else
# if defined(MSWIN)
			    (char_u *)"",	// set in set_init_1()
# else
			    (char_u *)"sh",
# endif
#endif // VMS
				(char_u *)0L} SCTX_INIT},
    {"shellcmdflag","shcf", P_STRING|P_VI_DEF|P_SECURE,
			    (char_u *)&p_shcf, PV_NONE, NULL, NULL,
			    {
#if defined(MSWIN)
			    (char_u *)"/c",
#else
			    (char_u *)"-c",
#endif
				(char_u *)0L} SCTX_INIT},
    {"shellpipe",   "sp",   P_STRING|P_VI_DEF|P_SECURE,
#ifdef FEAT_QUICKFIX
			    (char_u *)&p_sp, PV_NONE, NULL, NULL,
			    {
#if defined(UNIX)
			    (char_u *)"| tee",
#else
			    (char_u *)">",
#endif
				(char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"shellquote",  "shq",  P_STRING|P_VI_DEF|P_SECURE,
			    (char_u *)&p_shq, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"shellredir",  "srr",  P_STRING|P_VI_DEF|P_SECURE,
			    (char_u *)&p_srr, PV_NONE, NULL, NULL,
			    {(char_u *)">", (char_u *)0L} SCTX_INIT},
    {"shellslash",  "ssl",   P_BOOL|P_VI_DEF,
#ifdef BACKSLASH_IN_FILENAME
			    (char_u *)&p_ssl, PV_NONE, did_set_shellslash, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"shelltemp",   "stmp", P_BOOL,
			    (char_u *)&p_stmp, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)TRUE} SCTX_INIT},
    {"shelltype",   "st",   P_NUM|P_VI_DEF,
#ifdef AMIGA
			    (char_u *)&p_st, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"shellxescape", "sxe", P_STRING|P_VI_DEF|P_SECURE,
			    (char_u *)&p_sxe, PV_NONE, NULL, NULL,
			    {
#if defined(MSWIN)
			    (char_u *)"\"&|<>()@^",
#else
			    (char_u *)"",
#endif
				(char_u *)0L} SCTX_INIT},
    {"shellxquote", "sxq",  P_STRING|P_VI_DEF|P_SECURE,
			    (char_u *)&p_sxq, PV_NONE, NULL, NULL,
			    {
#if defined(UNIX) && defined(USE_SYSTEM)
			    (char_u *)"\"",
#else
			    (char_u *)"",
#endif
				(char_u *)0L} SCTX_INIT},
    {"shiftround",  "sr",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_sr, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"shiftwidth",  "sw",   P_NUM|P_VI_DEF,
			    (char_u *)&p_sw, PV_SW,
			    did_set_shiftwidth_tabstop, NULL,
			    {(char_u *)8L, (char_u *)0L} SCTX_INIT},
    {"shortmess",   "shm",  P_STRING|P_VIM|P_FLAGLIST,
			    (char_u *)&p_shm, PV_NONE, did_set_shortmess, expand_set_shortmess,
			    {(char_u *)"S", (char_u *)"filnxtToOS"}
			    SCTX_INIT},
    {"shortname",   "sn",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_sn, PV_SN, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"showbreak",   "sbr",  P_STRING|P_VI_DEF|P_RALL,
#ifdef FEAT_LINEBREAK
			    (char_u *)&p_sbr, PV_SBR, did_set_showbreak, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"showcmd",	    "sc",   P_BOOL|P_VIM,
			    (char_u *)&p_sc, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE,
#ifdef UNIX
				(char_u *)FALSE
#else
				(char_u *)TRUE
#endif
				} SCTX_INIT},
    {"showcmdloc",  "sloc", P_STRING|P_RSTAT,
			    (char_u *)&p_sloc, PV_NONE, did_set_showcmdloc, expand_set_showcmdloc,
			    {(char_u *)"last", (char_u *)"last"} SCTX_INIT},
    {"showfulltag", "sft",  P_BOOL|P_VI_DEF,
			    (char_u *)&p_sft, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"showmatch",   "sm",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_sm, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"showmode",    "smd",  P_BOOL|P_VIM,
			    (char_u *)&p_smd, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)TRUE} SCTX_INIT},
    {"showtabline", "stal", P_NUM|P_VI_DEF|P_RALL,
			    (char_u *)&p_stal, PV_NONE, did_set_showtabline, NULL,
			    {(char_u *)1L, (char_u *)0L} SCTX_INIT},
    {"sidescroll",  "ss",   P_NUM|P_VI_DEF,
			    (char_u *)&p_ss, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"sidescrolloff", "siso", P_NUM|P_VI_DEF|P_VIM|P_RBUF,
			    (char_u *)&p_siso, PV_SISO, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"signcolumn",   "scl",  P_STRING|P_ALLOCED|P_VI_DEF|P_RCLR,
#ifdef FEAT_SIGNS
			    (char_u *)VAR_WIN, PV_SCL, did_set_signcolumn, expand_set_signcolumn,
			    {(char_u *)"auto", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"slowopen",    "slow", P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"smartcase",   "scs",  P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_scs, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"smartindent", "si",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_si, PV_SI, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"smarttab",    "sta",  P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_sta, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"smoothscroll", "sms", P_BOOL|P_VI_DEF|P_RWIN,
			    (char_u *)VAR_WIN, PV_SMS, did_set_smoothscroll, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"softtabstop", "sts",  P_NUM|P_VI_DEF|P_VIM,
			    (char_u *)&p_sts, PV_STS, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"sourceany",   NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"spell",	    NULL,   P_BOOL|P_VI_DEF|P_RWIN,
#ifdef FEAT_SPELL
			    (char_u *)VAR_WIN, PV_SPELL, did_set_spell, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"spellcapcheck", "spc", P_STRING|P_ALLOCED|P_VI_DEF|P_RBUF,
#ifdef FEAT_SPELL
			    (char_u *)&p_spc, PV_SPC, did_set_spellcapcheck, NULL,
			    {(char_u *)"[.?!]\\_[\\])'\"	 ]\\+", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"spellfile",   "spf",  P_STRING|P_EXPAND|P_ALLOCED|P_VI_DEF|P_SECURE
								  |P_ONECOMMA,
#ifdef FEAT_SPELL
			    (char_u *)&p_spf, PV_SPF, did_set_spellfile, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"spelllang",   "spl",  P_STRING|P_ALLOCED|P_VI_DEF|P_ONECOMMA
							     |P_RBUF|P_EXPAND,
#ifdef FEAT_SPELL
			    (char_u *)&p_spl, PV_SPL, did_set_spelllang, NULL,
			    {(char_u *)"en", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"spelloptions", "spo",  P_STRING|P_ALLOCED|P_VI_DEF
						    |P_ONECOMMA|P_NODUP|P_RBUF,
#ifdef FEAT_SPELL
			    (char_u *)&p_spo, PV_SPO, did_set_spelloptions, expand_set_spelloptions,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"spellsuggest", "sps", P_STRING|P_VI_DEF|P_EXPAND|P_SECURE|P_ONECOMMA,
#ifdef FEAT_SPELL
			    (char_u *)&p_sps, PV_NONE, did_set_spellsuggest, expand_set_spellsuggest,
			    {(char_u *)"best", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"splitbelow",  "sb",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_sb, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"splitkeep",   "spk",  P_STRING,
			    (char_u *)&p_spk, PV_NONE, did_set_splitkeep, expand_set_splitkeep,
			    {(char_u *)"cursor", (char_u *)"cursor"} SCTX_INIT},
    {"splitright",  "spr",  P_BOOL|P_VI_DEF,
			    (char_u *)&p_spr, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"startofline", "sol",  P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_sol, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"statusline"  ,"stl",  P_STRING|P_VI_DEF|P_ALLOCED|P_RSTAT|P_MLE,
#ifdef FEAT_STL_OPT
			    (char_u *)&p_stl, PV_STL, did_set_statusline, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"suffixes",    "su",   P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_su, PV_NONE, NULL, NULL,
			    {(char_u *)".bak,~,.o,.h,.info,.swp,.obj",
				(char_u *)0L} SCTX_INIT},
    {"suffixesadd", "sua",  P_STRING|P_VI_DEF|P_ALLOCED|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_sua, PV_SUA, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"swapfile",    "swf",  P_BOOL|P_VI_DEF|P_RSTAT,
			    (char_u *)&p_swf, PV_SWF, did_set_swapfile, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"swapsync",    "sws",  P_STRING|P_VI_DEF,
			    (char_u *)&p_sws, PV_NONE, did_set_swapsync, expand_set_swapsync,
			    {(char_u *)"fsync", (char_u *)0L} SCTX_INIT},
    {"switchbuf",   "swb",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_swb, PV_NONE, did_set_switchbuf, expand_set_switchbuf,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"synmaxcol",   "smc",  P_NUM|P_VI_DEF|P_RBUF,
#ifdef FEAT_SYN_HL
			    (char_u *)&p_smc, PV_SMC, NULL, NULL,
			    {(char_u *)3000L, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"syntax",	    "syn",  P_STRING|P_ALLOCED|P_VI_DEF|P_NOGLOB|P_NFNAME,
#ifdef FEAT_SYN_HL
			    (char_u *)&p_syn, PV_SYN,
			    did_set_filetype_or_syntax, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"tabline",	    "tal",  P_STRING|P_VI_DEF|P_RALL|P_MLE,
#ifdef FEAT_STL_OPT
			    (char_u *)&p_tal, PV_NONE, did_set_tabline, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"tabpagemax",  "tpm",  P_NUM|P_VI_DEF,
			    (char_u *)&p_tpm, PV_NONE, NULL, NULL,
			    {(char_u *)10L, (char_u *)0L} SCTX_INIT},
    {"tabstop",	    "ts",   P_NUM|P_VI_DEF|P_RBUF,
			    (char_u *)&p_ts, PV_TS,
			    did_set_shiftwidth_tabstop, NULL,
			    {(char_u *)8L, (char_u *)0L} SCTX_INIT},
    {"tagbsearch",  "tbs",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_tbs, PV_NONE, NULL, NULL,
#ifdef VMS	// binary searching doesn't appear to work on VMS
			    {(char_u *)0L, (char_u *)0L}
#else
			    {(char_u *)TRUE, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"tagcase",	    "tc",   P_STRING|P_VIM,
			    (char_u *)&p_tc, PV_TC, did_set_tagcase, expand_set_tagcase,
			    {(char_u *)"followic", (char_u *)"followic"} SCTX_INIT},
    {"tagfunc",    "tfu",   P_STRING|P_ALLOCED|P_VI_DEF|P_SECURE|P_FUNC,
#ifdef FEAT_EVAL
			    (char_u *)&p_tfu, PV_TFU, did_set_tagfunc, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"taglength",   "tl",   P_NUM|P_VI_DEF,
			    (char_u *)&p_tl, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"tagrelative", "tr",   P_BOOL|P_VIM,
			    (char_u *)&p_tr, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)TRUE} SCTX_INIT},
    {"tags",	    "tag",  P_STRING|P_EXPAND|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_tags, PV_TAGS, NULL, NULL,
			    {
#if defined(FEAT_EMACS_TAGS) && !defined(CASE_INSENSITIVE_FILENAME)
			    (char_u *)"./tags,./TAGS,tags,TAGS",
#else
			    (char_u *)"./tags,tags",
#endif
				(char_u *)0L} SCTX_INIT},
    {"tagstack",    "tgst", P_BOOL|P_VI_DEF,
			    (char_u *)&p_tgst, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"tcldll",      NULL,   P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(DYNAMIC_TCL)
			    (char_u *)&p_tcldll, PV_NONE, NULL, NULL,
			    {(char_u *)DYNAMIC_TCL_DLL, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"term",	    NULL,   P_STRING|P_EXPAND|P_NODEFAULT|P_NO_MKRC|P_VI_DEF|P_RALL,
			    (char_u *)&T_NAME, PV_NONE, did_set_term, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"termbidi", "tbidi",   P_BOOL|P_VI_DEF,
#ifdef FEAT_ARABIC
			    (char_u *)&p_tbidi, PV_NONE, NULL, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"termencoding", "tenc", P_STRING|P_VI_DEF|P_RCLR,
			    (char_u *)&p_tenc, PV_NONE, did_set_encoding, expand_set_encoding,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"termguicolors", "tgc", P_BOOL|P_VI_DEF|P_VIM|P_RCLR,
#ifdef FEAT_TERMGUICOLORS
			    (char_u *)&p_tgc, PV_NONE, did_set_termguicolors, NULL,
			    {(char_u *)FALSE, (char_u *)FALSE}
#else
			    (char_u*)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)FALSE}
#endif
			    SCTX_INIT},
    {"termwinkey", "twk",   P_STRING|P_ALLOCED|P_RWIN|P_VI_DEF,
#ifdef FEAT_TERMINAL
			    (char_u *)VAR_WIN, PV_TWK, did_set_termwinkey, NULL,
			    {(char_u *)"", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"termwinscroll", "twsl", P_NUM|P_VI_DEF|P_VIM|P_RBUF,
#ifdef FEAT_TERMINAL
			    (char_u *)&p_twsl, PV_TWSL, NULL, NULL,
			    {(char_u *)10000L, (char_u *)10000L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"termwinsize", "tws",  P_STRING|P_ALLOCED|P_RWIN|P_VI_DEF,
#ifdef FEAT_TERMINAL
			    (char_u *)VAR_WIN, PV_TWS, did_set_termwinsize, NULL,
			    {(char_u *)"", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"termwintype", "twt",  P_STRING|P_ALLOCED|P_VI_DEF,
#if defined(MSWIN) && defined(FEAT_TERMINAL)
			    (char_u *)&p_twt, PV_NONE, did_set_termwintype, expand_set_termwintype,
			    {(char_u *)"", (char_u *)NULL}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"terse",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_terse, PV_NONE, did_set_terse, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"textauto",    "ta",   P_BOOL|P_VIM,
			    (char_u *)&p_ta, PV_NONE, did_set_textauto, NULL,
			    {(char_u *)DFLT_TEXTAUTO, (char_u *)TRUE}
			    SCTX_INIT},
    {"textmode",    "tx",   P_BOOL|P_VI_DEF|P_NO_MKRC,
			    (char_u *)&p_tx, PV_TX, did_set_textmode, NULL,
			    {
#ifdef USE_CRNL
			    (char_u *)TRUE,
#else
			    (char_u *)FALSE,
#endif
				(char_u *)0L} SCTX_INIT},
    {"textwidth",   "tw",   P_NUM|P_VI_DEF|P_VIM|P_RBUF,
			    (char_u *)&p_tw, PV_TW, did_set_textwidth, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"thesaurus",   "tsr",  P_STRING|P_EXPAND|P_VI_DEF|P_ONECOMMA|P_NODUP|P_NDNAME,
			    (char_u *)&p_tsr, PV_TSR, NULL, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"thesaurusfunc", "tsrfu",  P_STRING|P_ALLOCED|P_VI_DEF|P_SECURE|P_FUNC,
#ifdef FEAT_COMPL_FUNC
			    (char_u *)&p_tsrfu, PV_TSRFU,
			    did_set_thesaurusfunc, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"tildeop",	    "top",  P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_to, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"timeout",	    "to",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_timeout, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"timeoutlen",  "tm",   P_NUM|P_VI_DEF,
			    (char_u *)&p_tm, PV_NONE, NULL, NULL,
			    {(char_u *)1000L, (char_u *)0L} SCTX_INIT},
    {"title",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_title, PV_NONE, did_set_title_icon, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"titlelen",    NULL,   P_NUM|P_VI_DEF,
			    (char_u *)&p_titlelen, PV_NONE, did_set_titlelen, NULL,
			    {(char_u *)85L, (char_u *)0L} SCTX_INIT},
    {"titleold",    NULL,   P_STRING|P_VI_DEF|P_GETTEXT|P_SECURE|P_NO_MKRC,
			    (char_u *)&p_titleold, PV_NONE, NULL, NULL,
			    {(char_u *)N_("Thanks for flying Vim"),
							       (char_u *)0L}
			    SCTX_INIT},
    {"titlestring", NULL,   P_STRING|P_VI_DEF|P_MLE,
			    (char_u *)&p_titlestring, PV_NONE,
			    did_set_titlestring, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"toolbar",     "tb",   P_STRING|P_ONECOMMA|P_VI_DEF|P_NODUP,
#if defined(FEAT_TOOLBAR) && !defined(FEAT_GUI_MSWIN)
			    (char_u *)&p_toolbar, PV_NONE, did_set_toolbar, expand_set_toolbar,
			    {(char_u *)"icons,tooltips", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"toolbariconsize",	"tbis", P_STRING|P_VI_DEF,
#if defined(FEAT_TOOLBAR) && defined(FEAT_GUI_GTK)
			    (char_u *)&p_tbis, PV_NONE,
			    did_set_toolbariconsize, expand_set_toolbariconsize,
			    {(char_u *)"small", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"ttimeout",    NULL,   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_ttimeout, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"ttimeoutlen", "ttm",  P_NUM|P_VI_DEF,
			    (char_u *)&p_ttm, PV_NONE, NULL, NULL,
			    {(char_u *)-1L, (char_u *)0L} SCTX_INIT},
    {"ttybuiltin",  "tbi",  P_BOOL|P_VI_DEF,
			    (char_u *)&p_tbi, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"ttyfast",	    "tf",   P_BOOL|P_NO_MKRC|P_VI_DEF,
			    (char_u *)&p_tf, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"ttymouse",    "ttym", P_STRING|P_NODEFAULT|P_NO_MKRC|P_VI_DEF,
#if defined(UNIX) || defined(VMS)
			    (char_u *)&p_ttym, PV_NONE, did_set_ttymouse, expand_set_ttymouse,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"ttyscroll",   "tsl",  P_NUM|P_VI_DEF,
			    (char_u *)&p_ttyscroll, PV_NONE, NULL, NULL,
			    {(char_u *)999L, (char_u *)0L} SCTX_INIT},
    {"ttytype",	    "tty",  P_STRING|P_EXPAND|P_NODEFAULT|P_NO_MKRC|P_VI_DEF|P_RALL,
			    (char_u *)&T_NAME, PV_NONE, did_set_term, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"undodir",     "udir", P_STRING|P_EXPAND|P_ONECOMMA|P_NODUP|P_SECURE
								    |P_VI_DEF,
#ifdef FEAT_PERSISTENT_UNDO
			    (char_u *)&p_udir, PV_NONE, NULL, NULL,
			    {(char_u *)".", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"undofile",    "udf",  P_BOOL|P_VI_DEF|P_VIM,
#ifdef FEAT_PERSISTENT_UNDO
			    (char_u *)&p_udf, PV_UDF, did_set_undofile, NULL,
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
#endif
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"undolevels",  "ul",   P_NUM|P_VI_DEF,
			    (char_u *)&p_ul, PV_UL, did_set_undolevels, NULL,
			    {
#if defined(UNIX) || defined(MSWIN) || defined(VMS)
			    (char_u *)1000L,
#else
			    (char_u *)100L,
#endif
				(char_u *)0L} SCTX_INIT},
    {"undoreload",  "ur",   P_NUM|P_VI_DEF,
			    (char_u *)&p_ur, PV_NONE, NULL, NULL,
			    { (char_u *)10000L, (char_u *)0L} SCTX_INIT},
    {"updatecount", "uc",   P_NUM|P_VI_DEF,
			    (char_u *)&p_uc, PV_NONE, did_set_updatecount, NULL,
			    {(char_u *)200L, (char_u *)0L} SCTX_INIT},
    {"updatetime",  "ut",   P_NUM|P_VI_DEF,
			    (char_u *)&p_ut, PV_NONE, NULL, NULL,
			    {(char_u *)4000L, (char_u *)0L} SCTX_INIT},
    {"varsofttabstop", "vsts",  P_STRING|P_VI_DEF|P_VIM|P_COMMA,
#ifdef FEAT_VARTABS
			    (char_u *)&p_vsts, PV_VSTS,
			    did_set_varsofttabstop, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)NULL}
#endif
			    SCTX_INIT},
    {"vartabstop",  "vts",  P_STRING|P_VI_DEF|P_VIM|P_RBUF|P_COMMA,
#ifdef FEAT_VARTABS
			    (char_u *)&p_vts, PV_VTS, did_set_vartabstop, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)NULL}
#endif
			    SCTX_INIT},
    {"verbose",	    "vbs",  P_NUM|P_VI_DEF,
			    (char_u *)&p_verbose, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"verbosefile", "vfile", P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
			    (char_u *)&p_vfile, PV_NONE, did_set_verbosefile, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"viewdir",     "vdir", P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#ifdef FEAT_SESSION
			    (char_u *)&p_vdir, PV_NONE, NULL, NULL,
			    {(char_u *)DFLT_VDIR, (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"viewoptions", "vop",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
#ifdef FEAT_SESSION
			    (char_u *)&p_vop, PV_NONE, did_set_viewoptions, expand_set_sessionoptions,
			    {(char_u *)"folds,options,cursor,curdir",
								  (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"viminfo",	    "vi",   P_STRING|P_ONECOMMA|P_NODUP|P_SECURE,
#ifdef FEAT_VIMINFO
			    (char_u *)&p_viminfo, PV_NONE, did_set_viminfo, NULL,
#if defined(MSWIN)
			    {(char_u *)"", (char_u *)"'100,<50,s10,h,rA:,rB:"}
#elif defined(AMIGA)
			    {(char_u *)"",
				 (char_u *)"'100,<50,s10,h,rdf0:,rdf1:,rdf2:"}
#else
			    {(char_u *)"", (char_u *)"'100,<50,s10,h"}
#endif
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"viminfofile", "vif",  P_STRING|P_EXPAND|P_ONECOMMA|P_NODUP
							    |P_SECURE|P_VI_DEF,
#ifdef FEAT_VIMINFO
			    (char_u *)&p_viminfofile, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"virtualedit", "ve",   P_STRING|P_ONECOMMA|P_NODUP|P_VI_DEF
							    |P_VIM|P_CURSWANT,
			    (char_u *)&p_ve, PV_VE, did_set_virtualedit, expand_set_virtualedit,
			    {(char_u *)"", (char_u *)""}
			    SCTX_INIT},
    {"visualbell",  "vb",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_vb, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"w1200",	    NULL,   P_NUM|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"w300",	    NULL,   P_NUM|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"w9600",	    NULL,   P_NUM|P_VI_DEF,
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"warn",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_warn, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"weirdinvert", "wiv",  P_BOOL|P_VI_DEF|P_RCLR,
			    (char_u *)&p_wiv, PV_NONE, did_set_weirdinvert, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"whichwrap",   "ww",   P_STRING|P_VIM|P_ONECOMMA|P_FLAGLIST,
			    (char_u *)&p_ww, PV_NONE, did_set_whichwrap, expand_set_whichwrap,
			    {(char_u *)"", (char_u *)"b,s"} SCTX_INIT},
    {"wildchar",    "wc",   P_NUM|P_VIM,
			    (char_u *)&p_wc, PV_NONE, did_set_wildchar, NULL,
			    {(char_u *)(long)Ctrl_E, (char_u *)(long)TAB}
			    SCTX_INIT},
    {"wildcharm",   "wcm",  P_NUM|P_VI_DEF,
			    (char_u *)&p_wcm, PV_NONE, did_set_wildchar, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"wildignore",  "wig",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_wig, PV_NONE, NULL, NULL,
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},
    {"wildignorecase", "wic", P_BOOL|P_VI_DEF,
			    (char_u *)&p_wic, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"wildmenu",    "wmnu", P_BOOL|P_VI_DEF,
			    (char_u *)&p_wmnu, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"wildmode",    "wim",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP|P_COLON,
			    (char_u *)&p_wim, PV_NONE, did_set_wildmode, expand_set_wildmode,
			    {(char_u *)"full", (char_u *)0L} SCTX_INIT},
    {"wildoptions", "wop",  P_STRING|P_VI_DEF|P_ONECOMMA|P_NODUP,
			    (char_u *)&p_wop, PV_NONE, did_set_wildoptions, expand_set_wildoptions,
			    {(char_u *)"", (char_u *)0L}
			    SCTX_INIT},
    {"winaltkeys",  "wak",  P_STRING|P_VI_DEF,
#ifdef FEAT_WAK
			    (char_u *)&p_wak, PV_NONE, did_set_winaltkeys, expand_set_winaltkeys,
			    {(char_u *)"menu", (char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)NULL, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"wincolor", "wcr",	    P_STRING|P_ALLOCED|P_VI_DEF|P_RWIN,
			    (char_u *)VAR_WIN, PV_WCR, did_set_wincolor, expand_set_wincolor,
			    {(char_u *)"", (char_u *)NULL}
			    SCTX_INIT},
    {"window",	    "wi",   P_NUM|P_VI_DEF,
			    (char_u *)&p_window, PV_NONE, did_set_window, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"winfixheight", "wfh", P_BOOL|P_VI_DEF|P_RSTAT,
			    (char_u *)VAR_WIN, PV_WFH, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"winfixwidth", "wfw", P_BOOL|P_VI_DEF|P_RSTAT,
			    (char_u *)VAR_WIN, PV_WFW, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"winheight",   "wh",   P_NUM|P_VI_DEF,
			    (char_u *)&p_wh, PV_NONE,
			    did_set_winheight_helpheight, NULL,
			    {(char_u *)1L, (char_u *)0L} SCTX_INIT},
    {"winminheight", "wmh", P_NUM|P_VI_DEF,
			    (char_u *)&p_wmh, PV_NONE, did_set_winminheight, NULL,
			    {(char_u *)1L, (char_u *)0L} SCTX_INIT},
    {"winminwidth", "wmw", P_NUM|P_VI_DEF,
			    (char_u *)&p_wmw, PV_NONE, did_set_winminwidth, NULL,
			    {(char_u *)1L, (char_u *)0L} SCTX_INIT},
    {"winptydll", NULL,	    P_STRING|P_EXPAND|P_VI_DEF|P_SECURE,
#if defined(MSWIN) && defined(FEAT_TERMINAL)
			    (char_u *)&p_winptydll, PV_NONE, NULL, NULL, {
# ifdef _WIN64
			    (char_u *)"winpty64.dll",
# else
			    (char_u *)"winpty32.dll",
# endif
				(char_u *)0L}
#else
			    (char_u *)NULL, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L}
#endif
			    SCTX_INIT},
    {"winwidth",   "wiw",   P_NUM|P_VI_DEF,
			    (char_u *)&p_wiw, PV_NONE, did_set_winwidth, NULL,
			    {(char_u *)20L, (char_u *)0L} SCTX_INIT},
    {"wrap",	    NULL,   P_BOOL|P_VI_DEF|P_RWIN,
			    (char_u *)VAR_WIN, PV_WRAP, did_set_wrap, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"wrapmargin",  "wm",   P_NUM|P_VI_DEF,
			    (char_u *)&p_wm, PV_WM, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"wrapscan",    "ws",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_ws, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"write",	    NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_write, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},
    {"writeany",    "wa",   P_BOOL|P_VI_DEF,
			    (char_u *)&p_wa, PV_NONE, NULL, NULL,
			    {(char_u *)FALSE, (char_u *)0L} SCTX_INIT},
    {"writebackup", "wb",   P_BOOL|P_VI_DEF|P_VIM,
			    (char_u *)&p_wb, PV_NONE, NULL, NULL,
			    {
#ifdef FEAT_WRITEBACKUP
			    (char_u *)TRUE,
#else
			    (char_u *)FALSE,
#endif
				(char_u *)0L} SCTX_INIT},
    {"writedelay",  "wd",   P_NUM|P_VI_DEF,
			    (char_u *)&p_wd, PV_NONE, NULL, NULL,
			    {(char_u *)0L, (char_u *)0L} SCTX_INIT},
    {"xtermcodes",  NULL,   P_BOOL|P_VI_DEF,
			    (char_u *)&p_xtermcodes, PV_NONE, NULL, NULL,
			    {(char_u *)TRUE, (char_u *)0L} SCTX_INIT},

// terminal output codes
#define p_term(sss, vvv) \
			    {sss, NULL, P_STRING|P_VI_DEF|P_RALL|P_SECURE, \
			    (char_u *)&vvv, PV_NONE, did_set_term_option, NULL, \
			    {(char_u *)"", (char_u *)0L} SCTX_INIT},

    p_term("t_AB", T_CAB)
    p_term("t_AF", T_CAF)
    p_term("t_AU", T_CAU)
    p_term("t_AL", T_CAL)
    p_term("t_al", T_AL)
    p_term("t_bc", T_BC)
    p_term("t_BE", T_BE)
    p_term("t_BD", T_BD)
    p_term("t_cd", T_CD)
    p_term("t_ce", T_CE)
    p_term("t_Ce", T_UCE)
    p_term("t_CF", T_CFO)
    p_term("t_cl", T_CL)
    p_term("t_cm", T_CM)
    p_term("t_Co", T_CCO)
    p_term("t_CS", T_CCS)
    p_term("t_Cs", T_UCS)
    p_term("t_cs", T_CS)
    p_term("t_CV", T_CSV)
    p_term("t_da", T_DA)
    p_term("t_db", T_DB)
    p_term("t_DL", T_CDL)
    p_term("t_dl", T_DL)
    p_term("t_ds", T_DS)
    p_term("t_Ds", T_CDS)
    p_term("t_EC", T_CEC)
    p_term("t_EI", T_CEI)
    p_term("t_fs", T_FS)
    p_term("t_fd", T_FD)
    p_term("t_fe", T_FE)
    p_term("t_GP", T_CGP)
    p_term("t_IE", T_CIE)
    p_term("t_IS", T_CIS)
    p_term("t_ke", T_KE)
    p_term("t_ks", T_KS)
    p_term("t_le", T_LE)
    p_term("t_mb", T_MB)
    p_term("t_md", T_MD)
    p_term("t_me", T_ME)
    p_term("t_mr", T_MR)
    p_term("t_ms", T_MS)
    p_term("t_nd", T_ND)
    p_term("t_op", T_OP)
    p_term("t_RF", T_RFG)
    p_term("t_RB", T_RBG)
    p_term("t_RC", T_CRC)
    p_term("t_RI", T_CRI)
    p_term("t_Ri", T_SRI)
    p_term("t_RK", T_CRK)
    p_term("t_RS", T_CRS)
    p_term("t_RT", T_CRT)
    p_term("t_RV", T_CRV)
    p_term("t_Sb", T_CSB)
    p_term("t_SC", T_CSC)
    p_term("t_se", T_SE)
    p_term("t_Sf", T_CSF)
    p_term("t_SH", T_CSH)
    p_term("t_SI", T_CSI)
    p_term("t_Si", T_SSI)
    p_term("t_so", T_SO)
    p_term("t_SR", T_CSR)
    p_term("t_sr", T_SR)
    p_term("t_ST", T_CST)
    p_term("t_Te", T_STE)
    p_term("t_te", T_TE)
    p_term("t_TE", T_CTE)
    p_term("t_ti", T_TI)
    p_term("t_TI", T_CTI)
    p_term("t_Ts", T_STS)
    p_term("t_ts", T_TS)
    p_term("t_u7", T_U7)
    p_term("t_ue", T_UE)
    p_term("t_us", T_US)
    p_term("t_Us", T_USS)
    p_term("t_ut", T_UT)
    p_term("t_vb", T_VB)
    p_term("t_ve", T_VE)
    p_term("t_vi", T_VI)
    p_term("t_VS", T_CVS)
    p_term("t_vs", T_VS)
    p_term("t_WP", T_CWP)
    p_term("t_WS", T_CWS)
    p_term("t_XM", T_CXM)
    p_term("t_xn", T_XN)
    p_term("t_xs", T_XS)
    p_term("t_ZH", T_CZH)
    p_term("t_ZR", T_CZR)
    p_term("t_8f", T_8F)
    p_term("t_8b", T_8B)
    p_term("t_8u", T_8U)

// terminal key codes are not in here

    // end marker
    {NULL, NULL, 0, NULL, PV_NONE, NULL, NULL, {NULL, NULL} SCTX_INIT}
};

#define OPTION_COUNT ARRAY_LENGTH(options)

// The following is needed to make the gen_opt_test.vim script work.
// {"
