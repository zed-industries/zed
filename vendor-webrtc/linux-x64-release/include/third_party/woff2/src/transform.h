/* Copyright 2014 Google Inc. All Rights Reserved.

   Distributed under MIT license.
   See file LICENSE for detail or copy at https://opensource.org/licenses/MIT
*/

/* Library for preprocessing fonts as part of the WOFF 2.0 conversion. */

#ifndef WOFF2_TRANSFORM_H_
#define WOFF2_TRANSFORM_H_

#include "./font.h"

namespace woff2 {

// Adds the transformed versions of the glyf and loca tables to the font. The
// transformed loca table has zero length. The tag of the transformed tables is
// derived from the original tag by flipping the MSBs of every byte.
bool TransformGlyfAndLocaTables(Font* font);

// Apply transformation to hmtx table if applicable for this font.
bool TransformHmtxTable(Font* font);

} // namespace woff2

#endif  // WOFF2_TRANSFORM_H_
