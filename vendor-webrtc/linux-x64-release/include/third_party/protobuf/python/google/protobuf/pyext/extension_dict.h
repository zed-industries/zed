// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: anuraag@google.com (Anuraag Agrawal)
// Author: tibell@google.com (Johan Tibell)

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_EXTENSION_DICT_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_EXTENSION_DICT_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include "google/protobuf/pyext/message.h"

namespace google {
namespace protobuf {

class Message;
class FieldDescriptor;

namespace python {

typedef struct ExtensionDict {
  // clang-format off
  PyObject_HEAD

  // Strong, owned reference to the parent message. Never NULL.
  CMessage* parent;
  // clang-format on
} ExtensionDict;

extern PyTypeObject ExtensionDict_Type;
extern PyTypeObject ExtensionIterator_Type;

namespace extension_dict {

// Builds an Extensions dict for a specific message.
ExtensionDict* NewExtensionDict(CMessage* parent);

}  // namespace extension_dict
}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_EXTENSION_DICT_H__
