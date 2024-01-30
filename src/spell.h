/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * spell.h: common code for spell checking, used by spell.c and spellfile.c.
 */

// Use SPELL_PRINTTREE for debugging: dump the word tree after adding a word.
// Only use it for small word lists!
#if 0
# define SPELL_PRINTTREE
#endif

// Use SPELL_COMPRESS_ALWAYS for debugging: compress the word tree after
// adding a word.  Only use it for small word lists!
#if 0
# define SPELL_COMPRESS_ALWAYS
#endif

// Use DEBUG_TRIEWALK to print the changes made in suggest_trie_walk() for a
// specific word.
#if 0
# define DEBUG_TRIEWALK
#endif

#define MAXWLEN 254		// Assume max. word len is this many bytes.
				// Some places assume a word length fits in a
				// byte, thus it can't be above 255.
				// Must be >= PFD_NOTSPECIAL.

#define MAXREGIONS 8		// Number of regions supported.

// Type used for indexes in the word tree need to be at least 4 bytes.  If int
// is 8 bytes we could use something smaller, but what?
typedef int idx_T;

typedef int salfirst_T;

/*
 * Structure used to store words and other info for one language, loaded from
 * a .spl file.
 * The main access is through the tree in "sl_fbyts/sl_fidxs", storing the
 * case-folded words.  "sl_kbyts/sl_kidxs" is for keep-case words.
 *
 * The "byts" array stores the possible bytes in each tree node, preceded by
 * the number of possible bytes, sorted on byte value:
 *	<len> <byte1> <byte2> ...
 * The "idxs" array stores the index of the child node corresponding to the
 * byte in "byts".
 * Exception: when the byte is zero, the word may end here and "idxs" holds
 * the flags, region mask and affixID for the word.  There may be several
 * zeros in sequence for alternative flag/region/affixID combinations.
 */
typedef struct slang_S slang_T;
struct slang_S
{
    slang_T	*sl_next;	// next language
    char_u	*sl_name;	// language name "en", "en.rare", "nl", etc.
    char_u	*sl_fname;	// name of .spl file
    int		sl_add;		// TRUE if it's a .add file.

    char_u	*sl_fbyts;	// case-folded word bytes
    long	sl_fbyts_len;	// length of sl_fbyts
    idx_T	*sl_fidxs;	// case-folded word indexes
    char_u	*sl_kbyts;	// keep-case word bytes
    idx_T	*sl_kidxs;	// keep-case word indexes
    char_u	*sl_pbyts;	// prefix tree word bytes
    idx_T	*sl_pidxs;	// prefix tree word indexes

    char_u	*sl_info;	// infotext string or NULL

    char_u	sl_regions[MAXREGIONS * 2 + 1];
				// table with up to 8 region names plus NUL

    char_u	*sl_midword;	// MIDWORD string or NULL

    hashtab_T	sl_wordcount;	// hashtable with word count, wordcount_T

    int		sl_compmax;	// COMPOUNDWORDMAX (default: MAXWLEN)
    int		sl_compminlen;	// COMPOUNDMIN (default: 0)
    int		sl_compsylmax;	// COMPOUNDSYLMAX (default: MAXWLEN)
    int		sl_compoptions;	// COMP_* flags
    garray_T	sl_comppat;	// CHECKCOMPOUNDPATTERN items
    regprog_T	*sl_compprog;	// COMPOUNDRULE turned into a regexp progrm
				// (NULL when no compounding)
    char_u	*sl_comprules;	// all COMPOUNDRULE concatenated (or NULL)
    char_u	*sl_compstartflags; // flags for first compound word
    char_u	*sl_compallflags; // all flags for compound words
    char_u	sl_nobreak;	// When TRUE: no spaces between words
    char_u	*sl_syllable;	// SYLLABLE repeatable chars or NULL
    garray_T	sl_syl_items;	// syllable items

    int		sl_prefixcnt;	// number of items in "sl_prefprog"
    regprog_T	**sl_prefprog;	// table with regprogs for prefixes

    garray_T	sl_rep;		// list of fromto_T entries from REP lines
    short	sl_rep_first[256];  // indexes where byte first appears, -1 if
				    // there is none
    garray_T	sl_sal;		// list of salitem_T entries from SAL lines
    salfirst_T	sl_sal_first[256];  // indexes where byte first appears, -1 if
				    // there is none
    int		sl_followup;	// SAL followup
    int		sl_collapse;	// SAL collapse_result
    int		sl_rem_accents;	// SAL remove_accents
    int		sl_sofo;	// SOFOFROM and SOFOTO instead of SAL items:
				// "sl_sal_first" maps chars, when has_mbyte
				// "sl_sal" is a list of wide char lists.
    garray_T	sl_repsal;	// list of fromto_T entries from REPSAL lines
    short	sl_repsal_first[256];  // sl_rep_first for REPSAL lines
    int		sl_nosplitsugs;	// don't suggest splitting a word
    int		sl_nocompoundsugs; // don't suggest compounding

