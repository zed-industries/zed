/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2022 The NASM Authors - All Rights Reserved
 *   See the file AUTHORS included with the NASM distribution for
 *   the specific copyright holders.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following
 *   conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above
 *     copyright notice, this list of conditions and the following
 *     disclaimer in the documentation and/or other materials provided
 *     with the distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *     CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *     INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *     MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 *     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *     SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *     NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *     CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *     OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ----------------------------------------------------------------------- */

/*
 * nasm.h   main header file for the Netwide Assembler: inter-module interface
 */

#ifndef NASM_NASM_H
#define NASM_NASM_H

#include "compiler.h"

#include <time.h>

#include "nasmlib.h"
#include "nctype.h"
#include "strlist.h"
#include "preproc.h"
#include "insnsi.h"     /* For enum opcode */
#include "directiv.h"   /* For enum directive */
#include "labels.h"     /* For enum mangle_index, enum label_type */
#include "opflags.h"
#include "regs.h"
#include "srcfile.h"
#include "error.h"

/* Program name for error messages etc. */
extern const char *_progname;

/* Time stamp for the official start of compilation */
struct compile_time {
    time_t t;
    bool have_local, have_gm, have_posix;
    int64_t posix;
    struct tm local;
    struct tm gm;
};
extern struct compile_time official_compile_time;

/* POSIX timestamp if and only if we are not a reproducible build */
extern bool reproducible;
static inline int64_t posix_timestamp(void)
{
    return reproducible ? 0 : official_compile_time.posix;
}

#define NO_SEG  INT32_C(-1)     /* null segment value */
#define SEG_ABS 0x40000000L     /* mask for far-absolute segments */

#define IDLEN_MAX 4096
#define DECOLEN_MAX 32

/*
 * Name pollution problems: <time.h> on Digital UNIX pulls in some
 * strange hardware header file which sees fit to define R_SP. We
 * undefine it here so as not to break the enum below.
 */
#ifdef R_SP
#undef R_SP
#endif

/*
 * We must declare the existence of this structure type up here,
 * since we have to reference it before we define it...
 */
struct ofmt;

/*
 * Values for the `type' parameter to an output function.
 */
enum out_type {
    OUT_RAWDATA,    /* Plain bytes */
    OUT_RESERVE,    /* Reserved bytes (RESB et al) */
    OUT_ZERODATA,   /* Initialized data, but all zero */
    OUT_ADDRESS,    /* An address (symbol value) */
    OUT_RELADDR,    /* A relative address */
    OUT_SEGMENT,    /* A segment number */

    /*
     * These values are used by the legacy backend interface only;
     * see output/legacy.c for more information.  These should never
     * be used otherwise.  Once all backends have been migrated to the
     * new interface they should be removed.
     */
    OUT_REL1ADR,
    OUT_REL2ADR,
    OUT_REL4ADR,
    OUT_REL8ADR
};

enum out_flags {
    OUT_WRAP     = 0,           /* Undefined signedness (wraps) */
    OUT_SIGNED   = 1,           /* Value is signed */
    OUT_UNSIGNED = 2,           /* Value is unsigned */
    OUT_SIGNMASK = 3            /* Mask for signedness bits */
};

/*
 * The data we send down to the backend.
 * XXX: We still want to push down the base address symbol if
 * available, and replace the segment numbers with a structure.
 */
struct out_data {
    int64_t offset;             /* Offset within segment */
    int32_t segment;            /* Segment written to */
    enum out_type type;         /* See above */
    enum out_flags flags;       /* See above */
    int inslen;                 /* Length of instruction */
    int insoffs;                /* Offset inside instruction */
    int bits;                   /* Bits mode of compilation */
    uint64_t size;              /* Size of output */
    const struct itemplate *itemp; /* Instruction template */
    const void *data;           /* Data for OUT_RAWDATA */
    uint64_t toffset;           /* Target address offset for relocation */
    int32_t tsegment;           /* Target segment for relocation */
    int32_t twrt;               /* Relocation with respect to */
    int64_t relbase;            /* Relative base for OUT_RELADDR */
    struct src_location where;  /* Source file and line */
};

/*
 * And a label-definition function. The boolean parameter
 * `is_norm' states whether the label is a `normal' label (which
 * should affect the local-label system), or something odder like
 * an EQU or a segment-base symbol, which shouldn't.
 */
typedef void (*ldfunc)(char *label, int32_t segment, int64_t offset,
                       char *special, bool is_norm);

/*
 * Token types returned by the scanner, in addition to ordinary
 * ASCII character values, and zero for end-of-string.
 */
enum token_type { /* token types, other than chars */

    /* Token values shared between assembler and preprocessor */

    /* Special codes */
    TOKEN_INVALID = -1, /* a placeholder value */
    TOKEN_BLOCK   = -2, /* used for storage management */
    TOKEN_FREE    = -3, /* free token marker, use to catch leaks */
    TOKEN_EOS     = 0,  /* end of string */

    /*
     * Single-character operators. Enumerated here to keep strict
     * compilers happy, and for documentation.
     */
    TOKEN_WHITESPACE = ' ',     /* Preprocessor use */
    TOKEN_BOOL_NOT   = '!',
    TOKEN_AND        = '&',
    TOKEN_OR         = '|',
    TOKEN_XOR        = '^',
    TOKEN_NOT        = '~',
    TOKEN_MULT       = '*',
    TOKEN_DIV        = '/',
    TOKEN_MOD        = '%',
    TOKEN_LPAR       = '(',
    TOKEN_RPAR       = ')',
    TOKEN_PLUS       = '+',
    TOKEN_MINUS      = '-',
    TOKEN_COMMA      = ',',
    TOKEN_LBRACE     = '{',
    TOKEN_RBRACE     = '}',
    TOKEN_LBRACKET   = '[',
    TOKEN_RBRACKET   = ']',
    TOKEN_QMARK      = '?',
    TOKEN_EQ         = '=',     /* = or == */
    TOKEN_GT         = '>',
    TOKEN_LT         = '<',

