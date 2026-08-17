#ifndef THIRD_PARTY_UTF8_RANGE_UTF8_RANGE_H_
#define THIRD_PARTY_UTF8_RANGE_UTF8_RANGE_H_

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Returns 1 if the sequence of characters is a valid UTF-8 sequence, otherwise
// 0.
int utf8_range_IsValid(const char* data, size_t len);

// Returns the length in bytes of the prefix of str that is all
// structurally valid UTF-8.
size_t utf8_range_ValidPrefix(const char* data, size_t len);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // THIRD_PARTY_UTF8_RANGE_UTF8_RANGE_H_
