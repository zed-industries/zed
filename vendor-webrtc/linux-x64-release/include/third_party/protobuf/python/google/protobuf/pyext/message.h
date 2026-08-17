// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: anuraag@google.com (Anuraag Agrawal)
// Author: tibell@google.com (Johan Tibell)

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_MESSAGE_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_MESSAGE_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <cstdint>
#include <memory>
#include <string>
#include <unordered_map>

#include "absl/strings/string_view.h"

namespace google {
namespace protobuf {

class Message;
class Reflection;
class FieldDescriptor;
class Descriptor;
class DescriptorPool;
class MessageFactory;

namespace python {

struct ExtensionDict;
struct PyMessageFactory;
struct CMessageClass;

// Most of the complexity of the Message class comes from the "Release"
// behavior:
//
// When a field is cleared, it is only detached from its message. Existing
// references to submessages, to repeated container etc. won't see any change,
// as if the data was effectively managed by these containers.
//
// ExtensionDicts and UnknownFields containers do NOT follow this rule. They
// don't store any data, and always refer to their parent message.

struct ContainerBase {
  // clang-format off
  PyObject_HEAD

  // Strong reference to a parent message object. For a CMessage there are three
  // cases:
  // - For a top-level message, this pointer is NULL.
  // - For a sub-message, this points to the parent message.
  // - For a message managed externally, this is a owned reference to Py_None.
  //
  // For all other types: repeated containers, maps, it always point to a
  // valid parent CMessage.
  struct CMessage* parent;
  // clang-format on

  // If this object belongs to a parent message, describes which field it comes
  // from.
  // The pointer is owned by the DescriptorPool (which is kept alive
  // through the message's Python class)
  const FieldDescriptor* parent_field_descriptor;

  PyObject* AsPyObject() { return reinterpret_cast<PyObject*>(this); }

  // The Three methods below are only used by Repeated containers, and Maps.

  // This implementation works for all containers which have a parent.
  PyObject* DeepCopy();
  // Delete this container object from its parent. Does not work for messages.
  void RemoveFromParentCache();
};

typedef struct CMessage : public ContainerBase {
  // Pointer to the C++ Message object for this CMessage.
  // - If this object has no parent, we own this pointer.
  // - If this object has a parent message, the parent owns this pointer.
  Message* message;

  // Indicates this submessage is pointing to a default instance of a message.
  // Submessages are always first created as read only messages and are then
  // made writable, at which point this field is set to false.
  bool read_only;

  // A mapping indexed by field, containing weak references to contained objects
  // which need to implement the "Release" mechanism:
  // direct submessages, RepeatedCompositeContainer, RepeatedScalarContainer
  // and MapContainer.
  typedef std::unordered_map<const FieldDescriptor*, ContainerBase*>
      CompositeFieldsMap;
  CompositeFieldsMap* composite_fields;

  // A mapping containing weak references to indirect child messages, accessed
  // through containers: repeated messages, and values of message maps.
  // This avoid the creation of similar maps in each of those containers.
  typedef std::unordered_map<const Message*, CMessage*> SubMessagesMap;
  SubMessagesMap* child_submessages;

  // Implements the "weakref" protocol for this object.
  PyObject* weakreflist;

  // Return a *borrowed* reference to the message class.
  CMessageClass* GetMessageClass() {
    return reinterpret_cast<CMessageClass*>(Py_TYPE(this));
  }

  // For container containing messages, return a Python object for the given
  // pointer to a message.
  CMessage* BuildSubMessageFromPointer(const FieldDescriptor* field_descriptor,
                                       Message* sub_message,
                                       CMessageClass* message_class);
  CMessage* MaybeReleaseSubMessage(Message* sub_message);
} CMessage;

// The (meta) type of all Messages classes.
// It allows us to cache some C++ pointers in the class object itself, they are
// faster to extract than from the type's dictionary.

struct CMessageClass {
  // This is how CPython subclasses C structures: the base structure must be
  // the first member of the object.
  PyHeapTypeObject super;

