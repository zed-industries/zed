// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_REFERENCE_DRIVERS_OBJECT_H_
#define IPCZ_SRC_REFERENCE_DRIVERS_OBJECT_H_

#include <cstdint>

#include "build/build_config.h"
#include "ipcz/ipcz.h"
#include "util/ref_counted.h"

namespace ipcz::reference_drivers {

// Base class for all driver-managed objects used by both reference drivers.
class Object : public RefCounted<Object> {
 public:
  enum Type : uint32_t {
    kTransport,
    kMemory,
    kMapping,

#if defined(OS_LINUX)
    // A non-standard driver object type which wraps a FileDescriptor object.
    kFileDescriptor,
#endif
  };

  explicit Object(Type type);

  Type type() const { return type_; }

  IpczDriverHandle handle() const {
    return reinterpret_cast<IpczDriverHandle>(this);
  }

  static Object* FromHandle(IpczDriverHandle handle) {
    return reinterpret_cast<Object*>(handle);
  }

  static IpczDriverHandle ReleaseAsHandle(Ref<Object> object) {
    return reinterpret_cast<IpczDriverHandle>(object.release());
  }

  static Ref<Object> TakeFromHandle(IpczDriverHandle handle) {
    return AdoptRef(FromHandle(handle));
  }

  virtual IpczResult Close();

 protected:
  virtual ~Object();

 private:
  friend class RefCounted<Object>;

  const Type type_;
};

template <typename T, Object::Type kType>
class ObjectImpl : public Object {
 public:
  ObjectImpl() : Object(kType) {}

  static T* FromObject(Object* object) {
    if (!object || object->type() != kType) {
      return nullptr;
    }
    return static_cast<T*>(object);
  }

  static T* FromHandle(IpczDriverHandle handle) {
    return FromObject(Object::FromHandle(handle));
  }

  static Ref<T> TakeFromObject(Object* object) {
    return AdoptRef(FromObject(object));
  }

  static Ref<T> TakeFromHandle(IpczDriverHandle handle) {
    return AdoptRef(FromHandle(handle));
  }

 protected:
  virtual ~ObjectImpl() = default;
};

}  // namespace ipcz::reference_drivers

#endif  // IPCZ_SRC_REFERENCE_DRIVERS_OBJECT_H_
