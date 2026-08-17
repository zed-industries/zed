// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_MESSAGE_FACTORY_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_MESSAGE_FACTORY_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <unordered_map>
#include "google/protobuf/descriptor.h"
#include "google/protobuf/pyext/descriptor_pool.h"

namespace google {
namespace protobuf {
class MessageFactory;

namespace python {

// The (meta) type of all Messages classes.
struct CMessageClass;

struct PyMessageFactory {
  PyObject_HEAD

  // DynamicMessageFactory used to create C++ instances of messages.
  // This object cache the descriptors that were used, so the DescriptorPool
  // needs to get rid of it before it can delete itself.
  //
  // Note: A C++ MessageFactory is different from the PyMessageFactory.
  // The C++ one creates messages, when the Python one creates classes.
  MessageFactory* message_factory;

  // Owned reference to a Python DescriptorPool.
  // This reference must stay until the message_factory is destructed.
  PyDescriptorPool* pool;

  // Make our own mapping to retrieve Python classes from C++ descriptors.
  //
  // Descriptor pointers stored here are owned by the DescriptorPool above.
  // Python references to classes are owned by this PyDescriptorPool.
  typedef std::unordered_map<const Descriptor*, CMessageClass*>
      ClassesByMessageMap;
  ClassesByMessageMap* classes_by_descriptor;
};

extern PyTypeObject PyMessageFactory_Type;

namespace message_factory {

// Creates a new MessageFactory instance.
PyMessageFactory* NewMessageFactory(PyTypeObject* type, PyDescriptorPool* pool);

// Registers a new Python class for the given message descriptor.
// On error, returns -1 with a Python exception set.
int RegisterMessageClass(PyMessageFactory* self,
                         const Descriptor* message_descriptor,
                         CMessageClass* message_class);
// Retrieves the Python class registered with the given message descriptor, or
// fail with a TypeError. Returns a *borrowed* reference.
CMessageClass* GetMessageClass(PyMessageFactory* self,
                               const Descriptor* message_descriptor);
// Retrieves the Python class registered with the given message descriptor.
// The class is created if not done yet. Returns a *new* reference.
CMessageClass* GetOrCreateMessageClass(PyMessageFactory* self,
                                       const Descriptor* message_descriptor);
}  // namespace message_factory

// Initialize objects used by this module.
// On error, returns false with a Python exception set.
bool InitMessageFactory();

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_MESSAGE_FACTORY_H__
