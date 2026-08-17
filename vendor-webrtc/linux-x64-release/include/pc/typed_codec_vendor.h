/*
 *  Copyright 2025 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TYPED_CODEC_VENDOR_H_
#define PC_TYPED_CODEC_VENDOR_H_

#include "api/field_trials_view.h"
#include "api/media_types.h"
#include "media/base/codec_list.h"
#include "media/base/media_engine.h"

namespace webrtc {

// This class vends codecs of a specific type only.
// It is intended to eventually be owned by the RtpSender and RtpReceiver
// objects.
class TypedCodecVendor {
 public:
  // Constructor for the case where media engine is not provided. The resulting
  // vendor will always return an empty codec list.
  TypedCodecVendor() {}
  TypedCodecVendor(MediaEngineInterface* media_engine,
                   MediaType type,
                   bool is_sender,
                   bool rtx_enabled,
                   const FieldTrialsView& trials);
  const CodecList& codecs() const { return codecs_; }
  void set_codecs(const CodecList& codecs) { codecs_ = codecs; }
  // For easy initialization, copying is allowed.
  TypedCodecVendor(const TypedCodecVendor& from) = default;
  TypedCodecVendor& operator=(const TypedCodecVendor& from) = default;

 private:
  CodecList codecs_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::TypedCodecVendor;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_TYPED_CODEC_VENDOR_H_
