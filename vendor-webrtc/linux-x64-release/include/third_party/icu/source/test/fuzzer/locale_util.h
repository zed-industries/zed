// © 2019 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html

#ifndef I18N_ICU_FUZZ_LOCALE_UTIL_H_
#define I18N_ICU_FUZZ_LOCALE_UTIL_H_

#include <cstdint>
#include <string>

// Takes uint8_t data from fuzzer, and makes a zero terminated string.
std::string MakeZeroTerminatedInput(const uint8_t* data, int32_t size);

#endif  // I18N_ICU_FUZZ_LOCALE_UTIL_H_
