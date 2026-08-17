// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PYPB_MESSAGE_H__
#define PYPB_MESSAGE_H__

#include <stdbool.h>

#include "python/protobuf.h"
#include "upb/reflection/message.h"

// Removes the wrapper object for this field from the unset subobject cache.
void PyUpb_Message_CacheDelete(PyObject* _self, const upb_FieldDef* f);

// Sets the field value for `f` to `subobj`, evicting the wrapper object from
// the "unset subobject" cache now that real data exists for it.  The caller
// must also update the wrapper associated with `f` to point to `subobj` also.
void PyUpb_Message_SetConcreteSubobj(PyObject* _self, const upb_FieldDef* f,
                                     upb_MessageValue subobj);

// Gets a Python wrapper object for message `u_msg` of type `m`, returning a
// cached wrapper if one was previously created.  If a new object is created,
// it will reference `arena`, which must own `u_msg`.
PyObject* PyUpb_Message_Get(upb_Message* u_msg, const upb_MessageDef* m,
                            PyObject* arena);

// Verifies that a Python object is a message.  Sets a TypeError exception and
// returns false on failure.
bool PyUpb_Message_Verify(PyObject* self);

// Gets the upb_Message* for this message object if the message is reified.
// Otherwise returns NULL.
upb_Message* PyUpb_Message_GetIfReified(PyObject* _self);

// Returns the `upb_MessageDef` for a given Message.
const upb_MessageDef* PyUpb_Message_GetMsgdef(PyObject* self);

// Functions that match the corresponding methods on the message object.
PyObject* PyUpb_Message_MergeFrom(PyObject* self, PyObject* arg);
PyObject* PyUpb_Message_MergeFromString(PyObject* self, PyObject* arg);
PyObject* PyUpb_Message_SerializeToString(PyObject* self, PyObject* args,
                                          PyObject* kwargs);
PyObject* PyUpb_Message_SerializePartialToString(PyObject* self, PyObject* args,
                                                 PyObject* kwargs);

// Sets fields of the message according to the attribuges in `kwargs`.
int PyUpb_Message_InitAttributes(PyObject* _self, PyObject* args,
                                 PyObject* kwargs);

// Checks that `key` is a field descriptor for an extension type, and that the
// extendee is this message.  Otherwise returns NULL and sets a KeyError.
const upb_FieldDef* PyUpb_Message_GetExtensionDef(PyObject* _self,
                                                  PyObject* key);

// Clears the given field in this message.
void PyUpb_Message_DoClearField(PyObject* _self, const upb_FieldDef* f);

// Clears the ExtensionDict from the message.  The message must have an
// ExtensionDict set.
void PyUpb_Message_ClearExtensionDict(PyObject* _self);

// Implements the equivalent of getattr(msg, field), once `field` has
// already been resolved to a `upb_FieldDef*`.
PyObject* PyUpb_Message_GetFieldValue(PyObject* _self,
                                      const upb_FieldDef* field);

// Implements the equivalent of setattr(msg, field, value), once `field` has
// already been resolved to a `upb_FieldDef*`.
int PyUpb_Message_SetFieldValue(PyObject* _self, const upb_FieldDef* field,
                                PyObject* value, PyObject* exc);

// Creates message meta class.
PyObject* PyUpb_MessageMeta_DoCreateClass(PyObject* py_descriptor,
                                          const char* name, PyObject* dict);

// Returns the version associated with this message.  The version will be
// incremented when the message changes.
int PyUpb_Message_GetVersion(PyObject* _self);

// Module-level init.
bool PyUpb_InitMessage(PyObject* m);

#endif  // PYPB_MESSAGE_H__
