/*
 * fontconfig/src/fcint.h
 *
 * Copyright © 2000 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of the author(s) not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  The authors make no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * THE AUTHOR(S) DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */

#ifndef _FCINT_H_
#define _FCINT_H_

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "fcstdint.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>
#include <errno.h>
#include <limits.h>
#include <float.h>
#include <math.h>
#ifdef HAVE_UNISTD_H
#include <unistd.h>
#endif
#include <stddef.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <time.h>
#include <fontconfig/fontconfig.h>
#include <fontconfig/fcprivate.h>
#include "fcdeprecate.h"
#include "fcmutex.h"
#include "fcatomic.h"

#ifndef FC_CONFIG_PATH
#define FC_CONFIG_PATH "fonts.conf"
#endif

#ifdef _WIN32
#define FC_LIKELY(expr) (expr)
#define FC_UNLIKELY(expr) (expr)
#else
#define FC_LIKELY(expr) (__builtin_expect (((expr) ? 1 : 0), 1))
#define FC_UNLIKELY(expr) (__builtin_expect (((expr) ? 1 : 0), 0))
#endif

#ifdef _WIN32
#  include "fcwindows.h"
typedef UINT (WINAPI *pfnGetSystemWindowsDirectory)(LPSTR, UINT);
typedef HRESULT (WINAPI *pfnSHGetFolderPathA)(HWND, int, HANDLE, DWORD, LPSTR);
extern pfnGetSystemWindowsDirectory pGetSystemWindowsDirectory;
extern pfnSHGetFolderPathA pSHGetFolderPathA;
#  define FC_SEARCH_PATH_SEPARATOR ';'
#  define FC_DIR_SEPARATOR         '\\'
#  define FC_DIR_SEPARATOR_S       "\\"
#else
#  define FC_SEARCH_PATH_SEPARATOR ':'
#  define FC_DIR_SEPARATOR         '/'
#  define FC_DIR_SEPARATOR_S       "/"
#endif

#ifdef PATH_MAX
#define FC_PATH_MAX	PATH_MAX
#else
#define FC_PATH_MAX	128
#endif

#if __GNUC__ >= 4
#define FC_UNUSED	__attribute__((unused))
#else
#define FC_UNUSED
#endif

#ifndef FC_UINT64_FORMAT
#define FC_UINT64_FORMAT	"llu"
#endif

#define FC_DBG_MATCH	1
#define FC_DBG_MATCHV	2
#define FC_DBG_EDIT	4
#define FC_DBG_FONTSET	8
#define FC_DBG_CACHE	16
#define FC_DBG_CACHEV	32
#define FC_DBG_PARSE	64
#define FC_DBG_SCAN	128
#define FC_DBG_SCANV	256
#define FC_DBG_CONFIG	1024
#define FC_DBG_LANGSET	2048
#define FC_DBG_MATCH2	4096

#define _FC_ASSERT_STATIC1(_line, _cond) typedef int _static_assert_on_line_##_line##_failed[(_cond)?1:-1] FC_UNUSED
#define _FC_ASSERT_STATIC0(_line, _cond) _FC_ASSERT_STATIC1 (_line, (_cond))
#define FC_ASSERT_STATIC(_cond) _FC_ASSERT_STATIC0 (__LINE__, (_cond))

#define FC_MIN(a,b) ((a) < (b) ? (a) : (b))
#define FC_MAX(a,b) ((a) > (b) ? (a) : (b))

/* slim_internal.h */
#if (__GNUC__ > 3 || (__GNUC__ == 3 && __GNUC_MINOR__ >= 3)) && defined(__ELF__) && !defined(__sun)
#define FcPrivate		__attribute__((__visibility__("hidden")))
#define HAVE_GNUC_ATTRIBUTE 1
#include "fcalias.h"
#elif defined(__SUNPRO_C) && (__SUNPRO_C >= 0x550)
#define FcPrivate		__hidden
#else /* not gcc >= 3.3 and not Sun Studio >= 8 */
#define FcPrivate
#endif

/* NLS */
#ifdef ENABLE_NLS
#include <libintl.h>
#define _(x)		(dgettext(GETTEXT_PACKAGE, x))
#else
#define dgettext(d, s)	(s)
#define _(x)		(x)
#endif

#define N_(x)	x

FC_ASSERT_STATIC (sizeof (FcRef) == sizeof (int));

#define FcStrdup(s) ((FcChar8 *) strdup ((const char *) (s)))
#define FcFree(s) (free ((FcChar8 *) (s)))

/*
 * Serialized data structures use only offsets instead of pointers
 * A low bit of 1 indicates an offset.
 */

/* Is the provided pointer actually an offset? */
#define FcIsEncodedOffset(p)	((((intptr_t) (p)) & 1) != 0)

/* Encode offset in a pointer of type t */
#define FcOffsetEncode(o,t)	((t *) (intptr_t) ((o) | 1))

/* Decode a pointer into an offset */
#define FcOffsetDecode(p)	(((intptr_t) (p)) & ~1)

/* Compute pointer offset */
#define FcPtrToOffset(b,p)	((ptrdiff_t) ((intptr_t) (p) - (intptr_t) (b)))

/* Given base address, offset and type, return a pointer */
#define FcOffsetToPtr(b,o,t)	((t *) ((intptr_t) (b) + (ptrdiff_t) (o)))

/* Given base address, encoded offset and type, return a pointer */
#define FcEncodedOffsetToPtr(b,p,t) FcOffsetToPtr(b,FcOffsetDecode(p),t)

/* Given base address, pointer and type, return an encoded offset */
#define FcPtrToEncodedOffset(b,p,t) FcOffsetEncode(FcPtrToOffset(b,p),t)

