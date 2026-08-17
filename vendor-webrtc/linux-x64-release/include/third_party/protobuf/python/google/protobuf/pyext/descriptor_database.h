// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_DESCRIPTOR_DATABASE_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_DESCRIPTOR_DATABASE_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <string>
#include <vector>

#include "google/protobuf/descriptor_database.h"

namespace google {
namespace protobuf {
namespace python {

class PyDescriptorDatabase : public DescriptorDatabase {
 public:
  explicit PyDescriptorDatabase(PyObject* py_database);
  ~PyDescriptorDatabase() override;

  // Implement the abstract interface. All these functions fill the output
  // with a copy of FileDescriptorProto.

  // Find a file by file name.
  bool FindFileByName(const std::string& filename,
                      FileDescriptorProto* output) override;

  // Find the file that declares the given fully-qualified symbol name.
  bool FindFileContainingSymbol(const std::string& symbol_name,
                                FileDescriptorProto* output) override;

  // Find the file which defines an extension extending the given message type
  // with the given field number.
  // Containing_type must be a fully-qualified type name.
  // Python objects are not required to implement this method.
  bool FindFileContainingExtension(const std::string& containing_type,
                                   int field_number,
                                   FileDescriptorProto* output) override;

  // Finds the tag numbers used by all known extensions of
  // containing_type, and appends them to output in an undefined
  // order.
  // Python objects are not required to implement this method.
  bool FindAllExtensionNumbers(const std::string& containing_type,
                               std::vector<int>* output) override;

 private:
  // The python object that implements the database. The reference is owned.
  PyObject* py_database_;
};

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_DESCRIPTOR_DATABASE_H__
