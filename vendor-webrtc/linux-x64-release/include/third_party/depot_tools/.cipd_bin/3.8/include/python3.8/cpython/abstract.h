#ifndef Py_CPYTHON_ABSTRACTOBJECT_H
#  error "this header file must not be included directly"
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* === Object Protocol ================================================== */

#ifdef PY_SSIZE_T_CLEAN
#  define _PyObject_CallMethodId _PyObject_CallMethodId_SizeT
#endif

/* Convert keyword arguments from the FASTCALL (stack: C array, kwnames: tuple)
   format to a Python dictionary ("kwargs" dict).

   The type of kwnames keys is not checked. The final function getting
   arguments is responsible to check if all keys are strings, for example using
   PyArg_ParseTupleAndKeywords() or PyArg_ValidateKeywordArguments().

   Duplicate keys are merged using the last value. If duplicate keys must raise
   an exception, the caller is responsible to implement an explicit keys on
   kwnames. */
PyAPI_FUNC(PyObject *) _PyStack_AsDict(
    PyObject *const *values,
    PyObject *kwnames);

/* Convert (args, nargs, kwargs: dict) into a (stack, nargs, kwnames: tuple).

   Return 0 on success, raise an exception and return -1 on error.

   Write the new stack into *p_stack. If *p_stack is differen than args, it
   must be released by PyMem_Free().

   The stack uses borrowed references.

   The type of keyword keys is not checked, these checks should be done
   later (ex: _PyArg_ParseStackAndKeywords). */
PyAPI_FUNC(int) _PyStack_UnpackDict(
    PyObject *const *args,
    Py_ssize_t nargs,
    PyObject *kwargs,
    PyObject *const **p_stack,
    PyObject **p_kwnames);

/* Suggested size (number of positional arguments) for arrays of PyObject*
   allocated on a C stack to avoid allocating memory on the heap memory. Such
   array is used to pass positional arguments to call functions of the
   _PyObject_Vectorcall() family.

   The size is chosen to not abuse the C stack and so limit the risk of stack
   overflow. The size is also chosen to allow using the small stack for most
   function calls of the Python standard library. On 64-bit CPU, it allocates
   40 bytes on the stack. */
#define _PY_FASTCALL_SMALL_STACK 5

PyAPI_FUNC(PyObject *) _Py_CheckFunctionResult(PyObject *callable,
                                               PyObject *result,
                                               const char *where);

/* === Vectorcall protocol (PEP 590) ============================= */

/* Call callable using tp_call. Arguments are like _PyObject_Vectorcall()
   or _PyObject_FastCallDict() (both forms are supported),
   except that nargs is plainly the number of arguments without flags. */
PyAPI_FUNC(PyObject *) _PyObject_MakeTpCall(
    PyObject *callable,
    PyObject *const *args, Py_ssize_t nargs,
    PyObject *keywords);

#define PY_VECTORCALL_ARGUMENTS_OFFSET ((size_t)1 << (8 * sizeof(size_t) - 1))

static inline Py_ssize_t
PyVectorcall_NARGS(size_t n)
{
    return n & ~PY_VECTORCALL_ARGUMENTS_OFFSET;
}

static inline vectorcallfunc
_PyVectorcall_Function(PyObject *callable)
{
    PyTypeObject *tp = Py_TYPE(callable);
    Py_ssize_t offset = tp->tp_vectorcall_offset;
    vectorcallfunc ptr;
    if (!PyType_HasFeature(tp, _Py_TPFLAGS_HAVE_VECTORCALL)) {
        return NULL;
    }
    assert(PyCallable_Check(callable));
    assert(offset > 0);
    memcpy(&ptr, (char *) callable + offset, sizeof(ptr));
    return ptr;
}

/* Call the callable object 'callable' with the "vectorcall" calling
   convention.

   args is a C array for positional arguments.

   nargsf is the number of positional arguments plus optionally the flag
   PY_VECTORCALL_ARGUMENTS_OFFSET which means that the caller is allowed to
   modify args[-1].

   kwnames is a tuple of keyword names. The values of the keyword arguments
   are stored in "args" after the positional arguments (note that the number
   of keyword arguments does not change nargsf). kwnames can also be NULL if
   there are no keyword arguments.

   keywords must only contains str strings (no subclass), and all keys must
   be unique.

   Return the result on success. Raise an exception and return NULL on
   error. */
