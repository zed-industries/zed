/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 * Python extensions by Paul Moore.
 * Changes for Unix by David Leonard.
 *
 * This consists of four parts:
 * 1. Python interpreter main program
 * 2. Python output stream: writes output via [e]msg().
 * 3. Implementation of the Vim module for Python
 * 4. Utility functions for handling the interface between Vim and Python.
 */

#include "vim.h"

#include <limits.h>

// uncomment this if used with the debug version of python.
// Checked on 2.7.4.
// #define Py_DEBUG
// Note: most of time you can add -DPy_DEBUG to CFLAGS in place of uncommenting
// uncomment this if used with the debug version of python, but without its
// allocator
// #define Py_DEBUG_NO_PYMALLOC

// Python.h defines _POSIX_THREADS itself (if needed)
#ifdef _POSIX_THREADS
# undef _POSIX_THREADS
#endif

#if defined(MSWIN) && defined(HAVE_FCNTL_H)
# undef HAVE_FCNTL_H
#endif

#ifdef _DEBUG
# undef _DEBUG
#endif

#ifdef HAVE_STRFTIME
# undef HAVE_STRFTIME
#endif
#ifdef HAVE_STRING_H
# undef HAVE_STRING_H
#endif
#ifdef HAVE_PUTENV
# undef HAVE_PUTENV
#endif
#ifdef HAVE_STDARG_H
# undef HAVE_STDARG_H	// Python's config.h defines it as well.
#endif
#ifdef _POSIX_C_SOURCE
# undef _POSIX_C_SOURCE	// pyconfig.h defines it as well.
#endif
#ifdef _XOPEN_SOURCE
# undef _XOPEN_SOURCE	// pyconfig.h defines it as well.
#endif

#define PY_SSIZE_T_CLEAN

#include <Python.h>

#if !defined(PY_VERSION_HEX) || PY_VERSION_HEX < 0x02050000
# undef PY_SSIZE_T_CLEAN
#endif

// these are NULL for Python 2
#define ERRORS_DECODE_ARG NULL
#define ERRORS_ENCODE_ARG ERRORS_DECODE_ARG

#undef main // Defined in python.h - aargh
#undef HAVE_FCNTL_H // Clash with os_win32.h

// Perhaps leave this out for Python 2.6, which supports bytes?
#define PyBytes_FromString      PyString_FromString
#define PyBytes_Check		PyString_Check
#define PyBytes_AsStringAndSize PyString_AsStringAndSize
#define PyBytes_FromStringAndSize   PyString_FromStringAndSize

#if !defined(FEAT_PYTHON) && defined(PROTO)
// Use this to be able to generate prototypes without python being used.
# define PyObject Py_ssize_t
# define PyThreadState Py_ssize_t
# define PyTypeObject Py_ssize_t
struct PyMethodDef { Py_ssize_t a; };
# define PySequenceMethods Py_ssize_t
#endif

#if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
# define PY_USE_CAPSULE
#endif

#if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02050000
# define PyInt Py_ssize_t
# define PyInquiry lenfunc
# define PyIntArgFunc ssizeargfunc
# define PyIntIntArgFunc ssizessizeargfunc
# define PyIntObjArgProc ssizeobjargproc
# define PyIntIntObjArgProc ssizessizeobjargproc
# define Py_ssize_t_fmt "n"
#else
# define PyInt int
# define lenfunc inquiry
# define PyInquiry inquiry
# define PyIntArgFunc intargfunc
# define PyIntIntArgFunc intintargfunc
# define PyIntObjArgProc intobjargproc
# define PyIntIntObjArgProc intintobjargproc
# define Py_ssize_t_fmt "i"
#endif
#define Py_bytes_fmt "s"

// Parser flags
#define single_input	256
#define file_input	257
#define eval_input	258

#if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x020300F0
  // Python 2.3: can invoke ":python" recursively.
# define PY_CAN_RECURSE
#endif

#if defined(DYNAMIC_PYTHON) || defined(PROTO)
# ifndef DYNAMIC_PYTHON
#  define HINSTANCE long_u		// for generating prototypes
# endif

# ifdef MSWIN
#  define load_dll vimLoadLib
#  define close_dll FreeLibrary
#  define symbol_from_dll GetProcAddress
#  define load_dll_error GetWin32Error
# else
#  include <dlfcn.h>
#  define FARPROC void*
#  define HINSTANCE void*
#  if defined(PY_NO_RTLD_GLOBAL) && defined(PY3_NO_RTLD_GLOBAL)
#   define load_dll(n) dlopen((n), RTLD_LAZY)
#  else
#   define load_dll(n) dlopen((n), RTLD_LAZY|RTLD_GLOBAL)
#  endif
#  define close_dll dlclose
#  define symbol_from_dll dlsym
#  define load_dll_error dlerror
# endif

// This makes if_python.c compile without warnings against Python 2.5
// on Win32 and Win64.
# undef PyRun_SimpleString
# undef PyRun_String
# undef PyArg_Parse
# undef PyArg_ParseTuple
# undef Py_BuildValue
# undef Py_InitModule4
# undef Py_InitModule4_64
# undef PyObject_CallMethod
# undef PyObject_CallFunction

/*
 * Wrapper defines
 */
