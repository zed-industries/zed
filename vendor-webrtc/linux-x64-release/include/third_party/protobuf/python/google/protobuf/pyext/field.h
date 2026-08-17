// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_FIELD_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_FIELD_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

namespace google {
namespace protobuf {

class FieldDescriptor;

namespace python {

// A data descriptor that represents a field in a Message class.
struct PyMessageFieldProperty {
  // clang-format off
  PyObject_HEAD

  // This pointer is owned by the same pool as the Message class it belongs to.
  const FieldDescriptor* field_descriptor;
  // clang-format on
};

extern PyTypeObject* CFieldProperty_Type;

PyObject* NewFieldProperty(const FieldDescriptor* field_descriptor);

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_FIELD_H__