/* Given a structure, offset member and type, return pointer */
#define FcOffsetMember(s,m,t)	    FcOffsetToPtr(s,(s)->m,t)

/* Given a structure, encoded offset member and type, return pointer to member */
#define FcEncodedOffsetMember(s,m,t) FcOffsetToPtr(s,FcOffsetDecode((s)->m), t)

/* Given a structure, member and type, convert the member to a pointer */
#define FcPointerMember(s,m,t)	(FcIsEncodedOffset((s)->m) ? \
				 FcEncodedOffsetMember (s,m,t) : \
				 (s)->m)

/*
 * Serialized values may hold strings, charsets and langsets as pointers,
 * unfortunately FcValue is an exposed type so we can't just always use
 * offsets
 */
#define FcValueString(v)	FcPointerMember(v,u.s,FcChar8)
#define FcValueCharSet(v)	FcPointerMember(v,u.c,const FcCharSet)
#define FcValueLangSet(v)	FcPointerMember(v,u.l,const FcLangSet)
#define FcValueRange(v)		FcPointerMember(v,u.r,const FcRange)

typedef struct _FcValueList *FcValueListPtr;

typedef struct _FcValueList {
    struct _FcValueList	*next;
    FcValue		value;
    FcValueBinding	binding;
} FcValueList;

#define FcValueListNext(vl)	FcPointerMember(vl,next,FcValueList)

typedef int FcObject;

/* The 1024 is to leave some room for future added internal objects, such
 * that caches from newer fontconfig can still be used with older fontconfig
 * without getting confused. */
#define FC_EXT_OBJ_INDEX	1024
#define FC_OBJ_ID(_n_)	((_n_) & (~FC_EXT_OBJ_INDEX))

typedef struct _FcPatternElt *FcPatternEltPtr;

/*
 * Pattern elts are stuck in a structure connected to the pattern,
 * so they get moved around when the pattern is resized. Hence, the
 * values field must be a pointer/offset instead of just an offset
 */
typedef struct _FcPatternElt {
    FcObject		object;
    FcValueList		*values;
} FcPatternElt;

#define FcPatternEltValues(pe)	FcPointerMember(pe,values,FcValueList)

struct _FcPattern {
    int		    num;
    int		    size;
    intptr_t	    elts_offset;
    FcRef	    ref;
};

#define FcPatternElts(p)	FcOffsetMember(p,elts_offset,FcPatternElt)

#define FcFontSetFonts(fs)	FcPointerMember(fs,fonts,FcPattern *)

#define FcFontSetFont(fs,i)	(FcIsEncodedOffset((fs)->fonts) ? \
				 FcEncodedOffsetToPtr(fs, \
						      FcFontSetFonts(fs)[i], \
						      FcPattern) : \
				 fs->fonts[i])

typedef enum _FcOp {
    FcOpInteger, FcOpDouble, FcOpString, FcOpMatrix, FcOpRange, FcOpBool, FcOpCharSet, FcOpLangSet,
    FcOpNil,
    FcOpField, FcOpConst,
    FcOpAssign, FcOpAssignReplace,
    FcOpPrependFirst, FcOpPrepend, FcOpAppend, FcOpAppendLast,
    FcOpDelete, FcOpDeleteAll,
    FcOpQuest,
    FcOpOr, FcOpAnd, FcOpEqual, FcOpNotEqual,
    FcOpContains, FcOpListing, FcOpNotContains,
    FcOpLess, FcOpLessEqual, FcOpMore, FcOpMoreEqual,
    FcOpPlus, FcOpMinus, FcOpTimes, FcOpDivide,
    FcOpNot, FcOpComma, FcOpFloor, FcOpCeil, FcOpRound, FcOpTrunc,
    FcOpInvalid
} FcOp;

typedef enum _FcOpFlags {
	FcOpFlagIgnoreBlanks = 1U << 0
} FcOpFlags;

#define FC_OP_GET_OP(_x_)	((_x_) & 0xffff)
#define FC_OP_GET_FLAGS(_x_)	(((_x_) & 0xffff0000) >> 16)
#define FC_OP(_x_,_f_)		(FC_OP_GET_OP (_x_) | ((_f_) << 16))

typedef struct _FcExprMatrix {
  struct _FcExpr *xx, *xy, *yx, *yy;
} FcExprMatrix;

typedef struct _FcExprName {
  FcObject	object;
  FcMatchKind	kind;
} FcExprName;

struct _FcRange {
    double begin;
    double end;
};


typedef struct _FcExpr {
    FcOp   op;
    union {
	int		ival;
	double		dval;
	const FcChar8	*sval;
	FcExprMatrix	*mexpr;
	FcBool		bval;
	FcCharSet	*cval;
	FcLangSet	*lval;
	FcRange		*rval;

	FcExprName	name;
	const FcChar8	*constant;
	struct {
	    struct _FcExpr *left, *right;
	} tree;
    } u;
} FcExpr;

typedef struct _FcExprPage FcExprPage;

struct _FcExprPage {
  FcExprPage *next_page;
  FcExpr *next;
  FcExpr exprs[(1024 - 2/* two pointers */ - 2/* malloc overhead */) * sizeof (void *) / sizeof (FcExpr)];
  FcExpr end[FLEXIBLE_ARRAY_MEMBER];
};

typedef enum _FcQual {
    FcQualAny, FcQualAll, FcQualFirst, FcQualNotFirst
} FcQual;

#define FcMatchDefault	((FcMatchKind) -1)

typedef struct _FcTest {
    FcMatchKind		kind;
    FcQual		qual;
    FcObject		object;
    FcOp		op;
    FcExpr		*expr;
} FcTest;

typedef struct _FcEdit {
    FcObject	    object;
    FcOp	    op;
    FcExpr	    *expr;
    FcValueBinding  binding;
} FcEdit;