    /* Multi-character operators */
    TOKEN_SHL = 256,    /* << or <<< */
    TOKEN_SHR,          /* >> */
    TOKEN_SAR,          /* >>> */
    TOKEN_SDIV,         /* // */
    TOKEN_SMOD,         /* %% */
    TOKEN_GE,           /* >= */
    TOKEN_LE,           /* <= */
    TOKEN_NE,           /* <> (!= is same as <>) */
    TOKEN_LEG,          /* <=> */
    TOKEN_DBL_AND,      /* && */
    TOKEN_DBL_OR,       /* || */
    TOKEN_DBL_XOR,      /* ^^ */

    TOKEN_MAX_OPERATOR,

    TOKEN_NUM,          /* numeric constant */
    TOKEN_ERRNUM,       /* malformed numeric constant */
    TOKEN_STR,          /* string constant */
    TOKEN_ERRSTR,       /* unterminated string constant */
    TOKEN_ID,           /* identifier */
    TOKEN_FLOAT,        /* floating-point constant */
    TOKEN_HERE,         /* $, not '$' because it is not an operator */
    TOKEN_BASE,         /* $$ */

    /* Token values only used by the assembler */
    TOKEN_START_ASM,

    TOKEN_SEG,          /* SEG */
    TOKEN_WRT,          /* WRT */
    TOKEN_FLOATIZE,     /* __?floatX?__ */
    TOKEN_STRFUNC,      /* __utf16*__, __utf32*__ */
    TOKEN_IFUNC,        /* __ilog2*__ */
    TOKEN_DECORATOR,    /* decorators such as {...} */
    TOKEN_MASM_PTR,     /* __?masm_ptr?__ for the masm package */
    TOKEN_MASM_FLAT,    /* __?masm_flat?__ for the masm package */
    TOKEN_OPMASK,       /* translated token for opmask registers */
    TOKEN_SIZE,		/* BYTE, WORD, DWORD, QWORD, etc */
    TOKEN_SPECIAL,      /* REL, FAR, NEAR, STRICT, NOSPLIT, etc */
    TOKEN_PREFIX,       /* A32, O16, LOCK, REPNZ, TIMES, etc */
    TOKEN_REG,          /* register name */
    TOKEN_INSN,         /* instruction name */

    TOKEN_END_ASM,

    /* Token values only used by the preprocessor */

    TOKEN_START_PP = TOKEN_END_ASM,

    TOKEN_OTHER,           /* % sequence without (current) meaning */
    TOKEN_PREPROC_ID,      /* Preprocessor ID, e.g. %symbol */
    TOKEN_MMACRO_PARAM,    /* MMacro parameter, e.g. %1 */
    TOKEN_LOCAL_SYMBOL,    /* Local symbol, e.g. %%symbol */
    TOKEN_LOCAL_MACRO,     /* Context-local macro, e.g. %$symbol */
    TOKEN_ENVIRON,         /* %! */
    TOKEN_INTERNAL_STR,    /* Unquoted string that should remain so */
    TOKEN_NAKED_STR,       /* Unquoted string that can be re-quoted */
    TOKEN_PREPROC_Q,       /* %? */
    TOKEN_PREPROC_QQ,      /* %?? */
    TOKEN_PREPROC_SQ,      /* %*? */
    TOKEN_PREPROC_SQQ,     /* %*?? */
    TOKEN_PASTE,           /* %+ */
    TOKEN_COND_COMMA,      /* %, */
    TOKEN_INDIRECT,        /* %[...] */
    TOKEN_XDEF_PARAM,      /* Used during %xdefine processing */
    /* smacro parameters starting here; an arbitrary number. */
    TOKEN_SMAC_START_PARAMS,    /* MUST BE LAST IN THE LIST!!! */
    TOKEN_MAX = INT_MAX		/* Keep compiler from reducing the range */
};

/* Must match the fp_formats[] array in asm/floats.c */
enum floatize {
    FLOAT_8,
    FLOAT_16,
    FLOAT_B16,
    FLOAT_32,
    FLOAT_64,
    FLOAT_80M,
    FLOAT_80E,
    FLOAT_128L,
    FLOAT_128H,
    FLOAT_ERR                   /* Invalid format, MUST BE LAST */
};

/* Must match the list in string_transform(), in strfunc.c */
enum strfunc {
    STRFUNC_UTF16,
    STRFUNC_UTF16LE,
    STRFUNC_UTF16BE,
    STRFUNC_UTF32,
    STRFUNC_UTF32LE,
    STRFUNC_UTF32BE
};

enum ifunc {
    IFUNC_ILOG2E,
    IFUNC_ILOG2W,
    IFUNC_ILOG2F,
    IFUNC_ILOG2C
};

size_t string_transform(char *, size_t, char **, enum strfunc);

/*
 * The expression evaluator must be passed a scanner function; a
 * standard scanner is provided as part of nasmlib.c. The
 * preprocessor will use a different one. Scanners, and the
 * token-value structures they return, look like this.
 *
 * The return value from the scanner is always a copy of the
 * `t_type' field in the structure.
 */
struct tokenval {
    char                *t_charptr;
    int64_t             t_integer;
    int64_t             t_inttwo;
    enum token_type     t_type;
    int8_t              t_flag;
    const char		*t_start; /* Pointer to token in input buffer */
    int			t_len;    /* Length of token in input buffer */
};
typedef int (*scanner)(void *private_data, struct tokenval *tv);

struct location {
    int64_t offset;
    int32_t segment;
    int     known;
};
extern struct location location;

/*
 * Expression-evaluator datatype. Expressions, within the
 * evaluator, are stored as an array of these beasts, terminated by
 * a record with type==0. Mostly, it's a vector type: each type
 * denotes some kind of a component, and the value denotes the
 * multiple of that component present in the expression. The
 * exception is the WRT type, whose `value' field denotes the
 * segment to which the expression is relative. These segments will
 * be segment-base types, i.e. either odd segment values or SEG_ABS
 * types. So it is still valid to assume that anything with a
 * `value' field of zero is insignificant.
 */
typedef struct {
    int32_t type;                  /* a register, or EXPR_xxx */
    int64_t value;                 /* must be >= 32 bits */
} expr;

/*
 * Library routines to manipulate expression data types.
 */
bool is_reloc(const expr *vect);
bool is_simple(const expr *vect);
bool is_really_simple(const expr *vect);
bool is_unknown(const expr *vect);
bool is_just_unknown(const expr *vect);
int64_t reloc_value(const expr *vect);
int32_t reloc_seg(const expr *vect);
int32_t reloc_wrt(const expr *vect);
bool is_self_relative(const expr *vect);
void dump_expr(const expr *vect);

