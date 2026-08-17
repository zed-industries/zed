// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PYUPB_EXTENSION_DICT_H__
#define PYUPB_EXTENSION_DICT_H__

#include <stdbool.h>

#include "python/python_api.h"

PyObject* PyUpb_ExtensionDict_New(PyObject* msg);

bool PyUpb_InitExtensionDict(PyObject* m);

#endif  // PYUPB_EXTENSION_DICT_H__