typedef void (* FcDestroyFunc) (void *data);

typedef struct _FcPtrList	FcPtrList;
/* need to sync with FcConfigFileInfoIter at fontconfig.h */
typedef struct _FcPtrListIter {
    void *dummy1;
    void *dummy2;
    void *dummy3;
} FcPtrListIter;

typedef enum _FcRuleType {
    FcRuleUnknown, FcRuleTest, FcRuleEdit
} FcRuleType;

typedef struct _FcRule {
    struct _FcRule *next;
    FcRuleType      type;
    union {
	FcTest *test;
	FcEdit *edit;
    } u;
} FcRule;

typedef struct _FcRuleSet {
    FcRef	ref;
    FcChar8	*name;
    FcChar8	*description;
    FcChar8	*domain;
    FcBool	enabled;
    FcPtrList	*subst[FcMatchKindEnd];
} FcRuleSet;

typedef struct _FcCharLeaf {
    FcChar32	map[256/32];
} FcCharLeaf;

struct _FcCharSet {
    FcRef	    ref;	/* reference count */
    int		    num;	/* size of leaves and numbers arrays */
    intptr_t	    leaves_offset;
    intptr_t	    numbers_offset;
};

#define FcCharSetLeaves(c)	FcOffsetMember(c,leaves_offset,intptr_t)
#define FcCharSetLeaf(c,i)	(FcOffsetToPtr(FcCharSetLeaves(c), \
					       FcCharSetLeaves(c)[i], \
					       FcCharLeaf))
#define FcCharSetNumbers(c)	FcOffsetMember(c,numbers_offset,FcChar16)

#define FCSS_DEFAULT            0 /* default behavior */
#define FCSS_ALLOW_DUPLICATES   1 /* allows for duplicate strings in the set */
#define FCSS_GROW_BY_64         2 /* grows buffer by 64 elements instead of 1 */

#define FcStrSetHasControlBit(s,c)  (s->control & c)
#define FcStrSetHasControlBits(s,c) ( (c) == (s->control & (c)) )

struct _FcStrSet {
    FcRef	    ref;	/* reference count */
    int		    num;
    int		    size;
    FcChar8	    **strs;
    unsigned int    control;    /* control bits for set behavior */
};

struct _FcStrList {
    FcStrSet	    *set;
    int		    n;
};

typedef struct _FcStrBuf {
    FcChar8 *buf;
    FcBool  allocated;
    FcBool  failed;
    int	    len;
    int	    size;
    FcChar8 buf_static[16 * sizeof (void *)];
} FcStrBuf;

typedef struct _FcHashTable	FcHashTable;

typedef FcChar32 (* FcHashFunc)	   (const FcChar8 *data);
typedef int	 (* FcCompareFunc) (const FcChar8 *v1, const FcChar8 *v2);
typedef FcBool	 (* FcCopyFunc)	   (const void *src, void **dest);


struct _FcCache {
    unsigned int magic;              /* FC_CACHE_MAGIC_MMAP or FC_CACHE_ALLOC */
    int		version;	    /* FC_CACHE_VERSION_NUMBER */
    intptr_t	size;		    /* size of file */
    intptr_t	dir;		    /* offset to dir name */
    intptr_t	dirs;		    /* offset to subdirs */
    int		dirs_count;	    /* number of subdir strings */
    intptr_t	set;		    /* offset to font set */
    int		checksum;	    /* checksum of directory state */
    int64_t	checksum_nano;	    /* checksum of directory state */
};

#undef FcCacheDir
#undef FcCacheSubdir
#define FcCacheDir(c)	FcOffsetMember(c,dir,FcChar8)
#define FcCacheDirs(c)	FcOffsetMember(c,dirs,intptr_t)
#define FcCacheSet(c)	FcOffsetMember(c,set,FcFontSet)
#define FcCacheSubdir(c,i)  FcOffsetToPtr (FcCacheDirs(c),\
					   FcCacheDirs(c)[i], \
					   FcChar8)

/*
 * Used while constructing a directory cache object
 */

typedef union _FcAlign {
    double	d;
    int		i;
    intptr_t	ip;
    FcBool	b;
    void	*p;
} FcAlign;

typedef struct _FcSerializeBucket {
    const void	*object; /* key */
    uintptr_t	hash;    /* hash of key */
    intptr_t	offset;  /* value */
} FcSerializeBucket;

typedef struct _FcCharSetFreezer FcCharSetFreezer;

typedef struct _FcSerialize {
    intptr_t		size;
    FcCharSetFreezer	*cs_freezer;
    void		*linear;
    FcSerializeBucket	*buckets;
    size_t		buckets_count;
    size_t		buckets_used;
    size_t		buckets_used_max;
} FcSerialize;

/*
 * To map adobe glyph names to unicode values, a precomputed hash
 * table is used
 */

typedef struct _FcGlyphName {
    FcChar32	ucs;		/* unicode value */
    FcChar8	name[1];	/* name extends beyond struct */
} FcGlyphName;

/*
 * To perform case-insensitive string comparisons, a table
 * is used which holds three different kinds of folding data.
 *
 * The first is a range of upper case values mapping to a range
 * of their lower case equivalents.  Within each range, the offset
 * between upper and lower case is constant.
 *
 * The second is a range of upper case values which are interleaved
 * with their lower case equivalents.
 *
 * The third is a set of raw unicode values mapping to a list
 * of unicode values for comparison purposes.  This allows conversion
 * of ß to "ss" so that SS, ss and ß all match.  A separate array
 * holds the list of unicode values for each entry.
 *
 * These are packed into a single table.  Using a binary search,
 * the appropriate entry can be located.
 */

