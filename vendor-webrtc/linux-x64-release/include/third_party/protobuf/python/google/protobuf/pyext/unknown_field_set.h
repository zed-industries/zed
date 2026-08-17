// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_UNKNOWN_FIELD_SET_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_UNKNOWN_FIELD_SET_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <memory>
#include <set>

#include "google/protobuf/pyext/message.h"

namespace google {
namespace protobuf {

class UnknownField;
class UnknownFieldSet;

namespace python {
struct CMessage;

struct PyUnknownFieldSet {
  // clang-format off
  PyObject_HEAD
  // If parent is nullptr, it is a top UnknownFieldSet.
  PyUnknownFieldSet* parent;
  // clang-format on

  // Top UnknownFieldSet owns fields pointer. Sub UnknownFieldSet
  // does not own fields pointer.
  UnknownFieldSet* fields;
};

struct PyUnknownField {
  // clang-format off
  PyObject_HEAD
  // Every Python PyUnknownField holds a reference to its parent
  // PyUnknownFieldSet in order to keep it alive.
  PyUnknownFieldSet* parent;
  // clang-format on

  // The UnknownField index in UnknownFieldSet.
  Py_ssize_t index;
};

extern PyTypeObject PyUnknownFieldSet_Type;
extern PyTypeObject PyUnknownField_Type;

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_UNKNOWN_FIELD_SET_H__
