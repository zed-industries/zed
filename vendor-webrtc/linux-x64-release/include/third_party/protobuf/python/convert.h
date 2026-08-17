// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PYUPB_CONVERT_H__
#define PYUPB_CONVERT_H__

#include "protobuf.h"
#include "upb/reflection/def.h"
#include "upb/reflection/message.h"

// Converts `val` to a Python object according to the type information in `f`.
// Any newly-created Python objects that reference non-primitive data from `val`
// will take a reference on `arena`; the caller must ensure that `val` belongs
// to `arena`. If the conversion cannot be performed, returns NULL and sets a
// Python error.
PyObject* PyUpb_UpbToPy(upb_MessageValue val, const upb_FieldDef* f,
                        PyObject* arena);

// Converts `obj` to a upb_MessageValue `*val` according to the type information
// in `f`. If `arena` is provided, any string data will be copied into `arena`,
// otherwise the returned value will alias the Python-owned data (this can be
// useful for an ephemeral upb_MessageValue).  If the conversion cannot be
// performed, returns false.
bool PyUpb_PyToUpb(PyObject* obj, const upb_FieldDef* f, upb_MessageValue* val,
                   upb_Arena* arena);

// Returns true if the given messages (of type `m`) are equal.
bool upb_Message_IsEqualByDef(const upb_Message* msg1, const upb_Message* msg2,
                              const upb_MessageDef* msgdef, int options);

#endif  // PYUPB_CONVERT_H__
