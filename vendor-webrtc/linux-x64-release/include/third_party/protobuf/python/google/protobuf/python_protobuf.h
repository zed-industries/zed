// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: qrczak@google.com (Marcin Kowalczyk)
//
// This module exposes the C proto inside the given Python proto, in
// case the Python proto is implemented with a C proto.

#ifndef GOOGLE_PROTOBUF_PYTHON_PYTHON_PROTOBUF_H__
#define GOOGLE_PROTOBUF_PYTHON_PYTHON_PROTOBUF_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

namespace google {
namespace protobuf {

class Message;

namespace python {

// Return the pointer to the C proto inside the given Python proto,
// or NULL when this is not a Python proto implemented with a C proto.
const Message* GetCProtoInsidePyProto(PyObject* msg);
Message* MutableCProtoInsidePyProto(PyObject* msg);

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_PYTHON_PROTOBUF_H__
