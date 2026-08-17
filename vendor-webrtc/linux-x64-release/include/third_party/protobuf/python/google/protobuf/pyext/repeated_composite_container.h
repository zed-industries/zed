// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: anuraag@google.com (Anuraag Agrawal)
// Author: tibell@google.com (Johan Tibell)

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_REPEATED_COMPOSITE_CONTAINER_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_REPEATED_COMPOSITE_CONTAINER_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include "google/protobuf/pyext/message.h"

namespace google {
namespace protobuf {

class FieldDescriptor;
class Message;

namespace python {

struct CMessageClass;

// A RepeatedCompositeContainer always has a parent message.
// The parent message also caches reference to items of the container.
typedef struct RepeatedCompositeContainer : public ContainerBase {
  // The type used to create new child messages.
  CMessageClass* child_message_class;
} RepeatedCompositeContainer;

extern PyTypeObject RepeatedCompositeContainer_Type;

namespace repeated_composite_container {

// Builds a RepeatedCompositeContainer object, from a parent message and a
// field descriptor.
RepeatedCompositeContainer* NewContainer(
    CMessage* parent,
    const FieldDescriptor* parent_field_descriptor,
    CMessageClass *child_message_class);

// Appends a new CMessage to the container and returns it.  The
// CMessage is initialized using the content of kwargs.
//
// Returns a new reference if successful; returns NULL and sets an
// exception if unsuccessful.
PyObject* Add(RepeatedCompositeContainer* self,
              PyObject* args,
              PyObject* kwargs);

// Appends all the CMessages in the input iterator to the container.
//
// Returns None if successful; returns NULL and sets an exception if
// unsuccessful.
PyObject* Extend(RepeatedCompositeContainer* self, PyObject* value);

// Appends a new message to the container for each message in the
// input iterator, merging each data element in. Equivalent to extend.
//
// Returns None if successful; returns NULL and sets an exception if
// unsuccessful.
PyObject* MergeFrom(RepeatedCompositeContainer* self, PyObject* other);

// Accesses messages in the container.
//
// Returns a new reference to the message for an integer parameter.
// Returns a new reference to a list of messages for a slice.
PyObject* Subscript(RepeatedCompositeContainer* self, PyObject* slice);

// Deletes items from the container (cannot be used for assignment).
//
// Returns 0 on success, -1 on failure.
int AssignSubscript(RepeatedCompositeContainer* self,
                    PyObject* slice,
                    PyObject* value);
}  // namespace repeated_composite_container
}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_REPEATED_COMPOSITE_CONTAINER_H__
