/*
 * Copyright (c) 2009, Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_FACE_CREATION_PARAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_FACE_CREATION_PARAMS_H_

#include "base/check_op.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/case_folding_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hasher.h"

namespace blink {

enum FontFaceCreationType {
  kCreateFontByFamily,
  kCreateFontByFciIdAndTtcIndex
};

class FontFaceCreationParams {
  USING_FAST_MALLOC(FontFaceCreationParams);

 public:
  FontFaceCreationParams() : creation_type_(kCreateFontByFamily) {}

  explicit FontFaceCreationParams(AtomicString family)
      : creation_type_(kCreateFontByFamily), family_(family) {
#if BUILDFLAG(IS_WIN)
    // Leading "@" in the font name enables Windows vertical flow flag for the
    // font.  Because we do vertical flow by ourselves, we don't want to use the
    // Windows feature.  IE disregards "@" regardless of the orientation, so we
    // follow the behavior and normalize the family name.
    family_ = (family_.empty() || family_[0] != '@')
                  ? family_
                  : AtomicString(family_.Impl()->Substring(1));
#endif
  }

  FontFaceCreationParams(const std::string& filename,
                         int fontconfig_interface_id,
                         int ttc_index = 0)
      : creation_type_(kCreateFontByFciIdAndTtcIndex),
        filename_(filename),
        fontconfig_interface_id_(fontconfig_interface_id),
        ttc_index_(ttc_index) {}

  FontFaceCreationType CreationType() const { return creation_type_; }
  const AtomicString& Family() const {
    DCHECK_EQ(creation_type_, kCreateFontByFamily);
    return family_;
  }
  const std::string& Filename() const {
    DCHECK_EQ(creation_type_, kCreateFontByFciIdAndTtcIndex);
#if defined(ADDRESS_SANITIZER)
    DCHECK(filename_.has_value());
    return *filename_;
#else
    return filename_;
#endif
  }

  int FontconfigInterfaceId() const {
    DCHECK_EQ(creation_type_, kCreateFontByFciIdAndTtcIndex);
    return fontconfig_interface_id_;
  }
  int TtcIndex() const {
    DCHECK_EQ(creation_type_, kCreateFontByFciIdAndTtcIndex);
    return ttc_index_;
  }

  unsigned GetHash() const {
    if (creation_type_ == kCreateFontByFciIdAndTtcIndex) {
      // Hashing the filename and ints in this way is sensitive to character
      // encoding and endianness. However, since the hash is not transferred
      // over a network or permanently stored and only used for the runtime of
      // Chromium, this is not a concern.
      struct HashData {
        int index;
        int id;
        uint64_t filename_hash;
      } hash_data = {ttc_index_, fontconfig_interface_id_,
                     HasFilename() ? StringHasher::HashMemory(
                                         base::as_byte_span(Filename()))
                                   : 0};
      return StringHasher::HashMemory(base::byte_span_from_ref(hash_data));
    }
    return CaseFoldingHash::GetHash(family_.empty() ? g_empty_atom : family_);
  }

  bool operator==(const FontFaceCreationParams& other) const {
    return creation_type_ == other.creation_type_ &&
           DeprecatedEqualIgnoringCase(family_, other.family_) &&
           FilenameEqual(other) &&
           fontconfig_interface_id_ == other.fontconfig_interface_id_ &&
           ttc_index_ == other.ttc_index_;
  }

 private:
  FontFaceCreationType creation_type_;
  AtomicString family_;

  void SetFilename(std::string& filename) {
#if defined(ADDRESS_SANITIZER)
    *filename_ = filename;
#else
    filename_ = filename;
#endif
  }

  bool FilenameEqual(const FontFaceCreationParams& other) const {
#if defined(ADDRESS_SANITIZER)
    if (!filename_.has_value() || !other.filename_.has_value()) {
      return filename_.has_value() == other.filename_.has_value();
    }
    return *filename_ == *other.filename_;
#else
    return filename_ == other.filename_;
#endif
  }

  bool HasFilename() const {
#if defined(ADDRESS_SANITIZER)
    return filename_.has_value();
#else
    return true;
#endif
  }

#if defined(ADDRESS_SANITIZER)
  // We put the `std::string` behind an optional as ASAN counter checks require
  // that we properly call constructors and destructors for all strings. This is
  // not the case when `FontFaceCreationParams` is used in `WTF::HashMap` as key
  // where we also cosntruct empty and deleted values that are never properly
  // destroyed.
  //
  // See crbug.com/346174906.
  std::optional<std::string> filename_;
#else
  std::string filename_;
#endif
  int fontconfig_interface_id_ = 0;
  int ttc_index_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_FACE_CREATION_PARAMS_H_
