// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_MAP_CONTAINER_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_MAP_CONTAINER_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <cstdint>

#include "google/protobuf/descriptor.h"
#include "google/protobuf/message.h"
#include "google/protobuf/pyext/message.h"

namespace google {
namespace protobuf {

class Message;

namespace python {

struct CMessageClass;

// This struct is used directly for ScalarMap, and is the base class of
// MessageMapContainer, which is used for MessageMap.
struct MapContainer : public ContainerBase {
  // Use to get a mutable message when necessary.
  Message* GetMutableMessage();

  // We bump this whenever we perform a mutation, to invalidate existing
  // iterators.
  uint64_t version;
};

struct MessageMapContainer : public MapContainer {
  // The type used to create new child messages.
  CMessageClass* message_class;
};

bool InitMapContainers();

extern PyTypeObject* MessageMapContainer_Type;
extern PyTypeObject* ScalarMapContainer_Type;
extern PyTypeObject MapIterator_Type;  // Both map types use the same iterator.

// Builds a MapContainer object, from a parent message and a
// field descriptor.
extern MapContainer* NewScalarMapContainer(
    CMessage* parent, const FieldDescriptor* parent_field_descriptor);

// Builds a MessageMap object, from a parent message and a
// field descriptor.
extern MessageMapContainer* NewMessageMapContainer(
    CMessage* parent, const FieldDescriptor* parent_field_descriptor,
    CMessageClass* message_class);

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_MAP_CONTAINER_H__