# define PyArg_Parse dll_PyArg_Parse
# define PyArg_ParseTuple dll_PyArg_ParseTuple
# define PyMem_Free dll_PyMem_Free
# define PyMem_Malloc dll_PyMem_Malloc
# define PyDict_SetItemString dll_PyDict_SetItemString
# define PyErr_BadArgument dll_PyErr_BadArgument
# define PyErr_NewException dll_PyErr_NewException
# define PyErr_Clear dll_PyErr_Clear
# define PyErr_Format dll_PyErr_Format
# define PyErr_PrintEx dll_PyErr_PrintEx
# define PyErr_NoMemory dll_PyErr_NoMemory
# define PyErr_Occurred dll_PyErr_Occurred
# define PyErr_SetNone dll_PyErr_SetNone
# define PyErr_SetString dll_PyErr_SetString
# define PyErr_SetObject dll_PyErr_SetObject
# define PyErr_ExceptionMatches dll_PyErr_ExceptionMatches
# define PyEval_InitThreads dll_PyEval_InitThreads
# define PyEval_RestoreThread dll_PyEval_RestoreThread
# define PyEval_SaveThread dll_PyEval_SaveThread
# ifdef PY_CAN_RECURSE
#  define PyGILState_Ensure dll_PyGILState_Ensure
#  define PyGILState_Release dll_PyGILState_Release
# endif
# define PyInt_AsLong dll_PyInt_AsLong
# define PyInt_FromLong dll_PyInt_FromLong
# define PyLong_AsLong dll_PyLong_AsLong
# define PyLong_FromLong dll_PyLong_FromLong
# define PyBool_Type (*dll_PyBool_Type)
# define PyInt_Type (*dll_PyInt_Type)
# define PyLong_Type (*dll_PyLong_Type)
# define PyList_GetItem dll_PyList_GetItem
# define PyList_Append dll_PyList_Append
# define PyList_Insert dll_PyList_Insert
# define PyList_New dll_PyList_New
# define PyList_SetItem dll_PyList_SetItem
# define PyList_Size dll_PyList_Size
# define PyList_Type (*dll_PyList_Type)
# define PySequence_Check dll_PySequence_Check
# define PySequence_Size dll_PySequence_Size
# define PySequence_GetItem dll_PySequence_GetItem
# define PySequence_Fast dll_PySequence_Fast
# define PyTuple_Size dll_PyTuple_Size
# define PyTuple_GetItem dll_PyTuple_GetItem
# define PyTuple_Type (*dll_PyTuple_Type)
# define PySlice_GetIndicesEx dll_PySlice_GetIndicesEx
# define PyImport_ImportModule dll_PyImport_ImportModule
# define PyDict_New dll_PyDict_New
# define PyDict_GetItemString dll_PyDict_GetItemString
# define PyDict_Next dll_PyDict_Next
# define PyDict_Type (*dll_PyDict_Type)
# ifdef PyMapping_Keys
#  define PY_NO_MAPPING_KEYS
# else
#  define PyMapping_Keys dll_PyMapping_Keys
# endif
# define PyObject_GetItem dll_PyObject_GetItem
# define PyObject_CallMethod dll_PyObject_CallMethod
# define PyMapping_Check dll_PyMapping_Check
# define PyIter_Next dll_PyIter_Next
# define PyModule_GetDict dll_PyModule_GetDict
# define PyModule_AddObject dll_PyModule_AddObject
# define PyRun_SimpleString dll_PyRun_SimpleString
# define PyRun_String dll_PyRun_String
# define PyObject_GetAttrString dll_PyObject_GetAttrString
# define PyObject_HasAttrString dll_PyObject_HasAttrString
# define PyObject_SetAttrString dll_PyObject_SetAttrString
# define PyObject_CallFunctionObjArgs dll_PyObject_CallFunctionObjArgs
# define PyObject_CallFunction dll_PyObject_CallFunction
# define PyObject_Call dll_PyObject_Call
# define PyObject_Repr dll_PyObject_Repr
# define PyString_AsString dll_PyString_AsString
# define PyString_AsStringAndSize dll_PyString_AsStringAndSize
# define PyString_FromString dll_PyString_FromString
# define PyString_FromFormat dll_PyString_FromFormat
# define PyString_FromStringAndSize dll_PyString_FromStringAndSize
# define PyString_Size dll_PyString_Size
# define PyString_Type (*dll_PyString_Type)
# define PyUnicode_Type (*dll_PyUnicode_Type)
# undef PyUnicode_AsEncodedString
# define PyUnicode_AsEncodedString py_PyUnicode_AsEncodedString
# define PyFloat_AsDouble dll_PyFloat_AsDouble
# define PyFloat_FromDouble dll_PyFloat_FromDouble
# define PyFloat_Type (*dll_PyFloat_Type)
# define PyNumber_Check dll_PyNumber_Check
# define PyNumber_Long dll_PyNumber_Long
# define PyImport_AddModule (*dll_PyImport_AddModule)
# define PySys_SetObject dll_PySys_SetObject
# define PySys_GetObject dll_PySys_GetObject
# define PySys_SetArgv dll_PySys_SetArgv
# define PyType_Type (*dll_PyType_Type)
# define PyFile_Type (*dll_PyFile_Type)
# define PySlice_Type (*dll_PySlice_Type)
# define PyType_Ready (*dll_PyType_Ready)
# define PyType_GenericAlloc dll_PyType_GenericAlloc
# define Py_BuildValue dll_Py_BuildValue
# define Py_FindMethod dll_Py_FindMethod
# define Py_InitModule4 dll_Py_InitModule4
# define Py_SetPythonHome dll_Py_SetPythonHome
# define Py_Initialize dll_Py_Initialize
# define Py_Finalize dll_Py_Finalize
# define Py_IsInitialized dll_Py_IsInitialized
# define _PyObject_New dll__PyObject_New
# define _PyObject_GC_New dll__PyObject_GC_New
# ifdef PyObject_GC_Del
#  define Py_underscore_GC
#  define _PyObject_GC_Del dll__PyObject_GC_Del
#  define _PyObject_GC_UnTrack dll__PyObject_GC_UnTrack
# else
#  define PyObject_GC_Del dll_PyObject_GC_Del
#  define PyObject_GC_UnTrack dll_PyObject_GC_UnTrack
# endif
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
#  define _PyObject_NextNotImplemented (*dll__PyObject_NextNotImplemented)
# endif
# define _Py_NoneStruct (*dll__Py_NoneStruct)
# define _Py_ZeroStruct (*dll__Py_ZeroStruct)
# define _Py_TrueStruct (*dll__Py_TrueStruct)
# define PyObject_Init dll__PyObject_Init
# define PyObject_GetIter dll_PyObject_GetIter
# define PyObject_IsTrue dll_PyObject_IsTrue
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02020000
#  define PyType_IsSubtype dll_PyType_IsSubtype
#  ifdef Py_DEBUG
#   define _Py_NegativeRefcount dll__Py_NegativeRefcount
#   define _Py_RefTotal (*dll__Py_RefTotal)
#   define _Py_Dealloc dll__Py_Dealloc
#  endif
# endif
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02030000
#  if defined(Py_DEBUG) && !defined(Py_DEBUG_NO_PYMALLOC)
#   define _PyObject_DebugMalloc dll__PyObject_DebugMalloc
#   define _PyObject_DebugFree dll__PyObject_DebugFree
#  else
#   define PyObject_Malloc dll_PyObject_Malloc
#   define PyObject_Free dll_PyObject_Free
#  endif
# endif
# ifdef PY_USE_CAPSULE
#  define PyCapsule_New dll_PyCapsule_New
#  define PyCapsule_GetPointer dll_PyCapsule_GetPointer
# else
#  define PyCObject_FromVoidPtr dll_PyCObject_FromVoidPtr
#  define PyCObject_AsVoidPtr dll_PyCObject_AsVoidPtr
# endif
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
#  define Py_NoSiteFlag (*dll_Py_NoSiteFlag)
# endif

/*
 * Pointers for dynamic link
 */
static int(*dll_PyArg_Parse)(PyObject *, char *, ...);
static int(*dll_PyArg_ParseTuple)(PyObject *, char *, ...);
static void(*dll_PyMem_Free)(void *);
static void* (*dll_PyMem_Malloc)(size_t);
static int(*dll_PyDict_SetItemString)(PyObject *dp, char *key, PyObject *item);
static int(*dll_PyErr_BadArgument)(void);
static PyObject *(*dll_PyErr_NewException)(char *, PyObject *, PyObject *);
static void(*dll_PyErr_Clear)(void);
static PyObject*(*dll_PyErr_Format)(PyObject *, const char *, ...);
static void(*dll_PyErr_PrintEx)(int);
static PyObject*(*dll_PyErr_NoMemory)(void);
static PyObject*(*dll_PyErr_Occurred)(void);
static void(*dll_PyErr_SetNone)(PyObject *);
static void(*dll_PyErr_SetString)(PyObject *, const char *);
static void(*dll_PyErr_SetObject)(PyObject *, PyObject *);
static int(*dll_PyErr_ExceptionMatches)(PyObject *);
static void(*dll_PyEval_InitThreads)(void);
static void(*dll_PyEval_RestoreThread)(PyThreadState *);
static PyThreadState*(*dll_PyEval_SaveThread)(void);
# ifdef PY_CAN_RECURSE
static PyGILState_STATE	(*dll_PyGILState_Ensure)(void);
static void (*dll_PyGILState_Release)(PyGILState_STATE);
# endif
static long(*dll_PyInt_AsLong)(PyObject *);
static PyObject*(*dll_PyInt_FromLong)(long);
static long(*dll_PyLong_AsLong)(PyObject *);
static PyObject*(*dll_PyLong_FromLong)(long);
static PyTypeObject* dll_PyBool_Type;
static PyTypeObject* dll_PyInt_Type;
static PyTypeObject* dll_PyLong_Type;
static PyObject*(*dll_PyList_GetItem)(PyObject *, PyInt);
static int(*dll_PyList_Append)(PyObject *, PyObject *);
static int(*dll_PyList_Insert)(PyObject *, PyInt, PyObject *);
static PyObject*(*dll_PyList_New)(PyInt size);
static int(*dll_PyList_SetItem)(PyObject *, PyInt, PyObject *);
static PyInt(*dll_PyList_Size)(PyObject *);
static PyTypeObject* dll_PyList_Type;
static int (*dll_PySequence_Check)(PyObject *);
static PyInt(*dll_PySequence_Size)(PyObject *);
static PyObject*(*dll_PySequence_GetItem)(PyObject *, PyInt);
static PyObject*(*dll_PySequence_Fast)(PyObject *, const char *);
static PyInt(*dll_PyTuple_Size)(PyObject *);
static PyObject*(*dll_PyTuple_GetItem)(PyObject *, PyInt);
static PyTypeObject* dll_PyTuple_Type;
static int (*dll_PySlice_GetIndicesEx)(PySliceObject *r, PyInt length,
		     PyInt *start, PyInt *stop, PyInt *step,
		     PyInt *slicelen);
