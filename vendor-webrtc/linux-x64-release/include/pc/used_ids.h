/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef PC_USED_IDS_H_
#define PC_USED_IDS_H_

#include <set>
#include <vector>

#include "api/rtp_parameters.h"
#include "media/base/codec.h"
#include "rtc_base/checks.h"
#include "rtc_base/logging.h"

namespace webrtc {
template <typename IdStruct>
class UsedIds {
 public:
  UsedIds(int min_allowed_id, int max_allowed_id)
      : min_allowed_id_(min_allowed_id),
        max_allowed_id_(max_allowed_id),
        next_id_(max_allowed_id) {}
  virtual ~UsedIds() {}

  // Loops through all Id in `ids` and changes its id if it is
  // already in use by another IdStruct. Call this methods with all Id
  // in a session description to make sure no duplicate ids exists.
  // Note that typename Id must be a type of IdStruct.
  template <typename Id>
  void FindAndSetIdUsed(std::vector<Id>* ids) {
    for (const Id& id : *ids) {
      FindAndSetIdUsed(&id);
    }
  }

  // Finds and sets an unused id if the `idstruct` id is already in use.
  void FindAndSetIdUsed(IdStruct* idstruct) {
    const int original_id = idstruct->id;
    int new_id = idstruct->id;

    if (original_id > max_allowed_id_ || original_id < min_allowed_id_) {
      // If the original id is not in range - this is an id that can't be
      // dynamically changed.
      return;
    }

    if (IsIdUsed(original_id)) {
      new_id = FindUnusedId();
      // Duplicate id found. Reassign from the original id to the new.
      idstruct->id = new_id;
    }
    SetIdUsed(new_id);
  }

 protected:
  virtual bool IsIdUsed(int new_id) {
    return id_set_.find(new_id) != id_set_.end();
  }
  const int min_allowed_id_;
  const int max_allowed_id_;

 private:
  // Returns the first unused id in reverse order.
  // This hopefully reduces the risk of more collisions. We want to change the
  // default ids as little as possible. This function is virtual and can be
  // overriden if the search for unused IDs should follow a specific pattern.
  virtual int FindUnusedId() {
    while (IsIdUsed(next_id_) && next_id_ >= min_allowed_id_) {
      --next_id_;
    }
    RTC_DCHECK(next_id_ >= min_allowed_id_);
    return next_id_;
  }

  void SetIdUsed(int new_id) {
    RTC_DCHECK(new_id >= min_allowed_id_);
    RTC_DCHECK(new_id <= max_allowed_id_);
    RTC_DCHECK(!IsIdUsed(new_id));
    id_set_.insert(new_id);
  }
  int next_id_;
  std::set<int> id_set_;
};

// Helper class used for finding duplicate RTP payload types among audio, video
// and data codecs. When bundle is used the payload types may not collide.
class UsedPayloadTypes : public UsedIds<Codec> {
 public:
  UsedPayloadTypes()
      : UsedIds<Codec>(kFirstDynamicPayloadTypeLowerRange,
                       kLastDynamicPayloadTypeUpperRange) {}

  // Check if a payload type is valid. The range [64-95] is forbidden
  // when rtcp-mux is used.
  static bool IsIdValid(Codec codec, bool rtcp_mux) {
    if (rtcp_mux && (codec.id > kLastDynamicPayloadTypeLowerRange &&
                     codec.id < kFirstDynamicPayloadTypeUpperRange)) {
      return false;
    }
    return codec.id >= 0 && codec.id <= kLastDynamicPayloadTypeUpperRange;
  }

 protected:
  bool IsIdUsed(int new_id) override {
    // Range marked for RTCP avoidance is "used".
    if (new_id > kLastDynamicPayloadTypeLowerRange &&
        new_id < kFirstDynamicPayloadTypeUpperRange)
      return true;
    return UsedIds<Codec>::IsIdUsed(new_id);
  }

 private:
  static const int kFirstDynamicPayloadTypeLowerRange = 35;
  static const int kLastDynamicPayloadTypeLowerRange = 63;

  static const int kFirstDynamicPayloadTypeUpperRange = 96;
  static const int kLastDynamicPayloadTypeUpperRange = 127;
};

// Helper class used for finding duplicate RTP Header extension ids among
// audio and video extensions.
class UsedRtpHeaderExtensionIds : public UsedIds<RtpExtension> {
 public:
  enum class IdDomain {
    // Only allocate IDs that fit in one-byte header extensions.
    kOneByteOnly,
    // Prefer to allocate one-byte header extension IDs, but overflow to
    // two-byte if none are left.
    kTwoByteAllowed,
  };

  explicit UsedRtpHeaderExtensionIds(IdDomain id_domain)
      : UsedIds<RtpExtension>(RtpExtension::kMinId,
                              id_domain == IdDomain::kTwoByteAllowed
                                  ? RtpExtension::kMaxId
                                  : RtpExtension::kOneByteHeaderExtensionMaxId),
        id_domain_(id_domain),
        next_extension_id_(RtpExtension::kOneByteHeaderExtensionMaxId) {}

 private:
  // Returns the first unused id in reverse order from the max id of one byte
  // header extensions. This hopefully reduces the risk of more collisions. We
  // want to change the default ids as little as possible. If no unused id is
  // found and two byte header extensions are enabled (i.e.,
  // `extmap_allow_mixed_` is true), search for unused ids from 16 to 255.
  int FindUnusedId() override {
    if (next_extension_id_ <= RtpExtension::kOneByteHeaderExtensionMaxId) {
      // First search in reverse order from the max id of one byte header
      // extensions (14).
      while (IsIdUsed(next_extension_id_) &&
             next_extension_id_ >= min_allowed_id_) {
        --next_extension_id_;
      }
    }

    if (id_domain_ == IdDomain::kTwoByteAllowed) {
      if (next_extension_id_ < min_allowed_id_) {
        // We have searched among all one-byte IDs without finding an unused ID,
        // continue at the first two-byte ID (16; avoid 15 since it is somewhat
        // special per https://www.rfc-editor.org/rfc/rfc8285#section-4.2
        next_extension_id_ = RtpExtension::kOneByteHeaderExtensionMaxId + 2;
      }

      if (next_extension_id_ > RtpExtension::kOneByteHeaderExtensionMaxId) {
        while (IsIdUsed(next_extension_id_) &&
               next_extension_id_ <= max_allowed_id_) {
          ++next_extension_id_;
        }
      }
    }
    RTC_DCHECK(next_extension_id_ >= min_allowed_id_);
    RTC_DCHECK(next_extension_id_ <= max_allowed_id_);
    return next_extension_id_;
  }

  const IdDomain id_domain_;
  int next_extension_id_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::UsedIds;
using ::webrtc::UsedPayloadTypes;
using ::webrtc::UsedRtpHeaderExtensionIds;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_USED_IDS_H_