static inline PyObject *
_PyObject_Vectorcall(PyObject *callable, PyObject *const *args,
                     size_t nargsf, PyObject *kwnames)
{
    PyObject *res;
    vectorcallfunc func;
    assert(kwnames == NULL || PyTuple_Check(kwnames));
    assert(args != NULL || PyVectorcall_NARGS(nargsf) == 0);
    func = _PyVectorcall_Function(callable);
    if (func == NULL) {
        Py_ssize_t nargs = PyVectorcall_NARGS(nargsf);
        return _PyObject_MakeTpCall(callable, args, nargs, kwnames);
    }
    res = func(callable, args, nargsf, kwnames);
    return _Py_CheckFunctionResult(callable, res, NULL);
}

/* Same as _PyObject_Vectorcall except that keyword arguments are passed as
   dict, which may be NULL if there are no keyword arguments. */
PyAPI_FUNC(PyObject *) _PyObject_FastCallDict(
    PyObject *callable,
    PyObject *const *args,
    size_t nargsf,
    PyObject *kwargs);

/* Call "callable" (which must support vectorcall) with positional arguments
   "tuple" and keyword arguments "dict". "dict" may also be NULL */
PyAPI_FUNC(PyObject *) PyVectorcall_Call(PyObject *callable, PyObject *tuple, PyObject *dict);

/* Same as _PyObject_Vectorcall except without keyword arguments */
static inline PyObject *
_PyObject_FastCall(PyObject *func, PyObject *const *args, Py_ssize_t nargs)
{
    return _PyObject_Vectorcall(func, args, (size_t)nargs, NULL);
}

/* Call a callable without any arguments */
static inline PyObject *
_PyObject_CallNoArg(PyObject *func) {
    return _PyObject_Vectorcall(func, NULL, 0, NULL);
}

PyAPI_FUNC(PyObject *) _PyObject_Call_Prepend(
    PyObject *callable,
    PyObject *obj,
    PyObject *args,
    PyObject *kwargs);

PyAPI_FUNC(PyObject *) _PyObject_FastCall_Prepend(
    PyObject *callable,
    PyObject *obj,
    PyObject *const *args,
    Py_ssize_t nargs);

/* Like PyObject_CallMethod(), but expect a _Py_Identifier*
   as the method name. */
PyAPI_FUNC(PyObject *) _PyObject_CallMethodId(PyObject *obj,
                                              _Py_Identifier *name,
                                              const char *format, ...);

PyAPI_FUNC(PyObject *) _PyObject_CallMethodId_SizeT(PyObject *obj,
                                                    _Py_Identifier *name,
                                                    const char *format,
                                                    ...);

PyAPI_FUNC(PyObject *) _PyObject_CallMethodIdObjArgs(
    PyObject *obj,
    struct _Py_Identifier *name,
    ...);

PyAPI_FUNC(int) _PyObject_HasLen(PyObject *o);

/* Guess the size of object 'o' using len(o) or o.__length_hint__().
   If neither of those return a non-negative value, then return the default
   value.  If one of the calls fails, this function returns -1. */
PyAPI_FUNC(Py_ssize_t) PyObject_LengthHint(PyObject *o, Py_ssize_t);

/* === New Buffer API ============================================ */

/* Return 1 if the getbuffer function is available, otherwise return 0. */
#define PyObject_CheckBuffer(obj) \
    (((obj)->ob_type->tp_as_buffer != NULL) &&  \
     ((obj)->ob_type->tp_as_buffer->bf_getbuffer != NULL))

/* This is a C-API version of the getbuffer function call.  It checks
   to make sure object has the required function pointer and issues the
   call.

   Returns -1 and raises an error on failure and returns 0 on success. */
PyAPI_FUNC(int) PyObject_GetBuffer(PyObject *obj, Py_buffer *view,
                                   int flags);

/* Get the memory area pointed to by the indices for the buffer given.
   Note that view->ndim is the assumed size of indices. */
PyAPI_FUNC(void *) PyBuffer_GetPointer(Py_buffer *view, Py_ssize_t *indices);

/* Return the implied itemsize of the data-format area from a
   struct-style description. */
PyAPI_FUNC(int) PyBuffer_SizeFromFormat(const char *);

/* Implementation in memoryobject.c */
PyAPI_FUNC(int) PyBuffer_ToContiguous(void *buf, Py_buffer *view,
                                      Py_ssize_t len, char order);