#define FC_CASE_FOLD_RANGE	    0
#define FC_CASE_FOLD_EVEN_ODD	    1
#define FC_CASE_FOLD_FULL	    2

typedef struct _FcCaseFold {
    FcChar32	upper;
    FcChar16	method : 2;
    FcChar16	count : 14;
    short    	offset;	    /* lower - upper for RANGE, table id for FULL */
} FcCaseFold;

#define FC_MAX_FILE_LEN	    4096

#define FC_CACHE_MAGIC_MMAP	    0xFC02FC04
#define FC_CACHE_MAGIC_ALLOC	    0xFC02FC05

struct _FcAtomic {
    FcChar8	*file;		/* original file name */
    FcChar8	*new;		/* temp file name -- write data here */
    FcChar8	*lck;		/* lockfile name (used for locking) */
    FcChar8	*tmp;		/* tmpfile name (used for locking) */
};

struct _FcConfig {
    /*
     * File names loaded from the configuration -- saved here as the
     * cache file must be consulted before the directories are scanned,
     * and those directives may occur in any order
     */
    FcStrSet	*configDirs;	    /* directories to scan for fonts */
    /*
     * List of directories containing fonts,
     * built by recursively scanning the set
     * of configured directories
     */
    FcStrSet	*fontDirs;
    /*
     * List of directories containing cache files.
     */
    FcStrSet	*cacheDirs;
    /*
     * Names of all of the configuration files used
     * to create this configuration
     */
    FcStrSet	*configFiles;	    /* config files loaded */
    /*
     * Substitution instructions for patterns and fonts;
     * maxObjects is used to allocate appropriate intermediate storage
     * while performing a whole set of substitutions
     *
     * 0.. substitutions for patterns
     * 1.. substitutions for fonts
     * 2.. substitutions for scanned fonts
     */
    FcPtrList	*subst[FcMatchKindEnd];
    int		maxObjects;	    /* maximum number of tests in all substs */
    /*
     * List of patterns used to control font file selection
     */
    FcStrSet	*acceptGlobs;
    FcStrSet	*rejectGlobs;
    FcFontSet	*acceptPatterns;
    FcFontSet	*rejectPatterns;
    /*
     * The set of fonts loaded from the listed directories; the
     * order within the set does not determine the font selection,
     * except in the case of identical matches in which case earlier fonts
     * match preferrentially
     */
    FcFontSet	*fonts[FcSetApplication + 1];
    /*
     * Fontconfig can periodically rescan the system configuration
     * and font directories.  This rescanning occurs when font
     * listing requests are made, but no more often than rescanInterval
     * seconds apart.
     */
    time_t	rescanTime;	    /* last time information was scanned */
    int		rescanInterval;	    /* interval between scans */

    FcRef	ref;                /* reference count */

    FcExprPage  *expr_pool;	    /* pool of FcExpr's */

    FcChar8     *sysRoot;	    /* override the system root directory */
    FcStrSet	*availConfigFiles;  /* config files available */
    FcPtrList	*rulesetList;	    /* List of rulesets being installed */
};

typedef struct _FcFileTime {
    time_t  time;
    FcBool  set;
} FcFileTime;

typedef struct _FcCharMap FcCharMap;

typedef struct _FcStatFS    FcStatFS;

struct _FcStatFS {
    FcBool is_remote_fs;
    FcBool is_mtime_broken;
};

typedef struct _FcValuePromotionBuffer FcValuePromotionBuffer;

struct _FcValuePromotionBuffer {
  union {
    double d;
    int i;
    long l;
    char c[256]; /* Enlarge as needed */
  } u;
};

/* fccache.c */

FcPrivate FcCache *
FcDirCacheScan (const FcChar8 *dir, FcConfig *config);

FcPrivate FcCache *
FcDirCacheBuild (FcFontSet *set, const FcChar8 *dir, struct stat *dir_stat, FcStrSet *dirs);

FcPrivate FcCache *
FcDirCacheRebuild (FcCache *cache, struct stat *dir_stat, FcStrSet *dirs);

FcPrivate FcBool
FcDirCacheWrite (FcCache *cache, FcConfig *config);

FcPrivate FcBool
FcDirCacheCreateTagFile (const FcChar8 *cache_dir);

FcPrivate void
FcCacheObjectReference (void *object);

FcPrivate void
FcCacheObjectDereference (void *object);

FcPrivate void *
FcCacheAllocate (FcCache *cache, size_t len);

FcPrivate void
FcCacheFini (void);


FcPrivate void
FcDirCacheReference (FcCache *cache, int nref);

FcPrivate int
FcDirCacheLock (const FcChar8 *dir,
		FcConfig      *config);

FcPrivate void
FcDirCacheUnlock (int fd);

/* fccfg.c */

FcPrivate FcBool
FcConfigInit (void);

FcPrivate void
FcConfigFini (void);

FcPrivate FcChar8 *
FcConfigXdgCacheHome (void);

FcPrivate FcChar8 *
FcConfigXdgConfigHome (void);

FcPrivate FcChar8 *
FcConfigXdgDataHome (void);

FcPrivate FcStrSet *
FcConfigXdgDataDirs (void);

FcPrivate FcExpr *
FcConfigAllocExpr (FcConfig *config);

FcPrivate FcBool
FcConfigAddConfigDir (FcConfig	    *config,
		      const FcChar8 *d);

FcPrivate FcBool
FcConfigAddFontDir (FcConfig	    *config,
		    const FcChar8   *d,
		    const FcChar8   *m,
		    const FcChar8   *salt);

FcPrivate FcBool
FcConfigResetFontDirs (FcConfig *config);

FcPrivate FcChar8 *
FcConfigMapFontPath(FcConfig		*config,
		    const FcChar8	*path);

