// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// This file can be included by other C++ libraries, typically extension modules
// which want to interact with the Python Messages coming from the "cpp"
// implementation of protocol buffers.
//
// Usage:
// Declare a (probably static) variable to hold the API:
//    const PyProto_API* py_proto_api;
// In some initialization function, write:
//    py_proto_api = static_cast<const PyProto_API*>(PyCapsule_Import(
//        PyProtoAPICapsuleName(), 0));
//    if (!py_proto_api) { ...handle ImportError... }
// Then use the methods of the returned class:
//    py_proto_api->GetConstMessagePointer(...);

#ifndef GOOGLE_PROTOBUF_PYTHON_PROTO_API_H__
#define GOOGLE_PROTOBUF_PYTHON_PROTO_API_H__

#include <cstddef>
#include <string>
#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include "absl/status/status.h"
#include "google/protobuf/descriptor_database.h"
#include "google/protobuf/message.h"

PyObject* pymessage_mutate_const(PyObject* self, PyObject* args);

namespace google {
namespace protobuf {
namespace python {

class PythonMessageMutator;
class PythonConstMessagePointer;

// Note on the implementation:
// This API is designed after
// https://docs.python.org/3/extending/extending.html#providing-a-c-api-for-an-extension-module
// The class below contains no mutable state, and all methods are "const";
// we use a C++ class instead of a C struct with functions pointers just because
// the code looks more readable.
struct PyProto_API {
  // The API object is created at initialization time and never freed.
  // This destructor is never called.
  virtual ~PyProto_API() {}

  // Operations on Messages.

  // Returns a PythonMessageMutator which the python message has been cleared.
  // This API works with UPB, Cpp Extension and Pure Python.
  // Side-effect: The message will definitely be cleared. *When* the message
  // gets cleared is undefined (C++ will clear it up-front, python/upb will
  // clear it on destruction).  Nothing should rely on the python message
  // during the lifetime of this object.
  // User should not hold onto the returned PythonMessageMutator while
  // calling back into Python.
  // Warning: there is a risk of deadlock with Python/C++ if users use the
  // returned message->GetDescriptor()->file->pool()
  virtual absl::StatusOr<PythonMessageMutator> GetClearedMessageMutator(
      PyObject* msg) const = 0;

  // Returns a PythonConstMessagePointer. For UPB and Pure Python, it points
  // to a new c++ message copied from python message. For cpp extension, it
  // points the internal c++ message.
  // User should not hold onto the returned PythonConstMessagePointer
  // while calling back into Python.
  virtual absl::StatusOr<PythonConstMessagePointer> GetConstMessagePointer(
      PyObject* msg) const = 0;

  // If the passed object is a Python Message, returns its internal pointer.
  // Otherwise, returns NULL with an exception set.
  // TODO: Remove deprecated GetMessagePointer().
  [[deprecated(
      "GetMessagePointer() only work with Cpp Extension, "
      "please migrate to GetConstMessagePointer().")]]
  virtual const Message* GetMessagePointer(PyObject* msg) const = 0;

  // If the passed object is a Python Message, returns a mutable pointer.
  // Otherwise, returns NULL with an exception set.
  // This function will succeed only if there are no other Python objects
  // pointing to the message, like submessages or repeated containers.
  // With the current implementation, only empty messages are in this case.
  // TODO: Remove deprecated GetMutableMessagePointer().
  [[deprecated(
      "GetMutableMessagePointer() only work with Cpp Extension, "
      "please migrate to GetClearedMessageMutator().")]]
  virtual Message* GetMutableMessagePointer(PyObject* msg) const = 0;

  // If the passed object is a Python Message Descriptor, returns its internal
  // pointer.
  // Otherwise, returns NULL with an exception set.
  virtual const Descriptor* MessageDescriptor_AsDescriptor(
      PyObject* desc) const = 0;

  // If the passed object is a Python Enum Descriptor, returns its internal
  // pointer.
  // Otherwise, returns NULL with an exception set.
  virtual const EnumDescriptor* EnumDescriptor_AsDescriptor(
      PyObject* enum_desc) const = 0;

