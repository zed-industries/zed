// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PYUPB_DESCRIPTOR_POOL_H__
#define PYUPB_DESCRIPTOR_POOL_H__

#include <stdbool.h>

#include "protobuf.h"

// Returns a Python wrapper object for the given symtab. The symtab must have
// been created from a Python DescriptorPool originally.
PyObject* PyUpb_DescriptorPool_Get(const upb_DefPool* symtab);

// Given a Python DescriptorPool, returns the underlying symtab.
upb_DefPool* PyUpb_DescriptorPool_GetSymtab(PyObject* pool);

// Returns the default DescriptorPool (a global singleton).
PyObject* PyUpb_DescriptorPool_GetDefaultPool(void);

// Module-level init.
bool PyUpb_InitDescriptorPool(PyObject* m);

#endif  // PYUPB_DESCRIPTOR_POOL_H__