/*
 * The evaluator can also return hints about which of two registers
 * used in an expression should be the base register. See also the
 * `operand' structure.
 */
struct eval_hints {
    int64_t base;
    int     type;
};

/*
 * The actual expression evaluator function looks like this. When
 * called, it expects the first token of its expression to already
 * be in `*tv'; if it is not, set tv->t_type to TOKEN_INVALID and
 * it will start by calling the scanner.
 *
 * If a forward reference happens during evaluation, the evaluator
 * must set `*fwref' to true if `fwref' is non-NULL.
 *
 * `critical' is non-zero if the expression may not contain forward
 * references. The evaluator will report its own error if this
 * occurs; if `critical' is 1, the error will be "symbol not
 * defined before use", whereas if `critical' is 2, the error will
 * be "symbol undefined".
 *
 * If `critical' has bit 8 set (in addition to its main value: 0x101
 * and 0x102 correspond to 1 and 2) then an extended expression
 * syntax is recognised, in which relational operators such as =, <
 * and >= are accepted, as well as low-precedence logical operators
 * &&, ^^ and ||.
 *
 * If `hints' is non-NULL, it gets filled in with some hints as to
 * the base register in complex effective addresses.
 */
#define CRITICAL 0x100
typedef expr *(*evalfunc)(scanner sc, void *scprivate,
                          struct tokenval *tv, int *fwref, int critical,
                          struct eval_hints *hints);

/*
 * Special values for expr->type.
 * These come after EXPR_REG_END as defined in regs.h.
 * Expr types : 0 ~ EXPR_REG_END, EXPR_UNKNOWN, EXPR_...., EXPR_RDSAE,
 *              EXPR_SEGBASE ~ EXPR_SEGBASE + SEG_ABS, ...
 */
#define EXPR_UNKNOWN    (EXPR_REG_END+1) /* forward references */
#define EXPR_SIMPLE     (EXPR_REG_END+2)
#define EXPR_WRT        (EXPR_REG_END+3)
#define EXPR_RDSAE      (EXPR_REG_END+4)
#define EXPR_SEGBASE    (EXPR_REG_END+5)

/*
 * preprocessors ought to look like this:
 */

enum preproc_mode {
    PP_NORMAL,                  /* Assembly */
    PP_DEPS,                    /* Dependencies only */
    PP_PREPROC                  /* Preprocessing only */
};

enum preproc_opt {
    PP_TRIVIAL  = 1,            /* Only %line or # directives */
    PP_NOLINE   = 2,            /* Ignore %line and # directives */
    PP_TASM     = 4             /* TASM compatibility hacks */
};

/*
 * Called once at the very start of assembly.
 */
void pp_init(enum preproc_opt opt);

/*
 * Called at the start of a pass; given a file name, the number
 * of the pass, an error reporting function, an evaluator
 * function, and a listing generator to talk to.
 */
void pp_reset(const char *file, enum preproc_mode mode,
              struct strlist *deplist);

/*
 * Called to fetch a line of preprocessed source. The line
 * returned has been malloc'ed, and so should be freed after
 * use.
 */
char *pp_getline(void);

/* Called at the end of each pass. */
void pp_cleanup_pass(void);

/*
 * Called at the end of the assembly session,
 * after cleanup_pass() has been called for the
 * last pass.
 */
void pp_cleanup_session(void);

/* Additional macros specific to output format */
void pp_extra_stdmac(macros_t *macros);

/* Early definitions and undefinitions for macros */
void pp_pre_define(char *definition);
void pp_pre_undefine(char *definition);

/* Include file from command line */
void pp_pre_include(char *fname);

/* Add a command from the command line */
void pp_pre_command(const char *what, char *str);

/* Include path from command line */
void pp_include_path(struct strlist *ipath);

/* Unwind the macro stack when printing an error message */
void pp_error_list_macros(errflags severity);

/* Return true if an error message should be suppressed */
bool pp_suppress_error(errflags severity);

/* List of dependency files */
extern struct strlist *depend_list;

/* TASM mode changes some properties */
extern bool tasm_compatible_mode;

/*
 * inline function to skip past an identifier; returns the first character past
 * the identifier if valid, otherwise NULL.
 */
static inline char *nasm_skip_identifier(const char *str)
{
    const char *p = str;

    if (!nasm_isidstart(*p++)) {
        p = NULL;
    } else {
        while (nasm_isidchar(*p++))
            ;
    }
    return (char *)p;
}

/*
 * Data-type flags that get passed to listing-file routines.
 */
enum {
    LIST_READ,
    LIST_MACRO,
    LIST_INCLUDE,
    LIST_INCBIN,
    LIST_TIMES
};

/*
 * -----------------------------------------------------------
 * Format of the `insn' structure returned from `parser.c' and
 * passed into `assemble.c'
 * -----------------------------------------------------------
 */

/* Verify value to be a valid register */
static inline bool is_register(int reg)
{
    return reg >= EXPR_REG_START && reg < REG_ENUM_LIMIT;
}

/*
 * token flags
 */
#define TFLAG_BRC       (1 << 0)    /* valid only with braces. {1to8}, {rd-sae}, ...*/
#define TFLAG_BRC_OPT   (1 << 1)    /* may or may not have braces. opmasks {k1} */
#define TFLAG_BRC_ANY   (TFLAG_BRC | TFLAG_BRC_OPT)
#define TFLAG_BRDCAST   (1 << 2)    /* broadcasting decorator */
#define TFLAG_WARN	(1 << 3)    /* warning only, treat as ID */
#define TFLAG_DUP	(1 << 4)    /* valid ID but also has context-specific use */

/*
 * REX flags
 */
#define REX_MASK    0x4f    /* Actual REX prefix bits */
#define REX_B       0x01    /* ModRM r/m extension */
#define REX_X       0x02    /* SIB index extension */
#define REX_R       0x04    /* ModRM reg extension */
#define REX_W       0x08    /* 64-bit operand size */
#define REX_L       0x20    /* Use LOCK prefix instead of REX.R */
#define REX_P       0x40    /* REX prefix present/required */
#define REX_H       0x80    /* High register present, REX forbidden */
#define REX_V       0x0100  /* Instruction uses VEX/XOP instead of REX */
#define REX_NH      0x0200  /* Instruction which doesn't use high regs */
#define REX_EV      0x0400  /* Instruction uses EVEX instead of REX */

