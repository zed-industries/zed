// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_INITIAL_LETTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_INITIAL_LETTER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT StyleInitialLetter {
  DISALLOW_NEW();

 public:
  StyleInitialLetter();
  explicit StyleInitialLetter(float size);
  StyleInitialLetter(float size, int sink);

  bool operator==(const StyleInitialLetter& other) const;
  bool operator!=(const StyleInitialLetter& other) const;

  bool IsDrop() const { return sink_type_ == kDrop; }
  bool IsIntegerSink() const { return sink_type_ == kInteger; }
  bool IsNormal() const { return !size_; }
  bool IsRaise() const { return sink_type_ == kRaise; }

  int Sink() const { return sink_; }
  float Size() const { return size_; }

  static StyleInitialLetter Normal() { return StyleInitialLetter(); }
  static StyleInitialLetter Drop(float size);
  static StyleInitialLetter Raise(float size);

 private:
  enum SinkType {
    kNone,
    kOmitted,
    kInteger,
    kDrop,
    kRaise,
  };

  StyleInitialLetter(float size, SinkType sink_type);

  float size_ = 0;
  int sink_ = 0;
  SinkType sink_type_ = kNone;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_INITIAL_LETTER_H_
