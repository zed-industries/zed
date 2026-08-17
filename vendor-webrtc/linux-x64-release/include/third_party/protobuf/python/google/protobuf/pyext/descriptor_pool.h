// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_DESCRIPTOR_POOL_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_DESCRIPTOR_POOL_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include "absl/container/flat_hash_map.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace python {

struct PyMessageFactory;

// The (meta) type of all Messages classes.
struct CMessageClass;

// Wraps operations to the global DescriptorPool which contains information
// about all messages and fields.
//
// There is normally one pool per process. We make it a Python object only
// because it contains many Python references.
//
// "Methods" that interacts with this DescriptorPool are in the cdescriptor_pool
// namespace.
typedef struct PyDescriptorPool {
  // clang-format off
  PyObject_HEAD

  // The C++ pool containing Descriptors.
  const DescriptorPool* pool;
  // clang-format on

  // True if we should free the pointer above.
  bool is_owned;

  // True if this pool accepts new proto definitions.
  // In this case it is allowed to const_cast<DescriptorPool*>(pool).
  bool is_mutable;


  // The error collector to store error info. Can be NULL. This pointer is
  // owned.
  DescriptorPool::ErrorCollector* error_collector;

  // The C++ pool acting as an underlay. Can be NULL.
  // This pointer is not owned and must stay alive.
  const DescriptorPool* underlay;

  // The C++ descriptor database used to fetch unknown protos. Can be NULL.
  // This pointer is owned.
  const DescriptorDatabase* database;

  // The preferred MessageFactory to be used by descriptors.
  // TODO: Don't create the Factory from the DescriptorPool, but
  // use the one passed while creating message classes. And remove this member.
  PyMessageFactory* py_message_factory;

  // Cache the options for any kind of descriptor.
  // Descriptor pointers are owned by the DescriptorPool above.
  // Python objects are owned by the map.
  absl::flat_hash_map<const void*, PyObject*>* descriptor_options;
  // Similar cache for features.
  absl::flat_hash_map<const void*, PyObject*>* descriptor_features;
} PyDescriptorPool;

extern PyTypeObject PyDescriptorPool_Type;

namespace cdescriptor_pool {


// The functions below are also exposed as methods of the DescriptorPool type.

// Looks up a field by name. Returns a PyFieldDescriptor corresponding to
// the field on success, or NULL on failure.
//
// Returns a new reference.
PyObject* FindFieldByName(PyDescriptorPool* self, PyObject* name);

// Looks up an extension by name. Returns a PyFieldDescriptor corresponding
// to the field on success, or NULL on failure.
//
// Returns a new reference.
PyObject* FindExtensionByName(PyDescriptorPool* self, PyObject* arg);

// Looks up an enum type by name. Returns a PyEnumDescriptor corresponding
// to the field on success, or NULL on failure.
//
// Returns a new reference.
PyObject* FindEnumTypeByName(PyDescriptorPool* self, PyObject* arg);

// Looks up a oneof by name. Returns a COneofDescriptor corresponding
// to the oneof on success, or NULL on failure.
//
// Returns a new reference.
PyObject* FindOneofByName(PyDescriptorPool* self, PyObject* arg);

}  // namespace cdescriptor_pool

// Retrieves the global descriptor pool owned by the _message module.
// This is the one used by pb2.py generated modules.
// Returns a *borrowed* reference.
// "Default" pool used to register messages from _pb2.py modules.
PyDescriptorPool* GetDefaultDescriptorPool();

// Retrieves an existing python descriptor pool owning the C++ descriptor pool.
// Returns a *borrowed* reference.
PyDescriptorPool* GetDescriptorPool_FromPool(const DescriptorPool* pool);

// Wraps a C++ descriptor pool in a Python object, creates it if necessary.
// Returns a new reference.
PyObject* PyDescriptorPool_FromPool(const DescriptorPool* pool);

// Initialize objects used by this module.
bool InitDescriptorPool();

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_DESCRIPTOR_POOL_H__