/*
 * EVEX bit field
 */
#define EVEX_P0MM       0x0f        /* EVEX P[3:0] : Opcode map           */
#define EVEX_P0RP       0x10        /* EVEX P[4] : High-16 reg            */
#define EVEX_P0X        0x40        /* EVEX P[6] : High-16 rm             */
#define EVEX_P1PP       0x03        /* EVEX P[9:8] : Legacy prefix        */
#define EVEX_P1VVVV     0x78        /* EVEX P[14:11] : NDS register       */
#define EVEX_P1W        0x80        /* EVEX P[15] : Osize extension       */
#define EVEX_P2AAA      0x07        /* EVEX P[18:16] : Embedded opmask    */
#define EVEX_P2VP       0x08        /* EVEX P[19] : High-16 NDS reg       */
#define EVEX_P2B        0x10        /* EVEX P[20] : Broadcast / RC / SAE  */
#define EVEX_P2LL       0x60        /* EVEX P[22:21] : Vector length      */
#define EVEX_P2RC       EVEX_P2LL   /* EVEX P[22:21] : Rounding control   */
#define EVEX_P2Z        0x80        /* EVEX P[23] : Zeroing/Merging       */

/*
 * REX_V "classes" (prefixes which behave like VEX)
 */
enum vex_class {
    RV_VEX      = 0,    /* C4/C5 */
    RV_XOP      = 1,    /* 8F */
    RV_EVEX     = 2     /* 62 */
};

/*
 * Note that because segment registers may be used as instruction
 * prefixes, we must ensure the enumerations for prefixes and
 * register names do not overlap.
 */
enum prefixes { /* instruction prefixes */
    P_none = 0,
    PREFIX_ENUM_START = REG_ENUM_LIMIT,
    P_A16 = PREFIX_ENUM_START,
    P_A32,
    P_A64,
    P_ASP,
    P_LOCK,
    P_O16,
    P_O32,
    P_O64,
    P_OSP,
    P_REP,
    P_REPE,
    P_REPNE,
    P_REPNZ,
    P_REPZ,
    P_TIMES,
    P_WAIT,
    P_XACQUIRE,
    P_XRELEASE,
    P_BND,
    P_NOBND,
    P_REX,
    P_EVEX,
    P_VEX,
    P_VEX3,
    P_VEX2,
    PREFIX_ENUM_LIMIT
};

enum ea_flags { /* special EA flags */
    EAF_BYTEOFFS    =   1,  /* force offset part to byte size */
    EAF_WORDOFFS    =   2,  /* force offset part to [d]word size */
    EAF_TIMESTWO    =   4,  /* really do EAX*2 not EAX+EAX */
    EAF_REL         =   8,  /* IP-relative addressing */
    EAF_ABS         =  16,  /* non-IP-relative addressing */
    EAF_FSGS        =  32,  /* fs/gs segment override present */
    EAF_MIB         =  64,  /* mib operand */
    EAF_SIB         = 128   /* SIB encoding obligatory */
};

enum eval_hint { /* values for `hinttype' */
    EAH_NOHINT   = 0,       /* no hint at all - our discretion */
    EAH_MAKEBASE = 1,       /* try to make given reg the base */
    EAH_NOTBASE  = 2,       /* try _not_ to make reg the base */
    EAH_SUMMED   = 3        /* base and index are summed into index */
};

typedef struct operand { /* operand to an instruction */
    opflags_t       type;       /* type of operand */
    opflags_t       xsize;      /* size flags used in find_match() */
    enum reg_enum   basereg;
    enum reg_enum   indexreg;   /* address registers */
    int             scale;      /* index scale */
    int             hintbase;
    enum eval_hint  hinttype;   /* hint as to real base register */
    int32_t         segment;    /* immediate segment, if needed */
    int64_t         offset;     /* any immediate number */
    int32_t         wrt;        /* segment base it's relative to */
    int             eaflags;    /* special EA flags */
    int             opflags;    /* see OPFLAG_* defines below */
    decoflags_t     decoflags;  /* decorator flags such as {...} */
    bool            bcast;      /* broadcast operand */
    uint8_t         disp_size;  /* 0 means default; 16; 32; 64 */
    uint8_t         opidx;      /* Operand index */
} operand;

#define OPFLAG_FORWARD      1   /* operand is a forward reference */
#define OPFLAG_EXTERN       2   /* operand is an external reference */
#define OPFLAG_UNKNOWN      4   /* operand is an unknown reference
                                   (always a forward reference also) */
#define OPFLAG_RELATIVE     8   /* operand is self-relative, e.g. [foo - $]
                                   where foo is not in the current segment */

enum extop_type { /* extended operand types */
    EOT_NOTHING = 0,
    EOT_EXTOP,          /* Subexpression */
    EOT_DB_STRING,      /* Byte string */
    EOT_DB_FLOAT,       /* Floating-pointer number (special byte string) */
    EOT_DB_STRING_FREE, /* Byte string which should be nasm_free'd*/
    EOT_DB_NUMBER,      /* Integer */
    EOT_DB_RESERVE      /* ? */
};

typedef struct extop { /* extended operand */
    struct extop    *next;       /* linked list */
    union {
        struct {                 /* text or byte string */
            char    *data;
            size_t   len;
        } string;
        struct {                 /* numeric expression */
            int64_t  offset;     /* numeric value or address offset */
            int32_t  segment;    /* address segment */
            int32_t  wrt;        /* address wrt */
            bool     relative;   /* self-relative expression */
        } num;
        struct extop *subexpr;   /* actual expressions */
    } val;
    size_t dup;                  /* duplicated? */
    enum extop_type type;        /* defined above */
    int elem;                    /* element size override, if any (bytes) */
} extop;

enum ea_type {
    EA_INVALID,     /* Not a valid EA at all */
    EA_SCALAR,      /* Scalar EA */
    EA_XMMVSIB,     /* XMM vector EA */
    EA_YMMVSIB,     /* YMM vector EA */
    EA_ZMMVSIB      /* ZMM vector EA */
};

/*
 * Prefix positions: each type of prefix goes in a specific slot.
 * This affects the final ordering of the assembled output, which
 * shouldn't matter to the processor, but if you have stylistic
 * preferences, you can change this.  REX prefixes are handled
 * differently for the time being.
 *
 * LOCK and REP used to be one slot; this is no longer the case since
 * the introduction of HLE.
 */
