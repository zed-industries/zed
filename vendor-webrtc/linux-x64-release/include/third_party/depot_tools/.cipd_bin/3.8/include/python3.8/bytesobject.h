
/* Bytes (String) object interface */

#ifndef Py_BYTESOBJECT_H
#define Py_BYTESOBJECT_H
#ifdef __cplusplus
extern "C" {
#endif

#include <stdarg.h>

/*
Type PyBytesObject represents a character string.  An extra zero byte is
reserved at the end to ensure it is zero-terminated, but a size is
present so strings with null bytes in them can be represented.  This
is an immutable object type.

There are functions to create new string objects, to test
an object for string-ness, and to get the
string value.  The latter function returns a null pointer
if the object is not of the proper type.
There is a variant that takes an explicit size as well as a
variant that assumes a zero-terminated string.  Note that none of the
functions should be applied to nil objects.
*/

/* Caching the hash (ob_shash) saves recalculation of a string's hash value.
   This significantly speeds up dict lookups. */

#ifndef Py_LIMITED_API
typedef struct {
    PyObject_VAR_HEAD
    Py_hash_t ob_shash;
    char ob_sval[1];

    /* Invariants:
     *     ob_sval contains space for 'ob_size+1' elements.
     *     ob_sval[ob_size] == 0.
     *     ob_shash is the hash of the string or -1 if not computed yet.
     */
} PyBytesObject;
#endif

PyAPI_DATA(PyTypeObject) PyBytes_Type;
PyAPI_DATA(PyTypeObject) PyBytesIter_Type;

#define PyBytes_Check(op) \
                 PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_BYTES_SUBCLASS)
#define PyBytes_CheckExact(op) (Py_TYPE(op) == &PyBytes_Type)

PyAPI_FUNC(PyObject *) PyBytes_FromStringAndSize(const char *, Py_ssize_t);
PyAPI_FUNC(PyObject *) PyBytes_FromString(const char *);
PyAPI_FUNC(PyObject *) PyBytes_FromObject(PyObject *);
PyAPI_FUNC(PyObject *) PyBytes_FromFormatV(const char*, va_list)
                                Py_GCC_ATTRIBUTE((format(printf, 1, 0)));
PyAPI_FUNC(PyObject *) PyBytes_FromFormat(const char*, ...)
                                Py_GCC_ATTRIBUTE((format(printf, 1, 2)));
PyAPI_FUNC(Py_ssize_t) PyBytes_Size(PyObject *);
PyAPI_FUNC(char *) PyBytes_AsString(PyObject *);
PyAPI_FUNC(PyObject *) PyBytes_Repr(PyObject *, int);
PyAPI_FUNC(void) PyBytes_Concat(PyObject **, PyObject *);
PyAPI_FUNC(void) PyBytes_ConcatAndDel(PyObject **, PyObject *);
#ifndef Py_LIMITED_API
PyAPI_FUNC(int) _PyBytes_Resize(PyObject **, Py_ssize_t);
PyAPI_FUNC(PyObject*) _PyBytes_FormatEx(
    const char *format,
    Py_ssize_t format_len,
    PyObject *args,
    int use_bytearray);
PyAPI_FUNC(PyObject*) _PyBytes_FromHex(
    PyObject *string,
    int use_bytearray);
#endif
PyAPI_FUNC(PyObject *) PyBytes_DecodeEscape(const char *, Py_ssize_t,
                                            const char *, Py_ssize_t,
                                            const char *);
#ifndef Py_LIMITED_API
/* Helper for PyBytes_DecodeEscape that detects invalid escape chars. */
PyAPI_FUNC(PyObject *) _PyBytes_DecodeEscape(const char *, Py_ssize_t,
                                             const char *, Py_ssize_t,
                                             const char *,
                                             const char **);
#endif

/* Macro, trading safety for speed */
#ifndef Py_LIMITED_API
#define PyBytes_AS_STRING(op) (assert(PyBytes_Check(op)), \
                                (((PyBytesObject *)(op))->ob_sval))
#define PyBytes_GET_SIZE(op)  (assert(PyBytes_Check(op)),Py_SIZE(op))
#endif

/* _PyBytes_Join(sep, x) is like sep.join(x).  sep must be PyBytesObject*,
   x must be an iterable object. */
#ifndef Py_LIMITED_API
PyAPI_FUNC(PyObject *) _PyBytes_Join(PyObject *sep, PyObject *x);
#endif

/* Provides access to the internal data buffer and size of a string
   object or the default encoded version of a Unicode object. Passing
   NULL as *len parameter will force the string buffer to be
   0-terminated (passing a string with embedded NULL characters will
   cause an exception).  */