static PyObject*(*dll_PyImport_ImportModule)(const char *);
static PyObject*(*dll_PyDict_New)(void);
static PyObject*(*dll_PyDict_GetItemString)(PyObject *, const char *);
static int (*dll_PyDict_Next)(PyObject *, PyInt *, PyObject **, PyObject **);
static PyTypeObject* dll_PyDict_Type;
# ifndef PY_NO_MAPPING_KEYS
static PyObject* (*dll_PyMapping_Keys)(PyObject *);
# endif
static PyObject* (*dll_PyObject_GetItem)(PyObject *, PyObject *);
static PyObject* (*dll_PyObject_CallMethod)(PyObject *, char *, PyObject *);
static int (*dll_PyMapping_Check)(PyObject *);
static PyObject* (*dll_PyIter_Next)(PyObject *);
static PyObject*(*dll_PyModule_GetDict)(PyObject *);
static int(*dll_PyModule_AddObject)(PyObject *, const char *, PyObject *);
static int(*dll_PyRun_SimpleString)(char *);
static PyObject *(*dll_PyRun_String)(char *, int, PyObject *, PyObject *);
static PyObject* (*dll_PyObject_GetAttrString)(PyObject *, const char *);
static int (*dll_PyObject_HasAttrString)(PyObject *, const char *);
static int (*dll_PyObject_SetAttrString)(PyObject *, const char *, PyObject *);
static PyObject* (*dll_PyObject_CallFunctionObjArgs)(PyObject *, ...);
static PyObject* (*dll_PyObject_CallFunction)(PyObject *, char *, ...);
static PyObject* (*dll_PyObject_Call)(PyObject *, PyObject *, PyObject *);
static PyObject* (*dll_PyObject_Repr)(PyObject *);
static char*(*dll_PyString_AsString)(PyObject *);
static int(*dll_PyString_AsStringAndSize)(PyObject *, char **, PyInt *);
static PyObject*(*dll_PyString_FromString)(const char *);
static PyObject*(*dll_PyString_FromFormat)(const char *, ...);
static PyObject*(*dll_PyString_FromStringAndSize)(const char *, PyInt);
static PyInt(*dll_PyString_Size)(PyObject *);
static PyTypeObject* dll_PyString_Type;
static PyTypeObject* dll_PyUnicode_Type;
static PyObject *(*py_PyUnicode_AsEncodedString)(PyObject *, char *, char *);
static double(*dll_PyFloat_AsDouble)(PyObject *);
static PyObject*(*dll_PyFloat_FromDouble)(double);
static PyTypeObject* dll_PyFloat_Type;
static int(*dll_PyNumber_Check)(PyObject *);
static PyObject*(*dll_PyNumber_Long)(PyObject *);
static int(*dll_PySys_SetObject)(char *, PyObject *);
static PyObject *(*dll_PySys_GetObject)(char *);
static int(*dll_PySys_SetArgv)(int, char **);
static PyTypeObject* dll_PyType_Type;
static PyTypeObject* dll_PyFile_Type;
static PyTypeObject* dll_PySlice_Type;
static int (*dll_PyType_Ready)(PyTypeObject *type);
static PyObject* (*dll_PyType_GenericAlloc)(PyTypeObject *type, PyInt nitems);
static PyObject*(*dll_Py_BuildValue)(char *, ...);
static PyObject*(*dll_Py_FindMethod)(struct PyMethodDef[], PyObject *, char *);
static PyObject*(*dll_Py_InitModule4)(char *, struct PyMethodDef *, char *, PyObject *, int);
static PyObject*(*dll_PyImport_AddModule)(char *);
static void(*dll_Py_SetPythonHome)(char *home);
static void(*dll_Py_Initialize)(void);
static void(*dll_Py_Finalize)(void);
static int(*dll_Py_IsInitialized)(void);
static PyObject*(*dll__PyObject_New)(PyTypeObject *, PyObject *);
static PyObject*(*dll__PyObject_GC_New)(PyTypeObject *);
# ifdef Py_underscore_GC
static void(*dll__PyObject_GC_Del)(void *);
static void(*dll__PyObject_GC_UnTrack)(void *);
# else
static void(*dll_PyObject_GC_Del)(void *);
static void(*dll_PyObject_GC_UnTrack)(void *);
# endif
static PyObject*(*dll__PyObject_Init)(PyObject *, PyTypeObject *);
static PyObject* (*dll_PyObject_GetIter)(PyObject *);
static int (*dll_PyObject_IsTrue)(PyObject *);
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
static iternextfunc dll__PyObject_NextNotImplemented;
# endif
static PyObject* dll__Py_NoneStruct;
static PyObject* _Py_ZeroStruct;
static PyObject* dll__Py_TrueStruct;
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02020000
static int (*dll_PyType_IsSubtype)(PyTypeObject *, PyTypeObject *);
#  ifdef Py_DEBUG
static void (*dll__Py_NegativeRefcount)(const char *fname, int lineno, PyObject *op);
static PyInt* dll__Py_RefTotal;
static void (*dll__Py_Dealloc)(PyObject *obj);
#  endif
# endif
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02030000
#  if defined(Py_DEBUG) && !defined(Py_DEBUG_NO_PYMALLOC)
static void (*dll__PyObject_DebugFree)(void*);
static void* (*dll__PyObject_DebugMalloc)(size_t);
#  else
static void* (*dll_PyObject_Malloc)(size_t);
static void (*dll_PyObject_Free)(void*);
#  endif
# endif
# ifdef PY_USE_CAPSULE
static PyObject* (*dll_PyCapsule_New)(void *, char *, PyCapsule_Destructor);
static void* (*dll_PyCapsule_GetPointer)(PyObject *, char *);
# else
static PyObject* (*dll_PyCObject_FromVoidPtr)(void *cobj, void (*destr)(void *));
static void* (*dll_PyCObject_AsVoidPtr)(PyObject *);
# endif
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
static int* dll_Py_NoSiteFlag;
# endif

static HINSTANCE hinstPython = 0; // Instance of python.dll

// Imported exception objects
static PyObject *imp_PyExc_AttributeError;
static PyObject *imp_PyExc_IndexError;
static PyObject *imp_PyExc_KeyError;
static PyObject *imp_PyExc_KeyboardInterrupt;
static PyObject *imp_PyExc_TypeError;
static PyObject *imp_PyExc_ValueError;
static PyObject *imp_PyExc_SystemExit;
static PyObject *imp_PyExc_RuntimeError;
static PyObject *imp_PyExc_ImportError;
static PyObject *imp_PyExc_OverflowError;

# define PyExc_AttributeError imp_PyExc_AttributeError
# define PyExc_IndexError imp_PyExc_IndexError
# define PyExc_KeyError imp_PyExc_KeyError
# define PyExc_KeyboardInterrupt imp_PyExc_KeyboardInterrupt
# define PyExc_TypeError imp_PyExc_TypeError
# define PyExc_ValueError imp_PyExc_ValueError
# define PyExc_SystemExit imp_PyExc_SystemExit
# define PyExc_RuntimeError imp_PyExc_RuntimeError
# define PyExc_ImportError imp_PyExc_ImportError
# define PyExc_OverflowError imp_PyExc_OverflowError

/*
 * Table of name to function pointer of python.
 */