enum prefix_pos {
    PPS_TIMES = -1,     /* TIMES (not a slot, handled separately) */
    PPS_WAIT  =  0,   	/* WAIT (technically not a prefix!) */
    PPS_REP,    	/* REP/HLE prefix */
    PPS_LOCK,   	/* LOCK prefix */
    PPS_SEG,    	/* Segment override prefix */
    PPS_OSIZE,  	/* Operand size prefix */
    PPS_ASIZE,  	/* Address size prefix */
    PPS_REX,    	/* REX/VEX type */
    MAXPREFIX   	/* Total number of prefix slots */
};

/*
 * Tuple types that are used when determining Disp8*N eligibility
 * The order must match with a hash %tuple_codes in insns.pl
 */
enum ttypes {
    FV    = 001,
    HV    = 002,
    FVM   = 003,
    T1S8  = 004,
    T1S16 = 005,
    T1S   = 006,
    T1F32 = 007,
    T1F64 = 010,
    T2    = 011,
    T4    = 012,
    T8    = 013,
    HVM   = 014,
    QVM   = 015,
    OVM   = 016,
    M128  = 017,
    DUP   = 020
};

/* EVEX.L'L : Vector length on vector insns */
enum vectlens {
    VL128 = 0,
    VL256 = 1,
    VL512 = 2,
    VLMAX = 3
};

/* If you need to change this, also change it in insns.pl */
#define MAX_OPERANDS 5

typedef struct insn { /* an instruction itself */
    char            *label;                 /* the label defined, or NULL */
    int             prefixes[MAXPREFIX];    /* instruction prefixes, if any */
    enum opcode     opcode;                 /* the opcode - not just the string */
    int             operands;               /* how many operands? 0-3 (more if db et al) */
    int             addr_size;              /* address size */
    operand         oprs[MAX_OPERANDS];     /* the operands, defined as above */
    extop           *eops;                  /* extended operands */
    int             eops_float;             /* true if DD and floating */
    int32_t         times;                  /* repeat count (TIMES prefix) */
    bool            rex_done;               /* REX prefix emitted? */
    int             rex;                    /* Special REX Prefix */
    int             vexreg;                 /* Register encoded in VEX prefix */
    int             vex_cm;                 /* Class and M field for VEX prefix */
    int             vex_wlp;                /* W, P and L information for VEX prefix */
    uint8_t         evex_p[3];              /* EVEX.P0: [RXB,R',00,mm], P1: [W,vvvv,1,pp] */
                                            /* EVEX.P2: [z,L'L,b,V',aaa] */
    enum ttypes     evex_tuple;             /* Tuple type for compressed Disp8*N */
    int             evex_rm;                /* static rounding mode for AVX512 (EVEX) */
    struct operand *evex_brerop;            /* BR/ER/SAE operand position */
} insn;

/* Instruction flags type: IF_* flags are defined in insns.h */
typedef uint64_t iflags_t;

/*
 * What to return from a directive- or pragma-handling function.
 * Currently DIRR_OK and DIRR_ERROR are treated the same way;
 * in both cases the backend is expected to produce the appropriate
 * error message on its own.
 *
 * DIRR_BADPARAM causes a generic error message to be printed.  Note
 * that it is an error, not a warning, even in the case of pragmas;
 * don't use it where forward compatibility would be compromised
 * (instead consider adding a DIRR_WARNPARAM.)
 */
enum directive_result {
    DIRR_UNKNOWN,               /* Directive not handled by backend */
    DIRR_OK,                    /* Directive processed */
    DIRR_ERROR,                 /* Directive processed unsuccessfully */
    DIRR_BADPARAM               /* Print bad argument error message */
};

/*
 * A pragma facility: this structure is used to request passing a
 * parsed pragma directive for a specific facility.  If the handler is
 * NULL then this pragma facility is recognized but ignored; pragma
 * processing stops at that point.
 *
 * Note that the handler is passed a pointer to the facility structure
 * as part of the struct pragma.
 */
struct pragma;
typedef enum directive_result (*pragma_handler)(const struct pragma *);

struct pragma_facility {
    const char *name;
    pragma_handler handler;
};

/*
 * This structure defines how a pragma directive is passed to a
 * facility.  This structure may be augmented in the future.
 *
 * Any facility MAY, but is not required to, add its operations
 * keywords or a subset thereof into asm/directiv.dat, in which case
 * the "opcode" field will be set to the corresponding D_ constant
 * from directiv.h; otherwise it will be D_unknown.
 */
struct pragma {
    const struct pragma_facility *facility;
    const char *facility_name;  /* Facility name exactly as entered by user */
    const char *opname;         /* First word after the facility name */
    const char *tail;           /* Anything after the operation */
    enum directive opcode;     /* Operation as a D_ directives constant */
};

/*
 * These are semi-arbitrary limits to keep the assembler from going
 * into a black hole on certain kinds of bugs.  They can be overridden
 * by command-line options or %pragma.
 */
enum nasm_limit {
    LIMIT_PASSES,
    LIMIT_STALLED,
    LIMIT_MACRO_LEVELS,
    LIMIT_MACRO_TOKENS,
    LIMIT_MMACROS,
    LIMIT_REP,
    LIMIT_EVAL,
    LIMIT_LINES
};
#define LIMIT_MAX LIMIT_LINES
extern int64_t nasm_limit[LIMIT_MAX+1];
extern enum directive_result  nasm_set_limit(const char *, const char *);

/*
 * The data structure defining an output format driver, and the
 * interfaces to the functions therein.
 */
struct ofmt {
    /*
     * This is a short (one-liner) description of the type of
     * output generated by the driver.
     */
    const char *fullname;

    /*
     * This is a single keyword used to select the driver.
     */
    const char *shortname;

    /*
     * Default output filename extension, or a null string
     */
    const char *extension;

    /*
     * Output format flags.
     */
#define OFMT_TEXT		1	/* Text file format */
#define OFMT_KEEP_ADDR	2	/* Keep addr; no conversion to data */

    unsigned int flags;

    int maxbits;                /* Maximum segment bits supported */

    /*
     * this is a pointer to the first element of the debug information
     */
    const struct dfmt * const *debug_formats;