    // Info from the .sug file.  Loaded on demand.
    time_t	sl_sugtime;	// timestamp for .sug file
    char_u	*sl_sbyts;	// soundfolded word bytes
    idx_T	*sl_sidxs;	// soundfolded word indexes
    buf_T	*sl_sugbuf;	// buffer with word number table
    int		sl_sugloaded;	// TRUE when .sug file was loaded or failed to
				// load

    int		sl_has_map;	// TRUE if there is a MAP line
    hashtab_T	sl_map_hash;	// MAP for multi-byte chars
    int		sl_map_array[256]; // MAP for first 256 chars
    hashtab_T	sl_sounddone;	// table with soundfolded words that have
				// handled, see add_sound_suggest()
};

#ifdef VMS
# define SPL_FNAME_TMPL  "%s_%s.spl"
# define SPL_FNAME_ADD   "_add."
# define SPL_FNAME_ASCII "_ascii."
#else
# define SPL_FNAME_TMPL  "%s.%s.spl"
# define SPL_FNAME_ADD   ".add."
# define SPL_FNAME_ASCII ".ascii."
#endif

// Flags used for a word.  Only the lowest byte can be used, the region byte
// comes above it.
#define WF_REGION   0x01	// region byte follows
#define WF_ONECAP   0x02	// word with one capital (or all capitals)
#define WF_ALLCAP   0x04	// word must be all capitals
#define WF_RARE	    0x08	// rare word
#define WF_BANNED   0x10	// bad word
#define WF_AFX	    0x20	// affix ID follows
#define WF_FIXCAP   0x40	// keep-case word, allcap not allowed
#define WF_KEEPCAP  0x80	// keep-case word

#define WF_CAPMASK (WF_ONECAP | WF_ALLCAP | WF_KEEPCAP | WF_FIXCAP)

// for <flags2>, shifted up one byte to be used in wn_flags
#define WF_HAS_AFF  0x0100	// word includes affix
#define WF_NEEDCOMP 0x0200	// word only valid in compound
#define WF_NOSUGGEST 0x0400	// word not to be suggested
#define WF_COMPROOT 0x0800	// already compounded word, COMPOUNDROOT
#define WF_NOCOMPBEF 0x1000	// no compounding before this word
#define WF_NOCOMPAFT 0x2000	// no compounding after this word

// flags for <pflags>
#define WFP_RARE	    0x01	// rare prefix
#define WFP_NC		    0x02	// prefix is not combining
#define WFP_UP		    0x04	// to-upper prefix
#define WFP_COMPPERMIT	    0x08	// prefix with COMPOUNDPERMITFLAG
#define WFP_COMPFORBID	    0x10	// prefix with COMPOUNDFORBIDFLAG

// Flags for postponed prefixes in "sl_pidxs".  Must be above affixID (one
// byte) and prefcondnr (two bytes).
#define WF_RAREPFX  (WFP_RARE << 24)	// rare postponed prefix
#define WF_PFX_NC   (WFP_NC << 24)	// non-combining postponed prefix
#define WF_PFX_UP   (WFP_UP << 24)	// to-upper postponed prefix
#define WF_PFX_COMPPERMIT (WFP_COMPPERMIT << 24) // postponed prefix with
						 // COMPOUNDPERMITFLAG
#define WF_PFX_COMPFORBID (WFP_COMPFORBID << 24) // postponed prefix with
						 // COMPOUNDFORBIDFLAG

// flags for <compoptions>
#define COMP_CHECKDUP		1	// CHECKCOMPOUNDDUP
#define COMP_CHECKREP		2	// CHECKCOMPOUNDREP
#define COMP_CHECKCASE		4	// CHECKCOMPOUNDCASE
#define COMP_CHECKTRIPLE	8	// CHECKCOMPOUNDTRIPLE

// Info from "REP", "REPSAL" and "SAL" entries in ".aff" file used in si_rep,
// si_repsal, sl_rep, and si_sal.  Not for sl_sal!
// One replacement: from "ft_from" to "ft_to".
typedef struct fromto_S
{
    char_u	*ft_from;
    char_u	*ft_to;
} fromto_T;

// Info from "SAL" entries in ".aff" file used in sl_sal.
// The info is split for quick processing by spell_soundfold().
// Note that "sm_oneof" and "sm_rules" point into sm_lead.
typedef struct salitem_S
{
    char_u	*sm_lead;	// leading letters
    int		sm_leadlen;	// length of "sm_lead"
    char_u	*sm_oneof;	// letters from () or NULL
    char_u	*sm_rules;	// rules like ^, $, priority
    char_u	*sm_to;		// replacement.
    int		*sm_lead_w;	// wide character copy of "sm_lead"
    int		*sm_oneof_w;	// wide character copy of "sm_oneof"
    int		*sm_to_w;	// wide character copy of "sm_to"
} salitem_T;

