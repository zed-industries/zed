/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_STUN_DICTIONARY_H_
#define P2P_BASE_STUN_DICTIONARY_H_

#include <cstddef>
#include <cstdint>
#include <deque>
#include <map>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "api/rtc_error.h"
#include "api/transport/stun.h"

namespace webrtc {

// A StunDictionaryView is a dictionary of StunAttributes.
//   - the StunAttributes can be read using the |Get|-methods.
//   - the dictionary is updated by using the |ApplyDelta|-method.
//
// A StunDictionaryWriter is used to create |delta|s for the |ApplyDelta|-method
// - It keeps track of which updates has been applied at StunDictionaryView.
// - It optionally keeps a local StunDictionaryView contains modification made
// `locally`
//
// A pair StunDictionaryView(A)/StunDictionaryWriter(B) are linked so that
// modifications to B is transfered to A using the STUN_ATTR_GOOG_DELTA
// (StunByteStringAttribute) and the modification is ack:ed using
// STUN_ATTR_GOOG_DELTA_ACK (StunUInt64Attribute).
//
// Note:
// 1) It is possible to update one StunDictionaryView from multiple writers,
// but this only works of the different writers write disjoint keys (which
// is not checked/enforced by these classes).
// 2) The opposite, one writer updating multiple StunDictionaryView, is not
// possible.
class StunDictionaryView {
 public:
  // A reserved key used to transport the version number
  static constexpr uint16_t kVersionKey = 0xFFFF;

  // A magic number used when transporting deltas.
  static constexpr uint16_t kDeltaMagic = 0x7788;

  // The version number for the delta format.
  static constexpr uint16_t kDeltaVersion = 0x1;

  // Gets the desired attribute value, or NULL if no such attribute type exists.
  // The pointer returned is guaranteed to be valid until ApplyDelta is called.
  const StunAddressAttribute* GetAddress(int key) const;
  const StunUInt32Attribute* GetUInt32(int key) const;
  const StunUInt64Attribute* GetUInt64(int key) const;
  const StunByteStringAttribute* GetByteString(int key) const;
  const StunUInt16ListAttribute* GetUInt16List(int key) const;

  bool empty() const { return attrs_.empty(); }
  size_t size() const { return attrs_.size(); }
  int bytes_stored() const { return bytes_stored_; }
  void set_max_bytes_stored(int max_bytes_stored) {
    max_bytes_stored_ = max_bytes_stored;
  }

  // Apply a delta and return
  // a pair with
  // - StunUInt64Attribute to ack the |delta|.
  // - vector of keys that was modified.
  RTCErrorOr<
      std::pair<std::unique_ptr<StunUInt64Attribute>, std::vector<uint16_t>>>
  ApplyDelta(const StunByteStringAttribute& delta);

 private:
  friend class StunDictionaryWriter;

  const StunAttribute* GetOrNull(
      int key,
      std::optional<StunAttributeValueType> = std::nullopt) const;
  size_t GetLength(int key) const;
  static RTCErrorOr<
      std::pair<uint64_t, std::deque<std::unique_ptr<StunAttribute>>>>
  ParseDelta(const StunByteStringAttribute& delta);

  std::map<uint16_t, std::unique_ptr<StunAttribute>> attrs_;
  std::map<uint16_t, uint64_t> version_per_key_;

  int max_bytes_stored_ = 16384;
  int bytes_stored_ = 0;
};

class StunDictionaryWriter {
 public:
  StunDictionaryWriter() {
    dictionary_ = std::make_unique<StunDictionaryView>();
  }
  explicit StunDictionaryWriter(
      std::unique_ptr<StunDictionaryView> dictionary) {
    dictionary_ = std::move(dictionary);
  }

  // A pending modification.
  template <typename T>
  class Modification {
   public:
    ~Modification() { commit(); }

    T* operator->() { return attr_.get(); }

    void abort() { attr_ = nullptr; }
    void commit() {
      if (attr_) {
        writer_->Set(std::move(attr_));
      }
    }

   private:
    friend class StunDictionaryWriter;
    Modification(StunDictionaryWriter* writer, std::unique_ptr<T> attr)
        : writer_(writer), attr_(std::move(attr)) {}
    StunDictionaryWriter* writer_;
    std::unique_ptr<T> attr_;

    Modification(const Modification<T>&) =
        delete;  // not copyable (but movable).
    Modification& operator=(Modification<T>&) =
        delete;  // not copyable (but movable).
  };

  // Record a modification.
  Modification<StunAddressAttribute> SetAddress(int key) {
    return Modification<StunAddressAttribute>(
        this, StunAttribute::CreateAddress(key));
  }
  Modification<StunUInt32Attribute> SetUInt32(int key) {
    return Modification<StunUInt32Attribute>(this,
                                             StunAttribute::CreateUInt32(key));
  }
  Modification<StunUInt64Attribute> SetUInt64(int key) {
    return Modification<StunUInt64Attribute>(this,
                                             StunAttribute::CreateUInt64(key));
  }
  Modification<StunByteStringAttribute> SetByteString(int key) {
    return Modification<StunByteStringAttribute>(
        this, StunAttribute::CreateByteString(key));
  }
  Modification<StunUInt16ListAttribute> SetUInt16List(int key) {
    return Modification<StunUInt16ListAttribute>(
        this, StunAttribute::CreateUInt16ListAttribute(key));
  }

  // Delete a key.
  void Delete(int key);

  // Check if a key has a pending change (i.e a change
  // that has not been acked).
  bool Pending(int key) const;

  // Return number of of pending modifications.
  int Pending() const;

  // Create an StunByteStringAttribute containing the pending (e.g not ack:ed)
  // modifications.
  std::unique_ptr<StunByteStringAttribute> CreateDelta();

  // Apply an delta ack.
  void ApplyDeltaAck(const StunUInt64Attribute&);

  // Return pointer to (optional) StunDictionaryView.
  const StunDictionaryView* dictionary() { return dictionary_.get(); }
  const StunDictionaryView* operator->() { return dictionary_.get(); }

  // Disable writer,
  // i.e CreateDelta always return null, and no modifications are made.
  // This is called if remote peer does not support GOOG_DELTA.
  void Disable();
  bool disabled() const { return disabled_; }

 private:
  void Set(std::unique_ptr<StunAttribute> attr);

  bool disabled_ = false;

  // version of modification.
  int64_t version_ = 1;

  // (optional) StunDictionaryView.
  std::unique_ptr<StunDictionaryView> dictionary_;

  // sorted list of changes that has not been yet been ack:ed.
  std::vector<std::pair<uint64_t, const StunAttribute*>> pending_;

  // tombstones, i.e values that has been deleted but not yet acked.
  std::map<uint16_t, std::unique_ptr<StunAttribute>> tombstones_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::StunDictionaryView;
using ::webrtc::StunDictionaryWriter;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_STUN_DICTIONARY_H_
