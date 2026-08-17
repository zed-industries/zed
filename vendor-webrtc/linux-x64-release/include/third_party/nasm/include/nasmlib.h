/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2023 The NASM Authors - All Rights Reserved
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
 * nasmlib.h    header file for nasmlib.c
 */

#ifndef NASM_NASMLIB_H
#define NASM_NASMLIB_H

#include "compiler.h"
#include "bytesex.h"

/*
 * Useful construct for private values
 */
union intorptr {
    int64_t i;
    uint64_t u;
    size_t s;
    void *p;
    const void *cp;
    uintptr_t up;
};
typedef union intorptr intorptr;

/*
 * Wrappers around malloc, realloc, free and a few more. nasm_malloc
 * will fatal-error and die rather than return NULL; nasm_realloc will
 * do likewise, and will also guarantee to work right on being passed
 * a NULL pointer; nasm_free will do nothing if it is passed a NULL
 * pointer.
 */
void * safe_malloc(1) nasm_malloc(size_t);
void * safe_malloc(1) nasm_zalloc(size_t);
void * safe_malloc2(1,2) nasm_calloc(size_t, size_t);
void * safe_realloc(2) nasm_realloc(void *, size_t);
void nasm_free(void *);
char * safe_alloc nasm_strdup(const char *);
char * safe_alloc nasm_strndup(const char *, size_t);
char * safe_alloc nasm_strcat(const char *one, const char *two);
char * safe_alloc end_with_null nasm_strcatn(const char *one, ...);

/*
 * nasm_[v]asprintf() are variants of the semi-standard [v]asprintf()
 * functions, except that we return the pointer instead of a count.
 * The size of the string (including the final NUL!) is available
 * by calling nasm_aprintf_size() afterwards.
 *
 * nasm_[v]axprintf() are similar, but allocates a user-defined amount
 * of storage before the string, and returns a pointer to the
 * allocated buffer. The value of nasm_aprintf_size() does *not* include
 * this additional storage.
 */
char * safe_alloc printf_func(1, 2) nasm_asprintf(const char *fmt, ...);
char * safe_alloc vprintf_func(1) nasm_vasprintf(const char *fmt, va_list ap);
void * safe_alloc printf_func(2, 3) nasm_axprintf(size_t extra, const char *fmt, ...);
void * safe_alloc vprintf_func(2) nasm_vaxprintf(size_t extra, const char *fmt, va_list ap);

/*
 * nasm_last_string_len() returns the length of the last string allocated
 * by [v]asprintf, nasm_strdup, nasm_strcat, or nasm_strcatn.
 *
 * nasm_last_string_size() returns the equivalent size including the
 * final NUL.
 */
static inline size_t nasm_last_string_len(void)
{
    extern size_t _nasm_last_string_size;
    return _nasm_last_string_size - 1;
}
static inline size_t nasm_last_string_size(void)
{
    extern size_t _nasm_last_string_size;
    return _nasm_last_string_size;
}

/* Assert the argument is a pointer without evaluating it */
#define nasm_assert_pointer(p) ((void)sizeof(*(p)))

#define nasm_new(p) ((p) = nasm_zalloc(sizeof(*(p))))
#define nasm_newn(p,n) ((p) = nasm_calloc((n), sizeof(*(p))))
/*
 * This is broken on platforms where there are pointers which don't
 * match void * in their internal layout.  It unfortunately also
 * loses any "const" part of the argument, although hopefully the
 * compiler will warn in that case.
 */
#define nasm_delete(p)                          \
    do {                                        \
        void **_pp = (void **)&(p);             \
        nasm_assert_pointer(p);                 \
        nasm_free(*_pp);                        \
        *_pp = NULL;                            \
    } while (0)
#define nasm_zero(x) (memset(&(x), 0, sizeof(x)))
#define nasm_zeron(p,n) (memset((p), 0, (n)*sizeof(*(p))))

/*
 * Wrappers around fread()/fwrite() which fatal-errors on failure.
 * For fread(), only use this if EOF is supposed to be a fatal error!
 */
