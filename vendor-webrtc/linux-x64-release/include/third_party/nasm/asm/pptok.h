/* Automatically generated from ./asm/pptok.dat by ./asm/pptok.pl */
/* Do not edit */

enum preproc_token {
    PP_IF                    =   0,
    PP_IFCTX                 =   1,
    PP_IFDEF                 =   2,
    PP_IFDEFALIAS            =   3,
    PP_IFDIFI                =   4,
    PP_IFEMPTY               =   5,
    PP_IFENV                 =   6,
    PP_IFID                  =   7,
    PP_IFIDN                 =   8,
    PP_IFIDNI                =   9,
    PP_IFMACRO               =  10,
    PP_IFNUM                 =  11,
    PP_IFSTR                 =  12,
    PP_IFTOKEN               =  13,
    PP_IFUSABLE              =  14,
    PP_IFUSING               =  15,
    PP_IFN                   =  16,
    PP_IFNCTX                =  17,
    PP_IFNDEF                =  18,
    PP_IFNDEFALIAS           =  19,
    PP_IFNDIFI               =  20,
    PP_IFNEMPTY              =  21,
    PP_IFNENV                =  22,
    PP_IFNID                 =  23,
    PP_IFNIDN                =  24,
    PP_IFNIDNI               =  25,
    PP_IFNMACRO              =  26,
    PP_IFNNUM                =  27,
    PP_IFNSTR                =  28,
    PP_IFNTOKEN              =  29,
    PP_IFNUSABLE             =  30,
    PP_IFNUSING              =  31,
    PP_ELIF                  =  32,
    PP_ELIFCTX               =  33,
    PP_ELIFDEF               =  34,
    PP_ELIFDEFALIAS          =  35,
    PP_ELIFDIFI              =  36,
    PP_ELIFEMPTY             =  37,
    PP_ELIFENV               =  38,
    PP_ELIFID                =  39,
    PP_ELIFIDN               =  40,
    PP_ELIFIDNI              =  41,
    PP_ELIFMACRO             =  42,
    PP_ELIFNUM               =  43,
    PP_ELIFSTR               =  44,
    PP_ELIFTOKEN             =  45,
    PP_ELIFUSABLE            =  46,
    PP_ELIFUSING             =  47,
    PP_ELIFN                 =  48,
    PP_ELIFNCTX              =  49,
    PP_ELIFNDEF              =  50,
    PP_ELIFNDEFALIAS         =  51,
    PP_ELIFNDIFI             =  52,
    PP_ELIFNEMPTY            =  53,
    PP_ELIFNENV              =  54,
    PP_ELIFNID               =  55,
    PP_ELIFNIDN              =  56,
    PP_ELIFNIDNI             =  57,
    PP_ELIFNMACRO            =  58,
    PP_ELIFNNUM              =  59,
    PP_ELIFNSTR              =  60,
    PP_ELIFNTOKEN            =  61,
    PP_ELIFNUSABLE           =  62,
    PP_ELIFNUSING            =  63,
    PP_ALIASES               =  64,
    PP_ARG                   =  65,
    PP_CLEAR                 =  66,
    PP_DEPEND                =  67,
    PP_ELSE                  =  68,
    PP_ENDIF                 =  69,
    PP_ENDM                  =  70,
    PP_ENDMACRO              =  71,
    PP_ENDREP                =  72,
    PP_ERROR                 =  73,
    PP_EXITMACRO             =  74,
    PP_EXITREP               =  75,
    PP_FATAL                 =  76,
    PP_INCLUDE               =  77,
    PP_LINE                  =  78,
    PP_LOCAL                 =  79,
    PP_NOTE                  =  80,
    PP_NULL                  =  81,
    PP_POP                   =  82,
    PP_PRAGMA                =  83,
    PP_PUSH                  =  84,
    PP_REP                   =  85,
    PP_REPL                  =  86,
    PP_REQUIRE               =  87,
    PP_ROTATE                =  88,
    PP_STACKSIZE             =  89,
    PP_UNDEF                 =  90,
    PP_UNDEFALIAS            =  91,
    PP_USE                   =  92,
    PP_WARNING               =  93,
    PP_ASSIGN                =  94,
    PP_IASSIGN               =  95,
    PP_DEFALIAS              =  96,
    PP_IDEFALIAS             =  97,
    PP_DEFINE                =  98,
    PP_IDEFINE               =  99,
    PP_DEFSTR                = 100,
    PP_IDEFSTR               = 101,
    PP_DEFTOK                = 102,
    PP_IDEFTOK               = 103,
    PP_MACRO                 = 104,
    PP_IMACRO                = 105,
    PP_PATHSEARCH            = 106,
    PP_IPATHSEARCH           = 107,
    PP_RMACRO                = 108,
    PP_IRMACRO               = 109,
    PP_STRCAT                = 110,
    PP_ISTRCAT               = 111,
    PP_STRLEN                = 112,
    PP_ISTRLEN               = 113,
    PP_SUBSTR                = 114,
    PP_ISUBSTR               = 115,
    PP_XDEFINE               = 116,
    PP_IXDEFINE              = 117,
    PP_UNMACRO               = 118,
    PP_UNIMACRO              = 119,
    PP_count                 = 120,
    PP_invalid               =  -1
};

