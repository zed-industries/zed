/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_CODEC_LIST_H_
#define MEDIA_BASE_CODEC_LIST_H_

#include <cstddef>
#include <vector>

#include "api/rtc_error.h"
#include "media/base/codec.h"

namespace webrtc {

class CodecList {
 public:
  using iterator = std::vector<Codec>::iterator;
  using const_iterator = std::vector<Codec>::const_iterator;
  using value_type = Codec;

  CodecList() = default;
  // Copy and assign are available.
  CodecList(const CodecList&) = default;
  CodecList& operator=(const CodecList&) = default;
  CodecList(CodecList&&) = default;
  CodecList& operator=(CodecList&&) = default;
  bool operator==(const CodecList& o) const { return codecs_ == o.codecs_; }

  // Creates a codec list on untrusted data. If successful, the
  // resulting CodecList satisfies all the CodecList invariants.
  static RTCErrorOr<CodecList> Create(const std::vector<Codec>& codecs);
  // Creates a codec list on trusted data. Only for use when
  // the codec list is generated from internal code.
  static CodecList CreateFromTrustedData(const std::vector<Codec>& codecs) {
    return CodecList(codecs);
  }
  // Vector-compatible API to access the codecs.
  iterator begin() { return codecs_.begin(); }
  iterator end() { return codecs_.end(); }
  const_iterator begin() const { return codecs_.begin(); }
  const_iterator end() const { return codecs_.end(); }
  const Codec& operator[](size_t i) const { return codecs_[i]; }
  Codec& operator[](size_t i) { return codecs_[i]; }
  void push_back(const Codec& codec) {
    codecs_.push_back(codec);
    CheckConsistency();
  }
  bool empty() const { return codecs_.empty(); }
  void clear() { codecs_.clear(); }
  size_t size() const { return codecs_.size(); }
  // Access to the whole codec list
  const std::vector<Codec>& codecs() const { return codecs_; }
  std::vector<Codec>& writable_codecs() { return codecs_; }
  // Verify consistency of the codec list.
  // Examples: checking that all RTX codecs have APT pointing
  // to a codec in the list.
  // The function will CHECK or DCHECK on inconsistencies.
  void CheckConsistency();

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const CodecList& list) {
    absl::Format(&sink, "\n--- Codec list of size %d\n", list.size());
    for (Codec codec : list) {
      absl::Format(&sink, "%v\n", codec);
    }
    sink.Append("--- End\n");
  }

 private:
  // Creates a codec list on trusted data.
  explicit CodecList(const std::vector<Codec>& codecs) {
    codecs_ = codecs;
    CheckConsistency();
  }

  std::vector<Codec> codecs_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::CodecList;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_CODEC_LIST_H_
