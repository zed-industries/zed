// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_I18N_TRANSLITERATOR_H_
#define BASE_I18N_TRANSLITERATOR_H_

#include <stddef.h>

#include <memory>
#include <string>
#include <string_view>

#include "base/i18n/base_i18n_export.h"
#include "base/memory/raw_ptr.h"

// The Transliterator class transliterate a string.

namespace base {
namespace i18n {

class BASE_I18N_EXPORT Transliterator {
 public:
  virtual ~Transliterator() = default;
  virtual std::u16string Transliterate(std::u16string_view text) const = 0;
};

BASE_I18N_EXPORT std::unique_ptr<Transliterator> CreateTransliterator(
    std::string_view id);
BASE_I18N_EXPORT std::unique_ptr<Transliterator> CreateTransliteratorFromRules(
    std::string_view id,
    std::string_view rules);

}  // namespace i18n
}  // namespace base

#endif  // BASE_I18N_TRANSLITERATOR_H_