// Values for SP_*ERROR are negative, positive values are used by
// read_cnt_string().
#define	SP_TRUNCERROR	(-1)	// spell file truncated error
#define	SP_FORMERROR	(-2)	// format error in spell file
#define SP_OTHERERROR	(-3)	// other error while reading spell file

/*
 * Structure used in "b_langp", filled from 'spelllang'.
 */
typedef struct langp_S
{
    slang_T	*lp_slang;	// info for this language
    slang_T	*lp_sallang;	// language used for sound folding or NULL
    slang_T	*lp_replang;	// language used for REP items or NULL
    int		lp_region;	// bitmask for region or REGION_ALL
} langp_T;

#define LANGP_ENTRY(ga, i)	(((langp_T *)(ga).ga_data) + (i))

#define VIMSUGMAGIC "VIMsug"	// string at start of Vim .sug file
#define VIMSUGMAGICL 6
#define VIMSUGVERSION 1

/*
 * The tables used for recognizing word characters according to spelling.
 * These are only used for the first 256 characters of 'encoding'.
 */
typedef struct spelltab_S
{
    char_u  st_isw[256];	// flags: is word char
    char_u  st_isu[256];	// flags: is uppercase char
    char_u  st_fold[256];	// chars: folded case
    char_u  st_upper[256];	// chars: upper case
} spelltab_T;

/*
 * Use our own character-case definitions, because the current locale may
 * differ from what the .spl file uses.
 * These must not be called with negative number!
 */
#if defined(HAVE_WCHAR_H)
# include <wchar.h>	    // for towupper() and towlower()
#endif
// Multi-byte implementation.  For Unicode we can call utf_*(), but don't do
// that for ASCII, because we don't want to use 'casemap' here.  Otherwise use
// the "w" library function for characters above 255 if available.
#ifdef HAVE_TOWLOWER
# define SPELL_TOFOLD(c) (enc_utf8 && (c) >= 128 ? utf_fold(c) \
	    : (c) < 256 ? (int)spelltab.st_fold[c] : (int)towlower(c))
#else
# define SPELL_TOFOLD(c) (enc_utf8 && (c) >= 128 ? utf_fold(c) \
	    : (c) < 256 ? (int)spelltab.st_fold[c] : (c))
#endif

#ifdef HAVE_TOWUPPER
# define SPELL_TOUPPER(c) (enc_utf8 && (c) >= 128 ? utf_toupper(c) \
	    : (c) < 256 ? (int)spelltab.st_upper[c] : (int)towupper(c))
#else
# define SPELL_TOUPPER(c) (enc_utf8 && (c) >= 128 ? utf_toupper(c) \
	    : (c) < 256 ? (int)spelltab.st_upper[c] : (c))
#endif

#ifdef HAVE_ISWUPPER
# define SPELL_ISUPPER(c) (enc_utf8 && (c) >= 128 ? utf_isupper(c) \
	    : (c) < 256 ? spelltab.st_isu[c] : iswupper(c))
#else
# define SPELL_ISUPPER(c) (enc_utf8 && (c) >= 128 ? utf_isupper(c) \
	    : (c) < 256 ? spelltab.st_isu[c] : (FALSE))
#endif

#ifdef FEAT_SPELL
# ifdef IN_SPELL_C
#  define SPELL_EXTERN
#  define SPELL_INIT(x) x
# else
#  define SPELL_EXTERN extern
#  define SPELL_INIT(x)
# endif

// First language that is loaded, start of the linked list of loaded
// languages.
SPELL_EXTERN slang_T	*first_lang SPELL_INIT(= NULL);

// file used for "zG" and "zW"
SPELL_EXTERN char_u	*int_wordlist SPELL_INIT(= NULL);

SPELL_EXTERN spelltab_T   spelltab;
SPELL_EXTERN int	  did_set_spelltab;

// Values for "what" argument of spell_add_word()
#define SPELL_ADD_GOOD	0
#define SPELL_ADD_BAD	1
#define SPELL_ADD_RARE	2

typedef struct wordcount_S
{
    short_u	wc_count;	    // nr of times word was seen
    char_u	wc_word[1];	    // word, actually longer
} wordcount_T;

#define WC_KEY_OFF  offsetof(wordcount_T, wc_word)
#define HI2WC(hi)     ((wordcount_T *)((hi)->hi_key - WC_KEY_OFF))
#define MAXWORDCOUNT 0xffff

// Remember what "z?" replaced.
SPELL_EXTERN char_u	*repl_from SPELL_INIT(= NULL);
SPELL_EXTERN char_u	*repl_to SPELL_INIT(= NULL);
#endif
