// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_HYPHENATION_HYPHENATION_MINIKIN_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_HYPHENATION_HYPHENATION_MINIKIN_H_

#include "third_party/blink/renderer/platform/text/hyphenation.h"

#include "base/files/memory_mapped_file.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace base {
class File;
}  // namespace base

namespace android {
class Hyphenator;
}  // namespace andorid

namespace blink {

class PLATFORM_EXPORT HyphenationMinikin final : public Hyphenation {
 public:
  bool OpenDictionary(const AtomicString& locale);

  wtf_size_t LastHyphenLocation(const StringView& text,
                                wtf_size_t before_index) const override;
  Vector<wtf_size_t, 8> HyphenLocations(const StringView&) const override;

  // Extract the word to hyphenate by skipping leading and trailing spaces and
  // punctuations.
  static StringView WordToHyphenate(const StringView& text,
                                    unsigned* num_leading_chars_out);

  static AtomicString MapLocale(const AtomicString& locale);

  static scoped_refptr<HyphenationMinikin> FromFileForTesting(
      const AtomicString& locale,
      base::File);

 private:
  bool OpenDictionary(base::File);

  Vector<uint8_t> Hyphenate(const StringView&) const;

  base::MemoryMappedFile file_;
  std::unique_ptr<android::Hyphenator> hyphenator_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_HYPHENATION_HYPHENATION_MINIKIN_H_