  // Expose the underlying DescriptorPool and MessageFactory to enable C++ code
  // to create Python-compatible message.
  virtual const DescriptorPool* GetDefaultDescriptorPool() const = 0;
  virtual MessageFactory* GetDefaultMessageFactory() const = 0;

  // Allocate a new protocol buffer as a python object for the provided
  // descriptor. This function works even if no Python module has been imported
  // for the corresponding protocol buffer class.
  // The factory is usually null; when provided, it is the MessageFactory which
  // owns the Python class, and will be used to find and create Extensions for
  // this message.
  // When null is returned, a python error has already been set.
  virtual PyObject* NewMessage(const Descriptor* descriptor,
                               PyObject* py_message_factory) const = 0;

  // Allocate a new protocol buffer where the underlying object is owned by C++.
  // The factory must currently be null.  This function works even if no Python
  // module has been imported for the corresponding protocol buffer class.
  // When null is returned, a python error has already been set.
  //
  // Since this call returns a python object owned by C++, some operations
  // are risky, and it must be used carefully. In particular:
  // * Avoid modifying the returned object from the C++ side while there are
  // existing python references to it or it's subobjects.
  // * Avoid using python references to this object or any subobjects after the
  // C++ object has been freed.
  // * Calling this with the same C++ pointer will result in multiple distinct
  // python objects referencing the same C++ object.
  virtual PyObject* NewMessageOwnedExternally(
      Message* msg, PyObject* py_message_factory) const = 0;

  // Returns a new reference for the given DescriptorPool.
  // The returned object does not manage the C++ DescriptorPool: it is the
  // responsibility of the caller to keep it alive.
  // As long as the returned Python DescriptorPool object is kept alive,
  // functions that process C++ descriptors or messages created from this pool
  // can work and return their Python counterparts.
  virtual PyObject* DescriptorPool_FromPool(
      const google::protobuf::DescriptorPool* pool) const = 0;

 protected:
  PythonMessageMutator CreatePythonMessageMutator(Message* owned_msg,
                                                  Message* msg,
                                                  PyObject* py_msg) const;
  PythonConstMessagePointer CreatePythonConstMessagePointer(
      Message* owned_msg, const Message* msg, PyObject* py_msg) const;
};

// User should not hold onto this object while calling back into Python
class PythonMessageMutator {
 public:
  PythonMessageMutator(PythonMessageMutator&& other);
  ~PythonMessageMutator();

  Message* get() { return message_; }
  Message* operator->() { return message_; }
  const Message& operator*() { return *message_; }

 private:
  friend struct google::protobuf::python::PyProto_API;
  PythonMessageMutator(Message* owned_msg, Message* message, PyObject* py_msg);
  // owned_msg_ is set for UPB/Pure Python. Cpp
  // Extension should not set owned_msg_.
  // owned_msg_ is a new Message for UPB/Pure Python.
  // owned_msg_ is nullptr for Cpp Extension.
  std::unique_ptr<Message> owned_msg_;
  // message_ points to owned_msg_ for UPB/Pure Python.
  // message_ points to in-place Message* for Cpp Extension.
  Message* message_;
  // py_msg_ points to the python message. message_ content will be serialized
  // to py_msg_ at destructor for UPB/Pure Python, CPP Extension won't.
  PyObject* py_msg_;
};

class PythonConstMessagePointer {
 public:
  PythonConstMessagePointer(PythonConstMessagePointer&& other);
  ~PythonConstMessagePointer();

  const Message& get() { return *message_; }

 private:
  friend struct google::protobuf::python::PyProto_API;
  PythonConstMessagePointer(Message* owned_msg, const Message* message,
                            PyObject* py_msg);

  friend PyObject* ::pymessage_mutate_const(PyObject* self, PyObject* args);
  // Check if the const message has been changed.
  bool NotChanged();
  std::unique_ptr<Message> owned_msg_;
  const Message* message_;
  PyObject* py_msg_;
};

inline const char* PyProtoAPICapsuleName() {
  static const char kCapsuleName[] = "google.protobuf.pyext._message.proto_API";
  return kCapsuleName;
}

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_PROTO_API_H__
