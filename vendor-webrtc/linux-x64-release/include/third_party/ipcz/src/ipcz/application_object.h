// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_APPLICATION_OBJECT_H_
#define IPCZ_SRC_IPCZ_APPLICATION_OBJECT_H_

#include <cstddef>
#include <cstdint>
#include <optional>

#include "ipcz/ipcz.h"
#include "ipcz/parcel_wrapper.h"
#include "third_party/abseil-cpp/absl/types/span.h"
#include "util/ref_counted.h"

namespace ipcz {

class NodeLink;

// ApplicationObject encapsulates an opaque object identifier along with
// serialization and destruction functions, as provided by an application via
// the Box() API. This serves as a thin wrapper to model strong owenership of
// the opaque object within ipcz.
class ApplicationObject {
 public:
  // Constructs a new ApplicationObject to own the opaque `object`. Upon
  // destruction this object will invoke `destructor` on `object`; and if ipcz
  // needs to transmit this object to another node, it will attempt to serialize
  // the object using `serializer`.
  ApplicationObject(uintptr_t object,
                    IpczApplicationObjectSerializer serializer,
                    IpczApplicationObjectDestructor destructor);

  // NOTE: A moved-from ApplicationObject is invalid and must not be used to
  // call other methods.
  ApplicationObject(ApplicationObject&&);
  ~ApplicationObject();

  uintptr_t object() const { return *object_; }
  IpczApplicationObjectSerializer serializer() const { return serializer_; }
  IpczApplicationObjectDestructor destructor() const { return destructor_; }

  // Resets this object, invoking its custom destructor if necessary. This
  // invalidates the ApplicationObject.
  void reset();

  // Releases ownership of the application object and returns it. This
  // invalidates the ApplicationObject and insures that neither resetting nor
  // destroying the ApplicationObject will invoke the released object's custom
  // destructor.
  uintptr_t ReleaseObject();

  // Indicates whether this object can be serialized by ipcz.
  bool IsSerializable() const;

  // Serializes this object into a new Parcel to be transmitted across `link`.
  // Returns the wrapped new parcel, or null on failure. Note that this does
  // *not* invoke the object's destructor, which is still slated for invocation
  // when the ApplicationObject is reset or destroyed.
  //
  // Must only be called on objects which are known to be serializable.
  Ref<ParcelWrapper> Serialize(NodeLink& link);

 private:
  // Null iff this ApplicationObject has been moved-from.
  std::optional<uintptr_t> object_;
  const IpczApplicationObjectSerializer serializer_;
  const IpczApplicationObjectDestructor destructor_;
};

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_APPLICATION_OBJECT_H_
