// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_DESCRIPTOR_CONTAINERS_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_DESCRIPTOR_CONTAINERS_H__

// Mappings and Sequences of descriptors.
// They implement containers like fields_by_name, EnumDescriptor.values...
// See descriptor_containers.cc for more description.
#define PY_SSIZE_T_CLEAN
#include <Python.h>

namespace google {
namespace protobuf {

class Descriptor;
class FileDescriptor;
class EnumDescriptor;
class OneofDescriptor;
class ServiceDescriptor;

namespace python {

// Initialize the various types and objects.
bool InitDescriptorMappingTypes();

// Each function below returns a Mapping, or a Sequence of descriptors.
// They all return a new reference.

namespace message_descriptor {
PyObject* NewMessageFieldsByName(const Descriptor* descriptor);
PyObject* NewMessageFieldsByCamelcaseName(const Descriptor* descriptor);
PyObject* NewMessageFieldsByNumber(const Descriptor* descriptor);
PyObject* NewMessageFieldsSeq(const Descriptor* descriptor);

PyObject* NewMessageNestedTypesSeq(const Descriptor* descriptor);
PyObject* NewMessageNestedTypesByName(const Descriptor* descriptor);

PyObject* NewMessageEnumsByName(const Descriptor* descriptor);
PyObject* NewMessageEnumsSeq(const Descriptor* descriptor);
PyObject* NewMessageEnumValuesByName(const Descriptor* descriptor);

PyObject* NewMessageExtensionsByName(const Descriptor* descriptor);
PyObject* NewMessageExtensionsSeq(const Descriptor* descriptor);

PyObject* NewMessageOneofsByName(const Descriptor* descriptor);
PyObject* NewMessageOneofsSeq(const Descriptor* descriptor);
}  // namespace message_descriptor

namespace enum_descriptor {
PyObject* NewEnumValuesByName(const EnumDescriptor* descriptor);
PyObject* NewEnumValuesByNumber(const EnumDescriptor* descriptor);
PyObject* NewEnumValuesSeq(const EnumDescriptor* descriptor);
}  // namespace enum_descriptor

namespace oneof_descriptor {
PyObject* NewOneofFieldsSeq(const OneofDescriptor* descriptor);
}  // namespace oneof_descriptor

namespace file_descriptor {
PyObject* NewFileMessageTypesByName(const FileDescriptor* descriptor);

PyObject* NewFileEnumTypesByName(const FileDescriptor* descriptor);

PyObject* NewFileExtensionsByName(const FileDescriptor* descriptor);

PyObject* NewFileServicesByName(const FileDescriptor* descriptor);

PyObject* NewFileDependencies(const FileDescriptor* descriptor);
PyObject* NewFilePublicDependencies(const FileDescriptor* descriptor);
}  // namespace file_descriptor

namespace service_descriptor {
PyObject* NewServiceMethodsSeq(const ServiceDescriptor* descriptor);
PyObject* NewServiceMethodsByName(const ServiceDescriptor* descriptor);
}  // namespace service_descriptor


}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_DESCRIPTOR_CONTAINERS_H__