FcPrivate const FcChar8 *
FcConfigMapSalt (FcConfig      *config,
		 const FcChar8 *path);

FcPrivate FcBool
FcConfigAddCacheDir (FcConfig	    *config,
		     const FcChar8  *d);

FcPrivate FcBool
FcConfigAddConfigFile (FcConfig		*config,
		       const FcChar8	*f);

FcPrivate FcBool
FcConfigAddBlank (FcConfig	*config,
		  FcChar32    	blank);

FcBool
FcConfigAddRule (FcConfig	*config,
		 FcRule		*rule,
		 FcMatchKind	kind);

FcPrivate void
FcConfigSetFonts (FcConfig	*config,
		  FcFontSet	*fonts,
		  FcSetName	set);

FcPrivate FcBool
FcConfigCompareValue (const FcValue *m,
		      unsigned int   op_,
		      const FcValue *v);

FcPrivate FcBool
FcConfigGlobAdd (FcConfig	*config,
		 const FcChar8	*glob,
		 FcBool		accept);

FcPrivate FcBool
FcConfigAcceptFilename (FcConfig	*config,
			const FcChar8	*filename);

FcPrivate FcBool
FcConfigPatternsAdd (FcConfig	*config,
		     FcPattern	*pattern,
		     FcBool	accept);

FcPrivate FcBool
FcConfigAcceptFont (FcConfig	    *config,
		    const FcPattern *font);

FcPrivate FcFileTime
FcConfigModifiedTime (FcConfig *config);

FcPrivate FcBool
FcConfigAddCache (FcConfig *config, FcCache *cache,
		  FcSetName set, FcStrSet *dirSet, FcChar8 *forDir);

FcPrivate FcRuleSet *
FcRuleSetCreate (const FcChar8 *name);

FcPrivate void
FcRuleSetDestroy (FcRuleSet *rs);

FcPrivate void
FcRuleSetReference (FcRuleSet *rs);

FcPrivate void
FcRuleSetEnable (FcRuleSet	*rs,
		 FcBool		flag);

FcPrivate void
FcRuleSetAddDescription (FcRuleSet	*rs,
			 const FcChar8	*domain,
			 const FcChar8	*description);

FcPrivate int
FcRuleSetAdd (FcRuleSet		*rs,
	      FcRule		*rule,
	      FcMatchKind	kind);

/* fcserialize.c */
FcPrivate intptr_t
FcAlignSize (intptr_t size);

FcPrivate FcSerialize *
FcSerializeCreate (void);

FcPrivate void
FcSerializeDestroy (FcSerialize *serialize);

FcPrivate FcBool
FcSerializeAlloc (FcSerialize *serialize, const void *object, int size);

FcPrivate intptr_t
FcSerializeReserve (FcSerialize *serialize, int size);

FcPrivate intptr_t
FcSerializeOffset (FcSerialize *serialize, const void *object);

FcPrivate void *
FcSerializePtr (FcSerialize *serialize, const void *object);

FcPrivate FcBool
FcLangSetSerializeAlloc (FcSerialize *serialize, const FcLangSet *l);

FcPrivate FcLangSet *
FcLangSetSerialize(FcSerialize *serialize, const FcLangSet *l);

/* fccharset.c */
FcPrivate FcCharSet *
FcCharSetPromote (FcValuePromotionBuffer *vbuf);

FcPrivate void
FcLangCharSetPopulate (void);

FcPrivate FcCharSetFreezer *
FcCharSetFreezerCreate (void);

FcPrivate const FcCharSet *
FcCharSetFreeze (FcCharSetFreezer *freezer, const FcCharSet *fcs);

FcPrivate void
FcCharSetFreezerDestroy (FcCharSetFreezer *freezer);

FcPrivate FcBool
FcNameUnparseCharSet (FcStrBuf *buf, const FcCharSet *c);

FcPrivate FcCharSet *
FcNameParseCharSet (FcChar8 *string);

FcPrivate FcBool
FcNameUnparseValue (FcStrBuf    *buf,
                    FcValue     *v0,
		    FcChar8     *escape);

FcPrivate FcBool
FcNameUnparseValueList (FcStrBuf	*buf,
			FcValueListPtr	v,
			FcChar8		*escape);

FcPrivate FcCharLeaf *
FcCharSetFindLeafCreate (FcCharSet *fcs, FcChar32 ucs4);

FcPrivate FcBool
FcCharSetSerializeAlloc(FcSerialize *serialize, const FcCharSet *cs);

FcPrivate FcCharSet *
FcCharSetSerialize(FcSerialize *serialize, const FcCharSet *cs);

FcPrivate FcChar16 *
FcCharSetGetNumbers(const FcCharSet *c);

/* fccompat.c */
FcPrivate int
FcOpen(const char *pathname, int flags, ...);

FcPrivate int
FcMakeTempfile (char *template);

FcPrivate int32_t
FcRandom (void);

FcPrivate FcBool
FcMakeDirectory (const FcChar8 *dir);

FcPrivate ssize_t
FcReadLink (const FcChar8 *pathname,
	    FcChar8       *buf,
	    size_t         bufsiz);

/* fcdbg.c */

FcPrivate void
FcValuePrintFile (FILE *f, const FcValue v);

FcPrivate void
FcValuePrintWithPosition (const FcValue v, FcBool show_pos_mark);

FcPrivate void
FcValueListPrintWithPosition (FcValueListPtr l, const FcValueListPtr pos);

FcPrivate void
FcValueListPrint (FcValueListPtr l);

FcPrivate void
FcLangSetPrint (const FcLangSet *ls);

FcPrivate void
FcOpPrint (FcOp op);

FcPrivate void
FcTestPrint (const FcTest *test);