    /*
     * the default debugging format if -F is not specified
     */
    const struct dfmt *default_dfmt;

    /*
     * This, if non-NULL, is a NULL-terminated list of `char *'s
     * pointing to extra standard macros supplied by the object
     * format (e.g. a sensible initial default value of __?SECT?__,
     * and user-level equivalents for any format-specific
     * directives).
     */
    macros_t *stdmac;

    /*
     * This procedure is called at the start of an output session to set
     * up internal parameters.
     */
    void (*init)(void);

    /*
     * This procedure is called at the start of each pass.
     */
    void (*reset)(void);

    /*
     * This is the modern output function, which gets passed
     * a struct out_data with much more information.  See the
     * definition of struct out_data.
     */
    void (*output)(const struct out_data *data);

    /*
     * This procedure is called by assemble() to write actual
     * generated code or data to the object file. Typically it
     * doesn't have to actually _write_ it, just store it for
     * later.
     *
     * The `type' argument specifies the type of output data, and
     * usually the size as well: its contents are described below.
     *
     * This is used for backends which have not yet been ported to
     * the new interface, and should be NULL on ported backends.
     * To use this entry point, set the output pointer to
     * nasm_do_legacy_output.
     */
    void (*legacy_output)(int32_t segto, const void *data,
                          enum out_type type, uint64_t size,
                          int32_t segment, int32_t wrt);

    /*
     * This procedure is called once for every symbol defined in
     * the module being assembled. It gives the name and value of
     * the symbol, in NASM's terms, and indicates whether it has
     * been declared to be global. Note that the parameter "name",
     * when passed, will point to a piece of static storage
     * allocated inside the label manager - it's safe to keep using
     * that pointer, because the label manager doesn't clean up
     * until after the output driver has.
     *
     * Values of `is_global' are: 0 means the symbol is local; 1
     * means the symbol is global; 2 means the symbol is common (in
     * which case `offset' holds the _size_ of the variable).
     * Anything else is available for the output driver to use
     * internally.
     *
     * This routine explicitly _is_ allowed to call the label
     * manager to define further symbols, if it wants to, even
     * though it's been called _from_ the label manager. That much
     * re-entrancy is guaranteed in the label manager. However, the
     * label manager will in turn call this routine, so it should
     * be prepared to be re-entrant itself.
     *
     * The `special' parameter contains special information passed
     * through from the command that defined the label: it may have
     * been an EXTERN, a COMMON or a GLOBAL. The distinction should
     * be obvious to the output format from the other parameters.
     */
    void (*symdef)(char *name, int32_t segment, int64_t offset,
                   int is_global, char *special);

    /*
     * This procedure is called when the source code requests a
     * segment change. It should return the corresponding segment
     * _number_ for the name, or NO_SEG if the name is not a valid
     * segment name.
     *
     * It may also be called with NULL, in which case it is to
     * return the _default_ section number for starting assembly in.
     *
     * It is allowed to modify the string it is given a pointer to.
     *
     * It is also allowed to specify a default instruction size for
     * the segment, by setting `*bits' to 16, 32 or 64. Or, if it
     * doesn't wish to define a default, it can leave `bits' alone.
     */
    int32_t (*section)(char *name, int *bits);

    /*
     * This function is called when a label is defined
     * in the source code. It is allowed to change the section
     * number as a result, but not the bits value.
     * This is *only* called if the symbol defined is at the
     * current offset, i.e. "foo:" or "foo equ $".
     * The offset isn't passed; and may not be stable at this point.
     * The subsection number is a field available for use by the
     * backend. It is initialized to NO_SEG.
     *
     * If "copyoffset" is set by the backend then the offset is
     * copied from the previous segment, otherwise the new segment
     * is treated as a new segment the normal way.
     */
    int32_t (*herelabel)(const char *name, enum label_type type,
                         int32_t seg, int32_t *subsection,
                         bool *copyoffset);

    /*
     * This procedure is called to modify section alignment,
     * note there is a trick, the alignment can only increase
     */
    void (*sectalign)(int32_t seg, unsigned int value);

    /*
     * This procedure is called to modify the segment base values
     * returned from the SEG operator. It is given a segment base
     * value (i.e. a segment value with the low bit set), and is
     * required to produce in return a segment value which may be
     * different. It can map segment bases to absolute numbers by
     * means of returning SEG_ABS types.
     *
     * It should return NO_SEG if the segment base cannot be
     * determined; the evaluator (which calls this routine) is
     * responsible for throwing an error condition if that occurs
     * in pass two or in a critical expression.
     */
    int32_t (*segbase)(int32_t segment);

    /*
     * This procedure is called to allow the output driver to
     * process its own specific directives. When called, it has the
     * directive word in `directive' and the parameter string in
     * `value'.
     *
     * The following values are (currently) possible for
     * directive_result:
     *
     * 0 - DIRR_UNKNOWN		- directive not recognized by backend
     * 1 - DIRR_OK		- directive processed ok
     * 2 - DIRR_ERROR		- backend printed its own error message
     * 3 - DIRR_BADPARAM	- print the generic message
     *				  "invalid parameter to [*] directive"
     */
    enum directive_result
    (*directive)(enum directive directive, char *value);

    /*
     * This procedure is called after assembly finishes, to allow
     * the output driver to clean itself up and free its memory.
     * Typically, it will also be the point at which the object
     * file actually gets _written_.
     *
     * One thing the cleanup routine should always do is to close
     * the output file pointer.
     */
    void (*cleanup)(void);

    /*
     * List of pragma facility names that apply to this backend.
     */
    const struct pragma_facility *pragmas;
};

/*
 * Output format driver alias
 */
struct ofmt_alias {
    const char  *shortname;
    const struct ofmt *ofmt;
};

extern const struct ofmt *ofmt;
extern FILE *ofile;

/*
 * ------------------------------------------------------------
 * The data structure defining a debug format driver, and the
 * interfaces to the functions therein.
 * ------------------------------------------------------------
 */
struct debug_macro_info;

struct dfmt {
    /*
     * This is a short (one-liner) description of the type of
     * output generated by the driver.
     */
    const char *fullname;

    /*
     * This is a single keyword used to select the driver.
     */
    const char *shortname;

    /*
     * init - called initially to set up local pointer to object format.
     */
    void (*init)(void);

