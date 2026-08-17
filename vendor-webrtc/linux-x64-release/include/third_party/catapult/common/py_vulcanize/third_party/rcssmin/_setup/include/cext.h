/*
 * Copyright 2006 - 2014
 * Andr\xe9 Malo or his licensors, as applicable
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 * central naming stuff
 */

#ifndef SETUP_CEXT_H
#define SETUP_CEXT_H

#ifndef EXT_MODULE
#error EXT_MODULE must be defined outside of this file (-DEXT_MODULE=...)
#endif

/*
 * include core header files
 */
#define PY_SSIZE_T_CLEAN

#include "Python.h"
#include "structmember.h"

/*
 * define our helper macros depending on the stuff above
 */
#define STRINGIFY(n) STRINGIFY_HELPER(n)
#define STRINGIFY_HELPER(n) #n
#define CONCATENATE(first, second) CONCATENATE_HELPER(first, second)
#define CONCATENATE_HELPER(first, second) first##second

#define EXT_MODULE_NAME  STRINGIFY(EXT_MODULE)
#ifdef EXT_PACKAGE
#define EXT_PACKAGE_NAME STRINGIFY(EXT_PACKAGE)
#define EXT_MODULE_PATH  EXT_PACKAGE_NAME "." EXT_MODULE_NAME
#else
#define EXT_PACKAGE_NAME ""
#define EXT_MODULE_PATH  EXT_MODULE_NAME
#endif

#define EXT_DOCS_VAR      CONCATENATE(var, CONCATENATE(EXT_MODULE, __doc__))
#define EXT_METHODS_VAR   CONCATENATE(var, CONCATENATE(EXT_MODULE, _methods))
#define EXT_METHODS       static PyMethodDef EXT_METHODS_VAR[]

#define EXT_DEFINE_VAR    CONCATENATE(var, CONCATENATE(EXT_MODULE, _module))

/* Py3K Support */
#if PY_MAJOR_VERSION >= 3

#define EXT3

#ifndef PyMODINIT_FUNC
#define EXT_INIT_FUNC PyObject *CONCATENATE(PyInit_, EXT_MODULE)(void)
#else
#define EXT_INIT_FUNC PyMODINIT_FUNC CONCATENATE(PyInit_, EXT_MODULE)(void)
#endif

#define EXT_DEFINE(name, methods, doc) \
static struct PyModuleDef EXT_DEFINE_VAR = { \
    PyModuleDef_HEAD_INIT, \
    name,                  \
    doc,                   \
    -1,                    \
    methods,               \
    NULL,                  \
    NULL,                  \
    NULL,                  \
    NULL                   \
}

#define EXT_CREATE(def) (PyModule_Create(def))
#define EXT_INIT_ERROR(module) do {Py_XDECREF(module); return NULL;} while(0)
#define EXT_INIT_RETURN(module) return module

#else /* end py3k */

#define EXT2

#ifndef PyMODINIT_FUNC
#define EXT_INIT_FUNC void CONCATENATE(init, EXT_MODULE)(void)
#else
#define EXT_INIT_FUNC PyMODINIT_FUNC CONCATENATE(init, EXT_MODULE)(void)
#endif

#define EXT_DEFINE__STRUCT \
    CONCATENATE(struct, CONCATENATE(EXT_MODULE, _module))

struct EXT_DEFINE__STRUCT {
    char *m_name;
    char *m_doc;
    PyMethodDef *m_methods;
};
#define EXT_DEFINE(name, methods, doc)              \
static struct EXT_DEFINE__STRUCT EXT_DEFINE_VAR = { \
    name,                                           \
    doc,                                            \
    methods                                         \
}

#define EXT_CREATE(def) ((def)->m_doc                               \
    ? Py_InitModule3((def)->m_name, (def)->m_methods, (def)->m_doc) \
    : Py_InitModule((def)->m_name, (def)->m_methods)                \
)
#define EXT_INIT_ERROR(module) return
#define EXT_INIT_RETURN(module) return

#endif /* end py2K */

#define EXT_INIT_TYPE(module, type) do { \
    if (PyType_Ready(type) < 0)          \
        EXT_INIT_ERROR(module);          \
} while (0)

