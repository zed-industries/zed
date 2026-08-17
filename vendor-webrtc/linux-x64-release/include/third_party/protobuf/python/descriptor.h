// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PYUPB_DESCRIPTOR_H__
#define PYUPB_DESCRIPTOR_H__

#include <stdbool.h>

#include "python/python_api.h"
#include "upb/reflection/def.h"

typedef enum {
  kPyUpb_Descriptor = 0,
  kPyUpb_EnumDescriptor = 1,
  kPyUpb_EnumValueDescriptor = 2,
  kPyUpb_FieldDescriptor = 3,
  kPyUpb_FileDescriptor = 4,
  kPyUpb_MethodDescriptor = 5,
  kPyUpb_OneofDescriptor = 6,
  kPyUpb_ServiceDescriptor = 7,
  kPyUpb_Descriptor_Count = 8,
} PyUpb_DescriptorType;

// Given a descriptor object |desc|, returns a Python message class object for
// the msgdef |m|, which must be from the same pool.
PyObject* PyUpb_Descriptor_GetClass(const upb_MessageDef* m);

// Set the message descriptor's meta class.
void PyUpb_Descriptor_SetClass(PyObject* py_descriptor, PyObject* meta);

// Returns a Python wrapper object for the given def. This will return an
// existing object if one already exists, otherwise a new object will be
// created.  The caller always owns a ref on the returned object.
PyObject* PyUpb_Descriptor_Get(const upb_MessageDef* msgdef);
PyObject* PyUpb_EnumDescriptor_Get(const upb_EnumDef* enumdef);
PyObject* PyUpb_FieldDescriptor_Get(const upb_FieldDef* field);
PyObject* PyUpb_FileDescriptor_Get(const upb_FileDef* file);
PyObject* PyUpb_OneofDescriptor_Get(const upb_OneofDef* oneof);
PyObject* PyUpb_EnumValueDescriptor_Get(const upb_EnumValueDef* enumval);
PyObject* PyUpb_Descriptor_GetOrCreateWrapper(const upb_MessageDef* msg);
PyObject* PyUpb_ServiceDescriptor_Get(const upb_ServiceDef* s);
PyObject* PyUpb_MethodDescriptor_Get(const upb_MethodDef* s);

// Returns the underlying |def| for a given wrapper object. The caller must
// have already verified that the given Python object is of the expected type.
const upb_FileDef* PyUpb_FileDescriptor_GetDef(PyObject* file);
const upb_FieldDef* PyUpb_FieldDescriptor_GetDef(PyObject* file);
const upb_MessageDef* PyUpb_Descriptor_GetDef(PyObject* _self);
const void* PyUpb_AnyDescriptor_GetDef(PyObject* _self);

// Returns the underlying |def| for a given wrapper object. The caller must
// have already verified that the given Python object is of the expected type.
const upb_FileDef* PyUpb_FileDescriptor_GetDef(PyObject* file);

// Module-level init.
bool PyUpb_InitDescriptor(PyObject* m);

#endif  // PYUPB_DESCRIPTOR_H__