# define PYTHON_PROC FARPROC
static struct
{
    char *name;
    PYTHON_PROC *ptr;
} python_funcname_table[] =
{
# ifdef PY_SSIZE_T_CLEAN
    {"_PyArg_Parse_SizeT", (PYTHON_PROC*)&dll_PyArg_Parse},
    {"_PyArg_ParseTuple_SizeT", (PYTHON_PROC*)&dll_PyArg_ParseTuple},
    {"_Py_BuildValue_SizeT", (PYTHON_PROC*)&dll_Py_BuildValue},
# else
    {"PyArg_Parse", (PYTHON_PROC*)&dll_PyArg_Parse},
    {"PyArg_ParseTuple", (PYTHON_PROC*)&dll_PyArg_ParseTuple},
    {"Py_BuildValue", (PYTHON_PROC*)&dll_Py_BuildValue},
# endif
    {"PyMem_Free", (PYTHON_PROC*)&dll_PyMem_Free},
    {"PyMem_Malloc", (PYTHON_PROC*)&dll_PyMem_Malloc},
    {"PyDict_SetItemString", (PYTHON_PROC*)&dll_PyDict_SetItemString},
    {"PyErr_BadArgument", (PYTHON_PROC*)&dll_PyErr_BadArgument},
    {"PyErr_NewException", (PYTHON_PROC*)&dll_PyErr_NewException},
    {"PyErr_Clear", (PYTHON_PROC*)&dll_PyErr_Clear},
    {"PyErr_Format", (PYTHON_PROC*)&dll_PyErr_Format},
    {"PyErr_PrintEx", (PYTHON_PROC*)&dll_PyErr_PrintEx},
    {"PyErr_NoMemory", (PYTHON_PROC*)&dll_PyErr_NoMemory},
    {"PyErr_Occurred", (PYTHON_PROC*)&dll_PyErr_Occurred},
    {"PyErr_SetNone", (PYTHON_PROC*)&dll_PyErr_SetNone},
    {"PyErr_SetString", (PYTHON_PROC*)&dll_PyErr_SetString},
    {"PyErr_SetObject", (PYTHON_PROC*)&dll_PyErr_SetObject},
    {"PyErr_ExceptionMatches", (PYTHON_PROC*)&dll_PyErr_ExceptionMatches},
    {"PyEval_InitThreads", (PYTHON_PROC*)&dll_PyEval_InitThreads},
    {"PyEval_RestoreThread", (PYTHON_PROC*)&dll_PyEval_RestoreThread},
    {"PyEval_SaveThread", (PYTHON_PROC*)&dll_PyEval_SaveThread},
# ifdef PY_CAN_RECURSE
    {"PyGILState_Ensure", (PYTHON_PROC*)&dll_PyGILState_Ensure},
    {"PyGILState_Release", (PYTHON_PROC*)&dll_PyGILState_Release},
# endif
    {"PyInt_AsLong", (PYTHON_PROC*)&dll_PyInt_AsLong},
    {"PyInt_FromLong", (PYTHON_PROC*)&dll_PyInt_FromLong},
    {"PyLong_AsLong", (PYTHON_PROC*)&dll_PyLong_AsLong},
    {"PyLong_FromLong", (PYTHON_PROC*)&dll_PyLong_FromLong},
    {"PyBool_Type", (PYTHON_PROC*)&dll_PyBool_Type},
    {"PyInt_Type", (PYTHON_PROC*)&dll_PyInt_Type},
    {"PyLong_Type", (PYTHON_PROC*)&dll_PyLong_Type},
    {"PyList_GetItem", (PYTHON_PROC*)&dll_PyList_GetItem},
    {"PyList_Append", (PYTHON_PROC*)&dll_PyList_Append},
    {"PyList_Insert", (PYTHON_PROC*)&dll_PyList_Insert},
    {"PyList_New", (PYTHON_PROC*)&dll_PyList_New},
    {"PyList_SetItem", (PYTHON_PROC*)&dll_PyList_SetItem},
    {"PyList_Size", (PYTHON_PROC*)&dll_PyList_Size},
    {"PyList_Type", (PYTHON_PROC*)&dll_PyList_Type},
    {"PySequence_Size", (PYTHON_PROC*)&dll_PySequence_Size},
    {"PySequence_Check", (PYTHON_PROC*)&dll_PySequence_Check},
    {"PySequence_GetItem", (PYTHON_PROC*)&dll_PySequence_GetItem},
    {"PySequence_Fast", (PYTHON_PROC*)&dll_PySequence_Fast},
    {"PyTuple_GetItem", (PYTHON_PROC*)&dll_PyTuple_GetItem},
    {"PyTuple_Size", (PYTHON_PROC*)&dll_PyTuple_Size},
    {"PyTuple_Type", (PYTHON_PROC*)&dll_PyTuple_Type},
    {"PySlice_GetIndicesEx", (PYTHON_PROC*)&dll_PySlice_GetIndicesEx},
    {"PyImport_ImportModule", (PYTHON_PROC*)&dll_PyImport_ImportModule},
    {"PyDict_GetItemString", (PYTHON_PROC*)&dll_PyDict_GetItemString},
    {"PyDict_Next", (PYTHON_PROC*)&dll_PyDict_Next},
    {"PyDict_New", (PYTHON_PROC*)&dll_PyDict_New},
    {"PyDict_Type", (PYTHON_PROC*)&dll_PyDict_Type},
# ifndef PY_NO_MAPPING_KEYS
    {"PyMapping_Keys", (PYTHON_PROC*)&dll_PyMapping_Keys},
# endif
    {"PyObject_GetItem", (PYTHON_PROC*)&dll_PyObject_GetItem},
    {"PyObject_CallMethod", (PYTHON_PROC*)&dll_PyObject_CallMethod},
    {"PyMapping_Check", (PYTHON_PROC*)&dll_PyMapping_Check},
    {"PyIter_Next", (PYTHON_PROC*)&dll_PyIter_Next},
    {"PyModule_GetDict", (PYTHON_PROC*)&dll_PyModule_GetDict},
    {"PyModule_AddObject", (PYTHON_PROC*)&dll_PyModule_AddObject},
    {"PyRun_SimpleString", (PYTHON_PROC*)&dll_PyRun_SimpleString},
    {"PyRun_String", (PYTHON_PROC*)&dll_PyRun_String},
    {"PyObject_GetAttrString", (PYTHON_PROC*)&dll_PyObject_GetAttrString},
    {"PyObject_HasAttrString", (PYTHON_PROC*)&dll_PyObject_HasAttrString},
    {"PyObject_SetAttrString", (PYTHON_PROC*)&dll_PyObject_SetAttrString},
    {"PyObject_CallFunctionObjArgs", (PYTHON_PROC*)&dll_PyObject_CallFunctionObjArgs},
    {"PyObject_CallFunction", (PYTHON_PROC*)&dll_PyObject_CallFunction},
    {"PyObject_Call", (PYTHON_PROC*)&dll_PyObject_Call},
    {"PyObject_Repr", (PYTHON_PROC*)&dll_PyObject_Repr},
    {"PyString_AsString", (PYTHON_PROC*)&dll_PyString_AsString},
    {"PyString_AsStringAndSize", (PYTHON_PROC*)&dll_PyString_AsStringAndSize},
    {"PyString_FromString", (PYTHON_PROC*)&dll_PyString_FromString},
    {"PyString_FromFormat", (PYTHON_PROC*)&dll_PyString_FromFormat},
    {"PyString_FromStringAndSize", (PYTHON_PROC*)&dll_PyString_FromStringAndSize},
    {"PyString_Size", (PYTHON_PROC*)&dll_PyString_Size},
    {"PyString_Type", (PYTHON_PROC*)&dll_PyString_Type},
    {"PyUnicode_Type", (PYTHON_PROC*)&dll_PyUnicode_Type},
    {"PyFloat_Type", (PYTHON_PROC*)&dll_PyFloat_Type},
    {"PyFloat_AsDouble", (PYTHON_PROC*)&dll_PyFloat_AsDouble},
    {"PyFloat_FromDouble", (PYTHON_PROC*)&dll_PyFloat_FromDouble},
    {"PyImport_AddModule", (PYTHON_PROC*)&dll_PyImport_AddModule},
    {"PyNumber_Check", (PYTHON_PROC*)&dll_PyNumber_Check},
    {"PyNumber_Long", (PYTHON_PROC*)&dll_PyNumber_Long},
    {"PySys_SetObject", (PYTHON_PROC*)&dll_PySys_SetObject},
    {"PySys_GetObject", (PYTHON_PROC*)&dll_PySys_GetObject},
    {"PySys_SetArgv", (PYTHON_PROC*)&dll_PySys_SetArgv},
    {"PyType_Type", (PYTHON_PROC*)&dll_PyType_Type},
    {"PyFile_Type", (PYTHON_PROC*)&dll_PyFile_Type},
    {"PySlice_Type", (PYTHON_PROC*)&dll_PySlice_Type},
    {"PyType_Ready", (PYTHON_PROC*)&dll_PyType_Ready},
    {"PyType_GenericAlloc", (PYTHON_PROC*)&dll_PyType_GenericAlloc},
    {"Py_FindMethod", (PYTHON_PROC*)&dll_Py_FindMethod},
    {"Py_SetPythonHome", (PYTHON_PROC*)&dll_Py_SetPythonHome},
    {"Py_Initialize", (PYTHON_PROC*)&dll_Py_Initialize},
    {"Py_Finalize", (PYTHON_PROC*)&dll_Py_Finalize},
    {"Py_IsInitialized", (PYTHON_PROC*)&dll_Py_IsInitialized},
    {"_PyObject_New", (PYTHON_PROC*)&dll__PyObject_New},
    {"_PyObject_GC_New", (PYTHON_PROC*)&dll__PyObject_GC_New},
# ifdef Py_underscore_GC
    {"_PyObject_GC_Del", (PYTHON_PROC*)&dll__PyObject_GC_Del},
    {"_PyObject_GC_UnTrack", (PYTHON_PROC*)&dll__PyObject_GC_UnTrack},
# else
    {"PyObject_GC_Del", (PYTHON_PROC*)&dll_PyObject_GC_Del},
    {"PyObject_GC_UnTrack", (PYTHON_PROC*)&dll_PyObject_GC_UnTrack},
# endif
    {"PyObject_Init", (PYTHON_PROC*)&dll__PyObject_Init},
    {"PyObject_GetIter", (PYTHON_PROC*)&dll_PyObject_GetIter},
    {"PyObject_IsTrue", (PYTHON_PROC*)&dll_PyObject_IsTrue},
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
    {"_PyObject_NextNotImplemented", (PYTHON_PROC*)&dll__PyObject_NextNotImplemented},
# endif
    {"_Py_NoneStruct", (PYTHON_PROC*)&dll__Py_NoneStruct},
    {"_Py_ZeroStruct", (PYTHON_PROC*)&dll__Py_ZeroStruct},
    {"_Py_TrueStruct", (PYTHON_PROC*)&dll__Py_TrueStruct},
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02020000
#  ifdef Py_DEBUG
    {"_Py_NegativeRefcount", (PYTHON_PROC*)&dll__Py_NegativeRefcount},
    {"_Py_RefTotal", (PYTHON_PROC*)&dll__Py_RefTotal},
    {"_Py_Dealloc", (PYTHON_PROC*)&dll__Py_Dealloc},
#  endif
    {"PyType_IsSubtype", (PYTHON_PROC*)&dll_PyType_IsSubtype},
# endif
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02030000
#  if defined(Py_DEBUG) && !defined(Py_DEBUG_NO_PYMALLOC)
    {"_PyObject_DebugFree", (PYTHON_PROC*)&dll__PyObject_DebugFree},
    {"_PyObject_DebugMalloc", (PYTHON_PROC*)&dll__PyObject_DebugMalloc},
#  else
    {"PyObject_Malloc", (PYTHON_PROC*)&dll_PyObject_Malloc},
    {"PyObject_Free", (PYTHON_PROC*)&dll_PyObject_Free},
#  endif
# endif
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02050000 \
	&& SIZEOF_SIZE_T != VIM_SIZEOF_INT
#  ifdef Py_DEBUG
    {"Py_InitModule4TraceRefs_64", (PYTHON_PROC*)&dll_Py_InitModule4},
#  else
    {"Py_InitModule4_64", (PYTHON_PROC*)&dll_Py_InitModule4},
#  endif
# else
#  ifdef Py_DEBUG
    {"Py_InitModule4TraceRefs", (PYTHON_PROC*)&dll_Py_InitModule4},
#  else
    {"Py_InitModule4", (PYTHON_PROC*)&dll_Py_InitModule4},
#  endif
# endif
# ifdef PY_USE_CAPSULE
    {"PyCapsule_New", (PYTHON_PROC*)&dll_PyCapsule_New},
    {"PyCapsule_GetPointer", (PYTHON_PROC*)&dll_PyCapsule_GetPointer},
# else
    {"PyCObject_FromVoidPtr", (PYTHON_PROC*)&dll_PyCObject_FromVoidPtr},
    {"PyCObject_AsVoidPtr", (PYTHON_PROC*)&dll_PyCObject_AsVoidPtr},
# endif
# if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
    {"Py_NoSiteFlag", (PYTHON_PROC*)&dll_Py_NoSiteFlag},
# endif
    {"", NULL},
};

