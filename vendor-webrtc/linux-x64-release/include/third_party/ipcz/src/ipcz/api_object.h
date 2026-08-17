// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_API_OBJECT_H_
#define IPCZ_SRC_IPCZ_API_OBJECT_H_

#include "ipcz/ipcz.h"
#include "third_party/abseil-cpp/absl/base/macros.h"
#include "util/ref_counted.h"

namespace ipcz {

class Router;

// Base class for any object which can be referenced by an IpczHandle.
//
// A subclass T should inherit from APIObjectImpl<T, U> rather than inheriting
// this base class directly. See APIObjectImpl below.
class APIObject : public RefCounted<APIObject> {
 public:
  enum ObjectType {
    kNode,
    kPortal,
    kBox,
    kTransportListener,
    kParcel,
  };

  explicit APIObject(ObjectType type);

  ObjectType object_type() const { return type_; }

  static APIObject* FromHandle(IpczHandle handle) {
    return reinterpret_cast<APIObject*>(static_cast<uintptr_t>(handle));
  }

  // Takes ownership of an APIObject from an existing `handle`.
  static Ref<APIObject> TakeFromHandle(IpczHandle handle) {
    return AdoptRef(
        reinterpret_cast<APIObject*>(static_cast<uintptr_t>(handle)));
  }

  // Returns an IpczHandle which can be used to reference this object. The
  // reference is not owned by the caller.
  IpczHandle handle() const { return reinterpret_cast<uintptr_t>(this); }

  // Releases ownership of a Ref<APIObject> to produce a new IpczHandle which
  // implicilty owns the released reference.
  static IpczHandle ReleaseAsHandle(Ref<APIObject> object) {
    return static_cast<IpczHandle>(
        reinterpret_cast<uintptr_t>(object.release()));
  }

  // Closes this underlying object, ceasing its operations and freeing its
  // resources ASAP.
  virtual IpczResult Close() = 0;

  // Indicates whether it's possible to send this object from `sender`. By
  // default the answer is NO.
  virtual bool CanSendFrom(Router& sender);

 protected:
  friend class RefCounted<APIObject>;

  virtual ~APIObject();

  const ObjectType type_;
};

// Strongly-typed base class for any object which can be referenced by an
// IpczHandle. This is templated over the more specific subclass type, as well
// as an appropriate ObjectType value to use for runtime type idenitification.
template <typename T, APIObject::ObjectType kType>
class APIObjectImpl : public APIObject {
 public:
  constexpr APIObjectImpl() : APIObject(kType) {}

  static T* FromObject(APIObject* object) {
    if (object && object->object_type() == kType) {
      return static_cast<T*>(object);
    }
    return nullptr;
  }

  static T* FromHandle(IpczHandle handle) {
    return FromObject(APIObject::FromHandle(handle));
  }

  // Same as TakeFromHandle() but with type checking.
  static Ref<T> TakeFromHandle(IpczHandle handle) {
    return AdoptRef(FromHandle(handle));
  }

 protected:
  ~APIObjectImpl() override = default;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_API_OBJECT_H_
