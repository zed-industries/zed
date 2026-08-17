/*
 * Copyright (C) 2013 Apple Inc. All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_SRCSET_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_SRCSET_PARSER_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Document;

constexpr int kUninitializedDescriptor = -1;

class DescriptorParsingResult {
  STACK_ALLOCATED();

 public:
  DescriptorParsingResult()
      : density_(kUninitializedDescriptor),
        resource_width_(kUninitializedDescriptor),
        resource_height_(kUninitializedDescriptor) {}

  bool HasDensity() const { return density_ >= 0; }
  bool HasWidth() const { return resource_width_ >= 0; }
  bool HasHeight() const { return resource_height_ >= 0; }

  float Density() const {
    DCHECK(HasDensity());
    return density_;
  }
  unsigned GetResourceWidth() const {
    DCHECK(HasWidth());
    return resource_width_;
  }
  unsigned ResourceHeight() const {
    DCHECK(HasHeight());
    return resource_height_;
  }

  void SetResourceWidth(int width) {
    DCHECK_GE(width, 0);
    resource_width_ = (unsigned)width;
  }
  void SetResourceHeight(int height) {
    DCHECK_GE(height, 0);
    resource_height_ = (unsigned)height;
  }
  void SetDensity(float density_to_set) {
    DCHECK_GE(density_to_set, 0);
    density_ = density_to_set;
  }

 private:
  float density_;
  int resource_width_;
  int resource_height_;
};

class ImageCandidate {
  DISALLOW_NEW();

 public:
  enum OriginAttribute { kSrcsetOrigin, kSrcOrigin };

  ImageCandidate()
      : density_(1.0),
        resource_width_(kUninitializedDescriptor),
        origin_attribute_(kSrcsetOrigin) {}

  ImageCandidate(const String& source,
                 unsigned start,
                 unsigned length,
                 const DescriptorParsingResult& result,
                 OriginAttribute origin_attribute)
      : source_(source),
        string_(source, start, length),
        density_(result.HasDensity() ? result.Density()
                                     : kUninitializedDescriptor),
        resource_width_(result.HasWidth() ? result.GetResourceWidth()
                                          : kUninitializedDescriptor),
        origin_attribute_(origin_attribute) {}

  String ToString() const { return string_.ToString(); }

  AtomicString Url() const { return AtomicString(ToString()); }

  void SetDensity(float factor) { density_ = factor; }

  float Density() const { return density_; }

  int GetResourceWidth() const { return resource_width_; }

  bool SrcOrigin() const { return (origin_attribute_ == kSrcOrigin); }

  inline bool IsEmpty() const { return string_.empty(); }

 private:
  String source_;  // Keep the StringView buffer alive.
  StringView string_;
  float density_;
  int resource_width_;
  OriginAttribute origin_attribute_;
};

ImageCandidate BestFitSourceForSrcsetAttribute(float device_scale_factor,
                                               float source_size,
                                               const String& srcset_attribute,
                                               Document* = nullptr);

CORE_EXPORT ImageCandidate
BestFitSourceForImageAttributes(float device_scale_factor,
                                float source_size,
                                const String& src_attribute,
                                const String& srcset_attribute,
                                Document* = nullptr);

String BestFitSourceForImageAttributes(float device_scale_factor,
                                       float source_size,
                                       const String& src_attribute,
                                       ImageCandidate& srcset_image_candidate);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_SRCSET_PARSER_H_