FcPrivate void
FcExprPrint (const FcExpr *expr);

FcPrivate void
FcEditPrint (const FcEdit *edit);

FcPrivate void
FcRulePrint (const FcRule *rule);

FcPrivate void
FcCharSetPrint (const FcCharSet *c);

FcPrivate void
FcPatternPrint2 (FcPattern *p1, FcPattern *p2, const FcObjectSet *os);

extern FcPrivate int FcDebugVal;

#define FcDebug() (FcDebugVal)

FcPrivate void
FcInitDebug (void);

/* fcdefault.c */
FcPrivate FcChar8 *
FcGetDefaultLang (void);

FcPrivate FcChar8 *
FcGetPrgname (void);

FcPrivate FcChar8 *
FcGetDesktopName (void);

FcPrivate void
FcDefaultFini (void);

/* fcdir.c */

FcPrivate FcBool
FcFileIsLink (const FcChar8 *file);

FcPrivate FcBool
FcFileIsFile (const FcChar8 *file);

FcPrivate FcBool
FcFileScanConfig (FcFontSet	*set,
		  FcStrSet	*dirs,
		  const FcChar8 *file,
		  FcConfig	*config);

FcPrivate FcBool
FcDirScanConfig (FcFontSet	*set,
		 FcStrSet	*dirs,
		 const FcChar8	*dir,
		 FcBool		force,
		 FcConfig	*config);

/* fcfont.c */
FcPrivate int
FcFontDebug (void);

/* fcfs.c */

FcPrivate FcBool
FcFontSetSerializeAlloc (FcSerialize *serialize, const FcFontSet *s);

FcPrivate FcFontSet *
FcFontSetSerialize (FcSerialize *serialize, const FcFontSet * s);

FcPrivate FcFontSet *
FcFontSetDeserialize (const FcFontSet *set);

/* fcplist.c */
FcPrivate FcPtrList *
FcPtrListCreate (FcDestroyFunc func);

FcPrivate void
FcPtrListDestroy (FcPtrList *list);

FcPrivate void
FcPtrListIterInit (const FcPtrList	*list,
		   FcPtrListIter	*iter);

FcPrivate void
FcPtrListIterInitAtLast (FcPtrList	*list,
			 FcPtrListIter	*iter);

FcPrivate FcBool
FcPtrListIterNext (const FcPtrList	*list,
		   FcPtrListIter	*iter);

FcPrivate FcBool
FcPtrListIterIsValid (const FcPtrList		*list,
		      const FcPtrListIter	*iter);

FcPrivate void *
FcPtrListIterGetValue (const FcPtrList		*list,
		       const FcPtrListIter	*iter);

FcPrivate FcBool
FcPtrListIterAdd (FcPtrList	*list,
		  FcPtrListIter	*iter,
		void		*data);

FcPrivate FcBool
FcPtrListIterRemove (FcPtrList		*list,
		     FcPtrListIter	*iter);

/* fcinit.c */
FcPrivate FcConfig *
FcInitLoadOwnConfig (FcConfig *config);

FcPrivate FcConfig *
FcInitLoadOwnConfigAndFonts (FcConfig *config);

/* fcxml.c */
FcPrivate void
FcConfigPathFini (void);

FcPrivate void
FcTestDestroy (FcTest *test);

FcPrivate void
FcEditDestroy (FcEdit *e);

void
FcRuleDestroy (FcRule *rule);

/* fclang.c */
FcPrivate FcLangSet *
FcFreeTypeLangSet (const FcCharSet  *charset,
		   const FcChar8    *exclusiveLang);

FcPrivate FcLangResult
FcLangCompare (const FcChar8 *s1, const FcChar8 *s2);

FcPrivate FcLangSet *
FcLangSetPromote (const FcChar8 *lang, FcValuePromotionBuffer *buf);

FcPrivate FcLangSet *
FcNameParseLangSet (const FcChar8 *string);

FcPrivate FcBool
FcNameUnparseLangSet (FcStrBuf *buf, const FcLangSet *ls);

FcPrivate FcChar8 *
FcNameUnparseEscaped (FcPattern *pat, FcBool escape);

FcPrivate FcBool
FcConfigParseOnly (FcConfig		*config,
		   const FcChar8	*name,
		   FcBool		complain);

FcPrivate FcChar8 *
FcConfigRealFilename (FcConfig		*config,
		      const FcChar8	*url);

/* fclist.c */

FcPrivate FcBool
FcListPatternMatchAny (const FcPattern *p,
		       const FcPattern *font);

/* fcmatch.c */

/* fcname.c */

enum {
  FC_INVALID_OBJECT = 0,
#define FC_OBJECT(NAME, Type, Cmp) FC_##NAME##_OBJECT,
#include "fcobjs.h"
#undef FC_OBJECT
  FC_ONE_AFTER_MAX_BASE_OBJECT
#define FC_MAX_BASE_OBJECT (FC_ONE_AFTER_MAX_BASE_OBJECT - 1)
};

FcPrivate FcBool
FcNameConstantWithObjectCheck (const FcChar8 *string, const char *object, int *result);

FcPrivate FcBool
FcNameBool (const FcChar8 *v, FcBool *result);

FcPrivate FcBool
FcObjectValidType (FcObject object, FcType type);

FcPrivate FcObject
FcObjectFromName (const char * name);

FcPrivate const char *
FcObjectName (FcObject object);

FcPrivate FcObjectSet *
FcObjectGetSet (void);

#define FcObjectCompare(a, b)	((int) a - (int) b)

/* fcpat.c */

FcPrivate FcValue
FcValueCanonicalize (const FcValue *v);

FcPrivate FcValueListPtr
FcValueListCreate (void);

FcPrivate void
FcValueListDestroy (FcValueListPtr l);