/*
 * Load library and get all pointers.
 * Parameter 'libname' provides name of DLL.
 * Return OK or FAIL.
 */
    static int
python_runtime_link_init(char *libname, int verbose)
{
    int i;
    PYTHON_PROC *ucs_as_encoded_string =
				   (PYTHON_PROC*)&py_PyUnicode_AsEncodedString;

# if !(defined(PY_NO_RTLD_GLOBAL) && defined(PY3_NO_RTLD_GLOBAL)) && defined(UNIX) && defined(FEAT_PYTHON3)
    // Can't have Python and Python3 loaded at the same time.
    // It causes a crash, because RTLD_GLOBAL is needed for
    // standard C extension libraries of one or both python versions.
    if (python3_loaded())
    {
	if (verbose)
	    emsg(_(e_this_vim_cannot_execute_python_after_using_py3));
	return FAIL;
    }
# endif

    if (hinstPython)
	return OK;
    hinstPython = load_dll(libname);
    if (!hinstPython)
    {
	if (verbose)
	    semsg(_(e_could_not_load_library_str_str), libname, load_dll_error());
	return FAIL;
    }

    for (i = 0; python_funcname_table[i].ptr; ++i)
    {
	if ((*python_funcname_table[i].ptr = symbol_from_dll(hinstPython,
			python_funcname_table[i].name)) == NULL)
	{
	    close_dll(hinstPython);
	    hinstPython = 0;
	    if (verbose)
		semsg(_(e_could_not_load_library_function_str), python_funcname_table[i].name);
	    return FAIL;
	}
    }

    // Load unicode functions separately as only the ucs2 or the ucs4 functions
    // will be present in the library.
    *ucs_as_encoded_string = symbol_from_dll(hinstPython,
					     "PyUnicodeUCS2_AsEncodedString");
    if (*ucs_as_encoded_string == NULL)
	*ucs_as_encoded_string = symbol_from_dll(hinstPython,
					     "PyUnicodeUCS4_AsEncodedString");
    if (*ucs_as_encoded_string == NULL)
    {
	close_dll(hinstPython);
	hinstPython = 0;
	if (verbose)
	    semsg(_(e_could_not_load_library_function_str), "PyUnicode_UCSX_*");
	return FAIL;
    }

    return OK;
}

/*
 * If python is enabled (there is installed python on Windows system) return
 * TRUE, else FALSE.
 */
    int
python_enabled(int verbose)
{
    return python_runtime_link_init((char *)p_pydll, verbose) == OK;
}

/*
 * Load the standard Python exceptions - don't import the symbols from the
 * DLL, as this can cause errors (importing data symbols is not reliable).
 */
    static void
