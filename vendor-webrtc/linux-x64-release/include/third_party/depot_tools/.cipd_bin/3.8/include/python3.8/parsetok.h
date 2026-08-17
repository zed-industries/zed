/* Parser-tokenizer link interface */

#ifndef Py_LIMITED_API
#ifndef Py_PARSETOK_H
#define Py_PARSETOK_H
#ifdef __cplusplus
extern "C" {
#endif

#include "grammar.h"      /* grammar */
#include "node.h"         /* node */

typedef struct {
    int error;
    PyObject *filename;
    int lineno;
    int offset;
    char *text;                 /* UTF-8-encoded string */
    int token;
    int expected;
} perrdetail;

#if 0
#define PyPARSE_YIELD_IS_KEYWORD        0x0001
#endif

#define PyPARSE_DONT_IMPLY_DEDENT       0x0002

#if 0
#define PyPARSE_WITH_IS_KEYWORD         0x0003
#define PyPARSE_PRINT_IS_FUNCTION       0x0004
#define PyPARSE_UNICODE_LITERALS        0x0008
#endif

#define PyPARSE_IGNORE_COOKIE 0x0010
#define PyPARSE_BARRY_AS_BDFL 0x0020
#define PyPARSE_TYPE_COMMENTS 0x0040
#define PyPARSE_ASYNC_HACKS   0x0080

PyAPI_FUNC(node *) PyParser_ParseString(const char *, grammar *, int,
                                              perrdetail *);
PyAPI_FUNC(node *) PyParser_ParseFile (FILE *, const char *, grammar *, int,
                                             const char *, const char *,
                                             perrdetail *);

PyAPI_FUNC(node *) PyParser_ParseStringFlags(const char *, grammar *, int,
                                              perrdetail *, int);
PyAPI_FUNC(node *) PyParser_ParseFileFlags(
    FILE *fp,
    const char *filename,       /* decoded from the filesystem encoding */
    const char *enc,
    grammar *g,
    int start,
    const char *ps1,
    const char *ps2,
    perrdetail *err_ret,
    int flags);
PyAPI_FUNC(node *) PyParser_ParseFileFlagsEx(
    FILE *fp,
    const char *filename,       /* decoded from the filesystem encoding */
    const char *enc,
    grammar *g,
    int start,
    const char *ps1,
    const char *ps2,
    perrdetail *err_ret,
    int *flags);
PyAPI_FUNC(node *) PyParser_ParseFileObject(
    FILE *fp,
    PyObject *filename,
    const char *enc,
    grammar *g,
    int start,
    const char *ps1,
    const char *ps2,
    perrdetail *err_ret,
    int *flags);

PyAPI_FUNC(node *) PyParser_ParseStringFlagsFilename(
    const char *s,
    const char *filename,       /* decoded from the filesystem encoding */
    grammar *g,
    int start,
    perrdetail *err_ret,
    int flags);
PyAPI_FUNC(node *) PyParser_ParseStringFlagsFilenameEx(
    const char *s,
    const char *filename,       /* decoded from the filesystem encoding */
    grammar *g,
    int start,
    perrdetail *err_ret,
    int *flags);
PyAPI_FUNC(node *) PyParser_ParseStringObject(
    const char *s,
    PyObject *filename,
    grammar *g,
    int start,
    perrdetail *err_ret,
    int *flags);

/* Note that the following functions are defined in pythonrun.c,
   not in parsetok.c */
PyAPI_FUNC(void) PyParser_SetError(perrdetail *);
PyAPI_FUNC(void) PyParser_ClearError(perrdetail *);

#ifdef __cplusplus
}
#endif
#endif /* !Py_PARSETOK_H */
#endif /* !Py_LIMITED_API */
