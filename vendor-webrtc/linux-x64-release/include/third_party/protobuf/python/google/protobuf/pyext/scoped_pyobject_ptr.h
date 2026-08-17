// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: tibell@google.com (Johan Tibell)

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_SCOPED_PYOBJECT_PTR_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_SCOPED_PYOBJECT_PTR_H__

#define PY_SSIZE_T_CLEAN
#include <Python.h>
namespace google {
namespace protobuf {
namespace python {

// Owns a python object and decrements the reference count on destruction.
// This class is not threadsafe.
template <typename PyObjectStruct>
class ScopedPythonPtr {
 public:
  // Takes the ownership of the specified object to ScopedPythonPtr.
  // The reference count of the specified py_object is not incremented.
  explicit ScopedPythonPtr(PyObjectStruct* py_object = nullptr)
      : ptr_(py_object) {}
  ScopedPythonPtr(const ScopedPythonPtr&) = delete;
  ScopedPythonPtr& operator=(const ScopedPythonPtr&) = delete;

  // If a PyObject is owned, decrement its reference count.
  ~ScopedPythonPtr() { Py_XDECREF(ptr_); }

  // Deletes the current owned object, if any.
  // Then takes ownership of a new object without incrementing the reference
  // count.
  // This function must be called with a reference that you own.
  //   this->reset(this->get()) is wrong!
  //   this->reset(this->release()) is OK.
  PyObjectStruct* reset(PyObjectStruct* p = nullptr) {
    Py_XDECREF(ptr_);
    ptr_ = p;
    return ptr_;
  }

  // Releases ownership of the object without decrementing the reference count.
  // The caller now owns the returned reference.
  PyObjectStruct* release() {
    PyObject* p = ptr_;
    ptr_ = nullptr;
    return p;
  }

  PyObjectStruct* get() const { return ptr_; }

  PyObject* as_pyobject() const { return reinterpret_cast<PyObject*>(ptr_); }

  // Increments the reference count of the current object.
  // Should not be called when no object is held.
  void inc() const { Py_INCREF(ptr_); }

  // True when a ScopedPyObjectPtr and a raw pointer refer to the same object.
  // Comparison operators are non reflexive.
  bool operator==(const PyObjectStruct* p) const { return ptr_ == p; }
  bool operator!=(const PyObjectStruct* p) const { return ptr_ != p; }

 private:
  PyObjectStruct* ptr_;
};

typedef ScopedPythonPtr<PyObject> ScopedPyObjectPtr;

}  // namespace python
}  // namespace protobuf
}  // namespace google
#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_SCOPED_PYOBJECT_PTR_H__