get_exceptions(void)
{
    PyObject *exmod = PyImport_ImportModule("exceptions");
    PyObject *exdict = PyModule_GetDict(exmod);
    imp_PyExc_AttributeError = PyDict_GetItemString(exdict, "AttributeError");
    imp_PyExc_IndexError = PyDict_GetItemString(exdict, "IndexError");
    imp_PyExc_KeyError = PyDict_GetItemString(exdict, "KeyError");
    imp_PyExc_KeyboardInterrupt = PyDict_GetItemString(exdict, "KeyboardInterrupt");
    imp_PyExc_TypeError = PyDict_GetItemString(exdict, "TypeError");
    imp_PyExc_ValueError = PyDict_GetItemString(exdict, "ValueError");
    imp_PyExc_SystemExit = PyDict_GetItemString(exdict, "SystemExit");
    imp_PyExc_RuntimeError = PyDict_GetItemString(exdict, "RuntimeError");
    imp_PyExc_ImportError = PyDict_GetItemString(exdict, "ImportError");
    imp_PyExc_OverflowError = PyDict_GetItemString(exdict, "OverflowError");
    Py_XINCREF(imp_PyExc_AttributeError);
    Py_XINCREF(imp_PyExc_IndexError);
    Py_XINCREF(imp_PyExc_KeyError);
    Py_XINCREF(imp_PyExc_KeyboardInterrupt);
    Py_XINCREF(imp_PyExc_TypeError);
    Py_XINCREF(imp_PyExc_ValueError);
    Py_XINCREF(imp_PyExc_SystemExit);
    Py_XINCREF(imp_PyExc_RuntimeError);
    Py_XINCREF(imp_PyExc_ImportError);
    Py_XINCREF(imp_PyExc_OverflowError);
    Py_XDECREF(exmod);
}
#endif // DYNAMIC_PYTHON

static int initialised = 0;
#define PYINITIALISED initialised
static int python_end_called = FALSE;

#define DESTRUCTOR_FINISH(self) self->ob_type->tp_free((PyObject*)self);

#define WIN_PYTHON_REF(win) win->w_python_ref
#define BUF_PYTHON_REF(buf) buf->b_python_ref
#define TAB_PYTHON_REF(tab) tab->tp_python_ref

static PyObject *OutputGetattr(PyObject *, char *);
static PyObject *BufferGetattr(PyObject *, char *);
static PyObject *WindowGetattr(PyObject *, char *);
static PyObject *TabPageGetattr(PyObject *, char *);
static PyObject *RangeGetattr(PyObject *, char *);
static PyObject *DictionaryGetattr(PyObject *, char*);
static PyObject *ListGetattr(PyObject *, char *);
static PyObject *FunctionGetattr(PyObject *, char *);

#ifndef Py_VISIT
# define Py_VISIT(obj) visit(obj, arg)
#endif
#ifndef Py_CLEAR
# define Py_CLEAR(obj) \
    { \
	Py_XDECREF(obj); \
	obj = NULL; \
    }
#endif

#if defined(HAVE_LOCALE_H) || defined(X_LOCALE)
    static void *
py_memsave(void *p, size_t len)
{
    void	*r;

    if (!(r = PyMem_Malloc(len)))
	return NULL;
    mch_memmove(r, p, len);
    return r;
}

# define PY_STRSAVE(s) ((char_u *) py_memsave(s, STRLEN(s) + 1))
#endif

typedef PySliceObject PySliceObject_T;

/*
 * Include the code shared with if_python3.c
 */
#include "if_py_both.h"


///////////////////////////////////////////////////////
// Internal function prototypes.

static int PythonMod_Init(void);


///////////////////////////////////////////////////////
// 1. Python interpreter main program.

#if PYTHON_API_VERSION < 1007 // Python 1.4
typedef PyObject PyThreadState;
#endif

#ifndef PY_CAN_RECURSE
static PyThreadState *saved_python_thread = NULL;

/*
 * Suspend a thread of the Python interpreter, other threads are allowed to
 * run.
 */
    static void
Python_SaveThread(void)
{
    saved_python_thread = PyEval_SaveThread();
}

/*
 * Restore a thread of the Python interpreter, waits for other threads to
 * block.
 */
    static void
Python_RestoreThread(void)
{
    PyEval_RestoreThread(saved_python_thread);
    saved_python_thread = NULL;
}
#endif

    void
python_end(void)
{
    static int recurse = 0;

    // If a crash occurs while doing this, don't try again.
    if (recurse != 0)
	return;

    python_end_called = TRUE;
    ++recurse;

#ifdef DYNAMIC_PYTHON
    if (hinstPython && Py_IsInitialized())
    {
# ifdef PY_CAN_RECURSE
	PyGILState_Ensure();
# else
	Python_RestoreThread();	    // enter python
# endif
	Py_Finalize();
    }
#else
    if (Py_IsInitialized())
    {
# ifdef PY_CAN_RECURSE
	PyGILState_Ensure();
# else
	Python_RestoreThread();	    // enter python
# endif
	Py_Finalize();
    }
#endif

    --recurse;
}

#if (defined(DYNAMIC_PYTHON) && defined(FEAT_PYTHON3)) || defined(PROTO)
    int
python_loaded(void)
{
    return (hinstPython != 0);
}
#endif

static char *py_home_buf = NULL;

    static int
Python_Init(void)
{
    if (!initialised)
    {
#if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
	PyObject *site;
#endif

#ifdef DYNAMIC_PYTHON
	if (!python_enabled(TRUE))
	{
	    emsg(_(e_sorry_this_command_is_disabled_python_library_could_not_be_found));
	    goto fail;
	}
#endif

	if (*p_pyhome != NUL)
	{
	    // The string must not change later, make a copy in static memory.
	    py_home_buf = (char *)vim_strsave(p_pyhome);
	    if (py_home_buf != NULL)
		Py_SetPythonHome(py_home_buf);
	}
#ifdef PYTHON_HOME
	else if (mch_getenv((char_u *)"PYTHONHOME") == NULL)
	    Py_SetPythonHome(PYTHON_HOME);
#endif

	init_structs();

#if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
	// Disable implicit 'import site', because it may cause Vim to exit
	// when it can't be found.
	Py_NoSiteFlag++;
#endif

	Py_Initialize();

#if defined(PY_VERSION_HEX) && PY_VERSION_HEX >= 0x02070000
	// 'import site' explicitly.
	site = PyImport_ImportModule("site");
	if (site == NULL)
	{
	    emsg(_(e_sorry_this_command_is_disabled_python_side_module_could_not_be_loaded));
	    goto fail;
	}
	Py_DECREF(site);
#endif

	// Initialise threads, and below save the state using
	// PyEval_SaveThread.  Without the call to PyEval_SaveThread, thread
	// specific state (such as the system trace hook), will be lost
	// between invocations of Python code.
	PyEval_InitThreads();
#ifdef DYNAMIC_PYTHON
	get_exceptions();
#endif

	if (PythonIO_Init_io())
	    goto fail;

	if (PythonMod_Init())
	    goto fail;

	globals = PyModule_GetDict(PyImport_AddModule("__main__"));

	// Remove the element from sys.path that was added because of our
	// argv[0] value in PythonMod_Init().  Previously we used an empty
	// string, but depending on the OS we then get an empty entry or
	// the current directory in sys.path.
	PyRun_SimpleString("import sys; sys.path = filter(lambda x: x != '/must>not&exist', sys.path)");

	// lock is created and acquired in PyEval_InitThreads() and thread
	// state is created in Py_Initialize()
	// there _PyGILState_NoteThreadState() also sets gilcounter to 1
	// (python must have threads enabled!)
	// so the following does both: unlock GIL and save thread state in TLS
	// without deleting thread state
#ifndef PY_CAN_RECURSE
	saved_python_thread =
#endif
	    PyEval_SaveThread();

	initialised = 1;
    }

    return 0;

fail:
    // We call PythonIO_Flush() here to print any Python errors.
    // This is OK, as it is possible to call this function even
    // if PythonIO_Init_io() has not completed successfully (it will
    // not do anything in this case).
    PythonIO_Flush();
    return -1;
}

/*
 * External interface
 */
    static void