#define PP_COND(x)     ((x) & 0xf)
#define PP_IS_COND(x)  ((unsigned int)(x) < PP_ALIASES)
#define PP_COND_NEGATIVE(x) (!!((x) & 0x10))

#define PP_HAS_CASE(x) ((x) >= PP_ASSIGN)
#define PP_INSENSITIVE(x) ((x) & 1)
#define PP_TOKLEN_MAX 14

#define CASE_PP_IF \
	case PP_IF:\
	case PP_IFCTX:\
	case PP_IFDEF:\
	case PP_IFDEFALIAS:\
	case PP_IFDIFI:\
	case PP_IFEMPTY:\
	case PP_IFENV:\
	case PP_IFID:\
	case PP_IFIDN:\
	case PP_IFIDNI:\
	case PP_IFMACRO:\
	case PP_IFNUM:\
	case PP_IFSTR:\
	case PP_IFTOKEN:\
	case PP_IFUSABLE:\
	case PP_IFUSING:\
	case PP_IFN:\
	case PP_IFNCTX:\
	case PP_IFNDEF:\
	case PP_IFNDEFALIAS:\
	case PP_IFNDIFI:\
	case PP_IFNEMPTY:\
	case PP_IFNENV:\
	case PP_IFNID:\
	case PP_IFNIDN:\
	case PP_IFNIDNI:\
	case PP_IFNMACRO:\
	case PP_IFNNUM:\
	case PP_IFNSTR:\
	case PP_IFNTOKEN:\
	case PP_IFNUSABLE:\
	case PP_IFNUSING
#define CASE_PP_ELIF \
	case PP_ELIF:\
	case PP_ELIFCTX:\
	case PP_ELIFDEF:\
	case PP_ELIFDEFALIAS:\
	case PP_ELIFDIFI:\
	case PP_ELIFEMPTY:\
	case PP_ELIFENV:\
	case PP_ELIFID:\
	case PP_ELIFIDN:\
	case PP_ELIFIDNI:\
	case PP_ELIFMACRO:\
	case PP_ELIFNUM:\
	case PP_ELIFSTR:\
	case PP_ELIFTOKEN:\
	case PP_ELIFUSABLE:\
	case PP_ELIFUSING:\
	case PP_ELIFN:\
	case PP_ELIFNCTX:\
	case PP_ELIFNDEF:\
	case PP_ELIFNDEFALIAS:\
	case PP_ELIFNDIFI:\
	case PP_ELIFNEMPTY:\
	case PP_ELIFNENV:\
	case PP_ELIFNID:\
	case PP_ELIFNIDN:\
	case PP_ELIFNIDNI:\
	case PP_ELIFNMACRO:\
	case PP_ELIFNNUM:\
	case PP_ELIFNSTR:\
	case PP_ELIFNTOKEN:\
	case PP_ELIFNUSABLE:\
	case PP_ELIFNUSING