  // C++ descriptor of this message.
  const Descriptor* message_descriptor;

  // Owned reference, used to keep the pointer above alive.
  // This reference must stay alive until all message pointers are destructed.
  PyObject* py_message_descriptor;

  // The Python MessageFactory used to create the class. It is needed to resolve
  // fields descriptors, including extensions fields; its C++ MessageFactory is
  // used to instantiate submessages.
  // This reference must stay alive until all message pointers are destructed.
  PyMessageFactory* py_message_factory;

  PyObject* AsPyObject() { return reinterpret_cast<PyObject*>(this); }
};

extern PyTypeObject* CMessageClass_Type;
extern PyTypeObject* CMessage_Type;

namespace cmessage {

// Internal function to create a new empty Message Python object, but with empty
// pointers to the C++ objects.
// The caller must fill self->message, self->owner and eventually self->parent.
CMessage* NewEmptyMessage(CMessageClass* type);

// Retrieves the C++ descriptor of a Python Extension descriptor.
// On error, return NULL with an exception set.
const FieldDescriptor* GetExtensionDescriptor(PyObject* extension);

// Initializes a new CMessage instance for a submessage. Only called once per
// submessage as the result is cached in composite_fields.
//
// Corresponds to reflection api method GetMessage.
CMessage* InternalGetSubMessage(CMessage* self,
                                const FieldDescriptor* field_descriptor);

// Deletes a range of items in a repeated field (following a
// removal in a RepeatedCompositeContainer).
//
// Corresponds to reflection api method RemoveLast.
int DeleteRepeatedField(CMessage* self, const FieldDescriptor* field_descriptor,
                        PyObject* slice);

// Sets the specified scalar value to the message.
int InternalSetScalar(CMessage* self, const FieldDescriptor* field_descriptor,
                      PyObject* value);

// Sets the specified scalar value to the message.  Requires it is not a Oneof.
int InternalSetNonOneofScalar(Message* message,
                              const FieldDescriptor* field_descriptor,
                              PyObject* arg);

// Retrieves the specified scalar value from the message.
//
// Returns a new python reference.
PyObject* InternalGetScalar(const Message* message,
                            const FieldDescriptor* field_descriptor);

bool SetCompositeField(CMessage* self, const FieldDescriptor* field,
                       ContainerBase* value);

bool SetSubmessage(CMessage* self, CMessage* submessage);

// Clears the message, removing all contained data. Extension dictionary and
// submessages are released first if there are remaining external references.
//
// Corresponds to message api method Clear.
PyObject* Clear(CMessage* self);

// Clears the data described by the given descriptor.
// Returns -1 on error.
//
// Corresponds to reflection api method ClearField.
int ClearFieldByDescriptor(CMessage* self, const FieldDescriptor* descriptor);

// Checks if the message has the field described by the descriptor. Used for
// extensions (which have no name).
// Returns 1 if true, 0 if false, and -1 on error.
//
// Corresponds to reflection api method HasField
int HasFieldByDescriptor(CMessage* self,
                         const FieldDescriptor* field_descriptor);

// Checks if the message has the named field.
//
// Corresponds to reflection api method HasField.
PyObject* HasField(CMessage* self, PyObject* arg);

// Initializes values of fields on a newly constructed message.
// Note that positional arguments are disallowed: 'args' must be NULL or the
// empty tuple.
int InitAttributes(CMessage* self, PyObject* args, PyObject* kwargs);

PyObject* MergeFrom(CMessage* self, PyObject* arg);

// Get a field from a message.
PyObject* GetFieldValue(CMessage* self,
                        const FieldDescriptor* field_descriptor);
// Sets the value of a scalar field in a message.
// On error, return -1 with an extension set.
int SetFieldValue(CMessage* self, const FieldDescriptor* field_descriptor,
                  PyObject* value);

PyObject* FindInitializationErrors(CMessage* self);

int AssureWritable(CMessage* self);

// Returns the message factory for the given message.
// This is equivalent to message.MESSAGE_FACTORY
//
// The returned factory is suitable for finding fields and building submessages,
// even in the case of extensions.
// Returns a *borrowed* reference, and never fails because we pass a CMessage.
PyMessageFactory* GetFactoryForMessage(CMessage* message);

PyObject* SetAllowOversizeProtos(PyObject* m, PyObject* arg);

}  // namespace cmessage

/* Is 64bit */
#define IS_64BIT (SIZEOF_LONG == 8)

#define FIELD_IS_REPEATED(field_descriptor) \
  ((field_descriptor)->label() == FieldDescriptor::LABEL_REPEATED)

#define PROTOBUF_CHECK_GET_INT32(arg, value, err) \
  int32_t value;                                  \
  if (!CheckAndGetInteger(arg, &value)) {         \
    return err;                                   \
  }

#define PROTOBUF_CHECK_GET_INT64(arg, value, err) \
  int64_t value;                                  \
  if (!CheckAndGetInteger(arg, &value)) {         \
    return err;                                   \
  }

#define PROTOBUF_CHECK_GET_UINT32(arg, value, err) \
  uint32_t value;                                  \
  if (!CheckAndGetInteger(arg, &value)) {          \
    return err;                                    \
  }

#define PROTOBUF_CHECK_GET_UINT64(arg, value, err) \
  uint64_t value;                                  \
  if (!CheckAndGetInteger(arg, &value)) {          \
    return err;                                    \
  }

#define PROTOBUF_CHECK_GET_FLOAT(arg, value, err) \
  float value;                                    \
  if (!CheckAndGetFloat(arg, &value)) {           \
    return err;                                   \
  }

#define PROTOBUF_CHECK_GET_DOUBLE(arg, value, err) \
  double value;                                    \
  if (!CheckAndGetDouble(arg, &value)) {           \
    return err;                                    \
  }

#define PROTOBUF_CHECK_GET_BOOL(arg, value, err) \
  bool value;                                    \
  if (!CheckAndGetBool(arg, &value)) {           \
    return err;                                  \
  }

#define FULL_MODULE_NAME "google.protobuf.pyext._message"

void FormatTypeError(PyObject* arg, const char* expected_types);
template <class T>
bool CheckAndGetInteger(PyObject* arg, T* value);
bool CheckAndGetDouble(PyObject* arg, double* value);
bool CheckAndGetFloat(PyObject* arg, float* value);
bool CheckAndGetBool(PyObject* arg, bool* value);
PyObject* CheckString(PyObject* arg, const FieldDescriptor* descriptor);
bool CheckAndSetString(PyObject* arg, Message* message,
                       const FieldDescriptor* descriptor,
                       const Reflection* reflection, bool append, int index);
PyObject* ToStringObject(const FieldDescriptor* descriptor,
                         absl::string_view value);

// Check if the passed field descriptor belongs to the given message.
// If not, return false and set a Python exception (a KeyError)
bool CheckFieldBelongsToMessage(const FieldDescriptor* field_descriptor,
                                const Message* message);

extern PyObject* PickleError_class;

PyObject* PyMessage_New(const Descriptor* descriptor,
                        PyObject* py_message_factory);
const Message* PyMessage_GetMessagePointer(PyObject* msg);
Message* PyMessage_GetMutableMessagePointer(PyObject* msg);
PyObject* PyMessage_NewMessageOwnedExternally(Message* message,
                                              PyObject* py_message_factory);

bool InitProto2MessageModule(PyObject* m);

// These are referenced by repeated_scalar_container, and must
// be explicitly instantiated.
extern template bool CheckAndGetInteger<int32_t>(PyObject*, int32_t*);
extern template bool CheckAndGetInteger<int64_t>(PyObject*, int64_t*);
extern template bool CheckAndGetInteger<uint32_t>(PyObject*, uint32_t*);
extern template bool CheckAndGetInteger<uint64_t>(PyObject*, uint64_t*);

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_MESSAGE_H__