    /*
     * linenum - called any time there is output with a change of
     * line number or file.
     */
    void (*linenum)(const char *filename, int32_t linenumber, int32_t segto);

    /*
     * debug_deflabel - called whenever a label is defined. Parameters
     * are the same as to 'symdef()' in the output format. This function
     * is called after the output format version.
     */

    void (*debug_deflabel)(char *name, int32_t segment, int64_t offset,
                           int is_global, char *special);

    /*
     * debug_smacros - called when an smacro is defined or undefined
     * during the code-generation pass. The definition string contains
     * the macro name, any arguments, a single space, and the macro
     * definition; this is what is expected by e.g. DWARF.
     *
     * The definition is provided even for an undef.
     */
    void (*debug_smacros)(bool define, const char *def);

    /*
     * debug_include - called when a file is included or the include
     * is finished during the code-generation pass.  The filename is
     * kept by the srcfile system and so can be compared for pointer
     * equality.
     *
     * A filename of NULL means builtin (initial or %use) or command
     * line statements.
     */
    void (*debug_include)(bool start, struct src_location outer,
                          struct src_location inner);

    /*
     * debug_mmacros - called once at the end with a definition for each
     * non-.nolist macro that has been invoked at least once in the program,
     * and the corresponding address ranges. See dbginfo.h.
     */
    void (*debug_mmacros)(const struct debug_macro_info *);

    /*
     * debug_directive - called whenever a DEBUG directive other than 'LINE'
     * is encountered. 'directive' contains the first parameter to the
     * DEBUG directive, and params contains the rest. For example,
     * 'DEBUG VAR _somevar:int' would translate to a call to this
     * function with 'directive' equal to "VAR" and 'params' equal to
     * "_somevar:int".
     */
    void (*debug_directive)(const char *directive, const char *params);

    /*
     * typevalue - called whenever the assembler wishes to register a type
     * for the last defined label.  This routine MUST detect if a type was
     * already registered and not re-register it.
     */
    void (*debug_typevalue)(int32_t type);

    /*
     * debug_output - called whenever output is required
     * 'type' is the type of info required, and this is format-specific
     */
    void (*debug_output)(int type, void *param);

    /*
     * cleanup - called after processing of file is complete
     */
    void (*cleanup)(void);

    /*
     * List of pragma facility names that apply to this backend.
     */
    const struct pragma_facility *pragmas;
};

extern const struct dfmt *dfmt;

/*
 * The type definition macros
 * for debugging
 *
 * low 3 bits: reserved
 * next 5 bits: type
 * next 24 bits: number of elements for arrays (0 for labels)
 */

#define TY_UNKNOWN 0x00
#define TY_LABEL   0x08
#define TY_BYTE    0x10
#define TY_WORD    0x18
#define TY_DWORD   0x20
#define TY_FLOAT   0x28
#define TY_QWORD   0x30
#define TY_TBYTE   0x38
#define TY_OWORD   0x40
#define TY_YWORD   0x48
#define TY_ZWORD   0x50
#define TY_COMMON  0xE0
#define TY_SEG     0xE8
#define TY_EXTERN  0xF0
#define TY_EQU     0xF8

#define TYM_TYPE(x)     ((x) & 0xF8)
#define TYM_ELEMENTS(x) (((x) & 0xFFFFFF00) >> 8)

#define TYS_ELEMENTS(x) ((x) << 8)

/* Sizes corresponding to various tokens */
enum byte_sizes {
    SIZE_BYTE	=  1,
    SIZE_WORD	=  2,
    SIZE_DWORD	=  4,
    SIZE_QWORD	=  8,
    SIZE_TWORD  = 10,
    SIZE_OWORD  = 16,
    SIZE_YWORD  = 32,
    SIZE_ZWORD  = 64
};

enum special_tokens {
    SIZE_ENUM_START     = PREFIX_ENUM_LIMIT,
    S_BYTE              = SIZE_ENUM_START,
    S_WORD,
    S_DWORD,
    S_QWORD,
    S_TWORD,
    S_OWORD,
    S_YWORD,
    S_ZWORD,
    SIZE_ENUM_LIMIT,

    SPECIAL_ENUM_START  = SIZE_ENUM_LIMIT,
    S_ABS		= SPECIAL_ENUM_START,
    S_FAR,
    S_LONG,
    S_NEAR,
    S_NOSPLIT,
    S_REL,
    S_SHORT,
    S_STRICT,
    S_TO,
    SPECIAL_ENUM_LIMIT
};

enum decorator_tokens {
    DECORATOR_ENUM_START    = SPECIAL_ENUM_LIMIT,
    BRC_1TO2                = DECORATOR_ENUM_START,
    BRC_1TO4,
    BRC_1TO8,
    BRC_1TO16,
    BRC_1TO32,
    BRC_RN,
    BRC_RD,
    BRC_RU,
    BRC_RZ,
    BRC_SAE,
    BRC_Z,
    DECORATOR_ENUM_LIMIT
};

/*
 * AVX512 Decorator (decoflags_t) bits distribution (counted from 0)
 *  3         2         1
 * 10987654321098765432109876543210
 *                |
 *                | word boundary
 * ............................1111 opmask
 * ...........................1.... zeroing / merging
 * ..........................1..... broadcast
 * .........................1...... static rounding
 * ........................1....... SAE
 * ....................1111........ broadcast element size
 * .................111............ number of broadcast elements
 */
#define OP_GENVAL(val, bits, shift)     (((val) & ((UINT64_C(1) << (bits)) - 1)) << (shift))

/*
 * Opmask register number
 * identical to EVEX.aaa
 *
 * Bits: 0 - 3
 */
#define OPMASK_SHIFT            (0)
#define OPMASK_BITS             (4)
#define OPMASK_MASK             OP_GENMASK(OPMASK_BITS, OPMASK_SHIFT)
#define GEN_OPMASK(bit)         OP_GENBIT(bit, OPMASK_SHIFT)
#define VAL_OPMASK(val)         OP_GENVAL(val, OPMASK_BITS, OPMASK_SHIFT)

/*
 * zeroing / merging control available
 * matching to EVEX.z
 *
 * Bits: 4
 */
#define Z_SHIFT                 (4)
#define Z_BITS                  (1)
#define Z_MASK                  OP_GENMASK(Z_BITS, Z_SHIFT)
#define GEN_Z(bit)              OP_GENBIT(bit, Z_SHIFT)