PyAPI_FUNC(int) PyBytes_AsStringAndSize(
    PyObject *obj,      /* string or Unicode object */
    char **s,           /* pointer to buffer variable */
    Py_ssize_t *len     /* pointer to length variable or NULL
                           (only possible for 0-terminated
                           strings) */
    );

/* Using the current locale, insert the thousands grouping
   into the string pointed to by buffer.  For the argument descriptions,
   see Objects/stringlib/localeutil.h */
#ifndef Py_LIMITED_API
PyAPI_FUNC(Py_ssize_t) _PyBytes_InsertThousandsGroupingLocale(char *buffer,
                                                   Py_ssize_t n_buffer,
                                                   char *digits,
                                                   Py_ssize_t n_digits,
                                                   Py_ssize_t min_width);

/* Using explicit passed-in values, insert the thousands grouping
   into the string pointed to by buffer.  For the argument descriptions,
   see Objects/stringlib/localeutil.h */
PyAPI_FUNC(Py_ssize_t) _PyBytes_InsertThousandsGrouping(char *buffer,
                                                   Py_ssize_t n_buffer,
                                                   char *digits,
                                                   Py_ssize_t n_digits,
                                                   Py_ssize_t min_width,
                                                   const char *grouping,
                                                   const char *thousands_sep);
#endif

/* Flags used by string formatting */
#define F_LJUST (1<<0)
#define F_SIGN  (1<<1)
#define F_BLANK (1<<2)
#define F_ALT   (1<<3)
#define F_ZERO  (1<<4)

#ifndef Py_LIMITED_API
/* The _PyBytesWriter structure is big: it contains an embedded "stack buffer".
   A _PyBytesWriter variable must be declared at the end of variables in a
   function to optimize the memory allocation on the stack. */
typedef struct {
    /* bytes, bytearray or NULL (when the small buffer is used) */
    PyObject *buffer;

    /* Number of allocated size. */
    Py_ssize_t allocated;

    /* Minimum number of allocated bytes,
       incremented by _PyBytesWriter_Prepare() */
    Py_ssize_t min_size;

    /* If non-zero, use a bytearray instead of a bytes object for buffer. */
    int use_bytearray;

    /* If non-zero, overallocate the buffer (default: 0).
       This flag must be zero if use_bytearray is non-zero. */
    int overallocate;

    /* Stack buffer */
    int use_small_buffer;
    char small_buffer[512];
} _PyBytesWriter;

/* Initialize a bytes writer

   By default, the overallocation is disabled. Set the overallocate attribute
   to control the allocation of the buffer. */
PyAPI_FUNC(void) _PyBytesWriter_Init(_PyBytesWriter *writer);

/* Get the buffer content and reset the writer.
   Return a bytes object, or a bytearray object if use_bytearray is non-zero.
   Raise an exception and return NULL on error. */
PyAPI_FUNC(PyObject *) _PyBytesWriter_Finish(_PyBytesWriter *writer,
    void *str);

/* Deallocate memory of a writer (clear its internal buffer). */
PyAPI_FUNC(void) _PyBytesWriter_Dealloc(_PyBytesWriter *writer);

/* Allocate the buffer to write size bytes.
   Return the pointer to the beginning of buffer data.
   Raise an exception and return NULL on error. */
PyAPI_FUNC(void*) _PyBytesWriter_Alloc(_PyBytesWriter *writer,
    Py_ssize_t size);

/* Ensure that the buffer is large enough to write *size* bytes.
   Add size to the writer minimum size (min_size attribute).

   str is the current pointer inside the buffer.
   Return the updated current pointer inside the buffer.
   Raise an exception and return NULL on error. */
PyAPI_FUNC(void*) _PyBytesWriter_Prepare(_PyBytesWriter *writer,
    void *str,
    Py_ssize_t size);

/* Resize the buffer to make it larger.
   The new buffer may be larger than size bytes because of overallocation.
   Return the updated current pointer inside the buffer.
   Raise an exception and return NULL on error.

   Note: size must be greater than the number of allocated bytes in the writer.

   This function doesn't use the writer minimum size (min_size attribute).

   See also _PyBytesWriter_Prepare().
   */
PyAPI_FUNC(void*) _PyBytesWriter_Resize(_PyBytesWriter *writer,
    void *str,
    Py_ssize_t size);

/* Write bytes.
   Raise an exception and return NULL on error. */
PyAPI_FUNC(void*) _PyBytesWriter_WriteBytes(_PyBytesWriter *writer,
    void *str,
    const void *bytes,
    Py_ssize_t size);
#endif   /* Py_LIMITED_API */

#ifdef __cplusplus
}
#endif
#endif /* !Py_BYTESOBJECT_H */
