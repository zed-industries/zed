// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: anuraag@google.com (Anuraag Agrawal)
// Author: tibell@google.com (Johan Tibell)

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_REPEATED_SCALAR_CONTAINER_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_REPEATED_SCALAR_CONTAINER_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include "google/protobuf/descriptor.h"
#include "google/protobuf/pyext/message.h"

namespace google {
namespace protobuf {
namespace python {

typedef struct RepeatedScalarContainer : public ContainerBase {
} RepeatedScalarContainer;

extern PyTypeObject RepeatedScalarContainer_Type;

namespace repeated_scalar_container {

// Builds a RepeatedScalarContainer object, from a parent message and a
// field descriptor.
extern RepeatedScalarContainer* NewContainer(
    CMessage* parent, const FieldDescriptor* parent_field_descriptor);

// Appends the scalar 'item' to the end of the container 'self'.
//
// Returns None if successful; returns NULL and sets an exception if
// unsuccessful.
PyObject* Append(RepeatedScalarContainer* self, PyObject* item);

// Appends all the elements in the input iterator to the container.
//
// Returns None if successful; returns NULL and sets an exception if
// unsuccessful.
PyObject* Extend(RepeatedScalarContainer* self, PyObject* value);

}  // namespace repeated_scalar_container
}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_REPEATED_SCALAR_CONTAINER_H__
