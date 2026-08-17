/* Unicode name database interface */
#ifndef Py_LIMITED_API
#ifndef Py_UCNHASH_H
#define Py_UCNHASH_H
#ifdef __cplusplus
extern "C" {
#endif

/* revised ucnhash CAPI interface (exported through a "wrapper") */

#define PyUnicodeData_CAPSULE_NAME "unicodedata.ucnhash_CAPI"

typedef struct {

    /* Size of this struct */
    int size;

    /* Get name for a given character code.  Returns non-zero if
       success, zero if not.  Does not set Python exceptions.
       If self is NULL, data come from the default version of the database.
       If it is not NULL, it should be a unicodedata.ucd_X_Y_Z object */
    int (*getname)(PyObject *self, Py_UCS4 code, char* buffer, int buflen,
                   int with_alias_and_seq);

    /* Get character code for a given name.  Same error handling
       as for getname. */
    int (*getcode)(PyObject *self, const char* name, int namelen, Py_UCS4* code,
                   int with_named_seq);

} _PyUnicode_Name_CAPI;

#ifdef __cplusplus
}
#endif
#endif /* !Py_UCNHASH_H */
#endif /* !Py_LIMITED_API */
