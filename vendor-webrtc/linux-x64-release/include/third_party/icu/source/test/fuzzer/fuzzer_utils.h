// © 2019 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html

#ifndef FUZZER_UTILS_H_
#define FUZZER_UTILS_H_

#include <assert.h>

#include "unicode/locid.h"

struct IcuEnvironment {
  IcuEnvironment() {
    // nothing to initialize yet;
  }
};

const icu::Locale& GetRandomLocale(uint16_t rnd) {
  int32_t num_locales = 0;
  const icu::Locale* locales = icu::Locale::getAvailableLocales(num_locales);
  assert(num_locales > 0);
  return locales[rnd % num_locales];
}

#endif  // FUZZER_UTILS_H_