FcPrivate FcValueListPtr
FcValueListPrepend (FcValueListPtr vallist,
		    FcValue        value,
		    FcValueBinding binding);

FcPrivate FcValueListPtr
FcValueListAppend (FcValueListPtr vallist,
		   FcValue        value,
		   FcValueBinding binding);

FcPrivate FcValueListPtr
FcValueListDuplicate(FcValueListPtr orig);

FcPrivate FcPatternElt *
FcPatternObjectFindElt (const FcPattern *p, FcObject object);

FcPrivate FcPatternElt *
FcPatternObjectInsertElt (FcPattern *p, FcObject object);

FcPrivate FcBool
FcPatternObjectListAdd (FcPattern	*p,
			FcObject	object,
			FcValueListPtr	list,
			FcBool		append);

FcPrivate FcBool
FcPatternObjectAddWithBinding  (FcPattern	*p,
				FcObject	object,
				FcValue		value,
				FcValueBinding  binding,
				FcBool		append);

FcPrivate FcBool
FcPatternObjectAdd (FcPattern *p, FcObject object, FcValue value, FcBool append);

FcPrivate FcBool
FcPatternObjectAddWeak (FcPattern *p, FcObject object, FcValue value, FcBool append);

FcPrivate FcResult
FcPatternObjectGetWithBinding (const FcPattern *p, FcObject object, int id, FcValue *v, FcValueBinding *b);

FcPrivate FcResult
FcPatternObjectGet (const FcPattern *p, FcObject object, int id, FcValue *v);

FcPrivate FcBool
FcPatternObjectDel (FcPattern *p, FcObject object);

FcPrivate FcBool
FcPatternObjectRemove (FcPattern *p, FcObject object, int id);

FcPrivate FcBool
FcPatternObjectAddInteger (FcPattern *p, FcObject object, int i);

FcPrivate FcBool
FcPatternObjectAddDouble (FcPattern *p, FcObject object, double d);

FcPrivate FcBool
FcPatternObjectAddString (FcPattern *p, FcObject object, const FcChar8 *s);

FcPrivate FcBool
FcPatternObjectAddMatrix (FcPattern *p, FcObject object, const FcMatrix *s);

FcPrivate FcBool
FcPatternObjectAddCharSet (FcPattern *p, FcObject object, const FcCharSet *c);

FcPrivate FcBool
FcPatternObjectAddBool (FcPattern *p, FcObject object, FcBool b);

FcPrivate FcBool
FcPatternObjectAddLangSet (FcPattern *p, FcObject object, const FcLangSet *ls);

FcPrivate FcBool
FcPatternObjectAddRange (FcPattern *p, FcObject object, const FcRange *r);

FcPrivate FcResult
FcPatternObjectGetInteger (const FcPattern *p, FcObject object, int n, int *i);

FcPrivate FcResult
FcPatternObjectGetDouble (const FcPattern *p, FcObject object, int n, double *d);

FcPrivate FcResult
FcPatternObjectGetString (const FcPattern *p, FcObject object, int n, FcChar8 ** s);

FcPrivate FcResult
FcPatternObjectGetMatrix (const FcPattern *p, FcObject object, int n, FcMatrix **s);

FcPrivate FcResult
FcPatternObjectGetCharSet (const FcPattern *p, FcObject object, int n, FcCharSet **c);

FcPrivate FcResult
FcPatternObjectGetBool (const FcPattern *p, FcObject object, int n, FcBool *b);

FcPrivate FcResult
FcPatternObjectGetLangSet (const FcPattern *p, FcObject object, int n, FcLangSet **ls);

FcPrivate FcResult
FcPatternObjectGetRange (const FcPattern *p, FcObject object, int id, FcRange **r);

FcPrivate FcBool
FcPatternAppend (FcPattern *p, FcPattern *s);

FcPrivate int
FcPatternPosition (const FcPattern *p, const char *object);

FcPrivate FcBool
FcPatternFindObjectIter (const FcPattern *pat, FcPatternIter *iter, FcObject object);

FcPrivate FcObject
FcPatternIterGetObjectId (const FcPattern *pat, FcPatternIter *iter);

FcPrivate FcValueListPtr
FcPatternIterGetValues (const FcPattern *pat, FcPatternIter *iter);

FcPrivate FcPattern *
FcPatternCacheRewriteFile (const FcPattern *pat, FcCache *cache, const FcChar8 *relocated_font_file);

FcPrivate FcChar32
FcStringHash (const FcChar8 *s);

FcPrivate FcBool
FcPatternSerializeAlloc (FcSerialize *serialize, const FcPattern *pat);

FcPrivate FcPattern *
FcPatternSerialize (FcSerialize *serialize, const FcPattern *pat);

FcPrivate FcBool
FcValueListSerializeAlloc (FcSerialize *serialize, const FcValueList *pat);

FcPrivate FcValueList *
FcValueListSerialize (FcSerialize *serialize, const FcValueList *pat);

/* fcrender.c */

/* fcmatrix.c */

extern FcPrivate const FcMatrix    FcIdentityMatrix;

FcPrivate void
FcMatrixFree (FcMatrix *mat);

/* fcrange.c */

FcPrivate FcRange *
FcRangePromote (double v, FcValuePromotionBuffer *vbuf);

FcPrivate FcBool
FcRangeIsInRange (const FcRange *a, const FcRange *b);

FcPrivate FcBool
FcRangeCompare (FcOp op, const FcRange *a, const FcRange *b);

FcPrivate FcChar32
FcRangeHash (const FcRange *r);

FcPrivate FcBool
FcRangeSerializeAlloc (FcSerialize *serialize, const FcRange *r);

FcPrivate FcRange *
FcRangeSerialize (FcSerialize *serialize, const FcRange *r);