DoPyCommand(const char *cmd, rangeinitializer init_range, runner run, void *arg)
{
#ifndef PY_CAN_RECURSE
    static int		recursive = 0;
#endif
#if defined(HAVE_LOCALE_H) || defined(X_LOCALE)
    char		*saved_locale;
#endif
#ifdef PY_CAN_RECURSE
    PyGILState_STATE	pygilstate;
#endif

#ifndef PY_CAN_RECURSE
    if (recursive)
    {
	emsg(_(e_cannot_invoke_python_recursively));
	return;
    }
    ++recursive;
#endif
    if (python_end_called)
	return;

    if (Python_Init())
	goto theend;

    init_range(arg);

    Python_Release_Vim();	    // leave Vim

#if defined(HAVE_LOCALE_H) || defined(X_LOCALE)
    // Python only works properly when the LC_NUMERIC locale is "C".
    saved_locale = setlocale(LC_NUMERIC, NULL);
    if (saved_locale == NULL || STRCMP(saved_locale, "C") == 0)
	saved_locale = NULL;
    else
    {
	// Need to make a copy, value may change when setting new locale.
	saved_locale = (char *) PY_STRSAVE(saved_locale);
	(void)setlocale(LC_NUMERIC, "C");
    }
#endif

#ifdef PY_CAN_RECURSE
    pygilstate = PyGILState_Ensure();
#else
    Python_RestoreThread();	    // enter python
#endif

    run((char *) cmd, arg
#ifdef PY_CAN_RECURSE
	    , &pygilstate
#endif
	    );

#ifdef PY_CAN_RECURSE
    PyGILState_Release(pygilstate);
#else
    Python_SaveThread();	    // leave python
#endif

#if defined(HAVE_LOCALE_H) || defined(X_LOCALE)
    if (saved_locale != NULL)
    {
	(void)setlocale(LC_NUMERIC, saved_locale);
	PyMem_Free(saved_locale);
    }
#endif

    Python_Lock_Vim();		    // enter vim
    PythonIO_Flush();

theend:
#ifndef PY_CAN_RECURSE
    --recursive;
#endif
    return;
}

/*
 * ":python"
 */
    void
ex_python(exarg_T *eap)
{
    char_u *script;

    script = script_get(eap, eap->arg);
    if (!eap->skip)
    {
	if (p_pyx == 0)
	    p_pyx = 2;

	DoPyCommand(script == NULL ? (char *) eap->arg : (char *) script,
		init_range_cmd,
		(runner) run_cmd,
		(void *) eap);
    }
    vim_free(script);
}

#define BUFFER_SIZE 1024

/*
 * ":pyfile"
 */
    void
ex_pyfile(exarg_T *eap)
{
    static char buffer[BUFFER_SIZE];
    const char *file = (char *)eap->arg;
    char *p;

    if (p_pyx == 0)
	p_pyx = 2;

    // Have to do it like this. PyRun_SimpleFile requires you to pass a
    // stdio file pointer, but Vim and the Python DLL are compiled with
    // different options under Windows, meaning that stdio pointers aren't
    // compatible between the two. Yuk.
    //
    // Put the string "execfile('file')" into buffer. But, we need to
    // escape any backslashes or single quotes in the file name, so that
    // Python won't mangle the file name.
    strcpy(buffer, "execfile('");
    p = buffer + 10; // size of "execfile('"

    while (*file && p < buffer + (BUFFER_SIZE - 3))
    {
	if (*file == '\\' || *file == '\'')
	    *p++ = '\\';
	*p++ = *file++;
    }

    // If we didn't finish the file name, we hit a buffer overflow
    if (*file != '\0')
	return;

    // Put in the terminating "')" and a null
    *p++ = '\'';
    *p++ = ')';
    *p++ = '\0';

    // Execute the file
    DoPyCommand(buffer,
	    init_range_cmd,
	    (runner) run_cmd,
	    (void *) eap);
}

    void
ex_pydo(exarg_T *eap)
{
    if (p_pyx == 0)
	p_pyx = 2;

    DoPyCommand((char *)eap->arg,
	    init_range_cmd,
	    (runner)run_do,
	    (void *)eap);
}

///////////////////////////////////////////////////////
// 2. Python output stream: writes output via [e]msg().

// Implementation functions

    static PyObject *
OutputGetattr(PyObject *self, char *name)
{
    if (strcmp(name, "softspace") == 0)
	return PyInt_FromLong(((OutputObject *)(self))->softspace);
    else if (strcmp(name, "__members__") == 0)
	return ObjectDir(NULL, OutputAttrs);
    else if (strcmp(name, "errors") == 0)
	return PyString_FromString("strict");
    else if (strcmp(name, "encoding") == 0)
	return PyString_FromString(ENC_OPT);
    return Py_FindMethod(OutputMethods, self, name);
}

///////////////////////////////////////////////////////
// 3. Implementation of the Vim module for Python

// Window type - Implementation functions
// --------------------------------------

#define WindowType_Check(obj) ((obj)->ob_type == &WindowType)

// Buffer type - Implementation functions
// --------------------------------------

#define BufferType_Check(obj) ((obj)->ob_type == &BufferType)

static int BufferAssItem(PyObject *, PyInt, PyObject *);
static int BufferAssSlice(PyObject *, PyInt, PyInt, PyObject *);

// Line range type - Implementation functions
// --------------------------------------

#define RangeType_Check(obj) ((obj)->ob_type == &RangeType)

static int RangeAssItem(PyObject *, PyInt, PyObject *);
static int RangeAssSlice(PyObject *, PyInt, PyInt, PyObject *);

// Current objects type - Implementation functions
// -----------------------------------------------

static PySequenceMethods BufferAsSeq = {
    (PyInquiry)		BufferLength,	    // sq_length,    len(x)
    (binaryfunc)	0,		    // BufferConcat, sq_concat, x+y
    (PyIntArgFunc)	0,		    // BufferRepeat, sq_repeat, x*n
    (PyIntArgFunc)	BufferItem,	    // sq_item,      x[i]
    (PyIntIntArgFunc)	BufferSlice,	    // sq_slice,     x[i:j]
    (PyIntObjArgProc)	BufferAssItem,	    // sq_ass_item,  x[i]=v
    (PyIntIntObjArgProc) BufferAssSlice,    // sq_ass_slice, x[i:j]=v
    (objobjproc)	0,
    (binaryfunc)	0,
    0,
};

// Buffer object - Implementation

    static PyObject *
BufferGetattr(PyObject *self, char *name)
{
    PyObject *r;

    if ((r = BufferAttrValid((BufferObject *)(self), name)))
	return r;

    if (CheckBuffer((BufferObject *)(self)))
	return NULL;

    r = BufferAttr((BufferObject *)(self), name);
    if (r || PyErr_Occurred())
	return r;
    else
	return Py_FindMethod(BufferMethods, self, name);
}

//////////////////

    static int
BufferAssItem(PyObject *self, PyInt n, PyObject *val)
{
    return RBAsItem((BufferObject *)(self), n, val, 1, -1, NULL);
}

    static int
BufferAssSlice(PyObject *self, PyInt lo, PyInt hi, PyObject *val)
{
    return RBAsSlice((BufferObject *)(self), lo, hi, val, 1, -1, NULL);
}

static PySequenceMethods RangeAsSeq = {
    (PyInquiry)		RangeLength,	      // sq_length,    len(x)
    (binaryfunc)	0, /* RangeConcat, */ // sq_concat,    x+y
    (PyIntArgFunc)	0, /* RangeRepeat, */ // sq_repeat,    x*n
    (PyIntArgFunc)	RangeItem,	      // sq_item,      x[i]
    (PyIntIntArgFunc)	RangeSlice,	      // sq_slice,     x[i:j]
    (PyIntObjArgProc)	RangeAssItem,	      // sq_ass_item,  x[i]=v
    (PyIntIntObjArgProc) RangeAssSlice,	      // sq_ass_slice, x[i:j]=v
    (objobjproc)	0,
#if PY_MAJOR_VERSION >= 2
    (binaryfunc)	0,
    0,
#endif
};

