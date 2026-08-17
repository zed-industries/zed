// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PYUPB_PYTHON_H__
#define PYUPB_PYTHON_H__

// We restrict ourselves to the limited API, so that a single build can be
// ABI-compatible with a wide range of Python versions.
//
// The build system will define Py_LIMITED_API as appropriate (see BUILD). We
// only want to define it for our distribution packages, since we can do some
// extra assertions when Py_LIMITED_API is not defined.  Also Py_LIMITED_API is
// incompatible with Py_DEBUG.

// #define Py_LIMITED_API <val>  // Defined by build system when appropriate.

#include "Python.h"

// Ideally we could restrict ourselves to the limited API of 3.7, but this is
// a very important function that was not officially added to the limited API
// until 3.10.  Without this function, there is no way of getting data from a
// Python `str` object without a copy.
//
// While this function was not *officially* added to the limited API until
// Python 3.10, In practice it has been stable since Python 3.1.
//   https://bugs.python.org/issue41784
//
// On Linux/ELF and macOS/Mach-O, we can get away with using this function with
// the limited API prior to 3.10.

#if (defined(__linux__) || defined(__APPLE__)) && defined(Py_LIMITED_API) && \
    Py_LIMITED_API < 0x03100000
PyAPI_FUNC(const char*)
    PyUnicode_AsUTF8AndSize(PyObject* unicode, Py_ssize_t* size);
#endif

#endif  // PYUPB_PYTHON_H__
