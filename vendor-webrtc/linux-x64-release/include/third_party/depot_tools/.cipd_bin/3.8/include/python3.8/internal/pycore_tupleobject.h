#ifndef Py_INTERNAL_TUPLEOBJECT_H
#define Py_INTERNAL_TUPLEOBJECT_H
#ifdef __cplusplus
extern "C" {
#endif

#ifndef Py_BUILD_CORE
#  error "this header requires Py_BUILD_CORE define"
#endif

#include "tupleobject.h"

#define _PyTuple_ITEMS(op) (_PyTuple_CAST(op)->ob_item)
PyAPI_FUNC(PyObject *) _PyTuple_FromArray(PyObject *const *, Py_ssize_t);

#ifdef __cplusplus
}
#endif
#endif   /* !Py_INTERNAL_TUPLEOBJECT_H */
