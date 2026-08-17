/* Copyright 2014 Google Inc. All Rights Reserved.

   Distributed under MIT license.
   See file LICENSE for detail or copy at https://opensource.org/licenses/MIT
*/

/* Library for converting WOFF2 format font files to their TTF versions. */

#ifndef WOFF2_WOFF2_DEC_H_
#define WOFF2_WOFF2_DEC_H_

#include <stddef.h>
#include <inttypes.h>
#include <woff2/output.h>

namespace woff2 {

// Compute the size of the final uncompressed font, or 0 on error.
size_t ComputeWOFF2FinalSize(const uint8_t *data, size_t length);

// Decompresses the font into the target buffer. The result_length should
// be the same as determined by ComputeFinalSize(). Returns true on successful
// decompression.
// DEPRECATED; please prefer the version that takes a WOFF2Out*
bool ConvertWOFF2ToTTF(uint8_t *result, size_t result_length,
                       const uint8_t *data, size_t length);

// Decompresses the font into out. Returns true on success.
// Works even if WOFF2Header totalSfntSize is wrong.
// Please prefer this API.
bool ConvertWOFF2ToTTF(const uint8_t *data, size_t length,
                       WOFF2Out* out);

} // namespace woff2

#endif  // WOFF2_WOFF2_DEC_H_
