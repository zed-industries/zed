// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PYUPB_MAP_H__
#define PYUPB_MAP_H__

#include <stdbool.h>

#include "python/python_api.h"
#include "upb/reflection/def.h"

// Creates a new repeated field stub for field `f` of message object `parent`.
// Precondition: `parent` must be a stub.
PyObject* PyUpb_MapContainer_NewStub(PyObject* parent, const upb_FieldDef* f,
                                     PyObject* arena);

// Returns a map object wrapping `map`, of field type `f`, which must be on
// `arena`.  If an existing wrapper object exists, it will be returned,
// otherwise a new object will be created.  The caller always owns a ref on the
// returned value.
PyObject* PyUpb_MapContainer_GetOrCreateWrapper(upb_Map* map,
                                                const upb_FieldDef* f,
                                                PyObject* arena);

// Reifies a map stub to point to the concrete data in `map`.
// If `map` is NULL, an appropriate empty map will be constructed.
void PyUpb_MapContainer_Reify(PyObject* self, upb_Map* map);

// Reifies this map object if it is not already reified.
upb_Map* PyUpb_MapContainer_EnsureReified(PyObject* self);

// Invalidates any existing iterators for the map `obj`.
void PyUpb_MapContainer_Invalidate(PyObject* obj);

// Module-level init.
bool PyUpb_Map_Init(PyObject* m);

#endif  // PYUPB_MAP_H__
