/* Licensed under the Apache License: http://www.apache.org/licenses/LICENSE-2.0 */
/* For details: https://bitbucket.org/ned/coveragepy/src/default/NOTICE.txt */

#ifndef _COVERAGE_UTIL_H
#define _COVERAGE_UTIL_H

#include <Python.h>

/* Compile-time debugging helpers */
#undef WHAT_LOG         /* Define to log the WHAT params in the trace function. */
#undef TRACE_LOG        /* Define to log our bookkeeping. */
#undef COLLECT_STATS    /* Collect counters: stats are printed when tracer is stopped. */

/* Py 2.x and 3.x compatibility */

#if PY_MAJOR_VERSION >= 3

#define MyText_Type         PyUnicode_Type
#define MyText_AS_BYTES(o)  PyUnicode_AsASCIIString(o)
#define MyBytes_GET_SIZE(o) PyBytes_GET_SIZE(o)
#define MyBytes_AS_STRING(o) PyBytes_AS_STRING(o)
#define MyText_AsString(o)  PyUnicode_AsUTF8(o)
#define MyText_FromFormat   PyUnicode_FromFormat
#define MyInt_FromInt(i)    PyLong_FromLong((long)i)
#define MyInt_AsInt(o)      (int)PyLong_AsLong(o)
#define MyText_InternFromString(s) \
                            PyUnicode_InternFromString(s)

#define MyType_HEAD_INIT    PyVarObject_HEAD_INIT(NULL, 0)

#else

#define MyText_Type         PyString_Type
#define MyText_AS_BYTES(o)  (Py_INCREF(o), o)
#define MyBytes_GET_SIZE(o) PyString_GET_SIZE(o)
#define MyBytes_AS_STRING(o) PyString_AS_STRING(o)
#define MyText_AsString(o)  PyString_AsString(o)
#define MyText_FromFormat   PyUnicode_FromFormat
#define MyInt_FromInt(i)    PyInt_FromLong((long)i)
#define MyInt_AsInt(o)      (int)PyInt_AsLong(o)
#define MyText_InternFromString(s) \
                            PyString_InternFromString(s)

#define MyType_HEAD_INIT    PyObject_HEAD_INIT(NULL)  0,

#endif /* Py3k */

/* The values returned to indicate ok or error. */
#define RET_OK      0
#define RET_ERROR   -1

#endif /* _COVERAGE_UTIL_H */