/*
 * broadcast - Whether this operand can be broadcasted
 *
 * Bits: 5
 */
#define BRDCAST_SHIFT           (5)
#define BRDCAST_BITS            (1)
#define BRDCAST_MASK            OP_GENMASK(BRDCAST_BITS, BRDCAST_SHIFT)
#define GEN_BRDCAST(bit)        OP_GENBIT(bit, BRDCAST_SHIFT)

/*
 * Whether this instruction can have a static rounding mode.
 * It goes with the last simd operand because the static rounding mode
 * decorator is located between the last simd operand and imm8 (if any).
 *
 * Bits: 6
 */
#define STATICRND_SHIFT         (6)
#define STATICRND_BITS          (1)
#define STATICRND_MASK          OP_GENMASK(STATICRND_BITS, STATICRND_SHIFT)
#define GEN_STATICRND(bit)      OP_GENBIT(bit, STATICRND_SHIFT)

/*
 * SAE(Suppress all exception) available
 *
 * Bits: 7
 */
#define SAE_SHIFT               (7)
#define SAE_BITS                (1)
#define SAE_MASK                OP_GENMASK(SAE_BITS, SAE_SHIFT)
#define GEN_SAE(bit)            OP_GENBIT(bit, SAE_SHIFT)

/*
 * Broadcasting element size.
 *
 * Bits: 8 - 11
 */
#define BRSIZE_SHIFT            (8)
#define BRSIZE_BITS             (4)
#define BRSIZE_MASK             OP_GENMASK(BRSIZE_BITS, BRSIZE_SHIFT)
#define GEN_BRSIZE(bit)         OP_GENBIT(bit, BRSIZE_SHIFT)

#define BR_BITS8		GEN_BRSIZE(0) /* For potential future use */
#define BR_BITS16               GEN_BRSIZE(1)
#define BR_BITS32               GEN_BRSIZE(2)
#define BR_BITS64               GEN_BRSIZE(3)

/*
 * Number of broadcasting elements
 *
 * Bits: 12 - 14
 */
#define BRNUM_SHIFT             (12)
#define BRNUM_BITS              (3)
#define BRNUM_MASK              OP_GENMASK(BRNUM_BITS, BRNUM_SHIFT)
#define VAL_BRNUM(val)          OP_GENVAL(val, BRNUM_BITS, BRNUM_SHIFT)

#define BR_1TO2                 VAL_BRNUM(0)
#define BR_1TO4                 VAL_BRNUM(1)
#define BR_1TO8                 VAL_BRNUM(2)
#define BR_1TO16                VAL_BRNUM(3)
#define BR_1TO32                VAL_BRNUM(4)
#define BR_1TO64                VAL_BRNUM(5) /* For potential future use */

#define MASK                    OPMASK_MASK             /* Opmask (k1 ~ 7) can be used */
#define Z                       Z_MASK
#define B16                     (BRDCAST_MASK|BR_BITS16) /* {1to32} : broadcast 16b * 32 to zmm(512b) */
#define B32                     (BRDCAST_MASK|BR_BITS32) /* {1to16} : broadcast 32b * 16 to zmm(512b) */
#define B64                     (BRDCAST_MASK|BR_BITS64) /* {1to8}  : broadcast 64b *  8 to zmm(512b) */
#define ER                      STATICRND_MASK          /* ER(Embedded Rounding) == Static rounding mode */
#define SAE                     SAE_MASK                /* SAE(Suppress All Exception) */

/*
 * Broadcast flags (BR_BITS*) to sizes (BITS*)
 */
static inline opflags_t brsize_to_size(opflags_t brbits)
{
    return (brbits & BRSIZE_MASK) << (SIZE_SHIFT - BRSIZE_SHIFT);
}

/*
 * Global modes
 */

/*
 * flag to disable optimizations selectively
 * this is useful to turn-off certain optimizations
 */
enum optimization_disable_flag {
    OPTIM_ALL_ENABLED       = 0,
    OPTIM_DISABLE_JMP_MATCH = 1
};

struct optimization {
    int level;
    int flag;
};

/*
 * Various types of compiler passes we may execute.
 * If these are changed, you need to also change _pass_types[]
 * in asm/nasm.c.
 */
enum pass_type {
    PASS_INIT,            /* Initialization, not doing anything yet */
    PASS_PREPROC,         /* Preprocess-only mode (similar to PASS_FIRST) */
    PASS_FIRST,           /* The very first pass over the code */
    PASS_OPT,             /* Optimization pass */
    PASS_STAB,            /* Stabilization pass (original pass 1) */
    PASS_FINAL            /* Code generation pass (original pass 2) */
};
extern const char * const _pass_types[];
extern enum pass_type _pass_type;
static inline enum pass_type pass_type(void)
{
    return _pass_type;
}
static inline const char *pass_type_name(void)
{
    return _pass_types[_pass_type];
}
/* True during initialization, no code read yet */
static inline bool not_started(void)
{
    return pass_type() == PASS_INIT;
}
/* True for the initial pass and setup (old "pass2 < 2") */
static inline bool pass_first(void)
{
    return pass_type() <= PASS_FIRST;
}
/* At this point we better have stable definitions */
static inline bool pass_stable(void)
{
    return pass_type() >= PASS_STAB;
}
/* True for the code generation pass only, (old "pass1 >= 2") */
static inline bool pass_final(void)
{
    return pass_type() >= PASS_FINAL;
}
/* True for code generation *or* preprocess-only mode */
static inline bool pass_final_or_preproc(void)
{
    return pass_type() >= PASS_FINAL || pass_type() == PASS_PREPROC;
}

/*
 * The actual pass number. 0 is used during initialization, the very
 * first pass is 1, and then it is simply increasing numbers until we are
 * done.
 */
extern int64_t _passn;           /* Actual pass number */
static inline int64_t pass_count(void)
{
    return _passn;
}

extern struct optimization optimizing;
extern int globalbits;          /* 16, 32 or 64-bit mode */
extern int globalrel;           /* default to relative addressing? */
extern int globalbnd;           /* default to using bnd prefix? */

extern const char *inname;	/* primary input filename */
extern const char *outname;     /* output filename */

/*
 * Switch to a different segment and return the current offset
 */
int64_t switch_segment(int32_t segment);

#endif  /* NASM_NASM_H */