PyAPI_FUNC(int) PyBuffer_FromContiguous(Py_buffer *view, void *buf,
                                        Py_ssize_t len, char order);

/* Copy len bytes of data from the contiguous chunk of memory
   pointed to by buf into the buffer exported by obj.  Return
   0 on success and return -1 and raise a PyBuffer_Error on
   error (i.e. the object does not have a buffer interface or
   it is not working).

   If fort is 'F', then if the object is multi-dimensional,
   then the data will be copied into the array in
   Fortran-style (first dimension varies the fastest).  If
   fort is 'C', then the data will be copied into the array
   in C-style (last dimension varies the fastest).  If fort
   is 'A', then it does not matter and the copy will be made
   in whatever way is more efficient. */
PyAPI_FUNC(int) PyObject_CopyData(PyObject *dest, PyObject *src);

/* Copy the data from the src buffer to the buffer of destination. */
PyAPI_FUNC(int) PyBuffer_IsContiguous(const Py_buffer *view, char fort);

/*Fill the strides array with byte-strides of a contiguous
  (Fortran-style if fort is 'F' or C-style otherwise)
  array of the given shape with the given number of bytes
  per element. */
PyAPI_FUNC(void) PyBuffer_FillContiguousStrides(int ndims,
                                               Py_ssize_t *shape,
                                               Py_ssize_t *strides,
                                               int itemsize,
                                               char fort);

/* Fills in a buffer-info structure correctly for an exporter
   that can only share a contiguous chunk of memory of
   "unsigned bytes" of the given length.

   Returns 0 on success and -1 (with raising an error) on error. */
PyAPI_FUNC(int) PyBuffer_FillInfo(Py_buffer *view, PyObject *o, void *buf,
                                  Py_ssize_t len, int readonly,
                                  int flags);

/* Releases a Py_buffer obtained from getbuffer ParseTuple's "s*". */
PyAPI_FUNC(void) PyBuffer_Release(Py_buffer *view);

/* ==== Iterators ================================================ */

#define PyIter_Check(obj) \
    ((obj)->ob_type->tp_iternext != NULL && \
     (obj)->ob_type->tp_iternext != &_PyObject_NextNotImplemented)

/* === Number Protocol ================================================== */

#define PyIndex_Check(obj)                              \
    ((obj)->ob_type->tp_as_number != NULL &&            \
     (obj)->ob_type->tp_as_number->nb_index != NULL)

/* === Sequence protocol ================================================ */

/* Assume tp_as_sequence and sq_item exist and that 'i' does not
   need to be corrected for a negative index. */
#define PySequence_ITEM(o, i)\
    ( Py_TYPE(o)->tp_as_sequence->sq_item(o, i) )

#define PY_ITERSEARCH_COUNT    1
#define PY_ITERSEARCH_INDEX    2
#define PY_ITERSEARCH_CONTAINS 3

/* Iterate over seq.

   Result depends on the operation:

   PY_ITERSEARCH_COUNT:  return # of times obj appears in seq; -1 if
     error.
   PY_ITERSEARCH_INDEX:  return 0-based index of first occurrence of
     obj in seq; set ValueError and return -1 if none found;
     also return -1 on error.
   PY_ITERSEARCH_CONTAINS:  return 1 if obj in seq, else 0; -1 on
     error. */
PyAPI_FUNC(Py_ssize_t) _PySequence_IterSearch(PyObject *seq,
                                              PyObject *obj, int operation);

/* === Mapping protocol ================================================= */

PyAPI_FUNC(int) _PyObject_RealIsInstance(PyObject *inst, PyObject *cls);

PyAPI_FUNC(int) _PyObject_RealIsSubclass(PyObject *derived, PyObject *cls);

PyAPI_FUNC(char *const *) _PySequence_BytesToCharpArray(PyObject* self);

PyAPI_FUNC(void) _Py_FreeCharPArray(char *const array[]);

/* For internal use by buffer API functions */
PyAPI_FUNC(void) _Py_add_one_to_index_F(int nd, Py_ssize_t *index,
                                        const Py_ssize_t *shape);
PyAPI_FUNC(void) _Py_add_one_to_index_C(int nd, Py_ssize_t *index,
                                        const Py_ssize_t *shape);

/* Convert Python int to Py_ssize_t. Do nothing if the argument is None. */
PyAPI_FUNC(int) _Py_convert_optional_to_ssize_t(PyObject *, void *);

#ifdef __cplusplus
}
#endif
