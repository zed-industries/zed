// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PYUPB_UNKNOWN_FIELDS_H__
#define PYUPB_UNKNOWN_FIELDS_H__

#include <stdbool.h>

#include "python/python_api.h"

PyObject* PyUpb_UnknownFields_New(PyObject* msg);

bool PyUpb_UnknownFields_Init(PyObject* m);

#endif  // PYUPB_UNKNOWN_FIELDS_H__