// Line range object - Implementation

    static PyObject *
RangeGetattr(PyObject *self, char *name)
{
    if (strcmp(name, "start") == 0)
	return Py_BuildValue(Py_ssize_t_fmt, ((RangeObject *)(self))->start - 1);
    else if (strcmp(name, "end") == 0)
	return Py_BuildValue(Py_ssize_t_fmt, ((RangeObject *)(self))->end - 1);
    else if (strcmp(name, "__members__") == 0)
	return ObjectDir(NULL, RangeAttrs);
    else
	return Py_FindMethod(RangeMethods, self, name);
}

////////////////

    static int
RangeAssItem(PyObject *self, PyInt n, PyObject *val)
{
    return RBAsItem(((RangeObject *)(self))->buf, n, val,
		     ((RangeObject *)(self))->start,
		     ((RangeObject *)(self))->end,
		     &((RangeObject *)(self))->end);
}

    static int
RangeAssSlice(PyObject *self, PyInt lo, PyInt hi, PyObject *val)
{
    return RBAsSlice(((RangeObject *)(self))->buf, lo, hi, val,
		      ((RangeObject *)(self))->start,
		      ((RangeObject *)(self))->end,
		      &((RangeObject *)(self))->end);
}

// TabPage object - Implementation

    static PyObject *
TabPageGetattr(PyObject *self, char *name)
{
    PyObject *r;

    if ((r = TabPageAttrValid((TabPageObject *)(self), name)))
	return r;

    if (CheckTabPage((TabPageObject *)(self)))
	return NULL;

    r = TabPageAttr((TabPageObject *)(self), name);
    if (r || PyErr_Occurred())
	return r;
    else
	return Py_FindMethod(TabPageMethods, self, name);
}

// Window object - Implementation

    static PyObject *
WindowGetattr(PyObject *self, char *name)
{
    PyObject *r;

    if ((r = WindowAttrValid((WindowObject *)(self), name)))
	return r;

    if (CheckWindow((WindowObject *)(self)))
	return NULL;

    r = WindowAttr((WindowObject *)(self), name);
    if (r || PyErr_Occurred())
	return r;
    else
	return Py_FindMethod(WindowMethods, self, name);
}

// Tab page list object - Definitions

static PySequenceMethods TabListAsSeq = {
    (PyInquiry)		TabListLength,	    // sq_length,    len(x)
    (binaryfunc)	0,		    // sq_concat,    x+y
    (PyIntArgFunc)	0,		    // sq_repeat,    x*n
    (PyIntArgFunc)	TabListItem,	    // sq_item,      x[i]
    (PyIntIntArgFunc)	0,		    // sq_slice,     x[i:j]
    (PyIntObjArgProc)	0,		    // sq_ass_item,  x[i]=v
    (PyIntIntObjArgProc) 0,		    // sq_ass_slice, x[i:j]=v
    (objobjproc)	0,
#if PY_MAJOR_VERSION >= 2
    (binaryfunc)	0,
    0,
#endif
};

// Window list object - Definitions

static PySequenceMethods WinListAsSeq = {
    (PyInquiry)		WinListLength,	    // sq_length,    len(x)
    (binaryfunc)	0,		    // sq_concat,    x+y
    (PyIntArgFunc)	0,		    // sq_repeat,    x*n
    (PyIntArgFunc)	WinListItem,	    // sq_item,      x[i]
    (PyIntIntArgFunc)	0,		    // sq_slice,     x[i:j]
    (PyIntObjArgProc)	0,		    // sq_ass_item,  x[i]=v
    (PyIntIntObjArgProc) 0,		    // sq_ass_slice, x[i:j]=v
    (objobjproc)	0,
#if PY_MAJOR_VERSION >= 2
    (binaryfunc)	0,
    0,
#endif
};

// External interface

    void
python_buffer_free(buf_T *buf)
{
    BufferObject *bp = BUF_PYTHON_REF(buf);
    if (bp == NULL)
	return;
    bp->buf = INVALID_BUFFER_VALUE;
    BUF_PYTHON_REF(buf) = NULL;
}

    void
python_window_free(win_T *win)
{
    WindowObject *wp = WIN_PYTHON_REF(win);
    if (wp == NULL)
	return;
    wp->win = INVALID_WINDOW_VALUE;
    WIN_PYTHON_REF(win) = NULL;
}

    void
python_tabpage_free(tabpage_T *tab)
{
    TabPageObject *tp = TAB_PYTHON_REF(tab);
    if (tp == NULL)
	return;
    tp->tab = INVALID_TABPAGE_VALUE;
    TAB_PYTHON_REF(tab) = NULL;
}

    static int
PythonMod_Init(void)
{
    // The special value is removed from sys.path in Python_Init().
    static char	*(argv[2]) = {"/must>not&exist/foo", NULL};

    if (init_types())
	return -1;

    // Set sys.argv[] to avoid a crash in warn().
    PySys_SetArgv(1, argv);

    vim_module = Py_InitModule4("vim", VimMethods, (char *)NULL,
				(PyObject *)NULL, PYTHON_API_VERSION);

    if (populate_module(vim_module))
	return -1;

    if (init_sys_path())
	return -1;

    return 0;
}

//////////////////////////////////////////////////////////////////////////
// 4. Utility functions for handling the interface between Vim and Python.

// Convert a Vim line into a Python string.
// All internal newlines are replaced by null characters.
//
// On errors, the Python exception data is set, and NULL is returned.
    static PyObject *
LineToString(const char *str)
{
    PyObject *result;
    PyInt len = strlen(str);
    char *p;

    // Allocate a Python string object, with uninitialised contents. We
    // must do it this way, so that we can modify the string in place
    // later. See the Python source, Objects/stringobject.c for details.
    result = PyString_FromStringAndSize(NULL, len);
    if (result == NULL)
	return NULL;

    p = PyString_AsString(result);

    while (*str)
    {
	if (*str == '\n')
	    *p = '\0';
	else
	    *p = *str;

	++p;
	++str;
    }

    return result;
}

    static PyObject *
DictionaryGetattr(PyObject *self, char *name)
{
    DictionaryObject	*this = ((DictionaryObject *) (self));

    if (strcmp(name, "locked") == 0)
	return PyInt_FromLong(this->dict->dv_lock);
    else if (strcmp(name, "scope") == 0)
	return PyInt_FromLong(this->dict->dv_scope);
    else if (strcmp(name, "__members__") == 0)
	return ObjectDir(NULL, DictionaryAttrs);

    return Py_FindMethod(DictionaryMethods, self, name);
}

    static PyObject *
ListGetattr(PyObject *self, char *name)
{
    if (strcmp(name, "locked") == 0)
	return PyInt_FromLong(((ListObject *)(self))->list->lv_lock);
    else if (strcmp(name, "__members__") == 0)
	return ObjectDir(NULL, ListAttrs);

    return Py_FindMethod(ListMethods, self, name);
}

    static PyObject *
FunctionGetattr(PyObject *self, char *name)
{
    PyObject	*r;

    r = FunctionAttr((FunctionObject *)(self), name);

    if (r || PyErr_Occurred())
	return r;
    else
	return Py_FindMethod(FunctionMethods, self, name);
}

    void
do_pyeval(char_u *str, typval_T *rettv)
{
    DoPyCommand((char *) str,
	    init_range_eval,
	    (runner) run_eval,
	    (void *) rettv);
    if (rettv->v_type == VAR_UNKNOWN)
    {
	rettv->v_type = VAR_NUMBER;
	rettv->vval.v_number = 0;
    }
}

// Don't generate a prototype for the next function, it generates an error on
// newer Python versions.
#if PYTHON_API_VERSION < 1007 /* Python 1.4 */ && !defined(PROTO)

    char *
Py_GetProgramName(void)
{
    return "vim";
}
#endif // Python 1.4

    int
set_ref_in_python(int copyID)
{
    return set_ref_in_py(copyID);
}