void nasm_read(void *, size_t, FILE *);
void nasm_write(const void *, size_t, FILE *);

/*
 * NASM failure at build time if the argument is false
 */
#ifdef static_assert
# define nasm_static_assert(x) static_assert((x), #x)
#elif defined(HAVE_FUNC_ATTRIBUTE_ERROR) && defined(__OPTIMIZE__)
# define nasm_static_assert(x)                                              \
    do {                                                                    \
        if (!(x)) {                                                         \
            extern void __attribute__((error("assertion " #x " failed")))   \
                _nasm_static_fail(void);                                    \
            _nasm_static_fail();                                            \
        }                                                                   \
    } while (0)
#else
/* See http://www.drdobbs.com/compile-time-assertions/184401873 */
# define nasm_static_assert(x)                                              \
    do { enum { _static_assert_failed = 1/(!!(x)) }; } while (0)
#endif

/*
 * conditional static assert, if we know it is possible to determine
 * the assert value at compile time. Since if_constant triggers
 * pedantic warnings on gcc, turn them off explicitly around this code.
 */
#ifdef static_assert
# define nasm_try_static_assert(x)                                          \
    do {                                                                    \
        not_pedantic_start                                                  \
        static_assert(if_constant(x, true), #x);                            \
        not_pedantic_end                                                    \
    } while (0)
#elif defined(HAVE_FUNC_ATTRIBUTE_ERROR) && defined(__OPTIMIZE__)
# define nasm_try_static_assert(x)                                          \
    do {                                                                    \
        if (!if_constant(x, true)) {                                        \
            extern void __attribute__((error("assertion " #x " failed")))   \
                _nasm_static_fail(void);                                    \
            _nasm_static_fail();                                            \
        }                                                                   \
    } while (0)
#else
# define nasm_try_static_assert(x) ((void)0)
#endif

/*
 * NASM assert failure
 */
fatal_func nasm_assert_failed(const char *, int, const char *);
#define nasm_assert(x)                                          \
    do {                                                        \
        nasm_try_static_assert(x);                              \
        if (unlikely(!(x)))                                     \
            nasm_assert_failed(__FILE__,__LINE__,#x);           \
    } while (0)

/* Utility function to generate a string for an invalid enum */
const char *invalid_enum_str(int);

/*
 * ANSI doesn't guarantee the presence of `stricmp' or
 * `strcasecmp'.
 */
#if defined(HAVE_STRCASECMP)
#define nasm_stricmp strcasecmp
#elif defined(HAVE_STRICMP)
#define nasm_stricmp stricmp
#else
int pure_func nasm_stricmp(const char *, const char *);
#endif

#if defined(HAVE_STRNCASECMP)
#define nasm_strnicmp strncasecmp
#elif defined(HAVE_STRNICMP)
#define nasm_strnicmp strnicmp
#else
int pure_func nasm_strnicmp(const char *, const char *, size_t);
#endif

int pure_func nasm_memicmp(const char *, const char *, size_t);

#if defined(HAVE_STRSEP)
#define nasm_strsep strsep
#else
char *nasm_strsep(char **stringp, const char *delim);
#endif

#ifndef HAVE_DECL_STRNLEN
size_t pure_func strnlen(const char *, size_t);
#endif

/* This returns the numeric value of a given 'digit'; no check for validity */
static inline unsigned int numvalue(unsigned char c)
{
    c |= 0x20;
    return c >= 'a' ? c - 'a' + 10 : c - '0';
}

/*
 * Convert a string into a number, using NASM number rules. Sets
 * `*error' to true if an error occurs, and false otherwise.
 */
int64_t readnum(const char *str, bool *error);

/*
 * Get the numeric base corresponding to a character
 */
static inline unsigned int radix_letter(char c)
{
    switch (c) {
    case 'b': case 'B':
    case 'y': case 'Y':
	return 2;		/* Binary */
    case 'o': case 'O':
    case 'q': case 'Q':
	return 8;		/* Octal */
    case 'h': case 'H':
    case 'x': case 'X':
	return 16;		/* Hexadecimal */
    case 'd': case 'D':
    case 't': case 'T':
	return 10;		/* Decimal */
    default:
	return 0;		/* Not a known radix letter */
    }
}

/*
 * Convert a character constant into a number. Sets
 * `*warn' to true if an overflow occurs, and false otherwise.
 * str points to and length covers the middle of the string,
 * without the quotes.
 */
int64_t readstrnum(char *str, int length, bool *warn);

/*
 * Produce an unsigned integer string from a number with a specified
 * base, digits and signedness
 */
#define NUMSTR_MAXBASE 64
int numstr(char *buf, size_t buflen, uint64_t n,
           int digits, unsigned int base, bool ucase);

/*
 * seg_alloc: allocate a hitherto unused segment number.
 */
int32_t seg_alloc(void);

/*
 * Add/replace or remove an extension to the end of a filename
 */
const char *filename_set_extension(const char *inname, const char *extension);

/*
 * Utility macros...
 *
 * This is a useful #define which I keep meaning to use more often:
 * the number of elements of a statically defined array.
 */
#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof((arr)[0]))

/*
 * List handling
 *
 *  list_for_each - regular iterator over list
 *  list_for_each_safe - the same but safe against list items removal
 *  list_last - find the last element in a list
 *  list_reverse - reverse the order of a list
 *
 *  Arguments named with _ + single letter should be temp variables
 *  of the appropriate pointer type.
 */
#define list_for_each(pos, head)                        \
    for (pos = head; pos; pos = pos->next)
#define list_for_each_safe(pos, _n, head)                \
    for (pos = head, _n = (pos ? pos->next : NULL); pos; \
        pos = _n, _n = (_n ? _n->next : NULL))
#define list_last(pos, head)                            \
    for (pos = head; pos && pos->next; pos = pos->next) \
        ;
#define list_reverse(head)                              \
    do {                                                \
        void *_p, *_n;                                  \
        if (!head || !head->next)                       \
            break;                                      \
        _p = NULL;                                      \
        while (head) {                                  \
            _n = head->next;                            \
            head->next = _p;                            \
            _p = head;                                  \
            head = _n;                                  \
        }                                               \
        head = _p;                                      \
    } while (0)

/*
 * Power of 2 align helpers
 */
#undef ALIGN_MASK               /* Some BSD flavors define these in system headers */
#undef ALIGN
#define ALIGN_MASK(v, mask)     (((v) + (mask)) & ~(mask))
#define ALIGN(v, a)             ALIGN_MASK(v, (a) - 1)
#define IS_ALIGNED(v, a)        (((v) & ((a) - 1)) == 0)

/*
 * Routines to write littleendian data to a file
 */
#define fwriteint8_t(d,f) putc(d,f)
void fwriteint16_t(uint16_t data, FILE * fp);
void fwriteint32_t(uint32_t data, FILE * fp);
void fwriteint64_t(uint64_t data, FILE * fp);
void fwriteaddr(uint64_t data, int size, FILE * fp);

/*
 * Binary search routine. Returns index into `array' of an entry
 * matching `string', or <0 if no match. `array' is taken to
 * contain `size' elements.
 *
 * bsi() is case sensitive, bsii() is case insensitive.
 */
int bsi(const char *string, const char **array, int size);
int bsii(const char *string, const char **array, int size);

/*
 * Convenient string processing helper routines
 */
char *nasm_skip_spaces(const char *p);
char *nasm_skip_word(const char *p);
char *nasm_zap_spaces_fwd(char *p);
char *nasm_zap_spaces_rev(char *p);
char *nasm_trim_spaces(char *p);
char *nasm_get_word(char *p, char **tail);
char *nasm_opt_val(char *p, char **opt, char **val);

/*
 * Converts a relative pathname rel_path into an absolute path name.
 *
 * The buffer returned must be freed by the caller
 */
char * safe_alloc nasm_realpath(const char *rel_path);

/*
 * Path-splitting and merging functions
 */
char * safe_alloc nasm_dirname(const char *path);
char * safe_alloc nasm_basename(const char *path);
char * safe_alloc nasm_catfile(const char *dir, const char *path);

const char * pure_func prefix_name(int);

/*
 * Wrappers around fopen()... for future change to a dedicated structure
 */
enum file_flags {
    NF_BINARY   = 0x00000000,   /* Binary file (default) */
    NF_TEXT     = 0x00000001,   /* Text file */
    NF_NONFATAL = 0x00000000,   /* Don't die on open failure (default) */
    NF_FATAL    = 0x00000002,   /* Die on open failure */
    NF_FORMAP   = 0x00000004,   /* Intended to use nasm_map_file() */
    NF_IONBF    = 0x00000010,   /* Force unbuffered stdio */
    NF_IOLBF    = 0x00000020,   /* Force line buffered stdio */
    NF_IOFBF    = 0000000030    /* Force fully buffered stdio */
};
#define NF_BUF_MASK  0x30

FILE *nasm_open_read(const char *filename, enum file_flags flags);
FILE *nasm_open_write(const char *filename, enum file_flags flags);

void nasm_set_binary_mode(FILE *f);

/* Probe for existence of a file */
bool nasm_file_exists(const char *filename);

#define ZERO_BUF_SIZE 65536     /* Default value */
#if defined(BUFSIZ) && (BUFSIZ > ZERO_BUF_SIZE)
# undef ZERO_BUF_SIZE
# define ZERO_BUF_SIZE BUFSIZ
#endif
extern const uint8_t zero_buffer[ZERO_BUF_SIZE];

/* Missing fseeko/ftello */
#ifndef HAVE_FSEEKO
# undef off_t                   /* Just in case it is a macro */
# ifdef HAVE__FSEEKI64
#  define fseeko _fseeki64
#  define ftello _ftelli64
#  define off_t  int64_t
# else
#  define fseeko fseek
#  define ftello ftell
#  define off_t  long
# endif
#endif

const void *nasm_map_file(FILE *fp, off_t start, off_t len);
void nasm_unmap_file(const void *p, size_t len);
off_t nasm_file_size(FILE *f);
off_t nasm_file_size_by_path(const char *pathname);
bool nasm_file_time(time_t *t, const char *pathname);
void fwritezero(off_t bytes, FILE *fp);

static inline bool const_func overflow_general(int64_t value, int bytes)
{
    int sbit;
    int64_t vmax, vmin;

    if (bytes >= 8)
        return false;

    sbit = (bytes << 3) - 1;
    vmax =  ((int64_t)2 << sbit) - 1;
    vmin = -((int64_t)2 << sbit);

    return value < vmin || value > vmax;
}

static inline bool const_func overflow_signed(int64_t value, int bytes)
{
    int sbit;
    int64_t vmax, vmin;

    if (bytes >= 8)
        return false;

    sbit = (bytes << 3) - 1;
    vmax =  ((int64_t)1 << sbit) - 1;
    vmin = -((int64_t)1 << sbit);

    return value < vmin || value > vmax;
}

static inline bool const_func overflow_unsigned(int64_t value, int bytes)
{
    int sbit;
    int64_t vmax, vmin;

    if (bytes >= 8)
        return false;

    sbit = (bytes << 3) - 1;
    vmax = ((int64_t)2 << sbit) - 1;
    vmin = 0;

    return value < vmin || value > vmax;
}

static inline int64_t const_func signed_bits(int64_t value, int bits)
{
    if (bits < 64) {
        value &= ((int64_t)1 << bits) - 1;
        if (value & (int64_t)1 << (bits - 1))
            value |= (int64_t)((uint64_t)-1 << bits);
    }
    return value;
}

/* check if value is power of 2 */
#define is_power2(v)   ((v) && ((v) & ((v) - 1)) == 0)

/* try to get the system stack size */
extern size_t nasm_get_stack_size_limit(void);

#endif