/* fcstat.c */

FcPrivate int
FcStat (const FcChar8 *file, struct stat *statb);

FcPrivate int
FcStatChecksum (const FcChar8 *file, struct stat *statb);

FcPrivate FcBool
FcIsFsMmapSafe (int fd);

FcPrivate FcBool
FcIsFsMtimeBroken (const FcChar8 *dir);

/* fcstr.c */
FcPrivate FcStrSet *
FcStrSetCreateEx (unsigned int control);

FcPrivate FcBool
FcStrSetInsert (FcStrSet *set, const FcChar8 *s, int pos);

FcPrivate FcBool
FcStrSetAddLangs (FcStrSet *strs, const char *languages);

FcPrivate void
FcStrSetSort (FcStrSet * set);

FcPrivate FcBool
FcStrSetMemberAB (FcStrSet *set, const FcChar8 *a, FcChar8 *b, FcChar8 **ret);

FcPrivate FcBool
FcStrSetAddTriple (FcStrSet *set, const FcChar8 *a, const FcChar8 *b, const FcChar8 *c);

FcPrivate const FcChar8 *
FcStrTripleSecond (FcChar8 *s);

FcPrivate const FcChar8 *
FcStrTripleThird (FcChar8 *str);

FcPrivate FcBool
FcStrSetAddFilenamePairWithSalt (FcStrSet *strs, const FcChar8 *d, const FcChar8 *m, const FcChar8 *salt);

FcPrivate FcBool
FcStrSetDeleteAll (FcStrSet *set);

FcPrivate void
FcStrBufInit (FcStrBuf *buf, FcChar8 *init, int size);

FcPrivate void
FcStrBufDestroy (FcStrBuf *buf);

FcPrivate FcChar8 *
FcStrBufDone (FcStrBuf *buf);

FcPrivate FcChar8 *
FcStrBufDoneStatic (FcStrBuf *buf);

FcPrivate FcBool
FcStrBufChar (FcStrBuf *buf, FcChar8 c);

FcPrivate FcBool
FcStrBufString (FcStrBuf *buf, const FcChar8 *s);

FcPrivate FcBool
FcStrBufData (FcStrBuf *buf, const FcChar8 *s, int len);

FcPrivate int
FcStrCmpIgnoreBlanksAndCase (const FcChar8 *s1, const FcChar8 *s2);

FcPrivate int
FcStrCmpIgnoreCaseAndDelims (const FcChar8 *s1, const FcChar8 *s2, const FcChar8 *delims);

FcPrivate const FcChar8 *
FcStrContainsIgnoreBlanksAndCase (const FcChar8 *s1, const FcChar8 *s2);

FcPrivate const FcChar8 *
FcStrContainsIgnoreCase (const FcChar8 *s1, const FcChar8 *s2);

FcPrivate const FcChar8 *
FcStrContainsWord (const FcChar8 *s1, const FcChar8 *s2);

FcPrivate int
FcStrMatchIgnoreCaseAndDelims (const FcChar8 *s1, const FcChar8 *s2, const FcChar8 *delims);

FcPrivate FcBool
FcStrGlobMatch (const FcChar8 *glob,
		const FcChar8 *string);

FcPrivate FcBool
FcStrUsesHome (const FcChar8 *s);

FcPrivate FcBool
FcStrIsAbsoluteFilename (const FcChar8 *s);

FcPrivate FcChar8 *
FcStrLastSlash (const FcChar8  *path);

FcPrivate FcChar32
FcStrHashIgnoreCase (const FcChar8 *s);

FcPrivate FcChar32
FcStrHashIgnoreBlanksAndCase (const FcChar8 *s);

FcPrivate FcChar8 *
FcStrRealPath (const FcChar8 *path);

FcPrivate FcChar8 *
FcStrCanonFilename (const FcChar8 *s);

FcPrivate FcBool
FcStrSerializeAlloc (FcSerialize *serialize, const FcChar8 *str);

FcPrivate FcChar8 *
FcStrSerialize (FcSerialize *serialize, const FcChar8 *str);

/* fcobjs.c */

FcPrivate void
FcObjectFini (void);

FcPrivate FcObject
FcObjectLookupIdByName (const char *str);

FcPrivate FcObject
FcObjectLookupBuiltinIdByName (const char *str);

FcPrivate const char *
FcObjectLookupOtherNameById (FcObject id);

FcPrivate const FcObjectType *
FcObjectLookupOtherTypeById (FcObject id);

FcPrivate const FcObjectType *
FcObjectLookupOtherTypeByName (const char *str);

/* fchash.c */
FcPrivate FcBool
FcHashStrCopy (const void  *src,
	       void       **dest);

FcPrivate FcBool
FcHashUuidCopy (const void  *src,
		void       **dest);

FcPrivate void
FcHashUuidFree (void *data);

FcPrivate FcHashTable *
FcHashTableCreate (FcHashFunc    hash_func,
		   FcCompareFunc compare_func,
		   FcCopyFunc    key_copy_func,
		   FcCopyFunc    value_copy_func,
		   FcDestroyFunc key_destroy_func,
		   FcDestroyFunc value_destroy_func);

FcPrivate void
FcHashTableDestroy (FcHashTable *table);

FcPrivate FcBool
FcHashTableFind (FcHashTable  *table,
		 const void   *key,
		 void        **value);

FcPrivate FcBool
FcHashTableAdd (FcHashTable *table,
		void        *key,
		void        *value);

FcPrivate FcBool
FcHashTableReplace (FcHashTable *table,
		    void        *key,
		    void        *value);

FcPrivate FcBool
FcHashTableRemove (FcHashTable *table,
		   void        *key);

#endif /* _FC_INT_H_ */