#define EXT_ADD_TYPE(module, name, type) do {                     \
    Py_INCREF(type);                                              \
    if (PyModule_AddObject(module, name, (PyObject *)(type)) < 0) \
        EXT_INIT_ERROR(module);                                   \
} while (0)

#define EXT_ADD_UNICODE(module, name, string, encoding) do { \
    if (PyModule_AddObject(                                  \
            module,                                          \
            name,                                            \
            PyUnicode_Decode(                                \
                string,                                      \
                sizeof(string) - 1,                          \
                encoding,                                    \
                "strict"                                     \
            )) < 0)                                          \
        EXT_INIT_ERROR(module);                              \
} while (0)

#define EXT_ADD_STRING(module, name, string) do {             \
    if (PyModule_AddStringConstant(module, name, string) < 0) \
        EXT_INIT_ERROR(module);                               \
} while (0)

#define EXT_ADD_INT(module, name, number) do {             \
    if (PyModule_AddIntConstant(module, name, number) < 0) \
        EXT_INIT_ERROR(module);                            \
} while (0)


/* PEP 353 support, implemented as of python 2.5 */
#if PY_VERSION_HEX < 0x02050000
typedef int Py_ssize_t;
#define PyInt_FromSsize_t(arg) PyInt_FromLong((long)arg)
#define PyInt_AsSsize_t(arg) (int)PyInt_AsLong(arg)
#define PY_SSIZE_T_MAX ((Py_ssize_t)INT_MAX)
#endif

/*
 * some helper macros (Python 2.4)
 */
#ifndef Py_VISIT
#define Py_VISIT(op) do {            \
    if (op) {                        \
        int vret = visit((op), arg); \
        if (vret) return vret;       \
    }                                \
} while (0)
#endif

#ifdef Py_CLEAR
#undef Py_CLEAR
#endif
#define Py_CLEAR(op) do {                   \
    if (op) {                               \
        PyObject *tmp__ = (PyObject *)(op); \
        (op) = NULL;                        \
        Py_DECREF(tmp__);                   \
    }                                       \
} while (0)

#ifndef Py_RETURN_NONE
#define Py_RETURN_NONE return Py_INCREF(Py_None), Py_None
#endif

#ifndef Py_RETURN_FALSE
#define Py_RETURN_FALSE return Py_INCREF(Py_False), Py_False
#endif

#ifndef Py_RETURN_TRUE
#define Py_RETURN_TRUE return Py_INCREF(Py_True), Py_True
#endif

/* Macros for inline documentation. (Python 2.3) */
#ifndef PyDoc_VAR
#define PyDoc_VAR(name) static char name[]
#endif

#ifndef PyDoc_STRVAR
#define PyDoc_STRVAR(name,str) PyDoc_VAR(name) = PyDoc_STR(str)
#endif

#ifndef PyDoc_STR
#ifdef WITH_DOC_STRINGS
#define PyDoc_STR(str) str
#else
#define PyDoc_STR(str) ""
#endif
#endif

/* Basestring check (basestring introduced in Python 2.3) */
#if PY_VERSION_HEX < 0x02030000
#define BaseString_Check(type) (                  \
       PyObject_TypeCheck((type), &PyString_Type)  \
    || PyObject_TypeCheck((type), &PyUnicode_Type) \
)
#else
#define BaseString_Check(type) PyObject_TypeCheck((type), &PyBaseString_Type)
#endif

#define GENERIC_ALLOC(type) \
    ((void *)((PyTypeObject *)type)->tp_alloc(type, (Py_ssize_t)0))

/* PyPy doesn't define it */
#ifndef PyType_IS_GC
#define PyType_IS_GC(t) PyType_HasFeature((t), Py_TPFLAGS_HAVE_GC)
#endif

#define DEFINE_GENERIC_DEALLOC(prefix)                      \
static void prefix##_dealloc(void *self)                    \
{                                                           \
    if (PyType_IS_GC(((PyObject *)self)->ob_type))          \
        PyObject_GC_UnTrack(self);                          \
    (void)prefix##_clear(self);                             \
    ((PyObject *)self)->ob_type->tp_free((PyObject *)self); \
}

#endif /* SETUP_CEXT_H */
