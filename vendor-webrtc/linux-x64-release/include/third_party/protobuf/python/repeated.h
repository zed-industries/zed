// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PYUPB_REPEATED_H__
#define PYUPB_REPEATED_H__

#include <stdbool.h>

#include "python/python_api.h"
#include "upb/reflection/def.h"

// Creates a new repeated field stub for field `f` of message object `parent`.
// Precondition: `parent` must be a stub.
PyObject* PyUpb_RepeatedContainer_NewStub(PyObject* parent,
                                          const upb_FieldDef* f,
                                          PyObject* arena);

// Returns a repeated field object wrapping `arr`, of field type `f`, which
// must be on `arena`.  If an existing wrapper object exists, it will be
// returned, otherwise a new object will be created.  The caller always owns a
// ref on the returned value.
PyObject* PyUpb_RepeatedContainer_GetOrCreateWrapper(upb_Array* arr,
                                                     const upb_FieldDef* f,
                                                     PyObject* arena);

// Reifies a repeated field stub to point to the concrete data in `arr`.
// If `arr` is NULL, an appropriate empty array will be constructed.
void PyUpb_RepeatedContainer_Reify(PyObject* self, upb_Array* arr);

// Reifies this repeated object if it is not already reified.
upb_Array* PyUpb_RepeatedContainer_EnsureReified(PyObject* self);

// Implements repeated_field.extend(iterable).  `_self` must be a repeated
// field (either repeated composite or repeated scalar).
PyObject* PyUpb_RepeatedContainer_Extend(PyObject* _self, PyObject* value);

// Implements repeated_field.add(initial_values).  `_self` must be a repeated
// composite field.
PyObject* PyUpb_RepeatedCompositeContainer_Add(PyObject* _self, PyObject* args,
                                               PyObject* kwargs);

// Module-level init.
bool PyUpb_Repeated_Init(PyObject* m);

#endif  // PYUPB_REPEATED_H__
